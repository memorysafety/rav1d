use ::libc;
extern "C" {
    pub type Dav1dRef;
    pub type MuxerPriv;
    fn dav1d_picture_unref(p: *mut Dav1dPicture);
}

use crate::include::dav1d::picture::Dav1dPicture;
use crate::include::dav1d::picture::Dav1dPictureParameters;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Muxer {
    pub priv_data_size: libc::c_int,
    pub name: *const libc::c_char,
    pub extension: *const libc::c_char,
    pub write_header: Option<
        unsafe extern "C" fn(
            *mut MuxerPriv,
            *const libc::c_char,
            *const Dav1dPictureParameters,
            *const libc::c_uint,
        ) -> libc::c_int,
    >,
    pub write_picture:
        Option<unsafe extern "C" fn(*mut MuxerPriv, *mut Dav1dPicture) -> libc::c_int>,
    pub write_trailer: Option<unsafe extern "C" fn(*mut MuxerPriv) -> ()>,
    pub verify: Option<unsafe extern "C" fn(*mut MuxerPriv, *const libc::c_char) -> libc::c_int>,
}
pub type NullOutputContext = MuxerPriv;
unsafe extern "C" fn null_write(_c: *mut NullOutputContext, p: *mut Dav1dPicture) -> libc::c_int {
    dav1d_picture_unref(p);
    return 0 as libc::c_int;
}
#[no_mangle]
pub static mut null_muxer: Muxer = {
    let mut init = Muxer {
        priv_data_size: 0 as libc::c_int,
        name: b"null\0" as *const u8 as *const libc::c_char,
        extension: b"null\0" as *const u8 as *const libc::c_char,
        write_header: None,
        write_picture: Some(
            null_write
                as unsafe extern "C" fn(*mut NullOutputContext, *mut Dav1dPicture) -> libc::c_int,
        ),
        write_trailer: None,
        verify: None,
    };
    init
};
