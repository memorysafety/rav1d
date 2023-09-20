use crate::src::env::BlockContext;
use crate::src::levels::IntraPredMode;
use crate::src::levels::SMOOTH_H_PRED;
use crate::src::levels::SMOOTH_PRED;
use crate::src::levels::SMOOTH_V_PRED;
use std::ffi::c_int;
use std::ffi::c_uint;

#[inline]
pub unsafe extern "C" fn sm_flag(b: *const BlockContext, idx: c_int) -> c_int {
    if (*b).intra[idx as usize] == 0 {
        return 0 as c_int;
    }
    let m: IntraPredMode = (*b).mode[idx as usize] as IntraPredMode;
    return if m as c_uint == SMOOTH_PRED as c_int as c_uint
        || m as c_uint == SMOOTH_H_PRED as c_int as c_uint
        || m as c_uint == SMOOTH_V_PRED as c_int as c_uint
    {
        512 as c_int
    } else {
        0 as c_int
    };
}

#[inline]
pub unsafe extern "C" fn sm_uv_flag(b: *const BlockContext, idx: c_int) -> c_int {
    let m: IntraPredMode = (*b).uvmode[idx as usize] as IntraPredMode;
    return if m as c_uint == SMOOTH_PRED as c_int as c_uint
        || m as c_uint == SMOOTH_H_PRED as c_int as c_uint
        || m as c_uint == SMOOTH_V_PRED as c_int as c_uint
    {
        512 as c_int
    } else {
        0 as c_int
    };
}
