//! Wrapper that allows concurrent, disjoint mutation of a slice-like owned
//! structure.

#![deny(unsafe_op_in_unsafe_fn)]

use crate::align::AlignedByteChunk;
use crate::align::AlignedVec;
use std::cell::UnsafeCell;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::mem;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Index;
use std::ops::Range;
use std::ops::RangeFrom;
use std::ops::RangeFull;
use std::ops::RangeInclusive;
use std::ops::RangeTo;
use std::ops::RangeToInclusive;
use std::ptr;
use std::ptr::addr_of_mut;
use std::sync::Arc;
use zerocopy::AsBytes;
use zerocopy::FromBytes;

/// Wraps an indexable collection to allow unchecked concurrent mutable borrows.
///
/// This wrapper allows users to concurrently mutably borrow disjoint regions or
/// elements from a collection. This is necessary to allow multiple threads to
/// concurrently read and write to disjoint pixel data from the same arrays and
/// vectors.
///
/// In debug mode (debug assertions enabled), indexing returns a guard which
/// acts as a lock for the borrowed region and borrows are checked to ensure
/// that mutably borrowed regions are actually disjoint with all other borrows
/// for the lifetime of the returned borrow guard.
#[derive(Default)]
#[cfg_attr(not(debug_assertions), repr(transparent))]
pub struct DisjointMut<T: ?Sized + AsMutPtr> {
    #[cfg(debug_assertions)]
    bounds: debug::DisjointMutAllBounds,

    inner: UnsafeCell<T>,
}

/// SAFETY: If `T: `[`Send`], then sending [`DisjointMut`]`<T>` across threads is safe.
/// There is no non-[`Sync`] state that is left on another thread
/// when [`DisjointMut`] gets sent to another thread.
unsafe impl<T: ?Sized + AsMutPtr + Send> Send for DisjointMut<T> {}

/// SAFETY: [`DisjointMut`] only provides disjoint mutable access
/// to `T`'s elements through a shared `&`[`DisjointMut`]`<T>` reference.
/// Thus, sharing/[`Send`]ing a `&`[`DisjointMut`]`<T>` across threads is safe.
/// This disjointness is unchecked in release mode and relies on the `pub(crate)`
/// [`Self::index`] and [`Self::index_mut`] being used correctly (disjointly).
///
/// More precisely, `&`[`Self`] has the two core methods of
/// [`Self::index`] and [`Self::index_mut`]
/// that provide disjoint immutable and mutable access to `T`'s elements.
/// This disjointness guarantees that we do not create overlapping `&`s and `&mut`s,
/// which would be unsound and otherwise possible from multiple threads.
/// Furthermore, the safety guarantees of [`AsMutPtr::as_mut_ptr`]
/// ensure that no intermediary `&` or `&mut` references to `T`'s elements are created.
///
/// The disjointness guarantee is checked at runtime in debug mode
/// (i.e. [`#[cfg(debug_assertions)]`]), while in release mode,
/// disjointness must be manually guaranteed.
/// This is less than ideal, but crucial for performance.
/// Since [`DisjointMut`] and [`disjoint_mut`](module@self) are only `pub(crate)`,
/// we can ensure all uses are disjoint and thus sound.
///
/// Furthermore, all `T`s used have [`AsMutPtr::Target`]s
/// that are provenanceless, i.e. they have no internal references or pointers
/// or integers that hold pointer provenance.
/// Thus, a data race due the lack of runtime disjointness checking in release mode
/// would only result in wrong results, and cannot result in memory safety.
/// This is checked manually for now.
unsafe impl<T: ?Sized + AsMutPtr + Sync> Sync for DisjointMut<T> {}

impl<T: AsMutPtr> DisjointMut<T> {
    pub const fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(value),
            #[cfg(debug_assertions)]
            bounds: debug::DisjointMutAllBounds::new(),
        }
    }

    /// # Safety
    ///
    /// The returned ptr has the safety requirements of [`UnsafeCell::get`].
    /// In particular, the ptr returned by [`AsMutPtr::as_mut_ptr`] may be in use.
    pub const fn inner(&self) -> *mut T {
        self.inner.get()
    }

    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

#[cfg_attr(not(debug_assertions), repr(transparent))]
pub struct DisjointMutGuard<'a, T: ?Sized + AsMutPtr, V: ?Sized> {
    slice: &'a mut V,

    phantom: PhantomData<&'a DisjointMut<T>>,

    #[cfg(debug_assertions)]
    parent: &'a DisjointMut<T>,
    #[cfg(debug_assertions)]
    bounds: debug::DisjointMutBounds,
}

