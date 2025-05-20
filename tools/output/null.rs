use crate::output::output::Muxer;
use crate::output::output::MuxerPriv;
use rav1d::dav1d_picture_unref;
use rav1d::include::dav1d::picture::Dav1dPicture;
use std::ffi::c_char;
use std::ffi::c_int;
use std::ptr::NonNull;

type NullOutputContext = MuxerPriv;

unsafe extern "C" fn null_write(_c: *mut NullOutputContext, p: *mut Dav1dPicture) -> c_int {
    dav1d_picture_unref(NonNull::new(p));
    return 0 as c_int;
}

#[no_mangle]
static mut null_muxer: Muxer = Muxer {
    priv_data_size: 0 as c_int,
    name: b"null\0" as *const u8 as *const c_char,
    extension: b"null\0" as *const u8 as *const c_char,
    write_header: None,
    write_picture: Some(null_write),
    write_trailer: None,
    verify: None,
};
