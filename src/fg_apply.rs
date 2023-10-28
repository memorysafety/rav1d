use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::BPC;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::include::dav1d::headers::RAV1D_MC_IDENTITY;
use crate::include::dav1d::picture::Rav1dPicture;
use crate::src::align::ArrayDefault;
use crate::src::filmgrain::Rav1dFilmGrainDSPContext;
use crate::src::internal::GrainBD;
use libc::memcpy;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_void;

fn generate_scaling<BD: BitDepth>(bd: BD, points: &[[u8; 2]]) -> BD::Scaling {
    let mut scaling_array = ArrayDefault::default();

    if points.is_empty() {
        return scaling_array;
    }

    let shift_x = bd.bitdepth() - 8;
    let scaling_size = 1 << bd.bitdepth();
    let scaling = scaling_array.as_mut();

    // Fill up the preceding entries with the initial value
    scaling[..(points[0][0] as usize) << shift_x].fill(points[0][1]);

    // Linearly interpolate the values in the middle
    for ps in points.windows(2) {
        // TODO(kkysen) use array_windows when stabilized
        let [p0, p1] = ps.try_into().unwrap();
        let bx = p0[0] as usize;
        let by = p0[1] as isize;
        let ex = p1[0] as usize;
        let ey = p1[1] as isize;
        let dx = ex - bx;
        let dy = ey - by;
        assert!(dx > 0);
        let delta = dy * ((0x10000 + (dx >> 1)) / dx) as isize;
        let mut d = 0x8000;
        for x in 0..dx {
            scaling[bx + x << shift_x] = (by + (d >> 16)) as u8;
            d += delta;
        }
    }

    // Fill up the remaining entries with the final value
    let n = (points[points.len() - 1][0] as usize) << shift_x;
    scaling[n..][..scaling_size - n].fill(points[points.len() - 1][1]);

    if BD::BPC != BPC::BPC8 {
        let pad = 1 << shift_x;
        let rnd = pad >> 1;
        for ps in points.windows(2) {
            // TODO(kkysen) use array_windows when stabilized
            let [p0, p1] = ps.try_into().unwrap();
            let bx = (p0[0] as usize) << shift_x;
            let ex = (p1[0] as usize) << shift_x;
            let dx = ex - bx;
            for x in (0..dx).step_by(pad) {
                let range = scaling[bx + x + pad] as isize - scaling[(bx + x) as usize] as isize;
                let mut r = rnd as isize;
                for n in 1..pad {
                    r += range;
                    scaling[bx + x + n] = (scaling[bx + x] as isize + (r >> shift_x)) as u8;
                }
            }
        }
    }

    scaling_array
}

