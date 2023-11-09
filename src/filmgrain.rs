use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynEntry;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::bitdepth::DynScaling;
use crate::include::common::intops::iclip;
use crate::include::dav1d::headers::Dav1dFilmGrainData;
use crate::include::dav1d::headers::Rav1dFilmGrainData;
use crate::include::dav1d::headers::Rav1dPixelLayoutSubSampled;
use crate::src::enum_map::enum_map;
use crate::src::enum_map::DefaultValue;
use crate::src::enum_map::EnumMap;
use crate::src::internal::GrainLut;
use crate::src::tables::dav1d_gaussian_sequence;
use crate::src::wrap_fn_ptr::wrap_fn_ptr;
use crate::src::wrap_fn_ptr::WrappedFnPtr;
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
use crate::{include::common::bitdepth::bd_fn, src::cpu::rav1d_get_cpu_flags, src::cpu::CpuFlags};

pub const GRAIN_WIDTH: usize = 82;
pub const GRAIN_HEIGHT: usize = 73;

const BLOCK_SIZE: usize = 32;

const SUB_GRAIN_WIDTH: usize = 44;
const SUB_GRAIN_HEIGHT: usize = 38;

wrap_fn_ptr!(pub struct FnGenerateGrainY(unsafe extern "C" fn(
    buf: *mut GrainLut<DynEntry>,
    data: &Dav1dFilmGrainData,
    bitdepth_max: c_int,
) -> ()));

impl FnGenerateGrainY {
    pub unsafe fn call<BD: BitDepth>(
        &self,
        buf: &mut GrainLut<BD::Entry>,
        data: &Rav1dFilmGrainData,
        bd: BD,
    ) {
        let buf = (buf as *mut GrainLut<BD::Entry>).cast();
        let data = &data.clone().into();
        let bd = bd.into_c();
        (self.get())(buf, data, bd)
    }
}

wrap_fn_ptr!(pub struct FnGenerateGrainUV(unsafe extern "C" fn(
    buf: *mut GrainLut<DynEntry>,
    buf_y: *const GrainLut<DynEntry>,
    data: &Dav1dFilmGrainData,
    uv: intptr_t,
    bitdepth_max: c_int,
) -> ()));

impl FnGenerateGrainUV {
    pub unsafe fn call<BD: BitDepth>(
        &self,
        buf: &mut GrainLut<BD::Entry>,
        buf_y: &GrainLut<BD::Entry>,
        data: &Rav1dFilmGrainData,
        uv: bool,
        bd: BD,
    ) {
        let buf = (buf as *mut GrainLut<BD::Entry>).cast();
        let buf_y = (buf_y as *const GrainLut<BD::Entry>).cast();
        let data = &data.clone().into();
        let uv = uv.into();
        let bd = bd.into_c();
        (self.get())(buf, buf_y, data, uv, bd)
    }
}

wrap_fn_ptr!(pub struct FnFGY32x32xN(unsafe extern "C" fn(
    dst_row: *mut DynPixel,
    src_row: *const DynPixel,
    stride: ptrdiff_t,
    data: &Dav1dFilmGrainData,
    pw: usize,
    scaling: *const DynScaling,
    grain_lut: *const GrainLut<DynEntry>,
    bh: c_int,
    row_num: c_int,
    bitdepth_max: c_int,
) -> ()));

impl FnFGY32x32xN {
    pub unsafe fn call<BD: BitDepth>(
        &self,
        dst_row: *mut BD::Pixel,
        src_row: *const BD::Pixel,
        stride: ptrdiff_t,
        data: &Rav1dFilmGrainData,
        pw: usize,
        scaling: &BD::Scaling,
        grain_lut: &GrainLut<BD::Entry>,
        bh: usize,
        row_num: usize,
        bd: BD,
    ) {
        let dst_row = dst_row.cast();
        let src_row = src_row.cast();
        let data = &data.clone().into();
        let scaling = (scaling as *const BD::Scaling).cast();
        let grain_lut = (grain_lut as *const GrainLut<BD::Entry>).cast();
        let bh = bh as c_int;
        let row_num = row_num as c_int;
        let bd = bd.into_c();
        (self.get())(
            dst_row, src_row, stride, data, pw, scaling, grain_lut, bh, row_num, bd,
        )
    }
}

