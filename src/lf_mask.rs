use crate::include::common::intops::iclip;
use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::dav1d::headers::Dav1dLoopfilterModeRefDeltas;
use crate::include::dav1d::headers::Dav1dPixelLayout;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;
use crate::include::stddef::ptrdiff_t;
use crate::src::align::Align16;
use crate::src::ctx::case_set_upto16;
use crate::src::ctx::case_set_upto32_with_default;
use crate::src::ctx::SetCtxFn;
use crate::src::levels::BlockSize;
use crate::src::levels::RectTxfmSize;
use crate::src::levels::TX_4X4;
use crate::src::tables::dav1d_block_dimensions;
use crate::src::tables::dav1d_txfm_dimensions;

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

/// In `txa`, the array lengths represent from inner to outer:
/// * `32`: `x`
/// * `32`: `y`
/// * `2`: `txsz`, `step`
/// * `2`: `edge`
///
/// (Note: This is added here in the docs vs. inline `/* */` comments
/// at the array lengths because `rustfmt` deletes them
/// (tracked in [rust-lang/rustfmt#5297](https://github.com/rust-lang/rustfmt/issues/5297))).
///
/// The usage of `txa` here has been changed from the C version
/// as the C version was UB in Rust.
/// The C version offsetted `txa` in each recursive call
/// to an element of the inner 32x32 dimensional array,
/// but then casting that back to a pointer to the full 32x32x2x2 array,
/// even though the pointer no longer pointed to a complete 32x32x2x2 array.
/// This was (likely) okay in C given those now out-of-bounds elements were never accessed,
/// but in Rust, dereferencing such a pointer would be an out-of-bounds access, and thus UB.
/// Instead of offsetting `txa`, the offsets are calculated from
/// the existing `y_off` and `x_off` args and applied at each use site of `txa.
unsafe fn decomp_tx(
    txa: &mut [[[[u8; 32]; 32]; 2]; 2],
    from: RectTxfmSize,
    depth: usize,
    y_off: u8,
    x_off: u8,
    tx_masks: &[u16; 2],
) {
    debug_assert!(depth <= 2);
    let t_dim = &dav1d_txfm_dimensions[from as usize];

    let y0 = (y_off * t_dim.h) as usize;
    let x0 = (x_off * t_dim.w) as usize;

    let is_split = if from == TX_4X4 || depth > 1 {
        false
    } else {
        (tx_masks[depth] >> (y_off * 4 + x_off)) & 1 != 0
    };
    if is_split {
        let sub = t_dim.sub as RectTxfmSize;

        decomp_tx(txa, sub, depth + 1, y_off * 2 + 0, x_off * 2 + 0, tx_masks);
        if t_dim.w >= t_dim.h {
            decomp_tx(txa, sub, depth + 1, y_off * 2 + 0, x_off * 2 + 1, tx_masks);
        }
        if t_dim.h >= t_dim.w {
            decomp_tx(txa, sub, depth + 1, y_off * 2 + 1, x_off * 2 + 0, tx_masks);
            if t_dim.w >= t_dim.h {
                decomp_tx(txa, sub, depth + 1, y_off * 2 + 1, x_off * 2 + 1, tx_masks);
            }
        }
    } else {
        let lw = std::cmp::min(2, t_dim.lw);
        let lh = std::cmp::min(2, t_dim.lh);

        let mut set_ctx = |_dir: &mut (), _diridx, off, mul, rep_macro: SetCtxFn| {
            for y in 0..t_dim.h as usize {
                rep_macro(txa[0][0][y0 + y][x0..].as_mut_ptr(), off, mul * lw as u64);
                rep_macro(txa[1][0][y0 + y][x0..].as_mut_ptr(), off, mul * lh as u64);
                txa[0][1][y0 + y][x0] = t_dim.w;
            }
        };
        case_set_upto16(
            t_dim.w as libc::c_int,
            &mut (),            // Was nothing in C.
            Default::default(), // Was nothing in C.
            0,
            &mut set_ctx,
        );
        let mut set_ctx = |_dir: &mut (), _diridx, off, mul, rep_macro: SetCtxFn| {
            rep_macro(txa[1][1][y0][x0..].as_mut_ptr(), off, mul * t_dim.h as u64);
        };
        case_set_upto16(
            t_dim.w as libc::c_int,
            &mut (),            // Was nothing in C.
            Default::default(), // Was nothing in C.
            0,
            &mut set_ctx,
        );
    };
}

