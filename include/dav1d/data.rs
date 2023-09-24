use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::dav1d::dav1d::Dav1dRef;
use std::ptr;

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dData {
    pub data: *const u8,
    pub sz: usize,
    pub r#ref: *mut Dav1dRef,
    pub m: Dav1dDataProps,
}

impl Default for Dav1dData {
    fn default() -> Self {
        Self {
            data: ptr::null(),
            sz: Default::default(),
            r#ref: ptr::null_mut(),
            m: Default::default(),
        }
    }
}
