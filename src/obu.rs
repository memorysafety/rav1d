#![deny(unsafe_code)]

use crate::c_arc::CArc;
use crate::decode::rav1d_submit_frame;
use crate::env::get_poc_diff;
use crate::error::Rav1dError::EINVAL;
use crate::error::Rav1dError::ENOENT;
use crate::error::Rav1dError::ERANGE;
use crate::error::Rav1dResult;
use crate::getbits::GetBits;
use crate::include::common::intops::clip_u8;
use crate::include::common::intops::ulog2;
use crate::include::dav1d::common::Rav1dDataProps;
use crate::include::dav1d::data::Rav1dData;
use crate::include::dav1d::dav1d::Rav1dDecodeFrameType;
use crate::include::dav1d::headers::DRav1d;
use crate::include::dav1d::headers::Dav1dSequenceHeader;
use crate::include::dav1d::headers::Rav1dAdaptiveBoolean;
use crate::include::dav1d::headers::Rav1dChromaSamplePosition;
use crate::include::dav1d::headers::Rav1dColorPrimaries;
use crate::include::dav1d::headers::Rav1dContentLightLevel;
use crate::include::dav1d::headers::Rav1dFilmGrainData;
use crate::include::dav1d::headers::Rav1dFilterMode;
use crate::include::dav1d::headers::Rav1dFrameHeader;
use crate::include::dav1d::headers::Rav1dFrameHeaderCdef;
use crate::include::dav1d::headers::Rav1dFrameHeaderDelta;
use crate::include::dav1d::headers::Rav1dFrameHeaderDeltaLF;
use crate::include::dav1d::headers::Rav1dFrameHeaderDeltaQ;
use crate::include::dav1d::headers::Rav1dFrameHeaderFilmGrain;
use crate::include::dav1d::headers::Rav1dFrameHeaderLoopFilter;
use crate::include::dav1d::headers::Rav1dFrameHeaderOperatingPoint;
use crate::include::dav1d::headers::Rav1dFrameHeaderQuant;
use crate::include::dav1d::headers::Rav1dFrameHeaderRestoration;
use crate::include::dav1d::headers::Rav1dFrameHeaderSegmentation;
use crate::include::dav1d::headers::Rav1dFrameHeaderSuperRes;
use crate::include::dav1d::headers::Rav1dFrameHeaderTiling;
use crate::include::dav1d::headers::Rav1dFrameSize;
use crate::include::dav1d::headers::Rav1dFrameSkipMode;
use crate::include::dav1d::headers::Rav1dFrameType;
use crate::include::dav1d::headers::Rav1dITUTT35;
use crate::include::dav1d::headers::Rav1dLoopfilterModeRefDeltas;
use crate::include::dav1d::headers::Rav1dMasteringDisplay;
use crate::include::dav1d::headers::Rav1dMatrixCoefficients;
use crate::include::dav1d::headers::Rav1dObuType;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::include::dav1d::headers::Rav1dProfile;
use crate::include::dav1d::headers::Rav1dRestorationType;
use crate::include::dav1d::headers::Rav1dSegmentationData;
use crate::include::dav1d::headers::Rav1dSegmentationDataSet;
use crate::include::dav1d::headers::Rav1dSequenceHeader;
use crate::include::dav1d::headers::Rav1dSequenceHeaderOperatingParameterInfo;
use crate::include::dav1d::headers::Rav1dSequenceHeaderOperatingPoint;
use crate::include::dav1d::headers::Rav1dTransferCharacteristics;
use crate::include::dav1d::headers::Rav1dTxfmMode;
use crate::include::dav1d::headers::Rav1dWarpedMotionParams;
use crate::include::dav1d::headers::Rav1dWarpedMotionType;
use crate::include::dav1d::headers::RAV1D_MAX_CDEF_STRENGTHS;
use crate::include::dav1d::headers::RAV1D_MAX_OPERATING_POINTS;
use crate::include::dav1d::headers::RAV1D_MAX_TILE_COLS;
use crate::include::dav1d::headers::RAV1D_MAX_TILE_ROWS;
use crate::include::dav1d::headers::RAV1D_PRIMARY_REF_NONE;
use crate::include::dav1d::headers::RAV1D_REFS_PER_FRAME;
use crate::internal::Rav1dContext;
use crate::internal::Rav1dState;
use crate::internal::Rav1dTileGroup;
use crate::internal::Rav1dTileGroupHeader;
use crate::levels::ObuMetaType;
use crate::log::Rav1dLog as _;
use crate::picture::rav1d_picture_copy_props;
use crate::picture::PictureFlags;
use crate::thread_task::FRAME_ERROR;
use std::array;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::fmt;
use std::mem;
use std::sync::atomic::Ordering;
use std::sync::Arc;

struct Debug {
    enabled: bool,
    name: &'static str,
    start: usize,
}

impl Debug {
    pub const fn new(enabled: bool, name: &'static str, gb: &GetBits) -> Self {
        Self {
            enabled,
            name,
            start: gb.pos(),
        }
    }

    const fn named(&self, name: &'static str) -> Self {
        let &Self {
            enabled,
            name: _,
            start,
        } = self;
        Self {
            enabled,
            name,
            start,
        }
    }

    pub fn log(&self, gb: &GetBits, msg: fmt::Arguments) {
        let &Self {
            enabled,
            name,
            start,
        } = self;
        if !enabled {
            return;
        }
        let offset = gb.pos() - start;
        println!("{name}: {msg} [off={offset}]");
    }

    pub fn post(&self, gb: &GetBits, post: &str) {
        self.log(gb, format_args!("post-{post}"));
    }
}

fn check_trailing_bits(gb: &mut GetBits, strict_std_compliance: bool) -> Rav1dResult {
    let trailing_one_bit = gb.get_bit();

    if gb.has_error() != 0 {
        return Err(EINVAL);
    }

    if !strict_std_compliance {
        return Ok(());
    }

    if !trailing_one_bit || gb.pending_bits() != 0 {
        return Err(EINVAL);
    }

    gb.bytealign();

    if gb.get_bytes(gb.remaining_len()).iter().any(|&b| b != 0) {
        return Err(EINVAL);
    }

    Ok(())
}

