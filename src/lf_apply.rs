use crate::include::common::bitdepth::BitDepth;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::src::env::BlockContext;
use crate::src::internal::Rav1dContext;
use crate::src::internal::Rav1dDSPContext;
use crate::src::internal::Rav1dFrameContext;
use crate::src::lf_mask::Av1Filter;
use crate::src::lr_apply::LR_RESTORE_U;
use crate::src::lr_apply::LR_RESTORE_V;
use crate::src::lr_apply::LR_RESTORE_Y;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::slice;

// The loop filter buffer stores 12 rows of pixels. A superblock block will
// contain at most 2 stripes. Each stripe requires 4 rows pixels (2 above
// and 2 below) the final 4 rows are used to swap the bottom of the last
// stripe with the top of the next super block row.
unsafe fn backup_lpf<BD: BitDepth>(
    c: &Rav1dContext,
    f: *const Rav1dFrameContext,
    mut dst: *mut BD::Pixel,
    dst_stride: ptrdiff_t,
    mut src: *const BD::Pixel,
    src_stride: ptrdiff_t,
    ss_ver: c_int,
    sb128: c_int,
    mut row: c_int,
    row_h: c_int,
    src_w: c_int,
    h: c_int,
    ss_hor: c_int,
    lr_backup: c_int,
) {
    let cdef_backup = (lr_backup == 0) as c_int;
    let frame_hdr = &***(*f).frame_hdr.as_ref().unwrap();
    let dst_w = if frame_hdr.size.super_res.enabled != 0 {
        frame_hdr.size.width[1] + ss_hor >> ss_hor
    } else {
        src_w
    };
    // The first stripe of the frame is shorter by 8 luma pixel rows.
    let mut stripe_h = ((64 as c_int) << (cdef_backup & sb128)) - 8 * (row == 0) as c_int >> ss_ver;
    src = src.offset((stripe_h - 2) as isize * BD::pxstride(src_stride as usize) as isize);
    if c.n_tc == 1 as c_uint {
        if row != 0 {
            let top = (4 as c_int) << sb128;
            // Copy the top part of the stored loop filtered pixels from the
            // previous sb row needed above the first stripe of this sb row.
            BD::pixel_copy(
                slice::from_raw_parts_mut(
                    &mut *dst.offset(BD::pxstride(dst_stride as usize * 0) as isize)
                        as *mut BD::Pixel,
                    dst_w as usize,
                ),
                slice::from_raw_parts(
                    &mut *dst.offset(BD::pxstride(dst_stride as usize * top as usize) as isize)
                        as *mut BD::Pixel,
                    dst_w as usize,
                ),
                dst_w as usize,
            );
            BD::pixel_copy(
                slice::from_raw_parts_mut(
                    &mut *dst.offset(BD::pxstride(dst_stride as usize * 1) as isize)
                        as *mut BD::Pixel,
                    dst_w as usize,
                ),
                slice::from_raw_parts(
                    &mut *dst
                        .offset(BD::pxstride(dst_stride as usize * (top + 1) as usize) as isize)
                        as *mut BD::Pixel,
                    dst_w as usize,
                ),
                dst_w as usize,
            );
            BD::pixel_copy(
                slice::from_raw_parts_mut(
                    &mut *dst.offset(BD::pxstride(dst_stride as usize * 2) as isize)
                        as *mut BD::Pixel,
                    dst_w as usize,
                ),
                slice::from_raw_parts(
                    &mut *dst
                        .offset(BD::pxstride(dst_stride as usize * (top + 2) as usize) as isize)
                        as *mut BD::Pixel,
                    dst_w as usize,
                ),
                dst_w as usize,
            );
            BD::pixel_copy(
                slice::from_raw_parts_mut(
                    &mut *dst.offset(BD::pxstride(dst_stride as usize * 3) as isize)
                        as *mut BD::Pixel,
                    dst_w as usize,
                ),
                slice::from_raw_parts(
                    &mut *dst
                        .offset(BD::pxstride(dst_stride as usize * (top + 3) as usize) as isize)
                        as *mut BD::Pixel,
                    dst_w as usize,
                ),
                dst_w as usize,
            );
        }
        dst = dst.offset(4 * BD::pxstride(dst_stride as usize) as isize);
    }
    let frame_hdr = &***(*f).frame_hdr.as_ref().unwrap();
    if lr_backup != 0 && frame_hdr.size.width[0] != frame_hdr.size.width[1] {
        while row + stripe_h <= row_h {
            let n_lines = 4 - (row + stripe_h + 1 == h) as c_int;
            ((*(*f).dsp).mc.resize)(
                dst.cast(),
                dst_stride,
                src.cast(),
                src_stride,
                dst_w,
                n_lines,
                src_w,
                (*f).resize_step[ss_hor as usize],
                (*f).resize_start[ss_hor as usize],
                (*f).bitdepth_max,
            );
            row += stripe_h; // unmodified stripe_h for the 1st stripe
            stripe_h = 64 >> ss_ver;
            src = src.offset(stripe_h as isize * BD::pxstride(src_stride as usize) as isize);
            dst = dst.offset(n_lines as isize * BD::pxstride(dst_stride as usize) as isize);
            if n_lines == 3 {
                BD::pixel_copy(
                    slice::from_raw_parts_mut(dst, dst_w as usize),
                    slice::from_raw_parts(
                        &mut *dst.offset(-(BD::pxstride(dst_stride as usize) as isize))
                            as *mut BD::Pixel,
                        dst_w as usize,
                    ),
                    dst_w as usize,
                );
                dst = dst.offset(BD::pxstride(dst_stride as usize) as isize);
            }
        }
    } else {
        while row + stripe_h <= row_h {
            let n_lines_0 = 4 - (row + stripe_h + 1 == h) as c_int;
            let mut i = 0;
            while i < 4 {
                BD::pixel_copy(
                    slice::from_raw_parts_mut(dst, src_w as usize),
                    slice::from_raw_parts(
                        if i == n_lines_0 {
                            &mut *dst.offset(-(BD::pxstride(dst_stride as usize) as isize))
                                as *const BD::Pixel as *const BD::Pixel
                        } else {
                            src
                        },
                        src_w as usize,
                    ),
                    src_w as usize,
                );
                dst = dst.offset(BD::pxstride(dst_stride as usize) as isize);
                src = src.offset(BD::pxstride(src_stride as usize) as isize);
                i += 1;
            }
            row += stripe_h; // unmodified stripe_h for the 1st stripe
            stripe_h = 64 >> ss_ver;
            src = src.offset((stripe_h - 4) as isize * BD::pxstride(src_stride as usize) as isize);
        }
    };
}

