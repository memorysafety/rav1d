use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynCoef;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::intops::iclip;
use crate::src::levels::N_RECT_TX_SIZES;
use crate::src::levels::N_TX_TYPES_PLUS_LL;
use libc::memset;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_void;

pub type itx_1d_fn = Option<unsafe extern "C" fn(*mut i32, ptrdiff_t, c_int, c_int) -> ()>;

pub unsafe fn inv_txfm_add_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    coeff: *mut BD::Coef,
    eob: c_int,
    w: c_int,
    h: c_int,
    shift: c_int,
    first_1d_fn: itx_1d_fn,
    second_1d_fn: itx_1d_fn,
    has_dconly: c_int,
    bd: BD,
) {
    let bitdepth_max: c_int = bd.bitdepth_max().as_();
    let stride = stride as usize;
    if !(w >= 4 && w <= 64) {
        unreachable!();
    }
    if !(h >= 4 && h <= 64) {
        unreachable!();
    }
    if !(eob >= 0) {
        unreachable!();
    }
    let is_rect2: c_int = (w * 2 == h || h * 2 == w) as c_int;
    let rnd = (1 as c_int) << shift >> 1;
    if eob < has_dconly {
        let mut dc: c_int = (*coeff.offset(0)).as_();
        *coeff.offset(0) = 0.as_();
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
                    bd.iclip_pixel((*dst.offset(x as isize)).as_::<c_int>() + dc);
                x += 1;
            }
            y += 1;
            dst = dst.offset(BD::pxstride(stride) as isize);
        }
        return;
    }
    let sh = cmp::min(h, 32 as c_int);
    let sw = cmp::min(w, 32 as c_int);
    let row_clip_min;
    let col_clip_min;
    if BD::BITDEPTH == 8 {
        row_clip_min = i16::MIN as i32;
        col_clip_min = i16::MIN as i32;
    } else {
        row_clip_min = ((!bitdepth_max) << 7) as c_int;
        col_clip_min = ((!bitdepth_max) << 5) as c_int;
    }
    let row_clip_max = !row_clip_min;
    let col_clip_max = !col_clip_min;
    let mut tmp: [i32; 4096] = [0; 4096];
    let mut c: *mut i32 = tmp.as_mut_ptr();
    let mut y_0 = 0;
    while y_0 < sh {
        if is_rect2 != 0 {
            let mut x_0 = 0;
            while x_0 < sw {
                *c.offset(x_0 as isize) =
                    (*coeff.offset((y_0 + x_0 * sh) as isize)).as_::<c_int>() * 181 + 128 >> 8;
                x_0 += 1;
            }
        } else {
            let mut x_1 = 0;
            while x_1 < sw {
                *c.offset(x_1 as isize) = (*coeff.offset((y_0 + x_1 * sh) as isize)).as_();
                x_1 += 1;
            }
        }
        first_1d_fn.expect("non-null function pointer")(
            c,
            1 as c_int as ptrdiff_t,
            row_clip_min,
            row_clip_max,
        );
        y_0 += 1;
        c = c.offset(w as isize);
    }
    memset(
        coeff as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<BD::Coef>()
            .wrapping_mul(sw as usize)
            .wrapping_mul(sh as usize),
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
                bd.iclip_pixel((*dst.offset(x_3 as isize)).as_::<c_int>() + (*fresh0 + 8 >> 4));
            x_3 += 1;
        }
        y_1 += 1;
        dst = dst.offset(BD::pxstride(stride) as isize);
    }
}

pub type itxfm_fn =
    Option<unsafe extern "C" fn(*mut DynPixel, ptrdiff_t, *mut DynCoef, c_int, c_int) -> ()>;

#[repr(C)]
pub struct Rav1dInvTxfmDSPContext {
    pub itxfm_add: [[itxfm_fn; N_TX_TYPES_PLUS_LL]; N_RECT_TX_SIZES],
}

#[cfg(feature = "asm")]
macro_rules! decl_itx_fn {
    ($name:ident) => {
        // TODO(legare): Temporarily pub until init fns are deduplicated.
        pub(crate) fn $name(
            dst: *mut DynPixel,
            dst_stride: ptrdiff_t,
            coeff: *mut DynCoef,
            eob: c_int,
            bitdepth_max: c_int,
        );
    };

    ($type1:ident, $type2:ident, $w:literal x $h:literal, $bpc:literal bpc, $asm:ident) => {
        paste::paste! {
            decl_itx_fn!([<dav1d_inv_txfm_add_ $type1 _ $type2 _ $w x $h _ $bpc bpc_ $asm>]);
        }
    };

    ($type1:ident, $type2:ident, $w:literal x $h:literal, $asm:ident) => {
        #[cfg(feature = "bitdepth_8")]
        decl_itx_fn!($type1, $type2, $w x $h,  8 bpc, $asm);
        #[cfg(feature = "bitdepth_16")]
        decl_itx_fn!($type1, $type2, $w x $h, 16 bpc, $asm);
    };
}

