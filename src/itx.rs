use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynCoef;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::intops::iclip;
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
use crate::src::levels::N_RECT_TX_SIZES;
use crate::src::levels::N_TX_TYPES_PLUS_LL;
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
use crate::src::levels::TX_16X16;
use crate::src::levels::TX_32X32;
use crate::src::levels::TX_4X4;
use crate::src::levels::TX_64X64;
use crate::src::levels::TX_8X8;
use crate::src::levels::V_ADST;
use crate::src::levels::V_DCT;
use crate::src::levels::V_FLIPADST;
use crate::src::levels::WHT_WHT;
use libc::memset;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_void;

#[cfg(feature = "asm")]
use crate::src::cpu::{rav1d_get_cpu_flags, CpuFlags};

#[cfg(feature = "asm")]
use cfg_if::cfg_if;

#[cfg(feature = "asm")]
use crate::include::common::bitdepth::bd_fn;
#[cfg(feature = "asm")]
use crate::include::common::bitdepth::bpc_fn;

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
        fn $name(
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
            unsafe extern "C" fn [<inv_txfm_add_ $type1 _ $type2 _ $w x $h _c_erased>] <BD: BitDepth> (
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

#[cfg(feature = "asm")]
macro_rules! assign_itx_fn {
    ($c:ident, $BD:ty, $w:literal, $h:literal, $type:ident, $type_enum:ident, $ext:ident) => {{
        use paste::paste;

        paste! {
            (*$c).itxfm_add[[<TX_ $w X $h>] as usize][$type_enum as usize]
                = Some(bd_fn!(BD, [< inv_txfm_add_ $type _ $w x $h >], $ext));
        }
    }};
}

#[cfg(feature = "asm")]
macro_rules! assign_itx_bpc_fn {
    ($c:ident, $pfx:ident, $w:literal, $h:literal, $type:ident, $type_enum:ident, $bpc:literal bpc, $ext:ident) => {{
        use paste::paste;

        paste! {
            (*$c).itxfm_add[[<$pfx TX_ $w X $h>] as usize][$type_enum as usize]
                = Some(bpc_fn!($bpc bpc, [< inv_txfm_add_ $type _ $w x $h >], $ext));
        }
    }};

    ($c:ident, $w:literal, $h:literal, $type:ident, $type_enum:ident, $bpc:literal bpc, $ext:ident) => {{
        use paste::paste;

        paste! {
            (*$c).itxfm_add[[<TX_ $w X $h>] as usize][$type_enum as usize]
                = Some(bpc_fn!($bpc bpc, [< inv_txfm_add_ $type _ $w x $h >], $ext));
        }
    }};
}

#[cfg(feature = "asm")]
macro_rules! assign_itx1_bpc_fn {
    ($c:ident, $w:literal, $h:literal, $bpc:literal bpc, $ext:ident) => {{
        assign_itx_bpc_fn!($c, $w, $h, dct_dct, DCT_DCT, $bpc bpc, $ext)
    }};

    ($c:ident, $pfx:ident, $w:literal, $h:literal, $bpc:literal bpc, $ext:ident) => {{
        assign_itx_bpc_fn!($c, $pfx, $w, $h, dct_dct, DCT_DCT, $bpc bpc, $ext)
    }};
}

#[cfg(feature = "asm")]
macro_rules! assign_itx2_bpc_fn {
    ($c:ident, $w:literal, $h:literal, $bpc:literal bpc, $ext:ident) => {{
        assign_itx1_bpc_fn!($c, $w, $h, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $w, $h, identity_identity, IDTX, $bpc bpc, $ext)
    }};

    ($c:ident, $pfx:ident, $w:literal, $h:literal, $bpc:literal bpc, $ext:ident) => {{
        assign_itx1_bpc_fn!($c, $pfx, $w, $h, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $pfx, $w, $h, identity_identity, IDTX, $bpc bpc, $ext)
    }};
}

#[cfg(feature = "asm")]
macro_rules! assign_itx12_bpc_fn {
    ($c:ident, $w:literal, $h:literal, $bpc:literal bpc, $ext:ident) => {{
        assign_itx2_bpc_fn!($c, $w, $h, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $w, $h, dct_adst, ADST_DCT, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $w, $h, dct_flipadst, FLIPADST_DCT, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $w, $h, dct_identity, H_DCT, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $w, $h, adst_dct, DCT_ADST, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $w, $h, adst_adst, ADST_ADST, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $w, $h, adst_flipadst, FLIPADST_ADST, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $w, $h, flipadst_dct, DCT_FLIPADST, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $w, $h, flipadst_adst, ADST_FLIPADST, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $w, $h, flipadst_flipadst, FLIPADST_FLIPADST, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $w, $h, identity_dct, V_DCT, $bpc bpc, $ext);

    }};

    ($c:ident, $pfx:ident, $w:literal, $h:literal, $bpc:literal bpc, $ext:ident) => {{
        assign_itx2_bpc_fn!($c, $pfx, $w, $h, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $pfx, $w, $h, dct_adst, ADST_DCT, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $pfx, $w, $h, dct_flipadst, FLIPADST_DCT, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $pfx, $w, $h, dct_identity, H_DCT, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $pfx, $w, $h, adst_dct, DCT_ADST, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $pfx, $w, $h, adst_adst, ADST_ADST, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $pfx, $w, $h, adst_flipadst, FLIPADST_ADST, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $pfx, $w, $h, flipadst_dct, DCT_FLIPADST, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $pfx, $w, $h, flipadst_adst, ADST_FLIPADST, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $pfx, $w, $h, flipadst_flipadst, FLIPADST_FLIPADST, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $pfx, $w, $h, identity_dct, V_DCT, $bpc bpc, $ext);
    }};
}

#[cfg(feature = "asm")]
macro_rules! assign_itx16_bpc_fn {
    ($c:ident, $w:literal, $h:literal, $bpc:literal bpc, $ext:ident) => {{
        assign_itx12_bpc_fn!($c, $w, $h, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $w, $h, adst_identity, H_ADST, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $w, $h, flipadst_identity, H_FLIPADST, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $w, $h, identity_adst, V_ADST, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $w, $h, identity_flipadst, V_FLIPADST, $bpc bpc, $ext);
    }};

    ($c:ident, $pfx:ident, $w:literal, $h:literal, $bpc:literal bpc, $ext:ident) => {{
        assign_itx12_bpc_fn!($c, $pfx, $w, $h, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $pfx, $w, $h, adst_identity, H_ADST, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $pfx, $w, $h, flipadst_identity, H_FLIPADST, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $pfx, $w, $h, identity_adst, V_ADST, $bpc bpc, $ext);
        assign_itx_bpc_fn!($c, $pfx, $w, $h, identity_flipadst, V_FLIPADST, $bpc bpc, $ext);
    }};
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
#[inline(always)]
#[rustfmt::skip]
unsafe fn itx_dsp_init_x86<BD: BitDepth>(c: *mut Rav1dInvTxfmDSPContext, bpc: c_int) {

    let flags = rav1d_get_cpu_flags();

    if !flags.contains(CpuFlags::SSE2) {
        return;
    }

    assign_itx_fn!(c, BD, 4, 4, wht_wht, WHT_WHT, sse2);

    if !flags.contains(CpuFlags::SSSE3) {
        return;
    }

    if BD::BITDEPTH == 8 {
        assign_itx16_bpc_fn!(c,     4,  4, 8 bpc, ssse3);
        assign_itx16_bpc_fn!(c, R,  4,  8, 8 bpc, ssse3);
        assign_itx16_bpc_fn!(c, R,  8,  4, 8 bpc, ssse3);
        assign_itx16_bpc_fn!(c,     8,  8, 8 bpc, ssse3);
        assign_itx16_bpc_fn!(c, R,  4, 16, 8 bpc, ssse3);
        assign_itx16_bpc_fn!(c, R, 16,  4, 8 bpc, ssse3);
        assign_itx16_bpc_fn!(c, R,  8, 16, 8 bpc, ssse3);
        assign_itx16_bpc_fn!(c, R, 16,  8, 8 bpc, ssse3);
        assign_itx12_bpc_fn!(c,    16, 16, 8 bpc, ssse3);
        assign_itx2_bpc_fn! (c, R,  8, 32, 8 bpc, ssse3);
        assign_itx2_bpc_fn! (c, R, 32,  8, 8 bpc, ssse3);
        assign_itx2_bpc_fn! (c, R, 16, 32, 8 bpc, ssse3);
        assign_itx2_bpc_fn! (c, R, 32, 16, 8 bpc, ssse3);
        assign_itx2_bpc_fn! (c,    32, 32, 8 bpc, ssse3);
        assign_itx1_bpc_fn! (c, R, 16, 64, 8 bpc, ssse3);
        assign_itx1_bpc_fn! (c, R, 32, 64, 8 bpc, ssse3);
        assign_itx1_bpc_fn! (c, R, 64, 16, 8 bpc, ssse3);
        assign_itx1_bpc_fn! (c, R, 64, 32, 8 bpc, ssse3);
        assign_itx1_bpc_fn! (c,    64, 64, 8 bpc, ssse3);
    }

    if !flags.contains(CpuFlags::SSE41) {
        return;
    }

    if BD::BITDEPTH == 16 {
        if bpc == 10 {
            assign_itx16_bpc_fn!(c,     4,  4, 16 bpc, sse4);
            assign_itx16_bpc_fn!(c, R,  4,  8, 16 bpc, sse4);
            assign_itx16_bpc_fn!(c, R,  4, 16, 16 bpc, sse4);
            assign_itx16_bpc_fn!(c, R,  8,  4, 16 bpc, sse4);
            assign_itx16_bpc_fn!(c,     8,  8, 16 bpc, sse4);
            assign_itx16_bpc_fn!(c, R,  8, 16, 16 bpc, sse4);
            assign_itx16_bpc_fn!(c, R, 16,  4, 16 bpc, sse4);
            assign_itx16_bpc_fn!(c, R, 16,  8, 16 bpc, sse4);
            assign_itx12_bpc_fn!(c,    16, 16, 16 bpc, sse4);
            assign_itx2_bpc_fn! (c, R,  8, 32, 16 bpc, sse4);
            assign_itx2_bpc_fn! (c, R, 16, 32, 16 bpc, sse4);
            assign_itx2_bpc_fn! (c, R, 32,  8, 16 bpc, sse4);
            assign_itx2_bpc_fn! (c, R, 32, 16, 16 bpc, sse4);
            assign_itx2_bpc_fn! (c,    32, 32, 16 bpc, sse4);
            assign_itx1_bpc_fn! (c, R, 16, 64, 16 bpc, sse4);
            assign_itx1_bpc_fn! (c, R, 32, 64, 16 bpc, sse4);
            assign_itx1_bpc_fn! (c, R, 64, 16, 16 bpc, sse4);
            assign_itx1_bpc_fn! (c, R, 64, 32, 16 bpc, sse4);
            assign_itx1_bpc_fn! (c,    64, 64, 16 bpc, sse4);
        }
    }

    #[cfg(target_arch = "x86_64")]
    {
        if !flags.contains(CpuFlags::AVX2) {
            return;
        }

        assign_itx_fn!(c, BD, 4, 4, wht_wht, WHT_WHT, avx2);

        if BD::BITDEPTH == 8 {
            assign_itx16_bpc_fn!(c,     4,  4, 8 bpc, avx2);
            assign_itx16_bpc_fn!(c, R,  4,  8, 8 bpc, avx2);
            assign_itx16_bpc_fn!(c, R,  4, 16, 8 bpc, avx2);
            assign_itx16_bpc_fn!(c, R,  8,  4, 8 bpc, avx2);
            assign_itx16_bpc_fn!(c,     8,  8, 8 bpc, avx2);
            assign_itx16_bpc_fn!(c, R,  8, 16, 8 bpc, avx2);
            assign_itx16_bpc_fn!(c, R, 16,  4, 8 bpc, avx2);
            assign_itx16_bpc_fn!(c, R, 16,  8, 8 bpc, avx2);
            assign_itx12_bpc_fn!(c,    16, 16, 8 bpc, avx2);
            assign_itx2_bpc_fn! (c, R,  8, 32, 8 bpc, avx2);
            assign_itx2_bpc_fn! (c, R, 16, 32, 8 bpc, avx2);
            assign_itx2_bpc_fn! (c, R, 32,  8, 8 bpc, avx2);
            assign_itx2_bpc_fn! (c, R, 32, 16, 8 bpc, avx2);
            assign_itx2_bpc_fn! (c,    32, 32, 8 bpc, avx2);
            assign_itx1_bpc_fn! (c, R, 16, 64, 8 bpc, avx2);
            assign_itx1_bpc_fn! (c, R, 32, 64, 8 bpc, avx2);
            assign_itx1_bpc_fn! (c, R, 64, 16, 8 bpc, avx2);
            assign_itx1_bpc_fn! (c, R, 64, 32, 8 bpc, avx2);
            assign_itx1_bpc_fn! (c,    64, 64, 8 bpc, avx2);
        } else {
            if bpc == 10 {
                assign_itx16_bpc_fn!(c,     4,  4, 10 bpc, avx2);
                assign_itx16_bpc_fn!(c, R,  4,  8, 10 bpc, avx2);
                assign_itx16_bpc_fn!(c, R,  4, 16, 10 bpc, avx2);
                assign_itx16_bpc_fn!(c, R,  8,  4, 10 bpc, avx2);
                assign_itx16_bpc_fn!(c,     8,  8, 10 bpc, avx2);
                assign_itx16_bpc_fn!(c, R,  8, 16, 10 bpc, avx2);
                assign_itx16_bpc_fn!(c, R, 16,  4, 10 bpc, avx2);
                assign_itx16_bpc_fn!(c, R, 16,  8, 10 bpc, avx2);
                assign_itx12_bpc_fn!(c,    16, 16, 10 bpc, avx2);
                assign_itx2_bpc_fn! (c, R,  8, 32, 10 bpc, avx2);
                assign_itx2_bpc_fn! (c, R, 16, 32, 10 bpc, avx2);
                assign_itx2_bpc_fn! (c, R, 32,  8, 10 bpc, avx2);
                assign_itx2_bpc_fn! (c, R, 32, 16, 10 bpc, avx2);
                assign_itx2_bpc_fn! (c,    32, 32, 10 bpc, avx2);
                assign_itx1_bpc_fn! (c, R, 16, 64, 10 bpc, avx2);
                assign_itx1_bpc_fn! (c, R, 32, 64, 10 bpc, avx2);
                assign_itx1_bpc_fn! (c, R, 64, 16, 10 bpc, avx2);
                assign_itx1_bpc_fn! (c, R, 64, 32, 10 bpc, avx2);
                assign_itx1_bpc_fn! (c,    64, 64, 10 bpc, avx2);
            } else {
                assign_itx16_bpc_fn!(c,     4,  4, 12 bpc, avx2);
                assign_itx16_bpc_fn!(c, R,  4,  8, 12 bpc, avx2);
                assign_itx16_bpc_fn!(c, R,  4, 16, 12 bpc, avx2);
                assign_itx16_bpc_fn!(c, R,  8,  4, 12 bpc, avx2);
                assign_itx16_bpc_fn!(c,     8,  8, 12 bpc, avx2);
                assign_itx16_bpc_fn!(c, R,  8, 16, 12 bpc, avx2);
                assign_itx16_bpc_fn!(c, R, 16,  4, 12 bpc, avx2);
                assign_itx16_bpc_fn!(c, R, 16,  8, 12 bpc, avx2);
                assign_itx12_bpc_fn!(c,    16, 16, 12 bpc, avx2);
                assign_itx2_bpc_fn! (c, R,  8, 32, 12 bpc, avx2);
                assign_itx2_bpc_fn! (c, R, 32,  8, 12 bpc, avx2);
                assign_itx_bpc_fn!  (c, R, 16, 32, identity_identity, IDTX, 12 bpc, avx2);
                assign_itx_bpc_fn!  (c, R, 32, 16, identity_identity, IDTX, 12 bpc, avx2);
                assign_itx_bpc_fn!  (c,    32, 32, identity_identity, IDTX, 12 bpc, avx2);
            }
        }

        if !flags.contains(CpuFlags::AVX512ICL) {
            return;
        }

        if BD::BITDEPTH == 8 {
            assign_itx16_bpc_fn!(c,     4,  4, 8 bpc, avx512icl); // no wht
            assign_itx16_bpc_fn!(c, R,  4,  8, 8 bpc, avx512icl);
            assign_itx16_bpc_fn!(c, R,  4, 16, 8 bpc, avx512icl);
            assign_itx16_bpc_fn!(c, R,  8,  4, 8 bpc, avx512icl);
            assign_itx16_bpc_fn!(c,     8,  8, 8 bpc, avx512icl);
            assign_itx16_bpc_fn!(c, R,  8, 16, 8 bpc, avx512icl);
            assign_itx16_bpc_fn!(c, R, 16,  4, 8 bpc, avx512icl);
            assign_itx16_bpc_fn!(c, R, 16,  8, 8 bpc, avx512icl);
            assign_itx12_bpc_fn!(c,    16, 16, 8 bpc, avx512icl);
            assign_itx2_bpc_fn! (c, R,  8, 32, 8 bpc, avx512icl);
            assign_itx2_bpc_fn! (c, R, 16, 32, 8 bpc, avx512icl);
            assign_itx2_bpc_fn! (c, R, 32,  8, 8 bpc, avx512icl);
            assign_itx2_bpc_fn! (c, R, 32, 16, 8 bpc, avx512icl);
            assign_itx2_bpc_fn! (c,    32, 32, 8 bpc, avx512icl);
            assign_itx1_bpc_fn! (c, R, 16, 64, 8 bpc, avx512icl);
            assign_itx1_bpc_fn! (c, R, 32, 64, 8 bpc, avx512icl);
            assign_itx1_bpc_fn! (c, R, 64, 16, 8 bpc, avx512icl);
            assign_itx1_bpc_fn! (c, R, 64, 32, 8 bpc, avx512icl);
            assign_itx1_bpc_fn! (c,    64, 64, 8 bpc, avx512icl);
        } else {
            if bpc == 10 {
                assign_itx16_bpc_fn!(c,     8,  8, 10 bpc, avx512icl);
                assign_itx16_bpc_fn!(c, R,  8, 16, 10 bpc, avx512icl);
                assign_itx16_bpc_fn!(c, R, 16,  8, 10 bpc, avx512icl);
                assign_itx12_bpc_fn!(c,    16, 16, 10 bpc, avx512icl);
                assign_itx2_bpc_fn! (c, R,  8, 32, 10 bpc, avx512icl);
                assign_itx2_bpc_fn! (c, R, 16, 32, 10 bpc, avx512icl);
                assign_itx2_bpc_fn! (c, R, 32,  8, 10 bpc, avx512icl);
                assign_itx2_bpc_fn! (c, R, 32, 16, 10 bpc, avx512icl);
                assign_itx2_bpc_fn! (c,    32, 32, 10 bpc, avx512icl);
                assign_itx1_bpc_fn! (c, R, 16, 64, 10 bpc, avx512icl);
                assign_itx1_bpc_fn! (c, R, 32, 64, 10 bpc, avx512icl);
                assign_itx1_bpc_fn! (c, R, 64, 16, 10 bpc, avx512icl);
                assign_itx1_bpc_fn! (c, R, 64, 32, 10 bpc, avx512icl);
                assign_itx1_bpc_fn! (c,    64, 64, 10 bpc, avx512icl);
            }
        }
    }
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
#[inline(always)]
#[rustfmt::skip]
unsafe fn itx_dsp_init_arm<BD: BitDepth>(c: *mut Rav1dInvTxfmDSPContext, mut bpc: c_int) {
    let flags = rav1d_get_cpu_flags();

    if !flags.contains(CpuFlags::NEON) {
        return;
    }

    if BD::BITDEPTH == 16 && bpc != 10 {
        return;
    }

    (*c).itxfm_add[TX_4X4 as usize][DCT_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_dct_4x4, neon));
    (*c).itxfm_add[TX_4X4 as usize][IDTX as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_identity_4x4, neon));
    (*c).itxfm_add[TX_4X4 as usize][ADST_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_adst_4x4, neon));
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_flipadst_4x4, neon));
    (*c).itxfm_add[TX_4X4 as usize][H_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_identity_4x4, neon));
    (*c).itxfm_add[TX_4X4 as usize][DCT_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_dct_4x4, neon));
    (*c).itxfm_add[TX_4X4 as usize][ADST_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_adst_4x4, neon));
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_flipadst_4x4, neon));
    (*c).itxfm_add[TX_4X4 as usize][DCT_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_dct_4x4, neon));
    (*c).itxfm_add[TX_4X4 as usize][ADST_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_adst_4x4, neon));
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_flipadst_4x4, neon));
    (*c).itxfm_add[TX_4X4 as usize][V_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_dct_4x4, neon));
    (*c).itxfm_add[TX_4X4 as usize][H_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_identity_4x4, neon));
    (*c).itxfm_add[TX_4X4 as usize][H_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_identity_4x4, neon));
    (*c).itxfm_add[TX_4X4 as usize][V_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_adst_4x4, neon));
    (*c).itxfm_add[TX_4X4 as usize][V_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_flipadst_4x4, neon));
    (*c).itxfm_add[TX_4X4 as usize][WHT_WHT as usize] = Some(bd_fn!(BD, inv_txfm_add_wht_wht_4x4, neon));
    (*c).itxfm_add[RTX_4X8 as usize][DCT_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_dct_4x8, neon));
    (*c).itxfm_add[RTX_4X8 as usize][IDTX as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_identity_4x8, neon));
    (*c).itxfm_add[RTX_4X8 as usize][ADST_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_adst_4x8, neon));
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_flipadst_4x8, neon));
    (*c).itxfm_add[RTX_4X8 as usize][H_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_identity_4x8, neon));
    (*c).itxfm_add[RTX_4X8 as usize][DCT_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_dct_4x8, neon));
    (*c).itxfm_add[RTX_4X8 as usize][ADST_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_adst_4x8, neon));
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_flipadst_4x8, neon));
    (*c).itxfm_add[RTX_4X8 as usize][DCT_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_dct_4x8, neon));
    (*c).itxfm_add[RTX_4X8 as usize][ADST_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_adst_4x8, neon));
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_flipadst_4x8, neon));
    (*c).itxfm_add[RTX_4X8 as usize][V_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_dct_4x8, neon));
    (*c).itxfm_add[RTX_4X8 as usize][H_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_identity_4x8, neon));
    (*c).itxfm_add[RTX_4X8 as usize][H_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_identity_4x8, neon));
    (*c).itxfm_add[RTX_4X8 as usize][V_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_adst_4x8, neon));
    (*c).itxfm_add[RTX_4X8 as usize][V_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_flipadst_4x8, neon));
    (*c).itxfm_add[RTX_4X16 as usize][DCT_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_dct_4x16, neon));
    (*c).itxfm_add[RTX_4X16 as usize][IDTX as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_identity_4x16, neon));
    (*c).itxfm_add[RTX_4X16 as usize][ADST_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_adst_4x16, neon));
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_flipadst_4x16, neon));
    (*c).itxfm_add[RTX_4X16 as usize][H_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_identity_4x16, neon));
    (*c).itxfm_add[RTX_4X16 as usize][DCT_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_dct_4x16, neon));
    (*c).itxfm_add[RTX_4X16 as usize][ADST_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_adst_4x16, neon));
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_flipadst_4x16, neon));
    (*c).itxfm_add[RTX_4X16 as usize][DCT_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_dct_4x16, neon));
    (*c).itxfm_add[RTX_4X16 as usize][ADST_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_adst_4x16, neon));
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_flipadst_4x16, neon));
    (*c).itxfm_add[RTX_4X16 as usize][V_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_dct_4x16, neon));
    (*c).itxfm_add[RTX_4X16 as usize][H_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_identity_4x16, neon));
    (*c).itxfm_add[RTX_4X16 as usize][H_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_identity_4x16, neon));
    (*c).itxfm_add[RTX_4X16 as usize][V_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_adst_4x16, neon));
    (*c).itxfm_add[RTX_4X16 as usize][V_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_flipadst_4x16, neon));
    (*c).itxfm_add[RTX_8X4 as usize][DCT_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_dct_8x4, neon));
    (*c).itxfm_add[RTX_8X4 as usize][IDTX as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_identity_8x4, neon));
    (*c).itxfm_add[RTX_8X4 as usize][ADST_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_adst_8x4, neon));
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_flipadst_8x4, neon));
    (*c).itxfm_add[RTX_8X4 as usize][H_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_identity_8x4, neon));
    (*c).itxfm_add[RTX_8X4 as usize][DCT_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_dct_8x4, neon));
    (*c).itxfm_add[RTX_8X4 as usize][ADST_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_adst_8x4, neon));
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_flipadst_8x4, neon));
    (*c).itxfm_add[RTX_8X4 as usize][DCT_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_dct_8x4, neon));
    (*c).itxfm_add[RTX_8X4 as usize][ADST_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_adst_8x4, neon));
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_flipadst_8x4, neon));
    (*c).itxfm_add[RTX_8X4 as usize][V_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_dct_8x4, neon));
    (*c).itxfm_add[RTX_8X4 as usize][H_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_identity_8x4, neon));
    (*c).itxfm_add[RTX_8X4 as usize][H_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_identity_8x4, neon));
    (*c).itxfm_add[RTX_8X4 as usize][V_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_adst_8x4, neon));
    (*c).itxfm_add[RTX_8X4 as usize][V_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_flipadst_8x4, neon));
    (*c).itxfm_add[TX_8X8 as usize][DCT_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_dct_8x8, neon));
    (*c).itxfm_add[TX_8X8 as usize][IDTX as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_identity_8x8, neon));
    (*c).itxfm_add[TX_8X8 as usize][ADST_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_adst_8x8, neon));
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_flipadst_8x8, neon));
    (*c).itxfm_add[TX_8X8 as usize][H_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_identity_8x8, neon));
    (*c).itxfm_add[TX_8X8 as usize][DCT_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_dct_8x8, neon));
    (*c).itxfm_add[TX_8X8 as usize][ADST_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_adst_8x8, neon));
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_flipadst_8x8, neon));
    (*c).itxfm_add[TX_8X8 as usize][DCT_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_dct_8x8, neon));
    (*c).itxfm_add[TX_8X8 as usize][ADST_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_adst_8x8, neon));
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_flipadst_8x8, neon));
    (*c).itxfm_add[TX_8X8 as usize][V_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_dct_8x8, neon));
    (*c).itxfm_add[TX_8X8 as usize][H_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_identity_8x8, neon));
    (*c).itxfm_add[TX_8X8 as usize][H_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_identity_8x8, neon));
    (*c).itxfm_add[TX_8X8 as usize][V_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_adst_8x8, neon));
    (*c).itxfm_add[TX_8X8 as usize][V_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_flipadst_8x8, neon));
    (*c).itxfm_add[RTX_8X16 as usize][DCT_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_dct_8x16, neon));
    (*c).itxfm_add[RTX_8X16 as usize][IDTX as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_identity_8x16, neon));
    (*c).itxfm_add[RTX_8X16 as usize][ADST_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_adst_8x16, neon));
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_flipadst_8x16, neon));
    (*c).itxfm_add[RTX_8X16 as usize][H_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_identity_8x16, neon));
    (*c).itxfm_add[RTX_8X16 as usize][DCT_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_dct_8x16, neon));
    (*c).itxfm_add[RTX_8X16 as usize][ADST_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_adst_8x16, neon));
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_flipadst_8x16, neon));
    (*c).itxfm_add[RTX_8X16 as usize][DCT_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_dct_8x16, neon));
    (*c).itxfm_add[RTX_8X16 as usize][ADST_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_adst_8x16, neon));
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_flipadst_8x16, neon));
    (*c).itxfm_add[RTX_8X16 as usize][V_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_dct_8x16, neon));
    (*c).itxfm_add[RTX_8X16 as usize][H_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_identity_8x16, neon));
    (*c).itxfm_add[RTX_8X16 as usize][H_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_identity_8x16, neon));
    (*c).itxfm_add[RTX_8X16 as usize][V_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_adst_8x16, neon));
    (*c).itxfm_add[RTX_8X16 as usize][V_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_flipadst_8x16, neon));
    (*c).itxfm_add[RTX_8X32 as usize][DCT_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_dct_8x32, neon));
    (*c).itxfm_add[RTX_8X32 as usize][IDTX as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_identity_8x32, neon));
    (*c).itxfm_add[RTX_16X4 as usize][DCT_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_dct_16x4, neon));
    (*c).itxfm_add[RTX_16X4 as usize][IDTX as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_identity_16x4, neon));
    (*c).itxfm_add[RTX_16X4 as usize][ADST_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_adst_16x4, neon));
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_flipadst_16x4, neon));
    (*c).itxfm_add[RTX_16X4 as usize][H_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_identity_16x4, neon));
    (*c).itxfm_add[RTX_16X4 as usize][DCT_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_dct_16x4, neon));
    (*c).itxfm_add[RTX_16X4 as usize][ADST_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_adst_16x4, neon));
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_flipadst_16x4, neon));
    (*c).itxfm_add[RTX_16X4 as usize][DCT_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_dct_16x4, neon));
    (*c).itxfm_add[RTX_16X4 as usize][ADST_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_adst_16x4, neon));
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_flipadst_16x4, neon));
    (*c).itxfm_add[RTX_16X4 as usize][V_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_dct_16x4, neon));
    (*c).itxfm_add[RTX_16X4 as usize][H_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_identity_16x4, neon));
    (*c).itxfm_add[RTX_16X4 as usize][H_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_identity_16x4, neon));
    (*c).itxfm_add[RTX_16X4 as usize][V_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_adst_16x4, neon));
    (*c).itxfm_add[RTX_16X4 as usize][V_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_flipadst_16x4, neon));
    (*c).itxfm_add[RTX_16X8 as usize][DCT_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_dct_16x8, neon));
    (*c).itxfm_add[RTX_16X8 as usize][IDTX as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_identity_16x8, neon));
    (*c).itxfm_add[RTX_16X8 as usize][ADST_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_adst_16x8, neon));
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_flipadst_16x8, neon));
    (*c).itxfm_add[RTX_16X8 as usize][H_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_identity_16x8, neon));
    (*c).itxfm_add[RTX_16X8 as usize][DCT_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_dct_16x8, neon));
    (*c).itxfm_add[RTX_16X8 as usize][ADST_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_adst_16x8, neon));
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_flipadst_16x8, neon));
    (*c).itxfm_add[RTX_16X8 as usize][DCT_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_dct_16x8, neon));
    (*c).itxfm_add[RTX_16X8 as usize][ADST_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_adst_16x8, neon));
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_flipadst_16x8, neon));
    (*c).itxfm_add[RTX_16X8 as usize][V_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_dct_16x8, neon));
    (*c).itxfm_add[RTX_16X8 as usize][H_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_identity_16x8, neon));
    (*c).itxfm_add[RTX_16X8 as usize][H_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_identity_16x8, neon));
    (*c).itxfm_add[RTX_16X8 as usize][V_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_adst_16x8, neon));
    (*c).itxfm_add[RTX_16X8 as usize][V_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_flipadst_16x8, neon));
    (*c).itxfm_add[TX_16X16 as usize][DCT_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_dct_16x16, neon));
    (*c).itxfm_add[TX_16X16 as usize][IDTX as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_identity_16x16, neon));
    (*c).itxfm_add[TX_16X16 as usize][ADST_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_adst_16x16, neon));
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_flipadst_16x16, neon));
    (*c).itxfm_add[TX_16X16 as usize][H_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_identity_16x16, neon));
    (*c).itxfm_add[TX_16X16 as usize][DCT_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_dct_16x16, neon));
    (*c).itxfm_add[TX_16X16 as usize][ADST_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_adst_16x16, neon));
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_ADST as usize] = Some(bd_fn!(BD, inv_txfm_add_adst_flipadst_16x16, neon));
    (*c).itxfm_add[TX_16X16 as usize][DCT_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_dct_16x16, neon));
    (*c).itxfm_add[TX_16X16 as usize][ADST_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_adst_16x16, neon));
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_FLIPADST as usize] = Some(bd_fn!(BD, inv_txfm_add_flipadst_flipadst_16x16, neon));
    (*c).itxfm_add[TX_16X16 as usize][V_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_dct_16x16, neon));
    (*c).itxfm_add[RTX_16X32 as usize][DCT_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_dct_16x32, neon));
    (*c).itxfm_add[RTX_16X32 as usize][IDTX as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_identity_16x32, neon));
    (*c).itxfm_add[RTX_16X64 as usize][DCT_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_dct_16x64, neon));
    (*c).itxfm_add[RTX_32X8 as usize][DCT_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_dct_32x8, neon));
    (*c).itxfm_add[RTX_32X8 as usize][IDTX as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_identity_32x8, neon));
    (*c).itxfm_add[RTX_32X16 as usize][DCT_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_dct_32x16, neon));
    (*c).itxfm_add[RTX_32X16 as usize][IDTX as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_identity_32x16, neon));
    (*c).itxfm_add[TX_32X32 as usize][DCT_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_dct_32x32, neon));
    (*c).itxfm_add[TX_32X32 as usize][IDTX as usize] = Some(bd_fn!(BD, inv_txfm_add_identity_identity_32x32, neon));
    (*c).itxfm_add[RTX_32X64 as usize][DCT_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_dct_32x64, neon));
    (*c).itxfm_add[RTX_64X16 as usize][DCT_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_dct_64x16, neon));
    (*c).itxfm_add[RTX_64X32 as usize][DCT_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_dct_64x32, neon));
    (*c).itxfm_add[TX_64X64 as usize][DCT_DCT as usize] = Some(bd_fn!(BD, inv_txfm_add_dct_dct_64x64, neon));
}

