//! Wrapper that allows concurrent, disjoint mutation of a slice-like owned
//! structure.

#![deny(unsafe_op_in_unsafe_fn)]
// TODO(SJC): Remove when we use the whole module.
#![allow(unused)]

use std::cell::UnsafeCell;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
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
use std::ptr::addr_of_mut;

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
pub struct DisjointMut<T: ?Sized + AsMutPtr> {
    #[cfg(debug_assertions)]
    bounds: debug::DisjointMutAllBounds,

    inner: UnsafeCell<T>,
}

impl<T: AsMutPtr> DisjointMut<T> {
    pub const fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(value),
            #[cfg(debug_assertions)]
            bounds: debug::DisjointMutAllBounds::new(),
        }
    }
}

#[cfg_attr(not(debug_assertions), repr(transparent))]
pub struct DisjointMutGuard<'a, T: ?Sized + AsMutPtr, V: ?Sized> {
    slice: &'a mut V,

    phantom: PhantomData<&'a DisjointMut<T>>,

    #[cfg(debug_assertions)]
    parent: &'a DisjointMut<T>,
    #[cfg(debug_assertions)]
    bounds: Bounds,
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
    bounds: Bounds,
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
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn index_mut<'a, I>(
        &'a self,
        index: I,
    ) -> DisjointMutGuard<'a, T, <[<T as AsMutPtr>::Target] as Index<I>>::Output>
    where
        I: Into<Bounds> + Clone,
        [<T as AsMutPtr>::Target]: IndexMut<I>,
        I: DisjointMutIndex<
            [<T as AsMutPtr>::Target],
            Output = <[<T as AsMutPtr>::Target] as Index<I>>::Output,
        >,
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
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn index<'a, I>(
        &'a self,
        index: I,
    ) -> DisjointImmutGuard<'a, T, <[<T as AsMutPtr>::Target] as Index<I>>::Output>
    where
        I: Into<Bounds> + Clone,
        [<T as AsMutPtr>::Target]: Index<I>,
        I: DisjointMutIndex<
            [<T as AsMutPtr>::Target],
            Output = <[<T as AsMutPtr>::Target] as Index<I>>::Output,
        >,
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

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct Bounds {
    /// A [`Range::end`]` == `[`usize::MAX`] is considered unbounded,
    /// as lengths need to be less than [`isize::MAX`] already.
    range: Range<usize>,
}

