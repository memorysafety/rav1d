use crate::include::common::intops::umin;
use crate::include::dav1d::headers::Dav1dPixelLayout;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;
use crate::include::stdint::uint16_t;
use crate::include::stdint::uint32_t;
use crate::include::stdint::uint64_t;
use crate::include::stdint::uint8_t;
use crate::src::levels::BlockSize;
use crate::src::levels::TxClass;
use crate::src::levels::TX_16X16;
use crate::src::levels::TX_32X32;
use crate::src::levels::TX_4X4;
use crate::src::levels::TX_64X64;
use crate::src::levels::TX_8X8;
use crate::src::levels::TX_CLASS_2D;
use crate::src::msac::dav1d_msac_decode_bool_equi;
use crate::src::msac::MsacContext;
use crate::src::tables::dav1d_block_dimensions;
use crate::src::tables::dav1d_skip_ctx;
use crate::src::tables::TxfmInfo;

/// This is a macro that defines a function
/// because it takes `Dav1dFrameContext` and `Dav1dTaskContext` as arguments,
/// which have not yet been deduplicated/genericized over bitdepth.
///
/// TODO: This should not remain a macro.
/// It should either be a `fn` generic over bitdepth
/// or take `struct` arguments that are the subset of fields that are actually used in this `fn`,
/// as this would also solve some borrowck errors that had to be worked around.
macro_rules! define_DEBUG_BLOCK_INFO {
    () => {
        /// TODO: add feature and compile-time guard around this code
        unsafe fn DEBUG_BLOCK_INFO(f: &Dav1dFrameContext, t: &Dav1dTaskContext) -> bool {
            false
                && (*f.frame_hdr).frame_offset == 2
                && t.by >= 0
                && t.by < 4
                && t.bx >= 8
                && t.bx < 12
        }
    };
}

pub(crate) use define_DEBUG_BLOCK_INFO;

#[inline]
pub fn read_golomb(msac: &mut MsacContext) -> libc::c_uint {
    let mut len = 0;
    let mut val = 1;

    while !dav1d_msac_decode_bool_equi(msac) && len < 32 {
        len += 1;
    }
    for _ in 0..len {
        val = (val << 1) + dav1d_msac_decode_bool_equi(msac) as libc::c_uint;
    }

    val - 1
}

