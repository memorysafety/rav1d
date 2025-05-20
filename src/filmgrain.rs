#![deny(unsafe_op_in_unsafe_fn)]

use crate::cpu::CpuFlags;
use crate::enum_map::enum_map;
use crate::enum_map::enum_map_ty;
use crate::enum_map::DefaultValue;
use crate::ffi_safe::FFISafe;
use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynEntry;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::bitdepth::DynScaling;
use crate::include::common::intops::iclip;
use crate::include::dav1d::headers::Dav1dFilmGrainData;
use crate::include::dav1d::headers::Rav1dFilmGrainData;
use crate::include::dav1d::headers::Rav1dPixelLayoutSubSampled;
use crate::include::dav1d::picture::Rav1dPictureDataComponent;
use crate::include::dav1d::picture::Rav1dPictureDataComponentOffset;
use crate::internal::GrainLut;
use crate::strided::Strided as _;
use crate::tables::dav1d_gaussian_sequence;
use crate::wrap_fn_ptr::wrap_fn_ptr;
use libc::intptr_t;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::hint::assert_unchecked;
use std::mem;
use std::ops::Add;
use std::ops::Shl;
use std::ops::Shr;
use std::ptr;
use to_method::To;

#[cfg(all(
    feature = "asm",
    not(any(target_arch = "riscv64", target_arch = "riscv32"))
))]
use crate::include::common::bitdepth::bd_fn;

pub const GRAIN_WIDTH: usize = 82;
pub const GRAIN_HEIGHT: usize = 73;

pub const FG_BLOCK_SIZE: usize = 32;

const SUB_GRAIN_WIDTH: usize = 44;
const SUB_GRAIN_HEIGHT: usize = 38;

wrap_fn_ptr!(pub unsafe extern "C" fn generate_grain_y(
    buf: *mut GrainLut<DynEntry>,
    data: &Dav1dFilmGrainData,
    bitdepth_max: c_int,
) -> ());

