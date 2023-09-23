use crate::src::env::BlockContext;
use crate::src::levels::IntraPredMode;
use crate::src::levels::DC_128_PRED;
use crate::src::levels::DC_PRED;
use crate::src::levels::HOR_PRED;
use crate::src::levels::LEFT_DC_PRED;
use crate::src::levels::N_INTRA_PRED_MODES;
use crate::src::levels::PAETH_PRED;
use crate::src::levels::SMOOTH_H_PRED;
use crate::src::levels::SMOOTH_PRED;
use crate::src::levels::SMOOTH_V_PRED;
use crate::src::levels::TOP_DC_PRED;
use crate::src::levels::VERT_PRED;
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

// TODO(kkysen) make private once module is fully deduplicated
pub(super) static av1_mode_conv: [[[u8; 2]; 2]; N_INTRA_PRED_MODES] = [
    [[DC_128_PRED, TOP_DC_PRED], [LEFT_DC_PRED, DC_PRED]],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[DC_128_PRED, VERT_PRED], [HOR_PRED, PAETH_PRED]],
];

// TODO(kkysen) make private once module is fully deduplicated
pub(super) static av1_mode_to_angle_map: [u8; 8] = [90, 180, 45, 135, 113, 157, 203, 67];
