use crate::include::dav1d::picture::Dav1dPicAllocator;
use crate::include::dav1d::picture::Rav1dPicAllocator;
use crate::src::internal::Rav1dContext;
use crate::src::r#ref::Rav1dRef;
use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_void;

pub type Dav1dContext = Rav1dContext;
pub type Dav1dRef = Rav1dRef;

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dLogger {
    pub cookie: *mut c_void,
    pub callback:
        Option<unsafe extern "C" fn(*mut c_void, *const c_char, ::core::ffi::VaList) -> ()>,
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dLogger {
    pub cookie: *mut c_void,
    pub callback:
        Option<unsafe extern "C" fn(*mut c_void, *const c_char, ::core::ffi::VaList) -> ()>,
}

impl From<Dav1dLogger> for Rav1dLogger {
    fn from(value: Dav1dLogger) -> Self {
        let Dav1dLogger { cookie, callback } = value;
        Self { cookie, callback }
    }
}

impl From<Rav1dLogger> for Dav1dLogger {
    fn from(value: Rav1dLogger) -> Self {
        let Rav1dLogger { cookie, callback } = value;
        Self { cookie, callback }
    }
}

pub type Dav1dInloopFilterType = c_uint;
pub const DAV1D_INLOOPFILTER_ALL: Dav1dInloopFilterType = 7;
pub const DAV1D_INLOOPFILTER_RESTORATION: Dav1dInloopFilterType = 4;
pub const DAV1D_INLOOPFILTER_CDEF: Dav1dInloopFilterType = 2;
pub const DAV1D_INLOOPFILTER_DEBLOCK: Dav1dInloopFilterType = 1;
pub const DAV1D_INLOOPFILTER_NONE: Dav1dInloopFilterType = 0;

pub(crate) type Rav1dInloopFilterType = c_uint;
pub(crate) const RAV1D_INLOOPFILTER_ALL: Rav1dInloopFilterType = DAV1D_INLOOPFILTER_ALL;
pub(crate) const RAV1D_INLOOPFILTER_RESTORATION: Rav1dInloopFilterType =
    DAV1D_INLOOPFILTER_RESTORATION;
pub(crate) const RAV1D_INLOOPFILTER_CDEF: Rav1dInloopFilterType = DAV1D_INLOOPFILTER_CDEF;
pub(crate) const RAV1D_INLOOPFILTER_DEBLOCK: Rav1dInloopFilterType = DAV1D_INLOOPFILTER_DEBLOCK;
pub(crate) const _RAV1D_INLOOPFILTER_NONE: Rav1dInloopFilterType = DAV1D_INLOOPFILTER_NONE;

pub type Dav1dDecodeFrameType = c_uint;
pub const DAV1D_DECODEFRAMETYPE_KEY: Dav1dDecodeFrameType = 3;
pub const DAV1D_DECODEFRAMETYPE_INTRA: Dav1dDecodeFrameType = 2;
pub const DAV1D_DECODEFRAMETYPE_REFERENCE: Dav1dDecodeFrameType = 1;
pub const DAV1D_DECODEFRAMETYPE_ALL: Dav1dDecodeFrameType = 0;

pub(crate) type Rav1dDecodeFrameType = c_uint;
pub(crate) const RAV1D_DECODEFRAMETYPE_KEY: Rav1dDecodeFrameType = DAV1D_DECODEFRAMETYPE_KEY;
pub(crate) const RAV1D_DECODEFRAMETYPE_INTRA: Rav1dDecodeFrameType = DAV1D_DECODEFRAMETYPE_INTRA;
pub(crate) const RAV1D_DECODEFRAMETYPE_REFERENCE: Rav1dDecodeFrameType =
    DAV1D_DECODEFRAMETYPE_REFERENCE;
pub(crate) const RAV1D_DECODEFRAMETYPE_ALL: Rav1dDecodeFrameType = DAV1D_DECODEFRAMETYPE_ALL;

pub type Dav1dEventFlags = c_uint;
pub const DAV1D_EVENT_FLAG_NEW_OP_PARAMS_INFO: Dav1dEventFlags = 2;
pub const DAV1D_EVENT_FLAG_NEW_SEQUENCE: Dav1dEventFlags = 1;

pub(crate) type Rav1dEventFlags = c_uint;
pub(crate) const RAV1D_EVENT_FLAG_NEW_OP_PARAMS_INFO: Rav1dEventFlags =
    DAV1D_EVENT_FLAG_NEW_OP_PARAMS_INFO;
pub(crate) const RAV1D_EVENT_FLAG_NEW_SEQUENCE: Rav1dEventFlags = DAV1D_EVENT_FLAG_NEW_SEQUENCE;

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
    pub operating_point: c_int,
    pub all_layers: bool,
    pub frame_size_limit: c_uint,
    pub allocator: Rav1dPicAllocator,
    pub logger: Rav1dLogger,
    pub strict_std_compliance: bool,
    pub output_invisible_frames: c_int,
    pub inloop_filters: Rav1dInloopFilterType,
    pub decode_frame_type: Rav1dDecodeFrameType,
}

impl From<Dav1dSettings> for Rav1dSettings {
    fn from(value: Dav1dSettings) -> Self {
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
        Self {
            n_threads,
            max_frame_delay,
            apply_grain: apply_grain != 0,
            operating_point,
            all_layers: all_layers != 0,
            frame_size_limit,
            allocator: allocator.into(),
            logger: logger.into(),
            strict_std_compliance: strict_std_compliance != 0,
            output_invisible_frames,
            inloop_filters,
            decode_frame_type,
        }
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
            operating_point,
            all_layers: all_layers as c_int,
            frame_size_limit,
            allocator: allocator.into(),
            logger: logger.into(),
            strict_std_compliance: strict_std_compliance as c_int,
            output_invisible_frames,
            inloop_filters,
            decode_frame_type,
            reserved: Default::default(),
        }
    }
}
