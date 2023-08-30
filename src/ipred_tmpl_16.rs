use crate::include::stddef::*;
use crate::include::stdint::*;
#[cfg(feature = "asm")]
use cfg_if::cfg_if;
use libc;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
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
    fn dav1d_ipred_z1_16bpc_ssse3(
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
    fn dav1d_ipred_z2_16bpc_ssse3(
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
    fn dav1d_ipred_z3_16bpc_ssse3(
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
}

#[cfg(all(feature = "asm", target_arch = "x86_64"))]
extern "C" {
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

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
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

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
extern "C" {
    fn dav1d_ipred_z1_fill2_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        top: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        dx: libc::c_int,
        max_base_x: libc::c_int,
    );
    fn dav1d_ipred_z1_fill1_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        top: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        dx: libc::c_int,
        max_base_x: libc::c_int,
    );
    fn dav1d_ipred_z1_upsample_edge_16bpc_neon(
        out: *mut pixel,
        hsz: libc::c_int,
        in_0: *const pixel,
        end: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_z1_filter_edge_16bpc_neon(
        out: *mut pixel,
        sz: libc::c_int,
        in_0: *const pixel,
        end: libc::c_int,
        strength: libc::c_int,
    );
    fn dav1d_ipred_z2_fill3_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        top: *const pixel,
        left: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
    );
    fn dav1d_ipred_z2_fill2_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        top: *const pixel,
        left: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
    );
    fn dav1d_ipred_z2_fill1_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        top: *const pixel,
        left: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
    );
    fn dav1d_ipred_z2_upsample_edge_16bpc_neon(
        out: *mut pixel,
        hsz: libc::c_int,
        in_0: *const pixel,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_ipred_reverse_16bpc_neon(dst: *mut pixel, src: *const pixel, n: libc::c_int);
    fn dav1d_ipred_z3_fill2_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        dy: libc::c_int,
        max_base_y: libc::c_int,
    );
    fn dav1d_ipred_z3_fill1_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: *const pixel,
        width: libc::c_int,
        height: libc::c_int,
        dy: libc::c_int,
        max_base_y: libc::c_int,
    );
    fn dav1d_ipred_pixel_set_16bpc_neon(out: *mut pixel, px: pixel, n: libc::c_int);
}

use crate::src::tables::dav1d_dr_intra_derivative;
use crate::src::tables::dav1d_filter_intra_taps;
use crate::src::tables::dav1d_sm_weights;

pub type pixel = uint16_t;

use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I422;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;

use crate::src::levels::DC_128_PRED;
use crate::src::levels::DC_PRED;
use crate::src::levels::FILTER_PRED;
use crate::src::levels::HOR_PRED;
use crate::src::levels::LEFT_DC_PRED;
use crate::src::levels::PAETH_PRED;
use crate::src::levels::SMOOTH_H_PRED;
use crate::src::levels::SMOOTH_PRED;
use crate::src::levels::SMOOTH_V_PRED;
use crate::src::levels::TOP_DC_PRED;
use crate::src::levels::VERT_PRED;
use crate::src::levels::Z1_PRED;
use crate::src::levels::Z2_PRED;
use crate::src::levels::Z3_PRED;
pub type angular_ipred_fn = Option<
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
pub type cfl_ac_fn = Option<
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
pub type cfl_pred_fn = Option<
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
pub type pal_pred_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const uint16_t,
        *const uint8_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
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

use crate::include::common::attributes::ctz;
use crate::include::common::intops::apply_sign;
use crate::include::common::intops::iclip;
use crate::include::common::intops::imax;
use crate::include::common::intops::imin;

