use crate::include::common::bitdepth::BitDepth16;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::include::dav1d::headers::RAV1D_RESTORATION_NONE;

use crate::src::align::Align16;

use crate::src::internal::Rav1dFrameContext;
use crate::src::lf_mask::Av1RestorationUnit;

use crate::src::looprestoration::LrEdgeFlags;

use crate::src::looprestoration::LR_HAVE_LEFT;
use crate::src::looprestoration::LR_HAVE_RIGHT;
use crate::src::looprestoration::LR_HAVE_TOP;
use crate::src::lr_apply::backup4xU;
use crate::src::lr_apply::lr_stripe;
use crate::src::lr_apply::LR_RESTORE_U;
use crate::src::lr_apply::LR_RESTORE_V;
use crate::src::lr_apply::LR_RESTORE_Y;

use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;

pub type pixel = u16;

#[inline]
unsafe fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 != 0 {
        unreachable!();
    }
    return x >> 1;
}

unsafe fn lr_sbrow(
    f: *const Rav1dFrameContext,
    mut p: *mut pixel,
    y: c_int,
    w: c_int,
    h: c_int,
    row_h: c_int,
    plane: c_int,
) {
    let chroma = (plane != 0) as c_int;
    let ss_ver = chroma
        & ((*f).sr_cur.p.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint) as c_int;
    let ss_hor = chroma
        & ((*f).sr_cur.p.p.layout as c_uint != Rav1dPixelLayout::I444 as c_int as c_uint) as c_int;
    let p_stride: ptrdiff_t = (*f).sr_cur.p.stride[chroma as usize];
    let unit_size_log2 = (*(*f).frame_hdr).restoration.unit_size[(plane != 0) as c_int as usize];
    let unit_size = (1 as c_int) << unit_size_log2;
    let half_unit_size = unit_size >> 1;
    let max_unit_size = unit_size + half_unit_size;
    let row_y = y + (8 >> ss_ver) * (y != 0) as c_int;
    let shift_hor = 7 - ss_hor;
    let mut pre_lr_border: Align16<[[[pixel; 4]; 136]; 2]> = Align16([[[0; 4]; 136]; 2]);
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
    let sb_idx = (aligned_unit_pos >> 7) * (*f).sr_sb128w;
    let unit_idx = (aligned_unit_pos >> 6 & 1) << 1;
    lr[0] = (*((*f).lf.lr_mask).offset(sb_idx as isize)).lr[plane as usize][unit_idx as usize];
    let mut restore = (lr[0].r#type as c_int != RAV1D_RESTORATION_NONE as c_int) as c_int;
    let mut x = 0;
    let mut bit = 0;
    while x + max_unit_size <= w {
        let next_x = x + unit_size;
        let next_u_idx = unit_idx + (next_x >> shift_hor - 1 & 1);
        lr[(bit == 0) as c_int as usize] = (*((*f).lf.lr_mask)
            .offset((sb_idx + (next_x >> shift_hor)) as isize))
        .lr[plane as usize][next_u_idx as usize];
        let restore_next = (lr[(bit == 0) as c_int as usize].r#type as c_int
            != RAV1D_RESTORATION_NONE as c_int) as c_int;
        if restore_next != 0 {
            backup4xU::<BitDepth16>(
                (pre_lr_border[bit as usize]).as_mut_ptr(),
                p.offset(unit_size as isize).offset(-(4 as c_int as isize)),
                p_stride,
                row_h - y,
            );
        }
        if restore != 0 {
            lr_stripe::<BitDepth16>(
                f,
                p,
                (pre_lr_border[(bit == 0) as c_int as usize]).as_mut_ptr() as *const [pixel; 4],
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
        lr_stripe::<BitDepth16>(
            f,
            p,
            (pre_lr_border[(bit == 0) as c_int as usize]).as_mut_ptr() as *const [pixel; 4],
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

pub(crate) unsafe fn rav1d_lr_sbrow_16bpc(
    f: *mut Rav1dFrameContext,
    dst: *const *mut pixel,
    sby: c_int,
) {
    let offset_y = 8 * (sby != 0) as c_int;
    let dst_stride: *const ptrdiff_t = ((*f).sr_cur.p.stride).as_mut_ptr();
    let restore_planes = (*f).lf.restore_planes;
    let not_last = ((sby + 1) < (*f).sbh) as c_int;
    if restore_planes & LR_RESTORE_Y as c_int != 0 {
        let h = (*f).sr_cur.p.p.h;
        let w = (*f).sr_cur.p.p.w;
        let next_row_y = (sby + 1) << 6 + (*(*f).seq_hdr).sb128;
        let row_h = cmp::min(next_row_y - 8 * not_last, h);
        let y_stripe = (sby << 6 + (*(*f).seq_hdr).sb128) - offset_y;
        lr_sbrow(
            f,
            (*dst.offset(0))
                .offset(-((offset_y as isize * PXSTRIDE(*dst_stride.offset(0))) as isize)),
            y_stripe,
            w,
            h,
            row_h,
            0 as c_int,
        );
    }
    if restore_planes & (LR_RESTORE_U as c_int | LR_RESTORE_V as c_int) != 0 {
        let ss_ver = ((*f).sr_cur.p.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint)
            as c_int;
        let ss_hor = ((*f).sr_cur.p.p.layout as c_uint != Rav1dPixelLayout::I444 as c_int as c_uint)
            as c_int;
        let h_0 = (*f).sr_cur.p.p.h + ss_ver >> ss_ver;
        let w_0 = (*f).sr_cur.p.p.w + ss_hor >> ss_hor;
        let next_row_y_0 = (sby + 1) << 6 - ss_ver + (*(*f).seq_hdr).sb128;
        let row_h_0 = cmp::min(next_row_y_0 - (8 >> ss_ver) * not_last, h_0);
        let offset_uv = offset_y >> ss_ver;
        let y_stripe_0 = (sby << 6 - ss_ver + (*(*f).seq_hdr).sb128) - offset_uv;
        if restore_planes & LR_RESTORE_U as c_int != 0 {
            lr_sbrow(
                f,
                (*dst.offset(1))
                    .offset(-((offset_uv as isize * PXSTRIDE(*dst_stride.offset(1))) as isize)),
                y_stripe_0,
                w_0,
                h_0,
                row_h_0,
                1 as c_int,
            );
        }
        if restore_planes & LR_RESTORE_V as c_int != 0 {
            lr_sbrow(
                f,
                (*dst.offset(2))
                    .offset(-((offset_uv as isize * PXSTRIDE(*dst_stride.offset(1))) as isize)),
                y_stripe_0,
                w_0,
                h_0,
                row_h_0,
                2 as c_int,
            );
        }
    }
}