wrap_fn_ptr!(pub struct FnFGUV32x32xN(unsafe extern "C" fn(
    dst_row: *mut DynPixel,
    src_row: *const DynPixel,
    stride: ptrdiff_t,
    data: &Dav1dFilmGrainData,
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
) -> ()));

impl FnFGUV32x32xN {
    pub unsafe fn call<BD: BitDepth>(
        &self,
        dst_row: *mut BD::Pixel,
        src_row: *const BD::Pixel,
        stride: ptrdiff_t,
        data: &Rav1dFilmGrainData,
        pw: usize,
        scaling: &BD::Scaling,
        grain_lut: &GrainLut<BD::Entry>,
        bh: usize,
        row_num: usize,
        luma_row: *const BD::Pixel,
        luma_stride: ptrdiff_t,
        is_uv: bool,
        is_id: bool,
        bd: BD,
    ) {
        let dst_row = dst_row.cast();
        let src_row = src_row.cast();
        let data = &data.clone().into();
        let scaling = (scaling as *const BD::Scaling).cast();
        let grain_lut = (grain_lut as *const GrainLut<BD::Entry>).cast();
        let bh = bh as c_int;
        let row_num = row_num as c_int;
        let luma_row = luma_row.cast();
        let uv_pl = is_uv as c_int;
        let is_id = is_id as c_int;
        let bd = bd.into_c();
        (self.get())(
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
            bd,
        )
    }
}

#[repr(C)]
pub(crate) struct Rav1dFilmGrainDSPContext {
    pub generate_grain_y: FnGenerateGrainY,
    pub generate_grain_uv: EnumMap<Rav1dPixelLayoutSubSampled, FnGenerateGrainUV, 3>,
    pub fgy_32x32xn: FnFGY32x32xN,
    pub fguv_32x32xn: EnumMap<Rav1dPixelLayoutSubSampled, FnFGUV32x32xN, 3>,
}

#[cfg(feature = "asm")]
macro_rules! decl_generate_grain_y_fn {
    (fn $name:ident) => {{
        extern "C" {
            fn $name(buf: *mut GrainLut<DynEntry>, data: &Dav1dFilmGrainData, bitdepth_max: c_int);
        }

        FnGenerateGrainY::new($name)
    }};
}

