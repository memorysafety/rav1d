use crate::include::dav1d::picture::Dav1dPicAllocator;
use crate::include::dav1d::picture::Rav1dPicAllocator;
use crate::src::c_arc::RawArc;
use crate::src::error::Rav1dError;
use crate::src::internal::Rav1dContext;
pub use crate::src::log::Dav1dLogger;
use crate::src::log::Rav1dLogger;
use bitflags::bitflags;
use std::ffi::c_int;
use std::ffi::c_uint;
use strum::FromRepr;

pub type Dav1dContext = RawArc<Rav1dContext>;

pub type Dav1dRef = ();

pub type Dav1dInloopFilterType = c_uint;
pub const DAV1D_INLOOPFILTER_ALL: Dav1dInloopFilterType =
    Rav1dInloopFilterType::all().bits() as Dav1dInloopFilterType;
pub const DAV1D_INLOOPFILTER_NONE: Dav1dInloopFilterType =
    Rav1dInloopFilterType::empty().bits() as Dav1dInloopFilterType;
pub const DAV1D_INLOOPFILTER_DEBLOCK: Dav1dInloopFilterType =
    Rav1dInloopFilterType::DEBLOCK.bits() as Dav1dInloopFilterType;
pub const DAV1D_INLOOPFILTER_CDEF: Dav1dInloopFilterType =
    Rav1dInloopFilterType::CDEF.bits() as Dav1dInloopFilterType;
pub const DAV1D_INLOOPFILTER_RESTORATION: Dav1dInloopFilterType =
    Rav1dInloopFilterType::RESTORATION.bits() as Dav1dInloopFilterType;

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub(crate) struct Rav1dInloopFilterType: u8 {
        const DEBLOCK = 1 << 1;
        const CDEF = 1 << 2;
        const RESTORATION = 1 << 3;
    }
}

impl From<Rav1dInloopFilterType> for Dav1dInloopFilterType {
    fn from(value: Rav1dInloopFilterType) -> Self {
        value.bits().into()
    }
}

impl From<Dav1dInloopFilterType> for Rav1dInloopFilterType {
    fn from(value: Dav1dInloopFilterType) -> Self {
        Self::from_bits_retain(value as u8)
    }
}

pub type Dav1dDecodeFrameType = c_uint;
pub const DAV1D_DECODEFRAMETYPE_ALL: Dav1dDecodeFrameType =
    Rav1dDecodeFrameType::All as Dav1dDecodeFrameType;
pub const DAV1D_DECODEFRAMETYPE_REFERENCE: Dav1dDecodeFrameType =
    Rav1dDecodeFrameType::Reference as Dav1dDecodeFrameType;
pub const DAV1D_DECODEFRAMETYPE_INTRA: Dav1dDecodeFrameType =
    Rav1dDecodeFrameType::Intra as Dav1dDecodeFrameType;
pub const DAV1D_DECODEFRAMETYPE_KEY: Dav1dDecodeFrameType =
    Rav1dDecodeFrameType::Key as Dav1dDecodeFrameType;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromRepr, Default)]
pub(crate) enum Rav1dDecodeFrameType {
    /// decode and return all frames
    #[default]
    All = 0,
    /// decode and return frames referenced by other frames only
    Reference = 1,
    /// decode and return intra frames only (includes keyframes)
    Intra = 2,
    /// decode and return keyframes only
    Key = 3,
}

impl From<Rav1dDecodeFrameType> for Dav1dDecodeFrameType {
    fn from(value: Rav1dDecodeFrameType) -> Self {
        value as Self
    }
}

impl TryFrom<Dav1dDecodeFrameType> for Rav1dDecodeFrameType {
    type Error = Rav1dError;

    fn try_from(value: Dav1dDecodeFrameType) -> Result<Self, Self::Error> {
        Self::from_repr(value as usize).ok_or(Rav1dError::EINVAL)
    }
}

pub type Dav1dEventFlags = c_uint;
pub const DAV1D_EVENT_FLAG_NEW_SEQUENCE: Dav1dEventFlags =
    Rav1dEventFlags::NEW_SEQUENCE.bits() as Dav1dEventFlags;
