use crate::include::common::bitdepth::BitDepth;
use crate::include::dav1d::headers::Rav1dPixelLayout;

use crate::src::internal::Rav1dDSPContext;
use crate::src::internal::Rav1dFrameContext;

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

pub(crate) unsafe fn rav1d_copy_lpf<BD: BitDepth>(
    f: *mut Rav1dFrameContext,
    src: *const *mut BD::Pixel,
    sby: c_int,
) {
    let have_tt = ((*(*f).c).n_tc > 1 as c_uint) as c_int;
    let resize = ((*(*f).frame_hdr).width[0] != (*(*f).frame_hdr).width[1]) as c_int;
    let offset = 8 * (sby != 0) as c_int;
    let src_stride: *const ptrdiff_t = ((*f).cur.stride).as_mut_ptr();
    let lr_stride: *const ptrdiff_t = ((*f).sr_cur.p.stride).as_mut_ptr();
    let tt_off = have_tt * sby * ((4 as c_int) << (*(*f).seq_hdr).sb128);
    let dst: [*mut BD::Pixel; 3] = [
        ((*f).lf.lr_lpf_line[0] as *mut BD::Pixel)
            .offset(tt_off as isize * BD::pxstride(*lr_stride.offset(0) as usize) as isize),
        ((*f).lf.lr_lpf_line[1] as *mut BD::Pixel)
            .offset(tt_off as isize * BD::pxstride(*lr_stride.offset(1) as usize) as isize),
        ((*f).lf.lr_lpf_line[2] as *mut BD::Pixel)
            .offset(tt_off as isize * BD::pxstride(*lr_stride.offset(1) as usize) as isize),
    ];
    let restore_planes = (*f).lf.restore_planes;
    if (*(*f).seq_hdr).cdef != 0 || restore_planes & LR_RESTORE_Y as c_int != 0 {
        let h = (*f).cur.p.h;
        let w = (*f).bw << 2;
        let row_h = cmp::min((sby + 1) << 6 + (*(*f).seq_hdr).sb128, h - 1);
        let y_stripe = (sby << 6 + (*(*f).seq_hdr).sb128) - offset;
        if restore_planes & LR_RESTORE_Y as c_int != 0 || resize == 0 {
            backup_lpf::<BD>(
                f,
                dst[0],
                *lr_stride.offset(0),
                (*src.offset(0)).offset(
                    -(offset as isize * BD::pxstride(*src_stride.offset(0) as usize) as isize),
                ),
                *src_stride.offset(0),
                0 as c_int,
                (*(*f).seq_hdr).sb128,
                y_stripe,
                row_h,
                w,
                h,
                0 as c_int,
                1 as c_int,
            );
        }
        if have_tt != 0 && resize != 0 {
            let cdef_off_y: ptrdiff_t =
                (sby * 4) as isize * BD::pxstride(*src_stride.offset(0) as usize) as isize;
            backup_lpf::<BD>(
                f,
                ((*f).lf.cdef_lpf_line[0] as *mut BD::Pixel).offset(cdef_off_y as isize),
                *src_stride.offset(0),
                (*src.offset(0)).offset(
                    -offset as isize * BD::pxstride(*src_stride.offset(0) as usize) as isize,
                ),
                *src_stride.offset(0),
                0 as c_int,
                (*(*f).seq_hdr).sb128,
                y_stripe,
                row_h,
                w,
                h,
                0 as c_int,
                0 as c_int,
            );
        }
    }
    if ((*(*f).seq_hdr).cdef != 0
        || restore_planes & (LR_RESTORE_U as c_int | LR_RESTORE_V as c_int) != 0)
        && (*f).cur.p.layout as c_uint != Rav1dPixelLayout::I400 as c_int as c_uint
    {
        let ss_ver = ((*f).sr_cur.p.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint)
            as c_int;
        let ss_hor = ((*f).sr_cur.p.p.layout as c_uint != Rav1dPixelLayout::I444 as c_int as c_uint)
            as c_int;
        let h_0 = (*f).cur.p.h + ss_ver >> ss_ver;
        let w_0 = (*f).bw << 2 - ss_hor;
        let row_h_0 = cmp::min((sby + 1) << 6 - ss_ver + (*(*f).seq_hdr).sb128, h_0 - 1);
        let offset_uv = offset >> ss_ver;
        let y_stripe_0 = (sby << 6 - ss_ver + (*(*f).seq_hdr).sb128) - offset_uv;
        let cdef_off_uv: ptrdiff_t =
            sby as isize * 4 * BD::pxstride(*src_stride.offset(1) as usize) as isize;
        if (*(*f).seq_hdr).cdef != 0 || restore_planes & LR_RESTORE_U as c_int != 0 {
            if restore_planes & LR_RESTORE_U as c_int != 0 || resize == 0 {
                backup_lpf::<BD>(
                    f,
                    dst[1],
                    *lr_stride.offset(1),
                    (*src.offset(1)).offset(
                        -offset_uv as isize * BD::pxstride(*src_stride.offset(1) as usize) as isize,
                    ),
                    *src_stride.offset(1),
                    ss_ver,
                    (*(*f).seq_hdr).sb128,
                    y_stripe_0,
                    row_h_0,
                    w_0,
                    h_0,
                    ss_hor,
                    1 as c_int,
                );
            }
            if have_tt != 0 && resize != 0 {
                backup_lpf::<BD>(
                    f,
                    ((*f).lf.cdef_lpf_line[1] as *mut BD::Pixel).offset(cdef_off_uv as isize),
                    *src_stride.offset(1),
                    (*src.offset(1)).offset(
                        -offset_uv as isize * BD::pxstride(*src_stride.offset(1) as usize) as isize,
                    ),
                    *src_stride.offset(1),
                    ss_ver,
                    (*(*f).seq_hdr).sb128,
                    y_stripe_0,
                    row_h_0,
                    w_0,
                    h_0,
                    ss_hor,
                    0 as c_int,
                );
            }
        }
        if (*(*f).seq_hdr).cdef != 0 || restore_planes & LR_RESTORE_V as c_int != 0 {
            if restore_planes & LR_RESTORE_V as c_int != 0 || resize == 0 {
                backup_lpf::<BD>(
                    f,
                    dst[2],
                    *lr_stride.offset(1),
                    (*src.offset(2)).offset(
                        -offset_uv as isize * BD::pxstride(*src_stride.offset(1) as usize) as isize,
                    ),
                    *src_stride.offset(1),
                    ss_ver,
                    (*(*f).seq_hdr).sb128,
                    y_stripe_0,
                    row_h_0,
                    w_0,
                    h_0,
                    ss_hor,
                    1 as c_int,
                );
            }
            if have_tt != 0 && resize != 0 {
                backup_lpf::<BD>(
                    f,
                    ((*f).lf.cdef_lpf_line[2] as *mut BD::Pixel).offset(cdef_off_uv as isize),
                    *src_stride.offset(1),
                    (*src.offset(2)).offset(
                        -offset_uv as isize * BD::pxstride(*src_stride.offset(1) as usize) as isize,
                    ),
                    *src_stride.offset(1),
                    ss_ver,
                    (*(*f).seq_hdr).sb128,
                    y_stripe_0,
                    row_h_0,
                    w_0,
                    h_0,
                    ss_hor,
                    0 as c_int,
                );
            }
        }
    }
}