#[cfg(feature = "asm")]
macro_rules! decl_generate_grain_uv_fn {
    (fn $name:ident) => {{
        extern "C" {
            fn $name(
                buf: *mut GrainLut<DynEntry>,
                buf_y: *const GrainLut<DynEntry>,
                data: &Dav1dFilmGrainData,
                uv: intptr_t,
                bitdepth_max: c_int,
            );
        }

        FnGenerateGrainUV::new($name)
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
                data: &Dav1dFilmGrainData,
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

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        let fn_ = FnFGY32x32xN::new($name);

        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
        let fn_ = $name;

        fn_
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
                data: &Dav1dFilmGrainData,
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
                data: &Dav1dFilmGrainData,
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

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        let fn_ = FnFGUV32x32xN::new($name);

        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
        let fn_ = $name;

        fn_
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
fn round2<T>(x: T, shift: u8) -> T
where
    T: Add<Output = T> + From<u8> + Shl<u8, Output = T> + Shr<u8, Output = T>,
{
    (x + (T::from(1) << shift >> 1)) >> shift
}

/// For the returned `seed: [c_uint; 2]` array,
/// `seed[0]` contains the current row, and
/// `seed[1]` contains the previous row.
fn row_seed(rows: usize, row_num: usize, data: &Rav1dFilmGrainData) -> [c_uint; 2] {
    let mut seed: [c_uint; 2] = [0; 2];
    for i in 0..rows {
        seed[i] = data.seed;
        seed[i] ^= (((row_num - i) * 37 + 178 & 0xFF) << 8) as c_uint;
        seed[i] ^= ((row_num - i) * 173 + 105 & 0xFF) as c_uint;
    }
    seed
}

unsafe extern "C" fn generate_grain_y_c_erased<BD: BitDepth>(
    buf: *mut GrainLut<DynEntry>,
    data: &Dav1dFilmGrainData,
    bitdepth_max: c_int,
) {
    // Safety: Casting back to the original type from the `fn` ptr call.
    let buf = unsafe { &mut *buf.cast() };
    let data = &data.clone().into();
    let bd = BD::from_c(bitdepth_max);
    generate_grain_y_rust(buf, data, bd)
}

unsafe fn generate_grain_y_rust<BD: BitDepth>(
    buf: &mut GrainLut<BD::Entry>,
    data: &Rav1dFilmGrainData,
    bd: BD,
) {
    let bitdepth_min_8 = bd.bitdepth() - 8;
    let mut seed: c_uint = data.seed;
    let shift = 4 - bitdepth_min_8 + data.grain_scale_shift;
    let grain_ctr = (128 as c_int) << bitdepth_min_8;
    let grain_min = -grain_ctr;
    let grain_max = grain_ctr - 1;

    for y in 0..GRAIN_HEIGHT {
        for x in 0..GRAIN_WIDTH {
            let value = get_random_number(11, &mut seed);
            buf[y][x] = round2(dav1d_gaussian_sequence[value as usize], shift).as_::<BD::Entry>();
        }
    }

    let ar_pad = 3;
    let ar_lag = data.ar_coeff_lag as isize;

    for y in ar_pad..GRAIN_HEIGHT {
        for x in ar_pad..GRAIN_WIDTH - ar_pad {
            let mut coeff: *const i8 = (data.ar_coeffs_y).as_ptr();
            let mut sum = 0;
            for dy in -ar_lag..=0 {
                for dx in -ar_lag..=ar_lag {
                    if dx == 0 && dy == 0 {
                        break;
                    }
                    let fresh0 = coeff;
                    coeff = coeff.offset(1);
                    sum += *fresh0 as c_int
                        * buf[(y as isize + dy) as usize][(x as isize + dx) as usize]
                            .as_::<c_int>();
                }
            }

            let grain = buf[y][x].as_::<c_int>() + round2(sum, data.ar_coeff_shift);
            buf[y][x] = iclip(grain, grain_min, grain_max).as_::<BD::Entry>();
        }
    }
}

#[inline(never)]
unsafe fn generate_grain_uv_rust<BD: BitDepth>(
    buf: &mut GrainLut<BD::Entry>,
    buf_y: &GrainLut<BD::Entry>,
    data: &Rav1dFilmGrainData,
    uv: bool,
    is_subx: bool,
    is_suby: bool,
    bd: BD,
) {
    let [subx, suby] = [is_subx, is_suby].map(|it| it as u8);

    let bitdepth_min_8 = bd.bitdepth() - 8;
    let mut seed: c_uint =
        data.seed ^ (if uv { 0x49d8 as c_int } else { 0xb524 as c_int }) as c_uint;
    let shift = 4 - bitdepth_min_8 + data.grain_scale_shift;
    let grain_ctr = (128 as c_int) << bitdepth_min_8;
    let grain_min = -grain_ctr;
    let grain_max = grain_ctr - 1;

    let chromaW = if is_subx {
        SUB_GRAIN_WIDTH
    } else {
        GRAIN_WIDTH
    };
    let chromaH = if is_suby {
        SUB_GRAIN_HEIGHT
    } else {
        GRAIN_HEIGHT
    };

    for y in 0..chromaH {
        for x in 0..chromaW {
            let value = get_random_number(11, &mut seed);
            buf[y][x] = round2(dav1d_gaussian_sequence[value as usize], shift).as_::<BD::Entry>();
        }
    }

    let ar_pad = 3;
    let ar_lag = data.ar_coeff_lag as isize;

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
                                luma +=
                                    buf_y[lumaY + i as usize][lumaX + j as usize].as_::<c_int>();
                            }
                        }
                        luma = round2(luma, subx + suby);

                        sum += luma * *coeff as c_int;
                        break;
                    } else {
                        let fresh1 = coeff;
                        coeff = coeff.offset(1);

                        sum += *fresh1 as c_int
                            * buf[(y as isize + dy) as usize][(x as isize + dx) as usize]
                                .as_::<c_int>();
                    }
                }
            }

            let grain = buf[y][x].as_::<c_int>() + round2(sum, data.ar_coeff_shift);
            buf[y][x] = iclip(grain, grain_min, grain_max).as_::<BD::Entry>();
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
    data: &Dav1dFilmGrainData,
    uv: intptr_t,
    bitdepth_max: c_int,
) {
    // Safety: Casting back to the original type from the `fn` ptr call.
    let buf = unsafe { &mut *buf.cast() };
    // Safety: Casting back to the original type from the `fn` ptr call.
    let buf_y = unsafe { &*buf_y.cast() };
    let data = &data.clone().into();
    let uv = uv != 0;
    let bd = BD::from_c(bitdepth_max);
    generate_grain_uv_rust(buf, buf_y, data, uv, IS_SUBX, IS_SUBY, bd)
}