pub(crate) unsafe fn rav1d_prep_grain<BD: BitDepth>(
    dsp: &Rav1dFilmGrainDSPContext,
    out: &mut Rav1dPicture,
    r#in: &Rav1dPicture,
    grain: &mut GrainBD<BD>,
) {
    let GrainBD { grain_lut, scaling } = grain;
    let data = &(*out.frame_hdr).film_grain.data;
    let bitdepth_max = (1 << out.p.bpc) - 1;
    let bd = BD::from_c(bitdepth_max);

    // Generate grain LUTs as needed
    let [grain_lut_0, grain_lut_1, grain_lut_2] = &mut grain_lut.0;
    dsp.generate_grain_y.call(grain_lut_0, data, bd);
    if data.num_uv_points[0] != 0 || data.chroma_scaling_from_luma {
        dsp.generate_grain_uv[r#in.p.layout.try_into().unwrap()].call(
            grain_lut_1,
            grain_lut_0,
            data,
            false,
            bd,
        );
    }
    if data.num_uv_points[1] != 0 || data.chroma_scaling_from_luma {
        dsp.generate_grain_uv[r#in.p.layout.try_into().unwrap()].call(
            grain_lut_2,
            grain_lut_0,
            data,
            true,
            bd,
        );
    }

    // Generate scaling LUTs as needed
    let bd = BD::from_c((1 << r#in.p.bpc) - 1);
    scaling[0] = generate_scaling::<BD>(bd, &data.y_points[..data.num_y_points as usize]);
    scaling[1] = generate_scaling::<BD>(bd, &data.uv_points[0][..data.num_uv_points[0] as usize]);
    scaling[2] = generate_scaling::<BD>(bd, &data.uv_points[1][..data.num_uv_points[1] as usize]);

    // Copy over the non-modified planes
    // TODO: eliminate in favor of per-plane refs
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

    if r#in.p.layout != Rav1dPixelLayout::I400 && !data.chroma_scaling_from_luma {
        assert!(out.stride[1] == r#in.stride[1]);
        let ss_ver = (r#in.p.layout == Rav1dPixelLayout::I420) as c_int;
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
    // Synthesize grain for the affected planes
    let GrainBD { grain_lut, scaling } = grain;
    let data = &(*out.frame_hdr).film_grain.data;
    let data_c = &data.clone().into();
    let ss_y = (r#in.p.layout == Rav1dPixelLayout::I420) as c_int;
    let ss_x = (r#in.p.layout != Rav1dPixelLayout::I444) as c_int;
    let cpw = out.p.w + ss_x >> ss_x;
    let is_id = ((*out.seq_hdr).mtrx == RAV1D_MC_IDENTITY) as c_int;
    let luma_src = (r#in.data[0] as *mut BD::Pixel)
        .offset(((row * 32) as isize * BD::pxstride(r#in.stride[0] as usize) as isize) as isize);
    let bitdepth_max = (1 << out.p.bpc) - 1;
    let bd = BD::from_c(bitdepth_max);

    if data.num_y_points != 0 {
        let bh = cmp::min(out.p.h - row * 32, 32);
        dsp.fgy_32x32xn.call(
            (out.data[0] as *mut BD::Pixel).offset(
                ((row * 32) as isize * BD::pxstride(out.stride[0] as usize) as isize) as isize,
            ),
            luma_src.cast(),
            out.stride[0],
            data,
            out.p.w as usize,
            &scaling[0],
            &grain_lut[0],
            bh,
            row as usize,
            bd,
        );
    }

    if data.num_uv_points[0] == 0 && data.num_uv_points[1] == 0 && !data.chroma_scaling_from_luma {
        return;
    }

    let bh = cmp::min(out.p.h - row * 32, 32) + ss_y >> ss_y;

    // extend padding pixels
    if out.p.w & ss_x != 0 {
        let mut ptr = luma_src;
        for _ in 0..bh {
            *ptr.offset(out.p.w as isize) = *ptr.offset((out.p.w - 1) as isize);
            ptr = ptr.offset(((BD::pxstride(r#in.stride[0] as usize) as isize) << ss_y) as isize);
        }
    }

    let uv_off = (row * 32) as isize * BD::pxstride(out.stride[1] as usize) as isize >> ss_y;
    if data.chroma_scaling_from_luma {
        for pl in 0..2 {
            dsp.fguv_32x32xn[r#in.p.layout.try_into().unwrap()].call(
                (out.data[1 + pl] as *mut BD::Pixel).offset(uv_off as isize),
                (r#in.data[1 + pl] as *const BD::Pixel).offset(uv_off as isize),
                r#in.stride[1],
                data,
                cpw as usize,
                &scaling[0],
                &grain_lut[1 + pl],
                bh,
                row as usize,
                luma_src,
                r#in.stride[0],
                pl as c_int,
                is_id,
                bd,
            );
        }
    } else {
        for pl in 0..2 {
            if data.num_uv_points[pl] != 0 {
                dsp.fguv_32x32xn[r#in.p.layout.try_into().unwrap()].call(
                    (out.data[1 + pl] as *mut BD::Pixel).offset(uv_off as isize),
                    (r#in.data[1 + pl] as *const BD::Pixel).offset(uv_off as isize),
                    r#in.stride[1],
                    data_c,
                    cpw as usize,
                    &scaling[1 + pl],
                    &grain_lut[1 + pl],
                    bh,
                    row as usize,
                    luma_src,
                    r#in.stride[0],
                    pl as c_int,
                    is_id,
                    bd,
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
