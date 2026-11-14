use std::marker::PhantomData;
use std::ptr;

use crate::with_offset::WithOffset;

/// A type that bypasses `#[warn(improper_ctypes)]` checks of FFI safe types.
/// This type is meant to roundtrip a reference to a type `T` with lifetime `'a`
/// through an `extern "C" fn` ptr from a Rust caller to Rust callee.
/// Non-Rust callees should not access this type.
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
}

impl<'a, T> FFISafe<'a, WithOffset<&'a T>> {
    /// # Safety
    ///
    /// `this` must have been returned from [`WithOffset::as_ffi_safe`].
    pub unsafe fn from_with_offset(this: *const Self) -> WithOffset<&'a T> {
        // SAFETY: We required that the caller created `this` using `as_ffi_safe`, which uses `FFISafe::new`.
        *unsafe { FFISafe::get(this) }
    }
}

impl<'a, T> WithOffset<&'a T> {
    /// Convert `self` into an FFI-safe type.
    ///
    /// This is passed by pointer rather than by value, as a by-value 16-byte `struct`
    /// that ends up being passed on the stack is first spilled as two 8-byte stores
    /// and then copied with a 16-byte load, which is a store forwarding stall.
    pub fn as_ffi_safe<'b>(&'b self) -> *const FFISafe<'b, WithOffset<&'b T>>
    where
        'a: 'b,
    {
        FFISafe::new(self)
    }
}
