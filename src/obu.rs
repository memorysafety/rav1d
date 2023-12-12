use crate::include::common::intops::iclip_u8;
use crate::include::common::intops::ulog2;
use crate::include::dav1d::data::Rav1dData;
use crate::include::dav1d::dav1d::RAV1D_DECODEFRAMETYPE_INTRA;
use crate::include::dav1d::dav1d::RAV1D_DECODEFRAMETYPE_REFERENCE;
use crate::include::dav1d::headers::DRav1d;
use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::dav1d::headers::Dav1dITUTT35;
use crate::include::dav1d::headers::Dav1dSequenceHeader;
use crate::include::dav1d::headers::Rav1dAdaptiveBoolean;
use crate::include::dav1d::headers::Rav1dChromaSamplePosition;
use crate::include::dav1d::headers::Rav1dColorPrimaries;
use crate::include::dav1d::headers::Rav1dContentLightLevel;
use crate::include::dav1d::headers::Rav1dFilmGrainData;
use crate::include::dav1d::headers::Rav1dFilterMode;
use crate::include::dav1d::headers::Rav1dFrameHeader;
use crate::include::dav1d::headers::Rav1dFrameHeader_delta;
use crate::include::dav1d::headers::Rav1dFrameHeader_delta_lf;
use crate::include::dav1d::headers::Rav1dFrameHeader_delta_q;
use crate::include::dav1d::headers::Rav1dFrameHeader_loopfilter;
use crate::include::dav1d::headers::Rav1dFrameHeader_quant;
use crate::include::dav1d::headers::Rav1dFrameHeader_segmentation;
use crate::include::dav1d::headers::Rav1dFrameHeader_super_res;
use crate::include::dav1d::headers::Rav1dFrameHeader_tiling;
use crate::include::dav1d::headers::Rav1dFrameSize;
use crate::include::dav1d::headers::Rav1dFrameType;
use crate::include::dav1d::headers::Rav1dITUTT35;
use crate::include::dav1d::headers::Rav1dLoopfilterModeRefDeltas;
use crate::include::dav1d::headers::Rav1dMasteringDisplay;
use crate::include::dav1d::headers::Rav1dMatrixCoefficients;
use crate::include::dav1d::headers::Rav1dObuType;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::include::dav1d::headers::Rav1dRestorationType;
use crate::include::dav1d::headers::Rav1dSegmentationData;
use crate::include::dav1d::headers::Rav1dSegmentationDataSet;
use crate::include::dav1d::headers::Rav1dSequenceHeader;
use crate::include::dav1d::headers::Rav1dSequenceHeaderOperatingParameterInfo;
use crate::include::dav1d::headers::Rav1dSequenceHeaderOperatingPoint;
use crate::include::dav1d::headers::Rav1dTransferCharacteristics;
use crate::include::dav1d::headers::RAV1D_ADAPTIVE;
use crate::include::dav1d::headers::RAV1D_CHR_UNKNOWN;
use crate::include::dav1d::headers::RAV1D_COLOR_PRI_BT709;
use crate::include::dav1d::headers::RAV1D_COLOR_PRI_UNKNOWN;
use crate::include::dav1d::headers::RAV1D_FILTER_SWITCHABLE;
use crate::include::dav1d::headers::RAV1D_MAX_OPERATING_POINTS;
use crate::include::dav1d::headers::RAV1D_MAX_TILE_COLS;
use crate::include::dav1d::headers::RAV1D_MAX_TILE_ROWS;
use crate::include::dav1d::headers::RAV1D_MC_IDENTITY;
use crate::include::dav1d::headers::RAV1D_MC_UNKNOWN;
use crate::include::dav1d::headers::RAV1D_OBU_FRAME;
use crate::include::dav1d::headers::RAV1D_OBU_FRAME_HDR;
use crate::include::dav1d::headers::RAV1D_OBU_METADATA;
use crate::include::dav1d::headers::RAV1D_OBU_PADDING;
use crate::include::dav1d::headers::RAV1D_OBU_REDUNDANT_FRAME_HDR;
use crate::include::dav1d::headers::RAV1D_OBU_SEQ_HDR;
use crate::include::dav1d::headers::RAV1D_OBU_TD;
use crate::include::dav1d::headers::RAV1D_OBU_TILE_GRP;
use crate::include::dav1d::headers::RAV1D_PRIMARY_REF_NONE;
use crate::include::dav1d::headers::RAV1D_REFS_PER_FRAME;
use crate::include::dav1d::headers::RAV1D_RESTORATION_NONE;
use crate::include::dav1d::headers::RAV1D_TRC_SRGB;
use crate::include::dav1d::headers::RAV1D_TRC_UNKNOWN;
use crate::include::dav1d::headers::RAV1D_TX_4X4_ONLY;
use crate::include::dav1d::headers::RAV1D_TX_LARGEST;
use crate::include::dav1d::headers::RAV1D_TX_SWITCHABLE;
use crate::include::dav1d::headers::RAV1D_WM_TYPE_AFFINE;
use crate::include::dav1d::headers::RAV1D_WM_TYPE_IDENTITY;
use crate::include::dav1d::headers::RAV1D_WM_TYPE_ROT_ZOOM;
use crate::include::dav1d::headers::RAV1D_WM_TYPE_TRANSLATION;
use crate::include::stdatomic::atomic_int;
use crate::include::stdatomic::atomic_uint;
use crate::src::cdf::rav1d_cdf_thread_ref;
use crate::src::cdf::rav1d_cdf_thread_unref;
use crate::src::data::rav1d_data_props_copy;
use crate::src::data::rav1d_data_ref;
use crate::src::data::rav1d_data_unref_internal;
use crate::src::decode::rav1d_submit_frame;
use crate::src::env::get_poc_diff;
use crate::src::error::Rav1dError::EINVAL;
use crate::src::error::Rav1dError::ENOMEM;
use crate::src::error::Rav1dError::ERANGE;
use crate::src::error::Rav1dResult;
use crate::src::getbits::rav1d_bytealign_get_bits;
use crate::src::getbits::rav1d_get_bit;
use crate::src::getbits::rav1d_get_bits;
use crate::src::getbits::rav1d_get_bits_subexp;
use crate::src::getbits::rav1d_get_sbits;
use crate::src::getbits::rav1d_get_uleb128;
use crate::src::getbits::rav1d_get_uniform;
use crate::src::getbits::rav1d_get_vlc;
use crate::src::getbits::rav1d_init_get_bits;
use crate::src::getbits::GetBits;
use crate::src::internal::Rav1dContext;
use crate::src::internal::Rav1dTileGroup;
use crate::src::internal::Rav1dTileGroupHeader;
use crate::src::levels::ObuMetaType;
use crate::src::levels::OBU_META_HDR_CLL;
use crate::src::levels::OBU_META_HDR_MDCV;
use crate::src::levels::OBU_META_ITUT_T35;
use crate::src::levels::OBU_META_SCALABILITY;
use crate::src::levels::OBU_META_TIMECODE;
use crate::src::log::Rav1dLog as _;
use crate::src::picture::rav1d_picture_copy_props;
use crate::src::picture::rav1d_picture_get_event_flags;
use crate::src::picture::rav1d_thread_picture_ref;
use crate::src::picture::rav1d_thread_picture_unref;
use crate::src::picture::PICTURE_FLAG_NEW_OP_PARAMS_INFO;
use crate::src::picture::PICTURE_FLAG_NEW_SEQUENCE;
use crate::src::picture::PICTURE_FLAG_NEW_TEMPORAL_UNIT;
use crate::src::r#ref::rav1d_ref_create;
use crate::src::r#ref::rav1d_ref_create_using_pool;
use crate::src::r#ref::rav1d_ref_dec;
use crate::src::r#ref::rav1d_ref_inc;
use crate::src::r#ref::rav1d_ref_is_writable;
use crate::src::tables::dav1d_default_wm_params;
use crate::src::thread_task::FRAME_ERROR;
use libc::memset;
use libc::pthread_cond_wait;
use libc::pthread_mutex_lock;
use libc::pthread_mutex_unlock;
use std::array;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_void;
use std::fmt;
use std::mem::MaybeUninit;

struct Debug {
    enabled: bool,
    name: &'static str,
    init_ptr: *const u8,
}

impl Debug {
    pub const fn new(enabled: bool, name: &'static str, gb: &GetBits) -> Self {
        Self {
            enabled,
            name,
            init_ptr: gb.ptr,
        }
    }

    const fn named(&self, name: &'static str) -> Self {
        let &Self {
            enabled,
            name: _,
            init_ptr,
        } = self;
        Self {
            enabled,
            name,
            init_ptr,
        }
    }

    pub unsafe fn log(&self, gb: &GetBits, msg: fmt::Arguments) {
        let &Self {
            enabled,
            name,
            init_ptr,
        } = self;
        if !enabled {
            return;
        }
        let offset = gb.ptr.offset_from(init_ptr) * 8 - gb.bits_left as isize;
        println!("{name}: {msg} [off={offset}]");
    }

    pub unsafe fn post(&self, gb: &GetBits, post: &str) {
        self.log(gb, format_args!("post-{post}"));
    }
}

#[inline]
unsafe fn rav1d_get_bits_pos(c: &GetBits) -> c_uint {
    c.ptr.offset_from(c.ptr_start) as c_uint * 8 - c.bits_left as c_uint
}

