use crate::include::common::bitdepth::BitDepth;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::include::dav1d::headers::RAV1D_RESTORATION_NONE;
use crate::include::dav1d::headers::RAV1D_RESTORATION_SGRPROJ;
use crate::include::dav1d::headers::RAV1D_RESTORATION_WIENER;
use crate::src::align::Align16;
use crate::src::internal::Rav1dContext;
use crate::src::internal::Rav1dDSPContext;
use crate::src::internal::Rav1dFrameData;
use crate::src::lf_mask::Av1RestorationUnit;
use crate::src::looprestoration::looprestorationfilter_fn;
use crate::src::looprestoration::LooprestorationParams;
use crate::src::looprestoration::LrEdgeFlags;
use crate::src::looprestoration::LR_HAVE_BOTTOM;
use crate::src::looprestoration::LR_HAVE_LEFT;
use crate::src::looprestoration::LR_HAVE_RIGHT;
use crate::src::looprestoration::LR_HAVE_TOP;
use crate::src::tables::dav1d_sgr_params;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::slice;

pub type LrRestorePlanes = c_uint;
pub const LR_RESTORE_V: LrRestorePlanes = 4;
pub const LR_RESTORE_U: LrRestorePlanes = 2;
pub const LR_RESTORE_Y: LrRestorePlanes = 1;

