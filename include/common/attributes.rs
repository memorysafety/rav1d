#[inline]
pub unsafe extern "C" fn ctz(mask: libc::c_uint) -> libc::c_int {
    return mask.trailing_zeros() as i32;
}
