#[inline]
pub unsafe extern "C" fn get_upsample(
    wh: libc::c_int,
    angle: libc::c_int,
    is_sm: libc::c_int,
) -> libc::c_int {
    return (angle < 40 && wh <= 16 >> is_sm) as libc::c_int;
}