#[cold]
#[rustfmt::skip]
pub unsafe fn rav1d_itx_dsp_init<BD: BitDepth>(c: *mut Rav1dInvTxfmDSPContext, mut _bpc: c_int) {
    (*c).itxfm_add[TX_4X4 as usize][WHT_WHT as usize] = Some(inv_txfm_add_wht_wht_4x4_c_erased::<BD>);
    (*c).itxfm_add[TX_4X4 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_4x4_c_erased::<BD>);
    (*c).itxfm_add[TX_4X4 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_4x4_c_erased::<BD>);
    (*c).itxfm_add[TX_4X4 as usize][DCT_ADST as usize] =
        Some(inv_txfm_add_adst_dct_4x4_c_erased::<BD>);
    (*c).itxfm_add[TX_4X4 as usize][ADST_DCT as usize] =
        Some(inv_txfm_add_dct_adst_4x4_c_erased::<BD>);
    (*c).itxfm_add[TX_4X4 as usize][ADST_ADST as usize] =
        Some(inv_txfm_add_adst_adst_4x4_c_erased::<BD>);
    (*c).itxfm_add[TX_4X4 as usize][ADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_adst_4x4_c_erased::<BD>);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_ADST as usize] =
        Some(inv_txfm_add_adst_flipadst_4x4_c_erased::<BD>);
    (*c).itxfm_add[TX_4X4 as usize][DCT_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_dct_4x4_c_erased::<BD>);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_DCT as usize] =
        Some(inv_txfm_add_dct_flipadst_4x4_c_erased::<BD>);
    (*c).itxfm_add[TX_4X4 as usize][FLIPADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_flipadst_4x4_c_erased::<BD>);
    (*c).itxfm_add[TX_4X4 as usize][H_DCT as usize] =
        Some(inv_txfm_add_dct_identity_4x4_c_erased::<BD>);
    (*c).itxfm_add[TX_4X4 as usize][V_DCT as usize] =
        Some(inv_txfm_add_identity_dct_4x4_c_erased::<BD>);
    (*c).itxfm_add[TX_4X4 as usize][H_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_identity_4x4_c_erased::<BD>);
    (*c).itxfm_add[TX_4X4 as usize][V_FLIPADST as usize] =
        Some(inv_txfm_add_identity_flipadst_4x4_c_erased::<BD>);
    (*c).itxfm_add[TX_4X4 as usize][H_ADST as usize] =
        Some(inv_txfm_add_adst_identity_4x4_c_erased::<BD>);
    (*c).itxfm_add[TX_4X4 as usize][V_ADST as usize] =
        Some(inv_txfm_add_identity_adst_4x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_4x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X8 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_4x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_ADST as usize] =
        Some(inv_txfm_add_adst_dct_4x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_DCT as usize] =
        Some(inv_txfm_add_dct_adst_4x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_ADST as usize] =
        Some(inv_txfm_add_adst_adst_4x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X8 as usize][ADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_adst_4x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_ADST as usize] =
        Some(inv_txfm_add_adst_flipadst_4x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X8 as usize][DCT_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_dct_4x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_DCT as usize] =
        Some(inv_txfm_add_dct_flipadst_4x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X8 as usize][FLIPADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_flipadst_4x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X8 as usize][H_DCT as usize] =
        Some(inv_txfm_add_dct_identity_4x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X8 as usize][V_DCT as usize] =
        Some(inv_txfm_add_identity_dct_4x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X8 as usize][H_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_identity_4x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X8 as usize][V_FLIPADST as usize] =
        Some(inv_txfm_add_identity_flipadst_4x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X8 as usize][H_ADST as usize] =
        Some(inv_txfm_add_adst_identity_4x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X8 as usize][V_ADST as usize] =
        Some(inv_txfm_add_identity_adst_4x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_4x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X16 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_4x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_ADST as usize] =
        Some(inv_txfm_add_adst_dct_4x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_DCT as usize] =
        Some(inv_txfm_add_dct_adst_4x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_ADST as usize] =
        Some(inv_txfm_add_adst_adst_4x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X16 as usize][ADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_adst_4x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_ADST as usize] =
        Some(inv_txfm_add_adst_flipadst_4x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X16 as usize][DCT_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_dct_4x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_DCT as usize] =
        Some(inv_txfm_add_dct_flipadst_4x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X16 as usize][FLIPADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_flipadst_4x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X16 as usize][H_DCT as usize] =
        Some(inv_txfm_add_dct_identity_4x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X16 as usize][V_DCT as usize] =
        Some(inv_txfm_add_identity_dct_4x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X16 as usize][H_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_identity_4x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X16 as usize][V_FLIPADST as usize] =
        Some(inv_txfm_add_identity_flipadst_4x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X16 as usize][H_ADST as usize] =
        Some(inv_txfm_add_adst_identity_4x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_4X16 as usize][V_ADST as usize] =
        Some(inv_txfm_add_identity_adst_4x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_8x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X4 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_8x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_ADST as usize] =
        Some(inv_txfm_add_adst_dct_8x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_DCT as usize] =
        Some(inv_txfm_add_dct_adst_8x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_ADST as usize] =
        Some(inv_txfm_add_adst_adst_8x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X4 as usize][ADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_adst_8x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_ADST as usize] =
        Some(inv_txfm_add_adst_flipadst_8x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X4 as usize][DCT_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_dct_8x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_DCT as usize] =
        Some(inv_txfm_add_dct_flipadst_8x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X4 as usize][FLIPADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_flipadst_8x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X4 as usize][H_DCT as usize] =
        Some(inv_txfm_add_dct_identity_8x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X4 as usize][V_DCT as usize] =
        Some(inv_txfm_add_identity_dct_8x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X4 as usize][H_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_identity_8x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X4 as usize][V_FLIPADST as usize] =
        Some(inv_txfm_add_identity_flipadst_8x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X4 as usize][H_ADST as usize] =
        Some(inv_txfm_add_adst_identity_8x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X4 as usize][V_ADST as usize] =
        Some(inv_txfm_add_identity_adst_8x4_c_erased::<BD>);
    (*c).itxfm_add[TX_8X8 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_8x8_c_erased::<BD>);
    (*c).itxfm_add[TX_8X8 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_8x8_c_erased::<BD>);
    (*c).itxfm_add[TX_8X8 as usize][DCT_ADST as usize] =
        Some(inv_txfm_add_adst_dct_8x8_c_erased::<BD>);
    (*c).itxfm_add[TX_8X8 as usize][ADST_DCT as usize] =
        Some(inv_txfm_add_dct_adst_8x8_c_erased::<BD>);
    (*c).itxfm_add[TX_8X8 as usize][ADST_ADST as usize] =
        Some(inv_txfm_add_adst_adst_8x8_c_erased::<BD>);
    (*c).itxfm_add[TX_8X8 as usize][ADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_adst_8x8_c_erased::<BD>);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_ADST as usize] =
        Some(inv_txfm_add_adst_flipadst_8x8_c_erased::<BD>);
    (*c).itxfm_add[TX_8X8 as usize][DCT_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_dct_8x8_c_erased::<BD>);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_DCT as usize] =
        Some(inv_txfm_add_dct_flipadst_8x8_c_erased::<BD>);
    (*c).itxfm_add[TX_8X8 as usize][FLIPADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_flipadst_8x8_c_erased::<BD>);
    (*c).itxfm_add[TX_8X8 as usize][H_DCT as usize] =
        Some(inv_txfm_add_dct_identity_8x8_c_erased::<BD>);
    (*c).itxfm_add[TX_8X8 as usize][V_DCT as usize] =
        Some(inv_txfm_add_identity_dct_8x8_c_erased::<BD>);
    (*c).itxfm_add[TX_8X8 as usize][H_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_identity_8x8_c_erased::<BD>);
    (*c).itxfm_add[TX_8X8 as usize][V_FLIPADST as usize] =
        Some(inv_txfm_add_identity_flipadst_8x8_c_erased::<BD>);
    (*c).itxfm_add[TX_8X8 as usize][H_ADST as usize] =
        Some(inv_txfm_add_adst_identity_8x8_c_erased::<BD>);
    (*c).itxfm_add[TX_8X8 as usize][V_ADST as usize] =
        Some(inv_txfm_add_identity_adst_8x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_8x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X16 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_8x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_ADST as usize] =
        Some(inv_txfm_add_adst_dct_8x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_DCT as usize] =
        Some(inv_txfm_add_dct_adst_8x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_ADST as usize] =
        Some(inv_txfm_add_adst_adst_8x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X16 as usize][ADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_adst_8x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_ADST as usize] =
        Some(inv_txfm_add_adst_flipadst_8x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X16 as usize][DCT_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_dct_8x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_DCT as usize] =
        Some(inv_txfm_add_dct_flipadst_8x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X16 as usize][FLIPADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_flipadst_8x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X16 as usize][H_DCT as usize] =
        Some(inv_txfm_add_dct_identity_8x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X16 as usize][V_DCT as usize] =
        Some(inv_txfm_add_identity_dct_8x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X16 as usize][H_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_identity_8x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X16 as usize][V_FLIPADST as usize] =
        Some(inv_txfm_add_identity_flipadst_8x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X16 as usize][H_ADST as usize] =
        Some(inv_txfm_add_adst_identity_8x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X16 as usize][V_ADST as usize] =
        Some(inv_txfm_add_identity_adst_8x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X32 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_8x32_c_erased::<BD>);
    (*c).itxfm_add[RTX_8X32 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_8x32_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_16x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X4 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_16x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_ADST as usize] =
        Some(inv_txfm_add_adst_dct_16x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_DCT as usize] =
        Some(inv_txfm_add_dct_adst_16x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_ADST as usize] =
        Some(inv_txfm_add_adst_adst_16x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X4 as usize][ADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_adst_16x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_ADST as usize] =
        Some(inv_txfm_add_adst_flipadst_16x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X4 as usize][DCT_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_dct_16x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_DCT as usize] =
        Some(inv_txfm_add_dct_flipadst_16x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X4 as usize][FLIPADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_flipadst_16x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X4 as usize][H_DCT as usize] =
        Some(inv_txfm_add_dct_identity_16x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X4 as usize][V_DCT as usize] =
        Some(inv_txfm_add_identity_dct_16x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X4 as usize][H_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_identity_16x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X4 as usize][V_FLIPADST as usize] =
        Some(inv_txfm_add_identity_flipadst_16x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X4 as usize][H_ADST as usize] =
        Some(inv_txfm_add_adst_identity_16x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X4 as usize][V_ADST as usize] =
        Some(inv_txfm_add_identity_adst_16x4_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_16x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X8 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_16x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_ADST as usize] =
        Some(inv_txfm_add_adst_dct_16x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_DCT as usize] =
        Some(inv_txfm_add_dct_adst_16x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_ADST as usize] =
        Some(inv_txfm_add_adst_adst_16x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X8 as usize][ADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_adst_16x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_ADST as usize] =
        Some(inv_txfm_add_adst_flipadst_16x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X8 as usize][DCT_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_dct_16x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_DCT as usize] =
        Some(inv_txfm_add_dct_flipadst_16x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X8 as usize][FLIPADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_flipadst_16x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X8 as usize][H_DCT as usize] =
        Some(inv_txfm_add_dct_identity_16x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X8 as usize][V_DCT as usize] =
        Some(inv_txfm_add_identity_dct_16x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X8 as usize][H_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_identity_16x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X8 as usize][V_FLIPADST as usize] =
        Some(inv_txfm_add_identity_flipadst_16x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X8 as usize][H_ADST as usize] =
        Some(inv_txfm_add_adst_identity_16x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X8 as usize][V_ADST as usize] =
        Some(inv_txfm_add_identity_adst_16x8_c_erased::<BD>);
    (*c).itxfm_add[TX_16X16 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_16x16_c_erased::<BD>);
    (*c).itxfm_add[TX_16X16 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_16x16_c_erased::<BD>);
    (*c).itxfm_add[TX_16X16 as usize][DCT_ADST as usize] =
        Some(inv_txfm_add_adst_dct_16x16_c_erased::<BD>);
    (*c).itxfm_add[TX_16X16 as usize][ADST_DCT as usize] =
        Some(inv_txfm_add_dct_adst_16x16_c_erased::<BD>);
    (*c).itxfm_add[TX_16X16 as usize][ADST_ADST as usize] =
        Some(inv_txfm_add_adst_adst_16x16_c_erased::<BD>);
    (*c).itxfm_add[TX_16X16 as usize][ADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_adst_16x16_c_erased::<BD>);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_ADST as usize] =
        Some(inv_txfm_add_adst_flipadst_16x16_c_erased::<BD>);
    (*c).itxfm_add[TX_16X16 as usize][DCT_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_dct_16x16_c_erased::<BD>);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_DCT as usize] =
        Some(inv_txfm_add_dct_flipadst_16x16_c_erased::<BD>);
    (*c).itxfm_add[TX_16X16 as usize][FLIPADST_FLIPADST as usize] =
        Some(inv_txfm_add_flipadst_flipadst_16x16_c_erased::<BD>);
    (*c).itxfm_add[TX_16X16 as usize][H_DCT as usize] =
        Some(inv_txfm_add_dct_identity_16x16_c_erased::<BD>);
    (*c).itxfm_add[TX_16X16 as usize][V_DCT as usize] =
        Some(inv_txfm_add_identity_dct_16x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X32 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_16x32_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X32 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_16x32_c_erased::<BD>);
    (*c).itxfm_add[RTX_16X64 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_16x64_c_erased::<BD>);
    (*c).itxfm_add[RTX_32X8 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_32x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_32X8 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_32x8_c_erased::<BD>);
    (*c).itxfm_add[RTX_32X16 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_32x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_32X16 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_32x16_c_erased::<BD>);
    (*c).itxfm_add[TX_32X32 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_32x32_c_erased::<BD>);
    (*c).itxfm_add[TX_32X32 as usize][IDTX as usize] =
        Some(inv_txfm_add_identity_identity_32x32_c_erased::<BD>);
    (*c).itxfm_add[RTX_32X64 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_32x64_c_erased::<BD>);
    (*c).itxfm_add[RTX_64X16 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_64x16_c_erased::<BD>);
    (*c).itxfm_add[RTX_64X32 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_64x32_c_erased::<BD>);
    (*c).itxfm_add[TX_64X64 as usize][DCT_DCT as usize] =
        Some(inv_txfm_add_dct_dct_64x64_c_erased::<BD>);

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            itx_dsp_init_x86::<BD>(c, _bpc);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
            itx_dsp_init_arm::<BD>(c, _bpc);
        }
    }
}