impl<'a, T: AsMutPtr> DisjointMutGuard<'a, T, [u8]> {
    #[inline] // Inline to see alignment to potentially elide checks.
    fn cast_slice<V: AsBytes + FromBytes>(self) -> DisjointMutGuard<'a, T, [V]> {
        // We don't want to drop the old guard, because we aren't changing or
        // removing the bounds from parent here.
        let mut old_guard = ManuallyDrop::new(self);
        let bytes = mem::take(&mut old_guard.slice);
        DisjointMutGuard {
            slice: V::mut_slice_from(bytes).unwrap(),
            phantom: old_guard.phantom,
            #[cfg(debug_assertions)]
            parent: old_guard.parent,
            #[cfg(debug_assertions)]
            bounds: mem::take(&mut old_guard.bounds),
        }
    }

    #[inline] // Inline to see alignment to potentially elide checks.
    fn cast<V: AsBytes + FromBytes>(self) -> DisjointMutGuard<'a, T, V> {
        // We don't want to drop the old guard, because we aren't changing or
        // removing the bounds from parent here.
        let mut old_guard = ManuallyDrop::new(self);
        let bytes = mem::take(&mut old_guard.slice);
        DisjointMutGuard {
            slice: V::mut_from(bytes).unwrap(),
            phantom: old_guard.phantom,
            #[cfg(debug_assertions)]
            parent: old_guard.parent,
            #[cfg(debug_assertions)]
            bounds: mem::take(&mut old_guard.bounds),
        }
    }
}

impl<'a, T: ?Sized + AsMutPtr, V: ?Sized> Deref for DisjointMutGuard<'a, T, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.slice
    }
}

impl<'a, T: ?Sized + AsMutPtr, V: ?Sized> DerefMut for DisjointMutGuard<'a, T, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.slice
    }
}

#[cfg_attr(not(debug_assertions), repr(transparent))]
pub struct DisjointImmutGuard<'a, T: ?Sized + AsMutPtr, V: ?Sized> {
    slice: &'a V,

    phantom: PhantomData<&'a DisjointMut<T>>,

    #[cfg(debug_assertions)]
    parent: &'a DisjointMut<T>,
    #[cfg(debug_assertions)]
    bounds: debug::DisjointMutBounds,
}

impl<'a, T: AsMutPtr> DisjointImmutGuard<'a, T, [u8]> {
    #[inline] // Inline to see alignment to potentially elide checks.
    fn cast_slice<V: FromBytes>(self) -> DisjointImmutGuard<'a, T, [V]> {
        // We don't want to drop the old guard, because we aren't changing or
        // removing the bounds from parent here.
        let mut old_guard = ManuallyDrop::new(self);
        let bytes = mem::take(&mut old_guard.slice);
        DisjointImmutGuard {
            slice: V::slice_from(bytes).unwrap(),
            phantom: old_guard.phantom,
            #[cfg(debug_assertions)]
            parent: old_guard.parent,
            #[cfg(debug_assertions)]
            bounds: mem::take(&mut old_guard.bounds),
        }
    }

    #[inline] // Inline to see alignment to potentially elide checks.
    fn cast<V: FromBytes>(self) -> DisjointImmutGuard<'a, T, V> {
        // We don't want to drop the old guard, because we aren't changing or
        // removing the bounds from parent here.
        let mut old_guard = ManuallyDrop::new(self);
        let bytes = mem::take(&mut old_guard.slice);
        DisjointImmutGuard {
            slice: V::ref_from(bytes).unwrap(),
            phantom: old_guard.phantom,
            #[cfg(debug_assertions)]
            parent: old_guard.parent,
            #[cfg(debug_assertions)]
            bounds: mem::take(&mut old_guard.bounds),
        }
    }
}

impl<'a, T: ?Sized + AsMutPtr, V: ?Sized> Deref for DisjointImmutGuard<'a, T, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.slice
    }
}

/// Convert from a mutable pointer to a collection to a mutable pointer to the
/// underlying slice without ever creating a mutable reference to the slice.
///
/// This trait exists for the same reason as [`Vec::as_mut_ptr`] - we want to
/// create a mutable pointer to the underlying slice without ever creating a
/// mutable reference to the slice.
///
/// # Safety
///
/// This trait must not ever create a mutable reference to the underlying slice,
/// as it may be (partially) immutably borrowed concurrently.
pub unsafe trait AsMutPtr {
    type Target;

    /// Convert a mutable pointer to a collection to a mutable pointer to the
    /// underlying slice.
    ///
    /// # Safety
    ///
    /// This method may dereference `ptr` as an immutable reference, so this
    /// pointer must be safely dereferenceable.
    unsafe fn as_mut_slice(ptr: *mut Self) -> *mut [Self::Target] {
        // SAFETY: The safety precondition of this method requires that we can
        // immutably dereference `ptr`.
        let len = unsafe { (*ptr).len() };
        // SAFETY: Mutably dereferencing and calling `.as_mut_ptr()` does not
        // materialize a mutable reference to the underlying slice according to
        // its documentated behavior, so we can still allow concurrent immutable
        // references into that underlying slice.
        let data = unsafe { Self::as_mut_ptr(ptr) };
        ptr::slice_from_raw_parts_mut(data, len)
    }

    /// Convert a mutable pointer to a collection to a mutable pointer to the
    /// first element of the collection.
    ///
    /// # Safety
    ///
    /// This method may dereference `ptr` as an immutable reference, so this
    /// pointer must be safely dereferenceable.
    ///
    /// The returned pointer is only safe to dereference within the bounds of
    /// the underlying collection.
    unsafe fn as_mut_ptr(ptr: *mut Self) -> *mut Self::Target;

    fn len(&self) -> usize;
}