unsafe fn lr_stripe<BD: BitDepth>(
    c: &Rav1dContext,
    f: &Rav1dFrameData,
    mut p: *mut BD::Pixel,
    mut left: *const [BD::Pixel; 4],
    x: c_int,
    mut y: c_int,
    plane: c_int,
    unit_w: c_int,
    row_h: c_int,
    lr: Av1RestorationUnit,
    mut edges: LrEdgeFlags,
) {
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    let dsp: *const Rav1dDSPContext = f.dsp;
    let chroma = (plane != 0) as c_int;
    let ss_ver = chroma
        & (f.sr_cur.p.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint) as c_int;
    let stride: ptrdiff_t = f.sr_cur.p.stride[chroma as usize];
    let sby =
        y + (if y != 0 {
            (8 as c_int) << ss_ver
        } else {
            0 as c_int
        }) >> 6 - ss_ver + seq_hdr.sb128;
    let have_tt = (c.tc.len() > 1) as c_int;
    let mut lpf: *const BD::Pixel = (f.lf.lr_lpf_line[plane as usize] as *mut BD::Pixel)
        .offset(
            (have_tt * (sby * ((4 as c_int) << seq_hdr.sb128) - 4)) as isize
                * BD::pxstride(stride as usize) as isize,
        )
        .offset(x as isize);
    // The first stripe of the frame is shorter by 8 luma pixel rows.
    let mut stripe_h = cmp::min(64 - 8 * (y == 0) as c_int >> ss_ver, row_h - y);
    let lr_fn: looprestorationfilter_fn;
    let mut params: LooprestorationParams = LooprestorationParams {
        filter: [[0; 8]; 2].into(),
    };
    if lr.r#type as c_int == RAV1D_RESTORATION_WIENER as c_int {
        let filter: *mut [i16; 8] = (params.filter.0).as_mut_ptr();
        let ref mut fresh0 = (*filter.offset(0))[6];
        *fresh0 = lr.filter_h[0] as i16;
        (*filter.offset(0))[0] = *fresh0;
        let ref mut fresh1 = (*filter.offset(0))[5];
        *fresh1 = lr.filter_h[1] as i16;
        (*filter.offset(0))[1] = *fresh1;
        let ref mut fresh2 = (*filter.offset(0))[4];
        *fresh2 = lr.filter_h[2] as i16;
        (*filter.offset(0))[2] = *fresh2;
        (*filter.offset(0))[3] = (-((*filter.offset(0))[0] as c_int
            + (*filter.offset(0))[1] as c_int
            + (*filter.offset(0))[2] as c_int)
            * 2) as i16;
        let ref mut fresh3 = (*filter.offset(0))[3];
        if BD::BITDEPTH != 8 {
            // For 8-bit SIMD it's beneficial to handle the +128 separately
            // in order to avoid overflows.
            *fresh3 = (*fresh3 + 128) as i16;
        }
        let ref mut fresh4 = (*filter.offset(1))[6];
        *fresh4 = lr.filter_v[0] as i16;
        (*filter.offset(1))[0] = *fresh4;
        let ref mut fresh5 = (*filter.offset(1))[5];
        *fresh5 = lr.filter_v[1] as i16;
        (*filter.offset(1))[1] = *fresh5;
        let ref mut fresh6 = (*filter.offset(1))[4];
        *fresh6 = lr.filter_v[2] as i16;
        (*filter.offset(1))[2] = *fresh6;
        (*filter.offset(1))[3] = (128 as c_int
            - ((*filter.offset(1))[0] as c_int
                + (*filter.offset(1))[1] as c_int
                + (*filter.offset(1))[2] as c_int)
                * 2) as i16;
        lr_fn = (*dsp).lr.wiener[((*filter.offset(0))[0] as c_int | (*filter.offset(1))[0] as c_int
            == 0) as c_int as usize];
    } else {
        if !(lr.r#type as c_int == RAV1D_RESTORATION_SGRPROJ as c_int) {
            unreachable!();
        }
        let sgr_params: *const u16 = (dav1d_sgr_params[lr.sgr_idx as usize]).as_ptr();
        params.sgr.s0 = *sgr_params.offset(0) as u32;
        params.sgr.s1 = *sgr_params.offset(1) as u32;
        params.sgr.w0 = lr.sgr_weights[0] as i16;
        params.sgr.w1 =
            (128 as c_int - (lr.sgr_weights[0] as c_int + lr.sgr_weights[1] as c_int)) as i16;
        lr_fn = (*dsp).lr.sgr[((*sgr_params.offset(0) != 0) as c_int
            + (*sgr_params.offset(1) != 0) as c_int * 2
            - 1) as usize];
    }
    while y + stripe_h <= row_h {
        // Change the HAVE_BOTTOM bit in edges to (sby + 1 != f->sbh || y + stripe_h != row_h)
        edges = ::core::mem::transmute::<c_uint, LrEdgeFlags>(
            edges as c_uint
                ^ (-((sby + 1 != f.sbh || y + stripe_h != row_h) as c_int) as c_uint
                    ^ edges as c_uint)
                    & LR_HAVE_BOTTOM as c_int as c_uint,
        );
        lr_fn(
            p.cast(),
            stride,
            left.cast(),
            lpf.cast(),
            unit_w,
            stripe_h,
            &mut params,
            edges,
            f.bitdepth_max,
        );
        left = left.offset(stripe_h as isize);
        y += stripe_h;
        p = p.offset(stripe_h as isize * BD::pxstride(stride as usize) as isize);
        edges = ::core::mem::transmute::<c_uint, LrEdgeFlags>(
            edges as c_uint | LR_HAVE_TOP as c_int as c_uint,
        );
        stripe_h = cmp::min(64 >> ss_ver, row_h - y);
        if stripe_h == 0 {
            break;
        }
        lpf = lpf.offset(4 * BD::pxstride(stride as usize) as isize);
    }
}

unsafe fn backup4xU<BD: BitDepth>(
    mut dst: *mut [BD::Pixel; 4],
    mut src: *const BD::Pixel,
    src_stride: ptrdiff_t,
    mut u: c_int,
) {
    while u > 0 {
        BD::pixel_copy(
            slice::from_raw_parts_mut(&mut *dst as *mut BD::Pixel, 4),
            slice::from_raw_parts(&*src as *const BD::Pixel, 4),
            4,
        );
        u -= 1;
        dst = dst.offset(1);
        src = src.offset(BD::pxstride(src_stride as usize) as isize);
    }
}

