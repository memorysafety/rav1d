use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::BitDepth16;
use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
#[cfg(feature = "asm")]
use cfg_if::cfg_if;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
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

use crate::include::dav1d::headers::DAV1D_FILTER_8TAP_REGULAR;
use crate::include::dav1d::headers::DAV1D_FILTER_8TAP_SHARP;
use crate::include::dav1d::headers::DAV1D_FILTER_8TAP_SMOOTH;
use crate::src::levels::FILTER_2D_8TAP_REGULAR;
use crate::src::levels::FILTER_2D_8TAP_REGULAR_SHARP;
use crate::src::levels::FILTER_2D_8TAP_REGULAR_SMOOTH;
use crate::src::levels::FILTER_2D_8TAP_SHARP;
use crate::src::levels::FILTER_2D_8TAP_SHARP_REGULAR;
use crate::src::levels::FILTER_2D_8TAP_SHARP_SMOOTH;
use crate::src::levels::FILTER_2D_8TAP_SMOOTH;
use crate::src::levels::FILTER_2D_8TAP_SMOOTH_REGULAR;
use crate::src::levels::FILTER_2D_8TAP_SMOOTH_SHARP;
use crate::src::levels::FILTER_2D_BILINEAR;
pub type mc_fn = Option<
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
pub type mc_scaled_fn = Option<
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
pub type warp8x8_fn = Option<
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
pub type mct_fn = Option<
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
pub type mct_scaled_fn = Option<
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
pub type warp8x8t_fn = Option<
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
pub type avg_fn = Option<
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
pub type w_avg_fn = Option<
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
pub type mask_fn = Option<
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
pub type w_mask_fn = Option<
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
pub type blend_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        libc::c_int,
        libc::c_int,
        *const uint8_t,
    ) -> (),
>;
pub type blend_dir_fn = Option<
    unsafe extern "C" fn(*mut pixel, ptrdiff_t, *const pixel, libc::c_int, libc::c_int) -> (),
