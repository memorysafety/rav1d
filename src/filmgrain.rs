use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynEntry;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::bitdepth::DynScaling;
use crate::include::common::intops::iclip;
use crate::include::dav1d::headers::Rav1dFilmGrainData;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I422;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I444;
use crate::src::internal::GrainLut;
use crate::src::tables::dav1d_gaussian_sequence;
use libc::intptr_t;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ops::Add;
use std::ops::Shl;
use std::ops::Shr;
use to_method::To;

#[cfg(feature = "asm")]
use cfg_if::cfg_if;

#[cfg(feature = "asm")]
use crate::{include::common::bitdepth::bd_fn, src::cpu::rav1d_get_cpu_flags, src::cpu::CpuFlags};

pub const GRAIN_WIDTH: usize = 82;
pub const GRAIN_HEIGHT: usize = 73;

const BLOCK_SIZE: usize = 32;

const SUB_GRAIN_WIDTH: usize = 44;
const SUB_GRAIN_HEIGHT: usize = 38;

pub type generate_grain_y_fn = unsafe extern "C" fn(
    buf: *mut GrainLut<DynEntry>,
    data: &Rav1dFilmGrainData,
    bitdepth_max: c_int,
) -> ();

pub type generate_grain_uv_fn = unsafe extern "C" fn(
    buf: *mut GrainLut<DynEntry>,
    buf_y: *const GrainLut<DynEntry>,
    data: &Rav1dFilmGrainData,
    uv: intptr_t,
    bitdepth_max: c_int,
) -> ();

pub type fgy_32x32xn_fn = unsafe extern "C" fn(
    dst_row: *mut DynPixel,
    src_row: *const DynPixel,
    stride: ptrdiff_t,
    data: &Rav1dFilmGrainData,
    pw: usize,
    scaling: *const DynScaling,
    grain_lut: *const GrainLut<DynEntry>,
    bh: c_int,
    row_num: c_int,
    bitdepth_max: c_int,
) -> ();

pub type fguv_32x32xn_fn = unsafe extern "C" fn(
    dst_row: *mut DynPixel,
    src_row: *const DynPixel,
    stride: ptrdiff_t,
    data: &Rav1dFilmGrainData,
    pw: usize,
    scaling: *const DynScaling,
    grain_lut: *const GrainLut<DynEntry>,
    bh: c_int,
    row_num: c_int,
    luma_row: *const DynPixel,
    luma_stride: ptrdiff_t,
    uv_pl: c_int,
    is_id: c_int,
    bitdepth_max: c_int,
) -> ();

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
            fn $name(buf: *mut GrainLut<DynEntry>, data: &Rav1dFilmGrainData, bitdepth_max: c_int);
        }

        $name
    }};
}