#[inline(never)]
fn parse_seq_hdr(
    gb: &mut GetBits,
    strict_std_compliance: bool,
) -> Rav1dResult<Rav1dSequenceHeader> {
    let debug = Debug::new(false, "SEQHDR", gb);

    let profile = Rav1dProfile::from_repr(gb.get_bits(3) as usize).ok_or(EINVAL)?;
    debug.post(gb, "post-profile");

    let still_picture = gb.get_bit() as u8;
    let reduced_still_picture_header = gb.get_bit() as u8;
    if reduced_still_picture_header != 0 && still_picture == 0 {
        return Err(EINVAL);
    }
    debug.post(gb, "post-stillpicture_flags");

    let num_operating_points;
    let mut operating_points =
        [Rav1dSequenceHeaderOperatingPoint::default(); RAV1D_MAX_OPERATING_POINTS];
    let timing_info_present;
    let num_units_in_tick;
    let time_scale;
    let equal_picture_interval;
    let num_ticks_per_picture;
    let decoder_model_info_present;
    let encoder_decoder_buffer_delay_length;
    let num_units_in_decoding_tick;
    let buffer_removal_delay_length;
    let frame_presentation_delay_length;
    let display_model_info_present;
    let mut operating_parameter_info =
        [Rav1dSequenceHeaderOperatingParameterInfo::default(); RAV1D_MAX_OPERATING_POINTS];
    if reduced_still_picture_header != 0 {
        num_operating_points = 1;
        operating_points[0].major_level = gb.get_bits(3) as u8;
        operating_points[0].minor_level = gb.get_bits(2) as u8;
        operating_points[0].initial_display_delay = 10;

        // Default initialization.
        timing_info_present = Default::default();
        num_units_in_tick = Default::default();
        time_scale = Default::default();
        equal_picture_interval = Default::default();
        num_ticks_per_picture = Default::default();
        decoder_model_info_present = Default::default();
        encoder_decoder_buffer_delay_length = Default::default();
        num_units_in_decoding_tick = Default::default();
        buffer_removal_delay_length = Default::default();
        frame_presentation_delay_length = Default::default();
        display_model_info_present = Default::default();
    } else {
        timing_info_present = gb.get_bit() as u8;
        if timing_info_present != 0 {
            num_units_in_tick = gb.get_bits(32) as u32;
            time_scale = gb.get_bits(32) as u32;
            if strict_std_compliance && (num_units_in_tick == 0 || time_scale == 0) {
                return Err(EINVAL);
            }
            equal_picture_interval = gb.get_bit() as u8;
            if equal_picture_interval != 0 {
                let num_ticks_per_picture_ = gb.get_vlc();
                if num_ticks_per_picture_ == 0xffffffff {
                    return Err(EINVAL);
                }
                num_ticks_per_picture = num_ticks_per_picture_ + 1;
            } else {
                // Default initialization.
                num_ticks_per_picture = Default::default();
            }

            decoder_model_info_present = gb.get_bit() as u8;
            if decoder_model_info_present != 0 {
                encoder_decoder_buffer_delay_length = gb.get_bits(5) as u8 + 1;
                num_units_in_decoding_tick = gb.get_bits(32) as u32;
                if strict_std_compliance && num_units_in_decoding_tick == 0 {
                    return Err(EINVAL);
                }
                buffer_removal_delay_length = gb.get_bits(5) as u8 + 1;
                frame_presentation_delay_length = gb.get_bits(5) as u8 + 1;
            } else {
                // Default initialization.
                encoder_decoder_buffer_delay_length = Default::default();
                num_units_in_decoding_tick = Default::default();
                buffer_removal_delay_length = Default::default();
                frame_presentation_delay_length = Default::default();
            }
        } else {
            // Default initialization.
            num_units_in_tick = Default::default();
            time_scale = Default::default();
            equal_picture_interval = Default::default();
            num_ticks_per_picture = Default::default();
            decoder_model_info_present = Default::default();
            encoder_decoder_buffer_delay_length = Default::default();
            num_units_in_decoding_tick = Default::default();
            buffer_removal_delay_length = Default::default();
            frame_presentation_delay_length = Default::default();
        }
        debug.post(gb, "post-timinginfo");

        display_model_info_present = gb.get_bit() as u8;
        num_operating_points = gb.get_bits(5) as u8 + 1;
        for i in 0..num_operating_points {
            let op = &mut operating_points[i as usize];
            op.idc = gb.get_bits(12) as u16;
            if op.idc != 0 && (op.idc & 0xff == 0 || op.idc & 0xf00 == 0) {
                return Err(EINVAL);
            }
            op.major_level = 2 + gb.get_bits(3) as u8;
            op.minor_level = gb.get_bits(2) as u8;
            if op.major_level > 3 {
                op.tier = gb.get_bit() as u8;
            }
            if decoder_model_info_present != 0 {
                op.decoder_model_param_present = gb.get_bit() as u8;
                if op.decoder_model_param_present != 0 {
                    let opi = &mut operating_parameter_info[i as usize];
                    opi.decoder_buffer_delay =
                        gb.get_bits(encoder_decoder_buffer_delay_length.into()) as u32;
                    opi.encoder_buffer_delay =
                        gb.get_bits(encoder_decoder_buffer_delay_length.into()) as u32;
                    opi.low_delay_mode = gb.get_bit() as u8;
                }
            }
            if display_model_info_present != 0 {
                op.display_model_param_present = gb.get_bit() as u8;
            }
            op.initial_display_delay = if op.display_model_param_present != 0 {
                gb.get_bits(4) as u8 + 1
            } else {
                10
            };
        }
        debug.post(gb, "operating-points");
    }

    let width_n_bits = gb.get_bits(4) as u8 + 1;
    let height_n_bits = gb.get_bits(4) as u8 + 1;
    let max_width = gb.get_bits(width_n_bits.into()) as c_int + 1;
    let max_height = gb.get_bits(height_n_bits.into()) as c_int + 1;
    debug.post(gb, "size");
    let frame_id_numbers_present;
    let delta_frame_id_n_bits;
    let frame_id_n_bits;
    if reduced_still_picture_header == 0 {
        frame_id_numbers_present = gb.get_bit() as u8;
        if frame_id_numbers_present != 0 {
            delta_frame_id_n_bits = gb.get_bits(4) as u8 + 2;
            frame_id_n_bits = gb.get_bits(3) as u8 + delta_frame_id_n_bits + 1;
        } else {
            // Default initialization.
            delta_frame_id_n_bits = Default::default();
            frame_id_n_bits = Default::default();
        }
    } else {
        // Default initialization.
        frame_id_numbers_present = Default::default();
        delta_frame_id_n_bits = Default::default();
        frame_id_n_bits = Default::default();
    }
    debug.post(gb, "frame-id-numbers-present");

    let sb128 = gb.get_bit() as u8;
    let filter_intra = gb.get_bit() as u8;
    let intra_edge_filter = gb.get_bit() as u8;
    let screen_content_tools;
    let force_integer_mv;
    let inter_intra;
    let masked_compound;
    let warped_motion;
    let dual_filter;
    let order_hint;
    let jnt_comp;
    let ref_frame_mvs;
    let order_hint_n_bits;
    if reduced_still_picture_header != 0 {
        screen_content_tools = Rav1dAdaptiveBoolean::Adaptive;
        force_integer_mv = Rav1dAdaptiveBoolean::Adaptive;

        // Default initialization.
        inter_intra = Default::default();
        masked_compound = Default::default();
        warped_motion = Default::default();
        dual_filter = Default::default();
        order_hint = Default::default();
        jnt_comp = Default::default();
        ref_frame_mvs = Default::default();
        order_hint_n_bits = Default::default();
    } else {
        inter_intra = gb.get_bit() as u8;
        masked_compound = gb.get_bit() as u8;
        warped_motion = gb.get_bit() as u8;
        dual_filter = gb.get_bit() as u8;
        order_hint = gb.get_bit() as u8;
        if order_hint != 0 {
            jnt_comp = gb.get_bit() as u8;
            ref_frame_mvs = gb.get_bit() as u8;
        } else {
            // Default initialization.
            jnt_comp = Default::default();
            ref_frame_mvs = Default::default();
        }
        screen_content_tools = if gb.get_bit() {
            Rav1dAdaptiveBoolean::Adaptive
        } else {
            gb.get_bit().into()
        };
        debug.post(gb, "screentools");
        force_integer_mv = if screen_content_tools != Rav1dAdaptiveBoolean::Off {
            if gb.get_bit() {
                Rav1dAdaptiveBoolean::Adaptive
            } else {
                gb.get_bit().into()
            }
        } else {
            Rav1dAdaptiveBoolean::Adaptive
        };
        if order_hint != 0 {
            order_hint_n_bits = gb.get_bits(3) as u8 + 1;
        } else {
            // Default initialization.
            order_hint_n_bits = Default::default();
        }
    }
    let super_res = gb.get_bit() as u8;
    let cdef = gb.get_bit() as u8;
    let restoration = gb.get_bit() as u8;
    debug.post(gb, "featurebits");

    let hbd = {
        let mut hbd = gb.get_bit() as u8;
        if profile == Rav1dProfile::Professional && hbd != 0 {
            hbd += gb.get_bit() as u8;
        }
        hbd
    };
    let monochrome;
    if profile != Rav1dProfile::High {
        monochrome = gb.get_bit() as u8;
    } else {
        // Default initialization.
        monochrome = Default::default();
    }
    let color_description_present = gb.get_bit() as u8;
    let pri;
    let trc;
    let mtrx;
    if color_description_present != 0 {
        pri = Rav1dColorPrimaries(gb.get_bits(8) as u8);
        trc = Rav1dTransferCharacteristics(gb.get_bits(8) as u8);
        mtrx = Rav1dMatrixCoefficients(gb.get_bits(8) as u8)
    } else {
        pri = Rav1dColorPrimaries::UNKNOWN;
        trc = Rav1dTransferCharacteristics::UNKNOWN;
        mtrx = Rav1dMatrixCoefficients::UNKNOWN;
    }
    let color_range;
    let layout;
    let ss_ver;
    let ss_hor;
    let chr;
    if monochrome != 0 {
        color_range = gb.get_bit() as u8;
        layout = Rav1dPixelLayout::I400;
        ss_ver = 1;
        ss_hor = ss_ver;
        chr = Rav1dChromaSamplePosition::Unknown;
    } else if pri == Rav1dColorPrimaries::BT709
        && trc == Rav1dTransferCharacteristics::SRGB
        && mtrx == Rav1dMatrixCoefficients::IDENTITY
    {
        layout = Rav1dPixelLayout::I444;
        color_range = 1;
        if profile != Rav1dProfile::High && !(profile == Rav1dProfile::Professional && hbd == 2) {
            return Err(EINVAL);
        }

        // Default initialization.
        ss_hor = Default::default();
        ss_ver = Default::default();
        chr = Rav1dChromaSamplePosition::Unknown;
    } else {
        color_range = gb.get_bit() as u8;
        match profile {
            Rav1dProfile::Main => {
                layout = Rav1dPixelLayout::I420;
                ss_ver = 1;
                ss_hor = ss_ver;
            }
            Rav1dProfile::High => {
                layout = Rav1dPixelLayout::I444;

                // Default initialization.
                ss_hor = Default::default();
                ss_ver = Default::default();
            }
            Rav1dProfile::Professional => {
                if hbd == 2 {
                    ss_hor = gb.get_bit() as u8;
                    if ss_hor != 0 {
                        ss_ver = gb.get_bit() as u8;
                    } else {
                        // Default initialization.
                        ss_ver = Default::default();
                    }
                } else {
                    ss_hor = 1;

                    // Default initialization.
                    ss_ver = Default::default();
                }
                layout = if ss_hor != 0 {
                    if ss_ver != 0 {
                        Rav1dPixelLayout::I420
                    } else {
                        Rav1dPixelLayout::I422
                    }
                } else {
                    Rav1dPixelLayout::I444
                };
            }
        }
        chr = if ss_hor & ss_ver != 0 {
            Rav1dChromaSamplePosition::from_repr(gb.get_bits(2) as usize).unwrap()
        } else {
            Rav1dChromaSamplePosition::Unknown
        };
    }
    if strict_std_compliance
        && mtrx == Rav1dMatrixCoefficients::IDENTITY
        && layout != Rav1dPixelLayout::I444
    {
        return Err(EINVAL);
    }
    let separate_uv_delta_q;
    if monochrome == 0 {
        separate_uv_delta_q = gb.get_bit() as u8;
    } else {
        // Default initialization.
        separate_uv_delta_q = Default::default();
    }
    debug.post(gb, "colorinfo");

    let film_grain_present = gb.get_bit() as u8;
    debug.post(gb, "filmgrain");

    // We needn't bother flushing the OBU here: we'll check we didn't
    // overrun in the caller and will then discard gb, so there's no
    // point in setting its position properly.

    check_trailing_bits(gb, strict_std_compliance)?;
    Ok(Rav1dSequenceHeader {
        profile,
        max_width,
        max_height,
        layout,
        pri,
        trc,
        mtrx,
        chr,
        hbd,
        color_range,
        num_operating_points,
        operating_points,
        still_picture,
        reduced_still_picture_header,
        timing_info_present,
        num_units_in_tick,
        time_scale,
        equal_picture_interval,
        num_ticks_per_picture,
        decoder_model_info_present,
        encoder_decoder_buffer_delay_length,
        num_units_in_decoding_tick,
        buffer_removal_delay_length,
        frame_presentation_delay_length,
        display_model_info_present,
        width_n_bits,
        height_n_bits,
        frame_id_numbers_present,
        delta_frame_id_n_bits,
        frame_id_n_bits,
        sb128,
        filter_intra,
        intra_edge_filter,
        inter_intra,
        masked_compound,
        warped_motion,
        dual_filter,
        order_hint,
        jnt_comp,
        ref_frame_mvs,
        screen_content_tools,
        force_integer_mv,
        order_hint_n_bits,
        super_res,
        cdef,
        restoration,
        ss_hor,
        ss_ver,
        monochrome,
        color_description_present,
        separate_uv_delta_q,
        film_grain_present,
        operating_parameter_info,
    })
}

pub(crate) fn rav1d_parse_sequence_header(
    mut data: &[u8],
) -> Rav1dResult<DRav1d<Rav1dSequenceHeader, Dav1dSequenceHeader>> {
    let mut res = Err(ENOENT);

    while !data.is_empty() {
        let gb = &mut GetBits::new(data);

        gb.get_bit(); // obu_forbidden_bit
        let r#type = Rav1dObuType::from_repr(gb.get_bits(4) as usize);
        let has_extension = gb.get_bit();
        let has_length_field = gb.get_bit();
        gb.get_bits(1 + has_extension as i32 * 8); // reserved

        // obu length field
        let obu_end = if has_length_field {
            let len = gb.get_uleb128() as usize;
            let len = gb.byte_pos() + len;
            if len > data.len() {
                return Err(EINVAL);
            }
            len
        } else {
            data.len()
        };

        if r#type == Some(Rav1dObuType::SeqHdr) {
            res = Ok(parse_seq_hdr(gb, false)?);
            if gb.byte_pos() > obu_end {
                return Err(EINVAL);
            }
            gb.bytealign();
        }

        if gb.has_error() != 0 {
            return Err(EINVAL);
        }
        assert!(!gb.has_pending_bits());

        data = &data[obu_end..]
    }

    res.map(DRav1d::from_rav1d)
}

