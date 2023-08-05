use core::intrinsics::exact_div;
use core::slice::from_raw_parts;
use core::slice::from_raw_parts_mut;

mod private {
    pub trait Sealed {}

    impl<T> Sealed for [T] {}
}

pub trait SliceExt<T>: private::Sealed {
    unsafe fn ext_as_chunks_unchecked<const N: usize>(&self) -> &[[T; N]];

    unsafe fn ext_as_chunks_unchecked_mut<const N: usize>(&mut self) -> &mut [[T; N]];

    fn ext_as_chunks<const N: usize>(&self) -> (&[[T; N]], &[T]);

    fn ext_as_chunks_mut<const N: usize>(&mut self) -> (&mut [[T; N]], &mut [T]);
}

/// Copied directly from `libstd` to make it stable, with the following differences:
///
/// * Method names are prefixed with `ext_` to avoid collisions.
/// * Private `libstd` items with no effect, like `assert_unsafe_precondition!`, are commented out.
impl<T> SliceExt<T> for [T] {
    #[inline]
    #[must_use]
    unsafe fn ext_as_chunks_unchecked<const N: usize>(&self) -> &[[T; N]] {
        let this = self;
        // SAFETY: Caller must guarantee that `N` is nonzero and exactly divides the slice length
        let new_len = unsafe {
            // assert_unsafe_precondition!(
            //     "slice::as_chunks_unchecked requires `N != 0` and the slice to split exactly into `N`-element chunks",
            //     [T](this: &[T], N: usize) => N != 0 && this.len() % N == 0
            // );
            exact_div(this.len(), N)
        };
        // SAFETY: We cast a slice of `new_len * N` elements into
        // a slice of `new_len` many `N` elements chunks.
        unsafe { from_raw_parts(self.as_ptr().cast(), new_len) }
    }

    #[inline]
    #[must_use]
    unsafe fn ext_as_chunks_unchecked_mut<const N: usize>(&mut self) -> &mut [[T; N]] {
        let this = &*self;
        // SAFETY: Caller must guarantee that `N` is nonzero and exactly divides the slice length
        let new_len = unsafe {
            // assert_unsafe_precondition!(
            //     "slice::as_chunks_unchecked_mut requires `N != 0` and the slice to split exactly into `N`-element chunks",
            //     [T](this: &[T], N: usize) => N != 0 && this.len() % N == 0
            // );
            exact_div(this.len(), N)
        };
        // SAFETY: We cast a slice of `new_len * N` elements into
        // a slice of `new_len` many `N` elements chunks.
        unsafe { from_raw_parts_mut(self.as_mut_ptr().cast(), new_len) }
    }

    #[inline]
    #[track_caller]
    #[must_use]
    fn ext_as_chunks<const N: usize>(&self) -> (&[[T; N]], &[T]) {
        assert!(N != 0, "chunk size must be non-zero");
        let len = self.len() / N;
        let (multiple_of_n, remainder) = self.split_at(len * N);
        // SAFETY: We already panicked for zero, and ensured by construction
        // that the length of the subslice is a multiple of N.
        let array_slice = unsafe { multiple_of_n.ext_as_chunks_unchecked() };
        (array_slice, remainder)
    }

    #[inline]
    #[track_caller]
    #[must_use]
    fn ext_as_chunks_mut<const N: usize>(&mut self) -> (&mut [[T; N]], &mut [T]) {
        assert!(N != 0, "chunk size must be non-zero");
        let len = self.len() / N;
        let (multiple_of_n, remainder) = self.split_at_mut(len * N);
        // SAFETY: We already panicked for zero, and ensured by construction
        // that the length of the subslice is a multiple of N.
        let array_slice = unsafe { multiple_of_n.ext_as_chunks_unchecked_mut() };
        (array_slice, remainder)
    }
}
