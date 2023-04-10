
use crate::include::stdint::*;
use ::libc;
use crate::{stdout,stderr};
use crate::errno_location;
extern "C" {
    pub type Dav1dRef;
    fn fclose(__stream: *mut libc::FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::FILE;
    fn fprintf(_: *mut libc::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut libc::FILE,
    ) -> libc::c_ulong;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn dav1d_picture_unref(p: *mut Dav1dPicture);
}





































use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;

use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I400;

















































































use crate::include::dav1d::picture::Dav1dPictureParameters;
use crate::include::dav1d::picture::Dav1dPicture;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MuxerPriv {
    pub f: *mut libc::FILE,
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
pub type YuvOutputContext = MuxerPriv;
unsafe extern "C" fn yuv_open(
    c: *mut YuvOutputContext,
    file: *const libc::c_char,
    _p: *const Dav1dPictureParameters,
    mut _fps: *const libc::c_uint,
) -> libc::c_int {
    if strcmp(file, b"-\0" as *const u8 as *const libc::c_char) == 0 {
        (*c).f = stdout;
    } else {
        (*c).f = fopen(file, b"wb\0" as *const u8 as *const libc::c_char);
        if ((*c).f).is_null() {
            fprintf(
                stderr,
                b"Failed to open %s: %s\n\0" as *const u8 as *const libc::c_char,
                file,
                strerror(*errno_location()),
            );
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn yuv_write(
    c: *mut YuvOutputContext,
    p: *mut Dav1dPicture,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ptr: *mut uint8_t = 0 as *mut uint8_t;
    let hbd: libc::c_int = ((*p).p.bpc > 8 as libc::c_int) as libc::c_int;
    ptr = (*p).data[0 as libc::c_int as usize] as *mut uint8_t;
    let mut y: libc::c_int = 0 as libc::c_int;
    loop {
        if !(y < (*p).p.h) {
            current_block = 7095457783677275021;
            break;
        }
        if fwrite(
            ptr as *const libc::c_void,
            ((*p).p.w << hbd) as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            (*c).f,
        ) != 1 as libc::c_int as libc::c_ulong
        {
            current_block = 11680617278722171943;
            break;
        }
        ptr = ptr.offset((*p).stride[0 as libc::c_int as usize] as isize);
        y += 1;
    }
    match current_block {
        7095457783677275021 => {
            if (*p).p.layout as libc::c_uint
                != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint
            {
                let ss_ver: libc::c_int = ((*p).p.layout as libc::c_uint
                    == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint)
                    as libc::c_int;
                let ss_hor: libc::c_int = ((*p).p.layout as libc::c_uint
                    != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint)
                    as libc::c_int;
                let cw: libc::c_int = (*p).p.w + ss_hor >> ss_hor;
                let ch: libc::c_int = (*p).p.h + ss_ver >> ss_ver;
                let mut pl: libc::c_int = 1 as libc::c_int;
                's_40: loop {
                    if !(pl <= 2 as libc::c_int) {
                        current_block = 7976072742316086414;
                        break;
                    }
                    ptr = (*p).data[pl as usize] as *mut uint8_t;
                    let mut y_0: libc::c_int = 0 as libc::c_int;
                    while y_0 < ch {
                        if fwrite(
                            ptr as *const libc::c_void,
                            (cw << hbd) as libc::c_ulong,
                            1 as libc::c_int as libc::c_ulong,
                            (*c).f,
                        ) != 1 as libc::c_int as libc::c_ulong
                        {
                            current_block = 11680617278722171943;
                            break 's_40;
                        }
                        ptr = ptr
                            .offset((*p).stride[1 as libc::c_int as usize] as isize);
                        y_0 += 1;
                    }
                    pl += 1;
                }
            } else {
                current_block = 7976072742316086414;
            }
            match current_block {
                11680617278722171943 => {}
                _ => {
                    dav1d_picture_unref(p);
                    return 0 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    dav1d_picture_unref(p);
    fprintf(
        stderr,
        b"Failed to write frame data: %s\n\0" as *const u8 as *const libc::c_char,
        strerror(*errno_location()),
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn yuv_close(c: *mut YuvOutputContext) {
    if (*c).f != stdout {
        fclose((*c).f);
    }
}
#[no_mangle]
pub static mut yuv_muxer: Muxer = {
    let mut init = Muxer {
        priv_data_size: ::core::mem::size_of::<YuvOutputContext>() as libc::c_ulong
            as libc::c_int,
        name: b"yuv\0" as *const u8 as *const libc::c_char,
        extension: b"yuv\0" as *const u8 as *const libc::c_char,
        write_header: Some(
            yuv_open
                as unsafe extern "C" fn(
                    *mut YuvOutputContext,
                    *const libc::c_char,
                    *const Dav1dPictureParameters,
                    *const libc::c_uint,
                ) -> libc::c_int,
        ),
        write_picture: Some(
            yuv_write
                as unsafe extern "C" fn(
                    *mut YuvOutputContext,
                    *mut Dav1dPicture,
                ) -> libc::c_int,
        ),
        write_trailer: Some(
            yuv_close as unsafe extern "C" fn(*mut YuvOutputContext) -> (),
        ),
        verify: None,
    };
    init
};
