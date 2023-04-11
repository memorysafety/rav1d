use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
extern "C" {
    pub type Dav1dRef;
    pub type MuxerPriv;
    fn dav1d_picture_unref(p: *mut Dav1dPicture);
}


use crate::include::dav1d::common::Dav1dDataProps;





























































































use crate::include::dav1d::headers::Dav1dContentLightLevel;
use crate::include::dav1d::headers::Dav1dMasteringDisplay;
use crate::include::dav1d::headers::Dav1dITUTT35;
use crate::include::dav1d::headers::Dav1dSequenceHeader;






use crate::include::dav1d::headers::Dav1dFrameHeader;












use crate::include::dav1d::picture::Dav1dPictureParameters;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dPicture {
    pub seq_hdr: *mut Dav1dSequenceHeader,
    pub frame_hdr: *mut Dav1dFrameHeader,
    pub data: [*mut libc::c_void; 3],
    pub stride: [ptrdiff_t; 2],
    pub p: Dav1dPictureParameters,
    pub m: Dav1dDataProps,
    pub content_light: *mut Dav1dContentLightLevel,
    pub mastering_display: *mut Dav1dMasteringDisplay,
    pub itut_t35: *mut Dav1dITUTT35,
    pub reserved: [uintptr_t; 4],
    pub frame_hdr_ref: *mut Dav1dRef,
    pub seq_hdr_ref: *mut Dav1dRef,
    pub content_light_ref: *mut Dav1dRef,
    pub mastering_display_ref: *mut Dav1dRef,
    pub itut_t35_ref: *mut Dav1dRef,
    pub reserved_ref: [uintptr_t; 4],
    pub ref_0: *mut Dav1dRef,
    pub allocator_data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Muxer {
    pub priv_data_size: libc::c_int,
    pub name: *const libc::c_char,
    pub extension: *const libc::c_char,
    pub write_header: Option::<
        unsafe extern "C" fn(
            *mut MuxerPriv,
            *const libc::c_char,
            *const Dav1dPictureParameters,
            *const libc::c_uint,
        ) -> libc::c_int,
    >,
    pub write_picture: Option::<
        unsafe extern "C" fn(*mut MuxerPriv, *mut Dav1dPicture) -> libc::c_int,
    >,
    pub write_trailer: Option::<unsafe extern "C" fn(*mut MuxerPriv) -> ()>,
    pub verify: Option::<
        unsafe extern "C" fn(*mut MuxerPriv, *const libc::c_char) -> libc::c_int,
    >,
}
pub type NullOutputContext = MuxerPriv;
unsafe extern "C" fn null_write(
    _c: *mut NullOutputContext,
    p: *mut Dav1dPicture,
) -> libc::c_int {
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
                as unsafe extern "C" fn(
                    *mut NullOutputContext,
                    *mut Dav1dPicture,
                ) -> libc::c_int,
        ),
        write_trailer: None,
        verify: None,
    };
    init
};
