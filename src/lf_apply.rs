use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynPixel;
use crate::include::dav1d::headers::Rav1dFrameHeader;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::src::align::AlignedVec64;
use crate::src::disjoint_mut::DisjointMut;
use crate::src::internal::Rav1dBitDepthDSPContext;
use crate::src::internal::Rav1dContext;
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
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering;

// The loop filter buffer stores 12 rows of pixels. A superblock block will
// contain at most 2 stripes. Each stripe requires 4 rows pixels (2 above
// and 2 below) the final 4 rows are used to swap the bottom of the last
// stripe with the top of the next super block row.
unsafe fn backup_lpf<BD: BitDepth>(
    c: &Rav1dContext,
    dst: &DisjointMut<AlignedVec64<u8>>,
    mut dst_offset: usize, // in pixel units
    dst_stride: ptrdiff_t,
    src: &[BD::Pixel],
    mut src_offset: usize,
    src_stride: ptrdiff_t,
    ss_ver: c_int,
    sb128: u8,
    mut row: c_int,
    row_h: c_int,
    src_w: c_int,
    h: c_int,
    ss_hor: c_int,
    lr_backup: c_int,
    frame_hdr: &Rav1dFrameHeader,
    dsp: &Rav1dBitDepthDSPContext,
    resize_step: [c_int; 2],
    resize_start: [c_int; 2],
    bitdepth_max: c_int,
) {
    let cdef_backup = (lr_backup == 0) as c_int;
    let dst_w = if frame_hdr.size.super_res.enabled {
        frame_hdr.size.width[1] + ss_hor >> ss_hor
    } else {
        src_w
    };
    // The first stripe of the frame is shorter by 8 luma pixel rows.
    let mut stripe_h =
        ((64 as c_int) << (cdef_backup & sb128 as c_int)) - 8 * (row == 0) as c_int >> ss_ver;
    src_offset =
        (src_offset as isize + (stripe_h - 2) as isize * BD::pxstride(src_stride)) as usize;
    if c.tc.len() == 1 {
        if row != 0 {
            let top = 4 << sb128;
            let px_abs_stride = BD::pxstride(dst_stride.unsigned_abs());
            let top_size = top * px_abs_stride;
            // Copy the top part of the stored loop filtered pixels from the
            // previous sb row needed above the first stripe of this sb row.
            let (dst_idx, src_idx) = if dst_stride < 0 {
                (
                    dst_offset - 3 * px_abs_stride,
                    dst_offset - top_size - 3 * px_abs_stride,
                )
            } else {
                (dst_offset, dst_offset + top_size)
            };

            for i in 0..4 {
                BD::pixel_copy(
                    &mut dst.mut_slice_as(
                        dst_idx + i * px_abs_stride..dst_idx + i * px_abs_stride + dst_w as usize,
                    ),
                    &dst.slice_as(
                        src_idx + i * px_abs_stride..src_idx + i * px_abs_stride + dst_w as usize,
                    ),
                    dst_w as usize,
                );
            }
        }
        dst_offset = (dst_offset as isize + 4 * BD::pxstride(dst_stride)) as usize;
    }
    if lr_backup != 0 && frame_hdr.size.width[0] != frame_hdr.size.width[1] {
        while row + stripe_h <= row_h {
            let n_lines = 4 - (row + stripe_h + 1 == h) as c_int;
            let mut dst_guard = dst.mut_slice_as(dst_offset..dst_offset + dst_w as usize);
            (dsp.mc.resize)(
                dst_guard.as_mut_ptr() as *mut BD::Pixel as *mut DynPixel,
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
            src_offset =
                (src_offset as isize + stripe_h as isize * BD::pxstride(src_stride)) as usize;
            dst_offset =
                (dst_offset as isize + n_lines as isize * BD::pxstride(dst_stride)) as usize;

            if n_lines == 3 {
                let dst_abs_px_stride = BD::pxstride(dst_stride.unsigned_abs());
                let (src_idx, dst_idx) = if dst_stride < 0 {
                    (dst_offset + dst_abs_px_stride, dst_offset)
                } else {
                    (dst_offset - dst_abs_px_stride, dst_offset)
                };
                BD::pixel_copy(
                    &mut dst.mut_slice_as(dst_idx..dst_idx + dst_w as usize),
                    &dst.slice_as(src_idx..src_idx + dst_w as usize),
                    dst_w as usize,
                );
                dst_offset = (dst_offset as isize + BD::pxstride(dst_stride)) as usize;
            }
        }
    } else {
        while row + stripe_h <= row_h {
            let n_lines = 4 - (row + stripe_h + 1 == h) as c_int;
            for i in 0..4 {
                let dst_abs_px_stride = BD::pxstride(dst_stride.unsigned_abs());
                if i != n_lines {
                    BD::pixel_copy(
                        &mut dst.mut_slice_as(dst_offset..dst_offset + src_w as usize),
                        &src[src_offset..],
                        src_w as usize,
                    );
                } else {
                    let (src_idx, dst_idx) = if dst_stride < 0 {
                        (dst_offset + dst_abs_px_stride, dst_offset)
                    } else {
                        (dst_offset - dst_abs_px_stride, dst_offset)
                    };
                    BD::pixel_copy(
                        &mut dst.mut_slice_as(dst_idx..dst_idx + src_w as usize),
                        &dst.slice_as(src_idx..src_idx + src_w as usize),
                        src_w as usize,
                    )
                }
                dst_offset = (dst_offset as isize + BD::pxstride(dst_stride)) as usize;
                src_offset = (src_offset as isize + BD::pxstride(src_stride)) as usize;
            }
            row += stripe_h; // unmodified stripe_h for the 1st stripe
            stripe_h = 64 >> ss_ver;
            src_offset =
                src_offset.wrapping_add_signed((stripe_h - 4) as isize * BD::pxstride(src_stride));
        }
    };
}

pub(crate) unsafe fn rav1d_copy_lpf<BD: BitDepth>(
    c: &Rav1dContext,
    f: &Rav1dFrameData,
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

    let src_y_stride = BD::pxstride(src_stride[0]);
    let src_uv_stride = BD::pxstride(src_stride[1]);
    let y_stride = BD::pxstride(lr_stride[0]);
    let uv_stride = BD::pxstride(lr_stride[1]);

    let y_offset = tt_off as isize * y_stride;
    let uv_offset = tt_off as isize * uv_stride;
    let dst_offset = [
        f.lf.lr_lpf_line[0].wrapping_add_signed(y_offset),
        f.lf.lr_lpf_line[1].wrapping_add_signed(uv_offset),
        f.lf.lr_lpf_line[2].wrapping_add_signed(uv_offset),
    ];

    // TODO Also check block level restore type to reduce copying.
    let restore_planes = f.lf.restore_planes;

    if seq_hdr.cdef != 0 || restore_planes & LR_RESTORE_Y as c_int != 0 {
        let h = f.cur.p.h;
        let w = f.bw << 2;
        let row_h = cmp::min((sby + 1) << 6 + seq_hdr.sb128, h - 1);
        let y_stripe = (sby << 6 + seq_hdr.sb128) - offset;
        if restore_planes & LR_RESTORE_Y as c_int != 0 || resize == 0 {
            backup_lpf::<BD>(
                c,
                &f.lf.lr_line_buf,
                dst_offset[0],
                lr_stride[0],
                src[0],
                (src_offset[0] as isize - offset as isize * src_y_stride) as usize,
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
            let cdef_off_y: ptrdiff_t = (sby * 4) as isize * src_y_stride;
            let cdef_plane_y_sz = 4 * f.sbh as isize * src_y_stride;
            let y_span = cdef_plane_y_sz - src_y_stride;
            let cdef_line_start = (f.lf.cdef_lpf_line[0] as isize + cmp::min(y_span, 0)) as usize;
            backup_lpf::<BD>(
                c,
                &f.lf.cdef_line_buf,
                cdef_line_start + (cdef_off_y - cmp::min(y_span, 0)) as usize,
                src_stride[0],
                src[0],
                (src_offset[0] as isize - offset as isize * src_y_stride as isize) as usize,
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
        let row_h_0 = cmp::min((sby + 1) << 6 - ss_ver + seq_hdr.sb128 as c_int, h_0 - 1);
        let offset_uv = offset >> ss_ver;
        let y_stripe_0 = (sby << 6 - ss_ver + seq_hdr.sb128 as c_int) - offset_uv;
        let cdef_off_uv: ptrdiff_t = sby as isize * 4 * src_uv_stride;
        if seq_hdr.cdef != 0 || restore_planes & LR_RESTORE_U as c_int != 0 {
            if restore_planes & LR_RESTORE_U as c_int != 0 || resize == 0 {
                backup_lpf::<BD>(
                    c,
                    &f.lf.lr_line_buf,
                    dst_offset[1],
                    lr_stride[1],
                    src[1],
                    (src_offset[1] as isize - offset_uv as isize * src_uv_stride) as usize,
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
                let cdef_plane_uv_sz = 4 * f.sbh as isize * src_uv_stride;
                let uv_span = cdef_plane_uv_sz - src_uv_stride;
                let cdef_line_start =
                    (f.lf.cdef_lpf_line[1] as isize + cmp::min(uv_span, 0)) as usize;
                backup_lpf::<BD>(
                    c,
                    &f.lf.cdef_line_buf,
                    cdef_line_start + (cdef_off_uv - cmp::min(uv_span, 0)) as usize,
                    src_stride[1],
                    src[1],
                    (src_offset[1] as isize - offset_uv as isize * src_uv_stride) as usize,
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
                    &f.lf.lr_line_buf,
                    dst_offset[2],
                    lr_stride[1],
                    src[2],
                    (src_offset[1] as isize - offset_uv as isize * src_uv_stride) as usize,
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
                let cdef_plane_uv_sz = 4 * f.sbh as isize * src_uv_stride;
                let uv_span = cdef_plane_uv_sz - src_uv_stride;
                let cdef_line_start =
                    (f.lf.cdef_lpf_line[2] as isize + cmp::min(uv_span, 0)) as usize;
                backup_lpf::<BD>(
                    c,
                    &f.lf.cdef_line_buf,
                    cdef_line_start + (cdef_off_uv - cmp::min(uv_span, 0)) as usize,
                    src_stride[1],
                    src[2],
                    (src_offset[1] as isize - offset_uv as isize * src_uv_stride) as usize,
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
    mask: &[[[AtomicU16; 2]; 3]; 32],
    dst: &mut [BD::Pixel],
    dst_offset: usize,
    ls: ptrdiff_t,
    w: c_int,
    starty4: c_int,
    endy4: c_int,
) {
    let bd = BD::from_c(f.bitdepth_max);

    // filter edges between columns (e.g. block1 | block2)
    for x in 0..w as usize {
        if !(!have_left && x == 0) {
            let mut hmask: [u32; 4] = [0; 4];
            if starty4 == 0 {
                hmask[0] = mask[x][0][0].load(Ordering::Relaxed) as u32;
                hmask[1] = mask[x][1][0].load(Ordering::Relaxed) as u32;
                hmask[2] = mask[x][2][0].load(Ordering::Relaxed) as u32;
                if endy4 > 16 {
                    hmask[0] |= (mask[x][0][1].load(Ordering::Relaxed) as u32) << 16;
                    hmask[1] |= (mask[x][1][1].load(Ordering::Relaxed) as u32) << 16;
                    hmask[2] |= (mask[x][2][1].load(Ordering::Relaxed) as u32) << 16;
                }
            } else {
                hmask[0] = mask[x][0][1].load(Ordering::Relaxed) as u32;
                hmask[1] = mask[x][1][1].load(Ordering::Relaxed) as u32;
                hmask[2] = mask[x][2][1].load(Ordering::Relaxed) as u32;
            }
            // hmask[3] = 0; already initialized above
            f.dsp.lf.loop_filter_sb[0][0].call::<BD>(
                dst.as_mut_ptr().add(dst_offset + x * 4),
                ls,
                &hmask,
                &lvl[x..],
                b4_stride,
                &f.lf.lim_lut.0,
                endy4 - starty4,
                bd,
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
    mask: &[[[AtomicU16; 2]; 3]; 32],
    dst: &mut [BD::Pixel],
    mut dst_offset: usize,
    ls: ptrdiff_t,
    w: c_int,
    starty4: c_int,
    endy4: c_int,
) {
    let bd = BD::from_c(f.bitdepth_max);

    //                                 block1
    // filter edges between rows (e.g. ------)
    //                                 block2
    for (y, lvl) in (starty4..endy4).zip(lvl.chunks(b4_stride as usize)) {
        if !(!have_top && y == 0) {
            let vmask: [u32; 4] = [
                mask[y as usize][0][0].load(Ordering::Relaxed) as u32
                    | (mask[y as usize][0][1].load(Ordering::Relaxed) as u32) << 16,
                mask[y as usize][1][0].load(Ordering::Relaxed) as u32
                    | (mask[y as usize][1][1].load(Ordering::Relaxed) as u32) << 16,
                mask[y as usize][2][0].load(Ordering::Relaxed) as u32
                    | (mask[y as usize][2][1].load(Ordering::Relaxed) as u32) << 16,
                0,
            ];
            f.dsp.lf.loop_filter_sb[0][1].call::<BD>(
                dst.as_mut_ptr().add(dst_offset),
                ls,
                &vmask,
                unaligned_lvl_slice(&lvl[0..], 1),
                b4_stride,
                &f.lf.lim_lut.0,
                w,
                bd,
            );
        }
        dst_offset = (dst_offset as isize + 4 * BD::pxstride(ls)) as usize;
    }
}

#[inline]
unsafe fn filter_plane_cols_uv<BD: BitDepth>(
    f: &Rav1dFrameData,
    have_left: bool,
    lvl: &[[u8; 4]],
    b4_stride: ptrdiff_t,
    mask: &[[[AtomicU16; 2]; 2]; 32],
    u: &mut [BD::Pixel],
    v: &mut [BD::Pixel],
    uv_offset: usize,
    ls: ptrdiff_t,
    w: c_int,
    starty4: c_int,
    endy4: c_int,
    ss_ver: c_int,
) {
    let bd = BD::from_c(f.bitdepth_max);

    // filter edges between columns (e.g. block1 | block2)
    for x in 0..w as usize {
        if !(!have_left && x == 0) {
            let mut hmask: [u32; 3] = [0; 3];
            if starty4 == 0 {
                hmask[0] = mask[x as usize][0][0].load(Ordering::Relaxed) as u32;
                hmask[1] = mask[x as usize][1][0].load(Ordering::Relaxed) as u32;
                if endy4 > 16 >> ss_ver {
                    hmask[0] |=
                        (mask[x as usize][0][1].load(Ordering::Relaxed) as u32) << (16 >> ss_ver);
                    hmask[1] |=
                        (mask[x as usize][1][1].load(Ordering::Relaxed) as u32) << (16 >> ss_ver);
                }
            } else {
                hmask[0] = mask[x as usize][0][1].load(Ordering::Relaxed) as u32;
                hmask[1] = mask[x as usize][1][1].load(Ordering::Relaxed) as u32;
            }
            // hmask[2] = 0; Already initialized to 0 above
            f.dsp.lf.loop_filter_sb[1][0].call::<BD>(
                u.as_mut_ptr().add(uv_offset + x * 4),
                ls,
                &hmask,
                unaligned_lvl_slice(&lvl[x as usize..], 2),
                b4_stride,
                &f.lf.lim_lut.0,
                endy4 - starty4,
                bd,
            );
            f.dsp.lf.loop_filter_sb[1][0].call::<BD>(
                v.as_mut_ptr().add(uv_offset + x * 4),
                ls,
                &hmask,
                unaligned_lvl_slice(&lvl[x as usize..], 3),
                b4_stride,
                &f.lf.lim_lut.0,
                endy4 - starty4,
                bd,
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
    mask: &[[[AtomicU16; 2]; 2]; 32],
    u: &mut [BD::Pixel],
    v: &mut [BD::Pixel],
    uv_offset: usize,
    ls: ptrdiff_t,
    w: c_int,
    starty4: c_int,
    endy4: c_int,
    ss_hor: c_int,
) {
    let bd = BD::from_c(f.bitdepth_max);
    let mut off_l = uv_offset as ptrdiff_t;

    //                                 block1
    // filter edges between rows (e.g. ------)
    //                                 block2
    for (y, lvl) in (starty4..endy4).zip(lvl.chunks(b4_stride as usize)) {
        if !(!have_top && y == 0) {
            let vmask: [u32; 3] = [
                mask[y as usize][0][0].load(Ordering::Relaxed) as u32
                    | (mask[y as usize][0][1].load(Ordering::Relaxed) as u32) << (16 >> ss_hor),
                mask[y as usize][1][0].load(Ordering::Relaxed) as u32
                    | (mask[y as usize][1][1].load(Ordering::Relaxed) as u32) << (16 >> ss_hor),
                0,
            ];
            f.dsp.lf.loop_filter_sb[1][1].call::<BD>(
                u.as_mut_ptr().offset(off_l),
                ls,
                &vmask,
                unaligned_lvl_slice(&lvl[0..], 2),
                b4_stride,
                &f.lf.lim_lut.0,
                w,
                bd,
            );
            f.dsp.lf.loop_filter_sb[1][1].call::<BD>(
                v.as_mut_ptr().offset(off_l),
                ls,
                &vmask,
                unaligned_lvl_slice(&lvl[0..], 3),
                b4_stride,
                &f.lf.lim_lut.0,
                w,
                bd,
            );
        }
        off_l += 4 * BD::pxstride(ls);
    }
}

pub(crate) unsafe fn rav1d_loopfilter_sbrow_cols<BD: BitDepth>(
    f: &Rav1dFrameData,
    p: &mut [&mut [BD::Pixel]; 3],
    p_offset: &[usize; 2],
    lflvl_offset: usize,
    sby: c_int,
    start_of_tile_row: c_int,
) {
    let lflvl = &f.lf.mask[lflvl_offset..];
    let mut have_left; // Don't filter outside the frame
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
    let mut lpf_y_idx = (sby << sbl2) as usize;
    let mut lpf_uv_idx = (sby << sbl2 - ss_ver) as usize;
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();

    // fix lpf strength at tile col boundaries
    let mut tile_col = 1;
    loop {
        let mut x = frame_hdr.tiling.col_start_sb[tile_col as usize] as c_int;
        if x << sbl2 >= f.bw {
            break;
        }
        let bx4: c_int = if x & is_sb64 != 0 { 16 } else { 0 };
        let cbx4 = bx4 >> ss_hor;
        x >>= is_sb64;
        let y_hmask = &lflvl[x as usize].filter_y[0][bx4 as usize];
        let (lpf_y, lpf_uv) = f.lf.tx_lpf_right_edge.get(
            lpf_y_idx..lpf_y_idx + (endy4 - starty4) as usize,
            lpf_uv_idx..lpf_uv_idx + (uv_endy4 - (starty4 >> ss_ver)) as usize,
        );
        for y in starty4..endy4 {
            let mask: u32 = 1 << y;
            let sidx = (mask >= 0x10000) as usize;
            let smask = (mask >> (sidx << 4)) as u16;
            let idx = 2 * (y_hmask[2][sidx].load(Ordering::Relaxed) & smask != 0) as usize
                + (y_hmask[1][sidx].load(Ordering::Relaxed) & smask != 0) as usize;
            y_hmask[2][sidx].fetch_and(!smask, Ordering::Relaxed);
            y_hmask[1][sidx].fetch_and(!smask, Ordering::Relaxed);
            y_hmask[0][sidx].fetch_and(!smask, Ordering::Relaxed);
            y_hmask[cmp::min(idx, lpf_y[(y - starty4) as usize] as usize)][sidx]
                .fetch_or(smask, Ordering::Relaxed);
        }
        if f.cur.p.layout != Rav1dPixelLayout::I400 {
            let uv_hmask: &[[AtomicU16; 2]; 2] = &lflvl[x as usize].filter_uv[0][cbx4 as usize];
            for y in starty4 >> ss_ver..uv_endy4 {
                let uv_mask: u32 = 1 << y;
                let sidx = (uv_mask >= vmax) as usize;
                let smask = (uv_mask >> (sidx << 4 - ss_ver)) as u16;
                let idx = (uv_hmask[1][sidx].load(Ordering::Relaxed) & smask != 0) as usize;
                uv_hmask[1][sidx].fetch_and(!smask, Ordering::Relaxed);
                uv_hmask[0][sidx].fetch_and(!smask, Ordering::Relaxed);
                uv_hmask[cmp::min(idx, lpf_uv[(y - (starty4 >> ss_ver)) as usize] as usize)][sidx]
                    .fetch_or(smask, Ordering::Relaxed);
            }
        }
        lpf_y_idx += halign;
        lpf_uv_idx += halign >> ss_ver;
        tile_col += 1;
    }

    // fix lpf strength at tile row boundaries
    if start_of_tile_row != 0 {
        let mut a = &f.a[(f.sb128w * (start_of_tile_row - 1)) as usize..];
        for x in 0..f.sb128w {
            let y_vmask = &lflvl[x as usize].filter_y[1][starty4 as usize];
            let w = cmp::min(32, f.w4 - (x << 5)) as u32;
            for i in 0..w {
                let mask: u32 = 1 << i;
                let sidx = (mask >= 0x10000) as usize;
                let smask = (mask >> (sidx << 4)) as u16;
                let idx = 2 * (y_vmask[2][sidx].load(Ordering::Relaxed) & smask != 0) as usize
                    + (y_vmask[1][sidx].load(Ordering::Relaxed) & smask != 0) as usize;
                y_vmask[2][sidx].fetch_and(!smask, Ordering::Relaxed);
                y_vmask[1][sidx].fetch_and(!smask, Ordering::Relaxed);
                y_vmask[0][sidx].fetch_and(!smask, Ordering::Relaxed);
                y_vmask[cmp::min(idx, *a[0].tx_lpf_y.index(i as usize) as usize)][sidx]
                    .fetch_or(smask, Ordering::Relaxed);
            }
            if f.cur.p.layout != Rav1dPixelLayout::I400 {
                let cw: c_uint = w.wrapping_add(ss_hor as c_uint) >> ss_hor;
                let uv_vmask: &[[AtomicU16; 2]; 2] =
                    &lflvl[x as usize].filter_uv[1][(starty4 >> ss_ver) as usize];
                for i in 0..cw {
                    let uv_mask: u32 = 1 << i;
                    let sidx = (uv_mask >= hmax) as usize;
                    let smask = (uv_mask >> (sidx << 4 - ss_hor)) as u16;
                    let idx = (uv_vmask[1][sidx].load(Ordering::Relaxed) & smask != 0) as usize;
                    uv_vmask[1][sidx].fetch_and(!smask, Ordering::Relaxed);
                    uv_vmask[0][sidx].fetch_and(!smask, Ordering::Relaxed);
                    uv_vmask[cmp::min(idx, *a[0].tx_lpf_uv.index(i as usize) as usize)][sidx]
                        .fetch_or(smask, Ordering::Relaxed);
                }
            }
            a = &a[1..];
        }
    }
    let lflvl = &f.lf.mask[lflvl_offset..];
    let level_ptr_guard =
        f.lf.level
            .index((f.b4_stride * sby as isize * sbsz as isize) as usize..);
    let mut level_ptr = &*level_ptr_guard;
    let mut offset = p_offset[0];
    have_left = false;
    for x in 0..f.sb128w {
        filter_plane_cols_y::<BD>(
            f,
            have_left,
            level_ptr,
            f.b4_stride,
            &lflvl[x as usize].filter_y[0],
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
    let level_ptr_guard =
        f.lf.level
            .index((f.b4_stride * (sby * sbsz >> ss_ver) as isize) as usize..);
    let mut level_ptr = &*level_ptr_guard;
    let [_, pu, pv] = p;
    let mut uv_off = p_offset[1];
    have_left = false;
    for x in 0..f.sb128w {
        filter_plane_cols_uv::<BD>(
            f,
            have_left,
            level_ptr,
            f.b4_stride,
            &lflvl[x as usize].filter_uv[0],
            pu,
            pv,
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
    f: &Rav1dFrameData,
    p: &mut [&mut [BD::Pixel]; 3],
    p_offset: &[usize; 2],
    lflvl_offset: usize,
    sby: c_int,
) {
    let lflvl = &f.lf.mask[lflvl_offset..];

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

    let level_ptr_guard =
        f.lf.level
            .index((f.b4_stride * sby as isize * sbsz as isize) as usize..);
    let mut level_ptr = &*level_ptr_guard;
    for x in 0..f.sb128w {
        filter_plane_rows_y::<BD>(
            f,
            have_top,
            level_ptr,
            f.b4_stride,
            &lflvl[x as usize].filter_y[1],
            p[0],
            p_offset[0] + 128 * x as usize,
            f.cur.stride[0],
            cmp::min(32, f.w4 - x * 32),
            starty4,
            endy4 as c_int,
        );
        level_ptr = &level_ptr[32..];
    }

    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    if frame_hdr.loopfilter.level_u == 0 && frame_hdr.loopfilter.level_v == 0 {
        return;
    }

    let mut uv_off: usize = 0;
    let level_ptr_guard =
        f.lf.level
            .index((f.b4_stride * (sby * sbsz >> ss_ver) as isize) as usize..);
    let mut level_ptr = &*level_ptr_guard;
    let [_, pu, pv] = p;
    for x in 0..f.sb128w {
        filter_plane_rows_uv::<BD>(
            f,
            have_top,
            level_ptr,
            f.b4_stride,
            &lflvl[x as usize].filter_uv[1],
            pu,
            pv,
            p_offset[1] + uv_off,
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