impl generate_grain_y::Fn {
    pub fn call<BD: BitDepth>(
        &self,
        buf: &mut GrainLut<BD::Entry>,
        data: &Rav1dFilmGrainData,
        bd: BD,
    ) {
        let buf = ptr::from_mut(buf).cast();
        let data = &data.clone().into();
        let bd = bd.into_c();
        // SAFETY: Fallback `fn generate_grain_y_rust` is safe; asm is supposed to do the same.
        unsafe { self.get()(buf, data, bd) }
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn generate_grain_uv(
    buf: *mut GrainLut<DynEntry>,
    buf_y: *const GrainLut<DynEntry>,
    data: &Dav1dFilmGrainData,
    uv: intptr_t,
    bitdepth_max: c_int,
) -> ());

impl generate_grain_uv::Fn {
    pub fn call<BD: BitDepth>(
        &self,
        buf: &mut GrainLut<BD::Entry>,
        buf_y: &GrainLut<BD::Entry>,
        data: &Rav1dFilmGrainData,
        is_uv: bool,
        bd: BD,
    ) {
        let buf = ptr::from_mut(buf).cast();
        let buf_y = ptr::from_ref(buf_y).cast();
        let data = &data.clone().into();
        let uv = is_uv.into();
        let bd = bd.into_c();
        // SAFETY: Fallback `fn generate_grain_uv_rust` is safe; asm is supposed to do the same.
        unsafe { self.get()(buf, buf_y, data, uv, bd) }
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn fgy_32x32xn(
    dst_row_ptr: *mut DynPixel,
    src_row_ptr: *const DynPixel,
    stride: ptrdiff_t,
    data: &Dav1dFilmGrainData,
    pw: usize,
    scaling: *const DynScaling,
    grain_lut: *const GrainLut<DynEntry>,
    bh: c_int,
    row_num: c_int,
    bitdepth_max: c_int,
    _dst_row: *const FFISafe<Rav1dPictureDataComponentOffset>,
    _src_src: *const FFISafe<Rav1dPictureDataComponentOffset>,
) -> ());

impl fgy_32x32xn::Fn {
    pub fn call<BD: BitDepth>(
        &self,
        dst: &Rav1dPictureDataComponent,
        src: &Rav1dPictureDataComponent,
        data: &Rav1dFilmGrainData,
        pw: usize,
        scaling: &BD::Scaling,
        grain_lut: &GrainLut<BD::Entry>,
        bh: usize,
        row_num: usize,
        bd: BD,
    ) {
        let row_strides = (row_num * FG_BLOCK_SIZE) as isize;
        let dst_row = dst.with_offset::<BD>() + row_strides * dst.pixel_stride::<BD>();
        let src_row = src.with_offset::<BD>() + row_strides * src.pixel_stride::<BD>();
        let dst_row_ptr = dst_row.as_mut_ptr::<BD>().cast();
        let src_row_ptr = src_row.as_ptr::<BD>().cast();
        let stride = dst.stride();
        let data = &data.clone().into();
        let scaling = ptr::from_ref(scaling).cast();
        let grain_lut = ptr::from_ref(grain_lut).cast();
        let bh = bh as c_int;
        let row_num = row_num as c_int;
        let bd = bd.into_c();
        let dst_row = FFISafe::new(&dst_row);
        let src_row = FFISafe::new(&src_row);
        // SAFETY: Fallback `fn fgy_32x32xn_rust` is safe; asm is supposed to do the same.
        unsafe {
            self.get()(
                dst_row_ptr,
                src_row_ptr,
                stride,
                data,
                pw,
                scaling,
                grain_lut,
                bh,
                row_num,
                bd,
                dst_row,
                src_row,
            )
        }
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn fguv_32x32xn(
    dst_row_ptr: *mut DynPixel,
    src_row_ptr: *const DynPixel,
    stride: ptrdiff_t,
    data: &Dav1dFilmGrainData,
    pw: usize,
    scaling: *const DynScaling,
    grain_lut: *const GrainLut<DynEntry>,
    bh: c_int,
    row_num: c_int,
    luma_row_ptr: *const DynPixel,
    luma_stride: ptrdiff_t,
    uv_pl: c_int,
    is_id: c_int,
    bitdepth_max: c_int,
    _dst_row: *const FFISafe<Rav1dPictureDataComponentOffset>,
    _src_row: *const FFISafe<Rav1dPictureDataComponentOffset>,
    _luma_row: *const FFISafe<Rav1dPictureDataComponentOffset>,
) -> ());

impl fguv_32x32xn::Fn {
    pub fn call<BD: BitDepth>(
        &self,
        layout: Rav1dPixelLayoutSubSampled,
        dst: &Rav1dPictureDataComponent,
        src: &Rav1dPictureDataComponent,
        data: &Rav1dFilmGrainData,
        pw: usize,
        scaling: &BD::Scaling,
        grain_lut: &GrainLut<BD::Entry>,
        bh: usize,
        row_num: usize,
        luma: &Rav1dPictureDataComponent,
        is_uv: bool,
        is_id: bool,
        bd: BD,
    ) {
        let ss_y = (layout == Rav1dPixelLayoutSubSampled::I420) as usize;
        let row_strides = (row_num * FG_BLOCK_SIZE) as isize;
        let dst_row = dst.with_offset::<BD>() + (row_strides * dst.pixel_stride::<BD>() >> ss_y);
        let src_row = src.with_offset::<BD>() + (row_strides * src.pixel_stride::<BD>() >> ss_y);
        let dst_row_ptr = dst_row.as_mut_ptr::<BD>().cast();
        let src_row_ptr = src_row.as_ptr::<BD>().cast();
        let stride = dst.stride();
        let data = &data.clone().into();
        let scaling = (scaling as *const BD::Scaling).cast();
        let grain_lut = (grain_lut as *const GrainLut<BD::Entry>).cast();
        let bh = bh as c_int;
        let row_num = row_num as c_int;
        let luma_row = luma.with_offset::<BD>() + (row_strides * luma.pixel_stride::<BD>());
        let luma_row_ptr = luma_row.as_ptr::<BD>().cast();
        let luma_stride = luma.stride();
        let uv_pl = is_uv as c_int;
        let is_id = is_id as c_int;
        let bd = bd.into_c();
        let dst_row = FFISafe::new(&dst_row);
        let src_row = FFISafe::new(&src_row);
        let luma_row = FFISafe::new(&luma_row);
        // SAFETY: Fallback `fn fguv_32x32xn_rust` is safe; asm is supposed to do the same.
        unsafe {
            self.get()(
                dst_row_ptr,
                src_row_ptr,
                stride,
                data,
                pw,
                scaling,
                grain_lut,
                bh,
                row_num,
                luma_row_ptr,
                luma_stride,
                uv_pl,
                is_id,
                bd,
                dst_row,
                src_row,
                luma_row,
            )
        }
    }
}

pub struct Rav1dFilmGrainDSPContext {
    pub generate_grain_y: generate_grain_y::Fn,
    pub generate_grain_uv: enum_map_ty!(Rav1dPixelLayoutSubSampled, generate_grain_uv::Fn),
    pub fgy_32x32xn: fgy_32x32xn::Fn,
    pub fguv_32x32xn: enum_map_ty!(Rav1dPixelLayoutSubSampled, fguv_32x32xn::Fn),
}

#[inline]
fn get_random_number(bits: u8, state: &mut c_uint) -> c_int {
    let r = *state;
    let bit = (r ^ (r >> 1) ^ (r >> 3) ^ (r >> 12)) & 1;
    *state = (r >> 1) | bit << 15;

    (*state >> (16 - bits) & ((1 << bits) - 1)) as c_int
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
    let mut seed = [0; 2];
    for (i, seed) in seed.iter_mut().enumerate().take(rows) {
        *seed = data.seed;
        *seed ^= ((((row_num - i) * 37 + 178) & 0xFF) << 8) as c_uint;
        *seed ^= (((row_num - i) * 173 + 105) & 0xFF) as c_uint;
    }
    seed
}

/// # Safety
///
/// Must be called by [`generate_grain_y::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn generate_grain_y_c_erased<BD: BitDepth>(
    buf: *mut GrainLut<DynEntry>,
    data: &Dav1dFilmGrainData,
    bitdepth_max: c_int,
) {
    // SAFETY: Casting back to the original type from the `generate_grain_y::Fn::call`.
    let buf = unsafe { &mut *buf.cast() };
    let data = &data.clone().into();
    let bd = BD::from_c(bitdepth_max);
    generate_grain_y_rust(buf, data, bd)
}

const AR_PAD: usize = 3;

fn generate_grain_y_rust<BD: BitDepth>(
    buf: &mut GrainLut<BD::Entry>,
    data: &Rav1dFilmGrainData,
    bd: BD,
) {
    let bitdepth_min_8 = bd.bitdepth() - 8;
    let mut seed = data.seed;
    let shift = 4 - bitdepth_min_8 + data.grain_scale_shift;
    let grain_ctr = 128 << bitdepth_min_8;
    let grain_min = -grain_ctr;
    let grain_max = grain_ctr - 1;

    for row in &mut buf[..GRAIN_HEIGHT] {
        row[..GRAIN_WIDTH].fill_with(|| {
            let value = get_random_number(11, &mut seed);
            round2(dav1d_gaussian_sequence[value as usize], shift).as_::<BD::Entry>()
        });
    }

    // `ar_lag` is 2 bits; this tells the compiler it definitely is.
    // That also means `ar_lag <= ar_pad`.
    let ar_lag = data.ar_coeff_lag as usize & ((1 << 2) - 1);

    for y in 0..GRAIN_HEIGHT - AR_PAD {
        for x in 0..GRAIN_WIDTH - 2 * AR_PAD {
            let mut coeff = &data.ar_coeffs_y[..];
            let mut sum = 0;
            for (dy, buf_row) in buf[y..][AR_PAD - ar_lag..=AR_PAD].iter().enumerate() {
                for (dx, &buf_val) in buf_row[x..][AR_PAD - ar_lag..=AR_PAD + ar_lag]
                    .iter()
                    .enumerate()
                {
                    if dx == ar_lag && dy == ar_lag {
                        break;
                    }
                    sum += coeff[0] as c_int * buf_val.as_::<c_int>();
                    coeff = &coeff[1..];
                }
            }

            let buf_yx = &mut buf[y + AR_PAD][x + AR_PAD];
            let grain = (*buf_yx).as_::<c_int>() + round2(sum, data.ar_coeff_shift);
            *buf_yx = iclip(grain, grain_min, grain_max).as_::<BD::Entry>();
        }
    }
}

fn generate_grain_uv_rust<BD: BitDepth>(
    buf: &mut GrainLut<BD::Entry>,
    buf_y: &GrainLut<BD::Entry>,
    data: &Rav1dFilmGrainData,
    is_uv: bool,
    is_subx: bool,
    is_suby: bool,
    bd: BD,
) {
    let uv = is_uv as usize;

    struct IsSub {
        y: bool,
        x: bool,
    }

    impl IsSub {
        const fn chroma(&self) -> (usize, usize) {
            let h = if self.y {
                SUB_GRAIN_HEIGHT
            } else {
                GRAIN_HEIGHT
            };
            let w = if self.x { SUB_GRAIN_WIDTH } else { GRAIN_WIDTH };
            (h, w)
        }

        const fn len(&self) -> (usize, usize) {
            let (h, w) = self.chroma();
            (h - AR_PAD, w - 2 * AR_PAD)
        }

        const fn luma(&self, (y, x): (usize, usize)) -> (usize, usize) {
            (
                (y << self.y as usize) + AR_PAD,
                (x << self.x as usize) + AR_PAD,
            )
        }

        const fn buf_index(&self, (y, x): (usize, usize)) -> (usize, usize) {
            let (y, x) = self.luma((y, x));
            (y + self.y as usize, x + self.x as usize)
        }

        const fn max_buf_index(&self) -> (usize, usize) {
            let (y, x) = self.len();
            self.buf_index((y - 1, x - 1))
        }

        const fn check_buf_index<T, const Y: usize, const X: usize>(
            &self,
            _: &Option<[[T; X]; Y]>,
        ) {
            let (y, x) = self.max_buf_index();
            assert!(y < Y);
            assert!(x < X);
        }

        #[allow(dead_code)] // False positive; used in a `const`.
        const fn check_buf_index_all<T, const Y: usize, const X: usize>(buf: &Option<[[T; X]; Y]>) {
            Self { y: true, x: true }.check_buf_index(buf);
            Self { y: true, x: false }.check_buf_index(buf);
            Self { y: false, x: true }.check_buf_index(buf);
            Self { y: false, x: false }.check_buf_index(buf);
        }
    }

    let is_sub = IsSub {
        y: is_suby,
        x: is_subx,
    };

    let bitdepth_min_8 = bd.bitdepth() - 8;
    let mut seed = data.seed ^ if is_uv { 0x49d8 } else { 0xb524 };
    let shift = 4 - bitdepth_min_8 + data.grain_scale_shift;
    let grain_ctr = 128 << bitdepth_min_8;
    let grain_min = -grain_ctr;
    let grain_max = grain_ctr - 1;

    for row in &mut buf[..is_sub.chroma().0] {
        row[..is_sub.chroma().1].fill_with(|| {
            let value = get_random_number(11, &mut seed);
            round2(dav1d_gaussian_sequence[value as usize], shift).as_::<BD::Entry>()
        });
    }

    // `ar_lag` is 2 bits; this tells the compiler it definitely is.
    // That also means `ar_lag <= ar_pad`.
    let ar_lag = data.ar_coeff_lag as usize & ((1 << 2) - 1);

    for y in 0..is_sub.len().0 {
        for x in 0..is_sub.len().1 {
            let mut coeff = &data.ar_coeffs_uv[uv][..];
            let mut sum = 0;
            for (dy, buf_row) in buf[y..][AR_PAD - ar_lag..=AR_PAD].iter().enumerate() {
                for (dx, &buf_val) in buf_row[x..][AR_PAD - ar_lag..=AR_PAD + ar_lag]
                    .iter()
                    .enumerate()
                {
                    if dx == ar_lag && dy == ar_lag {
                        let mut luma = 0;
                        let (luma_y, luma_x) = is_sub.luma((y, x));
                        const _: () = IsSub::check_buf_index_all(&None::<GrainLut<()>>);
                        // The optimizer is not smart enough to deduce this on its own.
                        // SAFETY: The above static check checks all maximum index possibilities.
                        unsafe {
                            assert_unchecked(luma_y < GRAIN_HEIGHT + 1 - 1);
                            assert_unchecked(luma_x < GRAIN_WIDTH - 1);
                        }
                        for i in 0..1 + is_sub.y as usize {
                            for j in 0..1 + is_sub.x as usize {
                                luma += buf_y[luma_y + i][luma_x + j].as_::<c_int>();
                            }
                        }
                        luma = round2(luma, is_sub.y as u8 + is_sub.x as u8);

                        sum += luma * coeff[0] as c_int;
                        break;
                    }
                    sum += coeff[0] as c_int * buf_val.as_::<c_int>();
                    coeff = &coeff[1..];
                }
            }

            let buf_yx = &mut buf[y + AR_PAD][x + AR_PAD];
            let grain = (*buf_yx).as_::<c_int>() + round2(sum, data.ar_coeff_shift);
            *buf_yx = iclip(grain, grain_min, grain_max).as_::<BD::Entry>();
        }
    }
}

/// # Safety
///
/// Must be called by [`generate_grain_uv::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
#[inline(never)]
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
    // SAFETY: Casting back to the original type from the `generate_grain_uv::Fn::call`.
    let buf = unsafe { &mut *buf.cast() };
    // SAFETY: Casting back to the original type from the `generate_grain_uv::Fn::call`.
    let buf_y = unsafe { &*buf_y.cast() };
    let data = &data.clone().into();
    let is_uv = uv != 0;
    let bd = BD::from_c(bitdepth_max);
    generate_grain_uv_rust(buf, buf_y, data, is_uv, IS_SUBX, IS_SUBY, bd)
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
    grain_lut[offy + y + (FG_BLOCK_SIZE >> suby) * by][offx + x + (FG_BLOCK_SIZE >> subx) * bx]
        .as_::<i32>()
}

/// # Safety
///
/// Must be called by [`fgy_32x32xn::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn fgy_32x32xn_c_erased<BD: BitDepth>(
    _dst_row_ptr: *mut DynPixel,
    _src_row_ptr: *const DynPixel,
    _stride: ptrdiff_t,
    data: &Dav1dFilmGrainData,
    pw: usize,
    scaling: *const DynScaling,
    grain_lut: *const GrainLut<DynEntry>,
    bh: c_int,
    row_num: c_int,
    bitdepth_max: c_int,
    dst_row: *const FFISafe<Rav1dPictureDataComponentOffset>,
    src_row: *const FFISafe<Rav1dPictureDataComponentOffset>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `fgy_32x32xn::Fn::call`.
    let [dst_row, src_row] = [dst_row, src_row].map(|it| *unsafe { FFISafe::get(it) });
    let data = &data.clone().into();
    // SAFETY: Casting back to the original type from the `fn` ptr call.
    let scaling = unsafe { &*scaling.cast() };
    // SAFETY: Casting back to the original type from the `fn` ptr call.
    let grain_lut = unsafe { &*grain_lut.cast() };
    let bh = bh as usize;
    let row_num = row_num as usize;
    let bd = BD::from_c(bitdepth_max);
    fgy_32x32xn_rust(
        dst_row, src_row, data, pw, scaling, grain_lut, bh, row_num, bd,
    )
}

