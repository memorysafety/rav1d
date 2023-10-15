use crate::include::common::bitdepth::BitDepth16;
use crate::include::dav1d::headers::Dav1dFilmGrainData;
use crate::include::dav1d::headers::RAV1D_MC_IDENTITY;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I444;
use crate::include::dav1d::picture::Rav1dPicture;
use crate::src::align::Align1;
use crate::src::align::Align16;
use crate::src::fg_apply::rav1d_prep_grain;
use crate::src::filmgrain::Rav1dFilmGrainDSPContext;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;

pub type pixel = u16;
pub type entry = i16;

#[inline]
unsafe fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 != 0 {
        unreachable!();
    }
    return x >> 1;
}

pub(crate) unsafe fn rav1d_apply_grain_row_16bpc(
    dsp: *const Rav1dFilmGrainDSPContext,
    out: *mut Rav1dPicture,
    in_0: *const Rav1dPicture,
    scaling: *const [u8; 4096],
    grain_lut: *const [[entry; 82]; 74],
    row: c_int,
) {
    let data: *const Dav1dFilmGrainData = &mut (*(*out).frame_hdr).film_grain.data;
    let ss_y = ((*in_0).p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint) as c_int;
    let ss_x = ((*in_0).p.layout as c_uint != RAV1D_PIXEL_LAYOUT_I444 as c_int as c_uint) as c_int;
    let cpw = (*out).p.w + ss_x >> ss_x;
    let is_id = ((*(*out).seq_hdr).mtrx as c_uint == RAV1D_MC_IDENTITY as c_int as c_uint) as c_int;
    let luma_src: *mut pixel = ((*in_0).data[0] as *mut pixel)
        .offset(((row * 32) as isize * PXSTRIDE((*in_0).stride[0])) as isize);
    let bitdepth_max = ((1 as c_int) << (*out).p.bpc) - 1;
    if (*data).num_y_points != 0 {
        let bh = cmp::min((*out).p.h - row * 32, 32 as c_int);
        ((*dsp).fgy_32x32xn).expect("non-null function pointer")(
            ((*out).data[0] as *mut pixel)
                .offset(((row * 32) as isize * PXSTRIDE((*out).stride[0])) as isize)
                .cast(),
            luma_src.cast(),
            (*out).stride[0],
            data,
            (*out).p.w as usize,
            (*scaling.offset(0)).as_ptr(),
            (*grain_lut.offset(0)).as_ptr().cast(),
            bh,
            row,
            bitdepth_max,
        );
    }
    if (*data).num_uv_points[0] == 0
        && (*data).num_uv_points[1] == 0
        && (*data).chroma_scaling_from_luma == 0
    {
        return;
    }
    let bh_0 = cmp::min((*out).p.h - row * 32, 32 as c_int) + ss_y >> ss_y;
    if (*out).p.w & ss_x != 0 {
        let mut ptr: *mut pixel = luma_src;
        let mut y = 0;
        while y < bh_0 {
            *ptr.offset((*out).p.w as isize) = *ptr.offset(((*out).p.w - 1) as isize);
            ptr = ptr.offset((PXSTRIDE((*in_0).stride[0]) << ss_y) as isize);
            y += 1;
        }
    }
    let uv_off: ptrdiff_t = (row * 32) as isize * PXSTRIDE((*out).stride[1]) >> ss_y;
    if (*data).chroma_scaling_from_luma != 0 {
        let mut pl = 0;
        while pl < 2 {
            ((*dsp).fguv_32x32xn
                [((*in_0).p.layout as c_uint).wrapping_sub(1 as c_int as c_uint) as usize])
                .expect("non-null function pointer")(
                ((*out).data[(1 + pl) as usize] as *mut pixel)
                    .offset(uv_off as isize)
                    .cast(),
                ((*in_0).data[(1 + pl) as usize] as *const pixel)
                    .offset(uv_off as isize)
                    .cast(),
                (*in_0).stride[1],
                data,
                cpw as usize,
                (*scaling.offset(0)).as_ptr(),
                (*grain_lut.offset((1 + pl) as isize)).as_ptr().cast(),
                bh_0,
                row,
                luma_src.cast(),
                (*in_0).stride[0],
                pl,
                is_id,
                bitdepth_max,
            );
            pl += 1;
        }
    } else {
        let mut pl_0 = 0;
        while pl_0 < 2 {
            if (*data).num_uv_points[pl_0 as usize] != 0 {
                ((*dsp).fguv_32x32xn
                    [((*in_0).p.layout as c_uint).wrapping_sub(1 as c_int as c_uint) as usize])
                    .expect("non-null function pointer")(
                    ((*out).data[(1 + pl_0) as usize] as *mut pixel)
                        .offset(uv_off as isize)
                        .cast(),
                    ((*in_0).data[(1 + pl_0) as usize] as *const pixel)
                        .offset(uv_off as isize)
                        .cast(),
                    (*in_0).stride[1],
                    data,
                    cpw as usize,
                    (*scaling.offset((1 + pl_0) as isize)).as_ptr(),
                    (*grain_lut.offset((1 + pl_0) as isize)).as_ptr().cast(),
                    bh_0,
                    row,
                    luma_src.cast(),
                    (*in_0).stride[0],
                    pl_0,
                    is_id,
                    bitdepth_max,
                );
            }
            pl_0 += 1;
        }
    };
}

pub(crate) unsafe fn rav1d_apply_grain_16bpc(
    dsp: *const Rav1dFilmGrainDSPContext,
    out: *mut Rav1dPicture,
    in_0: *const Rav1dPicture,
) {
    let mut grain_lut = Align16([[[0; 82]; 74]; 3]);
    let mut scaling = Align1([[0; 4096]; 3]);
    let rows = (*out).p.h + 31 >> 5;
    rav1d_prep_grain::<BitDepth16>(
        dsp,
        out,
        in_0,
        scaling.0.as_mut_ptr(),
        grain_lut.0.as_mut_ptr(),
    );
    let mut row = 0;
    while row < rows {
        rav1d_apply_grain_row_16bpc(
            dsp,
            out,
            in_0,
            scaling.0.as_mut_ptr() as *const [u8; 4096],
            grain_lut.0.as_mut_ptr() as *const [[entry; 82]; 74],
            row,
        );
        row += 1;
    }
}
