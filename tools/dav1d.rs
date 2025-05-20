#![allow(non_upper_case_globals)]
#![allow(clippy::all)]

mod compat;
mod input {
    mod annexb;
    pub mod input;
    mod ivf;
    mod section5;
} // mod input
mod output {
    mod md5;
    mod null;
    pub mod output;
    mod y4m2;
    mod yuv;
} // mod output
mod dav1d_cli_parse;

use crate::compat::stdio::snprintf;
use crate::compat::stdio::stderr;
use crate::dav1d_cli_parse::parse;
use crate::dav1d_cli_parse::CLISettings;
use crate::dav1d_cli_parse::REALTIME_CUSTOM;
use crate::dav1d_cli_parse::REALTIME_DISABLE;
use crate::input::input::input_close;
use crate::input::input::input_open;
use crate::input::input::input_read;
use crate::input::input::DemuxerContext;
use crate::output::output::output_close;
use crate::output::output::output_open;
use crate::output::output::output_verify;
use crate::output::output::output_write;
use crate::output::output::MuxerContext;
use libc::calloc;
use libc::fclose;
use libc::fflush;
use libc::fileno;
use libc::fopen;
use libc::fprintf;
use libc::fputs;
use libc::free;
use libc::isatty;
use libc::memset;
use libc::ptrdiff_t;
use libc::strcpy;
use libc::strerror;
use libc::EAGAIN;
use libc::EINVAL;
use rav1d::dav1d_close;
use rav1d::dav1d_data_unref;
use rav1d::dav1d_get_picture;
use rav1d::dav1d_open;
use rav1d::dav1d_parse_sequence_header;
use rav1d::dav1d_send_data;
use rav1d::dav1d_version;
use rav1d::dav1d_version_api;
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
use rav1d::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;
use rav1d::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;
use rav1d::include::dav1d::picture::Dav1dPicAllocator;
use rav1d::include::dav1d::picture::Dav1dPicture;
use rav1d::include::dav1d::picture::DAV1D_PICTURE_ALIGNMENT;
use rav1d::send_sync_non_null::SendSyncNonNull;
use rav1d::Dav1dResult;
use rav1d::DAV1D_API_VERSION_MAJOR;
use rav1d::DAV1D_API_VERSION_MINOR;
use rav1d::DAV1D_API_VERSION_PATCH;
use std::ffi::c_char;
use std::ffi::c_double;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_ulonglong;
use std::ffi::c_void;
use std::ptr::NonNull;
use std::time::Duration;

#[cfg(target_os = "windows")]
unsafe fn get_time_nanos() -> u64 {
    use windows_sys::Win32::System::Performance::QueryPerformanceCounter;
    use windows_sys::Win32::System::Performance::QueryPerformanceFrequency;

    let mut frequency = 0i64;
    QueryPerformanceFrequency(&mut frequency);
    let mut t = 0i64;
    QueryPerformanceCounter(&mut t);
    let seconds: u64 = (t / frequency).try_into().unwrap();
    let fractions: u64 = (t % frequency).try_into().unwrap();
    return 1000000000 * seconds + 1000000000 * fractions / frequency as u64;
}

#[cfg(not(target_os = "windows"))]
unsafe fn get_time_nanos() -> u64 {
    let mut ts: libc::timespec = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    libc::clock_gettime(1, &mut ts);
    return (1000000000 as c_ulonglong)
        .wrapping_mul(ts.tv_sec as c_ulonglong)
        .wrapping_add(ts.tv_nsec as c_ulonglong) as u64;
}

unsafe fn sleep_nanos(d: u64) {
    std::thread::sleep(Duration::from_nanos(d));
}

unsafe fn synchronize(
    realtime: c_int,
    cache: c_uint,
    n_out: c_uint,
    nspf: u64,
    tfirst: u64,
    elapsed: *mut u64,
    frametimes: *mut libc::FILE,
) {
    let tcurr: u64 = get_time_nanos();
    let last: u64 = *elapsed;
    *elapsed = tcurr.wrapping_sub(tfirst);
    if realtime != 0 {
        let deadline: u64 = nspf.wrapping_mul(n_out as u64);
        if *elapsed < deadline {
            let remaining: u64 = deadline.wrapping_sub(*elapsed);
            if remaining > nspf.wrapping_mul(cache as u64) {
                sleep_nanos(remaining.wrapping_sub(nspf.wrapping_mul(cache as u64)));
            }
            *elapsed = deadline;
        }
    }
    if !frametimes.is_null() {
        let frametime: u64 = (*elapsed).wrapping_sub(last);
        fprintf(
            frametimes,
            b"%lu\n\0" as *const u8 as *const c_char,
            frametime,
        );
        fflush(frametimes);
    }
}

