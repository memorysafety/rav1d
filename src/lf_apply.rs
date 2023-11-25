use crate::include::common::bitdepth::BitDepth;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::src::env::BlockContext;
use crate::src::internal::Rav1dDSPContext;
use crate::src::internal::Rav1dFrameContext;
use crate::src::lf_mask::Av1Filter;
use crate::src::lr_apply::LR_RESTORE_U;
use crate::src::lr_apply::LR_RESTORE_V;
use crate::src::lr_apply::LR_RESTORE_Y;
use libc::memcpy;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_void;

// TODO(perl) Temporarily pub until mod is deduplicated
pub(crate) unsafe fn backup_lpf<BD: BitDepth>(
    f: *const Rav1dFrameContext,
    mut dst: *mut BD::Pixel,
    dst_stride: ptrdiff_t,
    mut src: *mut BD::Pixel,
    src_stride: ptrdiff_t,
    ss_ver: c_int,
    sb128: c_int,
    mut row: c_int,
    row_h: c_int,
    src_w: c_int,
    h: c_int,
    ss_hor: c_int,
    lr_backup: c_int,
) {
    let cdef_backup = (lr_backup == 0) as c_int;
    let dst_w = if (*(*f).frame_hdr).super_res.enabled != 0 {
        (*(*f).frame_hdr).width[1] + ss_hor >> ss_hor
    } else {
        src_w
    };
    let mut stripe_h = ((64 as c_int) << (cdef_backup & sb128)) - 8 * (row == 0) as c_int >> ss_ver;
    src = src.offset((stripe_h - 2) as isize * BD::pxstride(src_stride as usize) as isize);
    if (*(*f).c).n_tc == 1 as c_uint {
        if row != 0 {
            let top = (4 as c_int) << sb128;
            memcpy(
                &mut *dst.offset(BD::pxstride(dst_stride as usize * 0) as isize) as *mut BD::Pixel
                    as *mut c_void,
                &mut *dst.offset(BD::pxstride(dst_stride as usize * top as usize) as isize)
                    as *mut BD::Pixel as *const c_void,
                if BD::BITDEPTH == 8 {
                    dst_w as usize
                } else {
                    (dst_w << 1) as usize
                },
            );
            memcpy(
                &mut *dst.offset(BD::pxstride(dst_stride as usize * 1) as isize) as *mut BD::Pixel
                    as *mut c_void,
                &mut *dst.offset(BD::pxstride(dst_stride as usize * (top + 1) as usize) as isize)
                    as *mut BD::Pixel as *const c_void,
                if BD::BITDEPTH == 8 {
                    dst_w as usize
                } else {
                    (dst_w << 1) as usize
                },
            );
            memcpy(
                &mut *dst.offset(BD::pxstride(dst_stride as usize * 2) as isize) as *mut BD::Pixel
                    as *mut c_void,
                &mut *dst.offset(BD::pxstride(dst_stride as usize * (top + 2) as usize) as isize)
                    as *mut BD::Pixel as *const c_void,
                if BD::BITDEPTH == 8 {
                    dst_w as usize
                } else {
                    (dst_w << 1) as usize
                },
            );
            memcpy(
                &mut *dst.offset(BD::pxstride(dst_stride as usize * 3) as isize) as *mut BD::Pixel
                    as *mut c_void,
                &mut *dst.offset(BD::pxstride(dst_stride as usize * (top + 3) as usize) as isize)
                    as *mut BD::Pixel as *const c_void,
                if BD::BITDEPTH == 8 {
                    dst_w as usize
                } else {
                    (dst_w << 1) as usize
                },
            );
        }
        dst = dst.offset(4 * BD::pxstride(dst_stride as usize) as isize);
    }
    if lr_backup != 0 && (*(*f).frame_hdr).width[0] != (*(*f).frame_hdr).width[1] {
        while row + stripe_h <= row_h {
            let n_lines = 4 - (row + stripe_h + 1 == h) as c_int;
            ((*(*f).dsp).mc.resize)(
                dst.cast(),
                dst_stride,
                src.cast(),
                src_stride,
                dst_w,
                n_lines,
                src_w,
                (*f).resize_step[ss_hor as usize],
                (*f).resize_start[ss_hor as usize],
                (*f).bitdepth_max,
            );
            row += stripe_h;
            stripe_h = 64 >> ss_ver;
            src = src.offset(stripe_h as isize * BD::pxstride(src_stride as usize) as isize);
            dst = dst.offset(n_lines as isize * BD::pxstride(dst_stride as usize) as isize);
            if n_lines == 3 {
                memcpy(
                    dst as *mut c_void,
                    &mut *dst.offset(-1 * BD::pxstride(dst_stride as usize) as isize)
                        as *mut BD::Pixel as *const c_void,
                    if BD::BITDEPTH == 8 {
                        dst_w as usize
                    } else {
                        (dst_w << 1) as usize
                    },
                );
                dst = dst.offset(BD::pxstride(dst_stride as usize) as isize);
            }
        }
    } else {
        while row + stripe_h <= row_h {
            let n_lines_0 = 4 - (row + stripe_h + 1 == h) as c_int;
            let mut i = 0;
            while i < 4 {
                memcpy(
                    dst as *mut c_void,
                    (if i == n_lines_0 {
                        &mut *dst.offset(-1 * BD::pxstride(dst_stride as usize) as isize)
                            as *mut BD::Pixel as *const BD::Pixel
                    } else {
                        src
                    }) as *const c_void,
                    if BD::BITDEPTH == 8 {
                        src_w as usize
                    } else {
                        (src_w << 1) as usize
                    },
                );
                dst = dst.offset(BD::pxstride(dst_stride as usize) as isize);
                src = src.offset(BD::pxstride(src_stride as usize) as isize);
                i += 1;
            }
            row += stripe_h;
            stripe_h = 64 >> ss_ver;
            src = src.offset((stripe_h - 4) as isize * BD::pxstride(src_stride as usize) as isize);
        }
    };
}
