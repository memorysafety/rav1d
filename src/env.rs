use std::cmp::Ordering;

use crate::include::common::intops::apply_sign;
use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::dav1d::headers::Dav1dWarpedMotionParams;
use crate::include::dav1d::headers::DAV1D_N_SWITCHABLE_FILTERS;
use crate::include::stddef::ptrdiff_t;
use crate::include::stdint::int8_t;
use crate::include::stdint::uint8_t;
use crate::src::align::Align8;
use crate::src::levels::mv;
use crate::src::levels::BlockLevel;
use crate::src::levels::TxfmSize;
use crate::src::levels::TxfmType;
use crate::src::levels::BL_128X128;
use crate::src::levels::COMP_INTER_AVG;
use crate::src::levels::COMP_INTER_NONE;
use crate::src::levels::COMP_INTER_SEG;
use crate::src::levels::DCT_DCT;
use crate::src::levels::H_ADST;
use crate::src::levels::H_FLIPADST;
use crate::src::levels::IDTX;
use crate::src::levels::PARTITION_H;
use crate::src::levels::PARTITION_H4;
use crate::src::levels::PARTITION_SPLIT;
use crate::src::levels::PARTITION_T_LEFT_SPLIT;
use crate::src::levels::PARTITION_T_RIGHT_SPLIT;
use crate::src::levels::PARTITION_T_TOP_SPLIT;
use crate::src::levels::PARTITION_V;
use crate::src::levels::PARTITION_V4;
use crate::src::levels::TX_16X16;
use crate::src::levels::TX_32X32;
use crate::src::levels::V_ADST;
use crate::src::levels::V_FLIPADST;
use crate::src::refmvs::refmvs_candidate;
use crate::src::tables::TxfmInfo;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockContext {
    pub mode: Align8<[uint8_t; 32]>,
    pub lcoef: Align8<[uint8_t; 32]>,
    pub ccoef: Align8<[[uint8_t; 32]; 2]>,
    pub seg_pred: Align8<[uint8_t; 32]>,
    pub skip: Align8<[uint8_t; 32]>,
    pub skip_mode: Align8<[uint8_t; 32]>,
    pub intra: Align8<[uint8_t; 32]>,
    pub comp_type: Align8<[uint8_t; 32]>,
    pub r#ref: Align8<[[int8_t; 32]; 2]>,
    pub filter: Align8<[[uint8_t; 32]; 2]>,
    pub tx_intra: Align8<[int8_t; 32]>,
    pub tx: Align8<[int8_t; 32]>,
    pub tx_lpf_y: Align8<[uint8_t; 32]>,
    pub tx_lpf_uv: [uint8_t; 32],
    pub partition: [uint8_t; 16],
    pub uvmode: [uint8_t; 32],
    pub pal_sz: [uint8_t; 32],
}

