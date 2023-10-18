use rav1d::include::dav1d::picture::Dav1dPicture;
use rav1d::include::dav1d::picture::Dav1dPictureParameters;
use rav1d::src::lib::dav1d_picture_unref;
use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_uint;

extern "C" {
    pub type MuxerPriv;
}

#[repr(C)]
pub struct Muxer {
    pub priv_data_size: c_int,
    pub name: *const c_char,
    pub extension: *const c_char,
    pub write_header: Option<
        unsafe extern "C" fn(
            *mut MuxerPriv,
            *const c_char,
            *const Dav1dPictureParameters,
            *const c_uint,
        ) -> c_int,
    >,
    pub write_picture: Option<unsafe extern "C" fn(*mut MuxerPriv, *mut Dav1dPicture) -> c_int>,
    pub write_trailer: Option<unsafe extern "C" fn(*mut MuxerPriv) -> ()>,
    pub verify: Option<unsafe extern "C" fn(*mut MuxerPriv, *const c_char) -> c_int>,
}

pub type NullOutputContext = MuxerPriv;

unsafe extern "C" fn null_write(_c: *mut NullOutputContext, p: *mut Dav1dPicture) -> c_int {
    dav1d_picture_unref(p);
    return 0 as c_int;
}

#[no_mangle]
pub static mut null_muxer: Muxer = Muxer {
    priv_data_size: 0 as c_int,
    name: b"null\0" as *const u8 as *const c_char,
    extension: b"null\0" as *const u8 as *const c_char,
    write_header: None,
    write_picture: Some(null_write),
    write_trailer: None,
    verify: None,
};
