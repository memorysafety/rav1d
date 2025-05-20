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
use rav1d::include::dav1d::headers::DAV1D_CHR_UNKNOWN;
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
    pub first: c_int,
    pub fps: [c_uint; 2],
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

pub type Y4m2OutputContext = MuxerPriv;

unsafe extern "C" fn y4m2_open(
    c: *mut Y4m2OutputContext,
    file: *const c_char,
    mut _p: *const Dav1dPictureParameters,
    fps: *const c_uint,
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
    (*c).first = 1 as c_int;
    (*c).fps[0] = *fps.offset(0);
    (*c).fps[1] = *fps.offset(1);
    return 0 as c_int;
}

unsafe fn write_header(c: *mut Y4m2OutputContext, p: *const Dav1dPicture) -> c_int {
    static mut ss_names: [[*const c_char; 3]; 4] = [
        [
            b"mono\0" as *const u8 as *const c_char,
            b"mono10\0" as *const u8 as *const c_char,
            b"mono12\0" as *const u8 as *const c_char,
        ],
        [
            0 as *const c_char,
            b"420p10\0" as *const u8 as *const c_char,
            b"420p12\0" as *const u8 as *const c_char,
        ],
        [
            b"422\0" as *const u8 as *const c_char,
            b"422p10\0" as *const u8 as *const c_char,
            b"422p12\0" as *const u8 as *const c_char,
        ],
        [
            b"444\0" as *const u8 as *const c_char,
            b"444p10\0" as *const u8 as *const c_char,
            b"444p12\0" as *const u8 as *const c_char,
        ],
    ];
    static mut chr_names_8bpc_i420: [*const c_char; 3] = [
        b"420jpeg\0" as *const u8 as *const c_char,
        b"420mpeg2\0" as *const u8 as *const c_char,
        b"420\0" as *const u8 as *const c_char,
    ];
    let seq_hdr = (*p).seq_hdr.unwrap().as_ref();
    let frame_hdr = (*p).frame_hdr.unwrap().as_ref();
    let ss_name: *const c_char =
        if (*p).p.layout as c_uint == DAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint && (*p).p.bpc == 8
        {
            chr_names_8bpc_i420[(if seq_hdr.chr as c_uint > 2 as c_uint {
                DAV1D_CHR_UNKNOWN as c_int as c_uint
            } else {
                seq_hdr.chr as c_uint
            }) as usize]
        } else {
            ss_names[(*p).p.layout as usize][seq_hdr.hbd as usize]
        };
    let fw: c_uint = (*p).p.w as c_uint;
    let fh: c_uint = (*p).p.h as c_uint;
    let mut aw: u64 = (fh as u64).wrapping_mul(frame_hdr.render_width as u64);
    let mut ah: u64 = (fw as u64).wrapping_mul(frame_hdr.render_height as u64);
    let mut gcd: u64 = ah;
    let mut a: u64 = aw;
    let mut b: u64;
    loop {
        b = a.wrapping_rem(gcd);
        if !(b != 0) {
            break;
        }
        a = gcd;
        gcd = b;
    }
    aw = aw.wrapping_div(gcd);
    ah = ah.wrapping_div(gcd);
    fprintf(
        (*c).f,
        b"YUV4MPEG2 W%u H%u F%u:%u Ip A%lu:%lu C%s\n\0" as *const u8 as *const c_char,
        fw,
        fh,
        (*c).fps[0],
        (*c).fps[1],
        aw,
        ah,
        ss_name,
    );
    return 0 as c_int;
}

unsafe extern "C" fn y4m2_write(c: *mut Y4m2OutputContext, p: *mut Dav1dPicture) -> c_int {
    let mut current_block: u64;
    if (*c).first != 0 {
        (*c).first = 0 as c_int;
        let res = write_header(c, p);
        if res < 0 {
            return res;
        }
    }
    fprintf((*c).f, b"FRAME\n\0" as *const u8 as *const c_char);
    let mut ptr: *mut u8;
    let hbd = ((*p).p.bpc > 8) as c_int;
    ptr = (*p).data[0].map_or_else(ptr::null_mut, NonNull::as_ptr) as *mut u8;
    let mut y = 0;
    loop {
        if !(y < (*p).p.h) {
            current_block = 11812396948646013369;
            break;
        }
        if fwrite(ptr as *const c_void, ((*p).p.w << hbd) as usize, 1, (*c).f) != 1 {
            current_block = 11545648641752300099;
            break;
        }
        ptr = ptr.offset((*p).stride[0] as isize);
        y += 1;
    }
    match current_block {
        11812396948646013369 => {
            if (*p).p.layout as c_uint != DAV1D_PIXEL_LAYOUT_I400 as c_int as c_uint {
                let ss_ver = ((*p).p.layout as c_uint == DAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint)
                    as c_int;
                let ss_hor = ((*p).p.layout as c_uint != DAV1D_PIXEL_LAYOUT_I444 as c_int as c_uint)
                    as c_int;
                let cw = (*p).p.w + ss_hor >> ss_hor;
                let ch = (*p).p.h + ss_ver >> ss_ver;
                let mut pl = 1;
                's_64: loop {
                    if !(pl <= 2) {
                        current_block = 13797916685926291137;
                        break;
                    }
                    ptr = (*p).data[pl as usize].map_or_else(ptr::null_mut, NonNull::as_ptr)
                        as *mut u8;
                    let mut y_0 = 0;
                    while y_0 < ch {
                        if fwrite(ptr as *const c_void, (cw << hbd) as usize, 1, (*c).f) != 1 {
                            current_block = 11545648641752300099;
                            break 's_64;
                        }
                        ptr = ptr.offset((*p).stride[1] as isize);
                        y_0 += 1;
                    }
                    pl += 1;
                }
            } else {
                current_block = 13797916685926291137;
            }
            match current_block {
                11545648641752300099 => {}
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

unsafe extern "C" fn y4m2_close(c: *mut Y4m2OutputContext) {
    if (*c).f != stdout() {
        fclose((*c).f);
    }
}

#[no_mangle]
pub static mut y4m2_muxer: Muxer = Muxer {
    priv_data_size: ::core::mem::size_of::<Y4m2OutputContext>() as c_ulong as c_int,
    name: b"yuv4mpeg2\0" as *const u8 as *const c_char,
    extension: b"y4m\0" as *const u8 as *const c_char,
    write_header: Some(y4m2_open),
    write_picture: Some(y4m2_write),
    write_trailer: Some(y4m2_close),
    verify: None,
};
