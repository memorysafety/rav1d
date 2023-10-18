use crate::include::dav1d::common::Rav1dDataProps;
use crate::include::dav1d::data::Rav1dData;
use crate::src::error::Rav1dError::EINVAL;
use crate::src::error::Rav1dError::ENOMEM;
use crate::src::error::Rav1dResult;
use crate::src::r#ref::rav1d_ref_create;
use crate::src::r#ref::rav1d_ref_dec;
use crate::src::r#ref::rav1d_ref_inc;
use crate::src::r#ref::rav1d_ref_wrap;
use crate::src::r#ref::Rav1dRef;
use crate::stderr;
use libc::fprintf;
use libc::memset;
use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_void;

pub(crate) unsafe fn rav1d_data_create_internal(buf: *mut Rav1dData, sz: usize) -> *mut u8 {
    if buf.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"buf != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 27], &[c_char; 27]>(b"dav1d_data_create_internal\0"))
                .as_ptr(),
        );
        return 0 as *mut u8;
    }
    if sz > usize::MAX / 2 {
        return 0 as *mut u8;
    }
    (*buf).r#ref = rav1d_ref_create(sz);
    if ((*buf).r#ref).is_null() {
        return 0 as *mut u8;
    }
    (*buf).data = (*(*buf).r#ref).const_data as *const u8;
    (*buf).sz = sz;
    rav1d_data_props_set_defaults(&mut (*buf).m);
    (*buf).m.size = sz;
    return (*(*buf).r#ref).data as *mut u8;
}

pub(crate) unsafe fn rav1d_data_wrap_internal(
    buf: *mut Rav1dData,
    ptr: *const u8,
    sz: usize,
    free_callback: Option<unsafe extern "C" fn(*const u8, *mut c_void) -> ()>,
    cookie: *mut c_void,
) -> Rav1dResult {
    if buf.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"buf != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 25], &[c_char; 25]>(b"dav1d_data_wrap_internal\0"))
                .as_ptr(),
        );
        return Err(EINVAL);
    }
    if ptr.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"ptr != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 25], &[c_char; 25]>(b"dav1d_data_wrap_internal\0"))
                .as_ptr(),
        );
        return Err(EINVAL);
    }
    if free_callback.is_none() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"free_callback != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 25], &[c_char; 25]>(b"dav1d_data_wrap_internal\0"))
                .as_ptr(),
        );
        return Err(EINVAL);
    }
    (*buf).r#ref = rav1d_ref_wrap(ptr, free_callback, cookie);
    if ((*buf).r#ref).is_null() {
        return Err(ENOMEM);
    }
    (*buf).data = ptr;
    (*buf).sz = sz;
    rav1d_data_props_set_defaults(&mut (*buf).m);
    (*buf).m.size = sz;
    Ok(())
}

pub(crate) unsafe fn rav1d_data_wrap_user_data_internal(
    buf: *mut Rav1dData,
    user_data: *const u8,
    free_callback: Option<unsafe extern "C" fn(*const u8, *mut c_void) -> ()>,
    cookie: *mut c_void,
) -> Rav1dResult {
    if buf.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"buf != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 35], &[c_char; 35]>(
                b"dav1d_data_wrap_user_data_internal\0",
            ))
            .as_ptr(),
        );
        return Err(EINVAL);
    }
    if free_callback.is_none() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"free_callback != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 35], &[c_char; 35]>(
                b"dav1d_data_wrap_user_data_internal\0",
            ))
            .as_ptr(),
        );
        return Err(EINVAL);
    }
    (*buf).m.user_data.r#ref = rav1d_ref_wrap(user_data, free_callback, cookie);
    if ((*buf).m.user_data.r#ref).is_null() {
        return Err(ENOMEM);
    }
    (*buf).m.user_data.data = user_data;
    Ok(())
}

pub(crate) unsafe fn rav1d_data_ref(dst: *mut Rav1dData, src: *const Rav1dData) {
    if dst.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"dst != ((void*)0)\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 15], &[c_char; 15]>(b"rav1d_data_ref\0")).as_ptr(),
        );
        return;
    }
    if !((*dst).data).is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"dst->data == ((void*)0)\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 15], &[c_char; 15]>(b"rav1d_data_ref\0")).as_ptr(),
        );
        return;
    }
    if src.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"src != ((void*)0)\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 15], &[c_char; 15]>(b"rav1d_data_ref\0")).as_ptr(),
        );
        return;
    }
    if !((*src).r#ref).is_null() {
        if ((*src).data).is_null() {
            fprintf(
                stderr,
                b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
                b"src->data != ((void*)0)\0" as *const u8 as *const c_char,
                (*::core::mem::transmute::<&[u8; 15], &[c_char; 15]>(b"rav1d_data_ref\0")).as_ptr(),
            );
            return;
        }
        rav1d_ref_inc((*src).r#ref);
    }
    if !((*src).m.user_data.r#ref).is_null() {
        rav1d_ref_inc((*src).m.user_data.r#ref);
    }
    *dst = (*src).clone();
}

pub(crate) unsafe fn rav1d_data_props_copy(dst: *mut Rav1dDataProps, src: *const Rav1dDataProps) {
    if dst.is_null() {
        unreachable!();
    }
    if src.is_null() {
        unreachable!();
    }
    rav1d_ref_dec(&mut (*dst).user_data.r#ref);
    *dst = (*src).clone();
    if !((*dst).user_data.r#ref).is_null() {
        rav1d_ref_inc((*dst).user_data.r#ref);
    }
}

pub(crate) unsafe fn rav1d_data_props_set_defaults(props: *mut Rav1dDataProps) {
    if props.is_null() {
        unreachable!();
    }
    memset(
        props as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<Rav1dDataProps>(),
    );
    (*props).timestamp = i64::MIN;
    (*props).offset = -1;
}

pub(crate) unsafe fn rav1d_data_props_unref_internal(props: *mut Rav1dDataProps) {
    if props.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"props != ((void*)0)\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 32], &[c_char; 32]>(
                b"dav1d_data_props_unref_internal\0",
            ))
            .as_ptr(),
        );
        return;
    }
    let mut user_data_ref: *mut Rav1dRef = (*props).user_data.r#ref;
    rav1d_data_props_set_defaults(props);
    rav1d_ref_dec(&mut user_data_ref);
}

pub(crate) unsafe fn rav1d_data_unref_internal(buf: *mut Rav1dData) {
    if buf.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"buf != ((void*)0)\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 26], &[c_char; 26]>(b"dav1d_data_unref_internal\0"))
                .as_ptr(),
        );
        return;
    }
    let mut user_data_ref: *mut Rav1dRef = (*buf).m.user_data.r#ref;
    if !((*buf).r#ref).is_null() {
        if ((*buf).data).is_null() {
            fprintf(
                stderr,
                b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
                b"buf->data != ((void*)0)\0" as *const u8 as *const c_char,
                (*::core::mem::transmute::<&[u8; 26], &[c_char; 26]>(
                    b"dav1d_data_unref_internal\0",
                ))
                .as_ptr(),
            );
            return;
        }
        rav1d_ref_dec(&mut (*buf).r#ref);
    }
    memset(
        buf as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<Rav1dData>(),
    );
    rav1d_data_props_set_defaults(&mut (*buf).m);
    rav1d_ref_dec(&mut user_data_ref);
}
