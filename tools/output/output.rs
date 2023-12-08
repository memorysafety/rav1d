use libc::fprintf;
use libc::free;
use libc::malloc;
use libc::memcpy;
use libc::snprintf;
use libc::strchr;
use libc::strcmp;
use libc::strlen;
use libc::strncmp;
use libc::ENOMEM;
use libc::ENOPROTOOPT;
use rav1d::include::dav1d::picture::Dav1dPicture;
use rav1d::include::dav1d::picture::Dav1dPictureParameters;
use rav1d::stderr;
use std::cmp;
use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_long;
use std::ffi::c_uint;
use std::ffi::c_ulong;
use std::ffi::c_void;
use std::mem;

extern "C" {
    pub type MuxerPriv;
    static null_muxer: Muxer;
    static md5_muxer: Muxer;
    static yuv_muxer: Muxer;
    static y4m2_muxer: Muxer;
}

#[repr(C)]
pub struct MuxerContext {
    pub data: *mut MuxerPriv,
    pub impl_0: *const Muxer,
    pub one_file_per_frame: c_int,
    pub fps: [c_uint; 2],
    pub filename: *const c_char,
    pub framenum: c_int,
    pub priv_data: [u64; 0],
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

static mut muxers: [*const Muxer; 5] = unsafe {
    [
        &null_muxer as *const Muxer,
        &md5_muxer as *const Muxer,
        &yuv_muxer as *const Muxer,
        &y4m2_muxer as *const Muxer,
        0 as *const Muxer,
    ]
};

unsafe fn find_extension(f: *const c_char) -> *const c_char {
    let l: usize = strlen(f);
    if l == 0 {
        return 0 as *const c_char;
    }
    let end: *const c_char = &*f.offset(l.wrapping_sub(1) as isize) as *const c_char;
    let mut step: *const c_char = end;
    while *step as c_int >= 'a' as i32 && *step as c_int <= 'z' as i32
        || *step as c_int >= 'A' as i32 && *step as c_int <= 'Z' as i32
        || *step as c_int >= '0' as i32 && *step as c_int <= '9' as i32
    {
        step = step.offset(-1);
    }
    return if step < end
        && step > f
        && *step as c_int == '.' as i32
        && *step.offset(-1 as isize) as c_int != '/' as i32
    {
        &*step.offset(1) as *const c_char
    } else {
        0 as *const c_char
    };
}

// TODO(kkysen) These are used in `dav1d.rs` and `seek_stress.rs`
// but are still marked as unused since `[[bin]]` are only supposed to be one file in `cargo`.
#[allow(dead_code)]
pub unsafe fn output_open(
    c_out: *mut *mut MuxerContext,
    name: *const c_char,
    filename: *const c_char,
    p: *const Dav1dPictureParameters,
    fps: *const c_uint,
) -> c_int {
    let mut impl_0: *const Muxer = 0 as *const Muxer;
    let c: *mut MuxerContext;
    let mut i: c_uint;
    let mut res = 0;
    let mut name_offset = 0;
    if !name.is_null() {
        name_offset =
            5 as c_int * (strncmp(name, b"frame\0" as *const u8 as *const c_char, 5) == 0) as c_int;
        i = 0 as c_int as c_uint;
        while !(muxers[i as usize]).is_null() {
            if strcmp(
                (*muxers[i as usize]).name,
                &*name.offset(name_offset as isize),
            ) == 0
            {
                impl_0 = muxers[i as usize];
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        if (muxers[i as usize]).is_null() {
            fprintf(
                stderr,
                b"Failed to find muxer named \"%s\"\n\0" as *const u8 as *const c_char,
                name,
            );
            return -ENOPROTOOPT;
        }
    } else if strcmp(filename, b"/dev/null\0" as *const u8 as *const c_char) == 0 {
        impl_0 = muxers[0];
    } else {
        let ext: *const c_char = find_extension(filename);
        if ext.is_null() {
            fprintf(
                stderr,
                b"No extension found for file %s\n\0" as *const u8 as *const c_char,
                filename,
            );
            return -1;
        }
        i = 0 as c_int as c_uint;
        while !(muxers[i as usize]).is_null() {
            if strcmp((*muxers[i as usize]).extension, ext) == 0 {
                impl_0 = muxers[i as usize];
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        if (muxers[i as usize]).is_null() {
            fprintf(
                stderr,
                b"Failed to find muxer for extension \"%s\"\n\0" as *const u8 as *const c_char,
                ext,
            );
            return -ENOPROTOOPT;
        }
    }
    c = malloc(mem::offset_of!(MuxerContext, priv_data) + (*impl_0).priv_data_size as usize)
        as *mut MuxerContext;
    if c.is_null() {
        fprintf(
            stderr,
            b"Failed to allocate memory\n\0" as *const u8 as *const c_char,
        );
        return -ENOMEM;
    }
    (*c).impl_0 = impl_0;
    (*c).data = ((*c).priv_data).as_mut_ptr() as *mut MuxerPriv;
    let mut have_num_pattern = 0;
    let mut ptr: *const c_char = if !filename.is_null() {
        strchr(filename, '%' as i32)
    } else {
        0 as *mut c_char
    };
    while have_num_pattern == 0 && !ptr.is_null() {
        ptr = ptr.offset(1);
        while *ptr as c_int >= '0' as i32 && *ptr as c_int <= '9' as i32 {
            ptr = ptr.offset(1);
        }
        have_num_pattern = (*ptr as c_int == 'n' as i32) as c_int;
        ptr = strchr(ptr, '%' as i32);
    }
    (*c).one_file_per_frame =
        (name_offset != 0 || name.is_null() && have_num_pattern != 0) as c_int;
    if (*c).one_file_per_frame != 0 {
        (*c).fps[0] = *fps.offset(0);
        (*c).fps[1] = *fps.offset(1);
        (*c).filename = filename;
        (*c).framenum = 0 as c_int;
    } else if ((*impl_0).write_header).is_some() && {
        res = ((*impl_0).write_header).expect("non-null function pointer")(
            (*c).data,
            filename,
            p,
            fps,
        );
        res < 0
    } {
        free(c as *mut c_void);
        return res;
    }
    *c_out = c;
    return 0 as c_int;
}

unsafe fn safe_strncat(dst: *mut c_char, dst_len: c_int, src: *const c_char, src_len: c_int) {
    if src_len == 0 {
        return;
    }
    let dst_fill = strlen(dst) as c_int;
    if !(dst_fill < dst_len) {
        unreachable!();
    }
    let to_copy = cmp::min(src_len, dst_len - dst_fill - 1);
    if to_copy == 0 {
        return;
    }
    memcpy(
        dst.offset(dst_fill as isize) as *mut c_void,
        src as *const c_void,
        to_copy as usize,
    );
    *dst.offset((dst_fill + to_copy) as isize) = 0 as c_int as c_char;
}

unsafe fn assemble_field(
    dst: *mut c_char,
    dst_len: c_int,
    fmt: *const c_char,
    fmt_len: c_int,
    field: c_int,
) {
    let mut fmt_copy: [c_char; 32] = [0; 32];
    if !(*fmt.offset(0) as c_int == '%' as i32) {
        unreachable!();
    }
    fmt_copy[0] = '%' as i32 as c_char;
    if *fmt.offset(1) as c_int >= '1' as i32 && *fmt.offset(1) as c_int <= '9' as i32 {
        fmt_copy[1] = '0' as i32 as c_char;
        fmt_copy[2] = 0 as c_int as c_char;
    } else {
        fmt_copy[1] = 0 as c_int as c_char;
    }
    safe_strncat(
        fmt_copy.as_mut_ptr(),
        ::core::mem::size_of::<[c_char; 32]>() as c_ulong as c_int,
        &*fmt.offset(1),
        fmt_len - 1,
    );
    safe_strncat(
        fmt_copy.as_mut_ptr(),
        ::core::mem::size_of::<[c_char; 32]>() as c_ulong as c_int,
        b"d\0" as *const u8 as *const c_char,
        1 as c_int,
    );
    let mut tmp: [c_char; 32] = [0; 32];
    snprintf(
        tmp.as_mut_ptr(),
        ::core::mem::size_of::<[c_char; 32]>(),
        fmt_copy.as_mut_ptr(),
        field,
    );
    safe_strncat(
        dst,
        dst_len,
        tmp.as_mut_ptr(),
        strlen(tmp.as_mut_ptr()) as c_int,
    );
}

unsafe fn assemble_filename(
    ctx: *mut MuxerContext,
    filename: *mut c_char,
    filename_size: c_int,
    p: *const Dav1dPictureParameters,
) {
    *filename.offset(0) = 0 as c_int as c_char;
    let fresh0 = (*ctx).framenum;
    (*ctx).framenum = (*ctx).framenum + 1;
    let framenum = fresh0;
    if ((*ctx).filename).is_null() {
        unreachable!();
    }
    let mut ptr: *const c_char = (*ctx).filename;
    let mut iptr: *const c_char;
    loop {
        iptr = strchr(ptr, '%' as i32);
        if iptr.is_null() {
            break;
        }
        safe_strncat(
            filename,
            filename_size,
            ptr,
            iptr.offset_from(ptr) as c_long as c_int,
        );
        ptr = iptr;
        let mut iiptr: *const c_char = &*iptr.offset(1) as *const c_char;
        while *iiptr as c_int >= '0' as i32 && *iiptr as c_int <= '9' as i32 {
            iiptr = iiptr.offset(1);
        }
        match *iiptr as c_int {
            119 => {
                assemble_field(
                    filename,
                    filename_size,
                    ptr,
                    iiptr.offset_from(ptr) as c_long as c_int,
                    (*p).w,
                );
            }
            104 => {
                assemble_field(
                    filename,
                    filename_size,
                    ptr,
                    iiptr.offset_from(ptr) as c_long as c_int,
                    (*p).h,
                );
            }
            110 => {
                assemble_field(
                    filename,
                    filename_size,
                    ptr,
                    iiptr.offset_from(ptr) as c_long as c_int,
                    framenum,
                );
            }
            _ => {
                safe_strncat(
                    filename,
                    filename_size,
                    b"%\0" as *const u8 as *const c_char,
                    1 as c_int,
                );
                ptr = &*iptr.offset(1) as *const c_char;
                continue;
            }
        }
        ptr = &*iiptr.offset(1) as *const c_char;
    }
    safe_strncat(filename, filename_size, ptr, strlen(ptr) as c_int);
}

// TODO(kkysen) These are used in `dav1d.rs` and `seek_stress.rs`
// but are still marked as unused since `[[bin]]` are only supposed to be one file in `cargo`.
#[allow(dead_code)]
pub unsafe fn output_write(ctx: *mut MuxerContext, p: *mut Dav1dPicture) -> c_int {
    let mut res;
    if (*ctx).one_file_per_frame != 0 && ((*(*ctx).impl_0).write_header).is_some() {
        let mut filename: [c_char; 1024] = [0; 1024];
        assemble_filename(
            ctx,
            filename.as_mut_ptr(),
            ::core::mem::size_of::<[c_char; 1024]>() as c_ulong as c_int,
            &mut (*p).p,
        );
        res = ((*(*ctx).impl_0).write_header).expect("non-null function pointer")(
            (*ctx).data,
            filename.as_mut_ptr(),
            &mut (*p).p,
            ((*ctx).fps).as_mut_ptr() as *const c_uint,
        );
        if res < 0 {
            return res;
        }
    }
    res = ((*(*ctx).impl_0).write_picture).expect("non-null function pointer")((*ctx).data, p);
    if res < 0 {
        return res;
    }
    if (*ctx).one_file_per_frame != 0 && ((*(*ctx).impl_0).write_trailer).is_some() {
        ((*(*ctx).impl_0).write_trailer).expect("non-null function pointer")((*ctx).data);
    }
    return 0 as c_int;
}

// TODO(kkysen) These are used in `dav1d.rs` and `seek_stress.rs`
// but are still marked as unused since `[[bin]]` are only supposed to be one file in `cargo`.
#[allow(dead_code)]
pub unsafe fn output_close(ctx: *mut MuxerContext) {
    if (*ctx).one_file_per_frame == 0 && ((*(*ctx).impl_0).write_trailer).is_some() {
        ((*(*ctx).impl_0).write_trailer).expect("non-null function pointer")((*ctx).data);
    }
    free(ctx as *mut c_void);
}

// TODO(kkysen) These are used in `dav1d.rs` and `seek_stress.rs`
// but are still marked as unused since `[[bin]]` are only supposed to be one file in `cargo`.
#[allow(dead_code)]
pub unsafe fn output_verify(ctx: *mut MuxerContext, md5_str: *const c_char) -> c_int {
    let res = if ((*(*ctx).impl_0).verify).is_some() {
        ((*(*ctx).impl_0).verify).expect("non-null function pointer")((*ctx).data, md5_str)
    } else {
        0 as c_int
    };
    free(ctx as *mut c_void);
    return res;
}
