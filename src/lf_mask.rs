use crate::include::common::intops::clip;
use crate::include::common::intops::iclip;
use crate::include::dav1d::headers::Rav1dFrameHeader;
use crate::include::dav1d::headers::Rav1dLoopfilterModeRefDeltas;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::include::dav1d::headers::Rav1dRestorationType;
use crate::src::align::Align16;
use crate::src::align::ArrayDefault;
use crate::src::ctx::CaseSet;
use crate::src::disjoint_mut::DisjointMut;
use crate::src::internal::Bxy;
use crate::src::levels::BlockSize;
use crate::src::levels::SegmentId;
use crate::src::levels::TxfmSize;
use crate::src::relaxed_atomic::RelaxedAtomic;
use crate::src::tables::dav1d_txfm_dimensions;
use libc::ptrdiff_t;
use parking_lot::RwLock;
use std::cmp;
use std::ffi::c_int;
use std::mem::MaybeUninit;

#[repr(C)]
pub struct Av1FilterLUT {
    pub e: [u8; 64],
    pub i: [u8; 64],
    pub sharp: [u64; 2],
}

impl Default for Av1FilterLUT {
    fn default() -> Self {
        Self {
            e: [0; 64],
            i: [0; 64],
            sharp: Default::default(),
        }
    }
}

impl ArrayDefault for Av1FilterLUT {
    fn default() -> Self {
        Default::default()
    }
}

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct Av1RestorationUnit {
    /// SGR: type = DAV1D_RESTORATION_SGRPROJ + sgr_idx
    pub r#type: Rav1dRestorationType,
    pub filter_h: [i8; 3],
    pub filter_v: [i8; 3],
    pub sgr_weights: [i8; 2],
}

/// each struct describes one 128x128 area (1 or 4 SBs), pre-superres-scaling
#[derive(Default)]
#[repr(C)]
pub struct Av1Filter {
    // each bit is 1 col
    pub filter_y: [[[[RelaxedAtomic<u16>; 2]; 3]; 32]; 2], // 0=col, 1=row
    pub filter_uv: [[[[RelaxedAtomic<u16>; 2]; 2]; 32]; 2], // 0=col, 1=row
    /// -1 means "unset"
    pub cdef_idx: [RelaxedAtomic<i8>; 4],
    /// for 8x8 blocks, but stored on a 4x8 basis
    pub noskip_mask: [[RelaxedAtomic<u16>; 2]; 16],
}

