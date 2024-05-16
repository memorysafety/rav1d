use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::BPC;
use crate::include::common::dump::ac_dump;
use crate::include::common::dump::coef_dump;
use crate::include::common::dump::hex_dump;
use crate::include::common::intops::apply_sign64;
use crate::include::common::intops::clip;
use crate::include::common::intops::ulog2;
use crate::include::dav1d::dav1d::Rav1dInloopFilterType;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::include::dav1d::headers::Rav1dPixelLayoutSubSampled;
use crate::include::dav1d::headers::Rav1dWarpedMotionParams;
use crate::include::dav1d::headers::Rav1dWarpedMotionType;
use crate::include::dav1d::picture::RAV1D_PICTURE_ALIGNMENT;
use crate::src::cdef_apply::rav1d_cdef_brow;
use crate::src::ctx::CaseSet;
use crate::src::env::get_uv_inter_txtp;
use crate::src::internal::Bxy;
use crate::src::internal::Cf;
use crate::src::internal::CodedBlockInfo;
use crate::src::internal::Rav1dContext;
use crate::src::internal::Rav1dFrameData;
use crate::src::internal::Rav1dTaskContext;
use crate::src::internal::Rav1dTileStateContext;
use crate::src::internal::ScratchEmuEdge;
use crate::src::internal::TaskContextScratch;
use crate::src::internal::TileStateRef;
use crate::src::intra_edge::EdgeFlags;
use crate::src::ipred_prepare::rav1d_prepare_intra_edges;
use crate::src::ipred_prepare::sm_flag;
use crate::src::ipred_prepare::sm_uv_flag;
use crate::src::levels::mv;
use crate::src::levels::Av1Block;
use crate::src::levels::Av1BlockInter;
use crate::src::levels::Av1BlockIntra;
use crate::src::levels::Av1BlockIntraInter;
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
use std::ffi::c_uint;
use std::mem;
use std::ops::BitOr;
use std::ptr;
use std::slice;
use std::sync::atomic::Ordering;

impl Bxy {
    pub fn debug_block_info(&self) -> bool {
        let Self { x, y } = *self;
        (0..4).contains(&y) && (8..12).contains(&x)
    }
}

// TODO: add feature and compile-time guard around this code
/// Determine if we should print debug information for the current block.
///
/// Takes a [`Rav1dFrameData`] and a [`Bxy`] as arguments to
/// determine the current block and frame offset.
///
/// This a macro rather than a function so that the compiler can see which
/// specific fields are used to avoid borrowck errors.
///
/// [`Bxy`]: crate::src::internal::Bxy
macro_rules! debug_block_info {
    ($f:expr, $tb:expr) => {{
        use crate::src::internal::Bxy;

        let tb: Bxy = $tb;
        false && $f.frame_hdr.as_ref().unwrap().frame_offset == 2 && tb.debug_block_info()
    }};
}
pub(crate) use debug_block_info;

const DEBUG_B_PIXELS: bool = false;

pub(crate) type recon_b_intra_fn = unsafe fn(
    &Rav1dFrameData,
    &mut Rav1dTaskContext,
    Option<&mut Rav1dTileStateContext>,
    BlockSize,
    EdgeFlags,
    &Av1Block,
    &Av1BlockIntra,
) -> ();

pub(crate) type recon_b_inter_fn = unsafe fn(
    &Rav1dFrameData,
    &mut Rav1dTaskContext,
    Option<&mut Rav1dTileStateContext>,
    BlockSize,
    &Av1Block,
    &Av1BlockInter,
) -> Result<(), ()>;

pub(crate) type filter_sbrow_fn =
    unsafe fn(&Rav1dContext, &Rav1dFrameData, &mut Rav1dTaskContext, c_int) -> ();

pub(crate) type backup_ipred_edge_fn = unsafe fn(&Rav1dFrameData, &mut Rav1dTaskContext) -> ();

pub(crate) type read_coef_blocks_fn = unsafe fn(
    &Rav1dFrameData,
    &mut Rav1dTaskContext,
    &mut Rav1dTileStateContext,
    BlockSize,
    &Av1Block,
) -> ();

pub(crate) type copy_pal_block_fn = unsafe fn(
    t: &mut Rav1dTaskContext,
    f: &Rav1dFrameData,
    bx4: usize,
    by4: usize,
    bw4: usize,
    bh4: usize,
) -> ();

pub(crate) type read_pal_plane_fn = unsafe fn(
    t: &mut Rav1dTaskContext,
    f: &Rav1dFrameData,
    ts_c: &mut Rav1dTileStateContext,
    pl: bool,
    sz_ctx: u8,
    bx4: usize,
    by4: usize,
) -> u8; // `pal_sz`

