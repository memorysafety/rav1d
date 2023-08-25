use crate::include::common::bitdepth::DynCoef;
use crate::include::common::bitdepth::DynPixel;
use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
#[cfg(feature = "asm")]
use cfg_if::cfg_if;
extern "C" {
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
extern "C" {
    fn dav1d_inv_txfm_add_flipadst_dct_4x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_4x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_4x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_4x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_4x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_4x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_4x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_4x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_4x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_4x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_4x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_wht_wht_4x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_4x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_4x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_4x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_4x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_4x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_4x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_4x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_4x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_4x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_4x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_4x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_4x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_4x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_4x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_4x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_4x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_4x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_4x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_4x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_4x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_4x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_4x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_4x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_4x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_4x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_4x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_4x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_4x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_4x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_4x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_4x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_4x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_4x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_4x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_4x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_4x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_4x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_8x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_8x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_8x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_8x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_8x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_8x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_8x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_8x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_8x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_8x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_8x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_8x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_8x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_8x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_8x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_8x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_8x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_8x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_8x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_8x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_8x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_8x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_8x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_8x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_8x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_8x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_8x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_8x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_8x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_8x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_8x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_8x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_8x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_8x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_8x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_8x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_8x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_8x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_8x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_8x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_8x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_8x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_8x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_8x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_8x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_8x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_8x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_8x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_8x32_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_8x32_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_16x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_16x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_16x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_16x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_16x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_16x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_16x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_16x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_16x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_16x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_16x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_16x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_16x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_16x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_16x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_16x4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_flipadst_16x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_identity_16x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_adst_16x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_16x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_16x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_16x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_16x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_16x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_16x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_16x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_16x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_16x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_16x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_16x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_16x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_identity_16x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_flipadst_16x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_16x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_16x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_adst_16x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_flipadst_16x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_identity_16x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_dct_16x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_adst_16x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_adst_flipadst_16x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_dct_16x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_flipadst_adst_16x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_dct_16x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_16x32_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_16x32_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_16x64_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_32x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_32x8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_32x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_32x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_32x32_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_identity_identity_32x32_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_32x64_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_64x16_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_64x32_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_inv_txfm_add_dct_dct_64x64_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        coeff: *mut coef,
        eob: libc::c_int,
        bitdepth_max: libc::c_int,
    );
}

pub type pixel = uint16_t;
pub type coef = int32_t;

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

use crate::src::itx::Dav1dInvTxfmDSPContext;
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
pub type itx_1d_fn =
    Option<unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> ()>;
use crate::include::common::intops::iclip;
#[inline]
unsafe extern "C" fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 != 0 {
        unreachable!();
    }
    return x >> 1;
}
use crate::include::common::bitdepth::BitDepth16;

unsafe extern "C" fn inv_txfm_add_wht_wht_4x4_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    coeff: *mut DynCoef,
    eob: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    inv_txfm_add_wht_wht_4x4_rust(dst.cast(), stride, coeff.cast(), eob, bitdepth_max);
}

