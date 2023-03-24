use ::libc;
use crate::stderr;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn fprintf(_: *mut libc::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn dav1d_ref_dec(ref_0: *mut *mut Dav1dRef);
    fn dav1d_ref_wrap(
        ptr: *const uint8_t,
        free_callback: Option::<
            unsafe extern "C" fn(*const uint8_t, *mut libc::c_void) -> (),
        >,
        user_data: *mut libc::c_void,
    ) -> *mut Dav1dRef;
    fn dav1d_ref_create(size: size_t) -> *mut Dav1dRef;
}
pub type __uint8_t = libc::c_uchar;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dUserData {
    pub data: *const uint8_t,
    pub ref_0: *mut Dav1dRef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dRef {
    pub data: *mut libc::c_void,
    pub const_data: *const libc::c_void,
    pub ref_cnt: atomic_int,
    pub free_ref: libc::c_int,
    pub free_callback: Option::<
        unsafe extern "C" fn(*const uint8_t, *mut libc::c_void) -> (),
    >,
    pub user_data: *mut libc::c_void,
}
pub type atomic_int = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dDataProps {
    pub timestamp: int64_t,
    pub duration: int64_t,
    pub offset: int64_t,
    pub size: size_t,
    pub user_data: Dav1dUserData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dData {
    pub data: *const uint8_t,
    pub sz: size_t,
    pub ref_0: *mut Dav1dRef,
    pub m: Dav1dDataProps,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub const memory_order_relaxed: memory_order = 0;
pub type memory_order = libc::c_uint;
pub const memory_order_seq_cst: memory_order = 5;
pub const memory_order_acq_rel: memory_order = 4;
pub const memory_order_release: memory_order = 3;
pub const memory_order_acquire: memory_order = 2;
pub const memory_order_consume: memory_order = 1;
#[inline]
unsafe extern "C" fn dav1d_ref_inc(ref_0: *mut Dav1dRef) {
    ::core::intrinsics::atomic_xadd_relaxed(&mut (*ref_0).ref_cnt, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_create_internal(
    buf: *mut Dav1dData,
    sz: size_t,
) -> *mut uint8_t {
    if buf.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const libc::c_char,
            b"buf != NULL\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"dav1d_data_create_internal\0"))
                .as_ptr(),
        );
        return 0 as *mut uint8_t;
    }
    if sz
        > (18446744073709551615 as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        return 0 as *mut uint8_t;
    }
    (*buf).ref_0 = dav1d_ref_create(sz);
    if ((*buf).ref_0).is_null() {
        return 0 as *mut uint8_t;
    }
    (*buf).data = (*(*buf).ref_0).const_data as *const uint8_t;
    (*buf).sz = sz;
    dav1d_data_props_set_defaults(&mut (*buf).m);
    (*buf).m.size = sz;
    return (*(*buf).ref_0).data as *mut uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_wrap_internal(
    buf: *mut Dav1dData,
    ptr: *const uint8_t,
    sz: size_t,
    free_callback: Option::<
        unsafe extern "C" fn(*const uint8_t, *mut libc::c_void) -> (),
    >,
    cookie: *mut libc::c_void,
) -> libc::c_int {
    if buf.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const libc::c_char,
            b"buf != NULL\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"dav1d_data_wrap_internal\0"))
                .as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    if ptr.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const libc::c_char,
            b"ptr != NULL\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"dav1d_data_wrap_internal\0"))
                .as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    if free_callback.is_none() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const libc::c_char,
            b"free_callback != NULL\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"dav1d_data_wrap_internal\0"))
                .as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    (*buf).ref_0 = dav1d_ref_wrap(ptr, free_callback, cookie);
    if ((*buf).ref_0).is_null() {
        return -(12 as libc::c_int);
    }
    (*buf).data = ptr;
    (*buf).sz = sz;
    dav1d_data_props_set_defaults(&mut (*buf).m);
    (*buf).m.size = sz;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_wrap_user_data_internal(
    buf: *mut Dav1dData,
    user_data: *const uint8_t,
    free_callback: Option::<
        unsafe extern "C" fn(*const uint8_t, *mut libc::c_void) -> (),
    >,
    cookie: *mut libc::c_void,
) -> libc::c_int {
    if buf.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const libc::c_char,
            b"buf != NULL\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"dav1d_data_wrap_user_data_internal\0"))
                .as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    if free_callback.is_none() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const libc::c_char,
            b"free_callback != NULL\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"dav1d_data_wrap_user_data_internal\0"))
                .as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    (*buf).m.user_data.ref_0 = dav1d_ref_wrap(user_data, free_callback, cookie);
    if ((*buf).m.user_data.ref_0).is_null() {
        return -(12 as libc::c_int);
    }
    (*buf).m.user_data.data = user_data;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_ref(dst: *mut Dav1dData, src: *const Dav1dData) {
    if dst.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const libc::c_char,
            b"dst != ((void*)0)\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"dav1d_data_ref\0"))
                .as_ptr(),
        );
        return;
    }
    if !((*dst).data).is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const libc::c_char,
            b"dst->data == ((void*)0)\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"dav1d_data_ref\0"))
                .as_ptr(),
        );
        return;
    }
    if src.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const libc::c_char,
            b"src != ((void*)0)\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"dav1d_data_ref\0"))
                .as_ptr(),
        );
        return;
    }
    if !((*src).ref_0).is_null() {
        if ((*src).data).is_null() {
            fprintf(
                stderr,
                b"Input validation check '%s' failed in %s!\n\0" as *const u8
                    as *const libc::c_char,
                b"src->data != ((void*)0)\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 15],
                    &[libc::c_char; 15],
                >(b"dav1d_data_ref\0"))
                    .as_ptr(),
            );
            return;
        }
        dav1d_ref_inc((*src).ref_0);
    }
    if !((*src).m.user_data.ref_0).is_null() {
        dav1d_ref_inc((*src).m.user_data.ref_0);
    }
    *dst = *src;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_props_copy(
    dst: *mut Dav1dDataProps,
    src: *const Dav1dDataProps,
) {
    if dst.is_null() {
        unreachable!();
    }
    if src.is_null() {
        unreachable!();
    }
    dav1d_ref_dec(&mut (*dst).user_data.ref_0);
    *dst = *src;
    if !((*dst).user_data.ref_0).is_null() {
        dav1d_ref_inc((*dst).user_data.ref_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_props_set_defaults(props: *mut Dav1dDataProps) {
    if props.is_null() {
        unreachable!();
    }
    memset(
        props as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Dav1dDataProps>() as libc::c_ulong,
    );
    (*props)
        .timestamp = -(9223372036854775807 as libc::c_long)
        - 1 as libc::c_int as libc::c_long;
    (*props).offset = -(1 as libc::c_int) as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_props_unref_internal(props: *mut Dav1dDataProps) {
    if props.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const libc::c_char,
            b"props != ((void*)0)\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"dav1d_data_props_unref_internal\0"))
                .as_ptr(),
        );
        return;
    }
    let mut user_data_ref: *mut Dav1dRef = (*props).user_data.ref_0;
    dav1d_data_props_set_defaults(props);
    dav1d_ref_dec(&mut user_data_ref);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_unref_internal(buf: *mut Dav1dData) {
    if buf.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const libc::c_char,
            b"buf != ((void*)0)\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"dav1d_data_unref_internal\0"))
                .as_ptr(),
        );
        return;
    }
    let mut user_data_ref: *mut Dav1dRef = (*buf).m.user_data.ref_0;
    if !((*buf).ref_0).is_null() {
        if ((*buf).data).is_null() {
            fprintf(
                stderr,
                b"Input validation check '%s' failed in %s!\n\0" as *const u8
                    as *const libc::c_char,
                b"buf->data != ((void*)0)\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"dav1d_data_unref_internal\0"))
                    .as_ptr(),
            );
            return;
        }
        dav1d_ref_dec(&mut (*buf).ref_0);
    }
    memset(
        buf as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Dav1dData>() as libc::c_ulong,
    );
    dav1d_data_props_set_defaults(&mut (*buf).m);
    dav1d_ref_dec(&mut user_data_ref);
}