pub(crate) type read_pal_uv_fn = unsafe fn(
    t: &mut Rav1dTaskContext,
    f: &Rav1dFrameData,
    ts_c: &mut Rav1dTileStateContext,
    sz_ctx: u8,
    bx4: usize,
    by4: usize,
) -> u8; // `pal_sz[1]`

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
    chroma: bool,
    layout: Rav1dPixelLayout,
) -> u8 {
    let b_dim = &dav1d_block_dimensions[bs as usize];
    if chroma {
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
    ts: usize,
    ts_c: &mut Rav1dTileStateContext,
    dbg_block_info: bool,
    scratch: &mut TaskContextScratch,
    t_cf: &mut Cf,
    a: &mut [u8],
    l: &mut [u8],
    tx: RectTxfmSize,
    bs: BlockSize,
    b: &Av1Block,
    plane: usize,
    cf: CfSelect,
    txtp: &mut TxfmType,
    res_ctx: &mut u8,
) -> c_int {
    let dc_sign_ctx;
    let dc_sign;
    let mut dc_dq;
    let current_block: u64;
    let ts = &f.ts[ts];
    let chroma = plane != 0;
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    let lossless = frame_hdr.segmentation.lossless[b.seg_id as usize];
    let t_dim = &dav1d_txfm_dimensions[tx as usize];
    let dbg = dbg_block_info && plane != 0 && false;

    if dbg {
        println!("Start: r={}", ts_c.msac.rng);
    }

    // does this block have any non-zero coefficients
    let sctx = get_skip_ctx(t_dim, bs, a, l, chroma, f.cur.p.layout) as c_int;
    let all_skip = rav1d_msac_decode_bool_adapt(
        &mut ts_c.msac,
        &mut ts_c.cdf.coef.skip[t_dim.ctx as usize][sctx as usize],
    );
    if dbg {
        println!(
            "Post-non-zero[{}][{}][{}]: r={}",
            t_dim.ctx, sctx, all_skip, ts_c.msac.rng,
        );
    }
    if all_skip {
        *res_ctx = 0x40;
        *txtp = if lossless { WHT_WHT } else { DCT_DCT };
        return -1;
    }

    // transform type (chroma: derived, luma: explicitly coded)
    use Av1BlockIntraInter::*;
    *txtp = match &b.ii {
        _ if lossless => {
            assert!(t_dim.max == TX_4X4);
            WHT_WHT
        }
        Intra(_) if t_dim.max >= TX_32X32 => DCT_DCT,
        Inter(_) if t_dim.max >= TX_64X64 => DCT_DCT,
        Intra(intra) if chroma => dav1d_txtp_from_uvmode[intra.uv_mode as usize],
        // inferred from either the luma txtp (inter) or a LUT (intra)
        Inter(_) if chroma => get_uv_inter_txtp(t_dim, *txtp),
        // In libaom, lossless is checked by a literal qidx == 0, but not all
        // such blocks are actually lossless. The remainder gets an implicit
        // transform type (for luma)
        _ if frame_hdr.segmentation.qidx[b.seg_id as usize] == 0 => DCT_DCT,
        Intra(intra) => {
            let y_mode_nofilt = if intra.y_mode == FILTER_PRED {
                dav1d_filter_mode_to_y_mode[intra.y_angle as usize]
            } else {
                intra.y_mode
            };
            let idx;
            let txtp = if frame_hdr.reduced_txtp_set != 0 || t_dim.min == TX_16X16 {
                idx = rav1d_msac_decode_symbol_adapt4(
                    &mut ts_c.msac,
                    &mut ts_c.cdf.m.txtp_intra2[t_dim.min as usize][y_mode_nofilt as usize],
                    4,
                );
                dav1d_tx_types_per_set[idx as usize + 0]
            } else {
                idx = rav1d_msac_decode_symbol_adapt8(
                    &mut ts_c.msac,
                    &mut ts_c.cdf.m.txtp_intra1[t_dim.min as usize][y_mode_nofilt as usize],
                    6,
                );
                dav1d_tx_types_per_set[idx as usize + 5]
            };
            if dbg {
                println!(
                    "Post-txtp-intra[{}->{}][{}][{}->{}]: r={}",
                    tx, t_dim.min, y_mode_nofilt, idx, txtp, ts_c.msac.rng,
                );
            }
            txtp
        }
        Inter(_) => {
            let idx;
            let txtp = if frame_hdr.reduced_txtp_set != 0 || t_dim.max == TX_32X32 {
                let bool_idx = rav1d_msac_decode_bool_adapt(
                    &mut ts_c.msac,
                    &mut ts_c.cdf.m.txtp_inter3[t_dim.min as usize],
                );
                idx = bool_idx as c_uint;
                if bool_idx {
                    DCT_DCT
                } else {
                    IDTX
                }
            } else if t_dim.min == TX_16X16 {
                idx = rav1d_msac_decode_symbol_adapt16(
                    &mut ts_c.msac,
                    &mut ts_c.cdf.m.txtp_inter2.0,
                    11,
                );
                dav1d_tx_types_per_set[idx as usize + 12]
            } else {
                idx = rav1d_msac_decode_symbol_adapt16(
                    &mut ts_c.msac,
                    &mut ts_c.cdf.m.txtp_inter1[t_dim.min as usize],
                    15,
                );
                dav1d_tx_types_per_set[idx as usize + 24]
            };
            if dbg {
                println!(
                    "Post-txtp-inter[{}->{}][{}->{}]: r={}",
                    tx, t_dim.min, idx, txtp, ts_c.msac.rng,
                );
            }
            txtp
        }
    };

    // find end-of-block (eob)
    let tx2dszctx = cmp::min(t_dim.lw, TX_32X32 as u8) + cmp::min(t_dim.lh, TX_32X32 as u8);
    let tx_class = dav1d_tx_type_class[*txtp as usize];
    let chroma = chroma as usize;
    let is_1d = (tx_class != TxClass::TwoD) as usize;
    let eob_bin = match tx2dszctx {
        0 => {
            let eob_bin_cdf = &mut ts_c.cdf.coef.eob_bin_16[chroma][is_1d];
            rav1d_msac_decode_symbol_adapt4(&mut ts_c.msac, eob_bin_cdf, (4 + 0) as usize)
        }
        1 => {
            let eob_bin_cdf = &mut ts_c.cdf.coef.eob_bin_32[chroma][is_1d];
            rav1d_msac_decode_symbol_adapt8(&mut ts_c.msac, eob_bin_cdf, (4 + 1) as usize)
        }
        2 => {
            let eob_bin_cdf = &mut ts_c.cdf.coef.eob_bin_64[chroma][is_1d];
            rav1d_msac_decode_symbol_adapt8(&mut ts_c.msac, eob_bin_cdf, (4 + 2) as usize)
        }
        3 => {
            let eob_bin_cdf = &mut ts_c.cdf.coef.eob_bin_128[chroma][is_1d];
            rav1d_msac_decode_symbol_adapt8(&mut ts_c.msac, eob_bin_cdf, (4 + 3) as usize)
        }
        4 => {
            let eob_bin_cdf = &mut ts_c.cdf.coef.eob_bin_256[chroma][is_1d];
            rav1d_msac_decode_symbol_adapt16(&mut ts_c.msac, eob_bin_cdf, (4 + 4) as usize)
        }
        5 => {
            let eob_bin_cdf = &mut ts_c.cdf.coef.eob_bin_512[chroma];
            rav1d_msac_decode_symbol_adapt16(&mut ts_c.msac, eob_bin_cdf, (4 + 5) as usize)
        }
        6 => {
            let eob_bin_cdf = &mut ts_c.cdf.coef.eob_bin_1024[chroma];
            rav1d_msac_decode_symbol_adapt16(&mut ts_c.msac, eob_bin_cdf, (4 + 6) as usize)
        }
        // `tx2dszctx` is `cmp::min(_, 3) + cmp::min(_, 3)`, where `TX_32X32 as u8 == 3`,
        // and we cover `0..=6`.  `rustc` should eliminate this.
        _ => unreachable!(),
    };
    if dbg {
        println!(
            "Post-eob_bin_{}[{}][{}][{}]: r={}",
            16 << tx2dszctx,
            chroma,
            is_1d,
            eob_bin,
            ts_c.msac.rng,
        );
    }
    let eob;
    if eob_bin > 1 {
        let eob_hi_bit_cdf =
            &mut ts_c.cdf.coef.eob_hi_bit[t_dim.ctx as usize][chroma][eob_bin as usize];
        let eob_hi_bit = rav1d_msac_decode_bool_adapt(&mut ts_c.msac, eob_hi_bit_cdf) as c_uint;
        if dbg {
            println!(
                "Post-eob_hi_bit[{}][{}][{}][{}]: r={}",
                t_dim.ctx, chroma, eob_bin, eob_hi_bit, ts_c.msac.rng,
            );
        }
        eob = ((eob_hi_bit | 2) << eob_bin - 2
            | rav1d_msac_decode_bools(&mut ts_c.msac, eob_bin - 2)) as c_int;
        if dbg {
            println!("Post-eob[{}]: r={}", eob, ts_c.msac.rng);
        }
    } else {
        eob = eob_bin as c_int;
    }
    assert!(eob >= 0);

    // base tokens
    let eob_cdf = &mut ts_c.cdf.coef.eob_base_tok[t_dim.ctx as usize][chroma];
    let hi_cdf = &mut ts_c.cdf.coef.br_tok[cmp::min(t_dim.ctx, 3) as usize][chroma];
    let mut rc;
    let mut dc_tok;

    if eob != 0 {
        let lo_cdf = &mut ts_c.cdf.coef.base_tok[t_dim.ctx as usize][chroma];
        let levels = scratch.inter_intra_mut().levels_pal.levels_mut();
        let sw = cmp::min(t_dim.w, 8);
        let sh = cmp::min(t_dim.h, 8);

        // eob
        let mut ctx: c_uint = 1
            + (eob > sw as c_int * sh as c_int * 2) as c_uint
            + (eob > sw as c_int * sh as c_int * 4) as c_uint;
        let eob_tok =
            rav1d_msac_decode_symbol_adapt4(&mut ts_c.msac, &mut eob_cdf[ctx as usize], 2) as c_int;
        let mut tok = eob_tok + 1;
        let mut level_tok = tok * 0x41;
        let mut mag: c_uint = 0;

        let mut scan: &[u16] = &[];
        match tx_class {
            TxClass::TwoD => {
                let nonsquare_tx: c_uint = (tx >= RTX_4X8) as c_uint;
                let lo_ctx_offsets = Some(
                    &dav1d_lo_ctx_offsets
                        [nonsquare_tx.wrapping_add(tx as c_uint & nonsquare_tx) as usize],
                );
                scan = dav1d_scans[tx as usize];
                let stride = 4 * sh as usize;
                let shift: c_uint = if t_dim.lh < 4 {
                    t_dim.lh as c_uint + 2
                } else {
                    5
                };
                let shift2: c_uint = 0;
                let mask: c_uint = 4 * sh as c_uint - 1;
                levels[..stride * (4 * sw as usize + 2)].fill(0);
                let mut x: c_uint;
                let mut y: c_uint;
                match tx_class {
                    TxClass::TwoD => {
                        rc = scan[eob as usize] as c_uint;
                        x = rc >> shift;
                        y = rc & mask;
                    }
                    TxClass::H => {
                        // Transposing reduces the stride and padding requirements.
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
                        t_dim.ctx, chroma, ctx, eob, rc, tok, ts_c.msac.rng,
                    );
                }
                if eob_tok == 2 {
                    ctx = if if tx_class == TxClass::TwoD {
                        (x | y) > 1
                    } else {
                        y != 0
                    } {
                        14
                    } else {
                        7
                    };
                    tok = rav1d_msac_decode_hi_tok(&mut ts_c.msac, &mut hi_cdf[ctx as usize])
                        as c_int;
                    level_tok = tok + (3 << 6);
                    if dbg {
                        println!(
                            "Post-hi_tok[{}][{}][{}][{}={}={}]: r={}",
                            cmp::min(t_dim.ctx, 3),
                            chroma,
                            ctx,
                            eob,
                            rc,
                            tok,
                            ts_c.msac.rng,
                        );
                    }
                }
                cf.set::<BD>(f, t_cf, rc as usize, (tok << 11).as_::<BD::Coef>());
                levels[x as usize * stride + y as usize] = level_tok as u8;
                let mut i = eob - 1;
                while i > 0 {
                    // ac
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
                    assert!(x < 32 && y < 32);
                    let level = &mut levels[x as usize * stride + y as usize..];
                    ctx = get_lo_ctx(
                        level,
                        tx_class,
                        &mut mag,
                        lo_ctx_offsets,
                        x as usize,
                        y as usize,
                        stride,
                    ) as c_uint;
                    if tx_class == TxClass::TwoD {
                        y |= x;
                    }
                    tok = rav1d_msac_decode_symbol_adapt4(
                        &mut ts_c.msac,
                        &mut lo_cdf[ctx as usize],
                        3,
                    ) as c_int;
                    if dbg {
                        println!(
                            "Post-lo_tok[{}][{}][{}][{}={}={}]: r={}",
                            t_dim.ctx, chroma, ctx, i, rc_i, tok, ts_c.msac.rng,
                        );
                    }
                    if tok == 3 {
                        mag &= 63;
                        ctx = ((if y > (tx_class == TxClass::TwoD) as c_uint {
                            14
                        } else {
                            7
                        }) as c_uint)
                            .wrapping_add(if mag > 12 {
                                6
                            } else {
                                mag.wrapping_add(1) >> 1
                            });
                        tok = rav1d_msac_decode_hi_tok(&mut ts_c.msac, &mut hi_cdf[ctx as usize])
                            as c_int;
                        if dbg {
                            println!(
                                "Post-hi_tok[{}][{}][{}][{}={}={}]: r={}",
                                cmp::min(t_dim.ctx, 3),
                                chroma,
                                ctx,
                                i,
                                rc_i,
                                tok,
                                ts_c.msac.rng,
                            );
                        }
                        level[0] = (tok + (3 << 6)) as u8;
                        cf.set::<BD>(
                            f,
                            t_cf,
                            rc_i as usize,
                            ((tok << 11) as c_uint | rc).as_::<BD::Coef>(),
                        );
                        rc = rc_i;
                    } else {
                        // `0x1` for `tok`, `0x7ff` as bitmask for `rc`, `0x41` for `level_tok`.
                        tok *= 0x17ff41;
                        level[0] = tok as u8;
                        // `tok ? (tok << 11) | rc : 0`
                        tok = ((tok as c_uint >> 9) & rc.wrapping_add(!(0x7ff as c_uint))) as c_int;
                        if tok != 0 {
                            rc = rc_i;
                        }
                        cf.set::<BD>(f, t_cf, rc_i as usize, tok.as_::<BD::Coef>());
                    }
                    i -= 1;
                }
                // dc
                ctx = if tx_class == TxClass::TwoD {
                    0
                } else {
                    get_lo_ctx(levels, tx_class, &mut mag, lo_ctx_offsets, 0, 0, stride) as c_uint
                };
                dc_tok =
                    rav1d_msac_decode_symbol_adapt4(&mut ts_c.msac, &mut lo_cdf[ctx as usize], 3);
                if dbg {
                    println!(
                        "Post-dc_lo_tok[{}][{}][{}][{}]: r={}",
                        t_dim.ctx, chroma, ctx, dc_tok, ts_c.msac.rng,
                    );
                }
                if dc_tok == 3 {
                    if tx_class == TxClass::TwoD {
                        mag = levels[0 * stride + 1] as c_uint
                            + levels[1 * stride + 0] as c_uint
                            + levels[1 * stride + 1] as c_uint;
                    }
                    mag &= 63;
                    ctx = if mag > 12 {
                        6
                    } else {
                        mag.wrapping_add(1) >> 1
                    };
                    dc_tok = rav1d_msac_decode_hi_tok(&mut ts_c.msac, &mut hi_cdf[ctx as usize]);
                    if dbg {
                        println!(
                            "Post-dc_hi_tok[{}][{}][0][{}]: r={}",
                            cmp::min(t_dim.ctx, 3),
                            chroma,
                            dc_tok,
                            ts_c.msac.rng,
                        );
                    }
                }
            }
            TxClass::H => {
                let lo_ctx_offsets = None;
                let stride = 16;
                let shift: c_uint = t_dim.lh as c_uint + 2;
                let shift2: c_uint = 0;
                let mask: c_uint = 4 * sh as c_uint - 1;
                levels[..stride * (4 * sh as usize + 2)].fill(0);
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
                        t_dim.ctx, chroma, ctx, eob, rc, tok, ts_c.msac.rng,
                    );
                }
                if eob_tok == 2 {
                    ctx = if if tx_class == TxClass::TwoD {
                        (x | y) > 1
                    } else {
                        y != 0
                    } {
                        14
                    } else {
                        7
                    };
                    tok = rav1d_msac_decode_hi_tok(&mut ts_c.msac, &mut hi_cdf[ctx as usize])
                        as c_int;
                    level_tok = tok + (3 << 6);
                    if dbg {
                        println!(
                            "Post-hi_tok[{}][{}][{}][{}={}={}]: r={}",
                            cmp::min(t_dim.ctx, 3),
                            chroma,
                            ctx,
                            eob,
                            rc,
                            tok,
                            ts_c.msac.rng,
                        );
                    }
                }
                cf.set::<BD>(f, t_cf, rc as usize, (tok << 11).as_::<BD::Coef>());
                levels[x as usize * stride + y as usize] = level_tok as u8;
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
                    assert!(x < 32 && y < 32);
                    let level = &mut levels[x as usize * stride + y as usize..];
                    ctx = get_lo_ctx(
                        level,
                        tx_class,
                        &mut mag,
                        lo_ctx_offsets,
                        x as usize,
                        y as usize,
                        stride,
                    ) as c_uint;
                    if tx_class == TxClass::TwoD {
                        y |= x;
                    }
                    tok = rav1d_msac_decode_symbol_adapt4(
                        &mut ts_c.msac,
                        &mut lo_cdf[ctx as usize],
                        3,
                    ) as c_int;
                    if dbg {
                        println!(
                            "Post-lo_tok[{}][{}][{}][{}={}={}]: r={}",
                            t_dim.ctx, chroma, ctx, i, rc_i, tok, ts_c.msac.rng,
                        );
                    }
                    if tok == 3 {
                        mag &= 63;
                        ctx = ((if y > (tx_class == TxClass::TwoD) as c_uint {
                            14
                        } else {
                            7
                        }) as c_uint)
                            .wrapping_add(if mag > 12 {
                                6
                            } else {
                                mag.wrapping_add(1) >> 1
                            });
                        tok = rav1d_msac_decode_hi_tok(&mut ts_c.msac, &mut hi_cdf[ctx as usize])
                            as c_int;
                        if dbg {
                            println!(
                                "Post-hi_tok[{}][{}][{}][{}={}={}]: r={}",
                                cmp::min(t_dim.ctx, 3),
                                chroma,
                                ctx,
                                i,
                                rc_i,
                                tok,
                                ts_c.msac.rng,
                            );
                        }
                        level[0] = (tok + (3 << 6)) as u8;
                        cf.set::<BD>(
                            f,
                            t_cf,
                            rc_i as usize,
                            ((tok << 11) as c_uint | rc).as_::<BD::Coef>(),
                        );
                        rc = rc_i;
                    } else {
                        tok *= 0x17ff41;
                        level[0] = tok as u8;
                        tok = ((tok as c_uint >> 9) & rc.wrapping_add(!(0x7ff as c_uint))) as c_int;
                        if tok != 0 {
                            rc = rc_i;
                        }
                        cf.set::<BD>(f, t_cf, rc_i as usize, tok.as_::<BD::Coef>());
                    }
                    i -= 1;
                }
                ctx = if tx_class == TxClass::TwoD {
                    0
                } else {
                    get_lo_ctx(levels, tx_class, &mut mag, lo_ctx_offsets, 0, 0, stride) as c_uint
                };
                dc_tok =
                    rav1d_msac_decode_symbol_adapt4(&mut ts_c.msac, &mut lo_cdf[ctx as usize], 3);
                if dbg {
                    println!(
                        "Post-dc_lo_tok[{}][{}][{}][{}]: r={}",
                        t_dim.ctx, chroma, ctx, dc_tok, ts_c.msac.rng,
                    );
                }
                if dc_tok == 3 {
                    if tx_class == TxClass::TwoD {
                        mag = levels[0 * stride + 1] as c_uint
                            + levels[1 * stride + 0] as c_uint
                            + levels[1 * stride + 1] as c_uint;
                    }
                    mag &= 63;
                    ctx = if mag > 12 {
                        6
                    } else {
                        mag.wrapping_add(1) >> 1
                    };
                    dc_tok = rav1d_msac_decode_hi_tok(&mut ts_c.msac, &mut hi_cdf[ctx as usize]);
                    if dbg {
                        println!(
                            "Post-dc_hi_tok[{}][{}][0][{}]: r={}",
                            cmp::min(t_dim.ctx, 3),
                            chroma,
                            dc_tok,
                            ts_c.msac.rng,
                        );
                    }
                }
            }
            TxClass::V => {
                let lo_ctx_offsets = None;
                let stride = 16;
                let shift: c_uint = t_dim.lw as c_uint + 2;
                let shift2: c_uint = t_dim.lh as c_uint + 2;
                let mask: c_uint = 4 * sw as c_uint - 1;
                levels[..stride * (4 * sw as usize + 2)].fill(0);
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
                        t_dim.ctx, chroma, ctx, eob, rc, tok, ts_c.msac.rng,
                    );
                }
                if eob_tok == 2 {
                    ctx = if if tx_class == TxClass::TwoD {
                        (x | y) > 1
                    } else {
                        y != 0
                    } {
                        14
                    } else {
                        7
                    };
                    tok = rav1d_msac_decode_hi_tok(&mut ts_c.msac, &mut hi_cdf[ctx as usize])
                        as c_int;
                    level_tok = tok + (3 << 6);
                    if dbg {
                        println!(
                            "Post-hi_tok[{}][{}][{}][{}={}={}]: r={}",
                            cmp::min(t_dim.ctx, 3),
                            chroma,
                            ctx,
                            eob,
                            rc,
                            tok,
                            ts_c.msac.rng,
                        );
                    }
                }
                cf.set::<BD>(f, t_cf, rc as usize, (tok << 11).as_::<BD::Coef>());
                levels[x as usize * stride + y as usize] = level_tok as u8;
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
                    assert!(x < 32 && y < 32);
                    let level = &mut levels[x as usize * stride + y as usize..];
                    ctx = get_lo_ctx(
                        level,
                        tx_class,
                        &mut mag,
                        lo_ctx_offsets,
                        x as usize,
                        y as usize,
                        stride,
                    ) as c_uint;
                    if tx_class == TxClass::TwoD {
                        y |= x;
                    }
                    tok = rav1d_msac_decode_symbol_adapt4(
                        &mut ts_c.msac,
                        &mut lo_cdf[ctx as usize],
                        3,
                    ) as c_int;
                    if dbg {
                        println!(
                            "Post-lo_tok[{}][{}][{}][{}={}={}]: r={}",
                            t_dim.ctx, chroma, ctx, i, rc_i, tok, ts_c.msac.rng,
                        );
                    }
                    if tok == 3 {
                        mag &= 63;
                        ctx = ((if y > (tx_class == TxClass::TwoD) as c_uint {
                            14
                        } else {
                            7
                        }) as c_uint)
                            .wrapping_add(if mag > 12 {
                                6
                            } else {
                                mag.wrapping_add(1) >> 1
                            });
                        tok = rav1d_msac_decode_hi_tok(&mut ts_c.msac, &mut hi_cdf[ctx as usize])
                            as c_int;
                        if dbg {
                            println!(
                                "Post-hi_tok[{}][{}][{}][{}={}={}]: r={}",
                                cmp::min(t_dim.ctx, 3),
                                chroma,
                                ctx,
                                i,
                                rc_i,
                                tok,
                                ts_c.msac.rng,
                            );
                        }
                        level[0] = (tok + (3 << 6)) as u8;
                        cf.set::<BD>(
                            f,
                            t_cf,
                            rc_i as usize,
                            ((tok << 11) as c_uint | rc).as_::<BD::Coef>(),
                        );
                        rc = rc_i;
                    } else {
                        tok *= 0x17ff41;
                        level[0] = tok as u8;
                        tok = ((tok as c_uint >> 9) & rc.wrapping_add(!(0x7ff as c_uint))) as c_int;
                        if tok != 0 {
                            rc = rc_i;
                        }
                        cf.set::<BD>(f, t_cf, rc_i as usize, tok.as_::<BD::Coef>());
                    }
                    i -= 1;
                }
                ctx = if tx_class == TxClass::TwoD {
                    0
                } else {
                    get_lo_ctx(levels, tx_class, &mut mag, lo_ctx_offsets, 0, 0, stride) as c_uint
                };
                dc_tok =
                    rav1d_msac_decode_symbol_adapt4(&mut ts_c.msac, &mut lo_cdf[ctx as usize], 3);
                if dbg {
                    println!(
                        "Post-dc_lo_tok[{}][{}][{}][{}]: r={}",
                        t_dim.ctx, chroma, ctx, dc_tok, ts_c.msac.rng,
                    );
                }
                if dc_tok == 3 {
                    if tx_class == TxClass::TwoD {
                        mag = levels[0 * stride + 1] as c_uint
                            + levels[1 * stride + 0] as c_uint
                            + levels[1 * stride + 1] as c_uint;
                    }
                    mag &= 63;
                    ctx = if mag > 12 {
                        6
                    } else {
                        mag.wrapping_add(1) >> 1
                    };
                    dc_tok = rav1d_msac_decode_hi_tok(&mut ts_c.msac, &mut hi_cdf[ctx as usize]);
                    if dbg {
                        println!(
                            "Post-dc_hi_tok[{}][{}][0][{}]: r={}",
                            cmp::min(t_dim.ctx, 3),
                            chroma,
                            dc_tok,
                            ts_c.msac.rng,
                        );
                    }
                }
            }
        }
    } else {
        // dc-only
        let tok_br =
            rav1d_msac_decode_symbol_adapt4(&mut ts_c.msac, &mut eob_cdf[0], 2 as c_int as usize)
                as c_int;
        dc_tok = (1 + tok_br) as c_uint;
        if dbg {
            println!(
                "Post-dc_lo_tok[{}][{}][{}][{}]: r={}",
                t_dim.ctx, chroma, 0, dc_tok, ts_c.msac.rng,
            );
        }
        if tok_br == 2 {
            dc_tok = rav1d_msac_decode_hi_tok(&mut ts_c.msac, &mut hi_cdf[0]);
            if dbg {
                println!(
                    "Post-dc_hi_tok[{}][{}][0][{}]: r={}",
                    cmp::min(t_dim.ctx, 3),
                    chroma,
                    dc_tok,
                    ts_c.msac.rng,
                );
            }
        }
        rc = 0;
    }

    // residual and sign
    let dq = match ts.dq.load(Ordering::Relaxed) {
        TileStateRef::Frame => &f.dq,
        TileStateRef::Local => &ts.dqmem,
    };
    let dq_tbl = &dq[b.seg_id as usize][plane];
    let qm_tbl = if *txtp < IDTX {
        f.qm[tx as usize][plane]
    } else {
        None
    };
    let dq_shift = cmp::max(0, t_dim.ctx as c_int - 2);
    let cf_max = !(!127u32
        << (match BD::BPC {
            BPC::BPC8 => 8,
            BPC::BPC16 => f.cur.p.bpc,
        })) as c_int;
    let mut cul_level: c_uint;
    let dc_sign_level: c_uint;

    if dc_tok == 0 {
        cul_level = 0;
        dc_sign_level = 1 << 6;
        if qm_tbl.is_some() {
            // goto ac_qm;
            current_block = 1669574575799829731;
        } else {
            // goto ac_noqm;
            current_block = 2404388531445638768;
        }
    } else {
        dc_sign_ctx = get_dc_sign_ctx(tx, a, l) as c_int;
        let dc_sign_cdf = &mut ts_c.cdf.coef.dc_sign[chroma][dc_sign_ctx as usize];
        dc_sign = rav1d_msac_decode_bool_adapt(&mut ts_c.msac, dc_sign_cdf) as c_int;
        if dbg {
            println!(
                "Post-dc_sign[{}][{}][{}]: r={}",
                chroma, dc_sign_ctx, dc_sign, ts_c.msac.rng,
            );
        }

        dc_dq = dq_tbl[0].load(Ordering::Relaxed) as c_int;
        dc_sign_level = (dc_sign - 1 & 2 << 6) as c_uint;

        if let Some(qm_tbl) = qm_tbl {
            dc_dq = dc_dq * qm_tbl[0] as c_int + 16 >> 5;

            if dc_tok == 15 {
                dc_tok = (read_golomb(&mut ts_c.msac)).wrapping_add(15);
                if dbg {
                    println!(
                        "Post-dc_residual[{}->{}]: r={}",
                        dc_tok.wrapping_sub(15),
                        dc_tok,
                        ts_c.msac.rng,
                    );
                }

                dc_tok &= 0xfffff;
                dc_dq = ((dc_dq as c_uint).wrapping_mul(dc_tok) & 0xffffff) as c_int;
            } else {
                dc_dq = (dc_dq as c_uint).wrapping_mul(dc_tok) as c_int;
                assert!(dc_dq <= 0xffffff);
            }
            cul_level = dc_tok;
            dc_dq >>= dq_shift;
            dc_dq = cmp::min(dc_dq, cf_max + dc_sign);
            cf.set::<BD>(
                f,
                t_cf,
                0,
                (if dc_sign != 0 { -dc_dq } else { dc_dq }).as_::<BD::Coef>(),
            );

            if rc != 0 {
                current_block = 1669574575799829731;
            } else {
                current_block = 15494703142406051947;
            }
        } else {
            // non-qmatrix is the common case and allows for additional optimizations
            if dc_tok == 15 {
                dc_tok = (read_golomb(&mut ts_c.msac)).wrapping_add(15);
                if dbg {
                    println!(
                        "Post-dc_residual[{}->{}]: r={}",
                        dc_tok.wrapping_sub(15),
                        dc_tok,
                        ts_c.msac.rng,
                    );
                }

                dc_tok &= 0xfffff;
                dc_dq = (((dc_dq as c_uint).wrapping_mul(dc_tok) & 0xffffff as c_int as c_uint)
                    >> dq_shift) as c_int;
                dc_dq = cmp::min(dc_dq, cf_max + dc_sign);
            } else {
                dc_dq = ((dc_dq as c_uint).wrapping_mul(dc_tok) >> dq_shift) as c_int;
                assert!(dc_dq <= cf_max);
            }
            cul_level = dc_tok;
            cf.set::<BD>(
                f,
                t_cf,
                0,
                (if dc_sign != 0 { -dc_dq } else { dc_dq }).as_::<BD::Coef>(),
            );

            if rc != 0 {
                current_block = 2404388531445638768;
            } else {
                current_block = 15494703142406051947;
            }
        }
    }
    match current_block {
        // ac_qm:
        1669574575799829731 => {
            let ac_dq: c_uint = dq_tbl[1].load(Ordering::Relaxed) as c_uint;
            loop {
                let sign = rav1d_msac_decode_bool_equi(&mut ts_c.msac) as c_int;
                if dbg {
                    println!("Post-sign[{}={}]: r={}", rc, sign, ts_c.msac.rng);
                }
                let rc_tok: c_uint = cf.get::<BD>(f, t_cf, rc as usize).as_::<c_uint>();
                let mut tok: c_uint;
                let mut dq: c_uint = ac_dq
                    // TODO: Remove `unwrap` once state machine control flow is cleaned up.
                    .wrapping_mul(qm_tbl.unwrap()[rc as usize] as c_uint)
                    .wrapping_add(16)
                    >> 5;
                let dq_sat;

                if rc_tok >= 15 << 11 {
                    tok = (read_golomb(&mut ts_c.msac)).wrapping_add(15);
                    if dbg {
                        println!(
                            "Post-residual[{}={}->{}]: r={}",
                            rc,
                            tok.wrapping_sub(15),
                            tok,
                            ts_c.msac.rng,
                        );
                    }

                    tok &= 0xfffff;
                    dq = dq.wrapping_mul(tok) & 0xffffff;
                } else {
                    tok = rc_tok >> 11;
                    dq = dq.wrapping_mul(tok);
                    assert!(dq <= 0xffffff);
                }
                cul_level = cul_level.wrapping_add(tok);
                dq >>= dq_shift;
                dq_sat = cmp::min(dq as c_int, cf_max + sign);
                cf.set::<BD>(
                    f,
                    t_cf,
                    rc as usize,
                    (if sign != 0 { -dq_sat } else { dq_sat }).as_::<BD::Coef>(),
                );

                rc = rc_tok & 0x3ff;
                if !(rc != 0) {
                    break;
                }
            }
        }
        // ac_noqm:
        2404388531445638768 => {
            let ac_dq: c_uint = dq_tbl[1].load(Ordering::Relaxed) as c_uint;
            loop {
                let sign = rav1d_msac_decode_bool_equi(&mut ts_c.msac) as c_int;
                if dbg {
                    println!("Post-sign[{}={}]: r={}", rc, sign, ts_c.msac.rng);
                }
                let rc_tok: c_uint = cf.get::<BD>(f, t_cf, rc as usize).as_::<c_uint>();
                let mut tok: c_uint;
                let mut dq;

                // residual
                if rc_tok >= 15 << 11 {
                    tok = (read_golomb(&mut ts_c.msac)).wrapping_add(15);
                    if dbg {
                        println!(
                            "Post-residual[{}={}->{}]: r={}",
                            rc,
                            tok.wrapping_sub(15),
                            tok,
                            ts_c.msac.rng,
                        );
                    }

                    // coefficient parsing, see 5.11.39
                    tok &= 0xfffff;

                    // dequant, see 7.12.3
                    dq = ((ac_dq.wrapping_mul(tok) & 0xffffff) >> dq_shift) as c_int;
                    dq = cmp::min(dq, cf_max + sign);
                } else {
                    // cannot exceed `cf_max`, so we can avoid the clipping
                    tok = rc_tok >> 11;
                    dq = (ac_dq.wrapping_mul(tok) >> dq_shift) as c_int;
                    assert!(dq <= cf_max);
                }
                cul_level = cul_level.wrapping_add(tok);
                cf.set::<BD>(
                    f,
                    t_cf,
                    rc as usize,
                    (if sign != 0 { -dq } else { dq }).as_::<BD::Coef>(),
                );

                rc = rc_tok & 0x3ff; // next non-zero `rc`, zero if `eob`
                if !(rc != 0) {
                    break;
                }
            }
        }
        _ => {}
    }

    // context
    *res_ctx = (cmp::min(cul_level, 63) | dc_sign_level) as u8;

    eob
}

#[derive(Clone, Copy)]
enum CfSelect {
    // Use `f.frame_thread.cf` at the specified offset.
    Frame(usize),