#[inline]
unsafe fn mask_edges_inter(
    masks: &mut [[[[u16; 2]; 3]; 32]; 2],
    by4: usize,
    bx4: usize,
    w4: usize,
    h4: usize,
    skip: bool,
    max_tx: RectTxfmSize,
    tx_masks: &[u16; 2],
    a: &mut [u8],
    l: &mut [u8],
) {
    let t_dim = &dav1d_txfm_dimensions[max_tx as usize];

    // See [`decomp_tx`]'s docs for the `txa` arg.
    let mut txa = Align16([[[[0; 32]; 32]; 2]; 2]);

    for (y_off, _) in (0..h4).step_by(t_dim.h as usize).enumerate() {
        for (x_off, _) in (0..w4).step_by(t_dim.w as usize).enumerate() {
            decomp_tx(&mut txa.0, max_tx, 0, y_off as u8, x_off as u8, tx_masks);
        }
    }

    // left block edge
    for y in 0..h4 {
        let mask = 1u32 << (by4 + y);
        let sidx = (mask >= 0x10000) as usize;
        let smask = mask >> (sidx << 4);
        masks[0][bx4][std::cmp::min(txa[0][0][y][0], l[y]) as usize][sidx] |= smask as u16;
    }

    // top block edge
    for x in 0..w4 {
        let mask = 1u32 << (bx4 + x);
        let sidx = (mask >= 0x10000) as usize;
        let smask = mask >> (sidx << 4);
        masks[1][by4][std::cmp::min(txa[1][0][0][x], a[x]) as usize][sidx] |= smask as u16;
    }
    if !skip {
        // inner (tx) left|right edges
        for y in 0..h4 {
            let mask = 1u32 << (by4 + y);
            let sidx = (mask >= 0x10000) as usize;
            let smask = mask >> (sidx << 4);
            let mut ltx = txa[0][0][y][0];
            let step = txa[0][1][y][0] as usize;
            let mut x = step;
            while x < w4 {
                let rtx = txa[0][0][y][x];
                masks[0][bx4 + x][std::cmp::min(rtx, ltx) as usize][sidx] |= smask as u16;
                ltx = rtx;
                let step = txa[0][1][y][x] as usize;
                x += step;
            }
        }

        //            top
        // inner (tx) --- edges
        //           bottom
        for x in 0..w4 {
            let mask = 1u32 << (bx4 + x);
            let sidx = (mask >= 0x10000) as usize;
            let smask = mask >> (sidx << 4);
            let mut ttx = txa[1][0][0][x];
            let step = txa[1][1][0][x] as usize;
            let mut y = step;
            while y < h4 {
                let btx = txa[1][0][y][x];
                masks[1][by4 + y][std::cmp::min(ttx, btx) as usize][sidx] |= smask as u16;
                ttx = btx;
                let step = txa[1][1][y][x] as usize;
                y += step;
            }
        }
    }

    for (l, txa) in l[..h4].iter_mut().zip(&txa[0][0][..h4]) {
        *l = txa[w4 - 1];
    }
    a[..w4].copy_from_slice(&mut txa[1][0][h4 - 1][..w4]);
}