fn parse_frame_size(
    state: &Rav1dState,
    seqhdr: &Rav1dSequenceHeader,
    refidx: Option<&[i8; RAV1D_REFS_PER_FRAME]>,
    frame_size_override: bool,
    gb: &mut GetBits,
) -> Rav1dResult<Rav1dFrameSize> {
    if let Some(refidx) = refidx {
        for i in 0..7 {
            if gb.get_bit() {
                let r#ref = &state.refs[refidx[i as usize] as usize].p;
                let ref_size = &r#ref.p.frame_hdr.as_ref().ok_or(EINVAL)?.size;
                let width1 = ref_size.width[1];
                let height = ref_size.height;
                let render_width = ref_size.render_width;
                let render_height = ref_size.render_height;
                let enabled = seqhdr.super_res != 0 && gb.get_bit();
                let width_scale_denominator;
                let width0;
                if enabled {
                    width_scale_denominator = 9 + gb.get_bits(3) as u8;
                    let d = width_scale_denominator as c_int;
                    width0 = cmp::max((width1 * 8 + (d >> 1)) / d, cmp::min(16, width1));
                } else {
                    width_scale_denominator = 8;
                    width0 = width1;
                }
                let width = [width0, width1];
                return Ok(Rav1dFrameSize {
                    width,
                    height,
                    render_width,
                    render_height,
                    super_res: Rav1dFrameHeaderSuperRes {
                        enabled,
                        width_scale_denominator,
                    },
                    have_render_size: 0,
                });
            }
        }
    }

    let width1;
    let height;
    if frame_size_override {
        width1 = gb.get_bits(seqhdr.width_n_bits.into()) as c_int + 1;
        height = gb.get_bits(seqhdr.height_n_bits.into()) as c_int + 1;
    } else {
        width1 = seqhdr.max_width;
        height = seqhdr.max_height;
    }
    let enabled = seqhdr.super_res != 0 && gb.get_bit();
    let width_scale_denominator;
    let width0;
    if enabled {
        width_scale_denominator = 9 + gb.get_bits(3) as u8;
        let d = width_scale_denominator as c_int;
        width0 = cmp::max((width1 * 8 + (d >> 1)) / d, cmp::min(16, width1));
    } else {
        width_scale_denominator = 8;
        width0 = width1;
    }
    let have_render_size = gb.get_bit() as u8;
    let render_width;
    let render_height;
    if have_render_size != 0 {
        render_width = gb.get_bits(16) as c_int + 1;
        render_height = gb.get_bits(16) as c_int + 1;
    } else {
        render_width = width1;
        render_height = height;
    }
    let width = [width0, width1];
    Ok(Rav1dFrameSize {
        width,
        height,
        render_width,
        render_height,
        super_res: Rav1dFrameHeaderSuperRes {
            enabled,
            width_scale_denominator,
        },
        have_render_size,
    })
}

#[inline]
fn tile_log2(sz: c_int, tgt: c_int) -> u8 {
    let mut k = 0;
    while sz << k < tgt {
        k += 1;
    }
    k
}

static default_mode_ref_deltas: Rav1dLoopfilterModeRefDeltas = Rav1dLoopfilterModeRefDeltas {
    mode_delta: [0, 0],
    ref_delta: [1, 0, 0, 0, -1, 0, -1, -1],
};

fn parse_refidx(
    state: &Rav1dState,
    seqhdr: &Rav1dSequenceHeader,
    frame_ref_short_signaling: u8,
    frame_offset: u8,
    frame_id: u32,
    gb: &mut GetBits,
) -> Rav1dResult<[i8; RAV1D_REFS_PER_FRAME]> {
    let mut refidx = [-1; RAV1D_REFS_PER_FRAME];
    if frame_ref_short_signaling != 0 {
        // FIXME: Nearly verbatim copy from section 7.8
        refidx[0] = gb.get_bits(3) as i8;
        refidx[3] = gb.get_bits(3) as i8;

        let mut shifted_frame_offset = [0; 8];
        let current_frame_offset = 1 << seqhdr.order_hint_n_bits - 1;
        for i in 0..8 {
            shifted_frame_offset[i as usize] = current_frame_offset
                + get_poc_diff(
                    seqhdr.order_hint_n_bits,
                    state.refs[i as usize]
                        .p
                        .p
                        .frame_hdr
                        .as_ref()
                        .ok_or(EINVAL)?
                        .frame_offset as c_int,
                    frame_offset as c_int,
                );
        }

        let mut used_frame = [0, 0, 0, 0, 0, 0, 0, 0];
        used_frame[refidx[0] as usize] = 1;
        used_frame[refidx[3] as usize] = 1;

        let mut latest_frame_offset = -1;
        for i in 0..8 {
            let hint = shifted_frame_offset[i as usize];
            if used_frame[i as usize] == 0
                && hint >= current_frame_offset
                && hint >= latest_frame_offset
            {
                refidx[6] = i;
                latest_frame_offset = hint;
            }
        }
        if latest_frame_offset != -1 {
            used_frame[refidx[6] as usize] = 1;
        }

        let mut earliest_frame_offset = i32::MAX;
        for i in 0..8 {
            let hint = shifted_frame_offset[i as usize];
            if used_frame[i as usize] == 0
                && hint >= current_frame_offset
                && hint < earliest_frame_offset
            {
                refidx[4] = i;
                earliest_frame_offset = hint;
            }
        }
        if earliest_frame_offset != i32::MAX {
            used_frame[refidx[4] as usize] = 1;
        }

        earliest_frame_offset = i32::MAX;
        for i in 0..8 {
            let hint = shifted_frame_offset[i as usize];
            if used_frame[i as usize] == 0
                && hint >= current_frame_offset
                && hint < earliest_frame_offset
            {
                refidx[5] = i;
                earliest_frame_offset = hint;
            }
        }
        if earliest_frame_offset != i32::MAX {
            used_frame[refidx[5] as usize] = 1;
        }

        for i in 1..7 {
            if refidx[i as usize] < 0 {
                latest_frame_offset = -1;
                for j in 0..8 {
                    let hint = shifted_frame_offset[j as usize];
                    if used_frame[j as usize] == 0
                        && hint < current_frame_offset
                        && hint >= latest_frame_offset
                    {
                        refidx[i as usize] = j;
                        latest_frame_offset = hint;
                    }
                }
                if latest_frame_offset != -1 {
                    used_frame[refidx[i as usize] as usize] = 1;
                }
            }
        }

        earliest_frame_offset = i32::MAX;
        let mut r#ref = -1;
        for i in 0..8 {
            let hint = shifted_frame_offset[i as usize];
            if hint < earliest_frame_offset {
                r#ref = i;
                earliest_frame_offset = hint;
            }
        }
        for i in 0..7 {
            if refidx[i as usize] < 0 {
                refidx[i as usize] = r#ref;
            }
        }
    }
    for i in 0..7 {
        if frame_ref_short_signaling == 0 {
            refidx[i as usize] = gb.get_bits(3) as i8;
        }
        if seqhdr.frame_id_numbers_present != 0 {
            let delta_ref_frame_id = gb.get_bits(seqhdr.delta_frame_id_n_bits.into()) as u32 + 1;
            let ref_frame_id = frame_id + (1 << seqhdr.frame_id_n_bits) - delta_ref_frame_id
                & (1 << seqhdr.frame_id_n_bits) - 1;
            state.refs[refidx[i as usize] as usize]
                .p
                .p
                .frame_hdr
                .as_ref()
                .filter(|ref_frame_hdr| ref_frame_hdr.frame_id == ref_frame_id)
                .ok_or(EINVAL)?;
        }
    }
    Ok(refidx)
}

fn parse_tiling(
    seqhdr: &Rav1dSequenceHeader,
    size: &Rav1dFrameSize,
    debug: &Debug,
    gb: &mut GetBits,
) -> Rav1dResult<Rav1dFrameHeaderTiling> {
    let uniform = gb.get_bit() as u8;
    let sbsz_min1 = ((64) << seqhdr.sb128) - 1;
    let sbsz_log2 = 6 + seqhdr.sb128;
    let sbw = size.width[0] + sbsz_min1 >> sbsz_log2;
    let sbh = size.height + sbsz_min1 >> sbsz_log2;
    let max_tile_width_sb = 4096 >> sbsz_log2;
    let max_tile_area_sb = 4096 * 2304 >> 2 * sbsz_log2;
    let min_log2_cols = tile_log2(max_tile_width_sb, sbw);
    let max_log2_cols = tile_log2(1, cmp::min(sbw, RAV1D_MAX_TILE_COLS as c_int));
    let max_log2_rows = tile_log2(1, cmp::min(sbh, RAV1D_MAX_TILE_ROWS as c_int));
    let min_log2_tiles = cmp::max(tile_log2(max_tile_area_sb, sbw * sbh), min_log2_cols);
    let mut log2_cols;
    let mut cols;
    let mut log2_rows;
    let mut rows;
    let mut col_start_sb = [0; RAV1D_MAX_TILE_COLS + 1];
    let mut row_start_sb = [0; RAV1D_MAX_TILE_ROWS + 1];
    if uniform != 0 {
        log2_cols = min_log2_cols;
        while log2_cols < max_log2_cols && gb.get_bit() {
            log2_cols += 1;
        }
        let tile_w = 1 + (sbw - 1 >> log2_cols);
        cols = 0;
        let mut sbx = 0;
        while sbx < sbw {
            col_start_sb[cols as usize] = sbx as u16;
            sbx += tile_w;
            cols += 1;
        }
        let min_log2_rows = min_log2_tiles.saturating_sub(log2_cols);

        log2_rows = min_log2_rows;
        while log2_rows < max_log2_rows && gb.get_bit() {
            log2_rows += 1;
        }
        let tile_h = 1 + (sbh - 1 >> log2_rows);
        rows = 0;
        let mut sby = 0;
        while sby < sbh {
            row_start_sb[rows as usize] = sby as u16;
            sby += tile_h;
            rows += 1;
        }
    } else {
        cols = 0;
        let mut widest_tile = 0;
        let mut max_tile_area_sb = sbw * sbh;
        let mut sbx = 0;
        while sbx < sbw && cols < RAV1D_MAX_TILE_COLS as u8 {
            let tile_width_sb = cmp::min(sbw - sbx, max_tile_width_sb);
            let tile_w = if tile_width_sb > 1 {
                1 + gb.get_uniform(tile_width_sb as c_uint) as c_int
            } else {
                1
            };
            col_start_sb[cols as usize] = sbx as u16;
            sbx += tile_w;
            widest_tile = cmp::max(widest_tile, tile_w);
            cols += 1;
        }
        log2_cols = tile_log2(1, cols.into());
        if min_log2_tiles != 0 {
            max_tile_area_sb >>= min_log2_tiles + 1;
        }
        let max_tile_height_sb = cmp::max(max_tile_area_sb / widest_tile, 1);

        rows = 0;
        let mut sby = 0;
        while sby < sbh && rows < RAV1D_MAX_TILE_ROWS as u8 {
            let tile_height_sb = cmp::min(sbh - sby, max_tile_height_sb);
            let tile_h = if tile_height_sb > 1 {
                1 + gb.get_uniform(tile_height_sb as c_uint) as c_int
            } else {
                1
            };
            row_start_sb[rows as usize] = sby as u16;
            sby += tile_h;
            rows += 1;
        }
        log2_rows = tile_log2(1, rows.into());
    }
    col_start_sb[cols as usize] = sbw as u16;
    row_start_sb[rows as usize] = sbh as u16;
    let update;
    let n_bytes;
    if log2_cols != 0 || log2_rows != 0 {
        update = gb.get_bits((log2_cols + log2_rows).into()) as u16;
        if update >= cols as u16 * rows as u16 {
            return Err(EINVAL);
        }
        n_bytes = gb.get_bits(2) as u8 + 1;
    } else {
        update = 0;
        n_bytes = update as u8;
    }
    debug.post(gb, "tiling");
    Ok(Rav1dFrameHeaderTiling {
        uniform,
        n_bytes,
        min_log2_cols,
        max_log2_cols,
        log2_cols,
        cols,
        // TODO(kkysen) Never written or read in C; is this correct?
        min_log2_rows: 0,
        max_log2_rows,
        log2_rows,
        rows,
        col_start_sb,
        row_start_sb,
        update,
    })
}

