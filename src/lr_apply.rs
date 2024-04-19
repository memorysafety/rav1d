use crate::include::common::bitdepth::BitDepth;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::include::dav1d::headers::Rav1dRestorationType;
use crate::src::align::Align16;
use crate::src::internal::Rav1dContext;
use crate::src::internal::Rav1dFrameData;
use crate::src::lf_mask::Av1RestorationUnit;
use crate::src::looprestoration::looprestorationfilter_fn;
use crate::src::looprestoration::LooprestorationParams;
use crate::src::looprestoration::LrEdgeFlags;
use crate::src::looprestoration::LR_HAVE_BOTTOM;
use crate::src::looprestoration::LR_HAVE_LEFT;
use crate::src::looprestoration::LR_HAVE_RIGHT;
use crate::src::looprestoration::LR_HAVE_TOP;
use crate::src::tables::dav1d_sgr_params;
use assert_matches::assert_matches;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;

pub type LrRestorePlanes = c_uint;
pub const LR_RESTORE_V: LrRestorePlanes = 4;
pub const LR_RESTORE_U: LrRestorePlanes = 2;
pub const LR_RESTORE_Y: LrRestorePlanes = 1;

unsafe fn lr_stripe<BD: BitDepth>(
    c: &Rav1dContext,
    f: &Rav1dFrameData,
    p: &mut [BD::Pixel],
    mut p_offset: usize,
    left: &[[BD::Pixel; 4]; 128 + 8],
    x: c_int,
    mut y: c_int,
    plane: c_int,
    unit_w: c_int,
    row_h: c_int,
    lr: Av1RestorationUnit,
    mut edges: LrEdgeFlags,
) {
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    let chroma = (plane != 0) as c_int;
    let ss_ver = chroma & (f.sr_cur.p.p.layout == Rav1dPixelLayout::I420) as c_int;
    let stride: ptrdiff_t = f.sr_cur.p.stride[chroma as usize];
    let sby = y + (if y != 0 { 8 << ss_ver } else { 0 }) >> 6 - ss_ver + seq_hdr.sb128;
    let have_tt = (c.tc.len() > 1) as c_int;
    let lpf_stride = BD::pxstride(stride);
    let lr_line_buf = BD::cast_pixel_slice(&f.lf.lr_line_buf);
    let mut lpf_offset = f.lf.lr_lpf_line[plane as usize] as isize;
    lpf_offset += (have_tt * (sby * (4 << seq_hdr.sb128) - 4)) as isize * lpf_stride + x as isize;
    // The first stripe of the frame is shorter by 8 luma pixel rows.
    let mut stripe_h = cmp::min(64 - 8 * (y == 0) as c_int >> ss_ver, row_h - y);
    let lr_fn: looprestorationfilter_fn;
    let mut params: LooprestorationParams = LooprestorationParams {
        filter: [[0; 8]; 2].into(),
    };
    if lr.r#type == Rav1dRestorationType::Wiener {
        let filter = &mut params.filter.0;
        filter[0][0] = lr.filter_h[0] as i16;
        filter[0][1] = lr.filter_h[1] as i16;
        filter[0][2] = lr.filter_h[2] as i16;
        filter[0][6] = lr.filter_h[0] as i16;
        filter[0][5] = lr.filter_h[1] as i16;
        filter[0][4] = lr.filter_h[2] as i16;
        filter[0][3] = -(filter[0][0] + filter[0][1] + filter[0][2]) * 2;
        if BD::BITDEPTH != 8 {
            // For 8-bit SIMD it's beneficial to handle the +128 separately
            // in order to avoid overflows.
            filter[0][3] += 128;
        }

        filter[1][0] = lr.filter_v[0] as i16;
        filter[1][1] = lr.filter_v[1] as i16;
        filter[1][2] = lr.filter_v[2] as i16;
        filter[1][6] = lr.filter_v[0] as i16;
        filter[1][5] = lr.filter_v[1] as i16;
        filter[1][4] = lr.filter_v[2] as i16;
        filter[1][3] = 128 - (filter[1][0] + filter[1][1] + filter[1][2]) * 2;

        lr_fn = f.dsp.lr.wiener[((filter[0][0] | filter[1][0]) == 0) as usize];
    } else {
        let sgr_idx = assert_matches!(lr.r#type, Rav1dRestorationType::SgrProj(idx) => idx);
        let sgr_params = &dav1d_sgr_params[sgr_idx as usize];
        params.sgr.s0 = sgr_params[0] as u32;
        params.sgr.s1 = sgr_params[1] as u32;
        params.sgr.w0 = lr.sgr_weights[0] as i16;
        params.sgr.w1 = 128 - (lr.sgr_weights[0] as i16 + lr.sgr_weights[1] as i16);
        lr_fn = f.dsp.lr.sgr[(sgr_params[0] != 0) as usize + (sgr_params[1] != 0) as usize * 2 - 1];
    }
    let mut left = &left[..];
    while y + stripe_h <= row_h {
        // Change the HAVE_BOTTOM bit in edges to (sby + 1 != f->sbh || y + stripe_h != row_h)
        edges ^= (-((sby + 1 != f.sbh || y + stripe_h != row_h) as c_int) as LrEdgeFlags ^ edges)
            & LR_HAVE_BOTTOM;
        lr_fn(
            p.as_mut_ptr().add(p_offset).cast(),
            stride,
            left.as_ptr().cast(),
            // NOTE: The calculated pointer may point to before the beginning of
            // `lr_line_buf`, so we must use `.wrapping_offset` here.
            // `.wrapping_offset` is needed since `.offset` requires the pointer is in bounds,
            // which `.wrapping_offset` does not, and delays that requirement to when the pointer is dereferenced
            lr_line_buf.as_ptr().wrapping_offset(lpf_offset).cast(),
            unit_w,
            stripe_h,
            &mut params,
            edges,
            f.bitdepth_max,
        );
        left = &left[stripe_h as usize..];
        y += stripe_h;
        p_offset = (p_offset as isize + stripe_h as isize * BD::pxstride(stride)) as usize;
        edges |= LR_HAVE_TOP;
        stripe_h = cmp::min(64 >> ss_ver, row_h - y);
        if stripe_h == 0 {
            break;
        }

        lpf_offset += 4 * lpf_stride;
    }
}

