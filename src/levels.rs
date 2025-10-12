use std::fmt::{Display, Formatter};
use std::ops::Neg;
use std::{fmt, mem};

use bitflags::bitflags;
use strum::{EnumCount, FromRepr};
use zerocopy::{AsBytes, FromBytes, FromZeroes};

use crate::align::ArrayDefault;
use crate::enum_map::{enum_map, DefaultValue, EnumKey};
use crate::in_range::InRange;
use crate::include::dav1d::headers::Rav1dFilterMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRepr)]
pub enum ObuMetaType {
    HdrCll = 1,
    HdrMdcv = 2,
    Scalability = 3,
    ItutT35 = 4,
    Timecode = 5,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumCount, FromRepr, Default, Debug)]
pub enum TxfmSize {
    // Square
    #[default]
    S4x4 = 0,
    S8x8 = 1,
    S16x16 = 2,
    S32x32 = 3,
    S64x64 = 4,

    // Rectangular
    R4x8 = 5,
    R8x4 = 6,
    R8x16 = 7,
    R16x8 = 8,
    R16x32 = 9,
    R32x16 = 10,
    R32x64 = 11,
    R64x32 = 12,
    R4x16 = 13,
    R16x4 = 14,
    R8x32 = 15,
    R32x8 = 16,
    R16x64 = 17,
    R64x16 = 18,
}

impl TxfmSize {
    pub const NUM_SQUARE: usize = Self::S64x64 as usize + 1;
    pub const _NUM_RECT: usize = Self::COUNT;
}

impl DefaultValue for TxfmSize {
    const DEFAULT: Self = Self::S4x4;
}

impl ArrayDefault for TxfmSize {
    fn default() -> Self {
        Default::default()
    }
}

impl TxfmSize {
    pub const fn from_wh(w: usize, h: usize) -> Self {
        use TxfmSize::*;
        match (w, h) {
            // square
            (4, 4) => S4x4,
            (8, 8) => S8x8,
            (16, 16) => S16x16,
            (32, 32) => S32x32,
            (64, 64) => S64x64,
            // rect
            (4, 8) => R4x8,
            (8, 4) => R8x4,
            (8, 16) => R8x16,
            (16, 8) => R16x8,
            (16, 32) => R16x32,
            (32, 16) => R32x16,
            (32, 64) => R32x64,
            (64, 32) => R64x32,
            (4, 16) => R4x16,
            (16, 4) => R16x4,
            (8, 32) => R8x32,
            (32, 8) => R32x8,
            (16, 64) => R16x64,
            (64, 16) => R64x16,
            _ => {
                debug_assert!(false);
                DefaultValue::DEFAULT
            }
        }
    }

    pub const fn is_rect(self) -> bool {
        self as u8 >= Self::R4x8 as u8
    }
}

#[repr(u8)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, EnumCount, FromRepr)]
pub enum BlockLevel {
    #[default]
    Bl128x128 = 0,
    Bl64x64 = 1,
    Bl32x32 = 2,
    Bl16x16 = 3,
    Bl8x8 = 4,
}

impl BlockLevel {
    pub const fn decrease(self) -> Option<Self> {
        match self {
            BlockLevel::Bl8x8 => None,
            BlockLevel::Bl16x16 => Some(BlockLevel::Bl8x8),
            BlockLevel::Bl32x32 => Some(BlockLevel::Bl16x16),
            BlockLevel::Bl64x64 => Some(BlockLevel::Bl32x32),
            BlockLevel::Bl128x128 => Some(BlockLevel::Bl64x64),
        }
    }
}

pub type TxfmType = u8;
pub const N_TX_TYPES_PLUS_LL: usize = 17;
pub const WHT_WHT: TxfmType = 16;
pub const _N_TX_TYPES: usize = 16;
pub const H_FLIPADST: TxfmType = 15;
pub const V_FLIPADST: TxfmType = 14;
pub const H_ADST: TxfmType = 13;
pub const V_ADST: TxfmType = 12;
pub const H_DCT: TxfmType = 11;
pub const V_DCT: TxfmType = 10;
pub const IDTX: TxfmType = 9;
pub const FLIPADST_ADST: TxfmType = 8;
pub const ADST_FLIPADST: TxfmType = 7;
pub const FLIPADST_FLIPADST: TxfmType = 6;
pub const DCT_FLIPADST: TxfmType = 5;
pub const FLIPADST_DCT: TxfmType = 4;
pub const ADST_ADST: TxfmType = 3;
pub const DCT_ADST: TxfmType = 2;
pub const ADST_DCT: TxfmType = 1;
pub const DCT_DCT: TxfmType = 0;