// TODO(perl) Temporarily pub until mod is deduplicated
#[inline]
pub(crate) unsafe fn filter_plane_cols_y<BD: BitDepth>(
    f: *const Rav1dFrameContext,
    have_left: c_int,
    lvl: *const [u8; 4],
    b4_stride: ptrdiff_t,
    mask: *const [[u16; 2]; 3],
    dst: *mut BD::Pixel,
    ls: ptrdiff_t,
    w: c_int,
    starty4: c_int,
    endy4: c_int,
) {
    let dsp: *const Rav1dDSPContext = (*f).dsp;
    let mut x = 0;
    while x < w {
        if !(have_left == 0 && x == 0) {
            let mut hmask: [u32; 4] = [0; 4];
            if starty4 == 0 {
                hmask[0] = (*mask.offset(x as isize))[0][0] as u32;
                hmask[1] = (*mask.offset(x as isize))[1][0] as u32;
                hmask[2] = (*mask.offset(x as isize))[2][0] as u32;
                if endy4 > 16 {
                    hmask[0] |= ((*mask.offset(x as isize))[0][1] as c_uint) << 16;
                    hmask[1] |= ((*mask.offset(x as isize))[1][1] as c_uint) << 16;
                    hmask[2] |= ((*mask.offset(x as isize))[2][1] as c_uint) << 16;
                }
            } else {
                hmask[0] = (*mask.offset(x as isize))[0][1] as u32;
                hmask[1] = (*mask.offset(x as isize))[1][1] as u32;
                hmask[2] = (*mask.offset(x as isize))[2][1] as u32;
            }
            hmask[3] = 0 as c_int as u32;
            (*dsp).lf.loop_filter_sb[0][0](
                dst.offset((x * 4) as isize).cast(),
                ls,
                hmask.as_mut_ptr(),
                &*(*lvl.offset(x as isize)).as_ptr().offset(0) as *const u8 as *const [u8; 4],
                b4_stride,
                &(*f).lf.lim_lut.0,
                endy4 - starty4,
                (*f).bitdepth_max,
            );
        }
        x += 1;
    }
}

// TODO(perl) Temporarily pub until mod is deduplicated
#[inline]
pub(crate) unsafe fn filter_plane_rows_y<BD: BitDepth>(
    f: *const Rav1dFrameContext,
    have_top: c_int,
    mut lvl: *const [u8; 4],
    b4_stride: ptrdiff_t,
    mask: *const [[u16; 2]; 3],
    mut dst: *mut BD::Pixel,
    ls: ptrdiff_t,
    w: c_int,
    starty4: c_int,
    endy4: c_int,
) {
    let dsp: *const Rav1dDSPContext = (*f).dsp;
    let mut y = starty4;
    while y < endy4 {
        if !(have_top == 0 && y == 0) {
            let vmask: [u32; 4] = [
                (*mask.offset(y as isize))[0][0] as c_uint
                    | ((*mask.offset(y as isize))[0][1] as c_uint) << 16,
                (*mask.offset(y as isize))[1][0] as c_uint
                    | ((*mask.offset(y as isize))[1][1] as c_uint) << 16,
                (*mask.offset(y as isize))[2][0] as c_uint
                    | ((*mask.offset(y as isize))[2][1] as c_uint) << 16,
                0 as c_int as u32,
            ];
            (*dsp).lf.loop_filter_sb[0][1](
                dst.cast(),
                ls,
                vmask.as_ptr(),
                &*(*lvl.offset(0)).as_ptr().offset(1) as *const u8 as *const [u8; 4],
                b4_stride,
                &(*f).lf.lim_lut.0,
                w,
                (*f).bitdepth_max,
            );
        }
        y += 1;
        dst = dst.offset(4 * BD::pxstride(ls as usize) as isize);
        lvl = lvl.offset(b4_stride as isize);
    }
}