impl<T: ?Sized + AsMutPtr> DisjointMut<T> {
    pub fn len(&self) -> usize {
        // SAFETY: The inner cell is safe to access immutably. We never create a
        // mutable reference to the inner value.
        unsafe { (*self.inner.get()).len() }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_mut_slice(&self) -> *mut [<T as AsMutPtr>::Target] {
        // SAFETY: The inner cell is safe to access immutably. We never create a
        // mutable reference to the inner value.
        unsafe { AsMutPtr::as_mut_slice(self.inner.get()) }
    }

    pub fn as_mut_ptr(&self) -> *mut <T as AsMutPtr>::Target {
        // SAFETY: The inner cell is safe to access immutably. We never create a
        // mutable reference to the inner value.
        unsafe { AsMutPtr::as_mut_ptr(self.inner.get()) }
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }

    /// Mutably borrow a slice or element.
    ///
    /// This mutable borrow may be unchecked and callers must ensure that no
    /// other borrows from this collection overlap with the mutably borrowed
    /// region for the lifetime of that mutable borrow.
    ///
    /// # Safety
    ///
    /// This method is not marked unsafe but its safety requires correct usage
    /// alongside other calls to [`index`] and [`index_mut`]. Caller must ensure
    /// that no elements of the resulting borrowed slice or element are
    /// concurrently borrowed (immutably or mutably) at all during the lifetime
    /// of the returned mutable borrow. This is checked in debug builds, but
    /// checks are disabled in release builds for performance. We also require
    /// that the referenced data must be plain data and not contain any pointers
    /// or references to avoid other potential memory safety issues due to racy
    /// access.
    ///
    /// [`index`]: DisjointMut::index
    /// [`index_mut`]: DisjointMut::index_mut
    #[inline] // Inline to see bounds checks in order to potentially elide them.
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn index_mut<'a, I>(&'a self, index: I) -> DisjointMutGuard<'a, T, I::Output>
    where
        I: Into<Bounds> + Clone,
        I: DisjointMutIndex<[<T as AsMutPtr>::Target]>,
    {
        let bounds = index.clone().into();
        // SAFETY: The safety preconditions of `index` and `index_mut` imply
        // that the indexed region we are mutably borrowing is not concurrently
        // borrowed and will not be borrowed during the lifetime of the returned
        // reference.
        let slice = unsafe { &mut *index.get_mut(self.as_mut_slice()) };
        DisjointMutGuard::new(self, slice, bounds)
    }

    /// Immutably borrow a slice or element.
    ///
    /// This immutable borrow may be unchecked and callers must ensure that no
    /// other mutable borrows from this collection overlap with the returned
    /// immutably borrowed region for the lifetime of that borrow.
    ///
    /// # Safety
    ///
    /// This method is not marked as unsafe but its safety requires correct
    /// usage alongside [`index_mut`]. It cannot result in a race
    /// condition without creating an overlapping mutable range via
    /// [`index_mut`]. As an internal helper, we ensure that all calls are
    /// safe and document this when mutating rather than marking each immutable
    /// reference with virtually identical safety justifications.
    ///
    /// Caller must take care that no elements of the resulting borrowed slice
    /// or element are concurrently mutably borrowed at all by [`index_mut`]
    /// during the lifetime of the returned borrow.
    ///
    /// [`index_mut`]: DisjointMut::index_mut
    #[inline] // Inline to see bounds checks in order to potentially elide them.
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn index<'a, I>(&'a self, index: I) -> DisjointImmutGuard<'a, T, I::Output>
    where
        I: Into<Bounds> + Clone,
        I: DisjointMutIndex<[<T as AsMutPtr>::Target]>,
    {
        let bounds = index.clone().into();
        // SAFETY: The safety preconditions of `index` and `index_mut` imply
        // that the indexed region we are immutably borrowing is not
        // concurrently mutably borrowed and will not be mutably borrowed during
        // the lifetime of the returned reference.
        let slice = unsafe { &*index.get_mut(self.as_mut_slice()).cast_const() };
        DisjointImmutGuard::new(self, slice, bounds)
    }
}

impl<T: AsMutPtr<Target = u8>> DisjointMut<T> {
    /// When we slice with [`Self::index`] or [`Self::index_mut`]
    /// on a scaled/translated range, the multiplication can overflow,
    /// causing the compiler to not be able to reason about the length of the slice anymore.
    /// Instead of checking for overflows, we can instead just check
    /// if the length of the casted slice is as expected.
    ///
    /// If the overflow was impossible (e.x. the range was casted from [`u32`] to [`usize`]),
    /// this will be optimized out.
    #[inline] // Inline to see the check.
    fn check_cast_slice_len<I, V>(&self, index: I, slice: &[V])
    where
        I: SliceBounds,
    {
        let range = index.to_range(self.len() / mem::size_of::<V>());
        let range_len = range.end - range.start;
        assert!(slice.len() == range_len);
    }

