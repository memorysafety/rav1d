use crate::src::msac::dav1d_msac_decode_bool_equi;
use crate::src::msac::MsacContext;

#[inline]
pub unsafe extern "C" fn read_golomb(msac: *mut MsacContext) -> libc::c_uint {
    let mut len = 0;
    let mut val: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    while dav1d_msac_decode_bool_equi(msac) == 0 && len < 32 as libc::c_int {
        len += 1;
    }
    loop {
        let fresh3 = len;
        len = len - 1;
        if !(fresh3 != 0) {
            break;
        }
        val = (val << 1 as libc::c_int).wrapping_add(dav1d_msac_decode_bool_equi(msac));
    }
    return val.wrapping_sub(1 as libc::c_int as libc::c_uint);
}
