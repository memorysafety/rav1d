use crate::include::stdint::uint64_t;

#[inline]
pub unsafe extern "C" fn get_random_number(
    bits: libc::c_int,
    state: *mut libc::c_uint,
) -> libc::c_int {
    let r = *state as libc::c_int;
    let mut bit: libc::c_uint = ((r >> 0 ^ r >> 1 ^ r >> 3 ^ r >> 12) & 1) as libc::c_uint;
    *state = (r >> 1) as libc::c_uint | bit << 15;
    return (*state >> 16 - bits & (((1 as libc::c_int) << bits) - 1) as libc::c_uint)
        as libc::c_int;
}

#[inline]
pub unsafe extern "C" fn round2(x: libc::c_int, shift: uint64_t) -> libc::c_int {
    return x + ((1 as libc::c_int) << shift >> 1) >> shift;
}
