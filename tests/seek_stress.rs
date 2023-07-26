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
extern crate rav1d;
#[path = "../tools/input"]
mod input {
    mod annexb;
    mod input;
    mod ivf;
    mod section5;
} // mod input
#[path = "../tools/output"]
mod output {
    mod md5;
    mod null;
    mod output;
    mod y4m2;
    mod yuv;
} // mod output
use rav1d::include::dav1d::common::Dav1dDataProps;
use rav1d::include::dav1d::common::Dav1dUserData;
use rav1d::include::dav1d::data::Dav1dData;
use rav1d::include::dav1d::dav1d::Dav1dLogger;
use rav1d::include::dav1d::dav1d::DAV1D_DECODEFRAMETYPE_ALL;
use rav1d::include::dav1d::dav1d::DAV1D_INLOOPFILTER_NONE;
use rav1d::include::dav1d::headers::Dav1dColorPrimaries;
use rav1d::include::dav1d::headers::Dav1dContentLightLevel;
use rav1d::include::dav1d::headers::Dav1dFrameHeader;
use rav1d::include::dav1d::headers::Dav1dITUTT35;
use rav1d::include::dav1d::headers::Dav1dMasteringDisplay;
use rav1d::include::dav1d::headers::Dav1dSequenceHeader;
use rav1d::include::dav1d::headers::Dav1dSequenceHeaderOperatingParameterInfo;
use rav1d::include::dav1d::headers::Dav1dSequenceHeaderOperatingPoint;
use rav1d::include::dav1d::headers::Dav1dTransferCharacteristics;
use rav1d::include::dav1d::headers::DAV1D_CHR_UNKNOWN;
use rav1d::include::dav1d::headers::DAV1D_MC_IDENTITY;
use rav1d::include::dav1d::headers::DAV1D_OFF;
use rav1d::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I400;
use rav1d::include::dav1d::picture::Dav1dPicAllocator;
use rav1d::include::dav1d::picture::Dav1dPicture;
use rav1d::include::dav1d::picture::Dav1dPictureParameters;
use rav1d::include::stdint::int64_t;
use rav1d::include::stdint::uint32_t;
use rav1d::include::stdint::uint64_t;
use rav1d::include::stdint::uint8_t;
use rav1d::src::lib::dav1d_close;
use rav1d::src::lib::dav1d_flush;
use rav1d::src::lib::dav1d_get_picture;
use rav1d::src::lib::dav1d_open;
use rav1d::src::lib::dav1d_parse_sequence_header;
use rav1d::src::lib::dav1d_picture_unref;
use rav1d::src::lib::dav1d_send_data;
use rav1d::src::lib::dav1d_version;
use rav1d::src::lib::Dav1dContext;
use rav1d::src::lib::Dav1dSettings;
use rav1d::src::r#ref::Dav1dRef;
#[path = "../tools/dav1d_cli_parse.rs"]
mod dav1d_cli_parse;
extern "C" {
    pub type DemuxerContext;
    fn llround(_: libc::c_double) -> libc::c_longlong;
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
// NOTE: temporary code to support Linux and macOS, should be removed eventually
cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        extern "C" {
            pub static mut stdout: *mut libc::FILE;
            pub static mut stderr: *mut libc::FILE;
        }

        unsafe fn errno_location() -> *mut libc::c_int {
            libc::__errno_location()
        }
    } else if #[cfg(target_os = "macos")] {
        extern "C" {
            #[link_name = "__stdoutp"]
            static mut stdout: *mut libc::FILE;
            #[link_name = "__stderrp"]
            static mut stderr: *mut libc::FILE;
        }

        unsafe fn errno_location() -> *mut libc::c_int {
            libc::__error()
        }
    }
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
unsafe fn xor128_srand(mut seed: libc::c_uint) {
    xs_state[0] = seed;
    xs_state[1] = seed & 0xffff0000 | !seed & 0xffff;
    xs_state[2] = !seed & 0xffff0000 | seed & 0xffff;
    xs_state[3] = !seed;
}
unsafe fn xor128_rand() -> libc::c_int {
    let x: uint32_t = xs_state[0];
    let t: uint32_t = x ^ x << 11;
    xs_state[0] = xs_state[1];
    xs_state[1] = xs_state[2];
    xs_state[2] = xs_state[3];
    let mut w: uint32_t = xs_state[3];
    w = w ^ w >> 19 ^ (t ^ t >> 8);
    xs_state[3] = w;
    return w as libc::c_int >> 1;
}
#[inline]
unsafe extern "C" fn decode_frame(
    p: *mut Dav1dPicture,
    c: *mut Dav1dContext,
    data: *mut Dav1dData,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    libc::memset(
        p as *mut libc::c_void,
        0,
        ::core::mem::size_of::<Dav1dPicture>(),
    );
    res = dav1d_send_data(c, data);
    if res < 0 {
        if res != -11 {
            libc::fprintf(
                stderr,
                b"Error decoding frame: %s\n\0" as *const u8 as *const libc::c_char,
                libc::strerror(-res),
            );
            return res;
        }
    }
    res = dav1d_get_picture(c, p);
    if res < 0 {
        if res != -(11 as libc::c_int) {
            libc::fprintf(
                stderr,
                b"Error decoding frame: %s\n\0" as *const u8 as *const libc::c_char,
                libc::strerror(-res),
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
    let mut res: libc::c_int = 0;
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
    if libc::strcmp(version, b"966d63c1\0" as *const u8 as *const libc::c_char) != 0 {
        libc::fprintf(
            stderr,
            b"Version mismatch (library: %s, executable: %s)\n\0" as *const u8
                as *const libc::c_char,
            version,
            b"966d63c1\0" as *const u8 as *const libc::c_char,
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
        || i_timebase[0] == 0
        || i_timebase[1] == 0
        || i_fps[0] == 0
        || i_fps[1] == 0
    {
        return libc::EXIT_SUCCESS;
    }
    if dav1d_open(&mut c, &mut lib_settings) != 0 {
        return libc::EXIT_FAILURE;
    }
    timebase = i_timebase[1] as libc::c_double / i_timebase[0] as libc::c_double;
    spf = i_fps[1] as libc::c_double / i_fps[0] as libc::c_double;
    fps = i_fps[0] as libc::c_double / i_fps[1] as libc::c_double;
    if !(fps < 1 as libc::c_double) {
        let mut i: libc::c_int = 0;
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
                        // simulate seeking after the end of the file
                        let mut i_1: libc::c_int = 0;
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
                            let mut num_flush: libc::c_int = 1 + 64 + xor128_rand() % 64;
                            loop {
                                num_flush -= 1;
                                if num_flush == 0 {
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
    return libc::EXIT_SUCCESS;
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
