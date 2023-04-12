use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
use cfg_if::cfg_if;

extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static dav1d_sm_weights: [uint8_t; 128];
    static dav1d_dr_intra_derivative: [uint16_t; 44];
    static dav1d_filter_intra_taps: [[int8_t; 64]; 5];
}

#[cfg(feature = "asm")]
extern "C" {
    static mut dav1d_cpu_flags: libc::c_uint;
    static mut dav1d_cpu_flags_mask: libc::c_uint;
}

#[cfg(all(
    feature = "asm",
    any(target_arch = "x86", target_arch = "x86_64"),
))]
extern "C" {
    fn dav1d_ipred_dc_16bpc_ssse3(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_smooth_h_16bpc_avx512icl(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_dc_128_16bpc_ssse3(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_dc_top_16bpc_ssse3(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_dc_left_16bpc_ssse3(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_h_16bpc_ssse3(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_v_16bpc_ssse3(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_paeth_16bpc_ssse3(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_smooth_16bpc_ssse3(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_smooth_h_16bpc_ssse3(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_smooth_v_16bpc_ssse3(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_filter_16bpc_ssse3(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_cfl_16bpc_ssse3(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        ac: *const int16_t,
        alpha: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_cfl_128_16bpc_ssse3(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        ac: *const int16_t,
        alpha: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_cfl_top_16bpc_ssse3(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        ac: *const int16_t,
        alpha: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_cfl_left_16bpc_ssse3(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        ac: *const int16_t,
        alpha: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_cfl_ac_420_16bpc_ssse3(
        ac: *mut int16_t,
        y: *const pixel,
        stride: ptrdiff_t,
        w_pad: libc::c_int,
        h_pad: libc::c_int,
        cw: libc::c_int,
        ch: libc::c_int,
    );
    fn dav1d_ipred_cfl_ac_422_16bpc_ssse3(
        ac: *mut int16_t,
        y: *const pixel,
        stride: ptrdiff_t,
        w_pad: libc::c_int,
        h_pad: libc::c_int,
        cw: libc::c_int,
        ch: libc::c_int,
    );
    fn dav1d_ipred_cfl_ac_444_16bpc_ssse3(
        ac: *mut int16_t,
        y: *const pixel,
        stride: ptrdiff_t,
        w_pad: libc::c_int,
        h_pad: libc::c_int,
        cw: libc::c_int,
        ch: libc::c_int,
    );
    fn dav1d_pal_pred_16bpc_ssse3(
        dst: *mut pixel,
        stride: ptrdiff_t,
        pal: *const uint16_t,
        idx: *const uint8_t,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_ipred_dc_16bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_dc_128_16bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_dc_top_16bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_smooth_v_16bpc_avx512icl(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_filter_16bpc_avx512icl(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_pal_pred_16bpc_avx512icl(
        dst: *mut pixel,
        stride: ptrdiff_t,
        pal: *const uint16_t,
        idx: *const uint8_t,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_ipred_dc_left_16bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_h_16bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_v_16bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_paeth_16bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_smooth_16bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_smooth_h_16bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_smooth_v_16bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_z1_16bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_z2_16bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_z3_16bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_filter_16bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_cfl_16bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        ac: *const int16_t,
        alpha: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_cfl_128_16bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        ac: *const int16_t,
        alpha: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_cfl_top_16bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        ac: *const int16_t,
        alpha: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_cfl_left_16bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        ac: *const int16_t,
        alpha: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_cfl_ac_420_16bpc_avx2(
        ac: *mut int16_t,
        y: *const pixel,
        stride: ptrdiff_t,
        w_pad: libc::c_int,
        h_pad: libc::c_int,
        cw: libc::c_int,
        ch: libc::c_int,
    );
    fn dav1d_ipred_cfl_ac_422_16bpc_avx2(
        ac: *mut int16_t,
        y: *const pixel,
        stride: ptrdiff_t,
        w_pad: libc::c_int,
        h_pad: libc::c_int,
        cw: libc::c_int,
        ch: libc::c_int,
    );
    fn dav1d_ipred_cfl_ac_444_16bpc_avx2(
        ac: *mut int16_t,
        y: *const pixel,
        stride: ptrdiff_t,
        w_pad: libc::c_int,
        h_pad: libc::c_int,
        cw: libc::c_int,
        ch: libc::c_int,
    );
    fn dav1d_pal_pred_16bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        pal: *const uint16_t,
        idx: *const uint8_t,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_ipred_paeth_16bpc_avx512icl(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_smooth_16bpc_avx512icl(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
}

#[cfg(all(
    feature = "asm",
    any(target_arch = "arm", target_arch = "aarch64"),
))]
extern "C" {
    fn dav1d_ipred_filter_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_dc_128_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_dc_top_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_dc_left_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_h_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_v_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_smooth_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_smooth_v_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_smooth_h_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_paeth_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_dc_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        angle: libc::c_int,
        max_width: libc::c_int,
        max_height: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_cfl_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        ac: *const int16_t,
        alpha: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_cfl_128_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        ac: *const int16_t,
        alpha: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_cfl_top_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        ac: *const int16_t,
        alpha: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_cfl_left_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        topleft: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        ac: *const int16_t,
        alpha: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_cfl_ac_420_16bpc_neon(
        ac: *mut int16_t,
        y: *const pixel,
        stride: ptrdiff_t,
        w_pad: libc::c_int,
        h_pad: libc::c_int,
        cw: libc::c_int,
        ch: libc::c_int,
    );
    fn dav1d_ipred_cfl_ac_422_16bpc_neon(
        ac: *mut int16_t,
        y: *const pixel,
        stride: ptrdiff_t,
        w_pad: libc::c_int,
        h_pad: libc::c_int,
        cw: libc::c_int,
        ch: libc::c_int,
    );
    fn dav1d_ipred_cfl_ac_444_16bpc_neon(
        ac: *mut int16_t,
        y: *const pixel,
        stride: ptrdiff_t,
        w_pad: libc::c_int,
        h_pad: libc::c_int,
        cw: libc::c_int,
        ch: libc::c_int,
    );
    fn dav1d_pal_pred_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        pal: *const uint16_t,
        idx: *const uint8_t,
        w: libc::c_int,
        h: libc::c_int,
    );
}

pub type pixel = uint16_t;

use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I422;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;

use crate::src::levels::FILTER_PRED;
use crate::src::levels::Z3_PRED;
use crate::src::levels::Z2_PRED;
use crate::src::levels::Z1_PRED;
use crate::src::levels::DC_128_PRED;
use crate::src::levels::TOP_DC_PRED;
use crate::src::levels::LEFT_DC_PRED;




use crate::src::levels::PAETH_PRED;
use crate::src::levels::SMOOTH_H_PRED;
use crate::src::levels::SMOOTH_V_PRED;
use crate::src::levels::SMOOTH_PRED;






use crate::src::levels::HOR_PRED;
use crate::src::levels::VERT_PRED;
use crate::src::levels::DC_PRED;
pub type angular_ipred_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type cfl_ac_fn = Option::<
    unsafe extern "C" fn(
        *mut int16_t,
        *const pixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type cfl_pred_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        libc::c_int,
        libc::c_int,
        *const int16_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type pal_pred_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const uint16_t,
        *const uint8_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dIntraPredDSPContext {
    pub intra_pred: [angular_ipred_fn; 14],
    pub cfl_ac: [cfl_ac_fn; 3],
    pub cfl_pred: [cfl_pred_fn; 6],
    pub pal_pred: pal_pred_fn,
}
pub const DAV1D_X86_CPU_FLAG_AVX512ICL: CpuFlags = 16;
pub const DAV1D_X86_CPU_FLAG_SSE2: CpuFlags = 1;
pub const DAV1D_X86_CPU_FLAG_AVX2: CpuFlags = 8;
pub const DAV1D_X86_CPU_FLAG_SSSE3: CpuFlags = 2;
pub type CpuFlags = libc::c_uint;
pub const DAV1D_X86_CPU_FLAG_SLOW_GATHER: CpuFlags = 32;
pub const DAV1D_X86_CPU_FLAG_SSE41: CpuFlags = 4;
#[inline]
unsafe extern "C" fn ctz(mask: libc::c_uint) -> libc::c_int {
    return mask.trailing_zeros() as i32;
}
#[inline]
unsafe extern "C" fn imax(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn imin(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn iclip(
    v: libc::c_int,
    min: libc::c_int,
    max: libc::c_int,
) -> libc::c_int {
    return if v < min { min } else if v > max { max } else { v };
}
#[inline]
unsafe extern "C" fn apply_sign(v: libc::c_int, s: libc::c_int) -> libc::c_int {
    return if s < 0 as libc::c_int { -v } else { v };
}
#[inline]
unsafe extern "C" fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 != 0 {
        unreachable!();
    }
    return x >> 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn pixel_set(dst: *mut pixel, val: libc::c_int, num: libc::c_int) {
    let mut n: libc::c_int = 0 as libc::c_int;
    while n < num {
        *dst.offset(n as isize) = val as pixel;
        n += 1;
    }
}
#[inline(never)]
unsafe extern "C" fn splat_dc(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    width: libc::c_int,
    height: libc::c_int,
    dc: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    if !(dc <= bitdepth_max) {
        unreachable!();
    }
    let dcN: uint64_t = (dc as libc::c_ulonglong)
        .wrapping_mul(0x1000100010001 as libc::c_ulonglong) as uint64_t;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < height {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            *(&mut *dst.offset(x as isize) as *mut pixel as *mut uint64_t) = dcN;
            x = (x as libc::c_ulong)
                .wrapping_add(
                    ::core::mem::size_of::<uint64_t>() as libc::c_ulong
                        >> 1 as libc::c_int,
                ) as libc::c_int as libc::c_int;
        }
        dst = dst.offset(PXSTRIDE(stride) as isize);
        y += 1;
    }
}
#[inline(never)]
unsafe extern "C" fn cfl_pred(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    width: libc::c_int,
    height: libc::c_int,
    dc: libc::c_int,
    mut ac: *const int16_t,
    alpha: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < height {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            let diff: libc::c_int = alpha * *ac.offset(x as isize) as libc::c_int;
            *dst
                .offset(
                    x as isize,
                ) = iclip(
                dc + apply_sign(abs(diff) + 32 as libc::c_int >> 6 as libc::c_int, diff),
                0 as libc::c_int,
                bitdepth_max,
            ) as pixel;
            x += 1;
        }
        ac = ac.offset(width as isize);
        dst = dst.offset(PXSTRIDE(stride) as isize);
        y += 1;
    }
}
unsafe extern "C" fn dc_gen_top(
    topleft: *const pixel,
    width: libc::c_int,
) -> libc::c_uint {
    let mut dc: libc::c_uint = (width >> 1 as libc::c_int) as libc::c_uint;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < width {
        dc = dc
            .wrapping_add(
                *topleft.offset((1 as libc::c_int + i) as isize) as libc::c_uint,
            );
        i += 1;
    }
    return dc >> ctz(width as libc::c_uint);
}
unsafe extern "C" fn ipred_dc_top_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    _a: libc::c_int,
    _max_width: libc::c_int,
    _max_height: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    splat_dc(
        dst,
        stride,
        width,
        height,
        dc_gen_top(topleft, width) as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn ipred_cfl_top_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    mut ac: *const int16_t,
    alpha: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    cfl_pred(
        dst,
        stride,
        width,
        height,
        dc_gen_top(topleft, width) as libc::c_int,
        ac,
        alpha,
        bitdepth_max,
    );
}
unsafe extern "C" fn dc_gen_left(
    topleft: *const pixel,
    height: libc::c_int,
) -> libc::c_uint {
    let mut dc: libc::c_uint = (height >> 1 as libc::c_int) as libc::c_uint;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < height {
        dc = dc
            .wrapping_add(
                *topleft.offset(-(1 as libc::c_int + i) as isize) as libc::c_uint,
            );
        i += 1;
    }
    return dc >> ctz(height as libc::c_uint);
}
unsafe extern "C" fn ipred_dc_left_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    _a: libc::c_int,
    _max_width: libc::c_int,
    _max_height: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    splat_dc(
        dst,
        stride,
        width,
        height,
        dc_gen_left(topleft, height) as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn ipred_cfl_left_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    mut ac: *const int16_t,
    alpha: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let dc: libc::c_uint = dc_gen_left(topleft, height);
    cfl_pred(dst, stride, width, height, dc as libc::c_int, ac, alpha, bitdepth_max);
}
unsafe extern "C" fn dc_gen(
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
) -> libc::c_uint {
    let mut dc: libc::c_uint = (width + height >> 1 as libc::c_int) as libc::c_uint;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < width {
        dc = dc
            .wrapping_add(
                *topleft.offset((i + 1 as libc::c_int) as isize) as libc::c_uint,
            );
        i += 1;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < height {
        dc = dc
            .wrapping_add(
                *topleft.offset(-(i_0 + 1 as libc::c_int) as isize) as libc::c_uint,
            );
        i_0 += 1;
    }
    dc >>= ctz((width + height) as libc::c_uint);
    if width != height {
        dc = dc
            .wrapping_mul(
                (if width > height * 2 as libc::c_int
                    || height > width * 2 as libc::c_int
                {
                    0x6667 as libc::c_int
                } else {
                    0xaaab as libc::c_int
                }) as libc::c_uint,
            );
        dc >>= 17 as libc::c_int;
    }
    return dc;
}
unsafe extern "C" fn ipred_dc_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    _a: libc::c_int,
    _max_width: libc::c_int,
    _max_height: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    splat_dc(
        dst,
        stride,
        width,
        height,
        dc_gen(topleft, width, height) as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn ipred_cfl_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    mut ac: *const int16_t,
    alpha: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let mut dc: libc::c_uint = dc_gen(topleft, width, height);
    cfl_pred(dst, stride, width, height, dc as libc::c_int, ac, alpha, bitdepth_max);
}
unsafe extern "C" fn ipred_dc_128_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    _topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    _a: libc::c_int,
    _max_width: libc::c_int,
    _max_height: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let dc: libc::c_int = bitdepth_max + 1 as libc::c_int >> 1 as libc::c_int;
    splat_dc(dst, stride, width, height, dc, bitdepth_max);
}
unsafe extern "C" fn ipred_cfl_128_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    _topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    mut ac: *const int16_t,
    alpha: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let dc: libc::c_int = bitdepth_max + 1 as libc::c_int >> 1 as libc::c_int;
    cfl_pred(dst, stride, width, height, dc, ac, alpha, bitdepth_max);
}
unsafe extern "C" fn ipred_v_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    _a: libc::c_int,
    _max_width: libc::c_int,
    _max_height: libc::c_int,
    _bitdepth_max: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < height {
        memcpy(
            dst as *mut libc::c_void,
            topleft.offset(1 as libc::c_int as isize) as *const libc::c_void,
            (width << 1 as libc::c_int) as libc::c_ulong,
        );
        dst = dst.offset(PXSTRIDE(stride) as isize);
        y += 1;
    }
}
unsafe extern "C" fn ipred_h_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    _a: libc::c_int,
    _max_width: libc::c_int,
    _max_height: libc::c_int,
    _bitdepth_max: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < height {
        pixel_set(
            dst,
            *topleft.offset(-(1 as libc::c_int + y) as isize) as libc::c_int,
            width,
        );
        dst = dst.offset(PXSTRIDE(stride) as isize);
        y += 1;
    }
}
unsafe extern "C" fn ipred_paeth_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    tl_ptr: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    _a: libc::c_int,
    _max_width: libc::c_int,
    _max_height: libc::c_int,
    _bitdepth_max: libc::c_int,
) {
    let topleft: libc::c_int = *tl_ptr.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < height {
        let left: libc::c_int = *tl_ptr.offset(-(y + 1 as libc::c_int) as isize)
            as libc::c_int;
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            let top: libc::c_int = *tl_ptr.offset((1 as libc::c_int + x) as isize)
                as libc::c_int;
            let base: libc::c_int = left + top - topleft;
            let ldiff: libc::c_int = abs(left - base);
            let tdiff: libc::c_int = abs(top - base);
            let tldiff: libc::c_int = abs(topleft - base);
            *dst
                .offset(
                    x as isize,
                ) = (if ldiff <= tdiff && ldiff <= tldiff {
                left
            } else if tdiff <= tldiff {
                top
            } else {
                topleft
            }) as pixel;
            x += 1;
        }
        dst = dst.offset(PXSTRIDE(stride) as isize);
        y += 1;
    }
}
unsafe extern "C" fn ipred_smooth_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    _a: libc::c_int,
    _max_width: libc::c_int,
    _max_height: libc::c_int,
    _bitdepth_max: libc::c_int,
) {
    let weights_hor: *const uint8_t = &*dav1d_sm_weights.as_ptr().offset(width as isize)
        as *const uint8_t;
    let weights_ver: *const uint8_t = &*dav1d_sm_weights.as_ptr().offset(height as isize)
        as *const uint8_t;
    let right: libc::c_int = *topleft.offset(width as isize) as libc::c_int;
    let bottom: libc::c_int = *topleft.offset(-height as isize) as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < height {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            let pred: libc::c_int = *weights_ver.offset(y as isize) as libc::c_int
                * *topleft.offset((1 as libc::c_int + x) as isize) as libc::c_int
                + (256 as libc::c_int - *weights_ver.offset(y as isize) as libc::c_int)
                    * bottom
                + *weights_hor.offset(x as isize) as libc::c_int
                    * *topleft.offset(-(1 as libc::c_int + y) as isize) as libc::c_int
                + (256 as libc::c_int - *weights_hor.offset(x as isize) as libc::c_int)
                    * right;
            *dst
                .offset(
                    x as isize,
                ) = (pred + 256 as libc::c_int >> 9 as libc::c_int) as pixel;
            x += 1;
        }
        dst = dst.offset(PXSTRIDE(stride) as isize);
        y += 1;
    }
}
unsafe extern "C" fn ipred_smooth_v_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    _a: libc::c_int,
    _max_width: libc::c_int,
    _max_height: libc::c_int,
    _bitdepth_max: libc::c_int,
) {
    let weights_ver: *const uint8_t = &*dav1d_sm_weights.as_ptr().offset(height as isize)
        as *const uint8_t;
    let bottom: libc::c_int = *topleft.offset(-height as isize) as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < height {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            let pred: libc::c_int = *weights_ver.offset(y as isize) as libc::c_int
                * *topleft.offset((1 as libc::c_int + x) as isize) as libc::c_int
                + (256 as libc::c_int - *weights_ver.offset(y as isize) as libc::c_int)
                    * bottom;
            *dst
                .offset(
                    x as isize,
                ) = (pred + 128 as libc::c_int >> 8 as libc::c_int) as pixel;
            x += 1;
        }
        dst = dst.offset(PXSTRIDE(stride) as isize);
        y += 1;
    }
}
unsafe extern "C" fn ipred_smooth_h_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    _a: libc::c_int,
    _max_width: libc::c_int,
    _max_height: libc::c_int,
    _bitdepth_max: libc::c_int,
) {
    let weights_hor: *const uint8_t = &*dav1d_sm_weights.as_ptr().offset(width as isize)
        as *const uint8_t;
    let right: libc::c_int = *topleft.offset(width as isize) as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < height {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            let pred: libc::c_int = *weights_hor.offset(x as isize) as libc::c_int
                * *topleft.offset(-(y + 1 as libc::c_int) as isize) as libc::c_int
                + (256 as libc::c_int - *weights_hor.offset(x as isize) as libc::c_int)
                    * right;
            *dst
                .offset(
                    x as isize,
                ) = (pred + 128 as libc::c_int >> 8 as libc::c_int) as pixel;
            x += 1;
        }
        dst = dst.offset(PXSTRIDE(stride) as isize);
        y += 1;
    }
}
#[inline(never)]
unsafe extern "C" fn get_filter_strength(
    wh: libc::c_int,
    angle: libc::c_int,
    is_sm: libc::c_int,
) -> libc::c_int {
    if is_sm != 0 {
        if wh <= 8 as libc::c_int {
            if angle >= 64 as libc::c_int {
                return 2 as libc::c_int;
            }
            if angle >= 40 as libc::c_int {
                return 1 as libc::c_int;
            }
        } else if wh <= 16 as libc::c_int {
            if angle >= 48 as libc::c_int {
                return 2 as libc::c_int;
            }
            if angle >= 20 as libc::c_int {
                return 1 as libc::c_int;
            }
        } else if wh <= 24 as libc::c_int {
            if angle >= 4 as libc::c_int {
                return 3 as libc::c_int;
            }
        } else {
            return 3 as libc::c_int
        }
    } else if wh <= 8 as libc::c_int {
        if angle >= 56 as libc::c_int {
            return 1 as libc::c_int;
        }
    } else if wh <= 16 as libc::c_int {
        if angle >= 40 as libc::c_int {
            return 1 as libc::c_int;
        }
    } else if wh <= 24 as libc::c_int {
        if angle >= 32 as libc::c_int {
            return 3 as libc::c_int;
        }
        if angle >= 16 as libc::c_int {
            return 2 as libc::c_int;
        }
        if angle >= 8 as libc::c_int {
            return 1 as libc::c_int;
        }
    } else if wh <= 32 as libc::c_int {
        if angle >= 32 as libc::c_int {
            return 3 as libc::c_int;
        }
        if angle >= 4 as libc::c_int {
            return 2 as libc::c_int;
        }
        return 1 as libc::c_int;
    } else {
        return 3 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[inline(never)]
unsafe extern "C" fn filter_edge(
    out: *mut pixel,
    sz: libc::c_int,
    lim_from: libc::c_int,
    lim_to: libc::c_int,
    in_0: *const pixel,
    from: libc::c_int,
    to: libc::c_int,
    strength: libc::c_int,
) {
    static mut kernel: [[uint8_t; 5]; 3] = [
        [
            0 as libc::c_int as uint8_t,
            4 as libc::c_int as uint8_t,
            8 as libc::c_int as uint8_t,
            4 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
        ],
        [
            0 as libc::c_int as uint8_t,
            5 as libc::c_int as uint8_t,
            6 as libc::c_int as uint8_t,
            5 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
        ],
        [
            2 as libc::c_int as uint8_t,
            4 as libc::c_int as uint8_t,
            4 as libc::c_int as uint8_t,
            4 as libc::c_int as uint8_t,
            2 as libc::c_int as uint8_t,
        ],
    ];
    if !(strength > 0 as libc::c_int) {
        unreachable!();
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < imin(sz, lim_from) {
        *out
            .offset(
                i as isize,
            ) = *in_0.offset(iclip(i, from, to - 1 as libc::c_int) as isize);
        i += 1;
    }
    while i < imin(lim_to, sz) {
        let mut s: libc::c_int = 0 as libc::c_int;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 5 as libc::c_int {
            s
                += *in_0
                    .offset(
                        iclip(i - 2 as libc::c_int + j, from, to - 1 as libc::c_int)
                            as isize,
                    ) as libc::c_int
                    * kernel[(strength - 1 as libc::c_int) as usize][j as usize]
                        as libc::c_int;
            j += 1;
        }
        *out.offset(i as isize) = (s + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
        i += 1;
    }
    while i < sz {
        *out
            .offset(
                i as isize,
            ) = *in_0.offset(iclip(i, from, to - 1 as libc::c_int) as isize);
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn get_upsample(
    wh: libc::c_int,
    angle: libc::c_int,
    is_sm: libc::c_int,
) -> libc::c_int {
    return (angle < 40 as libc::c_int && wh <= 16 as libc::c_int >> is_sm)
        as libc::c_int;
}
#[inline(never)]
unsafe extern "C" fn upsample_edge(
    out: *mut pixel,
    hsz: libc::c_int,
    in_0: *const pixel,
    from: libc::c_int,
    to: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    static mut kernel: [int8_t; 4] = [
        -(1 as libc::c_int) as int8_t,
        9 as libc::c_int as int8_t,
        9 as libc::c_int as int8_t,
        -(1 as libc::c_int) as int8_t,
    ];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < hsz - 1 as libc::c_int {
        *out
            .offset(
                (i * 2 as libc::c_int) as isize,
            ) = *in_0.offset(iclip(i, from, to - 1 as libc::c_int) as isize);
        let mut s: libc::c_int = 0 as libc::c_int;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            s
                += *in_0
                    .offset(
                        iclip(i + j - 1 as libc::c_int, from, to - 1 as libc::c_int)
                            as isize,
                    ) as libc::c_int * kernel[j as usize] as libc::c_int;
            j += 1;
        }
        *out
            .offset(
                (i * 2 as libc::c_int + 1 as libc::c_int) as isize,
            ) = iclip(
            s + 8 as libc::c_int >> 4 as libc::c_int,
            0 as libc::c_int,
            bitdepth_max,
        ) as pixel;
        i += 1;
    }
    *out
        .offset(
            (i * 2 as libc::c_int) as isize,
        ) = *in_0.offset(iclip(i, from, to - 1 as libc::c_int) as isize);
}
unsafe extern "C" fn ipred_z1_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft_in: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    mut angle: libc::c_int,
    _max_width: libc::c_int,
    _max_height: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let is_sm: libc::c_int = angle >> 9 as libc::c_int & 0x1 as libc::c_int;
    let enable_intra_edge_filter: libc::c_int = angle >> 10 as libc::c_int;
    angle &= 511 as libc::c_int;
    if !(angle < 90 as libc::c_int) {
        unreachable!();
    }
    let mut dx: libc::c_int = dav1d_dr_intra_derivative[(angle >> 1 as libc::c_int)
        as usize] as libc::c_int;
    let mut top_out: [pixel; 128] = [0; 128];
    let mut top: *const pixel = 0 as *const pixel;
    let mut max_base_x: libc::c_int = 0;
    let upsample_above: libc::c_int = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, 90 as libc::c_int - angle, is_sm)
    } else {
        0 as libc::c_int
    };
    if upsample_above != 0 {
        upsample_edge(
            top_out.as_mut_ptr(),
            width + height,
            &*topleft_in.offset(1 as libc::c_int as isize),
            -(1 as libc::c_int),
            width + imin(width, height),
            bitdepth_max,
        );
        top = top_out.as_mut_ptr();
        max_base_x = 2 as libc::c_int * (width + height) - 2 as libc::c_int;
        dx <<= 1 as libc::c_int;
    } else {
        let filter_strength: libc::c_int = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, 90 as libc::c_int - angle, is_sm)
        } else {
            0 as libc::c_int
        };
        if filter_strength != 0 {
            filter_edge(
                top_out.as_mut_ptr(),
                width + height,
                0 as libc::c_int,
                width + height,
                &*topleft_in.offset(1 as libc::c_int as isize),
                -(1 as libc::c_int),
                width + imin(width, height),
                filter_strength,
            );
            top = top_out.as_mut_ptr();
            max_base_x = width + height - 1 as libc::c_int;
        } else {
            top = &*topleft_in.offset(1 as libc::c_int as isize) as *const pixel;
            max_base_x = width + imin(width, height) - 1 as libc::c_int;
        }
    }
    let base_inc: libc::c_int = 1 as libc::c_int + upsample_above;
    let mut y: libc::c_int = 0 as libc::c_int;
    let mut xpos: libc::c_int = dx;
    while y < height {
        let frac: libc::c_int = xpos & 0x3e as libc::c_int;
        let mut x: libc::c_int = 0 as libc::c_int;
        let mut base: libc::c_int = xpos >> 6 as libc::c_int;
        while x < width {
            if base < max_base_x {
                let v: libc::c_int = *top.offset(base as isize) as libc::c_int
                    * (64 as libc::c_int - frac)
                    + *top.offset((base + 1 as libc::c_int) as isize) as libc::c_int
                        * frac;
                *dst
                    .offset(
                        x as isize,
                    ) = (v + 32 as libc::c_int >> 6 as libc::c_int) as pixel;
                x += 1;
                base += base_inc;
            } else {
                pixel_set(
                    &mut *dst.offset(x as isize),
                    *top.offset(max_base_x as isize) as libc::c_int,
                    width - x,
                );
                break;
            }
        }
        y += 1;
        dst = dst.offset(PXSTRIDE(stride) as isize);
        xpos += dx;
    }
}
unsafe extern "C" fn ipred_z2_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft_in: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    mut angle: libc::c_int,
    max_width: libc::c_int,
    max_height: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let is_sm: libc::c_int = angle >> 9 as libc::c_int & 0x1 as libc::c_int;
    let enable_intra_edge_filter: libc::c_int = angle >> 10 as libc::c_int;
    angle &= 511 as libc::c_int;
    if !(angle > 90 as libc::c_int && angle < 180 as libc::c_int) {
        unreachable!();
    }
    let mut dy: libc::c_int = dav1d_dr_intra_derivative[(angle - 90 as libc::c_int
        >> 1 as libc::c_int) as usize] as libc::c_int;
    let mut dx: libc::c_int = dav1d_dr_intra_derivative[(180 as libc::c_int - angle
        >> 1 as libc::c_int) as usize] as libc::c_int;
    let upsample_left: libc::c_int = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, 180 as libc::c_int - angle, is_sm)
    } else {
        0 as libc::c_int
    };
    let upsample_above: libc::c_int = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, angle - 90 as libc::c_int, is_sm)
    } else {
        0 as libc::c_int
    };
    let mut edge: [pixel; 129] = [0; 129];
    let topleft: *mut pixel = &mut *edge.as_mut_ptr().offset(64 as libc::c_int as isize)
        as *mut pixel;
    if upsample_above != 0 {
        upsample_edge(
            topleft,
            width + 1 as libc::c_int,
            topleft_in,
            0 as libc::c_int,
            width + 1 as libc::c_int,
            bitdepth_max,
        );
        dx <<= 1 as libc::c_int;
    } else {
        let filter_strength: libc::c_int = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, angle - 90 as libc::c_int, is_sm)
        } else {
            0 as libc::c_int
        };
        if filter_strength != 0 {
            filter_edge(
                &mut *topleft.offset(1 as libc::c_int as isize),
                width,
                0 as libc::c_int,
                max_width,
                &*topleft_in.offset(1 as libc::c_int as isize),
                -(1 as libc::c_int),
                width,
                filter_strength,
            );
        } else {
            memcpy(
                &mut *topleft.offset(1 as libc::c_int as isize) as *mut pixel
                    as *mut libc::c_void,
                &*topleft_in.offset(1 as libc::c_int as isize) as *const pixel
                    as *const libc::c_void,
                (width << 1 as libc::c_int) as libc::c_ulong,
            );
        }
    }
    if upsample_left != 0 {
        upsample_edge(
            &mut *topleft.offset((-height * 2 as libc::c_int) as isize),
            height + 1 as libc::c_int,
            &*topleft_in.offset(-height as isize),
            0 as libc::c_int,
            height + 1 as libc::c_int,
            bitdepth_max,
        );
        dy <<= 1 as libc::c_int;
    } else {
        let filter_strength_0: libc::c_int = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, 180 as libc::c_int - angle, is_sm)
        } else {
            0 as libc::c_int
        };
        if filter_strength_0 != 0 {
            filter_edge(
                &mut *topleft.offset(-height as isize),
                height,
                height - max_height,
                height,
                &*topleft_in.offset(-height as isize),
                0 as libc::c_int,
                height + 1 as libc::c_int,
                filter_strength_0,
            );
        } else {
            memcpy(
                &mut *topleft.offset(-height as isize) as *mut pixel
                    as *mut libc::c_void,
                &*topleft_in.offset(-height as isize) as *const pixel
                    as *const libc::c_void,
                (height << 1 as libc::c_int) as libc::c_ulong,
            );
        }
    }
    *topleft = *topleft_in;
    let base_inc_x: libc::c_int = 1 as libc::c_int + upsample_above;
    let left: *const pixel = &mut *topleft
        .offset(-(1 as libc::c_int + upsample_left) as isize) as *mut pixel;
    let mut y: libc::c_int = 0 as libc::c_int;
    let mut xpos: libc::c_int = (1 as libc::c_int + upsample_above << 6 as libc::c_int)
        - dx;
    while y < height {
        let mut base_x: libc::c_int = xpos >> 6 as libc::c_int;
        let frac_x: libc::c_int = xpos & 0x3e as libc::c_int;
        let mut x: libc::c_int = 0 as libc::c_int;
        let mut ypos: libc::c_int = (y << 6 as libc::c_int + upsample_left) - dy;
        while x < width {
            let mut v: libc::c_int = 0;
            if base_x >= 0 as libc::c_int {
                v = *topleft.offset(base_x as isize) as libc::c_int
                    * (64 as libc::c_int - frac_x)
                    + *topleft.offset((base_x + 1 as libc::c_int) as isize)
                        as libc::c_int * frac_x;
            } else {
                let base_y: libc::c_int = ypos >> 6 as libc::c_int;
                if !(base_y >= -(1 as libc::c_int + upsample_left)) {
                    unreachable!();
                }
                let frac_y: libc::c_int = ypos & 0x3e as libc::c_int;
                v = *left.offset(-base_y as isize) as libc::c_int
                    * (64 as libc::c_int - frac_y)
                    + *left.offset(-(base_y + 1 as libc::c_int) as isize) as libc::c_int
                        * frac_y;
            }
            *dst
                .offset(
                    x as isize,
                ) = (v + 32 as libc::c_int >> 6 as libc::c_int) as pixel;
            x += 1;
            base_x += base_inc_x;
            ypos -= dy;
        }
        y += 1;
        xpos -= dx;
        dst = dst.offset(PXSTRIDE(stride) as isize);
    }
}
unsafe extern "C" fn ipred_z3_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft_in: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    mut angle: libc::c_int,
    _max_width: libc::c_int,
    _max_height: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let is_sm: libc::c_int = angle >> 9 as libc::c_int & 0x1 as libc::c_int;
    let enable_intra_edge_filter: libc::c_int = angle >> 10 as libc::c_int;
    angle &= 511 as libc::c_int;
    if !(angle > 180 as libc::c_int) {
        unreachable!();
    }
    let mut dy: libc::c_int = dav1d_dr_intra_derivative[(270 as libc::c_int - angle
        >> 1 as libc::c_int) as usize] as libc::c_int;
    let mut left_out: [pixel; 128] = [0; 128];
    let mut left: *const pixel = 0 as *const pixel;
    let mut max_base_y: libc::c_int = 0;
    let upsample_left: libc::c_int = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, angle - 180 as libc::c_int, is_sm)
    } else {
        0 as libc::c_int
    };
    if upsample_left != 0 {
        upsample_edge(
            left_out.as_mut_ptr(),
            width + height,
            &*topleft_in.offset(-(width + height) as isize),
            imax(width - height, 0 as libc::c_int),
            width + height + 1 as libc::c_int,
            bitdepth_max,
        );
        left = &mut *left_out
            .as_mut_ptr()
            .offset((2 as libc::c_int * (width + height) - 2 as libc::c_int) as isize)
            as *mut pixel;
        max_base_y = 2 as libc::c_int * (width + height) - 2 as libc::c_int;
        dy <<= 1 as libc::c_int;
    } else {
        let filter_strength: libc::c_int = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, angle - 180 as libc::c_int, is_sm)
        } else {
            0 as libc::c_int
        };
        if filter_strength != 0 {
            filter_edge(
                left_out.as_mut_ptr(),
                width + height,
                0 as libc::c_int,
                width + height,
                &*topleft_in.offset(-(width + height) as isize),
                imax(width - height, 0 as libc::c_int),
                width + height + 1 as libc::c_int,
                filter_strength,
            );
            left = &mut *left_out
                .as_mut_ptr()
                .offset((width + height - 1 as libc::c_int) as isize) as *mut pixel;
            max_base_y = width + height - 1 as libc::c_int;
        } else {
            left = &*topleft_in.offset(-(1 as libc::c_int) as isize) as *const pixel;
            max_base_y = height + imin(width, height) - 1 as libc::c_int;
        }
    }
    let base_inc: libc::c_int = 1 as libc::c_int + upsample_left;
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut ypos: libc::c_int = dy;
    while x < width {
        let frac: libc::c_int = ypos & 0x3e as libc::c_int;
        let mut y: libc::c_int = 0 as libc::c_int;
        let mut base: libc::c_int = ypos >> 6 as libc::c_int;
        while y < height {
            if base < max_base_y {
                let v: libc::c_int = *left.offset(-base as isize) as libc::c_int
                    * (64 as libc::c_int - frac)
                    + *left.offset(-(base + 1 as libc::c_int) as isize) as libc::c_int
                        * frac;
                *dst
                    .offset(
                        (y as isize * PXSTRIDE(stride) + x as isize)
                            as isize,
                    ) = (v + 32 as libc::c_int >> 6 as libc::c_int) as pixel;
                y += 1;
                base += base_inc;
            } else {
                loop {
                    *dst
                        .offset(
                            (y as isize * PXSTRIDE(stride) + x as isize)
                                as isize,
                        ) = *left.offset(-max_base_y as isize);
                    y += 1;
                    if !(y < height) {
                        break;
                    }
                }
                break;
            }
        }
        x += 1;
        ypos += dy;
    }
}
unsafe extern "C" fn ipred_filter_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft_in: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    mut filt_idx: libc::c_int,
    _max_width: libc::c_int,
    _max_height: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    filt_idx &= 511 as libc::c_int;
    if !(filt_idx < 5 as libc::c_int) {
        unreachable!();
    }
    let filter: *const int8_t = (dav1d_filter_intra_taps[filt_idx as usize]).as_ptr();
    let mut top: *const pixel = &*topleft_in.offset(1 as libc::c_int as isize)
        as *const pixel;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < height {
        let mut topleft: *const pixel = &*topleft_in.offset(-y as isize) as *const pixel;
        let mut left: *const pixel = &*topleft.offset(-(1 as libc::c_int) as isize)
            as *const pixel;
        let mut left_stride: ptrdiff_t = -(1 as libc::c_int) as ptrdiff_t;
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            let p0: libc::c_int = *topleft as libc::c_int;
            let p1: libc::c_int = *top.offset(0 as libc::c_int as isize) as libc::c_int;
            let p2: libc::c_int = *top.offset(1 as libc::c_int as isize) as libc::c_int;
            let p3: libc::c_int = *top.offset(2 as libc::c_int as isize) as libc::c_int;
            let p4: libc::c_int = *top.offset(3 as libc::c_int as isize) as libc::c_int;
            let p5: libc::c_int = *left
                .offset((0 * left_stride) as isize)
                as libc::c_int;
            let p6: libc::c_int = *left
                .offset((1 * left_stride) as isize)
                as libc::c_int;
            let mut ptr: *mut pixel = &mut *dst.offset(x as isize) as *mut pixel;
            let mut flt_ptr: *const int8_t = filter;
            let mut yy: libc::c_int = 0 as libc::c_int;
            while yy < 2 as libc::c_int {
                let mut xx: libc::c_int = 0 as libc::c_int;
                while xx < 4 as libc::c_int {
                    let acc: libc::c_int = *flt_ptr.offset(0 as libc::c_int as isize)
                        as libc::c_int * p0
                        + *flt_ptr.offset(1 as libc::c_int as isize) as libc::c_int * p1
                        + *flt_ptr.offset(16 as libc::c_int as isize) as libc::c_int * p2
                        + *flt_ptr.offset(17 as libc::c_int as isize) as libc::c_int * p3
                        + *flt_ptr.offset(32 as libc::c_int as isize) as libc::c_int * p4
                        + *flt_ptr.offset(33 as libc::c_int as isize) as libc::c_int * p5
                        + *flt_ptr.offset(48 as libc::c_int as isize) as libc::c_int
                            * p6;
                    *ptr
                        .offset(
                            xx as isize,
                        ) = iclip(
                        acc + 8 as libc::c_int >> 4 as libc::c_int,
                        0 as libc::c_int,
                        bitdepth_max,
                    ) as pixel;
                    xx += 1;
                    flt_ptr = flt_ptr.offset(2 as libc::c_int as isize);
                }
                ptr = ptr.offset(PXSTRIDE(stride) as isize);
                yy += 1;
            }
            left = &mut *dst.offset((x + 4 as libc::c_int - 1 as libc::c_int) as isize)
                as *mut pixel;
            left_stride = PXSTRIDE(stride);
            top = top.offset(4 as libc::c_int as isize);
            topleft = &*top.offset(-(1 as libc::c_int) as isize) as *const pixel;
            x += 4 as libc::c_int;
        }
        top = &mut *dst
            .offset(
                (PXSTRIDE as unsafe extern "C" fn(ptrdiff_t) -> ptrdiff_t)(stride)
                    as isize,
            ) as *mut pixel;
        dst = &mut *dst
            .offset(
                ((PXSTRIDE as unsafe extern "C" fn(ptrdiff_t) -> ptrdiff_t)(stride)
                    * 2) as isize,
            ) as *mut pixel;
        y += 2 as libc::c_int;
    }
}
#[inline(never)]
unsafe extern "C" fn cfl_ac_c(
    mut ac: *mut int16_t,
    mut ypx: *const pixel,
    stride: ptrdiff_t,
    w_pad: libc::c_int,
    h_pad: libc::c_int,
    width: libc::c_int,
    height: libc::c_int,
    ss_hor: libc::c_int,
    ss_ver: libc::c_int,
) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let ac_orig: *mut int16_t = ac;
    if !(w_pad >= 0 as libc::c_int && (w_pad * 4 as libc::c_int) < width) {
        unreachable!();
    }
    if !(h_pad >= 0 as libc::c_int && (h_pad * 4 as libc::c_int) < height) {
        unreachable!();
    }
    y = 0 as libc::c_int;
    while y < height - 4 as libc::c_int * h_pad {
        x = 0 as libc::c_int;
        while x < width - 4 as libc::c_int * w_pad {
            let mut ac_sum: libc::c_int = *ypx.offset((x << ss_hor) as isize)
                as libc::c_int;
            if ss_hor != 0 {
                ac_sum
                    += *ypx.offset((x * 2 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int;
            }
            if ss_ver != 0 {
                ac_sum
                    += *ypx
                        .offset(
                            ((x << ss_hor) as isize + PXSTRIDE(stride)) as isize,
                        ) as libc::c_int;
                if ss_hor != 0 {
                    ac_sum
                        += *ypx
                            .offset(
                                ((x * 2 as libc::c_int + 1 as libc::c_int) as isize
                                    + PXSTRIDE(stride)) as isize,
                            ) as libc::c_int;
                }
            }
            *ac
                .offset(
                    x as isize,
                ) = (ac_sum
                << 1 as libc::c_int + (ss_ver == 0) as libc::c_int
                    + (ss_hor == 0) as libc::c_int) as int16_t;
            x += 1;
        }
        while x < width {
            *ac.offset(x as isize) = *ac.offset((x - 1 as libc::c_int) as isize);
            x += 1;
        }
        ac = ac.offset(width as isize);
        ypx = ypx.offset((PXSTRIDE(stride) << ss_ver) as isize);
        y += 1;
    }
    while y < height {
        memcpy(
            ac as *mut libc::c_void,
            &mut *ac.offset(-width as isize) as *mut int16_t as *const libc::c_void,
            (width as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong),
        );
        ac = ac.offset(width as isize);
        y += 1;
    }
    let log2sz: libc::c_int = ctz(width as libc::c_uint) + ctz(height as libc::c_uint);
    let mut sum: libc::c_int = (1 as libc::c_int) << log2sz >> 1 as libc::c_int;
    ac = ac_orig;
    y = 0 as libc::c_int;
    while y < height {
        x = 0 as libc::c_int;
        while x < width {
            sum += *ac.offset(x as isize) as libc::c_int;
            x += 1;
        }
        ac = ac.offset(width as isize);
        y += 1;
    }
    sum >>= log2sz;
    ac = ac_orig;
    y = 0 as libc::c_int;
    while y < height {
        x = 0 as libc::c_int;
        while x < width {
            let ref mut fresh0 = *ac.offset(x as isize);
            *fresh0 = (*fresh0 as libc::c_int - sum) as int16_t;
            x += 1;
        }
        ac = ac.offset(width as isize);
        y += 1;
    }
}
unsafe extern "C" fn cfl_ac_420_c(
    ac: *mut int16_t,
    ypx: *const pixel,
    stride: ptrdiff_t,
    w_pad: libc::c_int,
    h_pad: libc::c_int,
    cw: libc::c_int,
    ch: libc::c_int,
) {
    cfl_ac_c(ac, ypx, stride, w_pad, h_pad, cw, ch, 1 as libc::c_int, 1 as libc::c_int);
}
unsafe extern "C" fn cfl_ac_422_c(
    ac: *mut int16_t,
    ypx: *const pixel,
    stride: ptrdiff_t,
    w_pad: libc::c_int,
    h_pad: libc::c_int,
    cw: libc::c_int,
    ch: libc::c_int,
) {
    cfl_ac_c(ac, ypx, stride, w_pad, h_pad, cw, ch, 1 as libc::c_int, 0 as libc::c_int);
}
unsafe extern "C" fn cfl_ac_444_c(
    ac: *mut int16_t,
    ypx: *const pixel,
    stride: ptrdiff_t,
    w_pad: libc::c_int,
    h_pad: libc::c_int,
    cw: libc::c_int,
    ch: libc::c_int,
) {
    cfl_ac_c(ac, ypx, stride, w_pad, h_pad, cw, ch, 0 as libc::c_int, 0 as libc::c_int);
}
unsafe extern "C" fn pal_pred_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    pal: *const uint16_t,
    mut idx: *const uint8_t,
    w: libc::c_int,
    h: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < h {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < w {
            *dst.offset(x as isize) = *pal.offset(*idx.offset(x as isize) as isize);
            x += 1;
        }
        idx = idx.offset(w as isize);
        dst = dst.offset(PXSTRIDE(stride) as isize);
        y += 1;
    }
}

