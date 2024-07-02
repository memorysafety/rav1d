use crate::src::align::ArrayDefault;
use crate::src::enum_map::EnumKey;
use crate::src::levels::SegmentId;
use crate::src::relaxed_atomic::RelaxedAtomic;
use parking_lot::Mutex;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::BitAnd;
use std::ops::Deref;
use std::ops::Sub;
use std::sync::Arc;
use strum::EnumCount;
use strum::FromRepr;

/// This is so we can store both `*mut D` and `*mut R`
/// for maintaining `dav1d` ABI compatibility,
/// where `D` is the `Dav1d*` type and `R` is the `Rav1d` type.
pub struct DRav1d<R, D> {
    pub rav1d: R,
    pub dav1d: D,
}

impl<R, D> DRav1d<R, D>
where
    R: Clone + Into<D>,
{
    pub fn from_rav1d(rav1d: R) -> Self {
        let dav1d = rav1d.clone().into();
        Self { rav1d, dav1d }
    }
}

/// Since the `D`/`Dav1d*` type is only used externally by C,
/// it's reasonable to `.deref()`
/// to the `R`/`Rav1d*` type used everywhere internally.
impl<R, D> Deref for DRav1d<R, D> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.rav1d
    }
}

impl<R, D> Default for DRav1d<R, D>
where
    R: Default,
    D: Default,
{
    fn default() -> Self {
        Self {
            rav1d: Default::default(),
            dav1d: Default::default(),
        }
    }
}

// Constants from Section 3. "Symbols and abbreviated terms"
pub const DAV1D_MAX_CDEF_STRENGTHS: usize = 8;
pub const DAV1D_MAX_OPERATING_POINTS: usize = 32;
pub const DAV1D_MAX_TILE_COLS: usize = 64;
pub const DAV1D_MAX_TILE_ROWS: usize = 64;
pub const DAV1D_MAX_SEGMENTS: u8 = SegmentId::COUNT as _;
pub const DAV1D_NUM_REF_FRAMES: usize = 8;
pub const DAV1D_PRIMARY_REF_NONE: u8 = 7;
pub const DAV1D_REFS_PER_FRAME: usize = 7;
pub const DAV1D_TOTAL_REFS_PER_FRAME: usize = DAV1D_REFS_PER_FRAME + 1;

pub(crate) const RAV1D_MAX_CDEF_STRENGTHS: usize = DAV1D_MAX_CDEF_STRENGTHS;
pub(crate) const RAV1D_MAX_OPERATING_POINTS: usize = DAV1D_MAX_OPERATING_POINTS;
pub(crate) const RAV1D_MAX_TILE_COLS: usize = DAV1D_MAX_TILE_COLS;
pub(crate) const RAV1D_MAX_TILE_ROWS: usize = DAV1D_MAX_TILE_ROWS;
pub(crate) const _RAV1D_NUM_REF_FRAMES: usize = DAV1D_NUM_REF_FRAMES;
pub(crate) const RAV1D_PRIMARY_REF_NONE: u8 = DAV1D_PRIMARY_REF_NONE;
pub(crate) const RAV1D_REFS_PER_FRAME: usize = DAV1D_REFS_PER_FRAME;
pub(crate) const RAV1D_TOTAL_REFS_PER_FRAME: usize = DAV1D_TOTAL_REFS_PER_FRAME;

pub type Dav1dObuType = c_uint;
pub const DAV1D_OBU_PADDING: Dav1dObuType = Rav1dObuType::Padding as Dav1dObuType;
pub const DAV1D_OBU_REDUNDANT_FRAME_HDR: Dav1dObuType =
    Rav1dObuType::RedundantFrameHdr as Dav1dObuType;
pub const DAV1D_OBU_FRAME: Dav1dObuType = Rav1dObuType::Frame as Dav1dObuType;
pub const DAV1D_OBU_METADATA: Dav1dObuType = Rav1dObuType::Metadata as Dav1dObuType;
pub const DAV1D_OBU_TILE_GRP: Dav1dObuType = Rav1dObuType::TileGrp as Dav1dObuType;
pub const DAV1D_OBU_FRAME_HDR: Dav1dObuType = Rav1dObuType::FrameHdr as Dav1dObuType;
pub const DAV1D_OBU_TD: Dav1dObuType = Rav1dObuType::Td as Dav1dObuType;
pub const DAV1D_OBU_SEQ_HDR: Dav1dObuType = Rav1dObuType::SeqHdr as Dav1dObuType;

#[derive(Clone, Copy, PartialEq, Eq, FromRepr)]
pub enum Rav1dObuType {
    SeqHdr = 1,
    Td = 2,
    FrameHdr = 3,
    TileGrp = 4,
    Metadata = 5,
    Frame = 6,
    RedundantFrameHdr = 7,
    Padding = 15,
}

pub type Dav1dTxfmMode = c_uint;
pub const DAV1D_N_TX_MODES: usize = Rav1dTxfmMode::COUNT;
pub const DAV1D_TX_4X4_ONLY: Dav1dTxfmMode = Rav1dTxfmMode::Only4x4 as Dav1dTxfmMode;
pub const DAV1D_TX_LARGEST: Dav1dTxfmMode = Rav1dTxfmMode::Largest as Dav1dTxfmMode;
pub const DAV1D_TX_SWITCHABLE: Dav1dTxfmMode = Rav1dTxfmMode::Switchable as Dav1dTxfmMode;

#[derive(Clone, Copy, PartialEq, Eq, FromRepr, EnumCount, Default)]
pub enum Rav1dTxfmMode {
    #[default] // Not really a real default.
    Only4x4 = 0,
    Largest = 1,
    Switchable = 2,
}

impl From<Rav1dTxfmMode> for Dav1dTxfmMode {
    fn from(value: Rav1dTxfmMode) -> Self {
        value as Dav1dTxfmMode
    }
}

impl TryFrom<Dav1dTxfmMode> for Rav1dTxfmMode {
    type Error = ();

    fn try_from(value: Dav1dTxfmMode) -> Result<Self, Self::Error> {
        Self::from_repr(value as usize).ok_or(())
    }
}

pub type Dav1dFilterMode = u8;
pub const DAV1D_N_SWITCHABLE_FILTERS: usize = Rav1dFilterMode::N_SWITCHABLE_FILTERS as usize;
pub const DAV1D_N_FILTERS: usize = Rav1dFilterMode::N_FILTERS as usize;
pub const DAV1D_FILTER_SWITCHABLE: Dav1dFilterMode = Rav1dFilterMode::Switchable as Dav1dFilterMode;
pub const DAV1D_FILTER_BILINEAR: Dav1dFilterMode = Rav1dFilterMode::Bilinear as Dav1dFilterMode;
pub const DAV1D_FILTER_8TAP_SHARP: Dav1dFilterMode = Rav1dFilterMode::Sharp8Tap as Dav1dFilterMode;
pub const DAV1D_FILTER_8TAP_SMOOTH: Dav1dFilterMode =
    Rav1dFilterMode::Smooth8Tap as Dav1dFilterMode;
pub const DAV1D_FILTER_8TAP_REGULAR: Dav1dFilterMode =
    Rav1dFilterMode::Regular8Tap as Dav1dFilterMode;

#[derive(Clone, Copy, PartialEq, Eq, FromRepr, Default, Debug)]
pub enum Rav1dFilterMode {
    #[default] // Not really a real default.
    Regular8Tap = 0,
    Smooth8Tap = 1,
    Sharp8Tap = 2,
    Bilinear = 3,
    Switchable = 4,
}

impl ArrayDefault for Rav1dFilterMode {
    fn default() -> Self {
        Default::default()
    }
}

impl Rav1dFilterMode {
    pub const N_FILTERS: usize = 4;
    pub const N_SWITCHABLE_FILTERS: Self = Self::Bilinear;
}

impl From<Rav1dFilterMode> for Dav1dFilterMode {
    fn from(value: Rav1dFilterMode) -> Self {
        value as Dav1dFilterMode
    }
}

impl TryFrom<Dav1dFilterMode> for Rav1dFilterMode {
    type Error = ();

    fn try_from(value: Dav1dFilterMode) -> Result<Self, Self::Error> {
        Self::from_repr(value as usize).ok_or(())
    }
}

pub type Dav1dAdaptiveBoolean = c_uint;
pub const DAV1D_OFF: Dav1dAdaptiveBoolean = Rav1dAdaptiveBoolean::Off as Dav1dAdaptiveBoolean;
pub const DAV1D_ON: Dav1dAdaptiveBoolean = Rav1dAdaptiveBoolean::On as Dav1dAdaptiveBoolean;
pub const DAV1D_ADAPTIVE: Dav1dAdaptiveBoolean =
    Rav1dAdaptiveBoolean::Adaptive as Dav1dAdaptiveBoolean;

#[derive(Clone, Copy, PartialEq, Eq, FromRepr)]
pub enum Rav1dAdaptiveBoolean {
    Off = 0,
    On = 1,
    Adaptive = 2,
}

impl From<bool> for Rav1dAdaptiveBoolean {
    fn from(value: bool) -> Self {
        match value {
            true => Self::On,
            false => Self::Off,
        }
    }
}

impl From<Rav1dAdaptiveBoolean> for Dav1dAdaptiveBoolean {
    fn from(value: Rav1dAdaptiveBoolean) -> Self {
        value as Dav1dAdaptiveBoolean
    }
}

impl TryFrom<Dav1dAdaptiveBoolean> for Rav1dAdaptiveBoolean {
    type Error = ();

    fn try_from(value: Dav1dAdaptiveBoolean) -> Result<Self, Self::Error> {
        Self::from_repr(value as usize).ok_or(())
    }
}

pub type Dav1dRestorationType = u8;
pub const DAV1D_RESTORATION_NONE: Dav1dRestorationType = Rav1dRestorationType::None.to_repr();
pub const DAV1D_RESTORATION_SWITCHABLE: Dav1dRestorationType =
    Rav1dRestorationType::Switchable.to_repr();
pub const DAV1D_RESTORATION_WIENER: Dav1dRestorationType = Rav1dRestorationType::Wiener.to_repr();
pub const DAV1D_RESTORATION_SGRPROJ: Dav1dRestorationType =
    Rav1dRestorationType::SgrProj(SgrIdx::I0).to_repr();

#[derive(Clone, Copy, PartialEq, Eq, FromRepr)]
pub enum SgrIdx {
    I0 = 0,
    I1 = 1,
    I2 = 2,
    I3 = 3,
    I4 = 4,
    I5 = 5,
    I6 = 6,
    I7 = 7,
    I8 = 8,
    I9 = 9,
    I10 = 10,
    I11 = 11,
    I12 = 12,
    I13 = 13,
    I14 = 14,
    I15 = 15,
}

impl Display for SgrIdx {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl Debug for SgrIdx {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Rav1dRestorationType {
    #[default]
    None,
    Switchable,
    Wiener,
    SgrProj(SgrIdx),
}

impl Rav1dRestorationType {
    pub const fn to_repr(&self) -> Dav1dRestorationType {
        match *self {
            Self::None => 0,
            Self::Switchable => 1,
            Self::Wiener => 2,
            Self::SgrProj(idx) => 3 + idx as Dav1dRestorationType,
        }
    }

