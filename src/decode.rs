use crate::include::common::attributes::ctz;
use crate::include::common::bitdepth::DynCoef;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::intops::apply_sign64;
use crate::include::common::intops::iclip;
use crate::include::common::intops::iclip_u8;
use crate::include::common::intops::ulog2;
use crate::include::dav1d::headers::Dav1dFilterMode;
use crate::include::dav1d::headers::Dav1dTxfmMode;
use crate::include::dav1d::headers::Rav1dFrameHeader;
use crate::include::dav1d::headers::Rav1dFrameHeader_tiling;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::include::dav1d::headers::Rav1dRestorationType;
use crate::include::dav1d::headers::Rav1dSequenceHeader;
use crate::include::dav1d::headers::Rav1dWarpedMotionParams;
use crate::include::dav1d::headers::RAV1D_FILTER_8TAP_REGULAR;
use crate::include::dav1d::headers::RAV1D_FILTER_SWITCHABLE;
use crate::include::dav1d::headers::RAV1D_MAX_SEGMENTS;
use crate::include::dav1d::headers::RAV1D_N_SWITCHABLE_FILTERS;
use crate::include::dav1d::headers::RAV1D_PRIMARY_REF_NONE;
use crate::include::dav1d::headers::RAV1D_RESTORATION_NONE;
use crate::include::dav1d::headers::RAV1D_RESTORATION_SGRPROJ;
use crate::include::dav1d::headers::RAV1D_RESTORATION_SWITCHABLE;
use crate::include::dav1d::headers::RAV1D_RESTORATION_WIENER;
use crate::include::dav1d::headers::RAV1D_TX_SWITCHABLE;
use crate::include::dav1d::headers::RAV1D_WM_TYPE_AFFINE;
use crate::include::dav1d::headers::RAV1D_WM_TYPE_IDENTITY;
use crate::include::dav1d::headers::RAV1D_WM_TYPE_TRANSLATION;
use crate::include::stdatomic::atomic_int;
use crate::src::align::Align16;
use crate::src::cdf::rav1d_cdf_thread_alloc;
use crate::src::cdf::rav1d_cdf_thread_copy;
use crate::src::cdf::rav1d_cdf_thread_init_static;
use crate::src::cdf::rav1d_cdf_thread_ref;
use crate::src::cdf::rav1d_cdf_thread_unref;
use crate::src::cdf::rav1d_cdf_thread_update;
use crate::src::cdf::CdfMvComponent;
use crate::src::cdf::CdfMvContext;
use crate::src::ctx::CaseSet;
use crate::src::data::rav1d_data_unref_internal;
use crate::src::dequant_tables::dav1d_dq_tbl;
use crate::src::enum_map::enum_map;
use crate::src::enum_map::enum_map_ty;
use crate::src::enum_map::DefaultValue;
use crate::src::env::av1_get_bwd_ref_1_ctx;
use crate::src::env::av1_get_bwd_ref_ctx;
use crate::src::env::av1_get_fwd_ref_1_ctx;
use crate::src::env::av1_get_fwd_ref_2_ctx;
use crate::src::env::av1_get_fwd_ref_ctx;
use crate::src::env::av1_get_ref_ctx;
use crate::src::env::av1_get_uni_p1_ctx;
use crate::src::env::fix_mv_precision;
use crate::src::env::gather_left_partition_prob;
use crate::src::env::gather_top_partition_prob;
use crate::src::env::get_comp_ctx;
use crate::src::env::get_comp_dir_ctx;
use crate::src::env::get_cur_frame_segid;
use crate::src::env::get_drl_context;
use crate::src::env::get_filter_ctx;
use crate::src::env::get_gmv_2d;
use crate::src::env::get_intra_ctx;
use crate::src::env::get_jnt_comp_ctx;
use crate::src::env::get_mask_comp_ctx;
use crate::src::env::get_partition_ctx;
use crate::src::env::get_poc_diff;
use crate::src::env::get_tx_ctx;
use crate::src::env::BlockContext;
use crate::src::error::Rav1dError::EINVAL;
use crate::src::error::Rav1dError::ENOMEM;
use crate::src::error::Rav1dError::ENOPROTOOPT;
use crate::src::error::Rav1dResult;
use crate::src::filmgrain::Rav1dFilmGrainDSPContext;
use crate::src::internal::CodedBlockInfo;
use crate::src::internal::Rav1dContext;
use crate::src::internal::Rav1dFrameContext;
use crate::src::internal::Rav1dFrameContext_bd_fn;
use crate::src::internal::Rav1dTaskContext;
use crate::src::internal::Rav1dTaskContext_scratch_pal;
use crate::src::internal::Rav1dTileState;
use crate::src::internal::ScalableMotionParams;
use crate::src::intra_edge::EdgeBranch;
use crate::src::intra_edge::EdgeFlags;
use crate::src::intra_edge::EdgeNode;
use crate::src::intra_edge::EdgeTip;
use crate::src::intra_edge::EDGE_I444_TOP_HAS_RIGHT;
use crate::src::ipred::rav1d_intra_pred_dsp_init;
use crate::src::levels::mv;
use crate::src::levels::Av1Block;
use crate::src::levels::BS_128x128;
use crate::src::levels::BS_4x4;
use crate::src::levels::BS_64x64;
use crate::src::levels::BlockLevel;
use crate::src::levels::BlockPartition;
use crate::src::levels::BlockSize;
use crate::src::levels::MotionMode;
use crate::src::levels::RectTxfmSize;
use crate::src::levels::TxfmSize;
use crate::src::levels::BL_128X128;
use crate::src::levels::BL_64X64;
use crate::src::levels::BL_8X8;
use crate::src::levels::CFL_PRED;
use crate::src::levels::COMP_INTER_AVG;
use crate::src::levels::COMP_INTER_NONE;
use crate::src::levels::COMP_INTER_SEG;
use crate::src::levels::COMP_INTER_WEDGE;
use crate::src::levels::COMP_INTER_WEIGHTED_AVG;
use crate::src::levels::DC_PRED;
use crate::src::levels::FILTER_2D_BILINEAR;
use crate::src::levels::FILTER_PRED;
use crate::src::levels::GLOBALMV;
use crate::src::levels::GLOBALMV_GLOBALMV;
use crate::src::levels::INTER_INTRA_BLEND;
use crate::src::levels::INTER_INTRA_NONE;
use crate::src::levels::INTER_INTRA_WEDGE;
use crate::src::levels::MM_OBMC;
use crate::src::levels::MM_TRANSLATION;
use crate::src::levels::MM_WARP;
use crate::src::levels::MV_JOINT_H;
use crate::src::levels::MV_JOINT_HV;
use crate::src::levels::MV_JOINT_V;
use crate::src::levels::NEARER_DRL;
use crate::src::levels::NEARESTMV;
use crate::src::levels::NEARESTMV_NEARESTMV;
use crate::src::levels::NEAREST_DRL;
use crate::src::levels::NEARISH_DRL;
use crate::src::levels::NEARMV;
use crate::src::levels::NEAR_DRL;
use crate::src::levels::NEWMV;
use crate::src::levels::NEWMV_NEWMV;
use crate::src::levels::N_COMP_INTER_PRED_MODES;
use crate::src::levels::N_INTER_INTRA_PRED_MODES;
use crate::src::levels::N_INTRA_PRED_MODES;
use crate::src::levels::N_MV_JOINTS;
use crate::src::levels::N_RECT_TX_SIZES;
use crate::src::levels::N_UV_INTRA_PRED_MODES;
use crate::src::levels::PARTITION_H;
use crate::src::levels::PARTITION_H4;
use crate::src::levels::PARTITION_NONE;
use crate::src::levels::PARTITION_SPLIT;
use crate::src::levels::PARTITION_T_BOTTOM_SPLIT;
use crate::src::levels::PARTITION_T_LEFT_SPLIT;
use crate::src::levels::PARTITION_T_RIGHT_SPLIT;
use crate::src::levels::PARTITION_T_TOP_SPLIT;
use crate::src::levels::PARTITION_V;
use crate::src::levels::PARTITION_V4;
use crate::src::levels::TX_4X4;
use crate::src::levels::TX_64X64;
use crate::src::levels::TX_8X8;
use crate::src::levels::VERT_LEFT_PRED;
use crate::src::levels::VERT_PRED;
use crate::src::lf_mask::rav1d_calc_eih;
use crate::src::lf_mask::rav1d_calc_lf_values;
use crate::src::lf_mask::rav1d_create_lf_mask_inter;
use crate::src::lf_mask::rav1d_create_lf_mask_intra;
use crate::src::lf_mask::Av1Filter;
use crate::src::lf_mask::Av1Restoration;
use crate::src::lf_mask::Av1RestorationUnit;
use crate::src::log::Rav1dLog as _;
use crate::src::loopfilter::rav1d_loop_filter_dsp_init;
use crate::src::looprestoration::rav1d_loop_restoration_dsp_init;
use crate::src::mc::rav1d_mc_dsp_init;
use crate::src::mem::freep;
use crate::src::mem::rav1d_alloc_aligned;
use crate::src::mem::rav1d_free_aligned;
use crate::src::mem::rav1d_freep_aligned;
use crate::src::msac::rav1d_msac_decode_bool;
use crate::src::msac::rav1d_msac_decode_bool_adapt;
use crate::src::msac::rav1d_msac_decode_bool_equi;
use crate::src::msac::rav1d_msac_decode_bools;
use crate::src::msac::rav1d_msac_decode_subexp;
use crate::src::msac::rav1d_msac_decode_symbol_adapt16;
use crate::src::msac::rav1d_msac_decode_symbol_adapt4;
use crate::src::msac::rav1d_msac_decode_symbol_adapt8;
use crate::src::msac::rav1d_msac_decode_uniform;
use crate::src::msac::rav1d_msac_init;
use crate::src::picture::rav1d_picture_alloc_copy;
use crate::src::picture::rav1d_picture_ref;
use crate::src::picture::rav1d_picture_unref_internal;
use crate::src::picture::rav1d_thread_picture_alloc;
use crate::src::picture::rav1d_thread_picture_ref;
use crate::src::picture::rav1d_thread_picture_unref;
use crate::src::picture::Rav1dThreadPicture;
use crate::src::qm::dav1d_qm_tbl;
use crate::src::r#ref::rav1d_ref_create_using_pool;
use crate::src::r#ref::rav1d_ref_dec;
use crate::src::r#ref::rav1d_ref_inc;
use crate::src::recon::DEBUG_BLOCK_INFO;
use crate::src::refmvs::rav1d_refmvs_find;
use crate::src::refmvs::rav1d_refmvs_init_frame;
use crate::src::refmvs::rav1d_refmvs_save_tmvs;
use crate::src::refmvs::rav1d_refmvs_tile_sbrow_init;
use crate::src::refmvs::refmvs_block;
use crate::src::refmvs::refmvs_block_unaligned;
use crate::src::refmvs::refmvs_mvpair;
use crate::src::refmvs::refmvs_refpair;
use crate::src::refmvs::refmvs_temporal_block;
use crate::src::tables::cfl_allowed_mask;
use crate::src::tables::dav1d_al_part_ctx;
use crate::src::tables::dav1d_block_dimensions;
use crate::src::tables::dav1d_block_sizes;
use crate::src::tables::dav1d_comp_inter_pred_modes;
use crate::src::tables::dav1d_filter_2d;
use crate::src::tables::dav1d_filter_dir;
use crate::src::tables::dav1d_intra_mode_context;
use crate::src::tables::dav1d_max_txfm_size_for_bs;
use crate::src::tables::dav1d_partition_type_count;
use crate::src::tables::dav1d_sgr_params;
use crate::src::tables::dav1d_txfm_dimensions;
use crate::src::tables::dav1d_wedge_ctx_lut;
use crate::src::tables::dav1d_ymode_size_context;
use crate::src::tables::interintra_allowed_mask;
use crate::src::tables::wedge_allowed_mask;
use crate::src::thread_task::rav1d_task_create_tile_sbrow;
use crate::src::thread_task::rav1d_task_frame_init;
use crate::src::thread_task::FRAME_ERROR;
use crate::src::thread_task::TILE_ERROR;
use crate::src::warpmv::rav1d_find_affine_int;
use crate::src::warpmv::rav1d_get_shear_params;
use crate::src::warpmv::rav1d_set_affine_mv2d;
use libc::free;
use libc::malloc;
use libc::pthread_cond_signal;
use libc::pthread_cond_wait;
use libc::pthread_mutex_lock;
use libc::pthread_mutex_unlock;
use libc::ptrdiff_t;
use libc::uintptr_t;
use std::array;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_void;
use std::iter;
use std::mem;
use std::ptr;
use std::ptr::addr_of_mut;
use std::slice;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;

#[cfg(feature = "bitdepth_8")]
use crate::{
    include::common::bitdepth::BitDepth8, src::cdef_tmpl_8::rav1d_cdef_dsp_init_8bpc,
    src::itx_tmpl_8::rav1d_itx_dsp_init_8bpc,
};

#[cfg(feature = "bitdepth_16")]
use crate::{
    include::common::bitdepth::BitDepth16, src::cdef_tmpl_16::rav1d_cdef_dsp_init_16bpc,
    src::itx_tmpl_16::rav1d_itx_dsp_init_16bpc,
};

fn init_quant_tables(
    seq_hdr: &Rav1dSequenceHeader,
    frame_hdr: &Rav1dFrameHeader,
    qidx: c_int,
    dq: &mut [[[u16; 2]; 3]],
) {
    let tbl = &dav1d_dq_tbl;

    let segmentation_is_enabled = frame_hdr.segmentation.enabled != 0;
    let len = if segmentation_is_enabled { 8 } else { 1 };
    for i in 0..len {
        let yac = if segmentation_is_enabled {
            iclip_u8(qidx + frame_hdr.segmentation.seg_data.d[i].delta_q)
        } else {
            qidx
        };
        let ydc = iclip_u8(yac + frame_hdr.quant.ydc_delta);
        let uac = iclip_u8(yac + frame_hdr.quant.uac_delta);
        let udc = iclip_u8(yac + frame_hdr.quant.udc_delta);
        let vac = iclip_u8(yac + frame_hdr.quant.vac_delta);
        let vdc = iclip_u8(yac + frame_hdr.quant.vdc_delta);
        dq[i][0][0] = tbl[seq_hdr.hbd as usize][ydc as usize][0];
        dq[i][0][1] = tbl[seq_hdr.hbd as usize][yac as usize][1];
        dq[i][1][0] = tbl[seq_hdr.hbd as usize][udc as usize][0];
        dq[i][1][1] = tbl[seq_hdr.hbd as usize][uac as usize][1];
        dq[i][2][0] = tbl[seq_hdr.hbd as usize][vdc as usize][0];
        dq[i][2][1] = tbl[seq_hdr.hbd as usize][vac as usize][1];
    }
}

unsafe fn read_mv_component_diff(
    t: &mut Rav1dTaskContext,
    mv_comp: &mut CdfMvComponent,
    have_fp: bool,
) -> c_int {
    let ts = &mut *t.ts;
    let f = &*t.f;
    let have_hp = f.frame_hdr.as_ref().unwrap().hp != 0;
    let sign = rav1d_msac_decode_bool_adapt(&mut ts.msac, &mut mv_comp.sign.0);
    let cl = rav1d_msac_decode_symbol_adapt16(&mut ts.msac, &mut mv_comp.classes.0, 10);
    let mut up;
    let fp;
    let hp;

    if cl == 0 {
        up = rav1d_msac_decode_bool_adapt(&mut ts.msac, &mut mv_comp.class0.0) as c_uint;
        if have_fp {
            fp = rav1d_msac_decode_symbol_adapt4(
                &mut ts.msac,
                &mut mv_comp.class0_fp[up as usize],
                3,
            );
            hp = if have_hp {
                rav1d_msac_decode_bool_adapt(&mut ts.msac, &mut mv_comp.class0_hp.0)
            } else {
                true
            };
        } else {
            fp = 3;
            hp = true;
        }
    } else {
        up = 1 << cl;
        for n in 0..cl as usize {
            up |=
                (rav1d_msac_decode_bool_adapt(&mut ts.msac, &mut mv_comp.classN[n]) as c_uint) << n;
        }
        if have_fp {
            fp = rav1d_msac_decode_symbol_adapt4(&mut ts.msac, &mut mv_comp.classN_fp.0, 3);
            hp = if have_hp {
                rav1d_msac_decode_bool_adapt(&mut ts.msac, &mut mv_comp.classN_hp.0)
            } else {
                true
            };
        } else {
            fp = 3;
            hp = true;
        }
    }
    let hp = hp as c_uint;

    let diff = ((up << 3 | fp << 1 | hp) + 1) as c_int;

    if sign {
        -diff
    } else {
        diff
    }
}

unsafe fn read_mv_residual(
    t: &mut Rav1dTaskContext,
    ref_mv: &mut mv,
    mv_cdf: &mut CdfMvContext,
    have_fp: bool,
) {
    let ts = &mut *t.ts;
    match rav1d_msac_decode_symbol_adapt4(
        &mut ts.msac,
        &mut ts.cdf.mv.joint.0,
        N_MV_JOINTS as usize - 1,
    ) {
        MV_JOINT_HV => {
            ref_mv.y += read_mv_component_diff(t, &mut mv_cdf.comp[0], have_fp) as i16;
            ref_mv.x += read_mv_component_diff(t, &mut mv_cdf.comp[1], have_fp) as i16;
        }
        MV_JOINT_H => {
            ref_mv.x += read_mv_component_diff(t, &mut mv_cdf.comp[1], have_fp) as i16;
        }
        MV_JOINT_V => {
            ref_mv.y += read_mv_component_diff(t, &mut mv_cdf.comp[0], have_fp) as i16;
        }
        _ => {}
    };
}

unsafe fn read_tx_tree(
    t: &mut Rav1dTaskContext,
    from: RectTxfmSize,
    depth: c_int,
    masks: &mut [u16; 2],
    x_off: usize,
    y_off: usize,
) {
    let f = &*t.f;
    let bx4 = t.bx & 31;
    let by4 = t.by & 31;
    let t_dim = &dav1d_txfm_dimensions[from as usize];
    let txw = t_dim.lw;
    let txh = t_dim.lh;
    let is_split;

    if depth < 2 && from > TX_4X4 {
        let cat = 2 * (TX_64X64 as c_int - t_dim.max as c_int) - depth;
        let a = ((*t.a).tx.0[bx4 as usize] < txw) as c_int;
        let l = (t.l.tx.0[by4 as usize] < txh) as c_int;

        is_split = rav1d_msac_decode_bool_adapt(
            &mut (*t.ts).msac,
            &mut (*t.ts).cdf.m.txpart[cat as usize][(a + l) as usize],
        );
        if is_split {
            masks[depth as usize] |= 1 << (y_off * 4 + x_off);
        }
    } else {
        is_split = false;
    }
    if is_split && t_dim.max as TxfmSize > TX_8X8 {
        let sub = t_dim.sub as RectTxfmSize;
        let sub_t_dim = &dav1d_txfm_dimensions[usize::from(sub)]; // `from` used instead of `into` for rust-analyzer type inference
        let txsw = sub_t_dim.w as c_int;
        let txsh = sub_t_dim.h as c_int;

        read_tx_tree(t, sub, depth + 1, masks, x_off * 2 + 0, y_off * 2 + 0);
        t.bx += txsw;
        if txw >= txh && t.bx < f.bw {
            read_tx_tree(t, sub, depth + 1, masks, x_off * 2 + 1, y_off * 2 + 0);
        }
        t.bx -= txsw;
        t.by += txsh;
        if txh >= txw && t.by < f.bh {
            read_tx_tree(t, sub, depth + 1, masks, x_off * 2 + 0, y_off * 2 + 1);
            t.bx += txsw;
            if txw >= txh && t.bx < f.bw {
                read_tx_tree(t, sub, depth + 1, masks, x_off * 2 + 1, y_off * 2 + 1);
            }
            t.bx -= txsw;
        }
        t.by -= txsh;
    } else {
        CaseSet::<16, false>::many(
            [(&mut t.l, txh), (&mut *t.a, txw)],
            [t_dim.h as usize, t_dim.w as usize],
            [by4 as usize, bx4 as usize],
            |case, (dir, val)| {
                case.set(&mut dir.tx.0, if is_split { TX_4X4 } else { val });
            },
        );
    };
}

fn neg_deinterleave(diff: c_int, r#ref: c_int, max: c_int) -> c_int {
    if r#ref == 0 {
        diff
    } else if r#ref >= max - 1 {
        max - diff - 1
    } else if 2 * r#ref < max {
        if diff <= 2 * r#ref {
            if diff & 1 != 0 {
                r#ref + (diff + 1 >> 1)
            } else {
                r#ref - (diff >> 1)
            }
        } else {
            diff
        }
    } else {
        if diff <= 2 * (max - r#ref - 1) {
            if diff & 1 != 0 {
                r#ref + (diff + 1 >> 1)
            } else {
                r#ref - (diff >> 1)
            }
        } else {
            max - (diff + 1)
        }
    }
}

unsafe fn find_matching_ref(
    t: &Rav1dTaskContext,
    intra_edge_flags: EdgeFlags,
    bw4: c_int,
    bh4: c_int,
    w4: c_int,
    h4: c_int,
    have_left: bool,
    have_top: bool,
    r#ref: i8,
    masks: &mut [u64; 2],
) {
    let r = &t.rt.r[((t.by & 31) + 5 - 1) as usize..];
    let mut count = 0;
    let mut have_topleft = have_top && have_left;
    let mut have_topright = cmp::max(bw4, bh4) < 32
        && have_top
        && t.bx + bw4 < (*t.ts).tiling.col_end
        && intra_edge_flags & EDGE_I444_TOP_HAS_RIGHT != 0;

    let bs = |rp: &refmvs_block| dav1d_block_dimensions[rp.0.bs as usize];
    let matches = |rp: &refmvs_block| rp.0.r#ref.r#ref[0] == r#ref + 1 && rp.0.r#ref.r#ref[1] == -1;

    if have_top {
        let mut r2 = r[0].offset(t.bx as isize) as *const _;
        let r2_ref = &*r2;
        if matches(r2_ref) {
            masks[0] |= 1;
            count = 1;
        }
        let mut aw4 = bs(r2_ref)[0] as c_int;
        if aw4 >= bw4 {
            let off = t.bx & aw4 - 1;
            if off != 0 {
                have_topleft = false;
            }
            if aw4 - off > bw4 {
                have_topright = false;
            }
        } else {
            let mut mask = 1 << aw4;
            let mut x = aw4;
            while x < w4 {
                r2 = r2.offset(aw4 as isize);
                let r2_ref = &*r2;
                if matches(r2_ref) {
                    masks[0] |= mask;
                    count += 1;
                    if count >= 8 {
                        return;
                    }
                }
                aw4 = bs(r2_ref)[0] as c_int;
                mask <<= aw4;
                x += aw4;
            }
        }
    }
    if have_left {
        let mut r2 = &r[1..];
        let r2_ref = &*r2[0].offset((t.bx - 1) as isize);
        if matches(r2_ref) {
            masks[1] |= 1;
            count += 1;
            if count >= 8 {
                return;
            }
        }
        let mut lh4 = bs(r2_ref)[1] as c_int;
        if lh4 >= bh4 {
            if t.by & lh4 - 1 != 0 {
                have_topleft = false;
            }
        } else {
            let mut mask = 1 << lh4;
            let mut y = lh4;
            while y < h4 {
                r2 = &r2[lh4 as usize..];
                let r2_ref = &*r2[0].offset((t.bx - 1) as isize);
                if matches(r2_ref) {
                    masks[1] |= mask;
                    count += 1;
                    if count >= 8 {
                        return;
                    }
                }
                lh4 = bs(r2_ref)[1] as c_int;
                mask <<= lh4;
                y += lh4;
            }
        }
    }
    if have_topleft && matches(&*r[0].offset((t.bx - 1) as isize)) {
        masks[1] |= 1 << 32;
        count += 1;
        if count >= 8 {
            return;
        }
    }
    if have_topright && matches(&*r[0].offset((t.bx + bw4) as isize)) {
        masks[0] |= 1 << 32;
    }
}

unsafe fn derive_warpmv(
    t: &Rav1dTaskContext,
    bw4: c_int,
    bh4: c_int,
    masks: &[u64; 2],
    mv: mv,
    mut wmp: Rav1dWarpedMotionParams,
) -> Rav1dWarpedMotionParams {
    let mut pts = [[[0; 2 /* x, y */]; 2 /* in, out */]; 8];
    let mut np = 0;
    let r = |i: isize| {
        // Need to use a closure here vs. a slice because `i` can be negative
        // (and not just by a constant -1).
        // See `-off` below.
        let offset = (t.by & 31) + 5;
        t.rt.r[(offset as isize + i) as usize]
    };

    let rp = |i: i32, j: i32| &*r(i as isize).offset(j as isize);

    let bs = |rp: &refmvs_block| dav1d_block_dimensions[(*rp).0.bs as usize];

    let mut add_sample = |np: usize, dx: i32, dy: i32, sx: i32, sy: i32, rp: &refmvs_block| {
        pts[np][0][0] = 16 * (2 * dx + sx * bs(rp)[0] as i32) - 8;
        pts[np][0][1] = 16 * (2 * dy + sy * bs(rp)[1] as i32) - 8;
        pts[np][1][0] = pts[np][0][0] + (*rp).0.mv.mv[0].x as i32;
        pts[np][1][1] = pts[np][0][1] + (*rp).0.mv.mv[0].y as i32;
        np + 1
    };

    // use masks[] to find the projectable motion vectors in the edges
    if masks[0] as u32 == 1 && masks[1] >> 32 == 0 {
        let off = t.bx & bs(rp(-1, t.bx))[0] as i32 - 1;
        np = add_sample(np, -off, 0, 1, -1, rp(-1, t.bx));
    } else {
        let mut off = 0;
        let mut xmask = masks[0] as u32;
        while np < 8 && xmask != 0 {
            let tz = ctz(xmask);
            off += tz;
            xmask >>= tz;
            np = add_sample(np, off, 0, 1, -1, rp(-1, t.bx + off));
            xmask &= !1;
        }
    }
    if np < 8 && masks[1] as u32 == 1 {
        let off = t.by & bs(rp(0, t.bx - 1))[1] as i32 - 1;
        np = add_sample(np, 0, -off, -1, 1, rp(-off, t.bx - 1));
    } else {
        let mut off = 0;
        let mut ymask = masks[1] as u32;
        while np < 8 && ymask != 0 {
            let tz = ctz(ymask);
            off += tz;
            ymask >>= tz;
            np = add_sample(np, 0, off, -1, 1, rp(off, t.bx - 1));
            ymask &= !1;
        }
    }
    if np < 8 && masks[1] >> 32 != 0 {
        // top/left
        np = add_sample(np, 0, 0, -1, -1, rp(-1, t.bx - 1));
    }
    if np < 8 && masks[0] >> 32 != 0 {
        // top/right
        np = add_sample(np, bw4, 0, 1, -1, rp(-1, t.bx + bw4));
    }
    assert!(np > 0 && np <= 8);

    // select according to motion vector difference against a threshold
    let mut mvd = [0; 8];
    let mut ret = 0;
    let thresh = 4 * iclip(cmp::max(bw4, bh4), 4, 28);
    for (mvd, pts) in std::iter::zip(&mut mvd[..np], &pts[..np]) {
        *mvd = (pts[1][0] - pts[0][0] - mv.x as i32).abs()
            + (pts[1][1] - pts[0][1] - mv.y as i32).abs();
        if *mvd > thresh {
            *mvd = -1;
        } else {
            ret += 1;
        }
    }
    if ret == 0 {
        ret = 1;
    } else {
        let mut i = 0;
        let mut j = np - 1;
        for _ in 0..np - ret {
            while mvd[i] != -1 {
                i += 1;
            }
            while mvd[j] == -1 {
                j -= 1;
            }
            assert!(i != j);
            if i > j {
                break;
            }
            // replace the discarded samples;
            mvd[i] = mvd[j];
            pts[i] = pts[j];
            i += 1;
            j -= 1;
        }
    }

    wmp.r#type = if !rav1d_find_affine_int(&pts, ret, bw4, bh4, mv, &mut wmp, t.bx, t.by)
        && !rav1d_get_shear_params(&mut wmp)
    {
        RAV1D_WM_TYPE_AFFINE
    } else {
        RAV1D_WM_TYPE_IDENTITY
    };
    wmp
}

