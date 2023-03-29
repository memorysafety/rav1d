#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
#![feature(c_variadic)]
use crate::include::stddef::*;
use crate::include::stdint::*;
use ::c2rust_out::*;
use crate::src::r#ref::Dav1dRef;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type Dav1dContext;
    pub type DemuxerContext;
    pub type MuxerContext;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fclose(__stream: *mut libc::FILE) -> libc::c_int;
    fn fflush(__stream: *mut libc::FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::FILE;
    fn fprintf(_: *mut libc::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut libc::FILE) -> libc::c_int;
    fn fileno(__stream: *mut libc::FILE) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> libc::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn dav1d_data_unref(data: *mut Dav1dData);
    fn dav1d_close(c_out: *mut *mut Dav1dContext);
    fn dav1d_parse_sequence_header(
        out: *mut Dav1dSequenceHeader,
        buf: *const uint8_t,
        sz: size_t,
    ) -> libc::c_int;
    fn dav1d_send_data(c: *mut Dav1dContext, in_0: *mut Dav1dData) -> libc::c_int;
    fn dav1d_open(c_out: *mut *mut Dav1dContext, s: *const Dav1dSettings) -> libc::c_int;
    fn dav1d_get_picture(c: *mut Dav1dContext, out: *mut Dav1dPicture) -> libc::c_int;
    fn dav1d_version() -> *const libc::c_char;
    fn input_open(
        c_out: *mut *mut DemuxerContext,
        name: *const libc::c_char,
        filename: *const libc::c_char,
        fps: *mut libc::c_uint,
        num_frames: *mut libc::c_uint,
        timebase: *mut libc::c_uint,
    ) -> libc::c_int;
    fn input_read(ctx: *mut DemuxerContext, data: *mut Dav1dData) -> libc::c_int;
    fn input_close(ctx: *mut DemuxerContext);
    fn output_open(
        c: *mut *mut MuxerContext,
        name: *const libc::c_char,
        filename: *const libc::c_char,
        p: *const Dav1dPictureParameters,
        fps: *const libc::c_uint,
    ) -> libc::c_int;
    fn output_write(ctx: *mut MuxerContext, pic: *mut Dav1dPicture) -> libc::c_int;
    fn output_close(ctx: *mut MuxerContext);
    fn output_verify(
        ctx: *mut MuxerContext,
        hash_string: *const libc::c_char,
    ) -> libc::c_int;
    fn parse(
        argc: libc::c_int,
        argv: *const *mut libc::c_char,
        cli_settings: *mut CLISettings,
        lib_settings: *mut Dav1dSettings,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}

pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type _IO_lock_t = ();
use crate::include::dav1d::common::Dav1dUserData;
use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::dav1d::headers::Dav1dTxfmMode;
use crate::include::dav1d::headers::DAV1D_N_TX_MODES;
use crate::include::dav1d::headers::DAV1D_TX_SWITCHABLE;
use crate::include::dav1d::headers::DAV1D_TX_LARGEST;
use crate::include::dav1d::headers::DAV1D_TX_4X4_ONLY;
use crate::include::dav1d::headers::Dav1dFilterMode;
use crate::include::dav1d::headers::DAV1D_FILTER_SWITCHABLE;
use crate::include::dav1d::headers::DAV1D_N_FILTERS;
use crate::include::dav1d::headers::DAV1D_FILTER_BILINEAR;
use crate::include::dav1d::headers::DAV1D_N_SWITCHABLE_FILTERS;
use crate::include::dav1d::headers::DAV1D_FILTER_8TAP_SHARP;
use crate::include::dav1d::headers::DAV1D_FILTER_8TAP_SMOOTH;
use crate::include::dav1d::headers::DAV1D_FILTER_8TAP_REGULAR;
use crate::include::dav1d::headers::Dav1dAdaptiveBoolean;
use crate::include::dav1d::headers::DAV1D_ADAPTIVE;
use crate::include::dav1d::headers::DAV1D_ON;
use crate::include::dav1d::headers::DAV1D_OFF;
use crate::include::dav1d::headers::Dav1dRestorationType;
use crate::include::dav1d::headers::DAV1D_RESTORATION_SGRPROJ;
use crate::include::dav1d::headers::DAV1D_RESTORATION_WIENER;
use crate::include::dav1d::headers::DAV1D_RESTORATION_SWITCHABLE;
use crate::include::dav1d::headers::DAV1D_RESTORATION_NONE;
use crate::include::dav1d::headers::Dav1dWarpedMotionType;
use crate::include::dav1d::headers::DAV1D_WM_TYPE_AFFINE;
use crate::include::dav1d::headers::DAV1D_WM_TYPE_ROT_ZOOM;
use crate::include::dav1d::headers::DAV1D_WM_TYPE_TRANSLATION;
use crate::include::dav1d::headers::DAV1D_WM_TYPE_IDENTITY;
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
use crate::include::dav1d::headers::Dav1dPixelLayout;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I422;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I400;
use crate::include::dav1d::headers::Dav1dFrameType;
use crate::include::dav1d::headers::DAV1D_FRAME_TYPE_SWITCH;
use crate::include::dav1d::headers::DAV1D_FRAME_TYPE_INTRA;
use crate::include::dav1d::headers::DAV1D_FRAME_TYPE_INTER;
use crate::include::dav1d::headers::DAV1D_FRAME_TYPE_KEY;
use crate::include::dav1d::headers::Dav1dColorPrimaries;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_RESERVED;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_EBU3213;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_SMPTE432;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_SMPTE431;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_XYZ;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_BT2020;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_FILM;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_SMPTE240;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_BT601;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_BT470BG;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_BT470M;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_UNKNOWN;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_BT709;
use crate::include::dav1d::headers::Dav1dTransferCharacteristics;
use crate::include::dav1d::headers::DAV1D_TRC_RESERVED;
use crate::include::dav1d::headers::DAV1D_TRC_HLG;
use crate::include::dav1d::headers::DAV1D_TRC_SMPTE428;
use crate::include::dav1d::headers::DAV1D_TRC_SMPTE2084;
use crate::include::dav1d::headers::DAV1D_TRC_BT2020_12BIT;
use crate::include::dav1d::headers::DAV1D_TRC_BT2020_10BIT;
use crate::include::dav1d::headers::DAV1D_TRC_SRGB;
use crate::include::dav1d::headers::DAV1D_TRC_BT1361;
use crate::include::dav1d::headers::DAV1D_TRC_IEC61966;
use crate::include::dav1d::headers::DAV1D_TRC_LOG100_SQRT10;
use crate::include::dav1d::headers::DAV1D_TRC_LOG100;
use crate::include::dav1d::headers::DAV1D_TRC_LINEAR;
use crate::include::dav1d::headers::DAV1D_TRC_SMPTE240;
use crate::include::dav1d::headers::DAV1D_TRC_BT601;
use crate::include::dav1d::headers::DAV1D_TRC_BT470BG;
use crate::include::dav1d::headers::DAV1D_TRC_BT470M;
use crate::include::dav1d::headers::DAV1D_TRC_UNKNOWN;
use crate::include::dav1d::headers::DAV1D_TRC_BT709;
use crate::include::dav1d::headers::Dav1dMatrixCoefficients;
use crate::include::dav1d::headers::DAV1D_MC_RESERVED;
use crate::include::dav1d::headers::DAV1D_MC_ICTCP;
use crate::include::dav1d::headers::DAV1D_MC_CHROMAT_CL;
use crate::include::dav1d::headers::DAV1D_MC_CHROMAT_NCL;
use crate::include::dav1d::headers::DAV1D_MC_SMPTE2085;
use crate::include::dav1d::headers::DAV1D_MC_BT2020_CL;
use crate::include::dav1d::headers::DAV1D_MC_BT2020_NCL;
use crate::include::dav1d::headers::DAV1D_MC_SMPTE_YCGCO;
use crate::include::dav1d::headers::DAV1D_MC_SMPTE240;
use crate::include::dav1d::headers::DAV1D_MC_BT601;
use crate::include::dav1d::headers::DAV1D_MC_BT470BG;
use crate::include::dav1d::headers::DAV1D_MC_FCC;
use crate::include::dav1d::headers::DAV1D_MC_UNKNOWN;
use crate::include::dav1d::headers::DAV1D_MC_BT709;
use crate::include::dav1d::headers::DAV1D_MC_IDENTITY;
use crate::include::dav1d::headers::Dav1dChromaSamplePosition;
use crate::include::dav1d::headers::DAV1D_CHR_COLOCATED;
use crate::include::dav1d::headers::DAV1D_CHR_VERTICAL;
use crate::include::dav1d::headers::DAV1D_CHR_UNKNOWN;
use crate::include::dav1d::headers::Dav1dContentLightLevel;
use crate::include::dav1d::headers::Dav1dMasteringDisplay;
use crate::include::dav1d::headers::Dav1dITUTT35;
use crate::include::dav1d::headers::Dav1dSequenceHeader;
use crate::include::dav1d::headers::Dav1dSequenceHeaderOperatingParameterInfo;
use crate::include::dav1d::headers::Dav1dSequenceHeaderOperatingPoint;
use crate::include::dav1d::headers::Dav1dSegmentationData;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dPictureParameters {
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub layout: Dav1dPixelLayout,
    pub bpc: libc::c_int,
}
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
pub struct Dav1dPicAllocator {
    pub cookie: *mut libc::c_void,
    pub alloc_picture_callback: Option::<
        unsafe extern "C" fn(*mut Dav1dPicture, *mut libc::c_void) -> libc::c_int,
    >,
    pub release_picture_callback: Option::<
        unsafe extern "C" fn(*mut Dav1dPicture, *mut libc::c_void) -> (),
    >,
}
use crate::include::dav1d::data::Dav1dData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dLogger {
    pub cookie: *mut libc::c_void,
    pub callback: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            ::core::ffi::VaList,
        ) -> (),
    >,
}
use crate::include::dav1d::dav1d::Dav1dInloopFilterType;
use crate::include::dav1d::dav1d::DAV1D_INLOOPFILTER_ALL;
use crate::include::dav1d::dav1d::DAV1D_INLOOPFILTER_RESTORATION;
use crate::include::dav1d::dav1d::DAV1D_INLOOPFILTER_CDEF;
use crate::include::dav1d::dav1d::DAV1D_INLOOPFILTER_DEBLOCK;
use crate::include::dav1d::dav1d::DAV1D_INLOOPFILTER_NONE;
pub type Dav1dDecodeFrameType = libc::c_uint;
pub const DAV1D_DECODEFRAMETYPE_KEY: Dav1dDecodeFrameType = 3;
pub const DAV1D_DECODEFRAMETYPE_INTRA: Dav1dDecodeFrameType = 2;
pub const DAV1D_DECODEFRAMETYPE_REFERENCE: Dav1dDecodeFrameType = 1;
pub const DAV1D_DECODEFRAMETYPE_ALL: Dav1dDecodeFrameType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dSettings {
    pub n_threads: libc::c_int,
    pub max_frame_delay: libc::c_int,
    pub apply_grain: libc::c_int,
    pub operating_point: libc::c_int,
    pub all_layers: libc::c_int,
    pub frame_size_limit: libc::c_uint,
    pub allocator: Dav1dPicAllocator,
    pub logger: Dav1dLogger,
    pub strict_std_compliance: libc::c_int,
    pub output_invisible_frames: libc::c_int,
    pub inloop_filters: Dav1dInloopFilterType,
    pub decode_frame_type: Dav1dDecodeFrameType,
    pub reserved: [uint8_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CLISettings {
    pub outputfile: *const libc::c_char,
    pub inputfile: *const libc::c_char,
    pub demuxer: *const libc::c_char,
    pub muxer: *const libc::c_char,
    pub frametimes: *const libc::c_char,
    pub verify: *const libc::c_char,
    pub limit: libc::c_uint,
    pub skip: libc::c_uint,
    pub quiet: libc::c_int,
    pub realtime: C2RustUnnamed_12,
    pub realtime_fps: libc::c_double,
    pub realtime_cache: libc::c_uint,
    pub neg_stride: libc::c_int,
}
pub type C2RustUnnamed_12 = libc::c_uint;
pub const REALTIME_CUSTOM: C2RustUnnamed_12 = 2;
pub const REALTIME_INPUT: C2RustUnnamed_12 = 1;
pub const REALTIME_DISABLE: C2RustUnnamed_12 = 0;
unsafe extern "C" fn get_time_nanos() -> uint64_t {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(1 as libc::c_int, &mut ts);
    return (1000000000 as libc::c_ulonglong)
        .wrapping_mul(ts.tv_sec as libc::c_ulonglong)
        .wrapping_add(ts.tv_nsec as libc::c_ulonglong) as uint64_t;
}
unsafe extern "C" fn sleep_nanos(mut d: uint64_t) {
    let ts: timespec = {
        let mut init = timespec {
            tv_sec: d.wrapping_div(1000000000 as libc::c_int as libc::c_ulong) as time_t,
            tv_nsec: d.wrapping_rem(1000000000 as libc::c_int as libc::c_ulong)
                as __syscall_slong_t,
        };
        init
    };
    nanosleep(&ts, 0 as *mut timespec);
}
unsafe extern "C" fn synchronize(
    realtime: libc::c_int,
    cache: libc::c_uint,
    n_out: libc::c_uint,
    nspf: uint64_t,
    tfirst: uint64_t,
    elapsed: *mut uint64_t,
    frametimes: *mut libc::FILE,
) {
    let tcurr: uint64_t = get_time_nanos();
    let last: uint64_t = *elapsed;
    *elapsed = tcurr.wrapping_sub(tfirst);
    if realtime != 0 {
        let deadline: uint64_t = nspf.wrapping_mul(n_out as libc::c_ulong);
        if *elapsed < deadline {
            let remaining: uint64_t = deadline.wrapping_sub(*elapsed);
            if remaining > nspf.wrapping_mul(cache as libc::c_ulong) {
                sleep_nanos(
                    remaining.wrapping_sub(nspf.wrapping_mul(cache as libc::c_ulong)),
                );
            }
            *elapsed = deadline;
        }
    }
    if !frametimes.is_null() {
        let frametime: uint64_t = (*elapsed).wrapping_sub(last);
        fprintf(frametimes, b"%lu\n\0" as *const u8 as *const libc::c_char, frametime);
        fflush(frametimes);
    }
}
unsafe extern "C" fn print_stats(
    istty: libc::c_int,
    n: libc::c_uint,
    num: libc::c_uint,
    elapsed: uint64_t,
    i_fps: libc::c_double,
) {
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut b: *mut libc::c_char = buf.as_mut_ptr();
    let end: *mut libc::c_char = buf.as_mut_ptr().offset(80 as libc::c_int as isize);
    if istty != 0 {
        let fresh0 = b;
        b = b.offset(1);
        *fresh0 = '\r' as i32 as libc::c_char;
    }
    if num == 0xffffffff as libc::c_uint {
        b = b
            .offset(
                snprintf(
                    b,
                    end.offset_from(b) as libc::c_long as libc::c_ulong,
                    b"Decoded %u frames\0" as *const u8 as *const libc::c_char,
                    n,
                ) as isize,
            );
    } else {
        b = b
            .offset(
                snprintf(
                    b,
                    end.offset_from(b) as libc::c_long as libc::c_ulong,
                    b"Decoded %u/%u frames (%.1lf%%)\0" as *const u8
                        as *const libc::c_char,
                    n,
                    num,
                    100.0f64 * n as libc::c_double / num as libc::c_double,
                ) as isize,
            );
    }
    if b < end {
        let d_fps: libc::c_double = 1e9f64 * n as libc::c_double
            / elapsed as libc::c_double;
        if i_fps != 0. {
            let speed: libc::c_double = d_fps / i_fps;
            b = b
                .offset(
                    snprintf(
                        b,
                        end.offset_from(b) as libc::c_long as libc::c_ulong,
                        b" - %.2lf/%.2lf fps (%.2lfx)\0" as *const u8
                            as *const libc::c_char,
                        d_fps,
                        i_fps,
                        speed,
                    ) as isize,
                );
        } else {
            b = b
                .offset(
                    snprintf(
                        b,
                        end.offset_from(b) as libc::c_long as libc::c_ulong,
                        b" - %.2lf fps\0" as *const u8 as *const libc::c_char,
                        d_fps,
                    ) as isize,
                );
        }
    }
    if istty == 0 {
        strcpy(
            if b > end.offset(-(2 as libc::c_int as isize)) {
                end.offset(-(2 as libc::c_int as isize))
            } else {
                b
            },
            b"\n\0" as *const u8 as *const libc::c_char,
        );
    }
    fputs(buf.as_mut_ptr(), stderr);
}
unsafe extern "C" fn picture_alloc(
    p: *mut Dav1dPicture,
    _: *mut libc::c_void,
) -> libc::c_int {
    let hbd: libc::c_int = ((*p).p.bpc > 8 as libc::c_int) as libc::c_int;
    let aligned_w: libc::c_int = (*p).p.w + 127 as libc::c_int & !(127 as libc::c_int);
    let aligned_h: libc::c_int = (*p).p.h + 127 as libc::c_int & !(127 as libc::c_int);
    let has_chroma: libc::c_int = ((*p).p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_ver: libc::c_int = ((*p).p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_hor: libc::c_int = ((*p).p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
    let mut y_stride: ptrdiff_t = (aligned_w << hbd) as ptrdiff_t;
    let mut uv_stride: ptrdiff_t = if has_chroma != 0 {
        y_stride >> ss_hor
    } else {
        0 as libc::c_int as libc::c_long
    };
    if y_stride & 1023 as libc::c_int as libc::c_long == 0 {
        y_stride += 64 as libc::c_int as libc::c_long;
    }
    if uv_stride & 1023 as libc::c_int as libc::c_long == 0 && has_chroma != 0 {
        uv_stride += 64 as libc::c_int as libc::c_long;
    }
    (*p).stride[0 as libc::c_int as usize] = -y_stride;
    (*p).stride[1 as libc::c_int as usize] = -uv_stride;
    let y_sz: size_t = (y_stride * aligned_h as libc::c_long) as size_t;
    let uv_sz: size_t = (uv_stride * (aligned_h >> ss_ver) as libc::c_long) as size_t;
    let pic_size: size_t = y_sz
        .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(uv_sz));
    let buf: *mut uint8_t = malloc(
        pic_size.wrapping_add((64 as libc::c_int * 2 as libc::c_int) as libc::c_ulong),
    ) as *mut uint8_t;
    if buf.is_null() {
        return -(12 as libc::c_int);
    }
    (*p).allocator_data = buf as *mut libc::c_void;
    let align_m1: ptrdiff_t = (64 as libc::c_int - 1 as libc::c_int) as ptrdiff_t;
    let data: *mut uint8_t = (buf as ptrdiff_t + align_m1 & !align_m1) as *mut uint8_t;
    (*p)
        .data[0 as libc::c_int
        as usize] = data.offset(y_sz as isize).offset(-(y_stride as isize))
        as *mut libc::c_void;
    (*p)
        .data[1 as libc::c_int
        as usize] = (if has_chroma != 0 {
        data.offset(y_sz as isize)
            .offset(uv_sz.wrapping_mul(1 as libc::c_int as libc::c_ulong) as isize)
            .offset(-(uv_stride as isize))
    } else {
        0 as *mut uint8_t
    }) as *mut libc::c_void;
    (*p)
        .data[2 as libc::c_int
        as usize] = (if has_chroma != 0 {
        data.offset(y_sz as isize)
            .offset(uv_sz.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize)
            .offset(-(uv_stride as isize))
    } else {
        0 as *mut uint8_t
    }) as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn picture_release(p: *mut Dav1dPicture, _: *mut libc::c_void) {
    free((*p).allocator_data);
}
unsafe fn main_0(argc: libc::c_int, argv: *const *mut libc::c_char) -> libc::c_int {
    let istty: libc::c_int = isatty(fileno(stderr));
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut cli_settings: CLISettings = CLISettings {
        outputfile: 0 as *const libc::c_char,
        inputfile: 0 as *const libc::c_char,
        demuxer: 0 as *const libc::c_char,
        muxer: 0 as *const libc::c_char,
        frametimes: 0 as *const libc::c_char,
        verify: 0 as *const libc::c_char,
        limit: 0,
        skip: 0,
        quiet: 0,
        realtime: REALTIME_DISABLE,
        realtime_fps: 0.,
        realtime_cache: 0,
        neg_stride: 0,
    };
    let mut lib_settings: Dav1dSettings = Dav1dSettings {
        n_threads: 0,
        max_frame_delay: 0,
        apply_grain: 0,
        operating_point: 0,
        all_layers: 0,
        frame_size_limit: 0,
        allocator: Dav1dPicAllocator {
            cookie: 0 as *mut libc::c_void,
            alloc_picture_callback: None,
            release_picture_callback: None,
        },
        logger: Dav1dLogger {
            cookie: 0 as *mut libc::c_void,
            callback: None,
        },
        strict_std_compliance: 0,
        output_invisible_frames: 0,
        inloop_filters: DAV1D_INLOOPFILTER_NONE,
        decode_frame_type: DAV1D_DECODEFRAMETYPE_ALL,
        reserved: [0; 16],
    };
    let mut in_0: *mut DemuxerContext = 0 as *mut DemuxerContext;
    let mut out: *mut MuxerContext = 0 as *mut MuxerContext;
    let mut p: Dav1dPicture = Dav1dPicture {
        seq_hdr: 0 as *mut Dav1dSequenceHeader,
        frame_hdr: 0 as *mut Dav1dFrameHeader,
        data: [0 as *mut libc::c_void; 3],
        stride: [0; 2],
        p: Dav1dPictureParameters {
            w: 0,
            h: 0,
            layout: DAV1D_PIXEL_LAYOUT_I400,
            bpc: 0,
        },
        m: Dav1dDataProps {
            timestamp: 0,
            duration: 0,
            offset: 0,
            size: 0,
            user_data: Dav1dUserData {
                data: 0 as *const uint8_t,
                ref_0: 0 as *mut Dav1dRef,
            },
        },
        content_light: 0 as *mut Dav1dContentLightLevel,
        mastering_display: 0 as *mut Dav1dMasteringDisplay,
        itut_t35: 0 as *mut Dav1dITUTT35,
        reserved: [0; 4],
        frame_hdr_ref: 0 as *mut Dav1dRef,
        seq_hdr_ref: 0 as *mut Dav1dRef,
        content_light_ref: 0 as *mut Dav1dRef,
        mastering_display_ref: 0 as *mut Dav1dRef,
        itut_t35_ref: 0 as *mut Dav1dRef,
        reserved_ref: [0; 4],
        ref_0: 0 as *mut Dav1dRef,
        allocator_data: 0 as *mut libc::c_void,
    };
    let mut c: *mut Dav1dContext = 0 as *mut Dav1dContext;
    let mut data: Dav1dData = Dav1dData {
        data: 0 as *const uint8_t,
        sz: 0,
        ref_0: 0 as *mut Dav1dRef,
        m: Dav1dDataProps {
            timestamp: 0,
            duration: 0,
            offset: 0,
            size: 0,
            user_data: Dav1dUserData {
                data: 0 as *const uint8_t,
                ref_0: 0 as *mut Dav1dRef,
            },
        },
    };
    let mut n_out: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut total: libc::c_uint = 0;
    let mut fps: [libc::c_uint; 2] = [0; 2];
    let mut timebase: [libc::c_uint; 2] = [0; 2];
    let mut nspf: uint64_t = 0;
    let mut tfirst: uint64_t = 0;
    let mut elapsed: uint64_t = 0;
    let mut i_fps: libc::c_double = 0.;
    let mut frametimes: *mut libc::FILE = 0 as *mut libc::FILE;
    let mut version: *const libc::c_char = dav1d_version();
    if strcmp(version, b"1.0.0-130-g26eca15\0" as *const u8 as *const libc::c_char) != 0
    {
        fprintf(
            stderr,
            b"Version mismatch (library: %s, executable: %s)\n\0" as *const u8
                as *const libc::c_char,
            version,
            b"1.0.0-130-g26eca15\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    parse(argc, argv, &mut cli_settings, &mut lib_settings);
    if cli_settings.neg_stride != 0 {
        lib_settings
            .allocator
            .alloc_picture_callback = Some(
            picture_alloc
                as unsafe extern "C" fn(
                    *mut Dav1dPicture,
                    *mut libc::c_void,
                ) -> libc::c_int,
        );
        lib_settings
            .allocator
            .release_picture_callback = Some(
            picture_release
                as unsafe extern "C" fn(*mut Dav1dPicture, *mut libc::c_void) -> (),
        );
    }
    res = input_open(
        &mut in_0,
        cli_settings.demuxer,
        cli_settings.inputfile,
        fps.as_mut_ptr(),
        &mut total,
        timebase.as_mut_ptr(),
    );
    if res < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i <= cli_settings.skip {
        res = input_read(in_0, &mut data);
        if res < 0 as libc::c_int {
            input_close(in_0);
            return 1 as libc::c_int;
        }
        if i < cli_settings.skip {
            dav1d_data_unref(&mut data);
        }
        i = i.wrapping_add(1);
    }
    if cli_settings.quiet == 0 {
        fprintf(
            stderr,
            b"dav1d %s - by VideoLAN\n\0" as *const u8 as *const libc::c_char,
            dav1d_version(),
        );
    }
    if cli_settings.skip != 0 {
        let mut seq: Dav1dSequenceHeader = Dav1dSequenceHeader {
            profile: 0,
            max_width: 0,
            max_height: 0,
            layout: DAV1D_PIXEL_LAYOUT_I400,
            pri: 0 as Dav1dColorPrimaries,
            trc: 0 as Dav1dTransferCharacteristics,
            mtrx: DAV1D_MC_IDENTITY,
            chr: DAV1D_CHR_UNKNOWN,
            hbd: 0,
            color_range: 0,
            num_operating_points: 0,
            operating_points: [Dav1dSequenceHeaderOperatingPoint {
                major_level: 0,
                minor_level: 0,
                initial_display_delay: 0,
                idc: 0,
                tier: 0,
                decoder_model_param_present: 0,
                display_model_param_present: 0,
            }; 32],
            still_picture: 0,
            reduced_still_picture_header: 0,
            timing_info_present: 0,
            num_units_in_tick: 0,
            time_scale: 0,
            equal_picture_interval: 0,
            num_ticks_per_picture: 0,
            decoder_model_info_present: 0,
            encoder_decoder_buffer_delay_length: 0,
            num_units_in_decoding_tick: 0,
            buffer_removal_delay_length: 0,
            frame_presentation_delay_length: 0,
            display_model_info_present: 0,
            width_n_bits: 0,
            height_n_bits: 0,
            frame_id_numbers_present: 0,
            delta_frame_id_n_bits: 0,
            frame_id_n_bits: 0,
            sb128: 0,
            filter_intra: 0,
            intra_edge_filter: 0,
            inter_intra: 0,
            masked_compound: 0,
            warped_motion: 0,
            dual_filter: 0,
            order_hint: 0,
            jnt_comp: 0,
            ref_frame_mvs: 0,
            screen_content_tools: DAV1D_OFF,
            force_integer_mv: DAV1D_OFF,
            order_hint_n_bits: 0,
            super_res: 0,
            cdef: 0,
            restoration: 0,
            ss_hor: 0,
            ss_ver: 0,
            monochrome: 0,
            color_description_present: 0,
            separate_uv_delta_q: 0,
            film_grain_present: 0,
            operating_parameter_info: [Dav1dSequenceHeaderOperatingParameterInfo {
                decoder_buffer_delay: 0,
                encoder_buffer_delay: 0,
                low_delay_mode: 0,
            }; 32],
        };
        let mut seq_skip: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while dav1d_parse_sequence_header(&mut seq, data.data, data.sz) != 0 {
            res = input_read(in_0, &mut data);
            if res < 0 as libc::c_int {
                input_close(in_0);
                return 1 as libc::c_int;
            }
            seq_skip = seq_skip.wrapping_add(1);
        }
        if seq_skip != 0 && cli_settings.quiet == 0 {
            fprintf(
                stderr,
                b"skipped %u packets due to missing sequence header\n\0" as *const u8
                    as *const libc::c_char,
                seq_skip,
            );
        }
    }
    if cli_settings.limit != 0 as libc::c_int as libc::c_uint
        && cli_settings.limit < total
    {
        total = cli_settings.limit;
    }
    res = dav1d_open(&mut c, &mut lib_settings);
    if res != 0 {
        return 1 as libc::c_int;
    }
    if !(cli_settings.frametimes).is_null() {
        frametimes = fopen(
            cli_settings.frametimes,
            b"w\0" as *const u8 as *const libc::c_char,
        );
    }
    if cli_settings.realtime as libc::c_uint
        != REALTIME_CUSTOM as libc::c_int as libc::c_uint
    {
        if fps[1 as libc::c_int as usize] == 0 as libc::c_int as libc::c_uint {
            i_fps = 0 as libc::c_int as libc::c_double;
            nspf = 0 as libc::c_int as uint64_t;
        } else {
            i_fps = fps[0 as libc::c_int as usize] as libc::c_double
                / fps[1 as libc::c_int as usize] as libc::c_double;
            nspf = (1000000000 as libc::c_ulonglong)
                .wrapping_mul(fps[1 as libc::c_int as usize] as libc::c_ulonglong)
                .wrapping_div(fps[0 as libc::c_int as usize] as libc::c_ulonglong)
                as uint64_t;
        }
    } else {
        i_fps = cli_settings.realtime_fps;
        nspf = (1000000000.0f64 / cli_settings.realtime_fps) as uint64_t;
    }
    tfirst = get_time_nanos();
    loop {
        memset(
            &mut p as *mut Dav1dPicture as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<Dav1dPicture>() as libc::c_ulong,
        );
        res = dav1d_send_data(c, &mut data);
        if res < 0 as libc::c_int {
            if res != -(11 as libc::c_int) {
                dav1d_data_unref(&mut data);
                fprintf(
                    stderr,
                    b"Error decoding frame: %s\n\0" as *const u8 as *const libc::c_char,
                    strerror(-res),
                );
                if res != -(22 as libc::c_int) {
                    break;
                }
            }
        }
        res = dav1d_get_picture(c, &mut p);
        if res < 0 as libc::c_int {
            if res != -(11 as libc::c_int) {
                fprintf(
                    stderr,
                    b"Error decoding frame: %s\n\0" as *const u8 as *const libc::c_char,
                    strerror(-res),
                );
                if res != -(22 as libc::c_int) {
                    break;
                }
            }
            res = 0 as libc::c_int;
        } else {
            if n_out == 0 {
                res = output_open(
                    &mut out,
                    cli_settings.muxer,
                    cli_settings.outputfile,
                    &mut p.p,
                    fps.as_mut_ptr() as *const libc::c_uint,
                );
                if res < 0 as libc::c_int {
                    if !frametimes.is_null() {
                        fclose(frametimes);
                    }
                    return 1 as libc::c_int;
                }
            }
            res = output_write(out, &mut p);
            if res < 0 as libc::c_int {
                break;
            }
            n_out = n_out.wrapping_add(1);
            if nspf != 0 || cli_settings.quiet == 0 {
                synchronize(
                    cli_settings.realtime as libc::c_int,
                    cli_settings.realtime_cache,
                    n_out,
                    nspf,
                    tfirst,
                    &mut elapsed,
                    frametimes,
                );
            }
            if cli_settings.quiet == 0 {
                print_stats(istty, n_out, total, elapsed, i_fps);
            }
        }
        if cli_settings.limit != 0 && n_out == cli_settings.limit {
            break;
        }
        if !(data.sz > 0 as libc::c_int as libc::c_ulong
            || input_read(in_0, &mut data) == 0)
        {
            break;
        }
    }
    if data.sz > 0 as libc::c_int as libc::c_ulong {
        dav1d_data_unref(&mut data);
    }
    if res == 0 as libc::c_int {
        while cli_settings.limit == 0 || n_out < cli_settings.limit {
            res = dav1d_get_picture(c, &mut p);
            if res < 0 as libc::c_int {
                if res != -(11 as libc::c_int) {
                    fprintf(
                        stderr,
                        b"Error decoding frame: %s\n\0" as *const u8
                            as *const libc::c_char,
                        strerror(-res),
                    );
                    if res != -(22 as libc::c_int) {
                        break;
                    }
                } else {
                    res = 0 as libc::c_int;
                    break;
                }
            } else {
                if n_out == 0 {
                    res = output_open(
                        &mut out,
                        cli_settings.muxer,
                        cli_settings.outputfile,
                        &mut p.p,
                        fps.as_mut_ptr() as *const libc::c_uint,
                    );
                    if res < 0 as libc::c_int {
                        if !frametimes.is_null() {
                            fclose(frametimes);
                        }
                        return 1 as libc::c_int;
                    }
                }
                res = output_write(out, &mut p);
                if res < 0 as libc::c_int {
                    break;
                }
                n_out = n_out.wrapping_add(1);
                if nspf != 0 || cli_settings.quiet == 0 {
                    synchronize(
                        cli_settings.realtime as libc::c_int,
                        cli_settings.realtime_cache,
                        n_out,
                        nspf,
                        tfirst,
                        &mut elapsed,
                        frametimes,
                    );
                }
                if cli_settings.quiet == 0 {
                    print_stats(istty, n_out, total, elapsed, i_fps);
                }
            }
        }
    }
    if !frametimes.is_null() {
        fclose(frametimes);
    }
    input_close(in_0);
    if !out.is_null() {
        if cli_settings.quiet == 0 && istty != 0 {
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        }
        if !(cli_settings.verify).is_null() {
            res |= output_verify(out, cli_settings.verify);
        } else {
            output_close(out);
        }
    } else {
        fprintf(stderr, b"No data decoded\n\0" as *const u8 as *const libc::c_char);
        res = 1 as libc::c_int;
    }
    dav1d_close(&mut c);
    return if res == 0 as libc::c_int { 0 as libc::c_int } else { 1 as libc::c_int };
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *const *mut libc::c_char,
            ) as i32,
        )
    }
}
