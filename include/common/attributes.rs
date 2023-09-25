use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_ulonglong;

#[inline]
pub fn ctz(mask: c_uint) -> c_int {
    mask.trailing_zeros() as i32
}

#[inline]
pub fn clz(mask: c_uint) -> c_int {
    mask.leading_zeros() as i32
}

#[inline]
pub fn clzll(mask: c_ulonglong) -> c_int {
    mask.leading_zeros() as i32
}
