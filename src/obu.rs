use crate::include::common::intops::iclip_u8;
use crate::include::common::intops::ulog2;
use crate::include::dav1d::data::Rav1dData;
use crate::include::dav1d::dav1d::Dav1dEventFlags;
use crate::include::dav1d::dav1d::RAV1D_DECODEFRAMETYPE_INTRA;
use crate::include::dav1d::dav1d::RAV1D_DECODEFRAMETYPE_REFERENCE;
use crate::include::dav1d::headers::DRav1d;
use crate::include::dav1d::headers::Dav1dAdaptiveBoolean;
use crate::include::dav1d::headers::Dav1dChromaSamplePosition;
use crate::include::dav1d::headers::Dav1dColorPrimaries;
use crate::include::dav1d::headers::Dav1dFilterMode;
use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::dav1d::headers::Dav1dFrameType;
use crate::include::dav1d::headers::Dav1dITUTT35;
use crate::include::dav1d::headers::Dav1dMatrixCoefficients;
use crate::include::dav1d::headers::Dav1dSequenceHeader;
use crate::include::dav1d::headers::Dav1dSequenceHeaderOperatingParameterInfo;
use crate::include::dav1d::headers::Dav1dTransferCharacteristics;
use crate::include::dav1d::headers::Dav1dTxfmMode;
use crate::include::dav1d::headers::Dav1dWarpedMotionType;
use crate::include::dav1d::headers::Rav1dContentLightLevel;
use crate::include::dav1d::headers::Rav1dFilmGrainData;
use crate::include::dav1d::headers::Rav1dFrameHeader;
use crate::include::dav1d::headers::Rav1dFrameHeaderOperatingPoint;
use crate::include::dav1d::headers::Rav1dITUTT35;
use crate::include::dav1d::headers::Rav1dLoopfilterModeRefDeltas;
use crate::include::dav1d::headers::Rav1dMasteringDisplay;
use crate::include::dav1d::headers::Rav1dObuType;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::include::dav1d::headers::Rav1dRestorationType;
use crate::include::dav1d::headers::Rav1dSegmentationData;
use crate::include::dav1d::headers::Rav1dSegmentationDataSet;
use crate::include::dav1d::headers::Rav1dSequenceHeader;
use crate::include::dav1d::headers::Rav1dSequenceHeaderOperatingParameterInfo;
use crate::include::dav1d::headers::Rav1dSequenceHeaderOperatingPoint;
use crate::include::dav1d::headers::Rav1dWarpedMotionParams;
use crate::include::dav1d::headers::RAV1D_ADAPTIVE;
use crate::include::dav1d::headers::RAV1D_CHR_UNKNOWN;
use crate::include::dav1d::headers::RAV1D_COLOR_PRI_BT709;
use crate::include::dav1d::headers::RAV1D_COLOR_PRI_UNKNOWN;
use crate::include::dav1d::headers::RAV1D_FILTER_SWITCHABLE;
use crate::include::dav1d::headers::RAV1D_FRAME_TYPE_INTER;
use crate::include::dav1d::headers::RAV1D_FRAME_TYPE_INTRA;
use crate::include::dav1d::headers::RAV1D_FRAME_TYPE_KEY;
use crate::include::dav1d::headers::RAV1D_FRAME_TYPE_SWITCH;
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
use crate::src::internal::Rav1dFrameContext;
use crate::src::internal::Rav1dTileGroup;
use crate::src::levels::ObuMetaType;
use crate::src::levels::OBU_META_HDR_CLL;
use crate::src::levels::OBU_META_HDR_MDCV;
use crate::src::levels::OBU_META_ITUT_T35;
use crate::src::levels::OBU_META_SCALABILITY;
use crate::src::levels::OBU_META_TIMECODE;
use crate::src::log::rav1d_log;
use crate::src::picture::rav1d_picture_get_event_flags;
use crate::src::picture::rav1d_thread_picture_ref;
use crate::src::picture::rav1d_thread_picture_unref;
use crate::src::picture::PictureFlags;
use crate::src::picture::Rav1dThreadPicture;
use crate::src::picture::PICTURE_FLAG_NEW_OP_PARAMS_INFO;
use crate::src::picture::PICTURE_FLAG_NEW_SEQUENCE;
use crate::src::picture::PICTURE_FLAG_NEW_TEMPORAL_UNIT;
use crate::src::r#ref::rav1d_ref_create;
use crate::src::r#ref::rav1d_ref_create_using_pool;
use crate::src::r#ref::rav1d_ref_dec;
use crate::src::r#ref::rav1d_ref_inc;
use crate::src::r#ref::rav1d_ref_is_writable;
use crate::src::r#ref::Rav1dRef;
use crate::src::tables::dav1d_default_wm_params;
use crate::src::thread_task::FRAME_ERROR;
use libc::memcmp;
use libc::memset;
use libc::pthread_cond_wait;
use libc::pthread_mutex_lock;
use libc::pthread_mutex_unlock;
use libc::realloc;
use std::cmp;
use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_long;
use std::ffi::c_uint;
use std::ffi::c_ulong;
use std::ffi::c_void;
use std::ptr::addr_of_mut;

#[inline]
unsafe fn rav1d_get_bits_pos(c: &GetBits) -> c_uint {
    (c.ptr.offset_from(c.ptr_start) as c_long as c_uint)
        .wrapping_mul(8 as c_int as c_uint)
        .wrapping_sub(c.bits_left as c_uint)
}

unsafe fn parse_seq_hdr(
    c: *mut Rav1dContext,
    gb: *mut GetBits,
    hdr: *mut Rav1dSequenceHeader,
) -> Rav1dResult {
    unsafe fn error(c: *mut Rav1dContext) -> Rav1dResult {
        rav1d_log(
            c,
            b"Error parsing sequence header\n\0" as *const u8 as *const c_char,
        );
        return Err(EINVAL);
    }

    memset(
        hdr as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<Rav1dSequenceHeader>(),
    );
    (*hdr).profile = rav1d_get_bits(gb, 3 as c_int) as c_int;
    if (*hdr).profile > 2 {
        return error(c);
    }
    (*hdr).still_picture = rav1d_get_bit(gb) as c_int;
    (*hdr).reduced_still_picture_header = rav1d_get_bit(gb) as c_int;
    if (*hdr).reduced_still_picture_header != 0 && (*hdr).still_picture == 0 {
        return error(c);
    }
    if (*hdr).reduced_still_picture_header != 0 {
        (*hdr).num_operating_points = 1 as c_int;
        (*hdr).operating_points[0].major_level = rav1d_get_bits(gb, 3 as c_int) as c_int;
        (*hdr).operating_points[0].minor_level = rav1d_get_bits(gb, 2 as c_int) as c_int;
        (*hdr).operating_points[0].initial_display_delay = 10 as c_int;
    } else {
        (*hdr).timing_info_present = rav1d_get_bit(gb) as c_int;
        if (*hdr).timing_info_present != 0 {
            (*hdr).num_units_in_tick = rav1d_get_bits(gb, 32 as c_int) as c_int;
            (*hdr).time_scale = rav1d_get_bits(gb, 32 as c_int) as c_int;
            if (*c).strict_std_compliance
                && ((*hdr).num_units_in_tick == 0 || (*hdr).time_scale == 0)
            {
                return error(c);
            }
            (*hdr).equal_picture_interval = rav1d_get_bit(gb) as c_int;
            if (*hdr).equal_picture_interval != 0 {
                let num_ticks_per_picture: c_uint = rav1d_get_vlc(gb);
                if num_ticks_per_picture == 0xffffffff as c_uint {
                    return error(c);
                }
                (*hdr).num_ticks_per_picture =
                    num_ticks_per_picture.wrapping_add(1 as c_int as c_uint);
            }
            (*hdr).decoder_model_info_present = rav1d_get_bit(gb) as c_int;
            if (*hdr).decoder_model_info_present != 0 {
                (*hdr).encoder_decoder_buffer_delay_length =
                    (rav1d_get_bits(gb, 5 as c_int)).wrapping_add(1 as c_int as c_uint) as c_int;
                (*hdr).num_units_in_decoding_tick = rav1d_get_bits(gb, 32 as c_int) as c_int;
                if (*c).strict_std_compliance && (*hdr).num_units_in_decoding_tick == 0 {
                    return error(c);
                }
                (*hdr).buffer_removal_delay_length =
                    (rav1d_get_bits(gb, 5 as c_int)).wrapping_add(1 as c_int as c_uint) as c_int;
                (*hdr).frame_presentation_delay_length =
                    (rav1d_get_bits(gb, 5 as c_int)).wrapping_add(1 as c_int as c_uint) as c_int;
            }
        }
        (*hdr).display_model_info_present = rav1d_get_bit(gb) as c_int;
        (*hdr).num_operating_points =
            (rav1d_get_bits(gb, 5 as c_int)).wrapping_add(1 as c_int as c_uint) as c_int;
        let mut i = 0;
        while i < (*hdr).num_operating_points {
            let op: *mut Rav1dSequenceHeaderOperatingPoint =
                &mut *((*hdr).operating_points).as_mut_ptr().offset(i as isize)
                    as *mut Rav1dSequenceHeaderOperatingPoint;
            (*op).idc = rav1d_get_bits(gb, 12 as c_int) as c_int;
            if (*op).idc != 0 && ((*op).idc & 0xff as c_int == 0 || (*op).idc & 0xf00 == 0) {
                return error(c);
            }
            (*op).major_level =
                (2 as c_int as c_uint).wrapping_add(rav1d_get_bits(gb, 3 as c_int)) as c_int;
            (*op).minor_level = rav1d_get_bits(gb, 2 as c_int) as c_int;
            if (*op).major_level > 3 {
                (*op).tier = rav1d_get_bit(gb) as c_int;
            }
            if (*hdr).decoder_model_info_present != 0 {
                (*op).decoder_model_param_present = rav1d_get_bit(gb) as c_int;
                if (*op).decoder_model_param_present != 0 {
                    let opi: *mut Rav1dSequenceHeaderOperatingParameterInfo = &mut *((*hdr)
                        .operating_parameter_info)
                        .as_mut_ptr()
                        .offset(i as isize)
                        as *mut Rav1dSequenceHeaderOperatingParameterInfo;
                    (*opi).decoder_buffer_delay =
                        rav1d_get_bits(gb, (*hdr).encoder_decoder_buffer_delay_length) as c_int;
                    (*opi).encoder_buffer_delay =
                        rav1d_get_bits(gb, (*hdr).encoder_decoder_buffer_delay_length) as c_int;
                    (*opi).low_delay_mode = rav1d_get_bit(gb) as c_int;
                }
            }
            if (*hdr).display_model_info_present != 0 {
                (*op).display_model_param_present = rav1d_get_bit(gb) as c_int;
            }
            (*op).initial_display_delay = (if (*op).display_model_param_present != 0 {
                (rav1d_get_bits(gb, 4 as c_int)).wrapping_add(1 as c_int as c_uint)
            } else {
                10 as c_int as c_uint
            }) as c_int;
            i += 1;
        }
    }
    let op_idx: c_int = if (*c).operating_point < (*hdr).num_operating_points {
        (*c).operating_point
    } else {
        0 as c_int
    };
    (*c).operating_point_idc = (*hdr).operating_points[op_idx as usize].idc as c_uint;
    let spatial_mask = (*c).operating_point_idc >> 8;
    (*c).max_spatial_id = if spatial_mask != 0 {
        ulog2(spatial_mask) != 0
    } else {
        false
    };
    (*hdr).width_n_bits =
        (rav1d_get_bits(gb, 4 as c_int)).wrapping_add(1 as c_int as c_uint) as c_int;
    (*hdr).height_n_bits =
        (rav1d_get_bits(gb, 4 as c_int)).wrapping_add(1 as c_int as c_uint) as c_int;
    (*hdr).max_width =
        (rav1d_get_bits(gb, (*hdr).width_n_bits)).wrapping_add(1 as c_int as c_uint) as c_int;
    (*hdr).max_height =
        (rav1d_get_bits(gb, (*hdr).height_n_bits)).wrapping_add(1 as c_int as c_uint) as c_int;
    if (*hdr).reduced_still_picture_header == 0 {
        (*hdr).frame_id_numbers_present = rav1d_get_bit(gb) as c_int;
        if (*hdr).frame_id_numbers_present != 0 {
            (*hdr).delta_frame_id_n_bits =
                (rav1d_get_bits(gb, 4 as c_int)).wrapping_add(2 as c_int as c_uint) as c_int;
            (*hdr).frame_id_n_bits = (rav1d_get_bits(gb, 3 as c_int))
                .wrapping_add((*hdr).delta_frame_id_n_bits as c_uint)
                .wrapping_add(1 as c_int as c_uint) as c_int;
        }
    }
    (*hdr).sb128 = rav1d_get_bit(gb) as c_int;
    (*hdr).filter_intra = rav1d_get_bit(gb) as c_int;
    (*hdr).intra_edge_filter = rav1d_get_bit(gb) as c_int;
    if (*hdr).reduced_still_picture_header != 0 {
        (*hdr).screen_content_tools = RAV1D_ADAPTIVE;
        (*hdr).force_integer_mv = RAV1D_ADAPTIVE;
    } else {
        (*hdr).inter_intra = rav1d_get_bit(gb) as c_int;
        (*hdr).masked_compound = rav1d_get_bit(gb) as c_int;
        (*hdr).warped_motion = rav1d_get_bit(gb) as c_int;
        (*hdr).dual_filter = rav1d_get_bit(gb) as c_int;
        (*hdr).order_hint = rav1d_get_bit(gb) as c_int;
        if (*hdr).order_hint != 0 {
            (*hdr).jnt_comp = rav1d_get_bit(gb) as c_int;
            (*hdr).ref_frame_mvs = rav1d_get_bit(gb) as c_int;
        }
        (*hdr).screen_content_tools = (if rav1d_get_bit(gb) != 0 {
            RAV1D_ADAPTIVE as c_int as c_uint
        } else {
            rav1d_get_bit(gb)
        }) as Dav1dAdaptiveBoolean;
        (*hdr).force_integer_mv = (if (*hdr).screen_content_tools as c_uint != 0 {
            if rav1d_get_bit(gb) != 0 {
                RAV1D_ADAPTIVE as c_int as c_uint
            } else {
                rav1d_get_bit(gb)
            }
        } else {
            2 as c_int as c_uint
        }) as Dav1dAdaptiveBoolean;
        if (*hdr).order_hint != 0 {
            (*hdr).order_hint_n_bits =
                (rav1d_get_bits(gb, 3 as c_int)).wrapping_add(1 as c_int as c_uint) as c_int;
        }
    }
    (*hdr).super_res = rav1d_get_bit(gb) as c_int;
    (*hdr).cdef = rav1d_get_bit(gb) as c_int;
    (*hdr).restoration = rav1d_get_bit(gb) as c_int;
    (*hdr).hbd = rav1d_get_bit(gb) as c_int;
    if (*hdr).profile == 2 && (*hdr).hbd != 0 {
        (*hdr).hbd = ((*hdr).hbd as c_uint).wrapping_add(rav1d_get_bit(gb)) as c_int as c_int;
    }
    if (*hdr).profile != 1 as c_int {
        (*hdr).monochrome = rav1d_get_bit(gb) as c_int;
    }
    (*hdr).color_description_present = rav1d_get_bit(gb) as c_int;
    if (*hdr).color_description_present != 0 {
        (*hdr).pri = rav1d_get_bits(gb, 8 as c_int) as Dav1dColorPrimaries;
        (*hdr).trc = rav1d_get_bits(gb, 8 as c_int) as Dav1dTransferCharacteristics;
        (*hdr).mtrx = rav1d_get_bits(gb, 8 as c_int) as Dav1dMatrixCoefficients;
    } else {
        (*hdr).pri = RAV1D_COLOR_PRI_UNKNOWN;
        (*hdr).trc = RAV1D_TRC_UNKNOWN;
        (*hdr).mtrx = RAV1D_MC_UNKNOWN;
    }
    if (*hdr).monochrome != 0 {
        (*hdr).color_range = rav1d_get_bit(gb) as c_int;
        (*hdr).layout = Rav1dPixelLayout::I400;
        (*hdr).ss_ver = 1 as c_int;
        (*hdr).ss_hor = (*hdr).ss_ver;
        (*hdr).chr = RAV1D_CHR_UNKNOWN;
    } else if (*hdr).pri as c_uint == RAV1D_COLOR_PRI_BT709 as c_int as c_uint
        && (*hdr).trc as c_uint == RAV1D_TRC_SRGB as c_int as c_uint
        && (*hdr).mtrx as c_uint == RAV1D_MC_IDENTITY as c_int as c_uint
    {
        (*hdr).layout = Rav1dPixelLayout::I444;
        (*hdr).color_range = 1 as c_int;
        if (*hdr).profile != 1 && !((*hdr).profile == 2 && (*hdr).hbd == 2) {
            return error(c);
        }
    } else {
        (*hdr).color_range = rav1d_get_bit(gb) as c_int;
        match (*hdr).profile {
            0 => {
                (*hdr).layout = Rav1dPixelLayout::I420;
                (*hdr).ss_ver = 1 as c_int;
                (*hdr).ss_hor = (*hdr).ss_ver;
            }
            1 => {
                (*hdr).layout = Rav1dPixelLayout::I444;
            }
            2 => {
                if (*hdr).hbd == 2 {
                    (*hdr).ss_hor = rav1d_get_bit(gb) as c_int;
                    if (*hdr).ss_hor != 0 {
                        (*hdr).ss_ver = rav1d_get_bit(gb) as c_int;
                    }
                } else {
                    (*hdr).ss_hor = 1 as c_int;
                }
                (*hdr).layout = if (*hdr).ss_hor != 0 {
                    if (*hdr).ss_ver != 0 {
                        Rav1dPixelLayout::I420
                    } else {
                        Rav1dPixelLayout::I422
                    }
                } else {
                    Rav1dPixelLayout::I444
                };
            }
            _ => {}
        }
        (*hdr).chr = (if (*hdr).ss_hor & (*hdr).ss_ver != 0 {
            rav1d_get_bits(gb, 2 as c_int)
        } else {
            RAV1D_CHR_UNKNOWN as c_int as c_uint
        }) as Dav1dChromaSamplePosition;
    }
    if (*c).strict_std_compliance
        && (*hdr).mtrx as c_uint == RAV1D_MC_IDENTITY as c_int as c_uint
        && (*hdr).layout as c_uint != Rav1dPixelLayout::I444 as c_int as c_uint
    {
        return error(c);
    }
    if (*hdr).monochrome == 0 {
        (*hdr).separate_uv_delta_q = rav1d_get_bit(gb) as c_int;
    }
    (*hdr).film_grain_present = rav1d_get_bit(gb) as c_int;
    rav1d_get_bit(gb);
    Ok(())
}

