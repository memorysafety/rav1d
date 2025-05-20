use crate::compat::errno::errno_location;
use crate::compat::stdio::fseeko;
use crate::compat::stdio::stderr;
use libc::fclose;
use libc::fopen;
use libc::fprintf;
use libc::fread;
use libc::strerror;
use rav1d::dav1d_data_create;
use rav1d::dav1d_data_unref;
use rav1d::include::dav1d::data::Dav1dData;
use rav1d::include::dav1d::headers::Dav1dObuType;
use rav1d::include::dav1d::headers::DAV1D_OBU_FRAME;
use rav1d::include::dav1d::headers::DAV1D_OBU_FRAME_HDR;
use rav1d::include::dav1d::headers::DAV1D_OBU_SEQ_HDR;
use rav1d::include::dav1d::headers::DAV1D_OBU_TD;
use rav1d::include::dav1d::headers::DAV1D_OBU_TILE_GRP;
use std::cmp;
use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_ulong;
use std::ffi::c_void;
use std::ptr::NonNull;

#[repr(C)]
pub struct DemuxerPriv {
    pub f: *mut libc::FILE,
    pub temporal_unit_size: usize,
    pub frame_unit_size: usize,
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

pub type AnnexbInputContext = DemuxerPriv;

unsafe fn leb128(f: *mut libc::FILE, len: *mut usize) -> c_int {
    let mut val: u64 = 0 as c_int as u64;
    let mut i: c_uint = 0 as c_int as c_uint;
    let mut more: c_uint;
    loop {
        let mut v: u8 = 0;
        if fread(&mut v as *mut u8 as *mut c_void, 1, 1, f) < 1 {
            return -1;
        }
        more = (v as c_int & 0x80 as c_int) as c_uint;
        val |= ((v as c_int & 0x7f as c_int) as u64) << i.wrapping_mul(7 as c_int as c_uint);
        i = i.wrapping_add(1);
        if !(more != 0 && i < 8 as c_uint) {
            break;
        }
    }
    if val > u32::MAX as u64 || more != 0 {
        return -1;
    }
    *len = val as usize;
    return i as c_int;
}

unsafe fn leb(mut ptr: *const u8, mut sz: c_int, len: *mut usize) -> c_int {
    let mut val: u64 = 0 as c_int as u64;
    let mut i: c_uint = 0 as c_int as c_uint;
    let mut more: c_uint;
    loop {
        let fresh0 = sz;
        sz = sz - 1;
        if fresh0 == 0 {
            return -1;
        }
        let fresh1 = ptr;
        ptr = ptr.offset(1);
        let v = *fresh1 as c_int;
        more = (v & 0x80 as c_int) as c_uint;
        val |= ((v & 0x7f as c_int) as u64) << i.wrapping_mul(7 as c_int as c_uint);
        i = i.wrapping_add(1);
        if !(more != 0 && i < 8 as c_uint) {
            break;
        }
    }
    if val > u32::MAX as u64 || more != 0 {
        return -1;
    }
    *len = val as usize;
    return i as c_int;
}

#[inline]
unsafe fn parse_obu_header(
    mut buf: *const u8,
    mut buf_size: c_int,
    obu_size: *mut usize,
    type_0: *mut Dav1dObuType,
    allow_implicit_size: c_int,
) -> c_int {
    let ret;
    let extension_flag;
    let has_size_flag;
    if buf_size == 0 {
        return -1;
    }
    if *buf as c_int & 0x80 as c_int != 0 {
        return -1;
    }
    *type_0 = ((*buf as c_int & 0x78 as c_int) >> 3) as Dav1dObuType;
    extension_flag = (*buf as c_int & 0x4 as c_int) >> 2;
    has_size_flag = (*buf as c_int & 0x2 as c_int) >> 1;
    buf = buf.offset(1);
    buf_size -= 1;
    if extension_flag != 0 {
        if buf_size == 0 {
            return -1;
        }
        buf = buf.offset(1);
        buf_size -= 1;
    }
    if has_size_flag != 0 {
        ret = leb(buf, buf_size, obu_size);
        if ret < 0 {
            return -1;
        }
        return *obu_size as c_int + ret + 1 + extension_flag;
    } else {
        if allow_implicit_size == 0 {
            return -1;
        }
    }
    *obu_size = buf_size as usize;
    return buf_size + 1 + extension_flag;
}

unsafe extern "C" fn annexb_probe(data: *const u8) -> c_int {
    let mut ret;
    let mut cnt = 0;
    let mut temporal_unit_size: usize = 0;
    ret = leb(
        data.offset(cnt as isize),
        2048 - cnt,
        &mut temporal_unit_size,
    );
    if ret < 0 {
        return 0 as c_int;
    }
    cnt += ret;
    let mut frame_unit_size: usize = 0;
    ret = leb(data.offset(cnt as isize), 2048 - cnt, &mut frame_unit_size);
    if ret < 0 || frame_unit_size.wrapping_add(ret as usize) > temporal_unit_size {
        return 0 as c_int;
    }
    cnt += ret;
    temporal_unit_size =
        (temporal_unit_size as c_ulong).wrapping_sub(ret as c_ulong) as usize as usize;
    let mut obu_unit_size: usize = 0;
    ret = leb(data.offset(cnt as isize), 2048 - cnt, &mut obu_unit_size);
    if ret < 0 || obu_unit_size.wrapping_add(ret as usize) >= frame_unit_size {
        return 0 as c_int;
    }
    cnt += ret;
    temporal_unit_size =
        (temporal_unit_size).wrapping_sub(obu_unit_size.wrapping_add(ret as usize));
    frame_unit_size = (frame_unit_size).wrapping_sub(obu_unit_size.wrapping_add(ret as usize));
    let mut obu_size: usize = 0;
    let mut type_0: Dav1dObuType = 0 as Dav1dObuType;
    ret = parse_obu_header(
        data.offset(cnt as isize),
        cmp::min(2048 - cnt, obu_unit_size as c_int),
        &mut obu_size,
        &mut type_0,
        1 as c_int,
    );
    if ret < 0 || type_0 as c_uint != DAV1D_OBU_TD as c_int as c_uint || obu_size > 0 {
        return 0 as c_int;
    }
    cnt += obu_unit_size as c_int;
    let mut seq = 0;
    while cnt < 2048 {
        ret = leb(data.offset(cnt as isize), 2048 - cnt, &mut obu_unit_size);
        if ret < 0 || obu_unit_size.wrapping_add(ret as usize) > frame_unit_size {
            return 0 as c_int;
        }
        cnt += ret;
        temporal_unit_size =
            (temporal_unit_size as c_ulong).wrapping_sub(ret as c_ulong) as usize as usize;
        frame_unit_size =
            (frame_unit_size as c_ulong).wrapping_sub(ret as c_ulong) as usize as usize;
        ret = parse_obu_header(
            data.offset(cnt as isize),
            cmp::min(2048 - cnt, obu_unit_size as c_int),
            &mut obu_size,
            &mut type_0,
            1 as c_int,
        );
        if ret < 0 {
            return 0 as c_int;
        }
        cnt += obu_unit_size as c_int;
        match type_0 as c_uint {
            DAV1D_OBU_SEQ_HDR => {
                seq = 1 as c_int;
            }
            DAV1D_OBU_FRAME | DAV1D_OBU_FRAME_HDR => return seq,
            DAV1D_OBU_TD | DAV1D_OBU_TILE_GRP => return 0 as c_int,
            _ => {}
        }
        temporal_unit_size = temporal_unit_size.wrapping_sub(obu_unit_size);
        frame_unit_size = frame_unit_size.wrapping_sub(obu_unit_size) as usize as usize;
        if frame_unit_size <= 0 {
            return 0 as c_int;
        }
    }
    return seq;
}

unsafe extern "C" fn annexb_open(
    c: *mut AnnexbInputContext,
    file: *const c_char,
    fps: *mut c_uint,
    num_frames: *mut c_uint,
    timebase: *mut c_uint,
) -> c_int {
    let mut res;
    let mut len: usize = 0;
    (*c).f = fopen(file, b"rb\0" as *const u8 as *const c_char);
    if ((*c).f).is_null() {
        fprintf(
            stderr(),
            b"Failed to open %s: %s\n\0" as *const u8 as *const c_char,
            file,
            strerror(*errno_location()),
        );
        return -1;
    }
    *fps.offset(0) = 25 as c_int as c_uint;
    *fps.offset(1) = 1 as c_int as c_uint;
    *timebase.offset(0) = 25 as c_int as c_uint;
    *timebase.offset(1) = 1 as c_int as c_uint;
    *num_frames = 0 as c_int as c_uint;
    loop {
        res = leb128((*c).f, &mut len);
        if res < 0 {
            break;
        }
        fseeko((*c).f, len as libc::off_t, 1 as c_int);
        *num_frames = (*num_frames).wrapping_add(1);
    }
    fseeko((*c).f, 0, 0 as c_int);
    return 0 as c_int;
}

unsafe extern "C" fn annexb_read(c: *mut AnnexbInputContext, data: *mut Dav1dData) -> c_int {
    let mut len: usize = 0;
    let mut res;
    if (*c).temporal_unit_size == 0 {
        res = leb128((*c).f, &mut (*c).temporal_unit_size);
        if res < 0 {
            return -1;
        }
    }
    if (*c).frame_unit_size == 0 {
        res = leb128((*c).f, &mut (*c).frame_unit_size);
        if res < 0 || ((*c).frame_unit_size).wrapping_add(res as usize) > (*c).temporal_unit_size {
            return -1;
        }
        (*c).temporal_unit_size =
            ((*c).temporal_unit_size as c_ulong).wrapping_sub(res as c_ulong) as usize as usize;
    }
    res = leb128((*c).f, &mut len);
    if res < 0 || len.wrapping_add(res as usize) > (*c).frame_unit_size {
        return -1;
    }
    let ptr: *mut u8 = dav1d_data_create(NonNull::new(data), len);
    if ptr.is_null() {
        return -1;
    }
    (*c).temporal_unit_size =
        ((*c).temporal_unit_size).wrapping_sub(len.wrapping_add(res as usize)) as usize as usize;
    (*c).frame_unit_size =
        ((*c).frame_unit_size).wrapping_sub(len.wrapping_add(res as usize)) as usize;
    if fread(ptr as *mut c_void, len, 1, (*c).f) != 1 {
        fprintf(
            stderr(),
            b"Failed to read frame data: %s\n\0" as *const u8 as *const c_char,
            strerror(*errno_location()),
        );
        dav1d_data_unref(NonNull::new(data));
        return -1;
    }
    return 0 as c_int;
}

unsafe extern "C" fn annexb_close(c: *mut AnnexbInputContext) {
    fclose((*c).f);
}

#[no_mangle]
pub static mut annexb_demuxer: Demuxer = Demuxer {
    priv_data_size: ::core::mem::size_of::<AnnexbInputContext>() as c_ulong as c_int,
    name: b"annexb\0" as *const u8 as *const c_char,
    probe_sz: 2048 as c_int,
    probe: Some(annexb_probe),
    open: Some(annexb_open),
    read: Some(annexb_read),
    seek: None,
    close: Some(annexb_close),
};
