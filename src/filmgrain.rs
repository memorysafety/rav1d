use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynEntry;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::intops::iclip;
use crate::include::dav1d::headers::Rav1dFilmGrainData;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I422;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I444;
use crate::src::tables::dav1d_gaussian_sequence;
use libc::intptr_t;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_ulong;

#[cfg(feature = "asm")]
use cfg_if::cfg_if;

#[cfg(feature = "asm")]
use crate::{include::common::bitdepth::bd_fn, src::cpu::rav1d_get_cpu_flags, src::cpu::CpuFlags};

pub const GRAIN_WIDTH: usize = 82;
pub const GRAIN_HEIGHT: usize = 73;

pub type generate_grain_y_fn = Option<
    unsafe extern "C" fn(*mut [DynEntry; GRAIN_WIDTH], *const Rav1dFilmGrainData, c_int) -> (),
>;

pub type generate_grain_uv_fn = Option<
    unsafe extern "C" fn(
        *mut [DynEntry; GRAIN_WIDTH],
        *const [DynEntry; GRAIN_WIDTH],
        *const Rav1dFilmGrainData,
        intptr_t,
        c_int,
    ) -> (),
>;

pub type fgy_32x32xn_fn = Option<
    unsafe extern "C" fn(
        *mut DynPixel,
        *const DynPixel,
        ptrdiff_t,
        *const Rav1dFilmGrainData,
        usize,
        *const u8,
        *const [DynEntry; GRAIN_WIDTH],
        c_int,
        c_int,
        c_int,
    ) -> (),
>;

pub type fguv_32x32xn_fn = Option<
    unsafe extern "C" fn(
        *mut DynPixel,
        *const DynPixel,
        ptrdiff_t,
        *const Rav1dFilmGrainData,
        usize,
        *const u8,
        *const [DynEntry; GRAIN_WIDTH],
        c_int,
        c_int,
        *const DynPixel,
        ptrdiff_t,
        c_int,
        c_int,
        c_int,
    ) -> (),
>;

#[repr(C)]
pub struct Rav1dFilmGrainDSPContext {
    pub generate_grain_y: generate_grain_y_fn,
    pub generate_grain_uv: [generate_grain_uv_fn; 3],
    pub fgy_32x32xn: fgy_32x32xn_fn,
    pub fguv_32x32xn: [fguv_32x32xn_fn; 3],
}

#[cfg(feature = "asm")]
macro_rules! decl_generate_grain_y_fn {
    (fn $name:ident) => {{
        extern "C" {
            fn $name(
                buf: *mut [DynEntry; GRAIN_WIDTH],
                data: *const Rav1dFilmGrainData,
                bitdepth_max: c_int,
            );
        }

        $name
    }};
}

#[cfg(feature = "asm")]
macro_rules! decl_generate_grain_uv_fn {
    (fn $name:ident) => {{
        extern "C" {
            fn $name(
                buf: *mut [DynEntry; GRAIN_WIDTH],
                buf_y: *const [DynEntry; GRAIN_WIDTH],
                data: *const Rav1dFilmGrainData,
                uv: intptr_t,
                bitdepth_max: c_int,
            );
        }

        $name
    }};
}

#[cfg(feature = "asm")]
macro_rules! decl_fgy_32x32xn_fn {
    (fn $name:ident) => {{
        extern "C" {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            fn $name(
                dst_row: *mut DynPixel,
                src_row: *const DynPixel,
                stride: ptrdiff_t,
                data: *const Rav1dFilmGrainData,
                pw: usize,
                scaling: *const u8,
                grain_lut: *const [DynEntry; GRAIN_WIDTH],
                bh: c_int,
                row_num: c_int,
                bitdepth_max: c_int,
            );

            #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
            fn $name(
                dst: *mut DynPixel,
                src: *const DynPixel,
                stride: ptrdiff_t,
                scaling: *const u8,
                scaling_shift: c_int,
                grain_lut: *const [DynEntry; GRAIN_WIDTH],
                offsets: *const [c_int; 2],
                h: c_int,
                clip: ptrdiff_t,
                type_0: ptrdiff_t,
                bitdepth_max: c_int,
            );
        }

        $name
    }};
}

#[cfg(feature = "asm")]
macro_rules! decl_fguv_32x32xn_fn {
    (fn $name:ident) => {{
        extern "C" {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            fn $name(
                dst_row: *mut DynPixel,
                src_row: *const DynPixel,
                stride: ptrdiff_t,
                data: *const Rav1dFilmGrainData,
                pw: usize,
                scaling: *const u8,
                grain_lut: *const [DynEntry; GRAIN_WIDTH],
                bh: c_int,
                row_num: c_int,
                luma_row: *const DynPixel,
                luma_stride: ptrdiff_t,
                uv_pl: c_int,
                is_id: c_int,
                bitdepth_max: c_int,
            );

            #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
            fn $name(
                dst: *mut DynPixel,
                src: *const DynPixel,
                stride: ptrdiff_t,
                scaling: *const u8,
                data: *const Rav1dFilmGrainData,
                grain_lut: *const [DynEntry; GRAIN_WIDTH],
                luma_row: *const DynPixel,
                luma_stride: ptrdiff_t,
                offsets: *const [c_int; 2],
                h: ptrdiff_t,
                uv: ptrdiff_t,
                is_id: ptrdiff_t,
                type_0: ptrdiff_t,
                bitdepth_max: c_int,
            );
        }

        $name
    }};
}

#[inline]
unsafe fn get_random_number(bits: c_int, state: *mut c_uint) -> c_int {
    let r = *state as c_int;
    let bit: c_uint = ((r >> 0 ^ r >> 1 ^ r >> 3 ^ r >> 12) & 1) as c_uint;
    *state = (r >> 1) as c_uint | bit << 15;
    return (*state >> 16 - bits & (((1 as c_int) << bits) - 1) as c_uint) as c_int;
}

#[inline]
unsafe fn round2(x: c_int, shift: u64) -> c_int {
    return x + ((1 as c_int) << shift >> 1) >> shift;
}

unsafe extern "C" fn generate_grain_y_c_erased<BD: BitDepth>(
    buf: *mut [DynEntry; GRAIN_WIDTH],
    data: *const Rav1dFilmGrainData,
    bitdepth_max: c_int,
) {
    generate_grain_y_rust(buf.cast(), data, BD::from_c(bitdepth_max))
}

unsafe fn generate_grain_y_rust<BD: BitDepth>(
    buf: *mut [BD::Entry; GRAIN_WIDTH],
    data: *const Rav1dFilmGrainData,
    bd: BD,
) {
    let bitdepth_min_8 = bd.bitdepth() as c_int - 8;
    let mut seed: c_uint = (*data).seed;
    let shift = 4 - bitdepth_min_8 + (*data).grain_scale_shift;
    let grain_ctr = (128 as c_int) << bitdepth_min_8;
    let grain_min = -grain_ctr;
    let grain_max = grain_ctr - 1;
    let mut y = 0;
    while y < 73 {
        let mut x = 0;
        while x < 82 {
            let value = get_random_number(11 as c_int, &mut seed);
            (*buf.offset(y as isize))[x as usize] = round2(
                dav1d_gaussian_sequence[value as usize] as c_int,
                shift as u64,
            )
            .as_::<BD::Entry>();
            x += 1;
        }
        y += 1;
    }
    let ar_pad = 3;
    let ar_lag = (*data).ar_coeff_lag;
    let mut y_0 = ar_pad;
    while y_0 < 73 {
        let mut x_0 = ar_pad;
        while x_0 < 82 - ar_pad {
            let mut coeff: *const i8 = ((*data).ar_coeffs_y).as_ptr();
            let mut sum = 0;
            let mut dy = -ar_lag;
            while dy <= 0 {
                let mut dx = -ar_lag;
                while dx <= ar_lag {
                    if dx == 0 && dy == 0 {
                        break;
                    }
                    let fresh0 = coeff;
                    coeff = coeff.offset(1);
                    sum += *fresh0 as c_int
                        * (*buf.offset((y_0 + dy) as isize))[(x_0 + dx) as usize].as_::<c_int>();
                    dx += 1;
                }
                dy += 1;
            }
            let grain = (*buf.offset(y_0 as isize))[x_0 as usize].as_::<c_int>()
                + round2(sum, (*data).ar_coeff_shift);
            (*buf.offset(y_0 as isize))[x_0 as usize] =
                iclip(grain, grain_min, grain_max).as_::<BD::Entry>();
            x_0 += 1;
        }
        y_0 += 1;
    }
}