#[derive(Clone, Copy, PartialEq, Eq, Debug, FromRepr)]
pub enum TxClass {
    TwoD,
    H,
    V,
}

pub type IntraPredMode = u8;
pub const FILTER_PRED: IntraPredMode = 13;
pub const Z3_PRED: IntraPredMode = 8;
pub const Z2_PRED: IntraPredMode = 7;
pub const Z1_PRED: IntraPredMode = 6;
pub const DC_128_PRED: IntraPredMode = 5;
pub const TOP_DC_PRED: IntraPredMode = 4;
pub const LEFT_DC_PRED: IntraPredMode = 3;
pub const N_IMPL_INTRA_PRED_MODES: usize = 14; // TODO(kkysen) symbolicate in struct Rav1dIntraPredDSPContext::intra_pred once deduplicated
pub const N_UV_INTRA_PRED_MODES: usize = 14;
pub const CFL_PRED: IntraPredMode = 13;
pub const N_INTRA_PRED_MODES: usize = 13;
pub const PAETH_PRED: IntraPredMode = 12;
pub const SMOOTH_H_PRED: IntraPredMode = 11;
pub const SMOOTH_V_PRED: IntraPredMode = 10;
pub const SMOOTH_PRED: IntraPredMode = 9;
pub const VERT_LEFT_PRED: IntraPredMode = 8;
pub const HOR_UP_PRED: IntraPredMode = 7;
pub const HOR_DOWN_PRED: IntraPredMode = 6;
pub const VERT_RIGHT_PRED: IntraPredMode = 5;
pub const DIAG_DOWN_RIGHT_PRED: IntraPredMode = 4;
pub const DIAG_DOWN_LEFT_PRED: IntraPredMode = 3;
pub const HOR_PRED: IntraPredMode = 2;
pub const VERT_PRED: IntraPredMode = 1;
pub const DC_PRED: IntraPredMode = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRepr, EnumCount, Default)]
pub enum InterIntraPredMode {
    #[default]
    Dc = 0,
    Vert = 1,
    Hor = 2,
    Smooth = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, FromRepr, EnumCount)]
pub enum BlockPartition {
    #[default]
    None = 0,
    H = 1,
    V = 2,
    Split = 3,
    TopSplit = 4,
    BottomSplit = 5,
    LeftSplit = 6,
    RightSplit = 7,
    H4 = 8,
    V4 = 9,
}

impl BlockPartition {
    pub const N_SUB8X8_PARTITIONS: usize = 4;
}

#[derive(Clone, Copy, PartialEq, Eq, FromRepr, EnumCount, FromZeroes, Default)]
#[repr(u8)]
pub enum BlockSize {
    #[default]
    Bs128x128 = 0,
    Bs128x64 = 1,
    Bs64x128 = 2,
    Bs64x64 = 3,
    Bs64x32 = 4,
    Bs64x16 = 5,
    Bs32x64 = 6,
    Bs32x32 = 7,
    Bs32x16 = 8,
    Bs32x8 = 9,
    Bs16x64 = 10,
    Bs16x32 = 11,
    Bs16x16 = 12,
    Bs16x8 = 13,
    Bs16x4 = 14,
    Bs8x32 = 15,
    Bs8x16 = 16,
    Bs8x8 = 17,
    Bs8x4 = 18,
    Bs4x16 = 19,
    Bs4x8 = 20,
    Bs4x4 = 21,
}

#[derive(Clone, Copy, PartialEq, Eq, EnumCount, Default, FromRepr)]
pub enum Filter2d {
    #[default]
    Regular8Tap = 0,
    RegularSmooth8Tap = 1,
    RegularSharp8Tap = 2,
    SharpRegular8Tap = 3,
    SharpSmooth8Tap = 4,
    Sharp8Tap = 5,
    SmoothRegular8Tap = 6,
    Smooth8Tap = 7,
    SmoothSharp8Tap = 8,
    Bilinear = 9,
}

impl EnumKey<{ Self::COUNT }> for Filter2d {
    const VALUES: [Self; Self::COUNT] = [
        Self::Regular8Tap,
        Self::RegularSmooth8Tap,
        Self::RegularSharp8Tap,
        Self::SharpRegular8Tap,
        Self::SharpSmooth8Tap,
        Self::Sharp8Tap,
        Self::SmoothRegular8Tap,
        Self::Smooth8Tap,
        Self::SmoothSharp8Tap,
        Self::Bilinear,
    ];

    fn as_usize(self) -> usize {
        self as usize
    }
}

