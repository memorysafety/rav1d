use crate::include::stddef::*;
use crate::include::stdint::*;
use crate::src::align::{Align16, Align64};
use ::libc;
extern "C" {
    pub type Dav1dRef;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}

use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;

use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I400;

use crate::include::dav1d::headers::DAV1D_MC_IDENTITY;

use crate::include::dav1d::headers::Dav1dFilmGrainData;
use crate::include::dav1d::picture::Dav1dPicture;
pub type pixel = uint8_t;
pub type entry = int8_t;
pub type generate_grain_y_fn = Option::<
    unsafe extern "C" fn(*mut [entry; 82], *const Dav1dFilmGrainData) -> (),
>;
pub type generate_grain_uv_fn = Option::<
    unsafe extern "C" fn(
        *mut [entry; 82],
        *const [entry; 82],
        *const Dav1dFilmGrainData,
        intptr_t,
    ) -> (),
>;
pub type fgy_32x32xn_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        *const pixel,
        ptrdiff_t,
        *const Dav1dFilmGrainData,
        size_t,
        *const uint8_t,
        *const [entry; 82],
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type fguv_32x32xn_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        *const pixel,
        ptrdiff_t,
        *const Dav1dFilmGrainData,
        size_t,
        *const uint8_t,
        *const [entry; 82],
        libc::c_int,
        libc::c_int,
        *const pixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFilmGrainDSPContext {
    pub generate_grain_y: generate_grain_y_fn,
    pub generate_grain_uv: [generate_grain_uv_fn; 3],
    pub fgy_32x32xn: fgy_32x32xn_fn,
    pub fguv_32x32xn: [fguv_32x32xn_fn; 3],
}
use crate::include::common::intops::imin;
unsafe extern "C" fn generate_scaling(
    _bitdepth: libc::c_int,
    mut points: *const [uint8_t; 2],
    num: libc::c_int,
    mut scaling: *mut uint8_t,
) {
    let shift_x = 0;
    let scaling_size = 256;
    if num == 0 {
        memset(
            scaling as *mut libc::c_void,
            0 as libc::c_int,
            scaling_size as libc::c_ulong,
        );
        return;
    }
    memset(
        scaling as *mut libc::c_void,
        (*points.offset(0))[1]
            as libc::c_int,
        (((*points.offset(0))[0]
            as libc::c_int) << shift_x) as libc::c_ulong,
    );
    let mut i = 0;
    while i < num - 1 {
        let bx: libc::c_int = (*points.offset(i as isize))[0]
            as libc::c_int;
        let by: libc::c_int = (*points.offset(i as isize))[1]
            as libc::c_int;
        let ex: libc::c_int = (*points
            .offset((i + 1) as isize))[0]
            as libc::c_int;
        let ey: libc::c_int = (*points
            .offset((i + 1) as isize))[1]
            as libc::c_int;
        let dx: libc::c_int = ex - bx;
        let dy: libc::c_int = ey - by;
        if !(dx > 0) {
            unreachable!();
        }
        let delta: libc::c_int = dy
            * ((0x10000 as libc::c_int + (dx >> 1)) / dx);
        let mut x = 0;
        let mut d: libc::c_int = 0x8000 as libc::c_int;
        while x < dx {
            *scaling
                .offset(
                    (bx + x << shift_x) as isize,
                ) = (by + (d >> 16)) as uint8_t;
            d += delta;
            x += 1;
        }
        i += 1;
    }
    let n: libc::c_int = ((*points
        .offset((num - 1) as isize))[0]
        as libc::c_int) << shift_x;
    memset(
        &mut *scaling.offset(n as isize) as *mut uint8_t as *mut libc::c_void,
        (*points.offset((num - 1) as isize))[1]
            as libc::c_int,
        (scaling_size - n) as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_prep_grain_8bpc(
    dsp: *const Dav1dFilmGrainDSPContext,
    out: *mut Dav1dPicture,
    in_0: *const Dav1dPicture,
    mut scaling: *mut [uint8_t; 256],
    mut grain_lut: *mut [[entry; 82]; 74],
) {
    let data: *const Dav1dFilmGrainData = &mut (*(*out).frame_hdr).film_grain.data;
    ((*dsp).generate_grain_y)
        .expect(
            "non-null function pointer",
        )((*grain_lut.offset(0)).as_mut_ptr(), data);
    if (*data).num_uv_points[0] != 0
        || (*data).chroma_scaling_from_luma != 0
    {
        ((*dsp)
            .generate_grain_uv[((*in_0).p.layout as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as usize])
            .expect(
                "non-null function pointer",
            )(
            (*grain_lut.offset(1)).as_mut_ptr(),
            (*grain_lut.offset(0)).as_mut_ptr()
                as *const [entry; 82],
            data,
            0 as libc::c_int as intptr_t,
        );
    }
    if (*data).num_uv_points[1] != 0
        || (*data).chroma_scaling_from_luma != 0
    {
        ((*dsp)
            .generate_grain_uv[((*in_0).p.layout as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as usize])
            .expect(
                "non-null function pointer",
            )(
            (*grain_lut.offset(2)).as_mut_ptr(),
            (*grain_lut.offset(0)).as_mut_ptr()
                as *const [entry; 82],
            data,
            1 as libc::c_int as intptr_t,
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
    if !((*out).stride[0]
        == (*in_0).stride[0])
    {
        unreachable!();
    }
    if (*data).num_y_points == 0 {
        let stride: ptrdiff_t = (*out).stride[0];
        let sz: ptrdiff_t = (*out).p.h as isize * stride;
        if sz < 0 {
            memcpy(
                ((*out).data[0] as *mut uint8_t)
                    .offset(sz as isize)
                    .offset(-(stride as isize)) as *mut libc::c_void,
                ((*in_0).data[0] as *mut uint8_t)
                    .offset(sz as isize)
                    .offset(-(stride as isize)) as *const libc::c_void,
                -sz as libc::c_ulong,
            );
        } else {
            memcpy(
                (*out).data[0],
                (*in_0).data[0],
                sz as libc::c_ulong,
            );
        }
    }
    if (*in_0).p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint
        && (*data).chroma_scaling_from_luma == 0
    {
        if !((*out).stride[1]
            == (*in_0).stride[1])
        {
            unreachable!();
        }
        let ss_ver: libc::c_int = ((*in_0).p.layout as libc::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
        let stride_0: ptrdiff_t = (*out).stride[1];
        let sz_0: ptrdiff_t = ((*out).p.h + ss_ver >> ss_ver) as isize * stride_0;
        if sz_0 < 0 {
            if (*data).num_uv_points[0] == 0 {
                memcpy(
                    ((*out).data[1] as *mut uint8_t)
                        .offset(sz_0 as isize)
                        .offset(-(stride_0 as isize)) as *mut libc::c_void,
                    ((*in_0).data[1] as *mut uint8_t)
                        .offset(sz_0 as isize)
                        .offset(-(stride_0 as isize)) as *const libc::c_void,
                    -sz_0 as libc::c_ulong,
                );
            }
            if (*data).num_uv_points[1] == 0 {
                memcpy(
                    ((*out).data[2] as *mut uint8_t)
                        .offset(sz_0 as isize)
                        .offset(-(stride_0 as isize)) as *mut libc::c_void,
                    ((*in_0).data[2] as *mut uint8_t)
                        .offset(sz_0 as isize)
                        .offset(-(stride_0 as isize)) as *const libc::c_void,
                    -sz_0 as libc::c_ulong,
                );
            }
        } else {
            if (*data).num_uv_points[0] == 0 {
                memcpy(
                    (*out).data[1],
                    (*in_0).data[1],
                    sz_0 as libc::c_ulong,
                );
            }
            if (*data).num_uv_points[1] == 0 {
                memcpy(
                    (*out).data[2],
                    (*in_0).data[2],
                    sz_0 as libc::c_ulong,
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_apply_grain_row_8bpc(
    dsp: *const Dav1dFilmGrainDSPContext,
    out: *mut Dav1dPicture,
    in_0: *const Dav1dPicture,
    mut scaling: *const [uint8_t; 256],
    mut grain_lut: *const [[entry; 82]; 74],
    row: libc::c_int,
) {
    let data: *const Dav1dFilmGrainData = &mut (*(*out).frame_hdr).film_grain.data;
    let ss_y: libc::c_int = ((*in_0).p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_x: libc::c_int = ((*in_0).p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
    let cpw: libc::c_int = (*out).p.w + ss_x >> ss_x;
    let is_id: libc::c_int = ((*(*out).seq_hdr).mtrx as libc::c_uint
        == DAV1D_MC_IDENTITY as libc::c_int as libc::c_uint) as libc::c_int;
    let luma_src: *mut pixel = ((*in_0).data[0] as *mut pixel)
        .offset(
            ((row * 32) as isize
                * (*in_0).stride[0]) as isize,
        );
    if (*data).num_y_points != 0 {
        let bh: libc::c_int = imin(
            (*out).p.h - row * 32,
            32 as libc::c_int,
        );
        ((*dsp).fgy_32x32xn)
            .expect(
                "non-null function pointer",
            )(
            ((*out).data[0] as *mut pixel)
                .offset(
                    ((row * 32) as isize
                        * (*out).stride[0]) as isize,
                ),
            luma_src,
            (*out).stride[0],
            data,
            (*out).p.w as size_t,
            (*scaling.offset(0)).as_ptr(),
            (*grain_lut.offset(0)).as_ptr(),
            bh,
            row,
        );
    }
    if (*data).num_uv_points[0] == 0
        && (*data).num_uv_points[1] == 0
        && (*data).chroma_scaling_from_luma == 0
    {
        return;
    }
    let bh_0: libc::c_int = imin((*out).p.h - row * 32, 32 as libc::c_int)
        + ss_y >> ss_y;
    if (*out).p.w & ss_x != 0 {
        let mut ptr: *mut pixel = luma_src;
        let mut y = 0;
        while y < bh_0 {
            *ptr
                .offset(
                    (*out).p.w as isize,
                ) = *ptr.offset(((*out).p.w - 1) as isize);
            ptr = ptr
                .offset(((*in_0).stride[0] << ss_y) as isize);
            y += 1;
        }
    }
    let uv_off: ptrdiff_t = (row * 32) as isize
        * (*out).stride[1] >> ss_y;
    if (*data).chroma_scaling_from_luma != 0 {
        let mut pl = 0;
        while pl < 2 {
            ((*dsp)
                .fguv_32x32xn[((*in_0).p.layout as libc::c_uint)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as usize])
                .expect(
                    "non-null function pointer",
                )(
                ((*out).data[(1 as libc::c_int + pl) as usize] as *mut pixel)
                    .offset(uv_off as isize),
                ((*in_0).data[(1 as libc::c_int + pl) as usize] as *const pixel)
                    .offset(uv_off as isize),
                (*in_0).stride[1],
                data,
                cpw as size_t,
                (*scaling.offset(0)).as_ptr(),
                (*grain_lut.offset((1 as libc::c_int + pl) as isize)).as_ptr(),
                bh_0,
                row,
                luma_src,
                (*in_0).stride[0],
                pl,
                is_id,
            );
            pl += 1;
        }
    } else {
        let mut pl_0 = 0;
        while pl_0 < 2 {
            if (*data).num_uv_points[pl_0 as usize] != 0 {
                ((*dsp)
                    .fguv_32x32xn[((*in_0).p.layout as libc::c_uint)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as usize])
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*out).data[(1 as libc::c_int + pl_0) as usize] as *mut pixel)
                        .offset(uv_off as isize),
                    ((*in_0).data[(1 as libc::c_int + pl_0) as usize] as *const pixel)
                        .offset(uv_off as isize),
                    (*in_0).stride[1],
                    data,
                    cpw as size_t,
                    (*scaling.offset((1 as libc::c_int + pl_0) as isize)).as_ptr(),
                    (*grain_lut.offset((1 as libc::c_int + pl_0) as isize)).as_ptr(),
                    bh_0,
                    row,
                    luma_src,
                    (*in_0).stride[0],
                    pl_0,
                    is_id,
                );
            }
            pl_0 += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_apply_grain_8bpc(
    dsp: *const Dav1dFilmGrainDSPContext,
    out: *mut Dav1dPicture,
    in_0: *const Dav1dPicture,
) {
    let mut grain_lut = Align16([[[0; 82]; 74]; 3]);
    let mut scaling = Align64([[0; 256]; 3]);
    let rows: libc::c_int = (*out).p.h + 31 >> 5;
    dav1d_prep_grain_8bpc(dsp, out, in_0, scaling.0.as_mut_ptr(), grain_lut.0.as_mut_ptr());
    let mut row = 0;
    while row < rows {
        dav1d_apply_grain_row_8bpc(
            dsp,
            out,
            in_0,
            scaling.0.as_mut_ptr() as *const [uint8_t; 256],
            grain_lut.0.as_mut_ptr() as *const [[entry; 82]; 74],
            row,
        );
        row += 1;
    }
}
