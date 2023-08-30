use crate::include::common::bitdepth::DynPixel;
use crate::include::stddef::ptrdiff_t;
use crate::include::stdint::uint32_t;
use crate::include::stdint::uint8_t;
use crate::src::lf_mask::Av1FilterLUT;

pub type loopfilter_sb_fn = unsafe extern "C" fn(
    *mut DynPixel,
    ptrdiff_t,
    *const uint32_t,
    *const [uint8_t; 4],
    ptrdiff_t,
    *const Av1FilterLUT,
    libc::c_int,
    libc::c_int,
) -> ();

#[repr(C)]
pub struct Dav1dLoopFilterDSPContext {
    pub loop_filter_sb: [[loopfilter_sb_fn; 2]; 2],
}

// TODO(legare): Temporarily pub until init fns have been deduplicated.
#[cfg(all(
    feature = "asm",
    feature = "bitdepth_8",
    any(target_arch = "x86", target_arch = "x86_64"),
))]
extern "C" {
    pub(crate) fn dav1d_lpf_v_sb_uv_8bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_h_sb_uv_8bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_v_sb_y_8bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_h_sb_y_8bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_v_sb_uv_8bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_h_sb_uv_8bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_v_sb_y_8bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_h_sb_y_8bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_v_sb_uv_8bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_h_sb_uv_8bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_v_sb_y_8bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_h_sb_y_8bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
}

// TODO(legare): Temporarily pub until init fns have been deduplicated.
#[cfg(all(
    feature = "asm",
    feature = "bitdepth_8",
    any(target_arch = "arm", target_arch = "aarch64")
))]
extern "C" {
    pub(crate) fn dav1d_lpf_h_sb_uv_8bpc_neon(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_v_sb_y_8bpc_neon(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_h_sb_y_8bpc_neon(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_v_sb_uv_8bpc_neon(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
#[cfg(all(
    feature = "asm",
    feature = "bitdepth_16",
    any(target_arch = "x86", target_arch = "x86_64"),
))]
extern "C" {
    pub(crate) fn dav1d_lpf_v_sb_uv_16bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_h_sb_uv_16bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_v_sb_y_16bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_h_sb_y_16bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_v_sb_uv_16bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_h_sb_uv_16bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_v_sb_y_16bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_h_sb_y_16bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_v_sb_uv_16bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_h_sb_uv_16bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_v_sb_y_16bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_h_sb_y_16bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
#[cfg(all(
    feature = "asm",
    feature = "bitdepth_16",
    any(target_arch = "arm", target_arch = "aarch64"),
))]
extern "C" {
    pub(crate) fn dav1d_lpf_v_sb_uv_16bpc_neon(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_h_sb_uv_16bpc_neon(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_v_sb_y_16bpc_neon(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    pub(crate) fn dav1d_lpf_h_sb_y_16bpc_neon(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
        bitdepth_max: libc::c_int,
    );
}
