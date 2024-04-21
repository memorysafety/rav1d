use std::ffi::c_int;
use std::ffi::c_void;
use std::sync::atomic::AtomicI32;

#[repr(C)]
pub struct Rav1dRef {
    pub(crate) data: *mut c_void,
    pub(crate) const_data: *const c_void,
    pub(crate) ref_cnt: AtomicI32,
    pub(crate) free_ref: c_int,
    pub(crate) free_callback: Option<unsafe extern "C" fn(*const u8, *mut c_void) -> ()>,
    pub(crate) user_data: *mut c_void,
}
