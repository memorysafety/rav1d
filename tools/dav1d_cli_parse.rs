use crate::compat::getopt::getopt_long;
use crate::compat::getopt::option;
use crate::compat::stdio::stderr;
use libc::fprintf;
use libc::memset;
use libc::strcat;
use libc::strcmp;
use libc::strcpy;
use libc::strncmp;
use libc::strtod;
use libc::strtoul;
use rav1d::cpu::dav1d_set_cpu_flags_mask;
#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "riscv32",
    target_arch = "riscv64"
))]
use rav1d::cpu::CpuFlags;
use rav1d::dav1d_default_settings;
use rav1d::dav1d_version;
use rav1d::include::dav1d::dav1d::Dav1dDecodeFrameType;
use rav1d::include::dav1d::dav1d::Dav1dInloopFilterType;
use rav1d::include::dav1d::dav1d::Dav1dSettings;
use rav1d::include::dav1d::dav1d::DAV1D_DECODEFRAMETYPE_ALL;
use rav1d::include::dav1d::dav1d::DAV1D_DECODEFRAMETYPE_INTRA;
use rav1d::include::dav1d::dav1d::DAV1D_DECODEFRAMETYPE_KEY;
use rav1d::include::dav1d::dav1d::DAV1D_DECODEFRAMETYPE_REFERENCE;
use rav1d::include::dav1d::dav1d::DAV1D_INLOOPFILTER_ALL;
use rav1d::include::dav1d::dav1d::DAV1D_INLOOPFILTER_CDEF;
use rav1d::include::dav1d::dav1d::DAV1D_INLOOPFILTER_DEBLOCK;
use rav1d::include::dav1d::dav1d::DAV1D_INLOOPFILTER_NONE;
use rav1d::include::dav1d::dav1d::DAV1D_INLOOPFILTER_RESTORATION;
use std::ffi::c_char;
use std::ffi::c_double;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_ulong;
use std::ffi::c_void;
use std::ffi::CStr;
use std::process::exit;
use std::ptr::NonNull;

use cfg_if::cfg_if;

extern "C" {
    static mut optarg: *mut c_char;
    static mut optind: c_int;
}

// TODO(kkysen) These are used in `dav1d.rs` and `seek_stress.rs`
// but are still marked as unused since `[[bin]]` are only supposed to be one file in `cargo`.
pub type CLISettingsRealTime = c_uint;
#[allow(dead_code)]
pub const REALTIME_CUSTOM: CLISettingsRealTime = 2;
#[allow(dead_code)]
pub const REALTIME_INPUT: CLISettingsRealTime = 1;
pub const REALTIME_DISABLE: CLISettingsRealTime = 0;

#[repr(C)]
pub struct CLISettings {
    pub outputfile: *const c_char,
    pub inputfile: *const c_char,
    pub demuxer: *const c_char,
    pub muxer: *const c_char,
    pub frametimes: *const c_char,
    pub verify: *const c_char,
    pub limit: c_uint,
    pub skip: c_uint,
    pub quiet: c_int,
    pub realtime: CLISettingsRealTime,
    pub realtime_fps: c_double,
    pub realtime_cache: c_uint,
    pub neg_stride: c_int,
}

#[repr(C)]
pub struct EnumParseTable {
    pub str_0: *const c_char,
    pub val: c_int,
}

