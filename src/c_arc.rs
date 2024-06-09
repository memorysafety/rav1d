use crate::src::c_box::CBox;
use crate::src::error::Rav1dResult;
use std::marker::PhantomData;
use std::ops::Deref;
use std::pin::Pin;
use std::ptr::NonNull;
use std::slice::SliceIndex;
use std::sync::Arc;

pub fn arc_into_raw<T: ?Sized>(arc: Arc<T>) -> NonNull<T> {
    let raw = Arc::into_raw(arc).cast_mut();
    // Safety: [`Arc::into_raw`] never returns null.
    unsafe { NonNull::new_unchecked(raw) }
}

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
/// because the [`CBox`] is [`Pin`]ned.
/// As long as [`Self::owner`] is never replaced
/// without also re-updating [`Self::stable_ref`], this is sound.
///
/// Furthermore, storing this stable ref ptr like this
/// allows for provenance projections of [`Self::stable_ref`],
/// such as slicing it for a `CArc<[T]>` (see [`Self::slice_in_place`]).
#[derive(Debug)]
pub struct CArc<T: ?Sized> {
    owner: Arc<Pin<CBox<T>>>,

    /// The same as [`Self::stable_ref`] but it never changes.
    #[cfg(debug_assertions)]
    base_stable_ref: NonNull<T>,

    stable_ref: NonNull<T>,
}

impl<T: ?Sized> AsRef<T> for CArc<T> {
    fn as_ref(&self) -> &T {
        #[cfg(debug_assertions)]
        {
            use std::mem;
            use to_method::To;

            // Some extra checks to check if our ptrs are definitely invalid.

            let real_ref = (*self.owner).as_ref().get_ref();
            assert_eq!(real_ref.to::<NonNull<T>>(), self.base_stable_ref);

            // Cast through `*const ()` and use [`pointer::byte_offset_from`]
            // to remove any fat ptr metadata.
            let offset = unsafe {
                self.stable_ref
                    .as_ptr()
                    .cast::<()>()
                    .byte_offset_from((real_ref as *const T).cast::<()>())
            };
            let offset = offset.try_to::<usize>().unwrap();
            let len = mem::size_of_val(real_ref);
            let out_of_bounds = offset > len;
            if out_of_bounds {
                dbg!(real_ref as *const T);
                dbg!(self.stable_ref.as_ptr());
                dbg!(offset);
                dbg!(len);
                panic!("CArc::stable_ref is out of bounds");
            }
        }

        // Safety: [`Self::stable_ref`] is a ptr
        // derived from [`Self::owner`]'s through [`CBox::as_ref`]
        // and is thus safe to dereference.
        // The [`CBox`] is [`Pin`]ned and
        // [`Self::stable_Ref`] is always updated on writes to [`Self::owner`],
        // so they are always in sync.
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
        let Self {
            owner,
            #[cfg(debug_assertions)]
            base_stable_ref,
            stable_ref,
        } = self;
        Self {
            owner: owner.clone(),
            #[cfg(debug_assertions)]
            base_stable_ref: base_stable_ref.clone(),
            stable_ref: stable_ref.clone(),
        }
    }
}

impl<T: ?Sized> From<Arc<Pin<CBox<T>>>> for CArc<T> {
    fn from(owner: Arc<Pin<CBox<T>>>) -> Self {
        let stable_ref = (*owner).as_ref().get_ref().into();
        Self {
            owner,
            #[cfg(debug_assertions)]
            base_stable_ref: stable_ref,
            stable_ref,
        }
    }
}

impl<T: ?Sized> CArc<T> {
    pub fn wrap(owner: CBox<T>) -> Rav1dResult<Self> {
        let owner = Arc::new(owner.into_pin()); // TODO fallible allocation
        Ok(owner.into())
    }
}

/// An opaque, raw [`Arc`] ptr.
///
/// See [`Arc::from_raw`], [`Arc::into_raw`], and [`arc_into_raw`].
///
/// The [`PhantomData`] is so it can be FFI-safe
/// without `T` having to be `#[repr(C)]`,
/// which it doesn't since it's opaque,
/// while still keeping `T` in the type.
#[repr(transparent)]
pub struct RawArc<T>(NonNull<PhantomData<T>>);

/// We need a manual `impl` since we don't require `T: Clone`.
///
/// # Safety
///
/// Note that this [`RawArc::clone`] does not call [`Arc::clone`],
/// since implicit clones/copies are expected to be done outside of Rust,
/// for which there is no way to force [`RawArc::clone`] to be called.
/// Instead, [`RawArc::as_ref`] and [`RawArc::into_arc`] are `unsafe`,
/// and require [`RawArc::clone`]s (actual explicit calls
/// or implicit ones outside of Rust) to respect the rules of [`Arc`].
impl<T> Clone for RawArc<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for RawArc<T> {}

impl<T> RawArc<T> {
    pub fn from_arc(arc: Arc<T>) -> Self {
        Self(arc_into_raw(arc).cast())
    }

    /// # Safety
    ///
    /// The [`RawArc`] must be originally from [`Self::from_arc`].
    pub unsafe fn as_ref(&self) -> &T {
        // SAFETY: `self` must be from `Self::from_arc`,
        // which calls `Arc::into_raw`,
        // which returns a ptr to its `T`.
        // `Arc` allows us to get a `&T` from it,
        // so this is allowed (unlike `&mut T`).
        // We don't call `Self::into_arc` since that's consuming,
        // so we'd have to `mem::forget` the `Arc`
        // and also do a redundant dereference.
        unsafe { self.0.cast().as_ref() }
    }

    /// # Safety
    ///
    /// The [`RawArc`] must be originally from [`Self::from_arc`].
    pub unsafe fn into_arc(self) -> Arc<T> {
        let raw = self.0.cast().as_ptr();
        // SAFETY: `self` must be from `Self::from_arc`,
        // which calls `Arc::into_raw`.
        // Thus, it is safe to call the inverse `Arc::from_raw` on it.
        unsafe { Arc::from_raw(raw) }
    }
}

#[repr(transparent)]
pub struct RawCArc<T: ?Sized>(RawArc<Pin<CBox<T>>>);

impl<T: ?Sized> CArc<T> {
    /// Convert into a raw, opaque form suitable for C FFI.
    pub fn into_raw(self) -> RawCArc<T> {
        RawCArc(RawArc::from_arc(self.owner))
    }

    /// # Safety
    ///
    /// The [`RawCArc`] must be originally from [`Self::into_raw`].
    pub unsafe fn from_raw(raw: RawCArc<T>) -> Self {
        // Safety: The [`RawCArc`] contains the output of [`Arc::into_raw`],
        // so we can call [`Arc::from_raw`] on it.
        let owner = raw.0.into_arc();
        owner.into()
    }
}

impl<T> CArc<[T]> {
    /// Slice [`Self::stable_ref`] in-place.
    ///
    /// The slice stays owned by the [`Arc`],
    /// but the [`Self::stable_ref`]/[`Self::as_ref`]/[`Self::deref`] view into it
    /// is assigned to the new sub-slice.
    pub fn slice_in_place<I>(&mut self, range: I)
    where
        I: SliceIndex<[T], Output = [T]>,
    {
        self.stable_ref = self.as_ref()[range].into();
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
