use crate::include::stdint::uint8_t;
use ::libc;
pub type EdgeFlags = uint8_t;
pub const EDGE_I420_LEFT_HAS_BOTTOM: EdgeFlags = 32;
pub const EDGE_I422_LEFT_HAS_BOTTOM: EdgeFlags = 16;
pub const EDGE_I444_LEFT_HAS_BOTTOM: EdgeFlags = 8;
pub const EDGE_I420_TOP_HAS_RIGHT: EdgeFlags = 4;
pub const EDGE_I422_TOP_HAS_RIGHT: EdgeFlags = 2;
pub const EDGE_I444_TOP_HAS_RIGHT: EdgeFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EdgeNode {
    pub o: EdgeFlags,
    pub h: [EdgeFlags; 2],
    pub v: [EdgeFlags; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EdgeTip {
    pub node: EdgeNode,
    pub split: [EdgeFlags; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EdgeBranch {
    pub node: EdgeNode,
    pub tts: [EdgeFlags; 3],
    pub tbs: [EdgeFlags; 3],
    pub tls: [EdgeFlags; 3],
    pub trs: [EdgeFlags; 3],
    pub h4: [EdgeFlags; 4],
    pub v4: [EdgeFlags; 4],
    pub split: [*mut EdgeNode; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ModeSelMem {
    pub nwc: [*mut EdgeBranch; 3],
    pub nt: *mut EdgeTip,
}
use crate::src::levels::BlockLevel;
use crate::src::levels::BL_32X32;
use crate::src::levels::BL_64X64;

use crate::src::levels::BL_128X128;
use crate::src::levels::BL_16X16;
use crate::src::levels::BL_8X8;

unsafe fn init_edges(node: *mut EdgeNode, bl: BlockLevel, edge_flags: EdgeFlags) {
    (*node).o = edge_flags;

    if bl == BL_8X8 {
        let nt = &mut *(node as *mut EdgeTip);
        let node = &mut nt.node;

        node.h[0] = edge_flags
            | (EDGE_I444_LEFT_HAS_BOTTOM | EDGE_I422_LEFT_HAS_BOTTOM | EDGE_I420_LEFT_HAS_BOTTOM);
        node.h[1] = edge_flags
            & ((EDGE_I444_LEFT_HAS_BOTTOM | EDGE_I422_LEFT_HAS_BOTTOM | EDGE_I420_LEFT_HAS_BOTTOM)
                | EDGE_I420_TOP_HAS_RIGHT);

        node.v[0] = edge_flags
            | (EDGE_I444_TOP_HAS_RIGHT | EDGE_I422_TOP_HAS_RIGHT | EDGE_I420_TOP_HAS_RIGHT);
        node.v[1] = edge_flags
            & ((EDGE_I444_TOP_HAS_RIGHT | EDGE_I422_TOP_HAS_RIGHT | EDGE_I420_TOP_HAS_RIGHT)
                | EDGE_I420_LEFT_HAS_BOTTOM
                | EDGE_I422_LEFT_HAS_BOTTOM);

        nt.split[0] = (EDGE_I444_TOP_HAS_RIGHT | EDGE_I422_TOP_HAS_RIGHT | EDGE_I420_TOP_HAS_RIGHT)
            | (EDGE_I444_LEFT_HAS_BOTTOM | EDGE_I422_LEFT_HAS_BOTTOM | EDGE_I420_LEFT_HAS_BOTTOM);
        nt.split[1] = (edge_flags
            & (EDGE_I444_TOP_HAS_RIGHT | EDGE_I422_TOP_HAS_RIGHT | EDGE_I420_TOP_HAS_RIGHT))
            | EDGE_I422_LEFT_HAS_BOTTOM;
        nt.split[2] = edge_flags | EDGE_I444_TOP_HAS_RIGHT;
        nt.split[3] = edge_flags
            & (EDGE_I420_TOP_HAS_RIGHT | EDGE_I420_LEFT_HAS_BOTTOM | EDGE_I422_LEFT_HAS_BOTTOM);
    } else {
        let nwc = &mut *(node as *mut EdgeBranch);
        let node = &mut nwc.node;

        node.h[0] = edge_flags
            | (EDGE_I444_LEFT_HAS_BOTTOM | EDGE_I422_LEFT_HAS_BOTTOM | EDGE_I420_LEFT_HAS_BOTTOM);
        node.h[1] = edge_flags
            & (EDGE_I444_LEFT_HAS_BOTTOM | EDGE_I422_LEFT_HAS_BOTTOM | EDGE_I420_LEFT_HAS_BOTTOM);

        node.v[0] = edge_flags
            | (EDGE_I444_TOP_HAS_RIGHT | EDGE_I422_TOP_HAS_RIGHT | EDGE_I420_TOP_HAS_RIGHT);
        node.v[1] = edge_flags
            & (EDGE_I444_TOP_HAS_RIGHT | EDGE_I422_TOP_HAS_RIGHT | EDGE_I420_TOP_HAS_RIGHT);

        nwc.h4[0] = edge_flags
            | (EDGE_I444_LEFT_HAS_BOTTOM | EDGE_I422_LEFT_HAS_BOTTOM | EDGE_I420_LEFT_HAS_BOTTOM);
        nwc.h4[1] =
            EDGE_I444_LEFT_HAS_BOTTOM | EDGE_I422_LEFT_HAS_BOTTOM | EDGE_I420_LEFT_HAS_BOTTOM;
        nwc.h4[2] = nwc.h4[1];
        nwc.h4[3] = edge_flags
            & (EDGE_I444_LEFT_HAS_BOTTOM | EDGE_I422_LEFT_HAS_BOTTOM | EDGE_I420_LEFT_HAS_BOTTOM);
        if bl == BL_16X16 {
            nwc.h4[1] |= edge_flags & EDGE_I420_TOP_HAS_RIGHT;
        }

        nwc.v4[0] = edge_flags
            | (EDGE_I444_TOP_HAS_RIGHT | EDGE_I422_TOP_HAS_RIGHT | EDGE_I420_TOP_HAS_RIGHT);
        nwc.v4[1] = EDGE_I444_TOP_HAS_RIGHT | EDGE_I422_TOP_HAS_RIGHT | EDGE_I420_TOP_HAS_RIGHT;
        nwc.v4[2] = nwc.v4[1];
        nwc.v4[3] = edge_flags
            & (EDGE_I444_TOP_HAS_RIGHT | EDGE_I422_TOP_HAS_RIGHT | EDGE_I420_TOP_HAS_RIGHT);
        if bl == BL_16X16 {
            nwc.v4[1] |= edge_flags & (EDGE_I420_LEFT_HAS_BOTTOM | EDGE_I422_LEFT_HAS_BOTTOM);
        }

        nwc.tls[0] = (EDGE_I444_TOP_HAS_RIGHT | EDGE_I422_TOP_HAS_RIGHT | EDGE_I420_TOP_HAS_RIGHT)
            | (EDGE_I444_LEFT_HAS_BOTTOM | EDGE_I422_LEFT_HAS_BOTTOM | EDGE_I420_LEFT_HAS_BOTTOM);
        nwc.tls[1] = edge_flags
            & (EDGE_I444_LEFT_HAS_BOTTOM | EDGE_I422_LEFT_HAS_BOTTOM | EDGE_I420_LEFT_HAS_BOTTOM);
        nwc.tls[2] = edge_flags
            & (EDGE_I444_TOP_HAS_RIGHT | EDGE_I422_TOP_HAS_RIGHT | EDGE_I420_TOP_HAS_RIGHT);

        nwc.trs[0] = edge_flags
            | (EDGE_I444_TOP_HAS_RIGHT | EDGE_I422_TOP_HAS_RIGHT | EDGE_I420_TOP_HAS_RIGHT);
        nwc.trs[1] = edge_flags
            | (EDGE_I444_LEFT_HAS_BOTTOM | EDGE_I422_LEFT_HAS_BOTTOM | EDGE_I420_LEFT_HAS_BOTTOM);
        nwc.trs[2] = 0 as EdgeFlags;

        nwc.tts[0] = (EDGE_I444_TOP_HAS_RIGHT | EDGE_I422_TOP_HAS_RIGHT | EDGE_I420_TOP_HAS_RIGHT)
            | (EDGE_I444_LEFT_HAS_BOTTOM | EDGE_I422_LEFT_HAS_BOTTOM | EDGE_I420_LEFT_HAS_BOTTOM);
        nwc.tts[1] = edge_flags
            & (EDGE_I444_TOP_HAS_RIGHT | EDGE_I422_TOP_HAS_RIGHT | EDGE_I420_TOP_HAS_RIGHT);
        nwc.tts[2] = edge_flags
            & (EDGE_I444_LEFT_HAS_BOTTOM | EDGE_I422_LEFT_HAS_BOTTOM | EDGE_I420_LEFT_HAS_BOTTOM);

        nwc.tbs[0] = edge_flags
            | (EDGE_I444_LEFT_HAS_BOTTOM | EDGE_I422_LEFT_HAS_BOTTOM | EDGE_I420_LEFT_HAS_BOTTOM);
        nwc.tbs[1] = edge_flags
            | (EDGE_I444_TOP_HAS_RIGHT | EDGE_I422_TOP_HAS_RIGHT | EDGE_I420_TOP_HAS_RIGHT);
        nwc.tbs[2] = 0 as EdgeFlags;
    };
}
unsafe extern "C" fn init_mode_node(
    nwc: *mut EdgeBranch,
    bl: BlockLevel,
    mem: *mut ModeSelMem,
    top_has_right: libc::c_int,
    left_has_bottom: libc::c_int,
) {
    init_edges(
        &mut (*nwc).node,
        bl,
        ((if top_has_right != 0 {
            EDGE_I444_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I422_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I420_TOP_HAS_RIGHT as libc::c_int
        } else {
            0 as libc::c_int
        }) | (if left_has_bottom != 0 {
            EDGE_I444_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I422_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I420_LEFT_HAS_BOTTOM as libc::c_int
        } else {
            0 as libc::c_int
        })) as EdgeFlags,
    );
    if bl as libc::c_uint == BL_16X16 as libc::c_int as libc::c_uint {
        let mut n = 0;
        while n < 4 {
            let fresh0 = (*mem).nt;
            (*mem).nt = ((*mem).nt).offset(1);
            let nt: *mut EdgeTip = fresh0;
            (*nwc).split[n as usize] = &mut (*nt).node;
            init_edges(
                &mut (*nt).node,
                (bl as libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint) as BlockLevel,
                ((if n == 3 || n == 1 && top_has_right == 0 {
                    0 as libc::c_int
                } else {
                    EDGE_I444_TOP_HAS_RIGHT as libc::c_int
                        | EDGE_I422_TOP_HAS_RIGHT as libc::c_int
                        | EDGE_I420_TOP_HAS_RIGHT as libc::c_int
                }) | (if !(n == 0 || n == 2 && left_has_bottom != 0) {
                    0 as libc::c_int
                } else {
                    EDGE_I444_LEFT_HAS_BOTTOM as libc::c_int
                        | EDGE_I422_LEFT_HAS_BOTTOM as libc::c_int
                        | EDGE_I420_LEFT_HAS_BOTTOM as libc::c_int
                })) as EdgeFlags,
            );
            n += 1;
        }
    } else {
        let mut n_0 = 0;
        while n_0 < 4 {
            let fresh1 = (*mem).nwc[bl as usize];
            (*mem).nwc[bl as usize] = ((*mem).nwc[bl as usize]).offset(1);
            let nwc_child: *mut EdgeBranch = fresh1;
            (*nwc).split[n_0 as usize] = &mut (*nwc_child).node;
            init_mode_node(
                nwc_child,
                (bl as libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint) as BlockLevel,
                mem,
                !(n_0 == 3 || n_0 == 1 && top_has_right == 0) as libc::c_int,
                (n_0 == 0 || n_0 == 2 && left_has_bottom != 0) as libc::c_int,
            );
            n_0 += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_init_mode_tree(
    root_node: *mut EdgeNode,
    nt: *mut EdgeTip,
    allow_sb128: libc::c_int,
) {
    let root: *mut EdgeBranch = root_node as *mut EdgeBranch;
    let mut mem: ModeSelMem = ModeSelMem {
        nwc: [0 as *mut EdgeBranch; 3],
        nt: 0 as *mut EdgeTip,
    };
    mem.nt = nt;
    if allow_sb128 != 0 {
        mem.nwc[BL_128X128 as libc::c_int as usize] = &mut *root.offset(1) as *mut EdgeBranch;
        mem.nwc[BL_64X64 as libc::c_int as usize] =
            &mut *root.offset((1 + 4) as isize) as *mut EdgeBranch;
        mem.nwc[BL_32X32 as libc::c_int as usize] =
            &mut *root.offset((1 + 4 + 16) as isize) as *mut EdgeBranch;
        init_mode_node(
            root,
            BL_128X128,
            &mut mem,
            1 as libc::c_int,
            0 as libc::c_int,
        );
        if !(mem.nwc[BL_128X128 as libc::c_int as usize]
            == &mut *root.offset((1 + 4) as isize) as *mut EdgeBranch)
        {
            unreachable!();
        }
        if !(mem.nwc[BL_64X64 as libc::c_int as usize]
            == &mut *root.offset((1 + 4 + 16) as isize) as *mut EdgeBranch)
        {
            unreachable!();
        }
        if !(mem.nwc[BL_32X32 as libc::c_int as usize]
            == &mut *root.offset((1 + 4 + 16 + 64) as isize) as *mut EdgeBranch)
        {
            unreachable!();
        }
        if !(mem.nt == &mut *nt.offset(256) as *mut EdgeTip) {
            unreachable!();
        }
    } else {
        mem.nwc[BL_128X128 as libc::c_int as usize] = 0 as *mut EdgeBranch;
        mem.nwc[BL_64X64 as libc::c_int as usize] = &mut *root.offset(1) as *mut EdgeBranch;
        mem.nwc[BL_32X32 as libc::c_int as usize] =
            &mut *root.offset((1 + 4) as isize) as *mut EdgeBranch;
        init_mode_node(root, BL_64X64, &mut mem, 1 as libc::c_int, 0 as libc::c_int);
        if !(mem.nwc[BL_64X64 as libc::c_int as usize]
            == &mut *root.offset((1 + 4) as isize) as *mut EdgeBranch)
        {
            unreachable!();
        }
        if !(mem.nwc[BL_32X32 as libc::c_int as usize]
            == &mut *root.offset((1 + 4 + 16) as isize) as *mut EdgeBranch)
        {
            unreachable!();
        }
        if !(mem.nt == &mut *nt.offset(64) as *mut EdgeTip) {
            unreachable!();
        }
    };
}
