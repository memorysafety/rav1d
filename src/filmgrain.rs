use crate::include::stdint::uint64_t;

#[inline]
pub unsafe extern "C" fn get_random_number(
    bits: libc::c_int,
    state: *mut libc::c_uint,
) -> libc::c_int {
    let r: libc::c_int = *state as libc::c_int;
    let mut bit: libc::c_uint = ((r >> 0 as libc::c_int ^ r >> 1 as libc::c_int
        ^ r >> 3 as libc::c_int ^ r >> 12 as libc::c_int) & 1 as libc::c_int)
        as libc::c_uint;
    *state = (r >> 1 as libc::c_int) as libc::c_uint | bit << 15 as libc::c_int;
    return (*state >> 16 as libc::c_int - bits
        & (((1 as libc::c_int) << bits) - 1 as libc::c_int) as libc::c_uint)
        as libc::c_int;
}

#[inline]
pub unsafe extern "C" fn round2(x: libc::c_int, shift: uint64_t) -> libc::c_int {
    return x + ((1 as libc::c_int) << shift >> 1 as libc::c_int) >> shift;
}
