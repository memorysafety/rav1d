use crate::include::common::bitdepth::BitDepth;
use crate::include::dav1d::headers::Rav1dFrameHeader;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::src::env::BlockContext;
use crate::src::internal::Rav1dContext;
use crate::src::internal::Rav1dDSPContext;
use crate::src::internal::Rav1dFrameData;
use crate::src::lr_apply::LR_RESTORE_U;
use crate::src::lr_apply::LR_RESTORE_V;
use crate::src::lr_apply::LR_RESTORE_Y;
use crate::src::unstable_extensions::as_chunks;
use crate::src::unstable_extensions::flatten;
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
    mut dst: *mut BD::Pixel,
    dst_stride: ptrdiff_t,
    src: &[BD::Pixel],
    mut src_offset: usize,
    src_stride: ptrdiff_t,
    ss_ver: c_int,
    sb128: c_int,
    mut row: c_int,
    row_h: c_int,
    src_w: c_int,
    h: c_int,
    ss_hor: c_int,
    lr_backup: c_int,
    frame_hdr: &Rav1dFrameHeader,
    dsp: *const Rav1dDSPContext,
    resize_step: [c_int; 2],
    resize_start: [c_int; 2],
    bitdepth_max: c_int,
) {
    let cdef_backup = (lr_backup == 0) as c_int;
    let dst_w = if frame_hdr.size.super_res.enabled != 0 {
        frame_hdr.size.width[1] + ss_hor >> ss_hor
    } else {
        src_w
    };
    // The first stripe of the frame is shorter by 8 luma pixel rows.
    let mut stripe_h = ((64 as c_int) << (cdef_backup & sb128)) - 8 * (row == 0) as c_int >> ss_ver;
    src_offset = src_offset
        .wrapping_add_signed((stripe_h - 2) as isize * BD::pxstride(src_stride as usize) as isize);
    if c.tc.len() == 1 {
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
    if lr_backup != 0 && frame_hdr.size.width[0] != frame_hdr.size.width[1] {
        while row + stripe_h <= row_h {
            let n_lines = 4 - (row + stripe_h + 1 == h) as c_int;
            ((*dsp).mc.resize)(
                dst.cast(),
                dst_stride,
                src.as_ptr().add(src_offset).cast(),
                src_stride,
                dst_w,
                n_lines,
                src_w,
                resize_step[ss_hor as usize],
                resize_start[ss_hor as usize],
                bitdepth_max,
            );
            row += stripe_h; // unmodified stripe_h for the 1st stripe
            stripe_h = 64 >> ss_ver;
            src_offset = (src_offset as isize
                + stripe_h as isize * BD::pxstride(src_stride as usize) as isize)
                as usize;
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
            for i in 0..4 {
                BD::pixel_copy(
                    slice::from_raw_parts_mut(dst, src_w as usize),
                    if i == n_lines_0 {
                        slice::from_raw_parts(
                            &mut *dst.offset(-(BD::pxstride(dst_stride as usize) as isize))
                                as *const BD::Pixel as *const BD::Pixel,
                            src_w as usize,
                        )
                    } else {
                        &src[src_offset..]
                    },
                    src_w as usize,
                );
                dst = dst.offset(BD::pxstride(dst_stride as usize) as isize);
                src_offset =
                    (src_offset as isize + BD::pxstride(src_stride as usize) as isize) as usize;
            }
            row += stripe_h; // unmodified stripe_h for the 1st stripe
            stripe_h = 64 >> ss_ver;
            src_offset = src_offset.wrapping_add_signed(
                (stripe_h - 4) as isize * BD::pxstride(src_stride as usize) as isize,
            );
        }
    };
}

pub(crate) unsafe fn rav1d_copy_lpf<BD: BitDepth>(
    c: &Rav1dContext,
    f: &mut Rav1dFrameData,
    src: &[&mut [BD::Pixel]; 3],
    src_offset: &[usize; 2],
    sby: c_int,
) {
    let have_tt = (c.tc.len() > 1) as c_int;
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    let resize = (frame_hdr.size.width[0] != frame_hdr.size.width[1]) as c_int;
    let offset = 8 * (sby != 0) as c_int;
    let src_stride = &f.cur.stride;
    let lr_stride = &f.sr_cur.p.stride;
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    let tt_off = have_tt * sby * ((4 as c_int) << seq_hdr.sb128);
    let dst: [*mut BD::Pixel; 3] = [
        (f.lf.lr_lpf_line[0] as *mut BD::Pixel)
            .offset(tt_off as isize * BD::pxstride(lr_stride[0] as usize) as isize),
        (f.lf.lr_lpf_line[1] as *mut BD::Pixel)
            .offset(tt_off as isize * BD::pxstride(lr_stride[1] as usize) as isize),
        (f.lf.lr_lpf_line[2] as *mut BD::Pixel)
            .offset(tt_off as isize * BD::pxstride(lr_stride[1] as usize) as isize),
    ];
    let restore_planes = f.lf.restore_planes;

    let cdef_line_buf = BD::cast_pixel_slice_mut(&mut f.lf.cdef_line_buf);

    if seq_hdr.cdef != 0 || restore_planes & LR_RESTORE_Y as c_int != 0 {
        let h = f.cur.p.h;
        let w = f.bw << 2;
        let row_h = cmp::min((sby + 1) << 6 + seq_hdr.sb128, h - 1);
        let y_stripe = (sby << 6 + seq_hdr.sb128) - offset;
        if restore_planes & LR_RESTORE_Y as c_int != 0 || resize == 0 {
            backup_lpf::<BD>(
                c,
                dst[0],
                lr_stride[0],
                src[0],
                (src_offset[0] as isize
                    - offset as isize * BD::pxstride(src_stride[0] as usize) as isize)
                    as usize,
                src_stride[0],
                0,
                seq_hdr.sb128,
                y_stripe,
                row_h,
                w,
                h,
                0,
                1,
                frame_hdr,
                f.dsp,
                f.resize_step,
                f.resize_start,
                f.bitdepth_max,
            );
        }
        if have_tt != 0 && resize != 0 {
            let cdef_off_y: ptrdiff_t =
                (sby * 4) as isize * BD::pxstride(src_stride[0] as usize) as isize;
            backup_lpf::<BD>(
                c,
                cdef_line_buf
                    .as_mut_ptr()
                    .add(f.lf.cdef_lpf_line[0])
                    .offset(cdef_off_y),
                src_stride[0],
                src[0],
                (src_offset[0] as isize
                    - offset as isize * BD::pxstride(src_stride[0] as usize) as isize)
                    as usize,
                src_stride[0],
                0,
                seq_hdr.sb128,
                y_stripe,
                row_h,
                w,
                h,
                0,
                0,
                frame_hdr,
                f.dsp,
                f.resize_step,
                f.resize_start,
                f.bitdepth_max,
            );
        }
    }
    if (seq_hdr.cdef != 0 || restore_planes & (LR_RESTORE_U as c_int | LR_RESTORE_V as c_int) != 0)
        && f.cur.p.layout != Rav1dPixelLayout::I400
    {
        let ss_ver = (f.sr_cur.p.p.layout == Rav1dPixelLayout::I420) as c_int;
        let ss_hor = (f.sr_cur.p.p.layout != Rav1dPixelLayout::I444) as c_int;
        let h_0 = f.cur.p.h + ss_ver >> ss_ver;
        let w_0 = f.bw << 2 - ss_hor;
        let row_h_0 = cmp::min((sby + 1) << 6 - ss_ver + seq_hdr.sb128, h_0 - 1);
        let offset_uv = offset >> ss_ver;
        let y_stripe_0 = (sby << 6 - ss_ver + seq_hdr.sb128) - offset_uv;
        let cdef_off_uv: ptrdiff_t =
            sby as isize * 4 * BD::pxstride(src_stride[1] as usize) as isize;
        if seq_hdr.cdef != 0 || restore_planes & LR_RESTORE_U as c_int != 0 {
            if restore_planes & LR_RESTORE_U as c_int != 0 || resize == 0 {
                backup_lpf::<BD>(
                    c,
                    dst[1],
                    lr_stride[1],
                    src[1],
                    (src_offset[1] as isize
                        - offset_uv as isize * BD::pxstride(src_stride[1] as usize) as isize)
                        as usize,
                    src_stride[1],
                    ss_ver,
                    seq_hdr.sb128,
                    y_stripe_0,
                    row_h_0,
                    w_0,
                    h_0,
                    ss_hor,
                    1,
                    frame_hdr,
                    f.dsp,
                    f.resize_step,
                    f.resize_start,
                    f.bitdepth_max,
                );
            }
            if have_tt != 0 && resize != 0 {
                backup_lpf::<BD>(
                    c,
                    cdef_line_buf
                        .as_mut_ptr()
                        .add(f.lf.cdef_lpf_line[1])
                        .offset(cdef_off_uv),
                    src_stride[1],
                    src[1],
                    (src_offset[1] as isize
                        - offset_uv as isize * BD::pxstride(src_stride[1] as usize) as isize)
                        as usize,
                    src_stride[1],
                    ss_ver,
                    seq_hdr.sb128,
                    y_stripe_0,
                    row_h_0,
                    w_0,
                    h_0,
                    ss_hor,
                    0,
                    frame_hdr,
                    f.dsp,
                    f.resize_step,
                    f.resize_start,
                    f.bitdepth_max,
                );
            }
        }
        if seq_hdr.cdef != 0 || restore_planes & LR_RESTORE_V as c_int != 0 {
            if restore_planes & LR_RESTORE_V as c_int != 0 || resize == 0 {
                backup_lpf::<BD>(
                    c,
                    dst[2],
                    lr_stride[1],
                    src[2],
                    (src_offset[1] as isize
                        - offset_uv as isize * BD::pxstride(src_stride[1] as usize) as isize)
                        as usize,
                    src_stride[1],
                    ss_ver,
                    seq_hdr.sb128,
                    y_stripe_0,
                    row_h_0,
                    w_0,
                    h_0,
                    ss_hor,
                    1,
                    frame_hdr,
                    f.dsp,
                    f.resize_step,
                    f.resize_start,
                    f.bitdepth_max,
                );
            }
            if have_tt != 0 && resize != 0 {
                backup_lpf::<BD>(
                    c,
                    cdef_line_buf
                        .as_mut_ptr()
                        .add(f.lf.cdef_lpf_line[2])
                        .offset(cdef_off_uv),
                    src_stride[1],
                    src[2],
                    (src_offset[1] as isize
                        - offset_uv as isize * BD::pxstride(src_stride[1] as usize) as isize)
                        as usize,
                    src_stride[1],
                    ss_ver,
                    seq_hdr.sb128,
                    y_stripe_0,
                    row_h_0,
                    w_0,
                    h_0,
                    ss_hor,
                    0,
                    frame_hdr,
                    f.dsp,
                    f.resize_step,
                    f.resize_start,
                    f.bitdepth_max,
                );
            }
        }
    }
}

/// Slice `[u8; 4]`s from `lvl`, but "unaligned",
/// meaning the `[u8; 4]`s can straddle
/// adjacent `[u8; 4]`s in the `lvl` slice.
///
/// Note that this does not result in actual unaligned reads,
/// since `[u8; 4]` has an alignment of 1.
/// This optimizes to a single slice with a bounds check.
#[inline(always)]
fn unaligned_lvl_slice(lvl: &[[u8; 4]], y: usize) -> &[[u8; 4]] {
    as_chunks(&flatten(lvl)[y..]).0
}

#[inline]
unsafe fn filter_plane_cols_y<BD: BitDepth>(
    f: &Rav1dFrameData,
    have_left: bool,
    lvl: &[[u8; 4]],
    b4_stride: ptrdiff_t,
    mask: &[[[u16; 2]; 3]; 32],
    dst: &mut [BD::Pixel],
    dst_offset: usize,
    ls: ptrdiff_t,
    w: c_int,
    starty4: c_int,
    endy4: c_int,
) {
    let dsp: &Rav1dDSPContext = &*f.dsp;
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
            // hmask[3] = 0; already initialized above
            (*dsp).lf.loop_filter_sb[0][0](
                dst.as_mut_ptr().add(dst_offset + x * 4).cast(),
                ls,
                hmask.as_mut_ptr(),
                lvl[x..].as_ptr(),
                b4_stride,
                &f.lf.lim_lut.0,
                endy4 - starty4,
                f.bitdepth_max,
            );
        }
    }
}

