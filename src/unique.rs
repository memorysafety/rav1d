//! Largely copied from [`rust-lang/rust/library/core/src/ptr/unique.rs`](https://github.com/rust-lang/rust/blob/e35364a521372ce682e4bd4a5850d97ea33b0eab/library/core/src/ptr/unique.rs#L58).

use std::fmt;
use std::marker::PhantomData;
use std::ptr::NonNull;

/// Same as [`core::ptr::Unique`].
///
/// A wrapper around a [`NonNull<T>`] that indicates that the possessor
/// of this wrapper owns the referent.
///
/// [`Unique<T>`] behaves "as if" it were an instance of `T`.
/// It implements [`Send`]/[`Sync`] if `T: `[`Send`]/[`Sync`].
/// It also implies the kind of strong aliasing guarantees an instance of `T` can expect:
/// the referent of the pointer should not be modified
/// without a unique path to its owning [`Unique`].
///
/// Unlike [`NonNull<T>`], [`Unique<T>`] is covariant over `T`.
/// This should always be correct for any type which upholds [`Unique`]'s aliasing requirements.
#[repr(transparent)]
pub struct Unique<T: ?Sized> {
    pointer: NonNull<T>,
    // NOTE: this marker has no consequences for variance, but is necessary
    // for dropck to understand that we logically own a `T`.
    //
    // For details, see:
    // https://github.com/rust-lang/rfcs/blob/master/text/0769-sound-generic-drop.md#phantom-data
    _marker: PhantomData<T>,
}

/// SAFETY: [`Unique<T>`] is [`Send`] if `T: `[`Send`]
/// because the data it references is unaliased.
unsafe impl<T: Send + ?Sized> Send for Unique<T> {}

/// SAFETY: [`Unique<T>`] is [`Sync`] if `T: `[`Sync`]
unsafe impl<T: Sync + ?Sized> Sync for Unique<T> {}

impl<T: Sized> Unique<T> {
    /// Creates a new [`Unique`] that is dangling, but well-aligned.
    ///
    /// This is useful for initializing types which lazily allocate, like
    /// [`Vec::new`] does.
    ///
    /// Note that the pointer value may potentially represent a valid pointer to
    /// a `T`, which means this must not be used as a "not yet initialized"
    /// sentinel value. Types that lazily allocate must track initialization by
    /// some other means.
    #[must_use]
    #[inline]
    pub const fn dangling() -> Self {
        // FIXME(const-hack) replace with `From`
        Unique {
            pointer: NonNull::dangling(),
            _marker: PhantomData,
        }
    }

    #[inline]
    #[must_use]
    pub fn is_aligned(&self) -> bool {
        self.pointer.is_aligned()
    }
}

impl<T: ?Sized> Unique<T> {
    /// Creates a new [`Unique`].
    ///
    /// # Safety
    ///
    /// `ptr` must be unique.
    #[inline]
    pub const unsafe fn new(ptr: NonNull<T>) -> Self {
        Unique {
            pointer: ptr,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn from_ref_mut(reference: &mut T) -> Self {
        let ptr = reference.into();
        // SAFETY: `&mut` guarantees uniqueness.
        unsafe { Self::new(ptr) }
    }

    /// # Safety
    ///
    /// `reference` must be unique.
    #[inline]
    pub unsafe fn from_ref(reference: &T) -> Self {
        let ptr = reference.into();
        // SAFETY: Guaranteed by safety preconditions.
        unsafe { Self::new(ptr) }
    }

    /// Acquires the underlying `*mut` pointer.
    #[must_use = "`self` will be dropped if the result is not used"]
    #[inline]
    pub const fn as_ptr(self) -> *mut T {
        self.pointer.as_ptr()
    }

    /// Acquires the underlying `*mut` pointer.
    #[must_use = "`self` will be dropped if the result is not used"]
    #[inline]
    pub const fn as_non_null_ptr(self) -> NonNull<T> {
        self.pointer
    }

    /// Dereferences the content.
    ///
    /// The resulting lifetime is bound to self so this behaves "as if"
    /// it were actually an instance of T that is getting borrowed. If a longer
    /// (unbound) lifetime is needed, use `&*my_ptr.as_ptr()`.
    ///
    /// # Safety
    ///
    /// The pointer must be valid to dereference.
    #[must_use]
    #[inline]
    pub const unsafe fn as_ref(&self) -> &T {
        // SAFETY: the caller must guarantee that `self` meets all the
        // requirements for a reference.
        unsafe { self.pointer.as_ref() }
    }

    /// Mutably dereferences the content.
    ///
    /// The resulting lifetime is bound to self so this behaves "as if"
    /// it were actually an instance of T that is getting borrowed. If a longer
    /// (unbound) lifetime is needed, use `&mut *my_ptr.as_ptr()`.
    ///
    /// # Safety
    ///
    /// The pointer must be valid to dereference.#[must_use]
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        // SAFETY: the caller must guarantee that `self` meets all the
        // requirements for a mutable reference.
        unsafe { self.pointer.as_mut() }
    }

    /// Casts to a pointer of another type.
    #[must_use = "`self` will be dropped if the result is not used"]
    #[inline]
    pub const fn cast<U>(self) -> Unique<U> {
        // FIXME(const-hack): replace with `From`
        // SAFETY: is `NonNull`
        Unique {
            pointer: self.pointer.cast(),
            _marker: PhantomData,
        }
    }

    pub fn map(self, f: impl FnOnce(NonNull<T>) -> NonNull<T>) -> Self {
        Self {
            pointer: f(self.pointer),
            _marker: PhantomData,
        }
    }
}

impl<T: ?Sized> Clone for Unique<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Copy for Unique<T> {}

impl<T: ?Sized> fmt::Debug for Unique<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.as_ptr(), f)
    }
}

impl<T: ?Sized> fmt::Pointer for Unique<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.as_ptr(), f)
    }
}

impl<T: ?Sized> From<&mut T> for Unique<T> {
    /// Converts a `&mut T` to a [`Unique<T>`].
    #[inline]
    fn from(reference: &mut T) -> Self {
        Self::from_ref_mut(reference)
    }
}
