use std::hint::unreachable_unchecked;

/// A stable version of [`core::intrinsic::assume`].
#[inline(always)]
pub unsafe fn assume(condition: bool) {
    if !condition {
        unreachable_unchecked();
    }
}