    pub const fn from_repr(repr: usize) -> Option<Self> {
        Some(match repr {
            0 => Self::None,
            1 => Self::Switchable,
            2 => Self::Wiener,
            3 => Self::SgrProj(SgrIdx::I0),
            _ => return None,
        })
    }
}

pub type Dav1dWarpedMotionType = c_uint;
pub const DAV1D_WM_TYPE_IDENTITY: Dav1dWarpedMotionType =
    Rav1dWarpedMotionType::Identity as Dav1dWarpedMotionType;
pub const DAV1D_WM_TYPE_TRANSLATION: Dav1dWarpedMotionType =
    Rav1dWarpedMotionType::Translation as Dav1dWarpedMotionType;
pub const DAV1D_WM_TYPE_ROT_ZOOM: Dav1dWarpedMotionType =
    Rav1dWarpedMotionType::RotZoom as Dav1dWarpedMotionType;
pub const DAV1D_WM_TYPE_AFFINE: Dav1dWarpedMotionType =
    Rav1dWarpedMotionType::Affine as Dav1dWarpedMotionType;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromRepr)]
pub enum Rav1dWarpedMotionType {
    Identity = 0,
    Translation = 1,
    RotZoom = 2,
    Affine = 3,
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dWarpedMotionParams {
    pub r#type: Dav1dWarpedMotionType,
    pub matrix: [i32; 6],
    pub abcd: [i16; 4],
}

impl Dav1dWarpedMotionParams {
    pub const fn alpha(&self) -> i16 {
        self.abcd[0]
    }

    pub const fn beta(&self) -> i16 {
        self.abcd[1]
    }

    pub const fn gamma(&self) -> i16 {
        self.abcd[2]
    }

    pub const fn delta(&self) -> i16 {
        self.abcd[3]
    }
}

#[derive(Clone)]
pub struct Rav1dWarpedMotionParams {
    pub r#type: Rav1dWarpedMotionType,
    pub matrix: [i32; 6],
    pub abcd: RelaxedAtomic<[i16; 4]>,
}

impl Rav1dWarpedMotionParams {
    pub fn alpha(&self) -> i16 {
        self.abcd.get()[0]
    }

    pub fn beta(&self) -> i16 {
        self.abcd.get()[1]
    }

    pub fn gamma(&self) -> i16 {
        self.abcd.get()[2]
    }

    pub fn delta(&self) -> i16 {
        self.abcd.get()[3]
    }
}

impl TryFrom<Dav1dWarpedMotionParams> for Rav1dWarpedMotionParams {
    type Error = ();

    fn try_from(value: Dav1dWarpedMotionParams) -> Result<Self, Self::Error> {
        let Dav1dWarpedMotionParams {
            r#type,
            matrix,
            abcd,
        } = value;
        Ok(Self {
            r#type: Rav1dWarpedMotionType::from_repr(r#type as usize).ok_or(())?,
            matrix,
            abcd: abcd.into(),
        })
    }
}

impl From<Rav1dWarpedMotionParams> for Dav1dWarpedMotionParams {
    fn from(value: Rav1dWarpedMotionParams) -> Self {
        let Rav1dWarpedMotionParams {
            r#type,
            matrix,
            abcd,
        } = value;
        Self {
            r#type: r#type as Dav1dWarpedMotionType,
            matrix,
            abcd: abcd.get(),
        }
    }
}

// TODO(kkysen) Eventually the [`impl Default`] might not be needed.
#[derive(Clone, Copy, PartialEq, Eq, EnumCount, FromRepr, Default)]
pub enum Rav1dPixelLayout {
    #[default]
    I400 = 0,
    I420 = 1,
    I422 = 2,
    I444 = 3,
}

impl Rav1dPixelLayout {
    pub const fn into_rav1d(self) -> Dav1dPixelLayout {
        self as Dav1dPixelLayout
    }
}

impl Sub for Rav1dPixelLayout {
    type Output = Rav1dPixelLayout;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_repr((self as u8 - rhs as u8) as usize).unwrap()
    }
}

pub type Dav1dPixelLayout = c_uint;
pub const DAV1D_PIXEL_LAYOUT_I400: Dav1dPixelLayout = Rav1dPixelLayout::I400.into_rav1d();
pub const DAV1D_PIXEL_LAYOUT_I420: Dav1dPixelLayout = Rav1dPixelLayout::I420.into_rav1d();
pub const DAV1D_PIXEL_LAYOUT_I422: Dav1dPixelLayout = Rav1dPixelLayout::I422.into_rav1d();
pub const DAV1D_PIXEL_LAYOUT_I444: Dav1dPixelLayout = Rav1dPixelLayout::I444.into_rav1d();

impl From<Rav1dPixelLayout> for Dav1dPixelLayout {
    fn from(value: Rav1dPixelLayout) -> Self {
        value.into_rav1d()
    }
}

impl TryFrom<Dav1dPixelLayout> for Rav1dPixelLayout {
    type Error = ();

    fn try_from(value: Dav1dPixelLayout) -> Result<Self, Self::Error> {
        Self::from_repr(value as usize).ok_or(())
    }
}

impl EnumKey<{ Self::COUNT }> for Rav1dPixelLayout {
    const VALUES: [Self; Self::COUNT] = [Self::I400, Self::I420, Self::I422, Self::I444];

    fn as_usize(self) -> usize {
        self as usize
    }
}

impl BitAnd for Rav1dPixelLayout {
    type Output = bool;

    fn bitand(self, rhs: Self) -> Self::Output {
        (self as usize & rhs as usize) != 0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, EnumCount)]
pub(crate) enum Rav1dPixelLayoutSubSampled {
    I420,
    I422,
    I444,
}

impl EnumKey<{ Self::COUNT }> for Rav1dPixelLayoutSubSampled {
    const VALUES: [Self; Self::COUNT] = [Self::I420, Self::I422, Self::I444];

    fn as_usize(self) -> usize {
        self as usize
    }
}

impl TryFrom<Rav1dPixelLayout> for Rav1dPixelLayoutSubSampled {
    type Error = ();

    fn try_from(value: Rav1dPixelLayout) -> Result<Self, Self::Error> {
        use Rav1dPixelLayout::*;
        Ok(match value {
            I400 => return Err(()),
            I420 => Self::I420,
            I422 => Self::I422,
            I444 => Self::I444,
        })
    }
}

impl From<Rav1dPixelLayoutSubSampled> for Rav1dPixelLayout {
    fn from(value: Rav1dPixelLayoutSubSampled) -> Self {
        use Rav1dPixelLayoutSubSampled::*;
        match value {
            I420 => Self::I420,
            I422 => Self::I422,
            I444 => Self::I444,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, FromRepr, Default)]
pub enum Rav1dFrameType {
    #[default] // Not really a real default.
    Key = 0,
    Inter = 1,
    Intra = 2,
    Switch = 3,
}

impl Rav1dFrameType {
    pub const fn into_rav1d(self) -> Dav1dFrameType {
        self as Dav1dFrameType
    }
}

pub type Dav1dFrameType = c_uint;
pub const DAV1D_FRAME_TYPE_KEY: Dav1dFrameType = Rav1dFrameType::Key.into_rav1d();
pub const DAV1D_FRAME_TYPE_INTER: Dav1dFrameType = Rav1dFrameType::Inter.into_rav1d();
pub const DAV1D_FRAME_TYPE_INTRA: Dav1dFrameType = Rav1dFrameType::Intra.into_rav1d();
pub const DAV1D_FRAME_TYPE_SWITCH: Dav1dFrameType = Rav1dFrameType::Switch.into_rav1d();

impl From<Rav1dFrameType> for Dav1dFrameType {
    fn from(value: Rav1dFrameType) -> Self {
        value.into_rav1d()
    }
}

impl TryFrom<Dav1dFrameType> for Rav1dFrameType {
    type Error = ();

    fn try_from(value: Dav1dFrameType) -> Result<Self, Self::Error> {
        Self::from_repr(value as usize).ok_or(())
    }
}

impl Rav1dFrameType {
    pub const fn is_inter_or_switch(&self) -> bool {
        matches!(self, Self::Inter | Self::Switch)
    }

    pub const fn is_key_or_intra(&self) -> bool {
        matches!(self, Self::Key | Self::Intra)
    }
}

pub type Dav1dColorPrimaries = c_uint;
pub const DAV1D_COLOR_PRI_BT709: Dav1dColorPrimaries = Rav1dColorPrimaries::BT709.to_dav1d();
pub const DAV1D_COLOR_PRI_UNKNOWN: Dav1dColorPrimaries = Rav1dColorPrimaries::UNKNOWN.to_dav1d();
pub const DAV1D_COLOR_PRI_BT470M: Dav1dColorPrimaries = Rav1dColorPrimaries::BT470M.to_dav1d();
pub const DAV1D_COLOR_PRI_BT470BG: Dav1dColorPrimaries = Rav1dColorPrimaries::BT470BG.to_dav1d();
pub const DAV1D_COLOR_PRI_BT601: Dav1dColorPrimaries = Rav1dColorPrimaries::BT601.to_dav1d();
pub const DAV1D_COLOR_PRI_SMPTE240: Dav1dColorPrimaries = Rav1dColorPrimaries::SMPTE240.to_dav1d();
pub const DAV1D_COLOR_PRI_FILM: Dav1dColorPrimaries = Rav1dColorPrimaries::FILM.to_dav1d();
pub const DAV1D_COLOR_PRI_BT2020: Dav1dColorPrimaries = Rav1dColorPrimaries::BT2020.to_dav1d();
pub const DAV1D_COLOR_PRI_XYZ: Dav1dColorPrimaries = Rav1dColorPrimaries::XYZ.to_dav1d();
pub const DAV1D_COLOR_PRI_SMPTE431: Dav1dColorPrimaries = Rav1dColorPrimaries::SMPTE431.to_dav1d();
pub const DAV1D_COLOR_PRI_SMPTE432: Dav1dColorPrimaries = Rav1dColorPrimaries::SMPTE432.to_dav1d();
pub const DAV1D_COLOR_PRI_EBU3213: Dav1dColorPrimaries = Rav1dColorPrimaries::EBU3213.to_dav1d();
// this symbol is defined by dav1d, but not part of the spec
pub const DAV1D_COLOR_PRI_RESERVED: Dav1dColorPrimaries = 255;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rav1dColorPrimaries(pub u8);

impl Rav1dColorPrimaries {
    pub const BT709: Self = Self(1);
    pub const UNKNOWN: Self = Self(2);
    pub const BT470M: Self = Self(4);
    pub const BT470BG: Self = Self(5);
    pub const BT601: Self = Self(6);
    pub const SMPTE240: Self = Self(7);
    pub const FILM: Self = Self(8);
    pub const BT2020: Self = Self(9);
    pub const XYZ: Self = Self(10);
    pub const SMPTE431: Self = Self(11);
    pub const SMPTE432: Self = Self(12);
    pub const EBU3213: Self = Self(22);

    const fn to_dav1d(self) -> Dav1dColorPrimaries {
        self.0 as Dav1dColorPrimaries
    }
}

impl From<Rav1dColorPrimaries> for Dav1dColorPrimaries {
    fn from(value: Rav1dColorPrimaries) -> Self {
        value.to_dav1d()
    }
}

impl TryFrom<Dav1dColorPrimaries> for Rav1dColorPrimaries {
    type Error = ();