pub const ARG_DECODE_FRAME_TYPE: Arg = 273;
pub const ARG_INLOOP_FILTERS: Arg = 272;
pub const ARG_OUTPUT_INVISIBLE: Arg = 271;
pub const ARG_NEG_STRIDE: Arg = 270;
pub const ARG_CPU_MASK: Arg = 269;
pub const ARG_STRICT_STD_COMPLIANCE: Arg = 268;
pub const ARG_SIZE_LIMIT: Arg = 267;
pub const ARG_ALL_LAYERS: Arg = 266;
pub const ARG_OPPOINT: Arg = 265;
pub const ARG_FILM_GRAIN: Arg = 264;
pub const ARG_VERIFY: Arg = 263;
pub const ARG_FRAME_DELAY: Arg = 262;
pub const ARG_THREADS: Arg = 261;
pub const ARG_REALTIME_CACHE: Arg = 260;
pub const ARG_REALTIME: Arg = 259;
pub const ARG_FRAME_TIMES: Arg = 258;
pub const ARG_MUXER: Arg = 257;
pub const ARG_DEMUXER: Arg = 256;
cfg_if! {
    if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        pub const X86_CPU_MASK_AVX512ICL: CpuMask = 31;
        pub const X86_CPU_MASK_AVX2: CpuMask = 15;
        pub const X86_CPU_MASK_SSE41: CpuMask = 7;
        pub const X86_CPU_MASK_SSSE3: CpuMask = 3;
        pub const X86_CPU_MASK_SSE2: CpuMask = 1;
        pub type CpuMask = c_uint;

        const ALLOWED_CPU_MASKS: &str = ", 'sse2', 'ssse3', 'sse41', 'avx2' or 'avx512icl'";
    } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
        const ALLOWED_CPU_MASKS: &str = " or 'neon'";
    } else if #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))] {
        const ALLOWED_CPU_MASKS: &str = " or 'rvv'";
    } else {
        const ALLOWED_CPU_MASKS: &str = "not yet implemented for this architecture";
    }
}
pub type Arg = c_uint;

static short_opts: [c_char; 11] =
    unsafe { *::core::mem::transmute::<&[u8; 11], &[c_char; 11]>(b"i:o:vql:s:\0") };