pub(crate) unsafe fn rav1d_copy_lpf<BD: BitDepth>(
    c: &Rav1dContext,
    f: *mut Rav1dFrameContext,
    src: *const *mut BD::Pixel,
    sby: c_int,
) {
    let have_tt = (c.n_tc > 1 as c_uint) as c_int;
    let frame_hdr = &***(*f).frame_hdr.as_ref().unwrap();
    let resize = (frame_hdr.size.width[0] != frame_hdr.size.width[1]) as c_int;
    let offset = 8 * (sby != 0) as c_int;
    let src_stride: *const ptrdiff_t = ((*f).cur.stride).as_mut_ptr();
    let lr_stride: *const ptrdiff_t = ((*f).sr_cur.p.stride).as_mut_ptr();
    let seq_hdr = &***(*f).seq_hdr.as_ref().unwrap();
    let tt_off = have_tt * sby * ((4 as c_int) << seq_hdr.sb128);
    let dst: [*mut BD::Pixel; 3] = [
        ((*f).lf.lr_lpf_line[0] as *mut BD::Pixel)
            .offset(tt_off as isize * BD::pxstride(*lr_stride.offset(0) as usize) as isize),
        ((*f).lf.lr_lpf_line[1] as *mut BD::Pixel)
            .offset(tt_off as isize * BD::pxstride(*lr_stride.offset(1) as usize) as isize),
        ((*f).lf.lr_lpf_line[2] as *mut BD::Pixel)
            .offset(tt_off as isize * BD::pxstride(*lr_stride.offset(1) as usize) as isize),
    ];
    let restore_planes = (*f).lf.restore_planes;
    if seq_hdr.cdef != 0 || restore_planes & LR_RESTORE_Y as c_int != 0 {
        let h = (*f).cur.p.h;
        let w = (*f).bw << 2;
        let row_h = cmp::min((sby + 1) << 6 + seq_hdr.sb128, h - 1);
        let y_stripe = (sby << 6 + seq_hdr.sb128) - offset;
        if restore_planes & LR_RESTORE_Y as c_int != 0 || resize == 0 {
            backup_lpf::<BD>(
                c,
                f,
                dst[0],
                *lr_stride.offset(0),
                (*src.offset(0)).offset(
                    -(offset as isize * BD::pxstride(*src_stride.offset(0) as usize) as isize),
                ),
                *src_stride.offset(0),
                0 as c_int,
                seq_hdr.sb128,
                y_stripe,
                row_h,
                w,
                h,
                0 as c_int,
                1 as c_int,
            );
        }
        if have_tt != 0 && resize != 0 {
            let cdef_off_y: ptrdiff_t =
                (sby * 4) as isize * BD::pxstride(*src_stride.offset(0) as usize) as isize;
            backup_lpf::<BD>(
                c,
                f,
                ((*f).lf.cdef_lpf_line[0] as *mut BD::Pixel).offset(cdef_off_y as isize),
                *src_stride.offset(0),
                (*src.offset(0)).offset(
                    -offset as isize * BD::pxstride(*src_stride.offset(0) as usize) as isize,
                ),
                *src_stride.offset(0),
                0 as c_int,
                seq_hdr.sb128,
                y_stripe,
                row_h,
                w,
                h,
                0 as c_int,
                0 as c_int,
            );
        }
    }
    if (seq_hdr.cdef != 0 || restore_planes & (LR_RESTORE_U as c_int | LR_RESTORE_V as c_int) != 0)
        && (*f).cur.p.layout as c_uint != Rav1dPixelLayout::I400 as c_int as c_uint
    {
        let ss_ver = ((*f).sr_cur.p.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint)
            as c_int;
        let ss_hor = ((*f).sr_cur.p.p.layout as c_uint != Rav1dPixelLayout::I444 as c_int as c_uint)
            as c_int;
        let h_0 = (*f).cur.p.h + ss_ver >> ss_ver;
        let w_0 = (*f).bw << 2 - ss_hor;
        let row_h_0 = cmp::min((sby + 1) << 6 - ss_ver + seq_hdr.sb128, h_0 - 1);
        let offset_uv = offset >> ss_ver;
        let y_stripe_0 = (sby << 6 - ss_ver + seq_hdr.sb128) - offset_uv;
        let cdef_off_uv: ptrdiff_t =
            sby as isize * 4 * BD::pxstride(*src_stride.offset(1) as usize) as isize;
        if seq_hdr.cdef != 0 || restore_planes & LR_RESTORE_U as c_int != 0 {
            if restore_planes & LR_RESTORE_U as c_int != 0 || resize == 0 {
                backup_lpf::<BD>(
                    c,
                    f,
                    dst[1],
                    *lr_stride.offset(1),
                    (*src.offset(1)).offset(
                        -offset_uv as isize * BD::pxstride(*src_stride.offset(1) as usize) as isize,
                    ),
                    *src_stride.offset(1),
                    ss_ver,
                    seq_hdr.sb128,
                    y_stripe_0,
                    row_h_0,
                    w_0,
                    h_0,
                    ss_hor,
                    1 as c_int,
                );
            }
            if have_tt != 0 && resize != 0 {
                backup_lpf::<BD>(
                    c,
                    f,
                    ((*f).lf.cdef_lpf_line[1] as *mut BD::Pixel).offset(cdef_off_uv as isize),
                    *src_stride.offset(1),
                    (*src.offset(1)).offset(
                        -offset_uv as isize * BD::pxstride(*src_stride.offset(1) as usize) as isize,
                    ),
                    *src_stride.offset(1),
                    ss_ver,
                    seq_hdr.sb128,
                    y_stripe_0,
                    row_h_0,
                    w_0,
                    h_0,
                    ss_hor,
                    0 as c_int,
                );
            }
        }
        if seq_hdr.cdef != 0 || restore_planes & LR_RESTORE_V as c_int != 0 {
            if restore_planes & LR_RESTORE_V as c_int != 0 || resize == 0 {
                backup_lpf::<BD>(
                    c,
                    f,
                    dst[2],
                    *lr_stride.offset(1),
                    (*src.offset(2)).offset(
                        -offset_uv as isize * BD::pxstride(*src_stride.offset(1) as usize) as isize,
                    ),
                    *src_stride.offset(1),
                    ss_ver,
                    seq_hdr.sb128,
                    y_stripe_0,
                    row_h_0,
                    w_0,
                    h_0,
                    ss_hor,
                    1 as c_int,
                );
            }
            if have_tt != 0 && resize != 0 {
                backup_lpf::<BD>(
                    c,
                    f,
                    ((*f).lf.cdef_lpf_line[2] as *mut BD::Pixel).offset(cdef_off_uv as isize),
                    *src_stride.offset(1),
                    (*src.offset(2)).offset(
                        -offset_uv as isize * BD::pxstride(*src_stride.offset(1) as usize) as isize,
                    ),
                    *src_stride.offset(1),
                    ss_ver,
                    seq_hdr.sb128,
                    y_stripe_0,
                    row_h_0,
                    w_0,
                    h_0,
                    ss_hor,
                    0 as c_int,
                );
            }
        }
    }
}

