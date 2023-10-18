use crate::include::dav1d::dav1d::Dav1dRef;
use std::ptr;

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dUserData {
    pub data: *const u8,
    pub r#ref: *mut Dav1dRef,
}

impl Default for Dav1dUserData {
    fn default() -> Self {
        Self {
            data: ptr::null(),
            r#ref: ptr::null_mut(),
        }
    }
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct Dav1dDataProps {
    pub timestamp: i64,
    pub duration: i64,
    pub offset: libc::off_t,
    pub size: usize,
    pub user_data: Dav1dUserData,
}
