use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::BPC;
use crate::include::dav1d::headers::RAV1D_MC_IDENTITY;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I400;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I444;
use crate::include::dav1d::picture::Rav1dPicture;
use crate::src::filmgrain::Rav1dFilmGrainDSPContext;
use crate::src::internal::GrainBD;
use libc::memcpy;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_void;

unsafe fn generate_scaling<BD: BitDepth>(bitdepth: c_int, points: &[[u8; 2]], scaling: &mut [u8]) {
    let (shift_x, scaling_size) = match BD::BPC {
        BPC::BPC8 => (0, 256),
        BPC::BPC16 => {
            assert!(bitdepth > 8);
            let shift_x = bitdepth - 8;
            let scaling_size = 1 << bitdepth;
            (shift_x, scaling_size)
        }
    };
    if points.is_empty() {
        scaling[..scaling_size as usize].fill(0);
        return;
    }
    scaling[..((points[0][0] as c_int) << shift_x) as usize].fill(points[0][1]);
    for ps in points.windows(2) {
        // TODO(kkysen) use array_windows when stabilized
        let [p0, p1] = ps.try_into().unwrap();
        let bx = p0[0] as c_int;
        let by = p0[1] as c_int;
        let ex = p1[0] as c_int;
        let ey = p1[1] as c_int;
        let dx = ex - bx;
        let dy = ey - by;
        assert!(dx > 0);
        let delta = dy * ((0x10000 + (dx >> 1)) / dx);
        let mut d = 0x8000;
        for x in 0..dx {
            scaling[(bx + x << shift_x) as usize] = (by + (d >> 16)) as u8;
            d += delta;
        }
    }
    let n = (points[points.len() - 1][0] as c_int) << shift_x;
    scaling[n as usize..][..(scaling_size - n) as usize].fill(points[points.len() - 1][1]);

    if BD::BPC != BPC::BPC8 {
        let pad = 1 << shift_x;
        let rnd = pad >> 1;
        for ps in points.windows(2) {
            // TODO(kkysen) use array_windows when stabilized
            let [p0, p1] = ps.try_into().unwrap();
            let bx = (p0[0] as c_int) << shift_x;
            let ex = (p1[0] as c_int) << shift_x;
            let dx = ex - bx;
            for x in (0..dx).step_by(pad as usize) {
                let range =
                    scaling[(bx + x + pad) as usize] as c_int - scaling[(bx + x) as usize] as c_int;
                let mut r = rnd;
                for n in 1..pad {
                    r += range;
                    scaling[(bx + x + n) as usize] =
                        (scaling[(bx + x) as usize] as c_int + (r >> shift_x)) as u8;
                }
            }
        }
    }
}

