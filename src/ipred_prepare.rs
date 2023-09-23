use crate::src::env::BlockContext;
use crate::src::levels::IntraPredMode;
use crate::src::levels::DC_128_PRED;
use crate::src::levels::DC_PRED;
use crate::src::levels::HOR_PRED;
use crate::src::levels::LEFT_DC_PRED;
use crate::src::levels::N_IMPL_INTRA_PRED_MODES;
use crate::src::levels::N_INTRA_PRED_MODES;
use crate::src::levels::PAETH_PRED;
use crate::src::levels::SMOOTH_H_PRED;
use crate::src::levels::SMOOTH_PRED;
use crate::src::levels::SMOOTH_V_PRED;
use crate::src::levels::TOP_DC_PRED;
use crate::src::levels::VERT_PRED;
use c2rust_bitfields::BitfieldStruct;
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

// TODO(kkysen) make private once module is fully deduplicated
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub(super) struct av1_intra_prediction_edge {
    #[bitfield(name = "needs_left", ty = "u8", bits = "0..=0")]
    #[bitfield(name = "needs_top", ty = "u8", bits = "1..=1")]
    #[bitfield(name = "needs_topleft", ty = "u8", bits = "2..=2")]
    #[bitfield(name = "needs_topright", ty = "u8", bits = "3..=3")]
    #[bitfield(name = "needs_bottomleft", ty = "u8", bits = "4..=4")]
    pub needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [u8; 1],
}

// TODO(kkysen) make private once module is fully deduplicated
pub(super) static mut av1_intra_prediction_edges: [av1_intra_prediction_edge;
    N_IMPL_INTRA_PRED_MODES] = [av1_intra_prediction_edge {
    needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
}; N_IMPL_INTRA_PRED_MODES];

unsafe extern "C" fn run_static_initializers() {
    av1_intra_prediction_edges = [
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as c_int as u8);
            init.set_needs_top(1 as c_int as u8);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(0);
            init.set_needs_top(1 as c_int as u8);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as c_int as u8);
            init.set_needs_top(0);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as c_int as u8);
            init.set_needs_top(0);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(0);
            init.set_needs_top(1 as c_int as u8);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(0 as c_int as u8);
            init.set_needs_top(0);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(0);
            init.set_needs_top(1 as c_int as u8);
            init.set_needs_topleft(1 as c_int as u8);
            init.set_needs_topright(1 as c_int as u8);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as c_int as u8);
            init.set_needs_top(1 as c_int as u8);
            init.set_needs_topleft(1 as c_int as u8);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as c_int as u8);
            init.set_needs_top(0);
            init.set_needs_topleft(1 as c_int as u8);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(1 as c_int as u8);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as c_int as u8);
            init.set_needs_top(1 as c_int as u8);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as c_int as u8);
            init.set_needs_top(1 as c_int as u8);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as c_int as u8);
            init.set_needs_top(1 as c_int as u8);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as c_int as u8);
            init.set_needs_top(1 as c_int as u8);
            init.set_needs_topleft(1 as c_int as u8);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as c_int as u8);
            init.set_needs_top(1 as c_int as u8);
            init.set_needs_topleft(1 as c_int as u8);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
    ];
}

#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
