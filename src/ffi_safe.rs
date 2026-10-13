use std::marker::PhantomData;
use std::{mem, ptr};

use crate::pixels::Pixels;
use crate::with_offset::WithOffset;

/// A type that bypasses `#[warn(improper_ctypes)]` checks of FFI safe types.
/// This type is meant to roundtrip a reference to a type `T` with lifetime `'a`
/// through an `extern "C" fn` ptr from a Rust caller to Rust callee.
/// Non-Rust callees should not access this type.
///
/// A `WithOffset<&'a T>` is roundtripped by passing `FFISafe::new(data)`
/// alongside the raw element ptr the caller already computed from it
/// (`base + offset`, e.g. [`WithOffset::as_ptr`]); the callee recomputes
/// the offset from that ptr with [`Self::with_offset_of`].
#[repr(C)]
pub struct FFISafe<'a, T> {
    phantom: PhantomData<&'a T>,
    non_zst: bool,
}

impl<'a, T> FFISafe<'a, T> {
    pub fn new(this: &'a T) -> *const Self {
        ptr::from_ref(this).cast()
    }

    pub fn _new_mut(this: &'a mut T) -> *mut Self {
        ptr::from_mut(this).cast()
    }

    /// # Safety
    ///
    /// `this` must have been returned from [`Self::new`].
    pub unsafe fn get(this: *const Self) -> &'a T {
        // SAFETY: `this` originally was a `&'a T` in `Self::new`.
        unsafe { &*this.cast() }
    }

    /// # Safety
    ///
    /// `this` must have been returned from [`Self::new_mut`].
    pub unsafe fn _get_mut(this: *mut Self) -> &'a mut T {
        // SAFETY: `this` originally was a `&'a mut T` in `Self::new_mut`.
        unsafe { &mut *this.cast() }
    }

    /// Reconstruct the `WithOffset<&'a T>` that `ptr` was computed from.
    ///
    /// Only the address of `ptr` is used; the returned reference carries the
    /// provenance of `this`. The offset is recovered with wrapping arithmetic,
    /// so a `ptr` computed with [`WithOffset::wrapping_as_ptr`] that lies
    /// outside the buffer still yields the original offset.
    ///
    /// # Safety
    ///
    /// `this` must have been returned from [`Self::new`], and `ptr` must have been
    /// computed from the same `WithOffset<&'a T>` as `base + offset`
    /// (e.g. [`WithOffset::as_ptr`]), where `base` is [`Pixels::as_byte_mut_ptr`]
    /// of the same `T` and `E` is the element type `offset` counts in.
    pub unsafe fn with_offset_of<E>(this: *const Self, ptr: *const E) -> WithOffset<&'a T>
    where
        T: Pixels,
    {
        // SAFETY: `this` was a `&'a T` in `Self::new`.
        let data = unsafe { Self::get(this) };
        let byte_offset = (ptr as usize).wrapping_sub(data.as_byte_mut_ptr() as usize);
        debug_assert_eq!(byte_offset % mem::size_of::<E>(), 0);
        WithOffset {
            data,
            offset: byte_offset / mem::size_of::<E>(),
        }
    }
}