#[inline]
pub unsafe fn get_skip_ctx(
    t_dim: &TxfmInfo,
    bs: BlockSize,
    a: &[u8],
    l: &[u8],
    chroma: libc::c_int,
    layout: Dav1dPixelLayout,
) -> libc::c_uint {
    let a = a.as_ptr();
    let l = l.as_ptr();

    let b_dim = &dav1d_block_dimensions[bs as usize];
    if chroma != 0 {
        let ss_ver = (layout as libc::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint)
            as libc::c_int;
        let ss_hor = (layout as libc::c_uint
            != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint)
            as libc::c_int;
        let not_one_blk = (b_dim[2] as libc::c_int - (b_dim[2] != 0 && ss_hor != 0) as libc::c_int
            > t_dim.lw as libc::c_int
            || b_dim[3] as libc::c_int - (b_dim[3] != 0 && ss_ver != 0) as libc::c_int
                > t_dim.lh as libc::c_int) as libc::c_int;
        let mut ca: libc::c_uint = 0;
        let mut cl: libc::c_uint = 0;
        let mut current_block_7: u64;
        match t_dim.lw as libc::c_int {
            0 => {
                current_block_7 = 11396040223254765297;
            }
            1 => {
                ca = (*(a as *const uint16_t) as libc::c_int != 0x4040 as libc::c_int)
                    as libc::c_int as libc::c_uint;
                current_block_7 = 2979737022853876585;
            }
            2 => {
                ca = (*(a as *const uint32_t) != 0x40404040 as libc::c_uint) as libc::c_int
                    as libc::c_uint;
                current_block_7 = 2979737022853876585;
            }
            3 => {
                ca = (*(a as *const uint64_t) as libc::c_ulonglong
                    != 0x4040404040404040 as libc::c_ulonglong) as libc::c_int
                    as libc::c_uint;
                current_block_7 = 2979737022853876585;
            }
            _ => {
                if 0 == 0 {
                    unreachable!();
                }
                current_block_7 = 11396040223254765297;
            }
        }
        match current_block_7 {
            11396040223254765297 => {
                ca = (*a as libc::c_int != 0x40 as libc::c_int) as libc::c_int as libc::c_uint;
            }
            _ => {}
        }
        let mut current_block_16: u64;
        match t_dim.lh as libc::c_int {
            0 => {
                current_block_16 = 15770135957368472560;
            }
            1 => {
                cl = (*(l as *const uint16_t) as libc::c_int != 0x4040 as libc::c_int)
                    as libc::c_int as libc::c_uint;
                current_block_16 = 11307063007268554308;
            }
            2 => {
                cl = (*(l as *const uint32_t) != 0x40404040 as libc::c_uint) as libc::c_int
                    as libc::c_uint;
                current_block_16 = 11307063007268554308;
            }
            3 => {
                cl = (*(l as *const uint64_t) as libc::c_ulonglong
                    != 0x4040404040404040 as libc::c_ulonglong) as libc::c_int
                    as libc::c_uint;
                current_block_16 = 11307063007268554308;
            }
            _ => {
                if 0 == 0 {
                    unreachable!();
                }
                current_block_16 = 15770135957368472560;
            }
        }
        match current_block_16 {
            15770135957368472560 => {
                cl = (*l as libc::c_int != 0x40 as libc::c_int) as libc::c_int as libc::c_uint;
            }
            _ => {}
        }
        return ((7 + not_one_blk * 3) as libc::c_uint)
            .wrapping_add(ca)
            .wrapping_add(cl);
    } else if b_dim[2] as libc::c_int == t_dim.lw as libc::c_int
        && b_dim[3] as libc::c_int == t_dim.lh as libc::c_int
    {
        return 0 as libc::c_int as libc::c_uint;
    } else {
        let mut la: libc::c_uint = 0;
        let mut ll: libc::c_uint = 0;
        let mut current_block_80: u64;
        match t_dim.lw as libc::c_int {
            0 => {
                current_block_80 = 15794479632267580089;
            }
            1 => {
                if TX_8X8 as libc::c_int == TX_64X64 as libc::c_int {
                    let mut tmp_0: uint64_t = *(a as *const uint64_t);
                    tmp_0 |= *(&*a.offset(8) as *const uint8_t as *const uint64_t);
                    la = (tmp_0 >> 32) as libc::c_uint | tmp_0 as libc::c_uint;
                } else {
                    la = *(a as *const uint16_t) as libc::c_uint;
                }
                if TX_8X8 as libc::c_int == TX_32X32 as libc::c_int {
                    la |= *(&*a.offset(::core::mem::size_of::<uint16_t>() as libc::c_ulong as isize)
                        as *const uint8_t as *const uint16_t)
                        as libc::c_uint;
                }
                if TX_8X8 as libc::c_int >= TX_16X16 as libc::c_int {
                    la |= la >> 16;
                }
                if TX_8X8 as libc::c_int >= TX_8X8 as libc::c_int {
                    la |= la >> 8;
                }
                current_block_80 = 17787701279558130514;
            }
            2 => {
                if TX_16X16 as libc::c_int == TX_64X64 as libc::c_int {
                    let mut tmp_1: uint64_t = *(a as *const uint64_t);
                    tmp_1 |= *(&*a.offset(8) as *const uint8_t as *const uint64_t);
                    la = (tmp_1 >> 32) as libc::c_uint | tmp_1 as libc::c_uint;
                } else {
                    la = *(a as *const uint32_t);
                }
                if TX_16X16 as libc::c_int == TX_32X32 as libc::c_int {
                    la |= *(&*a.offset(::core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
                        as *const uint8_t as *const uint32_t);
                }
                if TX_16X16 as libc::c_int >= TX_16X16 as libc::c_int {
                    la |= la >> 16;
                }
                if TX_16X16 as libc::c_int >= TX_8X8 as libc::c_int {
                    la |= la >> 8;
                }
                current_block_80 = 17787701279558130514;
            }
            3 => {
                if TX_32X32 as libc::c_int == TX_64X64 as libc::c_int {
                    let mut tmp_2: uint64_t = *(a as *const uint64_t);
                    tmp_2 |= *(&*a.offset(8) as *const uint8_t as *const uint64_t);
                    la = (tmp_2 >> 32) as libc::c_uint | tmp_2 as libc::c_uint;
                } else {
                    la = *(a as *const uint32_t);
                }
                if TX_32X32 as libc::c_int == TX_32X32 as libc::c_int {
                    la |= *(&*a.offset(::core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
                        as *const uint8_t as *const uint32_t);
                }
                if TX_32X32 as libc::c_int >= TX_16X16 as libc::c_int {
                    la |= la >> 16;
                }
                if TX_32X32 as libc::c_int >= TX_8X8 as libc::c_int {
                    la |= la >> 8;
                }
                current_block_80 = 17787701279558130514;
            }
            4 => {
                if TX_64X64 as libc::c_int == TX_64X64 as libc::c_int {
                    let mut tmp_3: uint64_t = *(a as *const uint64_t);
                    tmp_3 |= *(&*a.offset(8) as *const uint8_t as *const uint64_t);
                    la = (tmp_3 >> 32) as libc::c_uint | tmp_3 as libc::c_uint;
                } else {
                    la = *(a as *const uint32_t);
                }
                if TX_64X64 as libc::c_int == TX_32X32 as libc::c_int {
                    la |= *(&*a.offset(::core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
                        as *const uint8_t as *const uint32_t);
                }
                if TX_64X64 as libc::c_int >= TX_16X16 as libc::c_int {
                    la |= la >> 16;
                }
                if TX_64X64 as libc::c_int >= TX_8X8 as libc::c_int {
                    la |= la >> 8;
                }
                current_block_80 = 17787701279558130514;
            }
            _ => {
                if 0 == 0 {
                    unreachable!();
                }
                current_block_80 = 15794479632267580089;
            }
        }
        match current_block_80 {
            15794479632267580089 => {
                if TX_4X4 as libc::c_int == TX_64X64 as libc::c_int {
                    let mut tmp: uint64_t = *(a as *const uint64_t);
                    tmp |= *(&*a.offset(8) as *const uint8_t as *const uint64_t);
                    la = (tmp >> 32) as libc::c_uint | tmp as libc::c_uint;
                } else {
                    la = *a as libc::c_uint;
                }
                if TX_4X4 as libc::c_int == TX_32X32 as libc::c_int {
                    la |= *(&*a.offset(::core::mem::size_of::<uint8_t>() as libc::c_ulong as isize)
                        as *const uint8_t) as libc::c_uint;
                }
                if TX_4X4 as libc::c_int >= TX_16X16 as libc::c_int {
                    la |= la >> 16;
                }
                if TX_4X4 as libc::c_int >= TX_8X8 as libc::c_int {
                    la |= la >> 8;
                }
            }
            _ => {}
        }
        let mut current_block_140: u64;
        match t_dim.lh as libc::c_int {
            0 => {
                current_block_140 = 5167972421258071942;
            }
            1 => {
                if TX_8X8 as libc::c_int == TX_64X64 as libc::c_int {
                    let mut tmp_5: uint64_t = *(l as *const uint64_t);
                    tmp_5 |= *(&*l.offset(8) as *const uint8_t as *const uint64_t);
                    ll = (tmp_5 >> 32) as libc::c_uint | tmp_5 as libc::c_uint;
                } else {
                    ll = *(l as *const uint16_t) as libc::c_uint;
                }
                if TX_8X8 as libc::c_int == TX_32X32 as libc::c_int {
                    ll |= *(&*l.offset(::core::mem::size_of::<uint16_t>() as libc::c_ulong as isize)
                        as *const uint8_t as *const uint16_t)
                        as libc::c_uint;
                }
                if TX_8X8 as libc::c_int >= TX_16X16 as libc::c_int {
                    ll |= ll >> 16;
                }
                if TX_8X8 as libc::c_int >= TX_8X8 as libc::c_int {
                    ll |= ll >> 8;
                }
                current_block_140 = 7370318721998929769;
            }
            2 => {
                if TX_16X16 as libc::c_int == TX_64X64 as libc::c_int {
                    let mut tmp_6: uint64_t = *(l as *const uint64_t);
                    tmp_6 |= *(&*l.offset(8) as *const uint8_t as *const uint64_t);
                    ll = (tmp_6 >> 32) as libc::c_uint | tmp_6 as libc::c_uint;
                } else {
                    ll = *(l as *const uint32_t);
                }
                if TX_16X16 as libc::c_int == TX_32X32 as libc::c_int {
                    ll |= *(&*l.offset(::core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
                        as *const uint8_t as *const uint32_t);
                }
                if TX_16X16 as libc::c_int >= TX_16X16 as libc::c_int {
                    ll |= ll >> 16;
                }
                if TX_16X16 as libc::c_int >= TX_8X8 as libc::c_int {
                    ll |= ll >> 8;
                }
                current_block_140 = 7370318721998929769;
            }
            3 => {
                if TX_32X32 as libc::c_int == TX_64X64 as libc::c_int {
                    let mut tmp_7: uint64_t = *(l as *const uint64_t);
                    tmp_7 |= *(&*l.offset(8) as *const uint8_t as *const uint64_t);
                    ll = (tmp_7 >> 32) as libc::c_uint | tmp_7 as libc::c_uint;
                } else {
                    ll = *(l as *const uint32_t);
                }
                if TX_32X32 as libc::c_int == TX_32X32 as libc::c_int {
                    ll |= *(&*l.offset(::core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
                        as *const uint8_t as *const uint32_t);
                }
                if TX_32X32 as libc::c_int >= TX_16X16 as libc::c_int {
                    ll |= ll >> 16;
                }
                if TX_32X32 as libc::c_int >= TX_8X8 as libc::c_int {
                    ll |= ll >> 8;
                }
                current_block_140 = 7370318721998929769;
            }
            4 => {
                if TX_64X64 as libc::c_int == TX_64X64 as libc::c_int {
                    let mut tmp_8: uint64_t = *(l as *const uint64_t);
                    tmp_8 |= *(&*l.offset(8) as *const uint8_t as *const uint64_t);
                    ll = (tmp_8 >> 32) as libc::c_uint | tmp_8 as libc::c_uint;
                } else {
                    ll = *(l as *const uint32_t);
                }
                if TX_64X64 as libc::c_int == TX_32X32 as libc::c_int {
                    ll |= *(&*l.offset(::core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
                        as *const uint8_t as *const uint32_t);
                }
                if TX_64X64 as libc::c_int >= TX_16X16 as libc::c_int {
                    ll |= ll >> 16;
                }
                if TX_64X64 as libc::c_int >= TX_8X8 as libc::c_int {
                    ll |= ll >> 8;
                }
                current_block_140 = 7370318721998929769;
            }
            _ => {
                if 0 == 0 {
                    unreachable!();
                }
                current_block_140 = 5167972421258071942;
            }
        }
        match current_block_140 {
            5167972421258071942 => {
                if TX_4X4 as libc::c_int == TX_64X64 as libc::c_int {
                    let mut tmp_4: uint64_t = *(l as *const uint64_t);
                    tmp_4 |= *(&*l.offset(8) as *const uint8_t as *const uint64_t);
                    ll = (tmp_4 >> 32) as libc::c_uint | tmp_4 as libc::c_uint;
                } else {
                    ll = *l as libc::c_uint;
                }
                if TX_4X4 as libc::c_int == TX_32X32 as libc::c_int {
                    ll |= *(&*l.offset(::core::mem::size_of::<uint8_t>() as libc::c_ulong as isize)
                        as *const uint8_t) as libc::c_uint;
                }
                if TX_4X4 as libc::c_int >= TX_16X16 as libc::c_int {
                    ll |= ll >> 16;
                }
                if TX_4X4 as libc::c_int >= TX_8X8 as libc::c_int {
                    ll |= ll >> 8;
                }
            }
            _ => {}
        }
        return dav1d_skip_ctx[umin(
            la & 0x3f as libc::c_int as libc::c_uint,
            4 as libc::c_int as libc::c_uint,
        ) as usize][umin(
            ll & 0x3f as libc::c_int as libc::c_uint,
            4 as libc::c_int as libc::c_uint,
        ) as usize] as libc::c_uint;
    };
}

#[inline]
pub unsafe fn get_dc_sign_ctx(tx: libc::c_int, a: &[u8], l: &[u8]) -> libc::c_uint {
    let a = a.as_ptr();
    let l = l.as_ptr();

    let mut mask = 0xc0c0c0c0c0c0c0c0 as uint64_t;
    let mut mul = 0x101010101010101 as uint64_t;
    let mut s = 0;
    match tx {
        0 => {
            let mut t = *a as libc::c_int >> 6;
            t += *l as libc::c_int >> 6;
            s = t - 1 - 1;
        }
        1 => {
            let mut t = *(a as *const uint16_t) as uint32_t & mask as uint32_t;
            t = t.wrapping_add(*(l as *const uint16_t) as uint32_t & mask as uint32_t);
            t = t.wrapping_mul(0x4040404);
            s = (t >> 24) as libc::c_int - 2 - 2;
        }
        2 => {
            let mut t = (*(a as *const uint32_t) & mask as uint32_t) >> 6;
            t = t.wrapping_add((*(l as *const uint32_t) & mask as uint32_t) >> 6);
            t = t.wrapping_mul(mul as uint32_t);
            s = (t >> 24) as libc::c_int - 4 - 4;
        }
        3 => {
            let mut t = (*(a as *const uint64_t) & mask) >> 6;
            t = t.wrapping_add((*(l as *const uint64_t) & mask) >> 6);
            t = t.wrapping_mul(mul);
            s = (t >> 56) as libc::c_int - 8 - 8;
        }
        4 => {
            let mut t = (*(&*a.offset(0) as *const uint8_t as *const uint64_t) & mask) >> 6;
            t = t.wrapping_add((*(&*a.offset(8) as *const uint8_t as *const uint64_t) & mask) >> 6);
            t = t.wrapping_add((*(&*l.offset(0) as *const uint8_t as *const uint64_t) & mask) >> 6);
            t = t.wrapping_add((*(&*l.offset(8) as *const uint8_t as *const uint64_t) & mask) >> 6);
            t = t.wrapping_mul(mul);
            s = (t >> 56) as libc::c_int - 16 - 16;
        }
        5 => {
            let mut t = *a as uint32_t & mask as uint32_t;
            t = t.wrapping_add(*(l as *const uint16_t) as uint32_t & mask as uint32_t);
            t = t.wrapping_mul(0x4040404);
            s = (t >> 24) as libc::c_int - 1 - 2;
        }
        6 => {
            let mut t = *(a as *const uint16_t) as uint32_t & mask as uint32_t;
            t = t.wrapping_add(*l as uint32_t & mask as uint32_t);
            t = t.wrapping_mul(0x4040404);
            s = (t >> 24) as libc::c_int - 2 - 1;
        }
        7 => {
            let mut t = *(a as *const uint16_t) as uint32_t & mask as uint32_t;
            t = t.wrapping_add(*(l as *const uint32_t) & mask as uint32_t);
            t = (t >> 6).wrapping_mul(mul as uint32_t);
            s = (t >> 24) as libc::c_int - 2 - 4;
        }
        8 => {
            let mut t = *(a as *const uint32_t) & mask as uint32_t;
            t = t.wrapping_add(*(l as *const uint16_t) as libc::c_uint & mask as uint32_t);
            t = (t >> 6).wrapping_mul(mul as uint32_t);
            s = (t >> 24) as libc::c_int - 4 - 2;
        }
        9 => {
            let mut t = (*(a as *const uint32_t) & mask as uint32_t) as uint64_t;
            t = t.wrapping_add(*(l as *const uint64_t) & mask);
            t = (t >> 6).wrapping_mul(mul);
            s = (t >> 56) as libc::c_int - 4 - 8;
        }
        10 => {
            let mut t = *(a as *const uint64_t) & mask;
            t = t + (*(l as *const uint32_t) & mask as uint32_t) as uint64_t;
            t = (t >> 6).wrapping_mul(mul);
            s = (t >> 56) as libc::c_int - 8 - 4;
        }
        11 => {
            let mut t = (*(&*a.offset(0) as *const uint8_t as *const uint64_t) & mask) >> 6;
            t = t.wrapping_add((*(&*l.offset(0) as *const uint8_t as *const uint64_t) & mask) >> 6);
            t = t.wrapping_add((*(&*l.offset(8) as *const uint8_t as *const uint64_t) & mask) >> 6);
            t = t.wrapping_mul(mul);
            s = (t >> 56) as libc::c_int - 8 - 16;
        }
        12 => {
            let mut t = (*(&*a.offset(0) as *const uint8_t as *const uint64_t) & mask) >> 6;
            t = t.wrapping_add((*(&*a.offset(8) as *const uint8_t as *const uint64_t) & mask) >> 6);
            t = t.wrapping_add((*(&*l.offset(0) as *const uint8_t as *const uint64_t) & mask) >> 6);
            t = t.wrapping_mul(mul);
            s = (t >> 56) as libc::c_int - 16 - 8;
        }
        13 => {
            let mut t = *a as uint32_t & mask as uint32_t;
            t = t.wrapping_add(*(l as *const uint32_t) & mask as uint32_t);
            t = (t >> 6).wrapping_mul(mul as uint32_t);
            s = (t >> 24) as libc::c_int - 1 - 4;
        }
        14 => {
            let mut t = *(a as *const uint32_t) & mask as uint32_t;
            t = t.wrapping_add(*l as libc::c_uint & mask as uint32_t);
            t = (t >> 6).wrapping_mul(mul as uint32_t);
            s = (t >> 24) as libc::c_int - 4 - 1;
        }
        15 => {
            let mut t = (*(a as *const uint16_t) as libc::c_uint & mask as uint32_t) as uint64_t;
            t = t.wrapping_add(*(l as *const uint64_t) & mask);
            t = (t >> 6).wrapping_mul(mul);
            s = (t >> 56) as libc::c_int - 2 - 8;
        }
        16 => {
            let mut t = *(a as *const uint64_t) & mask;
            t = t
                .wrapping_add((*(l as *const uint16_t) as uint32_t & mask as uint32_t) as uint64_t);
            t = (t >> 6).wrapping_mul(mul);
            s = (t >> 56) as libc::c_int - 8 - 2;
        }
        17 => {
            let mut t = (*(a as *const uint32_t) & mask as uint32_t) as uint64_t;
            t = t.wrapping_add(*(&*l.offset(0) as *const uint8_t as *const uint64_t) & mask);
            t = (t >> 6)
                .wrapping_add((*(&*l.offset(8) as *const uint8_t as *const uint64_t) & mask) >> 6);
            t = t.wrapping_mul(mul);
            s = (t >> 56) as libc::c_int - 4 - 16;
        }
        18 => {
            let mut t = *(&*a.offset(0) as *const uint8_t as *const uint64_t) & mask;
            t = t + (*(l as *const uint32_t) & mask as uint32_t) as uint64_t;
            t = (t >> 6)
                .wrapping_add((*(&*a.offset(8) as *const uint8_t as *const uint64_t) & mask) >> 6);
            t = t.wrapping_mul(mul);
            s = (t >> 56) as libc::c_int - 16 - 4;
        }
        _ => unreachable!(),
    }
    return (s != 0) as libc::c_uint + (s > 0) as libc::c_uint;
}

#[inline]
pub fn get_lo_ctx(
    levels: &[u8],
    tx_class: TxClass,
    hi_mag: &mut libc::c_uint,
    ctx_offsets: Option<&[[u8; 5]; 5]>,
    x: usize,
    y: usize,
    stride: usize,
) -> usize {
    let level = |y, x| levels[y * stride + x] as usize;

    let mut mag = level(0, 1) + level(1, 0);
    let offset = if tx_class == TX_CLASS_2D {
        mag += level(1, 1);
        *hi_mag = mag as libc::c_uint;
        mag += level(0, 2) + level(2, 0);
        ctx_offsets.unwrap()[std::cmp::min(y, 4)][std::cmp::min(x, 4)] as usize
    } else {
        mag += level(0, 2);
        *hi_mag = mag as libc::c_uint;
        mag += level(0, 3) + level(0, 4);
        26 + if y > 1 { 10 } else { y * 5 }
    };
    offset + if mag > 512 { 4 } else { (mag + 64) >> 7 }
}