/// each struct describes one 128x128 area (1 or 4 SBs), post-superres-scaling
#[derive(Default)]
#[repr(C)]
pub struct Av1Restoration {
    pub lr: [[RwLock<Av1RestorationUnit>; 4]; 3],
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
///
/// Initializes:
/// * `txa[0][0][y][x]` for all `y` and `x` in the range of the current block
/// * `txa[1][0][y][x]` for all `y` and `x` in the range of the current block
/// * `txa[0][1][y][x_off * t_dim.w]` for all `y` in the range of the current block
/// * `txa[1][1][y_off * t_dim.h][x]` for all `x` in the range of the current block
fn decomp_tx(
    txa: &mut [[[[MaybeUninit<u8>; 32]; 32]; 2]; 2],
    from: TxfmSize,
    depth: usize,
    y_off: u8,
    x_off: u8,
    tx_masks: &[u16; 2],
) {
    debug_assert!(depth <= 2);
    let t_dim = &dav1d_txfm_dimensions[from as usize];

    let y0 = (y_off * t_dim.h) as usize;
    let x0 = (x_off * t_dim.w) as usize;

    let is_split = if from == TxfmSize::S4x4 || depth > 1 {
        false
    } else {
        (tx_masks[depth] >> (y_off * 4 + x_off)) & 1 != 0
    };
    if is_split {
        let sub = t_dim.sub;

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
        let lw = cmp::min(2, t_dim.lw);
        let lh = cmp::min(2, t_dim.lh);

        debug_assert!(t_dim.w == 1 << t_dim.lw && t_dim.w <= 16);
        CaseSet::<16, false>::one((), t_dim.w as usize, x0, |case, ()| {
            for y in 0..t_dim.h as usize {
                case.set(&mut txa[0][0][y0 + y], MaybeUninit::new(lw));
                case.set(&mut txa[1][0][y0 + y], MaybeUninit::new(lh));
                txa[0][1][y0 + y][x0].write(t_dim.w);
            }
        });
        CaseSet::<16, false>::one((), t_dim.w as usize, x0, |case, ()| {
            case.set(&mut txa[1][1][y0], MaybeUninit::new(t_dim.h));
        });
    };
}

#[inline]
fn mask_edges_inter(
    masks: &[[[[RelaxedAtomic<u16>; 2]; 3]; 32]; 2],
    by4: usize,
    bx4: usize,
    w4: usize,
    h4: usize,
    skip: bool,
    max_tx: TxfmSize,
    tx_masks: &[u16; 2],
    a: &mut [u8],
    l: &mut [u8],
) {
    let t_dim = &dav1d_txfm_dimensions[max_tx as usize];

    // See [`decomp_tx`]'s docs for the `txa` arg.

    let mut txa = Align16([[[[MaybeUninit::uninit(); 32]; 32]; 2]; 2]);

    for (y_off, _) in (0..h4).step_by(t_dim.h as usize).enumerate() {
        for (x_off, _) in (0..w4).step_by(t_dim.w as usize).enumerate() {
            decomp_tx(&mut txa.0, max_tx, 0, y_off as u8, x_off as u8, tx_masks);
        }
    }

    // After these calls to `decomp_tx`, the following elements of `txa` are initialized:
    // * `txa[0][0][0..h4][0..w4]`
    // * `txa[1][0][0..h4][0..w4]`
    // * `txa[0][1][0..h4][x]` where `x` is the start of a block edge
    // * `txa[1][1][y][0..w4]` where `y` is the start of a block edge

    // left block edge
    for y in 0..h4 {
        let mask = 1u32 << (by4 + y);
        let sidx = (mask >= 0x10000) as usize;
        let smask = mask >> (sidx << 4);
        // SAFETY: y < h4 so txa[0][0][y][0] is initialized.
        let txa_y = unsafe { txa[0][0][y][0].assume_init() };
        masks[0][bx4][cmp::min(txa_y, l[y]) as usize][sidx].update(|it| it | smask as u16);
    }

    // top block edge
    for x in 0..w4 {
        let mask = 1u32 << (bx4 + x);
        let sidx = (mask >= 0x10000) as usize;
        let smask = mask >> (sidx << 4);
        // SAFETY: x < h4 so txa[1][0][0][x] is initialized.
        let txa_x = unsafe { txa[1][0][0][x].assume_init() };
        masks[1][by4][cmp::min(txa_x, a[x]) as usize][sidx].update(|it| it | smask as u16);
    }
    if !skip {
        // inner (tx) left|right edges
        for y in 0..h4 {
            let mask = 1u32 << (by4 + y);
            let sidx = (mask >= 0x10000) as usize;
            let smask = mask >> (sidx << 4);
            // SAFETY: y < h4 so txa[0][0][y][0] is initialized.
            let mut ltx = unsafe { txa[0][0][y][0].assume_init() };
            // SAFETY: y < h4 and x == 0 so txa[0][1][y][0] is initialized.
            let step = unsafe { txa[0][1][y][0].assume_init() } as usize;
            let mut x = step;
            while x < w4 {
                // SAFETY: x < w4 and y < h4 so txa[0][0][y][x] is initialized.
                let rtx = unsafe { txa[0][0][y][x].assume_init() };
                masks[0][bx4 + x][cmp::min(rtx, ltx) as usize][sidx].update(|it| it | smask as u16);
                ltx = rtx;
                // SAFETY: x is incremented by tdim.w from previously
                // initialized element, so we know that this element is a block
                // edge and also initialized.
                let step = unsafe { txa[0][1][y][x].assume_init() } as usize;
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
            // SAFETY: x < w4 so txa[1][0][0][x] is initialized.
            let mut ttx = unsafe { txa[1][0][0][x].assume_init() };
            // SAFETY: x < h4 and y == 0 so txa[1][1][0][x] is initialized.
            let step = unsafe { txa[1][1][0][x].assume_init() } as usize;
            let mut y = step;
            while y < h4 {
                // SAFETY: x < w4 and y < h4 so txa[1][0][y][x] is initialized.
                let btx = unsafe { txa[1][0][y][x].assume_init() };
                masks[1][by4 + y][cmp::min(ttx, btx) as usize][sidx].update(|it| it | smask as u16);
                ttx = btx;
                // SAFETY: y is incremented by tdim.h from previously
                // initialized element, so we know that this element is a block
                // edge and also initialized.
                let step = unsafe { txa[1][1][y][x].assume_init() } as usize;
                y += step;
            }
        }
    }

    for y in 0..h4 {
        // SAFETY: y < h4 and w4 - 1 < w4 so txa[0][0][y][w4 - 1] is initialized.
        l[y] = unsafe { txa[0][0][y][w4 - 1].assume_init() };
    }
    // SAFETY: h4 - 1 < h4 and ..w4 < w4 so txa[1][0][h4 - 1][..w4] is
    // initialized. Note that this can be replaced by
    // `MaybeUninit::slice_assume_init_ref` if it is stabilized.
    let txa_slice =
        unsafe { &*(&txa[1][0][h4 - 1][..w4] as *const [MaybeUninit<u8>] as *const [u8]) };
    a[..w4].copy_from_slice(txa_slice);
}

#[inline]
fn mask_edges_intra(
    masks: &[[[[RelaxedAtomic<u16>; 2]; 3]; 32]; 2],
    by4: usize,
    bx4: usize,
    w4: usize,
    h4: usize,
    tx: TxfmSize,
    a: &mut [u8],
    l: &mut [u8],
) {
    let t_dim = &dav1d_txfm_dimensions[tx as usize];
    let twl4 = t_dim.lw;
    let thl4 = t_dim.lh;
    let twl4c = cmp::min(2, twl4);
    let thl4c = cmp::min(2, thl4);

    // left block edge
    for y in 0..h4 {
        let mask = 1u32 << (by4 + y);
        let sidx = (mask >= 0x10000) as usize;
        let smask = mask >> (sidx << 4);
        masks[0][bx4][cmp::min(twl4c, l[y]) as usize][sidx].update(|it| it | smask as u16);
    }

    // top block edge
    for x in 0..w4 {
        let mask = 1u32 << (bx4 + x);
        let sidx = (mask >= 0x10000) as usize;
        let smask = mask >> (sidx << 4);
        // SAFETY: No other mutable references to this sub-slice exist on other
        // threads.
        masks[1][by4][cmp::min(thl4c, a[x]) as usize][sidx].update(|it| it | smask as u16);
    }

    // inner (tx) left|right edges
    let hstep = t_dim.w as usize;
    let t = 1u32 << by4;
    let inner = (((t as u64) << h4) - (t as u64)) as u32;
    let inner = [inner as u16, (inner >> 16) as u16];
    for x in (hstep..w4).step_by(hstep) {
        // SAFETY: No other mutable references to this sub-slice exist on other
        // threads.
        if inner[0] != 0 {
            masks[0][bx4 + x][twl4c as usize][0].update(|it| it | inner[0]);
        }
        if inner[1] != 0 {
            masks[0][bx4 + x][twl4c as usize][1].update(|it| it | inner[1]);
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
            masks[1][by4 + y][thl4c as usize][0].update(|it| it | inner[0]);
        }
        if inner[1] != 0 {
            masks[1][by4 + y][thl4c as usize][1].update(|it| it | inner[1]);
        }
    }

    CaseSet::<32, true>::many(
        [(a, thl4c), (l, twl4c)],
        [w4 as usize, h4 as usize],
        [0, 0],
        |case, (dir, tl4c)| {
            case.set(dir, tl4c);
        },
    );
}

fn mask_edges_chroma(
    masks: &[[[[RelaxedAtomic<u16>; 2]; 2]; 32]; 2],
    cby4: usize,
    cbx4: usize,
    cw4: usize,
    ch4: usize,
    skip_inter: bool,
    tx: TxfmSize,
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
        masks[0][cbx4][cmp::min(twl4c, l[y]) as usize][sidx].update(|it| it | smask as u16);
    }

    // top block edge
    for x in 0..cw4 {
        let mask = 1u32 << (cbx4 + x);
        let sidx = (mask >= hmax) as usize;
        let smask = mask >> (sidx << hbits);
        masks[1][cby4][cmp::min(thl4c, a[x]) as usize][sidx].update(|it| it | smask as u16);
    }

    if !skip_inter {
        // inner (tx) left|right edges
        let hstep = t_dim.w as usize;
        let t = 1u32 << cby4;
        let inner = (((t as u64) << ch4) - (t as u64)) as u32;
        let inner = [(inner & ((1 << vmask) - 1)) as u16, (inner >> vmask) as u16];
        for x in (hstep..cw4).step_by(hstep) {
            if inner[0] != 0 {
                masks[0][cbx4 + x][twl4c as usize][0].update(|it| it | inner[0]);
            }
            if inner[1] != 0 {
                masks[0][cbx4 + x][twl4c as usize][1].update(|it| it | inner[1]);
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
                masks[1][cby4 + y][thl4c as usize][0].update(|it| it | inner[0]);
            }
            if inner[1] != 0 {
                masks[1][cby4 + y][thl4c as usize][1].update(|it| it | inner[1]);
            }
        }
    }

    CaseSet::<32, true>::many(
        [(a, thl4c), (l, twl4c)],
        [cw4 as usize, ch4 as usize],
        [0, 0],
        |case, (dir, tl4c)| {
            case.set(dir, tl4c);
        },
    );
}

// not inline in C, and inlining in Rust doesn't seem to improve performance
#[inline(never)]
pub(crate) fn rav1d_create_lf_mask_intra(
    lflvl: &Av1Filter,
    level_cache: &DisjointMut<Vec<u8>>,
    b4_stride: ptrdiff_t,
    filter_level: &Align16<[[[u8; 2]; 8]; 4]>,
    b: Bxy,
    iw: c_int,
    ih: c_int,
    bs: BlockSize,
    ytx: TxfmSize,
    uvtx: TxfmSize,
    layout: Rav1dPixelLayout,
    ay: &mut [u8],
    ly: &mut [u8],
    aluv: Option<(&mut [u8], &mut [u8])>,
) {
    let b4_stride = b4_stride as usize;
    let [bx, by, iw, ih] = [b.x, b.y, iw, ih].map(|it| it as usize);

    let b_dim = bs.dimensions();
    let b_dim = b_dim.map(|it| it as usize);
    let bw4 = cmp::min(iw - bx, b_dim[0]);
    let bh4 = cmp::min(ih - by, b_dim[1]);
    let bx4 = bx & 31;
    let by4 = by & 31;

    if bw4 != 0 && bh4 != 0 {
        let mut level_cache_off = by * b4_stride + bx;
        for _y in 0..bh4 {
            for x in 0..bw4 {
                let idx = 4 * (level_cache_off + x);
                // `0.., ..2` is for Y
                let lvl = &mut *level_cache.index_mut((idx + 0.., ..2));
                lvl[0] = filter_level[0][0][0];
                lvl[1] = filter_level[1][0][0];
            }
            level_cache_off += b4_stride;
        }

        mask_edges_intra(&lflvl.filter_y, by4, bx4, bw4, bh4, ytx, ay, ly);
    }

    let (auv, luv) = match aluv {
        None => return,
        Some(aluv) => aluv,
    };

    let ss_ver = (layout == Rav1dPixelLayout::I420) as usize;
    let ss_hor = (layout != Rav1dPixelLayout::I444) as usize;
    let cbw4 = cmp::min(
        (iw + ss_hor >> ss_hor) - (bx >> ss_hor),
        (b_dim[0] + ss_hor) >> ss_hor,
    );
    let cbh4 = cmp::min(
        (ih + ss_ver >> ss_ver) - (by >> ss_ver),
        (b_dim[1] + ss_ver) >> ss_ver,
    );

    if cbw4 == 0 || cbh4 == 0 {
        return;
    }

    let cbx4 = bx4 >> ss_hor;
    let cby4 = by4 >> ss_ver;

    let mut level_cache_off = (by >> ss_ver) * b4_stride + (bx >> ss_hor);
    for _y in 0..cbh4 {
        for x in 0..cbw4 {
            let idx = 4 * (level_cache_off + x);
            // `2.., ..2` is for UV
            let lvl = &mut *level_cache.index_mut((idx + 2.., ..2));
            lvl[0] = filter_level[2][0][0];
            lvl[1] = filter_level[3][0][0];
        }
        level_cache_off += b4_stride;
    }

    mask_edges_chroma(
        &lflvl.filter_uv,
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

#[inline(never)]
pub(crate) fn rav1d_create_lf_mask_inter(
    lflvl: &Av1Filter,
    level_cache: &DisjointMut<Vec<u8>>,
    b4_stride: ptrdiff_t,
    filter_level: &Align16<[[[u8; 2]; 8]; 4]>,
    r#ref: usize,
    is_gmv: bool,
    b: Bxy,
    iw: c_int,
    ih: c_int,
    skip: bool,
    bs: BlockSize,
    max_ytx: TxfmSize,
    tx_masks: &[u16; 2],
    uvtx: TxfmSize,
    layout: Rav1dPixelLayout,
    ay: &mut [u8],
    ly: &mut [u8],
    aluv: Option<(&mut [u8], &mut [u8])>,
) {
    let b4_stride = b4_stride as usize;
    let is_gmv = is_gmv as usize;
    let [bx, by, iw, ih] = [b.x, b.y, iw, ih].map(|it| it as usize);

    let b_dim = bs.dimensions();
    let b_dim = b_dim.map(|it| it as usize);
    let bw4 = cmp::min(iw - bx, b_dim[0]);
    let bh4 = cmp::min(ih - by, b_dim[1]);
    let bx4 = bx & 31;
    let by4 = by & 31;

    if bw4 != 0 && bh4 != 0 {
        let mut level_cache_off = by * b4_stride + bx;
        for _y in 0..bh4 {
            for x in 0..bw4 {
                let idx = 4 * (level_cache_off + x);
                // `0.., ..2` is for Y
                let lvl = &mut *level_cache.index_mut((idx + 0.., ..2));
                lvl[0] = filter_level[0][r#ref][is_gmv];
                lvl[1] = filter_level[1][r#ref][is_gmv];
            }
            level_cache_off += b4_stride;
        }

        mask_edges_inter(
            &lflvl.filter_y,
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

    let ss_ver = (layout == Rav1dPixelLayout::I420) as usize;
    let ss_hor = (layout != Rav1dPixelLayout::I444) as usize;
    let cbw4 = cmp::min(
        (iw + ss_hor >> ss_hor) - (bx >> ss_hor),
        (b_dim[0] + ss_hor) >> ss_hor,
    );
    let cbh4 = cmp::min(
        (ih + ss_ver >> ss_ver) - (by >> ss_ver),
        (b_dim[1] + ss_ver) >> ss_ver,
    );

    if cbw4 == 0 || cbh4 == 0 {
        return;
    }

    let cbx4 = bx4 >> ss_hor;
    let cby4 = by4 >> ss_ver;

    let mut level_cache_off = (by >> ss_ver) * b4_stride + (bx >> ss_hor);
    for _y in 0..cbh4 {
        for x in 0..cbw4 {
            let idx = 4 * (level_cache_off + x);
            // `2.., ..2` is for UV
            let lvl = &mut *level_cache.index_mut((idx + 2.., ..2));
            lvl[0] = filter_level[2][r#ref][is_gmv];
            lvl[1] = filter_level[3][r#ref][is_gmv];
        }
        level_cache_off += b4_stride;
    }

    mask_edges_chroma(
        &lflvl.filter_uv,
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

pub fn rav1d_calc_eih(lim_lut: &mut Av1FilterLUT, filter_sharpness: u8) {
    // set E/I/H values from loopfilter level
    let sharp = filter_sharpness;
    for level in 0..64 {
        let mut limit = level;

        if sharp > 0 {
            limit >>= sharp + 3 >> 2;
            limit = cmp::min(limit, 9 - sharp);
        }
        limit = cmp::max(limit, 1);

        lim_lut.i[level as usize] = limit;
        lim_lut.e[level as usize] = 2 * (level + 2) + limit;
    }
    let sharp = sharp as u64;
    lim_lut.sharp[0] = sharp + 3 >> 2;
    lim_lut.sharp[1] = if sharp != 0 { 9 - sharp } else { 0xff };
}

fn calc_lf_value(
    lflvl_values: &mut [[u8; 2]; 8],
    base_lvl: u8,
    lf_delta: i8,
    seg_delta: i8,
    mr_delta: Option<&Rav1dLoopfilterModeRefDeltas>,
) {
    let base = iclip(
        iclip(base_lvl as c_int + lf_delta as c_int, 0, 63) + seg_delta as c_int,
        0,
        63,
    );

    if let Some(mr_delta) = mr_delta {
        let sh = (base >= 32) as c_int;
        lflvl_values[0] = [clip(base + mr_delta.ref_delta[0] as i32 * (1 << sh), 0, 63); 2];
        for r in 1..8 {
            for m in 0..2 {
                let delta = mr_delta.mode_delta[m] + mr_delta.ref_delta[r];
                lflvl_values[r][m] = clip(base + delta as i32 * (1 << sh), 0, 63);
            }
        }
    } else {
        *lflvl_values = [[base as u8; 2]; 8];
    }
}

#[inline]
fn calc_lf_value_chroma(
    lflvl_values: &mut [[u8; 2]; 8],
    base_lvl: u8,
    lf_delta: i8,
    seg_delta: i8,
    mr_delta: Option<&Rav1dLoopfilterModeRefDeltas>,
) {
    if base_lvl == 0 {
        *lflvl_values = Default::default();
    } else {
        calc_lf_value(lflvl_values, base_lvl, lf_delta, seg_delta, mr_delta);
    };
}

pub(crate) fn rav1d_calc_lf_values(
    lflvl_values: &mut [Align16<[[[u8; 2]; 8]; 4]>; SegmentId::COUNT],
    hdr: &Rav1dFrameHeader,
    lf_delta: &[i8; 4],
) {
    let n_seg = if hdr.segmentation.enabled != 0 {
        SegmentId::COUNT
    } else {
        1
    };

    if hdr.loopfilter.level_y == [0; 2] {
        lflvl_values[..n_seg].fill_with(Default::default);
        return;
    }

    let mr_deltas = hdr.loopfilter.mode_ref_deltas.clone().into();
    let mr_deltas = if hdr.loopfilter.mode_ref_delta_enabled != 0 {
        Some(&mr_deltas)
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
