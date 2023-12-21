use crate::src::c_box::CBox;
use crate::src::error::Rav1dResult;
use std::marker::PhantomData;
use std::ops::AddAssign;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::Arc;

/// A C/custom [`Arc`].
///
/// That is, it is analogous to an [`Arc`],
/// but it lets you set a C-style `free` `fn` for deallocation
/// instead of the normal [`Box`] (de)allocator.
/// It can also store a normal [`Box`] as well.
///
/// It is built around the [`CBox`] abstraction.
/// However, that necessitates a double indirection
/// to reach the ptr through the [`Arc`] and [`CBox`].
/// To remedy this and improve performance,
/// a stable pointer is stored inline,
/// removing the double indirection.
/// This self-referential ptr is sound
/// because [`Arc`] and [`Box`]/[`CBox`]
/// never move their data ptr during moves, as it's on the heap.
/// As long as [`Self::owner`] is never moved
/// without also re-updating [`Self::stable_ref`], this is sound.
///
/// Furthermore, storing this stable ref ptr like this
/// allows for provenance projections of [`Self::stable_ref`],
/// such as slicing it for a `CArc<[T]>`.
#[derive(Debug)]
pub struct CArc<T: ?Sized> {
    owner: Arc<CBox<T>>,
    stable_ref: NonNull<T>,
}

impl<T: ?Sized> AsRef<T> for CArc<T> {
    fn as_ref(&self) -> &T {
        // Safety: [`Self::stable_ref`] is a ptr
        // derived from [`Self::owner`]'s through [`CBox::as_ref`]
        // and is thus safe to dereference.
        unsafe { self.stable_ref.as_ref() }
    }
}

impl<T: ?Sized> Deref for CArc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T: ?Sized> Clone for CArc<T> {
    fn clone(&self) -> Self {
        let Self { owner, stable_ref } = self;
        Self {
            owner: owner.clone(),
            // Safety: The ref remains stable across an [`Arc::clone`].
            stable_ref: stable_ref.clone(),
        }
    }
}

impl<T: ?Sized> From<Arc<CBox<T>>> for CArc<T> {
    fn from(owner: Arc<CBox<T>>) -> Self {
        let stable_ref = (*owner).as_ref().into();
        Self { owner, stable_ref }
    }
}

impl<T: ?Sized> CArc<T> {
    pub fn wrap(owner: CBox<T>) -> Rav1dResult<Self> {
        let owner = Arc::new(owner); // TODO fallible allocation
        Ok(owner.into())
    }
}

/// An opaque, raw [`CArc`], which is represented as a [`NonNull`] ptr to a [`CBox`].
///
/// To keep the type FFI-safe, a [`PhantomData`] wrapper is used,
/// but the ptr is actually a [`CBox`].
#[repr(transparent)]
pub struct RawCArc<T: ?Sized>(NonNull<PhantomData<CBox<T>>>);

impl<T: ?Sized> CArc<T> {
    /// Convert into a raw, opaque form suitable for C FFI.
    pub fn into_raw(self) -> RawCArc<T> {
        let raw = Arc::into_raw(self.owner);
        // Safety: [`Arc::into_raw`] guarantees it always returns non-null ptrs.
        let raw = unsafe { NonNull::new_unchecked(raw.cast_mut()) };
        RawCArc(raw.cast())
    }

    /// # Safety
    ///
    /// The [`RawCArc`] must be originally from [`Self::into_raw`].
    #[deny(unsafe_op_in_unsafe_fn)]
    pub unsafe fn from_raw(raw: RawCArc<T>) -> Self {
        // Safety: The [`RawCArc`] contains the output of [`Arc::into_raw`],
        // so we can call [`Arc::from_raw`] on it.
        let owner = unsafe { Arc::from_raw(raw.0.cast::<CBox<T>>().as_ptr()) };
        owner.into()
    }
}

impl<T> AddAssign<usize> for CArc<[T]> {
    /// Slice [`Self::stable_ref`].
    fn add_assign(&mut self, rhs: usize) {
        self.stable_ref = self[rhs..].into();
    }
}

impl<T> CArc<[T]>
where
    T: Default + 'static,
{
    pub fn zeroed_slice(size: usize) -> Rav1dResult<Self> {
        let owned_slice = (0..size).map(|_| Default::default()).collect::<Box<[_]>>(); // TODO fallible allocation
        Self::wrap(CBox::from_box(owned_slice))
    }
}