>;
pub type emu_edge_fn = Option<
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
pub type resize_fn = Option<
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
use crate::src::mc::prep_8tap_rust;
use crate::src::mc::prep_8tap_scaled_rust;
use crate::src::mc::put_8tap_rust;
use crate::src::mc::put_8tap_scaled_rust;
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
    put_8tap_rust(
        dst,
        dst_stride as usize,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        DAV1D_FILTER_8TAP_REGULAR | (DAV1D_FILTER_8TAP_REGULAR << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    put_8tap_scaled_rust(
        dst,
        dst_stride as usize,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        dx as usize,
        dy as usize,
        DAV1D_FILTER_8TAP_REGULAR | (DAV1D_FILTER_8TAP_REGULAR << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    prep_8tap_rust(
        tmp,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        DAV1D_FILTER_8TAP_REGULAR | (DAV1D_FILTER_8TAP_REGULAR << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    prep_8tap_scaled_rust(
        tmp,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        dx as usize,
        dy as usize,
        DAV1D_FILTER_8TAP_REGULAR | (DAV1D_FILTER_8TAP_REGULAR << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    put_8tap_rust(
        dst,
        dst_stride as usize,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        DAV1D_FILTER_8TAP_REGULAR | (DAV1D_FILTER_8TAP_SHARP << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    prep_8tap_scaled_rust(
        tmp,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        dx as usize,
        dy as usize,
        DAV1D_FILTER_8TAP_REGULAR | (DAV1D_FILTER_8TAP_SHARP << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    prep_8tap_rust(
        tmp,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        DAV1D_FILTER_8TAP_REGULAR | (DAV1D_FILTER_8TAP_SHARP << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    put_8tap_scaled_rust(
        dst,
        dst_stride as usize,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        dx as usize,
        dy as usize,
        DAV1D_FILTER_8TAP_REGULAR | (DAV1D_FILTER_8TAP_SHARP << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    prep_8tap_scaled_rust(
        tmp,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        dx as usize,
        dy as usize,
        DAV1D_FILTER_8TAP_REGULAR | (DAV1D_FILTER_8TAP_SMOOTH << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    prep_8tap_rust(
        tmp,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        DAV1D_FILTER_8TAP_REGULAR | (DAV1D_FILTER_8TAP_SMOOTH << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    put_8tap_scaled_rust(
        dst,
        dst_stride as usize,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        dx as usize,
        dy as usize,
        DAV1D_FILTER_8TAP_REGULAR | (DAV1D_FILTER_8TAP_SMOOTH << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    put_8tap_rust(
        dst,
        dst_stride as usize,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        DAV1D_FILTER_8TAP_REGULAR | (DAV1D_FILTER_8TAP_SMOOTH << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    prep_8tap_scaled_rust(
        tmp,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        dx as usize,
        dy as usize,
        DAV1D_FILTER_8TAP_SMOOTH | (DAV1D_FILTER_8TAP_SMOOTH << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    put_8tap_rust(
        dst,
        dst_stride as usize,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        DAV1D_FILTER_8TAP_SMOOTH | (DAV1D_FILTER_8TAP_SMOOTH << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    put_8tap_scaled_rust(
        dst,
        dst_stride as usize,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        dx as usize,
        dy as usize,
        DAV1D_FILTER_8TAP_SMOOTH | (DAV1D_FILTER_8TAP_SMOOTH << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    prep_8tap_rust(
        tmp,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        DAV1D_FILTER_8TAP_SMOOTH | (DAV1D_FILTER_8TAP_SMOOTH << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    put_8tap_scaled_rust(
        dst,
        dst_stride as usize,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        dx as usize,
        dy as usize,
        DAV1D_FILTER_8TAP_SMOOTH | (DAV1D_FILTER_8TAP_REGULAR << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    put_8tap_rust(
        dst,
        dst_stride as usize,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        DAV1D_FILTER_8TAP_SMOOTH | (DAV1D_FILTER_8TAP_REGULAR << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    prep_8tap_rust(
        tmp,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        DAV1D_FILTER_8TAP_SMOOTH | (DAV1D_FILTER_8TAP_REGULAR << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    prep_8tap_scaled_rust(
        tmp,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        dx as usize,
        dy as usize,
        DAV1D_FILTER_8TAP_SMOOTH | (DAV1D_FILTER_8TAP_REGULAR << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    put_8tap_rust(
        dst,
        dst_stride as usize,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        DAV1D_FILTER_8TAP_SMOOTH | (DAV1D_FILTER_8TAP_SHARP << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    prep_8tap_scaled_rust(
        tmp,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        dx as usize,
        dy as usize,
        DAV1D_FILTER_8TAP_SMOOTH | (DAV1D_FILTER_8TAP_SHARP << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    put_8tap_scaled_rust(
        dst,
        dst_stride as usize,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        dx as usize,
        dy as usize,
        DAV1D_FILTER_8TAP_SMOOTH | (DAV1D_FILTER_8TAP_SHARP << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    prep_8tap_rust(
        tmp,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        DAV1D_FILTER_8TAP_SMOOTH | (DAV1D_FILTER_8TAP_SHARP << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    prep_8tap_rust(
        tmp,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        DAV1D_FILTER_8TAP_SHARP | (DAV1D_FILTER_8TAP_SHARP << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    put_8tap_scaled_rust(
        dst,
        dst_stride as usize,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        dx as usize,
        dy as usize,
        DAV1D_FILTER_8TAP_SHARP | (DAV1D_FILTER_8TAP_SHARP << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    put_8tap_rust(
        dst,
        dst_stride as usize,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        DAV1D_FILTER_8TAP_SHARP | (DAV1D_FILTER_8TAP_SHARP << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    prep_8tap_scaled_rust(
        tmp,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        dx as usize,
        dy as usize,
        DAV1D_FILTER_8TAP_SHARP | (DAV1D_FILTER_8TAP_SHARP << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    prep_8tap_rust(
        tmp,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        DAV1D_FILTER_8TAP_SHARP | (DAV1D_FILTER_8TAP_REGULAR << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    put_8tap_rust(
        dst,
        dst_stride as usize,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        DAV1D_FILTER_8TAP_SHARP | (DAV1D_FILTER_8TAP_REGULAR << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    prep_8tap_scaled_rust(
        tmp,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        dx as usize,
        dy as usize,
        DAV1D_FILTER_8TAP_SHARP | (DAV1D_FILTER_8TAP_REGULAR << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    put_8tap_scaled_rust(
        dst,
        dst_stride as usize,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        dx as usize,
        dy as usize,
        DAV1D_FILTER_8TAP_SHARP | (DAV1D_FILTER_8TAP_REGULAR << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    prep_8tap_rust(
        tmp,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        DAV1D_FILTER_8TAP_SHARP | (DAV1D_FILTER_8TAP_SMOOTH << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    put_8tap_scaled_rust(
        dst,
        dst_stride as usize,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        dx as usize,
        dy as usize,
        DAV1D_FILTER_8TAP_SHARP | (DAV1D_FILTER_8TAP_SMOOTH << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    put_8tap_rust(
        dst,
        dst_stride as usize,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        DAV1D_FILTER_8TAP_SHARP | (DAV1D_FILTER_8TAP_SMOOTH << 2),
        BitDepth16::new(bitdepth_max as u16),
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
    prep_8tap_scaled_rust(
        tmp,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        dx as usize,
        dy as usize,
        DAV1D_FILTER_8TAP_SHARP | (DAV1D_FILTER_8TAP_SMOOTH << 2),
        BitDepth16::new(bitdepth_max as u16),
    );
}
use crate::src::mc::put_bilin_rust;
unsafe extern "C" fn put_bilin_c(
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
    put_bilin_rust(
        dst,
        dst_stride as usize,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        BitDepth16::new(bitdepth_max as u16),
    )
}
use crate::src::mc::put_bilin_scaled_rust;
unsafe extern "C" fn put_bilin_scaled_c(
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
    put_bilin_scaled_rust(
        dst,
        dst_stride as usize,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        dx as usize,
        dy as usize,
        BitDepth16::new(bitdepth_max as u16),
    )
}
use crate::src::mc::prep_bilin_rust;
unsafe extern "C" fn prep_bilin_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_bilin_rust(
        tmp,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        BitDepth16::new(bitdepth_max as u16),
    )
}
use crate::src::mc::prep_bilin_scaled_rust;
unsafe extern "C" fn prep_bilin_scaled_c(
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
    prep_bilin_scaled_rust(
        tmp,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        dx as usize,
        dy as usize,
        BitDepth16::new(bitdepth_max as u16),
    )
}
use crate::src::mc::avg_rust;
unsafe extern "C" fn avg_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    tmp1: *const int16_t,
    tmp2: *const int16_t,
    w: libc::c_int,
    h: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    avg_rust(
        dst,
        dst_stride as usize,
        tmp1,
        tmp2,
        w as usize,
        h as usize,
        BitDepth16::new(bitdepth_max as u16),
    )
}
use crate::src::mc::w_avg_rust;
unsafe extern "C" fn w_avg_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    tmp1: *const int16_t,
    tmp2: *const int16_t,
    w: libc::c_int,
    h: libc::c_int,
    weight: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    w_avg_rust(
        dst,
        dst_stride as usize,
        tmp1,
        tmp2,
        w as usize,
        h as usize,
        weight,
        BitDepth16::new(bitdepth_max as u16),
    )
}
use crate::src::mc::mask_rust;
unsafe extern "C" fn mask_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    tmp1: *const int16_t,
    tmp2: *const int16_t,
    w: libc::c_int,
    h: libc::c_int,
    mask: *const uint8_t,
    bitdepth_max: libc::c_int,
) {
    mask_rust(
        dst,
        dst_stride as usize,
        tmp1,
        tmp2,
        w as usize,
        h as usize,
        mask,
        BitDepth16::new(bitdepth_max as u16),
    )
}
use crate::src::mc::blend_rust;
unsafe extern "C" fn blend_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    tmp: *const pixel,
    w: libc::c_int,
    h: libc::c_int,
    mask: *const uint8_t,
) {
    blend_rust::<BitDepth16>(dst, dst_stride as usize, tmp, w as usize, h as usize, mask)
}
use crate::src::mc::blend_v_rust;
unsafe extern "C" fn blend_v_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    tmp: *const pixel,
    w: libc::c_int,
    h: libc::c_int,
) {
    blend_v_rust::<BitDepth16>(dst, dst_stride as usize, tmp, w as usize, h as usize)
}
use crate::src::mc::blend_h_rust;
unsafe extern "C" fn blend_h_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    tmp: *const pixel,
    w: libc::c_int,
    h: libc::c_int,
) {
    blend_h_rust::<BitDepth16>(dst, dst_stride as usize, tmp, w as usize, h as usize)
}
use crate::src::mc::w_mask_rust;
unsafe extern "C" fn w_mask_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    tmp1: *const int16_t,
    tmp2: *const int16_t,
    w: libc::c_int,
    h: libc::c_int,
    mask: *mut uint8_t,
    sign: libc::c_int,
    ss_hor: bool,
    ss_ver: bool,
    bitdepth_max: libc::c_int,
) {
    debug_assert!(sign == 0 || sign == 1);
    w_mask_rust(
        dst,
        dst_stride as usize,
        tmp1,
        tmp2,
        w as usize,
        h as usize,
        mask,
        sign != 0,
        ss_hor,
        ss_ver,
        BitDepth16::new(bitdepth_max as u16),
    )
}
unsafe extern "C" fn w_mask_444_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    tmp1: *const int16_t,
    tmp2: *const int16_t,
    w: libc::c_int,
    h: libc::c_int,
    mask: *mut uint8_t,
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
        false,
        false,
        bitdepth_max,
    )
}
unsafe extern "C" fn w_mask_422_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    tmp1: *const int16_t,
    tmp2: *const int16_t,
    w: libc::c_int,
    h: libc::c_int,
    mask: *mut uint8_t,
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
        true,
        false,
        bitdepth_max,
    )
}
unsafe extern "C" fn w_mask_420_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    tmp1: *const int16_t,
    tmp2: *const int16_t,
    w: libc::c_int,
    h: libc::c_int,
    mask: *mut uint8_t,
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
        true,
        true,
        bitdepth_max,
    )
}
use crate::src::mc::warp_affine_8x8_rust;
unsafe extern "C" fn warp_affine_8x8_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    abcd: *const int16_t,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    warp_affine_8x8_rust(
        dst,
        dst_stride,
        src,
        src_stride,
        abcd,
        mx,
        my,
        BitDepth16::new(bitdepth_max as u16),
    )
}
use crate::src::mc::warp_affine_8x8t_rust;
unsafe extern "C" fn warp_affine_8x8t_c(
    tmp: *mut int16_t,
    tmp_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    abcd: *const int16_t,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    warp_affine_8x8t_rust(
        tmp,
        tmp_stride,
        src,
        src_stride,
        abcd,
        mx,
        my,
        BitDepth16::new(bitdepth_max as u16),
    )
}
use crate::src::mc::emu_edge_rust;
unsafe extern "C" fn emu_edge_c(
    bw: intptr_t,
    bh: intptr_t,
    iw: intptr_t,
    ih: intptr_t,
    x: intptr_t,
    y: intptr_t,
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    r#ref: *const pixel,
    ref_stride: ptrdiff_t,
) {
    emu_edge_rust::<BitDepth16>(bw, bh, iw, ih, x, y, dst, dst_stride, r#ref, ref_stride)
}
use crate::src::mc::resize_rust;
unsafe extern "C" fn resize_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    dst_w: libc::c_int,
    h: libc::c_int,
    src_w: libc::c_int,
    dx: libc::c_int,
    mx0: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    resize_rust(
        dst,
        dst_stride,
        src,
        src_stride,
        dst_w,
        h,
        src_w,
        dx,
        mx0,
        BitDepth16::new(bitdepth_max as u16),
    )
}

#[cfg(feature = "asm")]
use crate::src::cpu::dav1d_get_cpu_flags;

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
#[inline(always)]
unsafe extern "C" fn mc_dsp_init_x86(c: *mut Dav1dMCDSPContext) {
    use crate::src::x86::cpu::*;

    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_X86_CPU_FLAG_SSE2 == 0 {
        return;
    }

    if flags & DAV1D_X86_CPU_FLAG_SSSE3 == 0 {
        return;
    }

    (*c).mc[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_put_8tap_regular_16bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
        Some(dav1d_put_8tap_regular_smooth_16bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(dav1d_put_8tap_regular_sharp_16bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
        Some(dav1d_put_8tap_smooth_regular_16bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_put_8tap_smooth_16bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_put_8tap_smooth_sharp_16bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(dav1d_put_8tap_sharp_regular_16bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_put_8tap_sharp_smooth_16bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_put_8tap_sharp_16bpc_ssse3);
    (*c).mc[FILTER_2D_BILINEAR as usize] = Some(dav1d_put_bilin_16bpc_ssse3);

    (*c).mct[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_prep_8tap_regular_16bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
        Some(dav1d_prep_8tap_regular_smooth_16bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
        Some(dav1d_prep_8tap_regular_sharp_16bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
        Some(dav1d_prep_8tap_smooth_regular_16bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_prep_8tap_smooth_16bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_prep_8tap_smooth_sharp_16bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
        Some(dav1d_prep_8tap_sharp_regular_16bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_prep_8tap_sharp_smooth_16bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_prep_8tap_sharp_16bpc_ssse3);
    (*c).mct[FILTER_2D_BILINEAR as usize] = Some(dav1d_prep_bilin_16bpc_ssse3);

    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR as usize] =
        Some(dav1d_put_8tap_scaled_regular_16bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
        Some(dav1d_put_8tap_scaled_regular_smooth_16bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
        Some(dav1d_put_8tap_scaled_regular_sharp_16bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
        Some(dav1d_put_8tap_scaled_smooth_regular_16bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_put_8tap_scaled_smooth_16bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
        Some(dav1d_put_8tap_scaled_smooth_sharp_16bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
        Some(dav1d_put_8tap_scaled_sharp_regular_16bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
        Some(dav1d_put_8tap_scaled_sharp_smooth_16bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_put_8tap_scaled_sharp_16bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_BILINEAR as usize] = Some(dav1d_put_bilin_scaled_16bpc_ssse3);

    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR as usize] =
        Some(dav1d_prep_8tap_scaled_regular_16bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
        Some(dav1d_prep_8tap_scaled_regular_smooth_16bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
        Some(dav1d_prep_8tap_scaled_regular_sharp_16bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
        Some(dav1d_prep_8tap_scaled_smooth_regular_16bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH as usize] =
        Some(dav1d_prep_8tap_scaled_smooth_16bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
        Some(dav1d_prep_8tap_scaled_smooth_sharp_16bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
        Some(dav1d_prep_8tap_scaled_sharp_regular_16bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
        Some(dav1d_prep_8tap_scaled_sharp_smooth_16bpc_ssse3);
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

    #[cfg(target_arch = "x86_64")]
    {
        if flags & DAV1D_X86_CPU_FLAG_AVX2 == 0 {
            return;
        }

        (*c).mc[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_put_8tap_regular_16bpc_avx2);
        (*c).mc[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
            Some(dav1d_put_8tap_regular_smooth_16bpc_avx2);
        (*c).mc[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
            Some(dav1d_put_8tap_regular_sharp_16bpc_avx2);
        (*c).mc[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
            Some(dav1d_put_8tap_smooth_regular_16bpc_avx2);
        (*c).mc[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_put_8tap_smooth_16bpc_avx2);
        (*c).mc[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
            Some(dav1d_put_8tap_smooth_sharp_16bpc_avx2);
        (*c).mc[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
            Some(dav1d_put_8tap_sharp_regular_16bpc_avx2);
        (*c).mc[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
            Some(dav1d_put_8tap_sharp_smooth_16bpc_avx2);
        (*c).mc[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_put_8tap_sharp_16bpc_avx2);
        (*c).mc[FILTER_2D_BILINEAR as usize] = Some(dav1d_put_bilin_16bpc_avx2);

        (*c).mct[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_prep_8tap_regular_16bpc_avx2);
        (*c).mct[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
            Some(dav1d_prep_8tap_regular_smooth_16bpc_avx2);
        (*c).mct[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
            Some(dav1d_prep_8tap_regular_sharp_16bpc_avx2);
        (*c).mct[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
            Some(dav1d_prep_8tap_smooth_regular_16bpc_avx2);
        (*c).mct[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_prep_8tap_smooth_16bpc_avx2);
        (*c).mct[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
            Some(dav1d_prep_8tap_smooth_sharp_16bpc_avx2);
        (*c).mct[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
            Some(dav1d_prep_8tap_sharp_regular_16bpc_avx2);
        (*c).mct[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
            Some(dav1d_prep_8tap_sharp_smooth_16bpc_avx2);
        (*c).mct[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_prep_8tap_sharp_16bpc_avx2);
        (*c).mct[FILTER_2D_BILINEAR as usize] = Some(dav1d_prep_bilin_16bpc_avx2);

        (*c).mc_scaled[FILTER_2D_8TAP_REGULAR as usize] =
            Some(dav1d_put_8tap_scaled_regular_16bpc_avx2);
        (*c).mc_scaled[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
            Some(dav1d_put_8tap_scaled_regular_smooth_16bpc_avx2);
        (*c).mc_scaled[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
            Some(dav1d_put_8tap_scaled_regular_sharp_16bpc_avx2);
        (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
            Some(dav1d_put_8tap_scaled_smooth_regular_16bpc_avx2);
        (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH as usize] =
            Some(dav1d_put_8tap_scaled_smooth_16bpc_avx2);
        (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
            Some(dav1d_put_8tap_scaled_smooth_sharp_16bpc_avx2);
        (*c).mc_scaled[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
            Some(dav1d_put_8tap_scaled_sharp_regular_16bpc_avx2);
        (*c).mc_scaled[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
            Some(dav1d_put_8tap_scaled_sharp_smooth_16bpc_avx2);
        (*c).mc_scaled[FILTER_2D_8TAP_SHARP as usize] =
            Some(dav1d_put_8tap_scaled_sharp_16bpc_avx2);
        (*c).mc_scaled[FILTER_2D_BILINEAR as usize] = Some(dav1d_put_bilin_scaled_16bpc_avx2);

        (*c).mct_scaled[FILTER_2D_8TAP_REGULAR as usize] =
            Some(dav1d_prep_8tap_scaled_regular_16bpc_avx2);
        (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
            Some(dav1d_prep_8tap_scaled_regular_smooth_16bpc_avx2);
        (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
            Some(dav1d_prep_8tap_scaled_regular_sharp_16bpc_avx2);
        (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
            Some(dav1d_prep_8tap_scaled_smooth_regular_16bpc_avx2);
        (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH as usize] =
            Some(dav1d_prep_8tap_scaled_smooth_16bpc_avx2);
        (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
            Some(dav1d_prep_8tap_scaled_smooth_sharp_16bpc_avx2);
        (*c).mct_scaled[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
            Some(dav1d_prep_8tap_scaled_sharp_regular_16bpc_avx2);
        (*c).mct_scaled[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
            Some(dav1d_prep_8tap_scaled_sharp_smooth_16bpc_avx2);
        (*c).mct_scaled[FILTER_2D_8TAP_SHARP as usize] =
            Some(dav1d_prep_8tap_scaled_sharp_16bpc_avx2);
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
        (*c).mc[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
            Some(dav1d_put_8tap_regular_smooth_16bpc_avx512icl);
        (*c).mc[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
            Some(dav1d_put_8tap_regular_sharp_16bpc_avx512icl);
        (*c).mc[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
            Some(dav1d_put_8tap_smooth_regular_16bpc_avx512icl);
        (*c).mc[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_put_8tap_smooth_16bpc_avx512icl);
        (*c).mc[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
            Some(dav1d_put_8tap_smooth_sharp_16bpc_avx512icl);
        (*c).mc[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
            Some(dav1d_put_8tap_sharp_regular_16bpc_avx512icl);
        (*c).mc[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
            Some(dav1d_put_8tap_sharp_smooth_16bpc_avx512icl);
        (*c).mc[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_put_8tap_sharp_16bpc_avx512icl);
        (*c).mc[FILTER_2D_BILINEAR as usize] = Some(dav1d_put_bilin_16bpc_avx512icl);

        (*c).mct[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_prep_8tap_regular_16bpc_avx512icl);
        (*c).mct[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
            Some(dav1d_prep_8tap_regular_smooth_16bpc_avx512icl);
        (*c).mct[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
            Some(dav1d_prep_8tap_regular_sharp_16bpc_avx512icl);
        (*c).mct[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
            Some(dav1d_prep_8tap_smooth_regular_16bpc_avx512icl);
        (*c).mct[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_prep_8tap_smooth_16bpc_avx512icl);
        (*c).mct[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
            Some(dav1d_prep_8tap_smooth_sharp_16bpc_avx512icl);
        (*c).mct[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
            Some(dav1d_prep_8tap_sharp_regular_16bpc_avx512icl);
        (*c).mct[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
            Some(dav1d_prep_8tap_sharp_smooth_16bpc_avx512icl);
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
    (*c).mc[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
        Some(dav1d_put_8tap_regular_smooth_16bpc_neon);
    (*c).mc[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(dav1d_put_8tap_regular_sharp_16bpc_neon);
    (*c).mc[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
        Some(dav1d_put_8tap_smooth_regular_16bpc_neon);
    (*c).mc[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_put_8tap_smooth_16bpc_neon);
    (*c).mc[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_put_8tap_smooth_sharp_16bpc_neon);
    (*c).mc[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(dav1d_put_8tap_sharp_regular_16bpc_neon);
    (*c).mc[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_put_8tap_sharp_smooth_16bpc_neon);
    (*c).mc[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_put_8tap_sharp_16bpc_neon);
    (*c).mc[FILTER_2D_BILINEAR as usize] = Some(dav1d_put_bilin_16bpc_neon);

    (*c).mct[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_prep_8tap_regular_16bpc_neon);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
        Some(dav1d_prep_8tap_regular_smooth_16bpc_neon);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
        Some(dav1d_prep_8tap_regular_sharp_16bpc_neon);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
        Some(dav1d_prep_8tap_smooth_regular_16bpc_neon);
    (*c).mct[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_prep_8tap_smooth_16bpc_neon);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_prep_8tap_smooth_sharp_16bpc_neon);
    (*c).mct[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
        Some(dav1d_prep_8tap_sharp_regular_16bpc_neon);
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
    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
        Some(prep_8tap_regular_smooth_scaled_c);
    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(prep_8tap_regular_sharp_scaled_c);
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(prep_8tap_sharp_regular_scaled_c);
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(prep_8tap_sharp_smooth_scaled_c);
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP as usize] = Some(prep_8tap_sharp_scaled_c);
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
        Some(prep_8tap_smooth_regular_scaled_c);
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
