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
    fn fclose(__stream: *mut libc::FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::FILE;
    fn fprintf(_: *mut libc::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut libc::FILE,
    ) -> libc::c_ulong;
    fn fseeko(
        __stream: *mut libc::FILE,
        __off: __off64_t,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn dav1d_data_create(data: *mut Dav1dData, sz: size_t) -> *mut uint8_t;
    fn dav1d_data_unref(data: *mut Dav1dData);
}



pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type Dav1dObuType = libc::c_uint;
pub const DAV1D_OBU_PADDING: Dav1dObuType = 15;
pub const DAV1D_OBU_REDUNDANT_FRAME_HDR: Dav1dObuType = 7;
pub const DAV1D_OBU_FRAME: Dav1dObuType = 6;
pub const DAV1D_OBU_METADATA: Dav1dObuType = 5;
pub const DAV1D_OBU_TILE_GRP: Dav1dObuType = 4;
pub const DAV1D_OBU_FRAME_HDR: Dav1dObuType = 3;
pub const DAV1D_OBU_TD: Dav1dObuType = 2;
pub const DAV1D_OBU_SEQ_HDR: Dav1dObuType = 1;
use crate::include::dav1d::common::Dav1dUserData;
use crate::include::dav1d::common::Dav1dDataProps;
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
pub struct DemuxerPriv {
    pub f: *mut libc::FILE,
    pub temporal_unit_size: size_t,
    pub frame_unit_size: size_t,
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
pub type AnnexbInputContext = DemuxerPriv;
#[inline]
unsafe extern "C" fn imin(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
unsafe extern "C" fn leb128(f: *mut libc::FILE, len: *mut size_t) -> libc::c_int {
    let mut val: uint64_t = 0 as libc::c_int as uint64_t;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut more: libc::c_uint = 0;
    loop {
        let mut v: uint8_t = 0;
        if fread(
            &mut v as *mut uint8_t as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            f,
        ) < 1 as libc::c_int as libc::c_ulong
        {
            return -(1 as libc::c_int);
        }
        more = (v as libc::c_int & 0x80 as libc::c_int) as libc::c_uint;
        val
            |= ((v as libc::c_int & 0x7f as libc::c_int) as uint64_t)
                << i.wrapping_mul(7 as libc::c_int as libc::c_uint);
        i = i.wrapping_add(1);
        if !(more != 0 && i < 8 as libc::c_int as libc::c_uint) {
            break;
        }
    }
    if val
        > (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_ulong || more != 0
    {
        return -(1 as libc::c_int);
    }
    *len = val;
    return i as libc::c_int;
}
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
        let v: libc::c_int = *fresh1 as libc::c_int;
        more = (v & 0x80 as libc::c_int) as libc::c_uint;
        val
            |= ((v & 0x7f as libc::c_int) as uint64_t)
                << i.wrapping_mul(7 as libc::c_int as libc::c_uint);
        i = i.wrapping_add(1);
        if !(more != 0 && i < 8 as libc::c_int as libc::c_uint) {
            break;
        }
    }
    if val
        > (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_ulong || more != 0
    {
        return -(1 as libc::c_int);
    }
    *len = val;
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
    let mut ret: libc::c_int = 0;
    let mut extension_flag: libc::c_int = 0;
    let mut has_size_flag: libc::c_int = 0;
    if buf_size == 0 {
        return -(1 as libc::c_int);
    }
    if *buf as libc::c_int & 0x80 as libc::c_int != 0 {
        return -(1 as libc::c_int);
    }
    *type_0 = ((*buf as libc::c_int & 0x78 as libc::c_int) >> 3 as libc::c_int)
        as Dav1dObuType;
    extension_flag = (*buf as libc::c_int & 0x4 as libc::c_int) >> 2 as libc::c_int;
    has_size_flag = (*buf as libc::c_int & 0x2 as libc::c_int) >> 1 as libc::c_int;
    buf = buf.offset(1);
    buf_size -= 1;
    if extension_flag != 0 {
        buf = buf.offset(1);
        buf_size -= 1;
    }
    if has_size_flag != 0 {
        ret = leb(buf, buf_size, obu_size);
        if ret < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        return *obu_size as libc::c_int + ret + 1 as libc::c_int + extension_flag;
    } else {
        if allow_implicit_size == 0 {
            return -(1 as libc::c_int);
        }
    }
    *obu_size = buf_size as size_t;
    return buf_size + 1 as libc::c_int + extension_flag;
}
unsafe extern "C" fn annexb_probe(mut data: *const uint8_t) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut temporal_unit_size: size_t = 0;
    ret = leb(
        data.offset(cnt as isize),
        2048 as libc::c_int - cnt,
        &mut temporal_unit_size,
    );
    if ret < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    cnt += ret;
    let mut frame_unit_size: size_t = 0;
    ret = leb(
        data.offset(cnt as isize),
        2048 as libc::c_int - cnt,
        &mut frame_unit_size,
    );
    if ret < 0 as libc::c_int
        || frame_unit_size.wrapping_add(ret as libc::c_ulong) > temporal_unit_size
    {
        return 0 as libc::c_int;
    }
    cnt += ret;
    temporal_unit_size = (temporal_unit_size as libc::c_ulong)
        .wrapping_sub(ret as libc::c_ulong) as size_t as size_t;
    let mut obu_unit_size: size_t = 0;
    ret = leb(data.offset(cnt as isize), 2048 as libc::c_int - cnt, &mut obu_unit_size);
    if ret < 0 as libc::c_int
        || obu_unit_size.wrapping_add(ret as libc::c_ulong) >= frame_unit_size
    {
        return 0 as libc::c_int;
    }
    cnt += ret;
    temporal_unit_size = (temporal_unit_size as libc::c_ulong)
        .wrapping_sub(obu_unit_size.wrapping_add(ret as libc::c_ulong)) as size_t
        as size_t;
    frame_unit_size = (frame_unit_size as libc::c_ulong)
        .wrapping_sub(obu_unit_size.wrapping_add(ret as libc::c_ulong)) as size_t
        as size_t;
    let mut obu_size: size_t = 0;
    let mut type_0: Dav1dObuType = 0 as Dav1dObuType;
    ret = parse_obu_header(
        data.offset(cnt as isize),
        imin(2048 as libc::c_int - cnt, obu_unit_size as libc::c_int),
        &mut obu_size,
        &mut type_0,
        1 as libc::c_int,
    );
    if ret < 0 as libc::c_int
        || type_0 as libc::c_uint != DAV1D_OBU_TD as libc::c_int as libc::c_uint
        || obu_size > 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    cnt += obu_unit_size as libc::c_int;
    let mut seq: libc::c_int = 0 as libc::c_int;
    while cnt < 2048 as libc::c_int {
        ret = leb(
            data.offset(cnt as isize),
            2048 as libc::c_int - cnt,
            &mut obu_unit_size,
        );
        if ret < 0 as libc::c_int
            || obu_unit_size.wrapping_add(ret as libc::c_ulong) > frame_unit_size
        {
            return 0 as libc::c_int;
        }
        cnt += ret;
        temporal_unit_size = (temporal_unit_size as libc::c_ulong)
            .wrapping_sub(ret as libc::c_ulong) as size_t as size_t;
        frame_unit_size = (frame_unit_size as libc::c_ulong)
            .wrapping_sub(ret as libc::c_ulong) as size_t as size_t;
        ret = parse_obu_header(
            data.offset(cnt as isize),
            imin(2048 as libc::c_int - cnt, obu_unit_size as libc::c_int),
            &mut obu_size,
            &mut type_0,
            1 as libc::c_int,
        );
        if ret < 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        cnt += obu_unit_size as libc::c_int;
        match type_0 as libc::c_uint {
            1 => {
                seq = 1 as libc::c_int;
            }
            6 | 3 => return seq,
            2 | 4 => return 0 as libc::c_int,
            _ => {}
        }
        temporal_unit_size = (temporal_unit_size as libc::c_ulong)
            .wrapping_sub(obu_unit_size) as size_t as size_t;
        frame_unit_size = (frame_unit_size as libc::c_ulong).wrapping_sub(obu_unit_size)
            as size_t as size_t;
        if frame_unit_size <= 0 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int;
        }
    }
    return seq;
}
unsafe extern "C" fn annexb_open(
    c: *mut AnnexbInputContext,
    file: *const libc::c_char,
    mut fps: *mut libc::c_uint,
    num_frames: *mut libc::c_uint,
    mut timebase: *mut libc::c_uint,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut len: size_t = 0;
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
    *fps.offset(0 as libc::c_int as isize) = 25 as libc::c_int as libc::c_uint;
    *fps.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uint;
    *timebase.offset(0 as libc::c_int as isize) = 25 as libc::c_int as libc::c_uint;
    *timebase.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uint;
    *num_frames = 0 as libc::c_int as libc::c_uint;
    loop {
        res = leb128((*c).f, &mut len);
        if res < 0 as libc::c_int {
            break;
        }
        fseeko((*c).f, len as __off64_t, 1 as libc::c_int);
        *num_frames = (*num_frames).wrapping_add(1);
    }
    fseeko((*c).f, 0 as libc::c_int as __off64_t, 0 as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn annexb_read(
    c: *mut AnnexbInputContext,
    data: *mut Dav1dData,
) -> libc::c_int {
    let mut len: size_t = 0;
    let mut res: libc::c_int = 0;
    if (*c).temporal_unit_size == 0 {
        res = leb128((*c).f, &mut (*c).temporal_unit_size);
        if res < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    if (*c).frame_unit_size == 0 {
        res = leb128((*c).f, &mut (*c).frame_unit_size);
        if res < 0 as libc::c_int
            || ((*c).frame_unit_size).wrapping_add(res as libc::c_ulong)
                > (*c).temporal_unit_size
        {
            return -(1 as libc::c_int);
        }
        (*c)
            .temporal_unit_size = ((*c).temporal_unit_size as libc::c_ulong)
            .wrapping_sub(res as libc::c_ulong) as size_t as size_t;
    }
    res = leb128((*c).f, &mut len);
    if res < 0 as libc::c_int
        || len.wrapping_add(res as libc::c_ulong) > (*c).frame_unit_size
    {
        return -(1 as libc::c_int);
    }
    let mut ptr: *mut uint8_t = dav1d_data_create(data, len);
    if ptr.is_null() {
        return -(1 as libc::c_int);
    }
    (*c)
        .temporal_unit_size = ((*c).temporal_unit_size as libc::c_ulong)
        .wrapping_sub(len.wrapping_add(res as libc::c_ulong)) as size_t as size_t;
    (*c)
        .frame_unit_size = ((*c).frame_unit_size as libc::c_ulong)
        .wrapping_sub(len.wrapping_add(res as libc::c_ulong)) as size_t as size_t;
    if fread(ptr as *mut libc::c_void, len, 1 as libc::c_int as libc::c_ulong, (*c).f)
        != 1 as libc::c_int as libc::c_ulong
    {
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
unsafe extern "C" fn annexb_close(c: *mut AnnexbInputContext) {
    fclose((*c).f);
}
#[no_mangle]
pub static mut annexb_demuxer: Demuxer = unsafe {
    {
        let mut init = Demuxer {
            priv_data_size: ::core::mem::size_of::<AnnexbInputContext>() as libc::c_ulong
                as libc::c_int,
            name: b"annexb\0" as *const u8 as *const libc::c_char,
            probe_sz: 2048 as libc::c_int,
            probe: Some(
                annexb_probe as unsafe extern "C" fn(*const uint8_t) -> libc::c_int,
            ),
            open: Some(
                annexb_open
                    as unsafe extern "C" fn(
                        *mut AnnexbInputContext,
                        *const libc::c_char,
                        *mut libc::c_uint,
                        *mut libc::c_uint,
                        *mut libc::c_uint,
                    ) -> libc::c_int,
            ),
            read: Some(
                annexb_read
                    as unsafe extern "C" fn(
                        *mut AnnexbInputContext,
                        *mut Dav1dData,
                    ) -> libc::c_int,
            ),
            seek: None,
            close: Some(
                annexb_close as unsafe extern "C" fn(*mut AnnexbInputContext) -> (),
            ),
        };
        init
    }
};
