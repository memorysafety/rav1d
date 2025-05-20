use crate::include::common::attributes::clz;
use crate::include::common::attributes::clzll;
use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::ToPrimitive;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_ulonglong;

/// # Safety
///
/// `U: Into<T>` and `T: TryInto<U>` must be well-formed
/// such that for all `u: U`, `u.into().try_into() == Ok(u)`.
#[inline]
pub fn clip<T, U>(v: T, min: U, max: U) -> U
where
    T: Copy + Ord + TryInto<U> + ToPrimitive<U>,
    U: Copy + Ord + Into<T>,
{
    debug_assert!(min <= max);
    if v < min.into() {
        min
    } else if v > max.into() {
        max
    } else {
        // Note that `v.try_into().unwrap()` is not always optimized out.
        // We use `.as_()`, a truncating cast, here instead of
        // `v.try_into().unwrap()`, which doesn't always optimized out,
        // or `unsafe { v.try_into().unwrap_unchecked() }`, which is unsafe
        // and depends on correct `{Try,}{From,Into}` `impl`s
        // and `assert!(min <= max)`, which may not always get optimized out.
        v.as_()
    }
}

#[inline]
pub fn clip_u8<T>(v: T) -> u8
where
    T: Copy + Ord + From<u8> + TryInto<u8> + ToPrimitive<u8>,
{
    clip(v, u8::MIN, u8::MAX)
}

#[inline]
pub fn iclip(v: c_int, min: c_int, max: c_int) -> c_int {
    clip(v, min, max)
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
