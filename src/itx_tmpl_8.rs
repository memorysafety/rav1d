use ::libc;
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
pub type ptrdiff_t = libc::c_long;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __int32_t = libc::c_int;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type pixel = uint8_t;
pub type coef = int16_t;
pub type TxfmSize = libc::c_uint;
pub const N_TX_SIZES: TxfmSize = 5;
pub const TX_64X64: TxfmSize = 4;
pub const TX_32X32: TxfmSize = 3;
pub const TX_16X16: TxfmSize = 2;
pub const TX_8X8: TxfmSize = 1;
pub const TX_4X4: TxfmSize = 0;
pub type RectTxfmSize = libc::c_uint;
pub const N_RECT_TX_SIZES: RectTxfmSize = 19;
pub const RTX_64X16: RectTxfmSize = 18;
pub const RTX_16X64: RectTxfmSize = 17;
pub const RTX_32X8: RectTxfmSize = 16;
pub const RTX_8X32: RectTxfmSize = 15;
pub const RTX_16X4: RectTxfmSize = 14;
pub const RTX_4X16: RectTxfmSize = 13;
pub const RTX_64X32: RectTxfmSize = 12;
pub const RTX_32X64: RectTxfmSize = 11;
pub const RTX_32X16: RectTxfmSize = 10;
pub const RTX_16X32: RectTxfmSize = 9;
pub const RTX_16X8: RectTxfmSize = 8;
pub const RTX_8X16: RectTxfmSize = 7;
pub const RTX_8X4: RectTxfmSize = 6;
pub const RTX_4X8: RectTxfmSize = 5;
pub type TxfmType = libc::c_uint;
pub const N_TX_TYPES_PLUS_LL: TxfmType = 17;
pub const WHT_WHT: TxfmType = 16;
pub const N_TX_TYPES: TxfmType = 16;
pub const H_FLIPADST: TxfmType = 15;
pub const V_FLIPADST: TxfmType = 14;
pub const H_ADST: TxfmType = 13;
pub const V_ADST: TxfmType = 12;
pub const H_DCT: TxfmType = 11;
pub const V_DCT: TxfmType = 10;
pub const IDTX: TxfmType = 9;
pub const FLIPADST_ADST: TxfmType = 8;
pub const ADST_FLIPADST: TxfmType = 7;
pub const FLIPADST_FLIPADST: TxfmType = 6;
pub const DCT_FLIPADST: TxfmType = 5;
pub const FLIPADST_DCT: TxfmType = 4;
pub const ADST_ADST: TxfmType = 3;
pub const DCT_ADST: TxfmType = 2;
pub const ADST_DCT: TxfmType = 1;
pub const DCT_DCT: TxfmType = 0;
pub type itxfm_fn =
    Option<unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> ()>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav1dInvTxfmDSPContext {
    pub itxfm_add: [[itxfm_fn; 17]; 19],
}
pub type itx_1d_fn =
    Option<unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> ()>;