unsafe extern "C" fn inv_txfm_add_wht_wht_4x4_rust(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    _eob: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    use crate::src::itx_1d::dav1d_inv_wht4_1d_c;

    let mut tmp: [int32_t; 16] = [0; 16];
    let mut c: *mut int32_t = tmp.as_mut_ptr();
    let mut y = 0;
    while y < 4 {
        let mut x = 0;
        while x < 4 {
            *c.offset(x as isize) = *coeff.offset((y + x * 4) as isize) >> 2;
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
            *dst.offset(x_1 as isize) = iclip(
                *dst.offset(x_1 as isize) as libc::c_int + *fresh1,
                0 as libc::c_int,
                bitdepth_max,
            ) as pixel;
            x_1 += 1;
        }
        y_0 += 1;
        dst = dst.offset(PXSTRIDE(stride) as isize);
    }
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
#[inline(always)]
#[rustfmt::skip]
unsafe extern "C" fn itx_dsp_init_x86(c: *mut Dav1dInvTxfmDSPContext, bpc: libc::c_int) {
    use crate::src::x86::cpu::*;
    // TODO(legare): Temporary import until init fns are deduplicated.
    use crate::src::itx::*;

    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_X86_CPU_FLAG_SSE2 == 0 {
        return;
    }

    (*c).itxfm_add[TX_4X4 as usize][WHT_WHT as usize] = Some(dav1d_inv_txfm_add_wht_wht_4x4_16bpc_sse2);

    if flags & DAV1D_X86_CPU_FLAG_SSSE3 == 0 {
        return;
    }

    if flags & DAV1D_X86_CPU_FLAG_SSE41 == 0 {
        return;
    }

    if bpc == 10 {
        (*c).itxfm_add[TX_4X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x4_16bpc_sse4);
        (*c).itxfm_add[TX_4X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x4_16bpc_sse4);
        (*c).itxfm_add[TX_4X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x4_16bpc_sse4);
        (*c).itxfm_add[TX_4X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x4_16bpc_sse4);
        (*c).itxfm_add[TX_4X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x4_16bpc_sse4);
        (*c).itxfm_add[TX_4X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x4_16bpc_sse4);
        (*c).itxfm_add[TX_4X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x4_16bpc_sse4);
        (*c).itxfm_add[TX_4X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x4_16bpc_sse4);
        (*c).itxfm_add[TX_4X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x4_16bpc_sse4);
        (*c).itxfm_add[TX_4X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x4_16bpc_sse4);
        (*c).itxfm_add[TX_4X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x4_16bpc_sse4);
        (*c).itxfm_add[TX_4X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x4_16bpc_sse4);
        (*c).itxfm_add[TX_4X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x4_16bpc_sse4);
        (*c).itxfm_add[TX_4X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x4_16bpc_sse4);
        (*c).itxfm_add[TX_4X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x4_16bpc_sse4);
        (*c).itxfm_add[TX_4X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x4_16bpc_sse4);
        (*c).itxfm_add[RTX_4X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x8_16bpc_sse4);
        (*c).itxfm_add[RTX_4X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x8_16bpc_sse4);
        (*c).itxfm_add[RTX_4X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x8_16bpc_sse4);
        (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x8_16bpc_sse4);
        (*c).itxfm_add[RTX_4X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x8_16bpc_sse4);
        (*c).itxfm_add[RTX_4X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x8_16bpc_sse4);
        (*c).itxfm_add[RTX_4X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x8_16bpc_sse4);
        (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x8_16bpc_sse4);
        (*c).itxfm_add[RTX_4X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x8_16bpc_sse4);
        (*c).itxfm_add[RTX_4X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x8_16bpc_sse4);
        (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x8_16bpc_sse4);
        (*c).itxfm_add[RTX_4X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x8_16bpc_sse4);
        (*c).itxfm_add[RTX_4X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x8_16bpc_sse4);
        (*c).itxfm_add[RTX_4X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x8_16bpc_sse4);
        (*c).itxfm_add[RTX_4X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x8_16bpc_sse4);
        (*c).itxfm_add[RTX_4X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x8_16bpc_sse4);
        (*c).itxfm_add[RTX_4X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x16_16bpc_sse4);
        (*c).itxfm_add[RTX_4X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x16_16bpc_sse4);
        (*c).itxfm_add[RTX_4X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x16_16bpc_sse4);
        (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x16_16bpc_sse4);
        (*c).itxfm_add[RTX_4X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x16_16bpc_sse4);
        (*c).itxfm_add[RTX_4X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x16_16bpc_sse4);
        (*c).itxfm_add[RTX_4X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x16_16bpc_sse4);
        (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x16_16bpc_sse4);
        (*c).itxfm_add[RTX_4X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x16_16bpc_sse4);
        (*c).itxfm_add[RTX_4X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x16_16bpc_sse4);
        (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x16_16bpc_sse4);
        (*c).itxfm_add[RTX_4X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x16_16bpc_sse4);
        (*c).itxfm_add[RTX_4X16 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x16_16bpc_sse4);
        (*c).itxfm_add[RTX_4X16 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x16_16bpc_sse4);
        (*c).itxfm_add[RTX_4X16 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x16_16bpc_sse4);
        (*c).itxfm_add[RTX_4X16 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x16_16bpc_sse4);
        (*c).itxfm_add[RTX_8X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x4_16bpc_sse4);
        (*c).itxfm_add[RTX_8X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x4_16bpc_sse4);
        (*c).itxfm_add[RTX_8X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x4_16bpc_sse4);
        (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x4_16bpc_sse4);
        (*c).itxfm_add[RTX_8X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x4_16bpc_sse4);
        (*c).itxfm_add[RTX_8X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x4_16bpc_sse4);
        (*c).itxfm_add[RTX_8X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x4_16bpc_sse4);
        (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x4_16bpc_sse4);
        (*c).itxfm_add[RTX_8X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x4_16bpc_sse4);
        (*c).itxfm_add[RTX_8X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x4_16bpc_sse4);
        (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x4_16bpc_sse4);
        (*c).itxfm_add[RTX_8X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x4_16bpc_sse4);
        (*c).itxfm_add[RTX_8X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x4_16bpc_sse4);
        (*c).itxfm_add[RTX_8X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x4_16bpc_sse4);
        (*c).itxfm_add[RTX_8X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x4_16bpc_sse4);
        (*c).itxfm_add[RTX_8X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x4_16bpc_sse4);
        (*c).itxfm_add[TX_8X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x8_16bpc_sse4);
        (*c).itxfm_add[TX_8X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x8_16bpc_sse4);
        (*c).itxfm_add[TX_8X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x8_16bpc_sse4);
        (*c).itxfm_add[TX_8X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x8_16bpc_sse4);
        (*c).itxfm_add[TX_8X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x8_16bpc_sse4);
        (*c).itxfm_add[TX_8X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x8_16bpc_sse4);
        (*c).itxfm_add[TX_8X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x8_16bpc_sse4);
        (*c).itxfm_add[TX_8X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x8_16bpc_sse4);
        (*c).itxfm_add[TX_8X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x8_16bpc_sse4);
        (*c).itxfm_add[TX_8X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x8_16bpc_sse4);
        (*c).itxfm_add[TX_8X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x8_16bpc_sse4);
        (*c).itxfm_add[TX_8X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x8_16bpc_sse4);
        (*c).itxfm_add[TX_8X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x8_16bpc_sse4);
        (*c).itxfm_add[TX_8X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x8_16bpc_sse4);
        (*c).itxfm_add[TX_8X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x8_16bpc_sse4);
        (*c).itxfm_add[TX_8X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x8_16bpc_sse4);
        (*c).itxfm_add[RTX_8X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x16_16bpc_sse4);
        (*c).itxfm_add[RTX_8X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x16_16bpc_sse4);
        (*c).itxfm_add[RTX_8X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x16_16bpc_sse4);
        (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x16_16bpc_sse4);
        (*c).itxfm_add[RTX_8X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x16_16bpc_sse4);
        (*c).itxfm_add[RTX_8X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x16_16bpc_sse4);
        (*c).itxfm_add[RTX_8X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x16_16bpc_sse4);
        (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x16_16bpc_sse4);
        (*c).itxfm_add[RTX_8X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x16_16bpc_sse4);
        (*c).itxfm_add[RTX_8X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x16_16bpc_sse4);
        (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x16_16bpc_sse4);
        (*c).itxfm_add[RTX_8X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x16_16bpc_sse4);
        (*c).itxfm_add[RTX_8X16 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x16_16bpc_sse4);
        (*c).itxfm_add[RTX_8X16 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x16_16bpc_sse4);
        (*c).itxfm_add[RTX_8X16 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x16_16bpc_sse4);
        (*c).itxfm_add[RTX_8X16 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x16_16bpc_sse4);
        (*c).itxfm_add[RTX_16X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x4_16bpc_sse4);
        (*c).itxfm_add[RTX_16X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x4_16bpc_sse4);
        (*c).itxfm_add[RTX_16X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x4_16bpc_sse4);
        (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x4_16bpc_sse4);
        (*c).itxfm_add[RTX_16X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x4_16bpc_sse4);
        (*c).itxfm_add[RTX_16X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x4_16bpc_sse4);
        (*c).itxfm_add[RTX_16X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x4_16bpc_sse4);
        (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x4_16bpc_sse4);
        (*c).itxfm_add[RTX_16X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x4_16bpc_sse4);
        (*c).itxfm_add[RTX_16X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x4_16bpc_sse4);
        (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x4_16bpc_sse4);
        (*c).itxfm_add[RTX_16X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x4_16bpc_sse4);
        (*c).itxfm_add[RTX_16X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_16x4_16bpc_sse4);
        (*c).itxfm_add[RTX_16X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_16x4_16bpc_sse4);
        (*c).itxfm_add[RTX_16X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_16x4_16bpc_sse4);
        (*c).itxfm_add[RTX_16X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_16x4_16bpc_sse4);
        (*c).itxfm_add[RTX_16X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x8_16bpc_sse4);
        (*c).itxfm_add[RTX_16X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x8_16bpc_sse4);
        (*c).itxfm_add[RTX_16X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x8_16bpc_sse4);
        (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x8_16bpc_sse4);
        (*c).itxfm_add[RTX_16X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x8_16bpc_sse4);
        (*c).itxfm_add[RTX_16X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x8_16bpc_sse4);
        (*c).itxfm_add[RTX_16X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x8_16bpc_sse4);
        (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x8_16bpc_sse4);
        (*c).itxfm_add[RTX_16X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x8_16bpc_sse4);
        (*c).itxfm_add[RTX_16X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x8_16bpc_sse4);
        (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x8_16bpc_sse4);
        (*c).itxfm_add[RTX_16X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x8_16bpc_sse4);
        (*c).itxfm_add[RTX_16X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_16x8_16bpc_sse4);
        (*c).itxfm_add[RTX_16X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_16x8_16bpc_sse4);
        (*c).itxfm_add[RTX_16X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_16x8_16bpc_sse4);
        (*c).itxfm_add[RTX_16X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_16x8_16bpc_sse4);
        (*c).itxfm_add[TX_16X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x16_16bpc_sse4);
        (*c).itxfm_add[TX_16X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x16_16bpc_sse4);
        (*c).itxfm_add[TX_16X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x16_16bpc_sse4);
        (*c).itxfm_add[TX_16X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x16_16bpc_sse4);
        (*c).itxfm_add[TX_16X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x16_16bpc_sse4);
        (*c).itxfm_add[TX_16X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x16_16bpc_sse4);
        (*c).itxfm_add[TX_16X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x16_16bpc_sse4);
        (*c).itxfm_add[TX_16X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x16_16bpc_sse4);
        (*c).itxfm_add[TX_16X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x16_16bpc_sse4);
        (*c).itxfm_add[TX_16X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x16_16bpc_sse4);
        (*c).itxfm_add[TX_16X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x16_16bpc_sse4);
        (*c).itxfm_add[TX_16X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x16_16bpc_sse4);
        (*c).itxfm_add[RTX_8X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x32_16bpc_sse4);
        (*c).itxfm_add[RTX_8X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x32_16bpc_sse4);
        (*c).itxfm_add[RTX_32X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x8_16bpc_sse4);
        (*c).itxfm_add[RTX_32X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x8_16bpc_sse4);
        (*c).itxfm_add[RTX_16X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x32_16bpc_sse4);
        (*c).itxfm_add[RTX_16X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x32_16bpc_sse4);
        (*c).itxfm_add[RTX_32X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x16_16bpc_sse4);
        (*c).itxfm_add[RTX_32X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x16_16bpc_sse4);
        (*c).itxfm_add[TX_32X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x32_16bpc_sse4);
        (*c).itxfm_add[TX_32X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x32_16bpc_sse4);
        (*c).itxfm_add[RTX_16X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x64_16bpc_sse4);
        (*c).itxfm_add[RTX_32X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x64_16bpc_sse4);
        (*c).itxfm_add[RTX_64X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x16_16bpc_sse4);
        (*c).itxfm_add[RTX_64X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x32_16bpc_sse4);
        (*c).itxfm_add[TX_64X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x64_16bpc_sse4);
    }


    #[cfg(target_arch = "x86_64")]
    {
        if flags & DAV1D_X86_CPU_FLAG_AVX2 == 0 {
            return;
        }

        (*c).itxfm_add[TX_4X4 as usize][WHT_WHT as usize] = Some(dav1d_inv_txfm_add_wht_wht_4x4_16bpc_avx2);

        if bpc == 10 {
            (*c).itxfm_add[TX_4X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x4_10bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x4_10bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x4_10bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x4_10bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x4_10bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x4_10bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x4_10bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x4_10bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x4_10bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x4_10bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x4_10bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x4_10bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x4_10bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x4_10bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x4_10bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x4_10bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x8_10bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x8_10bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x8_10bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x8_10bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x8_10bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x8_10bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x8_10bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x8_10bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x8_10bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x8_10bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x8_10bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x8_10bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x8_10bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x8_10bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x8_10bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x8_10bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x16_10bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x16_10bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x16_10bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x16_10bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x16_10bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x16_10bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x16_10bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x16_10bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x16_10bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x16_10bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x16_10bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x16_10bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x16_10bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x16_10bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x16_10bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x16_10bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x4_10bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x4_10bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x4_10bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x4_10bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x4_10bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x4_10bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x4_10bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x4_10bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x4_10bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x4_10bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x4_10bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x4_10bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x4_10bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x4_10bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x4_10bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x4_10bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x8_10bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x8_10bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x8_10bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x8_10bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x8_10bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x8_10bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x8_10bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x8_10bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x8_10bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x8_10bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x8_10bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x8_10bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x8_10bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x8_10bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x8_10bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x8_10bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x16_10bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x16_10bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x16_10bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x16_10bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x16_10bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x16_10bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x16_10bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x16_10bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x16_10bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x16_10bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x16_10bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x16_10bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x16_10bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x16_10bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x16_10bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x16_10bpc_avx2);
            (*c).itxfm_add[RTX_8X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x32_10bpc_avx2);
            (*c).itxfm_add[RTX_8X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x32_10bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x4_10bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x4_10bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x4_10bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x4_10bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x4_10bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x4_10bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x4_10bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x4_10bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x4_10bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x4_10bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x4_10bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x4_10bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_16x4_10bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_16x4_10bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_16x4_10bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_16x4_10bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x8_10bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x8_10bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x8_10bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x8_10bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x8_10bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x8_10bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x8_10bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x8_10bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x8_10bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x8_10bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x8_10bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x8_10bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_16x8_10bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_16x8_10bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_16x8_10bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_16x8_10bpc_avx2);
            (*c).itxfm_add[TX_16X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x16_10bpc_avx2);
            (*c).itxfm_add[TX_16X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x16_10bpc_avx2);
            (*c).itxfm_add[TX_16X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x16_10bpc_avx2);
            (*c).itxfm_add[TX_16X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x16_10bpc_avx2);
            (*c).itxfm_add[TX_16X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x16_10bpc_avx2);
            (*c).itxfm_add[TX_16X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x16_10bpc_avx2);
            (*c).itxfm_add[TX_16X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x16_10bpc_avx2);
            (*c).itxfm_add[TX_16X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x16_10bpc_avx2);
            (*c).itxfm_add[TX_16X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x16_10bpc_avx2);
            (*c).itxfm_add[TX_16X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x16_10bpc_avx2);
            (*c).itxfm_add[TX_16X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x16_10bpc_avx2);
            (*c).itxfm_add[TX_16X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x16_10bpc_avx2);
            (*c).itxfm_add[RTX_16X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x32_10bpc_avx2);
            (*c).itxfm_add[RTX_16X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x32_10bpc_avx2);
            (*c).itxfm_add[RTX_16X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x64_10bpc_avx2);
            (*c).itxfm_add[RTX_32X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x8_10bpc_avx2);
            (*c).itxfm_add[RTX_32X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x8_10bpc_avx2);
            (*c).itxfm_add[RTX_32X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x16_10bpc_avx2);
            (*c).itxfm_add[RTX_32X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x16_10bpc_avx2);
            (*c).itxfm_add[TX_32X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x32_10bpc_avx2);
            (*c).itxfm_add[TX_32X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x32_10bpc_avx2);
            (*c).itxfm_add[RTX_32X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x64_10bpc_avx2);
            (*c).itxfm_add[RTX_64X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x16_10bpc_avx2);
            (*c).itxfm_add[RTX_64X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x32_10bpc_avx2);
            (*c).itxfm_add[TX_64X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x64_10bpc_avx2);
        } else {
            (*c).itxfm_add[TX_4X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x4_12bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x4_12bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x4_12bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x4_12bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x4_12bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x4_12bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x4_12bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x4_12bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x4_12bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x4_12bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x4_12bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x4_12bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x4_12bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x4_12bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x4_12bpc_avx2);
            (*c).itxfm_add[TX_4X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x4_12bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x8_12bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x8_12bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x8_12bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x8_12bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x8_12bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x8_12bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x8_12bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x8_12bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x8_12bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x8_12bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x8_12bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x8_12bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x8_12bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x8_12bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x8_12bpc_avx2);
            (*c).itxfm_add[RTX_4X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x8_12bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_4x16_12bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_4x16_12bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_4x16_12bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_4x16_12bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_4x16_12bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_4x16_12bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_4x16_12bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_4x16_12bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_4x16_12bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_4x16_12bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x16_12bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_4x16_12bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_4x16_12bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_4x16_12bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_4x16_12bpc_avx2);
            (*c).itxfm_add[RTX_4X16 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_4x16_12bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x4_12bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x4_12bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x4_12bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x4_12bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x4_12bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x4_12bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x4_12bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x4_12bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x4_12bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x4_12bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x4_12bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x4_12bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x4_12bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x4_12bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x4_12bpc_avx2);
            (*c).itxfm_add[RTX_8X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x4_12bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x8_12bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x8_12bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x8_12bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x8_12bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x8_12bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x8_12bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x8_12bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x8_12bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x8_12bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x8_12bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x8_12bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x8_12bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x8_12bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x8_12bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x8_12bpc_avx2);
            (*c).itxfm_add[TX_8X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x8_12bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x16_12bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x16_12bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x16_12bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x16_12bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x16_12bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x16_12bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x16_12bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x16_12bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x16_12bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x16_12bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x16_12bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x16_12bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x16_12bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x16_12bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x16_12bpc_avx2);
            (*c).itxfm_add[RTX_8X16 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x16_12bpc_avx2);
            (*c).itxfm_add[RTX_8X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x32_12bpc_avx2);
            (*c).itxfm_add[RTX_8X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x32_12bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x4_12bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x4_12bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x4_12bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x4_12bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x4_12bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x4_12bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x4_12bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x4_12bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x4_12bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x4_12bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x4_12bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x4_12bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_16x4_12bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_16x4_12bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_16x4_12bpc_avx2);
            (*c).itxfm_add[RTX_16X4 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_16x4_12bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x8_12bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x8_12bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x8_12bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x8_12bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x8_12bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x8_12bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x8_12bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x8_12bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x8_12bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x8_12bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x8_12bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x8_12bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_16x8_12bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_16x8_12bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_16x8_12bpc_avx2);
            (*c).itxfm_add[RTX_16X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_16x8_12bpc_avx2);
            (*c).itxfm_add[TX_16X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x16_12bpc_avx2);
            (*c).itxfm_add[TX_16X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x16_12bpc_avx2);
            (*c).itxfm_add[TX_16X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x16_12bpc_avx2);
            (*c).itxfm_add[TX_16X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x16_12bpc_avx2);
            (*c).itxfm_add[TX_16X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x16_12bpc_avx2);
            (*c).itxfm_add[TX_16X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x16_12bpc_avx2);
            (*c).itxfm_add[TX_16X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x16_12bpc_avx2);
            (*c).itxfm_add[TX_16X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x16_12bpc_avx2);
            (*c).itxfm_add[TX_16X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x16_12bpc_avx2);
            (*c).itxfm_add[TX_16X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x16_12bpc_avx2);
            (*c).itxfm_add[TX_16X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x16_12bpc_avx2);
            (*c).itxfm_add[TX_16X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x16_12bpc_avx2);
            (*c).itxfm_add[RTX_32X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x8_12bpc_avx2);
            (*c).itxfm_add[RTX_32X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x8_12bpc_avx2);
            (*c).itxfm_add[RTX_16X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x32_12bpc_avx2);
            (*c).itxfm_add[RTX_32X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x16_12bpc_avx2);
            (*c).itxfm_add[TX_32X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x32_12bpc_avx2);
        }

        if flags & DAV1D_X86_CPU_FLAG_AVX512ICL == 0 {
            return;
        }

        if bpc == 10 {
            (*c).itxfm_add[TX_8X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x8_10bpc_avx512icl);
            (*c).itxfm_add[TX_8X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x8_10bpc_avx512icl);
            (*c).itxfm_add[TX_8X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x8_10bpc_avx512icl);
            (*c).itxfm_add[TX_8X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x8_10bpc_avx512icl);
            (*c).itxfm_add[TX_8X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x8_10bpc_avx512icl);
            (*c).itxfm_add[TX_8X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x8_10bpc_avx512icl);
            (*c).itxfm_add[TX_8X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x8_10bpc_avx512icl);
            (*c).itxfm_add[TX_8X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x8_10bpc_avx512icl);
            (*c).itxfm_add[TX_8X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x8_10bpc_avx512icl);
            (*c).itxfm_add[TX_8X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x8_10bpc_avx512icl);
            (*c).itxfm_add[TX_8X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x8_10bpc_avx512icl);
            (*c).itxfm_add[TX_8X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x8_10bpc_avx512icl);
            (*c).itxfm_add[TX_8X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x8_10bpc_avx512icl);
            (*c).itxfm_add[TX_8X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x8_10bpc_avx512icl);
            (*c).itxfm_add[TX_8X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x8_10bpc_avx512icl);
            (*c).itxfm_add[TX_8X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x8_10bpc_avx512icl);
            (*c).itxfm_add[RTX_8X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x16_10bpc_avx512icl);
            (*c).itxfm_add[RTX_8X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x16_10bpc_avx512icl);
            (*c).itxfm_add[RTX_8X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_8x16_10bpc_avx512icl);
            (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_8x16_10bpc_avx512icl);
            (*c).itxfm_add[RTX_8X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_8x16_10bpc_avx512icl);
            (*c).itxfm_add[RTX_8X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_8x16_10bpc_avx512icl);
            (*c).itxfm_add[RTX_8X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_8x16_10bpc_avx512icl);
            (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_8x16_10bpc_avx512icl);
            (*c).itxfm_add[RTX_8X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_8x16_10bpc_avx512icl);
            (*c).itxfm_add[RTX_8X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_8x16_10bpc_avx512icl);
            (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x16_10bpc_avx512icl);
            (*c).itxfm_add[RTX_8X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_8x16_10bpc_avx512icl);
            (*c).itxfm_add[RTX_8X16 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_8x16_10bpc_avx512icl);
            (*c).itxfm_add[RTX_8X16 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_8x16_10bpc_avx512icl);
            (*c).itxfm_add[RTX_8X16 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_8x16_10bpc_avx512icl);
            (*c).itxfm_add[RTX_8X16 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_8x16_10bpc_avx512icl);
            (*c).itxfm_add[RTX_8X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_8x32_10bpc_avx512icl);
            (*c).itxfm_add[RTX_8X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_8x32_10bpc_avx512icl);
            (*c).itxfm_add[RTX_16X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x8_10bpc_avx512icl);
            (*c).itxfm_add[RTX_16X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x8_10bpc_avx512icl);
            (*c).itxfm_add[RTX_16X8 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x8_10bpc_avx512icl);
            (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x8_10bpc_avx512icl);
            (*c).itxfm_add[RTX_16X8 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x8_10bpc_avx512icl);
            (*c).itxfm_add[RTX_16X8 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x8_10bpc_avx512icl);
            (*c).itxfm_add[RTX_16X8 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x8_10bpc_avx512icl);
            (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x8_10bpc_avx512icl);
            (*c).itxfm_add[RTX_16X8 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x8_10bpc_avx512icl);
            (*c).itxfm_add[RTX_16X8 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x8_10bpc_avx512icl);
            (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x8_10bpc_avx512icl);
            (*c).itxfm_add[RTX_16X8 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x8_10bpc_avx512icl);
            (*c).itxfm_add[RTX_16X8 as usize][H_ADST as usize] = Some(dav1d_inv_txfm_add_adst_identity_16x8_10bpc_avx512icl);
            (*c).itxfm_add[RTX_16X8 as usize][H_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_identity_16x8_10bpc_avx512icl);
            (*c).itxfm_add[RTX_16X8 as usize][V_ADST as usize] = Some(dav1d_inv_txfm_add_identity_adst_16x8_10bpc_avx512icl);
            (*c).itxfm_add[RTX_16X8 as usize][V_FLIPADST as usize] = Some(dav1d_inv_txfm_add_identity_flipadst_16x8_10bpc_avx512icl);
            (*c).itxfm_add[TX_16X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x16_10bpc_avx512icl);
            (*c).itxfm_add[TX_16X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x16_10bpc_avx512icl);
            (*c).itxfm_add[TX_16X16 as usize][ADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_adst_16x16_10bpc_avx512icl);
            (*c).itxfm_add[TX_16X16 as usize][FLIPADST_DCT as usize] = Some(dav1d_inv_txfm_add_dct_flipadst_16x16_10bpc_avx512icl);
            (*c).itxfm_add[TX_16X16 as usize][H_DCT as usize] = Some(dav1d_inv_txfm_add_dct_identity_16x16_10bpc_avx512icl);
            (*c).itxfm_add[TX_16X16 as usize][DCT_ADST as usize] = Some(dav1d_inv_txfm_add_adst_dct_16x16_10bpc_avx512icl);
            (*c).itxfm_add[TX_16X16 as usize][ADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_adst_16x16_10bpc_avx512icl);
            (*c).itxfm_add[TX_16X16 as usize][FLIPADST_ADST as usize] = Some(dav1d_inv_txfm_add_adst_flipadst_16x16_10bpc_avx512icl);
            (*c).itxfm_add[TX_16X16 as usize][DCT_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_dct_16x16_10bpc_avx512icl);
            (*c).itxfm_add[TX_16X16 as usize][ADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_adst_16x16_10bpc_avx512icl);
            (*c).itxfm_add[TX_16X16 as usize][FLIPADST_FLIPADST as usize] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x16_10bpc_avx512icl);
            (*c).itxfm_add[TX_16X16 as usize][V_DCT as usize] = Some(dav1d_inv_txfm_add_identity_dct_16x16_10bpc_avx512icl);
            (*c).itxfm_add[RTX_16X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x32_10bpc_avx512icl);
            (*c).itxfm_add[RTX_16X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_16x32_10bpc_avx512icl);
            (*c).itxfm_add[RTX_32X8 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x8_10bpc_avx512icl);
            (*c).itxfm_add[RTX_32X8 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x8_10bpc_avx512icl);
            (*c).itxfm_add[RTX_32X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x16_10bpc_avx512icl);
            (*c).itxfm_add[RTX_32X16 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x16_10bpc_avx512icl);
            (*c).itxfm_add[TX_32X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x32_10bpc_avx512icl);
            (*c).itxfm_add[TX_32X32 as usize][IDTX as usize] = Some(dav1d_inv_txfm_add_identity_identity_32x32_10bpc_avx512icl);
            (*c).itxfm_add[RTX_16X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_16x64_10bpc_avx512icl);
            (*c).itxfm_add[RTX_32X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_32x64_10bpc_avx512icl);
            (*c).itxfm_add[RTX_64X16 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x16_10bpc_avx512icl);
            (*c).itxfm_add[RTX_64X32 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x32_10bpc_avx512icl);
            (*c).itxfm_add[TX_64X64 as usize][DCT_DCT as usize] = Some(dav1d_inv_txfm_add_dct_dct_64x64_10bpc_avx512icl);
        }
    }
}

