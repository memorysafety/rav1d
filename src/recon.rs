use std::ops::BitOr;

use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::ToPrimitive;
use crate::include::common::intops::imax;
use crate::include::common::intops::imin;
use crate::include::common::intops::umin;
use crate::include::dav1d::headers::Dav1dPixelLayout;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;
use crate::include::stddef::ptrdiff_t;
use crate::include::stddef::size_t;
use crate::include::stdint::uint16_t;
use crate::include::stdint::uint8_t;
use crate::src::env::get_uv_inter_txtp;
use crate::src::levels::BlockSize;
use crate::src::levels::RectTxfmSize;
use crate::src::levels::TxClass;
use crate::src::levels::TxfmSize;
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
use crate::src::levels::TX_CLASS_2D;
use crate::src::msac::dav1d_msac_decode_bool_equi;
use crate::src::msac::dav1d_msac_decode_bools;
use crate::src::msac::MsacContext;
use crate::src::tables::dav1d_block_dimensions;
use crate::src::tables::dav1d_skip_ctx;
use crate::src::tables::TxfmInfo;

extern "C" {
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::size_t) -> *mut libc::c_void;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}

/// This is a macro that defines a function
/// because it takes `Dav1dFrameContext` and `Dav1dTaskContext` as arguments,
/// which have not yet been deduplicated/genericized over bitdepth.
///
/// TODO: This should not remain a macro.
/// It should either be a `fn` generic over bitdepth
/// or take `struct` arguments that are the subset of fields that are actually used in this `fn`,
/// as this would also solve some borrowck errors that had to be worked around.
macro_rules! define_DEBUG_BLOCK_INFO {
    () => {
        /// TODO: add feature and compile-time guard around this code
        unsafe fn DEBUG_BLOCK_INFO(f: &Dav1dFrameContext, t: &Dav1dTaskContext) -> bool {
            false
                && (*f.frame_hdr).frame_offset == 2
                && t.by >= 0
                && t.by < 4
                && t.bx >= 8
                && t.bx < 12
        }
    };
}

pub(crate) use define_DEBUG_BLOCK_INFO;

use super::levels::Av1Block;
use super::levels::IntraPredMode;
use super::levels::TxfmType;
use super::levels::DCT_DCT;
use super::levels::FILTER_PRED;
use super::levels::IDTX;
use super::levels::TX_CLASS_H;
use super::levels::TX_CLASS_V;
use super::levels::WHT_WHT;
use super::msac::dav1d_msac_decode_bool_adapt;
use super::msac::dav1d_msac_decode_hi_tok;
use super::msac::dav1d_msac_decode_symbol_adapt16;
use super::msac::dav1d_msac_decode_symbol_adapt4;
use super::msac::dav1d_msac_decode_symbol_adapt8;
use super::scan::dav1d_scans;
use super::tables::dav1d_filter_mode_to_y_mode;
use super::tables::dav1d_lo_ctx_offsets;
use super::tables::dav1d_tx_type_class;
use super::tables::dav1d_tx_types_per_set;
use super::tables::dav1d_txfm_dimensions;
use super::tables::dav1d_txtp_from_uvmode;

#[inline]
pub fn read_golomb(msac: &mut MsacContext) -> libc::c_uint {
    let mut len = 0;
    let mut val = 1;

    while !dav1d_msac_decode_bool_equi(msac) && len < 32 {
        len += 1;
    }
    for _ in 0..len {
        val = (val << 1) + dav1d_msac_decode_bool_equi(msac) as libc::c_uint;
    }

    val - 1
}

trait ReadInt {
    /// (Try to or panic) read [`Self`] from the front of `bytes` in native endianness.
    /// These are similar to the [`u32::from_ne_bytes`] type methods,
    /// but generalized into a `trait` to be generic,
    /// and operating on a slice that is sliced and converted into an array first.
    ///
    /// This replaces the previous code that used `unsafe` transmutes through casting.
    fn read_ne(bytes: &[u8]) -> Self;
}

macro_rules! impl_ReadInt {
    ($U:ty) => {
        impl ReadInt for $U {
            fn read_ne(bytes: &[u8]) -> Self {
                let n = std::mem::size_of::<Self>();
                Self::from_ne_bytes(bytes[..n].try_into().unwrap())
            }
        }
    };
}

impl_ReadInt!(u8);
impl_ReadInt!(u16);
impl_ReadInt!(u32);
impl_ReadInt!(u64);
impl_ReadInt!(u128);

trait MergeInt: Sized + Copy {
    type Output: BitOr<Self::Output, Output = Self::Output>;

    fn lo(self) -> Self::Output;

    fn hi(self) -> Self::Output;

    fn merge(self) -> Self::Output {
        self.lo() | self.hi()
    }
}

macro_rules! impl_MergeInt {
    ($T:ty, $U:ty) => {
        impl MergeInt for $T {
            type Output = $U;

            fn lo(self) -> Self::Output {
                self as $U
            }

            fn hi(self) -> Self::Output {
                (self >> <$U>::BITS) as $U
            }
        }
    };
}

impl_MergeInt!(u16, u8);
impl_MergeInt!(u32, u16);
impl_MergeInt!(u64, u32);
impl_MergeInt!(u128, u64);

#[inline]
pub fn get_skip_ctx(
    t_dim: &TxfmInfo,
    bs: BlockSize,
    a: &[u8],
    l: &[u8],
    chroma: libc::c_int,
    layout: Dav1dPixelLayout,
) -> u8 {
    let b_dim = &dav1d_block_dimensions[bs as usize];
    if chroma != 0 {
        let ss_ver = layout == DAV1D_PIXEL_LAYOUT_I420;
        let ss_hor = layout != DAV1D_PIXEL_LAYOUT_I444;
        let not_one_blk = b_dim[2] - (b_dim[2] != 0 && ss_hor) as u8 > t_dim.lw
            || b_dim[3] - (b_dim[3] != 0 && ss_ver) as u8 > t_dim.lh;
        fn merge_ctx<const N: usize>(dir: &[u8]) -> bool {
            dir[..N] != [0x40; N]
        }
        let [ca, cl] = [(a, t_dim.lw), (l, t_dim.lh)].map(|(dir, tx)| match tx as TxfmSize {
            TX_4X4 => merge_ctx::<1>(dir),
            TX_8X8 => merge_ctx::<2>(dir),
            TX_16X16 => merge_ctx::<4>(dir),
            TX_32X32 => merge_ctx::<8>(dir),
            _ => unreachable!(),
        });
        (7 + (not_one_blk as u8) * 3) + (ca as u8) + (cl as u8)
    } else if b_dim[2] == t_dim.lw && b_dim[3] == t_dim.lh {
        0
    } else {
        /// Read and xor all the bytes.
        fn merge_ctx(dir: &[u8], tx: TxfmSize) -> u8 {
            if tx == TX_4X4 {
                u8::read_ne(dir)
            } else {
                (if tx == TX_8X8 {
                    u16::read_ne(dir)
                } else {
                    (if tx == TX_16X16 {
                        u32::read_ne(dir)
                    } else {
                        (if tx == TX_32X32 {
                            u64::read_ne(dir)
                        } else {
                            (if tx == TX_64X64 {
                                u128::read_ne(dir)
                            } else {
                                unreachable!()
                            })
                            .merge()
                        })
                        .merge()
                    })
                    .merge()
                })
                .merge()
            }
        }
        let [la, ll] = [(a, t_dim.lw), (l, t_dim.lh)]
            .map(|(dir, tx)| merge_ctx(dir, tx as TxfmSize))
            .map(|ldir| std::cmp::min(ldir & 0x3f, 4) as usize);
        dav1d_skip_ctx[la][ll]
    }
}

