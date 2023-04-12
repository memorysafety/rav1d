use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static dav1d_block_dimensions: [[uint8_t; 4]; 22];
    static dav1d_txfm_dimensions: [TxfmInfo; 19];
}







use crate::src::ctx::alias64;
use crate::src::ctx::alias32;
use crate::src::ctx::alias16;
use crate::src::ctx::alias8;


























use crate::include::dav1d::headers::Dav1dPixelLayout;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;

use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;






use crate::include::dav1d::headers::Dav1dSegmentationData;

use crate::include::dav1d::headers::Dav1dLoopfilterModeRefDeltas;

use crate::include::dav1d::headers::Dav1dFrameHeader;


















use crate::src::levels::TX_4X4;
use crate::src::levels::RectTxfmSize;















use crate::src::levels::BlockSize;























#[derive(Copy, Clone)]
#[repr(C)]
pub struct Av1FilterLUT {
    pub e: [uint8_t; 64],
    pub i: [uint8_t; 64],
    pub sharp: [uint64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Av1RestorationUnit {
    pub type_0: uint8_t,
    pub filter_h: [int8_t; 3],
    pub filter_v: [int8_t; 3],
    pub sgr_idx: uint8_t,
    pub sgr_weights: [int8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Av1Filter {
    pub filter_y: [[[[uint16_t; 2]; 3]; 32]; 2],
    pub filter_uv: [[[[uint16_t; 2]; 2]; 32]; 2],
    pub cdef_idx: [int8_t; 4],
    pub noskip_mask: [[uint16_t; 2]; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Av1Restoration {
    pub lr: [[Av1RestorationUnit; 4]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TxfmInfo {
    pub w: uint8_t,
    pub h: uint8_t,
    pub lw: uint8_t,
    pub lh: uint8_t,
    pub min: uint8_t,
    pub max: uint8_t,
    pub sub: uint8_t,
    pub ctx: uint8_t,
}
#[inline]
unsafe extern "C" fn iclip(
    v: libc::c_int,
    min: libc::c_int,
    max: libc::c_int,
) -> libc::c_int {
    return if v < min { min } else if v > max { max } else { v };
}
#[inline]
unsafe extern "C" fn imax(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn imin(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
unsafe extern "C" fn decomp_tx(
    txa: *mut [[[uint8_t; 32]; 32]; 2],
    from: RectTxfmSize,
    depth: libc::c_int,
    y_off: libc::c_int,
    x_off: libc::c_int,
    tx_masks: *const uint16_t,
) {
    let t_dim: *const TxfmInfo = &*dav1d_txfm_dimensions.as_ptr().offset(from as isize)
        as *const TxfmInfo;
    let is_split: libc::c_int = if from as libc::c_uint
        == TX_4X4 as libc::c_int as libc::c_uint || depth > 1 as libc::c_int
    {
        0 as libc::c_int
    } else {
        *tx_masks.offset(depth as isize) as libc::c_int
            >> y_off * 4 as libc::c_int + x_off & 1 as libc::c_int
    };
    if is_split != 0 {
        let sub: RectTxfmSize = (*t_dim).sub as RectTxfmSize;
        let htw4: libc::c_int = (*t_dim).w as libc::c_int >> 1 as libc::c_int;
        let hth4: libc::c_int = (*t_dim).h as libc::c_int >> 1 as libc::c_int;
        decomp_tx(
            txa,
            sub,
            depth + 1 as libc::c_int,
            y_off * 2 as libc::c_int + 0 as libc::c_int,
            x_off * 2 as libc::c_int + 0 as libc::c_int,
            tx_masks,
        );
        if (*t_dim).w as libc::c_int >= (*t_dim).h as libc::c_int {
            decomp_tx(
                &mut *(*(*(*txa.offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(htw4 as isize) as *mut uint8_t
                    as *mut [[[uint8_t; 32]; 32]; 2],
                sub,
                depth + 1 as libc::c_int,
                y_off * 2 as libc::c_int + 0 as libc::c_int,
                x_off * 2 as libc::c_int + 1 as libc::c_int,
                tx_masks,
            );
        }
        if (*t_dim).h as libc::c_int >= (*t_dim).w as libc::c_int {
            decomp_tx(
                &mut *(*(*(*txa.offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(hth4 as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as *mut uint8_t
                    as *mut [[[uint8_t; 32]; 32]; 2],
                sub,
                depth + 1 as libc::c_int,
                y_off * 2 as libc::c_int + 1 as libc::c_int,
                x_off * 2 as libc::c_int + 0 as libc::c_int,
                tx_masks,
            );
            if (*t_dim).w as libc::c_int >= (*t_dim).h as libc::c_int {
                decomp_tx(
                    &mut *(*(*(*txa.offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(hth4 as isize))
                        .as_mut_ptr()
                        .offset(htw4 as isize) as *mut uint8_t
                        as *mut [[[uint8_t; 32]; 32]; 2],
                    sub,
                    depth + 1 as libc::c_int,
                    y_off * 2 as libc::c_int + 1 as libc::c_int,
                    x_off * 2 as libc::c_int + 1 as libc::c_int,
                    tx_masks,
                );
            }
        }
    } else {
        let lw: libc::c_int = imin(2 as libc::c_int, (*t_dim).lw as libc::c_int);
        let lh: libc::c_int = imin(2 as libc::c_int, (*t_dim).lh as libc::c_int);
        match (*t_dim).w as libc::c_int {
            1 => {
                let mut y: libc::c_int = 0 as libc::c_int;
                while y < (*t_dim).h as libc::c_int {
                    (*(&mut *(*(*(*txa.offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(y as isize))
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut uint8_t
                        as *mut alias8))
                        .u8_0 = (0x1 as libc::c_int * lw) as uint8_t;
                    (*(&mut *(*(*(*txa.offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(y as isize))
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut uint8_t
                        as *mut alias8))
                        .u8_0 = (0x1 as libc::c_int * lh) as uint8_t;
                    (*txa
                        .offset(
                            0 as libc::c_int as isize,
                        ))[1 as libc::c_int
                        as usize][y as usize][0 as libc::c_int as usize] = (*t_dim).w;
                    y += 1;
                }
            }
            2 => {
                let mut y_0: libc::c_int = 0 as libc::c_int;
                while y_0 < (*t_dim).h as libc::c_int {
                    (*(&mut *(*(*(*txa.offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(y_0 as isize))
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut uint8_t
                        as *mut alias16))
                        .u16_0 = (0x101 as libc::c_int * lw) as uint16_t;
                    (*(&mut *(*(*(*txa.offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(y_0 as isize))
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut uint8_t
                        as *mut alias16))
                        .u16_0 = (0x101 as libc::c_int * lh) as uint16_t;
                    (*txa
                        .offset(
                            0 as libc::c_int as isize,
                        ))[1 as libc::c_int
                        as usize][y_0 as usize][0 as libc::c_int as usize] = (*t_dim).w;
                    y_0 += 1;
                }
            }
            4 => {
                let mut y_1: libc::c_int = 0 as libc::c_int;
                while y_1 < (*t_dim).h as libc::c_int {
                    (*(&mut *(*(*(*txa.offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(y_1 as isize))
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut uint8_t
                        as *mut alias32))
                        .u32_0 = (0x1010101 as libc::c_uint)
                        .wrapping_mul(lw as libc::c_uint);
                    (*(&mut *(*(*(*txa.offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(y_1 as isize))
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut uint8_t
                        as *mut alias32))
                        .u32_0 = (0x1010101 as libc::c_uint)
                        .wrapping_mul(lh as libc::c_uint);
                    (*txa
                        .offset(
                            0 as libc::c_int as isize,
                        ))[1 as libc::c_int
                        as usize][y_1 as usize][0 as libc::c_int as usize] = (*t_dim).w;
                    y_1 += 1;
                }
            }
            8 => {
                let mut y_2: libc::c_int = 0 as libc::c_int;
                while y_2 < (*t_dim).h as libc::c_int {
                    (*(&mut *(*(*(*txa.offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(y_2 as isize))
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(lw as libc::c_ulonglong) as uint64_t;
                    (*(&mut *(*(*(*txa.offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(y_2 as isize))
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(lh as libc::c_ulonglong) as uint64_t;
                    (*txa
                        .offset(
                            0 as libc::c_int as isize,
                        ))[1 as libc::c_int
                        as usize][y_2 as usize][0 as libc::c_int as usize] = (*t_dim).w;
                    y_2 += 1;
                }
            }
            16 => {
                let mut y_3: libc::c_int = 0 as libc::c_int;
                while y_3 < (*t_dim).h as libc::c_int {
                    let const_val: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(lw as libc::c_ulonglong) as uint64_t;
                    (*(&mut *(*(*(*txa.offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(y_3 as isize))
                        .as_mut_ptr()
                        .offset((0 as libc::c_int + 0 as libc::c_int) as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = const_val;
                    (*(&mut *(*(*(*txa.offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(y_3 as isize))
                        .as_mut_ptr()
                        .offset((0 as libc::c_int + 8 as libc::c_int) as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = const_val;
                    let const_val_0: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(lh as libc::c_ulonglong) as uint64_t;
                    (*(&mut *(*(*(*txa.offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(y_3 as isize))
                        .as_mut_ptr()
                        .offset((0 as libc::c_int + 0 as libc::c_int) as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = const_val_0;
                    (*(&mut *(*(*(*txa.offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(y_3 as isize))
                        .as_mut_ptr()
                        .offset((0 as libc::c_int + 8 as libc::c_int) as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = const_val_0;
                    (*txa
                        .offset(
                            0 as libc::c_int as isize,
                        ))[1 as libc::c_int
                        as usize][y_3 as usize][0 as libc::c_int as usize] = (*t_dim).w;
                    y_3 += 1;
                }
            }
            _ => {}
        }
        match (*t_dim).w as libc::c_int {
            1 => {
                (*(&mut *(*(*(*txa.offset(1 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as *mut uint8_t as *mut alias8))
                    .u8_0 = (0x1 as libc::c_int * (*t_dim).h as libc::c_int) as uint8_t;
            }
            2 => {
                (*(&mut *(*(*(*txa.offset(1 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as *mut uint8_t as *mut alias16))
                    .u16_0 = (0x101 as libc::c_int * (*t_dim).h as libc::c_int)
                    as uint16_t;
            }
            4 => {
                (*(&mut *(*(*(*txa.offset(1 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as *mut uint8_t as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint)
                    .wrapping_mul((*t_dim).h as libc::c_uint);
            }
            8 => {
                (*(&mut *(*(*(*txa.offset(1 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as *mut uint8_t as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul((*t_dim).h as libc::c_ulonglong) as uint64_t;
            }
            16 => {
                let const_val_1: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul((*t_dim).h as libc::c_ulonglong) as uint64_t;
                (*(&mut *(*(*(*txa.offset(1 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset((0 as libc::c_int + 0 as libc::c_int) as isize)
                    as *mut uint8_t as *mut alias64))
                    .u64_0 = const_val_1;
                (*(&mut *(*(*(*txa.offset(1 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset((0 as libc::c_int + 8 as libc::c_int) as isize)
                    as *mut uint8_t as *mut alias64))
                    .u64_0 = const_val_1;
            }
            _ => {}
        }
    };
}
#[inline]
unsafe extern "C" fn mask_edges_inter(
    masks: *mut [[[uint16_t; 2]; 3]; 32],
    by4: libc::c_int,
    bx4: libc::c_int,
    w4: libc::c_int,
    h4: libc::c_int,
    skip: libc::c_int,
    max_tx: RectTxfmSize,
    tx_masks: *const uint16_t,
    a: *mut uint8_t,
    l: *mut uint8_t,
) {
    let t_dim: *const TxfmInfo = &*dav1d_txfm_dimensions.as_ptr().offset(max_tx as isize)
        as *const TxfmInfo;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut txa: [[[[uint8_t; 32]; 32]; 2]; 2] = [[[[0; 32]; 32]; 2]; 2];
    let mut y_off: libc::c_int = 0 as libc::c_int;
    let mut y_0: libc::c_int = 0 as libc::c_int;
    while y_0 < h4 {
        let mut x_off: libc::c_int = 0 as libc::c_int;
        let mut x_0: libc::c_int = 0 as libc::c_int;
        while x_0 < w4 {
            decomp_tx(
                &mut *(*(*(*txa.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(y_0 as isize))
                    .as_mut_ptr()
                    .offset(x_0 as isize) as *mut uint8_t
                    as *mut [[[uint8_t; 32]; 32]; 2],
                max_tx,
                0 as libc::c_int,
                y_off,
                x_off,
                tx_masks,
            );
            x_0 += (*t_dim).w as libc::c_int;
            x_off += 1;
        }
        y_0 += (*t_dim).h as libc::c_int;
        y_off += 1;
    }
    let mut mask: libc::c_uint = (1 as libc::c_uint) << by4;
    y = 0 as libc::c_int;
    while y < h4 {
        let sidx: libc::c_int = (mask >= 0x10000 as libc::c_int as libc::c_uint)
            as libc::c_int;
        let smask: libc::c_uint = mask >> (sidx << 4 as libc::c_int);
        let ref mut fresh0 = (*masks
            .offset(
                0 as libc::c_int as isize,
            ))[bx4
            as usize][imin(
            txa[0 as libc::c_int
                as usize][0 as libc::c_int
                as usize][y as usize][0 as libc::c_int as usize] as libc::c_int,
            *l.offset(y as isize) as libc::c_int,
        ) as usize][sidx as usize];
        *fresh0 = (*fresh0 as libc::c_uint | smask) as uint16_t;
        y += 1;
        mask <<= 1 as libc::c_int;
    }
    x = 0 as libc::c_int;
    mask = (1 as libc::c_uint) << bx4;
    while x < w4 {
        let sidx_0: libc::c_int = (mask >= 0x10000 as libc::c_int as libc::c_uint)
            as libc::c_int;
        let smask_0: libc::c_uint = mask >> (sidx_0 << 4 as libc::c_int);
        let ref mut fresh1 = (*masks
            .offset(
                1 as libc::c_int as isize,
            ))[by4
            as usize][imin(
            txa[1 as libc::c_int
                as usize][0 as libc::c_int
                as usize][0 as libc::c_int as usize][x as usize] as libc::c_int,
            *a.offset(x as isize) as libc::c_int,
        ) as usize][sidx_0 as usize];
        *fresh1 = (*fresh1 as libc::c_uint | smask_0) as uint16_t;
        x += 1;
        mask <<= 1 as libc::c_int;
    }
    if skip == 0 {
        y = 0 as libc::c_int;
        mask = (1 as libc::c_uint) << by4;
        while y < h4 {
            let sidx_1: libc::c_int = (mask >= 0x10000 as libc::c_uint) as libc::c_int;
            let smask_1: libc::c_uint = mask >> (sidx_1 << 4 as libc::c_int);
            let mut ltx: libc::c_int = txa[0 as libc::c_int
                as usize][0 as libc::c_int
                as usize][y as usize][0 as libc::c_int as usize] as libc::c_int;
            let mut step: libc::c_int = txa[0 as libc::c_int
                as usize][1 as libc::c_int
                as usize][y as usize][0 as libc::c_int as usize] as libc::c_int;
            x = step;
            while x < w4 {
                let rtx: libc::c_int = txa[0 as libc::c_int
                    as usize][0 as libc::c_int as usize][y as usize][x as usize]
                    as libc::c_int;
                let ref mut fresh2 = (*masks
                    .offset(
                        0 as libc::c_int as isize,
                    ))[(bx4 + x) as usize][imin(rtx, ltx) as usize][sidx_1 as usize];
                *fresh2 = (*fresh2 as libc::c_uint | smask_1) as uint16_t;
                ltx = rtx;
                step = txa[0 as libc::c_int
                    as usize][1 as libc::c_int as usize][y as usize][x as usize]
                    as libc::c_int;
                x += step;
            }
            y += 1;
            mask <<= 1 as libc::c_int;
        }
        x = 0 as libc::c_int;
        mask = (1 as libc::c_uint) << bx4;
        while x < w4 {
            let sidx_2: libc::c_int = (mask >= 0x10000 as libc::c_uint) as libc::c_int;
            let smask_2: libc::c_uint = mask >> (sidx_2 << 4 as libc::c_int);
            let mut ttx: libc::c_int = txa[1 as libc::c_int
                as usize][0 as libc::c_int
                as usize][0 as libc::c_int as usize][x as usize] as libc::c_int;
            let mut step_0: libc::c_int = txa[1 as libc::c_int
                as usize][1 as libc::c_int
                as usize][0 as libc::c_int as usize][x as usize] as libc::c_int;
            y = step_0;
            while y < h4 {
                let btx: libc::c_int = txa[1 as libc::c_int
                    as usize][0 as libc::c_int as usize][y as usize][x as usize]
                    as libc::c_int;
                let ref mut fresh3 = (*masks
                    .offset(
                        1 as libc::c_int as isize,
                    ))[(by4 + y) as usize][imin(ttx, btx) as usize][sidx_2 as usize];
                *fresh3 = (*fresh3 as libc::c_uint | smask_2) as uint16_t;
                ttx = btx;
                step_0 = txa[1 as libc::c_int
                    as usize][1 as libc::c_int as usize][y as usize][x as usize]
                    as libc::c_int;
                y += step_0;
            }
            x += 1;
            mask <<= 1 as libc::c_int;
        }
    }
    y = 0 as libc::c_int;
    while y < h4 {
        *l
            .offset(
                y as isize,
            ) = txa[0 as libc::c_int
            as usize][0 as libc::c_int
            as usize][y as usize][(w4 - 1 as libc::c_int) as usize];
        y += 1;
    }
    memcpy(
        a as *mut libc::c_void,
        (txa[1 as libc::c_int
            as usize][0 as libc::c_int as usize][(h4 - 1 as libc::c_int) as usize])
            .as_mut_ptr() as *const libc::c_void,
        w4 as libc::c_ulong,
    );
}
#[inline]
unsafe extern "C" fn mask_edges_intra(
    masks: *mut [[[uint16_t; 2]; 3]; 32],
    by4: libc::c_int,
    bx4: libc::c_int,
    w4: libc::c_int,
    h4: libc::c_int,
    tx: RectTxfmSize,
    a: *mut uint8_t,
    l: *mut uint8_t,
) {
    let t_dim: *const TxfmInfo = &*dav1d_txfm_dimensions.as_ptr().offset(tx as isize)
        as *const TxfmInfo;
    let twl4: libc::c_int = (*t_dim).lw as libc::c_int;
    let thl4: libc::c_int = (*t_dim).lh as libc::c_int;
    let twl4c: libc::c_int = imin(2 as libc::c_int, twl4);
    let thl4c: libc::c_int = imin(2 as libc::c_int, thl4);
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut mask: libc::c_uint = (1 as libc::c_uint) << by4;
    y = 0 as libc::c_int;
    while y < h4 {
        let sidx: libc::c_int = (mask >= 0x10000 as libc::c_int as libc::c_uint)
            as libc::c_int;
        let smask: libc::c_uint = mask >> (sidx << 4 as libc::c_int);
        let ref mut fresh4 = (*masks
            .offset(
                0 as libc::c_int as isize,
            ))[bx4
            as usize][imin(twl4c, *l.offset(y as isize) as libc::c_int)
            as usize][sidx as usize];
        *fresh4 = (*fresh4 as libc::c_uint | smask) as uint16_t;
        y += 1;
        mask <<= 1 as libc::c_int;
    }
    x = 0 as libc::c_int;
    mask = (1 as libc::c_uint) << bx4;
    while x < w4 {
        let sidx_0: libc::c_int = (mask >= 0x10000 as libc::c_int as libc::c_uint)
            as libc::c_int;
        let smask_0: libc::c_uint = mask >> (sidx_0 << 4 as libc::c_int);
        let ref mut fresh5 = (*masks
            .offset(
                1 as libc::c_int as isize,
            ))[by4
            as usize][imin(thl4c, *a.offset(x as isize) as libc::c_int)
            as usize][sidx_0 as usize];
        *fresh5 = (*fresh5 as libc::c_uint | smask_0) as uint16_t;
        x += 1;
        mask <<= 1 as libc::c_int;
    }
    let hstep: libc::c_int = (*t_dim).w as libc::c_int;
    let mut t: libc::c_uint = (1 as libc::c_uint) << by4;
    let mut inner: libc::c_uint = ((t as uint64_t) << h4)
        .wrapping_sub(t as uint64_t) as libc::c_uint;
    let mut inner1: libc::c_uint = inner & 0xffff as libc::c_int as libc::c_uint;
    let mut inner2: libc::c_uint = inner >> 16 as libc::c_int;
    x = hstep;
    while x < w4 {
        if inner1 != 0 {
            let ref mut fresh6 = (*masks
                .offset(
                    0 as libc::c_int as isize,
                ))[(bx4 + x) as usize][twl4c as usize][0 as libc::c_int as usize];
            *fresh6 = (*fresh6 as libc::c_uint | inner1) as uint16_t;
        }
        if inner2 != 0 {
            let ref mut fresh7 = (*masks
                .offset(
                    0 as libc::c_int as isize,
                ))[(bx4 + x) as usize][twl4c as usize][1 as libc::c_int as usize];
            *fresh7 = (*fresh7 as libc::c_uint | inner2) as uint16_t;
        }
        x += hstep;
    }
    let vstep: libc::c_int = (*t_dim).h as libc::c_int;
    t = (1 as libc::c_uint) << bx4;
    inner = ((t as uint64_t) << w4).wrapping_sub(t as uint64_t) as libc::c_uint;
    inner1 = inner & 0xffff as libc::c_int as libc::c_uint;
    inner2 = inner >> 16 as libc::c_int;
    y = vstep;
    while y < h4 {
        if inner1 != 0 {
            let ref mut fresh8 = (*masks
                .offset(
                    1 as libc::c_int as isize,
                ))[(by4 + y) as usize][thl4c as usize][0 as libc::c_int as usize];
            *fresh8 = (*fresh8 as libc::c_uint | inner1) as uint16_t;
        }
        if inner2 != 0 {
            let ref mut fresh9 = (*masks
                .offset(
                    1 as libc::c_int as isize,
                ))[(by4 + y) as usize][thl4c as usize][1 as libc::c_int as usize];
            *fresh9 = (*fresh9 as libc::c_uint | inner2) as uint16_t;
        }
        y += vstep;
    }
    match w4 {
        1 => {
            (*(&mut *a.offset(0 as libc::c_int as isize) as *mut uint8_t as *mut alias8))
                .u8_0 = (0x1 as libc::c_int * thl4c) as uint8_t;
        }
        2 => {
            (*(&mut *a.offset(0 as libc::c_int as isize) as *mut uint8_t
                as *mut alias16))
                .u16_0 = (0x101 as libc::c_int * thl4c) as uint16_t;
        }
        4 => {
            (*(&mut *a.offset(0 as libc::c_int as isize) as *mut uint8_t
                as *mut alias32))
                .u32_0 = (0x1010101 as libc::c_uint).wrapping_mul(thl4c as libc::c_uint);
        }
        8 => {
            (*(&mut *a.offset(0 as libc::c_int as isize) as *mut uint8_t
                as *mut alias64))
                .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                .wrapping_mul(thl4c as libc::c_ulonglong) as uint64_t;
        }
        16 => {
            let const_val: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                .wrapping_mul(thl4c as libc::c_ulonglong) as uint64_t;
            (*(&mut *a.offset((0 as libc::c_int + 0 as libc::c_int) as isize)
                as *mut uint8_t as *mut alias64))
                .u64_0 = const_val;
            (*(&mut *a.offset((0 as libc::c_int + 8 as libc::c_int) as isize)
                as *mut uint8_t as *mut alias64))
                .u64_0 = const_val;
        }
        32 => {
            let const_val_0: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                .wrapping_mul(thl4c as libc::c_ulonglong) as uint64_t;
            (*(&mut *a.offset((0 as libc::c_int + 0 as libc::c_int) as isize)
                as *mut uint8_t as *mut alias64))
                .u64_0 = const_val_0;
            (*(&mut *a.offset((0 as libc::c_int + 8 as libc::c_int) as isize)
                as *mut uint8_t as *mut alias64))
                .u64_0 = const_val_0;
            (*(&mut *a.offset((0 as libc::c_int + 16 as libc::c_int) as isize)
                as *mut uint8_t as *mut alias64))
                .u64_0 = const_val_0;
            (*(&mut *a.offset((0 as libc::c_int + 24 as libc::c_int) as isize)
                as *mut uint8_t as *mut alias64))
                .u64_0 = const_val_0;
        }
        _ => {
            memset(a as *mut libc::c_void, thl4c, w4 as libc::c_ulong);
        }
    }
    match h4 {
        1 => {
            (*(&mut *l.offset(0 as libc::c_int as isize) as *mut uint8_t as *mut alias8))
                .u8_0 = (0x1 as libc::c_int * twl4c) as uint8_t;
        }
        2 => {
            (*(&mut *l.offset(0 as libc::c_int as isize) as *mut uint8_t
                as *mut alias16))
                .u16_0 = (0x101 as libc::c_int * twl4c) as uint16_t;
        }
        4 => {
            (*(&mut *l.offset(0 as libc::c_int as isize) as *mut uint8_t
                as *mut alias32))
                .u32_0 = (0x1010101 as libc::c_uint).wrapping_mul(twl4c as libc::c_uint);
        }
        8 => {
            (*(&mut *l.offset(0 as libc::c_int as isize) as *mut uint8_t
                as *mut alias64))
                .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                .wrapping_mul(twl4c as libc::c_ulonglong) as uint64_t;
        }
        16 => {
            let const_val_1: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                .wrapping_mul(twl4c as libc::c_ulonglong) as uint64_t;
            (*(&mut *l.offset((0 as libc::c_int + 0 as libc::c_int) as isize)
                as *mut uint8_t as *mut alias64))
                .u64_0 = const_val_1;
            (*(&mut *l.offset((0 as libc::c_int + 8 as libc::c_int) as isize)
                as *mut uint8_t as *mut alias64))
                .u64_0 = const_val_1;
        }
        32 => {
            let const_val_2: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                .wrapping_mul(twl4c as libc::c_ulonglong) as uint64_t;
            (*(&mut *l.offset((0 as libc::c_int + 0 as libc::c_int) as isize)
                as *mut uint8_t as *mut alias64))
                .u64_0 = const_val_2;
            (*(&mut *l.offset((0 as libc::c_int + 8 as libc::c_int) as isize)
                as *mut uint8_t as *mut alias64))
                .u64_0 = const_val_2;
            (*(&mut *l.offset((0 as libc::c_int + 16 as libc::c_int) as isize)
                as *mut uint8_t as *mut alias64))
                .u64_0 = const_val_2;
            (*(&mut *l.offset((0 as libc::c_int + 24 as libc::c_int) as isize)
                as *mut uint8_t as *mut alias64))
                .u64_0 = const_val_2;
        }
        _ => {
            memset(l as *mut libc::c_void, twl4c, h4 as libc::c_ulong);
        }
    };
}
unsafe extern "C" fn mask_edges_chroma(
    masks: *mut [[[uint16_t; 2]; 2]; 32],
    cby4: libc::c_int,
    cbx4: libc::c_int,
    cw4: libc::c_int,
    ch4: libc::c_int,
    skip_inter: libc::c_int,
    tx: RectTxfmSize,
    a: *mut uint8_t,
    l: *mut uint8_t,
    ss_hor: libc::c_int,
    ss_ver: libc::c_int,
) {
    let t_dim: *const TxfmInfo = &*dav1d_txfm_dimensions.as_ptr().offset(tx as isize)
        as *const TxfmInfo;
    let twl4: libc::c_int = (*t_dim).lw as libc::c_int;
    let thl4: libc::c_int = (*t_dim).lh as libc::c_int;
    let twl4c: libc::c_int = (twl4 != 0) as libc::c_int;
    let thl4c: libc::c_int = (thl4 != 0) as libc::c_int;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let vbits: libc::c_int = 4 as libc::c_int - ss_ver;
    let hbits: libc::c_int = 4 as libc::c_int - ss_hor;
    let vmask: libc::c_int = 16 as libc::c_int >> ss_ver;
    let hmask: libc::c_int = 16 as libc::c_int >> ss_hor;
    let vmax: libc::c_uint = ((1 as libc::c_int) << vmask) as libc::c_uint;
    let hmax: libc::c_uint = ((1 as libc::c_int) << hmask) as libc::c_uint;
    let mut mask: libc::c_uint = (1 as libc::c_uint) << cby4;
    y = 0 as libc::c_int;
    while y < ch4 {
        let sidx: libc::c_int = (mask >= vmax) as libc::c_int;
        let smask: libc::c_uint = mask >> (sidx << vbits);
        let ref mut fresh10 = (*masks
            .offset(
                0 as libc::c_int as isize,
            ))[cbx4
            as usize][imin(twl4c, *l.offset(y as isize) as libc::c_int)
            as usize][sidx as usize];
        *fresh10 = (*fresh10 as libc::c_uint | smask) as uint16_t;
        y += 1;
        mask <<= 1 as libc::c_int;
    }
    x = 0 as libc::c_int;
    mask = (1 as libc::c_uint) << cbx4;
    while x < cw4 {
        let sidx_0: libc::c_int = (mask >= hmax) as libc::c_int;
        let smask_0: libc::c_uint = mask >> (sidx_0 << hbits);
        let ref mut fresh11 = (*masks
            .offset(
                1 as libc::c_int as isize,
            ))[cby4
            as usize][imin(thl4c, *a.offset(x as isize) as libc::c_int)
            as usize][sidx_0 as usize];
        *fresh11 = (*fresh11 as libc::c_uint | smask_0) as uint16_t;
        x += 1;
        mask <<= 1 as libc::c_int;
    }
    if skip_inter == 0 {
        let hstep: libc::c_int = (*t_dim).w as libc::c_int;
        let mut t: libc::c_uint = (1 as libc::c_uint) << cby4;
        let mut inner: libc::c_uint = ((t as uint64_t) << ch4)
            .wrapping_sub(t as uint64_t) as libc::c_uint;
        let mut inner1: libc::c_uint = inner
            & (((1 as libc::c_int) << vmask) - 1 as libc::c_int) as libc::c_uint;
        let mut inner2: libc::c_uint = inner >> vmask;
        x = hstep;
        while x < cw4 {
            if inner1 != 0 {
                let ref mut fresh12 = (*masks
                    .offset(
                        0 as libc::c_int as isize,
                    ))[(cbx4 + x) as usize][twl4c as usize][0 as libc::c_int as usize];
                *fresh12 = (*fresh12 as libc::c_uint | inner1) as uint16_t;
            }
            if inner2 != 0 {
                let ref mut fresh13 = (*masks
                    .offset(
                        0 as libc::c_int as isize,
                    ))[(cbx4 + x) as usize][twl4c as usize][1 as libc::c_int as usize];
                *fresh13 = (*fresh13 as libc::c_uint | inner2) as uint16_t;
            }
            x += hstep;
        }
        let vstep: libc::c_int = (*t_dim).h as libc::c_int;
        t = (1 as libc::c_uint) << cbx4;
        inner = ((t as uint64_t) << cw4).wrapping_sub(t as uint64_t)
            as libc::c_uint;
        inner1 = inner
            & (((1 as libc::c_int) << hmask) - 1 as libc::c_int) as libc::c_uint;
        inner2 = inner >> hmask;
        y = vstep;
        while y < ch4 {
            if inner1 != 0 {
                let ref mut fresh14 = (*masks
                    .offset(
                        1 as libc::c_int as isize,
                    ))[(cby4 + y) as usize][thl4c as usize][0 as libc::c_int as usize];
                *fresh14 = (*fresh14 as libc::c_uint | inner1) as uint16_t;
            }
            if inner2 != 0 {
                let ref mut fresh15 = (*masks
                    .offset(
                        1 as libc::c_int as isize,
                    ))[(cby4 + y) as usize][thl4c as usize][1 as libc::c_int as usize];
                *fresh15 = (*fresh15 as libc::c_uint | inner2) as uint16_t;
            }
            y += vstep;
        }
    }
    match cw4 {
        1 => {
            (*(&mut *a.offset(0 as libc::c_int as isize) as *mut uint8_t as *mut alias8))
                .u8_0 = (0x1 as libc::c_int * thl4c) as uint8_t;
        }
        2 => {
            (*(&mut *a.offset(0 as libc::c_int as isize) as *mut uint8_t
                as *mut alias16))
                .u16_0 = (0x101 as libc::c_int * thl4c) as uint16_t;
        }
        4 => {
            (*(&mut *a.offset(0 as libc::c_int as isize) as *mut uint8_t
                as *mut alias32))
                .u32_0 = (0x1010101 as libc::c_uint).wrapping_mul(thl4c as libc::c_uint);
        }
        8 => {
            (*(&mut *a.offset(0 as libc::c_int as isize) as *mut uint8_t
                as *mut alias64))
                .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                .wrapping_mul(thl4c as libc::c_ulonglong) as uint64_t;
        }
        16 => {
            let const_val: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                .wrapping_mul(thl4c as libc::c_ulonglong) as uint64_t;
            (*(&mut *a.offset((0 as libc::c_int + 0 as libc::c_int) as isize)
                as *mut uint8_t as *mut alias64))
                .u64_0 = const_val;
            (*(&mut *a.offset((0 as libc::c_int + 8 as libc::c_int) as isize)
                as *mut uint8_t as *mut alias64))
                .u64_0 = const_val;
        }
        32 => {
            let const_val_0: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                .wrapping_mul(thl4c as libc::c_ulonglong) as uint64_t;
            (*(&mut *a.offset((0 as libc::c_int + 0 as libc::c_int) as isize)
                as *mut uint8_t as *mut alias64))
                .u64_0 = const_val_0;
            (*(&mut *a.offset((0 as libc::c_int + 8 as libc::c_int) as isize)
                as *mut uint8_t as *mut alias64))
                .u64_0 = const_val_0;
            (*(&mut *a.offset((0 as libc::c_int + 16 as libc::c_int) as isize)
                as *mut uint8_t as *mut alias64))
                .u64_0 = const_val_0;
            (*(&mut *a.offset((0 as libc::c_int + 24 as libc::c_int) as isize)
                as *mut uint8_t as *mut alias64))
                .u64_0 = const_val_0;
        }
        _ => {
            memset(a as *mut libc::c_void, thl4c, cw4 as libc::c_ulong);
        }
    }
    match ch4 {
        1 => {
            (*(&mut *l.offset(0 as libc::c_int as isize) as *mut uint8_t as *mut alias8))
                .u8_0 = (0x1 as libc::c_int * twl4c) as uint8_t;
        }
        2 => {
            (*(&mut *l.offset(0 as libc::c_int as isize) as *mut uint8_t
                as *mut alias16))
                .u16_0 = (0x101 as libc::c_int * twl4c) as uint16_t;
        }
        4 => {
            (*(&mut *l.offset(0 as libc::c_int as isize) as *mut uint8_t
                as *mut alias32))
                .u32_0 = (0x1010101 as libc::c_uint).wrapping_mul(twl4c as libc::c_uint);
        }
        8 => {
            (*(&mut *l.offset(0 as libc::c_int as isize) as *mut uint8_t
                as *mut alias64))
                .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                .wrapping_mul(twl4c as libc::c_ulonglong) as uint64_t;
        }
        16 => {
            let const_val_1: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                .wrapping_mul(twl4c as libc::c_ulonglong) as uint64_t;
            (*(&mut *l.offset((0 as libc::c_int + 0 as libc::c_int) as isize)
                as *mut uint8_t as *mut alias64))
                .u64_0 = const_val_1;
            (*(&mut *l.offset((0 as libc::c_int + 8 as libc::c_int) as isize)
                as *mut uint8_t as *mut alias64))
                .u64_0 = const_val_1;
        }
        32 => {
            let const_val_2: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                .wrapping_mul(twl4c as libc::c_ulonglong) as uint64_t;
            (*(&mut *l.offset((0 as libc::c_int + 0 as libc::c_int) as isize)
                as *mut uint8_t as *mut alias64))
                .u64_0 = const_val_2;
            (*(&mut *l.offset((0 as libc::c_int + 8 as libc::c_int) as isize)
                as *mut uint8_t as *mut alias64))
                .u64_0 = const_val_2;
            (*(&mut *l.offset((0 as libc::c_int + 16 as libc::c_int) as isize)
                as *mut uint8_t as *mut alias64))
                .u64_0 = const_val_2;
            (*(&mut *l.offset((0 as libc::c_int + 24 as libc::c_int) as isize)
                as *mut uint8_t as *mut alias64))
                .u64_0 = const_val_2;
        }
        _ => {
            memset(l as *mut libc::c_void, twl4c, ch4 as libc::c_ulong);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_create_lf_mask_intra(
    lflvl: *mut Av1Filter,
    level_cache: *mut [uint8_t; 4],
    b4_stride: ptrdiff_t,
    mut filter_level: *const [[uint8_t; 2]; 8],
    bx: libc::c_int,
    by: libc::c_int,
    iw: libc::c_int,
    ih: libc::c_int,
    bs: BlockSize,
    ytx: RectTxfmSize,
    uvtx: RectTxfmSize,
    layout: Dav1dPixelLayout,
    ay: *mut uint8_t,
    ly: *mut uint8_t,
    auv: *mut uint8_t,
    luv: *mut uint8_t,
) {
    let b_dim: *const uint8_t = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bw4: libc::c_int = imin(
        iw - bx,
        *b_dim.offset(0 as libc::c_int as isize) as libc::c_int,
    );
    let bh4: libc::c_int = imin(
        ih - by,
        *b_dim.offset(1 as libc::c_int as isize) as libc::c_int,
    );
    let bx4: libc::c_int = bx & 31 as libc::c_int;
    let by4: libc::c_int = by & 31 as libc::c_int;
    if bw4 != 0 && bh4 != 0 {
        let mut level_cache_ptr: *mut [uint8_t; 4] = level_cache
            .offset(by as isize * b4_stride)
            .offset(bx as isize);
        let mut y: libc::c_int = 0 as libc::c_int;
        while y < bh4 {
            let mut x: libc::c_int = 0 as libc::c_int;
            while x < bw4 {
                (*level_cache_ptr
                    .offset(
                        x as isize,
                    ))[0 as libc::c_int
                    as usize] = (*filter_level
                    .offset(
                        0 as libc::c_int as isize,
                    ))[0 as libc::c_int as usize][0 as libc::c_int as usize];
                (*level_cache_ptr
                    .offset(
                        x as isize,
                    ))[1 as libc::c_int
                    as usize] = (*filter_level
                    .offset(
                        1 as libc::c_int as isize,
                    ))[0 as libc::c_int as usize][0 as libc::c_int as usize];
                x += 1;
            }
            level_cache_ptr = level_cache_ptr.offset(b4_stride as isize);
            y += 1;
        }
        mask_edges_intra(
            ((*lflvl).filter_y).as_mut_ptr(),
            by4,
            bx4,
            bw4,
            bh4,
            ytx,
            ay,
            ly,
        );
    }
    if auv.is_null() {
        return;
    }
    let ss_ver: libc::c_int = (layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_hor: libc::c_int = (layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
    let cbw4: libc::c_int = imin(
        (iw + ss_hor >> ss_hor) - (bx >> ss_hor),
        *b_dim.offset(0 as libc::c_int as isize) as libc::c_int + ss_hor >> ss_hor,
    );
    let cbh4: libc::c_int = imin(
        (ih + ss_ver >> ss_ver) - (by >> ss_ver),
        *b_dim.offset(1 as libc::c_int as isize) as libc::c_int + ss_ver >> ss_ver,
    );
    if cbw4 == 0 || cbh4 == 0 {
        return;
    }
    let cbx4: libc::c_int = bx4 >> ss_hor;
    let cby4: libc::c_int = by4 >> ss_ver;
    let mut level_cache_ptr_0: *mut [uint8_t; 4] = level_cache
        .offset(((by >> ss_ver) as isize * b4_stride) as isize)
        .offset((bx >> ss_hor) as isize);
    let mut y_0: libc::c_int = 0 as libc::c_int;
    while y_0 < cbh4 {
        let mut x_0: libc::c_int = 0 as libc::c_int;
        while x_0 < cbw4 {
            (*level_cache_ptr_0
                .offset(
                    x_0 as isize,
                ))[2 as libc::c_int
                as usize] = (*filter_level
                .offset(
                    2 as libc::c_int as isize,
                ))[0 as libc::c_int as usize][0 as libc::c_int as usize];
            (*level_cache_ptr_0
                .offset(
                    x_0 as isize,
                ))[3 as libc::c_int
                as usize] = (*filter_level
                .offset(
                    3 as libc::c_int as isize,
                ))[0 as libc::c_int as usize][0 as libc::c_int as usize];
            x_0 += 1;
        }
        level_cache_ptr_0 = level_cache_ptr_0.offset(b4_stride as isize);
        y_0 += 1;
    }
    mask_edges_chroma(
        ((*lflvl).filter_uv).as_mut_ptr(),
        cby4,
        cbx4,
        cbw4,
        cbh4,
        0 as libc::c_int,
        uvtx,
        auv,
        luv,
        ss_hor,
        ss_ver,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_create_lf_mask_inter(
    lflvl: *mut Av1Filter,
    level_cache: *mut [uint8_t; 4],
    b4_stride: ptrdiff_t,
    mut filter_level: *const [[uint8_t; 2]; 8],
    bx: libc::c_int,
    by: libc::c_int,
    iw: libc::c_int,
    ih: libc::c_int,
    skip: libc::c_int,
    bs: BlockSize,
    max_ytx: RectTxfmSize,
    tx_masks: *const uint16_t,
    uvtx: RectTxfmSize,
    layout: Dav1dPixelLayout,
    ay: *mut uint8_t,
    ly: *mut uint8_t,
    auv: *mut uint8_t,
    luv: *mut uint8_t,
) {
    let b_dim: *const uint8_t = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bw4: libc::c_int = imin(
        iw - bx,
        *b_dim.offset(0 as libc::c_int as isize) as libc::c_int,
    );
    let bh4: libc::c_int = imin(
        ih - by,
        *b_dim.offset(1 as libc::c_int as isize) as libc::c_int,
    );
    let bx4: libc::c_int = bx & 31 as libc::c_int;
    let by4: libc::c_int = by & 31 as libc::c_int;
    if bw4 != 0 && bh4 != 0 {
        let mut level_cache_ptr: *mut [uint8_t; 4] = level_cache
            .offset(by as isize * b4_stride)
            .offset(bx as isize);
        let mut y: libc::c_int = 0 as libc::c_int;
        while y < bh4 {
            let mut x: libc::c_int = 0 as libc::c_int;
            while x < bw4 {
                (*level_cache_ptr
                    .offset(
                        x as isize,
                    ))[0 as libc::c_int
                    as usize] = (*filter_level
                    .offset(
                        0 as libc::c_int as isize,
                    ))[0 as libc::c_int as usize][0 as libc::c_int as usize];
                (*level_cache_ptr
                    .offset(
                        x as isize,
                    ))[1 as libc::c_int
                    as usize] = (*filter_level
                    .offset(
                        1 as libc::c_int as isize,
                    ))[0 as libc::c_int as usize][0 as libc::c_int as usize];
                x += 1;
            }
            level_cache_ptr = level_cache_ptr.offset(b4_stride as isize);
            y += 1;
        }
        mask_edges_inter(
            ((*lflvl).filter_y).as_mut_ptr(),
            by4,
            bx4,
            bw4,
            bh4,
            skip,
            max_ytx,
            tx_masks,
            ay,
            ly,
        );
    }
    if auv.is_null() {
        return;
    }
    let ss_ver: libc::c_int = (layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_hor: libc::c_int = (layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
    let cbw4: libc::c_int = imin(
        (iw + ss_hor >> ss_hor) - (bx >> ss_hor),
        *b_dim.offset(0 as libc::c_int as isize) as libc::c_int + ss_hor >> ss_hor,
    );
    let cbh4: libc::c_int = imin(
        (ih + ss_ver >> ss_ver) - (by >> ss_ver),
        *b_dim.offset(1 as libc::c_int as isize) as libc::c_int + ss_ver >> ss_ver,
    );
    if cbw4 == 0 || cbh4 == 0 {
        return;
    }
    let cbx4: libc::c_int = bx4 >> ss_hor;
    let cby4: libc::c_int = by4 >> ss_ver;
    let mut level_cache_ptr_0: *mut [uint8_t; 4] = level_cache
        .offset(((by >> ss_ver) as isize * b4_stride) as isize)
        .offset((bx >> ss_hor) as isize);
    let mut y_0: libc::c_int = 0 as libc::c_int;
    while y_0 < cbh4 {
        let mut x_0: libc::c_int = 0 as libc::c_int;
        while x_0 < cbw4 {
            (*level_cache_ptr_0
                .offset(
                    x_0 as isize,
                ))[2 as libc::c_int
                as usize] = (*filter_level
                .offset(
                    2 as libc::c_int as isize,
                ))[0 as libc::c_int as usize][0 as libc::c_int as usize];
            (*level_cache_ptr_0
                .offset(
                    x_0 as isize,
                ))[3 as libc::c_int
                as usize] = (*filter_level
                .offset(
                    3 as libc::c_int as isize,
                ))[0 as libc::c_int as usize][0 as libc::c_int as usize];
            x_0 += 1;
        }
        level_cache_ptr_0 = level_cache_ptr_0.offset(b4_stride as isize);
        y_0 += 1;
    }
    mask_edges_chroma(
        ((*lflvl).filter_uv).as_mut_ptr(),
        cby4,
        cbx4,
        cbw4,
        cbh4,
        skip,
        uvtx,
        auv,
        luv,
        ss_hor,
        ss_ver,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_calc_eih(
    lim_lut: *mut Av1FilterLUT,
    filter_sharpness: libc::c_int,
) {
    let sharp: libc::c_int = filter_sharpness;
    let mut level: libc::c_int = 0 as libc::c_int;
    while level < 64 as libc::c_int {
        let mut limit: libc::c_int = level;
        if sharp > 0 as libc::c_int {
            limit >>= sharp + 3 as libc::c_int >> 2 as libc::c_int;
            limit = imin(limit, 9 as libc::c_int - sharp);
        }
        limit = imax(limit, 1 as libc::c_int);
        (*lim_lut).i[level as usize] = limit as uint8_t;
        (*lim_lut)
            .e[level
            as usize] = (2 as libc::c_int * (level + 2 as libc::c_int) + limit)
            as uint8_t;
        level += 1;
    }
    (*lim_lut)
        .sharp[0 as libc::c_int
        as usize] = (sharp + 3 as libc::c_int >> 2 as libc::c_int) as uint64_t;
    (*lim_lut)
        .sharp[1 as libc::c_int
        as usize] = (if sharp != 0 {
        9 as libc::c_int - sharp
    } else {
        0xff as libc::c_int
    }) as uint64_t;
}
unsafe extern "C" fn calc_lf_value(
    lflvl_values: *mut [uint8_t; 2],
    base_lvl: libc::c_int,
    lf_delta: libc::c_int,
    seg_delta: libc::c_int,
    mr_delta: *const Dav1dLoopfilterModeRefDeltas,
) {
    let base: libc::c_int = iclip(
        iclip(base_lvl + lf_delta, 0 as libc::c_int, 63 as libc::c_int) + seg_delta,
        0 as libc::c_int,
        63 as libc::c_int,
    );
    if mr_delta.is_null() {
        memset(
            lflvl_values as *mut libc::c_void,
            base,
            (8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong,
        );
    } else {
        let sh: libc::c_int = (base >= 32 as libc::c_int) as libc::c_int;
        let ref mut fresh16 = (*lflvl_values
            .offset(0 as libc::c_int as isize))[1 as libc::c_int as usize];
        *fresh16 = iclip(
            base
                + (*mr_delta).ref_delta[0 as libc::c_int as usize]
                    * ((1 as libc::c_int) << sh),
            0 as libc::c_int,
            63 as libc::c_int,
        ) as uint8_t;
        (*lflvl_values
            .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] = *fresh16;
        let mut r: libc::c_int = 1 as libc::c_int;
        while r < 8 as libc::c_int {
            let mut m: libc::c_int = 0 as libc::c_int;
            while m < 2 as libc::c_int {
                let delta: libc::c_int = (*mr_delta).mode_delta[m as usize]
                    + (*mr_delta).ref_delta[r as usize];
                (*lflvl_values
                    .offset(
                        r as isize,
                    ))[m
                    as usize] = iclip(
                    base + delta * ((1 as libc::c_int) << sh),
                    0 as libc::c_int,
                    63 as libc::c_int,
                ) as uint8_t;
                m += 1;
            }
            r += 1;
        }
    };
}
#[inline]
unsafe extern "C" fn calc_lf_value_chroma(
    lflvl_values: *mut [uint8_t; 2],
    base_lvl: libc::c_int,
    lf_delta: libc::c_int,
    seg_delta: libc::c_int,
    mr_delta: *const Dav1dLoopfilterModeRefDeltas,
) {
    if base_lvl == 0 {
        memset(
            lflvl_values as *mut libc::c_void,
            0 as libc::c_int,
            (8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong,
        );
    } else {
        calc_lf_value(lflvl_values, base_lvl, lf_delta, seg_delta, mr_delta);
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_calc_lf_values(
    lflvl_values: *mut [[[uint8_t; 2]; 8]; 4],
    hdr: *const Dav1dFrameHeader,
    mut lf_delta: *const int8_t,
) {
    let n_seg: libc::c_int = if (*hdr).segmentation.enabled != 0 {
        8 as libc::c_int
    } else {
        1 as libc::c_int
    };
    if (*hdr).loopfilter.level_y[0 as libc::c_int as usize] == 0
        && (*hdr).loopfilter.level_y[1 as libc::c_int as usize] == 0
    {
        memset(
            lflvl_values as *mut libc::c_void,
            0 as libc::c_int,
            (8 as libc::c_int * 4 as libc::c_int * 2 as libc::c_int * n_seg)
                as libc::c_ulong,
        );
        return;
    }
    let mr_deltas: *const Dav1dLoopfilterModeRefDeltas = if (*hdr)
        .loopfilter
        .mode_ref_delta_enabled != 0
    {
        &(*hdr).loopfilter.mode_ref_deltas
    } else {
        0 as *const Dav1dLoopfilterModeRefDeltas
    };
    let mut s: libc::c_int = 0 as libc::c_int;
    while s < n_seg {
        let segd: *const Dav1dSegmentationData = if (*hdr).segmentation.enabled != 0 {
            &*((*hdr).segmentation.seg_data.d).as_ptr().offset(s as isize)
                as *const Dav1dSegmentationData
        } else {
            0 as *const Dav1dSegmentationData
        };
        calc_lf_value(
            ((*lflvl_values.offset(s as isize))[0 as libc::c_int as usize]).as_mut_ptr(),
            (*hdr).loopfilter.level_y[0 as libc::c_int as usize],
            *lf_delta.offset(0 as libc::c_int as isize) as libc::c_int,
            if !segd.is_null() { (*segd).delta_lf_y_v } else { 0 as libc::c_int },
            mr_deltas,
        );
        calc_lf_value(
            ((*lflvl_values.offset(s as isize))[1 as libc::c_int as usize]).as_mut_ptr(),
            (*hdr).loopfilter.level_y[1 as libc::c_int as usize],
            *lf_delta
                .offset(
                    (if (*hdr).delta.lf.multi != 0 {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as isize,
                ) as libc::c_int,
            if !segd.is_null() { (*segd).delta_lf_y_h } else { 0 as libc::c_int },
            mr_deltas,
        );
        calc_lf_value_chroma(
            ((*lflvl_values.offset(s as isize))[2 as libc::c_int as usize]).as_mut_ptr(),
            (*hdr).loopfilter.level_u,
            *lf_delta
                .offset(
                    (if (*hdr).delta.lf.multi != 0 {
                        2 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as isize,
                ) as libc::c_int,
            if !segd.is_null() { (*segd).delta_lf_u } else { 0 as libc::c_int },
            mr_deltas,
        );
        calc_lf_value_chroma(
            ((*lflvl_values.offset(s as isize))[3 as libc::c_int as usize]).as_mut_ptr(),
            (*hdr).loopfilter.level_v,
            *lf_delta
                .offset(
                    (if (*hdr).delta.lf.multi != 0 {
                        3 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as isize,
                ) as libc::c_int,
            if !segd.is_null() { (*segd).delta_lf_v } else { 0 as libc::c_int },
            mr_deltas,
        );
        s += 1;
    }
}
