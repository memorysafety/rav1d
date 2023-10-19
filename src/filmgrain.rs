use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynEntry;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::intops::iclip;
use crate::include::dav1d::headers::Rav1dFilmGrainData;
use crate::src::tables::dav1d_gaussian_sequence;
use libc::intptr_t;
use libc::ptrdiff_t;
use std::ffi::c_int;
use std::ffi::c_uint;

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

// TODO(kkysen) temporarily pub until mod is deduplicated
#[inline]
pub(crate) unsafe fn get_random_number(bits: c_int, state: *mut c_uint) -> c_int {
    let r = *state as c_int;
    let bit: c_uint = ((r >> 0 ^ r >> 1 ^ r >> 3 ^ r >> 12) & 1) as c_uint;
    *state = (r >> 1) as c_uint | bit << 15;
    return (*state >> 16 - bits & (((1 as c_int) << bits) - 1) as c_uint) as c_int;
}

// TODO(kkysen) temporarily pub until mod is deduplicated
#[inline]
pub(crate) unsafe fn round2(x: c_int, shift: u64) -> c_int {
    return x + ((1 as c_int) << shift >> 1) >> shift;
}

// TODO(kkysen) temporarily pub until mod is deduplicated
pub(crate) unsafe extern "C" fn generate_grain_y_c_erased<BD: BitDepth>(
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

// TODO(kkysen) temporarily pub until mod is deduplicated
pub(crate) unsafe extern "C" fn generate_grain_uv_420_c_erased<BD: BitDepth>(
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

// TODO(kkysen) temporarily pub until mod is deduplicated
pub(crate) unsafe extern "C" fn generate_grain_uv_422_c_erased<BD: BitDepth>(
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

// TODO(kkysen) temporarily pub until mod is deduplicated
pub(crate) unsafe extern "C" fn generate_grain_uv_444_c_erased<BD: BitDepth>(
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