#[inline]
unsafe fn filter_plane_cols_y<BD: BitDepth>(
    f: *const Rav1dFrameContext,
    have_left: bool,
    lvl: &[[u8; 4]],
    b4_stride: ptrdiff_t,
    mask: &[[[u16; 2]; 3]; 32],
    dst: *mut BD::Pixel,
    ls: ptrdiff_t,
    w: c_int,
    starty4: c_int,
    endy4: c_int,
) {
    let dsp: *const Rav1dDSPContext = (*f).dsp;

    // filter edges between columns (e.g. block1 | block2)
    for x in 0..w as usize {
        if !(!have_left && x == 0) {
            let mut hmask: [u32; 4] = [0; 4];
            if starty4 == 0 {
                hmask[0] = mask[x][0][0] as u32;
                hmask[1] = mask[x][1][0] as u32;
                hmask[2] = mask[x][2][0] as u32;
                if endy4 > 16 {
                    hmask[0] |= (mask[x][0][1] as u32) << 16;
                    hmask[1] |= (mask[x][1][1] as u32) << 16;
                    hmask[2] |= (mask[x][2][1] as u32) << 16;
                }
            } else {
                hmask[0] = mask[x][0][1] as u32;
                hmask[1] = mask[x][1][1] as u32;
                hmask[2] = mask[x][2][1] as u32;
            }
            hmask[3] = 0;
            (*dsp).lf.loop_filter_sb[0][0](
                dst.add(x * 4).cast(),
                ls,
                hmask.as_mut_ptr(),
                lvl[x][0..].as_ptr() as *const [u8; 4],
                b4_stride,
                &(*f).lf.lim_lut.0,
                endy4 - starty4,
                (*f).bitdepth_max,
            );
        }
    }
}

