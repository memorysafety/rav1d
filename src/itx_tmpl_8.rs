use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
use cfg_if::cfg_if;
extern "C" {
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn dav1d_inv_dct4_1d_c(c: *mut int32_t, stride: ptrdiff_t, min: libc::c_int, max: libc::c_int);
    fn dav1d_inv_dct8_1d_c(c: *mut int32_t, stride: ptrdiff_t, min: libc::c_int, max: libc::c_int);
    fn dav1d_inv_dct16_1d_c(c: *mut int32_t, stride: ptrdiff_t, min: libc::c_int, max: libc::c_int);
    fn dav1d_inv_dct32_1d_c(c: *mut int32_t, stride: ptrdiff_t, min: libc::c_int, max: libc::c_int);
    fn dav1d_inv_dct64_1d_c(c: *mut int32_t, stride: ptrdiff_t, min: libc::c_int, max: libc::c_int);
    fn dav1d_inv_adst4_1d_c(c: *mut int32_t, stride: ptrdiff_t, min: libc::c_int, max: libc::c_int);
    fn dav1d_inv_adst8_1d_c(c: *mut int32_t, stride: ptrdiff_t, min: libc::c_int, max: libc::c_int);
    fn dav1d_inv_adst16_1d_c(
        c: *mut int32_t,
        stride: ptrdiff_t,
        min: libc::c_int,
        max: libc::c_int,
    );
    fn dav1d_inv_flipadst4_1d_c(
        c: *mut int32_t,
        stride: ptrdiff_t,
        min: libc::c_int,
        max: libc::c_int,
    );
    fn dav1d_inv_flipadst8_1d_c(
        c: *mut int32_t,
        stride: ptrdiff_t,
        min: libc::c_int,
        max: libc::c_int,
    );
    fn dav1d_inv_flipadst16_1d_c(
        c: *mut int32_t,
        stride: ptrdiff_t,
        min: libc::c_int,
        max: libc::c_int,
    );
    fn dav1d_inv_identity4_1d_c(
        c: *mut int32_t,
        stride: ptrdiff_t,
        min: libc::c_int,
        max: libc::c_int,
    );
    fn dav1d_inv_identity8_1d_c(
        c: *mut int32_t,
        stride: ptrdiff_t,
        min: libc::c_int,
        max: libc::c_int,
    );
    fn dav1d_inv_identity16_1d_c(
        c: *mut int32_t,
        stride: ptrdiff_t,
        min: libc::c_int,
        max: libc::c_int,
    );
    fn dav1d_inv_identity32_1d_c(
        c: *mut int32_t,
        stride: ptrdiff_t,
        min: libc::c_int,
        max: libc::c_int,
    );
    fn dav1d_inv_wht4_1d_c(c: *mut int32_t, stride: ptrdiff_t);
}