#[inline]
unsafe extern "C" fn imin(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn iclip(v: libc::c_int, min: libc::c_int, max: libc::c_int) -> libc::c_int {
    return if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    };
}
#[inline]
unsafe extern "C" fn iclip_u8(v: libc::c_int) -> libc::c_int {
    return iclip(v, 0i32, 255i32);
}
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
    if !(w >= 4i32 && w <= 64i32) {
        unreachable!();
    }
    if !(h >= 4i32 && h <= 64i32) {
        unreachable!();
    }
    if !(eob >= 0i32) {
        unreachable!();
    }
    let is_rect2: libc::c_int = (w * 2i32 == h || h * 2i32 == w) as libc::c_int;
    let rnd: libc::c_int = (1i32) << shift >> 1i32;
    if eob < has_dconly {
        let mut dc: libc::c_int = *coeff.offset(0isize) as libc::c_int;
        *coeff.offset(0isize) = 0i16;
        if is_rect2 != 0 {
            dc = dc * 181i32 + 128i32 >> 8i32;
        }
        dc = dc * 181i32 + 128i32 >> 8i32;
        dc = dc + rnd >> shift;
        dc = dc * 181i32 + 128i32 + 2048i32 >> 12i32;
        let mut y: libc::c_int = 0i32;
        while y < h {
            let mut x: libc::c_int = 0i32;
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
    let sh: libc::c_int = imin(h, 32i32);
    let sw: libc::c_int = imin(w, 32i32);
    let row_clip_min: libc::c_int = -(32767i32) - 1i32;
    let col_clip_min: libc::c_int = -(32767i32) - 1i32;
    let row_clip_max: libc::c_int = !row_clip_min;
    let col_clip_max: libc::c_int = !col_clip_min;
    let mut tmp: [int32_t; 4096] = [0; 4096];
    let mut c: *mut int32_t = tmp.as_mut_ptr();
    let mut y_0: libc::c_int = 0i32;
    while y_0 < sh {
        if is_rect2 != 0 {
            let mut x_0: libc::c_int = 0i32;
            while x_0 < sw {
                *c.offset(x_0 as isize) =
                    *coeff.offset((y_0 + x_0 * sh) as isize) as libc::c_int * 181i32 + 128i32
                        >> 8i32;
                x_0 += 1;
            }
        } else {
            let mut x_1: libc::c_int = 0i32;
            while x_1 < sw {
                *c.offset(x_1 as isize) = *coeff.offset((y_0 + x_1 * sh) as isize) as int32_t;
                x_1 += 1;
            }
        }
        first_1d_fn.expect("non-null function pointer")(c, 1i64, row_clip_min, row_clip_max);
        y_0 += 1;
        c = c.offset(w as isize);
    }
    memset(
        coeff as *mut libc::c_void,
        0i32,
        (::core::mem::size_of::<coef>() as libc::c_ulong)
            .wrapping_mul(sw as libc::c_ulong)
            .wrapping_mul(sh as libc::c_ulong),
    );
    let mut i: libc::c_int = 0i32;
    while i < w * sh {
        tmp[i as usize] = iclip(tmp[i as usize] + rnd >> shift, col_clip_min, col_clip_max);
        i += 1;
    }
    let mut x_2: libc::c_int = 0i32;
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
    let mut y_1: libc::c_int = 0i32;
    while y_1 < h {
        let mut x_3: libc::c_int = 0i32;
        while x_3 < w {
            let fresh0 = c;
            c = c.offset(1);
            *dst.offset(x_3 as isize) =
                iclip_u8(*dst.offset(x_3 as isize) as libc::c_int + (*fresh0 + 8i32 >> 4i32))
                    as pixel;
            x_3 += 1;
        }
        y_1 += 1;
        dst = dst.offset(stride as isize);
    }
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
        4i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
    );
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
        4i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1i32,
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
        4i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        8i32,
        0i32,
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        8i32,
        0i32,
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1i32,
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
        4i32,
        8i32,
        0i32,
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        8i32,
        0i32,
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        8i32,
        0i32,
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        8i32,
        0i32,
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        8i32,
        0i32,
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        8i32,
        0i32,
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        8i32,
        0i32,
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        8i32,
        0i32,
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        8i32,
        0i32,
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        8i32,
        0i32,
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        8i32,
        0i32,
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        8i32,
        0i32,
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        8i32,
        0i32,
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        8i32,
        0i32,
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1i32,
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
        4i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        4i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1i32,
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
        8i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        4i32,
        0i32,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1i32,
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
        8i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1i32,
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
        8i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        32i32,
        2i32,
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        8i32,
        32i32,
        2i32,
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1i32,
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
        16i32,
        4i32,
        1i32,
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        4i32,
        1i32,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1i32,
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
        16i32,
        4i32,
        1i32,
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        4i32,
        1i32,
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        4i32,
        1i32,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        4i32,
        1i32,
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        4i32,
        1i32,
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        4i32,
        1i32,
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        4i32,
        1i32,
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        4i32,
        1i32,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        4i32,
        1i32,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        4i32,
        1i32,
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        4i32,
        1i32,
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        4i32,
        1i32,
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        4i32,
        1i32,
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        4i32,
        1i32,
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst4_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1i32,
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
        16i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        8i32,
        1i32,
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        16i32,
        2i32,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1i32,
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
        16i32,
        16i32,
        2i32,
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        16i32,
        2i32,
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        16i32,
        2i32,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        16i32,
        2i32,
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        16i32,
        2i32,
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        16i32,
        2i32,
        Some(
            dav1d_inv_adst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        16i32,
        2i32,
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        16i32,
        2i32,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        16i32,
        2i32,
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_flipadst16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        16i32,
        2i32,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        16i32,
        2i32,
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        32i32,
        1i32,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1i32,
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
        16i32,
        32i32,
        1i32,
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        16i32,
        64i32,
        2i32,
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct64_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1i32,
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
        32i32,
        8i32,
        2i32,
        Some(
            dav1d_inv_dct32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1i32,
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
        32i32,
        8i32,
        2i32,
        Some(
            dav1d_inv_identity32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity8_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        32i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_dct32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1i32,
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
        32i32,
        16i32,
        1i32,
        Some(
            dav1d_inv_identity32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        32i32,
        32i32,
        2i32,
        Some(
            dav1d_inv_dct32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1i32,
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
        32i32,
        32i32,
        2i32,
        Some(
            dav1d_inv_identity32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_identity32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        0i32,
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
        32i32,
        64i32,
        1i32,
        Some(
            dav1d_inv_dct32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct64_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1i32,
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
        64i32,
        16i32,
        2i32,
        Some(
            dav1d_inv_dct64_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct16_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1i32,
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
        64i32,
        32i32,
        1i32,
        Some(
            dav1d_inv_dct64_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct32_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1i32,
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
        64i32,
        64i32,
        2i32,
        Some(
            dav1d_inv_dct64_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        Some(
            dav1d_inv_dct64_1d_c
                as unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> (),
        ),
        1i32,
    );
}
unsafe extern "C" fn inv_txfm_add_wht_wht_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: libc::c_int,
) {
    let mut tmp: [int32_t; 16] = [0; 16];
    let mut c: *mut int32_t = tmp.as_mut_ptr();
    let mut y: libc::c_int = 0i32;
    while y < 4i32 {
        let mut x: libc::c_int = 0i32;
        while x < 4i32 {
            *c.offset(x as isize) = *coeff.offset((y + x * 4i32) as isize) as libc::c_int >> 2i32;
            x += 1;
        }
        dav1d_inv_wht4_1d_c(c, 1i64);
        y += 1;
        c = c.offset(4isize);
    }
    memset(
        coeff as *mut libc::c_void,
        0i32,
        (::core::mem::size_of::<coef>() as libc::c_ulong)
            .wrapping_mul(4u64)
            .wrapping_mul(4u64),
    );
    let mut x_0: libc::c_int = 0i32;
    while x_0 < 4i32 {
        dav1d_inv_wht4_1d_c(&mut *tmp.as_mut_ptr().offset(x_0 as isize), 4i64);
        x_0 += 1;
    }
    c = tmp.as_mut_ptr();
    let mut y_0: libc::c_int = 0i32;
    while y_0 < 4i32 {
        let mut x_1: libc::c_int = 0i32;
        while x_1 < 4i32 {
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
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_itx_dsp_init_8bpc(
    c: *mut Dav1dInvTxfmDSPContext,
    mut bpc: libc::c_int,
) {
    (*c).itxfm_add[TX_4X4 as usize][WHT_WHT as usize] = Some(
        inv_txfm_add_wht_wht_4x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_4X4 as usize][DCT_DCT as usize] = Some(
        inv_txfm_add_dct_dct_4x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_4X4 as usize][IDTX as usize] = Some(
        inv_txfm_add_identity_identity_4x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_4X4 as usize][DCT_ADST as usize] = Some(
        inv_txfm_add_adst_dct_4x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_4X4 as usize][ADST_DCT as usize] = Some(
        inv_txfm_add_dct_adst_4x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_4X4 as usize][ADST_ADST as usize] = Some(
        inv_txfm_add_adst_adst_4x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_4X4 as usize][ADST_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_adst_4x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_ADST as usize] = Some(
        inv_txfm_add_adst_flipadst_4x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_4X4 as usize][DCT_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_dct_4x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_DCT as usize] = Some(
        inv_txfm_add_dct_flipadst_4x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_flipadst_4x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_4X4 as usize][H_DCT as usize] = Some(
        inv_txfm_add_dct_identity_4x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_4X4 as usize][V_DCT as usize] = Some(
        inv_txfm_add_identity_dct_4x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_4X4 as usize][H_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_identity_4x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_4X4 as usize][V_FLIPADST as usize] = Some(
        inv_txfm_add_identity_flipadst_4x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_4X4 as usize][H_ADST as usize] = Some(
        inv_txfm_add_adst_identity_4x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_4X4 as usize][V_ADST as usize] = Some(
        inv_txfm_add_identity_adst_4x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X8 as usize][DCT_DCT as usize] = Some(
        inv_txfm_add_dct_dct_4x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X8 as usize][IDTX as usize] = Some(
        inv_txfm_add_identity_identity_4x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X8 as usize][DCT_ADST as usize] = Some(
        inv_txfm_add_adst_dct_4x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X8 as usize][ADST_DCT as usize] = Some(
        inv_txfm_add_dct_adst_4x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X8 as usize][ADST_ADST as usize] = Some(
        inv_txfm_add_adst_adst_4x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X8 as usize][ADST_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_adst_4x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_ADST as usize] = Some(
        inv_txfm_add_adst_flipadst_4x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X8 as usize][DCT_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_dct_4x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_DCT as usize] = Some(
        inv_txfm_add_dct_flipadst_4x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_flipadst_4x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X8 as usize][H_DCT as usize] = Some(
        inv_txfm_add_dct_identity_4x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X8 as usize][V_DCT as usize] = Some(
        inv_txfm_add_identity_dct_4x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X8 as usize][H_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_identity_4x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X8 as usize][V_FLIPADST as usize] = Some(
        inv_txfm_add_identity_flipadst_4x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X8 as usize][H_ADST as usize] = Some(
        inv_txfm_add_adst_identity_4x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X8 as usize][V_ADST as usize] = Some(
        inv_txfm_add_identity_adst_4x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X16 as usize][DCT_DCT as usize] = Some(
        inv_txfm_add_dct_dct_4x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X16 as usize][IDTX as usize] = Some(
        inv_txfm_add_identity_identity_4x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X16 as usize][DCT_ADST as usize] = Some(
        inv_txfm_add_adst_dct_4x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X16 as usize][ADST_DCT as usize] = Some(
        inv_txfm_add_dct_adst_4x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X16 as usize][ADST_ADST as usize] = Some(
        inv_txfm_add_adst_adst_4x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X16 as usize][ADST_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_adst_4x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_ADST as usize] = Some(
        inv_txfm_add_adst_flipadst_4x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X16 as usize][DCT_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_dct_4x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_DCT as usize] = Some(
        inv_txfm_add_dct_flipadst_4x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_flipadst_4x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X16 as usize][H_DCT as usize] = Some(
        inv_txfm_add_dct_identity_4x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X16 as usize][V_DCT as usize] = Some(
        inv_txfm_add_identity_dct_4x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X16 as usize][H_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_identity_4x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X16 as usize][V_FLIPADST as usize] = Some(
        inv_txfm_add_identity_flipadst_4x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X16 as usize][H_ADST as usize] = Some(
        inv_txfm_add_adst_identity_4x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_4X16 as usize][V_ADST as usize] = Some(
        inv_txfm_add_identity_adst_4x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X4 as usize][DCT_DCT as usize] = Some(
        inv_txfm_add_dct_dct_8x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X4 as usize][IDTX as usize] = Some(
        inv_txfm_add_identity_identity_8x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X4 as usize][DCT_ADST as usize] = Some(
        inv_txfm_add_adst_dct_8x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X4 as usize][ADST_DCT as usize] = Some(
        inv_txfm_add_dct_adst_8x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X4 as usize][ADST_ADST as usize] = Some(
        inv_txfm_add_adst_adst_8x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X4 as usize][ADST_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_adst_8x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_ADST as usize] = Some(
        inv_txfm_add_adst_flipadst_8x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X4 as usize][DCT_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_dct_8x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_DCT as usize] = Some(
        inv_txfm_add_dct_flipadst_8x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_flipadst_8x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X4 as usize][H_DCT as usize] = Some(
        inv_txfm_add_dct_identity_8x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X4 as usize][V_DCT as usize] = Some(
        inv_txfm_add_identity_dct_8x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X4 as usize][H_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_identity_8x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X4 as usize][V_FLIPADST as usize] = Some(
        inv_txfm_add_identity_flipadst_8x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X4 as usize][H_ADST as usize] = Some(
        inv_txfm_add_adst_identity_8x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X4 as usize][V_ADST as usize] = Some(
        inv_txfm_add_identity_adst_8x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_8X8 as usize][DCT_DCT as usize] = Some(
        inv_txfm_add_dct_dct_8x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_8X8 as usize][IDTX as usize] = Some(
        inv_txfm_add_identity_identity_8x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_8X8 as usize][DCT_ADST as usize] = Some(
        inv_txfm_add_adst_dct_8x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_8X8 as usize][ADST_DCT as usize] = Some(
        inv_txfm_add_dct_adst_8x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_8X8 as usize][ADST_ADST as usize] = Some(
        inv_txfm_add_adst_adst_8x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_8X8 as usize][ADST_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_adst_8x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_ADST as usize] = Some(
        inv_txfm_add_adst_flipadst_8x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_8X8 as usize][DCT_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_dct_8x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_DCT as usize] = Some(
        inv_txfm_add_dct_flipadst_8x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_flipadst_8x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_8X8 as usize][H_DCT as usize] = Some(
        inv_txfm_add_dct_identity_8x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_8X8 as usize][V_DCT as usize] = Some(
        inv_txfm_add_identity_dct_8x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_8X8 as usize][H_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_identity_8x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_8X8 as usize][V_FLIPADST as usize] = Some(
        inv_txfm_add_identity_flipadst_8x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_8X8 as usize][H_ADST as usize] = Some(
        inv_txfm_add_adst_identity_8x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_8X8 as usize][V_ADST as usize] = Some(
        inv_txfm_add_identity_adst_8x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X16 as usize][DCT_DCT as usize] = Some(
        inv_txfm_add_dct_dct_8x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X16 as usize][IDTX as usize] = Some(
        inv_txfm_add_identity_identity_8x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X16 as usize][DCT_ADST as usize] = Some(
        inv_txfm_add_adst_dct_8x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X16 as usize][ADST_DCT as usize] = Some(
        inv_txfm_add_dct_adst_8x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X16 as usize][ADST_ADST as usize] = Some(
        inv_txfm_add_adst_adst_8x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X16 as usize][ADST_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_adst_8x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_ADST as usize] = Some(
        inv_txfm_add_adst_flipadst_8x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X16 as usize][DCT_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_dct_8x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_DCT as usize] = Some(
        inv_txfm_add_dct_flipadst_8x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_flipadst_8x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X16 as usize][H_DCT as usize] = Some(
        inv_txfm_add_dct_identity_8x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X16 as usize][V_DCT as usize] = Some(
        inv_txfm_add_identity_dct_8x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X16 as usize][H_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_identity_8x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X16 as usize][V_FLIPADST as usize] = Some(
        inv_txfm_add_identity_flipadst_8x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X16 as usize][H_ADST as usize] = Some(
        inv_txfm_add_adst_identity_8x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X16 as usize][V_ADST as usize] = Some(
        inv_txfm_add_identity_adst_8x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X32 as usize][DCT_DCT as usize] = Some(
        inv_txfm_add_dct_dct_8x32_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_8X32 as usize][IDTX as usize] = Some(
        inv_txfm_add_identity_identity_8x32_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X4 as usize][DCT_DCT as usize] = Some(
        inv_txfm_add_dct_dct_16x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X4 as usize][IDTX as usize] = Some(
        inv_txfm_add_identity_identity_16x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X4 as usize][DCT_ADST as usize] = Some(
        inv_txfm_add_adst_dct_16x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X4 as usize][ADST_DCT as usize] = Some(
        inv_txfm_add_dct_adst_16x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X4 as usize][ADST_ADST as usize] = Some(
        inv_txfm_add_adst_adst_16x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X4 as usize][ADST_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_adst_16x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_ADST as usize] = Some(
        inv_txfm_add_adst_flipadst_16x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X4 as usize][DCT_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_dct_16x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_DCT as usize] = Some(
        inv_txfm_add_dct_flipadst_16x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_flipadst_16x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X4 as usize][H_DCT as usize] = Some(
        inv_txfm_add_dct_identity_16x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X4 as usize][V_DCT as usize] = Some(
        inv_txfm_add_identity_dct_16x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X4 as usize][H_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_identity_16x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X4 as usize][V_FLIPADST as usize] = Some(
        inv_txfm_add_identity_flipadst_16x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X4 as usize][H_ADST as usize] = Some(
        inv_txfm_add_adst_identity_16x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X4 as usize][V_ADST as usize] = Some(
        inv_txfm_add_identity_adst_16x4_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X8 as usize][DCT_DCT as usize] = Some(
        inv_txfm_add_dct_dct_16x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X8 as usize][IDTX as usize] = Some(
        inv_txfm_add_identity_identity_16x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X8 as usize][DCT_ADST as usize] = Some(
        inv_txfm_add_adst_dct_16x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X8 as usize][ADST_DCT as usize] = Some(
        inv_txfm_add_dct_adst_16x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X8 as usize][ADST_ADST as usize] = Some(
        inv_txfm_add_adst_adst_16x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X8 as usize][ADST_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_adst_16x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_ADST as usize] = Some(
        inv_txfm_add_adst_flipadst_16x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X8 as usize][DCT_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_dct_16x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_DCT as usize] = Some(
        inv_txfm_add_dct_flipadst_16x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_flipadst_16x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X8 as usize][H_DCT as usize] = Some(
        inv_txfm_add_dct_identity_16x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X8 as usize][V_DCT as usize] = Some(
        inv_txfm_add_identity_dct_16x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X8 as usize][H_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_identity_16x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X8 as usize][V_FLIPADST as usize] = Some(
        inv_txfm_add_identity_flipadst_16x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X8 as usize][H_ADST as usize] = Some(
        inv_txfm_add_adst_identity_16x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X8 as usize][V_ADST as usize] = Some(
        inv_txfm_add_identity_adst_16x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_16X16 as usize][DCT_DCT as usize] = Some(
        inv_txfm_add_dct_dct_16x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_16X16 as usize][IDTX as usize] = Some(
        inv_txfm_add_identity_identity_16x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_16X16 as usize][DCT_ADST as usize] = Some(
        inv_txfm_add_adst_dct_16x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_16X16 as usize][ADST_DCT as usize] = Some(
        inv_txfm_add_dct_adst_16x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_16X16 as usize][ADST_ADST as usize] = Some(
        inv_txfm_add_adst_adst_16x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_16X16 as usize][ADST_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_adst_16x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_ADST as usize] = Some(
        inv_txfm_add_adst_flipadst_16x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_16X16 as usize][DCT_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_dct_16x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_DCT as usize] = Some(
        inv_txfm_add_dct_flipadst_16x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_FLIPADST as usize] = Some(
        inv_txfm_add_flipadst_flipadst_16x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_16X16 as usize][H_DCT as usize] = Some(
        inv_txfm_add_dct_identity_16x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_16X16 as usize][V_DCT as usize] = Some(
        inv_txfm_add_identity_dct_16x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X32 as usize][DCT_DCT as usize] = Some(
        inv_txfm_add_dct_dct_16x32_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X32 as usize][IDTX as usize] = Some(
        inv_txfm_add_identity_identity_16x32_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_16X64 as usize][DCT_DCT as usize] = Some(
        inv_txfm_add_dct_dct_16x64_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_32X8 as usize][DCT_DCT as usize] = Some(
        inv_txfm_add_dct_dct_32x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_32X8 as usize][IDTX as usize] = Some(
        inv_txfm_add_identity_identity_32x8_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_32X16 as usize][DCT_DCT as usize] = Some(
        inv_txfm_add_dct_dct_32x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_32X16 as usize][IDTX as usize] = Some(
        inv_txfm_add_identity_identity_32x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_32X32 as usize][DCT_DCT as usize] = Some(
        inv_txfm_add_dct_dct_32x32_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_32X32 as usize][IDTX as usize] = Some(
        inv_txfm_add_identity_identity_32x32_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_32X64 as usize][DCT_DCT as usize] = Some(
        inv_txfm_add_dct_dct_32x64_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_64X16 as usize][DCT_DCT as usize] = Some(
        inv_txfm_add_dct_dct_64x16_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[RTX_64X32 as usize][DCT_DCT as usize] = Some(
        inv_txfm_add_dct_dct_64x32_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
    (*c).itxfm_add[TX_64X64 as usize][DCT_DCT as usize] = Some(
        inv_txfm_add_dct_dct_64x64_c
            as unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
    );
}
