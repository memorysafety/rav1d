#![allow(non_upper_case_globals)]
#![allow(clippy::all)]

mod compat; // mod compat
mod input {
    mod annexb;
    pub mod input;
    mod ivf;
    mod section5;
} // mod input
mod output {
    mod md5;
    mod null;
    mod output;
    mod y4m2;
    mod yuv;
} // mod output
mod dav1d_cli_parse;

use crate::compat::stdio::stderr;
use crate::dav1d_cli_parse::parse;
use crate::dav1d_cli_parse::CLISettings;
use crate::dav1d_cli_parse::REALTIME_DISABLE;
use crate::input::input::input_close;
use crate::input::input::input_open;
use crate::input::input::input_read;
use crate::input::input::input_seek;
use crate::input::input::DemuxerContext;
use libc::EAGAIN;
use rav1d::dav1d_close;
use rav1d::dav1d_flush;
use rav1d::dav1d_get_picture;
use rav1d::dav1d_open;
use rav1d::dav1d_parse_sequence_header;
use rav1d::dav1d_picture_unref;
use rav1d::dav1d_send_data;
use rav1d::dav1d_version;
use rav1d::include::dav1d::common::Dav1dDataProps;
use rav1d::include::dav1d::common::Dav1dUserData;
use rav1d::include::dav1d::data::Dav1dData;
use rav1d::include::dav1d::dav1d::Dav1dContext;
use rav1d::include::dav1d::dav1d::Dav1dLogger;
use rav1d::include::dav1d::dav1d::Dav1dSettings;
use rav1d::include::dav1d::dav1d::DAV1D_DECODEFRAMETYPE_ALL;
use rav1d::include::dav1d::dav1d::DAV1D_INLOOPFILTER_NONE;
use rav1d::include::dav1d::headers::Dav1dColorPrimaries;
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
use std::ffi::c_char;
use std::ffi::c_double;
use std::ffi::c_float;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_void;
use std::ptr::NonNull;

#[cfg(target_os = "windows")]
unsafe fn get_seed() -> c_uint {
    windows_sys::Win32::System::SystemInformation::GetTickCount()
}

#[cfg(not(target_os = "windows"))]
unsafe fn get_seed() -> c_uint {
    use std::ffi::c_ulonglong;
    let mut ts: libc::timespec = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    libc::clock_gettime(1, &mut ts);
    return (1000000000 as c_ulonglong)
        .wrapping_mul(ts.tv_sec as c_ulonglong)
        .wrapping_add(ts.tv_nsec as c_ulonglong) as c_uint;
}

static mut xs_state: [u32; 4] = [0; 4];

unsafe fn xor128_srand(seed: c_uint) {
    xs_state[0] = seed;
    xs_state[1] = seed & 0xffff0000 | !seed & 0xffff;
    xs_state[2] = !seed & 0xffff0000 | seed & 0xffff;
    xs_state[3] = !seed;
}

unsafe fn xor128_rand() -> c_int {
    let x: u32 = xs_state[0];
    let t: u32 = x ^ x << 11;
    xs_state[0] = xs_state[1];
    xs_state[1] = xs_state[2];
    xs_state[2] = xs_state[3];
    let mut w: u32 = xs_state[3];
    w = w ^ w >> 19 ^ (t ^ t >> 8);
    xs_state[3] = w;
    return w as c_int >> 1;
}

#[inline]
unsafe fn decode_frame(
    p: *mut Dav1dPicture,
    c: Option<Dav1dContext>,
    data: *mut Dav1dData,
) -> c_int {
    let mut res: c_int;
    libc::memset(p as *mut c_void, 0, ::core::mem::size_of::<Dav1dPicture>());
    res = dav1d_send_data(c, NonNull::new(data)).0;
    if res < 0 {
        if res != -EAGAIN {
            libc::fprintf(
                stderr(),
                b"Error decoding frame: %s\n\0" as *const u8 as *const c_char,
                libc::strerror(-res),
            );
            return res;
        }
    }
    res = dav1d_get_picture(c, NonNull::new(p)).0;
    if res < 0 {
        if res != -EAGAIN {
            libc::fprintf(
                stderr(),
                b"Error decoding frame: %s\n\0" as *const u8 as *const c_char,
                libc::strerror(-res),
            );
            return res;
        }
    } else {
        dav1d_picture_unref(NonNull::new(p));
    }
    return 0 as c_int;
}

