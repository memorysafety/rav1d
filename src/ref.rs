use crate::src::mem::rav1d_alloc_aligned;
use crate::src::mem::rav1d_free_aligned;
use crate::src::mem::rav1d_mem_pool_pop;
use crate::src::mem::rav1d_mem_pool_push;
use crate::src::mem::Rav1dMemPool;
use crate::src::mem::Rav1dMemPoolBuffer;
use libc::free;
use libc::malloc;
use std::ffi::c_int;
use std::ffi::c_void;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;

#[repr(C)]
pub struct Rav1dRef {
    pub(crate) data: *mut c_void,
    pub(crate) const_data: *const c_void,
    pub(crate) ref_cnt: AtomicI32,
    pub(crate) free_ref: c_int,
    pub(crate) free_callback: Option<unsafe extern "C" fn(*const u8, *mut c_void) -> ()>,
    pub(crate) user_data: *mut c_void,
}

#[inline]
pub unsafe fn rav1d_ref_inc(r#ref: *mut Rav1dRef) {
    (*r#ref).ref_cnt.fetch_add(1, Ordering::Relaxed);
}

unsafe extern "C" fn default_free_callback(data: *const u8, user_data: *mut c_void) {
    if !(data == user_data as *const u8) {
        unreachable!();
    }
    rav1d_free_aligned(user_data);
}

pub unsafe fn rav1d_ref_create(mut size: usize) -> *mut Rav1dRef {
    size = size
        .wrapping_add(::core::mem::size_of::<*mut c_void>())
        .wrapping_sub(1)
        & !(::core::mem::size_of::<*mut c_void>()).wrapping_sub(1);
    let data: *mut u8 = rav1d_alloc_aligned(
        size.wrapping_add(::core::mem::size_of::<Rav1dRef>()),
        64 as c_int as usize,
    ) as *mut u8;
    if data.is_null() {
        return 0 as *mut Rav1dRef;
    }
    let res: *mut Rav1dRef = data.offset(size as isize) as *mut Rav1dRef;
    (*res).data = data as *mut c_void;
    (*res).user_data = (*res).data;
    (*res).const_data = (*res).user_data;
    (*res).ref_cnt = AtomicI32::new(1);
    (*res).free_ref = 0 as c_int;
    (*res).free_callback = Some(default_free_callback);
    return res;
}

unsafe extern "C" fn pool_free_callback(data: *const u8, user_data: *mut c_void) {
    rav1d_mem_pool_push(
        data as *mut Rav1dMemPool,
        user_data as *mut Rav1dMemPoolBuffer,
    );
}

pub unsafe fn rav1d_ref_create_using_pool(
    pool: *mut Rav1dMemPool,
    mut size: usize,
) -> *mut Rav1dRef {
    size = size
        .wrapping_add(::core::mem::size_of::<*mut c_void>())
        .wrapping_sub(1)
        & !(::core::mem::size_of::<*mut c_void>()).wrapping_sub(1);
    let buf: *mut Rav1dMemPoolBuffer =
        rav1d_mem_pool_pop(pool, size.wrapping_add(::core::mem::size_of::<Rav1dRef>()));
    if buf.is_null() {
        return 0 as *mut Rav1dRef;
    }
    let res: *mut Rav1dRef =
        &mut *(buf as *mut Rav1dRef).offset(-(1 as c_int) as isize) as *mut Rav1dRef;
    (*res).data = (*buf).data;
    (*res).const_data = pool as *const c_void;
    (*res).ref_cnt = AtomicI32::new(1);
    (*res).free_ref = 0 as c_int;
    (*res).free_callback = Some(pool_free_callback);
    (*res).user_data = buf as *mut c_void;
    return res;
}

pub unsafe fn rav1d_ref_wrap(
    ptr: *const u8,
    free_callback: Option<unsafe extern "C" fn(*const u8, *mut c_void) -> ()>,
    user_data: *mut c_void,
) -> *mut Rav1dRef {
    let res: *mut Rav1dRef = malloc(::core::mem::size_of::<Rav1dRef>()) as *mut Rav1dRef;
    if res.is_null() {
        return 0 as *mut Rav1dRef;
    }
    (*res).data = 0 as *mut c_void;
    (*res).const_data = ptr as *const c_void;
    (*res).ref_cnt = AtomicI32::new(1);
    (*res).free_ref = 1 as c_int;
    (*res).free_callback = free_callback;
    (*res).user_data = user_data;
    return res;
}

pub unsafe fn rav1d_ref_dec(pref: *mut *mut Rav1dRef) {
    if pref.is_null() {
        unreachable!();
    }
    let r#ref: *mut Rav1dRef = *pref;
    if r#ref.is_null() {
        return;
    }
    *pref = 0 as *mut Rav1dRef;
    if (*r#ref).ref_cnt.fetch_sub(1, Ordering::SeqCst) == 1 {
        let free_ref = (*r#ref).free_ref;
        ((*r#ref).free_callback).expect("non-null function pointer")(
            (*r#ref).const_data as *const u8,
            (*r#ref).user_data,
        );
        if free_ref != 0 {
            free(r#ref as *mut c_void);
        }
    }
}
