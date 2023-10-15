use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::BPC;
use crate::include::dav1d::headers::Rav1dFilmGrainData;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I400;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::picture::Rav1dPicture;
use crate::src::filmgrain::Rav1dFilmGrainDSPContext;
use libc::intptr_t;
use libc::memcpy;
use libc::memset;
use libc::ptrdiff_t;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_void;

unsafe fn generate_scaling<BD: BitDepth>(
    bitdepth: c_int,
    points: *const [u8; 2],
    num: c_int,
    scaling: *mut u8,
) {
    let (shift_x, scaling_size) = match BD::BPC {
        BPC::BPC8 => (0, 256),
        BPC::BPC16 => {
            if !(bitdepth > 8) {
                unreachable!();
            }
            let shift_x = bitdepth - 8;
            let scaling_size = (1 as c_int) << bitdepth;
            (shift_x, scaling_size)
        }
    };
    if num == 0 {
        memset(scaling as *mut c_void, 0 as c_int, scaling_size as usize);
        return;
    }
    memset(
        scaling as *mut c_void,
        (*points.offset(0))[1] as c_int,
        (((*points.offset(0))[0] as c_int) << shift_x) as usize,
    );
    let mut i = 0;
    while i < num - 1 {
        let bx = (*points.offset(i as isize))[0] as c_int;
        let by = (*points.offset(i as isize))[1] as c_int;
        let ex = (*points.offset((i + 1) as isize))[0] as c_int;
        let ey = (*points.offset((i + 1) as isize))[1] as c_int;
        let dx = ex - bx;
        let dy = ey - by;
        if !(dx > 0) {
            unreachable!();
        }
        let delta = dy * ((0x10000 + (dx >> 1)) / dx);
        let mut x = 0;
        let mut d = 0x8000 as c_int;
        while x < dx {
            *scaling.offset((bx + x << shift_x) as isize) = (by + (d >> 16)) as u8;
            d += delta;
            x += 1;
        }
        i += 1;
    }
    let n = ((*points.offset((num - 1) as isize))[0] as c_int) << shift_x;
    memset(
        &mut *scaling.offset(n as isize) as *mut u8 as *mut c_void,
        (*points.offset((num - 1) as isize))[1] as c_int,
        (scaling_size - n) as usize,
    );

    if BD::BPC != BPC::BPC8 {
        let pad = (1 as c_int) << shift_x;
        let rnd = pad >> 1;
        let mut i_0 = 0;
        while i_0 < num - 1 {
            let bx_0 = ((*points.offset(i_0 as isize))[0] as c_int) << shift_x;
            let ex_0 = ((*points.offset((i_0 + 1) as isize))[0] as c_int) << shift_x;
            let dx_0 = ex_0 - bx_0;
            let mut x_0 = 0;
            while x_0 < dx_0 {
                let range = *scaling.offset((bx_0 + x_0 + pad) as isize) as c_int
                    - *scaling.offset((bx_0 + x_0) as isize) as c_int;
                let mut n_0 = 1;
                let mut r = rnd;
                while n_0 < pad {
                    r += range;
                    *scaling.offset((bx_0 + x_0 + n_0) as isize) =
                        (*scaling.offset((bx_0 + x_0) as isize) as c_int + (r >> shift_x)) as u8;
                    n_0 += 1;
                }
                x_0 += pad;
            }
            i_0 += 1;
        }
    }
}