unsafe fn decode_rand(
    in_0: *mut DemuxerContext,
    c: Option<Dav1dContext>,
    data: *mut Dav1dData,
    fps: c_double,
) -> c_int {
    let mut res = 0;
    let mut p = Default::default();
    let num_frames: c_int = xor128_rand() % (fps * 5 as c_double) as c_int;
    let mut i = 0;
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

unsafe fn decode_all(
    in_0: *mut DemuxerContext,
    c: Option<Dav1dContext>,
    data: *mut Dav1dData,
) -> c_int {
    let mut res: c_int;
    let mut p = Default::default();
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

unsafe fn seek(
    in_0: *mut DemuxerContext,
    c: Option<Dav1dContext>,
    pts: u64,
    data: *mut Dav1dData,
) -> c_int {
    let mut res: c_int;
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
        if !(dav1d_parse_sequence_header(NonNull::new(&mut seq), (*data).data, (*data).sz).0 != 0) {
            break;
        }
    }
    dav1d_flush(c.unwrap());
    return res;
}

unsafe fn main_0(argc: c_int, argv: *const *mut c_char) -> c_int {
    let mut shift: c_uint;
    let mut current_block: u64;
    let version: *const c_char = dav1d_version();
    if libc::strcmp(version, b"966d63c1\0" as *const u8 as *const c_char) != 0 {
        libc::fprintf(
            stderr(),
            b"Version mismatch (library: %s, executable: %s)\n\0" as *const u8 as *const c_char,
            version,
            b"966d63c1\0" as *const u8 as *const c_char,
        );
        return 1 as c_int;
    }
    let mut cli_settings: CLISettings = CLISettings {
        outputfile: 0 as *const c_char,
        inputfile: 0 as *const c_char,
        demuxer: 0 as *const c_char,
        muxer: 0 as *const c_char,
        frametimes: 0 as *const c_char,
        verify: 0 as *const c_char,
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
            cookie: None,
            alloc_picture_callback: None,
            release_picture_callback: None,
        },
        logger: Dav1dLogger::new(None, None),
        strict_std_compliance: 0,
        output_invisible_frames: 0,
        inloop_filters: DAV1D_INLOOPFILTER_NONE,
        decode_frame_type: DAV1D_DECODEFRAMETYPE_ALL,
        reserved: [0; 16],
    };
    let mut in_0: *mut DemuxerContext = 0 as *mut DemuxerContext;
    let mut c: Option<Dav1dContext> = None;
    let mut data: Dav1dData = Dav1dData {
        data: None,
        sz: 0,
        r#ref: None,
        m: Dav1dDataProps {
            timestamp: 0,
            duration: 0,
            offset: 0,
            size: 0,
            user_data: Dav1dUserData {
                data: None,
                r#ref: None,
            },
        },
    };
    let mut total: c_uint = 0;
    let mut i_fps: [c_uint; 2] = [0; 2];
    let mut i_timebase: [c_uint; 2] = [0; 2];
    let timebase: c_double;
    let spf: c_double;
    let fps: c_double;
    let mut pts: u64;
    xor128_srand(get_seed());
    parse(argc, argv, &mut cli_settings, &mut lib_settings);
    if input_open(
        &mut in_0,
        b"ivf\0" as *const u8 as *const c_char,
        cli_settings.inputfile,
        i_fps.as_mut_ptr(),
        &mut total,
        i_timebase.as_mut_ptr(),
    ) < 0
        || i_timebase[0] == 0
        || i_timebase[1] == 0
        || i_fps[0] == 0
        || i_fps[1] == 0
    {
        return libc::EXIT_SUCCESS;
    }
    if dav1d_open(NonNull::new(&mut c), NonNull::new(&mut lib_settings)).0 != 0 {
        return libc::EXIT_FAILURE;
    }
    timebase = i_timebase[1] as c_double / i_timebase[0] as c_double;
    spf = i_fps[1] as c_double / i_fps[0] as c_double;
    fps = i_fps[0] as c_double / i_fps[1] as c_double;
    if !(fps < 1 as c_double) {
        let mut i = 0;
        loop {
            if !(i < 3) {
                current_block = 5948590327928692120;
                break;
            }
            pts =
                ((xor128_rand() as c_uint).wrapping_rem(total) as c_double * spf * 1000000000.0f64)
                    .round() as u64;
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
                pts = (data.m.timestamp as c_double * timebase * 1000000000.0f64).round() as u64;
                let mut i_0 = 0;
                let mut tries = 0;
                loop {
                    if !(i_0 - tries < 4 && tries < 4 / 2) {
                        current_block = 8693738493027456495;
                        break;
                    }
                    let sign: c_int = if xor128_rand() & 1 != 0 {
                        -1
                    } else {
                        1 as c_int
                    };
                    let diff: c_float = (xor128_rand() % 100 as c_int) as c_float / 100.0f32;
                    let mut new_pts: i64 = pts.wrapping_add((sign as u64).wrapping_mul(
                        (diff as c_double * fps * spf * 1000000000.0f64).round() as u64,
                    )) as i64;
                    let new_ts: i64 =
                        (new_pts as c_double / (timebase * 1000000000.0f64)).round() as i64;
                    new_pts =
                        (new_ts as c_double * timebase * 1000000000.0f64).round() as u64 as i64;
                    if new_pts < 0
                        || new_pts as u64
                            >= (total as c_double * spf * 1000000000.0f64).round() as u64
                    {
                        if seek(
                            in_0,
                            c,
                            (total.wrapping_div(2 as c_int as c_uint) as c_double
                                * spf
                                * 1000000000.0f64)
                                .round() as u64,
                            &mut data,
                        ) != 0
                        {
                            current_block = 8693738493027456495;
                            break;
                        }
                        pts = (data.m.timestamp as c_double * timebase * 1000000000.0f64).round()
                            as u64;
                        tries += 1;
                    } else {
                        if seek(in_0, c, new_pts as u64, &mut data) != 0 {
                            if seek(in_0, c, 0 as c_int as u64, &mut data) != 0 {
                                current_block = 1928200949476507836;
                                break;
                            }
                        }
                        if decode_rand(in_0, c, &mut data, fps) != 0 {
                            current_block = 1928200949476507836;
                            break;
                        }
                        pts = (data.m.timestamp as c_double * timebase * 1000000000.0f64).round()
                            as u64;
                    }
                    i_0 += 1;
                }
                match current_block {
                    1928200949476507836 => {}
                    _ => {
                        shift = 0 as c_int as c_uint;
                        loop {
                            shift = shift.wrapping_add(5 as c_int as c_uint);
                            if shift > total {
                                shift = total;
                            }
                            if !(seek(
                                in_0,
                                c,
                                (total.wrapping_sub(shift) as c_double * spf * 1000000000.0f64)
                                    .round() as u64,
                                &mut data,
                            ) != 0)
                            {
                                break;
                            }
                        }
                        // simulate seeking after the end of the file
                        let mut i_1 = 0;
                        while i_1 < 2 {
                            if seek(
                                in_0,
                                c,
                                (total.wrapping_sub(shift) as c_double * spf * 1000000000.0f64)
                                    .round() as u64,
                                &mut data,
                            ) != 0
                            {
                                break;
                            }
                            if decode_all(in_0, c, &mut data) != 0 {
                                break;
                            }
                            let mut num_flush = 1 + 64 + xor128_rand() % 64;
                            loop {
                                num_flush -= 1;
                                if num_flush == 0 {
                                    break;
                                }
                                dav1d_flush(c.unwrap());
                            }
                            i_1 += 1;
                        }
                    }
                }
            }
        }
    }
    input_close(in_0);
    dav1d_close(NonNull::new(&mut c));
    return libc::EXIT_SUCCESS;
}

pub fn main() {
    let mut args: Vec<*mut c_char> = Vec::new();
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
            (args.len() - 1) as c_int,
            args.as_mut_ptr() as *const *mut c_char,
        ) as i32)
    }
}
