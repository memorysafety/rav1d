use crate::include::common::intops::apply_sign;
use crate::include::common::intops::imax;
use crate::include::common::intops::imin;

extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
}

pub type CdefEdgeFlags = libc::c_uint;
pub const CDEF_HAVE_BOTTOM: CdefEdgeFlags = 8;
pub const CDEF_HAVE_TOP: CdefEdgeFlags = 4;
pub const CDEF_HAVE_RIGHT: CdefEdgeFlags = 2;
pub const CDEF_HAVE_LEFT: CdefEdgeFlags = 1;

#[inline]
pub unsafe extern "C" fn constrain(
    diff: libc::c_int,
    threshold: libc::c_int,
    shift: libc::c_int,
) -> libc::c_int {
    let adiff: libc::c_int = abs(diff);
    return apply_sign(
        imin(adiff, imax(0 as libc::c_int, threshold - (adiff >> shift))),
        diff,
    );
}