#[cfg(feature = "asm")]
use crate::src::cpu::dav1d_get_cpu_flags;

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
#[inline(always)]
unsafe extern "C" fn itx_dsp_init_arm(c: *mut Dav1dInvTxfmDSPContext, mut bpc: libc::c_int) {
    use crate::src::arm::cpu::DAV1D_ARM_CPU_FLAG_NEON;

    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
        return;
    }

    if bpc != 10 {
        return;
    }

    (*c).itxfm_add[TX_4X4 as usize][DCT_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_dct_4x4_16bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][IDTX as usize] =
        Some(dav1d_inv_txfm_add_identity_identity_4x4_16bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][ADST_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_adst_4x4_16bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_flipadst_4x4_16bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][H_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_identity_4x4_16bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][DCT_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_dct_4x4_16bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][ADST_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_adst_4x4_16bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_flipadst_4x4_16bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][DCT_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_dct_4x4_16bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][ADST_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_adst_4x4_16bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_flipadst_4x4_16bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][V_DCT as usize] =
        Some(dav1d_inv_txfm_add_identity_dct_4x4_16bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][H_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_identity_4x4_16bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][H_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_identity_4x4_16bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][V_ADST as usize] =
        Some(dav1d_inv_txfm_add_identity_adst_4x4_16bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][V_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_identity_flipadst_4x4_16bpc_neon);
    (*c).itxfm_add[TX_4X4 as usize][WHT_WHT as usize] =
        Some(dav1d_inv_txfm_add_wht_wht_4x4_16bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_dct_4x8_16bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][IDTX as usize] =
        Some(dav1d_inv_txfm_add_identity_identity_4x8_16bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_adst_4x8_16bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_flipadst_4x8_16bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][H_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_identity_4x8_16bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_dct_4x8_16bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_adst_4x8_16bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_flipadst_4x8_16bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_dct_4x8_16bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_adst_4x8_16bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_flipadst_4x8_16bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][V_DCT as usize] =
        Some(dav1d_inv_txfm_add_identity_dct_4x8_16bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][H_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_identity_4x8_16bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][H_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_identity_4x8_16bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][V_ADST as usize] =
        Some(dav1d_inv_txfm_add_identity_adst_4x8_16bpc_neon);
    (*c).itxfm_add[RTX_4X8 as usize][V_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_identity_flipadst_4x8_16bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_dct_4x16_16bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][IDTX as usize] =
        Some(dav1d_inv_txfm_add_identity_identity_4x16_16bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_adst_4x16_16bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_flipadst_4x16_16bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][H_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_identity_4x16_16bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_dct_4x16_16bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_adst_4x16_16bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_flipadst_4x16_16bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_dct_4x16_16bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_adst_4x16_16bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_flipadst_4x16_16bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][V_DCT as usize] =
        Some(dav1d_inv_txfm_add_identity_dct_4x16_16bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][H_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_identity_4x16_16bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][H_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_identity_4x16_16bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][V_ADST as usize] =
        Some(dav1d_inv_txfm_add_identity_adst_4x16_16bpc_neon);
    (*c).itxfm_add[RTX_4X16 as usize][V_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_identity_flipadst_4x16_16bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_dct_8x4_16bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][IDTX as usize] =
        Some(dav1d_inv_txfm_add_identity_identity_8x4_16bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_adst_8x4_16bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_flipadst_8x4_16bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][H_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_identity_8x4_16bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_dct_8x4_16bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_adst_8x4_16bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_flipadst_8x4_16bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_dct_8x4_16bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_adst_8x4_16bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_flipadst_8x4_16bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][V_DCT as usize] =
        Some(dav1d_inv_txfm_add_identity_dct_8x4_16bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][H_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_identity_8x4_16bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][H_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_identity_8x4_16bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][V_ADST as usize] =
        Some(dav1d_inv_txfm_add_identity_adst_8x4_16bpc_neon);
    (*c).itxfm_add[RTX_8X4 as usize][V_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_identity_flipadst_8x4_16bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][DCT_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_dct_8x8_16bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][IDTX as usize] =
        Some(dav1d_inv_txfm_add_identity_identity_8x8_16bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][ADST_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_adst_8x8_16bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_flipadst_8x8_16bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][H_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_identity_8x8_16bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][DCT_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_dct_8x8_16bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][ADST_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_adst_8x8_16bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_flipadst_8x8_16bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][DCT_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_dct_8x8_16bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][ADST_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_adst_8x8_16bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_flipadst_8x8_16bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][V_DCT as usize] =
        Some(dav1d_inv_txfm_add_identity_dct_8x8_16bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][H_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_identity_8x8_16bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][H_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_identity_8x8_16bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][V_ADST as usize] =
        Some(dav1d_inv_txfm_add_identity_adst_8x8_16bpc_neon);
    (*c).itxfm_add[TX_8X8 as usize][V_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_identity_flipadst_8x8_16bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_dct_8x16_16bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][IDTX as usize] =
        Some(dav1d_inv_txfm_add_identity_identity_8x16_16bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_adst_8x16_16bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_flipadst_8x16_16bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][H_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_identity_8x16_16bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_dct_8x16_16bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_adst_8x16_16bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_flipadst_8x16_16bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_dct_8x16_16bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_adst_8x16_16bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_flipadst_8x16_16bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][V_DCT as usize] =
        Some(dav1d_inv_txfm_add_identity_dct_8x16_16bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][H_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_identity_8x16_16bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][H_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_identity_8x16_16bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][V_ADST as usize] =
        Some(dav1d_inv_txfm_add_identity_adst_8x16_16bpc_neon);
    (*c).itxfm_add[RTX_8X16 as usize][V_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_identity_flipadst_8x16_16bpc_neon);
    (*c).itxfm_add[RTX_8X32 as usize][DCT_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_dct_8x32_16bpc_neon);
    (*c).itxfm_add[RTX_8X32 as usize][IDTX as usize] =
        Some(dav1d_inv_txfm_add_identity_identity_8x32_16bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_dct_16x4_16bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][IDTX as usize] =
        Some(dav1d_inv_txfm_add_identity_identity_16x4_16bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_adst_16x4_16bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_flipadst_16x4_16bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][H_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_identity_16x4_16bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_dct_16x4_16bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_adst_16x4_16bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_flipadst_16x4_16bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_dct_16x4_16bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_adst_16x4_16bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_flipadst_16x4_16bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][V_DCT as usize] =
        Some(dav1d_inv_txfm_add_identity_dct_16x4_16bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][H_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_identity_16x4_16bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][H_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_identity_16x4_16bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][V_ADST as usize] =
        Some(dav1d_inv_txfm_add_identity_adst_16x4_16bpc_neon);
    (*c).itxfm_add[RTX_16X4 as usize][V_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_identity_flipadst_16x4_16bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_dct_16x8_16bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][IDTX as usize] =
        Some(dav1d_inv_txfm_add_identity_identity_16x8_16bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_adst_16x8_16bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_flipadst_16x8_16bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][H_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_identity_16x8_16bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_dct_16x8_16bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_adst_16x8_16bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_flipadst_16x8_16bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_dct_16x8_16bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_adst_16x8_16bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_flipadst_16x8_16bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][V_DCT as usize] =
        Some(dav1d_inv_txfm_add_identity_dct_16x8_16bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][H_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_identity_16x8_16bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][H_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_identity_16x8_16bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][V_ADST as usize] =
        Some(dav1d_inv_txfm_add_identity_adst_16x8_16bpc_neon);
    (*c).itxfm_add[RTX_16X8 as usize][V_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_identity_flipadst_16x8_16bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][DCT_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_dct_16x16_16bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][IDTX as usize] =
        Some(dav1d_inv_txfm_add_identity_identity_16x16_16bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][ADST_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_adst_16x16_16bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_flipadst_16x16_16bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][H_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_identity_16x16_16bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][DCT_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_dct_16x16_16bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][ADST_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_adst_16x16_16bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_ADST as usize] =
        Some(dav1d_inv_txfm_add_adst_flipadst_16x16_16bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][DCT_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_dct_16x16_16bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][ADST_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_adst_16x16_16bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_FLIPADST as usize] =
        Some(dav1d_inv_txfm_add_flipadst_flipadst_16x16_16bpc_neon);
    (*c).itxfm_add[TX_16X16 as usize][V_DCT as usize] =
        Some(dav1d_inv_txfm_add_identity_dct_16x16_16bpc_neon);
    (*c).itxfm_add[RTX_16X32 as usize][DCT_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_dct_16x32_16bpc_neon);
    (*c).itxfm_add[RTX_16X32 as usize][IDTX as usize] =
        Some(dav1d_inv_txfm_add_identity_identity_16x32_16bpc_neon);
    (*c).itxfm_add[RTX_16X64 as usize][DCT_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_dct_16x64_16bpc_neon);
    (*c).itxfm_add[RTX_32X8 as usize][DCT_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_dct_32x8_16bpc_neon);
    (*c).itxfm_add[RTX_32X8 as usize][IDTX as usize] =
        Some(dav1d_inv_txfm_add_identity_identity_32x8_16bpc_neon);
    (*c).itxfm_add[RTX_32X16 as usize][DCT_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_dct_32x16_16bpc_neon);
    (*c).itxfm_add[RTX_32X16 as usize][IDTX as usize] =
        Some(dav1d_inv_txfm_add_identity_identity_32x16_16bpc_neon);
    (*c).itxfm_add[TX_32X32 as usize][DCT_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_dct_32x32_16bpc_neon);
    (*c).itxfm_add[TX_32X32 as usize][IDTX as usize] =
        Some(dav1d_inv_txfm_add_identity_identity_32x32_16bpc_neon);
    (*c).itxfm_add[RTX_32X64 as usize][DCT_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_dct_32x64_16bpc_neon);
    (*c).itxfm_add[RTX_64X16 as usize][DCT_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_dct_64x16_16bpc_neon);
    (*c).itxfm_add[RTX_64X32 as usize][DCT_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_dct_64x32_16bpc_neon);
    (*c).itxfm_add[TX_64X64 as usize][DCT_DCT as usize] =
        Some(dav1d_inv_txfm_add_dct_dct_64x64_16bpc_neon);
}

