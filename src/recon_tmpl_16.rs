use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::BitDepth16;
use crate::include::common::bitdepth::DynCoef;
use crate::include::common::dump::ac_dump;
use crate::include::common::dump::coef_dump;
use crate::include::common::dump::hex_dump;
use crate::include::dav1d::dav1d::RAV1D_INLOOPFILTER_CDEF;
use crate::include::dav1d::dav1d::RAV1D_INLOOPFILTER_DEBLOCK;
use crate::include::dav1d::dav1d::RAV1D_INLOOPFILTER_RESTORATION;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I400;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I444;
use crate::include::dav1d::headers::RAV1D_WM_TYPE_TRANSLATION;
use crate::src::cdef_apply_tmpl_16::rav1d_cdef_brow_16bpc;
use crate::src::ctx::CaseSet;
use crate::src::internal::CodedBlockInfo;
use crate::src::internal::Rav1dDSPContext;
use crate::src::internal::Rav1dFrameContext;
use crate::src::internal::Rav1dTaskContext;
use crate::src::internal::Rav1dTileState;
use crate::src::intra_edge::EdgeFlags;
use crate::src::intra_edge::EDGE_I420_LEFT_HAS_BOTTOM;
use crate::src::intra_edge::EDGE_I420_TOP_HAS_RIGHT;
use crate::src::intra_edge::EDGE_I444_LEFT_HAS_BOTTOM;
use crate::src::intra_edge::EDGE_I444_TOP_HAS_RIGHT;
use crate::src::ipred_prepare::rav1d_prepare_intra_edges;
use crate::src::ipred_prepare::sm_flag;
use crate::src::ipred_prepare::sm_uv_flag;
use crate::src::levels::Av1Block;
use crate::src::levels::BlockSize;
use crate::src::levels::Filter2d;
use crate::src::levels::IntraPredMode;
use crate::src::levels::RectTxfmSize;
use crate::src::levels::TxfmType;
use crate::src::levels::CFL_PRED;
use crate::src::levels::COMP_INTER_NONE;
use crate::src::levels::DCT_DCT;
use crate::src::levels::DC_PRED;
use crate::src::levels::FILTER_2D_BILINEAR;
use crate::src::levels::GLOBALMV;
use crate::src::levels::GLOBALMV_GLOBALMV;
use crate::src::levels::II_SMOOTH_PRED;
use crate::src::levels::INTER_INTRA_BLEND;
use crate::src::levels::MM_OBMC;
use crate::src::levels::MM_WARP;
use crate::src::levels::SMOOTH_PRED;
use crate::src::lf_apply_tmpl_16::rav1d_copy_lpf_16bpc;
use crate::src::lf_apply_tmpl_16::rav1d_loopfilter_sbrow_cols_16bpc;
use crate::src::lf_apply_tmpl_16::rav1d_loopfilter_sbrow_rows_16bpc;
use crate::src::lf_mask::Av1Filter;
use crate::src::lr_apply_tmpl_16::rav1d_lr_sbrow_16bpc;
use crate::src::picture::Rav1dThreadPicture;
use crate::src::recon::decode_coefs;
use crate::src::recon::mc;
use crate::src::recon::obmc;
use crate::src::recon::read_coef_tree;
use crate::src::recon::warp_affine;
use crate::src::recon::DEBUG_BLOCK_INFO;
use crate::src::refmvs::refmvs_block;
use crate::src::tables::dav1d_block_dimensions;
use crate::src::tables::dav1d_filter_2d;
use crate::src::tables::dav1d_txfm_dimensions;
use crate::src::tables::TxfmInfo;
use crate::src::wedge::dav1d_ii_masks;
use crate::src::wedge::dav1d_wedge_masks;
use libc::memcpy;
use libc::printf;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_ulong;
use std::ffi::c_void;

pub type pixel = u16;
pub type coef = i32;

#[inline]
unsafe fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 != 0 {
        unreachable!();
    }
    return x >> 1;
}