#[inline]
unsafe fn mask_edges_intra(
    masks: &mut [[[[u16; 2]; 3]; 32]; 2],
    by4: usize,
    bx4: usize,
    w4: usize,
    h4: usize,
    tx: RectTxfmSize,
    a: &mut [u8],
    l: &mut [u8],
) {
    let t_dim = &dav1d_txfm_dimensions[tx as usize];
    let twl4 = t_dim.lw;
    let thl4 = t_dim.lh;
    let twl4c = std::cmp::min(2, twl4);
    let thl4c = std::cmp::min(2, thl4);

    // left block edge
    for y in 0..h4 {
        let mask = 1u32 << (by4 + y);
        let sidx = (mask >= 0x10000) as usize;
        let smask = mask >> (sidx << 4);
        masks[0][bx4][std::cmp::min(twl4c, l[y]) as usize][sidx] |= smask as u16;
    }

    // top block edge
    for x in 0..w4 {
        let mask = 1u32 << (bx4 + x);
        let sidx = (mask >= 0x10000) as usize;
        let smask = mask >> (sidx << 4);
        masks[1][by4][std::cmp::min(thl4c, a[x]) as usize][sidx] |= smask as u16;
    }

    // inner (tx) left|right edges
    let hstep = t_dim.w as usize;
    let t = 1u32 << by4;
    let inner = (((t as u64) << h4) - (t as u64)) as u32;
    let inner = [inner as u16, (inner >> 16) as u16];
    for x in (hstep..w4).step_by(hstep) {
        if inner[0] != 0 {
            masks[0][bx4 + x][twl4c as usize][0] |= inner[0];
        }
        if inner[1] != 0 {
            masks[0][bx4 + x][twl4c as usize][1] |= inner[1];
        }
    }

    //            top
    // inner (tx) --- edges
    //           bottom
    let vstep = t_dim.h as usize;
    let t = 1u32 << bx4;
    let inner = (((t as u64) << w4) - (t as u64)) as u32;
    let inner = [inner as u16, (inner >> 16) as u16];
    for y in (vstep..h4).step_by(vstep) {
        if inner[0] != 0 {
            masks[1][by4 + y][thl4c as usize][0] |= inner[0];
        }
        if inner[1] != 0 {
            masks[1][by4 + y][thl4c as usize][1] |= inner[1];
        }
    }

    let mut set_ctx = |dir: &mut [u8], _diridx, off, mul, rep_macro: SetCtxFn| {
        rep_macro(dir.as_mut_ptr(), off, mul * thl4c as u64);
    };
    let default_memset = |dir: &mut [u8], _diridx, _off, var| {
        dir[..var as usize].fill(thl4c);
    };
    case_set_upto32_with_default(
        w4 as libc::c_int,
        a,                  // Was nothing in C; changed to `l` for borrowck.
        Default::default(), // Was nothing in C.
        0,
        &mut set_ctx,
        default_memset,
    );
    let mut set_ctx = |dir: &mut [u8], _diridx, off, mul, rep_macro: SetCtxFn| {
        rep_macro(dir.as_mut_ptr(), off, mul * twl4c as u64);
    };
    let default_memset = |dir: &mut [u8], _diridx, _off, var| {
        dir[..var as usize].fill(twl4c);
    };
    case_set_upto32_with_default(
        h4 as libc::c_int,
        l,                  // Was nothing in C; changed to `l` for borrowck.
        Default::default(), // Was nothing in C.
        0,
        &mut set_ctx,
        default_memset,
    );
}

