use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynCoef;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::intops::iclip;
use crate::src::cpu::CpuFlags;
use crate::src::enum_map::DefaultValue;
use crate::src::itx_1d::rav1d_inv_adst16_1d_c;
use crate::src::itx_1d::rav1d_inv_adst4_1d_c;
use crate::src::itx_1d::rav1d_inv_adst8_1d_c;
use crate::src::itx_1d::rav1d_inv_dct16_1d_c;
use crate::src::itx_1d::rav1d_inv_dct32_1d_c;
use crate::src::itx_1d::rav1d_inv_dct4_1d_c;
use crate::src::itx_1d::rav1d_inv_dct64_1d_c;
use crate::src::itx_1d::rav1d_inv_dct8_1d_c;
use crate::src::itx_1d::rav1d_inv_flipadst16_1d_c;
use crate::src::itx_1d::rav1d_inv_flipadst4_1d_c;
use crate::src::itx_1d::rav1d_inv_flipadst8_1d_c;
use crate::src::itx_1d::rav1d_inv_identity16_1d_c;
use crate::src::itx_1d::rav1d_inv_identity32_1d_c;
use crate::src::itx_1d::rav1d_inv_identity4_1d_c;
use crate::src::itx_1d::rav1d_inv_identity8_1d_c;
use crate::src::itx_1d::rav1d_inv_wht4_1d_c;
use crate::src::levels::TxfmType;
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
use crate::src::wrap_fn_ptr::wrap_fn_ptr;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::num::NonZeroUsize;
use std::slice;

#[cfg(all(
    feature = "asm",
    not(any(target_arch = "riscv64", target_arch = "riscv32"))
))]
use crate::include::common::bitdepth::bd_fn;

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
use crate::include::common::bitdepth::bpc_fn;

pub type itx_1d_fn = fn(c: &mut [i32], stride: NonZeroUsize, min: c_int, max: c_int);

#[inline(never)]
unsafe fn inv_txfm_add<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    coeff: *mut BD::Coef,
    eob: c_int,
    w: usize,
    h: usize,
    shift: u8,
    first_1d_fn: itx_1d_fn,
    second_1d_fn: itx_1d_fn,
    has_dc_only: bool,
    bd: BD,
) {
    let bitdepth_max = bd.bitdepth_max().as_::<c_int>();

    assert!(w >= 4 && w <= 64);
    assert!(h >= 4 && h <= 64);
    assert!(eob >= 0);

    let is_rect2 = w * 2 == h || h * 2 == w;
    let rnd = 1 << shift >> 1;

    if eob < has_dc_only as c_int {
        let coeff = slice::from_raw_parts_mut(coeff, 1);

        let mut dc = coeff[0].as_::<c_int>();
        coeff[0] = 0.as_();
        if is_rect2 {
            dc = dc * 181 + 128 >> 8;
        }
        dc = dc * 181 + 128 >> 8;
        dc = dc + rnd >> shift;
        dc = dc * 181 + 128 + 2048 >> 12;
        for _ in 0..h {
            for x in 0..w {
                *dst.add(x) = bd.iclip_pixel((*dst.add(x)).as_::<c_int>() + dc);
            }
            dst = dst.offset(BD::pxstride(stride));
        }
        return;
    }

    let sh = cmp::min(h, 32);
    let sw = cmp::min(w, 32);

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
        first_1d_fn(c, 1.try_into().unwrap(), row_clip_min, row_clip_max);
        c = &mut c[w..];
    }

    coeff.fill(0.into());
    for i in 0..w * sh {
        tmp[i] = iclip(tmp[i] + rnd >> shift, col_clip_min, col_clip_max);
    }

    for x in 0..w {
        second_1d_fn(
            &mut tmp[x..],
            w.try_into().unwrap(),
            col_clip_min,
            col_clip_max,
        );
    }

    for y in 0..h {
        for x in 0..w {
            *dst.add(x) = bd.iclip_pixel((*dst.add(x)).as_::<c_int>() + (tmp[y * w + x] + 8 >> 4));
        }
        dst = dst.offset(BD::pxstride(stride));
    }
}