unsafe fn parse_seq_hdr(
    c: &mut Rav1dContext,
    gb: &mut GetBits,
) -> Rav1dResult<Rav1dSequenceHeader> {
    let debug = Debug::new(false, "SEQHDR", gb);

    let profile = rav1d_get_bits(gb, 3) as c_int;
    if profile > 2 {
        return Err(EINVAL);
    }
    debug.post(gb, "post-profile");

    let still_picture = rav1d_get_bit(gb) as c_int;
    let reduced_still_picture_header = rav1d_get_bit(gb) as c_int;
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
        operating_points[0].major_level = rav1d_get_bits(gb, 3) as c_int;
        operating_points[0].minor_level = rav1d_get_bits(gb, 2) as c_int;
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
        timing_info_present = rav1d_get_bit(gb) as c_int;
        if timing_info_present != 0 {
            num_units_in_tick = rav1d_get_bits(gb, 32) as c_int;
            time_scale = rav1d_get_bits(gb, 32) as c_int;
            if c.strict_std_compliance && (num_units_in_tick == 0 || time_scale == 0) {
                return Err(EINVAL);
            }
            equal_picture_interval = rav1d_get_bit(gb) as c_int;
            if equal_picture_interval != 0 {
                let num_ticks_per_picture_ = rav1d_get_vlc(gb);
                if num_ticks_per_picture_ == 0xffffffff {
                    return Err(EINVAL);
                }
                num_ticks_per_picture = num_ticks_per_picture_ + 1;
            } else {
                // Default initialization.
                num_ticks_per_picture = Default::default();
            }

            decoder_model_info_present = rav1d_get_bit(gb) as c_int;
            if decoder_model_info_present != 0 {
                encoder_decoder_buffer_delay_length = rav1d_get_bits(gb, 5) as c_int + 1;
                num_units_in_decoding_tick = rav1d_get_bits(gb, 32) as c_int;
                if c.strict_std_compliance && num_units_in_decoding_tick == 0 {
                    return Err(EINVAL);
                }
                buffer_removal_delay_length = rav1d_get_bits(gb, 5) as c_int + 1;
                frame_presentation_delay_length = rav1d_get_bits(gb, 5) as c_int + 1;
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

        display_model_info_present = rav1d_get_bit(gb) as c_int;
        num_operating_points = rav1d_get_bits(gb, 5) as c_int + 1;
        for i in 0..num_operating_points {
            let op = &mut operating_points[i as usize];
            op.idc = rav1d_get_bits(gb, 12) as c_int;
            if op.idc != 0 && (op.idc & 0xff == 0 || op.idc & 0xf00 == 0) {
                return Err(EINVAL);
            }
            op.major_level = 2 + rav1d_get_bits(gb, 3) as c_int;
            op.minor_level = rav1d_get_bits(gb, 2) as c_int;
            if op.major_level > 3 {
                op.tier = rav1d_get_bit(gb) as c_int;
            }
            if decoder_model_info_present != 0 {
                op.decoder_model_param_present = rav1d_get_bit(gb) as c_int;
                if op.decoder_model_param_present != 0 {
                    let opi = &mut operating_parameter_info[i as usize];
                    opi.decoder_buffer_delay =
                        rav1d_get_bits(gb, encoder_decoder_buffer_delay_length) as c_int;
                    opi.encoder_buffer_delay =
                        rav1d_get_bits(gb, encoder_decoder_buffer_delay_length) as c_int;
                    opi.low_delay_mode = rav1d_get_bit(gb) as c_int;
                }
            }
            if display_model_info_present != 0 {
                op.display_model_param_present = rav1d_get_bit(gb) as c_int;
            }
            op.initial_display_delay = if op.display_model_param_present != 0 {
                rav1d_get_bits(gb, 4) as c_int + 1
            } else {
                10
            };
        }
        debug.post(gb, "operating-points");
    }

    let op_idx = if c.operating_point < num_operating_points {
        c.operating_point
    } else {
        0
    };
    c.operating_point_idc = operating_points[op_idx as usize].idc as c_uint;
    let spatial_mask = c.operating_point_idc >> 8;
    c.max_spatial_id = if spatial_mask != 0 {
        ulog2(spatial_mask) != 0
    } else {
        false
    };

    let width_n_bits = rav1d_get_bits(gb, 4) as c_int + 1;
    let height_n_bits = rav1d_get_bits(gb, 4) as c_int + 1;
    let max_width = rav1d_get_bits(gb, width_n_bits) as c_int + 1;
    let max_height = rav1d_get_bits(gb, height_n_bits) as c_int + 1;
    debug.post(gb, "size");
    let frame_id_numbers_present;
    let delta_frame_id_n_bits;
    let frame_id_n_bits;
    if reduced_still_picture_header == 0 {
        frame_id_numbers_present = rav1d_get_bit(gb) as c_int;
        if frame_id_numbers_present != 0 {
            delta_frame_id_n_bits = rav1d_get_bits(gb, 4) as c_int + 2;
            frame_id_n_bits = rav1d_get_bits(gb, 3) as c_int + delta_frame_id_n_bits + 1;
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

    let sb128 = rav1d_get_bit(gb) as c_int;
    let filter_intra = rav1d_get_bit(gb) as c_int;
    let intra_edge_filter = rav1d_get_bit(gb) as c_int;
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
        screen_content_tools = RAV1D_ADAPTIVE;
        force_integer_mv = RAV1D_ADAPTIVE;

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
        inter_intra = rav1d_get_bit(gb) as c_int;
        masked_compound = rav1d_get_bit(gb) as c_int;
        warped_motion = rav1d_get_bit(gb) as c_int;
        dual_filter = rav1d_get_bit(gb) as c_int;
        order_hint = rav1d_get_bit(gb) as c_int;
        if order_hint != 0 {
            jnt_comp = rav1d_get_bit(gb) as c_int;
            ref_frame_mvs = rav1d_get_bit(gb) as c_int;
        } else {
            // Default initialization.
            jnt_comp = Default::default();
            ref_frame_mvs = Default::default();
        }
        screen_content_tools = if rav1d_get_bit(gb) != 0 {
            RAV1D_ADAPTIVE
        } else {
            rav1d_get_bit(gb) as Rav1dAdaptiveBoolean
        };
        debug.post(gb, "screentools");
        force_integer_mv = if screen_content_tools as c_uint != 0 {
            if rav1d_get_bit(gb) != 0 {
                RAV1D_ADAPTIVE
            } else {
                rav1d_get_bit(gb) as Rav1dAdaptiveBoolean
            }
        } else {
            2
        };
        if order_hint != 0 {
            order_hint_n_bits = rav1d_get_bits(gb, 3) as c_int + 1;
        } else {
            // Default initialization.
            order_hint_n_bits = Default::default();
        }
    }
    let super_res = rav1d_get_bit(gb) as c_int;
    let cdef = rav1d_get_bit(gb) as c_int;
    let restoration = rav1d_get_bit(gb) as c_int;
    debug.post(gb, "featurebits");

    let hbd = {
        let mut hbd = rav1d_get_bit(gb) as c_int;
        if profile == 2 && hbd != 0 {
            hbd += rav1d_get_bit(gb) as c_int;
        }
        hbd
    };
    let monochrome;
    if profile != 1 {
        monochrome = rav1d_get_bit(gb) as c_int;
    } else {
        // Default initialization.
        monochrome = Default::default();
    }
    let color_description_present = rav1d_get_bit(gb) as c_int;
    let pri;
    let trc;
    let mtrx;
    if color_description_present != 0 {
        pri = rav1d_get_bits(gb, 8) as Rav1dColorPrimaries;
        trc = rav1d_get_bits(gb, 8) as Rav1dTransferCharacteristics;
        mtrx = rav1d_get_bits(gb, 8) as Rav1dMatrixCoefficients;
    } else {
        pri = RAV1D_COLOR_PRI_UNKNOWN;
        trc = RAV1D_TRC_UNKNOWN;
        mtrx = RAV1D_MC_UNKNOWN;
    }
    let color_range;
    let layout;
    let ss_ver;
    let ss_hor;
    let chr;
    if monochrome != 0 {
        color_range = rav1d_get_bit(gb) as c_int;
        layout = Rav1dPixelLayout::I400;
        ss_ver = 1;
        ss_hor = ss_ver;
        chr = RAV1D_CHR_UNKNOWN;
    } else if pri == RAV1D_COLOR_PRI_BT709 && trc == RAV1D_TRC_SRGB && mtrx == RAV1D_MC_IDENTITY {
        layout = Rav1dPixelLayout::I444;
        color_range = 1;
        if profile != 1 && !(profile == 2 && hbd == 2) {
            return Err(EINVAL);
        }

        // Default initialization.
        ss_hor = Default::default();
        ss_ver = Default::default();
        chr = Default::default();
    } else {
        color_range = rav1d_get_bit(gb) as c_int;
        match profile {
            0 => {
                layout = Rav1dPixelLayout::I420;
                ss_ver = 1;
                ss_hor = ss_ver;
            }
            1 => {
                layout = Rav1dPixelLayout::I444;

                // Default initialization.
                ss_hor = Default::default();
                ss_ver = Default::default();
            }
            2 => {
                if hbd == 2 {
                    ss_hor = rav1d_get_bit(gb) as c_int;
                    if ss_hor != 0 {
                        ss_ver = rav1d_get_bit(gb) as c_int;
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
            _ => unreachable!(), // TODO(kkysen) Make `profile` an `enum` so this isn't needed.
        }
        chr = if ss_hor & ss_ver != 0 {
            rav1d_get_bits(gb, 2) as Rav1dChromaSamplePosition
        } else {
            RAV1D_CHR_UNKNOWN
        };
    }
    if c.strict_std_compliance && mtrx == RAV1D_MC_IDENTITY && layout != Rav1dPixelLayout::I444 {
        return Err(EINVAL);
    }
    let separate_uv_delta_q;
    if monochrome == 0 {
        separate_uv_delta_q = rav1d_get_bit(gb) as c_int;
    } else {
        // Default initialization.
        separate_uv_delta_q = Default::default();
    }
    debug.post(gb, "colorinfo");

    let film_grain_present = rav1d_get_bit(gb) as c_int;
    debug.post(gb, "filmgrain");

    rav1d_get_bit(gb); // dummy bit

    // We needn't bother flushing the OBU here: we'll check we didn't
    // overrun in the caller and will then discard gb, so there's no
    // point in setting its position properly.

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

unsafe fn parse_frame_size(
    c: &Rav1dContext,
    seqhdr: &Rav1dSequenceHeader,
    hdr: &Rav1dFrameHeader,
    gb: &mut GetBits,
    use_ref: bool,
) -> Rav1dResult<Rav1dFrameSize> {
    if use_ref {
        for i in 0..7 {
            if rav1d_get_bit(gb) != 0 {
                let r#ref = &c.refs[hdr.refidx[i as usize] as usize].p;
                if (*r#ref).p.frame_hdr.is_null() {
                    return Err(EINVAL);
                }
                let ref_size = &(*(*r#ref).p.frame_hdr).size;
                let width1 = ref_size.width[1];
                let height = ref_size.height;
                let render_width = ref_size.render_width;
                let render_height = ref_size.render_height;
                let enabled = (seqhdr.super_res != 0 && rav1d_get_bit(gb) != 0) as c_int;
                let width_scale_denominator;
                let width0;
                if enabled != 0 {
                    width_scale_denominator = 9 + rav1d_get_bits(gb, 3) as c_int;
                    let d = width_scale_denominator;
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
                    super_res: Rav1dFrameHeader_super_res {
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
    if hdr.frame_size_override != 0 {
        width1 = rav1d_get_bits(gb, seqhdr.width_n_bits) as c_int + 1;
        height = rav1d_get_bits(gb, seqhdr.height_n_bits) as c_int + 1;
    } else {
        width1 = seqhdr.max_width;
        height = seqhdr.max_height;
    }
    let enabled = (seqhdr.super_res != 0 && rav1d_get_bit(gb) != 0) as c_int;
    let width_scale_denominator;
    let width0;
    if enabled != 0 {
        width_scale_denominator = 9 + rav1d_get_bits(gb, 3) as c_int;
        let d = width_scale_denominator;
        width0 = cmp::max((width1 * 8 + (d >> 1)) / d, cmp::min(16, width1));
    } else {
        width_scale_denominator = 8;
        width0 = width1;
    }
    let have_render_size = rav1d_get_bit(gb) as c_int;
    let render_width;
    let render_height;
    if have_render_size != 0 {
        render_width = rav1d_get_bits(gb, 16) as c_int + 1;
        render_height = rav1d_get_bits(gb, 16) as c_int + 1;
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
        super_res: Rav1dFrameHeader_super_res {
            enabled,
            width_scale_denominator,
        },
        have_render_size,
    })
}

#[inline]
fn tile_log2(sz: c_int, tgt: c_int) -> c_int {
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

unsafe fn parse_refidx(
    c: &Rav1dContext,
    seqhdr: &Rav1dSequenceHeader,
    frame_ref_short_signaling: c_int,
    frame_offset: c_int,
    frame_id: c_int,
    gb: &mut GetBits,
) -> Rav1dResult<[c_int; RAV1D_REFS_PER_FRAME]> {
    let mut refidx = [-1; RAV1D_REFS_PER_FRAME];
    if frame_ref_short_signaling != 0 {
        // FIXME: Nearly verbatim copy from section 7.8
        refidx[0] = rav1d_get_bits(gb, 3) as c_int;
        refidx[3] = rav1d_get_bits(gb, 3) as c_int;

        let mut shifted_frame_offset = [0; 8];
        let current_frame_offset = 1 << seqhdr.order_hint_n_bits - 1;
        for i in 0..8 {
            if c.refs[i as usize].p.p.frame_hdr.is_null() {
                return Err(EINVAL);
            }
            shifted_frame_offset[i as usize] = current_frame_offset
                + get_poc_diff(
                    seqhdr.order_hint_n_bits,
                    (*c.refs[i as usize].p.p.frame_hdr).frame_offset,
                    frame_offset,
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
            refidx[i as usize] = rav1d_get_bits(gb, 3) as c_int;
        }
        if seqhdr.frame_id_numbers_present != 0 {
            let delta_ref_frame_id_minus_1 =
                rav1d_get_bits(gb, seqhdr.delta_frame_id_n_bits) as c_int;
            let ref_frame_id =
                frame_id + ((1) << seqhdr.frame_id_n_bits) - delta_ref_frame_id_minus_1 - 1
                    & ((1) << seqhdr.frame_id_n_bits) - 1;
            let ref_frame_hdr = c.refs[refidx[i as usize] as usize].p.p.frame_hdr;
            if ref_frame_hdr.is_null() || (*ref_frame_hdr).frame_id != ref_frame_id {
                return Err(EINVAL);
            }
        }
    }
    Ok(refidx)
}

unsafe fn parse_tiling(
    seqhdr: &Rav1dSequenceHeader,
    size: &Rav1dFrameSize,
    debug: &Debug,
    gb: &mut GetBits,
) -> Rav1dResult<Rav1dFrameHeader_tiling> {
    let uniform = rav1d_get_bit(gb) as c_int;
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
        while log2_cols < max_log2_cols && rav1d_get_bit(gb) != 0 {
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
        let min_log2_rows = cmp::max(min_log2_tiles - log2_cols, 0);

        log2_rows = min_log2_rows;
        while log2_rows < max_log2_rows && rav1d_get_bit(gb) != 0 {
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
        while sbx < sbw && cols < RAV1D_MAX_TILE_COLS as c_int {
            let tile_width_sb = cmp::min(sbw - sbx, max_tile_width_sb);
            let tile_w = if tile_width_sb > 1 {
                1 + rav1d_get_uniform(gb, tile_width_sb as c_uint) as c_int
            } else {
                1
            };
            col_start_sb[cols as usize] = sbx as u16;
            sbx += tile_w;
            widest_tile = cmp::max(widest_tile, tile_w);
            cols += 1;
        }
        log2_cols = tile_log2(1, cols);
        if min_log2_tiles != 0 {
            max_tile_area_sb >>= min_log2_tiles + 1;
        }
        let max_tile_height_sb = cmp::max(max_tile_area_sb / widest_tile, 1);

        rows = 0;
        let mut sby = 0;
        while sby < sbh && rows < RAV1D_MAX_TILE_ROWS as c_int {
            let tile_height_sb = cmp::min(sbh - sby, max_tile_height_sb);
            let tile_h = if tile_height_sb > 1 {
                1 + rav1d_get_uniform(gb, tile_height_sb as c_uint) as c_int
            } else {
                1
            };
            row_start_sb[rows as usize] = sby as u16;
            sby += tile_h;
            rows += 1;
        }
        log2_rows = tile_log2(1, rows);
    }
    col_start_sb[cols as usize] = sbw as u16;
    row_start_sb[rows as usize] = sbh as u16;
    let update;
    let n_bytes;
    if log2_cols != 0 || log2_rows != 0 {
        update = rav1d_get_bits(gb, log2_cols + log2_rows) as c_int;
        if update >= cols * rows {
            return Err(EINVAL);
        }
        n_bytes = rav1d_get_bits(gb, 2) + 1;
    } else {
        update = 0;
        n_bytes = update as c_uint;
    }
    debug.post(gb, "tiling");
    Ok(Rav1dFrameHeader_tiling {
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

unsafe fn parse_quant(
    seqhdr: &Rav1dSequenceHeader,
    debug: &Debug,
    gb: &mut GetBits,
) -> Rav1dFrameHeader_quant {
    let yac = rav1d_get_bits(gb, 8) as c_int;
    let ydc_delta = if rav1d_get_bit(gb) != 0 {
        rav1d_get_sbits(gb, 7)
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
            rav1d_get_bit(gb) as c_int
        } else {
            0
        };
        udc_delta = if rav1d_get_bit(gb) != 0 {
            rav1d_get_sbits(gb, 7)
        } else {
            0
        };
        uac_delta = if rav1d_get_bit(gb) != 0 {
            rav1d_get_sbits(gb, 7)
        } else {
            0
        };
        if diff_uv_delta != 0 {
            vdc_delta = if rav1d_get_bit(gb) != 0 {
                rav1d_get_sbits(gb, 7)
            } else {
                0
            };
            vac_delta = if rav1d_get_bit(gb) != 0 {
                rav1d_get_sbits(gb, 7)
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
    let qm = rav1d_get_bit(gb) as c_int;
    let qm_y;
    let qm_u;
    let qm_v;
    if qm != 0 {
        qm_y = rav1d_get_bits(gb, 4) as c_int;
        qm_u = rav1d_get_bits(gb, 4) as c_int;
        qm_v = if seqhdr.separate_uv_delta_q != 0 {
            rav1d_get_bits(gb, 4) as c_int
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
    Rav1dFrameHeader_quant {
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

unsafe fn parse_seg_data(gb: &mut GetBits) -> Rav1dSegmentationDataSet {
    let mut preskip = 0;
    let mut last_active_segid = -1;
    let d = array::from_fn(|i| {
        let i = i as c_int;
        let delta_q;
        if rav1d_get_bit(gb) != 0 {
            delta_q = rav1d_get_sbits(gb, 9);
            last_active_segid = i;
        } else {
            delta_q = 0;
        }
        let delta_lf_y_v;
        if rav1d_get_bit(gb) != 0 {
            delta_lf_y_v = rav1d_get_sbits(gb, 7);
            last_active_segid = i;
        } else {
            delta_lf_y_v = 0;
        }
        let delta_lf_y_h;
        if rav1d_get_bit(gb) != 0 {
            delta_lf_y_h = rav1d_get_sbits(gb, 7);
            last_active_segid = i;
        } else {
            delta_lf_y_h = 0;
        }
        let delta_lf_u;
        if rav1d_get_bit(gb) != 0 {
            delta_lf_u = rav1d_get_sbits(gb, 7);
            last_active_segid = i;
        } else {
            delta_lf_u = 0;
        }
        let delta_lf_v;
        if rav1d_get_bit(gb) != 0 {
            delta_lf_v = rav1d_get_sbits(gb, 7);
            last_active_segid = i;
        } else {
            delta_lf_v = 0;
        }
        let r#ref;
        if rav1d_get_bit(gb) != 0 {
            r#ref = rav1d_get_bits(gb, 3) as c_int;
            last_active_segid = i;
            preskip = 1;
        } else {
            r#ref = -1;
        }
        let skip = rav1d_get_bit(gb) as c_int;
        if skip != 0 {
            last_active_segid = i;
            preskip = 1;
        }
        let globalmv = rav1d_get_bit(gb) as c_int;
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

unsafe fn parse_segmentation(
    c: &Rav1dContext,
    primary_ref_frame: c_int,
    refidx: &[c_int; RAV1D_REFS_PER_FRAME],
    quant: &Rav1dFrameHeader_quant,
    debug: &Debug,
    gb: &mut GetBits,
) -> Rav1dResult<Rav1dFrameHeader_segmentation> {
    let enabled = rav1d_get_bit(gb) as c_int;
    let update_map;
    let temporal;
    let update_data;
    let seg_data = if enabled != 0 {
        if primary_ref_frame == RAV1D_PRIMARY_REF_NONE {
            update_map = 1;
            temporal = 0;
            update_data = 1;
        } else {
            update_map = rav1d_get_bit(gb) as c_int;
            temporal = if update_map != 0 {
                rav1d_get_bit(gb) as c_int
            } else {
                0
            };
            update_data = rav1d_get_bit(gb) as c_int;
        }

        if update_data != 0 {
            parse_seg_data(gb)
        } else {
            // segmentation.update_data was false so we should copy
            // segmentation data from the reference frame.
            assert!(primary_ref_frame != RAV1D_PRIMARY_REF_NONE);
            let pri_ref = refidx[primary_ref_frame as usize];
            if (c.refs[pri_ref as usize].p.p.frame_hdr).is_null() {
                return Err(EINVAL);
            }
            (*c.refs[pri_ref as usize].p.p.frame_hdr)
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
    let delta_lossless = (quant.ydc_delta == 0
        && quant.udc_delta == 0
        && quant.uac_delta == 0
        && quant.vdc_delta == 0
        && quant.vac_delta == 0) as c_int;
    let qidx = array::from_fn(|i| {
        if enabled != 0 {
            iclip_u8(quant.yac + seg_data.d[i].delta_q)
        } else {
            quant.yac
        }
    });
    let lossless = array::from_fn(|i| (qidx[i] == 0 && delta_lossless != 0) as c_int);
    Ok(Rav1dFrameHeader_segmentation {
        enabled,
        update_map,
        temporal,
        update_data,
        seg_data,
        lossless,
        qidx,
    })
}

unsafe fn parse_delta(
    quant: &Rav1dFrameHeader_quant,
    allow_intrabc: c_int,
    debug: &Debug,
    gb: &mut GetBits,
) -> Rav1dFrameHeader_delta {
    let q = {
        let present = if quant.yac != 0 {
            rav1d_get_bit(gb) as c_int
        } else {
            0
        };
        let res_log2 = if present != 0 {
            rav1d_get_bits(gb, 2) as c_int
        } else {
            0
        };
        Rav1dFrameHeader_delta_q { present, res_log2 }
    };
    let lf = {
        let present = (q.present != 0 && allow_intrabc == 0 && rav1d_get_bit(gb) != 0) as c_int;
        let res_log2 = if present != 0 {
            rav1d_get_bits(gb, 2) as c_int
        } else {
            0
        };
        let multi = if present != 0 {
            rav1d_get_bit(gb) as c_int
        } else {
            0
        };
        Rav1dFrameHeader_delta_lf {
            present,
            res_log2,
            multi,
        }
    };
    debug.post(gb, "delta_q_lf_flags");
    Rav1dFrameHeader_delta { q, lf }
}

unsafe fn parse_loopfilter(
    c: &Rav1dContext,
    seqhdr: &Rav1dSequenceHeader,
    all_lossless: c_int,
    allow_intrabc: c_int,
    primary_ref_frame: c_int,
    refidx: &[c_int; RAV1D_REFS_PER_FRAME],
    debug: &Debug,
    gb: &mut GetBits,
) -> Rav1dResult<Rav1dFrameHeader_loopfilter> {
    let level_y;
    let level_u;
    let level_v;
    let mode_ref_delta_enabled;
    let mode_ref_delta_update;
    let mut mode_ref_deltas;
    let sharpness;
    if all_lossless != 0 || allow_intrabc != 0 {
        level_y = [0; 2];
        level_v = 0;
        level_u = level_v;
        sharpness = 0;
        mode_ref_delta_enabled = 1;
        mode_ref_delta_update = 1;
        mode_ref_deltas = default_mode_ref_deltas.clone();
    } else {
        level_y = [
            rav1d_get_bits(gb, 6) as c_int,
            rav1d_get_bits(gb, 6) as c_int,
        ];
        if seqhdr.monochrome == 0 && (level_y[0] != 0 || level_y[1] != 0) {
            level_u = rav1d_get_bits(gb, 6) as c_int;
            level_v = rav1d_get_bits(gb, 6) as c_int;
        } else {
            // Default initialization.
            level_u = Default::default();
            level_v = Default::default();
        }
        sharpness = rav1d_get_bits(gb, 3) as c_int;

        if primary_ref_frame == RAV1D_PRIMARY_REF_NONE {
            mode_ref_deltas = default_mode_ref_deltas.clone();
        } else {
            let r#ref = refidx[primary_ref_frame as usize];
            if (c.refs[r#ref as usize].p.p.frame_hdr).is_null() {
                return Err(EINVAL);
            }
            mode_ref_deltas = (*c.refs[r#ref as usize].p.p.frame_hdr)
                .loopfilter
                .mode_ref_deltas
                .clone();
        }
        mode_ref_delta_enabled = rav1d_get_bit(gb) as c_int;
        if mode_ref_delta_enabled != 0 {
            mode_ref_delta_update = rav1d_get_bit(gb) as c_int;
            if mode_ref_delta_update != 0 {
                for i in 0..8 {
                    if rav1d_get_bit(gb) != 0 {
                        mode_ref_deltas.ref_delta[i as usize] = rav1d_get_sbits(gb, 7);
                    }
                }
                for i in 0..2 {
                    if rav1d_get_bit(gb) != 0 {
                        mode_ref_deltas.mode_delta[i as usize] = rav1d_get_sbits(gb, 7);
                    }
                }
            }
        } else {
            // Default initialization.
            mode_ref_delta_update = Default::default();
        }
    }
    debug.post(gb, "lpf");
    Ok(Rav1dFrameHeader_loopfilter {
        level_y,
        level_u,
        level_v,
        mode_ref_delta_enabled,
        mode_ref_delta_update,
        mode_ref_deltas,
        sharpness,
    })
}

unsafe fn parse_cdef(
    seqhdr: &Rav1dSequenceHeader,
    hdr: &mut Rav1dFrameHeader,
    debug: &Debug,
    gb: &mut GetBits,
) {
    if hdr.all_lossless == 0 && seqhdr.cdef != 0 && hdr.allow_intrabc == 0 {
        hdr.cdef.damping = rav1d_get_bits(gb, 2) as c_int + 3;
        hdr.cdef.n_bits = rav1d_get_bits(gb, 2) as c_int;
        for i in 0..1 << hdr.cdef.n_bits {
            hdr.cdef.y_strength[i as usize] = rav1d_get_bits(gb, 6) as c_int;
            if seqhdr.monochrome == 0 {
                hdr.cdef.uv_strength[i as usize] = rav1d_get_bits(gb, 6) as c_int;
            }
        }
    } else {
        hdr.cdef.n_bits = 0;
        hdr.cdef.y_strength[0] = 0;
        hdr.cdef.uv_strength[0] = 0;
    }
    debug.post(gb, "cdef");
}

unsafe fn parse_restoration(
    seqhdr: &Rav1dSequenceHeader,
    hdr: &mut Rav1dFrameHeader,
    debug: &Debug,
    gb: &mut GetBits,
) -> Rav1dResult {
    if (hdr.all_lossless == 0 || hdr.size.super_res.enabled != 0)
        && seqhdr.restoration != 0
        && hdr.allow_intrabc == 0
    {
        hdr.restoration.r#type[0] = rav1d_get_bits(gb, 2) as Rav1dRestorationType;
        if seqhdr.monochrome == 0 {
            hdr.restoration.r#type[1] = rav1d_get_bits(gb, 2) as Rav1dRestorationType;
            hdr.restoration.r#type[2] = rav1d_get_bits(gb, 2) as Rav1dRestorationType;
        } else {
            hdr.restoration.r#type[2] = RAV1D_RESTORATION_NONE;
            hdr.restoration.r#type[1] = hdr.restoration.r#type[2];
        }

        if hdr.restoration.r#type[0] != 0
            || hdr.restoration.r#type[1] != 0
            || hdr.restoration.r#type[2] != 0
        {
            // Log2 of the restoration unit size.
            hdr.restoration.unit_size[0] = 6 + seqhdr.sb128;
            if rav1d_get_bit(gb) != 0 {
                hdr.restoration.unit_size[0] += 1;
                if seqhdr.sb128 == 0 {
                    hdr.restoration.unit_size[0] += rav1d_get_bit(gb) as c_int;
                }
            }
            hdr.restoration.unit_size[1] = hdr.restoration.unit_size[0];
            if (hdr.restoration.r#type[1] != 0 || hdr.restoration.r#type[2] != 0)
                && seqhdr.ss_hor == 1
                && seqhdr.ss_ver == 1
            {
                hdr.restoration.unit_size[1] -= rav1d_get_bit(gb) as c_int;
            }
        } else {
            hdr.restoration.unit_size[0] = 8;
        }
    } else {
        hdr.restoration.r#type[0] = RAV1D_RESTORATION_NONE;
        hdr.restoration.r#type[1] = RAV1D_RESTORATION_NONE;
        hdr.restoration.r#type[2] = RAV1D_RESTORATION_NONE;
    }
    debug.post(gb, "restoration");
    Ok(())
}

unsafe fn parse_skip_mode(
    c: &Rav1dContext,
    seqhdr: &Rav1dSequenceHeader,
    hdr: &mut Rav1dFrameHeader,
    debug: &Debug,
    gb: &mut GetBits,
) -> Rav1dResult {
    hdr.skip_mode_allowed = 0;
    if hdr.switchable_comp_refs != 0
        && hdr.frame_type.is_inter_or_switch()
        && seqhdr.order_hint != 0
    {
        let poc = hdr.frame_offset as c_uint;
        let mut off_before = 0xffffffff;
        let mut off_after = -1;
        let mut off_before_idx = 0;
        let mut off_after_idx = 0;
        for i in 0..7 {
            if c.refs[hdr.refidx[i as usize] as usize]
                .p
                .p
                .frame_hdr
                .is_null()
            {
                return Err(EINVAL);
            }
            let refpoc =
                (*c.refs[hdr.refidx[i as usize] as usize].p.p.frame_hdr).frame_offset as c_uint;

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
            hdr.skip_mode_refs[0] = cmp::min(off_before_idx, off_after_idx);
            hdr.skip_mode_refs[1] = cmp::max(off_before_idx, off_after_idx);
            hdr.skip_mode_allowed = 1;
        } else if off_before != 0xffffffff {
            let mut off_before2 = 0xffffffff;
            let mut off_before2_idx = 0;
            for i in 0..7 {
                if (c.refs[hdr.refidx[i as usize] as usize].p.p.frame_hdr).is_null() {
                    return Err(EINVAL);
                }
                let refpoc =
                    (*c.refs[hdr.refidx[i as usize] as usize].p.p.frame_hdr).frame_offset as c_uint;
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
                hdr.skip_mode_refs[0] = cmp::min(off_before_idx, off_before2_idx);
                hdr.skip_mode_refs[1] = cmp::max(off_before_idx, off_before2_idx);
                hdr.skip_mode_allowed = 1;
            }
        }
    }
    hdr.skip_mode_enabled = if hdr.skip_mode_allowed != 0 {
        rav1d_get_bit(gb) as c_int
    } else {
        0
    };
    debug.post(gb, "extskip");
    Ok(())
}

unsafe fn parse_gmv(
    c: &Rav1dContext,
    hdr: &mut Rav1dFrameHeader,
    debug: &Debug,
    gb: &mut GetBits,
) -> Rav1dResult {
    for i in 0..7 {
        hdr.gmv[i as usize] = dav1d_default_wm_params.clone();
    }

    if hdr.frame_type.is_inter_or_switch() {
        for i in 0..7 {
            hdr.gmv[i as usize].r#type = if rav1d_get_bit(gb) == 0 {
                RAV1D_WM_TYPE_IDENTITY
            } else if rav1d_get_bit(gb) != 0 {
                RAV1D_WM_TYPE_ROT_ZOOM
            } else if rav1d_get_bit(gb) != 0 {
                RAV1D_WM_TYPE_TRANSLATION
            } else {
                RAV1D_WM_TYPE_AFFINE
            };
            if hdr.gmv[i as usize].r#type == RAV1D_WM_TYPE_IDENTITY {
                continue;
            }

            let ref_gmv;
            if hdr.primary_ref_frame == RAV1D_PRIMARY_REF_NONE {
                ref_gmv = &dav1d_default_wm_params;
            } else {
                let pri_ref = hdr.refidx[hdr.primary_ref_frame as usize];
                if (c.refs[pri_ref as usize].p.p.frame_hdr).is_null() {
                    return Err(EINVAL);
                }
                ref_gmv = &mut (*c.refs[pri_ref as usize].p.p.frame_hdr).gmv[i as usize];
            }
            let mat = &mut hdr.gmv[i as usize].matrix;
            let ref_mat = &ref_gmv.matrix;
            let bits;
            let shift;

            if hdr.gmv[i as usize].r#type >= RAV1D_WM_TYPE_ROT_ZOOM {
                mat[2] =
                    ((1) << 16) + 2 * rav1d_get_bits_subexp(gb, ref_mat[2] - ((1) << 16) >> 1, 12);
                mat[3] = 2 * rav1d_get_bits_subexp(gb, ref_mat[3] >> 1, 12);

                bits = 12;
                shift = 10;
            } else {
                bits = 9 - (hdr.hp == 0) as c_int;
                shift = 13 + (hdr.hp == 0) as c_int;
            }

            if hdr.gmv[i as usize].r#type as c_uint == RAV1D_WM_TYPE_AFFINE as c_int as c_uint {
                mat[4] = 2 * rav1d_get_bits_subexp(gb, ref_mat[4] >> 1, 12);
                mat[5] =
                    (1 << 16) + 2 * rav1d_get_bits_subexp(gb, ref_mat[5] - ((1) << 16) >> 1, 12);
            } else {
                mat[4] = -mat[3];
                mat[5] = mat[2];
            }

            mat[0] = rav1d_get_bits_subexp(gb, ref_mat[0] >> shift, bits as c_uint) * (1 << shift);
            mat[1] = rav1d_get_bits_subexp(gb, ref_mat[1] >> shift, bits as c_uint) * (1 << shift);
        }
    }
    debug.post(gb, "gmv");
    Ok(())
}

unsafe fn parse_film_grain(
    c: &Rav1dContext,
    seqhdr: &Rav1dSequenceHeader,
    hdr: &mut Rav1dFrameHeader,
    debug: &Debug,
    gb: &mut GetBits,
) -> Rav1dResult {
    hdr.film_grain.present = (seqhdr.film_grain_present != 0
        && (hdr.show_frame != 0 || hdr.showable_frame != 0)
        && rav1d_get_bit(gb) != 0) as c_int;
    if hdr.film_grain.present != 0 {
        let seed = rav1d_get_bits(gb, 16);
        hdr.film_grain.update =
            (hdr.frame_type != Rav1dFrameType::Inter || rav1d_get_bit(gb) != 0) as c_int;
        if hdr.film_grain.update == 0 {
            let refidx = rav1d_get_bits(gb, 3) as c_int;
            let mut found = false;
            for i in 0..7 {
                if hdr.refidx[i as usize] == refidx {
                    found = true;
                    break;
                }
            }
            if !found || c.refs[refidx as usize].p.p.frame_hdr.is_null() {
                return Err(EINVAL);
            }
            hdr.film_grain.data = (*c.refs[refidx as usize].p.p.frame_hdr)
                .film_grain
                .data
                .clone();
            hdr.film_grain.data.seed = seed;
        } else {
            let fgd = &mut hdr.film_grain.data;
            fgd.seed = seed;

            fgd.num_y_points = rav1d_get_bits(gb, 4) as c_int;
            if fgd.num_y_points > 14 {
                return Err(EINVAL);
            }
            for i in 0..fgd.num_y_points {
                fgd.y_points[i as usize][0] = rav1d_get_bits(gb, 8) as u8;
                if i != 0
                    && fgd.y_points[(i - 1) as usize][0] as c_int
                        >= fgd.y_points[i as usize][0] as c_int
                {
                    return Err(EINVAL);
                }
                fgd.y_points[i as usize][1] = rav1d_get_bits(gb, 8) as u8;
            }

            fgd.chroma_scaling_from_luma = seqhdr.monochrome == 0 && rav1d_get_bit(gb) != 0;
            if seqhdr.monochrome != 0
                || fgd.chroma_scaling_from_luma
                || seqhdr.ss_ver == 1 && seqhdr.ss_hor == 1 && fgd.num_y_points == 0
            {
                fgd.num_uv_points[1] = 0;
                fgd.num_uv_points[0] = fgd.num_uv_points[1];
            } else {
                for pl in 0..2 {
                    fgd.num_uv_points[pl as usize] = rav1d_get_bits(gb, 4) as c_int;
                    if fgd.num_uv_points[pl as usize] > 10 {
                        return Err(EINVAL);
                    }
                    for i in 0..fgd.num_uv_points[pl as usize] {
                        fgd.uv_points[pl as usize][i as usize][0] = rav1d_get_bits(gb, 8) as u8;
                        if i != 0
                            && fgd.uv_points[pl as usize][(i - 1) as usize][0] as c_int
                                >= fgd.uv_points[pl as usize][i as usize][0] as c_int
                        {
                            return Err(EINVAL);
                        }
                        fgd.uv_points[pl as usize][i as usize][1] = rav1d_get_bits(gb, 8) as u8;
                    }
                }
            }

            if seqhdr.ss_hor == 1
                && seqhdr.ss_ver == 1
                && (fgd.num_uv_points[0] != 0) != (fgd.num_uv_points[1] != 0)
            {
                return Err(EINVAL);
            }

            fgd.scaling_shift = rav1d_get_bits(gb, 2) as u8 + 8;
            fgd.ar_coeff_lag = rav1d_get_bits(gb, 2) as c_int;
            let num_y_pos = 2 * fgd.ar_coeff_lag * (fgd.ar_coeff_lag + 1);
            if fgd.num_y_points != 0 {
                for i in 0..num_y_pos {
                    fgd.ar_coeffs_y[i as usize] = rav1d_get_bits(gb, 8).wrapping_sub(128) as i8;
                }
            }
            for pl in 0..2 {
                if fgd.num_uv_points[pl as usize] != 0 || fgd.chroma_scaling_from_luma {
                    let num_uv_pos = num_y_pos + (fgd.num_y_points != 0) as c_int;
                    for i in 0..num_uv_pos {
                        fgd.ar_coeffs_uv[pl as usize][i as usize] =
                            rav1d_get_bits(gb, 8).wrapping_sub(128) as i8;
                    }
                    if fgd.num_y_points == 0 {
                        fgd.ar_coeffs_uv[pl as usize][num_uv_pos as usize] = 0;
                    }
                }
            }
            fgd.ar_coeff_shift = rav1d_get_bits(gb, 2) as u8 + 6;
            fgd.grain_scale_shift = rav1d_get_bits(gb, 2) as u8;
            for pl in 0..2 {
                if fgd.num_uv_points[pl as usize] != 0 {
                    fgd.uv_mult[pl as usize] = rav1d_get_bits(gb, 8) as c_int - 128;
                    fgd.uv_luma_mult[pl as usize] = rav1d_get_bits(gb, 8) as c_int - 128;
                    fgd.uv_offset[pl as usize] = rav1d_get_bits(gb, 9) as c_int - 256;
                }
            }
            fgd.overlap_flag = rav1d_get_bit(gb) != 0;
            fgd.clip_to_restricted_range = rav1d_get_bit(gb) != 0;
        }
    } else {
        memset(
            &mut hdr.film_grain.data as *mut Rav1dFilmGrainData as *mut c_void,
            0,
            ::core::mem::size_of::<Rav1dFilmGrainData>(),
        );
    }
    debug.post(gb, "filmgrain");
    Ok(())
}

unsafe fn parse_frame_hdr(
    c: &Rav1dContext,
    seqhdr: &Rav1dSequenceHeader,
    temporal_id: c_int,
    spatial_id: c_int,
    gb: &mut GetBits,
) -> Rav1dResult<Rav1dFrameHeader> {
    let mut hdr = MaybeUninit::<Rav1dFrameHeader>::zeroed().assume_init();

    hdr.temporal_id = temporal_id;
    hdr.spatial_id = spatial_id;

    let debug = Debug::new(false, "HDR", gb);

    debug.post(gb, "show_existing_frame");
    hdr.show_existing_frame =
        (seqhdr.reduced_still_picture_header == 0 && rav1d_get_bit(gb) != 0) as c_int;
    if hdr.show_existing_frame != 0 {
        hdr.existing_frame_idx = rav1d_get_bits(gb, 3) as c_int;
        if seqhdr.decoder_model_info_present != 0 && seqhdr.equal_picture_interval == 0 {
            hdr.frame_presentation_delay =
                rav1d_get_bits(gb, seqhdr.frame_presentation_delay_length) as c_int;
        }
        if seqhdr.frame_id_numbers_present != 0 {
            hdr.frame_id = rav1d_get_bits(gb, seqhdr.frame_id_n_bits) as c_int;
            let ref_frame_hdr = c.refs[hdr.existing_frame_idx as usize].p.p.frame_hdr;
            if ref_frame_hdr.is_null() || (*ref_frame_hdr).frame_id != hdr.frame_id {
                return Err(EINVAL);
            }
        }
        return Ok(hdr);
    }

    hdr.frame_type = if seqhdr.reduced_still_picture_header != 0 {
        Rav1dFrameType::Key
    } else {
        Rav1dFrameType::from_repr(rav1d_get_bits(gb, 2) as usize).unwrap()
    };
    hdr.show_frame = (seqhdr.reduced_still_picture_header != 0 || rav1d_get_bit(gb) != 0) as c_int;
    if hdr.show_frame != 0 {
        if seqhdr.decoder_model_info_present != 0 && seqhdr.equal_picture_interval == 0 {
            hdr.frame_presentation_delay =
                rav1d_get_bits(gb, seqhdr.frame_presentation_delay_length) as c_int;
        }
        hdr.showable_frame = (hdr.frame_type != Rav1dFrameType::Key) as c_int;
    } else {
        hdr.showable_frame = rav1d_get_bit(gb) as c_int;
    }
    hdr.error_resilient_mode = (hdr.frame_type == Rav1dFrameType::Key && hdr.show_frame != 0
        || hdr.frame_type == Rav1dFrameType::Switch
        || seqhdr.reduced_still_picture_header != 0
        || rav1d_get_bit(gb) != 0) as c_int;
    debug.post(gb, "frametype_bits");
    hdr.disable_cdf_update = rav1d_get_bit(gb) as c_int;
    hdr.allow_screen_content_tools = (if seqhdr.screen_content_tools == RAV1D_ADAPTIVE {
        rav1d_get_bit(gb)
    } else {
        seqhdr.screen_content_tools
    }) as c_int;
    if hdr.allow_screen_content_tools != 0 {
        hdr.force_integer_mv = (if seqhdr.force_integer_mv == RAV1D_ADAPTIVE {
            rav1d_get_bit(gb)
        } else {
            seqhdr.force_integer_mv
        }) as c_int;
    } else {
        hdr.force_integer_mv = 0;
    }

    if hdr.frame_type.is_key_or_intra() {
        hdr.force_integer_mv = 1;
    }

    if seqhdr.frame_id_numbers_present != 0 {
        hdr.frame_id = rav1d_get_bits(gb, seqhdr.frame_id_n_bits) as c_int;
    }

    hdr.frame_size_override = (if seqhdr.reduced_still_picture_header != 0 {
        0
    } else if hdr.frame_type == Rav1dFrameType::Switch {
        1
    } else {
        rav1d_get_bit(gb)
    }) as c_int;
    debug.post(gb, "frame_size_override_flag");
    hdr.frame_offset = if seqhdr.order_hint != 0 {
        rav1d_get_bits(gb, seqhdr.order_hint_n_bits) as c_int
    } else {
        0
    };
    hdr.primary_ref_frame = if hdr.error_resilient_mode == 0 && hdr.frame_type.is_inter_or_switch()
    {
        rav1d_get_bits(gb, 3) as c_int
    } else {
        RAV1D_PRIMARY_REF_NONE
    };

    if seqhdr.decoder_model_info_present != 0 {
        hdr.buffer_removal_time_present = rav1d_get_bit(gb) as c_int;
        if hdr.buffer_removal_time_present != 0 {
            for i in 0..seqhdr.num_operating_points {
                let seqop = &seqhdr.operating_points[i as usize];
                let op = &mut hdr.operating_points[i as usize];
                if seqop.decoder_model_param_present != 0 {
                    let in_temporal_layer = seqop.idc >> hdr.temporal_id & 1;
                    let in_spatial_layer = seqop.idc >> hdr.spatial_id + 8 & 1;
                    if seqop.idc == 0 || in_temporal_layer != 0 && in_spatial_layer != 0 {
                        op.buffer_removal_time =
                            rav1d_get_bits(gb, seqhdr.buffer_removal_delay_length) as c_int;
                    }
                }
            }
        }
    }

    if hdr.frame_type.is_key_or_intra() {
        hdr.refresh_frame_flags = if hdr.frame_type == Rav1dFrameType::Key && hdr.show_frame != 0 {
            0xff
        } else {
            rav1d_get_bits(gb, 8) as c_int
        };
        if hdr.refresh_frame_flags != 0xff
            && hdr.error_resilient_mode != 0
            && seqhdr.order_hint != 0
        {
            for _ in 0..8 {
                rav1d_get_bits(gb, seqhdr.order_hint_n_bits);
            }
        }
        if c.strict_std_compliance
            && hdr.frame_type == Rav1dFrameType::Intra
            && hdr.refresh_frame_flags == 0xff
        {
            return Err(EINVAL);
        }
        hdr.size = parse_frame_size(c, seqhdr, &hdr, gb, false)?;
        hdr.allow_intrabc = (hdr.allow_screen_content_tools != 0
            && hdr.size.super_res.enabled == 0
            && rav1d_get_bit(gb) != 0) as c_int;
        hdr.use_ref_frame_mvs = 0;
    } else {
        hdr.allow_intrabc = 0;
        hdr.refresh_frame_flags = if hdr.frame_type == Rav1dFrameType::Switch {
            0xff
        } else {
            rav1d_get_bits(gb, 8) as c_int
        };
        if hdr.error_resilient_mode != 0 && seqhdr.order_hint != 0 {
            for _ in 0..8 {
                rav1d_get_bits(gb, seqhdr.order_hint_n_bits);
            }
        }
        hdr.frame_ref_short_signaling = (seqhdr.order_hint != 0 && rav1d_get_bit(gb) != 0) as c_int;
        hdr.refidx = parse_refidx(
            c,
            seqhdr,
            hdr.frame_ref_short_signaling,
            hdr.frame_offset,
            hdr.frame_id,
            gb,
        )?;
        let use_ref = hdr.error_resilient_mode == 0 && hdr.frame_size_override != 0;
        hdr.size = parse_frame_size(c, seqhdr, &hdr, gb, use_ref)?;
        hdr.hp = (hdr.force_integer_mv == 0 && rav1d_get_bit(gb) != 0) as c_int;
        hdr.subpel_filter_mode = if rav1d_get_bit(gb) != 0 {
            RAV1D_FILTER_SWITCHABLE
        } else {
            rav1d_get_bits(gb, 2) as Rav1dFilterMode
        };
        hdr.switchable_motion_mode = rav1d_get_bit(gb) as c_int;
        hdr.use_ref_frame_mvs = (hdr.error_resilient_mode == 0
            && seqhdr.ref_frame_mvs != 0
            && seqhdr.order_hint != 0
            && hdr.frame_type.is_inter_or_switch()
            && rav1d_get_bit(gb) != 0) as c_int;
    }
    debug.post(gb, "frametype-specific-bits");

    hdr.refresh_context = (seqhdr.reduced_still_picture_header == 0
        && hdr.disable_cdf_update == 0
        && rav1d_get_bit(gb) == 0) as c_int;
    debug.post(gb, "refresh_context");

    hdr.tiling = parse_tiling(seqhdr, &hdr.size, &debug, gb)?;
    hdr.quant = parse_quant(seqhdr, &debug, gb);
    hdr.segmentation = parse_segmentation(
        c,
        hdr.primary_ref_frame,
        &hdr.refidx,
        &hdr.quant,
        &debug,
        gb,
    )?;
    hdr.all_lossless = hdr.segmentation.lossless.iter().all(|&it| it != 0) as c_int;
    hdr.delta = parse_delta(&hdr.quant, hdr.allow_intrabc, &debug, gb);
    hdr.loopfilter = parse_loopfilter(
        c,
        seqhdr,
        hdr.all_lossless,
        hdr.allow_intrabc,
        hdr.primary_ref_frame,
        &hdr.refidx,
        &debug,
        gb,
    )?;
    parse_cdef(seqhdr, &mut hdr, &debug, gb);
    parse_restoration(seqhdr, &mut hdr, &debug, gb)?;

    hdr.txfm_mode = if hdr.all_lossless != 0 {
        RAV1D_TX_4X4_ONLY
    } else if rav1d_get_bit(gb) != 0 {
        RAV1D_TX_SWITCHABLE
    } else {
        RAV1D_TX_LARGEST
    };
    debug.post(gb, "txfmmode");
    hdr.switchable_comp_refs = if hdr.frame_type.is_inter_or_switch() {
        rav1d_get_bit(gb) as c_int
    } else {
        0
    };
    debug.post(gb, "refmode");
    parse_skip_mode(c, seqhdr, &mut hdr, &debug, gb)?;
    hdr.warp_motion = (hdr.error_resilient_mode == 0
        && hdr.frame_type.is_inter_or_switch()
        && seqhdr.warped_motion != 0
        && rav1d_get_bit(gb) != 0) as c_int;
    debug.post(gb, "warpmotionbit");
    hdr.reduced_txtp_set = rav1d_get_bit(gb) as c_int;
    debug.post(gb, "reducedtxtpset");

    parse_gmv(c, &mut hdr, &debug, gb)?;
    parse_film_grain(c, seqhdr, &mut hdr, &debug, gb)?;

    Ok(hdr)
}

unsafe fn parse_tile_hdr(
    tiling: &Rav1dFrameHeader_tiling,
    gb: &mut GetBits,
) -> Rav1dTileGroupHeader {
    let n_tiles = tiling.cols * tiling.rows;
    let have_tile_pos = if n_tiles > 1 {
        rav1d_get_bit(gb) as c_int
    } else {
        0
    };

    if have_tile_pos != 0 {
        let n_bits = tiling.log2_cols + tiling.log2_rows;
        let start = rav1d_get_bits(gb, n_bits) as c_int;
        let end = rav1d_get_bits(gb, n_bits) as c_int;
        Rav1dTileGroupHeader { start, end }
    } else {
        Rav1dTileGroupHeader {
            start: 0,
            end: n_tiles - 1,
        }
    }
}

/// Check that we haven't read more than `obu_len`` bytes
/// from the buffer since `init_bit_pos`.
unsafe fn check_for_overrun(
    c: &mut Rav1dContext,
    gb: &mut GetBits,
    init_bit_pos: c_uint,
    obu_len: c_uint,
) -> c_int {
    // Make sure we haven't actually read past the end of the `gb` buffer
    if gb.error != 0 {
        writeln!(c.logger, "Overrun in OBU bit buffer");
        return 1;
    }

    let pos = rav1d_get_bits_pos(gb);

    // We assume that `init_bit_pos` was the bit position of the buffer
    // at some point in the past, so cannot be smaller than `pos`.
    assert!(init_bit_pos <= pos);

    if pos - init_bit_pos > 8 * obu_len {
        writeln!(c.logger, "Overrun in OBU bit buffer into next OBU");
        return 1;
    }

    0
}

unsafe fn parse_obus(
    c: &mut Rav1dContext,
    r#in: &mut Rav1dData,
    global: c_int,
) -> Rav1dResult<c_uint> {
    unsafe fn skip(c: &mut Rav1dContext, len: c_uint, init_byte_pos: c_uint) -> c_uint {
        // update refs with only the headers in case we skip the frame
        for i in 0..8 {
            if (*c.frame_hdr).refresh_frame_flags & (1 << i) != 0 {
                rav1d_thread_picture_unref(&mut c.refs[i as usize].p);
                c.refs[i as usize].p.p.frame_hdr = c.frame_hdr;
                c.refs[i as usize].p.p.seq_hdr = c.seq_hdr;
                c.refs[i as usize].p.p.frame_hdr_ref = c.frame_hdr_ref;
                c.refs[i as usize].p.p.seq_hdr_ref = c.seq_hdr_ref;
                rav1d_ref_inc(c.frame_hdr_ref);
                rav1d_ref_inc(c.seq_hdr_ref);
            }
        }

        rav1d_ref_dec(&mut c.frame_hdr_ref);
        c.frame_hdr = 0 as *mut Rav1dFrameHeader;
        c.n_tiles = 0;

        len + init_byte_pos
    }

    let mut gb = GetBits {
        state: 0,
        bits_left: 0,
        error: 0,
        ptr: 0 as *const u8,
        ptr_start: 0 as *const u8,
        ptr_end: 0 as *const u8,
    };

    rav1d_init_get_bits(&mut gb, r#in.data, r#in.sz);

    // obu header
    rav1d_get_bit(&mut gb); // obu_forbidden_bit
    let r#type = rav1d_get_bits(&mut gb, 4) as Rav1dObuType;
    let has_extension = rav1d_get_bit(&mut gb) as c_int;
    let has_length_field = rav1d_get_bit(&mut gb) as c_int;
    rav1d_get_bit(&mut gb); // reserved

    let mut temporal_id = 0;
    let mut spatial_id = 0;
    if has_extension != 0 {
        temporal_id = rav1d_get_bits(&mut gb, 3) as c_int;
        spatial_id = rav1d_get_bits(&mut gb, 2) as c_int;
        rav1d_get_bits(&mut gb, 3); // reserved
    }

    // obu length field
    let len = if has_length_field != 0 {
        rav1d_get_uleb128(&mut gb)
    } else {
        r#in.sz as c_uint - 1 - has_extension as c_uint
    };
    if gb.error != 0 {
        return Err(EINVAL);
    }

    let init_bit_pos = rav1d_get_bits_pos(&mut gb);
    let init_byte_pos = init_bit_pos >> 3;

    // We must have read a whole number of bytes at this point
    // (1 byte for the header and whole bytes at a time
    // when reading the leb128 length field).
    assert!(init_bit_pos & 7 == 0);

    // We also know that we haven't tried to read more than `r#in.sz`
    // bytes yet (otherwise the error flag would have been set
    // by the code in [`crate::src::getbits`]).
    assert!(r#in.sz >= init_byte_pos as usize);

    // Make sure that there are enough bits left in the buffer
    // for the rest of the OBU.
    if len as usize > r#in.sz - init_byte_pos as usize {
        return Err(EINVAL);
    }

    // skip obu not belonging to the selected temporal/spatial layer
    if r#type != RAV1D_OBU_SEQ_HDR
        && r#type != RAV1D_OBU_TD
        && has_extension != 0
        && c.operating_point_idc != 0
    {
        let in_temporal_layer = (c.operating_point_idc >> temporal_id & 1) as c_int;
        let in_spatial_layer = (c.operating_point_idc >> spatial_id + 8 & 1) as c_int;
        if in_temporal_layer == 0 || in_spatial_layer == 0 {
            return Ok(len + init_byte_pos);
        }
    }

    unsafe fn parse_tile_grp(
        c: &mut Rav1dContext,
        r#in: &mut Rav1dData,
        gb: &mut GetBits,
        init_bit_pos: c_uint,
        init_byte_pos: c_uint,
        len: c_uint,
    ) -> Rav1dResult {
        if c.frame_hdr.is_null() {
            return Err(EINVAL);
        }

        let hdr = parse_tile_hdr(&(*c.frame_hdr).tiling, gb);
        // Align to the next byte boundary and check for overrun.
        rav1d_bytealign_get_bits(gb);
        if check_for_overrun(c, gb, init_bit_pos, len) != 0 {
            return Err(EINVAL);
        }

        // The current bit position is a multiple of 8
        // (because we just aligned it) and less than `8 * pkt_bytelen`
        // because otherwise the overrun check would have fired.
        let pkt_bytelen = init_byte_pos + len;
        let bit_pos = rav1d_get_bits_pos(gb);
        assert!(bit_pos & 7 == 0);
        assert!(pkt_bytelen >= bit_pos >> 3);
        let mut data = Default::default();
        rav1d_data_ref(&mut data, r#in);
        data.data = data.data.offset((bit_pos >> 3) as isize);
        data.sz = (pkt_bytelen - (bit_pos >> 3)) as usize;
        // Ensure tile groups are in order and sane; see 6.10.1.
        if hdr.start > hdr.end || hdr.start != c.n_tiles {
            for mut tile in c.tiles.drain(..) {
                rav1d_data_unref_internal(&mut tile.data);
            }
            c.n_tiles = 0;
            return Err(EINVAL);
        }
        if let Err(_) = c.tiles.try_reserve_exact(1) {
            return Err(EINVAL);
        }
        c.n_tiles += 1 + hdr.end - hdr.start;
        c.tiles.push(Rav1dTileGroup { data, hdr });

        Ok(())
    }

    match r#type {
        RAV1D_OBU_SEQ_HDR => {
            let mut r#ref = rav1d_ref_create_using_pool(
                c.seq_hdr_pool,
                ::core::mem::size_of::<DRav1d<Rav1dSequenceHeader, Dav1dSequenceHeader>>(),
            );
            if r#ref.is_null() {
                return Err(ENOMEM);
            }
            let seq_hdrs = (*r#ref)
                .data
                .cast::<DRav1d<Rav1dSequenceHeader, Dav1dSequenceHeader>>();
            let seq_hdr = parse_seq_hdr(c, &mut gb).inspect_err(|_| {
                writeln!(c.logger, "Error parsing sequence header");
                rav1d_ref_dec(&mut r#ref);
            })?;
            (*seq_hdrs) = DRav1d::from_rav1d(seq_hdr);
            let seq_hdr = &mut (*seq_hdrs).rav1d as *mut Rav1dSequenceHeader;
            if check_for_overrun(c, &mut gb, init_bit_pos, len) != 0 {
                rav1d_ref_dec(&mut r#ref);
                return Err(EINVAL);
            }
            // If we have read a sequence header which is different from the old one,
            // this is a new video sequence and can't use any previous state.
            // Free that state.

            if c.seq_hdr.is_null() {
                c.frame_hdr = 0 as *mut Rav1dFrameHeader;
                c.frame_flags |= PICTURE_FLAG_NEW_SEQUENCE;
            } else if !(*seq_hdr).eq_without_operating_parameter_info(&*c.seq_hdr) {
                // See 7.5, `operating_parameter_info` is allowed to change in
                // sequence headers of a single sequence.
                c.frame_hdr = 0 as *mut Rav1dFrameHeader;
                c.mastering_display = 0 as *mut Rav1dMasteringDisplay;
                c.content_light = 0 as *mut Rav1dContentLightLevel;
                rav1d_ref_dec(&mut c.mastering_display_ref);
                rav1d_ref_dec(&mut c.content_light_ref);
                for i in 0..8 {
                    if !c.refs[i as usize].p.p.frame_hdr.is_null() {
                        rav1d_thread_picture_unref(&mut c.refs[i as usize].p);
                    }
                    rav1d_ref_dec(&mut c.refs[i as usize].segmap);
                    rav1d_ref_dec(&mut c.refs[i as usize].refmvs);
                    rav1d_cdf_thread_unref(&mut c.cdf[i as usize]);
                }
                c.frame_flags |= PICTURE_FLAG_NEW_SEQUENCE;
            } else if (*seq_hdr).operating_parameter_info != (*c.seq_hdr).operating_parameter_info {
                // If operating_parameter_info changed, signal it
                c.frame_flags |= PICTURE_FLAG_NEW_OP_PARAMS_INFO;
            }
            rav1d_ref_dec(&mut c.seq_hdr_ref);
            c.seq_hdr_ref = r#ref;
            c.seq_hdr = seq_hdr;
        }
        RAV1D_OBU_REDUNDANT_FRAME_HDR if !c.frame_hdr.is_null() => {}
        RAV1D_OBU_REDUNDANT_FRAME_HDR | RAV1D_OBU_FRAME | RAV1D_OBU_FRAME_HDR if global != 0 => {}
        RAV1D_OBU_REDUNDANT_FRAME_HDR | RAV1D_OBU_FRAME | RAV1D_OBU_FRAME_HDR => {
            if c.seq_hdr.is_null() {
                return Err(EINVAL);
            }
            let frame_hdr = parse_frame_hdr(c, &*c.seq_hdr, temporal_id, spatial_id, &mut gb)
                .inspect_err(|_| {
                    writeln!(c.logger, "Error parsing frame header");
                    c.frame_hdr = 0 as *mut Rav1dFrameHeader;
                })?;
            let drav1d_frame_hdr = DRav1d::from_rav1d(frame_hdr);
            if c.frame_hdr_ref.is_null() {
                c.frame_hdr_ref = rav1d_ref_create_using_pool(
                    c.frame_hdr_pool,
                    ::core::mem::size_of::<DRav1d<Rav1dFrameHeader, Dav1dFrameHeader>>(),
                );
                if c.frame_hdr_ref.is_null() {
                    return Err(ENOMEM);
                }
            }
            // ensure that the reference is writable
            debug_assert!(rav1d_ref_is_writable(c.frame_hdr_ref) != 0);
            let drav1d_frame_hdr_ptr = (*c.frame_hdr_ref)
                .data
                .cast::<DRav1d<Rav1dFrameHeader, Dav1dFrameHeader>>();
            drav1d_frame_hdr_ptr.write(drav1d_frame_hdr);
            c.frame_hdr = &mut (*drav1d_frame_hdr_ptr).rav1d;

            for mut tile in c.tiles.drain(..) {
                rav1d_data_unref_internal(&mut tile.data);
            }
            c.n_tiles = 0;
            if r#type != RAV1D_OBU_FRAME {
                // This is actually a frame header OBU,
                // so read the trailing bit and check for overrun.
                rav1d_get_bit(&mut gb);
                if check_for_overrun(c, &mut gb, init_bit_pos, len) != 0 {
                    c.frame_hdr = 0 as *mut Rav1dFrameHeader;
                    return Err(EINVAL);
                }
            }

            if c.frame_size_limit != 0
                && (*c.frame_hdr).size.width[1] as i64 * (*c.frame_hdr).size.height as i64
                    > c.frame_size_limit as i64
            {
                writeln!(
                    c.logger,
                    "Frame size {}x{} exceeds limit {}",
                    (*c.frame_hdr).size.width[1],
                    (*c.frame_hdr).size.height,
                    c.frame_size_limit,
                );
                c.frame_hdr = 0 as *mut Rav1dFrameHeader;
                return Err(ERANGE);
            }

            if r#type == RAV1D_OBU_FRAME {
                // OBU_FRAMEs shouldn't be signaled with `show_existing_frame`.
                if (*c.frame_hdr).show_existing_frame != 0 {
                    c.frame_hdr = 0 as *mut Rav1dFrameHeader;
                    return Err(EINVAL);
                }

                // This is the frame header at the start of a frame OBU.
                // There's no trailing bit at the end to skip,
                // but we do need to align to the next byte.
                rav1d_bytealign_get_bits(&mut gb);
                if global == 0 {
                    parse_tile_grp(c, r#in, &mut gb, init_bit_pos, init_byte_pos, len)?;
                }
            }
        }
        RAV1D_OBU_TILE_GRP => {
            if global == 0 {
                parse_tile_grp(c, r#in, &mut gb, init_bit_pos, init_byte_pos, len)?;
            }
        }
        RAV1D_OBU_METADATA => {
            let debug = Debug::new(false, "OBU", &gb);

            // obu metadata type field
            let meta_type = rav1d_get_uleb128(&mut gb) as ObuMetaType;
            let meta_type_len = ((rav1d_get_bits_pos(&mut gb) - init_bit_pos) >> 3) as c_int;
            if gb.error != 0 {
                return Err(EINVAL);
            }

            match meta_type {
                OBU_META_HDR_CLL => {
                    let debug = debug.named("CLLOBU");
                    let max_content_light_level = rav1d_get_bits(&mut gb, 16) as c_int;
                    debug.log(
                        &gb,
                        format_args!("max-content-light-level: {max_content_light_level}"),
                    );
                    let max_frame_average_light_level = rav1d_get_bits(&mut gb, 16) as c_int;
                    debug.log(
                        &gb,
                        format_args!(
                            "max-frame-average-light-level: {max_frame_average_light_level}"
                        ),
                    );

                    // Skip the trailing bit, align to the next byte boundary and check for overrun.
                    rav1d_get_bit(&mut gb);
                    rav1d_bytealign_get_bits(&mut gb);
                    if check_for_overrun(c, &mut gb, init_bit_pos, len) != 0 {
                        return Err(EINVAL);
                    }

                    let r#ref = rav1d_ref_create(::core::mem::size_of::<Rav1dContentLightLevel>());
                    if r#ref.is_null() {
                        return Err(ENOMEM);
                    }
                    let content_light = (*r#ref).data as *mut Rav1dContentLightLevel;
                    content_light.write(Rav1dContentLightLevel {
                        max_content_light_level,
                        max_frame_average_light_level,
                    });
                    rav1d_ref_dec(&mut c.content_light_ref);
                    c.content_light = content_light;
                    c.content_light_ref = r#ref;
                }
                OBU_META_HDR_MDCV => {
                    let debug = debug.named("MDCVOBU");
                    let primaries = array::from_fn(|i| {
                        let primary = [
                            rav1d_get_bits(&mut gb, 16) as u16,
                            rav1d_get_bits(&mut gb, 16) as u16,
                        ];
                        debug.log(&gb, format_args!("primaries[{i}]: {primary:?}"));
                        primary
                    });
                    let white_point_x = rav1d_get_bits(&mut gb, 16) as u16;
                    debug.log(&gb, format_args!("white-point-x: {white_point_x}"));
                    let white_point_y = rav1d_get_bits(&mut gb, 16) as u16;
                    debug.log(&gb, format_args!("white-point-y: {white_point_y}"));
                    let white_point = [white_point_x, white_point_y];
                    let max_luminance = rav1d_get_bits(&mut gb, 32);
                    debug.log(&gb, format_args!("max-luminance: {max_luminance}"));
                    let min_luminance = rav1d_get_bits(&mut gb, 32);
                    debug.log(&gb, format_args!("min-luminance: {min_luminance}"));
                    // Skip the trailing bit, align to the next byte boundary and check for overrun.
                    rav1d_get_bit(&mut gb);
                    rav1d_bytealign_get_bits(&mut gb);
                    if check_for_overrun(c, &mut gb, init_bit_pos, len) != 0 {
                        return Err(EINVAL);
                    }

                    let r#ref = rav1d_ref_create(::core::mem::size_of::<Rav1dMasteringDisplay>());
                    if r#ref.is_null() {
                        return Err(ENOMEM);
                    }
                    let mastering_display = (*r#ref).data as *mut Rav1dMasteringDisplay;
                    mastering_display.write(Rav1dMasteringDisplay {
                        primaries,
                        white_point,
                        max_luminance,
                        min_luminance,
                    });
                    rav1d_ref_dec(&mut c.mastering_display_ref);
                    c.mastering_display = mastering_display;
                    c.mastering_display_ref = r#ref;
                }
                OBU_META_ITUT_T35 => {
                    let mut payload_size = len as c_int;
                    // Don't take into account all the trailing bits for `payload_size`.
                    while payload_size > 0
                        && *r#in
                            .data
                            .offset((init_byte_pos + payload_size as c_uint - 1) as isize)
                            == 0
                    {
                        payload_size -= 1; // trailing_zero_bit x 8
                    }
                    payload_size -= 1; // trailing_one_bit + trailing_zero_bit x 7

                    // Don't take into account meta_type bytes
                    payload_size -= meta_type_len;

                    let mut country_code_extension_byte = 0;
                    let country_code = rav1d_get_bits(&mut gb, 8) as c_int;
                    payload_size -= 1;
                    if country_code == 0xff {
                        country_code_extension_byte = rav1d_get_bits(&mut gb, 8) as c_int;
                        payload_size -= 1;
                    }

                    if payload_size <= 0 {
                        writeln!(c.logger, "Malformed ITU-T T.35 metadata message format");
                    } else {
                        let r#ref = rav1d_ref_create(
                            ::core::mem::size_of::<DRav1d<Rav1dITUTT35, Dav1dITUTT35>>()
                                + payload_size as usize * ::core::mem::size_of::<u8>(),
                        );
                        if r#ref.is_null() {
                            return Err(ENOMEM);
                        }

                        let country_code = country_code as u8;
                        let country_code_extension_byte = country_code_extension_byte as u8;
                        // We need our public headers to be C++ compatible, so payload can't be
                        // a flexible array member
                        let payload = (*r#ref)
                            .data
                            .cast::<u8>()
                            .offset(::core::mem::size_of::<DRav1d<Rav1dITUTT35, Dav1dITUTT35>>()
                                as isize);
                        let payload_size = payload_size as usize;
                        for i in 0..payload_size {
                            *payload.offset(i as isize) = rav1d_get_bits(&mut gb, 8) as u8;
                        }

                        let itut_t35_metadatas =
                            (*r#ref).data.cast::<DRav1d<Rav1dITUTT35, Dav1dITUTT35>>();
                        itut_t35_metadatas.write(DRav1d::from_rav1d(Rav1dITUTT35 {
                            country_code,
                            country_code_extension_byte,
                            payload,
                            payload_size,
                        }));
                        rav1d_ref_dec(&mut c.itut_t35_ref);
                        c.itut_t35 = &mut (*itut_t35_metadatas).rav1d;
                        c.itut_t35_ref = r#ref;
                    }
                }
                OBU_META_SCALABILITY | OBU_META_TIMECODE => {} // Ignore metadata OBUs we don't care about.
                _ => {
                    // Print a warning, but don't fail for unknown types.
                    writeln!(c.logger, "Unknown Metadata OBU type {meta_type}",);
                }
            }
        }
        RAV1D_OBU_TD => c.frame_flags |= PICTURE_FLAG_NEW_TEMPORAL_UNIT,
        RAV1D_OBU_PADDING => {} // Ignore OBUs we don't care about.
        _ => {
            // Print a warning, but don't fail for unknown types.
            writeln!(
                c.logger,
                "Unknown OBU type {} of size {}",
                r#type as c_uint, len,
            );
        }
    }

    if !c.seq_hdr.is_null() && !c.frame_hdr.is_null() {
        if (*c.frame_hdr).show_existing_frame != 0 {
            if c.refs[(*c.frame_hdr).existing_frame_idx as usize]
                .p
                .p
                .frame_hdr
                .is_null()
            {
                return Err(EINVAL);
            }
            match (*c.refs[(*c.frame_hdr).existing_frame_idx as usize]
                .p
                .p
                .frame_hdr)
                .frame_type
            {
                Rav1dFrameType::Inter | Rav1dFrameType::Switch => {
                    if c.decode_frame_type > RAV1D_DECODEFRAMETYPE_REFERENCE {
                        return Ok(skip(c, len, init_byte_pos));
                    }
                }
                Rav1dFrameType::Intra => {
                    if c.decode_frame_type > RAV1D_DECODEFRAMETYPE_INTRA {
                        return Ok(skip(c, len, init_byte_pos));
                    }
                }
                _ => {}
            }
            if c.refs[(*c.frame_hdr).existing_frame_idx as usize].p.p.data[0].is_null() {
                return Err(EINVAL);
            }
            if c.strict_std_compliance
                && !c.refs[(*c.frame_hdr).existing_frame_idx as usize]
                    .p
                    .showable
            {
                return Err(EINVAL);
            }
            if c.n_fc == 1 {
                rav1d_thread_picture_ref(
                    &mut c.out,
                    &mut c.refs[(*c.frame_hdr).existing_frame_idx as usize].p,
                );
                rav1d_picture_copy_props(
                    &mut (*c).out.p,
                    c.content_light,
                    c.content_light_ref,
                    c.mastering_display,
                    c.mastering_display_ref,
                    c.itut_t35,
                    c.itut_t35_ref,
                    &mut r#in.m,
                );
                // Must be removed from the context after being attached to the frame
                rav1d_ref_dec(&mut c.itut_t35_ref);
                c.itut_t35 = 0 as *mut Rav1dITUTT35;
                c.event_flags |= rav1d_picture_get_event_flags(
                    &mut c.refs[(*c.frame_hdr).existing_frame_idx as usize].p,
                );
            } else {
                pthread_mutex_lock(&mut c.task_thread.lock);
                // Need to append this to the frame output queue.
                let next = c.frame_thread.next;
                c.frame_thread.next += 1;
                if c.frame_thread.next == c.n_fc {
                    c.frame_thread.next = 0;
                }

                let f = &mut *c.fc.offset(next as isize);
                while !(*f).tiles.is_empty() {
                    pthread_cond_wait(
                        &mut (*f).task_thread.cond,
                        &mut (*(*f).task_thread.ttd).lock,
                    );
                }
                let out_delayed = &mut *c.frame_thread.out_delayed.offset(next as isize);
                if !(*out_delayed).p.data[0].is_null()
                    || ::core::intrinsics::atomic_load_seqcst(
                        &mut (*f).task_thread.error as *mut atomic_int,
                    ) != 0
                {
                    let first = ::core::intrinsics::atomic_load_seqcst(&mut c.task_thread.first);
                    if first + 1 < c.n_fc {
                        ::core::intrinsics::atomic_xadd_seqcst(&mut c.task_thread.first, 1);
                    } else {
                        ::core::intrinsics::atomic_store_seqcst(&mut c.task_thread.first, 0);
                    }
                    ::core::intrinsics::atomic_cxchg_seqcst_seqcst(
                        &mut c.task_thread.reset_task_cur,
                        first,
                        u32::MAX,
                    );
                    if c.task_thread.cur != 0 && c.task_thread.cur < c.n_fc {
                        c.task_thread.cur -= 1;
                    }
                }
                let error = (*f).task_thread.retval;
                if error.is_err() {
                    c.cached_error = error;
                    (*f).task_thread.retval = Ok(());
                    rav1d_data_props_copy(&mut c.cached_error_props, &mut (*out_delayed).p.m);
                    rav1d_thread_picture_unref(out_delayed);
                } else if !((*out_delayed).p.data[0]).is_null() {
                    let progress = ::core::intrinsics::atomic_load_relaxed(
                        &mut *((*out_delayed).progress).offset(1) as *mut atomic_uint,
                    );
                    if ((*out_delayed).visible || c.output_invisible_frames)
                        && progress != FRAME_ERROR
                    {
                        rav1d_thread_picture_ref(&mut c.out, out_delayed);
                        c.event_flags |= rav1d_picture_get_event_flags(out_delayed);
                    }
                    rav1d_thread_picture_unref(out_delayed);
                }
                rav1d_thread_picture_ref(
                    out_delayed,
                    &mut c.refs[(*c.frame_hdr).existing_frame_idx as usize].p,
                );
                (*out_delayed).visible = true;
                rav1d_picture_copy_props(
                    &mut (*out_delayed).p,
                    c.content_light,
                    c.content_light_ref,
                    c.mastering_display,
                    c.mastering_display_ref,
                    c.itut_t35,
                    c.itut_t35_ref,
                    &mut r#in.m,
                );
                // Must be removed from the context after being attached to the frame
                rav1d_ref_dec(&mut c.itut_t35_ref);
                c.itut_t35 = 0 as *mut Rav1dITUTT35;
                pthread_mutex_unlock(&mut c.task_thread.lock);
            }
            if (*c.refs[(*c.frame_hdr).existing_frame_idx as usize]
                .p
                .p
                .frame_hdr)
                .frame_type
                == Rav1dFrameType::Key
            {
                let r = (*c.frame_hdr).existing_frame_idx;
                c.refs[r as usize].p.showable = false;
                for i in 0..8 {
                    if i == r {
                        continue;
                    }

                    if !c.refs[i as usize].p.p.frame_hdr.is_null() {
                        rav1d_thread_picture_unref(&mut c.refs[i as usize].p);
                    }
                    rav1d_thread_picture_ref(&mut c.refs[i as usize].p, &mut c.refs[r as usize].p);

                    rav1d_cdf_thread_unref(&mut c.cdf[i as usize]);
                    rav1d_cdf_thread_ref(&mut c.cdf[i as usize], &mut c.cdf[r as usize]);

                    rav1d_ref_dec(&mut c.refs[i as usize].segmap);
                    c.refs[i as usize].segmap = c.refs[r as usize].segmap;
                    if !c.refs[r as usize].segmap.is_null() {
                        rav1d_ref_inc(c.refs[r as usize].segmap);
                    }
                    rav1d_ref_dec(&mut c.refs[i as usize].refmvs);
                }
            }
            c.frame_hdr = 0 as *mut Rav1dFrameHeader;
        } else if c.n_tiles == (*c.frame_hdr).tiling.cols * (*c.frame_hdr).tiling.rows {
            match (*c.frame_hdr).frame_type {
                Rav1dFrameType::Inter | Rav1dFrameType::Switch => {
                    if c.decode_frame_type > RAV1D_DECODEFRAMETYPE_REFERENCE
                        || c.decode_frame_type == RAV1D_DECODEFRAMETYPE_REFERENCE
                            && (*c.frame_hdr).refresh_frame_flags == 0
                    {
                        return Ok(skip(c, len, init_byte_pos));
                    }
                }
                Rav1dFrameType::Intra => {
                    if c.decode_frame_type > RAV1D_DECODEFRAMETYPE_INTRA
                        || c.decode_frame_type == RAV1D_DECODEFRAMETYPE_REFERENCE
                            && (*c.frame_hdr).refresh_frame_flags == 0
                    {
                        return Ok(skip(c, len, init_byte_pos));
                    }
                }
                _ => {}
            }
            if c.tiles.is_empty() {
                return Err(EINVAL);
            }
            rav1d_submit_frame(&mut *c)?;
            assert!(c.tiles.is_empty());
            c.frame_hdr = 0 as *mut Rav1dFrameHeader;
            c.n_tiles = 0;
        }
    }

    Ok(len + init_byte_pos)
}

pub(crate) unsafe fn rav1d_parse_obus(
    c: &mut Rav1dContext,
    r#in: &mut Rav1dData,
    global: c_int,
) -> Rav1dResult<c_uint> {
    parse_obus(c, r#in, global).inspect_err(|_| {
        rav1d_data_props_copy(&mut c.cached_error_props, &mut r#in.m);
        writeln!(c.logger, "Error parsing OBU data");
    })
}