// `tx: RectTxfmSize` arg is also `TxfmSize`.
// `TxfmSize` and `RectTxfmSize` should be part of the same `enum`.
#[inline]
pub fn get_dc_sign_ctx(tx: RectTxfmSize, a: &[u8], l: &[u8]) -> libc::c_uint {
    let mut mask = 0xc0c0c0c0c0c0c0c0 as u64;
    let mut mul = 0x101010101010101 as u64;

    let s = match tx {
        TX_4X4 => {
            let mut t = u8::read_ne(a) as i32 >> 6;
            t += u8::read_ne(l) as i32 >> 6;
            t - 1 - 1
        }
        TX_8X8 => {
            let mut t = u16::read_ne(a) as u32 & mask as u32;
            t += u16::read_ne(l) as u32 & mask as u32;
            t = t.wrapping_mul(0x4040404);
            (t >> 24) as i32 - 2 - 2
        }
        TX_16X16 => {
            let mut t = (u32::read_ne(a) & mask as u32) >> 6;
            t += (u32::read_ne(l) & mask as u32) >> 6;
            t = t.wrapping_mul(mul as u32);
            (t >> 24) as i32 - 4 - 4
        }
        TX_32X32 => {
            let mut t = (u64::read_ne(a) & mask) >> 6;
            t += (u64::read_ne(l) & mask) >> 6;
            t = t.wrapping_mul(mul);
            (t >> 56) as i32 - 8 - 8
        }
        TX_64X64 => {
            let mut t = (u64::read_ne(&a[0..]) & mask) >> 6;
            t += (u64::read_ne(&a[8..]) & mask) >> 6;
            t += (u64::read_ne(&l[0..]) & mask) >> 6;
            t += (u64::read_ne(&l[8..]) & mask) >> 6;
            t = t.wrapping_mul(mul);
            (t >> 56) as i32 - 16 - 16
        }
        RTX_4X8 => {
            let mut t = u8::read_ne(a) as u32 & mask as u32;
            t += u16::read_ne(l) as u32 & mask as u32;
            t = t.wrapping_mul(0x4040404);
            (t >> 24) as i32 - 1 - 2
        }
        RTX_8X4 => {
            let mut t = u16::read_ne(a) as u32 & mask as u32;
            t += u8::read_ne(l) as u32 & mask as u32;
            t = t.wrapping_mul(0x4040404);
            (t >> 24) as i32 - 2 - 1
        }
        RTX_8X16 => {
            let mut t = u16::read_ne(a) as u32 & mask as u32;
            t += u32::read_ne(l) & mask as u32;
            t = (t >> 6).wrapping_mul(mul as u32);
            (t >> 24) as i32 - 2 - 4
        }
        RTX_16X8 => {
            let mut t = u32::read_ne(a) & mask as u32;
            t += u16::read_ne(l) as libc::c_uint & mask as u32;
            t = (t >> 6).wrapping_mul(mul as u32);
            (t >> 24) as i32 - 4 - 2
        }
        RTX_16X32 => {
            let mut t = (u32::read_ne(a) & mask as u32) as u64;
            t += u64::read_ne(l) & mask;
            t = (t >> 6).wrapping_mul(mul);
            (t >> 56) as i32 - 4 - 8
        }
        RTX_32X16 => {
            let mut t = u64::read_ne(a) & mask;
            t += (u32::read_ne(l) & mask as u32) as u64;
            t = (t >> 6).wrapping_mul(mul);
            (t >> 56) as i32 - 8 - 4
        }
        RTX_32X64 => {
            let mut t = (u64::read_ne(&a[0..]) & mask) >> 6;
            t += (u64::read_ne(&l[0..]) & mask) >> 6;
            t += (u64::read_ne(&l[8..]) & mask) >> 6;
            t = t.wrapping_mul(mul);
            (t >> 56) as i32 - 8 - 16
        }
        RTX_64X32 => {
            let mut t = (u64::read_ne(&a[0..]) & mask) >> 6;
            t += (u64::read_ne(&a[8..]) & mask) >> 6;
            t += (u64::read_ne(&l[0..]) & mask) >> 6;
            t = t.wrapping_mul(mul);
            (t >> 56) as i32 - 16 - 8
        }
        RTX_4X16 => {
            let mut t = u8::read_ne(a) as u32 & mask as u32;
            t += u32::read_ne(l) & mask as u32;
            t = (t >> 6).wrapping_mul(mul as u32);
            (t >> 24) as i32 - 1 - 4
        }
        RTX_16X4 => {
            let mut t = u32::read_ne(a) & mask as u32;
            t += u8::read_ne(l) as u32 & mask as u32;
            t = (t >> 6).wrapping_mul(mul as u32);
            (t >> 24) as i32 - 4 - 1
        }
        RTX_8X32 => {
            let mut t = (u16::read_ne(a) as u32 & mask as u32) as u64;
            t += u64::read_ne(l) & mask;
            t = (t >> 6).wrapping_mul(mul);
            (t >> 56) as i32 - 2 - 8
        }
        RTX_32X8 => {
            let mut t = u64::read_ne(a) & mask;
            t += (u16::read_ne(l) as u32 & mask as u32) as u64;
            t = (t >> 6).wrapping_mul(mul);
            (t >> 56) as i32 - 8 - 2
        }
        RTX_16X64 => {
            let mut t = (u32::read_ne(a) & mask as u32) as u64;
            t += u64::read_ne(&l[0..]) & mask;
            t = (t >> 6) + ((u64::read_ne(&l[8..]) & mask) >> 6);
            t = t.wrapping_mul(mul);
            (t >> 56) as i32 - 4 - 16
        }
        RTX_64X16 => {
            let mut t = u64::read_ne(&a[0..]) & mask;
            t += (u32::read_ne(l) & mask as u32) as u64;
            t = (t >> 6) + ((u64::read_ne(&a[8..]) & mask) >> 6);
            t = t.wrapping_mul(mul);
            (t >> 56) as i32 - 16 - 4
        }
        _ => unreachable!(),
    };

    (s != 0) as libc::c_uint + (s > 0) as libc::c_uint
}

#[inline]
pub fn get_lo_ctx(
    levels: &[u8],
    tx_class: TxClass,
    hi_mag: &mut libc::c_uint,
    ctx_offsets: Option<&[[u8; 5]; 5]>,
    x: usize,
    y: usize,
    stride: usize,
) -> usize {
    let level = |y, x| levels[y * stride + x] as usize;

    let mut mag = level(0, 1) + level(1, 0);
    let offset = if tx_class == TX_CLASS_2D {
        mag += level(1, 1);
        *hi_mag = mag as libc::c_uint;
        mag += level(0, 2) + level(2, 0);
        ctx_offsets.unwrap()[std::cmp::min(y, 4)][std::cmp::min(x, 4)] as usize
    } else {
        mag += level(0, 2);
        *hi_mag = mag as libc::c_uint;
        mag += level(0, 3) + level(0, 4);
        26 + if y > 1 { 10 } else { y * 5 }
    };
    offset + if mag > 512 { 4 } else { (mag + 64) >> 7 }
}