impl Filter2d {
    pub const fn h(&self) -> Rav1dFilterMode {
        use Filter2d::*;
        match *self {
            Regular8Tap | RegularSmooth8Tap | RegularSharp8Tap => Rav1dFilterMode::Regular8Tap,
            SharpRegular8Tap | SharpSmooth8Tap | Sharp8Tap => Rav1dFilterMode::Sharp8Tap,
            SmoothRegular8Tap | Smooth8Tap | SmoothSharp8Tap => Rav1dFilterMode::Smooth8Tap,
            Bilinear => Rav1dFilterMode::Bilinear,
        }
    }

    pub const fn v(&self) -> Rav1dFilterMode {
        use Filter2d::*;
        match *self {
            Regular8Tap | SharpRegular8Tap | SmoothRegular8Tap => Rav1dFilterMode::Regular8Tap,
            RegularSharp8Tap | Sharp8Tap | SmoothSharp8Tap => Rav1dFilterMode::Sharp8Tap,
            RegularSmooth8Tap | SharpSmooth8Tap | Smooth8Tap => Rav1dFilterMode::Smooth8Tap,
            Bilinear => Rav1dFilterMode::Bilinear,
        }
    }

    pub const fn hv(&self) -> (Rav1dFilterMode, Rav1dFilterMode) {
        (self.h(), self.v())
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy)]
    pub struct MVJoint: u8 {
        const H = 1 << 0;
        const V = 1 << 1;
    }
}

#[expect(clippy::enum_variant_names, reason = "match dav1d naming")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterPredMode {
    NearestMv = 0,
    NearMv = 1,
    GlobalMv = 2,
    NewMv = 3,
}

impl DefaultValue for InterPredMode {
    const DEFAULT: Self = Self::NearestMv;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum DrlProximity {
    #[default]
    Nearest,
    Nearer,
    Near,
    Nearish,
}

/// Sometimes this can store a [`InterPredMode`] instead, which is smaller.
#[expect(clippy::enum_variant_names, reason = "match dav1d naming")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRepr, EnumCount, Default)]
pub enum CompInterPredMode {
    #[default]
    NearestMvNearestMv = 0,
    NearMvNearMv = 1,
    NearestMvNewMv = 2,
    NewMvNearestMv = 3,
    NearMvNewMv = 4,
    NewMvNearMv = 5,
    GlobalMvGlobalMv = 6,
    NewMvNewMv = 7,
}

impl From<InterPredMode> for CompInterPredMode {
    fn from(value: InterPredMode) -> Self {
        CompInterPredMode::from_repr(value as usize).unwrap()
    }
}

impl EnumKey<{ Self::COUNT }> for CompInterPredMode {
    const VALUES: [Self; Self::COUNT] = [
        Self::NearestMvNearestMv,
        Self::NearMvNearMv,
        Self::NearestMvNewMv,
        Self::NewMvNearestMv,
        Self::NearMvNewMv,
        Self::NewMvNearMv,
        Self::GlobalMvGlobalMv,
        Self::NewMvNewMv,
    ];

