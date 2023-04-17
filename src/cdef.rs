use crate::include::common::intops::apply_sign;
use crate::include::common::intops::imax;
use crate::include::common::intops::imin;
use crate::include::stdint::int16_t;
use crate::include::stddef::ptrdiff_t;

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
    let adiff: libc::c_int = diff.abs();
    return apply_sign(
        imin(adiff, imax(0 as libc::c_int, threshold - (adiff >> shift))),
        diff,
    );
}

#[inline]
pub unsafe extern "C" fn fill(
    mut tmp: *mut int16_t,
    stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < h {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < w {
            *tmp
                .offset(
                    x as isize,
                ) = (-(32767 as libc::c_int) - 1 as libc::c_int) as int16_t;
            x += 1;
        }
        tmp = tmp.offset(stride as isize);
        y += 1;
    }
}
