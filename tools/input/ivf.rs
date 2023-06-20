use crate::errno_location;
use crate::include::stddef::*;
use crate::include::stdint::*;
use crate::stderr;
use ::libc;
extern "C" {
    pub type Dav1dRef;
    fn llround(_: libc::c_double) -> libc::c_longlong;
    fn fclose(__stream: *mut libc::FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::FILE;
    fn fprintf(_: *mut libc::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(_: *mut libc::c_void, _: size_t, _: size_t, _: *mut libc::FILE) -> libc::c_ulong;
    fn fseeko(__stream: *mut libc::FILE, __off: __off_t, __whence: libc::c_int) -> libc::c_int;
    fn ftello(__stream: *mut libc::FILE) -> __off_t;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: size_t) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn dav1d_data_create(data: *mut Dav1dData, sz: size_t) -> *mut uint8_t;
    fn dav1d_data_unref(data: *mut Dav1dData);
}
use crate::include::sys::types::__off_t;

use crate::include::dav1d::data::Dav1dData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DemuxerPriv {
    pub f: *mut libc::FILE,
    pub broken: libc::c_int,
    pub timebase: libc::c_double,
    pub last_ts: uint64_t,
    pub step: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Demuxer {
    pub priv_data_size: libc::c_int,
    pub name: *const libc::c_char,
    pub probe_sz: libc::c_int,
    pub probe: Option<unsafe extern "C" fn(*const uint8_t) -> libc::c_int>,
    pub open: Option<
        unsafe extern "C" fn(
            *mut DemuxerPriv,
            *const libc::c_char,
            *mut libc::c_uint,
            *mut libc::c_uint,
            *mut libc::c_uint,
        ) -> libc::c_int,
    >,
    pub read: Option<unsafe extern "C" fn(*mut DemuxerPriv, *mut Dav1dData) -> libc::c_int>,
    pub seek: Option<unsafe extern "C" fn(*mut DemuxerPriv, uint64_t) -> libc::c_int>,
    pub close: Option<unsafe extern "C" fn(*mut DemuxerPriv) -> ()>,
}
pub type IvfInputContext = DemuxerPriv;
static mut probe_data: [uint8_t; 12] = [
    'D' as i32 as uint8_t,
    'K' as i32 as uint8_t,
    'I' as i32 as uint8_t,
    'F' as i32 as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x20 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    'A' as i32 as uint8_t,
    'V' as i32 as uint8_t,
    '0' as i32 as uint8_t,
    '1' as i32 as uint8_t,
];
unsafe extern "C" fn ivf_probe(data: *const uint8_t) -> libc::c_int {
    return (memcmp(
        data as *const libc::c_void,
        probe_data.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint8_t; 12]>(),
    ) == 0) as libc::c_int;
}
unsafe extern "C" fn rl32(p: *const uint8_t) -> libc::c_uint {
    return (*p.offset(3) as uint32_t) << 24 as libc::c_uint
        | ((*p.offset(2) as libc::c_int) << 16 as libc::c_uint) as libc::c_uint
        | ((*p.offset(1) as libc::c_int) << 8 as libc::c_uint) as libc::c_uint
        | *p.offset(0) as libc::c_uint;
}
unsafe extern "C" fn rl64(p: *const uint8_t) -> int64_t {
    return ((rl32(&*p.offset(4)) as uint64_t) << 32 | rl32(p) as uint64_t) as int64_t;
}
unsafe extern "C" fn ivf_open(
    c: *mut IvfInputContext,
    file: *const libc::c_char,
    mut fps: *mut libc::c_uint,
    num_frames: *mut libc::c_uint,
    mut timebase: *mut libc::c_uint,
) -> libc::c_int {
    let mut hdr: [uint8_t; 32] = [0; 32];
    (*c).f = fopen(file, b"rb\0" as *const u8 as *const libc::c_char);
    if ((*c).f).is_null() {
        fprintf(
            stderr,
            b"Failed to open %s: %s\n\0" as *const u8 as *const libc::c_char,
            file,
            strerror(*errno_location()),
        );
        return -(1 as libc::c_int);
    } else {
        if fread(hdr.as_mut_ptr() as *mut libc::c_void, 32, 1, (*c).f)
            != 1 as libc::c_int as libc::c_ulong
        {
            fprintf(
                stderr,
                b"Failed to read stream header: %s\n\0" as *const u8 as *const libc::c_char,
                strerror(*errno_location()),
            );
            fclose((*c).f);
            return -(1 as libc::c_int);
        } else {
            if memcmp(
                hdr.as_mut_ptr() as *const libc::c_void,
                b"DKIF\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                4,
            ) != 0
            {
                fprintf(
                    stderr,
                    b"%s is not an IVF file [tag=%.4s|0x%02x%02x%02x%02x]\n\0" as *const u8
                        as *const libc::c_char,
                    file,
                    hdr.as_mut_ptr(),
                    hdr[0] as libc::c_int,
                    hdr[1] as libc::c_int,
                    hdr[2] as libc::c_int,
                    hdr[3] as libc::c_int,
                );
                fclose((*c).f);
                return -(1 as libc::c_int);
            } else {
                if memcmp(
                    &mut *hdr.as_mut_ptr().offset(8) as *mut uint8_t as *const libc::c_void,
                    b"AV01\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    4,
                ) != 0
                {
                    fprintf(
                        stderr,
                        b"%s is not an AV1 file [tag=%.4s|0x%02x%02x%02x%02x]\n\0" as *const u8
                            as *const libc::c_char,
                        file,
                        &mut *hdr.as_mut_ptr().offset(8) as *mut uint8_t,
                        hdr[8] as libc::c_int,
                        hdr[9] as libc::c_int,
                        hdr[10] as libc::c_int,
                        hdr[11] as libc::c_int,
                    );
                    fclose((*c).f);
                    return -(1 as libc::c_int);
                }
            }
        }
    }
    *timebase.offset(0) = rl32(&mut *hdr.as_mut_ptr().offset(16));
    *timebase.offset(1) = rl32(&mut *hdr.as_mut_ptr().offset(20));
    let duration: libc::c_uint = rl32(&mut *hdr.as_mut_ptr().offset(24));
    let mut data: [uint8_t; 8] = [0; 8];
    (*c).broken = 0 as libc::c_int;
    *num_frames = 0 as libc::c_int as libc::c_uint;
    while !(fread(data.as_mut_ptr() as *mut libc::c_void, 4, 1, (*c).f)
        != 1 as libc::c_int as libc::c_ulong)
    {
        let mut sz: size_t = rl32(data.as_mut_ptr()) as size_t;
        if fread(data.as_mut_ptr() as *mut libc::c_void, 8, 1, (*c).f)
            != 1 as libc::c_int as libc::c_ulong
        {
            break;
        }
        let ts: uint64_t = rl64(data.as_mut_ptr()) as uint64_t;
        if *num_frames != 0 && ts <= (*c).last_ts {
            (*c).broken = 1 as libc::c_int;
        }
        (*c).last_ts = ts;
        fseeko((*c).f, sz as __off_t, 1 as libc::c_int);
        *num_frames = (*num_frames).wrapping_add(1);
    }
    let mut fps_num: uint64_t =
        (*timebase.offset(0) as uint64_t).wrapping_mul(*num_frames as uint64_t);
    let mut fps_den: uint64_t =
        (*timebase.offset(1) as uint64_t).wrapping_mul(duration as uint64_t);
    if fps_num != 0 && fps_den != 0 {
        let mut gcd: uint64_t = fps_num;
        let mut a: uint64_t = fps_den;
        let mut b: uint64_t = 0;
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
        *fps.offset(0) = fps_num as libc::c_uint;
        *fps.offset(1) = fps_den as libc::c_uint;
    } else {
        let ref mut fresh0 = *fps.offset(1);
        *fresh0 = 0 as libc::c_int as libc::c_uint;
        *fps.offset(0) = *fresh0;
    }
    (*c).timebase = *timebase.offset(0) as libc::c_double / *timebase.offset(1) as libc::c_double;
    (*c).step = duration.wrapping_div(*num_frames) as uint64_t;
    fseeko((*c).f, 32, 0);
    (*c).last_ts = 0 as libc::c_int as uint64_t;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn ivf_read_header(
    c: *mut IvfInputContext,
    sz: *mut ptrdiff_t,
    off_: *mut int64_t,
    ts: *mut uint64_t,
) -> libc::c_int {
    let mut data: [uint8_t; 8] = [0; 8];
    let off: int64_t = ftello((*c).f);
    if !off_.is_null() {
        *off_ = off;
    }
    if fread(data.as_mut_ptr() as *mut libc::c_void, 4, 1, (*c).f)
        != 1 as libc::c_int as libc::c_ulong
    {
        return -(1 as libc::c_int);
    }
    *sz = rl32(data.as_mut_ptr()) as ptrdiff_t;
    if (*c).broken == 0 {
        if fread(data.as_mut_ptr() as *mut libc::c_void, 8, 1, (*c).f)
            != 1 as libc::c_int as libc::c_ulong
        {
            return -(1 as libc::c_int);
        }
        *ts = rl64(data.as_mut_ptr()) as uint64_t;
    } else {
        if fseeko((*c).f, 8 as libc::c_int as __off_t, 1 as libc::c_int) != 0 {
            return -(1 as libc::c_int);
        }
        *ts = if off > 32 {
            ((*c).last_ts).wrapping_add((*c).step)
        } else {
            0
        };
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ivf_read(c: *mut IvfInputContext, buf: *mut Dav1dData) -> libc::c_int {
    let mut ptr: *mut uint8_t = 0 as *mut uint8_t;
    let mut sz: ptrdiff_t = 0;
    let mut off: int64_t = 0;
    let mut ts: uint64_t = 0;
    if ivf_read_header(c, &mut sz, &mut off, &mut ts) != 0 {
        return -(1 as libc::c_int);
    }
    ptr = dav1d_data_create(buf, sz as size_t);
    if ptr.is_null() {
        return -(1 as libc::c_int);
    }
    if fread(ptr as *mut libc::c_void, sz as size_t, 1, (*c).f) != 1 as libc::c_int as libc::c_ulong
    {
        fprintf(
            stderr,
            b"Failed to read frame data: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*errno_location()),
        );
        dav1d_data_unref(buf);
        return -(1 as libc::c_int);
    }
    (*buf).m.offset = off;
    (*buf).m.timestamp = ts as int64_t;
    (*c).last_ts = ts;
    return 0 as libc::c_int;
}
unsafe extern "C" fn ivf_seek(c: *mut IvfInputContext, pts: uint64_t) -> libc::c_int {
    let mut current_block: u64;
    let mut cur: uint64_t = 0;
    let ts: uint64_t = llround(pts as libc::c_double * (*c).timebase / 1000000000.0f64) as uint64_t;
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
                    stderr,
                    b"Failed to seek: %s\n\0" as *const u8 as *const libc::c_char,
                    strerror(*errno_location()),
                );
                return -(1 as libc::c_int);
            }
            _ => {
                let mut sz: ptrdiff_t = 0;
                if ivf_read_header(c, &mut sz, 0 as *mut int64_t, &mut cur) != 0 {
                    current_block = 679495355492430298;
                    continue;
                }
                if cur >= ts {
                    if fseeko((*c).f, -(12 as libc::c_int) as __off_t, 1 as libc::c_int) != 0 {
                        current_block = 679495355492430298;
                        continue;
                    }
                    return 0 as libc::c_int;
                } else {
                    if fseeko((*c).f, sz as __off_t, 1 as libc::c_int) != 0 {
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
pub static mut ivf_demuxer: Demuxer = {
    let mut init = Demuxer {
        priv_data_size: ::core::mem::size_of::<IvfInputContext>() as libc::c_ulong as libc::c_int,
        name: b"ivf\0" as *const u8 as *const libc::c_char,
        probe_sz: ::core::mem::size_of::<[uint8_t; 12]>() as libc::c_ulong as libc::c_int,
        probe: Some(ivf_probe as unsafe extern "C" fn(*const uint8_t) -> libc::c_int),
        open: Some(
            ivf_open
                as unsafe extern "C" fn(
                    *mut IvfInputContext,
                    *const libc::c_char,
                    *mut libc::c_uint,
                    *mut libc::c_uint,
                    *mut libc::c_uint,
                ) -> libc::c_int,
        ),
        read: Some(
            ivf_read as unsafe extern "C" fn(*mut IvfInputContext, *mut Dav1dData) -> libc::c_int,
        ),
        seek: Some(ivf_seek as unsafe extern "C" fn(*mut IvfInputContext, uint64_t) -> libc::c_int),
        close: Some(ivf_close as unsafe extern "C" fn(*mut IvfInputContext) -> ()),
    };
    init
};
