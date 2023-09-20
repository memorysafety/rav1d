use std::ffi::c_char;
use std::ffi::c_uint;
use std::ffi::c_void;

pub use crate::src::internal::Dav1dContext;
pub use crate::src::r#ref::Dav1dRef;

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dLogger {
    pub cookie: *mut c_void,
    pub callback:
        Option<unsafe extern "C" fn(*mut c_void, *const c_char, ::core::ffi::VaList) -> ()>,
}

pub type Dav1dInloopFilterType = c_uint;
pub const DAV1D_INLOOPFILTER_ALL: Dav1dInloopFilterType = 7;
pub const DAV1D_INLOOPFILTER_RESTORATION: Dav1dInloopFilterType = 4;
pub const DAV1D_INLOOPFILTER_CDEF: Dav1dInloopFilterType = 2;
pub const DAV1D_INLOOPFILTER_DEBLOCK: Dav1dInloopFilterType = 1;
pub const DAV1D_INLOOPFILTER_NONE: Dav1dInloopFilterType = 0;

pub type Dav1dDecodeFrameType = c_uint;
pub const DAV1D_DECODEFRAMETYPE_KEY: Dav1dDecodeFrameType = 3;
pub const DAV1D_DECODEFRAMETYPE_INTRA: Dav1dDecodeFrameType = 2;
pub const DAV1D_DECODEFRAMETYPE_REFERENCE: Dav1dDecodeFrameType = 1;
pub const DAV1D_DECODEFRAMETYPE_ALL: Dav1dDecodeFrameType = 0;

pub type Dav1dEventFlags = c_uint;
pub const DAV1D_EVENT_FLAG_NEW_OP_PARAMS_INFO: Dav1dEventFlags = 2;
pub const DAV1D_EVENT_FLAG_NEW_SEQUENCE: Dav1dEventFlags = 1;