fn fgy_32x32xn_rust<BD: BitDepth>(
    dst_row: Rav1dPictureDataComponentOffset,
    src_row: Rav1dPictureDataComponentOffset,
    data: &Rav1dFilmGrainData,
    pw: usize,
    scaling: &BD::Scaling,
    grain_lut: &GrainLut<BD::Entry>,
    bh: usize,
    row_num: usize,
    bd: BD,
) {
    let rows = 1 + (data.overlap_flag && row_num > 0) as usize;
    let bitdepth_min_8 = bd.bitdepth() - 8;
    let grain_ctr = 128 << bitdepth_min_8;
    let grain_min = -grain_ctr;
    let grain_max = grain_ctr - 1;

    let min_value;
    let max_value;
    if data.clip_to_restricted_range {
        min_value = 16 << bitdepth_min_8;
        max_value = 235 << bitdepth_min_8;
    } else {
        min_value = 0;
        max_value = bd.bitdepth_max().as_::<c_int>();
    }

    let mut seed = row_seed(rows, row_num, data);

    assert!(dst_row.stride() % (FG_BLOCK_SIZE * mem::size_of::<BD::Pixel>()) as isize == 0);

    let mut offsets: [[c_int; 2]; 2] = [[0; 2 /* row offset */]; 2 /* col offset */];

    // process this row in FG_BLOCK_SIZE^2 blocks
    for bx in (0..pw).step_by(FG_BLOCK_SIZE) {
        let bw = cmp::min(FG_BLOCK_SIZE, pw - bx);

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

        static W: [[c_int; 2]; 2] = [[27, 17], [17, 27]];

        let src_row_y = |y| {
            let row = src_row + y as isize * src_row.pixel_stride::<BD>();
            (row + bx).slice::<BD>(bw)
        };
        let dst_row_y = |y| {
            let row = dst_row + y as isize * dst_row.pixel_stride::<BD>();
            (row + bx).slice_mut::<BD>(bw)
        };

        let noise_y = |src: BD::Pixel, grain| {
            let noise = round2(
                scaling.as_ref()[src.to::<usize>()] as c_int * grain,
                data.scaling_shift,
            );
            iclip(src.as_::<c_int>() + noise, min_value, max_value).as_::<BD::Pixel>()
        };

        for y in ystart..bh {
            let src = &*src_row_y(y);
            let dst = &mut *dst_row_y(y);

            // Non-overlapped image region (straightforward)
            for x in xstart..bw {
                let grain = sample_lut::<BD>(grain_lut, &offsets, false, false, false, false, x, y);
                dst[x] = noise_y(src[x], grain);
            }

            // Special case for overlapped column
            for x in 0..xstart {
                let grain = sample_lut::<BD>(grain_lut, &offsets, false, false, false, false, x, y);
                let old = sample_lut::<BD>(grain_lut, &offsets, false, false, true, false, x, y);
                let grain = round2(old * W[x][0] + grain * W[x][1], 5);
                let grain = iclip(grain, grain_min, grain_max);
                dst[x] = noise_y(src[x], grain);
            }
        }
        for y in 0..ystart {
            let src = &*src_row_y(y);
            let dst = &mut *dst_row_y(y);

            // Special case for overlapped row (sans corner)
            for x in xstart..bw {
                let grain = sample_lut::<BD>(grain_lut, &offsets, false, false, false, false, x, y);
                let old = sample_lut::<BD>(grain_lut, &offsets, false, false, false, true, x, y);
                let grain = round2(old * W[y][0] + grain * W[y][1], 5);
                let grain = iclip(grain, grain_min, grain_max);
                dst[x] = noise_y(src[x], grain);
            }

            // Special case for doubly-overlapped corner
            for x in 0..xstart {
                // Blend the top pixel with the top left block
                let top = sample_lut::<BD>(grain_lut, &offsets, false, false, false, true, x, y);
                let old = sample_lut::<BD>(grain_lut, &offsets, false, false, true, true, x, y);
                let top = round2(old * W[x][0] + top * W[x][1], 5);
                let top = iclip(top, grain_min, grain_max);

                // Blend the current pixel with the left block
                let grain = sample_lut::<BD>(grain_lut, &offsets, false, false, false, false, x, y);
                let old = sample_lut::<BD>(grain_lut, &offsets, false, false, true, false, x, y);

                // Mix the row rows together and apply grain
                let grain = round2(old * W[x][0] + grain * W[x][1], 5);
                let grain = iclip(grain, grain_min, grain_max);
                let grain = round2(top * W[y][0] + grain * W[y][1], 5);
                let grain = iclip(grain, grain_min, grain_max);
                dst[x] = noise_y(src[x], grain);
            }
        }
    }
}