#[inline]
unsafe fn filter_plane_rows_y<BD: BitDepth>(
    f: &Rav1dFrameData,
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
    let dsp: &Rav1dDSPContext = &*f.dsp;
    for (y, lvl) in (starty4..endy4).zip(lvl.chunks(b4_stride as usize)) {
        if !(!have_top && y == 0) {
            let vmask: [u32; 4] = [
                mask[y as usize][0][0] as u32 | (mask[y as usize][0][1] as u32) << 16,
                mask[y as usize][1][0] as u32 | (mask[y as usize][1][1] as u32) << 16,
                mask[y as usize][2][0] as u32 | (mask[y as usize][2][1] as u32) << 16,
                0,
            ];
            (*dsp).lf.loop_filter_sb[0][1](
                dst.cast(),
                ls,
                vmask.as_ptr(),
                unaligned_lvl_slice(&lvl[0..], 1).as_ptr(),
                b4_stride,
                &f.lf.lim_lut.0,
                w,
                f.bitdepth_max,
            );
        }
        dst = dst.offset(4 * BD::pxstride(ls as usize) as isize);
    }
}

#[inline]
unsafe fn filter_plane_cols_uv<BD: BitDepth>(
    f: &Rav1dFrameData,
    have_left: bool,
    lvl: &[[u8; 4]],
    b4_stride: ptrdiff_t,
    mask: &[[[u16; 2]; 2]; 32],
    u: &mut [BD::Pixel],
    v: &mut [BD::Pixel],
    uv_offset: usize,
    ls: ptrdiff_t,
    w: c_int,
    starty4: c_int,
    endy4: c_int,
    ss_ver: c_int,
) {
    let dsp: &Rav1dDSPContext = &*f.dsp;
    for x in 0..w as usize {
        if !(!have_left && x == 0) {
            let mut hmask: [u32; 3] = [0; 3];
            if starty4 == 0 {
                hmask[0] = mask[x as usize][0][0] as u32;
                hmask[1] = mask[x as usize][1][0] as u32;
                if endy4 > 16 >> ss_ver {
                    hmask[0] |= (mask[x as usize][0][1] as u32) << (16 >> ss_ver);
                    hmask[1] |= (mask[x as usize][1][1] as u32) << (16 >> ss_ver);
                }
            } else {
                hmask[0] = mask[x as usize][0][1] as u32;
                hmask[1] = mask[x as usize][1][1] as u32;
            }
            // hmask[2] = 0; Already initialized to 0 above
            (*dsp).lf.loop_filter_sb[1][0](
                u.as_mut_ptr().add(uv_offset + x * 4).cast(),
                ls,
                hmask.as_mut_ptr(),
                unaligned_lvl_slice(&lvl[x as usize..], 2).as_ptr(),
                b4_stride,
                &f.lf.lim_lut.0,
                endy4 - starty4,
                f.bitdepth_max,
            );
            (*dsp).lf.loop_filter_sb[1][0](
                v.as_mut_ptr().add(uv_offset + x * 4).cast(),
                ls,
                hmask.as_mut_ptr(),
                unaligned_lvl_slice(&lvl[x as usize..], 3).as_ptr(),
                b4_stride,
                &f.lf.lim_lut.0,
                endy4 - starty4,
                f.bitdepth_max,
            );
        }
    }
}

