//! Wrapper that allows concurrent, disjoint mutation of a slice-like owned
//! structure.

#![deny(unsafe_op_in_unsafe_fn)]
// TODO(SJC): Remove when we use the whole module.
#![allow(unused)]

use std::cell::UnsafeCell;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Bound;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Range;
use std::ops::RangeBounds;
use std::ops::RangeFrom;
use std::ops::RangeInclusive;
use std::ops::RangeTo;
use std::ops::RangeToInclusive;
use std::ptr;

use crate::src::align::AlignedByteChunk;
use crate::src::align::AlignedVec;

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
pub struct DisjointMut<T: AsMutPtr> {
    inner: UnsafeCell<T>,

    #[cfg(debug_assertions)]
    ranges: debug::DisjointMutDebugRanges,
}

#[cfg_attr(not(debug_assertions), repr(transparent))]
pub struct DisjointMutGuard<'a, T: AsMutPtr, V: ?Sized> {
    slice: &'a mut V,

    phantom: PhantomData<&'a DisjointMut<T>>,

    #[cfg(debug_assertions)]
    parent: &'a DisjointMut<T>,
    #[cfg(debug_assertions)]
    range: (Bound<usize>, Bound<usize>),
}

impl<'a, T: AsMutPtr, V: ?Sized> Deref for DisjointMutGuard<'a, T, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.slice
    }
}

impl<'a, T: AsMutPtr, V: ?Sized> DerefMut for DisjointMutGuard<'a, T, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.slice
    }
}

#[cfg_attr(not(debug_assertions), repr(transparent))]
pub struct DisjointImmutGuard<'a, T: AsMutPtr, V: ?Sized> {
    slice: &'a V,

    phantom: PhantomData<&'a DisjointMut<T>>,

    #[cfg(debug_assertions)]
    parent: &'a DisjointMut<T>,
    #[cfg(debug_assertions)]
    range: (Bound<usize>, Bound<usize>),
}

impl<'a, T: AsMutPtr, V: ?Sized> Deref for DisjointImmutGuard<'a, T, V> {
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
pub unsafe trait AsMutPtr: Sized {
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

impl<T: AsMutPtr> DisjointMut<T> {
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
        &mut *self.inner.get_mut()
    }

    /// Mutably borrow a slice or element.
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
    pub unsafe fn index_mut<'a, I>(
        &'a self,
        index: I,
    ) -> DisjointMutGuard<'a, T, <[<T as AsMutPtr>::Target] as Index<I>>::Output>
    where
        [<T as AsMutPtr>::Target]: IndexMut<I>,
        I: DisjointMutIndex<
            [<T as AsMutPtr>::Target],
            Output = <[<T as AsMutPtr>::Target] as Index<I>>::Output,
        >,
    {
        let range = index.to_bounds();
        // SAFETY: The safety preconditions of `index` and `index_mut` imply
        // that the indexed region we are mutably borrowing is not concurrently
        // borrowed and will not be borrowed during the lifetime of the returned
        // reference.
        let slice = unsafe { &mut *index.get_mut(self.as_mut_slice()) };
        DisjointMutGuard::new(self, slice, range)
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
    pub fn index<'a, I>(
        &'a self,
        index: I,
    ) -> DisjointImmutGuard<'a, T, <[<T as AsMutPtr>::Target] as Index<I>>::Output>
    where
        [<T as AsMutPtr>::Target]: Index<I>,
        I: DisjointMutIndex<
            [<T as AsMutPtr>::Target],
            Output = <[<T as AsMutPtr>::Target] as Index<I>>::Output,
        >,
    {
        let range = index.to_bounds();
        // SAFETY: The safety preconditions of `index` and `index_mut` imply
        // that the indexed region we are immutably borrowing is not
        // concurrently mutably borrowed and will not be mutably borrowed during
        // the lifetime of the returned reference.
        let slice = unsafe { &*index.get_mut(self.as_mut_slice()).cast_const() };
        DisjointImmutGuard::new(self, slice, range)
    }
}

