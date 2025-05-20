use crate::compat::errno::errno_location;
use crate::compat::stdio::stderr;
use crate::compat::stdio::stdout;
use libc::fclose;
use libc::fopen;
use libc::fprintf;
use libc::fwrite;
use libc::strcmp;
use libc::strerror;
use rav1d::dav1d_picture_unref;
use rav1d::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I400;
use rav1d::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;
use rav1d::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;
use rav1d::include::dav1d::picture::Dav1dPicture;
use rav1d::include::dav1d::picture::Dav1dPictureParameters;
use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_ulong;
use std::ffi::c_void;
use std::ptr;
use std::ptr::NonNull;

#[repr(C)]
pub struct MuxerPriv {
    pub f: *mut libc::FILE,
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

pub type YuvOutputContext = MuxerPriv;

unsafe extern "C" fn yuv_open(
    c: *mut YuvOutputContext,
    file: *const c_char,
    _p: *const Dav1dPictureParameters,
    mut _fps: *const c_uint,
) -> c_int {
    if strcmp(file, b"-\0" as *const u8 as *const c_char) == 0 {
        (*c).f = stdout();
    } else {
        (*c).f = fopen(file, b"wb\0" as *const u8 as *const c_char);
        if ((*c).f).is_null() {
            fprintf(
                stderr(),
                b"Failed to open %s: %s\n\0" as *const u8 as *const c_char,
                file,
                strerror(*errno_location()),
            );
            return -1;
        }
    }
    return 0 as c_int;
}

unsafe extern "C" fn yuv_write(c: *mut YuvOutputContext, p: *mut Dav1dPicture) -> c_int {
    let mut current_block: u64;
    let mut ptr: *mut u8;
    let hbd = ((*p).p.bpc > 8) as c_int;
    ptr = (*p).data[0].map_or_else(ptr::null_mut, NonNull::as_ptr) as *mut u8;
    let mut y = 0;
    loop {
        if !(y < (*p).p.h) {
            current_block = 7095457783677275021;
            break;
        }
        if fwrite(ptr as *const c_void, ((*p).p.w << hbd) as usize, 1, (*c).f) != 1 {
            current_block = 11680617278722171943;
            break;
        }
        ptr = ptr.offset((*p).stride[0] as isize);
        y += 1;
    }
    match current_block {
        7095457783677275021 => {
            if (*p).p.layout as c_uint != DAV1D_PIXEL_LAYOUT_I400 as c_int as c_uint {
                let ss_ver = ((*p).p.layout as c_uint == DAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint)
                    as c_int;
                let ss_hor = ((*p).p.layout as c_uint != DAV1D_PIXEL_LAYOUT_I444 as c_int as c_uint)
                    as c_int;
                let cw = (*p).p.w + ss_hor >> ss_hor;
                let ch = (*p).p.h + ss_ver >> ss_ver;
                let mut pl = 1;
                's_40: loop {
                    if !(pl <= 2) {
                        current_block = 7976072742316086414;
                        break;
                    }
                    ptr = (*p).data[pl as usize].map_or_else(ptr::null_mut, NonNull::as_ptr)
                        as *mut u8;
                    let mut y_0 = 0;
                    while y_0 < ch {
                        if fwrite(ptr as *const c_void, (cw << hbd) as usize, 1, (*c).f) != 1 {
                            current_block = 11680617278722171943;
                            break 's_40;
                        }
                        ptr = ptr.offset((*p).stride[1] as isize);
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
                    dav1d_picture_unref(NonNull::new(p));
                    return 0 as c_int;
                }
            }
        }
        _ => {}
    }
    dav1d_picture_unref(NonNull::new(p));
    fprintf(
        stderr(),
        b"Failed to write frame data: %s\n\0" as *const u8 as *const c_char,
        strerror(*errno_location()),
    );
    return -1;
}

unsafe extern "C" fn yuv_close(c: *mut YuvOutputContext) {
    if (*c).f != stdout() {
        fclose((*c).f);
    }
}

#[no_mangle]
pub static mut yuv_muxer: Muxer = Muxer {
    priv_data_size: ::core::mem::size_of::<YuvOutputContext>() as c_ulong as c_int,
    name: b"yuv\0" as *const u8 as *const c_char,
    extension: b"yuv\0" as *const u8 as *const c_char,
    write_header: Some(yuv_open),
    write_picture: Some(yuv_write),
    write_trailer: Some(yuv_close),
    verify: None,
};
