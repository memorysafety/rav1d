use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
use cfg_if::cfg_if;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static dav1d_mc_subpel_filters: [[[int8_t; 8]; 15]; 6];
    static dav1d_mc_warp_filter: [[int8_t; 8]; 193];
    static dav1d_resize_filter: [[int8_t; 8]; 64];
    static dav1d_obmc_masks: [uint8_t; 64];
}

#[cfg(feature = "asm")]
extern "C" {
    static mut dav1d_cpu_flags: libc::c_uint;
    static mut dav1d_cpu_flags_mask: libc::c_uint;
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
extern "C" {
    fn dav1d_put_8tap_regular_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_regular_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_regular_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_regular_smooth_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_regular_smooth_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_regular_smooth_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_regular_sharp_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_regular_sharp_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_regular_sharp_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_smooth_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_smooth_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_smooth_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_smooth_regular_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_smooth_regular_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_smooth_regular_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_smooth_sharp_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_smooth_sharp_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_smooth_sharp_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_sharp_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_sharp_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_sharp_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_sharp_regular_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_bilin_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_16bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_sharp_smooth_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_sharp_regular_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_regular_16bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_sharp_16bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_smooth_16bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_w_mask_420_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *mut uint8_t,
        sign: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_sharp_16bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_regular_16bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_smooth_16bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_16bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_bilin_16bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_scaled_regular_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_scaled_regular_smooth_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_scaled_regular_sharp_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_scaled_smooth_regular_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_scaled_smooth_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_scaled_smooth_sharp_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_scaled_sharp_regular_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_scaled_sharp_smooth_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_scaled_sharp_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_bilin_scaled_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_regular_16bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_regular_smooth_16bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_regular_sharp_16bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_smooth_regular_16bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_smooth_16bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_smooth_sharp_16bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_sharp_regular_16bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_sharp_smooth_16bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_sharp_16bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_bilin_scaled_16bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_avg_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_w_avg_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        weight: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_mask_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *const uint8_t,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_w_mask_444_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *mut uint8_t,
        sign: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_w_mask_422_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *mut uint8_t,
        sign: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_w_mask_420_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *mut uint8_t,
        sign: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_blend_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
        mask: *const uint8_t,
    );
    fn dav1d_blend_v_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_blend_h_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_warp_affine_8x8_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        abcd: *const int16_t,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_warp_affine_8x8t_16bpc_ssse3(
        tmp: *mut int16_t,
        tmp_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        abcd: *const int16_t,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_emu_edge_16bpc_ssse3(
        bw: intptr_t,
        bh: intptr_t,
        iw: intptr_t,
        ih: intptr_t,
        x: intptr_t,
        y: intptr_t,
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
    );
    fn dav1d_resize_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        dst_w: libc::c_int,
        h: libc::c_int,
        src_w: libc::c_int,
        dx: libc::c_int,
        mx: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_sharp_regular_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_sharp_smooth_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_bilin_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_16bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_smooth_16bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_sharp_16bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_regular_16bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_16bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_sharp_16bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_regular_16bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_smooth_16bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_16bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_bilin_16bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_scaled_regular_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_scaled_regular_smooth_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_scaled_regular_sharp_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_scaled_smooth_regular_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_scaled_smooth_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_scaled_smooth_sharp_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_scaled_sharp_regular_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_scaled_sharp_smooth_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_scaled_sharp_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_bilin_scaled_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_regular_16bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_regular_smooth_16bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_regular_sharp_16bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_smooth_regular_16bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_smooth_16bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_smooth_sharp_16bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_sharp_regular_16bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_sharp_smooth_16bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_sharp_16bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_bilin_scaled_16bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_avg_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_w_avg_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        weight: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_mask_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *const uint8_t,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_w_mask_444_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *mut uint8_t,
        sign: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_w_mask_422_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *mut uint8_t,
        sign: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_w_mask_420_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *mut uint8_t,
        sign: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_blend_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
        mask: *const uint8_t,
    );
    fn dav1d_blend_v_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_blend_h_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_warp_affine_8x8_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        abcd: *const int16_t,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_warp_affine_8x8t_16bpc_avx2(
        tmp: *mut int16_t,
        tmp_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        abcd: *const int16_t,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_emu_edge_16bpc_avx2(
        bw: intptr_t,
        bh: intptr_t,
        iw: intptr_t,
        ih: intptr_t,
        x: intptr_t,
        y: intptr_t,
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
    );
    fn dav1d_resize_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        dst_w: libc::c_int,
        h: libc::c_int,
        src_w: libc::c_int,
        dx: libc::c_int,
        mx: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_sharp_smooth_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_bilin_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_16bpc_avx512icl(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_smooth_16bpc_avx512icl(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_sharp_16bpc_avx512icl(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_regular_16bpc_avx512icl(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_16bpc_avx512icl(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_sharp_16bpc_avx512icl(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_regular_16bpc_avx512icl(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_smooth_16bpc_avx512icl(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_16bpc_avx512icl(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_bilin_16bpc_avx512icl(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_avg_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_w_avg_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        weight: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_mask_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *const uint8_t,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_w_mask_444_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *mut uint8_t,
        sign: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_w_mask_422_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *mut uint8_t,
        sign: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_16bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_blend_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
        mask: *const uint8_t,
    );
    fn dav1d_blend_v_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_blend_h_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_warp_affine_8x8_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        abcd: *const int16_t,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_warp_affine_8x8t_16bpc_avx512icl(
        tmp: *mut int16_t,
        tmp_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        abcd: *const int16_t,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_resize_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        dst_w: libc::c_int,
        h: libc::c_int,
        src_w: libc::c_int,
        dx: libc::c_int,
        mx: libc::c_int,
        bitdepth_max: libc::c_int,
    );
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
extern "C" {
    fn dav1d_put_8tap_regular_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_regular_smooth_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_regular_sharp_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_smooth_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_smooth_regular_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_smooth_sharp_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_sharp_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_sharp_regular_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_8tap_sharp_smooth_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_put_bilin_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_16bpc_neon(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_smooth_16bpc_neon(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_sharp_16bpc_neon(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_regular_16bpc_neon(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_16bpc_neon(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_sharp_16bpc_neon(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_regular_16bpc_neon(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_smooth_16bpc_neon(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_16bpc_neon(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_prep_bilin_16bpc_neon(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_avg_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_w_avg_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        weight: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_mask_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *const uint8_t,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_blend_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
        mask: *const uint8_t,
    );
    fn dav1d_blend_h_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_blend_v_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_w_mask_444_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *mut uint8_t,
        sign: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_w_mask_422_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *mut uint8_t,
        sign: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_w_mask_420_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *mut uint8_t,
        sign: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_warp_affine_8x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        abcd: *const int16_t,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_warp_affine_8x8t_16bpc_neon(
        tmp: *mut int16_t,
        tmp_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        abcd: *const int16_t,
        mx: libc::c_int,
        my: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_emu_edge_16bpc_neon(
        bw: intptr_t,
        bh: intptr_t,
        iw: intptr_t,
        ih: intptr_t,
        x: intptr_t,
        y: intptr_t,
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
    );
}

pub type pixel = uint16_t;

use crate::include::dav1d::headers::DAV1D_FILTER_8TAP_SHARP;
use crate::include::dav1d::headers::DAV1D_FILTER_8TAP_SMOOTH;
use crate::include::dav1d::headers::DAV1D_FILTER_8TAP_REGULAR;
use crate::src::levels::FILTER_2D_BILINEAR;
use crate::src::levels::FILTER_2D_8TAP_SMOOTH_SHARP;
use crate::src::levels::FILTER_2D_8TAP_SMOOTH;
use crate::src::levels::FILTER_2D_8TAP_SMOOTH_REGULAR;
use crate::src::levels::FILTER_2D_8TAP_SHARP;
use crate::src::levels::FILTER_2D_8TAP_SHARP_SMOOTH;
use crate::src::levels::FILTER_2D_8TAP_SHARP_REGULAR;
use crate::src::levels::FILTER_2D_8TAP_REGULAR_SHARP;
use crate::src::levels::FILTER_2D_8TAP_REGULAR_SMOOTH;
use crate::src::levels::FILTER_2D_8TAP_REGULAR;
pub type mc_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mc_scaled_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type warp8x8_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mct_fn = Option::<
    unsafe extern "C" fn(
        *mut int16_t,
        *const pixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mct_scaled_fn = Option::<
    unsafe extern "C" fn(
        *mut int16_t,
        *const pixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type warp8x8t_fn = Option::<
    unsafe extern "C" fn(
        *mut int16_t,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type avg_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type w_avg_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mask_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        *const uint8_t,
        libc::c_int,
    ) -> (),
>;
pub type w_mask_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        *mut uint8_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type blend_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        libc::c_int,
        libc::c_int,
        *const uint8_t,
    ) -> (),
>;
pub type blend_dir_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type emu_edge_fn = Option::<
    unsafe extern "C" fn(
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
    ) -> (),
>;
pub type resize_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dMCDSPContext {
    pub mc: [mc_fn; 10],
    pub mc_scaled: [mc_scaled_fn; 10],
    pub mct: [mct_fn; 10],
    pub mct_scaled: [mct_scaled_fn; 10],
    pub avg: avg_fn,
    pub w_avg: w_avg_fn,
    pub mask: mask_fn,
    pub w_mask: [w_mask_fn; 3],
    pub blend: blend_fn,
    pub blend_v: blend_dir_fn,
    pub blend_h: blend_dir_fn,
    pub warp8x8: warp8x8_fn,
    pub warp8x8t: warp8x8t_fn,
    pub emu_edge: emu_edge_fn,
    pub resize: resize_fn,
}
use crate::include::common::attributes::clz;
use crate::include::common::intops::imin;
use crate::include::common::intops::iclip;
#[inline]
unsafe extern "C" fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 != 0 {
        unreachable!();
    }
    return x >> 1 as libc::c_int;
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
unsafe extern "C" fn put_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    mut h: libc::c_int,
) {
    loop {
        memcpy(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            (w << 1 as libc::c_int) as libc::c_ulong,
        );
        dst = dst.offset(dst_stride as isize);
        src = src.offset(src_stride as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
#[inline(never)]
unsafe extern "C" fn prep_c(
    mut tmp: *mut int16_t,
    mut src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    mut h: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14 as libc::c_int
        - (32 as libc::c_int - clz(bitdepth_max as libc::c_uint));
    loop {
        let mut x = 0;
        while x < w {
            *tmp
                .offset(
                    x as isize,
                ) = (((*src.offset(x as isize) as libc::c_int) << intermediate_bits)
                - 8192 as libc::c_int) as int16_t;
            x += 1;
        }
        tmp = tmp.offset(w as isize);
        src = src.offset(src_stride as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
#[inline(never)]
unsafe extern "C" fn put_8tap_c(
    mut dst: *mut pixel,
    mut dst_stride: ptrdiff_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: libc::c_int,
    mut h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    filter_type: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14 as libc::c_int
        - (32 as libc::c_int - clz(bitdepth_max as libc::c_uint));
    let intermediate_rnd: libc::c_int = 32 as libc::c_int
        + ((1 as libc::c_int) << 6 as libc::c_int - intermediate_bits
            >> 1 as libc::c_int);
    let fh: *const int8_t = if mx == 0 {
        0 as *const int8_t
    } else if w > 4 as libc::c_int {
        (dav1d_mc_subpel_filters[(filter_type & 3 as libc::c_int)
            as usize][(mx - 1 as libc::c_int) as usize])
            .as_ptr()
    } else {
        (dav1d_mc_subpel_filters[(3 as libc::c_int + (filter_type & 1 as libc::c_int))
            as usize][(mx - 1 as libc::c_int) as usize])
            .as_ptr()
    };
    let fv: *const int8_t = if my == 0 {
        0 as *const int8_t
    } else if h > 4 as libc::c_int {
        (dav1d_mc_subpel_filters[(filter_type >> 2 as libc::c_int)
            as usize][(my - 1 as libc::c_int) as usize])
            .as_ptr()
    } else {
        (dav1d_mc_subpel_filters[(3 as libc::c_int
            + (filter_type >> 2 as libc::c_int & 1 as libc::c_int))
            as usize][(my - 1 as libc::c_int) as usize])
            .as_ptr()
    };
    dst_stride = PXSTRIDE(dst_stride);
    src_stride = PXSTRIDE(src_stride);
    if !fh.is_null() {
        if !fv.is_null() {
            let mut tmp_h: libc::c_int = h + 7 as libc::c_int;
            let mut mid: [int16_t; 17280] = [0; 17280];
            let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
            src = src
                .offset(-((src_stride * 3) as isize));
            loop {
                let mut x = 0;
                while x < w {
                    *mid_ptr
                        .offset(
                            x as isize,
                        ) = (*fh.offset(0) as libc::c_int
                        * *src
                            .offset(
                                (x + -(3 as libc::c_int) * 1 as libc::c_int) as isize,
                            ) as libc::c_int
                        + *fh.offset(1) as libc::c_int
                            * *src
                                .offset(
                                    (x + -(2 as libc::c_int) * 1 as libc::c_int) as isize,
                                ) as libc::c_int
                        + *fh.offset(2) as libc::c_int
                            * *src
                                .offset(
                                    (x + -(1 as libc::c_int) * 1 as libc::c_int) as isize,
                                ) as libc::c_int
                        + *fh.offset(3) as libc::c_int
                            * *src
                                .offset((x + 0 as libc::c_int * 1 as libc::c_int) as isize)
                                as libc::c_int
                        + *fh.offset(4) as libc::c_int
                            * *src
                                .offset((x + 1 as libc::c_int * 1 as libc::c_int) as isize)
                                as libc::c_int
                        + *fh.offset(5) as libc::c_int
                            * *src
                                .offset((x + 2 as libc::c_int * 1 as libc::c_int) as isize)
                                as libc::c_int
                        + *fh.offset(6) as libc::c_int
                            * *src
                                .offset((x + 3 as libc::c_int * 1 as libc::c_int) as isize)
                                as libc::c_int
                        + *fh.offset(7) as libc::c_int
                            * *src
                                .offset((x + 4 as libc::c_int * 1 as libc::c_int) as isize)
                                as libc::c_int
                        + ((1 as libc::c_int) << 6 as libc::c_int - intermediate_bits
                            >> 1 as libc::c_int) >> 6 as libc::c_int - intermediate_bits)
                        as int16_t;
                    x += 1;
                }
                mid_ptr = mid_ptr.offset(128);
                src = src.offset(src_stride as isize);
                tmp_h -= 1;
                if !(tmp_h != 0) {
                    break;
                }
            }
            mid_ptr = mid
                .as_mut_ptr()
                .offset((128 as libc::c_int * 3 as libc::c_int) as isize);
            loop {
                let mut x_0 = 0;
                while x_0 < w {
                    *dst
                        .offset(
                            x_0 as isize,
                        ) = iclip(
                        *fv.offset(0) as libc::c_int
                            * *mid_ptr
                                .offset(
                                    (x_0 + -(3 as libc::c_int) * 128 as libc::c_int) as isize,
                                ) as libc::c_int
                            + *fv.offset(1) as libc::c_int
                                * *mid_ptr
                                    .offset(
                                        (x_0 + -(2 as libc::c_int) * 128 as libc::c_int) as isize,
                                    ) as libc::c_int
                            + *fv.offset(2) as libc::c_int
                                * *mid_ptr
                                    .offset(
                                        (x_0 + -(1 as libc::c_int) * 128 as libc::c_int) as isize,
                                    ) as libc::c_int
                            + *fv.offset(3) as libc::c_int
                                * *mid_ptr
                                    .offset(
                                        (x_0 + 0 as libc::c_int * 128 as libc::c_int) as isize,
                                    ) as libc::c_int
                            + *fv.offset(4) as libc::c_int
                                * *mid_ptr
                                    .offset(
                                        (x_0 + 1 as libc::c_int * 128 as libc::c_int) as isize,
                                    ) as libc::c_int
                            + *fv.offset(5) as libc::c_int
                                * *mid_ptr
                                    .offset(
                                        (x_0 + 2 as libc::c_int * 128 as libc::c_int) as isize,
                                    ) as libc::c_int
                            + *fv.offset(6) as libc::c_int
                                * *mid_ptr
                                    .offset(
                                        (x_0 + 3 as libc::c_int * 128 as libc::c_int) as isize,
                                    ) as libc::c_int
                            + *fv.offset(7) as libc::c_int
                                * *mid_ptr
                                    .offset(
                                        (x_0 + 4 as libc::c_int * 128 as libc::c_int) as isize,
                                    ) as libc::c_int
                            + ((1 as libc::c_int) << 6 as libc::c_int + intermediate_bits
                                >> 1 as libc::c_int)
                            >> 6 as libc::c_int + intermediate_bits,
                        0 as libc::c_int,
                        bitdepth_max,
                    ) as pixel;
                    x_0 += 1;
                }
                mid_ptr = mid_ptr.offset(128);
                dst = dst.offset(dst_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        } else {
            loop {
                let mut x_1 = 0;
                while x_1 < w {
                    *dst
                        .offset(
                            x_1 as isize,
                        ) = iclip(
                        *fh.offset(0) as libc::c_int
                            * *src
                                .offset(
                                    (x_1 + -(3 as libc::c_int) * 1 as libc::c_int) as isize,
                                ) as libc::c_int
                            + *fh.offset(1) as libc::c_int
                                * *src
                                    .offset(
                                        (x_1 + -(2 as libc::c_int) * 1 as libc::c_int) as isize,
                                    ) as libc::c_int
                            + *fh.offset(2) as libc::c_int
                                * *src
                                    .offset(
                                        (x_1 + -(1 as libc::c_int) * 1 as libc::c_int) as isize,
                                    ) as libc::c_int
                            + *fh.offset(3) as libc::c_int
                                * *src
                                    .offset(
                                        (x_1 + 0 as libc::c_int * 1 as libc::c_int) as isize,
                                    ) as libc::c_int
                            + *fh.offset(4) as libc::c_int
                                * *src
                                    .offset(
                                        (x_1 + 1 as libc::c_int * 1 as libc::c_int) as isize,
                                    ) as libc::c_int
                            + *fh.offset(5) as libc::c_int
                                * *src
                                    .offset(
                                        (x_1 + 2 as libc::c_int * 1 as libc::c_int) as isize,
                                    ) as libc::c_int
                            + *fh.offset(6) as libc::c_int
                                * *src
                                    .offset(
                                        (x_1 + 3 as libc::c_int * 1 as libc::c_int) as isize,
                                    ) as libc::c_int
                            + *fh.offset(7) as libc::c_int
                                * *src
                                    .offset(
                                        (x_1 + 4 as libc::c_int * 1 as libc::c_int) as isize,
                                    ) as libc::c_int + intermediate_rnd >> 6 as libc::c_int,
                        0 as libc::c_int,
                        bitdepth_max,
                    ) as pixel;
                    x_1 += 1;
                }
                dst = dst.offset(dst_stride as isize);
                src = src.offset(src_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        }
    } else if !fv.is_null() {
        loop {
            let mut x_2 = 0;
            while x_2 < w {
                *dst
                    .offset(
                        x_2 as isize,
                    ) = iclip(
                    *fv.offset(0) as libc::c_int
                        * *src
                            .offset(
                                (x_2 as isize
                                    + -(3 as libc::c_int) as isize * src_stride) as isize,
                            ) as libc::c_int
                        + *fv.offset(1) as libc::c_int
                            * *src
                                .offset(
                                    (x_2 as isize
                                        + -(2 as libc::c_int) as isize * src_stride) as isize,
                                ) as libc::c_int
                        + *fv.offset(2) as libc::c_int
                            * *src
                                .offset(
                                    (x_2 as isize
                                        + -(1 as libc::c_int) as isize * src_stride) as isize,
                                ) as libc::c_int
                        + *fv.offset(3) as libc::c_int
                            * *src
                                .offset(
                                    (x_2 as isize
                                        + 0 * src_stride) as isize,
                                ) as libc::c_int
                        + *fv.offset(4) as libc::c_int
                            * *src
                                .offset(
                                    (x_2 as isize
                                        + 1 * src_stride) as isize,
                                ) as libc::c_int
                        + *fv.offset(5) as libc::c_int
                            * *src
                                .offset(
                                    (x_2 as isize
                                        + 2 * src_stride) as isize,
                                ) as libc::c_int
                        + *fv.offset(6) as libc::c_int
                            * *src
                                .offset(
                                    (x_2 as isize
                                        + 3 * src_stride) as isize,
                                ) as libc::c_int
                        + *fv.offset(7) as libc::c_int
                            * *src
                                .offset(
                                    (x_2 as isize
                                        + 4 * src_stride) as isize,
                                ) as libc::c_int
                        + ((1 as libc::c_int) << 6 as libc::c_int >> 1 as libc::c_int)
                        >> 6 as libc::c_int,
                    0 as libc::c_int,
                    bitdepth_max,
                ) as pixel;
                x_2 += 1;
            }
            dst = dst.offset(dst_stride as isize);
            src = src.offset(src_stride as isize);
            h -= 1;
            if !(h != 0) {
                break;
            }
        }
    } else {
        put_c(dst, dst_stride, src, src_stride, w, h);
    };
}
#[inline(never)]
unsafe extern "C" fn put_8tap_scaled_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: libc::c_int,
    mut h: libc::c_int,
    mx: libc::c_int,
    mut my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    filter_type: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14 as libc::c_int
        - (32 as libc::c_int - clz(bitdepth_max as libc::c_uint));
    let intermediate_rnd: libc::c_int = (1 as libc::c_int) << intermediate_bits
        >> 1 as libc::c_int;
    let mut tmp_h: libc::c_int = ((h - 1 as libc::c_int) * dy + my >> 10 as libc::c_int)
        + 8 as libc::c_int;
    let mut mid: [int16_t; 33664] = [0; 33664];
    let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
    src_stride = PXSTRIDE(src_stride);
    src = src.offset(-((src_stride * 3) as isize));
    loop {
        let mut x: libc::c_int = 0;
        let mut imx: libc::c_int = mx;
        let mut ioff = 0;
        x = 0 as libc::c_int;
        while x < w {
            let fh: *const int8_t = if imx >> 6 as libc::c_int == 0 {
                0 as *const int8_t
            } else if w > 4 as libc::c_int {
                (dav1d_mc_subpel_filters[(filter_type & 3 as libc::c_int)
                    as usize][((imx >> 6 as libc::c_int) - 1 as libc::c_int) as usize])
                    .as_ptr()
            } else {
                (dav1d_mc_subpel_filters[(3 as libc::c_int
                    + (filter_type & 1 as libc::c_int))
                    as usize][((imx >> 6 as libc::c_int) - 1 as libc::c_int) as usize])
                    .as_ptr()
            };
            *mid_ptr
                .offset(
                    x as isize,
                ) = (if !fh.is_null() {
                *fh.offset(0) as libc::c_int
                    * *src
                        .offset((ioff + -(3 as libc::c_int) * 1 as libc::c_int) as isize)
                        as libc::c_int
                    + *fh.offset(1) as libc::c_int
                        * *src
                            .offset(
                                (ioff + -(2 as libc::c_int) * 1 as libc::c_int) as isize,
                            ) as libc::c_int
                    + *fh.offset(2) as libc::c_int
                        * *src
                            .offset(
                                (ioff + -(1 as libc::c_int) * 1 as libc::c_int) as isize,
                            ) as libc::c_int
                    + *fh.offset(3) as libc::c_int
                        * *src
                            .offset(
                                (ioff + 0 as libc::c_int * 1 as libc::c_int) as isize,
                            ) as libc::c_int
                    + *fh.offset(4) as libc::c_int
                        * *src
                            .offset(
                                (ioff + 1 as libc::c_int * 1 as libc::c_int) as isize,
                            ) as libc::c_int
                    + *fh.offset(5) as libc::c_int
                        * *src
                            .offset(
                                (ioff + 2 as libc::c_int * 1 as libc::c_int) as isize,
                            ) as libc::c_int
                    + *fh.offset(6) as libc::c_int
                        * *src
                            .offset(
                                (ioff + 3 as libc::c_int * 1 as libc::c_int) as isize,
                            ) as libc::c_int
                    + *fh.offset(7) as libc::c_int
                        * *src
                            .offset(
                                (ioff + 4 as libc::c_int * 1 as libc::c_int) as isize,
                            ) as libc::c_int
                    + ((1 as libc::c_int) << 6 as libc::c_int - intermediate_bits
                        >> 1 as libc::c_int) >> 6 as libc::c_int - intermediate_bits
            } else {
                (*src.offset(ioff as isize) as libc::c_int) << intermediate_bits
            }) as int16_t;
            imx += dx;
            ioff += imx >> 10 as libc::c_int;
            imx &= 0x3ff as libc::c_int;
            x += 1;
        }
        mid_ptr = mid_ptr.offset(128);
        src = src.offset(src_stride as isize);
        tmp_h -= 1;
        if !(tmp_h != 0) {
            break;
        }
    }
    mid_ptr = mid.as_mut_ptr().offset((128 as libc::c_int * 3 as libc::c_int) as isize);
    let mut y = 0;
    while y < h {
        let mut x_0: libc::c_int = 0;
        let fv: *const int8_t = if my >> 6 as libc::c_int == 0 {
            0 as *const int8_t
        } else if h > 4 as libc::c_int {
            (dav1d_mc_subpel_filters[(filter_type >> 2 as libc::c_int)
                as usize][((my >> 6 as libc::c_int) - 1 as libc::c_int) as usize])
                .as_ptr()
        } else {
            (dav1d_mc_subpel_filters[(3 as libc::c_int
                + (filter_type >> 2 as libc::c_int & 1 as libc::c_int))
                as usize][((my >> 6 as libc::c_int) - 1 as libc::c_int) as usize])
                .as_ptr()
        };
        x_0 = 0 as libc::c_int;
        while x_0 < w {
            *dst
                .offset(
                    x_0 as isize,
                ) = (if !fv.is_null() {
                iclip(
                    *fv.offset(0) as libc::c_int
                        * *mid_ptr
                            .offset(
                                (x_0 + -(3 as libc::c_int) * 128 as libc::c_int) as isize,
                            ) as libc::c_int
                        + *fv.offset(1) as libc::c_int
                            * *mid_ptr
                                .offset(
                                    (x_0 + -(2 as libc::c_int) * 128 as libc::c_int) as isize,
                                ) as libc::c_int
                        + *fv.offset(2) as libc::c_int
                            * *mid_ptr
                                .offset(
                                    (x_0 + -(1 as libc::c_int) * 128 as libc::c_int) as isize,
                                ) as libc::c_int
                        + *fv.offset(3) as libc::c_int
                            * *mid_ptr
                                .offset(
                                    (x_0 + 0 as libc::c_int * 128 as libc::c_int) as isize,
                                ) as libc::c_int
                        + *fv.offset(4) as libc::c_int
                            * *mid_ptr
                                .offset(
                                    (x_0 + 1 as libc::c_int * 128 as libc::c_int) as isize,
                                ) as libc::c_int
                        + *fv.offset(5) as libc::c_int
                            * *mid_ptr
                                .offset(
                                    (x_0 + 2 as libc::c_int * 128 as libc::c_int) as isize,
                                ) as libc::c_int
                        + *fv.offset(6) as libc::c_int
                            * *mid_ptr
                                .offset(
                                    (x_0 + 3 as libc::c_int * 128 as libc::c_int) as isize,
                                ) as libc::c_int
                        + *fv.offset(7) as libc::c_int
                            * *mid_ptr
                                .offset(
                                    (x_0 + 4 as libc::c_int * 128 as libc::c_int) as isize,
                                ) as libc::c_int
                        + ((1 as libc::c_int) << 6 as libc::c_int + intermediate_bits
                            >> 1 as libc::c_int) >> 6 as libc::c_int + intermediate_bits,
                    0 as libc::c_int,
                    bitdepth_max,
                )
            } else {
                iclip(
                    *mid_ptr.offset(x_0 as isize) as libc::c_int + intermediate_rnd
                        >> intermediate_bits,
                    0 as libc::c_int,
                    bitdepth_max,
                )
            }) as pixel;
            x_0 += 1;
        }
        my += dy;
        mid_ptr = mid_ptr
            .offset(((my >> 10 as libc::c_int) * 128 as libc::c_int) as isize);
        my &= 0x3ff as libc::c_int;
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        y += 1;
    }
}
#[inline(never)]
unsafe extern "C" fn prep_8tap_c(
    mut tmp: *mut int16_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: libc::c_int,
    mut h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    filter_type: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14 as libc::c_int
        - (32 as libc::c_int - clz(bitdepth_max as libc::c_uint));
    let fh: *const int8_t = if mx == 0 {
        0 as *const int8_t
    } else if w > 4 as libc::c_int {
        (dav1d_mc_subpel_filters[(filter_type & 3 as libc::c_int)
            as usize][(mx - 1 as libc::c_int) as usize])
            .as_ptr()
    } else {
        (dav1d_mc_subpel_filters[(3 as libc::c_int + (filter_type & 1 as libc::c_int))
            as usize][(mx - 1 as libc::c_int) as usize])
            .as_ptr()
    };
    let fv: *const int8_t = if my == 0 {
        0 as *const int8_t
    } else if h > 4 as libc::c_int {
        (dav1d_mc_subpel_filters[(filter_type >> 2 as libc::c_int)
            as usize][(my - 1 as libc::c_int) as usize])
            .as_ptr()
    } else {
        (dav1d_mc_subpel_filters[(3 as libc::c_int
            + (filter_type >> 2 as libc::c_int & 1 as libc::c_int))
            as usize][(my - 1 as libc::c_int) as usize])
            .as_ptr()
    };
    src_stride = PXSTRIDE(src_stride);
    if !fh.is_null() {
        if !fv.is_null() {
            let mut tmp_h: libc::c_int = h + 7 as libc::c_int;
            let mut mid: [int16_t; 17280] = [0; 17280];
            let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
            src = src
                .offset(-((src_stride * 3) as isize));
            loop {
                let mut x = 0;
                while x < w {
                    *mid_ptr
                        .offset(
                            x as isize,
                        ) = (*fh.offset(0) as libc::c_int
                        * *src
                            .offset(
                                (x + -(3 as libc::c_int) * 1 as libc::c_int) as isize,
                            ) as libc::c_int
                        + *fh.offset(1) as libc::c_int
                            * *src
                                .offset(
                                    (x + -(2 as libc::c_int) * 1 as libc::c_int) as isize,
                                ) as libc::c_int
                        + *fh.offset(2) as libc::c_int
                            * *src
                                .offset(
                                    (x + -(1 as libc::c_int) * 1 as libc::c_int) as isize,
                                ) as libc::c_int
                        + *fh.offset(3) as libc::c_int
                            * *src
                                .offset((x + 0 as libc::c_int * 1 as libc::c_int) as isize)
                                as libc::c_int
                        + *fh.offset(4) as libc::c_int
                            * *src
                                .offset((x + 1 as libc::c_int * 1 as libc::c_int) as isize)
                                as libc::c_int
                        + *fh.offset(5) as libc::c_int
                            * *src
                                .offset((x + 2 as libc::c_int * 1 as libc::c_int) as isize)
                                as libc::c_int
                        + *fh.offset(6) as libc::c_int
                            * *src
                                .offset((x + 3 as libc::c_int * 1 as libc::c_int) as isize)
                                as libc::c_int
                        + *fh.offset(7) as libc::c_int
                            * *src
                                .offset((x + 4 as libc::c_int * 1 as libc::c_int) as isize)
                                as libc::c_int
                        + ((1 as libc::c_int) << 6 as libc::c_int - intermediate_bits
                            >> 1 as libc::c_int) >> 6 as libc::c_int - intermediate_bits)
                        as int16_t;
                    x += 1;
                }
                mid_ptr = mid_ptr.offset(128);
                src = src.offset(src_stride as isize);
                tmp_h -= 1;
                if !(tmp_h != 0) {
                    break;
                }
            }
            mid_ptr = mid
                .as_mut_ptr()
                .offset((128 as libc::c_int * 3 as libc::c_int) as isize);
            loop {
                let mut x_0 = 0;
                while x_0 < w {
                    let mut t: libc::c_int = (*fv.offset(0)
                        as libc::c_int
                        * *mid_ptr
                            .offset(
                                (x_0 + -(3 as libc::c_int) * 128 as libc::c_int) as isize,
                            ) as libc::c_int
                        + *fv.offset(1) as libc::c_int
                            * *mid_ptr
                                .offset(
                                    (x_0 + -(2 as libc::c_int) * 128 as libc::c_int) as isize,
                                ) as libc::c_int
                        + *fv.offset(2) as libc::c_int
                            * *mid_ptr
                                .offset(
                                    (x_0 + -(1 as libc::c_int) * 128 as libc::c_int) as isize,
                                ) as libc::c_int
                        + *fv.offset(3) as libc::c_int
                            * *mid_ptr
                                .offset(
                                    (x_0 + 0 as libc::c_int * 128 as libc::c_int) as isize,
                                ) as libc::c_int
                        + *fv.offset(4) as libc::c_int
                            * *mid_ptr
                                .offset(
                                    (x_0 + 1 as libc::c_int * 128 as libc::c_int) as isize,
                                ) as libc::c_int
                        + *fv.offset(5) as libc::c_int
                            * *mid_ptr
                                .offset(
                                    (x_0 + 2 as libc::c_int * 128 as libc::c_int) as isize,
                                ) as libc::c_int
                        + *fv.offset(6) as libc::c_int
                            * *mid_ptr
                                .offset(
                                    (x_0 + 3 as libc::c_int * 128 as libc::c_int) as isize,
                                ) as libc::c_int
                        + *fv.offset(7) as libc::c_int
                            * *mid_ptr
                                .offset(
                                    (x_0 + 4 as libc::c_int * 128 as libc::c_int) as isize,
                                ) as libc::c_int
                        + ((1 as libc::c_int) << 6 as libc::c_int >> 1 as libc::c_int)
                        >> 6 as libc::c_int) - 8192 as libc::c_int;
                    if !(t >= -(32767 as libc::c_int) - 1 as libc::c_int
                        && t <= 32767 as libc::c_int)
                    {
                        unreachable!();
                    }
                    *tmp.offset(x_0 as isize) = t as int16_t;
                    x_0 += 1;
                }
                mid_ptr = mid_ptr.offset(128);
                tmp = tmp.offset(w as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        } else {
            loop {
                let mut x_1 = 0;
                while x_1 < w {
                    *tmp
                        .offset(
                            x_1 as isize,
                        ) = ((*fh.offset(0) as libc::c_int
                        * *src
                            .offset(
                                (x_1 + -(3 as libc::c_int) * 1 as libc::c_int) as isize,
                            ) as libc::c_int
                        + *fh.offset(1) as libc::c_int
                            * *src
                                .offset(
                                    (x_1 + -(2 as libc::c_int) * 1 as libc::c_int) as isize,
                                ) as libc::c_int
                        + *fh.offset(2) as libc::c_int
                            * *src
                                .offset(
                                    (x_1 + -(1 as libc::c_int) * 1 as libc::c_int) as isize,
                                ) as libc::c_int
                        + *fh.offset(3) as libc::c_int
                            * *src
                                .offset(
                                    (x_1 + 0 as libc::c_int * 1 as libc::c_int) as isize,
                                ) as libc::c_int
                        + *fh.offset(4) as libc::c_int
                            * *src
                                .offset(
                                    (x_1 + 1 as libc::c_int * 1 as libc::c_int) as isize,
                                ) as libc::c_int
                        + *fh.offset(5) as libc::c_int
                            * *src
                                .offset(
                                    (x_1 + 2 as libc::c_int * 1 as libc::c_int) as isize,
                                ) as libc::c_int
                        + *fh.offset(6) as libc::c_int
                            * *src
                                .offset(
                                    (x_1 + 3 as libc::c_int * 1 as libc::c_int) as isize,
                                ) as libc::c_int
                        + *fh.offset(7) as libc::c_int
                            * *src
                                .offset(
                                    (x_1 + 4 as libc::c_int * 1 as libc::c_int) as isize,
                                ) as libc::c_int
                        + ((1 as libc::c_int) << 6 as libc::c_int - intermediate_bits
                            >> 1 as libc::c_int) >> 6 as libc::c_int - intermediate_bits)
                        - 8192 as libc::c_int) as int16_t;
                    x_1 += 1;
                }
                tmp = tmp.offset(w as isize);
                src = src.offset(src_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        }
    } else if !fv.is_null() {
        loop {
            let mut x_2 = 0;
            while x_2 < w {
                *tmp
                    .offset(
                        x_2 as isize,
                    ) = ((*fv.offset(0) as libc::c_int
                    * *src
                        .offset(
                            (x_2 as isize
                                + -(3 as libc::c_int) as isize * src_stride) as isize,
                        ) as libc::c_int
                    + *fv.offset(1) as libc::c_int
                        * *src
                            .offset(
                                (x_2 as isize
                                    + -(2 as libc::c_int) as isize * src_stride) as isize,
                            ) as libc::c_int
                    + *fv.offset(2) as libc::c_int
                        * *src
                            .offset(
                                (x_2 as isize
                                    + -(1 as libc::c_int) as isize * src_stride) as isize,
                            ) as libc::c_int
                    + *fv.offset(3) as libc::c_int
                        * *src
                            .offset(
                                (x_2 as isize
                                    + 0 * src_stride) as isize,
                            ) as libc::c_int
                    + *fv.offset(4) as libc::c_int
                        * *src
                            .offset(
                                (x_2 as isize
                                    + 1 * src_stride) as isize,
                            ) as libc::c_int
                    + *fv.offset(5) as libc::c_int
                        * *src
                            .offset(
                                (x_2 as isize
                                    + 2 * src_stride) as isize,
                            ) as libc::c_int
                    + *fv.offset(6) as libc::c_int
                        * *src
                            .offset(
                                (x_2 as isize
                                    + 3 * src_stride) as isize,
                            ) as libc::c_int
                    + *fv.offset(7) as libc::c_int
                        * *src
                            .offset(
                                (x_2 as isize
                                    + 4 * src_stride) as isize,
                            ) as libc::c_int
                    + ((1 as libc::c_int) << 6 as libc::c_int - intermediate_bits
                        >> 1 as libc::c_int) >> 6 as libc::c_int - intermediate_bits)
                    - 8192 as libc::c_int) as int16_t;
                x_2 += 1;
            }
            tmp = tmp.offset(w as isize);
            src = src.offset(src_stride as isize);
            h -= 1;
            if !(h != 0) {
                break;
            }
        }
    } else {
        prep_c(tmp, src, src_stride, w, h, bitdepth_max);
    };
}
#[inline(never)]
unsafe extern "C" fn prep_8tap_scaled_c(
    mut tmp: *mut int16_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: libc::c_int,
    mut h: libc::c_int,
    mx: libc::c_int,
    mut my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    filter_type: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14 as libc::c_int
        - (32 as libc::c_int - clz(bitdepth_max as libc::c_uint));
    let mut tmp_h: libc::c_int = ((h - 1 as libc::c_int) * dy + my >> 10 as libc::c_int)
        + 8 as libc::c_int;
    let mut mid: [int16_t; 33664] = [0; 33664];
    let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
    src_stride = PXSTRIDE(src_stride);
    src = src.offset(-((src_stride * 3) as isize));
    loop {
        let mut x: libc::c_int = 0;
        let mut imx: libc::c_int = mx;
        let mut ioff = 0;
        x = 0 as libc::c_int;
        while x < w {
            let fh: *const int8_t = if imx >> 6 as libc::c_int == 0 {
                0 as *const int8_t
            } else if w > 4 as libc::c_int {
                (dav1d_mc_subpel_filters[(filter_type & 3 as libc::c_int)
                    as usize][((imx >> 6 as libc::c_int) - 1 as libc::c_int) as usize])
                    .as_ptr()
            } else {
                (dav1d_mc_subpel_filters[(3 as libc::c_int
                    + (filter_type & 1 as libc::c_int))
                    as usize][((imx >> 6 as libc::c_int) - 1 as libc::c_int) as usize])
                    .as_ptr()
            };
            *mid_ptr
                .offset(
                    x as isize,
                ) = (if !fh.is_null() {
                *fh.offset(0) as libc::c_int
                    * *src
                        .offset((ioff + -(3 as libc::c_int) * 1 as libc::c_int) as isize)
                        as libc::c_int
                    + *fh.offset(1) as libc::c_int
                        * *src
                            .offset(
                                (ioff + -(2 as libc::c_int) * 1 as libc::c_int) as isize,
                            ) as libc::c_int
                    + *fh.offset(2) as libc::c_int
                        * *src
                            .offset(
                                (ioff + -(1 as libc::c_int) * 1 as libc::c_int) as isize,
                            ) as libc::c_int
                    + *fh.offset(3) as libc::c_int
                        * *src
                            .offset(
                                (ioff + 0 as libc::c_int * 1 as libc::c_int) as isize,
                            ) as libc::c_int
                    + *fh.offset(4) as libc::c_int
                        * *src
                            .offset(
                                (ioff + 1 as libc::c_int * 1 as libc::c_int) as isize,
                            ) as libc::c_int
                    + *fh.offset(5) as libc::c_int
                        * *src
                            .offset(
                                (ioff + 2 as libc::c_int * 1 as libc::c_int) as isize,
                            ) as libc::c_int
                    + *fh.offset(6) as libc::c_int
                        * *src
                            .offset(
                                (ioff + 3 as libc::c_int * 1 as libc::c_int) as isize,
                            ) as libc::c_int
                    + *fh.offset(7) as libc::c_int
                        * *src
                            .offset(
                                (ioff + 4 as libc::c_int * 1 as libc::c_int) as isize,
                            ) as libc::c_int
                    + ((1 as libc::c_int) << 6 as libc::c_int - intermediate_bits
                        >> 1 as libc::c_int) >> 6 as libc::c_int - intermediate_bits
            } else {
                (*src.offset(ioff as isize) as libc::c_int) << intermediate_bits
            }) as int16_t;
            imx += dx;
            ioff += imx >> 10 as libc::c_int;
            imx &= 0x3ff as libc::c_int;
            x += 1;
        }
        mid_ptr = mid_ptr.offset(128);
        src = src.offset(src_stride as isize);
        tmp_h -= 1;
        if !(tmp_h != 0) {
            break;
        }
    }
    mid_ptr = mid.as_mut_ptr().offset((128 as libc::c_int * 3 as libc::c_int) as isize);
    let mut y = 0;
    while y < h {
        let mut x_0: libc::c_int = 0;
        let fv: *const int8_t = if my >> 6 as libc::c_int == 0 {
            0 as *const int8_t
        } else if h > 4 as libc::c_int {
            (dav1d_mc_subpel_filters[(filter_type >> 2 as libc::c_int)
                as usize][((my >> 6 as libc::c_int) - 1 as libc::c_int) as usize])
                .as_ptr()
        } else {
            (dav1d_mc_subpel_filters[(3 as libc::c_int
                + (filter_type >> 2 as libc::c_int & 1 as libc::c_int))
                as usize][((my >> 6 as libc::c_int) - 1 as libc::c_int) as usize])
                .as_ptr()
        };
        x_0 = 0 as libc::c_int;
        while x_0 < w {
            *tmp
                .offset(
                    x_0 as isize,
                ) = ((if !fv.is_null() {
                *fv.offset(0) as libc::c_int
                    * *mid_ptr
                        .offset(
                            (x_0 + -(3 as libc::c_int) * 128 as libc::c_int) as isize,
                        ) as libc::c_int
                    + *fv.offset(1) as libc::c_int
                        * *mid_ptr
                            .offset(
                                (x_0 + -(2 as libc::c_int) * 128 as libc::c_int) as isize,
                            ) as libc::c_int
                    + *fv.offset(2) as libc::c_int
                        * *mid_ptr
                            .offset(
                                (x_0 + -(1 as libc::c_int) * 128 as libc::c_int) as isize,
                            ) as libc::c_int
                    + *fv.offset(3) as libc::c_int
                        * *mid_ptr
                            .offset(
                                (x_0 + 0 as libc::c_int * 128 as libc::c_int) as isize,
                            ) as libc::c_int
                    + *fv.offset(4) as libc::c_int
                        * *mid_ptr
                            .offset(
                                (x_0 + 1 as libc::c_int * 128 as libc::c_int) as isize,
                            ) as libc::c_int
                    + *fv.offset(5) as libc::c_int
                        * *mid_ptr
                            .offset(
                                (x_0 + 2 as libc::c_int * 128 as libc::c_int) as isize,
                            ) as libc::c_int
                    + *fv.offset(6) as libc::c_int
                        * *mid_ptr
                            .offset(
                                (x_0 + 3 as libc::c_int * 128 as libc::c_int) as isize,
                            ) as libc::c_int
                    + *fv.offset(7) as libc::c_int
                        * *mid_ptr
                            .offset(
                                (x_0 + 4 as libc::c_int * 128 as libc::c_int) as isize,
                            ) as libc::c_int
                    + ((1 as libc::c_int) << 6 as libc::c_int >> 1 as libc::c_int)
                    >> 6 as libc::c_int
            } else {
                *mid_ptr.offset(x_0 as isize) as libc::c_int
            }) - 8192 as libc::c_int) as int16_t;
            x_0 += 1;
        }
        my += dy;
        mid_ptr = mid_ptr
            .offset(((my >> 10 as libc::c_int) * 128 as libc::c_int) as isize);
        my &= 0x3ff as libc::c_int;
        tmp = tmp.offset(w as isize);
        y += 1;
    }
}
unsafe extern "C" fn put_8tap_regular_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_REGULAR as libc::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_regular_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_REGULAR as libc::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_regular_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_REGULAR as libc::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_regular_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_REGULAR as libc::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_regular_sharp_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_REGULAR as libc::c_int
            | (DAV1D_FILTER_8TAP_SHARP as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_regular_sharp_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_REGULAR as libc::c_int
            | (DAV1D_FILTER_8TAP_SHARP as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_regular_sharp_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_REGULAR as libc::c_int
            | (DAV1D_FILTER_8TAP_SHARP as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_regular_sharp_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_REGULAR as libc::c_int
            | (DAV1D_FILTER_8TAP_SHARP as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_regular_smooth_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_REGULAR as libc::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_regular_smooth_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_REGULAR as libc::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_regular_smooth_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_REGULAR as libc::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_regular_smooth_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_REGULAR as libc::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_smooth_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SMOOTH as libc::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_smooth_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SMOOTH as libc::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_smooth_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SMOOTH as libc::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_smooth_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SMOOTH as libc::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_smooth_regular_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SMOOTH as libc::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_smooth_regular_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SMOOTH as libc::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_smooth_regular_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SMOOTH as libc::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_smooth_regular_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SMOOTH as libc::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_smooth_sharp_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SMOOTH as libc::c_int
            | (DAV1D_FILTER_8TAP_SHARP as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_smooth_sharp_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SMOOTH as libc::c_int
            | (DAV1D_FILTER_8TAP_SHARP as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_smooth_sharp_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SMOOTH as libc::c_int
            | (DAV1D_FILTER_8TAP_SHARP as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_smooth_sharp_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SMOOTH as libc::c_int
            | (DAV1D_FILTER_8TAP_SHARP as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_sharp_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SHARP as libc::c_int
            | (DAV1D_FILTER_8TAP_SHARP as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_sharp_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SHARP as libc::c_int
            | (DAV1D_FILTER_8TAP_SHARP as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_sharp_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SHARP as libc::c_int
            | (DAV1D_FILTER_8TAP_SHARP as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_sharp_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SHARP as libc::c_int
            | (DAV1D_FILTER_8TAP_SHARP as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_sharp_regular_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SHARP as libc::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_sharp_regular_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SHARP as libc::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_sharp_regular_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SHARP as libc::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_sharp_regular_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SHARP as libc::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_sharp_smooth_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SHARP as libc::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_sharp_smooth_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SHARP as libc::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_sharp_smooth_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SHARP as libc::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_sharp_smooth_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SHARP as libc::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as libc::c_int) << 2 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_bilin_c(
    mut dst: *mut pixel,
    mut dst_stride: ptrdiff_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: libc::c_int,
    mut h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14 as libc::c_int
        - (32 as libc::c_int - clz(bitdepth_max as libc::c_uint));
    let intermediate_rnd: libc::c_int = (1 as libc::c_int) << intermediate_bits
        >> 1 as libc::c_int;
    dst_stride = PXSTRIDE(dst_stride);
    src_stride = PXSTRIDE(src_stride);
    if mx != 0 {
        if my != 0 {
            let mut mid: [int16_t; 16512] = [0; 16512];
            let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
            let mut tmp_h: libc::c_int = h + 1 as libc::c_int;
            loop {
                let mut x = 0;
                while x < w {
                    *mid_ptr
                        .offset(
                            x as isize,
                        ) = (16 as libc::c_int * *src.offset(x as isize) as libc::c_int
                        + mx
                            * (*src.offset((x + 1 as libc::c_int) as isize)
                                as libc::c_int - *src.offset(x as isize) as libc::c_int)
                        + ((1 as libc::c_int) << 4 as libc::c_int - intermediate_bits
                            >> 1 as libc::c_int) >> 4 as libc::c_int - intermediate_bits)
                        as int16_t;
                    x += 1;
                }
                mid_ptr = mid_ptr.offset(128);
                src = src.offset(src_stride as isize);
                tmp_h -= 1;
                if !(tmp_h != 0) {
                    break;
                }
            }
            mid_ptr = mid.as_mut_ptr();
            loop {
                let mut x_0 = 0;
                while x_0 < w {
                    *dst
                        .offset(
                            x_0 as isize,
                        ) = iclip(
                        16 as libc::c_int * *mid_ptr.offset(x_0 as isize) as libc::c_int
                            + my
                                * (*mid_ptr.offset((x_0 + 128 as libc::c_int) as isize)
                                    as libc::c_int
                                    - *mid_ptr.offset(x_0 as isize) as libc::c_int)
                            + ((1 as libc::c_int) << 4 as libc::c_int + intermediate_bits
                                >> 1 as libc::c_int)
                            >> 4 as libc::c_int + intermediate_bits,
                        0 as libc::c_int,
                        bitdepth_max,
                    ) as pixel;
                    x_0 += 1;
                }
                mid_ptr = mid_ptr.offset(128);
                dst = dst.offset(dst_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        } else {
            loop {
                let mut x_1 = 0;
                while x_1 < w {
                    let px: libc::c_int = 16 as libc::c_int
                        * *src.offset(x_1 as isize) as libc::c_int
                        + mx
                            * (*src.offset((x_1 + 1 as libc::c_int) as isize)
                                as libc::c_int - *src.offset(x_1 as isize) as libc::c_int)
                        + ((1 as libc::c_int) << 4 as libc::c_int - intermediate_bits
                            >> 1 as libc::c_int) >> 4 as libc::c_int - intermediate_bits;
                    *dst
                        .offset(
                            x_1 as isize,
                        ) = iclip(
                        px + intermediate_rnd >> intermediate_bits,
                        0 as libc::c_int,
                        bitdepth_max,
                    ) as pixel;
                    x_1 += 1;
                }
                dst = dst.offset(dst_stride as isize);
                src = src.offset(src_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        }
    } else if my != 0 {
        loop {
            let mut x_2 = 0;
            while x_2 < w {
                *dst
                    .offset(
                        x_2 as isize,
                    ) = iclip(
                    16 as libc::c_int * *src.offset(x_2 as isize) as libc::c_int
                        + my
                            * (*src.offset((x_2 as isize + src_stride) as isize)
                                as libc::c_int - *src.offset(x_2 as isize) as libc::c_int)
                        + ((1 as libc::c_int) << 4 as libc::c_int >> 1 as libc::c_int)
                        >> 4 as libc::c_int,
                    0 as libc::c_int,
                    bitdepth_max,
                ) as pixel;
                x_2 += 1;
            }
            dst = dst.offset(dst_stride as isize);
            src = src.offset(src_stride as isize);
            h -= 1;
            if !(h != 0) {
                break;
            }
        }
    } else {
        put_c(dst, dst_stride, src, src_stride, w, h);
    };
}
unsafe extern "C" fn put_bilin_scaled_c(
    mut dst: *mut pixel,
    mut dst_stride: ptrdiff_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: libc::c_int,
    mut h: libc::c_int,
    mx: libc::c_int,
    mut my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14 as libc::c_int
        - (32 as libc::c_int - clz(bitdepth_max as libc::c_uint));
    let mut tmp_h: libc::c_int = ((h - 1 as libc::c_int) * dy + my >> 10 as libc::c_int)
        + 2 as libc::c_int;
    let mut mid: [int16_t; 32896] = [0; 32896];
    let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
    loop {
        let mut x: libc::c_int = 0;
        let mut imx: libc::c_int = mx;
        let mut ioff = 0;
        x = 0 as libc::c_int;
        while x < w {
            *mid_ptr
                .offset(
                    x as isize,
                ) = (16 as libc::c_int * *src.offset(ioff as isize) as libc::c_int
                + (imx >> 6 as libc::c_int)
                    * (*src.offset((ioff + 1 as libc::c_int) as isize) as libc::c_int
                        - *src.offset(ioff as isize) as libc::c_int)
                + ((1 as libc::c_int) << 4 as libc::c_int - intermediate_bits
                    >> 1 as libc::c_int) >> 4 as libc::c_int - intermediate_bits)
                as int16_t;
            imx += dx;
            ioff += imx >> 10 as libc::c_int;
            imx &= 0x3ff as libc::c_int;
            x += 1;
        }
        mid_ptr = mid_ptr.offset(128);
        src = src.offset(PXSTRIDE(src_stride) as isize);
        tmp_h -= 1;
        if !(tmp_h != 0) {
            break;
        }
    }
    mid_ptr = mid.as_mut_ptr();
    loop {
        let mut x_0: libc::c_int = 0;
        x_0 = 0 as libc::c_int;
        while x_0 < w {
            *dst
                .offset(
                    x_0 as isize,
                ) = iclip(
                16 as libc::c_int * *mid_ptr.offset(x_0 as isize) as libc::c_int
                    + (my >> 6 as libc::c_int)
                        * (*mid_ptr.offset((x_0 + 128 as libc::c_int) as isize)
                            as libc::c_int
                            - *mid_ptr.offset(x_0 as isize) as libc::c_int)
                    + ((1 as libc::c_int) << 4 as libc::c_int + intermediate_bits
                        >> 1 as libc::c_int) >> 4 as libc::c_int + intermediate_bits,
                0 as libc::c_int,
                bitdepth_max,
            ) as pixel;
            x_0 += 1;
        }
        my += dy;
        mid_ptr = mid_ptr
            .offset(((my >> 10 as libc::c_int) * 128 as libc::c_int) as isize);
        my &= 0x3ff as libc::c_int;
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
unsafe extern "C" fn prep_bilin_c(
    mut tmp: *mut int16_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: libc::c_int,
    mut h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14 as libc::c_int
        - (32 as libc::c_int - clz(bitdepth_max as libc::c_uint));
    src_stride = PXSTRIDE(src_stride);
    if mx != 0 {
        if my != 0 {
            let mut mid: [int16_t; 16512] = [0; 16512];
            let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
            let mut tmp_h: libc::c_int = h + 1 as libc::c_int;
            loop {
                let mut x = 0;
                while x < w {
                    *mid_ptr
                        .offset(
                            x as isize,
                        ) = (16 as libc::c_int * *src.offset(x as isize) as libc::c_int
                        + mx
                            * (*src.offset((x + 1 as libc::c_int) as isize)
                                as libc::c_int - *src.offset(x as isize) as libc::c_int)
                        + ((1 as libc::c_int) << 4 as libc::c_int - intermediate_bits
                            >> 1 as libc::c_int) >> 4 as libc::c_int - intermediate_bits)
                        as int16_t;
                    x += 1;
                }
                mid_ptr = mid_ptr.offset(128);
                src = src.offset(src_stride as isize);
                tmp_h -= 1;
                if !(tmp_h != 0) {
                    break;
                }
            }
            mid_ptr = mid.as_mut_ptr();
            loop {
                let mut x_0 = 0;
                while x_0 < w {
                    *tmp
                        .offset(
                            x_0 as isize,
                        ) = ((16 as libc::c_int
                        * *mid_ptr.offset(x_0 as isize) as libc::c_int
                        + my
                            * (*mid_ptr.offset((x_0 + 128 as libc::c_int) as isize)
                                as libc::c_int
                                - *mid_ptr.offset(x_0 as isize) as libc::c_int)
                        + ((1 as libc::c_int) << 4 as libc::c_int >> 1 as libc::c_int)
                        >> 4 as libc::c_int) - 8192 as libc::c_int) as int16_t;
                    x_0 += 1;
                }
                mid_ptr = mid_ptr.offset(128);
                tmp = tmp.offset(w as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        } else {
            loop {
                let mut x_1 = 0;
                while x_1 < w {
                    *tmp
                        .offset(
                            x_1 as isize,
                        ) = ((16 as libc::c_int
                        * *src.offset(x_1 as isize) as libc::c_int
                        + mx
                            * (*src.offset((x_1 + 1 as libc::c_int) as isize)
                                as libc::c_int - *src.offset(x_1 as isize) as libc::c_int)
                        + ((1 as libc::c_int) << 4 as libc::c_int - intermediate_bits
                            >> 1 as libc::c_int) >> 4 as libc::c_int - intermediate_bits)
                        - 8192 as libc::c_int) as int16_t;
                    x_1 += 1;
                }
                tmp = tmp.offset(w as isize);
                src = src.offset(src_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        }
    } else if my != 0 {
        loop {
            let mut x_2 = 0;
            while x_2 < w {
                *tmp
                    .offset(
                        x_2 as isize,
                    ) = ((16 as libc::c_int * *src.offset(x_2 as isize) as libc::c_int
                    + my
                        * (*src.offset((x_2 as isize + src_stride) as isize)
                            as libc::c_int - *src.offset(x_2 as isize) as libc::c_int)
                    + ((1 as libc::c_int) << 4 as libc::c_int - intermediate_bits
                        >> 1 as libc::c_int) >> 4 as libc::c_int - intermediate_bits)
                    - 8192 as libc::c_int) as int16_t;
                x_2 += 1;
            }
            tmp = tmp.offset(w as isize);
            src = src.offset(src_stride as isize);
            h -= 1;
            if !(h != 0) {
                break;
            }
        }
    } else {
        prep_c(tmp, src, src_stride, w, h, bitdepth_max);
    };
}
unsafe extern "C" fn prep_bilin_scaled_c(
    mut tmp: *mut int16_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: libc::c_int,
    mut h: libc::c_int,
    mx: libc::c_int,
    mut my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14 as libc::c_int
        - (32 as libc::c_int - clz(bitdepth_max as libc::c_uint));
    let mut tmp_h: libc::c_int = ((h - 1 as libc::c_int) * dy + my >> 10 as libc::c_int)
        + 2 as libc::c_int;
    let mut mid: [int16_t; 32896] = [0; 32896];
    let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
    loop {
        let mut x: libc::c_int = 0;
        let mut imx: libc::c_int = mx;
        let mut ioff = 0;
        x = 0 as libc::c_int;
        while x < w {
            *mid_ptr
                .offset(
                    x as isize,
                ) = (16 as libc::c_int * *src.offset(ioff as isize) as libc::c_int
                + (imx >> 6 as libc::c_int)
                    * (*src.offset((ioff + 1 as libc::c_int) as isize) as libc::c_int
                        - *src.offset(ioff as isize) as libc::c_int)
                + ((1 as libc::c_int) << 4 as libc::c_int - intermediate_bits
                    >> 1 as libc::c_int) >> 4 as libc::c_int - intermediate_bits)
                as int16_t;
            imx += dx;
            ioff += imx >> 10 as libc::c_int;
            imx &= 0x3ff as libc::c_int;
            x += 1;
        }
        mid_ptr = mid_ptr.offset(128);
        src = src.offset(PXSTRIDE(src_stride) as isize);
        tmp_h -= 1;
        if !(tmp_h != 0) {
            break;
        }
    }
    mid_ptr = mid.as_mut_ptr();
    loop {
        let mut x_0: libc::c_int = 0;
        x_0 = 0 as libc::c_int;
        while x_0 < w {
            *tmp
                .offset(
                    x_0 as isize,
                ) = ((16 as libc::c_int * *mid_ptr.offset(x_0 as isize) as libc::c_int
                + (my >> 6 as libc::c_int)
                    * (*mid_ptr.offset((x_0 + 128 as libc::c_int) as isize)
                        as libc::c_int - *mid_ptr.offset(x_0 as isize) as libc::c_int)
                + ((1 as libc::c_int) << 4 as libc::c_int >> 1 as libc::c_int)
                >> 4 as libc::c_int) - 8192 as libc::c_int) as int16_t;
            x_0 += 1;
        }
        my += dy;
        mid_ptr = mid_ptr
            .offset(((my >> 10 as libc::c_int) * 128 as libc::c_int) as isize);
        my &= 0x3ff as libc::c_int;
        tmp = tmp.offset(w as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
unsafe extern "C" fn avg_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut tmp1: *const int16_t,
    mut tmp2: *const int16_t,
    w: libc::c_int,
    mut h: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14 as libc::c_int
        - (32 as libc::c_int - clz(bitdepth_max as libc::c_uint));
    let sh: libc::c_int = intermediate_bits + 1 as libc::c_int;
    let rnd: libc::c_int = ((1 as libc::c_int) << intermediate_bits)
        + 8192 as libc::c_int * 2 as libc::c_int;
    loop {
        let mut x = 0;
        while x < w {
            *dst
                .offset(
                    x as isize,
                ) = iclip(
                *tmp1.offset(x as isize) as libc::c_int
                    + *tmp2.offset(x as isize) as libc::c_int + rnd >> sh,
                0 as libc::c_int,
                bitdepth_max,
            ) as pixel;
            x += 1;
        }
        tmp1 = tmp1.offset(w as isize);
        tmp2 = tmp2.offset(w as isize);
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
unsafe extern "C" fn w_avg_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut tmp1: *const int16_t,
    mut tmp2: *const int16_t,
    w: libc::c_int,
    mut h: libc::c_int,
    weight: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14 as libc::c_int
        - (32 as libc::c_int - clz(bitdepth_max as libc::c_uint));
    let sh: libc::c_int = intermediate_bits + 4 as libc::c_int;
    let rnd: libc::c_int = ((8 as libc::c_int) << intermediate_bits)
        + 8192 as libc::c_int * 16 as libc::c_int;
    loop {
        let mut x = 0;
        while x < w {
            *dst
                .offset(
                    x as isize,
                ) = iclip(
                *tmp1.offset(x as isize) as libc::c_int * weight
                    + *tmp2.offset(x as isize) as libc::c_int
                        * (16 as libc::c_int - weight) + rnd >> sh,
                0 as libc::c_int,
                bitdepth_max,
            ) as pixel;
            x += 1;
        }
        tmp1 = tmp1.offset(w as isize);
        tmp2 = tmp2.offset(w as isize);
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
unsafe extern "C" fn mask_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut tmp1: *const int16_t,
    mut tmp2: *const int16_t,
    w: libc::c_int,
    mut h: libc::c_int,
    mut mask: *const uint8_t,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14 as libc::c_int
        - (32 as libc::c_int - clz(bitdepth_max as libc::c_uint));
    let sh: libc::c_int = intermediate_bits + 6 as libc::c_int;
    let rnd: libc::c_int = ((32 as libc::c_int) << intermediate_bits)
        + 8192 as libc::c_int * 64 as libc::c_int;
    loop {
        let mut x = 0;
        while x < w {
            *dst
                .offset(
                    x as isize,
                ) = iclip(
                *tmp1.offset(x as isize) as libc::c_int
                    * *mask.offset(x as isize) as libc::c_int
                    + *tmp2.offset(x as isize) as libc::c_int
                        * (64 as libc::c_int - *mask.offset(x as isize) as libc::c_int)
                    + rnd >> sh,
                0 as libc::c_int,
                bitdepth_max,
            ) as pixel;
            x += 1;
        }
        tmp1 = tmp1.offset(w as isize);
        tmp2 = tmp2.offset(w as isize);
        mask = mask.offset(w as isize);
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
unsafe extern "C" fn blend_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut tmp: *const pixel,
    w: libc::c_int,
    mut h: libc::c_int,
    mut mask: *const uint8_t,
) {
    loop {
        let mut x = 0;
        while x < w {
            *dst
                .offset(
                    x as isize,
                ) = (*dst.offset(x as isize) as libc::c_int
                * (64 as libc::c_int - *mask.offset(x as isize) as libc::c_int)
                + *tmp.offset(x as isize) as libc::c_int
                    * *mask.offset(x as isize) as libc::c_int + 32 as libc::c_int
                >> 6 as libc::c_int) as pixel;
            x += 1;
        }
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        tmp = tmp.offset(w as isize);
        mask = mask.offset(w as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
unsafe extern "C" fn blend_v_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut tmp: *const pixel,
    w: libc::c_int,
    mut h: libc::c_int,
) {
    let mask: *const uint8_t = &*dav1d_obmc_masks.as_ptr().offset(w as isize)
        as *const uint8_t;
    loop {
        let mut x = 0;
        while x < w * 3 as libc::c_int >> 2 as libc::c_int {
            *dst
                .offset(
                    x as isize,
                ) = (*dst.offset(x as isize) as libc::c_int
                * (64 as libc::c_int - *mask.offset(x as isize) as libc::c_int)
                + *tmp.offset(x as isize) as libc::c_int
                    * *mask.offset(x as isize) as libc::c_int + 32 as libc::c_int
                >> 6 as libc::c_int) as pixel;
            x += 1;
        }
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        tmp = tmp.offset(w as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
unsafe extern "C" fn blend_h_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut tmp: *const pixel,
    w: libc::c_int,
    mut h: libc::c_int,
) {
    let mut mask: *const uint8_t = &*dav1d_obmc_masks.as_ptr().offset(h as isize)
        as *const uint8_t;
    h = h * 3 as libc::c_int >> 2 as libc::c_int;
    loop {
        let fresh0 = mask;
        mask = mask.offset(1);
        let m: libc::c_int = *fresh0 as libc::c_int;
        let mut x = 0;
        while x < w {
            *dst
                .offset(
                    x as isize,
                ) = (*dst.offset(x as isize) as libc::c_int * (64 as libc::c_int - m)
                + *tmp.offset(x as isize) as libc::c_int * m + 32 as libc::c_int
                >> 6 as libc::c_int) as pixel;
            x += 1;
        }
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        tmp = tmp.offset(w as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
unsafe extern "C" fn w_mask_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut tmp1: *const int16_t,
    mut tmp2: *const int16_t,
    w: libc::c_int,
    mut h: libc::c_int,
    mut mask: *mut uint8_t,
    sign: libc::c_int,
    ss_hor: libc::c_int,
    ss_ver: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14 as libc::c_int
        - (32 as libc::c_int - clz(bitdepth_max as libc::c_uint));
    let bitdepth: libc::c_int = 32 as libc::c_int - clz(bitdepth_max as libc::c_uint);
    let sh: libc::c_int = intermediate_bits + 6 as libc::c_int;
    let rnd: libc::c_int = ((32 as libc::c_int) << intermediate_bits)
        + 8192 as libc::c_int * 64 as libc::c_int;
    let mask_sh: libc::c_int = bitdepth + intermediate_bits - 4 as libc::c_int;
    let mask_rnd: libc::c_int = (1 as libc::c_int) << mask_sh - 5 as libc::c_int;
    loop {
        let mut x = 0;
        while x < w {
            let m: libc::c_int = imin(
                38 as libc::c_int
                    + ((
                        *tmp1.offset(x as isize) as libc::c_int
                            - *tmp2.offset(x as isize) as libc::c_int
                    ).abs() + mask_rnd >> mask_sh),
                64 as libc::c_int,
            );
            *dst
                .offset(
                    x as isize,
                ) = iclip(
                *tmp1.offset(x as isize) as libc::c_int * m
                    + *tmp2.offset(x as isize) as libc::c_int * (64 as libc::c_int - m)
                    + rnd >> sh,
                0 as libc::c_int,
                bitdepth_max,
            ) as pixel;
            if ss_hor != 0 {
                x += 1;
                let n: libc::c_int = imin(
                    38 as libc::c_int
                        + ((
                            *tmp1.offset(x as isize) as libc::c_int
                                - *tmp2.offset(x as isize) as libc::c_int
                        ).abs() + mask_rnd >> mask_sh),
                    64 as libc::c_int,
                );
                *dst
                    .offset(
                        x as isize,
                    ) = iclip(
                    *tmp1.offset(x as isize) as libc::c_int * n
                        + *tmp2.offset(x as isize) as libc::c_int
                            * (64 as libc::c_int - n) + rnd >> sh,
                    0 as libc::c_int,
                    bitdepth_max,
                ) as pixel;
                if h & ss_ver != 0 {
                    *mask
                        .offset(
                            (x >> 1 as libc::c_int) as isize,
                        ) = (m + n
                        + *mask.offset((x >> 1 as libc::c_int) as isize) as libc::c_int
                        + 2 as libc::c_int - sign >> 2 as libc::c_int) as uint8_t;
                } else if ss_ver != 0 {
                    *mask.offset((x >> 1 as libc::c_int) as isize) = (m + n) as uint8_t;
                } else {
                    *mask
                        .offset(
                            (x >> 1 as libc::c_int) as isize,
                        ) = (m + n + 1 as libc::c_int - sign >> 1 as libc::c_int)
                        as uint8_t;
                }
            } else {
                *mask.offset(x as isize) = m as uint8_t;
            }
            x += 1;
        }
        tmp1 = tmp1.offset(w as isize);
        tmp2 = tmp2.offset(w as isize);
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        if ss_ver == 0 || h & 1 as libc::c_int != 0 {
            mask = mask.offset((w >> ss_hor) as isize);
        }
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}
unsafe extern "C" fn w_mask_444_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    tmp1: *const int16_t,
    tmp2: *const int16_t,
    w: libc::c_int,
    h: libc::c_int,
    mut mask: *mut uint8_t,
    sign: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    w_mask_c(
        dst,
        dst_stride,
        tmp1,
        tmp2,
        w,
        h,
        mask,
        sign,
        0 as libc::c_int,
        0 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn w_mask_422_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    tmp1: *const int16_t,
    tmp2: *const int16_t,
    w: libc::c_int,
    h: libc::c_int,
    mut mask: *mut uint8_t,
    sign: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    w_mask_c(
        dst,
        dst_stride,
        tmp1,
        tmp2,
        w,
        h,
        mask,
        sign,
        1 as libc::c_int,
        0 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn w_mask_420_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    tmp1: *const int16_t,
    tmp2: *const int16_t,
    w: libc::c_int,
    h: libc::c_int,
    mut mask: *mut uint8_t,
    sign: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    w_mask_c(
        dst,
        dst_stride,
        tmp1,
        tmp2,
        w,
        h,
        mask,
        sign,
        1 as libc::c_int,
        1 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn warp_affine_8x8_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut src: *const pixel,
    src_stride: ptrdiff_t,
    abcd: *const int16_t,
    mut mx: libc::c_int,
    mut my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14 as libc::c_int
        - (32 as libc::c_int - clz(bitdepth_max as libc::c_uint));
    let mut mid: [int16_t; 120] = [0; 120];
    let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
    src = src
        .offset(-((3 * PXSTRIDE(src_stride)) as isize));
    let mut y = 0;
    while y < 15 as libc::c_int {
        let mut x = 0;
        let mut tmx: libc::c_int = mx;
        while x < 8 as libc::c_int {
            let filter: *const int8_t = (dav1d_mc_warp_filter[(64 as libc::c_int
                + (tmx + 512 as libc::c_int >> 10 as libc::c_int)) as usize])
                .as_ptr();
            *mid_ptr
                .offset(
                    x as isize,
                ) = (*filter.offset(0) as libc::c_int
                * *src.offset((x - 3 as libc::c_int * 1 as libc::c_int) as isize)
                    as libc::c_int
                + *filter.offset(1) as libc::c_int
                    * *src.offset((x - 2 as libc::c_int * 1 as libc::c_int) as isize)
                        as libc::c_int
                + *filter.offset(2) as libc::c_int
                    * *src.offset((x - 1 as libc::c_int * 1 as libc::c_int) as isize)
                        as libc::c_int
                + *filter.offset(3) as libc::c_int
                    * *src.offset((x + 0 as libc::c_int * 1 as libc::c_int) as isize)
                        as libc::c_int
                + *filter.offset(4) as libc::c_int
                    * *src.offset((x + 1 as libc::c_int * 1 as libc::c_int) as isize)
                        as libc::c_int
                + *filter.offset(5) as libc::c_int
                    * *src.offset((x + 2 as libc::c_int * 1 as libc::c_int) as isize)
                        as libc::c_int
                + *filter.offset(6) as libc::c_int
                    * *src.offset((x + 3 as libc::c_int * 1 as libc::c_int) as isize)
                        as libc::c_int
                + *filter.offset(7) as libc::c_int
                    * *src.offset((x + 4 as libc::c_int * 1 as libc::c_int) as isize)
                        as libc::c_int
                + ((1 as libc::c_int) << 7 as libc::c_int - intermediate_bits
                    >> 1 as libc::c_int) >> 7 as libc::c_int - intermediate_bits)
                as int16_t;
            x += 1;
            tmx += *abcd.offset(0) as libc::c_int;
        }
        src = src.offset(PXSTRIDE(src_stride) as isize);
        mid_ptr = mid_ptr.offset(8);
        y += 1;
        mx += *abcd.offset(1) as libc::c_int;
    }
    mid_ptr = &mut *mid
        .as_mut_ptr()
        .offset((3 as libc::c_int * 8 as libc::c_int) as isize) as *mut int16_t;
    let mut y_0 = 0;
    while y_0 < 8 as libc::c_int {
        let mut x_0 = 0;
        let mut tmy: libc::c_int = my;
        while x_0 < 8 as libc::c_int {
            let filter_0: *const int8_t = (dav1d_mc_warp_filter[(64 as libc::c_int
                + (tmy + 512 as libc::c_int >> 10 as libc::c_int)) as usize])
                .as_ptr();
            *dst
                .offset(
                    x_0 as isize,
                ) = iclip(
                *filter_0.offset(0) as libc::c_int
                    * *mid_ptr
                        .offset((x_0 - 3 as libc::c_int * 8 as libc::c_int) as isize)
                        as libc::c_int
                    + *filter_0.offset(1) as libc::c_int
                        * *mid_ptr
                            .offset((x_0 - 2 as libc::c_int * 8 as libc::c_int) as isize)
                            as libc::c_int
                    + *filter_0.offset(2) as libc::c_int
                        * *mid_ptr
                            .offset((x_0 - 1 as libc::c_int * 8 as libc::c_int) as isize)
                            as libc::c_int
                    + *filter_0.offset(3) as libc::c_int
                        * *mid_ptr
                            .offset((x_0 + 0 as libc::c_int * 8 as libc::c_int) as isize)
                            as libc::c_int
                    + *filter_0.offset(4) as libc::c_int
                        * *mid_ptr
                            .offset((x_0 + 1 as libc::c_int * 8 as libc::c_int) as isize)
                            as libc::c_int
                    + *filter_0.offset(5) as libc::c_int
                        * *mid_ptr
                            .offset((x_0 + 2 as libc::c_int * 8 as libc::c_int) as isize)
                            as libc::c_int
                    + *filter_0.offset(6) as libc::c_int
                        * *mid_ptr
                            .offset((x_0 + 3 as libc::c_int * 8 as libc::c_int) as isize)
                            as libc::c_int
                    + *filter_0.offset(7) as libc::c_int
                        * *mid_ptr
                            .offset((x_0 + 4 as libc::c_int * 8 as libc::c_int) as isize)
                            as libc::c_int
                    + ((1 as libc::c_int) << 7 as libc::c_int + intermediate_bits
                        >> 1 as libc::c_int) >> 7 as libc::c_int + intermediate_bits,
                0 as libc::c_int,
                bitdepth_max,
            ) as pixel;
            x_0 += 1;
            tmy += *abcd.offset(2) as libc::c_int;
        }
        mid_ptr = mid_ptr.offset(8);
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        y_0 += 1;
        my += *abcd.offset(3) as libc::c_int;
    }
}
unsafe extern "C" fn warp_affine_8x8t_c(
    mut tmp: *mut int16_t,
    tmp_stride: ptrdiff_t,
    mut src: *const pixel,
    src_stride: ptrdiff_t,
    abcd: *const int16_t,
    mut mx: libc::c_int,
    mut my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14 as libc::c_int
        - (32 as libc::c_int - clz(bitdepth_max as libc::c_uint));
    let mut mid: [int16_t; 120] = [0; 120];
    let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
    src = src
        .offset(-((3 * PXSTRIDE(src_stride)) as isize));
    let mut y = 0;
    while y < 15 as libc::c_int {
        let mut x = 0;
        let mut tmx: libc::c_int = mx;
        while x < 8 as libc::c_int {
            let filter: *const int8_t = (dav1d_mc_warp_filter[(64 as libc::c_int
                + (tmx + 512 as libc::c_int >> 10 as libc::c_int)) as usize])
                .as_ptr();
            *mid_ptr
                .offset(
                    x as isize,
                ) = (*filter.offset(0) as libc::c_int
                * *src.offset((x - 3 as libc::c_int * 1 as libc::c_int) as isize)
                    as libc::c_int
                + *filter.offset(1) as libc::c_int
                    * *src.offset((x - 2 as libc::c_int * 1 as libc::c_int) as isize)
                        as libc::c_int
                + *filter.offset(2) as libc::c_int
                    * *src.offset((x - 1 as libc::c_int * 1 as libc::c_int) as isize)
                        as libc::c_int
                + *filter.offset(3) as libc::c_int
                    * *src.offset((x + 0 as libc::c_int * 1 as libc::c_int) as isize)
                        as libc::c_int
                + *filter.offset(4) as libc::c_int
                    * *src.offset((x + 1 as libc::c_int * 1 as libc::c_int) as isize)
                        as libc::c_int
                + *filter.offset(5) as libc::c_int
                    * *src.offset((x + 2 as libc::c_int * 1 as libc::c_int) as isize)
                        as libc::c_int
                + *filter.offset(6) as libc::c_int
                    * *src.offset((x + 3 as libc::c_int * 1 as libc::c_int) as isize)
                        as libc::c_int
                + *filter.offset(7) as libc::c_int
                    * *src.offset((x + 4 as libc::c_int * 1 as libc::c_int) as isize)
                        as libc::c_int
                + ((1 as libc::c_int) << 7 as libc::c_int - intermediate_bits
                    >> 1 as libc::c_int) >> 7 as libc::c_int - intermediate_bits)
                as int16_t;
            x += 1;
            tmx += *abcd.offset(0) as libc::c_int;
        }
        src = src.offset(PXSTRIDE(src_stride) as isize);
        mid_ptr = mid_ptr.offset(8);
        y += 1;
        mx += *abcd.offset(1) as libc::c_int;
    }
    mid_ptr = &mut *mid
        .as_mut_ptr()
        .offset((3 as libc::c_int * 8 as libc::c_int) as isize) as *mut int16_t;
    let mut y_0 = 0;
    while y_0 < 8 as libc::c_int {
        let mut x_0 = 0;
        let mut tmy: libc::c_int = my;
        while x_0 < 8 as libc::c_int {
            let filter_0: *const int8_t = (dav1d_mc_warp_filter[(64 as libc::c_int
                + (tmy + 512 as libc::c_int >> 10 as libc::c_int)) as usize])
                .as_ptr();
            *tmp
                .offset(
                    x_0 as isize,
                ) = ((*filter_0.offset(0) as libc::c_int
                * *mid_ptr.offset((x_0 - 3 as libc::c_int * 8 as libc::c_int) as isize)
                    as libc::c_int
                + *filter_0.offset(1) as libc::c_int
                    * *mid_ptr
                        .offset((x_0 - 2 as libc::c_int * 8 as libc::c_int) as isize)
                        as libc::c_int
                + *filter_0.offset(2) as libc::c_int
                    * *mid_ptr
                        .offset((x_0 - 1 as libc::c_int * 8 as libc::c_int) as isize)
                        as libc::c_int
                + *filter_0.offset(3) as libc::c_int
                    * *mid_ptr
                        .offset((x_0 + 0 as libc::c_int * 8 as libc::c_int) as isize)
                        as libc::c_int
                + *filter_0.offset(4) as libc::c_int
                    * *mid_ptr
                        .offset((x_0 + 1 as libc::c_int * 8 as libc::c_int) as isize)
                        as libc::c_int
                + *filter_0.offset(5) as libc::c_int
                    * *mid_ptr
                        .offset((x_0 + 2 as libc::c_int * 8 as libc::c_int) as isize)
                        as libc::c_int
                + *filter_0.offset(6) as libc::c_int
                    * *mid_ptr
                        .offset((x_0 + 3 as libc::c_int * 8 as libc::c_int) as isize)
                        as libc::c_int
                + *filter_0.offset(7) as libc::c_int
                    * *mid_ptr
                        .offset((x_0 + 4 as libc::c_int * 8 as libc::c_int) as isize)
                        as libc::c_int
                + ((1 as libc::c_int) << 7 as libc::c_int >> 1 as libc::c_int)
                >> 7 as libc::c_int) - 8192 as libc::c_int) as int16_t;
            x_0 += 1;
            tmy += *abcd.offset(2) as libc::c_int;
        }
        mid_ptr = mid_ptr.offset(8);
        tmp = tmp.offset(tmp_stride as isize);
        y_0 += 1;
        my += *abcd.offset(3) as libc::c_int;
    }
}
unsafe extern "C" fn emu_edge_c(
    bw: intptr_t,
    bh: intptr_t,
    iw: intptr_t,
    ih: intptr_t,
    x: intptr_t,
    y: intptr_t,
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut r#ref: *const pixel,
    ref_stride: ptrdiff_t,
) {
    r#ref = r#ref
        .offset(
            iclip(
                y as libc::c_int,
                0 as libc::c_int,
                ih as libc::c_int - 1 as libc::c_int,
            ) as isize * PXSTRIDE(ref_stride)
                + iclip(
                    x as libc::c_int,
                    0 as libc::c_int,
                    iw as libc::c_int - 1 as libc::c_int,
                ) as isize,
        );
    let left_ext: libc::c_int = iclip(
        -x as libc::c_int,
        0 as libc::c_int,
        bw as libc::c_int - 1 as libc::c_int,
    );
    let right_ext: libc::c_int = iclip(
        (x + bw - iw) as libc::c_int,
        0 as libc::c_int,
        bw as libc::c_int - 1 as libc::c_int,
    );
    if !(((left_ext + right_ext) as isize) < bw) {
        unreachable!();
    }
    let top_ext: libc::c_int = iclip(
        -y as libc::c_int,
        0 as libc::c_int,
        bh as libc::c_int - 1 as libc::c_int,
    );
    let bottom_ext: libc::c_int = iclip(
        (y + bh - ih) as libc::c_int,
        0 as libc::c_int,
        bh as libc::c_int - 1 as libc::c_int,
    );
    if !(((top_ext + bottom_ext) as isize) < bh) {
        unreachable!();
    }
    let mut blk: *mut pixel = dst
        .offset((top_ext as isize * PXSTRIDE(dst_stride)) as isize);
    let center_w: libc::c_int = (bw - left_ext as isize - right_ext as isize) as libc::c_int;
    let center_h: libc::c_int = (bh - top_ext as isize - bottom_ext as isize) as libc::c_int;
    let mut y_0 = 0;
    while y_0 < center_h {
        memcpy(
            blk.offset(left_ext as isize) as *mut libc::c_void,
            r#ref as *const libc::c_void,
            (center_w << 1 as libc::c_int) as libc::c_ulong,
        );
        if left_ext != 0 {
            pixel_set(blk, *blk.offset(left_ext as isize) as libc::c_int, left_ext);
        }
        if right_ext != 0 {
            pixel_set(
                blk.offset(left_ext as isize).offset(center_w as isize),
                *blk.offset((left_ext + center_w - 1 as libc::c_int) as isize)
                    as libc::c_int,
                right_ext,
            );
        }
        r#ref = r#ref.offset(PXSTRIDE(ref_stride) as isize);
        blk = blk.offset(PXSTRIDE(dst_stride) as isize);
        y_0 += 1;
    }
    blk = dst.offset((top_ext as isize * PXSTRIDE(dst_stride)) as isize);
    let mut y_1 = 0;
    while y_1 < top_ext {
        memcpy(
            dst as *mut libc::c_void,
            blk as *const libc::c_void,
            (bw << 1 as libc::c_int) as libc::c_ulong,
        );
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        y_1 += 1;
    }
    dst = dst.offset((center_h as isize * PXSTRIDE(dst_stride)) as isize);
    let mut y_2 = 0;
    while y_2 < bottom_ext {
        memcpy(
            dst as *mut libc::c_void,
            &mut *dst
                .offset(
                    -(PXSTRIDE
                        as unsafe extern "C" fn(ptrdiff_t) -> ptrdiff_t)(dst_stride)
                        as isize,
                ) as *mut pixel as *const libc::c_void,
            (bw << 1 as libc::c_int) as libc::c_ulong,
        );
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        y_2 += 1;
    }
}
unsafe extern "C" fn resize_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut src: *const pixel,
    src_stride: ptrdiff_t,
    dst_w: libc::c_int,
    mut h: libc::c_int,
    src_w: libc::c_int,
    dx: libc::c_int,
    mx0: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    loop {
        let mut mx: libc::c_int = mx0;
        let mut src_x: libc::c_int = -(1 as libc::c_int);
        let mut x = 0;
        while x < dst_w {
            let F: *const int8_t = (dav1d_resize_filter[(mx >> 8 as libc::c_int)
                as usize])
                .as_ptr();
            *dst
                .offset(
                    x as isize,
                ) = iclip(
                -(*F.offset(0) as libc::c_int
                    * *src
                        .offset(
                            iclip(
                                src_x - 3 as libc::c_int,
                                0 as libc::c_int,
                                src_w - 1 as libc::c_int,
                            ) as isize,
                        ) as libc::c_int
                    + *F.offset(1) as libc::c_int
                        * *src
                            .offset(
                                iclip(
                                    src_x - 2 as libc::c_int,
                                    0 as libc::c_int,
                                    src_w - 1 as libc::c_int,
                                ) as isize,
                            ) as libc::c_int
                    + *F.offset(2) as libc::c_int
                        * *src
                            .offset(
                                iclip(
                                    src_x - 1 as libc::c_int,
                                    0 as libc::c_int,
                                    src_w - 1 as libc::c_int,
                                ) as isize,
                            ) as libc::c_int
                    + *F.offset(3) as libc::c_int
                        * *src
                            .offset(
                                iclip(
                                    src_x + 0 as libc::c_int,
                                    0 as libc::c_int,
                                    src_w - 1 as libc::c_int,
                                ) as isize,
                            ) as libc::c_int
                    + *F.offset(4) as libc::c_int
                        * *src
                            .offset(
                                iclip(
                                    src_x + 1 as libc::c_int,
                                    0 as libc::c_int,
                                    src_w - 1 as libc::c_int,
                                ) as isize,
                            ) as libc::c_int
                    + *F.offset(5) as libc::c_int
                        * *src
                            .offset(
                                iclip(
                                    src_x + 2 as libc::c_int,
                                    0 as libc::c_int,
                                    src_w - 1 as libc::c_int,
                                ) as isize,
                            ) as libc::c_int
                    + *F.offset(6) as libc::c_int
                        * *src
                            .offset(
                                iclip(
                                    src_x + 3 as libc::c_int,
                                    0 as libc::c_int,
                                    src_w - 1 as libc::c_int,
                                ) as isize,
                            ) as libc::c_int
                    + *F.offset(7) as libc::c_int
                        * *src
                            .offset(
                                iclip(
                                    src_x + 4 as libc::c_int,
                                    0 as libc::c_int,
                                    src_w - 1 as libc::c_int,
                                ) as isize,
                            ) as libc::c_int) + 64 as libc::c_int >> 7 as libc::c_int,
                0 as libc::c_int,
                bitdepth_max,
            ) as pixel;
            mx += dx;
            src_x += mx >> 14 as libc::c_int;
            mx &= 0x3fff as libc::c_int;
            x += 1;
        }
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        src = src.offset(PXSTRIDE(src_stride) as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    };
}

#[cfg(feature = "asm")]
use crate::src::cpu::dav1d_get_cpu_flags;

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
#[inline(always)]
unsafe extern "C" fn mc_dsp_init_x86(c: *mut Dav1dMCDSPContext) {
    use crate::src::x86::cpu::DAV1D_X86_CPU_FLAG_AVX512ICL;
    use crate::src::x86::cpu::DAV1D_X86_CPU_FLAG_SSE2;
    use crate::src::x86::cpu::DAV1D_X86_CPU_FLAG_AVX2;
    use crate::src::x86::cpu::DAV1D_X86_CPU_FLAG_SSE41;
    use crate::src::x86::cpu::DAV1D_X86_CPU_FLAG_SSSE3;

    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_X86_CPU_FLAG_SSE2 == 0 {
        return;
    }

    if flags & DAV1D_X86_CPU_FLAG_SSSE3 == 0 {
        return;
    }

    (*c).mc[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_put_8tap_regular_16bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] = Some(dav1d_put_8tap_regular_smooth_16bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(dav1d_put_8tap_regular_sharp_16bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] = Some(dav1d_put_8tap_smooth_regular_16bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_put_8tap_smooth_16bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_put_8tap_smooth_sharp_16bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(dav1d_put_8tap_sharp_regular_16bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_put_8tap_sharp_smooth_16bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_put_8tap_sharp_16bpc_ssse3);
    (*c).mc[FILTER_2D_BILINEAR as usize] = Some(dav1d_put_bilin_16bpc_ssse3);

    (*c).mct[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_prep_8tap_regular_16bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] = Some(dav1d_prep_8tap_regular_smooth_16bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(dav1d_prep_8tap_regular_sharp_16bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] = Some(dav1d_prep_8tap_smooth_regular_16bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_prep_8tap_smooth_16bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_prep_8tap_smooth_sharp_16bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(dav1d_prep_8tap_sharp_regular_16bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_prep_8tap_sharp_smooth_16bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_prep_8tap_sharp_16bpc_ssse3);
    (*c).mct[FILTER_2D_BILINEAR as usize] = Some(dav1d_prep_bilin_16bpc_ssse3);

    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_put_8tap_scaled_regular_16bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] = Some(dav1d_put_8tap_scaled_regular_smooth_16bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(dav1d_put_8tap_scaled_regular_sharp_16bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] = Some(dav1d_put_8tap_scaled_smooth_regular_16bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_put_8tap_scaled_smooth_16bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_put_8tap_scaled_smooth_sharp_16bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(dav1d_put_8tap_scaled_sharp_regular_16bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_put_8tap_scaled_sharp_smooth_16bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_put_8tap_scaled_sharp_16bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_BILINEAR as usize] = Some(dav1d_put_bilin_scaled_16bpc_ssse3);

    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_prep_8tap_scaled_regular_16bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] = Some(dav1d_prep_8tap_scaled_regular_smooth_16bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(dav1d_prep_8tap_scaled_regular_sharp_16bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] = Some(dav1d_prep_8tap_scaled_smooth_regular_16bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_prep_8tap_scaled_smooth_16bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_prep_8tap_scaled_smooth_sharp_16bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(dav1d_prep_8tap_scaled_sharp_regular_16bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_prep_8tap_scaled_sharp_smooth_16bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_prep_8tap_scaled_sharp_16bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_BILINEAR as usize] = Some(dav1d_prep_bilin_scaled_16bpc_ssse3);

    (*c).avg = Some(dav1d_avg_16bpc_ssse3);
    (*c).w_avg = Some(dav1d_w_avg_16bpc_ssse3);
    (*c).mask = Some(dav1d_mask_16bpc_ssse3);

    (*c).w_mask[0] = Some(dav1d_w_mask_444_16bpc_ssse3);
    (*c).w_mask[1] = Some(dav1d_w_mask_422_16bpc_ssse3);
    (*c).w_mask[2] = Some(dav1d_w_mask_420_16bpc_ssse3);

    (*c).blend = Some(dav1d_blend_16bpc_ssse3);
    (*c).blend_v = Some(dav1d_blend_v_16bpc_ssse3);
    (*c).blend_h = Some(dav1d_blend_h_16bpc_ssse3);
    (*c).warp8x8 = Some(dav1d_warp_affine_8x8_16bpc_ssse3);
    (*c).warp8x8t = Some(dav1d_warp_affine_8x8t_16bpc_ssse3);
    (*c).emu_edge = Some(dav1d_emu_edge_16bpc_ssse3);
    (*c).resize = Some(dav1d_resize_16bpc_ssse3);

    if flags & DAV1D_X86_CPU_FLAG_SSE41 == 0 {
        return;
    }

    if flags & DAV1D_X86_CPU_FLAG_AVX2 == 0 {
        return;
    }

    (*c).mc[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_put_8tap_regular_16bpc_avx2);
    (*c).mc[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] = Some(dav1d_put_8tap_regular_smooth_16bpc_avx2);
    (*c).mc[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(dav1d_put_8tap_regular_sharp_16bpc_avx2);
    (*c).mc[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] = Some(dav1d_put_8tap_smooth_regular_16bpc_avx2);
    (*c).mc[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_put_8tap_smooth_16bpc_avx2);
    (*c).mc[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_put_8tap_smooth_sharp_16bpc_avx2);
    (*c).mc[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(dav1d_put_8tap_sharp_regular_16bpc_avx2);
    (*c).mc[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_put_8tap_sharp_smooth_16bpc_avx2);
    (*c).mc[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_put_8tap_sharp_16bpc_avx2);
    (*c).mc[FILTER_2D_BILINEAR as usize] = Some(dav1d_put_bilin_16bpc_avx2);

    (*c).mct[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_prep_8tap_regular_16bpc_avx2);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] = Some(dav1d_prep_8tap_regular_smooth_16bpc_avx2);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(dav1d_prep_8tap_regular_sharp_16bpc_avx2);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] = Some(dav1d_prep_8tap_smooth_regular_16bpc_avx2);
    (*c).mct[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_prep_8tap_smooth_16bpc_avx2);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_prep_8tap_smooth_sharp_16bpc_avx2);
    (*c).mct[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(dav1d_prep_8tap_sharp_regular_16bpc_avx2);
    (*c).mct[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_prep_8tap_sharp_smooth_16bpc_avx2);
    (*c).mct[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_prep_8tap_sharp_16bpc_avx2);
    (*c).mct[FILTER_2D_BILINEAR as usize] = Some(dav1d_prep_bilin_16bpc_avx2);

    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_put_8tap_scaled_regular_16bpc_avx2);
    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] = Some(dav1d_put_8tap_scaled_regular_smooth_16bpc_avx2);
    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(dav1d_put_8tap_scaled_regular_sharp_16bpc_avx2);
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] = Some(dav1d_put_8tap_scaled_smooth_regular_16bpc_avx2);
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_put_8tap_scaled_smooth_16bpc_avx2);
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_put_8tap_scaled_smooth_sharp_16bpc_avx2);
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(dav1d_put_8tap_scaled_sharp_regular_16bpc_avx2);
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_put_8tap_scaled_sharp_smooth_16bpc_avx2);
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_put_8tap_scaled_sharp_16bpc_avx2);
    (*c).mc_scaled[FILTER_2D_BILINEAR as usize] = Some(dav1d_put_bilin_scaled_16bpc_avx2);

    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_prep_8tap_scaled_regular_16bpc_avx2);
    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] = Some(dav1d_prep_8tap_scaled_regular_smooth_16bpc_avx2);
    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(dav1d_prep_8tap_scaled_regular_sharp_16bpc_avx2);
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] = Some(dav1d_prep_8tap_scaled_smooth_regular_16bpc_avx2);
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_prep_8tap_scaled_smooth_16bpc_avx2);
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_prep_8tap_scaled_smooth_sharp_16bpc_avx2);
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(dav1d_prep_8tap_scaled_sharp_regular_16bpc_avx2);
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_prep_8tap_scaled_sharp_smooth_16bpc_avx2);
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_prep_8tap_scaled_sharp_16bpc_avx2);
    (*c).mct_scaled[FILTER_2D_BILINEAR as usize] = Some(dav1d_prep_bilin_scaled_16bpc_avx2);

    (*c).avg = Some(dav1d_avg_16bpc_avx2);
    (*c).w_avg = Some(dav1d_w_avg_16bpc_avx2);
    (*c).mask = Some(dav1d_mask_16bpc_avx2);

    (*c).w_mask[0] = Some(dav1d_w_mask_444_16bpc_avx2);
    (*c).w_mask[1] = Some(dav1d_w_mask_422_16bpc_avx2);
    (*c).w_mask[2] = Some(dav1d_w_mask_420_16bpc_avx2);

    (*c).blend = Some(dav1d_blend_16bpc_avx2);
    (*c).blend_v = Some(dav1d_blend_v_16bpc_avx2);
    (*c).blend_h = Some(dav1d_blend_h_16bpc_avx2);
    (*c).warp8x8 = Some(dav1d_warp_affine_8x8_16bpc_avx2);
    (*c).warp8x8t = Some(dav1d_warp_affine_8x8t_16bpc_avx2);
    (*c).emu_edge = Some(dav1d_emu_edge_16bpc_avx2);
    (*c).resize = Some(dav1d_resize_16bpc_avx2);

    if flags & DAV1D_X86_CPU_FLAG_AVX512ICL == 0 {
        return;
    }

    (*c).mc[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_put_8tap_regular_16bpc_avx512icl);
    (*c).mc[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] = Some(dav1d_put_8tap_regular_smooth_16bpc_avx512icl);
    (*c).mc[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(dav1d_put_8tap_regular_sharp_16bpc_avx512icl);
    (*c).mc[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] = Some(dav1d_put_8tap_smooth_regular_16bpc_avx512icl);
    (*c).mc[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_put_8tap_smooth_16bpc_avx512icl);
    (*c).mc[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_put_8tap_smooth_sharp_16bpc_avx512icl);
    (*c).mc[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(dav1d_put_8tap_sharp_regular_16bpc_avx512icl);
    (*c).mc[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_put_8tap_sharp_smooth_16bpc_avx512icl);
    (*c).mc[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_put_8tap_sharp_16bpc_avx512icl);
    (*c).mc[FILTER_2D_BILINEAR as usize] = Some(dav1d_put_bilin_16bpc_avx512icl);

    (*c).mct[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_prep_8tap_regular_16bpc_avx512icl);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] = Some(dav1d_prep_8tap_regular_smooth_16bpc_avx512icl);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(dav1d_prep_8tap_regular_sharp_16bpc_avx512icl);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] = Some(dav1d_prep_8tap_smooth_regular_16bpc_avx512icl);
    (*c).mct[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_prep_8tap_smooth_16bpc_avx512icl);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_prep_8tap_smooth_sharp_16bpc_avx512icl);
    (*c).mct[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(dav1d_prep_8tap_sharp_regular_16bpc_avx512icl);
    (*c).mct[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_prep_8tap_sharp_smooth_16bpc_avx512icl);
    (*c).mct[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_prep_8tap_sharp_16bpc_avx512icl);
    (*c).mct[FILTER_2D_BILINEAR as usize] = Some(dav1d_prep_bilin_16bpc_avx512icl);

    (*c).avg = Some(dav1d_avg_16bpc_avx512icl);
    (*c).w_avg = Some(dav1d_w_avg_16bpc_avx512icl);
    (*c).mask = Some(dav1d_mask_16bpc_avx512icl);

    (*c).w_mask[0] = Some(dav1d_w_mask_444_16bpc_avx512icl);
    (*c).w_mask[1] = Some(dav1d_w_mask_422_16bpc_avx512icl);
    (*c).w_mask[2] = Some(dav1d_w_mask_420_16bpc_avx512icl);

    (*c).blend = Some(dav1d_blend_16bpc_avx512icl);
    (*c).blend_v = Some(dav1d_blend_v_16bpc_avx512icl);
    (*c).blend_h = Some(dav1d_blend_h_16bpc_avx512icl);
    (*c).warp8x8 = Some(dav1d_warp_affine_8x8_16bpc_avx512icl);
    (*c).warp8x8t = Some(dav1d_warp_affine_8x8t_16bpc_avx512icl);
    (*c).resize = Some(dav1d_resize_16bpc_avx512icl);
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
#[inline(always)]
unsafe extern "C" fn mc_dsp_init_arm(c: *mut Dav1dMCDSPContext) {
    use crate::src::arm::cpu::DAV1D_ARM_CPU_FLAG_NEON;

    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
        return;
    }

    (*c).mc[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_put_8tap_regular_16bpc_neon);
    (*c).mc[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] = Some(dav1d_put_8tap_regular_smooth_16bpc_neon);
    (*c).mc[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(dav1d_put_8tap_regular_sharp_16bpc_neon);
    (*c).mc[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] = Some(dav1d_put_8tap_smooth_regular_16bpc_neon);
    (*c).mc[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_put_8tap_smooth_16bpc_neon);
    (*c).mc[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_put_8tap_smooth_sharp_16bpc_neon);
    (*c).mc[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(dav1d_put_8tap_sharp_regular_16bpc_neon);
    (*c).mc[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_put_8tap_sharp_smooth_16bpc_neon);
    (*c).mc[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_put_8tap_sharp_16bpc_neon);
    (*c).mc[FILTER_2D_BILINEAR as usize] = Some(dav1d_put_bilin_16bpc_neon);

    (*c).mct[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_prep_8tap_regular_16bpc_neon);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] = Some(dav1d_prep_8tap_regular_smooth_16bpc_neon);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(dav1d_prep_8tap_regular_sharp_16bpc_neon);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] = Some(dav1d_prep_8tap_smooth_regular_16bpc_neon);
    (*c).mct[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_prep_8tap_smooth_16bpc_neon);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_prep_8tap_smooth_sharp_16bpc_neon);
    (*c).mct[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(dav1d_prep_8tap_sharp_regular_16bpc_neon);
    (*c).mct[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_prep_8tap_sharp_smooth_16bpc_neon);
    (*c).mct[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_prep_8tap_sharp_16bpc_neon);
    (*c).mct[FILTER_2D_BILINEAR as usize] = Some(dav1d_prep_bilin_16bpc_neon);

    (*c).avg = Some(dav1d_avg_16bpc_neon);
    (*c).w_avg = Some(dav1d_w_avg_16bpc_neon);
    (*c).mask = Some(dav1d_mask_16bpc_neon);
    (*c).blend = Some(dav1d_blend_16bpc_neon);
    (*c).blend_h = Some(dav1d_blend_h_16bpc_neon);
    (*c).blend_v = Some(dav1d_blend_v_16bpc_neon);

    (*c).w_mask[0] = Some(dav1d_w_mask_444_16bpc_neon);
    (*c).w_mask[1] = Some(dav1d_w_mask_422_16bpc_neon);
    (*c).w_mask[2] = Some(dav1d_w_mask_420_16bpc_neon);

    (*c).warp8x8 = Some(dav1d_warp_affine_8x8_16bpc_neon);
    (*c).warp8x8t = Some(dav1d_warp_affine_8x8t_16bpc_neon);
    (*c).emu_edge = Some(dav1d_emu_edge_16bpc_neon);
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_mc_dsp_init_16bpc(c: *mut Dav1dMCDSPContext) {
    (*c).mc[FILTER_2D_8TAP_REGULAR as usize] = Some(put_8tap_regular_c);
    (*c).mc[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] = Some(put_8tap_regular_smooth_c);
    (*c).mc[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(put_8tap_regular_sharp_c);
    (*c).mc[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(put_8tap_sharp_regular_c);
    (*c).mc[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(put_8tap_sharp_smooth_c);
    (*c).mc[FILTER_2D_8TAP_SHARP as usize] = Some(put_8tap_sharp_c);
    (*c).mc[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] = Some(put_8tap_smooth_regular_c);
    (*c).mc[FILTER_2D_8TAP_SMOOTH as usize] = Some(put_8tap_smooth_c);
    (*c).mc[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(put_8tap_smooth_sharp_c);
    (*c).mc[FILTER_2D_BILINEAR as usize] = Some(put_bilin_c);

    (*c).mct[FILTER_2D_8TAP_REGULAR as usize] = Some(prep_8tap_regular_c);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] = Some(prep_8tap_regular_smooth_c);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(prep_8tap_regular_sharp_c);
    (*c).mct[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(prep_8tap_sharp_regular_c);
    (*c).mct[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(prep_8tap_sharp_smooth_c);
    (*c).mct[FILTER_2D_8TAP_SHARP as usize] = Some(prep_8tap_sharp_c);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] = Some(prep_8tap_smooth_regular_c);
    (*c).mct[FILTER_2D_8TAP_SMOOTH as usize] = Some(prep_8tap_smooth_c);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(prep_8tap_smooth_sharp_c);
    (*c).mct[FILTER_2D_BILINEAR as usize] = Some(prep_bilin_c);

    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR as usize] = Some(put_8tap_regular_scaled_c);
    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] = Some(put_8tap_regular_smooth_scaled_c);
    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(put_8tap_regular_sharp_scaled_c);
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(put_8tap_sharp_regular_scaled_c);
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(put_8tap_sharp_smooth_scaled_c);
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP as usize] = Some(put_8tap_sharp_scaled_c);
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] = Some(put_8tap_smooth_regular_scaled_c);
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH as usize] = Some(put_8tap_smooth_scaled_c);
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(put_8tap_smooth_sharp_scaled_c);
    (*c).mc_scaled[FILTER_2D_BILINEAR as usize] = Some(put_bilin_scaled_c);

    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR as usize] = Some(prep_8tap_regular_scaled_c);
    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] = Some(prep_8tap_regular_smooth_scaled_c);
    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(prep_8tap_regular_sharp_scaled_c);
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(prep_8tap_sharp_regular_scaled_c);
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(prep_8tap_sharp_smooth_scaled_c);
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP as usize] = Some(prep_8tap_sharp_scaled_c);
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] = Some(prep_8tap_smooth_regular_scaled_c);
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH as usize] = Some(prep_8tap_smooth_scaled_c);
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(prep_8tap_smooth_sharp_scaled_c);
    (*c).mct_scaled[FILTER_2D_BILINEAR as usize] = Some(prep_bilin_scaled_c);

    (*c).avg = Some(avg_c);
    (*c).w_avg = Some(w_avg_c);
    (*c).mask = Some(mask_c);

    (*c).w_mask[0 as usize] = Some(w_mask_444_c);
    (*c).w_mask[1 as usize] = Some(w_mask_422_c);
    (*c).w_mask[2 as usize] = Some(w_mask_420_c);

    (*c).blend = Some(blend_c);
    (*c).blend_v = Some(blend_v_c);
    (*c).blend_h = Some(blend_h_c);
    (*c).warp8x8 = Some(warp_affine_8x8_c);
    (*c).warp8x8t = Some(warp_affine_8x8t_c);
    (*c).emu_edge = Some(emu_edge_c);
    (*c).resize = Some(resize_c);

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            mc_dsp_init_x86(c);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
            mc_dsp_init_arm(c);
        }
    }
}
