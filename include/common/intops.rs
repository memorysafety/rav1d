use std::ffi::{c_int, c_uint, c_ulonglong};

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
pub fn clip<T, U>(v: T, min: U, max: U) -> U
where
    T: Copy + Ord + TryInto<U>,
    U: Copy + Ord + Into<T>,
{
    assert!(min <= max);
    if v < min.into() {
        min
    } else if v > max.into() {
        max
    } else {
        let v = v.try_into();
        // # Safety
        // `min <= v <= max`, `min: U`, `max: U`,
        // so `v` must be in `U`, too.
        //
        // Note that `v.try_into().unwrap()` is not always optimized out.
        unsafe { v.unwrap_unchecked() }
    }
}

#[inline]
pub fn clip_u8<T>(v: T) -> u8
where
    T: Copy + Ord + From<u8> + TryInto<u8>,
{
    clip(v, u8::MIN, u8::MAX)
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