pub(crate) unsafe extern "C" fn rav1d_recon_b_intra_16bpc(
    t: *mut Rav1dTaskContext,
    bs: BlockSize,
    intra_edge_flags: EdgeFlags,
    b: *const Av1Block,
) {
    let ts: *mut Rav1dTileState = (*t).ts;
    let f: *const Rav1dFrameContext = (*t).f;
    let dsp: *const Rav1dDSPContext = (*f).dsp;
    let bx4 = (*t).bx & 31;
    let by4 = (*t).by & 31;
    let ss_ver =
        ((*f).cur.p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint) as c_int;
    let ss_hor =
        ((*f).cur.p.layout as c_uint != RAV1D_PIXEL_LAYOUT_I444 as c_int as c_uint) as c_int;
    let cbx4 = bx4 >> ss_hor;
    let cby4 = by4 >> ss_ver;
    let b_dim: *const u8 = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bw4 = *b_dim.offset(0) as c_int;
    let bh4 = *b_dim.offset(1) as c_int;
    let w4 = cmp::min(bw4, (*f).bw - (*t).bx);
    let h4 = cmp::min(bh4, (*f).bh - (*t).by);
    let cw4 = w4 + ss_hor >> ss_hor;
    let ch4 = h4 + ss_ver >> ss_ver;
    let has_chroma = ((*f).cur.p.layout as c_uint != RAV1D_PIXEL_LAYOUT_I400 as c_int as c_uint
        && (bw4 > ss_hor || (*t).bx & 1 != 0)
        && (bh4 > ss_ver || (*t).by & 1 != 0)) as c_int;
    let t_dim: *const TxfmInfo = &*dav1d_txfm_dimensions
        .as_ptr()
        .offset((*b).c2rust_unnamed.c2rust_unnamed.tx as isize)
        as *const TxfmInfo;
    let uv_t_dim: *const TxfmInfo =
        &*dav1d_txfm_dimensions.as_ptr().offset((*b).uvtx as isize) as *const TxfmInfo;
    let edge: *mut pixel = ((*t)
        .scratch
        .c2rust_unnamed_0
        .c2rust_unnamed_0
        .c2rust_unnamed_0
        .edge_16bpc)
        .as_mut_ptr()
        .offset(128);
    let cbw4 = bw4 + ss_hor >> ss_hor;
    let cbh4 = bh4 + ss_ver >> ss_ver;
    let intra_edge_filter_flag = (*(*f).seq_hdr).intra_edge_filter << 10;
    let mut init_y = 0;
    while init_y < h4 {
        let sub_h4 = cmp::min(h4, 16 + init_y);
        let sub_ch4 = cmp::min(ch4, init_y + 16 >> ss_ver);
        let mut init_x = 0;
        while init_x < w4 {
            if (*b).c2rust_unnamed.c2rust_unnamed.pal_sz[0] != 0 {
                let dst: *mut pixel = ((*f).cur.data[0] as *mut pixel).offset(
                    (4 * ((*t).by as isize * PXSTRIDE((*f).cur.stride[0]) + (*t).bx as isize))
                        as isize,
                );
                let pal_idx: *const u8;
                if (*t).frame_thread.pass != 0 {
                    let p = (*t).frame_thread.pass & 1;
                    if ((*ts).frame_thread[p as usize].pal_idx).is_null() {
                        unreachable!();
                    }
                    pal_idx = (*ts).frame_thread[p as usize].pal_idx;
                    (*ts).frame_thread[p as usize].pal_idx =
                        ((*ts).frame_thread[p as usize].pal_idx).offset((bw4 * bh4 * 16) as isize);
                } else {
                    pal_idx = ((*t).scratch.c2rust_unnamed_0.pal_idx).as_mut_ptr();
                }
                let pal: *const u16 = if (*t).frame_thread.pass != 0 {
                    ((*((*f).frame_thread.pal).offset(
                        ((((*t).by as isize >> 1) + ((*t).bx as isize & 1)) * ((*f).b4_stride >> 1)
                            + (((*t).bx >> 1) + ((*t).by & 1)) as isize)
                            as isize,
                    ))[0])
                        .as_mut_ptr()
                } else {
                    ((*t).scratch.c2rust_unnamed_0.pal[0]).as_mut_ptr()
                };
                ((*(*f).dsp).ipred.pal_pred)(
                    dst.cast(),
                    (*f).cur.stride[0],
                    pal,
                    pal_idx,
                    bw4 * 4,
                    bh4 * 4,
                );
                if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                    hex_dump::<BitDepth16>(
                        dst,
                        PXSTRIDE((*f).cur.stride[0]) as usize,
                        bw4 as usize * 4,
                        bh4 as usize * 4,
                        "y-pal-pred",
                    );
                }
            }
            let intra_flags = sm_flag(&*(*t).a, bx4 as usize)
                | sm_flag(&mut (*t).l, by4 as usize)
                | intra_edge_filter_flag;
            let sb_has_tr = (if (init_x + 16) < w4 {
                1 as c_int as c_uint
            } else if init_y != 0 {
                0 as c_int as c_uint
            } else {
                intra_edge_flags as c_uint & EDGE_I444_TOP_HAS_RIGHT as c_int as c_uint
            }) as c_int;
            let sb_has_bl = (if init_x != 0 {
                0 as c_int as c_uint
            } else if (init_y + 16) < h4 {
                1 as c_int as c_uint
            } else {
                intra_edge_flags as c_uint & EDGE_I444_LEFT_HAS_BOTTOM as c_int as c_uint
            }) as c_int;
            let mut y;
            let mut x;
            let sub_w4 = cmp::min(w4, init_x + 16);
            y = init_y;
            (*t).by += init_y;
            while y < sub_h4 {
                let mut dst_0: *mut pixel = ((*f).cur.data[0] as *mut pixel).offset(
                    (4 * ((*t).by as isize * PXSTRIDE((*f).cur.stride[0])
                        + (*t).bx as isize
                        + init_x as isize)) as isize,
                );
                x = init_x;
                (*t).bx += init_x;
                while x < sub_w4 {
                    let mut angle;
                    let edge_flags: EdgeFlags;
                    let mut top_sb_edge: *const pixel;
                    let m: IntraPredMode;
                    if !((*b).c2rust_unnamed.c2rust_unnamed.pal_sz[0] != 0) {
                        angle = (*b).c2rust_unnamed.c2rust_unnamed.y_angle as c_int;
                        edge_flags = ((if (y > init_y || sb_has_tr == 0)
                            && x + (*t_dim).w as c_int >= sub_w4
                        {
                            0 as c_int
                        } else {
                            EDGE_I444_TOP_HAS_RIGHT as c_int
                        }) | (if x > init_x
                            || sb_has_bl == 0 && y + (*t_dim).h as c_int >= sub_h4
                        {
                            0 as c_int
                        } else {
                            EDGE_I444_LEFT_HAS_BOTTOM as c_int
                        })) as EdgeFlags;
                        top_sb_edge = 0 as *const pixel;
                        if (*t).by & (*f).sb_step - 1 == 0 {
                            top_sb_edge = (*f).ipred_edge[0] as *mut pixel;
                            let sby = (*t).by >> (*f).sb_shift;
                            top_sb_edge =
                                top_sb_edge.offset(((*f).sb128w * 128 * (sby - 1)) as isize);
                        }
                        m = rav1d_prepare_intra_edges::<BitDepth16>(
                            (*t).bx,
                            ((*t).bx > (*ts).tiling.col_start) as c_int,
                            (*t).by,
                            ((*t).by > (*ts).tiling.row_start) as c_int,
                            (*ts).tiling.col_end,
                            (*ts).tiling.row_end,
                            edge_flags,
                            dst_0,
                            (*f).cur.stride[0],
                            top_sb_edge,
                            (*b).c2rust_unnamed.c2rust_unnamed.y_mode as IntraPredMode,
                            &mut angle,
                            (*t_dim).w as c_int,
                            (*t_dim).h as c_int,
                            (*(*f).seq_hdr).intra_edge_filter,
                            edge,
                            BitDepth16::from_c((*f).bitdepth_max),
                        );
                        ((*dsp).ipred.intra_pred[m as usize]).expect("non-null function pointer")(
                            dst_0.cast(),
                            (*f).cur.stride[0],
                            edge.cast(),
                            (*t_dim).w as c_int * 4,
                            (*t_dim).h as c_int * 4,
                            angle | intra_flags,
                            4 * (*f).bw - 4 * (*t).bx,
                            4 * (*f).bh - 4 * (*t).by,
                            (*f).bitdepth_max,
                        );
                        if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                            hex_dump::<BitDepth16>(
                                edge.offset(-(((*t_dim).h as c_int * 4) as isize)),
                                (*t_dim).h as usize * 4,
                                (*t_dim).h as usize * 4,
                                2,
                                "l",
                            );
                            hex_dump::<BitDepth16>(edge, 0, 1, 1, "tl");
                            hex_dump::<BitDepth16>(
                                edge.offset(1),
                                (*t_dim).w as usize * 4,
                                (*t_dim).w as usize * 4,
                                2,
                                "t",
                            );
                            hex_dump::<BitDepth16>(
                                dst_0,
                                (*f).cur.stride[0] as usize,
                                (*t_dim).w as usize * 4,
                                (*t_dim).h as usize * 4,
                                "y-intra-pred",
                            );
                        }
                    }
                    if (*b).skip == 0 {
                        let cf: *mut coef;
                        let eob;
                        let mut txtp: TxfmType = DCT_DCT;
                        if (*t).frame_thread.pass != 0 {
                            let p_0 = (*t).frame_thread.pass & 1;
                            cf = (*ts).frame_thread[p_0 as usize].cf as *mut coef;
                            (*ts).frame_thread[p_0 as usize].cf =
                                ((*ts).frame_thread[p_0 as usize].cf as *mut coef).offset(
                                    (cmp::min((*t_dim).w as c_int, 8 as c_int)
                                        * cmp::min((*t_dim).h as c_int, 8 as c_int)
                                        * 16) as isize,
                                ) as *mut DynCoef;
                            let cbi: *const CodedBlockInfo = &mut *((*f).frame_thread.cbi).offset(
                                ((*t).by as isize * (*f).b4_stride + (*t).bx as isize) as isize,
                            )
                                as *mut CodedBlockInfo;
                            eob = (*cbi).eob[0] as c_int;
                            txtp = (*cbi).txtp[0] as TxfmType;
                        } else {
                            let mut cf_ctx: u8 = 0;
                            cf = ((*t).c2rust_unnamed.cf_16bpc).as_mut_ptr();
                            eob = decode_coefs::<BitDepth16>(
                                t,
                                &mut (*(*t).a).lcoef.0[(bx4 + x) as usize..],
                                &mut (*t).l.lcoef.0[(by4 + y) as usize..],
                                (*b).c2rust_unnamed.c2rust_unnamed.tx as RectTxfmSize,
                                bs,
                                b,
                                1 as c_int,
                                0 as c_int,
                                cf,
                                &mut txtp,
                                &mut cf_ctx,
                            );
                            if DEBUG_BLOCK_INFO(&*f, &*t) {
                                printf(
                                    b"Post-y-cf-blk[tx=%d,txtp=%d,eob=%d]: r=%d\n\0" as *const u8
                                        as *const c_char,
                                    (*b).c2rust_unnamed.c2rust_unnamed.tx as c_int,
                                    txtp as c_uint,
                                    eob,
                                    (*ts).msac.rng,
                                );
                            }
                            CaseSet::<16, true>::many(
                                [&mut (*t).l, &mut *(*t).a],
                                [
                                    cmp::min((*t_dim).h as i32, (*f).bh - (*t).by) as usize,
                                    cmp::min((*t_dim).w as i32, (*f).bw - (*t).bx) as usize,
                                ],
                                [(by4 + y) as usize, (bx4 + x) as usize],
                                |case, dir| {
                                    case.set(&mut dir.lcoef.0, cf_ctx);
                                },
                            );
                        }
                        if eob >= 0 {
                            if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                                coef_dump(
                                    cf,
                                    cmp::min((*t_dim).h as usize, 8) * 4,
                                    cmp::min((*t_dim).w as usize, 8) * 4,
                                    3,
                                    "dq",
                                );
                            }
                            ((*dsp).itx.itxfm_add[(*b).c2rust_unnamed.c2rust_unnamed.tx as usize]
                                [txtp as usize])
                                .expect("non-null function pointer")(
                                dst_0.cast(),
                                (*f).cur.stride[0],
                                cf.cast(),
                                eob,
                                (*f).bitdepth_max,
                            );
                            if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                                hex_dump::<BitDepth16>(
                                    dst_0,
                                    (*f).cur.stride[0] as usize,
                                    (*t_dim).w as usize * 4,
                                    (*t_dim).h as usize * 4,
                                    "recon",
                                );
                            }
                        }
                    } else if (*t).frame_thread.pass == 0 {
                        CaseSet::<16, false>::many(
                            [&mut (*t).l, &mut *(*t).a],
                            [(*t_dim).h as usize, (*t_dim).w as usize],
                            [(by4 + y) as usize, (bx4 + x) as usize],
                            |case, dir| {
                                case.set(&mut dir.lcoef.0, 0x40);
                            },
                        );
                    }
                    dst_0 = dst_0.offset((4 * (*t_dim).w as c_int) as isize);
                    x += (*t_dim).w as c_int;
                    (*t).bx += (*t_dim).w as c_int;
                }
                (*t).bx -= x;
                y += (*t_dim).h as c_int;
                (*t).by += (*t_dim).h as c_int;
            }
            (*t).by -= y;
            if !(has_chroma == 0) {
                let stride: ptrdiff_t = (*f).cur.stride[1];
                if (*b).c2rust_unnamed.c2rust_unnamed.uv_mode as c_int == CFL_PRED as c_int {
                    if !(init_x == 0 && init_y == 0) {
                        unreachable!();
                    }
                    let ac = &mut (*t).scratch.c2rust_unnamed_0.ac;
                    let y_src: *mut pixel = ((*f).cur.data[0] as *mut pixel)
                        .offset((4 * ((*t).bx & !ss_hor)) as isize)
                        .offset(
                            ((4 * ((*t).by & !ss_ver)) as isize * PXSTRIDE((*f).cur.stride[0]))
                                as isize,
                        );
                    let uv_off: ptrdiff_t = 4
                        * (((*t).bx >> ss_hor) as isize
                            + ((*t).by >> ss_ver) as isize * PXSTRIDE(stride));
                    let uv_dst: [*mut pixel; 2] = [
                        ((*f).cur.data[1] as *mut pixel).offset(uv_off as isize),
                        ((*f).cur.data[2] as *mut pixel).offset(uv_off as isize),
                    ];
                    let furthest_r =
                        (cw4 << ss_hor) + (*t_dim).w as c_int - 1 & !((*t_dim).w as c_int - 1);
                    let furthest_b =
                        (ch4 << ss_ver) + (*t_dim).h as c_int - 1 & !((*t_dim).h as c_int - 1);
                    (*dsp).ipred.cfl_ac
                        [((*f).cur.p.layout as c_uint).wrapping_sub(1 as c_int as c_uint) as usize](
                        ac.as_mut_ptr(),
                        y_src.cast(),
                        (*f).cur.stride[0],
                        cbw4 - (furthest_r >> ss_hor),
                        cbh4 - (furthest_b >> ss_ver),
                        cbw4 * 4,
                        cbh4 * 4,
                    );
                    let mut pl = 0;
                    while pl < 2 {
                        if !((*b).c2rust_unnamed.c2rust_unnamed.cfl_alpha[pl as usize] == 0) {
                            let mut angle_0 = 0;
                            let mut top_sb_edge_0: *const pixel = 0 as *const pixel;
                            if (*t).by & !ss_ver & (*f).sb_step - 1 == 0 {
                                top_sb_edge_0 = (*f).ipred_edge[(pl + 1) as usize] as *mut pixel;
                                let sby_0 = (*t).by >> (*f).sb_shift;
                                top_sb_edge_0 = top_sb_edge_0
                                    .offset(((*f).sb128w * 128 * (sby_0 - 1)) as isize);
                            }
                            let xpos = (*t).bx >> ss_hor;
                            let ypos = (*t).by >> ss_ver;
                            let xstart = (*ts).tiling.col_start >> ss_hor;
                            let ystart = (*ts).tiling.row_start >> ss_ver;
                            let m_0: IntraPredMode = rav1d_prepare_intra_edges::<BitDepth16>(
                                xpos,
                                (xpos > xstart) as c_int,
                                ypos,
                                (ypos > ystart) as c_int,
                                (*ts).tiling.col_end >> ss_hor,
                                (*ts).tiling.row_end >> ss_ver,
                                0 as EdgeFlags,
                                uv_dst[pl as usize],
                                stride,
                                top_sb_edge_0,
                                DC_PRED,
                                &mut angle_0,
                                (*uv_t_dim).w as c_int,
                                (*uv_t_dim).h as c_int,
                                0 as c_int,
                                edge,
                                BitDepth16::from_c((*f).bitdepth_max),
                            );
                            (*dsp).ipred.cfl_pred[m_0 as usize](
                                uv_dst[pl as usize].cast(),
                                stride,
                                edge.cast(),
                                (*uv_t_dim).w as c_int * 4,
                                (*uv_t_dim).h as c_int * 4,
                                ac.as_mut_ptr(),
                                (*b).c2rust_unnamed.c2rust_unnamed.cfl_alpha[pl as usize] as c_int,
                                (*f).bitdepth_max,
                            );
                        }
                        pl += 1;
                    }
                    if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                        ac_dump(ac, 4 * cbw4 as usize, 4 * cbh4 as usize, "ac");
                        hex_dump::<BitDepth16>(
                            uv_dst[0],
                            stride as usize,
                            cbw4 as usize * 4,
                            cbh4 as usize * 4,
                            "u-cfl-pred",
                        );
                        hex_dump::<BitDepth16>(
                            uv_dst[1],
                            stride as usize,
                            cbw4 as usize * 4,
                            cbh4 as usize * 4,
                            "v-cfl-pred",
                        );
                    }
                } else if (*b).c2rust_unnamed.c2rust_unnamed.pal_sz[1] != 0 {
                    let uv_dstoff: ptrdiff_t = 4
                        * (((*t).bx >> ss_hor) as isize
                            + ((*t).by >> ss_ver) as isize * PXSTRIDE((*f).cur.stride[1]));
                    let pal_0: *const [u16; 8];
                    let pal_idx_0: *const u8;
                    if (*t).frame_thread.pass != 0 {
                        let p_1 = (*t).frame_thread.pass & 1;
                        if ((*ts).frame_thread[p_1 as usize].pal_idx).is_null() {
                            unreachable!();
                        }
                        pal_0 = (*((*f).frame_thread.pal).offset(
                            ((((*t).by >> 1) + ((*t).bx & 1)) as isize * ((*f).b4_stride >> 1)
                                + (((*t).bx as isize >> 1) as isize + ((*t).by as isize & 1))
                                    as isize) as isize,
                        ))
                        .as_mut_ptr() as *const [u16; 8];
                        pal_idx_0 = (*ts).frame_thread[p_1 as usize].pal_idx;
                        (*ts).frame_thread[p_1 as usize].pal_idx =
                            ((*ts).frame_thread[p_1 as usize].pal_idx)
                                .offset((cbw4 * cbh4 * 16) as isize);
                    } else {
                        pal_0 = ((*t).scratch.c2rust_unnamed_0.pal).as_mut_ptr() as *const [u16; 8];
                        pal_idx_0 = &mut *((*t).scratch.c2rust_unnamed_0.pal_idx)
                            .as_mut_ptr()
                            .offset((bw4 * bh4 * 16) as isize)
                            as *mut u8;
                    }
                    ((*(*f).dsp).ipred.pal_pred)(
                        ((*f).cur.data[1] as *mut pixel)
                            .offset(uv_dstoff as isize)
                            .cast(),
                        (*f).cur.stride[1],
                        (*pal_0.offset(1)).as_ptr(),
                        pal_idx_0,
                        cbw4 * 4,
                        cbh4 * 4,
                    );
                    ((*(*f).dsp).ipred.pal_pred)(
                        ((*f).cur.data[2] as *mut pixel)
                            .offset(uv_dstoff as isize)
                            .cast(),
                        (*f).cur.stride[1],
                        (*pal_0.offset(2)).as_ptr(),
                        pal_idx_0,
                        cbw4 * 4,
                        cbh4 * 4,
                    );
                    if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                        hex_dump::<BitDepth16>(
                            ((*f).cur.data[1] as *mut pixel).offset(uv_dstoff as isize),
                            PXSTRIDE((*f).cur.stride[1]) as usize,
                            cbw4 as usize * 4,
                            cbh4 as usize * 4,
                            "u-pal-pred",
                        );
                        hex_dump::<BitDepth16>(
                            ((*f).cur.data[2] as *mut pixel).offset(uv_dstoff as isize),
                            PXSTRIDE((*f).cur.stride[1]) as usize,
                            cbw4 as usize * 4,
                            cbh4 as usize * 4,
                            "v-pal-pred",
                        );
                    }
                }
                let sm_uv_fl =
                    sm_uv_flag(&*(*t).a, cbx4 as usize) | sm_uv_flag(&mut (*t).l, cby4 as usize);
                let uv_sb_has_tr = (if init_x + 16 >> ss_hor < cw4 {
                    1 as c_int as c_uint
                } else if init_y != 0 {
                    0 as c_int as c_uint
                } else {
                    intra_edge_flags as c_uint
                        & (EDGE_I420_TOP_HAS_RIGHT as c_int
                            >> ((*f).cur.p.layout as c_uint).wrapping_sub(1 as c_int as c_uint))
                            as c_uint
                }) as c_int;
                let uv_sb_has_bl = (if init_x != 0 {
                    0 as c_int as c_uint
                } else if init_y + 16 >> ss_ver < ch4 {
                    1 as c_int as c_uint
                } else {
                    intra_edge_flags as c_uint
                        & (EDGE_I420_LEFT_HAS_BOTTOM as c_int
                            >> ((*f).cur.p.layout as c_uint).wrapping_sub(1 as c_int as c_uint))
                            as c_uint
                }) as c_int;
                let sub_cw4 = cmp::min(cw4, init_x + 16 >> ss_hor);
                let mut pl_0 = 0;
                while pl_0 < 2 {
                    y = init_y >> ss_ver;
                    (*t).by += init_y;
                    while y < sub_ch4 {
                        let mut dst_1: *mut pixel =
                            ((*f).cur.data[(1 + pl_0) as usize] as *mut pixel).offset(
                                (4 * (((*t).by >> ss_ver) as isize * PXSTRIDE(stride)
                                    + ((*t).bx + init_x >> ss_hor) as isize))
                                    as isize,
                            );
                        x = init_x >> ss_hor;
                        (*t).bx += init_x;
                        while x < sub_cw4 {
                            let mut angle_1;
                            let edge_flags_0: EdgeFlags;
                            let mut top_sb_edge_1: *const pixel;
                            let uv_mode: IntraPredMode;
                            let xpos_0;
                            let ypos_0;
                            let xstart_0;
                            let ystart_0;
                            let m_1: IntraPredMode;
                            if !((*b).c2rust_unnamed.c2rust_unnamed.uv_mode as c_int
                                == CFL_PRED as c_int
                                && (*b).c2rust_unnamed.c2rust_unnamed.cfl_alpha[pl_0 as usize]
                                    as c_int
                                    != 0
                                || (*b).c2rust_unnamed.c2rust_unnamed.pal_sz[1] as c_int != 0)
                            {
                                angle_1 = (*b).c2rust_unnamed.c2rust_unnamed.uv_angle as c_int;
                                edge_flags_0 = ((if (y > init_y >> ss_ver || uv_sb_has_tr == 0)
                                    && x + (*uv_t_dim).w as c_int >= sub_cw4
                                {
                                    0 as c_int
                                } else {
                                    EDGE_I444_TOP_HAS_RIGHT as c_int
                                }) | (if x > init_x >> ss_hor
                                    || uv_sb_has_bl == 0 && y + (*uv_t_dim).h as c_int >= sub_ch4
                                {
                                    0 as c_int
                                } else {
                                    EDGE_I444_LEFT_HAS_BOTTOM as c_int
                                })) as EdgeFlags;
                                top_sb_edge_1 = 0 as *const pixel;
                                if (*t).by & !ss_ver & (*f).sb_step - 1 == 0 {
                                    top_sb_edge_1 =
                                        (*f).ipred_edge[(1 + pl_0) as usize] as *mut pixel;
                                    let sby_1 = (*t).by >> (*f).sb_shift;
                                    top_sb_edge_1 = top_sb_edge_1
                                        .offset(((*f).sb128w * 128 * (sby_1 - 1)) as isize);
                                }
                                uv_mode = (if (*b).c2rust_unnamed.c2rust_unnamed.uv_mode as c_int
                                    == CFL_PRED as c_int
                                {
                                    DC_PRED as c_int
                                } else {
                                    (*b).c2rust_unnamed.c2rust_unnamed.uv_mode as c_int
                                }) as IntraPredMode;
                                xpos_0 = (*t).bx >> ss_hor;
                                ypos_0 = (*t).by >> ss_ver;
                                xstart_0 = (*ts).tiling.col_start >> ss_hor;
                                ystart_0 = (*ts).tiling.row_start >> ss_ver;
                                m_1 = rav1d_prepare_intra_edges::<BitDepth16>(
                                    xpos_0,
                                    (xpos_0 > xstart_0) as c_int,
                                    ypos_0,
                                    (ypos_0 > ystart_0) as c_int,
                                    (*ts).tiling.col_end >> ss_hor,
                                    (*ts).tiling.row_end >> ss_ver,
                                    edge_flags_0,
                                    dst_1,
                                    stride,
                                    top_sb_edge_1,
                                    uv_mode,
                                    &mut angle_1,
                                    (*uv_t_dim).w as c_int,
                                    (*uv_t_dim).h as c_int,
                                    (*(*f).seq_hdr).intra_edge_filter,
                                    edge,
                                    BitDepth16::from_c((*f).bitdepth_max),
                                );
                                angle_1 |= intra_edge_filter_flag;
                                ((*dsp).ipred.intra_pred[m_1 as usize])
                                    .expect("non-null function pointer")(
                                    dst_1.cast(),
                                    stride,
                                    edge.cast(),
                                    (*uv_t_dim).w as c_int * 4,
                                    (*uv_t_dim).h as c_int * 4,
                                    angle_1 | sm_uv_fl,
                                    4 * (*f).bw + ss_hor - 4 * ((*t).bx & !ss_hor) >> ss_hor,
                                    4 * (*f).bh + ss_ver - 4 * ((*t).by & !ss_ver) >> ss_ver,
                                    (*f).bitdepth_max,
                                );
                                if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                                    hex_dump::<BitDepth16>(
                                        edge.offset(-(((*uv_t_dim).h as c_int * 4) as isize)),
                                        (*uv_t_dim).h as usize * 4,
                                        (*uv_t_dim).h as usize * 4,
                                        2,
                                        "l",
                                    );
                                    hex_dump::<BitDepth16>(edge, 0, 1, 1, "tl");
                                    hex_dump::<BitDepth16>(
                                        edge.offset(1),
                                        (*uv_t_dim).w as usize * 4,
                                        (*uv_t_dim).w as usize * 4,
                                        2,
                                        "t",
                                    );
                                    hex_dump::<BitDepth16>(
                                        dst_1,
                                        stride as usize,
                                        (*uv_t_dim).w as usize * 4,
                                        (*uv_t_dim).h as usize * 4,
                                        if pl_0 != 0 {
                                            "v-intra-pred"
                                        } else {
                                            "u-intra-pred"
                                        },
                                    );
                                }
                            }
                            if (*b).skip == 0 {
                                let mut txtp_0: TxfmType = DCT_DCT;
                                let eob_0;
                                let cf_0: *mut coef;
                                if (*t).frame_thread.pass != 0 {
                                    let p_2 = (*t).frame_thread.pass & 1;
                                    cf_0 = (*ts).frame_thread[p_2 as usize].cf as *mut coef;
                                    (*ts).frame_thread[p_2 as usize].cf =
                                        ((*ts).frame_thread[p_2 as usize].cf as *mut coef).offset(
                                            ((*uv_t_dim).w as c_int * (*uv_t_dim).h as c_int * 16)
                                                as isize,
                                        ) as *mut DynCoef;
                                    let cbi_0: *const CodedBlockInfo = &mut *((*f).frame_thread.cbi)
                                        .offset(
                                            ((*t).by as isize * (*f).b4_stride + (*t).bx as isize)
                                                as isize,
                                        )
                                        as *mut CodedBlockInfo;
                                    eob_0 = (*cbi_0).eob[(pl_0 + 1) as usize] as c_int;
                                    txtp_0 = (*cbi_0).txtp[(pl_0 + 1) as usize] as TxfmType;
                                } else {
                                    let mut cf_ctx_0: u8 = 0;
                                    cf_0 = ((*t).c2rust_unnamed.cf_16bpc).as_mut_ptr();
                                    eob_0 = decode_coefs::<BitDepth16>(
                                        t,
                                        &mut (*(*t).a).ccoef.0[pl_0 as usize]
                                            [(cbx4 + x) as usize..],
                                        &mut (*t).l.ccoef.0[pl_0 as usize][(cby4 + y) as usize..],
                                        (*b).uvtx as RectTxfmSize,
                                        bs,
                                        b,
                                        1 as c_int,
                                        1 + pl_0,
                                        cf_0,
                                        &mut txtp_0,
                                        &mut cf_ctx_0,
                                    );
                                    if DEBUG_BLOCK_INFO(&*f, &*t) {
                                        printf(
                                            b"Post-uv-cf-blk[pl=%d,tx=%d,txtp=%d,eob=%d]: r=%d [x=%d,cbx4=%d]\n\0"
                                                as *const u8 as *const c_char,
                                            pl_0,
                                            (*b).uvtx as c_int,
                                            txtp_0 as c_uint,
                                            eob_0,
                                            (*ts).msac.rng,
                                            x,
                                            cbx4,
                                        );
                                    }
                                    CaseSet::<16, true>::many(
                                        [&mut (*t).l, &mut *(*t).a],
                                        [
                                            cmp::min(
                                                (*uv_t_dim).h as i32,
                                                (*f).bh - (*t).by + ss_ver >> ss_ver,
                                            ) as usize,
                                            cmp::min(
                                                (*uv_t_dim).w as i32,
                                                (*f).bw - (*t).bx + ss_hor >> ss_hor,
                                            ) as usize,
                                        ],
                                        [(cby4 + y) as usize, (cbx4 + x) as usize],
                                        |case, dir| {
                                            case.set(&mut dir.ccoef.0[pl_0 as usize], cf_ctx_0);
                                        },
                                    );
                                }
                                if eob_0 >= 0 {
                                    if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                                        coef_dump(
                                            cf_0,
                                            (*uv_t_dim).h as usize * 4,
                                            (*uv_t_dim).w as usize * 4,
                                            3,
                                            "dq",
                                        );
                                    }
                                    ((*dsp).itx.itxfm_add[(*b).uvtx as usize][txtp_0 as usize])
                                        .expect("non-null function pointer")(
                                        dst_1.cast(),
                                        stride,
                                        cf_0.cast(),
                                        eob_0,
                                        (*f).bitdepth_max,
                                    );
                                    if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                                        hex_dump::<BitDepth16>(
                                            dst_1,
                                            stride as usize,
                                            (*uv_t_dim).w as usize * 4,
                                            (*uv_t_dim).h as usize * 4,
                                            "recon",
                                        );
                                    }
                                }
                            } else if (*t).frame_thread.pass == 0 {
                                CaseSet::<16, false>::many(
                                    [&mut (*t).l, &mut *(*t).a],
                                    [(*uv_t_dim).h as usize, (*uv_t_dim).w as usize],
                                    [(cby4 + y) as usize, (cbx4 + x) as usize],
                                    |case, dir| {
                                        case.set(&mut dir.ccoef.0[pl_0 as usize], 0x40);
                                    },
                                );
                            }
                            dst_1 = dst_1.offset(((*uv_t_dim).w as c_int * 4) as isize);
                            x += (*uv_t_dim).w as c_int;
                            (*t).bx += ((*uv_t_dim).w as c_int) << ss_hor;
                        }
                        (*t).bx -= x << ss_hor;
                        y += (*uv_t_dim).h as c_int;
                        (*t).by += ((*uv_t_dim).h as c_int) << ss_ver;
                    }
                    (*t).by -= y << ss_ver;
                    pl_0 += 1;
                }
            }
            init_x += 16 as c_int;
        }
        init_y += 16 as c_int;
    }
}