#[cfg(feature = "asm")]
macro_rules! decl_itx1_fns {
    ($w:literal x $h:literal, $bpc:literal bpc, $asm:ident) => {
        decl_itx_fn!(dct, dct, $w x $h, $bpc bpc, $asm);
    };
}

#[cfg(feature = "asm")]
macro_rules! decl_itx2_fns {
    ($w:literal x $h:literal, $bpc:literal bpc, $asm:ident) => {
        decl_itx1_fns!($w x $h, $bpc bpc, $asm);
        decl_itx_fn!(identity, identity, $w x $h, $bpc bpc, $asm);
    };
}

#[cfg(feature = "asm")]
macro_rules! decl_itx12_fns {
    ($w:literal x $h:literal, $bpc:literal bpc, $asm:ident) => {
        decl_itx2_fns!($w x $h, $bpc bpc, $asm);
        decl_itx_fn!(dct, adst, $w x $h, $bpc bpc, $asm);
        decl_itx_fn!(dct, flipadst, $w x $h, $bpc bpc, $asm);
        decl_itx_fn!(dct, identity, $w x $h, $bpc bpc, $asm);
        decl_itx_fn!(adst, dct, $w x $h, $bpc bpc, $asm);
        decl_itx_fn!(adst, adst, $w x $h, $bpc bpc, $asm);
        decl_itx_fn!(adst, flipadst, $w x $h, $bpc bpc, $asm);
        decl_itx_fn!(flipadst, dct, $w x $h, $bpc bpc, $asm);
        decl_itx_fn!(flipadst, adst, $w x $h, $bpc bpc, $asm);
        decl_itx_fn!(flipadst, flipadst, $w x $h, $bpc bpc, $asm);
        decl_itx_fn!(identity, dct, $w x $h, $bpc bpc, $asm);
    };
}

#[cfg(feature = "asm")]
macro_rules! decl_itx16_fns {
    ($w:literal x $h:literal, $bpc:literal bpc, $asm:ident) => {
        decl_itx12_fns!($w x $h, $bpc bpc, $asm);
        decl_itx_fn!(adst, identity, $w x $h, $bpc bpc, $asm);
        decl_itx_fn!(flipadst, identity, $w x $h, $bpc bpc, $asm);
        decl_itx_fn!(identity, adst, $w x $h, $bpc bpc, $asm);
        decl_itx_fn!(identity, flipadst, $w x $h, $bpc bpc, $asm);
    };
}

#[cfg(feature = "asm")]
macro_rules! decl_itx_fns {
    ($bpc:literal bpc, $asm:ident) => {
        decl_itx16_fns!( 4 x  4, $bpc bpc, $asm);
        decl_itx16_fns!( 4 x  8, $bpc bpc, $asm);
        decl_itx16_fns!( 4 x 16, $bpc bpc, $asm);
        decl_itx16_fns!( 8 x  4, $bpc bpc, $asm);
        decl_itx16_fns!( 8 x  8, $bpc bpc, $asm);
        decl_itx16_fns!( 8 x 16, $bpc bpc, $asm);
        decl_itx16_fns!(16 x  4, $bpc bpc, $asm);
        decl_itx16_fns!(16 x  8, $bpc bpc, $asm);
        decl_itx12_fns!(16 x 16, $bpc bpc, $asm);
        decl_itx2_fns! ( 8 x 32, $bpc bpc, $asm);
        decl_itx2_fns! (16 x 32, $bpc bpc, $asm);
        decl_itx2_fns! (32 x  8, $bpc bpc, $asm);
        decl_itx2_fns! (32 x 16, $bpc bpc, $asm);
        decl_itx2_fns! (32 x 32, $bpc bpc, $asm);
        decl_itx1_fns! (16 x 64, $bpc bpc, $asm);
        decl_itx1_fns! (32 x 64, $bpc bpc, $asm);
        decl_itx1_fns! (64 x 16, $bpc bpc, $asm);
        decl_itx1_fns! (64 x 32, $bpc bpc, $asm);
        decl_itx1_fns! (64 x 64, $bpc bpc, $asm);
    };

    ($asm:ident) => {
        #[cfg(feature = "bitdepth_8")]
        decl_itx_fns!( 8 bpc, $asm);
        #[cfg(feature = "bitdepth_16")]
        decl_itx_fns!(16 bpc, $asm);
    };
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
extern "C" {
    decl_itx_fn!(wht, wht, 4 x 4, sse2);
}

#[cfg(all(
    feature = "asm",
    feature = "bitdepth_8",
    any(target_arch = "x86", target_arch = "x86_64"),
))]
extern "C" {
    decl_itx_fns!(8 bpc, ssse3);
}

