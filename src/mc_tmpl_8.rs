use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::BitDepth8;
use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
#[cfg(feature = "asm")]
use cfg_if::cfg_if;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
extern "C" {
    fn dav1d_put_8tap_regular_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_regular_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_regular_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_regular_smooth_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_regular_smooth_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_regular_smooth_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_regular_sharp_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_regular_sharp_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_regular_sharp_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_smooth_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_smooth_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_smooth_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_smooth_regular_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_smooth_regular_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_smooth_regular_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_smooth_sharp_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_smooth_sharp_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_smooth_sharp_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_sharp_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_sharp_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_sharp_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_sharp_regular_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_sharp_regular_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_sharp_regular_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_sharp_smooth_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_sharp_smooth_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_sharp_smooth_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_bilin_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_bilin_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_bilin_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_8bpc_sse2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_8bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_8bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_8bpc_avx512icl(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_smooth_8bpc_sse2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_smooth_8bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_smooth_8bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_smooth_8bpc_avx512icl(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_sharp_8bpc_sse2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_sharp_8bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_sharp_8bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_sharp_8bpc_avx512icl(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_8bpc_sse2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_8bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_8bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_8bpc_avx512icl(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_regular_8bpc_avx512icl(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_regular_8bpc_sse2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_regular_8bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_regular_8bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_sharp_8bpc_avx512icl(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_sharp_8bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_sharp_8bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_sharp_8bpc_sse2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_8bpc_sse2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_8bpc_avx512icl(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_8bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_8bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_regular_8bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_regular_8bpc_avx512icl(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_regular_8bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_regular_8bpc_sse2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_smooth_8bpc_avx512icl(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_smooth_8bpc_sse2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_smooth_8bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_smooth_8bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_bilin_8bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_bilin_8bpc_sse2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_bilin_8bpc_avx512icl(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_bilin_8bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_scaled_regular_8bpc_avx2(
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
    );
    fn dav1d_put_8tap_scaled_regular_8bpc_ssse3(
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
    );
    fn dav1d_put_8tap_scaled_regular_smooth_8bpc_avx2(
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
    );
    fn dav1d_put_8tap_scaled_regular_smooth_8bpc_ssse3(
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
    );
    fn dav1d_put_8tap_scaled_regular_sharp_8bpc_avx2(
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
    );
    fn dav1d_put_8tap_scaled_regular_sharp_8bpc_ssse3(
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
    );
    fn dav1d_put_8tap_scaled_smooth_8bpc_ssse3(
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
    );
    fn dav1d_put_8tap_scaled_smooth_8bpc_avx2(
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
    );
    fn dav1d_put_8tap_scaled_smooth_regular_8bpc_ssse3(
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
    );
    fn dav1d_put_8tap_scaled_smooth_regular_8bpc_avx2(
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
    );
    fn dav1d_put_8tap_scaled_smooth_sharp_8bpc_ssse3(
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
    );
    fn dav1d_put_8tap_scaled_smooth_sharp_8bpc_avx2(
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
    );
    fn dav1d_put_8tap_scaled_sharp_8bpc_avx2(
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
    );
    fn dav1d_put_8tap_scaled_sharp_8bpc_ssse3(
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
    );
    fn dav1d_put_8tap_scaled_sharp_regular_8bpc_avx2(
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
    );
    fn dav1d_put_8tap_scaled_sharp_regular_8bpc_ssse3(
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
    );
    fn dav1d_put_8tap_scaled_sharp_smooth_8bpc_avx2(
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
    );
    fn dav1d_put_8tap_scaled_sharp_smooth_8bpc_ssse3(
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
    );
    fn dav1d_put_bilin_scaled_8bpc_avx2(
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
    );
    fn dav1d_put_bilin_scaled_8bpc_ssse3(
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
    );
    fn dav1d_prep_8tap_scaled_regular_8bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_regular_8bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_regular_smooth_8bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_regular_smooth_8bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_regular_sharp_8bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_regular_sharp_8bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_smooth_8bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_smooth_8bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_smooth_regular_8bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_smooth_regular_8bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_smooth_sharp_8bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_smooth_sharp_8bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_sharp_8bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_sharp_8bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_sharp_regular_8bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_sharp_regular_8bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_sharp_smooth_8bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
    );
    fn dav1d_prep_8tap_scaled_sharp_smooth_8bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
    );
    fn dav1d_prep_bilin_scaled_8bpc_avx2(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
    );
    fn dav1d_prep_bilin_scaled_8bpc_ssse3(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
        dx: libc::c_int,
        dy: libc::c_int,
    );
    fn dav1d_avg_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_avg_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_avg_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_w_avg_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        weight: libc::c_int,
    );
    fn dav1d_w_avg_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        weight: libc::c_int,
    );
    fn dav1d_w_avg_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        weight: libc::c_int,
    );
    fn dav1d_mask_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *const uint8_t,
    );
    fn dav1d_mask_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *const uint8_t,
    );
    fn dav1d_mask_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *const uint8_t,
    );
    fn dav1d_w_mask_420_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *mut uint8_t,
        sign: libc::c_int,
    );
    fn dav1d_w_mask_420_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *mut uint8_t,
        sign: libc::c_int,
    );
    fn dav1d_w_mask_420_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *mut uint8_t,
        sign: libc::c_int,
    );
    fn dav1d_w_mask_422_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *mut uint8_t,
        sign: libc::c_int,
    );
    fn dav1d_w_mask_422_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *mut uint8_t,
        sign: libc::c_int,
    );
    fn dav1d_w_mask_422_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *mut uint8_t,
        sign: libc::c_int,
    );
    fn dav1d_w_mask_444_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *mut uint8_t,
        sign: libc::c_int,
    );
    fn dav1d_w_mask_444_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *mut uint8_t,
        sign: libc::c_int,
    );
    fn dav1d_w_mask_444_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *mut uint8_t,
        sign: libc::c_int,
    );
    fn dav1d_blend_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
        mask: *const uint8_t,
    );
    fn dav1d_blend_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
        mask: *const uint8_t,
    );
    fn dav1d_blend_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
        mask: *const uint8_t,
    );
    fn dav1d_blend_v_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_blend_v_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_blend_v_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_blend_h_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_blend_h_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_blend_h_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_warp_affine_8x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        abcd: *const int16_t,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_warp_affine_8x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        abcd: *const int16_t,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_warp_affine_8x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        abcd: *const int16_t,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_warp_affine_8x8_8bpc_sse2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        abcd: *const int16_t,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_warp_affine_8x8_8bpc_sse4(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        abcd: *const int16_t,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_warp_affine_8x8t_8bpc_avx512icl(
        tmp: *mut int16_t,
        tmp_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        abcd: *const int16_t,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_resize_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        dst_w: libc::c_int,
        h: libc::c_int,
        src_w: libc::c_int,
        dx: libc::c_int,
        mx: libc::c_int,
    );
    fn dav1d_warp_affine_8x8t_8bpc_ssse3(
        tmp: *mut int16_t,
        tmp_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        abcd: *const int16_t,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_emu_edge_8bpc_ssse3(
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
    fn dav1d_resize_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        dst_w: libc::c_int,
        h: libc::c_int,
        src_w: libc::c_int,
        dx: libc::c_int,
        mx: libc::c_int,
    );
    fn dav1d_warp_affine_8x8t_8bpc_sse4(
        tmp: *mut int16_t,
        tmp_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        abcd: *const int16_t,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_warp_affine_8x8t_8bpc_avx2(
        tmp: *mut int16_t,
        tmp_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        abcd: *const int16_t,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_emu_edge_8bpc_avx2(
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
    fn dav1d_resize_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        dst_w: libc::c_int,
        h: libc::c_int,
        src_w: libc::c_int,
        dx: libc::c_int,
        mx: libc::c_int,
    );
    fn dav1d_warp_affine_8x8t_8bpc_sse2(
        tmp: *mut int16_t,
        tmp_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        abcd: *const int16_t,
        mx: libc::c_int,
        my: libc::c_int,
    );
}
#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
extern "C" {
    fn dav1d_put_8tap_regular_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_regular_smooth_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_regular_sharp_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_smooth_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_smooth_regular_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_smooth_sharp_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_sharp_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_sharp_regular_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_8tap_sharp_smooth_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_put_bilin_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_8bpc_neon(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_smooth_8bpc_neon(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_regular_sharp_8bpc_neon(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_regular_8bpc_neon(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_8bpc_neon(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_smooth_sharp_8bpc_neon(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_regular_8bpc_neon(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_smooth_8bpc_neon(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_8tap_sharp_8bpc_neon(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_prep_bilin_8bpc_neon(
        tmp: *mut int16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_avg_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_w_avg_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        weight: libc::c_int,
    );
    fn dav1d_mask_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *const uint8_t,
    );
    fn dav1d_blend_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
        mask: *const uint8_t,
    );
    fn dav1d_blend_h_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_blend_v_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_w_mask_444_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *mut uint8_t,
        sign: libc::c_int,
    );
    fn dav1d_w_mask_422_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *mut uint8_t,
        sign: libc::c_int,
    );
    fn dav1d_w_mask_420_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp1: *const int16_t,
        tmp2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        mask: *mut uint8_t,
        sign: libc::c_int,
    );
    fn dav1d_warp_affine_8x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        abcd: *const int16_t,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_warp_affine_8x8t_8bpc_neon(
        tmp: *mut int16_t,
        tmp_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        abcd: *const int16_t,
        mx: libc::c_int,
        my: libc::c_int,
    );
    fn dav1d_emu_edge_8bpc_neon(
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

use crate::src::tables::dav1d_resize_filter;

pub type pixel = uint8_t;

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
use crate::include::common::intops::iclip;
use crate::include::common::intops::iclip_u8;
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        DAV1D_FILTER_8TAP_SHARP | (DAV1D_FILTER_8TAP_SMOOTH) << 2,
        BitDepth8::new(()),
    );
}
use crate::src::mc::put_bilin_rust;
unsafe extern "C" fn put_bilin_c(
    mut dst: *mut pixel,
    mut dst_stride: ptrdiff_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: libc::c_int,
    mut h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
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
        BitDepth8::new(()),
    )
}
use crate::src::mc::put_bilin_scaled_rust;
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
        BitDepth8::new(()),
    )
}
use crate::src::mc::prep_bilin_rust;
unsafe extern "C" fn prep_bilin_c(
    mut tmp: *mut int16_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: libc::c_int,
    mut h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
) {
    prep_bilin_rust(
        tmp,
        src,
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        BitDepth8::new(()),
    )
}
use crate::src::mc::prep_bilin_scaled_rust;
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
        BitDepth8::new(()),
    )
}
use crate::src::mc::avg_rust;
unsafe extern "C" fn avg_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut tmp1: *const int16_t,
    mut tmp2: *const int16_t,
    w: libc::c_int,
    mut h: libc::c_int,
) {
    avg_rust(
        dst,
        dst_stride as usize,
        tmp1,
        tmp2,
        w as usize,
        h as usize,
        BitDepth8::new(()),
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
) {
    w_avg_rust(
        dst,
        dst_stride as usize,
        tmp1,
        tmp2,
        w as usize,
        h as usize,
        weight,
        BitDepth8::new(()),
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
) {
    mask_rust(
        dst,
        dst_stride as usize,
        tmp1,
        tmp2,
        w as usize,
        h as usize,
        mask,
        BitDepth8::new(()),
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
    blend_rust::<BitDepth8>(dst, dst_stride as usize, tmp, w as usize, h as usize, mask)
}
use crate::src::mc::blend_v_rust;
unsafe extern "C" fn blend_v_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    tmp: *const pixel,
    w: libc::c_int,
    h: libc::c_int,
) {
    blend_v_rust::<BitDepth8>(dst, dst_stride as usize, tmp, w as usize, h as usize)
}
use crate::src::mc::blend_h_rust;
unsafe extern "C" fn blend_h_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    tmp: *const pixel,
    w: libc::c_int,
    h: libc::c_int,
) {
    blend_h_rust::<BitDepth8>(dst, dst_stride as usize, tmp, w as usize, h as usize)
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
        BitDepth8::new(()),
    )
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
) {
    w_mask_c(dst, dst_stride, tmp1, tmp2, w, h, mask, sign, false, false)
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
) {
    w_mask_c(dst, dst_stride, tmp1, tmp2, w, h, mask, sign, true, false)
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
) {
    w_mask_c(dst, dst_stride, tmp1, tmp2, w, h, mask, sign, true, true)
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
) {
    warp_affine_8x8_rust(
        dst,
        dst_stride,
        src,
        src_stride,
        abcd,
        mx,
        my,
        BitDepth8::new(()),
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
) {
    warp_affine_8x8t_rust(
        tmp,
        tmp_stride,
        src,
        src_stride,
        abcd,
        mx,
        my,
        BitDepth8::new(()),
    )
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
    r#ref = r#ref.offset(
        (iclip(y as libc::c_int, 0 as libc::c_int, ih as libc::c_int - 1) as isize * ref_stride
            + iclip(x as libc::c_int, 0 as libc::c_int, iw as libc::c_int - 1) as isize)
            as isize,
    );
    let left_ext = iclip(-x as libc::c_int, 0 as libc::c_int, bw as libc::c_int - 1);
    let right_ext = iclip(
        (x + bw - iw) as libc::c_int,
        0 as libc::c_int,
        bw as libc::c_int - 1,
    );
    if !(((left_ext + right_ext) as isize) < bw) {
        unreachable!();
    }
    let top_ext = iclip(-y as libc::c_int, 0 as libc::c_int, bh as libc::c_int - 1);
    let bottom_ext = iclip(
        (y + bh - ih) as libc::c_int,
        0 as libc::c_int,
        bh as libc::c_int - 1,
    );
    if !(((top_ext + bottom_ext) as isize) < bh) {
        unreachable!();
    }
    let mut blk: *mut pixel = dst.offset((top_ext as isize * dst_stride) as isize);
    let center_w = (bw - left_ext as isize - right_ext as isize) as libc::c_int;
    let center_h = (bh - top_ext as isize - bottom_ext as isize) as libc::c_int;
    let mut y_0 = 0;
    while y_0 < center_h {
        memcpy(
            blk.offset(left_ext as isize) as *mut libc::c_void,
            r#ref as *const libc::c_void,
            center_w as libc::c_ulong,
        );
        if left_ext != 0 {
            memset(
                blk as *mut libc::c_void,
                *blk.offset(left_ext as isize) as libc::c_int,
                left_ext as libc::c_ulong,
            );
        }
        if right_ext != 0 {
            memset(
                blk.offset(left_ext as isize).offset(center_w as isize) as *mut libc::c_void,
                *blk.offset((left_ext + center_w - 1) as isize) as libc::c_int,
                right_ext as libc::c_ulong,
            );
        }
        r#ref = r#ref.offset(ref_stride as isize);
        blk = blk.offset(dst_stride as isize);
        y_0 += 1;
    }
    blk = dst.offset((top_ext as isize * dst_stride) as isize);
    let mut y_1 = 0;
    while y_1 < top_ext {
        memcpy(
            dst as *mut libc::c_void,
            blk as *const libc::c_void,
            bw as libc::c_ulong,
        );
        dst = dst.offset(dst_stride as isize);
        y_1 += 1;
    }
    dst = dst.offset((center_h as isize * dst_stride) as isize);
    let mut y_2 = 0;
    while y_2 < bottom_ext {
        memcpy(
            dst as *mut libc::c_void,
            &mut *dst.offset(-dst_stride as isize) as *mut pixel as *const libc::c_void,
            bw as libc::c_ulong,
        );
        dst = dst.offset(dst_stride as isize);
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
) {
    loop {
        let mut mx = mx0;
        let mut src_x = -(1 as libc::c_int);
        let mut x = 0;
        while x < dst_w {
            let F: *const int8_t = (dav1d_resize_filter[(mx >> 8) as usize]).as_ptr();
            *dst.offset(x as isize) = iclip_u8(
                -(*F.offset(0) as libc::c_int
                    * *src.offset(iclip(src_x - 3, 0 as libc::c_int, src_w - 1) as isize)
                        as libc::c_int
                    + *F.offset(1) as libc::c_int
                        * *src.offset(iclip(src_x - 2, 0 as libc::c_int, src_w - 1) as isize)
                            as libc::c_int
                    + *F.offset(2) as libc::c_int
                        * *src.offset(iclip(src_x - 1, 0 as libc::c_int, src_w - 1) as isize)
                            as libc::c_int
                    + *F.offset(3) as libc::c_int
                        * *src.offset(iclip(src_x + 0, 0 as libc::c_int, src_w - 1) as isize)
                            as libc::c_int
                    + *F.offset(4) as libc::c_int
                        * *src.offset(iclip(src_x + 1, 0 as libc::c_int, src_w - 1) as isize)
                            as libc::c_int
                    + *F.offset(5) as libc::c_int
                        * *src.offset(iclip(src_x + 2, 0 as libc::c_int, src_w - 1) as isize)
                            as libc::c_int
                    + *F.offset(6) as libc::c_int
                        * *src.offset(iclip(src_x + 3, 0 as libc::c_int, src_w - 1) as isize)
                            as libc::c_int
                    + *F.offset(7) as libc::c_int
                        * *src.offset(iclip(src_x + 4, 0 as libc::c_int, src_w - 1) as isize)
                            as libc::c_int)
                    + 64
                    >> 7,
            ) as pixel;
            mx += dx;
            src_x += mx >> 14;
            mx &= 0x3fff as libc::c_int;
            x += 1;
        }
        dst = dst.offset(dst_stride as isize);
        src = src.offset(src_stride as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    }
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
#[inline(always)]
unsafe extern "C" fn mc_dsp_init_x86(c: *mut Dav1dMCDSPContext) {
    use crate::src::x86::cpu::*;

    let flags: libc::c_uint = dav1d_get_cpu_flags();

    if flags & DAV1D_X86_CPU_FLAG_SSE2 == 0 {
        return;
    }

    (*c).mct[FILTER_2D_BILINEAR as usize] = Some(dav1d_prep_bilin_8bpc_sse2);
    (*c).mct[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_prep_8tap_regular_8bpc_sse2);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
        Some(dav1d_prep_8tap_regular_smooth_8bpc_sse2);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(dav1d_prep_8tap_regular_sharp_8bpc_sse2);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
        Some(dav1d_prep_8tap_smooth_regular_8bpc_sse2);
    (*c).mct[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_prep_8tap_smooth_8bpc_sse2);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_prep_8tap_smooth_sharp_8bpc_sse2);
    (*c).mct[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(dav1d_prep_8tap_sharp_regular_8bpc_sse2);
    (*c).mct[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_prep_8tap_sharp_smooth_8bpc_sse2);
    (*c).mct[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_prep_8tap_sharp_8bpc_sse2);

    (*c).warp8x8 = Some(dav1d_warp_affine_8x8_8bpc_sse2);
    (*c).warp8x8t = Some(dav1d_warp_affine_8x8t_8bpc_sse2);

    if flags & DAV1D_X86_CPU_FLAG_SSSE3 == 0 {
        return;
    }

    (*c).mc[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_put_8tap_regular_8bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
        Some(dav1d_put_8tap_regular_smooth_8bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(dav1d_put_8tap_regular_sharp_8bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
        Some(dav1d_put_8tap_smooth_regular_8bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_put_8tap_smooth_8bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_put_8tap_smooth_sharp_8bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(dav1d_put_8tap_sharp_regular_8bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_put_8tap_sharp_smooth_8bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_put_8tap_sharp_8bpc_ssse3);
    (*c).mc[FILTER_2D_BILINEAR as usize] = Some(dav1d_put_bilin_8bpc_ssse3);

    (*c).mct[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_prep_8tap_regular_8bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
        Some(dav1d_prep_8tap_regular_smooth_8bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
        Some(dav1d_prep_8tap_regular_sharp_8bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
        Some(dav1d_prep_8tap_smooth_regular_8bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_prep_8tap_smooth_8bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_prep_8tap_smooth_sharp_8bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
        Some(dav1d_prep_8tap_sharp_regular_8bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_prep_8tap_sharp_smooth_8bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_prep_8tap_sharp_8bpc_ssse3);
    (*c).mct[FILTER_2D_BILINEAR as usize] = Some(dav1d_prep_bilin_8bpc_ssse3);

    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR as usize] =
        Some(dav1d_put_8tap_scaled_regular_8bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
        Some(dav1d_put_8tap_scaled_regular_smooth_8bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
        Some(dav1d_put_8tap_scaled_regular_sharp_8bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
        Some(dav1d_put_8tap_scaled_smooth_regular_8bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_put_8tap_scaled_smooth_8bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
        Some(dav1d_put_8tap_scaled_smooth_sharp_8bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
        Some(dav1d_put_8tap_scaled_sharp_regular_8bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
        Some(dav1d_put_8tap_scaled_sharp_smooth_8bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_put_8tap_scaled_sharp_8bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_BILINEAR as usize] = Some(dav1d_put_bilin_scaled_8bpc_ssse3);

    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR as usize] =
        Some(dav1d_prep_8tap_scaled_regular_8bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
        Some(dav1d_prep_8tap_scaled_regular_smooth_8bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
        Some(dav1d_prep_8tap_scaled_regular_sharp_8bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
        Some(dav1d_prep_8tap_scaled_smooth_regular_8bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH as usize] =
        Some(dav1d_prep_8tap_scaled_smooth_8bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
        Some(dav1d_prep_8tap_scaled_smooth_sharp_8bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
        Some(dav1d_prep_8tap_scaled_sharp_regular_8bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
        Some(dav1d_prep_8tap_scaled_sharp_smooth_8bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_prep_8tap_scaled_sharp_8bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_BILINEAR as usize] = Some(dav1d_prep_bilin_scaled_8bpc_ssse3);

    (*c).avg = Some(dav1d_avg_8bpc_ssse3);
    (*c).w_avg = Some(dav1d_w_avg_8bpc_ssse3);
    (*c).mask = Some(dav1d_mask_8bpc_ssse3);

    (*c).w_mask[0] = Some(dav1d_w_mask_444_8bpc_ssse3);
    (*c).w_mask[1] = Some(dav1d_w_mask_422_8bpc_ssse3);
    (*c).w_mask[2] = Some(dav1d_w_mask_420_8bpc_ssse3);

    (*c).blend = Some(dav1d_blend_8bpc_ssse3);
    (*c).blend_v = Some(dav1d_blend_v_8bpc_ssse3);
    (*c).blend_h = Some(dav1d_blend_h_8bpc_ssse3);
    (*c).warp8x8 = Some(dav1d_warp_affine_8x8_8bpc_ssse3);
    (*c).warp8x8t = Some(dav1d_warp_affine_8x8t_8bpc_ssse3);
    (*c).emu_edge = Some(dav1d_emu_edge_8bpc_ssse3);
    (*c).resize = Some(dav1d_resize_8bpc_ssse3);

    if flags & DAV1D_X86_CPU_FLAG_SSE41 == 0 {
        return;
    }

    (*c).warp8x8 = Some(dav1d_warp_affine_8x8_8bpc_sse4);
    (*c).warp8x8t = Some(dav1d_warp_affine_8x8t_8bpc_sse4);

    #[cfg(target_arch = "x86_64")]
    {
        if flags & DAV1D_X86_CPU_FLAG_AVX2 == 0 {
            return;
        }

        (*c).mc[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_put_8tap_regular_8bpc_avx2);
        (*c).mc[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
            Some(dav1d_put_8tap_regular_smooth_8bpc_avx2);
        (*c).mc[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
            Some(dav1d_put_8tap_regular_sharp_8bpc_avx2);
        (*c).mc[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
            Some(dav1d_put_8tap_smooth_regular_8bpc_avx2);
        (*c).mc[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_put_8tap_smooth_8bpc_avx2);
        (*c).mc[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_put_8tap_smooth_sharp_8bpc_avx2);
        (*c).mc[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
            Some(dav1d_put_8tap_sharp_regular_8bpc_avx2);
        (*c).mc[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_put_8tap_sharp_smooth_8bpc_avx2);
        (*c).mc[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_put_8tap_sharp_8bpc_avx2);
        (*c).mc[FILTER_2D_BILINEAR as usize] = Some(dav1d_put_bilin_8bpc_avx2);

        (*c).mct[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_prep_8tap_regular_8bpc_avx2);
        (*c).mct[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
            Some(dav1d_prep_8tap_regular_smooth_8bpc_avx2);
        (*c).mct[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
            Some(dav1d_prep_8tap_regular_sharp_8bpc_avx2);
        (*c).mct[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
            Some(dav1d_prep_8tap_smooth_regular_8bpc_avx2);
        (*c).mct[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_prep_8tap_smooth_8bpc_avx2);
        (*c).mct[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
            Some(dav1d_prep_8tap_smooth_sharp_8bpc_avx2);
        (*c).mct[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
            Some(dav1d_prep_8tap_sharp_regular_8bpc_avx2);
        (*c).mct[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
            Some(dav1d_prep_8tap_sharp_smooth_8bpc_avx2);
        (*c).mct[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_prep_8tap_sharp_8bpc_avx2);
        (*c).mct[FILTER_2D_BILINEAR as usize] = Some(dav1d_prep_bilin_8bpc_avx2);

        (*c).mc_scaled[FILTER_2D_8TAP_REGULAR as usize] =
            Some(dav1d_put_8tap_scaled_regular_8bpc_avx2);
        (*c).mc_scaled[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
            Some(dav1d_put_8tap_scaled_regular_smooth_8bpc_avx2);
        (*c).mc_scaled[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
            Some(dav1d_put_8tap_scaled_regular_sharp_8bpc_avx2);
        (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
            Some(dav1d_put_8tap_scaled_smooth_regular_8bpc_avx2);
        (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH as usize] =
            Some(dav1d_put_8tap_scaled_smooth_8bpc_avx2);
        (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
            Some(dav1d_put_8tap_scaled_smooth_sharp_8bpc_avx2);
        (*c).mc_scaled[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
            Some(dav1d_put_8tap_scaled_sharp_regular_8bpc_avx2);
        (*c).mc_scaled[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
            Some(dav1d_put_8tap_scaled_sharp_smooth_8bpc_avx2);
        (*c).mc_scaled[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_put_8tap_scaled_sharp_8bpc_avx2);
        (*c).mc_scaled[FILTER_2D_BILINEAR as usize] = Some(dav1d_put_bilin_scaled_8bpc_avx2);

        (*c).mct_scaled[FILTER_2D_8TAP_REGULAR as usize] =
            Some(dav1d_prep_8tap_scaled_regular_8bpc_avx2);
        (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
            Some(dav1d_prep_8tap_scaled_regular_smooth_8bpc_avx2);
        (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
            Some(dav1d_prep_8tap_scaled_regular_sharp_8bpc_avx2);
        (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
            Some(dav1d_prep_8tap_scaled_smooth_regular_8bpc_avx2);
        (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH as usize] =
            Some(dav1d_prep_8tap_scaled_smooth_8bpc_avx2);
        (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
            Some(dav1d_prep_8tap_scaled_smooth_sharp_8bpc_avx2);
        (*c).mct_scaled[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
            Some(dav1d_prep_8tap_scaled_sharp_regular_8bpc_avx2);
        (*c).mct_scaled[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
            Some(dav1d_prep_8tap_scaled_sharp_smooth_8bpc_avx2);
        (*c).mct_scaled[FILTER_2D_8TAP_SHARP as usize] =
            Some(dav1d_prep_8tap_scaled_sharp_8bpc_avx2);
        (*c).mct_scaled[FILTER_2D_BILINEAR as usize] = Some(dav1d_prep_bilin_scaled_8bpc_avx2);

        (*c).avg = Some(dav1d_avg_8bpc_avx2);
        (*c).w_avg = Some(dav1d_w_avg_8bpc_avx2);
        (*c).mask = Some(dav1d_mask_8bpc_avx2);

        (*c).w_mask[0] = Some(dav1d_w_mask_444_8bpc_avx2);
        (*c).w_mask[1] = Some(dav1d_w_mask_422_8bpc_avx2);
        (*c).w_mask[2] = Some(dav1d_w_mask_420_8bpc_avx2);

        (*c).blend = Some(dav1d_blend_8bpc_avx2);
        (*c).blend_v = Some(dav1d_blend_v_8bpc_avx2);
        (*c).blend_h = Some(dav1d_blend_h_8bpc_avx2);
        (*c).warp8x8 = Some(dav1d_warp_affine_8x8_8bpc_avx2);
        (*c).warp8x8t = Some(dav1d_warp_affine_8x8t_8bpc_avx2);
        (*c).emu_edge = Some(dav1d_emu_edge_8bpc_avx2);
        (*c).resize = Some(dav1d_resize_8bpc_avx2);

        if flags & DAV1D_X86_CPU_FLAG_AVX512ICL == 0 {
            return;
        }

        (*c).mc[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_put_8tap_regular_8bpc_avx512icl);
        (*c).mc[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
            Some(dav1d_put_8tap_regular_smooth_8bpc_avx512icl);
        (*c).mc[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
            Some(dav1d_put_8tap_regular_sharp_8bpc_avx512icl);
        (*c).mc[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
            Some(dav1d_put_8tap_smooth_regular_8bpc_avx512icl);
        (*c).mc[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_put_8tap_smooth_8bpc_avx512icl);
        (*c).mc[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
            Some(dav1d_put_8tap_smooth_sharp_8bpc_avx512icl);
        (*c).mc[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
            Some(dav1d_put_8tap_sharp_regular_8bpc_avx512icl);
        (*c).mc[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
            Some(dav1d_put_8tap_sharp_smooth_8bpc_avx512icl);
        (*c).mc[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_put_8tap_sharp_8bpc_avx512icl);
        (*c).mc[FILTER_2D_BILINEAR as usize] = Some(dav1d_put_bilin_8bpc_avx512icl);

        (*c).mct[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_prep_8tap_regular_8bpc_avx512icl);
        (*c).mct[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
            Some(dav1d_prep_8tap_regular_smooth_8bpc_avx512icl);
        (*c).mct[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
            Some(dav1d_prep_8tap_regular_sharp_8bpc_avx512icl);
        (*c).mct[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
            Some(dav1d_prep_8tap_smooth_regular_8bpc_avx512icl);
        (*c).mct[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_prep_8tap_smooth_8bpc_avx512icl);
        (*c).mct[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
            Some(dav1d_prep_8tap_smooth_sharp_8bpc_avx512icl);
        (*c).mct[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
            Some(dav1d_prep_8tap_sharp_regular_8bpc_avx512icl);
        (*c).mct[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
            Some(dav1d_prep_8tap_sharp_smooth_8bpc_avx512icl);
        (*c).mct[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_prep_8tap_sharp_8bpc_avx512icl);
        (*c).mct[FILTER_2D_BILINEAR as usize] = Some(dav1d_prep_bilin_8bpc_avx512icl);

        (*c).avg = Some(dav1d_avg_8bpc_avx512icl);
        (*c).w_avg = Some(dav1d_w_avg_8bpc_avx512icl);
        (*c).mask = Some(dav1d_mask_8bpc_avx512icl);

        (*c).w_mask[0] = Some(dav1d_w_mask_444_8bpc_avx512icl);
        (*c).w_mask[1] = Some(dav1d_w_mask_422_8bpc_avx512icl);
        (*c).w_mask[2] = Some(dav1d_w_mask_420_8bpc_avx512icl);

        (*c).blend = Some(dav1d_blend_8bpc_avx512icl);
        (*c).blend_v = Some(dav1d_blend_v_8bpc_avx512icl);
        (*c).blend_h = Some(dav1d_blend_h_8bpc_avx512icl);
        (*c).warp8x8 = Some(dav1d_warp_affine_8x8_8bpc_avx512icl);
        (*c).warp8x8t = Some(dav1d_warp_affine_8x8t_8bpc_avx512icl);
        (*c).resize = Some(dav1d_resize_8bpc_avx512icl);
    }
}

#[cfg(feature = "asm")]
use crate::src::cpu::dav1d_get_cpu_flags;

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
#[inline(always)]
unsafe extern "C" fn mc_dsp_init_arm(c: *mut Dav1dMCDSPContext) {
    use crate::src::arm::cpu::DAV1D_ARM_CPU_FLAG_NEON;

    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
        return;
    }

    (*c).mc[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_put_8tap_regular_8bpc_neon);
    (*c).mc[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] = Some(dav1d_put_8tap_regular_smooth_8bpc_neon);
    (*c).mc[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(dav1d_put_8tap_regular_sharp_8bpc_neon);
    (*c).mc[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] = Some(dav1d_put_8tap_smooth_regular_8bpc_neon);
    (*c).mc[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_put_8tap_smooth_8bpc_neon);
    (*c).mc[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_put_8tap_smooth_sharp_8bpc_neon);
    (*c).mc[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(dav1d_put_8tap_sharp_regular_8bpc_neon);
    (*c).mc[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_put_8tap_sharp_smooth_8bpc_neon);
    (*c).mc[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_put_8tap_sharp_8bpc_neon);
    (*c).mc[FILTER_2D_BILINEAR as usize] = Some(dav1d_put_bilin_8bpc_neon);

    (*c).mct[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_prep_8tap_regular_8bpc_neon);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
        Some(dav1d_prep_8tap_regular_smooth_8bpc_neon);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(dav1d_prep_8tap_regular_sharp_8bpc_neon);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
        Some(dav1d_prep_8tap_smooth_regular_8bpc_neon);
    (*c).mct[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_prep_8tap_smooth_8bpc_neon);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_prep_8tap_smooth_sharp_8bpc_neon);
    (*c).mct[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(dav1d_prep_8tap_sharp_regular_8bpc_neon);
    (*c).mct[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_prep_8tap_sharp_smooth_8bpc_neon);
    (*c).mct[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_prep_8tap_sharp_8bpc_neon);
    (*c).mct[FILTER_2D_BILINEAR as usize] = Some(dav1d_prep_bilin_8bpc_neon);

    (*c).avg = Some(dav1d_avg_8bpc_neon);
    (*c).w_avg = Some(dav1d_w_avg_8bpc_neon);
    (*c).mask = Some(dav1d_mask_8bpc_neon);
    (*c).blend = Some(dav1d_blend_8bpc_neon);
    (*c).blend_h = Some(dav1d_blend_h_8bpc_neon);
    (*c).blend_v = Some(dav1d_blend_v_8bpc_neon);

    (*c).w_mask[0] = Some(dav1d_w_mask_444_8bpc_neon);
    (*c).w_mask[1] = Some(dav1d_w_mask_422_8bpc_neon);
    (*c).w_mask[2] = Some(dav1d_w_mask_420_8bpc_neon);

    (*c).warp8x8 = Some(dav1d_warp_affine_8x8_8bpc_neon);
    (*c).warp8x8t = Some(dav1d_warp_affine_8x8t_8bpc_neon);
    (*c).emu_edge = Some(dav1d_emu_edge_8bpc_neon);
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_mc_dsp_init_8bpc(c: *mut Dav1dMCDSPContext) {
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