pub(crate) unsafe fn rav1d_prep_grain<BD: BitDepth>(
    dsp: &Rav1dFilmGrainDSPContext,
    out: &mut Rav1dPicture,
    r#in: &Rav1dPicture,
    grain: &mut GrainBD<BD>,
) {
    let GrainBD { grain_lut, scaling } = grain;
    let data = &mut (*out.frame_hdr).film_grain.data;
    let bitdepth_max = (1 << out.p.bpc) - 1;
    (dsp.generate_grain_y).expect("non-null function pointer")(
        grain_lut[0].as_mut_ptr().cast(),
        data,
        bitdepth_max,
    );
    if data.num_uv_points[0] != 0 || data.chroma_scaling_from_luma != 0 {
        (dsp.generate_grain_uv[r#in.p.layout as usize - 1]).expect("non-null function pointer")(
            grain_lut[1].as_mut_ptr().cast(),
            grain_lut[0].as_mut_ptr().cast(),
            data,
            0,
            bitdepth_max,
        );
    }
    if data.num_uv_points[1] != 0 || data.chroma_scaling_from_luma != 0 {
        (dsp.generate_grain_uv[r#in.p.layout as usize - 1]).expect("non-null function pointer")(
            grain_lut[2].as_mut_ptr().cast(),
            grain_lut[0].as_mut_ptr().cast(),
            data,
            1,
            bitdepth_max,
        );
    }
    if data.num_y_points != 0 || data.chroma_scaling_from_luma != 0 {
        generate_scaling::<BD>(
            r#in.p.bpc,
            &data.y_points[..data.num_y_points as usize],
            scaling[0].as_mut(),
        );
    }
    if data.num_uv_points[0] != 0 {
        generate_scaling::<BD>(
            r#in.p.bpc,
            &data.uv_points[0][..data.num_uv_points[0] as usize],
            scaling[1].as_mut(),
        );
    }
    if data.num_uv_points[1] != 0 {
        generate_scaling::<BD>(
            r#in.p.bpc,
            &data.uv_points[1][..data.num_uv_points[1] as usize],
            scaling[2].as_mut(),
        );
    }
    assert!(out.stride[0] == r#in.stride[0]);
    if data.num_y_points == 0 {
        let stride = out.stride[0];
        let sz = out.p.h as isize * stride;
        if sz < 0 {
            memcpy(
                (out.data[0] as *mut u8)
                    .offset(sz as isize)
                    .offset(-(stride as isize)) as *mut c_void,
                (r#in.data[0] as *mut u8)
                    .offset(sz as isize)
                    .offset(-(stride as isize)) as *const c_void,
                -sz as usize,
            );
        } else {
            memcpy(out.data[0], r#in.data[0], sz as usize);
        }
    }
    if r#in.p.layout != RAV1D_PIXEL_LAYOUT_I400 && data.chroma_scaling_from_luma == 0 {
        assert!(out.stride[1] == r#in.stride[1]);
        let ss_ver = (r#in.p.layout == RAV1D_PIXEL_LAYOUT_I420) as c_int;
        let stride = out.stride[1];
        let sz = (out.p.h + ss_ver >> ss_ver) as isize * stride;
        if sz < 0 {
            if data.num_uv_points[0] == 0 {
                memcpy(
                    (out.data[1] as *mut u8)
                        .offset(sz as isize)
                        .offset(-(stride as isize)) as *mut c_void,
                    (r#in.data[1] as *mut u8)
                        .offset(sz as isize)
                        .offset(-(stride as isize)) as *const c_void,
                    -sz as usize,
                );
            }
            if data.num_uv_points[1] == 0 {
                memcpy(
                    (out.data[2] as *mut u8)
                        .offset(sz as isize)
                        .offset(-(stride as isize)) as *mut c_void,
                    (r#in.data[2] as *mut u8)
                        .offset(sz as isize)
                        .offset(-(stride as isize)) as *const c_void,
                    -sz as usize,
                );
            }
        } else {
            if data.num_uv_points[0] == 0 {
                memcpy(out.data[1], r#in.data[1], sz as usize);
            }
            if data.num_uv_points[1] == 0 {
                memcpy(out.data[2], r#in.data[2], sz as usize);
            }
        }
    }
}

pub(crate) unsafe fn rav1d_apply_grain_row<BD: BitDepth>(
    dsp: &Rav1dFilmGrainDSPContext,
    out: &mut Rav1dPicture,
    r#in: &Rav1dPicture,
    grain: &GrainBD<BD>,
    row: c_int,
) {
    let GrainBD { grain_lut, scaling } = grain;
    let data = &mut (*out.frame_hdr).film_grain.data;
    let ss_y = (r#in.p.layout == RAV1D_PIXEL_LAYOUT_I420) as c_int;
    let ss_x = (r#in.p.layout != RAV1D_PIXEL_LAYOUT_I444) as c_int;
    let cpw = out.p.w + ss_x >> ss_x;
    let is_id = ((*out.seq_hdr).mtrx == RAV1D_MC_IDENTITY) as c_int;
    let luma_src = (r#in.data[0] as *mut BD::Pixel)
        .offset(((row * 32) as isize * BD::pxstride(r#in.stride[0] as usize) as isize) as isize);
    let bitdepth_max = (1 << out.p.bpc) - 1;
    if data.num_y_points != 0 {
        let bh = cmp::min(out.p.h - row * 32, 32);
        (dsp.fgy_32x32xn).expect("non-null function pointer")(
            (out.data[0] as *mut BD::Pixel)
                .offset(
                    ((row * 32) as isize * BD::pxstride(out.stride[0] as usize) as isize) as isize,
                )
                .cast(),
            luma_src.cast(),
            out.stride[0],
            data,
            out.p.w as usize,
            scaling[0].as_ref().as_ptr(),
            grain_lut[0].as_ptr().cast(),
            bh,
            row,
            bitdepth_max,
        );
    }
    if data.num_uv_points[0] == 0
        && data.num_uv_points[1] == 0
        && data.chroma_scaling_from_luma == 0
    {
        return;
    }
    let bh = cmp::min(out.p.h - row * 32, 32) + ss_y >> ss_y;
    if out.p.w & ss_x != 0 {
        let mut ptr = luma_src;
        for _ in 0..bh {
            *ptr.offset(out.p.w as isize) = *ptr.offset((out.p.w - 1) as isize);
            ptr = ptr.offset(((BD::pxstride(r#in.stride[0] as usize) as isize) << ss_y) as isize);
        }
    }
    let uv_off = (row * 32) as isize * BD::pxstride(out.stride[1] as usize) as isize >> ss_y;
    if data.chroma_scaling_from_luma != 0 {
        for pl in 0..2 {
            (dsp.fguv_32x32xn[r#in.p.layout as usize - 1]).expect("non-null function pointer")(
                (out.data[(1 + pl) as usize] as *mut BD::Pixel)
                    .offset(uv_off as isize)
                    .cast(),
                (r#in.data[(1 + pl) as usize] as *const BD::Pixel)
                    .offset(uv_off as isize)
                    .cast(),
                r#in.stride[1],
                data,
                cpw as usize,
                scaling[0].as_ref().as_ptr(),
                grain_lut[(1 + pl) as usize].as_ptr().cast(),
                bh,
                row,
                luma_src.cast(),
                r#in.stride[0],
                pl,
                is_id,
                bitdepth_max,
            );
        }
    } else {
        for pl in 0..2 {
            if data.num_uv_points[pl as usize] != 0 {
                (dsp.fguv_32x32xn[r#in.p.layout as usize - 1]).expect("non-null function pointer")(
                    (out.data[(1 + pl) as usize] as *mut BD::Pixel)
                        .offset(uv_off as isize)
                        .cast(),
                    (r#in.data[(1 + pl) as usize] as *const BD::Pixel)
                        .offset(uv_off as isize)
                        .cast(),
                    r#in.stride[1],
                    data,
                    cpw as usize,
                    scaling[(1 + pl) as usize].as_ref().as_ptr(),
                    grain_lut[(1 + pl) as usize].as_ptr().cast(),
                    bh,
                    row,
                    luma_src.cast(),
                    r#in.stride[0],
                    pl,
                    is_id,
                    bitdepth_max,
                );
            }
        }
    };
}

pub(crate) unsafe fn rav1d_apply_grain<BD: BitDepth>(
    dsp: &Rav1dFilmGrainDSPContext,
    out: &mut Rav1dPicture,
    r#in: &Rav1dPicture,
) {
    let mut grain = Default::default();
    let rows = out.p.h + 31 >> 5;
    rav1d_prep_grain::<BD>(dsp, out, r#in, &mut grain);
    for row in 0..rows {
        rav1d_apply_grain_row::<BD>(dsp, out, r#in, &grain, row);
    }
}
