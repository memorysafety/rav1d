use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynCoef;
use crate::include::common::bitdepth::BPC;
use crate::include::common::dump::coef_dump;
use crate::include::common::dump::hex_dump;
use crate::include::common::intops::apply_sign64;
use crate::include::common::intops::iclip;
use crate::include::dav1d::headers::Dav1dPixelLayout;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I400;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I444;
use crate::src::ctx::CaseSet;
use crate::src::env::get_uv_inter_txtp;
use crate::src::internal::CodedBlockInfo;
use crate::src::internal::Rav1dDSPContext;
use crate::src::internal::Rav1dFrameContext;
use crate::src::internal::Rav1dTaskContext;
use crate::src::internal::Rav1dTileState;
use crate::src::intra_edge::EdgeFlags;
use crate::src::levels::mv;
use crate::src::levels::Av1Block;
use crate::src::levels::BlockSize;
use crate::src::levels::Filter2d;
use crate::src::levels::IntraPredMode;
use crate::src::levels::RectTxfmSize;
use crate::src::levels::TxClass;
use crate::src::levels::TxfmSize;
use crate::src::levels::TxfmType;
use crate::src::levels::DCT_DCT;
use crate::src::levels::FILTER_PRED;
use crate::src::levels::IDTX;
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
use crate::src::levels::TX_CLASS_H;
use crate::src::levels::TX_CLASS_V;
use crate::src::levels::WHT_WHT;
use crate::src::msac::rav1d_msac_decode_bool_adapt;
use crate::src::msac::rav1d_msac_decode_bool_equi;
use crate::src::msac::rav1d_msac_decode_bools;
use crate::src::msac::rav1d_msac_decode_hi_tok;
use crate::src::msac::rav1d_msac_decode_symbol_adapt16;
use crate::src::msac::rav1d_msac_decode_symbol_adapt4;
use crate::src::msac::rav1d_msac_decode_symbol_adapt8;
use crate::src::msac::MsacContext;
use crate::src::picture::Rav1dThreadPicture;
use crate::src::refmvs::refmvs_block;
use crate::src::scan::dav1d_scans;
use crate::src::tables::dav1d_block_dimensions;
use crate::src::tables::dav1d_filter_2d;
use crate::src::tables::dav1d_filter_mode_to_y_mode;
use crate::src::tables::dav1d_lo_ctx_offsets;
use crate::src::tables::dav1d_skip_ctx;
use crate::src::tables::dav1d_tx_type_class;
use crate::src::tables::dav1d_tx_types_per_set;
use crate::src::tables::dav1d_txfm_dimensions;
use crate::src::tables::dav1d_txtp_from_uvmode;
use crate::src::tables::TxfmInfo;
use libc::intptr_t;
use libc::memset;
use libc::printf;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_longlong;
use std::ffi::c_uint;
use std::ffi::c_ulong;
use std::ffi::c_void;
use std::ops::BitOr;

/// TODO: add feature and compile-time guard around this code
pub(crate) unsafe fn DEBUG_BLOCK_INFO(f: &Rav1dFrameContext, t: &Rav1dTaskContext) -> bool {
    false && (*f.frame_hdr).frame_offset == 2 && t.by >= 0 && t.by < 4 && t.bx >= 8 && t.bx < 12
}

pub(crate) type recon_b_intra_fn = Option<
    unsafe extern "C" fn(*mut Rav1dTaskContext, BlockSize, EdgeFlags, *const Av1Block) -> (),
>;

pub(crate) type recon_b_inter_fn =
    Option<unsafe extern "C" fn(*mut Rav1dTaskContext, BlockSize, *const Av1Block) -> c_int>;

pub(crate) type filter_sbrow_fn = Option<unsafe extern "C" fn(*mut Rav1dFrameContext, c_int) -> ()>;

pub(crate) type backup_ipred_edge_fn = Option<unsafe extern "C" fn(*mut Rav1dTaskContext) -> ()>;

pub(crate) type read_coef_blocks_fn =
    Option<unsafe extern "C" fn(*mut Rav1dTaskContext, BlockSize, *const Av1Block) -> ()>;

