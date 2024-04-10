use crate::include::common::attributes::ctz;
use crate::include::common::bitdepth::BitDepth16;
use crate::include::common::bitdepth::BitDepth8;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::bitdepth::BPC;
use crate::include::common::intops::apply_sign64;
use crate::include::common::intops::iclip;
use crate::include::common::intops::iclip_u8;
use crate::include::dav1d::headers::Dav1dFilterMode;
use crate::include::dav1d::headers::Rav1dFilterMode;
use crate::include::dav1d::headers::Rav1dFrameHeader;
use crate::include::dav1d::headers::Rav1dFrameHeader_tiling;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::include::dav1d::headers::Rav1dRestorationType;
use crate::include::dav1d::headers::Rav1dSequenceHeader;
use crate::include::dav1d::headers::Rav1dTxfmMode;
use crate::include::dav1d::headers::Rav1dWarpedMotionParams;
use crate::include::dav1d::headers::Rav1dWarpedMotionType;
use crate::include::dav1d::headers::SgrIdx;
use crate::include::dav1d::headers::RAV1D_MAX_SEGMENTS;
use crate::include::dav1d::headers::RAV1D_PRIMARY_REF_NONE;
use crate::src::align::Align16;
use crate::src::cdef::rav1d_cdef_dsp_init;
use crate::src::cdf::rav1d_cdf_thread_alloc;
use crate::src::cdf::rav1d_cdf_thread_copy;
use crate::src::cdf::rav1d_cdf_thread_init_static;
use crate::src::cdf::rav1d_cdf_thread_update;
use crate::src::cdf::CdfMvComponent;
use crate::src::cdf::CdfMvContext;
use crate::src::ctx::CaseSet;
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
use crate::src::internal::Bxy;
use crate::src::internal::Rav1dContext;
use crate::src::internal::Rav1dContextTaskType;
use crate::src::internal::Rav1dFrameData;
use crate::src::internal::Rav1dTaskContext;
use crate::src::internal::Rav1dTaskContext_scratch_pal;
use crate::src::internal::Rav1dTileState;
use crate::src::internal::ScalableMotionParams;
use crate::src::internal::TileStateRef;
use crate::src::intra_edge::EdgeFlags;
use crate::src::intra_edge::EdgeIndex;
use crate::src::intra_edge::IntraEdges;
use crate::src::ipred::rav1d_intra_pred_dsp_init;
use crate::src::itx::rav1d_itx_dsp_init;
use crate::src::levels::mv;
use crate::src::levels::Av1Block;
use crate::src::levels::BlockLevel;
use crate::src::levels::BlockPartition;
use crate::src::levels::BlockSize;
use crate::src::levels::CompInterType;
use crate::src::levels::DrlProximity;
use crate::src::levels::Filter2d;
use crate::src::levels::InterIntraPredMode;
use crate::src::levels::InterIntraType;
use crate::src::levels::MVJoint;
use crate::src::levels::MotionMode;
use crate::src::levels::RectTxfmSize;
use crate::src::levels::TxfmSize;
use crate::src::levels::CFL_PRED;
use crate::src::levels::DC_PRED;
use crate::src::levels::FILTER_PRED;
use crate::src::levels::GLOBALMV;
use crate::src::levels::GLOBALMV_GLOBALMV;
use crate::src::levels::NEARESTMV;
use crate::src::levels::NEARESTMV_NEARESTMV;
use crate::src::levels::NEARMV;
use crate::src::levels::NEWMV;
use crate::src::levels::NEWMV_NEWMV;
use crate::src::levels::N_COMP_INTER_PRED_MODES;
use crate::src::levels::N_INTRA_PRED_MODES;
use crate::src::levels::N_RECT_TX_SIZES;
use crate::src::levels::N_UV_INTRA_PRED_MODES;
use crate::src::levels::TX_4X4;
use crate::src::levels::TX_64X64;
use crate::src::levels::TX_8X8;
use crate::src::levels::VERT_LEFT_PRED;
use crate::src::levels::VERT_PRED;
use crate::src::lf_mask::rav1d_calc_eih;
use crate::src::lf_mask::rav1d_calc_lf_values;
use crate::src::lf_mask::rav1d_create_lf_mask_inter;
use crate::src::lf_mask::rav1d_create_lf_mask_intra;
use crate::src::lf_mask::Av1RestorationUnit;
use crate::src::log::Rav1dLog as _;
use crate::src::loopfilter::rav1d_loop_filter_dsp_init;
use crate::src::looprestoration::rav1d_loop_restoration_dsp_init;
use crate::src::mc::rav1d_mc_dsp_init;
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
use crate::src::recon::debug_block_info;
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
use libc::ptrdiff_t;
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
use strum::EnumCount;

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
    f: &Rav1dFrameData,
    mv_comp: &mut CdfMvComponent,
    have_fp: bool,
) -> c_int {
    let ts = &mut *f.ts.offset(t.ts as isize);
    let have_hp = f.frame_hdr.as_ref().unwrap().hp;
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
    f: &Rav1dFrameData,
    ref_mv: &mut mv,
    mv_cdf: &mut CdfMvContext,
    have_fp: bool,
) {
    let ts = &mut *f.ts.offset(t.ts as isize);
    match MVJoint::from_repr(rav1d_msac_decode_symbol_adapt4(
        &mut ts.msac,
        &mut ts.cdf.mv.joint.0,
        MVJoint::COUNT - 1,
    ) as usize)
    .expect("valid variant")
    {
        MVJoint::HV => {
            ref_mv.y += read_mv_component_diff(t, f, &mut mv_cdf.comp[0], have_fp) as i16;
            ref_mv.x += read_mv_component_diff(t, f, &mut mv_cdf.comp[1], have_fp) as i16;
        }
        MVJoint::H => {
            ref_mv.x += read_mv_component_diff(t, f, &mut mv_cdf.comp[1], have_fp) as i16;
        }
        MVJoint::V => {
            ref_mv.y += read_mv_component_diff(t, f, &mut mv_cdf.comp[0], have_fp) as i16;
        }
        MVJoint::Zero => {}
    };
}

unsafe fn read_tx_tree(
    t: &mut Rav1dTaskContext,
    f: &Rav1dFrameData,
    from: RectTxfmSize,
    depth: c_int,
    masks: &mut [u16; 2],
    x_off: usize,
    y_off: usize,
) {
    let bx4 = t.b.x & 31;
    let by4 = t.b.y & 31;
    let t_dim = &dav1d_txfm_dimensions[from as usize];
    let txw = t_dim.lw;
    let txh = t_dim.lh;
    let is_split;
    let ts = &mut *f.ts.offset(t.ts as isize);

    if depth < 2 && from > TX_4X4 {
        let cat = 2 * (TX_64X64 as c_int - t_dim.max as c_int) - depth;
        let a = ((*t.a).tx.0[bx4 as usize] < txw) as c_int;
        let l = (t.l.tx.0[by4 as usize] < txh) as c_int;

        is_split = rav1d_msac_decode_bool_adapt(
            &mut ts.msac,
            &mut ts.cdf.m.txpart[cat as usize][(a + l) as usize],
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

        read_tx_tree(t, f, sub, depth + 1, masks, x_off * 2 + 0, y_off * 2 + 0);
        t.b.x += txsw;
        if txw >= txh && t.b.x < f.bw {
            read_tx_tree(t, f, sub, depth + 1, masks, x_off * 2 + 1, y_off * 2 + 0);
        }
        t.b.x -= txsw;
        t.b.y += txsh;
        if txh >= txw && t.b.y < f.bh {
            read_tx_tree(t, f, sub, depth + 1, masks, x_off * 2 + 0, y_off * 2 + 1);
            t.b.x += txsw;
            if txw >= txh && t.b.x < f.bw {
                read_tx_tree(t, f, sub, depth + 1, masks, x_off * 2 + 1, y_off * 2 + 1);
            }
            t.b.x -= txsw;
        }
        t.b.y -= txsh;
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
    f: &Rav1dFrameData,
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
    let r = &t.rt.r[(t.b.y as usize & 31) + 5 - 1..];
    let mut count = 0;
    let ts = &*f.ts.offset(t.ts as isize);
    let mut have_topleft = have_top && have_left;
    let mut have_topright = cmp::max(bw4, bh4) < 32
        && have_top
        && t.b.x + bw4 < ts.tiling.col_end
        && intra_edge_flags.contains(EdgeFlags::I444_TOP_HAS_RIGHT);

    let bs = |rp: &refmvs_block| dav1d_block_dimensions[rp.0.bs as usize];
    let matches = |rp: &refmvs_block| rp.0.r#ref.r#ref[0] == r#ref + 1 && rp.0.r#ref.r#ref[1] == -1;

    if have_top {
        let mut i = r[0] + t.b.x as usize;
        let r2 = &f.rf.r[i];
        if matches(r2) {
            masks[0] |= 1;
            count = 1;
        }
        let mut aw4 = bs(r2)[0] as c_int;
        if aw4 >= bw4 {
            let off = t.b.x & aw4 - 1;
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
                i += aw4 as usize;
                let r2 = &f.rf.r[i];
                if matches(r2) {
                    masks[0] |= mask;
                    count += 1;
                    if count >= 8 {
                        return;
                    }
                }
                aw4 = bs(r2)[0] as c_int;
                mask <<= aw4;
                x += aw4;
            }
        }
    }
    if have_left {
        let get_r2 = |i| &f.rf.r[r[i] + t.b.x as usize - 1];

        let mut i = 1;
        let r2 = get_r2(i);
        if matches(r2) {
            masks[1] |= 1;
            count += 1;
            if count >= 8 {
                return;
            }
        }
        let mut lh4 = bs(r2)[1] as c_int;
        if lh4 >= bh4 {
            if t.b.y & lh4 - 1 != 0 {
                have_topleft = false;
            }
        } else {
            let mut mask = 1 << lh4;
            let mut y = lh4;
            while y < h4 {
                i += lh4 as usize;
                let r2 = get_r2(i);
                if matches(r2) {
                    masks[1] |= mask;
                    count += 1;
                    if count >= 8 {
                        return;
                    }
                }
                lh4 = bs(r2)[1] as c_int;
                mask <<= lh4;
                y += lh4;
            }
        }
    }
    if have_topleft && matches(&f.rf.r[r[0] + t.b.x as usize - 1]) {
        masks[1] |= 1 << 32;
        count += 1;
        if count >= 8 {
            return;
        }
    }
    if have_topright && matches(&f.rf.r[r[0] + t.b.x as usize + bw4 as usize]) {
        masks[0] |= 1 << 32;
    }
}

unsafe fn derive_warpmv(
    r: &[refmvs_block],
    t: &Rav1dTaskContext,
    bw4: c_int,
    bh4: c_int,
    masks: &[u64; 2],
    mv: mv,
    mut wmp: Rav1dWarpedMotionParams,
) -> Rav1dWarpedMotionParams {
    let mut pts = [[[0; 2 /* x, y */]; 2 /* in, out */]; 8];
    let mut np = 0;
    let rp = |i: i32, j: i32| {
        // Need to use a closure here vs. a slice because `i` can be negative
        // (and not just by a constant -1).
        // See `-off` below.
        let offset = (t.b.y & 31) + 5;
        &r[t.rt.r[(offset as isize + i as isize) as usize] + j as usize]
    };

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
        let off = t.b.x & bs(rp(-1, t.b.x))[0] as i32 - 1;
        np = add_sample(np, -off, 0, 1, -1, rp(-1, t.b.x));
    } else {
        let mut off = 0;
        let mut xmask = masks[0] as u32;
        while np < 8 && xmask != 0 {
            let tz = ctz(xmask);
            off += tz;
            xmask >>= tz;
            np = add_sample(np, off, 0, 1, -1, rp(-1, t.b.x + off));
            xmask &= !1;
        }
    }
    if np < 8 && masks[1] as u32 == 1 {
        let off = t.b.y & bs(rp(0, t.b.x - 1))[1] as i32 - 1;
        np = add_sample(np, 0, -off, -1, 1, rp(-off, t.b.x - 1));
    } else {
        let mut off = 0;
        let mut ymask = masks[1] as u32;
        while np < 8 && ymask != 0 {
            let tz = ctz(ymask);
            off += tz;
            ymask >>= tz;
            np = add_sample(np, 0, off, -1, 1, rp(off, t.b.x - 1));
            ymask &= !1;
        }
    }
    if np < 8 && masks[1] >> 32 != 0 {
        // top/left
        np = add_sample(np, 0, 0, -1, -1, rp(-1, t.b.x - 1));
    }
    if np < 8 && masks[0] >> 32 != 0 {
        // top/right
        np = add_sample(np, bw4, 0, 1, -1, rp(-1, t.b.x + bw4));
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

    wmp.r#type = if !rav1d_find_affine_int(&pts, ret, bw4, bh4, mv, &mut wmp, t.b.x, t.b.y)
        && !rav1d_get_shear_params(&mut wmp)
    {
        Rav1dWarpedMotionType::Affine
    } else {
        Rav1dWarpedMotionType::Identity
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
    f: &mut Rav1dFrameData,
    b: &mut Av1Block,
    bs: BlockSize,
    bx4: c_int,
    by4: c_int,
) {
    let b_dim = &dav1d_block_dimensions[bs as usize];
    let bw4 = b_dim[0] as usize;
    let bh4 = b_dim[1] as usize;

    // var-tx tree coding
    let mut tx_split = [0u16; 2];
    *b.max_ytx_mut() = dav1d_max_txfm_size_for_bs[bs as usize][0];
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    let txfm_mode = frame_hdr.txfm_mode;
    if b.skip == 0
        && (frame_hdr.segmentation.lossless[b.seg_id as usize] != 0
            || b.max_ytx() as TxfmSize == TX_4X4)
    {
        b.uvtx = TX_4X4 as u8;
        *b.max_ytx_mut() = b.uvtx;
        if txfm_mode == Rav1dTxfmMode::Switchable {
            CaseSet::<32, false>::many(
                [&mut t.l, &mut *t.a],
                [bh4 as usize, bw4 as usize],
                [by4 as usize, bx4 as usize],
                |case, dir| {
                    case.set(&mut dir.tx.0, TX_4X4);
                },
            );
        }
    } else if txfm_mode != Rav1dTxfmMode::Switchable || b.skip != 0 {
        if txfm_mode == Rav1dTxfmMode::Switchable {
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
                    f,
                    b.max_ytx() as RectTxfmSize,
                    0,
                    &mut tx_split,
                    x_off,
                    y_off,
                );
                // contexts are updated inside read_tx_tree()
                t.b.x += w as c_int;
            }
            t.b.x -= bw4 as c_int;
            t.b.y += h as c_int;
        }
        t.b.y -= bh4 as c_int;
        if debug_block_info!(f, t.b) {
            let ts = &*f.ts.offset(t.ts as isize);
            println!(
                "Post-vartxtree[{}/{}]: r={}",
                tx_split[0], tx_split[1], ts.msac.rng
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
    frame_hdr: &Rav1dFrameHeader,
    b: Bxy,
    w4: c_int,
    h4: c_int,
    // It's very difficult to make this safe (a slice),
    // as it comes from [`Dav1dFrameContext::prev_segmap`],
    // which is set to [`Dav1dFrameContext::prev_segmap_ref`],
    // which is a [`Dav1dRef`], which has no size and is refcounted.
    ref_seg_map: *const u8,
    stride: ptrdiff_t,
) -> u8 {
    assert!(frame_hdr.primary_ref_frame != RAV1D_PRIMARY_REF_NONE);

    // Need checked casts here because an overflowing cast
    // would give a too large `len` to [`std::slice::from_raw_parts`], which would UB.
    let w4 = usize::try_from(w4).unwrap();
    let h4 = usize::try_from(h4).unwrap();
    let stride = usize::try_from(stride).unwrap();

    let mut prev_seg_id = 8;
    let ref_seg_map = std::slice::from_raw_parts(
        ref_seg_map.offset(b.y as isize * stride as isize + b.x as isize),
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
    t: &Rav1dTaskContext,
    r: &mut [refmvs_block],
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
                b.interintra_type().map(|_| 0).unwrap_or(-1),
            ],
        },
        bs,
        mf: (mode == GLOBALMV && cmp::min(bw4, bh4) >= 2) as u8 | (mode == NEWMV) as u8 * 2,
    }));
    c.refmvs_dsp.splat_mv(r, &t.rt, &tmpl, t.b, bw4, bh4);
}

