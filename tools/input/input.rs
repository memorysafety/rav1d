use crate::compat::errno::errno_location;
use crate::compat::stdio::stderr;
use libc::calloc;
use libc::fclose;
use libc::fopen;
use libc::fprintf;
use libc::fread;
use libc::free;
use libc::malloc;
use libc::strcmp;
use libc::strerror;
use libc::ENOMEM;
use libc::ENOPROTOOPT;
use rav1d::include::dav1d::data::Dav1dData;
use std::cmp;
use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_void;
use std::mem;

extern "C" {
    pub type DemuxerPriv;
    static ivf_demuxer: Demuxer;
    static annexb_demuxer: Demuxer;
    static section5_demuxer: Demuxer;
}

#[repr(C)]
pub struct DemuxerContext {
    pub data: *mut DemuxerPriv,
    pub impl_0: *const Demuxer,
    pub priv_data: [u64; 0],
}

#[repr(C)]
pub struct Demuxer {
    pub priv_data_size: c_int,
    pub name: *const c_char,
    pub probe_sz: c_int,
    pub probe: Option<unsafe extern "C" fn(*const u8) -> c_int>,
    pub open: Option<
        unsafe extern "C" fn(
            *mut DemuxerPriv,
            *const c_char,
            *mut c_uint,
            *mut c_uint,
            *mut c_uint,
        ) -> c_int,
    >,
    pub read: Option<unsafe extern "C" fn(*mut DemuxerPriv, *mut Dav1dData) -> c_int>,
    pub seek: Option<unsafe extern "C" fn(*mut DemuxerPriv, u64) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut DemuxerPriv) -> ()>,
}

static mut demuxers: [*const Demuxer; 4] = unsafe {
    [
        &ivf_demuxer as *const Demuxer,
        &annexb_demuxer as *const Demuxer,
        &section5_demuxer as *const Demuxer,
        0 as *const Demuxer,
    ]
};

pub unsafe fn input_open(
    c_out: *mut *mut DemuxerContext,
    name: *const c_char,
    filename: *const c_char,
    fps: *mut c_uint,
    num_frames: *mut c_uint,
    timebase: *mut c_uint,
) -> c_int {
    let mut impl_0: *const Demuxer = 0 as *const Demuxer;
    let c: *mut DemuxerContext;
    let mut res;
    let mut i;
    if !name.is_null() {
        i = 0 as c_int;
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
                b"Failed to find demuxer named \"%s\"\n\0" as *const u8 as *const c_char,
                name,
            );
            return -ENOPROTOOPT;
        }
    } else {
        let mut probe_sz = 0;
        i = 0 as c_int;
        while !(demuxers[i as usize]).is_null() {
            probe_sz = cmp::max(probe_sz, (*demuxers[i as usize]).probe_sz);
            i += 1;
        }
        let probe_data: *mut u8 = malloc(probe_sz as usize) as *mut u8;
        if probe_data.is_null() {
            fprintf(
                stderr,
                b"Failed to allocate memory\n\0" as *const u8 as *const c_char,
            );
            return -ENOMEM;
        }
        let f: *mut libc::FILE = fopen(filename, b"rb\0" as *const u8 as *const c_char);
        if f.is_null() {
            fprintf(
                stderr,
                b"Failed to open input file %s: %s\n\0" as *const u8 as *const c_char,
                filename,
                strerror(*errno_location()),
            );
            return if *errno_location() != 0 {
                -*errno_location()
            } else {
                -(5 as c_int)
            };
        }
        res = (fread(probe_data as *mut c_void, 1, probe_sz as usize, f) != 0) as c_int;
        fclose(f);
        if res == 0 {
            free(probe_data as *mut c_void);
            fprintf(
                stderr,
                b"Failed to read probe data\n\0" as *const u8 as *const c_char,
            );
            return if *errno_location() != 0 {
                -*errno_location()
            } else {
                -(5 as c_int)
            };
        }
        i = 0 as c_int;
        while !(demuxers[i as usize]).is_null() {
            if ((*demuxers[i as usize]).probe).expect("non-null function pointer")(probe_data) != 0
            {
                impl_0 = demuxers[i as usize];
                break;
            } else {
                i += 1;
            }
        }
        free(probe_data as *mut c_void);
        if (demuxers[i as usize]).is_null() {
            fprintf(
                stderr,
                b"Failed to probe demuxer for file %s\n\0" as *const u8 as *const c_char,
                filename,
            );
            return -ENOPROTOOPT;
        }
    }
    c = calloc(
        1,
        mem::size_of::<DemuxerContext>() + (*impl_0).priv_data_size as usize,
    ) as *mut DemuxerContext;
    if c.is_null() {
        fprintf(
            stderr,
            b"Failed to allocate memory\n\0" as *const u8 as *const c_char,
        );
        return -ENOMEM;
    }
    (*c).impl_0 = impl_0;
    (*c).data = ((*c).priv_data).as_mut_ptr() as *mut DemuxerPriv;
    res = ((*impl_0).open).expect("non-null function pointer")(
        (*c).data,
        filename,
        fps,
        num_frames,
        timebase,
    );
    if res < 0 {
        free(c as *mut c_void);
        return res;
    }
    *c_out = c;
    return 0 as c_int;
}

pub unsafe fn input_read(ctx: *mut DemuxerContext, data: *mut Dav1dData) -> c_int {
    return ((*(*ctx).impl_0).read).expect("non-null function pointer")((*ctx).data, data);
}

// TODO(kkysen) These are used in `dav1d.rs` and `seek_stress.rs`
// but are still marked as unused since `[[bin]]` are only supposed to be one file in `cargo`.
#[allow(dead_code)]
pub unsafe fn input_seek(ctx: *mut DemuxerContext, pts: u64) -> c_int {
    return if ((*(*ctx).impl_0).seek).is_some() {
        ((*(*ctx).impl_0).seek).expect("non-null function pointer")((*ctx).data, pts)
    } else {
        -1
    };
}

pub unsafe fn input_close(ctx: *mut DemuxerContext) {
    ((*(*ctx).impl_0).close).expect("non-null function pointer")((*ctx).data);
    free(ctx as *mut c_void);
}
