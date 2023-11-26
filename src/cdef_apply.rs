use crate::include::common::bitdepth::BitDepth;
use crate::include::common::intops::ulog2;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::src::align::Align16;
use crate::src::cdef::CdefEdgeFlags;
use crate::src::cdef::CDEF_HAVE_BOTTOM;
use crate::src::cdef::CDEF_HAVE_LEFT;
use crate::src::cdef::CDEF_HAVE_RIGHT;
use crate::src::cdef::CDEF_HAVE_TOP;
use crate::src::internal::Rav1dDSPContext;
use crate::src::internal::Rav1dFrameContext;
use crate::src::internal::Rav1dTaskContext;
use crate::src::lf_mask::Av1Filter;
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

#[inline]
unsafe fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 != 0 {
        unreachable!();
    }
    return x >> 1;
}

// TODO(perl) Temporarily pub until mod is deduplicated
pub(crate) unsafe fn backup2lines<BD: BitDepth>(
    dst: *const *mut BD::Pixel,
    src: *const *mut BD::Pixel,
    stride: *const ptrdiff_t,
    layout: Rav1dPixelLayout,
) {
    let y_stride: ptrdiff_t = BD::pxstride(*stride.offset(0) as usize) as isize;
    if y_stride < 0 {
        let n = if BD::BITDEPTH == 8 { (-2 * y_stride) as usize } else { (-2 * y_stride << 1) as usize};
        memcpy(
            (*dst.offset(0)).offset(y_stride as isize) as *mut c_void,
            (*src.offset(0)).offset((7 as c_int as isize * y_stride) as isize) as *const c_void,
            n,
        );
    } else {
        let n = if BD::BITDEPTH == 8 { (2 * y_stride) as usize } else { (2 * y_stride << 1) as usize};
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
            let n = if BD::BITDEPTH == 8 { (-2 * uv_stride) as usize } else { (-2 * uv_stride << 1) as usize};
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
            let n = if BD::BITDEPTH == 8 { (2 * uv_stride) as usize } else { (2 * uv_stride << 1) as usize};
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