/// Sample from the correct block of a grain LUT,
/// while taking into account the offsets
/// provided by the offsets cache.
#[inline]
fn sample_lut<BD: BitDepth>(
    grain_lut: &GrainLut<BD::Entry>,
    offsets: &[[c_int; 2]; 2],
    is_subx: bool,
    is_suby: bool,
    is_bx: bool,
    is_by: bool,
    x: usize,
    y: usize,
) -> i32 {
    let [subx, suby, bx, by] = [is_subx, is_suby, is_bx, is_by].map(|it| it as usize);

    let randval = offsets[bx][by] as usize;
    let offx = 3 + (2 >> subx) * (3 + (randval >> 4));
    let offy = 3 + (2 >> suby) * (3 + (randval & ((1 << 4) - 1)));
    grain_lut[offy + y + (BLOCK_SIZE >> suby) * by][offx + x + (BLOCK_SIZE >> subx) * bx]
        .as_::<i32>()
}

unsafe extern "C" fn fgy_32x32xn_c_erased<BD: BitDepth>(
    dst_row: *mut DynPixel,
    src_row: *const DynPixel,
    stride: ptrdiff_t,
    data: &Dav1dFilmGrainData,
    pw: usize,
    scaling: *const DynScaling,
    grain_lut: *const GrainLut<DynEntry>,
    bh: c_int,
    row_num: c_int,
    bitdepth_max: c_int,
) {
    let dst_row = dst_row.cast();
    let src_row = src_row.cast();
    let data = &data.clone().into();
    // Safety: Casting back to the original type from the `fn` ptr call.
    let scaling = unsafe { &*scaling.cast() };
    // Safety: Casting back to the original type from the `fn` ptr call.
    let grain_lut = unsafe { &*grain_lut.cast() };
    let bh = bh as usize;
    let row_num = row_num as usize;
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
    scaling: &BD::Scaling,
    grain_lut: &GrainLut<BD::Entry>,
    bh: usize,
    row_num: usize,
    bd: BD,
) {
    let rows = 1 + (data.overlap_flag && row_num > 0) as usize;
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

    let mut seed = row_seed(rows, row_num, data);

    assert!((stride as usize % (BLOCK_SIZE * ::core::mem::size_of::<BD::Pixel>())) == 0);

    let mut offsets: [[c_int; 2]; 2] = [[0; 2 /* row offset */]; 2 /* col offset */];

    // process this row in BLOCK_SIZE^2 blocks
    for bx in (0..pw).step_by(BLOCK_SIZE) {
        let bw = cmp::min(BLOCK_SIZE, pw - bx);

        if data.overlap_flag && bx != 0 {
            // shift previous offsets left
            for i in 0..rows {
                offsets[1][i] = offsets[0][i];
            }
        }

        // update current offsets
        for i in 0..rows {
            offsets[0][i] = get_random_number(8, &mut seed[i]);
        }

        // x/y block offsets to compensate for overlapped regions
        let ystart = if data.overlap_flag && row_num != 0 {
            cmp::min(2, bh)
        } else {
            0
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
                scaling.as_ref()[(*src).to::<usize>()] as c_int * grain,
                data.scaling_shift,
            );
            *dst = iclip((*src).as_::<c_int>() + noise, min_value, max_value).as_::<BD::Pixel>();
        };

        for y in ystart..bh {
            // Non-overlapped image region (straightforward)
            for x in xstart..bw {
                let grain = sample_lut::<BD>(grain_lut, &offsets, false, false, false, false, x, y);
                add_noise_y(x, y, grain);
            }

            // Special case for overlapped column
            for x in 0..xstart {
                let grain = sample_lut::<BD>(grain_lut, &offsets, false, false, false, false, x, y);
                let old = sample_lut::<BD>(grain_lut, &offsets, false, false, true, false, x, y);
                let grain = round2(old * w[x][0] + grain * w[x][1], 5);
                let grain = iclip(grain, grain_min, grain_max);
                add_noise_y(x, y, grain);
            }
        }
        for y in 0..ystart {
            // Special case for overlapped row (sans corner)
            for x in xstart..bw {
                let grain = sample_lut::<BD>(grain_lut, &offsets, false, false, false, false, x, y);
                let old = sample_lut::<BD>(grain_lut, &offsets, false, false, false, true, x, y);
                let grain = round2(old * w[y][0] + grain * w[y][1], 5);
                let grain = iclip(grain, grain_min, grain_max);
                add_noise_y(x, y, grain);
            }

            // Special case for doubly-overlapped corner
            for x in 0..xstart {
                // Blend the top pixel with the top left block
                let top = sample_lut::<BD>(grain_lut, &offsets, false, false, false, true, x, y);
                let old = sample_lut::<BD>(grain_lut, &offsets, false, false, true, true, x, y);
                let top = round2(old * w[x][0] + top * w[x][1], 5);
                let top = iclip(top, grain_min, grain_max);

                // Blend the current pixel with the left block
                let grain = sample_lut::<BD>(grain_lut, &offsets, false, false, false, false, x, y);
                let old = sample_lut::<BD>(grain_lut, &offsets, false, false, true, false, x, y);

                // Mix the row rows together and apply grain
                let grain = round2(old * w[x][0] + grain * w[x][1], 5);
                let grain = iclip(grain, grain_min, grain_max);
                let grain = round2(top * w[y][0] + grain * w[y][1], 5);
                let grain = iclip(grain, grain_min, grain_max);
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
    scaling: &BD::Scaling,
    grain_lut: &GrainLut<BD::Entry>,
    bh: usize,
    row_num: usize,
    luma_row: *const BD::Pixel,
    luma_stride: ptrdiff_t,
    is_uv: bool,
    is_id: bool,
    is_sx: bool,
    is_sy: bool,
    bd: BD,
) {
    let [uv, sx, sy] = [is_uv, is_sx, is_sy].map(|it| it as usize);

    let rows = 1 + (data.overlap_flag && row_num > 0) as usize;
    let bitdepth_min_8 = bd.bitdepth() - 8;
    let grain_ctr = (128 as c_int) << bitdepth_min_8;
    let grain_min = -grain_ctr;
    let grain_max = grain_ctr - 1;

    let min_value;
    let max_value;
    if data.clip_to_restricted_range {
        min_value = (16 as c_int) << bitdepth_min_8;
        max_value = (if is_id { 235 as c_int } else { 240 as c_int }) << bitdepth_min_8;
    } else {
        min_value = 0 as c_int;
        max_value = bd.bitdepth_max().as_::<c_int>();
    }

    let mut seed = row_seed(rows, row_num, data);

    assert!((stride as usize % (BLOCK_SIZE * ::core::mem::size_of::<BD::Pixel>())) == 0);

    let mut offsets: [[c_int; 2]; 2] = [[0; 2 /* row offset */]; 2 /* col offset */];

    // process this row in BLOCK_SIZE^2 blocks (subsampled)
    for bx in (0..pw).step_by(BLOCK_SIZE >> sx) {
        let bw = cmp::min(BLOCK_SIZE >> sx, pw - bx);
        if data.overlap_flag && bx != 0 {
            // shift previous offsets left
            for i in 0..rows {
                offsets[1][i] = offsets[0][i];
            }
        }

        // update current offsets
        for i in 0..rows {
            offsets[0][i] = get_random_number(8, &mut seed[i]);
        }

        // x/y block offsets to compensate for overlapped regions
        let ystart = if data.overlap_flag && row_num != 0 {
            cmp::min(2 >> sy, bh)
        } else {
            0
        };
        let xstart = if data.overlap_flag && bx != 0 {
            cmp::min(2 >> sx, bw)
        } else {
            0
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
                let combined = avg.as_::<c_int>() * data.uv_luma_mult[uv]
                    + (*src).as_::<c_int>() * data.uv_mult[uv];
                val = iclip(
                    (combined >> 6) + data.uv_offset[uv] * ((1 as c_int) << bitdepth_min_8),
                    0,
                    bd.bitdepth_max().as_::<c_int>(),
                );
            }
            let noise = round2(
                scaling.as_ref()[val as usize] as c_int * grain,
                data.scaling_shift,
            );
            *dst = iclip((*src).as_::<c_int>() + noise, min_value, max_value).as_::<BD::Pixel>();
        };

        for y in ystart..bh {
            // Non-overlapped image region (straightforward)
            for x in xstart..bw {
                let grain = sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, false, false, x, y);
                add_noise_uv(x, y, grain);
            }

            // Special case for overlapped column
            for x in 0..xstart {
                let grain = sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, false, false, x, y);
                let old = sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, true, false, x, y);
                let grain = round2(old * w[sx][x][0] + grain * w[sx][x][1], 5);
                let grain = iclip(grain, grain_min, grain_max);
                add_noise_uv(x, y, grain);
            }
        }
        for y in 0..ystart {
            // Special case for overlapped row (sans corner)
            for x in xstart..bw {
                let grain = sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, false, false, x, y);
                let old = sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, false, true, x, y);
                let grain = round2(old * w[sy][y][0] + grain * w[sy][y][1], 5);
                let grain = iclip(grain, grain_min, grain_max);
                add_noise_uv(x, y, grain);
            }

            // Special case for doubly-overlapped corner
            for x in 0..xstart {
                // Blend the top pixel with the top left block
                let top = sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, false, true, x, y);
                let old = sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, true, true, x, y);
                let top = round2(old * w[sx][x][0] + top * w[sx][x][1], 5);
                let top = iclip(top, grain_min, grain_max);

                // Blend the current pixel with the left block
                let grain = sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, false, false, x, y);
                let old = sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, true, false, x, y);

                // Mix the row rows together and apply to image
                let grain = round2(old * w[sx][x][0] + grain * w[sx][x][1], 5);
                let grain = iclip(grain, grain_min, grain_max);
                let grain = round2(top * w[sy][y][0] + grain * w[sy][y][1], 5);
                let grain = iclip(grain, grain_min, grain_max);
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
    data: &Dav1dFilmGrainData,
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
    let data = &data.clone().into();
    // Safety: Casting back to the original type from the `fn` ptr call.
    let scaling = unsafe { &*scaling.cast() };
    // Safety: Casting back to the original type from the `fn` ptr call.
    let grain_lut = unsafe { &*grain_lut.cast() };
    let bh = bh as usize;
    let row_num = row_num as usize;
    let luma_row = luma_row.cast();
    let uv_pl = uv_pl as usize;
    let is_id = is_id != 0;
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
        uv_pl != 0,
        is_id,
        IS_SX,
        IS_SY,
        bd,
    )
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn fgy_32x32xn_neon_erased<BD: BitDepth>(
    dst_row: *mut DynPixel,
    src_row: *const DynPixel,
    stride: ptrdiff_t,
    data: &Dav1dFilmGrainData,
    pw: usize,
    scaling: *const DynScaling,
    grain_lut: *const GrainLut<DynEntry>,
    bh: c_int,
    row_num: c_int,
    bitdepth_max: c_int,
) {
    let dst_row = dst_row.cast();
    let src_row = src_row.cast();
    let data = &data.clone().into();
    let scaling = scaling.cast();
    let grain_lut = grain_lut.cast();
    let row_num = row_num as usize;
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
    row_num: usize,
    bd: BD,
) {
    let rows = 1 + (data.overlap_flag && row_num > 0) as usize;

    let mut seed = row_seed(rows, row_num, data);

    let mut offsets: [[c_int; 2]; 2] = [[0; 2 /* row offset */]; 2 /* col offset */];

    // process this row in BLOCK_SIZE^2 blocks
    for bx in (0..pw).step_by(BLOCK_SIZE) {
        if data.overlap_flag && bx != 0 {
            // shift previous offsets left
            for i in 0..rows {
                offsets[1][i] = offsets[0][i];
            }
        }

        // update current offsets
        for i in 0..rows {
            offsets[0][i] = get_random_number(8, &mut seed[i]);
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
            data.scaling_shift.into(),
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
    data: &Dav1dFilmGrainData,
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
    let data_c = data;
    let data = &data.clone().into();
    let scaling = scaling.cast();
    let grain_lut = grain_lut.cast();
    let row_num = row_num as usize;
    let luma_row = luma_row.cast();
    let bd = BD::from_c(bitdepth_max);
    fguv_32x32xn_neon::<_, NM, IS_SX, IS_SY>(
        dst_row,
        src_row,
        stride,
        data,
        data_c,
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
    data_c: &Dav1dFilmGrainData,
    pw: usize,
    scaling: *const BD::Scaling,
    grain_lut: *const GrainLut<BD::Entry>,
    bh: c_int,
    row_num: usize,
    luma_row: *const BD::Pixel,
    luma_stride: ptrdiff_t,
    uv: c_int,
    is_id: c_int,
    bd: BD,
) {
    let [sx, _sy] = [IS_SX, IS_SY].map(|it| it as c_int);

    let rows = 1 + (data.overlap_flag && row_num > 0) as usize;

    let mut seed = row_seed(rows, row_num, data);

    let mut offsets: [[c_int; 2]; 2] = [[0; 2 /* row offset */]; 2 /* col offset */];

    // process this row in BLOCK_SIZE^2 blocks (subsampled)
    for bx in (0..pw).step_by(BLOCK_SIZE >> sx) {
        if data.overlap_flag && bx != 0 {
            // shift previous offsets left
            for i in 0..rows {
                offsets[1][i] = offsets[0][i];
            }
        }

        // update current offsets
        for i in 0..rows {
            offsets[0][i] = get_random_number(8, &mut seed[i]);
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
            data_c,
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

impl Rav1dFilmGrainDSPContext {
    const fn new_c<BD: BitDepth>() -> Self {
        Self {
            generate_grain_y: FnGenerateGrainY::new(generate_grain_y_c_erased::<BD>),
            generate_grain_uv: enum_map!(Rav1dPixelLayoutSubSampled => FnGenerateGrainUV; match key {
                I420 => FnGenerateGrainUV::new(generate_grain_uv_c_erased::<BD, 420, true, true>),
                I422 => FnGenerateGrainUV::new(generate_grain_uv_c_erased::<BD, 422, true, false>),
                I444 => FnGenerateGrainUV::new(generate_grain_uv_c_erased::<BD, 444, false, false>),
            }),
            fgy_32x32xn: FnFGY32x32xN::new(fgy_32x32xn_c_erased::<BD>),
            fguv_32x32xn: enum_map!(Rav1dPixelLayoutSubSampled => FnFGUV32x32xN; match key {
                I420 => FnFGUV32x32xN::new(fguv_32x32xn_c_erased::<BD, 420, true, true>),
                I422 => FnFGUV32x32xN::new(fguv_32x32xn_c_erased::<BD, 422, true, false>),
                I444 => FnFGUV32x32xN::new(fguv_32x32xn_c_erased::<BD, 444, false, false>),
            }),
        }
    }

    #[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
    #[inline(always)]
    const fn init_x86<BD: BitDepth>(mut self, flags: CpuFlags) -> Self {
        if !flags.contains(CpuFlags::SSSE3) {
            return self;
        }

        self.generate_grain_y = bd_fn!(decl_generate_grain_y_fn, BD, generate_grain_y, ssse3);
        self.generate_grain_uv = enum_map!(Rav1dPixelLayoutSubSampled => FnGenerateGrainUV; match key {
            I420 => bd_fn!(decl_generate_grain_uv_fn, BD, generate_grain_uv_420, ssse3),
            I422 => bd_fn!(decl_generate_grain_uv_fn, BD, generate_grain_uv_422, ssse3),
            I444 => bd_fn!(decl_generate_grain_uv_fn, BD, generate_grain_uv_444, ssse3),
        });

        self.fgy_32x32xn = bd_fn!(decl_fgy_32x32xn_fn, BD, fgy_32x32xn, ssse3);
        self.fguv_32x32xn = enum_map!(Rav1dPixelLayoutSubSampled => FnFGUV32x32xN; match key {
            I420 => bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32xn_i420, ssse3),
            I422 => bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32xn_i422, ssse3),
            I444 => bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32xn_i444, ssse3),
        });

        #[cfg(target_arch = "x86_64")]
        {
            if !flags.contains(CpuFlags::AVX2) {
                return self;
            }

            self.generate_grain_y = bd_fn!(decl_generate_grain_y_fn, BD, generate_grain_y, avx2);
            self.generate_grain_uv = enum_map!(Rav1dPixelLayoutSubSampled => FnGenerateGrainUV; match key {
                I420 => bd_fn!(decl_generate_grain_uv_fn, BD, generate_grain_uv_420, avx2),
                I422 => bd_fn!(decl_generate_grain_uv_fn, BD, generate_grain_uv_422, avx2),
                I444 => bd_fn!(decl_generate_grain_uv_fn, BD, generate_grain_uv_444, avx2),
            });

            if !flags.contains(CpuFlags::SLOW_GATHER) {
                self.fgy_32x32xn = bd_fn!(decl_fgy_32x32xn_fn, BD, fgy_32x32xn, avx2);
                self.fguv_32x32xn = enum_map!(Rav1dPixelLayoutSubSampled => FnFGUV32x32xN; match key {
                    I420 => bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32xn_i420, avx2),
                    I422 => bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32xn_i422, avx2),
                    I444 => bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32xn_i444, avx2),
                });
            }

            if !flags.contains(CpuFlags::AVX512ICL) {
                return self;
            }

            self.fgy_32x32xn = bd_fn!(decl_fgy_32x32xn_fn, BD, fgy_32x32xn, avx512icl);
            self.fguv_32x32xn = enum_map!(Rav1dPixelLayoutSubSampled => FnFGUV32x32xN; match key {
                I420 => bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32xn_i420, avx512icl),
                I422 => bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32xn_i422, avx512icl),
                I444 => bd_fn!(decl_fguv_32x32xn_fn, BD, fguv_32x32xn_i444, avx512icl),
            });
        }

        self
    }

    #[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
    #[inline(always)]
    const fn init_arm<BD: BitDepth>(mut self, flags: CpuFlags) -> Self {
        if !flags.contains(CpuFlags::NEON) {
            return self;
        }

        self.generate_grain_y = bd_fn!(decl_generate_grain_y_fn, BD, generate_grain_y, neon);
        self.generate_grain_uv = enum_map!(Rav1dPixelLayoutSubSampled => FnGenerateGrainUV; match key {
            I420 => bd_fn!(decl_generate_grain_uv_fn, BD, generate_grain_uv_420, neon),
            I422 => bd_fn!(decl_generate_grain_uv_fn, BD, generate_grain_uv_422, neon),
            I444 => bd_fn!(decl_generate_grain_uv_fn, BD, generate_grain_uv_444, neon),
        });

        self.fgy_32x32xn = FnFGY32x32xN::new(fgy_32x32xn_neon_erased::<BD>);
        self.fguv_32x32xn = enum_map!(Rav1dPixelLayoutSubSampled => FnFGUV32x32xN; match key {
            I420 => FnFGUV32x32xN::new(fguv_32x32xn_neon_erased::<BD, 420, true, true>),
            I422 => FnFGUV32x32xN::new(fguv_32x32xn_neon_erased::<BD, 422, true, false>),
            I444 => FnFGUV32x32xN::new(fguv_32x32xn_neon_erased::<BD, 444, false, false>),
        });

        self
    }

    fn init<BD: BitDepth>(self) -> Self {
        #[cfg(feature = "asm")]
        {
            let flags = rav1d_get_cpu_flags();

            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            {
                return self.init_x86::<BD>(flags);
            }
            #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
            {
                return self.init_arm::<BD>(flags);
            }
        }

        #[allow(unreachable_code)] // Reachable on some #[cfg]s.
        self
    }

    #[cold]
    pub fn new<BD: BitDepth>() -> Self {
        Self::new_c::<BD>().init::<BD>()
    }
}
