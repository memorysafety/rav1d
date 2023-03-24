use ::libc;
extern "C" {
    static dav1d_gaussian_sequence: [int16_t; 2048];
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type intptr_t = libc::c_long;
pub type pixel = uint16_t;
pub type Dav1dPixelLayout = libc::c_uint;
pub const DAV1D_PIXEL_LAYOUT_I444: Dav1dPixelLayout = 3;
pub const DAV1D_PIXEL_LAYOUT_I422: Dav1dPixelLayout = 2;
pub const DAV1D_PIXEL_LAYOUT_I420: Dav1dPixelLayout = 1;
pub const DAV1D_PIXEL_LAYOUT_I400: Dav1dPixelLayout = 0;

#[repr(C)]
#[derive(Copy, Clone)]
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
pub type entry = int16_t;
pub type generate_grain_y_fn =
    Option<unsafe extern "C" fn(*mut [entry; 82], *const Dav1dFilmGrainData, libc::c_int) -> ()>;
pub type generate_grain_uv_fn = Option<
    unsafe extern "C" fn(
        *mut [entry; 82],
        *const [entry; 82],
        *const Dav1dFilmGrainData,
        intptr_t,
        libc::c_int,
    ) -> (),
>;
pub type fgy_32x32xn_fn = Option<
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
pub type fguv_32x32xn_fn = Option<
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav1dFilmGrainDSPContext {
    pub generate_grain_y: generate_grain_y_fn,
    pub generate_grain_uv: [generate_grain_uv_fn; 3],
    pub fgy_32x32xn: fgy_32x32xn_fn,
    pub fguv_32x32xn: [fguv_32x32xn_fn; 3],
}
#[inline]
unsafe extern "C" fn clz(mask: libc::c_uint) -> libc::c_int {
    return mask.leading_zeros() as i32;
}
#[inline]
unsafe extern "C" fn iclip(v: libc::c_int, min: libc::c_int, max: libc::c_int) -> libc::c_int {
    return if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    };
}
#[inline]
unsafe extern "C" fn imin(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1i64 != 0 {
        unreachable!();
    }
    return x >> 1i32;
}
#[inline]
unsafe extern "C" fn get_random_number(bits: libc::c_int, state: *mut libc::c_uint) -> libc::c_int {
    let r: libc::c_int = *state as libc::c_int;
    let mut bit: libc::c_uint =
        ((r >> 0i32 ^ r >> 1i32 ^ r >> 3i32 ^ r >> 12i32) & 1i32) as libc::c_uint;
    *state = (r >> 1i32) as libc::c_uint | bit << 15i32;
    return (*state >> 16i32 - bits & (((1i32) << bits) - 1i32) as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn round2(x: libc::c_int, shift: uint64_t) -> libc::c_int {
    return x + ((1i32) << shift >> 1i32) >> shift;
}
unsafe extern "C" fn generate_grain_y_c(
    mut buf: *mut [entry; 82],
    data: *const Dav1dFilmGrainData,
    bitdepth_max: libc::c_int,
) {
    let bitdepth_min_8: libc::c_int = 32i32 - clz(bitdepth_max as libc::c_uint) - 8i32;
    let mut seed: libc::c_uint = (*data).seed;
    let shift: libc::c_int = 4i32 - bitdepth_min_8 + (*data).grain_scale_shift;
    let grain_ctr: libc::c_int = (128i32) << bitdepth_min_8;
    let grain_min: libc::c_int = -grain_ctr;
    let grain_max: libc::c_int = grain_ctr - 1i32;
    let mut y: libc::c_int = 0i32;
    while y < 73i32 {
        let mut x: libc::c_int = 0i32;
        while x < 82i32 {
            let value: libc::c_int = get_random_number(11i32, &mut seed);
            (*buf.offset(y as isize))[x as usize] = round2(
                dav1d_gaussian_sequence[value as usize] as libc::c_int,
                shift as uint64_t,
            ) as entry;
            x += 1;
        }
        y += 1;
    }
    let ar_pad: libc::c_int = 3i32;
    let ar_lag: libc::c_int = (*data).ar_coeff_lag;
    let mut y_0: libc::c_int = ar_pad;
    while y_0 < 73i32 {
        let mut x_0: libc::c_int = ar_pad;
        while x_0 < 82i32 - ar_pad {
            let mut coeff: *const int8_t = ((*data).ar_coeffs_y).as_ptr();
            let mut sum: libc::c_int = 0i32;
            let mut dy: libc::c_int = -ar_lag;
            while dy <= 0i32 {
                let mut dx: libc::c_int = -ar_lag;
                while dx <= ar_lag {
                    if dx == 0 && dy == 0 {
                        break;
                    }
                    let fresh0 = coeff;
                    coeff = coeff.offset(1);
                    sum += *fresh0 as libc::c_int
                        * (*buf.offset((y_0 + dy) as isize))[(x_0 + dx) as usize] as libc::c_int;
                    dx += 1;
                }
                dy += 1;
            }
            let grain: libc::c_int = (*buf.offset(y_0 as isize))[x_0 as usize] as libc::c_int
                + round2(sum, (*data).ar_coeff_shift);
            (*buf.offset(y_0 as isize))[x_0 as usize] = iclip(grain, grain_min, grain_max) as entry;
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
    let bitdepth_min_8: libc::c_int = 32i32 - clz(bitdepth_max as libc::c_uint) - 8i32;
    let mut seed: libc::c_uint =
        (*data).seed ^ (if uv != 0 { 0x49d8i32 } else { 0xb524i32 }) as libc::c_uint;
    let shift: libc::c_int = 4i32 - bitdepth_min_8 + (*data).grain_scale_shift;
    let grain_ctr: libc::c_int = (128i32) << bitdepth_min_8;
    let grain_min: libc::c_int = -grain_ctr;
    let grain_max: libc::c_int = grain_ctr - 1i32;
    let chromaW: libc::c_int = if subx != 0 { 44i32 } else { 82i32 };
    let chromaH: libc::c_int = if suby != 0 { 38i32 } else { 73i32 };
    let mut y: libc::c_int = 0i32;
    while y < chromaH {
        let mut x: libc::c_int = 0i32;
        while x < chromaW {
            let value: libc::c_int = get_random_number(11i32, &mut seed);
            (*buf.offset(y as isize))[x as usize] = round2(
                dav1d_gaussian_sequence[value as usize] as libc::c_int,
                shift as uint64_t,
            ) as entry;
            x += 1;
        }
        y += 1;
    }
    let ar_pad: libc::c_int = 3i32;
    let ar_lag: libc::c_int = (*data).ar_coeff_lag;
    let mut y_0: libc::c_int = ar_pad;
    while y_0 < chromaH {
        let mut x_0: libc::c_int = ar_pad;
        while x_0 < chromaW - ar_pad {
            let mut coeff: *const int8_t = ((*data).ar_coeffs_uv[uv as usize]).as_ptr();
            let mut sum: libc::c_int = 0i32;
            let mut dy: libc::c_int = -ar_lag;
            while dy <= 0i32 {
                let mut dx: libc::c_int = -ar_lag;
                while dx <= ar_lag {
                    if dx == 0 && dy == 0 {
                        if (*data).num_y_points == 0 {
                            break;
                        }
                        let mut luma: libc::c_int = 0i32;
                        let lumaX: libc::c_int = (x_0 - ar_pad << subx) + ar_pad;
                        let lumaY: libc::c_int = (y_0 - ar_pad << suby) + ar_pad;
                        let mut i: libc::c_int = 0i32;
                        while i <= suby {
                            let mut j: libc::c_int = 0i32;
                            while j <= subx {
                                luma += (*buf_y.offset((lumaY + i) as isize))[(lumaX + j) as usize]
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
                        sum += *fresh1 as libc::c_int
                            * (*buf.offset((y_0 + dy) as isize))[(x_0 + dx) as usize]
                                as libc::c_int;
                        dx += 1;
                    }
                }
                dy += 1;
            }
            let grain: libc::c_int = (*buf.offset(y_0 as isize))[x_0 as usize] as libc::c_int
                + round2(sum, (*data).ar_coeff_shift);
            (*buf.offset(y_0 as isize))[x_0 as usize] = iclip(grain, grain_min, grain_max) as entry;
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
    generate_grain_uv_c(buf, buf_y, data, uv, 1i32, 1i32, bitdepth_max);
}
unsafe extern "C" fn generate_grain_uv_422_c(
    mut buf: *mut [entry; 82],
    mut buf_y: *const [entry; 82],
    data: *const Dav1dFilmGrainData,
    uv: intptr_t,
    bitdepth_max: libc::c_int,
) {
    generate_grain_uv_c(buf, buf_y, data, uv, 1i32, 0i32, bitdepth_max);
}
unsafe extern "C" fn generate_grain_uv_444_c(
    mut buf: *mut [entry; 82],
    mut buf_y: *const [entry; 82],
    data: *const Dav1dFilmGrainData,
    uv: intptr_t,
    bitdepth_max: libc::c_int,
) {
    generate_grain_uv_c(buf, buf_y, data, uv, 0i32, 0i32, bitdepth_max);
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
    let offx: libc::c_int = 3i32 + (2i32 >> subx) * (3i32 + (randval >> 4i32));
    let offy: libc::c_int = 3i32 + (2i32 >> suby) * (3i32 + (randval & 0xfi32));
    return (*grain_lut.offset((offy + y + (32i32 >> suby) * by) as isize))
        [(offx + x + (32i32 >> subx) * bx) as usize];
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
    let rows: libc::c_int = 1i32 + ((*data).overlap_flag != 0 && row_num > 0i32) as libc::c_int;
    let bitdepth_min_8: libc::c_int = 32i32 - clz(bitdepth_max as libc::c_uint) - 8i32;
    let grain_ctr: libc::c_int = (128i32) << bitdepth_min_8;
    let grain_min: libc::c_int = -grain_ctr;
    let grain_max: libc::c_int = grain_ctr - 1i32;
    let mut min_value: libc::c_int = 0;
    let mut max_value: libc::c_int = 0;
    if (*data).clip_to_restricted_range != 0 {
        min_value = (16i32) << bitdepth_min_8;
        max_value = (235i32) << bitdepth_min_8;
    } else {
        min_value = 0i32;
        max_value = bitdepth_max;
    }
    let mut seed: [libc::c_uint; 2] = [0; 2];
    let mut i: libc::c_int = 0i32;
    while i < rows {
        seed[i as usize] = (*data).seed;
        seed[i as usize] ^= (((row_num - i) * 37i32 + 178i32 & 0xffi32) << 8i32) as libc::c_uint;
        seed[i as usize] ^= ((row_num - i) * 173i32 + 105i32 & 0xffi32) as libc::c_uint;
        i += 1;
    }
    if !((stride as libc::c_ulong)
        .wrapping_rem((32u64).wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong))
        == 0u64)
    {
        unreachable!();
    }
    let mut offsets: [[libc::c_int; 2]; 2] = [[0; 2]; 2];
    let mut bx: libc::c_uint = 0u32;
    while (bx as libc::c_ulong) < pw {
        let bw: libc::c_int = imin(32i32, (pw as libc::c_uint).wrapping_sub(bx) as libc::c_int);
        if (*data).overlap_flag != 0 && bx != 0 {
            let mut i_0: libc::c_int = 0i32;
            while i_0 < rows {
                offsets[1usize][i_0 as usize] = offsets[0usize][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1: libc::c_int = 0i32;
        while i_1 < rows {
            offsets[0usize][i_1 as usize] =
                get_random_number(8i32, &mut *seed.as_mut_ptr().offset(i_1 as isize));
            i_1 += 1;
        }
        let ystart: libc::c_int = if (*data).overlap_flag != 0 && row_num != 0 {
            imin(2i32, bh)
        } else {
            0i32
        };
        let xstart: libc::c_int = if (*data).overlap_flag != 0 && bx != 0 {
            imin(2i32, bw)
        } else {
            0i32
        };
        static mut w: [[libc::c_int; 2]; 2] = [[27i32, 17i32], [17i32, 27i32]];
        let mut y: libc::c_int = ystart;
        while y < bh {
            let mut x: libc::c_int = xstart;
            while x < bw {
                let mut grain: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    0i32,
                    0i32,
                    0i32,
                    0i32,
                    x,
                    y,
                ) as libc::c_int;
                let src: *const pixel = src_row
                    .offset((y as libc::c_long * PXSTRIDE(stride)) as isize)
                    .offset(x as isize)
                    .offset(bx as isize);
                let dst: *mut pixel = dst_row
                    .offset((y as libc::c_long * PXSTRIDE(stride)) as isize)
                    .offset(x as isize)
                    .offset(bx as isize);
                let noise: libc::c_int = round2(
                    *scaling.offset(*src as isize) as libc::c_int * grain,
                    (*data).scaling_shift as uint64_t,
                );
                *dst = iclip(*src as libc::c_int + noise, min_value, max_value) as pixel;
                x += 1;
            }
            let mut x_0: libc::c_int = 0i32;
            while x_0 < xstart {
                let mut grain_0: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    0i32,
                    0i32,
                    0i32,
                    0i32,
                    x_0,
                    y,
                ) as libc::c_int;
                let mut old: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    0i32,
                    0i32,
                    1i32,
                    0i32,
                    x_0,
                    y,
                ) as libc::c_int;
                grain_0 = round2(
                    old * w[x_0 as usize][0usize] + grain_0 * w[x_0 as usize][1usize],
                    5u64,
                );
                grain_0 = iclip(grain_0, grain_min, grain_max);
                let src_0: *const pixel = src_row
                    .offset((y as libc::c_long * PXSTRIDE(stride)) as isize)
                    .offset(x_0 as isize)
                    .offset(bx as isize);
                let dst_0: *mut pixel = dst_row
                    .offset((y as libc::c_long * PXSTRIDE(stride)) as isize)
                    .offset(x_0 as isize)
                    .offset(bx as isize);
                let noise_0: libc::c_int = round2(
                    *scaling.offset(*src_0 as isize) as libc::c_int * grain_0,
                    (*data).scaling_shift as uint64_t,
                );
                *dst_0 = iclip(*src_0 as libc::c_int + noise_0, min_value, max_value) as pixel;
                x_0 += 1;
            }
            y += 1;
        }
        let mut y_0: libc::c_int = 0i32;
        while y_0 < ystart {
            let mut x_1: libc::c_int = xstart;
            while x_1 < bw {
                let mut grain_1: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    0i32,
                    0i32,
                    0i32,
                    0i32,
                    x_1,
                    y_0,
                ) as libc::c_int;
                let mut old_0: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    0i32,
                    0i32,
                    0i32,
                    1i32,
                    x_1,
                    y_0,
                ) as libc::c_int;
                grain_1 = round2(
                    old_0 * w[y_0 as usize][0usize] + grain_1 * w[y_0 as usize][1usize],
                    5u64,
                );
                grain_1 = iclip(grain_1, grain_min, grain_max);
                let src_1: *const pixel = src_row
                    .offset((y_0 as libc::c_long * PXSTRIDE(stride)) as isize)
                    .offset(x_1 as isize)
                    .offset(bx as isize);
                let dst_1: *mut pixel = dst_row
                    .offset((y_0 as libc::c_long * PXSTRIDE(stride)) as isize)
                    .offset(x_1 as isize)
                    .offset(bx as isize);
                let noise_1: libc::c_int = round2(
                    *scaling.offset(*src_1 as isize) as libc::c_int * grain_1,
                    (*data).scaling_shift as uint64_t,
                );
                *dst_1 = iclip(*src_1 as libc::c_int + noise_1, min_value, max_value) as pixel;
                x_1 += 1;
            }
            let mut x_2: libc::c_int = 0i32;
            while x_2 < xstart {
                let mut top: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    0i32,
                    0i32,
                    0i32,
                    1i32,
                    x_2,
                    y_0,
                ) as libc::c_int;
                let mut old_1: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    0i32,
                    0i32,
                    1i32,
                    1i32,
                    x_2,
                    y_0,
                ) as libc::c_int;
                top = round2(
                    old_1 * w[x_2 as usize][0usize] + top * w[x_2 as usize][1usize],
                    5u64,
                );
                top = iclip(top, grain_min, grain_max);
                let mut grain_2: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    0i32,
                    0i32,
                    0i32,
                    0i32,
                    x_2,
                    y_0,
                ) as libc::c_int;
                old_1 = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    0i32,
                    0i32,
                    1i32,
                    0i32,
                    x_2,
                    y_0,
                ) as libc::c_int;
                grain_2 = round2(
                    old_1 * w[x_2 as usize][0usize] + grain_2 * w[x_2 as usize][1usize],
                    5u64,
                );
                grain_2 = iclip(grain_2, grain_min, grain_max);
                grain_2 = round2(
                    top * w[y_0 as usize][0usize] + grain_2 * w[y_0 as usize][1usize],
                    5u64,
                );
                grain_2 = iclip(grain_2, grain_min, grain_max);
                let src_2: *const pixel = src_row
                    .offset((y_0 as libc::c_long * PXSTRIDE(stride)) as isize)
                    .offset(x_2 as isize)
                    .offset(bx as isize);
                let dst_2: *mut pixel = dst_row
                    .offset((y_0 as libc::c_long * PXSTRIDE(stride)) as isize)
                    .offset(x_2 as isize)
                    .offset(bx as isize);
                let noise_2: libc::c_int = round2(
                    *scaling.offset(*src_2 as isize) as libc::c_int * grain_2,
                    (*data).scaling_shift as uint64_t,
                );
                *dst_2 = iclip(*src_2 as libc::c_int + noise_2, min_value, max_value) as pixel;
                x_2 += 1;
            }
            y_0 += 1;
        }
        bx = bx.wrapping_add(32u32);
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
    let rows: libc::c_int = 1i32 + ((*data).overlap_flag != 0 && row_num > 0i32) as libc::c_int;
    let bitdepth_min_8: libc::c_int = 32i32 - clz(bitdepth_max as libc::c_uint) - 8i32;
    let grain_ctr: libc::c_int = (128i32) << bitdepth_min_8;
    let grain_min: libc::c_int = -grain_ctr;
    let grain_max: libc::c_int = grain_ctr - 1i32;
    let mut min_value: libc::c_int = 0;
    let mut max_value: libc::c_int = 0;
    if (*data).clip_to_restricted_range != 0 {
        min_value = (16i32) << bitdepth_min_8;
        max_value = (if is_id != 0 { 235i32 } else { 240i32 }) << bitdepth_min_8;
    } else {
        min_value = 0i32;
        max_value = bitdepth_max;
    }
    let mut seed: [libc::c_uint; 2] = [0; 2];
    let mut i: libc::c_int = 0i32;
    while i < rows {
        seed[i as usize] = (*data).seed;
        seed[i as usize] ^= (((row_num - i) * 37i32 + 178i32 & 0xffi32) << 8i32) as libc::c_uint;
        seed[i as usize] ^= ((row_num - i) * 173i32 + 105i32 & 0xffi32) as libc::c_uint;
        i += 1;
    }
    if !((stride as libc::c_ulong)
        .wrapping_rem((32u64).wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong))
        == 0u64)
    {
        unreachable!();
    }
    let mut offsets: [[libc::c_int; 2]; 2] = [[0; 2]; 2];
    let mut bx: libc::c_uint = 0u32;
    while (bx as libc::c_ulong) < pw {
        let bw: libc::c_int = imin(
            32i32 >> sx,
            pw.wrapping_sub(bx as libc::c_ulong) as libc::c_int,
        );
        if (*data).overlap_flag != 0 && bx != 0 {
            let mut i_0: libc::c_int = 0i32;
            while i_0 < rows {
                offsets[1usize][i_0 as usize] = offsets[0usize][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1: libc::c_int = 0i32;
        while i_1 < rows {
            offsets[0usize][i_1 as usize] =
                get_random_number(8i32, &mut *seed.as_mut_ptr().offset(i_1 as isize));
            i_1 += 1;
        }
        let ystart: libc::c_int = if (*data).overlap_flag != 0 && row_num != 0 {
            imin(2i32 >> sy, bh)
        } else {
            0i32
        };
        let xstart: libc::c_int = if (*data).overlap_flag != 0 && bx != 0 {
            imin(2i32 >> sx, bw)
        } else {
            0i32
        };
        static mut w: [[[libc::c_int; 2]; 2]; 2] =
            [[[27i32, 17i32], [17i32, 27i32]], [[23i32, 22i32], [0; 2]]];
        let mut y: libc::c_int = ystart;
        while y < bh {
            let mut x: libc::c_int = xstart;
            while x < bw {
                let mut grain: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    sx,
                    sy,
                    0i32,
                    0i32,
                    x,
                    y,
                ) as libc::c_int;
                let lx: libc::c_int = (bx.wrapping_add(x as libc::c_uint) << sx) as libc::c_int;
                let ly: libc::c_int = y << sy;
                let luma: *const pixel = luma_row
                    .offset((ly as libc::c_long * PXSTRIDE(luma_stride)) as isize)
                    .offset(lx as isize);
                let mut avg: pixel = *luma.offset(0isize);
                if sx != 0 {
                    avg = (avg as libc::c_int + *luma.offset(1isize) as libc::c_int + 1i32 >> 1i32)
                        as pixel;
                }
                let src: *const pixel = src_row
                    .offset((y as libc::c_long * PXSTRIDE(stride)) as isize)
                    .offset(bx.wrapping_add(x as libc::c_uint) as isize);
                let dst: *mut pixel = dst_row
                    .offset((y as libc::c_long * PXSTRIDE(stride)) as isize)
                    .offset(bx.wrapping_add(x as libc::c_uint) as isize);
                let mut val: libc::c_int = avg as libc::c_int;
                if (*data).chroma_scaling_from_luma == 0 {
                    let combined: libc::c_int = avg as libc::c_int
                        * (*data).uv_luma_mult[uv as usize]
                        + *src as libc::c_int * (*data).uv_mult[uv as usize];
                    val = iclip(
                        (combined >> 6i32)
                            + (*data).uv_offset[uv as usize] * ((1i32) << bitdepth_min_8),
                        0i32,
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
            let mut x_0: libc::c_int = 0i32;
            while x_0 < xstart {
                let mut grain_0: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    sx,
                    sy,
                    0i32,
                    0i32,
                    x_0,
                    y,
                ) as libc::c_int;
                let mut old: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    sx,
                    sy,
                    1i32,
                    0i32,
                    x_0,
                    y,
                ) as libc::c_int;
                grain_0 = round2(
                    old * w[sx as usize][x_0 as usize][0usize]
                        + grain_0 * w[sx as usize][x_0 as usize][1usize],
                    5u64,
                );
                grain_0 = iclip(grain_0, grain_min, grain_max);
                let lx_0: libc::c_int = (bx.wrapping_add(x_0 as libc::c_uint) << sx) as libc::c_int;
                let ly_0: libc::c_int = y << sy;
                let luma_0: *const pixel = luma_row
                    .offset((ly_0 as libc::c_long * PXSTRIDE(luma_stride)) as isize)
                    .offset(lx_0 as isize);
                let mut avg_0: pixel = *luma_0.offset(0isize);
                if sx != 0 {
                    avg_0 = (avg_0 as libc::c_int + *luma_0.offset(1isize) as libc::c_int + 1i32
                        >> 1i32) as pixel;
                }
                let src_0: *const pixel = src_row
                    .offset((y as libc::c_long * PXSTRIDE(stride)) as isize)
                    .offset(bx.wrapping_add(x_0 as libc::c_uint) as isize);
                let dst_0: *mut pixel = dst_row
                    .offset((y as libc::c_long * PXSTRIDE(stride)) as isize)
                    .offset(bx.wrapping_add(x_0 as libc::c_uint) as isize);
                let mut val_0: libc::c_int = avg_0 as libc::c_int;
                if (*data).chroma_scaling_from_luma == 0 {
                    let combined_0: libc::c_int = avg_0 as libc::c_int
                        * (*data).uv_luma_mult[uv as usize]
                        + *src_0 as libc::c_int * (*data).uv_mult[uv as usize];
                    val_0 = iclip(
                        (combined_0 >> 6i32)
                            + (*data).uv_offset[uv as usize] * ((1i32) << bitdepth_min_8),
                        0i32,
                        bitdepth_max,
                    );
                }
                let noise_0: libc::c_int = round2(
                    *scaling.offset(val_0 as isize) as libc::c_int * grain_0,
                    (*data).scaling_shift as uint64_t,
                );
                *dst_0 = iclip(*src_0 as libc::c_int + noise_0, min_value, max_value) as pixel;
                x_0 += 1;
            }
            y += 1;
        }
        let mut y_0: libc::c_int = 0i32;
        while y_0 < ystart {
            let mut x_1: libc::c_int = xstart;
            while x_1 < bw {
                let mut grain_1: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    sx,
                    sy,
                    0i32,
                    0i32,
                    x_1,
                    y_0,
                ) as libc::c_int;
                let mut old_0: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    sx,
                    sy,
                    0i32,
                    1i32,
                    x_1,
                    y_0,
                ) as libc::c_int;
                grain_1 = round2(
                    old_0 * w[sy as usize][y_0 as usize][0usize]
                        + grain_1 * w[sy as usize][y_0 as usize][1usize],
                    5u64,
                );
                grain_1 = iclip(grain_1, grain_min, grain_max);
                let lx_1: libc::c_int = (bx.wrapping_add(x_1 as libc::c_uint) << sx) as libc::c_int;
                let ly_1: libc::c_int = y_0 << sy;
                let luma_1: *const pixel = luma_row
                    .offset((ly_1 as libc::c_long * PXSTRIDE(luma_stride)) as isize)
                    .offset(lx_1 as isize);
                let mut avg_1: pixel = *luma_1.offset(0isize);
                if sx != 0 {
                    avg_1 = (avg_1 as libc::c_int + *luma_1.offset(1isize) as libc::c_int + 1i32
                        >> 1i32) as pixel;
                }
                let src_1: *const pixel = src_row
                    .offset((y_0 as libc::c_long * PXSTRIDE(stride)) as isize)
                    .offset(bx.wrapping_add(x_1 as libc::c_uint) as isize);
                let dst_1: *mut pixel = dst_row
                    .offset((y_0 as libc::c_long * PXSTRIDE(stride)) as isize)
                    .offset(bx.wrapping_add(x_1 as libc::c_uint) as isize);
                let mut val_1: libc::c_int = avg_1 as libc::c_int;
                if (*data).chroma_scaling_from_luma == 0 {
                    let combined_1: libc::c_int = avg_1 as libc::c_int
                        * (*data).uv_luma_mult[uv as usize]
                        + *src_1 as libc::c_int * (*data).uv_mult[uv as usize];
                    val_1 = iclip(
                        (combined_1 >> 6i32)
                            + (*data).uv_offset[uv as usize] * ((1i32) << bitdepth_min_8),
                        0i32,
                        bitdepth_max,
                    );
                }
                let noise_1: libc::c_int = round2(
                    *scaling.offset(val_1 as isize) as libc::c_int * grain_1,
                    (*data).scaling_shift as uint64_t,
                );
                *dst_1 = iclip(*src_1 as libc::c_int + noise_1, min_value, max_value) as pixel;
                x_1 += 1;
            }
            let mut x_2: libc::c_int = 0i32;
            while x_2 < xstart {
                let mut top: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    sx,
                    sy,
                    0i32,
                    1i32,
                    x_2,
                    y_0,
                ) as libc::c_int;
                let mut old_1: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    sx,
                    sy,
                    1i32,
                    1i32,
                    x_2,
                    y_0,
                ) as libc::c_int;
                top = round2(
                    old_1 * w[sx as usize][x_2 as usize][0usize]
                        + top * w[sx as usize][x_2 as usize][1usize],
                    5u64,
                );
                top = iclip(top, grain_min, grain_max);
                let mut grain_2: libc::c_int = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    sx,
                    sy,
                    0i32,
                    0i32,
                    x_2,
                    y_0,
                ) as libc::c_int;
                old_1 = sample_lut(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [libc::c_int; 2],
                    sx,
                    sy,
                    1i32,
                    0i32,
                    x_2,
                    y_0,
                ) as libc::c_int;
                grain_2 = round2(
                    old_1 * w[sx as usize][x_2 as usize][0usize]
                        + grain_2 * w[sx as usize][x_2 as usize][1usize],
                    5u64,
                );
                grain_2 = iclip(grain_2, grain_min, grain_max);
                grain_2 = round2(
                    top * w[sy as usize][y_0 as usize][0usize]
                        + grain_2 * w[sy as usize][y_0 as usize][1usize],
                    5u64,
                );
                grain_2 = iclip(grain_2, grain_min, grain_max);
                let lx_2: libc::c_int = (bx.wrapping_add(x_2 as libc::c_uint) << sx) as libc::c_int;
                let ly_2: libc::c_int = y_0 << sy;
                let luma_2: *const pixel = luma_row
                    .offset((ly_2 as libc::c_long * PXSTRIDE(luma_stride)) as isize)
                    .offset(lx_2 as isize);
                let mut avg_2: pixel = *luma_2.offset(0isize);
                if sx != 0 {
                    avg_2 = (avg_2 as libc::c_int + *luma_2.offset(1isize) as libc::c_int + 1i32
                        >> 1i32) as pixel;
                }
                let src_2: *const pixel = src_row
                    .offset((y_0 as libc::c_long * PXSTRIDE(stride)) as isize)
                    .offset(bx.wrapping_add(x_2 as libc::c_uint) as isize);
                let dst_2: *mut pixel = dst_row
                    .offset((y_0 as libc::c_long * PXSTRIDE(stride)) as isize)
                    .offset(bx.wrapping_add(x_2 as libc::c_uint) as isize);
                let mut val_2: libc::c_int = avg_2 as libc::c_int;
                if (*data).chroma_scaling_from_luma == 0 {
                    let combined_2: libc::c_int = avg_2 as libc::c_int
                        * (*data).uv_luma_mult[uv as usize]
                        + *src_2 as libc::c_int * (*data).uv_mult[uv as usize];
                    val_2 = iclip(
                        (combined_2 >> 6i32)
                            + (*data).uv_offset[uv as usize] * ((1i32) << bitdepth_min_8),
                        0i32,
                        bitdepth_max,
                    );
                }
                let noise_2: libc::c_int = round2(
                    *scaling.offset(val_2 as isize) as libc::c_int * grain_2,
                    (*data).scaling_shift as uint64_t,
                );
                *dst_2 = iclip(*src_2 as libc::c_int + noise_2, min_value, max_value) as pixel;
                x_2 += 1;
            }
            y_0 += 1;
        }
        bx = bx.wrapping_add((32i32 >> sx) as libc::c_uint);
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
        1i32,
        1i32,
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
        1i32,
        0i32,
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
        0i32,
        0i32,
        bitdepth_max,
    );
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_film_grain_dsp_init_16bpc(c: *mut Dav1dFilmGrainDSPContext) {
    (*c).generate_grain_y = Some(
        generate_grain_y_c
            as unsafe extern "C" fn(*mut [entry; 82], *const Dav1dFilmGrainData, libc::c_int) -> (),
    );
    (*c).generate_grain_uv[(DAV1D_PIXEL_LAYOUT_I420 as libc::c_int - 1i32) as usize] = Some(
        generate_grain_uv_420_c
            as unsafe extern "C" fn(
                *mut [entry; 82],
                *const [entry; 82],
                *const Dav1dFilmGrainData,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*c).generate_grain_uv[(DAV1D_PIXEL_LAYOUT_I422 as libc::c_int - 1i32) as usize] = Some(
        generate_grain_uv_422_c
            as unsafe extern "C" fn(
                *mut [entry; 82],
                *const [entry; 82],
                *const Dav1dFilmGrainData,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*c).generate_grain_uv[(DAV1D_PIXEL_LAYOUT_I444 as libc::c_int - 1i32) as usize] = Some(
        generate_grain_uv_444_c
            as unsafe extern "C" fn(
                *mut [entry; 82],
                *const [entry; 82],
                *const Dav1dFilmGrainData,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*c).fgy_32x32xn = Some(
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
                libc::c_int,
            ) -> (),
    );
    (*c).fguv_32x32xn[(DAV1D_PIXEL_LAYOUT_I420 as libc::c_int - 1i32) as usize] = Some(
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
                libc::c_int,
            ) -> (),
    );
    (*c).fguv_32x32xn[(DAV1D_PIXEL_LAYOUT_I422 as libc::c_int - 1i32) as usize] = Some(
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
                libc::c_int,
            ) -> (),
    );
    (*c).fguv_32x32xn[(DAV1D_PIXEL_LAYOUT_I444 as libc::c_int - 1i32) as usize] = Some(
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
                libc::c_int,
            ) -> (),
    );
}