    /// Mutably borrow a slice of a convertible type.
    ///
    /// This method accesses a slice of elements of a type that implements
    /// `zerocopy::FromBytes` from a buffer of `u8`.
    ///
    /// This mutable borrow may be unchecked and callers must ensure that no
    /// other borrows from this collection overlap with the mutably borrowed
    /// region for the lifetime of that mutable borrow.
    ///
    /// # Safety
    ///
    /// Caller must ensure that no elements of the resulting borrowed slice or
    /// element are concurrently borrowed (immutably or mutably) at all during
    /// the lifetime of the returned mutable borrow. We require that the
    /// referenced data must be plain data and not contain any pointers or
    /// references to avoid other potential memory safety issues due to racy
    /// access.
    #[inline] // Inline to see bounds checks in order to potentially elide them.
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn mut_slice_as<'a, I, V>(&'a self, index: I) -> DisjointMutGuard<'a, T, [V]>
    where
        I: SliceBounds,
        V: AsBytes + FromBytes,
    {
        let slice = self.index_mut(index.mul(mem::size_of::<V>())).cast_slice();
        self.check_cast_slice_len(index, &slice);
        slice
    }

    /// Mutably borrow an element of a convertible type.
    ///
    /// This method accesses an element of a type that implements
    /// `zerocopy::FromBytes` from a buffer of `u8`.
    ///
    /// This mutable borrow may be unchecked and callers must ensure that no
    /// other borrows from this collection overlap with the mutably borrowed
    /// region for the lifetime of that mutable borrow.
    ///
    /// # Safety
    ///
    /// Caller must ensure that no elements of the resulting borrowed slice or
    /// element are concurrently borrowed (immutably or mutably) at all during
    /// the lifetime of the returned mutable borrow. We require that the
    /// referenced data must be plain data and not contain any pointers or
    /// references to avoid other potential memory safety issues due to racy
    /// access.
    #[inline] // Inline to see bounds checks in order to potentially elide them.
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn mut_element_as<'a, V>(&'a self, index: usize) -> DisjointMutGuard<'a, T, V>
    where
        V: AsBytes + FromBytes,
    {
        self.index_mut((index..index + 1).mul(mem::size_of::<V>()))
            .cast()
    }

    /// Immutably borrow a slice of a convertible type.
    ///
    /// This method accesses a slice of elements of a type that implements
    /// `zerocopy::FromBytes` from a buffer of `u8`.
    ///
    /// This immutable borrow may be unchecked and callers must ensure that no
    /// other mutable borrows from this collection overlap with the returned
    /// immutably borrowed region for the lifetime of that borrow.
    ///
    /// # Safety
    ///
    /// This method is not marked as unsafe but its safety requires correct
    /// usage alongside [`index_mut`]. It cannot result in a race
    /// condition without creating an overlapping mutable range via
    /// [`index_mut`]. As an internal helper, we ensure that all calls are
    /// safe and document this when mutating rather than marking each immutable
    /// reference with virtually identical safety justifications.
    ///
    /// Caller must take care that no elements of the resulting borrowed slice
    /// or element are concurrently mutably borrowed at all by [`index_mut`]
    /// during the lifetime of the returned borrow.
    ///
    /// [`index_mut`]: DisjointMut::index_mut
    #[inline] // Inline to see bounds checks in order to potentially elide them.
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn slice_as<'a, I, V>(&'a self, index: I) -> DisjointImmutGuard<'a, T, [V]>
    where
        I: SliceBounds,
        V: FromBytes,
    {
        let slice = self.index(index.mul(mem::size_of::<V>())).cast_slice();
        self.check_cast_slice_len(index, &slice);
        slice
    }

    /// Immutably borrow an element of a convertible type.
    ///
    /// This method accesses an element of a type that implements
    /// `zerocopy::FromBytes` from a buffer of `u8`.
    ///
    /// This immutable borrow may be unchecked and callers must ensure that no
    /// other mutable borrows from this collection overlap with the returned
    /// immutably borrowed region for the lifetime of that borrow.
    ///
    /// # Safety
    ///
    /// This method is not marked as unsafe but its safety requires correct
    /// usage alongside [`index_mut`]. It cannot result in a race condition
    /// without creating an overlapping mutable range via [`index_mut`]. As an
    /// internal helper, we ensure that all calls are safe and document this
    /// when mutating rather than marking each immutable reference with
    /// virtually identical safety justifications.
    ///
    /// Caller must take care that no elements of the resulting borrowed slice
    /// or element are concurrently mutably borrowed at all by [`index_mut`]
    /// during the lifetime of the returned borrow.
    ///
    /// [`index_mut`]: DisjointMut::index_mut
    #[inline] // Inline to see bounds checks in order to potentially elide them.
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn element_as<'a, V>(&'a self, index: usize) -> DisjointImmutGuard<'a, T, V>
    where
        V: FromBytes,
    {
        self.index((index..index + 1).mul(mem::size_of::<V>()))
            .cast()
    }
}

/// This trait is a stable implementation of [`std::slice::SliceIndex`] to allow
/// for indexing into mutable slice raw pointers.
pub trait DisjointMutIndex<T: ?Sized> {
    type Output: ?Sized;

    /// Returns a mutable pointer to the output at this indexed location. The
    /// `T` pointer must be valid to dereference to obtain the slice length.
    ///
    /// To implement, `T` should be a slice type that `Self` is a valid index
    /// into.
    ///
    /// This is a stable equivalent to
    /// [`std::slice::SliceIndex::get_unchecked_mut`] with bounds checking.
    ///
    /// # Safety
    ///
    /// `slice` must be a valid, dereferencable pointer that this function may
    /// dereference immutably.
    unsafe fn get_mut(self, slice: *mut T) -> *mut Self::Output;
}

pub trait TranslateRange {
    fn mul(&self, by: usize) -> Self;
}

impl TranslateRange for usize {
    fn mul(&self, by: usize) -> Self {
        *self * by
    }
}

impl TranslateRange for Range<usize> {
    fn mul(&self, by: usize) -> Self {
        self.start * by..self.end * by
    }
}

