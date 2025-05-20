#![deny(unsafe_op_in_unsafe_fn)]

use crate::cdef_apply::rav1d_cdef_brow;
use crate::ctx::CaseSet;
use crate::env::get_uv_inter_txtp;
use crate::in_range::InRange;
use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::ToPrimitive;
use crate::include::common::bitdepth::BPC;
use crate::include::common::dump::ac_dump;
use crate::include::common::dump::coef_dump;
use crate::include::common::dump::hex_dump;
use crate::include::common::dump::hex_dump_pic;
use crate::include::common::intops::apply_sign64;
use crate::include::common::intops::clip;
use crate::include::common::intops::ulog2;
use crate::include::dav1d::dav1d::Rav1dInloopFilterType;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::include::dav1d::headers::Rav1dPixelLayoutSubSampled;
use crate::include::dav1d::headers::Rav1dWarpedMotionParams;
use crate::include::dav1d::headers::Rav1dWarpedMotionType;
use crate::include::dav1d::picture::Rav1dPictureDataComponent;
use crate::include::dav1d::picture::Rav1dPictureDataComponentOffset;
use crate::internal::Bxy;
use crate::internal::Cf;
use crate::internal::CodedBlockInfo;
use crate::internal::Rav1dContext;
use crate::internal::Rav1dFrameData;
use crate::internal::Rav1dTaskContext;
use crate::internal::Rav1dTileStateContext;
use crate::internal::ScratchEmuEdge;
use crate::internal::TaskContextScratch;
use crate::internal::TileStateRef;
use crate::intra_edge::EdgeFlags;
use crate::ipred_prepare::rav1d_prepare_intra_edges;
use crate::ipred_prepare::sm_flag;
use crate::ipred_prepare::sm_uv_flag;
use crate::levels::Av1Block;
use crate::levels::Av1BlockInter;
use crate::levels::Av1BlockIntra;
use crate::levels::Av1BlockIntraInter;
use crate::levels::BlockSize;
use crate::levels::CompInterType;
use crate::levels::Filter2d;
use crate::levels::InterIntraPredMode;
use crate::levels::InterIntraType;
use crate::levels::IntraPredMode;
use crate::levels::MotionMode;
use crate::levels::Mv;
use crate::levels::TxClass;
use crate::levels::TxfmSize;
use crate::levels::TxfmType;
use crate::levels::CFL_PRED;
use crate::levels::DCT_DCT;
use crate::levels::DC_PRED;
use crate::levels::FILTER_PRED;
use crate::levels::GLOBALMV;
use crate::levels::GLOBALMV_GLOBALMV;
use crate::levels::IDTX;
use crate::levels::SMOOTH_PRED;
use crate::levels::WHT_WHT;
use crate::lf_apply::rav1d_copy_lpf;
use crate::lf_apply::rav1d_loopfilter_sbrow_cols;
use crate::lf_apply::rav1d_loopfilter_sbrow_rows;
use crate::lr_apply::rav1d_lr_sbrow;
use crate::msac::rav1d_msac_decode_bool_adapt;
use crate::msac::rav1d_msac_decode_bool_equi;
use crate::msac::rav1d_msac_decode_bools;
use crate::msac::rav1d_msac_decode_hi_tok;
use crate::msac::rav1d_msac_decode_symbol_adapt16;
use crate::msac::rav1d_msac_decode_symbol_adapt4;
use crate::msac::rav1d_msac_decode_symbol_adapt8;
use crate::msac::MsacContext;
use crate::picture::Rav1dThreadPicture;
use crate::pixels::Pixels as _;
use crate::scan::dav1d_scans;
use crate::strided::Strided as _;
use crate::tables::dav1d_filter_2d;
use crate::tables::dav1d_filter_mode_to_y_mode;
use crate::tables::dav1d_lo_ctx_offsets;
use crate::tables::dav1d_skip_ctx;
use crate::tables::dav1d_tx_type_class;
use crate::tables::dav1d_tx_types_per_set;
use crate::tables::dav1d_txfm_dimensions;
use crate::tables::dav1d_txtp_from_uvmode;
use crate::tables::TxfmInfo;
use crate::wedge::dav1d_ii_masks;
use crate::wedge::dav1d_wedge_masks;
use crate::with_offset::WithOffset;
use assert_matches::debug_assert_matches;
use libc::intptr_t;
use std::array;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::hint::assert_unchecked;
use std::ops::BitOr;
use std::ptr;
use to_method::To as _;

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
/// [`Bxy`]: crate::internal::Bxy
macro_rules! debug_block_info {
    ($f:expr, $tb:expr) => {{
        use crate::internal::Bxy;

        let tb: Bxy = $tb;
        false && $f.frame_hdr.as_ref().unwrap().frame_offset == 2 && tb.debug_block_info()
    }};
}
pub(crate) use debug_block_info;

const DEBUG_B_PIXELS: bool = false;

pub(crate) type ReconBIntraFn = fn(
    &Rav1dFrameData,
    &mut Rav1dTaskContext,
    Option<&mut Rav1dTileStateContext>,
    BlockSize,
    EdgeFlags,
    &Av1Block,
    &Av1BlockIntra,
) -> ();

pub(crate) type ReconBInterFn = fn(
    &Rav1dFrameData,
    &mut Rav1dTaskContext,
    Option<&mut Rav1dTileStateContext>,
    BlockSize,
    &Av1Block,
    &Av1BlockInter,
) -> Result<(), ()>;

pub(crate) type FilterSbrowFn =
    fn(&Rav1dContext, &Rav1dFrameData, &mut Rav1dTaskContext, c_int) -> ();

pub(crate) type BackupIpredEdgeFn = fn(&Rav1dFrameData, &mut Rav1dTaskContext) -> ();

pub(crate) type ReadCoefBlocksFn = fn(
    &Rav1dFrameData,
    &mut Rav1dTaskContext,
    &mut Rav1dTileStateContext,
    BlockSize,
    &Av1Block,
) -> ();

pub(crate) type CopyPalBlockFn = fn(
    t: &mut Rav1dTaskContext,
    f: &Rav1dFrameData,
    bx4: usize,
    by4: usize,
    bw4: usize,
    bh4: usize,
) -> ();

pub(crate) type ReadPalPlaneFn = fn(
    t: &mut Rav1dTaskContext,
    f: &Rav1dFrameData,
    ts_c: &mut Rav1dTileStateContext,
    pl: bool,
    sz_ctx: u8,
    bx4: usize,
    by4: usize,
) -> u8; // `pal_sz`

pub(crate) type ReadPalUVFn = fn(
    t: &mut Rav1dTaskContext,
    f: &Rav1dFrameData,
    ts_c: &mut Rav1dTileStateContext,
    sz_ctx: u8,
    bx4: usize,
    by4: usize,
) -> u8; // `pal_sz[1]`