#[cfg(all(
    feature = "asm",
    feature = "bitdepth_16",
    any(target_arch = "x86", target_arch = "x86_64"),
))]
extern "C" {
    decl_itx_fns!(16 bpc, sse4);
}

#[cfg(all(feature = "asm", feature = "bitdepth_8", target_arch = "x86_64"))]
extern "C" {
    decl_itx_fns!(8 bpc, avx2);
    decl_itx_fns!(8 bpc, avx512icl);
}

#[cfg(all(feature = "asm", target_arch = "x86_64",))]
extern "C" {
    decl_itx_fn!(wht, wht, 4 x 4, avx2);

    decl_itx_fns!(10 bpc, avx2);

    decl_itx16_fns!( 4 x  4, 12 bpc, avx2);
    decl_itx16_fns!( 4 x  8, 12 bpc, avx2);
    decl_itx16_fns!( 4 x 16, 12 bpc, avx2);
    decl_itx16_fns!( 8 x  4, 12 bpc, avx2);
    decl_itx16_fns!( 8 x  8, 12 bpc, avx2);
    decl_itx16_fns!( 8 x 16, 12 bpc, avx2);
    decl_itx16_fns!(16 x  4, 12 bpc, avx2);
    decl_itx16_fns!(16 x  8, 12 bpc, avx2);
    decl_itx12_fns!(16 x 16, 12 bpc, avx2);
    decl_itx2_fns! ( 8 x 32, 12 bpc, avx2);
    decl_itx2_fns! (32 x  8, 12 bpc, avx2);
    decl_itx_fn!(identity, identity, 16 x 32, 12 bpc, avx2);
    decl_itx_fn!(identity, identity, 32 x 16, 12 bpc, avx2);
    decl_itx_fn!(identity, identity, 32 x 32, 12 bpc, avx2);

    decl_itx16_fns!( 8 x  8, 10 bpc, avx512icl);
    decl_itx16_fns!( 8 x 16, 10 bpc, avx512icl);
    decl_itx16_fns!(16 x  8, 10 bpc, avx512icl);
    decl_itx12_fns!(16 x 16, 10 bpc, avx512icl);
    decl_itx2_fns! ( 8 x 32, 10 bpc, avx512icl);
    decl_itx2_fns! (16 x 32, 10 bpc, avx512icl);
    decl_itx2_fns! (32 x  8, 10 bpc, avx512icl);
    decl_itx2_fns! (32 x 16, 10 bpc, avx512icl);
    decl_itx2_fns! (32 x 32, 10 bpc, avx512icl);
    decl_itx1_fns! (16 x 64, 10 bpc, avx512icl);
    decl_itx1_fns! (32 x 64, 10 bpc, avx512icl);
    decl_itx1_fns! (64 x 16, 10 bpc, avx512icl);
    decl_itx1_fns! (64 x 32, 10 bpc, avx512icl);
    decl_itx1_fns! (64 x 64, 10 bpc, avx512icl);
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
extern "C" {
    decl_itx_fn!(wht, wht, 4 x 4, neon);
    decl_itx_fns!(neon);
}

macro_rules! inv_txfm_fn {
    ($type1:ident, $type2:ident, $w:literal, $h:literal, $shift:literal, $has_dconly:literal) => {
        paste::paste! {
            // TODO(legare): Temporarily pub until init fns are deduplicated.
            pub(crate) unsafe extern "C" fn [<inv_txfm_add_ $type1 _ $type2 _ $w x $h _c_erased>] <BD: BitDepth> (
                dst: *mut DynPixel,
                stride: ptrdiff_t,
                coeff: *mut DynCoef,
                eob: c_int,
                bitdepth_max: c_int,
            ) {
                use crate::src::itx_1d::*;
                inv_txfm_add_rust(
                    dst.cast(),
                    stride,
                    coeff.cast(),
                    eob,
                    $w,
                    $h,
                    $shift,
                    Some([<dav1d_inv_ $type1 $w _1d_c>]),
                    Some([<dav1d_inv_ $type2 $h _1d_c>]),
                    $has_dconly as c_int,
                    BD::from_c(bitdepth_max),
                );
            }
        }
    };
}

macro_rules! inv_txfm_fn64 {
    ($w:literal, $h:literal, $shift:literal) => {
        inv_txfm_fn!(dct, dct, $w, $h, $shift, true);
    };
}

macro_rules! inv_txfm_fn32 {
    ($w:literal, $h:literal, $shift:literal) => {
        inv_txfm_fn64!($w, $h, $shift);
        inv_txfm_fn!(identity, identity, $w, $h, $shift, false);
    };
}

macro_rules! inv_txfm_fn16 {
    ($w:literal, $h:literal, $shift:literal) => {
        inv_txfm_fn32!($w, $h, $shift);
        inv_txfm_fn!(adst, dct, $w, $h, $shift, false);
        inv_txfm_fn!(dct, adst, $w, $h, $shift, false);
        inv_txfm_fn!(adst, adst, $w, $h, $shift, false);
        inv_txfm_fn!(dct, flipadst, $w, $h, $shift, false);
        inv_txfm_fn!(flipadst, dct, $w, $h, $shift, false);
        inv_txfm_fn!(adst, flipadst, $w, $h, $shift, false);
        inv_txfm_fn!(flipadst, adst, $w, $h, $shift, false);
        inv_txfm_fn!(flipadst, flipadst, $w, $h, $shift, false);
        inv_txfm_fn!(identity, dct, $w, $h, $shift, false);
        inv_txfm_fn!(dct, identity, $w, $h, $shift, false);
    };
}

macro_rules! inv_txfm_fn84 {
    ($w:literal, $h:literal, $shift:literal) => {
        inv_txfm_fn16!($w, $h, $shift);
        inv_txfm_fn!(identity, flipadst, $w, $h, $shift, false);
        inv_txfm_fn!(flipadst, identity, $w, $h, $shift, false);
        inv_txfm_fn!(identity, adst, $w, $h, $shift, false);
        inv_txfm_fn!(adst, identity, $w, $h, $shift, false);
    };
}

inv_txfm_fn84!(4, 4, 0);
inv_txfm_fn84!(4, 8, 0);
inv_txfm_fn84!(4, 16, 1);
inv_txfm_fn84!(8, 4, 0);
inv_txfm_fn84!(8, 8, 1);
inv_txfm_fn84!(8, 16, 1);
inv_txfm_fn32!(8, 32, 2);
inv_txfm_fn84!(16, 4, 1);
inv_txfm_fn84!(16, 8, 1);
inv_txfm_fn16!(16, 16, 2);
inv_txfm_fn32!(16, 32, 1);
inv_txfm_fn64!(16, 64, 2);
inv_txfm_fn32!(32, 8, 2);
inv_txfm_fn32!(32, 16, 1);
inv_txfm_fn32!(32, 32, 2);
inv_txfm_fn64!(32, 64, 1);
inv_txfm_fn64!(64, 16, 2);
inv_txfm_fn64!(64, 32, 1);
inv_txfm_fn64!(64, 64, 2);

// TODO(perl): Temporarily pub until mod is deduplicated
pub(crate) unsafe extern "C" fn inv_txfm_add_wht_wht_4x4_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    coeff: *mut DynCoef,
    eob: c_int,
    bitdepth_max: c_int,
) {
    inv_txfm_add_wht_wht_4x4_rust::<BD>(
        dst.cast(),
        stride,
        coeff.cast(),
        eob,
        BD::from_c(bitdepth_max),
    );
}