static mut long_opts: [option; 25] = [
    {
        option {
            name: b"input\0" as *const u8 as *const c_char,
            has_arg: 1 as c_int,
            flag: 0 as *const c_int as *mut c_int,
            val: 'i' as i32,
        }
    },
    {
        option {
            name: b"output\0" as *const u8 as *const c_char,
            has_arg: 1 as c_int,
            flag: 0 as *const c_int as *mut c_int,
            val: 'o' as i32,
        }
    },
    {
        option {
            name: b"quiet\0" as *const u8 as *const c_char,
            has_arg: 0 as c_int,
            flag: 0 as *const c_int as *mut c_int,
            val: 'q' as i32,
        }
    },
    {
        option {
            name: b"demuxer\0" as *const u8 as *const c_char,
            has_arg: 1 as c_int,
            flag: 0 as *const c_int as *mut c_int,
            val: ARG_DEMUXER as c_int,
        }
    },
    {
        option {
            name: b"muxer\0" as *const u8 as *const c_char,
            has_arg: 1 as c_int,
            flag: 0 as *const c_int as *mut c_int,
            val: ARG_MUXER as c_int,
        }
    },
    {
        option {
            name: b"version\0" as *const u8 as *const c_char,
            has_arg: 0 as c_int,
            flag: 0 as *const c_int as *mut c_int,
            val: 'v' as i32,
        }
    },
    {
        option {
            name: b"frametimes\0" as *const u8 as *const c_char,
            has_arg: 1 as c_int,
            flag: 0 as *const c_int as *mut c_int,
            val: ARG_FRAME_TIMES as c_int,
        }
    },
    {
        option {
            name: b"limit\0" as *const u8 as *const c_char,
            has_arg: 1 as c_int,
            flag: 0 as *const c_int as *mut c_int,
            val: 'l' as i32,
        }
    },
    {
        option {
            name: b"skip\0" as *const u8 as *const c_char,
            has_arg: 1 as c_int,
            flag: 0 as *const c_int as *mut c_int,
            val: 's' as i32,
        }
    },
    {
        option {
            name: b"realtime\0" as *const u8 as *const c_char,
            has_arg: 2 as c_int,
            flag: 0 as *const c_int as *mut c_int,
            val: ARG_REALTIME as c_int,
        }
    },
    {
        option {
            name: b"realtimecache\0" as *const u8 as *const c_char,
            has_arg: 1 as c_int,
            flag: 0 as *const c_int as *mut c_int,
            val: ARG_REALTIME_CACHE as c_int,
        }
    },
    {
        option {
            name: b"threads\0" as *const u8 as *const c_char,
            has_arg: 1 as c_int,
            flag: 0 as *const c_int as *mut c_int,
            val: ARG_THREADS as c_int,
        }
    },
    {
        option {
            name: b"framedelay\0" as *const u8 as *const c_char,
            has_arg: 1 as c_int,
            flag: 0 as *const c_int as *mut c_int,
            val: ARG_FRAME_DELAY as c_int,
        }
    },
    {
        option {
            name: b"verify\0" as *const u8 as *const c_char,
            has_arg: 1 as c_int,
            flag: 0 as *const c_int as *mut c_int,
            val: ARG_VERIFY as c_int,
        }
    },
    {
        option {
            name: b"filmgrain\0" as *const u8 as *const c_char,
            has_arg: 1 as c_int,
            flag: 0 as *const c_int as *mut c_int,
            val: ARG_FILM_GRAIN as c_int,
        }
    },
    {
        option {
            name: b"oppoint\0" as *const u8 as *const c_char,
            has_arg: 1 as c_int,
            flag: 0 as *const c_int as *mut c_int,
            val: ARG_OPPOINT as c_int,
        }
    },
    {
        option {
            name: b"alllayers\0" as *const u8 as *const c_char,
            has_arg: 1 as c_int,
            flag: 0 as *const c_int as *mut c_int,
            val: ARG_ALL_LAYERS as c_int,
        }
    },
    {
        option {
            name: b"sizelimit\0" as *const u8 as *const c_char,
            has_arg: 1 as c_int,
            flag: 0 as *const c_int as *mut c_int,
            val: ARG_SIZE_LIMIT as c_int,
        }
    },
    {
        option {
            name: b"strict\0" as *const u8 as *const c_char,
            has_arg: 1 as c_int,
            flag: 0 as *const c_int as *mut c_int,
            val: ARG_STRICT_STD_COMPLIANCE as c_int,
        }
    },
    {
        option {
            name: b"cpumask\0" as *const u8 as *const c_char,
            has_arg: 1 as c_int,
            flag: 0 as *const c_int as *mut c_int,
            val: ARG_CPU_MASK as c_int,
        }
    },
    {
        option {
            name: b"negstride\0" as *const u8 as *const c_char,
            has_arg: 0 as c_int,
            flag: 0 as *const c_int as *mut c_int,
            val: ARG_NEG_STRIDE as c_int,
        }
    },
    {
        option {
            name: b"outputinvisible\0" as *const u8 as *const c_char,
            has_arg: 1 as c_int,
            flag: 0 as *const c_int as *mut c_int,
            val: ARG_OUTPUT_INVISIBLE as c_int,
        }
    },
    {
        option {
            name: b"inloopfilters\0" as *const u8 as *const c_char,
            has_arg: 1 as c_int,
            flag: 0 as *const c_int as *mut c_int,
            val: ARG_INLOOP_FILTERS as c_int,
        }
    },
    {
        option {
            name: b"decodeframetype\0" as *const u8 as *const c_char,
            has_arg: 1 as c_int,
            flag: 0 as *const c_int as *mut c_int,
            val: ARG_DECODE_FRAME_TYPE as c_int,
        }
    },
    {
        option {
            name: 0 as *const c_char,
            has_arg: 0 as c_int,
            flag: 0 as *const c_int as *mut c_int,
            val: 0 as c_int,
        }
    },
];

