use std::ffi::{c_int, c_uint, c_ulonglong};
use std::fmt::Debug;

use crate::include::common::attributes::clz;
use crate::include::common::attributes::clzll;

#[inline]
pub fn imax(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

#[inline]
pub fn imin(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
pub fn umin(a: c_uint, b: c_uint) -> c_uint {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
pub fn clip<T: Ord>(v: T, min: T, max: T) -> T {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

#[inline]
pub fn clip_u8<T>(v: T) -> u8
where
    T: Ord + From<u8> + TryInto<u8>,
    <T as TryInto<u8>>::Error: Debug,
{
    clip(v, u8::MIN.into(), u8::MAX.into()).try_into().unwrap()
}

#[inline]
pub fn iclip(v: c_int, min: c_int, max: c_int) -> c_int {
    clip(v, min, max)
}

#[inline]
pub fn iclip_u8(v: c_int) -> c_int {
    clip_u8(v).into()
}

#[inline]
pub fn apply_sign(v: c_int, s: c_int) -> c_int {
    if s < 0 {
        -v
    } else {
        v
    }
}

#[inline]
pub fn apply_sign64(v: c_int, s: i64) -> c_int {
    if s < 0 {
        -v
    } else {
        v
    }
}

#[inline]
pub fn ulog2(v: c_uint) -> c_int {
    return 31 - clz(v);
}

#[inline]
pub fn u64log2(v: u64) -> c_int {
    return 63 - clzll(v as c_ulonglong);
}

#[inline]
pub fn inv_recenter(r: c_uint, v: c_uint) -> c_uint {
    if v > r << 1 {
        v
    } else if v & 1 == 0 {
        (v >> 1).wrapping_add(r)
    } else {
        r.wrapping_sub(v.wrapping_add(1) >> 1)
    }
}