    fn as_usize(self) -> usize {
        self as usize
    }
}

impl CompInterPredMode {
    pub fn split(self) -> [InterPredMode; 2] {
        use InterPredMode::*;
        enum_map!(CompInterPredMode => [InterPredMode; 2]; match key {
            NearestMvNearestMv => [NearestMv, NearestMv],
            NearMvNearMv => [NearMv, NearMv],
            NearestMvNewMv => [NearestMv, NewMv],
            NewMvNearestMv => [NewMv, NearestMv],
            NearMvNewMv => [NearMv, NewMv],
            NewMvNearMv => [NewMv, NearMv],
            GlobalMvGlobalMv => [GlobalMv, GlobalMv],
            NewMvNewMv => [NewMv, NewMv],
        })[self]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompInterType {
    WeightedAvg = 1,
    Avg = 2,
    Seg = 3,
    Wedge = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterIntraType {
    Blend,
    Wedge,
}

/// A motion vector.
///
/// This is `align(4)` because the C version is a `union` with a `uint32_t`.
/// Being `align(4)` significantly speeds up things like `add_spatial_candidate`.
#[derive(Clone, Copy, Eq, Default, FromZeroes, FromBytes, AsBytes)]
#[repr(C, align(4))]
pub struct Mv {
    pub y: i16,
    pub x: i16,
}

impl Mv {
    pub const ZERO: Self = Self { y: 0, x: 0 };

    pub const INVALID: Self = Self {
        y: i16::MIN,
        x: i16::MIN,
    };

    pub fn is_invalid(self) -> bool {
        self == Self::INVALID
    }

    #[allow(dead_code)]
    pub fn is_valid(self) -> bool {
        !self.is_invalid()
    }

    /// Create a potentially [`UnalignedMv`] copy of this [`Mv`].
    pub fn into_unaligned(self) -> UnalignedMv {
        let Self { y, x } = self;
        UnalignedMv { y, x }
    }
}

impl Neg for Mv {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            y: -self.y,
            x: -self.x,
        }
    }
}

impl PartialEq for Mv {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        // `#[derive(PartialEq)]` compares per-field with `&&`,
        // which isn't optimized well and isn't coalesced into wider loads.
        // Comparing all of the bytes at once optimizes better with wider loads.
        // See <https://github.com/rust-lang/rust/issues/140167>.
        self.as_bytes() == other.as_bytes()
    }
}

/// A motion vector without alignment requirements.
///
/// In [`RefMvsTemporalBlock`], we have an [`UnalignedMv`] in a `packed` `struct`.
/// We cannot use a regular [`Mv`] `struct` there because Rust
/// does not permit an aligned `struct` to be inside a packed `struct`.
/// Instead, [`RefMvsTemporalBlock`] uses [`UnalignedMv`], and we provide conversion methods.
///
/// This must be [`Copy`] because it will be in a `packed` `struct`,
/// where we cannot take a reference.
///
/// [`RefMvsTemporalBlock`]: crate::refmvs::RefMvsTemporalBlock
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(C)]
pub struct UnalignedMv {
    pub y: i16,
    pub x: i16,
}

impl UnalignedMv {
    /// Create an aligned [`Mv`] from this [`UnalignedMv`].
    pub fn into_aligned(self) -> Mv {
        let Self { y, x } = self;
        Mv { y, x }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRepr, Default)]
pub enum MotionMode {
    #[default]
    Translation = 0,
    Obmc = 1,
    Warp = 2,
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct Av1BlockIntra {
    pub y_mode: u8,
    pub uv_mode: u8,
    pub tx: TxfmSize,
    pub pal_sz: [u8; 2],
    pub y_angle: i8,
    pub uv_angle: i8,
    pub cfl_alpha: [i8; 2],
}

/// Really an [`InterIntraPredMode`],
/// but stored as a `u8` so there are no invalid bits.
#[derive(Clone, Copy, PartialEq, Eq, FromZeroes, FromBytes, AsBytes)]
#[repr(transparent)]
pub struct MaskedInterIntraPredMode(u8);

impl MaskedInterIntraPredMode {
    pub const fn get(self) -> InterIntraPredMode {
        // Should just be an `& 3`.
        match InterIntraPredMode::from_repr(self.0 as usize % InterIntraPredMode::COUNT) {
            Some(it) => it,
            None => unreachable!(),
        }
    }

    pub const fn new(this: InterIntraPredMode) -> Self {
        Self(this as u8)
    }
}

impl Default for MaskedInterIntraPredMode {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl From<InterIntraPredMode> for MaskedInterIntraPredMode {
    fn from(value: InterIntraPredMode) -> Self {
        MaskedInterIntraPredMode::new(value)
    }
}

impl From<MaskedInterIntraPredMode> for InterIntraPredMode {
    fn from(value: MaskedInterIntraPredMode) -> Self {
        value.get()
    }
}

pub type WedgeIdx = InRange<u8, 0, 15>;

#[derive(Clone, Default, FromZeroes, FromBytes, AsBytes)]
#[repr(C)]
pub struct Av1BlockInter1d {
    pub mv: [Mv; 2],
    pub wedge_idx: WedgeIdx,

    /// Stored as a [`u8`] since [`bool`] is not [`FromBytes`].
    pub mask_sign: u8,

    pub interintra_mode: MaskedInterIntraPredMode,

    /// For `impl `[`AsBytes`].
    pub _padding: u8,
}

impl Av1BlockInter1d {
    pub fn mask_sign(&self) -> bool {
        self.mask_sign != 0
    }
}

#[derive(Clone, FromZeroes, FromBytes, AsBytes)]
#[repr(C)]
pub struct Av1BlockInter2d {
    pub mv2d: Mv,