fn fguv_32x32xn_rust<BD: BitDepth>(
    dst_row: Rav1dPictureDataComponentOffset,
    src_row: Rav1dPictureDataComponentOffset,
    data: &Rav1dFilmGrainData,
    pw: usize,
    scaling: &BD::Scaling,
    grain_lut: &GrainLut<BD::Entry>,
    bh: usize,
    row_num: usize,
    luma_row: Rav1dPictureDataComponentOffset,
    is_uv: bool,
    is_id: bool,
    is_sx: bool,
    is_sy: bool,
    bd: BD,
) {
    let [uv, sx, sy] = [is_uv, is_sx, is_sy].map(|it| it as usize);

    let rows = 1 + (data.overlap_flag && row_num > 0) as usize;
    let bitdepth_min_8 = bd.bitdepth() - 8;
    let grain_ctr = 128 << bitdepth_min_8;
    let grain_min = -grain_ctr;
    let grain_max = grain_ctr - 1;

    let min_value;
    let max_value;
    if data.clip_to_restricted_range {
        min_value = 16 << bitdepth_min_8;
        max_value = (if is_id { 235 } else { 240 }) << bitdepth_min_8;
    } else {
        min_value = 0;
        max_value = bd.bitdepth_max().as_::<c_int>();
    }

    let mut seed = row_seed(rows, row_num, data);

    assert!(dst_row.stride() % (FG_BLOCK_SIZE * mem::size_of::<BD::Pixel>()) as isize == 0);

    let mut offsets: [[c_int; 2]; 2] = [[0; 2 /* row offset */]; 2 /* col offset */];

    // process this row in FG_BLOCK_SIZE^2 blocks (subsampled)
    for bx in (0..pw).step_by(FG_BLOCK_SIZE >> sx) {
        let bw = cmp::min(FG_BLOCK_SIZE >> sx, pw - bx);
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

        static W: [[[c_int; 2]; 2 /* off */]; 2 /* sub */] = [[[27, 17], [17, 27]], [[23, 22], [0; 2]]];

        let luma_row_uv = |y| {
            let row = luma_row + (y << sy) as isize * luma_row.pixel_stride::<BD>();
            (row + (bx << sx)).slice::<BD>(bw << sx)
        };
        let src_row_uv = |y| {
            let row = src_row + y as isize * src_row.pixel_stride::<BD>();
            (row + bx).slice::<BD>(bw)
        };
        let dst_row_uv = |y| {
            let row = dst_row + y as isize * dst_row.pixel_stride::<BD>();
            (row + bx).slice_mut::<BD>(bw)
        };

        let noise_uv = |src: BD::Pixel, grain, luma: &[BD::Pixel]| {
            let mut avg = luma[0];
            if is_sx {
                avg = ((avg.as_::<c_int>() + luma[1].as_::<c_int>() + 1) >> 1).as_::<BD::Pixel>();
            }
            let mut val = avg.as_::<c_int>();
            if !data.chroma_scaling_from_luma {
                let combined = avg.as_::<c_int>() * data.uv_luma_mult[uv]
                    + src.as_::<c_int>() * data.uv_mult[uv];
                val = bd
                    .iclip_pixel((combined >> 6) + data.uv_offset[uv] * (1 << bitdepth_min_8))
                    .as_::<c_int>();
            }
            // `val` isn't out of bounds, so we can
            // eliminate extra panicking code by bit-truncating `val`.
            let noise = round2(
                scaling.as_ref()[val as usize % scaling.as_ref().len()] as c_int * grain,
                data.scaling_shift,
            );
            iclip(src.as_::<c_int>() + noise, min_value, max_value).as_::<BD::Pixel>()
        };

        for y in ystart..bh {
            let luma = &*luma_row_uv(y);
            let src = &*src_row_uv(y);
            let dst = &mut *dst_row_uv(y);

            // Non-overlapped image region (straightforward)
            for x in xstart..bw {
                let grain = sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, false, false, x, y);
                dst[x] = noise_uv(src[x], grain, &luma[x << sx..]);
            }

            // Special case for overlapped column
            for x in 0..xstart {
                let grain = sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, false, false, x, y);
                let old = sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, true, false, x, y);
                let grain = round2(old * W[sx][x][0] + grain * W[sx][x][1], 5);
                let grain = iclip(grain, grain_min, grain_max);
                dst[x] = noise_uv(src[x], grain, &luma[x << sx..]);
            }
        }
        for y in 0..ystart {
            let luma = &*luma_row_uv(y);
            let src = &*src_row_uv(y);
            let dst = &mut *dst_row_uv(y);

            // Special case for overlapped row (sans corner)
            for x in xstart..bw {
                let grain = sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, false, false, x, y);
                let old = sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, false, true, x, y);
                let grain = round2(old * W[sy][y][0] + grain * W[sy][y][1], 5);
                let grain = iclip(grain, grain_min, grain_max);
                dst[x] = noise_uv(src[x], grain, &luma[x << sx..]);
            }

            // Special case for doubly-overlapped corner
            for x in 0..xstart {
                // Blend the top pixel with the top left block
                let top = sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, false, true, x, y);
                let old = sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, true, true, x, y);
                let top = round2(old * W[sx][x][0] + top * W[sx][x][1], 5);
                let top = iclip(top, grain_min, grain_max);

                // Blend the current pixel with the left block
                let grain = sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, false, false, x, y);
                let old = sample_lut::<BD>(grain_lut, &offsets, is_sx, is_sy, true, false, x, y);

                // Mix the row rows together and apply to image
                let grain = round2(old * W[sx][x][0] + grain * W[sx][x][1], 5);
                let grain = iclip(grain, grain_min, grain_max);
                let grain = round2(top * W[sy][y][0] + grain * W[sy][y][1], 5);
                let grain = iclip(grain, grain_min, grain_max);
                dst[x] = noise_uv(src[x], grain, &luma[x << sx..]);
            }
        }
    }
}