    fn try_from(value: Dav1dColorPrimaries) -> Result<Self, Self::Error> {
        u8::try_from(value).map(Self).map_err(|_| ())
    }
}

pub type Dav1dTransferCharacteristics = c_uint;
pub const DAV1D_TRC_BT709: Dav1dTransferCharacteristics =
    Rav1dTransferCharacteristics::BT709.to_dav1d();
pub const DAV1D_TRC_UNKNOWN: Dav1dTransferCharacteristics =
    Rav1dTransferCharacteristics::UNKNOWN.to_dav1d();
pub const DAV1D_TRC_BT470M: Dav1dTransferCharacteristics =
    Rav1dTransferCharacteristics::BT470M.to_dav1d();
pub const DAV1D_TRC_BT470BG: Dav1dTransferCharacteristics =
    Rav1dTransferCharacteristics::BT470BG.to_dav1d();
pub const DAV1D_TRC_BT601: Dav1dTransferCharacteristics =
    Rav1dTransferCharacteristics::BT601.to_dav1d();
pub const DAV1D_TRC_SMPTE240: Dav1dTransferCharacteristics =
    Rav1dTransferCharacteristics::SMPTE240.to_dav1d();
pub const DAV1D_TRC_LINEAR: Dav1dTransferCharacteristics =
    Rav1dTransferCharacteristics::LINEAR.to_dav1d();
pub const DAV1D_TRC_LOG100: Dav1dTransferCharacteristics =
    Rav1dTransferCharacteristics::LOG100.to_dav1d();
pub const DAV1D_TRC_LOG100_SQRT10: Dav1dTransferCharacteristics =
    Rav1dTransferCharacteristics::LOG100_SQRT10.to_dav1d();
pub const DAV1D_TRC_IEC61966: Dav1dTransferCharacteristics =
    Rav1dTransferCharacteristics::IEC61966.to_dav1d();
pub const DAV1D_TRC_BT1361: Dav1dTransferCharacteristics =
    Rav1dTransferCharacteristics::BT1361.to_dav1d();
pub const DAV1D_TRC_SRGB: Dav1dTransferCharacteristics =
    Rav1dTransferCharacteristics::SRGB.to_dav1d();
pub const DAV1D_TRC_BT2020_10BIT: Dav1dTransferCharacteristics =
    Rav1dTransferCharacteristics::BT2020_10BIT.to_dav1d();
pub const DAV1D_TRC_BT2020_12BIT: Dav1dTransferCharacteristics =
    Rav1dTransferCharacteristics::BT2020_12BIT.to_dav1d();
pub const DAV1D_TRC_SMPTE2084: Dav1dTransferCharacteristics =
    Rav1dTransferCharacteristics::SMPTE2084.to_dav1d();
pub const DAV1D_TRC_SMPTE428: Dav1dTransferCharacteristics =
    Rav1dTransferCharacteristics::SMPTE428.to_dav1d();
pub const DAV1D_TRC_HLG: Dav1dTransferCharacteristics =
    Rav1dTransferCharacteristics::HLG.to_dav1d();
// this symbol is defined by dav1d, but not part of the spec
pub const DAV1D_TRC_RESERVED: Dav1dTransferCharacteristics = 255;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rav1dTransferCharacteristics(pub u8);

impl Rav1dTransferCharacteristics {
    pub const _RESERVED_0: Self = Self(0);
    pub const BT709: Self = Self(1);
    pub const UNKNOWN: Self = Self(2);
    pub const _RESERVED_3: Self = Self(3);
    pub const BT470M: Self = Self(4);
    pub const BT470BG: Self = Self(5);
    pub const BT601: Self = Self(6);
    pub const SMPTE240: Self = Self(7);
    pub const LINEAR: Self = Self(8);
    pub const LOG100: Self = Self(9);
    pub const LOG100_SQRT10: Self = Self(10);
    pub const IEC61966: Self = Self(11);
    pub const BT1361: Self = Self(12);
    pub const SRGB: Self = Self(13);
    pub const BT2020_10BIT: Self = Self(14);
    pub const BT2020_12BIT: Self = Self(15);
    pub const SMPTE2084: Self = Self(16);
    pub const SMPTE428: Self = Self(17);
    pub const HLG: Self = Self(18);

    const fn to_dav1d(self) -> Dav1dTransferCharacteristics {
        self.0 as Dav1dTransferCharacteristics
    }
}

impl From<Rav1dTransferCharacteristics> for Dav1dTransferCharacteristics {
    fn from(value: Rav1dTransferCharacteristics) -> Self {
        value.to_dav1d()
    }
}

impl TryFrom<Dav1dTransferCharacteristics> for Rav1dTransferCharacteristics {
    type Error = ();

    fn try_from(value: Dav1dTransferCharacteristics) -> Result<Self, Self::Error> {
        u8::try_from(value).map(Self).map_err(|_| ())
    }
}

pub type Dav1dMatrixCoefficients = c_uint;
pub const DAV1D_MC_IDENTITY: Dav1dMatrixCoefficients = Rav1dMatrixCoefficients::IDENTITY.to_dav1d();
pub const DAV1D_MC_BT709: Dav1dMatrixCoefficients = Rav1dMatrixCoefficients::BT709.to_dav1d();
pub const DAV1D_MC_UNKNOWN: Dav1dMatrixCoefficients = Rav1dMatrixCoefficients::UNKNOWN.to_dav1d();
pub const DAV1D_MC_FCC: Dav1dMatrixCoefficients = Rav1dMatrixCoefficients::FCC.to_dav1d();
pub const DAV1D_MC_BT470BG: Dav1dMatrixCoefficients = Rav1dMatrixCoefficients::BT470BG.to_dav1d();
pub const DAV1D_MC_BT601: Dav1dMatrixCoefficients = Rav1dMatrixCoefficients::BT601.to_dav1d();
pub const DAV1D_MC_SMPTE240: Dav1dMatrixCoefficients = Rav1dMatrixCoefficients::SMPTE240.to_dav1d();
pub const DAV1D_MC_SMPTE_YCGCO: Dav1dMatrixCoefficients =
    Rav1dMatrixCoefficients::SMPTE_YCGCO.to_dav1d();
pub const DAV1D_MC_BT2020_NCL: Dav1dMatrixCoefficients =
    Rav1dMatrixCoefficients::BT2020_CL.to_dav1d();
pub const DAV1D_MC_BT2020_CL: Dav1dMatrixCoefficients =
    Rav1dMatrixCoefficients::BT2020_CL.to_dav1d();
pub const DAV1D_MC_SMPTE2085: Dav1dMatrixCoefficients =
    Rav1dMatrixCoefficients::SMPTE2085.to_dav1d();
pub const DAV1D_MC_CHROMAT_NCL: Dav1dMatrixCoefficients =
    Rav1dMatrixCoefficients::CHROMAT_NCL.to_dav1d();
pub const DAV1D_MC_CHROMAT_CL: Dav1dMatrixCoefficients =
    Rav1dMatrixCoefficients::CHROMAT_CL.to_dav1d();
pub const DAV1D_MC_ICTCP: Dav1dMatrixCoefficients = Rav1dMatrixCoefficients::ICTCP.to_dav1d();
// this symbol is defined by dav1d, but not part of the spec
pub const DAV1D_MC_RESERVED: Dav1dMatrixCoefficients = 255;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rav1dMatrixCoefficients(pub u8);

impl Rav1dMatrixCoefficients {
    pub const IDENTITY: Self = Self(0);
    pub const BT709: Self = Self(1);
    pub const UNKNOWN: Self = Self(2);
    pub const _RESERVED_3: Self = Self(3);
    pub const FCC: Self = Self(4);
    pub const BT470BG: Self = Self(5);
    pub const BT601: Self = Self(6);
    pub const SMPTE240: Self = Self(7);
    pub const SMPTE_YCGCO: Self = Self(8);
    pub const BT2020_NCL: Self = Self(9);
    pub const BT2020_CL: Self = Self(10);
    pub const SMPTE2085: Self = Self(11);
    pub const CHROMAT_NCL: Self = Self(12);
    pub const CHROMAT_CL: Self = Self(13);
    pub const ICTCP: Self = Self(14);

    const fn to_dav1d(self) -> Dav1dMatrixCoefficients {
        self.0 as Dav1dMatrixCoefficients
    }
}

impl From<Rav1dMatrixCoefficients> for Dav1dMatrixCoefficients {
    fn from(value: Rav1dMatrixCoefficients) -> Self {
        value.to_dav1d()
    }
}

impl TryFrom<Dav1dMatrixCoefficients> for Rav1dMatrixCoefficients {
    type Error = ();

    fn try_from(value: Dav1dMatrixCoefficients) -> Result<Self, Self::Error> {
        u8::try_from(value).map(Self).map_err(|_| ())
    }
}

pub type Dav1dChromaSamplePosition = c_uint;
pub const DAV1D_CHR_UNKNOWN: Dav1dChromaSamplePosition =
    Rav1dChromaSamplePosition::Unknown as Dav1dChromaSamplePosition;
pub const DAV1D_CHR_VERTICAL: Dav1dChromaSamplePosition =
    Rav1dChromaSamplePosition::Vertical as Dav1dChromaSamplePosition;
pub const DAV1D_CHR_COLOCATED: Dav1dChromaSamplePosition =
    Rav1dChromaSamplePosition::Colocated as Dav1dChromaSamplePosition;

#[derive(Clone, Copy, PartialEq, Eq, FromRepr)]
pub enum Rav1dChromaSamplePosition {
    Unknown = 0,
    /// Horizontally co-located with (0, 0) luma sample, vertical position
    /// in the middle between two luma samples
    Vertical = 1,
    /// co-located with (0, 0) luma sample
    Colocated = 2,
    _Reserved = 3,
}

impl From<Rav1dChromaSamplePosition> for Dav1dChromaSamplePosition {
    fn from(value: Rav1dChromaSamplePosition) -> Self {
        value as Dav1dChromaSamplePosition
    }
}

impl TryFrom<Dav1dChromaSamplePosition> for Rav1dChromaSamplePosition {
    type Error = ();

    fn try_from(value: Dav1dChromaSamplePosition) -> Result<Self, Self::Error> {
        Self::from_repr(value as usize).ok_or(())
    }
}

#[repr(C)]
pub struct Rav1dContentLightLevel {
    pub max_content_light_level: u16,
    pub max_frame_average_light_level: u16,
}

pub type Dav1dContentLightLevel = Rav1dContentLightLevel;

#[repr(C)]
pub struct Rav1dMasteringDisplay {
    pub primaries: [[u16; 2]; 3],
    pub white_point: [u16; 2],
    pub max_luminance: u32,
    pub min_luminance: u32,
}

pub type Dav1dMasteringDisplay = Rav1dMasteringDisplay;

#[repr(C)]
pub struct Dav1dITUTT35 {
    pub country_code: u8,
    pub country_code_extension_byte: u8,
    pub payload_size: usize,

