use crate::include::common::intops::apply_sign;
use crate::include::dav1d::headers::Rav1dFilterMode;
use crate::include::dav1d::headers::Rav1dFrameHeader;
use crate::include::dav1d::headers::Rav1dWarpedMotionParams;
use crate::include::dav1d::headers::Rav1dWarpedMotionType;
use crate::src::align::Align8;
use crate::src::disjoint_mut::DisjointMut;
use crate::src::disjoint_mut::DisjointMutSlice;
use crate::src::internal::Bxy;
use crate::src::levels::BlockLevel;
use crate::src::levels::BlockPartition;
use crate::src::levels::CompInterType;
use crate::src::levels::Mv;
use crate::src::levels::SegmentId;
use crate::src::levels::TxfmSize;
use crate::src::levels::TxfmType;
use crate::src::levels::DCT_DCT;
use crate::src::levels::H_ADST;
use crate::src::levels::H_FLIPADST;
use crate::src::levels::IDTX;
use crate::src::levels::V_ADST;
use crate::src::levels::V_FLIPADST;
use crate::src::refmvs::RefMvsCandidate;
use crate::src::tables::TxfmInfo;
use std::cmp;
use std::cmp::Ordering;
use std::ffi::c_int;
use std::ffi::c_uint;

#[derive(Default)]
pub struct BlockContext {
    pub mode: DisjointMut<Align8<[u8; 32]>>,
    pub lcoef: DisjointMut<Align8<[u8; 32]>>,
    pub ccoef: [DisjointMut<Align8<[u8; 32]>>; 2],
    pub seg_pred: DisjointMut<Align8<[u8; 32]>>,
    pub skip: DisjointMut<Align8<[u8; 32]>>,
    pub skip_mode: DisjointMut<Align8<[u8; 32]>>,
    pub intra: DisjointMut<Align8<[u8; 32]>>,
    pub comp_type: DisjointMut<Align8<[Option<CompInterType>; 32]>>,
    pub r#ref: [DisjointMut<Align8<[i8; 32]>>; 2],

    /// No [`Rav1dFilterMode::Switchable`]s here.
    /// TODO(kkysen) split [`Rav1dFilterMode`] into a version without [`Rav1dFilterMode::Switchable`].
    pub filter: [DisjointMut<Align8<[Rav1dFilterMode; 32]>>; 2],

    pub tx_intra: DisjointMut<Align8<[i8; 32]>>,
    pub tx: DisjointMut<Align8<[TxfmSize; 32]>>,
    pub tx_lpf_y: DisjointMut<Align8<[u8; 32]>>,
    pub tx_lpf_uv: DisjointMut<Align8<[u8; 32]>>,
    pub partition: DisjointMut<Align8<[u8; 16]>>,
    pub uvmode: DisjointMut<Align8<[u8; 32]>>,
    pub pal_sz: DisjointMut<Align8<[u8; 32]>>,
}

#[inline]
pub fn get_intra_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: c_int,
    xb4: c_int,
    have_top: bool,
    have_left: bool,
) -> u8 {
    if have_left {
        if have_top {
            let ctx = *l.intra.index(yb4 as usize) + *a.intra.index(xb4 as usize);
            ctx + (ctx == 2) as u8
        } else {
            *l.intra.index(yb4 as usize) * 2
        }
    } else {
        if have_top {
            *a.intra.index(xb4 as usize) * 2
        } else {
            0
        }
    }
}

#[inline]
pub fn get_tx_ctx(
    a: &BlockContext,
    l: &BlockContext,
    max_tx: &TxfmInfo,
    yb4: c_int,
    xb4: c_int,
) -> u8 {
    (*l.tx_intra.index(yb4 as usize) as i32 >= max_tx.lh as i32) as u8
        + (*a.tx_intra.index(xb4 as usize) as i32 >= max_tx.lw as i32) as u8
}

#[inline]
pub fn get_partition_ctx(
    a: &BlockContext,
    l: &BlockContext,
    bl: BlockLevel,
    yb8: c_int,
    xb8: c_int,
) -> u8 {
    // the right-most ("index zero") bit of the partition represents the 8x8 block level,
    // but the BlockLevel enum represents the variants numerically in the opposite order
    // (128x128 = 0, 8x8 = 4). The shift reverses the ordering.
    let has_bl = |x| (x >> (4 - bl as u8)) & 1;
    has_bl(*a.partition.index(xb8 as usize)) + 2 * has_bl(*l.partition.index(yb8 as usize))
}

