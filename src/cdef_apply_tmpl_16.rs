use crate::include::common::bitdepth::BitDepth16;

use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::src::align::Align16;
use crate::src::cdef::CdefEdgeFlags;
use crate::src::cdef::CDEF_HAVE_BOTTOM;
use crate::src::cdef::CDEF_HAVE_LEFT;
use crate::src::cdef::CDEF_HAVE_RIGHT;
use crate::src::cdef::CDEF_HAVE_TOP;
use crate::src::cdef_apply::adjust_strength;
use crate::src::cdef_apply::backup2lines;
use crate::src::cdef_apply::backup2x8;
use crate::src::internal::Rav1dDSPContext;
use crate::src::internal::Rav1dFrameContext;
use crate::src::internal::Rav1dTaskContext;
use crate::src::lf_mask::Av1Filter;

use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;

pub type pixel = u16;

pub type Backup2x8Flags = c_uint;
pub const BACKUP_2X8_UV: Backup2x8Flags = 2;
pub const BACKUP_2X8_Y: Backup2x8Flags = 1;

#[inline]
unsafe fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 != 0 {
        unreachable!();
    }
    return x >> 1;
}

pub(crate) unsafe fn rav1d_cdef_brow_16bpc(
    tc: *mut Rav1dTaskContext,
    p: *const *mut pixel,
    lflvl: *const Av1Filter,
    by_start: c_int,
    by_end: c_int,
    sbrow_start: c_int,
    sby: c_int,
) {
    let f: *mut Rav1dFrameContext = (*tc).f as *mut Rav1dFrameContext;
    let bitdepth_min_8 = if 16 == 8 {
        0 as c_int
    } else {
        (*f).cur.p.bpc - 8
    };
    let dsp: *const Rav1dDSPContext = (*f).dsp;
    let mut edges: CdefEdgeFlags = (CDEF_HAVE_BOTTOM as c_int
        | (if by_start > 0 {
            CDEF_HAVE_TOP as c_int
        } else {
            0 as c_int
        })) as CdefEdgeFlags;
    let mut ptrs: [*mut pixel; 3] = [*p.offset(0), *p.offset(1), *p.offset(2)];
    let sbsz = 16;
    let sb64w = (*f).sb128w << 1;
    let damping = (*(*f).frame_hdr).cdef.damping + bitdepth_min_8;
    let layout: Rav1dPixelLayout = (*f).cur.p.layout;
    let uv_idx =
        (Rav1dPixelLayout::I444 as c_int as c_uint).wrapping_sub(layout as c_uint) as c_int;
    let ss_ver = (layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint) as c_int;
    let ss_hor = (layout as c_uint != Rav1dPixelLayout::I444 as c_int as c_uint) as c_int;
    static uv_dirs: [[u8; 8]; 2] = [[0, 1, 2, 3, 4, 5, 6, 7], [7, 0, 2, 4, 5, 6, 6, 6]];
    let uv_dir: *const u8 = (uv_dirs
        [(layout as c_uint == Rav1dPixelLayout::I422 as c_int as c_uint) as c_int as usize])
        .as_ptr();
    let have_tt = ((*(*f).c).n_tc > 1 as c_uint) as c_int;
    let sb128 = (*(*f).seq_hdr).sb128;
    let resize = ((*(*f).frame_hdr).size.width[0] != (*(*f).frame_hdr).size.width[1]) as c_int;
    let y_stride: ptrdiff_t = PXSTRIDE((*f).cur.stride[0]);
    let uv_stride: ptrdiff_t = PXSTRIDE((*f).cur.stride[1]);
    let mut bit = 0;
    let mut by = by_start;
    while by < by_end {
        let tf = (*tc).top_pre_cdef_toggle;
        let by_idx = (by & 30) >> 1;
        if by + 2 >= (*f).bh {
            edges = ::core::mem::transmute::<c_uint, CdefEdgeFlags>(
                edges as c_uint & !(CDEF_HAVE_BOTTOM as c_int) as c_uint,
            );
        }
        if (have_tt == 0 || sbrow_start != 0 || (by + 2) < by_end)
            && edges as c_uint & CDEF_HAVE_BOTTOM as c_int as c_uint != 0
        {
            let cdef_top_bak: [*mut pixel; 3] = [
                ((*f).lf.cdef_line[(tf == 0) as c_int as usize][0] as *mut pixel)
                    .offset(((have_tt * sby * 4) as isize * y_stride) as isize),
                ((*f).lf.cdef_line[(tf == 0) as c_int as usize][1] as *mut pixel)
                    .offset(((have_tt * sby * 8) as isize * uv_stride) as isize),
                ((*f).lf.cdef_line[(tf == 0) as c_int as usize][2] as *mut pixel)
                    .offset(((have_tt * sby * 8) as isize * uv_stride) as isize),
            ];
            backup2lines::<BitDepth16>(
                cdef_top_bak.as_ptr(),
                ptrs.as_mut_ptr() as *const *mut pixel,
                ((*f).cur.stride).as_mut_ptr() as *const ptrdiff_t,
                layout,
            );
        }
        let mut lr_bak: Align16<[[[[pixel; 2]; 8]; 3]; 2]> = Align16([[[[0; 2]; 8]; 3]; 2]);
        let mut iptrs: [*mut pixel; 3] = [ptrs[0], ptrs[1], ptrs[2]];
        edges = ::core::mem::transmute::<c_uint, CdefEdgeFlags>(
            edges as c_uint & !(CDEF_HAVE_LEFT as c_int) as c_uint,
        );
        edges = ::core::mem::transmute::<c_uint, CdefEdgeFlags>(
            edges as c_uint | CDEF_HAVE_RIGHT as c_int as c_uint,
        );
        let mut prev_flag: Backup2x8Flags = 0 as Backup2x8Flags;
        let mut sbx = 0;
        let mut last_skip = 1;
        while sbx < sb64w {
            let noskip_row: *const [u16; 2];
            let noskip_mask: c_uint;
            let y_lvl;
            let uv_lvl;
            let flag: Backup2x8Flags;
            let y_pri_lvl;
            let mut y_sec_lvl;
            let uv_pri_lvl;
            let mut uv_sec_lvl;
            let mut bptrs: [*mut pixel; 3];
            let sb128x = sbx >> 1;
            let sb64_idx = ((by & sbsz) >> 3) + (sbx & 1);
            let cdef_idx = (*lflvl.offset(sb128x as isize)).cdef_idx[sb64_idx as usize] as c_int;
            if cdef_idx == -(1 as c_int)
                || (*(*f).frame_hdr).cdef.y_strength[cdef_idx as usize] == 0
                    && (*(*f).frame_hdr).cdef.uv_strength[cdef_idx as usize] == 0
            {
                last_skip = 1 as c_int;
            } else {
                noskip_row = &*((*lflvl.offset(sb128x as isize)).noskip_mask)
                    .as_ptr()
                    .offset(by_idx as isize) as *const [u16; 2];
                noskip_mask = ((*noskip_row.offset(0))[1] as c_uint) << 16
                    | (*noskip_row.offset(0))[0] as c_uint;
                y_lvl = (*(*f).frame_hdr).cdef.y_strength[cdef_idx as usize];
                uv_lvl = (*(*f).frame_hdr).cdef.uv_strength[cdef_idx as usize];
                flag = ((y_lvl != 0) as c_int + (((uv_lvl != 0) as c_int) << 1)) as Backup2x8Flags;
                y_pri_lvl = (y_lvl >> 2) << bitdepth_min_8;
                y_sec_lvl = y_lvl & 3;
                y_sec_lvl += (y_sec_lvl == 3) as c_int;
                y_sec_lvl <<= bitdepth_min_8;
                uv_pri_lvl = (uv_lvl >> 2) << bitdepth_min_8;
                uv_sec_lvl = uv_lvl & 3;
                uv_sec_lvl += (uv_sec_lvl == 3) as c_int;
                uv_sec_lvl <<= bitdepth_min_8;
                bptrs = [iptrs[0], iptrs[1], iptrs[2]];
                let mut bx = sbx * sbsz;
                while bx < cmp::min((sbx + 1) * sbsz, (*f).bw) {
                    let uvdir;
                    let do_left;
                    let mut dir;
                    let mut variance: c_uint;
                    let mut top: *const pixel;
                    let mut bot: *const pixel;
                    let mut offset: ptrdiff_t;
                    let current_block_84: u64;
                    if bx + 2 >= (*f).bw {
                        edges = ::core::mem::transmute::<c_uint, CdefEdgeFlags>(
                            edges as c_uint & !(CDEF_HAVE_RIGHT as c_int) as c_uint,
                        );
                    }
                    let bx_mask: u32 = (3 as c_uint) << (bx & 30);
                    if noskip_mask & bx_mask == 0 {
                        last_skip = 1 as c_int;
                    } else {
                        do_left = (if last_skip != 0 {
                            flag as c_uint
                        } else {
                            (prev_flag as c_uint ^ flag as c_uint) & flag as c_uint
                        }) as c_int;
                        prev_flag = flag;
                        if do_left != 0 && edges as c_uint & CDEF_HAVE_LEFT as c_int as c_uint != 0
                        {
                            backup2x8::<BitDepth16>(
                                (lr_bak[bit as usize]).as_mut_ptr(),
                                bptrs.as_mut_ptr() as *const *mut pixel,
                                ((*f).cur.stride).as_mut_ptr() as *const ptrdiff_t,
                                0 as c_int,
                                layout,
                                do_left as Backup2x8Flags,
                            );
                        }
                        if edges as c_uint & CDEF_HAVE_RIGHT as c_int as c_uint != 0 {
                            backup2x8::<BitDepth16>(
                                (lr_bak[(bit == 0) as c_int as usize]).as_mut_ptr(),
                                bptrs.as_mut_ptr() as *const *mut pixel,
                                ((*f).cur.stride).as_mut_ptr() as *const ptrdiff_t,
                                8 as c_int,
                                layout,
                                flag,
                            );
                        }
                        dir = 0;
                        variance = 0;
                        if y_pri_lvl != 0 || uv_pri_lvl != 0 {
                            dir = ((*dsp).cdef.dir)(
                                bptrs[0].cast(),
                                (*f).cur.stride[0],
                                &mut variance,
                                (*f).bitdepth_max,
                            );
                        }
                        top = 0 as *const pixel;
                        bot = 0 as *const pixel;
                        if have_tt == 0 {
                            current_block_84 = 17728966195399430138;
                        } else if sbrow_start != 0 && by == by_start {
                            if resize != 0 {
                                offset = ((sby - 1) * 4) as isize * y_stride + (bx * 4) as isize;
                                top = &mut *((*((*f).lf.cdef_lpf_line).as_mut_ptr().offset(0))
                                    as *mut pixel)
                                    .offset(offset as isize);
                            } else {
                                offset = (sby * ((4 as c_int) << sb128) - 4) as isize * y_stride
                                    + (bx * 4) as isize;
                                top = &mut *((*((*f).lf.lr_lpf_line).as_mut_ptr().offset(0))
                                    as *mut pixel)
                                    .offset(offset as isize);
                            }
                            bot = (bptrs[0]).offset((8 * y_stride) as isize);
                            current_block_84 = 17075014677070940716;
                        } else if sbrow_start == 0 && by + 2 >= by_end {
                            top = &mut *((*(*((*f).lf.cdef_line).as_mut_ptr().offset(tf as isize))
                                .as_mut_ptr()
                                .offset(0)) as *mut pixel)
                                .offset(
                                    ((sby * 4) as isize * y_stride + (bx * 4) as isize) as isize,
                                );
                            if resize != 0 {
                                offset = (sby * 4 + 2) as isize * y_stride + (bx * 4) as isize;
                                bot = &mut *((*((*f).lf.cdef_lpf_line).as_mut_ptr().offset(0))
                                    as *mut pixel)
                                    .offset(offset as isize);
                            } else {
                                let line = sby * ((4 as c_int) << sb128) + 4 * sb128 + 2;
                                offset = line as isize * y_stride + (bx * 4) as isize;
                                bot = &mut *((*((*f).lf.lr_lpf_line).as_mut_ptr().offset(0))
                                    as *mut pixel)
                                    .offset(offset as isize);
                            }
                            current_block_84 = 17075014677070940716;
                        } else {
                            current_block_84 = 17728966195399430138;
                        }
                        match current_block_84 {
                            17728966195399430138 => {
                                offset = (sby * 4) as isize * y_stride;
                                top = &mut *((*(*((*f).lf.cdef_line)
                                    .as_mut_ptr()
                                    .offset(tf as isize))
                                .as_mut_ptr()
                                .offset(0))
                                    as *mut pixel)
                                    .offset(
                                        (have_tt as isize * offset + (bx * 4) as isize) as isize,
                                    );
                                bot = (bptrs[0]).offset((8 * y_stride) as isize);
                            }
                            _ => {}
                        }
                        if y_pri_lvl != 0 {
                            let adj_y_pri_lvl = adjust_strength(y_pri_lvl, variance);
                            if adj_y_pri_lvl != 0 || y_sec_lvl != 0 {
                                (*dsp).cdef.fb[0](
                                    bptrs[0].cast(),
                                    (*f).cur.stride[0],
                                    (lr_bak[bit as usize][0]).as_mut_ptr().cast(),
                                    top.cast(),
                                    bot.cast(),
                                    adj_y_pri_lvl,
                                    y_sec_lvl,
                                    dir,
                                    damping,
                                    edges,
                                    (*f).bitdepth_max,
                                );
                            }
                        } else if y_sec_lvl != 0 {
                            (*dsp).cdef.fb[0](
                                bptrs[0].cast(),
                                (*f).cur.stride[0],
                                (lr_bak[bit as usize][0]).as_mut_ptr().cast(),
                                top.cast(),
                                bot.cast(),
                                0 as c_int,
                                y_sec_lvl,
                                0 as c_int,
                                damping,
                                edges,
                                (*f).bitdepth_max,
                            );
                        }
                        if !(uv_lvl == 0) {
                            if !(layout as c_uint != Rav1dPixelLayout::I400 as c_int as c_uint) {
                                unreachable!();
                            }
                            uvdir = if uv_pri_lvl != 0 {
                                *uv_dir.offset(dir as isize) as c_int
                            } else {
                                0 as c_int
                            };
                            let mut pl = 1;
                            while pl <= 2 {
                                let current_block_77: u64;
                                if have_tt == 0 {
                                    current_block_77 = 5687667889785024198;
                                } else if sbrow_start != 0 && by == by_start {
                                    if resize != 0 {
                                        offset = ((sby - 1) * 4) as isize * uv_stride
                                            + (bx * 4 >> ss_hor) as isize;
                                        top = &mut *((*((*f).lf.cdef_lpf_line)
                                            .as_mut_ptr()
                                            .offset(pl as isize))
                                            as *mut pixel)
                                            .offset(offset as isize);
                                    } else {
                                        let line_0 = sby * ((4 as c_int) << sb128) - 4;
                                        offset = line_0 as isize * uv_stride
                                            + (bx * 4 >> ss_hor) as isize;
                                        top = &mut *((*((*f).lf.lr_lpf_line)
                                            .as_mut_ptr()
                                            .offset(pl as isize))
                                            as *mut pixel)
                                            .offset(offset as isize);
                                    }
                                    bot = (bptrs[pl as usize])
                                        .offset(((8 >> ss_ver) as isize * uv_stride) as isize);
                                    current_block_77 = 6540614962658479183;
                                } else if sbrow_start == 0 && by + 2 >= by_end {
                                    let top_offset: ptrdiff_t = (sby * 8) as isize * uv_stride
                                        + (bx * 4 >> ss_hor) as isize;
                                    top = &mut *((*(*((*f).lf.cdef_line)
                                        .as_mut_ptr()
                                        .offset(tf as isize))
                                    .as_mut_ptr()
                                    .offset(pl as isize))
                                        as *mut pixel)
                                        .offset(top_offset as isize);
                                    if resize != 0 {
                                        offset = (sby * 4 + 2) as isize * uv_stride
                                            + (bx * 4 >> ss_hor) as isize;
                                        bot = &mut *((*((*f).lf.cdef_lpf_line)
                                            .as_mut_ptr()
                                            .offset(pl as isize))
                                            as *mut pixel)
                                            .offset(offset as isize);
                                    } else {
                                        let line_1 = sby * ((4 as c_int) << sb128) + 4 * sb128 + 2;
                                        offset = line_1 as isize * uv_stride
                                            + (bx * 4 >> ss_hor) as isize;
                                        bot = &mut *((*((*f).lf.lr_lpf_line)
                                            .as_mut_ptr()
                                            .offset(pl as isize))
                                            as *mut pixel)
                                            .offset(offset as isize);
                                    }
                                    current_block_77 = 6540614962658479183;
                                } else {
                                    current_block_77 = 5687667889785024198;
                                }
                                match current_block_77 {
                                    5687667889785024198 => {
                                        let offset_0: ptrdiff_t = (sby * 8) as isize * uv_stride;
                                        top = &mut *((*(*((*f).lf.cdef_line)
                                            .as_mut_ptr()
                                            .offset(tf as isize))
                                        .as_mut_ptr()
                                        .offset(pl as isize))
                                            as *mut pixel)
                                            .offset(
                                                (have_tt as isize * offset_0
                                                    + (bx * 4 >> ss_hor) as isize)
                                                    as isize,
                                            );
                                        bot = (bptrs[pl as usize])
                                            .offset(((8 >> ss_ver) as isize * uv_stride) as isize);
                                    }
                                    _ => {}
                                }
                                (*dsp).cdef.fb[uv_idx as usize](
                                    bptrs[pl as usize].cast(),
                                    (*f).cur.stride[1],
                                    (lr_bak[bit as usize][pl as usize]).as_mut_ptr().cast(),
                                    top.cast(),
                                    bot.cast(),
                                    uv_pri_lvl,
                                    uv_sec_lvl,
                                    uvdir,
                                    damping - 1,
                                    edges,
                                    (*f).bitdepth_max,
                                );
                                pl += 1;
                            }
                        }
                        bit ^= 1 as c_int;
                        last_skip = 0 as c_int;
                    }
                    bptrs[0] = (bptrs[0]).offset(8);
                    bptrs[1] = (bptrs[1]).offset((8 >> ss_hor) as isize);
                    bptrs[2] = (bptrs[2]).offset((8 >> ss_hor) as isize);
                    bx += 2 as c_int;
                    edges = ::core::mem::transmute::<c_uint, CdefEdgeFlags>(
                        edges as c_uint | CDEF_HAVE_LEFT as c_int as c_uint,
                    );
                }
            }
            iptrs[0] = (iptrs[0]).offset((sbsz * 4) as isize);
            iptrs[1] = (iptrs[1]).offset((sbsz * 4 >> ss_hor) as isize);
            iptrs[2] = (iptrs[2]).offset((sbsz * 4 >> ss_hor) as isize);
            sbx += 1;
            edges = ::core::mem::transmute::<c_uint, CdefEdgeFlags>(
                edges as c_uint | CDEF_HAVE_LEFT as c_int as c_uint,
            );
        }
        ptrs[0] = (ptrs[0]).offset((8 * PXSTRIDE((*f).cur.stride[0])) as isize);
        ptrs[1] = (ptrs[1]).offset((8 * PXSTRIDE((*f).cur.stride[1]) >> ss_ver) as isize);
        ptrs[2] = (ptrs[2]).offset((8 * PXSTRIDE((*f).cur.stride[1]) >> ss_ver) as isize);
        (*tc).top_pre_cdef_toggle ^= 1 as c_int;
        by += 2 as c_int;
        edges = ::core::mem::transmute::<c_uint, CdefEdgeFlags>(
            edges as c_uint | CDEF_HAVE_TOP as c_int as c_uint,
        );
    }
}
