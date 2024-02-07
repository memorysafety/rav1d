use crate::include::common::bitdepth::BitDepth;
use crate::include::common::intops::ulog2;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::src::align::Align16;
use crate::src::cdef::CdefEdgeFlags;
use crate::src::internal::Rav1dContext;
use crate::src::internal::Rav1dDSPContext;
use crate::src::internal::Rav1dFrameContext;
use crate::src::internal::Rav1dTaskContext;
use crate::src::lf_mask::Av1Filter;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::slice;

const CDEF_HAVE_BOTTOM: CdefEdgeFlags = CdefEdgeFlags::CDEF_HAVE_BOTTOM;
const CDEF_HAVE_LEFT: CdefEdgeFlags = CdefEdgeFlags::CDEF_HAVE_LEFT;
const CDEF_HAVE_RIGHT: CdefEdgeFlags = CdefEdgeFlags::CDEF_HAVE_RIGHT;
const CDEF_HAVE_TOP: CdefEdgeFlags = CdefEdgeFlags::CDEF_HAVE_TOP;

pub type Backup2x8Flags = c_uint;
pub const BACKUP_2X8_UV: Backup2x8Flags = 2;
pub const BACKUP_2X8_Y: Backup2x8Flags = 1;

unsafe fn backup2lines<BD: BitDepth>(
    dst: &[*mut BD::Pixel],
    src: &[*mut BD::Pixel; 3],
    stride: &[ptrdiff_t; 2],
    layout: Rav1dPixelLayout,
) {
    let y_stride: ptrdiff_t = BD::pxstride(stride[0] as usize) as isize;
    if y_stride < 0 {
        let len = (-2 * y_stride) as usize;
        BD::pixel_copy(
            slice::from_raw_parts_mut(dst[0].offset(y_stride), len),
            slice::from_raw_parts(src[0].offset(7 * y_stride), len),
            len,
        );
    } else {
        let len = 2 * y_stride as usize;
        BD::pixel_copy(
            slice::from_raw_parts_mut(dst[0], len),
            slice::from_raw_parts(src[0].offset(6 * y_stride), len),
            len,
        );
    }

    if layout != Rav1dPixelLayout::I400 {
        let uv_stride: ptrdiff_t = BD::pxstride(stride[1] as usize) as isize;
        if uv_stride < 0 {
            let uv_off = if layout == Rav1dPixelLayout::I420 {
                3
            } else {
                7
            };

            let len = (-2 * uv_stride) as usize;
            BD::pixel_copy(
                slice::from_raw_parts_mut(dst[1].offset(uv_stride), len),
                slice::from_raw_parts(src[1].offset(uv_off * uv_stride), len),
                len,
            );
            BD::pixel_copy(
                slice::from_raw_parts_mut(dst[2].offset(uv_stride), len),
                slice::from_raw_parts(src[2].offset(uv_off * uv_stride), len),
                len,
            );
        } else {
            let uv_off = if layout == Rav1dPixelLayout::I420 {
                2
            } else {
                6
            };

            let len = 2 * uv_stride as usize;
            BD::pixel_copy(
                slice::from_raw_parts_mut(dst[1], len),
                slice::from_raw_parts(src[1].offset(uv_off * uv_stride), len),
                len,
            );
            BD::pixel_copy(
                slice::from_raw_parts_mut(dst[2], len),
                slice::from_raw_parts(src[2].offset(uv_off * uv_stride), len),
                len,
            );
        }
    }
}

unsafe fn backup2x8<BD: BitDepth>(
    dst: &mut [[[BD::Pixel; 2]; 8]; 3],
    src: &[*mut BD::Pixel; 3],
    src_stride: &[ptrdiff_t; 2],
    mut x_off: c_int,
    layout: Rav1dPixelLayout,
    flag: Backup2x8Flags,
) {
    let mut y_off: ptrdiff_t = 0 as c_int as ptrdiff_t;
    if flag & BACKUP_2X8_Y != 0 {
        for y in 0..8 {
            BD::pixel_copy(
                &mut dst[0][y],
                slice::from_raw_parts(&mut *src[0].offset(y_off + x_off as isize - 2), 2),
                2,
            );
            y_off += BD::pxstride(src_stride[0] as usize) as isize;
        }
    }
    if layout == Rav1dPixelLayout::I400 || flag & BACKUP_2X8_UV == 0 {
        return;
    }
    let ss_ver = (layout == Rav1dPixelLayout::I420) as c_int;
    let ss_hor = (layout != Rav1dPixelLayout::I444) as c_int;
    x_off >>= ss_hor;
    y_off = 0 as c_int as ptrdiff_t;
    for y in 0..8 >> ss_ver {
        BD::pixel_copy(
            &mut dst[1][y],
            slice::from_raw_parts(src[1].offset(y_off + x_off as isize - 2), 2),
            2,
        );
        BD::pixel_copy(
            &mut dst[2][y],
            slice::from_raw_parts(src[2].offset(y_off + x_off as isize - 2), 2),
            2,
        );
        y_off += BD::pxstride(src_stride[1] as usize) as isize;
    }
}