fn parse_quant(
    seqhdr: &Rav1dSequenceHeader,
    debug: &Debug,
    gb: &mut GetBits,
) -> Rav1dFrameHeaderQuant {
    let yac = gb.get_bits(8) as u8;
    let ydc_delta = if gb.get_bit() {
        gb.get_sbits(7) as i8
    } else {
        0
    };
    let udc_delta;
    let uac_delta;
    let vdc_delta;
    let vac_delta;
    if seqhdr.monochrome == 0 {
        // If the sequence header says that delta_q might be different
        // for U, V, we must check whether it actually is for this
        // frame.
        let diff_uv_delta = if seqhdr.separate_uv_delta_q != 0 {
            gb.get_bit() as c_int
        } else {
            0
        };
        udc_delta = if gb.get_bit() {
            gb.get_sbits(7) as i8
        } else {
            0
        };
        uac_delta = if gb.get_bit() {
            gb.get_sbits(7) as i8
        } else {
            0
        };
        if diff_uv_delta != 0 {
            vdc_delta = if gb.get_bit() {
                gb.get_sbits(7) as i8
            } else {
                0
            };
            vac_delta = if gb.get_bit() {
                gb.get_sbits(7) as i8
            } else {
                0
            };
        } else {
            vdc_delta = udc_delta;
            vac_delta = uac_delta;
        }
    } else {
        // Default initialization.
        udc_delta = Default::default();
        uac_delta = Default::default();
        vdc_delta = Default::default();
        vac_delta = Default::default();
    }
    debug.post(gb, "quant");
    let qm = gb.get_bit() as u8;
    let qm_y;
    let qm_u;
    let qm_v;
    if qm != 0 {
        qm_y = gb.get_bits(4) as u8;
        qm_u = gb.get_bits(4) as u8;
        qm_v = if seqhdr.separate_uv_delta_q != 0 {
            gb.get_bits(4) as u8
        } else {
            qm_u
        };
    } else {
        // Default initialization.
        qm_y = Default::default();
        qm_u = Default::default();
        qm_v = Default::default();
    }
    debug.post(gb, "qm");
    Rav1dFrameHeaderQuant {
        yac,
        ydc_delta,
        udc_delta,
        uac_delta,
        vdc_delta,
        vac_delta,
        qm,
        qm_y,
        qm_u,
        qm_v,
    }
}

fn parse_seg_data(gb: &mut GetBits) -> Rav1dSegmentationDataSet {
    let mut preskip = 0;
    let mut last_active_segid = -1 as i8;
    let d = array::from_fn(|i| {
        let i = i as i8;
        let delta_q;
        if gb.get_bit() {
            delta_q = gb.get_sbits(9) as i16;
            last_active_segid = i;
        } else {
            delta_q = 0;
        }
        let delta_lf_y_v;
        if gb.get_bit() {
            delta_lf_y_v = gb.get_sbits(7) as i8;
            last_active_segid = i;
        } else {
            delta_lf_y_v = 0;
        }
        let delta_lf_y_h;
        if gb.get_bit() {
            delta_lf_y_h = gb.get_sbits(7) as i8;
            last_active_segid = i;
        } else {
            delta_lf_y_h = 0;
        }
        let delta_lf_u;
        if gb.get_bit() {
            delta_lf_u = gb.get_sbits(7) as i8;
            last_active_segid = i;
        } else {
            delta_lf_u = 0;
        }
        let delta_lf_v;
        if gb.get_bit() {
            delta_lf_v = gb.get_sbits(7) as i8;
            last_active_segid = i;
        } else {
            delta_lf_v = 0;
        }
        let r#ref;
        if gb.get_bit() {
            r#ref = gb.get_bits(3) as i8;
            last_active_segid = i;
            preskip = 1;
        } else {
            r#ref = -1;
        }
        let skip = gb.get_bit() as u8;
        if skip != 0 {
            last_active_segid = i;
            preskip = 1;
        }
        let globalmv = gb.get_bit() as u8;
        if globalmv != 0 {
            last_active_segid = i;
            preskip = 1;
        }
        Rav1dSegmentationData {
            delta_q,
            delta_lf_y_v,
            delta_lf_y_h,
            delta_lf_u,
            delta_lf_v,
            r#ref,
            skip,
            globalmv,
        }
    });
    Rav1dSegmentationDataSet {
        d,
        preskip,
        last_active_segid,
    }
}

fn parse_segmentation(
    state: &Rav1dState,
    primary_ref_frame: u8,
    refidx: &[i8; RAV1D_REFS_PER_FRAME],
    quant: &Rav1dFrameHeaderQuant,
    debug: &Debug,
    gb: &mut GetBits,
) -> Rav1dResult<Rav1dFrameHeaderSegmentation> {
    let enabled = gb.get_bit() as u8;
    let update_map;
    let temporal;
    let update_data;
    let seg_data = if enabled != 0 {
        if primary_ref_frame == RAV1D_PRIMARY_REF_NONE {
            update_map = 1;
            temporal = 0;
            update_data = 1;
        } else {
            update_map = gb.get_bit() as u8;
            temporal = if update_map != 0 {
                gb.get_bit() as u8
            } else {
                0
            };
            update_data = gb.get_bit() as u8;
        }

        if update_data != 0 {
            parse_seg_data(gb)
        } else {
            // segmentation.update_data was false so we should copy
            // segmentation data from the reference frame.
            assert!(primary_ref_frame != RAV1D_PRIMARY_REF_NONE);
            let pri_ref = refidx[primary_ref_frame as usize];
            state.refs[pri_ref as usize]
                .p
                .p
                .frame_hdr
                .as_ref()
                .ok_or(EINVAL)?
                .segmentation
                .seg_data
                .clone()
        }
    } else {
        // Default initialization.
        update_map = Default::default();
        temporal = Default::default();
        update_data = Default::default();

        let mut seg_data = Rav1dSegmentationDataSet::default();
        for data in &mut seg_data.d {
            data.r#ref = -1;
        }
        seg_data
    };
    debug.post(gb, "segmentation");

    // derive lossless flags
    let delta_lossless = quant.ydc_delta == 0
        && quant.udc_delta == 0
        && quant.uac_delta == 0
        && quant.vdc_delta == 0
        && quant.vac_delta == 0;
    let qidx = array::from_fn(|i| {
        if enabled != 0 {
            clip_u8(quant.yac as c_int + seg_data.d[i].delta_q as c_int)
        } else {
            quant.yac
        }
    });
    let lossless = array::from_fn(|i| qidx[i] == 0 && delta_lossless);
    Ok(Rav1dFrameHeaderSegmentation {
        enabled,
        update_map,
        temporal,
        update_data,
        seg_data,
        lossless,
        qidx,
    })
}

fn parse_delta(
    quant: &Rav1dFrameHeaderQuant,
    allow_intrabc: bool,
    debug: &Debug,
    gb: &mut GetBits,
) -> Rav1dFrameHeaderDelta {
    let q = {
        let present = if quant.yac != 0 {
            gb.get_bit() as u8
        } else {
            0
        };
        let res_log2 = if present != 0 {
            gb.get_bits(2) as u8
        } else {
            0
        };
        Rav1dFrameHeaderDeltaQ { present, res_log2 }
    };
    let lf = {
        let present = (q.present != 0 && !allow_intrabc && gb.get_bit()) as u8;
        let res_log2 = if present != 0 {
            gb.get_bits(2) as u8
        } else {
            0
        };
        let multi = if present != 0 { gb.get_bit() as u8 } else { 0 };
        Rav1dFrameHeaderDeltaLF {
            present,
            res_log2,
            multi,
        }
    };
    debug.post(gb, "delta_q_lf_flags");
    Rav1dFrameHeaderDelta { q, lf }
}

