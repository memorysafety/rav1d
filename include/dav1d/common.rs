use crate::include::stdint::uint8_t;
use crate::src::r#ref::Dav1dRef;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dUserData {
    pub data: *const uint8_t,
    pub ref_0: *mut Dav1dRef,
}
