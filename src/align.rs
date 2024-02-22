//! Types for enforcing alignment on struct fields.
//!
//! This module defines a handful of `AlignN` types, where `N` is alignment
//! enforced by that type. These types also implement a few useful traits to
//! make them easier to use in common cases, e.g. [`From`] and
//! [`Index`]/[`IndexMut`] (since it's usually array fields that require
//! specific aligment for use with SIMD instructions).

use std::marker::PhantomData;
use std::mem;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Index;
use std::ops::IndexMut;
use std::slice;

/// [`Default`] isn't `impl`emented for all arrays `[T; N]`
/// because they were implemented before `const` generics
/// and thus only for low values of `N`.
pub trait ArrayDefault {
    fn default() -> Self;
}

impl<T: ArrayDefault + Copy, const N: usize> ArrayDefault for [T; N] {
    fn default() -> Self {
        [T::default(); N]
    }
}

macro_rules! impl_ArrayDefault {
    ($T:ty) => {
        impl ArrayDefault for $T {
            fn default() -> Self {
                <Self as Default>::default()
            }
        }
    };
}

// We want this to be implemented for all `T: Default` where `T` is not `[_; _]`,
// but we can't do that, so we can just add individual
// `impl`s here for types we need it for.
impl_ArrayDefault!(u8);
impl_ArrayDefault!(i8);
impl_ArrayDefault!(i16);
impl_ArrayDefault!(i32);
impl_ArrayDefault!(u16);

macro_rules! def_align {
    ($align:literal, $name:ident) => {
        #[derive(Clone, Copy)]
        #[repr(C, align($align))]
        pub struct $name<T>(pub T);

        impl<T> From<T> for $name<T> {
            fn from(from: T) -> Self {
                Self(from)
            }
        }

        impl<T: Index<usize>> Index<usize> for $name<T> {
            type Output = T::Output;

            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }

        impl<T: IndexMut<usize>> IndexMut<usize> for $name<T> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.0[index]
            }
        }

        impl<T: ArrayDefault> ArrayDefault for $name<T> {
            fn default() -> Self {
                Self(T::default())
            }
        }

        impl<T: ArrayDefault> Default for $name<T> {
            fn default() -> Self {
                <Self as ArrayDefault>::default()
            }
        }
    };
}

def_align!(1, Align1);
def_align!(2, Align2);
def_align!(4, Align4);
def_align!(8, Align8);
def_align!(16, Align16);
def_align!(32, Align32);
def_align!(64, Align64);

/// A [`Vec`] that uses a 64-byte aligned allocation.
///
/// Only works with [`Copy`] types so that we don't have to handle drop logic.
pub struct AlignedVec64<T: Copy> {
    inner: Vec<MaybeUninit<Align64<[u8; 64]>>>,

    /// The number of `T`s in [`Self::inner`] currently initialized.
    len: usize,
    _phantom: PhantomData<T>,
}

impl<T: Copy> AlignedVec64<T> {
    pub const fn new() -> Self {
        Self {
            inner: Vec::new(),
            len: 0,
            _phantom: PhantomData,
        }
    }

    /// Returns the number of elements in the vector.
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_ptr(&self) -> *const T {
        self.inner.as_ptr().cast()
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.inner.as_mut_ptr().cast()
    }

    /// Extracts a slice containing the entire vector.
    pub fn as_slice(&self) -> &[T] {
        // Safety: The first `len` elements have been initialized to `T`s in
        // `Self::resize_with`.
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len) }
    }

    /// Extracts a mutable slice of the entire vector.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        // Safety: The first `len` elements have been initialized to `T`s in
        // `Self::resize_with`.
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len) }
    }

    pub fn resize(&mut self, new_len: usize, value: T) {
        let old_len = self.len();

        // Resize the underlying vector to have enough chunks for the new length.
        //
        // NOTE: We don't need to `drop` any elements if the `Vec` is truncated since
        // `T: Copy`.
        let new_bytes = mem::size_of::<T>() * new_len;
        let new_chunks = if (new_bytes % 64) == 0 {
            new_bytes / 64
        } else {
            (new_bytes / 64) + 1
        };
        self.inner.resize_with(new_chunks, MaybeUninit::uninit);

        // If we grew the vector, initialize the new elements past `len`.
        for offset in old_len..new_len {
            // SAFETY: We've allocated enough space to write up to `new_len` elements into
            // the buffer.
            unsafe {
                self.as_mut_ptr().add(offset).write(value);
            }
        }

        self.len = new_len;
    }
}

impl<T: Copy> Deref for AlignedVec64<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T: Copy> DerefMut for AlignedVec64<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

// NOTE: Custom impl so that we don't require `T: Default`.
impl<T: Copy> Default for AlignedVec64<T> {
    fn default() -> Self {
        Self::new()
    }
}
