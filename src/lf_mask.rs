use crate::include::common::intops::iclip;
use crate::include::common::intops::imin;
use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::dav1d::headers::Dav1dLoopfilterModeRefDeltas;
use crate::include::dav1d::headers::Dav1dPixelLayout;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;
use crate::include::stddef::ptrdiff_t;
use crate::src::align::Align16;
use crate::src::ctx::alias16;
use crate::src::ctx::alias32;
use crate::src::ctx::alias64;
use crate::src::ctx::alias8;
use crate::src::ctx::case_set_upto16;
use crate::src::ctx::SetCtxFn;
use crate::src::levels::BlockSize;
use crate::src::levels::RectTxfmSize;
use crate::src::levels::TX_4X4;
use crate::src::tables::dav1d_block_dimensions;
use crate::src::tables::dav1d_txfm_dimensions;
use crate::src::tables::TxfmInfo;

extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Av1FilterLUT {
    pub e: [u8; 64],
    pub i: [u8; 64],
    pub sharp: [u64; 2],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Av1RestorationUnit {
    pub type_0: u8,
    pub filter_h: [i8; 3],
    pub filter_v: [i8; 3],
    pub sgr_idx: u8,
    pub sgr_weights: [i8; 2],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Av1Filter {
    pub filter_y: [[[[u16; 2]; 3]; 32]; 2],
    pub filter_uv: [[[[u16; 2]; 2]; 32]; 2],
    pub cdef_idx: [i8; 4],
    pub noskip_mask: [[u16; 2]; 16],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Av1Restoration {
    pub lr: [[Av1RestorationUnit; 4]; 3],
}

unsafe fn decomp_tx(
    txa: *mut [[[u8; 32]; 32]; 2],
    from: RectTxfmSize,
    depth: libc::c_int,
    y_off: libc::c_int,
    x_off: libc::c_int,
    tx_masks: *const u16,
) {
    let t_dim: *const TxfmInfo =
        &*dav1d_txfm_dimensions.as_ptr().offset(from as isize) as *const TxfmInfo;
    let is_split = if from as libc::c_uint == TX_4X4 as libc::c_int as libc::c_uint || depth > 1 {
        0 as libc::c_int
    } else {
        *tx_masks.offset(depth as isize) as libc::c_int >> y_off * 4 + x_off & 1
    };
    if is_split != 0 {
        let sub: RectTxfmSize = (*t_dim).sub as RectTxfmSize;
        let htw4 = (*t_dim).w as libc::c_int >> 1;
        let hth4 = (*t_dim).h as libc::c_int >> 1;
        decomp_tx(txa, sub, depth + 1, y_off * 2 + 0, x_off * 2 + 0, tx_masks);
        if (*t_dim).w as libc::c_int >= (*t_dim).h as libc::c_int {
            decomp_tx(
                &mut *(*(*(*txa.offset(0)).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(0))
                .as_mut_ptr()
                .offset(htw4 as isize) as *mut u8 as *mut [[[u8; 32]; 32]; 2],
                sub,
                depth + 1,
                y_off * 2 + 0,
                x_off * 2 + 1,
                tx_masks,
            );
        }
        if (*t_dim).h as libc::c_int >= (*t_dim).w as libc::c_int {
            decomp_tx(
                &mut *(*(*(*txa.offset(0)).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(hth4 as isize))
                .as_mut_ptr()
                .offset(0) as *mut u8 as *mut [[[u8; 32]; 32]; 2],
                sub,
                depth + 1,
                y_off * 2 + 1,
                x_off * 2 + 0,
                tx_masks,
            );
            if (*t_dim).w as libc::c_int >= (*t_dim).h as libc::c_int {
                decomp_tx(
                    &mut *(*(*(*txa.offset(0)).as_mut_ptr().offset(0))
                        .as_mut_ptr()
                        .offset(hth4 as isize))
                    .as_mut_ptr()
                    .offset(htw4 as isize) as *mut u8
                        as *mut [[[u8; 32]; 32]; 2],
                    sub,
                    depth + 1,
                    y_off * 2 + 1,
                    x_off * 2 + 1,
                    tx_masks,
                );
            }
        }
    } else {
        let lw = imin(2 as libc::c_int, (*t_dim).lw as libc::c_int);
        let lh = imin(2 as libc::c_int, (*t_dim).lh as libc::c_int);

        let mut set_ctx = |_dir: &mut (), _diridx, off, mul, rep_macro: SetCtxFn| {
            let mut y = 0;
            while y < (*t_dim).h as libc::c_int {
                rep_macro(
                    (*txa.offset(0))[0][y as usize].as_mut_ptr(),
                    off,
                    mul * lw as u64,
                );
                rep_macro(
                    (*txa.offset(1))[0][y as usize].as_mut_ptr(),
                    off,
                    mul * lh as u64,
                );
                (*txa.offset(0))[1][y as usize][0] = (*t_dim).w;
                y += 1;
            }
        };
        case_set_upto16(
            (*t_dim).w as libc::c_int,
            &mut (), // Was nothing in C.
            Default::default(), // Was nothing in C.
            0,
            &mut set_ctx,
        );

        let mut set_ctx = |_dir: &mut (), _diridx, off, mul, rep_macro: SetCtxFn| {
            rep_macro(
                (*txa.offset(1))[1][0].as_mut_ptr(),
                off,
                mul * (*t_dim).h as u64,
            );
        };
        case_set_upto16(
            (*t_dim).w as libc::c_int,
            &mut (), // Was nothing in C.
            Default::default(), // Was nothing in C.
            0,
            &mut set_ctx,
        );
    };
}

#[inline]
unsafe fn mask_edges_inter(
    masks: *mut [[[u16; 2]; 3]; 32],
    by4: libc::c_int,
    bx4: libc::c_int,
    w4: libc::c_int,
    h4: libc::c_int,
    skip: libc::c_int,
    max_tx: RectTxfmSize,
    tx_masks: *const u16,
    a: *mut u8,
    l: *mut u8,
) {
    let t_dim: *const TxfmInfo =
        &*dav1d_txfm_dimensions.as_ptr().offset(max_tx as isize) as *const TxfmInfo;
    let mut y = 0;
    let mut x = 0;
    let mut txa: Align16<[[[[u8; 32]; 32]; 2]; 2]> = Align16([[[[0; 32]; 32]; 2]; 2]);
    let mut y_off = 0;
    let mut y_0 = 0;
    while y_0 < h4 {
        let mut x_off = 0;
        let mut x_0 = 0;
        while x_0 < w4 {
            decomp_tx(
                &mut *(*(*(*txa.0.as_mut_ptr().offset(0)).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(y_0 as isize))
                .as_mut_ptr()
                .offset(x_0 as isize) as *mut u8 as *mut [[[u8; 32]; 32]; 2],
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
        let sidx = (mask >= 0x10000 as libc::c_int as libc::c_uint) as libc::c_int;
        let smask: libc::c_uint = mask >> (sidx << 4);
        let ref mut fresh0 = (*masks.offset(0))[bx4 as usize][imin(
            txa[0][0][y as usize][0] as libc::c_int,
            *l.offset(y as isize) as libc::c_int,
        ) as usize][sidx as usize];
        *fresh0 = (*fresh0 as libc::c_uint | smask) as u16;
        y += 1;
        mask <<= 1;
    }
    x = 0 as libc::c_int;
    mask = (1 as libc::c_uint) << bx4;
    while x < w4 {
        let sidx_0 = (mask >= 0x10000 as libc::c_int as libc::c_uint) as libc::c_int;
        let smask_0: libc::c_uint = mask >> (sidx_0 << 4);
        let ref mut fresh1 = (*masks.offset(1))[by4 as usize][imin(
            txa[1][0][0][x as usize] as libc::c_int,
            *a.offset(x as isize) as libc::c_int,
        ) as usize][sidx_0 as usize];
        *fresh1 = (*fresh1 as libc::c_uint | smask_0) as u16;
        x += 1;
        mask <<= 1;
    }
    if skip == 0 {
        y = 0 as libc::c_int;
        mask = (1 as libc::c_uint) << by4;
        while y < h4 {
            let sidx_1 = (mask >= 0x10000 as libc::c_uint) as libc::c_int;
            let smask_1: libc::c_uint = mask >> (sidx_1 << 4);
            let mut ltx = txa[0][0][y as usize][0] as libc::c_int;
            let mut step = txa[0][1][y as usize][0] as libc::c_int;
            x = step;
            while x < w4 {
                let rtx = txa[0][0][y as usize][x as usize] as libc::c_int;
                let ref mut fresh2 = (*masks.offset(0))[(bx4 + x) as usize]
                    [imin(rtx, ltx) as usize][sidx_1 as usize];
                *fresh2 = (*fresh2 as libc::c_uint | smask_1) as u16;
                ltx = rtx;
                step = txa[0][1][y as usize][x as usize] as libc::c_int;
                x += step;
            }
            y += 1;
            mask <<= 1;
        }
        x = 0 as libc::c_int;
        mask = (1 as libc::c_uint) << bx4;
        while x < w4 {
            let sidx_2 = (mask >= 0x10000 as libc::c_uint) as libc::c_int;
            let smask_2: libc::c_uint = mask >> (sidx_2 << 4);
            let mut ttx = txa[1][0][0][x as usize] as libc::c_int;
            let mut step_0 = txa[1][1][0][x as usize] as libc::c_int;
            y = step_0;
            while y < h4 {
                let btx = txa[1][0][y as usize][x as usize] as libc::c_int;
                let ref mut fresh3 = (*masks.offset(1))[(by4 + y) as usize]
                    [imin(ttx, btx) as usize][sidx_2 as usize];
                *fresh3 = (*fresh3 as libc::c_uint | smask_2) as u16;
                ttx = btx;
                step_0 = txa[1][1][y as usize][x as usize] as libc::c_int;
                y += step_0;
            }
            x += 1;
            mask <<= 1;
        }
    }
    y = 0 as libc::c_int;
    while y < h4 {
        *l.offset(y as isize) = txa[0][0][y as usize][(w4 - 1) as usize];
        y += 1;
    }
    memcpy(
        a as *mut libc::c_void,
        (txa[1][0][(h4 - 1) as usize]).as_mut_ptr() as *const libc::c_void,
        w4 as libc::c_ulong,
    );
}

#[inline]
unsafe fn mask_edges_intra(
    masks: *mut [[[u16; 2]; 3]; 32],
    by4: libc::c_int,
    bx4: libc::c_int,
    w4: libc::c_int,
    h4: libc::c_int,
    tx: RectTxfmSize,
    a: *mut u8,
    l: *mut u8,
) {
    let t_dim: *const TxfmInfo =
        &*dav1d_txfm_dimensions.as_ptr().offset(tx as isize) as *const TxfmInfo;
    let twl4 = (*t_dim).lw as libc::c_int;
    let thl4 = (*t_dim).lh as libc::c_int;
    let twl4c = imin(2 as libc::c_int, twl4);
    let thl4c = imin(2 as libc::c_int, thl4);
    let mut y = 0;
    let mut x = 0;
    let mut mask: libc::c_uint = (1 as libc::c_uint) << by4;
    y = 0 as libc::c_int;
    while y < h4 {
        let sidx = (mask >= 0x10000 as libc::c_int as libc::c_uint) as libc::c_int;
        let smask: libc::c_uint = mask >> (sidx << 4);
        let ref mut fresh4 = (*masks.offset(0))[bx4 as usize]
            [imin(twl4c, *l.offset(y as isize) as libc::c_int) as usize][sidx as usize];
        *fresh4 = (*fresh4 as libc::c_uint | smask) as u16;
        y += 1;
        mask <<= 1;
    }
    x = 0 as libc::c_int;
    mask = (1 as libc::c_uint) << bx4;
    while x < w4 {
        let sidx_0 = (mask >= 0x10000 as libc::c_int as libc::c_uint) as libc::c_int;
        let smask_0: libc::c_uint = mask >> (sidx_0 << 4);
        let ref mut fresh5 = (*masks.offset(1))[by4 as usize]
            [imin(thl4c, *a.offset(x as isize) as libc::c_int) as usize][sidx_0 as usize];
        *fresh5 = (*fresh5 as libc::c_uint | smask_0) as u16;
        x += 1;
        mask <<= 1;
    }
    let hstep = (*t_dim).w as libc::c_int;
    let mut t: libc::c_uint = (1 as libc::c_uint) << by4;
    let mut inner: libc::c_uint = ((t as u64) << h4).wrapping_sub(t as u64) as libc::c_uint;
    let mut inner1: libc::c_uint = inner & 0xffff as libc::c_int as libc::c_uint;
    let mut inner2: libc::c_uint = inner >> 16;
    x = hstep;
    while x < w4 {
        if inner1 != 0 {
            let ref mut fresh6 = (*masks.offset(0))[(bx4 + x) as usize][twl4c as usize][0];
            *fresh6 = (*fresh6 as libc::c_uint | inner1) as u16;
        }
        if inner2 != 0 {
            let ref mut fresh7 = (*masks.offset(0))[(bx4 + x) as usize][twl4c as usize][1];
            *fresh7 = (*fresh7 as libc::c_uint | inner2) as u16;
        }
        x += hstep;
    }
    let vstep = (*t_dim).h as libc::c_int;
    t = (1 as libc::c_uint) << bx4;
    inner = ((t as u64) << w4).wrapping_sub(t as u64) as libc::c_uint;
    inner1 = inner & 0xffff as libc::c_int as libc::c_uint;
    inner2 = inner >> 16;
    y = vstep;
    while y < h4 {
        if inner1 != 0 {
            let ref mut fresh8 = (*masks.offset(1))[(by4 + y) as usize][thl4c as usize][0];
            *fresh8 = (*fresh8 as libc::c_uint | inner1) as u16;
        }
        if inner2 != 0 {
            let ref mut fresh9 = (*masks.offset(1))[(by4 + y) as usize][thl4c as usize][1];
            *fresh9 = (*fresh9 as libc::c_uint | inner2) as u16;
        }
        y += vstep;
    }
    match w4 {
        1 => {
            (*(&mut *a.offset(0) as *mut u8 as *mut alias8)).u8_0 = (0x1 * thl4c) as u8;
        }
        2 => {
            (*(&mut *a.offset(0) as *mut u8 as *mut alias16)).u16_0 = (0x101 * thl4c) as u16;
        }
        4 => {
            (*(&mut *a.offset(0) as *mut u8 as *mut alias32)).u32_0 =
                (0x1010101 as libc::c_uint).wrapping_mul(thl4c as libc::c_uint);
        }
        8 => {
            (*(&mut *a.offset(0) as *mut u8 as *mut alias64)).u64_0 =
                (0x101010101010101 as libc::c_ulonglong).wrapping_mul(thl4c as libc::c_ulonglong)
                    as u64;
        }
        16 => {
            let const_val: u64 = (0x101010101010101 as libc::c_ulonglong)
                .wrapping_mul(thl4c as libc::c_ulonglong) as u64;
            (*(&mut *a.offset((0 + 0) as isize) as *mut u8 as *mut alias64)).u64_0 = const_val;
            (*(&mut *a.offset((0 + 8) as isize) as *mut u8 as *mut alias64)).u64_0 = const_val;
        }
        32 => {
            let const_val_0: u64 = (0x101010101010101 as libc::c_ulonglong)
                .wrapping_mul(thl4c as libc::c_ulonglong) as u64;
            (*(&mut *a.offset((0 + 0) as isize) as *mut u8 as *mut alias64)).u64_0 = const_val_0;
            (*(&mut *a.offset((0 + 8) as isize) as *mut u8 as *mut alias64)).u64_0 = const_val_0;
            (*(&mut *a.offset((0 + 16) as isize) as *mut u8 as *mut alias64)).u64_0 = const_val_0;
            (*(&mut *a.offset((0 + 24) as isize) as *mut u8 as *mut alias64)).u64_0 = const_val_0;
        }
        _ => {
            memset(a as *mut libc::c_void, thl4c, w4 as libc::c_ulong);
        }
    }
    match h4 {
        1 => {
            (*(&mut *l.offset(0) as *mut u8 as *mut alias8)).u8_0 = (0x1 * twl4c) as u8;
        }
        2 => {
            (*(&mut *l.offset(0) as *mut u8 as *mut alias16)).u16_0 = (0x101 * twl4c) as u16;
        }
        4 => {
            (*(&mut *l.offset(0) as *mut u8 as *mut alias32)).u32_0 =
                (0x1010101 as libc::c_uint).wrapping_mul(twl4c as libc::c_uint);
        }
        8 => {
            (*(&mut *l.offset(0) as *mut u8 as *mut alias64)).u64_0 =
                (0x101010101010101 as libc::c_ulonglong).wrapping_mul(twl4c as libc::c_ulonglong)
                    as u64;
        }
        16 => {
            let const_val_1: u64 = (0x101010101010101 as libc::c_ulonglong)
                .wrapping_mul(twl4c as libc::c_ulonglong) as u64;
            (*(&mut *l.offset((0 + 0) as isize) as *mut u8 as *mut alias64)).u64_0 = const_val_1;
            (*(&mut *l.offset((0 + 8) as isize) as *mut u8 as *mut alias64)).u64_0 = const_val_1;
        }
        32 => {
            let const_val_2: u64 = (0x101010101010101 as libc::c_ulonglong)
                .wrapping_mul(twl4c as libc::c_ulonglong) as u64;
            (*(&mut *l.offset((0 + 0) as isize) as *mut u8 as *mut alias64)).u64_0 = const_val_2;
            (*(&mut *l.offset((0 + 8) as isize) as *mut u8 as *mut alias64)).u64_0 = const_val_2;
            (*(&mut *l.offset((0 + 16) as isize) as *mut u8 as *mut alias64)).u64_0 = const_val_2;
            (*(&mut *l.offset((0 + 24) as isize) as *mut u8 as *mut alias64)).u64_0 = const_val_2;
        }
        _ => {
            memset(l as *mut libc::c_void, twl4c, h4 as libc::c_ulong);
        }
    };
}

unsafe fn mask_edges_chroma(
    masks: *mut [[[u16; 2]; 2]; 32],
    cby4: libc::c_int,
    cbx4: libc::c_int,
    cw4: libc::c_int,
    ch4: libc::c_int,
    skip_inter: libc::c_int,
    tx: RectTxfmSize,
    a: *mut u8,
    l: *mut u8,
    ss_hor: libc::c_int,
    ss_ver: libc::c_int,
) {
    let t_dim: *const TxfmInfo =
        &*dav1d_txfm_dimensions.as_ptr().offset(tx as isize) as *const TxfmInfo;
    let twl4 = (*t_dim).lw as libc::c_int;
    let thl4 = (*t_dim).lh as libc::c_int;
    let twl4c = (twl4 != 0) as libc::c_int;
    let thl4c = (thl4 != 0) as libc::c_int;
    let mut y = 0;
    let mut x = 0;
    let vbits = 4 - ss_ver;
    let hbits = 4 - ss_hor;
    let vmask = 16 >> ss_ver;
    let hmask = 16 >> ss_hor;
    let vmax: libc::c_uint = ((1 as libc::c_int) << vmask) as libc::c_uint;
    let hmax: libc::c_uint = ((1 as libc::c_int) << hmask) as libc::c_uint;
    let mut mask: libc::c_uint = (1 as libc::c_uint) << cby4;
    y = 0 as libc::c_int;
    while y < ch4 {
        let sidx = (mask >= vmax) as libc::c_int;
        let smask: libc::c_uint = mask >> (sidx << vbits);
        let ref mut fresh10 = (*masks.offset(0))[cbx4 as usize]
            [imin(twl4c, *l.offset(y as isize) as libc::c_int) as usize][sidx as usize];
        *fresh10 = (*fresh10 as libc::c_uint | smask) as u16;
        y += 1;
        mask <<= 1;
    }
    x = 0 as libc::c_int;
    mask = (1 as libc::c_uint) << cbx4;
    while x < cw4 {
        let sidx_0 = (mask >= hmax) as libc::c_int;
        let smask_0: libc::c_uint = mask >> (sidx_0 << hbits);
        let ref mut fresh11 = (*masks.offset(1))[cby4 as usize]
            [imin(thl4c, *a.offset(x as isize) as libc::c_int) as usize][sidx_0 as usize];
        *fresh11 = (*fresh11 as libc::c_uint | smask_0) as u16;
        x += 1;
        mask <<= 1;
    }
    if skip_inter == 0 {
        let hstep = (*t_dim).w as libc::c_int;
        let mut t: libc::c_uint = (1 as libc::c_uint) << cby4;
        let mut inner: libc::c_uint = ((t as u64) << ch4).wrapping_sub(t as u64) as libc::c_uint;
        let mut inner1: libc::c_uint = inner & (((1 as libc::c_int) << vmask) - 1) as libc::c_uint;
        let mut inner2: libc::c_uint = inner >> vmask;
        x = hstep;
        while x < cw4 {
            if inner1 != 0 {
                let ref mut fresh12 = (*masks.offset(0))[(cbx4 + x) as usize][twl4c as usize][0];
                *fresh12 = (*fresh12 as libc::c_uint | inner1) as u16;
            }
            if inner2 != 0 {
                let ref mut fresh13 = (*masks.offset(0))[(cbx4 + x) as usize][twl4c as usize][1];
                *fresh13 = (*fresh13 as libc::c_uint | inner2) as u16;
            }
            x += hstep;
        }
        let vstep = (*t_dim).h as libc::c_int;
        t = (1 as libc::c_uint) << cbx4;
        inner = ((t as u64) << cw4).wrapping_sub(t as u64) as libc::c_uint;
        inner1 = inner & (((1 as libc::c_int) << hmask) - 1) as libc::c_uint;
        inner2 = inner >> hmask;
        y = vstep;
        while y < ch4 {
            if inner1 != 0 {
                let ref mut fresh14 = (*masks.offset(1))[(cby4 + y) as usize][thl4c as usize][0];
                *fresh14 = (*fresh14 as libc::c_uint | inner1) as u16;
            }
            if inner2 != 0 {
                let ref mut fresh15 = (*masks.offset(1))[(cby4 + y) as usize][thl4c as usize][1];
                *fresh15 = (*fresh15 as libc::c_uint | inner2) as u16;
            }
            y += vstep;
        }
    }
    match cw4 {
        1 => {
            (*(&mut *a.offset(0) as *mut u8 as *mut alias8)).u8_0 = (0x1 * thl4c) as u8;
        }
        2 => {
            (*(&mut *a.offset(0) as *mut u8 as *mut alias16)).u16_0 = (0x101 * thl4c) as u16;
        }
        4 => {
            (*(&mut *a.offset(0) as *mut u8 as *mut alias32)).u32_0 =
                (0x1010101 as libc::c_uint).wrapping_mul(thl4c as libc::c_uint);
        }
        8 => {
            (*(&mut *a.offset(0) as *mut u8 as *mut alias64)).u64_0 =
                (0x101010101010101 as libc::c_ulonglong).wrapping_mul(thl4c as libc::c_ulonglong)
                    as u64;
        }
        16 => {
            let const_val: u64 = (0x101010101010101 as libc::c_ulonglong)
                .wrapping_mul(thl4c as libc::c_ulonglong) as u64;
            (*(&mut *a.offset((0 + 0) as isize) as *mut u8 as *mut alias64)).u64_0 = const_val;
            (*(&mut *a.offset((0 + 8) as isize) as *mut u8 as *mut alias64)).u64_0 = const_val;
        }
        32 => {
            let const_val_0: u64 = (0x101010101010101 as libc::c_ulonglong)
                .wrapping_mul(thl4c as libc::c_ulonglong) as u64;
            (*(&mut *a.offset((0 + 0) as isize) as *mut u8 as *mut alias64)).u64_0 = const_val_0;
            (*(&mut *a.offset((0 + 8) as isize) as *mut u8 as *mut alias64)).u64_0 = const_val_0;
            (*(&mut *a.offset((0 + 16) as isize) as *mut u8 as *mut alias64)).u64_0 = const_val_0;
            (*(&mut *a.offset((0 + 24) as isize) as *mut u8 as *mut alias64)).u64_0 = const_val_0;
        }
        _ => {
            memset(a as *mut libc::c_void, thl4c, cw4 as libc::c_ulong);
        }
    }
    match ch4 {
        1 => {
            (*(&mut *l.offset(0) as *mut u8 as *mut alias8)).u8_0 = (0x1 * twl4c) as u8;
        }
        2 => {
            (*(&mut *l.offset(0) as *mut u8 as *mut alias16)).u16_0 = (0x101 * twl4c) as u16;
        }
        4 => {
            (*(&mut *l.offset(0) as *mut u8 as *mut alias32)).u32_0 =
                (0x1010101 as libc::c_uint).wrapping_mul(twl4c as libc::c_uint);
        }
        8 => {
            (*(&mut *l.offset(0) as *mut u8 as *mut alias64)).u64_0 =
                (0x101010101010101 as libc::c_ulonglong).wrapping_mul(twl4c as libc::c_ulonglong)
                    as u64;
        }
        16 => {
            let const_val_1: u64 = (0x101010101010101 as libc::c_ulonglong)
                .wrapping_mul(twl4c as libc::c_ulonglong) as u64;
            (*(&mut *l.offset((0 + 0) as isize) as *mut u8 as *mut alias64)).u64_0 = const_val_1;
            (*(&mut *l.offset((0 + 8) as isize) as *mut u8 as *mut alias64)).u64_0 = const_val_1;
        }
        32 => {
            let const_val_2: u64 = (0x101010101010101 as libc::c_ulonglong)
                .wrapping_mul(twl4c as libc::c_ulonglong) as u64;
            (*(&mut *l.offset((0 + 0) as isize) as *mut u8 as *mut alias64)).u64_0 = const_val_2;
            (*(&mut *l.offset((0 + 8) as isize) as *mut u8 as *mut alias64)).u64_0 = const_val_2;
            (*(&mut *l.offset((0 + 16) as isize) as *mut u8 as *mut alias64)).u64_0 = const_val_2;
            (*(&mut *l.offset((0 + 24) as isize) as *mut u8 as *mut alias64)).u64_0 = const_val_2;
        }
        _ => {
            memset(l as *mut libc::c_void, twl4c, ch4 as libc::c_ulong);
        }
    };
}

pub unsafe fn dav1d_create_lf_mask_intra(
    lflvl: *mut Av1Filter,
    level_cache: *mut [u8; 4],
    b4_stride: ptrdiff_t,
    mut filter_level: *const [[u8; 2]; 8],
    bx: libc::c_int,
    by: libc::c_int,
    iw: libc::c_int,
    ih: libc::c_int,
    bs: BlockSize,
    ytx: RectTxfmSize,
    uvtx: RectTxfmSize,
    layout: Dav1dPixelLayout,
    ay: *mut u8,
    ly: *mut u8,
    auv: *mut u8,
    luv: *mut u8,
) {
    let b_dim: *const u8 = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bw4 = imin(iw - bx, *b_dim.offset(0) as libc::c_int);
    let bh4 = imin(ih - by, *b_dim.offset(1) as libc::c_int);
    let bx4 = bx & 31;
    let by4 = by & 31;
    if bw4 != 0 && bh4 != 0 {
        let mut level_cache_ptr: *mut [u8; 4] = level_cache
            .offset(by as isize * b4_stride)
            .offset(bx as isize);
        let mut y = 0;
        while y < bh4 {
            let mut x = 0;
            while x < bw4 {
                (*level_cache_ptr.offset(x as isize))[0] = (*filter_level.offset(0))[0][0];
                (*level_cache_ptr.offset(x as isize))[1] = (*filter_level.offset(1))[0][0];
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
    let ss_ver = (layout as libc::c_uint == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint)
        as libc::c_int;
    let ss_hor = (layout as libc::c_uint != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint)
        as libc::c_int;
    let cbw4 = imin(
        (iw + ss_hor >> ss_hor) - (bx >> ss_hor),
        *b_dim.offset(0) as libc::c_int + ss_hor >> ss_hor,
    );
    let cbh4 = imin(
        (ih + ss_ver >> ss_ver) - (by >> ss_ver),
        *b_dim.offset(1) as libc::c_int + ss_ver >> ss_ver,
    );
    if cbw4 == 0 || cbh4 == 0 {
        return;
    }
    let cbx4 = bx4 >> ss_hor;
    let cby4 = by4 >> ss_ver;
    let mut level_cache_ptr_0: *mut [u8; 4] = level_cache
        .offset(((by >> ss_ver) as isize * b4_stride) as isize)
        .offset((bx >> ss_hor) as isize);
    let mut y_0 = 0;
    while y_0 < cbh4 {
        let mut x_0 = 0;
        while x_0 < cbw4 {
            (*level_cache_ptr_0.offset(x_0 as isize))[2] = (*filter_level.offset(2))[0][0];
            (*level_cache_ptr_0.offset(x_0 as isize))[3] = (*filter_level.offset(3))[0][0];
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

pub unsafe fn dav1d_create_lf_mask_inter(
    lflvl: *mut Av1Filter,
    level_cache: *mut [u8; 4],
    b4_stride: ptrdiff_t,
    mut filter_level: *const [[u8; 2]; 8],
    bx: libc::c_int,
    by: libc::c_int,
    iw: libc::c_int,
    ih: libc::c_int,
    skip: libc::c_int,
    bs: BlockSize,
    max_ytx: RectTxfmSize,
    tx_masks: *const u16,
    uvtx: RectTxfmSize,
    layout: Dav1dPixelLayout,
    ay: *mut u8,
    ly: *mut u8,
    auv: *mut u8,
    luv: *mut u8,
) {
    let b_dim: *const u8 = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bw4 = imin(iw - bx, *b_dim.offset(0) as libc::c_int);
    let bh4 = imin(ih - by, *b_dim.offset(1) as libc::c_int);
    let bx4 = bx & 31;
    let by4 = by & 31;
    if bw4 != 0 && bh4 != 0 {
        let mut level_cache_ptr: *mut [u8; 4] = level_cache
            .offset(by as isize * b4_stride)
            .offset(bx as isize);
        let mut y = 0;
        while y < bh4 {
            let mut x = 0;
            while x < bw4 {
                (*level_cache_ptr.offset(x as isize))[0] = (*filter_level.offset(0))[0][0];
                (*level_cache_ptr.offset(x as isize))[1] = (*filter_level.offset(1))[0][0];
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
    let ss_ver = (layout as libc::c_uint == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint)
        as libc::c_int;
    let ss_hor = (layout as libc::c_uint != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint)
        as libc::c_int;
    let cbw4 = imin(
        (iw + ss_hor >> ss_hor) - (bx >> ss_hor),
        *b_dim.offset(0) as libc::c_int + ss_hor >> ss_hor,
    );
    let cbh4 = imin(
        (ih + ss_ver >> ss_ver) - (by >> ss_ver),
        *b_dim.offset(1) as libc::c_int + ss_ver >> ss_ver,
    );
    if cbw4 == 0 || cbh4 == 0 {
        return;
    }
    let cbx4 = bx4 >> ss_hor;
    let cby4 = by4 >> ss_ver;
    let mut level_cache_ptr_0: *mut [u8; 4] = level_cache
        .offset(((by >> ss_ver) as isize * b4_stride) as isize)
        .offset((bx >> ss_hor) as isize);
    let mut y_0 = 0;
    while y_0 < cbh4 {
        let mut x_0 = 0;
        while x_0 < cbw4 {
            (*level_cache_ptr_0.offset(x_0 as isize))[2] = (*filter_level.offset(2))[0][0];
            (*level_cache_ptr_0.offset(x_0 as isize))[3] = (*filter_level.offset(3))[0][0];
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

pub fn dav1d_calc_eih(lim_lut: &mut Av1FilterLUT, filter_sharpness: libc::c_int) {
    // set E/I/H values from loopfilter level
    let sharp = filter_sharpness as u8;
    for level in 0..64 {
        let mut limit = level;

        if sharp > 0 {
            limit >>= sharp + 3 >> 2;
            limit = std::cmp::min(limit, 9 - sharp);
        }
        limit = std::cmp::max(limit, 1);

        lim_lut.i[level as usize] = limit;
        lim_lut.e[level as usize] = 2 * (level + 2) + limit;
    }
    let sharp = sharp as u64;
    lim_lut.sharp[0] = sharp + 3 >> 2;
    lim_lut.sharp[1] = if sharp != 0 { 9 - sharp } else { 0xff };
}

fn calc_lf_value(
    lflvl_values: &mut [[u8; 2]; 8],
    base_lvl: libc::c_int,
    lf_delta: i8,
    seg_delta: libc::c_int,
    mr_delta: Option<&Dav1dLoopfilterModeRefDeltas>,
) {
    let base = iclip(
        iclip(base_lvl + lf_delta as libc::c_int, 0, 63) + seg_delta,
        0,
        63,
    );

    if let Some(mr_delta) = mr_delta {
        let sh = (base >= 32) as libc::c_int;
        lflvl_values[0] = [iclip(base + mr_delta.ref_delta[0] * (1 << sh), 0, 63) as u8; 2];
        for r in 1..8 {
            for m in 0..2 {
                let delta = mr_delta.mode_delta[m] + mr_delta.ref_delta[r];
                lflvl_values[r][m] = iclip(base + delta * (1 << sh), 0, 63) as u8;
            }
        }
    } else {
        *lflvl_values = [[base as u8; 2]; 8];
    }
}

#[inline]
fn calc_lf_value_chroma(
    lflvl_values: &mut [[u8; 2]; 8],
    base_lvl: libc::c_int,
    lf_delta: i8,
    seg_delta: libc::c_int,
    mr_delta: Option<&Dav1dLoopfilterModeRefDeltas>,
) {
    if base_lvl == 0 {
        *lflvl_values = Default::default();
    } else {
        calc_lf_value(lflvl_values, base_lvl, lf_delta, seg_delta, mr_delta);
    };
}

pub fn dav1d_calc_lf_values(
    lflvl_values: &mut [[[[u8; 2]; 8]; 4]; 8],
    hdr: &Dav1dFrameHeader,
    mut lf_delta: &[i8; 4],
) {
    let n_seg = if hdr.segmentation.enabled != 0 { 8 } else { 1 };

    if hdr.loopfilter.level_y[0] == 0 && hdr.loopfilter.level_y[1] == 0 {
        lflvl_values[..n_seg].fill_with(Default::default);
        return;
    }

    let mr_deltas = if hdr.loopfilter.mode_ref_delta_enabled != 0 {
        Some(&hdr.loopfilter.mode_ref_deltas)
    } else {
        None
    };
    for s in 0..n_seg {
        let segd = if hdr.segmentation.enabled != 0 {
            Some(&hdr.segmentation.seg_data.d[s])
        } else {
            None
        };

        calc_lf_value(
            &mut lflvl_values[s][0],
            hdr.loopfilter.level_y[0],
            lf_delta[0],
            segd.map(|segd| segd.delta_lf_y_v).unwrap_or(0),
            mr_deltas,
        );
        calc_lf_value(
            &mut lflvl_values[s][1],
            hdr.loopfilter.level_y[1],
            lf_delta[if hdr.delta.lf.multi != 0 { 1 } else { 0 }],
            segd.map(|segd| segd.delta_lf_y_h).unwrap_or(0),
            mr_deltas,
        );
        calc_lf_value_chroma(
            &mut lflvl_values[s][2],
            hdr.loopfilter.level_u,
            lf_delta[if hdr.delta.lf.multi != 0 { 2 } else { 0 }],
            segd.map(|segd| segd.delta_lf_u).unwrap_or(0),
            mr_deltas,
        );
        calc_lf_value_chroma(
            &mut lflvl_values[s][3],
            hdr.loopfilter.level_v,
            lf_delta[if hdr.delta.lf.multi != 0 { 3 } else { 0 }],
            segd.map(|segd| segd.delta_lf_v).unwrap_or(0),
            mr_deltas,
        );
    }
}
