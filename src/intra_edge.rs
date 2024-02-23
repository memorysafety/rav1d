use crate::src::enum_map::DefaultValue;
use crate::src::levels::BlockLevel;
use crate::src::levels::BL_128X128;
use crate::src::levels::BL_16X16;
use crate::src::levels::BL_32X32;
use crate::src::levels::BL_64X64;

pub type EdgeFlags = u8;
pub const EDGE_I420_LEFT_HAS_BOTTOM: EdgeFlags = 32;
pub const EDGE_I422_LEFT_HAS_BOTTOM: EdgeFlags = 16;
pub const EDGE_I444_LEFT_HAS_BOTTOM: EdgeFlags = 8;
pub const EDGE_I420_TOP_HAS_RIGHT: EdgeFlags = 4;
pub const EDGE_I422_TOP_HAS_RIGHT: EdgeFlags = 2;
pub const EDGE_I444_TOP_HAS_RIGHT: EdgeFlags = 1;

pub const EDGE_LEFT_HAS_BOTTOM: EdgeFlags =
    EDGE_I444_LEFT_HAS_BOTTOM | EDGE_I422_LEFT_HAS_BOTTOM | EDGE_I420_LEFT_HAS_BOTTOM;
pub const EDGE_TOP_HAS_RIGHT: EdgeFlags =
    EDGE_I444_TOP_HAS_RIGHT | EDGE_I422_TOP_HAS_RIGHT | EDGE_I420_TOP_HAS_RIGHT;

const B: usize = 4;

#[repr(C)]
pub struct EdgeNode {
    pub o: EdgeFlags,
    pub h: [EdgeFlags; 2],
    pub v: [EdgeFlags; 2],
}

#[repr(C)]
pub struct EdgeTip {
    pub node: EdgeNode,
    pub split: [EdgeFlags; B],
}

impl DefaultValue for EdgeTip {
    const DEFAULT: Self = Self::new(0 as EdgeFlags);
}

#[derive(Clone, Copy, Debug)]
pub enum EdgeKind {
    Tip,
    Branch,
}

#[derive(Clone, Copy, Debug)]
pub struct EdgeIndex {
    index: u8,
    kind: EdgeKind,
}

impl EdgeIndex {
    pub const fn root() -> Self {
        Self {
            index: 0,
            kind: EdgeKind::Branch,
        }
    }

    #[must_use]
    pub const fn pop_front(mut self) -> (Self, Self) {
        let front = self;
        self.index = self.index.wrapping_add(1);
        (front, self)
    }
}

#[repr(C)]
pub struct EdgeBranch {
    pub node: EdgeNode,
    pub tts: [EdgeFlags; 3],
    pub tbs: [EdgeFlags; 3],
    pub tls: [EdgeFlags; 3],
    pub trs: [EdgeFlags; 3],
    pub h4: [EdgeFlags; 4],
    pub v4: [EdgeFlags; 4],
    pub split: [EdgeIndex; B],
}

impl DefaultValue for EdgeBranch {
    const DEFAULT: Self = Self::new(0 as EdgeFlags, 0 as BlockLevel);
}

struct EdgeIndices {
    pub branch: [EdgeIndex; 3],
    pub tip: EdgeIndex,
}

impl EdgeTip {
    const fn new(edge_flags: EdgeFlags) -> Self {
        let o = edge_flags;
        let h = [
            edge_flags | EDGE_LEFT_HAS_BOTTOM,
            edge_flags & (EDGE_LEFT_HAS_BOTTOM | EDGE_I420_TOP_HAS_RIGHT),
        ];
        let v = [
            edge_flags | EDGE_TOP_HAS_RIGHT,
            edge_flags
                & (EDGE_TOP_HAS_RIGHT | EDGE_I420_LEFT_HAS_BOTTOM | EDGE_I422_LEFT_HAS_BOTTOM),
        ];
        let node = EdgeNode { o, h, v };

        let split = [
            EDGE_TOP_HAS_RIGHT | EDGE_LEFT_HAS_BOTTOM,
            (edge_flags & EDGE_TOP_HAS_RIGHT) | EDGE_I422_LEFT_HAS_BOTTOM,
            edge_flags | EDGE_I444_TOP_HAS_RIGHT,
            edge_flags
                & (EDGE_I420_TOP_HAS_RIGHT | EDGE_I420_LEFT_HAS_BOTTOM | EDGE_I422_LEFT_HAS_BOTTOM),
        ];

        Self { node, split }
    }
}