#[cfg(all(
    feature = "asm",
    any(target_arch = "x86", target_arch = "x86_64"),
))]
#[inline(always)]
unsafe extern "C" fn intra_pred_dsp_init_x86(c: *mut Dav1dIntraPredDSPContext) {
    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_X86_CPU_FLAG_SSSE3 == 0 {
        return;
    }

    (*c).intra_pred[DC_PRED as usize] = Some(dav1d_ipred_dc_16bpc_ssse3);
    (*c).intra_pred[DC_128_PRED as usize] = Some(dav1d_ipred_dc_128_16bpc_ssse3);
    (*c).intra_pred[TOP_DC_PRED as usize] = Some(dav1d_ipred_dc_top_16bpc_ssse3);
    (*c).intra_pred[LEFT_DC_PRED as usize] = Some(dav1d_ipred_dc_left_16bpc_ssse3);
    (*c).intra_pred[HOR_PRED as usize] = Some(dav1d_ipred_h_16bpc_ssse3);
    (*c).intra_pred[VERT_PRED as usize] = Some(dav1d_ipred_v_16bpc_ssse3);
    (*c).intra_pred[PAETH_PRED as usize] = Some(dav1d_ipred_paeth_16bpc_ssse3);
    (*c).intra_pred[SMOOTH_PRED as usize] = Some(dav1d_ipred_smooth_16bpc_ssse3);
    (*c).intra_pred[SMOOTH_H_PRED as usize] = Some(dav1d_ipred_smooth_h_16bpc_ssse3);
    (*c).intra_pred[SMOOTH_V_PRED as usize] = Some(dav1d_ipred_smooth_v_16bpc_ssse3);
    (*c).intra_pred[FILTER_PRED as usize] = Some(dav1d_ipred_filter_16bpc_ssse3);

    (*c).cfl_pred[DC_PRED as usize] = Some(dav1d_ipred_cfl_16bpc_ssse3);
    (*c).cfl_pred[DC_128_PRED as usize] = Some(dav1d_ipred_cfl_128_16bpc_ssse3);
    (*c).cfl_pred[TOP_DC_PRED as usize] = Some(dav1d_ipred_cfl_top_16bpc_ssse3);
    (*c).cfl_pred[LEFT_DC_PRED as usize] = Some(dav1d_ipred_cfl_left_16bpc_ssse3);

    (*c).cfl_ac[(DAV1D_PIXEL_LAYOUT_I420 - 1) as usize] = Some(dav1d_ipred_cfl_ac_420_16bpc_ssse3);
    (*c).cfl_ac[(DAV1D_PIXEL_LAYOUT_I422 - 1) as usize] = Some(dav1d_ipred_cfl_ac_422_16bpc_ssse3);
    (*c).cfl_ac[(DAV1D_PIXEL_LAYOUT_I444 - 1) as usize] = Some(dav1d_ipred_cfl_ac_444_16bpc_ssse3);

    (*c).pal_pred = Some(dav1d_pal_pred_16bpc_ssse3);

    if flags & DAV1D_X86_CPU_FLAG_AVX2 == 0 {
        return;
    }

    (*c).intra_pred[DC_PRED as usize] = Some(dav1d_ipred_dc_16bpc_avx2);
    (*c).intra_pred[DC_128_PRED as usize] = Some(dav1d_ipred_dc_128_16bpc_avx2);
    (*c).intra_pred[TOP_DC_PRED as usize] = Some(dav1d_ipred_dc_top_16bpc_avx2);
    (*c).intra_pred[LEFT_DC_PRED as usize] = Some(dav1d_ipred_dc_left_16bpc_avx2);
    (*c).intra_pred[HOR_PRED as usize] = Some(dav1d_ipred_h_16bpc_avx2);
    (*c).intra_pred[VERT_PRED as usize] = Some(dav1d_ipred_v_16bpc_avx2);
    (*c).intra_pred[PAETH_PRED as usize] = Some(dav1d_ipred_paeth_16bpc_avx2);
    (*c).intra_pred[SMOOTH_PRED as usize] = Some(dav1d_ipred_smooth_16bpc_avx2);
    (*c).intra_pred[SMOOTH_H_PRED as usize] = Some(dav1d_ipred_smooth_h_16bpc_avx2);
    (*c).intra_pred[SMOOTH_V_PRED as usize] = Some(dav1d_ipred_smooth_v_16bpc_avx2);
    (*c).intra_pred[Z1_PRED as usize] = Some(dav1d_ipred_z1_16bpc_avx2);
    (*c).intra_pred[Z2_PRED as usize] = Some(dav1d_ipred_z2_16bpc_avx2);
    (*c).intra_pred[Z3_PRED as usize] = Some(dav1d_ipred_z3_16bpc_avx2);
    (*c).intra_pred[FILTER_PRED as usize] = Some(dav1d_ipred_filter_16bpc_avx2);

    (*c).cfl_pred[DC_PRED as usize] = Some(dav1d_ipred_cfl_16bpc_avx2);
    (*c).cfl_pred[DC_128_PRED as usize] = Some(dav1d_ipred_cfl_128_16bpc_avx2);
    (*c).cfl_pred[TOP_DC_PRED as usize] = Some(dav1d_ipred_cfl_top_16bpc_avx2);
    (*c).cfl_pred[LEFT_DC_PRED as usize] = Some(dav1d_ipred_cfl_left_16bpc_avx2);

    (*c).cfl_ac[(DAV1D_PIXEL_LAYOUT_I420 - 1) as usize] = Some(dav1d_ipred_cfl_ac_420_16bpc_avx2);
    (*c).cfl_ac[(DAV1D_PIXEL_LAYOUT_I422 - 1) as usize] = Some(dav1d_ipred_cfl_ac_422_16bpc_avx2);
    (*c).cfl_ac[(DAV1D_PIXEL_LAYOUT_I444 - 1) as usize] = Some(dav1d_ipred_cfl_ac_444_16bpc_avx2);

    (*c).pal_pred = Some(dav1d_pal_pred_16bpc_avx2);

    if flags & DAV1D_X86_CPU_FLAG_AVX512ICL == 0 {
        return;
    }

    (*c).intra_pred[PAETH_PRED as usize] = Some(dav1d_ipred_paeth_16bpc_avx512icl);
    (*c).intra_pred[SMOOTH_PRED as usize] = Some(dav1d_ipred_smooth_16bpc_avx512icl);
    (*c).intra_pred[SMOOTH_H_PRED as usize] = Some(dav1d_ipred_smooth_h_16bpc_avx512icl);
    (*c).intra_pred[SMOOTH_V_PRED as usize] = Some(dav1d_ipred_smooth_v_16bpc_avx512icl);
    (*c).intra_pred[FILTER_PRED as usize] = Some(dav1d_ipred_filter_16bpc_avx512icl);

    (*c).pal_pred = Some(dav1d_pal_pred_16bpc_avx512icl);
}