pub(crate) unsafe fn rav1d_prep_grain<BD: BitDepth>(
    dsp: *const Rav1dFilmGrainDSPContext,
    out: *mut Rav1dPicture,
    in_0: *const Rav1dPicture,
    scaling: *mut BD::Scaling,
    grain_lut: *mut [[BD::Entry; 82]; 74],
) {
    let data: *const Rav1dFilmGrainData = &mut (*(*out).frame_hdr).film_grain.data;
    let bitdepth_max = ((1 as c_int) << (*out).p.bpc) - 1;
    ((*dsp).generate_grain_y).expect("non-null function pointer")(
        (*grain_lut.offset(0)).as_mut_ptr().cast(),
        data,
        bitdepth_max,
    );
    if (*data).num_uv_points[0] != 0 || (*data).chroma_scaling_from_luma != 0 {
        ((*dsp).generate_grain_uv
            [((*in_0).p.layout as c_uint).wrapping_sub(1 as c_int as c_uint) as usize])
            .expect("non-null function pointer")(
            (*grain_lut.offset(1)).as_mut_ptr().cast(),
            (*grain_lut.offset(0)).as_mut_ptr().cast(),
            data,
            0 as c_int as intptr_t,
            bitdepth_max,
        );
    }
    if (*data).num_uv_points[1] != 0 || (*data).chroma_scaling_from_luma != 0 {
        ((*dsp).generate_grain_uv
            [((*in_0).p.layout as c_uint).wrapping_sub(1 as c_int as c_uint) as usize])
            .expect("non-null function pointer")(
            (*grain_lut.offset(2)).as_mut_ptr().cast(),
            (*grain_lut.offset(0)).as_mut_ptr().cast(),
            data,
            1 as c_int as intptr_t,
            bitdepth_max,
        );
    }
    if (*data).num_y_points != 0 || (*data).chroma_scaling_from_luma != 0 {
        generate_scaling::<BD>(
            (*in_0).p.bpc,
            ((*data).y_points).as_ptr(),
            (*data).num_y_points,
            (*scaling.offset(0)).as_mut().as_mut_ptr(),
        );
    }
    if (*data).num_uv_points[0] != 0 {
        generate_scaling::<BD>(
            (*in_0).p.bpc,
            ((*data).uv_points[0]).as_ptr(),
            (*data).num_uv_points[0],
            (*scaling.offset(1)).as_mut().as_mut_ptr(),
        );
    }
    if (*data).num_uv_points[1] != 0 {
        generate_scaling::<BD>(
            (*in_0).p.bpc,
            ((*data).uv_points[1]).as_ptr(),
            (*data).num_uv_points[1],
            (*scaling.offset(2)).as_mut().as_mut_ptr(),
        );
    }
    if !((*out).stride[0] == (*in_0).stride[0]) {
        unreachable!();
    }
    if (*data).num_y_points == 0 {
        let stride: ptrdiff_t = (*out).stride[0];
        let sz: ptrdiff_t = (*out).p.h as isize * stride;
        if sz < 0 {
            memcpy(
                ((*out).data[0] as *mut u8)
                    .offset(sz as isize)
                    .offset(-(stride as isize)) as *mut c_void,
                ((*in_0).data[0] as *mut u8)
                    .offset(sz as isize)
                    .offset(-(stride as isize)) as *const c_void,
                -sz as usize,
            );
        } else {
            memcpy((*out).data[0], (*in_0).data[0], sz as usize);
        }
    }
    if (*in_0).p.layout as c_uint != RAV1D_PIXEL_LAYOUT_I400 as c_int as c_uint
        && (*data).chroma_scaling_from_luma == 0
    {
        if !((*out).stride[1] == (*in_0).stride[1]) {
            unreachable!();
        }
        let ss_ver =
            ((*in_0).p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint) as c_int;
        let stride_0: ptrdiff_t = (*out).stride[1];
        let sz_0: ptrdiff_t = ((*out).p.h + ss_ver >> ss_ver) as isize * stride_0;
        if sz_0 < 0 {
            if (*data).num_uv_points[0] == 0 {
                memcpy(
                    ((*out).data[1] as *mut u8)
                        .offset(sz_0 as isize)
                        .offset(-(stride_0 as isize)) as *mut c_void,
                    ((*in_0).data[1] as *mut u8)
                        .offset(sz_0 as isize)
                        .offset(-(stride_0 as isize)) as *const c_void,
                    -sz_0 as usize,
                );
            }
            if (*data).num_uv_points[1] == 0 {
                memcpy(
                    ((*out).data[2] as *mut u8)
                        .offset(sz_0 as isize)
                        .offset(-(stride_0 as isize)) as *mut c_void,
                    ((*in_0).data[2] as *mut u8)
                        .offset(sz_0 as isize)
                        .offset(-(stride_0 as isize)) as *const c_void,
                    -sz_0 as usize,
                );
            }
        } else {
            if (*data).num_uv_points[0] == 0 {
                memcpy((*out).data[1], (*in_0).data[1], sz_0 as usize);
            }
            if (*data).num_uv_points[1] == 0 {
                memcpy((*out).data[2], (*in_0).data[2], sz_0 as usize);
            }
        }
    }
}
