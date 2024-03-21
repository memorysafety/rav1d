use crate::src::enum_map::EnumKey;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ops::BitAnd;
use std::ops::Deref;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
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

// Constants from Section 3. "Symbols and abbreviated terms"
pub const DAV1D_MAX_CDEF_STRENGTHS: usize = 8;
pub const DAV1D_MAX_OPERATING_POINTS: usize = 32;
pub const DAV1D_MAX_TILE_COLS: usize = 64;
pub const DAV1D_MAX_TILE_ROWS: usize = 64;
pub const DAV1D_MAX_SEGMENTS: u8 = 8;
pub const DAV1D_NUM_REF_FRAMES: usize = 8;
pub const DAV1D_PRIMARY_REF_NONE: c_int = 7;
pub const DAV1D_REFS_PER_FRAME: usize = 7;
pub const DAV1D_TOTAL_REFS_PER_FRAME: usize = DAV1D_REFS_PER_FRAME + 1;

pub(crate) const RAV1D_MAX_CDEF_STRENGTHS: usize = DAV1D_MAX_CDEF_STRENGTHS;
pub(crate) const RAV1D_MAX_OPERATING_POINTS: usize = DAV1D_MAX_OPERATING_POINTS;
pub(crate) const RAV1D_MAX_TILE_COLS: usize = DAV1D_MAX_TILE_COLS;
pub(crate) const RAV1D_MAX_TILE_ROWS: usize = DAV1D_MAX_TILE_ROWS;
pub(crate) const RAV1D_MAX_SEGMENTS: u8 = DAV1D_MAX_SEGMENTS;
pub(crate) const _RAV1D_NUM_REF_FRAMES: usize = DAV1D_NUM_REF_FRAMES;
pub(crate) const RAV1D_PRIMARY_REF_NONE: c_int = DAV1D_PRIMARY_REF_NONE;
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

#[derive(Clone, Copy, PartialEq, Eq, FromRepr, EnumCount)]
pub enum Rav1dTxfmMode {
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

#[derive(Clone, Copy, PartialEq, Eq, FromRepr)]
pub enum Rav1dFilterMode {
    Regular8Tap = 0,
    Smooth8Tap = 1,
    Sharp8Tap = 2,
    Bilinear = 3,
    Switchable = 4,
}

impl Rav1dFilterMode {
    pub const N_FILTERS: usize = 4;
    pub const N_SWITCHABLE_FILTERS: u8 = 3;
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
pub const DAV1D_RESTORATION_NONE: Dav1dRestorationType =
    Rav1dRestorationType::None as Dav1dRestorationType;
pub const DAV1D_RESTORATION_SWITCHABLE: Dav1dRestorationType =
    Rav1dRestorationType::Switchable as Dav1dRestorationType;
pub const DAV1D_RESTORATION_WIENER: Dav1dRestorationType =
    Rav1dRestorationType::Wiener as Dav1dRestorationType;
pub const DAV1D_RESTORATION_SGRPROJ: Dav1dRestorationType =
    Rav1dRestorationType::SgrProj as Dav1dRestorationType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRepr, Default)]
pub enum Rav1dRestorationType {
    #[default]
    None = 0,
    Switchable = 1,
    Wiener = 2,
    SgrProj = 3,
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

#[derive(Default)]
pub struct Abcd(AtomicU64);

impl Abcd {
    fn pack(unpacked: [i16; 4]) -> u64 {
        let [[b1, b2], [b3, b4], [b5, b6], [b7, b8]] = unpacked.map(|x| x.to_ne_bytes());
        u64::from_ne_bytes([b1, b2, b3, b4, b5, b6, b7, b8])
    }

    fn unpack(packed: u64) -> [i16; 4] {
        let [b1, b2, b3, b4, b5, b6, b7, b8] = packed.to_ne_bytes();
        [[b1, b2], [b3, b4], [b5, b6], [b7, b8]].map(i16::from_ne_bytes)
    }

    pub fn new(abcd: [i16; 4]) -> Self {
        Self(AtomicU64::new(Self::pack(abcd)))
    }

    pub fn get(&self) -> [i16; 4] {
        Self::unpack(self.0.load(Ordering::Relaxed))
    }

