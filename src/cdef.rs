use crate::include::common::bitdepth::DynPixel;
use crate::include::common::bitdepth::LeftPixelRow2px;
use crate::include::common::intops::apply_sign;
use crate::include::common::intops::imin;
use crate::include::stddef::ptrdiff_t;
#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
use crate::include::stddef::size_t;
use crate::include::stdint::int16_t;
#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
use crate::include::stdint::uint16_t;

pub type CdefEdgeFlags = libc::c_uint;
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
    libc::c_int,
    libc::c_int,
    libc::c_int,
    libc::c_int,
    CdefEdgeFlags,
    libc::c_int,
) -> ();
pub type cdef_dir_fn =
    unsafe extern "C" fn(*const DynPixel, ptrdiff_t, *mut libc::c_uint, libc::c_int) -> libc::c_int;
#[repr(C)]
pub struct Dav1dCdefDSPContext {
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
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_8bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x4_8bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_cdef_dir_8bpc_sse4(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut libc::c_uint,
        bitdepth_max: libc::c_int,
    ) -> libc::c_int;
    pub(crate) fn dav1d_cdef_filter_8x8_8bpc_sse4(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_8bpc_sse4(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x4_8bpc_sse4(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_8bpc_sse2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_cdef_dir_8bpc_ssse3(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut libc::c_uint,
        bitdepth_max: libc::c_int,
    ) -> libc::c_int;
    pub(crate) fn dav1d_cdef_filter_4x4_8bpc_sse2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_cdef_filter_8x8_8bpc_sse2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
#[cfg(all(feature = "asm", feature = "bitdepth_8", target_arch = "x86_64",))]
extern "C" {
    pub(crate) fn dav1d_cdef_dir_8bpc_avx2(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut libc::c_uint,
        bitdepth_max: libc::c_int,
    ) -> libc::c_int;
    pub(crate) fn dav1d_cdef_filter_8x8_8bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_8bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x4_8bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_cdef_filter_8x8_8bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_8bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x4_8bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
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
        var: *mut libc::c_uint,
        bitdepth_max: libc::c_int,
    ) -> libc::c_int;
    pub(crate) fn dav1d_cdef_padding4_8bpc_neon(
        tmp: *mut uint16_t,
        src: *const DynPixel,
        src_stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        h: libc::c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_padding8_8bpc_neon(
        tmp: *mut uint16_t,
        src: *const DynPixel,
        src_stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        h: libc::c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_filter4_8bpc_neon(
        dst: *mut DynPixel,
        dst_stride: ptrdiff_t,
        tmp: *const uint16_t,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        h: libc::c_int,
        edges: size_t,
    );
    pub(crate) fn dav1d_cdef_filter8_8bpc_neon(
        dst: *mut DynPixel,
        dst_stride: ptrdiff_t,
        tmp: *const uint16_t,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        h: libc::c_int,
        edges: size_t,
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
        var: *mut libc::c_uint,
        bitdepth_max: libc::c_int,
    ) -> libc::c_int;
    pub(crate) fn dav1d_cdef_filter_4x4_16bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_16bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_cdef_filter_8x8_16bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_cdef_dir_16bpc_ssse3(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut libc::c_uint,
        bitdepth_max: libc::c_int,
    ) -> libc::c_int;
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
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_16bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_cdef_filter_8x8_16bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x4_16bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_16bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_cdef_filter_8x8_16bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_cdef_dir_16bpc_avx2(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut libc::c_uint,
        bitdepth_max: libc::c_int,
    ) -> libc::c_int;
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
        var: *mut libc::c_uint,
        bitdepth_max: libc::c_int,
    ) -> libc::c_int;
    pub(crate) fn dav1d_cdef_padding4_16bpc_neon(
        tmp: *mut uint16_t,
        src: *const DynPixel,
        src_stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        h: libc::c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_padding8_16bpc_neon(
        tmp: *mut uint16_t,
        src: *const DynPixel,
        src_stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        h: libc::c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_filter4_16bpc_neon(
        dst: *mut DynPixel,
        dst_stride: ptrdiff_t,
        tmp: *const uint16_t,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        h: libc::c_int,
        edges: size_t,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_cdef_filter8_16bpc_neon(
        dst: *mut DynPixel,
        dst_stride: ptrdiff_t,
        tmp: *const uint16_t,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        h: libc::c_int,
        edges: size_t,
        bitdepth_max: libc::c_int,
    );
}

#[inline]
pub unsafe extern "C" fn constrain(
    diff: libc::c_int,
    threshold: libc::c_int,
    shift: libc::c_int,
) -> libc::c_int {
    let adiff = diff.abs();
    return apply_sign(
        imin(
            adiff,
            std::cmp::max(0 as libc::c_int, threshold - (adiff >> shift)),
        ),
        diff,
    );
}

#[inline]
pub unsafe extern "C" fn fill(
    mut tmp: *mut int16_t,
    stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
) {
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