impl TranslateRange for RangeFrom<usize> {
    fn mul(&self, by: usize) -> Self {
        self.start * by..
    }
}

impl TranslateRange for RangeInclusive<usize> {
    fn mul(&self, by: usize) -> Self {
        *self.start() * by..=*self.end() * by
    }
}

impl TranslateRange for RangeTo<usize> {
    fn mul(&self, by: usize) -> Self {
        ..self.end * by
    }
}

impl TranslateRange for RangeToInclusive<usize> {
    fn mul(&self, by: usize) -> Self {
        ..=self.end * by
    }
}

impl TranslateRange for RangeFull {
    fn mul(&self, _by: usize) -> Self {
        *self
    }
}

impl TranslateRange for (RangeFrom<usize>, RangeTo<usize>) {
    fn mul(&self, by: usize) -> Self {
        (self.0.start * by.., ..self.1.end * by)
    }
}

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Bounds {
    /// A [`Range::end`]` == `[`usize::MAX`] is considered unbounded,
    /// as lengths need to be less than [`isize::MAX`] already.
    range: Range<usize>,
}

impl Display for Bounds {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Range { start, end } = self.range;
        if start != 0 {
            write!(f, "{start}")?;
        }
        write!(f, "..")?;
        if end != usize::MAX {
            write!(f, "{end}")?;
        }
        Ok(())
    }
}

impl Debug for Bounds {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Bounds {
    #[cfg(any(debug_assertions, test))]
    fn overlaps(&self, other: &Bounds) -> bool {
        let a = &self.range;
        let b = &other.range;
        a.start < b.end && b.start < a.end
    }
}

impl From<usize> for Bounds {
    fn from(index: usize) -> Self {
        Self {
            range: index..index + 1,
        }
    }
}

impl<T: SliceBounds> From<T> for Bounds {
    fn from(range: T) -> Self {
        Self {
            range: range.to_range(usize::MAX),
        }
    }
}

pub trait SliceBounds: TranslateRange + Clone {
    fn to_range(&self, len: usize) -> Range<usize>;
}

impl SliceBounds for Range<usize> {
    fn to_range(&self, _len: usize) -> Range<usize> {
        let Self { start, end } = *self;
        start..end
    }
}

impl SliceBounds for RangeFrom<usize> {
    fn to_range(&self, len: usize) -> Range<usize> {
        let Self { start } = *self;
        start..len
    }
}

impl SliceBounds for RangeInclusive<usize> {
    fn to_range(&self, _len: usize) -> Range<usize> {
        *self.start()..*self.end() + 1
    }
}

impl SliceBounds for RangeTo<usize> {
    fn to_range(&self, _len: usize) -> Range<usize> {
        let Self { end } = *self;
        0..end
    }
}

impl SliceBounds for RangeToInclusive<usize> {
    fn to_range(&self, _len: usize) -> Range<usize> {
        let Self { end } = *self;
        0..end + 1
    }
}

impl SliceBounds for RangeFull {
    fn to_range(&self, len: usize) -> Range<usize> {
        0..len
    }
}

/// A majority of our slice ranges are of the form `[start..][..len]`.
/// This is easy to express with normal slices where we can do the slicing multiple times,
/// but with [`DisjointMut`], that's harder, so this adds support for
/// `.index((start.., ..len))` to achieve the same.
/// It's not as clear what it means initially, but we use this idiom so much
/// I think it might be worth it for clarity through brevity.
impl SliceBounds for (RangeFrom<usize>, RangeTo<usize>) {
    fn to_range(&self, _len: usize) -> Range<usize> {
        let (RangeFrom { start }, RangeTo { end: range_len }) = *self;
        start..start + range_len
    }
}

impl<T> DisjointMutIndex<[T]> for usize {
    type Output = <[T] as Index<usize>>::Output;

    #[inline] // Inline to see bounds checks in order to potentially elide them.
    #[cfg_attr(debug_assertions, track_caller)]
    unsafe fn get_mut(self, slice: *mut [T]) -> *mut Self::Output {
        let index = self;
        let len = slice.len();
        if index < len {
            // SAFETY: We have checked that `self` is less than the allocation
            // length therefore cannot overflow. `slice` is a valid pointer into
            // an allocation of sufficient length.
            unsafe { (slice as *mut T).add(index) }
        } else {
            #[inline(never)]
            #[cfg_attr(debug_assertions, track_caller)]
            fn out_of_bounds(index: usize, len: usize) -> ! {
                panic!("index out of bounds: the len is {len} but the index is {index}")
            }
            out_of_bounds(index, len);
        }
    }
}

impl<T, I> DisjointMutIndex<[T]> for I
where
    I: SliceBounds,
{
    type Output = <[T] as Index<Range<usize>>>::Output;

    #[inline] // Inline to see bounds checks in order to potentially elide them.
    #[cfg_attr(debug_assertions, track_caller)]
    unsafe fn get_mut(self, slice: *mut [T]) -> *mut Self::Output {
        let len = slice.len();
        let Range { start, end } = self.to_range(len);
        if start <= end && end <= len {
            // SAFETY: We have checked that `start` is less than the
            // allocation length therefore cannot overflow. `slice` is a
            // valid pointer into an allocation of sufficient length.
            let data = unsafe { (slice as *mut T).add(start) };
            ptr::slice_from_raw_parts_mut(data, end - start)
        } else {
            #[inline(never)]
            #[cfg_attr(debug_assertions, track_caller)]
            fn out_of_bounds(start: usize, end: usize, len: usize) -> ! {
                if start > end {
                    panic!("slice index starts at {start} but ends at {end}");
                }
                if end > len {
                    panic!("range end index {end} out of range for slice of length {len}");
                }
                unreachable!();
            }
            out_of_bounds(start, end, len);
        }
    }
}

#[cfg(not(debug_assertions))]
mod release {
    use super::*;

