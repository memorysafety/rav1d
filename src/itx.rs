use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynCoef;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::intops::iclip;
use crate::src::cpu::CpuFlags;
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
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::slice;

#[cfg(all(
    feature = "asm",
    not(any(target_arch = "riscv64", target_arch = "riscv32"))
))]
use crate::include::common::bitdepth::bd_fn;

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
use crate::include::common::bitdepth::bpc_fn;

pub type itx_1d_fn = unsafe extern "C" fn(c: *mut i32, stride: ptrdiff_t, min: c_int, max: c_int);

pub unsafe fn inv_txfm_add_rust<
    const W: usize,
    const H: usize,
    const SHIFT: u8,
    const HAS_DC_ONLY: bool,
    BD: BitDepth,
>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    coeff: *mut BD::Coef,
    eob: c_int,
    first_1d_fn: itx_1d_fn,
    second_1d_fn: itx_1d_fn,
    bd: BD,
) {
    let bitdepth_max = bd.bitdepth_max().as_::<c_int>();

    assert!(W >= 4 && W <= 64);
    assert!(H >= 4 && H <= 64);
    assert!(eob >= 0);

    let is_rect2 = W * 2 == H || H * 2 == W;
    let rnd = 1 << SHIFT >> 1;

    if eob < HAS_DC_ONLY as c_int {
        let coeff = slice::from_raw_parts_mut(coeff, 1);

        let mut dc = coeff[0].as_::<c_int>();
        coeff[0] = 0.as_();
        if is_rect2 {
            dc = dc * 181 + 128 >> 8;
        }
        dc = dc * 181 + 128 >> 8;
        dc = dc + rnd >> SHIFT;
        dc = dc * 181 + 128 + 2048 >> 12;
        for _ in 0..H {
            for x in 0..W {
                *dst.add(x) = bd.iclip_pixel((*dst.add(x)).as_::<c_int>() + dc);
            }
            dst = dst.offset(BD::pxstride(stride));
        }
        return;
    }

    let sh = cmp::min(H, 32);
    let sw = cmp::min(W, 32);

    let coeff = slice::from_raw_parts_mut(coeff, sh * sw);

    let row_clip_min;
    let col_clip_min;
    if BD::BITDEPTH == 8 {
        row_clip_min = i16::MIN as i32;
        col_clip_min = i16::MIN as i32;
    } else {
        row_clip_min = (!bitdepth_max) << 7;
        col_clip_min = (!bitdepth_max) << 5;
    }
    let row_clip_max = !row_clip_min;
    let col_clip_max = !col_clip_min;

    let mut tmp = [0; 64 * 64]; // Should be `W * H`.
    let mut c = &mut tmp[..];
    for y in 0..sh {
        if is_rect2 {
            for x in 0..sw {
                c[x] = coeff[y + x * sh].as_::<c_int>() * 181 + 128 >> 8;
            }
        } else {
            for x in 0..sw {
                c[x] = coeff[y + x * sh].as_();
            }
        }
        first_1d_fn(c.as_mut_ptr(), 1, row_clip_min, row_clip_max);
        c = &mut c[W..];
    }

    coeff.fill(0.into());
    for i in 0..W * sh {
        tmp[i] = iclip(tmp[i] + rnd >> SHIFT, col_clip_min, col_clip_max);
    }

    for x in 0..W {
        second_1d_fn(
            tmp[x..].as_mut_ptr(),
            W as ptrdiff_t,
            col_clip_min,
            col_clip_max,
        );
    }

    let mut c = &tmp[..];
    for _ in 0..H {
        for x in 0..W {
            *dst.add(x) = bd.iclip_pixel((*dst.add(x)).as_::<c_int>() + (c[0] + 8 >> 4));
            c = &c[1..];
        }
        dst = dst.offset(BD::pxstride(stride));
    }
}

pub type itxfm_fn =
    Option<unsafe extern "C" fn(*mut DynPixel, ptrdiff_t, *mut DynCoef, c_int, c_int) -> ()>;

pub struct Rav1dInvTxfmDSPContext {
    pub itxfm_add: [[itxfm_fn; N_TX_TYPES_PLUS_LL]; N_RECT_TX_SIZES],
}

#[cfg(all(
    feature = "asm",
    not(any(target_arch = "riscv64", target_arch = "riscv32"))
))]
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

#[cfg(all(
    feature = "asm",
    not(any(target_arch = "riscv64", target_arch = "riscv32"))
))]
macro_rules! decl_itx1_fns {
    ($w:literal x $h:literal, $bpc:literal bpc, $asm:ident) => {
        decl_itx_fn!(dct, dct, $w x $h, $bpc bpc, $asm);
    };
}

#[cfg(all(
    feature = "asm",
    not(any(target_arch = "riscv64", target_arch = "riscv32"))
))]
macro_rules! decl_itx2_fns {
    ($w:literal x $h:literal, $bpc:literal bpc, $asm:ident) => {
        decl_itx1_fns!($w x $h, $bpc bpc, $asm);
        decl_itx_fn!(identity, identity, $w x $h, $bpc bpc, $asm);
    };
}

#[cfg(all(
    feature = "asm",
    not(any(target_arch = "riscv64", target_arch = "riscv32"))
))]
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

