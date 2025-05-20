use crate::compat::errno::errno_location;
use crate::compat::stdio::fseeko;
use crate::compat::stdio::ftello;
use crate::compat::stdio::stderr;
use libc::fclose;
use libc::fopen;
use libc::fprintf;
use libc::fread;
use libc::ptrdiff_t;
use libc::strerror;
use libc::ENOMEM;
use rav1d::dav1d_data_create;
use rav1d::dav1d_data_unref;
use rav1d::include::dav1d::data::Dav1dData;
use std::ffi::c_char;
use std::ffi::c_double;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_ulong;
use std::ffi::c_void;
use std::ptr::NonNull;

#[repr(C)]
pub struct DemuxerPriv {
    pub f: *mut libc::FILE,
    pub broken: c_int,
    pub timebase: c_double,
    pub last_ts: u64,
    pub step: u64,
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

pub type IvfInputContext = DemuxerPriv;

static probe_data: [u8; 12] = [
    b'D', b'K', b'I', b'F', 0, 0, 0x20, 0, b'A', b'V', b'0', b'1',
];

unsafe extern "C" fn ivf_probe(data: *const u8) -> c_int {
    (*(data as *const [u8; 12]) == probe_data) as c_int
}

unsafe fn rl32(p: *const u8) -> c_uint {
    return (*p.offset(3) as u32) << 24 as c_uint
        | ((*p.offset(2) as c_int) << 16 as c_uint) as c_uint
        | ((*p.offset(1) as c_int) << 8 as c_uint) as c_uint
        | *p.offset(0) as c_uint;
}

unsafe fn rl64(p: *const u8) -> i64 {
    return ((rl32(&*p.offset(4)) as u64) << 32 | rl32(p) as u64) as i64;
}

unsafe extern "C" fn ivf_open(
    c: *mut IvfInputContext,
    file: *const c_char,
    fps: *mut c_uint,
    num_frames: *mut c_uint,
    timebase: *mut c_uint,
) -> c_int {
    let mut hdr: [u8; 32] = [0; 32];
    (*c).f = fopen(file, b"rb\0" as *const u8 as *const c_char);
    if ((*c).f).is_null() {
        fprintf(
            stderr(),
            b"Failed to open %s: %s\n\0" as *const u8 as *const c_char,
            file,
            strerror(*errno_location()),
        );
        return -1;
    } else {
        if fread(hdr.as_mut_ptr() as *mut c_void, 32, 1, (*c).f) != 1 {
            fprintf(
                stderr(),
                b"Failed to read stream header: %s\n\0" as *const u8 as *const c_char,
                strerror(*errno_location()),
            );
            fclose((*c).f);
            return -1;
        } else {
            let dkif = b"DKIF";
            if hdr[..dkif.len()] != dkif[..] {
                fprintf(
                    stderr(),
                    b"%s is not an IVF file [tag=%.4s|0x%02x%02x%02x%02x]\n\0" as *const u8
                        as *const c_char,
                    file,
                    hdr.as_mut_ptr(),
                    hdr[0] as c_int,
                    hdr[1] as c_int,
                    hdr[2] as c_int,
                    hdr[3] as c_int,
                );
                fclose((*c).f);
                return -1;
            } else {
                let av01 = b"AV01";
                if hdr[8..][..av01.len()] != av01[..] {
                    fprintf(
                        stderr(),
                        b"%s is not an AV1 file [tag=%.4s|0x%02x%02x%02x%02x]\n\0" as *const u8
                            as *const c_char,
                        file,
                        &mut *hdr.as_mut_ptr().offset(8) as *mut u8,
                        hdr[8] as c_int,
                        hdr[9] as c_int,
                        hdr[10] as c_int,
                        hdr[11] as c_int,
                    );
                    fclose((*c).f);
                    return -1;
                }
            }
        }
    }
    *timebase.offset(0) = rl32(&mut *hdr.as_mut_ptr().offset(16));
    *timebase.offset(1) = rl32(&mut *hdr.as_mut_ptr().offset(20));
    let duration: c_uint = rl32(&mut *hdr.as_mut_ptr().offset(24));
    let mut data: [u8; 8] = [0; 8];
    (*c).broken = 0 as c_int;
    *num_frames = 0 as c_int as c_uint;
    while !(fread(data.as_mut_ptr() as *mut c_void, 4, 1, (*c).f) != 1) {
        let sz: usize = rl32(data.as_mut_ptr()) as usize;
        if fread(data.as_mut_ptr() as *mut c_void, 8, 1, (*c).f) != 1 {
            break;
        }
        let ts: u64 = rl64(data.as_mut_ptr()) as u64;
        if *num_frames != 0 && ts <= (*c).last_ts {
            (*c).broken = 1 as c_int;
        }
        (*c).last_ts = ts;
        fseeko((*c).f, sz as libc::off_t, 1 as c_int);
        *num_frames = (*num_frames).wrapping_add(1);
    }
    let mut fps_num: u64 = (*timebase.offset(0) as u64).wrapping_mul(*num_frames as u64);
    let mut fps_den: u64 = (*timebase.offset(1) as u64).wrapping_mul(duration as u64);
    if fps_num != 0 && fps_den != 0 {
        let mut gcd: u64 = fps_num;
        let mut a: u64 = fps_den;
        let mut b: u64;
        loop {
            b = a.wrapping_rem(gcd);
            if !(b != 0) {
                break;
            }
            a = gcd;
            gcd = b;
        }
        fps_num = fps_num.wrapping_div(gcd);
        fps_den = fps_den.wrapping_div(gcd);
        while fps_num | fps_den > u32::MAX as u64 {
            fps_num >>= 1;
            fps_den >>= 1;
        }
    }
    if fps_num != 0 && fps_den != 0 {
        *fps.offset(0) = fps_num as c_uint;
        *fps.offset(1) = fps_den as c_uint;
    } else {
        let ref mut fresh0 = *fps.offset(1);
        *fresh0 = 0 as c_int as c_uint;
        *fps.offset(0) = *fresh0;
    }
    (*c).timebase = *timebase.offset(0) as c_double / *timebase.offset(1) as c_double;
    (*c).step = duration.wrapping_div(*num_frames) as u64;
    fseeko((*c).f, 32, 0);
    (*c).last_ts = 0 as c_int as u64;
    return 0 as c_int;
}

#[inline]

unsafe fn ivf_read_header(
    c: *mut IvfInputContext,
    sz: *mut ptrdiff_t,
    off_: *mut libc::off_t,
    ts: *mut u64,
) -> c_int {
    let mut data: [u8; 8] = [0; 8];
    let off: libc::off_t = ftello((*c).f);
    if !off_.is_null() {
        *off_ = off;
    }
    if fread(data.as_mut_ptr() as *mut c_void, 4, 1, (*c).f) != 1 {
        return -1;
    }
    *sz = rl32(data.as_mut_ptr()) as ptrdiff_t;
    if (*c).broken == 0 {
        if fread(data.as_mut_ptr() as *mut c_void, 8, 1, (*c).f) != 1 {
            return -1;
        }
        *ts = rl64(data.as_mut_ptr()) as u64;
    } else {
        if fseeko((*c).f, 8 as libc::off_t, 1 as c_int) != 0 {
            return -1;
        }
        *ts = if off > 32 {
            ((*c).last_ts).wrapping_add((*c).step)
        } else {
            0
        };
    }
    return 0 as c_int;
}

unsafe extern "C" fn ivf_read(c: *mut IvfInputContext, buf: *mut Dav1dData) -> c_int {
    let ptr: *mut u8;
    let mut sz: ptrdiff_t = 0;
    let mut off: libc::off_t = 0;
    let mut ts: u64 = 0;
    if ivf_read_header(c, &mut sz, &mut off, &mut ts) != 0 {
        return -1;
    }
    ptr = dav1d_data_create(NonNull::new(buf), sz as usize);
    if ptr.is_null() {
        return -1;
    }
    if fread(ptr as *mut c_void, sz as usize, 1, (*c).f) != 1 {
        fprintf(
            stderr(),
            b"Failed to read frame data: %s\n\0" as *const u8 as *const c_char,
            strerror(*errno_location()),
        );
        dav1d_data_unref(NonNull::new(buf));
        return -1;
    }
    (*buf).m.offset = off;
    (*buf).m.timestamp = ts as i64;
    (*c).last_ts = ts;
    return 0 as c_int;
}

unsafe extern "C" fn ivf_seek(c: *mut IvfInputContext, pts: u64) -> c_int {
    let mut current_block: u64;
    let mut cur: u64 = 0;
    let ts: u64 = (pts as c_double * (*c).timebase / 1000000000.0f64).round() as u64;
    if ts <= (*c).last_ts {
        if fseeko((*c).f, 32, 0) != 0 {
            current_block = 679495355492430298;
        } else {
            current_block = 12675440807659640239;
        }
    } else {
        current_block = 12675440807659640239;
    }
    loop {
        match current_block {
            679495355492430298 => {
                fprintf(
                    stderr(),
                    b"Failed to seek: %s\n\0" as *const u8 as *const c_char,
                    strerror(*errno_location()),
                );
                return -1;
            }
            _ => {
                let mut sz: ptrdiff_t = 0;
                if ivf_read_header(c, &mut sz, 0 as *mut libc::off_t, &mut cur) != 0 {
                    current_block = 679495355492430298;
                    continue;
                }
                if cur >= ts {
                    if fseeko((*c).f, -ENOMEM as libc::off_t, 1 as c_int) != 0 {
                        current_block = 679495355492430298;
                        continue;
                    }
                    return 0 as c_int;
                } else {
                    if fseeko((*c).f, sz as libc::off_t, 1 as c_int) != 0 {
                        current_block = 679495355492430298;
                        continue;
                    }
                    (*c).last_ts = cur;
                    current_block = 12675440807659640239;
                }
            }
        }
    }
}

unsafe extern "C" fn ivf_close(c: *mut IvfInputContext) {
    fclose((*c).f);
}

#[no_mangle]
pub static mut ivf_demuxer: Demuxer = Demuxer {
    priv_data_size: ::core::mem::size_of::<IvfInputContext>() as c_ulong as c_int,
    name: b"ivf\0" as *const u8 as *const c_char,
    probe_sz: ::core::mem::size_of::<[u8; 12]>() as c_ulong as c_int,
    probe: Some(ivf_probe),
    open: Some(ivf_open),
    read: Some(ivf_read),
    seek: Some(ivf_seek),
    close: Some(ivf_close),
};
