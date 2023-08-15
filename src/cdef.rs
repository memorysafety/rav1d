use crate::include::common::intops::apply_sign;
use crate::include::common::intops::imax;
use crate::include::common::intops::imin;
use crate::include::stddef::ptrdiff_t;
#[cfg(all(
    feature = "asm",
    feature = "bitdepth_8",
    any(target_arch = "arm", target_arch = "aarch64")
))]
use crate::include::stddef::size_t;
use crate::include::stdint::int16_t;
#[cfg(all(
    feature = "asm",
    feature = "bitdepth_8",
    any(target_arch = "arm", target_arch = "aarch64")
))]
use crate::include::stdint::uint16_t;

pub type CdefEdgeFlags = libc::c_uint;
pub const CDEF_HAVE_BOTTOM: CdefEdgeFlags = 8;
pub const CDEF_HAVE_TOP: CdefEdgeFlags = 4;
pub const CDEF_HAVE_RIGHT: CdefEdgeFlags = 2;
pub const CDEF_HAVE_LEFT: CdefEdgeFlags = 1;

pub type pixel = libc::c_void;
pub type const_left_pixel_row_2px = *const libc::c_void; // *const [pixel; 2]
pub type cdef_fn = unsafe extern "C" fn(
    *mut pixel,
    ptrdiff_t,
    const_left_pixel_row_2px,
    *const pixel,
    *const pixel,
    libc::c_int,
    libc::c_int,
    libc::c_int,
    libc::c_int,
    CdefEdgeFlags,
) -> ();
pub type cdef_dir_fn =
    unsafe extern "C" fn(*const pixel, ptrdiff_t, *mut libc::c_uint) -> libc::c_int;
#[derive(Copy, Clone)]
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
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: const_left_pixel_row_2px,
        top: *const pixel,
        bottom: *const pixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_8bpc_ssse3(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: const_left_pixel_row_2px,
        top: *const pixel,
        bottom: *const pixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_filter_4x4_8bpc_ssse3(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: const_left_pixel_row_2px,
        top: *const pixel,
        bottom: *const pixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_dir_8bpc_sse4(
        dst: *const pixel,
        dst_stride: ptrdiff_t,
        var: *mut libc::c_uint,
    ) -> libc::c_int;
    pub(crate) fn dav1d_cdef_filter_8x8_8bpc_sse4(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: const_left_pixel_row_2px,
        top: *const pixel,
        bottom: *const pixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_8bpc_sse4(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: const_left_pixel_row_2px,
        top: *const pixel,
        bottom: *const pixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_filter_4x4_8bpc_sse4(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: const_left_pixel_row_2px,
        top: *const pixel,
        bottom: *const pixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_dir_8bpc_avx2(
        dst: *const pixel,
        dst_stride: ptrdiff_t,
        var: *mut libc::c_uint,
    ) -> libc::c_int;
    pub(crate) fn dav1d_cdef_filter_8x8_8bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: const_left_pixel_row_2px,
        top: *const pixel,
        bottom: *const pixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_8bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: const_left_pixel_row_2px,
        top: *const pixel,
        bottom: *const pixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_filter_4x4_8bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: const_left_pixel_row_2px,
        top: *const pixel,
        bottom: *const pixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_filter_8x8_8bpc_avx512icl(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: const_left_pixel_row_2px,
        top: *const pixel,
        bottom: *const pixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_8bpc_avx512icl(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: const_left_pixel_row_2px,
        top: *const pixel,
        bottom: *const pixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_filter_4x4_8bpc_avx512icl(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: const_left_pixel_row_2px,
        top: *const pixel,
        bottom: *const pixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_8bpc_sse2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: const_left_pixel_row_2px,
        top: *const pixel,
        bottom: *const pixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_dir_8bpc_ssse3(
        dst: *const pixel,
        dst_stride: ptrdiff_t,
        var: *mut libc::c_uint,
    ) -> libc::c_int;
    pub(crate) fn dav1d_cdef_filter_4x4_8bpc_sse2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: const_left_pixel_row_2px,
        top: *const pixel,
        bottom: *const pixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_filter_8x8_8bpc_sse2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: const_left_pixel_row_2px,
        top: *const pixel,
        bottom: *const pixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
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
        dst: *const pixel,
        dst_stride: ptrdiff_t,
        var: *mut libc::c_uint,
    ) -> libc::c_int;
    pub(crate) fn dav1d_cdef_padding4_8bpc_neon(
        tmp: *mut uint16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        left: *const [pixel; 2],
        top: *const pixel,
        bottom: *const pixel,
        h: libc::c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_padding8_8bpc_neon(
        tmp: *mut uint16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        left: *const [pixel; 2],
        top: *const pixel,
        bottom: *const pixel,
        h: libc::c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_filter4_8bpc_neon(
        dst: *mut pixel,
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
        dst: *mut pixel,
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

#[inline]
pub unsafe extern "C" fn constrain(
    diff: libc::c_int,
    threshold: libc::c_int,
    shift: libc::c_int,
) -> libc::c_int {
    let adiff = diff.abs();
    return apply_sign(
        imin(adiff, imax(0 as libc::c_int, threshold - (adiff >> shift))),
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
            *tmp.offset(x as isize) = (-(32767 as libc::c_int) - 1) as int16_t;
            x += 1;
        }
        tmp = tmp.offset(stride as isize);
        y += 1;
    }
}
