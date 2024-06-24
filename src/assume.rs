#![deny(unsafe_op_in_unsafe_fn)]

use std::hint::unreachable_unchecked;

/// A stable version of [`core::intrinsics::assume`].
///
/// # Safety
///
/// `condition` must always be `true`.
#[inline(always)]
pub const unsafe fn assume(condition: bool) {
    if !condition {
        // SAFETY: `condition` is `true` by the `# Safety` preconditions.
        unsafe { unreachable_unchecked() };
    }
}
