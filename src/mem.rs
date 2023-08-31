use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn posix_memalign(
        __memptr: *mut *mut libc::c_void,
        __alignment: size_t,
        __size: size_t,
    ) -> libc::c_int;
}

use libc::pthread_mutex_destroy;
use libc::pthread_mutex_init;
use libc::pthread_mutex_lock;
use libc::pthread_mutex_t;
use libc::pthread_mutex_unlock;

#[repr(C)]
pub struct Dav1dMemPool {
    pub lock: pthread_mutex_t,
    pub buf: *mut Dav1dMemPoolBuffer,
    pub ref_cnt: libc::c_int,
    pub end: libc::c_int,
}
#[repr(C)]
pub struct Dav1dMemPoolBuffer {
    pub data: *mut libc::c_void,
    pub next: *mut Dav1dMemPoolBuffer,
}
use libc::pthread_mutexattr_t;

#[inline]
pub unsafe extern "C" fn dav1d_alloc_aligned(sz: size_t, align: size_t) -> *mut libc::c_void {
    if align & align.wrapping_sub(1) != 0 {
        unreachable!();
    }
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if posix_memalign(&mut ptr, align, sz) != 0 {
        return 0 as *mut libc::c_void;
    }
    return ptr;
}

#[inline]
pub unsafe extern "C" fn dav1d_free_aligned(ptr: *mut libc::c_void) {
    free(ptr);
}

#[inline]
pub unsafe extern "C" fn dav1d_freep_aligned(ptr: *mut libc::c_void) {
    let mem: *mut *mut libc::c_void = ptr as *mut *mut libc::c_void;
    if !(*mem).is_null() {
        dav1d_free_aligned(*mem);
        *mem = 0 as *mut libc::c_void;
    }
}

#[inline]
pub unsafe extern "C" fn freep(ptr: *mut libc::c_void) {
    let mem: *mut *mut libc::c_void = ptr as *mut *mut libc::c_void;
    if !(*mem).is_null() {
        free(*mem);
        *mem = 0 as *mut libc::c_void;
    }
}

#[cold]
unsafe extern "C" fn mem_pool_destroy(pool: *mut Dav1dMemPool) {
    pthread_mutex_destroy(&mut (*pool).lock);
    free(pool as *mut libc::c_void);
}

pub unsafe fn dav1d_mem_pool_push(pool: *mut Dav1dMemPool, buf: *mut Dav1dMemPoolBuffer) {
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
        dav1d_free_aligned((*buf).data);
        if ref_cnt == 0 {
            mem_pool_destroy(pool);
        }
    };
}

pub unsafe fn dav1d_mem_pool_pop(pool: *mut Dav1dMemPool, size: size_t) -> *mut Dav1dMemPoolBuffer {
    if size & ::core::mem::size_of::<*mut libc::c_void>().wrapping_sub(1) != 0 {
        unreachable!();
    }
    pthread_mutex_lock(&mut (*pool).lock);
    let mut buf: *mut Dav1dMemPoolBuffer = (*pool).buf;
    (*pool).ref_cnt += 1;
    let mut data: *mut uint8_t;
    if !buf.is_null() {
        (*pool).buf = (*buf).next;
        pthread_mutex_unlock(&mut (*pool).lock);
        data = (*buf).data as *mut uint8_t;
        if (buf as uintptr_t).wrapping_sub(data as uintptr_t) == size {
            return buf;
        }
        dav1d_free_aligned(data as *mut libc::c_void);
    } else {
        pthread_mutex_unlock(&mut (*pool).lock);
    }
    data = dav1d_alloc_aligned(
        size.wrapping_add(::core::mem::size_of::<Dav1dMemPoolBuffer>()),
        64,
    ) as *mut uint8_t;
    if data.is_null() {
        pthread_mutex_lock(&mut (*pool).lock);
        (*pool).ref_cnt -= 1;
        let ref_cnt = (*pool).ref_cnt;
        pthread_mutex_unlock(&mut (*pool).lock);
        if ref_cnt == 0 {
            mem_pool_destroy(pool);
        }
        return 0 as *mut Dav1dMemPoolBuffer;
    }
    buf = data.offset(size as isize) as *mut Dav1dMemPoolBuffer;
    (*buf).data = data as *mut libc::c_void;
    return buf;
}

#[cold]
pub unsafe fn dav1d_mem_pool_init(ppool: *mut *mut Dav1dMemPool) -> libc::c_int {
    let pool: *mut Dav1dMemPool =
        malloc(::core::mem::size_of::<Dav1dMemPool>() as libc::c_ulong) as *mut Dav1dMemPool;
    if !pool.is_null() {
        if pthread_mutex_init(&mut (*pool).lock, 0 as *const pthread_mutexattr_t) == 0 {
            (*pool).buf = 0 as *mut Dav1dMemPoolBuffer;
            (*pool).ref_cnt = 1 as libc::c_int;
            (*pool).end = 0 as libc::c_int;
            *ppool = pool;
            return 0 as libc::c_int;
        }
        free(pool as *mut libc::c_void);
    }
    *ppool = 0 as *mut Dav1dMemPool;
    return -(12 as libc::c_int);
}

#[cold]
pub unsafe fn dav1d_mem_pool_end(pool: *mut Dav1dMemPool) {
    if !pool.is_null() {
        pthread_mutex_lock(&mut (*pool).lock);
        let mut buf: *mut Dav1dMemPoolBuffer = (*pool).buf;
        (*pool).ref_cnt -= 1;
        let ref_cnt = (*pool).ref_cnt;
        (*pool).buf = 0 as *mut Dav1dMemPoolBuffer;
        (*pool).end = 1 as libc::c_int;
        pthread_mutex_unlock(&mut (*pool).lock);
        while !buf.is_null() {
            let data: *mut libc::c_void = (*buf).data;
            buf = (*buf).next;
            dav1d_free_aligned(data);
        }
        if ref_cnt == 0 {
            mem_pool_destroy(pool);
        }
    }
}
