use libc::free;
use libc::posix_memalign;
use std::ffi::c_void;
use std::sync::Mutex;

pub struct MemPool<T> {
    bufs: Mutex<Vec<Vec<T>>>,
}

impl<T> MemPool<T> {
    pub const fn new() -> Self {
        Self {
            bufs: Mutex::new(Vec::new()),
        }
    }

    pub fn pop(&self, size: usize) -> Vec<T> {
        if let Some(mut buf) = self.bufs.lock().unwrap().pop() {
            if size > buf.capacity() {
                // TODO fallible allocation
                buf.reserve(size - buf.len());
            }
            return buf;
        }
        // TODO fallible allocation
        Vec::with_capacity(size)
    }

    pub fn push(&self, buf: Vec<T>) {
        self.bufs.lock().unwrap().push(buf);
    }
}

impl<T> Default for MemPool<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[inline]
pub unsafe fn rav1d_alloc_aligned(sz: usize, align: usize) -> *mut c_void {
    if align & align.wrapping_sub(1) != 0 {
        unreachable!();
    }
    let mut ptr: *mut c_void = 0 as *mut c_void;
    if posix_memalign(&mut ptr, align, sz) != 0 {
        return 0 as *mut c_void;
    }
    return ptr;
}

#[inline]
pub unsafe fn rav1d_free_aligned(ptr: *mut c_void) {
    free(ptr);
}