unsafe fn mask_edges_chroma(
    masks: &mut [[[[u16; 2]; 2]; 32]; 2],
    cby4: usize,
    cbx4: usize,
    cw4: usize,
    ch4: usize,
    skip_inter: bool,
    tx: RectTxfmSize,
    a: &mut [u8],
    l: &mut [u8],
    ss_hor: usize,
    ss_ver: usize,
) {
    let t_dim = &dav1d_txfm_dimensions[tx as usize];
    let twl4 = t_dim.lw;
    let thl4 = t_dim.lh;
    let twl4c = (twl4 != 0) as u8;
    let thl4c = (thl4 != 0) as u8;
    let vbits = 4 - ss_ver;
    let hbits = 4 - ss_hor;
    let vmask = 16 >> ss_ver;
    let hmask = 16 >> ss_hor;
    let vmax = 1u32 << vmask;
    let hmax = 1u32 << hmask;

    // left block edge
    for y in 0..ch4 {
        let mask = 1u32 << (cby4 + y);
        let sidx = (mask >= vmax) as usize;
        let smask = mask >> (sidx << vbits);
        masks[0][cbx4][std::cmp::min(twl4c, l[y]) as usize][sidx] |= smask as u16;
    }

    // top block edge
    for x in 0..cw4 {
        let mask = 1u32 << (cbx4 + x);
        let sidx = (mask >= hmax) as usize;
        let smask = mask >> (sidx << hbits);
        masks[1][cby4][std::cmp::min(thl4c, a[x]) as usize][sidx] |= smask as u16;
    }

    if !skip_inter {
        // inner (tx) left|right edges
        let hstep = t_dim.w as usize;
        let t = 1u32 << cby4;
        let inner = (((t as u64) << ch4) - (t as u64)) as u32;
        let inner = [(inner & ((1 << vmask) - 1)) as u16, (inner >> vmask) as u16];
        for x in (hstep..cw4).step_by(hstep) {
            if inner[0] != 0 {
                masks[0][cbx4 + x][twl4c as usize][0] |= inner[0];
            }
            if inner[1] != 0 {
                masks[0][cbx4 + x][twl4c as usize][1] |= inner[1];
            }
        }

        //            top
        // inner (tx) --- edges
        //           bottom
        let vstep = t_dim.h as usize;
        let t = 1u32 << cbx4;
        let inner = (((t as u64) << cw4) - (t as u64)) as u32;
        let inner = [(inner & ((1 << hmask) - 1)) as u16, (inner >> hmask) as u16];
        for y in (vstep..ch4).step_by(vstep) {
            if inner[0] != 0 {
                masks[1][cby4 + y][thl4c as usize][0] |= inner[0];
            }
            if inner[1] != 0 {
                masks[1][cby4 + y][thl4c as usize][1] |= inner[1];
            }
        }
    }

    let mut set_ctx = |dir: &mut [u8], _diridx, off, mul, rep_macro: SetCtxFn| {
        rep_macro(dir.as_mut_ptr(), off, mul * thl4c as u64);
    };
    let default_memset = |dir: &mut [u8], _diridx, _off, var| {
        dir[..var as usize].fill(thl4c);
    };
    case_set_upto32_with_default(
        cw4 as libc::c_int,
        a,                  // Was nothing in C; changed to `l` for borrowck.
        Default::default(), // Was nothing in C.
        0,
        &mut set_ctx,
        default_memset,
    );
    let mut set_ctx = |dir: &mut [u8], _diridx, off, mul, rep_macro: SetCtxFn| {
        rep_macro(dir.as_mut_ptr(), off, mul * twl4c as u64);
    };
    let default_memset = |dir: &mut [u8], _diridx, _off, var| {
        dir[..var as usize].fill(twl4c);
    };
    case_set_upto32_with_default(
        ch4 as libc::c_int,
        l,                  // Was nothing in C; changed to `l` for borrowck.
        Default::default(), // Was nothing in C.
        0,
        &mut set_ctx,
        default_memset,
    );
}

pub unsafe fn dav1d_create_lf_mask_intra(
    lflvl: &mut Av1Filter,
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
    ay: &mut [u8],
    ly: &mut [u8],
    aluv: Option<(&mut [u8], &mut [u8])>,
) {
    let b4_stride = b4_stride as usize;
    let [bx, by, iw, ih] = [bx, by, iw, ih].map(|it| it as usize);

    let b_dim = &dav1d_block_dimensions[bs as usize];
    let b_dim = b_dim.map(|it| it as usize);
    let bw4 = std::cmp::min(iw - bx, b_dim[0]);
    let bh4 = std::cmp::min(ih - by, b_dim[1]);
    let bx4 = bx & 31;
    let by4 = by & 31;

    if bw4 != 0 && bh4 != 0 {
        let mut level_cache_ptr = level_cache.offset((by * b4_stride + bx) as isize);
        for _ in 0..bh4 {
            for x in 0..bw4 {
                (*level_cache_ptr.offset(x as isize))[0] = (*filter_level.offset(0))[0][0];
                (*level_cache_ptr.offset(x as isize))[1] = (*filter_level.offset(1))[0][0];
            }
            level_cache_ptr = level_cache_ptr.offset(b4_stride as isize);
        }

        mask_edges_intra(&mut lflvl.filter_y, by4, bx4, bw4, bh4, ytx, ay, ly);
    }

    let (auv, luv) = match aluv {
        None => return,
        Some(aluv) => aluv,
    };

    let ss_ver = (layout == DAV1D_PIXEL_LAYOUT_I420) as usize;
    let ss_hor = (layout != DAV1D_PIXEL_LAYOUT_I444) as usize;
    let cbw4 = std::cmp::min(
        (iw + ss_hor >> ss_hor) - (bx >> ss_hor),
        (b_dim[0] + ss_hor) >> ss_hor,
    );
    let cbh4 = std::cmp::min(
        (ih + ss_ver >> ss_ver) - (by >> ss_ver),
        (b_dim[1] + ss_ver) >> ss_ver,
    );

    if cbw4 == 0 || cbh4 == 0 {
        return;
    }

    let cbx4 = bx4 >> ss_hor;
    let cby4 = by4 >> ss_ver;

    let mut level_cache_ptr =
        level_cache.offset(((by >> ss_ver) * b4_stride + (bx >> ss_hor)) as isize);
    for _ in 0..cbh4 {
        for x in 0..cbw4 {
            (*level_cache_ptr.offset(x as isize))[2] = (*filter_level.offset(2))[0][0];
            (*level_cache_ptr.offset(x as isize))[3] = (*filter_level.offset(3))[0][0];
        }
        level_cache_ptr = level_cache_ptr.offset(b4_stride as isize);
    }

    mask_edges_chroma(
        &mut lflvl.filter_uv,
        cby4,
        cbx4,
        cbw4,
        cbh4,
        false,
        uvtx,
        auv,
        luv,
        ss_hor,
        ss_ver,
    );
}

