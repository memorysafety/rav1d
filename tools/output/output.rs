use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
use crate::stderr;
extern "C" {
    pub type Dav1dRef;
    pub type MuxerPriv;
    fn fprintf(_: *mut libc::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn malloc(_: size_t) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: size_t,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> size_t;
    static null_muxer: Muxer;
    static md5_muxer: Muxer;
    static yuv_muxer: Muxer;
    static y4m2_muxer: Muxer;
}
use crate::include::dav1d::picture::Dav1dPictureParameters;
use crate::include::dav1d::picture::Dav1dPicture;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MuxerContext {
    pub data: *mut MuxerPriv,
    pub impl_0: *const Muxer,
    pub one_file_per_frame: libc::c_int,
    pub fps: [libc::c_uint; 2],
    pub filename: *const libc::c_char,
    pub framenum: libc::c_int,
    pub priv_data: [uint64_t; 0],
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
#[inline]
unsafe extern "C" fn imin(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
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
unsafe extern "C" fn find_extension(f: *const libc::c_char) -> *const libc::c_char {
    let l: size_t = strlen(f);
    if l == 0 {
        return 0 as *const libc::c_char;
    }
    let end: *const libc::c_char = &*f
        .offset(l.wrapping_sub(1) as isize)
        as *const libc::c_char;
    let mut step: *const libc::c_char = end;
    while *step as libc::c_int >= 'a' as i32 && *step as libc::c_int <= 'z' as i32
        || *step as libc::c_int >= 'A' as i32 && *step as libc::c_int <= 'Z' as i32
        || *step as libc::c_int >= '0' as i32 && *step as libc::c_int <= '9' as i32
    {
        step = step.offset(-1);
    }
    return if step < end && step > f && *step as libc::c_int == '.' as i32
        && *step.offset(-(1 as libc::c_int) as isize) as libc::c_int != '/' as i32
    {
        &*step.offset(1 as libc::c_int as isize) as *const libc::c_char
    } else {
        0 as *const libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn output_open(
    c_out: *mut *mut MuxerContext,
    name: *const libc::c_char,
    filename: *const libc::c_char,
    p: *const Dav1dPictureParameters,
    mut fps: *const libc::c_uint,
) -> libc::c_int {
    let mut impl_0: *const Muxer = 0 as *const Muxer;
    let mut c: *mut MuxerContext = 0 as *mut MuxerContext;
    let mut i: libc::c_uint = 0;
    let mut res: libc::c_int = 0;
    let mut name_offset: libc::c_int = 0 as libc::c_int;
    if !name.is_null() {
        name_offset = 5 as libc::c_int
            * (strncmp(
                name,
                b"frame\0" as *const u8 as *const libc::c_char,
                5,
            ) == 0) as libc::c_int;
        i = 0 as libc::c_int as libc::c_uint;
        while !(muxers[i as usize]).is_null() {
            if strcmp((*muxers[i as usize]).name, &*name.offset(name_offset as isize))
                == 0
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
                b"Failed to find muxer named \"%s\"\n\0" as *const u8
                    as *const libc::c_char,
                name,
            );
            return -(92 as libc::c_int);
        }
    } else if strcmp(filename, b"/dev/null\0" as *const u8 as *const libc::c_char) == 0 {
        impl_0 = muxers[0 as libc::c_int as usize];
    } else {
        let ext: *const libc::c_char = find_extension(filename);
        if ext.is_null() {
            fprintf(
                stderr,
                b"No extension found for file %s\n\0" as *const u8
                    as *const libc::c_char,
                filename,
            );
            return -(1 as libc::c_int);
        }
        i = 0 as libc::c_int as libc::c_uint;
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
                b"Failed to find muxer for extension \"%s\"\n\0" as *const u8
                    as *const libc::c_char,
                ext,
            );
            return -(92 as libc::c_int);
        }
    }
    c = malloc(
        (48 as size_t).wrapping_add((*impl_0).priv_data_size as size_t),
    ) as *mut MuxerContext;
    if c.is_null() {
        fprintf(
            stderr,
            b"Failed to allocate memory\n\0" as *const u8 as *const libc::c_char,
        );
        return -(12 as libc::c_int);
    }
    (*c).impl_0 = impl_0;
    (*c).data = ((*c).priv_data).as_mut_ptr() as *mut MuxerPriv;
    let mut have_num_pattern: libc::c_int = 0 as libc::c_int;
    let mut ptr: *const libc::c_char = if !filename.is_null() {
        strchr(filename, '%' as i32)
    } else {
        0 as *mut libc::c_char
    };
    while have_num_pattern == 0 && !ptr.is_null() {
        ptr = ptr.offset(1);
        while *ptr as libc::c_int >= '0' as i32 && *ptr as libc::c_int <= '9' as i32 {
            ptr = ptr.offset(1);
        }
        have_num_pattern = (*ptr as libc::c_int == 'n' as i32) as libc::c_int;
        ptr = strchr(ptr, '%' as i32);
    }
    (*c)
        .one_file_per_frame = (name_offset != 0
        || name.is_null() && have_num_pattern != 0) as libc::c_int;
    if (*c).one_file_per_frame != 0 {
        (*c).fps[0 as libc::c_int as usize] = *fps.offset(0 as libc::c_int as isize);
        (*c).fps[1 as libc::c_int as usize] = *fps.offset(1 as libc::c_int as isize);
        (*c).filename = filename;
        (*c).framenum = 0 as libc::c_int;
    } else if ((*impl_0).write_header).is_some()
        && {
            res = ((*impl_0).write_header)
                .expect("non-null function pointer")((*c).data, filename, p, fps);
            res < 0 as libc::c_int
        }
    {
        free(c as *mut libc::c_void);
        return res;
    }
    *c_out = c;
    return 0 as libc::c_int;
}
unsafe extern "C" fn safe_strncat(
    dst: *mut libc::c_char,
    dst_len: libc::c_int,
    src: *const libc::c_char,
    src_len: libc::c_int,
) {
    if src_len == 0 {
        return;
    }
    let dst_fill: libc::c_int = strlen(dst) as libc::c_int;
    if !(dst_fill < dst_len) {
        unreachable!();
    }
    let to_copy: libc::c_int = imin(src_len, dst_len - dst_fill - 1 as libc::c_int);
    if to_copy == 0 {
        return;
    }
    memcpy(
        dst.offset(dst_fill as isize) as *mut libc::c_void,
        src as *const libc::c_void,
        to_copy as size_t,
    );
    *dst.offset((dst_fill + to_copy) as isize) = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn assemble_field(
    dst: *mut libc::c_char,
    dst_len: libc::c_int,
    fmt: *const libc::c_char,
    fmt_len: libc::c_int,
    field: libc::c_int,
) {
    let mut fmt_copy: [libc::c_char; 32] = [0; 32];
    if !(*fmt.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32) {
        unreachable!();
    }
    fmt_copy[0 as libc::c_int as usize] = '%' as i32 as libc::c_char;
    if *fmt.offset(1 as libc::c_int as isize) as libc::c_int >= '1' as i32
        && *fmt.offset(1 as libc::c_int as isize) as libc::c_int <= '9' as i32
    {
        fmt_copy[1 as libc::c_int as usize] = '0' as i32 as libc::c_char;
        fmt_copy[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    } else {
        fmt_copy[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    }
    safe_strncat(
        fmt_copy.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
        &*fmt.offset(1 as libc::c_int as isize),
        fmt_len - 1 as libc::c_int,
    );
    safe_strncat(
        fmt_copy.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
        b"d\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    let mut tmp: [libc::c_char; 32] = [0; 32];
    snprintf(
        tmp.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 32]>(),
        fmt_copy.as_mut_ptr(),
        field,
    );
    safe_strncat(
        dst,
        dst_len,
        tmp.as_mut_ptr(),
        strlen(tmp.as_mut_ptr()) as libc::c_int,
    );
}
unsafe extern "C" fn assemble_filename(
    ctx: *mut MuxerContext,
    filename: *mut libc::c_char,
    filename_size: libc::c_int,
    p: *const Dav1dPictureParameters,
) {
    *filename.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    let fresh0 = (*ctx).framenum;
    (*ctx).framenum = (*ctx).framenum + 1;
    let framenum: libc::c_int = fresh0;
    if ((*ctx).filename).is_null() {
        unreachable!();
    }
    let mut ptr: *const libc::c_char = (*ctx).filename;
    let mut iptr: *const libc::c_char = 0 as *const libc::c_char;
    loop {
        iptr = strchr(ptr, '%' as i32);
        if iptr.is_null() {
            break;
        }
        safe_strncat(
            filename,
            filename_size,
            ptr,
            iptr.offset_from(ptr) as libc::c_long as libc::c_int,
        );
        ptr = iptr;
        let mut iiptr: *const libc::c_char = &*iptr.offset(1 as libc::c_int as isize)
            as *const libc::c_char;
        while *iiptr as libc::c_int >= '0' as i32 && *iiptr as libc::c_int <= '9' as i32
        {
            iiptr = iiptr.offset(1);
        }
        match *iiptr as libc::c_int {
            119 => {
                assemble_field(
                    filename,
                    filename_size,
                    ptr,
                    iiptr.offset_from(ptr) as libc::c_long as libc::c_int,
                    (*p).w,
                );
            }
            104 => {
                assemble_field(
                    filename,
                    filename_size,
                    ptr,
                    iiptr.offset_from(ptr) as libc::c_long as libc::c_int,
                    (*p).h,
                );
            }
            110 => {
                assemble_field(
                    filename,
                    filename_size,
                    ptr,
                    iiptr.offset_from(ptr) as libc::c_long as libc::c_int,
                    framenum,
                );
            }
            _ => {
                safe_strncat(
                    filename,
                    filename_size,
                    b"%\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                );
                ptr = &*iptr.offset(1 as libc::c_int as isize) as *const libc::c_char;
                continue;
            }
        }
        ptr = &*iiptr.offset(1 as libc::c_int as isize) as *const libc::c_char;
    }
    safe_strncat(filename, filename_size, ptr, strlen(ptr) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn output_write(
    ctx: *mut MuxerContext,
    p: *mut Dav1dPicture,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    if (*ctx).one_file_per_frame != 0 && ((*(*ctx).impl_0).write_header).is_some() {
        let mut filename: [libc::c_char; 1024] = [0; 1024];
        assemble_filename(
            ctx,
            filename.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                as libc::c_int,
            &mut (*p).p,
        );
        res = ((*(*ctx).impl_0).write_header)
            .expect(
                "non-null function pointer",
            )(
            (*ctx).data,
            filename.as_mut_ptr(),
            &mut (*p).p,
            ((*ctx).fps).as_mut_ptr() as *const libc::c_uint,
        );
        if res < 0 as libc::c_int {
            return res;
        }
    }
    res = ((*(*ctx).impl_0).write_picture)
        .expect("non-null function pointer")((*ctx).data, p);
    if res < 0 as libc::c_int {
        return res;
    }
    if (*ctx).one_file_per_frame != 0 && ((*(*ctx).impl_0).write_trailer).is_some() {
        ((*(*ctx).impl_0).write_trailer)
            .expect("non-null function pointer")((*ctx).data);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn output_close(ctx: *mut MuxerContext) {
    if (*ctx).one_file_per_frame == 0 && ((*(*ctx).impl_0).write_trailer).is_some() {
        ((*(*ctx).impl_0).write_trailer)
            .expect("non-null function pointer")((*ctx).data);
    }
    free(ctx as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn output_verify(
    ctx: *mut MuxerContext,
    md5_str: *const libc::c_char,
) -> libc::c_int {
    let res: libc::c_int = if ((*(*ctx).impl_0).verify).is_some() {
        ((*(*ctx).impl_0).verify)
            .expect("non-null function pointer")((*ctx).data, md5_str)
    } else {
        0 as libc::c_int
    };
    free(ctx as *mut libc::c_void);
    return res;
}
