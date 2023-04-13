#[inline]
pub unsafe extern "C" fn get_upsample(
    wh: libc::c_int,
    angle: libc::c_int,
    is_sm: libc::c_int,
) -> libc::c_int {
    return (angle < 40 as libc::c_int && wh <= 16 as libc::c_int >> is_sm)
        as libc::c_int;
}