// TODO(kkysen) make non-pub once recon callers are deduplicated
pub unsafe fn decode_coefs<BD: BitDepth>(
    t: *mut Dav1dTaskContext,
    a: &mut [u8],
    l: &mut [u8],
    tx: RectTxfmSize,
    bs: BlockSize,
    b: *const Av1Block,
    intra: libc::c_int,
    plane: libc::c_int,
    mut cf: *mut BD::Coef,
    txtp: *mut TxfmType,
    mut res_ctx: *mut uint8_t,
) -> libc::c_int {
    let mut dc_sign_ctx = 0;
    let mut dc_sign = 0;
    let mut dc_dq = 0;
    let mut current_block: u64;
    let ts: *mut Dav1dTileState = (*t).ts;
    let chroma = (plane != 0) as libc::c_int;
    let f: *const Dav1dFrameContext = (*t).f;
    let lossless = (*(*f).frame_hdr).segmentation.lossless[(*b).seg_id as usize];
    let t_dim = &dav1d_txfm_dimensions[tx as usize];
    let dbg = DEBUG_BLOCK_INFO(&*f, &*t) as libc::c_int;
    if dbg != 0 {
        printf(
            b"Start: r=%d\n\0" as *const u8 as *const libc::c_char,
            (*ts).msac.rng,
        );
    }
    let sctx = get_skip_ctx(t_dim, bs, a, l, chroma, (*f).cur.p.layout) as libc::c_int;
    let all_skip = dav1d_msac_decode_bool_adapt(
        &mut (*ts).msac,
        &mut (*ts).cdf.coef.skip[(*t_dim).ctx as usize][sctx as usize],
    ) as libc::c_int;
    if dbg != 0 {
        printf(
            b"Post-non-zero[%d][%d][%d]: r=%d\n\0" as *const u8 as *const libc::c_char,
            (*t_dim).ctx as libc::c_int,
            sctx,
            all_skip,
            (*ts).msac.rng,
        );
    }
    if all_skip != 0 {
        *res_ctx = 0x40 as libc::c_int as uint8_t;
        *txtp = (lossless * WHT_WHT as libc::c_int) as TxfmType;
        return -(1 as libc::c_int);
    }
    if lossless != 0 {
        if !((*t_dim).max as libc::c_int == TX_4X4 as libc::c_int) {
            unreachable!();
        }
        *txtp = WHT_WHT;
    } else if (*t_dim).max as libc::c_int + intra >= TX_64X64 as libc::c_int {
        *txtp = DCT_DCT;
    } else if chroma != 0 {
        *txtp = (if intra != 0 {
            dav1d_txtp_from_uvmode[(*b).c2rust_unnamed.c2rust_unnamed.uv_mode as usize]
                as libc::c_uint
        } else {
            get_uv_inter_txtp(&*t_dim, *txtp) as libc::c_uint
        }) as TxfmType;
    } else if (*(*f).frame_hdr).segmentation.qidx[(*b).seg_id as usize] == 0 {
        *txtp = DCT_DCT;
    } else {
        let mut idx: libc::c_uint = 0;
        if intra != 0 {
            let y_mode_nofilt: IntraPredMode = (if (*b).c2rust_unnamed.c2rust_unnamed.y_mode
                as libc::c_int
                == FILTER_PRED as libc::c_int
            {
                dav1d_filter_mode_to_y_mode[(*b).c2rust_unnamed.c2rust_unnamed.y_angle as usize]
                    as libc::c_int
            } else {
                (*b).c2rust_unnamed.c2rust_unnamed.y_mode as libc::c_int
            }) as IntraPredMode;
            if (*(*f).frame_hdr).reduced_txtp_set != 0
                || (*t_dim).min as libc::c_int == TX_16X16 as libc::c_int
            {
                idx = dav1d_msac_decode_symbol_adapt4(
                    &mut (*ts).msac,
                    &mut (*ts).cdf.m.txtp_intra2[(*t_dim).min as usize][y_mode_nofilt as usize],
                    4 as libc::c_int as size_t,
                );
                *txtp = dav1d_tx_types_per_set
                    [idx.wrapping_add(0 as libc::c_int as libc::c_uint) as usize]
                    as TxfmType;
            } else {
                idx = dav1d_msac_decode_symbol_adapt8(
                    &mut (*ts).msac,
                    &mut (*ts).cdf.m.txtp_intra1[(*t_dim).min as usize][y_mode_nofilt as usize],
                    6 as libc::c_int as size_t,
                );
                *txtp = dav1d_tx_types_per_set
                    [idx.wrapping_add(5 as libc::c_int as libc::c_uint) as usize]
                    as TxfmType;
            }
            if dbg != 0 {
                printf(
                    b"Post-txtp-intra[%d->%d][%d][%d->%d]: r=%d\n\0" as *const u8
                        as *const libc::c_char,
                    tx as libc::c_uint,
                    (*t_dim).min as libc::c_int,
                    y_mode_nofilt as libc::c_uint,
                    idx,
                    *txtp as libc::c_uint,
                    (*ts).msac.rng,
                );
            }
        } else {
            if (*(*f).frame_hdr).reduced_txtp_set != 0
                || (*t_dim).max as libc::c_int == TX_32X32 as libc::c_int
            {
                idx = dav1d_msac_decode_bool_adapt(
                    &mut (*ts).msac,
                    &mut (*ts).cdf.m.txtp_inter3[(*t_dim).min as usize],
                ) as libc::c_uint;
                *txtp = (idx.wrapping_sub(1 as libc::c_int as libc::c_uint)
                    & IDTX as libc::c_int as libc::c_uint) as TxfmType;
            } else if (*t_dim).min as libc::c_int == TX_16X16 as libc::c_int {
                idx = dav1d_msac_decode_symbol_adapt16(
                    &mut (*ts).msac,
                    &mut (*ts).cdf.m.txtp_inter2.0,
                    11 as libc::c_int as size_t,
                );
                *txtp = dav1d_tx_types_per_set
                    [idx.wrapping_add(12 as libc::c_int as libc::c_uint) as usize]
                    as TxfmType;
            } else {
                idx = dav1d_msac_decode_symbol_adapt16(
                    &mut (*ts).msac,
                    &mut (*ts).cdf.m.txtp_inter1[(*t_dim).min as usize],
                    15 as libc::c_int as size_t,
                );
                *txtp = dav1d_tx_types_per_set
                    [idx.wrapping_add(24 as libc::c_int as libc::c_uint) as usize]
                    as TxfmType;
            }
            if dbg != 0 {
                printf(
                    b"Post-txtp-inter[%d->%d][%d->%d]: r=%d\n\0" as *const u8
                        as *const libc::c_char,
                    tx as libc::c_uint,
                    (*t_dim).min as libc::c_int,
                    idx,
                    *txtp as libc::c_uint,
                    (*ts).msac.rng,
                );
            }
        }
    }
    let mut eob_bin = 0;
    let tx2dszctx = imin((*t_dim).lw as libc::c_int, TX_32X32 as libc::c_int)
        + imin((*t_dim).lh as libc::c_int, TX_32X32 as libc::c_int);
    let tx_class: TxClass = dav1d_tx_type_class[*txtp as usize] as TxClass;
    let is_1d =
        (tx_class as libc::c_uint != TX_CLASS_2D as libc::c_int as libc::c_uint) as libc::c_int;
    match tx2dszctx {
        0 => {
            let eob_bin_cdf = &mut (*ts).cdf.coef.eob_bin_16[chroma as usize][is_1d as usize];
            eob_bin =
                dav1d_msac_decode_symbol_adapt4(&mut (*ts).msac, eob_bin_cdf, (4 + 0) as size_t)
                    as libc::c_int;
        }
        1 => {
            let eob_bin_cdf_0 = &mut (*ts).cdf.coef.eob_bin_32[chroma as usize][is_1d as usize];
            eob_bin =
                dav1d_msac_decode_symbol_adapt8(&mut (*ts).msac, eob_bin_cdf_0, (4 + 1) as size_t)
                    as libc::c_int;
        }
        2 => {
            let eob_bin_cdf_1 = &mut (*ts).cdf.coef.eob_bin_64[chroma as usize][is_1d as usize];
            eob_bin =
                dav1d_msac_decode_symbol_adapt8(&mut (*ts).msac, eob_bin_cdf_1, (4 + 2) as size_t)
                    as libc::c_int;
        }
        3 => {
            let eob_bin_cdf_2 = &mut (*ts).cdf.coef.eob_bin_128[chroma as usize][is_1d as usize];
            eob_bin =
                dav1d_msac_decode_symbol_adapt8(&mut (*ts).msac, eob_bin_cdf_2, (4 + 3) as size_t)
                    as libc::c_int;
        }
        4 => {
            let eob_bin_cdf_3 = &mut (*ts).cdf.coef.eob_bin_256[chroma as usize][is_1d as usize];
            eob_bin =
                dav1d_msac_decode_symbol_adapt16(&mut (*ts).msac, eob_bin_cdf_3, (4 + 4) as size_t)
                    as libc::c_int;
        }
        5 => {
            let eob_bin_cdf_4 = &mut (*ts).cdf.coef.eob_bin_512[chroma as usize];
            eob_bin =
                dav1d_msac_decode_symbol_adapt16(&mut (*ts).msac, eob_bin_cdf_4, (4 + 5) as size_t)
                    as libc::c_int;
        }
        6 => {
            let eob_bin_cdf_5 = &mut (*ts).cdf.coef.eob_bin_1024[chroma as usize];
            eob_bin =
                dav1d_msac_decode_symbol_adapt16(&mut (*ts).msac, eob_bin_cdf_5, (4 + 6) as size_t)
                    as libc::c_int;
        }
        _ => {}
    }
    if dbg != 0 {
        printf(
            b"Post-eob_bin_%d[%d][%d][%d]: r=%d\n\0" as *const u8 as *const libc::c_char,
            (16 as libc::c_int) << tx2dszctx,
            chroma,
            is_1d,
            eob_bin,
            (*ts).msac.rng,
        );
    }
    let mut eob = 0;
    if eob_bin > 1 {
        let eob_hi_bit_cdf = &mut (*ts).cdf.coef.eob_hi_bit[(*t_dim).ctx as usize][chroma as usize]
            [eob_bin as usize];
        let eob_hi_bit =
            dav1d_msac_decode_bool_adapt(&mut (*ts).msac, eob_hi_bit_cdf) as libc::c_int;
        if dbg != 0 {
            printf(
                b"Post-eob_hi_bit[%d][%d][%d][%d]: r=%d\n\0" as *const u8 as *const libc::c_char,
                (*t_dim).ctx as libc::c_int,
                chroma,
                eob_bin,
                eob_hi_bit,
                (*ts).msac.rng,
            );
        }
        eob = (((eob_hi_bit | 2) << eob_bin - 2) as libc::c_uint
            | dav1d_msac_decode_bools(&mut (*ts).msac, (eob_bin - 2) as libc::c_uint))
            as libc::c_int;
        if dbg != 0 {
            printf(
                b"Post-eob[%d]: r=%d\n\0" as *const u8 as *const libc::c_char,
                eob,
                (*ts).msac.rng,
            );
        }
    } else {
        eob = eob_bin;
    }
    if !(eob >= 0) {
        unreachable!();
    }
    let eob_cdf: *mut [uint16_t; 4] =
        ((*ts).cdf.coef.eob_base_tok[(*t_dim).ctx as usize][chroma as usize]).as_mut_ptr();
    let hi_cdf: *mut [uint16_t; 4] = ((*ts).cdf.coef.br_tok
        [imin((*t_dim).ctx as libc::c_int, 3 as libc::c_int) as usize][chroma as usize])
        .as_mut_ptr();
    let mut rc: libc::c_uint = 0;
    let mut dc_tok: libc::c_uint = 0;
    if eob != 0 {
        let lo_cdf: *mut [uint16_t; 4] =
            ((*ts).cdf.coef.base_tok[(*t_dim).ctx as usize][chroma as usize]).as_mut_ptr();
        let levels = &mut (*t).scratch.c2rust_unnamed_0.c2rust_unnamed.levels;
        let sw = imin((*t_dim).w as libc::c_int, 8 as libc::c_int);
        let sh = imin((*t_dim).h as libc::c_int, 8 as libc::c_int);
        let mut ctx: libc::c_uint = (1 as libc::c_int
            + (eob > sw * sh * 2) as libc::c_int
            + (eob > sw * sh * 4) as libc::c_int)
            as libc::c_uint;
        let mut eob_tok = dav1d_msac_decode_symbol_adapt4(
            &mut (*ts).msac,
            &mut *eob_cdf.offset(ctx as isize),
            2 as libc::c_int as size_t,
        ) as libc::c_int;
        let mut tok = eob_tok + 1;
        let mut level_tok = tok * 0x41 as libc::c_int;
        let mut mag: libc::c_uint = 0;
        let mut scan: *const uint16_t = 0 as *const uint16_t;
        match tx_class as libc::c_uint {
            0 => {
                let nonsquare_tx: libc::c_uint = (tx as libc::c_uint
                    >= RTX_4X8 as libc::c_int as libc::c_uint)
                    as libc::c_int as libc::c_uint;
                let lo_ctx_offsets = Some(
                    &dav1d_lo_ctx_offsets
                        [nonsquare_tx.wrapping_add(tx as libc::c_uint & nonsquare_tx) as usize],
                );
                scan = dav1d_scans[tx as usize];
                let stride: ptrdiff_t = (4 * sh) as ptrdiff_t;
                let shift: libc::c_uint = (if ((*t_dim).lh as libc::c_int) < 4 {
                    (*t_dim).lh as libc::c_int + 2
                } else {
                    5 as libc::c_int
                }) as libc::c_uint;
                let shift2: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                let mask: libc::c_uint = (4 * sh - 1) as libc::c_uint;
                memset(
                    levels.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    (stride * (4 * sw + 2) as isize) as size_t,
                );
                let mut x: libc::c_uint = 0;
                let mut y: libc::c_uint = 0;
                if TX_CLASS_2D as libc::c_int == TX_CLASS_2D as libc::c_int {
                    rc = *scan.offset(eob as isize) as libc::c_uint;
                    x = rc >> shift;
                    y = rc & mask;
                } else if TX_CLASS_2D as libc::c_int == TX_CLASS_H as libc::c_int {
                    x = eob as libc::c_uint & mask;
                    y = (eob >> shift) as libc::c_uint;
                    rc = eob as libc::c_uint;
                } else {
                    x = eob as libc::c_uint & mask;
                    y = (eob >> shift) as libc::c_uint;
                    rc = x << shift2 | y;
                }
                if dbg != 0 {
                    printf(
                        b"Post-lo_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                            as *const libc::c_char,
                        (*t_dim).ctx as libc::c_int,
                        chroma,
                        ctx,
                        eob,
                        rc,
                        tok,
                        (*ts).msac.rng,
                    );
                }
                if eob_tok == 2 {
                    ctx = (if if TX_CLASS_2D as libc::c_int == TX_CLASS_2D as libc::c_int {
                        (x | y > 1 as libc::c_uint) as libc::c_int
                    } else {
                        (y != 0 as libc::c_int as libc::c_uint) as libc::c_int
                    } != 0
                    {
                        14 as libc::c_int
                    } else {
                        7 as libc::c_int
                    }) as libc::c_uint;
                    tok = dav1d_msac_decode_hi_tok(
                        &mut (*ts).msac,
                        &mut *hi_cdf.offset(ctx as isize),
                    ) as libc::c_int;
                    level_tok = tok + ((3 as libc::c_int) << 6);
                    if dbg != 0 {
                        printf(
                            b"Post-hi_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                as *const libc::c_char,
                            imin((*t_dim).ctx as libc::c_int, 3 as libc::c_int),
                            chroma,
                            ctx,
                            eob,
                            rc,
                            tok,
                            (*ts).msac.rng,
                        );
                    }
                }
                *cf.offset(rc as isize) = (tok << 11).as_::<BD::Coef>();
                levels[(x as isize * stride + y as isize) as usize] = level_tok as uint8_t;
                let mut i = eob - 1;
                while i > 0 {
                    let mut rc_i: libc::c_uint = 0;
                    if TX_CLASS_2D as libc::c_int == TX_CLASS_2D as libc::c_int {
                        rc_i = *scan.offset(i as isize) as libc::c_uint;
                        x = rc_i >> shift;
                        y = rc_i & mask;
                    } else if TX_CLASS_2D as libc::c_int == TX_CLASS_H as libc::c_int {
                        x = i as libc::c_uint & mask;
                        y = (i >> shift) as libc::c_uint;
                        rc_i = i as libc::c_uint;
                    } else {
                        x = i as libc::c_uint & mask;
                        y = (i >> shift) as libc::c_uint;
                        rc_i = x << shift2 | y;
                    }
                    if !(x < 32 as libc::c_uint && y < 32 as libc::c_uint) {
                        unreachable!();
                    }
                    let level = &mut levels[(x as isize * stride + y as isize) as usize..];
                    ctx = get_lo_ctx(
                        level,
                        TX_CLASS_2D,
                        &mut mag,
                        lo_ctx_offsets,
                        x as usize,
                        y as usize,
                        stride as usize,
                    ) as libc::c_uint;
                    if TX_CLASS_2D as libc::c_int == TX_CLASS_2D as libc::c_int {
                        y |= x;
                    }
                    tok = dav1d_msac_decode_symbol_adapt4(
                        &mut (*ts).msac,
                        &mut *lo_cdf.offset(ctx as isize),
                        3 as libc::c_int as size_t,
                    ) as libc::c_int;
                    if dbg != 0 {
                        printf(
                            b"Post-lo_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                as *const libc::c_char,
                            (*t_dim).ctx as libc::c_int,
                            chroma,
                            ctx,
                            i,
                            rc_i,
                            tok,
                            (*ts).msac.rng,
                        );
                    }
                    if tok == 3 {
                        mag &= 63 as libc::c_int as libc::c_uint;
                        ctx = ((if y
                            > (TX_CLASS_2D as libc::c_int == TX_CLASS_2D as libc::c_int)
                                as libc::c_int as libc::c_uint
                        {
                            14 as libc::c_int
                        } else {
                            7 as libc::c_int
                        }) as libc::c_uint)
                            .wrapping_add(if mag > 12 as libc::c_uint {
                                6 as libc::c_int as libc::c_uint
                            } else {
                                mag.wrapping_add(1 as libc::c_int as libc::c_uint) >> 1
                            });
                        tok = dav1d_msac_decode_hi_tok(
                            &mut (*ts).msac,
                            &mut *hi_cdf.offset(ctx as isize),
                        ) as libc::c_int;
                        if dbg != 0 {
                            printf(
                                b"Post-hi_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                    as *const libc::c_char,
                                imin((*t_dim).ctx as libc::c_int, 3 as libc::c_int),
                                chroma,
                                ctx,
                                i,
                                rc_i,
                                tok,
                                (*ts).msac.rng,
                            );
                        }
                        level[0] = (tok + ((3 as libc::c_int) << 6)) as uint8_t;
                        *cf.offset(rc_i as isize) = ((tok << 11) as libc::c_uint | rc).as_::<BD::Coef>();
                        rc = rc_i;
                    } else {
                        tok *= 0x17ff41 as libc::c_int;
                        level[0] = tok as uint8_t;
                        tok = ((tok >> 9) as libc::c_uint
                            & rc.wrapping_add(!(0x7ff as libc::c_uint)))
                            as libc::c_int;
                        if tok != 0 {
                            rc = rc_i;
                        }
                        *cf.offset(rc_i as isize) = tok.as_::<BD::Coef>();
                    }
                    i -= 1;
                }
                ctx = if TX_CLASS_2D as libc::c_int == TX_CLASS_2D as libc::c_int {
                    0 as libc::c_int as libc::c_uint
                } else {
                    get_lo_ctx(
                        levels,
                        TX_CLASS_2D,
                        &mut mag,
                        lo_ctx_offsets,
                        0,
                        0,
                        stride as usize,
                    ) as libc::c_uint
                };
                dc_tok = dav1d_msac_decode_symbol_adapt4(
                    &mut (*ts).msac,
                    &mut *lo_cdf.offset(ctx as isize),
                    3 as libc::c_int as size_t,
                );
                if dbg != 0 {
                    printf(
                        b"Post-dc_lo_tok[%d][%d][%d][%d]: r=%d\n\0" as *const u8
                            as *const libc::c_char,
                        (*t_dim).ctx as libc::c_int,
                        chroma,
                        ctx,
                        dc_tok,
                        (*ts).msac.rng,
                    );
                }
                if dc_tok == 3 as libc::c_uint {
                    if TX_CLASS_2D as libc::c_int == TX_CLASS_2D as libc::c_int {
                        mag = (levels[(0 * stride + 1) as usize] as libc::c_int
                            + levels[(1 * stride + 0) as usize] as libc::c_int
                            + levels[(1 * stride + 1) as usize] as libc::c_int)
                            as libc::c_uint;
                    }
                    mag &= 63 as libc::c_int as libc::c_uint;
                    ctx = if mag > 12 as libc::c_uint {
                        6 as libc::c_int as libc::c_uint
                    } else {
                        mag.wrapping_add(1 as libc::c_int as libc::c_uint) >> 1
                    };
                    dc_tok = dav1d_msac_decode_hi_tok(
                        &mut (*ts).msac,
                        &mut *hi_cdf.offset(ctx as isize),
                    );
                    if dbg != 0 {
                        printf(
                            b"Post-dc_hi_tok[%d][%d][0][%d]: r=%d\n\0" as *const u8
                                as *const libc::c_char,
                            imin((*t_dim).ctx as libc::c_int, 3 as libc::c_int),
                            chroma,
                            dc_tok,
                            (*ts).msac.rng,
                        );
                    }
                }
            }
            1 => {
                let lo_ctx_offsets_0 = None;
                let stride_0: ptrdiff_t = 16 as libc::c_int as ptrdiff_t;
                let shift_0: libc::c_uint = ((*t_dim).lh as libc::c_int + 2) as libc::c_uint;
                let shift2_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                let mask_0: libc::c_uint = (4 * sh - 1) as libc::c_uint;
                memset(
                    levels.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    (stride_0 * (4 * sh + 2) as isize) as usize,
                );
                let mut x_0: libc::c_uint = 0;
                let mut y_0: libc::c_uint = 0;
                if TX_CLASS_H as libc::c_int == TX_CLASS_2D as libc::c_int {
                    rc = *scan.offset(eob as isize) as libc::c_uint;
                    x_0 = rc >> shift_0;
                    y_0 = rc & mask_0;
                } else if TX_CLASS_H as libc::c_int == TX_CLASS_H as libc::c_int {
                    x_0 = eob as libc::c_uint & mask_0;
                    y_0 = (eob >> shift_0) as libc::c_uint;
                    rc = eob as libc::c_uint;
                } else {
                    x_0 = eob as libc::c_uint & mask_0;
                    y_0 = (eob >> shift_0) as libc::c_uint;
                    rc = x_0 << shift2_0 | y_0;
                }
                if dbg != 0 {
                    printf(
                        b"Post-lo_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                            as *const libc::c_char,
                        (*t_dim).ctx as libc::c_int,
                        chroma,
                        ctx,
                        eob,
                        rc,
                        tok,
                        (*ts).msac.rng,
                    );
                }
                if eob_tok == 2 {
                    ctx = (if if TX_CLASS_H as libc::c_int == TX_CLASS_2D as libc::c_int {
                        (x_0 | y_0 > 1 as libc::c_uint) as libc::c_int
                    } else {
                        (y_0 != 0 as libc::c_int as libc::c_uint) as libc::c_int
                    } != 0
                    {
                        14 as libc::c_int
                    } else {
                        7 as libc::c_int
                    }) as libc::c_uint;
                    tok = dav1d_msac_decode_hi_tok(
                        &mut (*ts).msac,
                        &mut *hi_cdf.offset(ctx as isize),
                    ) as libc::c_int;
                    level_tok = tok + ((3 as libc::c_int) << 6);
                    if dbg != 0 {
                        printf(
                            b"Post-hi_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                as *const libc::c_char,
                            imin((*t_dim).ctx as libc::c_int, 3 as libc::c_int),
                            chroma,
                            ctx,
                            eob,
                            rc,
                            tok,
                            (*ts).msac.rng,
                        );
                    }
                }
                *cf.offset(rc as isize) = (tok << 11).as_::<BD::Coef>();
                levels[(x_0 as isize * stride_0 + y_0 as isize) as usize] = level_tok as uint8_t;
                let mut i_0 = eob - 1;
                while i_0 > 0 {
                    let mut rc_i_0: libc::c_uint = 0;
                    if TX_CLASS_H as libc::c_int == TX_CLASS_2D as libc::c_int {
                        rc_i_0 = *scan.offset(i_0 as isize) as libc::c_uint;
                        x_0 = rc_i_0 >> shift_0;
                        y_0 = rc_i_0 & mask_0;
                    } else if TX_CLASS_H as libc::c_int == TX_CLASS_H as libc::c_int {
                        x_0 = i_0 as libc::c_uint & mask_0;
                        y_0 = (i_0 >> shift_0) as libc::c_uint;
                        rc_i_0 = i_0 as libc::c_uint;
                    } else {
                        x_0 = i_0 as libc::c_uint & mask_0;
                        y_0 = (i_0 >> shift_0) as libc::c_uint;
                        rc_i_0 = x_0 << shift2_0 | y_0;
                    }
                    if !(x_0 < 32 as libc::c_uint && y_0 < 32 as libc::c_uint) {
                        unreachable!();
                    }
                    let level_0 = &mut levels[(x_0 as isize * stride_0 + y_0 as isize) as usize..];
                    ctx = get_lo_ctx(
                        level_0,
                        TX_CLASS_H,
                        &mut mag,
                        lo_ctx_offsets_0,
                        x_0 as usize,
                        y_0 as usize,
                        stride_0 as usize,
                    ) as libc::c_uint;
                    if TX_CLASS_H as libc::c_int == TX_CLASS_2D as libc::c_int {
                        y_0 |= x_0;
                    }
                    tok = dav1d_msac_decode_symbol_adapt4(
                        &mut (*ts).msac,
                        &mut *lo_cdf.offset(ctx as isize),
                        3 as libc::c_int as size_t,
                    ) as libc::c_int;
                    if dbg != 0 {
                        printf(
                            b"Post-lo_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                as *const libc::c_char,
                            (*t_dim).ctx as libc::c_int,
                            chroma,
                            ctx,
                            i_0,
                            rc_i_0,
                            tok,
                            (*ts).msac.rng,
                        );
                    }
                    if tok == 3 {
                        mag &= 63 as libc::c_int as libc::c_uint;
                        ctx = ((if y_0
                            > (TX_CLASS_H as libc::c_int == TX_CLASS_2D as libc::c_int)
                                as libc::c_int as libc::c_uint
                        {
                            14 as libc::c_int
                        } else {
                            7 as libc::c_int
                        }) as libc::c_uint)
                            .wrapping_add(if mag > 12 as libc::c_uint {
                                6 as libc::c_int as libc::c_uint
                            } else {
                                mag.wrapping_add(1 as libc::c_int as libc::c_uint) >> 1
                            });
                        tok = dav1d_msac_decode_hi_tok(
                            &mut (*ts).msac,
                            &mut *hi_cdf.offset(ctx as isize),
                        ) as libc::c_int;
                        if dbg != 0 {
                            printf(
                                b"Post-hi_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                    as *const libc::c_char,
                                imin((*t_dim).ctx as libc::c_int, 3 as libc::c_int),
                                chroma,
                                ctx,
                                i_0,
                                rc_i_0,
                                tok,
                                (*ts).msac.rng,
                            );
                        }
                        level_0[0] = (tok + ((3 as libc::c_int) << 6)) as uint8_t;
                        *cf.offset(rc_i_0 as isize) =
                            ((tok << 11) as libc::c_uint | rc).as_::<BD::Coef>();
                        rc = rc_i_0;
                    } else {
                        tok *= 0x17ff41 as libc::c_int;
                        level_0[0] = tok as uint8_t;
                        tok = ((tok >> 9) as libc::c_uint
                            & rc.wrapping_add(!(0x7ff as libc::c_uint)))
                            as libc::c_int;
                        if tok != 0 {
                            rc = rc_i_0;
                        }
                        *cf.offset(rc_i_0 as isize) = tok.as_::<BD::Coef>();
                    }
                    i_0 -= 1;
                }
                ctx = if TX_CLASS_H as libc::c_int == TX_CLASS_2D as libc::c_int {
                    0 as libc::c_int as libc::c_uint
                } else {
                    get_lo_ctx(
                        levels,
                        TX_CLASS_H,
                        &mut mag,
                        lo_ctx_offsets_0,
                        0,
                        0,
                        stride_0 as usize,
                    ) as libc::c_uint
                };
                dc_tok = dav1d_msac_decode_symbol_adapt4(
                    &mut (*ts).msac,
                    &mut *lo_cdf.offset(ctx as isize),
                    3 as libc::c_int as size_t,
                );
                if dbg != 0 {
                    printf(
                        b"Post-dc_lo_tok[%d][%d][%d][%d]: r=%d\n\0" as *const u8
                            as *const libc::c_char,
                        (*t_dim).ctx as libc::c_int,
                        chroma,
                        ctx,
                        dc_tok,
                        (*ts).msac.rng,
                    );
                }
                if dc_tok == 3 as libc::c_uint {
                    if TX_CLASS_H as libc::c_int == TX_CLASS_2D as libc::c_int {
                        mag = (levels[(0 * stride_0 + 1) as usize] as libc::c_int
                            + levels[(1 * stride_0 + 0) as usize] as libc::c_int
                            + levels[(1 * stride_0 + 1) as usize] as libc::c_int)
                            as libc::c_uint;
                    }
                    mag &= 63 as libc::c_int as libc::c_uint;
                    ctx = if mag > 12 as libc::c_uint {
                        6 as libc::c_int as libc::c_uint
                    } else {
                        mag.wrapping_add(1 as libc::c_int as libc::c_uint) >> 1
                    };
                    dc_tok = dav1d_msac_decode_hi_tok(
                        &mut (*ts).msac,
                        &mut *hi_cdf.offset(ctx as isize),
                    );
                    if dbg != 0 {
                        printf(
                            b"Post-dc_hi_tok[%d][%d][0][%d]: r=%d\n\0" as *const u8
                                as *const libc::c_char,
                            imin((*t_dim).ctx as libc::c_int, 3 as libc::c_int),
                            chroma,
                            dc_tok,
                            (*ts).msac.rng,
                        );
                    }
                }
            }
            2 => {
                let lo_ctx_offsets_1 = None;
                let stride_1: ptrdiff_t = 16 as libc::c_int as ptrdiff_t;
                let shift_1: libc::c_uint = ((*t_dim).lw as libc::c_int + 2) as libc::c_uint;
                let shift2_1: libc::c_uint = ((*t_dim).lh as libc::c_int + 2) as libc::c_uint;
                let mask_1: libc::c_uint = (4 * sw - 1) as libc::c_uint;
                memset(
                    levels.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    (stride_1 * (4 * sw + 2) as isize) as size_t,
                );
                let mut x_1: libc::c_uint = 0;
                let mut y_1: libc::c_uint = 0;
                if TX_CLASS_V as libc::c_int == TX_CLASS_2D as libc::c_int {
                    rc = *scan.offset(eob as isize) as libc::c_uint;
                    x_1 = rc >> shift_1;
                    y_1 = rc & mask_1;
                } else if TX_CLASS_V as libc::c_int == TX_CLASS_H as libc::c_int {
                    x_1 = eob as libc::c_uint & mask_1;
                    y_1 = (eob >> shift_1) as libc::c_uint;
                    rc = eob as libc::c_uint;
                } else {
                    x_1 = eob as libc::c_uint & mask_1;
                    y_1 = (eob >> shift_1) as libc::c_uint;
                    rc = x_1 << shift2_1 | y_1;
                }
                if dbg != 0 {
                    printf(
                        b"Post-lo_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                            as *const libc::c_char,
                        (*t_dim).ctx as libc::c_int,
                        chroma,
                        ctx,
                        eob,
                        rc,
                        tok,
                        (*ts).msac.rng,
                    );
                }
                if eob_tok == 2 {
                    ctx = (if if TX_CLASS_V as libc::c_int == TX_CLASS_2D as libc::c_int {
                        (x_1 | y_1 > 1 as libc::c_uint) as libc::c_int
                    } else {
                        (y_1 != 0 as libc::c_int as libc::c_uint) as libc::c_int
                    } != 0
                    {
                        14 as libc::c_int
                    } else {
                        7 as libc::c_int
                    }) as libc::c_uint;
                    tok = dav1d_msac_decode_hi_tok(
                        &mut (*ts).msac,
                        &mut *hi_cdf.offset(ctx as isize),
                    ) as libc::c_int;
                    level_tok = tok + ((3 as libc::c_int) << 6);
                    if dbg != 0 {
                        printf(
                            b"Post-hi_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                as *const libc::c_char,
                            imin((*t_dim).ctx as libc::c_int, 3 as libc::c_int),
                            chroma,
                            ctx,
                            eob,
                            rc,
                            tok,
                            (*ts).msac.rng,
                        );
                    }
                }
                *cf.offset(rc as isize) = (tok << 11).as_::<BD::Coef>();
                levels[(x_1 as isize * stride_1 + y_1 as isize) as usize] = level_tok as uint8_t;
                let mut i_1 = eob - 1;
                while i_1 > 0 {
                    let mut rc_i_1: libc::c_uint = 0;
                    if TX_CLASS_V as libc::c_int == TX_CLASS_2D as libc::c_int {
                        rc_i_1 = *scan.offset(i_1 as isize) as libc::c_uint;
                        x_1 = rc_i_1 >> shift_1;
                        y_1 = rc_i_1 & mask_1;
                    } else if TX_CLASS_V as libc::c_int == TX_CLASS_H as libc::c_int {
                        x_1 = i_1 as libc::c_uint & mask_1;
                        y_1 = (i_1 >> shift_1) as libc::c_uint;
                        rc_i_1 = i_1 as libc::c_uint;
                    } else {
                        x_1 = i_1 as libc::c_uint & mask_1;
                        y_1 = (i_1 >> shift_1) as libc::c_uint;
                        rc_i_1 = x_1 << shift2_1 | y_1;
                    }
                    if !(x_1 < 32 as libc::c_uint && y_1 < 32 as libc::c_uint) {
                        unreachable!();
                    }
                    let level_1 = &mut levels[(x_1 as isize * stride_1 + y_1 as isize) as usize..];
                    ctx = get_lo_ctx(
                        level_1,
                        TX_CLASS_V,
                        &mut mag,
                        lo_ctx_offsets_1,
                        x_1 as usize,
                        y_1 as usize,
                        stride_1 as usize,
                    ) as libc::c_uint;
                    if TX_CLASS_V as libc::c_int == TX_CLASS_2D as libc::c_int {
                        y_1 |= x_1;
                    }
                    tok = dav1d_msac_decode_symbol_adapt4(
                        &mut (*ts).msac,
                        &mut *lo_cdf.offset(ctx as isize),
                        3 as libc::c_int as size_t,
                    ) as libc::c_int;
                    if dbg != 0 {
                        printf(
                            b"Post-lo_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                as *const libc::c_char,
                            (*t_dim).ctx as libc::c_int,
                            chroma,
                            ctx,
                            i_1,
                            rc_i_1,
                            tok,
                            (*ts).msac.rng,
                        );
                    }
                    if tok == 3 {
                        mag &= 63 as libc::c_int as libc::c_uint;
                        ctx = ((if y_1
                            > (TX_CLASS_V as libc::c_int == TX_CLASS_2D as libc::c_int)
                                as libc::c_int as libc::c_uint
                        {
                            14 as libc::c_int
                        } else {
                            7 as libc::c_int
                        }) as libc::c_uint)
                            .wrapping_add(if mag > 12 as libc::c_uint {
                                6 as libc::c_int as libc::c_uint
                            } else {
                                mag.wrapping_add(1 as libc::c_int as libc::c_uint) >> 1
                            });
                        tok = dav1d_msac_decode_hi_tok(
                            &mut (*ts).msac,
                            &mut *hi_cdf.offset(ctx as isize),
                        ) as libc::c_int;
                        if dbg != 0 {
                            printf(
                                b"Post-hi_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                    as *const libc::c_char,
                                imin((*t_dim).ctx as libc::c_int, 3 as libc::c_int),
                                chroma,
                                ctx,
                                i_1,
                                rc_i_1,
                                tok,
                                (*ts).msac.rng,
                            );
                        }
                        level_1[0] = (tok + ((3 as libc::c_int) << 6)) as uint8_t;
                        *cf.offset(rc_i_1 as isize) =
                            ((tok << 11) as libc::c_uint | rc).as_::<BD::Coef>();
                        rc = rc_i_1;
                    } else {
                        tok *= 0x17ff41 as libc::c_int;
                        level_1[0] = tok as uint8_t;
                        tok = ((tok >> 9) as libc::c_uint
                            & rc.wrapping_add(!(0x7ff as libc::c_uint)))
                            as libc::c_int;
                        if tok != 0 {
                            rc = rc_i_1;
                        }
                        *cf.offset(rc_i_1 as isize) = tok.as_::<BD::Coef>();
                    }
                    i_1 -= 1;
                }
                ctx = if TX_CLASS_V as libc::c_int == TX_CLASS_2D as libc::c_int {
                    0 as libc::c_int as libc::c_uint
                } else {
                    get_lo_ctx(
                        levels,
                        TX_CLASS_V,
                        &mut mag,
                        lo_ctx_offsets_1,
                        0,
                        0,
                        stride_1 as usize,
                    ) as libc::c_uint
                };
                dc_tok = dav1d_msac_decode_symbol_adapt4(
                    &mut (*ts).msac,
                    &mut *lo_cdf.offset(ctx as isize),
                    3 as libc::c_int as size_t,
                );
                if dbg != 0 {
                    printf(
                        b"Post-dc_lo_tok[%d][%d][%d][%d]: r=%d\n\0" as *const u8
                            as *const libc::c_char,
                        (*t_dim).ctx as libc::c_int,
                        chroma,
                        ctx,
                        dc_tok,
                        (*ts).msac.rng,
                    );
                }
                if dc_tok == 3 as libc::c_uint {
                    if TX_CLASS_V as libc::c_int == TX_CLASS_2D as libc::c_int {
                        mag = (levels[(0 * stride_1 + 1) as usize] as libc::c_int
                            + levels[(1 * stride_1 + 0) as usize] as libc::c_int
                            + levels[(1 * stride_1 + 1) as usize] as libc::c_int)
                            as libc::c_uint;
                    }
                    mag &= 63 as libc::c_int as libc::c_uint;
                    ctx = if mag > 12 as libc::c_uint {
                        6 as libc::c_int as libc::c_uint
                    } else {
                        mag.wrapping_add(1 as libc::c_int as libc::c_uint) >> 1
                    };
                    dc_tok = dav1d_msac_decode_hi_tok(
                        &mut (*ts).msac,
                        &mut *hi_cdf.offset(ctx as isize),
                    );
                    if dbg != 0 {
                        printf(
                            b"Post-dc_hi_tok[%d][%d][0][%d]: r=%d\n\0" as *const u8
                                as *const libc::c_char,
                            imin((*t_dim).ctx as libc::c_int, 3 as libc::c_int),
                            chroma,
                            dc_tok,
                            (*ts).msac.rng,
                        );
                    }
                }
            }
            _ => {
                if 0 == 0 {
                    unreachable!();
                }
            }
        }
    } else {
        let mut tok_br = dav1d_msac_decode_symbol_adapt4(
            &mut (*ts).msac,
            &mut *eob_cdf.offset(0),
            2 as libc::c_int as size_t,
        ) as libc::c_int;
        dc_tok = (1 + tok_br) as libc::c_uint;
        if dbg != 0 {
            printf(
                b"Post-dc_lo_tok[%d][%d][%d][%d]: r=%d\n\0" as *const u8 as *const libc::c_char,
                (*t_dim).ctx as libc::c_int,
                chroma,
                0 as libc::c_int,
                dc_tok,
                (*ts).msac.rng,
            );
        }
        if tok_br == 2 {
            dc_tok = dav1d_msac_decode_hi_tok(&mut (*ts).msac, &mut *hi_cdf.offset(0));
            if dbg != 0 {
                printf(
                    b"Post-dc_hi_tok[%d][%d][0][%d]: r=%d\n\0" as *const u8 as *const libc::c_char,
                    imin((*t_dim).ctx as libc::c_int, 3 as libc::c_int),
                    chroma,
                    dc_tok,
                    (*ts).msac.rng,
                );
            }
        }
        rc = 0 as libc::c_int as libc::c_uint;
    }
    let dq_tbl: *const uint16_t =
        ((*((*ts).dq).offset((*b).seg_id as isize))[plane as usize]).as_ptr();
    let qm_tbl: *const uint8_t = if (*txtp as libc::c_uint) < IDTX as libc::c_int as libc::c_uint {
        (*f).qm[tx as usize][plane as usize]
    } else {
        0 as *const uint8_t
    };
    let dq_shift = imax(0 as libc::c_int, (*t_dim).ctx as libc::c_int - 2);
    let cf_max = !(!(127 as libc::c_uint)
        << (if 8 == 8 {
            8 as libc::c_int
        } else {
            (*f).cur.p.bpc
        })) as libc::c_int;
    let mut cul_level: libc::c_uint = 0;
    let mut dc_sign_level: libc::c_uint = 0;
    if dc_tok == 0 {
        cul_level = 0 as libc::c_int as libc::c_uint;
        dc_sign_level = ((1 as libc::c_int) << 6) as libc::c_uint;
        if !qm_tbl.is_null() {
            current_block = 10687245492419339872;
        } else {
            current_block = 16948539754621368774;
        }
    } else {
        dc_sign_ctx = get_dc_sign_ctx(tx, a, l) as libc::c_int;
        let dc_sign_cdf = &mut (*ts).cdf.coef.dc_sign[chroma as usize][dc_sign_ctx as usize];
        dc_sign = dav1d_msac_decode_bool_adapt(&mut (*ts).msac, dc_sign_cdf) as libc::c_int;
        if dbg != 0 {
            printf(
                b"Post-dc_sign[%d][%d][%d]: r=%d\n\0" as *const u8 as *const libc::c_char,
                chroma,
                dc_sign_ctx,
                dc_sign,
                (*ts).msac.rng,
            );
        }
        dc_dq = *dq_tbl.offset(0) as libc::c_int;
        dc_sign_level = (dc_sign - 1 & (2 as libc::c_int) << 6) as libc::c_uint;
        if !qm_tbl.is_null() {
            dc_dq = dc_dq * *qm_tbl.offset(0) as libc::c_int + 16 >> 5;
            if dc_tok == 15 as libc::c_uint {
                dc_tok =
                    (read_golomb(&mut (*ts).msac)).wrapping_add(15 as libc::c_int as libc::c_uint);
                if dbg != 0 {
                    printf(
                        b"Post-dc_residual[%d->%d]: r=%d\n\0" as *const u8 as *const libc::c_char,
                        dc_tok.wrapping_sub(15 as libc::c_int as libc::c_uint),
                        dc_tok,
                        (*ts).msac.rng,
                    );
                }
                dc_tok &= 0xfffff as libc::c_int as libc::c_uint;
                dc_dq = ((dc_dq as libc::c_uint).wrapping_mul(dc_tok)
                    & 0xffffff as libc::c_int as libc::c_uint)
                    as libc::c_int;
            } else {
                dc_dq = (dc_dq as libc::c_uint).wrapping_mul(dc_tok) as libc::c_int as libc::c_int;
                if !(dc_dq <= 0xffffff as libc::c_int) {
                    unreachable!();
                }
            }
            cul_level = dc_tok;
            dc_dq >>= dq_shift;
            dc_dq = umin(dc_dq as libc::c_uint, (cf_max + dc_sign) as libc::c_uint) as libc::c_int;
            *cf.offset(0) = (if dc_sign != 0 { -dc_dq } else { dc_dq }).as_::<BD::Coef>();
            if rc != 0 {
                current_block = 10687245492419339872;
            } else {
                current_block = 15494703142406051947;
            }
        } else {
            if dc_tok == 15 as libc::c_uint {
                dc_tok =
                    (read_golomb(&mut (*ts).msac)).wrapping_add(15 as libc::c_int as libc::c_uint);
                if dbg != 0 {
                    printf(
                        b"Post-dc_residual[%d->%d]: r=%d\n\0" as *const u8 as *const libc::c_char,
                        dc_tok.wrapping_sub(15 as libc::c_int as libc::c_uint),
                        dc_tok,
                        (*ts).msac.rng,
                    );
                }
                dc_tok &= 0xfffff as libc::c_int as libc::c_uint;
                dc_dq = (((dc_dq as libc::c_uint).wrapping_mul(dc_tok)
                    & 0xffffff as libc::c_int as libc::c_uint)
                    >> dq_shift) as libc::c_int;
                dc_dq =
                    umin(dc_dq as libc::c_uint, (cf_max + dc_sign) as libc::c_uint) as libc::c_int;
            } else {
                dc_dq = ((dc_dq as libc::c_uint).wrapping_mul(dc_tok) >> dq_shift) as libc::c_int;
                if !(dc_dq <= cf_max) {
                    unreachable!();
                }
            }
            cul_level = dc_tok;
            *cf.offset(0) = (if dc_sign != 0 { -dc_dq } else { dc_dq }).as_::<BD::Coef>();
            if rc != 0 {
                current_block = 16948539754621368774;
            } else {
                current_block = 15494703142406051947;
            }
        }
    }
    match current_block {
        10687245492419339872 => {
            let ac_dq: libc::c_uint = *dq_tbl.offset(1) as libc::c_uint;
            loop {
                let sign = dav1d_msac_decode_bool_equi(&mut (*ts).msac) as libc::c_int;
                if dbg != 0 {
                    printf(
                        b"Post-sign[%d=%d]: r=%d\n\0" as *const u8 as *const libc::c_char,
                        rc,
                        sign,
                        (*ts).msac.rng,
                    );
                }
                let rc_tok: libc::c_uint = (*cf.offset(rc as isize)).as_::<libc::c_uint>();
                let mut tok_0: libc::c_uint = 0;
                let mut dq: libc::c_uint = ac_dq
                    .wrapping_mul(*qm_tbl.offset(rc as isize) as libc::c_uint)
                    .wrapping_add(16 as libc::c_int as libc::c_uint)
                    >> 5;
                let mut dq_sat = 0;
                if rc_tok >= ((15 as libc::c_int) << 11) as libc::c_uint {
                    tok_0 = (read_golomb(&mut (*ts).msac))
                        .wrapping_add(15 as libc::c_int as libc::c_uint);
                    if dbg != 0 {
                        printf(
                            b"Post-residual[%d=%d->%d]: r=%d\n\0" as *const u8
                                as *const libc::c_char,
                            rc,
                            tok_0.wrapping_sub(15 as libc::c_int as libc::c_uint),
                            tok_0,
                            (*ts).msac.rng,
                        );
                    }
                    tok_0 &= 0xfffff as libc::c_int as libc::c_uint;
                    dq = dq.wrapping_mul(tok_0) & 0xffffff as libc::c_int as libc::c_uint;
                } else {
                    tok_0 = rc_tok >> 11;
                    dq = dq.wrapping_mul(tok_0);
                    if !(dq <= 0xffffff as libc::c_int as libc::c_uint) {
                        unreachable!();
                    }
                }
                cul_level = cul_level.wrapping_add(tok_0);
                dq >>= dq_shift;
                dq_sat = umin(dq, (cf_max + sign) as libc::c_uint) as libc::c_int;
                *cf.offset(rc as isize) = (if sign != 0 { -dq_sat } else { dq_sat }).as_::<BD::Coef>();
                rc = rc_tok & 0x3ff as libc::c_int as libc::c_uint;
                if !(rc != 0) {
                    break;
                }
            }
        }
        16948539754621368774 => {
            let ac_dq_0: libc::c_uint = *dq_tbl.offset(1) as libc::c_uint;
            loop {
                let sign_0 = dav1d_msac_decode_bool_equi(&mut (*ts).msac) as libc::c_int;
                if dbg != 0 {
                    printf(
                        b"Post-sign[%d=%d]: r=%d\n\0" as *const u8 as *const libc::c_char,
                        rc,
                        sign_0,
                        (*ts).msac.rng,
                    );
                }
                let rc_tok_0: libc::c_uint = (*cf.offset(rc as isize)).as_::<libc::c_uint>();
                let mut tok_1: libc::c_uint = 0;
                let mut dq_0 = 0;
                if rc_tok_0 >= ((15 as libc::c_int) << 11) as libc::c_uint {
                    tok_1 = (read_golomb(&mut (*ts).msac))
                        .wrapping_add(15 as libc::c_int as libc::c_uint);
                    if dbg != 0 {
                        printf(
                            b"Post-residual[%d=%d->%d]: r=%d\n\0" as *const u8
                                as *const libc::c_char,
                            rc,
                            tok_1.wrapping_sub(15 as libc::c_int as libc::c_uint),
                            tok_1,
                            (*ts).msac.rng,
                        );
                    }
                    tok_1 &= 0xfffff as libc::c_int as libc::c_uint;
                    dq_0 = ((ac_dq_0.wrapping_mul(tok_1) & 0xffffff as libc::c_int as libc::c_uint)
                        >> dq_shift) as libc::c_int;
                    dq_0 = umin(dq_0 as libc::c_uint, (cf_max + sign_0) as libc::c_uint)
                        as libc::c_int;
                } else {
                    tok_1 = rc_tok_0 >> 11;
                    dq_0 = (ac_dq_0.wrapping_mul(tok_1) >> dq_shift) as libc::c_int;
                    if !(dq_0 <= cf_max) {
                        unreachable!();
                    }
                }
                cul_level = cul_level.wrapping_add(tok_1);
                *cf.offset(rc as isize) = (if sign_0 != 0 { -dq_0 } else { dq_0 }).as_::<BD::Coef>();
                rc = rc_tok_0 & 0x3ff as libc::c_int as libc::c_uint;
                if !(rc != 0) {
                    break;
                }
            }
        }
        _ => {}
    }
    *res_ctx = (umin(cul_level, 63 as libc::c_int as libc::c_uint) | dc_sign_level) as uint8_t;
    return eob;
}
