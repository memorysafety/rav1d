use crate::include::common::validate::validate_input;
use crate::include::dav1d::data::Rav1dData;
use crate::src::c_arc::CArc;
use crate::src::c_box::CBox;
use crate::src::c_box::FnFree;
use crate::src::c_box::Free;
use crate::src::error::Rav1dError::EINVAL;
use crate::src::error::Rav1dError::ENOMEM;
use crate::src::error::Rav1dResult;
use crate::src::r#ref::rav1d_ref_create;
use crate::src::r#ref::rav1d_ref_dec;
use crate::src::r#ref::rav1d_ref_inc;
use crate::src::r#ref::rav1d_ref_wrap;
use std::ffi::c_void;
use std::mem;
use std::ptr;
use std::ptr::NonNull;

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

impl Rav1dData {
    /// # Safety
    ///
    /// See [`CBox::from_c`]'s safety for `user_data`, `free_callback`, `cookie`.
    pub unsafe fn wrap_user_data(
        &mut self,
        user_data: NonNull<u8>,
        free_callback: Option<FnFree>,
        cookie: *mut c_void,
    ) -> Rav1dResult {
        let free = validate_input!(free_callback.ok_or(EINVAL))?;
        let free = Free { free, cookie };
        let user_data = CBox::from_c(user_data, free);
        let user_data = CArc::wrap(user_data)?;
        self.m.user_data = Some(user_data);
        Ok(())
    }
}

pub(crate) unsafe fn rav1d_data_ref(dst: &mut Rav1dData, src: &Rav1dData) {
    if validate_input!((*dst).data.is_null()).is_err() {
        return;
    }
    if !src.r#ref.is_null() {
        if validate_input!(!src.data.is_null()).is_err() {
            return;
        }
        rav1d_ref_inc(src.r#ref);
    }
    *dst = src.clone();
}

pub(crate) unsafe fn rav1d_data_unref_internal(buf: *mut Rav1dData) {
    if validate_input!(!buf.is_null()).is_err() {
        return;
    }
    let Rav1dData {
        data,
        sz: _,
        mut r#ref,
        m: _,
    } = mem::take(&mut *buf);
    let _ = mem::take(&mut (*buf).m);
    if !r#ref.is_null() {
        if validate_input!(!data.is_null()).is_err() {
            return;
        }
        rav1d_ref_dec(&mut r#ref);
    }
}