#[inline]
unsafe fn filter_plane_rows_y<BD: BitDepth>(
    f: *const Rav1dFrameContext,
    have_top: bool,
    lvl: &[[u8; 4]],
    b4_stride: ptrdiff_t,
    mask: &[[[u16; 2]; 3]; 32],
    mut dst: *mut BD::Pixel,
    ls: ptrdiff_t,
    w: c_int,
    starty4: c_int,
    endy4: c_int,
) {
    let dsp: *const Rav1dDSPContext = (*f).dsp;

    //                                 block1
    // filter edges between rows (e.g. ------)
    //                                 block2
    for (y, lvl) in (starty4 as usize..endy4 as usize).zip(lvl.chunks(b4_stride as usize)) {
        if !(!have_top && y == 0) {
            let vmask: [u32; 4] = [
                mask[y][0][0] as u32 | (mask[y][0][1] as u32) << 16,
                mask[y][1][0] as u32 | (mask[y][1][1] as u32) << 16,
                mask[y][2][0] as u32 | (mask[y][2][1] as u32) << 16,
                0,
            ];
            (*dsp).lf.loop_filter_sb[0][1](
                dst.cast(),
                ls,
                vmask.as_ptr(),
                lvl[0][1..].as_ptr() as *const [u8; 4],
                b4_stride,
                &(*f).lf.lim_lut.0,
                w,
                (*f).bitdepth_max,
            );
        }
        dst = dst.offset(4 * BD::pxstride(ls as usize) as isize);
    }
}

