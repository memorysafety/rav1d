use crate::include::common::bitdepth::BitDepth;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::include::dav1d::headers::RAV1D_RESTORATION_NONE;
use crate::include::dav1d::headers::RAV1D_RESTORATION_SGRPROJ;
use crate::include::dav1d::headers::RAV1D_RESTORATION_WIENER;
use crate::src::align::Align16;
use crate::src::internal::Rav1dDSPContext;
use crate::src::internal::Rav1dFrameContext;
use crate::src::lf_mask::Av1RestorationUnit;
use crate::src::looprestoration::looprestorationfilter_fn;
use crate::src::looprestoration::LooprestorationParams;
use crate::src::looprestoration::LrEdgeFlags;
use crate::src::looprestoration::LR_HAVE_BOTTOM;
use crate::src::looprestoration::LR_HAVE_TOP;
use crate::src::tables::dav1d_sgr_params;

use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;

pub type LrRestorePlanes = c_uint;
pub const LR_RESTORE_V: LrRestorePlanes = 4;
pub const LR_RESTORE_U: LrRestorePlanes = 2;
pub const LR_RESTORE_Y: LrRestorePlanes = 1;

// TODO(perl) Temporarily pub until mod is deduplicated
pub(crate) unsafe fn lr_stripe<BD: BitDepth>(
    f: *const Rav1dFrameContext,
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
    let dsp: *const Rav1dDSPContext = (*f).dsp;
    let chroma = (plane != 0) as c_int;
    let ss_ver = chroma
        & ((*f).sr_cur.p.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint) as c_int;
    let stride: ptrdiff_t = (*f).sr_cur.p.stride[chroma as usize];
    let sby =
        y + (if y != 0 {
            (8 as c_int) << ss_ver
        } else {
            0 as c_int
        }) >> 6 - ss_ver + (*(*f).seq_hdr).sb128;
    let have_tt = ((*(*f).c).n_tc > 1 as c_uint) as c_int;
    let mut lpf: *const BD::Pixel = ((*f).lf.lr_lpf_line[plane as usize] as *mut BD::Pixel)
        .offset(
            (have_tt * (sby * ((4 as c_int) << (*(*f).seq_hdr).sb128) - 4)) as isize
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
                ^ (-((sby + 1 != (*f).sbh || y + stripe_h != row_h) as c_int) as c_uint
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
            (*f).bitdepth_max,
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