fn usage(app: &str, reason: Option<&str>) {
    if let Some(reason) = reason {
        eprintln!("{reason}\n");
    }
    eprintln!("Usage: {app} [options]\n");
    eprintln!("Supported options:
 --input/-i $file:     input file
 --output/-o $file:    output file (%n, %w or %h will be filled in for per-frame files)
 --demuxer $name:      force demuxer type ('ivf', 'section5' or 'annexb'; default: detect from content)
 --muxer $name:        force muxer type ('md5', 'yuv', 'yuv4mpeg2' or 'null'; default: detect from extension)
                       use 'frame' as prefix to write per-frame files; if filename contains %n, will default to writing per-frame files
 --quiet/-q:           disable status messages
 --frametimes $file:   dump frame times to file
 --limit/-l $num:      stop decoding after $num frames
 --skip/-s $num:       skip decoding of the first $num frames
 --realtime [$fract]:  limit framerate, optional argument to override input framerate
 --realtimecache $num: set the size of the cache in realtime mode (default: 0)
 --version/-v:         print version and exit
 --threads $num:       number of threads (default: 0)
 --framedelay $num:    maximum frame delay, capped at $threads (default: 0);
                       set to 1 for low-latency decoding
 --filmgrain $num:     enable film grain application (default: 1, except if muxer is md5 or xxh3)
 --oppoint $num:       select an operating point of a scalable AV1 bitstream (0 - 31)
 --alllayers $num:     output all spatial layers of a scalable AV1 bitstream (default: 1)
 --sizelimit $num:     stop decoding if the frame size exceeds the specified limit
 --strict $num:        whether to abort decoding on standard compliance violations
                       that don't affect bitstream decoding (default: 1)
 --verify $md5:        verify decoded md5. implies --muxer md5, no output
 --cpumask $mask:      restrict permitted CPU instruction sets (0, {}; default: -1)
 --negstride:          use negative picture strides
                       this is mostly meant as a developer option
 --outputinvisible $num: whether to output invisible (alt-ref) frames (default: 0)
 --inloopfilters $str: which in-loop filters to enable (none, (no)deblock, (no)cdef, (no)restoration or all; default: all)
 --decodeframetype $str: which frame types to decode (reference, intra, key or all; default: all)", ALLOWED_CPU_MASKS);
    exit(1);
}

unsafe fn error(
    app: *const c_char,
    optarg_0: *const c_char,
    option: c_int,
    shouldbe: *const c_char,
) {
    let mut n;
    n = 0 as c_int;
    while !(long_opts[n as usize].name).is_null() {
        if long_opts[n as usize].val == option {
            break;
        }
        n += 1;
    }
    if (long_opts[n as usize].name).is_null() {
        unreachable!();
    }
    let long_name = CStr::from_ptr(long_opts[n as usize].name).to_str().unwrap();
    let optname = if long_opts[n as usize].val < 256 {
        format!("-{}/--{}", long_opts[n as usize].val, long_name)
    } else {
        format!("--{}", long_name)
    };
    assert!(!app.is_null());
    let app = CStr::from_ptr(app)
        .to_str()
        .expect("App is not valid UTF-8");
    assert!(!optarg_0.is_null());
    let arg = CStr::from_ptr(optarg_0)
        .to_str()
        .expect("Arg is not valid UTF-8");
    assert!(!shouldbe.is_null());
    let shouldbe = CStr::from_ptr(shouldbe).to_str().unwrap();
    usage(
        app,
        Some(&format!(
            "Invalid argument \"{arg}\" for option {optname}; should be {shouldbe}"
        )),
    );
}

unsafe fn parse_unsigned(optarg_0: *const c_char, option: c_int, app: *const c_char) -> c_uint {
    let mut end: *mut c_char = 0 as *mut c_char;
    let res: c_uint = strtoul(optarg_0, &mut end, 0 as c_int) as c_uint;
    if *end as c_int != 0 || end == optarg_0 as *mut c_char {
        error(
            app,
            optarg_0,
            option,
            b"an integer\0" as *const u8 as *const c_char,
        );
    }
    return res;
}

unsafe fn parse_optional_fraction(
    optarg_0: *const c_char,
    option: c_int,
    app: *const c_char,
    value: *mut c_double,
) -> c_int {
    if optarg_0.is_null() {
        return 0 as c_int;
    }
    let mut end: *mut c_char = 0 as *mut c_char;
    *value = strtod(optarg_0, &mut end);
    if *end as c_int == '/' as i32 && end != optarg_0 as *mut c_char {
        let optarg2: *const c_char = end.offset(1);
        *value /= strtod(optarg2, &mut end);
        if *end as c_int != 0 || end == optarg2 as *mut c_char {
            error(
                app,
                optarg_0,
                option,
                b"a fraction\0" as *const u8 as *const c_char,
            );
        }
    } else if *end as c_int != 0 || end == optarg_0 as *mut c_char {
        error(
            app,
            optarg_0,
            option,
            b"a fraction\0" as *const u8 as *const c_char,
        );
    }
    return 1 as c_int;
}

// TODO: add other architectures supported by dav1d
cfg_if! {
    if #[cfg(any(target_arch = "arm"))] {
        static mut cpu_mask_tbl: [EnumParseTable; 3] = [
            {
                EnumParseTable {
                    str_0: b"neon\0" as *const u8 as *const c_char,
                    val: CpuFlags::NEON.bits() as c_int,
                }
            },
            {
                EnumParseTable {
                    str_0: b"dotprod\0" as *const u8 as *const c_char,
                    val: CpuFlags::DOTPROD.bits() as c_int,
                }
            },
            {
                EnumParseTable {
                    str_0: b"i8mm\0" as *const u8 as *const c_char,
                    val: CpuFlags::I8MM.bits() as c_int,
                }
            },
        ];
    } else if #[cfg(any(target_arch = "aarch64"))] {
        static mut cpu_mask_tbl: [EnumParseTable; 5] = [
            {
                EnumParseTable {
                    str_0: b"neon\0" as *const u8 as *const c_char,
                    val: CpuFlags::NEON.bits() as c_int,
                }
            },
            {
                EnumParseTable {
                    str_0: b"dotprod\0" as *const u8 as *const c_char,
                    val: CpuFlags::DOTPROD.bits() as c_int,
                }
            },
            {
                EnumParseTable {
                    str_0: b"i8mm\0" as *const u8 as *const c_char,
                    val: CpuFlags::I8MM.bits() as c_int,
                }
            },
            {
                EnumParseTable {
                    str_0: b"sve\0" as *const u8 as *const c_char,
                    val: CpuFlags::SVE.bits() as c_int,
                }
            },
            {
                EnumParseTable {
                    str_0: b"sve2\0" as *const u8 as *const c_char,
                    val: CpuFlags::SVE2.bits() as c_int,
                }
            },
        ];
    } else if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        static mut cpu_mask_tbl: [EnumParseTable; 6] = [
            {
                EnumParseTable {
                    str_0: b"sse2\0" as *const u8 as *const c_char,
                    val: X86_CPU_MASK_SSE2 as c_int,
                }
            },
            {
                EnumParseTable {
                    str_0: b"ssse3\0" as *const u8 as *const c_char,
                    val: X86_CPU_MASK_SSSE3 as c_int,
                }
            },
            {
                EnumParseTable {
                    str_0: b"sse41\0" as *const u8 as *const c_char,
                    val: X86_CPU_MASK_SSE41 as c_int,
                }
            },
            {
                EnumParseTable {
                    str_0: b"avx2\0" as *const u8 as *const c_char,
                    val: X86_CPU_MASK_AVX2 as c_int,
                }
            },
            {
                EnumParseTable {
                    str_0: b"avx512icl\0" as *const u8 as *const c_char,
                    val: X86_CPU_MASK_AVX512ICL as c_int,
                }
            },
            {
                EnumParseTable {
                    str_0: b"none\0" as *const u8 as *const c_char,
                    val: 0 as c_int,
                }
            },
        ];
    } else if #[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))] {
        static mut cpu_mask_tbl: [EnumParseTable; 1] = [
            {
                EnumParseTable {
                    str_0: b"rvv\0" as *const u8 as *const c_char,
                    val: CpuFlags::V.bits() as c_int,
                }
            },
        ];
    } else {
        static mut cpu_mask_tbl: [EnumParseTable; 0] = [];
    }
}