/// # Safety
///
/// Must be called by [`fguv_32x32xn::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
#[inline(never)]
unsafe extern "C" fn fguv_32x32xn_c_erased<
    BD: BitDepth,
    const NM: usize,
    const IS_SX: bool,
    const IS_SY: bool,
>(
    _dst_row_ptr: *mut DynPixel,
    _src_row_ptr: *const DynPixel,
    _stride: ptrdiff_t,
    data: &Dav1dFilmGrainData,
    pw: usize,
    scaling: *const DynScaling,
    grain_lut: *const GrainLut<DynEntry>,
    bh: c_int,
    row_num: c_int,
    _luma_row_ptr: *const DynPixel,
    _luma_stride: ptrdiff_t,
    uv_pl: c_int,
    is_id: c_int,
    bitdepth_max: c_int,
    dst_row: *const FFISafe<Rav1dPictureDataComponentOffset>,
    src_row: *const FFISafe<Rav1dPictureDataComponentOffset>,
    luma_row: *const FFISafe<Rav1dPictureDataComponentOffset>,
) {
    let [dst_row, src_row, luma_row] = [dst_row, src_row, luma_row].map(|row| {
        // SAFETY: Was passed as `FFISafe::new(_)` in `fguv_32x32xn::Fn::call`.
        *unsafe { FFISafe::get(row) }
    });
    let data = &data.clone().into();
    // SAFETY: Casting back to the original type from the `fn` ptr call.
    let scaling = unsafe { &*scaling.cast() };
    // SAFETY: Casting back to the original type from the `fn` ptr call.
    let grain_lut = unsafe { &*grain_lut.cast() };
    let bh = bh as usize;
    let row_num = row_num as usize;
    let uv_pl = uv_pl as usize;
    let is_id = is_id != 0;
    let bd = BD::from_c(bitdepth_max);
    fguv_32x32xn_rust(
        dst_row,
        src_row,
        data,
        pw,
        scaling,
        grain_lut,
        bh,
        row_num,
        luma_row,
        uv_pl != 0,
        is_id,
        IS_SX,
        IS_SY,
        bd,
    )
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
mod neon {
    use super::*;

    wrap_fn_ptr!(unsafe extern "C" fn fgy_32x32xn_neon(
        dst: *mut DynPixel,
        src: *const DynPixel,
        stride: ptrdiff_t,
        scaling: *const DynScaling,
        scaling_shift: c_int,
        grain_lut: *const GrainLut<DynEntry>,
        offsets: &[[c_int; 2]; 2],
        h: c_int,
        clip: ptrdiff_t,
        r#type: ptrdiff_t,
        bitdepth_max: c_int,
    ) -> ());

    impl fgy_32x32xn_neon::Fn {
        /// Use [`ptrdiff_t`] instead of [`c_int`] for the last few parameters,
        /// to get the same layout of parameters on the stack across platforms.
        fn call<BD: BitDepth>(
            &self,
            dst: *mut BD::Pixel,
            src: *const BD::Pixel,
            stride: isize,
            scaling: *const BD::Scaling,
            scaling_shift: u8,
            grain_lut: *const GrainLut<BD::Entry>,
            offsets: &[[c_int; 2]; 2],
            h: c_int,
            clip: bool,
            r#type: c_int,
            bd: BD,
        ) {
            let dst = dst.cast();
            let src = src.cast();
            let scaling = scaling.cast();
            let scaling_shift = scaling_shift as c_int;
            let grain_lut = grain_lut.cast();
            let clip = clip as ptrdiff_t;
            let r#type = r#type as ptrdiff_t;
            let bd = bd.into_c();
            // SAFETY: asm should be safe.
            unsafe {
                self.get()(
                    dst,
                    src,
                    stride,
                    scaling,
                    scaling_shift,
                    grain_lut,
                    offsets,
                    h,
                    clip,
                    r#type,
                    bd,
                )
            }
        }
    }

    /// # Safety
    ///
    /// Must be called by [`fgy_32x32xn::Fn::call`].
    #[deny(unsafe_op_in_unsafe_fn)]
    pub unsafe extern "C" fn fgy_32x32xn_neon_erased<BD: BitDepth>(
        dst_row_ptr: *mut DynPixel,
        src_row_ptr: *const DynPixel,
        stride: ptrdiff_t,
        data: &Dav1dFilmGrainData,
        pw: usize,
        scaling: *const DynScaling,
        grain_lut: *const GrainLut<DynEntry>,
        bh: c_int,
        row_num: c_int,
        bitdepth_max: c_int,
        _dst_row: *const FFISafe<Rav1dPictureDataComponentOffset>,
        _src_row: *const FFISafe<Rav1dPictureDataComponentOffset>,
    ) {
        let dst_row = dst_row_ptr.cast();
        let src_row = src_row_ptr.cast();
        let data = &data.clone().into();
        let scaling = scaling.cast();
        let grain_lut = grain_lut.cast();
        let row_num = row_num as usize;
        let bd = BD::from_c(bitdepth_max);
        fgy_32x32xn_neon(
            dst_row, src_row, stride, data, pw, scaling, grain_lut, bh, row_num, bd,
        )
    }

    fn fgy_32x32xn_neon<BD: BitDepth>(
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

        // process this row in FG_BLOCK_SIZE^2 blocks
        for bx in (0..pw).step_by(FG_BLOCK_SIZE) {
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
                r#type |= 1; // overlap y
            }
            if data.overlap_flag && bx != 0 {
                r#type |= 2; // overlap x
            }

            bd_fn!(fgy_32x32xn_neon::decl_fn, BD, fgy_32x32, neon).call(
                dst_row.wrapping_add(bx),
                src_row.wrapping_add(bx),
                stride,
                scaling,
                data.scaling_shift,
                grain_lut,
                &offsets,
                bh,
                data.clip_to_restricted_range,
                r#type,
                bd,
            )
        }
    }

    wrap_fn_ptr!(unsafe extern "C" fn fguv_32x32xn_neon(
        dst: *mut DynPixel,
        src: *const DynPixel,
        stride: ptrdiff_t,
        scaling: *const DynScaling,
        data: &Dav1dFilmGrainData,
        grain_lut: *const GrainLut<DynEntry>,
        luma_row: *const DynPixel,
        luma_stride: ptrdiff_t,
        offsets: &[[c_int; 2]; 2],
        h: ptrdiff_t,
        uv: ptrdiff_t,
        is_id: ptrdiff_t,
        r#type: ptrdiff_t,
        bitdepth_max: c_int,
    ) -> ());

    impl fguv_32x32xn_neon::Fn {
        /// Use [`ptrdiff_t`] instead of [`c_int`] for the last few parameters,
        /// to get the parameters on the stack with the same layout across platforms.
        fn call<BD: BitDepth>(
            &self,
            dst: *mut BD::Pixel,
            src: *const BD::Pixel,
            stride: ptrdiff_t,
            scaling: *const BD::Scaling,
            data: &Dav1dFilmGrainData,
            grain_lut: *const GrainLut<BD::Entry>,
            luma_row: *const BD::Pixel,
            luma_stride: ptrdiff_t,
            offsets: &[[c_int; 2]; 2],
            h: c_int,
            uv: c_int,
            is_id: c_int,
            r#type: c_int,
            bd: BD,
        ) {
            let dst = dst.cast();
            let src = src.cast();
            let scaling = scaling.cast();
            let grain_lut = grain_lut.cast();
            let luma_row = luma_row.cast();
            let h = h as ptrdiff_t;
            let uv = uv as ptrdiff_t;
            let is_id = is_id as ptrdiff_t;
            let r#type = r#type as ptrdiff_t;
            let bd = bd.into_c();
            // SAFETY: asm should be safe.
            unsafe {
                self.get()(
                    dst,
                    src,
                    stride,
                    scaling,
                    data,
                    grain_lut,
                    luma_row,
                    luma_stride,
                    offsets,
                    h,
                    uv,
                    is_id,
                    r#type,
                    bd,
                )
            }
        }
    }

    /// # Safety
    ///
    /// Must be called by [`fguv_32x32xn::Fn::call`].
    #[deny(unsafe_op_in_unsafe_fn)]
    pub unsafe extern "C" fn fguv_32x32xn_neon_erased<
        BD: BitDepth,
        const NM: usize,
        const IS_SX: bool,
        const IS_SY: bool,
    >(
        dst_row_ptr: *mut DynPixel,
        src_row_ptr: *const DynPixel,
        stride: ptrdiff_t,
        data: &Dav1dFilmGrainData,
        pw: usize,
        scaling: *const DynScaling,
        grain_lut: *const GrainLut<DynEntry>,
        bh: c_int,
        row_num: c_int,
        luma_row_ptr: *const DynPixel,
        luma_stride: ptrdiff_t,
        uv: c_int,
        is_id: c_int,
        bitdepth_max: c_int,
        _dst_row: *const FFISafe<Rav1dPictureDataComponentOffset>,
        _src_row: *const FFISafe<Rav1dPictureDataComponentOffset>,
        _luma_row: *const FFISafe<Rav1dPictureDataComponentOffset>,
    ) {
        let dst_row = dst_row_ptr.cast();
        let src_row = src_row_ptr.cast();
        let data_c = data;
        let data = &data.clone().into();
        let scaling = scaling.cast();
        let grain_lut = grain_lut.cast();
        let row_num = row_num as usize;
        let luma_row = luma_row_ptr.cast();
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

    fn fguv_32x32xn_neon<BD: BitDepth, const NM: usize, const IS_SX: bool, const IS_SY: bool>(
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
        let [sx, _sy] = [IS_SX, IS_SY].map(|it| it as usize);

        let rows = 1 + (data.overlap_flag && row_num > 0) as usize;

        let mut seed = row_seed(rows, row_num, data);

        let mut offsets: [[c_int; 2]; 2] = [[0; 2 /* row offset */]; 2 /* col offset */];

        // process this row in FG_BLOCK_SIZE^2 blocks (subsampled)
        for bx in (0..pw).step_by(FG_BLOCK_SIZE >> sx) {
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
                r#type |= 1; // overlap y
            }
            if data.overlap_flag && bx != 0 {
                r#type |= 2; // overlap x
            }
            if data.chroma_scaling_from_luma {
                r#type |= 4;
            }
            (match NM {
                420 => bd_fn!(fguv_32x32xn_neon::decl_fn, BD, fguv_32x32_420, neon),
                422 => bd_fn!(fguv_32x32xn_neon::decl_fn, BD, fguv_32x32_422, neon),
                444 => bd_fn!(fguv_32x32xn_neon::decl_fn, BD, fguv_32x32_444, neon),
                _ => unreachable!(),
            })
            .call(
                dst_row.wrapping_add(bx),
                src_row.wrapping_add(bx),
                stride,
                scaling,
                data_c,
                grain_lut,
                luma_row.wrapping_add(bx << sx),
                luma_stride,
                &offsets,
                bh,
                uv,
                is_id,
                r#type,
                bd,
            )
        }
    }
}