fn parse_loopfilter(
    state: &Rav1dState,
    seqhdr: &Rav1dSequenceHeader,
    all_lossless: bool,
    allow_intrabc: bool,
    primary_ref_frame: u8,
    refidx: &[i8; RAV1D_REFS_PER_FRAME],
    debug: &Debug,
    gb: &mut GetBits,
) -> Rav1dResult<Rav1dFrameHeaderLoopFilter> {
    let level_y;
    let level_u;
    let level_v;
    let mode_ref_delta_enabled;
    let mode_ref_delta_update;
    let mut mode_ref_deltas;
    let sharpness;
    if all_lossless || allow_intrabc {
        level_y = [0; 2];
        level_v = 0;
        level_u = level_v;
        sharpness = 0;
        mode_ref_delta_enabled = 1;
        mode_ref_delta_update = 1;
        mode_ref_deltas = default_mode_ref_deltas.clone();
    } else {
        level_y = [gb.get_bits(6) as u8, gb.get_bits(6) as u8];
        if seqhdr.monochrome == 0 && (level_y[0] != 0 || level_y[1] != 0) {
            level_u = gb.get_bits(6) as u8;
            level_v = gb.get_bits(6) as u8;
        } else {
            // Default initialization.
            level_u = Default::default();
            level_v = Default::default();
        }
        sharpness = gb.get_bits(3) as u8;

        if primary_ref_frame == RAV1D_PRIMARY_REF_NONE {
            mode_ref_deltas = default_mode_ref_deltas.clone();
        } else {
            let r#ref = refidx[primary_ref_frame as usize];
            mode_ref_deltas = state.refs[r#ref as usize]
                .p
                .p
                .frame_hdr
                .as_ref()
                .ok_or(EINVAL)?
                .loopfilter
                .mode_ref_deltas
                .clone();
        }
        mode_ref_delta_enabled = gb.get_bit() as u8;
        if mode_ref_delta_enabled != 0 {
            mode_ref_delta_update = gb.get_bit() as u8;
            if mode_ref_delta_update != 0 {
                for i in 0..8 {
                    if gb.get_bit() {
                        mode_ref_deltas.ref_delta[i as usize] = gb.get_sbits(7) as i8;
                    }
                }
                for i in 0..2 {
                    if gb.get_bit() {
                        mode_ref_deltas.mode_delta[i as usize] = gb.get_sbits(7) as i8;
                    }
                }
            }
        } else {
            // Default initialization.
            mode_ref_delta_update = Default::default();
        }
    }
    debug.post(gb, "lpf");
    Ok(Rav1dFrameHeaderLoopFilter {
        level_y,
        level_u,
        level_v,
        mode_ref_delta_enabled,
        mode_ref_delta_update,
        mode_ref_deltas,
        sharpness,
    })
}

fn parse_cdef(
    seqhdr: &Rav1dSequenceHeader,
    all_lossless: bool,
    allow_intrabc: bool,
    debug: &Debug,
    gb: &mut GetBits,
) -> Rav1dFrameHeaderCdef {
    let damping;
    let n_bits;
    let mut y_strength = [0; RAV1D_MAX_CDEF_STRENGTHS];
    let mut uv_strength = [0; RAV1D_MAX_CDEF_STRENGTHS];
    if !all_lossless && seqhdr.cdef != 0 && !allow_intrabc {
        damping = gb.get_bits(2) as u8 + 3;
        n_bits = gb.get_bits(2) as u8;
        for i in 0..1 << n_bits {
            y_strength[i as usize] = gb.get_bits(6) as u8;
            if seqhdr.monochrome == 0 {
                uv_strength[i as usize] = gb.get_bits(6) as u8;
            }
        }
    } else {
        // Default initialization.
        damping = Default::default();

        n_bits = 0;
        y_strength[0] = 0;
        uv_strength[0] = 0;
    }
    debug.post(gb, "cdef");
    Rav1dFrameHeaderCdef {
        damping,
        n_bits,
        y_strength,
        uv_strength,
    }
}

fn parse_restoration(
    seqhdr: &Rav1dSequenceHeader,
    all_lossless: bool,
    super_res_enabled: bool,
    allow_intrabc: bool,
    debug: &Debug,
    gb: &mut GetBits,
) -> Rav1dFrameHeaderRestoration {
    let r#type;
    let unit_size;
    if (!all_lossless || super_res_enabled) && seqhdr.restoration != 0 && !allow_intrabc {
        let type_0 = Rav1dRestorationType::from_repr(gb.get_bits(2) as usize).unwrap();
        r#type = if seqhdr.monochrome == 0 {
            [
                type_0,
                Rav1dRestorationType::from_repr(gb.get_bits(2) as usize).unwrap(),
                Rav1dRestorationType::from_repr(gb.get_bits(2) as usize).unwrap(),
            ]
        } else {
            [
                type_0,
                Rav1dRestorationType::None,
                Rav1dRestorationType::None,
            ]
        };

        unit_size = match r#type {
            [Rav1dRestorationType::None, Rav1dRestorationType::None, Rav1dRestorationType::None] => {
                [8, 0]
            }
            _ => {
                // Log2 of the restoration unit size.
                let mut unit_size_0 = 6 + seqhdr.sb128;
                if gb.get_bit() {
                    unit_size_0 += 1;
                    if seqhdr.sb128 == 0 {
                        unit_size_0 += gb.get_bit() as u8;
                    }
                }

                let unit_size_1 = if (r#type[1] != Rav1dRestorationType::None
                    || r#type[2] != Rav1dRestorationType::None)
                    && seqhdr.ss_hor == 1
                    && seqhdr.ss_ver == 1
                {
                    unit_size_0 - gb.get_bit() as u8
                } else {
                    unit_size_0
                };

                [unit_size_0, unit_size_1]
            }
        };
    } else {
        r#type = [Rav1dRestorationType::None; 3];

        // Default initialization.
        unit_size = Default::default();
    }
    debug.post(gb, "restoration");
    Rav1dFrameHeaderRestoration { r#type, unit_size }
}

fn parse_skip_mode(
    state: &Rav1dState,
    seqhdr: &Rav1dSequenceHeader,
    switchable_comp_refs: u8,
    frame_type: Rav1dFrameType,
    frame_offset: u8,
    refidx: &[i8; RAV1D_REFS_PER_FRAME],
    debug: &Debug,
    gb: &mut GetBits,
) -> Rav1dResult<Rav1dFrameSkipMode> {
    let mut allowed = 0;
    let mut refs = Default::default();
    if switchable_comp_refs != 0 && frame_type.is_inter_or_switch() && seqhdr.order_hint != 0 {
        let poc = frame_offset as c_uint;
        let mut off_before = 0xffffffff;
        let mut off_after = -1;
        let mut off_before_idx = 0;
        let mut off_after_idx = 0;
        for i in 0..7 {
            let refpoc = state.refs[refidx[i as usize] as usize]
                .p
                .p
                .frame_hdr
                .as_ref()
                .ok_or(EINVAL)?
                .frame_offset as c_uint;

            let diff = get_poc_diff(seqhdr.order_hint_n_bits, refpoc as c_int, poc as c_int);
            if diff > 0 {
                if off_after == -1
                    || get_poc_diff(seqhdr.order_hint_n_bits, off_after, refpoc as c_int) > 0
                {
                    off_after = refpoc as c_int;
                    off_after_idx = i;
                }
            } else if diff < 0
                && (off_before == 0xffffffff
                    || get_poc_diff(
                        seqhdr.order_hint_n_bits,
                        refpoc as c_int,
                        off_before as c_int,
                    ) > 0)
            {
                off_before = refpoc;
                off_before_idx = i;
            }
        }

        if off_before != 0xffffffff && off_after != -1 {
            refs = [
                cmp::min(off_before_idx, off_after_idx),
                cmp::max(off_before_idx, off_after_idx),
            ];
            allowed = 1;
        } else if off_before != 0xffffffff {
            let mut off_before2 = 0xffffffff;
            let mut off_before2_idx = 0;
            for i in 0..7 {
                let refpoc = state.refs[refidx[i as usize] as usize]
                    .p
                    .p
                    .frame_hdr
                    .as_ref()
                    .ok_or(EINVAL)?
                    .frame_offset as c_uint;
                if get_poc_diff(
                    seqhdr.order_hint_n_bits,
                    refpoc as c_int,
                    off_before as c_int,
                ) < 0
                {
                    if off_before2 == 0xffffffff
                        || get_poc_diff(
                            seqhdr.order_hint_n_bits,
                            refpoc as c_int,
                            off_before2 as c_int,
                        ) > 0
                    {
                        off_before2 = refpoc;
                        off_before2_idx = i;
                    }
                }
            }

            if off_before2 != 0xffffffff {
                refs = [
                    cmp::min(off_before_idx, off_before2_idx),
                    cmp::max(off_before_idx, off_before2_idx),
                ];
                allowed = 1;
            }
        }
    }
    let enabled = if allowed != 0 { gb.get_bit() as u8 } else { 0 };
    debug.post(gb, "extskip");
    Ok(Rav1dFrameSkipMode {
        allowed,
        enabled,
        refs,
    })
}

fn parse_gmv(
    state: &Rav1dState,
    frame_type: Rav1dFrameType,
    primary_ref_frame: u8,
    refidx: &[i8; RAV1D_REFS_PER_FRAME],
    hp: bool,
    debug: &Debug,
    gb: &mut GetBits,
) -> Rav1dResult<[Rav1dWarpedMotionParams; RAV1D_REFS_PER_FRAME]> {
    let mut gmv = array::from_fn(|_| Rav1dWarpedMotionParams::default());

    if frame_type.is_inter_or_switch() {
        for (i, gmv) in gmv.iter_mut().enumerate() {
            gmv.r#type = if !gb.get_bit() {
                Rav1dWarpedMotionType::Identity
            } else if gb.get_bit() {
                Rav1dWarpedMotionType::RotZoom
            } else if gb.get_bit() {
                Rav1dWarpedMotionType::Translation
            } else {
                Rav1dWarpedMotionType::Affine
            };
            if gmv.r#type == Rav1dWarpedMotionType::Identity {
                continue;
            }

            let default_gmv = Default::default();
            let ref_gmv = if primary_ref_frame == RAV1D_PRIMARY_REF_NONE {
                &default_gmv
            } else {
                let pri_ref = refidx[primary_ref_frame as usize];
                &state.refs[pri_ref as usize]
                    .p
                    .p
                    .frame_hdr
                    .as_ref()
                    .ok_or(EINVAL)?
                    .gmv[i]
            };
            let mat = &mut gmv.matrix;
            let ref_mat = &ref_gmv.matrix;
            let bits;
            let shift;

            if gmv.r#type >= Rav1dWarpedMotionType::RotZoom {
                mat[2] = (1 << 16) + 2 * gb.get_bits_subexp(ref_mat[2] - (1 << 16) >> 1, 12);
                mat[3] = 2 * gb.get_bits_subexp(ref_mat[3] >> 1, 12);

                bits = 12;
                shift = 10;
            } else {
                bits = 9 - !hp as c_int;
                shift = 13 + !hp as c_int;
            }

            if gmv.r#type == Rav1dWarpedMotionType::Affine {
                mat[4] = 2 * gb.get_bits_subexp(ref_mat[4] >> 1, 12);
                mat[5] = (1 << 16) + 2 * gb.get_bits_subexp(ref_mat[5] - (1 << 16) >> 1, 12);
            } else {
                mat[4] = -mat[3];
                mat[5] = mat[2];
            }

            mat[0] = gb.get_bits_subexp(ref_mat[0] >> shift, bits as c_uint) * (1 << shift);
            mat[1] = gb.get_bits_subexp(ref_mat[1] >> shift, bits as c_uint) * (1 << shift);
        }
    }
    debug.post(gb, "gmv");
    Ok(gmv)
}

fn parse_film_grain_data(
    seqhdr: &Rav1dSequenceHeader,
    seed: c_uint,
    gb: &mut GetBits,
) -> Rav1dResult<Rav1dFilmGrainData> {
    let num_y_points = gb.get_bits(4) as c_int;
    if num_y_points > 14 {
        return Err(EINVAL);
    }

    let mut y_points = [[0; 2]; 14];
    for i in 0..num_y_points {
        y_points[i as usize][0] = gb.get_bits(8) as u8;
        if i != 0 && y_points[(i - 1) as usize][0] as c_int >= y_points[i as usize][0] as c_int {
            return Err(EINVAL);
        }
        y_points[i as usize][1] = gb.get_bits(8) as u8;
    }

    let chroma_scaling_from_luma = seqhdr.monochrome == 0 && gb.get_bit();
    let mut num_uv_points = [0; 2];
    let mut uv_points = [[[0; 2]; 10]; 2];
    if seqhdr.monochrome != 0
        || chroma_scaling_from_luma
        || seqhdr.ss_ver == 1 && seqhdr.ss_hor == 1 && num_y_points == 0
    {
        num_uv_points = [0; 2];
    } else {
        for pl in 0..2 {
            num_uv_points[pl as usize] = gb.get_bits(4) as c_int;
            if num_uv_points[pl as usize] > 10 {
                return Err(EINVAL);
            }
            for i in 0..num_uv_points[pl as usize] {
                uv_points[pl as usize][i as usize][0] = gb.get_bits(8) as u8;
                if i != 0
                    && uv_points[pl as usize][(i - 1) as usize][0] as c_int
                        >= uv_points[pl as usize][i as usize][0] as c_int
                {
                    return Err(EINVAL);
                }
                uv_points[pl as usize][i as usize][1] = gb.get_bits(8) as u8;
            }
        }
    }

    if seqhdr.ss_hor == 1
        && seqhdr.ss_ver == 1
        && (num_uv_points[0] != 0) != (num_uv_points[1] != 0)
    {
        return Err(EINVAL);
    }

    let scaling_shift = gb.get_bits(2) as u8 + 8;
    let ar_coeff_lag = gb.get_bits(2) as c_int;
    let num_y_pos = 2 * ar_coeff_lag * (ar_coeff_lag + 1);
    let mut ar_coeffs_y = [0; 24];
    if num_y_points != 0 {
        for i in 0..num_y_pos {
            ar_coeffs_y[i as usize] = gb.get_bits(8).wrapping_sub(128) as i8;
        }
    }
    let mut ar_coeffs_uv = [[0; 28]; 2];
    for pl in 0..2 {
        if num_uv_points[pl as usize] != 0 || chroma_scaling_from_luma {
            let num_uv_pos = num_y_pos + (num_y_points != 0) as c_int;
            for i in 0..num_uv_pos {
                ar_coeffs_uv[pl as usize][i as usize] = gb.get_bits(8).wrapping_sub(128) as i8;
            }
            if num_y_points == 0 {
                ar_coeffs_uv[pl as usize][num_uv_pos as usize] = 0;
            }
        }
    }
    let ar_coeff_shift = gb.get_bits(2) as u8 + 6;
    let grain_scale_shift = gb.get_bits(2) as u8;
    let mut uv_mult = [0; 2];
    let mut uv_luma_mult = [0; 2];
    let mut uv_offset = [0; 2];
    for pl in 0..2 {
        if num_uv_points[pl as usize] != 0 {
            uv_mult[pl as usize] = gb.get_bits(8) as c_int - 128;
            uv_luma_mult[pl as usize] = gb.get_bits(8) as c_int - 128;
            uv_offset[pl as usize] = gb.get_bits(9) as c_int - 256;
        }
    }
    let overlap_flag = gb.get_bit();
    let clip_to_restricted_range = gb.get_bit();
    Ok(Rav1dFilmGrainData {
        seed,
        num_y_points,
        y_points,
        chroma_scaling_from_luma,
        num_uv_points,
        uv_points,
        scaling_shift,
        ar_coeff_lag,
        ar_coeffs_y,
        ar_coeffs_uv,
        ar_coeff_shift,
        grain_scale_shift,
        uv_mult,
        uv_luma_mult,
        uv_offset,
        overlap_flag,
        clip_to_restricted_range,
    })
}

