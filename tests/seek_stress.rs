#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
#![feature(c_variadic)]
extern crate c2rust_out;
use c2rust_out::include::dav1d::common::Dav1dDataProps;
use c2rust_out::include::dav1d::common::Dav1dUserData;
use c2rust_out::include::dav1d::data::Dav1dData;
use c2rust_out::include::dav1d::dav1d::Dav1dLogger;
use c2rust_out::include::dav1d::headers::Dav1dContentLightLevel;
use c2rust_out::include::dav1d::headers::Dav1dFrameHeader;
use c2rust_out::include::dav1d::headers::Dav1dITUTT35;
use c2rust_out::include::dav1d::headers::Dav1dMasteringDisplay;
use c2rust_out::include::dav1d::headers::Dav1dSequenceHeader;
use c2rust_out::include::dav1d::headers::Dav1dSequenceHeaderOperatingParameterInfo;
use c2rust_out::include::dav1d::headers::Dav1dSequenceHeaderOperatingPoint;
use c2rust_out::include::dav1d::headers::Dav1dWarpedMotionType;
use c2rust_out::include::dav1d::headers::DAV1D_OFF;
use c2rust_out::include::dav1d::picture::Dav1dPicAllocator;
use c2rust_out::include::dav1d::picture::Dav1dPicture;
use c2rust_out::include::dav1d::picture::Dav1dPictureParameters;
use c2rust_out::include::stdint::int16_t;
use c2rust_out::include::stdint::int32_t;
use c2rust_out::include::stdint::int64_t;
use c2rust_out::include::stdint::int8_t;
use c2rust_out::include::stdint::uint32_t;
use c2rust_out::include::stdint::uint64_t;
use c2rust_out::include::stdint::uint8_t;
use c2rust_out::src::lib::Dav1dSettings;
use c2rust_out::src::r#ref::Dav1dRef;
use c2rust_out::stderr;
extern "C" {
    pub type Dav1dContext;
    pub type DemuxerContext;
    pub type DemuxerPriv;
    fn llround(_: libc::c_double) -> libc::c_longlong;
    fn fprintf(_: *mut libc::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::size_t) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn dav1d_version() -> *const libc::c_char;
    fn dav1d_open(c_out: *mut *mut Dav1dContext, s: *const Dav1dSettings) -> libc::c_int;
    fn dav1d_parse_sequence_header(
        out: *mut Dav1dSequenceHeader,
        buf: *const uint8_t,
        sz: libc::size_t,
    ) -> libc::c_int;
    fn dav1d_send_data(c: *mut Dav1dContext, in_0: *mut Dav1dData) -> libc::c_int;
    fn dav1d_get_picture(c: *mut Dav1dContext, out: *mut Dav1dPicture) -> libc::c_int;
    fn dav1d_close(c_out: *mut *mut Dav1dContext);
    fn dav1d_flush(c: *mut Dav1dContext);
    fn dav1d_picture_unref(p: *mut Dav1dPicture);
    fn input_open(
        c_out: *mut *mut DemuxerContext,
        name: *const libc::c_char,
        filename: *const libc::c_char,
        fps: *mut libc::c_uint,
        num_frames: *mut libc::c_uint,
        timebase: *mut libc::c_uint,
    ) -> libc::c_int;
    fn input_read(ctx: *mut DemuxerContext, data: *mut Dav1dData) -> libc::c_int;
    fn input_seek(ctx: *mut DemuxerContext, pts: uint64_t) -> libc::c_int;
    fn input_close(ctx: *mut DemuxerContext);
    fn parse(
        argc: libc::c_int,
        argv: *const *mut libc::c_char,
        cli_settings: *mut CLISettings,
        lib_settings: *mut Dav1dSettings,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dWarpedMotionParams {
    pub type_0: Dav1dWarpedMotionType,
    pub matrix: [int32_t; 6],
    pub u: Dav1dWarpedMotionParams_u,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Dav1dWarpedMotionParams_u {
    pub p: Dav1dWarpedMotionParams_u_p,
    pub abcd: [int16_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dWarpedMotionParams_u_p {
    pub alpha: int16_t,
    pub beta: int16_t,
    pub gamma: int16_t,
    pub delta: int16_t,
}
pub type Dav1dPixelLayout = libc::c_uint;
pub const DAV1D_PIXEL_LAYOUT_I444: Dav1dPixelLayout = 3;
pub const DAV1D_PIXEL_LAYOUT_I422: Dav1dPixelLayout = 2;
pub const DAV1D_PIXEL_LAYOUT_I420: Dav1dPixelLayout = 1;
pub const DAV1D_PIXEL_LAYOUT_I400: Dav1dPixelLayout = 0;
pub type Dav1dFrameType = libc::c_uint;
pub const DAV1D_FRAME_TYPE_SWITCH: Dav1dFrameType = 3;
pub const DAV1D_FRAME_TYPE_INTRA: Dav1dFrameType = 2;
pub const DAV1D_FRAME_TYPE_INTER: Dav1dFrameType = 1;
pub const DAV1D_FRAME_TYPE_KEY: Dav1dFrameType = 0;
pub type Dav1dColorPrimaries = libc::c_uint;
pub const DAV1D_COLOR_PRI_RESERVED: Dav1dColorPrimaries = 255;
pub const DAV1D_COLOR_PRI_EBU3213: Dav1dColorPrimaries = 22;
pub const DAV1D_COLOR_PRI_SMPTE432: Dav1dColorPrimaries = 12;
pub const DAV1D_COLOR_PRI_SMPTE431: Dav1dColorPrimaries = 11;
pub const DAV1D_COLOR_PRI_XYZ: Dav1dColorPrimaries = 10;
pub const DAV1D_COLOR_PRI_BT2020: Dav1dColorPrimaries = 9;
pub const DAV1D_COLOR_PRI_FILM: Dav1dColorPrimaries = 8;
pub const DAV1D_COLOR_PRI_SMPTE240: Dav1dColorPrimaries = 7;
pub const DAV1D_COLOR_PRI_BT601: Dav1dColorPrimaries = 6;
pub const DAV1D_COLOR_PRI_BT470BG: Dav1dColorPrimaries = 5;
pub const DAV1D_COLOR_PRI_BT470M: Dav1dColorPrimaries = 4;
pub const DAV1D_COLOR_PRI_UNKNOWN: Dav1dColorPrimaries = 2;
pub const DAV1D_COLOR_PRI_BT709: Dav1dColorPrimaries = 1;
pub type Dav1dTransferCharacteristics = libc::c_uint;
pub const DAV1D_TRC_RESERVED: Dav1dTransferCharacteristics = 255;
pub const DAV1D_TRC_HLG: Dav1dTransferCharacteristics = 18;
pub const DAV1D_TRC_SMPTE428: Dav1dTransferCharacteristics = 17;
pub const DAV1D_TRC_SMPTE2084: Dav1dTransferCharacteristics = 16;
pub const DAV1D_TRC_BT2020_12BIT: Dav1dTransferCharacteristics = 15;
pub const DAV1D_TRC_BT2020_10BIT: Dav1dTransferCharacteristics = 14;
pub const DAV1D_TRC_SRGB: Dav1dTransferCharacteristics = 13;
pub const DAV1D_TRC_BT1361: Dav1dTransferCharacteristics = 12;
pub const DAV1D_TRC_IEC61966: Dav1dTransferCharacteristics = 11;
pub const DAV1D_TRC_LOG100_SQRT10: Dav1dTransferCharacteristics = 10;
pub const DAV1D_TRC_LOG100: Dav1dTransferCharacteristics = 9;
pub const DAV1D_TRC_LINEAR: Dav1dTransferCharacteristics = 8;
pub const DAV1D_TRC_SMPTE240: Dav1dTransferCharacteristics = 7;
pub const DAV1D_TRC_BT601: Dav1dTransferCharacteristics = 6;
pub const DAV1D_TRC_BT470BG: Dav1dTransferCharacteristics = 5;
pub const DAV1D_TRC_BT470M: Dav1dTransferCharacteristics = 4;
pub const DAV1D_TRC_UNKNOWN: Dav1dTransferCharacteristics = 2;
pub const DAV1D_TRC_BT709: Dav1dTransferCharacteristics = 1;
pub type Dav1dMatrixCoefficients = libc::c_uint;
pub const DAV1D_MC_RESERVED: Dav1dMatrixCoefficients = 255;
pub const DAV1D_MC_ICTCP: Dav1dMatrixCoefficients = 14;
pub const DAV1D_MC_CHROMAT_CL: Dav1dMatrixCoefficients = 13;
pub const DAV1D_MC_CHROMAT_NCL: Dav1dMatrixCoefficients = 12;
pub const DAV1D_MC_SMPTE2085: Dav1dMatrixCoefficients = 11;
pub const DAV1D_MC_BT2020_CL: Dav1dMatrixCoefficients = 10;
pub const DAV1D_MC_BT2020_NCL: Dav1dMatrixCoefficients = 9;
pub const DAV1D_MC_SMPTE_YCGCO: Dav1dMatrixCoefficients = 8;
pub const DAV1D_MC_SMPTE240: Dav1dMatrixCoefficients = 7;
pub const DAV1D_MC_BT601: Dav1dMatrixCoefficients = 6;
pub const DAV1D_MC_BT470BG: Dav1dMatrixCoefficients = 5;
pub const DAV1D_MC_FCC: Dav1dMatrixCoefficients = 4;
pub const DAV1D_MC_UNKNOWN: Dav1dMatrixCoefficients = 2;
pub const DAV1D_MC_BT709: Dav1dMatrixCoefficients = 1;
pub const DAV1D_MC_IDENTITY: Dav1dMatrixCoefficients = 0;
pub type Dav1dChromaSamplePosition = libc::c_uint;
pub const DAV1D_CHR_COLOCATED: Dav1dChromaSamplePosition = 2;
pub const DAV1D_CHR_VERTICAL: Dav1dChromaSamplePosition = 1;
pub const DAV1D_CHR_UNKNOWN: Dav1dChromaSamplePosition = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dSegmentationData {
    pub delta_q: libc::c_int,
    pub delta_lf_y_v: libc::c_int,
    pub delta_lf_y_h: libc::c_int,
    pub delta_lf_u: libc::c_int,
    pub delta_lf_v: libc::c_int,
    pub ref_0: libc::c_int,
    pub skip: libc::c_int,
    pub globalmv: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dSegmentationDataSet {
    pub d: [Dav1dSegmentationData; 8],
    pub preskip: libc::c_int,
    pub last_active_segid: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dLoopfilterModeRefDeltas {
    pub mode_delta: [libc::c_int; 2],
    pub ref_delta: [libc::c_int; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFilmGrainData {
    pub seed: libc::c_uint,
    pub num_y_points: libc::c_int,
    pub y_points: [[uint8_t; 2]; 14],
    pub chroma_scaling_from_luma: libc::c_int,
    pub num_uv_points: [libc::c_int; 2],
    pub uv_points: [[[uint8_t; 2]; 10]; 2],
    pub scaling_shift: libc::c_int,
    pub ar_coeff_lag: libc::c_int,
    pub ar_coeffs_y: [int8_t; 24],
    pub ar_coeffs_uv: [[int8_t; 28]; 2],
    pub ar_coeff_shift: uint64_t,
    pub grain_scale_shift: libc::c_int,
    pub uv_mult: [libc::c_int; 2],
    pub uv_luma_mult: [libc::c_int; 2],
    pub uv_offset: [libc::c_int; 2],
    pub overlap_flag: libc::c_int,
    pub clip_to_restricted_range: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameHeaderOperatingPoint {
    pub buffer_removal_time: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader_film_grain {
    pub data: Dav1dFilmGrainData,
    pub present: libc::c_int,
    pub update: libc::c_int,
}
pub type Dav1dInloopFilterType = libc::c_uint;
pub const DAV1D_INLOOPFILTER_ALL: Dav1dInloopFilterType = 7;
pub const DAV1D_INLOOPFILTER_RESTORATION: Dav1dInloopFilterType = 4;
pub const DAV1D_INLOOPFILTER_CDEF: Dav1dInloopFilterType = 2;
pub const DAV1D_INLOOPFILTER_DEBLOCK: Dav1dInloopFilterType = 1;
pub const DAV1D_INLOOPFILTER_NONE: Dav1dInloopFilterType = 0;
pub type Dav1dDecodeFrameType = libc::c_uint;
pub const DAV1D_DECODEFRAMETYPE_KEY: Dav1dDecodeFrameType = 3;
pub const DAV1D_DECODEFRAMETYPE_INTRA: Dav1dDecodeFrameType = 2;
pub const DAV1D_DECODEFRAMETYPE_REFERENCE: Dav1dDecodeFrameType = 1;
pub const DAV1D_DECODEFRAMETYPE_ALL: Dav1dDecodeFrameType = 0;
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
    pub realtime: CLISettings_realtime,
    pub realtime_fps: libc::c_double,
    pub realtime_cache: libc::c_uint,
    pub neg_stride: libc::c_int,
}
pub type CLISettings_realtime = libc::c_uint;
pub const REALTIME_CUSTOM: CLISettings_realtime = 2;
pub const REALTIME_INPUT: CLISettings_realtime = 1;
pub const REALTIME_DISABLE: CLISettings_realtime = 0;
unsafe extern "C" fn get_seed() -> libc::c_uint {
    let mut ts: libc::timespec = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    libc::clock_gettime(1, &mut ts);
    return (1000000000 as libc::c_ulonglong)
        .wrapping_mul(ts.tv_sec as libc::c_ulonglong)
        .wrapping_add(ts.tv_nsec as libc::c_ulonglong) as libc::c_uint;
}
static mut xs_state: [uint32_t; 4] = [0; 4];
unsafe extern "C" fn xor128_srand(mut seed: libc::c_uint) {
    xs_state[0 as libc::c_int as usize] = seed;
    xs_state[1 as libc::c_int as usize] =
        seed & 0xffff0000 as libc::c_uint | !seed & 0xffff as libc::c_int as libc::c_uint;
    xs_state[2 as libc::c_int as usize] =
        !seed & 0xffff0000 as libc::c_uint | seed & 0xffff as libc::c_int as libc::c_uint;
    xs_state[3 as libc::c_int as usize] = !seed;
}
unsafe extern "C" fn xor128_rand() -> libc::c_int {
    let x: uint32_t = xs_state[0 as libc::c_int as usize];
    let t: uint32_t = x ^ x << 11 as libc::c_int;
    xs_state[0 as libc::c_int as usize] = xs_state[1 as libc::c_int as usize];
    xs_state[1 as libc::c_int as usize] = xs_state[2 as libc::c_int as usize];
    xs_state[2 as libc::c_int as usize] = xs_state[3 as libc::c_int as usize];
    let mut w: uint32_t = xs_state[3 as libc::c_int as usize];
    w = w ^ w >> 19 as libc::c_int ^ (t ^ t >> 8 as libc::c_int);
    xs_state[3 as libc::c_int as usize] = w;
    return (w >> 1 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn decode_frame(
    p: *mut Dav1dPicture,
    c: *mut Dav1dContext,
    data: *mut Dav1dData,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    memset(
        p as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Dav1dPicture>(),
    );
    res = dav1d_send_data(c, data);
    if res < 0 as libc::c_int {
        if res != -(11 as libc::c_int) {
            fprintf(
                stderr,
                b"Error decoding frame: %s\n\0" as *const u8 as *const libc::c_char,
                strerror(-res),
            );
            return res;
        }
    }
    res = dav1d_get_picture(c, p);
    if res < 0 as libc::c_int {
        if res != -(11 as libc::c_int) {
            fprintf(
                stderr,
                b"Error decoding frame: %s\n\0" as *const u8 as *const libc::c_char,
                strerror(-res),
            );
            return res;
        }
    } else {
        dav1d_picture_unref(p);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn decode_rand(
    in_0: *mut DemuxerContext,
    c: *mut Dav1dContext,
    data: *mut Dav1dData,
    fps: libc::c_double,
) -> libc::c_int {
    let mut res: libc::c_int = 0 as libc::c_int;
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
                r#ref: 0 as *mut Dav1dRef,
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
        r#ref: 0 as *mut Dav1dRef,
        allocator_data: 0 as *mut libc::c_void,
    };
    let num_frames: libc::c_int =
        xor128_rand() % (fps * 5 as libc::c_int as libc::c_double) as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < num_frames {
        res = decode_frame(&mut p, c, data);
        if res != 0 {
            break;
        }
        if input_read(in_0, data) != 0 || (*data).sz == 0 {
            break;
        }
        i += 1;
    }
    return res;
}
unsafe extern "C" fn decode_all(
    in_0: *mut DemuxerContext,
    c: *mut Dav1dContext,
    data: *mut Dav1dData,
) -> libc::c_int {
    let mut res: libc::c_int = 0 as libc::c_int;
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
                r#ref: 0 as *mut Dav1dRef,
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
        r#ref: 0 as *mut Dav1dRef,
        allocator_data: 0 as *mut libc::c_void,
    };
    loop {
        res = decode_frame(&mut p, c, data);
        if res != 0 {
            break;
        }
        if !(input_read(in_0, data) == 0 && (*data).sz > 0) {
            break;
        }
    }
    return res;
}
unsafe extern "C" fn seek(
    in_0: *mut DemuxerContext,
    c: *mut Dav1dContext,
    pts: uint64_t,
    data: *mut Dav1dData,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    res = input_seek(in_0, pts);
    if res != 0 {
        return res;
    }
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
    loop {
        res = input_read(in_0, data);
        if res != 0 {
            break;
        }
        if !(dav1d_parse_sequence_header(&mut seq, (*data).data, (*data).sz) != 0) {
            break;
        }
    }
    dav1d_flush(c);
    return res;
}
unsafe fn main_0(argc: libc::c_int, argv: *const *mut libc::c_char) -> libc::c_int {
    let mut shift: libc::c_uint = 0;
    let mut current_block: u64;
    let mut version: *const libc::c_char = dav1d_version();
    if strcmp(
        version,
        b"1.0.0-130-g26eca15\0" as *const u8 as *const libc::c_char,
    ) != 0
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
    let mut c: *mut Dav1dContext = 0 as *mut Dav1dContext;
    let mut data: Dav1dData = Dav1dData {
        data: 0 as *const uint8_t,
        sz: 0,
        r#ref: 0 as *mut Dav1dRef,
        m: Dav1dDataProps {
            timestamp: 0,
            duration: 0,
            offset: 0,
            size: 0,
            user_data: Dav1dUserData {
                data: 0 as *const uint8_t,
                r#ref: 0 as *mut Dav1dRef,
            },
        },
    };
    let mut total: libc::c_uint = 0;
    let mut i_fps: [libc::c_uint; 2] = [0; 2];
    let mut i_timebase: [libc::c_uint; 2] = [0; 2];
    let mut timebase: libc::c_double = 0.;
    let mut spf: libc::c_double = 0.;
    let mut fps: libc::c_double = 0.;
    let mut pts: uint64_t = 0;
    xor128_srand(get_seed());
    parse(argc, argv, &mut cli_settings, &mut lib_settings);
    if input_open(
        &mut in_0,
        b"ivf\0" as *const u8 as *const libc::c_char,
        cli_settings.inputfile,
        i_fps.as_mut_ptr(),
        &mut total,
        i_timebase.as_mut_ptr(),
    ) < 0 as libc::c_int
        || i_timebase[0 as libc::c_int as usize] == 0
        || i_timebase[1 as libc::c_int as usize] == 0
        || i_fps[0 as libc::c_int as usize] == 0
        || i_fps[1 as libc::c_int as usize] == 0
    {
        return 0 as libc::c_int;
    }
    if dav1d_open(&mut c, &mut lib_settings) != 0 {
        return 1 as libc::c_int;
    }
    timebase = i_timebase[1 as libc::c_int as usize] as libc::c_double
        / i_timebase[0 as libc::c_int as usize] as libc::c_double;
    spf = i_fps[1 as libc::c_int as usize] as libc::c_double
        / i_fps[0 as libc::c_int as usize] as libc::c_double;
    fps = i_fps[0 as libc::c_int as usize] as libc::c_double
        / i_fps[1 as libc::c_int as usize] as libc::c_double;
    if !(fps < 1 as libc::c_int as libc::c_double) {
        let mut i: libc::c_int = 0 as libc::c_int;
        loop {
            if !(i < 3 as libc::c_int) {
                current_block = 5948590327928692120;
                break;
            }
            pts = llround(
                (xor128_rand() as libc::c_uint).wrapping_rem(total) as libc::c_double
                    * spf
                    * 1000000000.0f64,
            ) as uint64_t;
            if !(seek(in_0, c, pts, &mut data) != 0) {
                if decode_rand(in_0, c, &mut data, fps) != 0 {
                    current_block = 1928200949476507836;
                    break;
                }
            }
            i += 1;
        }
        match current_block {
            1928200949476507836 => {}
            _ => {
                pts = llround(data.m.timestamp as libc::c_double * timebase * 1000000000.0f64)
                    as uint64_t;
                let mut i_0: libc::c_int = 0 as libc::c_int;
                let mut tries: libc::c_int = 0 as libc::c_int;
                loop {
                    if !(i_0 - tries < 4 as libc::c_int
                        && tries < 4 as libc::c_int / 2 as libc::c_int)
                    {
                        current_block = 8693738493027456495;
                        break;
                    }
                    let sign: libc::c_int = if xor128_rand() & 1 as libc::c_int != 0 {
                        -(1 as libc::c_int)
                    } else {
                        1 as libc::c_int
                    };
                    let diff: libc::c_float =
                        (xor128_rand() % 100 as libc::c_int) as libc::c_float / 100.0f32;
                    let mut new_pts: int64_t = pts.wrapping_add((sign as uint64_t).wrapping_mul(
                        llround(diff as libc::c_double * fps * spf * 1000000000.0f64) as uint64_t,
                    )) as int64_t;
                    let new_ts: int64_t =
                        llround(new_pts as libc::c_double / (timebase * 1000000000.0f64))
                            as int64_t;
                    new_pts = llround(new_ts as libc::c_double * timebase * 1000000000.0f64)
                        as uint64_t as int64_t;
                    if new_pts < 0
                        || new_pts as uint64_t
                            >= llround(total as libc::c_double * spf * 1000000000.0f64) as uint64_t
                    {
                        if seek(
                            in_0,
                            c,
                            llround(
                                total.wrapping_div(2 as libc::c_int as libc::c_uint)
                                    as libc::c_double
                                    * spf
                                    * 1000000000.0f64,
                            ) as uint64_t,
                            &mut data,
                        ) != 0
                        {
                            current_block = 8693738493027456495;
                            break;
                        }
                        pts = llround(
                            data.m.timestamp as libc::c_double * timebase * 1000000000.0f64,
                        ) as uint64_t;
                        tries += 1;
                    } else {
                        if seek(in_0, c, new_pts as uint64_t, &mut data) != 0 {
                            if seek(in_0, c, 0 as libc::c_int as uint64_t, &mut data) != 0 {
                                current_block = 1928200949476507836;
                                break;
                            }
                        }
                        if decode_rand(in_0, c, &mut data, fps) != 0 {
                            current_block = 1928200949476507836;
                            break;
                        }
                        pts = llround(
                            data.m.timestamp as libc::c_double * timebase * 1000000000.0f64,
                        ) as uint64_t;
                    }
                    i_0 += 1;
                }
                match current_block {
                    1928200949476507836 => {}
                    _ => {
                        shift = 0 as libc::c_int as libc::c_uint;
                        loop {
                            shift = shift.wrapping_add(5 as libc::c_int as libc::c_uint);
                            if shift > total {
                                shift = total;
                            }
                            if !(seek(
                                in_0,
                                c,
                                llround(
                                    total.wrapping_sub(shift) as libc::c_double
                                        * spf
                                        * 1000000000.0f64,
                                ) as uint64_t,
                                &mut data,
                            ) != 0)
                            {
                                break;
                            }
                        }
                        let mut i_1: libc::c_int = 0 as libc::c_int;
                        while i_1 < 2 as libc::c_int {
                            if seek(
                                in_0,
                                c,
                                llround(
                                    total.wrapping_sub(shift) as libc::c_double
                                        * spf
                                        * 1000000000.0f64,
                                ) as uint64_t,
                                &mut data,
                            ) != 0
                            {
                                break;
                            }
                            if decode_all(in_0, c, &mut data) != 0 {
                                break;
                            }
                            let mut num_flush: libc::c_int = 1 as libc::c_int
                                + 64 as libc::c_int
                                + xor128_rand() % 64 as libc::c_int;
                            loop {
                                let fresh0 = num_flush;
                                num_flush = num_flush - 1;
                                if !(fresh0 != 0) {
                                    break;
                                }
                                dav1d_flush(c);
                            }
                            i_1 += 1;
                        }
                    }
                }
            }
        }
    }
    input_close(in_0);
    dav1d_close(&mut c);
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *const *mut libc::c_char,
        ) as i32)
    }
}
