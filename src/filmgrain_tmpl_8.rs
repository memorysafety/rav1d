use ::libc;
extern "C" {
    static dav1d_gaussian_sequence: [int16_t; 2048];
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type intptr_t = libc::c_long;
pub type pixel = uint8_t;
pub type Dav1dPixelLayout = libc::c_uint;
pub const DAV1D_PIXEL_LAYOUT_I444: Dav1dPixelLayout = 3;
pub const DAV1D_PIXEL_LAYOUT_I422: Dav1dPixelLayout = 2;
pub const DAV1D_PIXEL_LAYOUT_I420: Dav1dPixelLayout = 1;
pub const DAV1D_PIXEL_LAYOUT_I400: Dav1dPixelLayout = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFilmGrainData {
    pub seed: libc::c_uint,
    pub num_y_points: libc::c_int,
    pub y_points: [[uint8_t; 2]; 14],
    pub chroma_scaling_from_luma: libc::c_int,
    pub num_uv_points: [libc::c_int; 2],
    pub uv_points: [[[uint8_t; 2]; 10]; 2],
    pub scaling_shift: libc::c_int,
    pub ar_coeff_lag: libc::c_int,
    pub ar_coeffs_y: [int8_t; 24],
    pub ar_coeffs_uv: [[int8_t; 28]; 2],
    pub ar_coeff_shift: uint64_t,
    pub grain_scale_shift: libc::c_int,
    pub uv_mult: [libc::c_int; 2],
    pub uv_luma_mult: [libc::c_int; 2],
    pub uv_offset: [libc::c_int; 2],
    pub overlap_flag: libc::c_int,
    pub clip_to_restricted_range: libc::c_int,
}
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
#[inline]
unsafe extern "C" fn iclip_u8(v: libc::c_int) -> libc::c_int {
    return iclip(v, 0 as libc::c_int, 255 as libc::c_int);
}
#[inline]
unsafe extern "C" fn iclip(
    v: libc::c_int,
    min: libc::c_int,
    max: libc::c_int,
) -> libc::c_int {
    return if v < min { min } else if v > max { max } else { v };
}
#[inline]
unsafe extern "C" fn imin(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn get_random_number(
    bits: libc::c_int,
    state: *mut libc::c_uint,
) -> libc::c_int {
    let r: libc::c_int = *state as libc::c_int;
    let mut bit: libc::c_uint = ((r >> 0 as libc::c_int ^ r >> 1 as libc::c_int
        ^ r >> 3 as libc::c_int ^ r >> 12 as libc::c_int) & 1 as libc::c_int)
        as libc::c_uint;
    *state = (r >> 1 as libc::c_int) as libc::c_uint | bit << 15 as libc::c_int;
    return (*state >> 16 as libc::c_int - bits
        & (((1 as libc::c_int) << bits) - 1 as libc::c_int) as libc::c_uint)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn round2(x: libc::c_int, shift: uint64_t) -> libc::c_int {
    return x + ((1 as libc::c_int) << shift >> 1 as libc::c_int) >> shift;
}
unsafe extern "C" fn generate_grain_y_c(
    mut buf: *mut [entry; 82],
    data: *const Dav1dFilmGrainData,
) {
    let bitdepth_min_8: libc::c_int = 8 as libc::c_int - 8 as libc::c_int;
    let mut seed: libc::c_uint = (*data).seed;
    let shift: libc::c_int = 4 as libc::c_int - bitdepth_min_8
        + (*data).grain_scale_shift;
    let grain_ctr: libc::c_int = (128 as libc::c_int) << bitdepth_min_8;
    let grain_min: libc::c_int = -grain_ctr;
    let grain_max: libc::c_int = grain_ctr - 1 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 73 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 82 as libc::c_int {
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
    let ar_pad: libc::c_int = 3 as libc::c_int;
    let ar_lag: libc::c_int = (*data).ar_coeff_lag;
    let mut y_0: libc::c_int = ar_pad;
    while y_0 < 73 as libc::c_int {
        let mut x_0: libc::c_int = ar_pad;
        while x_0 < 82 as libc::c_int - ar_pad {
            let mut coeff: *const int8_t = ((*data).ar_coeffs_y).as_ptr();
            let mut sum: libc::c_int = 0 as libc::c_int;
            let mut dy: libc::c_int = -ar_lag;
            while dy <= 0 as libc::c_int {
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
) {
    let bitdepth_min_8: libc::c_int = 8 as libc::c_int - 8 as libc::c_int;
    let mut seed: libc::c_uint = (*data).seed
        ^ (if uv != 0 { 0x49d8 as libc::c_int } else { 0xb524 as libc::c_int })
            as libc::c_uint;
    let shift: libc::c_int = 4 as libc::c_int - bitdepth_min_8
        + (*data).grain_scale_shift;
    let grain_ctr: libc::c_int = (128 as libc::c_int) << bitdepth_min_8;
    let grain_min: libc::c_int = -grain_ctr;
    let grain_max: libc::c_int = grain_ctr - 1 as libc::c_int;
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
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < chromaH {
        let mut x: libc::c_int = 0 as libc::c_int;
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
    let ar_pad: libc::c_int = 3 as libc::c_int;
    let ar_lag: libc::c_int = (*data).ar_coeff_lag;
    let mut y_0: libc::c_int = ar_pad;
    while y_0 < chromaH {
        let mut x_0: libc::c_int = ar_pad;
        while x_0 < chromaW - ar_pad {
            let mut coeff: *const int8_t = ((*data).ar_coeffs_uv[uv as usize]).as_ptr();
            let mut sum: libc::c_int = 0 as libc::c_int;
            let mut dy: libc::c_int = -ar_lag;
            while dy <= 0 as libc::c_int {
                let mut dx: libc::c_int = -ar_lag;
                while dx <= ar_lag {
                    if dx == 0 && dy == 0 {
                        if (*data).num_y_points == 0 {
                            break;
                        }
                        let mut luma: libc::c_int = 0 as libc::c_int;
                        let lumaX: libc::c_int = (x_0 - ar_pad << subx) + ar_pad;
                        let lumaY: libc::c_int = (y_0 - ar_pad << suby) + ar_pad;
                        let mut i: libc::c_int = 0 as libc::c_int;
                        while i <= suby {
                            let mut j: libc::c_int = 0 as libc::c_int;
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
) {
    generate_grain_uv_c(buf, buf_y, data, uv, 1 as libc::c_int, 1 as libc::c_int);
}
unsafe extern "C" fn generate_grain_uv_422_c(
    mut buf: *mut [entry; 82],
    mut buf_y: *const [entry; 82],
    data: *const Dav1dFilmGrainData,
    uv: intptr_t,
) {
    generate_grain_uv_c(buf, buf_y, data, uv, 1 as libc::c_int, 0 as libc::c_int);
}
unsafe extern "C" fn generate_grain_uv_444_c(
    mut buf: *mut [entry; 82],
    mut buf_y: *const [entry; 82],
    data: *const Dav1dFilmGrainData,
    uv: intptr_t,
) {
    generate_grain_uv_c(buf, buf_y, data, uv, 0 as libc::c_int, 0 as libc::c_int);
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
        + (2 as libc::c_int >> subx)
            * (3 as libc::c_int + (randval >> 4 as libc::c_int));
    let offy: libc::c_int = 3 as libc::c_int
        + (2 as libc::c_int >> suby)
            * (3 as libc::c_int + (randval & 0xf as libc::c_int));
    return (*grain_lut
        .offset(
            (offy + y + (32 as libc::c_int >> suby) * by) as isize,
        ))[(offx + x + (32 as libc::c_int >> subx) * bx) as usize];
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
) {
    let rows: libc::c_int = 1 as libc::c_int
        + ((*data).overlap_flag != 0 && row_num > 0 as libc::c_int) as libc::c_int;
    let bitdepth_min_8: libc::c_int = 8 as libc::c_int - 8 as libc::c_int;
    let grain_ctr: libc::c_int = (128 as libc::c_int) << bitdepth_min_8;
    let grain_min: libc::c_int = -grain_ctr;
    let grain_max: libc::c_int = grain_ctr - 1 as libc::c_int;
    let mut min_value: libc::c_int = 0;
    let mut max_value: libc::c_int = 0;
    if (*data).clip_to_restricted_range != 0 {
        min_value = (16 as libc::c_int) << bitdepth_min_8;
        max_value = (235 as libc::c_int) << bitdepth_min_8;
    } else {
        min_value = 0 as libc::c_int;
        max_value = 0xff as libc::c_int;
    }
    let mut seed: [libc::c_uint; 2] = [0; 2];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < rows {
        seed[i as usize] = (*data).seed;
        seed[i as usize]
            ^= (((row_num - i) * 37 as libc::c_int + 178 as libc::c_int
                & 0xff as libc::c_int) << 8 as libc::c_int) as libc::c_uint;
        seed[i as usize]
            ^= ((row_num - i) * 173 as libc::c_int + 105 as libc::c_int
                & 0xff as libc::c_int) as libc::c_uint;
        i += 1;
    }
    if !((stride as libc::c_ulong)
        .wrapping_rem(
            (32 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong),
        ) == 0 as libc::c_int as libc::c_ulong)
    {
        unreachable!();
    }
    let mut offsets: [[libc::c_int; 2]; 2] = [[0; 2]; 2];
    let mut bx: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (bx as libc::c_ulong) < pw {
        let bw: libc::c_int = imin(
            32 as libc::c_int,
            (pw as libc::c_int as libc::c_uint).wrapping_sub(bx) as libc::c_int,
        );
        if (*data).overlap_flag != 0 && bx != 0 {
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < rows {
                offsets[1 as libc::c_int
                    as usize][i_0
                    as usize] = offsets[0 as libc::c_int as usize][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1: libc::c_int = 0 as libc::c_int;
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
                    .offset((y as libc::c_long * stride) as isize)
                    .offset(x as isize)
                    .offset(bx as isize);
                let dst: *mut pixel = dst_row
                    .offset((y as libc::c_long * stride) as isize)
                    .offset(x as isize)
                    .offset(bx as isize);
                let noise: libc::c_int = round2(
                    *scaling.offset(*src as isize) as libc::c_int * grain,
                    (*data).scaling_shift as uint64_t,
                );
                *dst = iclip(*src as libc::c_int + noise, min_value, max_value) as pixel;
                x += 1;
            }
            let mut x_0: libc::c_int = 0 as libc::c_int;
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
                    old * w[x_0 as usize][0 as libc::c_int as usize]
                        + grain_0 * w[x_0 as usize][1 as libc::c_int as usize],
                    5 as libc::c_int as uint64_t,
                );
                grain_0 = iclip(grain_0, grain_min, grain_max);
                let src_0: *const pixel = src_row
                    .offset((y as libc::c_long * stride) as isize)
                    .offset(x_0 as isize)
                    .offset(bx as isize);
                let dst_0: *mut pixel = dst_row
                    .offset((y as libc::c_long * stride) as isize)
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
        let mut y_0: libc::c_int = 0 as libc::c_int;
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
                    old_0 * w[y_0 as usize][0 as libc::c_int as usize]
                        + grain_1 * w[y_0 as usize][1 as libc::c_int as usize],
                    5 as libc::c_int as uint64_t,
                );
                grain_1 = iclip(grain_1, grain_min, grain_max);
                let src_1: *const pixel = src_row
                    .offset((y_0 as libc::c_long * stride) as isize)
                    .offset(x_1 as isize)
                    .offset(bx as isize);
                let dst_1: *mut pixel = dst_row
                    .offset((y_0 as libc::c_long * stride) as isize)
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
            let mut x_2: libc::c_int = 0 as libc::c_int;
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
                    old_1 * w[x_2 as usize][0 as libc::c_int as usize]
                        + top * w[x_2 as usize][1 as libc::c_int as usize],
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
                    old_1 * w[x_2 as usize][0 as libc::c_int as usize]
                        + grain_2 * w[x_2 as usize][1 as libc::c_int as usize],
                    5 as libc::c_int as uint64_t,
                );
                grain_2 = iclip(grain_2, grain_min, grain_max);
                grain_2 = round2(
                    top * w[y_0 as usize][0 as libc::c_int as usize]
                        + grain_2 * w[y_0 as usize][1 as libc::c_int as usize],
                    5 as libc::c_int as uint64_t,
                );
                grain_2 = iclip(grain_2, grain_min, grain_max);
                let src_2: *const pixel = src_row
                    .offset((y_0 as libc::c_long * stride) as isize)
                    .offset(x_2 as isize)
                    .offset(bx as isize);
                let dst_2: *mut pixel = dst_row
                    .offset((y_0 as libc::c_long * stride) as isize)
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
) {
    let rows: libc::c_int = 1 as libc::c_int
        + ((*data).overlap_flag != 0 && row_num > 0 as libc::c_int) as libc::c_int;
    let bitdepth_min_8: libc::c_int = 8 as libc::c_int - 8 as libc::c_int;
    let grain_ctr: libc::c_int = (128 as libc::c_int) << bitdepth_min_8;
    let grain_min: libc::c_int = -grain_ctr;
    let grain_max: libc::c_int = grain_ctr - 1 as libc::c_int;
    let mut min_value: libc::c_int = 0;
    let mut max_value: libc::c_int = 0;
    if (*data).clip_to_restricted_range != 0 {
        min_value = (16 as libc::c_int) << bitdepth_min_8;
        max_value = (if is_id != 0 { 235 as libc::c_int } else { 240 as libc::c_int })
            << bitdepth_min_8;
    } else {
        min_value = 0 as libc::c_int;
        max_value = 0xff as libc::c_int;
    }
    let mut seed: [libc::c_uint; 2] = [0; 2];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < rows {
        seed[i as usize] = (*data).seed;
        seed[i as usize]
            ^= (((row_num - i) * 37 as libc::c_int + 178 as libc::c_int
                & 0xff as libc::c_int) << 8 as libc::c_int) as libc::c_uint;
        seed[i as usize]
            ^= ((row_num - i) * 173 as libc::c_int + 105 as libc::c_int
                & 0xff as libc::c_int) as libc::c_uint;
        i += 1;
    }
    if !((stride as libc::c_ulong)
        .wrapping_rem(
            (32 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong),
        ) == 0 as libc::c_int as libc::c_ulong)
    {
        unreachable!();
    }
    let mut offsets: [[libc::c_int; 2]; 2] = [[0; 2]; 2];
    let mut bx: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (bx as libc::c_ulong) < pw {
        let bw: libc::c_int = imin(
            32 as libc::c_int >> sx,
            pw.wrapping_sub(bx as libc::c_ulong) as libc::c_int,
        );
        if (*data).overlap_flag != 0 && bx != 0 {
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < rows {
                offsets[1 as libc::c_int
                    as usize][i_0
                    as usize] = offsets[0 as libc::c_int as usize][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1: libc::c_int = 0 as libc::c_int;
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
            imin(2 as libc::c_int >> sy, bh)
        } else {
            0 as libc::c_int
        };
        let xstart: libc::c_int = if (*data).overlap_flag != 0 && bx != 0 {
            imin(2 as libc::c_int >> sx, bw)
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
                    .offset((ly as libc::c_long * luma_stride) as isize)
                    .offset(lx as isize);
                let mut avg: pixel = *luma.offset(0 as libc::c_int as isize);
                if sx != 0 {
                    avg = (avg as libc::c_int
                        + *luma.offset(1 as libc::c_int as isize) as libc::c_int
                        + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
                }
                let src: *const pixel = src_row
                    .offset((y as libc::c_long * stride) as isize)
                    .offset(bx.wrapping_add(x as libc::c_uint) as isize);
                let dst: *mut pixel = dst_row
                    .offset((y as libc::c_long * stride) as isize)
                    .offset(bx.wrapping_add(x as libc::c_uint) as isize);
                let mut val: libc::c_int = avg as libc::c_int;
                if (*data).chroma_scaling_from_luma == 0 {
                    let combined: libc::c_int = avg as libc::c_int
                        * (*data).uv_luma_mult[uv as usize]
                        + *src as libc::c_int * (*data).uv_mult[uv as usize];
                    val = iclip_u8(
                        (combined >> 6 as libc::c_int)
                            + (*data).uv_offset[uv as usize]
                                * ((1 as libc::c_int) << bitdepth_min_8),
                    );
                }
                let noise: libc::c_int = round2(
                    *scaling.offset(val as isize) as libc::c_int * grain,
                    (*data).scaling_shift as uint64_t,
                );
                *dst = iclip(*src as libc::c_int + noise, min_value, max_value) as pixel;
                x += 1;
            }
            let mut x_0: libc::c_int = 0 as libc::c_int;
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
                    old * w[sx as usize][x_0 as usize][0 as libc::c_int as usize]
                        + grain_0
                            * w[sx as usize][x_0 as usize][1 as libc::c_int as usize],
                    5 as libc::c_int as uint64_t,
                );
                grain_0 = iclip(grain_0, grain_min, grain_max);
                let lx_0: libc::c_int = (bx.wrapping_add(x_0 as libc::c_uint) << sx)
                    as libc::c_int;
                let ly_0: libc::c_int = y << sy;
                let luma_0: *const pixel = luma_row
                    .offset((ly_0 as libc::c_long * luma_stride) as isize)
                    .offset(lx_0 as isize);
                let mut avg_0: pixel = *luma_0.offset(0 as libc::c_int as isize);
                if sx != 0 {
                    avg_0 = (avg_0 as libc::c_int
                        + *luma_0.offset(1 as libc::c_int as isize) as libc::c_int
                        + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
                }
                let src_0: *const pixel = src_row
                    .offset((y as libc::c_long * stride) as isize)
                    .offset(bx.wrapping_add(x_0 as libc::c_uint) as isize);
                let dst_0: *mut pixel = dst_row
                    .offset((y as libc::c_long * stride) as isize)
                    .offset(bx.wrapping_add(x_0 as libc::c_uint) as isize);
                let mut val_0: libc::c_int = avg_0 as libc::c_int;
                if (*data).chroma_scaling_from_luma == 0 {
                    let combined_0: libc::c_int = avg_0 as libc::c_int
                        * (*data).uv_luma_mult[uv as usize]
                        + *src_0 as libc::c_int * (*data).uv_mult[uv as usize];
                    val_0 = iclip_u8(
                        (combined_0 >> 6 as libc::c_int)
                            + (*data).uv_offset[uv as usize]
                                * ((1 as libc::c_int) << bitdepth_min_8),
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
        let mut y_0: libc::c_int = 0 as libc::c_int;
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
                    old_0 * w[sy as usize][y_0 as usize][0 as libc::c_int as usize]
                        + grain_1
                            * w[sy as usize][y_0 as usize][1 as libc::c_int as usize],
                    5 as libc::c_int as uint64_t,
                );
                grain_1 = iclip(grain_1, grain_min, grain_max);
                let lx_1: libc::c_int = (bx.wrapping_add(x_1 as libc::c_uint) << sx)
                    as libc::c_int;
                let ly_1: libc::c_int = y_0 << sy;
                let luma_1: *const pixel = luma_row
                    .offset((ly_1 as libc::c_long * luma_stride) as isize)
                    .offset(lx_1 as isize);
                let mut avg_1: pixel = *luma_1.offset(0 as libc::c_int as isize);
                if sx != 0 {
                    avg_1 = (avg_1 as libc::c_int
                        + *luma_1.offset(1 as libc::c_int as isize) as libc::c_int
                        + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
                }
                let src_1: *const pixel = src_row
                    .offset((y_0 as libc::c_long * stride) as isize)
                    .offset(bx.wrapping_add(x_1 as libc::c_uint) as isize);
                let dst_1: *mut pixel = dst_row
                    .offset((y_0 as libc::c_long * stride) as isize)
                    .offset(bx.wrapping_add(x_1 as libc::c_uint) as isize);
                let mut val_1: libc::c_int = avg_1 as libc::c_int;
                if (*data).chroma_scaling_from_luma == 0 {
                    let combined_1: libc::c_int = avg_1 as libc::c_int
                        * (*data).uv_luma_mult[uv as usize]
                        + *src_1 as libc::c_int * (*data).uv_mult[uv as usize];
                    val_1 = iclip_u8(
                        (combined_1 >> 6 as libc::c_int)
                            + (*data).uv_offset[uv as usize]
                                * ((1 as libc::c_int) << bitdepth_min_8),
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
            let mut x_2: libc::c_int = 0 as libc::c_int;
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
                    old_1 * w[sx as usize][x_2 as usize][0 as libc::c_int as usize]
                        + top * w[sx as usize][x_2 as usize][1 as libc::c_int as usize],
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
                    old_1 * w[sx as usize][x_2 as usize][0 as libc::c_int as usize]
                        + grain_2
                            * w[sx as usize][x_2 as usize][1 as libc::c_int as usize],
                    5 as libc::c_int as uint64_t,
                );
                grain_2 = iclip(grain_2, grain_min, grain_max);
                grain_2 = round2(
                    top * w[sy as usize][y_0 as usize][0 as libc::c_int as usize]
                        + grain_2
                            * w[sy as usize][y_0 as usize][1 as libc::c_int as usize],
                    5 as libc::c_int as uint64_t,
                );
                grain_2 = iclip(grain_2, grain_min, grain_max);
                let lx_2: libc::c_int = (bx.wrapping_add(x_2 as libc::c_uint) << sx)
                    as libc::c_int;
                let ly_2: libc::c_int = y_0 << sy;
                let luma_2: *const pixel = luma_row
                    .offset((ly_2 as libc::c_long * luma_stride) as isize)
                    .offset(lx_2 as isize);
                let mut avg_2: pixel = *luma_2.offset(0 as libc::c_int as isize);
                if sx != 0 {
                    avg_2 = (avg_2 as libc::c_int
                        + *luma_2.offset(1 as libc::c_int as isize) as libc::c_int
                        + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
                }
                let src_2: *const pixel = src_row
                    .offset((y_0 as libc::c_long * stride) as isize)
                    .offset(bx.wrapping_add(x_2 as libc::c_uint) as isize);
                let dst_2: *mut pixel = dst_row
                    .offset((y_0 as libc::c_long * stride) as isize)
                    .offset(bx.wrapping_add(x_2 as libc::c_uint) as isize);
                let mut val_2: libc::c_int = avg_2 as libc::c_int;
                if (*data).chroma_scaling_from_luma == 0 {
                    let combined_2: libc::c_int = avg_2 as libc::c_int
                        * (*data).uv_luma_mult[uv as usize]
                        + *src_2 as libc::c_int * (*data).uv_mult[uv as usize];
                    val_2 = iclip_u8(
                        (combined_2 >> 6 as libc::c_int)
                            + (*data).uv_offset[uv as usize]
                                * ((1 as libc::c_int) << bitdepth_min_8),
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
        bx = bx.wrapping_add((32 as libc::c_int >> sx) as libc::c_uint);
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
    );
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_film_grain_dsp_init_8bpc(
    c: *mut Dav1dFilmGrainDSPContext,
) {
    (*c)
        .generate_grain_y = Some(
        generate_grain_y_c
            as unsafe extern "C" fn(*mut [entry; 82], *const Dav1dFilmGrainData) -> (),
    );
    (*c)
        .generate_grain_uv[(DAV1D_PIXEL_LAYOUT_I420 as libc::c_int - 1 as libc::c_int)
        as usize] = Some(
        generate_grain_uv_420_c
            as unsafe extern "C" fn(
                *mut [entry; 82],
                *const [entry; 82],
                *const Dav1dFilmGrainData,
                intptr_t,
            ) -> (),
    );
    (*c)
        .generate_grain_uv[(DAV1D_PIXEL_LAYOUT_I422 as libc::c_int - 1 as libc::c_int)
        as usize] = Some(
        generate_grain_uv_422_c
            as unsafe extern "C" fn(
                *mut [entry; 82],
                *const [entry; 82],
                *const Dav1dFilmGrainData,
                intptr_t,
            ) -> (),
    );
    (*c)
        .generate_grain_uv[(DAV1D_PIXEL_LAYOUT_I444 as libc::c_int - 1 as libc::c_int)
        as usize] = Some(
        generate_grain_uv_444_c
            as unsafe extern "C" fn(
                *mut [entry; 82],
                *const [entry; 82],
                *const Dav1dFilmGrainData,
                intptr_t,
            ) -> (),
    );
    (*c)
        .fgy_32x32xn = Some(
        fgy_32x32xn_c
            as unsafe extern "C" fn(
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
    );
    (*c)
        .fguv_32x32xn[(DAV1D_PIXEL_LAYOUT_I420 as libc::c_int - 1 as libc::c_int)
        as usize] = Some(
        fguv_32x32xn_420_c
            as unsafe extern "C" fn(
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
    );
    (*c)
        .fguv_32x32xn[(DAV1D_PIXEL_LAYOUT_I422 as libc::c_int - 1 as libc::c_int)
        as usize] = Some(
        fguv_32x32xn_422_c
            as unsafe extern "C" fn(
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
    );
    (*c)
        .fguv_32x32xn[(DAV1D_PIXEL_LAYOUT_I444 as libc::c_int - 1 as libc::c_int)
        as usize] = Some(
        fguv_32x32xn_444_c
            as unsafe extern "C" fn(
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
    );
}