impl EdgeBranch {
    const fn new(edge_flags: EdgeFlags, bl: BlockLevel) -> Self {
        let o = edge_flags;
        let h = [
            edge_flags | EDGE_LEFT_HAS_BOTTOM,
            edge_flags & EDGE_LEFT_HAS_BOTTOM,
        ];
        let v = [
            edge_flags | EDGE_TOP_HAS_RIGHT,
            edge_flags & EDGE_TOP_HAS_RIGHT,
        ];
        let node = EdgeNode { o, h, v };

        let h4 = [
            edge_flags | EDGE_LEFT_HAS_BOTTOM,
            EDGE_LEFT_HAS_BOTTOM
                | (if bl == BL_16X16 {
                    edge_flags & EDGE_I420_TOP_HAS_RIGHT
                } else {
                    0 as EdgeFlags
                }),
            EDGE_LEFT_HAS_BOTTOM,
            edge_flags & EDGE_LEFT_HAS_BOTTOM,
        ];

        let v4 = [
            edge_flags | EDGE_TOP_HAS_RIGHT,
            EDGE_TOP_HAS_RIGHT
                | (if bl == BL_16X16 {
                    edge_flags & (EDGE_I420_LEFT_HAS_BOTTOM | EDGE_I422_LEFT_HAS_BOTTOM)
                } else {
                    0 as EdgeFlags
                }),
            EDGE_TOP_HAS_RIGHT,
            edge_flags & EDGE_TOP_HAS_RIGHT,
        ];

        let tls = [
            EDGE_TOP_HAS_RIGHT | EDGE_LEFT_HAS_BOTTOM,
            edge_flags & EDGE_LEFT_HAS_BOTTOM,
            edge_flags & EDGE_TOP_HAS_RIGHT,
        ];
        let trs = [
            edge_flags | EDGE_TOP_HAS_RIGHT,
            edge_flags | EDGE_LEFT_HAS_BOTTOM,
            0 as EdgeFlags,
        ];
        let tts = [
            EDGE_TOP_HAS_RIGHT | EDGE_LEFT_HAS_BOTTOM,
            edge_flags & EDGE_TOP_HAS_RIGHT,
            edge_flags & EDGE_LEFT_HAS_BOTTOM,
        ];
        let tbs = [
            edge_flags | EDGE_LEFT_HAS_BOTTOM,
            edge_flags | EDGE_TOP_HAS_RIGHT,
            0 as EdgeFlags,
        ];

        let split = [EdgeIndex::root(); 4];

        Self {
            node,
            h4,
            v4,
            tls,
            trs,
            tts,
            tbs,
            split,
        }
    }
}

impl<const SB128: bool, const N_BRANCH: usize, const N_TIP: usize>
    IntraEdge<SB128, N_BRANCH, N_TIP>
{
    #[must_use]
    const fn init_mode_node(
        mut self,
        branch_index: EdgeIndex,
        bl: BlockLevel,
        mut indices: EdgeIndices,
        top_has_right: bool,
        left_has_bottom: bool,
    ) -> (Self, EdgeIndices) {
        let mut branch = EdgeBranch::new(
            (if top_has_right {
                EDGE_TOP_HAS_RIGHT
            } else {
                0 as EdgeFlags
            }) | (if left_has_bottom {
                EDGE_LEFT_HAS_BOTTOM
            } else {
                0 as EdgeFlags
            }),
            bl,
        );
        if bl == BL_16X16 {
            let mut n = 0;
            while n < B as u8 {
                let (tip, next) = indices.tip.pop_front();
                indices.tip = next;
                branch.split[n as usize] = tip;
                let edge_flags = (if n == 3 || (n == 1 && !top_has_right) {
                    0 as EdgeFlags
                } else {
                    EDGE_TOP_HAS_RIGHT
                }) | (if !(n == 0 || (n == 2 && left_has_bottom)) {
                    0 as EdgeFlags
                } else {
                    EDGE_LEFT_HAS_BOTTOM
                });
                self.tip[tip.index as usize] = EdgeTip::new(edge_flags);
                n += 1;
            }
        } else {
            let mut n = 0;
            while n < B as u8 {
                let (child_branch, next) = indices.branch[bl as usize].pop_front();
                indices.branch[bl as usize] = next;
                branch.split[n as usize] = child_branch;
                (self, indices) = self.init_mode_node(
                    child_branch,
                    bl + 1,
                    indices,
                    !(n == 3 || (n == 1 && !top_has_right)),
                    n == 0 || (n == 2 && left_has_bottom),
                );
                n += 1;
            }
        };
        self.branch[branch_index.index as usize] = branch;
        (self, indices)
    }
}