#[inline]
pub fn get_intra_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: libc::c_int,
    xb4: libc::c_int,
    have_top: bool,
    have_left: bool,
) -> u8 {
    if have_left {
        if have_top {
            let ctx = l.intra[yb4 as usize] + a.intra[xb4 as usize];
            ctx + (ctx == 2) as u8
        } else {
            l.intra[yb4 as usize] * 2
        }
    } else {
        if have_top {
            a.intra[xb4 as usize] * 2
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
    yb4: libc::c_int,
    xb4: libc::c_int,
) -> u8 {
    (l.tx_intra[yb4 as usize] as i32 >= max_tx.lh as i32) as u8
        + (a.tx_intra[xb4 as usize] as i32 >= max_tx.lw as i32) as u8
}

#[inline]
pub fn get_partition_ctx(
    a: &BlockContext,
    l: &BlockContext,
    bl: BlockLevel,
    yb8: libc::c_int,
    xb8: libc::c_int,
) -> u8 {
    (a.partition[xb8 as usize] >> (4 - bl) & 1) + ((l.partition[yb8 as usize] >> (4 - bl) & 1) << 1)
}

#[inline]
pub fn gather_left_partition_prob(r#in: &[u16; 16], bl: BlockLevel) -> u32 {
    let mut out = r#in[(PARTITION_H - 1) as usize] as i32 - r#in[PARTITION_H as usize] as i32;
    // Exploit the fact that cdfs for PARTITION_SPLIT, PARTITION_T_TOP_SPLIT,
    // PARTITION_T_BOTTOM_SPLIT and PARTITION_T_LEFT_SPLIT are neighbors.
    out +=
        r#in[(PARTITION_SPLIT - 1) as usize] as i32 - r#in[PARTITION_T_LEFT_SPLIT as usize] as i32;
    if bl != BL_128X128 {
        out += r#in[(PARTITION_H4 - 1) as usize] as i32 - r#in[PARTITION_H4 as usize] as i32;
    }
    out as u32
}

#[inline]
pub fn gather_top_partition_prob(r#in: &[u16; 16], bl: BlockLevel) -> u32 {
    // Exploit the fact that cdfs for PARTITION_V, PARTITION_SPLIT and
    // PARTITION_T_TOP_SPLIT are neighbors.
    let mut out =
        r#in[(PARTITION_V - 1) as usize] as i32 - r#in[PARTITION_T_TOP_SPLIT as usize] as i32;
    // Exploit the facts that cdfs for PARTITION_T_LEFT_SPLIT and
    // PARTITION_T_RIGHT_SPLIT are neighbors, the probability for
    // PARTITION_V4 is always zero, and the probability for
    // PARTITION_T_RIGHT_SPLIT is zero in 128x128 blocks.
    out += r#in[(PARTITION_T_LEFT_SPLIT - 1) as usize] as i32;
    if bl != BL_128X128 {
        out += r#in[(PARTITION_V4 - 1) as usize] as i32
            - r#in[PARTITION_T_RIGHT_SPLIT as usize] as i32;
    }
    out as u32
}

#[inline]
pub fn get_uv_inter_txtp(uvt_dim: &TxfmInfo, ytxtp: TxfmType) -> TxfmType {
    if (*uvt_dim).max as TxfmSize == TX_32X32 {
        return if ytxtp == IDTX { IDTX } else { DCT_DCT };
    }
    if (*uvt_dim).min as TxfmSize == TX_16X16
        && ((1 << ytxtp) & ((1 << H_FLIPADST) | (1 << V_FLIPADST) | (1 << H_ADST) | (1 << V_ADST)))
            != 0
    {
        return DCT_DCT;
    }

    return ytxtp;
}

#[inline]
pub fn get_filter_ctx(
    a: &BlockContext,
    l: &BlockContext,
    comp: bool,
    dir: bool,
    r#ref: i8,
    yb4: libc::c_int,
    xb4: libc::c_int,
) -> u8 {
    let [a_filter, l_filter] = [(a, xb4), (l, yb4)].map(|(al, b4)| {
        if al.r#ref[0][b4 as usize] == r#ref || al.r#ref[1][b4 as usize] == r#ref {
            al.filter[dir as usize][b4 as usize]
        } else {
            DAV1D_N_SWITCHABLE_FILTERS as u8
        }
    });

    (comp as u8) * 4
        + if a_filter == l_filter {
            a_filter
        } else if a_filter == DAV1D_N_SWITCHABLE_FILTERS as u8 {
            l_filter
        } else if l_filter == DAV1D_N_SWITCHABLE_FILTERS as u8 {
            a_filter
        } else {
            DAV1D_N_SWITCHABLE_FILTERS as u8
        }
}

#[inline]
pub fn get_comp_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: libc::c_int,
    xb4: libc::c_int,
    have_top: bool,
    have_left: bool,
) -> u8 {
    if have_top {
        if have_left {
            if a.comp_type[xb4 as usize] != 0 {
                if l.comp_type[yb4 as usize] != 0 {
                    4
                } else {
                    // 4U means intra (-1) or bwd (>= 4)
                    2 + (l.r#ref[0][yb4 as usize] as libc::c_uint >= 4) as u8
                }
            } else if l.comp_type[yb4 as usize] != 0 {
                // 4U means intra (-1) or bwd (>= 4)
                2 + (a.r#ref[0][xb4 as usize] as libc::c_uint >= 4) as u8
            } else {
                ((l.r#ref[0][yb4 as usize] >= 4) ^ (a.r#ref[0][xb4 as usize] >= 4)) as u8
            }
        } else {
            if a.comp_type[xb4 as usize] != 0 {
                3
            } else {
                (a.r#ref[0][xb4 as usize] >= 4) as u8
            }
        }
    } else if have_left {
        if l.comp_type[yb4 as usize] != 0 {
            3
        } else {
            (l.r#ref[0][yb4 as usize] >= 4) as u8
        }
    } else {
        1
    }
}

#[inline]
pub fn get_comp_dir_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: libc::c_int,
    xb4: libc::c_int,
    have_top: bool,
    have_left: bool,
) -> u8 {
    let has_uni_comp = |edge: &BlockContext, off| {
        (edge.r#ref[0][off as usize] < 4) == (edge.r#ref[1][off as usize] < 4)
    };

    if have_top && have_left {
        let a_intra = a.intra[xb4 as usize] != 0;
        let l_intra = l.intra[yb4 as usize] != 0;

        if a_intra && l_intra {
            return 2;
        }
        if a_intra || l_intra {
            let edge = if a_intra { l } else { a };
            let off = if a_intra { yb4 } else { xb4 };

            if edge.comp_type[off as usize] == COMP_INTER_NONE as u8 {
                return 2;
            }
            return 1 + 2 * has_uni_comp(edge, off) as u8;
        }

        let a_comp = a.comp_type[xb4 as usize] != COMP_INTER_NONE as u8;
        let l_comp = l.comp_type[yb4 as usize] != COMP_INTER_NONE as u8;
        let a_ref0 = a.r#ref[0][xb4 as usize];
        let l_ref0 = l.r#ref[0][yb4 as usize];

        if !a_comp && !l_comp {
            return 1 + 2 * ((a_ref0 >= 4) == (l_ref0 >= 4)) as u8;
        } else if !a_comp || !l_comp {
            let edge = if a_comp { a } else { l };
            let off = if a_comp { xb4 } else { yb4 };

            if !has_uni_comp(edge, off) {
                return 1;
            }
            return 3 + ((a_ref0 >= 4) == (l_ref0 >= 4)) as u8;
        } else {
            let a_uni = has_uni_comp(a, xb4);
            let l_uni = has_uni_comp(l, yb4);

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

        if edge.intra[off as usize] != 0 {
            return 2;
        }
        if edge.comp_type[off as usize] == COMP_INTER_NONE as u8 {
            return 2;
        }
        return 4 * has_uni_comp(edge, off) as u8;
    } else {
        return 2;
    };
}

#[inline]
pub fn get_poc_diff(
    order_hint_n_bits: libc::c_int,
    poc0: libc::c_int,
    poc1: libc::c_int,
) -> libc::c_int {
    if order_hint_n_bits == 0 {
        return 0;
    }
    let mask = 1 << order_hint_n_bits - 1;
    let diff = poc0 - poc1;
    return (diff & mask - 1) - (diff & mask);
}

#[inline]
pub fn get_jnt_comp_ctx(
    order_hint_n_bits: libc::c_int,
    poc: libc::c_uint,
    ref0poc: libc::c_uint,
    ref1poc: libc::c_uint,
    a: &BlockContext,
    l: &BlockContext,
    yb4: libc::c_int,
    xb4: libc::c_int,
) -> u8 {
    let d0 = get_poc_diff(
        order_hint_n_bits,
        ref0poc as libc::c_int,
        poc as libc::c_int,
    )
    .abs();
    let d1 = get_poc_diff(
        order_hint_n_bits,
        poc as libc::c_int,
        ref1poc as libc::c_int,
    )
    .abs();
    let offset = (d0 == d1) as u8;
    let [a_ctx, l_ctx] = [(a, xb4), (l, yb4)].map(|(al, b4)| {
        (al.comp_type[b4 as usize] >= COMP_INTER_AVG as u8 || al.r#ref[0][b4 as usize] == 6) as u8
    });

    3 * offset + a_ctx + l_ctx
}

#[inline]
pub fn get_mask_comp_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: libc::c_int,
    xb4: libc::c_int,
) -> u8 {
    let [a_ctx, l_ctx] = [(a, xb4), (l, yb4)].map(|(al, b4)| {
        if al.comp_type[b4 as usize] >= COMP_INTER_SEG as u8 {
            1
        } else if al.r#ref[0][b4 as usize] == 6 {
            3
        } else {
            0
        }
    });

    std::cmp::min(a_ctx + l_ctx, 5)
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
    yb4: libc::c_int,
    xb4: libc::c_int,
    mut have_top: bool,
    mut have_left: bool,
) -> u8 {
    let mut cnt = [0; 2];

    if have_top && a.intra[xb4 as usize] == 0 {
        cnt[(a.r#ref[0][xb4 as usize] >= 4) as usize] += 1;
        if a.comp_type[xb4 as usize] != 0 {
            cnt[(a.r#ref[1][xb4 as usize] >= 4) as usize] += 1;
        }
    }

    if have_left && l.intra[yb4 as usize] == 0 {
        cnt[(l.r#ref[0][yb4 as usize] >= 4) as usize] += 1;
        if l.comp_type[yb4 as usize] != 0 {
            cnt[(l.r#ref[1][yb4 as usize] >= 4) as usize] += 1;
        }
    }

    cmp_counts(cnt[0], cnt[1])
}

#[inline]
pub fn av1_get_fwd_ref_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: libc::c_int,
    xb4: libc::c_int,
    have_top: bool,
    have_left: bool,
) -> u8 {
    let mut cnt = [0; 4];

    if have_top && a.intra[xb4 as usize] == 0 {
        if a.r#ref[0][xb4 as usize] < 4 {
            cnt[a.r#ref[0][xb4 as usize] as usize] += 1;
        }
        if a.comp_type[xb4 as usize] != 0 && a.r#ref[1][xb4 as usize] < 4 {
            cnt[a.r#ref[1][xb4 as usize] as usize] += 1;
        }
    }

    if have_left && l.intra[yb4 as usize] == 0 {
        if l.r#ref[0][yb4 as usize] < 4 {
            cnt[l.r#ref[0][yb4 as usize] as usize] += 1;
        }
        if l.comp_type[yb4 as usize] != 0 && l.r#ref[1][yb4 as usize] < 4 {
            cnt[l.r#ref[1][yb4 as usize] as usize] += 1;
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
    yb4: libc::c_int,
    xb4: libc::c_int,
    have_top: bool,
    have_left: bool,
) -> u8 {
    let mut cnt = [0; 2];

    if have_top && a.intra[xb4 as usize] == 0 {
        if a.r#ref[0][xb4 as usize] < 2 {
            cnt[a.r#ref[0][xb4 as usize] as usize] += 1;
        }
        if a.comp_type[xb4 as usize] != 0 && a.r#ref[1][xb4 as usize] < 2 {
            cnt[a.r#ref[1][xb4 as usize] as usize] += 1;
        }
    }

    if have_left && l.intra[yb4 as usize] == 0 {
        if l.r#ref[0][yb4 as usize] < 2 {
            cnt[l.r#ref[0][yb4 as usize] as usize] += 1;
        }
        if l.comp_type[yb4 as usize] != 0 && l.r#ref[1][yb4 as usize] < 2 {
            cnt[l.r#ref[1][yb4 as usize] as usize] += 1;
        }
    }

    cmp_counts(cnt[0], cnt[1])
}

#[inline]
pub fn av1_get_fwd_ref_2_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: libc::c_int,
    xb4: libc::c_int,
    have_top: bool,
    have_left: bool,
) -> u8 {
    let mut cnt = [0; 2];

    if have_top && a.intra[xb4 as usize] == 0 {
        if (a.r#ref[0][xb4 as usize] ^ 2) < 2 {
            cnt[(a.r#ref[0][xb4 as usize] - 2) as usize] += 1;
        }
        if a.comp_type[xb4 as usize] != 0 && (a.r#ref[1][xb4 as usize] ^ 2) < 2 {
            cnt[(a.r#ref[1][xb4 as usize] - 2) as usize] += 1;
        }
    }

    if have_left && l.intra[yb4 as usize] == 0 {
        if (l.r#ref[0][yb4 as usize] ^ 2) < 2 {
            cnt[(l.r#ref[0][yb4 as usize] - 2) as usize] += 1;
        }
        if l.comp_type[yb4 as usize] != 0 && (l.r#ref[1][yb4 as usize] ^ 2) < 2 {
            cnt[(l.r#ref[1][yb4 as usize] - 2) as usize] += 1;
        }
    }

    cmp_counts(cnt[0], cnt[1])
}

#[inline]
pub fn av1_get_bwd_ref_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: libc::c_int,
    xb4: libc::c_int,
    have_top: bool,
    have_left: bool,
) -> u8 {
    let mut cnt = [0; 3];

    if have_top && a.intra[xb4 as usize] == 0 {
        if a.r#ref[0][xb4 as usize] >= 4 {
            cnt[(a.r#ref[0][xb4 as usize] - 4) as usize] += 1;
        }
        if a.comp_type[xb4 as usize] != 0 && a.r#ref[1][xb4 as usize] >= 4 {
            cnt[(a.r#ref[1][xb4 as usize] - 4) as usize] += 1;
        }
    }

    if have_left && l.intra[yb4 as usize] == 0 {
        if l.r#ref[0][yb4 as usize] >= 4 {
            cnt[(l.r#ref[0][yb4 as usize] - 4) as usize] += 1;
        }
        if l.comp_type[yb4 as usize] != 0 && l.r#ref[1][yb4 as usize] >= 4 {
            cnt[(l.r#ref[1][yb4 as usize] - 4) as usize] += 1;
        }
    }

    cnt[1] += cnt[0];

    cmp_counts(cnt[1], cnt[2])
}

#[inline]
pub fn av1_get_bwd_ref_1_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: libc::c_int,
    xb4: libc::c_int,
    have_top: bool,
    have_left: bool,
) -> u8 {
    let mut cnt = [0; 3];

    if have_top && a.intra[xb4 as usize] == 0 {
        if a.r#ref[0][xb4 as usize] >= 4 {
            cnt[(a.r#ref[0][xb4 as usize] - 4) as usize] += 1;
        }
        if a.comp_type[xb4 as usize] != 0 && a.r#ref[1][xb4 as usize] >= 4 {
            cnt[(a.r#ref[1][xb4 as usize] - 4) as usize] += 1;
        }
    }

    if have_left && l.intra[yb4 as usize] == 0 {
        if l.r#ref[0][yb4 as usize] >= 4 {
            cnt[(l.r#ref[0][yb4 as usize] - 4) as usize] += 1;
        }
        if l.comp_type[yb4 as usize] != 0 && l.r#ref[1][yb4 as usize] >= 4 {
            cnt[(l.r#ref[1][yb4 as usize] - 4) as usize] += 1;
        }
    }

    cmp_counts(cnt[0], cnt[1])
}

#[inline]
pub fn av1_get_uni_p1_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: libc::c_int,
    xb4: libc::c_int,
    have_top: bool,
    have_left: bool,
) -> u8 {
    let mut cnt = [0; 3];

    if have_top && a.intra[xb4 as usize] == 0 {
        if let Some(cnt) = cnt.get_mut((a.r#ref[0][xb4 as usize] - 1) as usize) {
            *cnt += 1;
        }
        if a.comp_type[xb4 as usize] != 0
                && let Some(cnt) = cnt.get_mut((a.r#ref[1][xb4 as usize] - 1) as usize) {
            *cnt += 1;
        }
    }

    if have_left && l.intra[yb4 as usize] == 0 {
        if let Some(cnt) = cnt.get_mut((l.r#ref[0][yb4 as usize] - 1) as usize) {
            *cnt += 1;
        }
        if l.comp_type[yb4 as usize] != 0
                && let Some(cnt) = cnt.get_mut((l.r#ref[1][yb4 as usize] - 1) as usize) {
            *cnt += 1;
        }
    }

    cnt[1] += cnt[2];

    cmp_counts(cnt[0], cnt[1])
}

#[inline]
pub fn get_drl_context(ref_mv_stack: &[refmvs_candidate; 8], ref_idx: usize) -> libc::c_int {
    if ref_mv_stack[ref_idx].weight >= 640 {
        (ref_mv_stack[ref_idx + 1].weight < 640) as libc::c_int
    } else if ref_mv_stack[ref_idx + 1].weight < 640 {
        2
    } else {
        0
    }
}

#[inline]
pub unsafe fn get_cur_frame_segid(
    by: libc::c_int,
    bx: libc::c_int,
    have_top: bool,
    have_left: bool,
    // It's very difficult to make this safe (a slice),
    // as it is negatively indexed
    // and it comes from [`Dav1dFrameContext::cur_segmap`],
    // which is set to [`Dav1dFrameContext::cur_segmap_ref`] and [`Dav1dFrameContext::prev_segmap_ref`],
    // which are [`Dav1dRef`]s, which have no size and are refcounted.
    mut cur_seg_map: *const u8,
    stride: ptrdiff_t,
) -> (u8, u8) {
    cur_seg_map = cur_seg_map.offset(bx as isize + by as isize * stride);
    if have_left && have_top {
        let l = *cur_seg_map.offset(-1);
        let a = *cur_seg_map.offset(-stride as isize);
        let al = *cur_seg_map.offset(-(stride + 1) as isize);
        let seg_ctx = if l == a && al == l {
            2
        } else if l == a || al == l || a == al {
            1
        } else {
            0
        };
        let seg_id = if a == al { a } else { l };
        (seg_id, seg_ctx)
    } else {
        let seg_ctx = 0;
        let seg_id = if have_left {
            *cur_seg_map.offset(-1)
        } else if have_top {
            *cur_seg_map.offset(-stride as isize)
        } else {
            0
        };
        (seg_id, seg_ctx)
    }
}

#[inline]
fn fix_int_mv_precision(mv: &mut mv) {
    mv.x = (mv.x - (mv.x >> 15) + 3) & !7;
    mv.y = (mv.y - (mv.y >> 15) + 3) & !7;
}

#[inline]
pub fn fix_mv_precision(hdr: &Dav1dFrameHeader, mv: &mut mv) {
    if hdr.force_integer_mv != 0 {
        fix_int_mv_precision(mv);
    } else if (*hdr).hp == 0 {
        mv.x = (mv.x - (mv.x >> 15)) & !1;
        mv.y = (mv.y - (mv.y >> 15)) & !1;
    }
}

#[inline]
pub fn get_gmv_2d(
    gmv: &Dav1dWarpedMotionParams,
    bx4: libc::c_int,
    by4: libc::c_int,
    bw4: libc::c_int,
    bh4: libc::c_int,
    hdr: &Dav1dFrameHeader,
) -> mv {
    match gmv.type_0 {
        2 => {
            assert!(gmv.matrix[5] == gmv.matrix[2]);
            assert!(gmv.matrix[4] == -gmv.matrix[3]);
        }
        1 => {
            let mut res = mv {
                y: (gmv.matrix[0] >> 13) as i16,
                x: (gmv.matrix[1] >> 13) as i16,
            };
            if hdr.force_integer_mv != 0 {
                fix_int_mv_precision(&mut res);
            }
            return res;
        }
        0 => {
            return mv::ZERO;
        }
        3 | _ => {}
    }
    let x = bx4 * 4 + bw4 * 2 - 1;
    let y = by4 * 4 + bh4 * 2 - 1;
    let xc = (gmv.matrix[2] - (1 << 16)) * x + gmv.matrix[3] * y + gmv.matrix[0];
    let yc = (gmv.matrix[5] - (1 << 16)) * y + gmv.matrix[4] * x + gmv.matrix[1];
    let shift = 16 - (3 - (hdr.hp == 0) as libc::c_int);
    let round = 1 << shift >> 1;
    let mut res = mv {
        y: apply_sign(
            yc.abs() + round >> shift << (hdr.hp == 0) as libc::c_int,
            yc,
        ) as i16,
        x: apply_sign(
            xc.abs() + round >> shift << (hdr.hp == 0) as libc::c_int,
            xc,
        ) as i16,
    };
    if hdr.force_integer_mv != 0 {
        fix_int_mv_precision(&mut res);
    }
    return res;
}