    /// These are `i14`s (except for an [`i16::MIN`] stored as a discriminant).
    /// Not sure how we could stably use that niche, though.
    pub matrix: [i16; 4],
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct Av1BlockInterNd {
    /// Make [`Av1BlockInter1d`] the field instead of [`Av1BlockInter2d`]
    /// simply because it is used much more often, so it's more convenient.
    ///
    /// `[`[`u8`]`; 12]` is not used because it has a lower alignment.
    /// [`Av1BlockInter1d`] and [`Av1BlockInter2d`] both have the same size and alignment.
    pub one_d: Av1BlockInter1d,
}

impl Av1BlockInterNd {
    pub fn two_d(&self) -> &Av1BlockInter2d {
        // These asserts ensure this is a no-op.
        const _: () =
            assert!(mem::size_of::<Av1BlockInter1d>() == mem::size_of::<Av1BlockInter2d>());
        const _: () =
            assert!(mem::align_of::<Av1BlockInter1d>() == mem::align_of::<Av1BlockInter2d>());
        FromBytes::ref_from(AsBytes::as_bytes(&self.one_d)).unwrap()
    }
}

impl From<Av1BlockInter1d> for Av1BlockInterNd {
    fn from(one_d: Av1BlockInter1d) -> Self {
        Self { one_d }
    }
}

impl From<Av1BlockInter2d> for Av1BlockInterNd {
    fn from(two_d: Av1BlockInter2d) -> Self {
        let one_d = <Av1BlockInter1d as FromBytes>::ref_from(AsBytes::as_bytes(&two_d))
            .unwrap()
            .clone(); // Cheap 12-byte clone.
        Self { one_d }
    }
}

pub type Av1BlockInterRefIndex = InRange<i8, -1, 6>;

#[derive(Clone, Default)]
#[repr(C)]
pub struct Av1BlockInter {
    pub nd: Av1BlockInterNd,
    pub comp_type: Option<CompInterType>,
    pub inter_mode: CompInterPredMode,
    pub motion_mode: MotionMode,
    pub drl_idx: DrlProximity,
    pub r#ref: [Av1BlockInterRefIndex; 2],
    pub max_ytx: TxfmSize,
    pub filter2d: Filter2d,
    pub interintra_type: Option<InterIntraType>,
    pub tx_split0: u8,
    pub tx_split1: u16,
}

// pub enum Av1BlockIntraInter {
//     Intra(Av1BlockIntra),
//     Inter(Av1BlockInter),
// }

// impl Av1BlockIntraInter {
//     pub fn filter2d(&self) -> Filter2d {
//         // More optimal code if we use a default instead of just panicking.
//         match self {
//             Self::Inter(inter) => Some(inter),
//             _ => None,
//         }
//         .map(|inter| inter.filter2d)
//         .unwrap_or_default()
//     }
// }

// impl Default for Av1BlockIntraInter {
//     fn default() -> Self {
//         Self::Intra(Default::default())
//     }
// }

/// Within range `0..`[`SegmentId::COUNT`].
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct SegmentId {
    id: InRange<u8, 0, { Self::COUNT as i128 - 1 }>,
}

impl SegmentId {
    pub const COUNT: usize = 8;

    pub fn new(id: u8) -> Option<Self> {
        Some(Self {
            id: InRange::new(id)?,
        })
    }

    pub fn get(&self) -> usize {
        self.id.get() as usize
    }

    pub fn min() -> Self {
        Self::new(0).unwrap()
    }

    pub fn max() -> Self {
        Self::new(Self::COUNT as u8 - 1).unwrap()
    }
}

impl Display for SegmentId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[derive(Default)]
#[repr(C)]
pub struct Av1BlockOld {
    pub bl: BlockLevel,
    pub bs: BlockSize,
    pub bp: BlockPartition,
    pub seg_id: SegmentId,
    pub skip_mode: bool,
    pub skip: bool,
    pub uvtx: TxfmSize,
    pub ii: Av1BlockIntraInter,
}

#[derive(Default)]
#[repr(transparent)]
struct U8Bits {
    bits: u8,
}

impl U8Bits {
    const fn mask(num_bits: usize, start_bit_index: usize) -> u8 {
        ((1 << num_bits) - 1) << start_bit_index
    }

    pub const fn get_bits<const NUM_BITS: usize, const START_BIT_INDEX: usize>(&self) -> u8 {
        let mask = Self::mask(NUM_BITS, START_BIT_INDEX);
        self.bits & mask
    }

    pub const fn set_bits<const NUM_BITS: usize, const START_BIT_INDEX: usize>(
        &mut self,
        value: u8,
    ) {
        let mask = Self::mask(NUM_BITS, START_BIT_INDEX);
        self.bits = (self.bits & !mask) | ((value << START_BIT_INDEX) & mask)
    }
}

/// `u5 = max_ytx: TxfmSize` (19 variants)
/// `u3 = comp_type: Option<CompInterType>` (4 variants + Option)
#[derive(Default)]
#[repr(transparent)]
struct MaxYtxCompType(U8Bits);

impl MaxYtxCompType {
    pub const fn max_ytx(&self) -> TxfmSize {
        todo!()
    }