    // Use `t.cf`.
    Task,
}

impl CfSelect {
    unsafe fn set<BD: BitDepth>(
        self,
        f: &Rav1dFrameData,
        task_cf: &mut Cf,
        index: usize,
        value: BD::Coef,
    ) {
        match self {
            CfSelect::Frame(offset) => {
                let mut cf = unsafe { f.frame_thread.cf.mut_element_as(offset + index) };
                *cf = value;
            }
            CfSelect::Task => {
                let cf = &mut task_cf.select_mut::<BD>()[index];
                *cf = value;
            }
        };
    }

    fn get<BD: BitDepth>(self, f: &Rav1dFrameData, t_cf: &Cf, index: usize) -> BD::Coef {
        match self {
            CfSelect::Frame(offset) => *f.frame_thread.cf.element_as(offset + index),
            CfSelect::Task => t_cf.select::<BD>()[index],
        }
    }
}

unsafe fn read_coef_tree<BD: BitDepth>(
    f: &Rav1dFrameData,
    t: &mut Rav1dTaskContext,
    mut ts_c: Option<&mut Rav1dTileStateContext>,
    bs: BlockSize,
    b: &Av1Block,
    ytx: RectTxfmSize,
    depth: usize,
    tx_split: [u16; 2],
    x_off: c_int,
    y_off: c_int,
    mut dst: Option<*mut BD::Pixel>,
) {
    let ts = &f.ts[t.ts];
    let t_dim = &dav1d_txfm_dimensions[ytx as usize];
    let txw = t_dim.w;
    let txh = t_dim.h;

    // `y_off` can be larger than 3 since lossless blocks
    // use `TX_4X4` but can't be splitted.
    // Avoids an undefined left shift.
    if depth < 2 && tx_split[depth] != 0 && tx_split[depth] & 1 << y_off * 4 + x_off != 0 {
        let sub = t_dim.sub as RectTxfmSize;
        let sub_t_dim = &dav1d_txfm_dimensions[sub as usize];
        let txsw = sub_t_dim.w;
        let txsh = sub_t_dim.h;

        read_coef_tree::<BD>(
            f,
            t,
            ts_c.as_deref_mut(),
            bs,
            b,
            sub,
            depth + 1,
            tx_split,
            x_off * 2 + 0,
            y_off * 2 + 0,
            dst,
        );
        t.b.x += txsw as c_int;
        if txw >= txh && t.b.x < f.bw {
            read_coef_tree::<BD>(
                f,
                t,
                ts_c.as_deref_mut(),
                bs,
                b,
                sub,
                depth + 1,
                tx_split,
                x_off * 2 + 1,
                y_off * 2 + 0,
                dst.map(|dst| dst.add(4 * txsw as usize)),
            );
        }
        t.b.x -= txsw as c_int;
        t.b.y += txsh as c_int;
        if txh >= txw && t.b.y < f.bh {
            dst = dst.map(|dst| dst.offset(4 * txsh as isize * BD::pxstride(f.cur.stride[0])));
            read_coef_tree::<BD>(
                f,
                t,
                ts_c.as_deref_mut(),
                bs,
                b,
                sub,
                depth + 1,
                tx_split,
                x_off * 2 + 0,
                y_off * 2 + 1,
                dst,
            );
            t.b.x += txsw as c_int;
            if txw >= txh && t.b.x < f.bw {
                read_coef_tree::<BD>(
                    f,
                    t,
                    ts_c.as_deref_mut(),
                    bs,
                    b,
                    sub,
                    depth + 1,
                    tx_split,
                    x_off * 2 + 1,
                    y_off * 2 + 1,
                    dst.map(|dst| dst.add(4 * txsw as usize)),
                );
            }
            t.b.x -= txsw as c_int;
        }
        t.b.y -= txsh as c_int;
    } else {
        let bx4 = t.b.x as usize & 31;
        let by4 = t.b.y as usize & 31;
        let mut txtp = DCT_DCT;
        let mut cf_ctx = 0;
        let eob;
        let cf;
        let mut cbi_idx = 0;

        if t.frame_thread.pass != 0 {
            let p = t.frame_thread.pass & 1;
            let cf_idx = ts.frame_thread[p as usize].cf.load(Ordering::Relaxed);
            cf = CfSelect::Frame(cf_idx);
            ts.frame_thread[p as usize].cf.store(
                cf_idx + cmp::min(t_dim.w, 8) as usize * cmp::min(t_dim.h, 8) as usize * 16,
                Ordering::Relaxed,
            );
            cbi_idx = (t.b.y as isize * f.b4_stride + t.b.x as isize) as usize;
        } else {
            cf = CfSelect::Task;
        }
        if t.frame_thread.pass != 2 {
            let ts_c = ts_c.as_deref_mut().unwrap();
            eob = decode_coefs::<BD>(
                f,
                t.ts,
                ts_c,
                debug_block_info!(f, t.b),
                &mut t.scratch,
                &mut t.cf,
                &mut f.a[t.a].lcoef.index_mut(bx4..bx4 + txw as usize),
                &mut t.l.lcoef.index_mut(by4..by4 + txh as usize),
                ytx,
                bs,
                b,
                0,
                cf,
                &mut txtp,
                &mut cf_ctx,
            );
            if debug_block_info!(f, t.b) {
                println!(
                    "Post-y-cf-blk[tx={},txtp={},eob={}]: r={}",
                    ytx, txtp, eob, ts_c.msac.rng,
                );
            }
            CaseSet::<16, true>::many(
                [&t.l.lcoef, &f.a[t.a].lcoef],
                [
                    cmp::min(txh as c_int, f.bh - t.b.y) as usize,
                    cmp::min(txw as c_int, f.bw - t.b.x) as usize,
                ],
                [by4, bx4],
                |case, dir| {
                    case.set_disjoint(dir, cf_ctx);
                },
            );
            let txtp_map =
                &mut t.scratch.inter_intra_mut().ac_txtp_map.txtp_map_mut()[by4 * 32 + bx4..];
            CaseSet::<16, false>::one((), txw as usize, 0, |case, ()| {
                for txtp_map in txtp_map.chunks_mut(32).take(txh as usize) {
                    case.set(txtp_map, txtp);
                }
            });
            if t.frame_thread.pass == 1 {
                f.frame_thread.cbi[cbi_idx][0]
                    .store(CodedBlockInfo::new(eob as i16, txtp), Ordering::Relaxed);
            }
        } else {
            let cbi = f.frame_thread.cbi[cbi_idx][0].load(Ordering::Relaxed);
            eob = cbi.eob().into();
            txtp = cbi.txtp();
        }
        if t.frame_thread.pass & 1 == 0 {
            let dst = dst.unwrap();
            if eob >= 0 {
                let mut cf_guard;
                let cf = match cf {
                    CfSelect::Frame(offset) => {
                        let len =
                            cmp::min(t_dim.h as usize, 8) * 4 * cmp::min(t_dim.w as usize, 8) * 4;
                        cf_guard = f.frame_thread.cf.mut_slice_as(offset..offset + len);
                        &mut *cf_guard
                    }
                    CfSelect::Task => t.cf.select_mut::<BD>(),
                };
                if debug_block_info!(f, t.b) && DEBUG_B_PIXELS {
                    coef_dump(
                        cf,
                        cmp::min(t_dim.h as usize, 8) * 4,
                        cmp::min(t_dim.w as usize, 8) * 4,
                        3,
                        "dq",
                    );
                }
                (f.dsp.itx.itxfm_add[ytx as usize][txtp as usize])
                    .expect("non-null function pointer")(
                    dst.cast(),
                    f.cur.stride[0],
                    cf.as_mut_ptr().cast(),
                    eob,
                    f.bitdepth_max,
                );
                if debug_block_info!(f, t.b) && DEBUG_B_PIXELS {
                    hex_dump::<BD>(
                        dst,
                        f.cur.stride[0] as usize,
                        t_dim.w as usize * 4,
                        t_dim.h as usize * 4,
                        "recon",
                    );
                }
            }
        }
    };
}

pub(crate) unsafe fn rav1d_read_coef_blocks<BD: BitDepth>(
    f: &Rav1dFrameData,
    t: &mut Rav1dTaskContext,
    ts_c: &mut Rav1dTileStateContext,
    bs: BlockSize,
    b: &Av1Block,
) {
    let ss_ver = (f.cur.p.layout == Rav1dPixelLayout::I420) as u8;
    let ss_hor = (f.cur.p.layout != Rav1dPixelLayout::I444) as u8;
    let bx4 = t.b.x as usize & 31;
    let by4 = t.b.y as usize & 31;
    let cbx4 = bx4 >> ss_hor;
    let cby4 = by4 >> ss_ver;
    let b_dim = &dav1d_block_dimensions[bs as usize];
    let bw4 = b_dim[0];
    let bh4 = b_dim[1];
    let cbw4 = bw4 + ss_hor >> ss_hor;
    let cbh4 = bh4 + ss_ver >> ss_ver;
    let has_chroma = f.cur.p.layout != Rav1dPixelLayout::I400
        && (bw4 > ss_hor || t.b.x & 1 != 0)
        && (bh4 > ss_ver || t.b.y & 1 != 0);

    if b.skip != 0 {
        CaseSet::<32, false>::many(
            [&t.l, &f.a[t.a]],
            [bh4 as usize, bw4 as usize],
            [by4, bx4],
            |case, dir| {
                case.set_disjoint(&dir.lcoef, 0x40);
            },
        );
        if has_chroma {
            CaseSet::<32, false>::many(
                [&t.l, &f.a[t.a]],
                [cbh4 as usize, cbw4 as usize],
                [cby4, cbx4],
                |case, dir| {
                    for ccoef in &dir.ccoef {
                        case.set_disjoint(ccoef, 0x40)
                    }
                },
            );
        }
        return;
    }

    let ts = &f.ts[t.ts];
    let w4 = cmp::min(bw4 as c_int, f.bw - t.b.x) as u8;
    let h4 = cmp::min(bh4 as c_int, f.bh - t.b.y) as u8;
    let cw4 = w4 + ss_hor >> ss_hor;
    let ch4 = h4 + ss_ver >> ss_ver;
    assert!(t.frame_thread.pass == 1);
    assert!(b.skip == 0);
    let uv_t_dim = &dav1d_txfm_dimensions[b.uvtx as usize];
    let t_dim = &dav1d_txfm_dimensions[match &b.ii {
        Av1BlockIntraInter::Intra(intra) => intra.tx,
        Av1BlockIntraInter::Inter(inter) => inter.max_ytx,
    } as usize];

    for init_y in (0..h4).step_by(16) {
        let sub_h4 = cmp::min(h4, 16 + init_y);
        for init_x in (0..w4).step_by(16) {
            let sub_w4 = cmp::min(w4, init_x + 16);
            let mut y_off = (init_y != 0) as c_int;
            let mut y;
            let mut x;
            y = init_y;
            t.b.y += init_y as c_int;
            while y < sub_h4 {
                let cbi_idx = t.b.y as usize * f.b4_stride as usize;
                let mut x_off = (init_x != 0) as c_int;
                x = init_x;
                t.b.x += init_x as c_int;
                while x < sub_w4 {
                    match &b.ii {
                        Av1BlockIntraInter::Inter(inter) => {
                            let tx_split = [inter.tx_split0 as u16, inter.tx_split1];
                            read_coef_tree::<BD>(
                                f,
                                t,
                                Some(ts_c),
                                bs,
                                b,
                                inter.max_ytx,
                                0,
                                tx_split,
                                x_off,
                                y_off,
                                None,
                            );
                        }
                        Av1BlockIntraInter::Intra(intra) => {
                            let mut cf_ctx = 0x40;
                            let mut txtp = DCT_DCT;
                            let a_start = bx4 + x as usize;
                            let a_len = t_dim.w as usize;
                            let l_start = by4 + y as usize;
                            let l_len = t_dim.h as usize;
                            let cf_idx = ts.frame_thread[1].cf.load(Ordering::Relaxed);
                            let eob = decode_coefs::<BD>(
                                f,
                                t.ts,
                                ts_c,
                                debug_block_info!(f, t.b),
                                &mut t.scratch,
                                &mut t.cf,
                                &mut f.a[t.a].lcoef.index_mut((a_start.., ..a_len)),
                                &mut t.l.lcoef.index_mut((l_start.., ..l_len)),
                                intra.tx,
                                bs,
                                b,
                                0,
                                CfSelect::Frame(cf_idx),
                                &mut txtp,
                                &mut cf_ctx,
                            );
                            if debug_block_info!(f, t.b) {
                                println!(
                                    "Post-y-cf-blk[tx={},txtp={},eob={}]: r={}",
                                    intra.tx, txtp, eob, ts_c.msac.rng,
                                );
                            }
                            f.frame_thread.cbi[cbi_idx..][t.b.x as usize][0]
                                .store(CodedBlockInfo::new(eob as i16, txtp), Ordering::Relaxed);
                            ts.frame_thread[1].cf.store(
                                cf_idx
                                    + cmp::min(t_dim.w as usize, 8)
                                        * cmp::min(t_dim.h as usize, 8)
                                        * 16,
                                Ordering::Relaxed,
                            );
                            CaseSet::<16, true>::many(
                                [&t.l.lcoef, &f.a[t.a].lcoef],
                                [
                                    cmp::min(t_dim.h as i32, f.bh - t.b.y) as usize,
                                    cmp::min(t_dim.w as i32, f.bw - t.b.x) as usize,
                                ],
                                [by4 + y as usize, bx4 + x as usize],
                                |case, dir| {
                                    case.set_disjoint(dir, cf_ctx);
                                },
                            );
                        }
                    }
                    x += t_dim.w;
                    t.b.x += t_dim.w as c_int;
                    x_off += 1;
                }
                t.b.x -= x as c_int;
                y += t_dim.h;
                t.b.y += t_dim.h as c_int;
                y_off += 1;
            }
            t.b.y -= y as c_int;

            if !has_chroma {
                continue;
            }
            let sub_ch4 = cmp::min(ch4, init_y + 16 >> ss_ver);
            let sub_cw4 = cmp::min(cw4, init_x + 16 >> ss_hor);
            let mut pl = 0;
            while pl < 2 {
                y = init_y >> ss_ver;
                t.b.y += init_y as c_int;
                while y < sub_ch4 {
                    let cbi_idx = t.b.y as usize * f.b4_stride as usize;
                    x = init_x >> ss_hor;
                    t.b.x += init_x as c_int;
                    while x < sub_cw4 {
                        let mut cf_ctx = 0x40;
                        let mut txtp = match b.ii {
                            Av1BlockIntraInter::Intra(_) => DCT_DCT,
                            Av1BlockIntraInter::Inter(_) => {
                                t.scratch.inter_intra().ac_txtp_map.txtp_map()[(by4 as usize
                                    + (y << ss_ver) as usize)
                                    * 32
                                    + bx4
                                    + (x << ss_hor) as usize]
                            }
                        };
                        let a_start = cbx4 + x as usize;
                        let a_len = uv_t_dim.w as usize;
                        let a_ccoef = &f.a[t.a].ccoef[pl];
                        let l_start = cby4 + y as usize;
                        let l_len = uv_t_dim.h as usize;
                        let l_ccoef = &t.l.ccoef[pl];
                        let cf_idx = ts.frame_thread[1].cf.load(Ordering::Relaxed);
                        let eob = decode_coefs::<BD>(
                            f,
                            t.ts,
                            ts_c,
                            debug_block_info!(f, t.b),
                            &mut t.scratch,
                            &mut t.cf,
                            &mut a_ccoef.index_mut((a_start.., ..a_len)),
                            &mut l_ccoef.index_mut((l_start.., ..l_len)),
                            b.uvtx as RectTxfmSize,
                            bs,
                            b,
                            1 + pl,
                            CfSelect::Frame(cf_idx),
                            &mut txtp,
                            &mut cf_ctx,
                        );
                        if debug_block_info!(f, t.b) {
                            println!(
                                "Post-uv-cf-blk[pl={},tx={},txtp={},eob={}]: r={}",
                                pl, b.uvtx, txtp, eob, ts_c.msac.rng,
                            );
                        }
                        f.frame_thread.cbi[cbi_idx..][t.b.x as usize][(1 + pl) as usize]
                            .store(CodedBlockInfo::new(eob as i16, txtp), Ordering::Relaxed);
                        ts.frame_thread[1].cf.store(
                            cf_idx + uv_t_dim.w as usize * uv_t_dim.h as usize * 16,
                            Ordering::Relaxed,
                        );
                        CaseSet::<16, true>::many(
                            [l_ccoef, a_ccoef],
                            [
                                cmp::min(
                                    uv_t_dim.h as i32,
                                    f.bh - t.b.y + ss_ver as c_int >> ss_ver,
                                ) as usize,
                                cmp::min(
                                    uv_t_dim.w as i32,
                                    f.bw - t.b.x + ss_hor as c_int >> ss_hor,
                                ) as usize,
                            ],
                            [cby4 + y as usize, cbx4 as usize + x as usize],
                            |case, dir| {
                                case.set_disjoint(dir, cf_ctx);
                            },
                        );
                        x += uv_t_dim.w;
                        t.b.x += (uv_t_dim.w as c_int) << ss_hor;
                    }
                    t.b.x -= (x << ss_hor) as c_int;
                    y += uv_t_dim.h;
                    t.b.y += (uv_t_dim.h as c_int) << ss_ver;
                }
                t.b.y -= (y << ss_ver) as c_int;
                pl += 1;
            }
        }
    }
}

enum MaybeTempPixels<'tmp, BD: BitDepth, TmpStride> {
    NonTemp {
        dst: *mut BD::Pixel,
        dst_stride: isize,
    },
    Temp {
        tmp: &'tmp mut [i16],
        tmp_stride: TmpStride,
    },
}

