use crate::include::common::attributes::clz;
use crate::include::common::attributes::clzll;
use crate::include::stdint::int64_t;
use crate::include::stdint::uint64_t;

#[inline]
pub fn imax(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}

#[inline]
pub fn imin(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}

#[inline]
pub fn umin(a: libc::c_uint, b: libc::c_uint) -> libc::c_uint {
    return if a < b { a } else { b };
}

#[inline]
pub fn iclip(
    v: libc::c_int,
    min: libc::c_int,
    max: libc::c_int,
) -> libc::c_int {
    return if v < min { min } else if v > max { max } else { v };
}

#[inline]
pub fn iclip_u8(v: libc::c_int) -> libc::c_int {
    return iclip(v, 0 as libc::c_int, 255 as libc::c_int);
}

#[inline]
pub fn apply_sign(v: libc::c_int, s: libc::c_int) -> libc::c_int {
    return if s < 0 as libc::c_int { -v } else { v };
}

#[inline]
pub fn apply_sign64(v: libc::c_int, s: int64_t) -> libc::c_int {
    return if s < 0 { -v } else { v };
}

#[inline]
pub fn ulog2(v: libc::c_uint) -> libc::c_int {
    return 31 as libc::c_int - clz(v);
}

#[inline]
pub fn u64log2(v: uint64_t) -> libc::c_int {
    return 63 as libc::c_int - clzll(v as libc::c_ulonglong);
}

#[inline]
pub fn inv_recenter(r: libc::c_uint, v: libc::c_uint) -> libc::c_uint {
    if v > r << 1 as libc::c_int {
        return v
    } else if v & 1 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        return (v >> 1 as libc::c_int).wrapping_add(r)
    } else {
        return r
            .wrapping_sub(
                v.wrapping_add(1 as libc::c_int as libc::c_uint) >> 1 as libc::c_int,
            )
    };
}