#[inline]
pub fn gather_left_partition_prob(r#in: &[u16; 16], bl: BlockLevel) -> u32 {
    let mut out =
        r#in[BlockPartition::H as usize - 1] as i32 - r#in[BlockPartition::H as usize] as i32;
    // Exploit the fact that cdfs for BlockPartition::Split, BlockPartition::TopSplit,
    // BlockPartition::BottomSplit and BlockPartition::LeftSplit are neighbors.
    out += r#in[BlockPartition::Split as usize - 1] as i32
        - r#in[BlockPartition::LeftSplit as usize] as i32;
    if bl != BlockLevel::Bl128x128 {
        out +=
            r#in[BlockPartition::H4 as usize - 1] as i32 - r#in[BlockPartition::H4 as usize] as i32;
    }
    out as u32
}

#[inline]
pub fn gather_top_partition_prob(r#in: &[u16; 16], bl: BlockLevel) -> u32 {
    // Exploit the fact that cdfs for BlockPartition::V, BlockPartition::Split and
    // BlockPartition::TopSplit are neighbors.
    let mut out = r#in[BlockPartition::V as usize - 1] as i32
        - r#in[BlockPartition::TopSplit as usize] as i32;
    // Exploit the facts that cdfs for BlockPartition::LeftSplit and
    // BlockPartition::RightSplit are neighbors, the probability for
    // BlockPartition::V4 is always zero, and the probability for
    // BlockPartition::RightSplit is zero in 128x128 blocks.
    out += r#in[BlockPartition::LeftSplit as usize - 1] as i32;
    if bl != BlockLevel::Bl128x128 {
        out += r#in[BlockPartition::V4 as usize - 1] as i32
            - r#in[BlockPartition::RightSplit as usize] as i32;
    }
    out as u32
}

#[inline]
pub fn get_uv_inter_txtp(uvt_dim: &TxfmInfo, ytxtp: TxfmType) -> TxfmType {
    if uvt_dim.max == TxfmSize::S32x32 as _ {
        return if ytxtp == IDTX { IDTX } else { DCT_DCT };
    }
    if uvt_dim.min == TxfmSize::S16x16 as _
        && ((1 << ytxtp as u8)
            & ((1 << H_FLIPADST) | (1 << V_FLIPADST) | (1 << H_ADST) | (1 << V_ADST)))
            != 0
    {
        return DCT_DCT;
    }

    ytxtp
}

#[inline]
pub fn get_filter_ctx(
    a: &BlockContext,
    l: &BlockContext,
    comp: bool,
    dir: bool,
    r#ref: i8,
    yb4: c_int,
    xb4: c_int,
) -> u8 {
    let [a_filter, l_filter] = [(a, xb4), (l, yb4)].map(|(al, b4)| {
        if *al.r#ref[0].index(b4 as usize) == r#ref || *al.r#ref[1].index(b4 as usize) == r#ref {
            *al.filter[dir as usize].index(b4 as usize)
        } else {
            Rav1dFilterMode::N_SWITCHABLE_FILTERS
        }
    });

    (comp as u8) * 4
        + (if a_filter == l_filter {
            a_filter
        } else if a_filter == Rav1dFilterMode::N_SWITCHABLE_FILTERS {
            l_filter
        } else if l_filter == Rav1dFilterMode::N_SWITCHABLE_FILTERS {
            a_filter
        } else {
            Rav1dFilterMode::N_SWITCHABLE_FILTERS
        } as u8)
}

