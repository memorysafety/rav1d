use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::stddef::size_t;
use crate::include::stdint::uint8_t;
use crate::src::r#ref::Dav1dRef;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dData {
    pub data: *const uint8_t,
    pub sz: size_t,
    pub r#ref: *mut Dav1dRef,
    pub m: Dav1dDataProps,
}
