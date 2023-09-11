use crate::include::common::bitdepth::DynEntry;
use crate::include::common::bitdepth::DynPixel;
use crate::include::dav1d::headers::Dav1dFilmGrainData;
use crate::include::stddef::ptrdiff_t;
use crate::include::stddef::size_t;
use crate::include::stdint::intptr_t;
use crate::include::stdint::uint64_t;
use crate::include::stdint::uint8_t;

#[inline]
pub unsafe extern "C" fn get_random_number(
    bits: libc::c_int,
    state: *mut libc::c_uint,
) -> libc::c_int {
    let r = *state as libc::c_int;
    let bit: libc::c_uint = ((r >> 0 ^ r >> 1 ^ r >> 3 ^ r >> 12) & 1) as libc::c_uint;
    *state = (r >> 1) as libc::c_uint | bit << 15;
    return (*state >> 16 - bits & (((1 as libc::c_int) << bits) - 1) as libc::c_uint)
        as libc::c_int;
}

#[inline]
pub unsafe extern "C" fn round2(x: libc::c_int, shift: uint64_t) -> libc::c_int {
    return x + ((1 as libc::c_int) << shift >> 1) >> shift;
}

pub type EntryRow = [DynEntry; 82]; // [entry; 82]
pub type generate_grain_y_fn =
    Option<unsafe extern "C" fn(*mut EntryRow, *const Dav1dFilmGrainData, libc::c_int) -> ()>;
pub type generate_grain_uv_fn = Option<
    unsafe extern "C" fn(
        *mut EntryRow,
        *const EntryRow,
        *const Dav1dFilmGrainData,
        intptr_t,
        libc::c_int,
    ) -> (),
>;
pub type fgy_32x32xn_fn = Option<
    unsafe extern "C" fn(
        *mut DynPixel,
        *const DynPixel,
        ptrdiff_t,
        *const Dav1dFilmGrainData,
        size_t,
        *const uint8_t,
        *const EntryRow,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type fguv_32x32xn_fn = Option<
    unsafe extern "C" fn(
        *mut DynPixel,
        *const DynPixel,
        ptrdiff_t,
        *const Dav1dFilmGrainData,
        size_t,
        *const uint8_t,
        *const EntryRow,
        libc::c_int,
        libc::c_int,
        *const DynPixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
#[repr(C)]
pub struct Dav1dFilmGrainDSPContext {
    pub generate_grain_y: generate_grain_y_fn,
    pub generate_grain_uv: [generate_grain_uv_fn; 3],
    pub fgy_32x32xn: fgy_32x32xn_fn,
    pub fguv_32x32xn: [fguv_32x32xn_fn; 3],
}