fn backup4xU<BD: BitDepth>(
    dst: &mut [[BD::Pixel; 4]; 128 + 8],
    src: &[BD::Pixel],
    src_offset: usize,
    src_stride: ptrdiff_t,
    u: c_int,
) {
    let abs_px_stride = BD::pxstride(src_stride.unsigned_abs());
    if src_stride < 0 {
        for (src, dst) in src[src_offset - (u - 1) as usize * abs_px_stride..]
            .chunks(abs_px_stride)
            .rev()
            .zip(&mut dst[..u as usize])
        {
            BD::pixel_copy(dst, src, 4);
        }
    } else {
        for (src, dst) in src[src_offset..]
            .chunks(abs_px_stride)
            .zip(&mut dst[..u as usize])
        {
            BD::pixel_copy(dst, src, 4);
        }
    }
}

unsafe fn lr_sbrow<BD: BitDepth>(
    c: &Rav1dContext,
    f: &Rav1dFrameData,
    mut p: &mut [BD::Pixel],
    p_offset: usize,
    y: c_int,
    w: c_int,
    h: c_int,
    row_h: c_int,
    plane: c_int,
) {
    let chroma = (plane != 0) as c_int;
    let ss_ver = chroma & (f.sr_cur.p.p.layout == Rav1dPixelLayout::I420) as c_int;
    let ss_hor = chroma & (f.sr_cur.p.p.layout != Rav1dPixelLayout::I444) as c_int;
    let p_stride: ptrdiff_t = f.sr_cur.p.stride[chroma as usize];
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    let unit_size_log2 = frame_hdr.restoration.unit_size[(plane != 0) as usize];
    let unit_size = (1 as c_int) << unit_size_log2;
    let half_unit_size = unit_size >> 1;
    let max_unit_size = unit_size + half_unit_size;

    // Y coordinate of the sbrow (y is 8 luma pixel rows above row_y)
    let row_y = y + (8 >> ss_ver) * (y != 0) as c_int;

    // FIXME This is an ugly hack to lookup the proper AV1Filter unit for
    // chroma planes. Question: For Multithreaded decoding, is it better
    // to store the chroma LR information with collocated Luma information?
    // In other words. For a chroma restoration unit locate at 128,128 and
    // with a 4:2:0 chroma subsampling, do we store the filter information at
    // the AV1Filter unit located at (128,128) or (256,256)
    // TODO Support chroma subsampling.
    let shift_hor = 7 - ss_hor;

    // maximum sbrow height is 128 + 8 rows offset
    let mut pre_lr_border: Align16<[[[BD::Pixel; 4]; 128 + 8]; 2]> =
        Align16([[[0.into(); 4]; 128 + 8]; 2]);
    let mut lr = [Av1RestorationUnit::default(); 2];
    let mut edges: LrEdgeFlags = (if y > 0 { LR_HAVE_TOP } else { 0 }) | LR_HAVE_RIGHT;
    let mut aligned_unit_pos = row_y & !(unit_size - 1);
    if aligned_unit_pos != 0 && aligned_unit_pos + half_unit_size > h {
        aligned_unit_pos -= unit_size;
    }
    aligned_unit_pos <<= ss_ver;
    let sb_idx = (aligned_unit_pos >> 7) * f.sr_sb128w;
    let unit_idx = (aligned_unit_pos >> 6 & 1) << 1;
    lr[0] = *f.lf.lr_mask[sb_idx as usize].lr[plane as usize][unit_idx as usize]
        .try_read()
        .unwrap();
    let mut restore = lr[0].r#type != Rav1dRestorationType::None;
    let mut x = 0;
    let mut bit = false;
    while x + max_unit_size <= w {
        let next_x = x + unit_size;
        let next_u_idx = unit_idx + (next_x >> shift_hor - 1 & 1);
        lr[!bit as usize] = *f.lf.lr_mask[(sb_idx + (next_x >> shift_hor)) as usize].lr
            [plane as usize][next_u_idx as usize]
            .try_read()
            .unwrap();
        let restore_next = lr[!bit as usize].r#type != Rav1dRestorationType::None;
        if restore_next {
            backup4xU::<BD>(
                &mut pre_lr_border[bit as usize],
                &p[unit_size as usize - 4..],
                p_offset,
                p_stride,
                row_h - y,
            );
        }
        if restore {
            lr_stripe::<BD>(
                c,
                f,
                p,
                p_offset,
                &pre_lr_border[!bit as usize],
                x,
                y,
                plane,
                unit_size,
                row_h,
                lr[bit as usize],
                edges,
            );
        }
        x = next_x;
        restore = restore_next;
        p = &mut p[unit_size as usize..];
        edges |= LR_HAVE_LEFT;
        bit = !bit;
    }
    if restore {
        edges &= !LR_HAVE_RIGHT;
        let unit_w = w - x;
        lr_stripe::<BD>(
            c,
            f,
            p,
            p_offset,
            &pre_lr_border[!bit as usize],
            x,
            y,
            plane,
            unit_w,
            row_h,
            lr[bit as usize],
            edges,
        );
    }
}