static mut inloop_filters_tbl: [EnumParseTable; 8] = [
    {
        EnumParseTable {
            str_0: b"none\0" as *const u8 as *const c_char,
            val: DAV1D_INLOOPFILTER_NONE as c_int,
        }
    },
    {
        EnumParseTable {
            str_0: b"deblock\0" as *const u8 as *const c_char,
            val: DAV1D_INLOOPFILTER_DEBLOCK as c_int,
        }
    },
    {
        EnumParseTable {
            str_0: b"nodeblock\0" as *const u8 as *const c_char,
            val: DAV1D_INLOOPFILTER_ALL as c_int - DAV1D_INLOOPFILTER_DEBLOCK as c_int,
        }
    },
    {
        EnumParseTable {
            str_0: b"cdef\0" as *const u8 as *const c_char,
            val: DAV1D_INLOOPFILTER_CDEF as c_int,
        }
    },
    {
        EnumParseTable {
            str_0: b"nocdef\0" as *const u8 as *const c_char,
            val: DAV1D_INLOOPFILTER_ALL as c_int - DAV1D_INLOOPFILTER_CDEF as c_int,
        }
    },
    {
        EnumParseTable {
            str_0: b"restoration\0" as *const u8 as *const c_char,
            val: DAV1D_INLOOPFILTER_RESTORATION as c_int,
        }
    },
    {
        EnumParseTable {
            str_0: b"norestoration\0" as *const u8 as *const c_char,
            val: DAV1D_INLOOPFILTER_ALL as c_int - DAV1D_INLOOPFILTER_RESTORATION as c_int,
        }
    },
    {
        EnumParseTable {
            str_0: b"all\0" as *const u8 as *const c_char,
            val: DAV1D_INLOOPFILTER_ALL as c_int,
        }
    },
];

