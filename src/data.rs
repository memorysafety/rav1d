use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::dav1d::data::Dav1dData;
use crate::src::r#ref::dav1d_ref_create;
use crate::src::r#ref::dav1d_ref_dec;
use crate::src::r#ref::dav1d_ref_inc;
use crate::src::r#ref::dav1d_ref_wrap;
use crate::src::r#ref::Dav1dRef;
use crate::stderr;
use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_ulong;
use std::ffi::c_void;

extern "C" {
    fn memset(_: *mut c_void, _: c_int, _: c_ulong) -> *mut c_void;
    fn fprintf(_: *mut libc::FILE, _: *const c_char, _: ...) -> c_int;
}

pub unsafe fn dav1d_data_create_internal(buf: *mut Dav1dData, sz: usize) -> *mut u8 {
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
    (*buf).r#ref = dav1d_ref_create(sz);
    if ((*buf).r#ref).is_null() {
        return 0 as *mut u8;
    }
    (*buf).data = (*(*buf).r#ref).const_data as *const u8;
    (*buf).sz = sz;
    dav1d_data_props_set_defaults(&mut (*buf).m);
    (*buf).m.size = sz;
    return (*(*buf).r#ref).data as *mut u8;
}

pub unsafe fn dav1d_data_wrap_internal(
    buf: *mut Dav1dData,
    ptr: *const u8,
    sz: usize,
    free_callback: Option<unsafe extern "C" fn(*const u8, *mut c_void) -> ()>,
    cookie: *mut c_void,
) -> c_int {
    if buf.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"buf != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 25], &[c_char; 25]>(b"dav1d_data_wrap_internal\0"))
                .as_ptr(),
        );
        return -(22 as c_int);
    }
    if ptr.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"ptr != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 25], &[c_char; 25]>(b"dav1d_data_wrap_internal\0"))
                .as_ptr(),
        );
        return -(22 as c_int);
    }
    if free_callback.is_none() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"free_callback != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 25], &[c_char; 25]>(b"dav1d_data_wrap_internal\0"))
                .as_ptr(),
        );
        return -(22 as c_int);
    }
    (*buf).r#ref = dav1d_ref_wrap(ptr, free_callback, cookie);
    if ((*buf).r#ref).is_null() {
        return -(12 as c_int);
    }
    (*buf).data = ptr;
    (*buf).sz = sz;
    dav1d_data_props_set_defaults(&mut (*buf).m);
    (*buf).m.size = sz;
    return 0 as c_int;
}

pub unsafe fn dav1d_data_wrap_user_data_internal(
    buf: *mut Dav1dData,
    user_data: *const u8,
    free_callback: Option<unsafe extern "C" fn(*const u8, *mut c_void) -> ()>,
    cookie: *mut c_void,
) -> c_int {
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
        return -(22 as c_int);
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
        return -(22 as c_int);
    }
    (*buf).m.user_data.r#ref = dav1d_ref_wrap(user_data, free_callback, cookie);
    if ((*buf).m.user_data.r#ref).is_null() {
        return -(12 as c_int);
    }
    (*buf).m.user_data.data = user_data;
    return 0 as c_int;
}

pub unsafe fn dav1d_data_ref(dst: *mut Dav1dData, src: *const Dav1dData) {
    if dst.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"dst != ((void*)0)\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 15], &[c_char; 15]>(b"dav1d_data_ref\0")).as_ptr(),
        );
        return;
    }
    if !((*dst).data).is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"dst->data == ((void*)0)\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 15], &[c_char; 15]>(b"dav1d_data_ref\0")).as_ptr(),
        );
        return;
    }
    if src.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"src != ((void*)0)\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 15], &[c_char; 15]>(b"dav1d_data_ref\0")).as_ptr(),
        );
        return;
    }
    if !((*src).r#ref).is_null() {
        if ((*src).data).is_null() {
            fprintf(
                stderr,
                b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
                b"src->data != ((void*)0)\0" as *const u8 as *const c_char,
                (*::core::mem::transmute::<&[u8; 15], &[c_char; 15]>(b"dav1d_data_ref\0")).as_ptr(),
            );
            return;
        }
        dav1d_ref_inc((*src).r#ref);
    }
    if !((*src).m.user_data.r#ref).is_null() {
        dav1d_ref_inc((*src).m.user_data.r#ref);
    }
    *dst = (*src).clone();
}

pub unsafe fn dav1d_data_props_copy(dst: *mut Dav1dDataProps, src: *const Dav1dDataProps) {
    if dst.is_null() {
        unreachable!();
    }
    if src.is_null() {
        unreachable!();
    }
    dav1d_ref_dec(&mut (*dst).user_data.r#ref);
    *dst = (*src).clone();
    if !((*dst).user_data.r#ref).is_null() {
        dav1d_ref_inc((*dst).user_data.r#ref);
    }
}

pub unsafe fn dav1d_data_props_set_defaults(props: *mut Dav1dDataProps) {
    if props.is_null() {
        unreachable!();
    }
    memset(
        props as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<Dav1dDataProps>() as c_ulong,
    );
    (*props).timestamp = i64::MIN;
    (*props).offset = -1;
}

pub unsafe fn dav1d_data_props_unref_internal(props: *mut Dav1dDataProps) {
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
    let mut user_data_ref: *mut Dav1dRef = (*props).user_data.r#ref;
    dav1d_data_props_set_defaults(props);
    dav1d_ref_dec(&mut user_data_ref);
}

pub unsafe fn dav1d_data_unref_internal(buf: *mut Dav1dData) {
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
    let mut user_data_ref: *mut Dav1dRef = (*buf).m.user_data.r#ref;
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
        dav1d_ref_dec(&mut (*buf).r#ref);
    }
    memset(
        buf as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<Dav1dData>() as c_ulong,
    );
    dav1d_data_props_set_defaults(&mut (*buf).m);
    dav1d_ref_dec(&mut user_data_ref);
}