impl Rav1dFilmGrainDSPContext {
    pub const fn default<BD: BitDepth>() -> Self {
        Self {
            generate_grain_y: generate_grain_y::Fn::new(generate_grain_y_c_erased::<BD>),
            generate_grain_uv: enum_map!(Rav1dPixelLayoutSubSampled => generate_grain_uv::Fn; match key {
                I420 => generate_grain_uv::Fn::new(generate_grain_uv_c_erased::<BD, 420, true, true>),
                I422 => generate_grain_uv::Fn::new(generate_grain_uv_c_erased::<BD, 422, true, false>),
                I444 => generate_grain_uv::Fn::new(generate_grain_uv_c_erased::<BD, 444, false, false>),
            }),
            fgy_32x32xn: fgy_32x32xn::Fn::new(fgy_32x32xn_c_erased::<BD>),
            fguv_32x32xn: enum_map!(Rav1dPixelLayoutSubSampled => fguv_32x32xn::Fn; match key {
                I420 => fguv_32x32xn::Fn::new(fguv_32x32xn_c_erased::<BD, 420, true, true>),
                I422 => fguv_32x32xn::Fn::new(fguv_32x32xn_c_erased::<BD, 422, true, false>),
                I444 => fguv_32x32xn::Fn::new(fguv_32x32xn_c_erased::<BD, 444, false, false>),
            }),
        }
    }

