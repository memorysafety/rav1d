use crate::include::common::bitdepth::BitDepth8;
use crate::include::dav1d::headers::Rav1dPixelLayout;

use crate::src::internal::Rav1dFrameContext;

use crate::src::lr_apply::lr_sbrow;

use crate::src::lr_apply::LR_RESTORE_U;
use crate::src::lr_apply::LR_RESTORE_V;
use crate::src::lr_apply::LR_RESTORE_Y;

use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;

pub type pixel = u8;

pub(crate) unsafe fn rav1d_lr_sbrow_8bpc(
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
        lr_sbrow::<BitDepth8>(
            f,
            (*dst.offset(0)).offset(-((offset_y as isize * *dst_stride.offset(0)) as isize)),
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
            lr_sbrow::<BitDepth8>(
                f,
                (*dst.offset(1)).offset(-((offset_uv as isize * *dst_stride.offset(1)) as isize)),
                y_stripe_0,
                w_0,
                h_0,
                row_h_0,
                1 as c_int,
            );
        }
        if restore_planes & LR_RESTORE_V as c_int != 0 {
            lr_sbrow::<BitDepth8>(
                f,
                (*dst.offset(2)).offset(-((offset_uv as isize * *dst_stride.offset(1)) as isize)),
                y_stripe_0,
                w_0,
                h_0,
                row_h_0,
                2 as c_int,
            );
        }
    }
}