#[inline]
pub fn get_comp_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: c_int,
    xb4: c_int,
    have_top: bool,
    have_left: bool,
) -> u8 {
    if have_top {
        if have_left {
            if a.comp_type.index(xb4 as usize).is_some() {
                if l.comp_type.index(yb4 as usize).is_some() {
                    4
                } else {
                    // 4U means intra (-1) or bwd (>= 4)
                    2 + (*l.r#ref[0].index(yb4 as usize) as c_uint >= 4) as u8
                }
            } else if l.comp_type.index(yb4 as usize).is_some() {
                // 4U means intra (-1) or bwd (>= 4)
                2 + (*a.r#ref[0].index(xb4 as usize) as c_uint >= 4) as u8
            } else {
                ((*l.r#ref[0].index(yb4 as usize) >= 4) ^ (*a.r#ref[0].index(xb4 as usize) >= 4))
                    as u8
            }
        } else {
            if a.comp_type.index(xb4 as usize).is_some() {
                3
            } else {
                (*a.r#ref[0].index(xb4 as usize) >= 4) as u8
            }
        }
    } else if have_left {
        if l.comp_type.index(yb4 as usize).is_some() {
            3
        } else {
            (*l.r#ref[0].index(yb4 as usize) >= 4) as u8
        }
    } else {
        1
    }
}

#[inline]
pub fn get_comp_dir_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: c_int,
    xb4: c_int,
    have_top: bool,
    have_left: bool,
) -> u8 {
    let has_uni_comp = |edge: &BlockContext, off| {
        (*edge.r#ref[0].index(off as usize) < 4) == (*edge.r#ref[1].index(off as usize) < 4)
    };

    if have_top && have_left {
        let a_intra = *a.intra.index(xb4 as usize) != 0;
        let l_intra = *l.intra.index(yb4 as usize) != 0;

        if a_intra && l_intra {
            return 2;
        }
        if a_intra || l_intra {
            let edge = if a_intra { &l } else { &a };
            let off = if a_intra { yb4 } else { xb4 };

            if edge.comp_type.index(off as usize).is_none() {
                return 2;
            }
            return 1 + 2 * has_uni_comp(edge, off) as u8;
        }

        let a_comp = a.comp_type.index(xb4 as usize).is_some();
        let l_comp = l.comp_type.index(yb4 as usize).is_some();
        let a_ref0 = *a.r#ref[0].index(xb4 as usize);
        let l_ref0 = *l.r#ref[0].index(yb4 as usize);

        if !a_comp && !l_comp {
            return 1 + 2 * ((a_ref0 >= 4) == (l_ref0 >= 4)) as u8;
        } else if !a_comp || !l_comp {
            let edge = if a_comp { &a } else { &l };
            let off = if a_comp { xb4 } else { yb4 };

            if !has_uni_comp(edge, off) {
                return 1;
            }
            return 3 + ((a_ref0 >= 4) == (l_ref0 >= 4)) as u8;
        } else {
            let a_uni = has_uni_comp(&a, xb4);
            let l_uni = has_uni_comp(&l, yb4);

            if !a_uni && !l_uni {
                return 0;
            }
            if !a_uni || !l_uni {
                return 2;
            }
            return 3 + ((a_ref0 == 4) == (l_ref0 == 4)) as u8;
        }
    } else if have_top || have_left {
        let edge = if have_left { l } else { a };
        let off = if have_left { yb4 } else { xb4 };

        if *edge.intra.index(off as usize) != 0 {
            return 2;
        }
        if edge.comp_type.index(off as usize).is_none() {
            return 2;
        }
        return 4 * has_uni_comp(&edge, off) as u8;
    } else {
        return 2;
    };
}

#[inline]
pub fn get_poc_diff(order_hint_n_bits: u8, poc0: c_int, poc1: c_int) -> c_int {
    if order_hint_n_bits == 0 {
        return 0;
    }
    let mask = 1 << order_hint_n_bits - 1;
    let diff = poc0 - poc1;
    return (diff & mask - 1) - (diff & mask);
}

#[inline]
pub fn get_jnt_comp_ctx(
    order_hint_n_bits: u8,
    poc: c_uint,
    ref0poc: c_uint,
    ref1poc: c_uint,
    a: &BlockContext,
    l: &BlockContext,
    yb4: c_int,
    xb4: c_int,
) -> u8 {
    let d0 = get_poc_diff(order_hint_n_bits, ref0poc as c_int, poc as c_int).abs();
    let d1 = get_poc_diff(order_hint_n_bits, poc as c_int, ref1poc as c_int).abs();
    let offset = (d0 == d1) as u8;
    let [a_ctx, l_ctx] = [(a, xb4), (l, yb4)].map(|(al, b4)| {
        (*al.comp_type.index(b4 as usize) >= Some(CompInterType::Avg)
            || *al.r#ref[0].index(b4 as usize) == 6) as u8
    });

    3 * offset + a_ctx + l_ctx
}

#[inline]
pub fn get_mask_comp_ctx(a: &BlockContext, l: &BlockContext, yb4: c_int, xb4: c_int) -> u8 {
    let [a_ctx, l_ctx] = [(a, xb4), (l, yb4)].map(|(al, b4)| {
        if *al.comp_type.index(b4 as usize) >= Some(CompInterType::Seg) {
            1
        } else if *al.r#ref[0].index(b4 as usize) == 6 {
            3
        } else {
            0
        }
    });

    cmp::min(a_ctx + l_ctx, 5)
}

fn cmp_counts(c1: u8, c2: u8) -> u8 {
    use Ordering::*;
    match c1.cmp(&c2) {
        Less => 0,
        Equal => 1,
        Greater => 2,
    }
}

#[inline]
pub fn av1_get_ref_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: c_int,
    xb4: c_int,
    have_top: bool,
    have_left: bool,
) -> u8 {
    let mut cnt = [0; 2];

    if have_top && *a.intra.index(xb4 as usize) == 0 {
        cnt[(*a.r#ref[0].index(xb4 as usize) >= 4) as usize] += 1;
        if a.comp_type.index(xb4 as usize).is_some() {
            cnt[(*a.r#ref[1].index(xb4 as usize) >= 4) as usize] += 1;
        }
    }

    if have_left && *l.intra.index(yb4 as usize) == 0 {
        cnt[(*l.r#ref[0].index(yb4 as usize) >= 4) as usize] += 1;
        if l.comp_type.index(yb4 as usize).is_some() {
            cnt[(*l.r#ref[1].index(yb4 as usize) >= 4) as usize] += 1;
        }
    }

    cmp_counts(cnt[0], cnt[1])
}

#[inline]
pub fn av1_get_fwd_ref_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: c_int,
    xb4: c_int,
    have_top: bool,
    have_left: bool,
) -> u8 {
    let mut cnt = [0; 4];

    if have_top && *a.intra.index(xb4 as usize) == 0 {
        let ref0 = *a.r#ref[0].index(xb4 as usize);
        if ref0 < 4 {
            cnt[ref0 as usize] += 1;
        }
        let ref1 = *a.r#ref[1].index(xb4 as usize);
        if a.comp_type.index(xb4 as usize).is_some() && ref1 < 4 {
            cnt[ref1 as usize] += 1;
        }
    }

    if have_left && *l.intra.index(yb4 as usize) == 0 {
        let ref0 = *l.r#ref[0].index(yb4 as usize);
        if ref0 < 4 {
            cnt[ref0 as usize] += 1;
        }
        let ref1 = *l.r#ref[1].index(yb4 as usize);
        if l.comp_type.index(yb4 as usize).is_some() && ref1 < 4 {
            cnt[ref1 as usize] += 1;
        }
    }

    cnt[0] += cnt[1];
    cnt[2] += cnt[3];

    cmp_counts(cnt[0], cnt[2])
}

#[inline]
pub fn av1_get_fwd_ref_1_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: c_int,
    xb4: c_int,
    have_top: bool,
    have_left: bool,
) -> u8 {
    let mut cnt = [0; 2];

    if have_top && *a.intra.index(xb4 as usize) == 0 {
        let ref0 = *a.r#ref[0].index(xb4 as usize);
        if ref0 < 2 {
            cnt[ref0 as usize] += 1;
        }
        let ref1 = *a.r#ref[1].index(xb4 as usize);
        if a.comp_type.index(xb4 as usize).is_some() && ref1 < 2 {
            cnt[ref1 as usize] += 1;
        }
    }

    if have_left && *l.intra.index(yb4 as usize) == 0 {
        let ref0 = *l.r#ref[0].index(yb4 as usize);
        if ref0 < 2 {
            cnt[ref0 as usize] += 1;
        }
        let ref1 = *l.r#ref[1].index(yb4 as usize);
        if l.comp_type.index(yb4 as usize).is_some() && ref1 < 2 {
            cnt[ref1 as usize] += 1;
        }
    }

    cmp_counts(cnt[0], cnt[1])
}

#[inline]
pub fn av1_get_fwd_ref_2_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: c_int,
    xb4: c_int,
    have_top: bool,
    have_left: bool,
) -> u8 {
    let mut cnt = [0; 2];

    if have_top && *a.intra.index(xb4 as usize) == 0 {
        let ref0 = *a.r#ref[0].index(xb4 as usize);
        if (ref0 ^ 2) < 2 {
            cnt[(ref0 - 2) as usize] += 1;
        }
        let ref1 = *a.r#ref[1].index(xb4 as usize);
        if a.comp_type.index(xb4 as usize).is_some() && (ref1 ^ 2) < 2 {
            cnt[(ref1 - 2) as usize] += 1;
        }
    }

    if have_left && *l.intra.index(yb4 as usize) == 0 {
        let ref0 = *l.r#ref[0].index(yb4 as usize);
        if (ref0 ^ 2) < 2 {
            cnt[(ref0 - 2) as usize] += 1;
        }
        let ref1 = *l.r#ref[1].index(yb4 as usize);
        if l.comp_type.index(yb4 as usize).is_some() && (ref1 ^ 2) < 2 {
            cnt[(ref1 - 2) as usize] += 1;
        }
    }

    cmp_counts(cnt[0], cnt[1])
}

#[inline]
pub fn av1_get_bwd_ref_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: c_int,
    xb4: c_int,
    have_top: bool,
    have_left: bool,
) -> u8 {
    let mut cnt = [0; 3];

    if have_top && *a.intra.index(xb4 as usize) == 0 {
        let ref0 = *a.r#ref[0].index(xb4 as usize);
        if ref0 >= 4 {
            cnt[(ref0 - 4) as usize] += 1;
        }
        let ref1 = *a.r#ref[1].index(xb4 as usize);
        if a.comp_type.index(xb4 as usize).is_some() && ref1 >= 4 {
            cnt[(ref1 - 4) as usize] += 1;
        }
    }

    if have_left && *l.intra.index(yb4 as usize) == 0 {
        let ref0 = *l.r#ref[0].index(yb4 as usize);
        if ref0 >= 4 {
            cnt[(ref0 - 4) as usize] += 1;
        }
        let ref1 = *l.r#ref[1].index(yb4 as usize);
        if l.comp_type.index(yb4 as usize).is_some() && ref1 >= 4 {
            cnt[(ref1 - 4) as usize] += 1;
        }
    }

    cnt[1] += cnt[0];

    cmp_counts(cnt[1], cnt[2])
}

#[inline]
pub fn av1_get_bwd_ref_1_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: c_int,
    xb4: c_int,
    have_top: bool,
    have_left: bool,
) -> u8 {
    let mut cnt = [0; 3];

    if have_top && *a.intra.index(xb4 as usize) == 0 {
        let ref0 = *a.r#ref[0].index(xb4 as usize);
        if ref0 >= 4 {
            cnt[(ref0 - 4) as usize] += 1;
        }
        let ref1 = *a.r#ref[1].index(xb4 as usize);
        if a.comp_type.index(xb4 as usize).is_some() && ref1 >= 4 {
            cnt[(ref1 - 4) as usize] += 1;
        }
    }

    if have_left && *l.intra.index(yb4 as usize) == 0 {
        let ref0 = *l.r#ref[0].index(yb4 as usize);
        if ref0 >= 4 {
            cnt[(ref0 - 4) as usize] += 1;
        }
        let ref1 = *l.r#ref[1].index(yb4 as usize);
        if l.comp_type.index(yb4 as usize).is_some() && ref1 >= 4 {
            cnt[(ref1 - 4) as usize] += 1;
        }
    }

    cmp_counts(cnt[0], cnt[1])
}

#[inline]
pub fn av1_get_uni_p1_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: c_int,
    xb4: c_int,
    have_top: bool,
    have_left: bool,
) -> u8 {
    let mut cnt = [0; 3];

    if have_top && *a.intra.index(xb4 as usize) == 0 {
        if let Some(cnt) = cnt.get_mut((*a.r#ref[0].index(xb4 as usize) - 1) as usize) {
            *cnt += 1;
        }
        if a.comp_type.index(xb4 as usize).is_some() {
            if let Some(cnt) = cnt.get_mut((*a.r#ref[1].index(xb4 as usize) - 1) as usize) {
                *cnt += 1;
            }
        }
    }

    if have_left && *l.intra.index(yb4 as usize) == 0 {
        if let Some(cnt) = cnt.get_mut((*l.r#ref[0].index(yb4 as usize) - 1) as usize) {
            *cnt += 1;
        }
        if l.comp_type.index(yb4 as usize).is_some() {
            if let Some(cnt) = cnt.get_mut((*l.r#ref[1].index(yb4 as usize) - 1) as usize) {
                *cnt += 1;
            }
        }
    }

    cnt[1] += cnt[2];

    cmp_counts(cnt[0], cnt[1])
}

#[inline]
pub fn get_drl_context(ref_mv_stack: &[RefMvsCandidate; 8], ref_idx: usize) -> c_int {
    if ref_mv_stack[ref_idx].weight >= 640 {
        (ref_mv_stack[ref_idx + 1].weight < 640) as c_int
    } else if ref_mv_stack[ref_idx + 1].weight < 640 {
        2
    } else {
        0
    }
}

#[inline]
pub fn get_cur_frame_segid(
    b: Bxy,
    have_top: bool,
    have_left: bool,
    cur_seg_map: &DisjointMutSlice<SegmentId>,
    stride: usize,
) -> (SegmentId, u8) {
    let negative_adjustment = have_left as usize + have_top as usize * stride;
    let offset = b.x as usize + b.y as usize * stride - negative_adjustment;
    match (have_left, have_top) {
        (true, true) => {
            let l = *cur_seg_map.index(offset + stride);
            let a = *cur_seg_map.index(offset + 1);
            let al = *cur_seg_map.index(offset);
            let seg_ctx = if l == a && al == l {
                2
            } else if l == a || al == l || a == al {
                1
            } else {
                0
            };
            let seg_id = if a == al { a } else { l };
            (seg_id, seg_ctx)
        }
        (true, false) | (false, true) => (*cur_seg_map.index(offset), 0),
        (false, false) => (Default::default(), 0),
    }
}

#[inline]
fn fix_int_mv_precision(mv: &mut Mv) {
    mv.x = (mv.x - (mv.x >> 15) + 3) & !7;
    mv.y = (mv.y - (mv.y >> 15) + 3) & !7;
}

#[inline]
pub(crate) fn fix_mv_precision(hdr: &Rav1dFrameHeader, mv: &mut Mv) {
    if hdr.force_integer_mv {
        fix_int_mv_precision(mv);
    } else if !(*hdr).hp {
        mv.x = (mv.x - (mv.x >> 15)) & !1;
        mv.y = (mv.y - (mv.y >> 15)) & !1;
    }
}

#[inline]
pub(crate) fn get_gmv_2d(
    gmv: &Rav1dWarpedMotionParams,
    bx4: c_int,
    by4: c_int,
    bw4: c_int,
    bh4: c_int,
    hdr: &Rav1dFrameHeader,
) -> Mv {
    match gmv.r#type {
        Rav1dWarpedMotionType::RotZoom => {
            assert!(gmv.matrix[5] == gmv.matrix[2]);
            assert!(gmv.matrix[4] == -gmv.matrix[3]);
        }
        Rav1dWarpedMotionType::Translation => {
            let mut res = Mv {
                y: (gmv.matrix[0] >> 13) as i16,
                x: (gmv.matrix[1] >> 13) as i16,
            };
            if hdr.force_integer_mv {
                fix_int_mv_precision(&mut res);
            }
            return res;
        }
        Rav1dWarpedMotionType::Identity => {
            return Mv::ZERO;
        }
        Rav1dWarpedMotionType::Affine => {}
    }
    let x = bx4 * 4 + bw4 * 2 - 1;
    let y = by4 * 4 + bh4 * 2 - 1;
    let xc = (gmv.matrix[2] - (1 << 16)) * x + gmv.matrix[3] * y + gmv.matrix[0];
    let yc = (gmv.matrix[5] - (1 << 16)) * y + gmv.matrix[4] * x + gmv.matrix[1];
    let shift = 16 - (3 - !hdr.hp as c_int);
    let round = 1 << shift >> 1;
    let mut res = Mv {
        y: apply_sign(yc.abs() + round >> shift << !hdr.hp as c_int, yc) as i16,
        x: apply_sign(xc.abs() + round >> shift << !hdr.hp as c_int, xc) as i16,
    };
    if hdr.force_integer_mv {
        fix_int_mv_precision(&mut res);
    }
    return res;
}