fn parse_film_grain(
    state: &Rav1dState,
    seqhdr: &Rav1dSequenceHeader,
    show_frame: u8,
    showable_frame: u8,
    frame_type: Rav1dFrameType,
    ref_indices: &[i8; RAV1D_REFS_PER_FRAME],
    debug: &Debug,
    gb: &mut GetBits,
) -> Rav1dResult<Rav1dFrameHeaderFilmGrain> {
    let present = (seqhdr.film_grain_present != 0
        && (show_frame != 0 || showable_frame != 0)
        && gb.get_bit()) as u8;
    let update;
    let data = if present != 0 {
        let seed = gb.get_bits(16);
        update = (frame_type != Rav1dFrameType::Inter || gb.get_bit()) as u8;
        if update == 0 {
            let refidx = gb.get_bits(3) as i8;
            let mut found = false;
            for i in 0..7 {
                if ref_indices[i as usize] == refidx {
                    found = true;
                    break;
                }
            }
            if !found {
                return Err(EINVAL);
            }
            Rav1dFilmGrainData {
                seed,
                ..state.refs[refidx as usize]
                    .p
                    .p
                    .frame_hdr
                    .as_ref()
                    .ok_or(EINVAL)?
                    .film_grain
                    .data
                    .clone()
            }
        } else {
            parse_film_grain_data(seqhdr, seed, gb)?
        }
    } else {
        // Default initialization.
        update = Default::default();

        Default::default()
    };
    debug.post(gb, "filmgrain");
    Ok(Rav1dFrameHeaderFilmGrain {
        data,
        present,
        update,
    })
}

fn parse_frame_hdr(
    c: &Rav1dContext,
    state: &Rav1dState,
    seqhdr: &Rav1dSequenceHeader,
    temporal_id: u8,
    spatial_id: u8,
    gb: &mut GetBits,
) -> Rav1dResult<Rav1dFrameHeader> {
    let debug = Debug::new(false, "HDR", gb);

    debug.post(gb, "show_existing_frame");
    let show_existing_frame = (seqhdr.reduced_still_picture_header == 0 && gb.get_bit()) as u8;
    let existing_frame_idx;
    let mut frame_presentation_delay;
    if show_existing_frame != 0 {
        existing_frame_idx = gb.get_bits(3) as u8;
        if seqhdr.decoder_model_info_present != 0 && seqhdr.equal_picture_interval == 0 {
            frame_presentation_delay =
                gb.get_bits(seqhdr.frame_presentation_delay_length.into()) as u32;
        } else {
            // Default initialization.
            frame_presentation_delay = Default::default();
        }
        let frame_id;
        if seqhdr.frame_id_numbers_present != 0 {
            frame_id = gb.get_bits(seqhdr.frame_id_n_bits.into()) as u32;
            state.refs[existing_frame_idx as usize]
                .p
                .p
                .frame_hdr
                .as_ref()
                .filter(|ref_frame_hdr| ref_frame_hdr.frame_id == frame_id)
                .ok_or(EINVAL)?;
        } else {
            // Default initialization.
            frame_id = Default::default();
        }
        return Ok(Rav1dFrameHeader {
            spatial_id,
            temporal_id,
            show_existing_frame,
            existing_frame_idx,
            frame_presentation_delay,
            frame_id,
            // TODO(kkysen) I think an [`Option`] somewhere could avoid having to `#[derive(Default)]` everything.
            // There are also `enum`s that don't have a clear default other than being 0.
            ..Default::default()
        });
    } else {
        // Default initialization.
        existing_frame_idx = Default::default();
        frame_presentation_delay = Default::default();
    }

    let frame_type = if seqhdr.reduced_still_picture_header != 0 {
        Rav1dFrameType::Key
    } else {
        Rav1dFrameType::from_repr(gb.get_bits(2) as usize).unwrap()
    };
    let show_frame = (seqhdr.reduced_still_picture_header != 0 || gb.get_bit()) as u8;
    let showable_frame;
    if show_frame != 0 {
        if seqhdr.decoder_model_info_present != 0 && seqhdr.equal_picture_interval == 0 {
            frame_presentation_delay =
                gb.get_bits(seqhdr.frame_presentation_delay_length.into()) as u32;
        }
        showable_frame = (frame_type != Rav1dFrameType::Key) as u8;
    } else {
        showable_frame = gb.get_bit() as u8;
    }
    let error_resilient_mode = (frame_type == Rav1dFrameType::Key && show_frame != 0
        || frame_type == Rav1dFrameType::Switch
        || seqhdr.reduced_still_picture_header != 0
        || gb.get_bit()) as u8;
    debug.post(gb, "frametype_bits");
    let disable_cdf_update = gb.get_bit() as u8;
    let allow_screen_content_tools = match seqhdr.screen_content_tools {
        Rav1dAdaptiveBoolean::Adaptive => gb.get_bit(),
        Rav1dAdaptiveBoolean::On => true,
        Rav1dAdaptiveBoolean::Off => false,
    };
    let mut force_integer_mv = if allow_screen_content_tools {
        match seqhdr.force_integer_mv {
            Rav1dAdaptiveBoolean::Adaptive => gb.get_bit(),
            Rav1dAdaptiveBoolean::On => true,
            Rav1dAdaptiveBoolean::Off => false,
        }
    } else {
        false
    };

    if frame_type.is_key_or_intra() {
        force_integer_mv = true;
    }

    let frame_id;
    if seqhdr.frame_id_numbers_present != 0 {
        frame_id = gb.get_bits(seqhdr.frame_id_n_bits.into()) as u32;
    } else {
        // Default initialization.
        frame_id = Default::default();
    }

    let frame_size_override = if seqhdr.reduced_still_picture_header != 0 {
        false
    } else if frame_type == Rav1dFrameType::Switch {
        true
    } else {
        gb.get_bit()
    };
    debug.post(gb, "frame_size_override_flag");
    let frame_offset = if seqhdr.order_hint != 0 {
        gb.get_bits(seqhdr.order_hint_n_bits.into()) as u8
    } else {
        0
    };
    let primary_ref_frame = if error_resilient_mode == 0 && frame_type.is_inter_or_switch() {
        gb.get_bits(3) as u8
    } else {
        RAV1D_PRIMARY_REF_NONE
    };

    let buffer_removal_time_present;
    let mut operating_points =
        [Rav1dFrameHeaderOperatingPoint::default(); RAV1D_MAX_OPERATING_POINTS];
    if seqhdr.decoder_model_info_present != 0 {
        buffer_removal_time_present = gb.get_bit() as u8;
        if buffer_removal_time_present != 0 {
            for i in 0..seqhdr.num_operating_points {
                let seqop = &seqhdr.operating_points[i as usize];
                let op = &mut operating_points[i as usize];
                if seqop.decoder_model_param_present != 0 {
                    let in_temporal_layer = seqop.idc >> temporal_id & 1;
                    let in_spatial_layer = seqop.idc >> spatial_id + 8 & 1;
                    if seqop.idc == 0 || in_temporal_layer != 0 && in_spatial_layer != 0 {
                        op.buffer_removal_time =
                            gb.get_bits(seqhdr.buffer_removal_delay_length.into()) as u32;
                    }
                }
            }
        }
    } else {
        // Default initialization.
        buffer_removal_time_present = Default::default();
    }

    let refresh_frame_flags;
    let size;
    let refidx;
    let allow_intrabc;
    let use_ref_frame_mvs;
    let frame_ref_short_signaling;
    let hp;
    let subpel_filter_mode;
    let switchable_motion_mode;
    if frame_type.is_key_or_intra() {
        refresh_frame_flags = if frame_type == Rav1dFrameType::Key && show_frame != 0 {
            0xff
        } else {
            gb.get_bits(8) as u8
        };
        if refresh_frame_flags != 0xff && error_resilient_mode != 0 && seqhdr.order_hint != 0 {
            for _ in 0..8 {
                gb.get_bits(seqhdr.order_hint_n_bits.into());
            }
        }
        if c.strict_std_compliance
            && frame_type == Rav1dFrameType::Intra
            && refresh_frame_flags == 0xff
        {
            return Err(EINVAL);
        }
        size = parse_frame_size(state, seqhdr, None, frame_size_override, gb)?;
        allow_intrabc = allow_screen_content_tools && !size.super_res.enabled && gb.get_bit();
        use_ref_frame_mvs = 0;

        // Default initialization.
        refidx = Default::default();
        frame_ref_short_signaling = Default::default();
        hp = Default::default();
        subpel_filter_mode = Rav1dFilterMode::Regular8Tap;
        switchable_motion_mode = Default::default();
    } else {
        allow_intrabc = false;
        refresh_frame_flags = if frame_type == Rav1dFrameType::Switch {
            0xff
        } else {
            gb.get_bits(8) as u8
        };
        if error_resilient_mode != 0 && seqhdr.order_hint != 0 {
            for _ in 0..8 {
                gb.get_bits(seqhdr.order_hint_n_bits.into());
            }
        }
        frame_ref_short_signaling = (seqhdr.order_hint != 0 && gb.get_bit()) as u8;
        refidx = parse_refidx(
            state,
            seqhdr,
            frame_ref_short_signaling,
            frame_offset,
            frame_id,
            gb,
        )?;
        let use_ref = error_resilient_mode == 0 && frame_size_override;
        size = parse_frame_size(
            state,
            seqhdr,
            Some(&refidx).filter(|_| use_ref),
            frame_size_override,
            gb,
        )?;
        hp = !force_integer_mv && gb.get_bit();
        subpel_filter_mode = if gb.get_bit() {
            Rav1dFilterMode::Switchable
        } else {
            Rav1dFilterMode::from_repr(gb.get_bits(2) as usize).unwrap()
        };
        switchable_motion_mode = gb.get_bit() as u8;
        use_ref_frame_mvs = (error_resilient_mode == 0
            && seqhdr.ref_frame_mvs != 0
            && seqhdr.order_hint != 0
            && frame_type.is_inter_or_switch()
            && gb.get_bit()) as u8;
    }
    debug.post(gb, "frametype-specific-bits");

    let refresh_context = (seqhdr.reduced_still_picture_header == 0
        && disable_cdf_update == 0
        && !gb.get_bit()) as u8;
    debug.post(gb, "refresh_context");

    let tiling = parse_tiling(seqhdr, &size, &debug, gb)?;
    let quant = parse_quant(seqhdr, &debug, gb);
    let segmentation = parse_segmentation(state, primary_ref_frame, &refidx, &quant, &debug, gb)?;
    let all_lossless = segmentation.lossless.iter().all(|&it| it);
    let delta = parse_delta(&quant, allow_intrabc, &debug, gb);
    let loopfilter = parse_loopfilter(
        state,
        seqhdr,
        all_lossless,
        allow_intrabc,
        primary_ref_frame,
        &refidx,
        &debug,
        gb,
    )?;
    let cdef = parse_cdef(seqhdr, all_lossless, allow_intrabc, &debug, gb);
    let restoration = parse_restoration(
        seqhdr,
        all_lossless,
        size.super_res.enabled,
        allow_intrabc,
        &debug,
        gb,
    );

    let txfm_mode = if all_lossless {
        Rav1dTxfmMode::Only4x4
    } else if gb.get_bit() {
        Rav1dTxfmMode::Switchable
    } else {
        Rav1dTxfmMode::Largest
    };
    debug.post(gb, "txfmmode");
    let switchable_comp_refs = if frame_type.is_inter_or_switch() {
        gb.get_bit() as u8
    } else {
        0
    };
    debug.post(gb, "refmode");
    let skip_mode = parse_skip_mode(
        state,
        seqhdr,
        switchable_comp_refs,
        frame_type,
        frame_offset,
        &refidx,
        &debug,
        gb,
    )?;
    let warp_motion = (error_resilient_mode == 0
        && frame_type.is_inter_or_switch()
        && seqhdr.warped_motion != 0
        && gb.get_bit()) as u8;
    debug.post(gb, "warpmotionbit");
    let reduced_txtp_set = gb.get_bit() as u8;
    debug.post(gb, "reducedtxtpset");

    let gmv = parse_gmv(
        state,
        frame_type,
        primary_ref_frame,
        &refidx,
        hp,
        &debug,
        gb,
    )?;
    let film_grain = parse_film_grain(
        state,
        seqhdr,
        show_frame,
        showable_frame,
        frame_type,
        &refidx,
        &debug,
        gb,
    )?;

    Ok(Rav1dFrameHeader {
        size,
        film_grain,
        frame_type,
        frame_offset,
        temporal_id,
        spatial_id,
        show_existing_frame,
        existing_frame_idx,
        frame_id,
        frame_presentation_delay,
        show_frame,
        showable_frame,
        error_resilient_mode,
        disable_cdf_update,
        allow_screen_content_tools,
        force_integer_mv,
        frame_size_override,
        primary_ref_frame,
        buffer_removal_time_present,
        operating_points,
        refresh_frame_flags,
        allow_intrabc,
        frame_ref_short_signaling,
        refidx,
        hp,
        subpel_filter_mode,
        switchable_motion_mode,
        use_ref_frame_mvs,
        refresh_context,
        tiling,
        quant,
        segmentation,
        delta,
        all_lossless,
        loopfilter,
        cdef,
        restoration,
        txfm_mode,
        switchable_comp_refs,
        skip_mode,
        warp_motion,
        reduced_txtp_set,
        gmv,
    })
}