pub const DAV1D_EVENT_FLAG_NEW_OP_PARAMS_INFO: Dav1dEventFlags =
    Rav1dEventFlags::NEW_OP_PARAMS_INFO.bits() as Dav1dEventFlags;

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub(crate) struct Rav1dEventFlags: u8 {
        /// The last returned picture contains a reference
        /// to a new [`Rav1dSequenceHeader`],
        /// either because it's the start of a new coded sequence,
        /// or the decoder was flushed before it was generated.
        ///
        /// [`Rav1dSequenceHeader`]: crate::include::dav1d::headers::Rav1dSequenceHeader
        const NEW_SEQUENCE = 1 << 0;

        /// The last returned picture contains a reference to a
        /// [`Rav1dSequenceHeader`] with new [`Rav1dSequenceHeaderOperatingParameterInfo`]
        /// for the current coded sequence.
        ///
        /// [`Rav1dSequenceHeader`]: crate::include::dav1d::headers::Rav1dSequenceHeader
        /// [`Rav1dSequenceHeaderOperatingParameterInfo`]: crate::include::dav1d::headers::Rav1dSequenceHeaderOperatingParameterInfo
        const NEW_OP_PARAMS_INFO = 1 << 1;
    }
}

impl From<Rav1dEventFlags> for Dav1dEventFlags {
    fn from(value: Rav1dEventFlags) -> Self {
        value.bits().into()
    }
}

impl From<Dav1dEventFlags> for Rav1dEventFlags {
    fn from(value: Dav1dEventFlags) -> Self {
        Self::from_bits_retain(value as u8)
    }
}

#[repr(C)]
pub struct Dav1dSettings {
    pub n_threads: c_int,
    pub max_frame_delay: c_int,
    pub apply_grain: c_int,
    pub operating_point: c_int,
    pub all_layers: c_int,
    pub frame_size_limit: c_uint,
    pub allocator: Dav1dPicAllocator,
    pub logger: Dav1dLogger,
    pub strict_std_compliance: c_int,
    pub output_invisible_frames: c_int,
    pub inloop_filters: Dav1dInloopFilterType,
    pub decode_frame_type: Dav1dDecodeFrameType,
    pub reserved: [u8; 16],
}

#[repr(C)]
pub(crate) struct Rav1dSettings {
    pub n_threads: c_int,
    pub max_frame_delay: c_int,
    pub apply_grain: bool,
    pub operating_point: u8,
    pub all_layers: bool,
    pub frame_size_limit: c_uint,
    pub allocator: Rav1dPicAllocator,
    pub logger: Option<Rav1dLogger>,
    pub strict_std_compliance: bool,
    pub output_invisible_frames: bool,
    pub inloop_filters: Rav1dInloopFilterType,
    pub decode_frame_type: Rav1dDecodeFrameType,
}

impl TryFrom<Dav1dSettings> for Rav1dSettings {
    type Error = Rav1dError;

    fn try_from(value: Dav1dSettings) -> Result<Self, Self::Error> {
        let Dav1dSettings {
            n_threads,
            max_frame_delay,
            apply_grain,
            operating_point,
            all_layers,
            frame_size_limit,
            allocator,
            logger,
            strict_std_compliance,
            output_invisible_frames,
            inloop_filters,
            decode_frame_type,
            reserved: _,
        } = value;
        Ok(Self {
            n_threads,
            max_frame_delay,
            apply_grain: apply_grain != 0,
            operating_point: operating_point.try_into().unwrap(),
            all_layers: all_layers != 0,
            frame_size_limit,
            allocator: allocator.try_into()?,
            logger: logger.into(),
            strict_std_compliance: strict_std_compliance != 0,
            output_invisible_frames: output_invisible_frames != 0,
            inloop_filters: inloop_filters.into(),
            decode_frame_type: decode_frame_type.try_into()?,
        })
    }
}

impl From<Rav1dSettings> for Dav1dSettings {
    fn from(value: Rav1dSettings) -> Self {
        let Rav1dSettings {
            n_threads,
            max_frame_delay,
            apply_grain,
            operating_point,
            all_layers,
            frame_size_limit,
            allocator,
            logger,
            strict_std_compliance,
            output_invisible_frames,
            inloop_filters,
            decode_frame_type,
        } = value;
        Self {
            n_threads,
            max_frame_delay,
            apply_grain: apply_grain as c_int,
            operating_point: operating_point.into(),
            all_layers: all_layers as c_int,
            frame_size_limit,
            allocator: allocator.into(),
            logger: logger.into(),
            strict_std_compliance: strict_std_compliance as c_int,
            output_invisible_frames: output_invisible_frames as c_int,
            inloop_filters: inloop_filters.into(),
            decode_frame_type: decode_frame_type.into(),
            reserved: Default::default(),
        }
    }
}