    impl<'a, T: ?Sized + AsMutPtr, V: ?Sized> DisjointMutGuard<'a, T, V> {
        pub fn new(_parent: &'a DisjointMut<T>, slice: &'a mut V, _bounds: Bounds) -> Self {
            Self {
                slice,
                phantom: PhantomData,
            }
        }
    }

    impl<'a, T: ?Sized + AsMutPtr, V: ?Sized> DisjointImmutGuard<'a, T, V> {
        pub fn new(_parent: &'a DisjointMut<T>, slice: &'a V, _bounds: Bounds) -> Self {
            Self {
                slice,
                phantom: PhantomData,
            }
        }
    }
}

#[cfg(debug_assertions)]
mod debug {
    use super::*;
    use parking_lot::Mutex;
    use std::backtrace::Backtrace;
    use std::backtrace::BacktraceStatus;
    use std::fmt::Debug;
    use std::panic::Location;
    use std::thread;
    use std::thread::ThreadId;

    #[derive(Debug)]
    pub(super) struct DisjointMutBounds {
        bounds: Bounds,
        mutable: bool,
        location: &'static Location<'static>,
        backtrace: Backtrace,
        thread: ThreadId,
    }

    impl Default for DisjointMutBounds {
        fn default() -> Self {
            Self {
                bounds: Default::default(),
                mutable: Default::default(),
                location: Location::caller(),
                backtrace: Backtrace::disabled(),
                thread: thread::current().id(),
            }
        }
    }

    impl PartialEq for DisjointMutBounds {
        fn eq(&self, other: &Self) -> bool {
            self.bounds == other.bounds
                && self.mutable == other.mutable
                && self.location == other.location
                && self.thread == other.thread
        }
    }

    impl DisjointMutBounds {
        #[track_caller]
        pub fn new(bounds: Bounds, mutable: bool) -> Self {
            Self {
                bounds,
                mutable,
                location: Location::caller(),
                backtrace: Backtrace::capture(),
                thread: thread::current().id(),
            }
        }

        pub fn check_overlaps(&self, existing: &Self) {
            if !self.bounds.overlaps(&existing.bounds) {
                return;
            }
            // Example:
            //
            //         overlapping DisjointMut:
            //  current: &mut _[0..2] on ThreadId(2) at src/disjoint_mut.rs:855:24
            // existing:    & _[0..1] on ThreadId(2) at src/disjoint_mut.rs:854:24
            panic!("\toverlapping DisjointMut:\n current: {self}\nexisting: {existing}");
        }
    }

    impl Display for DisjointMutBounds {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let Self {
                bounds,
                mutable,
                location,
                backtrace,
                thread,
            } = self;
            let mutable = if *mutable { "&mut" } else { "   &" };
            write!(f, "{mutable} _[{bounds}] on {thread:?} at {location}")?;
            if backtrace.status() == BacktraceStatus::Captured {
                write!(f, ":\nstack backtrace:\n{backtrace}")?;
            }
            Ok(())
        }
    }

    #[derive(Default)]
    pub struct DisjointMutAllBounds {
        mutable: Mutex<Vec<DisjointMutBounds>>,

        immutable: Mutex<Vec<DisjointMutBounds>>,
    }

    impl DisjointMutAllBounds {
        pub const fn new() -> Self {
            Self {
                mutable: Mutex::new(Vec::new()),
                immutable: Mutex::new(Vec::new()),
            }
        }
    }

    impl<T: ?Sized + AsMutPtr> DisjointMut<T> {
        #[track_caller]
        fn add_mut_bounds(&self, current: DisjointMutBounds) {
            for existing in self.bounds.immutable.lock().iter() {
                current.check_overlaps(existing);
            }
            let mut mut_bounds = self.bounds.mutable.lock();
            for existing in mut_bounds.iter() {
                current.check_overlaps(existing);
            }
            mut_bounds.push(current);
        }

        #[track_caller]
        fn add_immut_bounds(&self, current: DisjointMutBounds) {
            let mut_bounds = self.bounds.mutable.lock();
            for existing in mut_bounds.iter() {
                current.check_overlaps(existing);
            }
            self.bounds.immutable.lock().push(current);
        }

        fn remove_bound(&self, bounds: &DisjointMutBounds) {
            let mut all_bounds = if bounds.mutable {
                self.bounds.mutable.lock()
            } else {
                self.bounds.immutable.lock()
            };
            let idx = all_bounds
                .iter()
                .position(|r| r == bounds)
                .expect("Expected range {range:?} to be in the active ranges");
            all_bounds.remove(idx);
        }
    }