#[cfg(feature = "asm")]
#[inline(always)]
unsafe extern "C" fn dav1d_get_cpu_flags() -> libc::c_uint {
    let mut flags: libc::c_uint = dav1d_cpu_flags & dav1d_cpu_flags_mask;
    flags |= DAV1D_X86_CPU_FLAG_SSE2 as libc::c_int as libc::c_uint;
    return flags;
}

#[cfg(all(
    feature = "asm",
    any(target_arch = "arm", target_arch = "aarch64"),
))]
#[inline(always)]
unsafe extern "C" fn intra_pred_dsp_init_arm(c: *mut Dav1dIntraPredDSPContext) {
    use crate::src::arm::cpu::DAV1D_ARM_CPU_FLAG_NEON;

    let flags = dav1d_get_cpu_flags();
    if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
        return;
    }

    (*c).intra_pred[DC_PRED as usize] = Some(dav1d_ipred_dc_16bpc_neon);
    (*c).intra_pred[DC_128_PRED as usize] = Some(dav1d_ipred_dc_128_16bpc_neon);
    (*c).intra_pred[TOP_DC_PRED as usize] = Some(dav1d_ipred_dc_top_16bpc_neon);
    (*c).intra_pred[LEFT_DC_PRED as usize] = Some(dav1d_ipred_dc_left_16bpc_neon);
    (*c).intra_pred[HOR_PRED as usize] = Some(dav1d_ipred_h_16bpc_neon);
    (*c).intra_pred[VERT_PRED as usize] = Some(dav1d_ipred_v_16bpc_neon);
    (*c).intra_pred[PAETH_PRED as usize] = Some(dav1d_ipred_paeth_16bpc_neon);
    (*c).intra_pred[SMOOTH_PRED as usize] = Some(dav1d_ipred_smooth_16bpc_neon);
    (*c).intra_pred[SMOOTH_V_PRED as usize] = Some(dav1d_ipred_smooth_v_16bpc_neon);
    (*c).intra_pred[SMOOTH_H_PRED as usize] = Some(dav1d_ipred_smooth_h_16bpc_neon);
    (*c).intra_pred[FILTER_PRED as usize] = Some(dav1d_ipred_filter_16bpc_neon);

    (*c).cfl_pred[DC_PRED as usize] = Some(dav1d_ipred_cfl_16bpc_neon);
    (*c).cfl_pred[DC_128_PRED as usize] = Some(dav1d_ipred_cfl_128_16bpc_neon);
    (*c).cfl_pred[TOP_DC_PRED as usize] = Some(dav1d_ipred_cfl_top_16bpc_neon);
    (*c).cfl_pred[LEFT_DC_PRED as usize] = Some(dav1d_ipred_cfl_left_16bpc_neon);

    (*c).cfl_ac[(DAV1D_PIXEL_LAYOUT_I420 - 1) as usize] = Some(dav1d_ipred_cfl_ac_420_16bpc_neon);
    (*c).cfl_ac[(DAV1D_PIXEL_LAYOUT_I422 - 1) as usize] = Some(dav1d_ipred_cfl_ac_422_16bpc_neon);
    (*c).cfl_ac[(DAV1D_PIXEL_LAYOUT_I444 - 1) as usize] = Some(dav1d_ipred_cfl_ac_444_16bpc_neon);

    (*c).pal_pred = Some(dav1d_pal_pred_16bpc_neon);
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_intra_pred_dsp_init_16bpc(
    c: *mut Dav1dIntraPredDSPContext,
) {
    (*c).intra_pred[DC_PRED as usize] = Some(ipred_dc_c);
    (*c).intra_pred[DC_128_PRED as usize] = Some(ipred_dc_128_c);
    (*c).intra_pred[TOP_DC_PRED as usize] = Some(ipred_dc_top_c);
    (*c).intra_pred[LEFT_DC_PRED as usize] = Some(ipred_dc_left_c);
    (*c).intra_pred[HOR_PRED as usize] = Some(ipred_h_c);
    (*c).intra_pred[VERT_PRED as usize] = Some(ipred_v_c);
    (*c).intra_pred[PAETH_PRED as usize] = Some(ipred_paeth_c);
    (*c).intra_pred[SMOOTH_PRED as usize] = Some(ipred_smooth_c);
    (*c).intra_pred[SMOOTH_V_PRED as usize] = Some(ipred_smooth_v_c);
    (*c).intra_pred[SMOOTH_H_PRED as usize] = Some(ipred_smooth_h_c);
    (*c).intra_pred[Z1_PRED as usize] = Some(ipred_z1_c);
    (*c).intra_pred[Z2_PRED as usize] = Some(ipred_z2_c);
    (*c).intra_pred[Z3_PRED as usize] = Some(ipred_z3_c);
    (*c).intra_pred[FILTER_PRED as usize] = Some(ipred_filter_c);

    (*c).cfl_ac[(DAV1D_PIXEL_LAYOUT_I420 - 1) as usize] = Some(cfl_ac_420_c);
    (*c).cfl_ac[(DAV1D_PIXEL_LAYOUT_I422 - 1) as usize] = Some(cfl_ac_422_c);
    (*c).cfl_ac[(DAV1D_PIXEL_LAYOUT_I444 - 1) as usize] = Some(cfl_ac_444_c);
    (*c).cfl_pred[DC_PRED as usize] = Some(ipred_cfl_c);

    (*c).cfl_pred[DC_128_PRED as usize] = Some(ipred_cfl_128_c);
    (*c).cfl_pred[TOP_DC_PRED as usize] = Some(ipred_cfl_top_c);
    (*c).cfl_pred[LEFT_DC_PRED as usize] = Some(ipred_cfl_left_c);

    (*c).pal_pred = Some(pal_pred_c);

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            intra_pred_dsp_init_x86(c);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
            intra_pred_dsp_init_arm(c);
        }
    }
}
