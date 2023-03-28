use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
use crate::stderr;
use crate::errno_location;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type Dav1dRef;
    pub type DemuxerPriv;
    fn fclose(__stream: *mut libc::FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::FILE;
    fn fprintf(_: *mut libc::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut libc::FILE,
    ) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    static ivf_demuxer: Demuxer;
    static annexb_demuxer: Demuxer;
    static section5_demuxer: Demuxer;
}



pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dUserData {
    pub data: *const uint8_t,
    pub ref_0: *mut Dav1dRef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dDataProps {
    pub timestamp: int64_t,
    pub duration: int64_t,
    pub offset: int64_t,
    pub size: size_t,
    pub user_data: Dav1dUserData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dData {
    pub data: *const uint8_t,
    pub sz: size_t,
    pub ref_0: *mut Dav1dRef,
    pub m: Dav1dDataProps,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DemuxerContext {
    pub data: *mut DemuxerPriv,
    pub impl_0: *const Demuxer,
    pub priv_data: [uint64_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Demuxer {
    pub priv_data_size: libc::c_int,
    pub name: *const libc::c_char,
    pub probe_sz: libc::c_int,
    pub probe: Option::<unsafe extern "C" fn(*const uint8_t) -> libc::c_int>,
    pub open: Option::<
        unsafe extern "C" fn(
            *mut DemuxerPriv,
            *const libc::c_char,
            *mut libc::c_uint,
            *mut libc::c_uint,
            *mut libc::c_uint,
        ) -> libc::c_int,
    >,
    pub read: Option::<
        unsafe extern "C" fn(*mut DemuxerPriv, *mut Dav1dData) -> libc::c_int,
    >,
    pub seek: Option::<unsafe extern "C" fn(*mut DemuxerPriv, uint64_t) -> libc::c_int>,
    pub close: Option::<unsafe extern "C" fn(*mut DemuxerPriv) -> ()>,
}
#[inline]
unsafe extern "C" fn imax(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
static mut demuxers: [*const Demuxer; 4] = unsafe {
    [
        &ivf_demuxer as *const Demuxer,
        &annexb_demuxer as *const Demuxer,
        &section5_demuxer as *const Demuxer,
        0 as *const Demuxer,
    ]
};
#[no_mangle]
pub unsafe extern "C" fn input_open(
    c_out: *mut *mut DemuxerContext,
    name: *const libc::c_char,
    filename: *const libc::c_char,
    mut fps: *mut libc::c_uint,
    num_frames: *mut libc::c_uint,
    mut timebase: *mut libc::c_uint,
) -> libc::c_int {
    let mut impl_0: *const Demuxer = 0 as *const Demuxer;
    let mut c: *mut DemuxerContext = 0 as *mut DemuxerContext;
    let mut res: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if !name.is_null() {
        i = 0 as libc::c_int;
        while !(demuxers[i as usize]).is_null() {
            if strcmp((*demuxers[i as usize]).name, name) == 0 {
                impl_0 = demuxers[i as usize];
                break;
            } else {
                i += 1;
            }
        }
        if (demuxers[i as usize]).is_null() {
            fprintf(
                stderr,
                b"Failed to find demuxer named \"%s\"\n\0" as *const u8
                    as *const libc::c_char,
                name,
            );
            return -(92 as libc::c_int);
        }
    } else {
        let mut probe_sz: libc::c_int = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while !(demuxers[i as usize]).is_null() {
            probe_sz = imax(probe_sz, (*demuxers[i as usize]).probe_sz);
            i += 1;
        }
        let probe_data: *mut uint8_t = malloc(probe_sz as libc::c_ulong) as *mut uint8_t;
        if probe_data.is_null() {
            fprintf(
                stderr,
                b"Failed to allocate memory\n\0" as *const u8 as *const libc::c_char,
            );
            return -(12 as libc::c_int);
        }
        let mut f: *mut libc::FILE = fopen(
            filename,
            b"rb\0" as *const u8 as *const libc::c_char,
        );
        if f.is_null() {
            fprintf(
                stderr,
                b"Failed to open input file %s: %s\n\0" as *const u8
                    as *const libc::c_char,
                filename,
                strerror(*errno_location()),
            );
            return if *errno_location() != 0 {
                -*errno_location()
            } else {
                -(5 as libc::c_int)
            };
        }
        res = (fread(
            probe_data as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            probe_sz as libc::c_ulong,
            f,
        ) != 0) as libc::c_int;
        fclose(f);
        if res == 0 {
            free(probe_data as *mut libc::c_void);
            fprintf(
                stderr,
                b"Failed to read probe data\n\0" as *const u8 as *const libc::c_char,
            );
            return if *errno_location() != 0 {
                -*errno_location()
            } else {
                -(5 as libc::c_int)
            };
        }
        i = 0 as libc::c_int;
        while !(demuxers[i as usize]).is_null() {
            if ((*demuxers[i as usize]).probe)
                .expect("non-null function pointer")(probe_data) != 0
            {
                impl_0 = demuxers[i as usize];
                break;
            } else {
                i += 1;
            }
        }
        free(probe_data as *mut libc::c_void);
        if (demuxers[i as usize]).is_null() {
            fprintf(
                stderr,
                b"Failed to probe demuxer for file %s\n\0" as *const u8
                    as *const libc::c_char,
                filename,
            );
            return -(92 as libc::c_int);
        }
    }
    c = calloc(
        1 as libc::c_int as libc::c_ulong,
        (16 as libc::c_ulong).wrapping_add((*impl_0).priv_data_size as libc::c_ulong),
    ) as *mut DemuxerContext;
    if c.is_null() {
        fprintf(
            stderr,
            b"Failed to allocate memory\n\0" as *const u8 as *const libc::c_char,
        );
        return -(12 as libc::c_int);
    }
    (*c).impl_0 = impl_0;
    (*c).data = ((*c).priv_data).as_mut_ptr() as *mut DemuxerPriv;
    res = ((*impl_0).open)
        .expect(
            "non-null function pointer",
        )((*c).data, filename, fps, num_frames, timebase);
    if res < 0 as libc::c_int {
        free(c as *mut libc::c_void);
        return res;
    }
    *c_out = c;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn input_read(
    ctx: *mut DemuxerContext,
    data: *mut Dav1dData,
) -> libc::c_int {
    return ((*(*ctx).impl_0).read)
        .expect("non-null function pointer")((*ctx).data, data);
}
#[no_mangle]
pub unsafe extern "C" fn input_seek(
    ctx: *mut DemuxerContext,
    pts: uint64_t,
) -> libc::c_int {
    return if ((*(*ctx).impl_0).seek).is_some() {
        ((*(*ctx).impl_0).seek).expect("non-null function pointer")((*ctx).data, pts)
    } else {
        -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn input_close(ctx: *mut DemuxerContext) {
    ((*(*ctx).impl_0).close).expect("non-null function pointer")((*ctx).data);
    free(ctx as *mut libc::c_void);
}