#[inline]
unsafe extern "C" fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 != 0 {
        unreachable!();
    }
    return x >> 1;
}
#[inline]
unsafe extern "C" fn pixel_set(dst: *mut pixel, val: libc::c_int, num: libc::c_int) {
    let mut n = 0;
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
    let dcN: uint64_t =
        (dc as libc::c_ulonglong).wrapping_mul(0x1000100010001 as libc::c_ulonglong) as uint64_t;
    let mut y = 0;
    while y < height {
        let mut x = 0;
        while x < width {
            *(&mut *dst.offset(x as isize) as *mut pixel as *mut uint64_t) = dcN;
            x = (x as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<uint64_t>() as libc::c_ulong >> 1)
                as libc::c_int as libc::c_int;
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
    let mut y = 0;
    while y < height {
        let mut x = 0;
        while x < width {
            let diff = alpha * *ac.offset(x as isize) as libc::c_int;
            *dst.offset(x as isize) = iclip(
                dc + apply_sign(diff.abs() + 32 >> 6, diff),
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
unsafe extern "C" fn dc_gen_top(topleft: *const pixel, width: libc::c_int) -> libc::c_uint {
    let mut dc: libc::c_uint = (width >> 1) as libc::c_uint;
    let mut i = 0;
    while i < width {
        dc = dc.wrapping_add(*topleft.offset((1 + i) as isize) as libc::c_uint);
        i += 1;
    }
    return dc >> ctz(width as libc::c_uint);
}
unsafe extern "C" fn ipred_dc_top_c(
    dst: *mut pixel,
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
    dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    ac: *const int16_t,
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
unsafe extern "C" fn dc_gen_left(topleft: *const pixel, height: libc::c_int) -> libc::c_uint {
    let mut dc: libc::c_uint = (height >> 1) as libc::c_uint;
    let mut i = 0;
    while i < height {
        dc = dc.wrapping_add(*topleft.offset(-(1 + i) as isize) as libc::c_uint);
        i += 1;
    }
    return dc >> ctz(height as libc::c_uint);
}
unsafe extern "C" fn ipred_dc_left_c(
    dst: *mut pixel,
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
    dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    ac: *const int16_t,
    alpha: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let dc: libc::c_uint = dc_gen_left(topleft, height);
    cfl_pred(
        dst,
        stride,
        width,
        height,
        dc as libc::c_int,
        ac,
        alpha,
        bitdepth_max,
    );
}
unsafe extern "C" fn dc_gen(
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
) -> libc::c_uint {
    let mut dc: libc::c_uint = (width + height >> 1) as libc::c_uint;
    let mut i = 0;
    while i < width {
        dc = dc.wrapping_add(*topleft.offset((i + 1) as isize) as libc::c_uint);
        i += 1;
    }
    let mut i_0 = 0;
    while i_0 < height {
        dc = dc.wrapping_add(*topleft.offset(-(i_0 + 1) as isize) as libc::c_uint);
        i_0 += 1;
    }
    dc >>= ctz((width + height) as libc::c_uint);
    if width != height {
        dc = dc.wrapping_mul(
            (if width > height * 2 || height > width * 2 {
                0x6667 as libc::c_int
            } else {
                0xaaab as libc::c_int
            }) as libc::c_uint,
        );
        dc >>= 17;
    }
    return dc;
}
unsafe extern "C" fn ipred_dc_c(
    dst: *mut pixel,
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
    dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    ac: *const int16_t,
    alpha: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let dc: libc::c_uint = dc_gen(topleft, width, height);
    cfl_pred(
        dst,
        stride,
        width,
        height,
        dc as libc::c_int,
        ac,
        alpha,
        bitdepth_max,
    );
}
unsafe extern "C" fn ipred_dc_128_c(
    dst: *mut pixel,
    stride: ptrdiff_t,
    _topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    _a: libc::c_int,
    _max_width: libc::c_int,
    _max_height: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let dc = bitdepth_max + 1 >> 1;
    splat_dc(dst, stride, width, height, dc, bitdepth_max);
}
unsafe extern "C" fn ipred_cfl_128_c(
    dst: *mut pixel,
    stride: ptrdiff_t,
    _topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    ac: *const int16_t,
    alpha: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let dc = bitdepth_max + 1 >> 1;
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
    let mut y = 0;
    while y < height {
        memcpy(
            dst as *mut libc::c_void,
            topleft.offset(1) as *const libc::c_void,
            (width << 1) as libc::c_ulong,
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
    let mut y = 0;
    while y < height {
        pixel_set(
            dst,
            *topleft.offset(-(1 + y) as isize) as libc::c_int,
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
    let topleft = *tl_ptr.offset(0) as libc::c_int;
    let mut y = 0;
    while y < height {
        let left = *tl_ptr.offset(-(y + 1) as isize) as libc::c_int;
        let mut x = 0;
        while x < width {
            let top = *tl_ptr.offset((1 + x) as isize) as libc::c_int;
            let base = left + top - topleft;
            let ldiff = (left - base).abs();
            let tdiff = (top - base).abs();
            let tldiff = (topleft - base).abs();
            *dst.offset(x as isize) = (if ldiff <= tdiff && ldiff <= tldiff {
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
    let weights_hor: *const uint8_t =
        &*dav1d_sm_weights.0.as_ptr().offset(width as isize) as *const uint8_t;
    let weights_ver: *const uint8_t =
        &*dav1d_sm_weights.0.as_ptr().offset(height as isize) as *const uint8_t;
    let right = *topleft.offset(width as isize) as libc::c_int;
    let bottom = *topleft.offset(-height as isize) as libc::c_int;
    let mut y = 0;
    while y < height {
        let mut x = 0;
        while x < width {
            let pred = *weights_ver.offset(y as isize) as libc::c_int
                * *topleft.offset((1 + x) as isize) as libc::c_int
                + (256 - *weights_ver.offset(y as isize) as libc::c_int) * bottom
                + *weights_hor.offset(x as isize) as libc::c_int
                    * *topleft.offset(-(1 + y) as isize) as libc::c_int
                + (256 - *weights_hor.offset(x as isize) as libc::c_int) * right;
            *dst.offset(x as isize) = (pred + 256 >> 9) as pixel;
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
    let weights_ver: *const uint8_t =
        &*dav1d_sm_weights.0.as_ptr().offset(height as isize) as *const uint8_t;
    let bottom = *topleft.offset(-height as isize) as libc::c_int;
    let mut y = 0;
    while y < height {
        let mut x = 0;
        while x < width {
            let pred = *weights_ver.offset(y as isize) as libc::c_int
                * *topleft.offset((1 + x) as isize) as libc::c_int
                + (256 - *weights_ver.offset(y as isize) as libc::c_int) * bottom;
            *dst.offset(x as isize) = (pred + 128 >> 8) as pixel;
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
    let weights_hor: *const uint8_t =
        &*dav1d_sm_weights.0.as_ptr().offset(width as isize) as *const uint8_t;
    let right = *topleft.offset(width as isize) as libc::c_int;
    let mut y = 0;
    while y < height {
        let mut x = 0;
        while x < width {
            let pred = *weights_hor.offset(x as isize) as libc::c_int
                * *topleft.offset(-(y + 1) as isize) as libc::c_int
                + (256 - *weights_hor.offset(x as isize) as libc::c_int) * right;
            *dst.offset(x as isize) = (pred + 128 >> 8) as pixel;
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
        if wh <= 8 {
            if angle >= 64 {
                return 2 as libc::c_int;
            }
            if angle >= 40 {
                return 1 as libc::c_int;
            }
        } else if wh <= 16 {
            if angle >= 48 {
                return 2 as libc::c_int;
            }
            if angle >= 20 {
                return 1 as libc::c_int;
            }
        } else if wh <= 24 {
            if angle >= 4 {
                return 3 as libc::c_int;
            }
        } else {
            return 3 as libc::c_int;
        }
    } else if wh <= 8 {
        if angle >= 56 {
            return 1 as libc::c_int;
        }
    } else if wh <= 16 {
        if angle >= 40 {
            return 1 as libc::c_int;
        }
    } else if wh <= 24 {
        if angle >= 32 {
            return 3 as libc::c_int;
        }
        if angle >= 16 {
            return 2 as libc::c_int;
        }
        if angle >= 8 {
            return 1 as libc::c_int;
        }
    } else if wh <= 32 {
        if angle >= 32 {
            return 3 as libc::c_int;
        }
        if angle >= 4 {
            return 2 as libc::c_int;
        }
        return 1 as libc::c_int;
    } else {
        return 3 as libc::c_int;
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
    if !(strength > 0) {
        unreachable!();
    }
    let mut i = 0;
    while i < imin(sz, lim_from) {
        *out.offset(i as isize) = *in_0.offset(iclip(i, from, to - 1) as isize);
        i += 1;
    }
    while i < imin(lim_to, sz) {
        let mut s = 0;
        let mut j = 0;
        while j < 5 {
            s += *in_0.offset(iclip(i - 2 + j, from, to - 1) as isize) as libc::c_int
                * kernel[(strength - 1) as usize][j as usize] as libc::c_int;
            j += 1;
        }
        *out.offset(i as isize) = (s + 8 >> 4) as pixel;
        i += 1;
    }
    while i < sz {
        *out.offset(i as isize) = *in_0.offset(iclip(i, from, to - 1) as isize);
        i += 1;
    }
}
use crate::src::ipred::get_upsample;
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
    let mut i;
    i = 0 as libc::c_int;
    while i < hsz - 1 {
        *out.offset((i * 2) as isize) = *in_0.offset(iclip(i, from, to - 1) as isize);
        let mut s = 0;
        let mut j = 0;
        while j < 4 {
            s += *in_0.offset(iclip(i + j - 1, from, to - 1) as isize) as libc::c_int
                * kernel[j as usize] as libc::c_int;
            j += 1;
        }
        *out.offset((i * 2 + 1) as isize) =
            iclip(s + 8 >> 4, 0 as libc::c_int, bitdepth_max) as pixel;
        i += 1;
    }
    *out.offset((i * 2) as isize) = *in_0.offset(iclip(i, from, to - 1) as isize);
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
    let is_sm = angle >> 9 & 0x1 as libc::c_int;
    let enable_intra_edge_filter = angle >> 10;
    angle &= 511 as libc::c_int;
    if !(angle < 90) {
        unreachable!();
    }
    let mut dx = dav1d_dr_intra_derivative[(angle >> 1) as usize] as libc::c_int;
    let mut top_out: [pixel; 128] = [0; 128];
    let top: *const pixel;
    let max_base_x;
    let upsample_above = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, 90 - angle, is_sm)
    } else {
        0 as libc::c_int
    };
    if upsample_above != 0 {
        upsample_edge(
            top_out.as_mut_ptr(),
            width + height,
            &*topleft_in.offset(1),
            -(1 as libc::c_int),
            width + imin(width, height),
            bitdepth_max,
        );
        top = top_out.as_mut_ptr();
        max_base_x = 2 * (width + height) - 2;
        dx <<= 1;
    } else {
        let filter_strength = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, 90 - angle, is_sm)
        } else {
            0 as libc::c_int
        };
        if filter_strength != 0 {
            filter_edge(
                top_out.as_mut_ptr(),
                width + height,
                0 as libc::c_int,
                width + height,
                &*topleft_in.offset(1),
                -(1 as libc::c_int),
                width + imin(width, height),
                filter_strength,
            );
            top = top_out.as_mut_ptr();
            max_base_x = width + height - 1;
        } else {
            top = &*topleft_in.offset(1) as *const pixel;
            max_base_x = width + imin(width, height) - 1;
        }
    }
    let base_inc = 1 + upsample_above;
    let mut y = 0;
    let mut xpos = dx;
    while y < height {
        let frac = xpos & 0x3e as libc::c_int;
        let mut x = 0;
        let mut base = xpos >> 6;
        while x < width {
            if base < max_base_x {
                let v = *top.offset(base as isize) as libc::c_int * (64 - frac)
                    + *top.offset((base + 1) as isize) as libc::c_int * frac;
                *dst.offset(x as isize) = (v + 32 >> 6) as pixel;
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
    let is_sm = angle >> 9 & 0x1 as libc::c_int;
    let enable_intra_edge_filter = angle >> 10;
    angle &= 511 as libc::c_int;
    if !(angle > 90 && angle < 180) {
        unreachable!();
    }
    let mut dy = dav1d_dr_intra_derivative[(angle - 90 >> 1) as usize] as libc::c_int;
    let mut dx = dav1d_dr_intra_derivative[(180 - angle >> 1) as usize] as libc::c_int;
    let upsample_left = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, 180 - angle, is_sm)
    } else {
        0 as libc::c_int
    };
    let upsample_above = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, angle - 90, is_sm)
    } else {
        0 as libc::c_int
    };
    let mut edge: [pixel; 129] = [0; 129];
    let topleft: *mut pixel = &mut *edge.as_mut_ptr().offset(64) as *mut pixel;
    if upsample_above != 0 {
        upsample_edge(
            topleft,
            width + 1,
            topleft_in,
            0 as libc::c_int,
            width + 1,
            bitdepth_max,
        );
        dx <<= 1;
    } else {
        let filter_strength = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, angle - 90, is_sm)
        } else {
            0 as libc::c_int
        };
        if filter_strength != 0 {
            filter_edge(
                &mut *topleft.offset(1),
                width,
                0 as libc::c_int,
                max_width,
                &*topleft_in.offset(1),
                -(1 as libc::c_int),
                width,
                filter_strength,
            );
        } else {
            memcpy(
                &mut *topleft.offset(1) as *mut pixel as *mut libc::c_void,
                &*topleft_in.offset(1) as *const pixel as *const libc::c_void,
                (width << 1) as libc::c_ulong,
            );
        }
    }
    if upsample_left != 0 {
        upsample_edge(
            &mut *topleft.offset((-height * 2) as isize),
            height + 1,
            &*topleft_in.offset(-height as isize),
            0 as libc::c_int,
            height + 1,
            bitdepth_max,
        );
        dy <<= 1;
    } else {
        let filter_strength_0 = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, 180 - angle, is_sm)
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
                height + 1,
                filter_strength_0,
            );
        } else {
            memcpy(
                &mut *topleft.offset(-height as isize) as *mut pixel as *mut libc::c_void,
                &*topleft_in.offset(-height as isize) as *const pixel as *const libc::c_void,
                (height << 1) as libc::c_ulong,
            );
        }
    }
    *topleft = *topleft_in;
    let base_inc_x = 1 + upsample_above;
    let left: *const pixel = &mut *topleft.offset(-(1 + upsample_left) as isize) as *mut pixel;
    let mut y = 0;
    let mut xpos = (1 + upsample_above << 6) - dx;
    while y < height {
        let mut base_x = xpos >> 6;
        let frac_x = xpos & 0x3e as libc::c_int;
        let mut x = 0;
        let mut ypos = (y << 6 + upsample_left) - dy;
        while x < width {
            let v;
            if base_x >= 0 {
                v = *topleft.offset(base_x as isize) as libc::c_int * (64 - frac_x)
                    + *topleft.offset((base_x + 1) as isize) as libc::c_int * frac_x;
            } else {
                let base_y = ypos >> 6;
                if !(base_y >= -(1 + upsample_left)) {
                    unreachable!();
                }
                let frac_y = ypos & 0x3e as libc::c_int;
                v = *left.offset(-base_y as isize) as libc::c_int * (64 - frac_y)
                    + *left.offset(-(base_y + 1) as isize) as libc::c_int * frac_y;
            }
            *dst.offset(x as isize) = (v + 32 >> 6) as pixel;
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
    dst: *mut pixel,
    stride: ptrdiff_t,
    topleft_in: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    mut angle: libc::c_int,
    _max_width: libc::c_int,
    _max_height: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let is_sm = angle >> 9 & 0x1 as libc::c_int;
    let enable_intra_edge_filter = angle >> 10;
    angle &= 511 as libc::c_int;
    if !(angle > 180) {
        unreachable!();
    }
    let mut dy = dav1d_dr_intra_derivative[(270 - angle >> 1) as usize] as libc::c_int;
    let mut left_out: [pixel; 128] = [0; 128];
    let left: *const pixel;
    let max_base_y;
    let upsample_left = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, angle - 180, is_sm)
    } else {
        0 as libc::c_int
    };
    if upsample_left != 0 {
        upsample_edge(
            left_out.as_mut_ptr(),
            width + height,
            &*topleft_in.offset(-(width + height) as isize),
            imax(width - height, 0 as libc::c_int),
            width + height + 1,
            bitdepth_max,
        );
        left = &mut *left_out
            .as_mut_ptr()
            .offset((2 * (width + height) - 2) as isize) as *mut pixel;
        max_base_y = 2 * (width + height) - 2;
        dy <<= 1;
    } else {
        let filter_strength = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, angle - 180, is_sm)
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
                width + height + 1,
                filter_strength,
            );
            left = &mut *left_out.as_mut_ptr().offset((width + height - 1) as isize) as *mut pixel;
            max_base_y = width + height - 1;
        } else {
            left = &*topleft_in.offset(-(1 as libc::c_int) as isize) as *const pixel;
            max_base_y = height + imin(width, height) - 1;
        }
    }
    let base_inc = 1 + upsample_left;
    let mut x = 0;
    let mut ypos = dy;
    while x < width {
        let frac = ypos & 0x3e as libc::c_int;
        let mut y = 0;
        let mut base = ypos >> 6;
        while y < height {
            if base < max_base_y {
                let v = *left.offset(-base as isize) as libc::c_int * (64 - frac)
                    + *left.offset(-(base + 1) as isize) as libc::c_int * frac;
                *dst.offset((y as isize * PXSTRIDE(stride) + x as isize) as isize) =
                    (v + 32 >> 6) as pixel;
                y += 1;
                base += base_inc;
            } else {
                loop {
                    *dst.offset((y as isize * PXSTRIDE(stride) + x as isize) as isize) =
                        *left.offset(-max_base_y as isize);
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
    use crate::src::ipred_tmpl::{filter_fn, FLT_INCR};

    filt_idx &= 511 as libc::c_int;
    if !(filt_idx < 5) {
        unreachable!();
    }
    let filter: *const int8_t = (dav1d_filter_intra_taps[filt_idx as usize]).as_ptr();
    let mut top: *const pixel = &*topleft_in.offset(1) as *const pixel;
    let mut y = 0;
    while y < height {
        let mut topleft: *const pixel = &*topleft_in.offset(-y as isize) as *const pixel;
        let mut left: *const pixel = &*topleft.offset(-(1 as libc::c_int) as isize) as *const pixel;
        let mut left_stride: ptrdiff_t = -(1 as libc::c_int) as ptrdiff_t;
        let mut x = 0;
        while x < width {
            let p0 = *topleft as libc::c_int;
            let p1 = *top.offset(0) as libc::c_int;
            let p2 = *top.offset(1) as libc::c_int;
            let p3 = *top.offset(2) as libc::c_int;
            let p4 = *top.offset(3) as libc::c_int;
            let p5 = *left.offset((0 * left_stride) as isize) as libc::c_int;
            let p6 = *left.offset((1 * left_stride) as isize) as libc::c_int;
            let mut ptr: *mut pixel = &mut *dst.offset(x as isize) as *mut pixel;
            let mut flt_ptr: *const int8_t = filter;
            let mut yy = 0;
            while yy < 2 {
                let mut xx = 0;
                while xx < 4 {
                    let acc = filter_fn(flt_ptr, p0, p1, p2, p3, p4, p5, p6);
                    *ptr.offset(xx as isize) =
                        iclip(acc + 8 >> 4, 0 as libc::c_int, bitdepth_max) as pixel;
                    xx += 1;
                    flt_ptr = flt_ptr.offset(FLT_INCR);
                }
                ptr = ptr.offset(PXSTRIDE(stride) as isize);
                yy += 1;
            }
            left = &mut *dst.offset((x + 4 - 1) as isize) as *mut pixel;
            left_stride = PXSTRIDE(stride);
            top = top.offset(4);
            topleft = &*top.offset(-(1 as libc::c_int) as isize) as *const pixel;
            x += 4 as libc::c_int;
        }
        top = &mut *dst
            .offset((PXSTRIDE as unsafe extern "C" fn(ptrdiff_t) -> ptrdiff_t)(stride) as isize)
            as *mut pixel;
        dst = &mut *dst.offset(
            ((PXSTRIDE as unsafe extern "C" fn(ptrdiff_t) -> ptrdiff_t)(stride) * 2) as isize,
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
    let mut y;
    let mut x: i32;
    let ac_orig: *mut int16_t = ac;
    if !(w_pad >= 0 && (w_pad * 4) < width) {
        unreachable!();
    }
    if !(h_pad >= 0 && (h_pad * 4) < height) {
        unreachable!();
    }
    y = 0 as libc::c_int;
    while y < height - 4 * h_pad {
        x = 0 as libc::c_int;
        while x < width - 4 * w_pad {
            let mut ac_sum = *ypx.offset((x << ss_hor) as isize) as libc::c_int;
            if ss_hor != 0 {
                ac_sum += *ypx.offset((x * 2 + 1) as isize) as libc::c_int;
            }
            if ss_ver != 0 {
                ac_sum += *ypx.offset(((x << ss_hor) as isize + PXSTRIDE(stride)) as isize)
                    as libc::c_int;
                if ss_hor != 0 {
                    ac_sum += *ypx.offset(((x * 2 + 1) as isize + PXSTRIDE(stride)) as isize)
                        as libc::c_int;
                }
            }
            *ac.offset(x as isize) = (ac_sum
                << 1 + (ss_ver == 0) as libc::c_int + (ss_hor == 0) as libc::c_int)
                as int16_t;
            x += 1;
        }
        while x < width {
            *ac.offset(x as isize) = *ac.offset((x - 1) as isize);
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
    let log2sz = ctz(width as libc::c_uint) + ctz(height as libc::c_uint);
    let mut sum = (1 as libc::c_int) << log2sz >> 1;
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
    cfl_ac_c(
        ac,
        ypx,
        stride,
        w_pad,
        h_pad,
        cw,
        ch,
        1 as libc::c_int,
        1 as libc::c_int,
    );
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
    cfl_ac_c(
        ac,
        ypx,
        stride,
        w_pad,
        h_pad,
        cw,
        ch,
        1 as libc::c_int,
        0 as libc::c_int,
    );
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
    cfl_ac_c(
        ac,
        ypx,
        stride,
        w_pad,
        h_pad,
        cw,
        ch,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn pal_pred_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    pal: *const uint16_t,
    mut idx: *const uint8_t,
    w: libc::c_int,
    h: libc::c_int,
) {
    let mut y = 0;
    while y < h {
        let mut x = 0;
        while x < w {
            *dst.offset(x as isize) = *pal.offset(*idx.offset(x as isize) as isize);
            x += 1;
        }
        idx = idx.offset(w as isize);
        dst = dst.offset(PXSTRIDE(stride) as isize);
        y += 1;
    }
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64"),))]
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
    (*c).intra_pred[Z1_PRED as usize] = Some(dav1d_ipred_z1_16bpc_ssse3);
    (*c).intra_pred[Z2_PRED as usize] = Some(dav1d_ipred_z2_16bpc_ssse3);
    (*c).intra_pred[Z3_PRED as usize] = Some(dav1d_ipred_z3_16bpc_ssse3);
    (*c).intra_pred[FILTER_PRED as usize] = Some(dav1d_ipred_filter_16bpc_ssse3);

    (*c).cfl_pred[DC_PRED as usize] = Some(dav1d_ipred_cfl_16bpc_ssse3);
    (*c).cfl_pred[DC_128_PRED as usize] = Some(dav1d_ipred_cfl_128_16bpc_ssse3);
    (*c).cfl_pred[TOP_DC_PRED as usize] = Some(dav1d_ipred_cfl_top_16bpc_ssse3);
    (*c).cfl_pred[LEFT_DC_PRED as usize] = Some(dav1d_ipred_cfl_left_16bpc_ssse3);

    (*c).cfl_ac[(DAV1D_PIXEL_LAYOUT_I420 - 1) as usize] = Some(dav1d_ipred_cfl_ac_420_16bpc_ssse3);
    (*c).cfl_ac[(DAV1D_PIXEL_LAYOUT_I422 - 1) as usize] = Some(dav1d_ipred_cfl_ac_422_16bpc_ssse3);
    (*c).cfl_ac[(DAV1D_PIXEL_LAYOUT_I444 - 1) as usize] = Some(dav1d_ipred_cfl_ac_444_16bpc_ssse3);

    (*c).pal_pred = Some(dav1d_pal_pred_16bpc_ssse3);

    #[cfg(target_arch = "x86_64")]
    {
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

        (*c).cfl_ac[(DAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
            Some(dav1d_ipred_cfl_ac_420_16bpc_avx2);
        (*c).cfl_ac[(DAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
            Some(dav1d_ipred_cfl_ac_422_16bpc_avx2);
        (*c).cfl_ac[(DAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
            Some(dav1d_ipred_cfl_ac_444_16bpc_avx2);

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
}

#[cfg(feature = "asm")]
use crate::src::cpu::dav1d_get_cpu_flags;

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
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
    #[cfg(target_arch = "aarch64")]
    {
        (*c).intra_pred[Z1_PRED as usize] = Some(ipred_z1_neon);
        (*c).intra_pred[Z2_PRED as usize] = Some(ipred_z2_neon);
        (*c).intra_pred[Z3_PRED as usize] = Some(ipred_z3_neon);
    }
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

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
unsafe extern "C" fn ipred_z3_neon(
    dst: *mut pixel,
    stride: ptrdiff_t,
    topleft_in: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    mut angle: libc::c_int,
    _max_width: libc::c_int,
    _max_height: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let is_sm = angle >> 9 & 0x1 as libc::c_int;
    let enable_intra_edge_filter = angle >> 10;
    angle &= 511 as libc::c_int;
    if !(angle > 180) {
        unreachable!();
    }
    let mut dy = dav1d_dr_intra_derivative[(270 - angle >> 1) as usize] as libc::c_int;
    let mut flipped: [pixel; 144] = [0; 144];
    let mut left_out: [pixel; 286] = [0; 286];
    let max_base_y;
    let upsample_left = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, angle - 180, is_sm)
    } else {
        0 as libc::c_int
    };
    if upsample_left != 0 {
        flipped[0] = *topleft_in.offset(0);
        dav1d_ipred_reverse_16bpc_neon(
            &mut *flipped.as_mut_ptr().offset(1),
            &*topleft_in.offset(0),
            height + imax(width, height),
        );
        dav1d_ipred_z1_upsample_edge_16bpc_neon(
            left_out.as_mut_ptr(),
            width + height,
            flipped.as_mut_ptr(),
            height + imin(width, height),
            bitdepth_max,
        );
        max_base_y = 2 * (width + height) - 2;
        dy <<= 1;
    } else {
        let filter_strength = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, angle - 180, is_sm)
        } else {
            0 as libc::c_int
        };
        if filter_strength != 0 {
            flipped[0] = *topleft_in.offset(0);
            dav1d_ipred_reverse_16bpc_neon(
                &mut *flipped.as_mut_ptr().offset(1),
                &*topleft_in.offset(0),
                height + imax(width, height),
            );
            dav1d_ipred_z1_filter_edge_16bpc_neon(
                left_out.as_mut_ptr(),
                width + height,
                flipped.as_mut_ptr(),
                height + imin(width, height),
                filter_strength,
            );
            max_base_y = width + height - 1;
        } else {
            dav1d_ipred_reverse_16bpc_neon(
                left_out.as_mut_ptr(),
                &*topleft_in.offset(0),
                height + imin(width, height),
            );
            max_base_y = height + imin(width, height) - 1;
        }
    }
    let base_inc = 1 + upsample_left;
    let pad_pixels = imax(64 - max_base_y - 1, height + 15);
    dav1d_ipred_pixel_set_16bpc_neon(
        &mut *left_out.as_mut_ptr().offset((max_base_y + 1) as isize) as *mut pixel,
        left_out[max_base_y as usize],
        (pad_pixels * base_inc) as libc::c_int,
    );
    if upsample_left != 0 {
        dav1d_ipred_z3_fill2_16bpc_neon(
            dst,
            stride,
            left_out.as_mut_ptr(),
            width,
            height,
            dy,
            max_base_y,
        );
    } else {
        dav1d_ipred_z3_fill1_16bpc_neon(
            dst,
            stride,
            left_out.as_mut_ptr(),
            width,
            height,
            dy,
            max_base_y,
        );
    };
}

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
unsafe extern "C" fn ipred_z2_neon(
    dst: *mut pixel,
    stride: ptrdiff_t,
    topleft_in: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    mut angle: libc::c_int,
    max_width: libc::c_int,
    max_height: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    use crate::include::common::bitdepth::BitDepth;
    use crate::include::common::bitdepth::BitDepth16;

    let is_sm = angle >> 9 & 0x1 as libc::c_int;
    let enable_intra_edge_filter = angle >> 10;
    angle &= 511 as libc::c_int;
    if !(angle > 90 && angle < 180) {
        unreachable!();
    }
    let mut dy = dav1d_dr_intra_derivative[((angle - 90) >> 1) as usize] as libc::c_int;
    let mut dx = dav1d_dr_intra_derivative[((180 - angle) >> 1) as usize] as libc::c_int;
    let mut buf: [pixel; 3 * (64 + 1)] = [0; 3 * (64 + 1)]; // NOTE: C code doesn't initialize

    // The asm can underread below the start of top[] and left[]; to avoid
    // surprising behaviour, make sure this is within the allocated stack space.
    let left_offset: isize = 2 * (64 + 1);
    let top_offset: isize = 1 * (64 + 1);
    let flipped_offset: isize = 0 * (64 + 1);

    let upsample_left = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, 180 - angle, is_sm)
    } else {
        0 as libc::c_int
    };
    let upsample_above = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, angle - 90, is_sm)
    } else {
        0 as libc::c_int
    };

    if upsample_above != 0 {
        dav1d_ipred_z2_upsample_edge_16bpc_neon(
            buf.as_mut_ptr().offset(top_offset),
            width,
            topleft_in,
            bitdepth_max,
        );
        dx <<= 1;
    } else {
        let filter_strength = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, angle - 90, is_sm)
        } else {
            0 as libc::c_int
        };

        if filter_strength != 0 {
            dav1d_ipred_z1_filter_edge_16bpc_neon(
                buf.as_mut_ptr().offset(1 + top_offset),
                imin(max_width, width),
                topleft_in,
                width,
                filter_strength,
            );

            if max_width < width {
                memcpy(
                    buf.as_mut_ptr().offset(top_offset + 1 + max_width as isize)
                        as *mut libc::c_void,
                    topleft_in.offset(1 + max_width as isize) as *const libc::c_void,
                    ((width - max_width) as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong),
                );
            }
        } else {
            BitDepth16::pixel_copy(
                &mut buf[1 + top_offset as usize..],
                core::slice::from_raw_parts(topleft_in.offset(1), width as usize),
                width as usize,
            );
        }
    }

    if upsample_left != 0 {
        buf[flipped_offset as usize] = *topleft_in;
        dav1d_ipred_reverse_16bpc_neon(
            &mut *buf.as_mut_ptr().offset(1 + flipped_offset),
            topleft_in,
            height,
        );
        dav1d_ipred_z2_upsample_edge_16bpc_neon(
            buf.as_mut_ptr().offset(left_offset),
            height,
            buf.as_ptr().offset(flipped_offset),
            bitdepth_max,
        );
        dy <<= 1;
    } else {
        let filter_strength = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, 180 - angle, is_sm)
        } else {
            0 as libc::c_int
        };
        if filter_strength != 0 {
            buf[flipped_offset as usize] = *topleft_in;
            dav1d_ipred_reverse_16bpc_neon(
                &mut *buf.as_mut_ptr().offset(1 + flipped_offset),
                topleft_in,
                height,
            );
            dav1d_ipred_z1_filter_edge_16bpc_neon(
                buf.as_mut_ptr().offset(1 + left_offset),
                imin(max_height, height),
                buf.as_ptr().offset(flipped_offset),
                height,
                filter_strength,
            );
            if max_height < height {
                memcpy(
                    buf.as_mut_ptr()
                        .offset(left_offset + 1 + max_height as isize)
                        as *mut libc::c_void,
                    buf.as_mut_ptr()
                        .offset(flipped_offset + 1 + max_height as isize)
                        as *const libc::c_void,
                    ((height - max_height) as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong),
                );
            }
        } else {
            dav1d_ipred_reverse_16bpc_neon(
                buf.as_mut_ptr().offset(left_offset + 1),
                topleft_in,
                height,
            );
        }
    }
    buf[top_offset as usize] = *topleft_in;
    buf[left_offset as usize] = *topleft_in;

    if upsample_above != 0 && upsample_left != 0 {
        unreachable!();
    }

    if upsample_above == 0 && upsample_left == 0 {
        dav1d_ipred_z2_fill1_16bpc_neon(
            dst,
            stride,
            buf.as_ptr().offset(top_offset),
            buf.as_ptr().offset(left_offset),
            width,
            height,
            dx,
            dy,
        );
    } else if upsample_above != 0 {
        dav1d_ipred_z2_fill2_16bpc_neon(
            dst,
            stride,
            buf.as_ptr().offset(top_offset),
            buf.as_ptr().offset(left_offset),
            width,
            height,
            dx,
            dy,
        );
    } else {
        dav1d_ipred_z2_fill3_16bpc_neon(
            dst,
            stride,
            buf.as_ptr().offset(top_offset),
            buf.as_ptr().offset(left_offset),
            width,
            height,
            dx,
            dy,
        );
    };
}

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
unsafe extern "C" fn ipred_z1_neon(
    dst: *mut pixel,
    stride: ptrdiff_t,
    topleft_in: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    mut angle: libc::c_int,
    _max_width: libc::c_int,
    _max_height: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let is_sm = angle >> 9 & 0x1 as libc::c_int;
    let enable_intra_edge_filter = angle >> 10;
    angle &= 511 as libc::c_int;
    let mut dx = dav1d_dr_intra_derivative[(angle >> 1) as usize] as libc::c_int;
    const top_out_size: usize = 64 + 64 * (64 + 15) * 2 + 16;
    let mut top_out: [pixel; top_out_size] = [0; top_out_size];
    let max_base_x;
    let upsample_above = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, 90 - angle, is_sm)
    } else {
        0 as libc::c_int
    };
    if upsample_above != 0 {
        dav1d_ipred_z1_upsample_edge_16bpc_neon(
            top_out.as_mut_ptr(),
            width + height,
            topleft_in,
            width + imin(width, height),
            bitdepth_max,
        );
        max_base_x = 2 * (width + height) - 2;
        dx <<= 1;
    } else {
        let filter_strength = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, 90 - angle, is_sm)
        } else {
            0 as libc::c_int
        };
        if filter_strength != 0 {
            dav1d_ipred_z1_filter_edge_16bpc_neon(
                top_out.as_mut_ptr(),
                width + height,
                topleft_in,
                width + imin(width, height),
                filter_strength,
            );
            max_base_x = width + height - 1;
        } else {
            max_base_x = width + imin(width, height) - 1;
            memcpy(
                top_out.as_mut_ptr() as *mut libc::c_void,
                &*topleft_in.offset(1) as *const pixel as *const libc::c_void,
                ((max_base_x + 1) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong),
            );
        }
    }
    let base_inc = 1 + upsample_above;
    let pad_pixels = width + 15;
    dav1d_ipred_pixel_set_16bpc_neon(
        &mut *top_out.as_mut_ptr().offset((max_base_x + 1) as isize) as *mut pixel,
        top_out[max_base_x as usize],
        (pad_pixels * base_inc) as libc::c_int,
    );
    if upsample_above != 0 {
        dav1d_ipred_z1_fill2_16bpc_neon(
            dst,
            stride,
            top_out.as_mut_ptr(),
            width,
            height,
            dx,
            max_base_x,
        );
    } else {
        dav1d_ipred_z1_fill1_16bpc_neon(
            dst,
            stride,
            top_out.as_mut_ptr(),
            width,
            height,
            dx,
            max_base_x,
        );
    };
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_intra_pred_dsp_init_16bpc(c: *mut Dav1dIntraPredDSPContext) {
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