#[cfg(feature = "asm")]
extern "C" {
    static mut dav1d_cpu_flags: libc::c_uint;
    static mut dav1d_cpu_flags_mask: libc::c_uint;
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
extern "C" {
    fn dav1d_inv_txfm_add_flipadst_adst_4x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_8x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_4x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_4x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_4x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_4x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_4x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_4x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_4x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_4x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_4x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_4x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_4x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_4x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_4x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_4x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_4x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_4x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_4x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_4x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_4x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_4x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_4x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_4x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_4x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_4x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_4x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_4x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_4x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_4x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_8x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_4x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_4x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_4x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_4x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_4x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_4x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_4x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_4x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_4x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_4x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_4x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_4x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_4x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_4x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_4x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_4x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_4x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_4x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_4x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_8x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_8x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_8x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_8x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_8x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_8x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_8x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_8x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_8x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_8x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_8x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_8x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_8x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_8x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_8x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_8x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_8x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_8x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_8x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_8x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_8x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_8x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_8x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_8x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_8x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_8x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_8x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_8x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_8x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_8x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_8x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_8x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_8x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_8x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_8x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_8x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_8x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_8x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_8x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_8x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_8x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_8x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_8x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_8x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_8x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_8x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_8x32_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_8x32_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_16x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_16x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_16x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_16x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_16x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_16x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_16x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_16x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_16x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_16x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_16x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_16x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_16x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_16x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_16x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_16x4_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_16x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_16x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_16x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_16x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_16x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_16x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_16x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_16x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_16x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_16x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_16x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_16x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_16x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_16x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_16x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_16x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_16x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_16x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_16x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_16x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_16x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_16x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_16x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_16x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_16x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_16x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_16x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_16x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_16x32_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_16x32_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_16x64_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_32x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_32x8_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_32x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_32x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_32x32_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_32x32_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_32x64_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_64x64_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_64x32_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_64x16_8bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_64x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_8x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_64x64_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_16x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_32x64_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_32x32_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_32x32_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_32x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_32x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_32x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_32x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_16x64_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_16x32_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_16x32_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_16x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_16x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_16x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_16x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_16x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_16x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_16x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_16x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_16x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_16x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_16x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_16x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_16x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_16x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_16x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_16x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_16x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_16x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_16x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_16x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_16x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_16x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_16x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_16x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_16x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_16x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_16x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_16x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_16x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_16x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_16x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_16x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_16x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_16x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_16x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_16x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_16x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_16x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_16x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_16x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_16x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_4x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_8x32_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_8x32_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_8x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_8x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_8x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_8x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_8x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_8x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_8x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_8x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_8x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_8x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_8x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_8x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_8x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_8x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_8x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_8x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_8x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_8x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_8x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_4x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_64x32_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_8x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_8x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_8x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_8x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_8x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_8x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_8x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_8x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_8x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_8x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_8x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_8x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_8x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_8x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_8x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_8x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_8x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_8x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_8x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_8x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_8x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_8x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_8x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_8x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_8x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_8x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_8x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_8x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_4x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_4x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_4x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_4x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_4x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_4x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_4x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_4x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_4x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_4x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_4x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_4x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_4x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_4x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_4x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_4x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_4x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_4x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_4x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_4x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_4x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_4x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_4x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_4x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_4x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_4x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_4x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_4x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_4x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_4x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_4x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_4x8_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_4x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_4x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_4x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_4x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_4x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_4x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_4x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_4x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_4x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_4x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_4x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_16x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_4x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_4x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_16x16_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_wht_wht_4x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_4x4_8bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_8x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_8x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_8x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_8x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_8x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_8x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_8x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_8x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_8x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_8x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_8x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_8x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_8x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_16x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_16x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_16x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_16x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_16x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_16x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_16x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_16x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_16x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_16x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_16x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_16x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_16x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_16x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_16x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_16x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_16x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_16x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_16x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_16x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_16x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_16x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_16x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_16x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_16x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_16x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_16x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_16x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_8x32_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_8x32_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_32x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_32x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_16x32_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_16x32_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_32x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_32x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_32x32_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_32x32_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_16x64_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_32x64_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_64x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_4x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_8x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_8x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_16x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_16x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_16x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_16x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_16x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_16x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_16x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_16x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_16x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_16x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_16x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_16x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_16x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_16x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_16x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_16x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_4x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_4x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_4x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_4x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_4x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_4x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_4x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_64x32_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_4x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_4x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_4x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_4x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_4x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_4x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_4x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_4x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_8x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_8x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_8x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_8x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_8x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_8x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_8x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_8x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_8x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_8x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_8x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_64x64_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_8x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_8x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_8x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_8x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_8x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_8x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_8x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_8x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_8x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_8x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_8x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_8x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_8x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_8x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_8x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_8x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_8x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_8x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_8x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_8x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_4x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_4x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_4x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_4x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_4x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_8x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_4x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_4x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_4x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_4x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_4x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_4x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_4x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_4x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_4x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_4x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_4x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_4x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_4x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_4x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_4x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_4x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_4x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_4x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_4x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_4x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_4x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_4x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_4x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_4x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_4x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_4x4_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_wht_wht_4x4_8bpc_sse2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_4x8_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_8x16_8bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
extern "C" {
    fn dav1d_inv_txfm_add_identity_dct_4x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_4x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_4x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_4x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_4x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_4x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_4x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_4x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_4x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_4x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_4x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_wht_wht_4x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_4x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_4x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_4x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_4x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_4x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_4x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_4x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_4x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_4x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_4x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_4x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_4x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_4x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_4x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_4x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_4x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_4x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_4x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_4x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_4x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_4x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_4x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_4x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_4x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_4x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_4x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_4x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_4x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_4x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_4x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_4x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_4x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_4x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_4x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_4x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_4x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_4x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_8x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_8x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_8x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_8x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_8x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_8x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_8x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_8x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_8x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_8x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_8x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_8x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_8x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_8x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_8x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_8x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_8x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_8x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_8x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_8x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_8x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_8x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_8x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_8x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_8x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_8x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_8x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_8x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_8x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_8x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_8x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_8x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_8x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_8x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_8x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_8x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_8x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_8x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_8x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_8x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_8x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_8x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_8x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_8x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_8x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_8x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_8x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_8x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_8x32_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_8x32_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_16x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_16x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_16x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_16x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_16x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_16x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_16x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_16x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_16x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_16x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_16x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_16x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_16x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_16x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_16x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_16x4_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_16x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_16x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_16x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_16x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_16x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_16x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_16x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_16x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_16x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_16x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_16x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_16x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_16x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_16x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_16x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_16x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_16x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_16x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_16x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_16x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_16x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_16x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_16x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_16x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_16x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_16x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_16x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_16x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_16x32_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_16x32_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_16x64_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_32x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_32x8_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_32x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_32x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_32x32_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_32x32_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_32x64_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_64x16_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_64x32_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_64x64_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
    );
}

pub type pixel = uint8_t;
pub type coef = int16_t;

use crate::src::levels::TX_16X16;
use crate::src::levels::TX_32X32;
use crate::src::levels::TX_4X4;
use crate::src::levels::TX_64X64;
use crate::src::levels::TX_8X8;

use crate::src::levels::RTX_16X32;
use crate::src::levels::RTX_16X4;
use crate::src::levels::RTX_16X64;
use crate::src::levels::RTX_16X8;
use crate::src::levels::RTX_32X16;
use crate::src::levels::RTX_32X64;
use crate::src::levels::RTX_32X8;
use crate::src::levels::RTX_4X16;
use crate::src::levels::RTX_4X8;
use crate::src::levels::RTX_64X16;
use crate::src::levels::RTX_64X32;
use crate::src::levels::RTX_8X16;
use crate::src::levels::RTX_8X32;
use crate::src::levels::RTX_8X4;

use crate::src::levels::WHT_WHT;

use crate::src::levels::ADST_ADST;
use crate::src::levels::ADST_DCT;
use crate::src::levels::ADST_FLIPADST;
use crate::src::levels::DCT_ADST;
use crate::src::levels::DCT_DCT;
use crate::src::levels::DCT_FLIPADST;
use crate::src::levels::FLIPADST_ADST;
use crate::src::levels::FLIPADST_DCT;
use crate::src::levels::FLIPADST_FLIPADST;
use crate::src::levels::H_ADST;
use crate::src::levels::H_DCT;
use crate::src::levels::H_FLIPADST;
use crate::src::levels::IDTX;
use crate::src::levels::V_ADST;
use crate::src::levels::V_DCT;
use crate::src::levels::V_FLIPADST;
pub type itxfm_fn =
    Option<unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dInvTxfmDSPContext {
    pub itxfm_add: [[itxfm_fn; 17]; 19],
}
pub type itx_1d_fn =
    Option<unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> ()>;
use crate::include::common::intops::iclip;
use crate::include::common::intops::iclip_u8;
use crate::include::common::intops::imin;
#[inline(never)]
unsafe extern "C" fn inv_txfm_add_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
    w: libc::c_int,
    h: libc::c_int,
    shift: libc::c_int,
    first_1d_fn: itx_1d_fn,
    second_1d_fn: itx_1d_fn,
    has_dconly: libc::c_int,
) {
    if !(w >= 4 && w <= 64) {
        unreachable!();
    }
    if !(h >= 4 && h <= 64) {
        unreachable!();
    }
    if !(eob >= 0) {
        unreachable!();
    }
    let is_rect2: libc::c_int = (w * 2 == h || h * 2 == w) as libc::c_int;
    let rnd = (1 as libc::c_int) << shift >> 1;
    if eob < has_dconly {
        let mut dc = *coeff.offset(0) as libc::c_int;
        *coeff.offset(0) = 0 as libc::c_int as coef;
        if is_rect2 != 0 {
            dc = dc * 181 + 128 >> 8;
        }
        dc = dc * 181 + 128 >> 8;
        dc = dc + rnd >> shift;
        dc = dc * 181 + 128 + 2048 >> 12;
        let mut y = 0;
        while y < h {
            let mut x = 0;
            while x < w {
                *dst.offset(x as isize) =
                    iclip_u8(*dst.offset(x as isize) as libc::c_int + dc) as pixel;
                x += 1;
            }
            y += 1;
            dst = dst.offset(stride as isize);
        }
        return;
    }
    let sh = imin(h, 32 as libc::c_int);
    let sw = imin(w, 32 as libc::c_int);
    let row_clip_min = -(32767 as libc::c_int) - 1;
    let col_clip_min = -(32767 as libc::c_int) - 1;
    let row_clip_max = !row_clip_min;
    let col_clip_max = !col_clip_min;
    let mut tmp: [int32_t; 4096] = [0; 4096];
    let mut c: *mut int32_t = tmp.as_mut_ptr();
    let mut y_0 = 0;
    while y_0 < sh {
        if is_rect2 != 0 {
            let mut x_0 = 0;
            while x_0 < sw {
                *c.offset(x_0 as isize) =
                    *coeff.offset((y_0 + x_0 * sh) as isize) as libc::c_int * 181 + 128 >> 8;
                x_0 += 1;
            }
        } else {
            let mut x_1 = 0;
            while x_1 < sw {
                *c.offset(x_1 as isize) = *coeff.offset((y_0 + x_1 * sh) as isize) as int32_t;
                x_1 += 1;
            }
        }
        first_1d_fn.expect("non-null function pointer")(
            c,
            1 as libc::c_int as ptrdiff_t,
            row_clip_min,
            row_clip_max,
        );
        y_0 += 1;
        c = c.offset(w as isize);
    }
    memset(
        coeff as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<coef>() as libc::c_ulong)
            .wrapping_mul(sw as libc::c_ulong)
            .wrapping_mul(sh as libc::c_ulong),
    );
    let mut i = 0;
    while i < w * sh {
        tmp[i as usize] = iclip(tmp[i as usize] + rnd >> shift, col_clip_min, col_clip_max);
        i += 1;
    }
    let mut x_2 = 0;
    while x_2 < w {
        second_1d_fn.expect("non-null function pointer")(
            &mut *tmp.as_mut_ptr().offset(x_2 as isize),
            w as ptrdiff_t,
            col_clip_min,
            col_clip_max,
        );
        x_2 += 1;
    }
    c = tmp.as_mut_ptr();
    let mut y_1 = 0;
    while y_1 < h {
        let mut x_3 = 0;
        while x_3 < w {
            let fresh0 = c;
            c = c.offset(1);
            *dst.offset(x_3 as isize) =
                iclip_u8(*dst.offset(x_3 as isize) as libc::c_int + (*fresh0 + 8 >> 4)) as pixel;
            x_3 += 1;
        }
        y_1 += 1;
        dst = dst.offset(stride as isize);
    }
}
unsafe extern "C" fn inv_txfm_add_flipadst_flipadst_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_dct_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_adst_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_identity_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_flipadst_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_identity_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_identity_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_flipadst_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_dct_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_flipadst_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_adst_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_adst_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_dct_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_adst_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_adst_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        8 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_adst_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        8 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_identity_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        8 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_flipadst_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        8 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_identity_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        8 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_dct_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        8 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        8 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_identity_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        8 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_flipadst_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        8 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_flipadst_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        8 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_dct_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        8 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        8 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_flipadst_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        8 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_adst_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        8 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_adst_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        8 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_dct_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        8 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_adst_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_identity_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_flipadst_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_identity_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_dct_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_identity_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_flipadst_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_flipadst_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_dct_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_flipadst_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_adst_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_adst_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_adst_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_dct_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        4 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_adst_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_flipadst_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_dct_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_adst_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_adst_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_adst_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_dct_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_flipadst_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_flipadst_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_identity_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_flipadst_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_identity_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_identity_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_dct_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_adst_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_identity_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_flipadst_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_identity_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_dct_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_identity_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_flipadst_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_adst_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_flipadst_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_dct_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_flipadst_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_dct_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_adst_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_adst_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_adst_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_identity_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_dct_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_adst_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_adst_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_flipadst_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_dct_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_flipadst_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_flipadst_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_dct_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_identity_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_flipadst_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_identity_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_adst_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_8x32_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        32 as libc::c_int,
        2 as libc::c_int,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_8x32_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        8 as libc::c_int,
        32 as libc::c_int,
        2 as libc::c_int,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_adst_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_identity_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_flipadst_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_dct_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_identity_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_adst_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_dct_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_identity_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_flipadst_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_flipadst_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_dct_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_flipadst_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_adst_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_adst_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_adst_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_flipadst_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_adst_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_dct_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_adst_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_adst_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_flipadst_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_dct_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_flipadst_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_flipadst_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_identity_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_identity_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_dct_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_identity_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        8 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_dct_16x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        16 as libc::c_int,
        2 as libc::c_int,
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_flipadst_16x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        16 as libc::c_int,
        2 as libc::c_int,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_16x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        16 as libc::c_int,
        2 as libc::c_int,
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_16x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        16 as libc::c_int,
        2 as libc::c_int,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_dct_16x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        16 as libc::c_int,
        2 as libc::c_int,
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_dct_16x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        16 as libc::c_int,
        2 as libc::c_int,
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_identity_16x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        16 as libc::c_int,
        2 as libc::c_int,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_adst_16x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        16 as libc::c_int,
        2 as libc::c_int,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_adst_16x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        16 as libc::c_int,
        2 as libc::c_int,
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_flipadst_16x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        16 as libc::c_int,
        2 as libc::c_int,
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_adst_16x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        16 as libc::c_int,
        2 as libc::c_int,
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_flipadst_16x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        16 as libc::c_int,
        2 as libc::c_int,
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_16x32_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        32 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_16x32_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        32 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_16x64_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        16 as libc::c_int,
        64 as libc::c_int,
        2 as libc::c_int,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct64_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_32x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        32 as libc::c_int,
        8 as libc::c_int,
        2 as libc::c_int,
        Some(
            dav1d_inv_identity32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_32x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        32 as libc::c_int,
        8 as libc::c_int,
        2 as libc::c_int,
        Some(
            dav1d_inv_dct32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_32x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        32 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_identity32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_32x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        32 as libc::c_int,
        16 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_dct32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_32x32_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        32 as libc::c_int,
        32 as libc::c_int,
        2 as libc::c_int,
        Some(
            dav1d_inv_identity32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_32x32_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        32 as libc::c_int,
        32 as libc::c_int,
        2 as libc::c_int,
        Some(
            dav1d_inv_dct32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_32x64_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        32 as libc::c_int,
        64 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_dct32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct64_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_64x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        64 as libc::c_int,
        16 as libc::c_int,
        2 as libc::c_int,
        Some(
            dav1d_inv_dct64_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_64x32_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        64 as libc::c_int,
        32 as libc::c_int,
        1 as libc::c_int,
        Some(
            dav1d_inv_dct64_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_64x64_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        64 as libc::c_int,
        64 as libc::c_int,
        2 as libc::c_int,
        Some(
            dav1d_inv_dct64_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct64_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1 as libc::c_int,
    );
}
unsafe extern "C" fn inv_txfm_add_wht_wht_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    _eob: libc::c_int,
) {
    let mut tmp: [int32_t; 16] = [0; 16];
    let mut c: *mut int32_t = tmp.as_mut_ptr();
    let mut y = 0;
    while y < 4 {
        let mut x = 0;
        while x < 4 {
            *c.offset(x as isize) = *coeff.offset((y + x * 4) as isize) as libc::c_int >> 2;
            x += 1;
        }
        dav1d_inv_wht4_1d_c(c, 1 as libc::c_int as ptrdiff_t);
        y += 1;
        c = c.offset(4);
    }
    memset(
        coeff as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<coef>() as libc::c_ulong)
            .wrapping_mul(4 as libc::c_int as libc::c_ulong)
            .wrapping_mul(4 as libc::c_int as libc::c_ulong),
    );
    let mut x_0 = 0;
    while x_0 < 4 {
        dav1d_inv_wht4_1d_c(
            &mut *tmp.as_mut_ptr().offset(x_0 as isize),
            4 as libc::c_int as ptrdiff_t,
        );
        x_0 += 1;
    }
    c = tmp.as_mut_ptr();
    let mut y_0 = 0;
    while y_0 < 4 {
        let mut x_1 = 0;
        while x_1 < 4 {
            let fresh1 = c;
            c = c.offset(1);
            *dst.offset(x_1 as isize) =
                iclip_u8(*dst.offset(x_1 as isize) as libc::c_int + *fresh1) as pixel;
            x_1 += 1;
        }
        y_0 += 1;
        dst = dst.offset(stride as isize);
    }
}

#[cfg(feature = "asm")]
use crate::src::cpu::dav1d_get_cpu_flags;

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
#[inline(always)]
#[rustfmt::skip]
unsafe extern "C" fn itx_dsp_init_x86(c: *mut Dav1dInvTxfmDSPContext, _bpc: libc::c_int) {
    use crate::src::x86::cpu::*;

    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_X86_CPU_FLAG_SSE2 == 0 {
        return;
    }

    (*c).itxfm_add[TX_4X4 as usize][WHT_WHT as usize] = Some(dav1d_inv_txfm_add_wht_wht_4x4_8bpc_sse2);

    if flags & DAV1D_X86_CPU_FLAG_SSSE3 == 0 {
        return;
    }

    (*c).itxfm_add[TX_4X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x4_8bpc_ssse3);
    (*c).itxfm_add[TX_4X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x4_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x8_8bpc_ssse3);
    (*c).itxfm_add[TX_8X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_4X16 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_16x4_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X16 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_16x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_16x8_8bpc_ssse3);
    (*c).itxfm_add[TX_16X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x16_8bpc_ssse3);
    (*c).itxfm_add[TX_16X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x16_8bpc_ssse3);
    (*c).itxfm_add[TX_16X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x16_8bpc_ssse3);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x16_8bpc_ssse3);
    (*c).itxfm_add[TX_16X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x16_8bpc_ssse3);
    (*c).itxfm_add[TX_16X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x16_8bpc_ssse3);
    (*c).itxfm_add[TX_16X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x16_8bpc_ssse3);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x16_8bpc_ssse3);
    (*c).itxfm_add[TX_16X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x16_8bpc_ssse3);
    (*c).itxfm_add[TX_16X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x16_8bpc_ssse3);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x16_8bpc_ssse3);
    (*c).itxfm_add[TX_16X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x32_8bpc_ssse3);
    (*c).itxfm_add[RTX_8X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x32_8bpc_ssse3);
    (*c).itxfm_add[RTX_32X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_32X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x8_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x32_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x32_8bpc_ssse3);
    (*c).itxfm_add[RTX_32X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_32X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x16_8bpc_ssse3);
    (*c).itxfm_add[TX_32X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x32_8bpc_ssse3);
    (*c).itxfm_add[TX_32X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x32_8bpc_ssse3);
    (*c).itxfm_add[RTX_16X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x64_8bpc_ssse3);
    (*c).itxfm_add[RTX_32X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x64_8bpc_ssse3);
    (*c).itxfm_add[RTX_64X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x16_8bpc_ssse3);
    (*c).itxfm_add[RTX_64X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x32_8bpc_ssse3);
    (*c).itxfm_add[TX_64X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x64_8bpc_ssse3);

    if flags & DAV1D_X86_CPU_FLAG_SSE41 == 0 {
        return;
    }

    #[cfg(target_arch = "x86_64")]
    {
        if flags & DAV1D_X86_CPU_FLAG_AVX2 == 0 {
            return;
        }

        (*c).itxfm_add[TX_4X4 as usize][WHT_WHT as usize] = Some(dav1d_inv_txfm_add_wht_wht_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x4_8bpc_avx2);
        (*c).itxfm_add[TX_4X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x4_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x8_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_4X16 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x4_8bpc_avx2);
        (*c).itxfm_add[RTX_8X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x4_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x8_8bpc_avx2);
        (*c).itxfm_add[TX_8X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x8_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X16 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x16_8bpc_avx2);
        (*c).itxfm_add[RTX_8X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x32_8bpc_avx2);
        (*c).itxfm_add[RTX_8X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x32_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_16x4_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_16x8_8bpc_avx2);
        (*c).itxfm_add[RTX_16X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_16x8_8bpc_avx2);
        (*c).itxfm_add[TX_16X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x16_8bpc_avx2);
        (*c).itxfm_add[TX_16X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x16_8bpc_avx2);
        (*c).itxfm_add[TX_16X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x16_8bpc_avx2);
        (*c).itxfm_add[TX_16X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x16_8bpc_avx2);
        (*c).itxfm_add[TX_16X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x16_8bpc_avx2);
        (*c).itxfm_add[TX_16X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x16_8bpc_avx2);
        (*c).itxfm_add[TX_16X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x16_8bpc_avx2);
        (*c).itxfm_add[TX_16X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x16_8bpc_avx2);
        (*c).itxfm_add[TX_16X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x16_8bpc_avx2);
        (*c).itxfm_add[TX_16X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x16_8bpc_avx2);
        (*c).itxfm_add[TX_16X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x16_8bpc_avx2);
        (*c).itxfm_add[TX_16X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x16_8bpc_avx2);
        (*c).itxfm_add[RTX_16X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x32_8bpc_avx2);
        (*c).itxfm_add[RTX_16X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x32_8bpc_avx2);
        (*c).itxfm_add[RTX_16X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x64_8bpc_avx2);
        (*c).itxfm_add[RTX_32X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x8_8bpc_avx2);
        (*c).itxfm_add[RTX_32X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x8_8bpc_avx2);
        (*c).itxfm_add[RTX_32X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x16_8bpc_avx2);
        (*c).itxfm_add[RTX_32X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x16_8bpc_avx2);
        (*c).itxfm_add[TX_32X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x32_8bpc_avx2);
        (*c).itxfm_add[TX_32X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x32_8bpc_avx2);
        (*c).itxfm_add[RTX_32X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x64_8bpc_avx2);
        (*c).itxfm_add[RTX_64X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x16_8bpc_avx2);
        (*c).itxfm_add[RTX_64X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x32_8bpc_avx2);
        (*c).itxfm_add[TX_64X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x64_8bpc_avx2);

        if flags & DAV1D_X86_CPU_FLAG_AVX512ICL == 0 {
            return;
        }

        (*c).itxfm_add[TX_4X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_4X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_4X16 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x4_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_8X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X16 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x32_8bpc_avx512icl);
        (*c).itxfm_add[RTX_8X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x32_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_16x4_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_16x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_16x8_8bpc_avx512icl);
        (*c).itxfm_add[TX_16X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x16_8bpc_avx512icl);
        (*c).itxfm_add[TX_16X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x16_8bpc_avx512icl);
        (*c).itxfm_add[TX_16X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x16_8bpc_avx512icl);
        (*c).itxfm_add[TX_16X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x16_8bpc_avx512icl);
        (*c).itxfm_add[TX_16X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x16_8bpc_avx512icl);
        (*c).itxfm_add[TX_16X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x16_8bpc_avx512icl);
        (*c).itxfm_add[TX_16X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x16_8bpc_avx512icl);
        (*c).itxfm_add[TX_16X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x16_8bpc_avx512icl);
        (*c).itxfm_add[TX_16X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x16_8bpc_avx512icl);
        (*c).itxfm_add[TX_16X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x16_8bpc_avx512icl);
        (*c).itxfm_add[TX_16X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x16_8bpc_avx512icl);
        (*c).itxfm_add[TX_16X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x32_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x32_8bpc_avx512icl);
        (*c).itxfm_add[RTX_16X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x64_8bpc_avx512icl);
        (*c).itxfm_add[RTX_32X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_32X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x8_8bpc_avx512icl);
        (*c).itxfm_add[RTX_32X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_32X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x16_8bpc_avx512icl);
        (*c).itxfm_add[TX_32X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x32_8bpc_avx512icl);
        (*c).itxfm_add[TX_32X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x32_8bpc_avx512icl);
        (*c).itxfm_add[RTX_32X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x64_8bpc_avx512icl);
        (*c).itxfm_add[RTX_64X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x16_8bpc_avx512icl);
        (*c).itxfm_add[RTX_64X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x32_8bpc_avx512icl);
        (*c).itxfm_add[TX_64X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x64_8bpc_avx512icl);
    }
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
#[inline(always)]
#[rustfmt::skip]
unsafe extern "C" fn itx_dsp_init_arm(c: *mut Dav1dInvTxfmDSPContext, mut _bpc: libc::c_int) {
    use crate::src::arm::cpu::DAV1D_ARM_CPU_FLAG_NEON;

    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
        return;
    }

    (*c).itxfm_add[TX_4X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x4_8bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][WHT_WHT as usize] = Some(dav1d_inv_txfm_add_wht_wht_4x4_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x8_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x4_8bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x4_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x8_8bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x8_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x16_8bpc_neon);
    (*c).itxfm_add[RTX_8X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x32_8bpc_neon);
    (*c).itxfm_add[RTX_8X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x32_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_16x4_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_16x8_8bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_16x8_8bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x16_8bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x16_8bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x16_8bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x16_8bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x16_8bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x16_8bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x16_8bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x16_8bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x16_8bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x16_8bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x16_8bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x16_8bpc_neon);
    (*c).itxfm_add[RTX_16X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x32_8bpc_neon);
    (*c).itxfm_add[RTX_16X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x32_8bpc_neon);
    (*c).itxfm_add[RTX_16X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x64_8bpc_neon);
    (*c).itxfm_add[RTX_32X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x8_8bpc_neon);
    (*c).itxfm_add[RTX_32X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x8_8bpc_neon);
    (*c).itxfm_add[RTX_32X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x16_8bpc_neon);
    (*c).itxfm_add[RTX_32X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x16_8bpc_neon);
    (*c).itxfm_add[TX_32X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x32_8bpc_neon);
    (*c).itxfm_add[TX_32X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x32_8bpc_neon);
    (*c).itxfm_add[RTX_32X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x64_8bpc_neon);
    (*c).itxfm_add[RTX_64X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x16_8bpc_neon);
    (*c).itxfm_add[RTX_64X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x32_8bpc_neon);
    (*c).itxfm_add[TX_64X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x64_8bpc_neon);
}