#[cfg(all(
    feature = "asm",
    not(any(target_arch = "riscv64", target_arch = "riscv32"))
))]
macro_rules! decl_itx16_fns {
    ($w:literal x $h:literal, $bpc:literal bpc, $asm:ident) => {
        decl_itx12_fns!($w x $h, $bpc bpc, $asm);
        decl_itx_fn!(adst, identity, $w x $h, $bpc bpc, $asm);
        decl_itx_fn!(flipadst, identity, $w x $h, $bpc bpc, $asm);
        decl_itx_fn!(identity, adst, $w x $h, $bpc bpc, $asm);
        decl_itx_fn!(identity, flipadst, $w x $h, $bpc bpc, $asm);
    };
}

#[cfg(all(
    feature = "asm",
    not(any(target_arch = "riscv64", target_arch = "riscv32"))
))]
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
                inv_txfm_add_rust::<$w, $h, $shift, $has_dconly, BD>(
                    dst.cast(),
                    stride,
                    coeff.cast(),
                    eob,
                    [<dav1d_inv_ $type1 $w _1d_c>],
                    [<dav1d_inv_ $type2 $h _1d_c>],
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

    const H: usize = 4;
    const W: usize = 4;

    let coeff = slice::from_raw_parts_mut(coeff, W * H);

    let mut tmp = [0; W * H];
    let mut c = &mut tmp[..];
    let mut y = 0;
    while y < H {
        let mut x = 0;
        while x < W {
            c[x as usize] = coeff[(y + x * H) as usize].as_::<i32>() >> 2;
            x += 1;
        }
        dav1d_inv_wht4_1d_c(c.as_mut_ptr(), 1);
        y += 1;
        c = &mut c[W..];
    }
    coeff.fill(0.into());
    let mut x = 0;
    while x < W {
        dav1d_inv_wht4_1d_c(tmp[x as usize..].as_mut_ptr(), H as isize);
        x += 1;
    }
    c = &mut tmp[..];
    let mut y = 0;
    while y < H {
        let mut x = 0;
        while x < W {
            *dst.offset(x as isize) =
                bd.iclip_pixel((*dst.offset(x as isize)).as_::<c_int>() + c[0]);
            c = &mut c[1..];
            x += 1;
        }
        y += 1;
        dst = dst.offset(BD::pxstride(stride as usize) as isize);
    }
}