const fn level_index(mut level: u8) -> u8 {
    let mut level_size = 1;
    let mut index = 0;
    while level > 0 {
        index += level_size;
        level_size *= B;
        level -= 1;
    }
    index as u8
}

impl<const SB128: bool, const N_BRANCH: usize, const N_TIP: usize>
    IntraEdge<SB128, N_BRANCH, N_TIP>
{
    const fn new() -> Self {
        let mut this = Self {
            branch: [EdgeBranch::DEFAULT; N_BRANCH],
            tip: [EdgeTip::DEFAULT; N_TIP],
        };
        let mut indices = EdgeIndices {
            branch: [EdgeIndex {
                index: 0,
                kind: EdgeKind::Branch,
            }; 3],
            tip: EdgeIndex {
                index: 0,
                kind: EdgeKind::Tip,
            },
        };

        let sb128 = SB128 as u8;

        let mut bl = BL_128X128;
        while bl <= BL_32X32 {
            indices.branch[bl as usize].index = level_index(bl + sb128);
            bl += 1;
        }

        let bl = if SB128 { BL_128X128 } else { BL_64X64 };
        (this, indices) = this.init_mode_node(EdgeIndex::root(), bl, indices, true, false);

        let mut bl = BL_128X128;
        while bl <= BL_32X32 {
            let index = indices.branch[bl as usize].index;
            if index != 0 {
                assert!(index == level_index(1 + bl + sb128));
            }
            bl += 1;
        }
        assert!(indices.tip.index == this.tip.len() as u8);

        this
    }
}

impl IntraEdges {
    pub const fn new() -> Self {
        Self {
            sb128: IntraEdge::new(),
            sb64: IntraEdge::new(),
        }
    }
}

#[repr(C)]
pub struct IntraEdge<const SB128: bool, const N_BRANCH: usize, const N_TIP: usize> {
    pub branch: [EdgeBranch; N_BRANCH],
    pub tip: [EdgeTip; N_TIP],
}

impl<const SB128: bool, const N_BRANCH: usize, const N_TIP: usize>
    IntraEdge<SB128, N_BRANCH, N_TIP>
{
    pub const fn branch(&self, branch: EdgeIndex) -> &EdgeBranch {
        // Only a debug assert since it is still memory safe without it.
        debug_assert!(matches!(branch.kind, EdgeKind::Branch));
        &self.branch[branch.index as usize]
    }

    pub const fn tip(&self, tip: EdgeIndex) -> &EdgeTip {
        // Only a debug assert since it is still memory safe without it.
        debug_assert!(matches!(tip.kind, EdgeKind::Tip));
        &self.tip[tip.index as usize]
    }

    pub const fn node(&self, node: EdgeIndex) -> &EdgeNode {
        match node.kind {
            EdgeKind::Branch => &self.branch(node).node,
            EdgeKind::Tip => &self.tip(node).node,
        }
    }
}

#[repr(C)]
pub struct IntraEdges {
    pub sb128: IntraEdge<true, 85, 256>,
    pub sb64: IntraEdge<false, 21, 64>,
}

impl IntraEdges {
    pub const fn branch(&self, sb128: bool, branch: EdgeIndex) -> &EdgeBranch {
        if sb128 {
            self.sb128.branch(branch)
        } else {
            self.sb64.branch(branch)
        }
    }

    pub const fn tip(&self, sb128: bool, tip: EdgeIndex) -> &EdgeTip {
        if sb128 {
            self.sb128.tip(tip)
        } else {
            self.sb64.tip(tip)
        }
    }

    pub const fn node(&self, sb128: bool, node: EdgeIndex) -> &EdgeNode {
        if sb128 {
            self.sb128.node(node)
        } else {
            self.sb64.node(node)
        }
    }
}