#[no_mangle]
#[cold]
#[rustfmt::skip]
pub unsafe extern "C" fn dav1d_itx_dsp_init_16bpc(
    c: *mut Dav1dInvTxfmDSPContext,
    mut _bpc: libc::c_int,
) {
    // TODO(legare): Temporary import until init fns are deduplicated.
    use crate::src::itx::*;

    (*c).itxfm_add[TX_4X4 as usize][WHT_WHT as usize] = Some(inv_txfm_add_wht_wht_4x4_c_erased);
    (*c).itxfm_add[TX_4X4 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_4x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_4X4 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_4x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_4X4 as usize][DCT_ADST as usize] = Some(inv_txfm_add_adst_dct_4x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_4X4 as usize][ADST_DCT as usize] = Some(inv_txfm_add_dct_adst_4x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_4X4 as usize][ADST_ADST as usize] = Some(inv_txfm_add_adst_adst_4x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_4X4 as usize][ADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_adst_4x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_ADST as usize] = Some(inv_txfm_add_adst_flipadst_4x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_4X4 as usize][DCT_FLIPADST as usize] = Some(inv_txfm_add_flipadst_dct_4x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_DCT as usize] = Some(inv_txfm_add_dct_flipadst_4x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_flipadst_4x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_4X4 as usize][H_DCT as usize] = Some(inv_txfm_add_dct_identity_4x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_4X4 as usize][V_DCT as usize] = Some(inv_txfm_add_identity_dct_4x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_4X4 as usize][H_FLIPADST as usize] = Some(inv_txfm_add_flipadst_identity_4x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_4X4 as usize][V_FLIPADST as usize] = Some(inv_txfm_add_identity_flipadst_4x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_4X4 as usize][H_ADST as usize] = Some(inv_txfm_add_adst_identity_4x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_4X4 as usize][V_ADST as usize] = Some(inv_txfm_add_identity_adst_4x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_4x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X8 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_4x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_ADST as usize] = Some(inv_txfm_add_adst_dct_4x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_DCT as usize] = Some(inv_txfm_add_dct_adst_4x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_ADST as usize] = Some(inv_txfm_add_adst_adst_4x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_adst_4x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_ADST as usize] = Some(inv_txfm_add_adst_flipadst_4x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_FLIPADST as usize] = Some(inv_txfm_add_flipadst_dct_4x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_DCT as usize] = Some(inv_txfm_add_dct_flipadst_4x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_flipadst_4x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X8 as usize][H_DCT as usize] = Some(inv_txfm_add_dct_identity_4x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X8 as usize][V_DCT as usize] = Some(inv_txfm_add_identity_dct_4x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X8 as usize][H_FLIPADST as usize] = Some(inv_txfm_add_flipadst_identity_4x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X8 as usize][V_FLIPADST as usize] = Some(inv_txfm_add_identity_flipadst_4x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X8 as usize][H_ADST as usize] = Some(inv_txfm_add_adst_identity_4x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X8 as usize][V_ADST as usize] = Some(inv_txfm_add_identity_adst_4x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_4x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X16 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_4x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_ADST as usize] = Some(inv_txfm_add_adst_dct_4x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_DCT as usize] = Some(inv_txfm_add_dct_adst_4x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_ADST as usize] = Some(inv_txfm_add_adst_adst_4x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_adst_4x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_ADST as usize] = Some(inv_txfm_add_adst_flipadst_4x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_FLIPADST as usize] = Some(inv_txfm_add_flipadst_dct_4x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_DCT as usize] = Some(inv_txfm_add_dct_flipadst_4x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_flipadst_4x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X16 as usize][H_DCT as usize] = Some(inv_txfm_add_dct_identity_4x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X16 as usize][V_DCT as usize] = Some(inv_txfm_add_identity_dct_4x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X16 as usize][H_FLIPADST as usize] = Some(inv_txfm_add_flipadst_identity_4x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X16 as usize][V_FLIPADST as usize] = Some(inv_txfm_add_identity_flipadst_4x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X16 as usize][H_ADST as usize] = Some(inv_txfm_add_adst_identity_4x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_4X16 as usize][V_ADST as usize] = Some(inv_txfm_add_identity_adst_4x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_8x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X4 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_8x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_ADST as usize] = Some(inv_txfm_add_adst_dct_8x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_DCT as usize] = Some(inv_txfm_add_dct_adst_8x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_ADST as usize] = Some(inv_txfm_add_adst_adst_8x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_adst_8x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_ADST as usize] = Some(inv_txfm_add_adst_flipadst_8x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_FLIPADST as usize] = Some(inv_txfm_add_flipadst_dct_8x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_DCT as usize] = Some(inv_txfm_add_dct_flipadst_8x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_flipadst_8x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X4 as usize][H_DCT as usize] = Some(inv_txfm_add_dct_identity_8x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X4 as usize][V_DCT as usize] = Some(inv_txfm_add_identity_dct_8x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X4 as usize][H_FLIPADST as usize] = Some(inv_txfm_add_flipadst_identity_8x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X4 as usize][V_FLIPADST as usize] = Some(inv_txfm_add_identity_flipadst_8x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X4 as usize][H_ADST as usize] = Some(inv_txfm_add_adst_identity_8x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X4 as usize][V_ADST as usize] = Some(inv_txfm_add_identity_adst_8x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_8X8 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_8x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_8X8 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_8x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_8X8 as usize][DCT_ADST as usize] = Some(inv_txfm_add_adst_dct_8x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_8X8 as usize][ADST_DCT as usize] = Some(inv_txfm_add_dct_adst_8x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_8X8 as usize][ADST_ADST as usize] = Some(inv_txfm_add_adst_adst_8x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_8X8 as usize][ADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_adst_8x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_ADST as usize] = Some(inv_txfm_add_adst_flipadst_8x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_8X8 as usize][DCT_FLIPADST as usize] = Some(inv_txfm_add_flipadst_dct_8x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_DCT as usize] = Some(inv_txfm_add_dct_flipadst_8x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_flipadst_8x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_8X8 as usize][H_DCT as usize] = Some(inv_txfm_add_dct_identity_8x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_8X8 as usize][V_DCT as usize] = Some(inv_txfm_add_identity_dct_8x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_8X8 as usize][H_FLIPADST as usize] = Some(inv_txfm_add_flipadst_identity_8x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_8X8 as usize][V_FLIPADST as usize] = Some(inv_txfm_add_identity_flipadst_8x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_8X8 as usize][H_ADST as usize] = Some(inv_txfm_add_adst_identity_8x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_8X8 as usize][V_ADST as usize] = Some(inv_txfm_add_identity_adst_8x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_8x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X16 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_8x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_ADST as usize] = Some(inv_txfm_add_adst_dct_8x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_DCT as usize] = Some(inv_txfm_add_dct_adst_8x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_ADST as usize] = Some(inv_txfm_add_adst_adst_8x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_adst_8x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_ADST as usize] = Some(inv_txfm_add_adst_flipadst_8x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_FLIPADST as usize] = Some(inv_txfm_add_flipadst_dct_8x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_DCT as usize] = Some(inv_txfm_add_dct_flipadst_8x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_flipadst_8x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X16 as usize][H_DCT as usize] = Some(inv_txfm_add_dct_identity_8x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X16 as usize][V_DCT as usize] = Some(inv_txfm_add_identity_dct_8x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X16 as usize][H_FLIPADST as usize] = Some(inv_txfm_add_flipadst_identity_8x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X16 as usize][V_FLIPADST as usize] = Some(inv_txfm_add_identity_flipadst_8x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X16 as usize][H_ADST as usize] = Some(inv_txfm_add_adst_identity_8x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X16 as usize][V_ADST as usize] = Some(inv_txfm_add_identity_adst_8x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X32 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_8x32_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_8X32 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_8x32_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_16x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X4 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_16x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_ADST as usize] = Some(inv_txfm_add_adst_dct_16x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_DCT as usize] = Some(inv_txfm_add_dct_adst_16x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_ADST as usize] = Some(inv_txfm_add_adst_adst_16x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_adst_16x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_ADST as usize] = Some(inv_txfm_add_adst_flipadst_16x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_FLIPADST as usize] = Some(inv_txfm_add_flipadst_dct_16x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_DCT as usize] = Some(inv_txfm_add_dct_flipadst_16x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_flipadst_16x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X4 as usize][H_DCT as usize] = Some(inv_txfm_add_dct_identity_16x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X4 as usize][V_DCT as usize] = Some(inv_txfm_add_identity_dct_16x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X4 as usize][H_FLIPADST as usize] = Some(inv_txfm_add_flipadst_identity_16x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X4 as usize][V_FLIPADST as usize] = Some(inv_txfm_add_identity_flipadst_16x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X4 as usize][H_ADST as usize] = Some(inv_txfm_add_adst_identity_16x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X4 as usize][V_ADST as usize] = Some(inv_txfm_add_identity_adst_16x4_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_16x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X8 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_16x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_ADST as usize] = Some(inv_txfm_add_adst_dct_16x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_DCT as usize] = Some(inv_txfm_add_dct_adst_16x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_ADST as usize] = Some(inv_txfm_add_adst_adst_16x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_adst_16x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_ADST as usize] = Some(inv_txfm_add_adst_flipadst_16x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_FLIPADST as usize] = Some(inv_txfm_add_flipadst_dct_16x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_DCT as usize] = Some(inv_txfm_add_dct_flipadst_16x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_flipadst_16x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X8 as usize][H_DCT as usize] = Some(inv_txfm_add_dct_identity_16x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X8 as usize][V_DCT as usize] = Some(inv_txfm_add_identity_dct_16x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X8 as usize][H_FLIPADST as usize] = Some(inv_txfm_add_flipadst_identity_16x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X8 as usize][V_FLIPADST as usize] = Some(inv_txfm_add_identity_flipadst_16x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X8 as usize][H_ADST as usize] = Some(inv_txfm_add_adst_identity_16x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X8 as usize][V_ADST as usize] = Some(inv_txfm_add_identity_adst_16x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_16X16 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_16x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_16X16 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_16x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_16X16 as usize][DCT_ADST as usize] = Some(inv_txfm_add_adst_dct_16x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_16X16 as usize][ADST_DCT as usize] = Some(inv_txfm_add_dct_adst_16x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_16X16 as usize][ADST_ADST as usize] = Some(inv_txfm_add_adst_adst_16x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_16X16 as usize][ADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_adst_16x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_ADST as usize] = Some(inv_txfm_add_adst_flipadst_16x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_16X16 as usize][DCT_FLIPADST as usize] = Some(inv_txfm_add_flipadst_dct_16x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_DCT as usize] = Some(inv_txfm_add_dct_flipadst_16x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_FLIPADST as usize] = Some(inv_txfm_add_flipadst_flipadst_16x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_16X16 as usize][H_DCT as usize] = Some(inv_txfm_add_dct_identity_16x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_16X16 as usize][V_DCT as usize] = Some(inv_txfm_add_identity_dct_16x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X32 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_16x32_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X32 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_16x32_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_16X64 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_16x64_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_32X8 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_32x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_32X8 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_32x8_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_32X16 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_32x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_32X16 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_32x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_32X32 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_32x32_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_32X32 as usize][IDTX as usize] = Some(inv_txfm_add_identity_identity_32x32_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_32X64 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_32x64_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_64X16 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_64x16_c_erased::<BitDepth16>);
    (*c).itxfm_add[RTX_64X32 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_64x32_c_erased::<BitDepth16>);
    (*c).itxfm_add[TX_64X64 as usize][DCT_DCT as usize] = Some(inv_txfm_add_dct_dct_64x64_c_erased::<BitDepth16>);

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            itx_dsp_init_x86(c, _bpc);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
            itx_dsp_init_arm(c, _bpc);
        }
    }
}
