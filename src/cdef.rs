use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::bitdepth::LeftPixelRow2px;
use crate::include::common::intops::apply_sign;

use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;

pub type CdefEdgeFlags = c_uint;
pub const CDEF_HAVE_BOTTOM: CdefEdgeFlags = 8;
pub const CDEF_HAVE_TOP: CdefEdgeFlags = 4;
pub const CDEF_HAVE_RIGHT: CdefEdgeFlags = 2;
pub const CDEF_HAVE_LEFT: CdefEdgeFlags = 1;

pub type cdef_fn = unsafe extern "C" fn(
    *mut DynPixel,
    ptrdiff_t,
    *const LeftPixelRow2px<DynPixel>,
    *const DynPixel,
    *const DynPixel,
    c_int,
    c_int,
    c_int,
    c_int,
    CdefEdgeFlags,
    c_int,
) -> ();

pub type cdef_dir_fn =
    unsafe extern "C" fn(*const DynPixel, ptrdiff_t, *mut c_uint, c_int) -> c_int;

#[repr(C)]
pub struct Rav1dCdefDSPContext {
    pub dir: cdef_dir_fn,
    pub fb: [cdef_fn; 3],
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
#[cfg(all(
    feature = "asm",
    feature = "bitdepth_8",
    any(target_arch = "x86", target_arch = "x86_64"),
))]
extern "C" {
    pub(crate) fn dav1d_cdef_filter_8x8_8bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_8bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x4_8bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_dir_8bpc_sse4(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut c_uint,
        bitdepth_max: c_int,
    ) -> c_int;
    pub(crate) fn dav1d_cdef_filter_8x8_8bpc_sse4(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_8bpc_sse4(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x4_8bpc_sse4(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_8bpc_sse2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_dir_8bpc_ssse3(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut c_uint,
        bitdepth_max: c_int,
    ) -> c_int;
    pub(crate) fn dav1d_cdef_filter_4x4_8bpc_sse2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_8x8_8bpc_sse2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
#[cfg(all(feature = "asm", feature = "bitdepth_8", target_arch = "x86_64",))]
extern "C" {
    pub(crate) fn dav1d_cdef_dir_8bpc_avx2(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut c_uint,
        bitdepth_max: c_int,
    ) -> c_int;
    pub(crate) fn dav1d_cdef_filter_8x8_8bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_8bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x4_8bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_8x8_8bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_8bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x4_8bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
#[cfg(all(
    feature = "asm",
    feature = "bitdepth_8",
    any(target_arch = "arm", target_arch = "aarch64"),
))]
extern "C" {
    pub(crate) fn dav1d_cdef_find_dir_8bpc_neon(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut c_uint,
        bitdepth_max: c_int,
    ) -> c_int;
    pub(crate) fn dav1d_cdef_padding4_8bpc_neon(
        tmp: *mut u16,
        src: *const DynPixel,
        src_stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        h: c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_padding8_8bpc_neon(
        tmp: *mut u16,
        src: *const DynPixel,
        src_stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        h: c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_filter4_8bpc_neon(
        dst: *mut DynPixel,
        dst_stride: ptrdiff_t,
        tmp: *const u16,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        h: c_int,
        edges: usize,
    );
    pub(crate) fn dav1d_cdef_filter8_8bpc_neon(
        dst: *mut DynPixel,
        dst_stride: ptrdiff_t,
        tmp: *const u16,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        h: c_int,
        edges: usize,
    );
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
#[cfg(all(
    feature = "asm",
    feature = "bitdepth_16",
    any(target_arch = "x86", target_arch = "x86_64"),
))]
extern "C" {
    pub(crate) fn dav1d_cdef_dir_16bpc_sse4(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut c_uint,
        bitdepth_max: c_int,
    ) -> c_int;
    pub(crate) fn dav1d_cdef_filter_4x4_16bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_16bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_8x8_16bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_dir_16bpc_ssse3(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut c_uint,
        bitdepth_max: c_int,
    ) -> c_int;
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
#[cfg(all(feature = "asm", feature = "bitdepth_16", target_arch = "x86_64",))]
extern "C" {
    pub(crate) fn dav1d_cdef_filter_4x4_16bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_16bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_8x8_16bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x4_16bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_16bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_8x8_16bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_dir_16bpc_avx2(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut c_uint,
        bitdepth_max: c_int,
    ) -> c_int;
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
#[cfg(all(
    feature = "asm",
    feature = "bitdepth_16",
    any(target_arch = "arm", target_arch = "aarch64"),
))]
extern "C" {
    pub(crate) fn dav1d_cdef_find_dir_16bpc_neon(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut c_uint,
        bitdepth_max: c_int,
    ) -> c_int;
    pub(crate) fn dav1d_cdef_padding4_16bpc_neon(
        tmp: *mut u16,
        src: *const DynPixel,
        src_stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        h: c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_padding8_16bpc_neon(
        tmp: *mut u16,
        src: *const DynPixel,
        src_stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        h: c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_filter4_16bpc_neon(
        dst: *mut DynPixel,
        dst_stride: ptrdiff_t,
        tmp: *const u16,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        h: c_int,
        edges: usize,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter8_16bpc_neon(
        dst: *mut DynPixel,
        dst_stride: ptrdiff_t,
        tmp: *const u16,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        h: c_int,
        edges: usize,
        bitdepth_max: c_int,
    );
}

#[inline]
pub unsafe fn constrain(diff: c_int, threshold: c_int, shift: c_int) -> c_int {
    let adiff = diff.abs();
    return apply_sign(
        cmp::min(adiff, cmp::max(0 as c_int, threshold - (adiff >> shift))),
        diff,
    );
}

#[inline]
pub unsafe fn fill(mut tmp: *mut i16, stride: ptrdiff_t, w: c_int, h: c_int) {
    let mut y = 0;
    while y < h {
        let mut x = 0;
        while x < w {
            *tmp.offset(x as isize) = i16::MIN;
            x += 1;
        }
        tmp = tmp.offset(stride as isize);
        y += 1;
    }
}

// TODO(perl): Temporarily pub until mod is deduplicated
pub(crate) unsafe fn padding<BD: BitDepth>(
    mut tmp: *mut i16,
    tmp_stride: ptrdiff_t,
    mut src: *const BD::Pixel,
    src_stride: ptrdiff_t,
    left: *const [BD::Pixel; 2],
    mut top: *const BD::Pixel,
    mut bottom: *const BD::Pixel,
    w: c_int,
    h: c_int,
    edges: CdefEdgeFlags,
) {
    let mut x_start = -(2 as c_int);
    let mut x_end = w + 2;
    let mut y_start = -(2 as c_int);
    let mut y_end = h + 2;
    if edges as c_uint & CDEF_HAVE_TOP as c_int as c_uint == 0 {
        fill(
            tmp.offset(-2).offset(-((2 * tmp_stride) as isize)),
            tmp_stride,
            w + 4,
            2 as c_int,
        );
        y_start = 0 as c_int;
    }
    if edges as c_uint & CDEF_HAVE_BOTTOM as c_int as c_uint == 0 {
        fill(
            tmp.offset((h as isize * tmp_stride) as isize)
                .offset(-(2 as c_int as isize)),
            tmp_stride,
            w + 4,
            2 as c_int,
        );
        y_end -= 2 as c_int;
    }
    if edges as c_uint & CDEF_HAVE_LEFT as c_int as c_uint == 0 {
        fill(
            tmp.offset((y_start as isize * tmp_stride) as isize)
                .offset(-(2 as c_int as isize)),
            tmp_stride,
            2 as c_int,
            y_end - y_start,
        );
        x_start = 0 as c_int;
    }
    if edges as c_uint & CDEF_HAVE_RIGHT as c_int as c_uint == 0 {
        fill(
            tmp.offset((y_start as isize * tmp_stride) as isize)
                .offset(w as isize),
            tmp_stride,
            2 as c_int,
            y_end - y_start,
        );
        x_end -= 2 as c_int;
    }
    let mut y = y_start;
    while y < 0 {
        let mut x = x_start;
        while x < x_end {
            *tmp.offset((x as isize + y as isize * tmp_stride) as isize) =
                (*top.offset(x as isize)).as_::<i16>();
            x += 1;
        }
        top = top.offset(BD::pxstride(src_stride as usize) as isize);
        y += 1;
    }
    let mut y_0 = 0;
    while y_0 < h {
        let mut x_0 = x_start;
        while x_0 < 0 {
            *tmp.offset((x_0 as isize + y_0 as isize * tmp_stride) as isize) =
                (*left.offset(y_0 as isize))[(2 + x_0) as usize].as_::<i16>();
            x_0 += 1;
        }
        y_0 += 1;
    }
    let mut y_1 = 0;
    while y_1 < h {
        let mut x_1 = if y_1 < h { 0 as c_int } else { x_start };
        while x_1 < x_end {
            *tmp.offset(x_1 as isize) = (*src.offset(x_1 as isize)).as_::<i16>();
            x_1 += 1;
        }
        src = src.offset(BD::pxstride(src_stride as usize) as isize);
        tmp = tmp.offset(tmp_stride as isize);
        y_1 += 1;
    }
    let mut y_2 = h;
    while y_2 < y_end {
        let mut x_2 = x_start;
        while x_2 < x_end {
            *tmp.offset(x_2 as isize) = (*bottom.offset(x_2 as isize)).as_::<i16>();
            x_2 += 1;
        }
        bottom = bottom.offset(BD::pxstride(src_stride as usize) as isize);
        tmp = tmp.offset(tmp_stride as isize);
        y_2 += 1;
    }
}