#[inline(never)]
unsafe fn generate_grain_uv_c<BD: BitDepth>(
    buf: *mut [BD::Entry; GRAIN_WIDTH],
    buf_y: *const [BD::Entry; GRAIN_WIDTH],
    data: *const Rav1dFilmGrainData,
    uv: intptr_t,
    subx: c_int,
    suby: c_int,
    bd: BD,
) {
    let bitdepth_min_8 = bd.bitdepth() as c_int - 8;
    let mut seed: c_uint = (*data).seed
        ^ (if uv != 0 {
            0x49d8 as c_int
        } else {
            0xb524 as c_int
        }) as c_uint;
    let shift = 4 - bitdepth_min_8 + (*data).grain_scale_shift;
    let grain_ctr = (128 as c_int) << bitdepth_min_8;
    let grain_min = -grain_ctr;
    let grain_max = grain_ctr - 1;
    let chromaW = if subx != 0 { 44 as c_int } else { 82 as c_int };
    let chromaH = if suby != 0 { 38 as c_int } else { 73 as c_int };
    let mut y = 0;
    while y < chromaH {
        let mut x = 0;
        while x < chromaW {
            let value = get_random_number(11 as c_int, &mut seed);
            (*buf.offset(y as isize))[x as usize] = round2(
                dav1d_gaussian_sequence[value as usize] as c_int,
                shift as u64,
            )
            .as_::<BD::Entry>();
            x += 1;
        }
        y += 1;
    }
    let ar_pad = 3;
    let ar_lag = (*data).ar_coeff_lag;
    let mut y_0 = ar_pad;
    while y_0 < chromaH {
        let mut x_0 = ar_pad;
        while x_0 < chromaW - ar_pad {
            let mut coeff: *const i8 = ((*data).ar_coeffs_uv[uv as usize]).as_ptr();
            let mut sum = 0;
            let mut dy = -ar_lag;
            while dy <= 0 {
                let mut dx = -ar_lag;
                while dx <= ar_lag {
                    if dx == 0 && dy == 0 {
                        if (*data).num_y_points == 0 {
                            break;
                        }
                        let mut luma = 0;
                        let lumaX = (x_0 - ar_pad << subx) + ar_pad;
                        let lumaY = (y_0 - ar_pad << suby) + ar_pad;
                        let mut i = 0;
                        while i <= suby {
                            let mut j = 0;
                            while j <= subx {
                                luma += (*buf_y.offset((lumaY + i) as isize))[(lumaX + j) as usize]
                                    .as_::<c_int>();
                                j += 1;
                            }
                            i += 1;
                        }
                        luma = round2(luma, (subx + suby) as u64);
                        sum += luma * *coeff as c_int;
                        break;
                    } else {
                        let fresh1 = coeff;
                        coeff = coeff.offset(1);
                        sum += *fresh1 as c_int
                            * (*buf.offset((y_0 + dy) as isize))[(x_0 + dx) as usize]
                                .as_::<c_int>();
                        dx += 1;
                    }
                }
                dy += 1;
            }
            let grain = (*buf.offset(y_0 as isize))[x_0 as usize].as_::<c_int>()
                + round2(sum, (*data).ar_coeff_shift);
            (*buf.offset(y_0 as isize))[x_0 as usize] =
                iclip(grain, grain_min, grain_max).as_::<BD::Entry>();
            x_0 += 1;
        }
        y_0 += 1;
    }
}

unsafe extern "C" fn generate_grain_uv_420_c_erased<BD: BitDepth>(
    buf: *mut [DynEntry; GRAIN_WIDTH],
    buf_y: *const [DynEntry; GRAIN_WIDTH],
    data: *const Rav1dFilmGrainData,
    uv: intptr_t,
    bitdepth_max: c_int,
) {
    generate_grain_uv_c::<BD>(
        buf.cast(),
        buf_y.cast(),
        data,
        uv,
        1 as c_int,
        1 as c_int,
        BD::from_c(bitdepth_max),
    );
}

unsafe extern "C" fn generate_grain_uv_422_c_erased<BD: BitDepth>(
    buf: *mut [DynEntry; GRAIN_WIDTH],
    buf_y: *const [DynEntry; GRAIN_WIDTH],
    data: *const Rav1dFilmGrainData,
    uv: intptr_t,
    bitdepth_max: c_int,
) {
    generate_grain_uv_c::<BD>(
        buf.cast(),
        buf_y.cast(),
        data,
        uv,
        1 as c_int,
        0 as c_int,
        BD::from_c(bitdepth_max),
    );
}

unsafe extern "C" fn generate_grain_uv_444_c_erased<BD: BitDepth>(
    buf: *mut [DynEntry; GRAIN_WIDTH],
    buf_y: *const [DynEntry; GRAIN_WIDTH],
    data: *const Rav1dFilmGrainData,
    uv: intptr_t,
    bitdepth_max: c_int,
) {
    generate_grain_uv_c::<BD>(
        buf.cast(),
        buf_y.cast(),
        data,
        uv,
        0 as c_int,
        0 as c_int,
        BD::from_c(bitdepth_max),
    );
}

#[inline]
unsafe fn sample_lut<BD: BitDepth>(
    grain_lut: *const [BD::Entry; GRAIN_WIDTH],
    offsets: *const [c_int; 2],
    subx: c_int,
    suby: c_int,
    bx: c_int,
    by: c_int,
    x: c_int,
    y: c_int,
) -> BD::Entry {
    let randval = (*offsets.offset(bx as isize))[by as usize];
    let offx = 3 + (2 >> subx) * (3 + (randval >> 4));
    let offy = 3 + (2 >> suby) * (3 + (randval & 0xf as c_int));
    return (*grain_lut.offset((offy + y + (32 >> suby) * by) as isize))
        [(offx + x + (32 >> subx) * bx) as usize];
}

unsafe extern "C" fn fgy_32x32xn_c_erased<BD: BitDepth>(
    dst_row: *mut DynPixel,
    src_row: *const DynPixel,
    stride: ptrdiff_t,
    data: *const Rav1dFilmGrainData,
    pw: usize,
    scaling: *const u8,
    grain_lut: *const [DynEntry; GRAIN_WIDTH],
    bh: c_int,
    row_num: c_int,
    bitdepth_max: c_int,
) {
    fgy_32x32xn_rust(
        dst_row.cast(),
        src_row.cast(),
        stride,
        data,
        pw,
        scaling,
        grain_lut.cast(),
        bh,
        row_num,
        BD::from_c(bitdepth_max),
    );
}