pub(crate) unsafe fn rav1d_lr_sbrow<BD: BitDepth>(
    c: &Rav1dContext,
    f: &mut Rav1dFrameData,
    dst: &mut [&mut [BD::Pixel]; 3],
    dst_offset: &[usize; 2],
    sby: c_int,
) {
    let offset_y = 8 * (sby != 0) as c_int;
    let dst_stride = &f.sr_cur.p.stride;
    let restore_planes = f.lf.restore_planes;
    let not_last = ((sby + 1) < f.sbh) as c_int;
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    if restore_planes & LR_RESTORE_Y as c_int != 0 {
        let h = f.sr_cur.p.p.h;
        let w = f.sr_cur.p.p.w;
        let next_row_y = (sby + 1) << 6 + seq_hdr.sb128;
        let row_h = cmp::min(next_row_y - 8 * not_last, h);
        let y_stripe = (sby << 6 + seq_hdr.sb128) - offset_y;
        lr_sbrow::<BD>(
            c,
            f,
            dst[0],
            (dst_offset[0] as isize - offset_y as isize * BD::pxstride(dst_stride[0])) as usize,
            y_stripe,
            w,
            h,
            row_h,
            0,
        );
    }
    if restore_planes & (LR_RESTORE_U | LR_RESTORE_V) as c_int != 0 {
        let ss_ver = (f.sr_cur.p.p.layout == Rav1dPixelLayout::I420) as c_int;
        let ss_hor = (f.sr_cur.p.p.layout != Rav1dPixelLayout::I444) as c_int;
        let h = f.sr_cur.p.p.h + ss_ver >> ss_ver;
        let w = f.sr_cur.p.p.w + ss_hor >> ss_hor;
        let next_row_y = (sby + 1) << 6 - ss_ver + seq_hdr.sb128;
        let row_h = cmp::min(next_row_y - (8 >> ss_ver) * not_last, h);
        let offset_uv = offset_y >> ss_ver;
        let y_stripe = (sby << 6 - ss_ver + seq_hdr.sb128) - offset_uv;
        if restore_planes & LR_RESTORE_U as c_int != 0 {
            lr_sbrow::<BD>(
                c,
                f,
                dst[1],
                (dst_offset[1] as isize - offset_uv as isize * BD::pxstride(dst_stride[1]))
                    as usize,
                y_stripe,
                w,
                h,
                row_h,
                1,
            );
        }
        if restore_planes & LR_RESTORE_V as c_int != 0 {
            lr_sbrow::<BD>(
                c,
                f,
                dst[2],
                (dst_offset[1] as isize - offset_uv as isize * BD::pxstride(dst_stride[1]))
                    as usize,
                y_stripe,
                w,
                h,
                row_h,
                2,
            );
        }
    }
}