static mut decode_frame_type_tbl: [EnumParseTable; 4] = [
    {
        EnumParseTable {
            str_0: b"all\0" as *const u8 as *const c_char,
            val: DAV1D_DECODEFRAMETYPE_ALL as c_int,
        }
    },
    {
        EnumParseTable {
            str_0: b"reference\0" as *const u8 as *const c_char,
            val: DAV1D_DECODEFRAMETYPE_REFERENCE as c_int,
        }
    },
    {
        EnumParseTable {
            str_0: b"intra\0" as *const u8 as *const c_char,
            val: DAV1D_DECODEFRAMETYPE_INTRA as c_int,
        }
    },
    {
        EnumParseTable {
            str_0: b"key\0" as *const u8 as *const c_char,
            val: DAV1D_DECODEFRAMETYPE_KEY as c_int,
        }
    },
];

unsafe fn parse_enum(
    optarg_0: *mut c_char,
    tbl: *const EnumParseTable,
    tbl_sz: c_int,
    option: c_int,
    app: *const c_char,
) -> c_uint {
    let mut str: [c_char; 1024] = [0; 1024];
    strcpy(str.as_mut_ptr(), b"any of \0" as *const u8 as *const c_char);
    let mut n = 0;
    while n < tbl_sz {
        if strcmp((*tbl.offset(n as isize)).str_0, optarg_0) == 0 {
            return (*tbl.offset(n as isize)).val as c_uint;
        }
        if n != 0 {
            if n < tbl_sz - 1 {
                strcat(str.as_mut_ptr(), b", \0" as *const u8 as *const c_char);
            } else {
                strcat(str.as_mut_ptr(), b" or \0" as *const u8 as *const c_char);
            }
        }
        strcat(str.as_mut_ptr(), (*tbl.offset(n as isize)).str_0);
        n += 1;
    }
    let mut end: *mut c_char = 0 as *mut c_char;
    let res: c_uint;
    if strncmp(optarg_0, b"0x\0" as *const u8 as *const c_char, 2) == 0 {
        res = strtoul(&mut *optarg_0.offset(2), &mut end, 16 as c_int) as c_uint;
    } else {
        res = strtoul(optarg_0, &mut end, 0 as c_int) as c_uint;
    }
    if *end as c_int != 0 || end == optarg_0 {
        strcat(
            str.as_mut_ptr(),
            b", a hexadecimal (starting with 0x), or an integer\0" as *const u8 as *const c_char,
        );
        error(app, optarg_0, option, str.as_mut_ptr());
    }
    return res;
}