#[inline]
unsafe fn filter_plane_cols_uv<BD: BitDepth>(
    f: *const Rav1dFrameContext,
    have_left: bool,
    lvl: &[[u8; 4]],
    b4_stride: ptrdiff_t,
    mask: &[[[u16; 2]; 2]; 32],
    u: *mut BD::Pixel,
    v: *mut BD::Pixel,
    ls: ptrdiff_t,
    w: c_int,
    starty4: c_int,
    endy4: c_int,
    ss_ver: c_int,
) {
    let dsp: *const Rav1dDSPContext = (*f).dsp;

    // filter edges between columns (e.g. block1 | block2)
    for x in 0..w as usize {
        if !(!have_left && x == 0) {
            let mut hmask: [u32; 3] = [0; 3];
            if starty4 == 0 {
                hmask[0] = mask[x][0][0] as u32;
                hmask[1] = mask[x][1][0] as u32;
                if endy4 > 16 >> ss_ver {
                    hmask[0] |= (mask[x][0][1] as u32) << (16 >> ss_ver);
                    hmask[1] |= (mask[x][1][1] as u32) << (16 >> ss_ver);
                }
            } else {
                hmask[0] = mask[x][0][1] as u32;
                hmask[1] = mask[x][1][1] as u32;
            }
            hmask[2] = 0 as c_int as u32;
            (*dsp).lf.loop_filter_sb[1][0](
                u.add(x * 4).cast(),
                ls,
                hmask.as_mut_ptr(),
                lvl[x as usize][2..].as_ptr() as *const [u8; 4],
                b4_stride,
                &(*f).lf.lim_lut.0,
                endy4 - starty4,
                (*f).bitdepth_max,
            );
            (*dsp).lf.loop_filter_sb[1][0](
                v.add(x * 4).cast(),
                ls,
                hmask.as_mut_ptr(),
                lvl[x as usize][3..].as_ptr() as *const [u8; 4],
                b4_stride,
                &(*f).lf.lim_lut.0,
                endy4 - starty4,
                (*f).bitdepth_max,
            );
        }
    }
}

#[inline]
unsafe fn filter_plane_rows_uv<BD: BitDepth>(
    f: *const Rav1dFrameContext,
    have_top: bool,
    lvl: &[[u8; 4]],
    b4_stride: ptrdiff_t,
    mask: &[[[u16; 2]; 2]; 32],
    u: *mut BD::Pixel,
    v: *mut BD::Pixel,
    ls: ptrdiff_t,
    w: c_int,
    starty4: c_int,
    endy4: c_int,
    ss_hor: c_int,
) {
    let dsp: *const Rav1dDSPContext = (*f).dsp;
    let mut off_l: ptrdiff_t = 0 as c_int as ptrdiff_t;

    //                                 block1
    // filter edges between rows (e.g. ------)
    //                                 block2
    for (y, lvl) in (starty4 as usize..endy4 as usize).zip(lvl.chunks(b4_stride as usize)) {
        if !(!have_top && y == 0) {
            let vmask: [u32; 3] = [
                mask[y][0][0] as u32 | (mask[y][0][1] as u32) << (16 >> ss_hor),
                mask[y][1][0] as u32 | (mask[y][1][1] as u32) << (16 >> ss_hor),
                0,
            ];
            (*dsp).lf.loop_filter_sb[1][1](
                u.offset(off_l as isize).cast(),
                ls,
                vmask.as_ptr(),
                lvl[0][2..].as_ptr() as *const [u8; 4],
                b4_stride,
                &(*f).lf.lim_lut.0,
                w,
                (*f).bitdepth_max,
            );
            (*dsp).lf.loop_filter_sb[1][1](
                v.offset(off_l as isize).cast(),
                ls,
                vmask.as_ptr(),
                lvl[0][3..].as_ptr() as *const [u8; 4],
                b4_stride,
                &(*f).lf.lim_lut.0,
                w,
                (*f).bitdepth_max,
            );
        }
        off_l += 4 * BD::pxstride(ls as usize) as isize;
    }
}