#[inline]
fn findoddzero(buf: &[u8]) -> bool {
    buf.iter()
        .enumerate()
        .find(|(i, &e)| i & 1 == 1 && e == 0)
        .is_some()
}

unsafe fn read_pal_plane(
    t: &mut Rav1dTaskContext,
    b: &mut Av1Block,
    pl: bool,
    sz_ctx: u8,
    bx4: usize,
    by4: usize,
) {
    let pli = pl as usize;
    let not_pl = !pl as u16;

    let ts = &mut *t.ts;
    let f = &*t.f;

    // Must come before `pal`, which mutably borrows `t`.
    // TODO: `DEBUG_BLOCK_INFO` really should take a subset of `f` and `t`,
    // i.e. only the fields it needs, as this would solve the bitdepth-dependence problem
    // as well as the borrowck error here if `dbg` is not hoisted.
    let dbg = DEBUG_BLOCK_INFO(f, t);

    let pal_sz = rav1d_msac_decode_symbol_adapt8(
        &mut ts.msac,
        &mut ts.cdf.m.pal_sz[pli][sz_ctx as usize],
        6,
    ) as u8
        + 2;
    b.pal_sz_mut()[pli] = pal_sz;
    let pal_sz = pal_sz as usize;
    let mut cache = <[u16; 16]>::default();
    let mut used_cache = <[u16; 8]>::default();
    let mut l_cache = if pl {
        t.pal_sz_uv[1][by4]
    } else {
        t.l.pal_sz.0[by4]
    };
    let mut n_cache = 0;
    // don't reuse above palette outside SB64 boundaries
    let mut a_cache = if by4 & 15 != 0 {
        if pl {
            t.pal_sz_uv[0][bx4]
        } else {
            (*t.a).pal_sz.0[bx4]
        }
    } else {
        0
    };
    let [a, l] = &mut t.al_pal;
    let mut l = &l[by4][pli][..];
    let mut a = &a[bx4][pli][..];

    // fill/sort cache
    // TODO: This logic could be replaced with `itertools`' `.merge` and `.dedup`, which would elide bounds checks.
    while l_cache != 0 && a_cache != 0 {
        if l[0] < a[0] {
            if n_cache == 0 || cache[n_cache - 1] != l[0] {
                cache[n_cache] = l[0];
                n_cache += 1;
            }
            l = &l[1..];
            l_cache -= 1;
        } else {
            if a[0] == l[0] {
                l = &l[1..];
                l_cache -= 1;
            }
            if n_cache == 0 || cache[n_cache - 1] != a[0] {
                cache[n_cache] = a[0];
                n_cache += 1;
            }
            a = &a[1..];
            a_cache -= 1;
        }
    }
    if l_cache != 0 {
        loop {
            if n_cache == 0 || cache[n_cache - 1] != l[0] {
                cache[n_cache] = l[0];
                n_cache += 1;
            }
            l = &l[1..];
            l_cache -= 1;
            if !(l_cache > 0) {
                break;
            }
        }
    } else if a_cache != 0 {
        loop {
            if n_cache == 0 || cache[n_cache - 1] != a[0] {
                cache[n_cache] = a[0];
                n_cache += 1;
            }
            a = &a[1..];
            a_cache -= 1;
            if !(a_cache > 0) {
                break;
            }
        }
    }
    let cache = &cache[..n_cache];

    // find reused cache entries
    // TODO: Bounds checks could be elided with more complex iterators.
    let mut i = 0;
    for cache in cache {
        if !(i < pal_sz) {
            break;
        }
        if rav1d_msac_decode_bool_equi(&mut ts.msac) {
            used_cache[i] = *cache;
            i += 1;
        }
    }
    let used_cache = &used_cache[..i];

    // parse new entries
    let pal = if t.frame_thread.pass != 0 {
        &mut (*(f.frame_thread.pal).offset(
            ((t.by >> 1) + (t.bx & 1)) as isize * (f.b4_stride >> 1)
                + ((t.bx >> 1) + (t.by & 1)) as isize,
        ))[pli]
    } else {
        &mut t.scratch.c2rust_unnamed_0.pal[pli]
    };
    let pal = &mut pal[..pal_sz];
    if i < pal.len() {
        let mut prev = rav1d_msac_decode_bools(&mut ts.msac, f.cur.p.bpc as u32) as u16;
        pal[i] = prev;
        i += 1;

        if i < pal.len() {
            let mut bits = f.cur.p.bpc as u32 + rav1d_msac_decode_bools(&mut ts.msac, 2) - 3;
            let max = (1 << f.cur.p.bpc) - 1;

            loop {
                let delta = rav1d_msac_decode_bools(&mut ts.msac, bits) as u16;
                prev = cmp::min(prev + delta + not_pl, max);
                pal[i] = prev;
                i += 1;
                if prev + not_pl >= max {
                    pal[i..].fill(max);
                    break;
                } else {
                    bits = cmp::min(bits, 1 + ulog2((max - prev - not_pl) as u32) as u32);
                    if !(i < pal.len()) {
                        break;
                    }
                }
            }
        }

        // merge cache+new entries
        let mut n = 0;
        let mut m = used_cache.len();
        for i in 0..pal.len() {
            if n < used_cache.len() && (m >= pal.len() || used_cache[n] <= pal[m]) {
                pal[i] = used_cache[n];
                n += 1;
            } else {
                pal[i] = pal[m];
                m += 1;
            }
        }
    } else {
        pal[..used_cache.len()].copy_from_slice(&used_cache);
    }

    if dbg {
        print!(
            "Post-pal[pl={},sz={},cache_size={},used_cache={}]: r={}, cache=",
            pli,
            pal_sz,
            cache.len(),
            used_cache.len(),
            ts.msac.rng
        );
        for (n, cache) in cache.iter().enumerate() {
            print!("{}{:02x}", if n != 0 { ' ' } else { '[' }, cache);
        }
        print!("{}, pal=", if cache.len() != 0 { "]" } else { "[]" });
        for (n, pal) in pal.iter().enumerate() {
            print!("{}{:02x}", if n != 0 { ' ' } else { '[' }, pal);
        }
        println!("]");
    }
}

unsafe fn read_pal_uv(
    t: &mut Rav1dTaskContext,
    b: &mut Av1Block,
    sz_ctx: u8,
    bx4: usize,
    by4: usize,
) {
    read_pal_plane(t, b, true, sz_ctx, bx4, by4);

    // V pal coding
    let ts = &mut *t.ts;
    let f = &*t.f;

    // Hoisted so the `&` borrow of `t`
    // doesn't conflict with `pal`'s `&mut` borrow of `t`.
    let dbg = DEBUG_BLOCK_INFO(&*f, &*t);

    let pal = if t.frame_thread.pass != 0 {
        &mut (*(f.frame_thread.pal).offset(
            ((t.by >> 1) + (t.bx & 1)) as isize * (f.b4_stride >> 1)
                + ((t.bx >> 1) + (t.by & 1)) as isize,
        ))[2]
    } else {
        &mut t.scratch.c2rust_unnamed_0.pal[2]
    };
    let pal = &mut pal[..b.pal_sz()[1] as usize];
    if rav1d_msac_decode_bool_equi(&mut ts.msac) {
        let bits = f.cur.p.bpc as u32 + rav1d_msac_decode_bools(&mut ts.msac, 2) - 4;
        let mut prev = rav1d_msac_decode_bools(&mut ts.msac, f.cur.p.bpc as c_uint) as u16;
        pal[0] = prev;
        let max = (1 << f.cur.p.bpc) - 1;
        for pal in &mut pal[1..] {
            let mut delta = rav1d_msac_decode_bools(&mut ts.msac, bits) as i16;
            if delta != 0 && rav1d_msac_decode_bool_equi(&mut ts.msac) {
                delta = -delta;
            }
            prev = ((prev as i16 + delta) as u16) & max;
            *pal = prev;
        }
    } else {
        pal.fill_with(|| rav1d_msac_decode_bools(&mut ts.msac, f.cur.p.bpc as c_uint) as u16);
    }
    if dbg {
        print!("Post-pal[pl=2]: r={} ", ts.msac.rng);
        for (n, pal) in pal.iter().enumerate() {
            print!("{}{:02x}", if n != 0 { ' ' } else { '[' }, pal);
        }
        println!("]");
    }
}

fn order_palette(
    pal_idx: &[u8],
    stride: usize,
    i: usize,
    first: usize,
    last: usize,
    order: &mut [[u8; u8::BITS as usize]; 64],
    ctx: &mut [u8; 64],
) {
    let mut have_top = i > first;

    let mut offset = first + (i - first) * stride;

    for ((ctx, order), j) in ctx
        .iter_mut()
        .zip(order.iter_mut())
        .zip((last..=first).rev())
    {
        let have_left = j > 0;

        assert!(have_left || have_top);

        let mut mask = 0u8;
        let mut o_idx = 0;
        let mut add = |v: u8| {
            assert!(v < u8::BITS as u8);
            order[o_idx] = v;
            o_idx += 1;
            mask |= 1 << v;
        };

        if !have_left {
            *ctx = 0;
            add(pal_idx[offset - stride]);
        } else if !have_top {
            *ctx = 0;
            add(pal_idx[offset - 1]);
        } else {
            let l = pal_idx[offset - 1];
            let t = pal_idx[offset - stride];
            let tl = pal_idx[offset - (stride + 1)];
            let same_t_l = t == l;
            let same_t_tl = t == tl;
            let same_l_tl = l == tl;
            let same_all = same_t_l & same_t_tl & same_l_tl;

            if same_all {
                *ctx = 4;
                add(t);
            } else if same_t_l {
                *ctx = 3;
                add(t);
                add(tl);
            } else if same_t_tl | same_l_tl {
                *ctx = 2;
                add(tl);
                add(if same_t_tl { l } else { t });
            } else {
                *ctx = 1;
                add(cmp::min(t, l));
                add(cmp::max(t, l));
                add(tl);
            }
        }
        for bit in 0..u8::BITS as u8 {
            if mask & (1 << bit) == 0 {
                order[o_idx] = bit;
                o_idx += 1;
            }
        }
        assert!(o_idx == u8::BITS as usize);
        have_top = true;
        offset += stride - 1;
    }
}

unsafe fn read_pal_indices(
    ts: &mut Rav1dTileState,
    scratch_pal: &mut Rav1dTaskContext_scratch_pal,
    pal_idx: &mut [u8],
    b: &Av1Block,
    pl: bool,
    w4: c_int,
    h4: c_int,
    bw4: c_int,
    bh4: c_int,
) {
    let [w4, h4, bw4, bh4] = [w4, h4, bw4, bh4].map(|n| usize::try_from(n).unwrap());
    let pli = pl as usize;
    let pal_sz = b.pal_sz()[pli] as usize;

    let stride = bw4 * 4;
    pal_idx[0] = rav1d_msac_decode_uniform(&mut ts.msac, pal_sz as c_uint) as u8;
    let color_map_cdf = &mut ts.cdf.m.color_map[pli][pal_sz - 2];
    let Rav1dTaskContext_scratch_pal {
        pal_order: order,
        pal_ctx: ctx,
    } = scratch_pal;
    for i in 1..4 * (w4 + h4) - 1 {
        // top/left-to-bottom/right diagonals ("wave-front")
        let first = cmp::min(i, w4 * 4 - 1);
        let last = (i + 1).checked_sub(h4 * 4).unwrap_or(0);
        order_palette(pal_idx, stride, i, first, last, order, ctx);
        for (m, j) in (last..=first).rev().enumerate() {
            let color_idx = rav1d_msac_decode_symbol_adapt8(
                &mut ts.msac,
                &mut color_map_cdf[ctx[m] as usize],
                pal_sz - 1,
            ) as usize;
            pal_idx[(i - j) * stride + j] = order[m][color_idx];
        }
    }
    // fill invisible edges
    if bw4 > w4 {
        for y in 0..4 * h4 {
            let offset = y * stride + (4 * w4);
            let len = 4 * (bw4 - w4);
            let filler = pal_idx[offset - 1];
            pal_idx[offset..][..len].fill(filler);
        }
    }
    if h4 < bh4 {
        let y_start = h4 * 4;
        let len = bw4 * 4;
        let (src, dests) = pal_idx.split_at_mut(stride * y_start);
        let src = &src[stride * (y_start - 1)..][..len];
        for y in 0..(bh4 - h4) * 4 {
            dests[y * stride..][..len].copy_from_slice(src);
        }
    }
}

unsafe fn read_vartx_tree(
    t: &mut Rav1dTaskContext,
    b: &mut Av1Block,
    bs: BlockSize,
    bx4: c_int,
    by4: c_int,
) {
    let f = &*t.f;
    let b_dim = &dav1d_block_dimensions[bs as usize];
    let bw4 = b_dim[0] as usize;
    let bh4 = b_dim[1] as usize;

    // var-tx tree coding
    let mut tx_split = [0u16; 2];
    *b.max_ytx_mut() = dav1d_max_txfm_size_for_bs[bs as usize][0];
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    let txfm_mode = frame_hdr.txfm_mode as Dav1dTxfmMode;
    if b.skip == 0
        && (frame_hdr.segmentation.lossless[b.seg_id as usize] != 0
            || b.max_ytx() as TxfmSize == TX_4X4)
    {
        b.uvtx = TX_4X4 as u8;
        *b.max_ytx_mut() = b.uvtx;
        if txfm_mode == RAV1D_TX_SWITCHABLE {
            CaseSet::<32, false>::many(
                [&mut t.l, &mut *t.a],
                [bh4 as usize, bw4 as usize],
                [by4 as usize, bx4 as usize],
                |case, dir| {
                    case.set(&mut dir.tx.0, TX_4X4);
                },
            );
        }
    } else if txfm_mode != RAV1D_TX_SWITCHABLE || b.skip != 0 {
        if txfm_mode == RAV1D_TX_SWITCHABLE {
            CaseSet::<32, false>::many(
                [(&mut t.l, 1), (&mut *t.a, 0)],
                [bh4 as usize, bw4 as usize],
                [by4 as usize, bx4 as usize],
                |case, (dir, dir_index)| {
                    case.set(&mut dir.tx.0, b_dim[2 + dir_index]);
                },
            );
        }
        b.uvtx = dav1d_max_txfm_size_for_bs[bs as usize][f.cur.p.layout as usize];
    } else {
        assert!(bw4 <= 16 || bh4 <= 16 || b.max_ytx() as TxfmSize == TX_64X64);
        let ytx = &dav1d_txfm_dimensions[b.max_ytx() as usize];
        let h = ytx.h as usize;
        let w = ytx.w as usize;
        debug_assert_eq!(bh4 % h, 0);
        debug_assert_eq!(bw4 % w, 0);
        for y_off in 0..bh4 / h {
            for x_off in 0..bw4 / w {
                read_tx_tree(
                    &mut *t,
                    b.max_ytx() as RectTxfmSize,
                    0,
                    &mut tx_split,
                    x_off,
                    y_off,
                );
                // contexts are updated inside read_tx_tree()
                t.bx += w as c_int;
            }
            t.bx -= bw4 as c_int;
            t.by += h as c_int;
        }
        t.by -= bh4 as c_int;
        if DEBUG_BLOCK_INFO(&*f, &*t) {
            println!(
                "Post-vartxtree[{}/{}]: r={}",
                tx_split[0],
                tx_split[1],
                (*t.ts).msac.rng
            );
        }
        b.uvtx = dav1d_max_txfm_size_for_bs[bs as usize][f.cur.p.layout as usize];
    }
    assert!(tx_split[0] & !0x33 == 0);
    b.c2rust_unnamed.c2rust_unnamed_0.tx_split0 = tx_split[0] as u8;
    b.c2rust_unnamed.c2rust_unnamed_0.tx_split1 = tx_split[1];
}

#[inline]
unsafe fn get_prev_frame_segid(
    f: &Rav1dFrameContext,
    by: c_int,
    bx: c_int,
    w4: c_int,
    h4: c_int,
    // It's very difficult to make this safe (a slice),
    // as it comes from [`Dav1dFrameContext::prev_segmap`],
    // which is set to [`Dav1dFrameContext::prev_segmap_ref`],
    // which is a [`Dav1dRef`], which has no size and is refcounted.
    ref_seg_map: *const u8,
    stride: ptrdiff_t,
) -> u8 {
    assert!(f.frame_hdr.as_ref().unwrap().primary_ref_frame != RAV1D_PRIMARY_REF_NONE);

    // Need checked casts here because an overflowing cast
    // would give a too large `len` to [`std::slice::from_raw_parts`], which would UB.
    let w4 = usize::try_from(w4).unwrap();
    let h4 = usize::try_from(h4).unwrap();
    let stride = usize::try_from(stride).unwrap();

    let mut prev_seg_id = 8;
    let ref_seg_map = std::slice::from_raw_parts(
        ref_seg_map.offset(by as isize * stride as isize + bx as isize),
        h4 * stride,
    );

    assert!(w4 <= stride);
    for ref_seg_map in ref_seg_map.chunks_exact(stride) {
        prev_seg_id = ref_seg_map[..w4]
            .iter()
            .copied()
            .fold(prev_seg_id, cmp::min);
        if prev_seg_id == 0 {
            break;
        }
    }
    assert!(prev_seg_id < 8);

    prev_seg_id
}

#[inline]
unsafe fn splat_oneref_mv(
    c: &Rav1dContext,
    t: &mut Rav1dTaskContext,
    bs: BlockSize,
    b: &Av1Block,
    bw4: usize,
    bh4: usize,
) {
    let mode = b.inter_mode();
    let tmpl = Align16(refmvs_block(refmvs_block_unaligned {
        mv: refmvs_mvpair {
            mv: [b.mv()[0], mv::ZERO],
        },
        r#ref: refmvs_refpair {
            r#ref: [
                b.r#ref()[0] + 1,
                if b.interintra_type() != 0 { 0 } else { -1 },
            ],
        },
        bs: bs as u8,
        mf: (mode == GLOBALMV && cmp::min(bw4, bh4) >= 2) as u8 | (mode == NEWMV) as u8 * 2,
    }));
    c.refmvs_dsp.splat_mv(
        &mut t.rt.r[((t.by & 31) + 5) as usize..],
        &tmpl.0,
        t.bx as usize,
        bw4,
        bh4,
    );
}

#[inline]
unsafe fn splat_intrabc_mv(
    c: &Rav1dContext,
    t: &mut Rav1dTaskContext,
    bs: BlockSize,
    b: &Av1Block,
    bw4: usize,
    bh4: usize,
) {
    let tmpl = Align16(refmvs_block(refmvs_block_unaligned {
        mv: refmvs_mvpair {
            mv: [b.mv()[0], mv::ZERO],
        },
        r#ref: refmvs_refpair { r#ref: [0, -1] },
        bs: bs as u8,
        mf: 0,
    }));
    c.refmvs_dsp.splat_mv(
        &mut t.rt.r[((t.by & 31) + 5) as usize..],
        &tmpl.0,
        t.bx as usize,
        bw4,
        bh4,
    );
}

#[inline]
unsafe fn splat_tworef_mv(
    c: &Rav1dContext,
    t: &mut Rav1dTaskContext,
    bs: BlockSize,
    b: &Av1Block,
    bw4: usize,
    bh4: usize,
) {
    assert!(bw4 >= 2 && bh4 >= 2);
    let mode = b.inter_mode();
    let tmpl = Align16(refmvs_block(refmvs_block_unaligned {
        mv: refmvs_mvpair { mv: *b.mv() },
        r#ref: refmvs_refpair {
            r#ref: [b.r#ref()[0] + 1, b.r#ref()[1] + 1],
        },
        bs: bs as u8,
        mf: (mode == GLOBALMV_GLOBALMV) as u8 | (1 << mode & 0xbc != 0) as u8 * 2,
    }));
    c.refmvs_dsp.splat_mv(
        &mut t.rt.r[((t.by & 31) + 5) as usize..],
        &tmpl.0,
        t.bx as usize,
        bw4,
        bh4,
    );
}

#[inline]
unsafe fn splat_intraref(
    c: &Rav1dContext,
    t: &mut Rav1dTaskContext,
    bs: BlockSize,
    bw4: usize,
    bh4: usize,
) {
    let tmpl = Align16(refmvs_block(refmvs_block_unaligned {
        mv: refmvs_mvpair {
            mv: [mv::INVALID, mv::ZERO],
        },
        r#ref: refmvs_refpair { r#ref: [0, -1] },
        bs: bs as u8,
        mf: 0,
    }));
    c.refmvs_dsp.splat_mv(
        &mut t.rt.r[((t.by & 31) + 5) as usize..],
        &tmpl.0,
        t.bx as usize,
        bw4,
        bh4,
    );
}

fn mc_lowest_px(
    dst: &mut c_int,
    by4: c_int,
    bh4: c_int,
    mvy: i16,
    ss_ver: c_int,
    smp: &ScalableMotionParams,
) {
    let mvy = mvy as c_int;

    let v_mul = 4 >> ss_ver;
    if smp.scale == 0 {
        let my = mvy >> 3 + ss_ver;
        let dy = mvy & 15 >> (ss_ver == 0) as c_int;
        *dst = cmp::max(*dst, (by4 + bh4) * v_mul + my + 4 * (dy != 0) as c_int);
    } else {
        let mut y = (by4 * v_mul << 4) + mvy * (1 << (ss_ver == 0) as c_int);
        let tmp = y as i64 * smp.scale as i64 + ((smp.scale - 0x4000) * 8) as i64;
        y = apply_sign64((tmp.abs() + 128 >> 8) as c_int, tmp) + 32;
        let bottom = (y + (bh4 * v_mul - 1) * smp.step >> 10) + 1 + 4;
        *dst = cmp::max(*dst, bottom);
    };
}

#[inline(always)]
fn affine_lowest_px(
    t: &Rav1dTaskContext,
    dst: &mut c_int,
    b_dim: &[u8; 4],
    wmp: &Rav1dWarpedMotionParams,
    ss_ver: c_int,
    ss_hor: c_int,
) {
    let h_mul = 4 >> ss_hor;
    let v_mul = 4 >> ss_ver;
    assert!(b_dim[0] as c_int * h_mul & 7 == 0 && b_dim[1] as c_int * v_mul & 7 == 0);
    let mat = &wmp.matrix;
    let y = b_dim[1] as c_int * v_mul - 8;
    let src_y = t.by * 4 + ((y + 4) << ss_ver);
    let mat5_y = mat[5] as i64 * src_y as i64 + mat[1] as i64;
    let mut x = 0;
    while x < b_dim[0] as c_int * h_mul {
        let src_x = t.bx * 4 + ((x + 4) << ss_hor);
        let mvy = mat[4] as i64 * src_x as i64 + mat5_y >> ss_ver;
        let dy = (mvy >> 16) as c_int - 4;
        *dst = cmp::max(*dst, dy + 4 + 8);
        x += cmp::max(8, b_dim[0] as c_int * h_mul - 8);
    }
}

#[inline(never)]
fn affine_lowest_px_luma(
    t: &Rav1dTaskContext,
    dst: &mut c_int,
    b_dim: &[u8; 4],
    wmp: &Rav1dWarpedMotionParams,
) {
    affine_lowest_px(t, dst, b_dim, wmp, 0, 0);
}

#[inline(never)]
unsafe fn affine_lowest_px_chroma(
    t: &Rav1dTaskContext,
    dst: &mut c_int,
    b_dim: &[u8; 4],
    wmp: &Rav1dWarpedMotionParams,
) {
    let f = &*t.f;
    assert!(f.cur.p.layout != Rav1dPixelLayout::I400);
    if f.cur.p.layout == Rav1dPixelLayout::I444 {
        affine_lowest_px_luma(t, dst, b_dim, wmp);
    } else {
        affine_lowest_px(
            t,
            dst,
            b_dim,
            wmp,
            (f.cur.p.layout & Rav1dPixelLayout::I420) as c_int,
            1,
        );
    };
}

