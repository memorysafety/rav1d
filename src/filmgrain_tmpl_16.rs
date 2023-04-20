use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
use cfg_if::cfg_if;

extern "C" {
    static dav1d_gaussian_sequence: [int16_t; 2048];
}

#[cfg(feature = "asm")]
extern "C" {
    static mut dav1d_cpu_flags: libc::c_uint;
    static mut dav1d_cpu_flags_mask: libc::c_uint;
}

#[cfg(all(
    feature = "asm",
    any(target_arch = "x86", target_arch = "x86_64")),
)]
extern "C" {
    fn dav1d_fguv_32x32xn_i422_16bpc_avx512icl(
        dst_row: *mut pixel,
        src_row: *const pixel,
        stride: ptrdiff_t,
        data: *const Dav1dFilmGrainData,
        pw: size_t,
        scaling: *const uint8_t,
        grain_lut: *const [entry; 82],
        bh: libc::c_int,
        row_num: libc::c_int,
        luma_row: *const pixel,
        luma_stride: ptrdiff_t,
        uv_pl: libc::c_int,
        is_id: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_fguv_32x32xn_i422_16bpc_ssse3(
        dst_row: *mut pixel,
        src_row: *const pixel,
        stride: ptrdiff_t,
        data: *const Dav1dFilmGrainData,
        pw: size_t,
        scaling: *const uint8_t,
        grain_lut: *const [entry; 82],
        bh: libc::c_int,
        row_num: libc::c_int,
        luma_row: *const pixel,
        luma_stride: ptrdiff_t,
        uv_pl: libc::c_int,
        is_id: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_generate_grain_uv_444_16bpc_ssse3(
        buf: *mut [entry; 82],
        buf_y: *const [entry; 82],
        data: *const Dav1dFilmGrainData,
        uv: intptr_t,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_generate_grain_uv_422_16bpc_ssse3(
        buf: *mut [entry; 82],
        buf_y: *const [entry; 82],
        data: *const Dav1dFilmGrainData,
        uv: intptr_t,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_fguv_32x32xn_i420_16bpc_ssse3(
        dst_row: *mut pixel,
        src_row: *const pixel,
        stride: ptrdiff_t,
        data: *const Dav1dFilmGrainData,
        pw: size_t,
        scaling: *const uint8_t,
        grain_lut: *const [entry; 82],
        bh: libc::c_int,
        row_num: libc::c_int,
        luma_row: *const pixel,
        luma_stride: ptrdiff_t,
        uv_pl: libc::c_int,
        is_id: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_fgy_32x32xn_16bpc_ssse3(
        dst_row: *mut pixel,
        src_row: *const pixel,
        stride: ptrdiff_t,
        data: *const Dav1dFilmGrainData,
        pw: size_t,
        scaling: *const uint8_t,
        grain_lut: *const [entry; 82],
        bh: libc::c_int,
        row_num: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_generate_grain_uv_420_16bpc_ssse3(
        buf: *mut [entry; 82],
        buf_y: *const [entry; 82],
        data: *const Dav1dFilmGrainData,
        uv: intptr_t,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_generate_grain_y_16bpc_ssse3(
        buf: *mut [entry; 82],
        data: *const Dav1dFilmGrainData,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_generate_grain_uv_444_16bpc_avx2(
        buf: *mut [entry; 82],
        buf_y: *const [entry; 82],
        data: *const Dav1dFilmGrainData,
        uv: intptr_t,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_fgy_32x32xn_16bpc_avx2(
        dst_row: *mut pixel,
        src_row: *const pixel,
        stride: ptrdiff_t,
        data: *const Dav1dFilmGrainData,
        pw: size_t,
        scaling: *const uint8_t,
        grain_lut: *const [entry; 82],
        bh: libc::c_int,
        row_num: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_fguv_32x32xn_i420_16bpc_avx2(
        dst_row: *mut pixel,
        src_row: *const pixel,
        stride: ptrdiff_t,
        data: *const Dav1dFilmGrainData,
        pw: size_t,
        scaling: *const uint8_t,
        grain_lut: *const [entry; 82],
        bh: libc::c_int,
        row_num: libc::c_int,
        luma_row: *const pixel,
        luma_stride: ptrdiff_t,
        uv_pl: libc::c_int,
        is_id: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_fguv_32x32xn_i422_16bpc_avx2(
        dst_row: *mut pixel,
        src_row: *const pixel,
        stride: ptrdiff_t,
        data: *const Dav1dFilmGrainData,
        pw: size_t,
        scaling: *const uint8_t,
        grain_lut: *const [entry; 82],
        bh: libc::c_int,
        row_num: libc::c_int,
        luma_row: *const pixel,
        luma_stride: ptrdiff_t,
        uv_pl: libc::c_int,
        is_id: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_fguv_32x32xn_i444_16bpc_avx2(
        dst_row: *mut pixel,
        src_row: *const pixel,
        stride: ptrdiff_t,
        data: *const Dav1dFilmGrainData,
        pw: size_t,
        scaling: *const uint8_t,
        grain_lut: *const [entry; 82],
        bh: libc::c_int,
        row_num: libc::c_int,
        luma_row: *const pixel,
        luma_stride: ptrdiff_t,
        uv_pl: libc::c_int,
        is_id: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_fgy_32x32xn_16bpc_avx512icl(
        dst_row: *mut pixel,
        src_row: *const pixel,
        stride: ptrdiff_t,
        data: *const Dav1dFilmGrainData,
        pw: size_t,
        scaling: *const uint8_t,
        grain_lut: *const [entry; 82],
        bh: libc::c_int,
        row_num: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_fguv_32x32xn_i420_16bpc_avx512icl(
        dst_row: *mut pixel,
        src_row: *const pixel,
        stride: ptrdiff_t,
        data: *const Dav1dFilmGrainData,
        pw: size_t,
        scaling: *const uint8_t,
        grain_lut: *const [entry; 82],
        bh: libc::c_int,
        row_num: libc::c_int,
        luma_row: *const pixel,
        luma_stride: ptrdiff_t,
        uv_pl: libc::c_int,
        is_id: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_fguv_32x32xn_i444_16bpc_ssse3(
        dst_row: *mut pixel,
        src_row: *const pixel,
        stride: ptrdiff_t,
        data: *const Dav1dFilmGrainData,
        pw: size_t,
        scaling: *const uint8_t,
        grain_lut: *const [entry; 82],
        bh: libc::c_int,
        row_num: libc::c_int,
        luma_row: *const pixel,
        luma_stride: ptrdiff_t,
        uv_pl: libc::c_int,
        is_id: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_fguv_32x32xn_i444_16bpc_avx512icl(
        dst_row: *mut pixel,
        src_row: *const pixel,
        stride: ptrdiff_t,
        data: *const Dav1dFilmGrainData,
        pw: size_t,
        scaling: *const uint8_t,
        grain_lut: *const [entry; 82],
        bh: libc::c_int,
        row_num: libc::c_int,
        luma_row: *const pixel,
        luma_stride: ptrdiff_t,
        uv_pl: libc::c_int,
        is_id: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_generate_grain_uv_420_16bpc_avx2(
        buf: *mut [entry; 82],
        buf_y: *const [entry; 82],
        data: *const Dav1dFilmGrainData,
        uv: intptr_t,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_generate_grain_y_16bpc_avx2(
        buf: *mut [entry; 82],
        data: *const Dav1dFilmGrainData,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_generate_grain_uv_422_16bpc_avx2(
        buf: *mut [entry; 82],
        buf_y: *const [entry; 82],
        data: *const Dav1dFilmGrainData,
        uv: intptr_t,
        bitdepth_max: libc::c_int,
    );
}

#[cfg(all(
    feature = "asm",
    any(target_arch = "arm", target_arch = "aarch64")),
)]
extern "C" {
    fn dav1d_fguv_32x32_420_16bpc_neon(
        dst: *mut pixel,
        src: *const pixel,
        stride: ptrdiff_t,
        scaling: *const uint8_t,
        data: *const Dav1dFilmGrainData,
        grain_lut: *const [entry; 82],
        luma_row: *const pixel,
        luma_stride: ptrdiff_t,
        offsets: *const [libc::c_int; 2],
        h: ptrdiff_t,
        uv: ptrdiff_t,
        is_id: ptrdiff_t,
        type_0: ptrdiff_t,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_generate_grain_uv_422_16bpc_neon(
        buf: *mut [entry; 82],
        buf_y: *const [entry; 82],
        data: *const Dav1dFilmGrainData,
        uv: intptr_t,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_generate_grain_uv_444_16bpc_neon(
        buf: *mut [entry; 82],
        buf_y: *const [entry; 82],
        data: *const Dav1dFilmGrainData,
        uv: intptr_t,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_generate_grain_y_16bpc_neon(
        buf: *mut [entry; 82],
        data: *const Dav1dFilmGrainData,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_generate_grain_uv_420_16bpc_neon(
        buf: *mut [entry; 82],
        buf_y: *const [entry; 82],
        data: *const Dav1dFilmGrainData,
        uv: intptr_t,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_fgy_32x32_16bpc_neon(
        dst: *mut pixel,
        src: *const pixel,
        stride: ptrdiff_t,
        scaling: *const uint8_t,
        scaling_shift: libc::c_int,
        grain_lut: *const [entry; 82],
        offsets: *const [libc::c_int; 2],
        h: libc::c_int,
        clip: ptrdiff_t,
        type_0: ptrdiff_t,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_fguv_32x32_422_16bpc_neon(
        dst: *mut pixel,
        src: *const pixel,
        stride: ptrdiff_t,
        scaling: *const uint8_t,
        data: *const Dav1dFilmGrainData,
        grain_lut: *const [entry; 82],
        luma_row: *const pixel,
        luma_stride: ptrdiff_t,
        offsets: *const [libc::c_int; 2],
        h: ptrdiff_t,
        uv: ptrdiff_t,
        is_id: ptrdiff_t,
        type_0: ptrdiff_t,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_fguv_32x32_444_16bpc_neon(
        dst: *mut pixel,
        src: *const pixel,
        stride: ptrdiff_t,
        scaling: *const uint8_t,
        data: *const Dav1dFilmGrainData,
        grain_lut: *const [entry; 82],
        luma_row: *const pixel,
        luma_stride: ptrdiff_t,
        offsets: *const [libc::c_int; 2],
        h: ptrdiff_t,
        uv: ptrdiff_t,
        is_id: ptrdiff_t,
        type_0: ptrdiff_t,
        bitdepth_max: libc::c_int,
    );
}

pub type pixel = uint16_t;

use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I422;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;

use crate::include::dav1d::headers::Dav1dFilmGrainData;
pub type entry = int16_t;
pub type generate_grain_y_fn = Option::<
    unsafe extern "C" fn(*mut [entry; 82], *const Dav1dFilmGrainData, libc::c_int) -> (),
>;
pub type generate_grain_uv_fn = Option::<
    unsafe extern "C" fn(
        *mut [entry; 82],
        *const [entry; 82],
        *const Dav1dFilmGrainData,
        intptr_t,
        libc::c_int,
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
use crate::include::common::attributes::clz;
use crate::include::common::intops::iclip;
use crate::include::common::intops::imin;
#[inline]
unsafe extern "C" fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 != 0 {
        unreachable!();
    }
    return x >> 1;
}
use crate::src::filmgrain::get_random_number;
use crate::src::filmgrain::round2;
unsafe extern "C" fn generate_grain_y_c(
    mut buf: *mut [entry; 82],
    data: *const Dav1dFilmGrainData,
    bitdepth_max: libc::c_int,
) {
    let bitdepth_min_8: libc::c_int = 32 as libc::c_int
        - clz(bitdepth_max as libc::c_uint) - 8;
    let mut seed: libc::c_uint = (*data).seed;
    let shift: libc::c_int = 4 - bitdepth_min_8
        + (*data).grain_scale_shift;
    let grain_ctr: libc::c_int = (128 as libc::c_int) << bitdepth_min_8;
    let grain_min: libc::c_int = -grain_ctr;
    let grain_max: libc::c_int = grain_ctr - 1;
    let mut y = 0;
    while y < 73 {
        let mut x = 0;
        while x < 82 {
            let value: libc::c_int = get_random_number(11 as libc::c_int, &mut seed);
            (*buf
                .offset(
                    y as isize,
                ))[x
                as usize] = round2(
                dav1d_gaussian_sequence[value as usize] as libc::c_int,
                shift as uint64_t,
            ) as entry;
            x += 1;
        }
        y += 1;
    }
    let ar_pad = 3;
    let ar_lag: libc::c_int = (*data).ar_coeff_lag;
    let mut y_0: libc::c_int = ar_pad;
    while y_0 < 73 {
        let mut x_0: libc::c_int = ar_pad;
        while x_0 < 82 - ar_pad {
            let mut coeff: *const int8_t = ((*data).ar_coeffs_y).as_ptr();
            let mut sum = 0;
            let mut dy: libc::c_int = -ar_lag;
            while dy <= 0 {
                let mut dx: libc::c_int = -ar_lag;
                while dx <= ar_lag {
                    if dx == 0 && dy == 0 {
                        break;
                    }
                    let fresh0 = coeff;
                    coeff = coeff.offset(1);
                    sum
                        += *fresh0 as libc::c_int
                            * (*buf.offset((y_0 + dy) as isize))[(x_0 + dx) as usize]
                                as libc::c_int;
                    dx += 1;
                }
                dy += 1;
            }
            let grain: libc::c_int = (*buf.offset(y_0 as isize))[x_0 as usize]
                as libc::c_int + round2(sum, (*data).ar_coeff_shift);
            (*buf
                .offset(
                    y_0 as isize,
                ))[x_0 as usize] = iclip(grain, grain_min, grain_max) as entry;
            x_0 += 1;
        }
        y_0 += 1;
    }
}
#[inline(never)]
unsafe extern "C" fn generate_grain_uv_c(
    mut buf: *mut [entry; 82],
    mut buf_y: *const [entry; 82],
    data: *const Dav1dFilmGrainData,
    uv: intptr_t,
    subx: libc::c_int,
    suby: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let bitdepth_min_8: libc::c_int = 32 as libc::c_int
        - clz(bitdepth_max as libc::c_uint) - 8;
    let mut seed: libc::c_uint = (*data).seed
        ^ (if uv != 0 { 0x49d8 as libc::c_int } else { 0xb524 as libc::c_int })
            as libc::c_uint;
    let shift: libc::c_int = 4 - bitdepth_min_8
        + (*data).grain_scale_shift;
    let grain_ctr: libc::c_int = (128 as libc::c_int) << bitdepth_min_8;
    let grain_min: libc::c_int = -grain_ctr;
    let grain_max: libc::c_int = grain_ctr - 1;
    let chromaW: libc::c_int = if subx != 0 {
        44 as libc::c_int
    } else {
        82 as libc::c_int
    };
    let chromaH: libc::c_int = if suby != 0 {
        38 as libc::c_int
    } else {
        73 as libc::c_int
    };
    let mut y = 0;
    while y < chromaH {
        let mut x = 0;
        while x < chromaW {
            let value: libc::c_int = get_random_number(11 as libc::c_int, &mut seed);
            (*buf
                .offset(
                    y as isize,
                ))[x
                as usize] = round2(
                dav1d_gaussian_sequence[value as usize] as libc::c_int,
                shift as uint64_t,
            ) as entry;
            x += 1;
        }
        y += 1;
    }
    let ar_pad = 3;
    let ar_lag: libc::c_int = (*data).ar_coeff_lag;
    let mut y_0: libc::c_int = ar_pad;
    while y_0 < chromaH {
        let mut x_0: libc::c_int = ar_pad;
        while x_0 < chromaW - ar_pad {
            let mut coeff: *const int8_t = ((*data).ar_coeffs_uv[uv as usize]).as_ptr();
            let mut sum = 0;
            let mut dy: libc::c_int = -ar_lag;
            while dy <= 0 {
                let mut dx: libc::c_int = -ar_lag;
                while dx <= ar_lag {
                    if dx == 0 && dy == 0 {
                        if (*data).num_y_points == 0 {
                            break;
                        }
                        let mut luma = 0;
                        let lumaX: libc::c_int = (x_0 - ar_pad << subx) + ar_pad;
                        let lumaY: libc::c_int = (y_0 - ar_pad << suby) + ar_pad;
                        let mut i = 0;
                        while i <= suby {
                            let mut j = 0;
                            while j <= subx {
                                luma
                                    += (*buf_y
                                        .offset((lumaY + i) as isize))[(lumaX + j) as usize]
                                        as libc::c_int;
                                j += 1;
                            }
                            i += 1;
                        }
                        luma = round2(luma, (subx + suby) as uint64_t);
                        sum += luma * *coeff as libc::c_int;
                        break;
                    } else {
                        let fresh1 = coeff;
                        coeff = coeff.offset(1);
                        sum
                            += *fresh1 as libc::c_int
                                * (*buf.offset((y_0 + dy) as isize))[(x_0 + dx) as usize]
                                    as libc::c_int;
                        dx += 1;
                    }
                }
                dy += 1;
            }
            let grain: libc::c_int = (*buf.offset(y_0 as isize))[x_0 as usize]
                as libc::c_int + round2(sum, (*data).ar_coeff_shift);
            (*buf
                .offset(
                    y_0 as isize,
                ))[x_0 as usize] = iclip(grain, grain_min, grain_max) as entry;
            x_0 += 1;
        }
        y_0 += 1;
    }
}
unsafe extern "C" fn generate_grain_uv_420_c(
    mut buf: *mut [entry; 82],
    mut buf_y: *const [entry; 82],
    data: *const Dav1dFilmGrainData,
    uv: intptr_t,
    bitdepth_max: libc::c_int,
) {
    generate_grain_uv_c(
        buf,
        buf_y,
        data,
        uv,
        1 as libc::c_int,
        1 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn generate_grain_uv_422_c(
    mut buf: *mut [entry; 82],
    mut buf_y: *const [entry; 82],
    data: *const Dav1dFilmGrainData,
    uv: intptr_t,
    bitdepth_max: libc::c_int,
) {
    generate_grain_uv_c(
        buf,
        buf_y,
        data,
        uv,
        1 as libc::c_int,
        0 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn generate_grain_uv_444_c(
    mut buf: *mut [entry; 82],
    mut buf_y: *const [entry; 82],
    data: *const Dav1dFilmGrainData,
    uv: intptr_t,
    bitdepth_max: libc::c_int,
) {
    generate_grain_uv_c(
        buf,
        buf_y,
        data,
        uv,
        0 as libc::c_int,
        0 as libc::c_int,
        bitdepth_max,
    );
}
#[inline]
unsafe extern "C" fn sample_lut(
    mut grain_lut: *const [entry; 82],
    mut offsets: *const [libc::c_int; 2],
    subx: libc::c_int,
    suby: libc::c_int,
    bx: libc::c_int,
    by: libc::c_int,
    x: libc::c_int,
    y: libc::c_int,
) -> entry {
    let randval: libc::c_int = (*offsets.offset(bx as isize))[by as usize];
    let offx: libc::c_int = 3 as libc::c_int
        + (2 >> subx)
            * (3 + (randval >> 4));
    let offy: libc::c_int = 3 as libc::c_int
        + (2 >> suby)
            * (3 + (randval & 0xf as libc::c_int));
    return (*grain_lut
        .offset(
            (offy + y + (32 >> suby) * by) as isize,
        ))[(offx + x + (32 >> subx) * bx) as usize];
}
unsafe extern "C" fn fgy_32x32xn_c(
    dst_row: *mut pixel,
    src_row: *const pixel,
    stride: ptrdiff_t,
    data: *const Dav1dFilmGrainData,
    pw: size_t,
    mut scaling: *const uint8_t,
    mut grain_lut: *const [entry; 82],
    bh: libc::c_int,
    row_num: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let rows: libc::c_int = 1 as libc::c_int
        + ((*data).overlap_flag != 0 && row_num > 0) as libc::c_int;
    let bitdepth_min_8: libc::c_int = 32 as libc::c_int
        - clz(bitdepth_max as libc::c_uint) - 8;
    let grain_ctr: libc::c_int = (128 as libc::c_int) << bitdepth_min_8;
    let grain_min: libc::c_int = -grain_ctr;
    let grain_max: libc::c_int = grain_ctr - 1;
    let mut min_value: libc::c_int = 0;
    let mut max_value: libc::c_int = 0;
    if (*data).clip_to_restricted_range != 0 {
        min_value = (16 as libc::c_int) << bitdepth_min_8;
        max_value = (235 as libc::c_int) << bitdepth_min_8;
    } else {
        min_value = 0 as libc::c_int;
        max_value = bitdepth_max;
    }
    let mut seed: [libc::c_uint; 2] = [0; 2];
    let mut i = 0;
    while i < rows {
        seed[i as usize] = (*data).seed;
        seed[i as usize]
            ^= (((row_num - i) * 37 + 178
                & 0xff as libc::c_int) << 8) as libc::c_uint;
        seed[i as usize]
            ^= ((row_num - i) * 173 + 105
                & 0xff as libc::c_int) as libc::c_uint;
        i += 1;
    }
    if !((stride as libc::c_ulong)
        .wrapping_rem(
            (32 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong),
        ) == 0 as libc::c_ulong)
    {
        unreachable!();
    }
    let mut offsets: [[libc::c_int; 2]; 2] = [[0; 2]; 2];
    let mut bx: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (bx as size_t) < pw {
        let bw: libc::c_int = imin(
            32 as libc::c_int,
            (pw as libc::c_int as libc::c_uint).wrapping_sub(bx) as libc::c_int,
        );
        if (*data).overlap_flag != 0 && bx != 0 {
            let mut i_0 = 0;
            while i_0 < rows {
                offsets[1 as libc::c_int
                    as usize][i_0
                    as usize] = offsets[0][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1 = 0;
        while i_1 < rows {
            offsets[0 as libc::c_int
                as usize][i_1
                as usize] = get_random_number(
                8 as libc::c_int,
                &mut *seed.as_mut_ptr().offset(i_1 as isize),
            );
            i_1 += 1;
        }
        let ystart: libc::c_int = if (*data).overlap_flag != 0 && row_num != 0 {
            imin(2 as libc::c_int, bh)
        } else {
            0 as libc::c_int
        };
        let xstart: libc::c_int = if (*data).overlap_flag != 0 && bx != 0 {
            imin(2 as libc::c_int, bw)
        } else {
            0 as libc::c_int
        };
        static mut w: [[libc::c_int; 2]; 2] = [
            [27 as libc::c_int, 17 as libc::c_int],
            [17 as libc::c_int, 27 as libc::c_int],
        ];
        let mut y: libc::c_int = ystart;
        while y < bh {
            let mut x: libc::c_int = xstart;
            while x < bw {
                let mut grain: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    x,
                    y,
                ) as libc::c_int;
                let src: *const pixel = src_row
                    .offset((y as isize * PXSTRIDE(stride)) as isize)
                    .offset(x as isize)
                    .offset(bx as isize);
                let dst: *mut pixel = dst_row
                    .offset((y as isize * PXSTRIDE(stride)) as isize)
                    .offset(x as isize)
                    .offset(bx as isize);
                let noise: libc::c_int = round2(
                    *scaling.offset(*src as isize) as libc::c_int * grain,
                    (*data).scaling_shift as uint64_t,
                );
                *dst = iclip(*src as libc::c_int + noise, min_value, max_value) as pixel;
                x += 1;
            }
            let mut x_0 = 0;
            while x_0 < xstart {
                let mut grain_0: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    x_0,
                    y,
                ) as libc::c_int;
                let mut old: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    0 as libc::c_int,
                    0 as libc::c_int,
                    1 as libc::c_int,
                    0 as libc::c_int,
                    x_0,
                    y,
                ) as libc::c_int;
                grain_0 = round2(
                    old * w[x_0 as usize][0]
                        + grain_0 * w[x_0 as usize][1],
                    5 as libc::c_int as uint64_t,
                );
                grain_0 = iclip(grain_0, grain_min, grain_max);
                let src_0: *const pixel = src_row
                    .offset((y as isize * PXSTRIDE(stride)) as isize)
                    .offset(x_0 as isize)
                    .offset(bx as isize);
                let dst_0: *mut pixel = dst_row
                    .offset((y as isize * PXSTRIDE(stride)) as isize)
                    .offset(x_0 as isize)
                    .offset(bx as isize);
                let noise_0: libc::c_int = round2(
                    *scaling.offset(*src_0 as isize) as libc::c_int * grain_0,
                    (*data).scaling_shift as uint64_t,
                );
                *dst_0 = iclip(*src_0 as libc::c_int + noise_0, min_value, max_value)
                    as pixel;
                x_0 += 1;
            }
            y += 1;
        }
        let mut y_0 = 0;
        while y_0 < ystart {
            let mut x_1: libc::c_int = xstart;
            while x_1 < bw {
                let mut grain_1: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    x_1,
                    y_0,
                ) as libc::c_int;
                let mut old_0: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    1 as libc::c_int,
                    x_1,
                    y_0,
                ) as libc::c_int;
                grain_1 = round2(
                    old_0 * w[y_0 as usize][0]
                        + grain_1 * w[y_0 as usize][1],
                    5 as libc::c_int as uint64_t,
                );
                grain_1 = iclip(grain_1, grain_min, grain_max);
                let src_1: *const pixel = src_row
                    .offset((y_0 as isize * PXSTRIDE(stride)) as isize)
                    .offset(x_1 as isize)
                    .offset(bx as isize);
                let dst_1: *mut pixel = dst_row
                    .offset((y_0 as isize * PXSTRIDE(stride)) as isize)
                    .offset(x_1 as isize)
                    .offset(bx as isize);
                let noise_1: libc::c_int = round2(
                    *scaling.offset(*src_1 as isize) as libc::c_int * grain_1,
                    (*data).scaling_shift as uint64_t,
                );
                *dst_1 = iclip(*src_1 as libc::c_int + noise_1, min_value, max_value)
                    as pixel;
                x_1 += 1;
            }
            let mut x_2 = 0;
            while x_2 < xstart {
                let mut top: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    1 as libc::c_int,
                    x_2,
                    y_0,
                ) as libc::c_int;
                let mut old_1: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    0 as libc::c_int,
                    0 as libc::c_int,
                    1 as libc::c_int,
                    1 as libc::c_int,
                    x_2,
                    y_0,
                ) as libc::c_int;
                top = round2(
                    old_1 * w[x_2 as usize][0]
                        + top * w[x_2 as usize][1],
                    5 as libc::c_int as uint64_t,
                );
                top = iclip(top, grain_min, grain_max);
                let mut grain_2: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    x_2,
                    y_0,
                ) as libc::c_int;
                old_1 = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    0 as libc::c_int,
                    0 as libc::c_int,
                    1 as libc::c_int,
                    0 as libc::c_int,
                    x_2,
                    y_0,
                ) as libc::c_int;
                grain_2 = round2(
                    old_1 * w[x_2 as usize][0]
                        + grain_2 * w[x_2 as usize][1],
                    5 as libc::c_int as uint64_t,
                );
                grain_2 = iclip(grain_2, grain_min, grain_max);
                grain_2 = round2(
                    top * w[y_0 as usize][0]
                        + grain_2 * w[y_0 as usize][1],
                    5 as libc::c_int as uint64_t,
                );
                grain_2 = iclip(grain_2, grain_min, grain_max);
                let src_2: *const pixel = src_row
                    .offset((y_0 as isize * PXSTRIDE(stride)) as isize)
                    .offset(x_2 as isize)
                    .offset(bx as isize);
                let dst_2: *mut pixel = dst_row
                    .offset((y_0 as isize * PXSTRIDE(stride)) as isize)
                    .offset(x_2 as isize)
                    .offset(bx as isize);
                let noise_2: libc::c_int = round2(
                    *scaling.offset(*src_2 as isize) as libc::c_int * grain_2,
                    (*data).scaling_shift as uint64_t,
                );
                *dst_2 = iclip(*src_2 as libc::c_int + noise_2, min_value, max_value)
                    as pixel;
                x_2 += 1;
            }
            y_0 += 1;
        }
        bx = bx.wrapping_add(32 as libc::c_int as libc::c_uint);
    }
}
#[inline(never)]
unsafe extern "C" fn fguv_32x32xn_c(
    dst_row: *mut pixel,
    src_row: *const pixel,
    stride: ptrdiff_t,
    data: *const Dav1dFilmGrainData,
    pw: size_t,
    mut scaling: *const uint8_t,
    mut grain_lut: *const [entry; 82],
    bh: libc::c_int,
    row_num: libc::c_int,
    luma_row: *const pixel,
    luma_stride: ptrdiff_t,
    uv: libc::c_int,
    is_id: libc::c_int,
    sx: libc::c_int,
    sy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let rows: libc::c_int = 1 as libc::c_int
        + ((*data).overlap_flag != 0 && row_num > 0) as libc::c_int;
    let bitdepth_min_8: libc::c_int = 32 as libc::c_int
        - clz(bitdepth_max as libc::c_uint) - 8;
    let grain_ctr: libc::c_int = (128 as libc::c_int) << bitdepth_min_8;
    let grain_min: libc::c_int = -grain_ctr;
    let grain_max: libc::c_int = grain_ctr - 1;
    let mut min_value: libc::c_int = 0;
    let mut max_value: libc::c_int = 0;
    if (*data).clip_to_restricted_range != 0 {
        min_value = (16 as libc::c_int) << bitdepth_min_8;
        max_value = (if is_id != 0 { 235 as libc::c_int } else { 240 as libc::c_int })
            << bitdepth_min_8;
    } else {
        min_value = 0 as libc::c_int;
        max_value = bitdepth_max;
    }
    let mut seed: [libc::c_uint; 2] = [0; 2];
    let mut i = 0;
    while i < rows {
        seed[i as usize] = (*data).seed;
        seed[i as usize]
            ^= (((row_num - i) * 37 + 178
                & 0xff as libc::c_int) << 8) as libc::c_uint;
        seed[i as usize]
            ^= ((row_num - i) * 173 + 105
                & 0xff as libc::c_int) as libc::c_uint;
        i += 1;
    }
    if !((stride as libc::c_ulong)
        .wrapping_rem(
            (32 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong),
        ) == 0 as libc::c_ulong)
    {
        unreachable!();
    }
    let mut offsets: [[libc::c_int; 2]; 2] = [[0; 2]; 2];
    let mut bx: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (bx as size_t) < pw {
        let bw: libc::c_int = imin(
            32 >> sx,
            pw.wrapping_sub(bx as size_t) as libc::c_int,
        );
        if (*data).overlap_flag != 0 && bx != 0 {
            let mut i_0 = 0;
            while i_0 < rows {
                offsets[1 as libc::c_int
                    as usize][i_0
                    as usize] = offsets[0][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1 = 0;
        while i_1 < rows {
            offsets[0 as libc::c_int
                as usize][i_1
                as usize] = get_random_number(
                8 as libc::c_int,
                &mut *seed.as_mut_ptr().offset(i_1 as isize),
            );
            i_1 += 1;
        }
        let ystart: libc::c_int = if (*data).overlap_flag != 0 && row_num != 0 {
            imin(2 >> sy, bh)
        } else {
            0 as libc::c_int
        };
        let xstart: libc::c_int = if (*data).overlap_flag != 0 && bx != 0 {
            imin(2 >> sx, bw)
        } else {
            0 as libc::c_int
        };
        static mut w: [[[libc::c_int; 2]; 2]; 2] = [
            [
                [27 as libc::c_int, 17 as libc::c_int],
                [17 as libc::c_int, 27 as libc::c_int],
            ],
            [[23 as libc::c_int, 22 as libc::c_int], [0; 2]],
        ];
        let mut y: libc::c_int = ystart;
        while y < bh {
            let mut x: libc::c_int = xstart;
            while x < bw {
                let mut grain: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    sx,
                    sy,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    x,
                    y,
                ) as libc::c_int;
                let lx: libc::c_int = (bx.wrapping_add(x as libc::c_uint) << sx)
                    as libc::c_int;
                let ly: libc::c_int = y << sy;
                let luma: *const pixel = luma_row
                    .offset((ly as isize * PXSTRIDE(luma_stride)) as isize)
                    .offset(lx as isize);
                let mut avg: pixel = *luma.offset(0);
                if sx != 0 {
                    avg = (avg as libc::c_int
                        + *luma.offset(1) as libc::c_int
                        + 1 >> 1) as pixel;
                }
                let src: *const pixel = src_row
                    .offset((y as isize * PXSTRIDE(stride)) as isize)
                    .offset(bx.wrapping_add(x as libc::c_uint) as isize);
                let dst: *mut pixel = dst_row
                    .offset((y as isize * PXSTRIDE(stride)) as isize)
                    .offset(bx.wrapping_add(x as libc::c_uint) as isize);
                let mut val: libc::c_int = avg as libc::c_int;
                if (*data).chroma_scaling_from_luma == 0 {
                    let combined: libc::c_int = avg as libc::c_int
                        * (*data).uv_luma_mult[uv as usize]
                        + *src as libc::c_int * (*data).uv_mult[uv as usize];
                    val = iclip(
                        (combined >> 6)
                            + (*data).uv_offset[uv as usize]
                                * ((1 as libc::c_int) << bitdepth_min_8),
                        0 as libc::c_int,
                        bitdepth_max,
                    );
                }
                let noise: libc::c_int = round2(
                    *scaling.offset(val as isize) as libc::c_int * grain,
                    (*data).scaling_shift as uint64_t,
                );
                *dst = iclip(*src as libc::c_int + noise, min_value, max_value) as pixel;
                x += 1;
            }
            let mut x_0 = 0;
            while x_0 < xstart {
                let mut grain_0: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    sx,
                    sy,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    x_0,
                    y,
                ) as libc::c_int;
                let mut old: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    sx,
                    sy,
                    1 as libc::c_int,
                    0 as libc::c_int,
                    x_0,
                    y,
                ) as libc::c_int;
                grain_0 = round2(
                    old * w[sx as usize][x_0 as usize][0]
                        + grain_0
                            * w[sx as usize][x_0 as usize][1],
                    5 as libc::c_int as uint64_t,
                );
                grain_0 = iclip(grain_0, grain_min, grain_max);
                let lx_0: libc::c_int = (bx.wrapping_add(x_0 as libc::c_uint) << sx)
                    as libc::c_int;
                let ly_0: libc::c_int = y << sy;
                let luma_0: *const pixel = luma_row
                    .offset((ly_0 as isize * PXSTRIDE(luma_stride)) as isize)
                    .offset(lx_0 as isize);
                let mut avg_0: pixel = *luma_0.offset(0);
                if sx != 0 {
                    avg_0 = (avg_0 as libc::c_int
                        + *luma_0.offset(1) as libc::c_int
                        + 1 >> 1) as pixel;
                }
                let src_0: *const pixel = src_row
                    .offset((y as isize * PXSTRIDE(stride)) as isize)
                    .offset(bx.wrapping_add(x_0 as libc::c_uint) as isize);
                let dst_0: *mut pixel = dst_row
                    .offset((y as isize * PXSTRIDE(stride)) as isize)
                    .offset(bx.wrapping_add(x_0 as libc::c_uint) as isize);
                let mut val_0: libc::c_int = avg_0 as libc::c_int;
                if (*data).chroma_scaling_from_luma == 0 {
                    let combined_0: libc::c_int = avg_0 as libc::c_int
                        * (*data).uv_luma_mult[uv as usize]
                        + *src_0 as libc::c_int * (*data).uv_mult[uv as usize];
                    val_0 = iclip(
                        (combined_0 >> 6)
                            + (*data).uv_offset[uv as usize]
                                * ((1 as libc::c_int) << bitdepth_min_8),
                        0 as libc::c_int,
                        bitdepth_max,
                    );
                }
                let noise_0: libc::c_int = round2(
                    *scaling.offset(val_0 as isize) as libc::c_int * grain_0,
                    (*data).scaling_shift as uint64_t,
                );
                *dst_0 = iclip(*src_0 as libc::c_int + noise_0, min_value, max_value)
                    as pixel;
                x_0 += 1;
            }
            y += 1;
        }
        let mut y_0 = 0;
        while y_0 < ystart {
            let mut x_1: libc::c_int = xstart;
            while x_1 < bw {
                let mut grain_1: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    sx,
                    sy,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    x_1,
                    y_0,
                ) as libc::c_int;
                let mut old_0: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    sx,
                    sy,
                    0 as libc::c_int,
                    1 as libc::c_int,
                    x_1,
                    y_0,
                ) as libc::c_int;
                grain_1 = round2(
                    old_0 * w[sy as usize][y_0 as usize][0]
                        + grain_1
                            * w[sy as usize][y_0 as usize][1],
                    5 as libc::c_int as uint64_t,
                );
                grain_1 = iclip(grain_1, grain_min, grain_max);
                let lx_1: libc::c_int = (bx.wrapping_add(x_1 as libc::c_uint) << sx)
                    as libc::c_int;
                let ly_1: libc::c_int = y_0 << sy;
                let luma_1: *const pixel = luma_row
                    .offset((ly_1 as isize * PXSTRIDE(luma_stride)) as isize)
                    .offset(lx_1 as isize);
                let mut avg_1: pixel = *luma_1.offset(0);
                if sx != 0 {
                    avg_1 = (avg_1 as libc::c_int
                        + *luma_1.offset(1) as libc::c_int
                        + 1 >> 1) as pixel;
                }
                let src_1: *const pixel = src_row
                    .offset((y_0 as isize * PXSTRIDE(stride)) as isize)
                    .offset(bx.wrapping_add(x_1 as libc::c_uint) as isize);
                let dst_1: *mut pixel = dst_row
                    .offset((y_0 as isize * PXSTRIDE(stride)) as isize)
                    .offset(bx.wrapping_add(x_1 as libc::c_uint) as isize);
                let mut val_1: libc::c_int = avg_1 as libc::c_int;
                if (*data).chroma_scaling_from_luma == 0 {
                    let combined_1: libc::c_int = avg_1 as libc::c_int
                        * (*data).uv_luma_mult[uv as usize]
                        + *src_1 as libc::c_int * (*data).uv_mult[uv as usize];
                    val_1 = iclip(
                        (combined_1 >> 6)
                            + (*data).uv_offset[uv as usize]
                                * ((1 as libc::c_int) << bitdepth_min_8),
                        0 as libc::c_int,
                        bitdepth_max,
                    );
                }
                let noise_1: libc::c_int = round2(
                    *scaling.offset(val_1 as isize) as libc::c_int * grain_1,
                    (*data).scaling_shift as uint64_t,
                );
                *dst_1 = iclip(*src_1 as libc::c_int + noise_1, min_value, max_value)
                    as pixel;
                x_1 += 1;
            }
            let mut x_2 = 0;
            while x_2 < xstart {
                let mut top: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    sx,
                    sy,
                    0 as libc::c_int,
                    1 as libc::c_int,
                    x_2,
                    y_0,
                ) as libc::c_int;
                let mut old_1: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    sx,
                    sy,
                    1 as libc::c_int,
                    1 as libc::c_int,
                    x_2,
                    y_0,
                ) as libc::c_int;
                top = round2(
                    old_1 * w[sx as usize][x_2 as usize][0]
                        + top * w[sx as usize][x_2 as usize][1],
                    5 as libc::c_int as uint64_t,
                );
                top = iclip(top, grain_min, grain_max);
                let mut grain_2: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    sx,
                    sy,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    x_2,
                    y_0,
                ) as libc::c_int;
                old_1 = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    sx,
                    sy,
                    1 as libc::c_int,
                    0 as libc::c_int,
                    x_2,
                    y_0,
                ) as libc::c_int;
                grain_2 = round2(
                    old_1 * w[sx as usize][x_2 as usize][0]
                        + grain_2
                            * w[sx as usize][x_2 as usize][1],
                    5 as libc::c_int as uint64_t,
                );
                grain_2 = iclip(grain_2, grain_min, grain_max);
                grain_2 = round2(
                    top * w[sy as usize][y_0 as usize][0]
                        + grain_2
                            * w[sy as usize][y_0 as usize][1],
                    5 as libc::c_int as uint64_t,
                );
                grain_2 = iclip(grain_2, grain_min, grain_max);
                let lx_2: libc::c_int = (bx.wrapping_add(x_2 as libc::c_uint) << sx)
                    as libc::c_int;
                let ly_2: libc::c_int = y_0 << sy;
                let luma_2: *const pixel = luma_row
                    .offset((ly_2 as isize * PXSTRIDE(luma_stride)) as isize)
                    .offset(lx_2 as isize);
                let mut avg_2: pixel = *luma_2.offset(0);
                if sx != 0 {
                    avg_2 = (avg_2 as libc::c_int
                        + *luma_2.offset(1) as libc::c_int
                        + 1 >> 1) as pixel;
                }
                let src_2: *const pixel = src_row
                    .offset((y_0 as isize * PXSTRIDE(stride)) as isize)
                    .offset(bx.wrapping_add(x_2 as libc::c_uint) as isize);
                let dst_2: *mut pixel = dst_row
                    .offset((y_0 as isize * PXSTRIDE(stride)) as isize)
                    .offset(bx.wrapping_add(x_2 as libc::c_uint) as isize);
                let mut val_2: libc::c_int = avg_2 as libc::c_int;
                if (*data).chroma_scaling_from_luma == 0 {
                    let combined_2: libc::c_int = avg_2 as libc::c_int
                        * (*data).uv_luma_mult[uv as usize]
                        + *src_2 as libc::c_int * (*data).uv_mult[uv as usize];
                    val_2 = iclip(
                        (combined_2 >> 6)
                            + (*data).uv_offset[uv as usize]
                                * ((1 as libc::c_int) << bitdepth_min_8),
                        0 as libc::c_int,
                        bitdepth_max,
                    );
                }
                let noise_2: libc::c_int = round2(
                    *scaling.offset(val_2 as isize) as libc::c_int * grain_2,
                    (*data).scaling_shift as uint64_t,
                );
                *dst_2 = iclip(*src_2 as libc::c_int + noise_2, min_value, max_value)
                    as pixel;
                x_2 += 1;
            }
            y_0 += 1;
        }
        bx = bx.wrapping_add((32 >> sx) as libc::c_uint);
    }
}
unsafe extern "C" fn fguv_32x32xn_420_c(
    mut dst_row: *mut pixel,
    mut src_row: *const pixel,
    mut stride: ptrdiff_t,
    mut data: *const Dav1dFilmGrainData,
    mut pw: size_t,
    mut scaling: *const uint8_t,
    mut grain_lut: *const [entry; 82],
    mut bh: libc::c_int,
    mut row_num: libc::c_int,
    mut luma_row: *const pixel,
    mut luma_stride: ptrdiff_t,
    mut uv_pl: libc::c_int,
    mut is_id: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    fguv_32x32xn_c(
        dst_row,
        src_row,
        stride,
        data,
        pw,
        scaling,
        grain_lut,
        bh,
        row_num,
        luma_row,
        luma_stride,
        uv_pl,
        is_id,
        1 as libc::c_int,
        1 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn fguv_32x32xn_422_c(
    mut dst_row: *mut pixel,
    mut src_row: *const pixel,
    mut stride: ptrdiff_t,
    mut data: *const Dav1dFilmGrainData,
    mut pw: size_t,
    mut scaling: *const uint8_t,
    mut grain_lut: *const [entry; 82],
    mut bh: libc::c_int,
    mut row_num: libc::c_int,
    mut luma_row: *const pixel,
    mut luma_stride: ptrdiff_t,
    mut uv_pl: libc::c_int,
    mut is_id: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    fguv_32x32xn_c(
        dst_row,
        src_row,
        stride,
        data,
        pw,
        scaling,
        grain_lut,
        bh,
        row_num,
        luma_row,
        luma_stride,
        uv_pl,
        is_id,
        1 as libc::c_int,
        0 as libc::c_int,
        bitdepth_max,
    );
}
unsafe extern "C" fn fguv_32x32xn_444_c(
    mut dst_row: *mut pixel,
    mut src_row: *const pixel,
    mut stride: ptrdiff_t,
    mut data: *const Dav1dFilmGrainData,
    mut pw: size_t,
    mut scaling: *const uint8_t,
    mut grain_lut: *const [entry; 82],
    mut bh: libc::c_int,
    mut row_num: libc::c_int,
    mut luma_row: *const pixel,
    mut luma_stride: ptrdiff_t,
    mut uv_pl: libc::c_int,
    mut is_id: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    fguv_32x32xn_c(
        dst_row,
        src_row,
        stride,
        data,
        pw,
        scaling,
        grain_lut,
        bh,
        row_num,
        luma_row,
        luma_stride,
        uv_pl,
        is_id,
        0 as libc::c_int,
        0 as libc::c_int,
        bitdepth_max,
    );
}

#[cfg(feature = "asm")]
use crate::src::cpu::dav1d_get_cpu_flags;

#[cfg(all(
    feature = "asm",
    any(target_arch = "x86", target_arch = "x86_64"),
))]
#[inline(always)]
unsafe extern "C" fn film_grain_dsp_init_x86(c: *mut Dav1dFilmGrainDSPContext) {
    use crate::src::x86::cpu::DAV1D_X86_CPU_FLAG_AVX512ICL;
    use crate::src::x86::cpu::DAV1D_X86_CPU_FLAG_SLOW_GATHER;
    use crate::src::x86::cpu::DAV1D_X86_CPU_FLAG_AVX2;
    use crate::src::x86::cpu::DAV1D_X86_CPU_FLAG_SSSE3;

    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_X86_CPU_FLAG_SSSE3 == 0 {
        return;
    }

    (*c).generate_grain_y = Some(dav1d_generate_grain_y_16bpc_ssse3);
    (*c).generate_grain_uv[(DAV1D_PIXEL_LAYOUT_I420 - 1) as usize] = Some(dav1d_generate_grain_uv_420_16bpc_ssse3);
    (*c).generate_grain_uv[(DAV1D_PIXEL_LAYOUT_I422 - 1) as usize] = Some(dav1d_generate_grain_uv_422_16bpc_ssse3);
    (*c).generate_grain_uv[(DAV1D_PIXEL_LAYOUT_I444 - 1) as usize] = Some(dav1d_generate_grain_uv_444_16bpc_ssse3);

    (*c).fgy_32x32xn = Some(dav1d_fgy_32x32xn_16bpc_ssse3);
    (*c).fguv_32x32xn[(DAV1D_PIXEL_LAYOUT_I420 - 1) as usize] = Some(dav1d_fguv_32x32xn_i420_16bpc_ssse3);
    (*c).fguv_32x32xn[(DAV1D_PIXEL_LAYOUT_I422 - 1) as usize] = Some(dav1d_fguv_32x32xn_i422_16bpc_ssse3);
    (*c).fguv_32x32xn[(DAV1D_PIXEL_LAYOUT_I444 - 1) as usize] = Some(dav1d_fguv_32x32xn_i444_16bpc_ssse3);

    if flags & DAV1D_X86_CPU_FLAG_AVX2 == 0 {
        return;
    }

    (*c).generate_grain_y = Some(dav1d_generate_grain_y_16bpc_avx2);
    (*c).generate_grain_uv[(DAV1D_PIXEL_LAYOUT_I420 - 1) as usize] = Some(dav1d_generate_grain_uv_420_16bpc_avx2);
    (*c).generate_grain_uv[(DAV1D_PIXEL_LAYOUT_I422 - 1) as usize] = Some(dav1d_generate_grain_uv_422_16bpc_avx2);
    (*c).generate_grain_uv[(DAV1D_PIXEL_LAYOUT_I444 - 1) as usize] = Some(dav1d_generate_grain_uv_444_16bpc_avx2);

    if flags & DAV1D_X86_CPU_FLAG_SLOW_GATHER == 0 {
        (*c).fgy_32x32xn = Some(dav1d_fgy_32x32xn_16bpc_avx2);
        (*c).fguv_32x32xn[(DAV1D_PIXEL_LAYOUT_I420 - 1) as usize] = Some(dav1d_fguv_32x32xn_i420_16bpc_avx2);
        (*c).fguv_32x32xn[(DAV1D_PIXEL_LAYOUT_I422 - 1) as usize] = Some(dav1d_fguv_32x32xn_i422_16bpc_avx2);
        (*c).fguv_32x32xn[(DAV1D_PIXEL_LAYOUT_I444 - 1) as usize] = Some(dav1d_fguv_32x32xn_i444_16bpc_avx2);
    }

    if flags & DAV1D_X86_CPU_FLAG_AVX512ICL == 0 {
        return;
    }

    (*c).fgy_32x32xn = Some(dav1d_fgy_32x32xn_16bpc_avx512icl);
    (*c).fguv_32x32xn[(DAV1D_PIXEL_LAYOUT_I420 - 1) as usize] = Some(dav1d_fguv_32x32xn_i420_16bpc_avx512icl);
    (*c).fguv_32x32xn[(DAV1D_PIXEL_LAYOUT_I422 - 1) as usize] = Some(dav1d_fguv_32x32xn_i422_16bpc_avx512icl);
    (*c).fguv_32x32xn[(DAV1D_PIXEL_LAYOUT_I444 - 1) as usize] = Some(dav1d_fguv_32x32xn_i444_16bpc_avx512icl);
}

#[cfg(all(
    feature = "asm",
    any(target_arch = "arm", target_arch = "aarch64"),
))]
#[inline(always)]
unsafe extern "C" fn film_grain_dsp_init_arm(c: *mut Dav1dFilmGrainDSPContext) {
    use crate::src::arm::cpu::DAV1D_ARM_CPU_FLAG_NEON;

    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
        return;
    }

    (*c).generate_grain_y = Some(dav1d_generate_grain_y_16bpc_neon);
    (*c).generate_grain_uv[(DAV1D_PIXEL_LAYOUT_I420 - 1) as usize] = Some(dav1d_generate_grain_uv_420_16bpc_neon);
    (*c).generate_grain_uv[(DAV1D_PIXEL_LAYOUT_I422 - 1) as usize] = Some(dav1d_generate_grain_uv_422_16bpc_neon);
    (*c).generate_grain_uv[(DAV1D_PIXEL_LAYOUT_I444 - 1) as usize] = Some(dav1d_generate_grain_uv_444_16bpc_neon);

    (*c).fgy_32x32xn = Some(fgy_32x32xn_neon);
    (*c).fguv_32x32xn[(DAV1D_PIXEL_LAYOUT_I420 - 1) as usize] = Some(fguv_32x32xn_420_neon);
    (*c).fguv_32x32xn[(DAV1D_PIXEL_LAYOUT_I422 - 1) as usize] = Some(fguv_32x32xn_422_neon);
    (*c).fguv_32x32xn[(DAV1D_PIXEL_LAYOUT_I444 - 1) as usize] = Some(fguv_32x32xn_444_neon);
}

#[cfg(all(
    feature = "asm",
    any(target_arch = "arm", target_arch = "aarch64"),
))]
unsafe extern "C" fn fgy_32x32xn_neon(
    dst_row: *mut pixel,
    src_row: *const pixel,
    stride: ptrdiff_t,
    data: *const Dav1dFilmGrainData,
    pw: size_t,
    mut scaling: *const uint8_t,
    mut grain_lut: *const [entry; 82],
    bh: libc::c_int,
    row_num: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let rows: libc::c_int = 1 as libc::c_int
        + ((*data).overlap_flag != 0 && row_num > 0) as libc::c_int;
    let mut seed: [libc::c_uint; 2] = [0; 2];
    let mut i = 0;
    while i < rows {
        seed[i as usize] = (*data).seed;
        seed[i as usize]
            ^= (((row_num - i) * 37 + 178
                & 0xff as libc::c_int) << 8) as libc::c_uint;
        seed[i as usize]
            ^= ((row_num - i) * 173 + 105
                & 0xff as libc::c_int) as libc::c_uint;
        i += 1;
    }
    let mut offsets: [[libc::c_int; 2]; 2] = [[0; 2]; 2];
    let mut bx: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (bx as size_t) < pw {
        if (*data).overlap_flag != 0 && bx != 0 {
            let mut i_0 = 0;
            while i_0 < rows {
                offsets[1 as libc::c_int
                    as usize][i_0
                    as usize] = offsets[0][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1 = 0;
        while i_1 < rows {
            offsets[0 as libc::c_int
                as usize][i_1
                as usize] = get_random_number(
                8 as libc::c_int,
                &mut *seed.as_mut_ptr().offset(i_1 as isize),
            );
            i_1 += 1;
        }
        let mut type_0 = 0;
        if (*data).overlap_flag != 0 && row_num != 0 {
            type_0 |= 1 as libc::c_int;
        }
        if (*data).overlap_flag != 0 && bx != 0 {
            type_0 |= 2 as libc::c_int;
        }
        dav1d_fgy_32x32_16bpc_neon(
            dst_row.offset(bx as isize),
            src_row.offset(bx as isize),
            stride,
            scaling,
            (*data).scaling_shift,
            grain_lut,
            offsets.as_mut_ptr() as *const [libc::c_int; 2],
            bh,
            (*data).clip_to_restricted_range as ptrdiff_t,
            type_0 as ptrdiff_t,
            bitdepth_max,
        );
        bx = bx.wrapping_add(32 as libc::c_int as libc::c_uint);
    }
}

#[cfg(all(
    feature = "asm",
    any(target_arch = "arm", target_arch = "aarch64"),
))]
unsafe extern "C" fn fguv_32x32xn_420_neon(
    dst_row: *mut pixel,
    src_row: *const pixel,
    stride: ptrdiff_t,
    data: *const Dav1dFilmGrainData,
    pw: size_t,
    mut scaling: *const uint8_t,
    mut grain_lut: *const [entry; 82],
    bh: libc::c_int,
    row_num: libc::c_int,
    luma_row: *const pixel,
    luma_stride: ptrdiff_t,
    uv: libc::c_int,
    is_id: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let rows: libc::c_int = 1 as libc::c_int
        + ((*data).overlap_flag != 0 && row_num > 0) as libc::c_int;
    let mut seed: [libc::c_uint; 2] = [0; 2];
    let mut i = 0;
    while i < rows {
        seed[i as usize] = (*data).seed;
        seed[i as usize]
            ^= (((row_num - i) * 37 + 178
                & 0xff as libc::c_int) << 8) as libc::c_uint;
        seed[i as usize]
            ^= ((row_num - i) * 173 + 105
                & 0xff as libc::c_int) as libc::c_uint;
        i += 1;
    }
    let mut offsets: [[libc::c_int; 2]; 2] = [[0; 2]; 2];
    let mut bx: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (bx as size_t) < pw {
        if (*data).overlap_flag != 0 && bx != 0 {
            let mut i_0 = 0;
            while i_0 < rows {
                offsets[1 as libc::c_int
                    as usize][i_0
                    as usize] = offsets[0][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1 = 0;
        while i_1 < rows {
            offsets[0 as libc::c_int
                as usize][i_1
                as usize] = get_random_number(
                8 as libc::c_int,
                &mut *seed.as_mut_ptr().offset(i_1 as isize),
            );
            i_1 += 1;
        }
        let mut type_0 = 0;
        if (*data).overlap_flag != 0 && row_num != 0 {
            type_0 |= 1 as libc::c_int;
        }
        if (*data).overlap_flag != 0 && bx != 0 {
            type_0 |= 2 as libc::c_int;
        }
        if (*data).chroma_scaling_from_luma != 0 {
            type_0 |= 4 as libc::c_int;
        }
        dav1d_fguv_32x32_420_16bpc_neon(
            dst_row.offset(bx as isize),
            src_row.offset(bx as isize),
            stride,
            scaling,
            data,
            grain_lut,
            luma_row.offset((bx << 1) as isize),
            luma_stride,
            offsets.as_mut_ptr() as *const [libc::c_int; 2],
            bh as ptrdiff_t,
            uv as ptrdiff_t,
            is_id as ptrdiff_t,
            type_0 as ptrdiff_t,
            bitdepth_max,
        );
        bx = bx.wrapping_add((32 >> 1) as libc::c_uint);
    }
}

#[cfg(all(
    feature = "asm",
    any(target_arch = "arm", target_arch = "aarch64"),
))]
unsafe extern "C" fn fguv_32x32xn_422_neon(
    dst_row: *mut pixel,
    src_row: *const pixel,
    stride: ptrdiff_t,
    data: *const Dav1dFilmGrainData,
    pw: size_t,
    mut scaling: *const uint8_t,
    mut grain_lut: *const [entry; 82],
    bh: libc::c_int,
    row_num: libc::c_int,
    luma_row: *const pixel,
    luma_stride: ptrdiff_t,
    uv: libc::c_int,
    is_id: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let rows: libc::c_int = 1 as libc::c_int
        + ((*data).overlap_flag != 0 && row_num > 0) as libc::c_int;
    let mut seed: [libc::c_uint; 2] = [0; 2];
    let mut i = 0;
    while i < rows {
        seed[i as usize] = (*data).seed;
        seed[i as usize]
            ^= (((row_num - i) * 37 + 178
                & 0xff as libc::c_int) << 8) as libc::c_uint;
        seed[i as usize]
            ^= ((row_num - i) * 173 + 105
                & 0xff as libc::c_int) as libc::c_uint;
        i += 1;
    }
    let mut offsets: [[libc::c_int; 2]; 2] = [[0; 2]; 2];
    let mut bx: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (bx as size_t) < pw {
        if (*data).overlap_flag != 0 && bx != 0 {
            let mut i_0 = 0;
            while i_0 < rows {
                offsets[1 as libc::c_int
                    as usize][i_0
                    as usize] = offsets[0][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1 = 0;
        while i_1 < rows {
            offsets[0 as libc::c_int
                as usize][i_1
                as usize] = get_random_number(
                8 as libc::c_int,
                &mut *seed.as_mut_ptr().offset(i_1 as isize),
            );
            i_1 += 1;
        }
        let mut type_0 = 0;
        if (*data).overlap_flag != 0 && row_num != 0 {
            type_0 |= 1 as libc::c_int;
        }
        if (*data).overlap_flag != 0 && bx != 0 {
            type_0 |= 2 as libc::c_int;
        }
        if (*data).chroma_scaling_from_luma != 0 {
            type_0 |= 4 as libc::c_int;
        }
        dav1d_fguv_32x32_422_16bpc_neon(
            dst_row.offset(bx as isize),
            src_row.offset(bx as isize),
            stride,
            scaling,
            data,
            grain_lut,
            luma_row.offset((bx << 1) as isize),
            luma_stride,
            offsets.as_mut_ptr() as *const [libc::c_int; 2],
            bh as ptrdiff_t,
            uv as ptrdiff_t,
            is_id as ptrdiff_t,
            type_0 as ptrdiff_t,
            bitdepth_max,
        );
        bx = bx.wrapping_add((32 >> 1) as libc::c_uint);
    }
}

#[cfg(all(
    feature = "asm",
    any(target_arch = "arm", target_arch = "aarch64"),
))]
unsafe extern "C" fn fguv_32x32xn_444_neon(
    dst_row: *mut pixel,
    src_row: *const pixel,
    stride: ptrdiff_t,
    data: *const Dav1dFilmGrainData,
    pw: size_t,
    mut scaling: *const uint8_t,
    mut grain_lut: *const [entry; 82],
    bh: libc::c_int,
    row_num: libc::c_int,
    luma_row: *const pixel,
    luma_stride: ptrdiff_t,
    uv: libc::c_int,
    is_id: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let rows: libc::c_int = 1 as libc::c_int
        + ((*data).overlap_flag != 0 && row_num > 0) as libc::c_int;
    let mut seed: [libc::c_uint; 2] = [0; 2];
    let mut i = 0;
    while i < rows {
        seed[i as usize] = (*data).seed;
        seed[i as usize]
            ^= (((row_num - i) * 37 + 178
                & 0xff as libc::c_int) << 8) as libc::c_uint;
        seed[i as usize]
            ^= ((row_num - i) * 173 + 105
                & 0xff as libc::c_int) as libc::c_uint;
        i += 1;
    }
    let mut offsets: [[libc::c_int; 2]; 2] = [[0; 2]; 2];
    let mut bx: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (bx as size_t) < pw {
        if (*data).overlap_flag != 0 && bx != 0 {
            let mut i_0 = 0;
            while i_0 < rows {
                offsets[1 as libc::c_int
                    as usize][i_0
                    as usize] = offsets[0][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1 = 0;
        while i_1 < rows {
            offsets[0 as libc::c_int
                as usize][i_1
                as usize] = get_random_number(
                8 as libc::c_int,
                &mut *seed.as_mut_ptr().offset(i_1 as isize),
            );
            i_1 += 1;
        }
        let mut type_0 = 0;
        if (*data).overlap_flag != 0 && row_num != 0 {
            type_0 |= 1 as libc::c_int;
        }
        if (*data).overlap_flag != 0 && bx != 0 {
            type_0 |= 2 as libc::c_int;
        }
        if (*data).chroma_scaling_from_luma != 0 {
            type_0 |= 4 as libc::c_int;
        }
        dav1d_fguv_32x32_444_16bpc_neon(
            dst_row.offset(bx as isize),
            src_row.offset(bx as isize),
            stride,
            scaling,
            data,
            grain_lut,
            luma_row.offset((bx << 0) as isize),
            luma_stride,
            offsets.as_mut_ptr() as *const [libc::c_int; 2],
            bh as ptrdiff_t,
            uv as ptrdiff_t,
            is_id as ptrdiff_t,
            type_0 as ptrdiff_t,
            bitdepth_max,
        );
        bx = bx.wrapping_add((32 >> 0) as libc::c_uint);
    }
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_film_grain_dsp_init_16bpc(c: *mut Dav1dFilmGrainDSPContext) {
    (*c).generate_grain_y = Some(generate_grain_y_c);
    (*c).generate_grain_uv[(DAV1D_PIXEL_LAYOUT_I420 - 1) as usize] = Some(generate_grain_uv_420_c);
    (*c).generate_grain_uv[(DAV1D_PIXEL_LAYOUT_I422 - 1) as usize] = Some(generate_grain_uv_422_c);
    (*c).generate_grain_uv[(DAV1D_PIXEL_LAYOUT_I444 - 1) as usize] = Some(generate_grain_uv_444_c);

    (*c).fgy_32x32xn = Some(fgy_32x32xn_c);
    (*c).fguv_32x32xn[(DAV1D_PIXEL_LAYOUT_I420 - 1) as usize] = Some(fguv_32x32xn_420_c);
    (*c).fguv_32x32xn[(DAV1D_PIXEL_LAYOUT_I422 - 1) as usize] = Some(fguv_32x32xn_422_c);
    (*c).fguv_32x32xn[(DAV1D_PIXEL_LAYOUT_I444 - 1) as usize] = Some(fguv_32x32xn_444_c);

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            film_grain_dsp_init_x86(c);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
            film_grain_dsp_init_arm(c);
        }
    }
}
