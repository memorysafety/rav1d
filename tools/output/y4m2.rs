use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
use crate::{stdout,stderr};
use crate::errno_location;
extern "C" {
    pub type Dav1dRef;
    fn fclose(__stream: *mut libc::FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::FILE;
    fn fprintf(_: *mut libc::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut libc::FILE,
    ) -> libc::c_ulong;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn dav1d_picture_unref(p: *mut Dav1dPicture);
}





use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::dav1d::headers::Dav1dTxfmMode;




use crate::include::dav1d::headers::Dav1dFilterMode;











use crate::include::dav1d::headers::Dav1dRestorationType;




use crate::include::dav1d::headers::Dav1dWarpedMotionType;




#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dWarpedMotionParams {
    pub type_0: Dav1dWarpedMotionType,
    pub matrix: [int32_t; 6],
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub p: C2RustUnnamed_0,
    pub abcd: [int16_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub alpha: int16_t,
    pub beta: int16_t,
    pub gamma: int16_t,
    pub delta: int16_t,
}

use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;

use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I400;
use crate::include::dav1d::headers::Dav1dFrameType;
























































use crate::include::dav1d::headers::DAV1D_CHR_UNKNOWN;
use crate::include::dav1d::headers::Dav1dContentLightLevel;
use crate::include::dav1d::headers::Dav1dMasteringDisplay;
use crate::include::dav1d::headers::Dav1dITUTT35;
use crate::include::dav1d::headers::Dav1dSequenceHeader;



use crate::include::dav1d::headers::Dav1dSegmentationDataSet;
use crate::include::dav1d::headers::Dav1dLoopfilterModeRefDeltas;
use crate::include::dav1d::headers::Dav1dFilmGrainData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader {
    pub film_grain: C2RustUnnamed_11,
    pub frame_type: Dav1dFrameType,
    pub width: [libc::c_int; 2],
    pub height: libc::c_int,
    pub frame_offset: libc::c_int,
    pub temporal_id: libc::c_int,
    pub spatial_id: libc::c_int,
    pub show_existing_frame: libc::c_int,
    pub existing_frame_idx: libc::c_int,
    pub frame_id: libc::c_int,
    pub frame_presentation_delay: libc::c_int,
    pub show_frame: libc::c_int,
    pub showable_frame: libc::c_int,
    pub error_resilient_mode: libc::c_int,
    pub disable_cdf_update: libc::c_int,
    pub allow_screen_content_tools: libc::c_int,
    pub force_integer_mv: libc::c_int,
    pub frame_size_override: libc::c_int,
    pub primary_ref_frame: libc::c_int,
    pub buffer_removal_time_present: libc::c_int,
    pub operating_points: [Dav1dFrameHeaderOperatingPoint; 32],
    pub refresh_frame_flags: libc::c_int,
    pub render_width: libc::c_int,
    pub render_height: libc::c_int,
    pub super_res: C2RustUnnamed_10,
    pub have_render_size: libc::c_int,
    pub allow_intrabc: libc::c_int,
    pub frame_ref_short_signaling: libc::c_int,
    pub refidx: [libc::c_int; 7],
    pub hp: libc::c_int,
    pub subpel_filter_mode: Dav1dFilterMode,
    pub switchable_motion_mode: libc::c_int,
    pub use_ref_frame_mvs: libc::c_int,
    pub refresh_context: libc::c_int,
    pub tiling: C2RustUnnamed_9,
    pub quant: C2RustUnnamed_8,
    pub segmentation: C2RustUnnamed_7,
    pub delta: C2RustUnnamed_4,
    pub all_lossless: libc::c_int,
    pub loopfilter: C2RustUnnamed_3,
    pub cdef: C2RustUnnamed_2,
    pub restoration: C2RustUnnamed_1,
    pub txfm_mode: Dav1dTxfmMode,
    pub switchable_comp_refs: libc::c_int,
    pub skip_mode_allowed: libc::c_int,
    pub skip_mode_enabled: libc::c_int,
    pub skip_mode_refs: [libc::c_int; 2],
    pub warp_motion: libc::c_int,
    pub reduced_txtp_set: libc::c_int,
    pub gmv: [Dav1dWarpedMotionParams; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub type_0: [Dav1dRestorationType; 3],
    pub unit_size: [libc::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub damping: libc::c_int,
    pub n_bits: libc::c_int,
    pub y_strength: [libc::c_int; 8],
    pub uv_strength: [libc::c_int; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub level_y: [libc::c_int; 2],
    pub level_u: libc::c_int,
    pub level_v: libc::c_int,
    pub mode_ref_delta_enabled: libc::c_int,
    pub mode_ref_delta_update: libc::c_int,
    pub mode_ref_deltas: Dav1dLoopfilterModeRefDeltas,
    pub sharpness: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub q: C2RustUnnamed_6,
    pub lf: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub present: libc::c_int,
    pub res_log2: libc::c_int,
    pub multi: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub present: libc::c_int,
    pub res_log2: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub enabled: libc::c_int,
    pub update_map: libc::c_int,
    pub temporal: libc::c_int,
    pub update_data: libc::c_int,
    pub seg_data: Dav1dSegmentationDataSet,
    pub lossless: [libc::c_int; 8],
    pub qidx: [libc::c_int; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub yac: libc::c_int,
    pub ydc_delta: libc::c_int,
    pub udc_delta: libc::c_int,
    pub uac_delta: libc::c_int,
    pub vdc_delta: libc::c_int,
    pub vac_delta: libc::c_int,
    pub qm: libc::c_int,
    pub qm_y: libc::c_int,
    pub qm_u: libc::c_int,
    pub qm_v: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub uniform: libc::c_int,
    pub n_bytes: libc::c_uint,
    pub min_log2_cols: libc::c_int,
    pub max_log2_cols: libc::c_int,
    pub log2_cols: libc::c_int,
    pub cols: libc::c_int,
    pub min_log2_rows: libc::c_int,
    pub max_log2_rows: libc::c_int,
    pub log2_rows: libc::c_int,
    pub rows: libc::c_int,
    pub col_start_sb: [uint16_t; 65],
    pub row_start_sb: [uint16_t; 65],
    pub update: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub width_scale_denominator: libc::c_int,
    pub enabled: libc::c_int,
}
use crate::include::dav1d::headers::Dav1dFrameHeaderOperatingPoint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub data: Dav1dFilmGrainData,
    pub present: libc::c_int,
    pub update: libc::c_int,
}
use crate::include::dav1d::picture::Dav1dPictureParameters;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dPicture {
    pub seq_hdr: *mut Dav1dSequenceHeader,
    pub frame_hdr: *mut Dav1dFrameHeader,
    pub data: [*mut libc::c_void; 3],
    pub stride: [ptrdiff_t; 2],
    pub p: Dav1dPictureParameters,
    pub m: Dav1dDataProps,
    pub content_light: *mut Dav1dContentLightLevel,
    pub mastering_display: *mut Dav1dMasteringDisplay,
    pub itut_t35: *mut Dav1dITUTT35,
    pub reserved: [uintptr_t; 4],
    pub frame_hdr_ref: *mut Dav1dRef,
    pub seq_hdr_ref: *mut Dav1dRef,
    pub content_light_ref: *mut Dav1dRef,
    pub mastering_display_ref: *mut Dav1dRef,
    pub itut_t35_ref: *mut Dav1dRef,
    pub reserved_ref: [uintptr_t; 4],
    pub ref_0: *mut Dav1dRef,
    pub allocator_data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MuxerPriv {
    pub f: *mut libc::FILE,
    pub first: libc::c_int,
    pub fps: [libc::c_uint; 2],
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
pub type Y4m2OutputContext = MuxerPriv;
unsafe extern "C" fn y4m2_open(
    c: *mut Y4m2OutputContext,
    file: *const libc::c_char,
    mut _p: *const Dav1dPictureParameters,
    mut fps: *const libc::c_uint,
) -> libc::c_int {
    if strcmp(file, b"-\0" as *const u8 as *const libc::c_char) == 0 {
        (*c).f = stdout;
    } else {
        (*c).f = fopen(file, b"wb\0" as *const u8 as *const libc::c_char);
        if ((*c).f).is_null() {
            fprintf(
                stderr,
                b"Failed to open %s: %s\n\0" as *const u8 as *const libc::c_char,
                file,
                strerror(*errno_location()),
            );
            return -(1 as libc::c_int);
        }
    }
    (*c).first = 1 as libc::c_int;
    (*c).fps[0 as libc::c_int as usize] = *fps.offset(0 as libc::c_int as isize);
    (*c).fps[1 as libc::c_int as usize] = *fps.offset(1 as libc::c_int as isize);
    return 0 as libc::c_int;
}
unsafe extern "C" fn write_header(
    c: *mut Y4m2OutputContext,
    p: *const Dav1dPicture,
) -> libc::c_int {
    static mut ss_names: [[*const libc::c_char; 3]; 4] = [
        [
            b"mono\0" as *const u8 as *const libc::c_char,
            b"mono10\0" as *const u8 as *const libc::c_char,
            b"mono12\0" as *const u8 as *const libc::c_char,
        ],
        [
            0 as *const libc::c_char,
            b"420p10\0" as *const u8 as *const libc::c_char,
            b"420p12\0" as *const u8 as *const libc::c_char,
        ],
        [
            b"422\0" as *const u8 as *const libc::c_char,
            b"422p10\0" as *const u8 as *const libc::c_char,
            b"422p12\0" as *const u8 as *const libc::c_char,
        ],
        [
            b"444\0" as *const u8 as *const libc::c_char,
            b"444p10\0" as *const u8 as *const libc::c_char,
            b"444p12\0" as *const u8 as *const libc::c_char,
        ],
    ];
    static mut chr_names_8bpc_i420: [*const libc::c_char; 3] = [
        b"420jpeg\0" as *const u8 as *const libc::c_char,
        b"420mpeg2\0" as *const u8 as *const libc::c_char,
        b"420\0" as *const u8 as *const libc::c_char,
    ];
    let ss_name: *const libc::c_char = if (*p).p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint
        && (*p).p.bpc == 8 as libc::c_int
    {
        chr_names_8bpc_i420[(if (*(*p).seq_hdr).chr as libc::c_uint
            > 2 as libc::c_int as libc::c_uint
        {
            DAV1D_CHR_UNKNOWN as libc::c_int as libc::c_uint
        } else {
            (*(*p).seq_hdr).chr as libc::c_uint
        }) as usize]
    } else {
        ss_names[(*p).p.layout as usize][(*(*p).seq_hdr).hbd as usize]
    };
    let fw: libc::c_uint = (*p).p.w as libc::c_uint;
    let fh: libc::c_uint = (*p).p.h as libc::c_uint;
    let mut aw: uint64_t = (fh as uint64_t)
        .wrapping_mul((*(*p).frame_hdr).render_width as uint64_t);
    let mut ah: uint64_t = (fw as uint64_t)
        .wrapping_mul((*(*p).frame_hdr).render_height as uint64_t);
    let mut gcd: uint64_t = ah;
    let mut a: uint64_t = aw;
    let mut b: uint64_t = 0;
    loop {
        b = a.wrapping_rem(gcd);
        if !(b != 0) {
            break;
        }
        a = gcd;
        gcd = b;
    }
    aw = aw.wrapping_div(gcd);
    ah = ah.wrapping_div(gcd);
    fprintf(
        (*c).f,
        b"YUV4MPEG2 W%u H%u F%u:%u Ip A%lu:%lu C%s\n\0" as *const u8
            as *const libc::c_char,
        fw,
        fh,
        (*c).fps[0 as libc::c_int as usize],
        (*c).fps[1 as libc::c_int as usize],
        aw,
        ah,
        ss_name,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn y4m2_write(
    c: *mut Y4m2OutputContext,
    p: *mut Dav1dPicture,
) -> libc::c_int {
    let mut current_block: u64;
    if (*c).first != 0 {
        (*c).first = 0 as libc::c_int;
        let res: libc::c_int = write_header(c, p);
        if res < 0 as libc::c_int {
            return res;
        }
    }
    fprintf((*c).f, b"FRAME\n\0" as *const u8 as *const libc::c_char);
    let mut ptr: *mut uint8_t = 0 as *mut uint8_t;
    let hbd: libc::c_int = ((*p).p.bpc > 8 as libc::c_int) as libc::c_int;
    ptr = (*p).data[0 as libc::c_int as usize] as *mut uint8_t;
    let mut y: libc::c_int = 0 as libc::c_int;
    loop {
        if !(y < (*p).p.h) {
            current_block = 11812396948646013369;
            break;
        }
        if fwrite(
            ptr as *const libc::c_void,
            ((*p).p.w << hbd) as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            (*c).f,
        ) != 1 as libc::c_int as libc::c_ulong
        {
            current_block = 11545648641752300099;
            break;
        }
        ptr = ptr.offset((*p).stride[0 as libc::c_int as usize] as isize);
        y += 1;
    }
    match current_block {
        11812396948646013369 => {
            if (*p).p.layout as libc::c_uint
                != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint
            {
                let ss_ver: libc::c_int = ((*p).p.layout as libc::c_uint
                    == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint)
                    as libc::c_int;
                let ss_hor: libc::c_int = ((*p).p.layout as libc::c_uint
                    != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint)
                    as libc::c_int;
                let cw: libc::c_int = (*p).p.w + ss_hor >> ss_hor;
                let ch: libc::c_int = (*p).p.h + ss_ver >> ss_ver;
                let mut pl: libc::c_int = 1 as libc::c_int;
                's_64: loop {
                    if !(pl <= 2 as libc::c_int) {
                        current_block = 13797916685926291137;
                        break;
                    }
                    ptr = (*p).data[pl as usize] as *mut uint8_t;
                    let mut y_0: libc::c_int = 0 as libc::c_int;
                    while y_0 < ch {
                        if fwrite(
                            ptr as *const libc::c_void,
                            (cw << hbd) as libc::c_ulong,
                            1 as libc::c_int as libc::c_ulong,
                            (*c).f,
                        ) != 1 as libc::c_int as libc::c_ulong
                        {
                            current_block = 11545648641752300099;
                            break 's_64;
                        }
                        ptr = ptr
                            .offset((*p).stride[1 as libc::c_int as usize] as isize);
                        y_0 += 1;
                    }
                    pl += 1;
                }
            } else {
                current_block = 13797916685926291137;
            }
            match current_block {
                11545648641752300099 => {}
                _ => {
                    dav1d_picture_unref(p);
                    return 0 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    dav1d_picture_unref(p);
    fprintf(
        stderr,
        b"Failed to write frame data: %s\n\0" as *const u8 as *const libc::c_char,
        strerror(*errno_location()),
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn y4m2_close(c: *mut Y4m2OutputContext) {
    if (*c).f != stdout {
        fclose((*c).f);
    }
}
#[no_mangle]
pub static mut y4m2_muxer: Muxer = {
    let mut init = Muxer {
        priv_data_size: ::core::mem::size_of::<Y4m2OutputContext>() as libc::c_ulong
            as libc::c_int,
        name: b"yuv4mpeg2\0" as *const u8 as *const libc::c_char,
        extension: b"y4m\0" as *const u8 as *const libc::c_char,
        write_header: Some(
            y4m2_open
                as unsafe extern "C" fn(
                    *mut Y4m2OutputContext,
                    *const libc::c_char,
                    *const Dav1dPictureParameters,
                    *const libc::c_uint,
                ) -> libc::c_int,
        ),
        write_picture: Some(
            y4m2_write
                as unsafe extern "C" fn(
                    *mut Y4m2OutputContext,
                    *mut Dav1dPicture,
                ) -> libc::c_int,
        ),
        write_trailer: Some(
            y4m2_close as unsafe extern "C" fn(*mut Y4m2OutputContext) -> (),
        ),
        verify: None,
    };
    init
};
