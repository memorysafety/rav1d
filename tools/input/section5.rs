use crate::errno_location;
use crate::include::stddef::*;
use crate::include::stdint::*;
use crate::stderr;
use ::libc;
extern "C" {
    pub type Dav1dRef;
    fn fclose(__stream: *mut libc::FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::FILE;
    fn fprintf(_: *mut libc::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(_: *mut libc::c_void, _: size_t, _: size_t, _: *mut libc::FILE) -> size_t;
    fn fseeko(__stream: *mut libc::FILE, __off: __off_t, __whence: libc::c_int) -> libc::c_int;
    fn feof(__stream: *mut libc::FILE) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn dav1d_data_unref(data: *mut Dav1dData);
    fn dav1d_data_create(data: *mut Dav1dData, sz: size_t) -> *mut uint8_t;
}
use crate::include::dav1d::headers::Dav1dObuType;
use crate::include::dav1d::headers::DAV1D_OBU_TD;
use crate::include::sys::types::__off_t;

use crate::include::dav1d::data::Dav1dData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DemuxerPriv {
    pub f: *mut libc::FILE,
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
pub type Section5InputContext = DemuxerPriv;
unsafe extern "C" fn leb(
    mut ptr: *const uint8_t,
    mut sz: libc::c_int,
    len: *mut size_t,
) -> libc::c_int {
    let mut val: uint64_t = 0 as libc::c_int as uint64_t;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut more: libc::c_uint = 0;
    loop {
        let fresh0 = sz;
        sz = sz - 1;
        if fresh0 == 0 {
            return -(1 as libc::c_int);
        }
        let fresh1 = ptr;
        ptr = ptr.offset(1);
        let v = *fresh1 as libc::c_int;
        more = (v & 0x80 as libc::c_int) as libc::c_uint;
        val |= ((v & 0x7f as libc::c_int) as uint64_t)
            << i.wrapping_mul(7 as libc::c_int as libc::c_uint);
        i = i.wrapping_add(1);
        if !(more != 0 && i < 8 as libc::c_uint) {
            break;
        }
    }
    if val > u32::MAX as uint64_t || more != 0 {
        return -1;
    }
    *len = val as usize;
    return i as libc::c_int;
}
#[inline]
unsafe extern "C" fn parse_obu_header(
    mut buf: *const uint8_t,
    mut buf_size: libc::c_int,
    obu_size: *mut size_t,
    type_0: *mut Dav1dObuType,
    allow_implicit_size: libc::c_int,
) -> libc::c_int {
    let mut ret = 0;
    let mut extension_flag = 0;
    let mut has_size_flag = 0;
    if buf_size == 0 {
        return -(1 as libc::c_int);
    }
    if *buf as libc::c_int & 0x80 as libc::c_int != 0 {
        return -(1 as libc::c_int);
    }
    *type_0 = ((*buf as libc::c_int & 0x78 as libc::c_int) >> 3) as Dav1dObuType;
    extension_flag = (*buf as libc::c_int & 0x4 as libc::c_int) >> 2;
    has_size_flag = (*buf as libc::c_int & 0x2 as libc::c_int) >> 1;
    buf = buf.offset(1);
    buf_size -= 1;
    if extension_flag != 0 {
        buf = buf.offset(1);
        buf_size -= 1;
    }
    if has_size_flag != 0 {
        ret = leb(buf, buf_size, obu_size);
        if ret < 0 {
            return -(1 as libc::c_int);
        }
        return *obu_size as libc::c_int + ret + 1 + extension_flag;
    } else {
        if allow_implicit_size == 0 {
            return -(1 as libc::c_int);
        }
    }
    *obu_size = buf_size as size_t;
    return buf_size + 1 + extension_flag;
}
unsafe extern "C" fn leb128(f: *mut libc::FILE, len: *mut size_t) -> libc::c_int {
    let mut val: uint64_t = 0 as libc::c_int as uint64_t;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut more: libc::c_uint = 0;
    loop {
        let mut v: uint8_t = 0;
        if fread(&mut v as *mut uint8_t as *mut libc::c_void, 1, 1, f) < 1 {
            return -(1 as libc::c_int);
        }
        more = (v as libc::c_int & 0x80 as libc::c_int) as libc::c_uint;
        val |= ((v as libc::c_int & 0x7f as libc::c_int) as uint64_t)
            << i.wrapping_mul(7 as libc::c_int as libc::c_uint);
        i = i.wrapping_add(1);
        if !(more != 0 && i < 8 as libc::c_uint) {
            break;
        }
    }
    if val > u32::MAX as uint64_t || more != 0 {
        return -1;
    }
    *len = val as usize;
    return i as libc::c_int;
}
unsafe extern "C" fn section5_probe(mut data: *const uint8_t) -> libc::c_int {
    let mut ret = 0;
    let mut cnt = 0;
    let mut obu_size: size_t = 0;
    let mut type_0: Dav1dObuType = 0 as Dav1dObuType;
    ret = parse_obu_header(
        data.offset(cnt as isize),
        2048 - cnt,
        &mut obu_size,
        &mut type_0,
        0 as libc::c_int,
    );
    if ret < 0
        || type_0 as libc::c_uint != DAV1D_OBU_TD as libc::c_int as libc::c_uint
        || obu_size > 0
    {
        return 0 as libc::c_int;
    }
    cnt += ret;
    let mut seq = 0;
    while cnt < 2048 {
        ret = parse_obu_header(
            data.offset(cnt as isize),
            2048 - cnt,
            &mut obu_size,
            &mut type_0,
            0 as libc::c_int,
        );
        if ret < 0 {
            return 0 as libc::c_int;
        }
        cnt += ret;
        match type_0 as libc::c_uint {
            1 => {
                seq = 1 as libc::c_int;
            }
            6 | 3 => return seq,
            2 | 4 => return 0 as libc::c_int,
            _ => {}
        }
    }
    return seq;
}
unsafe extern "C" fn section5_open(
    c: *mut Section5InputContext,
    file: *const libc::c_char,
    mut fps: *mut libc::c_uint,
    num_frames: *mut libc::c_uint,
    mut timebase: *mut libc::c_uint,
) -> libc::c_int {
    (*c).f = fopen(file, b"rb\0" as *const u8 as *const libc::c_char);
    if ((*c).f).is_null() {
        fprintf(
            stderr,
            b"Failed to open %s: %s\n\0" as *const u8 as *const libc::c_char,
            file,
            strerror(*errno_location()),
        );
        return -(1 as libc::c_int);
    }
    *fps.offset(0) = 25 as libc::c_int as libc::c_uint;
    *fps.offset(1) = 1 as libc::c_int as libc::c_uint;
    *timebase.offset(0) = 25 as libc::c_int as libc::c_uint;
    *timebase.offset(1) = 1 as libc::c_int as libc::c_uint;
    *num_frames = 0 as libc::c_int as libc::c_uint;
    loop {
        let mut byte: [uint8_t; 2] = [0; 2];
        if fread(
            &mut *byte.as_mut_ptr().offset(0) as *mut uint8_t as *mut libc::c_void,
            1,
            1,
            (*c).f,
        ) < 1
        {
            break;
        }
        let obu_type: Dav1dObuType =
            (byte[0] as libc::c_int >> 3 & 0xf as libc::c_int) as Dav1dObuType;
        if obu_type as libc::c_uint == DAV1D_OBU_TD as libc::c_int as libc::c_uint {
            *num_frames = (*num_frames).wrapping_add(1);
        }
        let has_length_field = byte[0] as libc::c_int & 0x2 as libc::c_int;
        if has_length_field == 0 {
            return -(1 as libc::c_int);
        }
        let has_extension = byte[0] as libc::c_int & 0x4 as libc::c_int;
        if has_extension != 0
            && fread(
                &mut *byte.as_mut_ptr().offset(1) as *mut uint8_t as *mut libc::c_void,
                1,
                1,
                (*c).f,
            ) < 1
        {
            return -(1 as libc::c_int);
        }
        let mut len: size_t = 0;
        let res = leb128((*c).f, &mut len);
        if res < 0 {
            return -(1 as libc::c_int);
        }
        fseeko((*c).f, len as __off_t, 1 as libc::c_int);
    }
    fseeko((*c).f, 0, 0 as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn section5_read(
    c: *mut Section5InputContext,
    data: *mut Dav1dData,
) -> libc::c_int {
    let mut total_bytes: size_t = 0 as libc::c_int as size_t;
    let mut first = 1;
    loop {
        let mut byte: [uint8_t; 2] = [0; 2];
        if fread(
            &mut *byte.as_mut_ptr().offset(0) as *mut uint8_t as *mut libc::c_void,
            1,
            1,
            (*c).f,
        ) < 1
        {
            if first == 0 && feof((*c).f) != 0 {
                break;
            }
            return -(1 as libc::c_int);
        } else {
            let obu_type: Dav1dObuType =
                (byte[0] as libc::c_int >> 3 & 0xf as libc::c_int) as Dav1dObuType;
            if first != 0 {
                if obu_type as libc::c_uint != DAV1D_OBU_TD as libc::c_int as libc::c_uint {
                    return -(1 as libc::c_int);
                }
            } else if obu_type as libc::c_uint == DAV1D_OBU_TD as libc::c_int as libc::c_uint {
                fseeko((*c).f, -(1 as libc::c_int) as __off_t, 1);
                break;
            }
            let has_length_field = byte[0] as libc::c_int & 0x2 as libc::c_int;
            if has_length_field == 0 {
                return -(1 as libc::c_int);
            }
            let has_extension = (byte[0] as libc::c_int & 0x4 as libc::c_int != 0) as libc::c_int;
            if has_extension != 0
                && fread(
                    &mut *byte.as_mut_ptr().offset(1) as *mut uint8_t as *mut libc::c_void,
                    1,
                    1,
                    (*c).f,
                ) < 1
            {
                return -(1 as libc::c_int);
            }
            let mut len: size_t = 0;
            let res = leb128((*c).f, &mut len);
            if res < 0 {
                return -(1 as libc::c_int);
            }
            total_bytes =
                total_bytes.wrapping_add(((1 + has_extension + res) as size_t).wrapping_add(len));
            fseeko((*c).f, len as __off_t, 1 as libc::c_int);
            first = 0 as libc::c_int;
        }
    }
    fseeko((*c).f, -(total_bytes as __off_t), 1 as libc::c_int);
    let mut ptr: *mut uint8_t = dav1d_data_create(data, total_bytes);
    if ptr.is_null() {
        return -(1 as libc::c_int);
    }
    if fread(ptr as *mut libc::c_void, total_bytes, 1, (*c).f) != 1 {
        fprintf(
            stderr,
            b"Failed to read frame data: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*errno_location()),
        );
        dav1d_data_unref(data);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn section5_close(c: *mut Section5InputContext) {
    fclose((*c).f);
}
#[no_mangle]
pub static mut section5_demuxer: Demuxer = {
    let mut init = Demuxer {
        priv_data_size: ::core::mem::size_of::<Section5InputContext>() as libc::c_ulong
            as libc::c_int,
        name: b"section5\0" as *const u8 as *const libc::c_char,
        probe_sz: 2048 as libc::c_int,
        probe: Some(section5_probe as unsafe extern "C" fn(*const uint8_t) -> libc::c_int),
        open: Some(
            section5_open
                as unsafe extern "C" fn(
                    *mut Section5InputContext,
                    *const libc::c_char,
                    *mut libc::c_uint,
                    *mut libc::c_uint,
                    *mut libc::c_uint,
                ) -> libc::c_int,
        ),
        read: Some(
            section5_read
                as unsafe extern "C" fn(*mut Section5InputContext, *mut Dav1dData) -> libc::c_int,
        ),
        seek: None,
        close: Some(section5_close as unsafe extern "C" fn(*mut Section5InputContext) -> ()),
    };
    init
};
