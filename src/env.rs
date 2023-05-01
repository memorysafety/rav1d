use crate::include::common::intops::apply_sign;
use crate::include::common::intops::imin;
use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::dav1d::headers::Dav1dWarpedMotionParams;
use crate::include::dav1d::headers::DAV1D_N_SWITCHABLE_FILTERS;
use crate::include::stddef::ptrdiff_t;
use crate::include::stdint::int8_t;
use crate::include::stdint::uint8_t;
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
    pub mode: [uint8_t; 32],
    pub lcoef: [uint8_t; 32],
    pub ccoef: [[uint8_t; 32]; 2],
    pub seg_pred: [uint8_t; 32],
    pub skip: [uint8_t; 32],
    pub skip_mode: [uint8_t; 32],
    pub intra: [uint8_t; 32],
    pub comp_type: [uint8_t; 32],
    pub r#ref: [[int8_t; 32]; 2],
    pub filter: [[uint8_t; 32]; 2],
    pub tx_intra: [int8_t; 32],
    pub tx: [int8_t; 32],
    pub tx_lpf_y: [uint8_t; 32],
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
    comp: libc::c_int,
    dir: libc::c_int,
    r#ref: i8,
    yb4: libc::c_int,
    xb4: libc::c_int,
) -> libc::c_int {
    let a_filter = if a.r#ref[0][xb4 as usize] == r#ref || a.r#ref[1][xb4 as usize] == r#ref {
        a.filter[dir as usize][xb4 as usize] as libc::c_int
    } else {
        DAV1D_N_SWITCHABLE_FILTERS as libc::c_int
    };
    let l_filter = if l.r#ref[0][yb4 as usize] == r#ref || l.r#ref[1][yb4 as usize] == r#ref {
        l.filter[dir as usize][yb4 as usize] as libc::c_int
    } else {
        DAV1D_N_SWITCHABLE_FILTERS as libc::c_int
    };
    comp * 4 + if a_filter == l_filter {
        a_filter
    } else if a_filter == DAV1D_N_SWITCHABLE_FILTERS as libc::c_int {
        l_filter
    } else if l_filter == DAV1D_N_SWITCHABLE_FILTERS as libc::c_int {
        a_filter
    } else {
        DAV1D_N_SWITCHABLE_FILTERS as libc::c_int
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
) -> libc::c_int {
    if have_top {
        if have_left {
            if a.comp_type[xb4 as usize] != 0 {
                if l.comp_type[yb4 as usize] != 0 {
                    return 4 as libc::c_int;
                } else {
                    return 2 as libc::c_int
                        + (l.r#ref[0][yb4 as usize] as libc::c_uint >= 4 as libc::c_uint)
                            as libc::c_int;
                }
            } else if l.comp_type[yb4 as usize] != 0 {
                return 2 as libc::c_int
                    + (a.r#ref[0][xb4 as usize] as libc::c_uint >= 4 as libc::c_uint)
                        as libc::c_int;
            } else {
                return (l.r#ref[0][yb4 as usize] as libc::c_int >= 4) as libc::c_int
                    ^ (a.r#ref[0][xb4 as usize] as libc::c_int >= 4) as libc::c_int;
            }
        } else {
            return if a.comp_type[xb4 as usize] as libc::c_int != 0 {
                3 as libc::c_int
            } else {
                (a.r#ref[0][xb4 as usize] as libc::c_int >= 4) as libc::c_int
            };
        }
    } else if have_left {
        return if l.comp_type[yb4 as usize] as libc::c_int != 0 {
            3 as libc::c_int
        } else {
            (l.r#ref[0][yb4 as usize] as libc::c_int >= 4) as libc::c_int
        };
    } else {
        return 1 as libc::c_int;
    };
}

#[inline]
pub fn get_comp_dir_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: libc::c_int,
    xb4: libc::c_int,
    have_top: bool,
    have_left: bool,
) -> libc::c_int {
    if have_top && have_left {
        let a_intra = a.intra[xb4 as usize] as libc::c_int;
        let l_intra = l.intra[yb4 as usize] as libc::c_int;
        if a_intra != 0 && l_intra != 0 {
            return 2 as libc::c_int;
        }
        if a_intra != 0 || l_intra != 0 {
            let edge = if a_intra != 0 { l } else { a };
            let off = if a_intra != 0 { yb4 } else { xb4 };
            if edge.comp_type[off as usize] as libc::c_int == COMP_INTER_NONE as libc::c_int {
                return 2 as libc::c_int;
            }
            return 1 as libc::c_int
                + 2 * (((edge.r#ref[0][off as usize] as libc::c_int) < 4) as libc::c_int
                    == ((edge.r#ref[1][off as usize] as libc::c_int) < 4) as libc::c_int)
                    as libc::c_int;
        }
        let a_comp = (a.comp_type[xb4 as usize] as libc::c_int != COMP_INTER_NONE as libc::c_int)
            as libc::c_int;
        let l_comp = (l.comp_type[yb4 as usize] as libc::c_int != COMP_INTER_NONE as libc::c_int)
            as libc::c_int;
        let a_ref0 = a.r#ref[0][xb4 as usize] as libc::c_int;
        let l_ref0 = l.r#ref[0][yb4 as usize] as libc::c_int;
        if a_comp == 0 && l_comp == 0 {
            return 1 as libc::c_int
                + 2 * ((a_ref0 >= 4) as libc::c_int == (l_ref0 >= 4) as libc::c_int)
                    as libc::c_int;
        } else if a_comp == 0 || l_comp == 0 {
            let edge_0 = if a_comp != 0 { a } else { l };
            let off_0 = if a_comp != 0 { xb4 } else { yb4 };
            if !(((edge_0.r#ref[0][off_0 as usize] as libc::c_int) < 4) as libc::c_int
                == ((edge_0.r#ref[1][off_0 as usize] as libc::c_int) < 4) as libc::c_int)
            {
                return 1 as libc::c_int;
            }
            return 3 as libc::c_int
                + ((a_ref0 >= 4) as libc::c_int == (l_ref0 >= 4) as libc::c_int) as libc::c_int;
        } else {
            let a_uni = (((a.r#ref[0][xb4 as usize] as libc::c_int) < 4) as libc::c_int
                == ((a.r#ref[1][xb4 as usize] as libc::c_int) < 4) as libc::c_int)
                as libc::c_int;
            let l_uni = (((l.r#ref[0][yb4 as usize] as libc::c_int) < 4) as libc::c_int
                == ((l.r#ref[1][yb4 as usize] as libc::c_int) < 4) as libc::c_int)
                as libc::c_int;
            if a_uni == 0 && l_uni == 0 {
                return 0 as libc::c_int;
            }
            if a_uni == 0 || l_uni == 0 {
                return 2 as libc::c_int;
            }
            return 3 as libc::c_int
                + ((a_ref0 == 4) as libc::c_int == (l_ref0 == 4) as libc::c_int) as libc::c_int;
        }
    } else if have_top || have_left {
        let edge_1 = if have_left { l } else { a };
        let off_1 = if have_left { yb4 } else { xb4 };
        if edge_1.intra[off_1 as usize] != 0 {
            return 2 as libc::c_int;
        }
        if edge_1.comp_type[off_1 as usize] as libc::c_int == COMP_INTER_NONE as libc::c_int {
            return 2 as libc::c_int;
        }
        return 4 as libc::c_int
            * (((edge_1.r#ref[0][off_1 as usize] as libc::c_int) < 4) as libc::c_int
                == ((edge_1.r#ref[1][off_1 as usize] as libc::c_int) < 4) as libc::c_int)
                as libc::c_int;
    } else {
        return 2 as libc::c_int;
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
) -> libc::c_int {
    let d0: libc::c_uint = get_poc_diff(
        order_hint_n_bits,
        ref0poc as libc::c_int,
        poc as libc::c_int,
    )
    .abs() as libc::c_uint;
    let d1: libc::c_uint = get_poc_diff(
        order_hint_n_bits,
        poc as libc::c_int,
        ref1poc as libc::c_int,
    )
    .abs() as libc::c_uint;
    let offset = (d0 == d1) as libc::c_int;
    let a_ctx = (a.comp_type[xb4 as usize] as libc::c_int >= COMP_INTER_AVG as libc::c_int
        || a.r#ref[0][xb4 as usize] as libc::c_int == 6) as libc::c_int;
    let l_ctx = (l.comp_type[yb4 as usize] as libc::c_int >= COMP_INTER_AVG as libc::c_int
        || l.r#ref[0][yb4 as usize] as libc::c_int == 6) as libc::c_int;
    return 3 * offset + a_ctx + l_ctx;
}

#[inline]
pub fn get_mask_comp_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: libc::c_int,
    xb4: libc::c_int,
) -> libc::c_int {
    let a_ctx = if a.comp_type[xb4 as usize] as libc::c_int >= COMP_INTER_SEG as libc::c_int {
        1 as libc::c_int
    } else if a.r#ref[0][xb4 as usize] as libc::c_int == 6 {
        3 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let l_ctx = if l.comp_type[yb4 as usize] as libc::c_int >= COMP_INTER_SEG as libc::c_int {
        1 as libc::c_int
    } else if l.r#ref[0][yb4 as usize] as libc::c_int == 6 {
        3 as libc::c_int
    } else {
        0 as libc::c_int
    };
    return imin(a_ctx + l_ctx, 5 as libc::c_int);
}

#[inline]
pub fn av1_get_ref_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: libc::c_int,
    xb4: libc::c_int,
    mut have_top: bool,
    mut have_left: bool,
) -> libc::c_int {
    let mut cnt: [libc::c_int; 2] = [0 as libc::c_int, 0];
    if have_top && a.intra[xb4 as usize] == 0 {
        cnt[(a.r#ref[0][xb4 as usize] as libc::c_int >= 4) as libc::c_int as usize] += 1;
        if a.comp_type[xb4 as usize] != 0 {
            cnt[(a.r#ref[1][xb4 as usize] as libc::c_int >= 4) as libc::c_int as usize] += 1;
        }
    }
    if have_left && l.intra[yb4 as usize] == 0 {
        cnt[(l.r#ref[0][yb4 as usize] as libc::c_int >= 4) as libc::c_int as usize] += 1;
        if l.comp_type[yb4 as usize] != 0 {
            cnt[(l.r#ref[1][yb4 as usize] as libc::c_int >= 4) as libc::c_int as usize] += 1;
        }
    }
    return if cnt[0] == cnt[1] {
        1 as libc::c_int
    } else if cnt[0] < cnt[1] {
        0 as libc::c_int
    } else {
        2 as libc::c_int
    };
}

#[inline]
pub fn av1_get_fwd_ref_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: libc::c_int,
    xb4: libc::c_int,
    have_top: bool,
    have_left: bool,
) -> libc::c_int {
    let mut cnt: [libc::c_int; 4] = [0 as libc::c_int, 0, 0, 0];
    if have_top && a.intra[xb4 as usize] == 0 {
        if (a.r#ref[0][xb4 as usize] as libc::c_int) < 4 {
            cnt[a.r#ref[0][xb4 as usize] as usize] += 1;
        }
        if a.comp_type[xb4 as usize] as libc::c_int != 0
            && (a.r#ref[1][xb4 as usize] as libc::c_int) < 4
        {
            cnt[a.r#ref[1][xb4 as usize] as usize] += 1;
        }
    }
    if have_left && l.intra[yb4 as usize] == 0 {
        if (l.r#ref[0][yb4 as usize] as libc::c_int) < 4 {
            cnt[l.r#ref[0][yb4 as usize] as usize] += 1;
        }
        if l.comp_type[yb4 as usize] as libc::c_int != 0
            && (l.r#ref[1][yb4 as usize] as libc::c_int) < 4
        {
            cnt[l.r#ref[1][yb4 as usize] as usize] += 1;
        }
    }
    cnt[0] += cnt[1];
    cnt[2] += cnt[3];
    return if cnt[0] == cnt[2] {
        1 as libc::c_int
    } else if cnt[0] < cnt[2] {
        0 as libc::c_int
    } else {
        2 as libc::c_int
    };
}

#[inline]
pub fn av1_get_fwd_ref_1_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: libc::c_int,
    xb4: libc::c_int,
    have_top: bool,
    have_left: bool,
) -> libc::c_int {
    let mut cnt: [libc::c_int; 2] = [0 as libc::c_int, 0];
    if have_top && a.intra[xb4 as usize] == 0 {
        if (a.r#ref[0][xb4 as usize] as libc::c_int) < 2 {
            cnt[a.r#ref[0][xb4 as usize] as usize] += 1;
        }
        if a.comp_type[xb4 as usize] as libc::c_int != 0
            && (a.r#ref[1][xb4 as usize] as libc::c_int) < 2
        {
            cnt[a.r#ref[1][xb4 as usize] as usize] += 1;
        }
    }
    if have_left && l.intra[yb4 as usize] == 0 {
        if (l.r#ref[0][yb4 as usize] as libc::c_int) < 2 {
            cnt[l.r#ref[0][yb4 as usize] as usize] += 1;
        }
        if l.comp_type[yb4 as usize] as libc::c_int != 0
            && (l.r#ref[1][yb4 as usize] as libc::c_int) < 2
        {
            cnt[l.r#ref[1][yb4 as usize] as usize] += 1;
        }
    }
    return if cnt[0] == cnt[1] {
        1 as libc::c_int
    } else if cnt[0] < cnt[1] {
        0 as libc::c_int
    } else {
        2 as libc::c_int
    };
}

#[inline]
pub fn av1_get_fwd_ref_2_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: libc::c_int,
    xb4: libc::c_int,
    have_top: bool,
    have_left: bool,
) -> libc::c_int {
    let mut cnt: [libc::c_int; 2] = [0 as libc::c_int, 0];
    if have_top && a.intra[xb4 as usize] == 0 {
        if (a.r#ref[0][xb4 as usize] as libc::c_uint ^ 2 as libc::c_uint) < 2 as libc::c_uint {
            cnt[(a.r#ref[0][xb4 as usize] as libc::c_int - 2) as usize] += 1;
        }
        if a.comp_type[xb4 as usize] as libc::c_int != 0
            && (a.r#ref[1][xb4 as usize] as libc::c_uint ^ 2 as libc::c_uint) < 2 as libc::c_uint
        {
            cnt[(a.r#ref[1][xb4 as usize] as libc::c_int - 2) as usize] += 1;
        }
    }
    if have_left && l.intra[yb4 as usize] == 0 {
        if (l.r#ref[0][yb4 as usize] as libc::c_uint ^ 2 as libc::c_uint) < 2 as libc::c_uint {
            cnt[(l.r#ref[0][yb4 as usize] as libc::c_int - 2) as usize] += 1;
        }
        if l.comp_type[yb4 as usize] as libc::c_int != 0
            && (l.r#ref[1][yb4 as usize] as libc::c_uint ^ 2 as libc::c_uint) < 2 as libc::c_uint
        {
            cnt[(l.r#ref[1][yb4 as usize] as libc::c_int - 2) as usize] += 1;
        }
    }
    return if cnt[0] == cnt[1] {
        1 as libc::c_int
    } else if cnt[0] < cnt[1] {
        0 as libc::c_int
    } else {
        2 as libc::c_int
    };
}

#[inline]
pub fn av1_get_bwd_ref_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: libc::c_int,
    xb4: libc::c_int,
    have_top: bool,
    have_left: bool,
) -> libc::c_int {
    let mut cnt: [libc::c_int; 3] = [0 as libc::c_int, 0, 0];
    if have_top && a.intra[xb4 as usize] == 0 {
        if a.r#ref[0][xb4 as usize] as libc::c_int >= 4 {
            cnt[(a.r#ref[0][xb4 as usize] as libc::c_int - 4) as usize] += 1;
        }
        if a.comp_type[xb4 as usize] as libc::c_int != 0
            && a.r#ref[1][xb4 as usize] as libc::c_int >= 4
        {
            cnt[(a.r#ref[1][xb4 as usize] as libc::c_int - 4) as usize] += 1;
        }
    }
    if have_left && l.intra[yb4 as usize] == 0 {
        if l.r#ref[0][yb4 as usize] as libc::c_int >= 4 {
            cnt[(l.r#ref[0][yb4 as usize] as libc::c_int - 4) as usize] += 1;
        }
        if l.comp_type[yb4 as usize] as libc::c_int != 0
            && l.r#ref[1][yb4 as usize] as libc::c_int >= 4
        {
            cnt[(l.r#ref[1][yb4 as usize] as libc::c_int - 4) as usize] += 1;
        }
    }
    cnt[1] += cnt[0];
    return if cnt[2] == cnt[1] {
        1 as libc::c_int
    } else if cnt[1] < cnt[2] {
        0 as libc::c_int
    } else {
        2 as libc::c_int
    };
}

#[inline]
pub fn av1_get_bwd_ref_1_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: libc::c_int,
    xb4: libc::c_int,
    have_top: bool,
    have_left: bool,
) -> libc::c_int {
    let mut cnt: [libc::c_int; 3] = [0 as libc::c_int, 0, 0];
    if have_top && a.intra[xb4 as usize] == 0 {
        if a.r#ref[0][xb4 as usize] as libc::c_int >= 4 {
            cnt[(a.r#ref[0][xb4 as usize] as libc::c_int - 4) as usize] += 1;
        }
        if a.comp_type[xb4 as usize] as libc::c_int != 0
            && a.r#ref[1][xb4 as usize] as libc::c_int >= 4
        {
            cnt[(a.r#ref[1][xb4 as usize] as libc::c_int - 4) as usize] += 1;
        }
    }
    if have_left && l.intra[yb4 as usize] == 0 {
        if l.r#ref[0][yb4 as usize] as libc::c_int >= 4 {
            cnt[(l.r#ref[0][yb4 as usize] as libc::c_int - 4) as usize] += 1;
        }
        if l.comp_type[yb4 as usize] as libc::c_int != 0
            && l.r#ref[1][yb4 as usize] as libc::c_int >= 4
        {
            cnt[(l.r#ref[1][yb4 as usize] as libc::c_int - 4) as usize] += 1;
        }
    }
    return if cnt[0] == cnt[1] {
        1 as libc::c_int
    } else if cnt[0] < cnt[1] {
        0 as libc::c_int
    } else {
        2 as libc::c_int
    };
}

#[inline]
pub fn av1_get_uni_p1_ctx(
    a: &BlockContext,
    l: &BlockContext,
    yb4: libc::c_int,
    xb4: libc::c_int,
    have_top: bool,
    have_left: bool,
) -> libc::c_int {
    let mut cnt: [libc::c_int; 3] = [0 as libc::c_int, 0, 0];
    if have_top && a.intra[xb4 as usize] == 0 {
        if (a.r#ref[0][xb4 as usize] as libc::c_uint).wrapping_sub(1 as libc::c_uint)
            < 3 as libc::c_uint
        {
            cnt[(a.r#ref[0][xb4 as usize] as libc::c_int - 1) as usize] += 1;
        }
        if a.comp_type[xb4 as usize] as libc::c_int != 0
            && (a.r#ref[1][xb4 as usize] as libc::c_uint).wrapping_sub(1 as libc::c_uint)
                < 3 as libc::c_uint
        {
            cnt[(a.r#ref[1][xb4 as usize] as libc::c_int - 1) as usize] += 1;
        }
    }
    if have_left && l.intra[yb4 as usize] == 0 {
        if (l.r#ref[0][yb4 as usize] as libc::c_uint).wrapping_sub(1 as libc::c_uint)
            < 3 as libc::c_uint
        {
            cnt[(l.r#ref[0][yb4 as usize] as libc::c_int - 1) as usize] += 1;
        }
        if l.comp_type[yb4 as usize] as libc::c_int != 0
            && (l.r#ref[1][yb4 as usize] as libc::c_uint).wrapping_sub(1 as libc::c_uint)
                < 3 as libc::c_uint
        {
            cnt[(l.r#ref[1][yb4 as usize] as libc::c_int - 1) as usize] += 1;
        }
    }
    cnt[1] += cnt[2];
    return if cnt[0] == cnt[1] {
        1 as libc::c_int
    } else if cnt[0] < cnt[1] {
        0 as libc::c_int
    } else {
        2 as libc::c_int
    };
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
    seg_ctx: *mut libc::c_int,
    mut cur_seg_map: *const uint8_t,
    stride: ptrdiff_t,
) -> libc::c_uint {
    cur_seg_map = cur_seg_map.offset(bx as isize + by as isize * stride);
    if have_left && have_top {
        let l = *cur_seg_map.offset(-(1 as libc::c_int) as isize) as libc::c_int;
        let a = *cur_seg_map.offset(-stride as isize) as libc::c_int;
        let al = *cur_seg_map.offset(-(stride + 1) as isize) as libc::c_int;
        if l == a && al == l {
            *seg_ctx = 2 as libc::c_int;
        } else if l == a || al == l || a == al {
            *seg_ctx = 1 as libc::c_int;
        } else {
            *seg_ctx = 0 as libc::c_int;
        }
        return (if a == al { a } else { l }) as libc::c_uint;
    } else {
        *seg_ctx = 0 as libc::c_int;
        return (if have_left {
            *cur_seg_map.offset(-(1 as libc::c_int) as isize) as libc::c_int
        } else if have_top {
            *cur_seg_map.offset(-stride as isize) as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uint;
    };
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