pub unsafe fn dav1d_create_lf_mask_inter(
    lflvl: &mut Av1Filter,
    level_cache: *mut [u8; 4],
    b4_stride: ptrdiff_t,
    mut filter_level: *const [[u8; 2]; 8],
    bx: libc::c_int,
    by: libc::c_int,
    iw: libc::c_int,
    ih: libc::c_int,
    skip: bool,
    bs: BlockSize,
    max_ytx: RectTxfmSize,
    tx_masks: &[u16; 2],
    uvtx: RectTxfmSize,
    layout: Dav1dPixelLayout,
    ay: &mut [u8],
    ly: &mut [u8],
    aluv: Option<(&mut [u8], &mut [u8])>,
) {
    let b4_stride = b4_stride as usize;
    let [bx, by, iw, ih] = [bx, by, iw, ih].map(|it| it as usize);

    let b_dim = &dav1d_block_dimensions[bs as usize];
    let b_dim = b_dim.map(|it| it as usize);
    let bw4 = std::cmp::min(iw - bx, b_dim[0]);
    let bh4 = std::cmp::min(ih - by, b_dim[1]);
    let bx4 = bx & 31;
    let by4 = by & 31;

    if bw4 != 0 && bh4 != 0 {
        let mut level_cache_ptr = level_cache.offset((by * b4_stride + bx) as isize);
        for _ in 0..bh4 {
            for x in 0..bw4 {
                (*level_cache_ptr.offset(x as isize))[0] = (*filter_level.offset(0))[0][0];
                (*level_cache_ptr.offset(x as isize))[1] = (*filter_level.offset(1))[0][0];
            }
            level_cache_ptr = level_cache_ptr.offset(b4_stride as isize);
        }

        mask_edges_inter(
            &mut lflvl.filter_y,
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

    let (auv, luv) = match aluv {
        None => return,
        Some(aluv) => aluv,
    };

    let ss_ver = (layout == DAV1D_PIXEL_LAYOUT_I420) as usize;
    let ss_hor = (layout != DAV1D_PIXEL_LAYOUT_I444) as usize;
    let cbw4 = std::cmp::min(
        (iw + ss_hor >> ss_hor) - (bx >> ss_hor),
        (b_dim[0] + ss_hor) >> ss_hor,
    );
    let cbh4 = std::cmp::min(
        (ih + ss_ver >> ss_ver) - (by >> ss_ver),
        (b_dim[1] + ss_ver) >> ss_ver,
    );

    if cbw4 == 0 || cbh4 == 0 {
        return;
    }

    let cbx4 = bx4 >> ss_hor;
    let cby4 = by4 >> ss_ver;

    let mut level_cache_ptr =
        level_cache.offset(((by >> ss_ver) * b4_stride + (bx >> ss_hor)) as isize);
    for _ in 0..cbh4 {
        for x in 0..cbw4 {
            (*level_cache_ptr.offset(x as isize))[2] = (*filter_level.offset(2))[0][0];
            (*level_cache_ptr.offset(x as isize))[3] = (*filter_level.offset(3))[0][0];
        }
        level_cache_ptr = level_cache_ptr.offset(b4_stride as isize);
    }

    mask_edges_chroma(
        &mut lflvl.filter_uv,
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