pub(crate) unsafe fn rav1d_loopfilter_sbrow_cols<BD: BitDepth>(
    f: *const Rav1dFrameContext,
    p: &[*mut BD::Pixel; 3],
    lflvl: *mut Av1Filter,
    sby: c_int,
    start_of_tile_row: c_int,
) {
    let mut x;
    let mut have_left;
    let seq_hdr = &***(*f).seq_hdr.as_ref().unwrap();
    let is_sb64 = (seq_hdr.sb128 == 0) as c_int;
    let starty4 = (sby & is_sb64) << 4;
    let sbsz = 32 >> is_sb64;
    let sbl2 = 5 - is_sb64;
    let halign = (*f).bh + 31 & !(31 as c_int);
    let ss_ver =
        ((*f).cur.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint) as c_int;
    let ss_hor =
        ((*f).cur.p.layout as c_uint != Rav1dPixelLayout::I444 as c_int as c_uint) as c_int;
    let vmask = 16 >> ss_ver;
    let hmask = 16 >> ss_hor;
    let vmax: c_uint = (1 as c_uint) << vmask;
    let hmax: c_uint = (1 as c_uint) << hmask;
    let endy4: c_uint = (starty4 + cmp::min((*f).h4 - sby * sbsz, sbsz)) as c_uint;
    let uv_endy4: c_uint = endy4.wrapping_add(ss_ver as c_uint) >> ss_ver;
    let mut lpf_y: *const u8 = &mut *(*((*f).lf.tx_lpf_right_edge).as_ptr().offset(0))
        .offset((sby << sbl2) as isize) as *mut u8;
    let mut lpf_uv: *const u8 = &mut *(*((*f).lf.tx_lpf_right_edge).as_ptr().offset(1))
        .offset((sby << sbl2 - ss_ver) as isize) as *mut u8;
    let frame_hdr = &***(*f).frame_hdr.as_ref().unwrap();
    let mut tile_col = 1;
    loop {
        x = frame_hdr.tiling.col_start_sb[tile_col as usize] as c_int;
        if x << sbl2 >= (*f).bw {
            break;
        }
        let bx4 = if x & is_sb64 != 0 {
            16 as c_int
        } else {
            0 as c_int
        };
        let cbx4 = bx4 >> ss_hor;
        x >>= is_sb64;
        let y_hmask: *mut [u16; 2] =
            ((*lflvl.offset(x as isize)).filter_y[0][bx4 as usize]).as_mut_ptr();
        let mut y: c_uint = starty4 as c_uint;
        let mut mask: c_uint = ((1 as c_int) << y) as c_uint;
        while y < endy4 {
            let sidx = (mask >= 0x10000 as c_uint) as c_int;
            let smask: c_uint = mask >> (sidx << 4);
            let idx = 2 as c_int
                * ((*y_hmask.offset(2))[sidx as usize] as c_uint & smask != 0) as c_int
                + ((*y_hmask.offset(1))[sidx as usize] as c_uint & smask != 0) as c_int;
            let ref mut fresh0 = (*y_hmask.offset(2))[sidx as usize];
            *fresh0 = (*fresh0 as c_uint & !smask) as u16;
            let ref mut fresh1 = (*y_hmask.offset(1))[sidx as usize];
            *fresh1 = (*fresh1 as c_uint & !smask) as u16;
            let ref mut fresh2 = (*y_hmask.offset(0))[sidx as usize];
            *fresh2 = (*fresh2 as c_uint & !smask) as u16;
            let ref mut fresh3 = (*y_hmask.offset(cmp::min(
                idx,
                *lpf_y.offset(y.wrapping_sub(starty4 as c_uint) as isize) as c_int,
            ) as isize))[sidx as usize];
            *fresh3 = (*fresh3 as c_uint | smask) as u16;
            y = y.wrapping_add(1);
            mask <<= 1;
        }
        if (*f).cur.p.layout as c_uint != Rav1dPixelLayout::I400 as c_int as c_uint {
            let uv_hmask: *mut [u16; 2] =
                ((*lflvl.offset(x as isize)).filter_uv[0][cbx4 as usize]).as_mut_ptr();
            let mut y_0: c_uint = (starty4 >> ss_ver) as c_uint;
            let mut uv_mask: c_uint = ((1 as c_int) << y_0) as c_uint;
            while y_0 < uv_endy4 {
                let sidx_0 = (uv_mask >= vmax) as c_int;
                let smask_0: c_uint = uv_mask >> (sidx_0 << 4 - ss_ver);
                let idx_0 =
                    ((*uv_hmask.offset(1))[sidx_0 as usize] as c_uint & smask_0 != 0) as c_int;
                let ref mut fresh4 = (*uv_hmask.offset(1))[sidx_0 as usize];
                *fresh4 = (*fresh4 as c_uint & !smask_0) as u16;
                let ref mut fresh5 = (*uv_hmask.offset(0))[sidx_0 as usize];
                *fresh5 = (*fresh5 as c_uint & !smask_0) as u16;
                let ref mut fresh6 = (*uv_hmask.offset(cmp::min(
                    idx_0,
                    *lpf_uv.offset(y_0.wrapping_sub((starty4 >> ss_ver) as c_uint) as isize)
                        as c_int,
                ) as isize))[sidx_0 as usize];
                *fresh6 = (*fresh6 as c_uint | smask_0) as u16;
                y_0 = y_0.wrapping_add(1);
                uv_mask <<= 1;
            }
        }
        lpf_y = lpf_y.offset(halign as isize);
        lpf_uv = lpf_uv.offset((halign >> ss_ver) as isize);
        tile_col += 1;
    }
    if start_of_tile_row != 0 {
        let mut a: *const BlockContext;
        x = 0 as c_int;
        a = &mut *((*f).a).offset(((*f).sb128w * (start_of_tile_row - 1)) as isize)
            as *mut BlockContext;
        while x < (*f).sb128w {
            let y_vmask: *mut [u16; 2] =
                ((*lflvl.offset(x as isize)).filter_y[1][starty4 as usize]).as_mut_ptr();
            let w: c_uint = cmp::min(32 as c_int, (*f).w4 - (x << 5)) as c_uint;
            let mut mask_0: c_uint = 1 as c_int as c_uint;
            let mut i: c_uint = 0 as c_int as c_uint;
            while i < w {
                let sidx_1 = (mask_0 >= 0x10000 as c_uint) as c_int;
                let smask_1: c_uint = mask_0 >> (sidx_1 << 4);
                let idx_1 = 2 as c_int
                    * ((*y_vmask.offset(2))[sidx_1 as usize] as c_uint & smask_1 != 0) as c_int
                    + ((*y_vmask.offset(1))[sidx_1 as usize] as c_uint & smask_1 != 0) as c_int;
                let ref mut fresh7 = (*y_vmask.offset(2))[sidx_1 as usize];
                *fresh7 = (*fresh7 as c_uint & !smask_1) as u16;
                let ref mut fresh8 = (*y_vmask.offset(1))[sidx_1 as usize];
                *fresh8 = (*fresh8 as c_uint & !smask_1) as u16;
                let ref mut fresh9 = (*y_vmask.offset(0))[sidx_1 as usize];
                *fresh9 = (*fresh9 as c_uint & !smask_1) as u16;
                let ref mut fresh10 = (*y_vmask
                    .offset(cmp::min(idx_1, (*a).tx_lpf_y[i as usize] as c_int) as isize))
                    [sidx_1 as usize];
                *fresh10 = (*fresh10 as c_uint | smask_1) as u16;
                mask_0 <<= 1;
                i = i.wrapping_add(1);
            }
            if (*f).cur.p.layout as c_uint != Rav1dPixelLayout::I400 as c_int as c_uint {
                let cw: c_uint = w.wrapping_add(ss_hor as c_uint) >> ss_hor;
                let uv_vmask: *mut [u16; 2] = ((*lflvl.offset(x as isize)).filter_uv[1]
                    [(starty4 >> ss_ver) as usize])
                    .as_mut_ptr();
                let mut uv_mask_0: c_uint = 1 as c_int as c_uint;
                let mut i_0: c_uint = 0 as c_int as c_uint;
                while i_0 < cw {
                    let sidx_2 = (uv_mask_0 >= hmax) as c_int;
                    let smask_2: c_uint = uv_mask_0 >> (sidx_2 << 4 - ss_hor);
                    let idx_2 =
                        ((*uv_vmask.offset(1))[sidx_2 as usize] as c_uint & smask_2 != 0) as c_int;
                    let ref mut fresh11 = (*uv_vmask.offset(1))[sidx_2 as usize];
                    *fresh11 = (*fresh11 as c_uint & !smask_2) as u16;
                    let ref mut fresh12 = (*uv_vmask.offset(0))[sidx_2 as usize];
                    *fresh12 = (*fresh12 as c_uint & !smask_2) as u16;
                    let ref mut fresh13 = (*uv_vmask
                        .offset(cmp::min(idx_2, (*a).tx_lpf_uv[i_0 as usize] as c_int) as isize))
                        [sidx_2 as usize];
                    *fresh13 = (*fresh13 as c_uint | smask_2) as u16;
                    uv_mask_0 <<= 1;
                    i_0 = i_0.wrapping_add(1);
                }
            }
            x += 1;
            a = a.offset(1);
        }
    }
    let mut ptr: *mut BD::Pixel;
    let level_ptr = &(*f).lf.level[((*f).b4_stride * sby as isize * sbsz as isize) as usize..];
    ptr = p[0];
    have_left = false;
    for (x, level_ptr) in (0..(*f).sb128w).zip(level_ptr.chunks(32)) {
        filter_plane_cols_y::<BD>(
            f,
            have_left,
            level_ptr,
            (*f).b4_stride,
            &(*lflvl.offset(x as isize)).filter_y[0],
            ptr,
            (*f).cur.stride[0],
            cmp::min(32 as c_int, (*f).w4 - x * 32),
            starty4,
            endy4 as c_int,
        );
        have_left = true;
        ptr = ptr.offset(128);
    }
    if frame_hdr.loopfilter.level_u == 0 && frame_hdr.loopfilter.level_v == 0 {
        return;
    }
    let mut uv_off: ptrdiff_t;
    let level_ptr = &(*f).lf.level[((*f).b4_stride * (sby * sbsz >> ss_ver) as isize) as usize..];
    have_left = false;
    uv_off = 0;
    for (x, level_ptr) in (0..(*f).sb128w).zip(level_ptr.chunks(32 >> ss_hor)) {
        filter_plane_cols_uv::<BD>(
            f,
            have_left,
            level_ptr,
            (*f).b4_stride,
            &(*lflvl.offset(x as isize)).filter_uv[0],
            &mut *p[1].offset(uv_off as isize),
            &mut *p[2].offset(uv_off as isize),
            (*f).cur.stride[1],
            cmp::min(32 as c_int, (*f).w4 - x * 32) + ss_hor >> ss_hor,
            starty4 >> ss_ver,
            uv_endy4 as c_int,
            ss_ver,
        );
        have_left = true;
        uv_off += 128 >> ss_hor;
    }
}

