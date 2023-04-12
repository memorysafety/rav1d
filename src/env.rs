use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::stdint::int16_t;
use crate::include::stdint::int8_t;
use crate::include::stdint::uint8_t;
use crate::src::levels::mv;
use crate::src::levels::TxfmType;
use crate::src::levels::DCT_DCT;
use crate::src::levels::H_ADST;
use crate::src::levels::H_FLIPADST;
use crate::src::levels::IDTX;
use crate::src::levels::TX_16X16;
use crate::src::levels::TX_32X32;
use crate::src::levels::V_ADST;
use crate::src::levels::V_FLIPADST;
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
    pub ref_0: [[int8_t; 32]; 2],
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
pub unsafe extern "C" fn get_uv_inter_txtp(
    uvt_dim: *const TxfmInfo,
    ytxtp: TxfmType,
) -> TxfmType {
    if (*uvt_dim).max as libc::c_int == TX_32X32 as libc::c_int {
        return (if ytxtp as libc::c_uint == IDTX as libc::c_int as libc::c_uint {
            IDTX as libc::c_int
        } else {
            DCT_DCT as libc::c_int
        }) as TxfmType;
    }
    if (*uvt_dim).min as libc::c_int == TX_16X16 as libc::c_int
        && (1 as libc::c_int) << ytxtp as libc::c_uint
            & ((1 as libc::c_int) << H_FLIPADST as libc::c_int
                | (1 as libc::c_int) << V_FLIPADST as libc::c_int
                | (1 as libc::c_int) << H_ADST as libc::c_int
                | (1 as libc::c_int) << V_ADST as libc::c_int) != 0
    {
        return DCT_DCT;
    }
    return ytxtp;
}

#[inline]
pub unsafe extern "C" fn get_poc_diff(
    order_hint_n_bits: libc::c_int,
    poc0: libc::c_int,
    poc1: libc::c_int,
) -> libc::c_int {
    if order_hint_n_bits == 0 {
        return 0 as libc::c_int;
    }
    let mask: libc::c_int = (1 as libc::c_int) << order_hint_n_bits - 1 as libc::c_int;
    let diff: libc::c_int = poc0 - poc1;
    return (diff & mask - 1 as libc::c_int) - (diff & mask);
}

#[inline]
pub unsafe extern "C" fn fix_int_mv_precision(mv: *mut mv) {
    (*mv)
        .c2rust_unnamed
        .x = (((*mv).c2rust_unnamed.x as libc::c_int
        - ((*mv).c2rust_unnamed.x as libc::c_int >> 15 as libc::c_int)
        + 3 as libc::c_int) as libc::c_uint & !(7 as libc::c_uint)) as int16_t;
    (*mv)
        .c2rust_unnamed
        .y = (((*mv).c2rust_unnamed.y as libc::c_int
        - ((*mv).c2rust_unnamed.y as libc::c_int >> 15 as libc::c_int)
        + 3 as libc::c_int) as libc::c_uint & !(7 as libc::c_uint)) as int16_t;
}

#[inline]
pub unsafe extern "C" fn fix_mv_precision(hdr: *const Dav1dFrameHeader, mv: *mut mv) {
    if (*hdr).force_integer_mv != 0 {
        fix_int_mv_precision(mv);
    } else if (*hdr).hp == 0 {
        (*mv)
            .c2rust_unnamed
            .x = (((*mv).c2rust_unnamed.x as libc::c_int
            - ((*mv).c2rust_unnamed.x as libc::c_int >> 15 as libc::c_int))
            as libc::c_uint & !(1 as libc::c_uint)) as int16_t;
        (*mv)
            .c2rust_unnamed
            .y = (((*mv).c2rust_unnamed.y as libc::c_int
            - ((*mv).c2rust_unnamed.y as libc::c_int >> 15 as libc::c_int))
            as libc::c_uint & !(1 as libc::c_uint)) as int16_t;
    }
}
