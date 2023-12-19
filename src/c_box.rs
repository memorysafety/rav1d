use std::ffi::c_void;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::drop_in_place;
use std::ptr::NonNull;

pub type FnFree = unsafe extern "C" fn(ptr: *const u8, cookie: *mut c_void);

/// A `free` "closure", i.e. a [`FnFree`] and an enclosed context [`Self::cookie`].
#[derive(Debug)]
pub struct Free {
    pub free: FnFree,
    pub cookie: *mut c_void,
}

impl Free {
    pub unsafe fn free(&self, ptr: *mut c_void) {
        (self.free)(ptr as *const u8, self.cookie)
    }
}

/// A C/custom [`Box`].
///
/// That is, it is analogous to a [`Box`],
/// but it lets you set a C-style `free` `fn` for deallocation
/// instead of the normal [`Box`] (de)allocator.
/// It can also store a normal [`Box`] as well.
#[derive(Debug)]
pub struct CBox<T: ?Sized> {
    data: NonNull<T>,
    /// If [`None`], [`Self::data`] should be freed as a [`Box`].
    free: Option<Free>,
    _phantom: PhantomData<T>,
}

impl<T: ?Sized> AsRef<T> for CBox<T> {
    fn as_ref(&self) -> &T {
        unsafe { self.data.as_ref() }
    }
}

impl<T: ?Sized> Deref for CBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T: ?Sized> Drop for CBox<T> {
    fn drop(&mut self) {
        let ptr = self.data.as_ptr();
        match &self.free {
            None => {
                // Safety: If [`Self::free`] is [`None`],
                // then [`Self::data`] is a [`Box`].
                // See [`Self::from_box`].
                let _ = unsafe { Box::from_raw(ptr) };
            }
            Some(free) => {
                // Safety: See below.
                // The [`FnFree`] won't run Rust's `fn drop`,
                // so we have to do this ourselves first.
                unsafe { drop_in_place(ptr) };
                let ptr = ptr.cast();
                // Safety: If [`Self::free`] is [`Some`],
                // then that [`FnFree`] should be used for `free`ing.
                // See [`Self::from_c`].
                unsafe { free.free(ptr) }
            }
        }
    }
}

impl<T: ?Sized> CBox<T> {
    pub fn from_c(data: NonNull<T>, free: Free) -> Self {
        Self {
            data,
            free: Some(free),
            _phantom: PhantomData,
        }
    }

    pub fn from_box(data: Box<T>) -> Self {
        let data = Box::into_raw(data);
        // Safety: [`Box::into_raw`] guarantees it always returns non-null ptrs.
        let data = unsafe { NonNull::new_unchecked(data) };
        Self {
            data,
            free: None,
            _phantom: PhantomData,
        }
    }
}