    pub const fn set_max_ytx(&mut self, max_ytx: TxfmSize) {
        todo!()
    }

    pub const fn comp_type(&self) -> Option<CompInterType> {
        todo!()
    }
}

/// `u3 = seg_id: SegId` (8 variants)
/// `u5 = uvtx: TxfmSize` (19 variants)
#[derive(Default)]
#[repr(transparent)]
struct SegIdUvtx(U8Bits);

impl SegIdUvtx {
    pub const fn seg_id(&self) -> SegmentId {
        todo!()
    }

    pub const fn set_seg_id(&mut self, seg_id: SegmentId) {
        todo!()
    }

    pub const fn uvtx(&self) -> TxfmSize {
        todo!()
    }

    pub const fn set_uvtx(&mut self, uvtx: TxfmSize) {
        todo!()
    }
}

/// `u3 = bl: BlockLevel` (5 variants)
/// `u5 = bs: BlockSize` (22 variants)
#[derive(Default)]
#[repr(transparent)]
pub struct BlockLevelBlockSize(U8Bits);

impl BlockLevelBlockSize {
    pub const fn bl(&self) -> BlockLevel {
        let bl = self.0.get_bits::<3, 0>();
        let bl = BlockLevel::from_repr(bl);
        // SAFETY: Bits can only be set in `Self::set_bl` from a `BlockLevel`.
        unsafe { bl.unwrap_unchecked() }
    }

    pub const fn set_bl(&mut self, bl: BlockLevel) {
        self.0.set_bits::<3, 0>(bl as _);
    }

    pub const fn bs(&self) -> BlockSize {
        let bs = self.0.get_bits::<5, { 0 + 3 }>();
        let bs: Option<BlockSize> = BlockSize::from_repr(bs);
        // SAFETY: Bits can only be set in `Self::set_bs` from a `BlockLevel`.
        unsafe { bs.unwrap_unchecked() }
    }

    pub const fn set_bs(&mut self, bs: BlockSize) {
        self.0.set_bits::<5, { 0 + 3 }>(bs as _);
    }
}

/// `u4 = bp: BlockPartition` (10 variants)
/// `u4 = filter2d: Filter2d` (10 variants)
#[derive(Default)]
#[repr(transparent)]
struct BlockPartitionFilter2d(U8Bits);

impl BlockPartitionFilter2d {
    pub const fn bp(&self) -> BlockPartition {
        todo!()
    }

    pub const fn set_bp(&mut self, bp: BlockPartition) {
        todo!()
    }

    pub const fn filter2d(&self) -> Filter2d {
        todo!()
    }

    pub const fn set_filter2d(&mut self, filter2d: Filter2d) {
        todo!()
    }
}

/// `u3 = inter_mode: CompInterPredMode` (8 variants)
/// `u2 = motion_mode: MotionMode` (3 variants)
/// `u2 = drl_idx: DrlProximity` (4 variants)
/// `u1 = mask_sign: bool` (2 variants)
#[derive(Default)]
#[repr(transparent)]
struct InterModeMotionModeDrlIdxMaskSign(U8Bits);

impl InterModeMotionModeDrlIdxMaskSign {
    pub const fn inter_mode(&self) -> CompInterPredMode {
        todo!()
    }

    pub const fn motion_mode(&self) -> MotionMode {
        todo!()
    }

    pub const fn drl_idx(&self) -> DrlProximity {
        todo!()
    }

    pub const fn mask_sign(&self) -> bool {
        todo!()
    }
}

/// `u1 = skip: bool` (2 variants)
/// `u1 = skip_mode: bool` (2 variants)
/// `[u3; 2] = r#ref: [Av1BlockInterRefIndex; 2]` ([8 variants; 2])
#[derive(Default)]
#[repr(transparent)]
struct SkipSkipModeRef(U8Bits);

impl SkipSkipModeRef {
    pub const fn skip(&self) -> bool {
        todo!()
    }

    pub const fn set_skip(&mut self, skip: bool) {
        todo!()
    }

    pub const fn skip_mode(&self) -> bool {
        todo!()
    }

    pub const fn set_skip_mode(&mut self, skip_mode: bool) {
        todo!()
    }

    pub const fn ref0(&self) -> Av1BlockInterRefIndex {
        todo!()
    }

