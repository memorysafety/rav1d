use std::ptr::NonNull;

/// A [`NonNull`] that is [`Send`]` + `[`Sync`].
///
/// Since [`Send`]/[`Sync`] safety can't be guaranteed by the type system
/// due to the raw [`NonNull`] pointers and [`Self::cast`],
/// [`Send`]/[`Sync`] safety is instead guaranteed by
/// the `unsafe`ty of [`Self::new_unchecked`].
///
/// Safe wrappers are provided around this, namely
/// * [`Self::from_ref`]
/// * [`Self::from_box`]
///
/// This can be reversed with the corresponding methods:
/// * [`Self::as_ref`]
/// * [`Self::into_box`]
///
/// though these are `unsafe` due to the raw pointers
/// and potential [`Self::cast`] called in between.
#[derive(Debug)]
#[repr(transparent)]
pub struct SendSyncNonNull<T: ?Sized>(NonNull<T>);

// `Self: Clone` doesn't depend on `T: Clone`.
impl<T: ?Sized> Clone for SendSyncNonNull<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Copy for SendSyncNonNull<T> {}

/// SAFETY: All public constructors, which all go through [`Self::new_unchecked`],
/// ensure that [`Self`] stores a [`Send`] type
/// that is only used in a [`Send`] way.
unsafe impl<T: ?Sized> Send for SendSyncNonNull<T> {}

/// SAFETY: All public constructors, which all go through [`Self::new_unchecked`],
/// ensure that [`Self`] stores a [`Sync`] type
/// that is only used in a [`Sync`] way.
unsafe impl<T: ?Sized> Sync for SendSyncNonNull<T> {}

impl<T: ?Sized> SendSyncNonNull<T> {
    pub const fn cast<U>(self) -> SendSyncNonNull<U> {
        SendSyncNonNull(self.0.cast())
    }
}

impl<T: ?Sized + Send + Sync> SendSyncNonNull<T> {
    /// # Safety
    ///
    /// `ptr` must be derived from a type that is [`Send`]` + `[`Sync`],
    /// and must only be used in a [`Send`]` + `[`Sync`] way.
    pub const unsafe fn new_unchecked(ptr: NonNull<T>) -> Self {
        Self(ptr)
    }

    pub fn from_ref(r#ref: &T) -> Self {
        let ptr = NonNull::from(r#ref);
        // SAFETY: `T: Send + Sync => &T: Send + Sync`.
        unsafe { Self::new_unchecked(ptr) }
    }

    pub fn from_box(r#box: Box<T>) -> Self {
        let ptr = NonNull::new(Box::into_raw(r#box)).unwrap();
        // SAFETY: `T: Send + Sync => Box<T>: Send + Sync`.
        unsafe { Self::new_unchecked(ptr) }
    }

    pub const fn as_ptr(&self) -> NonNull<T> {
        self.0
    }

    /// # Safety
    ///
    /// `self` originally came from [`Self::from_ref`].
    pub const unsafe fn as_ref(&self) -> &T {
        // SAFETY: `self` originally came from a `&T` in `Self::from_ref`.
        unsafe { self.0.as_ref() }
    }

    /// # Safety
    ///
    /// `self` originally came from [`Self::from_box`]
    /// and was not already converted into a [`Box`] with [`Self::into_box`].
    pub unsafe fn into_box(self) -> Box<T> {
        let ptr = self.as_ptr().as_ptr();
        // SAFETY: `self` originally came from a `Box<T>` in `Self::from_box`.
        // And since `Self::into_box` hasn't been called yet, it's unique.
        unsafe { Box::from_raw(ptr) }
    }
}
