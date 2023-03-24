use ::libc;
pub type EdgeFlags = libc::c_uint;
pub const EDGE_I420_LEFT_HAS_BOTTOM: EdgeFlags = 32;
pub const EDGE_I422_LEFT_HAS_BOTTOM: EdgeFlags = 16;
pub const EDGE_I444_LEFT_HAS_BOTTOM: EdgeFlags = 8;
pub const EDGE_I420_TOP_HAS_RIGHT: EdgeFlags = 4;
pub const EDGE_I422_TOP_HAS_RIGHT: EdgeFlags = 2;
pub const EDGE_I444_TOP_HAS_RIGHT: EdgeFlags = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EdgeNode {
    pub o: EdgeFlags,
    pub h: [EdgeFlags; 2],
    pub v: [EdgeFlags; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EdgeTip {
    pub node: EdgeNode,
    pub split: [EdgeFlags; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ModeSelMem {
    pub nwc: [*mut EdgeBranch; 3],
    pub nt: *mut EdgeTip,
}
pub const BL_32X32: BlockLevel = 2;
pub const BL_64X64: BlockLevel = 1;
pub type BlockLevel = libc::c_uint;
pub const N_BL_LEVELS: BlockLevel = 5;
pub const BL_8X8: BlockLevel = 4;
pub const BL_16X16: BlockLevel = 3;
pub const BL_128X128: BlockLevel = 0;
unsafe extern "C" fn init_edges(node: *mut EdgeNode, bl: BlockLevel, edge_flags: EdgeFlags) {
    (*node).o = edge_flags;
    if bl == BL_8X8 {
        let nt: *mut EdgeTip = node as *mut EdgeTip;
        (*node).h[0usize] = edge_flags
            | (EDGE_I444_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I422_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I420_LEFT_HAS_BOTTOM as libc::c_int) as libc::c_uint;
        (*node).h[1usize] = edge_flags
            & (EDGE_I444_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I422_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I420_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I420_TOP_HAS_RIGHT as libc::c_int) as libc::c_uint;
        (*node).v[0usize] = edge_flags
            | (EDGE_I444_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I422_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I420_TOP_HAS_RIGHT as libc::c_int) as libc::c_uint;
        (*node).v[1usize] = edge_flags
            & (EDGE_I444_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I422_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I420_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I420_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I422_LEFT_HAS_BOTTOM as libc::c_int) as libc::c_uint;
        (*nt).split[0usize] = (EDGE_I444_TOP_HAS_RIGHT as libc::c_int
            | EDGE_I422_TOP_HAS_RIGHT as libc::c_int
            | EDGE_I420_TOP_HAS_RIGHT as libc::c_int
            | (EDGE_I444_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I422_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I420_LEFT_HAS_BOTTOM as libc::c_int))
            as EdgeFlags;
        (*nt).split[1usize] = edge_flags
            & (EDGE_I444_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I422_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I420_TOP_HAS_RIGHT as libc::c_int) as libc::c_uint
            | EDGE_I422_LEFT_HAS_BOTTOM;
        (*nt).split[2usize] = edge_flags | EDGE_I444_TOP_HAS_RIGHT;
        (*nt).split[3usize] = edge_flags
            & (EDGE_I420_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I420_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I422_LEFT_HAS_BOTTOM as libc::c_int) as libc::c_uint;
    } else {
        let nwc: *mut EdgeBranch = node as *mut EdgeBranch;
        (*node).h[0usize] = edge_flags
            | (EDGE_I444_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I422_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I420_LEFT_HAS_BOTTOM as libc::c_int) as libc::c_uint;
        (*node).h[1usize] = edge_flags
            & (EDGE_I444_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I422_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I420_LEFT_HAS_BOTTOM as libc::c_int) as libc::c_uint;
        (*node).v[0usize] = edge_flags
            | (EDGE_I444_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I422_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I420_TOP_HAS_RIGHT as libc::c_int) as libc::c_uint;
        (*node).v[1usize] = edge_flags
            & (EDGE_I444_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I422_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I420_TOP_HAS_RIGHT as libc::c_int) as libc::c_uint;
        (*nwc).h4[0usize] = edge_flags
            | (EDGE_I444_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I422_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I420_LEFT_HAS_BOTTOM as libc::c_int) as libc::c_uint;
        (*nwc).h4[2usize] = (EDGE_I444_LEFT_HAS_BOTTOM as libc::c_int
            | EDGE_I422_LEFT_HAS_BOTTOM as libc::c_int
            | EDGE_I420_LEFT_HAS_BOTTOM as libc::c_int) as EdgeFlags;
        (*nwc).h4[1usize] = (*nwc).h4[2usize];
        (*nwc).h4[3usize] = edge_flags
            & (EDGE_I444_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I422_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I420_LEFT_HAS_BOTTOM as libc::c_int) as libc::c_uint;
        if bl == BL_16X16 {
            (*nwc).h4[1usize] = ::core::mem::transmute::<libc::c_uint, EdgeFlags>(
                (*nwc).h4[1usize] | edge_flags & EDGE_I420_TOP_HAS_RIGHT,
            );
        }
        (*nwc).v4[0usize] = edge_flags
            | (EDGE_I444_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I422_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I420_TOP_HAS_RIGHT as libc::c_int) as libc::c_uint;
        (*nwc).v4[2usize] = (EDGE_I444_TOP_HAS_RIGHT as libc::c_int
            | EDGE_I422_TOP_HAS_RIGHT as libc::c_int
            | EDGE_I420_TOP_HAS_RIGHT as libc::c_int) as EdgeFlags;
        (*nwc).v4[1usize] = (*nwc).v4[2usize];
        (*nwc).v4[3usize] = edge_flags
            & (EDGE_I444_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I422_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I420_TOP_HAS_RIGHT as libc::c_int) as libc::c_uint;
        if bl == BL_16X16 {
            (*nwc).v4[1usize] = ::core::mem::transmute::<libc::c_uint, EdgeFlags>(
                (*nwc).v4[1usize]
                    | edge_flags
                        & (EDGE_I420_LEFT_HAS_BOTTOM as libc::c_int
                            | EDGE_I422_LEFT_HAS_BOTTOM as libc::c_int)
                            as libc::c_uint,
            );
        }
        (*nwc).tls[0usize] = (EDGE_I444_TOP_HAS_RIGHT as libc::c_int
            | EDGE_I422_TOP_HAS_RIGHT as libc::c_int
            | EDGE_I420_TOP_HAS_RIGHT as libc::c_int
            | (EDGE_I444_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I422_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I420_LEFT_HAS_BOTTOM as libc::c_int))
            as EdgeFlags;
        (*nwc).tls[1usize] = edge_flags
            & (EDGE_I444_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I422_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I420_LEFT_HAS_BOTTOM as libc::c_int) as libc::c_uint;
        (*nwc).tls[2usize] = edge_flags
            & (EDGE_I444_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I422_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I420_TOP_HAS_RIGHT as libc::c_int) as libc::c_uint;
        (*nwc).trs[0usize] = edge_flags
            | (EDGE_I444_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I422_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I420_TOP_HAS_RIGHT as libc::c_int) as libc::c_uint;
        (*nwc).trs[1usize] = edge_flags
            | (EDGE_I444_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I422_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I420_LEFT_HAS_BOTTOM as libc::c_int) as libc::c_uint;
        (*nwc).trs[2usize] = 0u32;
        (*nwc).tts[0usize] = (EDGE_I444_TOP_HAS_RIGHT as libc::c_int
            | EDGE_I422_TOP_HAS_RIGHT as libc::c_int
            | EDGE_I420_TOP_HAS_RIGHT as libc::c_int
            | (EDGE_I444_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I422_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I420_LEFT_HAS_BOTTOM as libc::c_int))
            as EdgeFlags;
        (*nwc).tts[1usize] = edge_flags
            & (EDGE_I444_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I422_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I420_TOP_HAS_RIGHT as libc::c_int) as libc::c_uint;
        (*nwc).tts[2usize] = edge_flags
            & (EDGE_I444_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I422_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I420_LEFT_HAS_BOTTOM as libc::c_int) as libc::c_uint;
        (*nwc).tbs[0usize] = edge_flags
            | (EDGE_I444_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I422_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I420_LEFT_HAS_BOTTOM as libc::c_int) as libc::c_uint;
        (*nwc).tbs[1usize] = edge_flags
            | (EDGE_I444_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I422_TOP_HAS_RIGHT as libc::c_int
                | EDGE_I420_TOP_HAS_RIGHT as libc::c_int) as libc::c_uint;
        (*nwc).tbs[2usize] = 0u32;
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
            0i32
        }) | (if left_has_bottom != 0 {
            EDGE_I444_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I422_LEFT_HAS_BOTTOM as libc::c_int
                | EDGE_I420_LEFT_HAS_BOTTOM as libc::c_int
        } else {
            0i32
        })) as EdgeFlags,
    );
    if bl == BL_16X16 {
        let mut n: libc::c_int = 0i32;
        while n < 4i32 {
            let fresh0 = (*mem).nt;
            (*mem).nt = ((*mem).nt).offset(1);
            let nt: *mut EdgeTip = fresh0;
            (*nwc).split[n as usize] = &mut (*nt).node;
            init_edges(
                &mut (*nt).node,
                (bl).wrapping_add(1u32),
                ((if n == 3i32 || n == 1i32 && top_has_right == 0 {
                    0i32
                } else {
                    EDGE_I444_TOP_HAS_RIGHT as libc::c_int
                        | EDGE_I422_TOP_HAS_RIGHT as libc::c_int
                        | EDGE_I420_TOP_HAS_RIGHT as libc::c_int
                }) | (if !(n == 0i32 || n == 2i32 && left_has_bottom != 0) {
                    0i32
                } else {
                    EDGE_I444_LEFT_HAS_BOTTOM as libc::c_int
                        | EDGE_I422_LEFT_HAS_BOTTOM as libc::c_int
                        | EDGE_I420_LEFT_HAS_BOTTOM as libc::c_int
                })) as EdgeFlags,
            );
            n += 1;
        }
    } else {
        let mut n_0: libc::c_int = 0i32;
        while n_0 < 4i32 {
            let fresh1 = (*mem).nwc[bl as usize];
            (*mem).nwc[bl as usize] = ((*mem).nwc[bl as usize]).offset(1);
            let nwc_child: *mut EdgeBranch = fresh1;
            (*nwc).split[n_0 as usize] = &mut (*nwc_child).node;
            init_mode_node(
                nwc_child,
                (bl).wrapping_add(1u32),
                mem,
                !(n_0 == 3i32 || n_0 == 1i32 && top_has_right == 0) as libc::c_int,
                (n_0 == 0i32 || n_0 == 2i32 && left_has_bottom != 0) as libc::c_int,
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
        mem.nwc[BL_128X128 as usize] = &mut *root.offset(1isize) as *mut EdgeBranch;
        mem.nwc[BL_64X64 as usize] = &mut *root.offset((1i32 + 4i32) as isize) as *mut EdgeBranch;
        mem.nwc[BL_32X32 as usize] =
            &mut *root.offset((1i32 + 4i32 + 16i32) as isize) as *mut EdgeBranch;
        init_mode_node(root, BL_128X128, &mut mem, 1i32, 0i32);
        if !(mem.nwc[BL_128X128 as usize]
            == &mut *root.offset((1i32 + 4i32) as isize) as *mut EdgeBranch)
        {
            unreachable!();
        }
        if !(mem.nwc[BL_64X64 as usize]
            == &mut *root.offset((1i32 + 4i32 + 16i32) as isize) as *mut EdgeBranch)
        {
            unreachable!();
        }
        if !(mem.nwc[BL_32X32 as usize]
            == &mut *root.offset((1i32 + 4i32 + 16i32 + 64i32) as isize) as *mut EdgeBranch)
        {
            unreachable!();
        }
        if !(mem.nt == &mut *nt.offset(256isize) as *mut EdgeTip) {
            unreachable!();
        }
    } else {
        mem.nwc[BL_128X128 as usize] = 0 as *mut EdgeBranch;
        mem.nwc[BL_64X64 as usize] = &mut *root.offset(1isize) as *mut EdgeBranch;
        mem.nwc[BL_32X32 as usize] = &mut *root.offset((1i32 + 4i32) as isize) as *mut EdgeBranch;
        init_mode_node(root, BL_64X64, &mut mem, 1i32, 0i32);
        if !(mem.nwc[BL_64X64 as usize]
            == &mut *root.offset((1i32 + 4i32) as isize) as *mut EdgeBranch)
        {
            unreachable!();
        }
        if !(mem.nwc[BL_32X32 as usize]
            == &mut *root.offset((1i32 + 4i32 + 16i32) as isize) as *mut EdgeBranch)
        {
            unreachable!();
        }
        if !(mem.nt == &mut *nt.offset(64isize) as *mut EdgeTip) {
            unreachable!();
        }
    };
}