unsafe fn read_frame_size(c: *mut Rav1dContext, gb: *mut GetBits, use_ref: c_int) -> c_int {
    let seqhdr: *const Rav1dSequenceHeader = (*c).seq_hdr;
    let hdr: *mut Rav1dFrameHeader = (*c).frame_hdr;
    if use_ref != 0 {
        let mut i = 0;
        while i < 7 {
            if rav1d_get_bit(gb) != 0 {
                let r#ref: *const Rav1dThreadPicture = &mut (*((*c).refs)
                    .as_mut_ptr()
                    .offset(*((*(*c).frame_hdr).refidx).as_mut_ptr().offset(i as isize) as isize))
                .p;
                if ((*r#ref).p.frame_hdr).is_null() {
                    return -(1 as c_int);
                }
                (*hdr).width[1] = (*(*r#ref).p.frame_hdr).width[1];
                (*hdr).height = (*(*r#ref).p.frame_hdr).height;
                (*hdr).render_width = (*(*r#ref).p.frame_hdr).render_width;
                (*hdr).render_height = (*(*r#ref).p.frame_hdr).render_height;
                (*hdr).super_res.enabled =
                    ((*seqhdr).super_res != 0 && rav1d_get_bit(gb) != 0) as c_int;
                if (*hdr).super_res.enabled != 0 {
                    (*hdr).super_res.width_scale_denominator = (9 as c_int as c_uint)
                        .wrapping_add(rav1d_get_bits(gb, 3 as c_int))
                        as c_int;
                    let d = (*hdr).super_res.width_scale_denominator;
                    (*hdr).width[0] = cmp::max(
                        ((*hdr).width[1] * 8 + (d >> 1)) / d,
                        cmp::min(16 as c_int, (*hdr).width[1]),
                    );
                } else {
                    (*hdr).super_res.width_scale_denominator = 8 as c_int;
                    (*hdr).width[0] = (*hdr).width[1];
                }
                return 0 as c_int;
            }
            i += 1;
        }
    }
    if (*hdr).frame_size_override != 0 {
        (*hdr).width[1] = (rav1d_get_bits(gb, (*seqhdr).width_n_bits))
            .wrapping_add(1 as c_int as c_uint) as c_int;
        (*hdr).height = (rav1d_get_bits(gb, (*seqhdr).height_n_bits))
            .wrapping_add(1 as c_int as c_uint) as c_int;
    } else {
        (*hdr).width[1] = (*seqhdr).max_width;
        (*hdr).height = (*seqhdr).max_height;
    }
    (*hdr).super_res.enabled = ((*seqhdr).super_res != 0 && rav1d_get_bit(gb) != 0) as c_int;
    if (*hdr).super_res.enabled != 0 {
        (*hdr).super_res.width_scale_denominator =
            (9 as c_int as c_uint).wrapping_add(rav1d_get_bits(gb, 3 as c_int)) as c_int;
        let d = (*hdr).super_res.width_scale_denominator;
        (*hdr).width[0] = cmp::max(
            ((*hdr).width[1] * 8 + (d >> 1)) / d,
            cmp::min(16 as c_int, (*hdr).width[1]),
        );
    } else {
        (*hdr).super_res.width_scale_denominator = 8 as c_int;
        (*hdr).width[0] = (*hdr).width[1];
    }
    (*hdr).have_render_size = rav1d_get_bit(gb) as c_int;
    if (*hdr).have_render_size != 0 {
        (*hdr).render_width =
            (rav1d_get_bits(gb, 16 as c_int)).wrapping_add(1 as c_int as c_uint) as c_int;
        (*hdr).render_height =
            (rav1d_get_bits(gb, 16 as c_int)).wrapping_add(1 as c_int as c_uint) as c_int;
    } else {
        (*hdr).render_width = (*hdr).width[1];
        (*hdr).render_height = (*hdr).height;
    }
    0
}

#[inline]
unsafe fn tile_log2(sz: c_int, tgt: c_int) -> c_int {
    let mut k;
    k = 0 as c_int;
    while sz << k < tgt {
        k += 1;
    }
    k
}

static default_mode_ref_deltas: Rav1dLoopfilterModeRefDeltas = Rav1dLoopfilterModeRefDeltas {
    mode_delta: [0, 0],
    ref_delta: [1, 0, 0, 0, -1, 0, -1, -1],
};

unsafe fn parse_frame_hdr(c: *mut Rav1dContext, gb: *mut GetBits) -> Rav1dResult {
    unsafe fn error(c: *mut Rav1dContext) -> Rav1dResult {
        rav1d_log(
            c,
            b"Error parsing frame header\n\0" as *const u8 as *const c_char,
        );
        return Err(EINVAL);
    }

    let seqhdr: *const Rav1dSequenceHeader = (*c).seq_hdr;
    let hdr: *mut Rav1dFrameHeader = (*c).frame_hdr;
    (*hdr).show_existing_frame =
        ((*seqhdr).reduced_still_picture_header == 0 && rav1d_get_bit(gb) != 0) as c_int;
    if (*hdr).show_existing_frame != 0 {
        (*hdr).existing_frame_idx = rav1d_get_bits(gb, 3 as c_int) as c_int;
        if (*seqhdr).decoder_model_info_present != 0 && (*seqhdr).equal_picture_interval == 0 {
            (*hdr).frame_presentation_delay =
                rav1d_get_bits(gb, (*seqhdr).frame_presentation_delay_length) as c_int;
        }
        if (*seqhdr).frame_id_numbers_present != 0 {
            (*hdr).frame_id = rav1d_get_bits(gb, (*seqhdr).frame_id_n_bits) as c_int;
            let ref_frame_hdr: *mut Rav1dFrameHeader =
                (*c).refs[(*hdr).existing_frame_idx as usize].p.p.frame_hdr;
            if ref_frame_hdr.is_null() || (*ref_frame_hdr).frame_id != (*hdr).frame_id {
                return error(c);
            }
        }
        return Ok(());
    }
    (*hdr).frame_type = (if (*seqhdr).reduced_still_picture_header != 0 {
        RAV1D_FRAME_TYPE_KEY as c_int as c_uint
    } else {
        rav1d_get_bits(gb, 2 as c_int)
    }) as Dav1dFrameType;
    (*hdr).show_frame =
        ((*seqhdr).reduced_still_picture_header != 0 || rav1d_get_bit(gb) != 0) as c_int;
    if (*hdr).show_frame != 0 {
        if (*seqhdr).decoder_model_info_present != 0 && (*seqhdr).equal_picture_interval == 0 {
            (*hdr).frame_presentation_delay =
                rav1d_get_bits(gb, (*seqhdr).frame_presentation_delay_length) as c_int;
        }
        (*hdr).showable_frame =
            ((*hdr).frame_type as c_uint != RAV1D_FRAME_TYPE_KEY as c_int as c_uint) as c_int;
    } else {
        (*hdr).showable_frame = rav1d_get_bit(gb) as c_int;
    }
    (*hdr).error_resilient_mode = ((*hdr).frame_type as c_uint
        == RAV1D_FRAME_TYPE_KEY as c_int as c_uint
        && (*hdr).show_frame != 0
        || (*hdr).frame_type as c_uint == RAV1D_FRAME_TYPE_SWITCH as c_int as c_uint
        || (*seqhdr).reduced_still_picture_header != 0
        || rav1d_get_bit(gb) != 0) as c_int;
    (*hdr).disable_cdf_update = rav1d_get_bit(gb) as c_int;
    (*hdr).allow_screen_content_tools =
        (if (*seqhdr).screen_content_tools as c_uint == RAV1D_ADAPTIVE as c_int as c_uint {
            rav1d_get_bit(gb)
        } else {
            (*seqhdr).screen_content_tools as c_uint
        }) as c_int;
    if (*hdr).allow_screen_content_tools != 0 {
        (*hdr).force_integer_mv =
            (if (*seqhdr).force_integer_mv as c_uint == RAV1D_ADAPTIVE as c_int as c_uint {
                rav1d_get_bit(gb)
            } else {
                (*seqhdr).force_integer_mv as c_uint
            }) as c_int;
    } else {
        (*hdr).force_integer_mv = 0 as c_int;
    }
    if (*hdr).frame_type as c_uint & 1 as c_uint == 0 {
        (*hdr).force_integer_mv = 1 as c_int;
    }
    if (*seqhdr).frame_id_numbers_present != 0 {
        (*hdr).frame_id = rav1d_get_bits(gb, (*seqhdr).frame_id_n_bits) as c_int;
    }
    (*hdr).frame_size_override = (if (*seqhdr).reduced_still_picture_header != 0 {
        0 as c_int as c_uint
    } else if (*hdr).frame_type as c_uint == RAV1D_FRAME_TYPE_SWITCH as c_int as c_uint {
        1 as c_int as c_uint
    } else {
        rav1d_get_bit(gb)
    }) as c_int;
    (*hdr).frame_offset = (if (*seqhdr).order_hint != 0 {
        rav1d_get_bits(gb, (*seqhdr).order_hint_n_bits)
    } else {
        0 as c_int as c_uint
    }) as c_int;
    (*hdr).primary_ref_frame =
        (if (*hdr).error_resilient_mode == 0 && (*hdr).frame_type as c_uint & 1 as c_uint != 0 {
            rav1d_get_bits(gb, 3 as c_int)
        } else {
            7 as c_int as c_uint
        }) as c_int;
    if (*seqhdr).decoder_model_info_present != 0 {
        (*hdr).buffer_removal_time_present = rav1d_get_bit(gb) as c_int;
        if (*hdr).buffer_removal_time_present != 0 {
            let mut i = 0;
            while i < (*(*c).seq_hdr).num_operating_points {
                let seqop: *const Rav1dSequenceHeaderOperatingPoint =
                    &*((*seqhdr).operating_points).as_ptr().offset(i as isize)
                        as *const Rav1dSequenceHeaderOperatingPoint;
                let op: *mut Rav1dFrameHeaderOperatingPoint =
                    &mut *((*hdr).operating_points).as_mut_ptr().offset(i as isize)
                        as *mut Rav1dFrameHeaderOperatingPoint;
                if (*seqop).decoder_model_param_present != 0 {
                    let in_temporal_layer = (*seqop).idc >> (*hdr).temporal_id & 1;
                    let in_spatial_layer = (*seqop).idc >> (*hdr).spatial_id + 8 & 1;
                    if (*seqop).idc == 0 || in_temporal_layer != 0 && in_spatial_layer != 0 {
                        (*op).buffer_removal_time =
                            rav1d_get_bits(gb, (*seqhdr).buffer_removal_delay_length) as c_int;
                    }
                }
                i += 1;
            }
        }
    }
    if (*hdr).frame_type as c_uint & 1 as c_uint == 0 {
        (*hdr).refresh_frame_flags = (if (*hdr).frame_type as c_uint
            == RAV1D_FRAME_TYPE_KEY as c_int as c_uint
            && (*hdr).show_frame != 0
        {
            0xff as c_int as c_uint
        } else {
            rav1d_get_bits(gb, 8 as c_int)
        }) as c_int;
        if (*hdr).refresh_frame_flags != 0xff as c_int
            && (*hdr).error_resilient_mode != 0
            && (*seqhdr).order_hint != 0
        {
            let mut i = 0;
            while i < 8 {
                rav1d_get_bits(gb, (*seqhdr).order_hint_n_bits);
                i += 1;
            }
        }
        if (*c).strict_std_compliance
            && (*hdr).frame_type as c_uint == RAV1D_FRAME_TYPE_INTRA as c_int as c_uint
            && (*hdr).refresh_frame_flags == 0xff as c_int
        {
            return error(c);
        }
        if read_frame_size(c, gb, 0 as c_int) < 0 {
            return error(c);
        }
        (*hdr).allow_intrabc = ((*hdr).allow_screen_content_tools != 0
            && (*hdr).super_res.enabled == 0
            && rav1d_get_bit(gb) != 0) as c_int;
        (*hdr).use_ref_frame_mvs = 0 as c_int;
    } else {
        (*hdr).allow_intrabc = 0 as c_int;
        (*hdr).refresh_frame_flags =
            (if (*hdr).frame_type as c_uint == RAV1D_FRAME_TYPE_SWITCH as c_int as c_uint {
                0xff as c_int as c_uint
            } else {
                rav1d_get_bits(gb, 8 as c_int)
            }) as c_int;
        if (*hdr).error_resilient_mode != 0 && (*seqhdr).order_hint != 0 {
            let mut i = 0;
            while i < 8 {
                rav1d_get_bits(gb, (*seqhdr).order_hint_n_bits);
                i += 1;
            }
        }
        (*hdr).frame_ref_short_signaling =
            ((*seqhdr).order_hint != 0 && rav1d_get_bit(gb) != 0) as c_int;
        if (*hdr).frame_ref_short_signaling != 0 {
            (*hdr).refidx[0] = rav1d_get_bits(gb, 3 as c_int) as c_int;
            (*hdr).refidx[2] = -(1 as c_int);
            (*hdr).refidx[1] = (*hdr).refidx[2];
            (*hdr).refidx[3] = rav1d_get_bits(gb, 3 as c_int) as c_int;
            (*hdr).refidx[6] = -(1 as c_int);
            (*hdr).refidx[5] = (*hdr).refidx[6];
            (*hdr).refidx[4] = (*hdr).refidx[5];
            let mut shifted_frame_offset: [c_int; 8] = [0; 8];
            let current_frame_offset: c_int = (1 as c_int) << (*seqhdr).order_hint_n_bits - 1;
            let mut i = 0;
            while i < 8 {
                if ((*c).refs[i as usize].p.p.frame_hdr).is_null() {
                    return error(c);
                }
                shifted_frame_offset[i as usize] = current_frame_offset
                    + get_poc_diff(
                        (*seqhdr).order_hint_n_bits,
                        (*(*c).refs[i as usize].p.p.frame_hdr).frame_offset,
                        (*hdr).frame_offset,
                    );
                i += 1;
            }
            let mut used_frame: [c_int; 8] = [0 as c_int, 0, 0, 0, 0, 0, 0, 0];
            used_frame[(*hdr).refidx[0] as usize] = 1 as c_int;
            used_frame[(*hdr).refidx[3] as usize] = 1 as c_int;
            let mut latest_frame_offset: c_int = -(1 as c_int);
            let mut i = 0;
            while i < 8 {
                let hint: c_int = shifted_frame_offset[i as usize];
                if used_frame[i as usize] == 0
                    && hint >= current_frame_offset
                    && hint >= latest_frame_offset
                {
                    (*hdr).refidx[6] = i;
                    latest_frame_offset = hint;
                }
                i += 1;
            }
            if latest_frame_offset != -(1 as c_int) {
                used_frame[(*hdr).refidx[6] as usize] = 1 as c_int;
            }
            let mut earliest_frame_offset = i32::MAX;
            let mut i = 0;
            while i < 8 {
                let hint: c_int = shifted_frame_offset[i as usize];
                if used_frame[i as usize] == 0
                    && hint >= current_frame_offset
                    && hint < earliest_frame_offset
                {
                    (*hdr).refidx[4] = i;
                    earliest_frame_offset = hint;
                }
                i += 1;
            }
            if earliest_frame_offset != i32::MAX {
                used_frame[(*hdr).refidx[4] as usize] = 1 as c_int;
            }
            earliest_frame_offset = i32::MAX;
            let mut i = 0;
            while i < 8 {
                let hint: c_int = shifted_frame_offset[i as usize];
                if used_frame[i as usize] == 0
                    && hint >= current_frame_offset
                    && hint < earliest_frame_offset
                {
                    (*hdr).refidx[5] = i;
                    earliest_frame_offset = hint;
                }
                i += 1;
            }
            if earliest_frame_offset != i32::MAX {
                used_frame[(*hdr).refidx[5] as usize] = 1 as c_int;
            }
            let mut i = 1;
            while i < 7 {
                if (*hdr).refidx[i as usize] < 0 {
                    latest_frame_offset = -(1 as c_int);
                    let mut j = 0;
                    while j < 8 {
                        let hint: c_int = shifted_frame_offset[j as usize];
                        if used_frame[j as usize] == 0
                            && hint < current_frame_offset
                            && hint >= latest_frame_offset
                        {
                            (*hdr).refidx[i as usize] = j;
                            latest_frame_offset = hint;
                        }
                        j += 1;
                    }
                    if latest_frame_offset != -(1 as c_int) {
                        used_frame[(*hdr).refidx[i as usize] as usize] = 1 as c_int;
                    }
                }
                i += 1;
            }
            earliest_frame_offset = i32::MAX;
            let mut r#ref: c_int = -(1 as c_int);
            let mut i = 0;
            while i < 8 {
                let hint: c_int = shifted_frame_offset[i as usize];
                if hint < earliest_frame_offset {
                    r#ref = i;
                    earliest_frame_offset = hint;
                }
                i += 1;
            }
            let mut i = 0;
            while i < 7 {
                if (*hdr).refidx[i as usize] < 0 {
                    (*hdr).refidx[i as usize] = r#ref;
                }
                i += 1;
            }
        }
        let mut i = 0;
        while i < 7 {
            if (*hdr).frame_ref_short_signaling == 0 {
                (*hdr).refidx[i as usize] = rav1d_get_bits(gb, 3 as c_int) as c_int;
            }
            if (*seqhdr).frame_id_numbers_present != 0 {
                let delta_ref_frame_id_minus_1: c_int =
                    rav1d_get_bits(gb, (*seqhdr).delta_frame_id_n_bits) as c_int;
                let ref_frame_id: c_int = (*hdr).frame_id
                    + ((1 as c_int) << (*seqhdr).frame_id_n_bits)
                    - delta_ref_frame_id_minus_1
                    - 1
                    & ((1 as c_int) << (*seqhdr).frame_id_n_bits) - 1;
                let ref_frame_hdr: *mut Rav1dFrameHeader =
                    (*c).refs[(*hdr).refidx[i as usize] as usize].p.p.frame_hdr;
                if ref_frame_hdr.is_null() || (*ref_frame_hdr).frame_id != ref_frame_id {
                    return error(c);
                }
            }
            i += 1;
        }
        let use_ref: c_int =
            ((*hdr).error_resilient_mode == 0 && (*hdr).frame_size_override != 0) as c_int;
        if read_frame_size(c, gb, use_ref) < 0 {
            return error(c);
        }
        (*hdr).hp = ((*hdr).force_integer_mv == 0 && rav1d_get_bit(gb) != 0) as c_int;
        (*hdr).subpel_filter_mode = (if rav1d_get_bit(gb) != 0 {
            RAV1D_FILTER_SWITCHABLE as c_int as c_uint
        } else {
            rav1d_get_bits(gb, 2 as c_int)
        }) as Dav1dFilterMode;
        (*hdr).switchable_motion_mode = rav1d_get_bit(gb) as c_int;
        (*hdr).use_ref_frame_mvs = ((*hdr).error_resilient_mode == 0
            && (*seqhdr).ref_frame_mvs != 0
            && (*seqhdr).order_hint != 0
            && (*hdr).frame_type as c_uint & 1 as c_uint != 0
            && rav1d_get_bit(gb) != 0) as c_int;
    }
    (*hdr).refresh_context = ((*seqhdr).reduced_still_picture_header == 0
        && (*hdr).disable_cdf_update == 0
        && rav1d_get_bit(gb) == 0) as c_int;
    (*hdr).tiling.uniform = rav1d_get_bit(gb) as c_int;
    let sbsz_min1: c_int = ((64 as c_int) << (*seqhdr).sb128) - 1;
    let sbsz_log2 = 6 + (*seqhdr).sb128;
    let sbw: c_int = (*hdr).width[0] + sbsz_min1 >> sbsz_log2;
    let sbh: c_int = (*hdr).height + sbsz_min1 >> sbsz_log2;
    let max_tile_width_sb = 4096 >> sbsz_log2;
    let max_tile_area_sb = 4096 * 2304 >> 2 * sbsz_log2;
    (*hdr).tiling.min_log2_cols = tile_log2(max_tile_width_sb, sbw);
    (*hdr).tiling.max_log2_cols = tile_log2(1 as c_int, cmp::min(sbw, 64 as c_int));
    (*hdr).tiling.max_log2_rows = tile_log2(1 as c_int, cmp::min(sbh, 64 as c_int));
    let min_log2_tiles: c_int = cmp::max(
        tile_log2(max_tile_area_sb, sbw * sbh),
        (*hdr).tiling.min_log2_cols,
    );
    if (*hdr).tiling.uniform != 0 {
        (*hdr).tiling.log2_cols = (*hdr).tiling.min_log2_cols;
        while (*hdr).tiling.log2_cols < (*hdr).tiling.max_log2_cols && rav1d_get_bit(gb) != 0 {
            (*hdr).tiling.log2_cols += 1;
        }
        let tile_w = 1 + (sbw - 1 >> (*hdr).tiling.log2_cols);
        (*hdr).tiling.cols = 0 as c_int;
        let mut sbx = 0;
        while sbx < sbw {
            (*hdr).tiling.col_start_sb[(*hdr).tiling.cols as usize] = sbx as u16;
            sbx += tile_w;
            (*hdr).tiling.cols += 1;
        }
        (*hdr).tiling.min_log2_rows =
            cmp::max(min_log2_tiles - (*hdr).tiling.log2_cols, 0 as c_int);
        (*hdr).tiling.log2_rows = (*hdr).tiling.min_log2_rows;
        while (*hdr).tiling.log2_rows < (*hdr).tiling.max_log2_rows && rav1d_get_bit(gb) != 0 {
            (*hdr).tiling.log2_rows += 1;
        }
        let tile_h = 1 + (sbh - 1 >> (*hdr).tiling.log2_rows);
        (*hdr).tiling.rows = 0 as c_int;
        let mut sby = 0;
        while sby < sbh {
            (*hdr).tiling.row_start_sb[(*hdr).tiling.rows as usize] = sby as u16;
            sby += tile_h;
            (*hdr).tiling.rows += 1;
        }
    } else {
        (*hdr).tiling.cols = 0 as c_int;
        let mut widest_tile = 0;
        let mut max_tile_area_sb: c_int = sbw * sbh;
        let mut sbx = 0;
        while sbx < sbw && (*hdr).tiling.cols < 64 {
            let tile_width_sb: c_int = cmp::min(sbw - sbx, max_tile_width_sb);
            let tile_w: c_int = (if tile_width_sb > 1 {
                (1 as c_int as c_uint).wrapping_add(rav1d_get_uniform(gb, tile_width_sb as c_uint))
            } else {
                1 as c_int as c_uint
            }) as c_int;
            (*hdr).tiling.col_start_sb[(*hdr).tiling.cols as usize] = sbx as u16;
            sbx += tile_w;
            widest_tile = cmp::max(widest_tile, tile_w);
            (*hdr).tiling.cols += 1;
        }
        (*hdr).tiling.log2_cols = tile_log2(1 as c_int, (*hdr).tiling.cols);
        if min_log2_tiles != 0 {
            max_tile_area_sb >>= min_log2_tiles + 1;
        }
        let max_tile_height_sb: c_int = cmp::max(max_tile_area_sb / widest_tile, 1 as c_int);
        (*hdr).tiling.rows = 0 as c_int;
        let mut sby = 0;
        while sby < sbh && (*hdr).tiling.rows < 64 {
            let tile_height_sb: c_int = cmp::min(sbh - sby, max_tile_height_sb);
            let tile_h: c_int = (if tile_height_sb > 1 {
                (1 as c_int as c_uint).wrapping_add(rav1d_get_uniform(gb, tile_height_sb as c_uint))
            } else {
                1 as c_int as c_uint
            }) as c_int;
            (*hdr).tiling.row_start_sb[(*hdr).tiling.rows as usize] = sby as u16;
            sby += tile_h;
            (*hdr).tiling.rows += 1;
        }
        (*hdr).tiling.log2_rows = tile_log2(1 as c_int, (*hdr).tiling.rows);
    }
    (*hdr).tiling.col_start_sb[(*hdr).tiling.cols as usize] = sbw as u16;
    (*hdr).tiling.row_start_sb[(*hdr).tiling.rows as usize] = sbh as u16;
    if (*hdr).tiling.log2_cols != 0 || (*hdr).tiling.log2_rows != 0 {
        (*hdr).tiling.update =
            rav1d_get_bits(gb, (*hdr).tiling.log2_cols + (*hdr).tiling.log2_rows) as c_int;
        if (*hdr).tiling.update >= (*hdr).tiling.cols * (*hdr).tiling.rows {
            return error(c);
        }
        (*hdr).tiling.n_bytes = (rav1d_get_bits(gb, 2 as c_int)).wrapping_add(1 as c_int as c_uint);
    } else {
        (*hdr).tiling.update = 0 as c_int;
        (*hdr).tiling.n_bytes = (*hdr).tiling.update as c_uint;
    }
    (*hdr).quant.yac = rav1d_get_bits(gb, 8 as c_int) as c_int;
    (*hdr).quant.ydc_delta = if rav1d_get_bit(gb) != 0 {
        rav1d_get_sbits(gb, 7 as c_int)
    } else {
        0 as c_int
    };
    if (*seqhdr).monochrome == 0 {
        let diff_uv_delta: c_int = (if (*seqhdr).separate_uv_delta_q != 0 {
            rav1d_get_bit(gb)
        } else {
            0 as c_int as c_uint
        }) as c_int;
        (*hdr).quant.udc_delta = if rav1d_get_bit(gb) != 0 {
            rav1d_get_sbits(gb, 7 as c_int)
        } else {
            0 as c_int
        };
        (*hdr).quant.uac_delta = if rav1d_get_bit(gb) != 0 {
            rav1d_get_sbits(gb, 7 as c_int)
        } else {
            0 as c_int
        };
        if diff_uv_delta != 0 {
            (*hdr).quant.vdc_delta = if rav1d_get_bit(gb) != 0 {
                rav1d_get_sbits(gb, 7 as c_int)
            } else {
                0 as c_int
            };
            (*hdr).quant.vac_delta = if rav1d_get_bit(gb) != 0 {
                rav1d_get_sbits(gb, 7 as c_int)
            } else {
                0 as c_int
            };
        } else {
            (*hdr).quant.vdc_delta = (*hdr).quant.udc_delta;
            (*hdr).quant.vac_delta = (*hdr).quant.uac_delta;
        }
    }
    (*hdr).quant.qm = rav1d_get_bit(gb) as c_int;
    if (*hdr).quant.qm != 0 {
        (*hdr).quant.qm_y = rav1d_get_bits(gb, 4 as c_int) as c_int;
        (*hdr).quant.qm_u = rav1d_get_bits(gb, 4 as c_int) as c_int;
        (*hdr).quant.qm_v = if (*seqhdr).separate_uv_delta_q != 0 {
            rav1d_get_bits(gb, 4 as c_int) as c_int
        } else {
            (*hdr).quant.qm_u
        };
    }
    (*hdr).segmentation.enabled = rav1d_get_bit(gb) as c_int;
    if (*hdr).segmentation.enabled != 0 {
        if (*hdr).primary_ref_frame == 7 {
            (*hdr).segmentation.update_map = 1 as c_int;
            (*hdr).segmentation.temporal = 0 as c_int;
            (*hdr).segmentation.update_data = 1 as c_int;
        } else {
            (*hdr).segmentation.update_map = rav1d_get_bit(gb) as c_int;
            (*hdr).segmentation.temporal = (if (*hdr).segmentation.update_map != 0 {
                rav1d_get_bit(gb)
            } else {
                0 as c_int as c_uint
            }) as c_int;
            (*hdr).segmentation.update_data = rav1d_get_bit(gb) as c_int;
        }
        if (*hdr).segmentation.update_data != 0 {
            (*hdr).segmentation.seg_data.preskip = 0 as c_int;
            (*hdr).segmentation.seg_data.last_active_segid = -(1 as c_int);
            let mut i = 0;
            while i < 8 {
                let seg: *mut Rav1dSegmentationData = &mut *((*hdr).segmentation.seg_data.d)
                    .as_mut_ptr()
                    .offset(i as isize)
                    as *mut Rav1dSegmentationData;
                if rav1d_get_bit(gb) != 0 {
                    (*seg).delta_q = rav1d_get_sbits(gb, 9 as c_int);
                    (*hdr).segmentation.seg_data.last_active_segid = i;
                } else {
                    (*seg).delta_q = 0 as c_int;
                }
                if rav1d_get_bit(gb) != 0 {
                    (*seg).delta_lf_y_v = rav1d_get_sbits(gb, 7 as c_int);
                    (*hdr).segmentation.seg_data.last_active_segid = i;
                } else {
                    (*seg).delta_lf_y_v = 0 as c_int;
                }
                if rav1d_get_bit(gb) != 0 {
                    (*seg).delta_lf_y_h = rav1d_get_sbits(gb, 7 as c_int);
                    (*hdr).segmentation.seg_data.last_active_segid = i;
                } else {
                    (*seg).delta_lf_y_h = 0 as c_int;
                }
                if rav1d_get_bit(gb) != 0 {
                    (*seg).delta_lf_u = rav1d_get_sbits(gb, 7 as c_int);
                    (*hdr).segmentation.seg_data.last_active_segid = i;
                } else {
                    (*seg).delta_lf_u = 0 as c_int;
                }
                if rav1d_get_bit(gb) != 0 {
                    (*seg).delta_lf_v = rav1d_get_sbits(gb, 7 as c_int);
                    (*hdr).segmentation.seg_data.last_active_segid = i;
                } else {
                    (*seg).delta_lf_v = 0 as c_int;
                }
                if rav1d_get_bit(gb) != 0 {
                    (*seg).r#ref = rav1d_get_bits(gb, 3 as c_int) as c_int;
                    (*hdr).segmentation.seg_data.last_active_segid = i;
                    (*hdr).segmentation.seg_data.preskip = 1 as c_int;
                } else {
                    (*seg).r#ref = -(1 as c_int);
                }
                (*seg).skip = rav1d_get_bit(gb) as c_int;
                if (*seg).skip != 0 {
                    (*hdr).segmentation.seg_data.last_active_segid = i;
                    (*hdr).segmentation.seg_data.preskip = 1 as c_int;
                }
                (*seg).globalmv = rav1d_get_bit(gb) as c_int;
                if (*seg).globalmv != 0 {
                    (*hdr).segmentation.seg_data.last_active_segid = i;
                    (*hdr).segmentation.seg_data.preskip = 1 as c_int;
                }
                i += 1;
            }
        } else {
            if !((*hdr).primary_ref_frame != 7 as c_int) {
                unreachable!();
            }
            let pri_ref: c_int = (*hdr).refidx[(*hdr).primary_ref_frame as usize];
            if ((*c).refs[pri_ref as usize].p.p.frame_hdr).is_null() {
                return error(c);
            }
            (*hdr).segmentation.seg_data = (*(*c).refs[pri_ref as usize].p.p.frame_hdr)
                .segmentation
                .seg_data
                .clone();
        }
    } else {
        memset(
            &mut (*hdr).segmentation.seg_data as *mut Rav1dSegmentationDataSet as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<Rav1dSegmentationDataSet>(),
        );
        let mut i = 0;
        while i < 8 {
            (*hdr).segmentation.seg_data.d[i as usize].r#ref = -(1 as c_int);
            i += 1;
        }
    }
    (*hdr).delta.q.present = (if (*hdr).quant.yac != 0 {
        rav1d_get_bit(gb)
    } else {
        0 as c_int as c_uint
    }) as c_int;
    (*hdr).delta.q.res_log2 = (if (*hdr).delta.q.present != 0 {
        rav1d_get_bits(gb, 2 as c_int)
    } else {
        0 as c_int as c_uint
    }) as c_int;
    (*hdr).delta.lf.present = ((*hdr).delta.q.present != 0
        && (*hdr).allow_intrabc == 0
        && rav1d_get_bit(gb) != 0) as c_int;
    (*hdr).delta.lf.res_log2 = (if (*hdr).delta.lf.present != 0 {
        rav1d_get_bits(gb, 2 as c_int)
    } else {
        0 as c_int as c_uint
    }) as c_int;
    (*hdr).delta.lf.multi = (if (*hdr).delta.lf.present != 0 {
        rav1d_get_bit(gb)
    } else {
        0 as c_int as c_uint
    }) as c_int;
    let delta_lossless: c_int = ((*hdr).quant.ydc_delta == 0
        && (*hdr).quant.udc_delta == 0
        && (*hdr).quant.uac_delta == 0
        && (*hdr).quant.vdc_delta == 0
        && (*hdr).quant.vac_delta == 0) as c_int;
    (*hdr).all_lossless = 1 as c_int;
    let mut i = 0;
    while i < 8 {
        (*hdr).segmentation.qidx[i as usize] = if (*hdr).segmentation.enabled != 0 {
            iclip_u8((*hdr).quant.yac + (*hdr).segmentation.seg_data.d[i as usize].delta_q)
        } else {
            (*hdr).quant.yac
        };
        (*hdr).segmentation.lossless[i as usize] =
            ((*hdr).segmentation.qidx[i as usize] == 0 && delta_lossless != 0) as c_int;
        (*hdr).all_lossless &= (*hdr).segmentation.lossless[i as usize];
        i += 1;
    }
    if (*hdr).all_lossless != 0 || (*hdr).allow_intrabc != 0 {
        (*hdr).loopfilter.level_y[1] = 0 as c_int;
        (*hdr).loopfilter.level_y[0] = (*hdr).loopfilter.level_y[1];
        (*hdr).loopfilter.level_v = 0 as c_int;
        (*hdr).loopfilter.level_u = (*hdr).loopfilter.level_v;
        (*hdr).loopfilter.sharpness = 0 as c_int;
        (*hdr).loopfilter.mode_ref_delta_enabled = 1 as c_int;
        (*hdr).loopfilter.mode_ref_delta_update = 1 as c_int;
        (*hdr).loopfilter.mode_ref_deltas = default_mode_ref_deltas.clone();
    } else {
        (*hdr).loopfilter.level_y[0] = rav1d_get_bits(gb, 6 as c_int) as c_int;
        (*hdr).loopfilter.level_y[1] = rav1d_get_bits(gb, 6 as c_int) as c_int;
        if (*seqhdr).monochrome == 0
            && ((*hdr).loopfilter.level_y[0] != 0 || (*hdr).loopfilter.level_y[1] != 0)
        {
            (*hdr).loopfilter.level_u = rav1d_get_bits(gb, 6 as c_int) as c_int;
            (*hdr).loopfilter.level_v = rav1d_get_bits(gb, 6 as c_int) as c_int;
        }
        (*hdr).loopfilter.sharpness = rav1d_get_bits(gb, 3 as c_int) as c_int;
        if (*hdr).primary_ref_frame == 7 {
            (*hdr).loopfilter.mode_ref_deltas = default_mode_ref_deltas.clone();
        } else {
            let r#ref: c_int = (*hdr).refidx[(*hdr).primary_ref_frame as usize];
            if ((*c).refs[r#ref as usize].p.p.frame_hdr).is_null() {
                return error(c);
            }
            (*hdr).loopfilter.mode_ref_deltas = (*(*c).refs[r#ref as usize].p.p.frame_hdr)
                .loopfilter
                .mode_ref_deltas
                .clone();
        }
        (*hdr).loopfilter.mode_ref_delta_enabled = rav1d_get_bit(gb) as c_int;
        if (*hdr).loopfilter.mode_ref_delta_enabled != 0 {
            (*hdr).loopfilter.mode_ref_delta_update = rav1d_get_bit(gb) as c_int;
            if (*hdr).loopfilter.mode_ref_delta_update != 0 {
                let mut i = 0;
                while i < 8 {
                    if rav1d_get_bit(gb) != 0 {
                        (*hdr).loopfilter.mode_ref_deltas.ref_delta[i as usize] =
                            rav1d_get_sbits(gb, 7 as c_int);
                    }
                    i += 1;
                }
                let mut i = 0;
                while i < 2 {
                    if rav1d_get_bit(gb) != 0 {
                        (*hdr).loopfilter.mode_ref_deltas.mode_delta[i as usize] =
                            rav1d_get_sbits(gb, 7 as c_int);
                    }
                    i += 1;
                }
            }
        }
    }
    if (*hdr).all_lossless == 0 && (*seqhdr).cdef != 0 && (*hdr).allow_intrabc == 0 {
        (*hdr).cdef.damping =
            (rav1d_get_bits(gb, 2 as c_int)).wrapping_add(3 as c_int as c_uint) as c_int;
        (*hdr).cdef.n_bits = rav1d_get_bits(gb, 2 as c_int) as c_int;
        let mut i = 0;
        while i < (1 as c_int) << (*hdr).cdef.n_bits {
            (*hdr).cdef.y_strength[i as usize] = rav1d_get_bits(gb, 6 as c_int) as c_int;
            if (*seqhdr).monochrome == 0 {
                (*hdr).cdef.uv_strength[i as usize] = rav1d_get_bits(gb, 6 as c_int) as c_int;
            }
            i += 1;
        }
    } else {
        (*hdr).cdef.n_bits = 0 as c_int;
        (*hdr).cdef.y_strength[0] = 0 as c_int;
        (*hdr).cdef.uv_strength[0] = 0 as c_int;
    }
    if ((*hdr).all_lossless == 0 || (*hdr).super_res.enabled != 0)
        && (*seqhdr).restoration != 0
        && (*hdr).allow_intrabc == 0
    {
        (*hdr).restoration.r#type[0] = rav1d_get_bits(gb, 2 as c_int) as Rav1dRestorationType;
        if (*seqhdr).monochrome == 0 {
            (*hdr).restoration.r#type[1] = rav1d_get_bits(gb, 2 as c_int) as Rav1dRestorationType;
            (*hdr).restoration.r#type[2] = rav1d_get_bits(gb, 2 as c_int) as Rav1dRestorationType;
        } else {
            (*hdr).restoration.r#type[2] = RAV1D_RESTORATION_NONE;
            (*hdr).restoration.r#type[1] = (*hdr).restoration.r#type[2];
        }
        if (*hdr).restoration.r#type[0] as c_uint != 0
            || (*hdr).restoration.r#type[1] as c_uint != 0
            || (*hdr).restoration.r#type[2] as c_uint != 0
        {
            (*hdr).restoration.unit_size[0] = 6 + (*seqhdr).sb128;
            if rav1d_get_bit(gb) != 0 {
                (*hdr).restoration.unit_size[0] += 1;
                if (*seqhdr).sb128 == 0 {
                    (*hdr).restoration.unit_size[0] = ((*hdr).restoration.unit_size[0] as c_uint)
                        .wrapping_add(rav1d_get_bit(gb))
                        as c_int as c_int;
                }
            }
            (*hdr).restoration.unit_size[1] = (*hdr).restoration.unit_size[0];
            if ((*hdr).restoration.r#type[1] as c_uint != 0
                || (*hdr).restoration.r#type[2] as c_uint != 0)
                && (*seqhdr).ss_hor == 1
                && (*seqhdr).ss_ver == 1
            {
                (*hdr).restoration.unit_size[1] = ((*hdr).restoration.unit_size[1] as c_uint)
                    .wrapping_sub(rav1d_get_bit(gb))
                    as c_int as c_int;
            }
        } else {
            (*hdr).restoration.unit_size[0] = 8 as c_int;
        }
    } else {
        (*hdr).restoration.r#type[0] = RAV1D_RESTORATION_NONE;
        (*hdr).restoration.r#type[1] = RAV1D_RESTORATION_NONE;
        (*hdr).restoration.r#type[2] = RAV1D_RESTORATION_NONE;
    }
    (*hdr).txfm_mode = (if (*hdr).all_lossless != 0 {
        RAV1D_TX_4X4_ONLY as c_int
    } else if rav1d_get_bit(gb) != 0 {
        RAV1D_TX_SWITCHABLE as c_int
    } else {
        RAV1D_TX_LARGEST as c_int
    }) as Dav1dTxfmMode;
    (*hdr).switchable_comp_refs = (if (*hdr).frame_type as c_uint & 1 as c_uint != 0 {
        rav1d_get_bit(gb)
    } else {
        0 as c_int as c_uint
    }) as c_int;
    (*hdr).skip_mode_allowed = 0 as c_int;
    if (*hdr).switchable_comp_refs != 0
        && (*hdr).frame_type as c_uint & 1 as c_uint != 0
        && (*seqhdr).order_hint != 0
    {
        let poc: c_uint = (*hdr).frame_offset as c_uint;
        let mut off_before: c_uint = 0xffffffff as c_uint;
        let mut off_after: c_int = -(1 as c_int);
        let mut off_before_idx = 0;
        let mut off_after_idx = 0;
        let mut i = 0;
        while i < 7 {
            if ((*c).refs[(*hdr).refidx[i as usize] as usize].p.p.frame_hdr).is_null() {
                return error(c);
            }
            let refpoc: c_uint = (*(*c).refs[(*hdr).refidx[i as usize] as usize].p.p.frame_hdr)
                .frame_offset as c_uint;
            let diff: c_int =
                get_poc_diff((*seqhdr).order_hint_n_bits, refpoc as c_int, poc as c_int);
            if diff > 0 {
                if off_after == -(1 as c_int)
                    || get_poc_diff((*seqhdr).order_hint_n_bits, off_after, refpoc as c_int) > 0
                {
                    off_after = refpoc as c_int;
                    off_after_idx = i;
                }
            } else if diff < 0
                && (off_before == 0xffffffff as c_uint
                    || get_poc_diff(
                        (*seqhdr).order_hint_n_bits,
                        refpoc as c_int,
                        off_before as c_int,
                    ) > 0)
            {
                off_before = refpoc;
                off_before_idx = i;
            }
            i += 1;
        }
        if off_before != 0xffffffff as c_uint && off_after != -(1 as c_int) {
            (*hdr).skip_mode_refs[0] = cmp::min(off_before_idx, off_after_idx);
            (*hdr).skip_mode_refs[1] = cmp::max(off_before_idx, off_after_idx);
            (*hdr).skip_mode_allowed = 1 as c_int;
        } else if off_before != 0xffffffff as c_uint {
            let mut off_before2: c_uint = 0xffffffff as c_uint;
            let mut off_before2_idx = 0;
            let mut i = 0;
            while i < 7 {
                if ((*c).refs[(*hdr).refidx[i as usize] as usize].p.p.frame_hdr).is_null() {
                    return error(c);
                }
                let refpoc: c_uint = (*(*c).refs[(*hdr).refidx[i as usize] as usize].p.p.frame_hdr)
                    .frame_offset as c_uint;
                if get_poc_diff(
                    (*seqhdr).order_hint_n_bits,
                    refpoc as c_int,
                    off_before as c_int,
                ) < 0
                {
                    if off_before2 == 0xffffffff as c_uint
                        || get_poc_diff(
                            (*seqhdr).order_hint_n_bits,
                            refpoc as c_int,
                            off_before2 as c_int,
                        ) > 0
                    {
                        off_before2 = refpoc;
                        off_before2_idx = i;
                    }
                }
                i += 1;
            }
            if off_before2 != 0xffffffff as c_uint {
                (*hdr).skip_mode_refs[0] = cmp::min(off_before_idx, off_before2_idx);
                (*hdr).skip_mode_refs[1] = cmp::max(off_before_idx, off_before2_idx);
                (*hdr).skip_mode_allowed = 1 as c_int;
            }
        }
    }
    (*hdr).skip_mode_enabled = (if (*hdr).skip_mode_allowed != 0 {
        rav1d_get_bit(gb)
    } else {
        0 as c_int as c_uint
    }) as c_int;
    (*hdr).warp_motion = ((*hdr).error_resilient_mode == 0
        && (*hdr).frame_type as c_uint & 1 as c_uint != 0
        && (*seqhdr).warped_motion != 0
        && rav1d_get_bit(gb) != 0) as c_int;
    (*hdr).reduced_txtp_set = rav1d_get_bit(gb) as c_int;
    let mut i = 0;
    while i < 7 {
        (*hdr).gmv[i as usize] = dav1d_default_wm_params.clone();
        i += 1;
    }
    if (*hdr).frame_type as c_uint & 1 as c_uint != 0 {
        let mut i = 0;
        while i < 7 {
            (*hdr).gmv[i as usize].r#type = (if rav1d_get_bit(gb) == 0 {
                RAV1D_WM_TYPE_IDENTITY as c_int
            } else if rav1d_get_bit(gb) != 0 {
                RAV1D_WM_TYPE_ROT_ZOOM as c_int
            } else if rav1d_get_bit(gb) != 0 {
                RAV1D_WM_TYPE_TRANSLATION as c_int
            } else {
                RAV1D_WM_TYPE_AFFINE as c_int
            }) as Dav1dWarpedMotionType;
            if !((*hdr).gmv[i as usize].r#type as c_uint
                == RAV1D_WM_TYPE_IDENTITY as c_int as c_uint)
            {
                let ref_gmv: *const Rav1dWarpedMotionParams;
                if (*hdr).primary_ref_frame == 7 {
                    ref_gmv = &dav1d_default_wm_params;
                } else {
                    let pri_ref: c_int = (*hdr).refidx[(*hdr).primary_ref_frame as usize];
                    if ((*c).refs[pri_ref as usize].p.p.frame_hdr).is_null() {
                        return error(c);
                    }
                    ref_gmv = &mut *((*(*((*c).refs).as_mut_ptr().offset(pri_ref as isize))
                        .p
                        .p
                        .frame_hdr)
                        .gmv)
                        .as_mut_ptr()
                        .offset(i as isize)
                        as *mut Rav1dWarpedMotionParams;
                }
                let mat: *mut i32 = ((*hdr).gmv[i as usize].matrix).as_mut_ptr();
                let ref_mat: *const i32 = ((*ref_gmv).matrix).as_ptr();
                let bits: c_int;
                let shift: c_int;
                if (*hdr).gmv[i as usize].r#type as c_uint
                    >= RAV1D_WM_TYPE_ROT_ZOOM as c_int as c_uint
                {
                    *mat.offset(2) = ((1 as c_int) << 16)
                        + 2 * rav1d_get_bits_subexp(
                            gb,
                            *ref_mat.offset(2) - ((1 as c_int) << 16) >> 1,
                            12 as c_int as c_uint,
                        );
                    *mat.offset(3) = 2 as c_int
                        * rav1d_get_bits_subexp(gb, *ref_mat.offset(3) >> 1, 12 as c_int as c_uint);
                    bits = 12 as c_int;
                    shift = 10 as c_int;
                } else {
                    bits = 9 - ((*hdr).hp == 0) as c_int;
                    shift = 13 + ((*hdr).hp == 0) as c_int;
                }
                if (*hdr).gmv[i as usize].r#type as c_uint
                    == RAV1D_WM_TYPE_AFFINE as c_int as c_uint
                {
                    *mat.offset(4) = 2 as c_int
                        * rav1d_get_bits_subexp(gb, *ref_mat.offset(4) >> 1, 12 as c_int as c_uint);
                    *mat.offset(5) = ((1 as c_int) << 16)
                        + 2 * rav1d_get_bits_subexp(
                            gb,
                            *ref_mat.offset(5) - ((1 as c_int) << 16) >> 1,
                            12 as c_int as c_uint,
                        );
                } else {
                    *mat.offset(4) = -*mat.offset(3);
                    *mat.offset(5) = *mat.offset(2);
                }
                *mat.offset(0) =
                    rav1d_get_bits_subexp(gb, *ref_mat.offset(0) >> shift, bits as c_uint)
                        * ((1 as c_int) << shift);
                *mat.offset(1) =
                    rav1d_get_bits_subexp(gb, *ref_mat.offset(1) >> shift, bits as c_uint)
                        * ((1 as c_int) << shift);
            }
            i += 1;
        }
    }
    (*hdr).film_grain.present = ((*seqhdr).film_grain_present != 0
        && ((*hdr).show_frame != 0 || (*hdr).showable_frame != 0)
        && rav1d_get_bit(gb) != 0) as c_int;
    if (*hdr).film_grain.present != 0 {
        let seed: c_uint = rav1d_get_bits(gb, 16 as c_int);
        (*hdr).film_grain.update = ((*hdr).frame_type as c_uint
            != RAV1D_FRAME_TYPE_INTER as c_int as c_uint
            || rav1d_get_bit(gb) != 0) as c_int;
        if (*hdr).film_grain.update == 0 {
            let refidx: c_int = rav1d_get_bits(gb, 3 as c_int) as c_int;
            let mut i: c_int;
            i = 0 as c_int;
            while i < 7 {
                if (*hdr).refidx[i as usize] == refidx {
                    break;
                }
                i += 1;
            }
            if i == 7 || ((*c).refs[refidx as usize].p.p.frame_hdr).is_null() {
                return error(c);
            }
            (*hdr).film_grain.data = (*(*c).refs[refidx as usize].p.p.frame_hdr)
                .film_grain
                .data
                .clone();
            (*hdr).film_grain.data.seed = seed;
        } else {
            let fgd = &mut (*hdr).film_grain.data;
            (*fgd).seed = seed;
            (*fgd).num_y_points = rav1d_get_bits(gb, 4 as c_int) as c_int;
            if (*fgd).num_y_points > 14 {
                return error(c);
            }
            let mut i = 0;
            while i < (*fgd).num_y_points {
                (*fgd).y_points[i as usize][0] = rav1d_get_bits(gb, 8 as c_int) as u8;
                if i != 0
                    && (*fgd).y_points[(i - 1) as usize][0] as c_int
                        >= (*fgd).y_points[i as usize][0] as c_int
                {
                    return error(c);
                }
                (*fgd).y_points[i as usize][1] = rav1d_get_bits(gb, 8 as c_int) as u8;
                i += 1;
            }
            (*fgd).chroma_scaling_from_luma = (*seqhdr).monochrome == 0 && rav1d_get_bit(gb) != 0;
            if (*seqhdr).monochrome != 0
                || (*fgd).chroma_scaling_from_luma
                || (*seqhdr).ss_ver == 1 && (*seqhdr).ss_hor == 1 && (*fgd).num_y_points == 0
            {
                (*fgd).num_uv_points[1] = 0 as c_int;
                (*fgd).num_uv_points[0] = (*fgd).num_uv_points[1];
            } else {
                let mut pl = 0;
                while pl < 2 {
                    (*fgd).num_uv_points[pl as usize] = rav1d_get_bits(gb, 4 as c_int) as c_int;
                    if (*fgd).num_uv_points[pl as usize] > 10 {
                        return error(c);
                    }
                    let mut i = 0;
                    while i < (*fgd).num_uv_points[pl as usize] {
                        (*fgd).uv_points[pl as usize][i as usize][0] =
                            rav1d_get_bits(gb, 8 as c_int) as u8;
                        if i != 0
                            && (*fgd).uv_points[pl as usize][(i - 1) as usize][0] as c_int
                                >= (*fgd).uv_points[pl as usize][i as usize][0] as c_int
                        {
                            return error(c);
                        }
                        (*fgd).uv_points[pl as usize][i as usize][1] =
                            rav1d_get_bits(gb, 8 as c_int) as u8;
                        i += 1;
                    }
                    pl += 1;
                }
            }
            if (*seqhdr).ss_hor == 1
                && (*seqhdr).ss_ver == 1
                && ((*fgd).num_uv_points[0] != 0) as c_int
                    != ((*fgd).num_uv_points[1] != 0) as c_int
            {
                return error(c);
            }
            (*fgd).scaling_shift =
                (rav1d_get_bits(gb, 2 as c_int)).wrapping_add(8 as c_int as c_uint) as u8;
            (*fgd).ar_coeff_lag = rav1d_get_bits(gb, 2 as c_int) as c_int;
            let num_y_pos = 2 * (*fgd).ar_coeff_lag * ((*fgd).ar_coeff_lag + 1);
            if (*fgd).num_y_points != 0 {
                let mut i = 0;
                while i < num_y_pos {
                    (*fgd).ar_coeffs_y[i as usize] =
                        (rav1d_get_bits(gb, 8 as c_int)).wrapping_sub(128 as c_int as c_uint) as i8;
                    i += 1;
                }
            }
            let mut pl = 0;
            while pl < 2 {
                if (*fgd).num_uv_points[pl as usize] != 0 || (*fgd).chroma_scaling_from_luma {
                    let num_uv_pos: c_int = num_y_pos + ((*fgd).num_y_points != 0) as c_int;
                    let mut i = 0;
                    while i < num_uv_pos {
                        (*fgd).ar_coeffs_uv[pl as usize][i as usize] =
                            (rav1d_get_bits(gb, 8 as c_int)).wrapping_sub(128 as c_int as c_uint)
                                as i8;
                        i += 1;
                    }
                    if (*fgd).num_y_points == 0 {
                        (*fgd).ar_coeffs_uv[pl as usize][num_uv_pos as usize] = 0 as c_int as i8;
                    }
                }
                pl += 1;
            }
            (*fgd).ar_coeff_shift =
                (rav1d_get_bits(gb, 2 as c_int)).wrapping_add(6 as c_int as c_uint) as u8;
            (*fgd).grain_scale_shift = rav1d_get_bits(gb, 2 as c_int) as u8;
            let mut pl = 0;
            while pl < 2 {
                if (*fgd).num_uv_points[pl as usize] != 0 {
                    (*fgd).uv_mult[pl as usize] = (rav1d_get_bits(gb, 8 as c_int))
                        .wrapping_sub(128 as c_int as c_uint)
                        as c_int;
                    (*fgd).uv_luma_mult[pl as usize] = (rav1d_get_bits(gb, 8 as c_int))
                        .wrapping_sub(128 as c_int as c_uint)
                        as c_int;
                    (*fgd).uv_offset[pl as usize] = (rav1d_get_bits(gb, 9 as c_int))
                        .wrapping_sub(256 as c_int as c_uint)
                        as c_int;
                }
                pl += 1;
            }
            (*fgd).overlap_flag = rav1d_get_bit(gb) != 0;
            (*fgd).clip_to_restricted_range = rav1d_get_bit(gb) != 0;
        }
    } else {
        memset(
            &mut (*hdr).film_grain.data as *mut Rav1dFilmGrainData as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<Rav1dFilmGrainData>(),
        );
    }

    (*(*(*c).frame_hdr_ref)
        .data
        .cast::<DRav1d<Rav1dFrameHeader, Dav1dFrameHeader>>())
    .update_dav1d();

    Ok(())
}

unsafe fn parse_tile_hdr(c: *mut Rav1dContext, gb: *mut GetBits) {
    let n_tiles = (*(*c).frame_hdr).tiling.cols * (*(*c).frame_hdr).tiling.rows;
    let have_tile_pos = (if n_tiles > 1 {
        rav1d_get_bit(gb)
    } else {
        0 as c_int as c_uint
    }) as c_int;
    if have_tile_pos != 0 {
        let n_bits = (*(*c).frame_hdr).tiling.log2_cols + (*(*c).frame_hdr).tiling.log2_rows;
        (*((*c).tile).offset((*c).n_tile_data as isize)).start =
            rav1d_get_bits(gb, n_bits) as c_int;
        (*((*c).tile).offset((*c).n_tile_data as isize)).end = rav1d_get_bits(gb, n_bits) as c_int;
    } else {
        (*((*c).tile).offset((*c).n_tile_data as isize)).start = 0 as c_int;
        (*((*c).tile).offset((*c).n_tile_data as isize)).end = n_tiles - 1;
    };
}

unsafe fn check_for_overrun(
    c: *mut Rav1dContext,
    gb: &mut GetBits,
    init_bit_pos: c_uint,
    obu_len: c_uint,
) -> c_int {
    if gb.error != 0 {
        rav1d_log(
            c,
            b"Overrun in OBU bit buffer\n\0" as *const u8 as *const c_char,
        );
        return 1 as c_int;
    }
    let pos: c_uint = rav1d_get_bits_pos(gb);
    if !(init_bit_pos <= pos) {
        unreachable!();
    }
    if pos.wrapping_sub(init_bit_pos) > (8 as c_int as c_uint).wrapping_mul(obu_len) {
        rav1d_log(
            c,
            b"Overrun in OBU bit buffer into next OBU\n\0" as *const u8 as *const c_char,
        );
        return 1 as c_int;
    }
    0
}

pub(crate) unsafe fn rav1d_parse_obus(
    c: &mut Rav1dContext,
    r#in: &mut Rav1dData,
    global: c_int,
) -> Rav1dResult<c_uint> {
    unsafe fn error(c: &mut Rav1dContext, r#in: &mut Rav1dData) -> Rav1dResult {
        rav1d_data_props_copy(&mut c.cached_error_props, &mut r#in.m);
        rav1d_log(
            c,
            b"Error parsing OBU data\n\0" as *const u8 as *const c_char,
        );
        return Err(EINVAL);
    }

    unsafe fn skip(c: &mut Rav1dContext, len: c_uint, init_byte_pos: c_uint) -> c_uint {
        let mut i = 0;
        while i < 8 {
            if (*c.frame_hdr).refresh_frame_flags & (1 as c_int) << i != 0 {
                rav1d_thread_picture_unref(&mut (*(c.refs).as_mut_ptr().offset(i as isize)).p);
                c.refs[i as usize].p.p.frame_hdr = c.frame_hdr;
                c.refs[i as usize].p.p.seq_hdr = c.seq_hdr;
                c.refs[i as usize].p.p.frame_hdr_ref = c.frame_hdr_ref;
                c.refs[i as usize].p.p.seq_hdr_ref = c.seq_hdr_ref;
                rav1d_ref_inc(c.frame_hdr_ref);
                rav1d_ref_inc(c.seq_hdr_ref);
            }
            i += 1;
        }
        rav1d_ref_dec(&mut c.frame_hdr_ref);
        c.frame_hdr = 0 as *mut Rav1dFrameHeader;
        c.n_tiles = 0 as c_int;
        len.wrapping_add(init_byte_pos)
    }

    let mut gb: GetBits = GetBits {
        state: 0,
        bits_left: 0,
        error: 0,
        ptr: 0 as *const u8,
        ptr_start: 0 as *const u8,
        ptr_end: 0 as *const u8,
    };
    rav1d_init_get_bits(&mut gb, r#in.data, r#in.sz);
    rav1d_get_bit(&mut gb);
    let r#type: Rav1dObuType = rav1d_get_bits(&mut gb, 4 as c_int) as Rav1dObuType;
    let has_extension: c_int = rav1d_get_bit(&mut gb) as c_int;
    let has_length_field: c_int = rav1d_get_bit(&mut gb) as c_int;
    rav1d_get_bit(&mut gb);
    let mut temporal_id = 0;
    let mut spatial_id = 0;
    if has_extension != 0 {
        temporal_id = rav1d_get_bits(&mut gb, 3 as c_int) as c_int;
        spatial_id = rav1d_get_bits(&mut gb, 2 as c_int) as c_int;
        rav1d_get_bits(&mut gb, 3 as c_int);
    }
    let len: c_uint = if has_length_field != 0 {
        rav1d_get_uleb128(&mut gb)
    } else {
        (r#in.sz as c_uint)
            .wrapping_sub(1 as c_int as c_uint)
            .wrapping_sub(has_extension as c_uint)
    };
    if gb.error != 0 {
        error(c, r#in)?;
    }
    let init_bit_pos: c_uint = rav1d_get_bits_pos(&mut gb);
    let init_byte_pos: c_uint = init_bit_pos >> 3;
    if !(init_bit_pos & 7 as c_uint == 0 as c_uint) {
        unreachable!();
    }
    if !(r#in.sz >= init_byte_pos as usize) {
        unreachable!();
    }
    if len as usize > (r#in.sz).wrapping_sub(init_byte_pos as usize) {
        error(c, r#in)?;
    }
    if r#type as c_uint != RAV1D_OBU_SEQ_HDR as c_int as c_uint
        && r#type as c_uint != RAV1D_OBU_TD as c_int as c_uint
        && has_extension != 0
        && c.operating_point_idc != 0 as c_int as c_uint
    {
        let in_temporal_layer: c_int =
            (c.operating_point_idc >> temporal_id & 1 as c_uint) as c_int;
        let in_spatial_layer: c_int =
            (c.operating_point_idc >> spatial_id + 8 & 1 as c_uint) as c_int;
        if in_temporal_layer == 0 || in_spatial_layer == 0 {
            return Ok(len.wrapping_add(init_byte_pos));
        }
    }
    let mut current_block_188: u64;
    match r#type as c_uint {
        RAV1D_OBU_SEQ_HDR => {
            let mut r#ref: *mut Rav1dRef = rav1d_ref_create_using_pool(
                c.seq_hdr_pool,
                ::core::mem::size_of::<DRav1d<Rav1dSequenceHeader, Dav1dSequenceHeader>>(),
            );
            if r#ref.is_null() {
                return Err(ENOMEM);
            }
            let seq_hdrs = (*r#ref)
                .data
                .cast::<DRav1d<Rav1dSequenceHeader, Dav1dSequenceHeader>>();
            let seq_hdr: *mut Rav1dSequenceHeader = addr_of_mut!((*seq_hdrs).rav1d);
            let res = parse_seq_hdr(c, &mut gb, seq_hdr);
            (*seq_hdrs).update_dav1d();
            if res.is_err() {
                rav1d_ref_dec(&mut r#ref);
                error(c, r#in)?;
            }
            if check_for_overrun(c, &mut gb, init_bit_pos, len) != 0 {
                rav1d_ref_dec(&mut r#ref);
                error(c, r#in)?;
            }
            if (c.seq_hdr).is_null() {
                c.frame_hdr = 0 as *mut Rav1dFrameHeader;
                c.frame_flags = ::core::mem::transmute::<c_uint, PictureFlags>(
                    c.frame_flags as c_uint | PICTURE_FLAG_NEW_SEQUENCE as c_int as c_uint,
                );
            } else if memcmp(seq_hdr as *const c_void, c.seq_hdr as *const c_void, 1100) != 0 {
                c.frame_hdr = 0 as *mut Rav1dFrameHeader;
                c.mastering_display = 0 as *mut Rav1dMasteringDisplay;
                c.content_light = 0 as *mut Rav1dContentLightLevel;
                rav1d_ref_dec(&mut c.mastering_display_ref);
                rav1d_ref_dec(&mut c.content_light_ref);
                let mut i = 0;
                while i < 8 {
                    if !(c.refs[i as usize].p.p.frame_hdr).is_null() {
                        rav1d_thread_picture_unref(
                            &mut (*(c.refs).as_mut_ptr().offset(i as isize)).p,
                        );
                    }
                    rav1d_ref_dec(&mut (*(c.refs).as_mut_ptr().offset(i as isize)).segmap);
                    rav1d_ref_dec(&mut (*(c.refs).as_mut_ptr().offset(i as isize)).refmvs);
                    rav1d_cdf_thread_unref(&mut *(c.cdf).as_mut_ptr().offset(i as isize));
                    i += 1;
                }
                c.frame_flags = ::core::mem::transmute::<c_uint, PictureFlags>(
                    c.frame_flags as c_uint | PICTURE_FLAG_NEW_SEQUENCE as c_int as c_uint,
                );
            } else if memcmp(
                ((*seq_hdr).operating_parameter_info).as_mut_ptr() as *const c_void,
                ((*c.seq_hdr).operating_parameter_info).as_mut_ptr() as *const c_void,
                ::core::mem::size_of::<[Dav1dSequenceHeaderOperatingParameterInfo; 32]>(),
            ) != 0
            {
                c.frame_flags = ::core::mem::transmute::<c_uint, PictureFlags>(
                    c.frame_flags as c_uint | PICTURE_FLAG_NEW_OP_PARAMS_INFO as c_int as c_uint,
                );
            }
            rav1d_ref_dec(&mut c.seq_hdr_ref);
            c.seq_hdr_ref = r#ref;
            c.seq_hdr = seq_hdr;
            current_block_188 = 8953117030348968745;
        }
        RAV1D_OBU_REDUNDANT_FRAME_HDR => {
            if !(c.frame_hdr).is_null() {
                current_block_188 = 8953117030348968745;
            } else {
                current_block_188 = 14065157188459580465;
            }
        }
        RAV1D_OBU_FRAME | RAV1D_OBU_FRAME_HDR => {
            current_block_188 = 14065157188459580465;
        }
        RAV1D_OBU_TILE_GRP => {
            current_block_188 = 17787701279558130514;
        }
        RAV1D_OBU_METADATA => {
            let meta_type: ObuMetaType = rav1d_get_uleb128(&mut gb) as ObuMetaType;
            let meta_type_len: c_int =
                ((rav1d_get_bits_pos(&mut gb)).wrapping_sub(init_bit_pos) >> 3) as c_int;
            if gb.error != 0 {
                error(c, r#in)?;
            }
            match meta_type as c_uint {
                OBU_META_HDR_CLL => {
                    let mut r#ref: *mut Rav1dRef =
                        rav1d_ref_create(::core::mem::size_of::<Rav1dContentLightLevel>());
                    if r#ref.is_null() {
                        return Err(ENOMEM);
                    }
                    let content_light: *mut Rav1dContentLightLevel =
                        (*r#ref).data as *mut Rav1dContentLightLevel;
                    (*content_light).max_content_light_level =
                        rav1d_get_bits(&mut gb, 16 as c_int) as c_int;
                    (*content_light).max_frame_average_light_level =
                        rav1d_get_bits(&mut gb, 16 as c_int) as c_int;
                    rav1d_get_bit(&mut gb);
                    rav1d_bytealign_get_bits(&mut gb);
                    if check_for_overrun(c, &mut gb, init_bit_pos, len) != 0 {
                        rav1d_ref_dec(&mut r#ref);
                        error(c, r#in)?;
                    }
                    rav1d_ref_dec(&mut c.content_light_ref);
                    c.content_light = content_light;
                    c.content_light_ref = r#ref;
                }
                OBU_META_HDR_MDCV => {
                    let mut r#ref: *mut Rav1dRef =
                        rav1d_ref_create(::core::mem::size_of::<Rav1dMasteringDisplay>());
                    if r#ref.is_null() {
                        return Err(ENOMEM);
                    }
                    let mastering_display: *mut Rav1dMasteringDisplay =
                        (*r#ref).data as *mut Rav1dMasteringDisplay;
                    let mut i = 0;
                    while i < 3 {
                        (*mastering_display).primaries[i as usize][0] =
                            rav1d_get_bits(&mut gb, 16 as c_int) as u16;
                        (*mastering_display).primaries[i as usize][1] =
                            rav1d_get_bits(&mut gb, 16 as c_int) as u16;
                        i += 1;
                    }
                    (*mastering_display).white_point[0] =
                        rav1d_get_bits(&mut gb, 16 as c_int) as u16;
                    (*mastering_display).white_point[1] =
                        rav1d_get_bits(&mut gb, 16 as c_int) as u16;
                    (*mastering_display).max_luminance = rav1d_get_bits(&mut gb, 32 as c_int);
                    (*mastering_display).min_luminance = rav1d_get_bits(&mut gb, 32 as c_int);
                    rav1d_get_bit(&mut gb);
                    rav1d_bytealign_get_bits(&mut gb);
                    if check_for_overrun(c, &mut gb, init_bit_pos, len) != 0 {
                        rav1d_ref_dec(&mut r#ref);
                        error(c, r#in)?;
                    }
                    rav1d_ref_dec(&mut c.mastering_display_ref);
                    c.mastering_display = mastering_display;
                    c.mastering_display_ref = r#ref;
                }
                OBU_META_ITUT_T35 => {
                    let mut payload_size: c_int = len as c_int;
                    while payload_size > 0
                        && *(r#in.data).offset(
                            init_byte_pos
                                .wrapping_add(payload_size as c_uint)
                                .wrapping_sub(1 as c_int as c_uint)
                                as isize,
                        ) == 0
                    {
                        payload_size -= 1;
                    }
                    payload_size -= 1;
                    payload_size -= meta_type_len;
                    let mut country_code_extension_byte = 0;
                    let country_code: c_int = rav1d_get_bits(&mut gb, 8 as c_int) as c_int;
                    payload_size -= 1;
                    if country_code == 0xff as c_int {
                        country_code_extension_byte = rav1d_get_bits(&mut gb, 8 as c_int) as c_int;
                        payload_size -= 1;
                    }
                    if payload_size <= 0 {
                        rav1d_log(
                            c,
                            b"Malformed ITU-T T.35 metadata message format\n\0" as *const u8
                                as *const c_char,
                        );
                    } else {
                        let r#ref: *mut Rav1dRef = rav1d_ref_create(
                            (::core::mem::size_of::<DRav1d<Rav1dITUTT35, Dav1dITUTT35>>())
                                .wrapping_add(
                                    (payload_size as usize)
                                        .wrapping_mul(::core::mem::size_of::<u8>()),
                                ),
                        );
                        if r#ref.is_null() {
                            return Err(ENOMEM);
                        }
                        let itut_t32_metadatas =
                            (*r#ref).data.cast::<DRav1d<Rav1dITUTT35, Dav1dITUTT35>>();
                        let itut_t35_metadata: *mut Rav1dITUTT35 =
                            addr_of_mut!((*itut_t32_metadatas).rav1d);
                        (*itut_t35_metadata).payload = (*r#ref)
                            .data
                            .cast::<u8>()
                            .offset(::core::mem::size_of::<DRav1d<Rav1dITUTT35, Dav1dITUTT35>>()
                                as isize);
                        (*itut_t35_metadata).country_code = country_code as u8;
                        (*itut_t35_metadata).country_code_extension_byte =
                            country_code_extension_byte as u8;
                        let mut i = 0;
                        while i < payload_size {
                            *((*itut_t35_metadata).payload).offset(i as isize) =
                                rav1d_get_bits(&mut gb, 8 as c_int) as u8;
                            i += 1;
                        }
                        (*itut_t35_metadata).payload_size = payload_size as usize;
                        (*itut_t32_metadatas).update_dav1d();
                        rav1d_ref_dec(&mut c.itut_t35_ref);
                        c.itut_t35 = itut_t35_metadata;
                        c.itut_t35_ref = r#ref;
                    }
                }
                OBU_META_SCALABILITY | OBU_META_TIMECODE => {}
                _ => {
                    rav1d_log(
                        c,
                        b"Unknown Metadata OBU type %d\n\0" as *const u8 as *const c_char,
                        meta_type as c_uint,
                    );
                }
            }
            current_block_188 = 8953117030348968745;
        }
        RAV1D_OBU_TD => {
            c.frame_flags = ::core::mem::transmute::<c_uint, PictureFlags>(
                c.frame_flags as c_uint | PICTURE_FLAG_NEW_TEMPORAL_UNIT as c_int as c_uint,
            );
            current_block_188 = 8953117030348968745;
        }
        RAV1D_OBU_PADDING => {
            current_block_188 = 8953117030348968745;
        }
        _ => {
            rav1d_log(
                c,
                b"Unknown OBU type %d of size %u\n\0" as *const u8 as *const c_char,
                r#type as c_uint,
                len,
            );
            current_block_188 = 8953117030348968745;
        }
    }
    match current_block_188 {
        14065157188459580465 => {
            if global != 0 {
                current_block_188 = 8953117030348968745;
            } else {
                if (c.seq_hdr).is_null() {
                    error(c, r#in)?;
                }
                if (c.frame_hdr_ref).is_null() {
                    c.frame_hdr_ref = rav1d_ref_create_using_pool(
                        c.frame_hdr_pool,
                        ::core::mem::size_of::<DRav1d<Rav1dFrameHeader, Dav1dFrameHeader>>(),
                    );
                    if (c.frame_hdr_ref).is_null() {
                        return Err(ENOMEM);
                    }
                }
                // ensure that the reference is writable
                debug_assert!(rav1d_ref_is_writable(c.frame_hdr_ref) != 0);
                let frame_hdrs =
                    (*c.frame_hdr_ref).data as *mut DRav1d<Rav1dFrameHeader, Dav1dFrameHeader>;
                memset(
                    frame_hdrs as *mut c_void,
                    0 as c_int,
                    ::core::mem::size_of::<DRav1d<Rav1dFrameHeader, Dav1dFrameHeader>>(),
                );
                c.frame_hdr = &mut (*frame_hdrs).rav1d;
                (*c.frame_hdr).temporal_id = temporal_id;
                (*c.frame_hdr).spatial_id = spatial_id;
                let res = parse_frame_hdr(c, &mut gb);
                if res.is_err() {
                    c.frame_hdr = 0 as *mut Rav1dFrameHeader;
                    error(c, r#in)?;
                }
                let mut n = 0;
                while n < c.n_tile_data {
                    rav1d_data_unref_internal(&mut (*(c.tile).offset(n as isize)).data);
                    n += 1;
                }
                c.n_tile_data = 0 as c_int;
                c.n_tiles = 0 as c_int;
                if r#type as c_uint != RAV1D_OBU_FRAME as c_int as c_uint {
                    rav1d_get_bit(&mut gb);
                    if check_for_overrun(c, &mut gb, init_bit_pos, len) != 0 {
                        c.frame_hdr = 0 as *mut Rav1dFrameHeader;
                        error(c, r#in)?;
                    }
                }
                if c.frame_size_limit != 0
                    && (*c.frame_hdr).width[1] as i64 * (*c.frame_hdr).height as i64
                        > c.frame_size_limit as i64
                {
                    rav1d_log(
                        c,
                        b"Frame size %dx%d exceeds limit %u\n\0" as *const u8 as *const c_char,
                        (*c.frame_hdr).width[1],
                        (*c.frame_hdr).height,
                        c.frame_size_limit,
                    );
                    c.frame_hdr = 0 as *mut Rav1dFrameHeader;
                    return Err(ERANGE);
                }
                if r#type as c_uint != RAV1D_OBU_FRAME as c_int as c_uint {
                    current_block_188 = 8953117030348968745;
                } else {
                    if (*c.frame_hdr).show_existing_frame != 0 {
                        c.frame_hdr = 0 as *mut Rav1dFrameHeader;
                        error(c, r#in)?;
                    }
                    rav1d_bytealign_get_bits(&mut gb);
                    current_block_188 = 17787701279558130514;
                }
            }
        }
        _ => {}
    }
    match current_block_188 {
        17787701279558130514 => {
            if !(global != 0) {
                if (c.frame_hdr).is_null() {
                    error(c, r#in)?;
                }
                if c.n_tile_data_alloc < c.n_tile_data + 1 {
                    if c.n_tile_data + 1
                        > i32::MAX / ::core::mem::size_of::<Rav1dTileGroup>() as c_ulong as c_int
                    {
                        error(c, r#in)?;
                    }
                    let tile: *mut Rav1dTileGroup = realloc(
                        c.tile as *mut c_void,
                        ((c.n_tile_data + 1) as usize)
                            .wrapping_mul(::core::mem::size_of::<Rav1dTileGroup>()),
                    ) as *mut Rav1dTileGroup;
                    if tile.is_null() {
                        error(c, r#in)?;
                    }
                    c.tile = tile;
                    memset(
                        (c.tile).offset(c.n_tile_data as isize) as *mut c_void,
                        0 as c_int,
                        ::core::mem::size_of::<Rav1dTileGroup>(),
                    );
                    c.n_tile_data_alloc = c.n_tile_data + 1;
                }
                parse_tile_hdr(c, &mut gb);
                rav1d_bytealign_get_bits(&mut gb);
                if check_for_overrun(c, &mut gb, init_bit_pos, len) != 0 {
                    error(c, r#in)?;
                }
                let pkt_bytelen: c_uint = init_byte_pos.wrapping_add(len);
                let bit_pos: c_uint = rav1d_get_bits_pos(&mut gb);
                if !(bit_pos & 7 as c_uint == 0 as c_uint) {
                    unreachable!();
                }
                if !(pkt_bytelen >= bit_pos >> 3) {
                    unreachable!();
                }
                rav1d_data_ref(&mut (*(c.tile).offset(c.n_tile_data as isize)).data, r#in);
                let ref mut fresh0 = (*(c.tile).offset(c.n_tile_data as isize)).data.data;
                *fresh0 = (*fresh0).offset((bit_pos >> 3) as isize);
                (*(c.tile).offset(c.n_tile_data as isize)).data.sz =
                    pkt_bytelen.wrapping_sub(bit_pos >> 3) as usize;
                if (*(c.tile).offset(c.n_tile_data as isize)).start
                    > (*(c.tile).offset(c.n_tile_data as isize)).end
                    || (*(c.tile).offset(c.n_tile_data as isize)).start != c.n_tiles
                {
                    let mut i = 0;
                    while i <= c.n_tile_data {
                        rav1d_data_unref_internal(&mut (*(c.tile).offset(i as isize)).data);
                        i += 1;
                    }
                    c.n_tile_data = 0 as c_int;
                    c.n_tiles = 0 as c_int;
                    error(c, r#in)?;
                }
                c.n_tiles += 1 as c_int + (*(c.tile).offset(c.n_tile_data as isize)).end
                    - (*(c.tile).offset(c.n_tile_data as isize)).start;
                c.n_tile_data += 1;
            }
        }
        _ => {}
    }
    if !(c.seq_hdr).is_null() && !(c.frame_hdr).is_null() {
        if (*c.frame_hdr).show_existing_frame != 0 {
            if (c.refs[(*c.frame_hdr).existing_frame_idx as usize]
                .p
                .p
                .frame_hdr)
                .is_null()
            {
                error(c, r#in)?;
            }
            match (*c.refs[(*c.frame_hdr).existing_frame_idx as usize]
                .p
                .p
                .frame_hdr)
                .frame_type as c_uint
            {
                RAV1D_FRAME_TYPE_INTER | RAV1D_FRAME_TYPE_SWITCH => {
                    if c.decode_frame_type as c_uint
                        > RAV1D_DECODEFRAMETYPE_REFERENCE as c_int as c_uint
                    {
                        return Ok(skip(c, len, init_byte_pos));
                    }
                }
                RAV1D_FRAME_TYPE_INTRA => {
                    if c.decode_frame_type as c_uint
                        > RAV1D_DECODEFRAMETYPE_INTRA as c_int as c_uint
                    {
                        return Ok(skip(c, len, init_byte_pos));
                    }
                }
                _ => {}
            }
            if (c.refs[(*c.frame_hdr).existing_frame_idx as usize].p.p.data[0]).is_null() {
                error(c, r#in)?;
            }
            if c.strict_std_compliance
                && !c.refs[(*c.frame_hdr).existing_frame_idx as usize]
                    .p
                    .showable
            {
                error(c, r#in)?;
            }
            if c.n_fc == 1 as c_uint {
                rav1d_thread_picture_ref(
                    &mut c.out,
                    &mut (*(c.refs)
                        .as_mut_ptr()
                        .offset((*c.frame_hdr).existing_frame_idx as isize))
                    .p,
                );
                rav1d_data_props_copy(&mut c.out.p.m, &mut r#in.m);
                c.event_flags = ::core::mem::transmute::<c_uint, Dav1dEventFlags>(
                    c.event_flags as c_uint
                        | rav1d_picture_get_event_flags(
                            &mut (*(c.refs)
                                .as_mut_ptr()
                                .offset((*c.frame_hdr).existing_frame_idx as isize))
                            .p,
                        ) as c_uint,
                );
            } else {
                pthread_mutex_lock(&mut c.task_thread.lock);
                let fresh1 = c.frame_thread.next;
                c.frame_thread.next = (c.frame_thread.next).wrapping_add(1);
                let next: c_uint = fresh1;
                if c.frame_thread.next == c.n_fc {
                    c.frame_thread.next = 0 as c_int as c_uint;
                }
                let f: *mut Rav1dFrameContext =
                    &mut *(c.fc).offset(next as isize) as *mut Rav1dFrameContext;
                while (*f).n_tile_data > 0 {
                    pthread_cond_wait(
                        &mut (*f).task_thread.cond,
                        &mut (*(*f).task_thread.ttd).lock,
                    );
                }
                let out_delayed: *mut Rav1dThreadPicture = &mut *(c.frame_thread.out_delayed)
                    .offset(next as isize)
                    as *mut Rav1dThreadPicture;
                if !((*out_delayed).p.data[0]).is_null()
                    || ::core::intrinsics::atomic_load_seqcst(
                        &mut (*f).task_thread.error as *mut atomic_int,
                    ) != 0
                {
                    let mut first: c_uint =
                        ::core::intrinsics::atomic_load_seqcst(&mut c.task_thread.first);
                    if first.wrapping_add(1 as c_uint) < c.n_fc {
                        ::core::intrinsics::atomic_xadd_seqcst(
                            &mut c.task_thread.first,
                            1 as c_uint,
                        );
                    } else {
                        ::core::intrinsics::atomic_store_seqcst(
                            &mut c.task_thread.first,
                            0 as c_int as c_uint,
                        );
                    }
                    let fresh2 = ::core::intrinsics::atomic_cxchg_seqcst_seqcst(
                        &mut c.task_thread.reset_task_cur,
                        *&mut first,
                        u32::MAX,
                    );
                    *&mut first = fresh2.0;
                    fresh2.1;
                    if c.task_thread.cur != 0 && c.task_thread.cur < c.n_fc {
                        c.task_thread.cur = (c.task_thread.cur).wrapping_sub(1);
                    }
                }
                let error = (*f).task_thread.retval;
                if error.is_err() {
                    c.cached_error = error;
                    (*f).task_thread.retval = Ok(());
                    rav1d_data_props_copy(&mut c.cached_error_props, &mut (*out_delayed).p.m);
                    rav1d_thread_picture_unref(out_delayed);
                } else if !((*out_delayed).p.data[0]).is_null() {
                    let progress: c_uint = ::core::intrinsics::atomic_load_relaxed(
                        &mut *((*out_delayed).progress).offset(1) as *mut atomic_uint,
                    );
                    if ((*out_delayed).visible || c.output_invisible_frames)
                        && progress != FRAME_ERROR
                    {
                        rav1d_thread_picture_ref(&mut c.out, out_delayed);
                        c.event_flags = ::core::mem::transmute::<c_uint, Dav1dEventFlags>(
                            c.event_flags as c_uint
                                | rav1d_picture_get_event_flags(out_delayed) as c_uint,
                        );
                    }
                    rav1d_thread_picture_unref(out_delayed);
                }
                rav1d_thread_picture_ref(
                    out_delayed,
                    &mut (*(c.refs)
                        .as_mut_ptr()
                        .offset((*c.frame_hdr).existing_frame_idx as isize))
                    .p,
                );
                (*out_delayed).visible = true;
                rav1d_data_props_copy(&mut (*out_delayed).p.m, &mut r#in.m);
                pthread_mutex_unlock(&mut c.task_thread.lock);
            }
            if (*c.refs[(*c.frame_hdr).existing_frame_idx as usize]
                .p
                .p
                .frame_hdr)
                .frame_type as c_uint
                == RAV1D_FRAME_TYPE_KEY as c_int as c_uint
            {
                let r: c_int = (*c.frame_hdr).existing_frame_idx;
                c.refs[r as usize].p.showable = false;
                let mut i = 0;
                while i < 8 {
                    if !(i == r) {
                        if !(c.refs[i as usize].p.p.frame_hdr).is_null() {
                            rav1d_thread_picture_unref(
                                &mut (*(c.refs).as_mut_ptr().offset(i as isize)).p,
                            );
                        }
                        rav1d_thread_picture_ref(
                            &mut (*(c.refs).as_mut_ptr().offset(i as isize)).p,
                            &mut (*(c.refs).as_mut_ptr().offset(r as isize)).p,
                        );
                        rav1d_cdf_thread_unref(&mut *(c.cdf).as_mut_ptr().offset(i as isize));
                        rav1d_cdf_thread_ref(
                            &mut *(c.cdf).as_mut_ptr().offset(i as isize),
                            &mut *(c.cdf).as_mut_ptr().offset(r as isize),
                        );
                        rav1d_ref_dec(&mut (*(c.refs).as_mut_ptr().offset(i as isize)).segmap);
                        c.refs[i as usize].segmap = c.refs[r as usize].segmap;
                        if !(c.refs[r as usize].segmap).is_null() {
                            rav1d_ref_inc(c.refs[r as usize].segmap);
                        }
                        rav1d_ref_dec(&mut (*(c.refs).as_mut_ptr().offset(i as isize)).refmvs);
                    }
                    i += 1;
                }
            }
            c.frame_hdr = 0 as *mut Rav1dFrameHeader;
        } else if c.n_tiles == (*c.frame_hdr).tiling.cols * (*c.frame_hdr).tiling.rows {
            match (*c.frame_hdr).frame_type as c_uint {
                RAV1D_FRAME_TYPE_INTER | RAV1D_FRAME_TYPE_SWITCH => {
                    if c.decode_frame_type as c_uint
                        > RAV1D_DECODEFRAMETYPE_REFERENCE as c_int as c_uint
                        || c.decode_frame_type as c_uint
                            == RAV1D_DECODEFRAMETYPE_REFERENCE as c_int as c_uint
                            && (*c.frame_hdr).refresh_frame_flags == 0
                    {
                        return Ok(skip(c, len, init_byte_pos));
                    }
                }
                RAV1D_FRAME_TYPE_INTRA => {
                    if c.decode_frame_type as c_uint
                        > RAV1D_DECODEFRAMETYPE_INTRA as c_int as c_uint
                        || c.decode_frame_type as c_uint
                            == RAV1D_DECODEFRAMETYPE_REFERENCE as c_int as c_uint
                            && (*c.frame_hdr).refresh_frame_flags == 0
                    {
                        return Ok(skip(c, len, init_byte_pos));
                    }
                }
                _ => {}
            }
            if c.n_tile_data == 0 {
                error(c, r#in)?;
            }
            rav1d_submit_frame(&mut *c)?;
            if c.n_tile_data != 0 {
                unreachable!();
            }
            c.frame_hdr = 0 as *mut Rav1dFrameHeader;
            c.n_tiles = 0 as c_int;
        }
    }
    Ok(len.wrapping_add(init_byte_pos))
}