    pub fn set(&self, abcd: [i16; 4]) {
        self.0.store(Self::pack(abcd), Ordering::Relaxed);
    }
}

impl Clone for Abcd {
    fn clone(&self) -> Self {
        Self::new(self.get())
    }
}

#[derive(Clone)]
pub struct Rav1dWarpedMotionParams {
    pub r#type: Rav1dWarpedMotionType,
    pub matrix: [i32; 6],
    pub abcd: Abcd,
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
            abcd: Abcd::new(abcd),
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

#[derive(Clone, Copy, PartialEq, Eq, FromRepr)]
pub enum Rav1dFrameType {
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
pub const DAV1D_COLOR_PRI_RESERVED: Dav1dColorPrimaries = 255;
pub const DAV1D_COLOR_PRI_EBU3213: Dav1dColorPrimaries = 22;
pub const DAV1D_COLOR_PRI_SMPTE432: Dav1dColorPrimaries = 12;
pub const DAV1D_COLOR_PRI_SMPTE431: Dav1dColorPrimaries = 11;
pub const DAV1D_COLOR_PRI_XYZ: Dav1dColorPrimaries = 10;
pub const DAV1D_COLOR_PRI_BT2020: Dav1dColorPrimaries = 9;
pub const DAV1D_COLOR_PRI_FILM: Dav1dColorPrimaries = 8;
pub const DAV1D_COLOR_PRI_SMPTE240: Dav1dColorPrimaries = 7;
pub const DAV1D_COLOR_PRI_BT601: Dav1dColorPrimaries = 6;
pub const DAV1D_COLOR_PRI_BT470BG: Dav1dColorPrimaries = 5;
pub const DAV1D_COLOR_PRI_BT470M: Dav1dColorPrimaries = 4;
pub const DAV1D_COLOR_PRI_UNKNOWN: Dav1dColorPrimaries = 2;
pub const DAV1D_COLOR_PRI_BT709: Dav1dColorPrimaries = 1;

pub(crate) type Rav1dColorPrimaries = c_uint;
pub(crate) const _RAV1D_COLOR_PRI_RESERVED: Rav1dColorPrimaries = DAV1D_COLOR_PRI_RESERVED;
pub(crate) const _RAV1D_COLOR_PRI_EBU3213: Rav1dColorPrimaries = DAV1D_COLOR_PRI_EBU3213;
pub(crate) const _RAV1D_COLOR_PRI_SMPTE432: Rav1dColorPrimaries = DAV1D_COLOR_PRI_SMPTE432;
pub(crate) const _RAV1D_COLOR_PRI_SMPTE431: Rav1dColorPrimaries = DAV1D_COLOR_PRI_SMPTE431;
pub(crate) const _RAV1D_COLOR_PRI_XYZ: Rav1dColorPrimaries = DAV1D_COLOR_PRI_XYZ;
pub(crate) const _RAV1D_COLOR_PRI_BT2020: Rav1dColorPrimaries = DAV1D_COLOR_PRI_BT2020;
pub(crate) const _RAV1D_COLOR_PRI_FILM: Rav1dColorPrimaries = DAV1D_COLOR_PRI_FILM;
pub(crate) const _RAV1D_COLOR_PRI_SMPTE240: Rav1dColorPrimaries = DAV1D_COLOR_PRI_SMPTE240;
pub(crate) const _RAV1D_COLOR_PRI_BT601: Rav1dColorPrimaries = DAV1D_COLOR_PRI_BT601;
pub(crate) const _RAV1D_COLOR_PRI_BT470BG: Rav1dColorPrimaries = DAV1D_COLOR_PRI_BT470BG;
pub(crate) const _RAV1D_COLOR_PRI_BT470M: Rav1dColorPrimaries = DAV1D_COLOR_PRI_BT470M;
pub(crate) const RAV1D_COLOR_PRI_UNKNOWN: Rav1dColorPrimaries = DAV1D_COLOR_PRI_UNKNOWN;
pub(crate) const RAV1D_COLOR_PRI_BT709: Rav1dColorPrimaries = DAV1D_COLOR_PRI_BT709;

pub type Dav1dTransferCharacteristics = c_uint;
pub const DAV1D_TRC_RESERVED: Dav1dTransferCharacteristics = 255;
pub const DAV1D_TRC_HLG: Dav1dTransferCharacteristics = 18;
pub const DAV1D_TRC_SMPTE428: Dav1dTransferCharacteristics = 17;
pub const DAV1D_TRC_SMPTE2084: Dav1dTransferCharacteristics = 16;
pub const DAV1D_TRC_BT2020_12BIT: Dav1dTransferCharacteristics = 15;
pub const DAV1D_TRC_BT2020_10BIT: Dav1dTransferCharacteristics = 14;
pub const DAV1D_TRC_SRGB: Dav1dTransferCharacteristics = 13;
pub const DAV1D_TRC_BT1361: Dav1dTransferCharacteristics = 12;
pub const DAV1D_TRC_IEC61966: Dav1dTransferCharacteristics = 11;
pub const DAV1D_TRC_LOG100_SQRT10: Dav1dTransferCharacteristics = 10;
pub const DAV1D_TRC_LOG100: Dav1dTransferCharacteristics = 9;
pub const DAV1D_TRC_LINEAR: Dav1dTransferCharacteristics = 8;
pub const DAV1D_TRC_SMPTE240: Dav1dTransferCharacteristics = 7;
pub const DAV1D_TRC_BT601: Dav1dTransferCharacteristics = 6;
pub const DAV1D_TRC_BT470BG: Dav1dTransferCharacteristics = 5;
pub const DAV1D_TRC_BT470M: Dav1dTransferCharacteristics = 4;
pub const DAV1D_TRC_UNKNOWN: Dav1dTransferCharacteristics = 2;
pub const DAV1D_TRC_BT709: Dav1dTransferCharacteristics = 1;

pub(crate) type Rav1dTransferCharacteristics = c_uint;
pub(crate) const _RAV1D_TRC_RESERVED: Rav1dTransferCharacteristics = DAV1D_TRC_RESERVED;
pub(crate) const _RAV1D_TRC_HLG: Rav1dTransferCharacteristics = DAV1D_TRC_HLG;
pub(crate) const _RAV1D_TRC_SMPTE428: Rav1dTransferCharacteristics = DAV1D_TRC_SMPTE428;
pub(crate) const _RAV1D_TRC_SMPTE2084: Rav1dTransferCharacteristics = DAV1D_TRC_SMPTE2084;
pub(crate) const _RAV1D_TRC_BT2020_12BIT: Rav1dTransferCharacteristics = DAV1D_TRC_BT2020_12BIT;
pub(crate) const _RAV1D_TRC_BT2020_10BIT: Rav1dTransferCharacteristics = DAV1D_TRC_BT2020_10BIT;
pub(crate) const RAV1D_TRC_SRGB: Rav1dTransferCharacteristics = DAV1D_TRC_SRGB;
pub(crate) const _RAV1D_TRC_BT1361: Rav1dTransferCharacteristics = DAV1D_TRC_BT1361;
pub(crate) const _RAV1D_TRC_IEC61966: Rav1dTransferCharacteristics = DAV1D_TRC_IEC61966;
pub(crate) const _RAV1D_TRC_LOG100_SQRT10: Rav1dTransferCharacteristics = DAV1D_TRC_LOG100_SQRT10;
pub(crate) const _RAV1D_TRC_LOG100: Rav1dTransferCharacteristics = DAV1D_TRC_LOG100;
pub(crate) const _RAV1D_TRC_LINEAR: Rav1dTransferCharacteristics = DAV1D_TRC_LINEAR;
pub(crate) const _RAV1D_TRC_SMPTE240: Rav1dTransferCharacteristics = DAV1D_TRC_SMPTE240;
pub(crate) const _RAV1D_TRC_BT601: Rav1dTransferCharacteristics = DAV1D_TRC_BT601;
pub(crate) const _RAV1D_TRC_BT470BG: Rav1dTransferCharacteristics = DAV1D_TRC_BT470BG;
pub(crate) const _RAV1D_TRC_BT470M: Rav1dTransferCharacteristics = DAV1D_TRC_BT470M;
pub(crate) const RAV1D_TRC_UNKNOWN: Rav1dTransferCharacteristics = DAV1D_TRC_UNKNOWN;
pub(crate) const _RAV1D_TRC_BT709: Rav1dTransferCharacteristics = DAV1D_TRC_BT709;

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
// dav1d defines this value as 255, but the specification defines it as 3
pub const DAV1D_MC_RESERVED: Dav1dMatrixCoefficients = 255;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rav1dMatrixCoefficients(pub u8);

impl Rav1dMatrixCoefficients {
    pub const IDENTITY: Self = Self(0);
    pub const BT709: Self = Self(1);
    pub const UNKNOWN: Self = Self(2);
    pub const RESERVED: Self = Self(3);
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

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dITUTT35 {
    pub country_code: u8,
    pub country_code_extension_byte: u8,
    pub payload_size: usize,
    pub payload: *mut u8,
}

#[derive(Clone)]
#[repr(C)]
pub struct Rav1dITUTT35 {
    pub country_code: u8,
    pub country_code_extension_byte: u8,
    pub payload: Box<[u8]>,
}

impl From<Rav1dITUTT35> for Dav1dITUTT35 {
    fn from(value: Rav1dITUTT35) -> Self {
        let Rav1dITUTT35 {
            country_code,
            country_code_extension_byte,
            payload,
        } = value;
        Self {
            country_code,
            country_code_extension_byte,
            payload_size: payload.len(),
            // Cast to a thin ptr.
            payload: Box::into_raw(payload).cast::<u8>(),
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Dav1dSequenceHeaderOperatingPoint {
    pub major_level: c_int,
    pub minor_level: c_int,
    pub initial_display_delay: c_int,
    pub idc: c_int,
    pub tier: c_int,
    pub decoder_model_param_present: c_int,
    pub display_model_param_present: c_int,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(C)]
pub struct Rav1dSequenceHeaderOperatingPoint {
    pub major_level: c_int,
    pub minor_level: c_int,
    pub initial_display_delay: c_int,
    pub idc: c_int,
    pub tier: c_int,
    pub decoder_model_param_present: c_int,
    pub display_model_param_present: c_int,
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
            tier,
            decoder_model_param_present,
            display_model_param_present,
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
            tier,
            decoder_model_param_present,
            display_model_param_present,
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Dav1dSequenceHeaderOperatingParameterInfo {
    pub decoder_buffer_delay: c_int,
    pub encoder_buffer_delay: c_int,
    pub low_delay_mode: c_int,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(C)]
pub struct Rav1dSequenceHeaderOperatingParameterInfo {
    pub decoder_buffer_delay: c_int,
    pub encoder_buffer_delay: c_int,
    pub low_delay_mode: c_int,
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
            low_delay_mode,
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
            low_delay_mode,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dSequenceHeader {
    pub profile: c_int,
    pub max_width: c_int,
    pub max_height: c_int,
    pub layout: Dav1dPixelLayout,
    pub pri: Dav1dColorPrimaries,
    pub trc: Dav1dTransferCharacteristics,
    pub mtrx: Dav1dMatrixCoefficients,
    pub chr: Dav1dChromaSamplePosition,
    pub hbd: c_int,
    pub color_range: c_int,
    pub num_operating_points: c_int,
    pub operating_points: [Dav1dSequenceHeaderOperatingPoint; DAV1D_MAX_OPERATING_POINTS],
    pub still_picture: c_int,
    pub reduced_still_picture_header: c_int,
    pub timing_info_present: c_int,
    pub num_units_in_tick: c_int,
    pub time_scale: c_int,
    pub equal_picture_interval: c_int,
    pub num_ticks_per_picture: c_uint,
    pub decoder_model_info_present: c_int,
    pub encoder_decoder_buffer_delay_length: c_int,
    pub num_units_in_decoding_tick: c_int,
    pub buffer_removal_delay_length: c_int,
    pub frame_presentation_delay_length: c_int,
    pub display_model_info_present: c_int,
    pub width_n_bits: c_int,
    pub height_n_bits: c_int,
    pub frame_id_numbers_present: c_int,
    pub delta_frame_id_n_bits: c_int,
    pub frame_id_n_bits: c_int,
    pub sb128: c_int,
    pub filter_intra: c_int,
    pub intra_edge_filter: c_int,
    pub inter_intra: c_int,
    pub masked_compound: c_int,
    pub warped_motion: c_int,
    pub dual_filter: c_int,
    pub order_hint: c_int,
    pub jnt_comp: c_int,
    pub ref_frame_mvs: c_int,
    pub screen_content_tools: Dav1dAdaptiveBoolean,
    pub force_integer_mv: Dav1dAdaptiveBoolean,
    pub order_hint_n_bits: c_int,
    pub super_res: c_int,
    pub cdef: c_int,
    pub restoration: c_int,
    pub ss_hor: c_int,
    pub ss_ver: c_int,
    pub monochrome: c_int,
    pub color_description_present: c_int,
    pub separate_uv_delta_q: c_int,
    pub film_grain_present: c_int,
    pub operating_parameter_info:
        [Dav1dSequenceHeaderOperatingParameterInfo; DAV1D_MAX_OPERATING_POINTS],
}

#[derive(Clone, Copy, PartialEq, Eq, FromRepr)]
pub enum Rav1dProfile {
    Main = 0,
    High = 1,
    Professional = 2,
}

impl From<Rav1dProfile> for c_int {
    fn from(value: Rav1dProfile) -> Self {
        value as c_int
    }
}

impl TryFrom<c_int> for Rav1dProfile {
    type Error = ();

    fn try_from(value: c_int) -> Result<Self, Self::Error> {
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
    pub hbd: c_int,
    pub color_range: c_int,
    pub num_operating_points: c_int,
    pub operating_points: [Rav1dSequenceHeaderOperatingPoint; RAV1D_MAX_OPERATING_POINTS],
    pub still_picture: c_int,
    pub reduced_still_picture_header: c_int,
    pub timing_info_present: c_int,
    pub num_units_in_tick: c_int,
    pub time_scale: c_int,
    pub equal_picture_interval: c_int,
    pub num_ticks_per_picture: c_uint,
    pub decoder_model_info_present: c_int,
    pub encoder_decoder_buffer_delay_length: c_int,
    pub num_units_in_decoding_tick: c_int,
    pub buffer_removal_delay_length: c_int,
    pub frame_presentation_delay_length: c_int,
    pub display_model_info_present: c_int,
    pub width_n_bits: c_int,
    pub height_n_bits: c_int,
    pub frame_id_numbers_present: c_int,
    pub delta_frame_id_n_bits: c_int,
    pub frame_id_n_bits: c_int,
    pub sb128: c_int,
    pub filter_intra: c_int,
    pub intra_edge_filter: c_int,
    pub inter_intra: c_int,
    pub masked_compound: c_int,
    pub warped_motion: c_int,
    pub dual_filter: c_int,
    pub order_hint: c_int,
    pub jnt_comp: c_int,
    pub ref_frame_mvs: c_int,
    pub screen_content_tools: Rav1dAdaptiveBoolean,
    pub force_integer_mv: Rav1dAdaptiveBoolean,
    pub order_hint_n_bits: c_int,
    pub super_res: c_int,
    pub cdef: c_int,
    pub restoration: c_int,
    pub ss_hor: c_int,
    pub ss_ver: c_int,
    pub monochrome: c_int,
    pub color_description_present: c_int,
    pub separate_uv_delta_q: c_int,
    pub film_grain_present: c_int,
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
            pri,
            trc,
            mtrx: mtrx.try_into().unwrap(),
            chr: chr.try_into().unwrap(),
            hbd,
            color_range,
            num_operating_points,
            operating_points: operating_points.map(|c| c.into()),
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
            screen_content_tools: screen_content_tools.try_into().unwrap(),
            force_integer_mv: force_integer_mv.try_into().unwrap(),
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
            pri,
            trc,
            mtrx: mtrx.into(),
            chr: chr.into(),
            hbd,
            color_range,
            num_operating_points,
            operating_points: operating_points.map(|rust| rust.into()),
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
            screen_content_tools: screen_content_tools.into(),
            force_integer_mv: force_integer_mv.into(),
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
            operating_parameter_info: operating_parameter_info.map(|rust| rust.into()),
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dSegmentationData {
    pub delta_q: c_int,
    pub delta_lf_y_v: c_int,
    pub delta_lf_y_h: c_int,
    pub delta_lf_u: c_int,
    pub delta_lf_v: c_int,
    pub r#ref: c_int,
    pub skip: c_int,
    pub globalmv: c_int,
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct Rav1dSegmentationData {
    pub delta_q: c_int,
    pub delta_lf_y_v: c_int,
    pub delta_lf_y_h: c_int,
    pub delta_lf_u: c_int,
    pub delta_lf_v: c_int,
    pub r#ref: c_int,
    pub skip: c_int,
    pub globalmv: c_int,
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
            skip,
            globalmv,
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
            skip,
            globalmv,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dSegmentationDataSet {
    pub d: [Dav1dSegmentationData; DAV1D_MAX_SEGMENTS as usize],
    pub preskip: c_int,
    pub last_active_segid: c_int,
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct Rav1dSegmentationDataSet {
    pub d: [Rav1dSegmentationData; RAV1D_MAX_SEGMENTS as usize],
    pub preskip: c_int,
    pub last_active_segid: c_int,
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
            preskip,
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
            preskip,
            last_active_segid,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dLoopfilterModeRefDeltas {
    pub mode_delta: [c_int; 2],
    pub ref_delta: [c_int; DAV1D_TOTAL_REFS_PER_FRAME],
}

#[derive(Clone)]
#[repr(C)]
pub struct Rav1dLoopfilterModeRefDeltas {
    pub mode_delta: [c_int; 2],
    pub ref_delta: [c_int; RAV1D_TOTAL_REFS_PER_FRAME],
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
    pub present: c_int,
    pub update: c_int,
}

#[derive(Clone)]
#[repr(C)]
pub struct Rav1dFrameHeader_film_grain {
    pub data: Rav1dFilmGrainData,
    pub present: c_int,
    pub update: c_int,
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
    pub buffer_removal_time: c_int,
}

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct Rav1dFrameHeaderOperatingPoint {
    pub buffer_removal_time: c_int,
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
    pub width_scale_denominator: c_int,
    pub enabled: c_int,
}

#[derive(Clone)]
#[repr(C)]
pub struct Rav1dFrameHeader_super_res {
    pub width_scale_denominator: c_int,
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
            enabled: enabled as c_int,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_tiling {
    pub uniform: c_int,
    pub n_bytes: c_uint,
    pub min_log2_cols: c_int,
    pub max_log2_cols: c_int,
    pub log2_cols: c_int,
    pub cols: c_int,
    pub min_log2_rows: c_int,
    pub max_log2_rows: c_int,
    pub log2_rows: c_int,
    pub rows: c_int,
    pub col_start_sb: [u16; DAV1D_MAX_TILE_COLS + 1],
    pub row_start_sb: [u16; DAV1D_MAX_TILE_ROWS + 1],
    pub update: c_int,
}

#[derive(Clone)]
#[repr(C)]
pub struct Rav1dFrameHeader_tiling {
    pub uniform: c_int,
    pub n_bytes: c_uint,
    pub min_log2_cols: c_int,
    pub max_log2_cols: c_int,
    pub log2_cols: c_int,
    pub cols: c_int,
    pub min_log2_rows: c_int,
    pub max_log2_rows: c_int,
    pub log2_rows: c_int,
    pub rows: c_int,
    pub col_start_sb: [u16; RAV1D_MAX_TILE_COLS + 1],
    pub row_start_sb: [u16; RAV1D_MAX_TILE_ROWS + 1],
    pub update: c_int,
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
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_quant {
    pub yac: c_int,
    pub ydc_delta: c_int,
    pub udc_delta: c_int,
    pub uac_delta: c_int,
    pub vdc_delta: c_int,
    pub vac_delta: c_int,
    pub qm: c_int,
    pub qm_y: c_int,
    pub qm_u: c_int,
    pub qm_v: c_int,
}

#[derive(Clone)]
#[repr(C)]
pub struct Rav1dFrameHeader_quant {
    pub yac: c_int,
    pub ydc_delta: c_int,
    pub udc_delta: c_int,
    pub uac_delta: c_int,
    pub vdc_delta: c_int,
    pub vac_delta: c_int,
    pub qm: c_int,
    pub qm_y: c_int,
    pub qm_u: c_int,
    pub qm_v: c_int,
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
            qm,
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
            qm,
            qm_y,
            qm_u,
            qm_v,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_segmentation {
    pub enabled: c_int,
    pub update_map: c_int,
    pub temporal: c_int,
    pub update_data: c_int,
    pub seg_data: Dav1dSegmentationDataSet,
    pub lossless: [c_int; DAV1D_MAX_SEGMENTS as usize],
    pub qidx: [c_int; DAV1D_MAX_SEGMENTS as usize],
}

#[derive(Clone)]
#[repr(C)]
pub struct Rav1dFrameHeader_segmentation {
    pub enabled: c_int,
    pub update_map: c_int,
    pub temporal: c_int,
    pub update_data: c_int,
    pub seg_data: Rav1dSegmentationDataSet,
    pub lossless: [c_int; RAV1D_MAX_SEGMENTS as usize],
    pub qidx: [c_int; RAV1D_MAX_SEGMENTS as usize],
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
            enabled,
            update_map,
            temporal,
            update_data,
            seg_data: seg_data.into(),
            lossless,
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
            enabled,
            update_map,
            temporal,
            update_data,
            seg_data: seg_data.into(),
            lossless,
            qidx,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_delta_q {
    pub present: c_int,
    pub res_log2: c_int,
}

#[derive(Clone)]
#[repr(C)]
pub struct Rav1dFrameHeader_delta_q {
    pub present: c_int,
    pub res_log2: c_int,
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
    pub present: c_int,
    pub res_log2: c_int,
    pub multi: c_int,
}

#[derive(Clone)]
#[repr(C)]
pub struct Rav1dFrameHeader_delta_lf {
    pub present: c_int,
    pub res_log2: c_int,
    pub multi: c_int,
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

#[derive(Clone)]
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
    pub level_y: [c_int; 2],
    pub level_u: c_int,
    pub level_v: c_int,
    pub mode_ref_delta_enabled: c_int,
    pub mode_ref_delta_update: c_int,
    pub mode_ref_deltas: Dav1dLoopfilterModeRefDeltas,
    pub sharpness: c_int,
}

#[derive(Clone)]
#[repr(C)]
pub struct Rav1dFrameHeader_loopfilter {
    pub level_y: [c_int; 2],
    pub level_u: c_int,
    pub level_v: c_int,
    pub mode_ref_delta_enabled: c_int,
    pub mode_ref_delta_update: c_int,
    pub mode_ref_deltas: Rav1dLoopfilterModeRefDeltas,
    pub sharpness: c_int,
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
    pub damping: c_int,
    pub n_bits: c_int,
    pub y_strength: [c_int; DAV1D_MAX_CDEF_STRENGTHS],
    pub uv_strength: [c_int; DAV1D_MAX_CDEF_STRENGTHS],
}

#[derive(Clone)]
#[repr(C)]
pub struct Rav1dFrameHeader_cdef {
    pub damping: c_int,
    pub n_bits: c_int,
    pub y_strength: [c_int; RAV1D_MAX_CDEF_STRENGTHS],
    pub uv_strength: [c_int; RAV1D_MAX_CDEF_STRENGTHS],
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
    pub unit_size: [c_int; 2],
}

#[derive(Clone)]
#[repr(C)]
pub struct Rav1dFrameHeader_restoration {
    pub r#type: [Rav1dRestorationType; 3],
    pub unit_size: [c_int; 2],
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
            r#type: r#type.map(|e| e as u8),
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
    pub frame_offset: c_int,
    pub temporal_id: c_int,
    pub spatial_id: c_int,
    pub show_existing_frame: c_int,
    pub existing_frame_idx: c_int,
    pub frame_id: c_int,
    pub frame_presentation_delay: c_int,
    pub show_frame: c_int,
    pub showable_frame: c_int,
    pub error_resilient_mode: c_int,
    pub disable_cdf_update: c_int,
    pub allow_screen_content_tools: c_int,
    pub force_integer_mv: c_int,
    pub frame_size_override: c_int,
    pub primary_ref_frame: c_int,
    pub buffer_removal_time_present: c_int,
    pub operating_points: [Dav1dFrameHeaderOperatingPoint; DAV1D_MAX_OPERATING_POINTS],
    pub refresh_frame_flags: c_int,
    pub render_width: c_int,
    pub render_height: c_int,
    pub super_res: Dav1dFrameHeader_super_res,
    pub have_render_size: c_int,
    pub allow_intrabc: c_int,
    pub frame_ref_short_signaling: c_int,
    pub refidx: [c_int; DAV1D_REFS_PER_FRAME],
    pub hp: c_int,
    pub subpel_filter_mode: Dav1dFilterMode,
    pub switchable_motion_mode: c_int,
    pub use_ref_frame_mvs: c_int,
    pub refresh_context: c_int,
    pub tiling: Dav1dFrameHeader_tiling,
    pub quant: Dav1dFrameHeader_quant,
    pub segmentation: Dav1dFrameHeader_segmentation,
    pub delta: Dav1dFrameHeader_delta,
    pub all_lossless: c_int,
    pub loopfilter: Dav1dFrameHeader_loopfilter,
    pub cdef: Dav1dFrameHeader_cdef,
    pub restoration: Dav1dFrameHeader_restoration,
    pub txfm_mode: Dav1dTxfmMode,
    pub switchable_comp_refs: c_int,
    pub skip_mode_allowed: c_int,
    pub skip_mode_enabled: c_int,
    pub skip_mode_refs: [c_int; 2],
    pub warp_motion: c_int,
    pub reduced_txtp_set: c_int,
    pub gmv: [Dav1dWarpedMotionParams; DAV1D_REFS_PER_FRAME],
}

#[derive(Clone)]
#[repr(C)]
pub struct Rav1dFrameSize {
    pub width: [c_int; 2],
    pub height: c_int,
    pub render_width: c_int,
    pub render_height: c_int,
    pub super_res: Rav1dFrameHeader_super_res,
    pub have_render_size: c_int,
}

#[derive(Clone)]
#[repr(C)]
pub struct Rav1dFrameSkipMode {
    pub allowed: c_int,
    pub enabled: c_int,
    pub refs: [c_int; 2],
}

#[derive(Clone)]
#[repr(C)]
pub struct Rav1dFrameHeader {
    pub size: Rav1dFrameSize,
    pub film_grain: Rav1dFrameHeader_film_grain,
    pub frame_type: Rav1dFrameType,
    pub frame_offset: c_int,
    pub temporal_id: c_int,
    pub spatial_id: c_int,
    pub show_existing_frame: c_int,
    pub existing_frame_idx: c_int,
    pub frame_id: c_int,
    pub frame_presentation_delay: c_int,
    pub show_frame: c_int,
    pub showable_frame: c_int,
    pub error_resilient_mode: c_int,
    pub disable_cdf_update: c_int,
    pub allow_screen_content_tools: bool,
    pub force_integer_mv: bool,
    pub frame_size_override: bool,
    pub primary_ref_frame: c_int,
    pub buffer_removal_time_present: c_int,
    pub operating_points: [Rav1dFrameHeaderOperatingPoint; RAV1D_MAX_OPERATING_POINTS],
    pub refresh_frame_flags: c_int,
    pub allow_intrabc: bool,
    pub frame_ref_short_signaling: c_int,
    pub refidx: [c_int; RAV1D_REFS_PER_FRAME],
    pub hp: bool,
    pub subpel_filter_mode: Rav1dFilterMode,
    pub switchable_motion_mode: c_int,
    pub use_ref_frame_mvs: c_int,
    pub refresh_context: c_int,
    pub tiling: Rav1dFrameHeader_tiling,
    pub quant: Rav1dFrameHeader_quant,
    pub segmentation: Rav1dFrameHeader_segmentation,
    pub delta: Rav1dFrameHeader_delta,
    pub all_lossless: bool,
    pub loopfilter: Rav1dFrameHeader_loopfilter,
    pub cdef: Rav1dFrameHeader_cdef,
    pub restoration: Rav1dFrameHeader_restoration,
    pub txfm_mode: Rav1dTxfmMode,
    pub switchable_comp_refs: c_int,
    pub skip_mode: Rav1dFrameSkipMode,
    pub warp_motion: c_int,
    pub reduced_txtp_set: c_int,
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
                have_render_size,
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
                allowed: skip_mode_allowed,
                enabled: skip_mode_enabled,
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
            have_render_size,
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
            skip_mode_allowed,
            skip_mode_enabled,
            skip_mode_refs,
            warp_motion,
            reduced_txtp_set,
            gmv: gmv.map(|rust| rust.into()),
        }
    }
}
