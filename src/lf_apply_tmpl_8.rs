use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
use crate::src::cdf::CdfContext;
use crate::src::msac::MsacContext;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}








pub type pixel = uint8_t;
pub type coef = int16_t;
use crate::include::stdatomic::atomic_int;
use crate::include::stdatomic::atomic_uint;
use crate::include::dav1d::common::Dav1dUserData;
use crate::src::r#ref::Dav1dRef;
use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::dav1d::data::Dav1dData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameContext {
    pub seq_hdr_ref: *mut Dav1dRef,
    pub seq_hdr: *mut Dav1dSequenceHeader,
    pub frame_hdr_ref: *mut Dav1dRef,
    pub frame_hdr: *mut Dav1dFrameHeader,
    pub refp: [Dav1dThreadPicture; 7],
    pub cur: Dav1dPicture,
    pub sr_cur: Dav1dThreadPicture,
    pub mvs_ref: *mut Dav1dRef,
    pub mvs: *mut refmvs_temporal_block,
    pub ref_mvs: [*mut refmvs_temporal_block; 7],
    pub ref_mvs_ref: [*mut Dav1dRef; 7],
    pub cur_segmap_ref: *mut Dav1dRef,
    pub prev_segmap_ref: *mut Dav1dRef,
    pub cur_segmap: *mut uint8_t,
    pub prev_segmap: *const uint8_t,
    pub refpoc: [libc::c_uint; 7],
    pub refrefpoc: [[libc::c_uint; 7]; 7],
    pub gmv_warp_allowed: [uint8_t; 7],
    pub in_cdf: CdfThreadContext,
    pub out_cdf: CdfThreadContext,
    pub tile: *mut Dav1dTileGroup,
    pub n_tile_data_alloc: libc::c_int,
    pub n_tile_data: libc::c_int,
    pub svc: [[ScalableMotionParams; 2]; 7],
    pub resize_step: [libc::c_int; 2],
    pub resize_start: [libc::c_int; 2],
    pub c: *const Dav1dContext,
    pub ts: *mut Dav1dTileState,
    pub n_ts: libc::c_int,
    pub dsp: *const Dav1dDSPContext,
    pub bd_fn: C2RustUnnamed_28,
    pub ipred_edge_sz: libc::c_int,
    pub ipred_edge: [*mut pixel; 3],
    pub b4_stride: ptrdiff_t,
    pub w4: libc::c_int,
    pub h4: libc::c_int,
    pub bw: libc::c_int,
    pub bh: libc::c_int,
    pub sb128w: libc::c_int,
    pub sb128h: libc::c_int,
    pub sbh: libc::c_int,
    pub sb_shift: libc::c_int,
    pub sb_step: libc::c_int,
    pub sr_sb128w: libc::c_int,
    pub dq: [[[uint16_t; 2]; 3]; 8],
    pub qm: [[*const uint8_t; 3]; 19],
    pub a: *mut BlockContext,
    pub a_sz: libc::c_int,
    pub rf: refmvs_frame,
    pub jnt_weights: [[uint8_t; 7]; 7],
    pub bitdepth_max: libc::c_int,
    pub frame_thread: C2RustUnnamed_20,
    pub lf: C2RustUnnamed_19,
    pub task_thread: C2RustUnnamed,
    pub tile_thread: FrameTileThreadData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameTileThreadData {
    pub lowest_pixel_mem: *mut [[libc::c_int; 2]; 7],
    pub lowest_pixel_mem_sz: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub lock: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub ttd: *mut TaskThreadData,
    pub tasks: *mut Dav1dTask,
    pub tile_tasks: [*mut Dav1dTask; 2],
    pub init_task: Dav1dTask,
    pub num_tasks: libc::c_int,
    pub num_tile_tasks: libc::c_int,
    pub init_done: atomic_int,
    pub done: [atomic_int; 2],
    pub retval: libc::c_int,
    pub update_set: libc::c_int,
    pub error: atomic_int,
    pub task_counter: atomic_int,
    pub task_head: *mut Dav1dTask,
    pub task_tail: *mut Dav1dTask,
    pub task_cur_prev: *mut Dav1dTask,
    pub pending_tasks: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub merge: atomic_int,
    pub lock: pthread_mutex_t,
    pub head: *mut Dav1dTask,
    pub tail: *mut Dav1dTask,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTask {
    pub frame_idx: libc::c_uint,
    pub type_0: TaskType,
    pub sby: libc::c_int,
    pub recon_progress: libc::c_int,
    pub deblock_progress: libc::c_int,
    pub deps_skip: libc::c_int,
    pub next: *mut Dav1dTask,
}
pub type TaskType = libc::c_uint;
pub const DAV1D_TASK_TYPE_FG_APPLY: TaskType = 12;
pub const DAV1D_TASK_TYPE_FG_PREP: TaskType = 11;
pub const DAV1D_TASK_TYPE_RECONSTRUCTION_PROGRESS: TaskType = 10;
pub const DAV1D_TASK_TYPE_LOOP_RESTORATION: TaskType = 9;
pub const DAV1D_TASK_TYPE_SUPER_RESOLUTION: TaskType = 8;
pub const DAV1D_TASK_TYPE_CDEF: TaskType = 7;
pub const DAV1D_TASK_TYPE_DEBLOCK_ROWS: TaskType = 6;
pub const DAV1D_TASK_TYPE_DEBLOCK_COLS: TaskType = 5;
pub const DAV1D_TASK_TYPE_TILE_RECONSTRUCTION: TaskType = 4;
pub const DAV1D_TASK_TYPE_ENTROPY_PROGRESS: TaskType = 3;
pub const DAV1D_TASK_TYPE_TILE_ENTROPY: TaskType = 2;
pub const DAV1D_TASK_TYPE_INIT_CDF: TaskType = 1;
pub const DAV1D_TASK_TYPE_INIT: TaskType = 0;
use crate::include::pthread::pthread_mutex_t;



#[derive(Copy, Clone)]
#[repr(C)]
pub struct TaskThreadData {
    pub lock: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub first: atomic_uint,
    pub cur: libc::c_uint,
    pub reset_task_cur: atomic_uint,
    pub cond_signaled: atomic_int,
    pub delayed_fg: C2RustUnnamed_1,
    pub inited: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub exec: libc::c_int,
    pub cond: pthread_cond_t,
    pub in_0: *const Dav1dPicture,
    pub out: *mut Dav1dPicture,
    pub type_0: TaskType,
    pub progress: [atomic_int; 2],
    pub c2rust_unnamed: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub c2rust_unnamed: C2RustUnnamed_4,
    pub c2rust_unnamed_0: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub grain_lut_16bpc: [[[int16_t; 82]; 74]; 3],
    pub scaling_16bpc: [[uint8_t; 4096]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub grain_lut_8bpc: [[[int8_t; 82]; 74]; 3],
    pub scaling_8bpc: [[uint8_t; 256]; 3],
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
use crate::include::dav1d::headers::Dav1dITUTT35;
use crate::include::dav1d::headers::Dav1dMasteringDisplay;
use crate::include::dav1d::headers::Dav1dContentLightLevel;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dPictureParameters {
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub layout: Dav1dPixelLayout,
    pub bpc: libc::c_int,
}
use crate::include::dav1d::headers::Dav1dPixelLayout;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I422;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I400;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader {
    pub film_grain: C2RustUnnamed_17,
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
    pub super_res: C2RustUnnamed_16,
    pub have_render_size: libc::c_int,
    pub allow_intrabc: libc::c_int,
    pub frame_ref_short_signaling: libc::c_int,
    pub refidx: [libc::c_int; 7],
    pub hp: libc::c_int,
    pub subpel_filter_mode: Dav1dFilterMode,
    pub switchable_motion_mode: libc::c_int,
    pub use_ref_frame_mvs: libc::c_int,
    pub refresh_context: libc::c_int,
    pub tiling: C2RustUnnamed_15,
    pub quant: C2RustUnnamed_14,
    pub segmentation: C2RustUnnamed_13,
    pub delta: C2RustUnnamed_10,
    pub all_lossless: libc::c_int,
    pub loopfilter: C2RustUnnamed_9,
    pub cdef: C2RustUnnamed_8,
    pub restoration: C2RustUnnamed_7,
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
pub struct Dav1dWarpedMotionParams {
    pub type_0: Dav1dWarpedMotionType,
    pub matrix: [int32_t; 6],
    pub u: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub p: C2RustUnnamed_6,
    pub abcd: [int16_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub alpha: int16_t,
    pub beta: int16_t,
    pub gamma: int16_t,
    pub delta: int16_t,
}
use crate::include::dav1d::headers::Dav1dWarpedMotionType;
use crate::include::dav1d::headers::DAV1D_WM_TYPE_AFFINE;
use crate::include::dav1d::headers::DAV1D_WM_TYPE_ROT_ZOOM;
use crate::include::dav1d::headers::DAV1D_WM_TYPE_TRANSLATION;
use crate::include::dav1d::headers::DAV1D_WM_TYPE_IDENTITY;
use crate::include::dav1d::headers::Dav1dTxfmMode;
use crate::include::dav1d::headers::DAV1D_N_TX_MODES;
use crate::include::dav1d::headers::DAV1D_TX_SWITCHABLE;
use crate::include::dav1d::headers::DAV1D_TX_LARGEST;
use crate::include::dav1d::headers::DAV1D_TX_4X4_ONLY;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub type_0: [Dav1dRestorationType; 3],
    pub unit_size: [libc::c_int; 2],
}
use crate::include::dav1d::headers::Dav1dRestorationType;
use crate::include::dav1d::headers::DAV1D_RESTORATION_SGRPROJ;
use crate::include::dav1d::headers::DAV1D_RESTORATION_WIENER;
use crate::include::dav1d::headers::DAV1D_RESTORATION_SWITCHABLE;
use crate::include::dav1d::headers::DAV1D_RESTORATION_NONE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub damping: libc::c_int,
    pub n_bits: libc::c_int,
    pub y_strength: [libc::c_int; 8],
    pub uv_strength: [libc::c_int; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub level_y: [libc::c_int; 2],
    pub level_u: libc::c_int,
    pub level_v: libc::c_int,
    pub mode_ref_delta_enabled: libc::c_int,
    pub mode_ref_delta_update: libc::c_int,
    pub mode_ref_deltas: Dav1dLoopfilterModeRefDeltas,
    pub sharpness: libc::c_int,
}
use crate::include::dav1d::headers::Dav1dLoopfilterModeRefDeltas;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub q: C2RustUnnamed_12,
    pub lf: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub present: libc::c_int,
    pub res_log2: libc::c_int,
    pub multi: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub present: libc::c_int,
    pub res_log2: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub enabled: libc::c_int,
    pub update_map: libc::c_int,
    pub temporal: libc::c_int,
    pub update_data: libc::c_int,
    pub seg_data: Dav1dSegmentationDataSet,
    pub lossless: [libc::c_int; 8],
    pub qidx: [libc::c_int; 8],
}
use crate::include::dav1d::headers::Dav1dSegmentationDataSet;
use crate::include::dav1d::headers::Dav1dSegmentationData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
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
pub struct C2RustUnnamed_15 {
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
use crate::include::dav1d::headers::Dav1dFilterMode;
use crate::include::dav1d::headers::DAV1D_FILTER_SWITCHABLE;
use crate::include::dav1d::headers::DAV1D_N_FILTERS;
use crate::include::dav1d::headers::DAV1D_FILTER_BILINEAR;
use crate::include::dav1d::headers::DAV1D_N_SWITCHABLE_FILTERS;
use crate::include::dav1d::headers::DAV1D_FILTER_8TAP_SHARP;
use crate::include::dav1d::headers::DAV1D_FILTER_8TAP_SMOOTH;
use crate::include::dav1d::headers::DAV1D_FILTER_8TAP_REGULAR;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub width_scale_denominator: libc::c_int,
    pub enabled: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameHeaderOperatingPoint {
    pub buffer_removal_time: libc::c_int,
}
use crate::include::dav1d::headers::Dav1dFrameType;
use crate::include::dav1d::headers::DAV1D_FRAME_TYPE_SWITCH;
use crate::include::dav1d::headers::DAV1D_FRAME_TYPE_INTRA;
use crate::include::dav1d::headers::DAV1D_FRAME_TYPE_INTER;
use crate::include::dav1d::headers::DAV1D_FRAME_TYPE_KEY;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub data: Dav1dFilmGrainData,
    pub present: libc::c_int,
    pub update: libc::c_int,
}
use crate::include::dav1d::headers::Dav1dFilmGrainData;
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
use crate::include::dav1d::headers::Dav1dAdaptiveBoolean;
use crate::include::dav1d::headers::DAV1D_ADAPTIVE;
use crate::include::dav1d::headers::DAV1D_ON;
use crate::include::dav1d::headers::DAV1D_OFF;
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
pub type Dav1dChromaSamplePosition = libc::c_uint;
pub const DAV1D_CHR_COLOCATED: Dav1dChromaSamplePosition = 2;
pub const DAV1D_CHR_VERTICAL: Dav1dChromaSamplePosition = 1;
pub const DAV1D_CHR_UNKNOWN: Dav1dChromaSamplePosition = 0;
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
use crate::include::pthread::pthread_cond_t;



#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub level: *mut [uint8_t; 4],
    pub mask: *mut Av1Filter,
    pub lr_mask: *mut Av1Restoration,
    pub mask_sz: libc::c_int,
    pub lr_mask_sz: libc::c_int,
    pub cdef_buf_plane_sz: [libc::c_int; 2],
    pub cdef_buf_sbh: libc::c_int,
    pub lr_buf_plane_sz: [libc::c_int; 2],
    pub re_sz: libc::c_int,
    pub lim_lut: Av1FilterLUT,
    pub last_sharpness: libc::c_int,
    pub lvl: [[[[uint8_t; 2]; 8]; 4]; 8],
    pub tx_lpf_right_edge: [*mut uint8_t; 2],
    pub cdef_line_buf: *mut uint8_t,
    pub lr_line_buf: *mut uint8_t,
    pub cdef_line: [[*mut pixel; 3]; 2],
    pub cdef_lpf_line: [*mut pixel; 3],
    pub lr_lpf_line: [*mut pixel; 3],
    pub start_of_tile_row: *mut uint8_t,
    pub start_of_tile_row_sz: libc::c_int,
    pub need_cdef_lpf_copy: libc::c_int,
    pub p: [*mut pixel; 3],
    pub sr_p: [*mut pixel; 3],
    pub mask_ptr: *mut Av1Filter,
    pub prev_mask_ptr: *mut Av1Filter,
    pub restore_planes: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Av1Filter {
    pub filter_y: [[[[uint16_t; 2]; 3]; 32]; 2],
    pub filter_uv: [[[[uint16_t; 2]; 2]; 32]; 2],
    pub cdef_idx: [int8_t; 4],
    pub noskip_mask: [[uint16_t; 2]; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Av1FilterLUT {
    pub e: [uint8_t; 64],
    pub i: [uint8_t; 64],
    pub sharp: [uint64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Av1Restoration {
    pub lr: [[Av1RestorationUnit; 4]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Av1RestorationUnit {
    pub type_0: uint8_t,
    pub filter_h: [int8_t; 3],
    pub filter_v: [int8_t; 3],
    pub sgr_idx: uint8_t,
    pub sgr_weights: [int8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub next_tile_row: [libc::c_int; 2],
    pub entropy_progress: atomic_int,
    pub deblock_progress: atomic_int,
    pub frame_progress: *mut atomic_uint,
    pub copy_lpf_progress: *mut atomic_uint,
    pub b: *mut Av1Block,
    pub cbi: *mut CodedBlockInfo,
    pub pal: *mut [[uint16_t; 8]; 3],
    pub pal_idx: *mut uint8_t,
    pub cf: *mut coef,
    pub prog_sz: libc::c_int,
    pub pal_sz: libc::c_int,
    pub pal_idx_sz: libc::c_int,
    pub cf_sz: libc::c_int,
    pub tile_start_off: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CodedBlockInfo {
    pub eob: [int16_t; 3],
    pub txtp: [uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Av1Block {
    pub bl: uint8_t,
    pub bs: uint8_t,
    pub bp: uint8_t,
    pub intra: uint8_t,
    pub seg_id: uint8_t,
    pub skip_mode: uint8_t,
    pub skip: uint8_t,
    pub uvtx: uint8_t,
    pub c2rust_unnamed: C2RustUnnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_21 {
    pub c2rust_unnamed: C2RustUnnamed_27,
    pub c2rust_unnamed_0: C2RustUnnamed_22,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub c2rust_unnamed: C2RustUnnamed_23,
    pub comp_type: uint8_t,
    pub inter_mode: uint8_t,
    pub motion_mode: uint8_t,
    pub drl_idx: uint8_t,
    pub ref_0: [int8_t; 2],
    pub max_ytx: uint8_t,
    pub filter2d: uint8_t,
    pub interintra_type: uint8_t,
    pub tx_split0: uint8_t,
    pub tx_split1: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_23 {
    pub c2rust_unnamed: C2RustUnnamed_26,
    pub c2rust_unnamed_0: C2RustUnnamed_24,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub mv2d: mv,
    pub matrix: [int16_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union mv {
    pub c2rust_unnamed: C2RustUnnamed_25,
    pub n: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub y: int16_t,
    pub x: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub mv: [mv; 2],
    pub wedge_idx: uint8_t,
    pub mask_sign: uint8_t,
    pub interintra_mode: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub y_mode: uint8_t,
    pub uv_mode: uint8_t,
    pub tx: uint8_t,
    pub pal_sz: [uint8_t; 2],
    pub y_angle: int8_t,
    pub uv_angle: int8_t,
    pub cfl_alpha: [int8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
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
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct refmvs_block {
    pub mv: refmvs_mvpair,
    pub ref_0: refmvs_refpair,
    pub bs: uint8_t,
    pub mf: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union refmvs_refpair {
    pub ref_0: [int8_t; 2],
    pub pair: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union refmvs_mvpair {
    pub mv: [mv; 2],
    pub n: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct refmvs_temporal_block {
    pub mv: mv,
    pub ref_0: int8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockContext {
    pub mode: [uint8_t; 32],
    pub lcoef: [uint8_t; 32],
    pub ccoef: [[uint8_t; 32]; 2],
    pub seg_pred: [uint8_t; 32],
    pub skip: [uint8_t; 32],
    pub skip_mode: [uint8_t; 32],
    pub intra: [uint8_t; 32],
    pub comp_type: [uint8_t; 32],
    pub ref_0: [[int8_t; 32]; 2],
    pub filter: [[uint8_t; 32]; 2],
    pub tx_intra: [int8_t; 32],
    pub tx: [int8_t; 32],
    pub tx_lpf_y: [uint8_t; 32],
    pub tx_lpf_uv: [uint8_t; 32],
    pub partition: [uint8_t; 16],
    pub uvmode: [uint8_t; 32],
    pub pal_sz: [uint8_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub recon_b_intra: recon_b_intra_fn,
    pub recon_b_inter: recon_b_inter_fn,
    pub filter_sbrow: filter_sbrow_fn,
    pub filter_sbrow_deblock_cols: filter_sbrow_fn,
    pub filter_sbrow_deblock_rows: filter_sbrow_fn,
    pub filter_sbrow_cdef: Option::<
        unsafe extern "C" fn(*mut Dav1dTaskContext, libc::c_int) -> (),
    >,
    pub filter_sbrow_resize: filter_sbrow_fn,
    pub filter_sbrow_lr: filter_sbrow_fn,
    pub backup_ipred_edge: backup_ipred_edge_fn,
    pub read_coef_blocks: read_coef_blocks_fn,
}
pub type read_coef_blocks_fn = Option::<
    unsafe extern "C" fn(*mut Dav1dTaskContext, BlockSize, *const Av1Block) -> (),
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTaskContext {
    pub c: *const Dav1dContext,
    pub f: *const Dav1dFrameContext,
    pub ts: *mut Dav1dTileState,
    pub bx: libc::c_int,
    pub by: libc::c_int,
    pub l: BlockContext,
    pub a: *mut BlockContext,
    pub rt: refmvs_tile,
    pub c2rust_unnamed: C2RustUnnamed_42,
    pub al_pal: [[[[uint16_t; 8]; 3]; 32]; 2],
    pub pal_sz_uv: [[uint8_t; 32]; 2],
    pub txtp_map: [uint8_t; 1024],
    pub scratch: C2RustUnnamed_31,
    pub warpmv: Dav1dWarpedMotionParams,
    pub lf_mask: *mut Av1Filter,
    pub top_pre_cdef_toggle: libc::c_int,
    pub cur_sb_cdef_idx_ptr: *mut int8_t,
    pub tl_4x4_filter: Filter2d,
    pub frame_thread: C2RustUnnamed_30,
    pub task_thread: C2RustUnnamed_29,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub td: thread_data,
    pub ttd: *mut TaskThreadData,
    pub fttd: *mut FrameTileThreadData,
    pub flushed: libc::c_int,
    pub die: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thread_data {
    pub thread: pthread_t,
    pub cond: pthread_cond_t,
    pub lock: pthread_mutex_t,
    pub inited: libc::c_int,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
    pub pass: libc::c_int,
}
pub type Filter2d = libc::c_uint;
pub const N_2D_FILTERS: Filter2d = 10;
pub const FILTER_2D_BILINEAR: Filter2d = 9;
pub const FILTER_2D_8TAP_SMOOTH_SHARP: Filter2d = 8;
pub const FILTER_2D_8TAP_SMOOTH: Filter2d = 7;
pub const FILTER_2D_8TAP_SMOOTH_REGULAR: Filter2d = 6;
pub const FILTER_2D_8TAP_SHARP: Filter2d = 5;
pub const FILTER_2D_8TAP_SHARP_SMOOTH: Filter2d = 4;
pub const FILTER_2D_8TAP_SHARP_REGULAR: Filter2d = 3;
pub const FILTER_2D_8TAP_REGULAR_SHARP: Filter2d = 2;
pub const FILTER_2D_8TAP_REGULAR_SMOOTH: Filter2d = 1;
pub const FILTER_2D_8TAP_REGULAR: Filter2d = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_31 {
    pub c2rust_unnamed: C2RustUnnamed_38,
    pub c2rust_unnamed_0: C2RustUnnamed_32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_32 {
    pub c2rust_unnamed: C2RustUnnamed_36,
    pub ac: [int16_t; 1024],
    pub pal_idx: [uint8_t; 8192],
    pub pal: [[uint16_t; 8]; 3],
    pub c2rust_unnamed_0: C2RustUnnamed_33,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_33 {
    pub c2rust_unnamed: C2RustUnnamed_35,
    pub c2rust_unnamed_0: C2RustUnnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub interintra_16bpc: [uint16_t; 4096],
    pub edge_16bpc: [uint16_t; 257],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub interintra_8bpc: [uint8_t; 4096],
    pub edge_8bpc: [uint8_t; 257],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_36 {
    pub levels: [uint8_t; 1088],
    pub c2rust_unnamed: C2RustUnnamed_37,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_37 {
    pub pal_order: [[uint8_t; 8]; 64],
    pub pal_ctx: [uint8_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_38 {
    pub c2rust_unnamed: C2RustUnnamed_40,
    pub c2rust_unnamed_0: C2RustUnnamed_39,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_39 {
    pub emu_edge_8bpc: [uint8_t; 84160],
    pub emu_edge_16bpc: [uint16_t; 84160],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_40 {
    pub lap_8bpc: [uint8_t; 4096],
    pub lap_16bpc: [uint16_t; 4096],
    pub c2rust_unnamed: C2RustUnnamed_41,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_41 {
    pub compinter: [[int16_t; 16384]; 2],
    pub seg_mask: [uint8_t; 16384],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_42 {
    pub cf_8bpc: [int16_t; 1024],
    pub cf_16bpc: [int32_t; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct refmvs_tile {
    pub rf: *const refmvs_frame,
    pub r: [*mut refmvs_block; 37],
    pub rp_proj: *mut refmvs_temporal_block,
    pub tile_col: C2RustUnnamed_43,
    pub tile_row: C2RustUnnamed_43,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_43 {
    pub start: libc::c_int,
    pub end: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTileState {
    pub cdf: CdfContext,
    pub msac: MsacContext,
    pub tiling: C2RustUnnamed_45,
    pub progress: [atomic_int; 2],
    pub frame_thread: [C2RustUnnamed_44; 2],
    pub lowest_pixel: *mut [[libc::c_int; 2]; 7],
    pub dqmem: [[[uint16_t; 2]; 3]; 8],
    pub dq: *const [[uint16_t; 2]; 3],
    pub last_qidx: libc::c_int,
    pub last_delta_lf: [int8_t; 4],
    pub lflvlmem: [[[[uint8_t; 2]; 8]; 4]; 8],
    pub lflvl: *const [[[uint8_t; 2]; 8]; 4],
    pub lr_ref: [*mut Av1RestorationUnit; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_44 {
    pub pal_idx: *mut uint8_t,
    pub cf: *mut coef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_45 {
    pub col_start: libc::c_int,
    pub col_end: libc::c_int,
    pub row_start: libc::c_int,
    pub row_end: libc::c_int,
    pub col: libc::c_int,
    pub row: libc::c_int,
}
pub type ec_win = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dContext {
    pub fc: *mut Dav1dFrameContext,
    pub n_fc: libc::c_uint,
    pub tc: *mut Dav1dTaskContext,
    pub n_tc: libc::c_uint,
    pub tile: *mut Dav1dTileGroup,
    pub n_tile_data_alloc: libc::c_int,
    pub n_tile_data: libc::c_int,
    pub n_tiles: libc::c_int,
    pub seq_hdr_pool: *mut Dav1dMemPool,
    pub seq_hdr_ref: *mut Dav1dRef,
    pub seq_hdr: *mut Dav1dSequenceHeader,
    pub frame_hdr_pool: *mut Dav1dMemPool,
    pub frame_hdr_ref: *mut Dav1dRef,
    pub frame_hdr: *mut Dav1dFrameHeader,
    pub content_light_ref: *mut Dav1dRef,
    pub content_light: *mut Dav1dContentLightLevel,
    pub mastering_display_ref: *mut Dav1dRef,
    pub mastering_display: *mut Dav1dMasteringDisplay,
    pub itut_t35_ref: *mut Dav1dRef,
    pub itut_t35: *mut Dav1dITUTT35,
    pub in_0: Dav1dData,
    pub out: Dav1dThreadPicture,
    pub cache: Dav1dThreadPicture,
    pub flush_mem: atomic_int,
    pub flush: *mut atomic_int,
    pub frame_thread: C2RustUnnamed_50,
    pub task_thread: TaskThreadData,
    pub segmap_pool: *mut Dav1dMemPool,
    pub refmvs_pool: *mut Dav1dMemPool,
    pub refs: [C2RustUnnamed_49; 8],
    pub cdf_pool: *mut Dav1dMemPool,
    pub cdf: [CdfThreadContext; 8],
    pub dsp: [Dav1dDSPContext; 3],
    pub refmvs_dsp: Dav1dRefmvsDSPContext,
    pub intra_edge: C2RustUnnamed_46,
    pub allocator: Dav1dPicAllocator,
    pub apply_grain: libc::c_int,
    pub operating_point: libc::c_int,
    pub operating_point_idc: libc::c_uint,
    pub all_layers: libc::c_int,
    pub max_spatial_id: libc::c_int,
    pub frame_size_limit: libc::c_uint,
    pub strict_std_compliance: libc::c_int,
    pub output_invisible_frames: libc::c_int,
    pub inloop_filters: Dav1dInloopFilterType,
    pub decode_frame_type: Dav1dDecodeFrameType,
    pub drain: libc::c_int,
    pub frame_flags: PictureFlags,
    pub event_flags: Dav1dEventFlags,
    pub cached_error_props: Dav1dDataProps,
    pub cached_error: libc::c_int,
    pub logger: Dav1dLogger,
    pub picture_pool: *mut Dav1dMemPool,
}
use crate::src::mem::Dav1dMemPool;
use crate::src::mem::Dav1dMemPoolBuffer;
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
pub type Dav1dEventFlags = libc::c_uint;
pub const DAV1D_EVENT_FLAG_NEW_OP_PARAMS_INFO: Dav1dEventFlags = 2;
pub const DAV1D_EVENT_FLAG_NEW_SEQUENCE: Dav1dEventFlags = 1;
pub type PictureFlags = libc::c_uint;
pub const PICTURE_FLAG_NEW_TEMPORAL_UNIT: PictureFlags = 4;
pub const PICTURE_FLAG_NEW_OP_PARAMS_INFO: PictureFlags = 2;
pub const PICTURE_FLAG_NEW_SEQUENCE: PictureFlags = 1;
pub type Dav1dDecodeFrameType = libc::c_uint;
pub const DAV1D_DECODEFRAMETYPE_KEY: Dav1dDecodeFrameType = 3;
pub const DAV1D_DECODEFRAMETYPE_INTRA: Dav1dDecodeFrameType = 2;
pub const DAV1D_DECODEFRAMETYPE_REFERENCE: Dav1dDecodeFrameType = 1;
pub const DAV1D_DECODEFRAMETYPE_ALL: Dav1dDecodeFrameType = 0;
pub type Dav1dInloopFilterType = libc::c_uint;
pub const DAV1D_INLOOPFILTER_ALL: Dav1dInloopFilterType = 7;
pub const DAV1D_INLOOPFILTER_RESTORATION: Dav1dInloopFilterType = 4;
pub const DAV1D_INLOOPFILTER_CDEF: Dav1dInloopFilterType = 2;
pub const DAV1D_INLOOPFILTER_DEBLOCK: Dav1dInloopFilterType = 1;
pub const DAV1D_INLOOPFILTER_NONE: Dav1dInloopFilterType = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_46 {
    pub root: [*mut EdgeNode; 2],
    pub branch_sb128: [EdgeBranch; 85],
    pub branch_sb64: [EdgeBranch; 21],
    pub tip_sb128: [EdgeTip; 256],
    pub tip_sb64: [EdgeTip; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EdgeTip {
    pub node: EdgeNode,
    pub split: [EdgeFlags; 4],
}
pub type EdgeFlags = libc::c_uint;
pub const EDGE_I420_LEFT_HAS_BOTTOM: EdgeFlags = 32;
pub const EDGE_I422_LEFT_HAS_BOTTOM: EdgeFlags = 16;
pub const EDGE_I444_LEFT_HAS_BOTTOM: EdgeFlags = 8;
pub const EDGE_I420_TOP_HAS_RIGHT: EdgeFlags = 4;
pub const EDGE_I422_TOP_HAS_RIGHT: EdgeFlags = 2;
pub const EDGE_I444_TOP_HAS_RIGHT: EdgeFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EdgeNode {
    pub o: EdgeFlags,
    pub h: [EdgeFlags; 2],
    pub v: [EdgeFlags; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EdgeBranch {
    pub node: EdgeNode,
    pub tts: [EdgeFlags; 3],
    pub tbs: [EdgeFlags; 3],
    pub tls: [EdgeFlags; 3],
    pub trs: [EdgeFlags; 3],
    pub h4: [EdgeFlags; 4],
    pub v4: [EdgeFlags; 4],
    pub split: [*mut EdgeNode; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dRefmvsDSPContext {
    pub splat_mv: splat_mv_fn,
}
pub type splat_mv_fn = Option::<
    unsafe extern "C" fn(
        *mut *mut refmvs_block,
        *const refmvs_block,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dDSPContext {
    pub fg: Dav1dFilmGrainDSPContext,
    pub ipred: Dav1dIntraPredDSPContext,
    pub mc: Dav1dMCDSPContext,
    pub itx: Dav1dInvTxfmDSPContext,
    pub lf: Dav1dLoopFilterDSPContext,
    pub cdef: Dav1dCdefDSPContext,
    pub lr: Dav1dLoopRestorationDSPContext,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dLoopRestorationDSPContext {
    pub wiener: [looprestorationfilter_fn; 2],
    pub sgr: [looprestorationfilter_fn; 3],
}
pub type looprestorationfilter_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        const_left_pixel_row,
        *const pixel,
        libc::c_int,
        libc::c_int,
        *const LooprestorationParams,
        LrEdgeFlags,
    ) -> (),
>;
pub type LrEdgeFlags = libc::c_uint;
pub const LR_HAVE_BOTTOM: LrEdgeFlags = 8;
pub const LR_HAVE_TOP: LrEdgeFlags = 4;
pub const LR_HAVE_RIGHT: LrEdgeFlags = 2;
pub const LR_HAVE_LEFT: LrEdgeFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union LooprestorationParams {
    pub filter: [[int16_t; 8]; 2],
    pub sgr: C2RustUnnamed_47,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_47 {
    pub s0: uint32_t,
    pub s1: uint32_t,
    pub w0: int16_t,
    pub w1: int16_t,
}
pub type const_left_pixel_row = *const [pixel; 4];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dCdefDSPContext {
    pub dir: cdef_dir_fn,
    pub fb: [cdef_fn; 3],
}
pub type cdef_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        const_left_pixel_row_2px,
        *const pixel,
        *const pixel,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        CdefEdgeFlags,
    ) -> (),
>;
pub type CdefEdgeFlags = libc::c_uint;
pub const CDEF_HAVE_BOTTOM: CdefEdgeFlags = 8;
pub const CDEF_HAVE_TOP: CdefEdgeFlags = 4;
pub const CDEF_HAVE_RIGHT: CdefEdgeFlags = 2;
pub const CDEF_HAVE_LEFT: CdefEdgeFlags = 1;
pub type const_left_pixel_row_2px = *const [pixel; 2];
pub type cdef_dir_fn = Option::<
    unsafe extern "C" fn(*const pixel, ptrdiff_t, *mut libc::c_uint) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dLoopFilterDSPContext {
    pub loop_filter_sb: [[loopfilter_sb_fn; 2]; 2],
}
pub type loopfilter_sb_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const uint32_t,
        *const [uint8_t; 4],
        ptrdiff_t,
        *const Av1FilterLUT,
        libc::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dInvTxfmDSPContext {
    pub itxfm_add: [[itxfm_fn; 17]; 19],
}
pub type itxfm_fn = Option::<
    unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dMCDSPContext {
    pub mc: [mc_fn; 10],
    pub mc_scaled: [mc_scaled_fn; 10],
    pub mct: [mct_fn; 10],
    pub mct_scaled: [mct_scaled_fn; 10],
    pub avg: avg_fn,
    pub w_avg: w_avg_fn,
    pub mask: mask_fn,
    pub w_mask: [w_mask_fn; 3],
    pub blend: blend_fn,
    pub blend_v: blend_dir_fn,
    pub blend_h: blend_dir_fn,
    pub warp8x8: warp8x8_fn,
    pub warp8x8t: warp8x8t_fn,
    pub emu_edge: emu_edge_fn,
    pub resize: resize_fn,
}
pub type resize_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type emu_edge_fn = Option::<
    unsafe extern "C" fn(
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
    ) -> (),
>;
pub type warp8x8t_fn = Option::<
    unsafe extern "C" fn(
        *mut int16_t,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type warp8x8_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type blend_dir_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type blend_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        libc::c_int,
        libc::c_int,
        *const uint8_t,
    ) -> (),
>;
pub type w_mask_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        *mut uint8_t,
        libc::c_int,
    ) -> (),
>;
pub type mask_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        *const uint8_t,
    ) -> (),
>;
pub type w_avg_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type avg_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mct_scaled_fn = Option::<
    unsafe extern "C" fn(
        *mut int16_t,
        *const pixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mct_fn = Option::<
    unsafe extern "C" fn(
        *mut int16_t,
        *const pixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mc_scaled_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mc_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dIntraPredDSPContext {
    pub intra_pred: [angular_ipred_fn; 14],
    pub cfl_ac: [cfl_ac_fn; 3],
    pub cfl_pred: [cfl_pred_fn; 6],
    pub pal_pred: pal_pred_fn,
}
pub type pal_pred_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const uint16_t,
        *const uint8_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type cfl_pred_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        libc::c_int,
        libc::c_int,
        *const int16_t,
        libc::c_int,
    ) -> (),
>;
pub type cfl_ac_fn = Option::<
    unsafe extern "C" fn(
        *mut int16_t,
        *const pixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type angular_ipred_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        libc::c_int,
        libc::c_int,
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
    ) -> (),
>;
pub type entry = int8_t;
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
    ) -> (),
>;
pub type generate_grain_uv_fn = Option::<
    unsafe extern "C" fn(
        *mut [entry; 82],
        *const [entry; 82],
        *const Dav1dFilmGrainData,
        intptr_t,
    ) -> (),
>;
pub type generate_grain_y_fn = Option::<
    unsafe extern "C" fn(*mut [entry; 82], *const Dav1dFilmGrainData) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdfThreadContext {
    pub ref_0: *mut Dav1dRef,
    pub data: C2RustUnnamed_48,
    pub progress: *mut atomic_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_48 {
    pub cdf: *mut CdfContext,
    pub qcat: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_49 {
    pub p: Dav1dThreadPicture,
    pub segmap: *mut Dav1dRef,
    pub refmvs: *mut Dav1dRef,
    pub refpoc: [libc::c_uint; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dThreadPicture {
    pub p: Dav1dPicture,
    pub visible: libc::c_int,
    pub showable: libc::c_int,
    pub flags: PictureFlags,
    pub progress: *mut atomic_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_50 {
    pub out_delayed: *mut Dav1dThreadPicture,
    pub next: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTileGroup {
    pub data: Dav1dData,
    pub start: libc::c_int,
    pub end: libc::c_int,
}
pub type backup_ipred_edge_fn = Option::<
    unsafe extern "C" fn(*mut Dav1dTaskContext) -> (),
>;
pub type filter_sbrow_fn = Option::<
    unsafe extern "C" fn(*mut Dav1dFrameContext, libc::c_int) -> (),
>;
pub type recon_b_inter_fn = Option::<
    unsafe extern "C" fn(
        *mut Dav1dTaskContext,
        BlockSize,
        *const Av1Block,
    ) -> libc::c_int,
>;
pub type recon_b_intra_fn = Option::<
    unsafe extern "C" fn(
        *mut Dav1dTaskContext,
        BlockSize,
        EdgeFlags,
        *const Av1Block,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScalableMotionParams {
    pub scale: libc::c_int,
    pub step: libc::c_int,
}
pub const LR_RESTORE_V: LrRestorePlanes = 4;
pub const LR_RESTORE_U: LrRestorePlanes = 2;
pub const LR_RESTORE_Y: LrRestorePlanes = 1;
pub type LrRestorePlanes = libc::c_uint;
#[inline]
unsafe extern "C" fn imin(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
unsafe extern "C" fn backup_lpf(
    f: *const Dav1dFrameContext,
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut src: *const pixel,
    src_stride: ptrdiff_t,
    ss_ver: libc::c_int,
    sb128: libc::c_int,
    mut row: libc::c_int,
    row_h: libc::c_int,
    src_w: libc::c_int,
    h: libc::c_int,
    ss_hor: libc::c_int,
    lr_backup: libc::c_int,
) {
    let cdef_backup: libc::c_int = (lr_backup == 0) as libc::c_int;
    let dst_w: libc::c_int = if (*(*f).frame_hdr).super_res.enabled != 0 {
        (*(*f).frame_hdr).width[1 as libc::c_int as usize] + ss_hor >> ss_hor
    } else {
        src_w
    };
    let mut stripe_h: libc::c_int = ((64 as libc::c_int) << (cdef_backup & sb128))
        - 8 as libc::c_int * (row == 0) as libc::c_int >> ss_ver;
    src = src
        .offset(((stripe_h - 2 as libc::c_int) as libc::c_long * src_stride) as isize);
    if (*(*f).c).n_tc == 1 as libc::c_int as libc::c_uint {
        if row != 0 {
            let top: libc::c_int = (4 as libc::c_int) << sb128;
            memcpy(
                &mut *dst
                    .offset((dst_stride * 0 as libc::c_int as libc::c_long) as isize)
                    as *mut pixel as *mut libc::c_void,
                &mut *dst.offset((dst_stride * top as libc::c_long) as isize)
                    as *mut pixel as *const libc::c_void,
                dst_w as libc::c_ulong,
            );
            memcpy(
                &mut *dst
                    .offset((dst_stride * 1 as libc::c_int as libc::c_long) as isize)
                    as *mut pixel as *mut libc::c_void,
                &mut *dst
                    .offset(
                        (dst_stride * (top + 1 as libc::c_int) as libc::c_long) as isize,
                    ) as *mut pixel as *const libc::c_void,
                dst_w as libc::c_ulong,
            );
            memcpy(
                &mut *dst
                    .offset((dst_stride * 2 as libc::c_int as libc::c_long) as isize)
                    as *mut pixel as *mut libc::c_void,
                &mut *dst
                    .offset(
                        (dst_stride * (top + 2 as libc::c_int) as libc::c_long) as isize,
                    ) as *mut pixel as *const libc::c_void,
                dst_w as libc::c_ulong,
            );
            memcpy(
                &mut *dst
                    .offset((dst_stride * 3 as libc::c_int as libc::c_long) as isize)
                    as *mut pixel as *mut libc::c_void,
                &mut *dst
                    .offset(
                        (dst_stride * (top + 3 as libc::c_int) as libc::c_long) as isize,
                    ) as *mut pixel as *const libc::c_void,
                dst_w as libc::c_ulong,
            );
        }
        dst = dst.offset((4 as libc::c_int as libc::c_long * dst_stride) as isize);
    }
    if lr_backup != 0
        && (*(*f).frame_hdr).width[0 as libc::c_int as usize]
            != (*(*f).frame_hdr).width[1 as libc::c_int as usize]
    {
        while row + stripe_h <= row_h {
            let n_lines: libc::c_int = 4 as libc::c_int
                - (row + stripe_h + 1 as libc::c_int == h) as libc::c_int;
            ((*(*f).dsp).mc.resize)
                .expect(
                    "non-null function pointer",
                )(
                dst,
                dst_stride,
                src,
                src_stride,
                dst_w,
                n_lines,
                src_w,
                (*f).resize_step[ss_hor as usize],
                (*f).resize_start[ss_hor as usize],
            );
            row += stripe_h;
            stripe_h = 64 as libc::c_int >> ss_ver;
            src = src.offset((stripe_h as libc::c_long * src_stride) as isize);
            dst = dst.offset((n_lines as libc::c_long * dst_stride) as isize);
            if n_lines == 3 as libc::c_int {
                memcpy(
                    dst as *mut libc::c_void,
                    &mut *dst.offset(-dst_stride as isize) as *mut pixel
                        as *const libc::c_void,
                    dst_w as libc::c_ulong,
                );
                dst = dst.offset(dst_stride as isize);
            }
        }
    } else {
        while row + stripe_h <= row_h {
            let n_lines_0: libc::c_int = 4 as libc::c_int
                - (row + stripe_h + 1 as libc::c_int == h) as libc::c_int;
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < 4 as libc::c_int {
                memcpy(
                    dst as *mut libc::c_void,
                    (if i == n_lines_0 {
                        &mut *dst.offset(-dst_stride as isize) as *mut pixel
                            as *const pixel
                    } else {
                        src
                    }) as *const libc::c_void,
                    src_w as libc::c_ulong,
                );
                dst = dst.offset(dst_stride as isize);
                src = src.offset(src_stride as isize);
                i += 1;
            }
            row += stripe_h;
            stripe_h = 64 as libc::c_int >> ss_ver;
            src = src
                .offset(
                    ((stripe_h - 4 as libc::c_int) as libc::c_long * src_stride) as isize,
                );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_copy_lpf_8bpc(
    f: *mut Dav1dFrameContext,
    mut src: *const *mut pixel,
    sby: libc::c_int,
) {
    let have_tt: libc::c_int = ((*(*f).c).n_tc > 1 as libc::c_int as libc::c_uint)
        as libc::c_int;
    let resize: libc::c_int = ((*(*f).frame_hdr).width[0 as libc::c_int as usize]
        != (*(*f).frame_hdr).width[1 as libc::c_int as usize]) as libc::c_int;
    let offset: libc::c_int = 8 as libc::c_int * (sby != 0) as libc::c_int;
    let src_stride: *const ptrdiff_t = ((*f).cur.stride).as_mut_ptr();
    let lr_stride: *const ptrdiff_t = ((*f).sr_cur.p.stride).as_mut_ptr();
    let tt_off: libc::c_int = have_tt * sby
        * ((4 as libc::c_int) << (*(*f).seq_hdr).sb128);
    let dst: [*mut pixel; 3] = [
        ((*f).lf.lr_lpf_line[0 as libc::c_int as usize])
            .offset(
                (tt_off as libc::c_long * *lr_stride.offset(0 as libc::c_int as isize))
                    as isize,
            ),
        ((*f).lf.lr_lpf_line[1 as libc::c_int as usize])
            .offset(
                (tt_off as libc::c_long * *lr_stride.offset(1 as libc::c_int as isize))
                    as isize,
            ),
        ((*f).lf.lr_lpf_line[2 as libc::c_int as usize])
            .offset(
                (tt_off as libc::c_long * *lr_stride.offset(1 as libc::c_int as isize))
                    as isize,
            ),
    ];
    let restore_planes: libc::c_int = (*f).lf.restore_planes;
    if (*(*f).seq_hdr).cdef != 0 || restore_planes & LR_RESTORE_Y as libc::c_int != 0 {
        let h: libc::c_int = (*f).cur.p.h;
        let w: libc::c_int = (*f).bw << 2 as libc::c_int;
        let row_h: libc::c_int = imin(
            (sby + 1 as libc::c_int) << 6 as libc::c_int + (*(*f).seq_hdr).sb128,
            h - 1 as libc::c_int,
        );
        let y_stripe: libc::c_int = (sby << 6 as libc::c_int + (*(*f).seq_hdr).sb128)
            - offset;
        if restore_planes & LR_RESTORE_Y as libc::c_int != 0 || resize == 0 {
            backup_lpf(
                f,
                dst[0 as libc::c_int as usize],
                *lr_stride.offset(0 as libc::c_int as isize),
                (*src.offset(0 as libc::c_int as isize))
                    .offset(
                        -((offset as libc::c_long
                            * *src_stride.offset(0 as libc::c_int as isize)) as isize),
                    ),
                *src_stride.offset(0 as libc::c_int as isize),
                0 as libc::c_int,
                (*(*f).seq_hdr).sb128,
                y_stripe,
                row_h,
                w,
                h,
                0 as libc::c_int,
                1 as libc::c_int,
            );
        }
        if have_tt != 0 && resize != 0 {
            let cdef_off_y: ptrdiff_t = (sby * 4 as libc::c_int) as libc::c_long
                * *src_stride.offset(0 as libc::c_int as isize);
            backup_lpf(
                f,
                ((*f).lf.cdef_lpf_line[0 as libc::c_int as usize])
                    .offset(cdef_off_y as isize),
                *src_stride.offset(0 as libc::c_int as isize),
                (*src.offset(0 as libc::c_int as isize))
                    .offset(
                        -((offset as libc::c_long
                            * *src_stride.offset(0 as libc::c_int as isize)) as isize),
                    ),
                *src_stride.offset(0 as libc::c_int as isize),
                0 as libc::c_int,
                (*(*f).seq_hdr).sb128,
                y_stripe,
                row_h,
                w,
                h,
                0 as libc::c_int,
                0 as libc::c_int,
            );
        }
    }
    if ((*(*f).seq_hdr).cdef != 0
        || restore_planes & (LR_RESTORE_U as libc::c_int | LR_RESTORE_V as libc::c_int)
            != 0)
        && (*f).cur.p.layout as libc::c_uint
            != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint
    {
        let ss_ver: libc::c_int = ((*f).sr_cur.p.p.layout as libc::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
        let ss_hor: libc::c_int = ((*f).sr_cur.p.p.layout as libc::c_uint
            != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
        let h_0: libc::c_int = (*f).cur.p.h + ss_ver >> ss_ver;
        let w_0: libc::c_int = (*f).bw << 2 as libc::c_int - ss_hor;
        let row_h_0: libc::c_int = imin(
            (sby + 1 as libc::c_int)
                << 6 as libc::c_int - ss_ver + (*(*f).seq_hdr).sb128,
            h_0 - 1 as libc::c_int,
        );
        let offset_uv: libc::c_int = offset >> ss_ver;
        let y_stripe_0: libc::c_int = (sby
            << 6 as libc::c_int - ss_ver + (*(*f).seq_hdr).sb128) - offset_uv;
        let cdef_off_uv: ptrdiff_t = (sby * 4 as libc::c_int) as libc::c_long
            * *src_stride.offset(1 as libc::c_int as isize);
        if (*(*f).seq_hdr).cdef != 0 || restore_planes & LR_RESTORE_U as libc::c_int != 0
        {
            if restore_planes & LR_RESTORE_U as libc::c_int != 0 || resize == 0 {
                backup_lpf(
                    f,
                    dst[1 as libc::c_int as usize],
                    *lr_stride.offset(1 as libc::c_int as isize),
                    (*src.offset(1 as libc::c_int as isize))
                        .offset(
                            -((offset_uv as libc::c_long
                                * *src_stride.offset(1 as libc::c_int as isize)) as isize),
                        ),
                    *src_stride.offset(1 as libc::c_int as isize),
                    ss_ver,
                    (*(*f).seq_hdr).sb128,
                    y_stripe_0,
                    row_h_0,
                    w_0,
                    h_0,
                    ss_hor,
                    1 as libc::c_int,
                );
            }
            if have_tt != 0 && resize != 0 {
                backup_lpf(
                    f,
                    ((*f).lf.cdef_lpf_line[1 as libc::c_int as usize])
                        .offset(cdef_off_uv as isize),
                    *src_stride.offset(1 as libc::c_int as isize),
                    (*src.offset(1 as libc::c_int as isize))
                        .offset(
                            -((offset_uv as libc::c_long
                                * *src_stride.offset(1 as libc::c_int as isize)) as isize),
                        ),
                    *src_stride.offset(1 as libc::c_int as isize),
                    ss_ver,
                    (*(*f).seq_hdr).sb128,
                    y_stripe_0,
                    row_h_0,
                    w_0,
                    h_0,
                    ss_hor,
                    0 as libc::c_int,
                );
            }
        }
        if (*(*f).seq_hdr).cdef != 0 || restore_planes & LR_RESTORE_V as libc::c_int != 0
        {
            if restore_planes & LR_RESTORE_V as libc::c_int != 0 || resize == 0 {
                backup_lpf(
                    f,
                    dst[2 as libc::c_int as usize],
                    *lr_stride.offset(1 as libc::c_int as isize),
                    (*src.offset(2 as libc::c_int as isize))
                        .offset(
                            -((offset_uv as libc::c_long
                                * *src_stride.offset(1 as libc::c_int as isize)) as isize),
                        ),
                    *src_stride.offset(1 as libc::c_int as isize),
                    ss_ver,
                    (*(*f).seq_hdr).sb128,
                    y_stripe_0,
                    row_h_0,
                    w_0,
                    h_0,
                    ss_hor,
                    1 as libc::c_int,
                );
            }
            if have_tt != 0 && resize != 0 {
                backup_lpf(
                    f,
                    ((*f).lf.cdef_lpf_line[2 as libc::c_int as usize])
                        .offset(cdef_off_uv as isize),
                    *src_stride.offset(1 as libc::c_int as isize),
                    (*src.offset(2 as libc::c_int as isize))
                        .offset(
                            -((offset_uv as libc::c_long
                                * *src_stride.offset(1 as libc::c_int as isize)) as isize),
                        ),
                    *src_stride.offset(1 as libc::c_int as isize),
                    ss_ver,
                    (*(*f).seq_hdr).sb128,
                    y_stripe_0,
                    row_h_0,
                    w_0,
                    h_0,
                    ss_hor,
                    0 as libc::c_int,
                );
            }
        }
    }
}
#[inline]
unsafe extern "C" fn filter_plane_cols_y(
    f: *const Dav1dFrameContext,
    have_left: libc::c_int,
    mut lvl: *const [uint8_t; 4],
    b4_stride: ptrdiff_t,
    mask: *const [[uint16_t; 2]; 3],
    mut dst: *mut pixel,
    ls: ptrdiff_t,
    w: libc::c_int,
    starty4: libc::c_int,
    endy4: libc::c_int,
) {
    let dsp: *const Dav1dDSPContext = (*f).dsp;
    let mut x: libc::c_int = 0 as libc::c_int;
    while x < w {
        if !(have_left == 0 && x == 0) {
            let mut hmask: [uint32_t; 4] = [0; 4];
            if starty4 == 0 {
                hmask[0 as libc::c_int
                    as usize] = (*mask
                    .offset(
                        x as isize,
                    ))[0 as libc::c_int as usize][0 as libc::c_int as usize] as uint32_t;
                hmask[1 as libc::c_int
                    as usize] = (*mask
                    .offset(
                        x as isize,
                    ))[1 as libc::c_int as usize][0 as libc::c_int as usize] as uint32_t;
                hmask[2 as libc::c_int
                    as usize] = (*mask
                    .offset(
                        x as isize,
                    ))[2 as libc::c_int as usize][0 as libc::c_int as usize] as uint32_t;
                if endy4 > 16 as libc::c_int {
                    hmask[0 as libc::c_int as usize]
                        |= ((*mask
                            .offset(
                                x as isize,
                            ))[0 as libc::c_int as usize][1 as libc::c_int as usize]
                            as libc::c_uint) << 16 as libc::c_int;
                    hmask[1 as libc::c_int as usize]
                        |= ((*mask
                            .offset(
                                x as isize,
                            ))[1 as libc::c_int as usize][1 as libc::c_int as usize]
                            as libc::c_uint) << 16 as libc::c_int;
                    hmask[2 as libc::c_int as usize]
                        |= ((*mask
                            .offset(
                                x as isize,
                            ))[2 as libc::c_int as usize][1 as libc::c_int as usize]
                            as libc::c_uint) << 16 as libc::c_int;
                }
            } else {
                hmask[0 as libc::c_int
                    as usize] = (*mask
                    .offset(
                        x as isize,
                    ))[0 as libc::c_int as usize][1 as libc::c_int as usize] as uint32_t;
                hmask[1 as libc::c_int
                    as usize] = (*mask
                    .offset(
                        x as isize,
                    ))[1 as libc::c_int as usize][1 as libc::c_int as usize] as uint32_t;
                hmask[2 as libc::c_int
                    as usize] = (*mask
                    .offset(
                        x as isize,
                    ))[2 as libc::c_int as usize][1 as libc::c_int as usize] as uint32_t;
            }
            hmask[3 as libc::c_int as usize] = 0 as libc::c_int as uint32_t;
            ((*dsp)
                .lf
                .loop_filter_sb[0 as libc::c_int as usize][0 as libc::c_int as usize])
                .expect(
                    "non-null function pointer",
                )(
                &mut *dst.offset((x * 4 as libc::c_int) as isize),
                ls,
                hmask.as_mut_ptr(),
                &*(*lvl.offset(x as isize)).as_ptr().offset(0 as libc::c_int as isize)
                    as *const uint8_t as *const [uint8_t; 4],
                b4_stride,
                &(*f).lf.lim_lut,
                endy4 - starty4,
            );
        }
        x += 1;
    }
}
#[inline]
unsafe extern "C" fn filter_plane_rows_y(
    f: *const Dav1dFrameContext,
    have_top: libc::c_int,
    mut lvl: *const [uint8_t; 4],
    b4_stride: ptrdiff_t,
    mask: *const [[uint16_t; 2]; 3],
    mut dst: *mut pixel,
    ls: ptrdiff_t,
    w: libc::c_int,
    starty4: libc::c_int,
    endy4: libc::c_int,
) {
    let dsp: *const Dav1dDSPContext = (*f).dsp;
    let mut y: libc::c_int = starty4;
    while y < endy4 {
        if !(have_top == 0 && y == 0) {
            let vmask: [uint32_t; 4] = [
                (*mask
                    .offset(
                        y as isize,
                    ))[0 as libc::c_int as usize][0 as libc::c_int as usize]
                    as libc::c_uint
                    | ((*mask
                        .offset(
                            y as isize,
                        ))[0 as libc::c_int as usize][1 as libc::c_int as usize]
                        as libc::c_uint) << 16 as libc::c_int,
                (*mask
                    .offset(
                        y as isize,
                    ))[1 as libc::c_int as usize][0 as libc::c_int as usize]
                    as libc::c_uint
                    | ((*mask
                        .offset(
                            y as isize,
                        ))[1 as libc::c_int as usize][1 as libc::c_int as usize]
                        as libc::c_uint) << 16 as libc::c_int,
                (*mask
                    .offset(
                        y as isize,
                    ))[2 as libc::c_int as usize][0 as libc::c_int as usize]
                    as libc::c_uint
                    | ((*mask
                        .offset(
                            y as isize,
                        ))[2 as libc::c_int as usize][1 as libc::c_int as usize]
                        as libc::c_uint) << 16 as libc::c_int,
                0 as libc::c_int as uint32_t,
            ];
            ((*dsp)
                .lf
                .loop_filter_sb[0 as libc::c_int as usize][1 as libc::c_int as usize])
                .expect(
                    "non-null function pointer",
                )(
                dst,
                ls,
                vmask.as_ptr(),
                &*(*lvl.offset(0 as libc::c_int as isize))
                    .as_ptr()
                    .offset(1 as libc::c_int as isize) as *const uint8_t
                    as *const [uint8_t; 4],
                b4_stride,
                &(*f).lf.lim_lut,
                w,
            );
        }
        y += 1;
        dst = dst.offset((4 as libc::c_int as libc::c_long * ls) as isize);
        lvl = lvl.offset(b4_stride as isize);
    }
}
#[inline]
unsafe extern "C" fn filter_plane_cols_uv(
    f: *const Dav1dFrameContext,
    have_left: libc::c_int,
    mut lvl: *const [uint8_t; 4],
    b4_stride: ptrdiff_t,
    mask: *const [[uint16_t; 2]; 2],
    u: *mut pixel,
    v: *mut pixel,
    ls: ptrdiff_t,
    w: libc::c_int,
    starty4: libc::c_int,
    endy4: libc::c_int,
    ss_ver: libc::c_int,
) {
    let dsp: *const Dav1dDSPContext = (*f).dsp;
    let mut x: libc::c_int = 0 as libc::c_int;
    while x < w {
        if !(have_left == 0 && x == 0) {
            let mut hmask: [uint32_t; 3] = [0; 3];
            if starty4 == 0 {
                hmask[0 as libc::c_int
                    as usize] = (*mask
                    .offset(
                        x as isize,
                    ))[0 as libc::c_int as usize][0 as libc::c_int as usize] as uint32_t;
                hmask[1 as libc::c_int
                    as usize] = (*mask
                    .offset(
                        x as isize,
                    ))[1 as libc::c_int as usize][0 as libc::c_int as usize] as uint32_t;
                if endy4 > 16 as libc::c_int >> ss_ver {
                    hmask[0 as libc::c_int as usize]
                        |= ((*mask
                            .offset(
                                x as isize,
                            ))[0 as libc::c_int as usize][1 as libc::c_int as usize]
                            as libc::c_uint) << (16 as libc::c_int >> ss_ver);
                    hmask[1 as libc::c_int as usize]
                        |= ((*mask
                            .offset(
                                x as isize,
                            ))[1 as libc::c_int as usize][1 as libc::c_int as usize]
                            as libc::c_uint) << (16 as libc::c_int >> ss_ver);
                }
            } else {
                hmask[0 as libc::c_int
                    as usize] = (*mask
                    .offset(
                        x as isize,
                    ))[0 as libc::c_int as usize][1 as libc::c_int as usize] as uint32_t;
                hmask[1 as libc::c_int
                    as usize] = (*mask
                    .offset(
                        x as isize,
                    ))[1 as libc::c_int as usize][1 as libc::c_int as usize] as uint32_t;
            }
            hmask[2 as libc::c_int as usize] = 0 as libc::c_int as uint32_t;
            ((*dsp)
                .lf
                .loop_filter_sb[1 as libc::c_int as usize][0 as libc::c_int as usize])
                .expect(
                    "non-null function pointer",
                )(
                &mut *u.offset((x * 4 as libc::c_int) as isize),
                ls,
                hmask.as_mut_ptr(),
                &*(*lvl.offset(x as isize)).as_ptr().offset(2 as libc::c_int as isize)
                    as *const uint8_t as *const [uint8_t; 4],
                b4_stride,
                &(*f).lf.lim_lut,
                endy4 - starty4,
            );
            ((*dsp)
                .lf
                .loop_filter_sb[1 as libc::c_int as usize][0 as libc::c_int as usize])
                .expect(
                    "non-null function pointer",
                )(
                &mut *v.offset((x * 4 as libc::c_int) as isize),
                ls,
                hmask.as_mut_ptr(),
                &*(*lvl.offset(x as isize)).as_ptr().offset(3 as libc::c_int as isize)
                    as *const uint8_t as *const [uint8_t; 4],
                b4_stride,
                &(*f).lf.lim_lut,
                endy4 - starty4,
            );
        }
        x += 1;
    }
}
#[inline]
unsafe extern "C" fn filter_plane_rows_uv(
    f: *const Dav1dFrameContext,
    have_top: libc::c_int,
    mut lvl: *const [uint8_t; 4],
    b4_stride: ptrdiff_t,
    mask: *const [[uint16_t; 2]; 2],
    u: *mut pixel,
    v: *mut pixel,
    ls: ptrdiff_t,
    w: libc::c_int,
    starty4: libc::c_int,
    endy4: libc::c_int,
    ss_hor: libc::c_int,
) {
    let dsp: *const Dav1dDSPContext = (*f).dsp;
    let mut off_l: ptrdiff_t = 0 as libc::c_int as ptrdiff_t;
    let mut y: libc::c_int = starty4;
    while y < endy4 {
        if !(have_top == 0 && y == 0) {
            let vmask: [uint32_t; 3] = [
                (*mask
                    .offset(
                        y as isize,
                    ))[0 as libc::c_int as usize][0 as libc::c_int as usize]
                    as libc::c_uint
                    | ((*mask
                        .offset(
                            y as isize,
                        ))[0 as libc::c_int as usize][1 as libc::c_int as usize]
                        as libc::c_uint) << (16 as libc::c_int >> ss_hor),
                (*mask
                    .offset(
                        y as isize,
                    ))[1 as libc::c_int as usize][0 as libc::c_int as usize]
                    as libc::c_uint
                    | ((*mask
                        .offset(
                            y as isize,
                        ))[1 as libc::c_int as usize][1 as libc::c_int as usize]
                        as libc::c_uint) << (16 as libc::c_int >> ss_hor),
                0 as libc::c_int as uint32_t,
            ];
            ((*dsp)
                .lf
                .loop_filter_sb[1 as libc::c_int as usize][1 as libc::c_int as usize])
                .expect(
                    "non-null function pointer",
                )(
                &mut *u.offset(off_l as isize),
                ls,
                vmask.as_ptr(),
                &*(*lvl.offset(0 as libc::c_int as isize))
                    .as_ptr()
                    .offset(2 as libc::c_int as isize) as *const uint8_t
                    as *const [uint8_t; 4],
                b4_stride,
                &(*f).lf.lim_lut,
                w,
            );
            ((*dsp)
                .lf
                .loop_filter_sb[1 as libc::c_int as usize][1 as libc::c_int as usize])
                .expect(
                    "non-null function pointer",
                )(
                &mut *v.offset(off_l as isize),
                ls,
                vmask.as_ptr(),
                &*(*lvl.offset(0 as libc::c_int as isize))
                    .as_ptr()
                    .offset(3 as libc::c_int as isize) as *const uint8_t
                    as *const [uint8_t; 4],
                b4_stride,
                &(*f).lf.lim_lut,
                w,
            );
        }
        y += 1;
        off_l += 4 as libc::c_int as libc::c_long * ls;
        lvl = lvl.offset(b4_stride as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_loopfilter_sbrow_cols_8bpc(
    f: *const Dav1dFrameContext,
    mut p: *const *mut pixel,
    lflvl: *mut Av1Filter,
    mut sby: libc::c_int,
    start_of_tile_row: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    let mut have_left: libc::c_int = 0;
    let is_sb64: libc::c_int = ((*(*f).seq_hdr).sb128 == 0) as libc::c_int;
    let starty4: libc::c_int = (sby & is_sb64) << 4 as libc::c_int;
    let sbsz: libc::c_int = 32 as libc::c_int >> is_sb64;
    let sbl2: libc::c_int = 5 as libc::c_int - is_sb64;
    let halign: libc::c_int = (*f).bh + 31 as libc::c_int & !(31 as libc::c_int);
    let ss_ver: libc::c_int = ((*f).cur.p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_hor: libc::c_int = ((*f).cur.p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
    let vmask: libc::c_int = 16 as libc::c_int >> ss_ver;
    let hmask: libc::c_int = 16 as libc::c_int >> ss_hor;
    let vmax: libc::c_uint = (1 as libc::c_uint) << vmask;
    let hmax: libc::c_uint = (1 as libc::c_uint) << hmask;
    let endy4: libc::c_uint = (starty4 + imin((*f).h4 - sby * sbsz, sbsz))
        as libc::c_uint;
    let uv_endy4: libc::c_uint = endy4.wrapping_add(ss_ver as libc::c_uint) >> ss_ver;
    let mut lpf_y: *const uint8_t = &mut *(*((*f).lf.tx_lpf_right_edge)
        .as_ptr()
        .offset(0 as libc::c_int as isize))
        .offset((sby << sbl2) as isize) as *mut uint8_t;
    let mut lpf_uv: *const uint8_t = &mut *(*((*f).lf.tx_lpf_right_edge)
        .as_ptr()
        .offset(1 as libc::c_int as isize))
        .offset((sby << sbl2 - ss_ver) as isize) as *mut uint8_t;
    let mut tile_col: libc::c_int = 1 as libc::c_int;
    loop {
        x = (*(*f).frame_hdr).tiling.col_start_sb[tile_col as usize] as libc::c_int;
        if x << sbl2 >= (*f).bw {
            break;
        }
        let bx4: libc::c_int = if x & is_sb64 != 0 {
            16 as libc::c_int
        } else {
            0 as libc::c_int
        };
        let cbx4: libc::c_int = bx4 >> ss_hor;
        x >>= is_sb64;
        let y_hmask: *mut [uint16_t; 2] = ((*lflvl.offset(x as isize))
            .filter_y[0 as libc::c_int as usize][bx4 as usize])
            .as_mut_ptr();
        let mut y: libc::c_uint = starty4 as libc::c_uint;
        let mut mask: libc::c_uint = ((1 as libc::c_int) << y) as libc::c_uint;
        while y < endy4 {
            let sidx: libc::c_int = (mask >= 0x10000 as libc::c_uint) as libc::c_int;
            let smask: libc::c_uint = mask >> (sidx << 4 as libc::c_int);
            let idx: libc::c_int = 2 as libc::c_int
                * ((*y_hmask.offset(2 as libc::c_int as isize))[sidx as usize]
                    as libc::c_uint & smask != 0) as libc::c_int
                + ((*y_hmask.offset(1 as libc::c_int as isize))[sidx as usize]
                    as libc::c_uint & smask != 0) as libc::c_int;
            let ref mut fresh0 = (*y_hmask
                .offset(2 as libc::c_int as isize))[sidx as usize];
            *fresh0 = (*fresh0 as libc::c_uint & !smask) as uint16_t;
            let ref mut fresh1 = (*y_hmask
                .offset(1 as libc::c_int as isize))[sidx as usize];
            *fresh1 = (*fresh1 as libc::c_uint & !smask) as uint16_t;
            let ref mut fresh2 = (*y_hmask
                .offset(0 as libc::c_int as isize))[sidx as usize];
            *fresh2 = (*fresh2 as libc::c_uint & !smask) as uint16_t;
            let ref mut fresh3 = (*y_hmask
                .offset(
                    imin(
                        idx,
                        *lpf_y.offset(y.wrapping_sub(starty4 as libc::c_uint) as isize)
                            as libc::c_int,
                    ) as isize,
                ))[sidx as usize];
            *fresh3 = (*fresh3 as libc::c_uint | smask) as uint16_t;
            y = y.wrapping_add(1);
            mask <<= 1 as libc::c_int;
        }
        if (*f).cur.p.layout as libc::c_uint
            != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint
        {
            let uv_hmask: *mut [uint16_t; 2] = ((*lflvl.offset(x as isize))
                .filter_uv[0 as libc::c_int as usize][cbx4 as usize])
                .as_mut_ptr();
            let mut y_0: libc::c_uint = (starty4 >> ss_ver) as libc::c_uint;
            let mut uv_mask: libc::c_uint = ((1 as libc::c_int) << y_0) as libc::c_uint;
            while y_0 < uv_endy4 {
                let sidx_0: libc::c_int = (uv_mask >= vmax) as libc::c_int;
                let smask_0: libc::c_uint = uv_mask
                    >> (sidx_0 << 4 as libc::c_int - ss_ver);
                let idx_0: libc::c_int = ((*uv_hmask
                    .offset(1 as libc::c_int as isize))[sidx_0 as usize] as libc::c_uint
                    & smask_0 != 0) as libc::c_int;
                let ref mut fresh4 = (*uv_hmask
                    .offset(1 as libc::c_int as isize))[sidx_0 as usize];
                *fresh4 = (*fresh4 as libc::c_uint & !smask_0) as uint16_t;
                let ref mut fresh5 = (*uv_hmask
                    .offset(0 as libc::c_int as isize))[sidx_0 as usize];
                *fresh5 = (*fresh5 as libc::c_uint & !smask_0) as uint16_t;
                let ref mut fresh6 = (*uv_hmask
                    .offset(
                        imin(
                            idx_0,
                            *lpf_uv
                                .offset(
                                    y_0.wrapping_sub((starty4 >> ss_ver) as libc::c_uint)
                                        as isize,
                                ) as libc::c_int,
                        ) as isize,
                    ))[sidx_0 as usize];
                *fresh6 = (*fresh6 as libc::c_uint | smask_0) as uint16_t;
                y_0 = y_0.wrapping_add(1);
                uv_mask <<= 1 as libc::c_int;
            }
        }
        lpf_y = lpf_y.offset(halign as isize);
        lpf_uv = lpf_uv.offset((halign >> ss_ver) as isize);
        tile_col += 1;
    }
    if start_of_tile_row != 0 {
        let mut a: *const BlockContext = 0 as *const BlockContext;
        x = 0 as libc::c_int;
        a = &mut *((*f).a)
            .offset(((*f).sb128w * (start_of_tile_row - 1 as libc::c_int)) as isize)
            as *mut BlockContext;
        while x < (*f).sb128w {
            let y_vmask: *mut [uint16_t; 2] = ((*lflvl.offset(x as isize))
                .filter_y[1 as libc::c_int as usize][starty4 as usize])
                .as_mut_ptr();
            let w: libc::c_uint = imin(
                32 as libc::c_int,
                (*f).w4 - (x << 5 as libc::c_int),
            ) as libc::c_uint;
            let mut mask_0: libc::c_uint = 1 as libc::c_int as libc::c_uint;
            let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while i < w {
                let sidx_1: libc::c_int = (mask_0 >= 0x10000 as libc::c_uint)
                    as libc::c_int;
                let smask_1: libc::c_uint = mask_0 >> (sidx_1 << 4 as libc::c_int);
                let idx_1: libc::c_int = 2 as libc::c_int
                    * ((*y_vmask.offset(2 as libc::c_int as isize))[sidx_1 as usize]
                        as libc::c_uint & smask_1 != 0) as libc::c_int
                    + ((*y_vmask.offset(1 as libc::c_int as isize))[sidx_1 as usize]
                        as libc::c_uint & smask_1 != 0) as libc::c_int;
                let ref mut fresh7 = (*y_vmask
                    .offset(2 as libc::c_int as isize))[sidx_1 as usize];
                *fresh7 = (*fresh7 as libc::c_uint & !smask_1) as uint16_t;
                let ref mut fresh8 = (*y_vmask
                    .offset(1 as libc::c_int as isize))[sidx_1 as usize];
                *fresh8 = (*fresh8 as libc::c_uint & !smask_1) as uint16_t;
                let ref mut fresh9 = (*y_vmask
                    .offset(0 as libc::c_int as isize))[sidx_1 as usize];
                *fresh9 = (*fresh9 as libc::c_uint & !smask_1) as uint16_t;
                let ref mut fresh10 = (*y_vmask
                    .offset(
                        imin(idx_1, (*a).tx_lpf_y[i as usize] as libc::c_int) as isize,
                    ))[sidx_1 as usize];
                *fresh10 = (*fresh10 as libc::c_uint | smask_1) as uint16_t;
                mask_0 <<= 1 as libc::c_int;
                i = i.wrapping_add(1);
            }
            if (*f).cur.p.layout as libc::c_uint
                != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint
            {
                let cw: libc::c_uint = w.wrapping_add(ss_hor as libc::c_uint) >> ss_hor;
                let uv_vmask: *mut [uint16_t; 2] = ((*lflvl.offset(x as isize))
                    .filter_uv[1 as libc::c_int as usize][(starty4 >> ss_ver) as usize])
                    .as_mut_ptr();
                let mut uv_mask_0: libc::c_uint = 1 as libc::c_int as libc::c_uint;
                let mut i_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                while i_0 < cw {
                    let sidx_2: libc::c_int = (uv_mask_0 >= hmax) as libc::c_int;
                    let smask_2: libc::c_uint = uv_mask_0
                        >> (sidx_2 << 4 as libc::c_int - ss_hor);
                    let idx_2: libc::c_int = ((*uv_vmask
                        .offset(1 as libc::c_int as isize))[sidx_2 as usize]
                        as libc::c_uint & smask_2 != 0) as libc::c_int;
                    let ref mut fresh11 = (*uv_vmask
                        .offset(1 as libc::c_int as isize))[sidx_2 as usize];
                    *fresh11 = (*fresh11 as libc::c_uint & !smask_2) as uint16_t;
                    let ref mut fresh12 = (*uv_vmask
                        .offset(0 as libc::c_int as isize))[sidx_2 as usize];
                    *fresh12 = (*fresh12 as libc::c_uint & !smask_2) as uint16_t;
                    let ref mut fresh13 = (*uv_vmask
                        .offset(
                            imin(idx_2, (*a).tx_lpf_uv[i_0 as usize] as libc::c_int)
                                as isize,
                        ))[sidx_2 as usize];
                    *fresh13 = (*fresh13 as libc::c_uint | smask_2) as uint16_t;
                    uv_mask_0 <<= 1 as libc::c_int;
                    i_0 = i_0.wrapping_add(1);
                }
            }
            x += 1;
            a = a.offset(1);
        }
    }
    let mut ptr: *mut pixel = 0 as *mut pixel;
    let mut level_ptr: *mut [uint8_t; 4] = ((*f).lf.level)
        .offset(((*f).b4_stride * sby as libc::c_long * sbsz as libc::c_long) as isize);
    ptr = *p.offset(0 as libc::c_int as isize);
    have_left = 0 as libc::c_int;
    x = 0 as libc::c_int;
    while x < (*f).sb128w {
        filter_plane_cols_y(
            f,
            have_left,
            level_ptr as *const [uint8_t; 4],
            (*f).b4_stride,
            ((*lflvl.offset(x as isize)).filter_y[0 as libc::c_int as usize])
                .as_mut_ptr() as *const [[uint16_t; 2]; 3],
            ptr,
            (*f).cur.stride[0 as libc::c_int as usize],
            imin(32 as libc::c_int, (*f).w4 - x * 32 as libc::c_int),
            starty4,
            endy4 as libc::c_int,
        );
        x += 1;
        have_left = 1 as libc::c_int;
        ptr = ptr.offset(128 as libc::c_int as isize);
        level_ptr = level_ptr.offset(32 as libc::c_int as isize);
    }
    if (*(*f).frame_hdr).loopfilter.level_u == 0
        && (*(*f).frame_hdr).loopfilter.level_v == 0
    {
        return;
    }
    let mut uv_off: ptrdiff_t = 0;
    level_ptr = ((*f).lf.level)
        .offset(((*f).b4_stride * (sby * sbsz >> ss_ver) as libc::c_long) as isize);
    uv_off = 0 as libc::c_int as ptrdiff_t;
    have_left = 0 as libc::c_int;
    x = 0 as libc::c_int;
    while x < (*f).sb128w {
        filter_plane_cols_uv(
            f,
            have_left,
            level_ptr as *const [uint8_t; 4],
            (*f).b4_stride,
            ((*lflvl.offset(x as isize)).filter_uv[0 as libc::c_int as usize])
                .as_mut_ptr() as *const [[uint16_t; 2]; 2],
            &mut *(*p.offset(1 as libc::c_int as isize)).offset(uv_off as isize),
            &mut *(*p.offset(2 as libc::c_int as isize)).offset(uv_off as isize),
            (*f).cur.stride[1 as libc::c_int as usize],
            imin(32 as libc::c_int, (*f).w4 - x * 32 as libc::c_int) + ss_hor >> ss_hor,
            starty4 >> ss_ver,
            uv_endy4 as libc::c_int,
            ss_ver,
        );
        x += 1;
        have_left = 1 as libc::c_int;
        uv_off += (128 as libc::c_int >> ss_hor) as libc::c_long;
        level_ptr = level_ptr.offset((32 as libc::c_int >> ss_hor) as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_loopfilter_sbrow_rows_8bpc(
    f: *const Dav1dFrameContext,
    mut p: *const *mut pixel,
    lflvl: *mut Av1Filter,
    mut sby: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    let have_top: libc::c_int = (sby > 0 as libc::c_int) as libc::c_int;
    let is_sb64: libc::c_int = ((*(*f).seq_hdr).sb128 == 0) as libc::c_int;
    let starty4: libc::c_int = (sby & is_sb64) << 4 as libc::c_int;
    let sbsz: libc::c_int = 32 as libc::c_int >> is_sb64;
    let ss_ver: libc::c_int = ((*f).cur.p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_hor: libc::c_int = ((*f).cur.p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
    let endy4: libc::c_uint = (starty4 + imin((*f).h4 - sby * sbsz, sbsz))
        as libc::c_uint;
    let uv_endy4: libc::c_uint = endy4.wrapping_add(ss_ver as libc::c_uint) >> ss_ver;
    let mut ptr: *mut pixel = 0 as *mut pixel;
    let mut level_ptr: *mut [uint8_t; 4] = ((*f).lf.level)
        .offset(((*f).b4_stride * sby as libc::c_long * sbsz as libc::c_long) as isize);
    ptr = *p.offset(0 as libc::c_int as isize);
    x = 0 as libc::c_int;
    while x < (*f).sb128w {
        filter_plane_rows_y(
            f,
            have_top,
            level_ptr as *const [uint8_t; 4],
            (*f).b4_stride,
            ((*lflvl.offset(x as isize)).filter_y[1 as libc::c_int as usize])
                .as_mut_ptr() as *const [[uint16_t; 2]; 3],
            ptr,
            (*f).cur.stride[0 as libc::c_int as usize],
            imin(32 as libc::c_int, (*f).w4 - x * 32 as libc::c_int),
            starty4,
            endy4 as libc::c_int,
        );
        x += 1;
        ptr = ptr.offset(128 as libc::c_int as isize);
        level_ptr = level_ptr.offset(32 as libc::c_int as isize);
    }
    if (*(*f).frame_hdr).loopfilter.level_u == 0
        && (*(*f).frame_hdr).loopfilter.level_v == 0
    {
        return;
    }
    let mut uv_off: ptrdiff_t = 0;
    level_ptr = ((*f).lf.level)
        .offset(((*f).b4_stride * (sby * sbsz >> ss_ver) as libc::c_long) as isize);
    uv_off = 0 as libc::c_int as ptrdiff_t;
    x = 0 as libc::c_int;
    while x < (*f).sb128w {
        filter_plane_rows_uv(
            f,
            have_top,
            level_ptr as *const [uint8_t; 4],
            (*f).b4_stride,
            ((*lflvl.offset(x as isize)).filter_uv[1 as libc::c_int as usize])
                .as_mut_ptr() as *const [[uint16_t; 2]; 2],
            &mut *(*p.offset(1 as libc::c_int as isize)).offset(uv_off as isize),
            &mut *(*p.offset(2 as libc::c_int as isize)).offset(uv_off as isize),
            (*f).cur.stride[1 as libc::c_int as usize],
            imin(32 as libc::c_int, (*f).w4 - x * 32 as libc::c_int) + ss_hor >> ss_hor,
            starty4 >> ss_ver,
            uv_endy4 as libc::c_int,
            ss_hor,
        );
        x += 1;
        uv_off += (128 as libc::c_int >> ss_hor) as libc::c_long;
        level_ptr = level_ptr.offset((32 as libc::c_int >> ss_hor) as isize);
    }
}