unsafe fn inv_txfm_add_wht_wht_4x4_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    coeff: *mut BD::Coef,
    _eob: c_int,
    bd: BD,
) {
    use crate::src::itx_1d::dav1d_inv_wht4_1d_c;

    let mut tmp: [i32; 16] = [0; 16];
    let mut c: *mut i32 = tmp.as_mut_ptr();
    let mut y = 0;
    while y < 4 {
        let mut x = 0;
        while x < 4 {
            *c.offset(x as isize) = (*coeff.offset((y + x * 4) as isize)).as_::<i32>() >> 2;
            x += 1;
        }
        dav1d_inv_wht4_1d_c(c, 1 as c_int as ptrdiff_t);
        y += 1;
        c = c.offset(4);
    }
    memset(
        coeff as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<BD::Coef>()
            .wrapping_mul(4)
            .wrapping_mul(4),
    );
    let mut x_0 = 0;
    while x_0 < 4 {
        dav1d_inv_wht4_1d_c(
            &mut *tmp.as_mut_ptr().offset(x_0 as isize),
            4 as c_int as ptrdiff_t,
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
                bd.iclip_pixel((*dst.offset(x_1 as isize)).as_::<c_int>() + *fresh1);
            x_1 += 1;
        }
        y_0 += 1;
        dst = dst.offset(BD::pxstride(stride as usize) as isize);
    }
}
