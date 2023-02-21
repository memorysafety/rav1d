use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type Dav1dRef;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseeko(
        __stream: *mut FILE,
        __off: __off64_t,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn dav1d_data_unref(data: *mut Dav1dData);
    fn dav1d_data_create(data: *mut Dav1dData, sz: size_t) -> *mut uint8_t;
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type off_t = __off64_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type Dav1dObuType = libc::c_uint;
pub const DAV1D_OBU_PADDING: Dav1dObuType = 15;
pub const DAV1D_OBU_REDUNDANT_FRAME_HDR: Dav1dObuType = 7;
pub const DAV1D_OBU_FRAME: Dav1dObuType = 6;
pub const DAV1D_OBU_METADATA: Dav1dObuType = 5;
pub const DAV1D_OBU_TILE_GRP: Dav1dObuType = 4;
pub const DAV1D_OBU_FRAME_HDR: Dav1dObuType = 3;
pub const DAV1D_OBU_TD: Dav1dObuType = 2;
pub const DAV1D_OBU_SEQ_HDR: Dav1dObuType = 1;
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
pub struct DemuxerPriv {
    pub f: *mut FILE,
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
unsafe extern "C" fn leb128(f: *mut FILE, len: *mut size_t) -> libc::c_int {
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
unsafe extern "C" fn section5_probe(mut data: *const uint8_t) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut obu_size: size_t = 0;
    let mut type_0: Dav1dObuType = 0 as Dav1dObuType;
    ret = parse_obu_header(
        data.offset(cnt as isize),
        2048 as libc::c_int - cnt,
        &mut obu_size,
        &mut type_0,
        0 as libc::c_int,
    );
    if ret < 0 as libc::c_int
        || type_0 as libc::c_uint != DAV1D_OBU_TD as libc::c_int as libc::c_uint
        || obu_size > 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    cnt += ret;
    let mut seq: libc::c_int = 0 as libc::c_int;
    while cnt < 2048 as libc::c_int {
        ret = parse_obu_header(
            data.offset(cnt as isize),
            2048 as libc::c_int - cnt,
            &mut obu_size,
            &mut type_0,
            0 as libc::c_int,
        );
        if ret < 0 as libc::c_int {
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
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    *fps.offset(0 as libc::c_int as isize) = 25 as libc::c_int as libc::c_uint;
    *fps.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uint;
    *timebase.offset(0 as libc::c_int as isize) = 25 as libc::c_int as libc::c_uint;
    *timebase.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uint;
    *num_frames = 0 as libc::c_int as libc::c_uint;
    loop {
        let mut byte: [uint8_t; 2] = [0; 2];
        if fread(
            &mut *byte.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut uint8_t
                as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            (*c).f,
        ) < 1 as libc::c_int as libc::c_ulong
        {
            break;
        }
        let obu_type: Dav1dObuType = (byte[0 as libc::c_int as usize] as libc::c_int
            >> 3 as libc::c_int & 0xf as libc::c_int) as Dav1dObuType;
        if obu_type as libc::c_uint == DAV1D_OBU_TD as libc::c_int as libc::c_uint {
            *num_frames = (*num_frames).wrapping_add(1);
        }
        let has_length_field: libc::c_int = byte[0 as libc::c_int as usize]
            as libc::c_int & 0x2 as libc::c_int;
        if has_length_field == 0 {
            return -(1 as libc::c_int);
        }
        let has_extension: libc::c_int = byte[0 as libc::c_int as usize] as libc::c_int
            & 0x4 as libc::c_int;
        if has_extension != 0
            && fread(
                &mut *byte.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut uint8_t
                    as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                (*c).f,
            ) < 1 as libc::c_int as libc::c_ulong
        {
            return -(1 as libc::c_int);
        }
        let mut len: size_t = 0;
        let res: libc::c_int = leb128((*c).f, &mut len);
        if res < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        fseeko((*c).f, len as __off64_t, 1 as libc::c_int);
    }
    fseeko((*c).f, 0 as libc::c_int as __off64_t, 0 as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn section5_read(
    c: *mut Section5InputContext,
    data: *mut Dav1dData,
) -> libc::c_int {
    let mut total_bytes: size_t = 0 as libc::c_int as size_t;
    let mut first: libc::c_int = 1 as libc::c_int;
    loop {
        let mut byte: [uint8_t; 2] = [0; 2];
        if fread(
            &mut *byte.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut uint8_t
                as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            (*c).f,
        ) < 1 as libc::c_int as libc::c_ulong
        {
            if first == 0 && feof((*c).f) != 0 {
                break;
            }
            return -(1 as libc::c_int);
        } else {
            let obu_type: Dav1dObuType = (byte[0 as libc::c_int as usize] as libc::c_int
                >> 3 as libc::c_int & 0xf as libc::c_int) as Dav1dObuType;
            if first != 0 {
                if obu_type as libc::c_uint
                    != DAV1D_OBU_TD as libc::c_int as libc::c_uint
                {
                    return -(1 as libc::c_int);
                }
            } else if obu_type as libc::c_uint
                == DAV1D_OBU_TD as libc::c_int as libc::c_uint
            {
                fseeko((*c).f, -(1 as libc::c_int) as __off64_t, 1 as libc::c_int);
                break;
            }
            let has_length_field: libc::c_int = byte[0 as libc::c_int as usize]
                as libc::c_int & 0x2 as libc::c_int;
            if has_length_field == 0 {
                return -(1 as libc::c_int);
            }
            let has_extension: libc::c_int = (byte[0 as libc::c_int as usize]
                as libc::c_int & 0x4 as libc::c_int != 0) as libc::c_int;
            if has_extension != 0
                && fread(
                    &mut *byte.as_mut_ptr().offset(1 as libc::c_int as isize)
                        as *mut uint8_t as *mut libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    1 as libc::c_int as libc::c_ulong,
                    (*c).f,
                ) < 1 as libc::c_int as libc::c_ulong
            {
                return -(1 as libc::c_int);
            }
            let mut len: size_t = 0;
            let res: libc::c_int = leb128((*c).f, &mut len);
            if res < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            total_bytes = (total_bytes as libc::c_ulong)
                .wrapping_add(
                    ((1 as libc::c_int + has_extension + res) as libc::c_ulong)
                        .wrapping_add(len),
                ) as size_t as size_t;
            fseeko((*c).f, len as __off64_t, 1 as libc::c_int);
            first = 0 as libc::c_int;
        }
    }
    fseeko((*c).f, -(total_bytes as off_t), 1 as libc::c_int);
    let mut ptr: *mut uint8_t = dav1d_data_create(data, total_bytes);
    if ptr.is_null() {
        return -(1 as libc::c_int);
    }
    if fread(
        ptr as *mut libc::c_void,
        total_bytes,
        1 as libc::c_int as libc::c_ulong,
        (*c).f,
    ) != 1 as libc::c_int as libc::c_ulong
    {
        fprintf(
            stderr,
            b"Failed to read frame data: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
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
pub static mut section5_demuxer: Demuxer = unsafe {
    {
        let mut init = Demuxer {
            priv_data_size: ::core::mem::size_of::<Section5InputContext>()
                as libc::c_ulong as libc::c_int,
            name: b"section5\0" as *const u8 as *const libc::c_char,
            probe_sz: 2048 as libc::c_int,
            probe: Some(
                section5_probe as unsafe extern "C" fn(*const uint8_t) -> libc::c_int,
            ),
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
                    as unsafe extern "C" fn(
                        *mut Section5InputContext,
                        *mut Dav1dData,
                    ) -> libc::c_int,
            ),
            seek: None,
            close: Some(
                section5_close as unsafe extern "C" fn(*mut Section5InputContext) -> (),
            ),
        };
        init
    }
};