/// This trait is a stable implementation of [`std::slice::SliceIndex`] to allow
/// for indexing into mutable slice raw pointers.
pub trait DisjointMutIndex<T: ?Sized>: BoundsExt {
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

pub trait BoundsExt: Debug {
    fn to_bounds(&self) -> (Bound<usize>, Bound<usize>);
    fn overlaps<R: RangeBounds<usize>>(&self, other: &R) -> bool;
}

impl BoundsExt for usize {
    fn to_bounds(&self) -> (Bound<usize>, Bound<usize>) {
        (Bound::Included(*self), Bound::Excluded(*self + 1))
    }

    fn overlaps<R: RangeBounds<usize>>(&self, other: &R) -> bool {
        other.contains(self)
    }
}

impl<T> DisjointMutIndex<[T]> for usize {
    type Output = <[T] as Index<usize>>::Output;

    unsafe fn get_mut(self, slice: *mut [T]) -> *mut Self::Output {
        // SAFETY: The safety precondition for this trait method requires that
        // we can immutably dereference `slice`.
        let len = unsafe { (*slice).len() };
        if self < len {
            // SAFETY: We have checked that `self` is less than the allocation
            // length therefore cannot overflow. `slice` is a valid pointer into
            // an allocation of sufficient length.
            unsafe { (slice as *mut T).add(self) }
        } else {
            panic!("{:?} was not a valid index", &self);
        }
    }
}

macro_rules! impl_disjoint_mut_index {
    ($range_type:ty) => {
        impl BoundsExt for $range_type {
            fn to_bounds(&self) -> (Bound<usize>, Bound<usize>) {
                (self.start_bound().cloned(), self.end_bound().cloned())
            }

            fn overlaps<R: RangeBounds<usize>>(&self, other: &R) -> bool {
                match (other.start_bound(), self.end_bound()) {
                    (Bound::Included(i), Bound::Included(j)) => {
                        if i > j {
                            return false;
                        }
                    }
                    (Bound::Included(i), Bound::Excluded(j))
                    | (Bound::Excluded(i), Bound::Excluded(j))
                    | (Bound::Excluded(i), Bound::Included(j)) => {
                        if i >= j {
                            return false;
                        }
                    }
                    (Bound::Unbounded, _) => {}
                    (_, Bound::Unbounded) => {}
                }
                match (other.end_bound(), self.start_bound()) {
                    (Bound::Included(i), Bound::Included(j)) => i >= j,
                    (Bound::Included(i), Bound::Excluded(j))
                    | (Bound::Excluded(i), Bound::Included(j)) => i > j,
                    (Bound::Excluded(i), Bound::Excluded(j)) => i - 1 > *j,
                    (Bound::Unbounded, _) | (_, Bound::Unbounded) => true,
                }
            }
        }
        impl<T> DisjointMutIndex<[T]> for $range_type {
            type Output = <[T] as Index<$range_type>>::Output;

            unsafe fn get_mut(self, slice: *mut [T]) -> *mut Self::Output {
                // SAFETY: The safety precondition for this trait method
                // requires that we can immutably dereference `slice`.
                let len = unsafe { (*slice).len() };
                let start = match self.start_bound() {
                    Bound::Included(i) => *i,
                    Bound::Excluded(i) => i + 1,
                    Bound::Unbounded => 0,
                };
                let end = match self.end_bound() {
                    Bound::Included(i) => i + 1,
                    Bound::Excluded(i) => *i,
                    Bound::Unbounded => len,
                };
                if start <= end && start < len && end <= len {
                    // SAFETY: We have checked that `start` is less than the
                    // allocation length therefore cannot overflow. `slice` is a
                    // valid pointer into an allocation of sufficient length.
                    let data = unsafe { (slice as *mut T).add(start) };
                    ptr::slice_from_raw_parts_mut(data, end - start)
                } else {
                    panic!("{:?} was not a valid index", &self);
                }
            }
        }
    };
}

impl_disjoint_mut_index!(Range<usize>);
impl_disjoint_mut_index!(RangeFrom<usize>);
impl_disjoint_mut_index!(RangeInclusive<usize>);
impl_disjoint_mut_index!(RangeTo<usize>);
impl_disjoint_mut_index!(RangeToInclusive<usize>);
impl_disjoint_mut_index!((Bound<usize>, Bound<usize>));

#[cfg(not(debug_assertions))]
mod release {
    use super::*;