    /// An immutable ptr to [`Rav1dITUTT35::payload`].
    ///
    /// [`Rav1dITUTT35::payload`] is a [`Box`], so it doesn't move,
    /// and [`Self::payload`]'s lifetime is that of the [`Rav1dITUTT35`],
    /// which is itself stored in a [`Box`] as returned from [`Rav1dITUTT35::to_immut`].
    pub payload: *const u8,
}

#[repr(C)]
pub struct Rav1dITUTT35 {
    pub country_code: u8,
    pub country_code_extension_byte: u8,
    pub payload: Box<[u8]>,
}

impl From<&Rav1dITUTT35> for Dav1dITUTT35 {
    fn from(value: &Rav1dITUTT35) -> Self {
        let Rav1dITUTT35 {
            country_code,
            country_code_extension_byte,
            ref payload,
        } = *value;
        Self {
            country_code,
            country_code_extension_byte,
            payload_size: payload.len(),
            payload: payload.as_ptr(),
        }
    }
}

impl Rav1dITUTT35 {
    pub fn to_immut(
        mutable: Arc<Mutex<Vec<Rav1dITUTT35>>>,
    ) -> Arc<DRav1d<Box<[Rav1dITUTT35]>, Box<[Dav1dITUTT35]>>> {
        let mutable = Arc::into_inner(mutable).unwrap().into_inner();
        let immutable = mutable.into_boxed_slice();
        let rav1d = immutable;
        let dav1d = rav1d.iter().map(Dav1dITUTT35::from).collect();
        Arc::new(DRav1d { rav1d, dav1d })
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Dav1dSequenceHeaderOperatingPoint {
    pub major_level: u8,
    pub minor_level: u8,
    pub initial_display_delay: u8,
    pub idc: u16,
    pub tier: u8,
    pub decoder_model_param_present: u8,
    pub display_model_param_present: u8,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(C)]
pub struct Rav1dSequenceHeaderOperatingPoint {
    pub major_level: u8,
    pub minor_level: u8,
    pub initial_display_delay: u8,
    pub idc: u16,
    pub tier: bool,
    pub decoder_model_param_present: bool,
    pub display_model_param_present: bool,
}

impl From<Dav1dSequenceHeaderOperatingPoint> for Rav1dSequenceHeaderOperatingPoint {
    fn from(value: Dav1dSequenceHeaderOperatingPoint) -> Self {
        let Dav1dSequenceHeaderOperatingPoint {
            major_level,
            minor_level,
            initial_display_delay,
            idc,
            tier,
            decoder_model_param_present,
            display_model_param_present,
        } = value;
        Self {
            major_level,
            minor_level,
            initial_display_delay,
            idc,
            tier: tier != 0,
            decoder_model_param_present: decoder_model_param_present != 0,
            display_model_param_present: display_model_param_present != 0,
        }
    }
}

impl From<Rav1dSequenceHeaderOperatingPoint> for Dav1dSequenceHeaderOperatingPoint {
    fn from(value: Rav1dSequenceHeaderOperatingPoint) -> Self {
        let Rav1dSequenceHeaderOperatingPoint {
            major_level,
            minor_level,
            initial_display_delay,
            idc,
            tier,
            decoder_model_param_present,
            display_model_param_present,
        } = value;
        Self {
            major_level,
            minor_level,
            initial_display_delay,
            idc,
            tier: tier.into(),
            decoder_model_param_present: decoder_model_param_present.into(),
            display_model_param_present: display_model_param_present.into(),
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Dav1dSequenceHeaderOperatingParameterInfo {
    pub decoder_buffer_delay: u32,
    pub encoder_buffer_delay: u32,
    pub low_delay_mode: u8,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(C)]
pub struct Rav1dSequenceHeaderOperatingParameterInfo {
    pub decoder_buffer_delay: u32,
    pub encoder_buffer_delay: u32,
    pub low_delay_mode: bool,
}

impl From<Dav1dSequenceHeaderOperatingParameterInfo> for Rav1dSequenceHeaderOperatingParameterInfo {
    fn from(value: Dav1dSequenceHeaderOperatingParameterInfo) -> Self {
        let Dav1dSequenceHeaderOperatingParameterInfo {
            decoder_buffer_delay,
            encoder_buffer_delay,
            low_delay_mode,
        } = value;
        Self {
            decoder_buffer_delay,
            encoder_buffer_delay,
            low_delay_mode: low_delay_mode != 0,
        }
    }
}

impl From<Rav1dSequenceHeaderOperatingParameterInfo> for Dav1dSequenceHeaderOperatingParameterInfo {
    fn from(value: Rav1dSequenceHeaderOperatingParameterInfo) -> Self {
        let Rav1dSequenceHeaderOperatingParameterInfo {
            decoder_buffer_delay,
            encoder_buffer_delay,
            low_delay_mode,
        } = value;
        Self {
            decoder_buffer_delay,
            encoder_buffer_delay,
            low_delay_mode: low_delay_mode.into(),
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dSequenceHeader {
    pub profile: u8,
    pub max_width: c_int,
    pub max_height: c_int,
    pub layout: Dav1dPixelLayout,
    pub pri: Dav1dColorPrimaries,
    pub trc: Dav1dTransferCharacteristics,
    pub mtrx: Dav1dMatrixCoefficients,
    pub chr: Dav1dChromaSamplePosition,
    pub hbd: u8,
    pub color_range: u8,
    pub num_operating_points: u8,
    pub operating_points: [Dav1dSequenceHeaderOperatingPoint; DAV1D_MAX_OPERATING_POINTS],
    pub still_picture: u8,
    pub reduced_still_picture_header: u8,
    pub timing_info_present: u8,
    /// > 0 if defined, 0 otherwise
    pub num_units_in_tick: u32,
    /// > 0 if defined, 0 otherwise
    pub time_scale: u32,
    pub equal_picture_interval: u8,
    pub num_ticks_per_picture: u32,
    pub decoder_model_info_present: u8,
    pub encoder_decoder_buffer_delay_length: u8,
    /// > 0 if defined, 0 otherwise
    pub num_units_in_decoding_tick: u32,
    pub buffer_removal_delay_length: u8,
    pub frame_presentation_delay_length: u8,
    pub display_model_info_present: u8,
    pub width_n_bits: u8,
    pub height_n_bits: u8,
    pub frame_id_numbers_present: u8,
    pub delta_frame_id_n_bits: u8,
    pub frame_id_n_bits: u8,
    pub sb128: u8,
    pub filter_intra: u8,
    pub intra_edge_filter: u8,
    pub inter_intra: u8,
    pub masked_compound: u8,
    pub warped_motion: u8,
    pub dual_filter: u8,
    pub order_hint: u8,
    pub jnt_comp: u8,
    pub ref_frame_mvs: u8,
    pub screen_content_tools: Dav1dAdaptiveBoolean,
    pub force_integer_mv: Dav1dAdaptiveBoolean,
    pub order_hint_n_bits: u8,
    pub super_res: u8,
    pub cdef: u8,
    pub restoration: u8,
    pub ss_hor: u8,
    pub ss_ver: u8,
    pub monochrome: u8,
    pub color_description_present: u8,
    pub separate_uv_delta_q: u8,
    pub film_grain_present: u8,
    pub operating_parameter_info:
        [Dav1dSequenceHeaderOperatingParameterInfo; DAV1D_MAX_OPERATING_POINTS],
}

#[derive(Clone, Copy, PartialEq, Eq, FromRepr)]
pub enum Rav1dProfile {
    Main = 0,
    High = 1,
    Professional = 2,
}

impl From<Rav1dProfile> for u8 {
    fn from(value: Rav1dProfile) -> Self {
        value as u8
    }
}

impl TryFrom<u8> for Rav1dProfile {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_repr(value as usize).ok_or(())
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Rav1dSequenceHeader {
    pub profile: Rav1dProfile,
    pub max_width: c_int,
    pub max_height: c_int,
    pub layout: Rav1dPixelLayout,
    pub pri: Rav1dColorPrimaries,
    pub trc: Rav1dTransferCharacteristics,
    pub mtrx: Rav1dMatrixCoefficients,
    pub chr: Rav1dChromaSamplePosition,
    pub hbd: u8,
    pub color_range: bool,
    pub num_operating_points: u8,
    pub operating_points: [Rav1dSequenceHeaderOperatingPoint; RAV1D_MAX_OPERATING_POINTS],
    pub still_picture: bool,
    pub reduced_still_picture_header: bool,
    pub timing_info_present: bool,
    /// > 0 if defined, 0 otherwise
    pub num_units_in_tick: u32,
    /// > 0 if defined, 0 otherwise
    pub time_scale: u32,
    pub equal_picture_interval: bool,
    pub num_ticks_per_picture: u32,
    pub decoder_model_info_present: bool,
    pub encoder_decoder_buffer_delay_length: u8,
    /// > 0 if defined, 0 otherwise
    pub num_units_in_decoding_tick: u32,
    pub buffer_removal_delay_length: u8,
    pub frame_presentation_delay_length: u8,
    pub display_model_info_present: bool,
    pub width_n_bits: u8,
    pub height_n_bits: u8,
    pub frame_id_numbers_present: bool,
    pub delta_frame_id_n_bits: u8,
    pub frame_id_n_bits: u8,
    pub sb128: bool,
    pub filter_intra: bool,
    pub intra_edge_filter: bool,
    pub inter_intra: bool,
    pub masked_compound: bool,
    pub warped_motion: bool,
    pub dual_filter: bool,
    pub order_hint: bool,
    pub jnt_comp: bool,
    pub ref_frame_mvs: bool,
    pub screen_content_tools: Rav1dAdaptiveBoolean,
    pub force_integer_mv: Rav1dAdaptiveBoolean,
    pub order_hint_n_bits: u8,
    pub super_res: bool,
    pub cdef: bool,
    pub restoration: bool,
    pub ss_hor: bool,
    pub ss_ver: bool,
    pub monochrome: bool,
    pub color_description_present: bool,
    pub separate_uv_delta_q: bool,
    pub film_grain_present: bool,
    pub operating_parameter_info:
        [Rav1dSequenceHeaderOperatingParameterInfo; RAV1D_MAX_OPERATING_POINTS],
}

impl Rav1dSequenceHeader {
    /// TODO(kkysen) We should split [`Rav1dSequenceHeader`] into an inner `struct`
    /// without the `operating_parameter_info` field
    /// so that we can just `#[derive(PartialEq, Eq)]` it.
    pub fn eq_without_operating_parameter_info(&self, other: &Self) -> bool {
        // Destructure so that there's a compile error
        // if we add fields and forget to update them here.
        let Self {
            profile,
            max_width,
            max_height,
            layout,
            pri,
            trc,
            mtrx,
            chr,
            hbd,
            color_range,
            num_operating_points,
            operating_points,
            still_picture,
            reduced_still_picture_header,
            timing_info_present,
            num_units_in_tick,
            time_scale,
            equal_picture_interval,
            num_ticks_per_picture,
            decoder_model_info_present,
            encoder_decoder_buffer_delay_length,
            num_units_in_decoding_tick,
            buffer_removal_delay_length,
            frame_presentation_delay_length,
            display_model_info_present,
            width_n_bits,
            height_n_bits,
            frame_id_numbers_present,
            delta_frame_id_n_bits,
            frame_id_n_bits,
            sb128,
            filter_intra,
            intra_edge_filter,
            inter_intra,
            masked_compound,
            warped_motion,
            dual_filter,
            order_hint,
            jnt_comp,
            ref_frame_mvs,
            screen_content_tools,
            force_integer_mv,
            order_hint_n_bits,
            super_res,
            cdef,
            restoration,
            ss_hor,
            ss_ver,
            monochrome,
            color_description_present,
            separate_uv_delta_q,
            film_grain_present,
            operating_parameter_info: _,
        } = self;
        true && *profile == other.profile
            && *max_width == other.max_width
            && *max_height == other.max_height
            && *layout == other.layout
            && *pri == other.pri
            && *trc == other.trc
            && *mtrx == other.mtrx
            && *chr == other.chr
            && *hbd == other.hbd
            && *color_range == other.color_range
            && *num_operating_points == other.num_operating_points
            && *operating_points == other.operating_points
            && *still_picture == other.still_picture
            && *reduced_still_picture_header == other.reduced_still_picture_header
            && *timing_info_present == other.timing_info_present
            && *num_units_in_tick == other.num_units_in_tick
            && *time_scale == other.time_scale
            && *equal_picture_interval == other.equal_picture_interval
            && *num_ticks_per_picture == other.num_ticks_per_picture
            && *decoder_model_info_present == other.decoder_model_info_present
            && *encoder_decoder_buffer_delay_length == other.encoder_decoder_buffer_delay_length
            && *num_units_in_decoding_tick == other.num_units_in_decoding_tick
            && *buffer_removal_delay_length == other.buffer_removal_delay_length
            && *frame_presentation_delay_length == other.frame_presentation_delay_length
            && *display_model_info_present == other.display_model_info_present
            && *width_n_bits == other.width_n_bits
            && *height_n_bits == other.height_n_bits
            && *frame_id_numbers_present == other.frame_id_numbers_present
            && *delta_frame_id_n_bits == other.delta_frame_id_n_bits
            && *frame_id_n_bits == other.frame_id_n_bits
            && *sb128 == other.sb128
            && *filter_intra == other.filter_intra
            && *intra_edge_filter == other.intra_edge_filter
            && *inter_intra == other.inter_intra
            && *masked_compound == other.masked_compound
            && *warped_motion == other.warped_motion
            && *dual_filter == other.dual_filter
            && *order_hint == other.order_hint
            && *jnt_comp == other.jnt_comp
            && *ref_frame_mvs == other.ref_frame_mvs
            && *screen_content_tools == other.screen_content_tools
            && *force_integer_mv == other.force_integer_mv
            && *order_hint_n_bits == other.order_hint_n_bits
            && *super_res == other.super_res
            && *cdef == other.cdef
            && *restoration == other.restoration
            && *ss_hor == other.ss_hor
            && *ss_ver == other.ss_ver
            && *monochrome == other.monochrome
            && *color_description_present == other.color_description_present
            && *separate_uv_delta_q == other.separate_uv_delta_q
            && *film_grain_present == other.film_grain_present
    }
}

impl From<Dav1dSequenceHeader> for Rav1dSequenceHeader {
    fn from(value: Dav1dSequenceHeader) -> Self {
        let Dav1dSequenceHeader {
            profile,
            max_width,
            max_height,
            layout,
            pri,
            trc,
            mtrx,
            chr,
            hbd,
            color_range,
            num_operating_points,
            operating_points,
            still_picture,
            reduced_still_picture_header,
            timing_info_present,
            num_units_in_tick,
            time_scale,
            equal_picture_interval,
            num_ticks_per_picture,
            decoder_model_info_present,
            encoder_decoder_buffer_delay_length,
            num_units_in_decoding_tick,
            buffer_removal_delay_length,
            frame_presentation_delay_length,
            display_model_info_present,
            width_n_bits,
            height_n_bits,
            frame_id_numbers_present,
            delta_frame_id_n_bits,
            frame_id_n_bits,
            sb128,
            filter_intra,
            intra_edge_filter,
            inter_intra,
            masked_compound,
            warped_motion,
            dual_filter,
            order_hint,
            jnt_comp,
            ref_frame_mvs,
            screen_content_tools,
            force_integer_mv,
            order_hint_n_bits,
            super_res,
            cdef,
            restoration,
            ss_hor,
            ss_ver,
            monochrome,
            color_description_present,
            separate_uv_delta_q,
            film_grain_present,
            operating_parameter_info,
        } = value;
        Self {
            profile: profile.try_into().unwrap(),
            max_width,
            max_height,
            layout: layout.try_into().unwrap(),
            pri: pri.try_into().unwrap(),
            trc: trc.try_into().unwrap(),
            mtrx: mtrx.try_into().unwrap(),
            chr: chr.try_into().unwrap(),
            hbd,
            color_range: color_range != 0,
            num_operating_points,
            operating_points: operating_points.map(|c| c.into()),
            still_picture: still_picture != 0,
            reduced_still_picture_header: reduced_still_picture_header != 0,
            timing_info_present: timing_info_present != 0,
            num_units_in_tick,
            time_scale,
            equal_picture_interval: equal_picture_interval != 0,
            num_ticks_per_picture,
            decoder_model_info_present: decoder_model_info_present != 0,
            encoder_decoder_buffer_delay_length,
            num_units_in_decoding_tick,
            buffer_removal_delay_length,
            frame_presentation_delay_length,
            display_model_info_present: display_model_info_present != 0,
            width_n_bits,
            height_n_bits,
            frame_id_numbers_present: frame_id_numbers_present != 0,
            delta_frame_id_n_bits,
            frame_id_n_bits,
            sb128: sb128 != 0,
            filter_intra: filter_intra != 0,
            intra_edge_filter: intra_edge_filter != 0,
            inter_intra: inter_intra != 0,
            masked_compound: masked_compound != 0,
            warped_motion: warped_motion != 0,
            dual_filter: dual_filter != 0,
            order_hint: order_hint != 0,
            jnt_comp: jnt_comp != 0,
            ref_frame_mvs: ref_frame_mvs != 0,
            screen_content_tools: screen_content_tools.try_into().unwrap(),
            force_integer_mv: force_integer_mv.try_into().unwrap(),
            order_hint_n_bits,
            super_res: super_res != 0,
            cdef: cdef != 0,
            restoration: restoration != 0,
            ss_hor: ss_hor != 0,
            ss_ver: ss_ver != 0,
            monochrome: monochrome != 0,
            color_description_present: color_description_present != 0,
            separate_uv_delta_q: separate_uv_delta_q != 0,
            film_grain_present: film_grain_present != 0,
            operating_parameter_info: operating_parameter_info.map(|c| c.into()),
        }
    }
}

impl From<Rav1dSequenceHeader> for Dav1dSequenceHeader {
    fn from(value: Rav1dSequenceHeader) -> Self {
        let Rav1dSequenceHeader {
            profile,
            max_width,
            max_height,
            layout,
            pri,
            trc,
            mtrx,
            chr,
            hbd,
            color_range,
            num_operating_points,
            operating_points,
            still_picture,
            reduced_still_picture_header,
            timing_info_present,
            num_units_in_tick,
            time_scale,
            equal_picture_interval,
            num_ticks_per_picture,
            decoder_model_info_present,
            encoder_decoder_buffer_delay_length,
            num_units_in_decoding_tick,
            buffer_removal_delay_length,
            frame_presentation_delay_length,
            display_model_info_present,
            width_n_bits,
            height_n_bits,
            frame_id_numbers_present,
            delta_frame_id_n_bits,
            frame_id_n_bits,
            sb128,
            filter_intra,
            intra_edge_filter,
            inter_intra,
            masked_compound,
            warped_motion,
            dual_filter,
            order_hint,
            jnt_comp,
            ref_frame_mvs,
            screen_content_tools,
            force_integer_mv,
            order_hint_n_bits,
            super_res,
            cdef,
            restoration,
            ss_hor,
            ss_ver,
            monochrome,
            color_description_present,
            separate_uv_delta_q,
            film_grain_present,
            operating_parameter_info,
        } = value;
        Self {
            profile: profile.into(),
            max_width,
            max_height,
            layout: layout.into(),
            pri: pri.into(),
            trc: trc.into(),
            mtrx: mtrx.into(),
            chr: chr.into(),
            hbd,
            color_range: color_range.into(),
            num_operating_points,
            operating_points: operating_points.map(|rust| rust.into()),
            still_picture: still_picture.into(),
            reduced_still_picture_header: reduced_still_picture_header.into(),
            timing_info_present: timing_info_present.into(),
            num_units_in_tick,
            time_scale,
            equal_picture_interval: equal_picture_interval.into(),
            num_ticks_per_picture,
            decoder_model_info_present: decoder_model_info_present.into(),
            encoder_decoder_buffer_delay_length,
            num_units_in_decoding_tick,
            buffer_removal_delay_length,
            frame_presentation_delay_length,
            display_model_info_present: display_model_info_present.into(),
            width_n_bits,
            height_n_bits,
            frame_id_numbers_present: frame_id_numbers_present.into(),
            delta_frame_id_n_bits,
            frame_id_n_bits,
            sb128: sb128.into(),
            filter_intra: filter_intra.into(),
            intra_edge_filter: intra_edge_filter.into(),
            inter_intra: inter_intra.into(),
            masked_compound: masked_compound.into(),
            warped_motion: warped_motion.into(),
            dual_filter: dual_filter.into(),
            order_hint: order_hint.into(),
            jnt_comp: jnt_comp.into(),
            ref_frame_mvs: ref_frame_mvs.into(),
            screen_content_tools: screen_content_tools.into(),
            force_integer_mv: force_integer_mv.into(),
            order_hint_n_bits,
            super_res: super_res.into(),
            cdef: cdef.into(),
            restoration: restoration.into(),
            ss_hor: ss_hor.into(),
            ss_ver: ss_ver.into(),
            monochrome: monochrome.into(),
            color_description_present: color_description_present.into(),
            separate_uv_delta_q: separate_uv_delta_q.into(),
            film_grain_present: film_grain_present.into(),
            operating_parameter_info: operating_parameter_info.map(|rust| rust.into()),
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dSegmentationData {
    pub delta_q: i16,
    pub delta_lf_y_v: i8,
    pub delta_lf_y_h: i8,
    pub delta_lf_u: i8,
    pub delta_lf_v: i8,
    pub r#ref: i8,
    pub skip: u8,
    pub globalmv: u8,
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct Rav1dSegmentationData {
    pub delta_q: i16,
    pub delta_lf_y_v: i8,
    pub delta_lf_y_h: i8,
    pub delta_lf_u: i8,
    pub delta_lf_v: i8,
    pub r#ref: i8,
    pub skip: bool,
    pub globalmv: bool,
}

impl From<Dav1dSegmentationData> for Rav1dSegmentationData {
    fn from(value: Dav1dSegmentationData) -> Self {
        let Dav1dSegmentationData {
            delta_q,
            delta_lf_y_v,
            delta_lf_y_h,
            delta_lf_u,
            delta_lf_v,
            r#ref,
            skip,
            globalmv,
        } = value;
        Self {
            delta_q,
            delta_lf_y_v,
            delta_lf_y_h,
            delta_lf_u,
            delta_lf_v,
            r#ref,
            skip: skip != 0,
            globalmv: globalmv != 0,
        }
    }
}

impl From<Rav1dSegmentationData> for Dav1dSegmentationData {
    fn from(value: Rav1dSegmentationData) -> Self {
        let Rav1dSegmentationData {
            delta_q,
            delta_lf_y_v,
            delta_lf_y_h,
            delta_lf_u,
            delta_lf_v,
            r#ref,
            skip,
            globalmv,
        } = value;
        Self {
            delta_q,
            delta_lf_y_v,
            delta_lf_y_h,
            delta_lf_u,
            delta_lf_v,
            r#ref,
            skip: skip.into(),
            globalmv: globalmv.into(),
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dSegmentationDataSet {
    pub d: [Dav1dSegmentationData; DAV1D_MAX_SEGMENTS as usize],
    pub preskip: u8,
    pub last_active_segid: i8,
}

#[derive(Clone, Default)]
pub struct Rav1dSegmentationDataSet {
    pub d: [Rav1dSegmentationData; SegmentId::COUNT],
    pub preskip: bool,
    pub last_active_segid: i8,
}

impl From<Dav1dSegmentationDataSet> for Rav1dSegmentationDataSet {
    fn from(value: Dav1dSegmentationDataSet) -> Self {
        let Dav1dSegmentationDataSet {
            d,
            preskip,
            last_active_segid,
        } = value;
        Self {
            d: d.map(|c| c.into()),
            preskip: preskip != 0,
            last_active_segid,
        }
    }
}

impl From<Rav1dSegmentationDataSet> for Dav1dSegmentationDataSet {
    fn from(value: Rav1dSegmentationDataSet) -> Self {
        let Rav1dSegmentationDataSet {
            d,
            preskip,
            last_active_segid,
        } = value;
        Self {
            d: d.map(|rust| rust.into()),
            preskip: preskip.into(),
            last_active_segid,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dLoopfilterModeRefDeltas {
    pub mode_delta: [i8; 2],
    pub ref_delta: [i8; DAV1D_TOTAL_REFS_PER_FRAME],
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct Rav1dLoopfilterModeRefDeltas {
    pub mode_delta: [i8; 2],
    pub ref_delta: [i8; RAV1D_TOTAL_REFS_PER_FRAME],
}

impl From<Dav1dLoopfilterModeRefDeltas> for Rav1dLoopfilterModeRefDeltas {
    fn from(value: Dav1dLoopfilterModeRefDeltas) -> Self {
        let Dav1dLoopfilterModeRefDeltas {
            mode_delta,
            ref_delta,
        } = value;
        Self {
            mode_delta,
            ref_delta,
        }
    }
}

impl From<Rav1dLoopfilterModeRefDeltas> for Dav1dLoopfilterModeRefDeltas {
    fn from(value: Rav1dLoopfilterModeRefDeltas) -> Self {
        let Rav1dLoopfilterModeRefDeltas {
            mode_delta,
            ref_delta,
        } = value;
        Self {
            mode_delta,
            ref_delta,
        }
    }
}

#[derive(Clone, Default)]
pub struct Rav1dFilmGrainData {
    pub seed: c_uint,
    pub num_y_points: c_int,
    pub y_points: [[u8; 2]; 14],
    pub chroma_scaling_from_luma: bool,
    pub num_uv_points: [c_int; 2],
    pub uv_points: [[[u8; 2]; 10]; 2],
    pub scaling_shift: u8,
    pub ar_coeff_lag: c_int,
    pub ar_coeffs_y: [i8; 24],
    pub ar_coeffs_uv: [[i8; 28]; 2],
    pub ar_coeff_shift: u8,
    pub grain_scale_shift: u8,
    pub uv_mult: [c_int; 2],
    pub uv_luma_mult: [c_int; 2],
    pub uv_offset: [c_int; 2],
    pub overlap_flag: bool,
    pub clip_to_restricted_range: bool,
}

/// Must be 16-byte aligned for `psrad` on [`Self::ar_coeff_shift`].
/// See the docs for [`Self::ar_coeff_shift`] for an explanation.
#[derive(Clone)]
#[repr(C)]
#[repr(align(16))]
pub struct Dav1dFilmGrainData {
    pub seed: c_uint,
    pub num_y_points: c_int,
    pub y_points: [[u8; 2]; 14],
    pub chroma_scaling_from_luma: c_int,
    pub num_uv_points: [c_int; 2],
    pub uv_points: [[[u8; 2]; 10]; 2],
    pub scaling_shift: c_int,
    pub ar_coeff_lag: c_int,
    pub ar_coeffs_y: [i8; 24],
    pub ar_coeffs_uv: [[i8; 28]; 2],
    /// Must be 16-byte aligned for `psrad`.
    ///
    /// TODO(kkysen) This appears to be a bug in `dav1d`.
    ///
    /// x86 asm uses `psrad` on a pointer to [`Self::ar_coeff_shift`].
    /// When `psrad`'s shift operand is a memory address, i.e. a `XMMWORD PTR`,
    /// it loads 128 bits from it, shifts by the lower 64 bits,
    /// and requires the 128 bits to be 128-bit/16-byte aligned.
    ///
    /// Previously, and still in `dav1d`, [`Self::ar_coeff_shift`]
    /// is only 8-byte aligned, as is [`Self`]/[`Dav1dFilmGrainData`].
    /// However, in `dav1d`, [`Dav1dFilmGrainData`] is part of
    /// [`Dav1dFrameHeader`], which is allocated with [`malloc`].
    /// [`malloc`] happens to return 16-byte aligned pointers usually,
    /// but is not required to, so this is UB and will segfault if not aligned.
    ///
    /// Due to the [`Rav1dFilmGrainData`] to [`Dav1dFilmGrainData`]
    /// conversion done now, the [`Dav1dFilmGrainData`] is stored on the stack,
    /// and often will not be 16-byte aligned, and thus will often segfault.
    ///
    /// To fix this, [`Self::ar_coeff_shift`] must be 16-byte aligned.
    /// This cannot be done only for the field without changing the offsets of
    /// the subsequent fields, however, so we instead align
    /// [`Dav1dFilmGrainData`] itself with `#[repr(align(16))]`.
    /// [`Self::ar_coeff_shift`] is at offset `0xB0`/`176`,
    /// which is divisible by 16.
    ///
    /// `psrad` also loads a full 128 bits, not just the 64 bits of
    /// [`Self::ar_coeff_shift`], even if it doesn't read them,
    /// so we must ensure that the following 64 bits are also deferenceable.
    /// They indeed are in `dav1d`, but we must be careful,
    /// as [`Self::ar_coeff_shift`] being the last field would
    /// read 8 bytes out of bounds and be UB.
    ///
    /// [`malloc`]: libc::malloc
    pub ar_coeff_shift: u64,
    pub grain_scale_shift: c_int,
    pub uv_mult: [c_int; 2],
    pub uv_luma_mult: [c_int; 2],
    pub uv_offset: [c_int; 2],
    pub overlap_flag: c_int,
    pub clip_to_restricted_range: c_int,
}

impl From<Dav1dFilmGrainData> for Rav1dFilmGrainData {
    fn from(value: Dav1dFilmGrainData) -> Self {
        let Dav1dFilmGrainData {
            seed,
            num_y_points,
            y_points,
            chroma_scaling_from_luma,
            num_uv_points,
            uv_points,
            scaling_shift,
            ar_coeff_lag,
            ar_coeffs_y,
            ar_coeffs_uv,
            ar_coeff_shift,
            grain_scale_shift,
            uv_mult,
            uv_luma_mult,
            uv_offset,
            overlap_flag,
            clip_to_restricted_range,
        } = value;
        Self {
            seed,
            num_y_points,
            y_points,
            chroma_scaling_from_luma: chroma_scaling_from_luma != 0,
            num_uv_points,
            uv_points,
            scaling_shift: scaling_shift as u8,
            ar_coeff_lag,
            ar_coeffs_y,
            ar_coeffs_uv,
            ar_coeff_shift: ar_coeff_shift as u8,
            grain_scale_shift: grain_scale_shift as u8,
            uv_mult,
            uv_luma_mult,
            uv_offset,
            overlap_flag: overlap_flag != 0,
            clip_to_restricted_range: clip_to_restricted_range != 0,
        }
    }
}

impl From<Rav1dFilmGrainData> for Dav1dFilmGrainData {
    fn from(value: Rav1dFilmGrainData) -> Self {
        let Rav1dFilmGrainData {
            seed,
            num_y_points,
            y_points,
            chroma_scaling_from_luma,
            num_uv_points,
            uv_points,
            scaling_shift,
            ar_coeff_lag,
            ar_coeffs_y,
            ar_coeffs_uv,
            ar_coeff_shift,
            grain_scale_shift,
            uv_mult,
            uv_luma_mult,
            uv_offset,
            overlap_flag,
            clip_to_restricted_range,
        } = value;
        Self {
            seed,
            num_y_points,
            y_points,
            chroma_scaling_from_luma: chroma_scaling_from_luma as c_int,
            num_uv_points,
            uv_points,
            scaling_shift: scaling_shift.into(),
            ar_coeff_lag,
            ar_coeffs_y,
            ar_coeffs_uv,
            ar_coeff_shift: ar_coeff_shift.into(),
            grain_scale_shift: grain_scale_shift.into(),
            uv_mult,
            uv_luma_mult,
            uv_offset,
            overlap_flag: overlap_flag as c_int,
            clip_to_restricted_range: clip_to_restricted_range as c_int,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_film_grain {
    pub data: Dav1dFilmGrainData,
    pub present: u8,
    pub update: u8,
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct Rav1dFrameHeader_film_grain {
    pub data: Rav1dFilmGrainData,
    pub present: u8,
    pub update: u8,
}

impl From<Dav1dFrameHeader_film_grain> for Rav1dFrameHeader_film_grain {
    fn from(value: Dav1dFrameHeader_film_grain) -> Self {
        let Dav1dFrameHeader_film_grain {
            data,
            present,
            update,
        } = value;
        Self {
            data: data.into(),
            present,
            update,
        }
    }
}

impl From<Rav1dFrameHeader_film_grain> for Dav1dFrameHeader_film_grain {
    fn from(value: Rav1dFrameHeader_film_grain) -> Self {
        let Rav1dFrameHeader_film_grain {
            data,
            present,
            update,
        } = value;
        Self {
            data: data.into(),
            present,
            update,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeaderOperatingPoint {
    pub buffer_removal_time: u32,
}

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct Rav1dFrameHeaderOperatingPoint {
    pub buffer_removal_time: u32,
}

impl From<Dav1dFrameHeaderOperatingPoint> for Rav1dFrameHeaderOperatingPoint {
    fn from(value: Dav1dFrameHeaderOperatingPoint) -> Self {
        let Dav1dFrameHeaderOperatingPoint {
            buffer_removal_time,
        } = value;
        Self {
            buffer_removal_time,
        }
    }
}

impl From<Rav1dFrameHeaderOperatingPoint> for Dav1dFrameHeaderOperatingPoint {
    fn from(value: Rav1dFrameHeaderOperatingPoint) -> Self {
        let Rav1dFrameHeaderOperatingPoint {
            buffer_removal_time,
        } = value;
        Self {
            buffer_removal_time,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_super_res {
    pub width_scale_denominator: u8,
    pub enabled: u8,
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct Rav1dFrameHeader_super_res {
    pub width_scale_denominator: u8,
    pub enabled: bool,
}

impl From<Dav1dFrameHeader_super_res> for Rav1dFrameHeader_super_res {
    fn from(value: Dav1dFrameHeader_super_res) -> Self {
        let Dav1dFrameHeader_super_res {
            width_scale_denominator,
            enabled,
        } = value;
        Self {
            width_scale_denominator,
            enabled: enabled != 0,
        }
    }
}

impl From<Rav1dFrameHeader_super_res> for Dav1dFrameHeader_super_res {
    fn from(value: Rav1dFrameHeader_super_res) -> Self {
        let Rav1dFrameHeader_super_res {
            width_scale_denominator,
            enabled,
        } = value;
        Self {
            width_scale_denominator,
            enabled: enabled as u8,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_tiling {
    pub uniform: u8,
    pub n_bytes: u8,
    pub min_log2_cols: u8,
    pub max_log2_cols: u8,
    pub log2_cols: u8,
    pub cols: u8,
    pub min_log2_rows: u8,
    pub max_log2_rows: u8,
    pub log2_rows: u8,
    pub rows: u8,
    pub col_start_sb: [u16; DAV1D_MAX_TILE_COLS + 1],
    pub row_start_sb: [u16; DAV1D_MAX_TILE_ROWS + 1],
    pub update: u16,
}

#[derive(Clone)]
#[repr(C)]
pub struct Rav1dFrameHeader_tiling {
    pub uniform: bool,
    pub n_bytes: u8,
    pub min_log2_cols: u8,
    pub max_log2_cols: u8,
    pub log2_cols: u8,
    pub cols: u8,
    pub min_log2_rows: u8,
    pub max_log2_rows: u8,
    pub log2_rows: u8,
    pub rows: u8,
    pub col_start_sb: [u16; RAV1D_MAX_TILE_COLS + 1],
    pub row_start_sb: [u16; RAV1D_MAX_TILE_ROWS + 1],
    pub update: u16,
}

impl Default for Rav1dFrameHeader_tiling {
    fn default() -> Self {
        Self {
            uniform: Default::default(),
            n_bytes: Default::default(),
            min_log2_cols: Default::default(),
            max_log2_cols: Default::default(),
            log2_cols: Default::default(),
            cols: Default::default(),
            min_log2_rows: Default::default(),
            max_log2_rows: Default::default(),
            log2_rows: Default::default(),
            rows: Default::default(),
            col_start_sb: [Default::default(); RAV1D_MAX_TILE_COLS + 1],
            row_start_sb: [Default::default(); RAV1D_MAX_TILE_ROWS + 1],
            update: Default::default(),
        }
    }
}

impl From<Dav1dFrameHeader_tiling> for Rav1dFrameHeader_tiling {
    fn from(value: Dav1dFrameHeader_tiling) -> Self {
        let Dav1dFrameHeader_tiling {
            uniform,
            n_bytes,
            min_log2_cols,
            max_log2_cols,
            log2_cols,
            cols,
            min_log2_rows,
            max_log2_rows,
            log2_rows,
            rows,
            col_start_sb,
            row_start_sb,
            update,
        } = value;
        Self {
            uniform: uniform != 0,
            n_bytes,
            min_log2_cols,
            max_log2_cols,
            log2_cols,
            cols,
            min_log2_rows,
            max_log2_rows,
            log2_rows,
            rows,
            col_start_sb,
            row_start_sb,
            update,
        }
    }
}

impl From<Rav1dFrameHeader_tiling> for Dav1dFrameHeader_tiling {
    fn from(value: Rav1dFrameHeader_tiling) -> Self {
        let Rav1dFrameHeader_tiling {
            uniform,
            n_bytes,
            min_log2_cols,
            max_log2_cols,
            log2_cols,
            cols,
            min_log2_rows,
            max_log2_rows,
            log2_rows,
            rows,
            col_start_sb,
            row_start_sb,
            update,
        } = value;
        Self {
            uniform: uniform as u8,
            n_bytes,
            min_log2_cols,
            max_log2_cols,
            log2_cols,
            cols,
            min_log2_rows,
            max_log2_rows,
            log2_rows,
            rows,
            col_start_sb,
            row_start_sb,
            update,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_quant {
    pub yac: u8,
    pub ydc_delta: i8,
    pub udc_delta: i8,
    pub uac_delta: i8,
    pub vdc_delta: i8,
    pub vac_delta: i8,
    pub qm: u8,
    pub qm_y: u8,
    pub qm_u: u8,
    pub qm_v: u8,
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct Rav1dFrameHeader_quant {
    pub yac: u8,
    pub ydc_delta: i8,
    pub udc_delta: i8,
    pub uac_delta: i8,
    pub vdc_delta: i8,
    pub vac_delta: i8,
    pub qm: bool,
    pub qm_y: u8,
    pub qm_u: u8,
    pub qm_v: u8,
}

impl From<Dav1dFrameHeader_quant> for Rav1dFrameHeader_quant {
    fn from(value: Dav1dFrameHeader_quant) -> Self {
        let Dav1dFrameHeader_quant {
            yac,
            ydc_delta,
            udc_delta,
            uac_delta,
            vdc_delta,
            vac_delta,
            qm,
            qm_y,
            qm_u,
            qm_v,
        } = value;
        Self {
            yac,
            ydc_delta,
            udc_delta,
            uac_delta,
            vdc_delta,
            vac_delta,
            qm: qm != 0,
            qm_y,
            qm_u,
            qm_v,
        }
    }
}

impl From<Rav1dFrameHeader_quant> for Dav1dFrameHeader_quant {
    fn from(value: Rav1dFrameHeader_quant) -> Self {
        let Rav1dFrameHeader_quant {
            yac,
            ydc_delta,
            udc_delta,
            uac_delta,
            vdc_delta,
            vac_delta,
            qm,
            qm_y,
            qm_u,
            qm_v,
        } = value;
        Self {
            yac,
            ydc_delta,
            udc_delta,
            uac_delta,
            vdc_delta,
            vac_delta,
            qm: qm as u8,
            qm_y,
            qm_u,
            qm_v,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_segmentation {
    pub enabled: u8,
    pub update_map: u8,
    pub temporal: u8,
    pub update_data: u8,
    pub seg_data: Dav1dSegmentationDataSet,
    pub lossless: [u8; DAV1D_MAX_SEGMENTS as usize],
    pub qidx: [u8; DAV1D_MAX_SEGMENTS as usize],
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct Rav1dFrameHeader_segmentation {
    pub enabled: bool,
    pub update_map: bool,
    pub temporal: bool,
    pub update_data: bool,
    pub seg_data: Rav1dSegmentationDataSet,
    /// TODO compress `[bool; 8]` into `u8`.
    pub lossless: [bool; SegmentId::COUNT],
    pub qidx: [u8; SegmentId::COUNT],
}

impl From<Dav1dFrameHeader_segmentation> for Rav1dFrameHeader_segmentation {
    fn from(value: Dav1dFrameHeader_segmentation) -> Self {
        let Dav1dFrameHeader_segmentation {
            enabled,
            update_map,
            temporal,
            update_data,
            seg_data,
            lossless,
            qidx,
        } = value;
        Self {
            enabled: enabled != 0,
            update_map: update_map != 0,
            temporal: temporal != 0,
            update_data: update_data != 0,
            seg_data: seg_data.into(),
            lossless: lossless.map(|e| e != 0),
            qidx,
        }
    }
}

impl From<Rav1dFrameHeader_segmentation> for Dav1dFrameHeader_segmentation {
    fn from(value: Rav1dFrameHeader_segmentation) -> Self {
        let Rav1dFrameHeader_segmentation {
            enabled,
            update_map,
            temporal,
            update_data,
            seg_data,
            lossless,
            qidx,
        } = value;
        Self {
            enabled: enabled.into(),
            update_map: update_map.into(),
            temporal: temporal.into(),
            update_data: update_data.into(),
            seg_data: seg_data.into(),
            lossless: lossless.map(|e| e as u8),
            qidx,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_delta_q {
    pub present: u8,
    pub res_log2: u8,
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct Rav1dFrameHeader_delta_q {
    pub present: u8,
    pub res_log2: u8,
}

impl From<Dav1dFrameHeader_delta_q> for Rav1dFrameHeader_delta_q {
    fn from(value: Dav1dFrameHeader_delta_q) -> Self {
        let Dav1dFrameHeader_delta_q { present, res_log2 } = value;
        Self { present, res_log2 }
    }
}

impl From<Rav1dFrameHeader_delta_q> for Dav1dFrameHeader_delta_q {
    fn from(value: Rav1dFrameHeader_delta_q) -> Self {
        let Rav1dFrameHeader_delta_q { present, res_log2 } = value;
        Self { present, res_log2 }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_delta_lf {
    pub present: u8,
    pub res_log2: u8,
    pub multi: u8,
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct Rav1dFrameHeader_delta_lf {
    pub present: u8,
    pub res_log2: u8,
    pub multi: u8,
}

impl From<Dav1dFrameHeader_delta_lf> for Rav1dFrameHeader_delta_lf {
    fn from(value: Dav1dFrameHeader_delta_lf) -> Self {
        let Dav1dFrameHeader_delta_lf {
            present,
            res_log2,
            multi,
        } = value;
        Self {
            present,
            res_log2,
            multi,
        }
    }
}

impl From<Rav1dFrameHeader_delta_lf> for Dav1dFrameHeader_delta_lf {
    fn from(value: Rav1dFrameHeader_delta_lf) -> Self {
        let Rav1dFrameHeader_delta_lf {
            present,
            res_log2,
            multi,
        } = value;
        Self {
            present,
            res_log2,
            multi,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_delta {
    pub q: Dav1dFrameHeader_delta_q,
    pub lf: Dav1dFrameHeader_delta_lf,
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct Rav1dFrameHeader_delta {
    pub q: Rav1dFrameHeader_delta_q,
    pub lf: Rav1dFrameHeader_delta_lf,
}

impl From<Dav1dFrameHeader_delta> for Rav1dFrameHeader_delta {
    fn from(value: Dav1dFrameHeader_delta) -> Self {
        let Dav1dFrameHeader_delta { q, lf } = value;
        Self {
            q: q.into(),
            lf: lf.into(),
        }
    }
}

impl From<Rav1dFrameHeader_delta> for Dav1dFrameHeader_delta {
    fn from(value: Rav1dFrameHeader_delta) -> Self {
        let Rav1dFrameHeader_delta { q, lf } = value;
        Self {
            q: q.into(),
            lf: lf.into(),
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_loopfilter {
    pub level_y: [u8; 2],
    pub level_u: u8,
    pub level_v: u8,
    pub mode_ref_delta_enabled: u8,
    pub mode_ref_delta_update: u8,
    pub mode_ref_deltas: Dav1dLoopfilterModeRefDeltas,
    pub sharpness: u8,
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct Rav1dFrameHeader_loopfilter {
    pub level_y: [u8; 2],
    pub level_u: u8,
    pub level_v: u8,
    pub mode_ref_delta_enabled: u8,
    pub mode_ref_delta_update: u8,
    pub mode_ref_deltas: Rav1dLoopfilterModeRefDeltas,
    pub sharpness: u8,
}

impl From<Dav1dFrameHeader_loopfilter> for Rav1dFrameHeader_loopfilter {
    fn from(value: Dav1dFrameHeader_loopfilter) -> Self {
        let Dav1dFrameHeader_loopfilter {
            level_y,
            level_u,
            level_v,
            mode_ref_delta_enabled,
            mode_ref_delta_update,
            mode_ref_deltas,
            sharpness,
        } = value;
        Self {
            level_y,
            level_u,
            level_v,
            mode_ref_delta_enabled,
            mode_ref_delta_update,
            mode_ref_deltas: mode_ref_deltas.into(),
            sharpness,
        }
    }
}

impl From<Rav1dFrameHeader_loopfilter> for Dav1dFrameHeader_loopfilter {
    fn from(value: Rav1dFrameHeader_loopfilter) -> Self {
        let Rav1dFrameHeader_loopfilter {
            level_y,
            level_u,
            level_v,
            mode_ref_delta_enabled,
            mode_ref_delta_update,
            mode_ref_deltas,
            sharpness,
        } = value;
        Self {
            level_y,
            level_u,
            level_v,
            mode_ref_delta_enabled,
            mode_ref_delta_update,
            mode_ref_deltas: mode_ref_deltas.into(),
            sharpness,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_cdef {
    pub damping: u8,
    pub n_bits: u8,
    pub y_strength: [u8; DAV1D_MAX_CDEF_STRENGTHS],
    pub uv_strength: [u8; DAV1D_MAX_CDEF_STRENGTHS],
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct Rav1dFrameHeader_cdef {
    pub damping: u8,
    pub n_bits: u8,
    pub y_strength: [u8; RAV1D_MAX_CDEF_STRENGTHS],
    pub uv_strength: [u8; RAV1D_MAX_CDEF_STRENGTHS],
}

impl From<Dav1dFrameHeader_cdef> for Rav1dFrameHeader_cdef {
    fn from(value: Dav1dFrameHeader_cdef) -> Self {
        let Dav1dFrameHeader_cdef {
            damping,
            n_bits,
            y_strength,
            uv_strength,
        } = value;
        Self {
            damping,
            n_bits,
            y_strength,
            uv_strength,
        }
    }
}

impl From<Rav1dFrameHeader_cdef> for Dav1dFrameHeader_cdef {
    fn from(value: Rav1dFrameHeader_cdef) -> Self {
        let Rav1dFrameHeader_cdef {
            damping,
            n_bits,
            y_strength,
            uv_strength,
        } = value;
        Self {
            damping,
            n_bits,
            y_strength,
            uv_strength,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_restoration {
    pub r#type: [Dav1dRestorationType; 3],
    pub unit_size: [u8; 2],
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct Rav1dFrameHeader_restoration {
    pub r#type: [Rav1dRestorationType; 3],
    pub unit_size: [u8; 2],
}

impl From<Dav1dFrameHeader_restoration> for Rav1dFrameHeader_restoration {
    fn from(value: Dav1dFrameHeader_restoration) -> Self {
        let Dav1dFrameHeader_restoration { r#type, unit_size } = value;
        Self {
            r#type: r#type.map(|e| Rav1dRestorationType::from_repr(e as usize).unwrap()),
            unit_size,
        }
    }
}

impl From<Rav1dFrameHeader_restoration> for Dav1dFrameHeader_restoration {
    fn from(value: Rav1dFrameHeader_restoration) -> Self {
        let Rav1dFrameHeader_restoration { r#type, unit_size } = value;
        Self {
            r#type: r#type.map(|e| e.to_repr()),
            unit_size,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader {
    pub film_grain: Dav1dFrameHeader_film_grain,
    pub frame_type: Dav1dFrameType,
    pub width: [c_int; 2],
    pub height: c_int,
    pub frame_offset: u8,
    pub temporal_id: u8,
    pub spatial_id: u8,
    pub show_existing_frame: u8,
    pub existing_frame_idx: u8,
    pub frame_id: u32,
    pub frame_presentation_delay: u32,
    pub show_frame: u8,
    pub showable_frame: u8,
    pub error_resilient_mode: u8,
    pub disable_cdf_update: u8,
    pub allow_screen_content_tools: u8,
    pub force_integer_mv: u8,
    pub frame_size_override: u8,
    pub primary_ref_frame: u8,
    pub buffer_removal_time_present: u8,
    pub operating_points: [Dav1dFrameHeaderOperatingPoint; DAV1D_MAX_OPERATING_POINTS],
    pub refresh_frame_flags: u8,
    pub render_width: c_int,
    pub render_height: c_int,
    pub super_res: Dav1dFrameHeader_super_res,
    pub have_render_size: u8,
    pub allow_intrabc: u8,
    pub frame_ref_short_signaling: u8,
    pub refidx: [i8; DAV1D_REFS_PER_FRAME],
    pub hp: u8,
    pub subpel_filter_mode: Dav1dFilterMode,
    pub switchable_motion_mode: u8,
    pub use_ref_frame_mvs: u8,
    pub refresh_context: u8,
    pub tiling: Dav1dFrameHeader_tiling,
    pub quant: Dav1dFrameHeader_quant,
    pub segmentation: Dav1dFrameHeader_segmentation,
    pub delta: Dav1dFrameHeader_delta,
    pub all_lossless: u8,
    pub loopfilter: Dav1dFrameHeader_loopfilter,
    pub cdef: Dav1dFrameHeader_cdef,
    pub restoration: Dav1dFrameHeader_restoration,
    pub txfm_mode: Dav1dTxfmMode,
    pub switchable_comp_refs: u8,
    pub skip_mode_allowed: u8,
    pub skip_mode_enabled: u8,
    pub skip_mode_refs: [i8; 2],
    pub warp_motion: u8,
    pub reduced_txtp_set: u8,
    pub gmv: [Dav1dWarpedMotionParams; DAV1D_REFS_PER_FRAME],
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct Rav1dFrameSize {
    pub width: [c_int; 2],
    pub height: c_int,
    pub render_width: c_int,
    pub render_height: c_int,
    pub super_res: Rav1dFrameHeader_super_res,
    pub have_render_size: bool,
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct Rav1dFrameSkipMode {
    pub allowed: bool,
    pub enabled: bool,
    pub refs: [i8; 2],
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct Rav1dFrameHeader {
    pub size: Rav1dFrameSize,
    pub film_grain: Rav1dFrameHeader_film_grain,
    pub frame_type: Rav1dFrameType,
    pub frame_offset: u8,
    pub temporal_id: u8,
    pub spatial_id: u8,
    pub show_existing_frame: u8,
    pub existing_frame_idx: u8,
    pub frame_id: u32,
    pub frame_presentation_delay: u32,
    pub show_frame: u8,
    pub showable_frame: u8,
    pub error_resilient_mode: u8,
    pub disable_cdf_update: u8,
    pub allow_screen_content_tools: bool,
    pub force_integer_mv: bool,
    pub frame_size_override: bool,
    pub primary_ref_frame: u8,
    pub buffer_removal_time_present: u8,
    pub operating_points: [Rav1dFrameHeaderOperatingPoint; RAV1D_MAX_OPERATING_POINTS],
    pub refresh_frame_flags: u8,
    pub allow_intrabc: bool,
    pub frame_ref_short_signaling: u8,
    pub refidx: [i8; RAV1D_REFS_PER_FRAME],
    pub hp: bool,
    pub subpel_filter_mode: Rav1dFilterMode,
    pub switchable_motion_mode: u8,
    pub use_ref_frame_mvs: u8,
    pub refresh_context: u8,
    pub tiling: Rav1dFrameHeader_tiling,
    pub quant: Rav1dFrameHeader_quant,
    pub segmentation: Rav1dFrameHeader_segmentation,
    pub delta: Rav1dFrameHeader_delta,
    pub all_lossless: bool,
    pub loopfilter: Rav1dFrameHeader_loopfilter,
    pub cdef: Rav1dFrameHeader_cdef,
    pub restoration: Rav1dFrameHeader_restoration,
    pub txfm_mode: Rav1dTxfmMode,
    pub switchable_comp_refs: u8,
    pub skip_mode: Rav1dFrameSkipMode,
    pub warp_motion: u8,
    pub reduced_txtp_set: u8,
    pub gmv: [Rav1dWarpedMotionParams; RAV1D_REFS_PER_FRAME],
}

impl From<Dav1dFrameHeader> for Rav1dFrameHeader {
    fn from(value: Dav1dFrameHeader) -> Self {
        let Dav1dFrameHeader {
            film_grain,
            frame_type,
            width,
            height,
            frame_offset,
            temporal_id,
            spatial_id,
            show_existing_frame,
            existing_frame_idx,
            frame_id,
            frame_presentation_delay,
            show_frame,
            showable_frame,
            error_resilient_mode,
            disable_cdf_update,
            allow_screen_content_tools,
            force_integer_mv,
            frame_size_override,
            primary_ref_frame,
            buffer_removal_time_present,
            operating_points,
            refresh_frame_flags,
            render_width,
            render_height,
            super_res,
            have_render_size,
            allow_intrabc,
            frame_ref_short_signaling,
            refidx,
            hp,
            subpel_filter_mode,
            switchable_motion_mode,
            use_ref_frame_mvs,
            refresh_context,
            tiling,
            quant,
            segmentation,
            delta,
            all_lossless,
            loopfilter,
            cdef,
            restoration,
            txfm_mode,
            switchable_comp_refs,
            skip_mode_allowed,
            skip_mode_enabled,
            skip_mode_refs,
            warp_motion,
            reduced_txtp_set,
            gmv,
        } = value;
        Self {
            size: Rav1dFrameSize {
                width,
                height,
                render_width,
                render_height,
                super_res: super_res.into(),
                have_render_size: have_render_size != 0,
            },
            film_grain: film_grain.into(),
            frame_type: frame_type.try_into().unwrap(),
            frame_offset,
            temporal_id,
            spatial_id,
            show_existing_frame,
            existing_frame_idx,
            frame_id,
            frame_presentation_delay,
            show_frame,
            showable_frame,
            error_resilient_mode,
            disable_cdf_update,
            allow_screen_content_tools: allow_screen_content_tools != 0,
            force_integer_mv: force_integer_mv != 0,
            frame_size_override: frame_size_override != 0,
            primary_ref_frame,
            buffer_removal_time_present,
            operating_points: operating_points.map(|c| c.into()),
            refresh_frame_flags,
            allow_intrabc: allow_intrabc != 0,
            frame_ref_short_signaling,
            refidx,
            hp: hp != 0,
            subpel_filter_mode: subpel_filter_mode.try_into().unwrap(),
            switchable_motion_mode,
            use_ref_frame_mvs,
            refresh_context,
            tiling: tiling.into(),
            quant: quant.into(),
            segmentation: segmentation.into(),
            delta: delta.into(),
            all_lossless: all_lossless != 0,
            loopfilter: loopfilter.into(),
            cdef: cdef.into(),
            restoration: restoration.into(),
            txfm_mode: txfm_mode.try_into().unwrap(),
            switchable_comp_refs,
            skip_mode: Rav1dFrameSkipMode {
                allowed: skip_mode_allowed != 0,
                enabled: skip_mode_enabled != 0,
                refs: skip_mode_refs,
            },
            warp_motion,
            reduced_txtp_set,
            gmv: gmv.map(|c| c.try_into().unwrap()),
        }
    }
}

impl From<Rav1dFrameHeader> for Dav1dFrameHeader {
    fn from(value: Rav1dFrameHeader) -> Self {
        let Rav1dFrameHeader {
            size:
                Rav1dFrameSize {
                    width,
                    height,
                    render_width,
                    render_height,
                    super_res,
                    have_render_size,
                },
            film_grain,
            frame_type,
            frame_offset,
            temporal_id,
            spatial_id,
            show_existing_frame,
            existing_frame_idx,
            frame_id,
            frame_presentation_delay,
            show_frame,
            showable_frame,
            error_resilient_mode,
            disable_cdf_update,
            allow_screen_content_tools,
            force_integer_mv,
            frame_size_override,
            primary_ref_frame,
            buffer_removal_time_present,
            operating_points,
            refresh_frame_flags,
            allow_intrabc,
            frame_ref_short_signaling,
            refidx,
            hp,
            subpel_filter_mode,
            switchable_motion_mode,
            use_ref_frame_mvs,
            refresh_context,
            tiling,
            quant,
            segmentation,
            delta,
            all_lossless,
            loopfilter,
            cdef,
            restoration,
            txfm_mode,
            switchable_comp_refs,
            skip_mode:
                Rav1dFrameSkipMode {
                    allowed: skip_mode_allowed,
                    enabled: skip_mode_enabled,
                    refs: skip_mode_refs,
                },
            warp_motion,
            reduced_txtp_set,
            gmv,
        } = value;
        Self {
            film_grain: film_grain.into(),
            frame_type: frame_type.into(),
            width,
            height,
            frame_offset,
            temporal_id,
            spatial_id,
            show_existing_frame,
            existing_frame_idx,
            frame_id,
            frame_presentation_delay,
            show_frame,
            showable_frame,
            error_resilient_mode,
            disable_cdf_update,
            allow_screen_content_tools: allow_screen_content_tools.into(),
            force_integer_mv: force_integer_mv.into(),
            frame_size_override: frame_size_override.into(),
            primary_ref_frame,
            buffer_removal_time_present,
            operating_points: operating_points.map(|rust| rust.into()),
            refresh_frame_flags,
            render_width,
            render_height,
            super_res: super_res.into(),
            have_render_size: have_render_size.into(),
            allow_intrabc: allow_intrabc.into(),
            frame_ref_short_signaling,
            refidx,
            hp: hp.into(),
            subpel_filter_mode: subpel_filter_mode.into(),
            switchable_motion_mode,
            use_ref_frame_mvs,
            refresh_context,
            tiling: tiling.into(),
            quant: quant.into(),
            segmentation: segmentation.into(),
            delta: delta.into(),
            all_lossless: all_lossless.into(),
            loopfilter: loopfilter.into(),
            cdef: cdef.into(),
            restoration: restoration.into(),
            txfm_mode: txfm_mode.into(),
            switchable_comp_refs,
            skip_mode_allowed: skip_mode_allowed.into(),
            skip_mode_enabled: skip_mode_enabled.into(),
            skip_mode_refs,
            warp_motion,
            reduced_txtp_set,
            gmv: gmv.map(|rust| rust.into()),
        }
    }
}