    impl<'a, T: ?Sized + AsMutPtr, V: ?Sized> DisjointMutGuard<'a, T, V> {
        #[track_caller]
        pub fn new(parent: &'a DisjointMut<T>, slice: &'a mut V, bounds: Bounds) -> Self {
            parent.add_mut_bounds(DisjointMutBounds::new(bounds.clone(), true));
            let bounds = DisjointMutBounds::new(bounds, true);
            Self {
                parent,
                slice,
                bounds,
                phantom: PhantomData,
            }
        }
    }

    impl<'a, T: ?Sized + AsMutPtr, V: ?Sized> Drop for DisjointMutGuard<'a, T, V> {
        fn drop(&mut self) {
            self.parent.remove_bound(&self.bounds);
        }
    }

    impl<'a, T: ?Sized + AsMutPtr, V: ?Sized> DisjointImmutGuard<'a, T, V> {
        #[track_caller]
        pub fn new(parent: &'a DisjointMut<T>, slice: &'a V, bounds: Bounds) -> Self {
            parent.add_immut_bounds(DisjointMutBounds::new(bounds.clone(), false));
            let bounds = DisjointMutBounds::new(bounds, false);
            Self {
                parent,
                slice,
                bounds,
                phantom: PhantomData,
            }
        }
    }

    impl<'a, T: ?Sized + AsMutPtr, V: ?Sized> Drop for DisjointImmutGuard<'a, T, V> {
        fn drop(&mut self) {
            self.parent.remove_bound(&self.bounds);
        }
    }
}

impl<V: Clone> DisjointMut<Vec<V>> {
    pub fn resize(&mut self, new_len: usize, value: V) {
        self.inner.get_mut().resize(new_len, value)
    }
}

impl<V> DisjointMut<Vec<V>> {
    pub fn clear(&mut self) {
        self.inner.get_mut().clear()
    }

    pub fn resize_with<F>(&mut self, new_len: usize, f: F)
    where
        F: FnMut() -> V,
    {
        self.inner.get_mut().resize_with(new_len, f)
    }
}

/// SAFETY: We never materialize a `&mut [V]` since we
/// only materialize a `&mut Vec<V>` and call [`Vec::as_mut_ptr`] on it,
/// which never materializes a `&mut [V]`.
unsafe impl<V> AsMutPtr for Vec<V> {
    type Target = V;

    unsafe fn as_mut_ptr(ptr: *mut Self) -> *mut Self::Target {
        // SAFETY: Mutably dereferencing and calling `.as_mut_ptr()` does not
        // materialize a mutable reference to the underlying slice according to
        // its documentated behavior, so we can still allow concurrent immutable
        // references into that underlying slice.
        unsafe { (*ptr).as_mut_ptr() }
    }

    fn len(&self) -> usize {
        self.len()
    }
}

/// SAFETY: We never materialize a `&mut [V]` since we do a direct cast.
unsafe impl<V, const N: usize> AsMutPtr for [V; N] {
    type Target = V;

    unsafe fn as_mut_ptr(ptr: *mut Self) -> *mut V {
        ptr.cast()
    }

    fn len(&self) -> usize {
        N
    }
}

/// SAFETY: We never materialize a `&mut [V]` since we do a direct unsizing cast.
unsafe impl<V> AsMutPtr for [V] {
    type Target = V;

    unsafe fn as_mut_ptr(ptr: *mut Self) -> *mut Self::Target {
        ptr.cast()
    }

    fn len(&self) -> usize {
        self.len()
    }
}

/// SAFETY: We never materialize a `&mut [V]` since we go use [`addr_of_mut!`]
/// to create a `*mut [V]` directly, which we then unsize cast.
unsafe impl<V> AsMutPtr for Box<[V]> {
    type Target = V;