#[no_mangle]
#[cold]
#[rustfmt::skip]
pub unsafe extern "C" fn dav1d_itx_dsp_init_8bpc(
    c: *mut Dav1dInvTxfmDSPContext,
    mut bpc: libc::c_int,
) {
    (*c).itxfm_add[TX_4X4 as usize][WHT_WHT as usize] = Some(inv_txfm_add_wht_wht_4x4_c);
    (*c).itxfm_add[TX_4X4 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_4x4_c);
    (*c).itxfm_add[TX_4X4 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_4x4_c);
    (*c).itxfm_add[TX_4X4 as usize][DCT_ADST as usize] = Some(inv_txfm_add_adst_dct_4x4_c);
    (*c).itxfm_add[TX_4X4 as usize][ADST_DCT as usize] = Some(inv_txfm_add_dct_adst_4x4_c);
    (*c).itxfm_add[TX_4X4 as usize][ADST_ADST as usize] = Some(inv_txfm_add_adst_adst_4x4_c);
    (*c).itxfm_add[TX_4X4 as usize][ADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_adst_4x4_c);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_ADST as usize] = Some(inv_txfm_add_adst_flipadst_4x4_c);
    (*c).itxfm_add[TX_4X4 as usize][DCT_FLIPADST as usize] = Some(inv_txfm_add_flipadst_dct_4x4_c);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_DCT as usize] = Some(inv_txfm_add_dct_flipadst_4x4_c);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_flipadst_4x4_c);
    (*c).itxfm_add[TX_4X4 as usize][H_DCT as usize] = Some(inv_txfm_add_dct_identity_4x4_c);
    (*c).itxfm_add[TX_4X4 as usize][V_DCT as usize] = Some(inv_txfm_add_identity_dct_4x4_c);
    (*c).itxfm_add[TX_4X4 as usize][H_FLIPADST as usize] = Some(inv_txfm_add_flipadst_identity_4x4_c);
    (*c).itxfm_add[TX_4X4 as usize][V_FLIPADST as usize] = Some(inv_txfm_add_identity_flipadst_4x4_c);
    (*c).itxfm_add[TX_4X4 as usize][H_ADST as usize] = Some(inv_txfm_add_adst_identity_4x4_c);
    (*c).itxfm_add[TX_4X4 as usize][V_ADST as usize] = Some(inv_txfm_add_identity_adst_4x4_c);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_4x8_c);
    (*c).itxfm_add[RTX_4X8 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_4x8_c);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_ADST as usize] = Some(inv_txfm_add_adst_dct_4x8_c);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_DCT as usize] = Some(inv_txfm_add_dct_adst_4x8_c);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_ADST as usize] = Some(inv_txfm_add_adst_adst_4x8_c);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_adst_4x8_c);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_ADST as usize] = Some(inv_txfm_add_adst_flipadst_4x8_c);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_FLIPADST as usize] = Some(inv_txfm_add_flipadst_dct_4x8_c);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_DCT as usize] = Some(inv_txfm_add_dct_flipadst_4x8_c);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_flipadst_4x8_c);
    (*c).itxfm_add[RTX_4X8 as usize][H_DCT as usize] = Some(inv_txfm_add_dct_identity_4x8_c);
    (*c).itxfm_add[RTX_4X8 as usize][V_DCT as usize] = Some(inv_txfm_add_identity_dct_4x8_c);
    (*c).itxfm_add[RTX_4X8 as usize][H_FLIPADST as usize] = Some(inv_txfm_add_flipadst_identity_4x8_c);
    (*c).itxfm_add[RTX_4X8 as usize][V_FLIPADST as usize] = Some(inv_txfm_add_identity_flipadst_4x8_c);
    (*c).itxfm_add[RTX_4X8 as usize][H_ADST as usize] = Some(inv_txfm_add_adst_identity_4x8_c);
    (*c).itxfm_add[RTX_4X8 as usize][V_ADST as usize] = Some(inv_txfm_add_identity_adst_4x8_c);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_4x16_c);
    (*c).itxfm_add[RTX_4X16 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_4x16_c);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_ADST as usize] = Some(inv_txfm_add_adst_dct_4x16_c);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_DCT as usize] = Some(inv_txfm_add_dct_adst_4x16_c);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_ADST as usize] = Some(inv_txfm_add_adst_adst_4x16_c);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_adst_4x16_c);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_ADST as usize] = Some(inv_txfm_add_adst_flipadst_4x16_c);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_FLIPADST as usize] = Some(inv_txfm_add_flipadst_dct_4x16_c);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_DCT as usize] = Some(inv_txfm_add_dct_flipadst_4x16_c);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_flipadst_4x16_c);
    (*c).itxfm_add[RTX_4X16 as usize][H_DCT as usize] = Some(inv_txfm_add_dct_identity_4x16_c);
    (*c).itxfm_add[RTX_4X16 as usize][V_DCT as usize] = Some(inv_txfm_add_identity_dct_4x16_c);
    (*c).itxfm_add[RTX_4X16 as usize][H_FLIPADST as usize] = Some(inv_txfm_add_flipadst_identity_4x16_c);
    (*c).itxfm_add[RTX_4X16 as usize][V_FLIPADST as usize] = Some(inv_txfm_add_identity_flipadst_4x16_c);
    (*c).itxfm_add[RTX_4X16 as usize][H_ADST as usize] = Some(inv_txfm_add_adst_identity_4x16_c);
    (*c).itxfm_add[RTX_4X16 as usize][V_ADST as usize] = Some(inv_txfm_add_identity_adst_4x16_c);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_8x4_c);
    (*c).itxfm_add[RTX_8X4 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_8x4_c);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_ADST as usize] = Some(inv_txfm_add_adst_dct_8x4_c);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_DCT as usize] = Some(inv_txfm_add_dct_adst_8x4_c);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_ADST as usize] = Some(inv_txfm_add_adst_adst_8x4_c);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_adst_8x4_c);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_ADST as usize] = Some(inv_txfm_add_adst_flipadst_8x4_c);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_FLIPADST as usize] = Some(inv_txfm_add_flipadst_dct_8x4_c);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_DCT as usize] = Some(inv_txfm_add_dct_flipadst_8x4_c);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_flipadst_8x4_c);
    (*c).itxfm_add[RTX_8X4 as usize][H_DCT as usize] = Some(inv_txfm_add_dct_identity_8x4_c);
    (*c).itxfm_add[RTX_8X4 as usize][V_DCT as usize] = Some(inv_txfm_add_identity_dct_8x4_c);
    (*c).itxfm_add[RTX_8X4 as usize][H_FLIPADST as usize] = Some(inv_txfm_add_flipadst_identity_8x4_c);
    (*c).itxfm_add[RTX_8X4 as usize][V_FLIPADST as usize] = Some(inv_txfm_add_identity_flipadst_8x4_c);
    (*c).itxfm_add[RTX_8X4 as usize][H_ADST as usize] = Some(inv_txfm_add_adst_identity_8x4_c);
    (*c).itxfm_add[RTX_8X4 as usize][V_ADST as usize] = Some(inv_txfm_add_identity_adst_8x4_c);
    (*c).itxfm_add[TX_8X8 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_8x8_c);
    (*c).itxfm_add[TX_8X8 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_8x8_c);
    (*c).itxfm_add[TX_8X8 as usize][DCT_ADST as usize] = Some(inv_txfm_add_adst_dct_8x8_c);
    (*c).itxfm_add[TX_8X8 as usize][ADST_DCT as usize] = Some(inv_txfm_add_dct_adst_8x8_c);
    (*c).itxfm_add[TX_8X8 as usize][ADST_ADST as usize] = Some(inv_txfm_add_adst_adst_8x8_c);
    (*c).itxfm_add[TX_8X8 as usize][ADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_adst_8x8_c);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_ADST as usize] = Some(inv_txfm_add_adst_flipadst_8x8_c);
    (*c).itxfm_add[TX_8X8 as usize][DCT_FLIPADST as usize] = Some(inv_txfm_add_flipadst_dct_8x8_c);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_DCT as usize] = Some(inv_txfm_add_dct_flipadst_8x8_c);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_flipadst_8x8_c);
    (*c).itxfm_add[TX_8X8 as usize][H_DCT as usize] = Some(inv_txfm_add_dct_identity_8x8_c);
    (*c).itxfm_add[TX_8X8 as usize][V_DCT as usize] = Some(inv_txfm_add_identity_dct_8x8_c);
    (*c).itxfm_add[TX_8X8 as usize][H_FLIPADST as usize] = Some(inv_txfm_add_flipadst_identity_8x8_c);
    (*c).itxfm_add[TX_8X8 as usize][V_FLIPADST as usize] = Some(inv_txfm_add_identity_flipadst_8x8_c);
    (*c).itxfm_add[TX_8X8 as usize][H_ADST as usize] = Some(inv_txfm_add_adst_identity_8x8_c);
    (*c).itxfm_add[TX_8X8 as usize][V_ADST as usize] = Some(inv_txfm_add_identity_adst_8x8_c);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_8x16_c);
    (*c).itxfm_add[RTX_8X16 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_8x16_c);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_ADST as usize] = Some(inv_txfm_add_adst_dct_8x16_c);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_DCT as usize] = Some(inv_txfm_add_dct_adst_8x16_c);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_ADST as usize] = Some(inv_txfm_add_adst_adst_8x16_c);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_adst_8x16_c);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_ADST as usize] = Some(inv_txfm_add_adst_flipadst_8x16_c);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_FLIPADST as usize] = Some(inv_txfm_add_flipadst_dct_8x16_c);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_DCT as usize] = Some(inv_txfm_add_dct_flipadst_8x16_c);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_flipadst_8x16_c);
    (*c).itxfm_add[RTX_8X16 as usize][H_DCT as usize] = Some(inv_txfm_add_dct_identity_8x16_c);
    (*c).itxfm_add[RTX_8X16 as usize][V_DCT as usize] = Some(inv_txfm_add_identity_dct_8x16_c);
    (*c).itxfm_add[RTX_8X16 as usize][H_FLIPADST as usize] = Some(inv_txfm_add_flipadst_identity_8x16_c);
    (*c).itxfm_add[RTX_8X16 as usize][V_FLIPADST as usize] = Some(inv_txfm_add_identity_flipadst_8x16_c);
    (*c).itxfm_add[RTX_8X16 as usize][H_ADST as usize] = Some(inv_txfm_add_adst_identity_8x16_c);
    (*c).itxfm_add[RTX_8X16 as usize][V_ADST as usize] = Some(inv_txfm_add_identity_adst_8x16_c);
    (*c).itxfm_add[RTX_8X32 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_8x32_c);
    (*c).itxfm_add[RTX_8X32 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_8x32_c);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_16x4_c);
    (*c).itxfm_add[RTX_16X4 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_16x4_c);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_ADST as usize] = Some(inv_txfm_add_adst_dct_16x4_c);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_DCT as usize] = Some(inv_txfm_add_dct_adst_16x4_c);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_ADST as usize] = Some(inv_txfm_add_adst_adst_16x4_c);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_adst_16x4_c);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_ADST as usize] = Some(inv_txfm_add_adst_flipadst_16x4_c);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_FLIPADST as usize] = Some(inv_txfm_add_flipadst_dct_16x4_c);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_DCT as usize] = Some(inv_txfm_add_dct_flipadst_16x4_c);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_flipadst_16x4_c);
    (*c).itxfm_add[RTX_16X4 as usize][H_DCT as usize] = Some(inv_txfm_add_dct_identity_16x4_c);
    (*c).itxfm_add[RTX_16X4 as usize][V_DCT as usize] = Some(inv_txfm_add_identity_dct_16x4_c);
    (*c).itxfm_add[RTX_16X4 as usize][H_FLIPADST as usize] = Some(inv_txfm_add_flipadst_identity_16x4_c);
    (*c).itxfm_add[RTX_16X4 as usize][V_FLIPADST as usize] = Some(inv_txfm_add_identity_flipadst_16x4_c);
    (*c).itxfm_add[RTX_16X4 as usize][H_ADST as usize] = Some(inv_txfm_add_adst_identity_16x4_c);
    (*c).itxfm_add[RTX_16X4 as usize][V_ADST as usize] = Some(inv_txfm_add_identity_adst_16x4_c);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_16x8_c);
    (*c).itxfm_add[RTX_16X8 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_16x8_c);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_ADST as usize] = Some(inv_txfm_add_adst_dct_16x8_c);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_DCT as usize] = Some(inv_txfm_add_dct_adst_16x8_c);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_ADST as usize] = Some(inv_txfm_add_adst_adst_16x8_c);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_adst_16x8_c);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_ADST as usize] = Some(inv_txfm_add_adst_flipadst_16x8_c);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_FLIPADST as usize] = Some(inv_txfm_add_flipadst_dct_16x8_c);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_DCT as usize] = Some(inv_txfm_add_dct_flipadst_16x8_c);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_flipadst_16x8_c);
    (*c).itxfm_add[RTX_16X8 as usize][H_DCT as usize] = Some(inv_txfm_add_dct_identity_16x8_c);
    (*c).itxfm_add[RTX_16X8 as usize][V_DCT as usize] = Some(inv_txfm_add_identity_dct_16x8_c);
    (*c).itxfm_add[RTX_16X8 as usize][H_FLIPADST as usize] = Some(inv_txfm_add_flipadst_identity_16x8_c);
    (*c).itxfm_add[RTX_16X8 as usize][V_FLIPADST as usize] = Some(inv_txfm_add_identity_flipadst_16x8_c);
    (*c).itxfm_add[RTX_16X8 as usize][H_ADST as usize] = Some(inv_txfm_add_adst_identity_16x8_c);
    (*c).itxfm_add[RTX_16X8 as usize][V_ADST as usize] = Some(inv_txfm_add_identity_adst_16x8_c);
    (*c).itxfm_add[TX_16X16 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_16x16_c);
    (*c).itxfm_add[TX_16X16 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_16x16_c);
    (*c).itxfm_add[TX_16X16 as usize][DCT_ADST as usize] = Some(inv_txfm_add_adst_dct_16x16_c);
    (*c).itxfm_add[TX_16X16 as usize][ADST_DCT as usize] = Some(inv_txfm_add_dct_adst_16x16_c);
    (*c).itxfm_add[TX_16X16 as usize][ADST_ADST as usize] = Some(inv_txfm_add_adst_adst_16x16_c);
    (*c).itxfm_add[TX_16X16 as usize][ADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_adst_16x16_c);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_ADST as usize] = Some(inv_txfm_add_adst_flipadst_16x16_c);
    (*c).itxfm_add[TX_16X16 as usize][DCT_FLIPADST as usize] = Some(inv_txfm_add_flipadst_dct_16x16_c);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_DCT as usize] = Some(inv_txfm_add_dct_flipadst_16x16_c);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_flipadst_16x16_c);
    (*c).itxfm_add[TX_16X16 as usize][H_DCT as usize] = Some(inv_txfm_add_dct_identity_16x16_c);
    (*c).itxfm_add[TX_16X16 as usize][V_DCT as usize] = Some(inv_txfm_add_identity_dct_16x16_c);
    (*c).itxfm_add[RTX_16X32 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_16x32_c);
    (*c).itxfm_add[RTX_16X32 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_16x32_c);
    (*c).itxfm_add[RTX_16X64 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_16x64_c);
    (*c).itxfm_add[RTX_32X8 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_32x8_c);
    (*c).itxfm_add[RTX_32X8 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_32x8_c);
    (*c).itxfm_add[RTX_32X16 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_32x16_c);
    (*c).itxfm_add[RTX_32X16 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_32x16_c);
    (*c).itxfm_add[TX_32X32 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_32x32_c);
    (*c).itxfm_add[TX_32X32 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_32x32_c);
    (*c).itxfm_add[RTX_32X64 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_32x64_c);
    (*c).itxfm_add[RTX_64X16 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_64x16_c);
    (*c).itxfm_add[RTX_64X32 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_64x32_c);
    (*c).itxfm_add[TX_64X64 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_64x64_c);

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            itx_dsp_init_x86(c, bpc);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
            itx_dsp_init_arm(c, bpc);
        }
    }
}