    pub const fn ref1(&self) -> Av1BlockInterRefIndex {
        todo!()
    }
}

/// `u2 = interintra_type: Option<InterIntraType>` (2 variants + Option)
/// `u2 = interintra_mode: InterIntraPredMode` (4 variants)
/// `u4 = wedge_idx: WedgeIdx` (16 variants)
#[derive(Default)]
#[repr(transparent)]
struct InterIntraTypeWedgeIdxInterIntraMode(U8Bits);

impl InterIntraTypeWedgeIdxInterIntraMode {
    pub const fn interintra_type(&self) -> Option<InterIntraType> {
        todo!()
    }

    pub const fn interintra_mode(&self) -> InterIntraPredMode {
        todo!()
    }

    pub const fn wedge_idx(&self) -> WedgeIdx {
        todo!()
    }
}

#[derive(Default, FromZeroes, FromBytes, AsBytes)]
#[repr(C)] // For known layout, not C.
struct Av1BlockInterNdBytes {
    /// Bytes 0..4
    ///
    /// These are `i14`s (except for an [`i16::MIN`] stored as a discriminant).
    /// Not sure how we could stably use that niche, though.
    matrix3: i16,
    matrix4: i16,

    /// Bytes 4..12
    ///
    /// `mv1d[0]` is `mv2d`.
    /// `mv1d` is `Av1BlockIntra`, except for `tx`, which reuses `uvtx`.
    mv1d: [Mv; 2],
}

#[derive(Default)]
#[repr(C)]
pub enum Av1BlockIntraInter {
    #[default]
    Inter,
    Intra,
}

#[derive(Default)]
#[repr(C)] // For known layout, not C.
pub struct Av1Block {
    /// Bytes 0..12
    inter_nd: Av1BlockInterNdBytes,

    /// Byte 13
    ///
    /// [`Self::inter_max_ytx`] is reused as [`Self::intra_tx`],
    /// and the rest of [`Self::intra`] is from [`Self::inter_nd`]'s last 8 bytes ([`Av1BlockInterNdBytes::mv1d`]),
    /// so we want to keep [`Self::intra_tx`] adjacent to [`Self::intra`].
    max_ytx_comp_type: MaxYtxCompType,

    /// Bytes 13..16
    ///
    /// Bytes 12..16 aligned for a u32 load of both of them.
    pub inter_tx_split0: u8,
    pub inter_tx_split1: u16,

    /// Bytes 16..23
    /// 
    /// We have extra space for [`Self::ii`], but it could also be folded into a niche if needed.
    pub ii: Av1BlockIntraInter,
    seg_id_uvtx: SegIdUvtx,
    bl_bs: BlockLevelBlockSize,
    bp_filter2d: BlockPartitionFilter2d,
    inter_mode_motion_mode_drl_idx_mask_sign: InterModeMotionModeDrlIdxMaskSign,
    skip_skip_mode_ref: SkipSkipModeRef,
    interintra_type_interintra_mode_wedge_idx: InterIntraTypeWedgeIdxInterIntraMode,

    /// Explicit padding.
    _byte_23: u8,
}

/// [`Av1BlockIntra`] minus [`Av1BlockIntra::tx`], which reuses [`Av1BlockBytes::inter_max_ytx`].
/// The fields are rearranged somewhat for access patterns.
#[derive(FromZeroes, FromBytes, AsBytes)]
#[repr(C)] // For known layout, not C.
pub struct Av1BlockIntraMinuxTx {
    pub y_mode: u8,
    pub uv_mode: u8,
    pub y_angle: i8,
    pub uv_angle: i8,
    pub pal_sz: [u8; 2],
    pub cfl_alpha: [i8; 2],
}

impl Av1Block {
    pub const fn bl(&self) -> BlockLevel {
        self.bl_bs.bl()
    }

    pub const fn set_bl(&mut self, bl: BlockLevel) {
        self.bl_bs.set_bl(bl);
    }

    pub const fn bs(&self) -> BlockSize {
        self.bl_bs.bs()
    }

    pub const fn set_bs(&mut self, bs: BlockSize) {
        self.bl_bs.set_bs(bs);
    }

    pub const fn bp(&self) -> BlockPartition {
        self.bp_filter2d.bp()
    }

    pub const fn set_bp(&mut self, bp: BlockPartition) {
        self.bp_filter2d.set_bp(bp);
    }

    pub const fn seg_id(&self) -> SegmentId {
        self.seg_id_uvtx.seg_id()
    }

    pub const fn set_seg_id(&mut self, seg_id: SegmentId) {
        self.seg_id_uvtx.set_seg_id(seg_id);
    }

