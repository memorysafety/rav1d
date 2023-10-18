use crate::include::common::validate::validate_input;
use crate::include::dav1d::dav1d::Rav1dLogger;
use crate::src::internal::Rav1dContext;
use crate::stderr;
use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_void;
use std::ptr;

extern "C" {
    fn vfprintf(_: *mut libc::FILE, _: *const c_char, _: ::core::ffi::VaList) -> c_int;
}

#[cold]
pub unsafe extern "C" fn rav1d_log_default_callback(
    _cookie: *mut c_void,
    format: *const c_char,
    mut ap: ::core::ffi::VaList,
) {
    vfprintf(stderr, format, ap.as_va_list());
}

impl Default for Rav1dLogger {
    fn default() -> Self {
        Self {
            cookie: ptr::null_mut(),
            callback: Some(rav1d_log_default_callback),
        }
    }
}

#[cold]
pub unsafe extern "C" fn rav1d_log(c: *mut Rav1dContext, format: *const c_char, args: ...) {
    if validate_input!(!c.is_null()).is_err() {
        return;
    }
    if ((*c).logger.callback).is_none() {
        return;
    }
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    ((*c).logger.callback).expect("non-null function pointer")(
        (*c).logger.cookie,
        format,
        ap.as_va_list(),
    );
}
