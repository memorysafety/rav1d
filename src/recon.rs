use crate::src::msac::dav1d_msac_decode_bool_equi;
use crate::src::msac::MsacContext;

/// This is a macro that defines a function
/// because it takes `Dav1dFrameContext` and `Dav1dTaskContext` as arguments,
/// which have not yet been deduplicated/genericized over bitdepth.
macro_rules! define_DEBUG_BLOCK_INFO {
    () => {
        unsafe fn DEBUG_BLOCK_INFO(
            f: *const Dav1dFrameContext,
            t: *const Dav1dTaskContext,
        ) -> bool {
            /* TODO: add feature and compile-time guard around this code */
            0 != 0
                && (*(*f).frame_hdr).frame_offset == 2
                && (*t).by >= 0
                && (*t).by < 4
                && (*t).bx >= 8
                && (*t).bx < 12
            // true
        }
    };
}

pub(crate) use define_DEBUG_BLOCK_INFO;

#[inline]
pub unsafe extern "C" fn read_golomb(msac: &mut MsacContext) -> libc::c_uint {
    let mut len = 0;
    let mut val: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    while !dav1d_msac_decode_bool_equi(msac) && len < 32 {
        len += 1;
    }
    loop {
        let fresh3 = len;
        len = len - 1;
        if !(fresh3 != 0) {
            break;
        }
        val = (val << 1).wrapping_add(dav1d_msac_decode_bool_equi(msac) as libc::c_uint);
    }
    return val.wrapping_sub(1 as libc::c_int as libc::c_uint);
}
