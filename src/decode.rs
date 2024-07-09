use crate::include::common::attributes::ctz;
use crate::include::common::bitdepth::BPC;
use crate::include::common::intops::apply_sign64;
use crate::include::common::intops::clip;
use crate::include::common::intops::clip_u8;
use crate::include::common::intops::iclip;
use crate::include::dav1d::common::Rav1dDataProps;
use crate::include::dav1d::headers::Rav1dFilterMode;
use crate::include::dav1d::headers::Rav1dFrameHeader;
use crate::include::dav1d::headers::Rav1dFrameHeaderTiling;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::include::dav1d::headers::Rav1dRestorationType;
use crate::include::dav1d::headers::Rav1dSequenceHeader;
use crate::include::dav1d::headers::Rav1dTxfmMode;
use crate::include::dav1d::headers::Rav1dWarpedMotionParams;
use crate::include::dav1d::headers::Rav1dWarpedMotionType;
use crate::include::dav1d::headers::SgrIdx;
use crate::include::dav1d::headers::RAV1D_PRIMARY_REF_NONE;
use crate::include::dav1d::picture::Rav1dPicture;
use crate::src::align::Align16;
use crate::src::align::AlignedVec64;
use crate::src::c_arc::CArc;
use crate::src::cdf::rav1d_cdf_thread_alloc;
use crate::src::cdf::rav1d_cdf_thread_copy;
use crate::src::cdf::rav1d_cdf_thread_init_static;
use crate::src::cdf::rav1d_cdf_thread_update;
use crate::src::cdf::CdfMvComponent;
use crate::src::cdf::CdfThreadContext;
use crate::src::ctx::CaseSet;
use crate::src::dequant_tables::dav1d_dq_tbl;
use crate::src::disjoint_mut::DisjointMut;
use crate::src::disjoint_mut::DisjointMutSlice;
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
use crate::src::error::Rav1dError::ENOPROTOOPT;
use crate::src::error::Rav1dResult;
use crate::src::extensions::OptionError as _;
use crate::src::internal::Bxy;
use crate::src::internal::Rav1dBitDepthDSPContext;
use crate::src::internal::Rav1dContext;
use crate::src::internal::Rav1dContextTaskType;
use crate::src::internal::Rav1dFrameContext;
use crate::src::internal::Rav1dFrameContextFrameThread;
use crate::src::internal::Rav1dFrameContextLf;
use crate::src::internal::Rav1dFrameData;
use crate::src::internal::Rav1dState;
use crate::src::internal::Rav1dTaskContext;
use crate::src::internal::Rav1dTileState;
use crate::src::internal::Rav1dTileStateContext;
use crate::src::internal::ScalableMotionParams;
use crate::src::internal::ScratchPal;
use crate::src::internal::TileStateRef;
use crate::src::intra_edge::EdgeFlags;
use crate::src::intra_edge::EdgeIndex;
use crate::src::intra_edge::IntraEdges;
use crate::src::levels::Av1Block;
use crate::src::levels::Av1BlockInter;
use crate::src::levels::Av1BlockInter1d;
use crate::src::levels::Av1BlockInter2d;
use crate::src::levels::Av1BlockInterNd;
use crate::src::levels::Av1BlockIntra;
use crate::src::levels::Av1BlockIntraInter;
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
use crate::src::levels::Mv;
use crate::src::levels::SegmentId;
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
use crate::src::levels::N_UV_INTRA_PRED_MODES;
use crate::src::levels::VERT_LEFT_PRED;
use crate::src::levels::VERT_PRED;
use crate::src::lf_mask::rav1d_calc_eih;
use crate::src::lf_mask::rav1d_calc_lf_values;
use crate::src::lf_mask::rav1d_create_lf_mask_inter;
use crate::src::lf_mask::rav1d_create_lf_mask_intra;
use crate::src::lf_mask::Av1RestorationUnit;
use crate::src::log::Rav1dLog as _;
use crate::src::lr_apply::LrRestorePlanes;
use crate::src::msac::rav1d_msac_decode_bool;
use crate::src::msac::rav1d_msac_decode_bool_adapt;
use crate::src::msac::rav1d_msac_decode_bool_equi;
use crate::src::msac::rav1d_msac_decode_bools;
use crate::src::msac::rav1d_msac_decode_subexp;
use crate::src::msac::rav1d_msac_decode_symbol_adapt16;
use crate::src::msac::rav1d_msac_decode_symbol_adapt4;
use crate::src::msac::rav1d_msac_decode_symbol_adapt8;
use crate::src::msac::rav1d_msac_decode_uniform;
use crate::src::msac::MsacContext;
use crate::src::pal::Rav1dPalDSPContext;
use crate::src::picture::rav1d_picture_alloc_copy;
use crate::src::picture::rav1d_thread_picture_alloc;
use crate::src::picture::Rav1dThreadPicture;
use crate::src::qm::dav1d_qm_tbl;
use crate::src::recon::debug_block_info;
use crate::src::refmvs::rav1d_refmvs_find;
use crate::src::refmvs::rav1d_refmvs_init_frame;
use crate::src::refmvs::rav1d_refmvs_tile_sbrow_init;
use crate::src::refmvs::RefMvsBlock;
use crate::src::refmvs::RefMvsFrame;
use crate::src::refmvs::RefMvsMvPair;
use crate::src::refmvs::RefMvsRefPair;
use crate::src::relaxed_atomic::RelaxedAtomic;
use crate::src::tables::cfl_allowed_mask;
use crate::src::tables::dav1d_al_part_ctx;
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
use std::iter;
use std::mem;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;
use strum::EnumCount;

fn init_quant_tables(
    seq_hdr: &Rav1dSequenceHeader,
    frame_hdr: &Rav1dFrameHeader,
    qidx: u8,
    dq: &[[[RelaxedAtomic<u16>; 2]; 3]; SegmentId::COUNT],
) {
    let tbl = &dav1d_dq_tbl[seq_hdr.hbd as usize];

    let segmentation_is_enabled = frame_hdr.segmentation.enabled != 0;
    let len = if segmentation_is_enabled {
        SegmentId::COUNT
    } else {
        1
    };
    for i in 0..len {
        let yac = if segmentation_is_enabled {
            clip_u8(qidx as c_int + frame_hdr.segmentation.seg_data.d[i].delta_q as c_int)
        } else {
            qidx
        } as i16;
        let ydc = clip_u8(yac + frame_hdr.quant.ydc_delta as i16);
        let uac = clip_u8(yac + frame_hdr.quant.uac_delta as i16);
        let udc = clip_u8(yac + frame_hdr.quant.udc_delta as i16);
        let vac = clip_u8(yac + frame_hdr.quant.vac_delta as i16);
        let vdc = clip_u8(yac + frame_hdr.quant.vdc_delta as i16);

        let dq = &dq[i];
        dq[0][0].set(tbl[ydc as usize][0]);
        dq[0][1].set(tbl[yac as usize][1]);
        dq[1][0].set(tbl[udc as usize][0]);
        dq[1][1].set(tbl[uac as usize][1]);
        dq[2][0].set(tbl[vdc as usize][0]);
        dq[2][1].set(tbl[vac as usize][1]);
    }
}

fn read_mv_component_diff(
    msac: &mut MsacContext,
    mv_comp: &mut CdfMvComponent,
    mv_prec: i32,
) -> c_int {
    let sign = rav1d_msac_decode_bool_adapt(msac, &mut mv_comp.sign.0);
    let cl = rav1d_msac_decode_symbol_adapt16(msac, &mut mv_comp.classes.0, 10);
    let mut up;
    let mut fp = 3;
    let mut hp = true;

    if cl == 0 {
        up = rav1d_msac_decode_bool_adapt(msac, &mut mv_comp.class_0.0) as u16;
        if mv_prec >= 0 {
            // !force_integer_mv
            fp = rav1d_msac_decode_symbol_adapt4(msac, &mut mv_comp.class_0_fp[up as usize], 3);
            if mv_prec > 0 {
                // allow_high_precision_mv
                hp = rav1d_msac_decode_bool_adapt(msac, &mut mv_comp.class_0_hp.0);
            }
        }
    } else {
        // `cl` is in the range `0..=10`, so `up` is a `u10`.
        up = 1 << cl;
        for n in 0..cl as usize {
            up |= (rav1d_msac_decode_bool_adapt(msac, &mut mv_comp.class_n[n]) as u16) << n;
        }
        if mv_prec >= 0 {
            // !force_integer_mv
            fp = rav1d_msac_decode_symbol_adapt4(msac, &mut mv_comp.class_n_fp.0, 3);
            if mv_prec > 0 {
                // allow_high_precision_mv
                hp = rav1d_msac_decode_bool_adapt(msac, &mut mv_comp.class_n_hp.0);
            }
        }
    }
    let hp = hp as u16;

    let diff = ((up << 3 | (fp as u16) << 1 | hp) + 1) as c_int;

    if sign {
        -diff
    } else {
        diff
    }
}

fn read_mv_residual(ts_c: &mut Rav1dTileStateContext, ref_mv: &mut Mv, mv_prec: i32) {
    let mv_joint = MVJoint::from_bits_truncate(rav1d_msac_decode_symbol_adapt4(
        &mut ts_c.msac,
        &mut ts_c.cdf.mv.joint.0,
        MVJoint::all().bits(),
    ) as u8);

    let mv_cdf = &mut ts_c.cdf.mv;

    if mv_joint.contains(MVJoint::V) {
        ref_mv.y += read_mv_component_diff(&mut ts_c.msac, &mut mv_cdf.comp[0], mv_prec) as i16;
    }
    if mv_joint.contains(MVJoint::H) {
        ref_mv.x += read_mv_component_diff(&mut ts_c.msac, &mut mv_cdf.comp[1], mv_prec) as i16;
    }
}

fn read_tx_tree(
    t: &mut Rav1dTaskContext,
    f: &Rav1dFrameData,
    ts_c: &mut Rav1dTileStateContext,
    from: TxfmSize,
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

    if depth < 2 && from > TxfmSize::S4x4 {
        let cat = 2 * (TxfmSize::S64x64 as c_int - t_dim.max as c_int) - depth;
        let a = ((*f.a[t.a].tx.index(bx4 as usize) as u8) < txw) as c_int;
        let l = ((*t.l.tx.index(by4 as usize) as u8) < txh) as c_int;

        is_split = rav1d_msac_decode_bool_adapt(
            &mut ts_c.msac,
            &mut ts_c.cdf.m.txpart[cat as usize][(a + l) as usize],
        );
        if is_split {
            masks[depth as usize] |= 1 << (y_off * 4 + x_off);
        }
    } else {
        is_split = false;
    }
    if is_split && t_dim.max > TxfmSize::S8x8 as _ {
        let sub = t_dim.sub;
        let sub_t_dim = &dav1d_txfm_dimensions[sub as usize];
        let txsw = sub_t_dim.w as c_int;
        let txsh = sub_t_dim.h as c_int;

        read_tx_tree(
            t,
            f,
            ts_c,
            sub,
            depth + 1,
            masks,
            x_off * 2 + 0,
            y_off * 2 + 0,
        );
        t.b.x += txsw;
        if txw >= txh && t.b.x < f.bw {
            read_tx_tree(
                t,
                f,
                ts_c,
                sub,
                depth + 1,
                masks,
                x_off * 2 + 1,
                y_off * 2 + 0,
            );
        }
        t.b.x -= txsw;
        t.b.y += txsh;
        if txh >= txw && t.b.y < f.bh {
            read_tx_tree(
                t,
                f,
                ts_c,
                sub,
                depth + 1,
                masks,
                x_off * 2 + 0,
                y_off * 2 + 1,
            );
            t.b.x += txsw;
            if txw >= txh && t.b.x < f.bw {
                read_tx_tree(
                    t,
                    f,
                    ts_c,
                    sub,
                    depth + 1,
                    masks,
                    x_off * 2 + 1,
                    y_off * 2 + 1,
                );
            }
            t.b.x -= txsw;
        }
        t.b.y -= txsh;
    } else {
        CaseSet::<16, false>::many(
            [(&t.l, txh), (&f.a[t.a], txw)],
            [t_dim.h as usize, t_dim.w as usize],
            [by4 as usize, bx4 as usize],
            |case, (dir, val)| {
                let tx = if is_split {
                    TxfmSize::S4x4
                } else {
                    // TODO check unwrap is optimized out
                    TxfmSize::from_repr(val as _).unwrap()
                };
                case.set_disjoint(&dir.tx, tx);
            },
        );
    };
}

fn neg_deinterleave(diff: u8, r#ref: SegmentId, max: u8) -> u8 {
    let r#ref = r#ref.get() as u8;
    if r#ref == 0 {
        diff
    } else if r#ref + 1 >= max {
        // The C code returns a signed integer which is immediately cast to `uint8_t`
        max.wrapping_sub(diff + 1)
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
            // The C code returns a signed integer which is immediately cast to `uint8_t`
            max.wrapping_sub(diff + 1)
        }
    }
}

