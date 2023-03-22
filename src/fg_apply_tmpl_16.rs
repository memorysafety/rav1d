use ::libc;
extern "C" {
    pub type Dav1dRef;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
use crate::include::stddef::ptrdiff_t;
use crate::include::stddef::size_t;
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
pub type pixel = uint16_t;
pub type entry = int16_t;
pub type generate_grain_y_fn = Option::<
    unsafe extern "C" fn(*mut [entry; 82], *const Dav1dFilmGrainData, libc::c_int) -> (),
>;
pub type generate_grain_uv_fn = Option::<
    unsafe extern "C" fn(
        *mut [entry; 82],
        *const [entry; 82],
        *const Dav1dFilmGrainData,
        intptr_t,
        libc::c_int,
    ) -> (),
>;
pub type fgy_32x32xn_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        *const pixel,
        ptrdiff_t,
        *const Dav1dFilmGrainData,
        size_t,
        *const uint8_t,
        *const [entry; 82],
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type fguv_32x32xn_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        *const pixel,
        ptrdiff_t,
        *const Dav1dFilmGrainData,
        size_t,
        *const uint8_t,
        *const [entry; 82],
        libc::c_int,
        libc::c_int,
        *const pixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFilmGrainDSPContext {
    pub generate_grain_y: generate_grain_y_fn,
    pub generate_grain_uv: [generate_grain_uv_fn; 3],
    pub fgy_32x32xn: fgy_32x32xn_fn,
    pub fguv_32x32xn: [fguv_32x32xn_fn; 3],
}
#[inline]
unsafe extern "C" fn imin(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 as libc::c_int as libc::c_long != 0 {
        unreachable!();
    }
    return x >> 1 as libc::c_int;
}
unsafe extern "C" fn generate_scaling(
    bitdepth: libc::c_int,
    mut points: *const [uint8_t; 2],
    num: libc::c_int,
    mut scaling: *mut uint8_t,
) {
    if !(bitdepth > 8 as libc::c_int) {
        unreachable!();
    }
    let shift_x: libc::c_int = bitdepth - 8 as libc::c_int;
    let scaling_size: libc::c_int = (1 as libc::c_int) << bitdepth;
    if num == 0 as libc::c_int {
        memset(
            scaling as *mut libc::c_void,
            0 as libc::c_int,
            scaling_size as libc::c_ulong,
        );
        return;
    }
    memset(
        scaling as *mut libc::c_void,
        (*points.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
            as libc::c_int,
        (((*points.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
            as libc::c_int) << shift_x) as libc::c_ulong,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < num - 1 as libc::c_int {
        let bx: libc::c_int = (*points.offset(i as isize))[0 as libc::c_int as usize]
            as libc::c_int;
        let by: libc::c_int = (*points.offset(i as isize))[1 as libc::c_int as usize]
            as libc::c_int;
        let ex: libc::c_int = (*points
            .offset((i + 1 as libc::c_int) as isize))[0 as libc::c_int as usize]
            as libc::c_int;
        let ey: libc::c_int = (*points
            .offset((i + 1 as libc::c_int) as isize))[1 as libc::c_int as usize]
            as libc::c_int;
        let dx: libc::c_int = ex - bx;
        let dy: libc::c_int = ey - by;
        if !(dx > 0 as libc::c_int) {
            unreachable!();
        }
        let delta: libc::c_int = dy
            * ((0x10000 as libc::c_int + (dx >> 1 as libc::c_int)) / dx);
        let mut x: libc::c_int = 0 as libc::c_int;
        let mut d: libc::c_int = 0x8000 as libc::c_int;
        while x < dx {
            *scaling
                .offset(
                    (bx + x << shift_x) as isize,
                ) = (by + (d >> 16 as libc::c_int)) as uint8_t;
            d += delta;
            x += 1;
        }
        i += 1;
    }
    let n: libc::c_int = ((*points
        .offset((num - 1 as libc::c_int) as isize))[0 as libc::c_int as usize]
        as libc::c_int) << shift_x;
    memset(
        &mut *scaling.offset(n as isize) as *mut uint8_t as *mut libc::c_void,
        (*points.offset((num - 1 as libc::c_int) as isize))[1 as libc::c_int as usize]
            as libc::c_int,
        (scaling_size - n) as libc::c_ulong,
    );
    let pad: libc::c_int = (1 as libc::c_int) << shift_x;
    let rnd: libc::c_int = pad >> 1 as libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < num - 1 as libc::c_int {
        let bx_0: libc::c_int = ((*points
            .offset(i_0 as isize))[0 as libc::c_int as usize] as libc::c_int) << shift_x;
        let ex_0: libc::c_int = ((*points
            .offset((i_0 + 1 as libc::c_int) as isize))[0 as libc::c_int as usize]
            as libc::c_int) << shift_x;
        let dx_0: libc::c_int = ex_0 - bx_0;
        let mut x_0: libc::c_int = 0 as libc::c_int;
        while x_0 < dx_0 {
            let range: libc::c_int = *scaling.offset((bx_0 + x_0 + pad) as isize)
                as libc::c_int - *scaling.offset((bx_0 + x_0) as isize) as libc::c_int;
            let mut n_0: libc::c_int = 1 as libc::c_int;
            let mut r: libc::c_int = rnd;
            while n_0 < pad {
                r += range;
                *scaling
                    .offset(
                        (bx_0 + x_0 + n_0) as isize,
                    ) = (*scaling.offset((bx_0 + x_0) as isize) as libc::c_int
                    + (r >> shift_x)) as uint8_t;
                n_0 += 1;
            }
            x_0 += pad;
        }
        i_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_prep_grain_16bpc(
    dsp: *const Dav1dFilmGrainDSPContext,
    out: *mut Dav1dPicture,
    in_0: *const Dav1dPicture,
    mut scaling: *mut [uint8_t; 4096],
    mut grain_lut: *mut [[entry; 82]; 74],
) {
    let data: *const Dav1dFilmGrainData = &mut (*(*out).frame_hdr).film_grain.data;
    let bitdepth_max: libc::c_int = ((1 as libc::c_int) << (*out).p.bpc)
        - 1 as libc::c_int;
    ((*dsp).generate_grain_y)
        .expect(
            "non-null function pointer",
        )(
        (*grain_lut.offset(0 as libc::c_int as isize)).as_mut_ptr(),
        data,
        bitdepth_max,
    );
    if (*data).num_uv_points[0 as libc::c_int as usize] != 0
        || (*data).chroma_scaling_from_luma != 0
    {
        ((*dsp)
            .generate_grain_uv[((*in_0).p.layout as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as usize])
            .expect(
                "non-null function pointer",
            )(
            (*grain_lut.offset(1 as libc::c_int as isize)).as_mut_ptr(),
            (*grain_lut.offset(0 as libc::c_int as isize)).as_mut_ptr()
                as *const [entry; 82],
            data,
            0 as libc::c_int as intptr_t,
            bitdepth_max,
        );
    }
    if (*data).num_uv_points[1 as libc::c_int as usize] != 0
        || (*data).chroma_scaling_from_luma != 0
    {
        ((*dsp)
            .generate_grain_uv[((*in_0).p.layout as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as usize])
            .expect(
                "non-null function pointer",
            )(
            (*grain_lut.offset(2 as libc::c_int as isize)).as_mut_ptr(),
            (*grain_lut.offset(0 as libc::c_int as isize)).as_mut_ptr()
                as *const [entry; 82],
            data,
            1 as libc::c_int as intptr_t,
            bitdepth_max,
        );
    }
    if (*data).num_y_points != 0 || (*data).chroma_scaling_from_luma != 0 {
        generate_scaling(
            (*in_0).p.bpc,
            ((*data).y_points).as_ptr(),
            (*data).num_y_points,
            (*scaling.offset(0 as libc::c_int as isize)).as_mut_ptr(),
        );
    }
    if (*data).num_uv_points[0 as libc::c_int as usize] != 0 {
        generate_scaling(
            (*in_0).p.bpc,
            ((*data).uv_points[0 as libc::c_int as usize]).as_ptr(),
            (*data).num_uv_points[0 as libc::c_int as usize],
            (*scaling.offset(1 as libc::c_int as isize)).as_mut_ptr(),
        );
    }
    if (*data).num_uv_points[1 as libc::c_int as usize] != 0 {
        generate_scaling(
            (*in_0).p.bpc,
            ((*data).uv_points[1 as libc::c_int as usize]).as_ptr(),
            (*data).num_uv_points[1 as libc::c_int as usize],
            (*scaling.offset(2 as libc::c_int as isize)).as_mut_ptr(),
        );
    }
    if !((*out).stride[0 as libc::c_int as usize]
        == (*in_0).stride[0 as libc::c_int as usize])
    {
        unreachable!();
    }
    if (*data).num_y_points == 0 {
        let stride: ptrdiff_t = (*out).stride[0 as libc::c_int as usize];
        let sz: ptrdiff_t = (*out).p.h as libc::c_long * stride;
        if sz < 0 as libc::c_int as libc::c_long {
            memcpy(
                ((*out).data[0 as libc::c_int as usize] as *mut uint8_t)
                    .offset(sz as isize)
                    .offset(-(stride as isize)) as *mut libc::c_void,
                ((*in_0).data[0 as libc::c_int as usize] as *mut uint8_t)
                    .offset(sz as isize)
                    .offset(-(stride as isize)) as *const libc::c_void,
                -sz as libc::c_ulong,
            );
        } else {
            memcpy(
                (*out).data[0 as libc::c_int as usize],
                (*in_0).data[0 as libc::c_int as usize],
                sz as libc::c_ulong,
            );
        }
    }
    if (*in_0).p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint
        && (*data).chroma_scaling_from_luma == 0
    {
        if !((*out).stride[1 as libc::c_int as usize]
            == (*in_0).stride[1 as libc::c_int as usize])
        {
            unreachable!();
        }
        let ss_ver: libc::c_int = ((*in_0).p.layout as libc::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
        let stride_0: ptrdiff_t = (*out).stride[1 as libc::c_int as usize];
        let sz_0: ptrdiff_t = ((*out).p.h + ss_ver >> ss_ver) as libc::c_long * stride_0;
        if sz_0 < 0 as libc::c_int as libc::c_long {
            if (*data).num_uv_points[0 as libc::c_int as usize] == 0 {
                memcpy(
                    ((*out).data[1 as libc::c_int as usize] as *mut uint8_t)
                        .offset(sz_0 as isize)
                        .offset(-(stride_0 as isize)) as *mut libc::c_void,
                    ((*in_0).data[1 as libc::c_int as usize] as *mut uint8_t)
                        .offset(sz_0 as isize)
                        .offset(-(stride_0 as isize)) as *const libc::c_void,
                    -sz_0 as libc::c_ulong,
                );
            }
            if (*data).num_uv_points[1 as libc::c_int as usize] == 0 {
                memcpy(
                    ((*out).data[2 as libc::c_int as usize] as *mut uint8_t)
                        .offset(sz_0 as isize)
                        .offset(-(stride_0 as isize)) as *mut libc::c_void,
                    ((*in_0).data[2 as libc::c_int as usize] as *mut uint8_t)
                        .offset(sz_0 as isize)
                        .offset(-(stride_0 as isize)) as *const libc::c_void,
                    -sz_0 as libc::c_ulong,
                );
            }
        } else {
            if (*data).num_uv_points[0 as libc::c_int as usize] == 0 {
                memcpy(
                    (*out).data[1 as libc::c_int as usize],
                    (*in_0).data[1 as libc::c_int as usize],
                    sz_0 as libc::c_ulong,
                );
            }
            if (*data).num_uv_points[1 as libc::c_int as usize] == 0 {
                memcpy(
                    (*out).data[2 as libc::c_int as usize],
                    (*in_0).data[2 as libc::c_int as usize],
                    sz_0 as libc::c_ulong,
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_apply_grain_row_16bpc(
    dsp: *const Dav1dFilmGrainDSPContext,
    out: *mut Dav1dPicture,
    in_0: *const Dav1dPicture,
    mut scaling: *const [uint8_t; 4096],
    mut grain_lut: *const [[entry; 82]; 74],
    row: libc::c_int,
) {
    let data: *const Dav1dFilmGrainData = &mut (*(*out).frame_hdr).film_grain.data;
    let ss_y: libc::c_int = ((*in_0).p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_x: libc::c_int = ((*in_0).p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
    let cpw: libc::c_int = (*out).p.w + ss_x >> ss_x;
    let is_id: libc::c_int = ((*(*out).seq_hdr).mtrx as libc::c_uint
        == DAV1D_MC_IDENTITY as libc::c_int as libc::c_uint) as libc::c_int;
    let luma_src: *mut pixel = ((*in_0).data[0 as libc::c_int as usize] as *mut pixel)
        .offset(
            ((row * 32 as libc::c_int) as libc::c_long
                * PXSTRIDE((*in_0).stride[0 as libc::c_int as usize])) as isize,
        );
    let bitdepth_max: libc::c_int = ((1 as libc::c_int) << (*out).p.bpc)
        - 1 as libc::c_int;
    if (*data).num_y_points != 0 {
        let bh: libc::c_int = imin(
            (*out).p.h - row * 32 as libc::c_int,
            32 as libc::c_int,
        );
        ((*dsp).fgy_32x32xn)
            .expect(
                "non-null function pointer",
            )(
            ((*out).data[0 as libc::c_int as usize] as *mut pixel)
                .offset(
                    ((row * 32 as libc::c_int) as libc::c_long
                        * PXSTRIDE((*out).stride[0 as libc::c_int as usize])) as isize,
                ),
            luma_src,
            (*out).stride[0 as libc::c_int as usize],
            data,
            (*out).p.w as size_t,
            (*scaling.offset(0 as libc::c_int as isize)).as_ptr(),
            (*grain_lut.offset(0 as libc::c_int as isize)).as_ptr(),
            bh,
            row,
            bitdepth_max,
        );
    }
    if (*data).num_uv_points[0 as libc::c_int as usize] == 0
        && (*data).num_uv_points[1 as libc::c_int as usize] == 0
        && (*data).chroma_scaling_from_luma == 0
    {
        return;
    }
    let bh_0: libc::c_int = imin((*out).p.h - row * 32 as libc::c_int, 32 as libc::c_int)
        + ss_y >> ss_y;
    if (*out).p.w & ss_x != 0 {
        let mut ptr: *mut pixel = luma_src;
        let mut y: libc::c_int = 0 as libc::c_int;
        while y < bh_0 {
            *ptr
                .offset(
                    (*out).p.w as isize,
                ) = *ptr.offset(((*out).p.w - 1 as libc::c_int) as isize);
            ptr = ptr
                .offset(
                    (PXSTRIDE((*in_0).stride[0 as libc::c_int as usize]) << ss_y)
                        as isize,
                );
            y += 1;
        }
    }
    let uv_off: ptrdiff_t = (row * 32 as libc::c_int) as libc::c_long
        * PXSTRIDE((*out).stride[1 as libc::c_int as usize]) >> ss_y;
    if (*data).chroma_scaling_from_luma != 0 {
        let mut pl: libc::c_int = 0 as libc::c_int;
        while pl < 2 as libc::c_int {
            ((*dsp)
                .fguv_32x32xn[((*in_0).p.layout as libc::c_uint)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as usize])
                .expect(
                    "non-null function pointer",
                )(
                ((*out).data[(1 as libc::c_int + pl) as usize] as *mut pixel)
                    .offset(uv_off as isize),
                ((*in_0).data[(1 as libc::c_int + pl) as usize] as *const pixel)
                    .offset(uv_off as isize),
                (*in_0).stride[1 as libc::c_int as usize],
                data,
                cpw as size_t,
                (*scaling.offset(0 as libc::c_int as isize)).as_ptr(),
                (*grain_lut.offset((1 as libc::c_int + pl) as isize)).as_ptr(),
                bh_0,
                row,
                luma_src,
                (*in_0).stride[0 as libc::c_int as usize],
                pl,
                is_id,
                bitdepth_max,
            );
            pl += 1;
        }
    } else {
        let mut pl_0: libc::c_int = 0 as libc::c_int;
        while pl_0 < 2 as libc::c_int {
            if (*data).num_uv_points[pl_0 as usize] != 0 {
                ((*dsp)
                    .fguv_32x32xn[((*in_0).p.layout as libc::c_uint)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as usize])
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*out).data[(1 as libc::c_int + pl_0) as usize] as *mut pixel)
                        .offset(uv_off as isize),
                    ((*in_0).data[(1 as libc::c_int + pl_0) as usize] as *const pixel)
                        .offset(uv_off as isize),
                    (*in_0).stride[1 as libc::c_int as usize],
                    data,
                    cpw as size_t,
                    (*scaling.offset((1 as libc::c_int + pl_0) as isize)).as_ptr(),
                    (*grain_lut.offset((1 as libc::c_int + pl_0) as isize)).as_ptr(),
                    bh_0,
                    row,
                    luma_src,
                    (*in_0).stride[0 as libc::c_int as usize],
                    pl_0,
                    is_id,
                    bitdepth_max,
                );
            }
            pl_0 += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_apply_grain_16bpc(
    dsp: *const Dav1dFilmGrainDSPContext,
    out: *mut Dav1dPicture,
    in_0: *const Dav1dPicture,
) {
    let mut grain_lut: [[[entry; 82]; 74]; 3] = [[[0; 82]; 74]; 3];
    let mut scaling: [[uint8_t; 4096]; 3] = [[0; 4096]; 3];
    let rows: libc::c_int = (*out).p.h + 31 as libc::c_int >> 5 as libc::c_int;
    dav1d_prep_grain_16bpc(dsp, out, in_0, scaling.as_mut_ptr(), grain_lut.as_mut_ptr());
    let mut row: libc::c_int = 0 as libc::c_int;
    while row < rows {
        dav1d_apply_grain_row_16bpc(
            dsp,
            out,
            in_0,
            scaling.as_mut_ptr() as *const [uint8_t; 4096],
            grain_lut.as_mut_ptr() as *const [[entry; 82]; 74],
            row,
        );
        row += 1;
    }
}
