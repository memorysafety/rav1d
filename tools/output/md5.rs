use ::libc;
use crate::{stdout,stderr};
use crate::errno_location;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type Dav1dRef;
    fn fclose(__stream: *mut libc::FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::FILE;
    fn fprintf(_: *mut libc::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn dav1d_picture_unref(p: *mut Dav1dPicture);
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
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
pub type Dav1dTxfmMode = libc::c_uint;
pub const DAV1D_N_TX_MODES: Dav1dTxfmMode = 3;
pub const DAV1D_TX_SWITCHABLE: Dav1dTxfmMode = 2;
pub const DAV1D_TX_LARGEST: Dav1dTxfmMode = 1;
pub const DAV1D_TX_4X4_ONLY: Dav1dTxfmMode = 0;
pub type Dav1dFilterMode = libc::c_uint;
pub const DAV1D_FILTER_SWITCHABLE: Dav1dFilterMode = 4;
pub const DAV1D_N_FILTERS: Dav1dFilterMode = 4;
pub const DAV1D_FILTER_BILINEAR: Dav1dFilterMode = 3;
pub const DAV1D_N_SWITCHABLE_FILTERS: Dav1dFilterMode = 3;
pub const DAV1D_FILTER_8TAP_SHARP: Dav1dFilterMode = 2;
pub const DAV1D_FILTER_8TAP_SMOOTH: Dav1dFilterMode = 1;
pub const DAV1D_FILTER_8TAP_REGULAR: Dav1dFilterMode = 0;
pub type Dav1dAdaptiveBoolean = libc::c_uint;
pub const DAV1D_ADAPTIVE: Dav1dAdaptiveBoolean = 2;
pub const DAV1D_ON: Dav1dAdaptiveBoolean = 1;
pub const DAV1D_OFF: Dav1dAdaptiveBoolean = 0;
pub type Dav1dRestorationType = libc::c_uint;
pub const DAV1D_RESTORATION_SGRPROJ: Dav1dRestorationType = 3;
pub const DAV1D_RESTORATION_WIENER: Dav1dRestorationType = 2;
pub const DAV1D_RESTORATION_SWITCHABLE: Dav1dRestorationType = 1;
pub const DAV1D_RESTORATION_NONE: Dav1dRestorationType = 0;
pub type Dav1dWarpedMotionType = libc::c_uint;
pub const DAV1D_WM_TYPE_AFFINE: Dav1dWarpedMotionType = 3;
pub const DAV1D_WM_TYPE_ROT_ZOOM: Dav1dWarpedMotionType = 2;
pub const DAV1D_WM_TYPE_TRANSLATION: Dav1dWarpedMotionType = 1;
pub const DAV1D_WM_TYPE_IDENTITY: Dav1dWarpedMotionType = 0;
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
pub struct Dav1dContentLightLevel {
    pub max_content_light_level: libc::c_int,
    pub max_frame_average_light_level: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dMasteringDisplay {
    pub primaries: [[uint16_t; 2]; 3],
    pub white_point: [uint16_t; 2],
    pub max_luminance: uint32_t,
    pub min_luminance: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dITUTT35 {
    pub country_code: uint8_t,
    pub country_code_extension_byte: uint8_t,
    pub payload_size: size_t,
    pub payload: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dSequenceHeader {
    pub profile: libc::c_int,
    pub max_width: libc::c_int,
    pub max_height: libc::c_int,
    pub layout: Dav1dPixelLayout,
    pub pri: Dav1dColorPrimaries,
    pub trc: Dav1dTransferCharacteristics,
    pub mtrx: Dav1dMatrixCoefficients,
    pub chr: Dav1dChromaSamplePosition,
    pub hbd: libc::c_int,
    pub color_range: libc::c_int,
    pub num_operating_points: libc::c_int,
    pub operating_points: [Dav1dSequenceHeaderOperatingPoint; 32],
    pub still_picture: libc::c_int,
    pub reduced_still_picture_header: libc::c_int,
    pub timing_info_present: libc::c_int,
    pub num_units_in_tick: libc::c_int,
    pub time_scale: libc::c_int,
    pub equal_picture_interval: libc::c_int,
    pub num_ticks_per_picture: libc::c_uint,
    pub decoder_model_info_present: libc::c_int,
    pub encoder_decoder_buffer_delay_length: libc::c_int,
    pub num_units_in_decoding_tick: libc::c_int,
    pub buffer_removal_delay_length: libc::c_int,
    pub frame_presentation_delay_length: libc::c_int,
    pub display_model_info_present: libc::c_int,
    pub width_n_bits: libc::c_int,
    pub height_n_bits: libc::c_int,
    pub frame_id_numbers_present: libc::c_int,
    pub delta_frame_id_n_bits: libc::c_int,
    pub frame_id_n_bits: libc::c_int,
    pub sb128: libc::c_int,
    pub filter_intra: libc::c_int,
    pub intra_edge_filter: libc::c_int,
    pub inter_intra: libc::c_int,
    pub masked_compound: libc::c_int,
    pub warped_motion: libc::c_int,
    pub dual_filter: libc::c_int,
    pub order_hint: libc::c_int,
    pub jnt_comp: libc::c_int,
    pub ref_frame_mvs: libc::c_int,
    pub screen_content_tools: Dav1dAdaptiveBoolean,
    pub force_integer_mv: Dav1dAdaptiveBoolean,
    pub order_hint_n_bits: libc::c_int,
    pub super_res: libc::c_int,
    pub cdef: libc::c_int,
    pub restoration: libc::c_int,
    pub ss_hor: libc::c_int,
    pub ss_ver: libc::c_int,
    pub monochrome: libc::c_int,
    pub color_description_present: libc::c_int,
    pub separate_uv_delta_q: libc::c_int,
    pub film_grain_present: libc::c_int,
    pub operating_parameter_info: [Dav1dSequenceHeaderOperatingParameterInfo; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dSequenceHeaderOperatingParameterInfo {
    pub decoder_buffer_delay: libc::c_int,
    pub encoder_buffer_delay: libc::c_int,
    pub low_delay_mode: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dSequenceHeaderOperatingPoint {
    pub major_level: libc::c_int,
    pub minor_level: libc::c_int,
    pub initial_display_delay: libc::c_int,
    pub idc: libc::c_int,
    pub tier: libc::c_int,
    pub decoder_model_param_present: libc::c_int,
    pub display_model_param_present: libc::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameHeaderOperatingPoint {
    pub buffer_removal_time: libc::c_int,
}
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
pub struct MuxerPriv {
    pub abcd: [uint32_t; 4],
    pub c2rust_unnamed: C2RustUnnamed_12,
    pub len: uint64_t,
    pub f: *mut libc::FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub data: [uint8_t; 64],
    pub data32: [uint32_t; 16],
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
pub type MD5Context = MuxerPriv;
#[inline]
unsafe extern "C" fn umin(a: libc::c_uint, b: libc::c_uint) -> libc::c_uint {
    return if a < b { a } else { b };
}
static mut k: [uint32_t; 64] = [
    0xd76aa478 as libc::c_uint,
    0xe8c7b756 as libc::c_uint,
    0x242070db as libc::c_int as uint32_t,
    0xc1bdceee as libc::c_uint,
    0xf57c0faf as libc::c_uint,
    0x4787c62a as libc::c_int as uint32_t,
    0xa8304613 as libc::c_uint,
    0xfd469501 as libc::c_uint,
    0x698098d8 as libc::c_int as uint32_t,
    0x8b44f7af as libc::c_uint,
    0xffff5bb1 as libc::c_uint,
    0x895cd7be as libc::c_uint,
    0x6b901122 as libc::c_int as uint32_t,
    0xfd987193 as libc::c_uint,
    0xa679438e as libc::c_uint,
    0x49b40821 as libc::c_int as uint32_t,
    0xf61e2562 as libc::c_uint,
    0xc040b340 as libc::c_uint,
    0x265e5a51 as libc::c_int as uint32_t,
    0xe9b6c7aa as libc::c_uint,
    0xd62f105d as libc::c_uint,
    0x2441453 as libc::c_int as uint32_t,
    0xd8a1e681 as libc::c_uint,
    0xe7d3fbc8 as libc::c_uint,
    0x21e1cde6 as libc::c_int as uint32_t,
    0xc33707d6 as libc::c_uint,
    0xf4d50d87 as libc::c_uint,
    0x455a14ed as libc::c_int as uint32_t,
    0xa9e3e905 as libc::c_uint,
    0xfcefa3f8 as libc::c_uint,
    0x676f02d9 as libc::c_int as uint32_t,
    0x8d2a4c8a as libc::c_uint,
    0xfffa3942 as libc::c_uint,
    0x8771f681 as libc::c_uint,
    0x6d9d6122 as libc::c_int as uint32_t,
    0xfde5380c as libc::c_uint,
    0xa4beea44 as libc::c_uint,
    0x4bdecfa9 as libc::c_int as uint32_t,
    0xf6bb4b60 as libc::c_uint,
    0xbebfbc70 as libc::c_uint,
    0x289b7ec6 as libc::c_int as uint32_t,
    0xeaa127fa as libc::c_uint,
    0xd4ef3085 as libc::c_uint,
    0x4881d05 as libc::c_int as uint32_t,
    0xd9d4d039 as libc::c_uint,
    0xe6db99e5 as libc::c_uint,
    0x1fa27cf8 as libc::c_int as uint32_t,
    0xc4ac5665 as libc::c_uint,
    0xf4292244 as libc::c_uint,
    0x432aff97 as libc::c_int as uint32_t,
    0xab9423a7 as libc::c_uint,
    0xfc93a039 as libc::c_uint,
    0x655b59c3 as libc::c_int as uint32_t,
    0x8f0ccc92 as libc::c_uint,
    0xffeff47d as libc::c_uint,
    0x85845dd1 as libc::c_uint,
    0x6fa87e4f as libc::c_int as uint32_t,
    0xfe2ce6e0 as libc::c_uint,
    0xa3014314 as libc::c_uint,
    0x4e0811a1 as libc::c_int as uint32_t,
    0xf7537e82 as libc::c_uint,
    0xbd3af235 as libc::c_uint,
    0x2ad7d2bb as libc::c_int as uint32_t,
    0xeb86d391 as libc::c_uint,
];
unsafe extern "C" fn md5_open(
    md5: *mut MD5Context,
    file: *const libc::c_char,
    p: *const Dav1dPictureParameters,
    mut fps: *const libc::c_uint,
) -> libc::c_int {
    if strcmp(file, b"-\0" as *const u8 as *const libc::c_char) == 0 {
        (*md5).f = stdout;
    } else {
        (*md5).f = fopen(file, b"wb\0" as *const u8 as *const libc::c_char);
        if ((*md5).f).is_null() {
            fprintf(
                stderr,
                b"Failed to open %s: %s\n\0" as *const u8 as *const libc::c_char,
                file,
                strerror(*errno_location()),
            );
            return -(1 as libc::c_int);
        }
    }
    (*md5).abcd[0 as libc::c_int as usize] = 0x67452301 as libc::c_int as uint32_t;
    (*md5).abcd[1 as libc::c_int as usize] = 0xefcdab89 as libc::c_uint;
    (*md5).abcd[2 as libc::c_int as usize] = 0x98badcfe as libc::c_uint;
    (*md5).abcd[3 as libc::c_int as usize] = 0x10325476 as libc::c_int as uint32_t;
    (*md5).len = 0 as libc::c_int as uint64_t;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn leftrotate(x: uint32_t, c: libc::c_int) -> uint32_t {
    return x << c | x >> 32 as libc::c_int - c;
}
unsafe extern "C" fn md5_body(md5: *mut MD5Context, data: *const uint32_t) {
    let mut a: uint32_t = (*md5).abcd[0 as libc::c_int as usize];
    let mut b: uint32_t = (*md5).abcd[1 as libc::c_int as usize];
    let mut c: uint32_t = (*md5).abcd[2 as libc::c_int as usize];
    let mut d: uint32_t = (*md5).abcd[3 as libc::c_int as usize];
    a = b
        .wrapping_add(
            leftrotate(
                a
                    .wrapping_add(b & c | !b & d)
                    .wrapping_add(k[(0 as libc::c_int + 0 as libc::c_int) as usize])
                    .wrapping_add(
                        *data.offset((0 as libc::c_int + 0 as libc::c_int) as isize),
                    ),
                7 as libc::c_int,
            ),
        );
    d = a
        .wrapping_add(
            leftrotate(
                d
                    .wrapping_add(a & b | !a & c)
                    .wrapping_add(k[(0 as libc::c_int + 1 as libc::c_int) as usize])
                    .wrapping_add(
                        *data.offset((0 as libc::c_int + 1 as libc::c_int) as isize),
                    ),
                12 as libc::c_int,
            ),
        );
    c = d
        .wrapping_add(
            leftrotate(
                c
                    .wrapping_add(d & a | !d & b)
                    .wrapping_add(k[(0 as libc::c_int + 2 as libc::c_int) as usize])
                    .wrapping_add(
                        *data.offset((0 as libc::c_int + 2 as libc::c_int) as isize),
                    ),
                17 as libc::c_int,
            ),
        );
    b = c
        .wrapping_add(
            leftrotate(
                b
                    .wrapping_add(c & d | !c & a)
                    .wrapping_add(k[(0 as libc::c_int + 3 as libc::c_int) as usize])
                    .wrapping_add(
                        *data.offset((0 as libc::c_int + 3 as libc::c_int) as isize),
                    ),
                22 as libc::c_int,
            ),
        );
    a = b
        .wrapping_add(
            leftrotate(
                a
                    .wrapping_add(b & c | !b & d)
                    .wrapping_add(k[(4 as libc::c_int + 0 as libc::c_int) as usize])
                    .wrapping_add(
                        *data.offset((4 as libc::c_int + 0 as libc::c_int) as isize),
                    ),
                7 as libc::c_int,
            ),
        );
    d = a
        .wrapping_add(
            leftrotate(
                d
                    .wrapping_add(a & b | !a & c)
                    .wrapping_add(k[(4 as libc::c_int + 1 as libc::c_int) as usize])
                    .wrapping_add(
                        *data.offset((4 as libc::c_int + 1 as libc::c_int) as isize),
                    ),
                12 as libc::c_int,
            ),
        );
    c = d
        .wrapping_add(
            leftrotate(
                c
                    .wrapping_add(d & a | !d & b)
                    .wrapping_add(k[(4 as libc::c_int + 2 as libc::c_int) as usize])
                    .wrapping_add(
                        *data.offset((4 as libc::c_int + 2 as libc::c_int) as isize),
                    ),
                17 as libc::c_int,
            ),
        );
    b = c
        .wrapping_add(
            leftrotate(
                b
                    .wrapping_add(c & d | !c & a)
                    .wrapping_add(k[(4 as libc::c_int + 3 as libc::c_int) as usize])
                    .wrapping_add(
                        *data.offset((4 as libc::c_int + 3 as libc::c_int) as isize),
                    ),
                22 as libc::c_int,
            ),
        );
    a = b
        .wrapping_add(
            leftrotate(
                a
                    .wrapping_add(b & c | !b & d)
                    .wrapping_add(k[(8 as libc::c_int + 0 as libc::c_int) as usize])
                    .wrapping_add(
                        *data.offset((8 as libc::c_int + 0 as libc::c_int) as isize),
                    ),
                7 as libc::c_int,
            ),
        );
    d = a
        .wrapping_add(
            leftrotate(
                d
                    .wrapping_add(a & b | !a & c)
                    .wrapping_add(k[(8 as libc::c_int + 1 as libc::c_int) as usize])
                    .wrapping_add(
                        *data.offset((8 as libc::c_int + 1 as libc::c_int) as isize),
                    ),
                12 as libc::c_int,
            ),
        );
    c = d
        .wrapping_add(
            leftrotate(
                c
                    .wrapping_add(d & a | !d & b)
                    .wrapping_add(k[(8 as libc::c_int + 2 as libc::c_int) as usize])
                    .wrapping_add(
                        *data.offset((8 as libc::c_int + 2 as libc::c_int) as isize),
                    ),
                17 as libc::c_int,
            ),
        );
    b = c
        .wrapping_add(
            leftrotate(
                b
                    .wrapping_add(c & d | !c & a)
                    .wrapping_add(k[(8 as libc::c_int + 3 as libc::c_int) as usize])
                    .wrapping_add(
                        *data.offset((8 as libc::c_int + 3 as libc::c_int) as isize),
                    ),
                22 as libc::c_int,
            ),
        );
    a = b
        .wrapping_add(
            leftrotate(
                a
                    .wrapping_add(b & c | !b & d)
                    .wrapping_add(k[(12 as libc::c_int + 0 as libc::c_int) as usize])
                    .wrapping_add(
                        *data.offset((12 as libc::c_int + 0 as libc::c_int) as isize),
                    ),
                7 as libc::c_int,
            ),
        );
    d = a
        .wrapping_add(
            leftrotate(
                d
                    .wrapping_add(a & b | !a & c)
                    .wrapping_add(k[(12 as libc::c_int + 1 as libc::c_int) as usize])
                    .wrapping_add(
                        *data.offset((12 as libc::c_int + 1 as libc::c_int) as isize),
                    ),
                12 as libc::c_int,
            ),
        );
    c = d
        .wrapping_add(
            leftrotate(
                c
                    .wrapping_add(d & a | !d & b)
                    .wrapping_add(k[(12 as libc::c_int + 2 as libc::c_int) as usize])
                    .wrapping_add(
                        *data.offset((12 as libc::c_int + 2 as libc::c_int) as isize),
                    ),
                17 as libc::c_int,
            ),
        );
    b = c
        .wrapping_add(
            leftrotate(
                b
                    .wrapping_add(c & d | !c & a)
                    .wrapping_add(k[(12 as libc::c_int + 3 as libc::c_int) as usize])
                    .wrapping_add(
                        *data.offset((12 as libc::c_int + 3 as libc::c_int) as isize),
                    ),
                22 as libc::c_int,
            ),
        );
    a = b
        .wrapping_add(
            leftrotate(
                a
                    .wrapping_add(d & b | !d & c)
                    .wrapping_add(k[(16 as libc::c_int + 0 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (16 as libc::c_int + 1 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                5 as libc::c_int,
            ),
        );
    d = a
        .wrapping_add(
            leftrotate(
                d
                    .wrapping_add(c & a | !c & b)
                    .wrapping_add(k[(16 as libc::c_int + 1 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (16 as libc::c_int + 6 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                9 as libc::c_int,
            ),
        );
    c = d
        .wrapping_add(
            leftrotate(
                c
                    .wrapping_add(b & d | !b & a)
                    .wrapping_add(k[(16 as libc::c_int + 2 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (16 as libc::c_int + 11 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                14 as libc::c_int,
            ),
        );
    b = c
        .wrapping_add(
            leftrotate(
                b
                    .wrapping_add(a & c | !a & d)
                    .wrapping_add(k[(16 as libc::c_int + 3 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (16 as libc::c_int + 0 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                20 as libc::c_int,
            ),
        );
    a = b
        .wrapping_add(
            leftrotate(
                a
                    .wrapping_add(d & b | !d & c)
                    .wrapping_add(k[(20 as libc::c_int + 0 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (20 as libc::c_int + 1 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                5 as libc::c_int,
            ),
        );
    d = a
        .wrapping_add(
            leftrotate(
                d
                    .wrapping_add(c & a | !c & b)
                    .wrapping_add(k[(20 as libc::c_int + 1 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (20 as libc::c_int + 6 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                9 as libc::c_int,
            ),
        );
    c = d
        .wrapping_add(
            leftrotate(
                c
                    .wrapping_add(b & d | !b & a)
                    .wrapping_add(k[(20 as libc::c_int + 2 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (20 as libc::c_int + 11 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                14 as libc::c_int,
            ),
        );
    b = c
        .wrapping_add(
            leftrotate(
                b
                    .wrapping_add(a & c | !a & d)
                    .wrapping_add(k[(20 as libc::c_int + 3 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (20 as libc::c_int + 0 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                20 as libc::c_int,
            ),
        );
    a = b
        .wrapping_add(
            leftrotate(
                a
                    .wrapping_add(d & b | !d & c)
                    .wrapping_add(k[(24 as libc::c_int + 0 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (24 as libc::c_int + 1 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                5 as libc::c_int,
            ),
        );
    d = a
        .wrapping_add(
            leftrotate(
                d
                    .wrapping_add(c & a | !c & b)
                    .wrapping_add(k[(24 as libc::c_int + 1 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (24 as libc::c_int + 6 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                9 as libc::c_int,
            ),
        );
    c = d
        .wrapping_add(
            leftrotate(
                c
                    .wrapping_add(b & d | !b & a)
                    .wrapping_add(k[(24 as libc::c_int + 2 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (24 as libc::c_int + 11 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                14 as libc::c_int,
            ),
        );
    b = c
        .wrapping_add(
            leftrotate(
                b
                    .wrapping_add(a & c | !a & d)
                    .wrapping_add(k[(24 as libc::c_int + 3 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (24 as libc::c_int + 0 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                20 as libc::c_int,
            ),
        );
    a = b
        .wrapping_add(
            leftrotate(
                a
                    .wrapping_add(d & b | !d & c)
                    .wrapping_add(k[(28 as libc::c_int + 0 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (28 as libc::c_int + 1 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                5 as libc::c_int,
            ),
        );
    d = a
        .wrapping_add(
            leftrotate(
                d
                    .wrapping_add(c & a | !c & b)
                    .wrapping_add(k[(28 as libc::c_int + 1 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (28 as libc::c_int + 6 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                9 as libc::c_int,
            ),
        );
    c = d
        .wrapping_add(
            leftrotate(
                c
                    .wrapping_add(b & d | !b & a)
                    .wrapping_add(k[(28 as libc::c_int + 2 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (28 as libc::c_int + 11 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                14 as libc::c_int,
            ),
        );
    b = c
        .wrapping_add(
            leftrotate(
                b
                    .wrapping_add(a & c | !a & d)
                    .wrapping_add(k[(28 as libc::c_int + 3 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (28 as libc::c_int + 0 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                20 as libc::c_int,
            ),
        );
    a = b
        .wrapping_add(
            leftrotate(
                a
                    .wrapping_add(b ^ c ^ d)
                    .wrapping_add(k[(32 as libc::c_int + 0 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (5 as libc::c_int - 32 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                4 as libc::c_int,
            ),
        );
    d = a
        .wrapping_add(
            leftrotate(
                d
                    .wrapping_add(a ^ b ^ c)
                    .wrapping_add(k[(32 as libc::c_int + 1 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (8 as libc::c_int - 32 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                11 as libc::c_int,
            ),
        );
    c = d
        .wrapping_add(
            leftrotate(
                c
                    .wrapping_add(d ^ a ^ b)
                    .wrapping_add(k[(32 as libc::c_int + 2 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (11 as libc::c_int - 32 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                16 as libc::c_int,
            ),
        );
    b = c
        .wrapping_add(
            leftrotate(
                b
                    .wrapping_add(c ^ d ^ a)
                    .wrapping_add(k[(32 as libc::c_int + 3 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (14 as libc::c_int - 32 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                23 as libc::c_int,
            ),
        );
    a = b
        .wrapping_add(
            leftrotate(
                a
                    .wrapping_add(b ^ c ^ d)
                    .wrapping_add(k[(36 as libc::c_int + 0 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (5 as libc::c_int - 36 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                4 as libc::c_int,
            ),
        );
    d = a
        .wrapping_add(
            leftrotate(
                d
                    .wrapping_add(a ^ b ^ c)
                    .wrapping_add(k[(36 as libc::c_int + 1 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (8 as libc::c_int - 36 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                11 as libc::c_int,
            ),
        );
    c = d
        .wrapping_add(
            leftrotate(
                c
                    .wrapping_add(d ^ a ^ b)
                    .wrapping_add(k[(36 as libc::c_int + 2 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (11 as libc::c_int - 36 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                16 as libc::c_int,
            ),
        );
    b = c
        .wrapping_add(
            leftrotate(
                b
                    .wrapping_add(c ^ d ^ a)
                    .wrapping_add(k[(36 as libc::c_int + 3 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (14 as libc::c_int - 36 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                23 as libc::c_int,
            ),
        );
    a = b
        .wrapping_add(
            leftrotate(
                a
                    .wrapping_add(b ^ c ^ d)
                    .wrapping_add(k[(40 as libc::c_int + 0 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (5 as libc::c_int - 40 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                4 as libc::c_int,
            ),
        );
    d = a
        .wrapping_add(
            leftrotate(
                d
                    .wrapping_add(a ^ b ^ c)
                    .wrapping_add(k[(40 as libc::c_int + 1 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (8 as libc::c_int - 40 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                11 as libc::c_int,
            ),
        );
    c = d
        .wrapping_add(
            leftrotate(
                c
                    .wrapping_add(d ^ a ^ b)
                    .wrapping_add(k[(40 as libc::c_int + 2 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (11 as libc::c_int - 40 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                16 as libc::c_int,
            ),
        );
    b = c
        .wrapping_add(
            leftrotate(
                b
                    .wrapping_add(c ^ d ^ a)
                    .wrapping_add(k[(40 as libc::c_int + 3 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (14 as libc::c_int - 40 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                23 as libc::c_int,
            ),
        );
    a = b
        .wrapping_add(
            leftrotate(
                a
                    .wrapping_add(b ^ c ^ d)
                    .wrapping_add(k[(44 as libc::c_int + 0 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (5 as libc::c_int - 44 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                4 as libc::c_int,
            ),
        );
    d = a
        .wrapping_add(
            leftrotate(
                d
                    .wrapping_add(a ^ b ^ c)
                    .wrapping_add(k[(44 as libc::c_int + 1 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (8 as libc::c_int - 44 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                11 as libc::c_int,
            ),
        );
    c = d
        .wrapping_add(
            leftrotate(
                c
                    .wrapping_add(d ^ a ^ b)
                    .wrapping_add(k[(44 as libc::c_int + 2 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (11 as libc::c_int - 44 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                16 as libc::c_int,
            ),
        );
    b = c
        .wrapping_add(
            leftrotate(
                b
                    .wrapping_add(c ^ d ^ a)
                    .wrapping_add(k[(44 as libc::c_int + 3 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (14 as libc::c_int - 44 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                23 as libc::c_int,
            ),
        );
    a = b
        .wrapping_add(
            leftrotate(
                a
                    .wrapping_add(c ^ (b | !d))
                    .wrapping_add(k[(48 as libc::c_int + 0 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (0 as libc::c_int - 48 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                6 as libc::c_int,
            ),
        );
    d = a
        .wrapping_add(
            leftrotate(
                d
                    .wrapping_add(b ^ (a | !c))
                    .wrapping_add(k[(48 as libc::c_int + 1 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (7 as libc::c_int - 48 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                10 as libc::c_int,
            ),
        );
    c = d
        .wrapping_add(
            leftrotate(
                c
                    .wrapping_add(a ^ (d | !b))
                    .wrapping_add(k[(48 as libc::c_int + 2 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (14 as libc::c_int - 48 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                15 as libc::c_int,
            ),
        );
    b = c
        .wrapping_add(
            leftrotate(
                b
                    .wrapping_add(d ^ (c | !a))
                    .wrapping_add(k[(48 as libc::c_int + 3 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (5 as libc::c_int - 48 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                21 as libc::c_int,
            ),
        );
    a = b
        .wrapping_add(
            leftrotate(
                a
                    .wrapping_add(c ^ (b | !d))
                    .wrapping_add(k[(52 as libc::c_int + 0 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (0 as libc::c_int - 52 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                6 as libc::c_int,
            ),
        );
    d = a
        .wrapping_add(
            leftrotate(
                d
                    .wrapping_add(b ^ (a | !c))
                    .wrapping_add(k[(52 as libc::c_int + 1 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (7 as libc::c_int - 52 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                10 as libc::c_int,
            ),
        );
    c = d
        .wrapping_add(
            leftrotate(
                c
                    .wrapping_add(a ^ (d | !b))
                    .wrapping_add(k[(52 as libc::c_int + 2 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (14 as libc::c_int - 52 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                15 as libc::c_int,
            ),
        );
    b = c
        .wrapping_add(
            leftrotate(
                b
                    .wrapping_add(d ^ (c | !a))
                    .wrapping_add(k[(52 as libc::c_int + 3 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (5 as libc::c_int - 52 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                21 as libc::c_int,
            ),
        );
    a = b
        .wrapping_add(
            leftrotate(
                a
                    .wrapping_add(c ^ (b | !d))
                    .wrapping_add(k[(56 as libc::c_int + 0 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (0 as libc::c_int - 56 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                6 as libc::c_int,
            ),
        );
    d = a
        .wrapping_add(
            leftrotate(
                d
                    .wrapping_add(b ^ (a | !c))
                    .wrapping_add(k[(56 as libc::c_int + 1 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (7 as libc::c_int - 56 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                10 as libc::c_int,
            ),
        );
    c = d
        .wrapping_add(
            leftrotate(
                c
                    .wrapping_add(a ^ (d | !b))
                    .wrapping_add(k[(56 as libc::c_int + 2 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (14 as libc::c_int - 56 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                15 as libc::c_int,
            ),
        );
    b = c
        .wrapping_add(
            leftrotate(
                b
                    .wrapping_add(d ^ (c | !a))
                    .wrapping_add(k[(56 as libc::c_int + 3 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (5 as libc::c_int - 56 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                21 as libc::c_int,
            ),
        );
    a = b
        .wrapping_add(
            leftrotate(
                a
                    .wrapping_add(c ^ (b | !d))
                    .wrapping_add(k[(60 as libc::c_int + 0 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (0 as libc::c_int - 60 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                6 as libc::c_int,
            ),
        );
    d = a
        .wrapping_add(
            leftrotate(
                d
                    .wrapping_add(b ^ (a | !c))
                    .wrapping_add(k[(60 as libc::c_int + 1 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (7 as libc::c_int - 60 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                10 as libc::c_int,
            ),
        );
    c = d
        .wrapping_add(
            leftrotate(
                c
                    .wrapping_add(a ^ (d | !b))
                    .wrapping_add(k[(60 as libc::c_int + 2 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (14 as libc::c_int - 60 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                15 as libc::c_int,
            ),
        );
    b = c
        .wrapping_add(
            leftrotate(
                b
                    .wrapping_add(d ^ (c | !a))
                    .wrapping_add(k[(60 as libc::c_int + 3 as libc::c_int) as usize])
                    .wrapping_add(
                        *data
                            .offset(
                                (5 as libc::c_int - 60 as libc::c_int & 15 as libc::c_int)
                                    as isize,
                            ),
                    ),
                21 as libc::c_int,
            ),
        );
    (*md5)
        .abcd[0 as libc::c_int
        as usize] = ((*md5).abcd[0 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(a) as uint32_t as uint32_t;
    (*md5)
        .abcd[1 as libc::c_int
        as usize] = ((*md5).abcd[1 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(b) as uint32_t as uint32_t;
    (*md5)
        .abcd[2 as libc::c_int
        as usize] = ((*md5).abcd[2 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(c) as uint32_t as uint32_t;
    (*md5)
        .abcd[3 as libc::c_int
        as usize] = ((*md5).abcd[3 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(d) as uint32_t as uint32_t;
}
unsafe extern "C" fn md5_update(
    md5: *mut MD5Context,
    mut data: *const uint8_t,
    mut len: libc::c_uint,
) {
    if len == 0 {
        return;
    }
    if (*md5).len & 63 as libc::c_int as libc::c_ulong != 0 {
        let tmp: libc::c_uint = umin(
            len,
            (64 as libc::c_int as libc::c_ulong)
                .wrapping_sub((*md5).len & 63 as libc::c_int as libc::c_ulong)
                as libc::c_uint,
        );
        memcpy(
            &mut *((*md5).c2rust_unnamed.data)
                .as_mut_ptr()
                .offset(((*md5).len & 63 as libc::c_int as libc::c_ulong) as isize)
                as *mut uint8_t as *mut libc::c_void,
            data as *const libc::c_void,
            tmp as libc::c_ulong,
        );
        len = len.wrapping_sub(tmp);
        data = data.offset(tmp as isize);
        (*md5)
            .len = ((*md5).len as libc::c_ulong).wrapping_add(tmp as libc::c_ulong)
            as uint64_t as uint64_t;
        if (*md5).len & 63 as libc::c_int as libc::c_ulong == 0 {
            md5_body(md5, ((*md5).c2rust_unnamed.data32).as_mut_ptr());
        }
    }
    while len >= 64 as libc::c_int as libc::c_uint {
        memcpy(
            ((*md5).c2rust_unnamed.data).as_mut_ptr() as *mut libc::c_void,
            data as *const libc::c_void,
            64 as libc::c_int as libc::c_ulong,
        );
        md5_body(md5, ((*md5).c2rust_unnamed.data32).as_mut_ptr());
        (*md5)
            .len = ((*md5).len as libc::c_ulong)
            .wrapping_add(64 as libc::c_int as libc::c_ulong) as uint64_t as uint64_t;
        data = data.offset(64 as libc::c_int as isize);
        len = len.wrapping_sub(64 as libc::c_int as libc::c_uint);
    }
    if len != 0 {
        memcpy(
            ((*md5).c2rust_unnamed.data).as_mut_ptr() as *mut libc::c_void,
            data as *const libc::c_void,
            len as libc::c_ulong,
        );
        (*md5)
            .len = ((*md5).len as libc::c_ulong).wrapping_add(len as libc::c_ulong)
            as uint64_t as uint64_t;
    }
}
unsafe extern "C" fn md5_write(
    md5: *mut MD5Context,
    p: *mut Dav1dPicture,
) -> libc::c_int {
    let hbd: libc::c_int = ((*p).p.bpc > 8 as libc::c_int) as libc::c_int;
    let w: libc::c_int = (*p).p.w;
    let h: libc::c_int = (*p).p.h;
    let mut yptr: *mut uint8_t = (*p).data[0 as libc::c_int as usize] as *mut uint8_t;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < h {
        md5_update(md5, yptr, (w << hbd) as libc::c_uint);
        yptr = yptr.offset((*p).stride[0 as libc::c_int as usize] as isize);
        y += 1;
    }
    if (*p).p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint
    {
        let ss_ver: libc::c_int = ((*p).p.layout as libc::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
        let ss_hor: libc::c_int = ((*p).p.layout as libc::c_uint
            != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
        let cw: libc::c_int = w + ss_hor >> ss_hor;
        let ch: libc::c_int = h + ss_ver >> ss_ver;
        let mut pl: libc::c_int = 1 as libc::c_int;
        while pl <= 2 as libc::c_int {
            let mut uvptr: *mut uint8_t = (*p).data[pl as usize] as *mut uint8_t;
            let mut y_0: libc::c_int = 0 as libc::c_int;
            while y_0 < ch {
                md5_update(md5, uvptr, (cw << hbd) as libc::c_uint);
                uvptr = uvptr.offset((*p).stride[1 as libc::c_int as usize] as isize);
                y_0 += 1;
            }
            pl += 1;
        }
    }
    dav1d_picture_unref(p);
    return 0 as libc::c_int;
}
unsafe extern "C" fn md5_finish(md5: *mut MD5Context) {
    static mut bit: [uint8_t; 2] = [
        0x80 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    ];
    let len: uint64_t = (*md5).len << 3 as libc::c_int;
    md5_update(
        md5,
        &*bit.as_ptr().offset(0 as libc::c_int as isize),
        1 as libc::c_int as libc::c_uint,
    );
    while (*md5).len & 63 as libc::c_int as libc::c_ulong
        != 56 as libc::c_int as libc::c_ulong
    {
        md5_update(
            md5,
            &*bit.as_ptr().offset(1 as libc::c_int as isize),
            1 as libc::c_int as libc::c_uint,
        );
    }
    md5_update(
        md5,
        &len as *const uint64_t as *const uint8_t,
        8 as libc::c_int as libc::c_uint,
    );
}
unsafe extern "C" fn md5_close(md5: *mut MD5Context) {
    md5_finish(md5);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        fprintf(
            (*md5).f,
            b"%2.2x%2.2x%2.2x%2.2x\0" as *const u8 as *const libc::c_char,
            (*md5).abcd[i as usize] & 0xff as libc::c_int as libc::c_uint,
            (*md5).abcd[i as usize] >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint,
            (*md5).abcd[i as usize] >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint,
            (*md5).abcd[i as usize] >> 24 as libc::c_int,
        );
        i += 1;
    }
    fprintf((*md5).f, b"\n\0" as *const u8 as *const libc::c_char);
    if (*md5).f != stdout {
        fclose((*md5).f);
    }
}
unsafe extern "C" fn md5_verify(
    md5: *mut MD5Context,
    mut md5_str: *const libc::c_char,
) -> libc::c_int {
    md5_finish(md5);
    if strlen(md5_str) < 32 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    let mut abcd: [uint32_t; 4] = [0 as libc::c_int as uint32_t, 0, 0, 0];
    let mut t: [libc::c_char; 3] = [0 as libc::c_int as libc::c_char, 0, 0];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 32 as libc::c_int {
            let mut ignore: *mut libc::c_char = 0 as *mut libc::c_char;
            memcpy(
                t.as_mut_ptr() as *mut libc::c_void,
                md5_str as *const libc::c_void,
                2 as libc::c_int as libc::c_ulong,
            );
            md5_str = md5_str.offset(2 as libc::c_int as isize);
            abcd[i as usize]
                |= (strtoul(t.as_mut_ptr(), &mut ignore, 16 as libc::c_int) as uint32_t)
                    << j;
            j += 8 as libc::c_int;
        }
        i += 1;
    }
    return (memcmp(
        abcd.as_mut_ptr() as *const libc::c_void,
        ((*md5).abcd).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint32_t; 4]>() as libc::c_ulong,
    ) != 0) as libc::c_int;
}
#[no_mangle]
pub static mut md5_muxer: Muxer = unsafe {
    {
        let mut init = Muxer {
            priv_data_size: ::core::mem::size_of::<MD5Context>() as libc::c_ulong
                as libc::c_int,
            name: b"md5\0" as *const u8 as *const libc::c_char,
            extension: b"md5\0" as *const u8 as *const libc::c_char,
            write_header: Some(
                md5_open
                    as unsafe extern "C" fn(
                        *mut MD5Context,
                        *const libc::c_char,
                        *const Dav1dPictureParameters,
                        *const libc::c_uint,
                    ) -> libc::c_int,
            ),
            write_picture: Some(
                md5_write
                    as unsafe extern "C" fn(
                        *mut MD5Context,
                        *mut Dav1dPicture,
                    ) -> libc::c_int,
            ),
            write_trailer: Some(
                md5_close as unsafe extern "C" fn(*mut MD5Context) -> (),
            ),
            verify: Some(
                md5_verify
                    as unsafe extern "C" fn(
                        *mut MD5Context,
                        *const libc::c_char,
                    ) -> libc::c_int,
            ),
        };
        init
    }
};
