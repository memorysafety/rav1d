use crate::src::env::BlockContext;
use crate::src::levels::IntraPredMode;
use crate::src::levels::SMOOTH_H_PRED;
use crate::src::levels::SMOOTH_PRED;
use crate::src::levels::SMOOTH_V_PRED;

#[inline]
pub unsafe extern "C" fn sm_flag(b: *const BlockContext, idx: libc::c_int) -> libc::c_int {
    if (*b).intra[idx as usize] == 0 {
        return 0 as libc::c_int;
    }
    let m: IntraPredMode = (*b).mode[idx as usize] as IntraPredMode;
    return if m as libc::c_uint == SMOOTH_PRED as libc::c_int as libc::c_uint
        || m as libc::c_uint == SMOOTH_H_PRED as libc::c_int as libc::c_uint
        || m as libc::c_uint == SMOOTH_V_PRED as libc::c_int as libc::c_uint
    {
        512 as libc::c_int
    } else {
        0 as libc::c_int
    };
}

#[inline]
pub unsafe extern "C" fn sm_uv_flag(
    b: *const BlockContext,
    idx: libc::c_int,
) -> libc::c_int {
    let m: IntraPredMode = (*b).uvmode[idx as usize] as IntraPredMode;
    return if m as libc::c_uint == SMOOTH_PRED as libc::c_int as libc::c_uint
        || m as libc::c_uint == SMOOTH_H_PRED as libc::c_int as libc::c_uint
        || m as libc::c_uint == SMOOTH_V_PRED as libc::c_int as libc::c_uint
    {
        512 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