    unsafe fn as_mut_ptr(ptr: *mut Self) -> *mut Self::Target {
        // SAFETY: `AsMutPtr::as_mut_ptr` may derefence `ptr`.
        unsafe { addr_of_mut!(**ptr) }.cast()
    }

    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<V: Copy, C: AlignedByteChunk> DisjointMut<AlignedVec<V, C>> {
    pub fn resize(&mut self, new_len: usize, value: V) {
        self.inner.get_mut().resize(new_len, value)
    }
}

#[test]
fn test_overlapping_immut() {
    let mut v: DisjointMut<Vec<u8>> = Default::default();
    v.resize(10, 0u8);

    let guard1 = v.index(0..5);
    let guard2 = v.index(2..);

    assert_eq!(guard1[2], guard2[0]);
}

#[test]
#[cfg_attr(debug_assertions, should_panic)]
fn test_overlapping_mut() {
    let mut v: DisjointMut<Vec<u8>> = Default::default();
    v.resize(10, 0u8);

    let guard1 = v.index(0..5);
    let mut guard2 = v.index_mut(2..);

    guard2[0] = 42;
    assert_eq!(guard1[2], 42);
}

#[allow(clippy::undocumented_unsafe_blocks)]
#[cfg(debug_assertions)]
#[test]
fn test_pointer_write_debug() {
    let mut v: DisjointMut<Vec<[u8; 4]>> = Default::default();
    v.resize(10, [0u8; 4]);

    let guard = v.index(0..);
    let borrow = &guard[..];
    let ptr = v.as_mut_ptr().wrapping_offset(3) as *mut u8;
    unsafe {
        ptr.wrapping_offset(2).write(42);
    }

    // Miri complains if we dereference the guard after the pointer write here
    // instead of dereferencing and borrowing the slice before the write. I
    // think this is a Miri limitation although to be safe we might want to
    // change how we store the borrow in the guard. This is not an issue in
    // release mode without the guards (see test_pointer_write_release below).
    //
    // assert_eq!(guard[4][0], 0);

    assert_eq!(borrow[4][0], 0);
}

// Run with miri using the following command:
// RUSTFLAGS="-C debug-assertions=off" cargo miri test
#[allow(clippy::undocumented_unsafe_blocks)]
#[cfg(not(debug_assertions))]
#[test]
fn test_pointer_write_release() {
    let mut v: DisjointMut<Vec<[u8; 4]>> = Default::default();
    v.resize(10, [0u8; 4]);

    let borrow = v.index(0..);
    let ptr = v.as_mut_ptr().wrapping_offset(3) as *mut u8;
    unsafe {
        ptr.wrapping_offset(2).write(42);
    }
    assert_eq!(borrow[4][0], 0);

    // The following triggers UB because the access is through a borrow active
    // during the write.
    //
    // assert_eq!(borrow[3][2], 0);

    // We are fine to re-borrow at this point now that the write is done.
    assert_eq!(v.index(4)[0], 0);
    assert_eq!(v.index(3)[2], 42);
}

#[test]
fn test_range_overlap() {
    fn overlaps(a: impl Into<Bounds>, b: impl Into<Bounds>) -> bool {
        let a = a.into();
        let b = b.into();
        a.overlaps(&b)
    }

    // Range overlap.
    assert!(overlaps(5..7, 4..10));
    assert!(overlaps(4..10, 5..7));

    // RangeFrom overlap.
    assert!(overlaps(5.., 4..10));
    assert!(overlaps(4..10, 5..));

    // RangeTo overlap.
    assert!(overlaps(..7, 4..10));
    assert!(overlaps(4..10, ..7));

    // RangeInclusive overlap.
    assert!(overlaps(5..=7, 7..10));
    assert!(overlaps(7..10, 5..=7));

    // RangeToInclusive overlap.
    assert!(overlaps(..=7, 7..10));
    assert!(overlaps(7..10, ..=7));

    // Range no overlap.
    assert!(!overlaps(5..7, 10..20));
    assert!(!overlaps(10..20, 5..7));

    // RangeFrom no overlap.
    assert!(!overlaps(15.., 4..10));
    assert!(!overlaps(4..10, 15..));

    // RangeTo no overlap.
    assert!(!overlaps(..7, 10..20));
    assert!(!overlaps(10..20, ..7));

    // RangeInclusive no overlap.
    assert!(!overlaps(5..=7, 8..10));
    assert!(!overlaps(8..10, 5..=7));

    // RangeToInclusive no overlap.
    assert!(!overlaps(..=7, 8..10));
    assert!(!overlaps(8..10, ..=7));
}

#[cfg(debug_assertions)]
pub type DisjointMutSlice<T> = DisjointMut<Box<[T]>>;

#[cfg(not(debug_assertions))]
pub type DisjointMutSlice<T> = DisjointMut<[T]>;

/// A wrapper around an [`Arc`] of a [`DisjointMut`] slice.
/// An `Arc<[_]>` can be created, but adding a [`DisjointMut`] in between complicates it.
/// When `#[cfg(not(debug_assertions))]`, [`DisjointMut`] is `#[repr(transparent)]`
/// around an [`UnsafeCell`], which is also `#[repr(transparent)]`,
/// so we can just [`std::mem::transmute`] things.
/// But when `#[cfg(debug_assertions)]`, [`DisjointMut`] has other fields,
/// so we can't do this, so we add a [`Box`] around the slice.
/// Adding this extra allocation and indirection is not ideal,
/// which is why it's useful to avoid it in release builds.
/// In debug builds, the overhead is fine.
/// And because `Box` implements `Deref`,
/// we can treat them the same for the most part.
#[derive(Clone)]
pub struct DisjointMutArcSlice<T> {
    pub inner: Arc<DisjointMutSlice<T>>,
}

impl<T> FromIterator<T> for DisjointMutArcSlice<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        #[cfg(debug_assertions)]
        let inner = {
            let box_slice = iter.into_iter().collect::<Box<[_]>>();
            Arc::new(DisjointMut::new(box_slice))
        };
        #[cfg(not(debug_assertions))]
        let inner = {
            use std::mem;

            let arc_slice = iter.into_iter().collect::<Arc<[_]>>();

            // Do our best to check that `DisjointMut` is in fact `#[repr(transparent)]`.
            const {
                type A = Vec<u8>; // Some concrete sized type.
                assert!(mem::size_of::<DisjointMut<A>>() == mem::size_of::<A>());
                assert!(mem::align_of::<DisjointMut<A>>() == mem::align_of::<A>());
            }

            // SAFETY: When `#[cfg(not(debug_assertions))]`, `DisjointMut` is `#[repr(transparent)]`,
            // containing only an `UnsafeCell`, which is also `#[repr(transparent)]`.
            unsafe { Arc::from_raw(Arc::into_raw(arc_slice) as *const DisjointMut<[_]>) }
        };
        Self { inner }
    }
}

impl<T> Default for DisjointMutArcSlice<T> {
    fn default() -> Self {
        [].into_iter().collect()
    }
}