unsafe fn obmc_lowest_px(
    t: &mut Rav1dTaskContext,
    dst: &mut [[c_int; 2]; 7],
    is_chroma: bool,
    b_dim: &[u8; 4],
    _bx4: c_int,
    _by4: c_int,
    w4: c_int,
    h4: c_int,
) {
    assert!(t.bx & 1 == 0 && t.by & 1 == 0);
    let f = &*t.f;
    let r = &t.rt.r[(t.by as usize & 31) + 5 - 1..];
    let ss_ver = (is_chroma && f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
    let ss_hor = (is_chroma && f.cur.p.layout != Rav1dPixelLayout::I444) as c_int;
    let h_mul = 4 >> ss_hor;
    let v_mul = 4 >> ss_ver;
    if t.by > (*t.ts).tiling.row_start
        && (!is_chroma || b_dim[0] as c_int * h_mul + b_dim[1] as c_int * v_mul >= 16)
    {
        let mut i = 0;
        let mut x = 0;
        while x < w4 && i < cmp::min(b_dim[2] as c_int, 4) {
            let a_r = &*r[0].offset((t.bx + x + 1) as isize);
            let a_b_dim = &dav1d_block_dimensions[a_r.0.bs as usize];
            if a_r.0.r#ref.r#ref[0] as c_int > 0 {
                let oh4 = cmp::min(b_dim[1] as c_int, 16) >> 1;
                mc_lowest_px(
                    &mut dst[a_r.0.r#ref.r#ref[0] as usize - 1][is_chroma as usize],
                    t.by,
                    oh4 * 3 + 3 >> 2,
                    a_r.0.mv.mv[0].y,
                    ss_ver,
                    &f.svc[a_r.0.r#ref.r#ref[0] as usize - 1][1],
                );
                i += 1;
            }
            x += cmp::max(a_b_dim[0] as c_int, 2);
        }
    }
    if t.bx > (*t.ts).tiling.col_start {
        let mut i = 0;
        let mut y = 0;
        while y < h4 && i < cmp::min(b_dim[3] as c_int, 4) {
            let l_r = &*r[y as usize + 1 + 1].offset((t.bx - 1) as isize);
            let l_b_dim = &dav1d_block_dimensions[l_r.0.bs as usize];
            if l_r.0.r#ref.r#ref[0] as c_int > 0 {
                let oh4 = iclip(l_b_dim[1] as c_int, 2, b_dim[1] as c_int);
                mc_lowest_px(
                    &mut dst[l_r.0.r#ref.r#ref[0] as usize - 1][is_chroma as usize],
                    t.by + y,
                    oh4,
                    l_r.0.mv.mv[0].y,
                    ss_ver,
                    &f.svc[l_r.0.r#ref.r#ref[0] as usize - 1][1],
                );
                i += 1;
            }
            y += cmp::max(l_b_dim[1] as c_int, 2);
        }
    }
}

unsafe fn decode_b(
    t: &mut Rav1dTaskContext,
    bl: BlockLevel,
    bs: BlockSize,
    bp: BlockPartition,
    intra_edge_flags: EdgeFlags,
) -> Result<(), ()> {
    use std::fmt;

    /// Helper struct for printing a number as a signed hexidecimal value.
    struct SignAbs(i32);

    impl fmt::Display for SignAbs {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let sign = if self.0 < 0 { "-" } else { " " };
            write!(f, "{}{:x}", sign, self.0.abs())
        }
    }

    let ts = &mut *t.ts;
    let f = &*t.f;
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    let mut b_mem = Default::default();
    let b = if t.frame_thread.pass != 0 {
        &mut *f
            .frame_thread
            .b
            .offset(t.by as isize * f.b4_stride + t.bx as isize)
    } else {
        &mut b_mem
    };
    let b_dim = &dav1d_block_dimensions[bs as usize];
    let bx4 = t.bx & 31;
    let by4 = t.by & 31;
    let ss_ver = (f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
    let ss_hor = (f.cur.p.layout != Rav1dPixelLayout::I444) as c_int;
    let cbx4 = bx4 >> ss_hor;
    let cby4 = by4 >> ss_ver;
    let bw4 = b_dim[0] as c_int;
    let bh4 = b_dim[1] as c_int;
    let w4 = cmp::min(bw4, f.bw - t.bx);
    let h4 = cmp::min(bh4, f.bh - t.by);
    let cbw4 = bw4 + ss_hor >> ss_hor;
    let cbh4 = bh4 + ss_ver >> ss_ver;
    let have_left = t.bx > ts.tiling.col_start;
    let have_top = t.by > ts.tiling.row_start;
    let has_chroma = f.cur.p.layout != Rav1dPixelLayout::I400
        && (bw4 > ss_hor || t.bx & 1 != 0)
        && (bh4 > ss_ver || t.by & 1 != 0);

    if t.frame_thread.pass == 2 {
        if b.intra != 0 {
            f.bd_fn.recon_b_intra(t, bs, intra_edge_flags, b);

            let y_mode = b.y_mode();
            let y_mode_nofilt = if y_mode == FILTER_PRED {
                DC_PRED
            } else {
                y_mode
            };
            CaseSet::<32, false>::many(
                [&mut t.l, &mut *t.a],
                [bh4 as usize, bw4 as usize],
                [by4 as usize, bx4 as usize],
                |case, dir| {
                    case.set(&mut dir.mode.0, y_mode_nofilt);
                    case.set(&mut dir.intra.0, 1);
                },
            );
            if frame_hdr.frame_type.is_inter_or_switch() {
                let r = t.rt.r[((t.by & 31) + 5 + bh4 - 1) as usize].offset(t.bx as isize);
                for x in 0..bw4 {
                    let block = &mut *r.offset(x as isize);
                    block.0.r#ref.r#ref[0] = 0;
                    block.0.bs = bs as u8;
                }
                let rr = &t.rt.r[((t.by & 31) + 5) as usize..];
                for y in 0..bh4 - 1 {
                    let block = &mut *rr[y as usize].offset((t.bx + bw4 - 1) as isize);
                    block.0.r#ref.r#ref[0] = 0;
                    block.0.bs = bs as u8;
                }
            }

            if has_chroma {
                CaseSet::<32, false>::many(
                    [&mut t.l, &mut *t.a],
                    [cbh4 as usize, cbw4 as usize],
                    [cby4 as usize, cbx4 as usize],
                    |case, dir| {
                        case.set(&mut dir.uvmode.0, b.uv_mode());
                    },
                );
            }
        } else {
            if frame_hdr.frame_type.is_inter_or_switch() /* not intrabc */
                && b.comp_type() == COMP_INTER_NONE
                && b.motion_mode() as MotionMode == MM_WARP
            {
                if b.matrix()[0] == i16::MIN {
                    t.warpmv.r#type = RAV1D_WM_TYPE_IDENTITY;
                } else {
                    t.warpmv.r#type = RAV1D_WM_TYPE_AFFINE;
                    t.warpmv.matrix[2] = b.matrix()[0] as i32 + 0x10000;
                    t.warpmv.matrix[3] = b.matrix()[1] as i32;
                    t.warpmv.matrix[4] = b.matrix()[2] as i32;
                    t.warpmv.matrix[5] = b.matrix()[3] as i32 + 0x10000;
                    rav1d_set_affine_mv2d(bw4, bh4, *b.mv2d(), &mut t.warpmv, t.bx, t.by);
                    rav1d_get_shear_params(&mut t.warpmv);
                    if DEBUG_BLOCK_INFO(f, t) {
                        println!(
                            "[ {} {} {}\n  {} {} {} ]\n\
                            alpha={}, beta={}, gamma={}, deta={}, mv=y:{},x:{}",
                            SignAbs(t.warpmv.matrix[0]),
                            SignAbs(t.warpmv.matrix[1]),
                            SignAbs(t.warpmv.matrix[2]),
                            SignAbs(t.warpmv.matrix[3]),
                            SignAbs(t.warpmv.matrix[4]),
                            SignAbs(t.warpmv.matrix[5]),
                            SignAbs(t.warpmv.alpha().into()),
                            SignAbs(t.warpmv.beta().into()),
                            SignAbs(t.warpmv.gamma().into()),
                            SignAbs(t.warpmv.delta().into()),
                            b.mv2d().y,
                            b.mv2d().x,
                        );
                    }
                }
            }
            f.bd_fn.recon_b_inter(t, bs, b)?;

            let filter = &dav1d_filter_dir[b.filter2d() as usize];
            CaseSet::<32, false>::many(
                [&mut t.l, &mut *t.a],
                [bh4 as usize, bw4 as usize],
                [by4 as usize, bx4 as usize],
                |case, dir| {
                    case.set(&mut dir.filter.0[0], filter[0]);
                    case.set(&mut dir.filter.0[1], filter[1]);
                    case.set(&mut dir.intra.0, 0);
                },
            );

            if frame_hdr.frame_type.is_inter_or_switch() {
                let r = t.rt.r[((t.by & 31) + 5 + bh4 - 1) as usize].offset(t.bx as isize);
                let r = std::slice::from_raw_parts_mut(r, bw4 as usize);
                for r in r {
                    r.0.r#ref.r#ref[0] = b.r#ref()[0] + 1;
                    r.0.mv.mv[0] = b.mv()[0];
                    r.0.bs = bs as u8;
                }
                let rr = &t.rt.r[((t.by & 31) + 5) as usize..];
                for y in 0..bh4 as usize - 1 {
                    let r = &mut *rr[y].offset((t.bx + bw4 - 1) as isize);
                    r.0.r#ref.r#ref[0] = b.r#ref()[0] + 1;
                    r.0.mv.mv[0] = b.mv()[0];
                    r.0.bs = bs as u8;
                }
            }

            if has_chroma {
                CaseSet::<32, false>::many(
                    [&mut t.l, &mut *t.a],
                    [cbh4 as usize, cbw4 as usize],
                    [cby4 as usize, cbx4 as usize],
                    |case, dir| {
                        case.set(&mut dir.uvmode.0, DC_PRED);
                    },
                );
            }
        }

        return Ok(());
    }

    let cw4 = w4 + ss_hor >> ss_hor;
    let ch4 = h4 + ss_ver >> ss_ver;

    b.bl = bl as u8;
    b.bp = bp as u8;
    b.bs = bs as u8;

    let mut seg = None;

    // segment_id (if seg_feature for skip/ref/gmv is enabled)
    let mut seg_pred = false;
    if frame_hdr.segmentation.enabled != 0 {
        if frame_hdr.segmentation.update_map == 0 {
            if !(f.prev_segmap).is_null() {
                let seg_id =
                    get_prev_frame_segid(f, t.by, t.bx, w4, h4, f.prev_segmap, f.b4_stride);
                if seg_id >= RAV1D_MAX_SEGMENTS.into() {
                    return Err(());
                }
                b.seg_id = seg_id;
            } else {
                b.seg_id = 0;
            }
            seg = Some(&frame_hdr.segmentation.seg_data.d[b.seg_id as usize]);
        } else if frame_hdr.segmentation.seg_data.preskip != 0 {
            if frame_hdr.segmentation.temporal != 0 && {
                let index = (*t.a).seg_pred.0[bx4 as usize] + t.l.seg_pred.0[by4 as usize];
                seg_pred = rav1d_msac_decode_bool_adapt(
                    &mut ts.msac,
                    &mut ts.cdf.m.seg_pred.0[index as usize],
                );
                seg_pred
            } {
                // temporal predicted seg_id
                if !(f.prev_segmap).is_null() {
                    let seg_id =
                        get_prev_frame_segid(f, t.by, t.bx, w4, h4, f.prev_segmap, f.b4_stride);
                    if seg_id >= RAV1D_MAX_SEGMENTS.into() {
                        return Err(());
                    }
                    b.seg_id = seg_id;
                } else {
                    b.seg_id = 0;
                }
            } else {
                let (pred_seg_id, seg_ctx) =
                    get_cur_frame_segid(t.by, t.bx, have_top, have_left, f.cur_segmap, f.b4_stride);
                let diff = rav1d_msac_decode_symbol_adapt8(
                    &mut ts.msac,
                    &mut ts.cdf.m.seg_id[seg_ctx as usize],
                    RAV1D_MAX_SEGMENTS as usize - 1,
                );
                let last_active_seg_id = frame_hdr.segmentation.seg_data.last_active_segid;
                b.seg_id =
                    neg_deinterleave(diff as c_int, pred_seg_id as c_int, last_active_seg_id + 1)
                        as u8;
                if b.seg_id as c_int > last_active_seg_id {
                    b.seg_id = 0; // error?
                }
                if b.seg_id >= RAV1D_MAX_SEGMENTS {
                    b.seg_id = 0; // error?
                }
            }

            if DEBUG_BLOCK_INFO(f, t) {
                println!("Post-segid[preskip;{}]: r={}", b.seg_id, ts.msac.rng);
            }

            seg = Some(&frame_hdr.segmentation.seg_data.d[b.seg_id as usize]);
        }
    } else {
        b.seg_id = 0;
    }

    // skip_mode
    if seg
        .map(|seg| seg.globalmv == 0 && seg.r#ref == -1 && seg.skip == 0)
        .unwrap_or(true)
        && frame_hdr.skip_mode.enabled != 0
        && cmp::min(bw4, bh4) > 1
    {
        let smctx = (*t.a).skip_mode.0[bx4 as usize] + t.l.skip_mode.0[by4 as usize];
        b.skip_mode =
            rav1d_msac_decode_bool_adapt(&mut ts.msac, &mut ts.cdf.m.skip_mode.0[smctx as usize])
                as u8;
        if DEBUG_BLOCK_INFO(f, t) {
            println!("Post-skipmode[{}]: r={}", b.skip_mode, ts.msac.rng);
        }
    } else {
        b.skip_mode = 0;
    }

    // skip
    if b.skip_mode != 0 || seg.map(|seg| seg.skip != 0).unwrap_or(false) {
        b.skip = 1;
    } else {
        let sctx = (*t.a).skip[bx4 as usize] + t.l.skip[by4 as usize];
        b.skip =
            rav1d_msac_decode_bool_adapt(&mut ts.msac, &mut ts.cdf.m.skip[sctx as usize]) as u8;
        if DEBUG_BLOCK_INFO(f, t) {
            println!("Post-skip[{}]: r={}", b.skip, ts.msac.rng);
        }
    }

    // segment_id
    if frame_hdr.segmentation.enabled != 0
        && frame_hdr.segmentation.update_map != 0
        && frame_hdr.segmentation.seg_data.preskip == 0
    {
        if b.skip == 0 && frame_hdr.segmentation.temporal != 0 && {
            let index = (*t.a).seg_pred.0[bx4 as usize] + t.l.seg_pred.0[by4 as usize];
            seg_pred = rav1d_msac_decode_bool_adapt(
                &mut ts.msac,
                &mut ts.cdf.m.seg_pred.0[index as usize],
            );
            seg_pred
        } {
            // temporal predicted seg_id
            if !(f.prev_segmap).is_null() {
                let seg_id =
                    get_prev_frame_segid(f, t.by, t.bx, w4, h4, f.prev_segmap, f.b4_stride);
                if seg_id >= RAV1D_MAX_SEGMENTS.into() {
                    return Err(());
                }
                b.seg_id = seg_id;
            } else {
                b.seg_id = 0;
            }
        } else {
            let (pred_seg_id, seg_ctx) =
                get_cur_frame_segid(t.by, t.bx, have_top, have_left, f.cur_segmap, f.b4_stride);
            if b.skip != 0 {
                b.seg_id = pred_seg_id as u8;
            } else {
                let diff = rav1d_msac_decode_symbol_adapt8(
                    &mut ts.msac,
                    &mut ts.cdf.m.seg_id[seg_ctx as usize],
                    RAV1D_MAX_SEGMENTS as usize - 1,
                );
                let last_active_seg_id = frame_hdr.segmentation.seg_data.last_active_segid;
                b.seg_id =
                    neg_deinterleave(diff as c_int, pred_seg_id as c_int, last_active_seg_id + 1)
                        as u8;
                if b.seg_id as i32 > last_active_seg_id {
                    b.seg_id = 0; // error?
                }
            }
            if b.seg_id >= RAV1D_MAX_SEGMENTS {
                b.seg_id = 0; // error?
            }
        }

        seg = Some(&frame_hdr.segmentation.seg_data.d[b.seg_id as usize]);

        if DEBUG_BLOCK_INFO(f, t) {
            println!("Post-segid[postskip;{}]: r={}", b.seg_id, ts.msac.rng);
        }
    }

    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    // cdef index
    if b.skip == 0 {
        let idx = if seq_hdr.sb128 != 0 {
            ((t.bx & 16) >> 4) + ((t.by & 16) >> 3)
        } else {
            0
        } as isize;
        if *(t.cur_sb_cdef_idx_ptr).offset(idx) == -1 {
            let v = rav1d_msac_decode_bools(&mut ts.msac, frame_hdr.cdef.n_bits as c_uint) as i8;
            *(t.cur_sb_cdef_idx_ptr).offset(idx) = v;
            if bw4 > 16 {
                *(t.cur_sb_cdef_idx_ptr).offset(idx + 1) = v;
            }
            if bh4 > 16 {
                *(t.cur_sb_cdef_idx_ptr).offset(idx + 2) = v;
            }
            if bw4 == 32 && bh4 == 32 {
                *(t.cur_sb_cdef_idx_ptr).offset(idx + 3) = v;
            }

            if DEBUG_BLOCK_INFO(f, t) {
                println!(
                    "Post-cdef_idx[{}]: r={}",
                    *t.cur_sb_cdef_idx_ptr, ts.msac.rng
                );
            }
        }
    }

    // delta-q/lf
    let not_sb128 = (seq_hdr.sb128 == 0) as c_int;
    if t.bx & (31 >> not_sb128) == 0 && t.by & (31 >> not_sb128) == 0 {
        let prev_qidx = ts.last_qidx;
        let have_delta_q = frame_hdr.delta.q.present != 0
            && (bs
                != (if seq_hdr.sb128 != 0 {
                    BS_128x128
                } else {
                    BS_64x64
                })
                || b.skip == 0);

        let prev_delta_lf = ts.last_delta_lf;

        if have_delta_q {
            let mut delta_q =
                rav1d_msac_decode_symbol_adapt4(&mut ts.msac, &mut ts.cdf.m.delta_q.0, 3) as c_int;
            if delta_q == 3 {
                let n_bits = 1 + rav1d_msac_decode_bools(&mut ts.msac, 3);
                delta_q =
                    (rav1d_msac_decode_bools(&mut ts.msac, n_bits) + 1 + (1 << n_bits)) as c_int;
            }
            if delta_q != 0 {
                if rav1d_msac_decode_bool_equi(&mut ts.msac) {
                    delta_q = -delta_q;
                }
                delta_q *= 1 << frame_hdr.delta.q.res_log2;
            }
            ts.last_qidx = iclip(ts.last_qidx + delta_q, 1, 255);
            if have_delta_q && DEBUG_BLOCK_INFO(f, t) {
                println!(
                    "Post-delta_q[{}->{}]: r={}",
                    delta_q, ts.last_qidx, ts.msac.rng
                );
            }

            if frame_hdr.delta.lf.present != 0 {
                let n_lfs = if frame_hdr.delta.lf.multi != 0 {
                    if f.cur.p.layout != Rav1dPixelLayout::I400 {
                        4
                    } else {
                        2
                    }
                } else {
                    1
                };

                for i in 0..n_lfs as usize {
                    let delta_lf_index = i + frame_hdr.delta.lf.multi as usize;
                    let mut delta_lf = rav1d_msac_decode_symbol_adapt4(
                        &mut ts.msac,
                        &mut ts.cdf.m.delta_lf[delta_lf_index],
                        3,
                    ) as c_int;
                    if delta_lf == 3 {
                        let n_bits = 1 + rav1d_msac_decode_bools(&mut ts.msac, 3);
                        delta_lf = (rav1d_msac_decode_bools(&mut ts.msac, n_bits)
                            + 1
                            + (1 << n_bits)) as c_int;
                    }
                    if delta_lf != 0 {
                        if rav1d_msac_decode_bool_equi(&mut ts.msac) {
                            delta_lf = -delta_lf;
                        }
                        delta_lf *= 1 << frame_hdr.delta.lf.res_log2;
                    }
                    ts.last_delta_lf[i] =
                        iclip(ts.last_delta_lf[i] as c_int + delta_lf, -63, 63) as i8;
                    if have_delta_q && DEBUG_BLOCK_INFO(f, t) {
                        println!("Post-delta_lf[{}:{}]: r={}", i, delta_lf, ts.msac.rng);
                    }
                }
            }
        }
        if ts.last_qidx == frame_hdr.quant.yac {
            // assign frame-wide q values to this sb
            ts.dq = f.dq.as_ptr();
        } else if ts.last_qidx != prev_qidx {
            // find sb-specific quant parameters
            init_quant_tables(seq_hdr, frame_hdr, ts.last_qidx, &mut ts.dqmem);
            ts.dq = ts.dqmem.as_ptr();
        }
        if ts.last_delta_lf == [0, 0, 0, 0] {
            // assign frame-wide lf values to this sb
            ts.lflvl = f.lf.lvl.as_ptr();
        } else if ts.last_delta_lf != prev_delta_lf {
            // find sb-specific lf lvl parameters
            rav1d_calc_lf_values(&mut ts.lflvlmem, frame_hdr, &ts.last_delta_lf);
            ts.lflvl = ts.lflvlmem.as_ptr();
        }
    }

    if b.skip_mode != 0 {
        b.intra = 0;
    } else if frame_hdr.frame_type.is_inter_or_switch() {
        if let Some(seg) = seg.filter(|seg| seg.r#ref >= 0 || seg.globalmv != 0) {
            b.intra = (seg.r#ref == 0) as u8;
        } else {
            let ictx = get_intra_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
            b.intra =
                (!rav1d_msac_decode_bool_adapt(&mut ts.msac, &mut ts.cdf.m.intra[ictx.into()]))
                    as u8;
            if DEBUG_BLOCK_INFO(f, t) {
                println!("Post-intra[{}]: r={}", b.intra, ts.msac.rng);
            }
        }
    } else if frame_hdr.allow_intrabc != 0 {
        b.intra = (!rav1d_msac_decode_bool_adapt(&mut ts.msac, &mut ts.cdf.m.intrabc.0)) as u8;
        if DEBUG_BLOCK_INFO(f, t) {
            println!("Post-intrabcflag[{}]: r={}", b.intra, ts.msac.rng);
        }
    } else {
        b.intra = 1;
    }

    // intra/inter-specific stuff
    if b.intra != 0 {
        let ymode_cdf = if frame_hdr.frame_type.is_inter_or_switch() {
            &mut ts.cdf.m.y_mode[dav1d_ymode_size_context[bs as usize] as usize]
        } else {
            &mut ts.cdf.kfym
                [dav1d_intra_mode_context[(*t.a).mode.0[bx4 as usize] as usize] as usize]
                [dav1d_intra_mode_context[t.l.mode.0[by4 as usize] as usize] as usize]
        };
        *b.y_mode_mut() = rav1d_msac_decode_symbol_adapt16(
            &mut ts.msac,
            ymode_cdf,
            (N_INTRA_PRED_MODES - 1) as usize,
        ) as u8;
        if DEBUG_BLOCK_INFO(f, t) {
            println!("Post-ymode[{}]: r={}", b.y_mode(), ts.msac.rng);
        }

        // angle delta
        if b_dim[2] + b_dim[3] >= 2 && b.y_mode() >= VERT_PRED && b.y_mode() <= VERT_LEFT_PRED {
            let acdf = &mut ts.cdf.m.angle_delta[b.y_mode() as usize - VERT_PRED as usize];
            let angle = rav1d_msac_decode_symbol_adapt8(&mut ts.msac, acdf, 6);
            *b.y_angle_mut() = angle as i8 - 3;
        } else {
            *b.y_angle_mut() = 0;
        }

        if has_chroma {
            let cfl_allowed = if frame_hdr.segmentation.lossless[b.seg_id as usize] != 0 {
                cbw4 == 1 && cbh4 == 1
            } else {
                (cfl_allowed_mask & (1 << bs)) != 0
            };
            let uvmode_cdf = &mut ts.cdf.m.uv_mode[cfl_allowed as usize][b.y_mode() as usize];
            *b.uv_mode_mut() = rav1d_msac_decode_symbol_adapt16(
                &mut ts.msac,
                uvmode_cdf,
                (N_UV_INTRA_PRED_MODES as usize) - 1 - (!cfl_allowed as usize),
            ) as u8;
            if DEBUG_BLOCK_INFO(f, t) {
                println!("Post-uvmode[{}]: r={}", b.uv_mode(), ts.msac.rng);
            }

            *b.uv_angle_mut() = 0;
            if b.uv_mode() == CFL_PRED {
                let sign =
                    rav1d_msac_decode_symbol_adapt8(&mut ts.msac, &mut ts.cdf.m.cfl_sign.0, 7) + 1;
                let sign_u = sign * 0x56 >> 8;
                let sign_v = sign - sign_u * 3;
                assert!(sign_u == sign / 3);
                if sign_u != 0 {
                    let ctx = (sign_u == 2) as usize * 3 + sign_v as usize;
                    b.cfl_alpha_mut()[0] = rav1d_msac_decode_symbol_adapt16(
                        &mut ts.msac,
                        &mut ts.cdf.m.cfl_alpha[ctx],
                        15,
                    ) as i8
                        + 1;
                    if sign_u == 1 {
                        b.cfl_alpha_mut()[0] = -b.cfl_alpha()[0];
                    }
                } else {
                    b.cfl_alpha_mut()[0] = 0;
                }
                if sign_v != 0 {
                    let ctx = (sign_v == 2) as usize * 3 + sign_u as usize;
                    b.cfl_alpha_mut()[1] = rav1d_msac_decode_symbol_adapt16(
                        &mut ts.msac,
                        &mut ts.cdf.m.cfl_alpha[ctx],
                        15,
                    ) as i8
                        + 1;
                    if sign_v == 1 {
                        b.cfl_alpha_mut()[1] = -b.cfl_alpha()[1];
                    }
                } else {
                    b.cfl_alpha_mut()[1] = 0;
                }
                if DEBUG_BLOCK_INFO(f, t) {
                    println!(
                        "Post-uvalphas[{}/{}]: r={}",
                        b.cfl_alpha()[0],
                        b.cfl_alpha()[1],
                        ts.msac.rng,
                    );
                }
            } else if b_dim[2] + b_dim[3] >= 2
                && b.uv_mode() >= VERT_PRED as u8
                && b.uv_mode() <= VERT_LEFT_PRED as u8
            {
                let acdf = &mut ts.cdf.m.angle_delta[b.uv_mode() as usize - VERT_PRED as usize];
                let angle = rav1d_msac_decode_symbol_adapt8(&mut ts.msac, acdf, 6) as c_int;
                *b.uv_angle_mut() = (angle - 3) as i8;
            }
        }

        *b.pal_sz_mut() = [0, 0];
        if frame_hdr.allow_screen_content_tools != 0 && cmp::max(bw4, bh4) <= 16 && bw4 + bh4 >= 4 {
            let sz_ctx = b_dim[2] + b_dim[3] - 2;
            if b.y_mode() == DC_PRED {
                let pal_ctx = ((*t.a).pal_sz.0[bx4 as usize] > 0) as usize
                    + (t.l.pal_sz.0[by4 as usize] > 0) as usize;
                let use_y_pal = rav1d_msac_decode_bool_adapt(
                    &mut ts.msac,
                    &mut ts.cdf.m.pal_y[sz_ctx as usize][pal_ctx],
                );
                if DEBUG_BLOCK_INFO(f, t) {
                    println!("Post-y_pal[{}]: r={}", use_y_pal, ts.msac.rng);
                }
                if use_y_pal {
                    read_pal_plane(t, b, false, sz_ctx, bx4 as usize, by4 as usize);
                }
            }

            if has_chroma && b.uv_mode() == DC_PRED {
                let pal_ctx = b.pal_sz()[0] > 0;
                let use_uv_pal = rav1d_msac_decode_bool_adapt(
                    &mut ts.msac,
                    &mut ts.cdf.m.pal_uv[pal_ctx as usize],
                );
                if DEBUG_BLOCK_INFO(f, t) {
                    println!("Post-uv_pal[{}]: r={}", use_uv_pal, ts.msac.rng);
                }
                if use_uv_pal {
                    // see aomedia bug 2183 for why we use luma coordinates
                    read_pal_uv(t, b, sz_ctx, bx4 as usize, by4 as usize);
                }
            }
        }

        if b.y_mode() == DC_PRED
            && b.pal_sz()[0] == 0
            && cmp::max(b_dim[2], b_dim[3]) <= 3
            && seq_hdr.filter_intra != 0
        {
            let is_filter = rav1d_msac_decode_bool_adapt(
                &mut ts.msac,
                &mut ts.cdf.m.use_filter_intra[bs as usize],
            );
            if is_filter {
                *b.y_mode_mut() = FILTER_PRED as u8;
                *b.y_angle_mut() =
                    rav1d_msac_decode_symbol_adapt4(&mut ts.msac, &mut ts.cdf.m.filter_intra.0, 4)
                        as i8;
            }
            if DEBUG_BLOCK_INFO(f, t) {
                println!(
                    "Post-filterintramode[{}/{}]: r={}",
                    b.y_mode(),
                    b.y_angle(),
                    ts.msac.rng,
                );
            }
        }

        if b.pal_sz()[0] != 0 {
            let pal_idx;
            if t.frame_thread.pass != 0 {
                let p = t.frame_thread.pass & 1;
                let frame_thread = &mut ts.frame_thread[p as usize];
                assert!(!frame_thread.pal_idx.is_null());
                let len = usize::try_from(bw4 * bh4 * 16).unwrap();
                pal_idx = std::slice::from_raw_parts_mut(frame_thread.pal_idx, len);
                frame_thread.pal_idx = frame_thread.pal_idx.offset(len as isize);
            } else {
                pal_idx = &mut t.scratch.c2rust_unnamed_0.pal_idx;
            }
            read_pal_indices(
                &mut *t.ts,
                &mut t.scratch.c2rust_unnamed_0.c2rust_unnamed.c2rust_unnamed,
                pal_idx,
                b,
                false,
                w4,
                h4,
                bw4,
                bh4,
            );
            if DEBUG_BLOCK_INFO(f, t) {
                println!("Post-y-pal-indices: r={}", ts.msac.rng);
            }
        }

        if has_chroma && b.pal_sz()[1] != 0 {
            let pal_idx;
            if t.frame_thread.pass != 0 {
                let p = t.frame_thread.pass & 1;
                let frame_thread = &mut ts.frame_thread[p as usize];
                assert!(!(frame_thread.pal_idx).is_null());
                let len = usize::try_from(cbw4 * cbh4 * 16).unwrap();
                pal_idx = std::slice::from_raw_parts_mut(frame_thread.pal_idx, len);
                frame_thread.pal_idx = frame_thread.pal_idx.offset(len as isize);
            } else {
                pal_idx = &mut t.scratch.c2rust_unnamed_0.pal_idx[(bw4 * bh4 * 16) as usize..];
            }
            read_pal_indices(
                &mut *t.ts,
                &mut t.scratch.c2rust_unnamed_0.c2rust_unnamed.c2rust_unnamed,
                pal_idx,
                b,
                true,
                cw4,
                ch4,
                cbw4,
                cbh4,
            );
            if DEBUG_BLOCK_INFO(f, t) {
                println!("Post-uv-pal-indices: r={}", ts.msac.rng);
            }
        }

        let t_dim = if frame_hdr.segmentation.lossless[b.seg_id as usize] != 0 {
            b.uvtx = TX_4X4 as u8;
            *b.tx_mut() = b.uvtx;
            &dav1d_txfm_dimensions[TX_4X4 as usize]
        } else {
            *b.tx_mut() = dav1d_max_txfm_size_for_bs[bs as usize][0];
            b.uvtx = dav1d_max_txfm_size_for_bs[bs as usize][f.cur.p.layout as usize];
            let mut t_dim = &dav1d_txfm_dimensions[b.tx() as usize];
            if frame_hdr.txfm_mode == RAV1D_TX_SWITCHABLE && t_dim.max > TX_4X4 as u8 {
                let tctx = get_tx_ctx(&*t.a, &t.l, &*t_dim, by4, bx4);
                let tx_cdf = &mut ts.cdf.m.txsz[(t_dim.max - 1) as usize][tctx as usize];
                let depth = rav1d_msac_decode_symbol_adapt4(
                    &mut ts.msac,
                    tx_cdf,
                    cmp::min(t_dim.max, 2) as usize,
                ) as c_int;

                for _ in 0..depth {
                    *b.tx_mut() = t_dim.sub;
                    t_dim = &dav1d_txfm_dimensions[b.tx() as usize];
                }
            }
            if DEBUG_BLOCK_INFO(f, t) {
                println!("Post-tx[{}]: r={}", b.tx(), ts.msac.rng);
            }
            t_dim
        };

        // reconstruction
        if t.frame_thread.pass == 1 {
            f.bd_fn.read_coef_blocks(t, bs, b);
        } else {
            f.bd_fn.recon_b_intra(t, bs, intra_edge_flags, b);
        }

        if frame_hdr.loopfilter.level_y != [0, 0] {
            rav1d_create_lf_mask_intra(
                &mut *t.lf_mask,
                f.lf.level,
                f.b4_stride,
                &*ts.lflvl.offset(b.seg_id as isize),
                t.bx,
                t.by,
                f.w4,
                f.h4,
                bs,
                b.tx() as RectTxfmSize,
                b.uvtx as RectTxfmSize,
                f.cur.p.layout,
                &mut (*t.a).tx_lpf_y.0[bx4 as usize..],
                &mut t.l.tx_lpf_y.0[by4 as usize..],
                if has_chroma {
                    Some((
                        &mut (*t.a).tx_lpf_uv.0[cbx4 as usize..],
                        &mut t.l.tx_lpf_uv.0[cby4 as usize..],
                    ))
                } else {
                    None
                },
            );
        }

        // update contexts
        let y_mode_nofilt = if b.y_mode() == FILTER_PRED {
            DC_PRED
        } else {
            b.y_mode()
        };
        CaseSet::<32, false>::many(
            [(&mut t.l, t_dim.lh, 1), (&mut *t.a, t_dim.lw, 0)],
            [bh4 as usize, bw4 as usize],
            [by4 as usize, bx4 as usize],
            |case, (dir, lw_lh, dir_index)| {
                case.set(&mut dir.tx_intra.0, lw_lh as i8);
                case.set(&mut dir.tx.0, lw_lh);
                case.set(&mut dir.mode.0, y_mode_nofilt);
                case.set(&mut dir.pal_sz.0, b.pal_sz()[0]);
                case.set(&mut dir.seg_pred.0, seg_pred.into());
                case.set(&mut dir.skip_mode.0, 0);
                case.set(&mut dir.intra.0, 1);
                case.set(&mut dir.skip.0, b.skip);
                // see aomedia bug 2183 for why we use luma coordinates here
                case.set(
                    &mut t.pal_sz_uv[dir_index],
                    if has_chroma { b.pal_sz()[1] } else { 0 },
                );
                if frame_hdr.frame_type.is_inter_or_switch() {
                    case.set(&mut dir.comp_type.0, COMP_INTER_NONE);
                    case.set(&mut dir.r#ref[0], -1);
                    case.set(&mut dir.r#ref[1], -1);
                    case.set(&mut dir.filter.0[0], RAV1D_N_SWITCHABLE_FILTERS as u8);
                    case.set(&mut dir.filter.0[1], RAV1D_N_SWITCHABLE_FILTERS as u8);
                }
            },
        );
        if b.pal_sz()[0] != 0 {
            let pal = if t.frame_thread.pass != 0 {
                let index = ((t.by >> 1) + (t.bx & 1)) as isize * (f.b4_stride >> 1)
                    + ((t.bx >> 1) + (t.by & 1)) as isize;
                &(*f.frame_thread.pal.offset(index))[0]
            } else {
                &t.scratch.c2rust_unnamed_0.pal[0]
            };
            for al_pal in &mut t.al_pal[0][bx4 as usize..][..bw4 as usize] {
                al_pal[0] = *pal;
            }
            for al_pal in &mut t.al_pal[1][by4 as usize..][..bh4 as usize] {
                al_pal[0] = *pal;
            }
        }
        if has_chroma {
            CaseSet::<32, false>::many(
                [&mut t.l, &mut *t.a],
                [cbh4 as usize, cbw4 as usize],
                [cby4 as usize, cbx4 as usize],
                |case, dir| {
                    case.set(&mut dir.uvmode.0, b.uv_mode());
                },
            );
            if b.pal_sz()[1] != 0 {
                let pal = if t.frame_thread.pass != 0 {
                    let index = ((t.by >> 1) + (t.bx & 1)) as isize * (f.b4_stride >> 1)
                        + ((t.bx >> 1) + (t.by & 1)) as isize;
                    &*f.frame_thread.pal.offset(index)
                } else {
                    &t.scratch.c2rust_unnamed_0.pal
                };
                // see aomedia bug 2183 for why we use luma coordinates here
                for pl in 1..=2 {
                    for x in 0..bw4 {
                        t.al_pal[0][(bx4 + x) as usize][pl] = pal[pl];
                    }
                    for y in 0..bh4 {
                        t.al_pal[1][(by4 + y) as usize][pl] = pal[pl];
                    }
                }
            }
        }
        if frame_hdr.frame_type.is_inter_or_switch() || frame_hdr.allow_intrabc != 0 {
            splat_intraref(&*f.c, t, bs, bw4 as usize, bh4 as usize);
        }
    } else if frame_hdr.frame_type.is_key_or_intra() {
        // intra block copy
        let mut mvstack = [Default::default(); 8];
        let mut n_mvs = 0;
        let mut ctx = 0;
        rav1d_refmvs_find(
            &mut t.rt,
            &mut mvstack,
            &mut n_mvs,
            &mut ctx,
            [0, -1].into(),
            bs,
            intra_edge_flags,
            t.by,
            t.bx,
        );

        if mvstack[0].mv.mv[0] != mv::ZERO {
            b.mv_mut()[0] = mvstack[0].mv.mv[0];
        } else if mvstack[1].mv.mv[0] != mv::ZERO {
            b.mv_mut()[0] = mvstack[1].mv.mv[0];
        } else if t.by - (16 << seq_hdr.sb128) < ts.tiling.row_start {
            b.mv_mut()[0].y = 0;
            b.mv_mut()[0].x = (-(512 << seq_hdr.sb128) - 2048) as i16;
        } else {
            b.mv_mut()[0].y = -(512 << seq_hdr.sb128) as i16;
            b.mv_mut()[0].x = 0;
        }

        let r#ref = b.mv()[0];
        read_mv_residual(t, &mut b.mv_mut()[0], &mut ts.cdf.dmv, false);

        // clip intrabc motion vector to decoded parts of current tile
        let mut border_left = ts.tiling.col_start * 4;
        let mut border_top = ts.tiling.row_start * 4;
        if has_chroma {
            if bw4 < 2 && ss_hor != 0 {
                border_left += 4;
            }
            if bh4 < 2 && ss_ver != 0 {
                border_top += 4;
            }
        }
        let mut src_left = t.bx * 4 + (b.mv()[0].x as c_int >> 3);
        let mut src_top = t.by * 4 + (b.mv()[0].y as c_int >> 3);
        let mut src_right = src_left + bw4 * 4;
        let mut src_bottom = src_top + bh4 * 4;
        let border_right = (ts.tiling.col_end + (bw4 - 1) & !(bw4 - 1)) * 4;

        // check against left or right tile boundary and adjust if necessary
        if src_left < border_left {
            src_right += border_left - src_left;
            src_left += border_left - src_left;
        } else if src_right > border_right {
            src_left -= src_right - border_right;
            src_right -= src_right - border_right;
        }
        // check against top tile boundary and adjust if necessary
        if src_top < border_top {
            src_bottom += border_top - src_top;
            src_top += border_top - src_top;
        }

        let sbx = t.bx >> 4 + seq_hdr.sb128 << 6 + seq_hdr.sb128;
        let sby = t.by >> 4 + seq_hdr.sb128 << 6 + seq_hdr.sb128;
        let sb_size = 1 << 6 + seq_hdr.sb128;
        // check for overlap with current superblock
        if src_bottom > sby && src_right > sbx {
            if src_top - border_top >= src_bottom - sby {
                // if possible move src up into the previous suberblock row
                src_top -= src_bottom - sby;
                src_bottom -= src_bottom - sby;
            } else if src_left - border_left >= src_right - sbx {
                // if possible move src left into the previous suberblock
                src_left -= src_right - sbx;
                src_right -= src_right - sbx;
            }
        }
        // move src up if it is below current superblock row
        if src_bottom > sby + sb_size {
            src_top -= src_bottom - (sby + sb_size);
            src_bottom -= src_bottom - (sby + sb_size);
        }
        // error out if mv still overlaps with the current superblock
        if src_bottom > sby && src_right > sbx {
            return Err(());
        }

        b.mv_mut()[0].x = ((src_left - t.bx * 4) * 8) as i16;
        b.mv_mut()[0].y = ((src_top - t.by * 4) * 8) as i16;

        if DEBUG_BLOCK_INFO(f, t) {
            println!(
                "Post-dmv[{}/{},ref={}/{}|{}/{}]: r={}",
                b.mv()[0].y,
                b.mv()[0].x,
                r#ref.y,
                r#ref.x,
                mvstack[0].mv.mv[0].y,
                mvstack[0].mv.mv[0].x,
                ts.msac.rng,
            );
        }
        read_vartx_tree(t, b, bs, bx4, by4);

        // reconstruction
        if t.frame_thread.pass == 1 {
            f.bd_fn.read_coef_blocks(t, bs, b);
            *b.filter2d_mut() = FILTER_2D_BILINEAR as u8;
        } else {
            f.bd_fn.recon_b_inter(t, bs, b)?;
        }

        splat_intrabc_mv(&*f.c, t, bs, b, bw4 as usize, bh4 as usize);

        CaseSet::<32, false>::many(
            [(&mut t.l, 1), (&mut *t.a, 0)],
            [bh4 as usize, bw4 as usize],
            [by4 as usize, bx4 as usize],
            |case, (dir, dir_index)| {
                case.set(&mut dir.tx_intra.0, b_dim[2 + dir_index] as i8);
                case.set(&mut dir.mode.0, DC_PRED);
                case.set(&mut dir.pal_sz.0, 0);
                // see aomedia bug 2183 for why this is outside `if has_chroma {}`
                case.set(&mut t.pal_sz_uv[dir_index], 0);
                case.set(&mut dir.seg_pred.0, seg_pred.into());
                case.set(&mut dir.skip_mode.0, 0);
                case.set(&mut dir.intra.0, 0);
                case.set(&mut dir.skip.0, b.skip);
            },
        );
        if has_chroma {
            CaseSet::<32, false>::many(
                [&mut t.l, &mut *t.a],
                [cbh4 as usize, cbw4 as usize],
                [cby4 as usize, cbx4 as usize],
                |case, dir| {
                    case.set(&mut dir.uvmode.0, DC_PRED);
                },
            );
        }
    } else {
        // inter-specific mode/mv coding
        let mut has_subpel_filter;

        let is_comp = if b.skip_mode != 0 {
            true
        } else if seg
            .map(|seg| seg.r#ref == -1 && seg.globalmv == 0 && seg.skip == 0)
            .unwrap_or(true)
            && frame_hdr.switchable_comp_refs != 0
            && cmp::min(bw4, bh4) > 1
        {
            let ctx = get_comp_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
            let is_comp =
                rav1d_msac_decode_bool_adapt(&mut ts.msac, &mut ts.cdf.m.comp[ctx as usize]);
            if DEBUG_BLOCK_INFO(f, t) {
                println!("Post-compflag[{}]: r={}", is_comp, ts.msac.rng);
            }
            is_comp
        } else {
            false
        };

        if b.skip_mode != 0 {
            *b.ref_mut() = [
                frame_hdr.skip_mode.refs[0] as i8,
                frame_hdr.skip_mode.refs[1] as i8,
            ];
            *b.comp_type_mut() = COMP_INTER_AVG;
            *b.inter_mode_mut() = NEARESTMV_NEARESTMV;
            *b.drl_idx_mut() = NEAREST_DRL;
            has_subpel_filter = false;

            let mut mvstack = [Default::default(); 8];
            let mut n_mvs = 0;
            let mut ctx = 0;
            rav1d_refmvs_find(
                &mut t.rt,
                &mut mvstack,
                &mut n_mvs,
                &mut ctx,
                [b.r#ref()[0] + 1, b.r#ref()[1] + 1].into(),
                bs,
                intra_edge_flags,
                t.by,
                t.bx,
            );

            *b.mv_mut() = mvstack[0].mv.mv;
            fix_mv_precision(frame_hdr, &mut b.mv_mut()[0]);
            fix_mv_precision(frame_hdr, &mut b.mv_mut()[1]);
            if DEBUG_BLOCK_INFO(f, t) {
                println!(
                    "Post-skipmodeblock[mv=1:y={},x={},2:y={},x={},refs={}+{}",
                    b.mv()[0].y,
                    b.mv()[0].x,
                    b.mv()[1].y,
                    b.mv()[1].x,
                    b.r#ref()[0],
                    b.r#ref()[1],
                );
            }
        } else if is_comp {
            let dir_ctx = get_comp_dir_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
            if rav1d_msac_decode_bool_adapt(&mut ts.msac, &mut ts.cdf.m.comp_dir[dir_ctx as usize])
            {
                // bidir - first reference (fw)
                let ctx1 = av1_get_fwd_ref_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                if rav1d_msac_decode_bool_adapt(
                    &mut ts.msac,
                    &mut ts.cdf.m.comp_fwd_ref[0][ctx1 as usize],
                ) {
                    let ctx2 = av1_get_fwd_ref_2_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                    b.ref_mut()[0] = 2 + rav1d_msac_decode_bool_adapt(
                        &mut ts.msac,
                        &mut ts.cdf.m.comp_fwd_ref[2][ctx2 as usize],
                    ) as i8;
                } else {
                    let ctx2 = av1_get_fwd_ref_1_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                    b.ref_mut()[0] = rav1d_msac_decode_bool_adapt(
                        &mut ts.msac,
                        &mut ts.cdf.m.comp_fwd_ref[1][ctx2 as usize],
                    ) as i8;
                }

                // second reference (bw)
                let ctx3 = av1_get_bwd_ref_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                if rav1d_msac_decode_bool_adapt(
                    &mut ts.msac,
                    &mut ts.cdf.m.comp_bwd_ref[0][ctx3 as usize],
                ) {
                    b.ref_mut()[1] = 6;
                } else {
                    let ctx4 = av1_get_bwd_ref_1_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                    b.ref_mut()[1] = 4 + rav1d_msac_decode_bool_adapt(
                        &mut ts.msac,
                        &mut ts.cdf.m.comp_bwd_ref[1][ctx4 as usize],
                    ) as i8;
                }
            } else {
                // unidir
                let uctx_p = av1_get_ref_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                if rav1d_msac_decode_bool_adapt(
                    &mut ts.msac,
                    &mut ts.cdf.m.comp_uni_ref[0][uctx_p as usize],
                ) {
                    *b.ref_mut() = [4, 6];
                } else {
                    let uctx_p1 = av1_get_uni_p1_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                    *b.ref_mut() = [
                        0,
                        1 + rav1d_msac_decode_bool_adapt(
                            &mut ts.msac,
                            &mut ts.cdf.m.comp_uni_ref[1][uctx_p1 as usize],
                        ) as i8,
                    ];

                    if b.r#ref()[1] == 2 {
                        let uctx_p2 =
                            av1_get_fwd_ref_2_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                        b.ref_mut()[1] += rav1d_msac_decode_bool_adapt(
                            &mut ts.msac,
                            &mut ts.cdf.m.comp_uni_ref[2][uctx_p2 as usize],
                        ) as i8;
                    }
                }
            }
            if DEBUG_BLOCK_INFO(f, t) {
                println!(
                    "Post-refs[{}/{}]: r={}",
                    b.r#ref()[0],
                    b.r#ref()[1],
                    ts.msac.rng,
                );
            }

            let mut mvstack = [Default::default(); 8];
            let mut n_mvs = 0;
            let mut ctx = 0;
            rav1d_refmvs_find(
                &mut t.rt,
                &mut mvstack,
                &mut n_mvs,
                &mut ctx,
                [b.r#ref()[0] + 1, b.r#ref()[1] + 1].into(),
                bs,
                intra_edge_flags,
                t.by,
                t.bx,
            );

            *b.inter_mode_mut() = rav1d_msac_decode_symbol_adapt8(
                &mut ts.msac,
                &mut ts.cdf.m.comp_inter_mode[ctx as usize],
                N_COMP_INTER_PRED_MODES as usize - 1,
            ) as u8;
            if DEBUG_BLOCK_INFO(f, t) {
                println!(
                    "Post-compintermode[{},ctx={},n_mvs={}]: r={}",
                    b.inter_mode(),
                    ctx,
                    n_mvs,
                    ts.msac.rng,
                );
            }

            let im = &dav1d_comp_inter_pred_modes[b.inter_mode() as usize];
            *b.drl_idx_mut() = NEAREST_DRL;
            if b.inter_mode() == NEWMV_NEWMV {
                if n_mvs > 1 {
                    // NEARER, NEAR or NEARISH
                    let drl_ctx_v1 = get_drl_context(&mvstack, 0);
                    *b.drl_idx_mut() += rav1d_msac_decode_bool_adapt(
                        &mut ts.msac,
                        &mut ts.cdf.m.drl_bit[drl_ctx_v1 as usize],
                    ) as u8;
                    if b.drl_idx() == NEARER_DRL && n_mvs > 2 {
                        let drl_ctx_v2 = get_drl_context(&mvstack, 1);
                        *b.drl_idx_mut() += rav1d_msac_decode_bool_adapt(
                            &mut ts.msac,
                            &mut ts.cdf.m.drl_bit[drl_ctx_v2 as usize],
                        ) as u8;
                    }
                    if DEBUG_BLOCK_INFO(f, t) {
                        println!(
                            "Post-drlidx[{},n_mvs={}]: r={}",
                            b.drl_idx(),
                            n_mvs,
                            ts.msac.rng,
                        );
                    }
                }
            } else if im[0] == NEARMV || im[1] == NEARMV {
                *b.drl_idx_mut() = NEARER_DRL;
                if n_mvs > 2 {
                    // NEAR or NEARISH
                    let drl_ctx_v2 = get_drl_context(&mvstack, 1);
                    *b.drl_idx_mut() += rav1d_msac_decode_bool_adapt(
                        &mut ts.msac,
                        &mut ts.cdf.m.drl_bit[drl_ctx_v2 as usize],
                    ) as u8;
                    if b.drl_idx() == NEAR_DRL && n_mvs > 3 {
                        let drl_ctx_v3 = get_drl_context(&mvstack, 2);
                        *b.drl_idx_mut() += rav1d_msac_decode_bool_adapt(
                            &mut ts.msac,
                            &mut ts.cdf.m.drl_bit[drl_ctx_v3 as usize],
                        ) as u8;
                    }
                    if DEBUG_BLOCK_INFO(f, t) {
                        println!(
                            "Post-drlidx[{},n_mvs={}]: r={}",
                            b.drl_idx(),
                            n_mvs,
                            ts.msac.rng,
                        );
                    }
                }
            }
            assert!(b.drl_idx() >= NEAREST_DRL && b.drl_idx() <= NEARISH_DRL);

            has_subpel_filter = cmp::min(bw4, bh4) == 1 || b.inter_mode() != GLOBALMV_GLOBALMV;
            let mut assign_comp_mv = |idx: usize| match im[idx] {
                NEARMV | NEARESTMV => {
                    b.mv_mut()[idx] = mvstack[b.drl_idx() as usize].mv.mv[idx];
                    fix_mv_precision(frame_hdr, &mut b.mv_mut()[idx]);
                }
                GLOBALMV => {
                    has_subpel_filter |=
                        frame_hdr.gmv[b.r#ref()[idx] as usize].r#type == RAV1D_WM_TYPE_TRANSLATION;
                    b.mv_mut()[idx] = get_gmv_2d(
                        &frame_hdr.gmv[b.r#ref()[idx] as usize],
                        t.bx,
                        t.by,
                        bw4,
                        bh4,
                        frame_hdr,
                    );
                }
                NEWMV => {
                    b.mv_mut()[idx] = mvstack[b.drl_idx() as usize].mv.mv[idx];
                    read_mv_residual(
                        t,
                        &mut b.mv_mut()[idx],
                        &mut ts.cdf.mv,
                        frame_hdr.force_integer_mv == 0,
                    );
                }
                _ => {}
            };
            assign_comp_mv(0);
            assign_comp_mv(1);
            if DEBUG_BLOCK_INFO(f, t) {
                println!(
                    "Post-residual_mv[1:y={},x={},2:y={},x={}]: r={}",
                    b.mv()[0].y,
                    b.mv()[0].x,
                    b.mv()[1].y,
                    b.mv()[1].x,
                    ts.msac.rng,
                );
            }

            // jnt_comp vs. seg vs. wedge
            let mut is_segwedge = false;
            if seq_hdr.masked_compound != 0 {
                let mask_ctx = get_mask_comp_ctx(&*t.a, &t.l, by4, bx4);
                is_segwedge = rav1d_msac_decode_bool_adapt(
                    &mut ts.msac,
                    &mut ts.cdf.m.mask_comp[mask_ctx as usize],
                );
                if DEBUG_BLOCK_INFO(f, t) {
                    println!(
                        "Post-segwedge_vs_jntavg[{},ctx={}]: r={}",
                        is_segwedge, mask_ctx, ts.msac.rng,
                    );
                }
            }

            if !is_segwedge {
                if seq_hdr.jnt_comp != 0 {
                    let [ref0poc, ref1poc] = b.r#ref().map(|r#ref| {
                        f.refp[r#ref as usize]
                            .p
                            .frame_hdr
                            .as_ref()
                            .unwrap()
                            .frame_offset as c_uint
                    });
                    let jnt_ctx = get_jnt_comp_ctx(
                        seq_hdr.order_hint_n_bits,
                        f.cur.frame_hdr.as_ref().unwrap().frame_offset as c_uint,
                        ref0poc,
                        ref1poc,
                        &*t.a,
                        &t.l,
                        by4,
                        bx4,
                    );
                    *b.comp_type_mut() = COMP_INTER_WEIGHTED_AVG
                        + rav1d_msac_decode_bool_adapt(
                            &mut ts.msac,
                            &mut ts.cdf.m.jnt_comp[jnt_ctx as usize],
                        ) as u8;
                    if DEBUG_BLOCK_INFO(f, t) {
                        println!(
                            "Post-jnt_comp[{},ctx={}[ac:{},ar:{},lc:{},lr:{}]]: r={}",
                            b.comp_type() == COMP_INTER_AVG,
                            jnt_ctx,
                            (*t.a).comp_type[bx4 as usize],
                            (*t.a).r#ref[0][bx4 as usize],
                            t.l.comp_type[by4 as usize],
                            t.l.r#ref[0][by4 as usize],
                            ts.msac.rng,
                        );
                    }
                } else {
                    *b.comp_type_mut() = COMP_INTER_AVG;
                }
            } else {
                if wedge_allowed_mask & (1 << bs) != 0 {
                    let ctx = dav1d_wedge_ctx_lut[bs as usize] as usize;
                    *b.comp_type_mut() = COMP_INTER_WEDGE
                        - rav1d_msac_decode_bool_adapt(&mut ts.msac, &mut ts.cdf.m.wedge_comp[ctx])
                            as u8;
                    if b.comp_type() == COMP_INTER_WEDGE {
                        *b.wedge_idx_mut() = rav1d_msac_decode_symbol_adapt16(
                            &mut ts.msac,
                            &mut ts.cdf.m.wedge_idx[ctx],
                            15,
                        ) as u8;
                    }
                } else {
                    *b.comp_type_mut() = COMP_INTER_SEG;
                }
                *b.mask_sign_mut() = rav1d_msac_decode_bool_equi(&mut ts.msac) as u8;
                if DEBUG_BLOCK_INFO(f, t) {
                    println!(
                        "Post-seg/wedge[{},wedge_idx={},sign={}]: r={}",
                        b.comp_type() == COMP_INTER_WEDGE,
                        b.wedge_idx(),
                        b.mask_sign(),
                        ts.msac.rng,
                    );
                }
            }
        } else {
            *b.comp_type_mut() = COMP_INTER_NONE;

            // ref
            if let Some(seg) = seg.filter(|seg| seg.r#ref > 0) {
                b.ref_mut()[0] = seg.r#ref as i8 - 1;
            } else if let Some(_) = seg.filter(|seg| seg.globalmv != 0 || seg.skip != 0) {
                b.ref_mut()[0] = 0;
            } else {
                let ctx1 = av1_get_ref_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                if rav1d_msac_decode_bool_adapt(&mut ts.msac, &mut ts.cdf.m.r#ref[0][ctx1 as usize])
                {
                    let ctx2 = av1_get_bwd_ref_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                    if rav1d_msac_decode_bool_adapt(
                        &mut ts.msac,
                        &mut ts.cdf.m.r#ref[1][ctx2 as usize],
                    ) {
                        b.ref_mut()[0] = 6;
                    } else {
                        let ctx3 =
                            av1_get_bwd_ref_1_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                        b.ref_mut()[0] = 4 + rav1d_msac_decode_bool_adapt(
                            &mut ts.msac,
                            &mut ts.cdf.m.r#ref[5][ctx3 as usize],
                        ) as i8;
                    }
                } else {
                    let ctx2 = av1_get_fwd_ref_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                    if rav1d_msac_decode_bool_adapt(
                        &mut ts.msac,
                        &mut ts.cdf.m.r#ref[2][ctx2 as usize],
                    ) {
                        let ctx3 =
                            av1_get_fwd_ref_2_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                        b.ref_mut()[0] = 2 + rav1d_msac_decode_bool_adapt(
                            &mut ts.msac,
                            &mut ts.cdf.m.r#ref[4][ctx3 as usize],
                        ) as i8;
                    } else {
                        let ctx3 =
                            av1_get_fwd_ref_1_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                        b.ref_mut()[0] = rav1d_msac_decode_bool_adapt(
                            &mut ts.msac,
                            &mut ts.cdf.m.r#ref[3][ctx3 as usize],
                        ) as i8;
                    }
                }
                if DEBUG_BLOCK_INFO(f, t) {
                    println!("Post-ref[{}]: r={}", b.r#ref()[0], ts.msac.rng);
                }
            }
            b.ref_mut()[1] = -1;

            let mut mvstack = [Default::default(); 8];
            let mut n_mvs = 0;
            let mut ctx = 0;
            rav1d_refmvs_find(
                &mut t.rt,
                &mut mvstack,
                &mut n_mvs,
                &mut ctx,
                refmvs_refpair {
                    r#ref: [b.r#ref()[0] + 1, -1],
                },
                bs,
                intra_edge_flags,
                t.by,
                t.bx,
            );

            // mode parsing and mv derivation from ref_mvs
            if seg
                .map(|seg| seg.skip != 0 || seg.globalmv != 0)
                .unwrap_or(false)
                || rav1d_msac_decode_bool_adapt(
                    &mut ts.msac,
                    &mut ts.cdf.m.newmv_mode[(ctx & 7) as usize],
                )
            {
                if seg
                    .map(|seg| seg.skip != 0 || seg.globalmv != 0)
                    .unwrap_or(false)
                    || !rav1d_msac_decode_bool_adapt(
                        &mut ts.msac,
                        &mut ts.cdf.m.globalmv_mode[(ctx >> 3 & 1) as usize],
                    )
                {
                    *b.inter_mode_mut() = GLOBALMV;
                    b.mv_mut()[0] = get_gmv_2d(
                        &frame_hdr.gmv[b.r#ref()[0] as usize],
                        t.bx,
                        t.by,
                        bw4,
                        bh4,
                        frame_hdr,
                    );
                    has_subpel_filter = cmp::min(bw4, bh4) == 1
                        || frame_hdr.gmv[b.r#ref()[0] as usize].r#type == RAV1D_WM_TYPE_TRANSLATION;
                } else {
                    has_subpel_filter = true;
                    if rav1d_msac_decode_bool_adapt(
                        &mut ts.msac,
                        &mut ts.cdf.m.refmv_mode[(ctx >> 4 & 15) as usize],
                    ) {
                        // NEAREST, NEARER, NEAR or NEARISH
                        *b.inter_mode_mut() = NEARMV;
                        *b.drl_idx_mut() = NEARER_DRL;
                        if n_mvs > 2 {
                            // NEARER, NEAR or NEARISH
                            let drl_ctx_v2 = get_drl_context(&mvstack, 1);
                            *b.drl_idx_mut() = b.drl_idx()
                                + rav1d_msac_decode_bool_adapt(
                                    &mut ts.msac,
                                    &mut ts.cdf.m.drl_bit[drl_ctx_v2 as usize],
                                ) as u8;
                            if b.drl_idx() == NEAR_DRL && n_mvs > 3 {
                                // NEAR or NEARISH
                                let drl_ctx_v3 = get_drl_context(&mvstack, 2);
                                *b.drl_idx_mut() = b.drl_idx()
                                    + rav1d_msac_decode_bool_adapt(
                                        &mut ts.msac,
                                        &mut ts.cdf.m.drl_bit[drl_ctx_v3 as usize],
                                    ) as u8;
                            }
                        }
                    } else {
                        *b.inter_mode_mut() = NEARESTMV as u8;
                        *b.drl_idx_mut() = NEAREST_DRL;
                    }
                    assert!(b.drl_idx() >= NEAREST_DRL && b.drl_idx() <= NEARISH_DRL);
                    b.mv_mut()[0] = mvstack[b.drl_idx() as usize].mv.mv[0];
                    if b.drl_idx() < NEAR_DRL {
                        fix_mv_precision(frame_hdr, &mut b.mv_mut()[0]);
                    }
                }

                if DEBUG_BLOCK_INFO(f, t) {
                    println!(
                        "Post-intermode[{},drl={},mv=y:{},x:{},n_mvs={}]: r={}",
                        b.inter_mode(),
                        b.drl_idx(),
                        b.mv()[0].y,
                        b.mv()[0].x,
                        n_mvs,
                        ts.msac.rng,
                    );
                }
            } else {
                has_subpel_filter = true;
                *b.inter_mode_mut() = NEWMV;
                *b.drl_idx_mut() = NEAREST_DRL;
                if n_mvs > 1 {
                    // NEARER, NEAR or NEARISH
                    let drl_ctx_v1 = get_drl_context(&mvstack, 0);
                    *b.drl_idx_mut() = b.drl_idx()
                        + rav1d_msac_decode_bool_adapt(
                            &mut ts.msac,
                            &mut ts.cdf.m.drl_bit[drl_ctx_v1 as usize],
                        ) as u8;
                    if b.drl_idx() == NEARER_DRL && n_mvs > 2 {
                        // NEAR or NEARISH
                        let drl_ctx_v2 = get_drl_context(&mvstack, 1);
                        *b.drl_idx_mut() = b.drl_idx()
                            + rav1d_msac_decode_bool_adapt(
                                &mut ts.msac,
                                &mut ts.cdf.m.drl_bit[drl_ctx_v2 as usize],
                            ) as u8;
                    }
                }
                assert!(b.drl_idx() >= NEAREST_DRL && b.drl_idx() <= NEARISH_DRL);
                if n_mvs > 1 {
                    b.mv_mut()[0] = mvstack[b.drl_idx() as usize].mv.mv[0];
                } else {
                    assert!(b.drl_idx() == 0);
                    b.mv_mut()[0] = mvstack[0].mv.mv[0];
                    fix_mv_precision(frame_hdr, &mut b.mv_mut()[0]);
                }
                if DEBUG_BLOCK_INFO(f, t) {
                    println!(
                        "Post-intermode[{},drl={}]: r={}",
                        b.inter_mode(),
                        b.drl_idx(),
                        ts.msac.rng,
                    );
                }
                read_mv_residual(
                    t,
                    &mut *b.mv_mut().as_mut_ptr().offset(0),
                    &mut ts.cdf.mv,
                    frame_hdr.force_integer_mv == 0,
                );
                if DEBUG_BLOCK_INFO(f, t) {
                    println!(
                        "Post-residualmv[mv=y:{},x:{}]: r={}",
                        b.mv()[0].y,
                        b.mv()[0].x,
                        ts.msac.rng,
                    );
                }
            }

            // interintra flags
            let ii_sz_grp = dav1d_ymode_size_context[bs as usize] as c_int;
            if seq_hdr.inter_intra != 0
                && interintra_allowed_mask & (1 << bs) != 0
                && rav1d_msac_decode_bool_adapt(
                    &mut ts.msac,
                    &mut ts.cdf.m.interintra[ii_sz_grp as usize],
                )
            {
                *b.interintra_mode_mut() = rav1d_msac_decode_symbol_adapt4(
                    &mut ts.msac,
                    &mut ts.cdf.m.interintra_mode[ii_sz_grp as usize],
                    N_INTER_INTRA_PRED_MODES as usize - 1,
                ) as u8;
                let wedge_ctx = dav1d_wedge_ctx_lut[bs as usize] as c_int;
                *b.interintra_type_mut() = INTER_INTRA_BLEND
                    + rav1d_msac_decode_bool_adapt(
                        &mut ts.msac,
                        &mut ts.cdf.m.interintra_wedge[wedge_ctx as usize],
                    ) as u8;
                if b.interintra_type() == INTER_INTRA_WEDGE {
                    *b.wedge_idx_mut() = rav1d_msac_decode_symbol_adapt16(
                        &mut ts.msac,
                        &mut ts.cdf.m.wedge_idx[wedge_ctx as usize],
                        15,
                    ) as u8;
                }
            } else {
                *b.interintra_type_mut() = INTER_INTRA_NONE;
            }
            if DEBUG_BLOCK_INFO(f, t)
                && seq_hdr.inter_intra != 0
                && interintra_allowed_mask & (1 << bs) != 0
            {
                println!(
                    "Post-interintra[t={},m={},w={}]: r={}",
                    b.interintra_type(),
                    b.interintra_mode(),
                    b.wedge_idx(),
                    ts.msac.rng,
                );
            }

            // motion variation
            if frame_hdr.switchable_motion_mode != 0
                && b.interintra_type() == INTER_INTRA_NONE
                && cmp::min(bw4, bh4) >= 2
                // is not warped global motion
                && !(frame_hdr.force_integer_mv == 0
                    && b.inter_mode() == GLOBALMV
                    && frame_hdr.gmv[b.r#ref()[0] as usize].r#type > RAV1D_WM_TYPE_TRANSLATION)
                // has overlappable neighbours
                && (have_left && findoddzero(&t.l.intra.0[by4 as usize..][..h4 as usize])
                    || have_top && findoddzero(&(*t.a).intra.0[bx4 as usize..][..w4 as usize]))
            {
                // reaching here means the block allows obmc - check warp by
                // finding matching-ref blocks in top/left edges
                let mut mask = [0, 0];
                find_matching_ref(
                    t,
                    intra_edge_flags,
                    bw4,
                    bh4,
                    w4,
                    h4,
                    have_left,
                    have_top,
                    b.r#ref()[0],
                    &mut mask,
                );
                let allow_warp = (f.svc[b.r#ref()[0] as usize][0].scale == 0
                    && frame_hdr.force_integer_mv == 0
                    && frame_hdr.warp_motion != 0
                    && mask[0] | mask[1] != 0) as c_int;

                *b.motion_mode_mut() = if allow_warp != 0 {
                    rav1d_msac_decode_symbol_adapt4(
                        &mut ts.msac,
                        &mut ts.cdf.m.motion_mode[bs as usize],
                        2,
                    ) as u8
                } else {
                    rav1d_msac_decode_bool_adapt(&mut ts.msac, &mut ts.cdf.m.obmc[bs as usize])
                        as u8
                };
                if b.motion_mode() == MM_WARP as u8 {
                    has_subpel_filter = false;
                    t.warpmv = derive_warpmv(t, bw4, bh4, &mask, b.mv()[0], t.warpmv.clone());
                    if DEBUG_BLOCK_INFO(f, t) {
                        println!(
                            "[ {} {} {}\n  {} {} {} ]\n\
                            alpha={}, beta={}, gamma={}, deta={}, mv=y:{},x:{}",
                            SignAbs(t.warpmv.matrix[0]),
                            SignAbs(t.warpmv.matrix[1]),
                            SignAbs(t.warpmv.matrix[2]),
                            SignAbs(t.warpmv.matrix[3]),
                            SignAbs(t.warpmv.matrix[4]),
                            SignAbs(t.warpmv.matrix[5]),
                            SignAbs(t.warpmv.alpha().into()),
                            SignAbs(t.warpmv.beta().into()),
                            SignAbs(t.warpmv.gamma().into()),
                            SignAbs(t.warpmv.delta().into()),
                            b.mv()[0].y,
                            b.mv()[0].x,
                        );
                    }
                    if t.frame_thread.pass != 0 {
                        if t.warpmv.r#type == RAV1D_WM_TYPE_AFFINE {
                            b.matrix_mut()[0] = (t.warpmv.matrix[2] - 0x10000) as i16;
                            b.matrix_mut()[1] = t.warpmv.matrix[3] as i16;
                            b.matrix_mut()[2] = t.warpmv.matrix[4] as i16;
                            b.matrix_mut()[3] = (t.warpmv.matrix[5] - 0x10000) as i16;
                        } else {
                            b.matrix_mut()[0] = i16::MIN;
                        }
                    }
                }

                if DEBUG_BLOCK_INFO(f, t) {
                    println!(
                        "Post-motionmode[{}]: r={} [mask: 0x{:x}/0x{:x}]",
                        b.motion_mode(),
                        ts.msac.rng,
                        mask[0],
                        mask[1],
                    );
                }
            } else {
                *b.motion_mode_mut() = MM_TRANSLATION as u8;
            }
        }

        // subpel filter
        let filter = if frame_hdr.subpel_filter_mode == RAV1D_FILTER_SWITCHABLE {
            if has_subpel_filter {
                let comp = b.comp_type() != COMP_INTER_NONE;
                let ctx1 = get_filter_ctx(&*t.a, &t.l, comp, false, b.r#ref()[0], by4, bx4);
                let filter0 = rav1d_msac_decode_symbol_adapt4(
                    &mut ts.msac,
                    &mut ts.cdf.m.filter.0[0][ctx1 as usize],
                    RAV1D_N_SWITCHABLE_FILTERS as usize - 1,
                ) as Dav1dFilterMode;
                if seq_hdr.dual_filter != 0 {
                    let ctx2 = get_filter_ctx(&*t.a, &t.l, comp, true, b.r#ref()[0], by4, bx4);
                    if DEBUG_BLOCK_INFO(f, t) {
                        println!(
                            "Post-subpel_filter1[{},ctx={}]: r={}",
                            filter0, ctx1, ts.msac.rng,
                        );
                    }
                    let filter1 = rav1d_msac_decode_symbol_adapt4(
                        &mut ts.msac,
                        &mut ts.cdf.m.filter.0[1][ctx2 as usize],
                        RAV1D_N_SWITCHABLE_FILTERS as usize - 1,
                    ) as Dav1dFilterMode;
                    if DEBUG_BLOCK_INFO(f, t) {
                        println!(
                            "Post-subpel_filter2[{},ctx={}]: r={}",
                            filter1, ctx2, ts.msac.rng,
                        );
                    }
                    [filter0, filter1]
                } else {
                    if DEBUG_BLOCK_INFO(f, t) {
                        println!(
                            "Post-subpel_filter[{},ctx={}]: r={}",
                            filter0, ctx1, ts.msac.rng
                        );
                    }
                    [filter0; 2]
                }
            } else {
                [RAV1D_FILTER_8TAP_REGULAR; 2]
            }
        } else {
            [frame_hdr.subpel_filter_mode; 2]
        };
        *b.filter2d_mut() = dav1d_filter_2d[filter[1] as usize][filter[0] as usize];

        read_vartx_tree(t, b, bs, bx4, by4);

        // reconstruction
        if t.frame_thread.pass == 1 {
            f.bd_fn.read_coef_blocks(t, bs, b);
        } else {
            f.bd_fn.recon_b_inter(t, bs, b)?;
        }

        if frame_hdr.loopfilter.level_y != [0, 0] {
            let is_globalmv =
                (b.inter_mode() == if is_comp { GLOBALMV_GLOBALMV } else { GLOBALMV }) as c_int;
            let tx_split = [b.tx_split0() as u16, b.tx_split1()];
            let mut ytx = b.max_ytx() as RectTxfmSize;
            let mut uvtx = b.uvtx as RectTxfmSize;
            if frame_hdr.segmentation.lossless[b.seg_id as usize] != 0 {
                ytx = TX_4X4 as RectTxfmSize;
                uvtx = TX_4X4 as RectTxfmSize;
            }
            rav1d_create_lf_mask_inter(
                &mut *t.lf_mask,
                f.lf.level,
                f.b4_stride,
                // In C, the inner dimensions (`ref`, `is_gmv`) are offset,
                // but then cast back to a pointer to the full array,
                // even though the whole array is not passed.
                // Dereferencing this in Rust is UB, so instead
                // we pass the indices as args, which are then applied at the use sites.
                &*ts.lflvl.offset(b.seg_id as isize),
                (b.r#ref()[0] + 1) as usize,
                is_globalmv == 0,
                t.bx,
                t.by,
                f.w4,
                f.h4,
                b.skip != 0,
                bs,
                ytx,
                &tx_split,
                uvtx,
                f.cur.p.layout,
                &mut (*t.a).tx_lpf_y.0[bx4 as usize..],
                &mut t.l.tx_lpf_y.0[by4 as usize..],
                if has_chroma {
                    Some((
                        &mut (*t.a).tx_lpf_uv.0[cbx4 as usize..],
                        &mut t.l.tx_lpf_uv.0[cby4 as usize..],
                    ))
                } else {
                    None
                },
            );
        }

        // context updates
        if is_comp {
            splat_tworef_mv(&*f.c, t, bs, b, bw4 as usize, bh4 as usize);
        } else {
            splat_oneref_mv(&*f.c, t, bs, b, bw4 as usize, bh4 as usize);
        }

        CaseSet::<32, false>::many(
            [(&mut t.l, 1), (&mut *t.a, 0)],
            [bh4 as usize, bw4 as usize],
            [by4 as usize, bx4 as usize],
            |case, (dir, dir_index)| {
                case.set(&mut dir.seg_pred.0, seg_pred.into());
                case.set(&mut dir.skip_mode.0, b.skip_mode);
                case.set(&mut dir.intra.0, 0);
                case.set(&mut dir.skip.0, b.skip);
                case.set(&mut dir.pal_sz.0, 0);
                // see aomedia bug 2183 for why this is outside if (has_chroma)
                case.set(&mut t.pal_sz_uv[dir_index], 0);
                case.set(&mut dir.tx_intra.0, b_dim[2 + dir_index] as i8);
                case.set(&mut dir.comp_type.0, b.comp_type());
                case.set(&mut dir.filter.0[0], filter[0]);
                case.set(&mut dir.filter.0[1], filter[1]);
                case.set(&mut dir.mode.0, b.inter_mode());
                case.set(&mut dir.r#ref.0[0], b.r#ref()[0]);
                case.set(&mut dir.r#ref.0[1], b.r#ref()[1]);
            },
        );

        if has_chroma {
            CaseSet::<32, false>::many(
                [&mut t.l, &mut *t.a],
                [cbh4 as usize, cbw4 as usize],
                [cby4 as usize, cbx4 as usize],
                |case, dir| {
                    case.set(&mut dir.uvmode.0, DC_PRED);
                },
            );
        }
    }

    // update contexts
    if frame_hdr.segmentation.enabled != 0 && frame_hdr.segmentation.update_map != 0 {
        // Need checked casts here because we're using `from_raw_parts_mut` and an overflow would be UB.
        let [by, bx, bh4, bw4] = [t.by, t.bx, bh4, bw4].map(|it| usize::try_from(it).unwrap());
        let b4_stride = usize::try_from(f.b4_stride).unwrap();
        let cur_segmap_len = (by * b4_stride + bx)
            + if bh4 == 0 {
                0
            } else {
                (b4_stride * (bh4 - 1)) + bw4
            };
        let cur_segmap = std::slice::from_raw_parts_mut(f.cur_segmap, cur_segmap_len);
        let seg_ptr = &mut cur_segmap[by * b4_stride + bx..];

        CaseSet::<32, false>::one((), bw4, 0, |case, ()| {
            for seg_ptr in seg_ptr.chunks_mut(b4_stride).take(bh4) {
                case.set(seg_ptr, b.seg_id);
            }
        });
    }
    if b.skip == 0 {
        let mask = !0u32 >> 32 - bw4 << (bx4 & 15);
        let bx_idx = (bx4 & 16) >> 4;
        for noskip_mask in
            &mut (*t.lf_mask).noskip_mask[by4 as usize >> 1..][..(bh4 as usize + 1) / 2]
        {
            noskip_mask[bx_idx as usize] |= mask as u16;
            if bw4 == 32 {
                // this should be mask >> 16, but it's 0xffffffff anyway
                noskip_mask[1] |= mask as u16;
            }
        }
    }

    if t.frame_thread.pass == 1 && b.intra == 0 && frame_hdr.frame_type.is_inter_or_switch() {
        let sby = t.by - ts.tiling.row_start >> f.sb_shift;
        let lowest_px = &mut *ts.lowest_pixel.offset(sby as isize);
        // keep track of motion vectors for each reference
        if b.comp_type() == COMP_INTER_NONE {
            // y
            if cmp::min(bw4, bh4) > 1
                && (b.inter_mode() == GLOBALMV && f.gmv_warp_allowed[b.r#ref()[0] as usize] != 0
                    || b.motion_mode() == MM_WARP as u8
                        && t.warpmv.r#type > RAV1D_WM_TYPE_TRANSLATION)
            {
                affine_lowest_px_luma(
                    t,
                    &mut lowest_px[b.r#ref()[0] as usize][0],
                    b_dim,
                    if b.motion_mode() == MM_WARP as u8 {
                        &t.warpmv
                    } else {
                        &frame_hdr.gmv[b.r#ref()[0] as usize]
                    },
                );
            } else {
                mc_lowest_px(
                    &mut lowest_px[b.r#ref()[0] as usize][0],
                    t.by,
                    bh4,
                    b.mv()[0].y,
                    0,
                    &f.svc[b.r#ref()[0] as usize][1],
                );
                if b.motion_mode() == MM_OBMC as u8 {
                    obmc_lowest_px(t, lowest_px, false, b_dim, bx4, by4, w4, h4);
                }
            }

            // uv
            if has_chroma {
                // sub8x8 derivation
                let mut is_sub8x8 = bw4 == ss_hor || bh4 == ss_ver;
                let mut r = 0 as *const *mut refmvs_block;
                if is_sub8x8 {
                    assert!(ss_hor == 1);
                    r = &mut *(t.rt.r).as_mut_ptr().offset(((t.by & 31) + 5) as isize)
                        as *mut *mut refmvs_block;
                    if bw4 == 1 {
                        is_sub8x8 &=
                            (*(*r.offset(0)).offset((t.bx - 1) as isize)).0.r#ref.r#ref[0] > 0;
                    }
                    if bh4 == ss_ver {
                        is_sub8x8 &= (*(*r.offset(-1)).offset(t.bx as isize)).0.r#ref.r#ref[0] > 0;
                    }
                    if bw4 == 1 && bh4 == ss_ver {
                        is_sub8x8 &=
                            (*(*r.offset(-1)).offset((t.bx - 1) as isize)).0.r#ref.r#ref[0] > 0;
                    }
                }

                // chroma prediction
                if is_sub8x8 {
                    assert!(ss_hor == 1);
                    if bw4 == 1 && bh4 == ss_ver {
                        let rr = &mut *(*r.offset(-1)).offset((t.bx - 1) as isize)
                            as *const refmvs_block;
                        mc_lowest_px(
                            &mut lowest_px[(*rr).0.r#ref.r#ref[0] as usize - 1][1],
                            t.by - 1,
                            bh4,
                            (*rr).0.mv.mv[0].y,
                            ss_ver,
                            &f.svc[(*rr).0.r#ref.r#ref[0] as usize - 1][1],
                        );
                    }
                    if bw4 == 1 {
                        let rr =
                            &mut *(*r.offset(0)).offset((t.bx - 1) as isize) as *const refmvs_block;
                        mc_lowest_px(
                            &mut lowest_px[(*rr).0.r#ref.r#ref[0] as usize - 1][1],
                            t.by,
                            bh4,
                            (*rr).0.mv.mv[0].y,
                            ss_ver,
                            &f.svc[(*rr).0.r#ref.r#ref[0] as usize - 1][1],
                        );
                    }
                    if bh4 == ss_ver {
                        let rr = &mut *(*r.offset(-1)).offset(t.bx as isize) as *const refmvs_block;
                        mc_lowest_px(
                            &mut lowest_px[(*rr).0.r#ref.r#ref[0] as usize - 1][1],
                            t.by - 1,
                            bh4,
                            (*rr).0.mv.mv[0].y,
                            ss_ver,
                            &f.svc[(*rr).0.r#ref.r#ref[0] as usize - 1][1],
                        );
                    }
                    mc_lowest_px(
                        &mut lowest_px[b.r#ref()[0] as usize][1],
                        t.by,
                        bh4,
                        b.mv()[0].y,
                        ss_ver,
                        &f.svc[b.r#ref()[0] as usize][1],
                    );
                } else if cmp::min(cbw4, cbh4) > 1
                    && (b.inter_mode() == GLOBALMV
                        && f.gmv_warp_allowed[b.r#ref()[0] as usize] != 0
                        || b.motion_mode() == MM_WARP as u8
                            && t.warpmv.r#type > RAV1D_WM_TYPE_TRANSLATION)
                {
                    affine_lowest_px_chroma(
                        t,
                        &mut lowest_px[b.r#ref()[0] as usize][1],
                        b_dim,
                        if b.motion_mode() == MM_WARP as u8 {
                            &t.warpmv
                        } else {
                            &frame_hdr.gmv[b.r#ref()[0] as usize]
                        },
                    );
                } else {
                    mc_lowest_px(
                        &mut lowest_px[b.r#ref()[0] as usize][1],
                        t.by & !ss_ver,
                        bh4 << (bh4 == ss_ver) as c_int,
                        b.mv()[0].y,
                        ss_ver,
                        &f.svc[b.r#ref()[0] as usize][1],
                    );
                    if b.motion_mode() == MM_OBMC as u8 {
                        obmc_lowest_px(t, lowest_px, true, b_dim, bx4, by4, w4, h4);
                    }
                }
            }
        } else {
            // y
            let refmvs =
                || std::iter::zip(b.r#ref(), b.mv()).map(|(r#ref, mv)| (r#ref as usize, mv));
            for (r#ref, mv) in refmvs() {
                if b.inter_mode() == GLOBALMV_GLOBALMV && f.gmv_warp_allowed[r#ref] != 0 {
                    affine_lowest_px_luma(
                        t,
                        &mut lowest_px[r#ref][0],
                        b_dim,
                        &frame_hdr.gmv[r#ref],
                    );
                } else {
                    mc_lowest_px(
                        &mut lowest_px[r#ref][0],
                        t.by,
                        bh4,
                        mv.y,
                        0,
                        &f.svc[r#ref][1],
                    );
                }
            }
            for (r#ref, mv) in refmvs() {
                if b.inter_mode() == GLOBALMV_GLOBALMV && f.gmv_warp_allowed[r#ref] != 0 {
                    affine_lowest_px_luma(
                        t,
                        &mut lowest_px[r#ref][0],
                        b_dim,
                        &frame_hdr.gmv[r#ref],
                    );
                } else {
                    mc_lowest_px(
                        &mut lowest_px[r#ref][0],
                        t.by,
                        bh4,
                        mv.y,
                        0,
                        &f.svc[r#ref][1],
                    );
                }
            }

            // uv
            if has_chroma {
                for (r#ref, mv) in refmvs() {
                    if b.inter_mode() == GLOBALMV_GLOBALMV
                        && cmp::min(cbw4, cbh4) > 1
                        && f.gmv_warp_allowed[r#ref] != 0
                    {
                        affine_lowest_px_chroma(
                            t,
                            &mut lowest_px[r#ref][1],
                            b_dim,
                            &frame_hdr.gmv[r#ref],
                        );
                    } else {
                        mc_lowest_px(
                            &mut lowest_px[r#ref][1],
                            t.by,
                            bh4,
                            mv.y,
                            ss_ver,
                            &f.svc[r#ref][1],
                        );
                    }
                }
            }
        }
    }

    Ok(())
}

unsafe fn decode_sb(
    t: &mut Rav1dTaskContext,
    bl: BlockLevel,
    node: *const EdgeNode,
) -> Result<(), ()> {
    let f = &*t.f;
    let ts = &mut *t.ts;
    let hsz = 16 >> bl;
    let have_h_split = f.bw > t.bx + hsz;
    let have_v_split = f.bh > t.by + hsz;

    if !have_h_split && !have_v_split {
        assert!(bl < BL_8X8);
        return decode_sb(t, bl + 1, (*(node as *const EdgeBranch)).split[0]);
    }

    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();

    let bp;
    let mut ctx = 0;
    let mut bx8 = 0;
    let mut by8 = 0;
    let pc = if t.frame_thread.pass == 2 {
        None
    } else {
        if false && bl == BL_64X64 {
            println!(
                "poc={},y={},x={},bl={},r={}",
                frame_hdr.frame_offset, t.by, t.bx, bl, ts.msac.rng,
            );
        }
        bx8 = (t.bx & 31) >> 1;
        by8 = (t.by & 31) >> 1;
        ctx = get_partition_ctx(&*t.a, &t.l, bl, by8, bx8);
        Some(&mut ts.cdf.m.partition[bl as usize][ctx as usize])
    };

    if have_h_split && have_v_split {
        if let Some(pc) = pc {
            bp = rav1d_msac_decode_symbol_adapt16(
                &mut ts.msac,
                pc,
                dav1d_partition_type_count[bl as usize].into(),
            ) as BlockPartition;
            if f.cur.p.layout == Rav1dPixelLayout::I422
                && (bp == PARTITION_V
                    || bp == PARTITION_V4
                    || bp == PARTITION_T_LEFT_SPLIT
                    || bp == PARTITION_T_RIGHT_SPLIT)
            {
                return Err(());
            }
            if DEBUG_BLOCK_INFO(f, t) {
                println!(
                    "poc={},y={},x={},bl={},ctx={},bp={}: r={}",
                    frame_hdr.frame_offset, t.by, t.bx, bl, ctx, bp, ts.msac.rng,
                );
            }
        } else {
            let b = &mut *(f.frame_thread.b).offset(t.by as isize * f.b4_stride + t.bx as isize);
            bp = if b.bl == bl { b.bp } else { PARTITION_SPLIT };
        }
        let b = &dav1d_block_sizes[bl as usize][bp as usize];

        match bp {
            PARTITION_NONE => {
                let node = &*node;
                decode_b(t, bl, b[0], bp, node.o)?;
            }
            PARTITION_H => {
                let node = &*node;
                decode_b(t, bl, b[0], bp, node.h[0])?;
                t.by += hsz;
                decode_b(t, bl, b[0], bp, node.h[1])?;
                t.by -= hsz;
            }
            PARTITION_V => {
                let node = &*node;
                decode_b(t, bl, b[0], bp, node.v[0])?;
                t.bx += hsz;
                decode_b(t, bl, b[0], bp, node.v[1])?;
                t.bx -= hsz;
            }
            PARTITION_SPLIT => {
                if bl == BL_8X8 {
                    let tip = &*(node as *const EdgeTip);
                    assert!(hsz == 1);
                    decode_b(t, bl, BS_4x4, bp, tip.split[0])?;
                    let tl_filter = t.tl_4x4_filter;
                    t.bx += 1;
                    decode_b(t, bl, BS_4x4, bp, tip.split[1])?;
                    t.bx -= 1;
                    t.by += 1;
                    decode_b(t, bl, BS_4x4, bp, tip.split[2])?;
                    t.bx += 1;
                    t.tl_4x4_filter = tl_filter;
                    decode_b(t, bl, BS_4x4, bp, tip.split[3])?;
                    t.bx -= 1;
                    t.by -= 1;
                    if cfg!(target_arch = "x86_64") && t.frame_thread.pass != 0 {
                        // In 8-bit mode with 2-pass decoding the coefficient buffer
                        // can end up misaligned due to skips here.
                        // Work around the issue by explicitly realigning the buffer.
                        let p = (t.frame_thread.pass & 1) as usize;
                        ts.frame_thread[p].cf =
                            (((ts.frame_thread[p].cf as uintptr_t) + 63) & !63) as *mut DynCoef;
                    }
                } else {
                    let branch = &*(node as *const EdgeBranch);
                    decode_sb(t, bl + 1, branch.split[0])?;
                    t.bx += hsz;
                    decode_sb(t, bl + 1, branch.split[1])?;
                    t.bx -= hsz;
                    t.by += hsz;
                    decode_sb(t, bl + 1, branch.split[2])?;
                    t.bx += hsz;
                    decode_sb(t, bl + 1, branch.split[3])?;
                    t.bx -= hsz;
                    t.by -= hsz;
                }
            }
            PARTITION_T_TOP_SPLIT => {
                let branch = &*(node as *const EdgeBranch);
                decode_b(t, bl, b[0], bp, branch.tts[0])?;
                t.bx += hsz;
                decode_b(t, bl, b[0], bp, branch.tts[1])?;
                t.bx -= hsz;
                t.by += hsz;
                decode_b(t, bl, b[1], bp, branch.tts[2])?;
                t.by -= hsz;
            }
            PARTITION_T_BOTTOM_SPLIT => {
                let branch = &*(node as *const EdgeBranch);
                decode_b(t, bl, b[0], bp, branch.tbs[0])?;
                t.by += hsz;
                decode_b(t, bl, b[1], bp, branch.tbs[1])?;
                t.bx += hsz;
                decode_b(t, bl, b[1], bp, branch.tbs[2])?;
                t.bx -= hsz;
                t.by -= hsz;
            }
            PARTITION_T_LEFT_SPLIT => {
                let branch = &*(node as *const EdgeBranch);
                decode_b(t, bl, b[0], bp, branch.tls[0])?;
                t.by += hsz;
                decode_b(t, bl, b[0], bp, branch.tls[1])?;
                t.by -= hsz;
                t.bx += hsz;
                decode_b(t, bl, b[1], bp, branch.tls[2])?;
                t.bx -= hsz;
            }
            PARTITION_T_RIGHT_SPLIT => {
                let branch = &*(node as *const EdgeBranch);
                decode_b(t, bl, b[0], bp, branch.trs[0])?;
                t.bx += hsz;
                decode_b(t, bl, b[1], bp, branch.trs[1])?;
                t.by += hsz;
                decode_b(t, bl, b[1], bp, (*branch).trs[2])?;
                t.by -= hsz;
                t.bx -= hsz;
            }
            PARTITION_H4 => {
                let branch = &*(node as *const EdgeBranch);
                decode_b(t, bl, b[0], bp, branch.h4[0])?;
                t.by += hsz >> 1;
                decode_b(t, bl, b[0], bp, branch.h4[1])?;
                t.by += hsz >> 1;
                decode_b(t, bl, b[0], bp, branch.h4[2])?;
                t.by += hsz >> 1;
                if t.by < f.bh {
                    decode_b(t, bl, b[0], bp, branch.h4[3])?;
                }
                t.by -= hsz * 3 >> 1;
            }
            PARTITION_V4 => {
                let branch = &*(node as *const EdgeBranch);
                decode_b(t, bl, b[0], bp, branch.v4[0])?;
                t.bx += hsz >> 1;
                decode_b(t, bl, b[0], bp, branch.v4[1])?;
                t.bx += hsz >> 1;
                decode_b(t, bl, b[0], bp, branch.v4[2])?;
                t.bx += hsz >> 1;
                if t.bx < f.bw {
                    decode_b(t, bl, b[0], bp, branch.v4[3])?;
                }
                t.bx -= hsz * 3 >> 1;
            }
            _ => unreachable!(),
        }
    } else if have_h_split {
        let is_split;
        if let Some(pc) = pc {
            is_split = rav1d_msac_decode_bool(&mut ts.msac, gather_top_partition_prob(pc, bl));
            if DEBUG_BLOCK_INFO(f, t) {
                println!(
                    "poc={},y={},x={},bl={},ctx={},bp={}: r={}",
                    frame_hdr.frame_offset,
                    t.by,
                    t.bx,
                    bl,
                    ctx,
                    if is_split {
                        PARTITION_SPLIT
                    } else {
                        PARTITION_H
                    },
                    ts.msac.rng,
                );
            }
        } else {
            let b = &mut *(f.frame_thread.b).offset(t.by as isize * f.b4_stride + t.bx as isize);
            is_split = b.bl != bl;
        }

        assert!(bl < BL_8X8);
        if is_split {
            let branch = &*(node as *const EdgeBranch);
            bp = PARTITION_SPLIT;
            decode_sb(t, bl + 1, branch.split[0])?;
            t.bx += hsz;
            decode_sb(t, bl + 1, branch.split[1])?;
            t.bx -= hsz;
        } else {
            bp = PARTITION_H;
            decode_b(
                t,
                bl,
                dav1d_block_sizes[bl as usize][bp as usize][0],
                bp,
                (*node).h[0],
            )?;
        }
    } else {
        assert!(have_v_split);
        let is_split;
        if let Some(pc) = pc {
            is_split = rav1d_msac_decode_bool(&mut ts.msac, gather_left_partition_prob(pc, bl));
            if f.cur.p.layout == Rav1dPixelLayout::I422 && !is_split {
                return Err(());
            }
            if DEBUG_BLOCK_INFO(f, t) {
                println!(
                    "poc={},y={},x={},bl={},ctx={},bp={}: r={}",
                    frame_hdr.frame_offset,
                    t.by,
                    t.bx,
                    bl,
                    ctx,
                    if is_split {
                        PARTITION_SPLIT
                    } else {
                        PARTITION_V
                    },
                    ts.msac.rng,
                );
            }
        } else {
            let b = &mut *(f.frame_thread.b).offset(t.by as isize * f.b4_stride + t.bx as isize);
            is_split = b.bl != bl;
        }

        assert!(bl < BL_8X8);
        if is_split {
            let branch = &*(node as *const EdgeBranch);
            bp = PARTITION_SPLIT;
            decode_sb(t, bl + 1, branch.split[0])?;
            t.by += hsz;
            decode_sb(t, bl + 1, branch.split[2])?;
            t.by -= hsz;
        } else {
            bp = PARTITION_V;
            decode_b(
                t,
                bl,
                dav1d_block_sizes[bl as usize][bp as usize][0],
                bp,
                (*node).v[0],
            )?;
        }
    }

    if t.frame_thread.pass != 2 && (bp != PARTITION_SPLIT || bl == BL_8X8) {
        CaseSet::<16, false>::many(
            [(&mut *t.a, 0), (&mut t.l, 1)],
            [hsz as usize; 2],
            [bx8 as usize, by8 as usize],
            |case, (dir, dir_index)| {
                case.set(
                    &mut dir.partition.0,
                    dav1d_al_part_ctx[dir_index][bl as usize][bp as usize],
                );
            },
        );
    }

    Ok(())
}

fn reset_context(ctx: &mut BlockContext, keyframe: bool, pass: c_int) {
    ctx.intra.0.fill(keyframe.into());
    ctx.uvmode.0.fill(DC_PRED);
    if keyframe {
        ctx.mode.0.fill(DC_PRED);
    }

    if pass == 2 {
        return;
    }

    ctx.partition.0.fill(0);
    ctx.skip.0.fill(0);
    ctx.skip_mode.0.fill(0);
    ctx.tx_lpf_y.0.fill(2);
    ctx.tx_lpf_uv.0.fill(1);
    ctx.tx_intra.0.fill(-1);
    ctx.tx.0.fill(TX_64X64);
    if !keyframe {
        for r#ref in &mut ctx.r#ref.0 {
            r#ref.fill(-1);
        }
        ctx.comp_type.0.fill(0);
        ctx.mode.0.fill(NEARESTMV);
    }
    ctx.lcoef.0.fill(0x40);
    for ccoef in &mut ctx.ccoef.0 {
        ccoef.fill(0x40);
    }
    for filter in &mut ctx.filter.0 {
        filter.fill(RAV1D_N_SWITCHABLE_FILTERS as u8);
    }
    ctx.seg_pred.0.fill(0);
    ctx.pal_sz.0.fill(0);
}

impl DefaultValue for [u8; 2] {
    const DEFAULT: Self = [0; 2];
}

/// `{ Y+U+V, Y+U } * 4`
static ss_size_mul: enum_map_ty!(Rav1dPixelLayout, [u8; 2]) = enum_map!(Rav1dPixelLayout => [u8; 2]; match key {
    I400 => [4, 4],
    I420 => [6, 5],
    I422 => [8, 6],
    I444 => [12, 8],
});

unsafe fn setup_tile(
    ts: &mut Rav1dTileState,
    f: &Rav1dFrameContext,
    data: &[u8],
    tile_row: usize,
    tile_col: usize,
    tile_start_off: usize,
) {
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();

    let col_sb_start = frame_hdr.tiling.col_start_sb[tile_col] as c_int;
    let col_sb128_start = col_sb_start >> (seq_hdr.sb128 == 0) as c_int;
    let col_sb_end = frame_hdr.tiling.col_start_sb[tile_col + 1] as c_int;
    let row_sb_start = frame_hdr.tiling.row_start_sb[tile_row] as c_int;
    let row_sb_end = frame_hdr.tiling.row_start_sb[tile_row + 1] as c_int;
    let sb_shift = f.sb_shift;

    let size_mul = &ss_size_mul[f.cur.p.layout];
    for p in 0..2 {
        ts.frame_thread[p].pal_idx = if !(f.frame_thread.pal_idx).is_null() {
            f.frame_thread
                .pal_idx
                .offset((tile_start_off * size_mul[1] as usize / 4) as isize)
        } else {
            ptr::null_mut()
        };
        ts.frame_thread[p].cf = if !f.frame_thread.cf.is_null() {
            f.frame_thread
                .cf
                .cast::<u8>()
                .offset(
                    (tile_start_off * size_mul[0] as usize >> (seq_hdr.hbd == 0) as c_int) as isize,
                )
                .cast::<DynCoef>()
        } else {
            ptr::null_mut()
        };
    }

    rav1d_cdf_thread_copy(&mut ts.cdf, &f.in_cdf);
    ts.last_qidx = frame_hdr.quant.yac;
    ts.last_delta_lf.fill(0);

    rav1d_msac_init(
        &mut ts.msac,
        data.as_ptr(),
        data.len(),
        frame_hdr.disable_cdf_update != 0,
    );

    ts.tiling.row = tile_row as c_int;
    ts.tiling.col = tile_col as c_int;
    ts.tiling.col_start = col_sb_start << sb_shift;
    ts.tiling.col_end = cmp::min(col_sb_end << sb_shift, f.bw);
    ts.tiling.row_start = row_sb_start << sb_shift;
    ts.tiling.row_end = cmp::min(row_sb_end << sb_shift, f.bh);
    let diff_width = frame_hdr.size.width[0] != frame_hdr.size.width[1];

    // Reference Restoration Unit (used for exp coding)
    let (sb_idx, unit_idx) = if diff_width {
        // vertical components only
        (
            (ts.tiling.row_start >> 5) * f.sr_sb128w,
            (ts.tiling.row_start & 16) >> 3,
        )
    } else {
        (
            (ts.tiling.row_start >> 5) * f.sb128w + col_sb128_start,
            ((ts.tiling.row_start & 16) >> 3) + ((ts.tiling.col_start & 16) >> 4),
        )
    };
    for p in 0..3 {
        if !((f.lf.restore_planes >> p) & 1 != 0) {
            continue;
        }

        let lr_ref = if diff_width {
            let ss_hor = (p != 0 && f.cur.p.layout != Rav1dPixelLayout::I444) as c_int;
            let d = frame_hdr.size.super_res.width_scale_denominator;
            let unit_size_log2 = frame_hdr.restoration.unit_size[(p != 0) as usize];
            let rnd = (8 << unit_size_log2) - 1;
            let shift = unit_size_log2 + 3;
            let x = (4 * ts.tiling.col_start * d >> ss_hor) + rnd >> shift;
            let px_x = x << unit_size_log2 + ss_hor;
            let u_idx = unit_idx + ((px_x & 64) >> 6);
            let sb128x = px_x >> 7;
            if sb128x >= f.sr_sb128w {
                continue;
            }
            &mut (*f.lf.lr_mask.offset((sb_idx + sb128x) as isize)).lr[p][u_idx as usize]
        } else {
            &mut (*f.lf.lr_mask.offset(sb_idx as isize)).lr[p][unit_idx as usize]
        };

        *lr_ref = Av1RestorationUnit {
            filter_v: [3, -7, 15],
            filter_h: [3, -7, 15],
            sgr_weights: [-32, 31],
            ..*lr_ref
        };
        ts.lr_ref[p] = *lr_ref;
    }

    if (*f.c).n_tc > 1 {
        ts.progress.fill(row_sb_start as atomic_int);
    }
}

unsafe fn read_restoration_info(
    t: &mut Rav1dTaskContext,
    lr: &mut Av1RestorationUnit,
    p: usize,
    frame_type: Rav1dRestorationType,
) {
    let f = &*t.f;
    let ts = &mut *t.ts;
    let lr_ref = ts.lr_ref[p];

    if frame_type == RAV1D_RESTORATION_SWITCHABLE {
        let filter =
            rav1d_msac_decode_symbol_adapt4(&mut ts.msac, &mut ts.cdf.m.restore_switchable.0, 2);
        lr.r#type = if filter != 0 {
            if filter == 2 {
                RAV1D_RESTORATION_SGRPROJ
            } else {
                RAV1D_RESTORATION_WIENER
            }
        } else {
            RAV1D_RESTORATION_NONE
        };
    } else {
        let r#type = rav1d_msac_decode_bool_adapt(
            &mut ts.msac,
            if frame_type == RAV1D_RESTORATION_WIENER {
                &mut ts.cdf.m.restore_wiener.0
            } else {
                &mut ts.cdf.m.restore_sgrproj.0
            },
        );
        lr.r#type = if r#type {
            frame_type
        } else {
            RAV1D_RESTORATION_NONE
        };
    }

    fn msac_decode_lr_subexp(ts: &mut Rav1dTileState, r#ref: i8, k: u32, adjustment: i8) -> i8 {
        (rav1d_msac_decode_subexp(&mut ts.msac, (r#ref + adjustment) as c_uint, 8 << k, k)
            - adjustment as c_int) as i8
    }

    if lr.r#type == RAV1D_RESTORATION_WIENER {
        lr.filter_v[0] = if p != 0 {
            0
        } else {
            msac_decode_lr_subexp(ts, lr_ref.filter_v[0], 1, 5)
        };
        lr.filter_v[1] = msac_decode_lr_subexp(ts, lr_ref.filter_v[1], 2, 23);
        lr.filter_v[2] = msac_decode_lr_subexp(ts, lr_ref.filter_v[2], 3, 17);

        lr.filter_h[0] = if p != 0 {
            0
        } else {
            msac_decode_lr_subexp(ts, lr_ref.filter_h[0], 1, 5)
        };
        lr.filter_h[1] = msac_decode_lr_subexp(ts, lr_ref.filter_h[1], 2, 23);
        lr.filter_h[2] = msac_decode_lr_subexp(ts, lr_ref.filter_h[2], 3, 17);
        lr.sgr_weights = lr_ref.sgr_weights;
        ts.lr_ref[p] = *lr;
        if DEBUG_BLOCK_INFO(f, t) {
            println!(
                "Post-lr_wiener[pl={},v[{},{},{}],h[{},{},{}]]: r={}",
                p,
                lr.filter_v[0],
                lr.filter_v[1],
                lr.filter_v[2],
                lr.filter_h[0],
                lr.filter_h[1],
                lr.filter_h[2],
                ts.msac.rng,
            );
        }
    } else if lr.r#type == RAV1D_RESTORATION_SGRPROJ {
        let idx = rav1d_msac_decode_bools(&mut ts.msac, 4) as u8;
        let sgr_params = &dav1d_sgr_params[idx.into()];
        lr.sgr_idx = idx;
        lr.sgr_weights[0] = if sgr_params[0] != 0 {
            msac_decode_lr_subexp(ts, lr_ref.sgr_weights[0], 4, 96)
        } else {
            0
        };
        lr.sgr_weights[1] = if sgr_params[1] != 0 {
            msac_decode_lr_subexp(ts, lr_ref.sgr_weights[1], 4, 32)
        } else {
            95
        };
        lr.filter_v = lr_ref.filter_v;
        lr.filter_h = lr_ref.filter_h;
        ts.lr_ref[p] = *lr;
        if DEBUG_BLOCK_INFO(f, t) {
            println!(
                "Post-lr_sgrproj[pl={},idx={},w[{},{}]]: r={}",
                p, lr.sgr_idx, lr.sgr_weights[0], lr.sgr_weights[1], ts.msac.rng,
            );
        }
    }
}

pub(crate) unsafe fn rav1d_decode_tile_sbrow(t: &mut Rav1dTaskContext) -> Result<(), ()> {
    let f = &*t.f;
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    let root_bl = if seq_hdr.sb128 != 0 {
        BL_128X128
    } else {
        BL_64X64
    };
    let ts = &mut *t.ts;
    let c = &*f.c;
    let sb_step = f.sb_step;
    let tile_row = ts.tiling.row;
    let tile_col = ts.tiling.col;
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    let col_sb_start = frame_hdr.tiling.col_start_sb[tile_col as usize] as c_int;
    let col_sb128_start = col_sb_start >> (seq_hdr.sb128 == 0) as c_int;

    if frame_hdr.frame_type.is_inter_or_switch() || frame_hdr.allow_intrabc != 0 {
        rav1d_refmvs_tile_sbrow_init(
            &mut t.rt,
            &f.rf,
            ts.tiling.col_start,
            ts.tiling.col_end,
            ts.tiling.row_start,
            ts.tiling.row_end,
            t.by >> f.sb_shift,
            ts.tiling.row,
            t.frame_thread.pass,
        );
    }

    if frame_hdr.frame_type.is_inter_or_switch() && c.n_fc > 1 {
        let sby = t.by - ts.tiling.row_start >> f.sb_shift;
        *ts.lowest_pixel.offset(sby as isize) = [[i32::MIN; 2]; 7];
    }

    reset_context(
        &mut t.l,
        frame_hdr.frame_type.is_key_or_intra(),
        t.frame_thread.pass,
    );
    if t.frame_thread.pass == 2 {
        let off_2pass = if c.n_tc > 1 {
            f.sb128w * frame_hdr.tiling.rows
        } else {
            0
        };
        t.a =
            f.a.offset((off_2pass + col_sb128_start + tile_row * f.sb128w) as isize);
        for bx in (ts.tiling.col_start..ts.tiling.col_end).step_by(sb_step as usize) {
            t.bx = bx;
            if ::core::intrinsics::atomic_load_acquire(c.flush) != 0 {
                return Err(());
            }
            decode_sb(t, root_bl, c.intra_edge.root[root_bl as usize])?;
            if t.bx & 16 != 0 || seq_hdr.sb128 != 0 {
                t.a = (t.a).offset(1);
            }
        }
        (f.bd_fn.backup_ipred_edge)(t);
        return Ok(());
    }

    // error out on symbol decoder overread
    if ts.msac.cnt < -15 {
        return Err(());
    }

    if (*f.c).n_tc > 1 && frame_hdr.use_ref_frame_mvs != 0 {
        (*f.c)
            .refmvs_dsp
            .load_tmvs
            .expect("non-null function pointer")(
            &f.rf,
            ts.tiling.row,
            ts.tiling.col_start >> 1,
            ts.tiling.col_end >> 1,
            t.by >> 1,
            t.by + sb_step >> 1,
        );
    }
    t.pal_sz_uv[1] = Default::default();
    let sb128y = t.by >> 5;
    t.a = f.a.offset((col_sb128_start + tile_row * f.sb128w) as isize);
    t.lf_mask =
        f.lf.mask
            .offset((sb128y * f.sb128w + col_sb128_start) as isize);
    for bx in (ts.tiling.col_start..ts.tiling.col_end).step_by(sb_step as usize) {
        t.bx = bx;
        if ::core::intrinsics::atomic_load_acquire(c.flush) != 0 {
            return Err(());
        }
        let cdef_idx = &mut (*t.lf_mask).cdef_idx;
        if root_bl == BL_128X128 {
            *cdef_idx = [-1; 4];
            t.cur_sb_cdef_idx_ptr = cdef_idx.as_mut_ptr();
        } else {
            let cdef_idx = &mut cdef_idx[(((t.bx & 16) >> 4) + ((t.by & 16) >> 3)) as usize..];
            cdef_idx[0] = -1;
            t.cur_sb_cdef_idx_ptr = cdef_idx.as_mut_ptr();
        }
        // Restoration filter
        for p in 0..3 {
            if (f.lf.restore_planes >> p) & 1 == 0 {
                continue;
            }

            let ss_ver = (p != 0 && f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
            let ss_hor = (p != 0 && f.cur.p.layout != Rav1dPixelLayout::I444) as c_int;
            let unit_size_log2 = frame_hdr.restoration.unit_size[(p != 0) as usize];
            let y = t.by * 4 >> ss_ver;
            let h = f.cur.p.h + ss_ver >> ss_ver;

            let unit_size = 1 << unit_size_log2;
            let mask = (unit_size - 1) as c_uint;
            if y as c_uint & mask != 0 {
                continue;
            }
            let half_unit = unit_size >> 1;
            // Round half up at frame boundaries,
            // if there's more than one restoration unit.
            if y != 0 && y + half_unit > h {
                continue;
            }

            let frame_type = frame_hdr.restoration.r#type[p as usize];

            if frame_hdr.size.width[0] != frame_hdr.size.width[1] {
                let w = f.sr_cur.p.p.w + ss_hor >> ss_hor;
                let n_units = cmp::max(1, w + half_unit >> unit_size_log2);

                let d = frame_hdr.size.super_res.width_scale_denominator;
                let rnd = unit_size * 8 - 1;
                let shift = unit_size_log2 + 3;
                let x0 = (4 * t.bx * d >> ss_hor) + rnd >> shift;
                let x1 = (4 * (t.bx + sb_step) * d >> ss_hor) + rnd >> shift;

                for x in x0..cmp::min(x1, n_units) {
                    let px_x = x << unit_size_log2 + ss_hor;
                    let sb_idx = (t.by >> 5) * f.sr_sb128w + (px_x >> 7);
                    let unit_idx = ((t.by & 16) >> 3) + ((px_x & 64) >> 6);
                    let lr =
                        &mut (*(f.lf.lr_mask).offset(sb_idx as isize)).lr[p][unit_idx as usize];

                    read_restoration_info(t, lr, p, frame_type);
                }
            } else {
                let x = 4 * t.bx >> ss_hor;
                if x as c_uint & mask != 0 {
                    continue;
                }
                let w = f.cur.p.w + ss_hor >> ss_hor;
                // Round half up at frame boundaries,
                // if there's more than one restoration unit.
                if x != 0 && x + half_unit > w {
                    continue;
                }
                let sb_idx = (t.by >> 5) * f.sr_sb128w + (t.bx >> 5);
                let unit_idx = ((t.by & 16) >> 3) + ((t.bx & 16) >> 4);
                let lr = &mut (*(f.lf.lr_mask).offset(sb_idx as isize)).lr[p][unit_idx as usize];

                read_restoration_info(t, lr, p, frame_type);
            }
        }
        decode_sb(t, root_bl, c.intra_edge.root[root_bl as usize])?;
        if t.bx & 16 != 0 || seq_hdr.sb128 != 0 {
            t.a = (t.a).offset(1);
            t.lf_mask = (t.lf_mask).offset(1);
        }
    }

    if seq_hdr.ref_frame_mvs != 0 && (*f.c).n_tc > 1 && frame_hdr.frame_type.is_inter_or_switch() {
        rav1d_refmvs_save_tmvs(
            &(*f.c).refmvs_dsp,
            &mut t.rt,
            ts.tiling.col_start >> 1,
            ts.tiling.col_end >> 1,
            t.by >> 1,
            t.by + sb_step >> 1,
        );
    }

    // backup pre-loopfilter pixels for intra prediction of the next sbrow
    if t.frame_thread.pass != 1 {
        (f.bd_fn.backup_ipred_edge)(t);
    }

    // backup t->a/l.tx_lpf_y/uv at tile boundaries to use them to "fix"
    // up the initial value in neighbour tiles when running the loopfilter
    let mut align_h = f.bh + 31 & !31;
    slice::from_raw_parts_mut(
        f.lf.tx_lpf_right_edge[0],
        (align_h * tile_col + t.by + sb_step).try_into().unwrap(),
    )[(align_h * tile_col + t.by).try_into().unwrap()..][..sb_step.try_into().unwrap()]
        .copy_from_slice(&t.l.tx_lpf_y.0[(t.by & 16) as usize..][..sb_step.try_into().unwrap()]);
    let ss_ver = (f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
    align_h >>= ss_ver;
    slice::from_raw_parts_mut(
        f.lf.tx_lpf_right_edge[1],
        (align_h * tile_col + (t.by >> ss_ver) + (sb_step >> ss_ver))
            .try_into()
            .unwrap(),
    )[(align_h * tile_col + (t.by >> ss_ver)).try_into().unwrap()..]
        [..(sb_step >> ss_ver).try_into().unwrap()]
        .copy_from_slice(
            &t.l.tx_lpf_uv.0[((t.by & 16) >> ss_ver) as usize..]
                [..(sb_step >> ss_ver).try_into().unwrap()],
        );

    Ok(())
}

pub(crate) unsafe fn rav1d_decode_frame_init(f: &mut Rav1dFrameContext) -> Rav1dResult {
    let c = &*f.c;

    if f.sbh > f.lf.start_of_tile_row_sz {
        free(f.lf.start_of_tile_row as *mut c_void);
        f.lf.start_of_tile_row = malloc(f.sbh as usize * ::core::mem::size_of::<u8>()) as *mut u8;
        if f.lf.start_of_tile_row.is_null() {
            f.lf.start_of_tile_row_sz = 0;
            return Err(ENOMEM);
        }
        f.lf.start_of_tile_row_sz = f.sbh;
    }
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    let mut sby = 0;
    for tile_row in 0..frame_hdr.tiling.rows {
        *f.lf.start_of_tile_row.offset(sby as isize) = tile_row as u8;
        sby += 1;
        while sby < frame_hdr.tiling.row_start_sb[(tile_row + 1) as usize] as c_int {
            *f.lf.start_of_tile_row.offset(sby as isize) = 0;
            sby += 1;
        }
    }

    let n_ts = frame_hdr.tiling.cols * frame_hdr.tiling.rows;
    if n_ts != f.n_ts {
        if c.n_fc > 1 {
            freep(&mut f.frame_thread.tile_start_off as *mut *mut c_int as *mut c_void);
            f.frame_thread.tile_start_off =
                malloc(::core::mem::size_of::<c_int>() * n_ts as usize) as *mut c_int;
            if f.frame_thread.tile_start_off.is_null() {
                f.n_ts = 0;
                return Err(ENOMEM);
            }
        }
        rav1d_free_aligned(f.ts as *mut c_void);
        f.ts = rav1d_alloc_aligned(::core::mem::size_of::<Rav1dTileState>() * n_ts as usize, 32)
            as *mut Rav1dTileState;
        if f.ts.is_null() {
            return Err(ENOMEM);
        }
        f.n_ts = n_ts;
    }

    let a_sz = f.sb128w * frame_hdr.tiling.rows * (1 + (c.n_fc > 1 && c.n_tc > 1) as c_int);
    if a_sz != f.a_sz {
        freep(&mut f.a as *mut *mut BlockContext as *mut c_void);
        f.a = malloc(::core::mem::size_of::<BlockContext>() * a_sz as usize) as *mut BlockContext;
        if f.a.is_null() {
            f.a_sz = 0;
            return Err(ENOMEM);
        }
        f.a_sz = a_sz;
    }

    let num_sb128 = f.sb128w * f.sb128h;
    let size_mul = &ss_size_mul[f.cur.p.layout];
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    let hbd = (seq_hdr.hbd != 0) as c_int;
    if c.n_fc > 1 {
        let mut tile_idx = 0;
        for tile_row in 0..frame_hdr.tiling.rows {
            let row_off = frame_hdr.tiling.row_start_sb[tile_row as usize] as c_int
                * f.sb_step
                * 4
                * f.sb128w
                * 128;
            let b_diff = (frame_hdr.tiling.row_start_sb[(tile_row + 1) as usize] as c_int
                - frame_hdr.tiling.row_start_sb[tile_row as usize] as c_int)
                * f.sb_step
                * 4;
            for tile_col in 0..frame_hdr.tiling.cols {
                *f.frame_thread.tile_start_off.offset(tile_idx as isize) = row_off
                    + b_diff
                        * frame_hdr.tiling.col_start_sb[tile_col as usize] as c_int
                        * f.sb_step
                        * 4;

                tile_idx += 1;
            }
        }

        let lowest_pixel_mem_sz = frame_hdr.tiling.cols * f.sbh;
        if lowest_pixel_mem_sz != f.tile_thread.lowest_pixel_mem_sz {
            free(f.tile_thread.lowest_pixel_mem as *mut c_void);
            f.tile_thread.lowest_pixel_mem =
                malloc(lowest_pixel_mem_sz as usize * ::core::mem::size_of::<[[c_int; 2]; 7]>())
                    as *mut [[c_int; 2]; 7];
            if f.tile_thread.lowest_pixel_mem.is_null() {
                f.tile_thread.lowest_pixel_mem_sz = 0;
                return Err(ENOMEM);
            }
            f.tile_thread.lowest_pixel_mem_sz = lowest_pixel_mem_sz;
        }
        let mut lowest_pixel_ptr = f.tile_thread.lowest_pixel_mem;
        for tile_row in 0..frame_hdr.tiling.rows {
            let tile_row_base = tile_row * frame_hdr.tiling.cols;
            let tile_row_sb_h = frame_hdr.tiling.row_start_sb[(tile_row + 1) as usize] as c_int
                - frame_hdr.tiling.row_start_sb[tile_row as usize] as c_int;
            for tile_col in 0..frame_hdr.tiling.cols {
                (*f.ts.offset((tile_row_base + tile_col) as isize)).lowest_pixel = lowest_pixel_ptr;
                lowest_pixel_ptr = lowest_pixel_ptr.offset(tile_row_sb_h as isize);
            }
        }

        let cf_sz = (num_sb128 * size_mul[0] as c_int) << hbd;
        if cf_sz != f.frame_thread.cf_sz {
            rav1d_freep_aligned(&mut f.frame_thread.cf as *mut *mut DynCoef as *mut c_void);
            f.frame_thread.cf =
                rav1d_alloc_aligned(cf_sz as usize * 128 * 128 / 2, 64) as *mut DynCoef;
            if f.frame_thread.cf.is_null() {
                f.frame_thread.cf_sz = 0;
                return Err(ENOMEM);
            }
            slice::from_raw_parts_mut(
                f.frame_thread.cf.cast::<u8>(),
                usize::try_from(cf_sz).unwrap() * 128 * 128 / 2,
            )
            .fill(0);
            f.frame_thread.cf_sz = cf_sz;
        }

        if frame_hdr.allow_screen_content_tools != 0 {
            if num_sb128 != f.frame_thread.pal_sz {
                rav1d_freep_aligned(
                    &mut f.frame_thread.pal as *mut *mut [[u16; 8]; 3] as *mut c_void,
                );
                f.frame_thread.pal = rav1d_alloc_aligned(
                    ::core::mem::size_of::<[[u16; 8]; 3]>() * num_sb128 as usize * 16 * 16,
                    64,
                ) as *mut [[u16; 8]; 3];
                if f.frame_thread.pal.is_null() {
                    f.frame_thread.pal_sz = 0;
                    return Err(ENOMEM);
                }
                f.frame_thread.pal_sz = num_sb128;
            }

            let pal_idx_sz = num_sb128 * size_mul[1] as c_int;
            if pal_idx_sz != f.frame_thread.pal_idx_sz {
                rav1d_freep_aligned(&mut f.frame_thread.pal_idx as *mut *mut u8 as *mut c_void);
                f.frame_thread.pal_idx = rav1d_alloc_aligned(
                    ::core::mem::size_of::<u8>() * pal_idx_sz as usize * 128 * 128 / 4,
                    64,
                ) as *mut u8;
                if f.frame_thread.pal_idx.is_null() {
                    f.frame_thread.pal_idx_sz = 0;
                    return Err(ENOMEM);
                }
                f.frame_thread.pal_idx_sz = pal_idx_sz;
            }
        } else if !f.frame_thread.pal.is_null() {
            rav1d_freep_aligned(&mut f.frame_thread.pal as *mut *mut [[u16; 8]; 3] as *mut c_void);
            rav1d_freep_aligned(&mut f.frame_thread.pal_idx as *mut *mut u8 as *mut c_void);
            f.frame_thread.pal_idx_sz = 0;
            f.frame_thread.pal_sz = f.frame_thread.pal_idx_sz;
        }
    }

    // update allocation of block contexts for above
    let mut y_stride = f.cur.stride[0];
    let mut uv_stride = f.cur.stride[1];
    let has_resize = (frame_hdr.size.width[0] != frame_hdr.size.width[1]) as c_int;
    let need_cdef_lpf_copy = (c.n_tc > 1 && has_resize != 0) as c_int;
    if y_stride * f.sbh as isize * 4 != f.lf.cdef_buf_plane_sz[0] as isize
        || uv_stride * f.sbh as isize * 8 != f.lf.cdef_buf_plane_sz[1] as isize
        || need_cdef_lpf_copy != f.lf.need_cdef_lpf_copy
        || f.sbh != f.lf.cdef_buf_sbh
    {
        rav1d_free_aligned(f.lf.cdef_line_buf as *mut c_void);
        let mut alloc_sz: usize = 64;
        alloc_sz += (y_stride.unsigned_abs() * 4 * f.sbh as usize) << need_cdef_lpf_copy;
        alloc_sz += (uv_stride.unsigned_abs() * 8 * f.sbh as usize) << need_cdef_lpf_copy;
        f.lf.cdef_line_buf = rav1d_alloc_aligned(alloc_sz, 32) as *mut u8;
        let mut ptr = f.lf.cdef_line_buf;
        if ptr.is_null() {
            f.lf.cdef_buf_plane_sz[1] = 0;
            f.lf.cdef_buf_plane_sz[0] = f.lf.cdef_buf_plane_sz[1];
            return Err(ENOMEM);
        }

        ptr = ptr.offset(32);
        if y_stride < 0 {
            f.lf.cdef_line[0][0] =
                ptr.offset(-(y_stride * (f.sbh as isize * 4 - 1))) as *mut DynPixel;
            f.lf.cdef_line[1][0] =
                ptr.offset(-(y_stride * (f.sbh as isize * 4 - 3))) as *mut DynPixel;
        } else {
            f.lf.cdef_line[0][0] = ptr.offset(y_stride * 0) as *mut DynPixel;
            f.lf.cdef_line[1][0] = ptr.offset(y_stride * 2) as *mut DynPixel;
        }
        ptr = ptr.offset(y_stride.abs() * f.sbh as isize * 4);
        if uv_stride < 0 {
            f.lf.cdef_line[0][1] =
                ptr.offset(-(uv_stride * (f.sbh as isize * 8 - 1))) as *mut DynPixel;
            f.lf.cdef_line[0][2] =
                ptr.offset(-(uv_stride * (f.sbh as isize * 8 - 3))) as *mut DynPixel;
            f.lf.cdef_line[1][1] =
                ptr.offset(-(uv_stride * (f.sbh as isize * 8 - 5))) as *mut DynPixel;
            f.lf.cdef_line[1][2] =
                ptr.offset(-(uv_stride * (f.sbh as isize * 8 - 7))) as *mut DynPixel;
        } else {
            f.lf.cdef_line[0][1] = ptr.offset(uv_stride * 0) as *mut DynPixel;
            f.lf.cdef_line[0][2] = ptr.offset(uv_stride * 2) as *mut DynPixel;
            f.lf.cdef_line[1][1] = ptr.offset(uv_stride * 4) as *mut DynPixel;
            f.lf.cdef_line[1][2] = ptr.offset(uv_stride * 6) as *mut DynPixel;
        }

        if need_cdef_lpf_copy != 0 {
            ptr = ptr.offset(uv_stride.abs() * f.sbh as isize * 8);
            if y_stride < 0 {
                f.lf.cdef_lpf_line[0] =
                    ptr.offset(-(y_stride * (f.sbh as isize * 4 - 1))) as *mut DynPixel;
            } else {
                f.lf.cdef_lpf_line[0] = ptr as *mut DynPixel;
            }
            ptr = ptr.offset(y_stride.abs() * f.sbh as isize * 4);
            if uv_stride < 0 {
                f.lf.cdef_lpf_line[1] =
                    ptr.offset(-(uv_stride * (f.sbh as isize * 4 - 1))) as *mut DynPixel;
                f.lf.cdef_lpf_line[2] =
                    ptr.offset(-(uv_stride * (f.sbh as isize * 8 - 1))) as *mut DynPixel;
            } else {
                f.lf.cdef_lpf_line[1] = ptr as *mut DynPixel;
                f.lf.cdef_lpf_line[2] = ptr.offset(uv_stride * f.sbh as isize * 4) as *mut DynPixel;
            }
        }

        f.lf.cdef_buf_plane_sz[0] = y_stride as c_int * f.sbh * 4;
        f.lf.cdef_buf_plane_sz[1] = uv_stride as c_int * f.sbh * 8;
        f.lf.need_cdef_lpf_copy = need_cdef_lpf_copy;
        f.lf.cdef_buf_sbh = f.sbh;
    }

    let sb128 = seq_hdr.sb128;
    let num_lines = if c.n_tc > 1 { (f.sbh * 4) << sb128 } else { 12 };
    y_stride = f.sr_cur.p.stride[0];
    uv_stride = f.sr_cur.p.stride[1];
    if y_stride * num_lines as isize != f.lf.lr_buf_plane_sz[0] as isize
        || uv_stride * num_lines as isize * 2 != f.lf.lr_buf_plane_sz[1] as isize
    {
        rav1d_free_aligned(f.lf.lr_line_buf as *mut c_void);
        // lr simd may overread the input, so slightly over-allocate the lpf buffer
        let mut alloc_sz: usize = 128;
        alloc_sz += y_stride.unsigned_abs() * num_lines as usize;
        alloc_sz += uv_stride.unsigned_abs() * num_lines as usize * 2;
        f.lf.lr_line_buf = rav1d_alloc_aligned(alloc_sz, 64) as *mut u8;
        let mut ptr = f.lf.lr_line_buf;
        if ptr.is_null() {
            f.lf.lr_buf_plane_sz[1] = 0;
            f.lf.lr_buf_plane_sz[0] = f.lf.lr_buf_plane_sz[1];
            return Err(ENOMEM);
        }

        ptr = ptr.offset(64);
        if y_stride < 0 {
            f.lf.lr_lpf_line[0] =
                ptr.offset(-(y_stride * (num_lines as isize - 1))) as *mut DynPixel;
        } else {
            f.lf.lr_lpf_line[0] = ptr as *mut DynPixel;
        }
        ptr = ptr.offset(y_stride.abs() * num_lines as isize);
        if uv_stride < 0 {
            f.lf.lr_lpf_line[1] =
                ptr.offset(-(uv_stride * (num_lines as isize * 1 - 1))) as *mut DynPixel;
            f.lf.lr_lpf_line[2] =
                ptr.offset(-(uv_stride * (num_lines as isize * 2 - 1))) as *mut DynPixel;
        } else {
            f.lf.lr_lpf_line[1] = ptr as *mut DynPixel;
            f.lf.lr_lpf_line[2] = ptr.offset(uv_stride * num_lines as isize) as *mut DynPixel;
        }

        f.lf.lr_buf_plane_sz[0] = y_stride as c_int * num_lines;
        f.lf.lr_buf_plane_sz[1] = uv_stride as c_int * num_lines * 2;
    }

    // update allocation for loopfilter masks
    if num_sb128 != f.lf.mask_sz {
        freep(&mut f.lf.mask as *mut *mut Av1Filter as *mut c_void);
        freep(&mut f.lf.level as *mut *mut [u8; 4] as *mut c_void);
        f.lf.mask =
            malloc(::core::mem::size_of::<Av1Filter>() * num_sb128 as usize) as *mut Av1Filter;
        // over-allocate by 3 bytes since some of the SIMD implementations
        // index this from the level type and can thus over-read by up to 3
        f.lf.level = malloc(::core::mem::size_of::<[u8; 4]>() * num_sb128 as usize * 32 * 32 + 3)
            as *mut [u8; 4];
        if f.lf.mask.is_null() || f.lf.level.is_null() {
            f.lf.mask_sz = 0;
            return Err(ENOMEM);
        }
        if c.n_fc > 1 {
            freep(&mut f.frame_thread.b as *mut *mut Av1Block as *mut c_void);
            freep(&mut f.frame_thread.cbi as *mut *mut CodedBlockInfo as *mut c_void);
            f.frame_thread.b =
                malloc(::core::mem::size_of::<Av1Block>() * num_sb128 as usize * 32 * 32)
                    as *mut Av1Block;
            f.frame_thread.cbi =
                malloc(::core::mem::size_of::<CodedBlockInfo>() * num_sb128 as usize * 32 * 32)
                    as *mut CodedBlockInfo;
            if f.frame_thread.b.is_null() || f.frame_thread.cbi.is_null() {
                f.lf.mask_sz = 0;
                return Err(ENOMEM);
            }
        }
        f.lf.mask_sz = num_sb128;
    }

    f.sr_sb128w = f.sr_cur.p.p.w + 127 >> 7;
    let lr_mask_sz = f.sr_sb128w * f.sb128h;
    if lr_mask_sz != f.lf.lr_mask_sz {
        freep(&mut f.lf.lr_mask as *mut *mut Av1Restoration as *mut c_void);
        f.lf.lr_mask = malloc(::core::mem::size_of::<Av1Restoration>() * lr_mask_sz as usize)
            as *mut Av1Restoration;
        if f.lf.lr_mask.is_null() {
            f.lf.lr_mask_sz = 0;
            return Err(ENOMEM);
        }
        f.lf.lr_mask_sz = lr_mask_sz;
    }
    f.lf.restore_planes = frame_hdr
        .restoration
        .r#type
        .iter()
        .enumerate()
        .map(|(i, &r#type)| ((r#type != RAV1D_RESTORATION_NONE) as u8) << i)
        .sum::<u8>()
        .into();
    if frame_hdr.loopfilter.sharpness != f.lf.last_sharpness {
        rav1d_calc_eih(&mut f.lf.lim_lut.0, frame_hdr.loopfilter.sharpness);
        f.lf.last_sharpness = frame_hdr.loopfilter.sharpness;
    }
    rav1d_calc_lf_values(&mut f.lf.lvl, &frame_hdr, &[0, 0, 0, 0]);
    slice::from_raw_parts_mut(f.lf.mask, num_sb128.try_into().unwrap()).fill_with(Default::default);

    let ipred_edge_sz = f.sbh * f.sb128w << hbd;
    if ipred_edge_sz != f.ipred_edge_sz {
        rav1d_freep_aligned(
            &mut *f.ipred_edge.as_mut_ptr().offset(0) as *mut *mut DynPixel as *mut c_void,
        );
        f.ipred_edge[0] =
            rav1d_alloc_aligned(ipred_edge_sz as usize * 128 * 3, 64) as *mut DynPixel;
        let ptr = f.ipred_edge[0] as *mut u8;
        if ptr.is_null() {
            f.ipred_edge_sz = 0;
            return Err(ENOMEM);
        }
        f.ipred_edge[1] = ptr.offset(ipred_edge_sz as isize * 128 * 1) as *mut DynPixel;
        f.ipred_edge[2] = ptr.offset(ipred_edge_sz as isize * 128 * 2) as *mut DynPixel;
        f.ipred_edge_sz = ipred_edge_sz;
    }

    let re_sz = f.sb128h * frame_hdr.tiling.cols;
    if re_sz != f.lf.re_sz {
        freep(&mut *f.lf.tx_lpf_right_edge.as_mut_ptr().offset(0) as *mut *mut u8 as *mut c_void);
        f.lf.tx_lpf_right_edge[0] = malloc(re_sz as usize * 32 * 2) as *mut u8;
        if f.lf.tx_lpf_right_edge[0].is_null() {
            f.lf.re_sz = 0;
            return Err(ENOMEM);
        }
        f.lf.tx_lpf_right_edge[1] = f.lf.tx_lpf_right_edge[0].offset((re_sz * 32) as isize);
        f.lf.re_sz = re_sz;
    }

    // init ref mvs
    if frame_hdr.frame_type.is_inter_or_switch() || frame_hdr.allow_intrabc != 0 {
        let ret = rav1d_refmvs_init_frame(
            &mut f.rf,
            seq_hdr,
            frame_hdr,
            f.refpoc.as_ptr(),
            f.mvs,
            f.refrefpoc.as_ptr(),
            f.ref_mvs.as_ptr(),
            (*f.c).n_tc as c_int,
            (*f.c).n_fc as c_int,
        );
        if ret.is_err() {
            return Err(ENOMEM);
        }
    }

    // setup dequant tables
    init_quant_tables(&seq_hdr, &frame_hdr, frame_hdr.quant.yac, &mut f.dq);
    if frame_hdr.quant.qm != 0 {
        for i in 0..N_RECT_TX_SIZES {
            f.qm[i][0] = dav1d_qm_tbl[frame_hdr.quant.qm_y as usize][0][i]
                .map_or(std::ptr::null(), |qm| qm.as_ptr());
            f.qm[i][1] = dav1d_qm_tbl[frame_hdr.quant.qm_u as usize][1][i]
                .map_or(std::ptr::null(), |qm| qm.as_ptr());
            f.qm[i][2] = dav1d_qm_tbl[frame_hdr.quant.qm_v as usize][1][i]
                .map_or(std::ptr::null(), |qm| qm.as_ptr());
        }
    } else {
        f.qm = [[ptr::null(); 3]; 19]; // TODO(kkysen) can be Default::default once the type is Option
    }

    // setup jnt_comp weights
    if frame_hdr.switchable_comp_refs != 0 {
        let ref_pocs: [_; 7] =
            array::from_fn(|i| f.refp[i].p.frame_hdr.as_ref().unwrap().frame_offset);
        for i in 0..ref_pocs.len() {
            for j in i + 1..ref_pocs.len() {
                let d = [j, i].map(|ij| {
                    cmp::min(
                        (get_poc_diff(
                            seq_hdr.order_hint_n_bits,
                            ref_pocs[ij],
                            f.cur.frame_hdr.as_ref().unwrap().frame_offset,
                        ))
                        .unsigned_abs(),
                        31,
                    ) as u8
                });
                let order = d[0] <= d[1];

                static quant_dist_weight: [[u8; 2]; 3] = [[2, 3], [2, 5], [2, 7]];
                static quant_dist_lookup_table: [[u8; 2]; 4] = [[9, 7], [11, 5], [12, 4], [13, 3]];

                let k = quant_dist_weight
                    .into_iter()
                    .position(|weight| {
                        let c = [order, !order].map(|order| weight[order as usize]);
                        let dc: [_; 2] = array::from_fn(|i| d[i] * c[i]);
                        !order && dc[0] < dc[1] || order && dc[0] > dc[1]
                    })
                    .unwrap_or(quant_dist_weight.len());

                f.jnt_weights[i][j] = quant_dist_lookup_table[k][order as usize];
            }
        }
    }

    // Init loopfilter pointers. Increasing NULL pointers is technically UB,
    // so just point the chroma pointers in 4:0:0 to the luma plane here
    // to avoid having additional in-loop branches in various places.
    // We never dereference those pointers, so it doesn't really matter
    // what they point at, as long as the pointers are valid.
    let has_chroma = (f.cur.p.layout != Rav1dPixelLayout::I400) as usize;
    f.lf.mask_ptr = f.lf.mask;
    f.lf.p = array::from_fn(|i| f.cur.data[has_chroma * i].cast());
    f.lf.sr_p = array::from_fn(|i| f.sr_cur.p.data[has_chroma * i].cast());

    Ok(())
}

pub(crate) unsafe fn rav1d_decode_frame_init_cdf(f: &mut Rav1dFrameContext) -> Rav1dResult {
    let c = &*f.c;
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();

    if frame_hdr.refresh_context != 0 {
        rav1d_cdf_thread_copy(f.out_cdf.data.cdf, &mut f.in_cdf);
    }

    let uses_2pass = c.n_fc > 1;

    let tiling = &frame_hdr.tiling;

    let n_bytes = tiling.n_bytes.try_into().unwrap();
    let rows: usize = tiling.rows.try_into().unwrap();
    let cols = tiling.cols.try_into().unwrap();
    let sb128w: usize = f.sb128w.try_into().unwrap();

    // parse individual tiles per tile group
    let mut tile_row = 0;
    let mut tile_col = 0;
    f.task_thread.update_set = false;
    for tile in &f.tiles {
        let start = tile.hdr.start.try_into().unwrap();
        let end: usize = tile.hdr.end.try_into().unwrap();

        let mut data = slice::from_raw_parts(tile.data.data, tile.data.sz);

        for (j, (ts, tile_start_off)) in iter::zip(
            slice::from_raw_parts_mut(f.ts, end + 1),
            slice::from_raw_parts(
                f.frame_thread.tile_start_off,
                if uses_2pass { end + 1 } else { 0 },
            )
            .into_iter()
            .map(|&it| it as usize)
            .chain(iter::repeat(0)),
        )
        .enumerate()
        .skip(start)
        {
            let tile_sz = if j == end {
                data.len()
            } else {
                if n_bytes > data.len() {
                    return Err(EINVAL);
                }
                let (cur_data, rest_data) = data.split_at(n_bytes);
                let tile_sz = cur_data
                    .iter()
                    .enumerate()
                    .map(|(k, &data)| (data as usize) << (k * 8))
                    .fold(0, |tile_sz, data_k| tile_sz | data_k)
                    + 1;
                data = rest_data;
                if tile_sz > data.len() {
                    return Err(EINVAL);
                }
                tile_sz
            };

            let (cur_data, rest_data) = data.split_at(tile_sz);
            setup_tile(ts, f, cur_data, tile_row, tile_col, tile_start_off);
            tile_col += 1;

            if tile_col == cols {
                tile_col = 0;
                tile_row += 1;
            }
            if j == tiling.update as usize && frame_hdr.refresh_context != 0 {
                f.task_thread.update_set = true;
            }
            data = rest_data;
        }
    }

    if c.n_tc > 1 {
        for (n, ctx) in slice::from_raw_parts_mut(f.a, sb128w * rows * (1 + uses_2pass as usize))
            .iter_mut()
            .enumerate()
        {
            reset_context(
                ctx,
                frame_hdr.frame_type.is_key_or_intra(),
                if uses_2pass {
                    1 + (n >= sb128w * rows) as c_int
                } else {
                    0
                },
            );
        }
    }

    Ok(())
}

unsafe fn rav1d_decode_frame_main(f: &mut Rav1dFrameContext) -> Rav1dResult {
    let c = &*f.c;

    assert!(c.n_tc == 1);

    let t = &mut *c.tc.offset((f as *mut Rav1dFrameContext).offset_from(c.fc));
    t.f = f;
    t.frame_thread.pass = 0;

    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();

    for ctx in
        slice::from_raw_parts_mut(f.a, (f.sb128w * frame_hdr.tiling.rows).try_into().unwrap())
    {
        reset_context(ctx, frame_hdr.frame_type.is_key_or_intra(), 0);
    }

    // no threading - we explicitly interleave tile/sbrow decoding
    // and post-filtering, so that the full process runs in-line
    let Rav1dFrameHeader_tiling { rows, cols, .. } = frame_hdr.tiling;
    let [rows, cols] = [rows, cols].map(|it| it.try_into().unwrap());
    // Need to clone this because `(f.bd_fn.filter_sbrow)(f, sby);` takes a `&mut` to `f` within the loop.
    let row_start_sb = frame_hdr.tiling.row_start_sb.clone();
    for (tile_row, (sbh_start_end, ts)) in iter::zip(
        row_start_sb[..rows + 1].windows(2),
        slice::from_raw_parts_mut(f.ts, rows * cols).chunks_exact_mut(cols),
    )
    .enumerate()
    {
        // Needed until #[feature(array_windows)] stabilizes; it should hopefully optimize out.
        let [sbh_start, sbh_end] = <[u16; 2]>::try_from(sbh_start_end).unwrap();

        let sbh_end = cmp::min(sbh_end.into(), f.sbh);

        for sby in sbh_start.into()..sbh_end {
            let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
            let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
            t.by = sby << 4 + seq_hdr.sb128;
            let by_end = t.by + f.sb_step >> 1;
            if frame_hdr.use_ref_frame_mvs != 0 {
                ((*f.c).refmvs_dsp.load_tmvs).expect("non-null function pointer")(
                    &mut f.rf,
                    tile_row as c_int,
                    0,
                    f.bw >> 1,
                    t.by >> 1,
                    by_end,
                );
            }
            for tile in &mut ts[..] {
                t.ts = tile;
                rav1d_decode_tile_sbrow(t).map_err(|()| EINVAL)?;
            }
            if frame_hdr.frame_type.is_inter_or_switch() {
                rav1d_refmvs_save_tmvs(
                    &(*f.c).refmvs_dsp,
                    &mut t.rt,
                    0,
                    f.bw >> 1,
                    t.by >> 1,
                    by_end,
                );
            }

            // loopfilter + cdef + restoration
            (f.bd_fn.filter_sbrow)(f, sby);
        }
    }

    Ok(())
}

pub(crate) unsafe fn rav1d_decode_frame_exit(f: &mut Rav1dFrameContext, retval: Rav1dResult) {
    let c = &*f.c;
    if !f.sr_cur.p.data[0].is_null() {
        f.task_thread.error = 0;
    }
    if c.n_fc > 1 && retval.is_err() && !f.frame_thread.cf.is_null() {
        slice::from_raw_parts_mut(
            f.frame_thread.cf.cast::<u8>(),
            usize::try_from(f.frame_thread.cf_sz).unwrap() * 128 * 128 / 2,
        )
        .fill(0);
    }
    // TODO(kkysen) use array::zip when stable
    for i in 0..7 {
        if f.refp[i].p.frame_hdr.is_some() {
            rav1d_thread_picture_unref(&mut f.refp[i]);
        }
        rav1d_ref_dec(&mut f.ref_mvs_ref[i]);
    }
    rav1d_picture_unref_internal(&mut f.cur);
    rav1d_thread_picture_unref(&mut f.sr_cur);
    rav1d_cdf_thread_unref(&mut f.in_cdf);
    if let Some(frame_hdr) = &f.frame_hdr {
        if frame_hdr.refresh_context != 0 {
            if !f.out_cdf.progress.is_null() {
                ::core::intrinsics::atomic_store_seqcst(
                    f.out_cdf.progress,
                    if retval.is_ok() { 1 } else { TILE_ERROR as u32 },
                );
            }
            rav1d_cdf_thread_unref(&mut f.out_cdf);
        }
    }

    rav1d_ref_dec(&mut f.cur_segmap_ref);
    rav1d_ref_dec(&mut f.prev_segmap_ref);
    rav1d_ref_dec(&mut f.mvs_ref);
    let _ = mem::take(&mut f.seq_hdr);
    let _ = mem::take(&mut f.frame_hdr);
    for tile in &mut f.tiles {
        rav1d_data_unref_internal(&mut tile.data);
    }
    f.tiles.clear();
    f.task_thread.retval = retval;
}

pub(crate) unsafe fn rav1d_decode_frame(f: &mut Rav1dFrameContext) -> Rav1dResult {
    assert!((*f.c).n_fc == 1);
    // if n_tc > 1 (but n_fc == 1), we could run init/exit in the task
    // threads also. Not sure it makes a measurable difference.
    let mut res = rav1d_decode_frame_init(f);
    if res.is_ok() {
        res = rav1d_decode_frame_init_cdf(f);
    }
    // wait until all threads have completed
    if res.is_ok() {
        if (*f.c).n_tc > 1 {
            res = rav1d_task_create_tile_sbrow(f, 0, 1);
            pthread_mutex_lock(&mut (*f.task_thread.ttd).lock);
            pthread_cond_signal(&mut (*f.task_thread.ttd).cond);
            if res.is_ok() {
                while f.task_thread.done[0] == 0
                // TODO(kkysen) Make `.task_counter` an `AtomicI32`, but that requires recursively removing `impl Copy`s.
                    || (*(addr_of_mut!(f.task_thread.task_counter) as *mut AtomicI32))
                        .load(Ordering::SeqCst)
                        > 0
                {
                    pthread_cond_wait(&mut f.task_thread.cond, &mut (*f.task_thread.ttd).lock);
                }
            }
            pthread_mutex_unlock(&mut (*f.task_thread.ttd).lock);
            res = f.task_thread.retval;
        } else {
            res = rav1d_decode_frame_main(f);
            let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
            if res.is_ok() && frame_hdr.refresh_context != 0 && f.task_thread.update_set {
                rav1d_cdf_thread_update(
                    frame_hdr,
                    f.out_cdf.data.cdf,
                    &mut (*f.ts.offset(frame_hdr.tiling.update as isize)).cdf,
                );
            }
        }
    }
    rav1d_decode_frame_exit(f, res);
    res
}

fn get_upscale_x0(in_w: c_int, out_w: c_int, step: c_int) -> c_int {
    let err = out_w * step - (in_w << 14);
    let x0 = (-(out_w - in_w << 13) + (out_w >> 1)) / out_w + 128 - err / 2;
    x0 & 0x3fff
}

pub unsafe fn rav1d_submit_frame(c: &mut Rav1dContext) -> Rav1dResult {
    // wait for c->out_delayed[next] and move into c->out if visible
    let (f, out_delayed) = if c.n_fc > 1 {
        pthread_mutex_lock(&mut c.task_thread.lock);
        let next = c.frame_thread.next;
        c.frame_thread.next += 1;
        if c.frame_thread.next == c.n_fc {
            c.frame_thread.next = 0;
        }

        let f = &mut *c.fc.offset(next as isize);
        while !f.tiles.is_empty() {
            pthread_cond_wait(&mut f.task_thread.cond, &mut c.task_thread.lock);
        }
        let out_delayed = &mut *c.frame_thread.out_delayed.offset(next as isize);
        if !out_delayed.p.data[0].is_null()
            || ::core::intrinsics::atomic_load_seqcst(&mut f.task_thread.error as *mut atomic_int)
                != 0
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
        let error = f.task_thread.retval;
        if error.is_err() {
            f.task_thread.retval = Ok(());
            c.cached_error = error;
            c.cached_error_props = out_delayed.p.m.clone();
            rav1d_thread_picture_unref(out_delayed);
        } else if !out_delayed.p.data[0].is_null() {
            let progress = (*out_delayed.progress)[1].load(Ordering::Relaxed);
            if (out_delayed.visible || c.output_invisible_frames) && progress != FRAME_ERROR {
                rav1d_thread_picture_ref(&mut c.out, out_delayed);
                c.event_flags |= out_delayed.flags.into();
            }
            rav1d_thread_picture_unref(out_delayed);
        }
        (f, out_delayed as *mut _)
    } else {
        (&mut *c.fc, ptr::null_mut())
    };

    f.seq_hdr = c.seq_hdr.clone();
    f.frame_hdr = mem::take(&mut c.frame_hdr);
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    f.dsp = &mut c.dsp[seq_hdr.hbd as usize];

    let bpc = 8 + 2 * seq_hdr.hbd;

    unsafe fn on_error(
        f: &mut Rav1dFrameContext,
        c: &mut Rav1dContext,
        out_delayed: *mut Rav1dThreadPicture,
    ) {
        f.task_thread.error = 1;
        rav1d_cdf_thread_unref(&mut f.in_cdf);
        if f.frame_hdr.as_ref().unwrap().refresh_context != 0 {
            rav1d_cdf_thread_unref(&mut f.out_cdf);
        }
        for i in 0..7 {
            if f.refp[i].p.frame_hdr.is_some() {
                rav1d_thread_picture_unref(&mut f.refp[i]);
            }
            rav1d_ref_dec(&mut f.ref_mvs_ref[i]);
        }
        if c.n_fc == 1 {
            rav1d_thread_picture_unref(&mut c.out);
        } else {
            rav1d_thread_picture_unref(out_delayed);
        }
        rav1d_picture_unref_internal(&mut f.cur);
        rav1d_thread_picture_unref(&mut f.sr_cur);
        rav1d_ref_dec(&mut f.mvs_ref);
        let _ = mem::take(&mut f.seq_hdr);
        let _ = mem::take(&mut f.frame_hdr);
        c.cached_error_props = c.in_0.m.clone();

        for mut tile in f.tiles.drain(..) {
            rav1d_data_unref_internal(&mut tile.data);
        }

        if c.n_fc > 1 {
            pthread_mutex_unlock(&mut c.task_thread.lock);
        }
    }

    // TODO(kkysen) Rather than lazy initializing this,
    // we should probably initialize all the fn ptrs
    // when `c` is allocated during [`rav1d_open`].
    if !(*f.dsp).initialized {
        let dsp = &mut c.dsp[seq_hdr.hbd as usize];
        dsp.initialized = true;

        match bpc {
            #[cfg(feature = "bitdepth_8")]
            8 => {
                rav1d_cdef_dsp_init_8bpc(&mut dsp.cdef);
                rav1d_intra_pred_dsp_init::<BitDepth8>(&mut dsp.ipred);
                rav1d_itx_dsp_init_8bpc(&mut dsp.itx, bpc);
                rav1d_loop_filter_dsp_init::<BitDepth8>(&mut dsp.lf);
                rav1d_loop_restoration_dsp_init::<BitDepth8>(&mut dsp.lr, bpc);
                rav1d_mc_dsp_init::<BitDepth8>(&mut dsp.mc);
                dsp.fg = Rav1dFilmGrainDSPContext::new::<BitDepth8>();
            }
            #[cfg(feature = "bitdepth_16")]
            10 | 12 => {
                rav1d_cdef_dsp_init_16bpc(&mut dsp.cdef);
                rav1d_intra_pred_dsp_init::<BitDepth16>(&mut dsp.ipred);
                rav1d_itx_dsp_init_16bpc(&mut dsp.itx, bpc);
                rav1d_loop_filter_dsp_init::<BitDepth16>(&mut dsp.lf);
                rav1d_loop_restoration_dsp_init::<BitDepth16>(&mut dsp.lr, bpc);
                rav1d_mc_dsp_init::<BitDepth16>(&mut dsp.mc);
                dsp.fg = Rav1dFilmGrainDSPContext::new::<BitDepth16>();
            }
            _ => {
                writeln!(
                    c.logger,
                    "Compiled without support for {}-bit decoding",
                    8 + 2 * seq_hdr.hbd
                );
                on_error(f, c, out_delayed);
                return Err(ENOPROTOOPT);
            }
        }
    }
    if seq_hdr.hbd == 0 {
        #[cfg(feature = "bitdepth_8")]
        {
            f.bd_fn = Rav1dFrameContext_bd_fn::new::<BitDepth8>();
        }
    } else {
        #[cfg(feature = "bitdepth_16")]
        {
            f.bd_fn = Rav1dFrameContext_bd_fn::new::<BitDepth16>();
        }
    }

    fn scale_fac(ref_sz: i32, this_sz: i32) -> i32 {
        ((ref_sz << 14) + (this_sz >> 1)) / this_sz
    }

    let mut ref_coded_width = <[i32; 7]>::default();
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    if frame_hdr.frame_type.is_inter_or_switch() {
        if frame_hdr.primary_ref_frame != RAV1D_PRIMARY_REF_NONE {
            let pri_ref = frame_hdr.refidx[frame_hdr.primary_ref_frame as usize] as usize;
            if c.refs[pri_ref].p.p.data[0].is_null() {
                on_error(f, c, out_delayed);
                return Err(EINVAL);
            }
        }
        for i in 0..7 {
            let refidx = frame_hdr.refidx[i] as usize;
            if c.refs[refidx].p.p.data[0].is_null()
                || (frame_hdr.size.width[0] * 2) < c.refs[refidx].p.p.p.w
                || (frame_hdr.size.height * 2) < c.refs[refidx].p.p.p.h
                || frame_hdr.size.width[0] > c.refs[refidx].p.p.p.w * 16
                || frame_hdr.size.height > c.refs[refidx].p.p.p.h * 16
                || seq_hdr.layout != c.refs[refidx].p.p.p.layout
                || bpc != c.refs[refidx].p.p.p.bpc
            {
                for j in 0..i {
                    rav1d_thread_picture_unref(&mut f.refp[j]);
                }
                on_error(f, c, out_delayed);
                return Err(EINVAL);
            }
            rav1d_thread_picture_ref(&mut f.refp[i], &mut c.refs[refidx].p);
            ref_coded_width[i] = c.refs[refidx].p.p.frame_hdr.as_ref().unwrap().size.width[0];
            if frame_hdr.size.width[0] != c.refs[refidx].p.p.p.w
                || frame_hdr.size.height != c.refs[refidx].p.p.p.h
            {
                f.svc[i][0].scale = scale_fac(c.refs[refidx].p.p.p.w, frame_hdr.size.width[0]);
                f.svc[i][1].scale = scale_fac(c.refs[refidx].p.p.p.h, frame_hdr.size.height);
                f.svc[i][0].step = f.svc[i][0].scale + 8 >> 4;
                f.svc[i][1].step = f.svc[i][1].scale + 8 >> 4;
            } else {
                f.svc[i][1].scale = 0;
                f.svc[i][0].scale = f.svc[i][1].scale;
            }
            f.gmv_warp_allowed[i] = (frame_hdr.gmv[i].r#type > RAV1D_WM_TYPE_TRANSLATION
                && frame_hdr.force_integer_mv == 0
                && !rav1d_get_shear_params(&frame_hdr.gmv[i])
                && f.svc[i][0].scale == 0) as u8;
        }
    }

    // setup entropy
    if frame_hdr.primary_ref_frame == RAV1D_PRIMARY_REF_NONE {
        rav1d_cdf_thread_init_static(&mut f.in_cdf, frame_hdr.quant.yac);
    } else {
        let pri_ref = frame_hdr.refidx[frame_hdr.primary_ref_frame as usize] as usize;
        rav1d_cdf_thread_ref(&mut f.in_cdf, &mut c.cdf[pri_ref]);
    }
    if frame_hdr.refresh_context != 0 {
        let res = rav1d_cdf_thread_alloc(c, &mut f.out_cdf, (c.n_fc > 1) as c_int);
        if res.is_err() {
            on_error(f, c, out_delayed);
            return res;
        }
    }

    // FIXME qsort so tiles are in order (for frame threading)
    f.tiles.clear();
    mem::swap(&mut f.tiles, &mut c.tiles);

    // allocate frame
    let res = rav1d_thread_picture_alloc(c, f, bpc);
    if res.is_err() {
        on_error(f, c, out_delayed);
        return res;
    }

    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();

    if frame_hdr.size.width[0] != frame_hdr.size.width[1] {
        let res = rav1d_picture_alloc_copy(c, &mut f.cur, frame_hdr.size.width[0], &mut f.sr_cur.p);
        if res.is_err() {
            on_error(f, c, out_delayed);
            return res;
        }
    } else {
        rav1d_picture_ref(&mut f.cur, &mut f.sr_cur.p);
    }
    if frame_hdr.size.width[0] != frame_hdr.size.width[1] {
        f.resize_step[0] = scale_fac(f.cur.p.w, f.sr_cur.p.p.w);
        let ss_hor = (f.cur.p.layout != Rav1dPixelLayout::I444) as c_int;
        let in_cw = f.cur.p.w + ss_hor >> ss_hor;
        let out_cw = f.sr_cur.p.p.w + ss_hor >> ss_hor;
        f.resize_step[1] = scale_fac(in_cw, out_cw);
        f.resize_start[0] = get_upscale_x0(f.cur.p.w, f.sr_cur.p.p.w, f.resize_step[0]);
        f.resize_start[1] = get_upscale_x0(in_cw, out_cw, f.resize_step[1]);
    }

    // move f->cur into output queue
    if c.n_fc == 1 {
        if frame_hdr.show_frame != 0 || c.output_invisible_frames {
            rav1d_thread_picture_ref(&mut c.out, &mut f.sr_cur);
            c.event_flags |= f.sr_cur.flags.into();
        }
    } else {
        rav1d_thread_picture_ref(out_delayed, &mut f.sr_cur);
    }

    f.w4 = frame_hdr.size.width[0] + 3 >> 2;
    f.h4 = frame_hdr.size.height + 3 >> 2;
    f.bw = (frame_hdr.size.width[0] + 7 >> 3) << 1;
    f.bh = (frame_hdr.size.height + 7 >> 3) << 1;
    f.sb128w = f.bw + 31 >> 5;
    f.sb128h = f.bh + 31 >> 5;
    f.sb_shift = 4 + seq_hdr.sb128;
    f.sb_step = 16 << seq_hdr.sb128;
    f.sbh = f.bh + f.sb_step - 1 >> f.sb_shift;
    f.b4_stride = (f.bw + 31 & !31) as ptrdiff_t;
    f.bitdepth_max = (1 << f.cur.p.bpc) - 1;
    *&mut f.task_thread.error = 0;
    let uses_2pass = (c.n_fc > 1) as c_int;
    let cols = frame_hdr.tiling.cols;
    let rows = frame_hdr.tiling.rows;
    ::core::intrinsics::atomic_store_seqcst(
        &mut f.task_thread.task_counter,
        cols * rows + f.sbh << uses_2pass,
    );

    // ref_mvs
    if frame_hdr.frame_type.is_inter_or_switch() || frame_hdr.allow_intrabc != 0 {
        f.mvs_ref = rav1d_ref_create_using_pool(
            c.refmvs_pool,
            ::core::mem::size_of::<refmvs_temporal_block>()
                * f.sb128h as usize
                * 16
                * (f.b4_stride >> 1) as usize,
        );
        if f.mvs_ref.is_null() {
            on_error(f, c, out_delayed);
            return Err(ENOMEM);
        }
        f.mvs = (*f.mvs_ref).data.cast::<refmvs_temporal_block>();
        if frame_hdr.allow_intrabc == 0 {
            for i in 0..7 {
                f.refpoc[i] = f.refp[i].p.frame_hdr.as_ref().unwrap().frame_offset as c_uint;
            }
        } else {
            f.refpoc.fill(0);
        }
        if frame_hdr.use_ref_frame_mvs != 0 {
            for i in 0..7 {
                let refidx = frame_hdr.refidx[i] as usize;
                let ref_w = (ref_coded_width[i] + 7 >> 3) << 1;
                let ref_h = (f.refp[i].p.p.h + 7 >> 3) << 1;
                if !c.refs[refidx].refmvs.is_null() && ref_w == f.bw && ref_h == f.bh {
                    f.ref_mvs_ref[i] = c.refs[refidx].refmvs;
                    rav1d_ref_inc(f.ref_mvs_ref[i]);
                    f.ref_mvs[i] = (*c.refs[refidx].refmvs)
                        .data
                        .cast::<refmvs_temporal_block>();
                } else {
                    f.ref_mvs[i] = ptr::null_mut();
                    f.ref_mvs_ref[i] = ptr::null_mut();
                }
                f.refrefpoc[i] = c.refs[refidx].refpoc;
            }
        } else {
            f.ref_mvs_ref.fill_with(ptr::null_mut);
        }
    } else {
        f.mvs_ref = ptr::null_mut();
        f.ref_mvs_ref.fill_with(ptr::null_mut);
    }

    // segmap
    if frame_hdr.segmentation.enabled != 0 {
        // By default, the previous segmentation map is not initialised.
        f.prev_segmap_ref = ptr::null_mut();
        f.prev_segmap = ptr::null();

        // We might need a previous frame's segmentation map.
        // This happens if there is either no update or a temporal update.
        if frame_hdr.segmentation.temporal != 0 || frame_hdr.segmentation.update_map == 0 {
            let pri_ref = frame_hdr.primary_ref_frame as usize;
            assert!(pri_ref != RAV1D_PRIMARY_REF_NONE as usize);
            let ref_w = (ref_coded_width[pri_ref] + 7 >> 3) << 1;
            let ref_h = (f.refp[pri_ref].p.p.h + 7 >> 3) << 1;
            if ref_w == f.bw && ref_h == f.bh {
                f.prev_segmap_ref = c.refs[frame_hdr.refidx[pri_ref] as usize].segmap;
                if !f.prev_segmap_ref.is_null() {
                    rav1d_ref_inc(f.prev_segmap_ref);
                    f.prev_segmap = (*f.prev_segmap_ref).data.cast::<u8>();
                }
            }
        }

        if frame_hdr.segmentation.update_map != 0 {
            // We're updating an existing map,
            // but need somewhere to put the new values.
            // Allocate them here (the data actually gets set elsewhere).
            f.cur_segmap_ref = rav1d_ref_create_using_pool(
                c.segmap_pool,
                ::core::mem::size_of::<u8>() * f.b4_stride as usize * 32 * f.sb128h as usize,
            );
            if f.cur_segmap_ref.is_null() {
                rav1d_ref_dec(&mut f.prev_segmap_ref);
                on_error(f, c, out_delayed);
                return Err(ENOMEM);
            }
            f.cur_segmap = (*f.cur_segmap_ref).data.cast::<u8>();
        } else if !f.prev_segmap_ref.is_null() {
            // We're not updating an existing map,
            // and we have a valid reference. Use that.
            f.cur_segmap_ref = f.prev_segmap_ref;
            rav1d_ref_inc(f.cur_segmap_ref);
            f.cur_segmap = (*f.prev_segmap_ref).data.cast::<u8>();
        } else {
            // We need to make a new map. Allocate one here and zero it out.
            let segmap_size =
                ::core::mem::size_of::<u8>() * f.b4_stride as usize * 32 * f.sb128h as usize;
            f.cur_segmap_ref = rav1d_ref_create_using_pool(c.segmap_pool, segmap_size);
            if f.cur_segmap_ref.is_null() {
                on_error(f, c, out_delayed);
                return Err(ENOMEM);
            }
            f.cur_segmap = (*f.cur_segmap_ref).data.cast::<u8>();
            slice::from_raw_parts_mut(f.cur_segmap, segmap_size).fill(0);
        }
    } else {
        f.cur_segmap = ptr::null_mut();
        f.cur_segmap_ref = ptr::null_mut();
        f.prev_segmap_ref = ptr::null_mut();
    }

    // update references etc.
    let refresh_frame_flags = frame_hdr.refresh_frame_flags as c_uint;
    for i in 0..8 {
        if refresh_frame_flags & (1 << i) != 0 {
            if c.refs[i].p.p.frame_hdr.is_some() {
                rav1d_thread_picture_unref(&mut c.refs[i].p);
            }
            rav1d_thread_picture_ref(&mut c.refs[i].p, &mut f.sr_cur);

            rav1d_cdf_thread_unref(&mut c.cdf[i]);
            if frame_hdr.refresh_context != 0 {
                rav1d_cdf_thread_ref(&mut c.cdf[i], &mut f.out_cdf);
            } else {
                rav1d_cdf_thread_ref(&mut c.cdf[i], &mut f.in_cdf);
            }

            rav1d_ref_dec(&mut c.refs[i].segmap);
            c.refs[i].segmap = f.cur_segmap_ref;
            if !f.cur_segmap_ref.is_null() {
                rav1d_ref_inc(f.cur_segmap_ref);
            }
            rav1d_ref_dec(&mut c.refs[i].refmvs);
            if frame_hdr.allow_intrabc == 0 {
                c.refs[i].refmvs = f.mvs_ref;
                if !f.mvs_ref.is_null() {
                    rav1d_ref_inc(f.mvs_ref);
                }
            }
            c.refs[i].refpoc = f.refpoc;
        }
    }

    if c.n_fc == 1 {
        let res = rav1d_decode_frame(f);
        if res.is_err() {
            rav1d_thread_picture_unref(&mut c.out);
            for i in 0..8 {
                if refresh_frame_flags & (1 << i) != 0 {
                    if c.refs[i].p.p.frame_hdr.is_some() {
                        rav1d_thread_picture_unref(&mut c.refs[i].p);
                    }
                    rav1d_cdf_thread_unref(&mut c.cdf[i]);
                    rav1d_ref_dec(&mut c.refs[i].segmap);
                    rav1d_ref_dec(&mut c.refs[i].refmvs);
                }
            }
            on_error(f, c, out_delayed);
            return res;
        }
    } else {
        rav1d_task_frame_init(f);
        pthread_mutex_unlock(&mut c.task_thread.lock);
    }

    Ok(())
}