unsafe fn lr_sbrow<BD: BitDepth>(
    c: &Rav1dContext,
    f: &Rav1dFrameData,
    mut p: *mut BD::Pixel,
    y: c_int,
    w: c_int,
    h: c_int,
    row_h: c_int,
    plane: c_int,
) {
    let chroma = (plane != 0) as c_int;
    let ss_ver = chroma
        & (f.sr_cur.p.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint) as c_int;
    let ss_hor = chroma
        & (f.sr_cur.p.p.layout as c_uint != Rav1dPixelLayout::I444 as c_int as c_uint) as c_int;
    let p_stride: ptrdiff_t = f.sr_cur.p.stride[chroma as usize];
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    let unit_size_log2 = frame_hdr.restoration.unit_size[(plane != 0) as c_int as usize];
    let unit_size = (1 as c_int) << unit_size_log2;
    let half_unit_size = unit_size >> 1;
    let max_unit_size = unit_size + half_unit_size;

    // Y coordinate of the sbrow (y is 8 luma pixel rows above row_y)
    let row_y = y + (8 >> ss_ver) * (y != 0) as c_int;

    // FIXME This is an ugly hack to lookup the proper AV1Filter unit for
    // chroma planes. Question: For Multithreaded decoding, is it better
    // to store the chroma LR information with collocated Luma information?
    // In other words. For a chroma restoration unit locate at 128,128 and
    // with a 4:2:0 chroma subsampling, do we store the filter information at
    // the AV1Filter unit located at (128,128) or (256,256)
    // TODO Support chroma subsampling.
    let shift_hor = 7 - ss_hor;

    // maximum sbrow height is 128 + 8 rows offset
    let mut pre_lr_border: Align16<[[[BD::Pixel; 4]; 128 + 8]; 2]> =
        Align16([[[0.into(); 4]; 128 + 8]; 2]);
    let mut lr = [Av1RestorationUnit::default(); 2];
    let mut edges: LrEdgeFlags = ((if y > 0 {
        LR_HAVE_TOP as c_int
    } else {
        0 as c_int
    }) | LR_HAVE_RIGHT as c_int) as LrEdgeFlags;
    let mut aligned_unit_pos = row_y & !(unit_size - 1);
    if aligned_unit_pos != 0 && aligned_unit_pos + half_unit_size > h {
        aligned_unit_pos -= unit_size;
    }
    aligned_unit_pos <<= ss_ver;
    let sb_idx = (aligned_unit_pos >> 7) * f.sr_sb128w;
    let unit_idx = (aligned_unit_pos >> 6 & 1) << 1;
    lr[0] = (*(f.lf.lr_mask).offset(sb_idx as isize)).lr[plane as usize][unit_idx as usize];
    let mut restore = (lr[0].r#type as c_int != RAV1D_RESTORATION_NONE as c_int) as c_int;
    let mut x = 0;
    let mut bit = 0;
    while x + max_unit_size <= w {
        let next_x = x + unit_size;
        let next_u_idx = unit_idx + (next_x >> shift_hor - 1 & 1);
        lr[(bit == 0) as c_int as usize] = (*(f.lf.lr_mask)
            .offset((sb_idx + (next_x >> shift_hor)) as isize))
        .lr[plane as usize][next_u_idx as usize];
        let restore_next = (lr[(bit == 0) as c_int as usize].r#type as c_int
            != RAV1D_RESTORATION_NONE as c_int) as c_int;
        if restore_next != 0 {
            backup4xU::<BD>(
                (pre_lr_border[bit as usize]).as_mut_ptr(),
                p.offset(unit_size as isize).offset(-(4 as c_int as isize)),
                p_stride,
                row_h - y,
            );
        }
        if restore != 0 {
            lr_stripe::<BD>(
                c,
                f,
                p,
                (pre_lr_border[(bit == 0) as c_int as usize]).as_mut_ptr() as *const [BD::Pixel; 4],
                x,
                y,
                plane,
                unit_size,
                row_h,
                lr[bit as usize],
                edges,
            );
        }
        x = next_x;
        restore = restore_next;
        p = p.offset(unit_size as isize);
        edges = ::core::mem::transmute::<c_uint, LrEdgeFlags>(
            edges as c_uint | LR_HAVE_LEFT as c_int as c_uint,
        );
        bit ^= 1 as c_int;
    }
    if restore != 0 {
        edges = ::core::mem::transmute::<c_uint, LrEdgeFlags>(
            edges as c_uint & !(LR_HAVE_RIGHT as c_int) as c_uint,
        );
        let unit_w = w - x;
        lr_stripe::<BD>(
            c,
            f,
            p,
            (pre_lr_border[(bit == 0) as c_int as usize]).as_mut_ptr() as *const [BD::Pixel; 4],
            x,
            y,
            plane,
            unit_w,
            row_h,
            lr[bit as usize],
            edges,
        );
    }
}

pub(crate) unsafe fn rav1d_lr_sbrow<BD: BitDepth>(
    c: &Rav1dContext,
    f: &mut Rav1dFrameData,
    dst: *const *mut BD::Pixel,
    sby: c_int,
) {
    let offset_y = 8 * (sby != 0) as c_int;
    let dst_stride: *const ptrdiff_t = (f.sr_cur.p.stride).as_mut_ptr();
    let restore_planes = f.lf.restore_planes;
    let not_last = ((sby + 1) < f.sbh) as c_int;
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    if restore_planes & LR_RESTORE_Y as c_int != 0 {
        let h = f.sr_cur.p.p.h;
        let w = f.sr_cur.p.p.w;
        let next_row_y = (sby + 1) << 6 + seq_hdr.sb128;
        let row_h = cmp::min(next_row_y - 8 * not_last, h);
        let y_stripe = (sby << 6 + seq_hdr.sb128) - offset_y;
        lr_sbrow::<BD>(
            c,
            f,
            (*dst.offset(0)).offset(
                -(offset_y as isize * BD::pxstride(*dst_stride.offset(0) as usize) as isize),
            ),
            y_stripe,
            w,
            h,
            row_h,
            0 as c_int,
        );
    }
    if restore_planes & (LR_RESTORE_U as c_int | LR_RESTORE_V as c_int) != 0 {
        let ss_ver =
            (f.sr_cur.p.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint) as c_int;
        let ss_hor =
            (f.sr_cur.p.p.layout as c_uint != Rav1dPixelLayout::I444 as c_int as c_uint) as c_int;
        let h_0 = f.sr_cur.p.p.h + ss_ver >> ss_ver;
        let w_0 = f.sr_cur.p.p.w + ss_hor >> ss_hor;
        let next_row_y_0 = (sby + 1) << 6 - ss_ver + seq_hdr.sb128;
        let row_h_0 = cmp::min(next_row_y_0 - (8 >> ss_ver) * not_last, h_0);
        let offset_uv = offset_y >> ss_ver;
        let y_stripe_0 = (sby << 6 - ss_ver + seq_hdr.sb128) - offset_uv;
        if restore_planes & LR_RESTORE_U as c_int != 0 {
            lr_sbrow::<BD>(
                c,
                f,
                (*dst.offset(1)).offset(
                    -(offset_uv as isize * BD::pxstride(*dst_stride.offset(1) as usize) as isize),
                ),
                y_stripe_0,
                w_0,
                h_0,
                row_h_0,
                1 as c_int,
            );
        }
        if restore_planes & LR_RESTORE_V as c_int != 0 {
            lr_sbrow::<BD>(
                c,
                f,
                (*dst.offset(2)).offset(
                    -(offset_uv as isize * BD::pxstride(*dst_stride.offset(1) as usize) as isize),
                ),
                y_stripe_0,
                w_0,
                h_0,
                row_h_0,
                2 as c_int,
            );
        }
    }
}
