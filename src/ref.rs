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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dRef {
    pub data: *mut libc::c_void,
    pub const_data: *const libc::c_void,
    pub ref_cnt: atomic_int,
    pub free_ref: libc::c_int,
    pub free_callback: Option<unsafe extern "C" fn(*const uint8_t, *mut libc::c_void) -> ()>,
    pub user_data: *mut libc::c_void,
}
use crate::include::stdatomic::atomic_int;

use crate::src::mem::dav1d_alloc_aligned;
use crate::src::mem::dav1d_free_aligned;
use crate::src::mem::dav1d_mem_pool_pop;
use crate::src::mem::dav1d_mem_pool_push;
use crate::src::mem::Dav1dMemPool;
use crate::src::mem::Dav1dMemPoolBuffer;

#[inline]
pub unsafe extern "C" fn dav1d_ref_inc(r#ref: *mut Dav1dRef) {
    ::core::intrinsics::atomic_xadd_relaxed(&mut (*r#ref).ref_cnt, 1 as libc::c_int);
}

unsafe extern "C" fn default_free_callback(data: *const uint8_t, user_data: *mut libc::c_void) {
    if !(data == user_data as *const uint8_t) {
        unreachable!();
    }
    dav1d_free_aligned(user_data);
}

pub unsafe fn dav1d_ref_create(mut size: size_t) -> *mut Dav1dRef {
    size = size
        .wrapping_add(::core::mem::size_of::<*mut libc::c_void>())
        .wrapping_sub(1)
        & !(::core::mem::size_of::<*mut libc::c_void>()).wrapping_sub(1);
    let data: *mut uint8_t = dav1d_alloc_aligned(
        size.wrapping_add(::core::mem::size_of::<Dav1dRef>()),
        64 as libc::c_int as size_t,
    ) as *mut uint8_t;
    if data.is_null() {
        return 0 as *mut Dav1dRef;
    }
    let res: *mut Dav1dRef = data.offset(size as isize) as *mut Dav1dRef;
    (*res).data = data as *mut libc::c_void;
    (*res).user_data = (*res).data;
    (*res).const_data = (*res).user_data;
    *&mut (*res).ref_cnt = 1 as libc::c_int;
    (*res).free_ref = 0 as libc::c_int;
    (*res).free_callback = Some(
        default_free_callback as unsafe extern "C" fn(*const uint8_t, *mut libc::c_void) -> (),
    );
    return res;
}
unsafe extern "C" fn pool_free_callback(data: *const uint8_t, user_data: *mut libc::c_void) {
    dav1d_mem_pool_push(
        data as *mut Dav1dMemPool,
        user_data as *mut Dav1dMemPoolBuffer,
    );
}

pub unsafe fn dav1d_ref_create_using_pool(
    pool: *mut Dav1dMemPool,
    mut size: size_t,
) -> *mut Dav1dRef {
    size = size
        .wrapping_add(::core::mem::size_of::<*mut libc::c_void>())
        .wrapping_sub(1)
        & !(::core::mem::size_of::<*mut libc::c_void>()).wrapping_sub(1);
    let buf: *mut Dav1dMemPoolBuffer =
        dav1d_mem_pool_pop(pool, size.wrapping_add(::core::mem::size_of::<Dav1dRef>()));
    if buf.is_null() {
        return 0 as *mut Dav1dRef;
    }
    let res: *mut Dav1dRef =
        &mut *(buf as *mut Dav1dRef).offset(-(1 as libc::c_int) as isize) as *mut Dav1dRef;
    (*res).data = (*buf).data;
    (*res).const_data = pool as *const libc::c_void;
    *&mut (*res).ref_cnt = 1 as libc::c_int;
    (*res).free_ref = 0 as libc::c_int;
    (*res).free_callback =
        Some(pool_free_callback as unsafe extern "C" fn(*const uint8_t, *mut libc::c_void) -> ());
    (*res).user_data = buf as *mut libc::c_void;
    return res;
}

pub unsafe fn dav1d_ref_wrap(
    ptr: *const uint8_t,
    mut free_callback: Option<unsafe extern "C" fn(*const uint8_t, *mut libc::c_void) -> ()>,
    user_data: *mut libc::c_void,
) -> *mut Dav1dRef {
    let mut res: *mut Dav1dRef =
        malloc(::core::mem::size_of::<Dav1dRef>() as libc::c_ulong) as *mut Dav1dRef;
    if res.is_null() {
        return 0 as *mut Dav1dRef;
    }
    (*res).data = 0 as *mut libc::c_void;
    (*res).const_data = ptr as *const libc::c_void;
    *&mut (*res).ref_cnt = 1 as libc::c_int;
    (*res).free_ref = 1 as libc::c_int;
    (*res).free_callback = free_callback;
    (*res).user_data = user_data;
    return res;
}

pub unsafe fn dav1d_ref_dec(pref: *mut *mut Dav1dRef) {
    if pref.is_null() {
        unreachable!();
    }
    let r#ref: *mut Dav1dRef = *pref;
    if r#ref.is_null() {
        return;
    }
    *pref = 0 as *mut Dav1dRef;
    if ::core::intrinsics::atomic_xsub_seqcst(
        &mut (*r#ref).ref_cnt as *mut atomic_int,
        1 as libc::c_int,
    ) == 1
    {
        let free_ref = (*r#ref).free_ref;
        ((*r#ref).free_callback).expect("non-null function pointer")(
            (*r#ref).const_data as *const uint8_t,
            (*r#ref).user_data,
        );
        if free_ref != 0 {
            free(r#ref as *mut libc::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_ref_is_writable(r#ref: *mut Dav1dRef) -> libc::c_int {
    return (::core::intrinsics::atomic_load_seqcst(&mut (*r#ref).ref_cnt as *mut atomic_int) == 1
        && !((*r#ref).data).is_null()) as libc::c_int;
}