#[inline]
fn read_golomb(msac: &mut MsacContext) -> c_uint {
    let mut len = 0;
    let mut val = 1;

    while !rav1d_msac_decode_bool_equi(msac) && len < 32 {
        len += 1;
    }
    for _ in 0..len {
        val = (val << 1) + rav1d_msac_decode_bool_equi(msac) as c_uint;
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
fn get_skip_ctx(
    t_dim: &TxfmInfo,
    bs: BlockSize,
    a: &[u8],
    l: &[u8],
    chroma: c_int,
    layout: Dav1dPixelLayout,
) -> u8 {
    let b_dim = &dav1d_block_dimensions[bs as usize];
    if chroma != 0 {
        let ss_ver = layout == RAV1D_PIXEL_LAYOUT_I420;
        let ss_hor = layout != RAV1D_PIXEL_LAYOUT_I444;
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
            .map(|ldir| cmp::min(ldir & 0x3f, 4) as usize);
        dav1d_skip_ctx[la][ll]
    }
}

// `tx: RectTxfmSize` arg is also `TxfmSize`.
// `TxfmSize` and `RectTxfmSize` should be part of the same `enum`.
#[inline]
fn get_dc_sign_ctx(tx: RectTxfmSize, a: &[u8], l: &[u8]) -> c_uint {
    let mask = 0xc0c0c0c0c0c0c0c0 as u64;
    let mul = 0x101010101010101 as u64;

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
            t += u16::read_ne(l) as c_uint & mask as u32;
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

    (s != 0) as c_uint + (s > 0) as c_uint
}

#[inline]
fn get_lo_ctx(
    levels: &[u8],
    tx_class: TxClass,
    hi_mag: &mut c_uint,
    ctx_offsets: Option<&[[u8; 5]; 5]>,
    x: usize,
    y: usize,
    stride: usize,
) -> usize {
    let level = |y, x| levels[y * stride + x] as usize;

    let mut mag = level(0, 1) + level(1, 0);
    let offset = if tx_class == TX_CLASS_2D {
        mag += level(1, 1);
        *hi_mag = mag as c_uint;
        mag += level(0, 2) + level(2, 0);
        ctx_offsets.unwrap()[cmp::min(y, 4)][cmp::min(x, 4)] as usize
    } else {
        mag += level(0, 2);
        *hi_mag = mag as c_uint;
        mag += level(0, 3) + level(0, 4);
        26 + if y > 1 { 10 } else { y * 5 }
    };
    offset + if mag > 512 { 4 } else { (mag + 64) >> 7 }
}

// TODO(kkysen) pub(crate) temporarily until recon is fully deduplicated
pub(crate) unsafe fn decode_coefs<BD: BitDepth>(
    t: *mut Rav1dTaskContext,
    a: &mut [u8],
    l: &mut [u8],
    tx: RectTxfmSize,
    bs: BlockSize,
    b: *const Av1Block,
    intra: c_int,
    plane: c_int,
    cf: *mut BD::Coef,
    txtp: *mut TxfmType,
    res_ctx: *mut u8,
) -> c_int {
    let dc_sign_ctx;
    let dc_sign;
    let mut dc_dq;
    let current_block: u64;
    let ts: *mut Rav1dTileState = (*t).ts;
    let chroma = (plane != 0) as c_int;
    let f: *const Rav1dFrameContext = (*t).f;
    let lossless = (*(*f).frame_hdr).segmentation.lossless[(*b).seg_id as usize];
    let t_dim = &dav1d_txfm_dimensions[tx as usize];
    let dbg = DEBUG_BLOCK_INFO(&*f, &*t) as c_int;
    if dbg != 0 {
        printf(
            b"Start: r=%d\n\0" as *const u8 as *const c_char,
            (*ts).msac.rng,
        );
    }
    let sctx = get_skip_ctx(t_dim, bs, a, l, chroma, (*f).cur.p.layout) as c_int;
    let all_skip = rav1d_msac_decode_bool_adapt(
        &mut (*ts).msac,
        &mut (*ts).cdf.coef.skip[(*t_dim).ctx as usize][sctx as usize],
    ) as c_int;
    if dbg != 0 {
        printf(
            b"Post-non-zero[%d][%d][%d]: r=%d\n\0" as *const u8 as *const c_char,
            (*t_dim).ctx as c_int,
            sctx,
            all_skip,
            (*ts).msac.rng,
        );
    }
    if all_skip != 0 {
        *res_ctx = 0x40 as c_int as u8;
        *txtp = (lossless * WHT_WHT as c_int) as TxfmType;
        return -(1 as c_int);
    }
    if lossless != 0 {
        if !((*t_dim).max as c_int == TX_4X4 as c_int) {
            unreachable!();
        }
        *txtp = WHT_WHT;
    } else if (*t_dim).max as c_int + intra >= TX_64X64 as c_int {
        *txtp = DCT_DCT;
    } else if chroma != 0 {
        *txtp = (if intra != 0 {
            dav1d_txtp_from_uvmode[(*b).c2rust_unnamed.c2rust_unnamed.uv_mode as usize] as c_uint
        } else {
            get_uv_inter_txtp(&*t_dim, *txtp) as c_uint
        }) as TxfmType;
    } else if (*(*f).frame_hdr).segmentation.qidx[(*b).seg_id as usize] == 0 {
        *txtp = DCT_DCT;
    } else {
        let idx: c_uint;
        if intra != 0 {
            let y_mode_nofilt: IntraPredMode =
                (if (*b).c2rust_unnamed.c2rust_unnamed.y_mode as c_int == FILTER_PRED as c_int {
                    dav1d_filter_mode_to_y_mode[(*b).c2rust_unnamed.c2rust_unnamed.y_angle as usize]
                        as c_int
                } else {
                    (*b).c2rust_unnamed.c2rust_unnamed.y_mode as c_int
                }) as IntraPredMode;
            if (*(*f).frame_hdr).reduced_txtp_set != 0 || (*t_dim).min as c_int == TX_16X16 as c_int
            {
                idx = rav1d_msac_decode_symbol_adapt4(
                    &mut (*ts).msac,
                    &mut (*ts).cdf.m.txtp_intra2[(*t_dim).min as usize][y_mode_nofilt as usize],
                    4 as c_int as usize,
                );
                *txtp = dav1d_tx_types_per_set[idx.wrapping_add(0 as c_int as c_uint) as usize]
                    as TxfmType;
            } else {
                idx = rav1d_msac_decode_symbol_adapt8(
                    &mut (*ts).msac,
                    &mut (*ts).cdf.m.txtp_intra1[(*t_dim).min as usize][y_mode_nofilt as usize],
                    6 as c_int as usize,
                );
                *txtp = dav1d_tx_types_per_set[idx.wrapping_add(5 as c_int as c_uint) as usize]
                    as TxfmType;
            }
            if dbg != 0 {
                printf(
                    b"Post-txtp-intra[%d->%d][%d][%d->%d]: r=%d\n\0" as *const u8 as *const c_char,
                    tx as c_uint,
                    (*t_dim).min as c_int,
                    y_mode_nofilt as c_uint,
                    idx,
                    *txtp as c_uint,
                    (*ts).msac.rng,
                );
            }
        } else {
            if (*(*f).frame_hdr).reduced_txtp_set != 0 || (*t_dim).max as c_int == TX_32X32 as c_int
            {
                idx = rav1d_msac_decode_bool_adapt(
                    &mut (*ts).msac,
                    &mut (*ts).cdf.m.txtp_inter3[(*t_dim).min as usize],
                ) as c_uint;
                *txtp =
                    (idx.wrapping_sub(1 as c_int as c_uint) & IDTX as c_int as c_uint) as TxfmType;
            } else if (*t_dim).min as c_int == TX_16X16 as c_int {
                idx = rav1d_msac_decode_symbol_adapt16(
                    &mut (*ts).msac,
                    &mut (*ts).cdf.m.txtp_inter2.0,
                    11 as c_int as usize,
                );
                *txtp = dav1d_tx_types_per_set[idx.wrapping_add(12 as c_int as c_uint) as usize]
                    as TxfmType;
            } else {
                idx = rav1d_msac_decode_symbol_adapt16(
                    &mut (*ts).msac,
                    &mut (*ts).cdf.m.txtp_inter1[(*t_dim).min as usize],
                    15 as c_int as usize,
                );
                *txtp = dav1d_tx_types_per_set[idx.wrapping_add(24 as c_int as c_uint) as usize]
                    as TxfmType;
            }
            if dbg != 0 {
                printf(
                    b"Post-txtp-inter[%d->%d][%d->%d]: r=%d\n\0" as *const u8 as *const c_char,
                    tx as c_uint,
                    (*t_dim).min as c_int,
                    idx,
                    *txtp as c_uint,
                    (*ts).msac.rng,
                );
            }
        }
    }
    let mut eob_bin = 0;
    let tx2dszctx = cmp::min((*t_dim).lw as c_int, TX_32X32 as c_int)
        + cmp::min((*t_dim).lh as c_int, TX_32X32 as c_int);
    let tx_class: TxClass = dav1d_tx_type_class[*txtp as usize] as TxClass;
    let is_1d = (tx_class as c_uint != TX_CLASS_2D as c_int as c_uint) as c_int;
    match tx2dszctx {
        0 => {
            let eob_bin_cdf = &mut (*ts).cdf.coef.eob_bin_16[chroma as usize][is_1d as usize];
            eob_bin =
                rav1d_msac_decode_symbol_adapt4(&mut (*ts).msac, eob_bin_cdf, (4 + 0) as usize)
                    as c_int;
        }
        1 => {
            let eob_bin_cdf_0 = &mut (*ts).cdf.coef.eob_bin_32[chroma as usize][is_1d as usize];
            eob_bin =
                rav1d_msac_decode_symbol_adapt8(&mut (*ts).msac, eob_bin_cdf_0, (4 + 1) as usize)
                    as c_int;
        }
        2 => {
            let eob_bin_cdf_1 = &mut (*ts).cdf.coef.eob_bin_64[chroma as usize][is_1d as usize];
            eob_bin =
                rav1d_msac_decode_symbol_adapt8(&mut (*ts).msac, eob_bin_cdf_1, (4 + 2) as usize)
                    as c_int;
        }
        3 => {
            let eob_bin_cdf_2 = &mut (*ts).cdf.coef.eob_bin_128[chroma as usize][is_1d as usize];
            eob_bin =
                rav1d_msac_decode_symbol_adapt8(&mut (*ts).msac, eob_bin_cdf_2, (4 + 3) as usize)
                    as c_int;
        }
        4 => {
            let eob_bin_cdf_3 = &mut (*ts).cdf.coef.eob_bin_256[chroma as usize][is_1d as usize];
            eob_bin =
                rav1d_msac_decode_symbol_adapt16(&mut (*ts).msac, eob_bin_cdf_3, (4 + 4) as usize)
                    as c_int;
        }
        5 => {
            let eob_bin_cdf_4 = &mut (*ts).cdf.coef.eob_bin_512[chroma as usize];
            eob_bin =
                rav1d_msac_decode_symbol_adapt16(&mut (*ts).msac, eob_bin_cdf_4, (4 + 5) as usize)
                    as c_int;
        }
        6 => {
            let eob_bin_cdf_5 = &mut (*ts).cdf.coef.eob_bin_1024[chroma as usize];
            eob_bin =
                rav1d_msac_decode_symbol_adapt16(&mut (*ts).msac, eob_bin_cdf_5, (4 + 6) as usize)
                    as c_int;
        }
        _ => {}
    }
    if dbg != 0 {
        printf(
            b"Post-eob_bin_%d[%d][%d][%d]: r=%d\n\0" as *const u8 as *const c_char,
            (16 as c_int) << tx2dszctx,
            chroma,
            is_1d,
            eob_bin,
            (*ts).msac.rng,
        );
    }
    let eob;
    if eob_bin > 1 {
        let eob_hi_bit_cdf = &mut (*ts).cdf.coef.eob_hi_bit[(*t_dim).ctx as usize][chroma as usize]
            [eob_bin as usize];
        let eob_hi_bit = rav1d_msac_decode_bool_adapt(&mut (*ts).msac, eob_hi_bit_cdf) as c_int;
        if dbg != 0 {
            printf(
                b"Post-eob_hi_bit[%d][%d][%d][%d]: r=%d\n\0" as *const u8 as *const c_char,
                (*t_dim).ctx as c_int,
                chroma,
                eob_bin,
                eob_hi_bit,
                (*ts).msac.rng,
            );
        }
        eob = (((eob_hi_bit | 2) << eob_bin - 2) as c_uint
            | rav1d_msac_decode_bools(&mut (*ts).msac, (eob_bin - 2) as c_uint))
            as c_int;
        if dbg != 0 {
            printf(
                b"Post-eob[%d]: r=%d\n\0" as *const u8 as *const c_char,
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
    let eob_cdf: *mut [u16; 4] =
        ((*ts).cdf.coef.eob_base_tok[(*t_dim).ctx as usize][chroma as usize]).as_mut_ptr();
    let hi_cdf: *mut [u16; 4] = ((*ts).cdf.coef.br_tok
        [cmp::min((*t_dim).ctx as c_int, 3 as c_int) as usize][chroma as usize])
        .as_mut_ptr();
    let mut rc: c_uint = 0;
    let mut dc_tok: c_uint = 0;
    if eob != 0 {
        let lo_cdf: *mut [u16; 4] =
            ((*ts).cdf.coef.base_tok[(*t_dim).ctx as usize][chroma as usize]).as_mut_ptr();
        let levels = &mut (*t).scratch.c2rust_unnamed_0.c2rust_unnamed.levels;
        let sw = cmp::min((*t_dim).w as c_int, 8 as c_int);
        let sh = cmp::min((*t_dim).h as c_int, 8 as c_int);
        let mut ctx: c_uint =
            (1 as c_int + (eob > sw * sh * 2) as c_int + (eob > sw * sh * 4) as c_int) as c_uint;
        let eob_tok = rav1d_msac_decode_symbol_adapt4(
            &mut (*ts).msac,
            &mut *eob_cdf.offset(ctx as isize),
            2 as c_int as usize,
        ) as c_int;
        let mut tok = eob_tok + 1;
        let mut level_tok = tok * 0x41 as c_int;
        let mut mag: c_uint = 0;
        let mut scan: *const u16 = 0 as *const u16;
        match tx_class as c_uint {
            0 => {
                let nonsquare_tx: c_uint =
                    (tx as c_uint >= RTX_4X8 as c_int as c_uint) as c_int as c_uint;
                let lo_ctx_offsets = Some(
                    &dav1d_lo_ctx_offsets
                        [nonsquare_tx.wrapping_add(tx as c_uint & nonsquare_tx) as usize],
                );
                scan = dav1d_scans[tx as usize].as_ptr();
                let stride: ptrdiff_t = (4 * sh) as ptrdiff_t;
                let shift: c_uint = (if ((*t_dim).lh as c_int) < 4 {
                    (*t_dim).lh as c_int + 2
                } else {
                    5 as c_int
                }) as c_uint;
                let shift2: c_uint = 0 as c_int as c_uint;
                let mask: c_uint = (4 * sh - 1) as c_uint;
                memset(
                    levels.as_mut_ptr() as *mut c_void,
                    0 as c_int,
                    (stride * (4 * sw as isize + 2)) as usize,
                );
                let mut x: c_uint;
                let mut y: c_uint;
                if TX_CLASS_2D as c_int == TX_CLASS_2D as c_int {
                    rc = *scan.offset(eob as isize) as c_uint;
                    x = rc >> shift;
                    y = rc & mask;
                } else if TX_CLASS_2D as c_int == TX_CLASS_H as c_int {
                    x = eob as c_uint & mask;
                    y = (eob >> shift) as c_uint;
                    rc = eob as c_uint;
                } else {
                    x = eob as c_uint & mask;
                    y = (eob >> shift) as c_uint;
                    rc = x << shift2 | y;
                }
                if dbg != 0 {
                    printf(
                        b"Post-lo_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                            as *const c_char,
                        (*t_dim).ctx as c_int,
                        chroma,
                        ctx,
                        eob,
                        rc,
                        tok,
                        (*ts).msac.rng,
                    );
                }
                if eob_tok == 2 {
                    ctx = (if if TX_CLASS_2D as c_int == TX_CLASS_2D as c_int {
                        (x | y > 1 as c_uint) as c_int
                    } else {
                        (y != 0 as c_int as c_uint) as c_int
                    } != 0
                    {
                        14 as c_int
                    } else {
                        7 as c_int
                    }) as c_uint;
                    tok = rav1d_msac_decode_hi_tok(
                        &mut (*ts).msac,
                        &mut *hi_cdf.offset(ctx as isize),
                    ) as c_int;
                    level_tok = tok + ((3 as c_int) << 6);
                    if dbg != 0 {
                        printf(
                            b"Post-hi_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                as *const c_char,
                            cmp::min((*t_dim).ctx as c_int, 3 as c_int),
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
                levels[(x as isize * stride + y as isize) as usize] = level_tok as u8;
                let mut i = eob - 1;
                while i > 0 {
                    let rc_i: c_uint;
                    if TX_CLASS_2D as c_int == TX_CLASS_2D as c_int {
                        rc_i = *scan.offset(i as isize) as c_uint;
                        x = rc_i >> shift;
                        y = rc_i & mask;
                    } else if TX_CLASS_2D as c_int == TX_CLASS_H as c_int {
                        x = i as c_uint & mask;
                        y = (i >> shift) as c_uint;
                        rc_i = i as c_uint;
                    } else {
                        x = i as c_uint & mask;
                        y = (i >> shift) as c_uint;
                        rc_i = x << shift2 | y;
                    }
                    if !(x < 32 as c_uint && y < 32 as c_uint) {
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
                    ) as c_uint;
                    if TX_CLASS_2D as c_int == TX_CLASS_2D as c_int {
                        y |= x;
                    }
                    tok = rav1d_msac_decode_symbol_adapt4(
                        &mut (*ts).msac,
                        &mut *lo_cdf.offset(ctx as isize),
                        3 as c_int as usize,
                    ) as c_int;
                    if dbg != 0 {
                        printf(
                            b"Post-lo_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                as *const c_char,
                            (*t_dim).ctx as c_int,
                            chroma,
                            ctx,
                            i,
                            rc_i,
                            tok,
                            (*ts).msac.rng,
                        );
                    }
                    if tok == 3 {
                        mag &= 63 as c_int as c_uint;
                        ctx = ((if y
                            > (TX_CLASS_2D as c_int == TX_CLASS_2D as c_int) as c_int as c_uint
                        {
                            14 as c_int
                        } else {
                            7 as c_int
                        }) as c_uint)
                            .wrapping_add(if mag > 12 as c_uint {
                                6 as c_int as c_uint
                            } else {
                                mag.wrapping_add(1 as c_int as c_uint) >> 1
                            });
                        tok = rav1d_msac_decode_hi_tok(
                            &mut (*ts).msac,
                            &mut *hi_cdf.offset(ctx as isize),
                        ) as c_int;
                        if dbg != 0 {
                            printf(
                                b"Post-hi_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                    as *const c_char,
                                cmp::min((*t_dim).ctx as c_int, 3 as c_int),
                                chroma,
                                ctx,
                                i,
                                rc_i,
                                tok,
                                (*ts).msac.rng,
                            );
                        }
                        level[0] = (tok + ((3 as c_int) << 6)) as u8;
                        *cf.offset(rc_i as isize) = ((tok << 11) as c_uint | rc).as_::<BD::Coef>();
                        rc = rc_i;
                    } else {
                        tok *= 0x17ff41 as c_int;
                        level[0] = tok as u8;
                        tok = ((tok >> 9) as c_uint & rc.wrapping_add(!(0x7ff as c_uint))) as c_int;
                        if tok != 0 {
                            rc = rc_i;
                        }
                        *cf.offset(rc_i as isize) = tok.as_::<BD::Coef>();
                    }
                    i -= 1;
                }
                ctx = if TX_CLASS_2D as c_int == TX_CLASS_2D as c_int {
                    0 as c_int as c_uint
                } else {
                    get_lo_ctx(
                        levels,
                        TX_CLASS_2D,
                        &mut mag,
                        lo_ctx_offsets,
                        0,
                        0,
                        stride as usize,
                    ) as c_uint
                };
                dc_tok = rav1d_msac_decode_symbol_adapt4(
                    &mut (*ts).msac,
                    &mut *lo_cdf.offset(ctx as isize),
                    3 as c_int as usize,
                );
                if dbg != 0 {
                    printf(
                        b"Post-dc_lo_tok[%d][%d][%d][%d]: r=%d\n\0" as *const u8 as *const c_char,
                        (*t_dim).ctx as c_int,
                        chroma,
                        ctx,
                        dc_tok,
                        (*ts).msac.rng,
                    );
                }
                if dc_tok == 3 as c_uint {
                    if TX_CLASS_2D as c_int == TX_CLASS_2D as c_int {
                        mag = (levels[(0 * stride + 1) as usize] as c_int
                            + levels[(1 * stride + 0) as usize] as c_int
                            + levels[(1 * stride + 1) as usize] as c_int)
                            as c_uint;
                    }
                    mag &= 63 as c_int as c_uint;
                    ctx = if mag > 12 as c_uint {
                        6 as c_int as c_uint
                    } else {
                        mag.wrapping_add(1 as c_int as c_uint) >> 1
                    };
                    dc_tok = rav1d_msac_decode_hi_tok(
                        &mut (*ts).msac,
                        &mut *hi_cdf.offset(ctx as isize),
                    );
                    if dbg != 0 {
                        printf(
                            b"Post-dc_hi_tok[%d][%d][0][%d]: r=%d\n\0" as *const u8
                                as *const c_char,
                            cmp::min((*t_dim).ctx as c_int, 3 as c_int),
                            chroma,
                            dc_tok,
                            (*ts).msac.rng,
                        );
                    }
                }
            }
            1 => {
                let lo_ctx_offsets_0 = None;
                let stride_0: ptrdiff_t = 16 as c_int as ptrdiff_t;
                let shift_0: c_uint = ((*t_dim).lh as c_int + 2) as c_uint;
                let shift2_0: c_uint = 0 as c_int as c_uint;
                let mask_0: c_uint = (4 * sh - 1) as c_uint;
                memset(
                    levels.as_mut_ptr() as *mut c_void,
                    0 as c_int,
                    (stride_0 * (4 * sh + 2) as isize) as usize,
                );
                let mut x_0: c_uint;
                let mut y_0: c_uint;
                if TX_CLASS_H as c_int == TX_CLASS_2D as c_int {
                    rc = *scan.offset(eob as isize) as c_uint;
                    x_0 = rc >> shift_0;
                    y_0 = rc & mask_0;
                } else if TX_CLASS_H as c_int == TX_CLASS_H as c_int {
                    x_0 = eob as c_uint & mask_0;
                    y_0 = (eob >> shift_0) as c_uint;
                    rc = eob as c_uint;
                } else {
                    x_0 = eob as c_uint & mask_0;
                    y_0 = (eob >> shift_0) as c_uint;
                    rc = x_0 << shift2_0 | y_0;
                }
                if dbg != 0 {
                    printf(
                        b"Post-lo_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                            as *const c_char,
                        (*t_dim).ctx as c_int,
                        chroma,
                        ctx,
                        eob,
                        rc,
                        tok,
                        (*ts).msac.rng,
                    );
                }
                if eob_tok == 2 {
                    ctx = (if if TX_CLASS_H as c_int == TX_CLASS_2D as c_int {
                        (x_0 | y_0 > 1 as c_uint) as c_int
                    } else {
                        (y_0 != 0 as c_int as c_uint) as c_int
                    } != 0
                    {
                        14 as c_int
                    } else {
                        7 as c_int
                    }) as c_uint;
                    tok = rav1d_msac_decode_hi_tok(
                        &mut (*ts).msac,
                        &mut *hi_cdf.offset(ctx as isize),
                    ) as c_int;
                    level_tok = tok + ((3 as c_int) << 6);
                    if dbg != 0 {
                        printf(
                            b"Post-hi_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                as *const c_char,
                            cmp::min((*t_dim).ctx as c_int, 3 as c_int),
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
                levels[(x_0 as isize * stride_0 + y_0 as isize) as usize] = level_tok as u8;
                let mut i_0 = eob - 1;
                while i_0 > 0 {
                    let rc_i_0: c_uint;
                    if TX_CLASS_H as c_int == TX_CLASS_2D as c_int {
                        rc_i_0 = *scan.offset(i_0 as isize) as c_uint;
                        x_0 = rc_i_0 >> shift_0;
                        y_0 = rc_i_0 & mask_0;
                    } else if TX_CLASS_H as c_int == TX_CLASS_H as c_int {
                        x_0 = i_0 as c_uint & mask_0;
                        y_0 = (i_0 >> shift_0) as c_uint;
                        rc_i_0 = i_0 as c_uint;
                    } else {
                        x_0 = i_0 as c_uint & mask_0;
                        y_0 = (i_0 >> shift_0) as c_uint;
                        rc_i_0 = x_0 << shift2_0 | y_0;
                    }
                    if !(x_0 < 32 as c_uint && y_0 < 32 as c_uint) {
                        unreachable!();
                    }
                    let level_0 =
                        &mut levels[(x_0 as isize * stride_0 as isize + y_0 as isize) as usize..];
                    ctx = get_lo_ctx(
                        level_0,
                        TX_CLASS_H,
                        &mut mag,
                        lo_ctx_offsets_0,
                        x_0 as usize,
                        y_0 as usize,
                        stride_0 as usize,
                    ) as c_uint;
                    if TX_CLASS_H as c_int == TX_CLASS_2D as c_int {
                        y_0 |= x_0;
                    }
                    tok = rav1d_msac_decode_symbol_adapt4(
                        &mut (*ts).msac,
                        &mut *lo_cdf.offset(ctx as isize),
                        3 as c_int as usize,
                    ) as c_int;
                    if dbg != 0 {
                        printf(
                            b"Post-lo_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                as *const c_char,
                            (*t_dim).ctx as c_int,
                            chroma,
                            ctx,
                            i_0,
                            rc_i_0,
                            tok,
                            (*ts).msac.rng,
                        );
                    }
                    if tok == 3 {
                        mag &= 63 as c_int as c_uint;
                        ctx = ((if y_0
                            > (TX_CLASS_H as c_int == TX_CLASS_2D as c_int) as c_int as c_uint
                        {
                            14 as c_int
                        } else {
                            7 as c_int
                        }) as c_uint)
                            .wrapping_add(if mag > 12 as c_uint {
                                6 as c_int as c_uint
                            } else {
                                mag.wrapping_add(1 as c_int as c_uint) >> 1
                            });
                        tok = rav1d_msac_decode_hi_tok(
                            &mut (*ts).msac,
                            &mut *hi_cdf.offset(ctx as isize),
                        ) as c_int;
                        if dbg != 0 {
                            printf(
                                b"Post-hi_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                    as *const c_char,
                                cmp::min((*t_dim).ctx as c_int, 3 as c_int),
                                chroma,
                                ctx,
                                i_0,
                                rc_i_0,
                                tok,
                                (*ts).msac.rng,
                            );
                        }
                        level_0[0] = (tok + ((3 as c_int) << 6)) as u8;
                        *cf.offset(rc_i_0 as isize) =
                            ((tok << 11) as c_uint | rc).as_::<BD::Coef>();
                        rc = rc_i_0;
                    } else {
                        tok *= 0x17ff41 as c_int;
                        level_0[0] = tok as u8;
                        tok = ((tok >> 9) as c_uint & rc.wrapping_add(!(0x7ff as c_uint))) as c_int;
                        if tok != 0 {
                            rc = rc_i_0;
                        }
                        *cf.offset(rc_i_0 as isize) = tok.as_::<BD::Coef>();
                    }
                    i_0 -= 1;
                }
                ctx = if TX_CLASS_H as c_int == TX_CLASS_2D as c_int {
                    0 as c_int as c_uint
                } else {
                    get_lo_ctx(
                        levels,
                        TX_CLASS_H,
                        &mut mag,
                        lo_ctx_offsets_0,
                        0,
                        0,
                        stride_0 as usize,
                    ) as c_uint
                };
                dc_tok = rav1d_msac_decode_symbol_adapt4(
                    &mut (*ts).msac,
                    &mut *lo_cdf.offset(ctx as isize),
                    3 as c_int as usize,
                );
                if dbg != 0 {
                    printf(
                        b"Post-dc_lo_tok[%d][%d][%d][%d]: r=%d\n\0" as *const u8 as *const c_char,
                        (*t_dim).ctx as c_int,
                        chroma,
                        ctx,
                        dc_tok,
                        (*ts).msac.rng,
                    );
                }
                if dc_tok == 3 as c_uint {
                    if TX_CLASS_H as c_int == TX_CLASS_2D as c_int {
                        mag = (levels[(0 * stride_0 + 1) as usize] as c_int
                            + levels[(1 * stride_0 + 0) as usize] as c_int
                            + levels[(1 * stride_0 + 1) as usize] as c_int)
                            as c_uint;
                    }
                    mag &= 63 as c_int as c_uint;
                    ctx = if mag > 12 as c_uint {
                        6 as c_int as c_uint
                    } else {
                        mag.wrapping_add(1 as c_int as c_uint) >> 1
                    };
                    dc_tok = rav1d_msac_decode_hi_tok(
                        &mut (*ts).msac,
                        &mut *hi_cdf.offset(ctx as isize),
                    );
                    if dbg != 0 {
                        printf(
                            b"Post-dc_hi_tok[%d][%d][0][%d]: r=%d\n\0" as *const u8
                                as *const c_char,
                            cmp::min((*t_dim).ctx as c_int, 3 as c_int),
                            chroma,
                            dc_tok,
                            (*ts).msac.rng,
                        );
                    }
                }
            }
            2 => {
                let lo_ctx_offsets_1 = None;
                let stride_1: ptrdiff_t = 16 as c_int as ptrdiff_t;
                let shift_1: c_uint = ((*t_dim).lw as c_int + 2) as c_uint;
                let shift2_1: c_uint = ((*t_dim).lh as c_int + 2) as c_uint;
                let mask_1: c_uint = (4 * sw - 1) as c_uint;
                memset(
                    levels.as_mut_ptr() as *mut c_void,
                    0 as c_int,
                    (stride_1 * (4 * sw + 2) as isize) as usize,
                );
                let mut x_1: c_uint;
                let mut y_1: c_uint;
                if TX_CLASS_V as c_int == TX_CLASS_2D as c_int {
                    rc = *scan.offset(eob as isize) as c_uint;
                    x_1 = rc >> shift_1;
                    y_1 = rc & mask_1;
                } else if TX_CLASS_V as c_int == TX_CLASS_H as c_int {
                    x_1 = eob as c_uint & mask_1;
                    y_1 = (eob >> shift_1) as c_uint;
                    rc = eob as c_uint;
                } else {
                    x_1 = eob as c_uint & mask_1;
                    y_1 = (eob >> shift_1) as c_uint;
                    rc = x_1 << shift2_1 | y_1;
                }
                if dbg != 0 {
                    printf(
                        b"Post-lo_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                            as *const c_char,
                        (*t_dim).ctx as c_int,
                        chroma,
                        ctx,
                        eob,
                        rc,
                        tok,
                        (*ts).msac.rng,
                    );
                }
                if eob_tok == 2 {
                    ctx = (if if TX_CLASS_V as c_int == TX_CLASS_2D as c_int {
                        (x_1 | y_1 > 1 as c_uint) as c_int
                    } else {
                        (y_1 != 0 as c_int as c_uint) as c_int
                    } != 0
                    {
                        14 as c_int
                    } else {
                        7 as c_int
                    }) as c_uint;
                    tok = rav1d_msac_decode_hi_tok(
                        &mut (*ts).msac,
                        &mut *hi_cdf.offset(ctx as isize),
                    ) as c_int;
                    level_tok = tok + ((3 as c_int) << 6);
                    if dbg != 0 {
                        printf(
                            b"Post-hi_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                as *const c_char,
                            cmp::min((*t_dim).ctx as c_int, 3 as c_int),
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
                levels[(x_1 as isize * stride_1 + y_1 as isize) as usize] = level_tok as u8;
                let mut i_1 = eob - 1;
                while i_1 > 0 {
                    let rc_i_1: c_uint;
                    if TX_CLASS_V as c_int == TX_CLASS_2D as c_int {
                        rc_i_1 = *scan.offset(i_1 as isize) as c_uint;
                        x_1 = rc_i_1 >> shift_1;
                        y_1 = rc_i_1 & mask_1;
                    } else if TX_CLASS_V as c_int == TX_CLASS_H as c_int {
                        x_1 = i_1 as c_uint & mask_1;
                        y_1 = (i_1 >> shift_1) as c_uint;
                        rc_i_1 = i_1 as c_uint;
                    } else {
                        x_1 = i_1 as c_uint & mask_1;
                        y_1 = (i_1 >> shift_1) as c_uint;
                        rc_i_1 = x_1 << shift2_1 | y_1;
                    }
                    if !(x_1 < 32 as c_uint && y_1 < 32 as c_uint) {
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
                    ) as c_uint;
                    if TX_CLASS_V as c_int == TX_CLASS_2D as c_int {
                        y_1 |= x_1;
                    }
                    tok = rav1d_msac_decode_symbol_adapt4(
                        &mut (*ts).msac,
                        &mut *lo_cdf.offset(ctx as isize),
                        3 as c_int as usize,
                    ) as c_int;
                    if dbg != 0 {
                        printf(
                            b"Post-lo_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                as *const c_char,
                            (*t_dim).ctx as c_int,
                            chroma,
                            ctx,
                            i_1,
                            rc_i_1,
                            tok,
                            (*ts).msac.rng,
                        );
                    }
                    if tok == 3 {
                        mag &= 63 as c_int as c_uint;
                        ctx = ((if y_1
                            > (TX_CLASS_V as c_int == TX_CLASS_2D as c_int) as c_int as c_uint
                        {
                            14 as c_int
                        } else {
                            7 as c_int
                        }) as c_uint)
                            .wrapping_add(if mag > 12 as c_uint {
                                6 as c_int as c_uint
                            } else {
                                mag.wrapping_add(1 as c_int as c_uint) >> 1
                            });
                        tok = rav1d_msac_decode_hi_tok(
                            &mut (*ts).msac,
                            &mut *hi_cdf.offset(ctx as isize),
                        ) as c_int;
                        if dbg != 0 {
                            printf(
                                b"Post-hi_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                    as *const c_char,
                                cmp::min((*t_dim).ctx as c_int, 3 as c_int),
                                chroma,
                                ctx,
                                i_1,
                                rc_i_1,
                                tok,
                                (*ts).msac.rng,
                            );
                        }
                        level_1[0] = (tok + ((3 as c_int) << 6)) as u8;
                        *cf.offset(rc_i_1 as isize) =
                            ((tok << 11) as c_uint | rc).as_::<BD::Coef>();
                        rc = rc_i_1;
                    } else {
                        tok *= 0x17ff41 as c_int;
                        level_1[0] = tok as u8;
                        tok = ((tok >> 9) as c_uint & rc.wrapping_add(!(0x7ff as c_uint))) as c_int;
                        if tok != 0 {
                            rc = rc_i_1;
                        }
                        *cf.offset(rc_i_1 as isize) = tok.as_::<BD::Coef>();
                    }
                    i_1 -= 1;
                }
                ctx = if TX_CLASS_V as c_int == TX_CLASS_2D as c_int {
                    0 as c_int as c_uint
                } else {
                    get_lo_ctx(
                        levels,
                        TX_CLASS_V,
                        &mut mag,
                        lo_ctx_offsets_1,
                        0,
                        0,
                        stride_1 as usize,
                    ) as c_uint
                };
                dc_tok = rav1d_msac_decode_symbol_adapt4(
                    &mut (*ts).msac,
                    &mut *lo_cdf.offset(ctx as isize),
                    3 as c_int as usize,
                );
                if dbg != 0 {
                    printf(
                        b"Post-dc_lo_tok[%d][%d][%d][%d]: r=%d\n\0" as *const u8 as *const c_char,
                        (*t_dim).ctx as c_int,
                        chroma,
                        ctx,
                        dc_tok,
                        (*ts).msac.rng,
                    );
                }
                if dc_tok == 3 as c_uint {
                    if TX_CLASS_V as c_int == TX_CLASS_2D as c_int {
                        mag = (levels[(0 * stride_1 + 1) as usize] as c_int
                            + levels[(1 * stride_1 + 0) as usize] as c_int
                            + levels[(1 * stride_1 + 1) as usize] as c_int)
                            as c_uint;
                    }
                    mag &= 63 as c_int as c_uint;
                    ctx = if mag > 12 as c_uint {
                        6 as c_int as c_uint
                    } else {
                        mag.wrapping_add(1 as c_int as c_uint) >> 1
                    };
                    dc_tok = rav1d_msac_decode_hi_tok(
                        &mut (*ts).msac,
                        &mut *hi_cdf.offset(ctx as isize),
                    );
                    if dbg != 0 {
                        printf(
                            b"Post-dc_hi_tok[%d][%d][0][%d]: r=%d\n\0" as *const u8
                                as *const c_char,
                            cmp::min((*t_dim).ctx as c_int, 3 as c_int),
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
        let tok_br = rav1d_msac_decode_symbol_adapt4(
            &mut (*ts).msac,
            &mut *eob_cdf.offset(0),
            2 as c_int as usize,
        ) as c_int;
        dc_tok = (1 + tok_br) as c_uint;
        if dbg != 0 {
            printf(
                b"Post-dc_lo_tok[%d][%d][%d][%d]: r=%d\n\0" as *const u8 as *const c_char,
                (*t_dim).ctx as c_int,
                chroma,
                0 as c_int,
                dc_tok,
                (*ts).msac.rng,
            );
        }
        if tok_br == 2 {
            dc_tok = rav1d_msac_decode_hi_tok(&mut (*ts).msac, &mut *hi_cdf.offset(0));
            if dbg != 0 {
                printf(
                    b"Post-dc_hi_tok[%d][%d][0][%d]: r=%d\n\0" as *const u8 as *const c_char,
                    cmp::min((*t_dim).ctx as c_int, 3 as c_int),
                    chroma,
                    dc_tok,
                    (*ts).msac.rng,
                );
            }
        }
        rc = 0 as c_int as c_uint;
    }
    let dq_tbl: *const u16 = ((*((*ts).dq).offset((*b).seg_id as isize))[plane as usize]).as_ptr();
    let qm_tbl: *const u8 = if (*txtp as c_uint) < IDTX as c_int as c_uint {
        (*f).qm[tx as usize][plane as usize]
    } else {
        0 as *const u8
    };
    let dq_shift = cmp::max(0 as c_int, (*t_dim).ctx as c_int - 2);
    let cf_max =
        !(!(127 as c_uint) << (if 16 == 8 { 8 as c_int } else { (*f).cur.p.bpc })) as c_int;
    let mut cul_level: c_uint;
    let dc_sign_level: c_uint;
    if dc_tok == 0 {
        cul_level = 0 as c_int as c_uint;
        dc_sign_level = ((1 as c_int) << 6) as c_uint;
        if !qm_tbl.is_null() {
            current_block = 1669574575799829731;
        } else {
            current_block = 2404388531445638768;
        }
    } else {
        dc_sign_ctx = get_dc_sign_ctx(tx, a, l) as c_int;
        let dc_sign_cdf = &mut (*ts).cdf.coef.dc_sign[chroma as usize][dc_sign_ctx as usize];
        dc_sign = rav1d_msac_decode_bool_adapt(&mut (*ts).msac, dc_sign_cdf) as c_int;
        if dbg != 0 {
            printf(
                b"Post-dc_sign[%d][%d][%d]: r=%d\n\0" as *const u8 as *const c_char,
                chroma,
                dc_sign_ctx,
                dc_sign,
                (*ts).msac.rng,
            );
        }
        dc_dq = *dq_tbl.offset(0) as c_int;
        dc_sign_level = (dc_sign - 1 & (2 as c_int) << 6) as c_uint;
        if !qm_tbl.is_null() {
            dc_dq = dc_dq * *qm_tbl.offset(0) as c_int + 16 >> 5;
            if dc_tok == 15 as c_uint {
                dc_tok = (read_golomb(&mut (*ts).msac)).wrapping_add(15 as c_int as c_uint);
                if dbg != 0 {
                    printf(
                        b"Post-dc_residual[%d->%d]: r=%d\n\0" as *const u8 as *const c_char,
                        dc_tok.wrapping_sub(15 as c_int as c_uint),
                        dc_tok,
                        (*ts).msac.rng,
                    );
                }
                dc_tok &= 0xfffff as c_int as c_uint;
                dc_dq =
                    ((dc_dq as c_uint).wrapping_mul(dc_tok) & 0xffffff as c_int as c_uint) as c_int;
            } else {
                dc_dq = (dc_dq as c_uint).wrapping_mul(dc_tok) as c_int as c_int;
                if !(dc_dq <= 0xffffff as c_int) {
                    unreachable!();
                }
            }
            cul_level = dc_tok;
            dc_dq >>= dq_shift;
            dc_dq = cmp::min(dc_dq as c_uint, (cf_max + dc_sign) as c_uint) as c_int;
            *cf.offset(0) = (if dc_sign != 0 { -dc_dq } else { dc_dq }).as_::<BD::Coef>();
            if rc != 0 {
                current_block = 1669574575799829731;
            } else {
                current_block = 15494703142406051947;
            }
        } else {
            if dc_tok == 15 as c_uint {
                dc_tok = (read_golomb(&mut (*ts).msac)).wrapping_add(15 as c_int as c_uint);
                if dbg != 0 {
                    printf(
                        b"Post-dc_residual[%d->%d]: r=%d\n\0" as *const u8 as *const c_char,
                        dc_tok.wrapping_sub(15 as c_int as c_uint),
                        dc_tok,
                        (*ts).msac.rng,
                    );
                }
                dc_tok &= 0xfffff as c_int as c_uint;
                dc_dq = (((dc_dq as c_uint).wrapping_mul(dc_tok) & 0xffffff as c_int as c_uint)
                    >> dq_shift) as c_int;
                dc_dq = cmp::min(dc_dq as c_uint, (cf_max + dc_sign) as c_uint) as c_int;
            } else {
                dc_dq = ((dc_dq as c_uint).wrapping_mul(dc_tok) >> dq_shift) as c_int;
                if !(dc_dq <= cf_max) {
                    unreachable!();
                }
            }
            cul_level = dc_tok;
            *cf.offset(0) = (if dc_sign != 0 { -dc_dq } else { dc_dq }).as_::<BD::Coef>();
            if rc != 0 {
                current_block = 2404388531445638768;
            } else {
                current_block = 15494703142406051947;
            }
        }
    }
    match current_block {
        1669574575799829731 => {
            let ac_dq: c_uint = *dq_tbl.offset(1) as c_uint;
            loop {
                let sign = rav1d_msac_decode_bool_equi(&mut (*ts).msac) as c_int;
                if dbg != 0 {
                    printf(
                        b"Post-sign[%d=%d]: r=%d\n\0" as *const u8 as *const c_char,
                        rc,
                        sign,
                        (*ts).msac.rng,
                    );
                }
                let rc_tok: c_uint = (*cf.offset(rc as isize)).as_::<c_uint>();
                let mut tok_0: c_uint;
                let mut dq: c_uint = ac_dq
                    .wrapping_mul(*qm_tbl.offset(rc as isize) as c_uint)
                    .wrapping_add(16 as c_int as c_uint)
                    >> 5;
                let dq_sat;
                if rc_tok >= ((15 as c_int) << 11) as c_uint {
                    tok_0 = (read_golomb(&mut (*ts).msac)).wrapping_add(15 as c_int as c_uint);
                    if dbg != 0 {
                        printf(
                            b"Post-residual[%d=%d->%d]: r=%d\n\0" as *const u8 as *const c_char,
                            rc,
                            tok_0.wrapping_sub(15 as c_int as c_uint),
                            tok_0,
                            (*ts).msac.rng,
                        );
                    }
                    tok_0 &= 0xfffff as c_int as c_uint;
                    dq = dq.wrapping_mul(tok_0) & 0xffffff as c_int as c_uint;
                } else {
                    tok_0 = rc_tok >> 11;
                    dq = dq.wrapping_mul(tok_0);
                    if !(dq <= 0xffffff as c_int as c_uint) {
                        unreachable!();
                    }
                }
                cul_level = cul_level.wrapping_add(tok_0);
                dq >>= dq_shift;
                dq_sat = cmp::min(dq, (cf_max + sign) as c_uint) as c_int;
                *cf.offset(rc as isize) =
                    (if sign != 0 { -dq_sat } else { dq_sat }).as_::<BD::Coef>();
                rc = rc_tok & 0x3ff as c_int as c_uint;
                if !(rc != 0) {
                    break;
                }
            }
        }
        2404388531445638768 => {
            let ac_dq_0: c_uint = *dq_tbl.offset(1) as c_uint;
            loop {
                let sign_0 = rav1d_msac_decode_bool_equi(&mut (*ts).msac) as c_int;
                if dbg != 0 {
                    printf(
                        b"Post-sign[%d=%d]: r=%d\n\0" as *const u8 as *const c_char,
                        rc,
                        sign_0,
                        (*ts).msac.rng,
                    );
                }
                let rc_tok_0: c_uint = (*cf.offset(rc as isize)).as_::<c_uint>();
                let mut tok_1: c_uint;
                let mut dq_0;
                if rc_tok_0 >= ((15 as c_int) << 11) as c_uint {
                    tok_1 = (read_golomb(&mut (*ts).msac)).wrapping_add(15 as c_int as c_uint);
                    if dbg != 0 {
                        printf(
                            b"Post-residual[%d=%d->%d]: r=%d\n\0" as *const u8 as *const c_char,
                            rc,
                            tok_1.wrapping_sub(15 as c_int as c_uint),
                            tok_1,
                            (*ts).msac.rng,
                        );
                    }
                    tok_1 &= 0xfffff as c_int as c_uint;
                    dq_0 = ((ac_dq_0.wrapping_mul(tok_1) & 0xffffff as c_int as c_uint) >> dq_shift)
                        as c_int;
                    dq_0 = cmp::min(dq_0 as c_uint, (cf_max + sign_0) as c_uint) as c_int;
                } else {
                    tok_1 = rc_tok_0 >> 11;
                    dq_0 = (ac_dq_0.wrapping_mul(tok_1) >> dq_shift) as c_int;
                    if !(dq_0 <= cf_max) {
                        unreachable!();
                    }
                }
                cul_level = cul_level.wrapping_add(tok_1);
                *cf.offset(rc as isize) =
                    (if sign_0 != 0 { -dq_0 } else { dq_0 }).as_::<BD::Coef>();
                rc = rc_tok_0 & 0x3ff as c_int as c_uint;
                if !(rc != 0) {
                    break;
                }
            }
        }
        _ => {}
    }
    *res_ctx = (cmp::min(cul_level, 63 as c_int as c_uint) | dc_sign_level) as u8;
    return eob;
}

// TODO(kkysen) pub(crate) temporarily until recon is fully deduplicated
pub(crate) unsafe fn read_coef_tree<BD: BitDepth>(
    t: *mut Rav1dTaskContext,
    bs: BlockSize,
    b: *const Av1Block,
    ytx: RectTxfmSize,
    depth: c_int,
    tx_split: *const u16,
    x_off: c_int,
    y_off: c_int,
    mut dst: *mut BD::Pixel,
) {
    let f: *const Rav1dFrameContext = (*t).f;
    let ts: *mut Rav1dTileState = (*t).ts;
    let dsp: *const Rav1dDSPContext = (*f).dsp;
    let t_dim: *const TxfmInfo =
        &*dav1d_txfm_dimensions.as_ptr().offset(ytx as isize) as *const TxfmInfo;
    let txw = (*t_dim).w as c_int;
    let txh = (*t_dim).h as c_int;
    if depth < 2
        && *tx_split.offset(depth as isize) as c_int != 0
        && *tx_split.offset(depth as isize) as c_int & (1 as c_int) << y_off * 4 + x_off != 0
    {
        let sub: RectTxfmSize = (*t_dim).sub as RectTxfmSize;
        let sub_t_dim: *const TxfmInfo =
            &*dav1d_txfm_dimensions.as_ptr().offset(sub as isize) as *const TxfmInfo;
        let txsw = (*sub_t_dim).w as c_int;
        let txsh = (*sub_t_dim).h as c_int;
        read_coef_tree::<BD>(
            t,
            bs,
            b,
            sub,
            depth + 1,
            tx_split,
            x_off * 2 + 0,
            y_off * 2 + 0,
            dst,
        );
        (*t).bx += txsw;
        if txw >= txh && (*t).bx < (*f).bw {
            read_coef_tree::<BD>(
                t,
                bs,
                b,
                sub,
                depth + 1,
                tx_split,
                x_off * 2 + 1,
                y_off * 2 + 0,
                if !dst.is_null() {
                    &mut *dst.offset((4 * txsw) as isize)
                } else {
                    0 as *mut BD::Pixel
                },
            );
        }
        (*t).bx -= txsw;
        (*t).by += txsh;
        if txh >= txw && (*t).by < (*f).bh {
            if !dst.is_null() {
                dst = dst.offset(
                    (4 * txsh) as isize * BD::pxstride((*f).cur.stride[0] as usize) as isize,
                );
            }
            read_coef_tree::<BD>(
                t,
                bs,
                b,
                sub,
                depth + 1,
                tx_split,
                x_off * 2 + 0,
                y_off * 2 + 1,
                dst,
            );
            (*t).bx += txsw;
            if txw >= txh && (*t).bx < (*f).bw {
                read_coef_tree::<BD>(
                    t,
                    bs,
                    b,
                    sub,
                    depth + 1,
                    tx_split,
                    x_off * 2 + 1,
                    y_off * 2 + 1,
                    if !dst.is_null() {
                        &mut *dst.offset((4 * txsw) as isize)
                    } else {
                        0 as *mut BD::Pixel
                    },
                );
            }
            (*t).bx -= txsw;
        }
        (*t).by -= txsh;
    } else {
        let bx4 = (*t).bx & 31;
        let by4 = (*t).by & 31;
        let mut txtp: TxfmType = DCT_DCT;
        let mut cf_ctx: u8 = 0;
        let eob;
        let cf: *mut BD::Coef;
        let mut cbi: *mut CodedBlockInfo = 0 as *mut CodedBlockInfo;
        if (*t).frame_thread.pass != 0 {
            let p = (*t).frame_thread.pass & 1;
            if ((*ts).frame_thread[p as usize].cf).is_null() {
                unreachable!();
            }
            cf = (*ts).frame_thread[p as usize].cf as *mut BD::Coef;
            (*ts).frame_thread[p as usize].cf = ((*ts).frame_thread[p as usize].cf as *mut BD::Coef)
                .offset(
                    (cmp::min((*t_dim).w as c_int, 8 as c_int)
                        * cmp::min((*t_dim).h as c_int, 8 as c_int)
                        * 16) as isize,
                ) as *mut DynCoef;
            cbi = &mut *((*f).frame_thread.cbi)
                .offset(((*t).by as isize * (*f).b4_stride + (*t).bx as isize) as isize)
                as *mut CodedBlockInfo;
        } else {
            cf = match BD::BPC {
                BPC::BPC8 => (*t).c2rust_unnamed.cf_8bpc.as_mut_ptr().cast::<BD::Coef>(),
                BPC::BPC16 => (*t).c2rust_unnamed.cf_16bpc.as_mut_ptr().cast::<BD::Coef>(),
            };
        }
        if (*t).frame_thread.pass != 2 as c_int {
            eob = decode_coefs::<BD>(
                t,
                &mut (*(*t).a).lcoef.0[bx4 as usize..],
                &mut (*t).l.lcoef.0[by4 as usize..],
                ytx,
                bs,
                b,
                0 as c_int,
                0 as c_int,
                cf,
                &mut txtp,
                &mut cf_ctx,
            );
            if DEBUG_BLOCK_INFO(&*f, &*t) {
                printf(
                    b"Post-y-cf-blk[tx=%d,txtp=%d,eob=%d]: r=%d\n\0" as *const u8 as *const c_char,
                    ytx as c_uint,
                    txtp as c_uint,
                    eob,
                    (*ts).msac.rng,
                );
            }
            CaseSet::<16, true>::many(
                [&mut (*t).l, &mut *(*t).a],
                [
                    cmp::min(txh, (*f).bh - (*t).by) as usize,
                    cmp::min(txw, (*f).bw - (*t).bx) as usize,
                ],
                [by4 as usize, bx4 as usize],
                |case, dir| {
                    case.set(&mut dir.lcoef.0, cf_ctx);
                },
            );
            let txtp_map = &mut (*t).txtp_map[(by4 * 32 + bx4) as usize..];
            CaseSet::<16, false>::one((), txw as usize, 0, |case, ()| {
                for txtp_map in txtp_map.chunks_mut(32).take(txh as usize) {
                    case.set(txtp_map, txtp);
                }
            });
            if (*t).frame_thread.pass == 1 {
                (*cbi).eob[0] = eob as i16;
                (*cbi).txtp[0] = txtp as u8;
            }
        } else {
            eob = (*cbi).eob[0] as c_int;
            txtp = (*cbi).txtp[0] as TxfmType;
        }
        if (*t).frame_thread.pass & 1 == 0 {
            if dst.is_null() {
                unreachable!();
            }
            if eob >= 0 {
                if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                    coef_dump(
                        cf,
                        cmp::min((*t_dim).h as usize, 8) * 4,
                        cmp::min((*t_dim).w as usize, 8) * 4,
                        3,
                        "dq",
                    );
                }
                ((*dsp).itx.itxfm_add[ytx as usize][txtp as usize])
                    .expect("non-null function pointer")(
                    dst.cast(),
                    (*f).cur.stride[0],
                    cf.cast(),
                    eob,
                    (*f).bitdepth_max,
                );
                if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                    hex_dump::<BD>(
                        dst,
                        (*f).cur.stride[0] as usize,
                        (*t_dim).w as usize * 4,
                        (*t_dim).h as usize * 4,
                        "recon",
                    );
                }
            }
        }
    };
}

pub(crate) unsafe extern "C" fn rav1d_read_coef_blocks<BD: BitDepth>(
    t: *mut Rav1dTaskContext,
    bs: BlockSize,
    b: *const Av1Block,
) {
    let f: *const Rav1dFrameContext = (*t).f;
    let ss_ver =
        ((*f).cur.p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint) as c_int;
    let ss_hor =
        ((*f).cur.p.layout as c_uint != RAV1D_PIXEL_LAYOUT_I444 as c_int as c_uint) as c_int;
    let bx4 = (*t).bx & 31;
    let by4 = (*t).by & 31;
    let cbx4 = bx4 >> ss_hor;
    let cby4 = by4 >> ss_ver;
    let b_dim: *const u8 = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bw4 = *b_dim.offset(0) as c_int;
    let bh4 = *b_dim.offset(1) as c_int;
    let cbw4 = bw4 + ss_hor >> ss_hor;
    let cbh4 = bh4 + ss_ver >> ss_ver;
    let has_chroma = ((*f).cur.p.layout as c_uint != RAV1D_PIXEL_LAYOUT_I400 as c_int as c_uint
        && (bw4 > ss_hor || (*t).bx & 1 != 0)
        && (bh4 > ss_ver || (*t).by & 1 != 0)) as c_int;
    if (*b).skip != 0 {
        CaseSet::<32, false>::many(
            [&mut (*t).l, &mut *(*t).a],
            [bh4 as usize, bw4 as usize],
            [by4 as usize, bx4 as usize],
            |case, dir| {
                case.set(&mut dir.lcoef.0, 0x40);
            },
        );
        if has_chroma != 0 {
            CaseSet::<32, false>::many(
                [&mut (*t).l, &mut *(*t).a],
                [cbh4 as usize, cbw4 as usize],
                [cby4 as usize, cbx4 as usize],
                |case, dir| {
                    case.set(&mut dir.ccoef.0[0], 0x40);
                    case.set(&mut dir.ccoef.0[1], 0x40);
                },
            );
        }
        return;
    }
    let ts: *mut Rav1dTileState = (*t).ts;
    let w4 = cmp::min(bw4, (*f).bw - (*t).bx);
    let h4 = cmp::min(bh4, (*f).bh - (*t).by);
    let cw4 = w4 + ss_hor >> ss_hor;
    let ch4 = h4 + ss_ver >> ss_ver;
    if !((*t).frame_thread.pass == 1) {
        unreachable!();
    }
    if (*b).skip != 0 {
        unreachable!();
    }
    let uv_t_dim: *const TxfmInfo =
        &*dav1d_txfm_dimensions.as_ptr().offset((*b).uvtx as isize) as *const TxfmInfo;
    let t_dim: *const TxfmInfo = &*dav1d_txfm_dimensions.as_ptr().offset(
        (if (*b).intra as c_int != 0 {
            (*b).c2rust_unnamed.c2rust_unnamed.tx as c_int
        } else {
            (*b).c2rust_unnamed.c2rust_unnamed_0.max_ytx as c_int
        }) as isize,
    ) as *const TxfmInfo;
    let tx_split: [u16; 2] = [
        (*b).c2rust_unnamed.c2rust_unnamed_0.tx_split0 as u16,
        (*b).c2rust_unnamed.c2rust_unnamed_0.tx_split1,
    ];
    let mut init_y = 0;
    while init_y < h4 {
        let sub_h4 = cmp::min(h4, 16 + init_y);
        let mut init_x = 0;
        while init_x < w4 {
            let sub_w4 = cmp::min(w4, init_x + 16);
            let mut y_off = (init_y != 0) as c_int;
            let mut y;
            let mut x;
            y = init_y;
            (*t).by += init_y;
            while y < sub_h4 {
                let cbi: *mut CodedBlockInfo = &mut *((*f).frame_thread.cbi)
                    .offset(((*t).by as isize * (*f).b4_stride) as isize)
                    as *mut CodedBlockInfo;
                let mut x_off = (init_x != 0) as c_int;
                x = init_x;
                (*t).bx += init_x;
                while x < sub_w4 {
                    if (*b).intra == 0 {
                        read_coef_tree::<BD>(
                            t,
                            bs,
                            b,
                            (*b).c2rust_unnamed.c2rust_unnamed_0.max_ytx as RectTxfmSize,
                            0 as c_int,
                            tx_split.as_ptr(),
                            x_off,
                            y_off,
                            0 as *mut BD::Pixel,
                        );
                    } else {
                        let mut cf_ctx: u8 = 0x40 as c_int as u8;
                        let mut txtp: TxfmType = DCT_DCT;
                        let ref mut fresh4 = (*cbi.offset((*t).bx as isize)).eob[0];
                        *fresh4 = decode_coefs::<BD>(
                            t,
                            &mut (*(*t).a).lcoef.0[(bx4 + x) as usize..],
                            &mut (*t).l.lcoef.0[(by4 + y) as usize..],
                            (*b).c2rust_unnamed.c2rust_unnamed.tx as RectTxfmSize,
                            bs,
                            b,
                            1 as c_int,
                            0 as c_int,
                            (*ts).frame_thread[1].cf as *mut BD::Coef,
                            &mut txtp,
                            &mut cf_ctx,
                        ) as i16;
                        let eob = *fresh4 as c_int;
                        if DEBUG_BLOCK_INFO(&*f, &*t) {
                            printf(
                                b"Post-y-cf-blk[tx=%d,txtp=%d,eob=%d]: r=%d\n\0" as *const u8
                                    as *const c_char,
                                (*b).c2rust_unnamed.c2rust_unnamed.tx as c_int,
                                txtp as c_uint,
                                eob,
                                (*ts).msac.rng,
                            );
                        }
                        (*cbi.offset((*t).bx as isize)).txtp[0] = txtp as u8;
                        (*ts).frame_thread[1].cf = ((*ts).frame_thread[1].cf as *mut BD::Coef)
                            .offset(
                                (cmp::min((*t_dim).w as c_int, 8 as c_int)
                                    * cmp::min((*t_dim).h as c_int, 8 as c_int)
                                    * 16) as isize,
                            ) as *mut DynCoef;
                        CaseSet::<16, true>::many(
                            [&mut (*t).l, &mut *(*t).a],
                            [
                                cmp::min((*t_dim).h as i32, (*f).bh - (*t).by) as usize,
                                cmp::min((*t_dim).w as i32, (*f).bw - (*t).bx) as usize,
                            ],
                            [(by4 + y) as usize, (bx4 + x) as usize],
                            |case, dir| {
                                case.set(&mut dir.lcoef.0, cf_ctx);
                            },
                        );
                    }
                    x += (*t_dim).w as c_int;
                    (*t).bx += (*t_dim).w as c_int;
                    x_off += 1;
                }
                (*t).bx -= x;
                y += (*t_dim).h as c_int;
                (*t).by += (*t_dim).h as c_int;
                y_off += 1;
            }
            (*t).by -= y;
            if !(has_chroma == 0) {
                let sub_ch4 = cmp::min(ch4, init_y + 16 >> ss_ver);
                let sub_cw4 = cmp::min(cw4, init_x + 16 >> ss_hor);
                let mut pl = 0;
                while pl < 2 {
                    y = init_y >> ss_ver;
                    (*t).by += init_y;
                    while y < sub_ch4 {
                        let cbi_0: *mut CodedBlockInfo = &mut *((*f).frame_thread.cbi)
                            .offset(((*t).by as isize * (*f).b4_stride) as isize)
                            as *mut CodedBlockInfo;
                        x = init_x >> ss_hor;
                        (*t).bx += init_x;
                        while x < sub_cw4 {
                            let mut cf_ctx_0: u8 = 0x40 as c_int as u8;
                            let mut txtp_0: TxfmType = DCT_DCT;
                            if (*b).intra == 0 {
                                txtp_0 = (*t).txtp_map
                                    [((by4 + (y << ss_ver)) * 32 + bx4 + (x << ss_hor)) as usize]
                                    as TxfmType;
                            }
                            let ref mut fresh5 =
                                (*cbi_0.offset((*t).bx as isize)).eob[(1 + pl) as usize];
                            *fresh5 = decode_coefs::<BD>(
                                t,
                                &mut (*(*t).a).ccoef.0[pl as usize][(cbx4 + x) as usize..],
                                &mut (*t).l.ccoef.0[pl as usize][(cby4 + y) as usize..],
                                (*b).uvtx as RectTxfmSize,
                                bs,
                                b,
                                (*b).intra as c_int,
                                1 + pl,
                                (*ts).frame_thread[1].cf as *mut BD::Coef,
                                &mut txtp_0,
                                &mut cf_ctx_0,
                            ) as i16;
                            let eob_0 = *fresh5 as c_int;
                            if DEBUG_BLOCK_INFO(&*f, &*t) {
                                printf(
                                    b"Post-uv-cf-blk[pl=%d,tx=%d,txtp=%d,eob=%d]: r=%d\n\0"
                                        as *const u8
                                        as *const c_char,
                                    pl,
                                    (*b).uvtx as c_int,
                                    txtp_0 as c_uint,
                                    eob_0,
                                    (*ts).msac.rng,
                                );
                            }
                            (*cbi_0.offset((*t).bx as isize)).txtp[(1 + pl) as usize] =
                                txtp_0 as u8;
                            (*ts).frame_thread[1].cf =
                                ((*ts).frame_thread[1].cf as *mut BD::Coef).offset(
                                    ((*uv_t_dim).w as c_int * (*uv_t_dim).h as c_int * 16) as isize,
                                ) as *mut DynCoef;
                            CaseSet::<16, true>::many(
                                [&mut (*t).l, &mut *(*t).a],
                                [
                                    cmp::min(
                                        (*uv_t_dim).h as i32,
                                        (*f).bh - (*t).by + ss_ver >> ss_ver,
                                    ) as usize,
                                    cmp::min(
                                        (*uv_t_dim).w as i32,
                                        (*f).bw - (*t).bx + ss_hor >> ss_hor,
                                    ) as usize,
                                ],
                                [(cby4 + y) as usize, (cbx4 + x) as usize],
                                |case, dir| {
                                    case.set(&mut dir.ccoef.0[pl as usize], cf_ctx_0);
                                },
                            );
                            x += (*uv_t_dim).w as c_int;
                            (*t).bx += ((*uv_t_dim).w as c_int) << ss_hor;
                        }
                        (*t).bx -= x << ss_hor;
                        y += (*uv_t_dim).h as c_int;
                        (*t).by += ((*uv_t_dim).h as c_int) << ss_ver;
                    }
                    (*t).by -= y << ss_ver;
                    pl += 1;
                }
            }
            init_x += 16 as c_int;
        }
        init_y += 16 as c_int;
    }
}

// TODO(kkysen) pub(crate) temporarily until recon is fully deduplicated
pub(crate) unsafe fn mc<BD: BitDepth>(
    t: *mut Rav1dTaskContext,
    dst8: *mut BD::Pixel,
    dst16: *mut i16,
    dst_stride: ptrdiff_t,
    bw4: c_int,
    bh4: c_int,
    bx: c_int,
    by: c_int,
    pl: c_int,
    mv: mv,
    refp: *const Rav1dThreadPicture,
    refidx: c_int,
    filter_2d: Filter2d,
) -> c_int {
    if (dst8 != 0 as *mut c_void as *mut BD::Pixel) as c_int
        ^ (dst16 != 0 as *mut c_void as *mut i16) as c_int
        == 0
    {
        unreachable!();
    }
    let f: *const Rav1dFrameContext = (*t).f;
    let ss_ver = (pl != 0
        && (*f).cur.p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint)
        as c_int;
    let ss_hor = (pl != 0
        && (*f).cur.p.layout as c_uint != RAV1D_PIXEL_LAYOUT_I444 as c_int as c_uint)
        as c_int;
    let h_mul = 4 >> ss_hor;
    let v_mul = 4 >> ss_ver;
    let mvx = mv.x as c_int;
    let mvy = mv.y as c_int;
    let mx = mvx & 15 >> (ss_hor == 0) as c_int;
    let my = mvy & 15 >> (ss_ver == 0) as c_int;
    let mut ref_stride: ptrdiff_t = (*refp).p.stride[(pl != 0) as c_int as usize];
    let r#ref: *const BD::Pixel;
    if (*refp).p.p.w == (*f).cur.p.w && (*refp).p.p.h == (*f).cur.p.h {
        let dx = bx * h_mul + (mvx >> 3 + ss_hor);
        let dy = by * v_mul + (mvy >> 3 + ss_ver);
        let w;
        let h;
        if (*refp).p.data[0] != (*f).cur.data[0] {
            w = (*f).cur.p.w + ss_hor >> ss_hor;
            h = (*f).cur.p.h + ss_ver >> ss_ver;
        } else {
            w = (*f).bw * 4 >> ss_hor;
            h = (*f).bh * 4 >> ss_ver;
        }
        if dx < (mx != 0) as c_int * 3
            || dy < (my != 0) as c_int * 3
            || dx + bw4 * h_mul + (mx != 0) as c_int * 4 > w
            || dy + bh4 * v_mul + (my != 0) as c_int * 4 > h
        {
            let emu_edge_buf: *mut BD::Pixel = match BD::BPC {
                BPC::BPC8 => ((*t).scratch.c2rust_unnamed.c2rust_unnamed_0.emu_edge_8bpc)
                    .as_mut_ptr()
                    .cast::<BD::Pixel>(),
                BPC::BPC16 => ((*t).scratch.c2rust_unnamed.c2rust_unnamed_0.emu_edge_16bpc)
                    .as_mut_ptr()
                    .cast::<BD::Pixel>(),
            };
            ((*(*f).dsp).mc.emu_edge)(
                (bw4 * h_mul + (mx != 0) as c_int * 7) as intptr_t,
                (bh4 * v_mul + (my != 0) as c_int * 7) as intptr_t,
                w as intptr_t,
                h as intptr_t,
                (dx - (mx != 0) as c_int * 3) as intptr_t,
                (dy - (my != 0) as c_int * 3) as intptr_t,
                emu_edge_buf.cast(),
                (192 as c_int as c_ulong)
                    .wrapping_mul(::core::mem::size_of::<BD::Pixel>() as c_ulong)
                    as ptrdiff_t,
                (*refp).p.data[pl as usize].cast(),
                ref_stride,
            );
            r#ref = &mut *emu_edge_buf
                .offset((192 * (my != 0) as c_int * 3 + (mx != 0) as c_int * 3) as isize)
                as *mut BD::Pixel;
            ref_stride = (192 as c_int as c_ulong)
                .wrapping_mul(::core::mem::size_of::<BD::Pixel>() as c_ulong)
                as ptrdiff_t;
        } else {
            r#ref = ((*refp).p.data[pl as usize] as *mut BD::Pixel)
                .offset(BD::pxstride(ref_stride as usize) as isize * dy as isize)
                .offset(dx as isize);
        }
        if !dst8.is_null() {
            (*(*f).dsp).mc.mc[filter_2d as usize](
                dst8.cast(),
                dst_stride,
                r#ref.cast(),
                ref_stride,
                bw4 * h_mul,
                bh4 * v_mul,
                mx << (ss_hor == 0) as c_int,
                my << (ss_ver == 0) as c_int,
                (*f).bitdepth_max,
            );
        } else {
            (*(*f).dsp).mc.mct[filter_2d as usize](
                dst16,
                r#ref.cast(),
                ref_stride,
                bw4 * h_mul,
                bh4 * v_mul,
                mx << (ss_hor == 0) as c_int,
                my << (ss_ver == 0) as c_int,
                (*f).bitdepth_max,
            );
        }
    } else {
        if !(refp != &(*f).sr_cur as *const Rav1dThreadPicture) {
            unreachable!();
        }
        let orig_pos_y = (by * v_mul << 4) + mvy * ((1 as c_int) << (ss_ver == 0) as c_int);
        let orig_pos_x = (bx * h_mul << 4) + mvx * ((1 as c_int) << (ss_hor == 0) as c_int);
        let pos_y;
        let pos_x;
        let tmp: i64 = orig_pos_x as i64 * (*f).svc[refidx as usize][0].scale as i64
            + (((*f).svc[refidx as usize][0].scale - 0x4000 as c_int) * 8) as i64;
        pos_x = apply_sign64(
            ((tmp as c_longlong).abs() + 128 as c_longlong >> 8) as c_int,
            tmp,
        ) + 32;
        let tmp_0: i64 = orig_pos_y as i64 * (*f).svc[refidx as usize][1].scale as i64
            + (((*f).svc[refidx as usize][1].scale - 0x4000 as c_int) * 8) as i64;
        pos_y = apply_sign64(
            ((tmp_0 as c_longlong).abs() + 128 as c_longlong >> 8) as c_int,
            tmp_0,
        ) + 32;
        let left = pos_x >> 10;
        let top = pos_y >> 10;
        let right = (pos_x + (bw4 * h_mul - 1) * (*f).svc[refidx as usize][0].step >> 10) + 1;
        let bottom = (pos_y + (bh4 * v_mul - 1) * (*f).svc[refidx as usize][1].step >> 10) + 1;
        if DEBUG_BLOCK_INFO(&*f, &*t) {
            printf(
                b"Off %dx%d [%d,%d,%d], size %dx%d [%d,%d]\n\0" as *const u8 as *const c_char,
                left,
                top,
                orig_pos_x,
                (*f).svc[refidx as usize][0].scale,
                refidx,
                right - left,
                bottom - top,
                (*f).svc[refidx as usize][0].step,
                (*f).svc[refidx as usize][1].step,
            );
        }
        let w_0 = (*refp).p.p.w + ss_hor >> ss_hor;
        let h_0 = (*refp).p.p.h + ss_ver >> ss_ver;
        if left < 3 || top < 3 || right + 4 > w_0 || bottom + 4 > h_0 {
            let emu_edge_buf_0: *mut BD::Pixel = match BD::BPC {
                BPC::BPC8 => ((*t).scratch.c2rust_unnamed.c2rust_unnamed_0.emu_edge_8bpc)
                    .as_mut_ptr()
                    .cast::<BD::Pixel>(),
                BPC::BPC16 => ((*t).scratch.c2rust_unnamed.c2rust_unnamed_0.emu_edge_16bpc)
                    .as_mut_ptr()
                    .cast::<BD::Pixel>(),
            };
            ((*(*f).dsp).mc.emu_edge)(
                (right - left + 7) as intptr_t,
                (bottom - top + 7) as intptr_t,
                w_0 as intptr_t,
                h_0 as intptr_t,
                (left - 3) as intptr_t,
                (top - 3) as intptr_t,
                emu_edge_buf_0.cast(),
                (320 as c_int as c_ulong)
                    .wrapping_mul(::core::mem::size_of::<BD::Pixel>() as c_ulong)
                    as ptrdiff_t,
                (*refp).p.data[pl as usize].cast(),
                ref_stride,
            );
            r#ref = &mut *emu_edge_buf_0.offset((320 * 3 + 3) as isize) as *mut BD::Pixel;
            ref_stride = (320 as c_int as c_ulong)
                .wrapping_mul(::core::mem::size_of::<BD::Pixel>() as c_ulong)
                as ptrdiff_t;
            if DEBUG_BLOCK_INFO(&*f, &*t) {
                printf(b"Emu\n\0" as *const u8 as *const c_char);
            }
        } else {
            r#ref = ((*refp).p.data[pl as usize] as *mut BD::Pixel)
                .offset(BD::pxstride(ref_stride as usize) as isize * top as isize)
                .offset(left as isize);
        }
        if !dst8.is_null() {
            (*(*f).dsp).mc.mc_scaled[filter_2d as usize](
                dst8.cast(),
                dst_stride,
                r#ref.cast(),
                ref_stride,
                bw4 * h_mul,
                bh4 * v_mul,
                pos_x & 0x3ff as c_int,
                pos_y & 0x3ff as c_int,
                (*f).svc[refidx as usize][0].step,
                (*f).svc[refidx as usize][1].step,
                (*f).bitdepth_max,
            );
        } else {
            (*(*f).dsp).mc.mct_scaled[filter_2d as usize](
                dst16,
                r#ref.cast(),
                ref_stride,
                bw4 * h_mul,
                bh4 * v_mul,
                pos_x & 0x3ff as c_int,
                pos_y & 0x3ff as c_int,
                (*f).svc[refidx as usize][0].step,
                (*f).svc[refidx as usize][1].step,
                (*f).bitdepth_max,
            );
        }
    }
    return 0 as c_int;
}

// TODO(kkysen) pub(crate) temporarily until recon is fully deduplicated
pub(crate) unsafe fn obmc<BD: BitDepth>(
    t: *mut Rav1dTaskContext,
    dst: *mut BD::Pixel,
    dst_stride: ptrdiff_t,
    b_dim: *const u8,
    pl: c_int,
    bx4: c_int,
    by4: c_int,
    w4: c_int,
    h4: c_int,
) -> c_int {
    if !((*t).bx & 1 == 0 && (*t).by & 1 == 0) {
        unreachable!();
    }
    let f: *const Rav1dFrameContext = (*t).f;
    let r: *mut *mut refmvs_block = &mut *((*t).rt.r)
        .as_mut_ptr()
        .offset((((*t).by & 31) + 5) as isize)
        as *mut *mut refmvs_block;
    let lap: *mut BD::Pixel = match BD::BPC {
        BPC::BPC8 => ((*t).scratch.c2rust_unnamed.c2rust_unnamed.lap_8bpc)
            .as_mut_ptr()
            .cast::<BD::Pixel>(),
        BPC::BPC16 => ((*t).scratch.c2rust_unnamed.c2rust_unnamed.lap_16bpc)
            .as_mut_ptr()
            .cast::<BD::Pixel>(),
    };
    let ss_ver = (pl != 0
        && (*f).cur.p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint)
        as c_int;
    let ss_hor = (pl != 0
        && (*f).cur.p.layout as c_uint != RAV1D_PIXEL_LAYOUT_I444 as c_int as c_uint)
        as c_int;
    let h_mul = 4 >> ss_hor;
    let v_mul = 4 >> ss_ver;
    let mut res;
    if (*t).by > (*(*t).ts).tiling.row_start
        && (pl == 0 || *b_dim.offset(0) as c_int * h_mul + *b_dim.offset(1) as c_int * v_mul >= 16)
    {
        let mut i = 0;
        let mut x = 0;
        while x < w4 && i < cmp::min(*b_dim.offset(2) as c_int, 4 as c_int) {
            let a_r: *const refmvs_block = &mut *(*r.offset(-(1 as c_int) as isize))
                .offset(((*t).bx + x + 1) as isize)
                as *mut refmvs_block;
            let a_b_dim: *const u8 = (dav1d_block_dimensions[(*a_r).0.bs as usize]).as_ptr();
            let step4 = iclip(*a_b_dim.offset(0) as c_int, 2 as c_int, 16 as c_int);
            if (*a_r).0.r#ref.r#ref[0] as c_int > 0 {
                let ow4 = cmp::min(step4, *b_dim.offset(0) as c_int);
                let oh4 = cmp::min(*b_dim.offset(1) as c_int, 16 as c_int) >> 1;
                res = mc::<BD>(
                    t,
                    lap,
                    0 as *mut i16,
                    ((ow4 * h_mul) as c_ulong)
                        .wrapping_mul(::core::mem::size_of::<BD::Pixel>() as c_ulong)
                        as ptrdiff_t,
                    ow4,
                    oh4 * 3 + 3 >> 2,
                    (*t).bx + x,
                    (*t).by,
                    pl,
                    (*a_r).0.mv.mv[0],
                    &*((*f).refp)
                        .as_ptr()
                        .offset((*((*a_r).0.r#ref.r#ref).as_ptr().offset(0) as c_int - 1) as isize),
                    (*a_r).0.r#ref.r#ref[0] as c_int - 1,
                    dav1d_filter_2d[(*(*t).a).filter[1][(bx4 + x + 1) as usize] as usize]
                        [(*(*t).a).filter[0][(bx4 + x + 1) as usize] as usize]
                        as Filter2d,
                );
                if res != 0 {
                    return res;
                }
                ((*(*f).dsp).mc.blend_h)(
                    dst.offset((x * h_mul) as isize).cast(),
                    dst_stride,
                    lap.cast(),
                    h_mul * ow4,
                    v_mul * oh4,
                );
                i += 1;
            }
            x += step4;
        }
    }
    if (*t).bx > (*(*t).ts).tiling.col_start {
        let mut i_0 = 0;
        let mut y = 0;
        while y < h4 && i_0 < cmp::min(*b_dim.offset(3) as c_int, 4 as c_int) {
            let l_r: *const refmvs_block = &mut *(*r.offset((y + 1) as isize))
                .offset(((*t).bx - 1) as isize)
                as *mut refmvs_block;
            let l_b_dim: *const u8 = (dav1d_block_dimensions[(*l_r).0.bs as usize]).as_ptr();
            let step4_0 = iclip(*l_b_dim.offset(1) as c_int, 2 as c_int, 16 as c_int);
            if (*l_r).0.r#ref.r#ref[0] as c_int > 0 {
                let ow4_0 = cmp::min(*b_dim.offset(0) as c_int, 16 as c_int) >> 1;
                let oh4_0 = cmp::min(step4_0, *b_dim.offset(1) as c_int);
                res = mc::<BD>(
                    t,
                    lap,
                    0 as *mut i16,
                    ((h_mul * ow4_0) as c_ulong)
                        .wrapping_mul(::core::mem::size_of::<BD::Pixel>() as c_ulong)
                        as ptrdiff_t,
                    ow4_0,
                    oh4_0,
                    (*t).bx,
                    (*t).by + y,
                    pl,
                    (*l_r).0.mv.mv[0],
                    &*((*f).refp)
                        .as_ptr()
                        .offset((*((*l_r).0.r#ref.r#ref).as_ptr().offset(0) as c_int - 1) as isize),
                    (*l_r).0.r#ref.r#ref[0] as c_int - 1,
                    dav1d_filter_2d[(*t).l.filter[1][(by4 + y + 1) as usize] as usize]
                        [(*t).l.filter[0][(by4 + y + 1) as usize] as usize]
                        as Filter2d,
                );
                if res != 0 {
                    return res;
                }
                ((*(*f).dsp).mc.blend_v)(
                    dst.offset((y * v_mul) as isize * BD::pxstride(dst_stride as usize) as isize)
                        .cast(),
                    dst_stride,
                    lap.cast(),
                    h_mul * ow4_0,
                    v_mul * oh4_0,
                );
                i_0 += 1;
            }
            y += step4_0;
        }
    }
    return 0 as c_int;
}
