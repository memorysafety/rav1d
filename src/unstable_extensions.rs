//! Unstable `fn`s copied directly from `std`, with the following differences:
//! * They are free `fn`s now, not methods.
//! * `self` is replaced by `this`.
//! * Things only accessible by `std` are replaced with stable counterparts, such as:
//!     * `exact_div` => `/`
//!     * `.unchecked_mul` => `*`
//!     * `const` `.expect` => `match` and `panic!`

use std::mem;
use std::slice::from_raw_parts;
use std::slice::from_raw_parts_mut;

/// From `1.75.0`.
pub const fn flatten<const N: usize, T>(this: &[[T; N]]) -> &[T] {
    let len = if mem::size_of::<T>() == 0 {
        match this.len().checked_mul(N) {
            None => panic!("slice len overflow"),
            Some(it) => it,
        }
    } else {
        // SAFETY: `this.len() * N` cannot overflow because `self` is
        // already in the address space.
        /* unsafe */
        this.len() * N
    };
    // SAFETY: `[T]` is layout-identical to `[T; N]`
    unsafe { from_raw_parts(this.as_ptr().cast(), len) }
}

/// From `1.75.0`.
#[inline]
#[must_use]
pub const unsafe fn as_chunks_unchecked<const N: usize, T>(this: &[T]) -> &[[T; N]] {
    // SAFETY: Caller must guarantee that `N` is nonzero and exactly divides the slice length
    let new_len = /* unsafe */ {
        assert!(N != 0 && this.len() % N == 0);
        this.len() / N
    };
    // SAFETY: We cast a slice of `new_len * N` elements into
    // a slice of `new_len` many `N` elements chunks.
    unsafe { from_raw_parts(this.as_ptr().cast(), new_len) }
}

/// From `1.75.0`.
#[inline]
#[track_caller]
#[must_use]
pub const fn as_chunks<const N: usize, T>(this: &[T]) -> (&[[T; N]], &[T]) {
    assert!(N != 0, "chunk size must be non-zero");
    let len = this.len() / N;
    let (multiple_of_n, remainder) = this.split_at(len * N);
    // SAFETY: We already panicked for zero, and ensured by construction
    // that the length of the subslice is a multiple of N.
    let array_slice = unsafe { as_chunks_unchecked(multiple_of_n) };
    (array_slice, remainder)
}

#[inline]
#[must_use]
pub unsafe fn as_chunks_unchecked_mut<const N: usize, T>(this: &mut [T]) -> &mut [[T; N]] {
    // SAFETY: Caller must guarantee that `N` is nonzero and exactly divides the slice length
    let new_len = /* unsafe */ {
        assert!(N != 0 && this.len() % N == 0);
        this.len() / N
    };
    // SAFETY: We cast a slice of `new_len * N` elements into
    // a slice of `new_len` many `N` elements chunks.
    unsafe { from_raw_parts_mut(this.as_mut_ptr().cast(), new_len) }
}

#[inline]
#[track_caller]
#[must_use]
pub fn as_chunks_mut<'a, const N: usize, T>(this: &'a mut [T]) -> (&'a mut [[T; N]], &'a mut [T]) {
    assert!(N != 0, "chunk size must be non-zero");
    let len = this.len() / N;
    let (multiple_of_n, remainder) = this.split_at_mut(len * N);
    // SAFETY: We already panicked for zero, and ensured by construction
    // that the length of the subslice is a multiple of N.
    let array_slice = unsafe { as_chunks_unchecked_mut(multiple_of_n) };
    (array_slice, remainder)
}