    #[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
    #[inline(always)]
    const fn init_x86<BD: BitDepth>(mut self, flags: CpuFlags) -> Self {
        if !flags.contains(CpuFlags::SSSE3) {
            return self;
        }

        self.generate_grain_y = bd_fn!(generate_grain_y::decl_fn, BD, generate_grain_y, ssse3);
        self.generate_grain_uv = enum_map!(Rav1dPixelLayoutSubSampled => generate_grain_uv::Fn; match key {
            I420 => bd_fn!(generate_grain_uv::decl_fn, BD, generate_grain_uv_420, ssse3),
            I422 => bd_fn!(generate_grain_uv::decl_fn, BD, generate_grain_uv_422, ssse3),
            I444 => bd_fn!(generate_grain_uv::decl_fn, BD, generate_grain_uv_444, ssse3),
        });

        self.fgy_32x32xn = bd_fn!(fgy_32x32xn::decl_fn, BD, fgy_32x32xn, ssse3);
        self.fguv_32x32xn = enum_map!(Rav1dPixelLayoutSubSampled => fguv_32x32xn::Fn; match key {
            I420 => bd_fn!(fguv_32x32xn::decl_fn, BD, fguv_32x32xn_i420, ssse3),
            I422 => bd_fn!(fguv_32x32xn::decl_fn, BD, fguv_32x32xn_i422, ssse3),
            I444 => bd_fn!(fguv_32x32xn::decl_fn, BD, fguv_32x32xn_i444, ssse3),
        });

        #[cfg(target_arch = "x86_64")]
        {
            if !flags.contains(CpuFlags::AVX2) {
                return self;
            }

            self.generate_grain_y = bd_fn!(generate_grain_y::decl_fn, BD, generate_grain_y, avx2);
            self.generate_grain_uv = enum_map!(Rav1dPixelLayoutSubSampled => generate_grain_uv::Fn; match key {
                I420 => bd_fn!(generate_grain_uv::decl_fn, BD, generate_grain_uv_420, avx2),
                I422 => bd_fn!(generate_grain_uv::decl_fn, BD, generate_grain_uv_422, avx2),
                I444 => bd_fn!(generate_grain_uv::decl_fn, BD, generate_grain_uv_444, avx2),
            });

            if !flags.contains(CpuFlags::SLOW_GATHER) {
                self.fgy_32x32xn = bd_fn!(fgy_32x32xn::decl_fn, BD, fgy_32x32xn, avx2);
                self.fguv_32x32xn = enum_map!(Rav1dPixelLayoutSubSampled => fguv_32x32xn::Fn; match key {
                    I420 => bd_fn!(fguv_32x32xn::decl_fn, BD, fguv_32x32xn_i420, avx2),
                    I422 => bd_fn!(fguv_32x32xn::decl_fn, BD, fguv_32x32xn_i422, avx2),
                    I444 => bd_fn!(fguv_32x32xn::decl_fn, BD, fguv_32x32xn_i444, avx2),
                });
            }

            if !flags.contains(CpuFlags::AVX512ICL) {
                return self;
            }

            if BD::BITDEPTH == 8 || !flags.contains(CpuFlags::SLOW_GATHER) {
                self.fgy_32x32xn = bd_fn!(fgy_32x32xn::decl_fn, BD, fgy_32x32xn, avx512icl);
                self.fguv_32x32xn = enum_map!(Rav1dPixelLayoutSubSampled => fguv_32x32xn::Fn; match key {
                    I420 => bd_fn!(fguv_32x32xn::decl_fn, BD, fguv_32x32xn_i420, avx512icl),
                    I422 => bd_fn!(fguv_32x32xn::decl_fn, BD, fguv_32x32xn_i422, avx512icl),
                    I444 => bd_fn!(fguv_32x32xn::decl_fn, BD, fguv_32x32xn_i444, avx512icl),
                });
            }
        }

        self
    }