unsafe fn print_stats(istty: c_int, n: c_uint, num: c_uint, elapsed: u64, i_fps: c_double) {
    let mut buf: [c_char; 80] = [0; 80];
    let mut b: *mut c_char = buf.as_mut_ptr();
    let end: *mut c_char = buf.as_mut_ptr().offset(80);
    if istty != 0 {
        let fresh0 = b;
        b = b.offset(1);
        *fresh0 = '\r' as i32 as c_char;
    }
    if num == 0xffffffff as c_uint {
        b = b.offset(snprintf(
            b,
            end.offset_from(b) as usize,
            b"Decoded %u frames\0" as *const u8 as *const c_char,
            n,
        ) as isize);
    } else {
        b = b.offset(snprintf(
            b,
            end.offset_from(b) as usize,
            b"Decoded %u/%u frames (%.1lf%%)\0" as *const u8 as *const c_char,
            n,
            num,
            100.0f64 * n as c_double / num as c_double,
        ) as isize);
    }
    if b < end {
        let d_fps: c_double = 1e9f64 * n as c_double / elapsed as c_double;
        if i_fps != 0. {
            let speed: c_double = d_fps / i_fps;
            b = b.offset(snprintf(
                b,
                end.offset_from(b) as usize,
                b" - %.2lf/%.2lf fps (%.2lfx)\0" as *const u8 as *const c_char,
                d_fps,
                i_fps,
                speed,
            ) as isize);
        } else {
            b = b.offset(snprintf(
                b,
                end.offset_from(b) as usize,
                b" - %.2lf fps\0" as *const u8 as *const c_char,
                d_fps,
            ) as isize);
        }
    }
    if istty == 0 {
        strcpy(
            if b > end.offset(-(2 as c_int as isize)) {
                end.offset(-(2 as c_int as isize))
            } else {
                b
            },
            b"\n\0" as *const u8 as *const c_char,
        );
    }
    fputs(buf.as_mut_ptr(), stderr());
}

unsafe extern "C" fn picture_alloc(
    p: *mut Dav1dPicture,
    _: Option<SendSyncNonNull<c_void>>,
) -> Dav1dResult {
    let hbd = ((*p).p.bpc > 8) as c_int;
    let aligned_w = (*p).p.w + 127 & !(127 as c_int);
    let aligned_h = (*p).p.h + 127 & !(127 as c_int);
    let has_chroma =
        ((*p).p.layout as c_uint != DAV1D_PIXEL_LAYOUT_I400 as c_int as c_uint) as c_int;
    let ss_ver = ((*p).p.layout as c_uint == DAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint) as c_int;
    let ss_hor = ((*p).p.layout as c_uint != DAV1D_PIXEL_LAYOUT_I444 as c_int as c_uint) as c_int;
    let mut y_stride: ptrdiff_t = (aligned_w << hbd) as ptrdiff_t;
    let mut uv_stride: ptrdiff_t = if has_chroma != 0 {
        y_stride >> ss_hor
    } else {
        0
    };
    if y_stride & 1023 == 0 {
        y_stride += DAV1D_PICTURE_ALIGNMENT as isize;
    }
    if uv_stride & 1023 == 0 && has_chroma != 0 {
        uv_stride += DAV1D_PICTURE_ALIGNMENT as isize;
    }
    (*p).stride[0] = -y_stride;
    (*p).stride[1] = -uv_stride;
    let y_sz: usize = (y_stride * aligned_h as isize) as usize;
    let uv_sz: usize = (uv_stride * (aligned_h >> ss_ver) as isize) as usize;
    let pic_size: usize = y_sz.wrapping_add(2 * uv_sz);
    // Change for new `rav1d` safety requirement to initialize picture data.
    // `calloc` of a large size should be optimized to OS zero pages,
    // removing the overhead, and guaranteeing initialization safety.
    let buf: *mut u8 = calloc(pic_size.wrapping_add(DAV1D_PICTURE_ALIGNMENT), 1) as *mut u8;
    if buf.is_null() {
        return Dav1dResult(-12);
    }
    (*p).allocator_data = NonNull::new(buf).map(|ptr| SendSyncNonNull::new_unchecked(ptr).cast());
    let align_m1: ptrdiff_t = (DAV1D_PICTURE_ALIGNMENT - 1) as ptrdiff_t;
    let data: *mut u8 = (buf as ptrdiff_t + align_m1 & !align_m1) as *mut u8;
    (*p).data[0] =
        NonNull::new(data.offset(y_sz as isize).offset(-(y_stride as isize)) as *mut c_void);
    (*p).data[1] = NonNull::new(
        (if has_chroma != 0 {
            data.offset(y_sz as isize)
                .offset(uv_sz.wrapping_mul(1) as isize)
                .offset(-(uv_stride as isize))
        } else {
            0 as *mut u8
        }) as *mut c_void,
    );
    (*p).data[2] = NonNull::new(
        (if has_chroma != 0 {
            data.offset(y_sz as isize)
                .offset(uv_sz.wrapping_mul(2) as isize)
                .offset(-(uv_stride as isize))
        } else {
            0 as *mut u8
        }) as *mut c_void,
    );
    Dav1dResult(0)
}