fn find_matching_ref(
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
    let ts = &f.ts[t.ts];
    let mut have_topleft = have_top && have_left;
    let mut have_topright = cmp::max(bw4, bh4) < 32
        && have_top
        && t.b.x + bw4 < ts.tiling.col_end
        && intra_edge_flags.contains(EdgeFlags::I444_TOP_HAS_RIGHT);

    let bs = |rp: RefMvsBlock| rp.bs.dimensions();
    let matches = |rp: RefMvsBlock| rp.r#ref.r#ref[0] == r#ref + 1 && rp.r#ref.r#ref[1] == -1;

    if have_top {
        let mut i = r[0] + t.b.x as usize;
        let r2 = *f.rf.r.index(i);
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
                let r2 = *f.rf.r.index(i);
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
        let get_r2 = |i: usize| *f.rf.r.index(r[i] + t.b.x as usize - 1);

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
    if have_topleft && matches(*f.rf.r.index(r[0] + t.b.x as usize - 1)) {
        masks[1] |= 1 << 32;
        count += 1;
        if count >= 8 {
            return;
        }
    }
    if have_topright && matches(*f.rf.r.index(r[0] + t.b.x as usize + bw4 as usize)) {
        masks[0] |= 1 << 32;
    }
}

fn derive_warpmv(
    r: &DisjointMut<AlignedVec64<RefMvsBlock>>,
    t: &Rav1dTaskContext,
    bw4: c_int,
    bh4: c_int,
    masks: &[u64; 2],
    mv: Mv,
    mut wmp: Rav1dWarpedMotionParams,
) -> Rav1dWarpedMotionParams {
    let mut pts = [[[0; 2 /* x, y */]; 2 /* in, out */]; 8];
    let mut np = 0;
    let rp = |i: i32, j: i32| {
        // Need to use a closure here vs. a slice because `i` can be negative
        // (and not just by a constant -1).
        // See `-off` below.
        let offset = (t.b.y & 31) + 5;
        *r.index(t.rt.r[(offset as isize + i as isize) as usize] + j as usize)
    };

    let bs = |rp: RefMvsBlock| rp.bs.dimensions();

    let mut add_sample = |np: usize, dx: i32, dy: i32, sx: i32, sy: i32, rp: RefMvsBlock| {
        pts[np][0][0] = 16 * (2 * dx + sx * bs(rp)[0] as i32) - 8;
        pts[np][0][1] = 16 * (2 * dy + sy * bs(rp)[1] as i32) - 8;
        pts[np][1][0] = pts[np][0][0] + rp.mv.mv[0].x as i32;
        pts[np][1][1] = pts[np][0][1] + rp.mv.mv[0].y as i32;
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

fn read_pal_indices(
    ts_c: &mut Rav1dTileStateContext,
    pal_dsp: &Rav1dPalDSPContext,
    scratch_pal: &mut ScratchPal,
    pal_tmp: &mut [u8],
    pal_idx: Option<&mut [u8]>, // if None, use pal_tmp instead of pal_idx
    pal_sz: u8,
    pl: bool,
    w4: c_int,
    h4: c_int,
    bw4: c_int,
    bh4: c_int,
) {
    let [w4, h4, bw4, bh4] = [w4, h4, bw4, bh4].map(|n| usize::try_from(n).unwrap());
    let pli = pl as usize;

    let stride = bw4 * 4;
    pal_tmp[0] = rav1d_msac_decode_uniform(&mut ts_c.msac, pal_sz as c_uint) as u8;
    let color_map_cdf = &mut ts_c.cdf.m.color_map[pli][pal_sz as usize - 2];
    let ScratchPal {
        pal_order: order,
        pal_ctx: ctx,
    } = scratch_pal;
    for i in 1..4 * (w4 + h4) - 1 {
        // top/left-to-bottom/right diagonals ("wave-front")
        let first = cmp::min(i, w4 * 4 - 1);
        let last = (i + 1).checked_sub(h4 * 4).unwrap_or(0);
        order_palette(pal_tmp, stride, i, first, last, order, ctx);
        for (m, j) in (last..=first).rev().enumerate() {
            let color_idx = rav1d_msac_decode_symbol_adapt8(
                &mut ts_c.msac,
                &mut color_map_cdf[ctx[m] as usize],
                pal_sz as u8 - 1,
            ) as usize;
            pal_tmp[(i - j) * stride + j] = order[m][color_idx];
        }
    }
    // fill invisible edges and pack to 4-bit (2 pixels per byte)
    if bw4 > w4 {
        for y in 0..4 * h4 {
            let offset = y * stride + (4 * w4);
            let len = 4 * (bw4 - w4);
            let filler = pal_tmp[offset - 1];
            pal_tmp[offset..][..len].fill(filler);
        }
    }

    pal_dsp
        .pal_idx_finish
        .call(pal_idx, pal_tmp, bw4 * 4, bh4 * 4, w4 * 4, h4 * 4);
}

struct VarTx {
    uvtx: TxfmSize,
    max_ytx: TxfmSize,
    tx_split0: u8,
    tx_split1: u16,
}

// not inlined in C and inlining in Rust degrades performance slightly
#[inline(never)]
fn read_vartx_tree(
    t: &mut Rav1dTaskContext,
    f: &Rav1dFrameData,
    ts_c: &mut Rav1dTileStateContext,
    b: &Av1Block,
    bs: BlockSize,
    bx4: c_int,
    by4: c_int,
) -> VarTx {
    let b_dim = bs.dimensions();
    let bw4 = b_dim[0] as usize;
    let bh4 = b_dim[1] as usize;

    // var-tx tree coding
    let mut tx_split = [0u16; 2];
    let mut max_ytx = dav1d_max_txfm_size_for_bs[bs as usize][0];
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    let txfm_mode = frame_hdr.txfm_mode;
    let uvtx;
    if b.skip == 0 && (frame_hdr.segmentation.lossless[b.seg_id.get()] || max_ytx == TxfmSize::S4x4)
    {
        uvtx = TxfmSize::S4x4;
        max_ytx = uvtx;
        if txfm_mode == Rav1dTxfmMode::Switchable {
            CaseSet::<32, false>::many(
                [&t.l, &f.a[t.a]],
                [bh4 as usize, bw4 as usize],
                [by4 as usize, bx4 as usize],
                |case, dir| {
                    case.set_disjoint(&dir.tx, TxfmSize::S4x4);
                },
            );
        }
    } else if txfm_mode != Rav1dTxfmMode::Switchable || b.skip != 0 {
        if txfm_mode == Rav1dTxfmMode::Switchable {
            CaseSet::<32, false>::many(
                [(&t.l, 1), (&f.a[t.a], 0)],
                [bh4 as usize, bw4 as usize],
                [by4 as usize, bx4 as usize],
                |case, (dir, dir_index)| {
                    // TODO check unwrap is optimized out
                    let tx = TxfmSize::from_repr(b_dim[2 + dir_index] as _).unwrap();
                    case.set_disjoint(&dir.tx, tx);
                },
            );
        }
        uvtx = dav1d_max_txfm_size_for_bs[bs as usize][f.cur.p.layout as usize];
    } else {
        assert!(bw4 <= 16 || bh4 <= 16 || max_ytx == TxfmSize::S64x64);
        let ytx = &dav1d_txfm_dimensions[max_ytx as usize];
        let h = ytx.h as usize;
        let w = ytx.w as usize;
        debug_assert_eq!(bh4 % h, 0);
        debug_assert_eq!(bw4 % w, 0);
        for y_off in 0..bh4 / h {
            for x_off in 0..bw4 / w {
                read_tx_tree(t, f, ts_c, max_ytx, 0, &mut tx_split, x_off, y_off);
                // contexts are updated inside read_tx_tree()
                t.b.x += w as c_int;
            }
            t.b.x -= bw4 as c_int;
            t.b.y += h as c_int;
        }
        t.b.y -= bh4 as c_int;
        if debug_block_info!(f, t.b) {
            println!(
                "Post-vartxtree[{}/{}]: r={}",
                tx_split[0], tx_split[1], ts_c.msac.rng
            );
        }
        uvtx = dav1d_max_txfm_size_for_bs[bs as usize][f.cur.p.layout as usize];
    }
    assert!(tx_split[0] & !0x33 == 0);
    let tx_split0 = tx_split[0] as u8;
    let tx_split1 = tx_split[1];

    VarTx {
        uvtx,
        max_ytx,
        tx_split0,
        tx_split1,
    }
}

fn get_prev_frame_segid(
    frame_hdr: &Rav1dFrameHeader,
    b: Bxy,
    w4: c_int,
    h4: c_int,
    ref_seg_map: &DisjointMutSlice<SegmentId>,
    stride: ptrdiff_t,
) -> SegmentId {
    assert!(frame_hdr.primary_ref_frame != RAV1D_PRIMARY_REF_NONE);

    let mut prev_seg_id = SegmentId::max();
    for y in 0..h4 as usize {
        let offset = (b.y as usize + y) * stride as usize + b.x as usize;
        prev_seg_id = ref_seg_map
            .index((offset.., ..w4 as usize))
            .iter()
            .copied()
            .fold(prev_seg_id, cmp::min);
        if prev_seg_id == SegmentId::min() {
            break;
        }
    }

    prev_seg_id
}

#[inline]
fn splat_oneref_mv(
    c: &Rav1dContext,
    t: &Rav1dTaskContext,
    rf: &RefMvsFrame,
    bs: BlockSize,
    inter: &Av1BlockInter,
    bw4: usize,
    bh4: usize,
) {
    let mode = inter.inter_mode;
    let tmpl = Align16(RefMvsBlock {
        mv: RefMvsMvPair {
            mv: [inter.nd.one_d.mv[0], Mv::ZERO],
        },
        r#ref: RefMvsRefPair {
            r#ref: [
                inter.r#ref[0] + 1,
                inter.interintra_type.map(|_| 0).unwrap_or(-1),
            ],
        },
        bs,
        mf: (mode == GLOBALMV && cmp::min(bw4, bh4) >= 2) as u8 | (mode == NEWMV) as u8 * 2,
    });

    c.dsp.refmvs.splat_mv.call(rf, &t.rt, &tmpl, t.b, bw4, bh4);
}

#[inline]
fn splat_intrabc_mv(
    c: &Rav1dContext,
    t: &Rav1dTaskContext,
    rf: &RefMvsFrame,
    bs: BlockSize,
    r#ref: Mv,
    bw4: usize,
    bh4: usize,
) {
    let tmpl = Align16(RefMvsBlock {
        mv: RefMvsMvPair {
            mv: [r#ref, Mv::ZERO],
        },
        r#ref: RefMvsRefPair { r#ref: [0, -1] },
        bs,
        mf: 0,
    });
    c.dsp.refmvs.splat_mv.call(rf, &t.rt, &tmpl, t.b, bw4, bh4);
}

#[inline]
fn splat_tworef_mv(
    c: &Rav1dContext,
    t: &Rav1dTaskContext,
    rf: &RefMvsFrame,
    bs: BlockSize,
    inter: &Av1BlockInter,
    bw4: usize,
    bh4: usize,
) {
    assert!(bw4 >= 2 && bh4 >= 2);
    let mode = inter.inter_mode;
    let tmpl = Align16(RefMvsBlock {
        mv: RefMvsMvPair {
            mv: inter.nd.one_d.mv,
        },
        r#ref: RefMvsRefPair {
            r#ref: [inter.r#ref[0] + 1, inter.r#ref[1] + 1],
        },
        bs,
        mf: (mode == GLOBALMV_GLOBALMV) as u8 | (1 << mode & 0xbc != 0) as u8 * 2,
    });
    c.dsp.refmvs.splat_mv.call(rf, &t.rt, &tmpl, t.b, bw4, bh4);
}

#[inline]
fn splat_intraref(
    c: &Rav1dContext,
    t: &Rav1dTaskContext,
    rf: &RefMvsFrame,
    bs: BlockSize,
    bw4: usize,
    bh4: usize,
) {
    let tmpl = Align16(RefMvsBlock {
        mv: RefMvsMvPair {
            mv: [Mv::INVALID, Mv::ZERO],
        },
        r#ref: RefMvsRefPair { r#ref: [0, -1] },
        bs,
        mf: 0,
    });
    c.dsp.refmvs.splat_mv.call(rf, &t.rt, &tmpl, t.b, bw4, bh4);
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
fn affine_lowest_px_chroma(
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

fn obmc_lowest_px(
    r: &DisjointMut<AlignedVec64<RefMvsBlock>>,
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
            let a_r = *r.index(ri[0] + t.b.x as usize + x as usize + 1);
            let a_b_dim = a_r.bs.dimensions();
            if a_r.r#ref.r#ref[0] as c_int > 0 {
                let oh4 = cmp::min(b_dim[1] as c_int, 16) >> 1;
                mc_lowest_px(
                    &mut dst[a_r.r#ref.r#ref[0] as usize - 1][is_chroma as usize],
                    t.b.y,
                    oh4 * 3 + 3 >> 2,
                    a_r.mv.mv[0].y,
                    ss_ver,
                    &svc[a_r.r#ref.r#ref[0] as usize - 1][1],
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
            let l_r = *r.index(ri[y as usize + 1 + 1] + t.b.x as usize - 1);
            let l_b_dim = l_r.bs.dimensions();
            if l_r.r#ref.r#ref[0] as c_int > 0 {
                let oh4 = iclip(l_b_dim[1] as c_int, 2, b_dim[1] as c_int);
                mc_lowest_px(
                    &mut dst[l_r.r#ref.r#ref[0] as usize - 1][is_chroma as usize],
                    t.b.y + y,
                    oh4,
                    l_r.mv.mv[0].y,
                    ss_ver,
                    &svc[l_r.r#ref.r#ref[0] as usize - 1][1],
                );
                i += 1;
            }
            y += cmp::max(l_b_dim[1] as c_int, 2);
        }
    }
}

fn decode_b(
    c: &Rav1dContext,
    t: &mut Rav1dTaskContext,
    f: &Rav1dFrameData,
    pass: &mut FrameThreadPassState,
    bl: BlockLevel,
    bs: BlockSize,
    bp: BlockPartition,
    intra_edge_flags: EdgeFlags,
) -> Result<(), ()> {
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    use std::fmt;

    /// Helper struct for printing a number as a signed hexidecimal value.
    struct SignAbs(i32);

    impl fmt::Display for SignAbs {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let sign = if self.0 < 0 { "-" } else { " " };
            write!(f, "{}{:x}", sign, self.0.abs())
        }
    }

    let mut b_mem = Av1Block::default();
    let b = if t.frame_thread.pass != 0 {
        &mut *f
            .frame_thread
            .b
            .index_mut((t.b.y as isize * f.b4_stride + t.b.x as isize) as usize)
    } else {
        &mut b_mem
    };

    let ts = &f.ts[t.ts];
    let ta = &f.a[t.a];
    let bd_fn = f.bd_fn();
    let b_dim = bs.dimensions();
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

    let FrameThreadPassState::First(ts_c) = pass else {
        match &b.ii {
            Av1BlockIntraInter::Intra(intra) => {
                (bd_fn.recon_b_intra)(f, t, None, bs, intra_edge_flags, b, intra);

                let y_mode = intra.y_mode;
                let y_mode_nofilt = if y_mode == FILTER_PRED {
                    DC_PRED
                } else {
                    y_mode
                };
                CaseSet::<32, false>::many(
                    [&t.l, ta],
                    [bh4 as usize, bw4 as usize],
                    [by4 as usize, bx4 as usize],
                    |case, dir| {
                        case.set_disjoint(&dir.mode, y_mode_nofilt);
                        case.set_disjoint(&dir.intra, 1);
                    },
                );
                if frame_type.is_inter_or_switch() {
                    let ri = t.rt.r[(t.b.y as usize & 31) + 5 + bh4 as usize - 1] + t.b.x as usize;
                    let r = &mut *f.rf.r.index_mut(ri..ri + bw4 as usize);
                    for block in r {
                        block.r#ref.r#ref[0] = 0;
                        block.bs = bs;
                    }
                    let rr = &t.rt.r[(t.b.y as usize & 31) + 5..][..bh4 as usize - 1];
                    for r in rr {
                        let block = &mut f.rf.r.index_mut(r + t.b.x as usize + bw4 as usize - 1);
                        block.r#ref.r#ref[0] = 0;
                        block.bs = bs;
                    }
                }

                if has_chroma {
                    CaseSet::<32, false>::many(
                        [&t.l, ta],
                        [cbh4 as usize, cbw4 as usize],
                        [cby4 as usize, cbx4 as usize],
                        |case, dir| {
                            case.set_disjoint(&dir.uvmode, intra.uv_mode);
                        },
                    );
                }
            }
            Av1BlockIntraInter::Inter(inter) => {
                if frame_type.is_inter_or_switch() /* not intrabc */
                && inter.comp_type.is_none()
                && inter.motion_mode == MotionMode::Warp
                {
                    let two_d = inter.nd.two_d();
                    if two_d.matrix[0] == i16::MIN {
                        t.warpmv.r#type = Rav1dWarpedMotionType::Identity;
                    } else {
                        t.warpmv.r#type = Rav1dWarpedMotionType::Affine;
                        t.warpmv.matrix[2] = two_d.matrix[0] as i32 + 0x10000;
                        t.warpmv.matrix[3] = two_d.matrix[1] as i32;
                        t.warpmv.matrix[4] = two_d.matrix[2] as i32;
                        t.warpmv.matrix[5] = two_d.matrix[3] as i32 + 0x10000;
                        rav1d_set_affine_mv2d(bw4, bh4, two_d.mv2d, &mut t.warpmv, t.b.x, t.b.y);
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
                                two_d.mv2d.y,
                                two_d.mv2d.x,
                            );
                        }
                    }
                }

                (bd_fn.recon_b_inter)(f, t, None, bs, b, inter)?;

                let filter = &dav1d_filter_dir[inter.filter2d as usize];
                CaseSet::<32, false>::many(
                    [&t.l, ta],
                    [bh4 as usize, bw4 as usize],
                    [by4 as usize, bx4 as usize],
                    |case, dir| {
                        case.set_disjoint(&dir.filter[0], filter[0].into());
                        case.set_disjoint(&dir.filter[1], filter[1].into());
                        case.set_disjoint(&dir.intra, 0);
                    },
                );

                if frame_type.is_inter_or_switch() {
                    let ri = t.rt.r[(t.b.y as usize & 31) + 5 + bh4 as usize - 1] + t.b.x as usize;
                    let r = &mut *f.rf.r.index_mut(ri..ri + bw4 as usize);
                    for block in r {
                        block.r#ref.r#ref[0] = inter.r#ref[0] + 1;
                        block.mv.mv[0] = inter.nd.one_d.mv[0];
                        block.bs = bs;
                    }
                    let rr = &t.rt.r[(t.b.y as usize & 31) + 5..][..bh4 as usize - 1];
                    for r in rr {
                        let block = &mut f.rf.r.index_mut(r + t.b.x as usize + bw4 as usize - 1);
                        block.r#ref.r#ref[0] = inter.r#ref[0] + 1;
                        block.mv.mv[0] = inter.nd.one_d.mv[0];
                        block.bs = bs;
                    }
                }

                if has_chroma {
                    CaseSet::<32, false>::many(
                        [&t.l, ta],
                        [cbh4 as usize, cbw4 as usize],
                        [cby4 as usize, cbx4 as usize],
                        |case, dir| {
                            case.set_disjoint(&dir.uvmode, DC_PRED);
                        },
                    );
                }
            }
        }

        return Ok(());
    };

    let ts_c = &mut **ts_c;

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
            b.seg_id = f
                .prev_segmap
                .as_ref()
                .map(|prev_segmap| {
                    get_prev_frame_segid(frame_hdr, t.b, w4, h4, &prev_segmap.inner, f.b4_stride)
                })
                .unwrap_or_default();
            seg = Some(&frame_hdr.segmentation.seg_data.d[b.seg_id.get()]);
        } else if frame_hdr.segmentation.seg_data.preskip != 0 {
            if frame_hdr.segmentation.temporal != 0 && {
                let index = *ta.seg_pred.index(bx4 as usize) + *t.l.seg_pred.index(by4 as usize);
                seg_pred = rav1d_msac_decode_bool_adapt(
                    &mut ts_c.msac,
                    &mut ts_c.cdf.mi.seg_pred.0[index as usize],
                );
                seg_pred
            } {
                // temporal predicted seg_id
                b.seg_id = f
                    .prev_segmap
                    .as_ref()
                    .map(|prev_segmap| {
                        get_prev_frame_segid(
                            frame_hdr,
                            t.b,
                            w4,
                            h4,
                            &prev_segmap.inner,
                            f.b4_stride,
                        )
                    })
                    .unwrap_or_default();
            } else {
                let (pred_seg_id, seg_ctx) = get_cur_frame_segid(
                    t.b,
                    have_top,
                    have_left,
                    &f.cur_segmap.as_ref().unwrap().inner,
                    f.b4_stride as usize,
                );
                let diff = rav1d_msac_decode_symbol_adapt8(
                    &mut ts_c.msac,
                    &mut ts_c.cdf.m.seg_id[seg_ctx as usize],
                    SegmentId::COUNT as u8 - 1,
                );
                let last_active_seg_id_plus1 =
                    (frame_hdr.segmentation.seg_data.last_active_segid + 1) as u8;
                let mut seg_id =
                    neg_deinterleave(diff as u8, pred_seg_id, last_active_seg_id_plus1);
                if seg_id >= last_active_seg_id_plus1 {
                    seg_id = 0; // error?
                }
                b.seg_id = SegmentId::new(seg_id).unwrap_or_default(); // error?
            }

            if debug_block_info!(f, t.b) {
                println!("Post-segid[preskip;{}]: r={}", b.seg_id, ts_c.msac.rng);
            }

            seg = Some(&frame_hdr.segmentation.seg_data.d[b.seg_id.get()]);
        }
    } else {
        b.seg_id = Default::default();
    }

    // skip_mode
    if seg
        .map(|seg| seg.globalmv == 0 && seg.r#ref == -1 && seg.skip == 0)
        .unwrap_or(true)
        && frame_hdr.skip_mode.enabled != 0
        && cmp::min(bw4, bh4) > 1
    {
        let smctx = *ta.skip_mode.index(bx4 as usize) + *t.l.skip_mode.index(by4 as usize);
        b.skip_mode = rav1d_msac_decode_bool_adapt(
            &mut ts_c.msac,
            &mut ts_c.cdf.mi.skip_mode.0[smctx as usize],
        ) as u8;
        if debug_block_info!(f, t.b) {
            println!("Post-skipmode[{}]: r={}", b.skip_mode, ts_c.msac.rng);
        }
    } else {
        b.skip_mode = 0;
    }

    // skip
    if b.skip_mode != 0 || seg.map(|seg| seg.skip != 0).unwrap_or(false) {
        b.skip = 1;
    } else {
        let sctx = *ta.skip.index(bx4 as usize) + *t.l.skip.index(by4 as usize);
        b.skip =
            rav1d_msac_decode_bool_adapt(&mut ts_c.msac, &mut ts_c.cdf.m.skip[sctx as usize]) as u8;
        if debug_block_info!(f, t.b) {
            println!("Post-skip[{}]: r={}", b.skip, ts_c.msac.rng);
        }
    }

    // segment_id
    if frame_hdr.segmentation.enabled != 0
        && frame_hdr.segmentation.update_map != 0
        && frame_hdr.segmentation.seg_data.preskip == 0
    {
        if b.skip == 0 && frame_hdr.segmentation.temporal != 0 && {
            let index = *ta.seg_pred.index(bx4 as usize) + *t.l.seg_pred.index(by4 as usize);
            seg_pred = rav1d_msac_decode_bool_adapt(
                &mut ts_c.msac,
                &mut ts_c.cdf.mi.seg_pred.0[index as usize],
            );
            seg_pred
        } {
            // temporal predicted seg_id
            b.seg_id = f
                .prev_segmap
                .as_ref()
                .map(|prev_segmap| {
                    get_prev_frame_segid(frame_hdr, t.b, w4, h4, &prev_segmap.inner, f.b4_stride)
                })
                .unwrap_or_default();
        } else {
            let (pred_seg_id, seg_ctx) = get_cur_frame_segid(
                t.b,
                have_top,
                have_left,
                &f.cur_segmap.as_ref().unwrap().inner,
                f.b4_stride as usize,
            );
            b.seg_id = if b.skip != 0 {
                pred_seg_id
            } else {
                let diff = rav1d_msac_decode_symbol_adapt8(
                    &mut ts_c.msac,
                    &mut ts_c.cdf.m.seg_id[seg_ctx as usize],
                    SegmentId::COUNT as u8 - 1,
                );
                let last_active_seg_id_plus1 =
                    (frame_hdr.segmentation.seg_data.last_active_segid + 1) as u8;
                let mut seg_id =
                    neg_deinterleave(diff as u8, pred_seg_id, last_active_seg_id_plus1);
                if seg_id >= last_active_seg_id_plus1 {
                    seg_id = 0; // error?
                }
                SegmentId::new(seg_id).unwrap_or_default() // error?
            };
        }

        seg = Some(&frame_hdr.segmentation.seg_data.d[b.seg_id.get()]);

        if debug_block_info!(f, t.b) {
            println!("Post-segid[postskip;{}]: r={}", b.seg_id, ts_c.msac.rng);
        }
    }

    // cdef index
    if b.skip == 0 {
        let idx = if seq_hdr.sb128 != 0 {
            ((t.b.x & 16) >> 4) + ((t.b.y & 16) >> 3)
        } else {
            0
        } as usize;
        let cdef_idx = &f.lf.mask[t.lf_mask.unwrap()].cdef_idx;
        let cur_idx = t.cur_sb_cdef_idx + idx;
        if cdef_idx[cur_idx].get() == -1 {
            let v = rav1d_msac_decode_bools(&mut ts_c.msac, frame_hdr.cdef.n_bits) as i8;
            cdef_idx[cur_idx].set(v);
            if bw4 > 16 {
                cdef_idx[cur_idx + 1].set(v)
            }
            if bh4 > 16 {
                cdef_idx[cur_idx + 2].set(v)
            }
            if bw4 == 32 && bh4 == 32 {
                cdef_idx[cur_idx + 3].set(v)
            }

            if debug_block_info!(f, t.b) {
                println!(
                    "Post-cdef_idx[{}]: r={}",
                    cdef_idx[t.cur_sb_cdef_idx].get(),
                    ts_c.msac.rng
                );
            }
        }
    }

    // delta-q/lf
    let not_sb128 = (seq_hdr.sb128 == 0) as c_int;
    if t.b.x & (31 >> not_sb128) == 0 && t.b.y & (31 >> not_sb128) == 0 {
        let prev_qidx = ts.last_qidx.get();
        let have_delta_q = frame_hdr.delta.q.present != 0
            && (bs
                != (if seq_hdr.sb128 != 0 {
                    BlockSize::Bs128x128
                } else {
                    BlockSize::Bs64x64
                })
                || b.skip == 0);

        let prev_delta_lf = ts.last_delta_lf.get();

        if have_delta_q {
            let mut delta_q =
                rav1d_msac_decode_symbol_adapt4(&mut ts_c.msac, &mut ts_c.cdf.m.delta_q.0, 3)
                    as c_int;
            if delta_q == 3 {
                let n_bits = 1 + rav1d_msac_decode_bools(&mut ts_c.msac, 3) as u8;
                delta_q =
                    (rav1d_msac_decode_bools(&mut ts_c.msac, n_bits) + 1 + (1 << n_bits)) as c_int;
            }
            if delta_q != 0 {
                if rav1d_msac_decode_bool_equi(&mut ts_c.msac) {
                    delta_q = -delta_q;
                }
                delta_q *= 1 << frame_hdr.delta.q.res_log2;
            }
            let last_qidx = clip(ts.last_qidx.get() as c_int + delta_q, 1, 255);
            ts.last_qidx.set(last_qidx);
            if have_delta_q && debug_block_info!(f, t.b) {
                println!(
                    "Post-delta_q[{}->{}]: r={}",
                    delta_q, last_qidx, ts_c.msac.rng
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

                let mut last_delta_lf = ts.last_delta_lf.get();
                for i in 0..n_lfs as usize {
                    let delta_lf_index = i + frame_hdr.delta.lf.multi as usize;
                    let mut delta_lf = rav1d_msac_decode_symbol_adapt4(
                        &mut ts_c.msac,
                        &mut ts_c.cdf.m.delta_lf[delta_lf_index],
                        3,
                    ) as c_int;
                    if delta_lf == 3 {
                        let n_bits = 1 + rav1d_msac_decode_bools(&mut ts_c.msac, 3) as u8;
                        delta_lf = (rav1d_msac_decode_bools(&mut ts_c.msac, n_bits)
                            + 1
                            + (1 << n_bits)) as c_int;
                    }
                    if delta_lf != 0 {
                        if rav1d_msac_decode_bool_equi(&mut ts_c.msac) {
                            delta_lf = -delta_lf;
                        }
                        delta_lf *= 1 << frame_hdr.delta.lf.res_log2;
                    }
                    last_delta_lf[i] = clip(last_delta_lf[i] as c_int + delta_lf, -63, 63);
                    if have_delta_q && debug_block_info!(f, t.b) {
                        println!("Post-delta_lf[{}:{}]: r={}", i, delta_lf, ts_c.msac.rng);
                    }
                }
                ts.last_delta_lf.set(last_delta_lf);
            }
        }
        let last_qidx = ts.last_qidx.get();
        if last_qidx == frame_hdr.quant.yac {
            // assign frame-wide q values to this sb
            ts.dq.set(TileStateRef::Frame);
        } else if last_qidx != prev_qidx {
            // find sb-specific quant parameters
            init_quant_tables(seq_hdr, frame_hdr, last_qidx, &ts.dqmem);
            ts.dq.set(TileStateRef::Local);
        }
        let last_delta_lf = ts.last_delta_lf.get();
        if last_delta_lf == [0, 0, 0, 0] {
            // assign frame-wide lf values to this sb
            ts.lflvl.set(TileStateRef::Frame);
        } else if last_delta_lf != prev_delta_lf {
            // find sb-specific lf lvl parameters
            rav1d_calc_lf_values(
                &mut (*ts.lflvlmem.try_write().unwrap()),
                frame_hdr,
                &last_delta_lf,
            );
            ts.lflvl.set(TileStateRef::Local);
        }
    }

    let intra = if b.skip_mode != 0 {
        false
    } else if frame_hdr.frame_type.is_inter_or_switch() {
        if let Some(seg) = seg.filter(|seg| seg.r#ref >= 0 || seg.globalmv != 0) {
            seg.r#ref == 0
        } else {
            let ictx = get_intra_ctx(&ta, &t.l, by4, bx4, have_top, have_left);
            let intra =
                !rav1d_msac_decode_bool_adapt(&mut ts_c.msac, &mut ts_c.cdf.mi.intra[ictx.into()]);
            if debug_block_info!(f, t.b) {
                println!("Post-intra[{}]: r={}", intra, ts_c.msac.rng);
            }
            intra
        }
    } else if frame_hdr.allow_intrabc {
        let intra = !rav1d_msac_decode_bool_adapt(&mut ts_c.msac, &mut ts_c.cdf.m.intrabc.0);
        if debug_block_info!(f, t.b) {
            println!("Post-intrabcflag[{}]: r={}", intra, ts_c.msac.rng);
        }
        intra
    } else {
        true
    };

    // intra/inter-specific stuff
    if intra {
        let ymode_cdf = if frame_hdr.frame_type.is_inter_or_switch() {
            &mut ts_c.cdf.mi.y_mode[dav1d_ymode_size_context[bs as usize] as usize]
        } else {
            &mut ts_c.cdf.kfym
                [dav1d_intra_mode_context[*ta.mode.index(bx4 as usize) as usize] as usize]
                [dav1d_intra_mode_context[*t.l.mode.index(by4 as usize) as usize] as usize]
        };
        let y_mode = rav1d_msac_decode_symbol_adapt16(
            &mut ts_c.msac,
            ymode_cdf,
            N_INTRA_PRED_MODES as u8 - 1,
        );
        if debug_block_info!(f, t.b) {
            println!("Post-ymode[{}]: r={}", y_mode, ts_c.msac.rng);
        }

        // angle delta
        let y_angle = if b_dim[2] + b_dim[3] >= 2 && y_mode >= VERT_PRED && y_mode <= VERT_LEFT_PRED
        {
            let acdf = &mut ts_c.cdf.m.angle_delta[y_mode as usize - VERT_PRED as usize];
            let angle = rav1d_msac_decode_symbol_adapt8(&mut ts_c.msac, acdf, 6);
            angle as i8 - 3
        } else {
            0
        };

        let uv_mode;
        let uv_angle;
        let cfl_alpha;
        if has_chroma {
            let cfl_allowed = if frame_hdr.segmentation.lossless[b.seg_id.get()] {
                cbw4 == 1 && cbh4 == 1
            } else {
                (cfl_allowed_mask & (1 << bs as u8)) != 0
            };
            let uvmode_cdf = &mut ts_c.cdf.m.uv_mode[cfl_allowed as usize][y_mode as usize];
            uv_mode = rav1d_msac_decode_symbol_adapt16(
                &mut ts_c.msac,
                uvmode_cdf,
                (N_UV_INTRA_PRED_MODES as u8) - 1 - (!cfl_allowed as u8),
            );
            if debug_block_info!(f, t.b) {
                println!("Post-uvmode[{}]: r={}", uv_mode, ts_c.msac.rng);
            }

            if uv_mode == CFL_PRED {
                let sign =
                    rav1d_msac_decode_symbol_adapt8(&mut ts_c.msac, &mut ts_c.cdf.m.cfl_sign.0, 7)
                        + 1;
                let sign_u = (sign as u16 * 0x56 >> 8) as u8;
                let sign_v = sign - sign_u * 3;
                assert!(sign_u == sign / 3);
                let sign_uv = [sign_u, sign_v];
                cfl_alpha = array::from_fn(|i| {
                    if sign_uv[i] == 0 {
                        return 0;
                    }
                    let ctx = (sign_uv[i] == 2) as usize * 3 + sign_uv[1 - i] as usize;
                    let cfl_alpha = rav1d_msac_decode_symbol_adapt16(
                        &mut ts_c.msac,
                        &mut ts_c.cdf.m.cfl_alpha[ctx],
                        15,
                    ) as i8
                        + 1;
                    if sign_uv[i] == 1 {
                        -cfl_alpha
                    } else {
                        cfl_alpha
                    }
                });
                if debug_block_info!(f, t.b) {
                    println!(
                        "Post-uvalphas[{}/{}]: r={}",
                        cfl_alpha[0], cfl_alpha[1], ts_c.msac.rng,
                    );
                }
                uv_angle = 0;
            } else if b_dim[2] + b_dim[3] >= 2 && uv_mode >= VERT_PRED && uv_mode <= VERT_LEFT_PRED
            {
                let acdf = &mut ts_c.cdf.m.angle_delta[uv_mode as usize - VERT_PRED as usize];
                let angle = rav1d_msac_decode_symbol_adapt8(&mut ts_c.msac, acdf, 6);
                uv_angle = angle as i8 - 3;
                cfl_alpha = Default::default();
            } else {
                uv_angle = 0;
                cfl_alpha = Default::default();
            }
        } else {
            uv_mode = Default::default();
            uv_angle = Default::default();
            cfl_alpha = Default::default();
        }

        let mut pal_sz = [0; 2];
        if frame_hdr.allow_screen_content_tools && cmp::max(bw4, bh4) <= 16 && bw4 + bh4 >= 4 {
            let sz_ctx = b_dim[2] + b_dim[3] - 2;
            if y_mode == DC_PRED {
                let pal_ctx = (*ta.pal_sz.index(bx4 as usize) > 0) as usize
                    + (*t.l.pal_sz.index(by4 as usize) > 0) as usize;
                let use_y_pal = rav1d_msac_decode_bool_adapt(
                    &mut ts_c.msac,
                    &mut ts_c.cdf.m.pal_y[sz_ctx as usize][pal_ctx],
                );
                if debug_block_info!(f, t.b) {
                    println!("Post-y_pal[{}]: r={}", use_y_pal, ts_c.msac.rng);
                }
                if use_y_pal {
                    pal_sz[0] = (bd_fn.read_pal_plane)(
                        t,
                        f,
                        ts_c,
                        false,
                        sz_ctx,
                        bx4 as usize,
                        by4 as usize,
                    );
                }
            }

            if has_chroma && uv_mode == DC_PRED {
                let pal_ctx = pal_sz[0] > 0;
                let use_uv_pal = rav1d_msac_decode_bool_adapt(
                    &mut ts_c.msac,
                    &mut ts_c.cdf.m.pal_uv[pal_ctx as usize],
                );
                if debug_block_info!(f, t.b) {
                    println!("Post-uv_pal[{}]: r={}", use_uv_pal, ts_c.msac.rng);
                }
                if use_uv_pal {
                    // see aomedia bug 2183 for why we use luma coordinates
                    pal_sz[1] = (bd_fn.read_pal_uv)(t, f, ts_c, sz_ctx, bx4 as usize, by4 as usize);
                }
            }
        }
        let pal_sz = pal_sz;

        let mut y_mode = y_mode;
        let mut y_angle = y_angle;
        let seq_hdr = f.seq_hdr();
        if y_mode == DC_PRED
            && pal_sz[0] == 0
            && cmp::max(b_dim[2], b_dim[3]) <= 3
            && seq_hdr.filter_intra != 0
        {
            let is_filter = rav1d_msac_decode_bool_adapt(
                &mut ts_c.msac,
                &mut ts_c.cdf.m.use_filter_intra[bs as usize],
            );
            if is_filter {
                y_mode = FILTER_PRED as u8;
                y_angle = rav1d_msac_decode_symbol_adapt8(
                    &mut ts_c.msac,
                    &mut ts_c.cdf.m.filter_intra.0,
                    4,
                ) as i8;
            }
            if debug_block_info!(f, t.b) {
                println!(
                    "Post-filterintramode[{}/{}]: r={}",
                    y_mode, y_angle, ts_c.msac.rng,
                );
            }
        }
        let y_mode = y_mode;
        let y_angle = y_angle;

        if pal_sz[0] != 0 {
            let scratch = t.scratch.inter_intra_mut();
            let pal_idx = if t.frame_thread.pass != 0 {
                let p = t.frame_thread.pass & 1;
                let frame_thread = &ts.frame_thread[p as usize];
                let len = (bw4 * bh4 * 8) as u32;
                let pal_idx = frame_thread.pal_idx.get_update(|i| i + len);
                &mut *f
                    .frame_thread
                    .pal_idx
                    .index_mut((pal_idx as usize.., ..len as usize))
            } else {
                &mut scratch.pal_idx_y
            };
            read_pal_indices(
                ts_c,
                &c.dsp.pal,
                scratch.levels_pal.pal_mut(),
                &mut scratch.pal_idx_uv,
                Some(pal_idx),
                pal_sz[0],
                false,
                w4,
                h4,
                bw4,
                bh4,
            );
            if debug_block_info!(f, t.b) {
                println!("Post-y-pal-indices: r={}", ts_c.msac.rng);
            }
        }

        if has_chroma && pal_sz[1] != 0 {
            let scratch = t.scratch.inter_intra_mut();
            let mut pal_idx = if t.frame_thread.pass != 0 {
                let p = t.frame_thread.pass & 1;
                let frame_thread = &ts.frame_thread[p as usize];
                let len = (cbw4 * cbh4 * 8) as u32;
                let pal_idx = frame_thread.pal_idx.get_update(|i| i + len);
                Some(
                    f.frame_thread
                        .pal_idx
                        .index_mut((pal_idx as usize.., ..len as usize)),
                )
            } else {
                None
            };
            let pal_idx = pal_idx.as_deref_mut();
            read_pal_indices(
                ts_c,
                &c.dsp.pal,
                scratch.levels_pal.pal_mut(),
                &mut scratch.pal_idx_uv,
                pal_idx,
                pal_sz[1],
                true,
                cw4,
                ch4,
                cbw4,
                cbh4,
            );
            if debug_block_info!(f, t.b) {
                println!("Post-uv-pal-indices: r={}", ts_c.msac.rng);
            }
        }

        let frame_hdr = f.frame_hdr();

        let tx = if frame_hdr.segmentation.lossless[b.seg_id.get()] {
            b.uvtx = TxfmSize::S4x4;
            b.uvtx
        } else {
            let mut tx = dav1d_max_txfm_size_for_bs[bs as usize][0];
            b.uvtx = dav1d_max_txfm_size_for_bs[bs as usize][f.cur.p.layout as usize];
            let mut t_dim = &dav1d_txfm_dimensions[tx as usize];
            if frame_hdr.txfm_mode == Rav1dTxfmMode::Switchable && t_dim.max > TxfmSize::S4x4 as _ {
                let tctx = get_tx_ctx(ta, &t.l, t_dim, by4, bx4);
                let tx_cdf = &mut ts_c.cdf.m.txsz[(t_dim.max - 1) as usize][tctx as usize];
                let depth =
                    rav1d_msac_decode_symbol_adapt4(&mut ts_c.msac, tx_cdf, cmp::min(t_dim.max, 2))
                        as c_int;

                for _ in 0..depth {
                    tx = t_dim.sub;
                    t_dim = &dav1d_txfm_dimensions[tx as usize];
                }
            }
            if debug_block_info!(f, t.b) {
                println!("Post-tx[{:?}]: r={}", tx, ts_c.msac.rng);
            }
            tx
        };
        let t_dim = &dav1d_txfm_dimensions[tx as usize];

        let intra = Av1BlockIntra {
            y_mode,
            uv_mode,
            tx,
            pal_sz,
            y_angle,
            uv_angle,
            cfl_alpha,
        };
        b.ii = Av1BlockIntraInter::Intra(intra.clone()); // cheap 9-byte clone

        // reconstruction
        if t.frame_thread.pass == 1 {
            (bd_fn.read_coef_blocks)(f, t, ts_c, bs, b);
        } else {
            (bd_fn.recon_b_intra)(f, t, Some(ts_c), bs, intra_edge_flags, b, &intra);
        }

        if f.frame_hdr().loopfilter.level_y != [0, 0] {
            let lflvl = match ts.lflvl.get() {
                TileStateRef::Frame => &f.lf.lvl,
                TileStateRef::Local => &*ts.lflvlmem.try_read().unwrap(),
            };
            let mut a_uv_guard;
            let mut l_uv_guard;
            rav1d_create_lf_mask_intra(
                &f.lf.mask[t.lf_mask.unwrap()],
                &f.lf.level,
                f.b4_stride,
                &lflvl[b.seg_id.get()],
                t.b,
                f.w4,
                f.h4,
                bs,
                tx,
                b.uvtx,
                f.cur.p.layout,
                &mut ta.tx_lpf_y.index_mut((bx4 as usize.., ..bw4 as usize)),
                &mut t.l.tx_lpf_y.index_mut((by4 as usize.., ..bh4 as usize)),
                if has_chroma {
                    a_uv_guard = ta.tx_lpf_uv.index_mut((cbx4 as usize.., ..cbw4 as usize));
                    l_uv_guard = t.l.tx_lpf_uv.index_mut((cby4 as usize.., ..cbh4 as usize));
                    Some((&mut a_uv_guard, &mut l_uv_guard))
                } else {
                    None
                },
            );
        }

        // update contexts
        let y_mode_nofilt = if y_mode == FILTER_PRED {
            DC_PRED
        } else {
            y_mode
        };
        let is_inter_or_switch = f.frame_hdr().frame_type.is_inter_or_switch();
        CaseSet::<32, false>::many(
            [(&t.l, t_dim.lh, 1), (ta, t_dim.lw, 0)],
            [bh4 as usize, bw4 as usize],
            [by4 as usize, bx4 as usize],
            |case, (dir, lw_lh, dir_index)| {
                case.set_disjoint(&dir.tx_intra, lw_lh as i8);
                // TODO check unwrap is optimized out
                case.set_disjoint(&dir.tx, TxfmSize::from_repr(lw_lh as _).unwrap());
                case.set_disjoint(&dir.mode, y_mode_nofilt);
                case.set_disjoint(&dir.pal_sz, pal_sz[0]);
                case.set_disjoint(&dir.seg_pred, seg_pred.into());
                case.set_disjoint(&dir.skip_mode, 0);
                case.set_disjoint(&dir.intra, 1);
                case.set_disjoint(&dir.skip, b.skip);
                // see aomedia bug 2183 for why we use luma coordinates here
                case.set(
                    &mut t.pal_sz_uv[dir_index],
                    if has_chroma { pal_sz[1] } else { 0 },
                );
                if is_inter_or_switch {
                    case.set_disjoint(&dir.comp_type, None);
                    case.set_disjoint(&dir.r#ref[0], -1);
                    case.set_disjoint(&dir.r#ref[1], -1);
                    case.set_disjoint(&dir.filter[0], Rav1dFilterMode::N_SWITCHABLE_FILTERS);
                    case.set_disjoint(&dir.filter[1], Rav1dFilterMode::N_SWITCHABLE_FILTERS);
                }
            },
        );
        if pal_sz[0] != 0 {
            (bd_fn.copy_pal_block_y)(t, f, bx4 as usize, by4 as usize, bw4 as usize, bh4 as usize);
        }
        if has_chroma {
            CaseSet::<32, false>::many(
                [&t.l, ta],
                [cbh4 as usize, cbw4 as usize],
                [cby4 as usize, cbx4 as usize],
                |case, dir| {
                    case.set_disjoint(&dir.uvmode, uv_mode);
                },
            );
            if pal_sz[1] != 0 {
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
            splat_intraref(c, t, &f.rf, bs, bw4 as usize, bh4 as usize);
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

        let mut r#ref = if mvstack[0].mv.mv[0] != Mv::ZERO {
            mvstack[0].mv.mv[0]
        } else if mvstack[1].mv.mv[0] != Mv::ZERO {
            mvstack[1].mv.mv[0]
        } else if t.b.y - (16 << seq_hdr.sb128) < ts.tiling.row_start {
            Mv {
                y: 0,
                x: (-(512 << seq_hdr.sb128) - 2048) as i16,
            }
        } else {
            Mv {
                y: -(512 << seq_hdr.sb128) as i16,
                x: 0,
            }
        };

        read_mv_residual(ts_c, &mut r#ref, -1);

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
        let mut src_left = t.b.x * 4 + (r#ref.x as c_int >> 3);
        let mut src_top = t.b.y * 4 + (r#ref.y as c_int >> 3);
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

        let prev_ref = r#ref;
        let r#ref = Mv {
            x: ((src_left - t.b.x * 4) * 8) as i16,
            y: ((src_top - t.b.y * 4) * 8) as i16,
        };

        if debug_block_info!(f, t.b) {
            println!(
                "Post-dmv[{}/{},ref={}/{}|{}/{}]: r={}",
                r#ref.y,
                r#ref.x,
                prev_ref.y,
                prev_ref.x,
                mvstack[0].mv.mv[0].y,
                mvstack[0].mv.mv[0].x,
                ts_c.msac.rng,
            );
        }

        let VarTx {
            uvtx,
            max_ytx,
            tx_split0,
            tx_split1,
        } = read_vartx_tree(t, f, ts_c, b, bs, bx4, by4);

        let filter2d = if t.frame_thread.pass == 1 {
            Filter2d::Bilinear
        } else {
            Default::default()
        };

        b.uvtx = uvtx;
        let inter = Av1BlockInter {
            nd: Av1BlockInter1d {
                mv: [r#ref, Default::default()],
                ..Default::default()
            }
            .into(),
            comp_type: Default::default(),
            inter_mode: Default::default(),
            motion_mode: Default::default(),
            drl_idx: Default::default(),
            r#ref: Default::default(),
            max_ytx,
            filter2d,
            interintra_type: Default::default(),
            tx_split0,
            tx_split1,
        };
        b.ii = Av1BlockIntraInter::Inter(inter.clone()); // Cheap 24-byte clone

        // reconstruction
        if t.frame_thread.pass == 1 {
            (bd_fn.read_coef_blocks)(f, t, ts_c, bs, b);
        } else {
            (bd_fn.recon_b_inter)(f, t, Some(ts_c), bs, b, &inter)?;
        }

        splat_intrabc_mv(c, t, &f.rf, bs, r#ref, bw4 as usize, bh4 as usize);

        CaseSet::<32, false>::many(
            [(&t.l, 1), (ta, 0)],
            [bh4 as usize, bw4 as usize],
            [by4 as usize, bx4 as usize],
            |case, (dir, dir_index)| {
                case.set_disjoint(&dir.tx_intra, b_dim[2 + dir_index] as i8);
                case.set_disjoint(&dir.mode, DC_PRED);
                case.set_disjoint(&dir.pal_sz, 0);
                // see aomedia bug 2183 for why this is outside `if has_chroma {}`
                case.set(&mut t.pal_sz_uv[dir_index], 0);
                case.set_disjoint(&dir.seg_pred, seg_pred.into());
                case.set_disjoint(&dir.skip_mode, 0);
                case.set_disjoint(&dir.intra, 0);
                case.set_disjoint(&dir.skip, b.skip);
            },
        );
        if has_chroma {
            CaseSet::<32, false>::many(
                [&t.l, ta],
                [cbh4 as usize, cbw4 as usize],
                [cby4 as usize, cbx4 as usize],
                |case, dir| {
                    case.set_disjoint(&dir.uvmode, DC_PRED);
                },
            );
        }
    } else {
        // inter-specific mode/mv coding
        let mv_prec = || frame_hdr.hp as i32 - frame_hdr.force_integer_mv as i32;

        let mut has_subpel_filter;

        let is_comp = if b.skip_mode != 0 {
            true
        } else if seg
            .map(|seg| seg.r#ref == -1 && seg.globalmv == 0 && seg.skip == 0)
            .unwrap_or(true)
            && frame_hdr.switchable_comp_refs != 0
            && cmp::min(bw4, bh4) > 1
        {
            let ctx = get_comp_ctx(ta, &t.l, by4, bx4, have_top, have_left);
            let is_comp =
                rav1d_msac_decode_bool_adapt(&mut ts_c.msac, &mut ts_c.cdf.mi.comp[ctx as usize]);
            if debug_block_info!(f, t.b) {
                println!("Post-compflag[{}]: r={}", is_comp, ts_c.msac.rng);
            }
            is_comp
        } else {
            false
        };

        struct Inter {
            nd: Av1BlockInterNd,
            comp_type: Option<CompInterType>,
            inter_mode: u8,
            motion_mode: MotionMode,
            drl_idx: DrlProximity,
            r#ref: [i8; 2],
            interintra_type: Option<InterIntraType>,
        }
        let Inter {
            nd,
            comp_type,
            inter_mode,
            motion_mode,
            drl_idx,
            r#ref,
            interintra_type,
        } = if b.skip_mode != 0 {
            let r#ref = [
                frame_hdr.skip_mode.refs[0] as i8,
                frame_hdr.skip_mode.refs[1] as i8,
            ];
            let comp_type = CompInterType::Avg;
            let inter_mode = NEARESTMV_NEARESTMV;
            let drl_idx = DrlProximity::Nearest;
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
                [r#ref[0] + 1, r#ref[1] + 1].into(),
                bs,
                intra_edge_flags,
                t.b.y,
                t.b.x,
                frame_hdr,
            );

            let mut mv1d = mvstack[0].mv.mv;
            fix_mv_precision(frame_hdr, &mut mv1d[0]);
            fix_mv_precision(frame_hdr, &mut mv1d[1]);
            let mv1d = mv1d;
            if debug_block_info!(f, t.b) {
                println!(
                    "Post-skipmodeblock[mv=1:y={},x={},2:y={},x={},refs={}+{}",
                    mv1d[0].y, mv1d[0].x, mv1d[1].y, mv1d[1].x, r#ref[0], r#ref[1],
                );
            }

            Inter {
                nd: Av1BlockInter1d {
                    mv: mv1d,
                    ..Default::default()
                }
                .into(),
                comp_type: Some(comp_type),
                inter_mode,
                motion_mode: Default::default(),
                drl_idx,
                r#ref,
                interintra_type: None,
            }
        } else if is_comp {
            let dir_ctx = get_comp_dir_ctx(ta, &t.l, by4, bx4, have_top, have_left);
            let r#ref = if rav1d_msac_decode_bool_adapt(
                &mut ts_c.msac,
                &mut ts_c.cdf.mi.comp_dir[dir_ctx as usize],
            ) {
                // bidir - first reference (fw)
                let ctx1 = av1_get_fwd_ref_ctx(ta, &t.l, by4, bx4, have_top, have_left);
                let ref0 = if rav1d_msac_decode_bool_adapt(
                    &mut ts_c.msac,
                    &mut ts_c.cdf.mi.comp_fwd_ref[0][ctx1 as usize],
                ) {
                    let ctx2 = av1_get_fwd_ref_2_ctx(ta, &t.l, by4, bx4, have_top, have_left);
                    2 + rav1d_msac_decode_bool_adapt(
                        &mut ts_c.msac,
                        &mut ts_c.cdf.mi.comp_fwd_ref[2][ctx2 as usize],
                    ) as i8
                } else {
                    let ctx2 = av1_get_fwd_ref_1_ctx(ta, &t.l, by4, bx4, have_top, have_left);
                    rav1d_msac_decode_bool_adapt(
                        &mut ts_c.msac,
                        &mut ts_c.cdf.mi.comp_fwd_ref[1][ctx2 as usize],
                    ) as i8
                };

                // second reference (bw)
                let ctx3 = av1_get_bwd_ref_ctx(ta, &t.l, by4, bx4, have_top, have_left);
                let ref1 = if rav1d_msac_decode_bool_adapt(
                    &mut ts_c.msac,
                    &mut ts_c.cdf.mi.comp_bwd_ref[0][ctx3 as usize],
                ) {
                    6
                } else {
                    let ctx4 = av1_get_bwd_ref_1_ctx(ta, &t.l, by4, bx4, have_top, have_left);
                    4 + rav1d_msac_decode_bool_adapt(
                        &mut ts_c.msac,
                        &mut ts_c.cdf.mi.comp_bwd_ref[1][ctx4 as usize],
                    ) as i8
                };

                [ref0, ref1]
            } else {
                // unidir
                let uctx_p = av1_get_ref_ctx(ta, &t.l, by4, bx4, have_top, have_left);
                if rav1d_msac_decode_bool_adapt(
                    &mut ts_c.msac,
                    &mut ts_c.cdf.mi.comp_uni_ref[0][uctx_p as usize],
                ) {
                    [4, 6]
                } else {
                    let uctx_p1 = av1_get_uni_p1_ctx(ta, &t.l, by4, bx4, have_top, have_left);
                    let mut r#ref = [
                        0,
                        1 + rav1d_msac_decode_bool_adapt(
                            &mut ts_c.msac,
                            &mut ts_c.cdf.mi.comp_uni_ref[1][uctx_p1 as usize],
                        ) as i8,
                    ];

                    if r#ref[1] == 2 {
                        let uctx_p2 =
                            av1_get_fwd_ref_2_ctx(ta, &t.l, by4, bx4, have_top, have_left);
                        r#ref[1] += rav1d_msac_decode_bool_adapt(
                            &mut ts_c.msac,
                            &mut ts_c.cdf.mi.comp_uni_ref[2][uctx_p2 as usize],
                        ) as i8;
                    }

                    r#ref
                }
            };
            if debug_block_info!(f, t.b) {
                println!("Post-refs[{}/{}]: r={}", r#ref[0], r#ref[1], ts_c.msac.rng,);
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
                [r#ref[0] + 1, r#ref[1] + 1].into(),
                bs,
                intra_edge_flags,
                t.b.y,
                t.b.x,
                frame_hdr,
            );

            let inter_mode = rav1d_msac_decode_symbol_adapt8(
                &mut ts_c.msac,
                &mut ts_c.cdf.mi.comp_inter_mode[ctx as usize],
                N_COMP_INTER_PRED_MODES as u8 - 1,
            );
            if debug_block_info!(f, t.b) {
                println!(
                    "Post-compintermode[{},ctx={},n_mvs={}]: r={}",
                    inter_mode, ctx, n_mvs, ts_c.msac.rng,
                );
            }

            let im = &dav1d_comp_inter_pred_modes[inter_mode as usize];
            let mut drl_idx = DrlProximity::Nearest;
            if inter_mode == NEWMV_NEWMV {
                if n_mvs > 1 {
                    // `Nearer` or `Near`
                    let drl_ctx_v1 = get_drl_context(&mvstack, 0);
                    if rav1d_msac_decode_bool_adapt(
                        &mut ts_c.msac,
                        &mut ts_c.cdf.mi.drl_bit[drl_ctx_v1 as usize],
                    ) {
                        drl_idx = DrlProximity::Nearer;

                        if n_mvs > 2 {
                            let drl_ctx_v2 = get_drl_context(&mvstack, 1);
                            if rav1d_msac_decode_bool_adapt(
                                &mut ts_c.msac,
                                &mut ts_c.cdf.mi.drl_bit[drl_ctx_v2 as usize],
                            ) {
                                drl_idx = DrlProximity::Near;
                            }
                        }
                    }
                    if debug_block_info!(f, t.b) {
                        println!(
                            "Post-drlidx[{:?},n_mvs={}]: r={}",
                            drl_idx, n_mvs, ts_c.msac.rng,
                        );
                    }
                }
            } else if im[0] == NEARMV || im[1] == NEARMV {
                drl_idx = DrlProximity::Nearer;
                if n_mvs > 2 {
                    // `Near` or `Nearish`
                    let drl_ctx_v2 = get_drl_context(&mvstack, 1);
                    if rav1d_msac_decode_bool_adapt(
                        &mut ts_c.msac,
                        &mut ts_c.cdf.mi.drl_bit[drl_ctx_v2 as usize],
                    ) {
                        drl_idx = DrlProximity::Near;

                        if n_mvs > 3 {
                            let drl_ctx_v3 = get_drl_context(&mvstack, 2);
                            if rav1d_msac_decode_bool_adapt(
                                &mut ts_c.msac,
                                &mut ts_c.cdf.mi.drl_bit[drl_ctx_v3 as usize],
                            ) {
                                drl_idx = DrlProximity::Nearish;
                            }
                        }
                    }
                    if debug_block_info!(f, t.b) {
                        println!(
                            "Post-drlidx[{:?},n_mvs={}]: r={}",
                            drl_idx, n_mvs, ts_c.msac.rng,
                        );
                    }
                }
            }
            let drl_idx = drl_idx;

            has_subpel_filter = cmp::min(bw4, bh4) == 1 || inter_mode != GLOBALMV_GLOBALMV;
            let mv1d = array::from_fn(|i| match im[i] {
                NEARMV | NEARESTMV => {
                    let mut mv1d = mvstack[drl_idx as usize].mv.mv[i];
                    fix_mv_precision(frame_hdr, &mut mv1d);
                    mv1d
                }
                GLOBALMV => {
                    has_subpel_filter |= frame_hdr.gmv[r#ref[i] as usize].r#type
                        == Rav1dWarpedMotionType::Translation;
                    get_gmv_2d(
                        &frame_hdr.gmv[r#ref[i] as usize],
                        t.b.x,
                        t.b.y,
                        bw4,
                        bh4,
                        frame_hdr,
                    )
                }
                NEWMV => {
                    let mut mv1d = mvstack[drl_idx as usize].mv.mv[i];
                    read_mv_residual(ts_c, &mut mv1d, mv_prec());
                    mv1d
                }
                _ => unreachable!(),
            });
            if debug_block_info!(f, t.b) {
                println!(
                    "Post-residual_mv[1:y={},x={},2:y={},x={}]: r={}",
                    mv1d[0].y, mv1d[0].x, mv1d[1].y, mv1d[1].x, ts_c.msac.rng,
                );
            }

            // jnt_comp vs. seg vs. wedge
            let is_segwedge;
            if seq_hdr.masked_compound != 0 {
                let mask_ctx = get_mask_comp_ctx(ta, &t.l, by4, bx4);
                is_segwedge = rav1d_msac_decode_bool_adapt(
                    &mut ts_c.msac,
                    &mut ts_c.cdf.mi.mask_comp[mask_ctx as usize],
                );
                if debug_block_info!(f, t.b) {
                    println!(
                        "Post-segwedge_vs_jntavg[{},ctx={}]: r={}",
                        is_segwedge, mask_ctx, ts_c.msac.rng,
                    );
                }
            } else {
                is_segwedge = false;
            }

            let comp_type;
            let mask_sign;
            let mut wedge_idx = Default::default();
            if !is_segwedge {
                if seq_hdr.jnt_comp != 0 {
                    let [ref0poc, ref1poc] = r#ref.map(|r#ref| {
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
                        ta,
                        &t.l,
                        by4,
                        bx4,
                    );
                    comp_type = if rav1d_msac_decode_bool_adapt(
                        &mut ts_c.msac,
                        &mut ts_c.cdf.mi.jnt_comp[jnt_ctx as usize],
                    ) {
                        CompInterType::Avg
                    } else {
                        CompInterType::WeightedAvg
                    };
                    if debug_block_info!(f, t.b) {
                        let a = ta;
                        let l = &t.l;
                        println!(
                            "Post-jnt_comp[{},ctx={}[ac:{:?},ar:{},lc:{:?},lr:{}]]: r={}",
                            comp_type == CompInterType::Avg,
                            jnt_ctx,
                            *a.comp_type.index(bx4 as usize),
                            *a.r#ref[0].index(bx4 as usize),
                            *l.comp_type.index(by4 as usize),
                            *l.r#ref[0].index(by4 as usize),
                            ts_c.msac.rng,
                        );
                    }
                } else {
                    comp_type = CompInterType::Avg;
                }
                mask_sign = Default::default();
            } else {
                comp_type = if wedge_allowed_mask & (1 << bs as u8) != 0 {
                    let ctx = dav1d_wedge_ctx_lut[bs as usize] as usize;
                    let comp_type = if rav1d_msac_decode_bool_adapt(
                        &mut ts_c.msac,
                        &mut ts_c.cdf.mi.wedge_comp[ctx],
                    ) {
                        CompInterType::Seg
                    } else {
                        CompInterType::Wedge
                    };
                    if comp_type == CompInterType::Wedge {
                        wedge_idx = rav1d_msac_decode_symbol_adapt16(
                            &mut ts_c.msac,
                            &mut ts_c.cdf.mi.wedge_idx[ctx],
                            15,
                        ) as u8;
                    }
                    comp_type
                } else {
                    CompInterType::Seg
                };

                mask_sign = rav1d_msac_decode_bool_equi(&mut ts_c.msac);
                if debug_block_info!(f, t.b) {
                    println!(
                        "Post-seg/wedge[{},wedge_idx={},sign={}]: r={}",
                        comp_type == CompInterType::Wedge,
                        wedge_idx,
                        mask_sign,
                        ts_c.msac.rng,
                    );
                }
            }
            let wedge_idx = wedge_idx;

            Inter {
                nd: Av1BlockInter1d {
                    mv: mv1d,
                    wedge_idx,
                    mask_sign: mask_sign as u8,
                    ..Default::default()
                }
                .into(),
                comp_type: Some(comp_type),
                inter_mode,
                motion_mode: Default::default(),
                drl_idx,
                r#ref,
                interintra_type: None,
            }
        } else {
            // ref
            let ref0 = if let Some(seg) = seg.filter(|seg| seg.r#ref > 0) {
                seg.r#ref as i8 - 1
            } else if let Some(_) = seg.filter(|seg| seg.globalmv != 0 || seg.skip != 0) {
                0
            } else {
                let ctx1 = av1_get_ref_ctx(ta, &t.l, by4, bx4, have_top, have_left);
                let ref0 = if rav1d_msac_decode_bool_adapt(
                    &mut ts_c.msac,
                    &mut ts_c.cdf.mi.r#ref[0][ctx1 as usize],
                ) {
                    let ctx2 = av1_get_bwd_ref_ctx(ta, &t.l, by4, bx4, have_top, have_left);
                    if rav1d_msac_decode_bool_adapt(
                        &mut ts_c.msac,
                        &mut ts_c.cdf.mi.r#ref[1][ctx2 as usize],
                    ) {
                        6
                    } else {
                        let ctx3 = av1_get_bwd_ref_1_ctx(ta, &t.l, by4, bx4, have_top, have_left);
                        4 + rav1d_msac_decode_bool_adapt(
                            &mut ts_c.msac,
                            &mut ts_c.cdf.mi.r#ref[5][ctx3 as usize],
                        ) as i8
                    }
                } else {
                    let ctx2 = av1_get_fwd_ref_ctx(ta, &t.l, by4, bx4, have_top, have_left);
                    if rav1d_msac_decode_bool_adapt(
                        &mut ts_c.msac,
                        &mut ts_c.cdf.mi.r#ref[2][ctx2 as usize],
                    ) {
                        let ctx3 = av1_get_fwd_ref_2_ctx(ta, &t.l, by4, bx4, have_top, have_left);
                        2 + rav1d_msac_decode_bool_adapt(
                            &mut ts_c.msac,
                            &mut ts_c.cdf.mi.r#ref[4][ctx3 as usize],
                        ) as i8
                    } else {
                        let ctx3 = av1_get_fwd_ref_1_ctx(ta, &t.l, by4, bx4, have_top, have_left);
                        rav1d_msac_decode_bool_adapt(
                            &mut ts_c.msac,
                            &mut ts_c.cdf.mi.r#ref[3][ctx3 as usize],
                        ) as i8
                    }
                };
                if debug_block_info!(f, t.b) {
                    println!("Post-ref[{}]: r={}", ref0, ts_c.msac.rng);
                }
                ref0
            };
            let r#ref = [ref0, -1];

            let mut mvstack = [Default::default(); 8];
            let mut n_mvs = 0;
            let mut ctx = 0;
            rav1d_refmvs_find(
                &t.rt,
                &f.rf,
                &mut mvstack,
                &mut n_mvs,
                &mut ctx,
                RefMvsRefPair {
                    r#ref: [r#ref[0] + 1, -1],
                },
                bs,
                intra_edge_flags,
                t.b.y,
                t.b.x,
                frame_hdr,
            );

            // mode parsing and mv derivation from ref_mvs
            let inter_mode;
            let mut mv1d0;
            let mut drl_idx;
            if seg
                .map(|seg| seg.skip != 0 || seg.globalmv != 0)
                .unwrap_or(false)
                || rav1d_msac_decode_bool_adapt(
                    &mut ts_c.msac,
                    &mut ts_c.cdf.mi.newmv_mode[(ctx & 7) as usize],
                )
            {
                if seg
                    .map(|seg| seg.skip != 0 || seg.globalmv != 0)
                    .unwrap_or(false)
                    || !rav1d_msac_decode_bool_adapt(
                        &mut ts_c.msac,
                        &mut ts_c.cdf.mi.globalmv_mode[(ctx >> 3 & 1) as usize],
                    )
                {
                    inter_mode = GLOBALMV;
                    mv1d0 = get_gmv_2d(
                        &frame_hdr.gmv[r#ref[0] as usize],
                        t.b.x,
                        t.b.y,
                        bw4,
                        bh4,
                        frame_hdr,
                    );
                    has_subpel_filter = cmp::min(bw4, bh4) == 1
                        || frame_hdr.gmv[r#ref[0] as usize].r#type
                            == Rav1dWarpedMotionType::Translation;

                    drl_idx = Default::default();
                } else {
                    has_subpel_filter = true;
                    if rav1d_msac_decode_bool_adapt(
                        &mut ts_c.msac,
                        &mut ts_c.cdf.mi.refmv_mode[(ctx >> 4 & 15) as usize],
                    ) {
                        // `Nearer`, `Near` or `Nearish`
                        inter_mode = NEARMV;
                        drl_idx = DrlProximity::Nearer;
                        if n_mvs > 2 {
                            // `Nearer`, `Near` or `Nearish`
                            let drl_ctx_v2 = get_drl_context(&mvstack, 1);
                            if rav1d_msac_decode_bool_adapt(
                                &mut ts_c.msac,
                                &mut ts_c.cdf.mi.drl_bit[drl_ctx_v2 as usize],
                            ) {
                                drl_idx = DrlProximity::Near;

                                if n_mvs > 3 {
                                    // `Near` or `Nearish`
                                    let drl_ctx_v3 = get_drl_context(&mvstack, 2);
                                    if rav1d_msac_decode_bool_adapt(
                                        &mut ts_c.msac,
                                        &mut ts_c.cdf.mi.drl_bit[drl_ctx_v3 as usize],
                                    ) {
                                        drl_idx = DrlProximity::Nearish;
                                    }
                                }
                            }
                        }
                    } else {
                        inter_mode = NEARESTMV;
                        drl_idx = DrlProximity::Nearest;
                    }
                    mv1d0 = mvstack[drl_idx as usize].mv.mv[0];
                    if drl_idx < DrlProximity::Near {
                        fix_mv_precision(frame_hdr, &mut mv1d0);
                    }
                }

                if debug_block_info!(f, t.b) {
                    println!(
                        "Post-intermode[{},drl={:?},mv=y:{},x:{},n_mvs={}]: r={}",
                        inter_mode, drl_idx, mv1d0.y, mv1d0.x, n_mvs, ts_c.msac.rng,
                    );
                }
            } else {
                has_subpel_filter = true;
                inter_mode = NEWMV;
                drl_idx = DrlProximity::Nearest;
                if n_mvs > 1 {
                    // `Nearer`, `Near` or `Nearish`
                    let drl_ctx_v1 = get_drl_context(&mvstack, 0);
                    if rav1d_msac_decode_bool_adapt(
                        &mut ts_c.msac,
                        &mut ts_c.cdf.mi.drl_bit[drl_ctx_v1 as usize],
                    ) {
                        drl_idx = DrlProximity::Nearer;

                        if n_mvs > 2 {
                            // `Near` or `Nearish`
                            let drl_ctx_v2 = get_drl_context(&mvstack, 1);
                            if rav1d_msac_decode_bool_adapt(
                                &mut ts_c.msac,
                                &mut ts_c.cdf.mi.drl_bit[drl_ctx_v2 as usize],
                            ) {
                                drl_idx = DrlProximity::Near;
                            }
                        }
                    }
                }
                if n_mvs > 1 {
                    mv1d0 = mvstack[drl_idx as usize].mv.mv[0];
                } else {
                    assert_eq!(drl_idx, DrlProximity::Nearest);
                    mv1d0 = mvstack[0].mv.mv[0];
                    fix_mv_precision(frame_hdr, &mut mv1d0);
                }
                if debug_block_info!(f, t.b) {
                    println!(
                        "Post-intermode[{},drl={:?}]: r={}",
                        inter_mode, drl_idx, ts_c.msac.rng,
                    );
                }
                read_mv_residual(ts_c, &mut mv1d0, mv_prec());
                if debug_block_info!(f, t.b) {
                    println!(
                        "Post-residualmv[mv=y:{},x:{}]: r={}",
                        mv1d0.y, mv1d0.x, ts_c.msac.rng,
                    );
                }
            }
            let drl_idx = drl_idx;
            let mv1d0 = mv1d0;

            // interintra flags
            let interintra_mode;
            let interintra_type;
            let mut wedge_idx = Default::default();
            let ii_sz_grp = dav1d_ymode_size_context[bs as usize] as c_int;
            if seq_hdr.inter_intra != 0
                && interintra_allowed_mask & (1 << bs as u8) != 0
                && rav1d_msac_decode_bool_adapt(
                    &mut ts_c.msac,
                    &mut ts_c.cdf.mi.interintra[ii_sz_grp as usize],
                )
            {
                interintra_mode = InterIntraPredMode::from_repr(rav1d_msac_decode_symbol_adapt4(
                    &mut ts_c.msac,
                    &mut ts_c.cdf.mi.interintra_mode[ii_sz_grp as usize],
                    InterIntraPredMode::COUNT as u8 - 1,
                ) as usize)
                .expect("valid variant");
                let wedge_ctx = dav1d_wedge_ctx_lut[bs as usize] as c_int;
                let ii_type = if rav1d_msac_decode_bool_adapt(
                    &mut ts_c.msac,
                    &mut ts_c.cdf.mi.interintra_wedge[wedge_ctx as usize],
                ) {
                    InterIntraType::Wedge
                } else {
                    InterIntraType::Blend
                };
                interintra_type = Some(ii_type);
                if ii_type == InterIntraType::Wedge {
                    wedge_idx = rav1d_msac_decode_symbol_adapt16(
                        &mut ts_c.msac,
                        &mut ts_c.cdf.mi.wedge_idx[wedge_ctx as usize],
                        15,
                    ) as u8;
                }
            } else {
                interintra_mode = Default::default();
                interintra_type = None;
            }
            let wedge_idx = wedge_idx;
            if debug_block_info!(f, t.b)
                && seq_hdr.inter_intra != 0
                && interintra_allowed_mask & (1 << bs as u8) != 0
            {
                println!(
                    "Post-interintra[t={:?},m={:?},w={}]: r={}",
                    interintra_type, interintra_mode, wedge_idx, ts_c.msac.rng,
                );
            }

            // motion variation
            let motion_mode;
            let mut matrix = None;
            if frame_hdr.switchable_motion_mode != 0
                && interintra_type == None
                && cmp::min(bw4, bh4) >= 2
                // is not warped global motion
                && !(!frame_hdr.force_integer_mv
                    && inter_mode == GLOBALMV
                    && frame_hdr.gmv[r#ref[0] as usize].r#type > Rav1dWarpedMotionType::Translation)
                // has overlappable neighbours
                && (have_left && findoddzero(&t.l.intra.index(by4 as usize..(by4 + h4) as usize))
                    || have_top && findoddzero(&ta.intra.index(bx4 as usize..(bx4 + w4) as usize)))
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
                    r#ref[0],
                    &mut mask,
                );
                let allow_warp = (f.svc[r#ref[0] as usize][0].scale == 0
                    && !frame_hdr.force_integer_mv
                    && frame_hdr.warp_motion != 0
                    && mask[0] | mask[1] != 0) as c_int;

                motion_mode = MotionMode::from_repr(if allow_warp != 0 {
                    rav1d_msac_decode_symbol_adapt4(
                        &mut ts_c.msac,
                        &mut ts_c.cdf.mi.motion_mode[bs as usize],
                        2,
                    ) as usize
                } else {
                    rav1d_msac_decode_bool_adapt(&mut ts_c.msac, &mut ts_c.cdf.mi.obmc[bs as usize])
                        as usize
                })
                .expect("valid variant");
                if motion_mode == MotionMode::Warp {
                    has_subpel_filter = false;
                    t.warpmv = derive_warpmv(&f.rf.r, t, bw4, bh4, &mask, mv1d0, t.warpmv.clone());
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
                            mv1d0.y,
                            mv1d0.x,
                        );
                    }
                    if t.frame_thread.pass != 0 {
                        matrix = Some(if t.warpmv.r#type == Rav1dWarpedMotionType::Affine {
                            [
                                t.warpmv.matrix[2] - 0x10000,
                                t.warpmv.matrix[3],
                                t.warpmv.matrix[4],
                                t.warpmv.matrix[5] - 0x10000,
                            ]
                            .map(|coef| {
                                let coef = coef as i16;
                                // warped matrix coefs are at least `i14`s.
                                debug_assert!(coef < (1 << 13));
                                debug_assert!(coef >= -(1 << 13));
                                coef
                            })
                        } else {
                            [i16::MIN, 0, 0, 0]
                        });
                    }
                }

                if debug_block_info!(f, t.b) {
                    println!(
                        "Post-motionmode[{:?}]: r={} [mask: 0x{:x}/0x{:x}]",
                        motion_mode, ts_c.msac.rng, mask[0], mask[1],
                    );
                }
            } else {
                motion_mode = MotionMode::Translation;
            }
            let matrix = matrix;

            Inter {
                nd: match matrix {
                    None => Av1BlockInter1d {
                        mv: [mv1d0, Default::default()],
                        wedge_idx,
                        interintra_mode: interintra_mode.into(),
                        ..Default::default()
                    }
                    .into(),
                    Some(matrix) => Av1BlockInter2d {
                        mv2d: mv1d0,
                        matrix,
                    }
                    .into(),
                },
                comp_type: None,
                inter_mode,
                motion_mode,
                drl_idx,
                r#ref,
                interintra_type,
            }
        };

        // subpel filter
        let filter = if frame_hdr.subpel_filter_mode == Rav1dFilterMode::Switchable {
            if has_subpel_filter {
                let comp = comp_type.is_some();
                let ctx1 = get_filter_ctx(ta, &t.l, comp, false, r#ref[0], by4, bx4);
                let filter0 = Rav1dFilterMode::from_repr(rav1d_msac_decode_symbol_adapt4(
                    &mut ts_c.msac,
                    &mut ts_c.cdf.mi.filter.0[0][ctx1 as usize],
                    Rav1dFilterMode::N_SWITCHABLE_FILTERS as u8 - 1,
                ) as usize)
                .unwrap();
                if seq_hdr.dual_filter != 0 {
                    let ctx2 = get_filter_ctx(ta, &t.l, comp, true, r#ref[0], by4, bx4);
                    if debug_block_info!(f, t.b) {
                        println!(
                            "Post-subpel_filter1[{:?},ctx={}]: r={}",
                            filter0, ctx1, ts_c.msac.rng,
                        );
                    }
                    let filter1 = Rav1dFilterMode::from_repr(rav1d_msac_decode_symbol_adapt4(
                        &mut ts_c.msac,
                        &mut ts_c.cdf.mi.filter.0[1][ctx2 as usize],
                        Rav1dFilterMode::N_SWITCHABLE_FILTERS as u8 - 1,
                    ) as usize)
                    .unwrap();
                    if debug_block_info!(f, t.b) {
                        println!(
                            "Post-subpel_filter2[{:?},ctx={}]: r={}",
                            filter1, ctx2, ts_c.msac.rng,
                        );
                    }
                    [filter0, filter1]
                } else {
                    if debug_block_info!(f, t.b) {
                        println!(
                            "Post-subpel_filter[{:?},ctx={}]: r={}",
                            filter0, ctx1, ts_c.msac.rng
                        );
                    }
                    [filter0; 2]
                }
            } else {
                [Rav1dFilterMode::Regular8Tap; 2]
            }
        } else {
            [frame_hdr.subpel_filter_mode; 2]
        };
        let filter2d = dav1d_filter_2d[filter[1] as usize][filter[0] as usize];

        let VarTx {
            uvtx,
            max_ytx,
            tx_split0,
            tx_split1,
        } = read_vartx_tree(t, f, ts_c, b, bs, bx4, by4);

        b.uvtx = uvtx;
        let inter = Av1BlockInter {
            nd,
            comp_type,
            inter_mode,
            motion_mode,
            drl_idx,
            r#ref,
            max_ytx,
            filter2d,
            interintra_type,
            tx_split0,
            tx_split1,
        };
        b.ii = Av1BlockIntraInter::Inter(inter.clone());

        // reconstruction
        if t.frame_thread.pass == 1 {
            (bd_fn.read_coef_blocks)(f, t, ts_c, bs, b);
        } else {
            (bd_fn.recon_b_inter)(f, t, Some(ts_c), bs, b, &inter)?;
        }

        let frame_hdr = f.frame_hdr();
        if frame_hdr.loopfilter.level_y != [0, 0] {
            let is_globalmv =
                (inter_mode == if is_comp { GLOBALMV_GLOBALMV } else { GLOBALMV }) as c_int;
            let tx_split = [tx_split0 as u16, tx_split1];
            let mut ytx = max_ytx;
            let mut uvtx = b.uvtx;
            if frame_hdr.segmentation.lossless[b.seg_id.get()] {
                ytx = TxfmSize::S4x4;
                uvtx = TxfmSize::S4x4;
            }
            let lflvl = match ts.lflvl.get() {
                TileStateRef::Frame => &f.lf.lvl,
                TileStateRef::Local => &*ts.lflvlmem.try_read().unwrap(),
            };
            let mut a_uv_guard;
            let mut l_uv_guard;
            rav1d_create_lf_mask_inter(
                &f.lf.mask[t.lf_mask.unwrap()],
                &f.lf.level,
                f.b4_stride,
                // In C, the inner dimensions (`ref`, `is_gmv`) are offset,
                // but then cast back to a pointer to the full array,
                // even though the whole array is not passed.
                // Dereferencing this in Rust is UB, so instead
                // we pass the indices as args, which are then applied at the use sites.
                &lflvl[b.seg_id.get()],
                (r#ref[0] + 1) as usize,
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
                &mut ta.tx_lpf_y.index_mut((bx4 as usize.., ..bw4 as usize)),
                &mut t.l.tx_lpf_y.index_mut((by4 as usize.., ..bh4 as usize)),
                if has_chroma {
                    a_uv_guard = ta.tx_lpf_uv.index_mut((cbx4 as usize.., ..cbw4 as usize));
                    l_uv_guard = t.l.tx_lpf_uv.index_mut((cby4 as usize.., ..cbh4 as usize));
                    Some((&mut *a_uv_guard, &mut *l_uv_guard))
                } else {
                    None
                },
            );
        }

        // context updates
        if is_comp {
            splat_tworef_mv(c, t, &f.rf, bs, &inter, bw4 as usize, bh4 as usize);
        } else {
            splat_oneref_mv(c, t, &f.rf, bs, &inter, bw4 as usize, bh4 as usize);
        }

        CaseSet::<32, false>::many(
            [(&t.l, 1), (ta, 0)],
            [bh4 as usize, bw4 as usize],
            [by4 as usize, bx4 as usize],
            |case, (dir, dir_index)| {
                case.set_disjoint(&dir.seg_pred, seg_pred.into());
                case.set_disjoint(&dir.skip_mode, b.skip_mode);
                case.set_disjoint(&dir.intra, 0);
                case.set_disjoint(&dir.skip, b.skip);
                case.set_disjoint(&dir.pal_sz, 0);
                // see aomedia bug 2183 for why this is outside if (has_chroma)
                case.set(&mut t.pal_sz_uv[dir_index], 0);
                case.set_disjoint(&dir.tx_intra, b_dim[2 + dir_index] as i8);
                case.set_disjoint(&dir.comp_type, comp_type);
                case.set_disjoint(&dir.filter[0], filter[0]);
                case.set_disjoint(&dir.filter[1], filter[1]);
                case.set_disjoint(&dir.mode, inter_mode);
                case.set_disjoint(&dir.r#ref[0], r#ref[0]);
                case.set_disjoint(&dir.r#ref[1], r#ref[1]);
            },
        );

        if has_chroma {
            CaseSet::<32, false>::many(
                [&t.l, ta],
                [cbh4 as usize, cbw4 as usize],
                [cby4 as usize, cbx4 as usize],
                |case, dir| {
                    case.set_disjoint(&dir.uvmode, DC_PRED);
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
        let cur_segmap = &f.cur_segmap.as_ref().unwrap().inner;
        let offset = by * b4_stride + bx;
        CaseSet::<32, false>::one((), bw4, 0, |case, ()| {
            for i in 0..bh4 {
                let i = offset + i * b4_stride;
                case.set(&mut cur_segmap.index_mut((i.., ..bw4)), b.seg_id);
            }
        });
    }
    if b.skip == 0 {
        let mask = !0u32 >> 32 - bw4 << (bx4 & 15);
        let bx_idx = (bx4 & 16) >> 4;
        for noskip_mask in &f.lf.mask[t.lf_mask.unwrap()].noskip_mask[by4 as usize >> 1..]
            [..(bh4 as usize + 1) / 2]
        {
            noskip_mask[bx_idx as usize].update(|it| it | mask as u16);
            if bw4 == 32 {
                // this should be mask >> 16, but it's 0xffffffff anyway
                noskip_mask[1].update(|it| it | mask as u16);
            }
        }
    }

    match &b.ii {
        Av1BlockIntraInter::Inter(inter)
            if t.frame_thread.pass == 1 && frame_hdr.frame_type.is_inter_or_switch() =>
        {
            let sby = t.b.y - ts.tiling.row_start >> f.sb_shift;
            let mut lowest_px = f.lowest_pixel_mem.index_mut(ts.lowest_pixel + sby as usize);
            // keep track of motion vectors for each reference
            if inter.comp_type.is_none() {
                // y
                if cmp::min(bw4, bh4) > 1
                    && (inter.inter_mode == GLOBALMV
                        && f.gmv_warp_allowed[inter.r#ref[0] as usize] != 0
                        || inter.motion_mode == MotionMode::Warp
                            && t.warpmv.r#type > Rav1dWarpedMotionType::Translation)
                {
                    affine_lowest_px_luma(
                        t,
                        &mut lowest_px[inter.r#ref[0] as usize][0],
                        b_dim,
                        if inter.motion_mode == MotionMode::Warp {
                            &t.warpmv
                        } else {
                            &frame_hdr.gmv[inter.r#ref[0] as usize]
                        },
                    );
                } else {
                    mc_lowest_px(
                        &mut lowest_px[inter.r#ref[0] as usize][0],
                        t.b.y,
                        bh4,
                        inter.nd.one_d.mv[0].y,
                        0,
                        &f.svc[inter.r#ref[0] as usize][1],
                    );
                    if inter.motion_mode == MotionMode::Obmc {
                        obmc_lowest_px(
                            &f.rf.r,
                            t,
                            &f.ts[t.ts],
                            f.cur.p.layout,
                            &f.svc,
                            &mut lowest_px,
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
                        let r = <[_; 2]>::try_from(&t.rt.r[(t.b.y as usize & 31) + 5 - 1..][..2])
                            .unwrap();

                        if bw4 == 1 {
                            is_sub8x8 &= f.rf.r.index(r[1] + t.b.x as usize - 1).r#ref.r#ref[0] > 0;
                        }
                        if bh4 == ss_ver {
                            is_sub8x8 &= f.rf.r.index(r[0] + t.b.x as usize).r#ref.r#ref[0] > 0;
                        }
                        if bw4 == 1 && bh4 == ss_ver {
                            is_sub8x8 &= f.rf.r.index(r[0] + t.b.x as usize - 1).r#ref.r#ref[0] > 0;
                        }

                        r
                    } else {
                        Default::default() // Never actually used.
                    };

                    // chroma prediction
                    if is_sub8x8 {
                        if bw4 == 1 && bh4 == ss_ver {
                            let rr = *f.rf.r.index(r[0] + t.b.x as usize - 1);
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
                            let rr = *f.rf.r.index(r[1] + t.b.x as usize - 1);
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
                            let rr = *f.rf.r.index(r[0] + t.b.x as usize);
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
                            &mut lowest_px[inter.r#ref[0] as usize][1],
                            t.b.y,
                            bh4,
                            inter.nd.one_d.mv[0].y,
                            ss_ver,
                            &f.svc[inter.r#ref[0] as usize][1],
                        );
                    } else if cmp::min(cbw4, cbh4) > 1
                        && (inter.inter_mode == GLOBALMV
                            && f.gmv_warp_allowed[inter.r#ref[0] as usize] != 0
                            || inter.motion_mode == MotionMode::Warp
                                && t.warpmv.r#type > Rav1dWarpedMotionType::Translation)
                    {
                        affine_lowest_px_chroma(
                            t,
                            f.cur.p.layout,
                            &mut lowest_px[inter.r#ref[0] as usize][1],
                            b_dim,
                            if inter.motion_mode == MotionMode::Warp {
                                &t.warpmv
                            } else {
                                &frame_hdr.gmv[inter.r#ref[0] as usize]
                            },
                        );
                    } else {
                        mc_lowest_px(
                            &mut lowest_px[inter.r#ref[0] as usize][1],
                            t.b.y & !ss_ver,
                            bh4 << (bh4 == ss_ver) as c_int,
                            inter.nd.one_d.mv[0].y,
                            ss_ver,
                            &f.svc[inter.r#ref[0] as usize][1],
                        );
                        if inter.motion_mode == MotionMode::Obmc {
                            obmc_lowest_px(
                                &f.rf.r,
                                t,
                                &f.ts[t.ts],
                                f.cur.p.layout,
                                &f.svc,
                                &mut lowest_px,
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
                let refmvs = || {
                    std::iter::zip(inter.r#ref, inter.nd.one_d.mv)
                        .map(|(r#ref, mv)| (r#ref as usize, mv))
                };
                for (r#ref, mv) in refmvs() {
                    if inter.inter_mode == GLOBALMV_GLOBALMV && f.gmv_warp_allowed[r#ref] != 0 {
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
                    if inter.inter_mode == GLOBALMV_GLOBALMV && f.gmv_warp_allowed[r#ref] != 0 {
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
                        if inter.inter_mode == GLOBALMV_GLOBALMV
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
        _ => {}
    }

    Ok(())
}

enum FrameThreadPassState<'a> {
    First(&'a mut Rav1dTileStateContext),
    Second,
}

fn decode_sb(
    c: &Rav1dContext,
    t: &mut Rav1dTaskContext,
    f: &Rav1dFrameData,
    pass: &mut FrameThreadPassState,
    bl: BlockLevel,
    edge_index: EdgeIndex,
) -> Result<(), ()> {
    let ts = &f.ts[t.ts];
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
            pass,
            next_bl,
            intra_edge.branch(sb128, edge_index).split[0],
        );
    }

    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();

    let bp;
    let mut bx8 = 0;
    let mut by8 = 0;
    let ctx = match pass {
        FrameThreadPassState::First(ts_c) => {
            if false && bl == BlockLevel::Bl64x64 {
                println!(
                    "poc={},y={},x={},bl={:?},r={}",
                    frame_hdr.frame_offset, t.b.y, t.b.x, bl, ts_c.msac.rng,
                );
            }
            bx8 = (t.b.x & 31) >> 1;
            by8 = (t.b.y & 31) >> 1;
            Some((
                get_partition_ctx(&f.a[t.a], &t.l, bl, by8, bx8),
                &mut **ts_c,
            ))
        }
        FrameThreadPassState::Second => None,
    };

    if have_h_split && have_v_split {
        if let Some((ctx, ts_c)) = ctx {
            let pc = &mut ts_c.cdf.m.partition[bl as usize][ctx as usize];
            bp = BlockPartition::from_repr(rav1d_msac_decode_symbol_adapt16(
                &mut ts_c.msac,
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
                    frame_hdr.frame_offset, t.b.y, t.b.x, bl, ctx, bp, ts_c.msac.rng,
                );
            }
        } else {
            let b = f
                .frame_thread
                .b
                .index((t.b.y as isize * f.b4_stride + t.b.x as isize) as usize);
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
                decode_b(c, t, f, pass, bl, b[0], bp, node.o)?;
            }
            BlockPartition::H => {
                let node = intra_edge.node(sb128, edge_index);
                decode_b(c, t, f, pass, bl, b[0], bp, node.h[0])?;
                t.b.y += hsz;
                decode_b(c, t, f, pass, bl, b[0], bp, node.h[1])?;
                t.b.y -= hsz;
            }
            BlockPartition::V => {
                let node = intra_edge.node(sb128, edge_index);
                decode_b(c, t, f, pass, bl, b[0], bp, node.v[0])?;
                t.b.x += hsz;
                decode_b(c, t, f, pass, bl, b[0], bp, node.v[1])?;
                t.b.x -= hsz;
            }
            BlockPartition::Split => {
                match bl.decrease() {
                    None => {
                        let tip = intra_edge.tip(sb128, edge_index);
                        assert!(hsz == 1);
                        decode_b(
                            c,
                            t,
                            f,
                            pass,
                            bl,
                            BlockSize::Bs4x4,
                            bp,
                            EdgeFlags::ALL_TR_AND_BL,
                        )?;
                        let tl_filter = t.tl_4x4_filter;
                        t.b.x += 1;
                        decode_b(c, t, f, pass, bl, BlockSize::Bs4x4, bp, tip.split[0])?;
                        t.b.x -= 1;
                        t.b.y += 1;
                        decode_b(c, t, f, pass, bl, BlockSize::Bs4x4, bp, tip.split[1])?;
                        t.b.x += 1;
                        t.tl_4x4_filter = tl_filter;
                        decode_b(c, t, f, pass, bl, BlockSize::Bs4x4, bp, tip.split[2])?;
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
                            ts.frame_thread[p].cf.update(|cf| (cf + 31) & !31);
                        }
                    }
                    Some(next_bl) => {
                        let branch = intra_edge.branch(sb128, edge_index);
                        decode_sb(c, t, f, pass, next_bl, branch.split[0])?;
                        t.b.x += hsz;
                        decode_sb(c, t, f, pass, next_bl, branch.split[1])?;
                        t.b.x -= hsz;
                        t.b.y += hsz;
                        decode_sb(c, t, f, pass, next_bl, branch.split[2])?;
                        t.b.x += hsz;
                        decode_sb(c, t, f, pass, next_bl, branch.split[3])?;
                        t.b.x -= hsz;
                        t.b.y -= hsz;
                    }
                }
            }
            BlockPartition::TopSplit => {
                let node = intra_edge.node(sb128, edge_index);
                decode_b(c, t, f, pass, bl, b[0], bp, EdgeFlags::ALL_TR_AND_BL)?;
                t.b.x += hsz;
                decode_b(c, t, f, pass, bl, b[0], bp, node.v[1])?;
                t.b.x -= hsz;
                t.b.y += hsz;
                decode_b(c, t, f, pass, bl, b[1], bp, node.h[1])?;
                t.b.y -= hsz;
            }
            BlockPartition::BottomSplit => {
                let node = intra_edge.node(sb128, edge_index);
                decode_b(c, t, f, pass, bl, b[0], bp, node.h[0])?;
                t.b.y += hsz;
                decode_b(c, t, f, pass, bl, b[1], bp, node.v[0])?;
                t.b.x += hsz;
                decode_b(c, t, f, pass, bl, b[1], bp, EdgeFlags::empty())?;
                t.b.x -= hsz;
                t.b.y -= hsz;
            }
            BlockPartition::LeftSplit => {
                let node = intra_edge.node(sb128, edge_index);
                decode_b(c, t, f, pass, bl, b[0], bp, EdgeFlags::ALL_TR_AND_BL)?;
                t.b.y += hsz;
                decode_b(c, t, f, pass, bl, b[0], bp, node.h[1])?;
                t.b.y -= hsz;
                t.b.x += hsz;
                decode_b(c, t, f, pass, bl, b[1], bp, node.v[1])?;
                t.b.x -= hsz;
            }
            BlockPartition::RightSplit => {
                let node = intra_edge.node(sb128, edge_index);
                decode_b(c, t, f, pass, bl, b[0], bp, node.v[0])?;
                t.b.x += hsz;
                decode_b(c, t, f, pass, bl, b[1], bp, node.h[0])?;
                t.b.y += hsz;
                decode_b(c, t, f, pass, bl, b[1], bp, EdgeFlags::empty())?;
                t.b.y -= hsz;
                t.b.x -= hsz;
            }
            BlockPartition::H4 => {
                let branch = intra_edge.branch(sb128, edge_index);
                let node = &branch.node;
                decode_b(c, t, f, pass, bl, b[0], bp, node.h[0])?;
                t.b.y += hsz >> 1;
                decode_b(c, t, f, pass, bl, b[0], bp, branch.h4)?;
                t.b.y += hsz >> 1;
                decode_b(c, t, f, pass, bl, b[0], bp, EdgeFlags::ALL_LEFT_HAS_BOTTOM)?;
                t.b.y += hsz >> 1;
                if t.b.y < f.bh {
                    decode_b(c, t, f, pass, bl, b[0], bp, node.h[1])?;
                }
                t.b.y -= hsz * 3 >> 1;
            }
            BlockPartition::V4 => {
                let branch = intra_edge.branch(sb128, edge_index);
                let node = &branch.node;
                decode_b(c, t, f, pass, bl, b[0], bp, node.v[0])?;
                t.b.x += hsz >> 1;
                decode_b(c, t, f, pass, bl, b[0], bp, branch.v4)?;
                t.b.x += hsz >> 1;
                decode_b(c, t, f, pass, bl, b[0], bp, EdgeFlags::ALL_TOP_HAS_RIGHT)?;
                t.b.x += hsz >> 1;
                if t.b.x < f.bw {
                    decode_b(c, t, f, pass, bl, b[0], bp, node.v[1])?;
                }
                t.b.x -= hsz * 3 >> 1;
            }
        }
    } else if have_h_split {
        let is_split;
        if let Some((ctx, ts_c)) = ctx {
            let pc = &mut ts_c.cdf.m.partition[bl as usize][ctx as usize];
            is_split = rav1d_msac_decode_bool(&mut ts_c.msac, gather_top_partition_prob(pc, bl));
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
                    ts_c.msac.rng,
                );
            }
        } else {
            let b = &f
                .frame_thread
                .b
                .index((t.b.y as isize * f.b4_stride + t.b.x as isize) as usize);
            is_split = b.bl != bl;
        }

        let next_bl = bl
            .decrease()
            .expect("BlockLevel::BL_8X8 should never make it here");

        if is_split {
            let branch = intra_edge.branch(sb128, edge_index);
            bp = BlockPartition::Split;
            decode_sb(c, t, f, pass, next_bl, branch.split[0])?;
            t.b.x += hsz;
            decode_sb(c, t, f, pass, next_bl, branch.split[1])?;
            t.b.x -= hsz;
        } else {
            let node = intra_edge.node(sb128, edge_index);
            bp = BlockPartition::H;
            decode_b(
                c,
                t,
                f,
                pass,
                bl,
                dav1d_block_sizes[bl as usize][bp as usize][0],
                bp,
                node.h[0],
            )?;
        }
    } else {
        assert!(have_v_split);
        let is_split;
        if let Some((ctx, ts_c)) = ctx {
            let pc = &mut ts_c.cdf.m.partition[bl as usize][ctx as usize];
            is_split = rav1d_msac_decode_bool(&mut ts_c.msac, gather_left_partition_prob(pc, bl));
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
                    ts_c.msac.rng,
                );
            }
        } else {
            let b = &f
                .frame_thread
                .b
                .index((t.b.y as isize * f.b4_stride + t.b.x as isize) as usize);
            is_split = b.bl != bl;
        }

        let next_bl = bl
            .decrease()
            .expect("BlockLevel::BL_8X8 should never make it here");

        if is_split {
            let branch = intra_edge.branch(sb128, edge_index);
            bp = BlockPartition::Split;
            decode_sb(c, t, f, pass, next_bl, branch.split[0])?;
            t.b.y += hsz;
            decode_sb(c, t, f, pass, next_bl, branch.split[2])?;
            t.b.y -= hsz;
        } else {
            let node = intra_edge.node(sb128, edge_index);
            bp = BlockPartition::V;
            decode_b(
                c,
                t,
                f,
                pass,
                bl,
                dav1d_block_sizes[bl as usize][bp as usize][0],
                bp,
                node.v[0],
            )?;
        }
    }

    if matches!(pass, FrameThreadPassState::First(_))
        && (bp != BlockPartition::Split || bl == BlockLevel::Bl8x8)
    {
        CaseSet::<16, false>::many(
            [(&f.a[t.a], 0), (&t.l, 1)],
            [hsz as usize; 2],
            [bx8 as usize, by8 as usize],
            |case, (dir, dir_index)| {
                case.set_disjoint(
                    &dir.partition,
                    dav1d_al_part_ctx[dir_index][bl as usize][bp as usize],
                );
            },
        );
    }

    Ok(())
}

fn reset_context(ctx: &mut BlockContext, keyframe: bool, pass: c_int) {
    ctx.intra.get_mut().0.fill(keyframe.into());
    ctx.uvmode.get_mut().0.fill(DC_PRED);
    if keyframe {
        ctx.mode.get_mut().0.fill(DC_PRED);
    }

    if pass == 2 {
        return;
    }

    ctx.partition.get_mut().0.fill(0);
    ctx.skip.get_mut().0.fill(0);
    ctx.skip_mode.get_mut().0.fill(0);
    ctx.tx_lpf_y.get_mut().0.fill(2);
    ctx.tx_lpf_uv.get_mut().0.fill(1);
    ctx.tx_intra.get_mut().0.fill(-1);
    ctx.tx.get_mut().0.fill(TxfmSize::S64x64);
    if !keyframe {
        for r#ref in &mut ctx.r#ref {
            r#ref.get_mut().0.fill(-1);
        }
        ctx.comp_type.get_mut().0.fill(None);
        ctx.mode.get_mut().0.fill(NEARESTMV);
    }
    ctx.lcoef.get_mut().0.fill(0x40);
    for ccoef in &mut ctx.ccoef {
        ccoef.get_mut().0.fill(0x40);
    }
    for filter in &mut ctx.filter {
        filter
            .get_mut()
            .0
            .fill(Rav1dFilterMode::N_SWITCHABLE_FILTERS);
    }
    ctx.seg_pred.get_mut().0.fill(0);
    ctx.pal_sz.get_mut().0.fill(0);
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

fn setup_tile(
    c: &Rav1dContext,
    ts: &mut Rav1dTileState,
    seq_hdr: &Rav1dSequenceHeader,
    frame_hdr: &Rav1dFrameHeader,
    bitdepth_max: i32,
    sb_shift: i32,
    cur: &Rav1dPicture,
    bw: i32,
    bh: i32,
    frame_thread: &Rav1dFrameContextFrameThread,
    sr_sb128w: i32,
    sb128w: i32,
    lf: &mut Rav1dFrameContextLf,
    in_cdf: &CdfThreadContext,
    data: CArc<[u8]>,
    tile_row: usize,
    tile_col: usize,
    tile_start_off: u32,
) {
    let col_sb_start = frame_hdr.tiling.col_start_sb[tile_col] as c_int;
    let col_sb128_start = col_sb_start >> (seq_hdr.sb128 == 0) as c_int;
    let col_sb_end = frame_hdr.tiling.col_start_sb[tile_col + 1] as c_int;
    let row_sb_start = frame_hdr.tiling.row_start_sb[tile_row] as c_int;
    let row_sb_end = frame_hdr.tiling.row_start_sb[tile_row + 1] as c_int;

    let size_mul = &ss_size_mul[cur.p.layout];
    for p in 0..2 {
        ts.frame_thread[p]
            .pal_idx
            .set(if !frame_thread.pal_idx.is_empty() {
                tile_start_off * size_mul[1] as u32 / 8
            } else {
                0
            });
        ts.frame_thread[p]
            .cbi_idx
            .set(if !frame_thread.cbi.is_empty() {
                tile_start_off * size_mul[0] as u32 / 64
            } else {
                0
            });
        ts.frame_thread[p].cf.set(if !frame_thread.cf.is_empty() {
            let bpc = BPC::from_bitdepth_max(bitdepth_max);
            bpc.coef_stride(tile_start_off * size_mul[0] as u32 >> (seq_hdr.hbd == 0) as c_int)
        } else {
            0
        });
    }

    let ts_c = &mut *ts.context.try_lock().unwrap();
    ts_c.cdf = rav1d_cdf_thread_copy(in_cdf);
    ts.last_qidx = frame_hdr.quant.yac.into();
    ts.last_delta_lf = Default::default();

    ts_c.msac = MsacContext::new(data, frame_hdr.disable_cdf_update != 0, &c.dsp.msac);

    ts.tiling.row = tile_row as i32;
    ts.tiling.col = tile_col as i32;
    ts.tiling.col_start = col_sb_start << sb_shift;
    ts.tiling.col_end = cmp::min(col_sb_end << sb_shift, bw);
    ts.tiling.row_start = row_sb_start << sb_shift;
    ts.tiling.row_end = cmp::min(row_sb_end << sb_shift, bh);
    let diff_width = frame_hdr.size.width[0] != frame_hdr.size.width[1];

    // Reference Restoration Unit (used for exp coding)
    let (sb_idx, unit_idx) = if diff_width {
        // vertical components only
        (
            (ts.tiling.row_start >> 5) * sr_sb128w,
            (ts.tiling.row_start & 16) >> 3,
        )
    } else {
        (
            (ts.tiling.row_start >> 5) * sb128w + col_sb128_start,
            ((ts.tiling.row_start & 16) >> 3) + ((ts.tiling.col_start & 16) >> 4),
        )
    };
    for p in 0..3 {
        if !((lf.restore_planes.bits() >> p) & 1 != 0) {
            continue;
        }

        let lr_ref = if diff_width {
            let ss_hor = (p != 0 && cur.p.layout != Rav1dPixelLayout::I444) as u8;
            let d = frame_hdr.size.super_res.width_scale_denominator as c_int;
            let unit_size_log2 = frame_hdr.restoration.unit_size[(p != 0) as usize];
            let rnd = (8 << unit_size_log2) - 1;
            let shift = unit_size_log2 + 3;
            let x = (4 * ts.tiling.col_start * d >> ss_hor) + rnd >> shift;
            let px_x = x << unit_size_log2 + ss_hor;
            let u_idx = unit_idx + ((px_x & 64) >> 6);
            let sb128x = px_x >> 7;
            if sb128x >= sr_sb128w {
                continue;
            }
            &mut lf.lr_mask[(sb_idx + sb128x) as usize].lr[p][u_idx as usize]
        } else {
            &mut lf.lr_mask[sb_idx as usize].lr[p][unit_idx as usize]
        };

        let lr = lr_ref.get_mut();
        *lr = Av1RestorationUnit {
            filter_v: [3, -7, 15],
            filter_h: [3, -7, 15],
            sgr_weights: [-32, 31],
            ..*lr
        };
        ts.lr_ref.get_mut()[p] = *lr;
    }

    if c.tc.len() > 1 {
        ts.progress.fill_with(|| AtomicI32::new(row_sb_start));
    }
}

fn read_restoration_info(
    ts: &Rav1dTileState,
    lr: &mut Av1RestorationUnit,
    p: usize,
    frame_type: Rav1dRestorationType,
    debug_block_info: bool,
) {
    let ts_c = &mut *ts.context.try_lock().unwrap();
    let lr_ref = ts.lr_ref.try_read().unwrap()[p];

    if frame_type == Rav1dRestorationType::Switchable {
        let filter = rav1d_msac_decode_symbol_adapt4(
            &mut ts_c.msac,
            &mut ts_c.cdf.m.restore_switchable.0,
            2,
        );
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
            &mut ts_c.msac,
            if frame_type == Rav1dRestorationType::Wiener {
                &mut ts_c.cdf.m.restore_wiener.0
            } else {
                &mut ts_c.cdf.m.restore_sgrproj.0
            },
        );
        lr.r#type = if r#type {
            frame_type
        } else {
            Rav1dRestorationType::None
        };
    }

    fn msac_decode_lr_subexp(
        ts_c: &mut Rav1dTileStateContext,
        r#ref: i8,
        k: u8,
        adjustment: i8,
    ) -> i8 {
        (rav1d_msac_decode_subexp(&mut ts_c.msac, (r#ref + adjustment) as c_uint, 8 << k, k)
            - adjustment as c_int) as i8
    }

    match lr.r#type {
        Rav1dRestorationType::Wiener => {
            lr.filter_v[0] = if p != 0 {
                0
            } else {
                msac_decode_lr_subexp(ts_c, lr_ref.filter_v[0], 1, 5)
            };
            lr.filter_v[1] = msac_decode_lr_subexp(ts_c, lr_ref.filter_v[1], 2, 23);
            lr.filter_v[2] = msac_decode_lr_subexp(ts_c, lr_ref.filter_v[2], 3, 17);

            lr.filter_h[0] = if p != 0 {
                0
            } else {
                msac_decode_lr_subexp(ts_c, lr_ref.filter_h[0], 1, 5)
            };
            lr.filter_h[1] = msac_decode_lr_subexp(ts_c, lr_ref.filter_h[1], 2, 23);
            lr.filter_h[2] = msac_decode_lr_subexp(ts_c, lr_ref.filter_h[2], 3, 17);
            lr.sgr_weights = lr_ref.sgr_weights;
            ts.lr_ref.try_write().unwrap()[p] = *lr;
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
                    ts_c.msac.rng,
                );
            }
        }
        Rav1dRestorationType::SgrProj(_) => {
            let sgr_idx =
                SgrIdx::from_repr(rav1d_msac_decode_bools(&mut ts_c.msac, 4) as usize).unwrap();
            let sgr_params = &dav1d_sgr_params[sgr_idx as usize];
            lr.r#type = Rav1dRestorationType::SgrProj(sgr_idx);
            lr.sgr_weights[0] = if sgr_params[0] != 0 {
                msac_decode_lr_subexp(ts_c, lr_ref.sgr_weights[0], 4, 96)
            } else {
                0
            };
            lr.sgr_weights[1] = if sgr_params[1] != 0 {
                msac_decode_lr_subexp(ts_c, lr_ref.sgr_weights[1], 4, 32)
            } else {
                95
            };
            lr.filter_v = lr_ref.filter_v;
            lr.filter_h = lr_ref.filter_h;
            ts.lr_ref.try_write().unwrap()[p] = *lr;
            if debug_block_info {
                println!(
                    "Post-lr_sgrproj[pl={},idx={},w[{},{}]]: r={}",
                    p, sgr_idx, lr.sgr_weights[0], lr.sgr_weights[1], ts_c.msac.rng,
                );
            }
        }
        _ => {}
    }
}

// modeled after the equivalent function in aomdec:decodeframe.c
fn check_trailing_bits_after_symbol_coder(msac: &MsacContext) -> Result<(), ()> {
    // check marker bit (single 1), followed by zeroes
    let n_bits = -(msac.cnt + 14);
    assert!(n_bits <= 0); // this assumes we errored out when cnt <= -15 in caller
    let n_bytes = (n_bits + 7) >> 3;
    let trailing_bytes_offset = msac.buf_index().wrapping_add_signed(n_bytes as isize - 1);
    let trailing_bytes = &msac.data()[trailing_bytes_offset..];
    let pattern = 128 >> ((n_bits - 1) & 7);
    // use x + (x - 1) instead of 2x - 1 to avoid overflow
    if (trailing_bytes[0] & (pattern + (pattern - 1))) != pattern {
        return Err(());
    }

    // check remainder zero bytes
    if trailing_bytes[1..].iter().any(|&x| x != 0) {
        return Err(());
    }

    return Ok(());
}

pub(crate) fn rav1d_decode_tile_sbrow(
    c: &Rav1dContext,
    t: &mut Rav1dTaskContext,
    f: &Rav1dFrameData,
) -> Result<(), ()> {
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    let root_bl = if seq_hdr.sb128 != 0 {
        BlockLevel::Bl128x128
    } else {
        BlockLevel::Bl64x64
    };
    let ts = &f.ts[t.ts];
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

    if frame_hdr.frame_type.is_inter_or_switch() && c.fc.len() > 1 {
        let sby = t.b.y - ts.tiling.row_start >> f.sb_shift;
        *f.lowest_pixel_mem.index_mut(ts.lowest_pixel + sby as usize) = [[i32::MIN; 2]; 7];
    }

    reset_context(
        &mut t.l,
        frame_hdr.frame_type.is_key_or_intra(),
        t.frame_thread.pass,
    );
    if t.frame_thread.pass == 2 {
        let off_2pass = if c.tc.len() > 1 {
            f.sb128w * frame_hdr.tiling.rows as c_int
        } else {
            0
        };
        t.a = (off_2pass + col_sb128_start + tile_row * f.sb128w) as usize;
        for bx in (ts.tiling.col_start..ts.tiling.col_end).step_by(sb_step as usize) {
            t.b.x = bx;
            if c.flush.load(Ordering::Acquire) {
                return Err(());
            }
            decode_sb(
                c,
                t,
                f,
                &mut FrameThreadPassState::Second,
                root_bl,
                EdgeIndex::root(),
            )?;
            if t.b.x & 16 != 0 || f.seq_hdr().sb128 != 0 {
                t.a += 1;
            }
        }
        (f.bd_fn().backup_ipred_edge)(f, t);
        return Ok(());
    }

    if c.tc.len() > 1 && frame_hdr.use_ref_frame_mvs != 0 {
        c.dsp.refmvs.load_tmvs.call(
            &f.rf,
            &f.mvs,
            &f.ref_mvs,
            ts.tiling.row,
            ts.tiling.col_start >> 1,
            ts.tiling.col_end >> 1,
            t.b.y >> 1,
            t.b.y + sb_step >> 1,
        );
    }
    t.pal_sz_uv[1] = Default::default();
    let sb128y = t.b.y >> 5;
    t.a = (col_sb128_start + tile_row * f.sb128w) as usize;
    t.lf_mask = Some((sb128y * f.sb128w + col_sb128_start) as usize);
    for bx in (ts.tiling.col_start..ts.tiling.col_end).step_by(sb_step as usize) {
        t.b.x = bx;
        if c.flush.load(Ordering::Acquire) {
            return Err(());
        }
        let cdef_idx = &f.lf.mask[t.lf_mask.unwrap()].cdef_idx;
        if root_bl == BlockLevel::Bl128x128 {
            for cdef_idx in cdef_idx {
                cdef_idx.set(-1);
            }
            t.cur_sb_cdef_idx = 0;
        } else {
            t.cur_sb_cdef_idx = (((t.b.x & 16) >> 4) + ((t.b.y & 16) >> 3)) as usize;
            let cdef_idx = &cdef_idx[t.cur_sb_cdef_idx..];
            cdef_idx[0].set(-1);
        }
        let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
        // Restoration filter
        for p in 0..3 {
            if (f.lf.restore_planes.bits() >> p) & 1 == 0 {
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

                let d = frame_hdr.size.super_res.width_scale_denominator as c_int;
                let rnd = unit_size * 8 - 1;
                let shift = unit_size_log2 + 3;
                let x0 = (4 * t.b.x * d >> ss_hor) + rnd >> shift;
                let x1 = (4 * (t.b.x + sb_step) * d >> ss_hor) + rnd >> shift;

                for x in x0..cmp::min(x1, n_units) {
                    let px_x = x << unit_size_log2 + ss_hor as u8;
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
        decode_sb(
            c,
            t,
            f,
            &mut FrameThreadPassState::First(&mut f.ts[t.ts].context.try_lock().unwrap()),
            root_bl,
            EdgeIndex::root(),
        )?;
        if t.b.x & 16 != 0 || f.seq_hdr().sb128 != 0 {
            t.a += 1;
            t.lf_mask = t.lf_mask.map(|i| i + 1);
        }
    }

    if f.seq_hdr().ref_frame_mvs != 0
        && c.tc.len() > 1
        && f.frame_hdr().frame_type.is_inter_or_switch()
    {
        c.dsp.refmvs.save_tmvs.call(
            &t.rt,
            &f.rf,
            &f.mvs,
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
    let start_y = (align_h * tile_col + t.b.y) as usize;
    let len_y = sb_step as usize;
    let start_lpf_y = (t.b.y & 16) as usize;
    f.lf.tx_lpf_right_edge.copy_from_slice_y(
        start_y..start_y + len_y,
        &t.l.tx_lpf_y.index(start_lpf_y..start_lpf_y + len_y),
    );
    let ss_ver = (f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
    align_h >>= ss_ver;
    let start_uv = (align_h * tile_col + (t.b.y >> ss_ver)) as usize;
    let len_uv = (sb_step >> ss_ver) as usize;
    let lpf_uv_start = ((t.b.y & 16) >> ss_ver) as usize;
    f.lf.tx_lpf_right_edge.copy_from_slice_uv(
        start_uv..start_uv + len_uv,
        &t.l.tx_lpf_uv.index(lpf_uv_start..lpf_uv_start + len_uv),
    );

    // error out on symbol decoder overread
    if ts.context.try_lock().unwrap().msac.cnt <= -15 {
        return Err(());
    }

    if c.strict_std_compliance
        && (t.b.y >> f.sb_shift) + 1
            >= f.frame_hdr().tiling.row_start_sb[tile_row as usize + 1].into()
    {
        return check_trailing_bits_after_symbol_coder(&ts.context.try_lock().unwrap().msac);
    }
    Ok(())
}

pub(crate) fn rav1d_decode_frame_init(c: &Rav1dContext, fc: &Rav1dFrameContext) -> Rav1dResult {
    let mut f = fc.data.try_write().unwrap();
    let f = &mut *f;

    // TODO: Fallible allocation
    f.lf.start_of_tile_row.resize(f.sbh as usize, 0);

    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    let mut sby = 0;
    for tile_row in 0..frame_hdr.tiling.rows {
        f.lf.start_of_tile_row[sby as usize] = tile_row;
        sby += 1;
        while sby < frame_hdr.tiling.row_start_sb[(tile_row + 1) as usize] as c_int {
            f.lf.start_of_tile_row[sby as usize] = 0;
            sby += 1;
        }
    }

    let n_ts = frame_hdr.tiling.cols as c_int * frame_hdr.tiling.rows as c_int;
    if c.fc.len() > 1 {
        // TODO: Fallible allocation
        f.frame_thread.tile_start_off.resize(n_ts as usize, 0);
    }
    // TODO: Fallible allocation
    f.ts.resize_with(n_ts as usize, Default::default);

    let a_sz = f.sb128w
        * frame_hdr.tiling.rows as c_int
        * (1 + (c.fc.len() > 1 && c.tc.len() > 1) as c_int);
    // TODO: Fallible allocation
    f.a.resize_with(a_sz as usize, Default::default);

    let num_sb128 = f.sb128w * f.sb128h;
    let size_mul = &ss_size_mul[f.cur.p.layout];
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    let hbd = (seq_hdr.hbd != 0) as c_int;
    if c.fc.len() > 1 {
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

        let lowest_pixel_mem_sz = frame_hdr.tiling.cols as usize * f.sbh as usize;
        // TODO: Fallible allocation
        f.lowest_pixel_mem
            .resize(lowest_pixel_mem_sz, Default::default());

        let mut lowest_pixel_offset = 0;
        for tile_row in 0..frame_hdr.tiling.rows as usize {
            let tile_row_base = tile_row * frame_hdr.tiling.cols as usize;
            let tile_row_sb_h = frame_hdr.tiling.row_start_sb[tile_row + 1] as usize
                - frame_hdr.tiling.row_start_sb[tile_row] as usize;
            for tile_col in 0..frame_hdr.tiling.cols as usize {
                f.ts[tile_row_base + tile_col].lowest_pixel = lowest_pixel_offset;
                lowest_pixel_offset += tile_row_sb_h;
            }
        }

        let cbi_sz = num_sb128 * size_mul[0] as c_int;
        // TODO: Fallible allocation
        f.frame_thread
            .cbi
            .resize_with(cbi_sz as usize * 32 * 32 / 4, Default::default);

        let cf_sz = (num_sb128 * size_mul[0] as c_int) << hbd;
        // TODO: Fallible allocation
        f.frame_thread
            .cf
            .get_mut()
            .resize(cf_sz as usize * 128 * 128 / 2, 0);

        if frame_hdr.allow_screen_content_tools {
            // TODO: Fallible allocation
            f.frame_thread
                .pal
                .resize(num_sb128 as usize * 16 * 16 << hbd);

            let pal_idx_sz = num_sb128 * size_mul[1] as c_int;
            // TODO: Fallible allocation
            f.frame_thread
                .pal_idx
                .resize(pal_idx_sz as usize * 128 * 128 / 8, Default::default());
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
    // over-allocate by 3 bytes since some of the SIMD implementations
    // index this from the level type and can thus over-read by up to 3 bytes.
    f.lf.level
        .resize_with(4 * num_sb128 as usize * 32 * 32 + 3, Default::default); // TODO: Fallible allocation
    if c.fc.len() > 1 {
        // TODO: Fallible allocation
        f.frame_thread
            .b
            .resize_with(num_sb128 as usize * 32 * 32, Default::default);
    }

    f.sr_sb128w = f.sr_cur.p.p.w + 127 >> 7;
    let lr_mask_sz = f.sr_sb128w * f.sb128h;
    // TODO: Fallible allocation
    f.lf.lr_mask
        .resize_with(lr_mask_sz as usize, Default::default);
    f.lf.restore_planes = LrRestorePlanes::from_bits_truncate(
        frame_hdr
            .restoration
            .r#type
            .iter()
            .enumerate()
            .map(|(i, &r#type)| ((r#type != Rav1dRestorationType::None) as u8) << i)
            .sum::<u8>(),
    );
    if frame_hdr.loopfilter.sharpness != f.lf.last_sharpness {
        rav1d_calc_eih(&mut f.lf.lim_lut.0, frame_hdr.loopfilter.sharpness);
        f.lf.last_sharpness = frame_hdr.loopfilter.sharpness;
    }
    rav1d_calc_lf_values(&mut f.lf.lvl, &frame_hdr, &[0, 0, 0, 0]);

    let ipred_edge_sz = f.sbh * f.sb128w << hbd;
    // TODO: Fallible allocation
    f.ipred_edge.resize(ipred_edge_sz as usize * 128 * 3, 0);
    f.ipred_edge_off = bpc.pxstride(ipred_edge_sz as usize * 128);

    let re_sz = f.sb128h as usize * frame_hdr.tiling.cols as usize;
    // TODO: Fallible allocation
    f.lf.tx_lpf_right_edge.resize(re_sz, 0);

    // init ref mvs
    if frame_hdr.frame_type.is_inter_or_switch() || frame_hdr.allow_intrabc {
        rav1d_refmvs_init_frame(
            &mut f.rf,
            seq_hdr,
            frame_hdr,
            &f.refpoc,
            &f.refrefpoc,
            &f.ref_mvs,
            c.tc.len() as u32,
            c.fc.len() as u32,
        )?;
    }

    // setup dequant tables
    init_quant_tables(&seq_hdr, &frame_hdr, frame_hdr.quant.yac, &f.dq);
    if frame_hdr.quant.qm != 0 {
        for i in 0..TxfmSize::COUNT {
            f.qm[i][0] = dav1d_qm_tbl[frame_hdr.quant.qm_y as usize][0][i];
            f.qm[i][1] = dav1d_qm_tbl[frame_hdr.quant.qm_u as usize][1][i];
            f.qm[i][2] = dav1d_qm_tbl[frame_hdr.quant.qm_v as usize][1][i];
        }
    } else {
        f.qm = Default::default();
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
                            ref_pocs[ij] as c_int,
                            f.cur.frame_hdr.as_ref().unwrap().frame_offset as c_int,
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

    Ok(())
}

pub(crate) fn rav1d_decode_frame_init_cdf(
    c: &Rav1dContext,
    fc: &Rav1dFrameContext,
    f: &mut Rav1dFrameData,
    in_cdf: &CdfThreadContext,
) -> Rav1dResult {
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();

    if frame_hdr.refresh_context != 0 {
        *f.out_cdf.cdf_write() = rav1d_cdf_thread_copy(in_cdf);
    }

    let uses_2pass = c.fc.len() > 1;

    let tiling = &frame_hdr.tiling;

    let n_bytes = tiling.n_bytes.try_into().unwrap();
    let rows: usize = tiling.rows.try_into().unwrap();
    let cols = tiling.cols.try_into().unwrap();
    let sb128w: usize = f.sb128w.try_into().unwrap();

    // parse individual tiles per tile group
    let mut tile_row = 0;
    let mut tile_col = 0;
    fc.task_thread.update_set.set(false);
    for tile in &f.tiles {
        let start = tile.hdr.start.try_into().unwrap();
        let end: usize = tile.hdr.end.try_into().unwrap();

        let mut data = tile.data.data.clone().unwrap();
        for (j, (ts, tile_start_off)) in iter::zip(
            &mut f.ts[..end + 1],
            if uses_2pass {
                &f.frame_thread.tile_start_off[..end + 1]
            } else {
                &[]
            }
            .into_iter()
            .copied()
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
                let (cur_data, rest_data) = CArc::split_at(data, n_bytes);
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

            let (cur_data, rest_data) = CArc::split_at(data, tile_sz);
            setup_tile(
                c,
                ts,
                &***f.seq_hdr.as_ref().unwrap(),
                frame_hdr,
                f.bitdepth_max,
                f.sb_shift,
                &f.cur,
                f.bw,
                f.bh,
                &f.frame_thread,
                f.sr_sb128w,
                f.sb128w,
                &mut f.lf,
                in_cdf,
                cur_data,
                tile_row,
                tile_col,
                tile_start_off,
            );
            tile_col += 1;

            if tile_col == cols {
                tile_col = 0;
                tile_row += 1;
            }
            if j == tiling.update as usize && frame_hdr.refresh_context != 0 {
                fc.task_thread.update_set.set(true);
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

fn rav1d_decode_frame_main(c: &Rav1dContext, f: &mut Rav1dFrameData) -> Rav1dResult {
    assert!(c.tc.len() == 1);

    let Rav1dContextTaskType::Single(t) = &c.tc[0].task else {
        panic!("Expected a single-threaded context");
    };
    let mut t = t.lock();

    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();

    for ctx in &mut f.a[..f.sb128w as usize * frame_hdr.tiling.rows as usize] {
        reset_context(ctx, frame_hdr.frame_type.is_key_or_intra(), 0);
    }

    // no threading - we explicitly interleave tile/sbrow decoding
    // and post-filtering, so that the full process runs in-line
    let Rav1dFrameHeaderTiling { rows, cols, .. } = frame_hdr.tiling;
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
                c.dsp.refmvs.load_tmvs.call(
                    &f.rf,
                    &f.mvs,
                    &f.ref_mvs,
                    tile_row as c_int,
                    0,
                    f.bw >> 1,
                    t.b.y >> 1,
                    by_end,
                );
            }
            for col in 0..cols {
                t.ts = tile_row * cols + col;
                rav1d_decode_tile_sbrow(c, &mut t, f).map_err(|()| EINVAL)?;
            }
            if f.frame_hdr().frame_type.is_inter_or_switch() {
                c.dsp
                    .refmvs
                    .save_tmvs
                    .call(&t.rt, &f.rf, &f.mvs, 0, f.bw >> 1, t.b.y >> 1, by_end);
            }

            // loopfilter + cdef + restoration
            (f.bd_fn().filter_sbrow)(c, f, &mut t, sby);
        }
    }

    Ok(())
}

pub(crate) fn rav1d_decode_frame_exit(
    c: &Rav1dContext,
    fc: &Rav1dFrameContext,
    mut retval: Rav1dResult,
) -> Rav1dResult {
    let task_thread = &fc.task_thread;
    // We use a blocking lock here because we have rare contention with other
    // threads.
    let mut f = fc.data.write();
    if f.sr_cur.p.data.is_some() {
        task_thread.error.store(0, Ordering::Relaxed);
    }
    let cf = f.frame_thread.cf.get_mut();
    if c.fc.len() > 1 && retval.is_err() {
        cf.fill_with(Default::default);
    }

    if retval.is_ok() && c.fc.len() > 1 && c.strict_std_compliance {
        if f.refp.iter().any(|rf| {
            rf.p.frame_hdr.is_some()
                && rf.progress.as_ref().unwrap()[1].load(Ordering::SeqCst) == FRAME_ERROR
        }) {
            retval = Err(EINVAL);
            task_thread.error.store(1, Ordering::SeqCst);
            f.sr_cur.progress.as_mut().unwrap()[1].store(FRAME_ERROR, Ordering::SeqCst);
        }
    }

    let _ = mem::take(&mut f.refp);
    let _ = mem::take(&mut f.ref_mvs);
    let _ = mem::take(&mut f.cur);
    let _ = mem::take(&mut f.sr_cur);
    let _ = mem::take(&mut *fc.in_cdf.try_write().unwrap());
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

    let _ = mem::take(&mut f.cur_segmap);
    let _ = mem::take(&mut f.prev_segmap);
    let _ = mem::take(&mut f.mvs);
    let _ = mem::take(&mut f.seq_hdr);
    let _ = mem::take(&mut f.frame_hdr);
    f.tiles.clear();
    task_thread.finished.store(true, Ordering::SeqCst);
    *task_thread.retval.try_lock().unwrap() = retval.err();
    retval
}

pub(crate) fn rav1d_decode_frame(c: &Rav1dContext, fc: &Rav1dFrameContext) -> Rav1dResult {
    assert!(c.fc.len() == 1);
    // if.tc.len() > 1 (but n_fc == 1), we could run init/exit in the task
    // threads also. Not sure it makes a measurable difference.
    let mut res = rav1d_decode_frame_init(c, fc);
    {
        // scope ensures f is dropped before rav1d_decode_frame_exit is called
        let mut f = fc.data.try_write().unwrap();
        if res.is_ok() {
            res = rav1d_decode_frame_init_cdf(c, fc, &mut f, &fc.in_cdf());
        }
        // wait until all threads have completed
        if res.is_ok() {
            if c.tc.len() > 1 {
                res = rav1d_task_create_tile_sbrow(fc, &f, 0, 1);
                drop(f); // release the frame data before waiting for the other threads
                let mut task_thread_lock = (*fc.task_thread.ttd).lock.lock();
                (*fc.task_thread.ttd).cond.notify_one();
                if res.is_ok() {
                    while fc.task_thread.done[0].load(Ordering::SeqCst) == 0
                        || fc.task_thread.task_counter.load(Ordering::SeqCst) > 0
                    {
                        fc.task_thread.cond.wait(&mut task_thread_lock);
                    }
                }
                drop(task_thread_lock);
                res = fc.task_thread.retval.try_lock().unwrap().err_or(());
            } else {
                res = rav1d_decode_frame_main(c, &mut f);
                let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
                if res.is_ok() && frame_hdr.refresh_context != 0 && fc.task_thread.update_set.get()
                {
                    rav1d_cdf_thread_update(
                        frame_hdr,
                        &mut f.out_cdf.cdf_write(),
                        &f.ts[frame_hdr.tiling.update as usize]
                            .context
                            .try_lock()
                            .unwrap()
                            .cdf,
                    );
                }
            }
        }
    }
    rav1d_decode_frame_exit(c, fc, res)
}

fn get_upscale_x0(in_w: c_int, out_w: c_int, step: c_int) -> c_int {
    let err = out_w * step - (in_w << 14);
    let x0 = (-(out_w - in_w << 13) + (out_w >> 1)) / out_w + 128 - err / 2;
    x0 & 0x3fff
}

pub fn rav1d_submit_frame(c: &Rav1dContext, state: &mut Rav1dState) -> Rav1dResult {
    // wait for c->out_delayed[next] and move into c->out if visible
    let (fc, out, _task_thread_lock) = if c.fc.len() > 1 {
        let mut task_thread_lock = c.task_thread.lock.lock();
        let next = state.frame_thread.next;
        state.frame_thread.next = (state.frame_thread.next + 1) % c.fc.len() as u32;

        let fc = &c.fc[next as usize];
        while !fc.task_thread.finished.load(Ordering::SeqCst) {
            fc.task_thread.cond.wait(&mut task_thread_lock);
        }
        let out_delayed = &mut state.frame_thread.out_delayed[next as usize];
        if out_delayed.p.data.is_some() || fc.task_thread.error.load(Ordering::SeqCst) != 0 {
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
            // `cur` is not actually mutated from multiple threads concurrently
            let cur = c.task_thread.cur.get();
            if cur != 0 && (cur as usize) < c.fc.len() {
                c.task_thread.cur.update(|cur| cur - 1);
            }
        }
        let error = &mut *fc.task_thread.retval.try_lock().unwrap();
        if error.is_some() {
            state.cached_error = mem::take(&mut *error);
            state.cached_error_props = out_delayed.p.m.clone();
            let _ = mem::take(out_delayed);
        } else if out_delayed.p.data.is_some() {
            let progress = out_delayed.progress.as_ref().unwrap()[1].load(Ordering::Relaxed);
            if (out_delayed.visible || c.output_invisible_frames) && progress != FRAME_ERROR {
                state.out = out_delayed.clone();
                state.event_flags |= out_delayed.flags.into();
            }
            let _ = mem::take(out_delayed);
        }
        (fc, out_delayed, Some(task_thread_lock))
    } else {
        (&c.fc[0], &mut state.out, None)
    };

    let mut f = fc.data.try_write().unwrap();
    f.seq_hdr = state.seq_hdr.clone();
    f.frame_hdr = mem::take(&mut state.frame_hdr);
    let seq_hdr = f.seq_hdr.clone().unwrap();

    fn on_error(
        fc: &Rav1dFrameContext,
        f: &mut Rav1dFrameData,
        out: &mut Rav1dThreadPicture,
        cached_error_props: &mut Rav1dDataProps,
        m: &Rav1dDataProps,
    ) {
        fc.task_thread.error.store(1, Ordering::Relaxed);
        let _ = mem::take(&mut *fc.in_cdf.try_write().unwrap());
        if f.frame_hdr.as_ref().unwrap().refresh_context != 0 {
            let _ = mem::take(&mut f.out_cdf);
        }
        for i in 0..7 {
            if f.refp[i].p.frame_hdr.is_some() {
                let _ = mem::take(&mut f.refp[i]);
            }
            let _ = mem::take(&mut f.ref_mvs[i]);
        }
        let _ = mem::take(out);
        let _ = mem::take(&mut f.cur);
        let _ = mem::take(&mut f.sr_cur);
        let _ = mem::take(&mut f.mvs);
        let _ = mem::take(&mut f.seq_hdr);
        let _ = mem::take(&mut f.frame_hdr);
        *cached_error_props = m.clone();

        f.tiles.clear();
        fc.task_thread.finished.store(true, Ordering::SeqCst);
    }

    let bpc = 8 + 2 * seq_hdr.hbd;
    match Rav1dBitDepthDSPContext::get(bpc) {
        Some(dsp) => f.dsp = dsp,
        None => {
            writeln!(c.logger, "Compiled without support for {bpc}-bit decoding",);
            on_error(
                fc,
                &mut f,
                out,
                &mut state.cached_error_props,
                &state.in_0.m,
            );
            return Err(ENOPROTOOPT);
        }
    };

    fn scale_fac(ref_sz: i32, this_sz: i32) -> i32 {
        ((ref_sz << 14) + (this_sz >> 1)) / this_sz
    }

    let mut ref_coded_width = <[i32; 7]>::default();
    let frame_hdr = f.frame_hdr.as_ref().unwrap().clone();
    if frame_hdr.frame_type.is_inter_or_switch() {
        if frame_hdr.primary_ref_frame != RAV1D_PRIMARY_REF_NONE {
            let pri_ref = frame_hdr.refidx[frame_hdr.primary_ref_frame as usize] as usize;
            if state.refs[pri_ref].p.p.data.is_none() {
                on_error(
                    fc,
                    &mut f,
                    out,
                    &mut state.cached_error_props,
                    &state.in_0.m,
                );
                return Err(EINVAL);
            }
        }
        for i in 0..7 {
            let refidx = frame_hdr.refidx[i] as usize;
            if state.refs[refidx].p.p.data.is_none()
                || (frame_hdr.size.width[0] * 2) < state.refs[refidx].p.p.p.w
                || (frame_hdr.size.height * 2) < state.refs[refidx].p.p.p.h
                || frame_hdr.size.width[0] > state.refs[refidx].p.p.p.w * 16
                || frame_hdr.size.height > state.refs[refidx].p.p.p.h * 16
                || seq_hdr.layout != state.refs[refidx].p.p.p.layout
                || bpc != state.refs[refidx].p.p.p.bpc
            {
                for j in 0..i {
                    let _ = mem::take(&mut f.refp[j]);
                }
                on_error(
                    fc,
                    &mut f,
                    out,
                    &mut state.cached_error_props,
                    &state.in_0.m,
                );
                return Err(EINVAL);
            }
            f.refp[i] = state.refs[refidx].p.clone();
            ref_coded_width[i] = state.refs[refidx]
                .p
                .p
                .frame_hdr
                .as_ref()
                .unwrap()
                .size
                .width[0];
            if frame_hdr.size.width[0] != state.refs[refidx].p.p.p.w
                || frame_hdr.size.height != state.refs[refidx].p.p.p.h
            {
                f.svc[i][0].scale = scale_fac(state.refs[refidx].p.p.p.w, frame_hdr.size.width[0]);
                f.svc[i][1].scale = scale_fac(state.refs[refidx].p.p.p.h, frame_hdr.size.height);
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
        *fc.in_cdf.try_write().unwrap() = rav1d_cdf_thread_init_static(frame_hdr.quant.yac);
    } else {
        let pri_ref = frame_hdr.refidx[frame_hdr.primary_ref_frame as usize] as usize;
        *fc.in_cdf.try_write().unwrap() = state.cdf[pri_ref].clone();
    }
    if frame_hdr.refresh_context != 0 {
        let res = rav1d_cdf_thread_alloc(c.fc.len() > 1);
        match res {
            Err(e) => {
                on_error(
                    fc,
                    &mut f,
                    out,
                    &mut state.cached_error_props,
                    &state.in_0.m,
                );
                return Err(e);
            }
            Ok(res) => {
                f.out_cdf = res;
            }
        }
    }

    // FIXME qsort so tiles are in order (for frame threading)
    f.tiles.clear();
    mem::swap(&mut f.tiles, &mut state.tiles);
    fc.task_thread
        .finished
        .store(f.tiles.is_empty(), Ordering::SeqCst);

    // allocate frame

    // We must take itut_t35 out of the context before the call so borrowck can
    // see we mutably borrow `c.itut_t35` disjointly from the task thread lock.
    let itut_t35 = mem::take(&mut state.itut_t35);
    let res = rav1d_thread_picture_alloc(
        &c.fc,
        &c.logger,
        &c.allocator,
        state.content_light.clone(),
        state.mastering_display.clone(),
        c.output_invisible_frames,
        state.max_spatial_id,
        &mut state.frame_flags,
        &mut f,
        bpc,
        itut_t35,
    );
    if res.is_err() {
        on_error(
            fc,
            &mut f,
            out,
            &mut state.cached_error_props,
            &state.in_0.m,
        );
        return res;
    }

    let seq_hdr = f.seq_hdr.as_ref().unwrap().clone();
    let frame_hdr = f.frame_hdr.as_ref().unwrap().clone();

    if frame_hdr.size.width[0] != frame_hdr.size.width[1] {
        // Re-borrow to allow independent borrows of fields
        let f = &mut *f;
        let res =
            rav1d_picture_alloc_copy(&c.logger, &mut f.cur, frame_hdr.size.width[0], &f.sr_cur.p);
        if res.is_err() {
            on_error(fc, f, out, &mut state.cached_error_props, &state.in_0.m);
            return res;
        }
    } else {
        f.cur = f.sr_cur.p.clone();
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
    if c.fc.len() == 1 {
        if frame_hdr.show_frame != 0 || c.output_invisible_frames {
            *out = f.sr_cur.clone();
            state.event_flags |= f.sr_cur.flags.into();
        }
    } else {
        *out = f.sr_cur.clone();
    }

    f.w4 = frame_hdr.size.width[0] + 3 >> 2;
    f.h4 = frame_hdr.size.height + 3 >> 2;
    f.bw = (frame_hdr.size.width[0] + 7 >> 3) << 1;
    f.bh = (frame_hdr.size.height + 7 >> 3) << 1;
    f.sb128w = f.bw + 31 >> 5;
    f.sb128h = f.bh + 31 >> 5;
    f.sb_shift = 4 + seq_hdr.sb128 as c_int;
    f.sb_step = 16 << seq_hdr.sb128;
    f.sbh = f.bh + f.sb_step - 1 >> f.sb_shift;
    f.b4_stride = (f.bw + 31 & !31) as ptrdiff_t;
    f.bitdepth_max = (1 << f.cur.p.bpc) - 1;
    fc.task_thread.error.store(0, Ordering::Relaxed);
    let uses_2pass = (c.fc.len() > 1) as c_int;
    let cols = frame_hdr.tiling.cols;
    let rows = frame_hdr.tiling.rows;
    fc.task_thread.task_counter.store(
        cols as c_int * rows as c_int + f.sbh << uses_2pass,
        Ordering::SeqCst,
    );

    // ref_mvs
    if frame_hdr.frame_type.is_inter_or_switch() || frame_hdr.allow_intrabc {
        // TODO fallible allocation
        f.mvs = Some(
            (0..f.sb128h as usize * 16 * (f.b4_stride >> 1) as usize)
                .map(|_| Default::default())
                .collect(),
        );
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
                if ref_w == f.bw && ref_h == f.bh {
                    f.ref_mvs[i] = state.refs[refidx].refmvs.clone();
                } else {
                    f.ref_mvs[i] = None;
                }
                f.refrefpoc[i] = state.refs[refidx].refpoc;
            }
        } else {
            f.ref_mvs.fill_with(Default::default);
        }
    } else {
        f.mvs = None;
        f.ref_mvs.fill_with(Default::default);
    }

    // segmap
    if frame_hdr.segmentation.enabled != 0 {
        // By default, the previous segmentation map is not initialised.
        f.prev_segmap = None;

        // We might need a previous frame's segmentation map.
        // This happens if there is either no update or a temporal update.
        if frame_hdr.segmentation.temporal != 0 || frame_hdr.segmentation.update_map == 0 {
            let pri_ref = frame_hdr.primary_ref_frame as usize;
            assert!(pri_ref != RAV1D_PRIMARY_REF_NONE as usize);
            let ref_w = (ref_coded_width[pri_ref] + 7 >> 3) << 1;
            let ref_h = (f.refp[pri_ref].p.p.h + 7 >> 3) << 1;
            if ref_w == f.bw && ref_h == f.bh {
                f.prev_segmap = state.refs[frame_hdr.refidx[pri_ref] as usize]
                    .segmap
                    .clone();
            }
        }

        f.cur_segmap = Some(
            match (
                frame_hdr.segmentation.update_map != 0,
                f.prev_segmap.as_mut(),
            ) {
                (true, _) | (false, None) => {
                    // If we're updating an existing map,
                    // we need somewhere to put the new values.
                    // Allocate them here (the data actually gets set elsewhere).
                    // Since this is Rust, we have to initialize it anyways.

                    // Otherwise if there's no previous, we need to make a new map.
                    // Allocate one here and zero it out.
                    let segmap_size = f.b4_stride as usize * 32 * f.sb128h as usize;
                    // TODO fallible allocation
                    (0..segmap_size).map(|_| Default::default()).collect()
                }
                (_, Some(prev_segmap)) => {
                    // We're not updating an existing map,
                    // and we have a valid reference. Use that.
                    prev_segmap.clone()
                }
            },
        );
    } else {
        f.cur_segmap = None;
        f.prev_segmap = None;
    }

    // update references etc.
    let refresh_frame_flags = frame_hdr.refresh_frame_flags as c_uint;
    for i in 0..8 {
        if refresh_frame_flags & (1 << i) != 0 {
            if state.refs[i].p.p.frame_hdr.is_some() {
                let _ = mem::take(&mut state.refs[i].p);
            }
            state.refs[i].p = f.sr_cur.clone();

            if frame_hdr.refresh_context != 0 {
                state.cdf[i] = f.out_cdf.clone();
            } else {
                state.cdf[i] = fc.in_cdf.try_read().unwrap().clone();
            }

            state.refs[i].segmap = f.cur_segmap.clone();
            let _ = mem::take(&mut state.refs[i].refmvs);
            if !frame_hdr.allow_intrabc {
                state.refs[i].refmvs = f.mvs.clone();
            }
            state.refs[i].refpoc = f.refpoc;
        }
    }
    drop(f);

    if c.fc.len() == 1 {
        let res = rav1d_decode_frame(c, &fc);
        if res.is_err() {
            let _ = mem::take(&mut state.out);
            for i in 0..8 {
                if refresh_frame_flags & (1 << i) != 0 {
                    if state.refs[i].p.p.frame_hdr.is_some() {
                        let _ = mem::take(&mut state.refs[i].p);
                    }
                    let _ = mem::take(&mut state.cdf[i]);
                    let _ = mem::take(&mut state.refs[i].segmap);
                    let _ = mem::take(&mut state.refs[i].refmvs);
                }
            }
            let mut f = fc.data.try_write().unwrap();
            on_error(
                fc,
                &mut f,
                &mut state.out,
                &mut state.cached_error_props,
                &state.in_0.m,
            );
            return res;
        }
    } else {
        rav1d_task_frame_init(c, fc);
    }

    Ok(())
}
