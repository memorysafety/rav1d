use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::src::enum_map::DefaultValue;
use crate::src::levels::BlockLevel;
use bitflags::bitflags;
use std::ops::Shr;

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub(crate) struct EdgeFlags: u8 {
        const I444_TOP_HAS_RIGHT = 1 << 0;
        const I422_TOP_HAS_RIGHT = 1 << 1;
        const I420_TOP_HAS_RIGHT = 1 << 2;

        const I444_LEFT_HAS_BOTTOM = 1 << 3;
        const I422_LEFT_HAS_BOTTOM = 1 << 4;
        const I420_LEFT_HAS_BOTTOM = 1 << 5;
    }
}

impl EdgeFlags {
    pub const ALL_LEFT_HAS_BOTTOM: Self = Self::union_all([
        Self::I444_LEFT_HAS_BOTTOM,
        Self::I422_LEFT_HAS_BOTTOM,
        Self::I420_LEFT_HAS_BOTTOM,
    ]);

    pub const ALL_TOP_HAS_RIGHT: Self = Self::union_all([
        Self::I444_TOP_HAS_RIGHT,
        Self::I422_TOP_HAS_RIGHT,
        Self::I420_TOP_HAS_RIGHT,
    ]);

    pub const ALL_TR_AND_BL: Self =
        Self::union_all([Self::ALL_LEFT_HAS_BOTTOM, Self::ALL_TOP_HAS_RIGHT]);

    pub const fn union_all<const N: usize>(flags: [Self; N]) -> Self {
        let mut i = 0;
        let mut output = Self::empty();

        while i < N {
            output = output.union(flags[i]);
            i += 1;
        }

        output
    }

    pub(crate) const fn select(&self, select: bool) -> Self {
        if select {
            *self
        } else {
            Self::empty()
        }
    }
}

impl Shr<Rav1dPixelLayout> for EdgeFlags {
    type Output = Self;

    fn shr(self, rhs: Rav1dPixelLayout) -> Self::Output {
        Self::from_bits_retain(self.bits() >> (rhs as u32).wrapping_sub(1))
    }
}

const B: usize = 4;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EdgeKind {
    Tip,
    Branch,
}

#[derive(Clone, Copy)]
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
pub struct EdgeNode {
    pub o: EdgeFlags,
    pub h: [EdgeFlags; 2],
    pub v: [EdgeFlags; 2],
}

#[repr(C)]
pub struct EdgeTip {
    pub node: EdgeNode,
    pub split: [EdgeFlags; 3],
}

#[repr(C)]
pub struct EdgeBranch {
    pub node: EdgeNode,
    pub h4: EdgeFlags,
    pub v4: EdgeFlags,
    pub split: [EdgeIndex; B],
}

impl EdgeTip {
    const fn new(edge_flags: EdgeFlags) -> Self {
        let o = edge_flags;
        let h = [
            edge_flags.union(EdgeFlags::ALL_LEFT_HAS_BOTTOM),
            edge_flags.intersection(EdgeFlags::union_all([
                EdgeFlags::ALL_LEFT_HAS_BOTTOM,
                EdgeFlags::I420_TOP_HAS_RIGHT,
            ])),
        ];
        let v = [
            edge_flags.union(EdgeFlags::ALL_TOP_HAS_RIGHT),
            edge_flags.intersection(EdgeFlags::union_all([
                EdgeFlags::ALL_TOP_HAS_RIGHT,
                EdgeFlags::I420_LEFT_HAS_BOTTOM,
                EdgeFlags::I422_LEFT_HAS_BOTTOM,
            ])),
        ];
        let node = EdgeNode { o, h, v };

        let split = [
            edge_flags
                .intersection(EdgeFlags::ALL_TOP_HAS_RIGHT)
                .union(EdgeFlags::I422_LEFT_HAS_BOTTOM),
            edge_flags.union(EdgeFlags::I444_TOP_HAS_RIGHT),
            edge_flags.intersection(EdgeFlags::union_all([
                EdgeFlags::I420_TOP_HAS_RIGHT,
                EdgeFlags::I420_LEFT_HAS_BOTTOM,
                EdgeFlags::I422_LEFT_HAS_BOTTOM,
            ])),
        ];

        Self { node, split }
    }
}

impl EdgeBranch {
    const fn new(edge_flags: EdgeFlags, bl: BlockLevel) -> Self {
        let o = edge_flags;
        let h = [
            edge_flags.union(EdgeFlags::ALL_LEFT_HAS_BOTTOM),
            edge_flags.intersection(EdgeFlags::ALL_LEFT_HAS_BOTTOM),
        ];
        let v = [
            edge_flags.union(EdgeFlags::ALL_TOP_HAS_RIGHT),
            edge_flags.intersection(EdgeFlags::ALL_TOP_HAS_RIGHT),
        ];
        let node = EdgeNode { o, h, v };

        let h4 = edge_flags
            .intersection(EdgeFlags::I420_TOP_HAS_RIGHT)
            .select(matches!(bl, BlockLevel::Bl16x16))
            .union(EdgeFlags::ALL_LEFT_HAS_BOTTOM);

        let v4 = edge_flags
            .intersection(EdgeFlags::union_all([
                EdgeFlags::I420_LEFT_HAS_BOTTOM,
                EdgeFlags::I422_LEFT_HAS_BOTTOM,
            ]))
            .select(matches!(bl, BlockLevel::Bl16x16))
            .union(EdgeFlags::ALL_TOP_HAS_RIGHT);

        let split = [EdgeIndex::root(); 4];

        Self {
            node,
            h4,
            v4,
            split,
        }
    }
}