#[cfg(all(
    feature = "asm",
    not(any(target_arch = "riscv64", target_arch = "riscv32"))
))]
macro_rules! assign_itx_fn {
    ($c:ident, $BD:ty, $w:literal, $h:literal, $type:ident, $type_enum:ident, $ext:ident) => {{
        use paste::paste;

        paste! {
            $c.itxfm_add[[<TX_ $w X $h>] as usize][$type_enum as usize]
                = Some(bd_fn!(BD, [< inv_txfm_add_ $type _ $w x $h >], $ext));
        }
    }};

    ($c:ident, $BD:ty, $pfx:ident, $w:literal, $h:literal, $type:ident, $type_enum:ident, $ext:ident) => {{
        use paste::paste;

        paste! {
            $c.itxfm_add[[<$pfx TX_ $w X $h>] as usize][$type_enum as usize]
                = Some(bd_fn!(BD, [< inv_txfm_add_ $type _ $w x $h >], $ext));
        }
    }};
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
macro_rules! assign_itx_bpc_fn {
    ($c:ident, $pfx:ident, $w:literal, $h:literal, $type:ident, $type_enum:ident, $bpc:literal bpc, $ext:ident) => {{
        use paste::paste;

        paste! {
            $c.itxfm_add[[<$pfx TX_ $w X $h>] as usize][$type_enum as usize]
                = Some(bpc_fn!($bpc bpc, [< inv_txfm_add_ $type _ $w x $h >], $ext));
        }
    }};

    ($c:ident, $w:literal, $h:literal, $type:ident, $type_enum:ident, $bpc:literal bpc, $ext:ident) => {{
        use paste::paste;

        paste! {
            $c.itxfm_add[[<TX_ $w X $h>] as usize][$type_enum as usize]
                = Some(bpc_fn!($bpc bpc, [< inv_txfm_add_ $type _ $w x $h >], $ext));
        }
    }};
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
macro_rules! assign_itx1_bpc_fn {
    ($c:ident, $w:literal, $h:literal, $bpc:literal bpc, $ext:ident) => {{
        assign_itx_bpc_fn!($c, $w, $h, dct_dct, DCT_DCT, $bpc bpc, $ext)
    }};

    ($c:ident, $pfx:ident, $w:literal, $h:literal, $bpc:literal bpc, $ext:ident) => {{
        assign_itx_bpc_fn!($c, $pfx, $w, $h, dct_dct, DCT_DCT, $bpc bpc, $ext)
    }};
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
macro_rules! assign_itx1_fn {
    ($c:ident, $BD:ty, $w:literal, $h:literal, $ext:ident) => {{
        assign_itx_fn!($c, BD, $w, $h, dct_dct, DCT_DCT, $ext)
    }};

    ($c:ident, $BD:ty, $pfx:ident, $w:literal, $h:literal, $ext:ident) => {{
        assign_itx_fn!($c, BD, $pfx, $w, $h, dct_dct, DCT_DCT, $ext)
    }};
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
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

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
macro_rules! assign_itx2_fn {
    ($c:ident, $BD:ty, $w:literal, $h:literal, $ext:ident) => {{
        assign_itx1_fn!($c, BD, $w, $h, $ext);
        assign_itx_fn!($c, BD, $w, $h, identity_identity, IDTX, $ext)
    }};

    ($c:ident, $BD:ty, $pfx:ident, $w:literal, $h:literal, $ext:ident) => {{
        assign_itx1_fn!($c, BD, $pfx, $w, $h, $ext);
        assign_itx_fn!($c, BD, $pfx, $w, $h, identity_identity, IDTX, $ext)
    }};
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
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

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
macro_rules! assign_itx12_fn {
    ($c:ident, $BD:ty, $w:literal, $h:literal, $ext:ident) => {{
        assign_itx2_fn!($c, BD, $w, $h, $ext);
        assign_itx_fn!($c, BD, $w, $h, dct_flipadst, FLIPADST_DCT, $ext);
        assign_itx_fn!($c, BD, $w, $h, dct_adst, ADST_DCT, $ext);
        assign_itx_fn!($c, BD, $w, $h, dct_identity, H_DCT, $ext);
        assign_itx_fn!($c, BD, $w, $h, adst_dct, DCT_ADST, $ext);
        assign_itx_fn!($c, BD, $w, $h, adst_adst, ADST_ADST, $ext);
        assign_itx_fn!($c, BD, $w, $h, adst_flipadst, FLIPADST_ADST, $ext);
        assign_itx_fn!($c, BD, $w, $h, flipadst_dct, DCT_FLIPADST, $ext);
        assign_itx_fn!($c, BD, $w, $h, flipadst_adst, ADST_FLIPADST, $ext);
        assign_itx_fn!($c, BD, $w, $h, flipadst_flipadst, FLIPADST_FLIPADST, $ext);
        assign_itx_fn!($c, BD, $w, $h, identity_dct, V_DCT, $ext);
    }};

    ($c:ident, $BD:ty, $pfx:ident, $w:literal, $h:literal, $ext:ident) => {{
        assign_itx2_fn!($c, BD, $pfx, $w, $h, $ext);
        assign_itx_fn!($c, BD, $pfx, $w, $h, dct_flipadst, FLIPADST_DCT, $ext);
        assign_itx_fn!($c, BD, $pfx, $w, $h, dct_adst, ADST_DCT, $ext);
        assign_itx_fn!($c, BD, $pfx, $w, $h, dct_identity, H_DCT, $ext);
        assign_itx_fn!($c, BD, $pfx, $w, $h, adst_dct, DCT_ADST, $ext);
        assign_itx_fn!($c, BD, $pfx, $w, $h, adst_adst, ADST_ADST, $ext);
        assign_itx_fn!($c, BD, $pfx, $w, $h, adst_flipadst, FLIPADST_ADST, $ext);
        assign_itx_fn!($c, BD, $pfx, $w, $h, flipadst_dct, DCT_FLIPADST, $ext);
        assign_itx_fn!($c, BD, $pfx, $w, $h, flipadst_adst, ADST_FLIPADST, $ext);
        assign_itx_fn!(
            $c,
            BD,
            $pfx,
            $w,
            $h,
            flipadst_flipadst,
            FLIPADST_FLIPADST,
            $ext
        );
        assign_itx_fn!($c, BD, $pfx, $w, $h, identity_dct, V_DCT, $ext);
    }};
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
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

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
macro_rules! assign_itx16_fn {
    ($c:ident, $BD:ty, $w:literal, $h:literal, $ext:ident) => {{
        assign_itx12_fn!($c, BD, $w, $h, $ext);
        assign_itx_fn!($c, BD, $w, $h, adst_identity, H_ADST, $ext);
        assign_itx_fn!($c, BD, $w, $h, flipadst_identity, H_FLIPADST, $ext);
        assign_itx_fn!($c, BD, $w, $h, identity_adst, V_ADST, $ext);
        assign_itx_fn!($c, BD, $w, $h, identity_flipadst, V_FLIPADST, $ext);
    }};

    ($c:ident, $BD:ty, $pfx:ident, $w:literal, $h:literal, $ext:ident) => {{
        assign_itx12_fn!($c, BD, $pfx, $w, $h, $ext);
        assign_itx_fn!($c, BD, $pfx, $w, $h, adst_identity, H_ADST, $ext);
        assign_itx_fn!($c, BD, $pfx, $w, $h, flipadst_identity, H_FLIPADST, $ext);
        assign_itx_fn!($c, BD, $pfx, $w, $h, identity_adst, V_ADST, $ext);
        assign_itx_fn!($c, BD, $pfx, $w, $h, identity_flipadst, V_FLIPADST, $ext);
    }};
}

macro_rules! assign_itx_all_fn64 {
    ($c:ident, $BD:ty, $w:literal, $h:literal) => {{
        use paste::paste;

        paste! {
            $c.itxfm_add[[<TX_ $w X $h>] as usize][DCT_DCT as usize] =
                Some([< inv_txfm_add_dct_dct_ $w x $h _c_erased >]::<BD>);
        }
    }};

    ($c:ident, $BD:ty, $w:literal, $h:literal, $pfx:ident) => {{
        use paste::paste;

        paste! {
            $c.itxfm_add[[<$pfx TX_ $w X $h>] as usize][DCT_DCT as usize]
                = Some([< inv_txfm_add_dct_dct_ $w x $h _c_erased >]::<BD>);
        }
    }};
}

macro_rules! assign_itx_all_fn32 {
    ($c:ident, $BD:ty, $w:literal, $h:literal) => {{
        use paste::paste;

        assign_itx_all_fn64!($c, BD, $w, $h);
        paste! {
            $c.itxfm_add[[<TX_ $w X $h>] as usize][IDTX as usize]
                = Some([< inv_txfm_add_identity_identity_ $w x $h _c_erased >]::<BD>);
        }
    }};

    ($c:ident, $BD:ty, $w:literal, $h:literal, $pfx:ident) => {{
        use paste::paste;

        assign_itx_all_fn64!($c, BD, $w, $h, $pfx);
        paste! {
            $c.itxfm_add[[<$pfx TX_ $w X $h>] as usize][IDTX as usize]
                = Some([< inv_txfm_add_identity_identity_ $w x $h _c_erased >]::<BD>);
        }
    }};
}

macro_rules! assign_itx_all_fn16 {
    ($c:ident, $BD:ty, $w:literal, $h:literal) => {{
        use paste::paste;

        assign_itx_all_fn32!($c, BD, $w, $h);
        paste! {
            $c.itxfm_add[[<TX_ $w X $h>] as usize][DCT_ADST as usize]
                = Some([< inv_txfm_add_adst_dct_ $w x $h _c_erased >]::<BD>);
            $c.itxfm_add[[<TX_ $w X $h>] as usize][ADST_DCT as usize]
                = Some([< inv_txfm_add_dct_adst_ $w x $h _c_erased >]::<BD>);
            $c.itxfm_add[[<TX_ $w X $h>] as usize][ADST_ADST as usize]
                = Some([< inv_txfm_add_adst_adst_ $w x $h _c_erased >]::<BD>);
            $c.itxfm_add[[<TX_ $w X $h>] as usize][ADST_FLIPADST as usize]
                = Some([< inv_txfm_add_flipadst_adst_ $w x $h _c_erased >]::<BD>);
            $c.itxfm_add[[<TX_ $w X $h>] as usize][FLIPADST_ADST as usize]
                = Some([< inv_txfm_add_adst_flipadst_ $w x $h _c_erased >]::<BD>);
            $c.itxfm_add[[<TX_ $w X $h>] as usize][DCT_FLIPADST as usize]
                = Some([< inv_txfm_add_flipadst_dct_ $w x $h _c_erased >]::<BD>);
            $c.itxfm_add[[<TX_ $w X $h>] as usize][FLIPADST_DCT as usize]
                = Some([< inv_txfm_add_dct_flipadst_ $w x $h _c_erased >]::<BD>);
            $c.itxfm_add[[<TX_ $w X $h>] as usize][FLIPADST_FLIPADST as usize]
                = Some([< inv_txfm_add_flipadst_flipadst_ $w x $h _c_erased >]::<BD>);
            $c.itxfm_add[[<TX_ $w X $h>] as usize][H_DCT as usize]
                = Some([< inv_txfm_add_dct_identity_ $w x $h _c_erased >]::<BD>);
            $c.itxfm_add[[<TX_ $w X $h>] as usize][V_DCT as usize]
                = Some([< inv_txfm_add_identity_dct_ $w x $h _c_erased >]::<BD>);
        }
    }};

    ($c:ident, $BD:ty, $w:literal, $h:literal, $pfx:ident) => {{
        use paste::paste;

        assign_itx_all_fn32!($c, BD, $w, $h, $pfx);
        paste! {
            $c.itxfm_add[[<$pfx TX_ $w X $h>] as usize][DCT_ADST as usize]
                = Some([< inv_txfm_add_adst_dct_ $w x $h _c_erased >]::<BD>);
            $c.itxfm_add[[<$pfx TX_ $w X $h>] as usize][ADST_DCT as usize]
                = Some([< inv_txfm_add_dct_adst_ $w x $h _c_erased >]::<BD>);
            $c.itxfm_add[[<$pfx TX_ $w X $h>] as usize][ADST_ADST as usize]
                = Some([< inv_txfm_add_adst_adst_ $w x $h _c_erased >]::<BD>);
            $c.itxfm_add[[<$pfx TX_ $w X $h>] as usize][ADST_FLIPADST as usize]
                = Some([< inv_txfm_add_flipadst_adst_ $w x $h _c_erased >]::<BD>);
            $c.itxfm_add[[<$pfx TX_ $w X $h>] as usize][FLIPADST_ADST as usize]
                = Some([< inv_txfm_add_adst_flipadst_ $w x $h _c_erased >]::<BD>);
            $c.itxfm_add[[<$pfx TX_ $w X $h>] as usize][DCT_FLIPADST as usize]
                = Some([< inv_txfm_add_flipadst_dct_ $w x $h _c_erased >]::<BD>);
            $c.itxfm_add[[<$pfx TX_ $w X $h>] as usize][FLIPADST_DCT as usize]
                = Some([< inv_txfm_add_dct_flipadst_ $w x $h _c_erased >]::<BD>);
            $c.itxfm_add[[<$pfx TX_ $w X $h>] as usize][FLIPADST_FLIPADST as usize]
                = Some([< inv_txfm_add_flipadst_flipadst_ $w x $h _c_erased >]::<BD>);
            $c.itxfm_add[[<$pfx TX_ $w X $h>] as usize][H_DCT as usize]
                = Some([< inv_txfm_add_dct_identity_ $w x $h _c_erased >]::<BD>);
            $c.itxfm_add[[<$pfx TX_ $w X $h>] as usize][V_DCT as usize]
                = Some([< inv_txfm_add_identity_dct_ $w x $h _c_erased >]::<BD>);
        }
    }};
}

macro_rules! assign_itx_all_fn84 {
    ($c:ident, $BD:ty, $w:literal, $h:literal) => {{
        use paste::paste;

        assign_itx_all_fn16!($c, BD, $w, $h);
        paste! {
            $c.itxfm_add[[<TX_ $w X $h>] as usize][H_FLIPADST as usize]
                = Some([< inv_txfm_add_flipadst_identity_ $w x $h _c_erased >]::<BD>);
            $c.itxfm_add[[<TX_ $w X $h>] as usize][V_FLIPADST as usize]
                = Some([< inv_txfm_add_identity_flipadst_ $w x $h _c_erased >]::<BD>);
            $c.itxfm_add[[<TX_ $w X $h>] as usize][H_ADST as usize]
                = Some([< inv_txfm_add_adst_identity_ $w x $h _c_erased >]::<BD>);
            $c.itxfm_add[[<TX_ $w X $h>] as usize][V_ADST as usize]
                = Some([< inv_txfm_add_identity_adst_ $w x $h _c_erased >]::<BD>);
        }
    }};

    ($c:ident, $BD:ty, $w:literal, $h:literal, $pfx:ident) => {{
        use paste::paste;

        assign_itx_all_fn16!($c, BD, $w, $h, $pfx);
        paste! {
            $c.itxfm_add[[<$pfx TX_ $w X $h>] as usize][H_FLIPADST as usize]
                = Some([< inv_txfm_add_flipadst_identity_ $w x $h _c_erased >]::<BD>);
            $c.itxfm_add[[<$pfx TX_ $w X $h>] as usize][V_FLIPADST as usize]
                = Some([< inv_txfm_add_identity_flipadst_ $w x $h _c_erased >]::<BD>);
            $c.itxfm_add[[<$pfx TX_ $w X $h>] as usize][H_ADST as usize]
                = Some([< inv_txfm_add_adst_identity_ $w x $h _c_erased >]::<BD>);
            $c.itxfm_add[[<$pfx TX_ $w X $h>] as usize][V_ADST as usize]
                = Some([< inv_txfm_add_identity_adst_ $w x $h _c_erased >]::<BD>);
        }
    }};
}

impl Rav1dInvTxfmDSPContext {
    pub const fn default<BD: BitDepth>() -> Self {
        let mut c = Self {
            itxfm_add: [[None; N_TX_TYPES_PLUS_LL]; N_RECT_TX_SIZES],
        };

        c.itxfm_add[TX_4X4 as usize][WHT_WHT as usize] =
            Some(inv_txfm_add_wht_wht_4x4_c_erased::<BD>);

        #[rustfmt::skip]
        const fn assign<BD: BitDepth>(mut c: Rav1dInvTxfmDSPContext) -> Rav1dInvTxfmDSPContext {
            assign_itx_all_fn84!(c, BD,  4,  4   );
            assign_itx_all_fn84!(c, BD,  4,  8, R);
            assign_itx_all_fn84!(c, BD,  4, 16, R);
            assign_itx_all_fn84!(c, BD,  8,  4, R);
            assign_itx_all_fn84!(c, BD,  8,  8   );
            assign_itx_all_fn84!(c, BD,  8, 16, R);
            assign_itx_all_fn32!(c, BD,  8, 32, R);
            assign_itx_all_fn84!(c, BD, 16,  4, R);
            assign_itx_all_fn84!(c, BD, 16,  8, R);
            assign_itx_all_fn16!(c, BD, 16, 16   );
            assign_itx_all_fn32!(c, BD, 16, 32, R);
            assign_itx_all_fn64!(c, BD, 16, 64, R);
            assign_itx_all_fn32!(c, BD, 32,  8, R);
            assign_itx_all_fn32!(c, BD, 32, 16, R);
            assign_itx_all_fn32!(c, BD, 32, 32   );
            assign_itx_all_fn64!(c, BD, 32, 64, R);
            assign_itx_all_fn64!(c, BD, 64, 16, R);
            assign_itx_all_fn64!(c, BD, 64, 32, R);
            assign_itx_all_fn64!(c, BD, 64, 64   );

            c
        }

        assign::<BD>(c)
    }

    #[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
    #[inline(always)]
    const fn init_x86<BD: BitDepth>(mut self, flags: CpuFlags, bpc: u8) -> Self {
        if !flags.contains(CpuFlags::SSE2) {
            return self;
        }

        assign_itx_fn!(self, BD, 4, 4, wht_wht, WHT_WHT, sse2);

        if !flags.contains(CpuFlags::SSSE3) {
            return self;
        }

        if BD::BITDEPTH == 8 {
            assign_itx16_bpc_fn!(self,     4,  4, 8 bpc, ssse3);
            assign_itx16_bpc_fn!(self, R,  4,  8, 8 bpc, ssse3);
            assign_itx16_bpc_fn!(self, R,  8,  4, 8 bpc, ssse3);
            assign_itx16_bpc_fn!(self,     8,  8, 8 bpc, ssse3);
            assign_itx16_bpc_fn!(self, R,  4, 16, 8 bpc, ssse3);
            assign_itx16_bpc_fn!(self, R, 16,  4, 8 bpc, ssse3);
            assign_itx16_bpc_fn!(self, R,  8, 16, 8 bpc, ssse3);
            assign_itx16_bpc_fn!(self, R, 16,  8, 8 bpc, ssse3);
            assign_itx12_bpc_fn!(self,    16, 16, 8 bpc, ssse3);
            assign_itx2_bpc_fn! (self, R,  8, 32, 8 bpc, ssse3);
            assign_itx2_bpc_fn! (self, R, 32,  8, 8 bpc, ssse3);
            assign_itx2_bpc_fn! (self, R, 16, 32, 8 bpc, ssse3);
            assign_itx2_bpc_fn! (self, R, 32, 16, 8 bpc, ssse3);
            assign_itx2_bpc_fn! (self,    32, 32, 8 bpc, ssse3);
            assign_itx1_bpc_fn! (self, R, 16, 64, 8 bpc, ssse3);
            assign_itx1_bpc_fn! (self, R, 32, 64, 8 bpc, ssse3);
            assign_itx1_bpc_fn! (self, R, 64, 16, 8 bpc, ssse3);
            assign_itx1_bpc_fn! (self, R, 64, 32, 8 bpc, ssse3);
            assign_itx1_bpc_fn! (self,    64, 64, 8 bpc, ssse3);
        }

        if !flags.contains(CpuFlags::SSE41) {
            return self;
        }

        if BD::BITDEPTH == 16 {
            if bpc == 10 {
                assign_itx16_bpc_fn!(self,     4,  4, 16 bpc, sse4);
                assign_itx16_bpc_fn!(self, R,  4,  8, 16 bpc, sse4);
                assign_itx16_bpc_fn!(self, R,  4, 16, 16 bpc, sse4);
                assign_itx16_bpc_fn!(self, R,  8,  4, 16 bpc, sse4);
                assign_itx16_bpc_fn!(self,     8,  8, 16 bpc, sse4);
                assign_itx16_bpc_fn!(self, R,  8, 16, 16 bpc, sse4);
                assign_itx16_bpc_fn!(self, R, 16,  4, 16 bpc, sse4);
                assign_itx16_bpc_fn!(self, R, 16,  8, 16 bpc, sse4);
                assign_itx12_bpc_fn!(self,    16, 16, 16 bpc, sse4);
                assign_itx2_bpc_fn! (self, R,  8, 32, 16 bpc, sse4);
                assign_itx2_bpc_fn! (self, R, 16, 32, 16 bpc, sse4);
                assign_itx2_bpc_fn! (self, R, 32,  8, 16 bpc, sse4);
                assign_itx2_bpc_fn! (self, R, 32, 16, 16 bpc, sse4);
                assign_itx2_bpc_fn! (self,    32, 32, 16 bpc, sse4);
                assign_itx1_bpc_fn! (self, R, 16, 64, 16 bpc, sse4);
                assign_itx1_bpc_fn! (self, R, 32, 64, 16 bpc, sse4);
                assign_itx1_bpc_fn! (self, R, 64, 16, 16 bpc, sse4);
                assign_itx1_bpc_fn! (self, R, 64, 32, 16 bpc, sse4);
                assign_itx1_bpc_fn! (self,    64, 64, 16 bpc, sse4);
            }
        }

        #[cfg(target_arch = "x86_64")]
        {
            if !flags.contains(CpuFlags::AVX2) {
                return self;
            }

            assign_itx_fn!(self, BD, 4, 4, wht_wht, WHT_WHT, avx2);

            if BD::BITDEPTH == 8 {
                assign_itx16_bpc_fn!(self,     4,  4, 8 bpc, avx2);
                assign_itx16_bpc_fn!(self, R,  4,  8, 8 bpc, avx2);
                assign_itx16_bpc_fn!(self, R,  4, 16, 8 bpc, avx2);
                assign_itx16_bpc_fn!(self, R,  8,  4, 8 bpc, avx2);
                assign_itx16_bpc_fn!(self,     8,  8, 8 bpc, avx2);
                assign_itx16_bpc_fn!(self, R,  8, 16, 8 bpc, avx2);
                assign_itx16_bpc_fn!(self, R, 16,  4, 8 bpc, avx2);
                assign_itx16_bpc_fn!(self, R, 16,  8, 8 bpc, avx2);
                assign_itx12_bpc_fn!(self,    16, 16, 8 bpc, avx2);
                assign_itx2_bpc_fn! (self, R,  8, 32, 8 bpc, avx2);
                assign_itx2_bpc_fn! (self, R, 16, 32, 8 bpc, avx2);
                assign_itx2_bpc_fn! (self, R, 32,  8, 8 bpc, avx2);
                assign_itx2_bpc_fn! (self, R, 32, 16, 8 bpc, avx2);
                assign_itx2_bpc_fn! (self,    32, 32, 8 bpc, avx2);
                assign_itx1_bpc_fn! (self, R, 16, 64, 8 bpc, avx2);
                assign_itx1_bpc_fn! (self, R, 32, 64, 8 bpc, avx2);
                assign_itx1_bpc_fn! (self, R, 64, 16, 8 bpc, avx2);
                assign_itx1_bpc_fn! (self, R, 64, 32, 8 bpc, avx2);
                assign_itx1_bpc_fn! (self,    64, 64, 8 bpc, avx2);
            } else {
                if bpc == 10 {
                    assign_itx16_bpc_fn!(self,     4,  4, 10 bpc, avx2);
                    assign_itx16_bpc_fn!(self, R,  4,  8, 10 bpc, avx2);
                    assign_itx16_bpc_fn!(self, R,  4, 16, 10 bpc, avx2);
                    assign_itx16_bpc_fn!(self, R,  8,  4, 10 bpc, avx2);
                    assign_itx16_bpc_fn!(self,     8,  8, 10 bpc, avx2);
                    assign_itx16_bpc_fn!(self, R,  8, 16, 10 bpc, avx2);
                    assign_itx16_bpc_fn!(self, R, 16,  4, 10 bpc, avx2);
                    assign_itx16_bpc_fn!(self, R, 16,  8, 10 bpc, avx2);
                    assign_itx12_bpc_fn!(self,    16, 16, 10 bpc, avx2);
                    assign_itx2_bpc_fn! (self, R,  8, 32, 10 bpc, avx2);
                    assign_itx2_bpc_fn! (self, R, 16, 32, 10 bpc, avx2);
                    assign_itx2_bpc_fn! (self, R, 32,  8, 10 bpc, avx2);
                    assign_itx2_bpc_fn! (self, R, 32, 16, 10 bpc, avx2);
                    assign_itx2_bpc_fn! (self,    32, 32, 10 bpc, avx2);
                    assign_itx1_bpc_fn! (self, R, 16, 64, 10 bpc, avx2);
                    assign_itx1_bpc_fn! (self, R, 32, 64, 10 bpc, avx2);
                    assign_itx1_bpc_fn! (self, R, 64, 16, 10 bpc, avx2);
                    assign_itx1_bpc_fn! (self, R, 64, 32, 10 bpc, avx2);
                    assign_itx1_bpc_fn! (self,    64, 64, 10 bpc, avx2);
                } else {
                    assign_itx16_bpc_fn!(self,     4,  4, 12 bpc, avx2);
                    assign_itx16_bpc_fn!(self, R,  4,  8, 12 bpc, avx2);
                    assign_itx16_bpc_fn!(self, R,  4, 16, 12 bpc, avx2);
                    assign_itx16_bpc_fn!(self, R,  8,  4, 12 bpc, avx2);
                    assign_itx16_bpc_fn!(self,     8,  8, 12 bpc, avx2);
                    assign_itx16_bpc_fn!(self, R,  8, 16, 12 bpc, avx2);
                    assign_itx16_bpc_fn!(self, R, 16,  4, 12 bpc, avx2);
                    assign_itx16_bpc_fn!(self, R, 16,  8, 12 bpc, avx2);
                    assign_itx12_bpc_fn!(self,    16, 16, 12 bpc, avx2);
                    assign_itx2_bpc_fn! (self, R,  8, 32, 12 bpc, avx2);
                    assign_itx2_bpc_fn! (self, R, 32,  8, 12 bpc, avx2);
                    assign_itx_bpc_fn!  (self, R, 16, 32, identity_identity, IDTX, 12 bpc, avx2);
                    assign_itx_bpc_fn!  (self, R, 32, 16, identity_identity, IDTX, 12 bpc, avx2);
                    assign_itx_bpc_fn!  (self,    32, 32, identity_identity, IDTX, 12 bpc, avx2);
                }
            }

            if !flags.contains(CpuFlags::AVX512ICL) {
                return self;
            }

            if BD::BITDEPTH == 8 {
                assign_itx16_bpc_fn!(self,     4,  4, 8 bpc, avx512icl); // no wht
                assign_itx16_bpc_fn!(self, R,  4,  8, 8 bpc, avx512icl);
                assign_itx16_bpc_fn!(self, R,  4, 16, 8 bpc, avx512icl);
                assign_itx16_bpc_fn!(self, R,  8,  4, 8 bpc, avx512icl);
                assign_itx16_bpc_fn!(self,     8,  8, 8 bpc, avx512icl);
                assign_itx16_bpc_fn!(self, R,  8, 16, 8 bpc, avx512icl);
                assign_itx16_bpc_fn!(self, R, 16,  4, 8 bpc, avx512icl);
                assign_itx16_bpc_fn!(self, R, 16,  8, 8 bpc, avx512icl);
                assign_itx12_bpc_fn!(self,    16, 16, 8 bpc, avx512icl);
                assign_itx2_bpc_fn! (self, R,  8, 32, 8 bpc, avx512icl);
                assign_itx2_bpc_fn! (self, R, 16, 32, 8 bpc, avx512icl);
                assign_itx2_bpc_fn! (self, R, 32,  8, 8 bpc, avx512icl);
                assign_itx2_bpc_fn! (self, R, 32, 16, 8 bpc, avx512icl);
                assign_itx2_bpc_fn! (self,    32, 32, 8 bpc, avx512icl);
                assign_itx1_bpc_fn! (self, R, 16, 64, 8 bpc, avx512icl);
                assign_itx1_bpc_fn! (self, R, 32, 64, 8 bpc, avx512icl);
                assign_itx1_bpc_fn! (self, R, 64, 16, 8 bpc, avx512icl);
                assign_itx1_bpc_fn! (self, R, 64, 32, 8 bpc, avx512icl);
                assign_itx1_bpc_fn! (self,    64, 64, 8 bpc, avx512icl);
            } else {
                if bpc == 10 {
                    assign_itx16_bpc_fn!(self,     8,  8, 10 bpc, avx512icl);
                    assign_itx16_bpc_fn!(self, R,  8, 16, 10 bpc, avx512icl);
                    assign_itx16_bpc_fn!(self, R, 16,  8, 10 bpc, avx512icl);
                    assign_itx12_bpc_fn!(self,    16, 16, 10 bpc, avx512icl);
                    assign_itx2_bpc_fn! (self, R,  8, 32, 10 bpc, avx512icl);
                    assign_itx2_bpc_fn! (self, R, 16, 32, 10 bpc, avx512icl);
                    assign_itx2_bpc_fn! (self, R, 32,  8, 10 bpc, avx512icl);
                    assign_itx2_bpc_fn! (self, R, 32, 16, 10 bpc, avx512icl);
                    assign_itx2_bpc_fn! (self,    32, 32, 10 bpc, avx512icl);
                    assign_itx1_bpc_fn! (self, R, 16, 64, 10 bpc, avx512icl);
                    assign_itx1_bpc_fn! (self, R, 32, 64, 10 bpc, avx512icl);
                    assign_itx1_bpc_fn! (self, R, 64, 16, 10 bpc, avx512icl);
                    assign_itx1_bpc_fn! (self, R, 64, 32, 10 bpc, avx512icl);
                    assign_itx1_bpc_fn! (self,    64, 64, 10 bpc, avx512icl);
                }
            }
        }

        self
    }

    #[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
    #[inline(always)]
    const fn init_arm<BD: BitDepth>(mut self, flags: CpuFlags, bpc: u8) -> Self {
        if !flags.contains(CpuFlags::NEON) {
            return self;
        }

        if BD::BITDEPTH == 16 && bpc != 10 {
            return self;
        }

        assign_itx_fn!(self, BD, 4, 4, wht_wht, WHT_WHT, neon);

        #[rustfmt::skip]
        const fn assign<BD: BitDepth>(mut c: Rav1dInvTxfmDSPContext) -> Rav1dInvTxfmDSPContext {
            assign_itx16_fn!(c, BD,     4,  4, neon);
            assign_itx16_fn!(c, BD, R,  4,  8, neon);
            assign_itx16_fn!(c, BD, R,  4, 16, neon);
            assign_itx16_fn!(c, BD, R,  8,  4, neon);
            assign_itx16_fn!(c, BD,     8,  8, neon);
            assign_itx16_fn!(c, BD, R,  8, 16, neon);
            assign_itx16_fn!(c, BD, R, 16,  4, neon);
            assign_itx16_fn!(c, BD, R, 16,  8, neon);
            assign_itx12_fn!(c, BD,    16, 16, neon);
            assign_itx2_fn! (c, BD, R,  8, 32, neon);
            assign_itx2_fn! (c, BD, R, 16, 32, neon);
            assign_itx2_fn! (c, BD, R, 32,  8, neon);
            assign_itx2_fn! (c, BD, R, 32, 16, neon);
            assign_itx2_fn! (c, BD,    32, 32, neon);
            assign_itx1_fn! (c, BD, R, 16, 64, neon);
            assign_itx1_fn! (c, BD, R, 32, 64, neon);
            assign_itx1_fn! (c, BD, R, 64, 16, neon);
            assign_itx1_fn! (c, BD, R, 64, 32, neon);
            assign_itx1_fn! (c, BD,    64, 64, neon);

            c
        }

        assign::<BD>(self)
    }

    #[inline(always)]
    const fn init<BD: BitDepth>(self, flags: CpuFlags, bpc: u8) -> Self {
        #[cfg(feature = "asm")]
        {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            {
                return self.init_x86::<BD>(flags, bpc);
            }
            #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
            {
                return self.init_arm::<BD>(flags, bpc);
            }
        }

        #[allow(unreachable_code)] // Reachable on some #[cfg]s.
        {
            let _ = flags;
            let _ = bpc;
            self
        }
    }

    pub const fn new<BD: BitDepth>(flags: CpuFlags, bpc: u8) -> Self {
        Self::default::<BD>().init::<BD>(flags, bpc)
    }
}