pub(crate) unsafe extern "C" fn rav1d_recon_b_inter_16bpc(
    t: *mut Rav1dTaskContext,
    bs: BlockSize,
    b: *const Av1Block,
) -> c_int {
    let ts: *mut Rav1dTileState = (*t).ts;
    let f: *const Rav1dFrameContext = (*t).f;
    let dsp: *const Rav1dDSPContext = (*f).dsp;
    let bx4 = (*t).bx & 31;
    let by4 = (*t).by & 31;
    let ss_ver =
        ((*f).cur.p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint) as c_int;
    let ss_hor =
        ((*f).cur.p.layout as c_uint != RAV1D_PIXEL_LAYOUT_I444 as c_int as c_uint) as c_int;
    let cbx4 = bx4 >> ss_hor;
    let cby4 = by4 >> ss_ver;
    let b_dim: *const u8 = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bw4 = *b_dim.offset(0) as c_int;
    let bh4 = *b_dim.offset(1) as c_int;
    let w4 = cmp::min(bw4, (*f).bw - (*t).bx);
    let h4 = cmp::min(bh4, (*f).bh - (*t).by);
    let has_chroma = ((*f).cur.p.layout as c_uint != RAV1D_PIXEL_LAYOUT_I400 as c_int as c_uint
        && (bw4 > ss_hor || (*t).bx & 1 != 0)
        && (bh4 > ss_ver || (*t).by & 1 != 0)) as c_int;
    let chr_layout_idx =
        (if (*f).cur.p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I400 as c_int as c_uint {
            0 as c_int as c_uint
        } else {
            (RAV1D_PIXEL_LAYOUT_I444 as c_int as c_uint).wrapping_sub((*f).cur.p.layout as c_uint)
        }) as c_int;
    let mut res;
    let cbh4 = bh4 + ss_ver >> ss_ver;
    let cbw4 = bw4 + ss_hor >> ss_hor;
    let mut dst: *mut pixel = ((*f).cur.data[0] as *mut pixel).offset(
        (4 * ((*t).by as isize * PXSTRIDE((*f).cur.stride[0]) + (*t).bx as isize)) as isize,
    );
    let uvdstoff: ptrdiff_t = 4
        * (((*t).bx >> ss_hor) as isize
            + ((*t).by >> ss_ver) as isize * PXSTRIDE((*f).cur.stride[1]));
    if (*(*f).frame_hdr).frame_type as c_uint & 1 as c_uint == 0 {
        if (*(*f).frame_hdr).super_res.enabled != 0 {
            unreachable!();
        }
        res = mc::<BitDepth16>(
            t,
            dst,
            0 as *mut i16,
            (*f).cur.stride[0],
            bw4,
            bh4,
            (*t).bx,
            (*t).by,
            0 as c_int,
            (*b).c2rust_unnamed
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c2rust_unnamed
                .mv[0],
            &(*f).sr_cur,
            0 as c_int,
            FILTER_2D_BILINEAR,
        );
        if res != 0 {
            return res;
        }
        if has_chroma != 0 {
            let mut pl = 1;
            while pl < 3 {
                res = mc::<BitDepth16>(
                    t,
                    ((*f).cur.data[pl as usize] as *mut pixel).offset(uvdstoff as isize),
                    0 as *mut i16,
                    (*f).cur.stride[1],
                    bw4 << (bw4 == ss_hor) as c_int,
                    bh4 << (bh4 == ss_ver) as c_int,
                    (*t).bx & !ss_hor,
                    (*t).by & !ss_ver,
                    pl,
                    (*b).c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0],
                    &(*f).sr_cur,
                    0 as c_int,
                    FILTER_2D_BILINEAR,
                );
                if res != 0 {
                    return res;
                }
                pl += 1;
            }
        }
    } else if (*b).c2rust_unnamed.c2rust_unnamed_0.comp_type as c_int == COMP_INTER_NONE as c_int {
        let mut is_sub8x8;
        let mut r: *const *mut refmvs_block;
        let refp: *const Rav1dThreadPicture = &*((*f).refp).as_ptr().offset(
            *((*b).c2rust_unnamed.c2rust_unnamed_0.r#ref)
                .as_ptr()
                .offset(0) as isize,
        ) as *const Rav1dThreadPicture;
        let filter_2d: Filter2d = (*b).c2rust_unnamed.c2rust_unnamed_0.filter2d as Filter2d;
        if cmp::min(bw4, bh4) > 1
            && ((*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode as c_int == GLOBALMV as c_int
                && (*f).gmv_warp_allowed[(*b).c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as usize]
                    as c_int
                    != 0
                || (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode as c_int == MM_WARP as c_int
                    && (*t).warpmv.type_0 as c_uint > RAV1D_WM_TYPE_TRANSLATION as c_int as c_uint)
        {
            res = warp_affine::<BitDepth16>(
                t,
                dst,
                0 as *mut i16,
                (*f).cur.stride[0],
                b_dim,
                0 as c_int,
                refp,
                if (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode as c_int == MM_WARP as c_int {
                    &mut (*t).warpmv
                } else {
                    &mut *((*(*f).frame_hdr).gmv).as_mut_ptr().offset(
                        *((*b).c2rust_unnamed.c2rust_unnamed_0.r#ref)
                            .as_ptr()
                            .offset(0) as isize,
                    )
                },
            );
            if res != 0 {
                return res;
            }
        } else {
            res = mc::<BitDepth16>(
                t,
                dst,
                0 as *mut i16,
                (*f).cur.stride[0],
                bw4,
                bh4,
                (*t).bx,
                (*t).by,
                0 as c_int,
                (*b).c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .mv[0],
                refp,
                (*b).c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as c_int,
                filter_2d,
            );
            if res != 0 {
                return res;
            }
            if (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode as c_int == MM_OBMC as c_int {
                res = obmc::<BitDepth16>(
                    t,
                    dst,
                    (*f).cur.stride[0],
                    b_dim,
                    0 as c_int,
                    bx4,
                    by4,
                    w4,
                    h4,
                );
                if res != 0 {
                    return res;
                }
            }
        }
        if (*b).c2rust_unnamed.c2rust_unnamed_0.interintra_type != 0 {
            let tl_edge: *mut pixel = ((*t)
                .scratch
                .c2rust_unnamed_0
                .c2rust_unnamed_0
                .c2rust_unnamed_0
                .edge_16bpc)
                .as_mut_ptr()
                .offset(32);
            let mut m: IntraPredMode = (if (*b)
                .c2rust_unnamed
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c2rust_unnamed
                .interintra_mode as c_int
                == II_SMOOTH_PRED as c_int
            {
                SMOOTH_PRED as c_int
            } else {
                (*b).c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .interintra_mode as c_int
            }) as IntraPredMode;
            let tmp: *mut pixel = ((*t)
                .scratch
                .c2rust_unnamed_0
                .c2rust_unnamed_0
                .c2rust_unnamed_0
                .interintra_16bpc)
                .as_mut_ptr();
            let mut angle = 0;
            let mut top_sb_edge: *const pixel = 0 as *const pixel;
            if (*t).by & (*f).sb_step - 1 == 0 {
                top_sb_edge = (*f).ipred_edge[0] as *mut pixel;
                let sby = (*t).by >> (*f).sb_shift;
                top_sb_edge = top_sb_edge.offset(((*f).sb128w * 128 * (sby - 1)) as isize);
            }
            m = rav1d_prepare_intra_edges::<BitDepth16>(
                (*t).bx,
                ((*t).bx > (*ts).tiling.col_start) as c_int,
                (*t).by,
                ((*t).by > (*ts).tiling.row_start) as c_int,
                (*ts).tiling.col_end,
                (*ts).tiling.row_end,
                0 as EdgeFlags,
                dst,
                (*f).cur.stride[0],
                top_sb_edge,
                m,
                &mut angle,
                bw4,
                bh4,
                0 as c_int,
                tl_edge,
                BitDepth16::from_c((*f).bitdepth_max),
            );
            ((*dsp).ipred.intra_pred[m as usize]).expect("non-null function pointer")(
                tmp.cast(),
                ((4 * bw4) as c_ulong).wrapping_mul(::core::mem::size_of::<pixel>() as c_ulong)
                    as ptrdiff_t,
                tl_edge.cast(),
                bw4 * 4,
                bh4 * 4,
                0 as c_int,
                0 as c_int,
                0 as c_int,
                (*f).bitdepth_max,
            );
            let ii_mask = if (*b).c2rust_unnamed.c2rust_unnamed_0.interintra_type as c_int
                == INTER_INTRA_BLEND as c_int
            {
                dav1d_ii_masks[bs as usize][0][(*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .interintra_mode as usize]
            } else {
                dav1d_wedge_masks[bs as usize][0][0][(*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .wedge_idx as usize]
            };
            ((*dsp).mc.blend)(
                dst.cast(),
                (*f).cur.stride[0],
                tmp.cast(),
                bw4 * 4,
                bh4 * 4,
                ii_mask.as_ptr(),
            );
        }
        if !(has_chroma == 0) {
            is_sub8x8 = (bw4 == ss_hor || bh4 == ss_ver) as c_int;
            r = 0 as *const *mut refmvs_block;
            if is_sub8x8 != 0 {
                if !(ss_hor == 1) {
                    unreachable!();
                }
                r = &mut *((*t).rt.r)
                    .as_mut_ptr()
                    .offset((((*t).by & 31) + 5) as isize)
                    as *mut *mut refmvs_block;
                if bw4 == 1 {
                    is_sub8x8 &= ((*(*r.offset(0)).offset(((*t).bx - 1) as isize))
                        .0
                        .r#ref
                        .r#ref[0] as c_int
                        > 0) as c_int;
                }
                if bh4 == ss_ver {
                    is_sub8x8 &= ((*(*r.offset(-(1 as c_int) as isize)).offset((*t).bx as isize))
                        .0
                        .r#ref
                        .r#ref[0] as c_int
                        > 0) as c_int;
                }
                if bw4 == 1 && bh4 == ss_ver {
                    is_sub8x8 &= ((*(*r.offset(-(1 as c_int) as isize))
                        .offset(((*t).bx - 1) as isize))
                    .0
                    .r#ref
                    .r#ref[0] as c_int
                        > 0) as c_int;
                }
            }
            if is_sub8x8 != 0 {
                if !(ss_hor == 1) {
                    unreachable!();
                }
                let mut h_off: ptrdiff_t = 0 as c_int as ptrdiff_t;
                let mut v_off: ptrdiff_t = 0 as c_int as ptrdiff_t;
                if bw4 == 1 && bh4 == ss_ver {
                    let mut pl_0 = 0;
                    while pl_0 < 2 {
                        res = mc::<BitDepth16>(
                            t,
                            ((*f).cur.data[(1 + pl_0) as usize] as *mut pixel)
                                .offset(uvdstoff as isize),
                            0 as *mut i16,
                            (*f).cur.stride[1],
                            bw4,
                            bh4,
                            (*t).bx - 1,
                            (*t).by - 1,
                            1 + pl_0,
                            (*(*r.offset(-(1 as c_int) as isize)).offset(((*t).bx - 1) as isize))
                                .0
                                .mv
                                .mv[0],
                            &*((*f).refp).as_ptr().offset(
                                (*((*(*r.offset(-(1 as c_int) as isize))
                                    .offset(((*t).bx - 1) as isize))
                                .0
                                .r#ref
                                .r#ref)
                                    .as_mut_ptr()
                                    .offset(0) as c_int
                                    - 1) as isize,
                            ),
                            (*(*r.offset(-(1 as c_int) as isize)).offset(((*t).bx - 1) as isize))
                                .0
                                .r#ref
                                .r#ref[0] as c_int
                                - 1,
                            (if (*t).frame_thread.pass != 2 as c_int {
                                (*t).tl_4x4_filter as c_uint
                            } else {
                                (*((*f).frame_thread.b).offset(
                                    (((*t).by - 1) as isize * (*f).b4_stride + (*t).bx as isize - 1)
                                        as isize,
                                ))
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .filter2d as c_uint
                            }) as Filter2d,
                        );
                        if res != 0 {
                            return res;
                        }
                        pl_0 += 1;
                    }
                    v_off = 2 * PXSTRIDE((*f).cur.stride[1]);
                    h_off = 2 as c_int as ptrdiff_t;
                }
                if bw4 == 1 {
                    let left_filter_2d: Filter2d = dav1d_filter_2d
                        [(*t).l.filter[1][by4 as usize] as usize]
                        [(*t).l.filter[0][by4 as usize] as usize]
                        as Filter2d;
                    let mut pl_1 = 0;
                    while pl_1 < 2 {
                        res = mc::<BitDepth16>(
                            t,
                            ((*f).cur.data[(1 + pl_1) as usize] as *mut pixel)
                                .offset(uvdstoff as isize)
                                .offset(v_off as isize),
                            0 as *mut i16,
                            (*f).cur.stride[1],
                            bw4,
                            bh4,
                            (*t).bx - 1,
                            (*t).by,
                            1 + pl_1,
                            (*(*r.offset(0)).offset(((*t).bx - 1) as isize)).0.mv.mv[0],
                            &*((*f).refp).as_ptr().offset(
                                (*((*(*r.offset(0)).offset(((*t).bx - 1) as isize))
                                    .0
                                    .r#ref
                                    .r#ref)
                                    .as_mut_ptr()
                                    .offset(0) as c_int
                                    - 1) as isize,
                            ),
                            (*(*r.offset(0)).offset(((*t).bx - 1) as isize))
                                .0
                                .r#ref
                                .r#ref[0] as c_int
                                - 1,
                            (if (*t).frame_thread.pass != 2 as c_int {
                                left_filter_2d as c_uint
                            } else {
                                (*((*f).frame_thread.b).offset(
                                    ((*t).by as isize * (*f).b4_stride + (*t).bx as isize - 1)
                                        as isize,
                                ))
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .filter2d as c_uint
                            }) as Filter2d,
                        );
                        if res != 0 {
                            return res;
                        }
                        pl_1 += 1;
                    }
                    h_off = 2 as c_int as ptrdiff_t;
                }
                if bh4 == ss_ver {
                    let top_filter_2d: Filter2d = dav1d_filter_2d
                        [(*(*t).a).filter[1][bx4 as usize] as usize]
                        [(*(*t).a).filter[0][bx4 as usize] as usize]
                        as Filter2d;
                    let mut pl_2 = 0;
                    while pl_2 < 2 {
                        res = mc::<BitDepth16>(
                            t,
                            ((*f).cur.data[(1 + pl_2) as usize] as *mut pixel)
                                .offset(uvdstoff as isize)
                                .offset(h_off as isize),
                            0 as *mut i16,
                            (*f).cur.stride[1],
                            bw4,
                            bh4,
                            (*t).bx,
                            (*t).by - 1,
                            1 + pl_2,
                            (*(*r.offset(-(1 as c_int) as isize)).offset((*t).bx as isize))
                                .0
                                .mv
                                .mv[0],
                            &*((*f).refp).as_ptr().offset(
                                (*((*(*r.offset(-(1 as c_int) as isize)).offset((*t).bx as isize))
                                    .0
                                    .r#ref
                                    .r#ref)
                                    .as_mut_ptr()
                                    .offset(0) as c_int
                                    - 1) as isize,
                            ),
                            (*(*r.offset(-(1 as c_int) as isize)).offset((*t).bx as isize))
                                .0
                                .r#ref
                                .r#ref[0] as c_int
                                - 1,
                            (if (*t).frame_thread.pass != 2 as c_int {
                                top_filter_2d as c_uint
                            } else {
                                (*((*f).frame_thread.b).offset(
                                    (((*t).by - 1) as isize * (*f).b4_stride + (*t).bx as isize)
                                        as isize,
                                ))
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .filter2d as c_uint
                            }) as Filter2d,
                        );
                        if res != 0 {
                            return res;
                        }
                        pl_2 += 1;
                    }
                    v_off = 2 * PXSTRIDE((*f).cur.stride[1]);
                }
                let mut pl_3 = 0;
                while pl_3 < 2 {
                    res = mc::<BitDepth16>(
                        t,
                        ((*f).cur.data[(1 + pl_3) as usize] as *mut pixel)
                            .offset(uvdstoff as isize)
                            .offset(h_off as isize)
                            .offset(v_off as isize),
                        0 as *mut i16,
                        (*f).cur.stride[1],
                        bw4,
                        bh4,
                        (*t).bx,
                        (*t).by,
                        1 + pl_3,
                        (*b).c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mv[0],
                        refp,
                        (*b).c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as c_int,
                        filter_2d,
                    );
                    if res != 0 {
                        return res;
                    }
                    pl_3 += 1;
                }
            } else {
                if cmp::min(cbw4, cbh4) > 1
                    && ((*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode as c_int
                        == GLOBALMV as c_int
                        && (*f).gmv_warp_allowed
                            [(*b).c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as usize]
                            as c_int
                            != 0
                        || (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode as c_int
                            == MM_WARP as c_int
                            && (*t).warpmv.type_0 as c_uint
                                > RAV1D_WM_TYPE_TRANSLATION as c_int as c_uint)
                {
                    let mut pl_4 = 0;
                    while pl_4 < 2 {
                        res = warp_affine::<BitDepth16>(
                            t,
                            ((*f).cur.data[(1 + pl_4) as usize] as *mut pixel)
                                .offset(uvdstoff as isize),
                            0 as *mut i16,
                            (*f).cur.stride[1],
                            b_dim,
                            1 + pl_4,
                            refp,
                            if (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode as c_int
                                == MM_WARP as c_int
                            {
                                &mut (*t).warpmv
                            } else {
                                &mut *((*(*f).frame_hdr).gmv).as_mut_ptr().offset(
                                    *((*b).c2rust_unnamed.c2rust_unnamed_0.r#ref)
                                        .as_ptr()
                                        .offset(0) as isize,
                                )
                            },
                        );
                        if res != 0 {
                            return res;
                        }
                        pl_4 += 1;
                    }
                } else {
                    let mut pl_5 = 0;
                    while pl_5 < 2 {
                        res = mc::<BitDepth16>(
                            t,
                            ((*f).cur.data[(1 + pl_5) as usize] as *mut pixel)
                                .offset(uvdstoff as isize),
                            0 as *mut i16,
                            (*f).cur.stride[1],
                            bw4 << (bw4 == ss_hor) as c_int,
                            bh4 << (bh4 == ss_ver) as c_int,
                            (*t).bx & !ss_hor,
                            (*t).by & !ss_ver,
                            1 + pl_5,
                            (*b).c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .mv[0],
                            refp,
                            (*b).c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as c_int,
                            filter_2d,
                        );
                        if res != 0 {
                            return res;
                        }
                        if (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode as c_int
                            == MM_OBMC as c_int
                        {
                            res = obmc::<BitDepth16>(
                                t,
                                ((*f).cur.data[(1 + pl_5) as usize] as *mut pixel)
                                    .offset(uvdstoff as isize),
                                (*f).cur.stride[1],
                                b_dim,
                                1 + pl_5,
                                bx4,
                                by4,
                                w4,
                                h4,
                            );
                            if res != 0 {
                                return res;
                            }
                        }
                        pl_5 += 1;
                    }
                }
                if (*b).c2rust_unnamed.c2rust_unnamed_0.interintra_type != 0 {
                    let ii_mask_0 = if (*b).c2rust_unnamed.c2rust_unnamed_0.interintra_type as c_int
                        == INTER_INTRA_BLEND as c_int
                    {
                        dav1d_ii_masks[bs as usize][chr_layout_idx as usize][(*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .interintra_mode
                            as usize]
                    } else {
                        dav1d_wedge_masks[bs as usize][chr_layout_idx as usize][0][(*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .wedge_idx
                            as usize]
                    };
                    let mut pl_6 = 0;
                    while pl_6 < 2 {
                        let tmp_0: *mut pixel = ((*t)
                            .scratch
                            .c2rust_unnamed_0
                            .c2rust_unnamed_0
                            .c2rust_unnamed_0
                            .interintra_16bpc)
                            .as_mut_ptr();
                        let tl_edge_0: *mut pixel = ((*t)
                            .scratch
                            .c2rust_unnamed_0
                            .c2rust_unnamed_0
                            .c2rust_unnamed_0
                            .edge_16bpc)
                            .as_mut_ptr()
                            .offset(32);
                        let mut m_0: IntraPredMode = (if (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .interintra_mode
                            as c_int
                            == II_SMOOTH_PRED as c_int
                        {
                            SMOOTH_PRED as c_int
                        } else {
                            (*b).c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .interintra_mode as c_int
                        }) as IntraPredMode;
                        let mut angle_0 = 0;
                        let uvdst: *mut pixel = ((*f).cur.data[(1 + pl_6) as usize] as *mut pixel)
                            .offset(uvdstoff as isize);
                        let mut top_sb_edge_0: *const pixel = 0 as *const pixel;
                        if (*t).by & (*f).sb_step - 1 == 0 {
                            top_sb_edge_0 = (*f).ipred_edge[(pl_6 + 1) as usize] as *mut pixel;
                            let sby_0 = (*t).by >> (*f).sb_shift;
                            top_sb_edge_0 =
                                top_sb_edge_0.offset(((*f).sb128w * 128 * (sby_0 - 1)) as isize);
                        }
                        m_0 = rav1d_prepare_intra_edges::<BitDepth16>(
                            (*t).bx >> ss_hor,
                            ((*t).bx >> ss_hor > (*ts).tiling.col_start >> ss_hor) as c_int,
                            (*t).by >> ss_ver,
                            ((*t).by >> ss_ver > (*ts).tiling.row_start >> ss_ver) as c_int,
                            (*ts).tiling.col_end >> ss_hor,
                            (*ts).tiling.row_end >> ss_ver,
                            0 as EdgeFlags,
                            uvdst,
                            (*f).cur.stride[1],
                            top_sb_edge_0,
                            m_0,
                            &mut angle_0,
                            cbw4,
                            cbh4,
                            0 as c_int,
                            tl_edge_0,
                            BitDepth16::from_c((*f).bitdepth_max),
                        );
                        ((*dsp).ipred.intra_pred[m_0 as usize]).expect("non-null function pointer")(
                            tmp_0.cast(),
                            ((cbw4 * 4) as c_ulong)
                                .wrapping_mul(::core::mem::size_of::<pixel>() as c_ulong)
                                as ptrdiff_t,
                            tl_edge_0.cast(),
                            cbw4 * 4,
                            cbh4 * 4,
                            0 as c_int,
                            0 as c_int,
                            0 as c_int,
                            (*f).bitdepth_max,
                        );
                        ((*dsp).mc.blend)(
                            uvdst.cast(),
                            (*f).cur.stride[1],
                            tmp_0.cast(),
                            cbw4 * 4,
                            cbh4 * 4,
                            ii_mask_0.as_ptr(),
                        );
                        pl_6 += 1;
                    }
                }
            }
        }
        (*t).tl_4x4_filter = filter_2d;
    } else {
        let filter_2d_0: Filter2d = (*b).c2rust_unnamed.c2rust_unnamed_0.filter2d as Filter2d;
        let tmp_1: *mut [i16; 16384] = ((*t)
            .scratch
            .c2rust_unnamed
            .c2rust_unnamed
            .c2rust_unnamed
            .compinter)
            .as_mut_ptr();
        let mut jnt_weight = 0;
        let seg_mask: *mut u8 = ((*t)
            .scratch
            .c2rust_unnamed
            .c2rust_unnamed
            .c2rust_unnamed
            .seg_mask)
            .as_mut_ptr();
        let mut mask: *const u8 = 0 as *const u8;
        let mut i = 0;
        while i < 2 {
            let refp_0: *const Rav1dThreadPicture = &*((*f).refp).as_ptr().offset(
                *((*b).c2rust_unnamed.c2rust_unnamed_0.r#ref)
                    .as_ptr()
                    .offset(i as isize) as isize,
            ) as *const Rav1dThreadPicture;
            if (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode as c_int
                == GLOBALMV_GLOBALMV as c_int
                && (*f).gmv_warp_allowed
                    [(*b).c2rust_unnamed.c2rust_unnamed_0.r#ref[i as usize] as usize]
                    as c_int
                    != 0
            {
                res = warp_affine::<BitDepth16>(
                    t,
                    0 as *mut pixel,
                    (*tmp_1.offset(i as isize)).as_mut_ptr(),
                    (bw4 * 4) as ptrdiff_t,
                    b_dim,
                    0 as c_int,
                    refp_0,
                    &mut *((*(*f).frame_hdr).gmv).as_mut_ptr().offset(
                        *((*b).c2rust_unnamed.c2rust_unnamed_0.r#ref)
                            .as_ptr()
                            .offset(i as isize) as isize,
                    ),
                );
                if res != 0 {
                    return res;
                }
            } else {
                res = mc::<BitDepth16>(
                    t,
                    0 as *mut pixel,
                    (*tmp_1.offset(i as isize)).as_mut_ptr(),
                    0 as c_int as ptrdiff_t,
                    bw4,
                    bh4,
                    (*t).bx,
                    (*t).by,
                    0 as c_int,
                    (*b).c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[i as usize],
                    refp_0,
                    (*b).c2rust_unnamed.c2rust_unnamed_0.r#ref[i as usize] as c_int,
                    filter_2d_0,
                );
                if res != 0 {
                    return res;
                }
            }
            i += 1;
        }
        match (*b).c2rust_unnamed.c2rust_unnamed_0.comp_type as c_int {
            2 => {
                ((*dsp).mc.avg)(
                    dst.cast(),
                    (*f).cur.stride[0],
                    (*tmp_1.offset(0)).as_mut_ptr(),
                    (*tmp_1.offset(1)).as_mut_ptr(),
                    bw4 * 4,
                    bh4 * 4,
                    (*f).bitdepth_max,
                );
            }
            1 => {
                jnt_weight = (*f).jnt_weights
                    [(*b).c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as usize]
                    [(*b).c2rust_unnamed.c2rust_unnamed_0.r#ref[1] as usize]
                    as c_int;
                ((*dsp).mc.w_avg)(
                    dst.cast(),
                    (*f).cur.stride[0],
                    (*tmp_1.offset(0)).as_mut_ptr(),
                    (*tmp_1.offset(1)).as_mut_ptr(),
                    bw4 * 4,
                    bh4 * 4,
                    jnt_weight,
                    (*f).bitdepth_max,
                );
            }
            3 => {
                (*dsp).mc.w_mask[chr_layout_idx as usize](
                    dst.cast(),
                    (*f).cur.stride[0],
                    (*tmp_1.offset(
                        (*b).c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mask_sign as isize,
                    ))
                    .as_mut_ptr(),
                    (*tmp_1.offset(
                        ((*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mask_sign
                            == 0) as c_int as isize,
                    ))
                    .as_mut_ptr(),
                    bw4 * 4,
                    bh4 * 4,
                    seg_mask,
                    (*b).c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mask_sign as c_int,
                    (*f).bitdepth_max,
                );
                mask = seg_mask;
            }
            4 => {
                mask = dav1d_wedge_masks[bs as usize][0][0][(*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .wedge_idx
                    as usize]
                    .as_ptr();
                ((*dsp).mc.mask)(
                    dst.cast(),
                    (*f).cur.stride[0],
                    (*tmp_1.offset(
                        (*b).c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mask_sign as isize,
                    ))
                    .as_mut_ptr(),
                    (*tmp_1.offset(
                        ((*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mask_sign
                            == 0) as c_int as isize,
                    ))
                    .as_mut_ptr(),
                    bw4 * 4,
                    bh4 * 4,
                    mask,
                    (*f).bitdepth_max,
                );
                if has_chroma != 0 {
                    mask = dav1d_wedge_masks[bs as usize][chr_layout_idx as usize][(*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mask_sign
                        as usize][(*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .wedge_idx as usize]
                        .as_ptr();
                }
            }
            _ => {}
        }
        if has_chroma != 0 {
            let mut pl_7 = 0;
            while pl_7 < 2 {
                let mut i_0 = 0;
                while i_0 < 2 {
                    let refp_1: *const Rav1dThreadPicture = &*((*f).refp).as_ptr().offset(
                        *((*b).c2rust_unnamed.c2rust_unnamed_0.r#ref)
                            .as_ptr()
                            .offset(i_0 as isize) as isize,
                    )
                        as *const Rav1dThreadPicture;
                    if (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode as c_int
                        == GLOBALMV_GLOBALMV as c_int
                        && cmp::min(cbw4, cbh4) > 1
                        && (*f).gmv_warp_allowed
                            [(*b).c2rust_unnamed.c2rust_unnamed_0.r#ref[i_0 as usize] as usize]
                            as c_int
                            != 0
                    {
                        res = warp_affine::<BitDepth16>(
                            t,
                            0 as *mut pixel,
                            (*tmp_1.offset(i_0 as isize)).as_mut_ptr(),
                            (bw4 * 4 >> ss_hor) as ptrdiff_t,
                            b_dim,
                            1 + pl_7,
                            refp_1,
                            &mut *((*(*f).frame_hdr).gmv).as_mut_ptr().offset(
                                *((*b).c2rust_unnamed.c2rust_unnamed_0.r#ref)
                                    .as_ptr()
                                    .offset(i_0 as isize) as isize,
                            ),
                        );
                        if res != 0 {
                            return res;
                        }
                    } else {
                        res = mc::<BitDepth16>(
                            t,
                            0 as *mut pixel,
                            (*tmp_1.offset(i_0 as isize)).as_mut_ptr(),
                            0 as c_int as ptrdiff_t,
                            bw4,
                            bh4,
                            (*t).bx,
                            (*t).by,
                            1 + pl_7,
                            (*b).c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .mv[i_0 as usize],
                            refp_1,
                            (*b).c2rust_unnamed.c2rust_unnamed_0.r#ref[i_0 as usize] as c_int,
                            filter_2d_0,
                        );
                        if res != 0 {
                            return res;
                        }
                    }
                    i_0 += 1;
                }
                let uvdst_0: *mut pixel =
                    ((*f).cur.data[(1 + pl_7) as usize] as *mut pixel).offset(uvdstoff as isize);
                match (*b).c2rust_unnamed.c2rust_unnamed_0.comp_type as c_int {
                    2 => {
                        ((*dsp).mc.avg)(
                            uvdst_0.cast(),
                            (*f).cur.stride[1],
                            (*tmp_1.offset(0)).as_mut_ptr(),
                            (*tmp_1.offset(1)).as_mut_ptr(),
                            bw4 * 4 >> ss_hor,
                            bh4 * 4 >> ss_ver,
                            (*f).bitdepth_max,
                        );
                    }
                    1 => {
                        ((*dsp).mc.w_avg)(
                            uvdst_0.cast(),
                            (*f).cur.stride[1],
                            (*tmp_1.offset(0)).as_mut_ptr(),
                            (*tmp_1.offset(1)).as_mut_ptr(),
                            bw4 * 4 >> ss_hor,
                            bh4 * 4 >> ss_ver,
                            jnt_weight,
                            (*f).bitdepth_max,
                        );
                    }
                    4 | 3 => {
                        ((*dsp).mc.mask)(
                            uvdst_0.cast(),
                            (*f).cur.stride[1],
                            (*tmp_1.offset(
                                (*b).c2rust_unnamed
                                    .c2rust_unnamed_0
                                    .c2rust_unnamed
                                    .c2rust_unnamed
                                    .mask_sign as isize,
                            ))
                            .as_mut_ptr(),
                            (*tmp_1.offset(
                                ((*b)
                                    .c2rust_unnamed
                                    .c2rust_unnamed_0
                                    .c2rust_unnamed
                                    .c2rust_unnamed
                                    .mask_sign
                                    == 0) as c_int as isize,
                            ))
                            .as_mut_ptr(),
                            bw4 * 4 >> ss_hor,
                            bh4 * 4 >> ss_ver,
                            mask,
                            (*f).bitdepth_max,
                        );
                    }
                    _ => {}
                }
                pl_7 += 1;
            }
        }
    }
    if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
        hex_dump::<BitDepth16>(
            dst,
            (*f).cur.stride[0] as usize,
            *b_dim.offset(0) as usize * 4,
            *b_dim.offset(1) as usize * 4,
            "y-pred",
        );
        if has_chroma != 0 {
            hex_dump::<BitDepth16>(
                &mut *(*((*f).cur.data).as_ptr().offset(1) as *mut pixel).offset(uvdstoff as isize),
                (*f).cur.stride[1] as usize,
                cbw4 as usize * 4,
                cbh4 as usize * 4,
                "u-pred",
            );
            hex_dump::<BitDepth16>(
                &mut *(*((*f).cur.data).as_ptr().offset(2) as *mut pixel).offset(uvdstoff as isize),
                (*f).cur.stride[1] as usize,
                cbw4 as usize * 4,
                cbh4 as usize * 4,
                "v-pred",
            );
        }
    }
    let cw4 = w4 + ss_hor >> ss_hor;
    let ch4 = h4 + ss_ver >> ss_ver;
    if (*b).skip != 0 {
        CaseSet::<32, false>::many(
            [&mut (*t).l, &mut *(*t).a],
            [bh4 as usize, bw4 as usize],
            [by4 as usize, bx4 as usize],
            |case, dir| {
                case.set(&mut dir.lcoef.0, 0x40);
            },
        );
        if has_chroma != 0 {
            CaseSet::<32, false>::many(
                [&mut (*t).l, &mut *(*t).a],
                [cbh4 as usize, cbw4 as usize],
                [cby4 as usize, cbx4 as usize],
                |case, dir| {
                    case.set(&mut dir.ccoef.0[0], 0x40);
                    case.set(&mut dir.ccoef.0[1], 0x40);
                },
            );
        }
        return 0 as c_int;
    }
    let uvtx: *const TxfmInfo =
        &*dav1d_txfm_dimensions.as_ptr().offset((*b).uvtx as isize) as *const TxfmInfo;
    let ytx: *const TxfmInfo = &*dav1d_txfm_dimensions
        .as_ptr()
        .offset((*b).c2rust_unnamed.c2rust_unnamed_0.max_ytx as isize)
        as *const TxfmInfo;
    let tx_split: [u16; 2] = [
        (*b).c2rust_unnamed.c2rust_unnamed_0.tx_split0 as u16,
        (*b).c2rust_unnamed.c2rust_unnamed_0.tx_split1,
    ];
    let mut init_y = 0;
    while init_y < bh4 {
        let mut init_x = 0;
        while init_x < bw4 {
            let mut y_off = (init_y != 0) as c_int;
            let mut y;
            dst = dst.offset((PXSTRIDE((*f).cur.stride[0]) * 4 * init_y as isize) as isize);
            y = init_y;
            (*t).by += init_y;
            while y < cmp::min(h4, init_y + 16) {
                let mut x;
                let mut x_off = (init_x != 0) as c_int;
                x = init_x;
                (*t).bx += init_x;
                while x < cmp::min(w4, init_x + 16) {
                    read_coef_tree::<BitDepth16>(
                        t,
                        bs,
                        b,
                        (*b).c2rust_unnamed.c2rust_unnamed_0.max_ytx as RectTxfmSize,
                        0 as c_int,
                        tx_split.as_ptr(),
                        x_off,
                        y_off,
                        &mut *dst.offset((x * 4) as isize),
                    );
                    (*t).bx += (*ytx).w as c_int;
                    x += (*ytx).w as c_int;
                    x_off += 1;
                }
                dst = dst.offset((PXSTRIDE((*f).cur.stride[0]) * 4 * (*ytx).h as isize) as isize);
                (*t).bx -= x;
                (*t).by += (*ytx).h as c_int;
                y += (*ytx).h as c_int;
                y_off += 1;
            }
            dst = dst.offset(-((PXSTRIDE((*f).cur.stride[0]) * 4 * y as isize) as isize));
            (*t).by -= y;
            if has_chroma != 0 {
                let mut pl_8 = 0;
                while pl_8 < 2 {
                    let mut uvdst_1: *mut pixel = ((*f).cur.data[(1 + pl_8) as usize]
                        as *mut pixel)
                        .offset(uvdstoff as isize)
                        .offset(
                            (PXSTRIDE((*f).cur.stride[1]) * init_y as isize * 4 >> ss_ver) as isize,
                        );
                    y = init_y >> ss_ver;
                    (*t).by += init_y;
                    while y < cmp::min(ch4, init_y + 16 >> ss_ver) {
                        let mut x_0;
                        x_0 = init_x >> ss_hor;
                        (*t).bx += init_x;
                        while x_0 < cmp::min(cw4, init_x + 16 >> ss_hor) {
                            let cf: *mut coef;
                            let eob;
                            let mut txtp: TxfmType;
                            if (*t).frame_thread.pass != 0 {
                                let p = (*t).frame_thread.pass & 1;
                                cf = (*ts).frame_thread[p as usize].cf as *mut coef;
                                (*ts).frame_thread[p as usize].cf =
                                    ((*ts).frame_thread[p as usize].cf as *mut coef).offset(
                                        ((*uvtx).w as c_int * (*uvtx).h as c_int * 16) as isize,
                                    ) as *mut DynCoef;
                                let cbi: *const CodedBlockInfo =
                                    &mut *((*f).frame_thread.cbi).offset(
                                        ((*t).by as isize * (*f).b4_stride + (*t).bx as isize)
                                            as isize,
                                    ) as *mut CodedBlockInfo;
                                eob = (*cbi).eob[(1 + pl_8) as usize] as c_int;
                                txtp = (*cbi).txtp[(1 + pl_8) as usize] as TxfmType;
                            } else {
                                let mut cf_ctx: u8 = 0;
                                cf = ((*t).c2rust_unnamed.cf_16bpc).as_mut_ptr();
                                txtp = (*t).txtp_map
                                    [((by4 + (y << ss_ver)) * 32 + bx4 + (x_0 << ss_hor)) as usize]
                                    as TxfmType;
                                eob = decode_coefs::<BitDepth16>(
                                    t,
                                    &mut (*(*t).a).ccoef.0[pl_8 as usize][(cbx4 + x_0) as usize..],
                                    &mut (*t).l.ccoef.0[pl_8 as usize][(cby4 + y) as usize..],
                                    (*b).uvtx as RectTxfmSize,
                                    bs,
                                    b,
                                    0 as c_int,
                                    1 + pl_8,
                                    cf,
                                    &mut txtp,
                                    &mut cf_ctx,
                                );
                                if DEBUG_BLOCK_INFO(&*f, &*t) {
                                    printf(
                                        b"Post-uv-cf-blk[pl=%d,tx=%d,txtp=%d,eob=%d]: r=%d\n\0"
                                            as *const u8
                                            as *const c_char,
                                        pl_8,
                                        (*b).uvtx as c_int,
                                        txtp as c_uint,
                                        eob,
                                        (*ts).msac.rng,
                                    );
                                }
                                CaseSet::<16, true>::many(
                                    [&mut (*t).l, &mut *(*t).a],
                                    [
                                        cmp::min(
                                            (*uvtx).h as i32,
                                            (*f).bh - (*t).by + ss_ver >> ss_ver,
                                        ) as usize,
                                        cmp::min(
                                            (*uvtx).w as i32,
                                            (*f).bw - (*t).bx + ss_hor >> ss_hor,
                                        ) as usize,
                                    ],
                                    [(cby4 + y) as usize, (cbx4 + x_0) as usize],
                                    |case, dir| {
                                        case.set(&mut dir.ccoef.0[pl_8 as usize], cf_ctx);
                                    },
                                );
                            }
                            if eob >= 0 {
                                if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                                    coef_dump(
                                        cf,
                                        (*uvtx).h as usize * 4,
                                        (*uvtx).w as usize * 4,
                                        3,
                                        "dq",
                                    );
                                }
                                ((*dsp).itx.itxfm_add[(*b).uvtx as usize][txtp as usize])
                                    .expect("non-null function pointer")(
                                    uvdst_1.offset((4 * x_0) as isize).cast(),
                                    (*f).cur.stride[1],
                                    cf.cast(),
                                    eob,
                                    (*f).bitdepth_max,
                                );
                                if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                                    hex_dump::<BitDepth16>(
                                        &mut *uvdst_1.offset((4 * x_0) as isize),
                                        (*f).cur.stride[1] as usize,
                                        (*uvtx).w as usize * 4,
                                        (*uvtx).h as usize * 4,
                                        "recon",
                                    );
                                }
                            }
                            (*t).bx += ((*uvtx).w as c_int) << ss_hor;
                            x_0 += (*uvtx).w as c_int;
                        }
                        uvdst_1 = uvdst_1.offset(
                            (PXSTRIDE((*f).cur.stride[1]) * 4 * (*uvtx).h as isize) as isize,
                        );
                        (*t).bx -= x_0 << ss_hor;
                        (*t).by += ((*uvtx).h as c_int) << ss_ver;
                        y += (*uvtx).h as c_int;
                    }
                    (*t).by -= y << ss_ver;
                    pl_8 += 1;
                }
            }
            init_x += 16 as c_int;
        }
        init_y += 16 as c_int;
    }
    return 0 as c_int;
}

pub(crate) unsafe extern "C" fn rav1d_filter_sbrow_deblock_cols_16bpc(
    f: *mut Rav1dFrameContext,
    sby: c_int,
) {
    if (*(*f).c).inloop_filters as c_uint & RAV1D_INLOOPFILTER_DEBLOCK as c_int as c_uint == 0
        || (*(*f).frame_hdr).loopfilter.level_y[0] == 0
            && (*(*f).frame_hdr).loopfilter.level_y[1] == 0
    {
        return;
    }
    let y = sby * (*f).sb_step * 4;
    let ss_ver =
        ((*f).cur.p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint) as c_int;
    let p: [*mut pixel; 3] = [
        ((*f).lf.p[0] as *mut pixel).offset((y as isize * PXSTRIDE((*f).cur.stride[0])) as isize),
        ((*f).lf.p[1] as *mut pixel)
            .offset((y as isize * PXSTRIDE((*f).cur.stride[1]) >> ss_ver) as isize),
        ((*f).lf.p[2] as *mut pixel)
            .offset((y as isize * PXSTRIDE((*f).cur.stride[1]) >> ss_ver) as isize),
    ];
    let mask: *mut Av1Filter = ((*f).lf.mask)
        .offset(((sby >> ((*(*f).seq_hdr).sb128 == 0) as c_int) * (*f).sb128w) as isize);
    rav1d_loopfilter_sbrow_cols_16bpc(
        f,
        p.as_ptr(),
        mask,
        sby,
        *((*f).lf.start_of_tile_row).offset(sby as isize) as c_int,
    );
}

pub(crate) unsafe extern "C" fn rav1d_filter_sbrow_deblock_rows_16bpc(
    f: *mut Rav1dFrameContext,
    sby: c_int,
) {
    let y = sby * (*f).sb_step * 4;
    let ss_ver =
        ((*f).cur.p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint) as c_int;
    let p: [*mut pixel; 3] = [
        ((*f).lf.p[0] as *mut pixel).offset((y as isize * PXSTRIDE((*f).cur.stride[0])) as isize),
        ((*f).lf.p[1] as *mut pixel)
            .offset((y as isize * PXSTRIDE((*f).cur.stride[1]) >> ss_ver) as isize),
        ((*f).lf.p[2] as *mut pixel)
            .offset((y as isize * PXSTRIDE((*f).cur.stride[1]) >> ss_ver) as isize),
    ];
    let mask: *mut Av1Filter = ((*f).lf.mask)
        .offset(((sby >> ((*(*f).seq_hdr).sb128 == 0) as c_int) * (*f).sb128w) as isize);
    if (*(*f).c).inloop_filters as c_uint & RAV1D_INLOOPFILTER_DEBLOCK as c_int as c_uint != 0
        && ((*(*f).frame_hdr).loopfilter.level_y[0] != 0
            || (*(*f).frame_hdr).loopfilter.level_y[1] != 0)
    {
        rav1d_loopfilter_sbrow_rows_16bpc(f, p.as_ptr(), mask, sby);
    }
    if (*(*f).seq_hdr).cdef != 0 || (*f).lf.restore_planes != 0 {
        rav1d_copy_lpf_16bpc(f, p.as_ptr(), sby);
    }
}

pub(crate) unsafe extern "C" fn rav1d_filter_sbrow_cdef_16bpc(
    tc: *mut Rav1dTaskContext,
    sby: c_int,
) {
    let f: *const Rav1dFrameContext = (*tc).f;
    if (*(*f).c).inloop_filters as c_uint & RAV1D_INLOOPFILTER_CDEF as c_int as c_uint == 0 {
        return;
    }
    let sbsz = (*f).sb_step;
    let y = sby * sbsz * 4;
    let ss_ver =
        ((*f).cur.p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint) as c_int;
    let p: [*mut pixel; 3] = [
        ((*f).lf.p[0] as *mut pixel).offset((y as isize * PXSTRIDE((*f).cur.stride[0])) as isize),
        ((*f).lf.p[1] as *mut pixel)
            .offset((y as isize * PXSTRIDE((*f).cur.stride[1]) >> ss_ver) as isize),
        ((*f).lf.p[2] as *mut pixel)
            .offset((y as isize * PXSTRIDE((*f).cur.stride[1]) >> ss_ver) as isize),
    ];
    let prev_mask: *mut Av1Filter = ((*f).lf.mask)
        .offset(((sby - 1 >> ((*(*f).seq_hdr).sb128 == 0) as c_int) * (*f).sb128w) as isize);
    let mask: *mut Av1Filter = ((*f).lf.mask)
        .offset(((sby >> ((*(*f).seq_hdr).sb128 == 0) as c_int) * (*f).sb128w) as isize);
    let start = sby * sbsz;
    if sby != 0 {
        let ss_ver_0 =
            ((*f).cur.p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint) as c_int;
        let mut p_up: [*mut pixel; 3] = [
            (p[0]).offset(-((8 * PXSTRIDE((*f).cur.stride[0])) as isize)),
            (p[1]).offset(-((8 * PXSTRIDE((*f).cur.stride[1]) >> ss_ver_0) as isize)),
            (p[2]).offset(-((8 * PXSTRIDE((*f).cur.stride[1]) >> ss_ver_0) as isize)),
        ];
        rav1d_cdef_brow_16bpc(
            tc,
            p_up.as_mut_ptr() as *const *mut pixel,
            prev_mask,
            start - 2,
            start,
            1 as c_int,
            sby,
        );
    }
    let n_blks = sbsz - 2 * ((sby + 1) < (*f).sbh) as c_int;
    let end = cmp::min(start + n_blks, (*f).bh);
    rav1d_cdef_brow_16bpc(tc, p.as_ptr(), mask, start, end, 0 as c_int, sby);
}

pub(crate) unsafe extern "C" fn rav1d_filter_sbrow_resize_16bpc(
    f: *mut Rav1dFrameContext,
    sby: c_int,
) {
    let sbsz = (*f).sb_step;
    let y = sby * sbsz * 4;
    let ss_ver =
        ((*f).cur.p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint) as c_int;
    let p: [*const pixel; 3] = [
        ((*f).lf.p[0] as *mut pixel).offset((y as isize * PXSTRIDE((*f).cur.stride[0])) as isize)
            as *const pixel,
        ((*f).lf.p[1] as *mut pixel)
            .offset((y as isize * PXSTRIDE((*f).cur.stride[1]) >> ss_ver) as isize)
            as *const pixel,
        ((*f).lf.p[2] as *mut pixel)
            .offset((y as isize * PXSTRIDE((*f).cur.stride[1]) >> ss_ver) as isize)
            as *const pixel,
    ];
    let sr_p: [*mut pixel; 3] = [
        ((*f).lf.sr_p[0] as *mut pixel)
            .offset((y as isize * PXSTRIDE((*f).sr_cur.p.stride[0])) as isize),
        ((*f).lf.sr_p[1] as *mut pixel)
            .offset((y as isize * PXSTRIDE((*f).sr_cur.p.stride[1]) >> ss_ver) as isize),
        ((*f).lf.sr_p[2] as *mut pixel)
            .offset((y as isize * PXSTRIDE((*f).sr_cur.p.stride[1]) >> ss_ver) as isize),
    ];
    let has_chroma =
        ((*f).cur.p.layout as c_uint != RAV1D_PIXEL_LAYOUT_I400 as c_int as c_uint) as c_int;
    let mut pl = 0;
    while pl < 1 + 2 * has_chroma {
        let ss_ver_0 = (pl != 0
            && (*f).cur.p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint)
            as c_int;
        let h_start = 8 * (sby != 0) as c_int >> ss_ver_0;
        let dst_stride: ptrdiff_t = (*f).sr_cur.p.stride[(pl != 0) as c_int as usize];
        let dst: *mut pixel =
            (sr_p[pl as usize]).offset(-((h_start as isize * PXSTRIDE(dst_stride)) as isize));
        let src_stride: ptrdiff_t = (*f).cur.stride[(pl != 0) as c_int as usize];
        let src: *const pixel = (p[pl as usize]).offset(-(h_start as isize * PXSTRIDE(src_stride)));
        let h_end = 4 * (sbsz - 2 * ((sby + 1) < (*f).sbh) as c_int) >> ss_ver_0;
        let ss_hor = (pl != 0
            && (*f).cur.p.layout as c_uint != RAV1D_PIXEL_LAYOUT_I444 as c_int as c_uint)
            as c_int;
        let dst_w = (*f).sr_cur.p.p.w + ss_hor >> ss_hor;
        let src_w = 4 * (*f).bw + ss_hor >> ss_hor;
        let img_h = (*f).cur.p.h - sbsz * 4 * sby + ss_ver_0 >> ss_ver_0;
        ((*(*f).dsp).mc.resize)(
            dst.cast(),
            dst_stride,
            src.cast(),
            src_stride,
            dst_w,
            cmp::min(img_h, h_end) + h_start,
            src_w,
            (*f).resize_step[(pl != 0) as c_int as usize],
            (*f).resize_start[(pl != 0) as c_int as usize],
            (*f).bitdepth_max,
        );
        pl += 1;
    }
}

pub(crate) unsafe extern "C" fn rav1d_filter_sbrow_lr_16bpc(f: *mut Rav1dFrameContext, sby: c_int) {
    if (*(*f).c).inloop_filters as c_uint & RAV1D_INLOOPFILTER_RESTORATION as c_int as c_uint == 0 {
        return;
    }
    let y = sby * (*f).sb_step * 4;
    let ss_ver =
        ((*f).cur.p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint) as c_int;
    let sr_p: [*mut pixel; 3] = [
        ((*f).lf.sr_p[0] as *mut pixel).offset(y as isize * PXSTRIDE((*f).sr_cur.p.stride[0])),
        ((*f).lf.sr_p[1] as *mut pixel)
            .offset(y as isize * PXSTRIDE((*f).sr_cur.p.stride[1]) >> ss_ver),
        ((*f).lf.sr_p[2] as *mut pixel)
            .offset(y as isize * PXSTRIDE((*f).sr_cur.p.stride[1]) >> ss_ver),
    ];
    rav1d_lr_sbrow_16bpc(f, sr_p.as_ptr(), sby);
}

pub(crate) unsafe extern "C" fn rav1d_filter_sbrow_16bpc(f: *mut Rav1dFrameContext, sby: c_int) {
    rav1d_filter_sbrow_deblock_cols_16bpc(f, sby);
    rav1d_filter_sbrow_deblock_rows_16bpc(f, sby);
    if (*(*f).seq_hdr).cdef != 0 {
        rav1d_filter_sbrow_cdef_16bpc((*(*f).c).tc, sby);
    }
    if (*(*f).frame_hdr).width[0] != (*(*f).frame_hdr).width[1] {
        rav1d_filter_sbrow_resize_16bpc(f, sby);
    }
    if (*f).lf.restore_planes != 0 {
        rav1d_filter_sbrow_lr_16bpc(f, sby);
    }
}

pub(crate) unsafe extern "C" fn rav1d_backup_ipred_edge_16bpc(t: *mut Rav1dTaskContext) {
    let f: *const Rav1dFrameContext = (*t).f;
    let ts: *mut Rav1dTileState = (*t).ts;
    let sby = (*t).by >> (*f).sb_shift;
    let sby_off = (*f).sb128w * 128 * sby;
    let x_off = (*ts).tiling.col_start;
    let y: *const pixel = ((*f).cur.data[0] as *const pixel)
        .offset((x_off * 4) as isize)
        .offset(
            ((((*t).by + (*f).sb_step) * 4 - 1) as isize * PXSTRIDE((*f).cur.stride[0])) as isize,
        );
    memcpy(
        &mut *(*((*f).ipred_edge).as_ptr().offset(0) as *mut pixel)
            .offset((sby_off + x_off * 4) as isize) as *mut pixel as *mut c_void,
        y as *const c_void,
        (4 * ((*ts).tiling.col_end - x_off) << 1) as usize,
    );
    if (*f).cur.p.layout as c_uint != RAV1D_PIXEL_LAYOUT_I400 as c_int as c_uint {
        let ss_ver =
            ((*f).cur.p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint) as c_int;
        let ss_hor =
            ((*f).cur.p.layout as c_uint != RAV1D_PIXEL_LAYOUT_I444 as c_int as c_uint) as c_int;
        let uv_off: ptrdiff_t = (x_off * 4 >> ss_hor) as isize
            + ((((*t).by + (*f).sb_step) * 4 >> ss_ver) - 1) as isize
                * PXSTRIDE((*f).cur.stride[1]);
        let mut pl = 1;
        while pl <= 2 {
            memcpy(
                &mut *(*((*f).ipred_edge).as_ptr().offset(pl as isize) as *mut pixel)
                    .offset((sby_off + (x_off * 4 >> ss_hor)) as isize)
                    as *mut pixel as *mut c_void,
                &*(*((*f).cur.data).as_ptr().offset(pl as isize) as *const pixel)
                    .offset(uv_off as isize) as *const pixel as *const c_void,
                (4 * ((*ts).tiling.col_end - x_off) >> ss_hor << 1) as usize,
            );
            pl += 1;
        }
    }
}
