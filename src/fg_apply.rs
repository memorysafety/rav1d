#![deny(unsafe_code)]

use crate::align::ArrayDefault;
use crate::filmgrain::Rav1dFilmGrainDSPContext;
use crate::filmgrain::FG_BLOCK_SIZE;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::BPC;
use crate::include::dav1d::headers::Rav1dMatrixCoefficients;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::include::dav1d::picture::Rav1dPicture;
use crate::internal::GrainBD;
use crate::strided::Strided as _;
use std::cmp;

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

pub(crate) fn rav1d_prep_grain<BD: BitDepth>(
    dsp: &Rav1dFilmGrainDSPContext,
    out: &mut Rav1dPicture,
    r#in: &Rav1dPicture,
    grain: &mut GrainBD<BD>,
) {
    let GrainBD { grain_lut, scaling } = grain;
    let frame_hdr = &***out.frame_hdr.as_ref().unwrap();
    let data = &frame_hdr.film_grain.data;
    let bitdepth_max = (1 << out.p.bpc) - 1;
    let bd = BD::from_c(bitdepth_max);
    let layout = || r#in.p.layout.try_into().unwrap();

    // Generate grain LUTs as needed
    let [grain_lut_0, grain_lut_1, grain_lut_2] = &mut grain_lut.0;
    dsp.generate_grain_y.call(grain_lut_0, data, bd);
    if data.num_uv_points[0] != 0 || data.chroma_scaling_from_luma {
        dsp.generate_grain_uv[layout()].call(grain_lut_1, grain_lut_0, data, false, bd);
    }
    if data.num_uv_points[1] != 0 || data.chroma_scaling_from_luma {
        dsp.generate_grain_uv[layout()].call(grain_lut_2, grain_lut_0, data, true, bd);
    }

    // Generate scaling LUTs as needed
    let bd = BD::from_c((1 << r#in.p.bpc) - 1);
    scaling[0] = generate_scaling::<BD>(bd, &data.y_points[..data.num_y_points as usize]);
    scaling[1] = generate_scaling::<BD>(bd, &data.uv_points[0][..data.num_uv_points[0] as usize]);
    scaling[2] = generate_scaling::<BD>(bd, &data.uv_points[1][..data.num_uv_points[1] as usize]);

    // Copy over the non-modified planes
    // TODO: eliminate in favor of per-plane refs
    assert!(out.stride[0] == r#in.stride[0]);
    let has_chroma = r#in.p.layout != Rav1dPixelLayout::I400 && !data.chroma_scaling_from_luma;
    if has_chroma {
        assert!(out.stride[1] == r#in.stride[1]);
    }
    let num_points = [
        data.num_y_points,
        data.num_uv_points[0],
        data.num_uv_points[1],
    ];
    let [in_data, out_data] = [r#in, out].map(|p| &p.data.as_ref().unwrap().data);
    for i in 0..3 {
        if (i == 0 || has_chroma) && num_points[i] == 0 {
            out_data[i].copy_from(&in_data[i]);
        }
    }
}

pub(crate) fn rav1d_apply_grain_row<BD: BitDepth>(
    dsp: &Rav1dFilmGrainDSPContext,
    out: &Rav1dPicture,
    r#in: &Rav1dPicture,
    grain: &GrainBD<BD>,
    row: usize,
) {
    // Synthesize grain for the affected planes
    let GrainBD { grain_lut, scaling } = grain;
    let seq_hdr = &***out.seq_hdr.as_ref().unwrap();
    let frame_hdr = &***out.frame_hdr.as_ref().unwrap();
    let data = &frame_hdr.film_grain.data;
    let in_data = &r#in.data.as_ref().unwrap().data;
    let out_data = &out.data.as_ref().unwrap().data;
    let w = out.p.w as usize;
    let h = out.p.h as usize;

    let ss_y = (r#in.p.layout == Rav1dPixelLayout::I420) as usize;
    let ss_x = (r#in.p.layout != Rav1dPixelLayout::I444) as usize;
    let cpw = w + ss_x >> ss_x;
    let is_id = seq_hdr.mtrx == Rav1dMatrixCoefficients::IDENTITY;
    let bitdepth_max = (1 << out.p.bpc) - 1;
    let bd = BD::from_c(bitdepth_max);

    if data.num_y_points != 0 {
        let bh = cmp::min(h - row * FG_BLOCK_SIZE, FG_BLOCK_SIZE);
        dsp.fgy_32x32xn.call(
            &out_data[0],
            &in_data[0],
            data,
            w,
            &scaling[0],
            &grain_lut[0],
            bh,
            row,
            bd,
        );
    }

    if data.num_uv_points[0] == 0 && data.num_uv_points[1] == 0 && !data.chroma_scaling_from_luma {
        return;
    }

    let bh = cmp::min(h - row * FG_BLOCK_SIZE, FG_BLOCK_SIZE) + ss_y >> ss_y;

    // extend padding pixels
    if out.p.w as usize & ss_x != 0 {
        let luma = in_data[0].with_offset::<BD>();
        let luma = luma + (row * FG_BLOCK_SIZE) as isize * luma.pixel_stride::<BD>();
        for y in 0..bh {
            let luma = luma + (y as isize * (luma.pixel_stride::<BD>() << ss_y));
            let padding = &mut *(luma + (out.p.w as usize - 1)).slice_mut::<BD>(2);
            padding[1] = padding[0];
        }
    }

    let layout = r#in.p.layout.try_into().unwrap();
    if data.chroma_scaling_from_luma {
        for pl in 0..2 {
            dsp.fguv_32x32xn[layout].call(
                layout,
                &out_data[1 + pl],
                &in_data[1 + pl],
                data,
                cpw,
                &scaling[0],
                &grain_lut[1 + pl],
                bh,
                row,
                &in_data[0],
                pl != 0,
                is_id,
                bd,
            );
        }
    } else {
        for pl in 0..2 {
            if data.num_uv_points[pl] != 0 {
                dsp.fguv_32x32xn[layout].call(
                    layout,
                    &out_data[1 + pl],
                    &in_data[1 + pl],
                    data,
                    cpw,
                    &scaling[1 + pl],
                    &grain_lut[1 + pl],
                    bh,
                    row,
                    &in_data[0],
                    pl != 0,
                    is_id,
                    bd,
                );
            }
        }
    }
}

pub(crate) fn rav1d_apply_grain<BD: BitDepth>(
    dsp: &Rav1dFilmGrainDSPContext,
    out: &mut Rav1dPicture,
    r#in: &Rav1dPicture,
) {
    let mut grain = Default::default();
    let rows = (out.p.h as usize + FG_BLOCK_SIZE - 1) / FG_BLOCK_SIZE;

    rav1d_prep_grain::<BD>(dsp, out, r#in, &mut grain);
    for row in 0..rows {
        rav1d_apply_grain_row::<BD>(dsp, out, r#in, &grain, row);
    }
}