unsafe fn adjust_strength(strength: c_int, var: c_uint) -> c_int {
    if var == 0 {
        return 0;
    }

    let i = if var >> 6 != 0 {
        cmp::min(ulog2(var >> 6), 12 as c_int)
    } else {
        0
    };

    return strength * (4 + i) + 8 >> 4;
}

pub(crate) unsafe fn rav1d_cdef_brow<BD: BitDepth>(
    c: &Rav1dContext,
    tc: &mut Rav1dTaskContext,
    p: &[*mut BD::Pixel; 3],
    lflvl: *const Av1Filter,
    by_start: c_int,
    by_end: c_int,
    sbrow_start: c_int,
    sby: c_int,
) {
    let f: *mut Rav1dFrameContext = tc.f as *mut Rav1dFrameContext;
    let bitdepth_min_8 = if 16 == 8 { 0 } else { (*f).cur.p.bpc - 8 };
    let dsp: *const Rav1dDSPContext = (*f).dsp;
    let mut edges: CdefEdgeFlags = if by_start > 0 {
        CDEF_HAVE_BOTTOM | CDEF_HAVE_TOP
    } else {
        CDEF_HAVE_BOTTOM
    };
    let mut ptrs: [*mut BD::Pixel; 3] = *p;
    let sbsz = 16;
    let sb64w = (*f).sb128w << 1;
    let frame_hdr = &***(*f).frame_hdr.as_ref().unwrap();
    let damping = frame_hdr.cdef.damping + bitdepth_min_8;
    let layout: Rav1dPixelLayout = (*f).cur.p.layout;
    let uv_idx = (Rav1dPixelLayout::I444 as c_uint).wrapping_sub(layout as c_uint) as c_int;
    let ss_ver = (layout == Rav1dPixelLayout::I420) as c_int;
    let ss_hor = (layout != Rav1dPixelLayout::I444) as c_int;

    static UV_DIRS: [[u8; 8]; 2] = [[0, 1, 2, 3, 4, 5, 6, 7], [7, 0, 2, 4, 5, 6, 6, 6]];
    let uv_dir: &[u8; 8] = &UV_DIRS[(layout == Rav1dPixelLayout::I422) as usize];

    let have_tt = (c.tc.len() > 1) as c_int;
    let sb128 = (*f).seq_hdr.as_ref().unwrap().sb128;
    let resize = (frame_hdr.size.width[0] != frame_hdr.size.width[1]) as c_int;
    let y_stride: ptrdiff_t = BD::pxstride((*f).cur.stride[0] as usize) as isize;
    let uv_stride: ptrdiff_t = BD::pxstride((*f).cur.stride[1] as usize) as isize;
    let mut bit = 0;
    for by in (by_start..by_end).step_by(2) {
        let tf = tc.top_pre_cdef_toggle;
        let by_idx = (by & 30) >> 1;
        if by + 2 >= (*f).bh {
            edges.remove(CDEF_HAVE_BOTTOM);
        }
        if (have_tt == 0 || sbrow_start != 0 || (by + 2) < by_end)
            && edges.contains(CDEF_HAVE_BOTTOM)
        {
            let cdef_top_bak: [*mut BD::Pixel; 3] = [
                ((*f).lf.cdef_line[(tf == 0) as usize][0] as *mut BD::Pixel)
                    .offset((have_tt * sby * 4) as isize * y_stride),
                ((*f).lf.cdef_line[(tf == 0) as usize][1] as *mut BD::Pixel)
                    .offset((have_tt * sby * 8) as isize * uv_stride),
                ((*f).lf.cdef_line[(tf == 0) as usize][2] as *mut BD::Pixel)
                    .offset((have_tt * sby * 8) as isize * uv_stride),
            ];
            backup2lines::<BD>(&cdef_top_bak, &ptrs, &(*f).cur.stride, layout);
        }
        let mut lr_bak: Align16<[[[[BD::Pixel; 2]; 8]; 3]; 2]> =
            Align16([[[[0.into(); 2]; 8]; 3]; 2]);
        let mut iptrs: [*mut BD::Pixel; 3] = ptrs;
        edges.remove(CDEF_HAVE_LEFT);
        edges.insert(CDEF_HAVE_RIGHT);
        let mut prev_flag: Backup2x8Flags = 0 as Backup2x8Flags;
        let mut last_skip = true;
        for sbx in 0..sb64w {
            let noskip_row: *const [u16; 2];
            let noskip_mask: c_uint;
            let y_lvl;
            let uv_lvl;
            let flag: Backup2x8Flags;
            let y_pri_lvl;
            let mut y_sec_lvl;
            let uv_pri_lvl;
            let mut uv_sec_lvl;
            let mut bptrs: [*mut BD::Pixel; 3];
            let sb128x = sbx >> 1;
            let sb64_idx = ((by & sbsz) >> 3) + (sbx & 1);
            let cdef_idx = (*lflvl.offset(sb128x as isize)).cdef_idx[sb64_idx as usize] as c_int;
            if cdef_idx == -(1 as c_int)
                || frame_hdr.cdef.y_strength[cdef_idx as usize] == 0
                    && frame_hdr.cdef.uv_strength[cdef_idx as usize] == 0
            {
                last_skip = true;
            } else {
                noskip_row = &*((*lflvl.offset(sb128x as isize)).noskip_mask)
                    .as_ptr()
                    .offset(by_idx as isize) as *const [u16; 2];
                noskip_mask = ((*noskip_row.offset(0))[1] as c_uint) << 16
                    | (*noskip_row.offset(0))[0] as c_uint;
                y_lvl = frame_hdr.cdef.y_strength[cdef_idx as usize];
                uv_lvl = frame_hdr.cdef.uv_strength[cdef_idx as usize];
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
                for bx in (sbx * sbsz..cmp::min((sbx + 1) * sbsz, (*f).bw)).step_by(2) {
                    let uvdir;
                    let do_left;
                    let mut dir;
                    let mut variance: c_uint;
                    let mut top: *const BD::Pixel;
                    let mut bot: *const BD::Pixel;
                    let mut offset: ptrdiff_t;
                    let st_y: bool;
                    if bx + 2 >= (*f).bw {
                        edges.remove(CDEF_HAVE_RIGHT);
                    }
                    let bx_mask: u32 = (3 as c_uint) << (bx & 30);
                    if noskip_mask & bx_mask == 0 {
                        last_skip = true;
                    } else {
                        do_left = (if last_skip {
                            flag
                        } else {
                            (prev_flag ^ flag) & flag
                        }) as c_int;
                        prev_flag = flag;
                        if do_left != 0 && edges.contains(CDEF_HAVE_LEFT) {
                            backup2x8::<BD>(
                                &mut lr_bak[bit as usize],
                                &bptrs,
                                &(*f).cur.stride,
                                0 as c_int,
                                layout,
                                do_left as Backup2x8Flags,
                            );
                        }
                        if edges.contains(CDEF_HAVE_RIGHT) {
                            backup2x8::<BD>(
                                &mut lr_bak[(bit == 0) as usize],
                                &bptrs,
                                &(*f).cur.stride,
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
                        top = 0 as *const BD::Pixel;
                        bot = 0 as *const BD::Pixel;
                        if have_tt == 0 {
                            st_y = true;
                        } else if sbrow_start != 0 && by == by_start {
                            if resize != 0 {
                                offset = ((sby - 1) * 4) as isize * y_stride + (bx * 4) as isize;
                                top = (*f).lf.cdef_lpf_line[0].cast::<BD::Pixel>().offset(offset);
                            } else {
                                offset = (sby * ((4 as c_int) << sb128) - 4) as isize * y_stride
                                    + (bx * 4) as isize;
                                top = (*f).lf.lr_lpf_line[0].cast::<BD::Pixel>().offset(offset);
                            }
                            bot = bptrs[0].offset(8 * y_stride as isize);
                            st_y = false;
                        } else if sbrow_start == 0 && by + 2 >= by_end {
                            top = (*f).lf.cdef_line[tf as usize][0]
                                .cast::<BD::Pixel>()
                                .offset((sby * 4) as isize * y_stride + (bx * 4) as isize);
                            if resize != 0 {
                                offset = (sby * 4 + 2) as isize * y_stride + (bx * 4) as isize;
                                bot = (*f).lf.cdef_lpf_line[0].cast::<BD::Pixel>().offset(offset);
                            } else {
                                let line = sby * ((4 as c_int) << sb128) + 4 * sb128 + 2;
                                offset = line as isize * y_stride + (bx * 4) as isize;
                                bot = (*f).lf.lr_lpf_line[0].cast::<BD::Pixel>().offset(offset);
                            }
                            st_y = false;
                        } else {
                            st_y = true;
                        }

                        if st_y {
                            offset = (sby * 4) as isize * y_stride;
                            top = (*f).lf.cdef_line[tf as usize][0]
                                .cast::<BD::Pixel>()
                                .offset(have_tt as isize * offset + (bx * 4) as isize);
                            bot = bptrs[0].offset(8 * y_stride as isize);
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
                            if !(layout != Rav1dPixelLayout::I400) {
                                unreachable!();
                            }
                            uvdir = if uv_pri_lvl != 0 {
                                uv_dir[dir as usize] as c_int
                            } else {
                                0
                            };
                            for pl in 1..=2 {
                                let st_uv: bool;
                                if have_tt == 0 {
                                    st_uv = true;
                                } else if sbrow_start != 0 && by == by_start {
                                    if resize != 0 {
                                        offset = ((sby - 1) * 4) as isize * uv_stride
                                            + (bx * 4 >> ss_hor) as isize;
                                        top = (*f).lf.cdef_lpf_line[pl]
                                            .cast::<BD::Pixel>()
                                            .offset(offset);
                                    } else {
                                        let line_0 = sby * ((4 as c_int) << sb128) - 4;
                                        offset = line_0 as isize * uv_stride
                                            + (bx * 4 >> ss_hor) as isize;
                                        top = (*f).lf.lr_lpf_line[pl]
                                            .cast::<BD::Pixel>()
                                            .offset(offset);
                                    }
                                    bot = bptrs[pl].offset(((8 >> ss_ver) * uv_stride) as isize);
                                    st_uv = false;
                                } else if sbrow_start == 0 && by + 2 >= by_end {
                                    let top_offset: ptrdiff_t = (sby * 8) as isize * uv_stride
                                        + (bx * 4 >> ss_hor) as isize;
                                    top = (*f).lf.cdef_line[tf as usize][pl]
                                        .cast::<BD::Pixel>()
                                        .offset(top_offset);
                                    if resize != 0 {
                                        offset = (sby * 4 + 2) as isize * uv_stride
                                            + (bx * 4 >> ss_hor) as isize;
                                        bot = (*f).lf.cdef_lpf_line[pl]
                                            .cast::<BD::Pixel>()
                                            .offset(offset);
                                    } else {
                                        let line_1 = sby * ((4 as c_int) << sb128) + 4 * sb128 + 2;
                                        offset = line_1 as isize * uv_stride
                                            + (bx * 4 >> ss_hor) as isize;
                                        bot = (*f).lf.lr_lpf_line[pl]
                                            .cast::<BD::Pixel>()
                                            .offset(offset);
                                    }
                                    st_uv = false;
                                } else {
                                    st_uv = true;
                                }

                                if st_uv {
                                    let offset_0 = (sby * 8) as isize * uv_stride;
                                    top = (*f).lf.cdef_line[tf as usize][pl]
                                        .cast::<BD::Pixel>()
                                        .offset(
                                            have_tt as isize * offset_0
                                                + (bx * 4 >> ss_hor) as isize,
                                        );
                                    bot = bptrs[pl].offset((8 >> ss_ver) * uv_stride);
                                }

                                (*dsp).cdef.fb[uv_idx as usize](
                                    bptrs[pl].cast(),
                                    (*f).cur.stride[1],
                                    (lr_bak[bit as usize][pl]).as_mut_ptr().cast(),
                                    top.cast(),
                                    bot.cast(),
                                    uv_pri_lvl,
                                    uv_sec_lvl,
                                    uvdir,
                                    damping - 1,
                                    edges,
                                    (*f).bitdepth_max,
                                );
                            }
                        }
                        bit ^= 1 as c_int;
                        last_skip = false;
                    }
                    bptrs[0] = bptrs[0].add(8);
                    bptrs[1] = bptrs[1].add(8 >> ss_hor);
                    bptrs[2] = bptrs[2].add(8 >> ss_hor);
                    edges.insert(CDEF_HAVE_LEFT);
                }
            }
            iptrs[0] = iptrs[0].add(sbsz as usize * 4);
            iptrs[1] = iptrs[1].add(sbsz as usize * 4 >> ss_hor);
            iptrs[2] = iptrs[2].add(sbsz as usize * 4 >> ss_hor);
            edges.insert(CDEF_HAVE_LEFT);
        }
        ptrs[0] = ptrs[0].offset(8 * BD::pxstride((*f).cur.stride[0] as usize) as isize);
        ptrs[1] = ptrs[1].offset(8 * BD::pxstride((*f).cur.stride[1] as usize) as isize >> ss_ver);
        ptrs[2] = ptrs[2].offset(8 * BD::pxstride((*f).cur.stride[1] as usize) as isize >> ss_ver);
        tc.top_pre_cdef_toggle ^= 1 as c_int;
        edges.insert(CDEF_HAVE_TOP);
    }
}
