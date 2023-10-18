use libc::free;
use libc::malloc;
use libc::posix_memalign;
use libc::pthread_mutex_destroy;
use libc::pthread_mutex_init;
use libc::pthread_mutex_lock;
use libc::pthread_mutex_t;
use libc::pthread_mutex_unlock;
use libc::pthread_mutexattr_t;
use libc::uintptr_t;
use std::ffi::c_int;
use std::ffi::c_void;

#[repr(C)]
pub struct Rav1dMemPool {
    pub lock: pthread_mutex_t,
    pub buf: *mut Rav1dMemPoolBuffer,
    pub ref_cnt: c_int,
    pub end: c_int,
}

#[repr(C)]
pub struct Rav1dMemPoolBuffer {
    pub data: *mut c_void,
    pub next: *mut Rav1dMemPoolBuffer,
}

#[inline]
pub unsafe extern "C" fn rav1d_alloc_aligned(sz: usize, align: usize) -> *mut c_void {
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
pub unsafe extern "C" fn rav1d_free_aligned(ptr: *mut c_void) {
    free(ptr);
}

#[inline]
pub unsafe extern "C" fn rav1d_freep_aligned(ptr: *mut c_void) {
    let mem: *mut *mut c_void = ptr as *mut *mut c_void;
    if !(*mem).is_null() {
        rav1d_free_aligned(*mem);
        *mem = 0 as *mut c_void;
    }
}

#[inline]
pub unsafe extern "C" fn freep(ptr: *mut c_void) {
    let mem: *mut *mut c_void = ptr as *mut *mut c_void;
    if !(*mem).is_null() {
        free(*mem);
        *mem = 0 as *mut c_void;
    }
}

#[cold]
unsafe extern "C" fn mem_pool_destroy(pool: *mut Rav1dMemPool) {
    pthread_mutex_destroy(&mut (*pool).lock);
    free(pool as *mut c_void);
}

pub unsafe fn rav1d_mem_pool_push(pool: *mut Rav1dMemPool, buf: *mut Rav1dMemPoolBuffer) {
    pthread_mutex_lock(&mut (*pool).lock);
    (*pool).ref_cnt -= 1;
    let ref_cnt = (*pool).ref_cnt;
    if (*pool).end == 0 {
        (*buf).next = (*pool).buf;
        (*pool).buf = buf;
        pthread_mutex_unlock(&mut (*pool).lock);
        if !(ref_cnt > 0) {
            unreachable!();
        }
    } else {
        pthread_mutex_unlock(&mut (*pool).lock);
        rav1d_free_aligned((*buf).data);
        if ref_cnt == 0 {
            mem_pool_destroy(pool);
        }
    };
}

pub unsafe fn rav1d_mem_pool_pop(pool: *mut Rav1dMemPool, size: usize) -> *mut Rav1dMemPoolBuffer {
    if size & ::core::mem::size_of::<*mut c_void>().wrapping_sub(1) != 0 {
        unreachable!();
    }
    pthread_mutex_lock(&mut (*pool).lock);
    let mut buf: *mut Rav1dMemPoolBuffer = (*pool).buf;
    (*pool).ref_cnt += 1;
    let mut data: *mut u8;
    if !buf.is_null() {
        (*pool).buf = (*buf).next;
        pthread_mutex_unlock(&mut (*pool).lock);
        data = (*buf).data as *mut u8;
        if (buf as uintptr_t).wrapping_sub(data as uintptr_t) == size {
            return buf;
        }
        rav1d_free_aligned(data as *mut c_void);
    } else {
        pthread_mutex_unlock(&mut (*pool).lock);
    }
    data = rav1d_alloc_aligned(
        size.wrapping_add(::core::mem::size_of::<Rav1dMemPoolBuffer>()),
        64,
    ) as *mut u8;
    if data.is_null() {
        pthread_mutex_lock(&mut (*pool).lock);
        (*pool).ref_cnt -= 1;
        let ref_cnt = (*pool).ref_cnt;
        pthread_mutex_unlock(&mut (*pool).lock);
        if ref_cnt == 0 {
            mem_pool_destroy(pool);
        }
        return 0 as *mut Rav1dMemPoolBuffer;
    }
    buf = data.offset(size as isize) as *mut Rav1dMemPoolBuffer;
    (*buf).data = data as *mut c_void;
    return buf;
}

#[cold]
pub unsafe fn rav1d_mem_pool_init(ppool: *mut *mut Rav1dMemPool) -> c_int {
    let pool: *mut Rav1dMemPool =
        malloc(::core::mem::size_of::<Rav1dMemPool>()) as *mut Rav1dMemPool;
    if !pool.is_null() {
        if pthread_mutex_init(&mut (*pool).lock, 0 as *const pthread_mutexattr_t) == 0 {
            (*pool).buf = 0 as *mut Rav1dMemPoolBuffer;
            (*pool).ref_cnt = 1 as c_int;
            (*pool).end = 0 as c_int;
            *ppool = pool;
            return 0 as c_int;
        }
        free(pool as *mut c_void);
    }
    *ppool = 0 as *mut Rav1dMemPool;
    return -(12 as c_int);
}

#[cold]
pub unsafe fn rav1d_mem_pool_end(pool: *mut Rav1dMemPool) {
    if !pool.is_null() {
        pthread_mutex_lock(&mut (*pool).lock);
        let mut buf: *mut Rav1dMemPoolBuffer = (*pool).buf;
        (*pool).ref_cnt -= 1;
        let ref_cnt = (*pool).ref_cnt;
        (*pool).buf = 0 as *mut Rav1dMemPoolBuffer;
        (*pool).end = 1 as c_int;
        pthread_mutex_unlock(&mut (*pool).lock);
        while !buf.is_null() {
            let data: *mut c_void = (*buf).data;
            buf = (*buf).next;
            rav1d_free_aligned(data);
        }
        if ref_cnt == 0 {
            mem_pool_destroy(pool);
        }
    }
}
