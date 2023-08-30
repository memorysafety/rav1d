use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynCoef;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::intops::iclip;
use crate::include::common::intops::imin;
use crate::include::stddef::*;
use crate::include::stdint::*;

extern "C" {
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type itx_1d_fn =
    Option<unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> ()>;

pub unsafe extern "C" fn inv_txfm_add_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    coeff: *mut BD::Coef,
    eob: libc::c_int,
    w: libc::c_int,
    h: libc::c_int,
    shift: libc::c_int,
    first_1d_fn: itx_1d_fn,
    second_1d_fn: itx_1d_fn,
    has_dconly: libc::c_int,
    bd: BD,
) {
    let bitdepth_max: libc::c_int = bd.bitdepth_max().as_();
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
    let is_rect2: libc::c_int = (w * 2 == h || h * 2 == w) as libc::c_int;
    let rnd = (1 as libc::c_int) << shift >> 1;
    if eob < has_dconly {
        let mut dc: libc::c_int = (*coeff.offset(0)).as_();
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
                    bd.iclip_pixel((*dst.offset(x as isize)).as_::<libc::c_int>() + dc);
                x += 1;
            }
            y += 1;
            dst = dst.offset(BD::pxstride(stride) as isize);
        }
        return;
    }
    let sh = imin(h, 32 as libc::c_int);
    let sw = imin(w, 32 as libc::c_int);
    let row_clip_min;
    let col_clip_min;
    if BD::BITDEPTH == 8 {
        row_clip_min = std::i16::MIN as i32;
        col_clip_min = std::i16::MIN as i32;
    } else {
        row_clip_min = ((!bitdepth_max) << 7) as libc::c_int;
        col_clip_min = ((!bitdepth_max) << 5) as libc::c_int;
    }
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
                    (*coeff.offset((y_0 + x_0 * sh) as isize)).as_::<libc::c_int>() * 181 + 128
                        >> 8;
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
        (::core::mem::size_of::<BD::Coef>() as libc::c_ulong)
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
            *dst.offset(x_3 as isize) = bd
                .iclip_pixel((*dst.offset(x_3 as isize)).as_::<libc::c_int>() + (*fresh0 + 8 >> 4));
            x_3 += 1;
        }
        y_1 += 1;
        dst = dst.offset(BD::pxstride(stride) as isize);
    }
}

pub type itxfm_fn = Option<
    unsafe extern "C" fn(*mut DynPixel, ptrdiff_t, *mut DynCoef, libc::c_int, libc::c_int) -> (),
>;
#[repr(C)]
pub struct Dav1dInvTxfmDSPContext {
    pub itxfm_add: [[itxfm_fn; 17]; 19],
}