impl Display for Bounds {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Range { start, mut end } = self.range;
        write!(f, "{start}..")?;
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

impl From<Range<usize>> for Bounds {
    fn from(range: Range<usize>) -> Self {
        Self { range }
    }
}

impl From<RangeFrom<usize>> for Bounds {
    fn from(range: RangeFrom<usize>) -> Self {
        Self {
            range: range.start..usize::MAX,
        }
    }
}

impl From<RangeInclusive<usize>> for Bounds {
    fn from(range: RangeInclusive<usize>) -> Self {
        Self {
            range: *range.start()..*range.end() + 1,
        }
    }
}

impl From<RangeTo<usize>> for Bounds {
    fn from(range: RangeTo<usize>) -> Self {
        Self {
            range: 0..range.end,
        }
    }
}

impl From<RangeToInclusive<usize>> for Bounds {
    fn from(range: RangeToInclusive<usize>) -> Self {
        Self {
            range: 0..range.end + 1,
        }
    }
}

trait SliceBounds: Into<Bounds> + Clone + Debug {}

impl SliceBounds for Range<usize> {}
impl SliceBounds for RangeFrom<usize> {}
impl SliceBounds for RangeInclusive<usize> {}
impl SliceBounds for RangeTo<usize> {}
impl SliceBounds for RangeToInclusive<usize> {}

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

impl<T, I> DisjointMutIndex<[T]> for I
where
    I: SliceBounds,
{
    type Output = <[T] as Index<Range<usize>>>::Output;

    unsafe fn get_mut(self, slice: *mut [T]) -> *mut Self::Output {
        // SAFETY: The safety precondition for this trait method
        // requires that we can immutably dereference `slice`.
        let len = unsafe { (*slice).len() };
        let Range { start, end } = self.clone().into().range;
        let end = if end == usize::MAX { len } else { end };
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
    use std::backtrace::Backtrace;
    use std::fmt::Debug;
    use std::ops::Bound;
    use std::sync::Mutex;
    use std::thread;
    use std::thread::ThreadId;

    #[derive(Debug)]
    struct DisjointMutBounds {
        bounds: Bounds,

        #[allow(unused)]
        backtrace: Backtrace,
        #[allow(unused)]
        thread: ThreadId,
    }

    impl DisjointMutBounds {
        fn new(bounds: Bounds) -> Self {
            Self {
                bounds,
                backtrace: Backtrace::capture(),
                thread: thread::current().id(),
            }
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

    #[track_caller]
    fn check_overlaps(
        current_bounds: &Bounds,
        current_mutable: bool,
        existing: &DisjointMutBounds,
        existing_mutable: bool,
    ) {
        let DisjointMutBounds {
            bounds: existing_bounds,
            backtrace: existing_backtrace,
            thread: existing_thread,
        } = existing;
        if !current_bounds.overlaps(existing_bounds) {
            return;
        }
        let current_thread = thread::current().id();
        let [current_mutable, existing_mutable] =
            [current_mutable, existing_mutable].map(|mutable| if mutable { "&mut" } else { "&" });
        // Example:
        //
        // &mut _[0..8] on ThreadId(3) overlaps with existing &mut _[0..8] on ThreadId(2):
        // stack backtrace:
        //    0: rav1d::src::disjoint_mut::debug::DisjointMutBounds::new
        //              at ./src/disjoint_mut.rs:443:28
        panic!("{current_mutable} _[{current_bounds}] on {current_thread:?} overlaps with existing {existing_mutable} _[{existing_bounds}] on {existing_thread:?}:\nstack backtrace:\n{existing_backtrace}");
    }

    impl<T: ?Sized + AsMutPtr> DisjointMut<T> {
        #[track_caller]
        fn add_mut_bounds(&self, bounds: Bounds) {
            for b in self.bounds.immutable.lock().unwrap().iter() {
                check_overlaps(&bounds, true, b, false);
            }
            let mut mut_bounds = self.bounds.mutable.lock().unwrap();
            for b in mut_bounds.iter() {
                check_overlaps(&bounds, true, b, true);
            }
            mut_bounds.push(DisjointMutBounds::new(bounds));
        }

        #[track_caller]
        fn add_immut_bounds(&self, bounds: Bounds) {
            let mut_bounds = self.bounds.mutable.lock().unwrap();
            for b in mut_bounds.iter() {
                check_overlaps(&bounds, false, b, true);
            }
            self.bounds
                .immutable
                .lock()
                .unwrap()
                .push(DisjointMutBounds::new(bounds));
        }

        fn remove_bound(&self, bounds: &Bounds, mutable: bool) {
            let all_bounds = if mutable {
                self.bounds.mutable.lock()
            } else {
                self.bounds.immutable.lock()
            };
            let Ok(mut all_bounds) = all_bounds else {
                // Another thread has panicked holding a range lock. We can't
                // remove anything.
                return;
            };
            let idx = all_bounds
                .iter()
                .position(|r| r.bounds == *bounds)
                .expect("Expected range {range:?} to be in the active ranges");
            all_bounds.remove(idx);
        }
    }

    impl<'a, T: ?Sized + AsMutPtr, V: ?Sized> DisjointMutGuard<'a, T, V> {
        #[track_caller]
        pub fn new(parent: &'a DisjointMut<T>, slice: &'a mut V, bounds: Bounds) -> Self {
            parent.add_mut_bounds(bounds.clone());
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
            self.parent.remove_bound(&self.bounds, true);
        }
    }

    impl<'a, T: ?Sized + AsMutPtr, V: ?Sized> DisjointImmutGuard<'a, T, V> {
        #[track_caller]
        pub fn new(parent: &'a DisjointMut<T>, slice: &'a V, bounds: Bounds) -> Self {
            parent.add_immut_bounds(bounds.clone());
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
            self.parent.remove_bound(&self.bounds, false);
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
        ptr.cast()
    }

    fn len(&self) -> usize {
        N
    }
}

unsafe impl<V> AsMutPtr for [V] {
    type Target = V;

    unsafe fn as_mut_ptr(ptr: *mut Self) -> *mut Self::Target {
        ptr.cast()
    }

    fn len(&self) -> usize {
        self.len()
    }
}

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