unsafe fn inv_txfm_add_rust<const W: usize, const H: usize, const TYPE: TxfmType, BD: BitDepth>(
    dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    coeff: *mut BD::Coef,
    eob: c_int,
    bd: BD,
) {
    let shift = match (W, H) {
        (4, 4) => 0,
        (4, 8) => 0,
        (4, 16) => 1,
        (8, 4) => 0,
        (8, 8) => 1,
        (8, 16) => 1,
        (8, 32) => 2,
        (16, 4) => 1,
        (16, 8) => 1,
        (16, 16) => 2,
        (16, 32) => 1,
        (16, 64) => 2,
        (32, 8) => 2,
        (32, 16) => 1,
        (32, 32) => 2,
        (32, 64) => 1,
        (64, 16) => 2,
        (64, 32) => 1,
        (64, 64) => 2,
        _ => unreachable!(),
    };
    let has_dc_only = TYPE == DCT_DCT;

    enum Type {
        Identity,
        Dct,
        Adst,
        FlipAdst,
    }
    use Type::*;
    // For some reason, this is flipped.
    let (second, first) = match TYPE {
        IDTX => (Identity, Identity),
        DCT_DCT => (Dct, Dct),
        ADST_DCT => (Adst, Dct),
        FLIPADST_DCT => (FlipAdst, Dct),
        H_DCT => (Identity, Dct),
        DCT_ADST => (Dct, Adst),
        ADST_ADST => (Adst, Adst),
        FLIPADST_ADST => (FlipAdst, Adst),
        DCT_FLIPADST => (Dct, FlipAdst),
        ADST_FLIPADST => (Adst, FlipAdst),
        FLIPADST_FLIPADST => (FlipAdst, FlipAdst),
        V_DCT => (Dct, Identity),
        H_ADST => (Identity, Adst),
        H_FLIPADST => (Identity, FlipAdst),
        V_ADST => (Adst, Identity),
        V_FLIPADST => (FlipAdst, Identity),
        _ => unreachable!(),
    };

    fn resolve_1d_fn(r#type: Type, n: usize) -> itx_1d_fn {
        match (r#type, n) {
            (Identity, 4) => rav1d_inv_identity4_1d_c,
            (Identity, 8) => rav1d_inv_identity8_1d_c,
            (Identity, 16) => rav1d_inv_identity16_1d_c,
            (Identity, 32) => rav1d_inv_identity32_1d_c,
            (Dct, 4) => rav1d_inv_dct4_1d_c,
            (Dct, 8) => rav1d_inv_dct8_1d_c,
            (Dct, 16) => rav1d_inv_dct16_1d_c,
            (Dct, 32) => rav1d_inv_dct32_1d_c,
            (Dct, 64) => rav1d_inv_dct64_1d_c,
            (Adst, 4) => rav1d_inv_adst4_1d_c,
            (Adst, 8) => rav1d_inv_adst8_1d_c,
            (Adst, 16) => rav1d_inv_adst16_1d_c,
            (FlipAdst, 4) => rav1d_inv_flipadst4_1d_c,
            (FlipAdst, 8) => rav1d_inv_flipadst8_1d_c,
            (FlipAdst, 16) => rav1d_inv_flipadst16_1d_c,
            _ => unreachable!(),
        }
    }

    let first_1d_fn = resolve_1d_fn(first, W);
    let second_1d_fn = resolve_1d_fn(second, H);

    inv_txfm_add(
        dst,
        stride,
        coeff,
        eob,
        W,
        H,
        shift,
        first_1d_fn,
        second_1d_fn,
        has_dc_only,
        bd,
    )
}

unsafe extern "C" fn inv_txfm_add_c_erased<
    const W: usize,
    const H: usize,
    const TYPE: TxfmType,
    BD: BitDepth,
>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    coeff: *mut DynCoef,
    eob: c_int,
    bitdepth_max: c_int,
) {
    let dst = dst.cast();
    let coeff = coeff.cast();
    let bd = BD::from_c(bitdepth_max);
    inv_txfm_add_rust::<W, H, TYPE, BD>(dst, stride, coeff, eob, bd)
}

wrap_fn_ptr!(unsafe extern "C" fn itxfm(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    coeff: *mut DynCoef,
    eob: c_int,
    bitdepth_max: c_int,
) -> ());

impl itxfm::Fn {
    pub unsafe fn call<BD: BitDepth>(
        &self,
        dst: *mut BD::Pixel,
        dst_stride: ptrdiff_t,
        coeff: &mut [BD::Coef],
        eob: c_int,
        bd: BD,
    ) {
        let dst = dst.cast();
        let coeff = coeff.as_mut_ptr().cast();
        let bd = bd.into_c();
        self.get()(dst, dst_stride, coeff, eob, bd)
    }
}

