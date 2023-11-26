use crate::include::common::bitdepth::BitDepth;
use crate::include::common::intops::ulog2;
use crate::include::dav1d::headers::Rav1dPixelLayout;

use libc::memcpy;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_void;

pub type pixel = u16;

pub type Backup2x8Flags = c_uint;
pub const BACKUP_2X8_UV: Backup2x8Flags = 2;
pub const BACKUP_2X8_Y: Backup2x8Flags = 1;

// TODO(perl) Temporarily pub until mod is deduplicated
pub(crate) unsafe fn backup2lines<BD: BitDepth>(
    dst: *const *mut BD::Pixel,
    src: *const *mut BD::Pixel,
    stride: *const ptrdiff_t,
    layout: Rav1dPixelLayout,
) {
    let y_stride: ptrdiff_t = BD::pxstride(*stride.offset(0) as usize) as isize;
    if y_stride < 0 {
        let n = if BD::BITDEPTH == 8 {
            (-2 * y_stride) as usize
        } else {
            (-2 * y_stride << 1) as usize
        };
        memcpy(
            (*dst.offset(0)).offset(y_stride as isize) as *mut c_void,
            (*src.offset(0)).offset((7 as c_int as isize * y_stride) as isize) as *const c_void,
            n,
        );
    } else {
        let n = if BD::BITDEPTH == 8 {
            (2 * y_stride) as usize
        } else {
            (2 * y_stride << 1) as usize
        };
        memcpy(
            *dst.offset(0) as *mut c_void,
            (*src.offset(0)).offset(6 * y_stride as isize) as *const c_void,
            n,
        );
    }
    if layout as c_uint != Rav1dPixelLayout::I400 as c_int as c_uint {
        let uv_stride: ptrdiff_t = BD::pxstride(*stride.offset(1) as usize) as isize;
        if uv_stride < 0 {
            let uv_off = if layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint {
                3 as c_int
            } else {
                7 as c_int
            };
            let n = if BD::BITDEPTH == 8 {
                (-2 * uv_stride) as usize
            } else {
                (-2 * uv_stride << 1) as usize
            };
            memcpy(
                (*dst.offset(1)).offset(uv_stride as isize) as *mut c_void,
                (*src.offset(1)).offset((uv_off as isize * uv_stride) as isize) as *const c_void,
                n,
            );
            memcpy(
                (*dst.offset(2)).offset(uv_stride as isize) as *mut c_void,
                (*src.offset(2)).offset((uv_off as isize * uv_stride) as isize) as *const c_void,
                n,
            );
        } else {
            let uv_off_0 = if layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint {
                2 as c_int
            } else {
                6 as c_int
            };
            let n = if BD::BITDEPTH == 8 {
                (2 * uv_stride) as usize
            } else {
                (2 * uv_stride << 1) as usize
            };
            memcpy(
                *dst.offset(1) as *mut c_void,
                (*src.offset(1)).offset((uv_off_0 as isize * uv_stride) as isize) as *const c_void,
                n,
            );
            memcpy(
                *dst.offset(2) as *mut c_void,
                (*src.offset(2)).offset((uv_off_0 as isize * uv_stride) as isize) as *const c_void,
                n,
            );
        }
    }
}

// TODO(perl) Temporarily pub until mod is deduplicated
pub(crate) unsafe fn backup2x8<BD: BitDepth>(
    dst: *mut [[BD::Pixel; 2]; 8],
    src: *const *mut BD::Pixel,
    src_stride: *const ptrdiff_t,
    mut x_off: c_int,
    layout: Rav1dPixelLayout,
    flag: Backup2x8Flags,
) {
    let mut y_off: ptrdiff_t = 0 as c_int as ptrdiff_t;
    let n = if BD::BITDEPTH == 8 {
        2usize
    } else {
        (2usize << 1) as usize
    };
    if flag as c_uint & BACKUP_2X8_Y as c_int as c_uint != 0 {
        let mut y = 0;
        while y < 8 {
            memcpy(
                ((*dst.offset(0))[y as usize]).as_mut_ptr() as *mut c_void,
                &mut *(*src.offset(0)).offset((y_off + x_off as isize - 2 as isize) as isize)
                    as *mut BD::Pixel as *const c_void,
                n,
            );
            y += 1;
            y_off += BD::pxstride(*src_stride.offset(0) as usize) as isize;
        }
    }
    if layout as c_uint == Rav1dPixelLayout::I400 as c_int as c_uint
        || flag as c_uint & BACKUP_2X8_UV as c_int as c_uint == 0
    {
        return;
    }
    let ss_ver = (layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint) as c_int;
    let ss_hor = (layout as c_uint != Rav1dPixelLayout::I444 as c_int as c_uint) as c_int;
    x_off >>= ss_hor;
    y_off = 0 as c_int as ptrdiff_t;
    let mut y_0 = 0;
    while y_0 < 8 >> ss_ver {
        memcpy(
            ((*dst.offset(1))[y_0 as usize]).as_mut_ptr() as *mut c_void,
            &mut *(*src.offset(1)).offset((y_off + x_off as isize - 2) as isize) as *mut BD::Pixel
                as *const c_void,
            n,
        );
        memcpy(
            ((*dst.offset(2))[y_0 as usize]).as_mut_ptr() as *mut c_void,
            &mut *(*src.offset(2)).offset((y_off + x_off as isize - 2) as isize) as *mut BD::Pixel
                as *const c_void,
            n,
        );
        y_0 += 1;
        y_off += BD::pxstride(*src_stride.offset(1) as usize) as isize;
    }
}

// TODO(perl) Temporarily pub until mod is deduplicated
pub(crate) unsafe fn adjust_strength(strength: c_int, var: c_uint) -> c_int {
    if var == 0 {
        return 0 as c_int;
    }
    let i = if var >> 6 != 0 {
        cmp::min(ulog2(var >> 6), 12 as c_int)
    } else {
        0 as c_int
    };
    return strength * (4 + i) + 8 >> 4;
}
