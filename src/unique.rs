use std::marker::PhantomData;
use std::ptr::NonNull;

/// Same as [`core::ptr::Unique`].
///
/// A wrapper around a [`NonNull`]`<T>` that indicates that the possessor
/// of this wrapper owns the referent.
///
/// [`Unique`]`<T>` behaves "as if" it were an instance of `T`.
/// It implements [`Send`]/[`Sync`] if `T: `[`Send`]/[`Sync`].
/// It also implies the kind of strong aliasing guarantees an instance of `T` can expect:
/// the referent of the pointer should not be modified
/// without a unique path to its owning [`Unique`].
///
/// Unlike [`NonNull`]`<T>`, `Unique<T>` is covariant over `T`.
/// This should always be correct for any type which upholds [`Unique`]'s aliasing requirements.
#[derive(Debug)]
pub struct Unique<T: ?Sized> {
    pub(crate) pointer: NonNull<T>,
    // NOTE: this marker has no consequences for variance, but is necessary
    // for dropck to understand that we logically own a `T`.
    //
    // For details, see:
    // https://github.com/rust-lang/rfcs/blob/master/text/0769-sound-generic-drop.md#phantom-data
    pub(crate) _marker: PhantomData<T>,
}

/// SAFETY: [`Unique`] is [`Send`] if `T: `[`Send`]
/// because the data it references is unaliased.
unsafe impl<T: Send + ?Sized> Send for Unique<T> {}

/// SAFETY: [`Unique`] is [`Sync`] if `T: `[`Sync`]
unsafe impl<T: Sync + ?Sized> Sync for Unique<T> {}