#[inline]
unsafe fn filter_plane_rows_uv<BD: BitDepth>(
    f: &Rav1dFrameData,
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
    let dsp: &Rav1dDSPContext = &*f.dsp;
    let mut off_l: ptrdiff_t = 0;
    for (y, lvl) in (starty4..endy4).zip(lvl.chunks(b4_stride as usize)) {
        if !(!have_top && y == 0) {
            let vmask: [u32; 3] = [
                mask[y as usize][0][0] as u32 | (mask[y as usize][0][1] as u32) << (16 >> ss_hor),
                mask[y as usize][1][0] as u32 | (mask[y as usize][1][1] as u32) << (16 >> ss_hor),
                0,
            ];
            (*dsp).lf.loop_filter_sb[1][1](
                u.offset(off_l as isize).cast(),
                ls,
                vmask.as_ptr(),
                unaligned_lvl_slice(&lvl[0..], 2).as_ptr(),
                b4_stride,
                &f.lf.lim_lut.0,
                w,
                f.bitdepth_max,
            );
            (*dsp).lf.loop_filter_sb[1][1](
                v.offset(off_l as isize).cast(),
                ls,
                vmask.as_ptr(),
                unaligned_lvl_slice(&lvl[0..], 3).as_ptr(),
                b4_stride,
                &f.lf.lim_lut.0,
                w,
                f.bitdepth_max,
            );
        }
        off_l += 4 * BD::pxstride(ls as usize) as isize;
    }
}

