use super::internal::Dav1dTaskContext;
use super::intra_edge::EdgeFlags;
use super::levels::Av1Block;
use super::levels::BlockSize;
use crate::src::internal::Dav1dFrameContext;
use crate::src::msac::dav1d_msac_decode_bool_equi;
use crate::src::msac::MsacContext;

pub type recon_b_intra_fn = Option<
    unsafe extern "C" fn(*mut Dav1dTaskContext, BlockSize, EdgeFlags, *const Av1Block) -> (),
>;

pub type recon_b_inter_fn =
    Option<unsafe extern "C" fn(*mut Dav1dTaskContext, BlockSize, *const Av1Block) -> libc::c_int>;

use crate::src::internal::filter_sbrow_fn;

/// This is a macro that defines a function
/// because it takes `Dav1dFrameContext` and `Dav1dTaskContext` as arguments,
/// which have not yet been deduplicated/genericized over bitdepth.
///
/// TODO: This should not remain a macro.
/// It should either be a `fn` generic over bitdepth
/// or take `struct` arguments that are the subset of fields that are actually used in this `fn`,
/// as this would also solve some borrowck errors that had to be worked around.
macro_rules! define_DEBUG_BLOCK_INFO {
    () => {
        /// TODO: add feature and compile-time guard around this code
        unsafe fn DEBUG_BLOCK_INFO(f: &Dav1dFrameContext, t: &Dav1dTaskContext) -> bool {
            false
                && (*f.frame_hdr).frame_offset == 2
                && t.by >= 0
                && t.by < 4
                && t.bx >= 8
                && t.bx < 12
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
