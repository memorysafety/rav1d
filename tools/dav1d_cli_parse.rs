use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
use crate::stderr;
extern "C" {
    pub type Dav1dRef;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn fprintf(_: *mut libc::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut libc::FILE,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn exit(_: libc::c_int) -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn dav1d_version() -> *const libc::c_char;
    fn dav1d_default_settings(s: *mut Dav1dSettings);
    fn dav1d_set_cpu_flags_mask(mask: libc::c_uint);
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
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





use crate::include::dav1d::headers::Dav1dFrameType;

























































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
pub struct Dav1dPicAllocator {
    pub cookie: *mut libc::c_void,
    pub alloc_picture_callback: Option::<
        unsafe extern "C" fn(*mut Dav1dPicture, *mut libc::c_void) -> libc::c_int,
    >,
    pub release_picture_callback: Option::<
        unsafe extern "C" fn(*mut Dav1dPicture, *mut libc::c_void) -> (),
    >,
}
use crate::include::dav1d::dav1d::Dav1dLogger;
use crate::include::dav1d::dav1d::Dav1dInloopFilterType;
use crate::include::dav1d::dav1d::DAV1D_INLOOPFILTER_ALL;
use crate::include::dav1d::dav1d::DAV1D_INLOOPFILTER_RESTORATION;
use crate::include::dav1d::dav1d::DAV1D_INLOOPFILTER_CDEF;
use crate::include::dav1d::dav1d::DAV1D_INLOOPFILTER_DEBLOCK;
use crate::include::dav1d::dav1d::DAV1D_INLOOPFILTER_NONE;
use crate::include::dav1d::dav1d::Dav1dDecodeFrameType;
use crate::include::dav1d::dav1d::DAV1D_DECODEFRAMETYPE_KEY;
use crate::include::dav1d::dav1d::DAV1D_DECODEFRAMETYPE_INTRA;
use crate::include::dav1d::dav1d::DAV1D_DECODEFRAMETYPE_REFERENCE;
use crate::include::dav1d::dav1d::DAV1D_DECODEFRAMETYPE_ALL;
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
pub const ARG_DECODE_FRAME_TYPE: C2RustUnnamed_13 = 273;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EnumParseTable {
    pub str_0: *const libc::c_char,
    pub val: libc::c_int,
}
pub const ARG_INLOOP_FILTERS: C2RustUnnamed_13 = 272;
pub const ARG_OUTPUT_INVISIBLE: C2RustUnnamed_13 = 271;
pub const ARG_NEG_STRIDE: C2RustUnnamed_13 = 270;
pub const ARG_CPU_MASK: C2RustUnnamed_13 = 269;
pub const ARG_STRICT_STD_COMPLIANCE: C2RustUnnamed_13 = 268;
pub const ARG_SIZE_LIMIT: C2RustUnnamed_13 = 267;
pub const ARG_ALL_LAYERS: C2RustUnnamed_13 = 266;
pub const ARG_OPPOINT: C2RustUnnamed_13 = 265;
pub const ARG_FILM_GRAIN: C2RustUnnamed_13 = 264;
pub const ARG_VERIFY: C2RustUnnamed_13 = 263;
pub const ARG_FRAME_DELAY: C2RustUnnamed_13 = 262;
pub const ARG_THREADS: C2RustUnnamed_13 = 261;
pub const ARG_REALTIME_CACHE: C2RustUnnamed_13 = 260;
pub const ARG_REALTIME: C2RustUnnamed_13 = 259;
pub const ARG_FRAME_TIMES: C2RustUnnamed_13 = 258;
pub const ARG_MUXER: C2RustUnnamed_13 = 257;
pub const ARG_DEMUXER: C2RustUnnamed_13 = 256;
pub const X86_CPU_MASK_AVX512ICL: CpuMask = 31;
pub const X86_CPU_MASK_AVX2: CpuMask = 15;
pub const X86_CPU_MASK_SSE41: CpuMask = 7;
pub const X86_CPU_MASK_SSSE3: CpuMask = 3;
pub const X86_CPU_MASK_SSE2: CpuMask = 1;
pub type C2RustUnnamed_13 = libc::c_uint;
pub type CpuMask = libc::c_uint;
static mut short_opts: [libc::c_char; 11] = unsafe {
    *::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"i:o:vql:s:\0")
};
static mut long_opts: [option; 25] = [
    {
        let mut init = option {
            name: b"input\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"output\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'o' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"quiet\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"demuxer\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: ARG_DEMUXER as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"muxer\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: ARG_MUXER as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"frametimes\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: ARG_FRAME_TIMES as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"limit\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"skip\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"realtime\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: ARG_REALTIME as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"realtimecache\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: ARG_REALTIME_CACHE as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"threads\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: ARG_THREADS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"framedelay\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: ARG_FRAME_DELAY as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"verify\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: ARG_VERIFY as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"filmgrain\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: ARG_FILM_GRAIN as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"oppoint\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: ARG_OPPOINT as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"alllayers\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: ARG_ALL_LAYERS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"sizelimit\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: ARG_SIZE_LIMIT as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"strict\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: ARG_STRICT_STD_COMPLIANCE as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"cpumask\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: ARG_CPU_MASK as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"negstride\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: ARG_NEG_STRIDE as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"outputinvisible\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: ARG_OUTPUT_INVISIBLE as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"inloopfilters\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: ARG_INLOOP_FILTERS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"decodeframetype\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: ARG_DECODE_FRAME_TYPE as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn usage(
    app: *const libc::c_char,
    reason: *const libc::c_char,
    mut args: ...
) {
    if !reason.is_null() {
        let mut args_0: ::core::ffi::VaListImpl;
        args_0 = args.clone();
        vfprintf(stderr, reason, args_0.as_va_list());
        fprintf(stderr, b"\n\n\0" as *const u8 as *const libc::c_char);
    }
    fprintf(
        stderr,
        b"Usage: %s [options]\n\n\0" as *const u8 as *const libc::c_char,
        app,
    );
    fprintf(
        stderr,
        b"Supported options:\n --input/-i $file:     input file\n --output/-o $file:    output file (%%n, %%w or %%h will be filled in for per-frame files)\n --demuxer $name:      force demuxer type ('ivf', 'section5' or 'annexb'; default: detect from content)\n --muxer $name:        force muxer type ('md5', 'yuv', 'yuv4mpeg2' or 'null'; default: detect from extension)\n                       use 'frame' as prefix to write per-frame files; if filename contains %%n, will default to writing per-frame files\n --quiet/-q:           disable status messages\n --frametimes $file:   dump frame times to file\n --limit/-l $num:      stop decoding after $num frames\n --skip/-s $num:       skip decoding of the first $num frames\n --realtime [$fract]:  limit framerate, optional argument to override input framerate\n --realtimecache $num: set the size of the cache in realtime mode (default: 0)\n --version/-v:         print version and exit\n --threads $num:       number of threads (default: 0)\n --framedelay $num:    maximum frame delay, capped at $threads (default: 0);\n                       set to 1 for low-latency decoding\n --filmgrain $num:     enable film grain application (default: 1, except if muxer is md5 or xxh3)\n --oppoint $num:       select an operating point of a scalable AV1 bitstream (0 - 31)\n --alllayers $num:     output all spatial layers of a scalable AV1 bitstream (default: 1)\n --sizelimit $num:     stop decoding if the frame size exceeds the specified limit\n --strict $num:        whether to abort decoding on standard compliance violations\n                       that don't affect bitstream decoding (default: 1)\n --verify $md5:        verify decoded md5. implies --muxer md5, no output\n --cpumask $mask:      restrict permitted CPU instruction sets (0, 'sse2', 'ssse3', 'sse41', 'avx2' or 'avx512icl'; default: -1)\n --negstride:          use negative picture strides\n                       this is mostly meant as a developer option\n --outputinvisible $num: whether to output invisible (alt-ref) frames (default: 0)\n --inloopfilters $str: which in-loop filters to enable (none, (no)deblock, (no)cdef, (no)restoration or all; default: all)\n --decodeframetype $str: which frame types to decode (reference, intra, key or all; default: all)\n\0"
            as *const u8 as *const libc::c_char,
    );
    exit(1 as libc::c_int);
}
unsafe extern "C" fn error(
    app: *const libc::c_char,
    optarg_0: *const libc::c_char,
    option: libc::c_int,
    shouldbe: *const libc::c_char,
) {
    let mut optname: [libc::c_char; 256] = [0; 256];
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while !(long_opts[n as usize].name).is_null() {
        if long_opts[n as usize].val == option {
            break;
        }
        n += 1;
    }
    if (long_opts[n as usize].name).is_null() {
        unreachable!();
    }
    if long_opts[n as usize].val < 256 as libc::c_int {
        sprintf(
            optname.as_mut_ptr(),
            b"-%c/--%s\0" as *const u8 as *const libc::c_char,
            long_opts[n as usize].val,
            long_opts[n as usize].name,
        );
    } else {
        sprintf(
            optname.as_mut_ptr(),
            b"--%s\0" as *const u8 as *const libc::c_char,
            long_opts[n as usize].name,
        );
    }
    usage(
        app,
        b"Invalid argument \"%s\" for option %s; should be %s\0" as *const u8
            as *const libc::c_char,
        optarg_0,
        optname.as_mut_ptr(),
        shouldbe,
    );
}
unsafe extern "C" fn parse_unsigned(
    optarg_0: *const libc::c_char,
    option: libc::c_int,
    app: *const libc::c_char,
) -> libc::c_uint {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let res: libc::c_uint = strtoul(optarg_0, &mut end, 0 as libc::c_int)
        as libc::c_uint;
    if *end as libc::c_int != 0 || end == optarg_0 as *mut libc::c_char {
        error(
            app,
            optarg_0,
            option,
            b"an integer\0" as *const u8 as *const libc::c_char,
        );
    }
    return res;
}
unsafe extern "C" fn parse_optional_fraction(
    optarg_0: *const libc::c_char,
    option: libc::c_int,
    app: *const libc::c_char,
    mut value: *mut libc::c_double,
) -> libc::c_int {
    if optarg_0.is_null() {
        return 0 as libc::c_int;
    }
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    *value = strtod(optarg_0, &mut end);
    if *end as libc::c_int == '/' as i32 && end != optarg_0 as *mut libc::c_char {
        let mut optarg2: *const libc::c_char = end.offset(1 as libc::c_int as isize);
        *value /= strtod(optarg2, &mut end);
        if *end as libc::c_int != 0 || end == optarg2 as *mut libc::c_char {
            error(
                app,
                optarg_0,
                option,
                b"a fraction\0" as *const u8 as *const libc::c_char,
            );
        }
    } else if *end as libc::c_int != 0 || end == optarg_0 as *mut libc::c_char {
        error(
            app,
            optarg_0,
            option,
            b"a fraction\0" as *const u8 as *const libc::c_char,
        );
    }
    return 1 as libc::c_int;
}
static mut cpu_mask_tbl: [EnumParseTable; 6] = [
    {
        let mut init = EnumParseTable {
            str_0: b"sse2\0" as *const u8 as *const libc::c_char,
            val: X86_CPU_MASK_SSE2 as libc::c_int,
        };
        init
    },
    {
        let mut init = EnumParseTable {
            str_0: b"ssse3\0" as *const u8 as *const libc::c_char,
            val: X86_CPU_MASK_SSSE3 as libc::c_int,
        };
        init
    },
    {
        let mut init = EnumParseTable {
            str_0: b"sse41\0" as *const u8 as *const libc::c_char,
            val: X86_CPU_MASK_SSE41 as libc::c_int,
        };
        init
    },
    {
        let mut init = EnumParseTable {
            str_0: b"avx2\0" as *const u8 as *const libc::c_char,
            val: X86_CPU_MASK_AVX2 as libc::c_int,
        };
        init
    },
    {
        let mut init = EnumParseTable {
            str_0: b"avx512icl\0" as *const u8 as *const libc::c_char,
            val: X86_CPU_MASK_AVX512ICL as libc::c_int,
        };
        init
    },
    {
        let mut init = EnumParseTable {
            str_0: b"none\0" as *const u8 as *const libc::c_char,
            val: 0 as libc::c_int,
        };
        init
    },
];
static mut inloop_filters_tbl: [EnumParseTable; 8] = [
    {
        let mut init = EnumParseTable {
            str_0: b"none\0" as *const u8 as *const libc::c_char,
            val: DAV1D_INLOOPFILTER_NONE as libc::c_int,
        };
        init
    },
    {
        let mut init = EnumParseTable {
            str_0: b"deblock\0" as *const u8 as *const libc::c_char,
            val: DAV1D_INLOOPFILTER_DEBLOCK as libc::c_int,
        };
        init
    },
    {
        let mut init = EnumParseTable {
            str_0: b"nodeblock\0" as *const u8 as *const libc::c_char,
            val: DAV1D_INLOOPFILTER_ALL as libc::c_int
                - DAV1D_INLOOPFILTER_DEBLOCK as libc::c_int,
        };
        init
    },
    {
        let mut init = EnumParseTable {
            str_0: b"cdef\0" as *const u8 as *const libc::c_char,
            val: DAV1D_INLOOPFILTER_CDEF as libc::c_int,
        };
        init
    },
    {
        let mut init = EnumParseTable {
            str_0: b"nocdef\0" as *const u8 as *const libc::c_char,
            val: DAV1D_INLOOPFILTER_ALL as libc::c_int
                - DAV1D_INLOOPFILTER_CDEF as libc::c_int,
        };
        init
    },
    {
        let mut init = EnumParseTable {
            str_0: b"restoration\0" as *const u8 as *const libc::c_char,
            val: DAV1D_INLOOPFILTER_RESTORATION as libc::c_int,
        };
        init
    },
    {
        let mut init = EnumParseTable {
            str_0: b"norestoration\0" as *const u8 as *const libc::c_char,
            val: DAV1D_INLOOPFILTER_ALL as libc::c_int
                - DAV1D_INLOOPFILTER_RESTORATION as libc::c_int,
        };
        init
    },
    {
        let mut init = EnumParseTable {
            str_0: b"all\0" as *const u8 as *const libc::c_char,
            val: DAV1D_INLOOPFILTER_ALL as libc::c_int,
        };
        init
    },
];
static mut decode_frame_type_tbl: [EnumParseTable; 4] = [
    {
        let mut init = EnumParseTable {
            str_0: b"all\0" as *const u8 as *const libc::c_char,
            val: DAV1D_DECODEFRAMETYPE_ALL as libc::c_int,
        };
        init
    },
    {
        let mut init = EnumParseTable {
            str_0: b"reference\0" as *const u8 as *const libc::c_char,
            val: DAV1D_DECODEFRAMETYPE_REFERENCE as libc::c_int,
        };
        init
    },
    {
        let mut init = EnumParseTable {
            str_0: b"intra\0" as *const u8 as *const libc::c_char,
            val: DAV1D_DECODEFRAMETYPE_INTRA as libc::c_int,
        };
        init
    },
    {
        let mut init = EnumParseTable {
            str_0: b"key\0" as *const u8 as *const libc::c_char,
            val: DAV1D_DECODEFRAMETYPE_KEY as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn parse_enum(
    mut optarg_0: *mut libc::c_char,
    tbl: *const EnumParseTable,
    tbl_sz: libc::c_int,
    option: libc::c_int,
    mut app: *const libc::c_char,
) -> libc::c_uint {
    let mut str: [libc::c_char; 1024] = [0; 1024];
    strcpy(str.as_mut_ptr(), b"any of \0" as *const u8 as *const libc::c_char);
    let mut n: libc::c_int = 0 as libc::c_int;
    while n < tbl_sz {
        if strcmp((*tbl.offset(n as isize)).str_0, optarg_0) == 0 {
            return (*tbl.offset(n as isize)).val as libc::c_uint;
        }
        if n != 0 {
            if n < tbl_sz - 1 as libc::c_int {
                strcat(str.as_mut_ptr(), b", \0" as *const u8 as *const libc::c_char);
            } else {
                strcat(str.as_mut_ptr(), b" or \0" as *const u8 as *const libc::c_char);
            }
        }
        strcat(str.as_mut_ptr(), (*tbl.offset(n as isize)).str_0);
        n += 1;
    }
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut res: libc::c_uint = 0;
    if strncmp(
        optarg_0,
        b"0x\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        res = strtoul(
            &mut *optarg_0.offset(2 as libc::c_int as isize),
            &mut end,
            16 as libc::c_int,
        ) as libc::c_uint;
    } else {
        res = strtoul(optarg_0, &mut end, 0 as libc::c_int) as libc::c_uint;
    }
    if *end as libc::c_int != 0 || end == optarg_0 {
        strcat(
            str.as_mut_ptr(),
            b", a hexadecimal (starting with 0x), or an integer\0" as *const u8
                as *const libc::c_char,
        );
        error(app, optarg_0, option, str.as_mut_ptr());
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn parse(
    argc: libc::c_int,
    argv: *const *mut libc::c_char,
    cli_settings: *mut CLISettings,
    lib_settings: *mut Dav1dSettings,
) {
    let mut o: libc::c_int = 0;
    memset(
        cli_settings as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<CLISettings>() as libc::c_ulong,
    );
    dav1d_default_settings(lib_settings);
    (*lib_settings).strict_std_compliance = 1 as libc::c_int;
    let mut grain_specified: libc::c_int = 0 as libc::c_int;
    loop {
        o = getopt_long(
            argc,
            argv,
            short_opts.as_ptr(),
            long_opts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(o != -(1 as libc::c_int)) {
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
                (*cli_settings).quiet = 1 as libc::c_int;
            }
            108 => {
                (*cli_settings)
                    .limit = parse_unsigned(
                    optarg,
                    'l' as i32,
                    *argv.offset(0 as libc::c_int as isize),
                );
            }
            115 => {
                (*cli_settings)
                    .skip = parse_unsigned(
                    optarg,
                    's' as i32,
                    *argv.offset(0 as libc::c_int as isize),
                );
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
                if optarg.is_null() && optind < argc
                    && !(*argv.offset(optind as isize)).is_null()
                    && *(*argv.offset(optind as isize)).offset(0 as libc::c_int as isize)
                        as libc::c_int != '-' as i32
                {
                    optarg = *argv.offset(optind as isize);
                    optind += 1;
                }
                (*cli_settings)
                    .realtime = (1 as libc::c_int
                    + parse_optional_fraction(
                        optarg,
                        ARG_REALTIME as libc::c_int,
                        *argv.offset(0 as libc::c_int as isize),
                        &mut (*cli_settings).realtime_fps,
                    )) as C2RustUnnamed_12;
            }
            260 => {
                (*cli_settings)
                    .realtime_cache = parse_unsigned(
                    optarg,
                    ARG_REALTIME_CACHE as libc::c_int,
                    *argv.offset(0 as libc::c_int as isize),
                );
            }
            262 => {
                (*lib_settings)
                    .max_frame_delay = parse_unsigned(
                    optarg,
                    ARG_FRAME_DELAY as libc::c_int,
                    *argv.offset(0 as libc::c_int as isize),
                ) as libc::c_int;
            }
            261 => {
                (*lib_settings)
                    .n_threads = parse_unsigned(
                    optarg,
                    ARG_THREADS as libc::c_int,
                    *argv.offset(0 as libc::c_int as isize),
                ) as libc::c_int;
            }
            263 => {
                (*cli_settings).verify = optarg;
            }
            264 => {
                (*lib_settings)
                    .apply_grain = (parse_unsigned(
                    optarg,
                    ARG_FILM_GRAIN as libc::c_int,
                    *argv.offset(0 as libc::c_int as isize),
                ) != 0) as libc::c_int;
                grain_specified = 1 as libc::c_int;
            }
            265 => {
                (*lib_settings)
                    .operating_point = parse_unsigned(
                    optarg,
                    ARG_OPPOINT as libc::c_int,
                    *argv.offset(0 as libc::c_int as isize),
                ) as libc::c_int;
            }
            266 => {
                (*lib_settings)
                    .all_layers = (parse_unsigned(
                    optarg,
                    ARG_ALL_LAYERS as libc::c_int,
                    *argv.offset(0 as libc::c_int as isize),
                ) != 0) as libc::c_int;
            }
            267 => {
                let mut arg: *mut libc::c_char = optarg;
                let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut res: uint64_t = strtoul(arg, &mut end, 0) as uint64_t;
                if *end as libc::c_int == 'x' as i32 {
                    arg = end.offset(1 as libc::c_int as isize);
                    res = (res as libc::c_ulong)
                        .wrapping_mul(strtoul(arg, &mut end, 0))
                        as uint64_t as uint64_t;
                }
                if *end as libc::c_int != 0 || end == arg
                    || res >= u32::MAX as uint64_t
                {
                    error(
                        *argv.offset(0 as libc::c_int as isize),
                        optarg,
                        ARG_SIZE_LIMIT as libc::c_int,
                        b"an integer or dimension\0" as *const u8 as *const libc::c_char,
                    );
                }
                (*lib_settings).frame_size_limit = res as libc::c_uint;
            }
            268 => {
                (*lib_settings)
                    .strict_std_compliance = parse_unsigned(
                    optarg,
                    ARG_STRICT_STD_COMPLIANCE as libc::c_int,
                    *argv.offset(0 as libc::c_int as isize),
                ) as libc::c_int;
            }
            118 => {
                fprintf(
                    stderr,
                    b"%s\n\0" as *const u8 as *const libc::c_char,
                    dav1d_version(),
                );
                exit(0 as libc::c_int);
            }
            269 => {
                dav1d_set_cpu_flags_mask(
                    parse_enum(
                        optarg,
                        cpu_mask_tbl.as_ptr(),
                        (::core::mem::size_of::<[EnumParseTable; 6]>() as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<EnumParseTable>() as libc::c_ulong,
                            ) as libc::c_int,
                        ARG_CPU_MASK as libc::c_int,
                        *argv.offset(0 as libc::c_int as isize),
                    ),
                );
            }
            270 => {
                (*cli_settings).neg_stride = 1 as libc::c_int;
            }
            271 => {
                (*lib_settings)
                    .output_invisible_frames = (parse_unsigned(
                    optarg,
                    ARG_OUTPUT_INVISIBLE as libc::c_int,
                    *argv.offset(0 as libc::c_int as isize),
                ) != 0) as libc::c_int;
            }
            272 => {
                (*lib_settings)
                    .inloop_filters = parse_enum(
                    optarg,
                    inloop_filters_tbl.as_ptr(),
                    (::core::mem::size_of::<[EnumParseTable; 8]>() as libc::c_ulong)
                        .wrapping_div(
                            ::core::mem::size_of::<EnumParseTable>() as libc::c_ulong,
                        ) as libc::c_int,
                    ARG_INLOOP_FILTERS as libc::c_int,
                    *argv.offset(0 as libc::c_int as isize),
                ) as Dav1dInloopFilterType;
            }
            273 => {
                (*lib_settings)
                    .decode_frame_type = parse_enum(
                    optarg,
                    decode_frame_type_tbl.as_ptr(),
                    (::core::mem::size_of::<[EnumParseTable; 4]>() as libc::c_ulong)
                        .wrapping_div(
                            ::core::mem::size_of::<EnumParseTable>() as libc::c_ulong,
                        ) as libc::c_int,
                    ARG_DECODE_FRAME_TYPE as libc::c_int,
                    *argv.offset(0 as libc::c_int as isize),
                ) as Dav1dDecodeFrameType;
            }
            _ => {
                usage(*argv.offset(0 as libc::c_int as isize), 0 as *const libc::c_char);
            }
        }
    }
    if optind < argc {
        usage(
            *argv.offset(0 as libc::c_int as isize),
            b"Extra/unused arguments found, e.g. '%s'\n\0" as *const u8
                as *const libc::c_char,
            *argv.offset(optind as isize),
        );
    }
    if !((*cli_settings).verify).is_null() {
        if !((*cli_settings).outputfile).is_null() {
            usage(
                *argv.offset(0 as libc::c_int as isize),
                b"Verification (--verify) requires output file (-o/--output) to not set\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if !((*cli_settings).muxer).is_null()
            && strcmp(
                (*cli_settings).muxer,
                b"md5\0" as *const u8 as *const libc::c_char,
            ) != 0
            && strcmp(
                (*cli_settings).muxer,
                b"xxh3\0" as *const u8 as *const libc::c_char,
            ) != 0
        {
            usage(
                *argv.offset(0 as libc::c_int as isize),
                b"Verification (--verify) requires a checksum muxer (md5 or xxh3)\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        (*cli_settings).outputfile = b"-\0" as *const u8 as *const libc::c_char;
        if ((*cli_settings).muxer).is_null() {
            (*cli_settings).muxer = b"md5\0" as *const u8 as *const libc::c_char;
        }
    }
    if grain_specified == 0 && !((*cli_settings).muxer).is_null()
        && (strcmp((*cli_settings).muxer, b"md5\0" as *const u8 as *const libc::c_char)
            == 0
            || strcmp(
                (*cli_settings).muxer,
                b"xxh3\0" as *const u8 as *const libc::c_char,
            ) == 0)
    {
        (*lib_settings).apply_grain = 0 as libc::c_int;
    }
    if ((*cli_settings).inputfile).is_null() {
        usage(
            *argv.offset(0 as libc::c_int as isize),
            b"Input file (-i/--input) is required\0" as *const u8 as *const libc::c_char,
        );
    }
    if (((*cli_settings).muxer).is_null()
        || strcmp((*cli_settings).muxer, b"null\0" as *const u8 as *const libc::c_char)
            != 0) && ((*cli_settings).outputfile).is_null()
    {
        usage(
            *argv.offset(0 as libc::c_int as isize),
            b"Output file (-o/--output) is required\0" as *const u8
                as *const libc::c_char,
        );
    }
}
