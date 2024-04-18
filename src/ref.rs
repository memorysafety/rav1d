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