unsafe fn mc<BD: BitDepth>(
    f: &Rav1dFrameData,
    emu_edge: &mut ScratchEmuEdge,
    b: Bxy,
    dst: MaybeTempPixels<BD, ()>,
    bw4: c_int,
    bh4: c_int,
    bx: c_int,
    by: c_int,
    pl: usize,
    mv: mv,
    refp: &Rav1dThreadPicture,
    refidx: usize,
    filter_2d: Filter2d,
) -> Result<(), ()> {
    let bd = BD::from_c(f.bitdepth_max);

    let ss_ver = (pl != 0 && f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
    let ss_hor = (pl != 0 && f.cur.p.layout != Rav1dPixelLayout::I444) as c_int;
    let h_mul = 4 >> ss_hor;
    let v_mul = 4 >> ss_ver;
    let mvx = mv.x as c_int;
    let mvy = mv.y as c_int;
    let mx = mvx & 15 >> (ss_hor == 0) as c_int;
    let my = mvy & 15 >> (ss_ver == 0) as c_int;
    let mut ref_stride = refp.p.stride[(pl != 0) as usize];
    let r#ref;

    if refp.p.p.w == f.cur.p.w && refp.p.p.h == f.cur.p.h {
        let dx = bx * h_mul + (mvx >> 3 + ss_hor);
        let dy = by * v_mul + (mvy >> 3 + ss_ver);
        let w;
        let h;

        if refp.p.data.as_ref().unwrap().data[0] != f.cur.data.as_ref().unwrap().data[0] {
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
            let emu_edge_buf = emu_edge.buf_mut::<BD>();
            (f.dsp.mc.emu_edge)(
                (bw4 * h_mul + (mx != 0) as c_int * 7) as intptr_t,
                (bh4 * v_mul + (my != 0) as c_int * 7) as intptr_t,
                w as intptr_t,
                h as intptr_t,
                (dx - (mx != 0) as c_int * 3) as intptr_t,
                (dy - (my != 0) as c_int * 3) as intptr_t,
                emu_edge_buf.as_mut_ptr().cast(),
                192 * mem::size_of::<BD::Pixel>(),
                refp.p.data.as_ref().unwrap().data[pl].cast(),
                ref_stride,
            );
            r#ref = emu_edge_buf
                .as_mut_ptr()
                .add((192 * (my != 0) as c_int * 3 + (mx != 0) as c_int * 3) as usize);
            ref_stride = 192 * ::core::mem::size_of::<BD::Pixel>() as isize;
        } else {
            r#ref = (refp.p.data.as_ref().unwrap().data[pl] as *mut BD::Pixel)
                .offset(BD::pxstride(ref_stride) * dy as isize)
                .add(dx as usize);
        }

        let w = bw4 * h_mul;
        let h = bh4 * v_mul;
        let mx = mx << (ss_hor == 0) as u8;
        let my = my << (ss_ver == 0) as u8;
        match dst {
            MaybeTempPixels::NonTemp { dst, dst_stride } => {
                f.dsp.mc.mc[filter_2d]
                    .call::<BD>(dst, dst_stride, r#ref, ref_stride, w, h, mx, my, bd);
            }
            MaybeTempPixels::Temp { tmp, tmp_stride: _ } => {
                f.dsp.mc.mct[filter_2d].call::<BD>(tmp, r#ref, ref_stride, w, h, mx, my, bd);
            }
        }
    } else {
        assert!(!ptr::eq(refp, &f.sr_cur));

        let orig_pos_y = (by * v_mul << 4) + mvy * (1 << (ss_ver == 0) as c_int);
        let orig_pos_x = (bx * h_mul << 4) + mvx * (1 << (ss_hor == 0) as c_int);

        let scale_mv = |val, scale| {
            let tmp = val as i64 * scale as i64 + ((scale - 0x4000) * 8) as i64;
            apply_sign64(((tmp.abs() + 128) >> 8) as c_int, tmp) + 32
        };

        let pos_x = scale_mv(orig_pos_x, f.svc[refidx][0].scale);
        let pos_y = scale_mv(orig_pos_y, f.svc[refidx][1].scale);
        let left = pos_x >> 10;
        let top = pos_y >> 10;
        let right = (pos_x + (bw4 * h_mul - 1) * (*f).svc[refidx][0].step >> 10) + 1;
        let bottom = (pos_y + (bh4 * v_mul - 1) * (*f).svc[refidx][1].step >> 10) + 1;

        if debug_block_info!(f, b) {
            println!(
                "Off {}x{} [{},{},{}], size {}x{} [{},{}]",
                left,
                top,
                orig_pos_x,
                f.svc[refidx][0].scale,
                refidx,
                right - left,
                bottom - top,
                f.svc[refidx][0].step,
                f.svc[refidx][1].step,
            );
        }

        let w = refp.p.p.w + ss_hor >> ss_hor;
        let h = refp.p.p.h + ss_ver >> ss_ver;
        if left < 3 || top < 3 || right + 4 > w || bottom + 4 > h {
            let emu_edge_buf = emu_edge.buf_mut::<BD>();
            (f.dsp.mc.emu_edge)(
                (right - left + 7) as intptr_t,
                (bottom - top + 7) as intptr_t,
                w as intptr_t,
                h as intptr_t,
                (left - 3) as intptr_t,
                (top - 3) as intptr_t,
                emu_edge_buf.as_mut_ptr().cast(),
                320 * mem::size_of::<BD::Pixel>(),
                refp.p.data.as_ref().unwrap().data[pl].cast(),
                ref_stride,
            );
            r#ref = emu_edge_buf.as_mut_ptr().add((320 * 3 + 3) as usize);
            ref_stride = 320 * ::core::mem::size_of::<BD::Pixel>() as isize;
            if debug_block_info!(f, b) {
                println!("Emu");
            }
        } else {
            r#ref = (refp.p.data.as_ref().unwrap().data[pl] as *mut BD::Pixel)
                .offset(BD::pxstride(ref_stride) * top as isize)
                .offset(left as isize);
        }

        let w = bw4 * h_mul;
        let h = bh4 * v_mul;
        let mx = pos_x & 0x3ff;
        let my = pos_y & 0x3ff;
        let dx = f.svc[refidx][0].step;
        let dy = f.svc[refidx][1].step;
        match dst {
            MaybeTempPixels::NonTemp { dst, dst_stride } => {
                f.dsp.mc.mc_scaled[filter_2d]
                    .call::<BD>(dst, dst_stride, r#ref, ref_stride, w, h, mx, my, dx, dy, bd);
            }
            MaybeTempPixels::Temp { tmp, tmp_stride: _ } => {
                f.dsp.mc.mct_scaled[filter_2d]
                    .call::<BD>(tmp, r#ref, ref_stride, w, h, mx, my, dx, dy, bd);
            }
        }
    }

    Ok(())
}

unsafe fn obmc<BD: BitDepth>(
    f: &Rav1dFrameData,
    t: &mut Rav1dTaskContext,
    dst: *mut BD::Pixel,
    dst_stride: ptrdiff_t,
    b_dim: &[u8; 4],
    pl: usize,
    bx4: c_int,
    by4: c_int,
    w4: c_int,
    h4: c_int,
) -> Result<(), ()> {
    assert!(t.b.x & 1 == 0 && t.b.y & 1 == 0);
    let r = &t.rt.r[(t.b.y as usize & 31) + 5 - 1..];
    let scratch = t.scratch.inter_mut();
    let lap = scratch.lap_inter.lap_mut::<BD>().as_mut_ptr();
    let ss_ver = (pl != 0 && f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
    let ss_hor = (pl != 0 && f.cur.p.layout != Rav1dPixelLayout::I444) as c_int;
    let h_mul = 4 >> ss_hor;
    let v_mul = 4 >> ss_ver;
    let ts = &f.ts[t.ts];

    if t.b.y > ts.tiling.row_start
        && (pl == 0 || b_dim[0] as c_int * h_mul + b_dim[1] as c_int * v_mul >= 16)
    {
        let mut i = 0;
        let mut x = 0;
        while x < w4 && i < cmp::min(b_dim[2], 4) {
            // only odd blocks are considered for overlap handling, hence +1
            let a_r = *f.rf.r.index(r[0] + t.b.x as usize + x as usize + 1);
            let a_b_dim = &dav1d_block_dimensions[a_r.bs as usize];
            let step4 = clip(a_b_dim[0], 2, 16);

            if a_r.r#ref.r#ref[0] > 0 {
                let ow4 = cmp::min(step4, b_dim[0]);
                let oh4 = cmp::min(b_dim[1], 16) >> 1;
                mc::<BD>(
                    f,
                    &mut scratch.emu_edge,
                    t.b,
                    MaybeTempPixels::NonTemp {
                        dst: lap,
                        dst_stride: ow4 as isize
                            * h_mul as isize
                            * ::core::mem::size_of::<BD::Pixel>() as isize,
                    },
                    ow4 as c_int,
                    oh4 as c_int * 3 + 3 >> 2,
                    t.b.x + x,
                    t.b.y,
                    pl,
                    a_r.mv.mv[0],
                    &f.refp[a_r.r#ref.r#ref[0] as usize - 1],
                    a_r.r#ref.r#ref[0] as usize - 1,
                    dav1d_filter_2d[*f.a[t.a].filter[1].index((bx4 + x + 1) as usize) as usize]
                        [*f.a[t.a].filter[0].index((bx4 + x + 1) as usize) as usize],
                )?;
                (f.dsp.mc.blend_h)(
                    dst.add((x * h_mul) as usize).cast(),
                    dst_stride,
                    lap.cast(),
                    h_mul * ow4 as c_int,
                    v_mul * oh4 as c_int,
                );
                i += 1;
            }
            x += step4 as c_int;
        }
    }

    if t.b.x > ts.tiling.col_start {
        let mut i = 0;
        let mut y = 0;
        while y < h4 && i < cmp::min(b_dim[3], 4) {
            // only odd blocks are considered for overlap handling, hence +1
            let l_r = *f.rf.r.index(r[y as usize + 1 + 1] + t.b.x as usize - 1);
            let l_b_dim = &dav1d_block_dimensions[l_r.bs as usize];
            let step4 = clip(l_b_dim[1], 2, 16);

            if l_r.r#ref.r#ref[0] > 0 {
                let ow4 = cmp::min(b_dim[0], 16) >> 1;
                let oh4 = cmp::min(step4, b_dim[1]);
                mc::<BD>(
                    f,
                    &mut scratch.emu_edge,
                    t.b,
                    MaybeTempPixels::NonTemp {
                        dst: lap,
                        dst_stride: h_mul as isize
                            * ow4 as isize
                            * ::core::mem::size_of::<BD::Pixel>() as isize,
                    },
                    ow4 as c_int,
                    oh4 as c_int,
                    t.b.x,
                    t.b.y + y,
                    pl,
                    l_r.mv.mv[0],
                    &f.refp[l_r.r#ref.r#ref[0] as usize - 1],
                    l_r.r#ref.r#ref[0] as usize - 1,
                    dav1d_filter_2d[*t.l.filter[1].index((by4 + y + 1) as usize) as usize]
                        [*t.l.filter[0].index((by4 + y + 1) as usize) as usize],
                )?;
                (f.dsp.mc.blend_v)(
                    dst.offset((y * v_mul) as isize * BD::pxstride(dst_stride))
                        .cast(),
                    dst_stride,
                    lap.cast(),
                    h_mul * ow4 as c_int,
                    v_mul * oh4 as c_int,
                );
                i += 1;
            }
            y += step4 as c_int;
        }
    }
    Ok(())
}

unsafe fn warp_affine<BD: BitDepth>(
    f: &Rav1dFrameData,
    emu_edge: &mut ScratchEmuEdge,
    b: Bxy,
    mut dst: MaybeTempPixels<BD, usize>,
    b_dim: &[u8; 4],
    pl: usize,
    refp: &Rav1dThreadPicture,
    wmp: &Rav1dWarpedMotionParams,
) -> Result<(), ()> {
    let abcd = &wmp.abcd.get();
    let bd = BD::from_c(f.bitdepth_max);

    let ss_ver = (pl != 0 && f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
    let ss_hor = (pl != 0 && f.cur.p.layout != Rav1dPixelLayout::I444) as c_int;
    let h_mul = 4 >> ss_hor;
    let v_mul = 4 >> ss_ver;
    assert!(b_dim[0] as c_int * h_mul & 7 == 0 && b_dim[1] as c_int * v_mul & 7 == 0);
    let mat = &wmp.matrix;
    let width = refp.p.p.w + ss_hor >> ss_hor;
    let height = refp.p.p.h + ss_ver >> ss_ver;

    for y in (0..b_dim[1] as c_int * v_mul).step_by(8) {
        let src_y = b.y * 4 + ((y + 4) << ss_ver);
        let mat3_y = mat[3] as i64 * src_y as i64 + mat[0] as i64;
        let mat5_y = mat[5] as i64 * src_y as i64 + mat[1] as i64;
        for x in (0..b_dim[0] as c_int * h_mul).step_by(8) {
            // Calculate transformation relative to
            // center of 8x8 block in luma pixel units.
            let src_x = b.x * 4 + ((x + 4) << ss_hor);
            let mvx = mat[2] as i64 * src_x as i64 + mat3_y >> ss_hor;
            let mvy = mat[4] as i64 * src_x as i64 + mat5_y >> ss_ver;

            let dx = (mvx >> 16) as i32 - 4;
            let mx = (mvx as i32 & 0xffff) - wmp.alpha() as i32 * 4 - wmp.beta() as i32 * 7 & !0x3f;

            let dy = (mvy >> 16) as i32 - 4;
            let my =
                (mvy as i32 & 0xffff) - wmp.gamma() as i32 * 4 - wmp.delta() as i32 * 4 & !0x3f;

            let ref_ptr;
            let mut ref_stride = refp.p.stride[(pl != 0) as usize];

            if dx < 3 || dx + 8 + 4 > width || dy < 3 || dy + 8 + 4 > height {
                let emu_edge_buf = emu_edge.buf_mut::<BD>();
                (f.dsp.mc.emu_edge)(
                    15,
                    15,
                    width as intptr_t,
                    height as intptr_t,
                    (dx - 3) as intptr_t,
                    (dy - 3) as intptr_t,
                    emu_edge_buf.as_mut_ptr().cast(),
                    32 * mem::size_of::<BD::Pixel>(),
                    refp.p.data.as_ref().unwrap().data[pl].cast(),
                    ref_stride,
                );
                ref_ptr = emu_edge_buf.as_ptr().add(32 * 3 + 3);
                ref_stride = 32 * ::core::mem::size_of::<BD::Pixel>() as isize;
            } else {
                ref_ptr = (refp.p.data.as_ref().unwrap().data[pl] as *const BD::Pixel)
                    .offset(BD::pxstride(ref_stride) * dy as isize)
                    .offset(dx as isize);
            }
            match dst {
                MaybeTempPixels::Temp {
                    ref mut tmp,
                    tmp_stride,
                } => {
                    f.dsp.mc.warp8x8t.call(
                        &mut tmp[x as usize..],
                        tmp_stride,
                        ref_ptr,
                        ref_stride,
                        abcd,
                        mx,
                        my,
                        bd,
                    );
                }
                MaybeTempPixels::NonTemp { dst, dst_stride } => {
                    f.dsp.mc.warp8x8.call(
                        dst.add(x as usize),
                        dst_stride,
                        ref_ptr,
                        ref_stride,
                        abcd,
                        mx,
                        my,
                        bd,
                    );
                }
            }
        }
        dst = match dst {
            MaybeTempPixels::NonTemp { dst, dst_stride } => MaybeTempPixels::NonTemp {
                dst: dst.offset(8 * BD::pxstride(dst_stride)),
                dst_stride,
            },
            MaybeTempPixels::Temp { tmp, tmp_stride } => MaybeTempPixels::Temp {
                tmp: &mut tmp[8 * tmp_stride..],
                tmp_stride,
            },
        };
    }
    Ok(())
}

pub(crate) unsafe fn rav1d_recon_b_intra<BD: BitDepth>(
    f: &Rav1dFrameData,
    t: &mut Rav1dTaskContext,
    mut ts_c: Option<&mut Rav1dTileStateContext>,
    bs: BlockSize,
    intra_edge_flags: EdgeFlags,
    b: &Av1Block,
    intra: &Av1BlockIntra,
) {
    let bd = BD::from_c(f.bitdepth_max);
    let ts = &f.ts[t.ts];

    let bx4 = t.b.x & 31;
    let by4 = t.b.y & 31;
    let ss_ver = (f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
    let ss_hor = (f.cur.p.layout != Rav1dPixelLayout::I444) as c_int;
    let cbx4 = bx4 >> ss_hor;
    let cby4 = by4 >> ss_ver;
    let b_dim = &dav1d_block_dimensions[bs as usize];
    let bw4 = b_dim[0] as c_int;
    let bh4 = b_dim[1] as c_int;
    let w4 = cmp::min(bw4, f.bw - t.b.x);
    let h4 = cmp::min(bh4, f.bh - t.b.y);
    let cw4 = w4 + ss_hor >> ss_hor;
    let ch4 = h4 + ss_ver >> ss_ver;
    let has_chroma = f.cur.p.layout != Rav1dPixelLayout::I400
        && (bw4 > ss_hor || t.b.x & 1 != 0)
        && (bh4 > ss_ver || t.b.y & 1 != 0);
    let t_dim = &dav1d_txfm_dimensions[intra.tx as usize];
    let uv_t_dim = &dav1d_txfm_dimensions[b.uvtx as usize];

    // coefficient coding
    let cbw4 = bw4 + ss_hor >> ss_hor;
    let cbh4 = bh4 + ss_ver >> ss_ver;

    let intra_edge_filter = f.seq_hdr.as_ref().unwrap().intra_edge_filter;
    let intra_edge_filter_flag = (intra_edge_filter as c_int) << 10;

    for init_y in (0..h4).step_by(16) {
        let sub_h4 = cmp::min(h4, 16 + init_y);
        let sub_ch4 = cmp::min(ch4, init_y + 16 >> ss_ver);
        for init_x in (0..w4).step_by(16) {
            if intra.pal_sz[0] != 0 {
                let dst: *mut BD::Pixel = (f.cur.data.as_ref().unwrap().data[0] as *mut BD::Pixel)
                    .offset(4 * (t.b.y as isize * BD::pxstride(f.cur.stride[0]) + t.b.x as isize));
                let pal_idx_guard;
                let scratch = t.scratch.inter_intra_mut();
                let pal_idx = if t.frame_thread.pass != 0 {
                    let p = (t.frame_thread.pass & 1) as usize;
                    let frame_thread = &ts.frame_thread[p];
                    let len = (bw4 * bh4 * 8) as usize;
                    let pal_idx = frame_thread.pal_idx.load(Ordering::Relaxed);
                    pal_idx_guard = f.frame_thread.pal_idx.index((pal_idx.., ..len));
                    frame_thread.pal_idx.store(pal_idx + len, Ordering::Relaxed);
                    &*pal_idx_guard
                } else {
                    &scratch.pal_idx_y
                };
                let pal_guard;
                let pal = if t.frame_thread.pass != 0 {
                    let x = t.b.x as usize;
                    let y = t.b.y as usize;
                    let index =
                        ((y >> 1) + (x & 1)) * (f.b4_stride as usize >> 1) + (x >> 1) + (y & 1);
                    pal_guard = f.frame_thread.pal.index::<BD>(index);
                    &pal_guard[0]
                } else {
                    &scratch.interintra_edge_pal.pal.buf::<BD>()[0]
                };
                f.dsp.ipred.pal_pred.call::<BD>(
                    dst,
                    f.cur.stride[0],
                    pal.as_ptr(),
                    pal_idx.as_ptr(),
                    bw4 * 4,
                    bh4 * 4,
                );
                if debug_block_info!(f, t.b) && DEBUG_B_PIXELS {
                    hex_dump::<BD>(
                        dst,
                        BD::pxstride(f.cur.stride[0]) as usize,
                        bw4 as usize * 4,
                        bh4 as usize * 4,
                        "y-pal-pred",
                    );
                }
            }

            let intra_flags = sm_flag(&f.a[t.a], bx4 as usize)
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
            t.b.y += init_y;
            while y < sub_h4 {
                let mut dst: *mut BD::Pixel =
                    (f.cur.data.as_ref().unwrap().data[0] as *mut BD::Pixel).offset(
                        4 * (t.b.y as isize * BD::pxstride(f.cur.stride[0])
                            + t.b.x as isize
                            + init_x as isize),
                    );
                x = init_x;
                t.b.x += init_x;
                while x < sub_w4 {
                    let mut angle;
                    let edge_flags;
                    let m;
                    if !(intra.pal_sz[0] != 0) {
                        angle = intra.y_angle as c_int;
                        edge_flags = EdgeFlags::union_all([
                            EdgeFlags::I444_TOP_HAS_RIGHT.select(
                                !((y > init_y || !sb_has_tr) && x + t_dim.w as c_int >= sub_w4),
                            ),
                            EdgeFlags::I444_LEFT_HAS_BOTTOM.select(
                                !(x > init_x || (!sb_has_bl && y + t_dim.h as c_int >= sub_h4)),
                            ),
                        ]);
                        let top_sb_edge_slice = if t.b.y & f.sb_step - 1 == 0 {
                            let sby = t.b.y >> f.sb_shift;
                            let offset = f.ipred_edge_off as isize * 0
                                + (f.sb128w * 128 * (sby - 1)) as isize;
                            Some((&f.ipred_edge, offset))
                        } else {
                            None
                        };
                        let edge_array = t
                            .scratch
                            .inter_intra_mut()
                            .interintra_edge_pal
                            .edge
                            .buf_mut::<BD>();
                        let edge_offset = 128;
                        let data_stride = BD::pxstride(f.cur.stride[0]);
                        let data_width = 4 * ts.tiling.col_end;
                        let data_height = 4 * ts.tiling.row_end;
                        let data_diff = (data_height - 1) as isize * data_stride;
                        let dst_slice = slice::from_raw_parts(
                            (f.cur.data.as_ref().unwrap().data[0] as *const BD::Pixel)
                                .offset(cmp::min(data_diff, 0)),
                            data_diff.unsigned_abs() + data_width as usize,
                        );
                        m = rav1d_prepare_intra_edges(
                            t.b.x,
                            t.b.x > ts.tiling.col_start,
                            t.b.y,
                            t.b.y > ts.tiling.row_start,
                            ts.tiling.col_end,
                            ts.tiling.row_end,
                            edge_flags,
                            dst_slice,
                            f.cur.stride[0],
                            top_sb_edge_slice,
                            intra.y_mode as IntraPredMode,
                            &mut angle,
                            t_dim.w as c_int,
                            t_dim.h as c_int,
                            intra_edge_filter,
                            edge_array,
                            edge_offset,
                            bd,
                        );
                        let edge = edge_array.as_ptr().add(edge_offset);
                        f.dsp.ipred.intra_pred[m as usize].call(
                            dst,
                            f.cur.stride[0],
                            edge,
                            t_dim.w as c_int * 4,
                            t_dim.h as c_int * 4,
                            angle | intra_flags,
                            4 * f.bw - 4 * t.b.x,
                            4 * f.bh - 4 * t.b.y,
                            bd,
                        );

                        if debug_block_info!(f, t.b) && DEBUG_B_PIXELS {
                            hex_dump::<BD>(
                                edge.offset(-(t_dim.h as isize * 4)),
                                t_dim.h as usize * 4,
                                t_dim.h as usize * 4,
                                2,
                                "l",
                            );
                            hex_dump::<BD>(edge, 0, 1, 1, "tl");
                            hex_dump::<BD>(
                                edge.add(1),
                                t_dim.w as usize * 4,
                                t_dim.w as usize * 4,
                                2,
                                "t",
                            );
                            hex_dump::<BD>(
                                dst,
                                f.cur.stride[0] as usize,
                                t_dim.w as usize * 4,
                                t_dim.h as usize * 4,
                                "y-intra-pred",
                            );
                        }
                    }

                    if b.skip == 0 {
                        let mut cf_guard;
                        let cf;
                        let eob;
                        let mut txtp = DCT_DCT;
                        if t.frame_thread.pass != 0 {
                            let p = (t.frame_thread.pass & 1) as usize;
                            let len = cmp::min(t_dim.h as usize, 8)
                                * 4
                                * cmp::min(t_dim.w as usize, 8)
                                * 4;
                            let cf_idx = ts.frame_thread[p].cf.load(Ordering::Relaxed);
                            cf_guard = f.frame_thread.cf.mut_slice_as(cf_idx..cf_idx + len);
                            cf = &mut *cf_guard;
                            ts.frame_thread[p].cf.store(cf_idx + len, Ordering::Relaxed);
                            let cbi = f.frame_thread.cbi
                                [t.b.y as usize * f.b4_stride as usize + t.b.x as usize][0]
                                .load(Ordering::Relaxed);
                            eob = cbi.eob().into();
                            txtp = cbi.txtp();
                        } else {
                            let mut cf_ctx = 0;
                            let a_start = (bx4 + x) as usize;
                            let l_start = (by4 + y) as usize;
                            eob = decode_coefs::<BD>(
                                f,
                                t.ts,
                                ts_c.as_deref_mut().unwrap(),
                                debug_block_info!(f, t.b),
                                &mut t.scratch,
                                &mut t.cf,
                                &mut f.a[t.a]
                                    .lcoef
                                    .index_mut(a_start..a_start + t_dim.w as usize),
                                &mut t.l.lcoef.index_mut(l_start..l_start + t_dim.h as usize),
                                intra.tx as RectTxfmSize,
                                bs,
                                b,
                                0,
                                CfSelect::Task,
                                &mut txtp,
                                &mut cf_ctx,
                            );
                            cf = t.cf.select_mut::<BD>();
                            if debug_block_info!(f, t.b) {
                                println!(
                                    "Post-y-cf-blk[tx={},txtp={},eob={}]: r={}",
                                    intra.tx,
                                    txtp,
                                    eob,
                                    ts_c.as_deref().unwrap().msac.rng,
                                );
                            }
                            CaseSet::<16, true>::many(
                                [&t.l, &f.a[t.a]],
                                [
                                    cmp::min(t_dim.h as i32, f.bh - t.b.y) as usize,
                                    cmp::min(t_dim.w as i32, f.bw - t.b.x) as usize,
                                ],
                                [(by4 + y) as usize, (bx4 + x) as usize],
                                |case, dir| {
                                    case.set_disjoint(&dir.lcoef, cf_ctx);
                                },
                            );
                        }
                        if eob >= 0 {
                            if debug_block_info!(f, t.b) && DEBUG_B_PIXELS {
                                coef_dump(
                                    cf,
                                    cmp::min(t_dim.h as usize, 8) * 4,
                                    cmp::min(t_dim.w as usize, 8) * 4,
                                    3,
                                    "dq",
                                );
                            }
                            (f.dsp.itx.itxfm_add[intra.tx as usize][txtp as usize])
                                .expect("non-null function pointer")(
                                dst.cast(),
                                f.cur.stride[0],
                                cf.as_mut_ptr().cast(),
                                eob,
                                f.bitdepth_max,
                            );
                            if debug_block_info!(f, t.b) && DEBUG_B_PIXELS {
                                hex_dump::<BD>(
                                    dst,
                                    f.cur.stride[0] as usize,
                                    t_dim.w as usize * 4,
                                    t_dim.h as usize * 4,
                                    "recon",
                                );
                            }
                        }
                    } else if t.frame_thread.pass == 0 {
                        CaseSet::<16, false>::many(
                            [&t.l, &f.a[t.a]],
                            [t_dim.h as usize, t_dim.w as usize],
                            [(by4 + y) as usize, (bx4 + x) as usize],
                            |case, dir| {
                                case.set_disjoint(&dir.lcoef, 0x40);
                            },
                        );
                    }
                    dst = dst.add(4 * t_dim.w as usize);
                    x += t_dim.w as c_int;
                    t.b.x += t_dim.w as c_int;
                }
                t.b.x -= x;
                y += t_dim.h as c_int;
                t.b.y += t_dim.h as c_int;
            }
            t.b.y -= y;

            if !has_chroma {
                continue;
            }

            let stride = f.cur.stride[1];

            if intra.uv_mode == CFL_PRED {
                assert!(init_x == 0 && init_y == 0);

                let scratch = t.scratch.inter_intra_mut();
                let ac = scratch.ac_txtp_map.ac_mut();
                let y_src = (f.cur.data.as_ref().unwrap().data[0] as *mut BD::Pixel)
                    .add((4 * (t.b.x & !ss_hor)) as usize)
                    .offset((4 * (t.b.y & !ss_ver)) as isize * BD::pxstride(f.cur.stride[0]));
                let uv_off = 4
                    * ((t.b.x >> ss_hor) as isize
                        + (t.b.y >> ss_ver) as isize * BD::pxstride(stride));
                let uv_dst = [
                    (f.cur.data.as_ref().unwrap().data[1] as *mut BD::Pixel).offset(uv_off),
                    (f.cur.data.as_ref().unwrap().data[2] as *mut BD::Pixel).offset(uv_off),
                ];

                let furthest_r = (cw4 << ss_hor) + t_dim.w as c_int - 1 & !(t_dim.w as c_int - 1);
                let furthest_b = (ch4 << ss_ver) + t_dim.h as c_int - 1 & !(t_dim.h as c_int - 1);
                f.dsp.ipred.cfl_ac[f.cur.p.layout.try_into().unwrap()].call::<BD>(
                    ac.as_mut_ptr(),
                    y_src,
                    f.cur.stride[0],
                    cbw4 - (furthest_r >> ss_hor),
                    cbh4 - (furthest_b >> ss_ver),
                    cbw4 * 4,
                    cbh4 * 4,
                );
                for pl in 0..2 {
                    if intra.cfl_alpha[pl] == 0 {
                        continue;
                    }
                    let mut angle = 0;
                    let top_sb_edge_slice = if t.b.y & !ss_ver & f.sb_step - 1 == 0 {
                        let sby = t.b.y >> f.sb_shift;
                        let offset = (f.ipred_edge_off * (pl + 1)) as isize
                            + (f.sb128w * 128 * (sby - 1)) as isize;
                        Some((&f.ipred_edge, offset))
                    } else {
                        None
                    };
                    let xpos = t.b.x >> ss_hor;
                    let ypos = t.b.y >> ss_ver;
                    let xstart = ts.tiling.col_start >> ss_hor;
                    let ystart = ts.tiling.row_start >> ss_ver;
                    let edge_array = scratch.interintra_edge_pal.edge.buf_mut::<BD>();
                    let edge_offset = 128;
                    let data_stride = BD::pxstride(f.cur.stride[1]);
                    let data_width = 4 * ts.tiling.col_end >> ss_hor;
                    let data_height = 4 * ts.tiling.row_end >> ss_ver;
                    let data_diff = (data_height - 1) as isize * data_stride;
                    let uvdst_slice = slice::from_raw_parts(
                        (f.cur.data.as_ref().unwrap().data[1 + pl] as *const BD::Pixel)
                            .offset(cmp::min(data_diff, 0)),
                        data_diff.unsigned_abs() + data_width as usize,
                    );
                    let m: IntraPredMode = rav1d_prepare_intra_edges(
                        xpos,
                        xpos > xstart,
                        ypos,
                        ypos > ystart,
                        ts.tiling.col_end >> ss_hor,
                        ts.tiling.row_end >> ss_ver,
                        EdgeFlags::empty(),
                        uvdst_slice,
                        stride,
                        top_sb_edge_slice,
                        DC_PRED,
                        &mut angle,
                        uv_t_dim.w as c_int,
                        uv_t_dim.h as c_int,
                        0,
                        edge_array,
                        edge_offset,
                        bd,
                    );
                    let edge = edge_array.as_ptr().add(edge_offset);
                    f.dsp.ipred.cfl_pred[m as usize].call(
                        uv_dst[pl],
                        stride,
                        edge,
                        uv_t_dim.w as c_int * 4,
                        uv_t_dim.h as c_int * 4,
                        ac.as_mut_ptr(),
                        intra.cfl_alpha[pl] as c_int,
                        bd,
                    );
                }
                if debug_block_info!(&*f, t.b) && DEBUG_B_PIXELS {
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
            } else if intra.pal_sz[1] != 0 {
                let uv_dstoff = 4
                    * ((t.b.x >> ss_hor) as isize
                        + (t.b.y >> ss_ver) as isize * BD::pxstride(f.cur.stride[1]));
                let pal_idx_guard;
                let pal_guard;
                let (pal, pal_idx) = if t.frame_thread.pass != 0 {
                    let p = (t.frame_thread.pass & 1) as usize;
                    let x = t.b.x as usize;
                    let y = t.b.y as usize;
                    let index =
                        ((y >> 1) + (x & 1)) * (f.b4_stride as usize >> 1) + (x >> 1) + (y & 1);
                    let pal_idx_offset = ts.frame_thread[p].pal_idx.load(Ordering::Relaxed);
                    let len = (cbw4 * cbh4 * 8) as usize;
                    pal_idx_guard = f
                        .frame_thread
                        .pal_idx
                        .index(pal_idx_offset..pal_idx_offset + len);
                    ts.frame_thread[p]
                        .pal_idx
                        .store(pal_idx_offset + len, Ordering::Relaxed);
                    pal_guard = f.frame_thread.pal.index::<BD>(index);
                    (&*pal_guard, &*pal_idx_guard)
                } else {
                    let scratch = t.scratch.inter_intra_mut();
                    (
                        scratch.interintra_edge_pal.pal.buf::<BD>(),
                        scratch.pal_idx_uv.as_slice(),
                    )
                };

                f.dsp.ipred.pal_pred.call::<BD>(
                    (f.cur.data.as_ref().unwrap().data[1] as *mut BD::Pixel).offset(uv_dstoff),
                    f.cur.stride[1],
                    pal[1].as_ptr(),
                    pal_idx.as_ptr(),
                    cbw4 * 4,
                    cbh4 * 4,
                );
                f.dsp.ipred.pal_pred.call::<BD>(
                    (f.cur.data.as_ref().unwrap().data[2] as *mut BD::Pixel).offset(uv_dstoff),
                    f.cur.stride[1],
                    pal[2].as_ptr(),
                    pal_idx.as_ptr(),
                    cbw4 * 4,
                    cbh4 * 4,
                );
                if debug_block_info!(f, t.b) && DEBUG_B_PIXELS {
                    hex_dump::<BD>(
                        (f.cur.data.as_ref().unwrap().data[1] as *mut BD::Pixel).offset(uv_dstoff),
                        BD::pxstride(f.cur.stride[1] as usize),
                        cbw4 as usize * 4,
                        cbh4 as usize * 4,
                        "u-pal-pred",
                    );
                    hex_dump::<BD>(
                        (f.cur.data.as_ref().unwrap().data[2] as *mut BD::Pixel).offset(uv_dstoff),
                        BD::pxstride(f.cur.stride[1] as usize),
                        cbw4 as usize * 4,
                        cbh4 as usize * 4,
                        "v-pal-pred",
                    );
                }
            }

            let sm_uv_fl =
                sm_uv_flag(&f.a[t.a], cbx4 as usize) | sm_uv_flag(&mut t.l, cby4 as usize);
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
            for pl in 0..2 {
                y = init_y >> ss_ver;
                t.b.y += init_y;
                while y < sub_ch4 {
                    let mut dst = (f.cur.data.as_ref().unwrap().data[(1 + pl) as usize]
                        as *mut BD::Pixel)
                        .offset(
                            4 * ((t.b.y >> ss_ver) as isize * BD::pxstride(stride)
                                + (t.b.x + init_x >> ss_hor) as isize),
                        );
                    x = init_x >> ss_hor;
                    t.b.x += init_x;
                    while x < sub_cw4 {
                        let mut angle;
                        let edge_flags;
                        let uv_mode;
                        let xpos;
                        let ypos;
                        let xstart;
                        let ystart;
                        let m;
                        if !(intra.uv_mode == CFL_PRED && intra.cfl_alpha[pl] != 0
                            || intra.pal_sz[1] != 0)
                        {
                            angle = intra.uv_angle as c_int;
                            // This probably looks weird because we're using
                            // luma flags in a chroma loop, but that's because
                            // `rav1d_prepare_intra_edges` expects luma flags as input.
                            edge_flags = EdgeFlags::I444_TOP_HAS_RIGHT.select(
                                !((y > init_y >> ss_ver || !uv_sb_has_tr)
                                    && x + uv_t_dim.w as c_int >= sub_cw4),
                            ) | EdgeFlags::I444_LEFT_HAS_BOTTOM.select(
                                !(x > init_x >> ss_hor
                                    || !uv_sb_has_bl && y + uv_t_dim.h as c_int >= sub_ch4),
                            );
                            let top_sb_edge_slice = if t.b.y & !ss_ver & f.sb_step - 1 == 0 {
                                let sby = t.b.y >> f.sb_shift;
                                let offset = (f.ipred_edge_off * (1 + pl)) as isize
                                    + (f.sb128w * 128 * (sby - 1)) as isize;
                                Some((&f.ipred_edge, offset))
                            } else {
                                None
                            };
                            uv_mode = if intra.uv_mode == CFL_PRED {
                                DC_PRED
                            } else {
                                intra.uv_mode
                            };
                            xpos = t.b.x >> ss_hor;
                            ypos = t.b.y >> ss_ver;
                            xstart = ts.tiling.col_start >> ss_hor;
                            ystart = ts.tiling.row_start >> ss_ver;
                            let edge_array = t
                                .scratch
                                .inter_intra_mut()
                                .interintra_edge_pal
                                .edge
                                .buf_mut::<BD>();
                            let edge_offset = 128;
                            let data_stride = BD::pxstride(f.cur.stride[1]);
                            let data_width = 4 * ts.tiling.col_end >> ss_hor;
                            let data_height = 4 * ts.tiling.row_end >> ss_ver;
                            let data_diff = (data_height - 1) as isize * data_stride;
                            let dstuv_slice = slice::from_raw_parts(
                                (f.cur.data.as_ref().unwrap().data[1 + pl as usize]
                                    as *const BD::Pixel)
                                    .offset(cmp::min(data_diff, 0)),
                                data_diff.unsigned_abs() + data_width as usize,
                            );
                            m = rav1d_prepare_intra_edges(
                                xpos,
                                xpos > xstart,
                                ypos,
                                ypos > ystart,
                                ts.tiling.col_end >> ss_hor,
                                ts.tiling.row_end >> ss_ver,
                                edge_flags,
                                dstuv_slice,
                                stride,
                                top_sb_edge_slice,
                                uv_mode,
                                &mut angle,
                                uv_t_dim.w as c_int,
                                uv_t_dim.h as c_int,
                                intra_edge_filter,
                                edge_array,
                                edge_offset,
                                bd,
                            );
                            angle |= intra_edge_filter_flag;
                            let edge = edge_array.as_ptr().add(edge_offset);
                            f.dsp.ipred.intra_pred[m as usize].call(
                                dst,
                                stride,
                                edge,
                                uv_t_dim.w as c_int * 4,
                                uv_t_dim.h as c_int * 4,
                                angle | sm_uv_fl,
                                4 * f.bw + ss_hor - 4 * (t.b.x & !ss_hor) >> ss_hor,
                                4 * f.bh + ss_ver - 4 * (t.b.y & !ss_ver) >> ss_ver,
                                bd,
                            );
                            if debug_block_info!(f, t.b) && DEBUG_B_PIXELS {
                                hex_dump::<BD>(
                                    edge.offset(-(uv_t_dim.h as isize * 4)),
                                    uv_t_dim.h as usize * 4,
                                    uv_t_dim.h as usize * 4,
                                    2,
                                    "l",
                                );
                                hex_dump::<BD>(edge, 0, 1, 1, "tl");
                                hex_dump::<BD>(
                                    edge.add(1),
                                    uv_t_dim.w as usize * 4,
                                    uv_t_dim.w as usize * 4,
                                    2,
                                    "t",
                                );
                                hex_dump::<BD>(
                                    dst,
                                    stride as usize,
                                    uv_t_dim.w as usize * 4,
                                    uv_t_dim.h as usize * 4,
                                    if pl != 0 {
                                        "v-intra-pred"
                                    } else {
                                        "u-intra-pred"
                                    },
                                );
                            }
                        }

                        if b.skip == 0 {
                            let mut txtp = DCT_DCT;
                            let eob;
                            let mut cf_guard;
                            let cf;
                            if t.frame_thread.pass != 0 {
                                let p = (t.frame_thread.pass & 1) as usize;
                                let len = uv_t_dim.w as usize * 4 * uv_t_dim.h as usize * 4;
                                let cf_idx = ts.frame_thread[p].cf.load(Ordering::Relaxed);
                                cf_guard = f.frame_thread.cf.mut_slice_as(cf_idx..cf_idx + len);
                                cf = &mut *cf_guard;
                                ts.frame_thread[p].cf.store(cf_idx + len, Ordering::Relaxed);
                                let cbi = f.frame_thread.cbi
                                    [t.b.y as usize * f.b4_stride as usize + t.b.x as usize]
                                    [pl + 1]
                                    .load(Ordering::Relaxed);
                                eob = cbi.eob().into();
                                txtp = cbi.txtp();
                            } else {
                                let mut cf_ctx: u8 = 0;
                                let a_start = (cbx4 + x) as usize;
                                let a_ccoef = &f.a[t.a].ccoef[pl];
                                let l_start = (cby4 + y) as usize;
                                let l_ccoef = &t.l.ccoef[pl];
                                eob = decode_coefs::<BD>(
                                    f,
                                    t.ts,
                                    ts_c.as_deref_mut().unwrap(),
                                    debug_block_info!(f, t.b),
                                    &mut t.scratch,
                                    &mut t.cf,
                                    &mut a_ccoef.index_mut(a_start..a_start + uv_t_dim.w as usize),
                                    &mut l_ccoef.index_mut(l_start..l_start + uv_t_dim.h as usize),
                                    b.uvtx as RectTxfmSize,
                                    bs,
                                    b,
                                    1 + pl,
                                    CfSelect::Task,
                                    &mut txtp,
                                    &mut cf_ctx,
                                );
                                cf = t.cf.select_mut::<BD>();
                                if debug_block_info!(f, t.b) {
                                    println!(
                                            "Post-uv-cf-blk[pl={},tx={},txtp={},eob={}]: r={} [x={},cbx4={}]",
                                            pl,
                                            b.uvtx,
                                            txtp,
                                            eob,
                                            ts_c.as_deref().unwrap().msac.rng,
                                            x,
                                            cbx4,
                                        );
                                }
                                CaseSet::<16, true>::many(
                                    [l_ccoef, a_ccoef],
                                    [
                                        cmp::min(uv_t_dim.h as i32, f.bh - t.b.y + ss_ver >> ss_ver)
                                            as usize,
                                        cmp::min(uv_t_dim.w as i32, f.bw - t.b.x + ss_hor >> ss_hor)
                                            as usize,
                                    ],
                                    [(cby4 + y) as usize, (cbx4 + x) as usize],
                                    |case, dir| {
                                        case.set_disjoint(dir, cf_ctx);
                                    },
                                );
                            }
                            if eob >= 0 {
                                if debug_block_info!(f, t.b) && DEBUG_B_PIXELS {
                                    coef_dump(
                                        cf,
                                        uv_t_dim.h as usize * 4,
                                        uv_t_dim.w as usize * 4,
                                        3,
                                        "dq",
                                    );
                                }
                                (f.dsp.itx.itxfm_add[b.uvtx as usize][txtp as usize])
                                    .expect("non-null function pointer")(
                                    dst.cast(),
                                    stride,
                                    cf.as_mut_ptr().cast(),
                                    eob,
                                    f.bitdepth_max,
                                );
                                if debug_block_info!(f, t.b) && DEBUG_B_PIXELS {
                                    hex_dump::<BD>(
                                        dst,
                                        stride as usize,
                                        uv_t_dim.w as usize * 4,
                                        uv_t_dim.h as usize * 4,
                                        "recon",
                                    );
                                }
                            }
                        } else if t.frame_thread.pass == 0 {
                            CaseSet::<16, false>::many(
                                [&t.l, &f.a[t.a]],
                                [uv_t_dim.h as usize, uv_t_dim.w as usize],
                                [(cby4 + y) as usize, (cbx4 + x) as usize],
                                |case, dir| {
                                    case.set_disjoint(&dir.ccoef[pl], 0x40);
                                },
                            );
                        }
                        dst = dst.add(uv_t_dim.w as usize * 4);
                        x += uv_t_dim.w as c_int;
                        t.b.x += (uv_t_dim.w as c_int) << ss_hor;
                    }
                    t.b.x -= x << ss_hor;
                    y += uv_t_dim.h as c_int;
                    t.b.y += (uv_t_dim.h as c_int) << ss_ver;
                }
                t.b.y -= y << ss_ver;
            }
        }
    }
}

pub(crate) unsafe fn rav1d_recon_b_inter<BD: BitDepth>(
    f: &Rav1dFrameData,
    t: &mut Rav1dTaskContext,
    mut ts_c: Option<&mut Rav1dTileStateContext>,
    bs: BlockSize,
    b: &Av1Block,
    inter: &Av1BlockInter,
) -> Result<(), ()> {
    let bd = BD::from_c(f.bitdepth_max);

    let ts = &f.ts[t.ts];
    let bx4 = t.b.x & 31;
    let by4 = t.b.y & 31;
    let ss_ver = (f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
    let ss_hor = (f.cur.p.layout != Rav1dPixelLayout::I444) as c_int;
    let cbx4 = bx4 >> ss_hor;
    let cby4 = by4 >> ss_ver;
    let b_dim = &dav1d_block_dimensions[bs as usize];
    let bw4 = b_dim[0] as c_int;
    let bh4 = b_dim[1] as c_int;
    let w4 = cmp::min(bw4, f.bw - t.b.x);
    let h4 = cmp::min(bh4, f.bh - t.b.y);
    let has_chroma = f.cur.p.layout != Rav1dPixelLayout::I400
        && (bw4 > ss_hor || t.b.x & 1 != 0)
        && (bh4 > ss_ver || t.b.y & 1 != 0);
    let chr_layout_idx = if f.cur.p.layout == Rav1dPixelLayout::I400 {
        Rav1dPixelLayout::I400
    } else {
        Rav1dPixelLayout::I444 - f.cur.p.layout
    } as usize;
    let chr_layout_idx_w_mask = f
        .cur
        .p
        .layout
        .try_into()
        .unwrap_or(Rav1dPixelLayoutSubSampled::I444);

    // prediction
    let cbh4 = bh4 + ss_ver >> ss_ver;
    let cbw4 = bw4 + ss_hor >> ss_hor;
    let mut dst = (f.cur.data.as_ref().unwrap().data[0] as *mut BD::Pixel)
        .offset(4 * (t.b.y as isize * BD::pxstride(f.cur.stride[0]) + t.b.x as isize));
    let uvdstoff = 4
        * ((t.b.x >> ss_hor) as isize + (t.b.y >> ss_ver) as isize * BD::pxstride(f.cur.stride[1]));
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    if frame_hdr.frame_type.is_key_or_intra() {
        // intrabc
        assert!(!frame_hdr.size.super_res.enabled);
        let scratch = t.scratch.inter_mut();
        mc::<BD>(
            f,
            &mut scratch.emu_edge,
            t.b,
            MaybeTempPixels::NonTemp {
                dst,
                dst_stride: f.cur.stride[0],
            },
            bw4,
            bh4,
            t.b.x,
            t.b.y,
            0,
            inter.nd.one_d.mv[0],
            &f.sr_cur,
            0, // unused
            Filter2d::Bilinear,
        )?;
        if has_chroma {
            for pl in 1..3 {
                mc::<BD>(
                    f,
                    &mut scratch.emu_edge,
                    t.b,
                    MaybeTempPixels::NonTemp {
                        dst: (f.cur.data.as_ref().unwrap().data[pl] as *mut BD::Pixel)
                            .offset(uvdstoff),
                        dst_stride: f.cur.stride[1],
                    },
                    bw4 << (bw4 == ss_hor) as c_int,
                    bh4 << (bh4 == ss_ver) as c_int,
                    t.b.x & !ss_hor,
                    t.b.y & !ss_ver,
                    pl,
                    inter.nd.one_d.mv[0],
                    &f.sr_cur,
                    0, // unused
                    Filter2d::Bilinear,
                )?;
            }
        }
    } else if let Some(comp_inter_type) = inter.comp_type {
        let filter_2d = inter.filter2d;
        // Maximum super block size is 128x128
        let scratch = t.scratch.inter_mut();
        let scratch_inter = scratch.lap_inter.inter_mut();
        let tmp = &mut scratch_inter.compinter;
        let mut jnt_weight = 0;
        let seg_mask = &mut scratch_inter.seg_mask;

        for i in 0..2 {
            let refp = &f.refp[inter.r#ref[i] as usize];

            if inter.inter_mode == GLOBALMV_GLOBALMV
                && f.gmv_warp_allowed[inter.r#ref[i] as usize] != 0
            {
                warp_affine::<BD>(
                    f,
                    &mut scratch.emu_edge,
                    t.b,
                    MaybeTempPixels::Temp {
                        tmp: &mut tmp[i],
                        tmp_stride: bw4 as usize * 4,
                    },
                    b_dim,
                    0,
                    refp,
                    &frame_hdr.gmv[inter.r#ref[i] as usize],
                )?;
            } else {
                mc::<BD>(
                    f,
                    &mut scratch.emu_edge,
                    t.b,
                    MaybeTempPixels::Temp {
                        tmp: &mut tmp[i],
                        tmp_stride: (),
                    },
                    bw4,
                    bh4,
                    t.b.x,
                    t.b.y,
                    0,
                    inter.nd.one_d.mv[i],
                    refp,
                    inter.r#ref[i] as usize,
                    filter_2d,
                )?;
            }
        }

        let mut mask = &[][..];
        match comp_inter_type {
            CompInterType::Avg => {
                (f.dsp.mc.avg)(
                    dst.cast(),
                    f.cur.stride[0],
                    &tmp[0],
                    &tmp[1],
                    bw4 * 4,
                    bh4 * 4,
                    f.bitdepth_max,
                );
            }
            CompInterType::WeightedAvg => {
                jnt_weight =
                    f.jnt_weights[inter.r#ref[0] as usize][inter.r#ref[1] as usize] as c_int;
                (f.dsp.mc.w_avg)(
                    dst.cast(),
                    f.cur.stride[0],
                    &tmp[0],
                    &tmp[1],
                    bw4 * 4,
                    bh4 * 4,
                    jnt_weight,
                    f.bitdepth_max,
                );
            }
            CompInterType::Seg => {
                f.dsp.mc.w_mask[chr_layout_idx_w_mask].call(
                    dst,
                    f.cur.stride[0],
                    &tmp[inter.nd.one_d.mask_sign as usize],
                    &tmp[(inter.nd.one_d.mask_sign == 0) as usize],
                    bw4 * 4,
                    bh4 * 4,
                    seg_mask.as_mut_ptr(),
                    inter.nd.one_d.mask_sign as c_int,
                    bd,
                );
                mask = &seg_mask[..];
            }
            CompInterType::Wedge => {
                mask = dav1d_wedge_masks[bs as usize][0][0][inter.nd.one_d.wedge_idx as usize];
                (f.dsp.mc.mask)(
                    dst.cast(),
                    f.cur.stride[0],
                    &tmp[inter.nd.one_d.mask_sign as usize],
                    &tmp[(inter.nd.one_d.mask_sign == 0) as usize],
                    bw4 * 4,
                    bh4 * 4,
                    mask.as_ptr(),
                    f.bitdepth_max,
                );
                if has_chroma {
                    mask = dav1d_wedge_masks[bs as usize][chr_layout_idx]
                        [inter.nd.one_d.mask_sign as usize]
                        [inter.nd.one_d.wedge_idx as usize];
                }
            }
        }

        // chroma
        if has_chroma {
            for pl in 0..2 {
                for i in 0..2 {
                    let refp = &f.refp[inter.r#ref[i] as usize];
                    if inter.inter_mode == GLOBALMV_GLOBALMV
                        && cmp::min(cbw4, cbh4) > 1
                        && f.gmv_warp_allowed[inter.r#ref[i] as usize] != 0
                    {
                        warp_affine::<BD>(
                            f,
                            &mut scratch.emu_edge,
                            t.b,
                            MaybeTempPixels::Temp {
                                tmp: &mut tmp[i],
                                tmp_stride: bw4 as usize * 4 >> ss_hor,
                            },
                            b_dim,
                            1 + pl,
                            refp,
                            &frame_hdr.gmv[inter.r#ref[i] as usize],
                        )?;
                    } else {
                        mc::<BD>(
                            f,
                            &mut scratch.emu_edge,
                            t.b,
                            MaybeTempPixels::Temp {
                                tmp: &mut tmp[i],
                                tmp_stride: (),
                            },
                            bw4,
                            bh4,
                            t.b.x,
                            t.b.y,
                            1 + pl,
                            inter.nd.one_d.mv[i],
                            refp,
                            inter.r#ref[i] as usize,
                            filter_2d,
                        )?;
                    }
                }

                let uvdst =
                    (f.cur.data.as_ref().unwrap().data[1 + pl] as *mut BD::Pixel).offset(uvdstoff);
                match comp_inter_type {
                    CompInterType::Avg => {
                        (f.dsp.mc.avg)(
                            uvdst.cast(),
                            f.cur.stride[1],
                            &tmp[0],
                            &tmp[1],
                            bw4 * 4 >> ss_hor,
                            bh4 * 4 >> ss_ver,
                            f.bitdepth_max,
                        );
                    }
                    CompInterType::WeightedAvg => {
                        (f.dsp.mc.w_avg)(
                            uvdst.cast(),
                            f.cur.stride[1],
                            &tmp[0],
                            &tmp[1],
                            bw4 * 4 >> ss_hor,
                            bh4 * 4 >> ss_ver,
                            jnt_weight,
                            f.bitdepth_max,
                        );
                    }
                    CompInterType::Seg | CompInterType::Wedge => {
                        (f.dsp.mc.mask)(
                            uvdst.cast(),
                            f.cur.stride[1],
                            &tmp[inter.nd.one_d.mask_sign as usize],
                            &tmp[(inter.nd.one_d.mask_sign == 0) as usize],
                            bw4 * 4 >> ss_hor,
                            bh4 * 4 >> ss_ver,
                            mask.as_ptr(),
                            f.bitdepth_max,
                        );
                    }
                }
            }
        }
    } else {
        let refp = &f.refp[inter.r#ref[0] as usize];
        let filter_2d = inter.filter2d;

        if cmp::min(bw4, bh4) > 1
            && (inter.inter_mode == GLOBALMV && f.gmv_warp_allowed[inter.r#ref[0] as usize] != 0
                || inter.motion_mode == MotionMode::Warp
                    && t.warpmv.r#type > Rav1dWarpedMotionType::Translation)
        {
            warp_affine::<BD>(
                f,
                &mut t.scratch.inter_mut().emu_edge,
                t.b,
                MaybeTempPixels::NonTemp {
                    dst,
                    dst_stride: f.cur.stride[0],
                },
                b_dim,
                0,
                refp,
                if inter.motion_mode == MotionMode::Warp {
                    &t.warpmv
                } else {
                    &frame_hdr.gmv[inter.r#ref[0] as usize]
                },
            )?;
        } else {
            mc::<BD>(
                f,
                &mut t.scratch.inter_mut().emu_edge,
                t.b,
                MaybeTempPixels::NonTemp {
                    dst,
                    dst_stride: f.cur.stride[0],
                },
                bw4,
                bh4,
                t.b.x,
                t.b.y,
                0,
                inter.nd.one_d.mv[0],
                refp,
                inter.r#ref[0] as usize,
                filter_2d,
            )?;
            if inter.motion_mode == MotionMode::Obmc {
                obmc::<BD>(f, t, dst, f.cur.stride[0], b_dim, 0, bx4, by4, w4, h4)?;
            }
        }
        if let Some(interintra_type) = inter.interintra_type {
            let interintra_edge_pal = &mut t.scratch.inter_intra_mut().interintra_edge_pal;
            let tl_edge_array = interintra_edge_pal.edge.buf_mut::<BD>();
            let tl_edge_offset = 32;
            let mut m = match inter.nd.one_d.interintra_mode.get() {
                InterIntraPredMode::Smooth => SMOOTH_PRED,
                mode => mode as IntraPredMode,
            };
            let mut angle = 0;
            let top_sb_edge_slice = if t.b.y & f.sb_step - 1 == 0 {
                let sby = t.b.y >> f.sb_shift;
                let offset =
                    (f.ipred_edge_off * 0) as isize + (f.sb128w * 128 * (sby - 1)) as isize;
                Some((&f.ipred_edge, offset))
            } else {
                None
            };
            let data_stride = BD::pxstride(f.cur.stride[0]);
            let data_width = 4 * ts.tiling.col_end;
            let data_height = 4 * ts.tiling.row_end;
            let data_diff = (data_height - 1) as isize * data_stride;
            let dst_slice = slice::from_raw_parts(
                (f.cur.data.as_ref().unwrap().data[0] as *const BD::Pixel)
                    .offset(cmp::min(data_diff, 0)),
                data_diff.unsigned_abs() + data_width as usize,
            );
            m = rav1d_prepare_intra_edges(
                t.b.x,
                t.b.x > ts.tiling.col_start,
                t.b.y,
                t.b.y > ts.tiling.row_start,
                ts.tiling.col_end,
                ts.tiling.row_end,
                EdgeFlags::empty(),
                dst_slice,
                f.cur.stride[0],
                top_sb_edge_slice,
                m,
                &mut angle,
                bw4,
                bh4,
                0,
                tl_edge_array,
                tl_edge_offset,
                bd,
            );
            let tl_edge = &tl_edge_array[tl_edge_offset..];
            let tmp = interintra_edge_pal.interintra.buf_mut::<BD>();
            f.dsp.ipred.intra_pred[m as usize].call(
                tmp.as_mut_ptr(),
                4 * bw4 as isize * ::core::mem::size_of::<BD::Pixel>() as isize,
                tl_edge.as_ptr(),
                bw4 * 4,
                bh4 * 4,
                0,
                0,
                0,
                bd,
            );
            let ii_mask = match interintra_type {
                InterIntraType::Blend => {
                    dav1d_ii_masks[bs as usize][0][inter.nd.one_d.interintra_mode.get() as usize]
                }
                InterIntraType::Wedge => {
                    dav1d_wedge_masks[bs as usize][0][0][inter.nd.one_d.wedge_idx as usize]
                }
            };
            (f.dsp.mc.blend)(
                dst.cast(),
                f.cur.stride[0],
                tmp.as_mut_ptr().cast(),
                bw4 * 4,
                bh4 * 4,
                ii_mask.as_ptr(),
            );
        }

        if has_chroma {
            // sub8x8 derivation
            let mut is_sub8x8 = bw4 == ss_hor || bh4 == ss_ver;
            let r = if is_sub8x8 {
                assert!(ss_hor == 1);
                let r = &t.rt.r[(t.b.y as usize & 31) + 5 - 1..];
                if bw4 == 1 {
                    is_sub8x8 &= f.rf.r.index(r[1] + t.b.x as usize - 1).r#ref.r#ref[0] > 0;
                }
                if bh4 == ss_ver {
                    is_sub8x8 &= f.rf.r.index(r[0] + t.b.x as usize).r#ref.r#ref[0] > 0;
                }
                if bw4 == 1 && bh4 == ss_ver {
                    is_sub8x8 &= f.rf.r.index(r[0] + t.b.x as usize - 1).r#ref.r#ref[0] > 0;
                }
                r
            } else {
                &[] // Never actually used.
            };

            // chroma prediction
            if is_sub8x8 {
                let mut h_off = 0isize;
                let mut v_off = 0isize;
                if bw4 == 1 && bh4 == ss_ver {
                    for pl in 0..2 {
                        let r = *f.rf.r.index(r[0] + t.b.x as usize - 1);
                        mc::<BD>(
                            f,
                            &mut t.scratch.inter_mut().emu_edge,
                            t.b,
                            MaybeTempPixels::NonTemp {
                                dst: (f.cur.data.as_ref().unwrap().data[1 + pl] as *mut BD::Pixel)
                                    .offset(uvdstoff),
                                dst_stride: f.cur.stride[1],
                            },
                            bw4,
                            bh4,
                            t.b.x - 1,
                            t.b.y - 1,
                            1 + pl,
                            r.mv.mv[0],
                            &f.refp[r.r#ref.r#ref[0] as usize - 1],
                            r.r#ref.r#ref[0] as usize - 1,
                            if t.frame_thread.pass != 2 {
                                t.tl_4x4_filter
                            } else {
                                f.frame_thread
                                    .b
                                    .index(
                                        (t.b.y as usize - 1) * f.b4_stride as usize
                                            + t.b.x as usize
                                            - 1,
                                    )
                                    .ii
                                    .filter2d()
                            },
                        )?;
                    }
                    v_off = 2 * BD::pxstride(f.cur.stride[1]);
                    h_off = 2;
                }
                if bw4 == 1 {
                    let left_filter_2d = dav1d_filter_2d
                        [*t.l.filter[1].index(by4 as usize) as usize]
                        [*t.l.filter[0].index(by4 as usize) as usize];
                    for pl in 0..2 {
                        let r = *f.rf.r.index(r[1] + t.b.x as usize - 1);
                        mc::<BD>(
                            f,
                            &mut t.scratch.inter_mut().emu_edge,
                            t.b,
                            MaybeTempPixels::NonTemp {
                                dst: (f.cur.data.as_ref().unwrap().data[1 + pl] as *mut BD::Pixel)
                                    .offset(uvdstoff + v_off),
                                dst_stride: f.cur.stride[1],
                            },
                            bw4,
                            bh4,
                            t.b.x - 1,
                            t.b.y,
                            1 + pl,
                            r.mv.mv[0],
                            &f.refp[r.r#ref.r#ref[0] as usize - 1],
                            r.r#ref.r#ref[0] as usize - 1,
                            if t.frame_thread.pass != 2 {
                                left_filter_2d
                            } else {
                                f.frame_thread
                                    .b
                                    .index(
                                        t.b.y as usize * f.b4_stride as usize + t.b.x as usize - 1,
                                    )
                                    .ii
                                    .filter2d()
                            },
                        )?;
                    }
                    h_off = 2;
                }
                if bh4 == ss_ver {
                    let top_filter_2d = dav1d_filter_2d
                        [*f.a[t.a].filter[1].index(bx4 as usize) as usize]
                        [*f.a[t.a].filter[0].index(bx4 as usize) as usize];
                    for pl in 0..2 {
                        let r = *f.rf.r.index(r[0] + t.b.x as usize);
                        mc::<BD>(
                            f,
                            &mut t.scratch.inter_mut().emu_edge,
                            t.b,
                            MaybeTempPixels::NonTemp {
                                dst: (f.cur.data.as_ref().unwrap().data[1 + pl] as *mut BD::Pixel)
                                    .offset(uvdstoff + h_off),
                                dst_stride: f.cur.stride[1],
                            },
                            bw4,
                            bh4,
                            t.b.x,
                            t.b.y - 1,
                            1 + pl,
                            r.mv.mv[0],
                            &f.refp[r.r#ref.r#ref[0] as usize - 1],
                            r.r#ref.r#ref[0] as usize - 1,
                            if t.frame_thread.pass != 2 {
                                top_filter_2d
                            } else {
                                f.frame_thread
                                    .b
                                    .index(
                                        (t.b.y as usize - 1) * f.b4_stride as usize
                                            + t.b.x as usize,
                                    )
                                    .ii
                                    .filter2d()
                            },
                        )?;
                    }
                    v_off = 2 * BD::pxstride(f.cur.stride[1]);
                }
                for pl in 0..2 {
                    mc::<BD>(
                        f,
                        &mut t.scratch.inter_mut().emu_edge,
                        t.b,
                        MaybeTempPixels::NonTemp {
                            dst: (f.cur.data.as_ref().unwrap().data[1 + pl] as *mut BD::Pixel)
                                .offset(uvdstoff + h_off + v_off),
                            dst_stride: f.cur.stride[1],
                        },
                        bw4,
                        bh4,
                        t.b.x,
                        t.b.y,
                        1 + pl,
                        inter.nd.one_d.mv[0],
                        refp,
                        inter.r#ref[0] as usize,
                        filter_2d,
                    )?;
                }
            } else {
                if cmp::min(cbw4, cbh4) > 1
                    && (inter.inter_mode == GLOBALMV
                        && f.gmv_warp_allowed[inter.r#ref[0] as usize] != 0
                        || inter.motion_mode == MotionMode::Warp
                            && t.warpmv.r#type > Rav1dWarpedMotionType::Translation)
                {
                    for pl in 0..2 {
                        warp_affine::<BD>(
                            f,
                            &mut t.scratch.inter_mut().emu_edge,
                            t.b,
                            MaybeTempPixels::NonTemp {
                                dst: (f.cur.data.as_ref().unwrap().data[1 + pl] as *mut BD::Pixel)
                                    .offset(uvdstoff),
                                dst_stride: f.cur.stride[1],
                            },
                            b_dim,
                            1 + pl,
                            refp,
                            if inter.motion_mode == MotionMode::Warp {
                                &t.warpmv
                            } else {
                                &frame_hdr.gmv[inter.r#ref[0] as usize]
                            },
                        )?;
                    }
                } else {
                    for pl in 0..2 {
                        mc::<BD>(
                            f,
                            &mut t.scratch.inter_mut().emu_edge,
                            t.b,
                            MaybeTempPixels::NonTemp {
                                dst: (f.cur.data.as_ref().unwrap().data[1 + pl] as *mut BD::Pixel)
                                    .offset(uvdstoff),
                                dst_stride: f.cur.stride[1],
                            },
                            bw4 << (bw4 == ss_hor) as c_int,
                            bh4 << (bh4 == ss_ver) as c_int,
                            t.b.x & !ss_hor,
                            t.b.y & !ss_ver,
                            1 + pl,
                            inter.nd.one_d.mv[0],
                            refp,
                            inter.r#ref[0] as usize,
                            filter_2d,
                        )?;
                        if inter.motion_mode == MotionMode::Obmc {
                            obmc::<BD>(
                                f,
                                t,
                                (f.cur.data.as_ref().unwrap().data[1 + pl] as *mut BD::Pixel)
                                    .offset(uvdstoff),
                                f.cur.stride[1],
                                b_dim,
                                1 + pl,
                                bx4,
                                by4,
                                w4,
                                h4,
                            )?;
                        }
                    }
                }
                if let Some(interintra_type) = inter.interintra_type {
                    // FIXME for 8x32 with 4:2:2 subsampling, this probably does
                    // the wrong thing since it will select 4x16, not 4x32, as a
                    // transform size...
                    let ii_mask = match interintra_type {
                        InterIntraType::Blend => {
                            dav1d_ii_masks[bs as usize][chr_layout_idx]
                                [inter.nd.one_d.interintra_mode.get() as usize]
                        }
                        InterIntraType::Wedge => {
                            dav1d_wedge_masks[bs as usize][chr_layout_idx][0]
                                [inter.nd.one_d.wedge_idx as usize]
                        }
                    };

                    for pl in 0..2 {
                        let interintra_edge_pal =
                            &mut t.scratch.inter_intra_mut().interintra_edge_pal;
                        let tl_edge_array = interintra_edge_pal.edge.buf_mut::<BD>();
                        let tl_edge_offset = 32;
                        let mut m = match inter.nd.one_d.interintra_mode.get() {
                            InterIntraPredMode::Smooth => SMOOTH_PRED,
                            mode => mode as IntraPredMode,
                        };
                        let mut angle = 0;
                        let uvdst = (f.cur.data.as_ref().unwrap().data[1 + pl] as *mut BD::Pixel)
                            .offset(uvdstoff);
                        let top_sb_edge_slice = if t.b.y & f.sb_step - 1 == 0 {
                            let sby = t.b.y >> f.sb_shift;
                            let offset = (f.ipred_edge_off * (pl + 1)) as isize
                                + (f.sb128w * 128 * (sby - 1)) as isize;
                            Some((&f.ipred_edge, offset))
                        } else {
                            None
                        };
                        let data_stride = BD::pxstride(f.cur.stride[1]);
                        let data_width = 4 * ts.tiling.col_end >> ss_hor;
                        let data_height = 4 * ts.tiling.row_end >> ss_ver;
                        let data_diff = (data_height - 1) as isize * data_stride;
                        let dstuv_slice = slice::from_raw_parts(
                            (f.cur.data.as_ref().unwrap().data[1 + pl as usize]
                                as *const BD::Pixel)
                                .offset(cmp::min(data_diff, 0)),
                            data_diff.unsigned_abs() + data_width as usize,
                        );
                        m = rav1d_prepare_intra_edges(
                            t.b.x >> ss_hor,
                            t.b.x >> ss_hor > ts.tiling.col_start >> ss_hor,
                            t.b.y >> ss_ver,
                            t.b.y >> ss_ver > ts.tiling.row_start >> ss_ver,
                            ts.tiling.col_end >> ss_hor,
                            ts.tiling.row_end >> ss_ver,
                            EdgeFlags::empty(),
                            dstuv_slice,
                            f.cur.stride[1],
                            top_sb_edge_slice,
                            m,
                            &mut angle,
                            cbw4,
                            cbh4,
                            0,
                            tl_edge_array,
                            tl_edge_offset,
                            bd,
                        );
                        let tl_edge = &tl_edge_array[tl_edge_offset..];
                        let tmp = interintra_edge_pal.interintra.buf_mut::<BD>();
                        f.dsp.ipred.intra_pred[m as usize].call(
                            tmp.as_mut_ptr(),
                            cbw4 as isize * 4 * ::core::mem::size_of::<BD::Pixel>() as isize,
                            tl_edge.as_ptr(),
                            cbw4 * 4,
                            cbh4 * 4,
                            0,
                            0,
                            0,
                            bd,
                        );
                        (f.dsp.mc.blend)(
                            uvdst.cast(),
                            f.cur.stride[1],
                            tmp.as_mut_ptr().cast(),
                            cbw4 * 4,
                            cbh4 * 4,
                            ii_mask.as_ptr(),
                        );
                    }
                }
            }
        }
        t.tl_4x4_filter = filter_2d;
    }

    if debug_block_info!(f, t.b) && DEBUG_B_PIXELS {
        hex_dump::<BD>(
            dst,
            f.cur.stride[0] as usize,
            b_dim[0] as usize * 4,
            b_dim[1] as usize * 4,
            "y-pred",
        );
        if has_chroma {
            hex_dump::<BD>(
                (f.cur.data.as_ref().unwrap().data[1] as *mut BD::Pixel).offset(uvdstoff),
                f.cur.stride[1] as usize,
                cbw4 as usize * 4,
                cbh4 as usize * 4,
                "u-pred",
            );
            hex_dump::<BD>(
                (f.cur.data.as_ref().unwrap().data[2] as *mut BD::Pixel).offset(uvdstoff),
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
        // reset coef contexts
        CaseSet::<32, false>::many(
            [&t.l, &f.a[t.a]],
            [bh4 as usize, bw4 as usize],
            [by4 as usize, bx4 as usize],
            |case, dir| {
                case.set_disjoint(&dir.lcoef, 0x40);
            },
        );
        if has_chroma {
            CaseSet::<32, false>::many(
                [&t.l, &f.a[t.a]],
                [cbh4 as usize, cbw4 as usize],
                [cby4 as usize, cbx4 as usize],
                |case, dir| {
                    for ccoef in &dir.ccoef {
                        case.set_disjoint(ccoef, 0x40);
                    }
                },
            );
        }
        return Ok(());
    }

    let uvtx = &dav1d_txfm_dimensions[b.uvtx as usize];
    let ytx = &dav1d_txfm_dimensions[inter.max_ytx as usize];
    let tx_split = [inter.tx_split0 as u16, inter.tx_split1];

    for init_y in (0..bh4).step_by(16) {
        for init_x in (0..bw4).step_by(16) {
            // coefficient coding & inverse transforms
            let mut y_off = (init_y != 0) as c_int;
            let mut y;
            dst = dst.offset(BD::pxstride(f.cur.stride[0]) * 4 * init_y as isize);
            y = init_y;
            t.b.y += init_y;
            while y < cmp::min(h4, init_y + 16) {
                let mut x;
                let mut x_off = (init_x != 0) as c_int;
                x = init_x;
                t.b.x += init_x;
                while x < cmp::min(w4, init_x + 16) {
                    read_coef_tree::<BD>(
                        f,
                        t,
                        ts_c.as_deref_mut(),
                        bs,
                        b,
                        inter.max_ytx,
                        0,
                        tx_split,
                        x_off,
                        y_off,
                        Some(dst.add(x as usize * 4)),
                    );
                    t.b.x += ytx.w as c_int;
                    x += ytx.w as c_int;
                    x_off += 1;
                }
                dst = dst.offset(BD::pxstride(f.cur.stride[0]) * 4 * ytx.h as isize);
                t.b.x -= x;
                t.b.y += ytx.h as c_int;
                y += ytx.h as c_int;
                y_off += 1;
            }
            dst = dst.offset(-BD::pxstride(f.cur.stride[0]) * 4 * y as isize);
            t.b.y -= y;

            // chroma coefs and inverse transform
            if has_chroma {
                for pl in 0..2 {
                    let mut uvdst = (f.cur.data.as_ref().unwrap().data[1 + pl] as *mut BD::Pixel)
                        .offset(
                            uvdstoff
                                + (BD::pxstride(f.cur.stride[1]) * init_y as isize * 4 >> ss_ver),
                        );
                    y = init_y >> ss_ver;
                    t.b.y += init_y;
                    while y < cmp::min(ch4, init_y + 16 >> ss_ver) {
                        let mut x;
                        x = init_x >> ss_hor;
                        t.b.x += init_x;
                        while x < cmp::min(cw4, init_x + 16 >> ss_hor) {
                            let mut cf_guard;
                            let cf;
                            let eob;
                            let mut txtp;
                            if t.frame_thread.pass != 0 {
                                let p = t.frame_thread.pass & 1;
                                let len = uvtx.h as usize * 4 * uvtx.w as usize * 4;
                                let cf_idx = ts.frame_thread[p as usize].cf.load(Ordering::Relaxed);
                                cf_guard = f.frame_thread.cf.mut_slice_as(cf_idx..cf_idx + len);
                                cf = &mut *cf_guard;
                                ts.frame_thread[p as usize]
                                    .cf
                                    .store(cf_idx + len, Ordering::Relaxed);
                                let cbi = f.frame_thread.cbi
                                    [(t.b.y as isize * f.b4_stride + t.b.x as isize) as usize]
                                    [(1 + pl) as usize]
                                    .load(Ordering::Relaxed);
                                eob = cbi.eob().into();
                                txtp = cbi.txtp();
                            } else {
                                let mut cf_ctx = 0;
                                txtp = t.scratch.inter_intra().ac_txtp_map.txtp_map()
                                    [((by4 + (y << ss_ver)) * 32 + bx4 + (x << ss_hor)) as usize];
                                let a_ccoef = &f.a[t.a].ccoef[pl];
                                let a_start = (cbx4 + x) as usize;
                                let l_ccoef = &t.l.ccoef[pl];
                                let l_start = (cby4 + y) as usize;
                                eob = decode_coefs::<BD>(
                                    f,
                                    t.ts,
                                    ts_c.as_deref_mut().unwrap(),
                                    debug_block_info!(f, t.b),
                                    &mut t.scratch,
                                    &mut t.cf,
                                    &mut a_ccoef.index_mut((a_start.., ..uvtx.w as usize)),
                                    &mut l_ccoef.index_mut((l_start.., ..uvtx.h as usize)),
                                    b.uvtx,
                                    bs,
                                    b,
                                    1 + pl,
                                    CfSelect::Task,
                                    &mut txtp,
                                    &mut cf_ctx,
                                );
                                cf = t.cf.select_mut::<BD>();
                                if debug_block_info!(f, t.b) {
                                    println!(
                                        "Post-uv-cf-blk[pl={},tx={},txtp={},eob={}]: r={}",
                                        pl,
                                        b.uvtx,
                                        txtp,
                                        eob,
                                        ts_c.as_deref().unwrap().msac.rng,
                                    );
                                }
                                CaseSet::<16, true>::many(
                                    [l_ccoef, a_ccoef],
                                    [
                                        cmp::min(uvtx.h as i32, f.bh - t.b.y + ss_ver >> ss_ver)
                                            as usize,
                                        cmp::min(uvtx.w as i32, f.bw - t.b.x + ss_hor >> ss_hor)
                                            as usize,
                                    ],
                                    [(cby4 + y) as usize, (cbx4 + x) as usize],
                                    |case, dir| {
                                        case.set_disjoint(dir, cf_ctx);
                                    },
                                );
                            }
                            if eob >= 0 {
                                if debug_block_info!(f, t.b) && DEBUG_B_PIXELS {
                                    coef_dump(
                                        cf,
                                        uvtx.h as usize * 4,
                                        uvtx.w as usize * 4,
                                        3,
                                        "dq",
                                    );
                                }
                                (f.dsp.itx.itxfm_add[b.uvtx as usize][txtp as usize])
                                    .expect("non-null function pointer")(
                                    uvdst.add(4 * x as usize).cast(),
                                    f.cur.stride[1],
                                    cf.as_mut_ptr().cast(),
                                    eob,
                                    f.bitdepth_max,
                                );
                                if debug_block_info!(f, t.b) && DEBUG_B_PIXELS {
                                    hex_dump::<BD>(
                                        uvdst.add(4 * x as usize),
                                        f.cur.stride[1] as usize,
                                        uvtx.w as usize * 4,
                                        uvtx.h as usize * 4,
                                        "recon",
                                    );
                                }
                            }
                            t.b.x += (uvtx.w as c_int) << ss_hor;
                            x += uvtx.w as c_int;
                        }
                        uvdst = uvdst.offset(BD::pxstride(f.cur.stride[1]) * 4 * uvtx.h as isize);
                        t.b.x -= x << ss_hor;
                        t.b.y += (uvtx.h as c_int) << ss_ver;
                        y += uvtx.h as c_int;
                    }
                    t.b.y -= y << ss_ver;
                }
            }
        }
    }
    Ok(())
}

pub(crate) unsafe fn rav1d_filter_sbrow_deblock_cols<BD: BitDepth>(
    c: &Rav1dContext,
    f: &Rav1dFrameData,
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
    let ss_ver = (f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
    let ss_hor = (f.cur.p.layout != Rav1dPixelLayout::I444) as c_int;

    let (mut p, p_offset) = {
        let y_stride = BD::pxstride((*f).cur.stride[0]);
        let y_width = (*f).cur.p.w + 127 & !127;
        let y_height = (*f).cur.p.h + 127 & !127;
        let y_span = (y_height - 1) as isize * y_stride;
        let uv_stride = BD::pxstride((*f).cur.stride[1]);
        let uv_width = y_width >> ss_hor;
        let uv_height = y_height >> ss_ver;
        let uv_span = (uv_height - 1) as isize * uv_stride;

        let p = [
            slice::from_raw_parts_mut(
                f.cur.data.as_ref().unwrap().data[f.lf.p[0]]
                    .cast::<BD::Pixel>()
                    .offset(cmp::min(y_span, 0)),
                y_span.unsigned_abs() + y_width as usize + RAV1D_PICTURE_ALIGNMENT,
            ),
            slice::from_raw_parts_mut(
                f.cur.data.as_ref().unwrap().data[f.lf.p[1]]
                    .cast::<BD::Pixel>()
                    .offset(cmp::min(uv_span, 0)),
                uv_span.unsigned_abs() + uv_width as usize + RAV1D_PICTURE_ALIGNMENT,
            ),
            slice::from_raw_parts_mut(
                f.cur.data.as_ref().unwrap().data[f.lf.p[2]]
                    .cast::<BD::Pixel>()
                    .offset(cmp::min(uv_span, 0)),
                uv_span.unsigned_abs() + uv_width as usize + RAV1D_PICTURE_ALIGNMENT,
            ),
        ];
        let p_offset = [
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
    f: &Rav1dFrameData,
    _t: &mut Rav1dTaskContext,
    sby: c_int,
) {
    let y = sby * f.sb_step * 4;
    let ss_ver = (f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
    let ss_hor = (f.cur.p.layout != Rav1dPixelLayout::I444) as c_int;

    let (mut p, p_offset) = {
        let y_stride = BD::pxstride((*f).cur.stride[0]);
        let y_width = (*f).cur.p.w + 127 & !127;
        let y_height = (*f).cur.p.h + 127 & !127;
        let y_span = (y_height - 1) as isize * y_stride;
        let uv_stride = BD::pxstride((*f).cur.stride[1]);
        let uv_width = y_width >> ss_hor;
        let uv_height = y_height >> ss_ver;
        let uv_span = (uv_height - 1) as isize * uv_stride;

        let p = [
            slice::from_raw_parts_mut(
                f.cur.data.as_ref().unwrap().data[f.lf.p[0]]
                    .cast::<BD::Pixel>()
                    .offset(cmp::min(y_span, 0)),
                y_span.unsigned_abs() + y_width as usize + RAV1D_PICTURE_ALIGNMENT,
            ),
            slice::from_raw_parts_mut(
                f.cur.data.as_ref().unwrap().data[f.lf.p[1]]
                    .cast::<BD::Pixel>()
                    .offset(cmp::min(uv_span, 0)),
                uv_span.unsigned_abs() + uv_width as usize + RAV1D_PICTURE_ALIGNMENT,
            ),
            slice::from_raw_parts_mut(
                f.cur.data.as_ref().unwrap().data[f.lf.p[2]]
                    .cast::<BD::Pixel>()
                    .offset(cmp::min(uv_span, 0)),
                uv_span.unsigned_abs() + uv_width as usize + RAV1D_PICTURE_ALIGNMENT,
            ),
        ];
        let p_offset = [
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
        // Store loop filtered pixels required by CDEF / LR.
        rav1d_copy_lpf::<BD>(c, f, &p, &p_offset, sby);
    }
}

pub(crate) unsafe fn rav1d_filter_sbrow_cdef<BD: BitDepth>(
    c: &Rav1dContext,
    f: &Rav1dFrameData,
    tc: &mut Rav1dTaskContext,
    sby: c_int,
) {
    if !c.inloop_filters.contains(Rav1dInloopFilterType::CDEF) {
        return;
    }
    let sbsz = f.sb_step;
    let y = sby * sbsz * 4;
    let ss_ver = (f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
    let p = [
        f.cur.data.as_ref().unwrap().data[f.lf.p[0]]
            .cast::<BD::Pixel>()
            .offset((y as isize * BD::pxstride(f.cur.stride[0])) as isize),
        f.cur.data.as_ref().unwrap().data[f.lf.p[1]]
            .cast::<BD::Pixel>()
            .offset((y as isize * BD::pxstride(f.cur.stride[1]) >> ss_ver) as isize),
        f.cur.data.as_ref().unwrap().data[f.lf.p[2]]
            .cast::<BD::Pixel>()
            .offset((y as isize * BD::pxstride(f.cur.stride[1]) >> ss_ver) as isize),
    ];
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    let prev_mask = (sby - 1 >> (seq_hdr.sb128 == 0) as c_int) * f.sb128w;
    let mask_offset = (sby >> (seq_hdr.sb128 == 0) as c_int) * f.sb128w;
    let start = sby * sbsz;
    if sby != 0 {
        let ss_ver = (f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
        let p_up = [
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
    f: &Rav1dFrameData,
    _t: &mut Rav1dTaskContext,
    sby: c_int,
) {
    let sbsz = f.sb_step;
    let y = sby * sbsz * 4;
    let ss_ver = (f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
    let p = [
        f.cur.data.as_ref().unwrap().data[f.lf.p[0]]
            .cast::<BD::Pixel>()
            .offset(y as isize * BD::pxstride(f.cur.stride[0])),
        f.cur.data.as_ref().unwrap().data[f.lf.p[1]]
            .cast::<BD::Pixel>()
            .offset(y as isize * BD::pxstride(f.cur.stride[1]) >> ss_ver),
        f.cur.data.as_ref().unwrap().data[f.lf.p[2]]
            .cast::<BD::Pixel>()
            .offset(y as isize * BD::pxstride(f.cur.stride[1]) >> ss_ver),
    ];
    let sr_p = [
        f.sr_cur.p.data.as_ref().unwrap().data[f.lf.sr_p[0]]
            .cast::<BD::Pixel>()
            .offset((y as isize * BD::pxstride(f.sr_cur.p.stride[0])) as isize),
        f.sr_cur.p.data.as_ref().unwrap().data[f.lf.sr_p[1]]
            .cast::<BD::Pixel>()
            .offset((y as isize * BD::pxstride(f.sr_cur.p.stride[1]) >> ss_ver) as isize),
        f.sr_cur.p.data.as_ref().unwrap().data[f.lf.sr_p[2]]
            .cast::<BD::Pixel>()
            .offset((y as isize * BD::pxstride(f.sr_cur.p.stride[1]) >> ss_ver) as isize),
    ];
    let has_chroma = (f.cur.p.layout != Rav1dPixelLayout::I400) as usize;
    for pl in 0..1 + 2 * has_chroma {
        let ss_ver = (pl != 0 && f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
        let h_start = 8 * (sby != 0) as c_int >> ss_ver;
        let dst_stride = f.sr_cur.p.stride[(pl != 0) as usize];
        let dst = sr_p[pl].offset(-((h_start as isize * BD::pxstride(dst_stride)) as isize));
        let src_stride = f.cur.stride[(pl != 0) as usize];
        let src = p[pl].offset(-(h_start as isize * BD::pxstride(src_stride)));
        let h_end = 4 * (sbsz - 2 * ((sby + 1) < f.sbh) as c_int) >> ss_ver;
        let ss_hor = (pl != 0 && f.cur.p.layout != Rav1dPixelLayout::I444) as c_int;
        let dst_w = f.sr_cur.p.p.w + ss_hor >> ss_hor;
        let src_w = 4 * f.bw + ss_hor >> ss_hor;
        let img_h = f.cur.p.h - sbsz * 4 * sby + ss_ver >> ss_ver;

        (f.dsp.mc.resize)(
            dst.cast(),
            dst_stride,
            src.cast(),
            src_stride,
            dst_w,
            cmp::min(img_h, h_end) + h_start,
            src_w,
            f.resize_step[(pl != 0) as usize],
            f.resize_start[(pl != 0) as usize],
            f.bitdepth_max,
        );
    }
}

pub(crate) unsafe fn rav1d_filter_sbrow_lr<BD: BitDepth>(
    c: &Rav1dContext,
    f: &Rav1dFrameData,
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
    let ss_ver = (f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
    let h = f.sr_cur.p.p.h + 127 & !127;
    let mut sr_p = [
        slice::from_raw_parts_mut(
            f.sr_cur.p.data.as_ref().unwrap().data[f.lf.sr_p[0]].cast::<BD::Pixel>(),
            (h as isize * BD::pxstride(f.sr_cur.p.stride[0])) as usize,
        ),
        slice::from_raw_parts_mut(
            f.sr_cur.p.data.as_ref().unwrap().data[f.lf.sr_p[1]].cast::<BD::Pixel>(),
            (h as isize * BD::pxstride(f.sr_cur.p.stride[1])) as usize >> ss_ver,
        ),
        slice::from_raw_parts_mut(
            f.sr_cur.p.data.as_ref().unwrap().data[f.lf.sr_p[2]].cast::<BD::Pixel>(),
            (h as isize * BD::pxstride(f.sr_cur.p.stride[1])) as usize >> ss_ver,
        ),
    ];
    let sr_p_offset = [
        (y as isize * BD::pxstride(f.sr_cur.p.stride[0])) as usize,
        (y as isize * BD::pxstride(f.sr_cur.p.stride[1]) >> ss_ver) as usize,
    ];
    rav1d_lr_sbrow::<BD>(c, f, &mut sr_p, &sr_p_offset, sby);
}

pub(crate) unsafe fn rav1d_filter_sbrow<BD: BitDepth>(
    c: &Rav1dContext,
    f: &Rav1dFrameData,
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
    let ts = &f.ts[t.ts];
    let sby = t.b.y >> f.sb_shift;
    let sby_off = f.sb128w * 128 * sby;
    let x_off = ts.tiling.col_start;

    let y = (f.cur.data.as_ref().unwrap().data[0] as *const BD::Pixel).offset(
        (x_off * 4) as isize
            + ((t.b.y + f.sb_step) * 4 - 1) as isize * BD::pxstride(f.cur.stride[0]),
    );
    let ipred_edge_off = (f.ipred_edge_off * 0) + (sby_off + x_off * 4) as usize;
    let n = 4 * (ts.tiling.col_end - x_off) as usize;
    BD::pixel_copy(
        &mut f
            .ipred_edge
            .mut_slice_as(ipred_edge_off..ipred_edge_off + n),
        slice::from_raw_parts(y, n),
        n,
    );

    if f.cur.p.layout != Rav1dPixelLayout::I400 {
        let ss_ver = (f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
        let ss_hor = (f.cur.p.layout != Rav1dPixelLayout::I444) as c_int;

        let uv_off = (x_off * 4 >> ss_hor) as isize
            + (((t.b.y + f.sb_step) * 4 >> ss_ver) - 1) as isize * BD::pxstride(f.cur.stride[1]);
        for pl in 1..3 {
            let ipred_edge_off =
                (f.ipred_edge_off * pl) + (sby_off + (x_off * 4 >> ss_hor)) as usize;
            let n = 4 * (ts.tiling.col_end - x_off) as usize >> ss_hor;
            BD::pixel_copy(
                &mut f
                    .ipred_edge
                    .mut_slice_as(ipred_edge_off..ipred_edge_off + n),
                slice::from_raw_parts(
                    f.cur.data.as_ref().unwrap().data[pl]
                        .cast::<BD::Pixel>()
                        .offset(uv_off),
                    n,
                ),
                n,
            );
        }
    }
}

pub(crate) unsafe fn rav1d_copy_pal_block_y<BD: BitDepth>(
    t: &mut Rav1dTaskContext,
    f: &Rav1dFrameData,
    bx4: usize,
    by4: usize,
    bw4: usize,
    bh4: usize,
) {
    let pal_guard;
    let pal = if t.frame_thread.pass != 0 {
        let x = t.b.x as usize;
        let y = t.b.y as usize;
        let index = ((y >> 1) + (x & 1)) * (f.b4_stride as usize >> 1) + (x >> 1) + (y & 1);
        pal_guard = f.frame_thread.pal.index::<BD>(index);
        &pal_guard[0]
    } else {
        &t.scratch.inter_intra().interintra_edge_pal.pal.buf::<BD>()[0]
    };
    let al_pal = t.al_pal.select_mut::<BD>();
    for al_pal in &mut al_pal[0][bx4..][..bw4] {
        al_pal[0] = *pal;
    }
    for al_pal in &mut al_pal[1][by4..][..bh4] {
        al_pal[0] = *pal;
    }
}

pub(crate) unsafe fn rav1d_copy_pal_block_uv<BD: BitDepth>(
    t: &mut Rav1dTaskContext,
    f: &Rav1dFrameData,
    bx4: usize,
    by4: usize,
    bw4: usize,
    bh4: usize,
) {
    let pal_guard;
    let pal = if t.frame_thread.pass != 0 {
        let x = t.b.x as usize;
        let y = t.b.y as usize;
        let index = ((y >> 1) + (x & 1)) * (f.b4_stride as usize >> 1) + (x >> 1) + (y & 1);
        pal_guard = f.frame_thread.pal.index::<BD>(index);
        &pal_guard
    } else {
        t.scratch.inter_intra().interintra_edge_pal.pal.buf::<BD>()
    };
    // see aomedia bug 2183 for why we use luma coordinates here
    let al_pal = t.al_pal.select_mut::<BD>();
    for pl in 1..3 {
        for x in 0..bw4 {
            al_pal[0][bx4 + x][pl] = pal[pl];
        }
        for y in 0..bh4 {
            al_pal[1][by4 + y][pl] = pal[pl];
        }
    }
}

/// Return `pal_sz`.
pub(crate) unsafe fn rav1d_read_pal_plane<BD: BitDepth>(
    t: &mut Rav1dTaskContext,
    f: &Rav1dFrameData,
    ts_c: &mut Rav1dTileStateContext,
    pl: bool,
    sz_ctx: u8,
    bx4: usize,
    by4: usize,
) -> u8 {
    let pli = pl as usize;
    let not_pl = !pl as u16;

    let pal_sz_u8 = rav1d_msac_decode_symbol_adapt8(
        &mut ts_c.msac,
        &mut ts_c.cdf.m.pal_sz[pli][sz_ctx as usize],
        6,
    ) as u8
        + 2;
    let pal_sz = pal_sz_u8 as usize;
    let mut cache = [0.as_::<BD::Pixel>(); 16];
    let mut used_cache = [0.as_::<BD::Pixel>(); 8];
    let mut l_cache = if pl {
        t.pal_sz_uv[1][by4]
    } else {
        *t.l.pal_sz.index(by4)
    };
    let mut n_cache = 0;
    // don't reuse above palette outside SB64 boundaries
    let mut a_cache = if by4 & 15 != 0 {
        if pl {
            t.pal_sz_uv[0][bx4]
        } else {
            *f.a[t.a].pal_sz.index(bx4)
        }
    } else {
        0
    };
    let [a, l] = t.al_pal.select_mut::<BD>();
    let mut l = &l[by4][pli][..];
    let mut a = &a[bx4][pli][..];

    // fill/sort cache
    // TODO: This logic could be replaced with `itertools`' `.merge` and `.dedup`, which would elide bounds checks.
    while l_cache != 0 && a_cache != 0 {
        if l[0] < a[0] {
            if n_cache == 0 || cache[n_cache - 1] != l[0] {
                cache[n_cache] = l[0];
                n_cache += 1;
            }
            l = &l[1..];
            l_cache -= 1;
        } else {
            if a[0] == l[0] {
                l = &l[1..];
                l_cache -= 1;
            }
            if n_cache == 0 || cache[n_cache - 1] != a[0] {
                cache[n_cache] = a[0];
                n_cache += 1;
            }
            a = &a[1..];
            a_cache -= 1;
        }
    }
    if l_cache != 0 {
        loop {
            if n_cache == 0 || cache[n_cache - 1] != l[0] {
                cache[n_cache] = l[0];
                n_cache += 1;
            }
            l = &l[1..];
            l_cache -= 1;
            if !(l_cache > 0) {
                break;
            }
        }
    } else if a_cache != 0 {
        loop {
            if n_cache == 0 || cache[n_cache - 1] != a[0] {
                cache[n_cache] = a[0];
                n_cache += 1;
            }
            a = &a[1..];
            a_cache -= 1;
            if !(a_cache > 0) {
                break;
            }
        }
    }
    let cache = &cache[..n_cache];

    // find reused cache entries
    // TODO: Bounds checks could be elided with more complex iterators.
    let mut i = 0;
    for cache in cache {
        if !(i < pal_sz) {
            break;
        }
        if rav1d_msac_decode_bool_equi(&mut ts_c.msac) {
            used_cache[i] = *cache;
            i += 1;
        }
    }
    let used_cache = &used_cache[..i];

    // parse new entries
    let mut pal_guard;
    let pal = if t.frame_thread.pass != 0 {
        let pal_start = (((t.b.y >> 1) + (t.b.x & 1)) as isize * (f.b4_stride >> 1)
            + ((t.b.x >> 1) + (t.b.y & 1)) as isize) as usize;
        pal_guard = f.frame_thread.pal.index_mut::<BD>(pal_start);
        &mut pal_guard[pli]
    } else {
        &mut t
            .scratch
            .inter_intra_mut()
            .interintra_edge_pal
            .pal
            .buf_mut::<BD>()[pli]
    };
    let pal = &mut pal[..pal_sz];
    if i < pal.len() {
        let mut prev = rav1d_msac_decode_bools(&mut ts_c.msac, f.cur.p.bpc as u32) as u16;
        pal[i] = prev.as_::<BD::Pixel>();
        i += 1;

        if i < pal.len() {
            let mut bits = f.cur.p.bpc as u32 + rav1d_msac_decode_bools(&mut ts_c.msac, 2) - 3;
            let max = (1 << f.cur.p.bpc) - 1;

            loop {
                let delta = rav1d_msac_decode_bools(&mut ts_c.msac, bits) as u16;
                prev = cmp::min(prev + delta + not_pl, max);
                pal[i] = prev.as_::<BD::Pixel>();
                i += 1;
                if prev + not_pl >= max {
                    pal[i..].fill(max.as_::<BD::Pixel>());
                    break;
                } else {
                    bits = cmp::min(bits, 1 + ulog2((max - prev - not_pl) as u32) as u32);
                    if !(i < pal.len()) {
                        break;
                    }
                }
            }
        }

        // merge cache+new entries
        let mut n = 0;
        let mut m = used_cache.len();
        for i in 0..pal.len() {
            if n < used_cache.len() && (m >= pal.len() || used_cache[n] <= pal[m]) {
                pal[i] = used_cache[n];
                n += 1;
            } else {
                pal[i] = pal[m];
                m += 1;
            }
        }
    } else {
        pal[..used_cache.len()].copy_from_slice(&used_cache);
    }

    if debug_block_info!(f, t.b) {
        print!(
            "Post-pal[pl={},sz={},cache_size={},used_cache={}]: r={}, cache=",
            pli,
            pal_sz,
            cache.len(),
            used_cache.len(),
            ts_c.msac.rng
        );
        for (n, cache) in cache.iter().enumerate() {
            print!(
                "{}{:02x}",
                if n != 0 { ' ' } else { '[' },
                (*cache).as_::<c_int>()
            );
        }
        print!("{}, pal=", if cache.len() != 0 { "]" } else { "[]" });
        for (n, pal) in pal.iter().enumerate() {
            print!(
                "{}{:02x}",
                if n != 0 { ' ' } else { '[' },
                (*pal).as_::<c_int>()
            );
        }
        println!("]");
    }

    pal_sz_u8
}

/// Return `pal_sz[1]`.
pub(crate) unsafe fn rav1d_read_pal_uv<BD: BitDepth>(
    t: &mut Rav1dTaskContext,
    f: &Rav1dFrameData,
    ts_c: &mut Rav1dTileStateContext,
    sz_ctx: u8,
    bx4: usize,
    by4: usize,
) -> u8 {
    let pal_sz = rav1d_read_pal_plane::<BD>(t, f, ts_c, true, sz_ctx, bx4, by4);

    // V pal coding
    let mut pal_guard;
    let pal = if t.frame_thread.pass != 0 {
        pal_guard = f.frame_thread.pal.index_mut::<BD>(
            (((t.b.y >> 1) + (t.b.x & 1)) as isize * (f.b4_stride >> 1)
                + ((t.b.x >> 1) + (t.b.y & 1)) as isize) as usize,
        );
        &mut pal_guard[2]
    } else {
        &mut t
            .scratch
            .inter_intra_mut()
            .interintra_edge_pal
            .pal
            .buf_mut::<BD>()[2]
    };
    let pal = &mut pal[..pal_sz as usize];
    if rav1d_msac_decode_bool_equi(&mut ts_c.msac) {
        let bits = f.cur.p.bpc as u32 + rav1d_msac_decode_bools(&mut ts_c.msac, 2) - 4;
        let mut prev = rav1d_msac_decode_bools(&mut ts_c.msac, f.cur.p.bpc as c_uint) as u16;
        pal[0] = prev.as_::<BD::Pixel>();
        let max = (1 << f.cur.p.bpc) - 1;
        for pal in &mut pal[1..] {
            let mut delta = rav1d_msac_decode_bools(&mut ts_c.msac, bits) as i16;
            if delta != 0 && rav1d_msac_decode_bool_equi(&mut ts_c.msac) {
                delta = -delta;
            }
            prev = (prev as i16 + delta) as u16 & max;
            *pal = prev.as_::<BD::Pixel>();
        }
    } else {
        pal.fill_with(|| {
            rav1d_msac_decode_bools(&mut ts_c.msac, f.cur.p.bpc as c_uint).as_::<BD::Pixel>()
        });
    }
    if debug_block_info!(f, t.b) {
        print!("Post-pal[pl=2]: r={} ", ts_c.msac.rng);
        for (n, pal) in pal.iter().enumerate() {
            print!(
                "{}{:02x}",
                if n != 0 { ' ' } else { '[' },
                (*pal).as_::<c_int>()
            );
        }
        println!("]");
    }

    pal_sz
}
