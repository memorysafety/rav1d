use crate::include::common::validate::validate_input;
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
use libc::memset;
use std::ffi::c_int;
use std::ffi::c_void;
use std::ptr;

pub(crate) unsafe fn rav1d_data_create_internal(buf: *mut Rav1dData, sz: usize) -> *mut u8 {
    if let Err(e) = validate_input!((!buf.is_null(), ptr::null_mut())) {
        return e;
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
    (*buf).m = Default::default();
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
    validate_input!((!buf.is_null(), EINVAL))?;
    validate_input!((!ptr.is_null(), EINVAL))?;
    validate_input!((free_callback.is_some(), EINVAL))?;
    (*buf).r#ref = rav1d_ref_wrap(ptr, free_callback, cookie);
    if ((*buf).r#ref).is_null() {
        return Err(ENOMEM);
    }
    (*buf).data = ptr;
    (*buf).sz = sz;
    (*buf).m = Default::default();
    (*buf).m.size = sz;
    Ok(())
}

pub(crate) unsafe fn rav1d_data_wrap_user_data_internal(
    buf: *mut Rav1dData,
    user_data: *const u8,
    free_callback: Option<unsafe extern "C" fn(*const u8, *mut c_void) -> ()>,
    cookie: *mut c_void,
) -> Rav1dResult {
    validate_input!((!buf.is_null(), EINVAL))?;
    validate_input!((free_callback.is_some(), EINVAL))?;
    (*buf).m.user_data.r#ref = rav1d_ref_wrap(user_data, free_callback, cookie);
    if ((*buf).m.user_data.r#ref).is_null() {
        return Err(ENOMEM);
    }
    (*buf).m.user_data.data = user_data;
    Ok(())
}

pub(crate) unsafe fn rav1d_data_ref(dst: *mut Rav1dData, src: *const Rav1dData) {
    if validate_input!(!dst.is_null()).is_err() {
        return;
    }
    if validate_input!((*dst).data.is_null()).is_err() {
        return;
    }
    if validate_input!(!src.is_null()).is_err() {
        return;
    }
    if !((*src).r#ref).is_null() {
        if validate_input!(!(*src).data.is_null()).is_err() {
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

pub(crate) unsafe fn rav1d_data_props_unref_internal(props: *mut Rav1dDataProps) {
    if validate_input!(!props.is_null()).is_err() {
        return;
    }
    let mut user_data_ref: *mut Rav1dRef = (*props).user_data.r#ref;
    (*props) = Default::default();
    rav1d_ref_dec(&mut user_data_ref);
}

pub(crate) unsafe fn rav1d_data_unref_internal(buf: *mut Rav1dData) {
    if validate_input!(!buf.is_null()).is_err() {
        return;
    }
    let mut user_data_ref: *mut Rav1dRef = (*buf).m.user_data.r#ref;
    if !((*buf).r#ref).is_null() {
        if validate_input!(!(*buf).data.is_null()).is_err() {
            return;
        }
        rav1d_ref_dec(&mut (*buf).r#ref);
    }
    memset(
        buf as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<Rav1dData>(),
    );
    (*buf).m = Default::default();
    rav1d_ref_dec(&mut user_data_ref);
}