#[cfg(feature = "asm")]
macro_rules! decl_generate_grain_uv_fn {
    (fn $name:ident) => {{
        extern "C" {
            fn $name(
                buf: *mut GrainLut<DynEntry>,
                buf_y: *const GrainLut<DynEntry>,
                data: &Rav1dFilmGrainData,
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
                data: &Rav1dFilmGrainData,
                pw: usize,
                scaling: *const DynScaling,
                grain_lut: *const GrainLut<DynEntry>,
                bh: c_int,
                row_num: c_int,
                bitdepth_max: c_int,
            );

            // Use [`ptrdiff_t`] instead of [`c_int`] for the last few parameters,
            // to get the same layout of parameters on the stack across platforms.
            #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
            fn $name(
                dst: *mut DynPixel,
                src: *const DynPixel,
                stride: ptrdiff_t,
                scaling: *const DynScaling,
                scaling_shift: c_int,
                grain_lut: *const GrainLut<DynEntry>,
                offsets: *const [[c_int; 2]; 2],
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
                data: &Rav1dFilmGrainData,
                pw: usize,
                scaling: *const DynScaling,
                grain_lut: *const GrainLut<DynEntry>,
                bh: c_int,
                row_num: c_int,
                luma_row: *const DynPixel,
                luma_stride: ptrdiff_t,
                uv_pl: c_int,
                is_id: c_int,
                bitdepth_max: c_int,
            );

            // Use [`ptrdiff_t`] instead of [`c_int`] for the last few parameters,
            // to get the parameters on the stack with the same layout across platforms.
            #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
            fn $name(
                dst: *mut DynPixel,
                src: *const DynPixel,
                stride: ptrdiff_t,
                scaling: *const DynScaling,
                data: &Rav1dFilmGrainData,
                grain_lut: *const GrainLut<DynEntry>,
                luma_row: *const DynPixel,
                luma_stride: ptrdiff_t,
                offsets: *const [[c_int; 2]; 2],
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
fn get_random_number(bits: u8, state: &mut c_uint) -> c_int {
    let r = *state;
    let bit = ((r >> 0) ^ (r >> 1) ^ (r >> 3) ^ (r >> 12)) & 1;
    *state = (r >> 1) | bit << 15;

    (*state >> 16 - bits & (1 << bits) - 1) as c_int
}

#[inline]
fn round2<T, B>(x: T, shift: B) -> T
where
    T: Add<Output = T> + From<u8> + Shl<B, Output = T> + Shr<B, Output = T> + Shr<u8, Output = T>,
    B: Copy,
{
    (x + (T::from(1) << shift >> 1)) >> shift
}

unsafe extern "C" fn generate_grain_y_c_erased<BD: BitDepth>(
    buf: *mut GrainLut<DynEntry>,
    data: &Rav1dFilmGrainData,
    bitdepth_max: c_int,
) {
    // Safety: Casting back to the original type from the `fn` ptr call.
    let buf = unsafe { &mut *buf.cast() };
    let bd = BD::from_c(bitdepth_max);
    generate_grain_y_rust(buf, data, bd)
}

unsafe fn generate_grain_y_rust<BD: BitDepth>(
    buf: &mut GrainLut<BD::Entry>,
    data: &Rav1dFilmGrainData,
    bd: BD,
) {
    let bitdepth_min_8 = bd.bitdepth() as c_int - 8;
    let mut seed: c_uint = data.seed;
    let shift = 4 - bitdepth_min_8 + data.grain_scale_shift;
    let grain_ctr = (128 as c_int) << bitdepth_min_8;
    let grain_min = -grain_ctr;
    let grain_max = grain_ctr - 1;

    for y in 0..GRAIN_HEIGHT {
        for x in 0..GRAIN_WIDTH {
            let value = get_random_number(11, &mut seed);
            buf[y as usize][x as usize] =
                round2(dav1d_gaussian_sequence[value as usize], shift).as_::<BD::Entry>();
        }
    }

    let ar_pad = 3;
    let ar_lag = data.ar_coeff_lag;

    for y in ar_pad..GRAIN_HEIGHT as c_int {
        for x in ar_pad..GRAIN_WIDTH as c_int - ar_pad {
            let mut coeff: *const i8 = (data.ar_coeffs_y).as_ptr();
            let mut sum = 0;
            for dy in -ar_lag..=0 {
                for dx in -ar_lag..=ar_lag {
                    if dx == 0 && dy == 0 {
                        break;
                    }
                    let fresh0 = coeff;
                    coeff = coeff.offset(1);
                    sum +=
                        *fresh0 as c_int * buf[(y + dy) as usize][(x + dx) as usize].as_::<c_int>();
                }
            }

            let grain =
                buf[y as usize][x as usize].as_::<c_int>() + round2(sum, data.ar_coeff_shift);
            buf[y as usize][x as usize] = iclip(grain, grain_min, grain_max).as_::<BD::Entry>();
        }
    }
}

#[inline(never)]
unsafe fn generate_grain_uv_rust<BD: BitDepth>(
    buf: &mut GrainLut<BD::Entry>,
    buf_y: &GrainLut<BD::Entry>,
    data: &Rav1dFilmGrainData,
    uv: intptr_t,
    is_subx: bool,
    is_suby: bool,
    bd: BD,
) {
    let [subx, suby] = [is_subx, is_suby].map(|it| it as c_int);

    let bitdepth_min_8 = bd.bitdepth() as c_int - 8;
    let mut seed: c_uint = data.seed
        ^ (if uv != 0 {
            0x49d8 as c_int
        } else {
            0xb524 as c_int
        }) as c_uint;
    let shift = 4 - bitdepth_min_8 + data.grain_scale_shift;
    let grain_ctr = (128 as c_int) << bitdepth_min_8;
    let grain_min = -grain_ctr;
    let grain_max = grain_ctr - 1;

    let chromaW = if is_subx {
        SUB_GRAIN_WIDTH as c_int
    } else {
        GRAIN_WIDTH as c_int
    };
    let chromaH = if is_suby {
        SUB_GRAIN_HEIGHT as c_int
    } else {
        GRAIN_HEIGHT as c_int
    };

    for y in 0..chromaH {
        for x in 0..chromaW {
            let value = get_random_number(11, &mut seed);
            buf[y as usize][x as usize] =
                round2(dav1d_gaussian_sequence[value as usize], shift).as_::<BD::Entry>();
        }
    }

    let ar_pad = 3;
    let ar_lag = data.ar_coeff_lag;

    for y in ar_pad..chromaH {
        for x in ar_pad..chromaW - ar_pad {
            let mut coeff: *const i8 = (data.ar_coeffs_uv[uv as usize]).as_ptr();
            let mut sum = 0;
            for dy in -ar_lag..=0 {
                for dx in -ar_lag..=ar_lag {
                    // For the final (current) pixel, we need to add in the
                    // contribution from the luma grain texture.
                    if dx == 0 && dy == 0 {
                        if data.num_y_points == 0 {
                            break;
                        }
                        let mut luma = 0;
                        let lumaX = (x - ar_pad << subx) + ar_pad;
                        let lumaY = (y - ar_pad << suby) + ar_pad;
                        for i in 0..=suby {
                            for j in 0..=subx {
                                luma += buf_y[(lumaY + i) as usize][(lumaX + j) as usize]
                                    .as_::<c_int>();
                            }
                        }
                        luma = round2(luma, subx + suby);

                        sum += luma * *coeff as c_int;
                        break;
                    } else {
                        let fresh1 = coeff;
                        coeff = coeff.offset(1);

                        sum += *fresh1 as c_int
                            * buf[(y + dy) as usize][(x + dx) as usize].as_::<c_int>();
                    }
                }
            }

            let grain =
                buf[y as usize][x as usize].as_::<c_int>() + round2(sum, data.ar_coeff_shift);
            buf[y as usize][x as usize] = iclip(grain, grain_min, grain_max).as_::<BD::Entry>();
        }
    }
}

unsafe extern "C" fn generate_grain_uv_c_erased<
    BD: BitDepth,
    const NM: usize,
    const IS_SUBX: bool,
    const IS_SUBY: bool,
>(
    buf: *mut GrainLut<DynEntry>,
    buf_y: *const GrainLut<DynEntry>,
    data: &Rav1dFilmGrainData,
    uv: intptr_t,
    bitdepth_max: c_int,
) {
    // Safety: Casting back to the original type from the `fn` ptr call.
    let buf = unsafe { &mut *buf.cast() };
    // Safety: Casting back to the original type from the `fn` ptr call.
    let buf_y = unsafe { &*buf_y.cast() };
    let bd = BD::from_c(bitdepth_max);
    generate_grain_uv_rust(buf, buf_y, data, uv, IS_SUBX, IS_SUBY, bd)
}

/// Sample from the correct block of a grain LUT,
/// while taking into account the offsets
/// provided by the offsets cache.
#[inline]
unsafe fn sample_lut<BD: BitDepth>(
    grain_lut: &GrainLut<BD::Entry>,
    offsets: &[[c_int; 2]; 2],
    is_subx: bool,
    is_suby: bool,
    is_bx: bool,
    is_by: bool,
    x: c_int,
    y: c_int,
) -> BD::Entry {
    let [subx, suby, bx, by] = [is_subx, is_suby, is_bx, is_by].map(|it| it as c_int);

    let randval = offsets[bx as usize][by as usize];
    let offx = 3 + (2 >> subx) * (3 + (randval >> 4));
    let offy = 3 + (2 >> suby) * (3 + (randval & 0xf as c_int));
    grain_lut[(offy + y + (BLOCK_SIZE as c_int >> suby) * by) as usize]
        [(offx + x + (BLOCK_SIZE as c_int >> subx) * bx) as usize]
}

unsafe extern "C" fn fgy_32x32xn_c_erased<BD: BitDepth>(
    dst_row: *mut DynPixel,
    src_row: *const DynPixel,
    stride: ptrdiff_t,
    data: &Rav1dFilmGrainData,
    pw: usize,
    scaling: *const DynScaling,
    grain_lut: *const GrainLut<DynEntry>,
    bh: c_int,
    row_num: c_int,
    bitdepth_max: c_int,
) {
    let dst_row = dst_row.cast();
    let src_row = src_row.cast();
    let scaling = scaling.cast();
    // Safety: Casting back to the original type from the `fn` ptr call.
    let grain_lut = unsafe { &*grain_lut.cast() };
    let bd = BD::from_c(bitdepth_max);
    fgy_32x32xn_rust(
        dst_row, src_row, stride, data, pw, scaling, grain_lut, bh, row_num, bd,
    )
}

unsafe fn fgy_32x32xn_rust<BD: BitDepth>(
    dst_row: *mut BD::Pixel,
    src_row: *const BD::Pixel,
    stride: ptrdiff_t,
    data: &Rav1dFilmGrainData,
    pw: usize,
    scaling: *const BD::Scaling,
    grain_lut: &GrainLut<BD::Entry>,
    bh: c_int,
    row_num: c_int,
    bd: BD,
) {
    let rows = 1 + (data.overlap_flag && row_num > 0) as c_int;
    let bitdepth_min_8 = bd.bitdepth() as c_int - 8;
    let grain_ctr = (128 as c_int) << bitdepth_min_8;
    let grain_min = -grain_ctr;
    let grain_max = grain_ctr - 1;

    let min_value;
    let max_value;
    if data.clip_to_restricted_range {
        min_value = (16 as c_int) << bitdepth_min_8;
        max_value = (235 as c_int) << bitdepth_min_8;
    } else {
        min_value = 0 as c_int;
        max_value = bd.bitdepth_max().as_::<c_int>();
    }

    // seed[0] contains the current row, seed[1] contains the previous
    let mut seed: [c_uint; 2] = [0; 2];
    for i in 0..rows {
        seed[i as usize] = data.seed;
        seed[i as usize] ^= (((row_num - i) * 37 + 178 & 0xff as c_int) << 8) as c_uint;
        seed[i as usize] ^= ((row_num - i) * 173 + 105 & 0xff as c_int) as c_uint;
    }

    assert!((stride as usize % (BLOCK_SIZE * ::core::mem::size_of::<BD::Pixel>())) == 0);

    let mut offsets: [[c_int; 2]; 2] = [[0; 2 /* row offset */]; 2 /* col offset */];

    // process this row in BLOCK_SIZE^2 blocks
    for bx in (0..pw).step_by(BLOCK_SIZE) {
        let bw = cmp::min(BLOCK_SIZE, pw - bx);

        if data.overlap_flag && bx != 0 {
            // shift previous offsets left
            for i in 0..rows {
                offsets[1][i as usize] = offsets[0][i as usize];
            }
        }

        // update current offsets
        for i in 0..rows {
            offsets[0][i as usize] =
                get_random_number(8, &mut *seed.as_mut_ptr().offset(i as isize));
        }

        // x/y block offsets to compensate for overlapped regions
        let ystart = if data.overlap_flag && row_num != 0 {
            cmp::min(2 as c_int, bh)
        } else {
            0 as c_int
        };
        let xstart = if data.overlap_flag && bx != 0 {
            cmp::min(2, bw)
        } else {
            0
        };

        static w: [[c_int; 2]; 2] = [[27, 17], [17, 27]];

        let add_noise_y = |x, y, grain| {
            let src: *const BD::Pixel = src_row
                .offset((y as isize * BD::pxstride(stride as usize) as isize) as isize)
                .offset(x as isize)
                .offset(bx as isize);
            let dst: *mut BD::Pixel = dst_row
                .offset((y as isize * BD::pxstride(stride as usize) as isize) as isize)
                .offset(x as isize)
                .offset(bx as isize);
            let noise = round2(
                (*scaling).as_ref()[(*src).to::<usize>()] as c_int * grain,
                data.scaling_shift,
            );
            *dst = iclip((*src).as_::<c_int>() + noise, min_value, max_value).as_::<BD::Pixel>();
        };

        for y in ystart..bh {
            // Non-overlapped image region (straightforward)
            for x in xstart..bw {
                let grain = sample_lut::<BD>(
                    grain_lut, &offsets, false, false, false, false, x as c_int, y,
                )
                .as_::<c_int>();
                add_noise_y(x, y, grain);
            }

            // Special case for overlapped column
            for x in 0..xstart {
                let mut grain = sample_lut::<BD>(
                    grain_lut, &offsets, false, false, false, false, x as c_int, y,
                )
                .as_::<c_int>();
                let old = sample_lut::<BD>(
                    grain_lut, &offsets, false, false, true, false, x as c_int, y,
                )
                .as_::<c_int>();
                grain = round2(old * w[x as usize][0] + grain * w[x as usize][1], 5);
                grain = iclip(grain, grain_min, grain_max);
                add_noise_y(x, y, grain);
            }
        }
        for y in 0..ystart {
            // Special case for overlapped row (sans corner)
            for x in xstart..bw {
                let mut grain = sample_lut::<BD>(
                    grain_lut, &offsets, false, false, false, false, x as c_int, y,
                )
                .as_::<c_int>();
                let old = sample_lut::<BD>(
                    grain_lut, &offsets, false, false, false, true, x as c_int, y,
                )
                .as_::<c_int>();
                grain = round2(old * w[y as usize][0] + grain * w[y as usize][1], 5);
                grain = iclip(grain, grain_min, grain_max);
                add_noise_y(x, y, grain);
            }

            // Special case for doubly-overlapped corner
            for x in 0..xstart {
                // Blend the top pixel with the top left block
                let mut top = sample_lut::<BD>(
                    grain_lut, &offsets, false, false, false, true, x as c_int, y,
                )
                .as_::<c_int>();
                let mut old =
                    sample_lut::<BD>(grain_lut, &offsets, false, false, true, true, x as c_int, y)
                        .as_::<c_int>();
                top = round2(old * w[x as usize][0] + top * w[x as usize][1], 5);
                top = iclip(top, grain_min, grain_max);

                // Blend the current pixel with the left block
                let mut grain = sample_lut::<BD>(
                    grain_lut, &offsets, false, false, false, false, x as c_int, y,
                )
                .as_::<c_int>();
                old = sample_lut::<BD>(
                    grain_lut, &offsets, false, false, true, false, x as c_int, y,
                )
                .as_::<c_int>();

                // Mix the row rows together and apply grain
                grain = round2(old * w[x as usize][0] + grain * w[x as usize][1], 5);
                grain = iclip(grain, grain_min, grain_max);
                grain = round2(top * w[y as usize][0] + grain * w[y as usize][1], 5);
                grain = iclip(grain, grain_min, grain_max);
                add_noise_y(x, y, grain);
            }
        }
    }
}

#[inline(never)]
unsafe fn fguv_32x32xn_rust<BD: BitDepth>(
    dst_row: *mut BD::Pixel,
    src_row: *const BD::Pixel,
    stride: ptrdiff_t,
    data: &Rav1dFilmGrainData,
    pw: usize,
    scaling: *const BD::Scaling,
    grain_lut: &GrainLut<BD::Entry>,
    bh: c_int,
    row_num: c_int,
    luma_row: *const BD::Pixel,
    luma_stride: ptrdiff_t,
    uv: c_int,
    is_id: c_int,
    is_sx: bool,
    is_sy: bool,
    bd: BD,
) {
    let [sx, sy] = [is_sx, is_sy].map(|it| it as c_int);

    let rows = 1 + (data.overlap_flag && row_num > 0) as c_int;
    let bitdepth_min_8 = bd.bitdepth() - 8;
    let grain_ctr = (128 as c_int) << bitdepth_min_8;
    let grain_min = -grain_ctr;
    let grain_max = grain_ctr - 1;

    let min_value;
    let max_value;
    if data.clip_to_restricted_range {
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

    // seed[0] contains the current row, seed[1] contains the previous
    let mut seed: [c_uint; 2] = [0; 2];
    for i in 0..rows {
        seed[i as usize] = data.seed;
        seed[i as usize] ^= (((row_num - i) * 37 + 178 & 0xff as c_int) << 8) as c_uint;
        seed[i as usize] ^= ((row_num - i) * 173 + 105 & 0xff as c_int) as c_uint;
    }

    assert!((stride as usize % (BLOCK_SIZE * ::core::mem::size_of::<BD::Pixel>())) == 0);

    let mut offsets: [[c_int; 2]; 2] = [[0; 2 /* row offset */]; 2 /* col offset */];

    // process this row in BLOCK_SIZE^2 blocks (subsampled)
    for bx in (0..pw).step_by(BLOCK_SIZE >> sx) {
        let bw = cmp::min(
            BLOCK_SIZE as c_int >> sx,
            pw.wrapping_sub(bx as usize) as c_int,
        );
        if data.overlap_flag && bx != 0 {
            // shift previous offsets left
            for i in 0..rows {
                offsets[1][i as usize] = offsets[0][i as usize];
            }
        }

        // update current offsets
        for i in 0..rows {
            offsets[0][i as usize] =
                get_random_number(8, &mut *seed.as_mut_ptr().offset(i as isize));
        }

        // x/y block offsets to compensate for overlapped regions
        let ystart = if data.overlap_flag && row_num != 0 {
            cmp::min(2 >> sy, bh)
        } else {
            0 as c_int
        };
        let xstart = if data.overlap_flag && bx != 0 {
            cmp::min(2 >> sx, bw)
        } else {
            0 as c_int
        };

        static w: [[[c_int; 2]; 2 /* off */]; 2 /* sub */] = [[[27, 17], [17, 27]], [[23, 22], [0; 2]]];

        let add_noise_uv = |x, y, grain| {
            let lx = (bx.wrapping_add(x as usize) << sx) as c_int;
            let ly = y << sy;
            let luma: *const BD::Pixel = luma_row
                .offset((ly as isize * BD::pxstride(luma_stride as usize) as isize) as isize)
                .offset(lx as isize);
            let mut avg: BD::Pixel = *luma.offset(0);
            if is_sx {
                avg = (avg.as_::<c_int>() + (*luma.offset(1)).as_::<c_int>() + 1 >> 1)
                    .as_::<BD::Pixel>();
            }
            let src: *const BD::Pixel = src_row
                .offset((y as isize * BD::pxstride(stride as usize) as isize) as isize)
                .offset(bx.wrapping_add(x as usize) as isize);
            let dst: *mut BD::Pixel = dst_row
                .offset((y as isize * BD::pxstride(stride as usize) as isize) as isize)
                .offset(bx.wrapping_add(x as usize) as isize);
            let mut val = avg.as_::<c_int>();
            if !data.chroma_scaling_from_luma {
                let combined = avg.as_::<c_int>() * data.uv_luma_mult[uv as usize]
                    + (*src).as_::<c_int>() * data.uv_mult[uv as usize];
                val = iclip(
                    (combined >> 6)
                        + data.uv_offset[uv as usize] * ((1 as c_int) << bitdepth_min_8),
                    0,
                    bd.bitdepth_max().as_::<c_int>(),
                );
            }
            let noise = round2(
                (*scaling).as_ref()[val as usize] as c_int * grain,
                data.scaling_shift,
            );
            *dst = iclip((*src).as_::<c_int>() + noise, min_value, max_value).as_::<BD::Pixel>();
        };

        for y in ystart..bh {
            // Non-overlapped image region (straightforward)
            for x in xstart..bw {
                let grain = sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, false, false, x, y)
                    .as_::<c_int>();
                add_noise_uv(x, y, grain);
            }

            // Special case for overlapped column
            for x in 0..xstart {
                let mut grain =
                    sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, false, false, x, y)
                        .as_::<c_int>();
                let old = sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, true, false, x, y)
                    .as_::<c_int>();
                grain = round2(
                    old * w[sx as usize][x as usize][0] + grain * w[sx as usize][x as usize][1],
                    5,
                );
                grain = iclip(grain, grain_min, grain_max);
                add_noise_uv(x, y, grain);
            }
        }
        for y in 0..ystart {
            // Special case for overlapped row (sans corner)
            for x in xstart..bw {
                let mut grain =
                    sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, false, false, x, y)
                        .as_::<c_int>();
                let old = sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, false, true, x, y)
                    .as_::<c_int>();
                grain = round2(
                    old * w[sy as usize][y as usize][0] + grain * w[sy as usize][y as usize][1],
                    5,
                );
                grain = iclip(grain, grain_min, grain_max);
                add_noise_uv(x, y, grain);
            }

            // Special case for doubly-overlapped corner
            for x in 0..xstart {
                // Blend the top pixel with the top left block
                let mut top =
                    sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, false, true, x, y)
                        .as_::<c_int>();
                let mut old = sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, true, true, x, y)
                    .as_::<c_int>();
                top = round2(
                    old * w[sx as usize][x as usize][0] + top * w[sx as usize][x as usize][1],
                    5,
                );
                top = iclip(top, grain_min, grain_max);

                // Blend the current pixel with the left block
                let mut grain =
                    sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, false, false, x, y)
                        .as_::<c_int>();
                old = sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, true, false, x, y)
                    .as_::<c_int>();

                // Mix the row rows together and apply to image
                grain = round2(
                    old * w[sx as usize][x as usize][0] + grain * w[sx as usize][x as usize][1],
                    5,
                );
                grain = iclip(grain, grain_min, grain_max);
                grain = round2(
                    top * w[sy as usize][y as usize][0] + grain * w[sy as usize][y as usize][1],
                    5,
                );
                grain = iclip(grain, grain_min, grain_max);
                add_noise_uv(x, y, grain);
            }
        }
    }
}