#[inline]
fn read_golomb(msac: &mut MsacContext) -> u32 {
    let mut len = 0;
    let mut val = 1;

    while !rav1d_msac_decode_bool_equi(msac) && len < 32 {
        len += 1;
    }
    for _ in 0..len {
        val = (val << 1) + rav1d_msac_decode_bool_equi(msac) as u32;
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
) -> InRange<u8, 0, { 13 - 1 }> {
    let b_dim = bs.dimensions();
    let skip_ctx = if chroma {
        let ss_ver = layout == Rav1dPixelLayout::I420;
        let ss_hor = layout != Rav1dPixelLayout::I444;
        let not_one_blk = b_dim[2] - (b_dim[2] != 0 && ss_hor) as u8 > t_dim.lw
            || b_dim[3] - (b_dim[3] != 0 && ss_ver) as u8 > t_dim.lh;
        fn merge_ctx<const N: usize>(dir: &[u8]) -> bool {
            dir[..N] != [0x40; N]
        }

        fn cdir(dir: &[u8]) -> u8 {
            let cdir = match dir.len() {
                1 => merge_ctx::<1>(dir),
                2 => merge_ctx::<2>(dir),
                4 => merge_ctx::<4>(dir),
                8 => merge_ctx::<8>(dir),
                _ => {
                    debug_assert!(false);
                    false
                }
            };
            cdir as u8
        }

        (7 + (not_one_blk as u8) * 3) + cdir(a) + cdir(l)
    } else if b_dim[2] == t_dim.lw && b_dim[3] == t_dim.lh {
        0
    } else {
        /// Read and xor all the bytes.
        fn merge_ctx(dir: &[u8]) -> u8 {
            let n = dir.len();
            if n == 1 {
                u8::read_ne(dir)
            } else {
                (if n == 2 {
                    u16::read_ne(dir)
                } else {
                    (if n == 4 {
                        u32::read_ne(dir)
                    } else {
                        (if n == 8 {
                            u64::read_ne(dir)
                        } else {
                            (if n == 16 {
                                u128::read_ne(dir)
                            } else {
                                debug_assert!(false);
                                0
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

        fn ldir(dir: &[u8]) -> usize {
            let ldir = merge_ctx(dir);
            cmp::min(ldir & 0x3f, 4) as usize
        }

        dav1d_skip_ctx[ldir(a)][ldir(l)]
    };
    InRange::new(skip_ctx).unwrap()
}

#[inline]
fn get_dc_sign_ctx(tx: TxfmSize, a: &[u8], l: &[u8]) -> c_uint {
    let mask = 0xc0c0c0c0c0c0c0c0 as u64;
    let mul = 0x101010101010101 as u64;

    use TxfmSize::*;
    let s = match tx {
        S4x4 => {
            let mut t = u8::read_ne(a) as i32 >> 6;
            t += u8::read_ne(l) as i32 >> 6;
            t - 1 - 1
        }
        S8x8 => {
            let mut t = u16::read_ne(a) as u32 & mask as u32;
            t += u16::read_ne(l) as u32 & mask as u32;
            t = t.wrapping_mul(0x4040404);
            (t >> 24) as i32 - 2 - 2
        }
        S16x16 => {
            let mut t = (u32::read_ne(a) & mask as u32) >> 6;
            t += (u32::read_ne(l) & mask as u32) >> 6;
            t = t.wrapping_mul(mul as u32);
            (t >> 24) as i32 - 4 - 4
        }
        S32x32 => {
            let mut t = (u64::read_ne(a) & mask) >> 6;
            t += (u64::read_ne(l) & mask) >> 6;
            t = t.wrapping_mul(mul);
            (t >> 56) as i32 - 8 - 8
        }
        S64x64 => {
            let mut t = (u64::read_ne(&a[0..]) & mask) >> 6;
            t += (u64::read_ne(&a[8..]) & mask) >> 6;
            t += (u64::read_ne(&l[0..]) & mask) >> 6;
            t += (u64::read_ne(&l[8..]) & mask) >> 6;
            t = t.wrapping_mul(mul);
            (t >> 56) as i32 - 16 - 16
        }
        R4x8 => {
            let mut t = u8::read_ne(a) as u32 & mask as u32;
            t += u16::read_ne(l) as u32 & mask as u32;
            t = t.wrapping_mul(0x4040404);
            (t >> 24) as i32 - 1 - 2
        }
        R8x4 => {
            let mut t = u16::read_ne(a) as u32 & mask as u32;
            t += u8::read_ne(l) as u32 & mask as u32;
            t = t.wrapping_mul(0x4040404);
            (t >> 24) as i32 - 2 - 1
        }
        R8x16 => {
            let mut t = u16::read_ne(a) as u32 & mask as u32;
            t += u32::read_ne(l) & mask as u32;
            t = (t >> 6).wrapping_mul(mul as u32);
            (t >> 24) as i32 - 2 - 4
        }
        R16x8 => {
            let mut t = u32::read_ne(a) & mask as u32;
            t += u16::read_ne(l) as c_uint & mask as u32;
            t = (t >> 6).wrapping_mul(mul as u32);
            (t >> 24) as i32 - 4 - 2
        }
        R16x32 => {
            let mut t = (u32::read_ne(a) & mask as u32) as u64;
            t += u64::read_ne(l) & mask;
            t = (t >> 6).wrapping_mul(mul);
            (t >> 56) as i32 - 4 - 8
        }
        R32x16 => {
            let mut t = u64::read_ne(a) & mask;
            t += (u32::read_ne(l) & mask as u32) as u64;
            t = (t >> 6).wrapping_mul(mul);
            (t >> 56) as i32 - 8 - 4
        }
        R32x64 => {
            let mut t = (u64::read_ne(&a[0..]) & mask) >> 6;
            t += (u64::read_ne(&l[0..]) & mask) >> 6;
            t += (u64::read_ne(&l[8..]) & mask) >> 6;
            t = t.wrapping_mul(mul);
            (t >> 56) as i32 - 8 - 16
        }
        R64x32 => {
            let mut t = (u64::read_ne(&a[0..]) & mask) >> 6;
            t += (u64::read_ne(&a[8..]) & mask) >> 6;
            t += (u64::read_ne(&l[0..]) & mask) >> 6;
            t = t.wrapping_mul(mul);
            (t >> 56) as i32 - 16 - 8
        }
        R4x16 => {
            let mut t = u8::read_ne(a) as u32 & mask as u32;
            t += u32::read_ne(l) & mask as u32;
            t = (t >> 6).wrapping_mul(mul as u32);
            (t >> 24) as i32 - 1 - 4
        }
        R16x4 => {
            let mut t = u32::read_ne(a) & mask as u32;
            t += u8::read_ne(l) as u32 & mask as u32;
            t = (t >> 6).wrapping_mul(mul as u32);
            (t >> 24) as i32 - 4 - 1
        }
        R8x32 => {
            let mut t = (u16::read_ne(a) as u32 & mask as u32) as u64;
            t += u64::read_ne(l) & mask;
            t = (t >> 6).wrapping_mul(mul);
            (t >> 56) as i32 - 2 - 8
        }
        R32x8 => {
            let mut t = u64::read_ne(a) & mask;
            t += (u16::read_ne(l) as u32 & mask as u32) as u64;
            t = (t >> 6).wrapping_mul(mul);
            (t >> 56) as i32 - 8 - 2
        }
        R16x64 => {
            let mut t = (u32::read_ne(a) & mask as u32) as u64;
            t += u64::read_ne(&l[0..]) & mask;
            t = (t >> 6) + ((u64::read_ne(&l[8..]) & mask) >> 6);
            t = t.wrapping_mul(mul);
            (t >> 56) as i32 - 4 - 16
        }
        R64x16 => {
            let mut t = u64::read_ne(&a[0..]) & mask;
            t += (u32::read_ne(l) & mask as u32) as u64;
            t = (t >> 6) + ((u64::read_ne(&a[8..]) & mask) >> 6);
            t = t.wrapping_mul(mul);
            (t >> 56) as i32 - 16 - 4
        }
    };

    (s != 0) as c_uint + (s > 0) as c_uint
}

#[inline]
fn get_lo_ctx(
    levels: &[u8],
    tx_class: TxClass,
    hi_mag: &mut u32,
    ctx_offsets: Option<&[[u8; 5]; 5]>,
    x: u8,
    y: u8,
    stride: u8,
) -> u8 {
    let stride = stride as usize;
    let level = |y, x| levels[y * stride + x] as u32;

    // Note that the first `mag` initialization is moved inside the `match`
    // so that the different bounds checks can be done inside the `match`,
    // as putting them outside the `match` in an identical one trips up LLVM.
    let mut mag;
    let offset;
    match ctx_offsets {
        Some(ctx_offsets) => {
            level(2, 1); // Bounds check all at once.
            mag = level(0, 1) + level(1, 0);
            debug_assert_matches!(tx_class, TxClass::TwoD);
            mag += level(1, 1);
            *hi_mag = mag;
            mag += level(0, 2) + level(2, 0);
            offset = ctx_offsets[cmp::min(y as usize, 4)][cmp::min(x as usize, 4)];
        }
        None => {
            debug_assert_matches!(tx_class, TxClass::H | TxClass::V);
            level(1, 4); // Bounds check all at once.
            mag = level(0, 1) + level(1, 0);
            mag += level(0, 2);
            *hi_mag = mag;
            mag += level(0, 3) + level(0, 4);
            offset = 26 + if y > 1 { 10 } else { y * 5 };
        }
    }
    offset
        + if mag > 512 {
            4
        } else {
            ((mag + 64) >> 7) as u8
        }
}

fn decode_coefs<BD: BitDepth>(
    f: &Rav1dFrameData,
    ts: usize,
    ts_c: &mut Rav1dTileStateContext,
    dbg_block_info: bool,
    scratch: &mut TaskContextScratch,
    t_cf: &mut Cf,
    a: &mut [u8],
    l: &mut [u8],
    tx: TxfmSize,
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
    let ts = &f.ts[ts];
    let chroma = plane != 0;
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    let lossless = frame_hdr.segmentation.lossless[b.seg_id.get()];
    let t_dim = &dav1d_txfm_dimensions[tx as usize];
    let dbg = dbg_block_info && plane != 0 && false;

    if dbg {
        println!("Start: r={}", ts_c.msac.rng);
    }

    // does this block have any non-zero coefficients
    let sctx = get_skip_ctx(t_dim, bs, a, l, chroma, f.cur.p.layout);
    let all_skip = rav1d_msac_decode_bool_adapt(
        &mut ts_c.msac,
        &mut ts_c.cdf.coef.skip[t_dim.ctx as usize][sctx.get() as usize],
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
            assert!(t_dim.max == TxfmSize::S4x4 as _);
            WHT_WHT
        }
        Intra(_) if t_dim.max >= TxfmSize::S32x32 as _ => DCT_DCT,
        Inter(_) if t_dim.max >= TxfmSize::S64x64 as _ => DCT_DCT,
        Intra(intra) if chroma => dav1d_txtp_from_uvmode[intra.uv_mode as usize],
        // inferred from either the luma txtp (inter) or a LUT (intra)
        Inter(_) if chroma => get_uv_inter_txtp(t_dim, *txtp),
        // In libaom, lossless is checked by a literal qidx == 0, but not all
        // such blocks are actually lossless. The remainder gets an implicit
        // transform type (for luma)
        _ if frame_hdr.segmentation.qidx[b.seg_id.get()] == 0 => DCT_DCT,
        Intra(intra) => {
            let y_mode_nofilt = if intra.y_mode == FILTER_PRED {
                dav1d_filter_mode_to_y_mode[intra.y_angle as usize]
            } else {
                intra.y_mode
            };
            let idx;
            let txtp = if frame_hdr.reduced_txtp_set != 0 || t_dim.min == TxfmSize::S16x16 as _ {
                idx = rav1d_msac_decode_symbol_adapt8(
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
                    "Post-txtp-intra[{:?}->{}][{}][{}->{}]: r={}",
                    tx, t_dim.min, y_mode_nofilt, idx, txtp, ts_c.msac.rng,
                );
            }
            txtp
        }
        Inter(_) => {
            let idx;
            let txtp = if frame_hdr.reduced_txtp_set != 0 || t_dim.max == TxfmSize::S32x32 as _ {
                let bool_idx = rav1d_msac_decode_bool_adapt(
                    &mut ts_c.msac,
                    &mut ts_c.cdf.m.txtp_inter3[t_dim.min as usize],
                );
                idx = bool_idx as u8;
                if bool_idx {
                    DCT_DCT
                } else {
                    IDTX
                }
            } else if t_dim.min == TxfmSize::S16x16 as _ {
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
                    "Post-txtp-inter[{:?}->{}][{}->{}]: r={}",
                    tx, t_dim.min, idx, txtp, ts_c.msac.rng,
                );
            }
            txtp
        }
    };

    // find end-of-block (eob)
    let tx2dszctx =
        cmp::min(t_dim.lw, TxfmSize::S32x32 as u8) + cmp::min(t_dim.lh, TxfmSize::S32x32 as u8);
    let tx_class = dav1d_tx_type_class[*txtp as usize];
    let chroma = chroma as usize;
    let is_1d = (tx_class != TxClass::TwoD) as usize;
    let eob_bin = match tx2dszctx {
        0 => {
            let eob_bin_cdf = &mut ts_c.cdf.coef.eob_bin_16[chroma][is_1d];
            rav1d_msac_decode_symbol_adapt8(&mut ts_c.msac, eob_bin_cdf, 4 + 0)
        }
        1 => {
            let eob_bin_cdf = &mut ts_c.cdf.coef.eob_bin_32[chroma][is_1d];
            rav1d_msac_decode_symbol_adapt8(&mut ts_c.msac, eob_bin_cdf, 4 + 1)
        }
        2 => {
            let eob_bin_cdf = &mut ts_c.cdf.coef.eob_bin_64[chroma][is_1d];
            rav1d_msac_decode_symbol_adapt8(&mut ts_c.msac, eob_bin_cdf, 4 + 2)
        }
        3 => {
            let eob_bin_cdf = &mut ts_c.cdf.coef.eob_bin_128[chroma][is_1d];
            rav1d_msac_decode_symbol_adapt8(&mut ts_c.msac, eob_bin_cdf, 4 + 3)
        }
        4 => {
            let eob_bin_cdf = &mut ts_c.cdf.coef.eob_bin_256[chroma][is_1d];
            rav1d_msac_decode_symbol_adapt16(&mut ts_c.msac, eob_bin_cdf, 4 + 4)
        }
        5 => {
            let eob_bin_cdf = &mut ts_c.cdf.coef.eob_bin_512[chroma];
            rav1d_msac_decode_symbol_adapt16(&mut ts_c.msac, eob_bin_cdf, 4 + 5)
        }
        6 => {
            let eob_bin_cdf = &mut ts_c.cdf.coef.eob_bin_1024[chroma];
            rav1d_msac_decode_symbol_adapt16(&mut ts_c.msac, eob_bin_cdf, 4 + 6)
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
    let eob = if eob_bin > 1 {
        let eob_hi_bit_cdf =
            &mut ts_c.cdf.coef.eob_hi_bit[t_dim.ctx as usize][chroma][eob_bin as usize];
        let eob_hi_bit = rav1d_msac_decode_bool_adapt(&mut ts_c.msac, eob_hi_bit_cdf) as u16;
        if dbg {
            println!(
                "Post-eob_hi_bit[{}][{}][{}][{}]: r={}",
                t_dim.ctx, chroma, eob_bin, eob_hi_bit, ts_c.msac.rng,
            );
        }
        let eob = ((eob_hi_bit | 2) << (eob_bin - 2))
            | rav1d_msac_decode_bools(&mut ts_c.msac, eob_bin - 2) as u16;
        if dbg {
            println!("Post-eob[{}]: r={}", eob, ts_c.msac.rng);
        }
        eob
    } else {
        eob_bin as u16
    };

    struct Cf<'a, BD: BitDepth>(&'a mut [BD::Coef]);

    impl<'a, BD: BitDepth> Cf<'a, BD> {
        fn index(&self, rc: u16) -> usize {
            let i = rc as usize & (self.0.len() - 1);
            // SAFETY: `self.0.len()` is either `cf_len` or `CF_LEN`,
            // both of which are powers of 2.
            // `cf_len` is a power of 2 since it's from `1 << n`, etc.
            // Thus, `& (self.0.len() - 1)` is the same as `% self.0.len()`.
            unsafe { assert_unchecked(i < self.0.len()) };
            i
        }

        #[cfg_attr(debug_assertions, track_caller)]
        pub fn get(&self, rc: u16) -> i32 {
            self.0[self.index(rc)].into()
        }

        #[cfg_attr(debug_assertions, track_caller)]
        pub fn set<T: ToPrimitive<BD::Coef>>(&mut self, rc: u16, value: T) {
            self.0[self.index(rc)] = value.as_();
        }
    }

    let sw = cmp::min(1 << t_dim.lw, 8) as usize;
    let sh = cmp::min(1 << t_dim.lh, 8) as usize;
    let cf_len = sw * 4 * sh * 4;
    let cf = match cf {
        CfSelect::Frame(offset) => &mut *f
            .frame_thread
            .cf
            .mut_slice_as((offset as usize.., ..cf_len)),
        CfSelect::Task => t_cf.select_mut::<BD>(),
    };
    let mut cf = Cf::<BD>(cf);

    // base tokens
    let mut rc;
    let mut dc_tok;

    #[inline]
    fn decode_coefs_class<const TX_CLASS: usize, BD: BitDepth>(
        ts_c: &mut Rav1dTileStateContext,
        t_dim: &TxfmInfo,
        chroma: usize,
        scratch: &mut TaskContextScratch,
        eob: u16,
        tx: TxfmSize,
        dbg: bool,
        cf: &mut Cf<BD>,
    ) -> (u16, u32) {
        let tx_class = const { TxClass::from_repr(TX_CLASS) }.unwrap();

        let eob_cdf = &mut ts_c.cdf.coef.eob_base_tok[t_dim.ctx as usize][chroma];
        let hi_cdf = &mut ts_c.cdf.coef.br_tok[cmp::min(t_dim.ctx, 3) as usize][chroma];

        let lo_cdf = &mut ts_c.cdf.coef.base_tok[t_dim.ctx as usize][chroma];
        let levels = scratch.inter_intra_mut().levels_pal.levels_mut();

        let slw = cmp::min(t_dim.lw, TxfmSize::S32x32 as u8);
        let slh = cmp::min(t_dim.lh, TxfmSize::S32x32 as u8);
        let tx2dszctx = slw + slh;

        // eob
        let mut ctx =
            1 + (eob > (2 << tx2dszctx) as u16) as u8 + (eob > (4 << tx2dszctx) as u16) as u8;
        let eob_tok =
            rav1d_msac_decode_symbol_adapt4(&mut ts_c.msac, &mut eob_cdf[ctx as usize], 2);
        let mut tok = eob_tok + 1;
        let mut level_tok = tok * 0x41;
        let mut mag = 0;

        let lo_ctx_offsets;
        let scan;
        let stride;
        match tx_class {
            TxClass::TwoD => {
                let is_rect = tx.is_rect() as usize;
                lo_ctx_offsets = Some(&dav1d_lo_ctx_offsets[is_rect + (tx as usize & is_rect)]);
                scan = dav1d_scans[tx as usize];
                stride = 4 << slh;
            }
            TxClass::H | TxClass::V => {
                lo_ctx_offsets = None;
                scan = &[];
                stride = 16;
            }
        }

        let shift;
        let shift2;
        let mask;
        let slwh_zero;
        match tx_class {
            TxClass::TwoD => {
                shift = slh + 2;
                shift2 = 0;
                mask = (4 << slh) - 1;
                slwh_zero = slw;
            }
            TxClass::H => {
                shift = slh + 2;
                shift2 = 0;
                mask = (4 << slh) - 1;
                slwh_zero = slh;
            }
            TxClass::V => {
                shift = slw + 2;
                shift2 = slh + 2;
                mask = (4 << slw) - 1;
                slwh_zero = slw;
            }
        }

        // Optimizes better than `.fill(0)`,
        // which doesn't elide the bounds check, inline, or vectorize.
        for i in 0..stride as usize * ((4 << slwh_zero) as usize + 2) {
            levels[i] = 0;
        }

        let mut rc;
        let mut x;
        let mut y;
        match tx_class {
            TxClass::TwoD => {
                rc = scan[eob as usize].get();
                x = (rc >> shift) as u8;
                y = rc as u8 & mask;
            }
            TxClass::H => {
                // Transposing reduces the stride and padding requirements.
                x = eob as u8 & mask;
                y = (eob >> shift) as u8;
                rc = eob as u16;
            }
            TxClass::V => {
                x = eob as u8 & mask;
                y = (eob >> shift) as u8;
                rc = (x as u16) << shift2 | y as u16;
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
            tok = rav1d_msac_decode_hi_tok(&mut ts_c.msac, &mut hi_cdf[ctx as usize]);
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
        cf.set(rc, tok.to::<i16>() << 11);
        let level_off = if tx_class == TxClass::TwoD {
            rc as usize
        } else {
            x as usize * stride as usize + y as usize
        };
        levels[level_off] = level_tok as u8;

        for i in (1..eob).rev() {
            // ac
            let rc_i;
            match tx_class {
                TxClass::TwoD => {
                    rc_i = scan[i as usize].get();
                    x = (rc_i >> shift) as u8;
                    y = rc_i as u8 & mask;
                }
                TxClass::H => {
                    x = i as u8 & mask;
                    y = (i >> shift) as u8;
                    rc_i = i as u16;
                }
                TxClass::V => {
                    x = i as u8 & mask;
                    y = (i >> shift) as u8;
                    rc_i = (x as u16) << shift2 | y as u16;
                }
            }
            debug_assert!(x < 32 && y < 32);
            x %= 32;
            y %= 32;
            let level_off = if tx_class == TxClass::TwoD {
                rc_i as usize
            } else {
                x as usize * stride as usize + y as usize
            };
            let level = &mut levels[level_off..];
            ctx = get_lo_ctx(level, tx_class, &mut mag, lo_ctx_offsets, x, y, stride);
            if tx_class == TxClass::TwoD {
                y |= x;
            }
            tok = rav1d_msac_decode_symbol_adapt4(&mut ts_c.msac, &mut lo_cdf[ctx as usize], 3);
            if dbg {
                println!(
                    "Post-lo_tok[{}][{}][{}][{}={}={}]: r={}",
                    t_dim.ctx, chroma, ctx, i, rc_i, tok, ts_c.msac.rng,
                );
            }
            if tok == 3 {
                let mag = mag as u8 & 63;
                ctx = if y > (tx_class == TxClass::TwoD) as u8 {
                    14
                } else {
                    7
                } + if mag > 12 { 6 } else { (mag + 1) >> 1 };
                tok = rav1d_msac_decode_hi_tok(&mut ts_c.msac, &mut hi_cdf[ctx as usize]);
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
                cf.set(rc_i, ((tok as u16) << 11) | rc);
                rc = rc_i;
            } else {
                // `0x1` for `tok`, `0x7ff` as bitmask for `rc`, `0x41` for `level_tok`.
                let tok = tok as u32 * 0x17ff41;
                level[0] = tok as u8;

                let tok_check = if tok != 0 {
                    ((tok as u16) << 11) | rc
                } else {
                    0
                };

                // This is optimized differently from C to avoid branches,
                // as simple branches are not always optimized to branchless `cmov`s.
                let mask = tok >> 9;
                let tok = mask & (rc as u32 + !0x7ff);
                let mask = mask as u16;
                rc = (rc_i & mask) | (rc & !mask);

                debug_assert!(tok == tok_check as u32);
                cf.set(rc_i, tok);
            }
        }
        // dc
        ctx = if tx_class == TxClass::TwoD {
            0
        } else {
            get_lo_ctx(levels, tx_class, &mut mag, lo_ctx_offsets, 0, 0, stride)
        };
        let mut dc_tok =
            rav1d_msac_decode_symbol_adapt4(&mut ts_c.msac, &mut lo_cdf[ctx as usize], 3) as c_uint;
        if dbg {
            println!(
                "Post-dc_lo_tok[{}][{}][{}][{}]: r={}",
                t_dim.ctx, chroma, ctx, dc_tok, ts_c.msac.rng,
            );
        }
        if dc_tok == 3 {
            if tx_class == TxClass::TwoD {
                mag = levels[0 * stride as usize + 1] as c_uint
                    + levels[1 * stride as usize + 0] as c_uint
                    + levels[1 * stride as usize + 1] as c_uint;
            }
            let mag = mag as u8 & 63;
            ctx = if mag > 12 { 6 } else { (mag + 1) >> 1 };
            dc_tok = rav1d_msac_decode_hi_tok(&mut ts_c.msac, &mut hi_cdf[ctx as usize]) as c_uint;
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

        (rc, dc_tok)
    }

    if eob != 0 {
        let cf = &mut cf;
        (rc, dc_tok) = match tx_class {
            TxClass::TwoD => decode_coefs_class::<{ TxClass::TwoD as _ }, BD>(
                ts_c, t_dim, chroma, scratch, eob, tx, dbg, cf,
            ),
            TxClass::H => decode_coefs_class::<{ TxClass::H as _ }, BD>(
                ts_c, t_dim, chroma, scratch, eob, tx, dbg, cf,
            ),
            TxClass::V => decode_coefs_class::<{ TxClass::V as _ }, BD>(
                ts_c, t_dim, chroma, scratch, eob, tx, dbg, cf,
            ),
        };
    } else {
        let eob_cdf = &mut ts_c.cdf.coef.eob_base_tok[t_dim.ctx as usize][chroma];
        let hi_cdf = &mut ts_c.cdf.coef.br_tok[cmp::min(t_dim.ctx, 3) as usize][chroma];

        // dc-only
        let tok_br = rav1d_msac_decode_symbol_adapt4(&mut ts_c.msac, &mut eob_cdf[0], 2) as c_uint;
        dc_tok = 1 + tok_br;
        if dbg {
            println!(
                "Post-dc_lo_tok[{}][{}][{}][{}]: r={}",
                t_dim.ctx, chroma, 0, dc_tok, ts_c.msac.rng,
            );
        }
        if tok_br == 2 {
            dc_tok = rav1d_msac_decode_hi_tok(&mut ts_c.msac, &mut hi_cdf[0]) as c_uint;
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
    let dq = match ts.dq.get() {
        TileStateRef::Frame => &f.dq,
        TileStateRef::Local => &ts.dqmem,
    };
    let dq_tbl = &dq[b.seg_id.get()][plane];
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
        })) as i32;
    let mut cul_level: c_uint;
    let dc_sign_level: c_uint;

    enum Ac<'a> {
        Qm(&'a [u8]),
        NoQm,
    }

    let ac;
    if dc_tok == 0 {
        cul_level = 0;
        dc_sign_level = 1 << 6;
        ac = Some(match qm_tbl {
            Some(qm_tbl) => Ac::Qm(qm_tbl),
            None => Ac::NoQm,
        });
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

        dc_dq = dq_tbl[0].get() as c_int;
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
                dc_dq = ((dc_dq as c_uint).wrapping_mul(dc_tok as c_uint) & 0xffffff) as c_int;
            } else {
                dc_dq = (dc_dq as c_uint).wrapping_mul(dc_tok as c_uint) as c_int;
                assert!(dc_dq <= 0xffffff);
            }
            cul_level = dc_tok;
            dc_dq >>= dq_shift;
            dc_dq = cmp::min(dc_dq, cf_max + dc_sign);
            cf.set(0, if dc_sign != 0 { -dc_dq } else { dc_dq });

            ac = if rc != 0 { Some(Ac::Qm(qm_tbl)) } else { None };
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
                dc_dq = (((dc_dq as c_uint).wrapping_mul(dc_tok as c_uint)
                    & 0xffffff as c_int as c_uint)
                    >> dq_shift) as c_int;
                dc_dq = cmp::min(dc_dq, cf_max + dc_sign);
            } else {
                dc_dq = ((dc_dq as c_uint).wrapping_mul(dc_tok as c_uint) >> dq_shift) as c_int;
                assert!(dc_dq <= cf_max);
            }
            cul_level = dc_tok;
            cf.set(0, if dc_sign != 0 { -dc_dq } else { dc_dq });

            ac = if rc != 0 { Some(Ac::NoQm) } else { None };
        }
    }
    match ac {
        Some(Ac::Qm(qm_tbl)) => {
            let ac_dq: c_uint = dq_tbl[1].get() as c_uint;
            loop {
                let sign = rav1d_msac_decode_bool_equi(&mut ts_c.msac);
                if dbg {
                    println!("Post-sign[{}={}]: r={}", rc, sign, ts_c.msac.rng);
                }
                let rc_tok = cf.get(rc) as u32;
                let mut tok;
                let mut dq: c_uint = ac_dq
                    .wrapping_mul(qm_tbl[rc as usize] as c_uint)
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
                    assert!(dq <= 0xffffff); // Optimized out.
                }
                cul_level = cul_level.wrapping_add(tok);
                dq >>= dq_shift;
                dq_sat = cmp::min(dq as c_int, cf_max + sign as i32);
                cf.set(rc, if sign { -dq_sat } else { dq_sat });

                rc = rc_tok as u16 & 0x3ff;
                if !(rc != 0) {
                    break;
                }
            }
        }
        Some(Ac::NoQm) => {
            let ac_dq: c_uint = dq_tbl[1].get() as c_uint;
            loop {
                let sign = rav1d_msac_decode_bool_equi(&mut ts_c.msac) as c_int;
                if dbg {
                    println!("Post-sign[{}={}]: r={}", rc, sign, ts_c.msac.rng);
                }
                let rc_tok = cf.get(rc) as u32;
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
                cf.set(rc, if sign != 0 { -dq } else { dq });

                rc = rc_tok as u16 & 0x3ff; // next non-zero `rc`, zero if `eob`
                if !(rc != 0) {
                    break;
                }
            }
        }
        None => {}
    }

    // context
    *res_ctx = (cmp::min(cul_level, 63) | dc_sign_level) as u8;

    eob as i32
}

#[derive(Clone, Copy)]
enum CfSelect {
    /// Use `f.frame_thread.cf` at the specified offset.
    Frame(u32),

    /// Use `t.cf`.
    Task,
}

fn read_coef_tree<BD: BitDepth>(
    f: &Rav1dFrameData,
    t: &mut Rav1dTaskContext,
    mut ts_c: Option<&mut Rav1dTileStateContext>,
    bs: BlockSize,
    b: &Av1Block,
    ytx: TxfmSize,
    depth: usize,
    tx_split: [u16; 2],
    x_off: c_int,
    y_off: c_int,
    mut y_dst: Option<Rav1dPictureDataComponentOffset>,
) {
    let bd = BD::from_c(f.bitdepth_max);

    let ts = &f.ts[t.ts];
    let t_dim = &dav1d_txfm_dimensions[ytx as usize];
    let txw = t_dim.w;
    let txh = t_dim.h;

    // `y_off` can be larger than 3 since lossless blocks
    // use `TX_4X4` but can't be splitted.
    // Avoids an undefined left shift.
    if depth < 2 && tx_split[depth] != 0 && tx_split[depth] & 1 << y_off * 4 + x_off != 0 {
        let sub = t_dim.sub;
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
            y_dst,
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
                y_dst.map(|dst| dst + (4 * txsw as usize)),
            );
        }
        t.b.x -= txsw as c_int;
        t.b.y += txsh as c_int;
        if txh >= txw && t.b.y < f.bh {
            y_dst = y_dst.map(|dst| dst + (4 * txsh as isize * dst.pixel_stride::<BD>()));
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
                y_dst,
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
                    y_dst.map(|dst| dst + (4 * txsw as usize)),
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

        let cf =
            if t.frame_thread.pass != 0 {
                let p = t.frame_thread.pass & 1;
                CfSelect::Frame(ts.frame_thread[p as usize].cf.get_update(|i| {
                    i + cmp::min(t_dim.w, 8) as u32 * cmp::min(t_dim.h, 8) as u32 * 16
                }))
            } else {
                CfSelect::Task
            };
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
                    "Post-y-cf-blk[tx={:?},txtp={},eob={}]: r={}",
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
                let cbi_idx = ts.frame_thread[1].cbi_idx.get_update(|i| i + 1);
                f.frame_thread.cbi[cbi_idx as usize].set(CodedBlockInfo::new(eob as i16, txtp));
            }
        } else {
            let cbi_idx = ts.frame_thread[0].cbi_idx.get_update(|i| i + 1);
            let cbi = f.frame_thread.cbi[cbi_idx as usize].get();
            eob = cbi.eob().into();
            txtp = cbi.txtp();
        }
        if t.frame_thread.pass & 1 == 0 {
            let y_dst = y_dst.unwrap();
            if eob >= 0 {
                let cf = match cf {
                    CfSelect::Frame(offset) => {
                        let len =
                            cmp::min(t_dim.h as usize, 8) * 4 * cmp::min(t_dim.w as usize, 8) * 4;
                        &mut *f.frame_thread.cf.mut_slice_as((offset as usize.., ..len))
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
                f.dsp.itx.itxfm_add[ytx as usize][txtp as usize].call::<BD>(y_dst, cf, eob, bd);
                if debug_block_info!(f, t.b) && DEBUG_B_PIXELS {
                    hex_dump_pic::<BD>(y_dst, t_dim.w as usize * 4, t_dim.h as usize * 4, "recon");
                }
            }
        }
    };
}

pub(crate) fn rav1d_read_coef_blocks<BD: BitDepth>(
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
    let b_dim = bs.dimensions();
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
                            // Increment moved up from after `decode_coefs` call in C.
                            // This is fine since `decode_coefs` does not access `cf`.
                            // `decode_coefs` must not be changed to access `cf`.
                            let cf = CfSelect::Frame(ts.frame_thread[1].cf.get_update(|i| {
                                i + cmp::min(t_dim.w, 8) as u32 * cmp::min(t_dim.h, 8) as u32 * 16
                            }));
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
                                cf,
                                &mut txtp,
                                &mut cf_ctx,
                            );
                            if debug_block_info!(f, t.b) {
                                println!(
                                    "Post-y-cf-blk[tx={:?},txtp={},eob={}]: r={}",
                                    intra.tx, txtp, eob, ts_c.msac.rng,
                                );
                            }
                            let cbi_idx = ts.frame_thread[1].cbi_idx.get_update(|i| i + 1);
                            f.frame_thread.cbi[cbi_idx as usize]
                                .set(CodedBlockInfo::new(eob as i16, txtp));
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
                        let cf_idx = ts.frame_thread[1].cf.get();
                        let eob = decode_coefs::<BD>(
                            f,
                            t.ts,
                            ts_c,
                            debug_block_info!(f, t.b),
                            &mut t.scratch,
                            &mut t.cf,
                            &mut a_ccoef.index_mut((a_start.., ..a_len)),
                            &mut l_ccoef.index_mut((l_start.., ..l_len)),
                            b.uvtx,
                            bs,
                            b,
                            1 + pl,
                            CfSelect::Frame(cf_idx),
                            &mut txtp,
                            &mut cf_ctx,
                        );
                        if debug_block_info!(f, t.b) {
                            println!(
                                "Post-uv-cf-blk[pl={},tx={:?},txtp={},eob={}]: r={}",
                                pl, b.uvtx, txtp, eob, ts_c.msac.rng,
                            );
                        }
                        let cbi_idx = ts.frame_thread[1].cbi_idx.get_update(|i| i + 1);
                        f.frame_thread.cbi[cbi_idx as usize]
                            .set(CodedBlockInfo::new(eob as i16, txtp));
                        ts.frame_thread[1]
                            .cf
                            .set(cf_idx + uv_t_dim.w as u32 * uv_t_dim.h as u32 * 16);
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

enum MaybeTempPixels<'a, TmpStride> {
    NonTemp {
        dst: Rav1dPictureDataComponentOffset<'a>,
    },
    Temp {
        tmp: &'a mut [i16],
        tmp_stride: TmpStride,
    },
}

fn mc<BD: BitDepth>(
    f: &Rav1dFrameData,
    emu_edge: &mut ScratchEmuEdge,
    b: Bxy,
    dst: MaybeTempPixels<()>,
    bw4: c_int,
    bh4: c_int,
    bx: c_int,
    by: c_int,
    pl: usize,
    mv: Mv,
    refp: &Rav1dThreadPicture,
    refidx: usize,
    filter_2d: Filter2d,
) -> Result<(), ()> {
    let bd = BD::from_c(f.bitdepth_max);
    let ref_data = &refp.p.data.as_ref().unwrap().data;
    let cur_data = &f.cur.data.as_ref().unwrap().data;

    let ss_ver = (pl != 0 && f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
    let ss_hor = (pl != 0 && f.cur.p.layout != Rav1dPixelLayout::I444) as c_int;
    let h_mul = 4 >> ss_hor;
    let v_mul = 4 >> ss_ver;
    let mvx = mv.x as c_int;
    let mvy = mv.y as c_int;
    let mx = mvx & 15 >> (ss_hor == 0) as c_int;
    let my = mvy & 15 >> (ss_ver == 0) as c_int;

    if refp.p.p.w == f.cur.p.w && refp.p.p.h == f.cur.p.h {
        let dx = bx * h_mul + (mvx >> 3 + ss_hor);
        let dy = by * v_mul + (mvy >> 3 + ss_ver);
        let w;
        let h;

        if !ref_data[0].ref_eq(&cur_data[0]) {
            w = f.cur.p.w + ss_hor >> ss_hor;
            h = f.cur.p.h + ss_ver >> ss_ver;
        } else {
            w = f.bw * 4 >> ss_hor;
            h = f.bh * 4 >> ss_ver;
        }
        let r#ref = if dx < (mx != 0) as c_int * 3
            || dy < (my != 0) as c_int * 3
            || dx + bw4 * h_mul + (mx != 0) as c_int * 4 > w
            || dy + bh4 * v_mul + (my != 0) as c_int * 4 > h
        {
            let emu_edge_buf = emu_edge.buf_mut::<BD>();
            f.dsp.mc.emu_edge.call::<BD>(
                (bw4 * h_mul + (mx != 0) as c_int * 7) as intptr_t,
                (bh4 * v_mul + (my != 0) as c_int * 7) as intptr_t,
                w as intptr_t,
                h as intptr_t,
                (dx - (mx != 0) as c_int * 3) as intptr_t,
                (dy - (my != 0) as c_int * 3) as intptr_t,
                emu_edge_buf,
                192,
                &ref_data[pl],
            );
            let stride = 192;
            Rav1dPictureDataComponentOffset {
                data: &Rav1dPictureDataComponent::wrap_buf::<BD>(emu_edge_buf, stride),
                offset: stride * (my != 0) as usize * 3 + (mx != 0) as usize * 3,
            }
        } else {
            let r#ref = &ref_data[pl];
            r#ref.with_offset::<BD>() + (dy as isize * r#ref.pixel_stride::<BD>()) + dx as usize
        };

        let w = bw4 * h_mul;
        let h = bh4 * v_mul;
        let mx = mx << (ss_hor == 0) as u8;
        let my = my << (ss_ver == 0) as u8;
        match dst {
            MaybeTempPixels::NonTemp { dst } => {
                f.dsp.mc.mc[filter_2d].call::<BD>(dst, r#ref, w, h, mx, my, bd);
            }
            MaybeTempPixels::Temp { tmp, tmp_stride: _ } => {
                f.dsp.mc.mct[filter_2d].call::<BD>(tmp, r#ref, w, h, mx, my, bd);
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
        let r#ref = if left < 3 || top < 3 || right + 4 > w || bottom + 4 > h {
            let emu_edge_buf = emu_edge.buf_mut::<BD>();
            f.dsp.mc.emu_edge.call::<BD>(
                (right - left + 7) as intptr_t,
                (bottom - top + 7) as intptr_t,
                w as intptr_t,
                h as intptr_t,
                (left - 3) as intptr_t,
                (top - 3) as intptr_t,
                emu_edge_buf,
                320,
                &ref_data[pl],
            );
            if debug_block_info!(f, b) {
                println!("Emu");
            }
            let stride = 320;
            Rav1dPictureDataComponentOffset {
                data: &Rav1dPictureDataComponent::wrap_buf::<BD>(emu_edge_buf, stride),
                offset: stride * 3 + 3,
            }
        } else {
            let r#ref = &ref_data[pl];
            r#ref.with_offset::<BD>() + (top as isize * r#ref.pixel_stride::<BD>()) + left as isize
        };

        let w = bw4 * h_mul;
        let h = bh4 * v_mul;
        let mx = pos_x & 0x3ff;
        let my = pos_y & 0x3ff;
        let dx = f.svc[refidx][0].step;
        let dy = f.svc[refidx][1].step;
        match dst {
            MaybeTempPixels::NonTemp { dst } => {
                f.dsp.mc.mc_scaled[filter_2d].call::<BD>(dst, r#ref, w, h, mx, my, dx, dy, bd);
            }
            MaybeTempPixels::Temp { tmp, tmp_stride: _ } => {
                f.dsp.mc.mct_scaled[filter_2d].call::<BD>(tmp, r#ref, w, h, mx, my, dx, dy, bd);
            }
        }
    }

    Ok(())
}

fn obmc<BD: BitDepth>(
    f: &Rav1dFrameData,
    t: &mut Rav1dTaskContext,
    dst: Rav1dPictureDataComponentOffset,
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
    let lap = scratch.lap_inter.lap_mut::<BD>();
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
            let a_b_dim = a_r.bs.dimensions();
            let step4 = clip(a_b_dim[0], 2, 16);

            if a_r.r#ref.r#ref[0] > 0 {
                let ow4 = cmp::min(step4, b_dim[0]);
                let oh4 = cmp::min(b_dim[1], 16) >> 1;
                mc::<BD>(
                    f,
                    &mut scratch.emu_edge,
                    t.b,
                    MaybeTempPixels::NonTemp {
                        dst: Rav1dPictureDataComponentOffset {
                            data: &Rav1dPictureDataComponent::wrap_buf::<BD>(
                                lap,
                                ow4 as usize * h_mul as usize,
                            ),
                            offset: 0,
                        },
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
                f.dsp.mc.blend_h.call::<BD>(
                    dst + (x * h_mul) as usize,
                    lap,
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
            let l_b_dim = l_r.bs.dimensions();
            let step4 = clip(l_b_dim[1], 2, 16);

            if l_r.r#ref.r#ref[0] > 0 {
                let ow4 = cmp::min(b_dim[0], 16) >> 1;
                let oh4 = cmp::min(step4, b_dim[1]);
                mc::<BD>(
                    f,
                    &mut scratch.emu_edge,
                    t.b,
                    MaybeTempPixels::NonTemp {
                        dst: Rav1dPictureDataComponentOffset {
                            data: &Rav1dPictureDataComponent::wrap_buf::<BD>(
                                lap,
                                ow4 as usize * h_mul as usize,
                            ),
                            offset: 0,
                        },
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
                f.dsp.mc.blend_v.call::<BD>(
                    dst + (y * v_mul) as isize * dst.pixel_stride::<BD>(),
                    lap,
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

fn warp_affine<BD: BitDepth>(
    f: &Rav1dFrameData,
    emu_edge: &mut ScratchEmuEdge,
    b: Bxy,
    mut dst: MaybeTempPixels<usize>,
    b_dim: &[u8; 4],
    pl: usize,
    refp: &Rav1dThreadPicture,
    wmp: &Rav1dWarpedMotionParams,
) -> Result<(), ()> {
    let abcd = &wmp.abcd.get();
    let bd = BD::from_c(f.bitdepth_max);
    let ref_data = &refp.p.data.as_ref().unwrap().data;

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

            let r#ref = if dx < 3 || dx + 8 + 4 > width || dy < 3 || dy + 8 + 4 > height {
                let emu_edge_buf = emu_edge.buf_mut::<BD>();
                f.dsp.mc.emu_edge.call::<BD>(
                    15,
                    15,
                    width as intptr_t,
                    height as intptr_t,
                    (dx - 3) as intptr_t,
                    (dy - 3) as intptr_t,
                    emu_edge_buf,
                    32,
                    &ref_data[pl],
                );
                let stride = 32;
                Rav1dPictureDataComponentOffset {
                    data: &Rav1dPictureDataComponent::wrap_buf::<BD>(emu_edge_buf, stride),
                    offset: stride * 3 + 3,
                }
            } else {
                let r#ref = &ref_data[pl];
                r#ref.with_offset::<BD>() + (dy as isize * r#ref.pixel_stride::<BD>()) + dx as usize
            };
            let x = x as usize;
            match dst {
                MaybeTempPixels::Temp {
                    ref mut tmp,
                    tmp_stride,
                } => {
                    f.dsp
                        .mc
                        .warp8x8t
                        .call(&mut tmp[x..], tmp_stride, r#ref, abcd, mx, my, bd);
                }
                MaybeTempPixels::NonTemp { dst } => {
                    f.dsp.mc.warp8x8.call(dst + x, r#ref, abcd, mx, my, bd);
                }
            }
        }
        dst = match dst {
            MaybeTempPixels::NonTemp { dst } => MaybeTempPixels::NonTemp {
                dst: dst + 8 * dst.pixel_stride::<BD>(),
            },
            MaybeTempPixels::Temp { tmp, tmp_stride } => MaybeTempPixels::Temp {
                tmp: &mut tmp[8 * tmp_stride..],
                tmp_stride,
            },
        };
    }
    Ok(())
}

pub(crate) fn rav1d_recon_b_intra<BD: BitDepth>(
    f: &Rav1dFrameData,
    t: &mut Rav1dTaskContext,
    mut ts_c: Option<&mut Rav1dTileStateContext>,
    bs: BlockSize,
    intra_edge_flags: EdgeFlags,
    b: &Av1Block,
    intra: &Av1BlockIntra,
) {
    let bd = BD::from_c(f.bitdepth_max);
    let cur_data = &f.cur.data.as_ref().unwrap().data;
    let ts = &f.ts[t.ts];

    let bx4 = t.b.x & 31;
    let by4 = t.b.y & 31;
    let ss_ver = (f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
    let ss_hor = (f.cur.p.layout != Rav1dPixelLayout::I444) as c_int;
    let cbx4 = bx4 >> ss_hor;
    let cby4 = by4 >> ss_ver;
    let b_dim = bs.dimensions();
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
                let y_dst = &cur_data[0];
                let y_dst = y_dst.with_offset::<BD>()
                    + 4 * (t.b.y as isize * y_dst.pixel_stride::<BD>() + t.b.x as isize);
                let scratch = t.scratch.inter_intra_mut();
                let pal_idx = if t.frame_thread.pass != 0 {
                    let p = (t.frame_thread.pass & 1) as usize;
                    let frame_thread = &ts.frame_thread[p];
                    let len = (bw4 * bh4 * 8) as u32;
                    let pal_idx = frame_thread.pal_idx.get_update(|i| i + len);
                    &*f.frame_thread
                        .pal_idx
                        .index((pal_idx as usize.., ..len as usize))
                } else {
                    &scratch.pal_idx_y
                };
                let pal = if t.frame_thread.pass != 0 {
                    let x = t.b.x as usize;
                    let y = t.b.y as usize;
                    let index =
                        ((y >> 1) + (x & 1)) * (f.b4_stride as usize >> 1) + (x >> 1) + (y & 1);
                    &*f.frame_thread.pal.index::<BD>(index)
                } else {
                    scratch.interintra_edge_pal.pal.buf::<BD>()
                };
                f.dsp
                    .ipred
                    .pal_pred
                    .call::<BD>(y_dst, &pal[0], pal_idx, bw4 * 4, bh4 * 4);
                if debug_block_info!(f, t.b) && DEBUG_B_PIXELS {
                    hex_dump_pic::<BD>(y_dst, bw4 as usize * 4, bh4 as usize * 4, "y-pal-pred");
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
                let y_dst = &cur_data[0];
                let mut y_dst = y_dst.with_offset::<BD>()
                    + 4 * (t.b.y as isize * y_dst.pixel_stride::<BD>()
                        + t.b.x as isize
                        + init_x as isize);
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
                        m = rav1d_prepare_intra_edges(
                            t.b.x,
                            t.b.x > ts.tiling.col_start,
                            t.b.y,
                            t.b.y > ts.tiling.row_start,
                            ts.tiling.col_end,
                            ts.tiling.row_end,
                            edge_flags,
                            y_dst,
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
                        f.dsp.ipred.intra_pred[m as usize].call(
                            y_dst,
                            edge_array,
                            edge_offset,
                            t_dim.w as c_int * 4,
                            t_dim.h as c_int * 4,
                            angle | intra_flags,
                            4 * f.bw - 4 * t.b.x,
                            4 * f.bh - 4 * t.b.y,
                            bd,
                        );

                        if debug_block_info!(f, t.b) && DEBUG_B_PIXELS {
                            hex_dump::<BD>(
                                &edge_array[edge_offset - t_dim.h as usize * 4..],
                                t_dim.h as usize * 4,
                                t_dim.h as usize * 4,
                                2,
                                "l",
                            );
                            hex_dump::<BD>(&edge_array[edge_offset..], 0, 1, 1, "tl");
                            hex_dump::<BD>(
                                &edge_array[edge_offset + 1..],
                                t_dim.w as usize * 4,
                                t_dim.w as usize * 4,
                                2,
                                "t",
                            );
                            hex_dump_pic::<BD>(
                                y_dst,
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
                            let len =
                                cmp::min(t_dim.h, 8) as u32 * 4 * cmp::min(t_dim.w, 8) as u32 * 4;
                            let cf_idx = ts.frame_thread[p].cf.get_update(|i| i + len);
                            cf_guard = f
                                .frame_thread
                                .cf
                                .mut_slice_as((cf_idx as usize.., ..len as usize));
                            cf = &mut *cf_guard;
                            let cbi_idx = ts.frame_thread[p].cbi_idx.get_update(|i| i + 1);
                            let cbi = f.frame_thread.cbi[cbi_idx as usize].get();
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
                                intra.tx,
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
                                    "Post-y-cf-blk[tx={:?},txtp={},eob={}]: r={}",
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
                            f.dsp.itx.itxfm_add[intra.tx as usize][txtp as usize]
                                .call::<BD>(y_dst, cf, eob, bd);
                            if debug_block_info!(f, t.b) && DEBUG_B_PIXELS {
                                hex_dump_pic::<BD>(
                                    y_dst,
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
                    y_dst += 4 * t_dim.w as usize;
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
                let y_src = &cur_data[0];
                let y_src = y_src.with_offset::<BD>()
                    + 4 * (t.b.x & !ss_hor) as usize
                    + 4 * (t.b.y & !ss_ver) as isize * y_src.pixel_stride::<BD>();
                let uv_off = 4
                    * ((t.b.x >> ss_hor) as isize
                        + (t.b.y >> ss_ver) as isize * BD::pxstride(stride));

                let furthest_r = (cw4 << ss_hor) + t_dim.w as c_int - 1 & !(t_dim.w as c_int - 1);
                let furthest_b = (ch4 << ss_ver) + t_dim.h as c_int - 1 & !(t_dim.h as c_int - 1);
                let layout = f.cur.p.layout.try_into().unwrap();
                f.dsp.ipred.cfl_ac[layout].call::<BD>(
                    ac,
                    y_src,
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
                    let uv_dst = &cur_data[1 + pl];
                    let uv_dst = uv_dst.with_offset::<BD>()
                        + 4 * ((t.b.x >> ss_hor) as isize
                            + (t.b.y >> ss_ver) as isize * uv_dst.pixel_stride::<BD>());
                    let m: IntraPredMode = rav1d_prepare_intra_edges(
                        xpos,
                        xpos > xstart,
                        ypos,
                        ypos > ystart,
                        ts.tiling.col_end >> ss_hor,
                        ts.tiling.row_end >> ss_ver,
                        EdgeFlags::empty(),
                        uv_dst,
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
                    f.dsp.ipred.cfl_pred[m as usize].call(
                        uv_dst,
                        edge_array,
                        edge_offset,
                        uv_t_dim.w as c_int * 4,
                        uv_t_dim.h as c_int * 4,
                        ac,
                        intra.cfl_alpha[pl] as c_int,
                        bd,
                    );
                }
                if debug_block_info!(&*f, t.b) && DEBUG_B_PIXELS {
                    ac_dump(ac, 4 * cbw4 as usize, 4 * cbh4 as usize, "ac");
                    for pl in 1..3 {
                        let uv_dst = cur_data[pl].with_offset::<BD>() + uv_off;
                        hex_dump_pic::<BD>(
                            uv_dst,
                            cbw4 as usize * 4,
                            cbh4 as usize * 4,
                            ["", "u-cfl-pred", "v-cfl-pred"][pl],
                        );
                    }
                }
            } else if intra.pal_sz[1] != 0 {
                let uv_dstoff = 4
                    * ((t.b.x >> ss_hor) as isize
                        + (t.b.y >> ss_ver) as isize * BD::pxstride(f.cur.stride[1]));
                let (pal, pal_idx) = if t.frame_thread.pass != 0 {
                    let p = (t.frame_thread.pass & 1) as usize;
                    let x = t.b.x as usize;
                    let y = t.b.y as usize;
                    let index =
                        ((y >> 1) + (x & 1)) * (f.b4_stride as usize >> 1) + (x >> 1) + (y & 1);
                    let len = (cbw4 * cbh4 * 8) as u32;
                    let pal_idx_offset = ts.frame_thread[p].pal_idx.get_update(|i| i + len);
                    (
                        &*f.frame_thread.pal.index::<BD>(index),
                        &*f.frame_thread
                            .pal_idx
                            .index((pal_idx_offset as usize.., ..len as usize)),
                    )
                } else {
                    let scratch = t.scratch.inter_intra_mut();
                    (
                        scratch.interintra_edge_pal.pal.buf::<BD>(),
                        scratch.pal_idx_uv.as_slice(),
                    )
                };

                for pl in 1..3 {
                    let uv = cur_data[pl].with_offset::<BD>() + uv_dstoff;
                    f.dsp
                        .ipred
                        .pal_pred
                        .call::<BD>(uv, &pal[pl], pal_idx, cbw4 * 4, cbh4 * 4);
                    if debug_block_info!(f, t.b) && DEBUG_B_PIXELS {
                        hex_dump_pic::<BD>(
                            uv,
                            cbw4 as usize * 4,
                            cbh4 as usize * 4,
                            ["", "u-pal-pred", "v-pal-pred"][pl],
                        );
                    }
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
                    let uv_dst = &cur_data[1 + pl];
                    let mut uv_dst = uv_dst.with_offset::<BD>()
                        + 4 * ((t.b.y >> ss_ver) as isize * uv_dst.pixel_stride::<BD>()
                            + (t.b.x + init_x >> ss_hor) as isize);
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
                            m = rav1d_prepare_intra_edges(
                                xpos,
                                xpos > xstart,
                                ypos,
                                ypos > ystart,
                                ts.tiling.col_end >> ss_hor,
                                ts.tiling.row_end >> ss_ver,
                                edge_flags,
                                uv_dst,
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
                            f.dsp.ipred.intra_pred[m as usize].call(
                                uv_dst,
                                edge_array,
                                edge_offset,
                                uv_t_dim.w as c_int * 4,
                                uv_t_dim.h as c_int * 4,
                                angle | sm_uv_fl,
                                4 * f.bw + ss_hor - 4 * (t.b.x & !ss_hor) >> ss_hor,
                                4 * f.bh + ss_ver - 4 * (t.b.y & !ss_ver) >> ss_ver,
                                bd,
                            );
                            if debug_block_info!(f, t.b) && DEBUG_B_PIXELS {
                                hex_dump::<BD>(
                                    &edge_array[edge_offset - uv_t_dim.h as usize * 4..],
                                    uv_t_dim.h as usize * 4,
                                    uv_t_dim.h as usize * 4,
                                    2,
                                    "l",
                                );
                                hex_dump::<BD>(&edge_array[edge_offset..], 0, 1, 1, "tl");
                                hex_dump::<BD>(
                                    &edge_array[edge_offset + 1..],
                                    uv_t_dim.w as usize * 4,
                                    uv_t_dim.w as usize * 4,
                                    2,
                                    "t",
                                );
                                hex_dump_pic::<BD>(
                                    uv_dst,
                                    uv_t_dim.w as usize * 4,
                                    uv_t_dim.h as usize * 4,
                                    ["u-intra-pred", "v-intra-pred"][pl],
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
                                let len = uv_t_dim.w as u32 * 4 * uv_t_dim.h as u32 * 4;
                                let cf_idx = ts.frame_thread[p].cf.get_update(|i| i + len);
                                cf_guard = f
                                    .frame_thread
                                    .cf
                                    .mut_slice_as((cf_idx as usize.., ..len as usize));
                                cf = &mut *cf_guard;
                                let cbi_idx = ts.frame_thread[p].cbi_idx.get_update(|i| i + 1);
                                let cbi = f.frame_thread.cbi[cbi_idx as usize].get();
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
                                            "Post-uv-cf-blk[pl={},tx={:?},txtp={},eob={}]: r={} [x={},cbx4={}]",
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
                                f.dsp.itx.itxfm_add[b.uvtx as usize][txtp as usize]
                                    .call::<BD>(uv_dst, cf, eob, bd);
                                if debug_block_info!(f, t.b) && DEBUG_B_PIXELS {
                                    hex_dump_pic::<BD>(
                                        uv_dst,
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
                        uv_dst += uv_t_dim.w as usize * 4;
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

pub(crate) fn rav1d_recon_b_inter<BD: BitDepth>(
    f: &Rav1dFrameData,
    t: &mut Rav1dTaskContext,
    mut ts_c: Option<&mut Rav1dTileStateContext>,
    bs: BlockSize,
    b: &Av1Block,
    inter: &Av1BlockInter,
) -> Result<(), ()> {
    let bd = BD::from_c(f.bitdepth_max);
    let cur_data = &f.cur.data.as_ref().unwrap().data;

    let ts = &f.ts[t.ts];
    let bx4 = t.b.x & 31;
    let by4 = t.b.y & 31;
    let ss_ver = (f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
    let ss_hor = (f.cur.p.layout != Rav1dPixelLayout::I444) as c_int;
    let cbx4 = bx4 >> ss_hor;
    let cby4 = by4 >> ss_ver;
    let b_dim = bs.dimensions();
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
    let y_dst = &cur_data[0];
    let mut y_dst = y_dst.with_offset::<BD>()
        + 4 * (t.b.y as isize * y_dst.pixel_stride::<BD>() + t.b.x as isize);
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
            MaybeTempPixels::NonTemp { dst: y_dst },
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
                        dst: cur_data[pl].with_offset::<BD>() + uvdstoff,
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
                f.dsp
                    .mc
                    .avg
                    .call::<BD>(y_dst, &tmp[0], &tmp[1], bw4 * 4, bh4 * 4, bd);
            }
            CompInterType::WeightedAvg => {
                jnt_weight =
                    f.jnt_weights[inter.r#ref[0] as usize][inter.r#ref[1] as usize] as c_int;
                f.dsp.mc.w_avg.call::<BD>(
                    y_dst,
                    &tmp[0],
                    &tmp[1],
                    bw4 * 4,
                    bh4 * 4,
                    jnt_weight,
                    bd,
                );
            }
            CompInterType::Seg => {
                f.dsp.mc.w_mask[chr_layout_idx_w_mask].call(
                    y_dst,
                    &tmp[inter.nd.one_d.mask_sign() as usize],
                    &tmp[!inter.nd.one_d.mask_sign() as usize],
                    bw4 * 4,
                    bh4 * 4,
                    seg_mask,
                    inter.nd.one_d.mask_sign() as c_int,
                    bd,
                );
                mask = &seg_mask[..];
            }
            CompInterType::Wedge => {
                mask = dav1d_wedge_masks[bs as usize][0][0][inter.nd.one_d.wedge_idx as usize];
                f.dsp.mc.mask.call::<BD>(
                    y_dst,
                    &tmp[inter.nd.one_d.mask_sign() as usize],
                    &tmp[!inter.nd.one_d.mask_sign() as usize],
                    bw4 * 4,
                    bh4 * 4,
                    mask,
                    bd,
                );
                if has_chroma {
                    mask = dav1d_wedge_masks[bs as usize][chr_layout_idx]
                        [inter.nd.one_d.mask_sign() as usize]
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

                let uv_dst = cur_data[1 + pl].with_offset::<BD>() + uvdstoff;
                match comp_inter_type {
                    CompInterType::Avg => {
                        f.dsp.mc.avg.call::<BD>(
                            uv_dst,
                            &tmp[0],
                            &tmp[1],
                            bw4 * 4 >> ss_hor,
                            bh4 * 4 >> ss_ver,
                            bd,
                        );
                    }
                    CompInterType::WeightedAvg => {
                        f.dsp.mc.w_avg.call::<BD>(
                            uv_dst,
                            &tmp[0],
                            &tmp[1],
                            bw4 * 4 >> ss_hor,
                            bh4 * 4 >> ss_ver,
                            jnt_weight,
                            bd,
                        );
                    }
                    CompInterType::Seg | CompInterType::Wedge => {
                        f.dsp.mc.mask.call::<BD>(
                            uv_dst,
                            &tmp[inter.nd.one_d.mask_sign() as usize],
                            &tmp[!inter.nd.one_d.mask_sign() as usize],
                            bw4 * 4 >> ss_hor,
                            bh4 * 4 >> ss_ver,
                            mask,
                            bd,
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
                MaybeTempPixels::NonTemp { dst: y_dst },
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
                MaybeTempPixels::NonTemp { dst: y_dst },
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
                obmc::<BD>(f, t, y_dst, b_dim, 0, bx4, by4, w4, h4)?;
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
            m = rav1d_prepare_intra_edges(
                t.b.x,
                t.b.x > ts.tiling.col_start,
                t.b.y,
                t.b.y > ts.tiling.row_start,
                ts.tiling.col_end,
                ts.tiling.row_end,
                EdgeFlags::empty(),
                y_dst,
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
            let tmp = interintra_edge_pal.interintra.buf_mut::<BD>();
            f.dsp.ipred.intra_pred[m as usize].call(
                Rav1dPictureDataComponentOffset {
                    data: &Rav1dPictureDataComponent::wrap_buf::<BD>(tmp, 4 * bw4 as usize),
                    offset: 0,
                },
                tl_edge_array,
                tl_edge_offset,
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
            f.dsp
                .mc
                .blend
                .call::<BD>(y_dst, tmp, bw4 * 4, bh4 * 4, ii_mask);
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
                                dst: cur_data[1 + pl].with_offset::<BD>() + uvdstoff,
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
                                dst: cur_data[1 + pl].with_offset::<BD>() + uvdstoff + v_off,
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
                                dst: cur_data[1 + pl].with_offset::<BD>() + uvdstoff + h_off,
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
                            dst: cur_data[1 + pl].with_offset::<BD>() + uvdstoff + h_off + v_off,
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
                                dst: cur_data[1 + pl].with_offset::<BD>() + uvdstoff,
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
                                dst: cur_data[1 + pl].with_offset::<BD>() + uvdstoff,
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
                        let uv_dst = cur_data[1 + pl].with_offset::<BD>() + uvdstoff;
                        if inter.motion_mode == MotionMode::Obmc {
                            obmc::<BD>(f, t, uv_dst, b_dim, 1 + pl, bx4, by4, w4, h4)?;
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
                        let uv_dst = cur_data[1 + pl].with_offset::<BD>() + uvdstoff;
                        let top_sb_edge_slice = if t.b.y & f.sb_step - 1 == 0 {
                            let sby = t.b.y >> f.sb_shift;
                            let offset = (f.ipred_edge_off * (pl + 1)) as isize
                                + (f.sb128w * 128 * (sby - 1)) as isize;
                            Some((&f.ipred_edge, offset))
                        } else {
                            None
                        };
                        m = rav1d_prepare_intra_edges(
                            t.b.x >> ss_hor,
                            (t.b.x >> ss_hor) > (ts.tiling.col_start >> ss_hor),
                            t.b.y >> ss_ver,
                            (t.b.y >> ss_ver) > (ts.tiling.row_start >> ss_ver),
                            ts.tiling.col_end >> ss_hor,
                            ts.tiling.row_end >> ss_ver,
                            EdgeFlags::empty(),
                            uv_dst,
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
                        let tmp = interintra_edge_pal.interintra.buf_mut::<BD>();
                        f.dsp.ipred.intra_pred[m as usize].call(
                            Rav1dPictureDataComponentOffset {
                                data: &Rav1dPictureDataComponent::wrap_buf::<BD>(
                                    tmp,
                                    4 * cbw4 as usize,
                                ),
                                offset: 0,
                            },
                            tl_edge_array,
                            tl_edge_offset,
                            cbw4 * 4,
                            cbh4 * 4,
                            0,
                            0,
                            0,
                            bd,
                        );
                        f.dsp
                            .mc
                            .blend
                            .call::<BD>(uv_dst, tmp, cbw4 * 4, cbh4 * 4, ii_mask);
                    }
                }
            }
        }
        t.tl_4x4_filter = filter_2d;
    }

    if debug_block_info!(f, t.b) && DEBUG_B_PIXELS {
        hex_dump_pic::<BD>(
            y_dst,
            b_dim[0] as usize * 4,
            b_dim[1] as usize * 4,
            "y-pred",
        );
        if has_chroma {
            for pl in 1..3 {
                let uv_dst = cur_data[pl].with_offset::<BD>() + uvdstoff;
                hex_dump_pic::<BD>(
                    uv_dst,
                    cbw4 as usize * 4,
                    cbh4 as usize * 4,
                    ["", "u-pred", "v-pred"][pl],
                );
            }
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
            y_dst += y_dst.pixel_stride::<BD>() * 4 * init_y as isize;
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
                        Some(y_dst + (x as usize * 4)),
                    );
                    t.b.x += ytx.w as c_int;
                    x += ytx.w as c_int;
                    x_off += 1;
                }
                y_dst += y_dst.pixel_stride::<BD>() * 4 * ytx.h as isize;
                t.b.x -= x;
                t.b.y += ytx.h as c_int;
                y += ytx.h as c_int;
                y_off += 1;
            }
            y_dst -= y_dst.pixel_stride::<BD>() * 4 * y as isize;
            t.b.y -= y;

            // chroma coefs and inverse transform
            if has_chroma {
                for pl in 0..2 {
                    let uv_dst = &cur_data[1 + pl];
                    let mut uv_dst = uv_dst.with_offset::<BD>()
                        + uvdstoff
                        + (uv_dst.pixel_stride::<BD>() * init_y as isize * 4 >> ss_ver);
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
                                let p = t.frame_thread.pass as usize & 1;
                                let len = uvtx.h as u32 * 4 * uvtx.w as u32 * 4;
                                let cf_idx = ts.frame_thread[p].cf.get_update(|i| i + len);
                                cf_guard = f
                                    .frame_thread
                                    .cf
                                    .mut_slice_as((cf_idx as usize.., ..len as usize));
                                cf = &mut *cf_guard;
                                let cbi_idx = ts.frame_thread[p].cbi_idx.get_update(|i| i + 1);
                                let cbi = f.frame_thread.cbi[cbi_idx as usize].get();
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
                                        "Post-uv-cf-blk[pl={},tx={:?},txtp={},eob={}]: r={}",
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
                                f.dsp.itx.itxfm_add[b.uvtx as usize][txtp as usize].call::<BD>(
                                    uv_dst + 4 * x as usize,
                                    cf,
                                    eob,
                                    bd,
                                );
                                if debug_block_info!(f, t.b) && DEBUG_B_PIXELS {
                                    let uv_dst = uv_dst + (4 * x as usize);
                                    hex_dump_pic::<BD>(
                                        uv_dst,
                                        uvtx.w as usize * 4,
                                        uvtx.h as usize * 4,
                                        "recon",
                                    );
                                }
                            }
                            t.b.x += (uvtx.w as c_int) << ss_hor;
                            x += uvtx.w as c_int;
                        }
                        uv_dst += uv_dst.pixel_stride::<BD>() * 4 * uvtx.h as isize;
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

pub(crate) fn rav1d_filter_sbrow_deblock_cols<BD: BitDepth>(
    c: &Rav1dContext,
    f: &Rav1dFrameData,
    _t: &mut Rav1dTaskContext,
    sby: c_int,
) {
    if !c.inloop_filters.contains(Rav1dInloopFilterType::DEBLOCK) {
        return;
    }

    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    if frame_hdr.loopfilter.level_y == [0; 2] {
        return;
    }

    let y = sby * f.sb_step * 4;
    let p = f.cur.lf_offsets::<BD>(y);
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    let mask_offset = (sby >> (seq_hdr.sb128 == 0) as c_int) * f.sb128w;
    rav1d_loopfilter_sbrow_cols::<BD>(
        f,
        p,
        mask_offset as usize,
        sby,
        f.lf.start_of_tile_row[sby as usize] as c_int,
    );
}

pub(crate) fn rav1d_filter_sbrow_deblock_rows<BD: BitDepth>(
    c: &Rav1dContext,
    f: &Rav1dFrameData,
    _t: &mut Rav1dTaskContext,
    sby: c_int,
) {
    let y = sby * f.sb_step * 4;
    let p = f.cur.lf_offsets::<BD>(y);
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    let sb128 = seq_hdr.sb128;
    let cdef = seq_hdr.cdef;
    let mask_offset = (sby >> (sb128 == 0) as c_int) * f.sb128w;
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    if c.inloop_filters.contains(Rav1dInloopFilterType::DEBLOCK)
        && (frame_hdr.loopfilter.level_y != [0; 2])
    {
        rav1d_loopfilter_sbrow_rows::<BD>(f, p, mask_offset as usize, sby);
    }
    if cdef != 0 || !f.lf.restore_planes.is_empty() {
        // Store loop filtered pixels required by CDEF / LR.
        rav1d_copy_lpf::<BD>(c, f, p, sby);
    }
}

pub(crate) fn rav1d_filter_sbrow_cdef<BD: BitDepth>(
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
    let p = f.cur.lf_offsets::<BD>(y);
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    let prev_mask = (sby - 1 >> (seq_hdr.sb128 == 0) as c_int) * f.sb128w;
    let mask_offset = (sby >> (seq_hdr.sb128 == 0) as c_int) * f.sb128w;
    let start = sby * sbsz;
    if sby != 0 {
        let p_up = array::from_fn(|i| {
            let ss_ver = f.cur.p.layout == Rav1dPixelLayout::I420 && i != 0;
            p[i] - ((8 * p[i].pixel_stride::<BD>()) >> ss_ver as u8)
        });
        rav1d_cdef_brow::<BD>(c, tc, f, p_up, prev_mask, start - 2, start, true, sby);
    }

    let n_blks = sbsz - 2 * ((sby + 1) < f.sbh) as c_int;
    let end = cmp::min(start + n_blks, f.bh);
    rav1d_cdef_brow::<BD>(c, tc, f, p, mask_offset, start, end, false, sby);
}

pub(crate) fn rav1d_filter_sbrow_resize<BD: BitDepth>(
    _c: &Rav1dContext,
    f: &Rav1dFrameData,
    _t: &mut Rav1dTaskContext,
    sby: c_int,
) {
    let bd = BD::from_c(f.bitdepth_max);

    let sbsz = f.sb_step;
    let y = sby * sbsz * 4;
    let p = f.cur.lf_offsets::<BD>(y);
    let sr_p = f.sr_cur.p.lf_offsets::<BD>(y);
    let has_chroma = (f.cur.p.layout != Rav1dPixelLayout::I400) as usize;
    for pl in 0..1 + 2 * has_chroma {
        let ss_ver = (pl != 0 && f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
        let h_start = 8 * (sby != 0) as c_int >> ss_ver;
        let dst = sr_p[pl];
        let dst = dst - (h_start as isize * dst.pixel_stride::<BD>());
        let src = p[pl];
        let src = src - (h_start as isize * src.pixel_stride::<BD>());
        let h_end = 4 * (sbsz - 2 * ((sby + 1) < f.sbh) as c_int) >> ss_ver;
        let ss_hor = (pl != 0 && f.cur.p.layout != Rav1dPixelLayout::I444) as c_int;
        let dst_w = f.sr_cur.p.p.w + ss_hor >> ss_hor;
        let src_w = 4 * f.bw + ss_hor >> ss_hor;
        let img_h = f.cur.p.h - sbsz * 4 * sby + ss_ver >> ss_ver;

        f.dsp.mc.resize.call::<BD>(
            WithOffset::pic(dst),
            src,
            dst_w as usize,
            (cmp::min(img_h, h_end) + h_start) as usize,
            src_w as usize,
            f.resize_step[(pl != 0) as usize],
            f.resize_start[(pl != 0) as usize],
            bd,
        );
    }
}

pub(crate) fn rav1d_filter_sbrow_lr<BD: BitDepth>(
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
    let sr_p = f.sr_cur.p.lf_offsets::<BD>(y);
    rav1d_lr_sbrow::<BD>(c, f, sr_p, sby);
}

pub(crate) fn rav1d_filter_sbrow<BD: BitDepth>(
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
    if !f.lf.restore_planes.is_empty() {
        rav1d_filter_sbrow_lr::<BD>(c, f, t, sby);
    }
}

pub(crate) fn rav1d_backup_ipred_edge<BD: BitDepth>(f: &Rav1dFrameData, t: &mut Rav1dTaskContext) {
    let cur_data = &f.cur.data.as_ref().unwrap().data;

    let ts = &f.ts[t.ts];
    let sby = t.b.y >> f.sb_shift;
    let sby_off = f.sb128w * 128 * sby;
    let x_off = ts.tiling.col_start;

    let y = &cur_data[0];
    let y = y.with_offset::<BD>()
        + x_off as usize * 4
        + ((t.b.y + f.sb_step) * 4 - 1) as isize * y.pixel_stride::<BD>();
    let ipred_edge_off = (f.ipred_edge_off * 0) + (sby_off + x_off * 4) as usize;
    let n = 4 * (ts.tiling.col_end - x_off) as usize;
    BD::pixel_copy(
        &mut f.ipred_edge.mut_slice_as((ipred_edge_off.., ..n)),
        &y.slice::<BD>(n),
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
            let uv = &cur_data[pl];
            let uv = uv.with_offset::<BD>() + uv_off;
            BD::pixel_copy(
                &mut f.ipred_edge.mut_slice_as((ipred_edge_off.., ..n)),
                &uv.slice::<BD>(n),
                n,
            );
        }
    }
}

pub(crate) fn rav1d_copy_pal_block_y<BD: BitDepth>(
    t: &mut Rav1dTaskContext,
    f: &Rav1dFrameData,
    bx4: usize,
    by4: usize,
    bw4: usize,
    bh4: usize,
) {
    let pal = if t.frame_thread.pass != 0 {
        let x = t.b.x as usize;
        let y = t.b.y as usize;
        let index = ((y >> 1) + (x & 1)) * (f.b4_stride as usize >> 1) + (x >> 1) + (y & 1);
        &f.frame_thread.pal.index::<BD>(index)[0]
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

pub(crate) fn rav1d_copy_pal_block_uv<BD: BitDepth>(
    t: &mut Rav1dTaskContext,
    f: &Rav1dFrameData,
    bx4: usize,
    by4: usize,
    bw4: usize,
    bh4: usize,
) {
    let pal = if t.frame_thread.pass != 0 {
        let x = t.b.x as usize;
        let y = t.b.y as usize;
        let index = ((y >> 1) + (x & 1)) * (f.b4_stride as usize >> 1) + (x >> 1) + (y & 1);
        &*f.frame_thread.pal.index::<BD>(index)
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
pub(crate) fn rav1d_read_pal_plane<BD: BitDepth>(
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
    ) + 2;
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
    let pal = if t.frame_thread.pass != 0 {
        let pal_start = (((t.b.y >> 1) + (t.b.x & 1)) as isize * (f.b4_stride >> 1)
            + ((t.b.x >> 1) + (t.b.y & 1)) as isize) as usize;
        &mut f.frame_thread.pal.index_mut::<BD>(pal_start)[pli]
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
        let mut prev = rav1d_msac_decode_bools(&mut ts_c.msac, f.cur.p.bpc) as u16;
        pal[i] = prev.as_::<BD::Pixel>();
        i += 1;

        if i < pal.len() {
            let mut bits = f.cur.p.bpc + rav1d_msac_decode_bools(&mut ts_c.msac, 2) as u8 - 3;
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
                    bits = cmp::min(bits, 1 + ulog2((max - prev - not_pl) as u32) as u8);
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
pub(crate) fn rav1d_read_pal_uv<BD: BitDepth>(
    t: &mut Rav1dTaskContext,
    f: &Rav1dFrameData,
    ts_c: &mut Rav1dTileStateContext,
    sz_ctx: u8,
    bx4: usize,
    by4: usize,
) -> u8 {
    let pal_sz = rav1d_read_pal_plane::<BD>(t, f, ts_c, true, sz_ctx, bx4, by4);

    // V pal coding
    let pal = if t.frame_thread.pass != 0 {
        &mut f.frame_thread.pal.index_mut::<BD>(
            (((t.b.y >> 1) + (t.b.x & 1)) as isize * (f.b4_stride >> 1)
                + ((t.b.x >> 1) + (t.b.y & 1)) as isize) as usize,
        )[2]
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
        let bits = f.cur.p.bpc + rav1d_msac_decode_bools(&mut ts_c.msac, 2) as u8 - 4;
        let mut prev = rav1d_msac_decode_bools(&mut ts_c.msac, f.cur.p.bpc) as u16;
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
        pal.fill_with(|| rav1d_msac_decode_bools(&mut ts_c.msac, f.cur.p.bpc).as_::<BD::Pixel>());
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
