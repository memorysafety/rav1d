use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynCoef;
use crate::include::common::bitdepth::BPC;
use crate::include::common::dump::ac_dump;
use crate::include::common::dump::coef_dump;
use crate::include::common::dump::hex_dump;
use crate::include::common::intops::apply_sign64;
use crate::include::common::intops::iclip;
use crate::include::dav1d::dav1d::Rav1dInloopFilterType;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::include::dav1d::headers::Rav1dWarpedMotionParams;
use crate::include::dav1d::headers::Rav1dWarpedMotionType;
use crate::include::dav1d::picture::RAV1D_PICTURE_ALIGNMENT;
use crate::src::cdef_apply::rav1d_cdef_brow;
use crate::src::ctx::CaseSet;
use crate::src::env::get_uv_inter_txtp;
use crate::src::internal::CodedBlockInfo;
use crate::src::internal::Rav1dContext;
use crate::src::internal::Rav1dDSPContext;
use crate::src::internal::Rav1dFrameData;
use crate::src::internal::Rav1dTaskContext;
use crate::src::internal::Rav1dTileState;
use crate::src::internal::TileStateRef;
use crate::src::intra_edge::EdgeFlags;
use crate::src::ipred_prepare::rav1d_prepare_intra_edges;
use crate::src::ipred_prepare::sm_flag;
use crate::src::ipred_prepare::sm_uv_flag;
use crate::src::levels::mv;
use crate::src::levels::Av1Block;
use crate::src::levels::BlockSize;
use crate::src::levels::CompInterType;
use crate::src::levels::Filter2d;
use crate::src::levels::InterIntraPredMode;
use crate::src::levels::InterIntraType;
use crate::src::levels::IntraPredMode;
use crate::src::levels::MotionMode;
use crate::src::levels::RectTxfmSize;
use crate::src::levels::TxClass;
use crate::src::levels::TxfmSize;
use crate::src::levels::TxfmType;
use crate::src::levels::CFL_PRED;
use crate::src::levels::DCT_DCT;
use crate::src::levels::DC_PRED;
use crate::src::levels::FILTER_PRED;
use crate::src::levels::GLOBALMV;
use crate::src::levels::GLOBALMV_GLOBALMV;
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
use crate::src::levels::SMOOTH_PRED;
use crate::src::levels::TX_16X16;
use crate::src::levels::TX_32X32;
use crate::src::levels::TX_4X4;
use crate::src::levels::TX_64X64;
use crate::src::levels::TX_8X8;
use crate::src::levels::WHT_WHT;
use crate::src::lf_apply::rav1d_copy_lpf;
use crate::src::lf_apply::rav1d_loopfilter_sbrow_cols;
use crate::src::lf_apply::rav1d_loopfilter_sbrow_rows;
use crate::src::lr_apply::rav1d_lr_sbrow;
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
use crate::src::wedge::dav1d_ii_masks;
use crate::src::wedge::dav1d_wedge_masks;
use libc::intptr_t;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_longlong;
use std::ffi::c_uint;
use std::ffi::c_ulong;
use std::ffi::c_void;
use std::ops::BitOr;
use std::slice;

// TODO: add feature and compile-time guard around this code
/// Determine if we should print debug information for the current block.
///
/// Takes a [`Rav1dFrameData`] and a [`Rav1dTaskContext`] as arguments to
/// determine the current block and frame offset.
///
/// This a macro rather than a function so that the compiler can see which
/// specific fields are used to avoid borrowck errors.
macro_rules! debug_block_info {
    ($f:expr, $t:expr) => {
        false
            && $f.frame_hdr.as_ref().unwrap().frame_offset == 2
            && $t.by >= 0
            && $t.by < 4
            && $t.bx >= 8
            && $t.bx < 12
    };
}
pub(crate) use debug_block_info;

pub(crate) type recon_b_intra_fn =
    unsafe fn(&Rav1dFrameData, &mut Rav1dTaskContext, BlockSize, EdgeFlags, &Av1Block) -> ();

pub(crate) type recon_b_inter_fn =
    unsafe fn(&mut Rav1dFrameData, &mut Rav1dTaskContext, BlockSize, &Av1Block) -> c_int;

pub(crate) type filter_sbrow_fn =
    unsafe fn(&Rav1dContext, &mut Rav1dFrameData, &mut Rav1dTaskContext, c_int) -> ();

pub(crate) type backup_ipred_edge_fn = unsafe fn(&Rav1dFrameData, &mut Rav1dTaskContext) -> ();