unsafe extern "C" fn fguv_32x32xn_c_erased<
    BD: BitDepth,
    const NM: usize,
    const IS_SX: bool,
    const IS_SY: bool,
>(
    dst_row: *mut DynPixel,
    src_row: *const DynPixel,
    stride: ptrdiff_t,
    data: &Rav1dFilmGrainData,
    pw: usize,
    scaling: *const DynScaling,
    grain_lut: *const GrainLut<DynEntry>,
    bh: c_int,
    row_num: c_int,
    luma_row: *const DynPixel,
    luma_stride: ptrdiff_t,
    uv_pl: c_int,
    is_id: c_int,
    bitdepth_max: c_int,
) {
    let dst_row = dst_row.cast();
    let src_row = src_row.cast();
    let scaling = scaling.cast();
    // Safety: Casting back to the original type from the `fn` ptr call.
    let grain_lut = unsafe { &*grain_lut.cast() };
    let luma_row = luma_row.cast();
    let bd = BD::from_c(bitdepth_max);
    fguv_32x32xn_rust(
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
        IS_SX,
        IS_SY,
        bd,
    )
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
#[inline(always)]
unsafe fn film_grain_dsp_init_x86<BD: BitDepth>(c: *mut Rav1dFilmGrainDSPContext) {
    let flags = rav1d_get_cpu_flags();

    if !flags.contains(CpuFlags::SSSE3) {
        return;
    }

    (*c).generate_grain_y = bd_fn!(decl_generate_grain_y_fn, BD, generate_grain_y, ssse3);
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
        bd_fn!(decl_generate_grain_uv_fn, BD, generate_grain_uv_420, ssse3);
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
        bd_fn!(decl_generate_grain_uv_fn, BD, generate_grain_uv_422, ssse3);
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
        bd_fn!(decl_generate_grain_uv_fn, BD, generate_grain_uv_444, ssse3);

    (*c).fgy_32x32xn = bd_fn!(decl_fgy_32x32xn_fn, BD, fgy_32x32xn, ssse3);
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
        bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32xn_i420, ssse3);
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
        bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32xn_i422, ssse3);
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
        bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32xn_i444, ssse3);

    #[cfg(target_arch = "x86_64")]
    {
        if !flags.contains(CpuFlags::AVX2) {
            return;
        }

        (*c).generate_grain_y = bd_fn!(decl_generate_grain_y_fn, BD, generate_grain_y, avx2);
        (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
            bd_fn!(decl_generate_grain_uv_fn, BD, generate_grain_uv_420, avx2);
        (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
            bd_fn!(decl_generate_grain_uv_fn, BD, generate_grain_uv_422, avx2);
        (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
            bd_fn!(decl_generate_grain_uv_fn, BD, generate_grain_uv_444, avx2);

        if !flags.contains(CpuFlags::SLOW_GATHER) {
            (*c).fgy_32x32xn = bd_fn!(decl_fgy_32x32xn_fn, BD, fgy_32x32xn, avx2);
            (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
                bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32xn_i420, avx2);
            (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
                bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32xn_i422, avx2);
            (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
                bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32xn_i444, avx2);
        }

        if !flags.contains(CpuFlags::AVX512ICL) {
            return;
        }

        (*c).fgy_32x32xn = bd_fn!(decl_fgy_32x32xn_fn, BD, fgy_32x32xn, avx512icl);
        (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
            bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32xn_i420, avx512icl);
        (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
            bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32xn_i422, avx512icl);
        (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
            bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32xn_i444, avx512icl);
    }
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn fgy_32x32xn_neon_erased<BD: BitDepth>(
    dst_row: *mut DynPixel,
    src_row: *const DynPixel,
    stride: ptrdiff_t,
    data: &Rav1dFilmGrainData,
    pw: usize,
    scaling: *const DynScaling,
    grain_lut: *const GrainLut<DynEntry>,
    bh: c_int,
    row_num: c_int,
    bitdepth_max: c_int,
) {
    let dst_row = dst_row.cast();
    let src_row = src_row.cast();
    let scaling = scaling.cast();
    let grain_lut = grain_lut.cast();
    let bd = BD::from_c(bitdepth_max);
    fgy_32x32xn_neon(
        dst_row, src_row, stride, data, pw, scaling, grain_lut, bh, row_num, bd,
    )
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe fn fgy_32x32xn_neon<BD: BitDepth>(
    dst_row: *mut BD::Pixel,
    src_row: *const BD::Pixel,
    stride: ptrdiff_t,
    data: &Rav1dFilmGrainData,
    pw: usize,
    scaling: *const BD::Scaling,
    grain_lut: *const GrainLut<BD::Entry>,
    bh: c_int,
    row_num: c_int,
    bd: BD,
) {
    let rows = 1 + (data.overlap_flag && row_num > 0) as c_int;

    // seed[0] contains the current row, seed[1] contains the previous
    let mut seed: [c_uint; 2] = [0; 2];
    for i in 0..rows {
        seed[i as usize] = data.seed;
        seed[i as usize] ^= (((row_num - i) * 37 + 178 & 0xff as c_int) << 8) as c_uint;
        seed[i as usize] ^= ((row_num - i) * 173 + 105 & 0xff as c_int) as c_uint;
    }

    let mut offsets: [[c_int; 2]; 2] = [[0; 2 /* row offset */]; 2 /* col offset */];

    // process this row in BLOCK_SIZE^2 blocks
    for bx in (0..pw).step_by(BLOCK_SIZE) {
        if data.overlap_flag && bx != 0 {
            // shift previous offsets left
            for i in 0..rows {
                offsets[1][i as usize] = offsets[0][i as usize];
            }
        }

        // update current offsets
        for i in 0..rows {
            offsets[0][i as usize] =
                get_random_number(8, &mut *seed.as_mut_ptr().offset(i as isize));
        }

        let mut r#type = 0;
        if data.overlap_flag && row_num != 0 {
            r#type |= 1 as c_int; // overlap y
        }
        if data.overlap_flag && bx != 0 {
            r#type |= 2 as c_int; // overlap x
        }

        bd_fn!(decl_fgy_32x32xn_fn, BD, fgy_32x32, neon)(
            dst_row.offset(bx as isize).cast(),
            src_row.offset(bx as isize).cast(),
            stride,
            scaling.cast(),
            data.scaling_shift,
            grain_lut.cast(),
            &offsets,
            bh,
            data.clip_to_restricted_range as ptrdiff_t,
            r#type as ptrdiff_t,
            bd.into_c(),
        );
    }
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn fguv_32x32xn_neon_erased<
    BD: BitDepth,
    const NM: usize,
    const IS_SX: bool,
    const IS_SY: bool,
>(
    dst_row: *mut DynPixel,
    src_row: *const DynPixel,
    stride: ptrdiff_t,
    data: &Rav1dFilmGrainData,
    pw: usize,
    scaling: *const DynScaling,
    grain_lut: *const GrainLut<DynEntry>,
    bh: c_int,
    row_num: c_int,
    luma_row: *const DynPixel,
    luma_stride: ptrdiff_t,
    uv: c_int,
    is_id: c_int,
    bitdepth_max: c_int,
) {
    let dst_row = dst_row.cast();
    let src_row = src_row.cast();
    let scaling = scaling.cast();
    let grain_lut = grain_lut.cast();
    let luma_row = luma_row.cast();
    let bd = BD::from_c(bitdepth_max);
    fguv_32x32xn_neon::<_, NM, IS_SX, IS_SY>(
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
        uv,
        is_id,
        bd,
    )
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe fn fguv_32x32xn_neon<BD: BitDepth, const NM: usize, const IS_SX: bool, const IS_SY: bool>(
    dst_row: *mut BD::Pixel,
    src_row: *const BD::Pixel,
    stride: ptrdiff_t,
    data: &Rav1dFilmGrainData,
    pw: usize,
    scaling: *const BD::Scaling,
    grain_lut: *const GrainLut<BD::Entry>,
    bh: c_int,
    row_num: c_int,
    luma_row: *const BD::Pixel,
    luma_stride: ptrdiff_t,
    uv: c_int,
    is_id: c_int,
    bd: BD,
) {
    let [sx, _sy] = [IS_SX, IS_SY].map(|it| it as c_int);

    let rows = 1 + (data.overlap_flag && row_num > 0) as c_int;

    // seed[0] contains the current row, seed[1] contains the previous
    let mut seed: [c_uint; 2] = [0; 2];
    for i in 0..rows {
        seed[i as usize] = data.seed;
        seed[i as usize] ^= (((row_num - i) * 37 + 178 & 0xff as c_int) << 8) as c_uint;
        seed[i as usize] ^= ((row_num - i) * 173 + 105 & 0xff as c_int) as c_uint;
    }

    let mut offsets: [[c_int; 2]; 2] = [[0; 2 /* row offset */]; 2 /* col offset */];

    // process this row in BLOCK_SIZE^2 blocks (subsampled)
    for bx in (0..pw).step_by(BLOCK_SIZE >> sx) {
        if data.overlap_flag && bx != 0 {
            // shift previous offsets left
            for i in 0..rows {
                offsets[1][i as usize] = offsets[0][i as usize];
            }
        }

        // update current offsets
        for i in 0..rows {
            offsets[0][i as usize] =
                get_random_number(8, &mut *seed.as_mut_ptr().offset(i as isize));
        }

        let mut r#type = 0;
        if data.overlap_flag && row_num != 0 {
            r#type |= 1 as c_int; // overlap y
        }
        if data.overlap_flag && bx != 0 {
            r#type |= 2 as c_int; // overlap x
        }
        if data.chroma_scaling_from_luma {
            r#type |= 4 as c_int;
        }
        (match NM {
            420 => bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32_420, neon),
            422 => bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32_422, neon),
            444 => bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32_444, neon),
            _ => unreachable!(),
        })(
            dst_row.offset(bx as isize).cast(),
            src_row.offset(bx as isize).cast(),
            stride,
            scaling.cast(),
            data,
            grain_lut.cast(),
            luma_row.offset((bx << sx) as isize).cast(),
            luma_stride,
            &offsets,
            bh as ptrdiff_t,
            uv as ptrdiff_t,
            is_id as ptrdiff_t,
            r#type as ptrdiff_t,
            bd.into_c(),
        );
    }
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
#[inline(always)]
unsafe fn film_grain_dsp_init_arm<BD: BitDepth>(c: *mut Rav1dFilmGrainDSPContext) {
    let flags = rav1d_get_cpu_flags();

    if !flags.contains(CpuFlags::NEON) {
        return;
    }

    (*c).generate_grain_y = bd_fn!(decl_generate_grain_y_fn, BD, generate_grain_y, neon);
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
        bd_fn!(decl_generate_grain_uv_fn, BD, generate_grain_uv_420, neon);
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
        bd_fn!(decl_generate_grain_uv_fn, BD, generate_grain_uv_422, neon);
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
        bd_fn!(decl_generate_grain_uv_fn, BD, generate_grain_uv_444, neon);

    (*c).fgy_32x32xn = fgy_32x32xn_neon_erased::<BD>;
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
        fguv_32x32xn_neon_erased::<BD, 420, true, true>;
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
        fguv_32x32xn_neon_erased::<BD, 422, true, false>;
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
        fguv_32x32xn_neon_erased::<BD, 444, false, false>;
}

#[cold]
pub unsafe fn rav1d_film_grain_dsp_init<BD: BitDepth>(c: *mut Rav1dFilmGrainDSPContext) {
    (*c).generate_grain_y = generate_grain_y_c_erased::<BD>;
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
        generate_grain_uv_c_erased::<BD, 420, true, true>;
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
        generate_grain_uv_c_erased::<BD, 422, true, false>;
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
        generate_grain_uv_c_erased::<BD, 444, false, false>;

    (*c).fgy_32x32xn = fgy_32x32xn_c_erased::<BD>;
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
        fguv_32x32xn_c_erased::<BD, 420, true, true>;
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
        fguv_32x32xn_c_erased::<BD, 422, true, false>;
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
        fguv_32x32xn_c_erased::<BD, 444, false, false>;

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            film_grain_dsp_init_x86::<BD>(c);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
            film_grain_dsp_init_arm::<BD>(c);
        }
    }
}