#[inline]
unsafe fn splat_intrabc_mv(
    c: &Rav1dContext,
    t: &Rav1dTaskContext,
    r: &mut [refmvs_block],
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
        bs,
        mf: 0,
    }));
    c.refmvs_dsp.splat_mv(r, &t.rt, &tmpl, t.b, bw4, bh4);
}

#[inline]
unsafe fn splat_tworef_mv(
    c: &Rav1dContext,
    t: &Rav1dTaskContext,
    r: &mut [refmvs_block],
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
        bs,
        mf: (mode == GLOBALMV_GLOBALMV) as u8 | (1 << mode & 0xbc != 0) as u8 * 2,
    }));
    c.refmvs_dsp.splat_mv(r, &t.rt, &tmpl, t.b, bw4, bh4);
}

#[inline]
unsafe fn splat_intraref(
    c: &Rav1dContext,
    t: &Rav1dTaskContext,
    r: &mut [refmvs_block],
    bs: BlockSize,
    bw4: usize,
    bh4: usize,
) {
    let tmpl = Align16(refmvs_block(refmvs_block_unaligned {
        mv: refmvs_mvpair {
            mv: [mv::INVALID, mv::ZERO],
        },
        r#ref: refmvs_refpair { r#ref: [0, -1] },
        bs,
        mf: 0,
    }));
    c.refmvs_dsp.splat_mv(r, &t.rt, &tmpl, t.b, bw4, bh4);
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
    let src_y = t.b.y * 4 + ((y + 4) << ss_ver);
    let mat5_y = mat[5] as i64 * src_y as i64 + mat[1] as i64;
    let mut x = 0;
    while x < b_dim[0] as c_int * h_mul {
        let src_x = t.b.x * 4 + ((x + 4) << ss_hor);
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
    layout: Rav1dPixelLayout,
    dst: &mut c_int,
    b_dim: &[u8; 4],
    wmp: &Rav1dWarpedMotionParams,
) {
    assert!(layout != Rav1dPixelLayout::I400);
    if layout == Rav1dPixelLayout::I444 {
        affine_lowest_px_luma(t, dst, b_dim, wmp);
    } else {
        affine_lowest_px(
            t,
            dst,
            b_dim,
            wmp,
            (layout & Rav1dPixelLayout::I420) as c_int,
            1,
        );
    };
}

unsafe fn obmc_lowest_px(
    r: &[refmvs_block],
    t: &mut Rav1dTaskContext,
    ts: &Rav1dTileState,
    layout: Rav1dPixelLayout,
    svc: &[[ScalableMotionParams; 2]; 7],
    dst: &mut [[c_int; 2]; 7],
    is_chroma: bool,
    b_dim: &[u8; 4],
    _bx4: c_int,
    _by4: c_int,
    w4: c_int,
    h4: c_int,
) {
    assert!(t.b.x & 1 == 0 && t.b.y & 1 == 0);
    let ri = &t.rt.r[(t.b.y as usize & 31) + 5 - 1..];
    let ss_ver = (is_chroma && layout == Rav1dPixelLayout::I420) as c_int;
    let ss_hor = (is_chroma && layout != Rav1dPixelLayout::I444) as c_int;
    let h_mul = 4 >> ss_hor;
    let v_mul = 4 >> ss_ver;
    if t.b.y > ts.tiling.row_start
        && (!is_chroma || b_dim[0] as c_int * h_mul + b_dim[1] as c_int * v_mul >= 16)
    {
        let mut i = 0;
        let mut x = 0;
        while x < w4 && i < cmp::min(b_dim[2] as c_int, 4) {
            let a_r = &r[ri[0] + t.b.x as usize + x as usize + 1];
            let a_b_dim = &dav1d_block_dimensions[a_r.0.bs as usize];
            if a_r.0.r#ref.r#ref[0] as c_int > 0 {
                let oh4 = cmp::min(b_dim[1] as c_int, 16) >> 1;
                mc_lowest_px(
                    &mut dst[a_r.0.r#ref.r#ref[0] as usize - 1][is_chroma as usize],
                    t.b.y,
                    oh4 * 3 + 3 >> 2,
                    a_r.0.mv.mv[0].y,
                    ss_ver,
                    &svc[a_r.0.r#ref.r#ref[0] as usize - 1][1],
                );
                i += 1;
            }
            x += cmp::max(a_b_dim[0] as c_int, 2);
        }
    }
    if t.b.x > ts.tiling.col_start {
        let mut i = 0;
        let mut y = 0;
        while y < h4 && i < cmp::min(b_dim[3] as c_int, 4) {
            let l_r = &r[ri[y as usize + 1 + 1] + t.b.x as usize - 1];
            let l_b_dim = &dav1d_block_dimensions[l_r.0.bs as usize];
            if l_r.0.r#ref.r#ref[0] as c_int > 0 {
                let oh4 = iclip(l_b_dim[1] as c_int, 2, b_dim[1] as c_int);
                mc_lowest_px(
                    &mut dst[l_r.0.r#ref.r#ref[0] as usize - 1][is_chroma as usize],
                    t.b.y + y,
                    oh4,
                    l_r.0.mv.mv[0].y,
                    ss_ver,
                    &svc[l_r.0.r#ref.r#ref[0] as usize - 1][1],
                );
                i += 1;
            }
            y += cmp::max(l_b_dim[1] as c_int, 2);
        }
    }
}

unsafe fn decode_b(
    c: &Rav1dContext,
    t: &mut Rav1dTaskContext,
    f: &mut Rav1dFrameData,
    bl: BlockLevel,
    bs: BlockSize,
    bp: BlockPartition,
    intra_edge_flags: EdgeFlags,
) -> Result<(), ()> {
    // Pull out the current block from Rav1dFrameData so that we can operate on
    // it without borrow check errors.
    let (mut b_mem, b_idx) = if t.frame_thread.pass != 0 {
        let b_idx = (t.b.y as isize * f.b4_stride + t.b.x as isize) as usize;
        (mem::take(&mut f.frame_thread.b[b_idx]), Some(b_idx))
    } else {
        (Default::default(), None)
    };
    let b = &mut b_mem;
    let res = decode_b_inner(c, t, f, bl, bs, bp, intra_edge_flags, b);
    if let Some(i) = b_idx {
        let _old_b = mem::replace(&mut f.frame_thread.b[i], b_mem);
        // TODO(SJC): We should be able to compare Av1Blocks, but there are C
        // unions in them.
        // assert_eq!(old_b, Default::default());
    }
    res
}

unsafe fn decode_b_inner(
    c: &Rav1dContext,
    t: &mut Rav1dTaskContext,
    f: &mut Rav1dFrameData,
    bl: BlockLevel,
    bs: BlockSize,
    bp: BlockPartition,
    intra_edge_flags: EdgeFlags,
    b: &mut Av1Block,
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

    let ts = &mut *f.ts.offset(t.ts as isize);
    let bd_fn = f.bd_fn();
    let b_dim = &dav1d_block_dimensions[bs as usize];
    let bx4 = t.b.x & 31;
    let by4 = t.b.y & 31;
    let ss_ver = (f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
    let ss_hor = (f.cur.p.layout != Rav1dPixelLayout::I444) as c_int;
    let cbx4 = bx4 >> ss_hor;
    let cby4 = by4 >> ss_ver;
    let bw4 = b_dim[0] as c_int;
    let bh4 = b_dim[1] as c_int;
    let w4 = cmp::min(bw4, f.bw - t.b.x);
    let h4 = cmp::min(bh4, f.bh - t.b.y);
    let cbw4 = bw4 + ss_hor >> ss_hor;
    let cbh4 = bh4 + ss_ver >> ss_ver;
    let have_left = t.b.x > ts.tiling.col_start;
    let have_top = t.b.y > ts.tiling.row_start;
    let has_chroma = f.cur.p.layout != Rav1dPixelLayout::I400
        && (bw4 > ss_hor || t.b.x & 1 != 0)
        && (bh4 > ss_ver || t.b.y & 1 != 0);
    let frame_type = f.frame_hdr.as_ref().unwrap().frame_type;

    if t.frame_thread.pass == 2 {
        if b.intra != 0 {
            bd_fn.recon_b_intra(f, t, bs, intra_edge_flags, b);

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
            if frame_type.is_inter_or_switch() {
                let r = &mut f.rf.r
                    [t.rt.r[(t.b.y as usize & 31) + 5 + bh4 as usize - 1] + t.b.x as usize..]
                    [..bw4 as usize];
                for block in r {
                    block.0.r#ref.r#ref[0] = 0;
                    block.0.bs = bs;
                }
                let rr = &t.rt.r[(t.b.y as usize & 31) + 5..][..bh4 as usize - 1];
                for r in rr {
                    let block = &mut f.rf.r[r + t.b.x as usize + bw4 as usize - 1];
                    block.0.r#ref.r#ref[0] = 0;
                    block.0.bs = bs;
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
            if frame_type.is_inter_or_switch() /* not intrabc */
                && b.comp_type().is_none()
                && b.motion_mode() == MotionMode::Warp
            {
                if b.matrix()[0] == i16::MIN {
                    t.warpmv.r#type = Rav1dWarpedMotionType::Identity;
                } else {
                    t.warpmv.r#type = Rav1dWarpedMotionType::Affine;
                    t.warpmv.matrix[2] = b.matrix()[0] as i32 + 0x10000;
                    t.warpmv.matrix[3] = b.matrix()[1] as i32;
                    t.warpmv.matrix[4] = b.matrix()[2] as i32;
                    t.warpmv.matrix[5] = b.matrix()[3] as i32 + 0x10000;
                    rav1d_set_affine_mv2d(bw4, bh4, *b.mv2d(), &mut t.warpmv, t.b.x, t.b.y);
                    rav1d_get_shear_params(&mut t.warpmv);
                    if debug_block_info!(f, t.b) {
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
            bd_fn.recon_b_inter(f, t, bs, b)?;

            let filter = &dav1d_filter_dir[b.filter2d() as usize];
            CaseSet::<32, false>::many(
                [&mut t.l, &mut *t.a],
                [bh4 as usize, bw4 as usize],
                [by4 as usize, bx4 as usize],
                |case, dir| {
                    case.set(&mut dir.filter.0[0], filter[0].into());
                    case.set(&mut dir.filter.0[1], filter[1].into());
                    case.set(&mut dir.intra.0, 0);
                },
            );

            if frame_type.is_inter_or_switch() {
                let r = &mut f.rf.r
                    [t.rt.r[(t.b.y as usize & 31) + 5 + bh4 as usize - 1] + t.b.x as usize..]
                    [..bw4 as usize];
                for block in r {
                    block.0.r#ref.r#ref[0] = b.r#ref()[0] + 1;
                    block.0.mv.mv[0] = b.mv()[0];
                    block.0.bs = bs;
                }
                let rr = &t.rt.r[(t.b.y as usize & 31) + 5..][..bh4 as usize - 1];
                for r in rr {
                    let block = &mut f.rf.r[r + t.b.x as usize + bw4 as usize - 1];
                    block.0.r#ref.r#ref[0] = b.r#ref()[0] + 1;
                    block.0.mv.mv[0] = b.mv()[0];
                    block.0.bs = bs;
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

    b.bl = bl;
    b.bp = bp;
    b.bs = bs as u8;

    let mut seg = None;

    // segment_id (if seg_feature for skip/ref/gmv is enabled)
    let mut seg_pred = false;
    let frame_hdr: &Rav1dFrameHeader = &f.frame_hdr.as_ref().unwrap();
    if frame_hdr.segmentation.enabled != 0 {
        if frame_hdr.segmentation.update_map == 0 {
            if !(f.prev_segmap).is_null() {
                let seg_id =
                    get_prev_frame_segid(frame_hdr, t.b, w4, h4, f.prev_segmap, f.b4_stride);
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
                        get_prev_frame_segid(frame_hdr, t.b, w4, h4, f.prev_segmap, f.b4_stride);
                    if seg_id >= RAV1D_MAX_SEGMENTS.into() {
                        return Err(());
                    }
                    b.seg_id = seg_id;
                } else {
                    b.seg_id = 0;
                }
            } else {
                let (pred_seg_id, seg_ctx) = get_cur_frame_segid(
                    t.b,
                    have_top,
                    have_left,
                    f.cur_segmap,
                    f.b4_stride as usize,
                );
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

            if debug_block_info!(f, t.b) {
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
        if debug_block_info!(f, t.b) {
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
        if debug_block_info!(f, t.b) {
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
                    get_prev_frame_segid(frame_hdr, t.b, w4, h4, f.prev_segmap, f.b4_stride);
                if seg_id >= RAV1D_MAX_SEGMENTS.into() {
                    return Err(());
                }
                b.seg_id = seg_id;
            } else {
                b.seg_id = 0;
            }
        } else {
            let (pred_seg_id, seg_ctx) =
                get_cur_frame_segid(t.b, have_top, have_left, f.cur_segmap, f.b4_stride as usize);
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

        if debug_block_info!(f, t.b) {
            println!("Post-segid[postskip;{}]: r={}", b.seg_id, ts.msac.rng);
        }
    }

    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    // cdef index
    if b.skip == 0 {
        let idx = if seq_hdr.sb128 != 0 {
            ((t.b.x & 16) >> 4) + ((t.b.y & 16) >> 3)
        } else {
            0
        } as usize;
        let cdef_idx = &f.lf.mask[t.lf_mask.unwrap()].cdef_idx;
        let cur_idx = t.cur_sb_cdef_idx + idx;
        if cdef_idx[cur_idx].load(Ordering::Relaxed) == -1 {
            let v = rav1d_msac_decode_bools(&mut ts.msac, frame_hdr.cdef.n_bits as c_uint) as i8;
            cdef_idx[cur_idx].store(v, Ordering::Relaxed);
            if bw4 > 16 {
                cdef_idx[cur_idx + 1].store(v, Ordering::Relaxed)
            }
            if bh4 > 16 {
                cdef_idx[cur_idx + 2].store(v, Ordering::Relaxed)
            }
            if bw4 == 32 && bh4 == 32 {
                cdef_idx[cur_idx + 3].store(v, Ordering::Relaxed)
            }

            if debug_block_info!(f, t.b) {
                println!(
                    "Post-cdef_idx[{}]: r={}",
                    cdef_idx[t.cur_sb_cdef_idx].load(Ordering::Relaxed),
                    ts.msac.rng
                );
            }
        }
    }

    // delta-q/lf
    let not_sb128 = (seq_hdr.sb128 == 0) as c_int;
    if t.b.x & (31 >> not_sb128) == 0 && t.b.y & (31 >> not_sb128) == 0 {
        let prev_qidx = ts.last_qidx;
        let have_delta_q = frame_hdr.delta.q.present != 0
            && (bs
                != (if seq_hdr.sb128 != 0 {
                    BlockSize::Bs128x128
                } else {
                    BlockSize::Bs64x64
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
            if have_delta_q && debug_block_info!(f, t.b) {
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
                    if have_delta_q && debug_block_info!(f, t.b) {
                        println!("Post-delta_lf[{}:{}]: r={}", i, delta_lf, ts.msac.rng);
                    }
                }
            }
        }
        if ts.last_qidx == frame_hdr.quant.yac {
            // assign frame-wide q values to this sb
            ts.dq = TileStateRef::Frame;
        } else if ts.last_qidx != prev_qidx {
            // find sb-specific quant parameters
            init_quant_tables(seq_hdr, frame_hdr, ts.last_qidx, &mut ts.dqmem);
            ts.dq = TileStateRef::Local;
        }
        if ts.last_delta_lf == [0, 0, 0, 0] {
            // assign frame-wide lf values to this sb
            ts.lflvl = TileStateRef::Frame;
        } else if ts.last_delta_lf != prev_delta_lf {
            // find sb-specific lf lvl parameters
            rav1d_calc_lf_values(&mut ts.lflvlmem, frame_hdr, &ts.last_delta_lf);
            ts.lflvl = TileStateRef::Local;
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
            if debug_block_info!(f, t.b) {
                println!("Post-intra[{}]: r={}", b.intra, ts.msac.rng);
            }
        }
    } else if frame_hdr.allow_intrabc {
        b.intra = (!rav1d_msac_decode_bool_adapt(&mut ts.msac, &mut ts.cdf.m.intrabc.0)) as u8;
        if debug_block_info!(f, t.b) {
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
        if debug_block_info!(f, t.b) {
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
                (cfl_allowed_mask & (1 << bs as u8)) != 0
            };
            let uvmode_cdf = &mut ts.cdf.m.uv_mode[cfl_allowed as usize][b.y_mode() as usize];
            *b.uv_mode_mut() = rav1d_msac_decode_symbol_adapt16(
                &mut ts.msac,
                uvmode_cdf,
                (N_UV_INTRA_PRED_MODES as usize) - 1 - (!cfl_allowed as usize),
            ) as u8;
            if debug_block_info!(f, t.b) {
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
                if debug_block_info!(f, t.b) {
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
        if frame_hdr.allow_screen_content_tools && cmp::max(bw4, bh4) <= 16 && bw4 + bh4 >= 4 {
            let sz_ctx = b_dim[2] + b_dim[3] - 2;
            if b.y_mode() == DC_PRED {
                let pal_ctx = ((*t.a).pal_sz.0[bx4 as usize] > 0) as usize
                    + (t.l.pal_sz.0[by4 as usize] > 0) as usize;
                let use_y_pal = rav1d_msac_decode_bool_adapt(
                    &mut ts.msac,
                    &mut ts.cdf.m.pal_y[sz_ctx as usize][pal_ctx],
                );
                if debug_block_info!(f, t.b) {
                    println!("Post-y_pal[{}]: r={}", use_y_pal, ts.msac.rng);
                }
                if use_y_pal {
                    (bd_fn.read_pal_plane)(t, f, b, false, sz_ctx, bx4 as usize, by4 as usize);
                }
            }

            if has_chroma && b.uv_mode() == DC_PRED {
                let pal_ctx = b.pal_sz()[0] > 0;
                let use_uv_pal = rav1d_msac_decode_bool_adapt(
                    &mut ts.msac,
                    &mut ts.cdf.m.pal_uv[pal_ctx as usize],
                );
                if debug_block_info!(f, t.b) {
                    println!("Post-uv_pal[{}]: r={}", use_uv_pal, ts.msac.rng);
                }
                if use_uv_pal {
                    // see aomedia bug 2183 for why we use luma coordinates
                    (bd_fn.read_pal_uv)(t, f, b, sz_ctx, bx4 as usize, by4 as usize);
                }
            }
        }

        let seq_hdr = f.seq_hdr();
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
            if debug_block_info!(f, t.b) {
                println!(
                    "Post-filterintramode[{}/{}]: r={}",
                    b.y_mode(),
                    b.y_angle(),
                    ts.msac.rng,
                );
            }
        }

        if b.pal_sz()[0] != 0 {
            let pal_idx = if t.frame_thread.pass != 0 {
                let p = t.frame_thread.pass & 1;
                let frame_thread = &mut ts.frame_thread[p as usize];
                let len = usize::try_from(bw4 * bh4 * 16).unwrap();
                let pal_idx = &mut f.frame_thread.pal_idx[frame_thread.pal_idx..][..len];
                frame_thread.pal_idx += len;
                pal_idx
            } else {
                &mut t.scratch.c2rust_unnamed_0.pal_idx
            };
            read_pal_indices(
                ts,
                &mut t.scratch.c2rust_unnamed_0.c2rust_unnamed.c2rust_unnamed,
                pal_idx,
                b,
                false,
                w4,
                h4,
                bw4,
                bh4,
            );
            if debug_block_info!(f, t.b) {
                println!("Post-y-pal-indices: r={}", ts.msac.rng);
            }
        }

        if has_chroma && b.pal_sz()[1] != 0 {
            let pal_idx = if t.frame_thread.pass != 0 {
                let p = t.frame_thread.pass & 1;
                let frame_thread = &mut ts.frame_thread[p as usize];
                let len = usize::try_from(cbw4 * cbh4 * 16).unwrap();
                let pal_idx = &mut f.frame_thread.pal_idx[frame_thread.pal_idx..][..len];
                frame_thread.pal_idx += len;
                pal_idx
            } else {
                &mut t.scratch.c2rust_unnamed_0.pal_idx[(bw4 * bh4 * 16) as usize..]
            };
            read_pal_indices(
                ts,
                &mut t.scratch.c2rust_unnamed_0.c2rust_unnamed.c2rust_unnamed,
                pal_idx,
                b,
                true,
                cw4,
                ch4,
                cbw4,
                cbh4,
            );
            if debug_block_info!(f, t.b) {
                println!("Post-uv-pal-indices: r={}", ts.msac.rng);
            }
        }

        let frame_hdr = f.frame_hdr();
        let t_dim = if frame_hdr.segmentation.lossless[b.seg_id as usize] != 0 {
            b.uvtx = TX_4X4 as u8;
            *b.tx_mut() = b.uvtx;
            &dav1d_txfm_dimensions[TX_4X4 as usize]
        } else {
            *b.tx_mut() = dav1d_max_txfm_size_for_bs[bs as usize][0];
            b.uvtx = dav1d_max_txfm_size_for_bs[bs as usize][f.cur.p.layout as usize];
            let mut t_dim = &dav1d_txfm_dimensions[b.tx() as usize];
            if frame_hdr.txfm_mode == Rav1dTxfmMode::Switchable && t_dim.max > TX_4X4 as u8 {
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
            if debug_block_info!(f, t.b) {
                println!("Post-tx[{}]: r={}", b.tx(), ts.msac.rng);
            }
            t_dim
        };

        // reconstruction
        if t.frame_thread.pass == 1 {
            bd_fn.read_coef_blocks(f, t, bs, b);
        } else {
            bd_fn.recon_b_intra(f, t, bs, intra_edge_flags, b);
        }

        if f.frame_hdr().loopfilter.level_y != [0, 0] {
            let lflvl = match ts.lflvl {
                TileStateRef::Frame => &f.lf.lvl,
                TileStateRef::Local => &ts.lflvlmem,
            };
            rav1d_create_lf_mask_intra(
                &mut f.lf.mask[t.lf_mask.unwrap()],
                &f.lf.level,
                f.b4_stride,
                &lflvl[b.seg_id as usize],
                t.b,
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
                if f.frame_hdr().frame_type.is_inter_or_switch() {
                    case.set(&mut dir.comp_type.0, None);
                    case.set(&mut dir.r#ref[0], -1);
                    case.set(&mut dir.r#ref[1], -1);
                    case.set(&mut dir.filter.0[0], Rav1dFilterMode::N_SWITCHABLE_FILTERS);
                    case.set(&mut dir.filter.0[1], Rav1dFilterMode::N_SWITCHABLE_FILTERS);
                }
            },
        );
        if b.pal_sz()[0] != 0 {
            (bd_fn.copy_pal_block_y)(t, f, bx4 as usize, by4 as usize, bw4 as usize, bh4 as usize);
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
                (bd_fn.copy_pal_block_uv)(
                    t,
                    f,
                    bx4 as usize,
                    by4 as usize,
                    bw4 as usize,
                    bh4 as usize,
                );
            }
        }
        let frame_hdr = f.frame_hdr();
        if frame_hdr.frame_type.is_inter_or_switch() || frame_hdr.allow_intrabc {
            splat_intraref(c, t, &mut f.rf.r, bs, bw4 as usize, bh4 as usize);
        }
    } else if frame_hdr.frame_type.is_key_or_intra() {
        // intra block copy
        let mut mvstack = [Default::default(); 8];
        let mut n_mvs = 0;
        let mut ctx = 0;
        rav1d_refmvs_find(
            &t.rt,
            &f.rf,
            &mut mvstack,
            &mut n_mvs,
            &mut ctx,
            [0, -1].into(),
            bs,
            intra_edge_flags,
            t.b.y,
            t.b.x,
            frame_hdr,
        );

        if mvstack[0].mv.mv[0] != mv::ZERO {
            b.mv_mut()[0] = mvstack[0].mv.mv[0];
        } else if mvstack[1].mv.mv[0] != mv::ZERO {
            b.mv_mut()[0] = mvstack[1].mv.mv[0];
        } else if t.b.y - (16 << seq_hdr.sb128) < ts.tiling.row_start {
            b.mv_mut()[0].y = 0;
            b.mv_mut()[0].x = (-(512 << seq_hdr.sb128) - 2048) as i16;
        } else {
            b.mv_mut()[0].y = -(512 << seq_hdr.sb128) as i16;
            b.mv_mut()[0].x = 0;
        }

        let r#ref = b.mv()[0];
        read_mv_residual(t, f, &mut b.mv_mut()[0], &mut ts.cdf.dmv, false);

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
        let mut src_left = t.b.x * 4 + (b.mv()[0].x as c_int >> 3);
        let mut src_top = t.b.y * 4 + (b.mv()[0].y as c_int >> 3);
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

        let sbx = t.b.x >> 4 + seq_hdr.sb128 << 6 + seq_hdr.sb128;
        let sby = t.b.y >> 4 + seq_hdr.sb128 << 6 + seq_hdr.sb128;
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

        b.mv_mut()[0].x = ((src_left - t.b.x * 4) * 8) as i16;
        b.mv_mut()[0].y = ((src_top - t.b.y * 4) * 8) as i16;

        if debug_block_info!(f, t.b) {
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
        read_vartx_tree(t, f, b, bs, bx4, by4);

        // reconstruction
        if t.frame_thread.pass == 1 {
            bd_fn.read_coef_blocks(f, t, bs, b);
            *b.filter2d_mut() = Filter2d::Bilinear;
        } else {
            bd_fn.recon_b_inter(f, t, bs, b)?;
        }

        splat_intrabc_mv(c, t, &mut f.rf.r, bs, b, bw4 as usize, bh4 as usize);

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
            if debug_block_info!(f, t.b) {
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
            *b.comp_type_mut() = Some(CompInterType::Avg);
            *b.inter_mode_mut() = NEARESTMV_NEARESTMV;
            *b.drl_idx_mut() = DrlProximity::Nearest;
            has_subpel_filter = false;

            let mut mvstack = [Default::default(); 8];
            let mut n_mvs = 0;
            let mut ctx = 0;
            rav1d_refmvs_find(
                &t.rt,
                &f.rf,
                &mut mvstack,
                &mut n_mvs,
                &mut ctx,
                [b.r#ref()[0] + 1, b.r#ref()[1] + 1].into(),
                bs,
                intra_edge_flags,
                t.b.y,
                t.b.x,
                frame_hdr,
            );

            *b.mv_mut() = mvstack[0].mv.mv;
            fix_mv_precision(frame_hdr, &mut b.mv_mut()[0]);
            fix_mv_precision(frame_hdr, &mut b.mv_mut()[1]);
            if debug_block_info!(f, t.b) {
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
            if debug_block_info!(f, t.b) {
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
                &t.rt,
                &f.rf,
                &mut mvstack,
                &mut n_mvs,
                &mut ctx,
                [b.r#ref()[0] + 1, b.r#ref()[1] + 1].into(),
                bs,
                intra_edge_flags,
                t.b.y,
                t.b.x,
                frame_hdr,
            );

            *b.inter_mode_mut() = rav1d_msac_decode_symbol_adapt8(
                &mut ts.msac,
                &mut ts.cdf.m.comp_inter_mode[ctx as usize],
                N_COMP_INTER_PRED_MODES as usize - 1,
            ) as u8;
            if debug_block_info!(f, t.b) {
                println!(
                    "Post-compintermode[{},ctx={},n_mvs={}]: r={}",
                    b.inter_mode(),
                    ctx,
                    n_mvs,
                    ts.msac.rng,
                );
            }

            let im = &dav1d_comp_inter_pred_modes[b.inter_mode() as usize];
            *b.drl_idx_mut() = DrlProximity::Nearest;
            if b.inter_mode() == NEWMV_NEWMV {
                if n_mvs > 1 {
                    // `Nearer` or `Near`
                    let drl_ctx_v1 = get_drl_context(&mvstack, 0);
                    if rav1d_msac_decode_bool_adapt(
                        &mut ts.msac,
                        &mut ts.cdf.m.drl_bit[drl_ctx_v1 as usize],
                    ) {
                        *b.drl_idx_mut() = DrlProximity::Nearer;

                        if n_mvs > 2 {
                            let drl_ctx_v2 = get_drl_context(&mvstack, 1);
                            if rav1d_msac_decode_bool_adapt(
                                &mut ts.msac,
                                &mut ts.cdf.m.drl_bit[drl_ctx_v2 as usize],
                            ) {
                                *b.drl_idx_mut() = DrlProximity::Near;
                            }
                        }
                    }
                    if debug_block_info!(f, t.b) {
                        println!(
                            "Post-drlidx[{:?},n_mvs={}]: r={}",
                            b.drl_idx(),
                            n_mvs,
                            ts.msac.rng,
                        );
                    }
                }
            } else if im[0] == NEARMV || im[1] == NEARMV {
                *b.drl_idx_mut() = DrlProximity::Nearer;
                if n_mvs > 2 {
                    // `Near` or `Nearish`
                    let drl_ctx_v2 = get_drl_context(&mvstack, 1);
                    if rav1d_msac_decode_bool_adapt(
                        &mut ts.msac,
                        &mut ts.cdf.m.drl_bit[drl_ctx_v2 as usize],
                    ) {
                        *b.drl_idx_mut() = DrlProximity::Near;

                        if n_mvs > 3 {
                            let drl_ctx_v3 = get_drl_context(&mvstack, 2);
                            if rav1d_msac_decode_bool_adapt(
                                &mut ts.msac,
                                &mut ts.cdf.m.drl_bit[drl_ctx_v3 as usize],
                            ) {
                                *b.drl_idx_mut() = DrlProximity::Nearish;
                            }
                        }
                    }
                    if debug_block_info!(f, t.b) {
                        println!(
                            "Post-drlidx[{:?},n_mvs={}]: r={}",
                            b.drl_idx(),
                            n_mvs,
                            ts.msac.rng,
                        );
                    }
                }
            }

            has_subpel_filter = cmp::min(bw4, bh4) == 1 || b.inter_mode() != GLOBALMV_GLOBALMV;
            let mut assign_comp_mv = |idx: usize| match im[idx] {
                NEARMV | NEARESTMV => {
                    b.mv_mut()[idx] = mvstack[b.drl_idx() as usize].mv.mv[idx];
                    fix_mv_precision(frame_hdr, &mut b.mv_mut()[idx]);
                }
                GLOBALMV => {
                    has_subpel_filter |= frame_hdr.gmv[b.r#ref()[idx] as usize].r#type
                        == Rav1dWarpedMotionType::Translation;
                    b.mv_mut()[idx] = get_gmv_2d(
                        &frame_hdr.gmv[b.r#ref()[idx] as usize],
                        t.b.x,
                        t.b.y,
                        bw4,
                        bh4,
                        frame_hdr,
                    );
                }
                NEWMV => {
                    b.mv_mut()[idx] = mvstack[b.drl_idx() as usize].mv.mv[idx];
                    read_mv_residual(
                        t,
                        f,
                        &mut b.mv_mut()[idx],
                        &mut ts.cdf.mv,
                        !frame_hdr.force_integer_mv,
                    );
                }
                _ => {}
            };
            assign_comp_mv(0);
            assign_comp_mv(1);
            if debug_block_info!(f, t.b) {
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
                if debug_block_info!(f, t.b) {
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
                    let comp_type = if rav1d_msac_decode_bool_adapt(
                        &mut ts.msac,
                        &mut ts.cdf.m.jnt_comp[jnt_ctx as usize],
                    ) {
                        CompInterType::Avg
                    } else {
                        CompInterType::WeightedAvg
                    };
                    *b.comp_type_mut() = Some(comp_type);
                    if debug_block_info!(f, t.b) {
                        println!(
                            "Post-jnt_comp[{},ctx={}[ac:{:?},ar:{},lc:{:?},lr:{}]]: r={}",
                            comp_type == CompInterType::Avg,
                            jnt_ctx,
                            (*t.a).comp_type[bx4 as usize],
                            (*t.a).r#ref[0][bx4 as usize],
                            t.l.comp_type[by4 as usize],
                            t.l.r#ref[0][by4 as usize],
                            ts.msac.rng,
                        );
                    }
                } else {
                    *b.comp_type_mut() = Some(CompInterType::Avg);
                }
            } else {
                if wedge_allowed_mask & (1 << bs as u8) != 0 {
                    let ctx = dav1d_wedge_ctx_lut[bs as usize] as usize;
                    let comp_type = if rav1d_msac_decode_bool_adapt(
                        &mut ts.msac,
                        &mut ts.cdf.m.wedge_comp[ctx],
                    ) {
                        CompInterType::Seg
                    } else {
                        CompInterType::Wedge
                    };
                    *b.comp_type_mut() = Some(comp_type);
                    if comp_type == CompInterType::Wedge {
                        *b.wedge_idx_mut() = rav1d_msac_decode_symbol_adapt16(
                            &mut ts.msac,
                            &mut ts.cdf.m.wedge_idx[ctx],
                            15,
                        ) as u8;
                    }
                } else {
                    *b.comp_type_mut() = Some(CompInterType::Seg);
                }
                *b.mask_sign_mut() = rav1d_msac_decode_bool_equi(&mut ts.msac) as u8;
                if debug_block_info!(f, t.b) {
                    println!(
                        "Post-seg/wedge[{},wedge_idx={},sign={}]: r={}",
                        b.comp_type() == Some(CompInterType::Wedge),
                        b.wedge_idx(),
                        b.mask_sign(),
                        ts.msac.rng,
                    );
                }
            }
        } else {
            *b.comp_type_mut() = None;

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
                if debug_block_info!(f, t.b) {
                    println!("Post-ref[{}]: r={}", b.r#ref()[0], ts.msac.rng);
                }
            }
            b.ref_mut()[1] = -1;

            let mut mvstack = [Default::default(); 8];
            let mut n_mvs = 0;
            let mut ctx = 0;
            rav1d_refmvs_find(
                &t.rt,
                &f.rf,
                &mut mvstack,
                &mut n_mvs,
                &mut ctx,
                refmvs_refpair {
                    r#ref: [b.r#ref()[0] + 1, -1],
                },
                bs,
                intra_edge_flags,
                t.b.y,
                t.b.x,
                frame_hdr,
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
                        t.b.x,
                        t.b.y,
                        bw4,
                        bh4,
                        frame_hdr,
                    );
                    has_subpel_filter = cmp::min(bw4, bh4) == 1
                        || frame_hdr.gmv[b.r#ref()[0] as usize].r#type
                            == Rav1dWarpedMotionType::Translation;
                } else {
                    has_subpel_filter = true;
                    if rav1d_msac_decode_bool_adapt(
                        &mut ts.msac,
                        &mut ts.cdf.m.refmv_mode[(ctx >> 4 & 15) as usize],
                    ) {
                        // `Nearer`, `Near` or `Nearish`
                        *b.inter_mode_mut() = NEARMV;
                        *b.drl_idx_mut() = DrlProximity::Nearer;
                        if n_mvs > 2 {
                            // `Nearer`, `Near` or `Nearish`
                            let drl_ctx_v2 = get_drl_context(&mvstack, 1);
                            if rav1d_msac_decode_bool_adapt(
                                &mut ts.msac,
                                &mut ts.cdf.m.drl_bit[drl_ctx_v2 as usize],
                            ) {
                                *b.drl_idx_mut() = DrlProximity::Near;

                                if n_mvs > 3 {
                                    // `Near` or `Nearish`
                                    let drl_ctx_v3 = get_drl_context(&mvstack, 2);
                                    if rav1d_msac_decode_bool_adapt(
                                        &mut ts.msac,
                                        &mut ts.cdf.m.drl_bit[drl_ctx_v3 as usize],
                                    ) {
                                        *b.drl_idx_mut() = DrlProximity::Nearish;
                                    }
                                }
                            }
                        }
                    } else {
                        *b.inter_mode_mut() = NEARESTMV as u8;
                        *b.drl_idx_mut() = DrlProximity::Nearest;
                    }
                    b.mv_mut()[0] = mvstack[b.drl_idx() as usize].mv.mv[0];
                    if b.drl_idx() < DrlProximity::Near {
                        fix_mv_precision(frame_hdr, &mut b.mv_mut()[0]);
                    }
                }

                if debug_block_info!(f, t.b) {
                    println!(
                        "Post-intermode[{},drl={:?},mv=y:{},x:{},n_mvs={}]: r={}",
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
                *b.drl_idx_mut() = DrlProximity::Nearest;
                if n_mvs > 1 {
                    // `Nearer`, `Near` or `Nearish`
                    let drl_ctx_v1 = get_drl_context(&mvstack, 0);
                    if rav1d_msac_decode_bool_adapt(
                        &mut ts.msac,
                        &mut ts.cdf.m.drl_bit[drl_ctx_v1 as usize],
                    ) {
                        *b.drl_idx_mut() = DrlProximity::Nearer;

                        if n_mvs > 2 {
                            // `Near` or `Nearish`
                            let drl_ctx_v2 = get_drl_context(&mvstack, 1);
                            if rav1d_msac_decode_bool_adapt(
                                &mut ts.msac,
                                &mut ts.cdf.m.drl_bit[drl_ctx_v2 as usize],
                            ) {
                                *b.drl_idx_mut() = DrlProximity::Near;
                            }
                        }
                    }
                }
                if n_mvs > 1 {
                    b.mv_mut()[0] = mvstack[b.drl_idx() as usize].mv.mv[0];
                } else {
                    assert_eq!(b.drl_idx(), DrlProximity::Nearest);
                    b.mv_mut()[0] = mvstack[0].mv.mv[0];
                    fix_mv_precision(frame_hdr, &mut b.mv_mut()[0]);
                }
                if debug_block_info!(f, t.b) {
                    println!(
                        "Post-intermode[{},drl={:?}]: r={}",
                        b.inter_mode(),
                        b.drl_idx(),
                        ts.msac.rng,
                    );
                }
                read_mv_residual(
                    t,
                    f,
                    &mut *b.mv_mut().as_mut_ptr().offset(0),
                    &mut ts.cdf.mv,
                    !frame_hdr.force_integer_mv,
                );
                if debug_block_info!(f, t.b) {
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
                && interintra_allowed_mask & (1 << bs as u8) != 0
                && rav1d_msac_decode_bool_adapt(
                    &mut ts.msac,
                    &mut ts.cdf.m.interintra[ii_sz_grp as usize],
                )
            {
                *b.interintra_mode_mut() =
                    InterIntraPredMode::from_repr(rav1d_msac_decode_symbol_adapt4(
                        &mut ts.msac,
                        &mut ts.cdf.m.interintra_mode[ii_sz_grp as usize],
                        InterIntraPredMode::COUNT as usize - 1,
                    ) as usize)
                    .expect("valid variant");
                let wedge_ctx = dav1d_wedge_ctx_lut[bs as usize] as c_int;
                let ii_type = if rav1d_msac_decode_bool_adapt(
                    &mut ts.msac,
                    &mut ts.cdf.m.interintra_wedge[wedge_ctx as usize],
                ) {
                    InterIntraType::Wedge
                } else {
                    InterIntraType::Blend
                };
                *b.interintra_type_mut() = Some(ii_type);
                if ii_type == InterIntraType::Wedge {
                    *b.wedge_idx_mut() = rav1d_msac_decode_symbol_adapt16(
                        &mut ts.msac,
                        &mut ts.cdf.m.wedge_idx[wedge_ctx as usize],
                        15,
                    ) as u8;
                }
            } else {
                *b.interintra_type_mut() = None;
            }
            if debug_block_info!(f, t.b)
                && seq_hdr.inter_intra != 0
                && interintra_allowed_mask & (1 << bs as u8) != 0
            {
                println!(
                    "Post-interintra[t={:?},m={:?},w={}]: r={}",
                    b.interintra_type(),
                    b.interintra_mode(),
                    b.wedge_idx(),
                    ts.msac.rng,
                );
            }

            // motion variation
            if frame_hdr.switchable_motion_mode != 0
                && b.interintra_type() == None
                && cmp::min(bw4, bh4) >= 2
                // is not warped global motion
                && !(!frame_hdr.force_integer_mv
                    && b.inter_mode() == GLOBALMV
                    && frame_hdr.gmv[b.r#ref()[0] as usize].r#type > Rav1dWarpedMotionType::Translation)
                // has overlappable neighbours
                && (have_left && findoddzero(&t.l.intra.0[by4 as usize..][..h4 as usize])
                    || have_top && findoddzero(&(*t.a).intra.0[bx4 as usize..][..w4 as usize]))
            {
                // reaching here means the block allows obmc - check warp by
                // finding matching-ref blocks in top/left edges
                let mut mask = [0, 0];
                find_matching_ref(
                    f,
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
                    && !frame_hdr.force_integer_mv
                    && frame_hdr.warp_motion != 0
                    && mask[0] | mask[1] != 0) as c_int;

                *b.motion_mode_mut() = MotionMode::from_repr(if allow_warp != 0 {
                    rav1d_msac_decode_symbol_adapt4(
                        &mut ts.msac,
                        &mut ts.cdf.m.motion_mode[bs as usize],
                        2,
                    ) as usize
                } else {
                    rav1d_msac_decode_bool_adapt(&mut ts.msac, &mut ts.cdf.m.obmc[bs as usize])
                        as usize
                })
                .expect("valid variant");
                if b.motion_mode() == MotionMode::Warp {
                    has_subpel_filter = false;
                    t.warpmv =
                        derive_warpmv(&f.rf.r, t, bw4, bh4, &mask, b.mv()[0], t.warpmv.clone());
                    if debug_block_info!(f, t.b) {
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
                        if t.warpmv.r#type == Rav1dWarpedMotionType::Affine {
                            b.matrix_mut()[0] = (t.warpmv.matrix[2] - 0x10000) as i16;
                            b.matrix_mut()[1] = t.warpmv.matrix[3] as i16;
                            b.matrix_mut()[2] = t.warpmv.matrix[4] as i16;
                            b.matrix_mut()[3] = (t.warpmv.matrix[5] - 0x10000) as i16;
                        } else {
                            b.matrix_mut()[0] = i16::MIN;
                        }
                    }
                }

                if debug_block_info!(f, t.b) {
                    println!(
                        "Post-motionmode[{:?}]: r={} [mask: 0x{:x}/0x{:x}]",
                        b.motion_mode(),
                        ts.msac.rng,
                        mask[0],
                        mask[1],
                    );
                }
            } else {
                *b.motion_mode_mut() = MotionMode::Translation;
            }
        }

        // subpel filter
        let filter = if frame_hdr.subpel_filter_mode == Rav1dFilterMode::Switchable {
            if has_subpel_filter {
                let comp = b.comp_type().is_some();
                let ctx1 = get_filter_ctx(&*t.a, &t.l, comp, false, b.r#ref()[0], by4, bx4);
                let filter0 = rav1d_msac_decode_symbol_adapt4(
                    &mut ts.msac,
                    &mut ts.cdf.m.filter.0[0][ctx1 as usize],
                    Rav1dFilterMode::N_SWITCHABLE_FILTERS as usize - 1,
                ) as Dav1dFilterMode;
                if seq_hdr.dual_filter != 0 {
                    let ctx2 = get_filter_ctx(&*t.a, &t.l, comp, true, b.r#ref()[0], by4, bx4);
                    if debug_block_info!(f, t.b) {
                        println!(
                            "Post-subpel_filter1[{},ctx={}]: r={}",
                            filter0, ctx1, ts.msac.rng,
                        );
                    }
                    let filter1 = rav1d_msac_decode_symbol_adapt4(
                        &mut ts.msac,
                        &mut ts.cdf.m.filter.0[1][ctx2 as usize],
                        Rav1dFilterMode::N_SWITCHABLE_FILTERS as usize - 1,
                    ) as Dav1dFilterMode;
                    if debug_block_info!(f, t.b) {
                        println!(
                            "Post-subpel_filter2[{},ctx={}]: r={}",
                            filter1, ctx2, ts.msac.rng,
                        );
                    }
                    [filter0, filter1]
                } else {
                    if debug_block_info!(f, t.b) {
                        println!(
                            "Post-subpel_filter[{},ctx={}]: r={}",
                            filter0, ctx1, ts.msac.rng
                        );
                    }
                    [filter0; 2]
                }
            } else {
                [Rav1dFilterMode::Regular8Tap as u8; 2]
            }
        } else {
            [frame_hdr.subpel_filter_mode as u8; 2]
        };
        *b.filter2d_mut() = dav1d_filter_2d[filter[1] as usize][filter[0] as usize];

        read_vartx_tree(t, f, b, bs, bx4, by4);

        // reconstruction
        if t.frame_thread.pass == 1 {
            bd_fn.read_coef_blocks(f, t, bs, b);
        } else {
            bd_fn.recon_b_inter(f, t, bs, b)?;
        }

        let frame_hdr = f.frame_hdr();
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
            let lflvl = match ts.lflvl {
                TileStateRef::Frame => &f.lf.lvl,
                TileStateRef::Local => &ts.lflvlmem,
            };
            rav1d_create_lf_mask_inter(
                &mut f.lf.mask[t.lf_mask.unwrap()],
                &f.lf.level,
                f.b4_stride,
                // In C, the inner dimensions (`ref`, `is_gmv`) are offset,
                // but then cast back to a pointer to the full array,
                // even though the whole array is not passed.
                // Dereferencing this in Rust is UB, so instead
                // we pass the indices as args, which are then applied at the use sites.
                &lflvl[b.seg_id as usize],
                (b.r#ref()[0] + 1) as usize,
                is_globalmv == 0,
                t.b,
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
            splat_tworef_mv(c, t, &mut f.rf.r, bs, b, bw4 as usize, bh4 as usize);
        } else {
            splat_oneref_mv(c, t, &mut f.rf.r, bs, b, bw4 as usize, bh4 as usize);
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
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    if frame_hdr.segmentation.enabled != 0 && frame_hdr.segmentation.update_map != 0 {
        // Need checked casts here because we're using `from_raw_parts_mut` and an overflow would be UB.
        let [by, bx, bh4, bw4] = [t.b.y, t.b.x, bh4, bw4].map(|it| usize::try_from(it).unwrap());
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
        for noskip_mask in &mut f.lf.mask[t.lf_mask.unwrap()].noskip_mask[by4 as usize >> 1..]
            [..(bh4 as usize + 1) / 2]
        {
            noskip_mask[bx_idx as usize].fetch_or(mask as u16, Ordering::Relaxed);
            if bw4 == 32 {
                // this should be mask >> 16, but it's 0xffffffff anyway
                noskip_mask[1].fetch_or(mask as u16, Ordering::Relaxed);
            }
        }
    }

    if t.frame_thread.pass == 1 && b.intra == 0 && frame_hdr.frame_type.is_inter_or_switch() {
        let sby = t.b.y - ts.tiling.row_start >> f.sb_shift;
        let lowest_px = &mut f.lowest_pixel_mem[ts.lowest_pixel + sby as usize];
        // keep track of motion vectors for each reference
        if b.comp_type().is_none() {
            // y
            if cmp::min(bw4, bh4) > 1
                && (b.inter_mode() == GLOBALMV && f.gmv_warp_allowed[b.r#ref()[0] as usize] != 0
                    || b.motion_mode() == MotionMode::Warp
                        && t.warpmv.r#type > Rav1dWarpedMotionType::Translation)
            {
                affine_lowest_px_luma(
                    t,
                    &mut lowest_px[b.r#ref()[0] as usize][0],
                    b_dim,
                    if b.motion_mode() == MotionMode::Warp {
                        &t.warpmv
                    } else {
                        &frame_hdr.gmv[b.r#ref()[0] as usize]
                    },
                );
            } else {
                mc_lowest_px(
                    &mut lowest_px[b.r#ref()[0] as usize][0],
                    t.b.y,
                    bh4,
                    b.mv()[0].y,
                    0,
                    &f.svc[b.r#ref()[0] as usize][1],
                );
                if b.motion_mode() == MotionMode::Obmc {
                    obmc_lowest_px(
                        &f.rf.r,
                        t,
                        &*f.ts.offset(t.ts as isize),
                        f.cur.p.layout,
                        &f.svc,
                        lowest_px,
                        false,
                        b_dim,
                        bx4,
                        by4,
                        w4,
                        h4,
                    );
                }
            }

            // uv
            if has_chroma {
                // sub8x8 derivation
                let mut is_sub8x8 = bw4 == ss_hor || bh4 == ss_ver;
                let r = if is_sub8x8 {
                    assert!(ss_hor == 1);
                    let r =
                        <[_; 2]>::try_from(&t.rt.r[(t.b.y as usize & 31) + 5 - 1..][..2]).unwrap();

                    if bw4 == 1 {
                        is_sub8x8 &= f.rf.r[r[1] + t.b.x as usize - 1].0.r#ref.r#ref[0] > 0;
                    }
                    if bh4 == ss_ver {
                        is_sub8x8 &= f.rf.r[r[0] + t.b.x as usize].0.r#ref.r#ref[0] > 0;
                    }
                    if bw4 == 1 && bh4 == ss_ver {
                        is_sub8x8 &= f.rf.r[r[0] + t.b.x as usize - 1].0.r#ref.r#ref[0] > 0;
                    }

                    r
                } else {
                    Default::default() // Never actually used.
                };

                // chroma prediction
                if is_sub8x8 {
                    if bw4 == 1 && bh4 == ss_ver {
                        let rr = f.rf.r[r[0] + t.b.x as usize - 1].0;
                        mc_lowest_px(
                            &mut lowest_px[rr.r#ref.r#ref[0] as usize - 1][1],
                            t.b.y - 1,
                            bh4,
                            rr.mv.mv[0].y,
                            ss_ver,
                            &f.svc[rr.r#ref.r#ref[0] as usize - 1][1],
                        );
                    }
                    if bw4 == 1 {
                        let rr = f.rf.r[r[1] + t.b.x as usize - 1].0;
                        mc_lowest_px(
                            &mut lowest_px[rr.r#ref.r#ref[0] as usize - 1][1],
                            t.b.y,
                            bh4,
                            rr.mv.mv[0].y,
                            ss_ver,
                            &f.svc[rr.r#ref.r#ref[0] as usize - 1][1],
                        );
                    }
                    if bh4 == ss_ver {
                        let rr = f.rf.r[r[0] + t.b.x as usize].0;
                        mc_lowest_px(
                            &mut lowest_px[rr.r#ref.r#ref[0] as usize - 1][1],
                            t.b.y - 1,
                            bh4,
                            rr.mv.mv[0].y,
                            ss_ver,
                            &f.svc[rr.r#ref.r#ref[0] as usize - 1][1],
                        );
                    }
                    mc_lowest_px(
                        &mut lowest_px[b.r#ref()[0] as usize][1],
                        t.b.y,
                        bh4,
                        b.mv()[0].y,
                        ss_ver,
                        &f.svc[b.r#ref()[0] as usize][1],
                    );
                } else if cmp::min(cbw4, cbh4) > 1
                    && (b.inter_mode() == GLOBALMV
                        && f.gmv_warp_allowed[b.r#ref()[0] as usize] != 0
                        || b.motion_mode() == MotionMode::Warp
                            && t.warpmv.r#type > Rav1dWarpedMotionType::Translation)
                {
                    affine_lowest_px_chroma(
                        t,
                        f.cur.p.layout,
                        &mut lowest_px[b.r#ref()[0] as usize][1],
                        b_dim,
                        if b.motion_mode() == MotionMode::Warp {
                            &t.warpmv
                        } else {
                            &frame_hdr.gmv[b.r#ref()[0] as usize]
                        },
                    );
                } else {
                    mc_lowest_px(
                        &mut lowest_px[b.r#ref()[0] as usize][1],
                        t.b.y & !ss_ver,
                        bh4 << (bh4 == ss_ver) as c_int,
                        b.mv()[0].y,
                        ss_ver,
                        &f.svc[b.r#ref()[0] as usize][1],
                    );
                    if b.motion_mode() == MotionMode::Obmc {
                        obmc_lowest_px(
                            &f.rf.r,
                            t,
                            &*f.ts.offset(t.ts as isize),
                            f.cur.p.layout,
                            &f.svc,
                            lowest_px,
                            true,
                            b_dim,
                            bx4,
                            by4,
                            w4,
                            h4,
                        );
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
                        t.b.y,
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
                        t.b.y,
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
                            f.cur.p.layout,
                            &mut lowest_px[r#ref][1],
                            b_dim,
                            &frame_hdr.gmv[r#ref],
                        );
                    } else {
                        mc_lowest_px(
                            &mut lowest_px[r#ref][1],
                            t.b.y,
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
    c: &Rav1dContext,
    t: &mut Rav1dTaskContext,
    f: &mut Rav1dFrameData,
    bl: BlockLevel,
    edge_index: EdgeIndex,
) -> Result<(), ()> {
    let ts = &mut *f.ts.offset(t.ts as isize);
    let hsz = 16 >> bl as u8;
    let have_h_split = f.bw > t.b.x + hsz;
    let have_v_split = f.bh > t.b.y + hsz;

    let sb128 = f.seq_hdr().sb128 != 0;
    let intra_edge = &IntraEdges::DEFAULT;

    if !have_h_split && !have_v_split {
        let next_bl = bl
            .decrease()
            .expect("BlockLevel::BL_8X8 should never make it here");

        return decode_sb(
            c,
            t,
            f,
            next_bl,
            intra_edge.branch(sb128, edge_index).split[0],
        );
    }

    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();

    let bp;
    let mut ctx = 0;
    let mut bx8 = 0;
    let mut by8 = 0;
    let pc = if t.frame_thread.pass == 2 {
        None
    } else {
        if false && bl == BlockLevel::Bl64x64 {
            println!(
                "poc={},y={},x={},bl={:?},r={}",
                frame_hdr.frame_offset, t.b.y, t.b.x, bl, ts.msac.rng,
            );
        }
        bx8 = (t.b.x & 31) >> 1;
        by8 = (t.b.y & 31) >> 1;
        ctx = get_partition_ctx(&*t.a, &t.l, bl, by8, bx8);
        Some(&mut ts.cdf.m.partition[bl as usize][ctx as usize])
    };

    if have_h_split && have_v_split {
        if let Some(pc) = pc {
            bp = BlockPartition::from_repr(rav1d_msac_decode_symbol_adapt16(
                &mut ts.msac,
                pc,
                dav1d_partition_type_count[bl as usize].into(),
            ) as usize)
            .expect("valid variant");
            if f.cur.p.layout == Rav1dPixelLayout::I422
                && matches!(
                    bp,
                    BlockPartition::V
                        | BlockPartition::V4
                        | BlockPartition::LeftSplit
                        | BlockPartition::RightSplit
                )
            {
                return Err(());
            }
            if debug_block_info!(f, t.b) {
                println!(
                    "poc={},y={},x={},bl={:?},ctx={},bp={:?}: r={}",
                    frame_hdr.frame_offset, t.b.y, t.b.x, bl, ctx, bp, ts.msac.rng,
                );
            }
        } else {
            let b = &f.frame_thread.b[(t.b.y as isize * f.b4_stride + t.b.x as isize) as usize];
            bp = if b.bl == bl {
                b.bp
            } else {
                BlockPartition::Split
            };
        }
        let b = &dav1d_block_sizes[bl as usize][bp as usize];

        match bp {
            BlockPartition::None => {
                let node = intra_edge.node(sb128, edge_index);
                decode_b(c, t, f, bl, b[0], bp, node.o)?;
            }
            BlockPartition::H => {
                let node = intra_edge.node(sb128, edge_index);
                decode_b(c, t, f, bl, b[0], bp, node.h[0])?;
                t.b.y += hsz;
                decode_b(c, t, f, bl, b[0], bp, node.h[1])?;
                t.b.y -= hsz;
            }
            BlockPartition::V => {
                let node = intra_edge.node(sb128, edge_index);
                decode_b(c, t, f, bl, b[0], bp, node.v[0])?;
                t.b.x += hsz;
                decode_b(c, t, f, bl, b[0], bp, node.v[1])?;
                t.b.x -= hsz;
            }
            BlockPartition::Split => {
                match bl.decrease() {
                    None => {
                        let tip = intra_edge.tip(sb128, edge_index);
                        assert!(hsz == 1);
                        decode_b(c, t, f, bl, BlockSize::Bs4x4, bp, EdgeFlags::ALL_TR_AND_BL)?;
                        let tl_filter = t.tl_4x4_filter;
                        t.b.x += 1;
                        decode_b(c, t, f, bl, BlockSize::Bs4x4, bp, tip.split[0])?;
                        t.b.x -= 1;
                        t.b.y += 1;
                        decode_b(c, t, f, bl, BlockSize::Bs4x4, bp, tip.split[1])?;
                        t.b.x += 1;
                        t.tl_4x4_filter = tl_filter;
                        decode_b(c, t, f, bl, BlockSize::Bs4x4, bp, tip.split[2])?;
                        t.b.x -= 1;
                        t.b.y -= 1;
                        if cfg!(target_arch = "x86_64") && t.frame_thread.pass != 0 {
                            // In 8-bit mode with 2-pass decoding the coefficient buffer
                            // can end up misaligned due to skips here.
                            // Work around the issue by explicitly realigning the buffer.
                            //
                            // In 8-bit mode coef is 2 bytes wide, so we align to 32
                            // elements to get 64 byte alignment.
                            let p = (t.frame_thread.pass & 1) as usize;
                            ts.frame_thread[p].cf = (ts.frame_thread[p].cf + 31) & !31;
                        }
                    }
                    Some(next_bl) => {
                        let branch = intra_edge.branch(sb128, edge_index);
                        decode_sb(c, t, f, next_bl, branch.split[0])?;
                        t.b.x += hsz;
                        decode_sb(c, t, f, next_bl, branch.split[1])?;
                        t.b.x -= hsz;
                        t.b.y += hsz;
                        decode_sb(c, t, f, next_bl, branch.split[2])?;
                        t.b.x += hsz;
                        decode_sb(c, t, f, next_bl, branch.split[3])?;
                        t.b.x -= hsz;
                        t.b.y -= hsz;
                    }
                }
            }
            BlockPartition::TopSplit => {
                let node = intra_edge.node(sb128, edge_index);
                decode_b(c, t, f, bl, b[0], bp, EdgeFlags::ALL_TR_AND_BL)?;
                t.b.x += hsz;
                decode_b(c, t, f, bl, b[0], bp, node.v[1])?;
                t.b.x -= hsz;
                t.b.y += hsz;
                decode_b(c, t, f, bl, b[1], bp, node.h[1])?;
                t.b.y -= hsz;
            }
            BlockPartition::BottomSplit => {
                let node = intra_edge.node(sb128, edge_index);
                decode_b(c, t, f, bl, b[0], bp, node.h[0])?;
                t.b.y += hsz;
                decode_b(c, t, f, bl, b[1], bp, node.v[0])?;
                t.b.x += hsz;
                decode_b(c, t, f, bl, b[1], bp, EdgeFlags::empty())?;
                t.b.x -= hsz;
                t.b.y -= hsz;
            }
            BlockPartition::LeftSplit => {
                let node = intra_edge.node(sb128, edge_index);
                decode_b(c, t, f, bl, b[0], bp, EdgeFlags::ALL_TR_AND_BL)?;
                t.b.y += hsz;
                decode_b(c, t, f, bl, b[0], bp, node.h[1])?;
                t.b.y -= hsz;
                t.b.x += hsz;
                decode_b(c, t, f, bl, b[1], bp, node.v[1])?;
                t.b.x -= hsz;
            }
            BlockPartition::RightSplit => {
                let node = intra_edge.node(sb128, edge_index);
                decode_b(c, t, f, bl, b[0], bp, node.v[0])?;
                t.b.x += hsz;
                decode_b(c, t, f, bl, b[1], bp, node.h[0])?;
                t.b.y += hsz;
                decode_b(c, t, f, bl, b[1], bp, EdgeFlags::empty())?;
                t.b.y -= hsz;
                t.b.x -= hsz;
            }
            BlockPartition::H4 => {
                let branch = intra_edge.branch(sb128, edge_index);
                let node = &branch.node;
                decode_b(c, t, f, bl, b[0], bp, node.h[0])?;
                t.b.y += hsz >> 1;
                decode_b(c, t, f, bl, b[0], bp, branch.h4)?;
                t.b.y += hsz >> 1;
                decode_b(c, t, f, bl, b[0], bp, EdgeFlags::ALL_LEFT_HAS_BOTTOM)?;
                t.b.y += hsz >> 1;
                if t.b.y < f.bh {
                    decode_b(c, t, f, bl, b[0], bp, node.h[1])?;
                }
                t.b.y -= hsz * 3 >> 1;
            }
            BlockPartition::V4 => {
                let branch = intra_edge.branch(sb128, edge_index);
                let node = &branch.node;
                decode_b(c, t, f, bl, b[0], bp, node.v[0])?;
                t.b.x += hsz >> 1;
                decode_b(c, t, f, bl, b[0], bp, branch.v4)?;
                t.b.x += hsz >> 1;
                decode_b(c, t, f, bl, b[0], bp, EdgeFlags::ALL_TOP_HAS_RIGHT)?;
                t.b.x += hsz >> 1;
                if t.b.x < f.bw {
                    decode_b(c, t, f, bl, b[0], bp, node.v[1])?;
                }
                t.b.x -= hsz * 3 >> 1;
            }
        }
    } else if have_h_split {
        let is_split;
        if let Some(pc) = pc {
            is_split = rav1d_msac_decode_bool(&mut ts.msac, gather_top_partition_prob(pc, bl));
            if debug_block_info!(f, t.b) {
                println!(
                    "poc={},y={},x={},bl={:?},ctx={},bp={:?}: r={}",
                    frame_hdr.frame_offset,
                    t.b.y,
                    t.b.x,
                    bl,
                    ctx,
                    if is_split {
                        BlockPartition::Split
                    } else {
                        BlockPartition::H
                    },
                    ts.msac.rng,
                );
            }
        } else {
            let b = &f.frame_thread.b[(t.b.y as isize * f.b4_stride + t.b.x as isize) as usize];
            is_split = b.bl != bl;
        }

        let next_bl = bl
            .decrease()
            .expect("BlockLevel::BL_8X8 should never make it here");

        if is_split {
            let branch = intra_edge.branch(sb128, edge_index);
            bp = BlockPartition::Split;
            decode_sb(c, t, f, next_bl, branch.split[0])?;
            t.b.x += hsz;
            decode_sb(c, t, f, next_bl, branch.split[1])?;
            t.b.x -= hsz;
        } else {
            let node = intra_edge.node(sb128, edge_index);
            bp = BlockPartition::H;
            decode_b(
                c,
                t,
                f,
                bl,
                dav1d_block_sizes[bl as usize][bp as usize][0],
                bp,
                node.h[0],
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
            if debug_block_info!(f, t.b) {
                println!(
                    "poc={},y={},x={},bl={:?},ctx={},bp={:?}: r={}",
                    frame_hdr.frame_offset,
                    t.b.y,
                    t.b.x,
                    bl,
                    ctx,
                    if is_split {
                        BlockPartition::Split
                    } else {
                        BlockPartition::V
                    },
                    ts.msac.rng,
                );
            }
        } else {
            let b = &f.frame_thread.b[(t.b.y as isize * f.b4_stride + t.b.x as isize) as usize];
            is_split = b.bl != bl;
        }

        let next_bl = bl
            .decrease()
            .expect("BlockLevel::BL_8X8 should never make it here");

        if is_split {
            let branch = intra_edge.branch(sb128, edge_index);
            bp = BlockPartition::Split;
            decode_sb(c, t, f, next_bl, branch.split[0])?;
            t.b.y += hsz;
            decode_sb(c, t, f, next_bl, branch.split[2])?;
            t.b.y -= hsz;
        } else {
            let node = intra_edge.node(sb128, edge_index);
            bp = BlockPartition::V;
            decode_b(
                c,
                t,
                f,
                bl,
                dav1d_block_sizes[bl as usize][bp as usize][0],
                bp,
                node.v[0],
            )?;
        }
    }

    if t.frame_thread.pass != 2 && (bp != BlockPartition::Split || bl == BlockLevel::Bl8x8) {
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
        ctx.comp_type.0.fill(None);
        ctx.mode.0.fill(NEARESTMV);
    }
    ctx.lcoef.0.fill(0x40);
    for ccoef in &mut ctx.ccoef.0 {
        ccoef.fill(0x40);
    }
    for filter in &mut ctx.filter.0 {
        filter.fill(Rav1dFilterMode::N_SWITCHABLE_FILTERS as u8);
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
    c: &Rav1dContext,
    ts: &mut Rav1dTileState,
    f: &Rav1dFrameData,
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
        ts.frame_thread[p].pal_idx = if !f.frame_thread.pal_idx.is_empty() {
            tile_start_off * size_mul[1] as usize / 4
        } else {
            0
        };
        ts.frame_thread[p].cf = if !f.frame_thread.cf.is_empty() {
            let bpc = BPC::from_bitdepth_max(f.bitdepth_max);
            bpc.coef_stride(tile_start_off * size_mul[0] as usize >> (seq_hdr.hbd == 0) as c_int)
        } else {
            0
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
            &f.lf.lr_mask[(sb_idx + sb128x) as usize].lr[p][u_idx as usize]
        } else {
            &f.lf.lr_mask[sb_idx as usize].lr[p][unit_idx as usize]
        };

        let mut lr = lr_ref.try_write().unwrap();
        *lr = Av1RestorationUnit {
            filter_v: [3, -7, 15],
            filter_h: [3, -7, 15],
            sgr_weights: [-32, 31],
            ..*lr
        };
        ts.lr_ref[p] = *lr;
    }

    if c.tc.len() > 1 {
        ts.progress.fill_with(|| AtomicI32::new(row_sb_start));
    }
}

unsafe fn read_restoration_info(
    ts: &mut Rav1dTileState,
    lr: &mut Av1RestorationUnit,
    p: usize,
    frame_type: Rav1dRestorationType,
    debug_block_info: bool,
) {
    let lr_ref = ts.lr_ref[p];

    if frame_type == Rav1dRestorationType::Switchable {
        let filter =
            rav1d_msac_decode_symbol_adapt4(&mut ts.msac, &mut ts.cdf.m.restore_switchable.0, 2);
        lr.r#type = if filter != 0 {
            if filter == 2 {
                Rav1dRestorationType::SgrProj(SgrIdx::I0)
            } else {
                Rav1dRestorationType::Wiener
            }
        } else {
            Rav1dRestorationType::None
        };
    } else {
        let r#type = rav1d_msac_decode_bool_adapt(
            &mut ts.msac,
            if frame_type == Rav1dRestorationType::Wiener {
                &mut ts.cdf.m.restore_wiener.0
            } else {
                &mut ts.cdf.m.restore_sgrproj.0
            },
        );
        lr.r#type = if r#type {
            frame_type
        } else {
            Rav1dRestorationType::None
        };
    }

    fn msac_decode_lr_subexp(ts: &mut Rav1dTileState, r#ref: i8, k: u32, adjustment: i8) -> i8 {
        (rav1d_msac_decode_subexp(&mut ts.msac, (r#ref + adjustment) as c_uint, 8 << k, k)
            - adjustment as c_int) as i8
    }

    match lr.r#type {
        Rav1dRestorationType::Wiener => {
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
            if debug_block_info {
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
        }
        Rav1dRestorationType::SgrProj(_) => {
            let sgr_idx =
                SgrIdx::from_repr(rav1d_msac_decode_bools(&mut ts.msac, 4) as usize).unwrap();
            let sgr_params = &dav1d_sgr_params[sgr_idx as usize];
            lr.r#type = Rav1dRestorationType::SgrProj(sgr_idx);
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
            if debug_block_info {
                println!(
                    "Post-lr_sgrproj[pl={},idx={},w[{},{}]]: r={}",
                    p, sgr_idx, lr.sgr_weights[0], lr.sgr_weights[1], ts.msac.rng,
                );
            }
        }
        _ => {}
    }
}

pub(crate) unsafe fn rav1d_decode_tile_sbrow(
    c: &Rav1dContext,
    t: &mut Rav1dTaskContext,
    f: &mut Rav1dFrameData,
) -> Result<(), ()> {
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    let root_bl = if seq_hdr.sb128 != 0 {
        BlockLevel::Bl128x128
    } else {
        BlockLevel::Bl64x64
    };
    let ts = &mut *f.ts.offset(t.ts as isize);
    let sb_step = f.sb_step;
    let tile_row = ts.tiling.row;
    let tile_col = ts.tiling.col;
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    let col_sb_start = frame_hdr.tiling.col_start_sb[tile_col as usize] as c_int;
    let col_sb128_start = col_sb_start >> (seq_hdr.sb128 == 0) as c_int;

    if frame_hdr.frame_type.is_inter_or_switch() || frame_hdr.allow_intrabc {
        t.rt = rav1d_refmvs_tile_sbrow_init(
            &f.rf,
            ts.tiling.col_start,
            ts.tiling.col_end,
            ts.tiling.row_start,
            ts.tiling.row_end,
            t.b.y >> f.sb_shift,
            ts.tiling.row,
            t.frame_thread.pass,
        );
    }

    if frame_hdr.frame_type.is_inter_or_switch() && c.n_fc > 1 {
        let sby = t.b.y - ts.tiling.row_start >> f.sb_shift;
        f.lowest_pixel_mem[ts.lowest_pixel + sby as usize] = [[i32::MIN; 2]; 7];
    }

    reset_context(
        &mut t.l,
        frame_hdr.frame_type.is_key_or_intra(),
        t.frame_thread.pass,
    );
    if t.frame_thread.pass == 2 {
        let off_2pass = if c.tc.len() > 1 {
            f.sb128w * frame_hdr.tiling.rows
        } else {
            0
        };
        t.a =
            f.a.as_mut_ptr()
                .offset((off_2pass + col_sb128_start + tile_row * f.sb128w) as isize);
        for bx in (ts.tiling.col_start..ts.tiling.col_end).step_by(sb_step as usize) {
            t.b.x = bx;
            if c.flush.load(Ordering::Acquire) != 0 {
                return Err(());
            }
            decode_sb(c, t, f, root_bl, EdgeIndex::root())?;
            if t.b.x & 16 != 0 || f.seq_hdr().sb128 != 0 {
                t.a = (t.a).offset(1);
            }
        }
        (f.bd_fn().backup_ipred_edge)(f, t);
        return Ok(());
    }

    // error out on symbol decoder overread
    if ts.msac.cnt < -15 {
        return Err(());
    }

    if c.tc.len() > 1 && frame_hdr.use_ref_frame_mvs != 0 {
        let rf = f.rf.as_dav1d();
        (c.refmvs_dsp.load_tmvs)(
            &rf,
            ts.tiling.row,
            ts.tiling.col_start >> 1,
            ts.tiling.col_end >> 1,
            t.b.y >> 1,
            t.b.y + sb_step >> 1,
        );
    }
    t.pal_sz_uv[1] = Default::default();
    let sb128y = t.b.y >> 5;
    t.a =
        f.a.as_mut_ptr()
            .offset((col_sb128_start + tile_row * f.sb128w) as isize);
    t.lf_mask = Some((sb128y * f.sb128w + col_sb128_start) as usize);
    for bx in (ts.tiling.col_start..ts.tiling.col_end).step_by(sb_step as usize) {
        t.b.x = bx;
        if c.flush.load(Ordering::Acquire) != 0 {
            return Err(());
        }
        let cdef_idx = &f.lf.mask[t.lf_mask.unwrap()].cdef_idx;
        if root_bl == BlockLevel::Bl128x128 {
            for cdef_idx in cdef_idx {
                cdef_idx.store(-1, Ordering::Relaxed);
            }
            t.cur_sb_cdef_idx = 0;
        } else {
            t.cur_sb_cdef_idx = (((t.b.x & 16) >> 4) + ((t.b.y & 16) >> 3)) as usize;
            let cdef_idx = &cdef_idx[t.cur_sb_cdef_idx..];
            cdef_idx[0].store(-1, Ordering::Relaxed);
        }
        let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
        // Restoration filter
        for p in 0..3 {
            if (f.lf.restore_planes >> p) & 1 == 0 {
                continue;
            }

            let ss_ver = (p != 0 && f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
            let ss_hor = (p != 0 && f.cur.p.layout != Rav1dPixelLayout::I444) as c_int;
            let unit_size_log2 = frame_hdr.restoration.unit_size[(p != 0) as usize];
            let y = t.b.y * 4 >> ss_ver;
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
                let x0 = (4 * t.b.x * d >> ss_hor) + rnd >> shift;
                let x1 = (4 * (t.b.x + sb_step) * d >> ss_hor) + rnd >> shift;

                for x in x0..cmp::min(x1, n_units) {
                    let px_x = x << unit_size_log2 + ss_hor;
                    let sb_idx = (t.b.y >> 5) * f.sr_sb128w + (px_x >> 7);
                    let unit_idx = ((t.b.y & 16) >> 3) + ((px_x & 64) >> 6);
                    let mut lr = f.lf.lr_mask[sb_idx as usize].lr[p][unit_idx as usize]
                        .try_write()
                        .unwrap();

                    read_restoration_info(ts, &mut lr, p, frame_type, debug_block_info!(f, t.b));
                }
            } else {
                let x = 4 * t.b.x >> ss_hor;
                if x as c_uint & mask != 0 {
                    continue;
                }
                let w = f.cur.p.w + ss_hor >> ss_hor;
                // Round half up at frame boundaries,
                // if there's more than one restoration unit.
                if x != 0 && x + half_unit > w {
                    continue;
                }
                let sb_idx = (t.b.y >> 5) * f.sr_sb128w + (t.b.x >> 5);
                let unit_idx = ((t.b.y & 16) >> 3) + ((t.b.x & 16) >> 4);
                let mut lr = f.lf.lr_mask[sb_idx as usize].lr[p][unit_idx as usize]
                    .try_write()
                    .unwrap();

                read_restoration_info(ts, &mut lr, p, frame_type, debug_block_info!(f, t.b));
            }
        }
        decode_sb(c, t, f, root_bl, EdgeIndex::root())?;
        if t.b.x & 16 != 0 || f.seq_hdr().sb128 != 0 {
            t.a = (t.a).offset(1);
            t.lf_mask = t.lf_mask.map(|i| i + 1);
        }
    }

    if f.seq_hdr().ref_frame_mvs != 0
        && c.tc.len() > 1
        && f.frame_hdr().frame_type.is_inter_or_switch()
    {
        rav1d_refmvs_save_tmvs(
            &c.refmvs_dsp,
            &t.rt,
            &f.rf,
            ts.tiling.col_start >> 1,
            ts.tiling.col_end >> 1,
            t.b.y >> 1,
            t.b.y + sb_step >> 1,
        );
    }

    // backup pre-loopfilter pixels for intra prediction of the next sbrow
    if t.frame_thread.pass != 1 {
        (f.bd_fn().backup_ipred_edge)(f, t);
    }

    // backup t->a/l.tx_lpf_y/uv at tile boundaries to use them to "fix"
    // up the initial value in neighbour tiles when running the loopfilter
    let mut align_h = f.bh + 31 & !31;
    let (tx_lpf_right_edge_y, tx_lpf_right_edge_uv) = f.lf.tx_lpf_right_edge.get_mut();
    tx_lpf_right_edge_y[(align_h * tile_col + t.b.y) as usize..][..sb_step as usize]
        .copy_from_slice(&t.l.tx_lpf_y.0[(t.b.y & 16) as usize..][..sb_step as usize]);
    let ss_ver = (f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
    align_h >>= ss_ver;
    tx_lpf_right_edge_uv[(align_h * tile_col + (t.b.y >> ss_ver)) as usize..]
        [..(sb_step >> ss_ver) as usize]
        .copy_from_slice(
            &t.l.tx_lpf_uv.0[((t.b.y & 16) >> ss_ver) as usize..][..(sb_step >> ss_ver) as usize],
        );

    Ok(())
}

pub(crate) unsafe fn rav1d_decode_frame_init(
    c: &Rav1dContext,
    f: &mut Rav1dFrameData,
) -> Rav1dResult {
    // TODO: Fallible allocation
    f.lf.start_of_tile_row.resize(f.sbh as usize, 0);

    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    let mut sby = 0;
    for tile_row in 0..frame_hdr.tiling.rows {
        f.lf.start_of_tile_row[sby as usize] = tile_row as u8;
        sby += 1;
        while sby < frame_hdr.tiling.row_start_sb[(tile_row + 1) as usize] as c_int {
            f.lf.start_of_tile_row[sby as usize] = 0;
            sby += 1;
        }
    }

    let n_ts = frame_hdr.tiling.cols * frame_hdr.tiling.rows;
    if n_ts != f.n_ts {
        if c.n_fc > 1 {
            // TODO: Fallible allocation
            f.frame_thread.tile_start_off.resize(n_ts as usize, 0);
        }
        rav1d_free_aligned(f.ts as *mut c_void);
        f.ts = rav1d_alloc_aligned(::core::mem::size_of::<Rav1dTileState>() * n_ts as usize, 32)
            as *mut Rav1dTileState;
        if f.ts.is_null() {
            return Err(ENOMEM);
        }
        f.n_ts = n_ts;
    }

    let a_sz = f.sb128w * frame_hdr.tiling.rows * (1 + (c.n_fc > 1 && c.tc.len() > 1) as c_int);
    // TODO: Fallible allocation
    f.a.resize_with(a_sz as usize, Default::default);

    let num_sb128 = f.sb128w * f.sb128h;
    let size_mul = &ss_size_mul[f.cur.p.layout];
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    let hbd = (seq_hdr.hbd != 0) as c_int;
    if c.n_fc > 1 {
        let mut tile_idx = 0;
        let sb_step4 = f.sb_step as u32 * 4;
        for tile_row in 0..frame_hdr.tiling.rows {
            let row_off = frame_hdr.tiling.row_start_sb[tile_row as usize] as u32
                * sb_step4
                * f.sb128w as u32
                * 128;
            let b_diff = (frame_hdr.tiling.row_start_sb[(tile_row + 1) as usize] as u32
                - frame_hdr.tiling.row_start_sb[tile_row as usize] as u32)
                * sb_step4;
            for tile_col in 0..frame_hdr.tiling.cols {
                f.frame_thread.tile_start_off[tile_idx] = row_off
                    + b_diff * frame_hdr.tiling.col_start_sb[tile_col as usize] as u32 * sb_step4;

                tile_idx += 1;
            }
        }

        let lowest_pixel_mem_sz = frame_hdr.tiling.cols * f.sbh;
        // TODO: Fallible allocation
        f.lowest_pixel_mem
            .resize(lowest_pixel_mem_sz as usize, Default::default());

        let mut lowest_pixel_offset = 0;
        for tile_row in 0..frame_hdr.tiling.rows {
            let tile_row_base = tile_row * frame_hdr.tiling.cols;
            let tile_row_sb_h = frame_hdr.tiling.row_start_sb[(tile_row + 1) as usize] as c_int
                - frame_hdr.tiling.row_start_sb[tile_row as usize] as c_int;
            for tile_col in 0..frame_hdr.tiling.cols {
                (*f.ts.offset((tile_row_base + tile_col) as isize)).lowest_pixel =
                    lowest_pixel_offset;
                lowest_pixel_offset += tile_row_sb_h as usize;
            }
        }

        let cf_sz = (num_sb128 * size_mul[0] as c_int) << hbd;
        // TODO: Fallible allocation
        f.frame_thread.cf.resize(cf_sz as usize * 128 * 128 / 2, 0);

        if frame_hdr.allow_screen_content_tools {
            // TODO: Fallible allocation
            f.frame_thread
                .pal
                .resize(num_sb128 as usize * 16 * 16 << hbd);

            let pal_idx_sz = num_sb128 * size_mul[1] as c_int;
            // TODO: Fallible allocation
            f.frame_thread
                .pal_idx
                .resize(pal_idx_sz as usize * 128 * 128 / 4, Default::default());
        } else if !f.frame_thread.pal.is_empty() {
            let _ = mem::take(&mut f.frame_thread.pal);
            let _ = mem::take(&mut f.frame_thread.pal_idx);
        }
    }

    // update allocation of block contexts for above
    let mut y_stride = f.cur.stride[0];
    let mut uv_stride = f.cur.stride[1];
    let has_resize = (frame_hdr.size.width[0] != frame_hdr.size.width[1]) as c_int;
    let need_cdef_lpf_copy = (c.tc.len() > 1 && has_resize != 0) as c_int;
    let mut alloc_sz: usize = 64;
    alloc_sz += (y_stride.unsigned_abs() * 4 * f.sbh as usize) << need_cdef_lpf_copy;
    alloc_sz += (uv_stride.unsigned_abs() * 8 * f.sbh as usize) << need_cdef_lpf_copy;
    // TODO: Fallible allocation.
    f.lf.cdef_line_buf.resize(alloc_sz, 0);

    let bpc = BPC::from_bitdepth_max(f.bitdepth_max);
    let y_stride_px = bpc.pxstride(f.cur.stride[0]);
    let uv_stride_px = bpc.pxstride(f.cur.stride[1]);

    let mut offset = bpc.pxstride(32usize);
    if y_stride < 0 {
        f.lf.cdef_line[0][0] =
            offset.wrapping_add_signed(-(y_stride_px * (f.sbh as isize * 4 - 1)));
        f.lf.cdef_line[1][0] =
            offset.wrapping_add_signed(-(y_stride_px * (f.sbh as isize * 4 - 3)));
    } else {
        f.lf.cdef_line[0][0] = offset.wrapping_add_signed(y_stride_px * 0);
        f.lf.cdef_line[1][0] = offset.wrapping_add_signed(y_stride_px * 2);
    }
    offset = offset.wrapping_add_signed(y_stride_px.abs() * f.sbh as isize * 4);
    if uv_stride < 0 {
        f.lf.cdef_line[0][1] =
            offset.wrapping_add_signed(-(uv_stride_px * (f.sbh as isize * 8 - 1)));
        f.lf.cdef_line[0][2] =
            offset.wrapping_add_signed(-(uv_stride_px * (f.sbh as isize * 8 - 3)));
        f.lf.cdef_line[1][1] =
            offset.wrapping_add_signed(-(uv_stride_px * (f.sbh as isize * 8 - 5)));
        f.lf.cdef_line[1][2] =
            offset.wrapping_add_signed(-(uv_stride_px * (f.sbh as isize * 8 - 7)));
    } else {
        f.lf.cdef_line[0][1] = offset.wrapping_add_signed(uv_stride_px * 0);
        f.lf.cdef_line[0][2] = offset.wrapping_add_signed(uv_stride_px * 2);
        f.lf.cdef_line[1][1] = offset.wrapping_add_signed(uv_stride_px * 4);
        f.lf.cdef_line[1][2] = offset.wrapping_add_signed(uv_stride_px * 6);
    }

    if need_cdef_lpf_copy != 0 {
        offset = offset.wrapping_add_signed(uv_stride_px.abs() * f.sbh as isize * 8);
        if y_stride < 0 {
            f.lf.cdef_lpf_line[0] =
                offset.wrapping_add_signed(-(y_stride_px * (f.sbh as isize * 4 - 1)));
        } else {
            f.lf.cdef_lpf_line[0] = offset;
        }
        offset = offset.wrapping_add_signed(y_stride_px.abs() * f.sbh as isize * 4);
        if uv_stride < 0 {
            f.lf.cdef_lpf_line[1] =
                offset.wrapping_add_signed(-(uv_stride_px * (f.sbh as isize * 4 - 1)));
            f.lf.cdef_lpf_line[2] =
                offset.wrapping_add_signed(-(uv_stride_px * (f.sbh as isize * 8 - 1)));
        } else {
            f.lf.cdef_lpf_line[1] = offset;
            f.lf.cdef_lpf_line[2] = offset.wrapping_add_signed(uv_stride_px * f.sbh as isize * 4);
        }
    }

    let sb128 = seq_hdr.sb128;
    let num_lines = if c.tc.len() > 1 {
        (f.sbh * 4) << sb128
    } else {
        12
    };
    y_stride = f.sr_cur.p.stride[0];
    uv_stride = f.sr_cur.p.stride[1];

    // lr simd may overread the input, so slightly over-allocate the lpf buffer
    let mut alloc_sz: usize = 128;
    alloc_sz += y_stride.unsigned_abs() * num_lines as usize;
    alloc_sz += uv_stride.unsigned_abs() * num_lines as usize * 2;
    // TODO: Fallible allocation
    f.lf.lr_line_buf.resize(alloc_sz, 0);

    let y_stride_px = bpc.pxstride(y_stride);
    let uv_stride_px = bpc.pxstride(uv_stride);

    let mut offset = bpc.pxstride(64usize);
    if y_stride < 0 {
        f.lf.lr_lpf_line[0] = offset.wrapping_add_signed(-(y_stride_px * (num_lines as isize - 1)));
    } else {
        f.lf.lr_lpf_line[0] = offset;
    }
    offset = offset.wrapping_add_signed(y_stride_px.abs() * num_lines as isize);
    if uv_stride < 0 {
        f.lf.lr_lpf_line[1] =
            offset.wrapping_add_signed(-(uv_stride_px * (num_lines as isize * 1 - 1)));
        f.lf.lr_lpf_line[2] =
            offset.wrapping_add_signed(-(uv_stride_px * (num_lines as isize * 2 - 1)));
    } else {
        f.lf.lr_lpf_line[1] = offset;
        f.lf.lr_lpf_line[2] = offset.wrapping_add_signed(uv_stride_px * num_lines as isize);
    }

    // update allocation for loopfilter masks

    f.lf.mask.clear();
    // TODO: Fallible allocation.
    f.lf.mask.resize_with(num_sb128 as usize, Default::default);
    // over-allocate one element (4 bytes) since some of the SIMD implementations
    // index this from the level type and can thus over-read by up to 3 bytes.
    f.lf.level
        .resize(num_sb128 as usize * 32 * 32 + 1, [0u8; 4]); // TODO: Fallible allocation
    if c.n_fc > 1 {
        // TODO: Fallible allocation
        f.frame_thread
            .b
            .resize_with(num_sb128 as usize * 32 * 32, Default::default);

        // TODO: fallible allocation
        f.frame_thread
            .cbi
            .resize_with(num_sb128 as usize * 32 * 32, Default::default);
    }

    f.sr_sb128w = f.sr_cur.p.p.w + 127 >> 7;
    let lr_mask_sz = f.sr_sb128w * f.sb128h;
    // TODO: Fallible allocation
    f.lf.lr_mask
        .resize_with(lr_mask_sz as usize, Default::default);
    f.lf.restore_planes = frame_hdr
        .restoration
        .r#type
        .iter()
        .enumerate()
        .map(|(i, &r#type)| ((r#type != Rav1dRestorationType::None) as u8) << i)
        .sum::<u8>()
        .into();
    if frame_hdr.loopfilter.sharpness != f.lf.last_sharpness {
        rav1d_calc_eih(&mut f.lf.lim_lut.0, frame_hdr.loopfilter.sharpness);
        f.lf.last_sharpness = frame_hdr.loopfilter.sharpness;
    }
    rav1d_calc_lf_values(&mut f.lf.lvl, &frame_hdr, &[0, 0, 0, 0]);

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
    // TODO: Fallible allocation
    f.lf.tx_lpf_right_edge.resize(re_sz as usize, 0);

    // init ref mvs
    if frame_hdr.frame_type.is_inter_or_switch() || frame_hdr.allow_intrabc {
        rav1d_refmvs_init_frame(
            &mut f.rf,
            seq_hdr,
            frame_hdr,
            &f.refpoc,
            f.mvs,
            &f.refrefpoc,
            &f.ref_mvs,
            c.tc.len(),
            c.n_fc as usize,
        )?;
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

    // Init loopfilter offsets. Point the chroma offsets in 4:0:0 to the luma
    // plane here to avoid having additional in-loop branches in various places.
    // We never use those values, so it doesn't really matter what they point
    // at, as long as the offsets are valid.
    let has_chroma = (f.cur.p.layout != Rav1dPixelLayout::I400) as usize;
    f.lf.p = array::from_fn(|i| has_chroma * i);
    f.lf.sr_p = array::from_fn(|i| has_chroma * i);

    Ok(())
}

pub(crate) unsafe fn rav1d_decode_frame_init_cdf(
    c: &Rav1dContext,
    f: &mut Rav1dFrameData,
) -> Rav1dResult {
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();

    if frame_hdr.refresh_context != 0 {
        rav1d_cdf_thread_copy(&mut f.out_cdf.cdf_write(), &f.in_cdf);
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

        let mut data = tile.data.as_ref();
        for (j, (ts, tile_start_off)) in iter::zip(
            slice::from_raw_parts_mut(f.ts, end + 1),
            if uses_2pass {
                &f.frame_thread.tile_start_off[..end + 1]
            } else {
                &[]
            }
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
            setup_tile(c, ts, f, cur_data, tile_row, tile_col, tile_start_off);
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

    if c.tc.len() > 1 {
        for (n, ctx) in f.a[..sb128w * rows * (1 + uses_2pass as usize)]
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

unsafe fn rav1d_decode_frame_main(c: &Rav1dContext, f: &mut Rav1dFrameData) -> Rav1dResult {
    assert!(c.tc.len() == 1);

    let Rav1dContextTaskType::Single(t) = &c.tc[0].task else {
        panic!("Expected a single-threaded context");
    };
    let mut t = t.lock().unwrap();

    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();

    for ctx in &mut f.a[..(f.sb128w * frame_hdr.tiling.rows) as usize] {
        reset_context(ctx, frame_hdr.frame_type.is_key_or_intra(), 0);
    }

    // no threading - we explicitly interleave tile/sbrow decoding
    // and post-filtering, so that the full process runs in-line
    let Rav1dFrameHeader_tiling { rows, cols, .. } = frame_hdr.tiling;
    let [rows, cols] = [rows, cols].map(|it| it.try_into().unwrap());
    // Need to clone this because `(f.bd_fn().filter_sbrow)(f, sby);` takes a `&mut` to `f` within the loop.
    let row_start_sb = frame_hdr.tiling.row_start_sb.clone();
    for (tile_row, sbh_start_end) in row_start_sb[..rows + 1].windows(2).take(rows).enumerate() {
        // Needed until #[feature(array_windows)] stabilizes; it should hopefully optimize out.
        let [sbh_start, sbh_end] = <[u16; 2]>::try_from(sbh_start_end).unwrap();

        let sbh_end = cmp::min(sbh_end.into(), f.sbh);

        for sby in sbh_start.into()..sbh_end {
            let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
            let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
            t.b.y = sby << 4 + seq_hdr.sb128;
            let by_end = t.b.y + f.sb_step >> 1;
            if frame_hdr.use_ref_frame_mvs != 0 {
                let rf = f.rf.as_dav1d();
                (c.refmvs_dsp.load_tmvs)(&rf, tile_row as c_int, 0, f.bw >> 1, t.b.y >> 1, by_end);
            }
            for col in 0..cols {
                t.ts = tile_row * cols + col;
                rav1d_decode_tile_sbrow(c, &mut t, f).map_err(|()| EINVAL)?;
            }
            if f.frame_hdr().frame_type.is_inter_or_switch() {
                rav1d_refmvs_save_tmvs(
                    &c.refmvs_dsp,
                    &t.rt,
                    &f.rf,
                    0,
                    f.bw >> 1,
                    t.b.y >> 1,
                    by_end,
                );
            }

            // loopfilter + cdef + restoration
            (f.bd_fn().filter_sbrow)(c, f, &mut t, sby);
        }
    }

    Ok(())
}

pub(crate) unsafe fn rav1d_decode_frame_exit(
    c: &Rav1dContext,
    f: &mut Rav1dFrameData,
    retval: Rav1dResult,
) {
    if !f.sr_cur.p.data.data[0].is_null() {
        f.task_thread.error = AtomicI32::new(0);
    }
    if c.n_fc > 1 && retval.is_err() && !f.frame_thread.cf.is_empty() {
        f.frame_thread.cf.fill_with(Default::default)
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
    let _ = mem::take(&mut f.in_cdf);
    if let Some(frame_hdr) = &f.frame_hdr {
        if frame_hdr.refresh_context != 0 {
            if let Some(progress) = f.out_cdf.progress() {
                progress.store(
                    if retval.is_ok() { 1 } else { TILE_ERROR as u32 },
                    Ordering::SeqCst,
                );
            }
            let _ = mem::take(&mut f.out_cdf);
        }
    }

    rav1d_ref_dec(&mut f.cur_segmap_ref);
    rav1d_ref_dec(&mut f.prev_segmap_ref);
    rav1d_ref_dec(&mut f.mvs_ref);
    let _ = mem::take(&mut f.seq_hdr);
    let _ = mem::take(&mut f.frame_hdr);
    f.tiles.clear();
    f.task_thread.retval = retval;
}

pub(crate) unsafe fn rav1d_decode_frame(c: &Rav1dContext, f: &mut Rav1dFrameData) -> Rav1dResult {
    assert!(c.n_fc == 1);
    // if.tc.len() > 1 (but n_fc == 1), we could run init/exit in the task
    // threads also. Not sure it makes a measurable difference.
    let mut res = rav1d_decode_frame_init(c, f);
    if res.is_ok() {
        res = rav1d_decode_frame_init_cdf(c, f);
    }
    // wait until all threads have completed
    if res.is_ok() {
        if c.tc.len() > 1 {
            res = rav1d_task_create_tile_sbrow(c, f, 0, 1);
            let mut task_thread_lock = (*f.task_thread.ttd).delayed_fg.lock().unwrap();
            (*f.task_thread.ttd).cond.notify_one();
            if res.is_ok() {
                while f.task_thread.done[0].load(Ordering::Relaxed) == 0
                // TODO(kkysen) Make `.task_counter` an `AtomicI32`, but that requires recursively removing `impl Copy`s.
                    || (*(addr_of_mut!(f.task_thread.task_counter) as *mut AtomicI32))
                        .load(Ordering::SeqCst)
                        > 0
                {
                    task_thread_lock = f.task_thread.cond.wait(task_thread_lock).unwrap();
                }
            }
            drop(task_thread_lock);
            res = f.task_thread.retval;
        } else {
            res = rav1d_decode_frame_main(c, f);
            let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
            if res.is_ok() && frame_hdr.refresh_context != 0 && f.task_thread.update_set {
                rav1d_cdf_thread_update(
                    frame_hdr,
                    &mut f.out_cdf.cdf_write(),
                    &(*f.ts.offset(frame_hdr.tiling.update as isize)).cdf,
                );
            }
        }
    }
    rav1d_decode_frame_exit(c, f, res);
    res
}

fn get_upscale_x0(in_w: c_int, out_w: c_int, step: c_int) -> c_int {
    let err = out_w * step - (in_w << 14);
    let x0 = (-(out_w - in_w << 13) + (out_w >> 1)) / out_w + 128 - err / 2;
    x0 & 0x3fff
}

pub unsafe fn rav1d_submit_frame(c: &mut Rav1dContext) -> Rav1dResult {
    // wait for c->out_delayed[next] and move into c->out if visible
    let (f, out, _task_thread_lock) = if c.n_fc > 1 {
        let mut task_thread_lock = c.task_thread.delayed_fg.lock().unwrap();
        let next = c.frame_thread.next;
        c.frame_thread.next += 1;
        if c.frame_thread.next == c.n_fc {
            c.frame_thread.next = 0;
        }

        let f = &mut *c.fc.offset(next as isize);
        while !f.tiles.is_empty() {
            task_thread_lock = f.task_thread.cond.wait(task_thread_lock).unwrap();
        }
        let out_delayed = &mut c.frame_thread.out_delayed[next as usize];
        if !out_delayed.p.data.data[0].is_null() || f.task_thread.error.load(Ordering::SeqCst) != 0
        {
            let first = c.task_thread.first.load(Ordering::SeqCst);
            if first + 1 < c.n_fc {
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
            // `cur` is not actually mutated from multiple threads concurrently
            let cur = c.task_thread.cur.load(Ordering::Relaxed);
            if cur != 0 && cur < c.n_fc {
                c.task_thread.cur.fetch_sub(1, Ordering::Relaxed);
            }
        }
        let error = f.task_thread.retval;
        if error.is_err() {
            f.task_thread.retval = Ok(());
            c.cached_error = error;
            *c.cached_error_props.get_mut().unwrap() = out_delayed.p.m.clone();
            rav1d_thread_picture_unref(out_delayed);
        } else if !out_delayed.p.data.data[0].is_null() {
            let progress = out_delayed.progress.as_ref().unwrap()[1].load(Ordering::Relaxed);
            if (out_delayed.visible || c.output_invisible_frames) && progress != FRAME_ERROR {
                rav1d_thread_picture_ref(&mut c.out, out_delayed);
                c.event_flags |= out_delayed.flags.into();
            }
            rav1d_thread_picture_unref(out_delayed);
        }
        (f, out_delayed as *mut _, Some(task_thread_lock))
    } else {
        (&mut *c.fc, &mut c.out as *mut _, None)
    };

    f.seq_hdr = c.seq_hdr.clone();
    f.frame_hdr = mem::take(&mut c.frame_hdr);
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    f.dsp = &mut c.dsp[seq_hdr.hbd as usize];

    let bpc = 8 + 2 * seq_hdr.hbd;

    unsafe fn on_error(f: &mut Rav1dFrameData, c: &Rav1dContext, out: *mut Rav1dThreadPicture) {
        f.task_thread.error = AtomicI32::new(1);
        let _ = mem::take(&mut f.in_cdf);
        if f.frame_hdr.as_ref().unwrap().refresh_context != 0 {
            let _ = mem::take(&mut f.out_cdf);
        }
        for i in 0..7 {
            if f.refp[i].p.frame_hdr.is_some() {
                rav1d_thread_picture_unref(&mut f.refp[i]);
            }
            rav1d_ref_dec(&mut f.ref_mvs_ref[i]);
        }
        rav1d_thread_picture_unref(out);
        rav1d_picture_unref_internal(&mut f.cur);
        rav1d_thread_picture_unref(&mut f.sr_cur);
        rav1d_ref_dec(&mut f.mvs_ref);
        let _ = mem::take(&mut f.seq_hdr);
        let _ = mem::take(&mut f.frame_hdr);
        *c.cached_error_props.lock().unwrap() = c.in_0.m.clone();

        f.tiles.clear();
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
                rav1d_cdef_dsp_init::<BitDepth8>(&mut dsp.cdef);
                rav1d_intra_pred_dsp_init::<BitDepth8>(&mut dsp.ipred);
                rav1d_itx_dsp_init::<BitDepth8>(&mut dsp.itx, bpc);
                rav1d_loop_filter_dsp_init::<BitDepth8>(&mut dsp.lf);
                rav1d_loop_restoration_dsp_init::<BitDepth8>(&mut dsp.lr, bpc);
                rav1d_mc_dsp_init::<BitDepth8>(&mut dsp.mc);
                dsp.fg = Rav1dFilmGrainDSPContext::new::<BitDepth8>();
            }
            #[cfg(feature = "bitdepth_16")]
            10 | 12 => {
                rav1d_cdef_dsp_init::<BitDepth16>(&mut dsp.cdef);
                rav1d_intra_pred_dsp_init::<BitDepth16>(&mut dsp.ipred);
                rav1d_itx_dsp_init::<BitDepth16>(&mut dsp.itx, bpc);
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
                on_error(f, c, out);
                return Err(ENOPROTOOPT);
            }
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
            if c.refs[pri_ref].p.p.data.data[0].is_null() {
                on_error(f, c, out);
                return Err(EINVAL);
            }
        }
        for i in 0..7 {
            let refidx = frame_hdr.refidx[i] as usize;
            if c.refs[refidx].p.p.data.data[0].is_null()
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
                on_error(f, c, out);
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
            f.gmv_warp_allowed[i] = (frame_hdr.gmv[i].r#type > Rav1dWarpedMotionType::Translation
                && !frame_hdr.force_integer_mv
                && !rav1d_get_shear_params(&frame_hdr.gmv[i])
                && f.svc[i][0].scale == 0) as u8;
        }
    }

    // setup entropy
    if frame_hdr.primary_ref_frame == RAV1D_PRIMARY_REF_NONE {
        f.in_cdf = rav1d_cdf_thread_init_static(frame_hdr.quant.yac);
    } else {
        let pri_ref = frame_hdr.refidx[frame_hdr.primary_ref_frame as usize] as usize;
        f.in_cdf = c.cdf[pri_ref].clone();
    }
    if frame_hdr.refresh_context != 0 {
        let res = rav1d_cdf_thread_alloc(c, (c.n_fc > 1) as c_int);
        if res.is_err() {
            on_error(f, c, out);
        }
        f.out_cdf = res?;
    }

    // FIXME qsort so tiles are in order (for frame threading)
    f.tiles.clear();
    mem::swap(&mut f.tiles, &mut c.tiles);

    // allocate frame

    // We must take itut_t35 out of the context before the call so borrowck can
    // see we mutably borrow `c.itut_t35` disjointly from the task thread lock.
    let itut_t35 = c.itut_t35.take();
    let res = rav1d_thread_picture_alloc(c, f, bpc, itut_t35);
    if res.is_err() {
        on_error(f, c, out);
        return res;
    }

    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();

    if frame_hdr.size.width[0] != frame_hdr.size.width[1] {
        let res = rav1d_picture_alloc_copy(c, &mut f.cur, frame_hdr.size.width[0], &mut f.sr_cur.p);
        if res.is_err() {
            on_error(f, c, out);
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
        rav1d_thread_picture_ref(out, &mut f.sr_cur);
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
    f.task_thread.error = AtomicI32::new(0);
    let uses_2pass = (c.n_fc > 1) as c_int;
    let cols = frame_hdr.tiling.cols;
    let rows = frame_hdr.tiling.rows;
    f.task_thread
        .task_counter
        .store(cols * rows + f.sbh << uses_2pass, Ordering::SeqCst);

    // ref_mvs
    if frame_hdr.frame_type.is_inter_or_switch() || frame_hdr.allow_intrabc {
        f.mvs_ref = rav1d_ref_create_using_pool(
            c.refmvs_pool,
            ::core::mem::size_of::<refmvs_temporal_block>()
                * f.sb128h as usize
                * 16
                * (f.b4_stride >> 1) as usize,
        );
        if f.mvs_ref.is_null() {
            on_error(f, c, out);
            return Err(ENOMEM);
        }
        f.mvs = (*f.mvs_ref).data.cast::<refmvs_temporal_block>();
        if !frame_hdr.allow_intrabc {
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
                on_error(f, c, out);
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
                on_error(f, c, out);
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

            if frame_hdr.refresh_context != 0 {
                c.cdf[i] = f.out_cdf.clone();
            } else {
                c.cdf[i] = f.in_cdf.clone();
            }

            rav1d_ref_dec(&mut c.refs[i].segmap);
            c.refs[i].segmap = f.cur_segmap_ref;
            if !f.cur_segmap_ref.is_null() {
                rav1d_ref_inc(f.cur_segmap_ref);
            }
            rav1d_ref_dec(&mut c.refs[i].refmvs);
            if !frame_hdr.allow_intrabc {
                c.refs[i].refmvs = f.mvs_ref;
                if !f.mvs_ref.is_null() {
                    rav1d_ref_inc(f.mvs_ref);
                }
            }
            c.refs[i].refpoc = f.refpoc;
        }
    }

    if c.n_fc == 1 {
        let res = rav1d_decode_frame(c, f);
        if res.is_err() {
            rav1d_thread_picture_unref(&mut c.out);
            for i in 0..8 {
                if refresh_frame_flags & (1 << i) != 0 {
                    if c.refs[i].p.p.frame_hdr.is_some() {
                        rav1d_thread_picture_unref(&mut c.refs[i].p);
                    }
                    let _ = mem::take(&mut c.cdf[i]);
                    rav1d_ref_dec(&mut c.refs[i].segmap);
                    rav1d_ref_dec(&mut c.refs[i].refmvs);
                }
            }
            on_error(f, c, out);
            return res;
        }
    } else {
        rav1d_task_frame_init(c, f);
    }

    Ok(())
}