    impl<'a, T: AsMutPtr, V: ?Sized> DisjointMutGuard<'a, T, V> {
        pub fn new(
            _parent: &'a DisjointMut<T>,
            slice: &'a mut V,
            _range: (Bound<usize>, Bound<usize>),
        ) -> Self {
            Self {
                slice,
                phantom: PhantomData,
            }
        }
    }

    impl<'a, T: AsMutPtr, V: ?Sized> DisjointImmutGuard<'a, T, V> {
        pub fn new(
            _parent: &'a DisjointMut<T>,
            slice: &'a V,
            _range: (Bound<usize>, Bound<usize>),
        ) -> Self {
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
    use std::backtrace::Backtrace;
    use std::fmt::Debug;
    use std::ops::Bound;
    use std::sync::Mutex;
    use std::thread;
    use std::thread::ThreadId;

    #[derive(Debug)]
    struct DisjointMutRange {
        start: Bound<usize>,
        end: Bound<usize>,

        #[allow(unused)]
        backtrace: Backtrace,
        #[allow(unused)]
        thread: ThreadId,
    }

    impl RangeBounds<usize> for DisjointMutRange {
        fn start_bound(&self) -> Bound<&usize> {
            self.start.as_ref()
        }

        fn end_bound(&self) -> Bound<&usize> {
            self.end.as_ref()
        }
    }

    impl PartialEq<(Bound<usize>, Bound<usize>)> for DisjointMutRange {
        fn eq(&self, (start, end): &(Bound<usize>, Bound<usize>)) -> bool {
            &self.start == start && &self.end == end
        }
    }

    impl DisjointMutRange {
        fn new((start, end): (Bound<usize>, Bound<usize>)) -> Self {
            Self {
                start,
                end,
                backtrace: Backtrace::capture(),
                thread: thread::current().id(),
            }
        }
    }

    #[derive(Default)]
    pub struct DisjointMutDebugRanges {
        mutable: Mutex<Vec<DisjointMutRange>>,

        immutable: Mutex<Vec<DisjointMutRange>>,
    }

    impl<T: AsMutPtr> DisjointMut<T> {
        fn add_mut_range<R: BoundsExt>(&self, range: &R) {
            for r in self.ranges.immutable.lock().unwrap().iter() {
                if range.overlaps(r) {
                    let thread = thread::current().id();
                    panic!("{range:?} on thread {thread:?} overlaps with an existing immutable range: {r:#?}");
                }
            }
            let mut ranges = self.ranges.mutable.lock().unwrap();
            for r in ranges.iter() {
                if range.overlaps(r) {
                    let thread = thread::current().id();
                    panic!("{range:?} on thread {thread:?} overlaps with an existing mutable range: {r:#?}");
                }
            }
            ranges.push(DisjointMutRange::new(range.to_bounds()));
        }

        fn add_immut_range<R: BoundsExt>(&self, range: &R) {
            let ranges = self.ranges.mutable.lock().unwrap();
            for r in ranges.iter() {
                if range.overlaps(r) {
                    let thread = thread::current().id();
                    panic!("{range:?} on thread {thread:?} overlaps with an existing mutable range: {r:#?}");
                }
            }
            self.ranges
                .immutable
                .lock()
                .unwrap()
                .push(DisjointMutRange::new(range.to_bounds()));
        }

        fn remove_range(&self, range: (Bound<usize>, Bound<usize>), mutable: bool) {
            let ranges = if mutable {
                self.ranges.mutable.lock()
            } else {
                self.ranges.immutable.lock()
            };
            let Ok(mut ranges) = ranges else {
                // Another thread has panicked holding a range lock. We can't
                // remove anything.
                return;
            };
            let idx = ranges
                .iter()
                .position(|r| r == &range)
                .expect("Expected range {range:?} to be in the active ranges");
            ranges.remove(idx);
        }
    }

    impl<'a, T: AsMutPtr, V: ?Sized> DisjointMutGuard<'a, T, V> {
        pub fn new(
            parent: &'a DisjointMut<T>,
            slice: &'a mut V,
            range: (Bound<usize>, Bound<usize>),
        ) -> Self {
            parent.add_mut_range(&range);
            Self {
                parent,
                slice,
                range,
                phantom: PhantomData,
            }
        }
    }

    impl<'a, T: AsMutPtr, V: ?Sized> Drop for DisjointMutGuard<'a, T, V> {
        fn drop(&mut self) {
            self.parent.remove_range(self.range, true);
        }
    }

    impl<'a, T: AsMutPtr, V: ?Sized> DisjointImmutGuard<'a, T, V> {
        pub fn new(
            parent: &'a DisjointMut<T>,
            slice: &'a V,
            range: (Bound<usize>, Bound<usize>),
        ) -> Self {
            parent.add_immut_range(&range);
            Self {
                parent,
                slice,
                range,
                phantom: PhantomData,
            }
        }
    }

    impl<'a, T: AsMutPtr, V: ?Sized> Drop for DisjointImmutGuard<'a, T, V> {
        fn drop(&mut self) {
            self.parent.remove_range(self.range, false);
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

unsafe impl<V, const N: usize> AsMutPtr for [V; N] {
    type Target = V;

    unsafe fn as_mut_ptr(ptr: *mut Self) -> *mut V {
        ptr as *mut V
    }

    fn len(&self) -> usize {
        N
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

    let guard1 = unsafe { v.index(0..5) };
    let guard2 = unsafe { v.index(2..) };

    assert_eq!(guard1[2], guard2[0]);
}

#[test]
#[cfg_attr(debug_assertions, should_panic)]
fn test_overlapping_mut() {
    let mut v: DisjointMut<Vec<u8>> = Default::default();
    v.resize(10, 0u8);

    let guard1 = unsafe { v.index(0..5) };
    let mut guard2 = unsafe { v.index_mut(2..) };

    guard2[0] = 42;
    assert_eq!(guard1[2], 42);
}

#[cfg(debug_assertions)]
#[test]
fn test_pointer_write_debug() {
    let mut v: DisjointMut<Vec<[u8; 4]>> = Default::default();
    v.resize(10, [0u8; 4]);

    let guard = unsafe { v.index(0..) };
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
#[cfg(not(debug_assertions))]
#[test]
fn test_pointer_write_release() {
    let mut v: DisjointMut<Vec<[u8; 4]>> = Default::default();
    v.resize(10, [0u8; 4]);

    let borrow = unsafe { v.index(0..) };
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
    assert_eq!(unsafe { v.index(4)[0] }, 0);
    assert_eq!(unsafe { v.index(3)[2] }, 42);
}

#[test]
fn test_range_overlap() {
    // Range overlap.
    assert!((5..7).overlaps(&(4..10)));
    assert!((4..10).overlaps(&(5..7)));

    // RangeFrom overlap.
    assert!((5..).overlaps(&(4..10)));
    assert!((4..10).overlaps(&(5..)));

    // RangeTo overlap.
    assert!((..7).overlaps(&(4..10)));
    assert!((4..10).overlaps(&(..7)));

    // RangeInclusive overlap.
    assert!((5..=7).overlaps(&(7..10)));
    assert!((7..10).overlaps(&(5..=7)));

    // RangeToInclusive overlap.
    assert!((..=7).overlaps(&(7..10)));
    assert!((7..10).overlaps(&(..=7)));

    // Range no overlap.
    assert!(!(5..7).overlaps(&(10..20)));
    assert!(!(10..20).overlaps(&(5..7)));

    // RangeFrom no overlap.
    assert!(!(15..).overlaps(&(4..10)));
    assert!(!(4..10).overlaps(&(15..)));

    // RangeTo no overlap.
    assert!(!(..7).overlaps(&(10..20)));
    assert!(!(10..20).overlaps(&(..7)));

    // RangeInclusive no overlap.
    assert!(!(5..=7).overlaps(&(8..10)));
    assert!(!(8..10).overlaps(&(5..=7)));

    // RangeToInclusive no overlap.
    assert!(!(..=7).overlaps(&(8..10)));
    assert!(!(8..10).overlaps(&(..=7)));
}