impl DefaultValue for EdgeTip {
    const DEFAULT: Self = Self::new(EdgeFlags::empty());
}

impl DefaultValue for EdgeBranch {
    const DEFAULT: Self = Self::new(EdgeFlags::empty(), BlockLevel::Bl128x128);
}

struct EdgeIndices {
    branch: [EdgeIndex; 3],
    tip: EdgeIndex,
}

#[repr(C)]
struct IntraEdge<const SB128: bool, const N_BRANCH: usize, const N_TIP: usize> {
    branch: [EdgeBranch; N_BRANCH],
    tip: [EdgeTip; N_TIP],
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
            EdgeFlags::union_all([
                EdgeFlags::ALL_TOP_HAS_RIGHT.select(top_has_right),
                EdgeFlags::ALL_LEFT_HAS_BOTTOM.select(left_has_bottom),
            ]),
            bl,
        );
        if matches!(bl, BlockLevel::Bl16x16) {
            let mut n = 0;
            while n < B as u8 {
                let (tip, next) = indices.tip.pop_front();
                indices.tip = next;
                branch.split[n as usize] = tip;
                self.tip[tip.index as usize] = EdgeTip::new(EdgeFlags::union_all([
                    EdgeFlags::ALL_TOP_HAS_RIGHT.select(!(n == 3 || (n == 1 && !top_has_right))),
                    EdgeFlags::ALL_LEFT_HAS_BOTTOM.select(n == 0 || (n == 2 && left_has_bottom)),
                ]));
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
                    match bl.decrease() {
                        None => panic!("BlockLevel::BL_8X8 should never make it here"),
                        Some(next_bl) => next_bl,
                    },
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

    const fn init(mut self) -> Self {
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

        let mut bl = BlockLevel::Bl128x128 as u8;
        while bl <= BlockLevel::Bl32x32 as u8 {
            indices.branch[bl as usize].index = level_index(bl as u8 + sb128);
            bl += 1;
        }

        let bl = if SB128 {
            BlockLevel::Bl128x128
        } else {
            BlockLevel::Bl64x64
        };
        (self, indices) = self.init_mode_node(EdgeIndex::root(), bl, indices, true, false);

        let mut bl = BlockLevel::Bl128x128 as u8;
        while bl <= BlockLevel::Bl32x32 as u8 {
            let index = indices.branch[bl as usize].index;
            if index != 0 {
                assert!(index == level_index(1 + bl + sb128));
            }
            bl += 1;
        }
        assert!(indices.tip.index == self.tip.len() as u8);

        self
    }

    /// Check that all indices are in bound so that bounds checks are not needed at runtime.
    const fn check_indices(self) -> Self {
        let mut i = 0;
        while i < self.branch.len() {
            let mut j = 0;
            while j < B {
                let edge = self.branch[i].split[j];
                let index = edge.index as usize;
                match edge.kind {
                    EdgeKind::Branch => assert!(index < self.branch.len()),
                    EdgeKind::Tip => assert!(index < self.tip.len()),
                }
                j += 1;
            }
            i += 1;
        }

        self
    }

    const fn new() -> Self {
        Self {
            branch: [EdgeBranch::DEFAULT; N_BRANCH],
            tip: [EdgeTip::DEFAULT; N_TIP],
        }
        .init()
        .check_indices()
    }

    fn edge<E, const N: usize>(edges: &[E; N], edge: EdgeIndex, kind: EdgeKind) -> &E {
        assert!(edge.kind == kind);
        if cfg!(debug_assertions) {
            &edges[edge.index as usize]
        } else {
            // SAFETY: Already checked in `Self::check_indices`, and `EdgeIndex`'s fields are private.
            unsafe { edges.get_unchecked(edge.index as usize) }
        }
    }

    pub fn branch(&self, branch: EdgeIndex) -> &EdgeBranch {
        Self::edge(&self.branch, branch, EdgeKind::Branch)
    }

    pub fn tip(&self, tip: EdgeIndex) -> &EdgeTip {
        Self::edge(&self.tip, tip, EdgeKind::Tip)
    }

    pub fn node(&self, node: EdgeIndex) -> &EdgeNode {
        match node.kind {
            EdgeKind::Branch => &self.branch(node).node,
            EdgeKind::Tip => &self.tip(node).node,
        }
    }
}

/// A tree to keep track of which edges are available.
#[repr(C)]
pub struct IntraEdges {
    sb128: IntraEdge<true, 85, 256>,
    sb64: IntraEdge<false, 21, 64>,
}

impl IntraEdges {
    #[inline(always)]
    const fn new() -> Self {
        Self {
            sb128: IntraEdge::new(),
            sb64: IntraEdge::new(),
        }
    }

    pub fn branch(&self, sb128: bool, branch: EdgeIndex) -> &EdgeBranch {
        assert!(branch.kind == EdgeKind::Branch); // Optimizes better before the `if`.
        if sb128 {
            self.sb128.branch(branch)
        } else {
            self.sb64.branch(branch)
        }
    }

    pub fn tip(&self, sb128: bool, tip: EdgeIndex) -> &EdgeTip {
        assert!(tip.kind == EdgeKind::Tip); // Optimizes better before the `if`.
        if sb128 {
            self.sb128.tip(tip)
        } else {
            self.sb64.tip(tip)
        }
    }

    pub fn node(&self, sb128: bool, node: EdgeIndex) -> &EdgeNode {
        if sb128 {
            self.sb128.node(node)
        } else {
            self.sb64.node(node)
        }
    }
}

impl DefaultValue for IntraEdges {
    const DEFAULT: Self = Self::new();
}
