use crate::include::common::bitdepth::DynEntry;
use crate::include::common::bitdepth::DynPixel;
use crate::include::dav1d::headers::Dav1dFilmGrainData;
use libc::intptr_t;
use libc::ptrdiff_t;
use std::ffi::c_int;
use std::ffi::c_uint;

#[inline]
pub unsafe fn get_random_number(bits: c_int, state: *mut c_uint) -> c_int {
    let r = *state as c_int;
    let bit: c_uint = ((r >> 0 ^ r >> 1 ^ r >> 3 ^ r >> 12) & 1) as c_uint;
    *state = (r >> 1) as c_uint | bit << 15;
    return (*state >> 16 - bits & (((1 as c_int) << bits) - 1) as c_uint) as c_int;
}

#[inline]
pub unsafe fn round2(x: c_int, shift: u64) -> c_int {
    return x + ((1 as c_int) << shift >> 1) >> shift;
}

pub const GRAIN_WIDTH: usize = 82;
pub const GRAIN_HEIGHT: usize = 73;

pub type generate_grain_y_fn = Option<
    unsafe extern "C" fn(*mut [DynEntry; GRAIN_WIDTH], *const Dav1dFilmGrainData, c_int) -> (),
>;

pub type generate_grain_uv_fn = Option<
    unsafe extern "C" fn(
        *mut [DynEntry; GRAIN_WIDTH],
        *const [DynEntry; GRAIN_WIDTH],
        *const Dav1dFilmGrainData,
        intptr_t,
        c_int,
    ) -> (),
>;

pub type fgy_32x32xn_fn = Option<
    unsafe extern "C" fn(
        *mut DynPixel,
        *const DynPixel,
        ptrdiff_t,
        *const Dav1dFilmGrainData,
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
        *const Dav1dFilmGrainData,
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