unsafe fn fgy_32x32xn_rust<BD: BitDepth>(
    dst_row: *mut BD::Pixel,
    src_row: *const BD::Pixel,
    stride: ptrdiff_t,
    data: *const Rav1dFilmGrainData,
    pw: usize,
    scaling: *const u8,
    grain_lut: *const [BD::Entry; GRAIN_WIDTH],
    bh: c_int,
    row_num: c_int,
    bd: BD,
) {
    let rows = 1 + ((*data).overlap_flag && row_num > 0) as c_int;
    let bitdepth_min_8 = bd.bitdepth() as c_int - 8;
    let grain_ctr = (128 as c_int) << bitdepth_min_8;
    let grain_min = -grain_ctr;
    let grain_max = grain_ctr - 1;
    let min_value;
    let max_value;
    if (*data).clip_to_restricted_range {
        min_value = (16 as c_int) << bitdepth_min_8;
        max_value = (235 as c_int) << bitdepth_min_8;
    } else {
        min_value = 0 as c_int;
        max_value = bd.bitdepth_max().as_::<c_int>();
    }
    let mut seed: [c_uint; 2] = [0; 2];
    let mut i = 0;
    while i < rows {
        seed[i as usize] = (*data).seed;
        seed[i as usize] ^= (((row_num - i) * 37 + 178 & 0xff as c_int) << 8) as c_uint;
        seed[i as usize] ^= ((row_num - i) * 173 + 105 & 0xff as c_int) as c_uint;
        i += 1;
    }
    if !((stride as c_ulong).wrapping_rem(
        (32 as c_int as c_ulong).wrapping_mul(::core::mem::size_of::<BD::Pixel>() as c_ulong),
    ) == 0 as c_ulong)
    {
        unreachable!();
    }
    let mut offsets: [[c_int; 2]; 2] = [[0; 2]; 2];
    let mut bx: c_uint = 0 as c_int as c_uint;
    while (bx as usize) < pw {
        let bw = cmp::min(
            32 as c_int,
            (pw as c_int as c_uint).wrapping_sub(bx) as c_int,
        );
        if (*data).overlap_flag && bx != 0 {
            let mut i_0 = 0;
            while i_0 < rows {
                offsets[1][i_0 as usize] = offsets[0][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1 = 0;
        while i_1 < rows {
            offsets[0][i_1 as usize] =
                get_random_number(8 as c_int, &mut *seed.as_mut_ptr().offset(i_1 as isize));
            i_1 += 1;
        }
        let ystart = if (*data).overlap_flag && row_num != 0 {
            cmp::min(2 as c_int, bh)
        } else {
            0 as c_int
        };
        let xstart = if (*data).overlap_flag && bx != 0 {
            cmp::min(2 as c_int, bw)
        } else {
            0 as c_int
        };
        static w: [[c_int; 2]; 2] = [[27, 17], [17, 27]];
        let mut y = ystart;
        while y < bh {
            let mut x = xstart;
            while x < bw {
                let grain = sample_lut::<BD>(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [c_int; 2],
                    0 as c_int,
                    0 as c_int,
                    0 as c_int,
                    0 as c_int,
                    x,
                    y,
                )
                .as_::<c_int>();
                let src: *const BD::Pixel = src_row
                    .offset((y as isize * BD::pxstride(stride as usize) as isize) as isize)
                    .offset(x as isize)
                    .offset(bx as isize);
                let dst: *mut BD::Pixel = dst_row
                    .offset((y as isize * BD::pxstride(stride as usize) as isize) as isize)
                    .offset(x as isize)
                    .offset(bx as isize);
                let noise = round2(
                    *scaling.offset((*src).as_::<isize>()) as c_int * grain,
                    (*data).scaling_shift as u64,
                );
                *dst =
                    iclip((*src).as_::<c_int>() + noise, min_value, max_value).as_::<BD::Pixel>();
                x += 1;
            }
            let mut x_0 = 0;
            while x_0 < xstart {
                let mut grain_0 = sample_lut::<BD>(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [c_int; 2],
                    0 as c_int,
                    0 as c_int,
                    0 as c_int,
                    0 as c_int,
                    x_0,
                    y,
                )
                .as_::<c_int>();
                let old = sample_lut::<BD>(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [c_int; 2],
                    0 as c_int,
                    0 as c_int,
                    1 as c_int,
                    0 as c_int,
                    x_0,
                    y,
                )
                .as_::<c_int>();
                grain_0 = round2(
                    old * w[x_0 as usize][0] + grain_0 * w[x_0 as usize][1],
                    5 as c_int as u64,
                );
                grain_0 = iclip(grain_0, grain_min, grain_max);
                let src_0: *const BD::Pixel = src_row
                    .offset((y as isize * BD::pxstride(stride as usize) as isize) as isize)
                    .offset(x_0 as isize)
                    .offset(bx as isize);
                let dst_0: *mut BD::Pixel = dst_row
                    .offset((y as isize * BD::pxstride(stride as usize) as isize) as isize)
                    .offset(x_0 as isize)
                    .offset(bx as isize);
                let noise_0 = round2(
                    *scaling.offset((*src_0).as_::<isize>()) as c_int * grain_0,
                    (*data).scaling_shift as u64,
                );
                *dst_0 = iclip((*src_0).as_::<c_int>() + noise_0, min_value, max_value)
                    .as_::<BD::Pixel>();
                x_0 += 1;
            }
            y += 1;
        }
        let mut y_0 = 0;
        while y_0 < ystart {
            let mut x_1 = xstart;
            while x_1 < bw {
                let mut grain_1 = sample_lut::<BD>(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [c_int; 2],
                    0 as c_int,
                    0 as c_int,
                    0 as c_int,
                    0 as c_int,
                    x_1,
                    y_0,
                )
                .as_::<c_int>();
                let old_0 = sample_lut::<BD>(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [c_int; 2],
                    0 as c_int,
                    0 as c_int,
                    0 as c_int,
                    1 as c_int,
                    x_1,
                    y_0,
                )
                .as_::<c_int>();
                grain_1 = round2(
                    old_0 * w[y_0 as usize][0] + grain_1 * w[y_0 as usize][1],
                    5 as c_int as u64,
                );
                grain_1 = iclip(grain_1, grain_min, grain_max);
                let src_1: *const BD::Pixel = src_row
                    .offset((y_0 as isize * BD::pxstride(stride as usize) as isize) as isize)
                    .offset(x_1 as isize)
                    .offset(bx as isize);
                let dst_1: *mut BD::Pixel = dst_row
                    .offset((y_0 as isize * BD::pxstride(stride as usize) as isize) as isize)
                    .offset(x_1 as isize)
                    .offset(bx as isize);
                let noise_1 = round2(
                    *scaling.offset((*src_1).as_::<isize>()) as c_int * grain_1,
                    (*data).scaling_shift as u64,
                );
                *dst_1 = iclip((*src_1).as_::<c_int>() + noise_1, min_value, max_value)
                    .as_::<BD::Pixel>();
                x_1 += 1;
            }
            let mut x_2 = 0;
            while x_2 < xstart {
                let mut top = sample_lut::<BD>(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [c_int; 2],
                    0 as c_int,
                    0 as c_int,
                    0 as c_int,
                    1 as c_int,
                    x_2,
                    y_0,
                )
                .as_::<c_int>();
                let mut old_1 = sample_lut::<BD>(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [c_int; 2],
                    0 as c_int,
                    0 as c_int,
                    1 as c_int,
                    1 as c_int,
                    x_2,
                    y_0,
                )
                .as_::<c_int>();
                top = round2(
                    old_1 * w[x_2 as usize][0] + top * w[x_2 as usize][1],
                    5 as c_int as u64,
                );
                top = iclip(top, grain_min, grain_max);
                let mut grain_2 = sample_lut::<BD>(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [c_int; 2],
                    0 as c_int,
                    0 as c_int,
                    0 as c_int,
                    0 as c_int,
                    x_2,
                    y_0,
                )
                .as_::<c_int>();
                old_1 = sample_lut::<BD>(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [c_int; 2],
                    0 as c_int,
                    0 as c_int,
                    1 as c_int,
                    0 as c_int,
                    x_2,
                    y_0,
                )
                .as_::<c_int>();
                grain_2 = round2(
                    old_1 * w[x_2 as usize][0] + grain_2 * w[x_2 as usize][1],
                    5 as c_int as u64,
                );
                grain_2 = iclip(grain_2, grain_min, grain_max);
                grain_2 = round2(
                    top * w[y_0 as usize][0] + grain_2 * w[y_0 as usize][1],
                    5 as c_int as u64,
                );
                grain_2 = iclip(grain_2, grain_min, grain_max);
                let src_2: *const BD::Pixel = src_row
                    .offset((y_0 as isize * BD::pxstride(stride as usize) as isize) as isize)
                    .offset(x_2 as isize)
                    .offset(bx as isize);
                let dst_2: *mut BD::Pixel = dst_row
                    .offset((y_0 as isize * BD::pxstride(stride as usize) as isize) as isize)
                    .offset(x_2 as isize)
                    .offset(bx as isize);
                let noise_2 = round2(
                    *scaling.offset((*src_2).as_::<isize>()) as c_int * grain_2,
                    (*data).scaling_shift as u64,
                );
                *dst_2 = iclip((*src_2).as_::<c_int>() + noise_2, min_value, max_value)
                    .as_::<BD::Pixel>();
                x_2 += 1;
            }
            y_0 += 1;
        }
        bx = bx.wrapping_add(32 as c_int as c_uint);
    }
}

#[inline(never)]
unsafe fn fguv_32x32xn_c<BD: BitDepth>(
    dst_row: *mut BD::Pixel,
    src_row: *const BD::Pixel,
    stride: ptrdiff_t,
    data: *const Rav1dFilmGrainData,
    pw: usize,
    scaling: *const u8,
    grain_lut: *const [BD::Entry; GRAIN_WIDTH],
    bh: c_int,
    row_num: c_int,
    luma_row: *const BD::Pixel,
    luma_stride: ptrdiff_t,
    uv: c_int,
    is_id: c_int,
    sx: c_int,
    sy: c_int,
    bd: BD,
) {
    let rows = 1 + ((*data).overlap_flag && row_num > 0) as c_int;
    let bitdepth_min_8 = bd.bitdepth() - 8;
    let grain_ctr = (128 as c_int) << bitdepth_min_8;
    let grain_min = -grain_ctr;
    let grain_max = grain_ctr - 1;
    let min_value;
    let max_value;
    if (*data).clip_to_restricted_range {
        min_value = (16 as c_int) << bitdepth_min_8;
        max_value = (if is_id != 0 {
            235 as c_int
        } else {
            240 as c_int
        }) << bitdepth_min_8;
    } else {
        min_value = 0 as c_int;
        max_value = bd.bitdepth_max().as_::<c_int>();
    }
    let mut seed: [c_uint; 2] = [0; 2];
    let mut i = 0;
    while i < rows {
        seed[i as usize] = (*data).seed;
        seed[i as usize] ^= (((row_num - i) * 37 + 178 & 0xff as c_int) << 8) as c_uint;
        seed[i as usize] ^= ((row_num - i) * 173 + 105 & 0xff as c_int) as c_uint;
        i += 1;
    }
    if !((stride as c_ulong).wrapping_rem(
        (32 as c_int as c_ulong).wrapping_mul(::core::mem::size_of::<BD::Pixel>() as c_ulong),
    ) == 0 as c_ulong)
    {
        unreachable!();
    }
    let mut offsets: [[c_int; 2]; 2] = [[0; 2]; 2];
    let mut bx: c_uint = 0 as c_int as c_uint;
    while (bx as usize) < pw {
        let bw = cmp::min(32 >> sx, pw.wrapping_sub(bx as usize) as c_int);
        if (*data).overlap_flag && bx != 0 {
            let mut i_0 = 0;
            while i_0 < rows {
                offsets[1][i_0 as usize] = offsets[0][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1 = 0;
        while i_1 < rows {
            offsets[0][i_1 as usize] =
                get_random_number(8 as c_int, &mut *seed.as_mut_ptr().offset(i_1 as isize));
            i_1 += 1;
        }
        let ystart = if (*data).overlap_flag && row_num != 0 {
            cmp::min(2 >> sy, bh)
        } else {
            0 as c_int
        };
        let xstart = if (*data).overlap_flag && bx != 0 {
            cmp::min(2 >> sx, bw)
        } else {
            0 as c_int
        };
        static w: [[[c_int; 2]; 2]; 2] = [[[27, 17], [17, 27]], [[23, 22], [0; 2]]];
        let mut y = ystart;
        while y < bh {
            let mut x = xstart;
            while x < bw {
                let grain = sample_lut::<BD>(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [c_int; 2],
                    sx,
                    sy,
                    0 as c_int,
                    0 as c_int,
                    x,
                    y,
                )
                .as_::<c_int>();
                let lx = (bx.wrapping_add(x as c_uint) << sx) as c_int;
                let ly = y << sy;
                let luma: *const BD::Pixel = luma_row
                    .offset((ly as isize * BD::pxstride(luma_stride as usize) as isize) as isize)
                    .offset(lx as isize);
                let mut avg: BD::Pixel = *luma.offset(0);
                if sx != 0 {
                    avg = (avg.as_::<c_int>() + (*luma.offset(1)).as_::<c_int>() + 1 >> 1)
                        .as_::<BD::Pixel>();
                }
                let src: *const BD::Pixel = src_row
                    .offset((y as isize * BD::pxstride(stride as usize) as isize) as isize)
                    .offset(bx.wrapping_add(x as c_uint) as isize);
                let dst: *mut BD::Pixel = dst_row
                    .offset((y as isize * BD::pxstride(stride as usize) as isize) as isize)
                    .offset(bx.wrapping_add(x as c_uint) as isize);
                let mut val = avg.as_::<c_int>();
                if !(*data).chroma_scaling_from_luma {
                    let combined = avg.as_::<c_int>() * (*data).uv_luma_mult[uv as usize]
                        + (*src).as_::<c_int>() * (*data).uv_mult[uv as usize];
                    val = iclip(
                        (combined >> 6)
                            + (*data).uv_offset[uv as usize] * ((1 as c_int) << bitdepth_min_8),
                        0 as c_int,
                        bd.bitdepth_max().as_::<c_int>(),
                    );
                }
                let noise = round2(
                    *scaling.offset(val as isize) as c_int * grain,
                    (*data).scaling_shift as u64,
                );
                *dst =
                    iclip((*src).as_::<c_int>() + noise, min_value, max_value).as_::<BD::Pixel>();
                x += 1;
            }
            let mut x_0 = 0;
            while x_0 < xstart {
                let mut grain_0 = sample_lut::<BD>(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [c_int; 2],
                    sx,
                    sy,
                    0 as c_int,
                    0 as c_int,
                    x_0,
                    y,
                )
                .as_::<c_int>();
                let old = sample_lut::<BD>(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [c_int; 2],
                    sx,
                    sy,
                    1 as c_int,
                    0 as c_int,
                    x_0,
                    y,
                )
                .as_::<c_int>();
                grain_0 = round2(
                    old * w[sx as usize][x_0 as usize][0]
                        + grain_0 * w[sx as usize][x_0 as usize][1],
                    5 as c_int as u64,
                );
                grain_0 = iclip(grain_0, grain_min, grain_max);
                let lx_0 = (bx.wrapping_add(x_0 as c_uint) << sx) as c_int;
                let ly_0 = y << sy;
                let luma_0: *const BD::Pixel = luma_row
                    .offset((ly_0 as isize * BD::pxstride(luma_stride as usize) as isize) as isize)
                    .offset(lx_0 as isize);
                let mut avg_0: BD::Pixel = *luma_0.offset(0);
                if sx != 0 {
                    avg_0 = (avg_0.as_::<c_int>() + (*luma_0.offset(1)).as_::<c_int>() + 1 >> 1)
                        .as_::<BD::Pixel>();
                }
                let src_0: *const BD::Pixel = src_row
                    .offset((y as isize * BD::pxstride(stride as usize) as isize) as isize)
                    .offset(bx.wrapping_add(x_0 as c_uint) as isize);
                let dst_0: *mut BD::Pixel = dst_row
                    .offset((y as isize * BD::pxstride(stride as usize) as isize) as isize)
                    .offset(bx.wrapping_add(x_0 as c_uint) as isize);
                let mut val_0 = avg_0.as_::<c_int>();
                if !(*data).chroma_scaling_from_luma {
                    let combined_0 = avg_0.as_::<c_int>() * (*data).uv_luma_mult[uv as usize]
                        + (*src_0).as_::<c_int>() * (*data).uv_mult[uv as usize];
                    val_0 = iclip(
                        (combined_0 >> 6)
                            + (*data).uv_offset[uv as usize] * ((1 as c_int) << bitdepth_min_8),
                        0 as c_int,
                        bd.bitdepth_max().as_::<c_int>(),
                    );
                }
                let noise_0 = round2(
                    *scaling.offset(val_0 as isize) as c_int * grain_0,
                    (*data).scaling_shift as u64,
                );
                *dst_0 = iclip((*src_0).as_::<c_int>() + noise_0, min_value, max_value)
                    .as_::<BD::Pixel>();
                x_0 += 1;
            }
            y += 1;
        }
        let mut y_0 = 0;
        while y_0 < ystart {
            let mut x_1 = xstart;
            while x_1 < bw {
                let mut grain_1 = sample_lut::<BD>(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [c_int; 2],
                    sx,
                    sy,
                    0 as c_int,
                    0 as c_int,
                    x_1,
                    y_0,
                )
                .as_::<c_int>();
                let old_0 = sample_lut::<BD>(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [c_int; 2],
                    sx,
                    sy,
                    0 as c_int,
                    1 as c_int,
                    x_1,
                    y_0,
                )
                .as_::<c_int>();
                grain_1 = round2(
                    old_0 * w[sy as usize][y_0 as usize][0]
                        + grain_1 * w[sy as usize][y_0 as usize][1],
                    5 as c_int as u64,
                );
                grain_1 = iclip(grain_1, grain_min, grain_max);
                let lx_1 = (bx.wrapping_add(x_1 as c_uint) << sx) as c_int;
                let ly_1 = y_0 << sy;
                let luma_1: *const BD::Pixel = luma_row
                    .offset((ly_1 as isize * BD::pxstride(luma_stride as usize) as isize) as isize)
                    .offset(lx_1 as isize);
                let mut avg_1: BD::Pixel = *luma_1.offset(0);
                if sx != 0 {
                    avg_1 = (avg_1.as_::<c_int>() + (*luma_1.offset(1)).as_::<c_int>() + 1 >> 1)
                        .as_::<BD::Pixel>();
                }
                let src_1: *const BD::Pixel = src_row
                    .offset((y_0 as isize * BD::pxstride(stride as usize) as isize) as isize)
                    .offset(bx.wrapping_add(x_1 as c_uint) as isize);
                let dst_1: *mut BD::Pixel = dst_row
                    .offset((y_0 as isize * BD::pxstride(stride as usize) as isize) as isize)
                    .offset(bx.wrapping_add(x_1 as c_uint) as isize);
                let mut val_1 = avg_1.as_::<c_int>();
                if !(*data).chroma_scaling_from_luma {
                    let combined_1 = avg_1.as_::<c_int>() * (*data).uv_luma_mult[uv as usize]
                        + (*src_1).as_::<c_int>() * (*data).uv_mult[uv as usize];
                    val_1 = iclip(
                        (combined_1 >> 6)
                            + (*data).uv_offset[uv as usize] * ((1 as c_int) << bitdepth_min_8),
                        0 as c_int,
                        bd.bitdepth_max().as_::<c_int>(),
                    );
                }
                let noise_1 = round2(
                    *scaling.offset(val_1 as isize) as c_int * grain_1,
                    (*data).scaling_shift as u64,
                );
                *dst_1 = iclip((*src_1).as_::<c_int>() + noise_1, min_value, max_value)
                    .as_::<BD::Pixel>();
                x_1 += 1;
            }
            let mut x_2 = 0;
            while x_2 < xstart {
                let mut top = sample_lut::<BD>(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [c_int; 2],
                    sx,
                    sy,
                    0 as c_int,
                    1 as c_int,
                    x_2,
                    y_0,
                )
                .as_::<c_int>();
                let mut old_1 = sample_lut::<BD>(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [c_int; 2],
                    sx,
                    sy,
                    1 as c_int,
                    1 as c_int,
                    x_2,
                    y_0,
                )
                .as_::<c_int>();
                top = round2(
                    old_1 * w[sx as usize][x_2 as usize][0] + top * w[sx as usize][x_2 as usize][1],
                    5 as c_int as u64,
                );
                top = iclip(top, grain_min, grain_max);
                let mut grain_2 = sample_lut::<BD>(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [c_int; 2],
                    sx,
                    sy,
                    0 as c_int,
                    0 as c_int,
                    x_2,
                    y_0,
                )
                .as_::<c_int>();
                old_1 = sample_lut::<BD>(
                    grain_lut,
                    offsets.as_mut_ptr() as *const [c_int; 2],
                    sx,
                    sy,
                    1 as c_int,
                    0 as c_int,
                    x_2,
                    y_0,
                )
                .as_::<c_int>();
                grain_2 = round2(
                    old_1 * w[sx as usize][x_2 as usize][0]
                        + grain_2 * w[sx as usize][x_2 as usize][1],
                    5 as c_int as u64,
                );
                grain_2 = iclip(grain_2, grain_min, grain_max);
                grain_2 = round2(
                    top * w[sy as usize][y_0 as usize][0]
                        + grain_2 * w[sy as usize][y_0 as usize][1],
                    5 as c_int as u64,
                );
                grain_2 = iclip(grain_2, grain_min, grain_max);
                let lx_2 = (bx.wrapping_add(x_2 as c_uint) << sx) as c_int;
                let ly_2 = y_0 << sy;
                let luma_2: *const BD::Pixel = luma_row
                    .offset((ly_2 as isize * BD::pxstride(luma_stride as usize) as isize) as isize)
                    .offset(lx_2 as isize);
                let mut avg_2: BD::Pixel = *luma_2.offset(0);
                if sx != 0 {
                    avg_2 = (avg_2.as_::<c_int>() + (*luma_2.offset(1)).as_::<c_int>() + 1 >> 1)
                        .as_::<BD::Pixel>();
                }
                let src_2: *const BD::Pixel = src_row
                    .offset((y_0 as isize * BD::pxstride(stride as usize) as isize) as isize)
                    .offset(bx.wrapping_add(x_2 as c_uint) as isize);
                let dst_2: *mut BD::Pixel = dst_row
                    .offset((y_0 as isize * BD::pxstride(stride as usize) as isize) as isize)
                    .offset(bx.wrapping_add(x_2 as c_uint) as isize);
                let mut val_2 = avg_2.as_::<c_int>();
                if !(*data).chroma_scaling_from_luma {
                    let combined_2 = avg_2.as_::<c_int>() * (*data).uv_luma_mult[uv as usize]
                        + (*src_2).as_::<c_int>() * (*data).uv_mult[uv as usize];
                    val_2 = iclip(
                        (combined_2 >> 6)
                            + (*data).uv_offset[uv as usize] * ((1 as c_int) << bitdepth_min_8),
                        0 as c_int,
                        bd.bitdepth_max().as_::<c_int>(),
                    );
                }
                let noise_2 = round2(
                    *scaling.offset(val_2 as isize) as c_int * grain_2,
                    (*data).scaling_shift as u64,
                );
                *dst_2 = iclip((*src_2).as_::<c_int>() + noise_2, min_value, max_value)
                    .as_::<BD::Pixel>();
                x_2 += 1;
            }
            y_0 += 1;
        }
        bx = bx.wrapping_add((32 >> sx) as c_uint);
    }
}

unsafe extern "C" fn fguv_32x32xn_420_c_erased<BD: BitDepth>(
    dst_row: *mut DynPixel,
    src_row: *const DynPixel,
    stride: ptrdiff_t,
    data: *const Rav1dFilmGrainData,
    pw: usize,
    scaling: *const u8,
    grain_lut: *const [DynEntry; GRAIN_WIDTH],
    bh: c_int,
    row_num: c_int,
    luma_row: *const DynPixel,
    luma_stride: ptrdiff_t,
    uv_pl: c_int,
    is_id: c_int,
    bitdepth_max: c_int,
) {
    fguv_32x32xn_c::<BD>(
        dst_row.cast(),
        src_row.cast(),
        stride,
        data,
        pw,
        scaling,
        grain_lut.cast(),
        bh,
        row_num,
        luma_row.cast(),
        luma_stride,
        uv_pl,
        is_id,
        1 as c_int,
        1 as c_int,
        BD::from_c(bitdepth_max),
    );
}

unsafe extern "C" fn fguv_32x32xn_422_c_erased<BD: BitDepth>(
    dst_row: *mut DynPixel,
    src_row: *const DynPixel,
    stride: ptrdiff_t,
    data: *const Rav1dFilmGrainData,
    pw: usize,
    scaling: *const u8,
    grain_lut: *const [DynEntry; GRAIN_WIDTH],
    bh: c_int,
    row_num: c_int,
    luma_row: *const DynPixel,
    luma_stride: ptrdiff_t,
    uv_pl: c_int,
    is_id: c_int,
    bitdepth_max: c_int,
) {
    fguv_32x32xn_c::<BD>(
        dst_row.cast(),
        src_row.cast(),
        stride,
        data,
        pw,
        scaling,
        grain_lut.cast(),
        bh,
        row_num,
        luma_row.cast(),
        luma_stride,
        uv_pl,
        is_id,
        1 as c_int,
        0 as c_int,
        BD::from_c(bitdepth_max),
    );
}

unsafe extern "C" fn fguv_32x32xn_444_c_erased<BD: BitDepth>(
    dst_row: *mut DynPixel,
    src_row: *const DynPixel,
    stride: ptrdiff_t,
    data: *const Rav1dFilmGrainData,
    pw: usize,
    scaling: *const u8,
    grain_lut: *const [DynEntry; GRAIN_WIDTH],
    bh: c_int,
    row_num: c_int,
    luma_row: *const DynPixel,
    luma_stride: ptrdiff_t,
    uv_pl: c_int,
    is_id: c_int,
    bitdepth_max: c_int,
) {
    fguv_32x32xn_c::<BD>(
        dst_row.cast(),
        src_row.cast(),
        stride,
        data,
        pw,
        scaling,
        grain_lut.cast(),
        bh,
        row_num,
        luma_row.cast(),
        luma_stride,
        uv_pl,
        is_id,
        0 as c_int,
        0 as c_int,
        BD::from_c(bitdepth_max),
    );
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
#[inline(always)]
unsafe fn film_grain_dsp_init_x86<BD: BitDepth>(c: *mut Rav1dFilmGrainDSPContext) {
    let flags = rav1d_get_cpu_flags();

    if !flags.contains(CpuFlags::SSSE3) {
        return;
    }

    (*c).generate_grain_y = Some(bd_fn!(
        decl_generate_grain_y_fn,
        BD,
        generate_grain_y,
        ssse3
    ));
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] = Some(bd_fn!(
        decl_generate_grain_uv_fn,
        BD,
        generate_grain_uv_420,
        ssse3
    ));
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] = Some(bd_fn!(
        decl_generate_grain_uv_fn,
        BD,
        generate_grain_uv_422,
        ssse3
    ));
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] = Some(bd_fn!(
        decl_generate_grain_uv_fn,
        BD,
        generate_grain_uv_444,
        ssse3
    ));

    (*c).fgy_32x32xn = Some(bd_fn!(decl_fgy_32x32xn_fn, BD, fgy_32x32xn, ssse3));
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
        Some(bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32xn_i420, ssse3));
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
        Some(bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32xn_i422, ssse3));
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
        Some(bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32xn_i444, ssse3));

    #[cfg(target_arch = "x86_64")]
    {
        if !flags.contains(CpuFlags::AVX2) {
            return;
        }

        (*c).generate_grain_y = Some(bd_fn!(decl_generate_grain_y_fn, BD, generate_grain_y, avx2));
        (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] = Some(bd_fn!(
            decl_generate_grain_uv_fn,
            BD,
            generate_grain_uv_420,
            avx2
        ));
        (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] = Some(bd_fn!(
            decl_generate_grain_uv_fn,
            BD,
            generate_grain_uv_422,
            avx2
        ));
        (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] = Some(bd_fn!(
            decl_generate_grain_uv_fn,
            BD,
            generate_grain_uv_444,
            avx2
        ));

        if !flags.contains(CpuFlags::SLOW_GATHER) {
            (*c).fgy_32x32xn = Some(bd_fn!(decl_fgy_32x32xn_fn, BD, fgy_32x32xn, avx2));
            (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
                Some(bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32xn_i420, avx2));
            (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
                Some(bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32xn_i422, avx2));
            (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
                Some(bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32xn_i444, avx2));
        }

        if !flags.contains(CpuFlags::AVX512ICL) {
            return;
        }

        (*c).fgy_32x32xn = Some(bd_fn!(decl_fgy_32x32xn_fn, BD, fgy_32x32xn, avx512icl));
        (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] = Some(bd_fn!(
            decl_fguv_32x32xn_fn,
            BD,
            fguv_32x32xn_i420,
            avx512icl
        ));
        (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] = Some(bd_fn!(
            decl_fguv_32x32xn_fn,
            BD,
            fguv_32x32xn_i422,
            avx512icl
        ));
        (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] = Some(bd_fn!(
            decl_fguv_32x32xn_fn,
            BD,
            fguv_32x32xn_i444,
            avx512icl
        ));
    }
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn fgy_32x32xn_neon_erased<BD: BitDepth>(
    dst_row: *mut DynPixel,
    src_row: *const DynPixel,
    stride: ptrdiff_t,
    data: *const Rav1dFilmGrainData,
    pw: usize,
    scaling: *const u8,
    grain_lut: *const [DynEntry; GRAIN_WIDTH],
    bh: c_int,
    row_num: c_int,
    bitdepth_max: c_int,
) {
    fgy_32x32xn_neon(
        dst_row.cast(),
        src_row.cast(),
        stride,
        data,
        pw,
        scaling,
        grain_lut.cast(),
        bh,
        row_num,
        BD::from_c(bitdepth_max),
    );
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe fn fgy_32x32xn_neon<BD: BitDepth>(
    dst_row: *mut BD::Pixel,
    src_row: *const BD::Pixel,
    stride: ptrdiff_t,
    data: *const Rav1dFilmGrainData,
    pw: usize,
    scaling: *const u8,
    grain_lut: *const [BD::Entry; GRAIN_WIDTH],
    bh: c_int,
    row_num: c_int,
    bd: BD,
) {
    let rows = 1 + ((*data).overlap_flag && row_num > 0) as c_int;
    let mut seed: [c_uint; 2] = [0; 2];
    let mut i = 0;
    while i < rows {
        seed[i as usize] = (*data).seed;
        seed[i as usize] ^= (((row_num - i) * 37 + 178 & 0xff as c_int) << 8) as c_uint;
        seed[i as usize] ^= ((row_num - i) * 173 + 105 & 0xff as c_int) as c_uint;
        i += 1;
    }
    let mut offsets: [[c_int; 2]; 2] = [[0; 2]; 2];
    let mut bx: c_uint = 0 as c_int as c_uint;
    while (bx as usize) < pw {
        if (*data).overlap_flag && bx != 0 {
            let mut i_0 = 0;
            while i_0 < rows {
                offsets[1][i_0 as usize] = offsets[0][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1 = 0;
        while i_1 < rows {
            offsets[0][i_1 as usize] =
                get_random_number(8 as c_int, &mut *seed.as_mut_ptr().offset(i_1 as isize));
            i_1 += 1;
        }
        let mut type_0 = 0;
        if (*data).overlap_flag && row_num != 0 {
            type_0 |= 1 as c_int;
        }
        if (*data).overlap_flag && bx != 0 {
            type_0 |= 2 as c_int;
        }
        bd_fn!(decl_fgy_32x32xn_fn, BD, fgy_32x32, neon)(
            dst_row.offset(bx as isize).cast(),
            src_row.offset(bx as isize).cast(),
            stride,
            scaling,
            (*data).scaling_shift,
            grain_lut.cast(),
            offsets.as_mut_ptr() as *const [c_int; 2],
            bh,
            (*data).clip_to_restricted_range as ptrdiff_t,
            type_0 as ptrdiff_t,
            bd.into_c(),
        );
        bx = bx.wrapping_add(32 as c_int as c_uint);
    }
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn fguv_32x32xn_420_neon_erased<BD: BitDepth>(
    dst_row: *mut DynPixel,
    src_row: *const DynPixel,
    stride: ptrdiff_t,
    data: *const Rav1dFilmGrainData,
    pw: usize,
    scaling: *const u8,
    grain_lut: *const [DynEntry; GRAIN_WIDTH],
    bh: c_int,
    row_num: c_int,
    luma_row: *const DynPixel,
    luma_stride: ptrdiff_t,
    uv: c_int,
    is_id: c_int,
    bitdepth_max: c_int,
) {
    fguv_32x32xn_420_neon::<BD>(
        dst_row.cast(),
        src_row.cast(),
        stride,
        data,
        pw,
        scaling,
        grain_lut.cast(),
        bh,
        row_num,
        luma_row.cast(),
        luma_stride,
        uv,
        is_id,
        BD::from_c(bitdepth_max),
    );
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe fn fguv_32x32xn_420_neon<BD: BitDepth>(
    dst_row: *mut BD::Pixel,
    src_row: *const BD::Pixel,
    stride: ptrdiff_t,
    data: *const Rav1dFilmGrainData,
    pw: usize,
    scaling: *const u8,
    grain_lut: *const [BD::Entry; GRAIN_WIDTH],
    bh: c_int,
    row_num: c_int,
    luma_row: *const BD::Pixel,
    luma_stride: ptrdiff_t,
    uv: c_int,
    is_id: c_int,
    bd: BD,
) {
    let rows = 1 + ((*data).overlap_flag && row_num > 0) as c_int;
    let mut seed: [c_uint; 2] = [0; 2];
    let mut i = 0;
    while i < rows {
        seed[i as usize] = (*data).seed;
        seed[i as usize] ^= (((row_num - i) * 37 + 178 & 0xff as c_int) << 8) as c_uint;
        seed[i as usize] ^= ((row_num - i) * 173 + 105 & 0xff as c_int) as c_uint;
        i += 1;
    }
    let mut offsets: [[c_int; 2]; 2] = [[0; 2]; 2];
    let mut bx: c_uint = 0 as c_int as c_uint;
    while (bx as usize) < pw {
        if (*data).overlap_flag && bx != 0 {
            let mut i_0 = 0;
            while i_0 < rows {
                offsets[1][i_0 as usize] = offsets[0][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1 = 0;
        while i_1 < rows {
            offsets[0][i_1 as usize] =
                get_random_number(8 as c_int, &mut *seed.as_mut_ptr().offset(i_1 as isize));
            i_1 += 1;
        }
        let mut type_0 = 0;
        if (*data).overlap_flag && row_num != 0 {
            type_0 |= 1 as c_int;
        }
        if (*data).overlap_flag && bx != 0 {
            type_0 |= 2 as c_int;
        }
        if (*data).chroma_scaling_from_luma {
            type_0 |= 4 as c_int;
        }
        bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32_420, neon)(
            dst_row.offset(bx as isize).cast(),
            src_row.offset(bx as isize).cast(),
            stride,
            scaling,
            data,
            grain_lut.cast(),
            luma_row.offset((bx << 1) as isize).cast(),
            luma_stride,
            offsets.as_mut_ptr() as *const [c_int; 2],
            bh as ptrdiff_t,
            uv as ptrdiff_t,
            is_id as ptrdiff_t,
            type_0 as ptrdiff_t,
            bd.into_c(),
        );
        bx = bx.wrapping_add((32 >> 1) as c_uint);
    }
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn fguv_32x32xn_422_neon_erased<BD: BitDepth>(
    dst_row: *mut DynPixel,
    src_row: *const DynPixel,
    stride: ptrdiff_t,
    data: *const Rav1dFilmGrainData,
    pw: usize,
    scaling: *const u8,
    grain_lut: *const [DynEntry; GRAIN_WIDTH],
    bh: c_int,
    row_num: c_int,
    luma_row: *const DynPixel,
    luma_stride: ptrdiff_t,
    uv: c_int,
    is_id: c_int,
    bitdepth_max: c_int,
) {
    fguv_32x32xn_422_neon::<BD>(
        dst_row.cast(),
        src_row.cast(),
        stride,
        data,
        pw,
        scaling,
        grain_lut.cast(),
        bh,
        row_num,
        luma_row.cast(),
        luma_stride,
        uv,
        is_id,
        BD::from_c(bitdepth_max),
    );
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe fn fguv_32x32xn_422_neon<BD: BitDepth>(
    dst_row: *mut BD::Pixel,
    src_row: *const BD::Pixel,
    stride: ptrdiff_t,
    data: *const Rav1dFilmGrainData,
    pw: usize,
    scaling: *const u8,
    grain_lut: *const [BD::Entry; GRAIN_WIDTH],
    bh: c_int,
    row_num: c_int,
    luma_row: *const BD::Pixel,
    luma_stride: ptrdiff_t,
    uv: c_int,
    is_id: c_int,
    bd: BD,
) {
    let rows = 1 + ((*data).overlap_flag && row_num > 0) as c_int;
    let mut seed: [c_uint; 2] = [0; 2];
    let mut i = 0;
    while i < rows {
        seed[i as usize] = (*data).seed;
        seed[i as usize] ^= (((row_num - i) * 37 + 178 & 0xff as c_int) << 8) as c_uint;
        seed[i as usize] ^= ((row_num - i) * 173 + 105 & 0xff as c_int) as c_uint;
        i += 1;
    }
    let mut offsets: [[c_int; 2]; 2] = [[0; 2]; 2];
    let mut bx: c_uint = 0 as c_int as c_uint;
    while (bx as usize) < pw {
        if (*data).overlap_flag && bx != 0 {
            let mut i_0 = 0;
            while i_0 < rows {
                offsets[1][i_0 as usize] = offsets[0][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1 = 0;
        while i_1 < rows {
            offsets[0][i_1 as usize] =
                get_random_number(8 as c_int, &mut *seed.as_mut_ptr().offset(i_1 as isize));
            i_1 += 1;
        }
        let mut type_0 = 0;
        if (*data).overlap_flag && row_num != 0 {
            type_0 |= 1 as c_int;
        }
        if (*data).overlap_flag && bx != 0 {
            type_0 |= 2 as c_int;
        }
        if (*data).chroma_scaling_from_luma {
            type_0 |= 4 as c_int;
        }
        bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32_422, neon)(
            dst_row.offset(bx as isize).cast(),
            src_row.offset(bx as isize).cast(),
            stride,
            scaling,
            data,
            grain_lut.cast(),
            luma_row.offset((bx << 1) as isize).cast(),
            luma_stride,
            offsets.as_mut_ptr() as *const [c_int; 2],
            bh as ptrdiff_t,
            uv as ptrdiff_t,
            is_id as ptrdiff_t,
            type_0 as ptrdiff_t,
            bd.into_c(),
        );
        bx = bx.wrapping_add((32 >> 1) as c_uint);
    }
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn fguv_32x32xn_444_neon_erased<BD: BitDepth>(
    dst_row: *mut DynPixel,
    src_row: *const DynPixel,
    stride: ptrdiff_t,
    data: *const Rav1dFilmGrainData,
    pw: usize,
    scaling: *const u8,
    grain_lut: *const [DynEntry; GRAIN_WIDTH],
    bh: c_int,
    row_num: c_int,
    luma_row: *const DynPixel,
    luma_stride: ptrdiff_t,
    uv: c_int,
    is_id: c_int,
    bitdepth_max: c_int,
) {
    fguv_32x32xn_444_neon::<BD>(
        dst_row.cast(),
        src_row.cast(),
        stride,
        data,
        pw,
        scaling,
        grain_lut.cast(),
        bh,
        row_num,
        luma_row.cast(),
        luma_stride,
        uv,
        is_id,
        BD::from_c(bitdepth_max),
    );
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe fn fguv_32x32xn_444_neon<BD: BitDepth>(
    dst_row: *mut BD::Pixel,
    src_row: *const BD::Pixel,
    stride: ptrdiff_t,
    data: *const Rav1dFilmGrainData,
    pw: usize,
    scaling: *const u8,
    grain_lut: *const [BD::Entry; GRAIN_WIDTH],
    bh: c_int,
    row_num: c_int,
    luma_row: *const BD::Pixel,
    luma_stride: ptrdiff_t,
    uv: c_int,
    is_id: c_int,
    bd: BD,
) {
    let rows = 1 + ((*data).overlap_flag && row_num > 0) as c_int;
    let mut seed: [c_uint; 2] = [0; 2];
    let mut i = 0;
    while i < rows {
        seed[i as usize] = (*data).seed;
        seed[i as usize] ^= (((row_num - i) * 37 + 178 & 0xff as c_int) << 8) as c_uint;
        seed[i as usize] ^= ((row_num - i) * 173 + 105 & 0xff as c_int) as c_uint;
        i += 1;
    }
    let mut offsets: [[c_int; 2]; 2] = [[0; 2]; 2];
    let mut bx: c_uint = 0 as c_int as c_uint;
    while (bx as usize) < pw {
        if (*data).overlap_flag && bx != 0 {
            let mut i_0 = 0;
            while i_0 < rows {
                offsets[1][i_0 as usize] = offsets[0][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1 = 0;
        while i_1 < rows {
            offsets[0][i_1 as usize] =
                get_random_number(8 as c_int, &mut *seed.as_mut_ptr().offset(i_1 as isize));
            i_1 += 1;
        }
        let mut type_0 = 0;
        if (*data).overlap_flag && row_num != 0 {
            type_0 |= 1 as c_int;
        }
        if (*data).overlap_flag && bx != 0 {
            type_0 |= 2 as c_int;
        }
        if (*data).chroma_scaling_from_luma {
            type_0 |= 4 as c_int;
        }
        bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32_444, neon)(
            dst_row.offset(bx as isize).cast(),
            src_row.offset(bx as isize).cast(),
            stride,
            scaling,
            data,
            grain_lut.cast(),
            luma_row.offset((bx << 0) as isize).cast(),
            luma_stride,
            offsets.as_mut_ptr() as *const [c_int; 2],
            bh as ptrdiff_t,
            uv as ptrdiff_t,
            is_id as ptrdiff_t,
            type_0 as ptrdiff_t,
            bd.into_c(),
        );
        bx = bx.wrapping_add((32 >> 0) as c_uint);
    }
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
#[inline(always)]
unsafe fn film_grain_dsp_init_arm<BD: BitDepth>(c: *mut Rav1dFilmGrainDSPContext) {
    let flags = rav1d_get_cpu_flags();

    if !flags.contains(CpuFlags::NEON) {
        return;
    }

    (*c).generate_grain_y = Some(bd_fn!(decl_generate_grain_y_fn, BD, generate_grain_y, neon));
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] = Some(bd_fn!(
        decl_generate_grain_uv_fn,
        BD,
        generate_grain_uv_420,
        neon
    ));
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] = Some(bd_fn!(
        decl_generate_grain_uv_fn,
        BD,
        generate_grain_uv_422,
        neon
    ));
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] = Some(bd_fn!(
        decl_generate_grain_uv_fn,
        BD,
        generate_grain_uv_444,
        neon
    ));

    (*c).fgy_32x32xn = Some(fgy_32x32xn_neon_erased::<BD>);
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
        Some(fguv_32x32xn_420_neon_erased::<BD>);
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
        Some(fguv_32x32xn_422_neon_erased::<BD>);
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
        Some(fguv_32x32xn_444_neon_erased::<BD>);
}

#[cold]
pub unsafe fn rav1d_film_grain_dsp_init<BD: BitDepth>(c: *mut Rav1dFilmGrainDSPContext) {
    (*c).generate_grain_y = Some(generate_grain_y_c_erased::<BD>);
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
        Some(generate_grain_uv_420_c_erased::<BD>);
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
        Some(generate_grain_uv_422_c_erased::<BD>);
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
        Some(generate_grain_uv_444_c_erased::<BD>);

    (*c).fgy_32x32xn = Some(fgy_32x32xn_c_erased::<BD>);
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
        Some(fguv_32x32xn_420_c_erased::<BD>);
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
        Some(fguv_32x32xn_422_c_erased::<BD>);
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
        Some(fguv_32x32xn_444_c_erased::<BD>);

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            film_grain_dsp_init_x86::<BD>(c);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
            film_grain_dsp_init_arm::<BD>(c);
        }
    }
}
