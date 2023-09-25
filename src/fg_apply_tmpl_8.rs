use crate::include::dav1d::headers::Dav1dFilmGrainData;
use crate::include::dav1d::headers::DAV1D_MC_IDENTITY;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I400;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;
use crate::include::dav1d::picture::Rav1dPicture;
use crate::src::align::Align16;
use crate::src::filmgrain::Rav1dFilmGrainDSPContext;
use cfg_if::cfg_if;
use libc::intptr_t;
use libc::memcpy;
use libc::memset;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_void;

pub type pixel = u8;
pub type entry = i8;

unsafe extern "C" fn generate_scaling(
    _bitdepth: c_int,
    points: *const [u8; 2],
    num: c_int,
    scaling: *mut u8,
) {
    let shift_x = 0;
    let scaling_size = 256;
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
}

pub(crate) unsafe fn rav1d_prep_grain_8bpc(
    dsp: *const Rav1dFilmGrainDSPContext,
    out: *mut Rav1dPicture,
    in_0: *const Rav1dPicture,
    scaling: *mut [u8; 256],
    grain_lut: *mut [[entry; 82]; 74],
) {
    let data: *const Dav1dFilmGrainData = &mut (*(*out).frame_hdr).film_grain.data;
    ((*dsp).generate_grain_y).expect("non-null function pointer")(
        (*grain_lut.offset(0)).as_mut_ptr().cast(),
        data,
        8,
    );
    if (*data).num_uv_points[0] != 0 || (*data).chroma_scaling_from_luma != 0 {
        ((*dsp).generate_grain_uv
            [((*in_0).p.layout as c_uint).wrapping_sub(1 as c_int as c_uint) as usize])
            .expect("non-null function pointer")(
            (*grain_lut.offset(1)).as_mut_ptr().cast(),
            (*grain_lut.offset(0)).as_mut_ptr().cast(),
            data,
            0 as c_int as intptr_t,
            8,
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
            8,
        );
    }
    if (*data).num_y_points != 0 || (*data).chroma_scaling_from_luma != 0 {
        generate_scaling(
            (*in_0).p.bpc,
            ((*data).y_points).as_ptr(),
            (*data).num_y_points,
            (*scaling.offset(0)).as_mut_ptr(),
        );
    }
    if (*data).num_uv_points[0] != 0 {
        generate_scaling(
            (*in_0).p.bpc,
            ((*data).uv_points[0]).as_ptr(),
            (*data).num_uv_points[0],
            (*scaling.offset(1)).as_mut_ptr(),
        );
    }
    if (*data).num_uv_points[1] != 0 {
        generate_scaling(
            (*in_0).p.bpc,
            ((*data).uv_points[1]).as_ptr(),
            (*data).num_uv_points[1],
            (*scaling.offset(2)).as_mut_ptr(),
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
    if (*in_0).p.layout as c_uint != DAV1D_PIXEL_LAYOUT_I400 as c_int as c_uint
        && (*data).chroma_scaling_from_luma == 0
    {
        if !((*out).stride[1] == (*in_0).stride[1]) {
            unreachable!();
        }
        let ss_ver =
            ((*in_0).p.layout as c_uint == DAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint) as c_int;
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

pub(crate) unsafe fn rav1d_apply_grain_row_8bpc(
    dsp: *const Rav1dFilmGrainDSPContext,
    out: *mut Rav1dPicture,
    in_0: *const Rav1dPicture,
    scaling: *const [u8; 256],
    grain_lut: *const [[entry; 82]; 74],
    row: c_int,
) {
    let data: *const Dav1dFilmGrainData = &mut (*(*out).frame_hdr).film_grain.data;
    let ss_y = ((*in_0).p.layout as c_uint == DAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint) as c_int;
    let ss_x = ((*in_0).p.layout as c_uint != DAV1D_PIXEL_LAYOUT_I444 as c_int as c_uint) as c_int;
    let cpw = (*out).p.w + ss_x >> ss_x;
    let is_id = ((*(*out).seq_hdr).mtrx as c_uint == DAV1D_MC_IDENTITY as c_int as c_uint) as c_int;
    let luma_src: *mut pixel =
        ((*in_0).data[0] as *mut pixel).offset(((row * 32) as isize * (*in_0).stride[0]) as isize);
    if (*data).num_y_points != 0 {
        let bh = cmp::min((*out).p.h - row * 32, 32 as c_int);
        ((*dsp).fgy_32x32xn).expect("non-null function pointer")(
            ((*out).data[0] as *mut pixel)
                .offset(((row * 32) as isize * (*out).stride[0]) as isize)
                .cast(),
            luma_src.cast(),
            (*out).stride[0],
            data,
            (*out).p.w as usize,
            (*scaling.offset(0)).as_ptr(),
            (*grain_lut.offset(0)).as_ptr().cast(),
            bh,
            row,
            8,
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
            ptr = ptr.offset(((*in_0).stride[0] << ss_y) as isize);
            y += 1;
        }
    }
    let uv_off: ptrdiff_t = (row * 32) as isize * (*out).stride[1] >> ss_y;
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
                8,
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
                    8,
                );
            }
            pl_0 += 1;
        }
    };
}

pub(crate) unsafe fn rav1d_apply_grain_8bpc(
    dsp: *const Rav1dFilmGrainDSPContext,
    out: *mut Rav1dPicture,
    in_0: *const Rav1dPicture,
) {
    let mut grain_lut = Align16([[[0; 82]; 74]; 3]);
    cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            use crate::src::align::Align64;

            let mut scaling = Align64([[0; 256]; 3]);
        } else {
            use crate::src::align::Align1;

            let mut scaling = Align1([[0; 256]; 3]);
        }
    }
    let rows = (*out).p.h + 31 >> 5;
    rav1d_prep_grain_8bpc(
        dsp,
        out,
        in_0,
        scaling.0.as_mut_ptr(),
        grain_lut.0.as_mut_ptr(),
    );
    let mut row = 0;
    while row < rows {
        rav1d_apply_grain_row_8bpc(
            dsp,
            out,
            in_0,
            scaling.0.as_mut_ptr() as *const [u8; 256],
            grain_lut.0.as_mut_ptr() as *const [[entry; 82]; 74],
            row,
        );
        row += 1;
    }
}