    pub const fn skip_mode(&self) -> bool {
        self.skip_skip_mode_ref.skip_mode()
    }

    pub const fn set_skip_mode(&mut self, skip_mode: bool) {
        self.skip_skip_mode_ref.set_skip_mode(skip_mode);
    }

    pub const fn skip(&self) -> bool {
        self.skip_skip_mode_ref.skip()
    }

    pub const fn set_skip(&mut self, skip: bool) {
        self.skip_skip_mode_ref.set_skip(skip);
    }

    pub const fn uvtx(&self) -> TxfmSize {
        self.seg_id_uvtx.uvtx()
    }

    pub const fn set_uvtx(&mut self, uvtx: TxfmSize) {
        self.seg_id_uvtx.set_uvtx(uvtx);
    }

    pub fn intra(&self) -> &Av1BlockIntraMinuxTx {
        FromBytes::ref_from(AsBytes::as_bytes(&self.inter_nd.mv1d)).unwrap()
    }

    pub fn intra_mut(&mut self) -> &mut Av1BlockIntraMinuxTx {
        FromBytes::mut_from(AsBytes::as_bytes_mut(&mut self.inter_nd.mv1d)).unwrap()
    }

    pub const fn intra_tx(&self) -> TxfmSize {
        // Reuse `inter_max_ytx` since it's `inter`.
        // It will also remove a select between `intra_tx` and `inter_max_ytx`.
        self.inter_max_ytx()
    }

    pub const fn set_intra_tx(&mut self, tx: TxfmSize) {
        self.set_inter_max_ytx(tx)
    }

    pub const fn inter_1d_mv(&self) -> &[Mv; 2] {
        &self.inter_nd.mv1d
    }

    pub const fn inter_1d_mv_mut(&mut self) -> &mut [Mv; 2] {
        &mut self.inter_nd.mv1d
    }

    pub const fn inter_1d_wedge_idx(&self) -> WedgeIdx {
        self.interintra_type_interintra_mode_wedge_idx.wedge_idx()
    }

    pub const fn inter_1d_mask_sign(&self) -> bool {
        self.inter_mode_motion_mode_drl_idx_mask_sign.mask_sign()
    }

    pub const fn inter_1d_interintra_mode(&self) -> InterIntraPredMode {
        self.interintra_type_interintra_mode_wedge_idx.interintra_mode()
    }

    pub const fn inter_2d_mv(&self) -> &Mv {
        &self.inter_nd.mv1d[0]
    }

    pub fn inter_2d_matrix(&self) -> &[i16; 4] {
        FromBytes::ref_from(&AsBytes::as_bytes(&self.inter_nd)[0..8]).unwrap()
    }

    pub const fn inter_comp_type(&self) -> Option<CompInterType> {
        self.max_ytx_comp_type.comp_type()
    }

    // TODO does `inter_inter` naming make sense, or does one `inter` suffice.
    pub const fn inter_inter_mode(&self) -> CompInterPredMode {
        self.inter_mode_motion_mode_drl_idx_mask_sign.inter_mode()
    }

    pub const fn inter_motion_mode(&self) -> MotionMode {
        self.inter_mode_motion_mode_drl_idx_mask_sign.motion_mode()
    }

    pub const fn inter_drl_idx(&self) -> DrlProximity {
        self.inter_mode_motion_mode_drl_idx_mask_sign.drl_idx()
    }

    pub const fn inter_ref0(&self) -> Av1BlockInterRefIndex {
        self.skip_skip_mode_ref.ref0()
    }

    pub const fn inter_ref1(&self) -> Av1BlockInterRefIndex {
        self.skip_skip_mode_ref.ref1()
    }

    pub const fn inter_ref(&self) -> [Av1BlockInterRefIndex; 2] {
        [self.inter_ref0(), self.inter_ref1()]
    }

    pub const fn inter_max_ytx(&self) -> TxfmSize {
        self.max_ytx_comp_type.max_ytx()
    }

    pub const fn set_inter_max_ytx(&mut self, max_ytx: TxfmSize) {
        self.max_ytx_comp_type.set_max_ytx(max_ytx);
    }

    pub const fn inter_filter2d(&self) -> Filter2d {
        self.bp_filter2d.filter2d()
    }

    pub const fn set_inter_filter2d(&mut self, filter2d: Filter2d) {
        self.bp_filter2d.set_filter2d(filter2d);
    }

    // TODO does `inter_inter` naming make sense, or does one `inter` suffice.
    pub const fn inter_interintra_type(&self) -> Option<InterIntraType> {
        self.interintra_type_interintra_mode_wedge_idx.interintra_type()
    }
}
