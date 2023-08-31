use std::ptr;

use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::stddef::size_t;
use crate::include::stdint::uint8_t;
use crate::src::r#ref::Dav1dRef;

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dData {
    pub data: *const uint8_t,
    pub sz: size_t,
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