pub(crate) unsafe fn rav1d_loopfilter_sbrow_rows<BD: BitDepth>(
    f: *const Rav1dFrameContext,
    p: &[*mut BD::Pixel; 3],
    lflvl: *mut Av1Filter,
    sby: c_int,
) {
    // Don't filter outside the frame
    let have_top = sby > 0;
    let seq_hdr = &***(*f).seq_hdr.as_ref().unwrap();
    let is_sb64 = (seq_hdr.sb128 == 0) as c_int;
    let starty4 = (sby & is_sb64) << 4;
    let sbsz = 32 >> is_sb64;
    let ss_ver = ((*f).cur.p.layout == Rav1dPixelLayout::I420) as c_int;
    let ss_hor = ((*f).cur.p.layout != Rav1dPixelLayout::I444) as c_int;
    let endy4: c_uint = (starty4 + cmp::min((*f).h4 - sby * sbsz, sbsz)) as c_uint;
    let uv_endy4: c_uint = endy4.wrapping_add(ss_ver as c_uint) >> ss_ver;

    let mut ptr: *mut BD::Pixel;
    let mut level_ptr = &(*f).lf.level[((*f).b4_stride * sby as isize * sbsz as isize) as usize..];
    ptr = p[0];
    for x in 0..(*f).sb128w {
        filter_plane_rows_y::<BD>(
            f,
            have_top,
            level_ptr,
            (*f).b4_stride,
            &(*lflvl.offset(x as isize)).filter_y[1],
            ptr,
            (*f).cur.stride[0],
            cmp::min(32, (*f).w4 - x * 32),
            starty4,
            endy4 as c_int,
        );
        ptr = ptr.offset(128);
        level_ptr = &level_ptr[32..];
    }

    let frame_hdr = &***(*f).frame_hdr.as_ref().unwrap();
    if frame_hdr.loopfilter.level_u == 0 && frame_hdr.loopfilter.level_v == 0 {
        return;
    }

    let mut uv_off: ptrdiff_t;
    let mut level_ptr =
        &(*f).lf.level[((*f).b4_stride * (sby * sbsz >> ss_ver) as isize) as usize..];
    uv_off = 0;
    for x in 0..(*f).sb128w {
        filter_plane_rows_uv::<BD>(
            f,
            have_top,
            level_ptr,
            (*f).b4_stride,
            &(*lflvl.offset(x as isize)).filter_uv[1],
            &mut *p[1].offset(uv_off as isize),
            &mut *p[2].offset(uv_off as isize),
            (*f).cur.stride[1],
            cmp::min(32 as c_int, (*f).w4 - x * 32) + ss_hor >> ss_hor,
            starty4 >> ss_ver,
            uv_endy4 as c_int,
            ss_hor,
        );
        uv_off += 128 >> ss_hor;
        level_ptr = &level_ptr[32 >> ss_hor..];
    }
}