fn parse_tile_hdr(tiling: &Rav1dFrameHeaderTiling, gb: &mut GetBits) -> Rav1dTileGroupHeader {
    let n_tiles = tiling.cols as c_int * tiling.rows as c_int;
    let have_tile_pos = if n_tiles > 1 {
        gb.get_bit() as c_int
    } else {
        0
    };

    if have_tile_pos != 0 {
        let n_bits = tiling.log2_cols + tiling.log2_rows;
        let start = gb.get_bits(n_bits.into()) as c_int;
        let end = gb.get_bits(n_bits.into()) as c_int;
        Rav1dTileGroupHeader { start, end }
    } else {
        Rav1dTileGroupHeader {
            start: 0,
            end: n_tiles - 1,
        }
    }
}

fn parse_obus(
    c: &Rav1dContext,
    state: &mut Rav1dState,
    r#in: &CArc<[u8]>,
    props: &Rav1dDataProps,
    gb: &mut GetBits,
) -> Rav1dResult<()> {
    fn skip(state: &mut Rav1dState) {
        // update refs with only the headers in case we skip the frame
        for i in 0..8 {
            if state.frame_hdr.as_ref().unwrap().refresh_frame_flags & (1 << i) != 0 {
                let _ = mem::take(&mut state.refs[i as usize].p);
                state.refs[i as usize].p.p.frame_hdr = state.frame_hdr.clone();
                state.refs[i as usize].p.p.seq_hdr = state.seq_hdr.clone();
            }
        }

        let _ = mem::take(&mut state.frame_hdr);
        state.n_tiles = 0;
    }

    // obu header
    let obu_forbidden_bit = gb.get_bit();
    if c.strict_std_compliance && obu_forbidden_bit {
        return Err(EINVAL);
    }
    let raw_type = gb.get_bits(4);
    let r#type = Rav1dObuType::from_repr(raw_type as usize);
    let has_extension = gb.get_bit();
    let has_length_field = gb.get_bit();
    gb.get_bit(); // reserved

    let mut temporal_id = 0;
    let mut spatial_id = 0;
    if has_extension {
        temporal_id = gb.get_bits(3) as u8;
        spatial_id = gb.get_bits(2) as u8;
        gb.get_bits(3); // reserved
    }

    // obu length field
    if has_length_field {
        let len = gb.get_uleb128() as usize;
        gb.set_remaining_len(len).ok_or(EINVAL)?;
    }
    if gb.has_error() != 0 {
        return Err(EINVAL);
    }

    // We must have read a whole number of bytes at this point
    // (1 byte for the header and whole bytes at a time
    // when reading the leb128 length field).

    assert!(gb.is_byte_aligned());

    // skip obu not belonging to the selected temporal/spatial layer
    if !matches!(r#type, Some(Rav1dObuType::SeqHdr | Rav1dObuType::Td))
        && has_extension
        && state.operating_point_idc != 0
    {
        let in_temporal_layer = (state.operating_point_idc >> temporal_id & 1) as c_int;
        let in_spatial_layer = (state.operating_point_idc >> spatial_id + 8 & 1) as c_int;
        if in_temporal_layer == 0 || in_spatial_layer == 0 {
            return Ok(());
        }
    }

    fn parse_tile_grp(
        state: &mut Rav1dState,
        r#in: &CArc<[u8]>,
        props: &Rav1dDataProps,
        gb: &mut GetBits,
    ) -> Rav1dResult {
        let hdr = parse_tile_hdr(&state.frame_hdr.as_ref().ok_or(EINVAL)?.tiling, gb);
        // Align to the next byte boundary and check for overrun.
        gb.bytealign();
        if gb.has_error() != 0 {
            return Err(EINVAL);
        }

        let mut data = r#in.clone();
        data.slice_in_place(gb.byte_pos()..);
        data.slice_in_place(..gb.remaining_len());
        // Ensure tile groups are in order and sane; see 6.10.1.
        if hdr.start > hdr.end || hdr.start != state.n_tiles {
            state.tiles.clear();
            state.n_tiles = 0;
            return Err(EINVAL);
        }
        if let Err(_) = state.tiles.try_reserve_exact(1) {
            return Err(EINVAL);
        }
        state.n_tiles += 1 + hdr.end - hdr.start;
        state.tiles.push(Rav1dTileGroup {
            data: Rav1dData {
                data: Some(data),
                // TODO(kkysen) Are props needed here?
                // Also, if it's not needed, we don't need the `Option` for `CArc<[u8]>` either.
                m: props.clone(),
            },
            hdr,
        });

        Ok(())
    }

    match r#type {
        Some(Rav1dObuType::SeqHdr) => {
            let seq_hdr = parse_seq_hdr(gb, c.strict_std_compliance).inspect_err(|_| {
                writeln!(c.logger, "Error parsing sequence header");
            })?;
            if gb.has_error() != 0 {
                return Err(EINVAL);
            }

            let op_idx = if c.operating_point < seq_hdr.num_operating_points {
                c.operating_point
            } else {
                0
            };
            state.operating_point_idc = seq_hdr.operating_points[op_idx as usize].idc as c_uint;
            let spatial_mask = state.operating_point_idc >> 8;
            state.max_spatial_id = if spatial_mask != 0 {
                ulog2(spatial_mask) as u8
            } else {
                0
            };

            // If we have read a sequence header which is different from the old one,
            // this is a new video sequence and can't use any previous state.
            // Free that state.

            match &state.seq_hdr {
                None => {
                    state.frame_hdr = None;
                    state.frame_flags |= PictureFlags::NEW_SEQUENCE;
                }
                Some(c_seq_hdr) if !seq_hdr.eq_without_operating_parameter_info(&c_seq_hdr) => {
                    // See 7.5, `operating_parameter_info` is allowed to change in
                    // sequence headers of a single sequence.
                    state.frame_hdr = None;
                    let _ = mem::take(&mut state.content_light);
                    let _ = mem::take(&mut state.mastering_display);
                    for i in 0..8 {
                        if state.refs[i as usize].p.p.frame_hdr.is_some() {
                            let _ = mem::take(&mut state.refs[i as usize].p);
                        }
                        let _ = mem::take(&mut state.refs[i as usize].segmap);
                        let _ = mem::take(&mut state.refs[i as usize].refmvs);
                        let _ = mem::take(&mut state.cdf[i]);
                    }
                    state.frame_flags |= PictureFlags::NEW_SEQUENCE;
                }
                Some(c_seq_hdr)
                    if seq_hdr.operating_parameter_info != c_seq_hdr.operating_parameter_info =>
                {
                    // If operating_parameter_info changed, signal it
                    state.frame_flags |= PictureFlags::NEW_OP_PARAMS_INFO;
                }
                _ => {}
            }
            state.seq_hdr = Some(Arc::new(DRav1d::from_rav1d(seq_hdr))); // TODO(kkysen) fallible allocation
        }
        Some(Rav1dObuType::RedundantFrameHdr) if state.frame_hdr.is_some() => {}
        Some(Rav1dObuType::RedundantFrameHdr | Rav1dObuType::Frame | Rav1dObuType::FrameHdr) => {
            state.frame_hdr = None;
            // TODO(kkysen) C originally re-used this allocation,
            // but it was also pooling, which we've dropped for now.

            let frame_hdr = parse_frame_hdr(
                c,
                state,
                state.seq_hdr.as_ref().ok_or(EINVAL)?,
                temporal_id,
                spatial_id,
                gb,
            )
            .inspect_err(|_| writeln!(c.logger, "Error parsing frame header"))?;

            state.tiles.clear();
            state.n_tiles = 0;
            if r#type != Some(Rav1dObuType::Frame) {
                // This is actually a frame header OBU,
                // so read the trailing bit and check for overrun.
                check_trailing_bits(gb, c.strict_std_compliance)?;
            }

            if c.frame_size_limit != 0
                && frame_hdr.size.width[1] as i64 * frame_hdr.size.height as i64
                    > c.frame_size_limit as i64
            {
                writeln!(
                    c.logger,
                    "Frame size {}x{} exceeds limit {}",
                    frame_hdr.size.width[1], frame_hdr.size.height, c.frame_size_limit,
                );
                return Err(ERANGE);
            }

            if r#type == Some(Rav1dObuType::Frame) {
                // OBU_FRAMEs shouldn't be signaled with `show_existing_frame`.
                if frame_hdr.show_existing_frame != 0 {
                    return Err(EINVAL);
                }
            }

            state.frame_hdr = Some(Arc::new(DRav1d::from_rav1d(frame_hdr))); // TODO(kkysen) fallible allocation

            if r#type == Some(Rav1dObuType::Frame) {
                // This is the frame header at the start of a frame OBU.
                // There's no trailing bit at the end to skip,
                // but we do need to align to the next byte.
                gb.bytealign();
                parse_tile_grp(state, r#in, props, gb)?;
            }
        }
        Some(Rav1dObuType::TileGrp) => {
            parse_tile_grp(state, r#in, props, gb)?;
        }
        Some(Rav1dObuType::Metadata) => {
            let debug = Debug::new(false, "OBU", &gb);

            // obu metadata type field
            let meta_type = gb.get_uleb128();
            if gb.has_error() != 0 {
                return Err(EINVAL);
            }

            match ObuMetaType::from_repr(meta_type as usize) {
                Some(ObuMetaType::HdrCll) => {
                    let debug = debug.named("CLLOBU");
                    let max_content_light_level = gb.get_bits(16) as u16;
                    debug.log(
                        &gb,
                        format_args!("max-content-light-level: {max_content_light_level}"),
                    );
                    let max_frame_average_light_level = gb.get_bits(16) as u16;
                    debug.log(
                        &gb,
                        format_args!(
                            "max-frame-average-light-level: {max_frame_average_light_level}"
                        ),
                    );

                    check_trailing_bits(gb, c.strict_std_compliance)?;

                    state.content_light = Some(Arc::new(Rav1dContentLightLevel {
                        max_content_light_level,
                        max_frame_average_light_level,
                    })); // TODO(kkysen) fallible allocation
                }
                Some(ObuMetaType::HdrMdcv) => {
                    let debug = debug.named("MDCVOBU");
                    let primaries = array::from_fn(|i| {
                        let primary = [gb.get_bits(16) as u16, gb.get_bits(16) as u16];
                        debug.log(&gb, format_args!("primaries[{i}]: {primary:?}"));
                        primary
                    });
                    let white_point_x = gb.get_bits(16) as u16;
                    debug.log(&gb, format_args!("white-point-x: {white_point_x}"));
                    let white_point_y = gb.get_bits(16) as u16;
                    debug.log(&gb, format_args!("white-point-y: {white_point_y}"));
                    let white_point = [white_point_x, white_point_y];
                    let max_luminance = gb.get_bits(32);
                    debug.log(&gb, format_args!("max-luminance: {max_luminance}"));
                    let min_luminance = gb.get_bits(32);
                    debug.log(&gb, format_args!("min-luminance: {min_luminance}"));
                    check_trailing_bits(gb, c.strict_std_compliance)?;

                    state.mastering_display = Some(Arc::new(Rav1dMasteringDisplay {
                        primaries,
                        white_point,
                        max_luminance,
                        min_luminance,
                    })); // TODO(kkysen) fallible allocation
                }
                Some(ObuMetaType::ItutT35) => {
                    let mut payload_size = gb.remaining_len();
                    // Don't take into account all the trailing bits for `payload_size`.
                    while payload_size > 0 && gb[payload_size as usize - 1] == 0 {
                        payload_size -= 1; // trailing_zero_bit x 8
                    }
                    payload_size -= 1; // trailing_one_bit + trailing_zero_bit x 7

                    let mut country_code_extension_byte = 0;
                    let country_code = gb.get_bits(8) as c_int;
                    payload_size -= 1;
                    if country_code == 0xff {
                        country_code_extension_byte = gb.get_bits(8) as c_int;
                        payload_size -= 1;
                    }

                    if payload_size <= 0 || gb[payload_size] != 0x80 {
                        writeln!(c.logger, "Malformed ITU-T T.35 metadata message format");
                    } else {
                        let country_code = country_code as u8;
                        let country_code_extension_byte = country_code_extension_byte as u8;
                        let payload = gb.get_bytes(payload_size as usize).into(); // TODO fallible allocation
                        let itut_t35 = Rav1dITUTT35 {
                            country_code,
                            country_code_extension_byte,
                            payload,
                        };
                        state.itut_t35.push(itut_t35); // TODO fallible allocation
                    }
                }
                Some(ObuMetaType::Scalability | ObuMetaType::Timecode) => {} // Ignore metadata OBUs we don't care about.
                None => {
                    // Print a warning, but don't fail for unknown types.
                    writeln!(c.logger, "Unknown Metadata OBU type {meta_type}");
                }
            }
        }
        Some(Rav1dObuType::Td) => state.frame_flags |= PictureFlags::NEW_TEMPORAL_UNIT,
        Some(Rav1dObuType::Padding) => {} // Ignore OBUs we don't care about.
        None => {
            // Print a warning, but don't fail for unknown types.
            let len = gb.remaining_len();
            writeln!(c.logger, "Unknown OBU type {raw_type} of size {len}");
        }
    }

    if let (Some(_), Some(frame_hdr)) = (state.seq_hdr.as_ref(), state.frame_hdr.as_ref()) {
        let frame_hdr = &***frame_hdr;
        if frame_hdr.show_existing_frame != 0 {
            match state.refs[frame_hdr.existing_frame_idx as usize]
                .p
                .p
                .frame_hdr
                .as_ref()
                .ok_or(EINVAL)?
                .frame_type
            {
                Rav1dFrameType::Inter | Rav1dFrameType::Switch => {
                    if c.decode_frame_type > Rav1dDecodeFrameType::Reference {
                        return Ok(skip(state));
                    }
                }
                Rav1dFrameType::Intra => {
                    if c.decode_frame_type > Rav1dDecodeFrameType::Intra {
                        return Ok(skip(state));
                    }
                }
                _ => {}
            }
            if state.refs[frame_hdr.existing_frame_idx as usize]
                .p
                .p
                .data
                .is_none()
            {
                return Err(EINVAL);
            }
            if c.strict_std_compliance
                && !state.refs[frame_hdr.existing_frame_idx as usize].p.showable
            {
                return Err(EINVAL);
            }
            if c.fc.len() == 1 {
                state.out = state.refs[frame_hdr.existing_frame_idx as usize].p.clone();
                rav1d_picture_copy_props(
                    &mut state.out.p,
                    state.content_light.clone(),
                    state.mastering_display.clone(),
                    // Must be moved from the context to the frame.
                    Rav1dITUTT35::to_immut(mem::take(&mut state.itut_t35)),
                    props.clone(),
                );
                state.event_flags |= state.refs[frame_hdr.existing_frame_idx as usize]
                    .p
                    .flags
                    .into();
            } else {
                let mut task_thread_lock = c.task_thread.lock.lock();
                // Need to append this to the frame output queue.
                let next = state.frame_thread.next;
                state.frame_thread.next = (state.frame_thread.next + 1) % c.fc.len() as u32;

                let fc = &c.fc[next as usize];
                while !fc.task_thread.finished.load(Ordering::SeqCst) {
                    fc.task_thread.cond.wait(&mut task_thread_lock);
                }
                let out_delayed = &mut state.frame_thread.out_delayed[next as usize];
                if out_delayed.p.data.is_some() || fc.task_thread.error.load(Ordering::SeqCst) != 0
                {
                    let first = c.task_thread.first.load(Ordering::SeqCst);
                    if first as usize + 1 < c.fc.len() {
                        c.task_thread.first.fetch_add(1, Ordering::SeqCst);
                    } else {
                        c.task_thread.first.store(0, Ordering::SeqCst);
                    }
                    let _ = c.task_thread.reset_task_cur.compare_exchange(
                        first,
                        u32::MAX,
                        Ordering::SeqCst,
                        Ordering::SeqCst,
                    );
                    if c.task_thread.cur.get() != 0
                        && (c.task_thread.cur.get() as usize) < c.fc.len()
                    {
                        c.task_thread.cur.update(|cur| cur - 1);
                    }
                }
                let error = &mut *fc.task_thread.retval.try_lock().unwrap();
                if error.is_some() {
                    state.cached_error = mem::take(error);
                    state.cached_error_props = out_delayed.p.m.clone();
                    let _ = mem::take(out_delayed);
                } else if out_delayed.p.data.is_some() {
                    let progress =
                        out_delayed.progress.as_ref().unwrap()[1].load(Ordering::Relaxed);
                    if (out_delayed.visible || c.output_invisible_frames) && progress != FRAME_ERROR
                    {
                        state.out = out_delayed.clone();
                        state.event_flags |= out_delayed.flags.into();
                    }
                    let _ = mem::take(out_delayed);
                }
                *out_delayed = state.refs[frame_hdr.existing_frame_idx as usize].p.clone();
                out_delayed.visible = true;
                rav1d_picture_copy_props(
                    &mut out_delayed.p,
                    state.content_light.clone(),
                    state.mastering_display.clone(),
                    // Must be moved from the context to the frame.
                    Rav1dITUTT35::to_immut(mem::take(&mut state.itut_t35)),
                    props.clone(),
                );
            }
            if state.refs[frame_hdr.existing_frame_idx as usize]
                .p
                .p
                .frame_hdr
                .as_ref()
                .unwrap()
                .frame_type
                == Rav1dFrameType::Key
            {
                let r = frame_hdr.existing_frame_idx;
                state.refs[r as usize].p.showable = false;
                for i in 0..8 {
                    if i == r {
                        continue;
                    }

                    if state.refs[i as usize].p.p.frame_hdr.is_some() {
                        let _ = mem::take(&mut state.refs[i as usize].p);
                    }
                    state.refs[i as usize].p = state.refs[r as usize].p.clone();

                    state.cdf[i as usize] = state.cdf[r as usize].clone();

                    state.refs[i as usize].segmap = state.refs[r as usize].segmap.clone();
                    let _ = mem::take(&mut state.refs[i as usize].refmvs);
                }
            }
            state.frame_hdr = None;
        } else if state.n_tiles == frame_hdr.tiling.cols as c_int * frame_hdr.tiling.rows as c_int {
            match frame_hdr.frame_type {
                Rav1dFrameType::Inter | Rav1dFrameType::Switch => {
                    if c.decode_frame_type > Rav1dDecodeFrameType::Reference
                        || c.decode_frame_type == Rav1dDecodeFrameType::Reference
                            && frame_hdr.refresh_frame_flags == 0
                    {
                        return Ok(skip(state));
                    }
                }
                Rav1dFrameType::Intra => {
                    if c.decode_frame_type > Rav1dDecodeFrameType::Intra
                        || c.decode_frame_type == Rav1dDecodeFrameType::Reference
                            && frame_hdr.refresh_frame_flags == 0
                    {
                        return Ok(skip(state));
                    }
                }
                _ => {}
            }
            if state.tiles.is_empty() {
                return Err(EINVAL);
            }
            rav1d_submit_frame(c, state)?;
            assert!(state.tiles.is_empty());
            state.frame_hdr = None;
            state.n_tiles = 0;
        }
    }

    Ok(())
}

pub(crate) fn rav1d_parse_obus(
    c: &Rav1dContext,
    state: &mut Rav1dState,
    r#in: &CArc<[u8]>,
    props: &Rav1dDataProps,
) -> Rav1dResult<usize> {
    let gb = &mut GetBits::new(r#in);

    parse_obus(c, state, r#in, props, gb)
        .inspect_err(|_| {
            state.cached_error_props = props.clone();
            writeln!(
                c.logger,
                "{}",
                if gb.has_error() != 0 {
                    "Overrun in OBU bit buffer"
                } else {
                    "Error parsing OBU data"
                }
            );
        })
        .map(|_| gb.len())
}