#[cfg(feature = "asm")]
macro_rules! decl_itx_fn {
    ($name:ident) => {
        // TODO(legare): Temporarily pub until init fns are deduplicated.
        #[allow(dead_code)] // TODO(kkysen) Way more asm fns than exist are declared.
        pub(crate) fn $name(
            dst: *mut DynPixel,
            dst_stride: ptrdiff_t,
            coeff: *mut DynCoef,
            eob: libc::c_int,
            bitdepth_max: libc::c_int,
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
    ($w:literal x $h:literal, $asm:ident) => {
        #[cfg(feature = "bitdepth_8")]
        decl_itx1_fns!($w x $h,  8 bpc, $asm);
        #[cfg(feature = "bitdepth_16")]
        decl_itx1_fns!($w x $h, 16 bpc, $asm);
    };
}

#[cfg(feature = "asm")]
macro_rules! decl_itx2_fns {
    ($w:literal x $h:literal, $bpc:literal bpc, $asm:ident) => {
        decl_itx1_fns!($w x $h, $bpc bpc, $asm);
        decl_itx_fn!(identity, identity, $w x $h, $bpc bpc, $asm);
    };
    ($w:literal x $h:literal, $asm:ident) => {
        #[cfg(feature = "bitdepth_8")]
        decl_itx2_fns!($w x $h,  8 bpc, $asm);
        #[cfg(feature = "bitdepth_16")]
        decl_itx2_fns!($w x $h, 16 bpc, $asm);
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
    ($w:literal x $h:literal, $asm:ident) => {
        #[cfg(feature = "bitdepth_8")]
        decl_itx12_fns!($w x $h,  8 bpc, $asm);
        #[cfg(feature = "bitdepth_16")]
        decl_itx12_fns!($w x $h, 16 bpc, $asm);
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
    ($w:literal x $h:literal, $asm:ident) => {
        #[cfg(feature = "bitdepth_8")]
        decl_itx16_fns!($w x $h,  8 bpc, $asm);
        #[cfg(feature = "bitdepth_16")]
        decl_itx16_fns!($w x $h, 16 bpc, $asm);
    };
}

#[cfg(feature = "asm")]
macro_rules! decl_itx17_fns {
    ($w:literal x $h:literal, $bpc:literal bpc, $asm:ident) => {
        decl_itx16_fns!($w x $h, $bpc bpc, $asm);
        decl_itx_fn!(wht, wht, $w x $h, $bpc bpc, $asm);
    };
    ($w:literal x $h:literal, $asm:ident) => {
        #[cfg(feature = "bitdepth_8")]
        decl_itx17_fns!($w x $h,  8 bpc, $asm);
        #[cfg(feature = "bitdepth_16")]
        decl_itx17_fns!($w x $h, 16 bpc, $asm);
    };
}

#[cfg(feature = "asm")]
macro_rules! decl_itx_fns {
    ($bpc:literal bpc, $asm:ident) => {
        decl_itx17_fns!( 4 x  4, $bpc bpc, $asm);
        decl_itx16_fns!( 4 x  8, $bpc bpc, $asm);
        decl_itx16_fns!( 4 x 16, $bpc bpc, $asm);
        decl_itx16_fns!( 8 x  4, $bpc bpc, $asm);
        decl_itx16_fns!( 8 x  8, $bpc bpc, $asm);
        decl_itx16_fns!( 8 x 16, $bpc bpc, $asm);
        decl_itx2_fns! ( 8 x 32, $bpc bpc, $asm);
        decl_itx16_fns!(16 x  4, $bpc bpc, $asm);
        decl_itx16_fns!(16 x  8, $bpc bpc, $asm);
        decl_itx12_fns!(16 x 16, $bpc bpc, $asm);
        decl_itx2_fns! (16 x 32, $bpc bpc, $asm);
        decl_itx2_fns! (32 x  8, $bpc bpc, $asm);
        decl_itx2_fns! (32 x 16, $bpc bpc, $asm);
        decl_itx2_fns! (32 x 32, $bpc bpc, $asm);
        decl_itx_fn!(dct, dct, 16 x 64, $bpc bpc, $asm);
        decl_itx_fn!(dct, dct, 32 x 64, $bpc bpc, $asm);
        decl_itx_fn!(dct, dct, 64 x 16, $bpc bpc, $asm);
        decl_itx_fn!(dct, dct, 64 x 32, $bpc bpc, $asm);
        decl_itx_fn!(dct, dct, 64 x 64, $bpc bpc, $asm);
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
    decl_itx_fns!(sse4);
    decl_itx_fns!(ssse3);
    decl_itx_fn!(wht, wht, 4 x 4, sse2);
}

#[cfg(all(feature = "asm", target_arch = "x86_64"))]
extern "C" {
    decl_itx_fns!(avx512icl);
    decl_itx_fns!(10 bpc, avx512icl);
    decl_itx_fns!(avx2);
    decl_itx_fns!(10 bpc, avx2);
    decl_itx_fns!(12 bpc, avx2);
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
extern "C" {
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
                eob: libc::c_int,
                bitdepth_max: libc::c_int,
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
                    $has_dconly as libc::c_int,
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