unsafe extern "C" fn picture_release(p: *mut Dav1dPicture, _: Option<SendSyncNonNull<c_void>>) {
    if let Some(data) = (*p).allocator_data {
        free(data.as_ptr().as_ptr());
    }
}

unsafe fn main_0(argc: c_int, argv: *const *mut c_char) -> c_int {
    let istty = isatty(fileno(stderr()));
    let mut res;
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
    let mut out: *mut MuxerContext = 0 as *mut MuxerContext;
    let mut p = Default::default();
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
    let mut n_out: c_uint = 0 as c_int as c_uint;
    let mut total: c_uint = 0;
    let mut fps: [c_uint; 2] = [0; 2];
    let mut timebase: [c_uint; 2] = [0; 2];
    let nspf: u64;
    let tfirst: u64;
    let mut elapsed: u64 = 0;
    let i_fps: c_double;
    let mut frametimes: *mut libc::FILE = 0 as *mut libc::FILE;
    let [_, major, minor, patch] = dav1d_version_api().to_be_bytes();
    if DAV1D_API_VERSION_MAJOR != major || DAV1D_API_VERSION_MINOR > minor {
        fprintf(
            stderr(),
            b"Version mismatch (library: %d.%d.%d, executable: %d.%d.%d)\n\0" as *const u8
                as *const c_char,
            major as c_int,
            minor as c_int,
            patch as c_int,
            DAV1D_API_VERSION_MAJOR as c_int,
            DAV1D_API_VERSION_MINOR as c_int,
            DAV1D_API_VERSION_PATCH as c_int,
        );
        return 1 as c_int;
    }
    parse(argc, argv, &mut cli_settings, &mut lib_settings);
    if cli_settings.neg_stride != 0 {
        lib_settings.allocator.alloc_picture_callback = Some(picture_alloc);
        lib_settings.allocator.release_picture_callback = Some(picture_release);
    }
    res = input_open(
        &mut in_0,
        cli_settings.demuxer,
        cli_settings.inputfile,
        fps.as_mut_ptr(),
        &mut total,
        timebase.as_mut_ptr(),
    );
    if res < 0 {
        return 1 as c_int;
    }
    let mut i: c_uint = 0 as c_int as c_uint;
    while i <= cli_settings.skip {
        res = input_read(in_0, &mut data);
        if res < 0 {
            input_close(in_0);
            return 1 as c_int;
        }
        if i < cli_settings.skip {
            dav1d_data_unref(NonNull::new(&mut data));
        }
        i = i.wrapping_add(1);
    }
    if cli_settings.quiet == 0 {
        fprintf(
            stderr(),
            b"dav1d %s - by VideoLAN\n\0" as *const u8 as *const c_char,
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
        let mut seq_skip: c_uint = 0 as c_int as c_uint;
        while dav1d_parse_sequence_header(NonNull::new(&mut seq), data.data, data.sz).0 != 0 {
            res = input_read(in_0, &mut data);
            if res < 0 {
                input_close(in_0);
                return 1 as c_int;
            }
            seq_skip = seq_skip.wrapping_add(1);
        }
        if seq_skip != 0 && cli_settings.quiet == 0 {
            fprintf(
                stderr(),
                b"skipped %u packets due to missing sequence header\n\0" as *const u8
                    as *const c_char,
                seq_skip,
            );
        }
    }
    if cli_settings.limit != 0 as c_int as c_uint && cli_settings.limit < total {
        total = cli_settings.limit;
    }
    res = dav1d_open(NonNull::new(&mut c), NonNull::new(&mut lib_settings)).0;
    if res != 0 {
        return 1 as c_int;
    }
    if !(cli_settings.frametimes).is_null() {
        frametimes = fopen(
            cli_settings.frametimes,
            b"w\0" as *const u8 as *const c_char,
        );
    }
    if cli_settings.realtime as c_uint != REALTIME_CUSTOM as c_int as c_uint {
        if fps[1] == 0 as c_uint {
            i_fps = 0 as c_int as c_double;
            nspf = 0 as c_int as u64;
        } else {
            i_fps = fps[0] as c_double / fps[1] as c_double;
            nspf = (1000000000 as c_ulonglong)
                .wrapping_mul(fps[1] as c_ulonglong)
                .wrapping_div(fps[0] as c_ulonglong) as u64;
        }
    } else {
        i_fps = cli_settings.realtime_fps;
        nspf = (1000000000.0f64 / cli_settings.realtime_fps) as u64;
    }
    tfirst = get_time_nanos();
    loop {
        memset(
            &mut p as *mut Dav1dPicture as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<Dav1dPicture>(),
        );
        res = dav1d_send_data(c, NonNull::new(&mut data)).0;
        if res < 0 {
            if res != -EAGAIN {
                dav1d_data_unref(NonNull::new(&mut data));
                fprintf(
                    stderr(),
                    b"Error decoding frame: %s\n\0" as *const u8 as *const c_char,
                    strerror(-res),
                );
                if res != -EINVAL {
                    break;
                }
            }
        }
        res = dav1d_get_picture(c, NonNull::new(&mut p)).0;
        if res < 0 {
            if res != -EAGAIN {
                fprintf(
                    stderr(),
                    b"Error decoding frame: %s\n\0" as *const u8 as *const c_char,
                    strerror(-res),
                );
                if res != -EINVAL {
                    break;
                }
            }
            res = 0 as c_int;
        } else {
            if n_out == 0 {
                res = output_open(
                    &mut out,
                    cli_settings.muxer,
                    cli_settings.outputfile,
                    &mut p.p,
                    fps.as_mut_ptr() as *const c_uint,
                );
                if res < 0 {
                    if !frametimes.is_null() {
                        fclose(frametimes);
                    }
                    return 1 as c_int;
                }
            }
            res = output_write(out, &mut p);
            if res < 0 {
                break;
            }
            n_out = n_out.wrapping_add(1);
            if nspf != 0 || cli_settings.quiet == 0 {
                synchronize(
                    cli_settings.realtime as c_int,
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
        if !(data.sz > 0 || input_read(in_0, &mut data) == 0) {
            break;
        }
    }
    if data.sz > 0 {
        dav1d_data_unref(NonNull::new(&mut data));
    }
    if res == 0 {
        while cli_settings.limit == 0 || n_out < cli_settings.limit {
            res = dav1d_get_picture(c, NonNull::new(&mut p)).0;
            if res < 0 {
                if res != -EAGAIN {
                    fprintf(
                        stderr(),
                        b"Error decoding frame: %s\n\0" as *const u8 as *const c_char,
                        strerror(-res),
                    );
                    if res != -EINVAL {
                        break;
                    }
                } else {
                    res = 0 as c_int;
                    break;
                }
            } else {
                if n_out == 0 {
                    res = output_open(
                        &mut out,
                        cli_settings.muxer,
                        cli_settings.outputfile,
                        &mut p.p,
                        fps.as_mut_ptr() as *const c_uint,
                    );
                    if res < 0 {
                        if !frametimes.is_null() {
                            fclose(frametimes);
                        }
                        return 1 as c_int;
                    }
                }
                res = output_write(out, &mut p);
                if res < 0 {
                    break;
                }
                n_out = n_out.wrapping_add(1);
                if nspf != 0 || cli_settings.quiet == 0 {
                    synchronize(
                        cli_settings.realtime as c_int,
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
            fprintf(stderr(), b"\n\0" as *const u8 as *const c_char);
        }
        if !(cli_settings.verify).is_null() {
            res |= output_verify(out, cli_settings.verify);
        } else {
            output_close(out);
        }
    } else {
        fprintf(
            stderr(),
            b"No data decoded\n\0" as *const u8 as *const c_char,
        );
        res = 1 as c_int;
    }
    dav1d_close(NonNull::new(&mut c));
    return if res == 0 { 0 as c_int } else { 1 as c_int };
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