    #[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
    #[inline(always)]
    const fn init_arm<BD: BitDepth>(mut self, flags: CpuFlags) -> Self {
        if !flags.contains(CpuFlags::NEON) {
            return self;
        }

        self.generate_grain_y = bd_fn!(generate_grain_y::decl_fn, BD, generate_grain_y, neon);
        self.generate_grain_uv = enum_map!(Rav1dPixelLayoutSubSampled => generate_grain_uv::Fn; match key {
            I420 => bd_fn!(generate_grain_uv::decl_fn, BD, generate_grain_uv_420, neon),
            I422 => bd_fn!(generate_grain_uv::decl_fn, BD, generate_grain_uv_422, neon),
            I444 => bd_fn!(generate_grain_uv::decl_fn, BD, generate_grain_uv_444, neon),
        });

        self.fgy_32x32xn = fgy_32x32xn::Fn::new(neon::fgy_32x32xn_neon_erased::<BD>);
        self.fguv_32x32xn = enum_map!(Rav1dPixelLayoutSubSampled => fguv_32x32xn::Fn; match key {
            I420 => fguv_32x32xn::Fn::new(neon::fguv_32x32xn_neon_erased::<BD, 420, true, true>),
            I422 => fguv_32x32xn::Fn::new(neon::fguv_32x32xn_neon_erased::<BD, 422, true, false>),
            I444 => fguv_32x32xn::Fn::new(neon::fguv_32x32xn_neon_erased::<BD, 444, false, false>),
        });

        self
    }

    #[inline(always)]
    const fn init<BD: BitDepth>(self, flags: CpuFlags) -> Self {
        #[cfg(feature = "asm")]
        {
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
        {
            let _ = flags;
            self
        }
    }

    pub const fn new<BD: BitDepth>(flags: CpuFlags) -> Self {
        Self::default::<BD>().init::<BD>(flags)
    }
}