pub(crate) unsafe fn rav1d_loopfilter_sbrow_cols<BD: BitDepth>(
    f: &mut Rav1dFrameData,
    p: &mut [&mut [BD::Pixel]; 3],
    p_offset: &[usize; 2],
    lflvl_offset: usize,
    sby: c_int,
    start_of_tile_row: c_int,
) {
    let lflvl = f.lf.mask[lflvl_offset..].as_mut_ptr();
    let mut have_left;
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    let is_sb64 = (seq_hdr.sb128 == 0) as c_int;
    let starty4 = ((sby & is_sb64) as u32) << 4;
    let sbsz = 32 >> is_sb64;
    let sbl2 = 5 - is_sb64;
    let halign = (f.bh + 31 & !31) as usize;
    let ss_ver = (f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
    let ss_hor = (f.cur.p.layout != Rav1dPixelLayout::I444) as c_int;
    let vmask = 16 >> ss_ver;
    let hmask = 16 >> ss_hor;
    let vmax = (1 as c_uint) << vmask;
    let hmax = (1 as c_uint) << hmask;
    let endy4 = starty4 + cmp::min(f.h4 - sby * sbsz, sbsz) as u32;
    let uv_endy4 = (endy4 + ss_ver as u32) >> ss_ver;
    let (lpf_y, lpf_uv) = f.lf.tx_lpf_right_edge.get();
    let mut lpf_y = &lpf_y[(sby << sbl2) as usize..];
    let mut lpf_uv = &lpf_uv[(sby << sbl2 - ss_ver) as usize..];
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    let mut tile_col = 1;
    loop {
        let mut x = frame_hdr.tiling.col_start_sb[tile_col as usize] as c_int;
        if x << sbl2 >= f.bw {
            break;
        }
        let bx4: c_int = if x & is_sb64 != 0 { 16 } else { 0 };
        let cbx4 = bx4 >> ss_hor;
        x >>= is_sb64;
        let y_hmask: &mut [[u16; 2]; 3] =
            &mut (*lflvl.offset(x as isize)).filter_y[0][bx4 as usize];
        for y in starty4..endy4 {
            let mask: u32 = 1 << y;
            let sidx = (mask >= 0x10000) as usize;
            let smask = (mask >> (sidx << 4)) as u16;
            let idx = 2 * (y_hmask[2][sidx] & smask != 0) as usize
                + (y_hmask[1][sidx] & smask != 0) as usize;
            y_hmask[2][sidx] &= !smask;
            y_hmask[1][sidx] &= !smask;
            y_hmask[0][sidx] &= !smask;
            y_hmask[cmp::min(idx, lpf_y[(y - starty4) as usize] as usize)][sidx] |= smask;
        }
        if f.cur.p.layout != Rav1dPixelLayout::I400 {
            let uv_hmask: &mut [[u16; 2]; 2] =
                &mut (*lflvl.offset(x as isize)).filter_uv[0][cbx4 as usize];
            for y in starty4 >> ss_ver..uv_endy4 {
                let uv_mask: u32 = 1 << y;
                let sidx = (uv_mask >= vmax) as usize;
                let smask = (uv_mask >> (sidx << 4 - ss_ver)) as u16;
                let idx = (uv_hmask[1][sidx] & smask != 0) as usize;
                uv_hmask[1][sidx] &= !smask;
                uv_hmask[0][sidx] &= !smask;
                uv_hmask[cmp::min(idx, lpf_uv[(y - (starty4 >> ss_ver)) as usize] as usize)]
                    [sidx] |= smask;
            }
        }
        lpf_y = &lpf_y[halign..];
        lpf_uv = &lpf_uv[(halign >> ss_ver)..];
        tile_col += 1;
    }
    if start_of_tile_row != 0 {
        let mut a: &[BlockContext] = slice::from_raw_parts(f.a, f.a_sz as usize);
        a = &a[(f.sb128w * (start_of_tile_row - 1)) as usize..];
        for x in 0..f.sb128w {
            let y_vmask: &mut [[u16; 2]; 3] =
                &mut (*lflvl.offset(x as isize)).filter_y[1][starty4 as usize];
            let w = cmp::min(32, f.w4 - (x << 5)) as u32;
            for i in 0..w {
                let mask: u32 = 1 << i;
                let sidx = (mask >= 0x10000) as usize;
                let smask = (mask >> (sidx << 4)) as u16;
                let idx = 2 * (y_vmask[2][sidx] & smask != 0) as usize
                    + (y_vmask[1][sidx] & smask != 0) as usize;
                y_vmask[2][sidx] &= !smask;
                y_vmask[1][sidx] &= !smask;
                y_vmask[0][sidx] &= !smask;
                y_vmask[cmp::min(idx, a[0].tx_lpf_y[i as usize] as usize)][sidx] |= smask;
            }
            if f.cur.p.layout != Rav1dPixelLayout::I400 {
                let cw: c_uint = w.wrapping_add(ss_hor as c_uint) >> ss_hor;
                let uv_vmask: &mut [[u16; 2]; 2] =
                    &mut (*lflvl.offset(x as isize)).filter_uv[1][(starty4 >> ss_ver) as usize];
                for i in 0..cw {
                    let uv_mask: u32 = 1 << i;
                    let sidx = (uv_mask >= hmax) as usize;
                    let smask = (uv_mask >> (sidx << 4 - ss_hor)) as u16;
                    let idx = (uv_vmask[1][sidx] & smask != 0) as usize;
                    uv_vmask[1][sidx] &= !smask;
                    uv_vmask[0][sidx] &= !smask;
                    uv_vmask[cmp::min(idx, a[0].tx_lpf_uv[i as usize] as usize)][sidx] |= smask;
                }
            }
            a = &a[1..];
        }
    }
    let mut level_ptr = &f.lf.level[(f.b4_stride * sby as isize * sbsz as isize) as usize..];
    let mut offset = p_offset[0];
    have_left = false;
    for x in 0..f.sb128w {
        filter_plane_cols_y::<BD>(
            f,
            have_left,
            level_ptr,
            f.b4_stride,
            &(*lflvl.offset(x as isize)).filter_y[0],
            p[0],
            offset,
            f.cur.stride[0],
            cmp::min(32, f.w4 - x * 32),
            starty4 as c_int,
            endy4 as c_int,
        );
        have_left = true;
        level_ptr = &level_ptr[32..];
        offset += 128;
    }
    if frame_hdr.loopfilter.level_u == 0 && frame_hdr.loopfilter.level_v == 0 {
        return;
    }
    let mut level_ptr = &f.lf.level[(f.b4_stride * (sby * sbsz >> ss_ver) as isize) as usize..];
    let (pu, pv) = p[1..].split_at_mut(1);
    let mut uv_off = p_offset[1];
    have_left = false;
    for x in 0..f.sb128w {
        filter_plane_cols_uv::<BD>(
            f,
            have_left,
            level_ptr,
            f.b4_stride,
            &(*lflvl.offset(x as isize)).filter_uv[0],
            pu[0],
            pv[0],
            uv_off,
            f.cur.stride[1],
            cmp::min(32, f.w4 - x * 32) + ss_hor >> ss_hor,
            starty4 as c_int >> ss_ver,
            uv_endy4 as c_int,
            ss_ver,
        );
        have_left = true;
        uv_off += 128 >> ss_hor;
        level_ptr = &level_ptr[32 >> ss_hor..];
    }
}

pub(crate) unsafe fn rav1d_loopfilter_sbrow_rows<BD: BitDepth>(
    f: &mut Rav1dFrameData,
    p: &mut [&mut [BD::Pixel]; 3],
    p_offset: &[usize; 2],
    lflvl_offset: usize,
    sby: c_int,
) {
    let lflvl = f.lf.mask[lflvl_offset..].as_mut_ptr();

    // Don't filter outside the frame
    let have_top = sby > 0;
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    let is_sb64 = (seq_hdr.sb128 == 0) as c_int;
    let starty4 = (sby & is_sb64) << 4;
    let sbsz = 32 >> is_sb64;
    let ss_ver = (f.cur.p.layout == Rav1dPixelLayout::I420) as c_int;
    let ss_hor = (f.cur.p.layout != Rav1dPixelLayout::I444) as c_int;
    let endy4: c_uint = (starty4 + cmp::min(f.h4 - sby * sbsz, sbsz)) as c_uint;
    let uv_endy4: c_uint = endy4.wrapping_add(ss_ver as c_uint) >> ss_ver;

    let mut slice: &mut [BD::Pixel];
    let mut level_ptr = &f.lf.level[(f.b4_stride * sby as isize * sbsz as isize) as usize..];
    slice = p[0];
    for x in 0..f.sb128w {
        filter_plane_rows_y::<BD>(
            f,
            have_top,
            level_ptr,
            f.b4_stride,
            &(*lflvl.offset(x as isize)).filter_y[1],
            slice.as_mut_ptr().offset(p_offset[0] as isize),
            f.cur.stride[0],
            cmp::min(32, f.w4 - x * 32),
            starty4,
            endy4 as c_int,
        );
        slice = &mut slice[128..];
        level_ptr = &level_ptr[32..];
    }

    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    if frame_hdr.loopfilter.level_u == 0 && frame_hdr.loopfilter.level_v == 0 {
        return;
    }

    let mut uv_off: ptrdiff_t = 0;
    let mut level_ptr = &f.lf.level[(f.b4_stride * (sby * sbsz >> ss_ver) as isize) as usize..];
    for x in 0..f.sb128w {
        filter_plane_rows_uv::<BD>(
            f,
            have_top,
            level_ptr,
            f.b4_stride,
            &(*lflvl.offset(x as isize)).filter_uv[1],
            p[1][uv_off as usize..].as_mut_ptr().add(p_offset[1]),
            p[2][uv_off as usize..].as_mut_ptr().add(p_offset[1]),
            f.cur.stride[1],
            cmp::min(32 as c_int, f.w4 - x * 32) + ss_hor >> ss_hor,
            starty4 >> ss_ver,
            uv_endy4 as c_int,
            ss_hor,
        );
        uv_off += 128 >> ss_hor;
        level_ptr = &level_ptr[32 >> ss_hor..];
    }
}