pub(crate) type read_coef_blocks_fn =
    unsafe fn(&mut Rav1dFrameData, &mut Rav1dTaskContext, BlockSize, &Av1Block) -> ();

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
    layout: Rav1dPixelLayout,
) -> u8 {
    let b_dim = &dav1d_block_dimensions[bs as usize];
    if chroma != 0 {
        let ss_ver = layout == Rav1dPixelLayout::I420;
        let ss_hor = layout != Rav1dPixelLayout::I444;
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
    let offset = match tx_class {
        TxClass::TwoD => {
            mag += level(1, 1);
            *hi_mag = mag as c_uint;
            mag += level(0, 2) + level(2, 0);
            ctx_offsets.unwrap()[cmp::min(y, 4)][cmp::min(x, 4)] as usize
        }
        TxClass::H | TxClass::V => {
            mag += level(0, 2);
            *hi_mag = mag as c_uint;
            mag += level(0, 3) + level(0, 4);
            26 + if y > 1 { 10 } else { y * 5 }
        }
    };
    offset + if mag > 512 { 4 } else { (mag + 64) >> 7 }
}

unsafe fn decode_coefs<BD: BitDepth>(
    f: &Rav1dFrameData,
    t: *mut Rav1dTaskContext,
    a: &mut [u8],
    l: &mut [u8],
    tx: RectTxfmSize,
    bs: BlockSize,
    b: &Av1Block,
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
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    let lossless = frame_hdr.segmentation.lossless[b.seg_id as usize];
    let t_dim = &dav1d_txfm_dimensions[tx as usize];
    let dbg = debug_block_info!(f, &*t) && plane != 0 && false;
    if dbg {
        println!("Start: r={}", (*ts).msac.rng);
    }
    let sctx = get_skip_ctx(t_dim, bs, a, l, chroma, f.cur.p.layout) as c_int;
    let all_skip = rav1d_msac_decode_bool_adapt(
        &mut (*ts).msac,
        &mut (*ts).cdf.coef.skip[(*t_dim).ctx as usize][sctx as usize],
    ) as c_int;
    if dbg {
        println!(
            "Post-non-zero[{}][{}][{}]: r={}",
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
            dav1d_txtp_from_uvmode[b.c2rust_unnamed.c2rust_unnamed.uv_mode as usize] as c_uint
        } else {
            get_uv_inter_txtp(&*t_dim, *txtp) as c_uint
        }) as TxfmType;
    } else if frame_hdr.segmentation.qidx[b.seg_id as usize] == 0 {
        *txtp = DCT_DCT;
    } else {
        let idx: c_uint;
        if intra != 0 {
            let y_mode_nofilt: IntraPredMode =
                (if b.c2rust_unnamed.c2rust_unnamed.y_mode as c_int == FILTER_PRED as c_int {
                    dav1d_filter_mode_to_y_mode[b.c2rust_unnamed.c2rust_unnamed.y_angle as usize]
                        as c_int
                } else {
                    b.c2rust_unnamed.c2rust_unnamed.y_mode as c_int
                }) as IntraPredMode;
            if frame_hdr.reduced_txtp_set != 0 || (*t_dim).min as c_int == TX_16X16 as c_int {
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
            if dbg {
                println!(
                    "Post-txtp-intra[{}->{}][{}][{}->{}]: r={}",
                    tx as c_uint,
                    (*t_dim).min as c_int,
                    y_mode_nofilt as c_uint,
                    idx,
                    *txtp as c_uint,
                    (*ts).msac.rng,
                );
            }
        } else {
            if frame_hdr.reduced_txtp_set != 0 || (*t_dim).max as c_int == TX_32X32 as c_int {
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
            if dbg {
                println!(
                    "Post-txtp-inter[{}->{}][{}->{}]: r={}",
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
    let tx_class = dav1d_tx_type_class[*txtp as usize];
    let is_1d = tx_class != TxClass::TwoD;
    match tx2dszctx {
        0 => {
            let eob_bin_cdf = &mut (*ts).cdf.coef.eob_bin_16[chroma as usize][is_1d as usize];
            eob_bin =
                rav1d_msac_decode_symbol_adapt4(&mut (*ts).msac, eob_bin_cdf, (4 + 0) as usize)
                    as c_int;
        }
        1 => {
            let eob_bin_cdf = &mut (*ts).cdf.coef.eob_bin_32[chroma as usize][is_1d as usize];
            eob_bin =
                rav1d_msac_decode_symbol_adapt8(&mut (*ts).msac, eob_bin_cdf, (4 + 1) as usize)
                    as c_int;
        }
        2 => {
            let eob_bin_cdf = &mut (*ts).cdf.coef.eob_bin_64[chroma as usize][is_1d as usize];
            eob_bin =
                rav1d_msac_decode_symbol_adapt8(&mut (*ts).msac, eob_bin_cdf, (4 + 2) as usize)
                    as c_int;
        }
        3 => {
            let eob_bin_cdf = &mut (*ts).cdf.coef.eob_bin_128[chroma as usize][is_1d as usize];
            eob_bin =
                rav1d_msac_decode_symbol_adapt8(&mut (*ts).msac, eob_bin_cdf, (4 + 3) as usize)
                    as c_int;
        }
        4 => {
            let eob_bin_cdf = &mut (*ts).cdf.coef.eob_bin_256[chroma as usize][is_1d as usize];
            eob_bin =
                rav1d_msac_decode_symbol_adapt16(&mut (*ts).msac, eob_bin_cdf, (4 + 4) as usize)
                    as c_int;
        }
        5 => {
            let eob_bin_cdf = &mut (*ts).cdf.coef.eob_bin_512[chroma as usize];
            eob_bin =
                rav1d_msac_decode_symbol_adapt16(&mut (*ts).msac, eob_bin_cdf, (4 + 5) as usize)
                    as c_int;
        }
        6 => {
            let eob_bin_cdf = &mut (*ts).cdf.coef.eob_bin_1024[chroma as usize];
            eob_bin =
                rav1d_msac_decode_symbol_adapt16(&mut (*ts).msac, eob_bin_cdf, (4 + 6) as usize)
                    as c_int;
        }
        _ => {}
    }
    if dbg {
        println!(
            "Post-eob_bin_{}[{}][{}][{}]: r={}",
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
        if dbg {
            println!(
                "Post-eob_hi_bit[{}][{}][{}][{}]: r={}",
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
        if dbg {
            println!("Post-eob[{}]: r={}", eob, (*ts).msac.rng);
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
    let mut rc;
    let mut dc_tok;
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
        let mut scan: &[u16] = &[];
        match tx_class {
            TxClass::TwoD => {
                let nonsquare_tx: c_uint =
                    (tx as c_uint >= RTX_4X8 as c_int as c_uint) as c_int as c_uint;
                let lo_ctx_offsets = Some(
                    &dav1d_lo_ctx_offsets
                        [nonsquare_tx.wrapping_add(tx as c_uint & nonsquare_tx) as usize],
                );
                scan = dav1d_scans[tx as usize];
                let stride: ptrdiff_t = (4 * sh) as ptrdiff_t;
                let shift: c_uint = (if ((*t_dim).lh as c_int) < 4 {
                    (*t_dim).lh as c_int + 2
                } else {
                    5 as c_int
                }) as c_uint;
                let shift2: c_uint = 0 as c_int as c_uint;
                let mask: c_uint = (4 * sh - 1) as c_uint;
                levels[..(stride * (4 * sw as isize + 2)) as usize].fill(0);
                let mut x: c_uint;
                let mut y: c_uint;
                match tx_class {
                    TxClass::TwoD => {
                        rc = scan[eob as usize] as c_uint;
                        x = rc >> shift;
                        y = rc & mask;
                    }
                    TxClass::H => {
                        x = eob as c_uint & mask;
                        y = (eob >> shift) as c_uint;
                        rc = eob as c_uint;
                    }
                    TxClass::V => {
                        x = eob as c_uint & mask;
                        y = (eob >> shift) as c_uint;
                        rc = x << shift2 | y;
                    }
                }
                if dbg {
                    println!(
                        "Post-lo_tok[{}][{}][{}][{}={}={}]: r={}",
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
                    ctx = (if if tx_class == TxClass::TwoD {
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
                    if dbg {
                        println!(
                            "Post-hi_tok[{}][{}][{}][{}={}={}]: r={}",
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
                    match tx_class {
                        TxClass::TwoD => {
                            rc_i = scan[i as usize] as c_uint;
                            x = rc_i >> shift;
                            y = rc_i & mask;
                        }
                        TxClass::H => {
                            x = i as c_uint & mask;
                            y = (i >> shift) as c_uint;
                            rc_i = i as c_uint;
                        }
                        TxClass::V => {
                            x = i as c_uint & mask;
                            y = (i >> shift) as c_uint;
                            rc_i = x << shift2 | y;
                        }
                    }
                    if !(x < 32 as c_uint && y < 32 as c_uint) {
                        unreachable!();
                    }
                    let level = &mut levels[(x as isize * stride + y as isize) as usize..];
                    ctx = get_lo_ctx(
                        level,
                        tx_class,
                        &mut mag,
                        lo_ctx_offsets,
                        x as usize,
                        y as usize,
                        stride as usize,
                    ) as c_uint;
                    if tx_class == TxClass::TwoD {
                        y |= x;
                    }
                    tok = rav1d_msac_decode_symbol_adapt4(
                        &mut (*ts).msac,
                        &mut *lo_cdf.offset(ctx as isize),
                        3 as c_int as usize,
                    ) as c_int;
                    if dbg {
                        println!(
                            "Post-lo_tok[{}][{}][{}][{}={}={}]: r={}",
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
                        ctx = ((if y > (tx_class == TxClass::TwoD) as c_uint {
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
                        if dbg {
                            println!(
                                "Post-hi_tok[{}][{}][{}][{}={}={}]: r={}",
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
                ctx = if tx_class == TxClass::TwoD {
                    0
                } else {
                    get_lo_ctx(
                        levels,
                        tx_class,
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
                if dbg {
                    println!(
                        "Post-dc_lo_tok[{}][{}][{}][{}]: r={}",
                        (*t_dim).ctx as c_int,
                        chroma,
                        ctx,
                        dc_tok,
                        (*ts).msac.rng,
                    );
                }
                if dc_tok == 3 as c_uint {
                    if tx_class == TxClass::TwoD {
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
                    if dbg {
                        println!(
                            "Post-dc_hi_tok[{}][{}][0][{}]: r={}",
                            cmp::min((*t_dim).ctx as c_int, 3 as c_int),
                            chroma,
                            dc_tok,
                            (*ts).msac.rng,
                        );
                    }
                }
            }
            TxClass::H => {
                let lo_ctx_offsets = None;
                let stride: ptrdiff_t = 16 as c_int as ptrdiff_t;
                let shift: c_uint = ((*t_dim).lh as c_int + 2) as c_uint;
                let shift2: c_uint = 0 as c_int as c_uint;
                let mask: c_uint = (4 * sh - 1) as c_uint;
                levels[..(stride * (4 * sh + 2) as isize) as usize].fill(0);
                let mut x: c_uint;
                let mut y: c_uint;
                match tx_class {
                    TxClass::TwoD => {
                        rc = scan[eob as usize] as c_uint;
                        x = rc >> shift;
                        y = rc & mask;
                    }
                    TxClass::H => {
                        x = eob as c_uint & mask;
                        y = (eob >> shift) as c_uint;
                        rc = eob as c_uint;
                    }
                    TxClass::V => {
                        x = eob as c_uint & mask;
                        y = (eob >> shift) as c_uint;
                        rc = x << shift2 | y;
                    }
                }
                if dbg {
                    println!(
                        "Post-lo_tok[{}][{}][{}][{}={}={}]: r={}",
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
                    ctx = (if if tx_class == TxClass::TwoD {
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
                    if dbg {
                        println!(
                            "Post-hi_tok[{}][{}][{}][{}={}={}]: r={}",
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
                    match tx_class {
                        TxClass::TwoD => {
                            rc_i = scan[i as usize] as c_uint;
                            x = rc_i >> shift;
                            y = rc_i & mask;
                        }
                        TxClass::H => {
                            x = i as c_uint & mask;
                            y = (i >> shift) as c_uint;
                            rc_i = i as c_uint;
                        }
                        TxClass::V => {
                            x = i as c_uint & mask;
                            y = (i >> shift) as c_uint;
                            rc_i = x << shift2 | y;
                        }
                    }
                    if !(x < 32 as c_uint && y < 32 as c_uint) {
                        unreachable!();
                    }
                    let level = &mut levels[(x as isize * stride as isize + y as isize) as usize..];
                    ctx = get_lo_ctx(
                        level,
                        tx_class,
                        &mut mag,
                        lo_ctx_offsets,
                        x as usize,
                        y as usize,
                        stride as usize,
                    ) as c_uint;
                    if tx_class == TxClass::TwoD {
                        y |= x;
                    }
                    tok = rav1d_msac_decode_symbol_adapt4(
                        &mut (*ts).msac,
                        &mut *lo_cdf.offset(ctx as isize),
                        3 as c_int as usize,
                    ) as c_int;
                    if dbg {
                        println!(
                            "Post-lo_tok[{}][{}][{}][{}={}={}]: r={}",
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
                        ctx = ((if y > (tx_class == TxClass::TwoD) as c_uint {
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
                        if dbg {
                            println!(
                                "Post-hi_tok[{}][{}][{}][{}={}={}]: r={}",
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
                ctx = if tx_class == TxClass::TwoD {
                    0 as c_int as c_uint
                } else {
                    get_lo_ctx(
                        levels,
                        tx_class,
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
                if dbg {
                    println!(
                        "Post-dc_lo_tok[{}][{}][{}][{}]: r={}",
                        (*t_dim).ctx as c_int,
                        chroma,
                        ctx,
                        dc_tok,
                        (*ts).msac.rng,
                    );
                }
                if dc_tok == 3 as c_uint {
                    if tx_class == TxClass::TwoD {
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
                    if dbg {
                        println!(
                            "Post-dc_hi_tok[{}][{}][0][{}]: r={}",
                            cmp::min((*t_dim).ctx as c_int, 3 as c_int),
                            chroma,
                            dc_tok,
                            (*ts).msac.rng,
                        );
                    }
                }
            }
            TxClass::V => {
                let lo_ctx_offsets = None;
                let stride: ptrdiff_t = 16 as c_int as ptrdiff_t;
                let shift: c_uint = ((*t_dim).lw as c_int + 2) as c_uint;
                let shift2: c_uint = ((*t_dim).lh as c_int + 2) as c_uint;
                let mask: c_uint = (4 * sw - 1) as c_uint;
                levels[..(stride * (4 * sw + 2) as isize) as usize].fill(0);
                let mut x: c_uint;
                let mut y: c_uint;
                match tx_class {
                    TxClass::TwoD => {
                        rc = scan[eob as usize] as c_uint;
                        x = rc >> shift;
                        y = rc & mask;
                    }
                    TxClass::H => {
                        x = eob as c_uint & mask;
                        y = (eob >> shift) as c_uint;
                        rc = eob as c_uint;
                    }
                    TxClass::V => {
                        x = eob as c_uint & mask;
                        y = (eob >> shift) as c_uint;
                        rc = x << shift2 | y;
                    }
                }
                if dbg {
                    println!(
                        "Post-lo_tok[{}][{}][{}][{}={}={}]: r={}",
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
                    ctx = (if if tx_class == TxClass::TwoD {
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
                    if dbg {
                        println!(
                            "Post-hi_tok[{}][{}][{}][{}={}={}]: r={}",
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
                    match tx_class {
                        TxClass::TwoD => {
                            rc_i = scan[i as usize] as c_uint;
                            x = rc_i >> shift;
                            y = rc_i & mask;
                        }
                        TxClass::H => {
                            x = i as c_uint & mask;
                            y = (i >> shift) as c_uint;
                            rc_i = i as c_uint;
                        }
                        TxClass::V => {
                            x = i as c_uint & mask;
                            y = (i >> shift) as c_uint;
                            rc_i = x << shift2 | y;
                        }
                    }
                    if !(x < 32 as c_uint && y < 32 as c_uint) {
                        unreachable!();
                    }
                    let level = &mut levels[(x as isize * stride + y as isize) as usize..];
                    ctx = get_lo_ctx(
                        level,
                        tx_class,
                        &mut mag,
                        lo_ctx_offsets,
                        x as usize,
                        y as usize,
                        stride as usize,
                    ) as c_uint;
                    if tx_class == TxClass::TwoD {
                        y |= x;
                    }
                    tok = rav1d_msac_decode_symbol_adapt4(
                        &mut (*ts).msac,
                        &mut *lo_cdf.offset(ctx as isize),
                        3 as c_int as usize,
                    ) as c_int;
                    if dbg {
                        println!(
                            "Post-lo_tok[{}][{}][{}][{}={}={}]: r={}",
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
                        ctx = ((if y > (tx_class == TxClass::TwoD) as c_int as c_uint {
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
                        if dbg {
                            println!(
                                "Post-hi_tok[{}][{}][{}][{}={}={}]: r={}",
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
                ctx = if tx_class == TxClass::TwoD {
                    0 as c_int as c_uint
                } else {
                    get_lo_ctx(
                        levels,
                        tx_class,
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
                if dbg {
                    println!(
                        "Post-dc_lo_tok[{}][{}][{}][{}]: r={}",
                        (*t_dim).ctx as c_int,
                        chroma,
                        ctx,
                        dc_tok,
                        (*ts).msac.rng,
                    );
                }
                if dc_tok == 3 as c_uint {
                    if tx_class == TxClass::TwoD {
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
                    if dbg {
                        println!(
                            "Post-dc_hi_tok[{}][{}][0][{}]: r={}",
                            cmp::min((*t_dim).ctx as c_int, 3 as c_int),
                            chroma,
                            dc_tok,
                            (*ts).msac.rng,
                        );
                    }
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
        if dbg {
            println!(
                "Post-dc_lo_tok[{}][{}][{}][{}]: r={}",
                (*t_dim).ctx as c_int,
                chroma,
                0 as c_int,
                dc_tok,
                (*ts).msac.rng,
            );
        }
        if tok_br == 2 {
            dc_tok = rav1d_msac_decode_hi_tok(&mut (*ts).msac, &mut *hi_cdf.offset(0));
            if dbg {
                println!(
                    "Post-dc_hi_tok[{}][{}][0][{}]: r={}",
                    cmp::min((*t_dim).ctx as c_int, 3 as c_int),
                    chroma,
                    dc_tok,
                    (*ts).msac.rng,
                );
            }
        }
        rc = 0 as c_int as c_uint;
    }
    let dq = match (*ts).dq {
        TileStateRef::Frame => &f.dq,
        TileStateRef::Local => &(*ts).dqmem,
    };
    let dq_tbl = &dq[b.seg_id as usize][plane as usize];
    let qm_tbl: *const u8 = if (*txtp as c_uint) < IDTX as c_int as c_uint {
        f.qm[tx as usize][plane as usize]
    } else {
        0 as *const u8
    };
    let dq_shift = cmp::max(0 as c_int, (*t_dim).ctx as c_int - 2);
    let cf_max = !(!(127 as c_uint)
        << (match BD::BPC {
            BPC::BPC8 => 8,
            BPC::BPC16 => f.cur.p.bpc,
        })) as c_int;
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
        if dbg {
            println!(
                "Post-dc_sign[{}][{}][{}]: r={}",
                chroma,
                dc_sign_ctx,
                dc_sign,
                (*ts).msac.rng,
            );
        }
        dc_dq = dq_tbl[0] as c_int;
        dc_sign_level = (dc_sign - 1 & (2 as c_int) << 6) as c_uint;
        if !qm_tbl.is_null() {
            dc_dq = dc_dq * *qm_tbl.offset(0) as c_int + 16 >> 5;
            if dc_tok == 15 as c_uint {
                dc_tok = (read_golomb(&mut (*ts).msac)).wrapping_add(15 as c_int as c_uint);
                if dbg {
                    println!(
                        "Post-dc_residual[{}->{}]: r={}",
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
                if dbg {
                    println!(
                        "Post-dc_residual[{}->{}]: r={}",
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
            let ac_dq: c_uint = dq_tbl[1] as c_uint;
            loop {
                let sign = rav1d_msac_decode_bool_equi(&mut (*ts).msac) as c_int;
                if dbg {
                    println!("Post-sign[{}={}]: r={}", rc, sign, (*ts).msac.rng);
                }
                let rc_tok: c_uint = (*cf.offset(rc as isize)).as_::<c_uint>();
                let mut tok: c_uint;
                let mut dq: c_uint = ac_dq
                    .wrapping_mul(*qm_tbl.offset(rc as isize) as c_uint)
                    .wrapping_add(16 as c_int as c_uint)
                    >> 5;
                let dq_sat;
                if rc_tok >= ((15 as c_int) << 11) as c_uint {
                    tok = (read_golomb(&mut (*ts).msac)).wrapping_add(15 as c_int as c_uint);
                    if dbg {
                        println!(
                            "Post-residual[{}={}->{}]: r={}",
                            rc,
                            tok.wrapping_sub(15 as c_int as c_uint),
                            tok,
                            (*ts).msac.rng,
                        );
                    }
                    tok &= 0xfffff as c_int as c_uint;
                    dq = dq.wrapping_mul(tok) & 0xffffff as c_int as c_uint;
                } else {
                    tok = rc_tok >> 11;
                    dq = dq.wrapping_mul(tok);
                    if !(dq <= 0xffffff as c_int as c_uint) {
                        unreachable!();
                    }
                }
                cul_level = cul_level.wrapping_add(tok);
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
            let ac_dq: c_uint = dq_tbl[1] as c_uint;
            loop {
                let sign = rav1d_msac_decode_bool_equi(&mut (*ts).msac) as c_int;
                if dbg {
                    println!("Post-sign[{}={}]: r={}", rc, sign, (*ts).msac.rng);
                }
                let rc_tok: c_uint = (*cf.offset(rc as isize)).as_::<c_uint>();
                let mut tok: c_uint;
                let mut dq;
                if rc_tok >= ((15 as c_int) << 11) as c_uint {
                    tok = (read_golomb(&mut (*ts).msac)).wrapping_add(15 as c_int as c_uint);
                    if dbg {
                        println!(
                            "Post-residual[{}={}->{}]: r={}",
                            rc,
                            tok.wrapping_sub(15 as c_int as c_uint),
                            tok,
                            (*ts).msac.rng,
                        );
                    }
                    tok &= 0xfffff as c_int as c_uint;
                    dq = ((ac_dq.wrapping_mul(tok) & 0xffffff as c_int as c_uint) >> dq_shift)
                        as c_int;
                    dq = cmp::min(dq as c_uint, (cf_max + sign) as c_uint) as c_int;
                } else {
                    tok = rc_tok >> 11;
                    dq = (ac_dq.wrapping_mul(tok) >> dq_shift) as c_int;
                    if !(dq <= cf_max) {
                        unreachable!();
                    }
                }
                cul_level = cul_level.wrapping_add(tok);
                *cf.offset(rc as isize) = (if sign != 0 { -dq } else { dq }).as_::<BD::Coef>();
                rc = rc_tok & 0x3ff as c_int as c_uint;
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

unsafe fn read_coef_tree<BD: BitDepth>(
    f: &mut Rav1dFrameData,
    t: *mut Rav1dTaskContext,
    bs: BlockSize,
    b: &Av1Block,
    ytx: RectTxfmSize,
    depth: c_int,
    tx_split: *const u16,
    x_off: c_int,
    y_off: c_int,
    mut dst: *mut BD::Pixel,
) {
    let ts: *mut Rav1dTileState = (*t).ts;
    let dsp: *const Rav1dDSPContext = f.dsp;
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
            f,
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
        if txw >= txh && (*t).bx < f.bw {
            read_coef_tree::<BD>(
                f,
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
        if txh >= txw && (*t).by < f.bh {
            if !dst.is_null() {
                dst = dst.offset((4 * txsh) as isize * BD::pxstride(f.cur.stride[0]));
            }
            read_coef_tree::<BD>(
                f,
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
            if txw >= txh && (*t).bx < f.bw {
                read_coef_tree::<BD>(
                    f,
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
        let mut cbi_idx = 0;
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
            cbi_idx = ((*t).by as isize * f.b4_stride + (*t).bx as isize) as usize;
        } else {
            cf = BD::select_mut(&mut (*t).cf).0.as_mut_ptr();
        }
        if (*t).frame_thread.pass != 2 as c_int {
            eob = decode_coefs::<BD>(
                f,
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
            if debug_block_info!(f, &*t) {
                println!(
                    "Post-y-cf-blk[tx={},txtp={},eob={}]: r={}",
                    ytx as c_uint,
                    txtp as c_uint,
                    eob,
                    (*ts).msac.rng,
                );
            }
            CaseSet::<16, true>::many(
                [&mut (*t).l, &mut *(*t).a],
                [
                    cmp::min(txh, f.bh - (*t).by) as usize,
                    cmp::min(txw, f.bw - (*t).bx) as usize,
                ],
                [by4 as usize, bx4 as usize],
                |case, dir| {
                    case.set(&mut dir.lcoef.0, cf_ctx);
                },
            );
            let txtp_map = &mut (*t).scratch.c2rust_unnamed_0.ac_txtp_map.txtp_map
                [(by4 * 32 + bx4) as usize..];
            CaseSet::<16, false>::one((), txw as usize, 0, |case, ()| {
                for txtp_map in txtp_map.chunks_mut(32).take(txh as usize) {
                    case.set(txtp_map, txtp);
                }
            });
            if (*t).frame_thread.pass == 1 {
                f.frame_thread.cbi[cbi_idx][0] = CodedBlockInfo::new(eob as i16, txtp);
            }
        } else {
            let cbi = f.frame_thread.cbi[cbi_idx][0];
            eob = cbi.eob().into();
            txtp = cbi.txtp();
        }
        if (*t).frame_thread.pass & 1 == 0 {
            if dst.is_null() {
                unreachable!();
            }
            if eob >= 0 {
                if debug_block_info!(f, &*t) && 0 != 0 {
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
                    f.cur.stride[0],
                    cf.cast(),
                    eob,
                    f.bitdepth_max,
                );
                if debug_block_info!(f, &*t) && 0 != 0 {
                    hex_dump::<BD>(
                        dst,
                        f.cur.stride[0] as usize,
                        (*t_dim).w as usize * 4,
                        (*t_dim).h as usize * 4,
                        "recon",
                    );
                }
            }
        }
    };
}

pub(crate) unsafe fn rav1d_read_coef_blocks<BD: BitDepth>(
    f: &mut Rav1dFrameData,
    t: &mut Rav1dTaskContext,
    bs: BlockSize,
    b: &Av1Block,
) {
    let ss_ver = (f.cur.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint) as c_int;
    let ss_hor = (f.cur.p.layout as c_uint != Rav1dPixelLayout::I444 as c_int as c_uint) as c_int;
    let bx4 = t.bx & 31;
    let by4 = t.by & 31;
    let cbx4 = bx4 >> ss_hor;
    let cby4 = by4 >> ss_ver;
    let b_dim: *const u8 = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bw4 = *b_dim.offset(0) as c_int;
    let bh4 = *b_dim.offset(1) as c_int;
    let cbw4 = bw4 + ss_hor >> ss_hor;
    let cbh4 = bh4 + ss_ver >> ss_ver;
    let has_chroma = (f.cur.p.layout as c_uint != Rav1dPixelLayout::I400 as c_int as c_uint
        && (bw4 > ss_hor || t.bx & 1 != 0)
        && (bh4 > ss_ver || t.by & 1 != 0)) as c_int;
    if b.skip != 0 {
        CaseSet::<32, false>::many(
            [&mut t.l, &mut *t.a],
            [bh4 as usize, bw4 as usize],
            [by4 as usize, bx4 as usize],
            |case, dir| {
                case.set(&mut dir.lcoef.0, 0x40);
            },
        );
        if has_chroma != 0 {
            CaseSet::<32, false>::many(
                [&mut t.l, &mut *t.a],
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
    let ts: *mut Rav1dTileState = t.ts;
    let w4 = cmp::min(bw4, f.bw - t.bx);
    let h4 = cmp::min(bh4, f.bh - t.by);
    let cw4 = w4 + ss_hor >> ss_hor;
    let ch4 = h4 + ss_ver >> ss_ver;
    if !(t.frame_thread.pass == 1) {
        unreachable!();
    }
    if b.skip != 0 {
        unreachable!();
    }
    let uv_t_dim: *const TxfmInfo =
        &*dav1d_txfm_dimensions.as_ptr().offset(b.uvtx as isize) as *const TxfmInfo;
    let t_dim: *const TxfmInfo = &*dav1d_txfm_dimensions.as_ptr().offset(
        (if b.intra as c_int != 0 {
            b.c2rust_unnamed.c2rust_unnamed.tx as c_int
        } else {
            b.c2rust_unnamed.c2rust_unnamed_0.max_ytx as c_int
        }) as isize,
    ) as *const TxfmInfo;
    let tx_split: [u16; 2] = [
        b.c2rust_unnamed.c2rust_unnamed_0.tx_split0 as u16,
        b.c2rust_unnamed.c2rust_unnamed_0.tx_split1,
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
            t.by += init_y;
            while y < sub_h4 {
                let cbi_idx = (t.by as isize * f.b4_stride) as usize;
                let mut x_off = (init_x != 0) as c_int;
                x = init_x;
                t.bx += init_x;
                while x < sub_w4 {
                    if b.intra == 0 {
                        read_coef_tree::<BD>(
                            f,
                            t,
                            bs,
                            b,
                            b.c2rust_unnamed.c2rust_unnamed_0.max_ytx as RectTxfmSize,
                            0 as c_int,
                            tx_split.as_ptr(),
                            x_off,
                            y_off,
                            0 as *mut BD::Pixel,
                        );
                    } else {
                        let mut cf_ctx: u8 = 0x40 as c_int as u8;
                        let mut txtp: TxfmType = DCT_DCT;
                        let eob = decode_coefs::<BD>(
                            f,
                            t,
                            &mut (*t.a).lcoef.0[(bx4 + x) as usize..],
                            &mut t.l.lcoef.0[(by4 + y) as usize..],
                            b.c2rust_unnamed.c2rust_unnamed.tx as RectTxfmSize,
                            bs,
                            b,
                            1 as c_int,
                            0 as c_int,
                            (*ts).frame_thread[1].cf as *mut BD::Coef,
                            &mut txtp,
                            &mut cf_ctx,
                        ) as c_int;
                        if debug_block_info!(f, t) {
                            println!(
                                "Post-y-cf-blk[tx={},txtp={},eob={}]: r={}",
                                b.c2rust_unnamed.c2rust_unnamed.tx as c_int,
                                txtp as c_uint,
                                eob,
                                (*ts).msac.rng,
                            );
                        }
                        f.frame_thread.cbi[cbi_idx..][t.bx as usize][0] =
                            CodedBlockInfo::new(eob as i16, txtp);
                        (*ts).frame_thread[1].cf = ((*ts).frame_thread[1].cf as *mut BD::Coef)
                            .offset(
                                (cmp::min((*t_dim).w as c_int, 8 as c_int)
                                    * cmp::min((*t_dim).h as c_int, 8 as c_int)
                                    * 16) as isize,
                            ) as *mut DynCoef;
                        CaseSet::<16, true>::many(
                            [&mut t.l, &mut *t.a],
                            [
                                cmp::min((*t_dim).h as i32, f.bh - t.by) as usize,
                                cmp::min((*t_dim).w as i32, f.bw - t.bx) as usize,
                            ],
                            [(by4 + y) as usize, (bx4 + x) as usize],
                            |case, dir| {
                                case.set(&mut dir.lcoef.0, cf_ctx);
                            },
                        );
                    }
                    x += (*t_dim).w as c_int;
                    t.bx += (*t_dim).w as c_int;
                    x_off += 1;
                }
                t.bx -= x;
                y += (*t_dim).h as c_int;
                t.by += (*t_dim).h as c_int;
                y_off += 1;
            }
            t.by -= y;
            if !(has_chroma == 0) {
                let sub_ch4 = cmp::min(ch4, init_y + 16 >> ss_ver);
                let sub_cw4 = cmp::min(cw4, init_x + 16 >> ss_hor);
                let mut pl = 0;
                while pl < 2 {
                    y = init_y >> ss_ver;
                    t.by += init_y;
                    while y < sub_ch4 {
                        let cbi_idx = (t.by as isize * f.b4_stride) as usize;
                        x = init_x >> ss_hor;
                        t.bx += init_x;
                        while x < sub_cw4 {
                            let mut cf_ctx: u8 = 0x40 as c_int as u8;
                            let mut txtp: TxfmType = DCT_DCT;
                            if b.intra == 0 {
                                txtp = t.scratch.c2rust_unnamed_0.ac_txtp_map.txtp_map
                                    [((by4 + (y << ss_ver)) * 32 + bx4 + (x << ss_hor)) as usize]
                                    as TxfmType;
                            }
                            let eob = decode_coefs::<BD>(
                                f,
                                t,
                                &mut (*t.a).ccoef.0[pl as usize][(cbx4 + x) as usize..],
                                &mut t.l.ccoef.0[pl as usize][(cby4 + y) as usize..],
                                b.uvtx as RectTxfmSize,
                                bs,
                                b,
                                b.intra as c_int,
                                1 + pl,
                                (*ts).frame_thread[1].cf as *mut BD::Coef,
                                &mut txtp,
                                &mut cf_ctx,
                            );
                            if debug_block_info!(f, t) {
                                println!(
                                    "Post-uv-cf-blk[pl={},tx={},txtp={},eob={}]: r={}",
                                    pl,
                                    b.uvtx as c_int,
                                    txtp as c_uint,
                                    eob,
                                    (*ts).msac.rng,
                                );
                            }
                            f.frame_thread.cbi[cbi_idx..][t.bx as usize][(1 + pl) as usize] =
                                CodedBlockInfo::new(eob as i16, txtp);
                            (*ts).frame_thread[1].cf =
                                ((*ts).frame_thread[1].cf as *mut BD::Coef).offset(
                                    ((*uv_t_dim).w as c_int * (*uv_t_dim).h as c_int * 16) as isize,
                                ) as *mut DynCoef;
                            CaseSet::<16, true>::many(
                                [&mut t.l, &mut *t.a],
                                [
                                    cmp::min((*uv_t_dim).h as i32, f.bh - t.by + ss_ver >> ss_ver)
                                        as usize,
                                    cmp::min((*uv_t_dim).w as i32, f.bw - t.bx + ss_hor >> ss_hor)
                                        as usize,
                                ],
                                [(cby4 + y) as usize, (cbx4 + x) as usize],
                                |case, dir| {
                                    case.set(&mut dir.ccoef.0[pl as usize], cf_ctx);
                                },
                            );
                            x += (*uv_t_dim).w as c_int;
                            t.bx += ((*uv_t_dim).w as c_int) << ss_hor;
                        }
                        t.bx -= x << ss_hor;
                        y += (*uv_t_dim).h as c_int;
                        t.by += ((*uv_t_dim).h as c_int) << ss_ver;
                    }
                    t.by -= y << ss_ver;
                    pl += 1;
                }
            }
            init_x += 16 as c_int;
        }
        init_y += 16 as c_int;
    }
}

unsafe fn mc<BD: BitDepth>(
    f: &Rav1dFrameData,
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
    let ss_ver =
        (pl != 0 && f.cur.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint) as c_int;
    let ss_hor =
        (pl != 0 && f.cur.p.layout as c_uint != Rav1dPixelLayout::I444 as c_int as c_uint) as c_int;
    let h_mul = 4 >> ss_hor;
    let v_mul = 4 >> ss_ver;
    let mvx = mv.x as c_int;
    let mvy = mv.y as c_int;
    let mx = mvx & 15 >> (ss_hor == 0) as c_int;
    let my = mvy & 15 >> (ss_ver == 0) as c_int;
    let mut ref_stride: ptrdiff_t = (*refp).p.stride[(pl != 0) as c_int as usize];
    let r#ref: *const BD::Pixel;
    if (*refp).p.p.w == f.cur.p.w && (*refp).p.p.h == f.cur.p.h {
        let dx = bx * h_mul + (mvx >> 3 + ss_hor);
        let dy = by * v_mul + (mvy >> 3 + ss_ver);
        let w;
        let h;
        if (*refp).p.data.data[0] != f.cur.data.data[0] {
            w = f.cur.p.w + ss_hor >> ss_hor;
            h = f.cur.p.h + ss_ver >> ss_ver;
        } else {
            w = f.bw * 4 >> ss_hor;
            h = f.bh * 4 >> ss_ver;
        }
        if dx < (mx != 0) as c_int * 3
            || dy < (my != 0) as c_int * 3
            || dx + bw4 * h_mul + (mx != 0) as c_int * 4 > w
            || dy + bh4 * v_mul + (my != 0) as c_int * 4 > h
        {
            let emu_edge_buf =
                BD::select_mut(&mut (*t).scratch.c2rust_unnamed.emu_edge).as_mut_ptr();
            ((*f.dsp).mc.emu_edge)(
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
                (*refp).p.data.data[pl as usize].cast(),
                ref_stride,
            );
            r#ref = &mut *emu_edge_buf
                .offset((192 * (my != 0) as c_int * 3 + (mx != 0) as c_int * 3) as isize)
                as *mut BD::Pixel;
            ref_stride = (192 as c_int as c_ulong)
                .wrapping_mul(::core::mem::size_of::<BD::Pixel>() as c_ulong)
                as ptrdiff_t;
        } else {
            r#ref = ((*refp).p.data.data[pl as usize] as *mut BD::Pixel)
                .offset(BD::pxstride(ref_stride) * dy as isize)
                .offset(dx as isize);
        }
        if !dst8.is_null() {
            (*f.dsp).mc.mc[filter_2d as usize](
                dst8.cast(),
                dst_stride,
                r#ref.cast(),
                ref_stride,
                bw4 * h_mul,
                bh4 * v_mul,
                mx << (ss_hor == 0) as c_int,
                my << (ss_ver == 0) as c_int,
                f.bitdepth_max,
            );
        } else {
            (*f.dsp).mc.mct[filter_2d as usize](
                dst16,
                r#ref.cast(),
                ref_stride,
                bw4 * h_mul,
                bh4 * v_mul,
                mx << (ss_hor == 0) as c_int,
                my << (ss_ver == 0) as c_int,
                f.bitdepth_max,
            );
        }
    } else {
        if !(refp != &f.sr_cur as *const Rav1dThreadPicture) {
            unreachable!();
        }
        let orig_pos_y = (by * v_mul << 4) + mvy * ((1 as c_int) << (ss_ver == 0) as c_int);
        let orig_pos_x = (bx * h_mul << 4) + mvx * ((1 as c_int) << (ss_hor == 0) as c_int);
        let pos_y;
        let pos_x;
        let tmp: i64 = orig_pos_x as i64 * f.svc[refidx as usize][0].scale as i64
            + ((f.svc[refidx as usize][0].scale - 0x4000 as c_int) * 8) as i64;
        pos_x = apply_sign64(
            ((tmp as c_longlong).abs() + 128 as c_longlong >> 8) as c_int,
            tmp,
        ) + 32;
        let tmp: i64 = orig_pos_y as i64 * f.svc[refidx as usize][1].scale as i64
            + ((f.svc[refidx as usize][1].scale - 0x4000 as c_int) * 8) as i64;
        pos_y = apply_sign64(
            ((tmp as c_longlong).abs() + 128 as c_longlong >> 8) as c_int,
            tmp,
        ) + 32;
        let left = pos_x >> 10;
        let top = pos_y >> 10;
        let right = (pos_x + (bw4 * h_mul - 1) * (*f).svc[refidx as usize][0].step >> 10) + 1;
        let bottom = (pos_y + (bh4 * v_mul - 1) * (*f).svc[refidx as usize][1].step >> 10) + 1;
        if debug_block_info!(&*f, &*t) {
            println!(
                "Off {}x{} [{},{},{}], size {}x{} [{},{}]",
                left,
                top,
                orig_pos_x,
                f.svc[refidx as usize][0].scale,
                refidx,
                right - left,
                bottom - top,
                f.svc[refidx as usize][0].step,
                f.svc[refidx as usize][1].step,
            );
        }
        let w = (*refp).p.p.w + ss_hor >> ss_hor;
        let h = (*refp).p.p.h + ss_ver >> ss_ver;
        if left < 3 || top < 3 || right + 4 > w || bottom + 4 > h {
            let emu_edge_buf =
                BD::select_mut(&mut (*t).scratch.c2rust_unnamed.emu_edge).as_mut_ptr();
            ((*f.dsp).mc.emu_edge)(
                (right - left + 7) as intptr_t,
                (bottom - top + 7) as intptr_t,
                w as intptr_t,
                h as intptr_t,
                (left - 3) as intptr_t,
                (top - 3) as intptr_t,
                emu_edge_buf.cast(),
                (320 as c_int as c_ulong)
                    .wrapping_mul(::core::mem::size_of::<BD::Pixel>() as c_ulong)
                    as ptrdiff_t,
                (*refp).p.data.data[pl as usize].cast(),
                ref_stride,
            );
            r#ref = &mut *emu_edge_buf.offset((320 * 3 + 3) as isize) as *mut BD::Pixel;
            ref_stride = (320 as c_int as c_ulong)
                .wrapping_mul(::core::mem::size_of::<BD::Pixel>() as c_ulong)
                as ptrdiff_t;
            if debug_block_info!(&*f, &*t) {
                println!("Emu");
            }
        } else {
            r#ref = ((*refp).p.data.data[pl as usize] as *mut BD::Pixel)
                .offset(BD::pxstride(ref_stride) * top as isize)
                .offset(left as isize);
        }
        if !dst8.is_null() {
            (*f.dsp).mc.mc_scaled[filter_2d as usize](
                dst8.cast(),
                dst_stride,
                r#ref.cast(),
                ref_stride,
                bw4 * h_mul,
                bh4 * v_mul,
                pos_x & 0x3ff as c_int,
                pos_y & 0x3ff as c_int,
                f.svc[refidx as usize][0].step,
                f.svc[refidx as usize][1].step,
                f.bitdepth_max,
            );
        } else {
            (*f.dsp).mc.mct_scaled[filter_2d as usize](
                dst16,
                r#ref.cast(),
                ref_stride,
                bw4 * h_mul,
                bh4 * v_mul,
                pos_x & 0x3ff as c_int,
                pos_y & 0x3ff as c_int,
                f.svc[refidx as usize][0].step,
                f.svc[refidx as usize][1].step,
                f.bitdepth_max,
            );
        }
    }
    return 0 as c_int;
}

unsafe fn obmc<BD: BitDepth>(
    f: &Rav1dFrameData,
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
    let r: *mut *mut refmvs_block = &mut *((*t).rt.r)
        .as_mut_ptr()
        .offset((((*t).by & 31) + 5) as isize)
        as *mut *mut refmvs_block;
    let lap = BD::select_mut(&mut (*t).scratch.c2rust_unnamed.c2rust_unnamed.lap).as_mut_ptr();
    let ss_ver =
        (pl != 0 && f.cur.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint) as c_int;
    let ss_hor =
        (pl != 0 && f.cur.p.layout as c_uint != Rav1dPixelLayout::I444 as c_int as c_uint) as c_int;
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
                    f,
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
                    &*(f.refp)
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
                ((*f.dsp).mc.blend_h)(
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
        let mut i = 0;
        let mut y = 0;
        while y < h4 && i < cmp::min(*b_dim.offset(3) as c_int, 4 as c_int) {
            let l_r: *const refmvs_block = &mut *(*r.offset((y + 1) as isize))
                .offset(((*t).bx - 1) as isize)
                as *mut refmvs_block;
            let l_b_dim: *const u8 = (dav1d_block_dimensions[(*l_r).0.bs as usize]).as_ptr();
            let step4 = iclip(*l_b_dim.offset(1) as c_int, 2 as c_int, 16 as c_int);
            if (*l_r).0.r#ref.r#ref[0] as c_int > 0 {
                let ow4 = cmp::min(*b_dim.offset(0) as c_int, 16 as c_int) >> 1;
                let oh4 = cmp::min(step4, *b_dim.offset(1) as c_int);
                res = mc::<BD>(
                    f,
                    t,
                    lap,
                    0 as *mut i16,
                    ((h_mul * ow4) as c_ulong)
                        .wrapping_mul(::core::mem::size_of::<BD::Pixel>() as c_ulong)
                        as ptrdiff_t,
                    ow4,
                    oh4,
                    (*t).bx,
                    (*t).by + y,
                    pl,
                    (*l_r).0.mv.mv[0],
                    &*(f.refp)
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
                ((*f.dsp).mc.blend_v)(
                    dst.offset((y * v_mul) as isize * BD::pxstride(dst_stride))
                        .cast(),
                    dst_stride,
                    lap.cast(),
                    h_mul * ow4,
                    v_mul * oh4,
                );
                i += 1;
            }
            y += step4;
        }
    }
    return 0 as c_int;
}

unsafe fn warp_affine<BD: BitDepth>(
    f: &Rav1dFrameData,
    t: *mut Rav1dTaskContext,
    mut dst8: *mut BD::Pixel,
    mut dst16: *mut i16,
    dstride: ptrdiff_t,
    b_dim: *const u8,
    pl: c_int,
    refp: *const Rav1dThreadPicture,
    wmp: *const Rav1dWarpedMotionParams,
) -> c_int {
    if (dst8 != 0 as *mut c_void as *mut BD::Pixel) as c_int
        ^ (dst16 != 0 as *mut c_void as *mut i16) as c_int
        == 0
    {
        unreachable!();
    }
    let dsp: *const Rav1dDSPContext = f.dsp;
    let ss_ver =
        (pl != 0 && f.cur.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint) as c_int;
    let ss_hor =
        (pl != 0 && f.cur.p.layout as c_uint != Rav1dPixelLayout::I444 as c_int as c_uint) as c_int;
    let h_mul = 4 >> ss_hor;
    let v_mul = 4 >> ss_ver;
    if !(*b_dim.offset(0) as c_int * h_mul & 7 == 0 && *b_dim.offset(1) as c_int * v_mul & 7 == 0) {
        unreachable!();
    }
    let mat: *const i32 = ((*wmp).matrix).as_ptr();
    let width = (*refp).p.p.w + ss_hor >> ss_hor;
    let height = (*refp).p.p.h + ss_ver >> ss_ver;
    let mut y = 0;
    while y < *b_dim.offset(1) as c_int * v_mul {
        let src_y = (*t).by * 4 + ((y + 4) << ss_ver);
        let mat3_y: i64 = *mat.offset(3) as i64 * src_y as i64 + *mat.offset(0) as i64;
        let mat5_y: i64 = *mat.offset(5) as i64 * src_y as i64 + *mat.offset(1) as i64;
        let mut x = 0;
        while x < *b_dim.offset(0) as c_int * h_mul {
            let src_x = (*t).bx * 4 + ((x + 4) << ss_hor);
            let mvx: i64 = *mat.offset(2) as i64 * src_x as i64 + mat3_y >> ss_hor;
            let mvy: i64 = *mat.offset(4) as i64 * src_x as i64 + mat5_y >> ss_ver;
            let dx = (mvx >> 16) as c_int - 4;
            let mx = (mvx as c_int & 0xffff as c_int)
                - (*wmp).alpha() as c_int * 4
                - (*wmp).beta() as c_int * 7
                & !(0x3f as c_int);
            let dy = (mvy >> 16) as c_int - 4;
            let my = (mvy as c_int & 0xffff as c_int)
                - (*wmp).gamma() as c_int * 4
                - (*wmp).delta() as c_int * 4
                & !(0x3f as c_int);
            let ref_ptr: *const BD::Pixel;
            let mut ref_stride: ptrdiff_t = (*refp).p.stride[(pl != 0) as c_int as usize];
            if dx < 3 || dx + 8 + 4 > width || dy < 3 || dy + 8 + 4 > height {
                let emu_edge_buf =
                    BD::select_mut(&mut (*t).scratch.c2rust_unnamed.emu_edge).as_mut_ptr();
                ((*f.dsp).mc.emu_edge)(
                    15 as c_int as intptr_t,
                    15 as c_int as intptr_t,
                    width as intptr_t,
                    height as intptr_t,
                    (dx - 3) as intptr_t,
                    (dy - 3) as intptr_t,
                    emu_edge_buf.cast(),
                    (32 as c_int as c_ulong)
                        .wrapping_mul(::core::mem::size_of::<BD::Pixel>() as c_ulong)
                        as ptrdiff_t,
                    (*refp).p.data.data[pl as usize].cast(),
                    ref_stride,
                );
                ref_ptr = &mut *emu_edge_buf.offset((32 * 3 + 3) as isize) as *mut BD::Pixel;
                ref_stride = (32 as c_int as c_ulong)
                    .wrapping_mul(::core::mem::size_of::<BD::Pixel>() as c_ulong)
                    as ptrdiff_t;
            } else {
                ref_ptr = ((*refp).p.data.data[pl as usize] as *mut BD::Pixel)
                    .offset((BD::pxstride(ref_stride) * dy as isize) as isize)
                    .offset(dx as isize);
            }
            if !dst16.is_null() {
                ((*dsp).mc.warp8x8t)(
                    &mut *dst16.offset(x as isize),
                    dstride,
                    ref_ptr.cast(),
                    ref_stride,
                    (*wmp).abcd.get().as_ptr(),
                    mx,
                    my,
                    f.bitdepth_max,
                );
            } else {
                ((*dsp).mc.warp8x8)(
                    dst8.offset(x as isize).cast(),
                    dstride,
                    ref_ptr.cast(),
                    ref_stride,
                    (*wmp).abcd.get().as_ptr(),
                    mx,
                    my,
                    f.bitdepth_max,
                );
            }
            x += 8 as c_int;
        }
        if !dst8.is_null() {
            dst8 = dst8.offset(8 * BD::pxstride(dstride));
        } else {
            dst16 = dst16.offset((8 * dstride) as isize);
        }
        y += 8 as c_int;
    }
    return 0 as c_int;
}

pub(crate) unsafe fn rav1d_recon_b_intra<BD: BitDepth>(
    f: &Rav1dFrameData,
    t: &mut Rav1dTaskContext,
    bs: BlockSize,
    intra_edge_flags: EdgeFlags,
    b: &Av1Block,
) {
    let ts: *mut Rav1dTileState = t.ts;

    let dsp: *const Rav1dDSPContext = f.dsp;
    let bx4 = t.bx & 31;
    let by4 = t.by & 31;
    let ss_ver = (f.cur.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint) as c_int;
    let ss_hor = (f.cur.p.layout as c_uint != Rav1dPixelLayout::I444 as c_int as c_uint) as c_int;
    let cbx4 = bx4 >> ss_hor;
    let cby4 = by4 >> ss_ver;
    let b_dim: *const u8 = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bw4 = *b_dim.offset(0) as c_int;
    let bh4 = *b_dim.offset(1) as c_int;
    let w4 = cmp::min(bw4, f.bw - t.bx);
    let h4 = cmp::min(bh4, f.bh - t.by);
    let cw4 = w4 + ss_hor >> ss_hor;
    let ch4 = h4 + ss_ver >> ss_ver;
    let has_chroma = (f.cur.p.layout as c_uint != Rav1dPixelLayout::I400 as c_int as c_uint
        && (bw4 > ss_hor || t.bx & 1 != 0)
        && (bh4 > ss_ver || t.by & 1 != 0)) as c_int;
    let t_dim: *const TxfmInfo = &*dav1d_txfm_dimensions
        .as_ptr()
        .offset(b.c2rust_unnamed.c2rust_unnamed.tx as isize)
        as *const TxfmInfo;
    let uv_t_dim: *const TxfmInfo =
        &*dav1d_txfm_dimensions.as_ptr().offset(b.uvtx as isize) as *const TxfmInfo;
    let cbw4 = bw4 + ss_hor >> ss_hor;
    let cbh4 = bh4 + ss_ver >> ss_ver;
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    let intra_edge_filter_flag = seq_hdr.intra_edge_filter << 10;
    let mut init_y = 0;
    while init_y < h4 {
        let sub_h4 = cmp::min(h4, 16 + init_y);
        let sub_ch4 = cmp::min(ch4, init_y + 16 >> ss_ver);
        let mut init_x = 0;
        while init_x < w4 {
            if b.c2rust_unnamed.c2rust_unnamed.pal_sz[0] != 0 {
                let dst: *mut BD::Pixel = (f.cur.data.data[0] as *mut BD::Pixel).offset(
                    (4 * (t.by as isize * BD::pxstride(f.cur.stride[0]) + t.bx as isize)) as isize,
                );
                let pal_idx = if t.frame_thread.pass != 0 {
                    let p = t.frame_thread.pass & 1;
                    let frame_thread = &mut (*ts).frame_thread[p as usize];
                    let len = (bw4 * bh4 * 16) as usize;
                    let pal_idx = &f.frame_thread.pal_idx[frame_thread.pal_idx..][..len];
                    frame_thread.pal_idx += len;
                    pal_idx
                } else {
                    &t.scratch.c2rust_unnamed_0.pal_idx
                };
                let pal: *const u16 = if t.frame_thread.pass != 0 {
                    let index = (((t.by as isize >> 1) + (t.bx as isize & 1)) * (f.b4_stride >> 1)
                        + ((t.bx >> 1) + (t.by & 1)) as isize)
                        as isize;
                    f.frame_thread.pal[index as usize][0].as_ptr()
                } else {
                    (t.scratch.c2rust_unnamed_0.pal[0]).as_ptr()
                };
                (*f.dsp).ipred.pal_pred.call::<BD>(
                    dst,
                    f.cur.stride[0],
                    pal,
                    pal_idx.as_ptr(),
                    bw4 * 4,
                    bh4 * 4,
                );
                if debug_block_info!(f, t) && 0 != 0 {
                    hex_dump::<BD>(
                        dst,
                        BD::pxstride(f.cur.stride[0] as usize),
                        bw4 as usize * 4,
                        bh4 as usize * 4,
                        "y-pal-pred",
                    );
                }
            }
            let intra_flags = sm_flag(&*t.a, bx4 as usize)
                | sm_flag(&mut t.l, by4 as usize)
                | intra_edge_filter_flag;
            let sb_has_tr = if (init_x + 16) < w4 {
                true
            } else if init_y != 0 {
                false
            } else {
                intra_edge_flags.contains(EdgeFlags::I444_TOP_HAS_RIGHT)
            };
            let sb_has_bl = if init_x != 0 {
                false
            } else if (init_y + 16) < h4 {
                true
            } else {
                intra_edge_flags.contains(EdgeFlags::I444_LEFT_HAS_BOTTOM)
            };
            let mut y;
            let mut x;
            let sub_w4 = cmp::min(w4, init_x + 16);
            y = init_y;
            t.by += init_y;
            while y < sub_h4 {
                let mut dst: *mut BD::Pixel = (f.cur.data.data[0] as *mut BD::Pixel).offset(
                    (4 * (t.by as isize * BD::pxstride(f.cur.stride[0])
                        + t.bx as isize
                        + init_x as isize)) as isize,
                );
                x = init_x;
                t.bx += init_x;
                while x < sub_w4 {
                    let mut angle;
                    let edge_flags: EdgeFlags;
                    let m: IntraPredMode;
                    if !(b.c2rust_unnamed.c2rust_unnamed.pal_sz[0] != 0) {
                        angle = b.c2rust_unnamed.c2rust_unnamed.y_angle as c_int;
                        edge_flags = EdgeFlags::union_all([
                            EdgeFlags::I444_TOP_HAS_RIGHT.select(
                                !((y > init_y || !sb_has_tr) && x + (*t_dim).w as c_int >= sub_w4),
                            ),
                            EdgeFlags::I444_LEFT_HAS_BOTTOM.select(
                                !(x > init_x || (!sb_has_bl && y + (*t_dim).h as c_int >= sub_h4)),
                            ),
                        ]);
                        let top_sb_edge_slice = if t.by & f.sb_step - 1 == 0 {
                            let mut top_sb_edge: *const BD::Pixel =
                                f.ipred_edge[0] as *mut BD::Pixel;
                            let sby = t.by >> f.sb_shift;
                            top_sb_edge = top_sb_edge.offset((f.sb128w * 128 * (sby - 1)) as isize);
                            Some(slice::from_raw_parts(top_sb_edge, f.sb128w as usize * 128))
                        } else {
                            None
                        };
                        let interintra_edge =
                            BD::select_mut(&mut t.scratch.c2rust_unnamed_0.interintra_edge);
                        let edge_array = &mut interintra_edge.0.edge;
                        let edge_offset = 128;
                        let data_stride = BD::pxstride(f.cur.stride[0]);
                        let data_width = 4 * (*ts).tiling.col_end;
                        let data_height = 4 * (*ts).tiling.row_end;
                        let data_diff = (data_height - 1) as isize * data_stride;
                        let dst_slice = slice::from_raw_parts(
                            (f.cur.data.data[0] as *const BD::Pixel).offset(cmp::min(data_diff, 0)),
                            data_diff.unsigned_abs() + data_width as usize,
                        );
                        m = rav1d_prepare_intra_edges(
                            t.bx,
                            t.bx > (*ts).tiling.col_start,
                            t.by,
                            t.by > (*ts).tiling.row_start,
                            (*ts).tiling.col_end,
                            (*ts).tiling.row_end,
                            edge_flags,
                            dst_slice,
                            f.cur.stride[0],
                            top_sb_edge_slice,
                            b.c2rust_unnamed.c2rust_unnamed.y_mode as IntraPredMode,
                            &mut angle,
                            (*t_dim).w as c_int,
                            (*t_dim).h as c_int,
                            seq_hdr.intra_edge_filter,
                            edge_array,
                            edge_offset,
                            BD::from_c(f.bitdepth_max),
                        );
                        let edge = edge_array.as_ptr().add(edge_offset);
                        (*dsp).ipred.intra_pred[m as usize].call(
                            dst,
                            f.cur.stride[0],
                            edge,
                            (*t_dim).w as c_int * 4,
                            (*t_dim).h as c_int * 4,
                            angle | intra_flags,
                            4 * f.bw - 4 * t.bx,
                            4 * f.bh - 4 * t.by,
                            BD::from_c(f.bitdepth_max),
                        );
                        if debug_block_info!(f, t) && 0 != 0 {
                            hex_dump::<BD>(
                                edge.offset(-(((*t_dim).h as c_int * 4) as isize)),
                                (*t_dim).h as usize * 4,
                                (*t_dim).h as usize * 4,
                                2,
                                "l",
                            );
                            hex_dump::<BD>(edge, 0, 1, 1, "tl");
                            hex_dump::<BD>(
                                edge.offset(1),
                                (*t_dim).w as usize * 4,
                                (*t_dim).w as usize * 4,
                                2,
                                "t",
                            );
                            hex_dump::<BD>(
                                dst,
                                f.cur.stride[0] as usize,
                                (*t_dim).w as usize * 4,
                                (*t_dim).h as usize * 4,
                                "y-intra-pred",
                            );
                        }
                    }
                    if b.skip == 0 {
                        let cf: *mut BD::Coef;
                        let eob;
                        let mut txtp: TxfmType = DCT_DCT;
                        if t.frame_thread.pass != 0 {
                            let p = t.frame_thread.pass & 1;
                            cf = (*ts).frame_thread[p as usize].cf as *mut BD::Coef;
                            (*ts).frame_thread[p as usize].cf =
                                ((*ts).frame_thread[p as usize].cf as *mut BD::Coef).offset(
                                    (cmp::min((*t_dim).w as c_int, 8 as c_int)
                                        * cmp::min((*t_dim).h as c_int, 8 as c_int)
                                        * 16) as isize,
                                ) as *mut DynCoef;
                            let cbi = f.frame_thread.cbi
                                [(t.by as isize * f.b4_stride + t.bx as isize) as usize][0];
                            eob = cbi.eob().into();
                            txtp = cbi.txtp();
                        } else {
                            let mut cf_ctx: u8 = 0;
                            cf = BD::select_mut(&mut (*t).cf).0.as_mut_ptr();
                            eob = decode_coefs::<BD>(
                                f,
                                t,
                                &mut (*t.a).lcoef.0[(bx4 + x) as usize..],
                                &mut t.l.lcoef.0[(by4 + y) as usize..],
                                b.c2rust_unnamed.c2rust_unnamed.tx as RectTxfmSize,
                                bs,
                                b,
                                1 as c_int,
                                0 as c_int,
                                cf,
                                &mut txtp,
                                &mut cf_ctx,
                            );
                            if debug_block_info!(f, t) {
                                println!(
                                    "Post-y-cf-blk[tx={},txtp={},eob={}]: r={}",
                                    b.c2rust_unnamed.c2rust_unnamed.tx as c_int,
                                    txtp as c_uint,
                                    eob,
                                    (*ts).msac.rng,
                                );
                            }
                            CaseSet::<16, true>::many(
                                [&mut t.l, &mut *t.a],
                                [
                                    cmp::min((*t_dim).h as i32, f.bh - t.by) as usize,
                                    cmp::min((*t_dim).w as i32, f.bw - t.bx) as usize,
                                ],
                                [(by4 + y) as usize, (bx4 + x) as usize],
                                |case, dir| {
                                    case.set(&mut dir.lcoef.0, cf_ctx);
                                },
                            );
                        }
                        if eob >= 0 {
                            if debug_block_info!(f, t) && 0 != 0 {
                                coef_dump(
                                    cf,
                                    cmp::min((*t_dim).h as usize, 8) * 4,
                                    cmp::min((*t_dim).w as usize, 8) * 4,
                                    3,
                                    "dq",
                                );
                            }
                            ((*dsp).itx.itxfm_add[b.c2rust_unnamed.c2rust_unnamed.tx as usize]
                                [txtp as usize])
                                .expect("non-null function pointer")(
                                dst.cast(),
                                f.cur.stride[0],
                                cf.cast(),
                                eob,
                                f.bitdepth_max,
                            );
                            if debug_block_info!(f, t) && 0 != 0 {
                                hex_dump::<BD>(
                                    dst,
                                    f.cur.stride[0] as usize,
                                    (*t_dim).w as usize * 4,
                                    (*t_dim).h as usize * 4,
                                    "recon",
                                );
                            }
                        }
                    } else if t.frame_thread.pass == 0 {
                        CaseSet::<16, false>::many(
                            [&mut t.l, &mut *t.a],
                            [(*t_dim).h as usize, (*t_dim).w as usize],
                            [(by4 + y) as usize, (bx4 + x) as usize],
                            |case, dir| {
                                case.set(&mut dir.lcoef.0, 0x40);
                            },
                        );
                    }
                    dst = dst.offset((4 * (*t_dim).w as c_int) as isize);
                    x += (*t_dim).w as c_int;
                    t.bx += (*t_dim).w as c_int;
                }
                t.bx -= x;
                y += (*t_dim).h as c_int;
                t.by += (*t_dim).h as c_int;
            }
            t.by -= y;
            if !(has_chroma == 0) {
                let stride: ptrdiff_t = f.cur.stride[1];
                if b.c2rust_unnamed.c2rust_unnamed.uv_mode as c_int == CFL_PRED as c_int {
                    if !(init_x == 0 && init_y == 0) {
                        unreachable!();
                    }
                    let ac = &mut t.scratch.c2rust_unnamed_0.ac_txtp_map.ac;
                    let y_src: *mut BD::Pixel = (f.cur.data.data[0] as *mut BD::Pixel)
                        .offset((4 * (t.bx & !ss_hor)) as isize)
                        .offset(
                            ((4 * (t.by & !ss_ver)) as isize
                                * BD::pxstride(f.cur.stride[0] as usize) as isize)
                                as isize,
                        );
                    let uv_off: ptrdiff_t = 4
                        * ((t.bx >> ss_hor) as isize
                            + (t.by >> ss_ver) as isize * BD::pxstride(stride));
                    let uv_dst: [*mut BD::Pixel; 2] = [
                        (f.cur.data.data[1] as *mut BD::Pixel).offset(uv_off as isize),
                        (f.cur.data.data[2] as *mut BD::Pixel).offset(uv_off as isize),
                    ];
                    let furthest_r =
                        (cw4 << ss_hor) + (*t_dim).w as c_int - 1 & !((*t_dim).w as c_int - 1);
                    let furthest_b =
                        (ch4 << ss_ver) + (*t_dim).h as c_int - 1 & !((*t_dim).h as c_int - 1);
                    (*dsp).ipred.cfl_ac[f.cur.p.layout.try_into().unwrap()].call::<BD>(
                        ac.as_mut_ptr(),
                        y_src,
                        f.cur.stride[0],
                        cbw4 - (furthest_r >> ss_hor),
                        cbh4 - (furthest_b >> ss_ver),
                        cbw4 * 4,
                        cbh4 * 4,
                    );
                    let mut pl = 0;
                    while pl < 2 {
                        if !(b.c2rust_unnamed.c2rust_unnamed.cfl_alpha[pl as usize] == 0) {
                            let mut angle = 0;
                            let top_sb_edge_slice = if t.by & !ss_ver & f.sb_step - 1 == 0 {
                                let mut top_sb_edge: *const BD::Pixel =
                                    f.ipred_edge[(pl + 1) as usize] as *mut BD::Pixel;
                                let sby = t.by >> f.sb_shift;
                                top_sb_edge =
                                    top_sb_edge.offset((f.sb128w * 128 * (sby - 1)) as isize);
                                Some(slice::from_raw_parts(top_sb_edge, f.sb128w as usize * 128))
                            } else {
                                None
                            };
                            let xpos = t.bx >> ss_hor;
                            let ypos = t.by >> ss_ver;
                            let xstart = (*ts).tiling.col_start >> ss_hor;
                            let ystart = (*ts).tiling.row_start >> ss_ver;
                            let interintra_edge =
                                BD::select_mut(&mut t.scratch.c2rust_unnamed_0.interintra_edge);
                            let edge_array = &mut interintra_edge.0.edge;
                            let edge_offset = 128;
                            let data_stride = BD::pxstride(f.cur.stride[1]);
                            let data_width = 4 * (*ts).tiling.col_end >> ss_hor;
                            let data_height = 4 * (*ts).tiling.row_end >> ss_ver;
                            let data_diff = (data_height - 1) as isize * data_stride;
                            let uvdst_slice = slice::from_raw_parts(
                                (f.cur.data.data[1 + pl as usize] as *const BD::Pixel)
                                    .offset(cmp::min(data_diff, 0)),
                                data_diff.unsigned_abs() + data_width as usize,
                            );
                            let m: IntraPredMode = rav1d_prepare_intra_edges(
                                xpos,
                                xpos > xstart,
                                ypos,
                                ypos > ystart,
                                (*ts).tiling.col_end >> ss_hor,
                                (*ts).tiling.row_end >> ss_ver,
                                EdgeFlags::empty(),
                                uvdst_slice,
                                stride,
                                top_sb_edge_slice,
                                DC_PRED,
                                &mut angle,
                                (*uv_t_dim).w as c_int,
                                (*uv_t_dim).h as c_int,
                                0 as c_int,
                                edge_array,
                                edge_offset,
                                BD::from_c(f.bitdepth_max),
                            );
                            let edge = edge_array.as_ptr().add(edge_offset);
                            (*dsp).ipred.cfl_pred[m as usize].call(
                                uv_dst[pl as usize],
                                stride,
                                edge,
                                (*uv_t_dim).w as c_int * 4,
                                (*uv_t_dim).h as c_int * 4,
                                ac.as_mut_ptr(),
                                b.c2rust_unnamed.c2rust_unnamed.cfl_alpha[pl as usize] as c_int,
                                BD::from_c(f.bitdepth_max),
                            );
                        }
                        pl += 1;
                    }
                    if debug_block_info!(&*f, t) && 0 != 0 {
                        ac_dump(ac, 4 * cbw4 as usize, 4 * cbh4 as usize, "ac");
                        hex_dump::<BD>(
                            uv_dst[0],
                            stride as usize,
                            cbw4 as usize * 4,
                            cbh4 as usize * 4,
                            "u-cfl-pred",
                        );
                        hex_dump::<BD>(
                            uv_dst[1],
                            stride as usize,
                            cbw4 as usize * 4,
                            cbh4 as usize * 4,
                            "v-cfl-pred",
                        );
                    }
                } else if b.c2rust_unnamed.c2rust_unnamed.pal_sz[1] != 0 {
                    let uv_dstoff: ptrdiff_t = 4
                        * ((t.bx >> ss_hor) as isize
                            + (t.by >> ss_ver) as isize * BD::pxstride(f.cur.stride[1]));
                    let (pal, pal_idx) = if t.frame_thread.pass != 0 {
                        let p = t.frame_thread.pass & 1;
                        let index = (((t.by >> 1) + (t.bx & 1)) as isize * (f.b4_stride >> 1)
                            + ((t.bx as isize >> 1) as isize + (t.by as isize & 1)) as isize)
                            as isize;
                        let pal_idx_offset = &mut (*ts).frame_thread[p as usize].pal_idx;
                        let len = (cbw4 * cbh4 * 16) as usize;
                        let pal_idx = &f.frame_thread.pal_idx[*pal_idx_offset..][..len];
                        *pal_idx_offset += len;
                        (&f.frame_thread.pal[index as usize], pal_idx)
                    } else {
                        (
                            &t.scratch.c2rust_unnamed_0.pal,
                            &t.scratch.c2rust_unnamed_0.pal_idx[(bw4 * bh4 * 16) as usize..],
                        )
                    };
                    (*f.dsp).ipred.pal_pred.call::<BD>(
                        (f.cur.data.data[1] as *mut BD::Pixel).offset(uv_dstoff as isize),
                        f.cur.stride[1],
                        pal[1].as_ptr(),
                        pal_idx.as_ptr(),
                        cbw4 * 4,
                        cbh4 * 4,
                    );
                    (*f.dsp).ipred.pal_pred.call::<BD>(
                        (f.cur.data.data[2] as *mut BD::Pixel).offset(uv_dstoff as isize),
                        f.cur.stride[1],
                        pal[2].as_ptr(),
                        pal_idx.as_ptr(),
                        cbw4 * 4,
                        cbh4 * 4,
                    );
                    if debug_block_info!(f, t) && 0 != 0 {
                        hex_dump::<BD>(
                            (f.cur.data.data[1] as *mut BD::Pixel).offset(uv_dstoff as isize),
                            BD::pxstride(f.cur.stride[1] as usize),
                            cbw4 as usize * 4,
                            cbh4 as usize * 4,
                            "u-pal-pred",
                        );
                        hex_dump::<BD>(
                            (f.cur.data.data[2] as *mut BD::Pixel).offset(uv_dstoff as isize),
                            BD::pxstride(f.cur.stride[1] as usize),
                            cbw4 as usize * 4,
                            cbh4 as usize * 4,
                            "v-pal-pred",
                        );
                    }
                }
                let sm_uv_fl =
                    sm_uv_flag(&*t.a, cbx4 as usize) | sm_uv_flag(&mut t.l, cby4 as usize);
                let uv_sb_has_tr = if init_x + 16 >> ss_hor < cw4 {
                    true
                } else if init_y != 0 {
                    false
                } else {
                    intra_edge_flags.contains(EdgeFlags::I420_TOP_HAS_RIGHT >> f.cur.p.layout)
                };
                let uv_sb_has_bl = if init_x != 0 {
                    false
                } else if init_y + 16 >> ss_ver < ch4 {
                    true
                } else {
                    intra_edge_flags.contains(EdgeFlags::I420_LEFT_HAS_BOTTOM >> f.cur.p.layout)
                };
                let sub_cw4 = cmp::min(cw4, init_x + 16 >> ss_hor);
                let mut pl = 0;
                while pl < 2 {
                    y = init_y >> ss_ver;
                    t.by += init_y;
                    while y < sub_ch4 {
                        let mut dst: *mut BD::Pixel =
                            (f.cur.data.data[(1 + pl) as usize] as *mut BD::Pixel).offset(
                                (4 * ((t.by >> ss_ver) as isize * BD::pxstride(stride)
                                    + (t.bx + init_x >> ss_hor) as isize))
                                    as isize,
                            );
                        x = init_x >> ss_hor;
                        t.bx += init_x;
                        while x < sub_cw4 {
                            let mut angle;
                            let edge_flags: EdgeFlags;
                            let uv_mode: IntraPredMode;
                            let xpos;
                            let ypos;
                            let xstart;
                            let ystart;
                            let m: IntraPredMode;
                            if !(b.c2rust_unnamed.c2rust_unnamed.uv_mode as c_int
                                == CFL_PRED as c_int
                                && b.c2rust_unnamed.c2rust_unnamed.cfl_alpha[pl as usize] as c_int
                                    != 0
                                || b.c2rust_unnamed.c2rust_unnamed.pal_sz[1] as c_int != 0)
                            {
                                angle = b.c2rust_unnamed.c2rust_unnamed.uv_angle as c_int;
                                edge_flags = (if (y > init_y >> ss_ver || !uv_sb_has_tr)
                                    && x + (*uv_t_dim).w as c_int >= sub_cw4
                                {
                                    EdgeFlags::empty()
                                } else {
                                    EdgeFlags::I444_TOP_HAS_RIGHT
                                }) | (if x > init_x >> ss_hor
                                    || !uv_sb_has_bl && y + (*uv_t_dim).h as c_int >= sub_ch4
                                {
                                    EdgeFlags::empty()
                                } else {
                                    EdgeFlags::I444_LEFT_HAS_BOTTOM
                                });
                                let top_sb_edge_slice = if t.by & !ss_ver & f.sb_step - 1 == 0 {
                                    let mut top_sb_edge: *const BD::Pixel =
                                        f.ipred_edge[(1 + pl) as usize] as *const BD::Pixel;
                                    let sby = t.by >> f.sb_shift;
                                    top_sb_edge =
                                        top_sb_edge.offset((f.sb128w * 128 * (sby - 1)) as isize);
                                    Some(slice::from_raw_parts(
                                        top_sb_edge,
                                        f.sb128w as usize * 128,
                                    ))
                                } else {
                                    None
                                };
                                uv_mode = (if b.c2rust_unnamed.c2rust_unnamed.uv_mode as c_int
                                    == CFL_PRED as c_int
                                {
                                    DC_PRED as c_int
                                } else {
                                    b.c2rust_unnamed.c2rust_unnamed.uv_mode as c_int
                                }) as IntraPredMode;
                                xpos = t.bx >> ss_hor;
                                ypos = t.by >> ss_ver;
                                xstart = (*ts).tiling.col_start >> ss_hor;
                                ystart = (*ts).tiling.row_start >> ss_ver;
                                let interintra_edge =
                                    BD::select_mut(&mut t.scratch.c2rust_unnamed_0.interintra_edge);
                                let edge_array = &mut interintra_edge.0.edge;
                                let edge_offset = 128;
                                let data_stride = BD::pxstride(f.cur.stride[1]);
                                let data_width = 4 * (*ts).tiling.col_end >> ss_hor;
                                let data_height = 4 * (*ts).tiling.row_end >> ss_ver;
                                let data_diff = (data_height - 1) as isize * data_stride;
                                let dstuv_slice = slice::from_raw_parts(
                                    (f.cur.data.data[1 + pl as usize] as *const BD::Pixel)
                                        .offset(cmp::min(data_diff, 0)),
                                    data_diff.unsigned_abs() + data_width as usize,
                                );
                                m = rav1d_prepare_intra_edges(
                                    xpos,
                                    xpos > xstart,
                                    ypos,
                                    ypos > ystart,
                                    (*ts).tiling.col_end >> ss_hor,
                                    (*ts).tiling.row_end >> ss_ver,
                                    edge_flags,
                                    dstuv_slice,
                                    stride,
                                    top_sb_edge_slice,
                                    uv_mode,
                                    &mut angle,
                                    (*uv_t_dim).w as c_int,
                                    (*uv_t_dim).h as c_int,
                                    seq_hdr.intra_edge_filter,
                                    edge_array,
                                    edge_offset,
                                    BD::from_c(f.bitdepth_max),
                                );
                                angle |= intra_edge_filter_flag;
                                let edge = edge_array.as_ptr().add(edge_offset);
                                (*dsp).ipred.intra_pred[m as usize].call(
                                    dst,
                                    stride,
                                    edge,
                                    (*uv_t_dim).w as c_int * 4,
                                    (*uv_t_dim).h as c_int * 4,
                                    angle | sm_uv_fl,
                                    4 * f.bw + ss_hor - 4 * (t.bx & !ss_hor) >> ss_hor,
                                    4 * f.bh + ss_ver - 4 * (t.by & !ss_ver) >> ss_ver,
                                    BD::from_c(f.bitdepth_max),
                                );
                                if debug_block_info!(f, t) && 0 != 0 {
                                    hex_dump::<BD>(
                                        edge.offset(-(((*uv_t_dim).h as c_int * 4) as isize)),
                                        (*uv_t_dim).h as usize * 4,
                                        (*uv_t_dim).h as usize * 4,
                                        2,
                                        "l",
                                    );
                                    hex_dump::<BD>(edge, 0, 1, 1, "tl");
                                    hex_dump::<BD>(
                                        edge.offset(1),
                                        (*uv_t_dim).w as usize * 4,
                                        (*uv_t_dim).w as usize * 4,
                                        2,
                                        "t",
                                    );
                                    hex_dump::<BD>(
                                        dst,
                                        stride as usize,
                                        (*uv_t_dim).w as usize * 4,
                                        (*uv_t_dim).h as usize * 4,
                                        if pl != 0 {
                                            "v-intra-pred"
                                        } else {
                                            "u-intra-pred"
                                        },
                                    );
                                }
                            }
                            if b.skip == 0 {
                                let mut txtp: TxfmType = DCT_DCT;
                                let eob;
                                let cf: *mut BD::Coef;
                                if t.frame_thread.pass != 0 {
                                    let p = t.frame_thread.pass & 1;
                                    cf = (*ts).frame_thread[p as usize].cf as *mut BD::Coef;
                                    (*ts).frame_thread[p as usize].cf = ((*ts).frame_thread
                                        [p as usize]
                                        .cf
                                        as *mut BD::Coef)
                                        .offset(
                                            ((*uv_t_dim).w as c_int * (*uv_t_dim).h as c_int * 16)
                                                as isize,
                                        )
                                        as *mut DynCoef;
                                    let cbi = f.frame_thread.cbi
                                        [(t.by as isize * f.b4_stride + t.bx as isize) as usize]
                                        [(pl + 1) as usize];
                                    eob = cbi.eob().into();
                                    txtp = cbi.txtp();
                                } else {
                                    let mut cf_ctx: u8 = 0;
                                    cf = BD::select_mut(&mut (*t).cf).0.as_mut_ptr();
                                    eob = decode_coefs::<BD>(
                                        f,
                                        t,
                                        &mut (*t.a).ccoef.0[pl as usize][(cbx4 + x) as usize..],
                                        &mut t.l.ccoef.0[pl as usize][(cby4 + y) as usize..],
                                        b.uvtx as RectTxfmSize,
                                        bs,
                                        b,
                                        1 as c_int,
                                        1 + pl,
                                        cf,
                                        &mut txtp,
                                        &mut cf_ctx,
                                    );
                                    if debug_block_info!(f, t) {
                                        println!(
                                            "Post-uv-cf-blk[pl={},tx={},txtp={},eob={}]: r={} [x={},cbx4={}]",
                                            pl,
                                            b.uvtx as c_int,
                                            txtp as c_uint,
                                            eob,
                                            (*ts).msac.rng,
                                            x,
                                            cbx4,
                                        );
                                    }
                                    CaseSet::<16, true>::many(
                                        [&mut t.l, &mut *t.a],
                                        [
                                            cmp::min(
                                                (*uv_t_dim).h as i32,
                                                f.bh - t.by + ss_ver >> ss_ver,
                                            ) as usize,
                                            cmp::min(
                                                (*uv_t_dim).w as i32,
                                                f.bw - t.bx + ss_hor >> ss_hor,
                                            ) as usize,
                                        ],
                                        [(cby4 + y) as usize, (cbx4 + x) as usize],
                                        |case, dir| {
                                            case.set(&mut dir.ccoef.0[pl as usize], cf_ctx);
                                        },
                                    );
                                }
                                if eob >= 0 {
                                    if debug_block_info!(f, t) && 0 != 0 {
                                        coef_dump(
                                            cf,
                                            (*uv_t_dim).h as usize * 4,
                                            (*uv_t_dim).w as usize * 4,
                                            3,
                                            "dq",
                                        );
                                    }
                                    ((*dsp).itx.itxfm_add[b.uvtx as usize][txtp as usize])
                                        .expect("non-null function pointer")(
                                        dst.cast(),
                                        stride,
                                        cf.cast(),
                                        eob,
                                        f.bitdepth_max,
                                    );
                                    if debug_block_info!(f, t) && 0 != 0 {
                                        hex_dump::<BD>(
                                            dst,
                                            stride as usize,
                                            (*uv_t_dim).w as usize * 4,
                                            (*uv_t_dim).h as usize * 4,
                                            "recon",
                                        );
                                    }
                                }
                            } else if t.frame_thread.pass == 0 {
                                CaseSet::<16, false>::many(
                                    [&mut t.l, &mut *t.a],
                                    [(*uv_t_dim).h as usize, (*uv_t_dim).w as usize],
                                    [(cby4 + y) as usize, (cbx4 + x) as usize],
                                    |case, dir| {
                                        case.set(&mut dir.ccoef.0[pl as usize], 0x40);
                                    },
                                );
                            }
                            dst = dst.offset(((*uv_t_dim).w as c_int * 4) as isize);
                            x += (*uv_t_dim).w as c_int;
                            t.bx += ((*uv_t_dim).w as c_int) << ss_hor;
                        }
                        t.bx -= x << ss_hor;
                        y += (*uv_t_dim).h as c_int;
                        t.by += ((*uv_t_dim).h as c_int) << ss_ver;
                    }
                    t.by -= y << ss_ver;
                    pl += 1;
                }
            }
            init_x += 16 as c_int;
        }
        init_y += 16 as c_int;
    }
}

pub(crate) unsafe fn rav1d_recon_b_inter<BD: BitDepth>(
    f: &mut Rav1dFrameData,
    t: &mut Rav1dTaskContext,
    bs: BlockSize,
    b: &Av1Block,
) -> c_int {
    let ts: *mut Rav1dTileState = t.ts;
    let dsp: *const Rav1dDSPContext = f.dsp;
    let bx4 = t.bx & 31;
    let by4 = t.by & 31;
    let ss_ver = (f.cur.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint) as c_int;
    let ss_hor = (f.cur.p.layout as c_uint != Rav1dPixelLayout::I444 as c_int as c_uint) as c_int;
    let cbx4 = bx4 >> ss_hor;
    let cby4 = by4 >> ss_ver;
    let b_dim: *const u8 = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bw4 = *b_dim.offset(0) as c_int;
    let bh4 = *b_dim.offset(1) as c_int;
    let w4 = cmp::min(bw4, f.bw - t.bx);
    let h4 = cmp::min(bh4, f.bh - t.by);
    let has_chroma = (f.cur.p.layout as c_uint != Rav1dPixelLayout::I400 as c_int as c_uint
        && (bw4 > ss_hor || t.bx & 1 != 0)
        && (bh4 > ss_ver || t.by & 1 != 0)) as c_int;
    let chr_layout_idx = (if f.cur.p.layout as c_uint == Rav1dPixelLayout::I400 as c_int as c_uint {
        0 as c_int as c_uint
    } else {
        (Rav1dPixelLayout::I444 as c_int as c_uint).wrapping_sub(f.cur.p.layout as c_uint)
    }) as c_int;
    let mut res;
    let cbh4 = bh4 + ss_ver >> ss_ver;
    let cbw4 = bw4 + ss_hor >> ss_hor;
    let mut dst: *mut BD::Pixel = (f.cur.data.data[0] as *mut BD::Pixel)
        .offset((4 * (t.by as isize * BD::pxstride(f.cur.stride[0]) + t.bx as isize)) as isize);
    let uvdstoff: ptrdiff_t =
        4 * ((t.bx >> ss_hor) as isize + (t.by >> ss_ver) as isize * BD::pxstride(f.cur.stride[1]));
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    if frame_hdr.frame_type.is_key_or_intra() {
        if frame_hdr.size.super_res.enabled {
            unreachable!();
        }
        res = mc::<BD>(
            f,
            t,
            dst,
            0 as *mut i16,
            f.cur.stride[0],
            bw4,
            bh4,
            t.bx,
            t.by,
            0 as c_int,
            b.c2rust_unnamed
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c2rust_unnamed
                .mv[0],
            &f.sr_cur,
            0 as c_int,
            Filter2d::Bilinear,
        );
        if res != 0 {
            return res;
        }
        if has_chroma != 0 {
            let mut pl = 1;
            while pl < 3 {
                res = mc::<BD>(
                    f,
                    t,
                    (f.cur.data.data[pl as usize] as *mut BD::Pixel).offset(uvdstoff as isize),
                    0 as *mut i16,
                    f.cur.stride[1],
                    bw4 << (bw4 == ss_hor) as c_int,
                    bh4 << (bh4 == ss_ver) as c_int,
                    t.bx & !ss_hor,
                    t.by & !ss_ver,
                    pl,
                    b.c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0],
                    &f.sr_cur,
                    0 as c_int,
                    Filter2d::Bilinear,
                );
                if res != 0 {
                    return res;
                }
                pl += 1;
            }
        }
    } else if let Some(comp_inter_type) = b.c2rust_unnamed.c2rust_unnamed_0.comp_type {
        let filter_2d: Filter2d = b.c2rust_unnamed.c2rust_unnamed_0.filter2d;
        let tmp: *mut [i16; 16384] = (t
            .scratch
            .c2rust_unnamed
            .c2rust_unnamed
            .c2rust_unnamed
            .compinter)
            .as_mut_ptr();
        let mut jnt_weight = 0;
        let seg_mask: *mut u8 = (t
            .scratch
            .c2rust_unnamed
            .c2rust_unnamed
            .c2rust_unnamed
            .seg_mask)
            .as_mut_ptr();
        let mut mask: *const u8 = 0 as *const u8;
        let mut i = 0;
        while i < 2 {
            let refp: *const Rav1dThreadPicture = &*(f.refp).as_ptr().offset(
                *(b.c2rust_unnamed.c2rust_unnamed_0.r#ref)
                    .as_ptr()
                    .offset(i as isize) as isize,
            ) as *const Rav1dThreadPicture;
            if b.c2rust_unnamed.c2rust_unnamed_0.inter_mode as c_int == GLOBALMV_GLOBALMV as c_int
                && f.gmv_warp_allowed[b.c2rust_unnamed.c2rust_unnamed_0.r#ref[i as usize] as usize]
                    as c_int
                    != 0
            {
                res = warp_affine::<BD>(
                    f,
                    t,
                    0 as *mut BD::Pixel,
                    (*tmp.offset(i as isize)).as_mut_ptr(),
                    (bw4 * 4) as ptrdiff_t,
                    b_dim,
                    0 as c_int,
                    refp,
                    &frame_hdr.gmv[b.c2rust_unnamed.c2rust_unnamed_0.r#ref[i as usize] as usize],
                );
                if res != 0 {
                    return res;
                }
            } else {
                res = mc::<BD>(
                    f,
                    t,
                    0 as *mut BD::Pixel,
                    (*tmp.offset(i as isize)).as_mut_ptr(),
                    0 as c_int as ptrdiff_t,
                    bw4,
                    bh4,
                    t.bx,
                    t.by,
                    0 as c_int,
                    b.c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[i as usize],
                    refp,
                    b.c2rust_unnamed.c2rust_unnamed_0.r#ref[i as usize] as c_int,
                    filter_2d,
                );
                if res != 0 {
                    return res;
                }
            }
            i += 1;
        }

        match comp_inter_type {
            CompInterType::Avg => {
                ((*dsp).mc.avg)(
                    dst.cast(),
                    f.cur.stride[0],
                    (*tmp.offset(0)).as_mut_ptr(),
                    (*tmp.offset(1)).as_mut_ptr(),
                    bw4 * 4,
                    bh4 * 4,
                    f.bitdepth_max,
                );
            }
            CompInterType::WeightedAvg => {
                jnt_weight = f.jnt_weights[b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as usize]
                    [b.c2rust_unnamed.c2rust_unnamed_0.r#ref[1] as usize]
                    as c_int;
                ((*dsp).mc.w_avg)(
                    dst.cast(),
                    f.cur.stride[0],
                    (*tmp.offset(0)).as_mut_ptr(),
                    (*tmp.offset(1)).as_mut_ptr(),
                    bw4 * 4,
                    bh4 * 4,
                    jnt_weight,
                    f.bitdepth_max,
                );
            }
            CompInterType::Seg => {
                (*dsp).mc.w_mask[chr_layout_idx as usize](
                    dst.cast(),
                    f.cur.stride[0],
                    (*tmp.offset(
                        b.c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mask_sign as isize,
                    ))
                    .as_mut_ptr(),
                    (*tmp.offset(
                        (b.c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mask_sign
                            == 0) as c_int as isize,
                    ))
                    .as_mut_ptr(),
                    bw4 * 4,
                    bh4 * 4,
                    seg_mask,
                    b.c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mask_sign as c_int,
                    f.bitdepth_max,
                );
                mask = seg_mask;
            }
            CompInterType::Wedge => {
                mask = dav1d_wedge_masks[bs as usize][0][0][b
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .wedge_idx
                    as usize]
                    .as_ptr();
                ((*dsp).mc.mask)(
                    dst.cast(),
                    f.cur.stride[0],
                    (*tmp.offset(
                        b.c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mask_sign as isize,
                    ))
                    .as_mut_ptr(),
                    (*tmp.offset(
                        (b.c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mask_sign
                            == 0) as c_int as isize,
                    ))
                    .as_mut_ptr(),
                    bw4 * 4,
                    bh4 * 4,
                    mask,
                    f.bitdepth_max,
                );
                if has_chroma != 0 {
                    mask = dav1d_wedge_masks[bs as usize][chr_layout_idx as usize][b
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mask_sign
                        as usize][b
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .wedge_idx as usize]
                        .as_ptr();
                }
            }
        }
        if has_chroma != 0 {
            let mut pl = 0;
            while pl < 2 {
                let mut i = 0;
                while i < 2 {
                    let refp: *const Rav1dThreadPicture = &*(f.refp).as_ptr().offset(
                        *(b.c2rust_unnamed.c2rust_unnamed_0.r#ref)
                            .as_ptr()
                            .offset(i as isize) as isize,
                    )
                        as *const Rav1dThreadPicture;
                    if b.c2rust_unnamed.c2rust_unnamed_0.inter_mode as c_int
                        == GLOBALMV_GLOBALMV as c_int
                        && cmp::min(cbw4, cbh4) > 1
                        && f.gmv_warp_allowed
                            [b.c2rust_unnamed.c2rust_unnamed_0.r#ref[i as usize] as usize]
                            as c_int
                            != 0
                    {
                        res = warp_affine::<BD>(
                            f,
                            t,
                            0 as *mut BD::Pixel,
                            (*tmp.offset(i as isize)).as_mut_ptr(),
                            (bw4 * 4 >> ss_hor) as ptrdiff_t,
                            b_dim,
                            1 + pl,
                            refp,
                            &frame_hdr.gmv
                                [b.c2rust_unnamed.c2rust_unnamed_0.r#ref[i as usize] as usize],
                        );
                        if res != 0 {
                            return res;
                        }
                    } else {
                        res = mc::<BD>(
                            f,
                            t,
                            0 as *mut BD::Pixel,
                            (*tmp.offset(i as isize)).as_mut_ptr(),
                            0 as c_int as ptrdiff_t,
                            bw4,
                            bh4,
                            t.bx,
                            t.by,
                            1 + pl,
                            b.c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .mv[i as usize],
                            refp,
                            b.c2rust_unnamed.c2rust_unnamed_0.r#ref[i as usize] as c_int,
                            filter_2d,
                        );
                        if res != 0 {
                            return res;
                        }
                    }
                    i += 1;
                }

                let uvdst: *mut BD::Pixel = (f.cur.data.data[(1 + pl) as usize] as *mut BD::Pixel)
                    .offset(uvdstoff as isize);
                match comp_inter_type {
                    CompInterType::Avg => {
                        ((*dsp).mc.avg)(
                            uvdst.cast(),
                            f.cur.stride[1],
                            (*tmp.offset(0)).as_mut_ptr(),
                            (*tmp.offset(1)).as_mut_ptr(),
                            bw4 * 4 >> ss_hor,
                            bh4 * 4 >> ss_ver,
                            f.bitdepth_max,
                        );
                    }
                    CompInterType::WeightedAvg => {
                        ((*dsp).mc.w_avg)(
                            uvdst.cast(),
                            f.cur.stride[1],
                            (*tmp.offset(0)).as_mut_ptr(),
                            (*tmp.offset(1)).as_mut_ptr(),
                            bw4 * 4 >> ss_hor,
                            bh4 * 4 >> ss_ver,
                            jnt_weight,
                            f.bitdepth_max,
                        );
                    }
                    CompInterType::Seg | CompInterType::Wedge => {
                        ((*dsp).mc.mask)(
                            uvdst.cast(),
                            f.cur.stride[1],
                            (*tmp.offset(
                                b.c2rust_unnamed
                                    .c2rust_unnamed_0
                                    .c2rust_unnamed
                                    .c2rust_unnamed
                                    .mask_sign as isize,
                            ))
                            .as_mut_ptr(),
                            (*tmp.offset(
                                (b.c2rust_unnamed
                                    .c2rust_unnamed_0
                                    .c2rust_unnamed
                                    .c2rust_unnamed
                                    .mask_sign
                                    == 0) as c_int as isize,
                            ))
                            .as_mut_ptr(),
                            bw4 * 4 >> ss_hor,
                            bh4 * 4 >> ss_ver,
                            mask,
                            f.bitdepth_max,
                        );
                    }
                }
                pl += 1;
            }
        }
    } else {
        let mut is_sub8x8;
        let mut r: *const *mut refmvs_block;
        let refp: *const Rav1dThreadPicture = &*(f.refp)
            .as_ptr()
            .offset(*(b.c2rust_unnamed.c2rust_unnamed_0.r#ref).as_ptr().offset(0) as isize)
            as *const Rav1dThreadPicture;
        let filter_2d: Filter2d = b.c2rust_unnamed.c2rust_unnamed_0.filter2d as Filter2d;
        if cmp::min(bw4, bh4) > 1
            && (b.c2rust_unnamed.c2rust_unnamed_0.inter_mode as c_int == GLOBALMV as c_int
                && f.gmv_warp_allowed[b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as usize] as c_int
                    != 0
                || b.c2rust_unnamed.c2rust_unnamed_0.motion_mode == MotionMode::Warp
                    && t.warpmv.r#type > Rav1dWarpedMotionType::Translation)
        {
            res = warp_affine::<BD>(
                f,
                t,
                dst,
                0 as *mut i16,
                f.cur.stride[0],
                b_dim,
                0 as c_int,
                refp,
                if b.c2rust_unnamed.c2rust_unnamed_0.motion_mode == MotionMode::Warp {
                    &t.warpmv
                } else {
                    &frame_hdr.gmv[b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as usize]
                },
            );
            if res != 0 {
                return res;
            }
        } else {
            res = mc::<BD>(
                f,
                t,
                dst,
                0 as *mut i16,
                f.cur.stride[0],
                bw4,
                bh4,
                t.bx,
                t.by,
                0 as c_int,
                b.c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .mv[0],
                refp,
                b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as c_int,
                filter_2d,
            );
            if res != 0 {
                return res;
            }
            if b.c2rust_unnamed.c2rust_unnamed_0.motion_mode == MotionMode::Obmc {
                res = obmc::<BD>(
                    f,
                    t,
                    dst,
                    (*f).cur.stride[0],
                    b_dim,
                    0 as c_int,
                    bx4,
                    by4,
                    w4,
                    h4,
                );
                if res != 0 {
                    return res;
                }
            }
        }
        if let Some(interintra_type) = b.c2rust_unnamed.c2rust_unnamed_0.interintra_type {
            let interintra_edge = BD::select_mut(&mut t.scratch.c2rust_unnamed_0.interintra_edge);
            let tl_edge_array = &mut interintra_edge.0.edge;
            let tl_edge_offset = 32;
            let mut m: IntraPredMode = (if b
                .c2rust_unnamed
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c2rust_unnamed
                .interintra_mode
                == InterIntraPredMode::Smooth
            {
                SMOOTH_PRED as c_int
            } else {
                b.c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .interintra_mode as c_int
            }) as IntraPredMode;
            let mut angle = 0;
            let top_sb_edge_slice = if t.by & f.sb_step - 1 == 0 {
                let mut top_sb_edge: *const BD::Pixel = f.ipred_edge[0] as *const BD::Pixel;
                let sby = t.by >> f.sb_shift;
                top_sb_edge = top_sb_edge.offset((f.sb128w * 128 * (sby - 1)) as isize);
                Some(slice::from_raw_parts(top_sb_edge, f.sb128w as usize * 128))
            } else {
                None
            };
            let data_stride = BD::pxstride(f.cur.stride[0]);
            let data_width = 4 * (*ts).tiling.col_end;
            let data_height = 4 * (*ts).tiling.row_end;
            let data_diff = (data_height - 1) as isize * data_stride;
            let dst_slice = slice::from_raw_parts(
                (f.cur.data.data[0] as *const BD::Pixel).offset(cmp::min(data_diff, 0)),
                data_diff.unsigned_abs() + data_width as usize,
            );
            m = rav1d_prepare_intra_edges(
                t.bx,
                t.bx > (*ts).tiling.col_start,
                t.by,
                t.by > (*ts).tiling.row_start,
                (*ts).tiling.col_end,
                (*ts).tiling.row_end,
                EdgeFlags::empty(),
                dst_slice,
                f.cur.stride[0],
                top_sb_edge_slice,
                m,
                &mut angle,
                bw4,
                bh4,
                0 as c_int,
                tl_edge_array,
                tl_edge_offset,
                BD::from_c(f.bitdepth_max),
            );
            let tl_edge = tl_edge_array[tl_edge_offset..].as_ptr();
            let tmp = interintra_edge.0.interintra.as_mut_ptr();
            (*dsp).ipred.intra_pred[m as usize].call(
                tmp,
                ((4 * bw4) as c_ulong).wrapping_mul(::core::mem::size_of::<BD::Pixel>() as c_ulong)
                    as ptrdiff_t,
                tl_edge,
                bw4 * 4,
                bh4 * 4,
                0 as c_int,
                0 as c_int,
                0 as c_int,
                BD::from_c(f.bitdepth_max),
            );
            let ii_mask = match interintra_type {
                InterIntraType::Blend => {
                    dav1d_ii_masks[bs as usize][0][b
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .interintra_mode
                        as usize]
                }
                InterIntraType::Wedge => {
                    dav1d_wedge_masks[bs as usize][0][0][b
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .wedge_idx
                        as usize]
                }
            };
            ((*dsp).mc.blend)(
                dst.cast(),
                f.cur.stride[0],
                tmp.cast(),
                bw4 * 4,
                bh4 * 4,
                ii_mask.as_ptr(),
            );
        }
        if !(has_chroma == 0) {
            is_sub8x8 = (bw4 == ss_hor || bh4 == ss_ver) as c_int;
            r = 0 as *const *mut refmvs_block;
            if is_sub8x8 != 0 {
                if !(ss_hor == 1) {
                    unreachable!();
                }
                r = &mut *(t.rt.r).as_mut_ptr().offset(((t.by & 31) + 5) as isize)
                    as *mut *mut refmvs_block;
                if bw4 == 1 {
                    is_sub8x8 &= ((*(*r.offset(0)).offset((t.bx - 1) as isize)).0.r#ref.r#ref[0]
                        as c_int
                        > 0) as c_int;
                }
                if bh4 == ss_ver {
                    is_sub8x8 &= ((*(*r.offset(-(1 as c_int) as isize)).offset(t.bx as isize))
                        .0
                        .r#ref
                        .r#ref[0] as c_int
                        > 0) as c_int;
                }
                if bw4 == 1 && bh4 == ss_ver {
                    is_sub8x8 &= ((*(*r.offset(-(1 as c_int) as isize)).offset((t.bx - 1) as isize))
                        .0
                        .r#ref
                        .r#ref[0] as c_int
                        > 0) as c_int;
                }
            }
            if is_sub8x8 != 0 {
                if !(ss_hor == 1) {
                    unreachable!();
                }
                let mut h_off: ptrdiff_t = 0 as c_int as ptrdiff_t;
                let mut v_off: ptrdiff_t = 0 as c_int as ptrdiff_t;
                if bw4 == 1 && bh4 == ss_ver {
                    let mut pl = 0;
                    while pl < 2 {
                        res = mc::<BD>(
                            f,
                            t,
                            (f.cur.data.data[(1 + pl) as usize] as *mut BD::Pixel)
                                .offset(uvdstoff as isize),
                            0 as *mut i16,
                            f.cur.stride[1],
                            bw4,
                            bh4,
                            t.bx - 1,
                            t.by - 1,
                            1 + pl,
                            (*(*r.offset(-(1 as c_int) as isize)).offset((t.bx - 1) as isize))
                                .0
                                .mv
                                .mv[0],
                            &*(f.refp).as_ptr().offset(
                                (*((*(*r.offset(-(1 as c_int) as isize))
                                    .offset((t.bx - 1) as isize))
                                .0
                                .r#ref
                                .r#ref)
                                    .as_mut_ptr()
                                    .offset(0) as c_int
                                    - 1) as isize,
                            ),
                            (*(*r.offset(-(1 as c_int) as isize)).offset((t.bx - 1) as isize))
                                .0
                                .r#ref
                                .r#ref[0] as c_int
                                - 1,
                            if t.frame_thread.pass != 2 {
                                t.tl_4x4_filter
                            } else {
                                f.frame_thread.b[((t.by - 1) as isize * f.b4_stride + t.bx as isize
                                    - 1) as usize]
                                    .c2rust_unnamed
                                    .c2rust_unnamed_0
                                    .filter2d
                            },
                        );
                        if res != 0 {
                            return res;
                        }
                        pl += 1;
                    }
                    v_off = 2 * BD::pxstride(f.cur.stride[1]);
                    h_off = 2 as c_int as ptrdiff_t;
                }
                if bw4 == 1 {
                    let left_filter_2d: Filter2d = dav1d_filter_2d
                        [t.l.filter[1][by4 as usize] as usize]
                        [t.l.filter[0][by4 as usize] as usize]
                        as Filter2d;
                    let mut pl = 0;
                    while pl < 2 {
                        res = mc::<BD>(
                            f,
                            t,
                            (f.cur.data.data[(1 + pl) as usize] as *mut BD::Pixel)
                                .offset(uvdstoff as isize)
                                .offset(v_off as isize),
                            0 as *mut i16,
                            f.cur.stride[1],
                            bw4,
                            bh4,
                            t.bx - 1,
                            t.by,
                            1 + pl,
                            (*(*r.offset(0)).offset((t.bx - 1) as isize)).0.mv.mv[0],
                            &*(f.refp).as_ptr().offset(
                                (*((*(*r.offset(0)).offset((t.bx - 1) as isize)).0.r#ref.r#ref)
                                    .as_mut_ptr()
                                    .offset(0) as c_int
                                    - 1) as isize,
                            ),
                            (*(*r.offset(0)).offset((t.bx - 1) as isize)).0.r#ref.r#ref[0] as c_int
                                - 1,
                            if t.frame_thread.pass != 2 as c_int {
                                left_filter_2d
                            } else {
                                f.frame_thread.b
                                    [(t.by as isize * f.b4_stride + t.bx as isize - 1) as usize]
                                    .c2rust_unnamed
                                    .c2rust_unnamed_0
                                    .filter2d
                            },
                        );
                        if res != 0 {
                            return res;
                        }
                        pl += 1;
                    }
                    h_off = 2 as c_int as ptrdiff_t;
                }
                if bh4 == ss_ver {
                    let top_filter_2d: Filter2d = dav1d_filter_2d
                        [(*t.a).filter[1][bx4 as usize] as usize]
                        [(*t.a).filter[0][bx4 as usize] as usize]
                        as Filter2d;
                    let mut pl = 0;
                    while pl < 2 {
                        res = mc::<BD>(
                            f,
                            t,
                            (f.cur.data.data[(1 + pl) as usize] as *mut BD::Pixel)
                                .offset(uvdstoff as isize)
                                .offset(h_off as isize),
                            0 as *mut i16,
                            f.cur.stride[1],
                            bw4,
                            bh4,
                            t.bx,
                            t.by - 1,
                            1 + pl,
                            (*(*r.offset(-(1 as c_int) as isize)).offset(t.bx as isize))
                                .0
                                .mv
                                .mv[0],
                            &*(f.refp).as_ptr().offset(
                                (*((*(*r.offset(-(1 as c_int) as isize)).offset(t.bx as isize))
                                    .0
                                    .r#ref
                                    .r#ref)
                                    .as_mut_ptr()
                                    .offset(0) as c_int
                                    - 1) as isize,
                            ),
                            (*(*r.offset(-(1 as c_int) as isize)).offset(t.bx as isize))
                                .0
                                .r#ref
                                .r#ref[0] as c_int
                                - 1,
                            if t.frame_thread.pass != 2 as c_int {
                                top_filter_2d
                            } else {
                                f.frame_thread.b
                                    [((t.by - 1) as isize * f.b4_stride + t.bx as isize) as usize]
                                    .c2rust_unnamed
                                    .c2rust_unnamed_0
                                    .filter2d
                            },
                        );
                        if res != 0 {
                            return res;
                        }
                        pl += 1;
                    }
                    v_off = 2 * BD::pxstride(f.cur.stride[1]);
                }
                let mut pl = 0;
                while pl < 2 {
                    res = mc::<BD>(
                        f,
                        t,
                        (f.cur.data.data[(1 + pl) as usize] as *mut BD::Pixel)
                            .offset(uvdstoff as isize)
                            .offset(h_off as isize)
                            .offset(v_off as isize),
                        0 as *mut i16,
                        f.cur.stride[1],
                        bw4,
                        bh4,
                        t.bx,
                        t.by,
                        1 + pl,
                        b.c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mv[0],
                        refp,
                        b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as c_int,
                        filter_2d,
                    );
                    if res != 0 {
                        return res;
                    }
                    pl += 1;
                }
            } else {
                if cmp::min(cbw4, cbh4) > 1
                    && (b.c2rust_unnamed.c2rust_unnamed_0.inter_mode as c_int == GLOBALMV as c_int
                        && f.gmv_warp_allowed[b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as usize]
                            as c_int
                            != 0
                        || b.c2rust_unnamed.c2rust_unnamed_0.motion_mode == MotionMode::Warp
                            && t.warpmv.r#type > Rav1dWarpedMotionType::Translation)
                {
                    let mut pl = 0;
                    while pl < 2 {
                        res = warp_affine::<BD>(
                            f,
                            t,
                            (f.cur.data.data[(1 + pl) as usize] as *mut BD::Pixel)
                                .offset(uvdstoff as isize),
                            0 as *mut i16,
                            f.cur.stride[1],
                            b_dim,
                            1 + pl,
                            refp,
                            if b.c2rust_unnamed.c2rust_unnamed_0.motion_mode == MotionMode::Warp {
                                &t.warpmv
                            } else {
                                &frame_hdr.gmv[b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as usize]
                            },
                        );
                        if res != 0 {
                            return res;
                        }
                        pl += 1;
                    }
                } else {
                    let mut pl = 0;
                    while pl < 2 {
                        res = mc::<BD>(
                            f,
                            t,
                            (f.cur.data.data[(1 + pl) as usize] as *mut BD::Pixel)
                                .offset(uvdstoff as isize),
                            0 as *mut i16,
                            f.cur.stride[1],
                            bw4 << (bw4 == ss_hor) as c_int,
                            bh4 << (bh4 == ss_ver) as c_int,
                            t.bx & !ss_hor,
                            t.by & !ss_ver,
                            1 + pl,
                            b.c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .mv[0],
                            refp,
                            b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as c_int,
                            filter_2d,
                        );
                        if res != 0 {
                            return res;
                        }
                        if b.c2rust_unnamed.c2rust_unnamed_0.motion_mode == MotionMode::Obmc {
                            res = obmc::<BD>(
                                f,
                                t,
                                (f.cur.data.data[(1 + pl) as usize] as *mut BD::Pixel)
                                    .offset(uvdstoff as isize),
                                f.cur.stride[1],
                                b_dim,
                                1 + pl,
                                bx4,
                                by4,
                                w4,
                                h4,
                            );
                            if res != 0 {
                                return res;
                            }
                        }
                        pl += 1;
                    }
                }
                if let Some(interintra_type) = b.c2rust_unnamed.c2rust_unnamed_0.interintra_type {
                    let ii_mask = match interintra_type {
                        InterIntraType::Blend => {
                            dav1d_ii_masks[bs as usize][chr_layout_idx as usize][b
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .interintra_mode
                                as usize]
                        }
                        InterIntraType::Wedge => {
                            dav1d_wedge_masks[bs as usize][chr_layout_idx as usize][0][b
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .wedge_idx
                                as usize]
                        }
                    };
                    let mut pl = 0;
                    while pl < 2 {
                        let interintra_edge =
                            BD::select_mut(&mut t.scratch.c2rust_unnamed_0.interintra_edge);
                        let tl_edge_array = &mut interintra_edge.0.edge;
                        let tl_edge_offset = 32;
                        let mut m: IntraPredMode = (if b
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .interintra_mode
                            == InterIntraPredMode::Smooth
                        {
                            SMOOTH_PRED as c_int
                        } else {
                            b.c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .interintra_mode as c_int
                        }) as IntraPredMode;
                        let mut angle = 0;
                        let uvdst: *mut BD::Pixel = (f.cur.data.data[(1 + pl) as usize]
                            as *mut BD::Pixel)
                            .offset(uvdstoff as isize);
                        let top_sb_edge_slice = if t.by & f.sb_step - 1 == 0 {
                            let mut top_sb_edge: *const BD::Pixel =
                                f.ipred_edge[(pl + 1) as usize] as *const BD::Pixel;
                            let sby = t.by >> f.sb_shift;
                            top_sb_edge = top_sb_edge.offset((f.sb128w * 128 * (sby - 1)) as isize);
                            Some(slice::from_raw_parts(top_sb_edge, f.sb128w as usize * 128))
                        } else {
                            None
                        };
                        let data_stride = BD::pxstride(f.cur.stride[1]);
                        let data_width = 4 * (*ts).tiling.col_end >> ss_hor;
                        let data_height = 4 * (*ts).tiling.row_end >> ss_ver;
                        let data_diff = (data_height - 1) as isize * data_stride;
                        let dstuv_slice = slice::from_raw_parts(
                            (f.cur.data.data[1 + pl as usize] as *const BD::Pixel)
                                .offset(cmp::min(data_diff, 0)),
                            data_diff.unsigned_abs() + data_width as usize,
                        );
                        m = rav1d_prepare_intra_edges(
                            t.bx >> ss_hor,
                            t.bx >> ss_hor > (*ts).tiling.col_start >> ss_hor,
                            t.by >> ss_ver,
                            t.by >> ss_ver > (*ts).tiling.row_start >> ss_ver,
                            (*ts).tiling.col_end >> ss_hor,
                            (*ts).tiling.row_end >> ss_ver,
                            EdgeFlags::empty(),
                            dstuv_slice,
                            f.cur.stride[1],
                            top_sb_edge_slice,
                            m,
                            &mut angle,
                            cbw4,
                            cbh4,
                            0 as c_int,
                            tl_edge_array,
                            tl_edge_offset,
                            BD::from_c(f.bitdepth_max),
                        );
                        let tl_edge = tl_edge_array[tl_edge_offset..].as_ptr();
                        let tmp = interintra_edge.0.interintra.as_mut_ptr();
                        (*dsp).ipred.intra_pred[m as usize].call(
                            tmp,
                            ((cbw4 * 4) as c_ulong)
                                .wrapping_mul(::core::mem::size_of::<BD::Pixel>() as c_ulong)
                                as ptrdiff_t,
                            tl_edge,
                            cbw4 * 4,
                            cbh4 * 4,
                            0 as c_int,
                            0 as c_int,
                            0 as c_int,
                            BD::from_c(f.bitdepth_max),
                        );
                        ((*dsp).mc.blend)(
                            uvdst.cast(),
                            f.cur.stride[1],
                            tmp.cast(),
                            cbw4 * 4,
                            cbh4 * 4,
                            ii_mask.as_ptr(),
                        );
                        pl += 1;
                    }
                }
            }
        }
        t.tl_4x4_filter = filter_2d;
    }

    if debug_block_info!(f, t) && 0 != 0 {
        hex_dump::<BD>(
            dst,
            f.cur.stride[0] as usize,
            *b_dim.offset(0) as usize * 4,
            *b_dim.offset(1) as usize * 4,
            "y-pred",
        );
        if has_chroma != 0 {
            hex_dump::<BD>(
                &mut *(*(f.cur.data.data).as_ptr().offset(1) as *mut BD::Pixel)
                    .offset(uvdstoff as isize),
                f.cur.stride[1] as usize,
                cbw4 as usize * 4,
                cbh4 as usize * 4,
                "u-pred",
            );
            hex_dump::<BD>(
                &mut *(*(f.cur.data.data).as_ptr().offset(2) as *mut BD::Pixel)
                    .offset(uvdstoff as isize),
                f.cur.stride[1] as usize,
                cbw4 as usize * 4,
                cbh4 as usize * 4,
                "v-pred",
            );
        }
    }
    let cw4 = w4 + ss_hor >> ss_hor;
    let ch4 = h4 + ss_ver >> ss_ver;
    if b.skip != 0 {
        CaseSet::<32, false>::many(
            [&mut t.l, &mut *t.a],
            [bh4 as usize, bw4 as usize],
            [by4 as usize, bx4 as usize],
            |case, dir| {
                case.set(&mut dir.lcoef.0, 0x40);
            },
        );
        if has_chroma != 0 {
            CaseSet::<32, false>::many(
                [&mut t.l, &mut *t.a],
                [cbh4 as usize, cbw4 as usize],
                [cby4 as usize, cbx4 as usize],
                |case, dir| {
                    case.set(&mut dir.ccoef.0[0], 0x40);
                    case.set(&mut dir.ccoef.0[1], 0x40);
                },
            );
        }
        return 0 as c_int;
    }
    let uvtx: *const TxfmInfo =
        &*dav1d_txfm_dimensions.as_ptr().offset(b.uvtx as isize) as *const TxfmInfo;
    let ytx: *const TxfmInfo = &*dav1d_txfm_dimensions
        .as_ptr()
        .offset(b.c2rust_unnamed.c2rust_unnamed_0.max_ytx as isize)
        as *const TxfmInfo;
    let tx_split: [u16; 2] = [
        b.c2rust_unnamed.c2rust_unnamed_0.tx_split0 as u16,
        b.c2rust_unnamed.c2rust_unnamed_0.tx_split1,
    ];
    let mut init_y = 0;
    while init_y < bh4 {
        let mut init_x = 0;
        while init_x < bw4 {
            let mut y_off = (init_y != 0) as c_int;
            let mut y;
            dst = dst.offset((BD::pxstride(f.cur.stride[0]) * 4 * init_y as isize) as isize);
            y = init_y;
            t.by += init_y;
            while y < cmp::min(h4, init_y + 16) {
                let mut x;
                let mut x_off = (init_x != 0) as c_int;
                x = init_x;
                t.bx += init_x;
                while x < cmp::min(w4, init_x + 16) {
                    read_coef_tree::<BD>(
                        f,
                        t,
                        bs,
                        b,
                        b.c2rust_unnamed.c2rust_unnamed_0.max_ytx as RectTxfmSize,
                        0 as c_int,
                        tx_split.as_ptr(),
                        x_off,
                        y_off,
                        &mut *dst.offset((x * 4) as isize),
                    );
                    t.bx += (*ytx).w as c_int;
                    x += (*ytx).w as c_int;
                    x_off += 1;
                }
                dst = dst.offset((BD::pxstride(f.cur.stride[0]) * 4 * (*ytx).h as isize) as isize);
                t.bx -= x;
                t.by += (*ytx).h as c_int;
                y += (*ytx).h as c_int;
                y_off += 1;
            }
            dst = dst.offset(-((BD::pxstride(f.cur.stride[0]) * 4 * y as isize) as isize));
            t.by -= y;
            if has_chroma != 0 {
                let mut pl = 0;
                while pl < 2 {
                    let mut uvdst: *mut BD::Pixel = (f.cur.data.data[(1 + pl) as usize]
                        as *mut BD::Pixel)
                        .offset(uvdstoff as isize)
                        .offset(
                            (BD::pxstride(f.cur.stride[1]) * init_y as isize * 4 >> ss_ver)
                                as isize,
                        );
                    y = init_y >> ss_ver;
                    t.by += init_y;
                    while y < cmp::min(ch4, init_y + 16 >> ss_ver) {
                        let mut x;
                        x = init_x >> ss_hor;
                        t.bx += init_x;
                        while x < cmp::min(cw4, init_x + 16 >> ss_hor) {
                            let cf: *mut BD::Coef;
                            let eob;
                            let mut txtp: TxfmType;
                            if t.frame_thread.pass != 0 {
                                let p = t.frame_thread.pass & 1;
                                cf = (*ts).frame_thread[p as usize].cf as *mut BD::Coef;
                                (*ts).frame_thread[p as usize].cf =
                                    ((*ts).frame_thread[p as usize].cf as *mut BD::Coef).offset(
                                        ((*uvtx).w as c_int * (*uvtx).h as c_int * 16) as isize,
                                    ) as *mut DynCoef;
                                let cbi = f.frame_thread.cbi
                                    [(t.by as isize * f.b4_stride + t.bx as isize) as usize]
                                    [(1 + pl) as usize];
                                eob = cbi.eob().into();
                                txtp = cbi.txtp();
                            } else {
                                let mut cf_ctx: u8 = 0;
                                cf = BD::select_mut(&mut (*t).cf).0.as_mut_ptr();
                                txtp = t.scratch.c2rust_unnamed_0.ac_txtp_map.txtp_map
                                    [((by4 + (y << ss_ver)) * 32 + bx4 + (x << ss_hor)) as usize]
                                    as TxfmType;
                                eob = decode_coefs::<BD>(
                                    f,
                                    t,
                                    &mut (*t.a).ccoef.0[pl as usize][(cbx4 + x) as usize..],
                                    &mut t.l.ccoef.0[pl as usize][(cby4 + y) as usize..],
                                    b.uvtx as RectTxfmSize,
                                    bs,
                                    b,
                                    0 as c_int,
                                    1 + pl,
                                    cf,
                                    &mut txtp,
                                    &mut cf_ctx,
                                );
                                if debug_block_info!(f, t) {
                                    println!(
                                        "Post-uv-cf-blk[pl={},tx={},txtp={},eob={}]: r={}",
                                        pl,
                                        b.uvtx as c_int,
                                        txtp as c_uint,
                                        eob,
                                        (*ts).msac.rng,
                                    );
                                }
                                CaseSet::<16, true>::many(
                                    [&mut t.l, &mut *t.a],
                                    [
                                        cmp::min((*uvtx).h as i32, f.bh - t.by + ss_ver >> ss_ver)
                                            as usize,
                                        cmp::min((*uvtx).w as i32, f.bw - t.bx + ss_hor >> ss_hor)
                                            as usize,
                                    ],
                                    [(cby4 + y) as usize, (cbx4 + x) as usize],
                                    |case, dir| {
                                        case.set(&mut dir.ccoef.0[pl as usize], cf_ctx);
                                    },
                                );
                            }
                            if eob >= 0 {
                                if debug_block_info!(f, t) && 0 != 0 {
                                    coef_dump(
                                        cf,
                                        (*uvtx).h as usize * 4,
                                        (*uvtx).w as usize * 4,
                                        3,
                                        "dq",
                                    );
                                }
                                ((*dsp).itx.itxfm_add[b.uvtx as usize][txtp as usize])
                                    .expect("non-null function pointer")(
                                    uvdst.offset((4 * x) as isize).cast(),
                                    f.cur.stride[1],
                                    cf.cast(),
                                    eob,
                                    f.bitdepth_max,
                                );
                                if debug_block_info!(f, t) && 0 != 0 {
                                    hex_dump::<BD>(
                                        &mut *uvdst.offset((4 * x) as isize),
                                        f.cur.stride[1] as usize,
                                        (*uvtx).w as usize * 4,
                                        (*uvtx).h as usize * 4,
                                        "recon",
                                    );
                                }
                            }
                            t.bx += ((*uvtx).w as c_int) << ss_hor;
                            x += (*uvtx).w as c_int;
                        }
                        uvdst = uvdst.offset(
                            (BD::pxstride(f.cur.stride[1]) * 4 * (*uvtx).h as isize) as isize,
                        );
                        t.bx -= x << ss_hor;
                        t.by += ((*uvtx).h as c_int) << ss_ver;
                        y += (*uvtx).h as c_int;
                    }
                    t.by -= y << ss_ver;
                    pl += 1;
                }
            }
            init_x += 16 as c_int;
        }
        init_y += 16 as c_int;
    }
    return 0 as c_int;
}

pub(crate) unsafe fn rav1d_filter_sbrow_deblock_cols<BD: BitDepth>(
    c: &Rav1dContext,
    f: &mut Rav1dFrameData,
    _t: &mut Rav1dTaskContext,
    sby: c_int,
) {
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    if !c.inloop_filters.contains(Rav1dInloopFilterType::DEBLOCK)
        || frame_hdr.loopfilter.level_y[0] == 0 && frame_hdr.loopfilter.level_y[1] == 0
    {
        return;
    }
    let y = sby * f.sb_step * 4;
    let ss_ver = (f.cur.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint) as c_int;
    let ss_hor = (f.cur.p.layout as c_uint != Rav1dPixelLayout::I444 as c_int as c_uint) as c_int;

    let (mut p, p_offset) = {
        let y_stride = BD::pxstride((*f).cur.stride[0]);
        let y_width = (*f).cur.p.w + 127 & !127;
        let y_height = (*f).cur.p.h + 127 & !127;
        let y_span = (y_height - 1) as isize * y_stride;
        let uv_stride = BD::pxstride((*f).cur.stride[1]);
        let uv_width = y_width >> ss_hor;
        let uv_height = y_height >> ss_ver;
        let uv_span = (uv_height - 1) as isize * uv_stride;

        let p: [&mut [BD::Pixel]; 3] = [
            slice::from_raw_parts_mut(
                f.cur.data.data[f.lf.p[0]]
                    .cast::<BD::Pixel>()
                    .offset(cmp::min(y_span, 0)),
                y_span.unsigned_abs() + y_width as usize + RAV1D_PICTURE_ALIGNMENT,
            ),
            slice::from_raw_parts_mut(
                f.cur.data.data[f.lf.p[1]]
                    .cast::<BD::Pixel>()
                    .offset(cmp::min(uv_span, 0)),
                uv_span.unsigned_abs() + uv_width as usize + RAV1D_PICTURE_ALIGNMENT,
            ),
            slice::from_raw_parts_mut(
                f.cur.data.data[f.lf.p[2]]
                    .cast::<BD::Pixel>()
                    .offset(cmp::min(uv_span, 0)),
                uv_span.unsigned_abs() + uv_width as usize + RAV1D_PICTURE_ALIGNMENT,
            ),
        ];
        let p_offset: [usize; 2] = [
            (cmp::max(0, -y_span) + y as isize * y_stride) as usize,
            (cmp::max(0, -uv_span) + y as isize * uv_stride >> ss_ver) as usize,
        ];
        (p, p_offset)
    };
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    let mask_offset = (sby >> (seq_hdr.sb128 == 0) as c_int) * f.sb128w;
    rav1d_loopfilter_sbrow_cols::<BD>(
        f,
        &mut p,
        &p_offset,
        mask_offset as usize,
        sby,
        f.lf.start_of_tile_row[sby as usize] as c_int,
    );
}

pub(crate) unsafe fn rav1d_filter_sbrow_deblock_rows<BD: BitDepth>(
    c: &Rav1dContext,
    f: &mut Rav1dFrameData,
    _t: &mut Rav1dTaskContext,
    sby: c_int,
) {
    let y = sby * f.sb_step * 4;
    let ss_ver = (f.cur.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint) as c_int;
    let ss_hor = (f.cur.p.layout as c_uint != Rav1dPixelLayout::I444 as c_int as c_uint) as c_int;

    let (mut p, p_offset) = {
        let y_stride = BD::pxstride((*f).cur.stride[0]);
        let y_width = (*f).cur.p.w + 127 & !127;
        let y_height = (*f).cur.p.h + 127 & !127;
        let y_span = (y_height - 1) as isize * y_stride;
        let uv_stride = BD::pxstride((*f).cur.stride[1]);
        let uv_width = y_width >> ss_hor;
        let uv_height = y_height >> ss_ver;
        let uv_span = (uv_height - 1) as isize * uv_stride;

        let p: [&mut [BD::Pixel]; 3] = [
            slice::from_raw_parts_mut(
                f.cur.data.data[f.lf.p[0]]
                    .cast::<BD::Pixel>()
                    .offset(cmp::min(y_span, 0)),
                y_span.unsigned_abs() + y_width as usize + RAV1D_PICTURE_ALIGNMENT,
            ),
            slice::from_raw_parts_mut(
                f.cur.data.data[f.lf.p[1]]
                    .cast::<BD::Pixel>()
                    .offset(cmp::min(uv_span, 0)),
                uv_span.unsigned_abs() + uv_width as usize + RAV1D_PICTURE_ALIGNMENT,
            ),
            slice::from_raw_parts_mut(
                f.cur.data.data[f.lf.p[2]]
                    .cast::<BD::Pixel>()
                    .offset(cmp::min(uv_span, 0)),
                uv_span.unsigned_abs() + uv_width as usize + RAV1D_PICTURE_ALIGNMENT,
            ),
        ];
        let p_offset: [usize; 2] = [
            (cmp::max(0, -y_span) + y as isize * y_stride) as usize,
            (cmp::max(0, -uv_span) + y as isize * uv_stride >> ss_ver) as usize,
        ];
        (p, p_offset)
    };
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    let sb128 = seq_hdr.sb128;
    let cdef = seq_hdr.cdef;
    let mask_offset = (sby >> (sb128 == 0) as c_int) * f.sb128w;
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    if c.inloop_filters.contains(Rav1dInloopFilterType::DEBLOCK)
        && (frame_hdr.loopfilter.level_y[0] != 0 || frame_hdr.loopfilter.level_y[1] != 0)
    {
        rav1d_loopfilter_sbrow_rows::<BD>(f, &mut p, &p_offset, mask_offset as usize, sby);
    }
    if cdef != 0 || f.lf.restore_planes != 0 {
        rav1d_copy_lpf::<BD>(c, f, &p, &p_offset, sby);
    }
}

pub(crate) unsafe fn rav1d_filter_sbrow_cdef<BD: BitDepth>(
    c: &Rav1dContext,
    f: &mut Rav1dFrameData,
    tc: &mut Rav1dTaskContext,
    sby: c_int,
) {
    if !c.inloop_filters.contains(Rav1dInloopFilterType::CDEF) {
        return;
    }
    let sbsz = f.sb_step;
    let y = sby * sbsz * 4;
    let ss_ver = (f.cur.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint) as c_int;
    let p: [*mut BD::Pixel; 3] = [
        f.cur.data.data[f.lf.p[0]]
            .cast::<BD::Pixel>()
            .offset((y as isize * BD::pxstride(f.cur.stride[0])) as isize),
        f.cur.data.data[f.lf.p[1]]
            .cast::<BD::Pixel>()
            .offset((y as isize * BD::pxstride(f.cur.stride[1]) >> ss_ver) as isize),
        f.cur.data.data[f.lf.p[2]]
            .cast::<BD::Pixel>()
            .offset((y as isize * BD::pxstride(f.cur.stride[1]) >> ss_ver) as isize),
    ];
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    let prev_mask = (sby - 1 >> (seq_hdr.sb128 == 0) as c_int) * f.sb128w;
    let mask_offset = (sby >> (seq_hdr.sb128 == 0) as c_int) * f.sb128w;
    let start = sby * sbsz;
    if sby != 0 {
        let ss_ver =
            (f.cur.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint) as c_int;
        let p_up: [*mut BD::Pixel; 3] = [
            (p[0]).offset(-((8 * BD::pxstride(f.cur.stride[0])) as isize)),
            (p[1]).offset(-((8 * BD::pxstride(f.cur.stride[1]) >> ss_ver) as isize)),
            (p[2]).offset(-((8 * BD::pxstride(f.cur.stride[1]) >> ss_ver) as isize)),
        ];
        rav1d_cdef_brow::<BD>(c, tc, f, &p_up, prev_mask, start - 2, start, true, sby);
    }

    let n_blks = sbsz - 2 * ((sby + 1) < f.sbh) as c_int;
    let end = cmp::min(start + n_blks, f.bh);
    rav1d_cdef_brow::<BD>(c, tc, f, &p, mask_offset, start, end, false, sby);
}

pub(crate) unsafe fn rav1d_filter_sbrow_resize<BD: BitDepth>(
    _c: &Rav1dContext,
    f: &mut Rav1dFrameData,
    _t: &mut Rav1dTaskContext,
    sby: c_int,
) {
    let sbsz = f.sb_step;
    let y = sby * sbsz * 4;
    let ss_ver = (f.cur.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint) as c_int;
    let p: [*const BD::Pixel; 3] = [
        f.cur.data.data[f.lf.p[0]]
            .cast::<BD::Pixel>()
            .offset(y as isize * BD::pxstride(f.cur.stride[0])),
        f.cur.data.data[f.lf.p[1]]
            .cast::<BD::Pixel>()
            .offset(y as isize * BD::pxstride(f.cur.stride[1]) >> ss_ver),
        f.cur.data.data[f.lf.p[2]]
            .cast::<BD::Pixel>()
            .offset(y as isize * BD::pxstride(f.cur.stride[1]) >> ss_ver),
    ];
    let sr_p: [*mut BD::Pixel; 3] = [
        f.sr_cur.p.data.data[f.lf.sr_p[0]]
            .cast::<BD::Pixel>()
            .offset((y as isize * BD::pxstride(f.sr_cur.p.stride[0])) as isize),
        f.sr_cur.p.data.data[f.lf.sr_p[1]]
            .cast::<BD::Pixel>()
            .offset((y as isize * BD::pxstride(f.sr_cur.p.stride[1]) >> ss_ver) as isize),
        f.sr_cur.p.data.data[f.lf.sr_p[2]]
            .cast::<BD::Pixel>()
            .offset((y as isize * BD::pxstride(f.sr_cur.p.stride[1]) >> ss_ver) as isize),
    ];
    let has_chroma =
        (f.cur.p.layout as c_uint != Rav1dPixelLayout::I400 as c_int as c_uint) as c_int;
    let mut pl = 0;
    while pl < 1 + 2 * has_chroma {
        let ss_ver = (pl != 0
            && f.cur.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint)
            as c_int;
        let h_start = 8 * (sby != 0) as c_int >> ss_ver;
        let dst_stride: ptrdiff_t = f.sr_cur.p.stride[(pl != 0) as c_int as usize];
        let dst: *mut BD::Pixel =
            (sr_p[pl as usize]).offset(-((h_start as isize * BD::pxstride(dst_stride)) as isize));
        let src_stride: ptrdiff_t = f.cur.stride[(pl != 0) as c_int as usize];
        let src: *const BD::Pixel =
            (p[pl as usize]).offset(-(h_start as isize * BD::pxstride(src_stride)));
        let h_end = 4 * (sbsz - 2 * ((sby + 1) < f.sbh) as c_int) >> ss_ver;
        let ss_hor = (pl != 0
            && f.cur.p.layout as c_uint != Rav1dPixelLayout::I444 as c_int as c_uint)
            as c_int;
        let dst_w = f.sr_cur.p.p.w + ss_hor >> ss_hor;
        let src_w = 4 * f.bw + ss_hor >> ss_hor;
        let img_h = f.cur.p.h - sbsz * 4 * sby + ss_ver >> ss_ver;
        ((*f.dsp).mc.resize)(
            dst.cast(),
            dst_stride,
            src.cast(),
            src_stride,
            dst_w,
            cmp::min(img_h, h_end) + h_start,
            src_w,
            f.resize_step[(pl != 0) as c_int as usize],
            f.resize_start[(pl != 0) as c_int as usize],
            f.bitdepth_max,
        );
        pl += 1;
    }
}

pub(crate) unsafe fn rav1d_filter_sbrow_lr<BD: BitDepth>(
    c: &Rav1dContext,
    f: &mut Rav1dFrameData,
    _t: &mut Rav1dTaskContext,
    sby: c_int,
) {
    if !c
        .inloop_filters
        .contains(Rav1dInloopFilterType::RESTORATION)
    {
        return;
    }
    let y = sby * f.sb_step * 4;
    let ss_ver = (f.cur.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint) as c_int;
    let h = (*f).sr_cur.p.p.h + 127 & !127;
    let mut sr_p: [&mut [BD::Pixel]; 3] = [
        slice::from_raw_parts_mut(
            f.sr_cur.p.data.data[f.lf.sr_p[0]].cast::<BD::Pixel>(),
            (h as isize * BD::pxstride(f.sr_cur.p.stride[0])) as usize,
        ),
        slice::from_raw_parts_mut(
            f.sr_cur.p.data.data[f.lf.sr_p[1]].cast::<BD::Pixel>(),
            (h as isize * BD::pxstride(f.sr_cur.p.stride[1])) as usize >> ss_ver,
        ),
        slice::from_raw_parts_mut(
            f.sr_cur.p.data.data[f.lf.sr_p[2]].cast::<BD::Pixel>(),
            (h as isize * BD::pxstride(f.sr_cur.p.stride[1])) as usize >> ss_ver,
        ),
    ];
    let sr_p_offset: [usize; 2] = [
        (y as isize * BD::pxstride(f.sr_cur.p.stride[0])) as usize,
        (y as isize * BD::pxstride(f.sr_cur.p.stride[1]) >> ss_ver) as usize,
    ];
    rav1d_lr_sbrow::<BD>(c, f, &mut sr_p, &sr_p_offset, sby);
}

pub(crate) unsafe fn rav1d_filter_sbrow<BD: BitDepth>(
    c: &Rav1dContext,
    f: &mut Rav1dFrameData,
    t: &mut Rav1dTaskContext,
    sby: c_int,
) {
    rav1d_filter_sbrow_deblock_cols::<BD>(c, f, t, sby);
    rav1d_filter_sbrow_deblock_rows::<BD>(c, f, t, sby);
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    if seq_hdr.cdef != 0 {
        rav1d_filter_sbrow_cdef::<BD>(c, f, t, sby);
    }
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    if frame_hdr.size.width[0] != frame_hdr.size.width[1] {
        rav1d_filter_sbrow_resize::<BD>(c, f, t, sby);
    }
    if f.lf.restore_planes != 0 {
        rav1d_filter_sbrow_lr::<BD>(c, f, t, sby);
    }
}

pub(crate) unsafe fn rav1d_backup_ipred_edge<BD: BitDepth>(
    f: &Rav1dFrameData,
    t: &mut Rav1dTaskContext,
) {
    let ts: *mut Rav1dTileState = t.ts;
    let sby = t.by >> f.sb_shift;
    let sby_off = f.sb128w * 128 * sby;
    let x_off = (*ts).tiling.col_start;
    let y: *const BD::Pixel = (f.cur.data.data[0] as *const BD::Pixel)
        .offset((x_off * 4) as isize)
        .offset((((t.by + f.sb_step) * 4 - 1) as isize * BD::pxstride(f.cur.stride[0])) as isize);
    BD::pixel_copy(
        &mut slice::from_raw_parts_mut(
            f.ipred_edge[0].cast(),
            (sby_off + x_off * 4 + (4 * ((*ts).tiling.col_end - x_off)))
                .try_into()
                .unwrap(),
        )[(sby_off + x_off * 4).try_into().unwrap()..],
        slice::from_raw_parts(y, (4 * ((*ts).tiling.col_end - x_off)).try_into().unwrap()),
        (4 * ((*ts).tiling.col_end - x_off)).try_into().unwrap(),
    );
    if f.cur.p.layout as c_uint != Rav1dPixelLayout::I400 as c_int as c_uint {
        let ss_ver =
            (f.cur.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint) as c_int;
        let ss_hor =
            (f.cur.p.layout as c_uint != Rav1dPixelLayout::I444 as c_int as c_uint) as c_int;
        let uv_off: ptrdiff_t = (x_off * 4 >> ss_hor) as isize
            + (((t.by + f.sb_step) * 4 >> ss_ver) - 1) as isize * BD::pxstride(f.cur.stride[1]);
        let mut pl = 1;
        while pl <= 2 {
            BD::pixel_copy(
                &mut slice::from_raw_parts_mut(
                    f.ipred_edge[pl as usize].cast(),
                    (sby_off
                        + (x_off * 4 >> ss_hor)
                        + (4 * ((*ts).tiling.col_end - x_off) >> ss_hor))
                        .try_into()
                        .unwrap(),
                )[(sby_off + (x_off * 4 >> ss_hor)).try_into().unwrap()..],
                &slice::from_raw_parts(
                    f.cur.data.data[pl as usize].cast(),
                    (uv_off + (4 * ((*ts).tiling.col_end - x_off) >> ss_hor) as isize)
                        .try_into()
                        .unwrap(),
                )[uv_off.try_into().unwrap()..],
                (4 * ((*ts).tiling.col_end - x_off) >> ss_hor)
                    .try_into()
                    .unwrap(),
            );
            pl += 1;
        }
    }
}