#[allow(static_mut_refs)]
pub unsafe fn parse(
    argc: c_int,
    argv: *const *mut c_char,
    cli_settings: *mut CLISettings,
    lib_settings: *mut Dav1dSettings,
) {
    let app = *argv.offset(0);
    assert!(!app.is_null(), "argv[0] should never be null");
    let app = CStr::from_ptr(app).to_str().unwrap();

    let mut o;
    memset(
        cli_settings as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<CLISettings>(),
    );
    dav1d_default_settings(NonNull::new(lib_settings).unwrap());
    (*lib_settings).strict_std_compliance = 1 as c_int;
    let mut grain_specified = 0;
    loop {
        o = getopt_long(
            argc,
            argv,
            short_opts.as_ptr(),
            long_opts.as_ptr(),
            0 as *mut c_int,
        );
        if !(o != -1) {
            break;
        }
        match o {
            111 => {
                (*cli_settings).outputfile = optarg;
            }
            105 => {
                (*cli_settings).inputfile = optarg;
            }
            113 => {
                (*cli_settings).quiet = 1 as c_int;
            }
            108 => {
                (*cli_settings).limit = parse_unsigned(optarg, 'l' as i32, *argv.offset(0));
            }
            115 => {
                (*cli_settings).skip = parse_unsigned(optarg, 's' as i32, *argv.offset(0));
            }
            256 => {
                (*cli_settings).demuxer = optarg;
            }
            257 => {
                (*cli_settings).muxer = optarg;
            }
            258 => {
                (*cli_settings).frametimes = optarg;
            }
            259 => {
                if optarg.is_null()
                    && optind < argc
                    && !(*argv.offset(optind as isize)).is_null()
                    && *(*argv.offset(optind as isize)).offset(0) as c_int != '-' as i32
                {
                    optarg = *argv.offset(optind as isize);
                    optind += 1;
                }
                (*cli_settings).realtime = (1 as c_int
                    + parse_optional_fraction(
                        optarg,
                        ARG_REALTIME as c_int,
                        *argv.offset(0),
                        &mut (*cli_settings).realtime_fps,
                    )) as CLISettingsRealTime;
            }
            260 => {
                (*cli_settings).realtime_cache =
                    parse_unsigned(optarg, ARG_REALTIME_CACHE as c_int, *argv.offset(0));
            }
            262 => {
                (*lib_settings).max_frame_delay =
                    parse_unsigned(optarg, ARG_FRAME_DELAY as c_int, *argv.offset(0)) as c_int;
            }
            261 => {
                (*lib_settings).n_threads =
                    parse_unsigned(optarg, ARG_THREADS as c_int, *argv.offset(0)) as c_int;
            }
            263 => {
                (*cli_settings).verify = optarg;
            }
            264 => {
                (*lib_settings).apply_grain =
                    (parse_unsigned(optarg, ARG_FILM_GRAIN as c_int, *argv.offset(0)) != 0)
                        as c_int;
                grain_specified = 1 as c_int;
            }
            265 => {
                (*lib_settings).operating_point =
                    parse_unsigned(optarg, ARG_OPPOINT as c_int, *argv.offset(0)) as c_int;
            }
            266 => {
                (*lib_settings).all_layers =
                    (parse_unsigned(optarg, ARG_ALL_LAYERS as c_int, *argv.offset(0)) != 0)
                        as c_int;
            }
            267 => {
                let mut arg: *mut c_char = optarg;
                let mut end: *mut c_char = 0 as *mut c_char;
                let mut res: u64 = strtoul(arg, &mut end, 0) as u64;
                if *end as c_int == 'x' as i32 {
                    arg = end.offset(1);
                    res = (res as c_ulong).wrapping_mul(strtoul(arg, &mut end, 0)) as u64 as u64;
                }
                if *end as c_int != 0 || end == arg || res >= u32::MAX as u64 {
                    error(
                        *argv.offset(0),
                        optarg,
                        ARG_SIZE_LIMIT as c_int,
                        b"an integer or dimension\0" as *const u8 as *const c_char,
                    );
                }
                (*lib_settings).frame_size_limit = res as c_uint;
            }
            268 => {
                (*lib_settings).strict_std_compliance =
                    parse_unsigned(optarg, ARG_STRICT_STD_COMPLIANCE as c_int, *argv.offset(0))
                        as c_int;
            }
            118 => {
                fprintf(
                    stderr(),
                    b"%s\n\0" as *const u8 as *const c_char,
                    dav1d_version(),
                );
                exit(0 as c_int);
            }
            269 => {
                dav1d_set_cpu_flags_mask(parse_enum(
                    optarg,
                    cpu_mask_tbl.as_ptr(),
                    (::core::mem::size_of::<[EnumParseTable; 6]>() as c_ulong)
                        .wrapping_div(::core::mem::size_of::<EnumParseTable>() as c_ulong)
                        as c_int,
                    ARG_CPU_MASK as c_int,
                    *argv.offset(0),
                ));
            }
            270 => {
                (*cli_settings).neg_stride = 1 as c_int;
            }
            271 => {
                (*lib_settings).output_invisible_frames =
                    (parse_unsigned(optarg, ARG_OUTPUT_INVISIBLE as c_int, *argv.offset(0)) != 0)
                        as c_int;
            }
            272 => {
                (*lib_settings).inloop_filters = parse_enum(
                    optarg,
                    inloop_filters_tbl.as_ptr(),
                    (::core::mem::size_of::<[EnumParseTable; 8]>() as c_ulong)
                        .wrapping_div(::core::mem::size_of::<EnumParseTable>() as c_ulong)
                        as c_int,
                    ARG_INLOOP_FILTERS as c_int,
                    *argv.offset(0),
                ) as Dav1dInloopFilterType;
            }
            273 => {
                (*lib_settings).decode_frame_type = parse_enum(
                    optarg,
                    decode_frame_type_tbl.as_ptr(),
                    (::core::mem::size_of::<[EnumParseTable; 4]>() as c_ulong)
                        .wrapping_div(::core::mem::size_of::<EnumParseTable>() as c_ulong)
                        as c_int,
                    ARG_DECODE_FRAME_TYPE as c_int,
                    *argv.offset(0),
                ) as Dav1dDecodeFrameType;
            }
            _ => {
                usage(app, None);
            }
        }
    }
    if optind < argc {
        let optind_arg = *argv.offset(optind as isize);
        assert!(
            !optind_arg.is_null(),
            "argv[optind] should never be null when optind < argc"
        );
        usage(
            app,
            Some(&format!(
                "Extra/unused arguments found, e.g. '{}'",
                CStr::from_ptr(optind_arg)
                    .to_str()
                    .unwrap_or_else(|_| panic!("argv[{}] is not valid UTF-8", optind)),
            )),
        );
    }
    if !((*cli_settings).verify).is_null() {
        if !((*cli_settings).outputfile).is_null() {
            usage(
                app,
                Some(&"Verification (--verify) requires output file (-o/--output) to not set"),
            );
        }
        if !((*cli_settings).muxer).is_null()
            && strcmp(
                (*cli_settings).muxer,
                b"md5\0" as *const u8 as *const c_char,
            ) != 0
            && strcmp(
                (*cli_settings).muxer,
                b"xxh3\0" as *const u8 as *const c_char,
            ) != 0
        {
            usage(
                app,
                Some(&"Verification (--verify) requires a checksum muxer (md5 or xxh3)"),
            );
        }
        (*cli_settings).outputfile = b"-\0" as *const u8 as *const c_char;
        if ((*cli_settings).muxer).is_null() {
            (*cli_settings).muxer = b"md5\0" as *const u8 as *const c_char;
        }
    }
    if grain_specified == 0
        && !((*cli_settings).muxer).is_null()
        && (strcmp(
            (*cli_settings).muxer,
            b"md5\0" as *const u8 as *const c_char,
        ) == 0
            || strcmp(
                (*cli_settings).muxer,
                b"xxh3\0" as *const u8 as *const c_char,
            ) == 0)
    {
        (*lib_settings).apply_grain = 0 as c_int;
    }
    if ((*cli_settings).inputfile).is_null() {
        usage(app, Some(&"Input file (-i/--input) is required"));
    }
    if (((*cli_settings).muxer).is_null()
        || strcmp(
            (*cli_settings).muxer,
            b"null\0" as *const u8 as *const c_char,
        ) != 0)
        && ((*cli_settings).outputfile).is_null()
    {
        usage(app, Some(&"Output file (-o/--output) is required"));
    }
}
