use ::libc;
extern "C" {
    fn free(_: *mut libc::c_void);
    fn posix_memalign(
        __memptr: *mut *mut libc::c_void,
        __alignment: size_t,
        __size: size_t,
    ) -> libc::c_int;
    fn abs(_: libc::c_int) -> libc::c_int;
    static dav1d_block_dimensions: [[uint8_t; 4]; 22];
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type ptrdiff_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav1dWarpedMotionParams {
    pub type_0: Dav1dWarpedMotionType,
    pub matrix: [int32_t; 6],
    pub u: C2RustUnnamed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed {
    pub p: C2RustUnnamed_0,
    pub abcd: [int16_t; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav1dSequenceHeaderOperatingParameterInfo {
    pub decoder_buffer_delay: libc::c_int,
    pub encoder_buffer_delay: libc::c_int,
    pub low_delay_mode: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav1dSequenceHeaderOperatingPoint {
    pub major_level: libc::c_int,
    pub minor_level: libc::c_int,
    pub initial_display_delay: libc::c_int,
    pub idc: libc::c_int,
    pub tier: libc::c_int,
    pub decoder_model_param_present: libc::c_int,
    pub display_model_param_present: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav1dSegmentationDataSet {
    pub d: [Dav1dSegmentationData; 8],
    pub preskip: libc::c_int,
    pub last_active_segid: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav1dLoopfilterModeRefDeltas {
    pub mode_delta: [libc::c_int; 2],
    pub ref_delta: [libc::c_int; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub type_0: [Dav1dRestorationType; 3],
    pub unit_size: [libc::c_int; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_2 {
    pub damping: libc::c_int,
    pub n_bits: libc::c_int,
    pub y_strength: [libc::c_int; 8],
    pub uv_strength: [libc::c_int; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
    pub level_y: [libc::c_int; 2],
    pub level_u: libc::c_int,
    pub level_v: libc::c_int,
    pub mode_ref_delta_enabled: libc::c_int,
    pub mode_ref_delta_update: libc::c_int,
    pub mode_ref_deltas: Dav1dLoopfilterModeRefDeltas,
    pub sharpness: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_4 {
    pub q: C2RustUnnamed_6,
    pub lf: C2RustUnnamed_5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_5 {
    pub present: libc::c_int,
    pub res_log2: libc::c_int,
    pub multi: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_6 {
    pub present: libc::c_int,
    pub res_log2: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_7 {
    pub enabled: libc::c_int,
    pub update_map: libc::c_int,
    pub temporal: libc::c_int,
    pub update_data: libc::c_int,
    pub seg_data: Dav1dSegmentationDataSet,
    pub lossless: [libc::c_int; 8],
    pub qidx: [libc::c_int; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_10 {
    pub width_scale_denominator: libc::c_int,
    pub enabled: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav1dFrameHeaderOperatingPoint {
    pub buffer_removal_time: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_11 {
    pub data: Dav1dFilmGrainData,
    pub present: libc::c_int,
    pub update: libc::c_int,
}
pub type BlockSize = libc::c_uint;
pub const N_BS_SIZES: BlockSize = 22;
pub const BS_4x4: BlockSize = 21;
pub const BS_4x8: BlockSize = 20;
pub const BS_4x16: BlockSize = 19;
pub const BS_8x4: BlockSize = 18;
pub const BS_8x8: BlockSize = 17;
pub const BS_8x16: BlockSize = 16;
pub const BS_8x32: BlockSize = 15;
pub const BS_16x4: BlockSize = 14;
pub const BS_16x8: BlockSize = 13;
pub const BS_16x16: BlockSize = 12;
pub const BS_16x32: BlockSize = 11;
pub const BS_16x64: BlockSize = 10;
pub const BS_32x8: BlockSize = 9;
pub const BS_32x16: BlockSize = 8;
pub const BS_32x32: BlockSize = 7;
pub const BS_32x64: BlockSize = 6;
pub const BS_64x16: BlockSize = 5;
pub const BS_64x32: BlockSize = 4;
pub const BS_64x64: BlockSize = 3;
pub const BS_64x128: BlockSize = 2;
pub const BS_128x64: BlockSize = 1;
pub const BS_128x128: BlockSize = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub union mv {
    pub c2rust_unnamed: C2RustUnnamed_12,
    pub n: uint32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_12 {
    pub y: int16_t,
    pub x: int16_t,
}
pub type EdgeFlags = libc::c_uint;
pub const EDGE_I420_LEFT_HAS_BOTTOM: EdgeFlags = 32;
pub const EDGE_I422_LEFT_HAS_BOTTOM: EdgeFlags = 16;
pub const EDGE_I444_LEFT_HAS_BOTTOM: EdgeFlags = 8;
pub const EDGE_I420_TOP_HAS_RIGHT: EdgeFlags = 4;
pub const EDGE_I422_TOP_HAS_RIGHT: EdgeFlags = 2;
pub const EDGE_I444_TOP_HAS_RIGHT: EdgeFlags = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct refmvs_temporal_block {
    pub mv: mv,
    pub ref_0: int8_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union refmvs_refpair {
    pub ref_0: [int8_t; 2],
    pub pair: uint16_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union refmvs_mvpair {
    pub mv: [mv; 2],
    pub n: uint64_t,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct refmvs_block {
    pub mv: refmvs_mvpair,
    pub ref_0: refmvs_refpair,
    pub bs: uint8_t,
    pub mf: uint8_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct refmvs_frame {
    pub frm_hdr: *const Dav1dFrameHeader,
    pub iw4: libc::c_int,
    pub ih4: libc::c_int,
    pub iw8: libc::c_int,
    pub ih8: libc::c_int,
    pub sbsz: libc::c_int,
    pub use_ref_frame_mvs: libc::c_int,
    pub sign_bias: [uint8_t; 7],
    pub mfmv_sign: [uint8_t; 7],
    pub pocdiff: [int8_t; 7],
    pub mfmv_ref: [uint8_t; 3],
    pub mfmv_ref2cur: [libc::c_int; 3],
    pub mfmv_ref2ref: [[libc::c_int; 7]; 3],
    pub n_mfmvs: libc::c_int,
    pub rp: *mut refmvs_temporal_block,
    pub rp_ref: *const *mut refmvs_temporal_block,
    pub rp_proj: *mut refmvs_temporal_block,
    pub rp_stride: ptrdiff_t,
    pub r: *mut refmvs_block,
    pub r_stride: ptrdiff_t,
    pub n_tile_rows: libc::c_int,
    pub n_tile_threads: libc::c_int,
    pub n_frame_threads: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct refmvs_tile {
    pub rf: *const refmvs_frame,
    pub r: [*mut refmvs_block; 37],
    pub rp_proj: *mut refmvs_temporal_block,
    pub tile_col: C2RustUnnamed_13,
    pub tile_row: C2RustUnnamed_13,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_13 {
    pub start: libc::c_int,
    pub end: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct refmvs_candidate {
    pub mv: refmvs_mvpair,
    pub weight: libc::c_int,
}
pub type splat_mv_fn = Option<
    unsafe extern "C" fn(
        *mut *mut refmvs_block,
        *const refmvs_block,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav1dRefmvsDSPContext {
    pub splat_mv: splat_mv_fn,
}
#[inline]
unsafe extern "C" fn imax(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn imin(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn iclip(v: libc::c_int, min: libc::c_int, max: libc::c_int) -> libc::c_int {
    return if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    };
}
#[inline]
unsafe extern "C" fn apply_sign(v: libc::c_int, s: libc::c_int) -> libc::c_int {
    return if s < 0i32 { -v } else { v };
}
#[inline]
unsafe extern "C" fn get_poc_diff(
    order_hint_n_bits: libc::c_int,
    poc0: libc::c_int,
    poc1: libc::c_int,
) -> libc::c_int {
    if order_hint_n_bits == 0 {
        return 0i32;
    }
    let mask: libc::c_int = (1i32) << order_hint_n_bits - 1i32;
    let diff: libc::c_int = poc0 - poc1;
    return (diff & mask - 1i32) - (diff & mask);
}
#[inline]
unsafe extern "C" fn fix_mv_precision(hdr: *const Dav1dFrameHeader, mv: *mut mv) {
    if (*hdr).force_integer_mv != 0 {
        fix_int_mv_precision(mv);
    } else if (*hdr).hp == 0 {
        (*mv).c2rust_unnamed.x = (((*mv).c2rust_unnamed.x as libc::c_int
            - ((*mv).c2rust_unnamed.x as libc::c_int >> 15i32))
            as libc::c_uint
            & !(1u32)) as int16_t;
        (*mv).c2rust_unnamed.y = (((*mv).c2rust_unnamed.y as libc::c_int
            - ((*mv).c2rust_unnamed.y as libc::c_int >> 15i32))
            as libc::c_uint
            & !(1u32)) as int16_t;
    }
}
#[inline]
unsafe extern "C" fn fix_int_mv_precision(mv: *mut mv) {
    (*mv).c2rust_unnamed.x = (((*mv).c2rust_unnamed.x as libc::c_int
        - ((*mv).c2rust_unnamed.x as libc::c_int >> 15i32)
        + 3i32) as libc::c_uint
        & !(7u32)) as int16_t;
    (*mv).c2rust_unnamed.y = (((*mv).c2rust_unnamed.y as libc::c_int
        - ((*mv).c2rust_unnamed.y as libc::c_int >> 15i32)
        + 3i32) as libc::c_uint
        & !(7u32)) as int16_t;
}
#[inline]
unsafe extern "C" fn get_gmv_2d(
    gmv: *const Dav1dWarpedMotionParams,
    bx4: libc::c_int,
    by4: libc::c_int,
    bw4: libc::c_int,
    bh4: libc::c_int,
    hdr: *const Dav1dFrameHeader,
) -> mv {
    match (*gmv).type_0 {
        2 => {
            if !((*gmv).matrix[5usize] == (*gmv).matrix[2usize]) {
                unreachable!();
            }
            if !((*gmv).matrix[4usize] == -(*gmv).matrix[3usize]) {
                unreachable!();
            }
        }
        1 => {
            let mut res_0: mv = mv {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_12 {
                        y: ((*gmv).matrix[0usize] >> 13i32) as int16_t,
                        x: ((*gmv).matrix[1usize] >> 13i32) as int16_t,
                    };
                    init
                },
            };
            if (*hdr).force_integer_mv != 0 {
                fix_int_mv_precision(&mut res_0);
            }
            return res_0;
        }
        0 => {
            return mv {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_12 { y: 0i16, x: 0i16 };
                    init
                },
            };
        }
        3 | _ => {}
    }
    let x: libc::c_int = bx4 * 4i32 + bw4 * 2i32 - 1i32;
    let y: libc::c_int = by4 * 4i32 + bh4 * 2i32 - 1i32;
    let xc: libc::c_int = ((*gmv).matrix[2usize] - ((1i32) << 16i32)) * x
        + (*gmv).matrix[3usize] * y
        + (*gmv).matrix[0usize];
    let yc: libc::c_int = ((*gmv).matrix[5usize] - ((1i32) << 16i32)) * y
        + (*gmv).matrix[4usize] * x
        + (*gmv).matrix[1usize];
    let shift: libc::c_int = 16i32 - (3i32 - ((*hdr).hp == 0) as libc::c_int);
    let round: libc::c_int = (1i32) << shift >> 1i32;
    let mut res: mv = mv {
        c2rust_unnamed: {
            let mut init = C2RustUnnamed_12 {
                y: apply_sign(
                    abs(yc) + round >> shift << ((*hdr).hp == 0) as libc::c_int,
                    yc,
                ) as int16_t,
                x: apply_sign(
                    abs(xc) + round >> shift << ((*hdr).hp == 0) as libc::c_int,
                    xc,
                ) as int16_t,
            };
            init
        },
    };
    if (*hdr).force_integer_mv != 0 {
        fix_int_mv_precision(&mut res);
    }
    return res;
}
#[inline]
unsafe extern "C" fn dav1d_freep_aligned(mut ptr: *mut libc::c_void) {
    let mut mem: *mut *mut libc::c_void = ptr as *mut *mut libc::c_void;
    if !(*mem).is_null() {
        dav1d_free_aligned(*mem);
        *mem = 0 as *mut libc::c_void;
    }
}
#[inline]
unsafe extern "C" fn dav1d_free_aligned(mut ptr: *mut libc::c_void) {
    free(ptr);
}
#[inline]
unsafe extern "C" fn dav1d_alloc_aligned(mut sz: size_t, mut align: size_t) -> *mut libc::c_void {
    if align & align.wrapping_sub(1u64) != 0 {
        unreachable!();
    }
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if posix_memalign(&mut ptr, align, sz) != 0 {
        return 0 as *mut libc::c_void;
    }
    return ptr;
}
unsafe extern "C" fn add_spatial_candidate(
    mvstack: *mut refmvs_candidate,
    cnt: *mut libc::c_int,
    weight: libc::c_int,
    b: *const refmvs_block,
    ref_0: refmvs_refpair,
    mut gmv: *const mv,
    have_newmv_match: *mut libc::c_int,
    have_refmv_match: *mut libc::c_int,
) {
    if (*b).mv.mv[0usize].n == 0x80008000u32 {
        return;
    }
    if ref_0.ref_0[1usize] as libc::c_int == -(1i32) {
        let mut n: libc::c_int = 0i32;
        while n < 2i32 {
            if (*b).ref_0.ref_0[n as usize] as libc::c_int == ref_0.ref_0[0usize] as libc::c_int {
                let cand_mv: mv = if (*b).mf as libc::c_int & 1i32 != 0
                    && (*gmv.offset(0isize)).n != 0x80008000u32
                {
                    *gmv.offset(0isize)
                } else {
                    (*b).mv.mv[n as usize]
                };
                *have_refmv_match = 1i32;
                *have_newmv_match |= (*b).mf as libc::c_int >> 1i32;
                let last: libc::c_int = *cnt;
                let mut m: libc::c_int = 0i32;
                while m < last {
                    if (*mvstack.offset(m as isize)).mv.mv[0usize].n == cand_mv.n {
                        (*mvstack.offset(m as isize)).weight += weight;
                        return;
                    }
                    m += 1;
                }
                if last < 8i32 {
                    (*mvstack.offset(last as isize)).mv.mv[0usize] = cand_mv;
                    (*mvstack.offset(last as isize)).weight = weight;
                    *cnt = last + 1i32;
                }
                return;
            }
            n += 1;
        }
    } else if (*b).ref_0.pair as libc::c_int == ref_0.pair as libc::c_int {
        let cand_mv_0: refmvs_mvpair = refmvs_mvpair {
            mv: [
                if (*b).mf as libc::c_int & 1i32 != 0 && (*gmv.offset(0isize)).n != 0x80008000u32 {
                    *gmv.offset(0isize)
                } else {
                    (*b).mv.mv[0usize]
                },
                if (*b).mf as libc::c_int & 1i32 != 0 && (*gmv.offset(1isize)).n != 0x80008000u32 {
                    *gmv.offset(1isize)
                } else {
                    (*b).mv.mv[1usize]
                },
            ],
        };
        *have_refmv_match = 1i32;
        *have_newmv_match |= (*b).mf as libc::c_int >> 1i32;
        let last_0: libc::c_int = *cnt;
        let mut n_0: libc::c_int = 0i32;
        while n_0 < last_0 {
            if (*mvstack.offset(n_0 as isize)).mv.n == cand_mv_0.n {
                (*mvstack.offset(n_0 as isize)).weight += weight;
                return;
            }
            n_0 += 1;
        }
        if last_0 < 8i32 {
            (*mvstack.offset(last_0 as isize)).mv = cand_mv_0;
            (*mvstack.offset(last_0 as isize)).weight = weight;
            *cnt = last_0 + 1i32;
        }
    }
}
unsafe extern "C" fn scan_row(
    mvstack: *mut refmvs_candidate,
    cnt: *mut libc::c_int,
    ref_0: refmvs_refpair,
    mut gmv: *const mv,
    mut b: *const refmvs_block,
    bw4: libc::c_int,
    w4: libc::c_int,
    max_rows: libc::c_int,
    step: libc::c_int,
    have_newmv_match: *mut libc::c_int,
    have_refmv_match: *mut libc::c_int,
) -> libc::c_int {
    let mut cand_b: *const refmvs_block = b;
    let first_cand_bs: BlockSize = (*cand_b).bs as BlockSize;
    let first_cand_b_dim: *const uint8_t =
        (dav1d_block_dimensions[first_cand_bs as usize]).as_ptr();
    let mut cand_bw4: libc::c_int = *first_cand_b_dim.offset(0isize) as libc::c_int;
    let mut len: libc::c_int = imax(step, imin(bw4, cand_bw4));
    if bw4 <= cand_bw4 {
        let weight: libc::c_int = if bw4 == 1i32 {
            2i32
        } else {
            imax(
                2i32,
                imin(
                    2i32 * max_rows,
                    *first_cand_b_dim.offset(1isize) as libc::c_int,
                ),
            )
        };
        add_spatial_candidate(
            mvstack,
            cnt,
            len * weight,
            cand_b,
            ref_0,
            gmv,
            have_newmv_match,
            have_refmv_match,
        );
        return weight >> 1i32;
    }
    let mut x: libc::c_int = 0i32;
    loop {
        add_spatial_candidate(
            mvstack,
            cnt,
            len * 2i32,
            cand_b,
            ref_0,
            gmv,
            have_newmv_match,
            have_refmv_match,
        );
        x += len;
        if x >= w4 {
            return 1i32;
        }
        cand_b = &*b.offset(x as isize) as *const refmvs_block;
        cand_bw4 = dav1d_block_dimensions[(*cand_b).bs as usize][0usize] as libc::c_int;
        if !(cand_bw4 < bw4) {
            unreachable!();
        }
        len = imax(step, cand_bw4);
    }
}
unsafe extern "C" fn scan_col(
    mvstack: *mut refmvs_candidate,
    cnt: *mut libc::c_int,
    ref_0: refmvs_refpair,
    mut gmv: *const mv,
    mut b: *const *mut refmvs_block,
    bh4: libc::c_int,
    h4: libc::c_int,
    bx4: libc::c_int,
    max_cols: libc::c_int,
    step: libc::c_int,
    have_newmv_match: *mut libc::c_int,
    have_refmv_match: *mut libc::c_int,
) -> libc::c_int {
    let mut cand_b: *const refmvs_block =
        &mut *(*b.offset(0isize)).offset(bx4 as isize) as *mut refmvs_block;
    let first_cand_bs: BlockSize = (*cand_b).bs as BlockSize;
    let first_cand_b_dim: *const uint8_t =
        (dav1d_block_dimensions[first_cand_bs as usize]).as_ptr();
    let mut cand_bh4: libc::c_int = *first_cand_b_dim.offset(1isize) as libc::c_int;
    let mut len: libc::c_int = imax(step, imin(bh4, cand_bh4));
    if bh4 <= cand_bh4 {
        let weight: libc::c_int = if bh4 == 1i32 {
            2i32
        } else {
            imax(
                2i32,
                imin(
                    2i32 * max_cols,
                    *first_cand_b_dim.offset(0isize) as libc::c_int,
                ),
            )
        };
        add_spatial_candidate(
            mvstack,
            cnt,
            len * weight,
            cand_b,
            ref_0,
            gmv,
            have_newmv_match,
            have_refmv_match,
        );
        return weight >> 1i32;
    }
    let mut y: libc::c_int = 0i32;
    loop {
        add_spatial_candidate(
            mvstack,
            cnt,
            len * 2i32,
            cand_b,
            ref_0,
            gmv,
            have_newmv_match,
            have_refmv_match,
        );
        y += len;
        if y >= h4 {
            return 1i32;
        }
        cand_b = &mut *(*b.offset(y as isize)).offset(bx4 as isize) as *mut refmvs_block;
        cand_bh4 = dav1d_block_dimensions[(*cand_b).bs as usize][1usize] as libc::c_int;
        if !(cand_bh4 < bh4) {
            unreachable!();
        }
        len = imax(step, cand_bh4);
    }
}
#[inline]
unsafe extern "C" fn mv_projection(mv: mv, num: libc::c_int, den: libc::c_int) -> mv {
    static mut div_mult: [uint16_t; 32] = [
        0u16, 16384u16, 8192u16, 5461u16, 4096u16, 3276u16, 2730u16, 2340u16, 2048u16, 1820u16,
        1638u16, 1489u16, 1365u16, 1260u16, 1170u16, 1092u16, 1024u16, 963u16, 910u16, 862u16,
        819u16, 780u16, 744u16, 712u16, 682u16, 655u16, 630u16, 606u16, 585u16, 564u16, 546u16,
        528u16,
    ];
    if !(den > 0i32 && den < 32i32) {
        unreachable!();
    }
    if !(num > -(32i32) && num < 32i32) {
        unreachable!();
    }
    let frac: libc::c_int = num * div_mult[den as usize] as libc::c_int;
    let y: libc::c_int = mv.c2rust_unnamed.y as libc::c_int * frac;
    let x: libc::c_int = mv.c2rust_unnamed.x as libc::c_int * frac;
    return mv {
        c2rust_unnamed: {
            let mut init = C2RustUnnamed_12 {
                y: iclip(y + 8192i32 + (y >> 31i32) >> 14i32, -(0x3fffi32), 0x3fffi32) as int16_t,
                x: iclip(x + 8192i32 + (x >> 31i32) >> 14i32, -(0x3fffi32), 0x3fffi32) as int16_t,
            };
            init
        },
    };
}
unsafe extern "C" fn add_temporal_candidate(
    rf: *const refmvs_frame,
    mvstack: *mut refmvs_candidate,
    cnt: *mut libc::c_int,
    rb: *const refmvs_temporal_block,
    ref_0: refmvs_refpair,
    globalmv_ctx: *mut libc::c_int,
    mut gmv: *const mv,
) {
    if (*rb).mv.n == 0x80008000u32 {
        return;
    }
    let mut mv: mv = mv_projection(
        (*rb).mv,
        (*rf).pocdiff[(ref_0.ref_0[0usize] as libc::c_int - 1i32) as usize] as libc::c_int,
        (*rb).ref_0 as libc::c_int,
    );
    fix_mv_precision((*rf).frm_hdr, &mut mv);
    let last: libc::c_int = *cnt;
    if ref_0.ref_0[1usize] as libc::c_int == -(1i32) {
        if !globalmv_ctx.is_null() {
            *globalmv_ctx = (abs(mv.c2rust_unnamed.x as libc::c_int
                - (*gmv.offset(0isize)).c2rust_unnamed.x as libc::c_int)
                | abs(mv.c2rust_unnamed.y as libc::c_int
                    - (*gmv.offset(0isize)).c2rust_unnamed.y as libc::c_int)
                >= 16i32) as libc::c_int;
        }
        let mut n: libc::c_int = 0i32;
        while n < last {
            if (*mvstack.offset(n as isize)).mv.mv[0usize].n == mv.n {
                (*mvstack.offset(n as isize)).weight += 2i32;
                return;
            }
            n += 1;
        }
        if last < 8i32 {
            (*mvstack.offset(last as isize)).mv.mv[0usize] = mv;
            (*mvstack.offset(last as isize)).weight = 2i32;
            *cnt = last + 1i32;
        }
    } else {
        let mut mvp: refmvs_mvpair = refmvs_mvpair {
            mv: [
                mv,
                mv_projection(
                    (*rb).mv,
                    (*rf).pocdiff[(ref_0.ref_0[1usize] as libc::c_int - 1i32) as usize]
                        as libc::c_int,
                    (*rb).ref_0 as libc::c_int,
                ),
            ],
        };
        fix_mv_precision((*rf).frm_hdr, &mut *(mvp.mv).as_mut_ptr().offset(1isize));
        let mut n_0: libc::c_int = 0i32;
        while n_0 < last {
            if (*mvstack.offset(n_0 as isize)).mv.n == mvp.n {
                (*mvstack.offset(n_0 as isize)).weight += 2i32;
                return;
            }
            n_0 += 1;
        }
        if last < 8i32 {
            (*mvstack.offset(last as isize)).mv = mvp;
            (*mvstack.offset(last as isize)).weight = 2i32;
            *cnt = last + 1i32;
        }
    };
}
unsafe extern "C" fn add_compound_extended_candidate(
    same: *mut refmvs_candidate,
    same_count: *mut libc::c_int,
    cand_b: *const refmvs_block,
    sign0: libc::c_int,
    sign1: libc::c_int,
    ref_0: refmvs_refpair,
    sign_bias: *const uint8_t,
) {
    let diff: *mut refmvs_candidate = &mut *same.offset(2isize) as *mut refmvs_candidate;
    let diff_count: *mut libc::c_int = &mut *same_count.offset(2isize) as *mut libc::c_int;
    let mut n: libc::c_int = 0i32;
    while n < 2i32 {
        let cand_ref: libc::c_int = (*cand_b).ref_0.ref_0[n as usize] as libc::c_int;
        if cand_ref <= 0i32 {
            break;
        }
        let mut cand_mv: mv = (*cand_b).mv.mv[n as usize];
        if cand_ref == ref_0.ref_0[0usize] as libc::c_int {
            if *same_count.offset(0isize) < 2i32 {
                let ref mut fresh0 = *same_count.offset(0isize);
                let fresh1 = *fresh0;
                *fresh0 = *fresh0 + 1;
                (*same.offset(fresh1 as isize)).mv.mv[0usize] = cand_mv;
            }
            if *diff_count.offset(1isize) < 2i32 {
                if sign1 ^ *sign_bias.offset((cand_ref - 1i32) as isize) as libc::c_int != 0 {
                    cand_mv.c2rust_unnamed.y =
                        -(cand_mv.c2rust_unnamed.y as libc::c_int) as int16_t;
                    cand_mv.c2rust_unnamed.x =
                        -(cand_mv.c2rust_unnamed.x as libc::c_int) as int16_t;
                }
                let ref mut fresh2 = *diff_count.offset(1isize);
                let fresh3 = *fresh2;
                *fresh2 = *fresh2 + 1;
                (*diff.offset(fresh3 as isize)).mv.mv[1usize] = cand_mv;
            }
        } else if cand_ref == ref_0.ref_0[1usize] as libc::c_int {
            if *same_count.offset(1isize) < 2i32 {
                let ref mut fresh4 = *same_count.offset(1isize);
                let fresh5 = *fresh4;
                *fresh4 = *fresh4 + 1;
                (*same.offset(fresh5 as isize)).mv.mv[1usize] = cand_mv;
            }
            if *diff_count.offset(0isize) < 2i32 {
                if sign0 ^ *sign_bias.offset((cand_ref - 1i32) as isize) as libc::c_int != 0 {
                    cand_mv.c2rust_unnamed.y =
                        -(cand_mv.c2rust_unnamed.y as libc::c_int) as int16_t;
                    cand_mv.c2rust_unnamed.x =
                        -(cand_mv.c2rust_unnamed.x as libc::c_int) as int16_t;
                }
                let ref mut fresh6 = *diff_count.offset(0isize);
                let fresh7 = *fresh6;
                *fresh6 = *fresh6 + 1;
                (*diff.offset(fresh7 as isize)).mv.mv[0usize] = cand_mv;
            }
        } else {
            let mut i_cand_mv: mv = mv {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_12 {
                        y: -(cand_mv.c2rust_unnamed.y as libc::c_int) as int16_t,
                        x: -(cand_mv.c2rust_unnamed.x as libc::c_int) as int16_t,
                    };
                    init
                },
            };
            if *diff_count.offset(0isize) < 2i32 {
                let ref mut fresh8 = *diff_count.offset(0isize);
                let fresh9 = *fresh8;
                *fresh8 = *fresh8 + 1;
                (*diff.offset(fresh9 as isize)).mv.mv[0usize] =
                    if sign0 ^ *sign_bias.offset((cand_ref - 1i32) as isize) as libc::c_int != 0 {
                        i_cand_mv
                    } else {
                        cand_mv
                    };
            }
            if *diff_count.offset(1isize) < 2i32 {
                let ref mut fresh10 = *diff_count.offset(1isize);
                let fresh11 = *fresh10;
                *fresh10 = *fresh10 + 1;
                (*diff.offset(fresh11 as isize)).mv.mv[1usize] =
                    if sign1 ^ *sign_bias.offset((cand_ref - 1i32) as isize) as libc::c_int != 0 {
                        i_cand_mv
                    } else {
                        cand_mv
                    };
            }
        }
        n += 1;
    }
}
unsafe extern "C" fn add_single_extended_candidate(
    mut mvstack: *mut refmvs_candidate,
    cnt: *mut libc::c_int,
    cand_b: *const refmvs_block,
    sign: libc::c_int,
    sign_bias: *const uint8_t,
) {
    let mut n: libc::c_int = 0i32;
    while n < 2i32 {
        let cand_ref: libc::c_int = (*cand_b).ref_0.ref_0[n as usize] as libc::c_int;
        if cand_ref <= 0i32 {
            break;
        }
        let mut cand_mv: mv = (*cand_b).mv.mv[n as usize];
        if sign ^ *sign_bias.offset((cand_ref - 1i32) as isize) as libc::c_int != 0 {
            cand_mv.c2rust_unnamed.y = -(cand_mv.c2rust_unnamed.y as libc::c_int) as int16_t;
            cand_mv.c2rust_unnamed.x = -(cand_mv.c2rust_unnamed.x as libc::c_int) as int16_t;
        }
        let mut m: libc::c_int = 0;
        let last: libc::c_int = *cnt;
        m = 0i32;
        while m < last {
            if cand_mv.n == (*mvstack.offset(m as isize)).mv.mv[0usize].n {
                break;
            }
            m += 1;
        }
        if m == last {
            (*mvstack.offset(m as isize)).mv.mv[0usize] = cand_mv;
            (*mvstack.offset(m as isize)).weight = 2i32;
            *cnt = last + 1i32;
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_refmvs_find(
    rt: *const refmvs_tile,
    mut mvstack: *mut refmvs_candidate,
    cnt: *mut libc::c_int,
    ctx: *mut libc::c_int,
    ref_0: refmvs_refpair,
    bs: BlockSize,
    edge_flags: EdgeFlags,
    by4: libc::c_int,
    bx4: libc::c_int,
) {
    let rf: *const refmvs_frame = (*rt).rf;
    let b_dim: *const uint8_t = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bw4: libc::c_int = *b_dim.offset(0isize) as libc::c_int;
    let w4: libc::c_int = imin(imin(bw4, 16i32), (*rt).tile_col.end - bx4);
    let bh4: libc::c_int = *b_dim.offset(1isize) as libc::c_int;
    let h4: libc::c_int = imin(imin(bh4, 16i32), (*rt).tile_row.end - by4);
    let mut gmv: [mv; 2] = [mv {
        c2rust_unnamed: C2RustUnnamed_12 { y: 0, x: 0 },
    }; 2];
    let mut tgmv: [mv; 2] = [mv {
        c2rust_unnamed: C2RustUnnamed_12 { y: 0, x: 0 },
    }; 2];
    *cnt = 0i32;
    if !(ref_0.ref_0[0usize] as libc::c_int >= 0i32
        && ref_0.ref_0[0usize] as libc::c_int <= 8i32
        && ref_0.ref_0[1usize] as libc::c_int >= -(1i32)
        && ref_0.ref_0[1usize] as libc::c_int <= 8i32)
    {
        unreachable!();
    }
    if ref_0.ref_0[0usize] as libc::c_int > 0i32 {
        tgmv[0usize] = get_gmv_2d(
            &*((*(*rf).frm_hdr).gmv)
                .as_ptr()
                .offset((*(ref_0.ref_0).as_ptr().offset(0isize) as libc::c_int - 1i32) as isize),
            bx4,
            by4,
            bw4,
            bh4,
            (*rf).frm_hdr,
        );
        gmv[0usize] = if (*(*rf).frm_hdr).gmv[(ref_0.ref_0[0usize] as libc::c_int - 1i32) as usize]
            .type_0
            > DAV1D_WM_TYPE_TRANSLATION
        {
            tgmv[0usize]
        } else {
            mv { n: 0x80008000u32 }
        };
    } else {
        tgmv[0usize] = mv { n: 0u32 };
        gmv[0usize] = mv { n: 0x80008000u32 };
    }
    if ref_0.ref_0[1usize] as libc::c_int > 0i32 {
        tgmv[1usize] = get_gmv_2d(
            &*((*(*rf).frm_hdr).gmv)
                .as_ptr()
                .offset((*(ref_0.ref_0).as_ptr().offset(1isize) as libc::c_int - 1i32) as isize),
            bx4,
            by4,
            bw4,
            bh4,
            (*rf).frm_hdr,
        );
        gmv[1usize] = if (*(*rf).frm_hdr).gmv[(ref_0.ref_0[1usize] as libc::c_int - 1i32) as usize]
            .type_0
            > DAV1D_WM_TYPE_TRANSLATION
        {
            tgmv[1usize]
        } else {
            mv { n: 0x80008000u32 }
        };
    }
    let mut have_newmv: libc::c_int = 0i32;
    let mut have_col_mvs: libc::c_int = 0i32;
    let mut have_row_mvs: libc::c_int = 0i32;
    let mut max_rows: libc::c_uint = 0u32;
    let mut n_rows: libc::c_uint = !(0i32) as libc::c_uint;
    let mut b_top: *const refmvs_block = 0 as *const refmvs_block;
    if by4 > (*rt).tile_row.start {
        max_rows = imin(
            by4 - (*rt).tile_row.start + 1i32 >> 1i32,
            2i32 + (bh4 > 1i32) as libc::c_int,
        ) as libc::c_uint;
        b_top = &mut *(*((*rt).r)
            .as_ptr()
            .offset(((by4 & 31i32) - 1i32 + 5i32) as isize))
        .offset(bx4 as isize) as *mut refmvs_block;
        n_rows = scan_row(
            mvstack,
            cnt,
            ref_0,
            gmv.as_mut_ptr() as *const mv,
            b_top,
            bw4,
            w4,
            max_rows as libc::c_int,
            if bw4 >= 16i32 { 4i32 } else { 1i32 },
            &mut have_newmv,
            &mut have_row_mvs,
        ) as libc::c_uint;
    }
    let mut max_cols: libc::c_uint = 0u32;
    let mut n_cols: libc::c_uint = !(0u32);
    let mut b_left: *const *mut refmvs_block = 0 as *const *mut refmvs_block;
    if bx4 > (*rt).tile_col.start {
        max_cols = imin(
            bx4 - (*rt).tile_col.start + 1i32 >> 1i32,
            2i32 + (bw4 > 1i32) as libc::c_int,
        ) as libc::c_uint;
        b_left = &*((*rt).r).as_ptr().offset(((by4 & 31i32) + 5i32) as isize)
            as *const *mut refmvs_block;
        n_cols = scan_col(
            mvstack,
            cnt,
            ref_0,
            gmv.as_mut_ptr() as *const mv,
            b_left,
            bh4,
            h4,
            bx4 - 1i32,
            max_cols as libc::c_int,
            if bh4 >= 16i32 { 4i32 } else { 1i32 },
            &mut have_newmv,
            &mut have_col_mvs,
        ) as libc::c_uint;
    }
    if n_rows != !(0u32)
        && edge_flags & EDGE_I444_TOP_HAS_RIGHT != 0
        && imax(bw4, bh4) <= 16i32
        && bw4 + bx4 < (*rt).tile_col.end
    {
        add_spatial_candidate(
            mvstack,
            cnt,
            4i32,
            &*b_top.offset(bw4 as isize),
            ref_0,
            gmv.as_mut_ptr() as *const mv,
            &mut have_newmv,
            &mut have_row_mvs,
        );
    }
    let nearest_match: libc::c_int = have_col_mvs + have_row_mvs;
    let nearest_cnt: libc::c_int = *cnt;
    let mut n: libc::c_int = 0i32;
    while n < nearest_cnt {
        (*mvstack.offset(n as isize)).weight += 640i32;
        n += 1;
    }
    let mut globalmv_ctx: libc::c_int = (*(*rf).frm_hdr).use_ref_frame_mvs;
    if (*rf).use_ref_frame_mvs != 0 {
        let stride: ptrdiff_t = (*rf).rp_stride;
        let by8: libc::c_int = by4 >> 1i32;
        let bx8: libc::c_int = bx4 >> 1i32;
        let rbi: *const refmvs_temporal_block = &mut *((*rt).rp_proj)
            .offset(((by8 & 15i32) as libc::c_long * stride + bx8 as libc::c_long) as isize)
            as *mut refmvs_temporal_block;
        let mut rb: *const refmvs_temporal_block = rbi;
        let step_h: libc::c_int = if bw4 >= 16i32 { 2i32 } else { 1i32 };
        let step_v: libc::c_int = if bh4 >= 16i32 { 2i32 } else { 1i32 };
        let w8: libc::c_int = imin(w4 + 1i32 >> 1i32, 8i32);
        let h8: libc::c_int = imin(h4 + 1i32 >> 1i32, 8i32);
        let mut y: libc::c_int = 0i32;
        while y < h8 {
            let mut x: libc::c_int = 0i32;
            while x < w8 {
                add_temporal_candidate(
                    rf,
                    mvstack,
                    cnt,
                    &*rb.offset(x as isize),
                    ref_0,
                    if x | y == 0 {
                        &mut globalmv_ctx
                    } else {
                        0 as *mut libc::c_int
                    },
                    tgmv.as_mut_ptr() as *const mv,
                );
                x += step_h;
            }
            rb = rb.offset((stride * step_v as libc::c_long) as isize);
            y += step_v;
        }
        if imin(bw4, bh4) >= 2i32 && imax(bw4, bh4) < 16i32 {
            let bh8: libc::c_int = bh4 >> 1i32;
            let bw8: libc::c_int = bw4 >> 1i32;
            rb = &*rbi.offset((bh8 as libc::c_long * stride) as isize)
                as *const refmvs_temporal_block;
            let has_bottom: libc::c_int = (by8 + bh8
                < imin((*rt).tile_row.end >> 1i32, (by8 & !(7i32)) + 8i32))
                as libc::c_int;
            if has_bottom != 0 && bx8 - 1i32 >= imax((*rt).tile_col.start >> 1i32, bx8 & !(7i32)) {
                add_temporal_candidate(
                    rf,
                    mvstack,
                    cnt,
                    &*rb.offset(-1isize),
                    ref_0,
                    0 as *mut libc::c_int,
                    0 as *const mv,
                );
            }
            if bx8 + bw8 < imin((*rt).tile_col.end >> 1i32, (bx8 & !(7i32)) + 8i32) {
                if has_bottom != 0 {
                    add_temporal_candidate(
                        rf,
                        mvstack,
                        cnt,
                        &*rb.offset(bw8 as isize),
                        ref_0,
                        0 as *mut libc::c_int,
                        0 as *const mv,
                    );
                }
                if (by8 + bh8 - 1i32) < imin((*rt).tile_row.end >> 1i32, (by8 & !(7i32)) + 8i32) {
                    add_temporal_candidate(
                        rf,
                        mvstack,
                        cnt,
                        &*rb.offset((bw8 as libc::c_long - stride) as isize),
                        ref_0,
                        0 as *mut libc::c_int,
                        0 as *const mv,
                    );
                }
            }
        }
    }
    if !(*cnt <= 8i32) {
        unreachable!();
    }
    let mut have_dummy_newmv_match: libc::c_int = 0;
    if n_rows | n_cols != !(0u32) {
        add_spatial_candidate(
            mvstack,
            cnt,
            4i32,
            &*b_top.offset(-1isize),
            ref_0,
            gmv.as_mut_ptr() as *const mv,
            &mut have_dummy_newmv_match,
            &mut have_row_mvs,
        );
    }
    let mut n_0: libc::c_int = 2i32;
    while n_0 <= 3i32 {
        if n_0 as libc::c_uint > n_rows && n_0 as libc::c_uint <= max_rows {
            n_rows = n_rows.wrapping_add(scan_row(
                mvstack,
                cnt,
                ref_0,
                gmv.as_mut_ptr() as *const mv,
                &mut *(*((*rt).r)
                    .as_ptr()
                    .offset((((by4 & 31i32) - 2i32 * n_0 + 1i32 | 1i32) + 5i32) as isize))
                .offset((bx4 | 1i32) as isize),
                bw4,
                w4,
                (1u32)
                    .wrapping_add(max_rows)
                    .wrapping_sub(n_0 as libc::c_uint) as libc::c_int,
                if bw4 >= 16i32 { 4i32 } else { 2i32 },
                &mut have_dummy_newmv_match,
                &mut have_row_mvs,
            ) as libc::c_uint);
        }
        if n_0 as libc::c_uint > n_cols && n_0 as libc::c_uint <= max_cols {
            n_cols = n_cols.wrapping_add(scan_col(
                mvstack,
                cnt,
                ref_0,
                gmv.as_mut_ptr() as *const mv,
                &*((*rt).r)
                    .as_ptr()
                    .offset(((by4 & 31i32 | 1i32) + 5i32) as isize),
                bh4,
                h4,
                bx4 - n_0 * 2i32 + 1i32 | 1i32,
                (1u32)
                    .wrapping_add(max_cols)
                    .wrapping_sub(n_0 as libc::c_uint) as libc::c_int,
                if bh4 >= 16i32 { 4i32 } else { 2i32 },
                &mut have_dummy_newmv_match,
                &mut have_col_mvs,
            ) as libc::c_uint);
        }
        n_0 += 1;
    }
    if !(*cnt <= 8i32) {
        unreachable!();
    }
    let ref_match_count: libc::c_int = have_col_mvs + have_row_mvs;
    let mut refmv_ctx: libc::c_int = 0;
    let mut newmv_ctx: libc::c_int = 0;
    match nearest_match {
        0 => {
            refmv_ctx = imin(2i32, ref_match_count);
            newmv_ctx = (ref_match_count > 0i32) as libc::c_int;
        }
        1 => {
            refmv_ctx = imin(ref_match_count * 3i32, 4i32);
            newmv_ctx = 3i32 - have_newmv;
        }
        2 => {
            refmv_ctx = 5i32;
            newmv_ctx = 5i32 - have_newmv;
        }
        _ => {}
    }
    let mut len: libc::c_int = nearest_cnt;
    while len != 0 {
        let mut last: libc::c_int = 0i32;
        let mut n_1: libc::c_int = 1i32;
        while n_1 < len {
            if (*mvstack.offset((n_1 - 1i32) as isize)).weight
                < (*mvstack.offset(n_1 as isize)).weight
            {
                let mut tmp: refmvs_candidate = *mvstack.offset((n_1 - 1i32) as isize);
                *mvstack.offset((n_1 - 1i32) as isize) = *mvstack.offset(n_1 as isize);
                *mvstack.offset(n_1 as isize) = tmp;
                last = n_1;
            }
            n_1 += 1;
        }
        len = last;
    }
    len = *cnt;
    while len > nearest_cnt {
        let mut last_0: libc::c_int = nearest_cnt;
        let mut n_2: libc::c_int = nearest_cnt + 1i32;
        while n_2 < len {
            if (*mvstack.offset((n_2 - 1i32) as isize)).weight
                < (*mvstack.offset(n_2 as isize)).weight
            {
                let mut tmp_0: refmvs_candidate = *mvstack.offset((n_2 - 1i32) as isize);
                *mvstack.offset((n_2 - 1i32) as isize) = *mvstack.offset(n_2 as isize);
                *mvstack.offset(n_2 as isize) = tmp_0;
                last_0 = n_2;
            }
            n_2 += 1;
        }
        len = last_0;
    }
    if ref_0.ref_0[1usize] as libc::c_int > 0i32 {
        if *cnt < 2i32 {
            let sign0: libc::c_int = (*rf).sign_bias
                [(ref_0.ref_0[0usize] as libc::c_int - 1i32) as usize]
                as libc::c_int;
            let sign1: libc::c_int = (*rf).sign_bias
                [(ref_0.ref_0[1usize] as libc::c_int - 1i32) as usize]
                as libc::c_int;
            let sz4: libc::c_int = imin(w4, h4);
            let same: *mut refmvs_candidate =
                &mut *mvstack.offset(*cnt as isize) as *mut refmvs_candidate;
            let mut same_count: [libc::c_int; 4] = [0i32, 0, 0, 0];
            if n_rows != !(0u32) {
                let mut x_0: libc::c_int = 0i32;
                while x_0 < sz4 {
                    let cand_b: *const refmvs_block =
                        &*b_top.offset(x_0 as isize) as *const refmvs_block;
                    add_compound_extended_candidate(
                        same,
                        same_count.as_mut_ptr(),
                        cand_b,
                        sign0,
                        sign1,
                        ref_0,
                        ((*rf).sign_bias).as_ptr(),
                    );
                    x_0 += dav1d_block_dimensions[(*cand_b).bs as usize][0usize] as libc::c_int;
                }
            }
            if n_cols != !(0u32) {
                let mut y_0: libc::c_int = 0i32;
                while y_0 < sz4 {
                    let cand_b_0: *const refmvs_block = &mut *(*b_left.offset(y_0 as isize))
                        .offset((bx4 - 1i32) as isize)
                        as *mut refmvs_block;
                    add_compound_extended_candidate(
                        same,
                        same_count.as_mut_ptr(),
                        cand_b_0,
                        sign0,
                        sign1,
                        ref_0,
                        ((*rf).sign_bias).as_ptr(),
                    );
                    y_0 += dav1d_block_dimensions[(*cand_b_0).bs as usize][1usize] as libc::c_int;
                }
            }
            let diff: *mut refmvs_candidate = &mut *same.offset(2isize) as *mut refmvs_candidate;
            let diff_count: *const libc::c_int =
                &mut *same_count.as_mut_ptr().offset(2isize) as *mut libc::c_int;
            let mut current_block_118: u64;
            let mut n_3: libc::c_int = 0i32;
            while n_3 < 2i32 {
                let mut m: libc::c_int = same_count[n_3 as usize];
                if !(m >= 2i32) {
                    let l: libc::c_int = *diff_count.offset(n_3 as isize);
                    if l != 0 {
                        (*same.offset(m as isize)).mv.mv[n_3 as usize] =
                            (*diff.offset(0isize)).mv.mv[n_3 as usize];
                        m += 1;
                        if m == 2i32 {
                            current_block_118 = 13740693533991687037;
                        } else if l == 2i32 {
                            (*same.offset(1isize)).mv.mv[n_3 as usize] =
                                (*diff.offset(1isize)).mv.mv[n_3 as usize];
                            current_block_118 = 13740693533991687037;
                        } else {
                            current_block_118 = 9430418855388998878;
                        }
                    } else {
                        current_block_118 = 9430418855388998878;
                    }
                    match current_block_118 {
                        13740693533991687037 => {}
                        _ => loop {
                            (*same.offset(m as isize)).mv.mv[n_3 as usize] = tgmv[n_3 as usize];
                            m += 1;
                            if !(m < 2i32) {
                                break;
                            }
                        },
                    }
                }
                n_3 += 1;
            }
            let mut n_4: libc::c_int = *cnt;
            if n_4 == 1i32 && (*mvstack.offset(0isize)).mv.n == (*same.offset(0isize)).mv.n {
                (*mvstack.offset(1isize)).mv = (*mvstack.offset(2isize)).mv;
            }
            loop {
                (*mvstack.offset(n_4 as isize)).weight = 2i32;
                n_4 += 1;
                if !(n_4 < 2i32) {
                    break;
                }
            }
            *cnt = 2i32;
        }
        let left: libc::c_int = -(bx4 + bw4 + 4i32) * 4i32 * 8i32;
        let right: libc::c_int = ((*rf).iw4 - bx4 + 4i32) * 4i32 * 8i32;
        let top: libc::c_int = -(by4 + bh4 + 4i32) * 4i32 * 8i32;
        let bottom: libc::c_int = ((*rf).ih4 - by4 + 4i32) * 4i32 * 8i32;
        let n_refmvs: libc::c_int = *cnt;
        let mut n_5: libc::c_int = 0i32;
        loop {
            (*mvstack.offset(n_5 as isize)).mv.mv[0usize]
                .c2rust_unnamed
                .x = iclip(
                (*mvstack.offset(n_5 as isize)).mv.mv[0usize]
                    .c2rust_unnamed
                    .x as libc::c_int,
                left,
                right,
            ) as int16_t;
            (*mvstack.offset(n_5 as isize)).mv.mv[0usize]
                .c2rust_unnamed
                .y = iclip(
                (*mvstack.offset(n_5 as isize)).mv.mv[0usize]
                    .c2rust_unnamed
                    .y as libc::c_int,
                top,
                bottom,
            ) as int16_t;
            (*mvstack.offset(n_5 as isize)).mv.mv[1usize]
                .c2rust_unnamed
                .x = iclip(
                (*mvstack.offset(n_5 as isize)).mv.mv[1usize]
                    .c2rust_unnamed
                    .x as libc::c_int,
                left,
                right,
            ) as int16_t;
            (*mvstack.offset(n_5 as isize)).mv.mv[1usize]
                .c2rust_unnamed
                .y = iclip(
                (*mvstack.offset(n_5 as isize)).mv.mv[1usize]
                    .c2rust_unnamed
                    .y as libc::c_int,
                top,
                bottom,
            ) as int16_t;
            n_5 += 1;
            if !(n_5 < n_refmvs) {
                break;
            }
        }
        match refmv_ctx >> 1i32 {
            0 => {
                *ctx = imin(newmv_ctx, 1i32);
            }
            1 => {
                *ctx = 1i32 + imin(newmv_ctx, 3i32);
            }
            2 => {
                *ctx = iclip(3i32 + newmv_ctx, 4i32, 7i32);
            }
            _ => {}
        }
        return;
    } else {
        if *cnt < 2i32 && ref_0.ref_0[0usize] as libc::c_int > 0i32 {
            let sign: libc::c_int = (*rf).sign_bias
                [(ref_0.ref_0[0usize] as libc::c_int - 1i32) as usize]
                as libc::c_int;
            let sz4_0: libc::c_int = imin(w4, h4);
            if n_rows != !(0u32) {
                let mut x_1: libc::c_int = 0i32;
                while x_1 < sz4_0 && *cnt < 2i32 {
                    let cand_b_1: *const refmvs_block =
                        &*b_top.offset(x_1 as isize) as *const refmvs_block;
                    add_single_extended_candidate(
                        mvstack,
                        cnt,
                        cand_b_1,
                        sign,
                        ((*rf).sign_bias).as_ptr(),
                    );
                    x_1 += dav1d_block_dimensions[(*cand_b_1).bs as usize][0usize] as libc::c_int;
                }
            }
            if n_cols != !(0u32) {
                let mut y_1: libc::c_int = 0i32;
                while y_1 < sz4_0 && *cnt < 2i32 {
                    let cand_b_2: *const refmvs_block = &mut *(*b_left.offset(y_1 as isize))
                        .offset((bx4 - 1i32) as isize)
                        as *mut refmvs_block;
                    add_single_extended_candidate(
                        mvstack,
                        cnt,
                        cand_b_2,
                        sign,
                        ((*rf).sign_bias).as_ptr(),
                    );
                    y_1 += dav1d_block_dimensions[(*cand_b_2).bs as usize][1usize] as libc::c_int;
                }
            }
        }
    }
    if !(*cnt <= 8i32) {
        unreachable!();
    }
    let mut n_refmvs_0: libc::c_int = *cnt;
    if n_refmvs_0 != 0 {
        let left_0: libc::c_int = -(bx4 + bw4 + 4i32) * 4i32 * 8i32;
        let right_0: libc::c_int = ((*rf).iw4 - bx4 + 4i32) * 4i32 * 8i32;
        let top_0: libc::c_int = -(by4 + bh4 + 4i32) * 4i32 * 8i32;
        let bottom_0: libc::c_int = ((*rf).ih4 - by4 + 4i32) * 4i32 * 8i32;
        let mut n_6: libc::c_int = 0i32;
        loop {
            (*mvstack.offset(n_6 as isize)).mv.mv[0usize]
                .c2rust_unnamed
                .x = iclip(
                (*mvstack.offset(n_6 as isize)).mv.mv[0usize]
                    .c2rust_unnamed
                    .x as libc::c_int,
                left_0,
                right_0,
            ) as int16_t;
            (*mvstack.offset(n_6 as isize)).mv.mv[0usize]
                .c2rust_unnamed
                .y = iclip(
                (*mvstack.offset(n_6 as isize)).mv.mv[0usize]
                    .c2rust_unnamed
                    .y as libc::c_int,
                top_0,
                bottom_0,
            ) as int16_t;
            n_6 += 1;
            if !(n_6 < n_refmvs_0) {
                break;
            }
        }
    }
    let mut n_7: libc::c_int = *cnt;
    while n_7 < 2i32 {
        (*mvstack.offset(n_7 as isize)).mv.mv[0usize] = tgmv[0usize];
        n_7 += 1;
    }
    *ctx = refmv_ctx << 4i32 | globalmv_ctx << 3i32 | newmv_ctx;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_refmvs_tile_sbrow_init(
    rt: *mut refmvs_tile,
    rf: *const refmvs_frame,
    tile_col_start4: libc::c_int,
    tile_col_end4: libc::c_int,
    tile_row_start4: libc::c_int,
    tile_row_end4: libc::c_int,
    sby: libc::c_int,
    mut tile_row_idx: libc::c_int,
    pass: libc::c_int,
) {
    if (*rf).n_tile_threads == 1i32 {
        tile_row_idx = 0i32;
    }
    (*rt).rp_proj = &mut *((*rf).rp_proj)
        .offset((16i64 * (*rf).rp_stride * tile_row_idx as libc::c_long) as isize)
        as *mut refmvs_temporal_block;
    let uses_2pass: libc::c_int =
        ((*rf).n_tile_threads > 1i32 && (*rf).n_frame_threads > 1i32) as libc::c_int;
    let pass_off: ptrdiff_t = if uses_2pass != 0 && pass == 2i32 {
        35i64 * (*rf).r_stride * (*rf).n_tile_rows as libc::c_long
    } else {
        0i64
    };
    let mut r: *mut refmvs_block = &mut *((*rf).r)
        .offset((35i64 * (*rf).r_stride * tile_row_idx as libc::c_long + pass_off) as isize)
        as *mut refmvs_block;
    let sbsz: libc::c_int = (*rf).sbsz;
    let off: libc::c_int = sbsz * sby & 16i32;
    let mut i: libc::c_int = 0i32;
    while i < sbsz {
        (*rt).r[(off + 5i32 + i) as usize] = r;
        i += 1;
        r = r.offset((*rf).r_stride as isize);
    }
    (*rt).r[(off + 0i32) as usize] = r;
    r = r.offset((*rf).r_stride as isize);
    (*rt).r[(off + 1i32) as usize] = 0 as *mut refmvs_block;
    (*rt).r[(off + 2i32) as usize] = r;
    r = r.offset((*rf).r_stride as isize);
    (*rt).r[(off + 3i32) as usize] = 0 as *mut refmvs_block;
    (*rt).r[(off + 4i32) as usize] = r;
    if sby & 1i32 != 0 {
        let tmp: *mut libc::c_void = (*rt).r[(off + 0i32) as usize] as *mut libc::c_void;
        (*rt).r[(off + 0i32) as usize] = (*rt).r[(off + sbsz + 0i32) as usize];
        (*rt).r[(off + sbsz + 0i32) as usize] = tmp as *mut refmvs_block;
        let tmp_0: *mut libc::c_void = (*rt).r[(off + 2i32) as usize] as *mut libc::c_void;
        (*rt).r[(off + 2i32) as usize] = (*rt).r[(off + sbsz + 2i32) as usize];
        (*rt).r[(off + sbsz + 2i32) as usize] = tmp_0 as *mut refmvs_block;
        let tmp_1: *mut libc::c_void = (*rt).r[(off + 4i32) as usize] as *mut libc::c_void;
        (*rt).r[(off + 4i32) as usize] = (*rt).r[(off + sbsz + 4i32) as usize];
        (*rt).r[(off + sbsz + 4i32) as usize] = tmp_1 as *mut refmvs_block;
    }
    (*rt).rf = rf;
    (*rt).tile_row.start = tile_row_start4;
    (*rt).tile_row.end = imin(tile_row_end4, (*rf).ih4);
    (*rt).tile_col.start = tile_col_start4;
    (*rt).tile_col.end = imin(tile_col_end4, (*rf).iw4);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_refmvs_load_tmvs(
    rf: *const refmvs_frame,
    mut tile_row_idx: libc::c_int,
    col_start8: libc::c_int,
    col_end8: libc::c_int,
    row_start8: libc::c_int,
    mut row_end8: libc::c_int,
) {
    if (*rf).n_tile_threads == 1i32 {
        tile_row_idx = 0i32;
    }
    if !(row_start8 >= 0i32) {
        unreachable!();
    }
    if !((row_end8 - row_start8) as libc::c_uint <= 16u32) {
        unreachable!();
    }
    row_end8 = imin(row_end8, (*rf).ih8);
    let col_start8i: libc::c_int = imax(col_start8 - 8i32, 0i32);
    let col_end8i: libc::c_int = imin(col_end8 + 8i32, (*rf).iw8);
    let stride: ptrdiff_t = (*rf).rp_stride;
    let mut rp_proj: *mut refmvs_temporal_block = &mut *((*rf).rp_proj).offset(
        (16i64 * stride * tile_row_idx as libc::c_long
            + (row_start8 & 15i32) as libc::c_long * stride) as isize,
    ) as *mut refmvs_temporal_block;
    let mut y: libc::c_int = row_start8;
    while y < row_end8 {
        let mut x: libc::c_int = col_start8;
        while x < col_end8 {
            (*rp_proj.offset(x as isize)).mv.n = 0x80008000u32;
            x += 1;
        }
        rp_proj = rp_proj.offset(stride as isize);
        y += 1;
    }
    rp_proj = &mut *((*rf).rp_proj).offset((16i64 * stride * tile_row_idx as libc::c_long) as isize)
        as *mut refmvs_temporal_block;
    let mut n: libc::c_int = 0i32;
    while n < (*rf).n_mfmvs {
        let ref2cur: libc::c_int = (*rf).mfmv_ref2cur[n as usize];
        if !(ref2cur == -(2147483647i32) - 1i32) {
            let ref_0: libc::c_int = (*rf).mfmv_ref[n as usize] as libc::c_int;
            let ref_sign: libc::c_int = ref_0 - 4i32;
            let mut r: *const refmvs_temporal_block = &mut *(*((*rf).rp_ref).offset(ref_0 as isize))
                .offset((row_start8 as libc::c_long * stride) as isize)
                as *mut refmvs_temporal_block;
            let mut y_0: libc::c_int = row_start8;
            while y_0 < row_end8 {
                let y_sb_align: libc::c_int = y_0 & !(7i32);
                let y_proj_start: libc::c_int = imax(y_sb_align, row_start8);
                let y_proj_end: libc::c_int = imin(y_sb_align + 8i32, row_end8);
                let mut x_0: libc::c_int = col_start8i;
                while x_0 < col_end8i {
                    let mut rb: *const refmvs_temporal_block =
                        &*r.offset(x_0 as isize) as *const refmvs_temporal_block;
                    let b_ref: libc::c_int = (*rb).ref_0 as libc::c_int;
                    if !(b_ref == 0) {
                        let ref2ref: libc::c_int =
                            (*rf).mfmv_ref2ref[n as usize][(b_ref - 1i32) as usize];
                        if !(ref2ref == 0) {
                            let b_mv: mv = (*rb).mv;
                            let offset: mv = mv_projection(b_mv, ref2cur, ref2ref);
                            let mut pos_x: libc::c_int = x_0
                                + apply_sign(
                                    abs(offset.c2rust_unnamed.x as libc::c_int) >> 6i32,
                                    offset.c2rust_unnamed.x as libc::c_int ^ ref_sign,
                                );
                            let pos_y: libc::c_int = y_0
                                + apply_sign(
                                    abs(offset.c2rust_unnamed.y as libc::c_int) >> 6i32,
                                    offset.c2rust_unnamed.y as libc::c_int ^ ref_sign,
                                );
                            if pos_y >= y_proj_start && pos_y < y_proj_end {
                                let pos: ptrdiff_t = (pos_y & 15i32) as libc::c_long * stride;
                                loop {
                                    let x_sb_align: libc::c_int = x_0 & !(7i32);
                                    if pos_x >= imax(x_sb_align - 8i32, col_start8)
                                        && pos_x < imin(x_sb_align + 16i32, col_end8)
                                    {
                                        (*rp_proj.offset((pos + pos_x as libc::c_long) as isize))
                                            .mv = (*rb).mv;
                                        (*rp_proj.offset((pos + pos_x as libc::c_long) as isize))
                                            .ref_0 = ref2ref as int8_t;
                                    }
                                    x_0 += 1;
                                    if x_0 >= col_end8i {
                                        break;
                                    }
                                    rb = rb.offset(1);
                                    if (*rb).ref_0 as libc::c_int != b_ref || (*rb).mv.n != b_mv.n {
                                        break;
                                    }
                                    pos_x += 1;
                                }
                            } else {
                                loop {
                                    x_0 += 1;
                                    if x_0 >= col_end8i {
                                        break;
                                    }
                                    rb = rb.offset(1);
                                    if (*rb).ref_0 as libc::c_int != b_ref || (*rb).mv.n != b_mv.n {
                                        break;
                                    }
                                }
                            }
                            x_0 -= 1;
                        }
                    }
                    x_0 += 1;
                }
                r = r.offset(stride as isize);
                y_0 += 1;
            }
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_refmvs_save_tmvs(
    rt: *const refmvs_tile,
    col_start8: libc::c_int,
    mut col_end8: libc::c_int,
    row_start8: libc::c_int,
    mut row_end8: libc::c_int,
) {
    let rf: *const refmvs_frame = (*rt).rf;
    if !(row_start8 >= 0i32) {
        unreachable!();
    }
    if !((row_end8 - row_start8) as libc::c_uint <= 16u32) {
        unreachable!();
    }
    row_end8 = imin(row_end8, (*rf).ih8);
    col_end8 = imin(col_end8, (*rf).iw8);
    let stride: ptrdiff_t = (*rf).rp_stride;
    let ref_sign: *const uint8_t = ((*rf).mfmv_sign).as_ptr();
    let mut rp: *mut refmvs_temporal_block = &mut *((*rf).rp)
        .offset((row_start8 as libc::c_long * stride) as isize)
        as *mut refmvs_temporal_block;
    let mut y: libc::c_int = row_start8;
    while y < row_end8 {
        let b: *const refmvs_block = (*rt).r[(6i32 + (y & 15i32) * 2i32) as usize];
        let mut x: libc::c_int = col_start8;
        while x < col_end8 {
            let cand_b: *const refmvs_block =
                &*b.offset((x * 2i32 + 1i32) as isize) as *const refmvs_block;
            let bw8: libc::c_int =
                dav1d_block_dimensions[(*cand_b).bs as usize][0usize] as libc::c_int + 1i32 >> 1i32;
            if (*cand_b).ref_0.ref_0[1usize] as libc::c_int > 0i32
                && *ref_sign.offset(((*cand_b).ref_0.ref_0[1usize] as libc::c_int - 1i32) as isize)
                    as libc::c_int
                    != 0
                && abs((*cand_b).mv.mv[1usize].c2rust_unnamed.y as libc::c_int)
                    | abs((*cand_b).mv.mv[1usize].c2rust_unnamed.x as libc::c_int)
                    < 4096i32
            {
                let mut n: libc::c_int = 0i32;
                while n < bw8 {
                    *rp.offset(x as isize) = {
                        let mut init = refmvs_temporal_block {
                            mv: (*cand_b).mv.mv[1usize],
                            ref_0: (*cand_b).ref_0.ref_0[1usize],
                        };
                        init
                    };
                    n += 1;
                    x += 1;
                }
            } else if (*cand_b).ref_0.ref_0[0usize] as libc::c_int > 0i32
                && *ref_sign.offset(((*cand_b).ref_0.ref_0[0usize] as libc::c_int - 1i32) as isize)
                    as libc::c_int
                    != 0
                && abs((*cand_b).mv.mv[0usize].c2rust_unnamed.y as libc::c_int)
                    | abs((*cand_b).mv.mv[0usize].c2rust_unnamed.x as libc::c_int)
                    < 4096i32
            {
                let mut n_0: libc::c_int = 0i32;
                while n_0 < bw8 {
                    *rp.offset(x as isize) = {
                        let mut init = refmvs_temporal_block {
                            mv: (*cand_b).mv.mv[0usize],
                            ref_0: (*cand_b).ref_0.ref_0[0usize],
                        };
                        init
                    };
                    n_0 += 1;
                    x += 1;
                }
            } else {
                let mut n_1: libc::c_int = 0i32;
                while n_1 < bw8 {
                    (*rp.offset(x as isize)).ref_0 = 0i8;
                    n_1 += 1;
                    x += 1;
                }
            }
        }
        rp = rp.offset(stride as isize);
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_refmvs_init_frame(
    rf: *mut refmvs_frame,
    seq_hdr: *const Dav1dSequenceHeader,
    frm_hdr: *const Dav1dFrameHeader,
    mut ref_poc: *const libc::c_uint,
    rp: *mut refmvs_temporal_block,
    mut ref_ref_poc: *const [libc::c_uint; 7],
    mut rp_ref: *const *mut refmvs_temporal_block,
    n_tile_threads: libc::c_int,
    n_frame_threads: libc::c_int,
) -> libc::c_int {
    (*rf).sbsz = (16i32) << (*seq_hdr).sb128;
    (*rf).frm_hdr = frm_hdr;
    (*rf).iw8 = (*frm_hdr).width[0usize] + 7i32 >> 3i32;
    (*rf).ih8 = (*frm_hdr).height + 7i32 >> 3i32;
    (*rf).iw4 = (*rf).iw8 << 1i32;
    (*rf).ih4 = (*rf).ih8 << 1i32;
    let r_stride: ptrdiff_t =
        (((*frm_hdr).width[0usize] + 127i32 & !(127i32)) >> 2i32) as ptrdiff_t;
    let n_tile_rows: libc::c_int = if n_tile_threads > 1i32 {
        (*frm_hdr).tiling.rows
    } else {
        1i32
    };
    if r_stride != (*rf).r_stride || n_tile_rows != (*rf).n_tile_rows {
        if !((*rf).r).is_null() {
            dav1d_freep_aligned(&mut (*rf).r as *mut *mut refmvs_block as *mut libc::c_void);
        }
        let uses_2pass: libc::c_int =
            (n_tile_threads > 1i32 && n_frame_threads > 1i32) as libc::c_int;
        (*rf).r = dav1d_alloc_aligned(
            (::core::mem::size_of::<refmvs_block>() as libc::c_ulong)
                .wrapping_mul(35u64)
                .wrapping_mul(r_stride as libc::c_ulong)
                .wrapping_mul(n_tile_rows as libc::c_ulong)
                .wrapping_mul((1i32 + uses_2pass) as libc::c_ulong),
            64u64,
        ) as *mut refmvs_block;
        if ((*rf).r).is_null() {
            return -(12i32);
        }
        (*rf).r_stride = r_stride;
    }
    let rp_stride: ptrdiff_t = r_stride >> 1i32;
    if rp_stride != (*rf).rp_stride || n_tile_rows != (*rf).n_tile_rows {
        if !((*rf).rp_proj).is_null() {
            dav1d_freep_aligned(
                &mut (*rf).rp_proj as *mut *mut refmvs_temporal_block as *mut libc::c_void,
            );
        }
        (*rf).rp_proj = dav1d_alloc_aligned(
            (::core::mem::size_of::<refmvs_temporal_block>() as libc::c_ulong)
                .wrapping_mul(16u64)
                .wrapping_mul(rp_stride as libc::c_ulong)
                .wrapping_mul(n_tile_rows as libc::c_ulong),
            64u64,
        ) as *mut refmvs_temporal_block;
        if ((*rf).rp_proj).is_null() {
            return -(12i32);
        }
        (*rf).rp_stride = rp_stride;
    }
    (*rf).n_tile_rows = n_tile_rows;
    (*rf).n_tile_threads = n_tile_threads;
    (*rf).n_frame_threads = n_frame_threads;
    (*rf).rp = rp;
    (*rf).rp_ref = rp_ref;
    let poc: libc::c_uint = (*frm_hdr).frame_offset as libc::c_uint;
    let mut i: libc::c_int = 0i32;
    while i < 7i32 {
        let poc_diff: libc::c_int = get_poc_diff(
            (*seq_hdr).order_hint_n_bits,
            *ref_poc.offset(i as isize) as libc::c_int,
            poc as libc::c_int,
        );
        (*rf).sign_bias[i as usize] = (poc_diff > 0i32) as uint8_t;
        (*rf).mfmv_sign[i as usize] = (poc_diff < 0i32) as uint8_t;
        (*rf).pocdiff[i as usize] = iclip(
            get_poc_diff(
                (*seq_hdr).order_hint_n_bits,
                poc as libc::c_int,
                *ref_poc.offset(i as isize) as libc::c_int,
            ),
            -(31i32),
            31i32,
        ) as int8_t;
        i += 1;
    }
    (*rf).n_mfmvs = 0i32;
    if (*frm_hdr).use_ref_frame_mvs != 0 && (*seq_hdr).order_hint_n_bits != 0 {
        let mut total: libc::c_int = 2i32;
        if !(*rp_ref.offset(0isize)).is_null()
            && (*ref_ref_poc.offset(0isize))[6usize] != *ref_poc.offset(3isize)
        {
            let fresh12 = (*rf).n_mfmvs;
            (*rf).n_mfmvs = (*rf).n_mfmvs + 1;
            (*rf).mfmv_ref[fresh12 as usize] = 0u8;
            total = 3i32;
        }
        if !(*rp_ref.offset(4isize)).is_null()
            && get_poc_diff(
                (*seq_hdr).order_hint_n_bits,
                *ref_poc.offset(4isize) as libc::c_int,
                (*frm_hdr).frame_offset,
            ) > 0i32
        {
            let fresh13 = (*rf).n_mfmvs;
            (*rf).n_mfmvs = (*rf).n_mfmvs + 1;
            (*rf).mfmv_ref[fresh13 as usize] = 4u8;
        }
        if !(*rp_ref.offset(5isize)).is_null()
            && get_poc_diff(
                (*seq_hdr).order_hint_n_bits,
                *ref_poc.offset(5isize) as libc::c_int,
                (*frm_hdr).frame_offset,
            ) > 0i32
        {
            let fresh14 = (*rf).n_mfmvs;
            (*rf).n_mfmvs = (*rf).n_mfmvs + 1;
            (*rf).mfmv_ref[fresh14 as usize] = 5u8;
        }
        if (*rf).n_mfmvs < total
            && !(*rp_ref.offset(6isize)).is_null()
            && get_poc_diff(
                (*seq_hdr).order_hint_n_bits,
                *ref_poc.offset(6isize) as libc::c_int,
                (*frm_hdr).frame_offset,
            ) > 0i32
        {
            let fresh15 = (*rf).n_mfmvs;
            (*rf).n_mfmvs = (*rf).n_mfmvs + 1;
            (*rf).mfmv_ref[fresh15 as usize] = 6u8;
        }
        if (*rf).n_mfmvs < total && !(*rp_ref.offset(1isize)).is_null() {
            let fresh16 = (*rf).n_mfmvs;
            (*rf).n_mfmvs = (*rf).n_mfmvs + 1;
            (*rf).mfmv_ref[fresh16 as usize] = 1u8;
        }
        let mut n: libc::c_int = 0i32;
        while n < (*rf).n_mfmvs {
            let rpoc: libc::c_uint = *ref_poc.offset((*rf).mfmv_ref[n as usize] as isize);
            let diff1: libc::c_int = get_poc_diff(
                (*seq_hdr).order_hint_n_bits,
                rpoc as libc::c_int,
                (*frm_hdr).frame_offset,
            );
            if abs(diff1) > 31i32 {
                (*rf).mfmv_ref2cur[n as usize] = -(2147483647i32) - 1i32;
            } else {
                (*rf).mfmv_ref2cur[n as usize] =
                    if ((*rf).mfmv_ref[n as usize] as libc::c_int) < 4i32 {
                        -diff1
                    } else {
                        diff1
                    };
                let mut m: libc::c_int = 0i32;
                while m < 7i32 {
                    let rrpoc: libc::c_uint =
                        (*ref_ref_poc.offset((*rf).mfmv_ref[n as usize] as isize))[m as usize];
                    let diff2: libc::c_int = get_poc_diff(
                        (*seq_hdr).order_hint_n_bits,
                        rpoc as libc::c_int,
                        rrpoc as libc::c_int,
                    );
                    (*rf).mfmv_ref2ref[n as usize][m as usize] = if diff2 as libc::c_uint > 31u32 {
                        0i32
                    } else {
                        diff2
                    };
                    m += 1;
                }
            }
            n += 1;
        }
    }
    (*rf).use_ref_frame_mvs = ((*rf).n_mfmvs > 0i32) as libc::c_int;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_refmvs_init(rf: *mut refmvs_frame) {
    (*rf).r = 0 as *mut refmvs_block;
    (*rf).r_stride = 0i64;
    (*rf).rp_proj = 0 as *mut refmvs_temporal_block;
    (*rf).rp_stride = 0i64;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_refmvs_clear(rf: *mut refmvs_frame) {
    if !((*rf).r).is_null() {
        dav1d_freep_aligned(&mut (*rf).r as *mut *mut refmvs_block as *mut libc::c_void);
    }
    if !((*rf).rp_proj).is_null() {
        dav1d_freep_aligned(
            &mut (*rf).rp_proj as *mut *mut refmvs_temporal_block as *mut libc::c_void,
        );
    }
}
unsafe extern "C" fn splat_mv_c(
    mut rr: *mut *mut refmvs_block,
    rmv: *const refmvs_block,
    bx4: libc::c_int,
    bw4: libc::c_int,
    mut bh4: libc::c_int,
) {
    loop {
        let fresh17 = rr;
        rr = rr.offset(1);
        let r: *mut refmvs_block = (*fresh17).offset(bx4 as isize);
        let mut x: libc::c_int = 0i32;
        while x < bw4 {
            *r.offset(x as isize) = *rmv;
            x += 1;
        }
        bh4 -= 1;
        if !(bh4 != 0) {
            break;
        }
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_refmvs_dsp_init(c: *mut Dav1dRefmvsDSPContext) {
    (*c).splat_mv = Some(
        splat_mv_c
            as unsafe extern "C" fn(
                *mut *mut refmvs_block,
                *const refmvs_block,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
}
