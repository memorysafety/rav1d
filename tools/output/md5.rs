use crate::compat::errno::errno_location;
use crate::compat::stdio::stderr;
use crate::compat::stdio::stdout;
use libc::fclose;
use libc::fopen;
use libc::fprintf;
use libc::memcpy;
use libc::strcmp;
use libc::strerror;
use libc::strlen;
use libc::strtoul;
use rav1d::dav1d_picture_unref;
use rav1d::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I400;
use rav1d::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;
use rav1d::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;
use rav1d::include::dav1d::picture::Dav1dPicture;
use rav1d::include::dav1d::picture::Dav1dPictureParameters;
use std::cmp;
use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_ulong;
use std::ffi::c_void;
use std::ptr;
use std::ptr::NonNull;

#[repr(C)]
pub struct MuxerPriv {
    pub abcd: [u32; 4],
    pub c2rust_unnamed: C2RustUnnamed,
    pub len: u64,
    pub f: *mut libc::FILE,
}

#[repr(C)]
pub union C2RustUnnamed {
    pub data: [u8; 64],
    pub data32: [u32; 16],
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

pub type MD5Context = MuxerPriv;

static k: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x2441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x4881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

unsafe extern "C" fn md5_open(
    md5: *mut MD5Context,
    file: *const c_char,
    _p: *const Dav1dPictureParameters,
    mut _fps: *const c_uint,
) -> c_int {
    if strcmp(file, b"-\0" as *const u8 as *const c_char) == 0 {
        (*md5).f = stdout();
    } else {
        (*md5).f = fopen(file, b"wb\0" as *const u8 as *const c_char);
        if ((*md5).f).is_null() {
            fprintf(
                stderr(),
                b"Failed to open %s: %s\n\0" as *const u8 as *const c_char,
                file,
                strerror(*errno_location()),
            );
            return -1;
        }
    }
    (*md5).abcd[0] = 0x67452301 as c_int as u32;
    (*md5).abcd[1] = 0xefcdab89 as c_uint;
    (*md5).abcd[2] = 0x98badcfe as c_uint;
    (*md5).abcd[3] = 0x10325476 as c_int as u32;
    (*md5).len = 0 as c_int as u64;
    return 0 as c_int;
}

#[inline]
unsafe fn leftrotate(x: u32, c: c_int) -> u32 {
    return x << c | x >> 32 - c;
}

unsafe fn md5_body(md5: *mut MD5Context, data: *const u32) {
    let mut a: u32 = (*md5).abcd[0];
    let mut b: u32 = (*md5).abcd[1];
    let mut c: u32 = (*md5).abcd[2];
    let mut d: u32 = (*md5).abcd[3];
    a = b.wrapping_add(leftrotate(
        a.wrapping_add(b & c | !b & d)
            .wrapping_add(k[(0 + 0) as usize])
            .wrapping_add(*data.offset((0 + 0) as isize)),
        7 as c_int,
    ));
    d = a.wrapping_add(leftrotate(
        d.wrapping_add(a & b | !a & c)
            .wrapping_add(k[(0 + 1) as usize])
            .wrapping_add(*data.offset((0 + 1) as isize)),
        12 as c_int,
    ));
    c = d.wrapping_add(leftrotate(
        c.wrapping_add(d & a | !d & b)
            .wrapping_add(k[(0 + 2) as usize])
            .wrapping_add(*data.offset((0 + 2) as isize)),
        17 as c_int,
    ));
    b = c.wrapping_add(leftrotate(
        b.wrapping_add(c & d | !c & a)
            .wrapping_add(k[(0 + 3) as usize])
            .wrapping_add(*data.offset((0 + 3) as isize)),
        22 as c_int,
    ));
    a = b.wrapping_add(leftrotate(
        a.wrapping_add(b & c | !b & d)
            .wrapping_add(k[(4 + 0) as usize])
            .wrapping_add(*data.offset((4 + 0) as isize)),
        7 as c_int,
    ));
    d = a.wrapping_add(leftrotate(
        d.wrapping_add(a & b | !a & c)
            .wrapping_add(k[(4 + 1) as usize])
            .wrapping_add(*data.offset((4 + 1) as isize)),
        12 as c_int,
    ));
    c = d.wrapping_add(leftrotate(
        c.wrapping_add(d & a | !d & b)
            .wrapping_add(k[(4 + 2) as usize])
            .wrapping_add(*data.offset((4 + 2) as isize)),
        17 as c_int,
    ));
    b = c.wrapping_add(leftrotate(
        b.wrapping_add(c & d | !c & a)
            .wrapping_add(k[(4 + 3) as usize])
            .wrapping_add(*data.offset((4 + 3) as isize)),
        22 as c_int,
    ));
    a = b.wrapping_add(leftrotate(
        a.wrapping_add(b & c | !b & d)
            .wrapping_add(k[(8 + 0) as usize])
            .wrapping_add(*data.offset((8 + 0) as isize)),
        7 as c_int,
    ));
    d = a.wrapping_add(leftrotate(
        d.wrapping_add(a & b | !a & c)
            .wrapping_add(k[(8 + 1) as usize])
            .wrapping_add(*data.offset((8 + 1) as isize)),
        12 as c_int,
    ));
    c = d.wrapping_add(leftrotate(
        c.wrapping_add(d & a | !d & b)
            .wrapping_add(k[(8 + 2) as usize])
            .wrapping_add(*data.offset((8 + 2) as isize)),
        17 as c_int,
    ));
    b = c.wrapping_add(leftrotate(
        b.wrapping_add(c & d | !c & a)
            .wrapping_add(k[(8 + 3) as usize])
            .wrapping_add(*data.offset((8 + 3) as isize)),
        22 as c_int,
    ));
    a = b.wrapping_add(leftrotate(
        a.wrapping_add(b & c | !b & d)
            .wrapping_add(k[(12 + 0) as usize])
            .wrapping_add(*data.offset((12 + 0) as isize)),
        7 as c_int,
    ));
    d = a.wrapping_add(leftrotate(
        d.wrapping_add(a & b | !a & c)
            .wrapping_add(k[(12 + 1) as usize])
            .wrapping_add(*data.offset((12 + 1) as isize)),
        12 as c_int,
    ));
    c = d.wrapping_add(leftrotate(
        c.wrapping_add(d & a | !d & b)
            .wrapping_add(k[(12 + 2) as usize])
            .wrapping_add(*data.offset((12 + 2) as isize)),
        17 as c_int,
    ));
    b = c.wrapping_add(leftrotate(
        b.wrapping_add(c & d | !c & a)
            .wrapping_add(k[(12 + 3) as usize])
            .wrapping_add(*data.offset((12 + 3) as isize)),
        22 as c_int,
    ));
    a = b.wrapping_add(leftrotate(
        a.wrapping_add(d & b | !d & c)
            .wrapping_add(k[(16 + 0) as usize])
            .wrapping_add(*data.offset((16 + 1 & 15) as isize)),
        5 as c_int,
    ));
    d = a.wrapping_add(leftrotate(
        d.wrapping_add(c & a | !c & b)
            .wrapping_add(k[(16 + 1) as usize])
            .wrapping_add(*data.offset((16 + 6 & 15) as isize)),
        9 as c_int,
    ));
    c = d.wrapping_add(leftrotate(
        c.wrapping_add(b & d | !b & a)
            .wrapping_add(k[(16 + 2) as usize])
            .wrapping_add(*data.offset((16 + 11 & 15) as isize)),
        14 as c_int,
    ));
    b = c.wrapping_add(leftrotate(
        b.wrapping_add(a & c | !a & d)
            .wrapping_add(k[(16 + 3) as usize])
            .wrapping_add(*data.offset((16 + 0 & 15) as isize)),
        20 as c_int,
    ));
    a = b.wrapping_add(leftrotate(
        a.wrapping_add(d & b | !d & c)
            .wrapping_add(k[(20 + 0) as usize])
            .wrapping_add(*data.offset((20 + 1 & 15) as isize)),
        5 as c_int,
    ));
    d = a.wrapping_add(leftrotate(
        d.wrapping_add(c & a | !c & b)
            .wrapping_add(k[(20 + 1) as usize])
            .wrapping_add(*data.offset((20 + 6 & 15) as isize)),
        9 as c_int,
    ));
    c = d.wrapping_add(leftrotate(
        c.wrapping_add(b & d | !b & a)
            .wrapping_add(k[(20 + 2) as usize])
            .wrapping_add(*data.offset((20 + 11 & 15) as isize)),
        14 as c_int,
    ));
    b = c.wrapping_add(leftrotate(
        b.wrapping_add(a & c | !a & d)
            .wrapping_add(k[(20 + 3) as usize])
            .wrapping_add(*data.offset((20 + 0 & 15) as isize)),
        20 as c_int,
    ));
    a = b.wrapping_add(leftrotate(
        a.wrapping_add(d & b | !d & c)
            .wrapping_add(k[(24 + 0) as usize])
            .wrapping_add(*data.offset((24 + 1 & 15) as isize)),
        5 as c_int,
    ));
    d = a.wrapping_add(leftrotate(
        d.wrapping_add(c & a | !c & b)
            .wrapping_add(k[(24 + 1) as usize])
            .wrapping_add(*data.offset((24 + 6 & 15) as isize)),
        9 as c_int,
    ));
    c = d.wrapping_add(leftrotate(
        c.wrapping_add(b & d | !b & a)
            .wrapping_add(k[(24 + 2) as usize])
            .wrapping_add(*data.offset((24 + 11 & 15) as isize)),
        14 as c_int,
    ));
    b = c.wrapping_add(leftrotate(
        b.wrapping_add(a & c | !a & d)
            .wrapping_add(k[(24 + 3) as usize])
            .wrapping_add(*data.offset((24 + 0 & 15) as isize)),
        20 as c_int,
    ));
    a = b.wrapping_add(leftrotate(
        a.wrapping_add(d & b | !d & c)
            .wrapping_add(k[(28 + 0) as usize])
            .wrapping_add(*data.offset((28 + 1 & 15) as isize)),
        5 as c_int,
    ));
    d = a.wrapping_add(leftrotate(
        d.wrapping_add(c & a | !c & b)
            .wrapping_add(k[(28 + 1) as usize])
            .wrapping_add(*data.offset((28 + 6 & 15) as isize)),
        9 as c_int,
    ));
    c = d.wrapping_add(leftrotate(
        c.wrapping_add(b & d | !b & a)
            .wrapping_add(k[(28 + 2) as usize])
            .wrapping_add(*data.offset((28 + 11 & 15) as isize)),
        14 as c_int,
    ));
    b = c.wrapping_add(leftrotate(
        b.wrapping_add(a & c | !a & d)
            .wrapping_add(k[(28 + 3) as usize])
            .wrapping_add(*data.offset((28 + 0 & 15) as isize)),
        20 as c_int,
    ));
    a = b.wrapping_add(leftrotate(
        a.wrapping_add(b ^ c ^ d)
            .wrapping_add(k[(32 + 0) as usize])
            .wrapping_add(*data.offset((5 - 32 & 15) as isize)),
        4 as c_int,
    ));
    d = a.wrapping_add(leftrotate(
        d.wrapping_add(a ^ b ^ c)
            .wrapping_add(k[(32 + 1) as usize])
            .wrapping_add(*data.offset((8 - 32 & 15) as isize)),
        11 as c_int,
    ));
    c = d.wrapping_add(leftrotate(
        c.wrapping_add(d ^ a ^ b)
            .wrapping_add(k[(32 + 2) as usize])
            .wrapping_add(*data.offset((11 - 32 & 15) as isize)),
        16 as c_int,
    ));
    b = c.wrapping_add(leftrotate(
        b.wrapping_add(c ^ d ^ a)
            .wrapping_add(k[(32 + 3) as usize])
            .wrapping_add(*data.offset((14 - 32 & 15) as isize)),
        23 as c_int,
    ));
    a = b.wrapping_add(leftrotate(
        a.wrapping_add(b ^ c ^ d)
            .wrapping_add(k[(36 + 0) as usize])
            .wrapping_add(*data.offset((5 - 36 & 15) as isize)),
        4 as c_int,
    ));
    d = a.wrapping_add(leftrotate(
        d.wrapping_add(a ^ b ^ c)
            .wrapping_add(k[(36 + 1) as usize])
            .wrapping_add(*data.offset((8 - 36 & 15) as isize)),
        11 as c_int,
    ));
    c = d.wrapping_add(leftrotate(
        c.wrapping_add(d ^ a ^ b)
            .wrapping_add(k[(36 + 2) as usize])
            .wrapping_add(*data.offset((11 - 36 & 15) as isize)),
        16 as c_int,
    ));
    b = c.wrapping_add(leftrotate(
        b.wrapping_add(c ^ d ^ a)
            .wrapping_add(k[(36 + 3) as usize])
            .wrapping_add(*data.offset((14 - 36 & 15) as isize)),
        23 as c_int,
    ));
    a = b.wrapping_add(leftrotate(
        a.wrapping_add(b ^ c ^ d)
            .wrapping_add(k[(40 + 0) as usize])
            .wrapping_add(*data.offset((5 - 40 & 15) as isize)),
        4 as c_int,
    ));
    d = a.wrapping_add(leftrotate(
        d.wrapping_add(a ^ b ^ c)
            .wrapping_add(k[(40 + 1) as usize])
            .wrapping_add(*data.offset((8 - 40 & 15) as isize)),
        11 as c_int,
    ));
    c = d.wrapping_add(leftrotate(
        c.wrapping_add(d ^ a ^ b)
            .wrapping_add(k[(40 + 2) as usize])
            .wrapping_add(*data.offset((11 - 40 & 15) as isize)),
        16 as c_int,
    ));
    b = c.wrapping_add(leftrotate(
        b.wrapping_add(c ^ d ^ a)
            .wrapping_add(k[(40 + 3) as usize])
            .wrapping_add(*data.offset((14 - 40 & 15) as isize)),
        23 as c_int,
    ));
    a = b.wrapping_add(leftrotate(
        a.wrapping_add(b ^ c ^ d)
            .wrapping_add(k[(44 + 0) as usize])
            .wrapping_add(*data.offset((5 - 44 & 15) as isize)),
        4 as c_int,
    ));
    d = a.wrapping_add(leftrotate(
        d.wrapping_add(a ^ b ^ c)
            .wrapping_add(k[(44 + 1) as usize])
            .wrapping_add(*data.offset((8 - 44 & 15) as isize)),
        11 as c_int,
    ));
    c = d.wrapping_add(leftrotate(
        c.wrapping_add(d ^ a ^ b)
            .wrapping_add(k[(44 + 2) as usize])
            .wrapping_add(*data.offset((11 - 44 & 15) as isize)),
        16 as c_int,
    ));
    b = c.wrapping_add(leftrotate(
        b.wrapping_add(c ^ d ^ a)
            .wrapping_add(k[(44 + 3) as usize])
            .wrapping_add(*data.offset((14 - 44 & 15) as isize)),
        23 as c_int,
    ));
    a = b.wrapping_add(leftrotate(
        a.wrapping_add(c ^ (b | !d))
            .wrapping_add(k[(48 + 0) as usize])
            .wrapping_add(*data.offset((0 - 48 & 15) as isize)),
        6 as c_int,
    ));
    d = a.wrapping_add(leftrotate(
        d.wrapping_add(b ^ (a | !c))
            .wrapping_add(k[(48 + 1) as usize])
            .wrapping_add(*data.offset((7 - 48 & 15) as isize)),
        10 as c_int,
    ));
    c = d.wrapping_add(leftrotate(
        c.wrapping_add(a ^ (d | !b))
            .wrapping_add(k[(48 + 2) as usize])
            .wrapping_add(*data.offset((14 - 48 & 15) as isize)),
        15 as c_int,
    ));
    b = c.wrapping_add(leftrotate(
        b.wrapping_add(d ^ (c | !a))
            .wrapping_add(k[(48 + 3) as usize])
            .wrapping_add(*data.offset((5 - 48 & 15) as isize)),
        21 as c_int,
    ));
    a = b.wrapping_add(leftrotate(
        a.wrapping_add(c ^ (b | !d))
            .wrapping_add(k[(52 + 0) as usize])
            .wrapping_add(*data.offset((0 - 52 & 15) as isize)),
        6 as c_int,
    ));
    d = a.wrapping_add(leftrotate(
        d.wrapping_add(b ^ (a | !c))
            .wrapping_add(k[(52 + 1) as usize])
            .wrapping_add(*data.offset((7 - 52 & 15) as isize)),
        10 as c_int,
    ));
    c = d.wrapping_add(leftrotate(
        c.wrapping_add(a ^ (d | !b))
            .wrapping_add(k[(52 + 2) as usize])
            .wrapping_add(*data.offset((14 - 52 & 15) as isize)),
        15 as c_int,
    ));
    b = c.wrapping_add(leftrotate(
        b.wrapping_add(d ^ (c | !a))
            .wrapping_add(k[(52 + 3) as usize])
            .wrapping_add(*data.offset((5 - 52 & 15) as isize)),
        21 as c_int,
    ));
    a = b.wrapping_add(leftrotate(
        a.wrapping_add(c ^ (b | !d))
            .wrapping_add(k[(56 + 0) as usize])
            .wrapping_add(*data.offset((0 - 56 & 15) as isize)),
        6 as c_int,
    ));
    d = a.wrapping_add(leftrotate(
        d.wrapping_add(b ^ (a | !c))
            .wrapping_add(k[(56 + 1) as usize])
            .wrapping_add(*data.offset((7 - 56 & 15) as isize)),
        10 as c_int,
    ));
    c = d.wrapping_add(leftrotate(
        c.wrapping_add(a ^ (d | !b))
            .wrapping_add(k[(56 + 2) as usize])
            .wrapping_add(*data.offset((14 - 56 & 15) as isize)),
        15 as c_int,
    ));
    b = c.wrapping_add(leftrotate(
        b.wrapping_add(d ^ (c | !a))
            .wrapping_add(k[(56 + 3) as usize])
            .wrapping_add(*data.offset((5 - 56 & 15) as isize)),
        21 as c_int,
    ));
    a = b.wrapping_add(leftrotate(
        a.wrapping_add(c ^ (b | !d))
            .wrapping_add(k[(60 + 0) as usize])
            .wrapping_add(*data.offset((0 - 60 & 15) as isize)),
        6 as c_int,
    ));
    d = a.wrapping_add(leftrotate(
        d.wrapping_add(b ^ (a | !c))
            .wrapping_add(k[(60 + 1) as usize])
            .wrapping_add(*data.offset((7 - 60 & 15) as isize)),
        10 as c_int,
    ));
    c = d.wrapping_add(leftrotate(
        c.wrapping_add(a ^ (d | !b))
            .wrapping_add(k[(60 + 2) as usize])
            .wrapping_add(*data.offset((14 - 60 & 15) as isize)),
        15 as c_int,
    ));
    b = c.wrapping_add(leftrotate(
        b.wrapping_add(d ^ (c | !a))
            .wrapping_add(k[(60 + 3) as usize])
            .wrapping_add(*data.offset((5 - 60 & 15) as isize)),
        21 as c_int,
    ));
    (*md5).abcd[0] = ((*md5).abcd[0] as c_uint).wrapping_add(a) as u32 as u32;
    (*md5).abcd[1] = ((*md5).abcd[1] as c_uint).wrapping_add(b) as u32 as u32;
    (*md5).abcd[2] = ((*md5).abcd[2] as c_uint).wrapping_add(c) as u32 as u32;
    (*md5).abcd[3] = ((*md5).abcd[3] as c_uint).wrapping_add(d) as u32 as u32;
}

unsafe fn md5_update(md5: *mut MD5Context, mut data: *const u8, mut len: c_uint) {
    if len == 0 {
        return;
    }
    if ((*md5).len & 63) != 0 {
        let tmp: c_uint = cmp::min(len, 64 - ((*md5).len & 63) as c_uint);
        memcpy(
            &mut *((*md5).c2rust_unnamed.data)
                .as_mut_ptr()
                .offset(((*md5).len & 63) as isize) as *mut u8 as *mut c_void,
            data as *const c_void,
            tmp as usize,
        );
        len = len.wrapping_sub(tmp);
        data = data.offset(tmp as isize);
        (*md5).len = ((*md5).len as c_ulong).wrapping_add(tmp as c_ulong) as u64 as u64;
        if ((*md5).len & 63) == 0 {
            md5_body(md5, ((*md5).c2rust_unnamed.data32).as_mut_ptr());
        }
    }
    while len >= 64 as c_uint {
        memcpy(
            ((*md5).c2rust_unnamed.data).as_mut_ptr() as *mut c_void,
            data as *const c_void,
            64,
        );
        md5_body(md5, ((*md5).c2rust_unnamed.data32).as_mut_ptr());
        (*md5).len = ((*md5).len as c_ulong).wrapping_add(64 as c_int as c_ulong) as u64 as u64;
        data = data.offset(64);
        len = len.wrapping_sub(64 as c_int as c_uint);
    }
    if len != 0 {
        memcpy(
            ((*md5).c2rust_unnamed.data).as_mut_ptr() as *mut c_void,
            data as *const c_void,
            len as usize,
        );
        (*md5).len = ((*md5).len as c_ulong).wrapping_add(len as c_ulong) as u64 as u64;
    }
}

unsafe extern "C" fn md5_write(md5: *mut MD5Context, p: *mut Dav1dPicture) -> c_int {
    let hbd = ((*p).p.bpc > 8) as c_int;
    let w = (*p).p.w;
    let h = (*p).p.h;
    let mut yptr: *mut u8 = (*p).data[0].map_or_else(ptr::null_mut, NonNull::as_ptr) as *mut u8;
    let mut y = 0;
    while y < h {
        md5_update(md5, yptr, (w << hbd) as c_uint);
        yptr = yptr.offset((*p).stride[0] as isize);
        y += 1;
    }
    if (*p).p.layout as c_uint != DAV1D_PIXEL_LAYOUT_I400 as c_int as c_uint {
        let ss_ver =
            ((*p).p.layout as c_uint == DAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint) as c_int;
        let ss_hor =
            ((*p).p.layout as c_uint != DAV1D_PIXEL_LAYOUT_I444 as c_int as c_uint) as c_int;
        let cw = w + ss_hor >> ss_hor;
        let ch = h + ss_ver >> ss_ver;
        let mut pl = 1;
        while pl <= 2 {
            let mut uvptr: *mut u8 =
                (*p).data[pl as usize].map_or_else(ptr::null_mut, NonNull::as_ptr) as *mut u8;
            let mut y_0 = 0;
            while y_0 < ch {
                md5_update(md5, uvptr, (cw << hbd) as c_uint);
                uvptr = uvptr.offset((*p).stride[1] as isize);
                y_0 += 1;
            }
            pl += 1;
        }
    }
    dav1d_picture_unref(NonNull::new(p));
    return 0 as c_int;
}

unsafe fn md5_finish(md5: *mut MD5Context) {
    static bit: [u8; 2] = [0x80, 0];
    let len: u64 = (*md5).len << 3;
    md5_update(md5, &*bit.as_ptr().offset(0), 1 as c_int as c_uint);
    while ((*md5).len & 63) != 56 {
        md5_update(md5, &*bit.as_ptr().offset(1), 1 as c_int as c_uint);
    }
    md5_update(md5, &len as *const u64 as *const u8, 8 as c_int as c_uint);
}

unsafe extern "C" fn md5_close(md5: *mut MD5Context) {
    md5_finish(md5);
    let mut i = 0;
    while i < 4 {
        fprintf(
            (*md5).f,
            b"%2.2x%2.2x%2.2x%2.2x\0" as *const u8 as *const c_char,
            (*md5).abcd[i as usize] & 0xff as c_int as c_uint,
            (*md5).abcd[i as usize] >> 8 & 0xff as c_int as c_uint,
            (*md5).abcd[i as usize] >> 16 & 0xff as c_int as c_uint,
            (*md5).abcd[i as usize] >> 24,
        );
        i += 1;
    }
    fprintf((*md5).f, b"\n\0" as *const u8 as *const c_char);
    if (*md5).f != stdout() {
        fclose((*md5).f);
    }
}

unsafe extern "C" fn md5_verify(md5: *mut MD5Context, mut md5_str: *const c_char) -> c_int {
    md5_finish(md5);
    if strlen(md5_str) < 32 {
        return -1;
    }
    let mut abcd: [u32; 4] = [0 as c_int as u32, 0, 0, 0];
    let mut t: [c_char; 3] = [0 as c_int as c_char, 0, 0];
    let mut i = 0;
    while i < 4 {
        let mut j = 0;
        while j < 32 {
            let mut ignore: *mut c_char = 0 as *mut c_char;
            memcpy(t.as_mut_ptr() as *mut c_void, md5_str as *const c_void, 2);
            md5_str = md5_str.offset(2);
            abcd[i as usize] |= (strtoul(t.as_mut_ptr(), &mut ignore, 16 as c_int) as u32) << j;
            j += 8 as c_int;
        }
        i += 1;
    }
    (abcd != (*md5).abcd) as c_int
}

#[no_mangle]
pub static mut md5_muxer: Muxer = Muxer {
    priv_data_size: ::core::mem::size_of::<MD5Context>() as c_ulong as c_int,
    name: b"md5\0" as *const u8 as *const c_char,
    extension: b"md5\0" as *const u8 as *const c_char,
    write_header: Some(md5_open),
    write_picture: Some(md5_write),
    write_trailer: Some(md5_close),
    verify: Some(md5_verify),
};