pub struct Rav1dInvTxfmDSPContext {
    pub itxfm_add: [[itxfm::Fn; N_TX_TYPES_PLUS_LL]; N_RECT_TX_SIZES],
}

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
    const H: usize = 4;
    const W: usize = 4;

    let coeff = slice::from_raw_parts_mut(coeff, W * H);

    let mut tmp = [0; W * H];
    let mut c = &mut tmp[..];
    for y in 0..H {
        for x in 0..W {
            c[x] = coeff[y + x * H].as_::<i32>() >> 2;
        }
        rav1d_inv_wht4_1d_c(c, 1.try_into().unwrap());
        c = &mut c[W..];
    }
    coeff.fill(0.into());

    for x in 0..W {
        rav1d_inv_wht4_1d_c(&mut tmp[x..], H.try_into().unwrap());
    }

    for y in 0..H {
        for x in 0..W {
            *dst.add(x) = bd.iclip_pixel((*dst.add(x)).as_::<c_int>() + tmp[y * W + x]);
        }
        dst = dst.offset(BD::pxstride(stride));
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
                = bd_fn!(itxfm::decl_fn, BD, [< inv_txfm_add_ $type _ $w x $h >], $ext);
        }
    }};

    ($c:ident, $BD:ty, $pfx:ident, $w:literal, $h:literal, $type:ident, $type_enum:ident, $ext:ident) => {{
        use paste::paste;

        paste! {
            $c.itxfm_add[[<$pfx TX_ $w X $h>] as usize][$type_enum as usize]
                = bd_fn!(itxfm::decl_fn, BD, [< inv_txfm_add_ $type _ $w x $h >], $ext);
        }
    }};
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
macro_rules! assign_itx_bpc_fn {
    ($c:ident, $pfx:ident, $w:literal, $h:literal, $type:ident, $type_enum:ident, $bpc:literal bpc, $ext:ident) => {{
        use paste::paste;

        paste! {
            $c.itxfm_add[[<$pfx TX_ $w X $h>] as usize][$type_enum as usize]
                = bpc_fn!(itxfm::decl_fn, $bpc bpc, [< inv_txfm_add_ $type _ $w x $h >], $ext);
        }
    }};

    ($c:ident, $w:literal, $h:literal, $type:ident, $type_enum:ident, $bpc:literal bpc, $ext:ident) => {{
        use paste::paste;

        paste! {
            $c.itxfm_add[[<TX_ $w X $h>] as usize][$type_enum as usize]
                = bpc_fn!(itxfm::decl_fn, $bpc bpc, [< inv_txfm_add_ $type _ $w x $h >], $ext);
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

macro_rules! assign_itx_all_fn {
    ($c:expr, $BD:ty, $w:expr, $h:expr, $tx:expr, $type:expr) => {{
        $c.itxfm_add[$tx as usize][$type as usize] =
            itxfm::Fn::new(inv_txfm_add_c_erased::<$w, $h, $type, $BD>);
    }};
}

macro_rules! assign_itx_all_fn64 {
    ($c:ident, $BD:ty, $w:literal, $h:literal) => {{
        use paste::paste;

        let tx = paste! { [<TX_ $w X $h>] };
        assign_itx_all_fn!($c, $BD, $w, $h, tx, DCT_DCT);
    }};

    ($c:ident, $BD:ty, $w:literal, $h:literal, $pfx:ident) => {{
        use paste::paste;

        let tx = paste! { [<$pfx TX_ $w X $h>] };
        assign_itx_all_fn!($c, $BD, $w, $h, tx, DCT_DCT);
    }};
}

macro_rules! assign_itx_all_fn32 {
    ($c:ident, $BD:ty, $w:literal, $h:literal) => {{
        use paste::paste;

        assign_itx_all_fn64!($c, BD, $w, $h);
        let tx = paste! { [<TX_ $w X $h>] };
        assign_itx_all_fn!($c, $BD, $w, $h, tx, IDTX);
    }};

    ($c:ident, $BD:ty, $w:literal, $h:literal, $pfx:ident) => {{
        use paste::paste;

        assign_itx_all_fn64!($c, BD, $w, $h, $pfx);
        let tx = paste! { [<$pfx TX_ $w X $h>] };
        assign_itx_all_fn!($c, $BD, $w, $h, tx, IDTX);
    }};
}

macro_rules! assign_itx_all_fn16 {
    ($c:ident, $BD:ty, $w:literal, $h:literal) => {{
        use paste::paste;

        assign_itx_all_fn32!($c, BD, $w, $h);
        let tx = paste! { [<TX_ $w X $h>] };
        assign_itx_all_fn!($c, $BD, $w, $h, tx, DCT_ADST);
        assign_itx_all_fn!($c, $BD, $w, $h, tx, ADST_DCT);
        assign_itx_all_fn!($c, $BD, $w, $h, tx, ADST_ADST);
        assign_itx_all_fn!($c, $BD, $w, $h, tx, ADST_FLIPADST);
        assign_itx_all_fn!($c, $BD, $w, $h, tx, FLIPADST_ADST);
        assign_itx_all_fn!($c, $BD, $w, $h, tx, DCT_FLIPADST);
        assign_itx_all_fn!($c, $BD, $w, $h, tx, FLIPADST_DCT);
        assign_itx_all_fn!($c, $BD, $w, $h, tx, FLIPADST_FLIPADST);
        assign_itx_all_fn!($c, $BD, $w, $h, tx, H_DCT);
        assign_itx_all_fn!($c, $BD, $w, $h, tx, V_DCT);
    }};

    ($c:ident, $BD:ty, $w:literal, $h:literal, $pfx:ident) => {{
        use paste::paste;

        assign_itx_all_fn32!($c, BD, $w, $h, $pfx);
        let tx = paste! { [<$pfx TX_ $w X $h>] };
        assign_itx_all_fn!($c, $BD, $w, $h, tx, DCT_ADST);
        assign_itx_all_fn!($c, $BD, $w, $h, tx, ADST_DCT);
        assign_itx_all_fn!($c, $BD, $w, $h, tx, ADST_ADST);
        assign_itx_all_fn!($c, $BD, $w, $h, tx, ADST_FLIPADST);
        assign_itx_all_fn!($c, $BD, $w, $h, tx, FLIPADST_ADST);
        assign_itx_all_fn!($c, $BD, $w, $h, tx, DCT_FLIPADST);
        assign_itx_all_fn!($c, $BD, $w, $h, tx, FLIPADST_DCT);
        assign_itx_all_fn!($c, $BD, $w, $h, tx, FLIPADST_FLIPADST);
        assign_itx_all_fn!($c, $BD, $w, $h, tx, H_DCT);
        assign_itx_all_fn!($c, $BD, $w, $h, tx, V_DCT);
    }};
}

macro_rules! assign_itx_all_fn84 {
    ($c:ident, $BD:ty, $w:literal, $h:literal) => {{
        use paste::paste;

        assign_itx_all_fn16!($c, BD, $w, $h);
        let tx = paste! { [<TX_ $w X $h>] };
        assign_itx_all_fn!($c, $BD, $w, $h, tx, H_FLIPADST);
        assign_itx_all_fn!($c, $BD, $w, $h, tx, V_FLIPADST);
        assign_itx_all_fn!($c, $BD, $w, $h, tx, H_ADST);
        assign_itx_all_fn!($c, $BD, $w, $h, tx, V_ADST);
    }};

    ($c:ident, $BD:ty, $w:literal, $h:literal, $pfx:ident) => {{
        use paste::paste;

        assign_itx_all_fn16!($c, BD, $w, $h, $pfx);
        let tx = paste! { [<$pfx TX_ $w X $h>] };
        assign_itx_all_fn!($c, $BD, $w, $h, tx, H_FLIPADST);
        assign_itx_all_fn!($c, $BD, $w, $h, tx, V_FLIPADST);
        assign_itx_all_fn!($c, $BD, $w, $h, tx, H_ADST);
        assign_itx_all_fn!($c, $BD, $w, $h, tx, V_ADST);
    }};
}

impl Rav1dInvTxfmDSPContext {
    pub const fn default<BD: BitDepth>() -> Self {
        let mut c = Self {
            itxfm_add: [[itxfm::Fn::DEFAULT; N_TX_TYPES_PLUS_LL]; N_RECT_TX_SIZES],
        };

        c.itxfm_add[TX_4X4 as usize][WHT_WHT as usize] =
            itxfm::Fn::new(inv_txfm_add_wht_wht_4x4_c_erased::<BD>);

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

        assign_itx_fn!(self, BD, 4, 4, wht_wht, WHT_WHT, neon);

        if BD::BITDEPTH == 16 && bpc != 10 {
            return self;
        }

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
