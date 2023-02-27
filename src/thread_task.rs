use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn abort() -> !;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn prctl(__option: libc::c_int, _: ...) -> libc::c_int;
    fn dav1d_cdf_thread_update(
        hdr: *const Dav1dFrameHeader,
        dst: *mut CdfContext,
        src: *const CdfContext,
    );
    fn dav1d_decode_frame_init(f: *mut Dav1dFrameContext) -> libc::c_int;
    fn dav1d_decode_frame_init_cdf(f: *mut Dav1dFrameContext) -> libc::c_int;
    fn dav1d_decode_tile_sbrow(t: *mut Dav1dTaskContext) -> libc::c_int;
    fn dav1d_decode_frame_exit(f: *mut Dav1dFrameContext, retval: libc::c_int);
    fn dav1d_prep_grain_16bpc(
        dsp: *const Dav1dFilmGrainDSPContext,
        out: *mut Dav1dPicture,
        in_0: *const Dav1dPicture,
        scaling: *mut libc::c_void,
        grain_lut: *mut libc::c_void,
    );
    fn dav1d_apply_grain_row_16bpc(
        dsp: *const Dav1dFilmGrainDSPContext,
        out: *mut Dav1dPicture,
        in_0: *const Dav1dPicture,
        scaling: *mut libc::c_void,
        grain_lut: *mut libc::c_void,
        row: libc::c_int,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub type atomic_int = libc::c_int;
pub type atomic_uint = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dUserData {
    pub data: *const uint8_t,
    pub ref_0: *mut Dav1dRef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dRef {
    pub data: *mut libc::c_void,
    pub const_data: *const libc::c_void,
    pub ref_cnt: atomic_int,
    pub free_ref: libc::c_int,
    pub free_callback: Option::<
        unsafe extern "C" fn(*const uint8_t, *mut libc::c_void) -> (),
    >,
    pub user_data: *mut libc::c_void,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dData {
    pub data: *const uint8_t,
    pub sz: size_t,
    pub ref_0: *mut Dav1dRef,
    pub m: Dav1dDataProps,
}
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
    pub ipred_edge: [*mut libc::c_void; 3],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
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
pub struct Dav1dMasteringDisplay {
    pub primaries: [[uint16_t; 2]; 3],
    pub white_point: [uint16_t; 2],
    pub max_luminance: uint32_t,
    pub min_luminance: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dContentLightLevel {
    pub max_content_light_level: libc::c_int,
    pub max_frame_average_light_level: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dPictureParameters {
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub layout: Dav1dPixelLayout,
    pub bpc: libc::c_int,
}
pub type Dav1dPixelLayout = libc::c_uint;
pub const DAV1D_PIXEL_LAYOUT_I444: Dav1dPixelLayout = 3;
pub const DAV1D_PIXEL_LAYOUT_I422: Dav1dPixelLayout = 2;
pub const DAV1D_PIXEL_LAYOUT_I420: Dav1dPixelLayout = 1;
pub const DAV1D_PIXEL_LAYOUT_I400: Dav1dPixelLayout = 0;
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
pub type Dav1dWarpedMotionType = libc::c_uint;
pub const DAV1D_WM_TYPE_AFFINE: Dav1dWarpedMotionType = 3;
pub const DAV1D_WM_TYPE_ROT_ZOOM: Dav1dWarpedMotionType = 2;
pub const DAV1D_WM_TYPE_TRANSLATION: Dav1dWarpedMotionType = 1;
pub const DAV1D_WM_TYPE_IDENTITY: Dav1dWarpedMotionType = 0;
pub type Dav1dTxfmMode = libc::c_uint;
pub const DAV1D_N_TX_MODES: Dav1dTxfmMode = 3;
pub const DAV1D_TX_SWITCHABLE: Dav1dTxfmMode = 2;
pub const DAV1D_TX_LARGEST: Dav1dTxfmMode = 1;
pub const DAV1D_TX_4X4_ONLY: Dav1dTxfmMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub type_0: [Dav1dRestorationType; 3],
    pub unit_size: [libc::c_int; 2],
}
pub type Dav1dRestorationType = libc::c_uint;
pub const DAV1D_RESTORATION_SGRPROJ: Dav1dRestorationType = 3;
pub const DAV1D_RESTORATION_WIENER: Dav1dRestorationType = 2;
pub const DAV1D_RESTORATION_SWITCHABLE: Dav1dRestorationType = 1;
pub const DAV1D_RESTORATION_NONE: Dav1dRestorationType = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dLoopfilterModeRefDeltas {
    pub mode_delta: [libc::c_int; 2],
    pub ref_delta: [libc::c_int; 8],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dSegmentationDataSet {
    pub d: [Dav1dSegmentationData; 8],
    pub preskip: libc::c_int,
    pub last_active_segid: libc::c_int,
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
pub type Dav1dFilterMode = libc::c_uint;
pub const DAV1D_FILTER_SWITCHABLE: Dav1dFilterMode = 4;
pub const DAV1D_N_FILTERS: Dav1dFilterMode = 4;
pub const DAV1D_FILTER_BILINEAR: Dav1dFilterMode = 3;
pub const DAV1D_N_SWITCHABLE_FILTERS: Dav1dFilterMode = 3;
pub const DAV1D_FILTER_8TAP_SHARP: Dav1dFilterMode = 2;
pub const DAV1D_FILTER_8TAP_SMOOTH: Dav1dFilterMode = 1;
pub const DAV1D_FILTER_8TAP_REGULAR: Dav1dFilterMode = 0;
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
pub type Dav1dFrameType = libc::c_uint;
pub const DAV1D_FRAME_TYPE_SWITCH: Dav1dFrameType = 3;
pub const DAV1D_FRAME_TYPE_INTRA: Dav1dFrameType = 2;
pub const DAV1D_FRAME_TYPE_INTER: Dav1dFrameType = 1;
pub const DAV1D_FRAME_TYPE_KEY: Dav1dFrameType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub data: Dav1dFilmGrainData,
    pub present: libc::c_int,
    pub update: libc::c_int,
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
pub type Dav1dAdaptiveBoolean = libc::c_uint;
pub const DAV1D_ADAPTIVE: Dav1dAdaptiveBoolean = 2;
pub const DAV1D_ON: Dav1dAdaptiveBoolean = 1;
pub const DAV1D_OFF: Dav1dAdaptiveBoolean = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: libc::c_ulonglong,
    pub __value32: C2RustUnnamed_18,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
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
    pub cdef_line: [[*mut libc::c_void; 3]; 2],
    pub cdef_lpf_line: [*mut libc::c_void; 3],
    pub lr_lpf_line: [*mut libc::c_void; 3],
    pub start_of_tile_row: *mut uint8_t,
    pub start_of_tile_row_sz: libc::c_int,
    pub need_cdef_lpf_copy: libc::c_int,
    pub p: [*mut libc::c_void; 3],
    pub sr_p: [*mut libc::c_void; 3],
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
pub type pixel = ();
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
    pub cf: *mut libc::c_void,
    pub prog_sz: libc::c_int,
    pub pal_sz: libc::c_int,
    pub pal_idx_sz: libc::c_int,
    pub cf_sz: libc::c_int,
    pub tile_start_off: *mut libc::c_int,
}
pub type coef = ();
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
    pub cf: *mut libc::c_void,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MsacContext {
    pub buf_pos: *const uint8_t,
    pub buf_end: *const uint8_t,
    pub dif: ec_win,
    pub rng: libc::c_uint,
    pub cnt: libc::c_int,
    pub allow_update_cdf: libc::c_int,
}
pub type ec_win = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdfContext {
    pub m: CdfModeContext,
    pub kfym: [[[uint16_t; 16]; 5]; 5],
    pub coef: CdfCoefContext,
    pub mv: CdfMvContext,
    pub dmv: CdfMvContext,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdfMvContext {
    pub comp: [CdfMvComponent; 2],
    pub joint: [uint16_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdfMvComponent {
    pub classes: [uint16_t; 16],
    pub class0_fp: [[uint16_t; 4]; 2],
    pub classN_fp: [uint16_t; 4],
    pub class0_hp: [uint16_t; 2],
    pub classN_hp: [uint16_t; 2],
    pub class0: [uint16_t; 2],
    pub classN: [[uint16_t; 2]; 10],
    pub sign: [uint16_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdfCoefContext {
    pub eob_bin_16: [[[uint16_t; 8]; 2]; 2],
    pub eob_bin_32: [[[uint16_t; 8]; 2]; 2],
    pub eob_bin_64: [[[uint16_t; 8]; 2]; 2],
    pub eob_bin_128: [[[uint16_t; 8]; 2]; 2],
    pub eob_bin_256: [[[uint16_t; 16]; 2]; 2],
    pub eob_bin_512: [[uint16_t; 16]; 2],
    pub eob_bin_1024: [[uint16_t; 16]; 2],
    pub eob_base_tok: [[[[uint16_t; 4]; 4]; 2]; 5],
    pub base_tok: [[[[uint16_t; 4]; 41]; 2]; 5],
    pub br_tok: [[[[uint16_t; 4]; 21]; 2]; 4],
    pub eob_hi_bit: [[[[uint16_t; 2]; 11]; 2]; 5],
    pub skip: [[[uint16_t; 2]; 13]; 5],
    pub dc_sign: [[[uint16_t; 2]; 3]; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdfModeContext {
    pub y_mode: [[uint16_t; 16]; 4],
    pub uv_mode: [[[uint16_t; 16]; 13]; 2],
    pub wedge_idx: [[uint16_t; 16]; 9],
    pub partition: [[[uint16_t; 16]; 4]; 5],
    pub cfl_alpha: [[uint16_t; 16]; 6],
    pub txtp_inter1: [[uint16_t; 16]; 2],
    pub txtp_inter2: [uint16_t; 16],
    pub txtp_intra1: [[[uint16_t; 8]; 13]; 2],
    pub txtp_intra2: [[[uint16_t; 8]; 13]; 3],
    pub cfl_sign: [uint16_t; 8],
    pub angle_delta: [[uint16_t; 8]; 8],
    pub filter_intra: [uint16_t; 8],
    pub comp_inter_mode: [[uint16_t; 8]; 8],
    pub seg_id: [[uint16_t; 8]; 3],
    pub pal_sz: [[[uint16_t; 8]; 7]; 2],
    pub color_map: [[[[uint16_t; 8]; 5]; 7]; 2],
    pub filter: [[[uint16_t; 4]; 8]; 2],
    pub txsz: [[[uint16_t; 4]; 3]; 4],
    pub motion_mode: [[uint16_t; 4]; 22],
    pub delta_q: [uint16_t; 4],
    pub delta_lf: [[uint16_t; 4]; 5],
    pub interintra_mode: [[uint16_t; 4]; 4],
    pub restore_switchable: [uint16_t; 4],
    pub restore_wiener: [uint16_t; 2],
    pub restore_sgrproj: [uint16_t; 2],
    pub interintra: [[uint16_t; 2]; 7],
    pub interintra_wedge: [[uint16_t; 2]; 7],
    pub txtp_inter3: [[uint16_t; 2]; 4],
    pub use_filter_intra: [[uint16_t; 2]; 22],
    pub newmv_mode: [[uint16_t; 2]; 6],
    pub globalmv_mode: [[uint16_t; 2]; 2],
    pub refmv_mode: [[uint16_t; 2]; 6],
    pub drl_bit: [[uint16_t; 2]; 3],
    pub intra: [[uint16_t; 2]; 4],
    pub comp: [[uint16_t; 2]; 5],
    pub comp_dir: [[uint16_t; 2]; 5],
    pub jnt_comp: [[uint16_t; 2]; 6],
    pub mask_comp: [[uint16_t; 2]; 6],
    pub wedge_comp: [[uint16_t; 2]; 9],
    pub ref_0: [[[uint16_t; 2]; 3]; 6],
    pub comp_fwd_ref: [[[uint16_t; 2]; 3]; 3],
    pub comp_bwd_ref: [[[uint16_t; 2]; 3]; 2],
    pub comp_uni_ref: [[[uint16_t; 2]; 3]; 3],
    pub txpart: [[[uint16_t; 2]; 3]; 7],
    pub skip: [[uint16_t; 2]; 3],
    pub skip_mode: [[uint16_t; 2]; 3],
    pub seg_pred: [[uint16_t; 2]; 3],
    pub obmc: [[uint16_t; 2]; 22],
    pub pal_y: [[[uint16_t; 2]; 3]; 7],
    pub pal_uv: [[uint16_t; 2]; 2],
    pub intrabc: [uint16_t; 2],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dMemPool {
    pub lock: pthread_mutex_t,
    pub buf: *mut Dav1dMemPoolBuffer,
    pub ref_cnt: libc::c_int,
    pub end: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dMemPoolBuffer {
    pub data: *mut libc::c_void,
    pub next: *mut Dav1dMemPoolBuffer,
}
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
        *mut libc::c_void,
        ptrdiff_t,
        const_left_pixel_row,
        *const libc::c_void,
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
pub type const_left_pixel_row = *const libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dCdefDSPContext {
    pub dir: cdef_dir_fn,
    pub fb: [cdef_fn; 3],
}
pub type cdef_fn = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        const_left_pixel_row_2px,
        *const libc::c_void,
        *const libc::c_void,
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
pub type const_left_pixel_row_2px = *const libc::c_void;
pub type cdef_dir_fn = Option::<
    unsafe extern "C" fn(
        *const libc::c_void,
        ptrdiff_t,
        *mut libc::c_uint,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dLoopFilterDSPContext {
    pub loop_filter_sb: [[loopfilter_sb_fn; 2]; 2],
}
pub type loopfilter_sb_fn = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
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
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *mut libc::c_void,
        libc::c_int,
    ) -> (),
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
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
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
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
        ptrdiff_t,
    ) -> (),
>;
pub type warp8x8t_fn = Option::<
    unsafe extern "C" fn(
        *mut int16_t,
        ptrdiff_t,
        *const libc::c_void,
        ptrdiff_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type warp8x8_fn = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
        ptrdiff_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type blend_dir_fn = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type blend_fn = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
        libc::c_int,
        libc::c_int,
        *const uint8_t,
    ) -> (),
>;
pub type w_mask_fn = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
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
        *mut libc::c_void,
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
        *mut libc::c_void,
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
        *mut libc::c_void,
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
        *const libc::c_void,
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
        *const libc::c_void,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mc_scaled_fn = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
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
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
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
        *mut libc::c_void,
        ptrdiff_t,
        *const uint16_t,
        *const uint8_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type cfl_pred_fn = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
        libc::c_int,
        libc::c_int,
        *const int16_t,
        libc::c_int,
    ) -> (),
>;
pub type cfl_ac_fn = Option::<
    unsafe extern "C" fn(
        *mut int16_t,
        *const libc::c_void,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type angular_ipred_fn = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
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
        *mut libc::c_void,
        *const libc::c_void,
        ptrdiff_t,
        *const Dav1dFilmGrainData,
        size_t,
        *const uint8_t,
        *const [entry; 82],
        libc::c_int,
        libc::c_int,
        *const libc::c_void,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type entry = int8_t;
pub type fgy_32x32xn_fn = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_void,
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
#[inline]
unsafe extern "C" fn ctz(mask: libc::c_uint) -> libc::c_int {
    return mask.trailing_zeros() as i32;
}
#[inline]
unsafe extern "C" fn dav1d_set_thread_name(name: *const libc::c_char) {
    prctl(15 as libc::c_int, name);
}
#[inline]
unsafe extern "C" fn imax(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn umin(a: libc::c_uint, b: libc::c_uint) -> libc::c_uint {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn iclip(
    v: libc::c_int,
    min: libc::c_int,
    max: libc::c_int,
) -> libc::c_int {
    return if v < min { min } else if v > max { max } else { v };
}
#[inline]
unsafe extern "C" fn reset_task_cur(
    c: *const Dav1dContext,
    ttd: *mut TaskThreadData,
    mut frame_idx: libc::c_uint,
) -> libc::c_int {
    let mut min_frame_idx: libc::c_uint = 0;
    let mut cur_frame_idx: libc::c_uint = 0;
    let mut current_block: u64;
    let first: libc::c_uint = ::core::intrinsics::atomic_load_seqcst(&mut (*ttd).first);
    let mut reset_frame_idx: libc::c_uint = ::core::intrinsics::atomic_xchg_seqcst(
        &mut (*ttd).reset_task_cur,
        (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint),
    );
    if reset_frame_idx < first {
        if frame_idx
            == (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint)
        {
            return 0 as libc::c_int;
        }
        reset_frame_idx = (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint);
    }
    if (*ttd).cur == 0
        && ((*((*c).fc).offset(first as isize)).task_thread.task_cur_prev).is_null()
    {
        return 0 as libc::c_int;
    }
    if reset_frame_idx
        != (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint)
    {
        if frame_idx
            == (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint)
        {
            if reset_frame_idx > first.wrapping_add((*ttd).cur) {
                return 0 as libc::c_int;
            }
            (*ttd).cur = reset_frame_idx.wrapping_sub(first);
            current_block = 8033822862565898926;
        } else {
            current_block = 5399440093318478209;
        }
    } else {
        if frame_idx
            == (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint)
        {
            return 0 as libc::c_int;
        }
        current_block = 5399440093318478209;
    }
    match current_block {
        5399440093318478209 => {
            if frame_idx < first {
                frame_idx = frame_idx.wrapping_add((*c).n_fc);
            }
            min_frame_idx = umin(reset_frame_idx, frame_idx);
            cur_frame_idx = first.wrapping_add((*ttd).cur);
            if (*ttd).cur < (*c).n_fc && cur_frame_idx < min_frame_idx {
                return 0 as libc::c_int;
            }
            (*ttd).cur = min_frame_idx.wrapping_sub(first);
            while (*ttd).cur < (*c).n_fc {
                if !((*((*c).fc)
                    .offset(
                        first.wrapping_add((*ttd).cur).wrapping_rem((*c).n_fc) as isize,
                    ))
                    .task_thread
                    .task_head)
                    .is_null()
                {
                    break;
                }
                (*ttd).cur = ((*ttd).cur).wrapping_add(1);
            }
        }
        _ => {}
    }
    let mut i: libc::c_uint = (*ttd).cur;
    while i < (*c).n_fc {
        let ref mut fresh0 = (*((*c).fc)
            .offset(first.wrapping_add(i).wrapping_rem((*c).n_fc) as isize))
            .task_thread
            .task_cur_prev;
        *fresh0 = 0 as *mut Dav1dTask;
        i = i.wrapping_add(1);
    }
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn reset_task_cur_async(
    ttd: *mut TaskThreadData,
    mut frame_idx: libc::c_uint,
    mut n_frames: libc::c_uint,
) {
    let first: libc::c_uint = ::core::intrinsics::atomic_load_seqcst(&mut (*ttd).first);
    if frame_idx < first {
        frame_idx = frame_idx.wrapping_add(n_frames);
    }
    let mut last_idx: libc::c_uint = frame_idx;
    loop {
        frame_idx = last_idx;
        last_idx = ::core::intrinsics::atomic_xchg_seqcst(
            &mut (*ttd).reset_task_cur,
            frame_idx,
        );
        if !(last_idx < frame_idx) {
            break;
        }
    }
    if frame_idx == first
        && ::core::intrinsics::atomic_load_seqcst(&mut (*ttd).first as *mut atomic_uint)
            != first
    {
        let mut expected: libc::c_uint = frame_idx;
        let fresh1 = ::core::intrinsics::atomic_cxchg_seqcst_seqcst(
            &mut (*ttd).reset_task_cur,
            *&mut expected,
            (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint),
        );
        *&mut expected = fresh1.0;
        fresh1.1;
    }
}
unsafe extern "C" fn insert_tasks_between(
    f: *mut Dav1dFrameContext,
    first: *mut Dav1dTask,
    last: *mut Dav1dTask,
    a: *mut Dav1dTask,
    b: *mut Dav1dTask,
    cond_signal: libc::c_int,
) {
    let ttd: *mut TaskThreadData = (*f).task_thread.ttd;
    if ::core::intrinsics::atomic_load_seqcst((*(*f).c).flush) != 0 {
        return;
    }
    if !(a.is_null() || (*a).next == b) {
        unreachable!();
    }
    if a.is_null() {
        (*f).task_thread.task_head = first;
    } else {
        (*a).next = first;
    }
    if b.is_null() {
        (*f).task_thread.task_tail = last;
    }
    (*last).next = b;
    reset_task_cur((*f).c, ttd, (*first).frame_idx);
    if cond_signal != 0
        && {
            let fresh2 = &mut (*ttd).cond_signaled as *mut atomic_int;
            let fresh3 = 1 as libc::c_int;
            ::core::intrinsics::atomic_or_seqcst(fresh2, fresh3) == 0
        }
    {
        pthread_cond_signal(&mut (*ttd).cond);
    }
}
unsafe extern "C" fn insert_tasks(
    f: *mut Dav1dFrameContext,
    first: *mut Dav1dTask,
    last: *mut Dav1dTask,
    cond_signal: libc::c_int,
) {
    let mut t_ptr: *mut Dav1dTask = 0 as *mut Dav1dTask;
    let mut prev_t: *mut Dav1dTask = 0 as *mut Dav1dTask;
    let mut current_block_34: u64;
    t_ptr = (*f).task_thread.task_head;
    while !t_ptr.is_null() {
        if (*t_ptr).type_0 as libc::c_uint
            == DAV1D_TASK_TYPE_TILE_ENTROPY as libc::c_int as libc::c_uint
        {
            if (*first).type_0 as libc::c_uint
                > DAV1D_TASK_TYPE_TILE_ENTROPY as libc::c_int as libc::c_uint
            {
                current_block_34 = 11174649648027449784;
            } else if (*first).sby > (*t_ptr).sby {
                current_block_34 = 11174649648027449784;
            } else {
                if (*first).sby < (*t_ptr).sby {
                    insert_tasks_between(f, first, last, prev_t, t_ptr, cond_signal);
                    return;
                }
                current_block_34 = 15904375183555213903;
            }
        } else {
            if (*first).type_0 as libc::c_uint
                == DAV1D_TASK_TYPE_TILE_ENTROPY as libc::c_int as libc::c_uint
            {
                insert_tasks_between(f, first, last, prev_t, t_ptr, cond_signal);
                return;
            }
            if (*first).sby > (*t_ptr).sby {
                current_block_34 = 11174649648027449784;
            } else {
                if (*first).sby < (*t_ptr).sby {
                    insert_tasks_between(f, first, last, prev_t, t_ptr, cond_signal);
                    return;
                }
                if (*first).type_0 as libc::c_uint > (*t_ptr).type_0 as libc::c_uint {
                    current_block_34 = 11174649648027449784;
                } else {
                    if ((*first).type_0 as libc::c_uint)
                        < (*t_ptr).type_0 as libc::c_uint
                    {
                        insert_tasks_between(f, first, last, prev_t, t_ptr, cond_signal);
                        return;
                    }
                    current_block_34 = 15904375183555213903;
                }
            }
        }
        match current_block_34 {
            15904375183555213903 => {
                if !((*first).type_0 as libc::c_uint
                    == DAV1D_TASK_TYPE_TILE_RECONSTRUCTION as libc::c_int as libc::c_uint
                    || (*first).type_0 as libc::c_uint
                        == DAV1D_TASK_TYPE_TILE_ENTROPY as libc::c_int as libc::c_uint)
                {
                    unreachable!();
                }
                if !((*first).type_0 as libc::c_uint == (*t_ptr).type_0 as libc::c_uint)
                {
                    unreachable!();
                }
                if !((*t_ptr).sby == (*first).sby) {
                    unreachable!();
                }
                let p: libc::c_int = ((*first).type_0 as libc::c_uint
                    == DAV1D_TASK_TYPE_TILE_ENTROPY as libc::c_int as libc::c_uint)
                    as libc::c_int;
                let t_tile_idx: libc::c_int = first
                    .offset_from((*f).task_thread.tile_tasks[p as usize]) as libc::c_long
                    as libc::c_int;
                let p_tile_idx: libc::c_int = t_ptr
                    .offset_from((*f).task_thread.tile_tasks[p as usize]) as libc::c_long
                    as libc::c_int;
                if !(t_tile_idx != p_tile_idx) {
                    unreachable!();
                }
                if !(t_tile_idx > p_tile_idx) {
                    insert_tasks_between(f, first, last, prev_t, t_ptr, cond_signal);
                    return;
                }
            }
            _ => {}
        }
        prev_t = t_ptr;
        t_ptr = (*t_ptr).next;
    }
    insert_tasks_between(f, first, last, prev_t, 0 as *mut Dav1dTask, cond_signal);
}
#[inline]
unsafe extern "C" fn insert_task(
    f: *mut Dav1dFrameContext,
    t: *mut Dav1dTask,
    cond_signal: libc::c_int,
) {
    insert_tasks(f, t, t, cond_signal);
}
#[inline]
unsafe extern "C" fn add_pending(f: *mut Dav1dFrameContext, t: *mut Dav1dTask) {
    pthread_mutex_lock(&mut (*f).task_thread.pending_tasks.lock);
    (*t).next = 0 as *mut Dav1dTask;
    if ((*f).task_thread.pending_tasks.head).is_null() {
        (*f).task_thread.pending_tasks.head = t;
    } else {
        (*(*f).task_thread.pending_tasks.tail).next = t;
    }
    (*f).task_thread.pending_tasks.tail = t;
    ::core::intrinsics::atomic_store_seqcst(
        &mut (*f).task_thread.pending_tasks.merge,
        1 as libc::c_int,
    );
    pthread_mutex_unlock(&mut (*f).task_thread.pending_tasks.lock);
}
#[inline]
unsafe extern "C" fn merge_pending_frame(f: *mut Dav1dFrameContext) -> libc::c_int {
    let merge: libc::c_int = ::core::intrinsics::atomic_load_seqcst(
        &mut (*f).task_thread.pending_tasks.merge,
    );
    if merge != 0 {
        pthread_mutex_lock(&mut (*f).task_thread.pending_tasks.lock);
        let mut t: *mut Dav1dTask = (*f).task_thread.pending_tasks.head;
        (*f).task_thread.pending_tasks.head = 0 as *mut Dav1dTask;
        (*f).task_thread.pending_tasks.tail = 0 as *mut Dav1dTask;
        ::core::intrinsics::atomic_store_seqcst(
            &mut (*f).task_thread.pending_tasks.merge,
            0 as libc::c_int,
        );
        pthread_mutex_unlock(&mut (*f).task_thread.pending_tasks.lock);
        while !t.is_null() {
            let tmp: *mut Dav1dTask = (*t).next;
            insert_task(f, t, 0 as libc::c_int);
            t = tmp;
        }
    }
    return merge;
}
#[inline]
unsafe extern "C" fn merge_pending(c: *const Dav1dContext) -> libc::c_int {
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*c).n_fc {
        res |= merge_pending_frame(&mut *((*c).fc).offset(i as isize));
        i = i.wrapping_add(1);
    }
    return res;
}
unsafe extern "C" fn create_filter_sbrow(
    f: *mut Dav1dFrameContext,
    pass: libc::c_int,
    mut res_t: *mut *mut Dav1dTask,
) -> libc::c_int {
    let has_deblock: libc::c_int = ((*(*f).frame_hdr)
        .loopfilter
        .level_y[0 as libc::c_int as usize] != 0
        || (*(*f).frame_hdr).loopfilter.level_y[1 as libc::c_int as usize] != 0)
        as libc::c_int;
    let has_cdef: libc::c_int = (*(*f).seq_hdr).cdef;
    let has_resize: libc::c_int = ((*(*f).frame_hdr).width[0 as libc::c_int as usize]
        != (*(*f).frame_hdr).width[1 as libc::c_int as usize]) as libc::c_int;
    let has_lr: libc::c_int = (*f).lf.restore_planes;
    let mut tasks: *mut Dav1dTask = (*f).task_thread.tasks;
    let uses_2pass: libc::c_int = ((*(*f).c).n_fc > 1 as libc::c_int as libc::c_uint)
        as libc::c_int;
    let mut num_tasks: libc::c_int = (*f).sbh * (1 as libc::c_int + uses_2pass);
    if num_tasks > (*f).task_thread.num_tasks {
        let size: size_t = (::core::mem::size_of::<Dav1dTask>() as libc::c_ulong)
            .wrapping_mul(num_tasks as libc::c_ulong);
        tasks = realloc((*f).task_thread.tasks as *mut libc::c_void, size)
            as *mut Dav1dTask;
        if tasks.is_null() {
            return -(1 as libc::c_int);
        }
        memset(tasks as *mut libc::c_void, 0 as libc::c_int, size);
        (*f).task_thread.tasks = tasks;
        (*f).task_thread.num_tasks = num_tasks;
    }
    tasks = tasks.offset(((*f).sbh * (pass & 1 as libc::c_int)) as isize);
    if pass & 1 as libc::c_int != 0 {
        (*f).frame_thread.entropy_progress = 0 as libc::c_int;
    } else {
        let prog_sz: libc::c_int = ((*f).sbh + 31 as libc::c_int & !(31 as libc::c_int))
            >> 5 as libc::c_int;
        if prog_sz > (*f).frame_thread.prog_sz {
            let prog: *mut atomic_uint = realloc(
                (*f).frame_thread.frame_progress as *mut libc::c_void,
                ((2 as libc::c_int * prog_sz) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<atomic_uint>() as libc::c_ulong),
            ) as *mut atomic_uint;
            if prog.is_null() {
                return -(1 as libc::c_int);
            }
            (*f).frame_thread.frame_progress = prog;
            (*f).frame_thread.copy_lpf_progress = prog.offset(prog_sz as isize);
        }
        (*f).frame_thread.prog_sz = prog_sz;
        memset(
            (*f).frame_thread.frame_progress as *mut libc::c_void,
            0 as libc::c_int,
            (prog_sz as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<atomic_uint>() as libc::c_ulong),
        );
        memset(
            (*f).frame_thread.copy_lpf_progress as *mut libc::c_void,
            0 as libc::c_int,
            (prog_sz as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<atomic_uint>() as libc::c_ulong),
        );
        ::core::intrinsics::atomic_store_seqcst(
            &mut (*f).frame_thread.deblock_progress,
            0 as libc::c_int,
        );
    }
    (*f)
        .frame_thread
        .next_tile_row[(pass & 1 as libc::c_int) as usize] = 0 as libc::c_int;
    let mut t: *mut Dav1dTask = &mut *tasks.offset(0 as libc::c_int as isize)
        as *mut Dav1dTask;
    (*t).sby = 0 as libc::c_int;
    (*t).recon_progress = 1 as libc::c_int;
    (*t).deblock_progress = 0 as libc::c_int;
    (*t)
        .type_0 = (if pass == 1 as libc::c_int {
        DAV1D_TASK_TYPE_ENTROPY_PROGRESS as libc::c_int
    } else if has_deblock != 0 {
        DAV1D_TASK_TYPE_DEBLOCK_COLS as libc::c_int
    } else if has_cdef != 0 || has_lr != 0 {
        DAV1D_TASK_TYPE_DEBLOCK_ROWS as libc::c_int
    } else if has_resize != 0 {
        DAV1D_TASK_TYPE_SUPER_RESOLUTION as libc::c_int
    } else {
        DAV1D_TASK_TYPE_RECONSTRUCTION_PROGRESS as libc::c_int
    }) as TaskType;
    (*t)
        .frame_idx = f.offset_from((*(*f).c).fc) as libc::c_long as libc::c_int
        as libc::c_uint;
    *res_t = t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_task_create_tile_sbrow(
    f: *mut Dav1dFrameContext,
    pass: libc::c_int,
    cond_signal: libc::c_int,
) -> libc::c_int {
    let mut tasks: *mut Dav1dTask = (*f)
        .task_thread
        .tile_tasks[0 as libc::c_int as usize];
    let uses_2pass: libc::c_int = ((*(*f).c).n_fc > 1 as libc::c_int as libc::c_uint)
        as libc::c_int;
    let num_tasks: libc::c_int = (*(*f).frame_hdr).tiling.cols
        * (*(*f).frame_hdr).tiling.rows;
    if pass < 2 as libc::c_int {
        let mut alloc_num_tasks: libc::c_int = num_tasks
            * (1 as libc::c_int + uses_2pass);
        if alloc_num_tasks > (*f).task_thread.num_tile_tasks {
            let size: size_t = (::core::mem::size_of::<Dav1dTask>() as libc::c_ulong)
                .wrapping_mul(alloc_num_tasks as libc::c_ulong);
            tasks = realloc(
                (*f).task_thread.tile_tasks[0 as libc::c_int as usize]
                    as *mut libc::c_void,
                size,
            ) as *mut Dav1dTask;
            if tasks.is_null() {
                return -(1 as libc::c_int);
            }
            memset(tasks as *mut libc::c_void, 0 as libc::c_int, size);
            (*f).task_thread.tile_tasks[0 as libc::c_int as usize] = tasks;
            (*f).task_thread.num_tile_tasks = alloc_num_tasks;
        }
        (*f)
            .task_thread
            .tile_tasks[1 as libc::c_int as usize] = tasks.offset(num_tasks as isize);
    }
    tasks = tasks.offset((num_tasks * (pass & 1 as libc::c_int)) as isize);
    let mut pf_t: *mut Dav1dTask = 0 as *mut Dav1dTask;
    if create_filter_sbrow(f, pass, &mut pf_t) != 0 {
        return -(1 as libc::c_int);
    }
    let mut prev_t: *mut Dav1dTask = 0 as *mut Dav1dTask;
    let mut tile_idx: libc::c_int = 0 as libc::c_int;
    while tile_idx < num_tasks {
        let ts: *mut Dav1dTileState = &mut *((*f).ts).offset(tile_idx as isize)
            as *mut Dav1dTileState;
        let mut t: *mut Dav1dTask = &mut *tasks.offset(tile_idx as isize)
            as *mut Dav1dTask;
        (*t).sby = (*ts).tiling.row_start >> (*f).sb_shift;
        if !pf_t.is_null() && (*t).sby != 0 {
            (*prev_t).next = pf_t;
            prev_t = pf_t;
            pf_t = 0 as *mut Dav1dTask;
        }
        (*t).recon_progress = 0 as libc::c_int;
        (*t).deblock_progress = 0 as libc::c_int;
        (*t).deps_skip = 0 as libc::c_int;
        (*t)
            .type_0 = (if pass != 1 as libc::c_int {
            DAV1D_TASK_TYPE_TILE_RECONSTRUCTION as libc::c_int
        } else {
            DAV1D_TASK_TYPE_TILE_ENTROPY as libc::c_int
        }) as TaskType;
        (*t)
            .frame_idx = f.offset_from((*(*f).c).fc) as libc::c_long as libc::c_int
            as libc::c_uint;
        if !prev_t.is_null() {
            (*prev_t).next = t;
        }
        prev_t = t;
        tile_idx += 1;
    }
    if !pf_t.is_null() {
        (*prev_t).next = pf_t;
        prev_t = pf_t;
    }
    (*prev_t).next = 0 as *mut Dav1dTask;
    ::core::intrinsics::atomic_store_seqcst(
        &mut *((*f).task_thread.done)
            .as_mut_ptr()
            .offset((pass & 1 as libc::c_int) as isize) as *mut atomic_int,
        0 as libc::c_int,
    );
    pthread_mutex_lock(&mut (*f).task_thread.pending_tasks.lock);
    if !(((*f).task_thread.pending_tasks.head).is_null() || pass == 2 as libc::c_int) {
        unreachable!();
    }
    if ((*f).task_thread.pending_tasks.head).is_null() {
        (*f)
            .task_thread
            .pending_tasks
            .head = &mut *tasks.offset(0 as libc::c_int as isize) as *mut Dav1dTask;
    } else {
        (*(*f).task_thread.pending_tasks.tail)
            .next = &mut *tasks.offset(0 as libc::c_int as isize) as *mut Dav1dTask;
    }
    (*f).task_thread.pending_tasks.tail = prev_t;
    ::core::intrinsics::atomic_store_seqcst(
        &mut (*f).task_thread.pending_tasks.merge,
        1 as libc::c_int,
    );
    pthread_mutex_unlock(&mut (*f).task_thread.pending_tasks.lock);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_task_frame_init(f: *mut Dav1dFrameContext) {
    let c: *const Dav1dContext = (*f).c;
    ::core::intrinsics::atomic_store_seqcst(
        &mut (*f).task_thread.init_done,
        0 as libc::c_int,
    );
    let t: *mut Dav1dTask = &mut (*f).task_thread.init_task;
    (*t).type_0 = DAV1D_TASK_TYPE_INIT;
    (*t)
        .frame_idx = f.offset_from((*c).fc) as libc::c_long as libc::c_int
        as libc::c_uint;
    (*t).sby = 0 as libc::c_int;
    (*t).deblock_progress = 0 as libc::c_int;
    (*t).recon_progress = (*t).deblock_progress;
    insert_task(f, t, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_task_delayed_fg(
    c: *mut Dav1dContext,
    out: *mut Dav1dPicture,
    in_0: *const Dav1dPicture,
) {
    let ttd: *mut TaskThreadData = &mut (*c).task_thread;
    (*ttd).delayed_fg.in_0 = in_0;
    (*ttd).delayed_fg.out = out;
    (*ttd).delayed_fg.type_0 = DAV1D_TASK_TYPE_FG_PREP;
    *(&mut *((*ttd).delayed_fg.progress).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut atomic_int) = 0 as libc::c_int;
    *(&mut *((*ttd).delayed_fg.progress).as_mut_ptr().offset(1 as libc::c_int as isize)
        as *mut atomic_int) = 0 as libc::c_int;
    pthread_mutex_lock(&mut (*ttd).lock);
    (*ttd).delayed_fg.exec = 1 as libc::c_int;
    pthread_cond_signal(&mut (*ttd).cond);
    pthread_cond_wait(&mut (*ttd).delayed_fg.cond, &mut (*ttd).lock);
    pthread_mutex_unlock(&mut (*ttd).lock);
}
#[inline]
unsafe extern "C" fn ensure_progress(
    ttd: *mut TaskThreadData,
    f: *mut Dav1dFrameContext,
    t: *mut Dav1dTask,
    type_0: TaskType,
    state: *mut atomic_int,
    target: *mut libc::c_int,
) -> libc::c_int {
    let mut p1: libc::c_int = ::core::intrinsics::atomic_load_seqcst(state);
    if p1 < (*t).sby {
        (*t).type_0 = type_0;
        (*t).deblock_progress = 0 as libc::c_int;
        (*t).recon_progress = (*t).deblock_progress;
        *target = (*t).sby;
        add_pending(f, t);
        pthread_mutex_lock(&mut (*ttd).lock);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn check_tile(
    t: *mut Dav1dTask,
    f: *mut Dav1dFrameContext,
    frame_mt: libc::c_int,
) -> libc::c_int {
    let tp: libc::c_int = ((*t).type_0 as libc::c_uint
        == DAV1D_TASK_TYPE_TILE_ENTROPY as libc::c_int as libc::c_uint) as libc::c_int;
    let tile_idx: libc::c_int = t.offset_from((*f).task_thread.tile_tasks[tp as usize])
        as libc::c_long as libc::c_int;
    let ts: *mut Dav1dTileState = &mut *((*f).ts).offset(tile_idx as isize)
        as *mut Dav1dTileState;
    let p1: libc::c_int = ::core::intrinsics::atomic_load_seqcst(
        &mut *((*ts).progress).as_mut_ptr().offset(tp as isize) as *mut atomic_int,
    );
    if p1 < (*t).sby {
        return 1 as libc::c_int;
    }
    let mut error: libc::c_int = (p1 == 2147483647 as libc::c_int - 1 as libc::c_int)
        as libc::c_int;
    let fresh4 = &mut (*f).task_thread.error;
    let fresh5 = error;
    error |= ::core::intrinsics::atomic_or_seqcst(fresh4, fresh5);
    if error == 0 && frame_mt != 0 && tp == 0 {
        let p2: libc::c_int = ::core::intrinsics::atomic_load_seqcst(
            &mut *((*ts).progress).as_mut_ptr().offset(1 as libc::c_int as isize)
                as *mut atomic_int,
        );
        if p2 <= (*t).sby {
            return 1 as libc::c_int;
        }
        error = (p2 == 2147483647 as libc::c_int - 1 as libc::c_int) as libc::c_int;
        let fresh6 = &mut (*f).task_thread.error;
        let fresh7 = error;
        error |= ::core::intrinsics::atomic_or_seqcst(fresh6, fresh7);
    }
    if error == 0 && frame_mt != 0
        && (*(*f).frame_hdr).frame_type as libc::c_uint
            & 1 as libc::c_int as libc::c_uint != 0
    {
        let mut p: *const Dav1dThreadPicture = &mut (*f).sr_cur;
        let ss_ver: libc::c_int = ((*p).p.p.layout as libc::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
        let p_b: libc::c_uint = (((*t).sby + 1 as libc::c_int)
            << (*f).sb_shift + 2 as libc::c_int) as libc::c_uint;
        let tile_sby: libc::c_int = (*t).sby - ((*ts).tiling.row_start >> (*f).sb_shift);
        let lowest_px: *const [libc::c_int; 2] = (*((*ts).lowest_pixel)
            .offset(tile_sby as isize))
            .as_mut_ptr() as *const [libc::c_int; 2];
        let mut current_block_14: u64;
        let mut n: libc::c_int = (*t).deps_skip;
        while n < 7 as libc::c_int {
            let mut lowest: libc::c_uint = 0;
            if tp != 0 {
                lowest = p_b;
                current_block_14 = 2370887241019905314;
            } else {
                let y: libc::c_int = if (*lowest_px
                    .offset(n as isize))[0 as libc::c_int as usize]
                    == -(2147483647 as libc::c_int) - 1 as libc::c_int
                {
                    -(2147483647 as libc::c_int) - 1 as libc::c_int
                } else {
                    (*lowest_px.offset(n as isize))[0 as libc::c_int as usize]
                        + 8 as libc::c_int
                };
                let uv: libc::c_int = if (*lowest_px
                    .offset(n as isize))[1 as libc::c_int as usize]
                    == -(2147483647 as libc::c_int) - 1 as libc::c_int
                {
                    -(2147483647 as libc::c_int) - 1 as libc::c_int
                } else {
                    (*lowest_px.offset(n as isize))[1 as libc::c_int as usize]
                        * ((1 as libc::c_int) << ss_ver) + 8 as libc::c_int
                };
                let max: libc::c_int = imax(y, uv);
                if max == -(2147483647 as libc::c_int) - 1 as libc::c_int {
                    current_block_14 = 7651349459974463963;
                } else {
                    lowest = iclip(max, 1 as libc::c_int, (*f).refp[n as usize].p.p.h)
                        as libc::c_uint;
                    current_block_14 = 2370887241019905314;
                }
            }
            match current_block_14 {
                2370887241019905314 => {
                    let p3: libc::c_uint = ::core::intrinsics::atomic_load_seqcst(
                        &mut *((*((*f).refp).as_mut_ptr().offset(n as isize)).progress)
                            .offset((tp == 0) as libc::c_int as isize)
                            as *mut atomic_uint,
                    );
                    if p3 < lowest {
                        return 1 as libc::c_int;
                    }
                    let fresh8 = &mut (*f).task_thread.error;
                    let fresh9 = (p3
                        == (2147483647 as libc::c_int as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint))
                        as libc::c_int;
                    ::core::intrinsics::atomic_or_seqcst(fresh8, fresh9);
                }
                _ => {}
            }
            n += 1;
            (*t).deps_skip += 1;
        }
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn get_frame_progress(
    c: *const Dav1dContext,
    f: *const Dav1dFrameContext,
) -> libc::c_int {
    let mut frame_prog: libc::c_uint = if (*c).n_fc > 1 as libc::c_int as libc::c_uint {
        ::core::intrinsics::atomic_load_seqcst(
            &mut *((*f).sr_cur.progress).offset(1 as libc::c_int as isize)
                as *mut atomic_uint,
        )
    } else {
        0 as libc::c_int as libc::c_uint
    };
    if frame_prog
        >= (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        return (*f).sbh - 1 as libc::c_int;
    }
    let mut idx: libc::c_int = (frame_prog >> (*f).sb_shift + 7 as libc::c_int)
        as libc::c_int;
    let mut prog: libc::c_int = 0;
    loop {
        let mut state: *mut atomic_uint = &mut *((*f).frame_thread.frame_progress)
            .offset(idx as isize) as *mut atomic_uint;
        let val: libc::c_uint = !::core::intrinsics::atomic_load_seqcst(state);
        prog = if val != 0 { ctz(val) } else { 32 as libc::c_int };
        if prog != 32 as libc::c_int {
            break;
        }
        prog = 0 as libc::c_int;
        idx += 1;
        if !(idx < (*f).frame_thread.prog_sz) {
            break;
        }
    }
    return (idx << 5 as libc::c_int | prog) - 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn abort_frame(f: *mut Dav1dFrameContext, error: libc::c_int) {
    ::core::intrinsics::atomic_store_seqcst(
        &mut (*f).task_thread.error,
        if error == -(22 as libc::c_int) {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        },
    );
    ::core::intrinsics::atomic_store_seqcst(
        &mut (*f).task_thread.task_counter,
        0 as libc::c_int,
    );
    ::core::intrinsics::atomic_store_seqcst(
        &mut *((*f).task_thread.done).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut atomic_int,
        1 as libc::c_int,
    );
    ::core::intrinsics::atomic_store_seqcst(
        &mut *((*f).task_thread.done).as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut atomic_int,
        1 as libc::c_int,
    );
    ::core::intrinsics::atomic_store_seqcst(
        &mut *((*f).sr_cur.progress).offset(0 as libc::c_int as isize)
            as *mut atomic_uint,
        (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    ::core::intrinsics::atomic_store_seqcst(
        &mut *((*f).sr_cur.progress).offset(1 as libc::c_int as isize)
            as *mut atomic_uint,
        (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    dav1d_decode_frame_exit(f, error);
    (*f).n_tile_data = 0 as libc::c_int;
    pthread_cond_signal(&mut (*f).task_thread.cond);
}
#[inline]
unsafe extern "C" fn delayed_fg_task(c: *const Dav1dContext, ttd: *mut TaskThreadData) {
    let in_0: *const Dav1dPicture = (*ttd).delayed_fg.in_0;
    let out: *mut Dav1dPicture = (*ttd).delayed_fg.out;
    let mut off: libc::c_int = 0;
    if (*out).p.bpc != 8 as libc::c_int {
        off = ((*out).p.bpc >> 1 as libc::c_int) - 4 as libc::c_int;
    }
    let mut row: libc::c_int = 0;
    let mut progmax: libc::c_int = 0;
    let mut done: libc::c_int = 0;
    match (*ttd).delayed_fg.type_0 as libc::c_uint {
        11 => {
            (*ttd).delayed_fg.exec = 0 as libc::c_int;
            if ::core::intrinsics::atomic_load_seqcst(
                &mut (*ttd).cond_signaled as *mut atomic_int,
            ) != 0
            {
                pthread_cond_signal(&mut (*ttd).cond);
            }
            pthread_mutex_unlock(&mut (*ttd).lock);
            match (*out).p.bpc {
                10 | 12 => {
                    dav1d_prep_grain_16bpc(
                        &(*((*c).dsp).as_ptr().offset(off as isize)).fg,
                        out,
                        in_0,
                        ((*ttd).delayed_fg.c2rust_unnamed.c2rust_unnamed_0.scaling_16bpc)
                            .as_mut_ptr() as *mut libc::c_void,
                        ((*ttd)
                            .delayed_fg
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .grain_lut_16bpc)
                            .as_mut_ptr() as *mut libc::c_void,
                    );
                }
                _ => {
                    abort();
                }
            }
            (*ttd).delayed_fg.type_0 = DAV1D_TASK_TYPE_FG_APPLY;
            pthread_mutex_lock(&mut (*ttd).lock);
            (*ttd).delayed_fg.exec = 1 as libc::c_int;
        }
        12 => {}
        _ => {
            abort();
        }
    }
    row = ::core::intrinsics::atomic_xadd_seqcst(
        &mut *((*ttd).delayed_fg.progress).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut atomic_int,
        1 as libc::c_int,
    );
    pthread_mutex_unlock(&mut (*ttd).lock);
    progmax = (*out).p.h + 31 as libc::c_int >> 5 as libc::c_int;
    loop {
        if (row + 1 as libc::c_int) < progmax {
            pthread_cond_signal(&mut (*ttd).cond);
        } else if row + 1 as libc::c_int >= progmax {
            pthread_mutex_lock(&mut (*ttd).lock);
            (*ttd).delayed_fg.exec = 0 as libc::c_int;
            if row >= progmax {
                break;
            }
            pthread_mutex_unlock(&mut (*ttd).lock);
        }
        match (*out).p.bpc {
            10 | 12 => {
                dav1d_apply_grain_row_16bpc(
                    &(*((*c).dsp).as_ptr().offset(off as isize)).fg,
                    out,
                    in_0,
                    ((*ttd).delayed_fg.c2rust_unnamed.c2rust_unnamed_0.scaling_16bpc)
                        .as_mut_ptr() as *mut libc::c_void,
                    ((*ttd).delayed_fg.c2rust_unnamed.c2rust_unnamed_0.grain_lut_16bpc)
                        .as_mut_ptr() as *mut libc::c_void,
                    row,
                );
            }
            _ => {
                abort();
            }
        }
        row = ::core::intrinsics::atomic_xadd_seqcst(
            &mut *((*ttd).delayed_fg.progress)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut atomic_int,
            1 as libc::c_int,
        );
        done = ::core::intrinsics::atomic_xadd_seqcst(
            &mut *((*ttd).delayed_fg.progress)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize) as *mut atomic_int,
            1 as libc::c_int,
        ) + 1 as libc::c_int;
        if row < progmax {
            continue;
        }
        pthread_mutex_lock(&mut (*ttd).lock);
        (*ttd).delayed_fg.exec = 0 as libc::c_int;
        break;
    }
    done = ::core::intrinsics::atomic_xadd_seqcst(
        &mut *((*ttd).delayed_fg.progress).as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut atomic_int,
        1 as libc::c_int,
    ) + 1 as libc::c_int;
    progmax = ::core::intrinsics::atomic_load_seqcst(
        &mut *((*ttd).delayed_fg.progress).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut atomic_int,
    );
    if !(done < progmax) {
        pthread_cond_signal(&mut (*ttd).delayed_fg.cond);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_worker_task(
    mut data: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut flush: libc::c_int = 0;
    let mut error_0: libc::c_int = 0;
    let mut sby: libc::c_int = 0;
    let mut f: *mut Dav1dFrameContext = 0 as *mut Dav1dFrameContext;
    let mut t: *mut Dav1dTask = 0 as *mut Dav1dTask;
    let mut prev_t: *mut Dav1dTask = 0 as *mut Dav1dTask;
    let mut current_block: u64;
    let tc: *mut Dav1dTaskContext = data as *mut Dav1dTaskContext;
    let c: *const Dav1dContext = (*tc).c;
    let ttd: *mut TaskThreadData = (*tc).task_thread.ttd;
    dav1d_set_thread_name(b"dav1d-worker\0" as *const u8 as *const libc::c_char);
    pthread_mutex_lock(&mut (*ttd).lock);
    's_18: while !((*tc).task_thread.die != 0) {
        if !(::core::intrinsics::atomic_load_seqcst((*c).flush) != 0) {
            merge_pending(c);
            if (*ttd).delayed_fg.exec != 0 {
                delayed_fg_task(c, ttd);
                continue;
            } else {
                f = 0 as *mut Dav1dFrameContext;
                t = 0 as *mut Dav1dTask;
                prev_t = 0 as *mut Dav1dTask;
                if (*c).n_fc > 1 as libc::c_int as libc::c_uint {
                    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                    loop {
                        if !(i < (*c).n_fc) {
                            current_block = 5601891728916014340;
                            break;
                        }
                        let first: libc::c_uint = ::core::intrinsics::atomic_load_seqcst(
                            &mut (*ttd).first,
                        );
                        f = &mut *((*c).fc)
                            .offset(
                                first.wrapping_add(i).wrapping_rem((*c).n_fc) as isize,
                            ) as *mut Dav1dFrameContext;
                        if !(::core::intrinsics::atomic_load_seqcst(
                            &mut (*f).task_thread.init_done as *mut atomic_int,
                        ) != 0)
                        {
                            t = (*f).task_thread.task_head;
                            if !t.is_null() {
                                if (*t).type_0 as libc::c_uint
                                    == DAV1D_TASK_TYPE_INIT as libc::c_int as libc::c_uint
                                {
                                    current_block = 13951626279954010388;
                                    break;
                                }
                                if (*t).type_0 as libc::c_uint
                                    == DAV1D_TASK_TYPE_INIT_CDF as libc::c_int as libc::c_uint
                                {
                                    let p1: libc::c_int = (if !((*f).in_cdf.progress).is_null()
                                    {
                                        ::core::intrinsics::atomic_load_seqcst((*f).in_cdf.progress)
                                    } else {
                                        1 as libc::c_int as libc::c_uint
                                    }) as libc::c_int;
                                    if p1 != 0 {
                                        let fresh18 = &mut (*f).task_thread.error;
                                        let fresh19 = (p1
                                            == 2147483647 as libc::c_int - 1 as libc::c_int)
                                            as libc::c_int;
                                        ::core::intrinsics::atomic_or_seqcst(fresh18, fresh19);
                                        current_block = 13951626279954010388;
                                        break;
                                    }
                                }
                            }
                        }
                        i = i.wrapping_add(1);
                    }
                } else {
                    current_block = 5601891728916014340;
                }
                's_107: loop {
                    match current_block {
                        5601891728916014340 => {
                            if (*ttd).cur < (*c).n_fc {
                                let first_0: libc::c_uint = ::core::intrinsics::atomic_load_seqcst(
                                    &mut (*ttd).first,
                                );
                                f = &mut *((*c).fc)
                                    .offset(
                                        first_0.wrapping_add((*ttd).cur).wrapping_rem((*c).n_fc)
                                            as isize,
                                    ) as *mut Dav1dFrameContext;
                                merge_pending_frame(f);
                                prev_t = (*f).task_thread.task_cur_prev;
                                t = if !prev_t.is_null() {
                                    (*prev_t).next
                                } else {
                                    (*f).task_thread.task_head
                                };
                                while !t.is_null() {
                                    if !((*t).type_0 as libc::c_uint
                                        == DAV1D_TASK_TYPE_INIT_CDF as libc::c_int as libc::c_uint)
                                    {
                                        if (*t).type_0 as libc::c_uint
                                            == DAV1D_TASK_TYPE_TILE_ENTROPY as libc::c_int
                                                as libc::c_uint
                                            || (*t).type_0 as libc::c_uint
                                                == DAV1D_TASK_TYPE_TILE_RECONSTRUCTION as libc::c_int
                                                    as libc::c_uint
                                        {
                                            if check_tile(
                                                t,
                                                f,
                                                ((*c).n_fc > 1 as libc::c_int as libc::c_uint)
                                                    as libc::c_int,
                                            ) == 0
                                            {
                                                current_block = 13951626279954010388;
                                                continue 's_107;
                                            }
                                        } else if (*t).recon_progress != 0 {
                                            let p: libc::c_int = ((*t).type_0 as libc::c_uint
                                                == DAV1D_TASK_TYPE_ENTROPY_PROGRESS as libc::c_int
                                                    as libc::c_uint) as libc::c_int;
                                            let mut error: libc::c_int = ::core::intrinsics::atomic_load_seqcst(
                                                &mut (*f).task_thread.error,
                                            );
                                            if !(::core::intrinsics::atomic_load_seqcst(
                                                &mut *((*f).task_thread.done)
                                                    .as_mut_ptr()
                                                    .offset(p as isize) as *mut atomic_int,
                                            ) == 0 || error != 0)
                                            {
                                                unreachable!();
                                            }
                                            let tile_row_base: libc::c_int = (*(*f).frame_hdr)
                                                .tiling
                                                .cols * (*f).frame_thread.next_tile_row[p as usize];
                                            if p != 0 {
                                                let prog: *mut atomic_int = &mut (*f)
                                                    .frame_thread
                                                    .entropy_progress;
                                                let p1_0: libc::c_int = ::core::intrinsics::atomic_load_seqcst(
                                                    prog,
                                                );
                                                if p1_0 < (*t).sby {
                                                    current_block = 1373748322570045674;
                                                } else {
                                                    let fresh20 = &mut (*f).task_thread.error;
                                                    let fresh21 = (p1_0
                                                        == 2147483647 as libc::c_int - 1 as libc::c_int)
                                                        as libc::c_int;
                                                    ::core::intrinsics::atomic_or_seqcst(fresh20, fresh21);
                                                    current_block = 14832935472441733737;
                                                }
                                            } else {
                                                current_block = 14832935472441733737;
                                            }
                                            match current_block {
                                                1373748322570045674 => {}
                                                _ => {
                                                    let mut tc_0: libc::c_int = 0 as libc::c_int;
                                                    loop {
                                                        if !(tc_0 < (*(*f).frame_hdr).tiling.cols) {
                                                            current_block = 3222590281903869779;
                                                            break;
                                                        }
                                                        let ts: *mut Dav1dTileState = &mut *((*f).ts)
                                                            .offset((tile_row_base + tc_0) as isize)
                                                            as *mut Dav1dTileState;
                                                        let p2: libc::c_int = ::core::intrinsics::atomic_load_seqcst(
                                                            &mut *((*ts).progress).as_mut_ptr().offset(p as isize)
                                                                as *mut atomic_int,
                                                        );
                                                        if p2 < (*t).recon_progress {
                                                            current_block = 1373748322570045674;
                                                            break;
                                                        }
                                                        let fresh22 = &mut (*f).task_thread.error;
                                                        let fresh23 = (p2
                                                            == 2147483647 as libc::c_int - 1 as libc::c_int)
                                                            as libc::c_int;
                                                        ::core::intrinsics::atomic_or_seqcst(fresh22, fresh23);
                                                        tc_0 += 1;
                                                    }
                                                    match current_block {
                                                        1373748322570045674 => {}
                                                        _ => {
                                                            if ((*t).sby + 1 as libc::c_int) < (*f).sbh {
                                                                let mut next_t: *mut Dav1dTask = &mut *t
                                                                    .offset(1 as libc::c_int as isize) as *mut Dav1dTask;
                                                                *next_t = *t;
                                                                (*next_t).sby += 1;
                                                                let ntr: libc::c_int = (*f)
                                                                    .frame_thread
                                                                    .next_tile_row[p as usize] + 1 as libc::c_int;
                                                                let start: libc::c_int = (*(*f).frame_hdr)
                                                                    .tiling
                                                                    .row_start_sb[ntr as usize] as libc::c_int;
                                                                if (*next_t).sby == start {
                                                                    (*f).frame_thread.next_tile_row[p as usize] = ntr;
                                                                }
                                                                (*next_t).recon_progress = (*next_t).sby + 1 as libc::c_int;
                                                                insert_task(f, next_t, 0 as libc::c_int);
                                                            }
                                                            current_block = 13951626279954010388;
                                                            continue 's_107;
                                                        }
                                                    }
                                                }
                                            }
                                        } else if (*t).type_0 as libc::c_uint
                                            == DAV1D_TASK_TYPE_CDEF as libc::c_int as libc::c_uint
                                        {
                                            let mut prog_0: *mut atomic_uint = (*f)
                                                .frame_thread
                                                .copy_lpf_progress;
                                            let p1_1: libc::c_int = ::core::intrinsics::atomic_load_seqcst(
                                                &mut *prog_0
                                                    .offset(
                                                        ((*t).sby - 1 as libc::c_int >> 5 as libc::c_int) as isize,
                                                    ) as *mut atomic_uint,
                                            ) as libc::c_int;
                                            if p1_1 as libc::c_uint
                                                & (1 as libc::c_uint)
                                                    << ((*t).sby - 1 as libc::c_int & 31 as libc::c_int) != 0
                                            {
                                                current_block = 13951626279954010388;
                                                continue 's_107;
                                            }
                                        } else {
                                            if (*t).deblock_progress == 0 {
                                                unreachable!();
                                            }
                                            let p1_2: libc::c_int = ::core::intrinsics::atomic_load_seqcst(
                                                &mut (*f).frame_thread.deblock_progress,
                                            );
                                            if p1_2 >= (*t).deblock_progress {
                                                let fresh24 = &mut (*f).task_thread.error;
                                                let fresh25 = (p1_2
                                                    == 2147483647 as libc::c_int - 1 as libc::c_int)
                                                    as libc::c_int;
                                                ::core::intrinsics::atomic_or_seqcst(fresh24, fresh25);
                                                current_block = 13951626279954010388;
                                                continue 's_107;
                                            }
                                        }
                                    }
                                    prev_t = t;
                                    t = (*t).next;
                                    (*f).task_thread.task_cur_prev = prev_t;
                                }
                                (*ttd).cur = ((*ttd).cur).wrapping_add(1);
                                current_block = 5601891728916014340;
                            } else {
                                if reset_task_cur(
                                    c,
                                    ttd,
                                    (2147483647 as libc::c_int as libc::c_uint)
                                        .wrapping_mul(2 as libc::c_uint)
                                        .wrapping_add(1 as libc::c_uint),
                                ) != 0
                                {
                                    continue 's_18;
                                }
                                if merge_pending(c) != 0 {
                                    continue 's_18;
                                } else {
                                    current_block = 2777461603309497930;
                                    break;
                                }
                            }
                        }
                        _ => {
                            if !prev_t.is_null() {
                                (*prev_t).next = (*t).next;
                            } else {
                                (*f).task_thread.task_head = (*t).next;
                            }
                            if ((*t).next).is_null() {
                                (*f).task_thread.task_tail = prev_t;
                            }
                            if (*t).type_0 as libc::c_uint
                                > DAV1D_TASK_TYPE_INIT_CDF as libc::c_int as libc::c_uint
                                && ((*f).task_thread.task_head).is_null()
                            {
                                (*ttd).cur = ((*ttd).cur).wrapping_add(1);
                            }
                            (*t).next = 0 as *mut Dav1dTask;
                            ::core::intrinsics::atomic_store_seqcst(
                                &mut (*ttd).cond_signaled,
                                1 as libc::c_int,
                            );
                            pthread_cond_signal(&mut (*ttd).cond);
                            pthread_mutex_unlock(&mut (*ttd).lock);
                            current_block = 8464383504555462953;
                            break;
                        }
                    }
                }
                match current_block {
                    2777461603309497930 => {}
                    _ => {
                        loop {
                            flush = ::core::intrinsics::atomic_load_seqcst((*c).flush);
                            let fresh26 = &mut (*f).task_thread.error;
                            let fresh27 = flush;
                            error_0 = ::core::intrinsics::atomic_or_seqcst(
                                fresh26,
                                fresh27,
                            ) | flush;
                            (*tc).f = f;
                            sby = (*t).sby;
                            match (*t).type_0 as libc::c_uint {
                                0 => {
                                    if !((*c).n_fc > 1 as libc::c_int as libc::c_uint) {
                                        unreachable!();
                                    }
                                    let mut res: libc::c_int = dav1d_decode_frame_init(f);
                                    let mut p1_3: libc::c_int = (if !((*f).in_cdf.progress)
                                        .is_null()
                                    {
                                        ::core::intrinsics::atomic_load_seqcst((*f).in_cdf.progress)
                                    } else {
                                        1 as libc::c_int as libc::c_uint
                                    }) as libc::c_int;
                                    if res != 0
                                        || p1_3 == 2147483647 as libc::c_int - 1 as libc::c_int
                                    {
                                        pthread_mutex_lock(&mut (*ttd).lock);
                                        abort_frame(
                                            f,
                                            if res != 0 { res } else { -(22 as libc::c_int) },
                                        );
                                        reset_task_cur(c, ttd, (*t).frame_idx);
                                        continue 's_18;
                                    } else {
                                        (*t).type_0 = DAV1D_TASK_TYPE_INIT_CDF;
                                        if p1_3 != 0 {
                                            continue;
                                        }
                                        add_pending(f, t);
                                        pthread_mutex_lock(&mut (*ttd).lock);
                                        continue 's_18;
                                    }
                                }
                                1 => {
                                    if !((*c).n_fc > 1 as libc::c_int as libc::c_uint) {
                                        unreachable!();
                                    }
                                    let mut res_0: libc::c_int = -(22 as libc::c_int);
                                    if ::core::intrinsics::atomic_load_seqcst(
                                        &mut (*f).task_thread.error as *mut atomic_int,
                                    ) == 0
                                    {
                                        res_0 = dav1d_decode_frame_init_cdf(f);
                                    }
                                    if (*(*f).frame_hdr).refresh_context != 0
                                        && (*f).task_thread.update_set == 0
                                    {
                                        ::core::intrinsics::atomic_store_seqcst(
                                            (*f).out_cdf.progress,
                                            (if res_0 < 0 as libc::c_int {
                                                2147483647 as libc::c_int - 1 as libc::c_int
                                            } else {
                                                1 as libc::c_int
                                            }) as libc::c_uint,
                                        );
                                    }
                                    if res_0 == 0 {
                                        if !((*c).n_fc > 1 as libc::c_int as libc::c_uint) {
                                            unreachable!();
                                        }
                                        let mut p_0: libc::c_int = 1 as libc::c_int;
                                        while p_0 <= 2 as libc::c_int {
                                            let res_1: libc::c_int = dav1d_task_create_tile_sbrow(
                                                f,
                                                p_0,
                                                0 as libc::c_int,
                                            );
                                            if res_1 != 0 {
                                                pthread_mutex_lock(&mut (*ttd).lock);
                                                ::core::intrinsics::atomic_store_seqcst(
                                                    &mut *((*f).task_thread.done)
                                                        .as_mut_ptr()
                                                        .offset((2 as libc::c_int - p_0) as isize)
                                                        as *mut atomic_int,
                                                    1 as libc::c_int,
                                                );
                                                ::core::intrinsics::atomic_store_seqcst(
                                                    &mut (*f).task_thread.error,
                                                    -(1 as libc::c_int),
                                                );
                                                let fresh28 = &mut (*f).task_thread.task_counter;
                                                let fresh29 = (*(*f).frame_hdr).tiling.cols
                                                    * (*(*f).frame_hdr).tiling.rows + (*f).sbh;
                                                ::core::intrinsics::atomic_xsub_seqcst(fresh28, fresh29);
                                                ::core::intrinsics::atomic_store_seqcst(
                                                    &mut *((*f).sr_cur.progress)
                                                        .offset((p_0 - 1 as libc::c_int) as isize)
                                                        as *mut atomic_uint,
                                                    (2147483647 as libc::c_int as libc::c_uint)
                                                        .wrapping_mul(2 as libc::c_uint)
                                                        .wrapping_add(1 as libc::c_uint)
                                                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                                                );
                                                if p_0 == 2 as libc::c_int
                                                    && ::core::intrinsics::atomic_load_seqcst(
                                                        &mut *((*f).task_thread.done)
                                                            .as_mut_ptr()
                                                            .offset(1 as libc::c_int as isize) as *mut atomic_int,
                                                    ) != 0
                                                {
                                                    if ::core::intrinsics::atomic_load_seqcst(
                                                        &mut (*f).task_thread.task_counter as *mut atomic_int,
                                                    ) != 0
                                                    {
                                                        unreachable!();
                                                    }
                                                    dav1d_decode_frame_exit(f, -(12 as libc::c_int));
                                                    (*f).n_tile_data = 0 as libc::c_int;
                                                    pthread_cond_signal(&mut (*f).task_thread.cond);
                                                    ::core::intrinsics::atomic_store_seqcst(
                                                        &mut (*f).task_thread.init_done,
                                                        1 as libc::c_int,
                                                    );
                                                } else {
                                                    pthread_mutex_unlock(&mut (*ttd).lock);
                                                }
                                            }
                                            p_0 += 1;
                                        }
                                        ::core::intrinsics::atomic_store_seqcst(
                                            &mut (*f).task_thread.init_done,
                                            1 as libc::c_int,
                                        );
                                        pthread_mutex_lock(&mut (*ttd).lock);
                                    } else {
                                        pthread_mutex_lock(&mut (*ttd).lock);
                                        abort_frame(f, res_0);
                                        reset_task_cur(c, ttd, (*t).frame_idx);
                                        ::core::intrinsics::atomic_store_seqcst(
                                            &mut (*f).task_thread.init_done,
                                            1 as libc::c_int,
                                        );
                                    }
                                    continue 's_18;
                                }
                                2 | 4 => {
                                    let p_1: libc::c_int = ((*t).type_0 as libc::c_uint
                                        == DAV1D_TASK_TYPE_TILE_ENTROPY as libc::c_int
                                            as libc::c_uint) as libc::c_int;
                                    let tile_idx: libc::c_int = t
                                        .offset_from((*f).task_thread.tile_tasks[p_1 as usize])
                                        as libc::c_long as libc::c_int;
                                    let ts_0: *mut Dav1dTileState = &mut *((*f).ts)
                                        .offset(tile_idx as isize) as *mut Dav1dTileState;
                                    (*tc).ts = ts_0;
                                    (*tc).by = sby << (*f).sb_shift;
                                    let uses_2pass: libc::c_int = ((*c).n_fc
                                        > 1 as libc::c_int as libc::c_uint) as libc::c_int;
                                    (*tc)
                                        .frame_thread
                                        .pass = if uses_2pass == 0 {
                                        0 as libc::c_int
                                    } else {
                                        1 as libc::c_int
                                            + ((*t).type_0 as libc::c_uint
                                                == DAV1D_TASK_TYPE_TILE_RECONSTRUCTION as libc::c_int
                                                    as libc::c_uint) as libc::c_int
                                    };
                                    if error_0 == 0 {
                                        error_0 = dav1d_decode_tile_sbrow(tc);
                                    }
                                    let progress: libc::c_int = if error_0 != 0 {
                                        2147483647 as libc::c_int - 1 as libc::c_int
                                    } else {
                                        1 as libc::c_int + sby
                                    };
                                    let fresh30 = &mut (*f).task_thread.error;
                                    let fresh31 = error_0;
                                    ::core::intrinsics::atomic_or_seqcst(fresh30, fresh31);
                                    if (sby + 1 as libc::c_int) << (*f).sb_shift
                                        < (*ts_0).tiling.row_end
                                    {
                                        (*t).sby += 1;
                                        (*t).deps_skip = 0 as libc::c_int;
                                        if check_tile(t, f, uses_2pass) == 0 {
                                            ::core::intrinsics::atomic_store_seqcst(
                                                &mut *((*ts_0).progress).as_mut_ptr().offset(p_1 as isize)
                                                    as *mut atomic_int,
                                                progress,
                                            );
                                            reset_task_cur_async(ttd, (*t).frame_idx, (*c).n_fc);
                                            let fresh32 = &mut (*ttd).cond_signaled as *mut atomic_int;
                                            let fresh33 = 1 as libc::c_int;
                                            if ::core::intrinsics::atomic_or_seqcst(fresh32, fresh33)
                                                == 0
                                            {
                                                pthread_cond_signal(&mut (*ttd).cond);
                                            }
                                        } else {
                                            ::core::intrinsics::atomic_store_seqcst(
                                                &mut *((*ts_0).progress).as_mut_ptr().offset(p_1 as isize)
                                                    as *mut atomic_int,
                                                progress,
                                            );
                                            add_pending(f, t);
                                            pthread_mutex_lock(&mut (*ttd).lock);
                                            continue 's_18;
                                        }
                                    } else {
                                        pthread_mutex_lock(&mut (*ttd).lock);
                                        ::core::intrinsics::atomic_store_seqcst(
                                            &mut *((*ts_0).progress).as_mut_ptr().offset(p_1 as isize)
                                                as *mut atomic_int,
                                            progress,
                                        );
                                        reset_task_cur(c, ttd, (*t).frame_idx);
                                        error_0 = ::core::intrinsics::atomic_load_seqcst(
                                            &mut (*f).task_thread.error,
                                        );
                                        if (*(*f).frame_hdr).refresh_context != 0
                                            && (*tc).frame_thread.pass <= 1 as libc::c_int
                                            && (*f).task_thread.update_set != 0
                                            && (*(*f).frame_hdr).tiling.update == tile_idx
                                        {
                                            if error_0 == 0 {
                                                dav1d_cdf_thread_update(
                                                    (*f).frame_hdr,
                                                    (*f).out_cdf.data.cdf,
                                                    &mut (*((*f).ts)
                                                        .offset((*(*f).frame_hdr).tiling.update as isize))
                                                        .cdf,
                                                );
                                            }
                                            if (*c).n_fc > 1 as libc::c_int as libc::c_uint {
                                                ::core::intrinsics::atomic_store_seqcst(
                                                    (*f).out_cdf.progress,
                                                    (if error_0 != 0 {
                                                        2147483647 as libc::c_int - 1 as libc::c_int
                                                    } else {
                                                        1 as libc::c_int
                                                    }) as libc::c_uint,
                                                );
                                            }
                                        }
                                        let fresh34 = &mut (*f).task_thread.task_counter
                                            as *mut atomic_int;
                                        let fresh35 = 1 as libc::c_int;
                                        if ::core::intrinsics::atomic_xsub_seqcst(fresh34, fresh35)
                                            - 1 as libc::c_int == 0 as libc::c_int
                                            && ::core::intrinsics::atomic_load_seqcst(
                                                &mut *((*f).task_thread.done)
                                                    .as_mut_ptr()
                                                    .offset(0 as libc::c_int as isize) as *mut atomic_int,
                                            ) != 0
                                            && (uses_2pass == 0
                                                || ::core::intrinsics::atomic_load_seqcst(
                                                    &mut *((*f).task_thread.done)
                                                        .as_mut_ptr()
                                                        .offset(1 as libc::c_int as isize) as *mut atomic_int,
                                                ) != 0)
                                        {
                                            dav1d_decode_frame_exit(
                                                f,
                                                if error_0 == 1 as libc::c_int {
                                                    -(22 as libc::c_int)
                                                } else if error_0 != 0 {
                                                    -(12 as libc::c_int)
                                                } else {
                                                    0 as libc::c_int
                                                },
                                            );
                                            (*f).n_tile_data = 0 as libc::c_int;
                                            pthread_cond_signal(&mut (*f).task_thread.cond);
                                        }
                                        if !(::core::intrinsics::atomic_load_seqcst(
                                            &mut (*f).task_thread.task_counter as *mut atomic_int,
                                        ) >= 0 as libc::c_int)
                                        {
                                            unreachable!();
                                        }
                                        let fresh36 = &mut (*ttd).cond_signaled as *mut atomic_int;
                                        let fresh37 = 1 as libc::c_int;
                                        if ::core::intrinsics::atomic_or_seqcst(fresh36, fresh37)
                                            == 0
                                        {
                                            pthread_cond_signal(&mut (*ttd).cond);
                                        }
                                        continue 's_18;
                                    }
                                }
                                5 => {
                                    if ::core::intrinsics::atomic_load_seqcst(
                                        &mut (*f).task_thread.error as *mut atomic_int,
                                    ) == 0
                                    {
                                        ((*f).bd_fn.filter_sbrow_deblock_cols)
                                            .expect("non-null function pointer")(f, sby);
                                    }
                                    if ensure_progress(
                                        ttd,
                                        f,
                                        t,
                                        DAV1D_TASK_TYPE_DEBLOCK_ROWS,
                                        &mut (*f).frame_thread.deblock_progress,
                                        &mut (*t).deblock_progress,
                                    ) != 0
                                    {
                                        continue 's_18;
                                    } else {
                                        current_block = 6008637731897673938;
                                        break;
                                    }
                                }
                                6 => {
                                    current_block = 6008637731897673938;
                                    break;
                                }
                                7 => {
                                    current_block = 13240214649988793380;
                                    break;
                                }
                                8 => {
                                    current_block = 14546186248892381816;
                                    break;
                                }
                                9 => {
                                    current_block = 6750403331223000732;
                                    break;
                                }
                                10 => {
                                    current_block = 8542881708851390082;
                                    break;
                                }
                                3 => {
                                    current_block = 7301429464686428633;
                                    break;
                                }
                                _ => {
                                    abort();
                                }
                            }
                        }
                        match current_block {
                            6008637731897673938 => {
                                if ::core::intrinsics::atomic_load_seqcst(
                                    &mut (*f).task_thread.error as *mut atomic_int,
                                ) == 0
                                {
                                    ((*f).bd_fn.filter_sbrow_deblock_rows)
                                        .expect("non-null function pointer")(f, sby);
                                }
                                if (*(*f).frame_hdr)
                                    .loopfilter
                                    .level_y[0 as libc::c_int as usize] != 0
                                    || (*(*f).frame_hdr)
                                        .loopfilter
                                        .level_y[1 as libc::c_int as usize] != 0
                                {
                                    error_0 = ::core::intrinsics::atomic_load_seqcst(
                                        &mut (*f).task_thread.error,
                                    );
                                    ::core::intrinsics::atomic_store_seqcst(
                                        &mut (*f).frame_thread.deblock_progress,
                                        if error_0 != 0 {
                                            2147483647 as libc::c_int - 1 as libc::c_int
                                        } else {
                                            sby + 1 as libc::c_int
                                        },
                                    );
                                    reset_task_cur_async(ttd, (*t).frame_idx, (*c).n_fc);
                                    let fresh38 = &mut (*ttd).cond_signaled as *mut atomic_int;
                                    let fresh39 = 1 as libc::c_int;
                                    if ::core::intrinsics::atomic_or_seqcst(fresh38, fresh39)
                                        == 0
                                    {
                                        pthread_cond_signal(&mut (*ttd).cond);
                                    }
                                } else if (*(*f).seq_hdr).cdef != 0
                                    || (*f).lf.restore_planes != 0
                                {
                                    let fresh40 = &mut *((*f).frame_thread.copy_lpf_progress)
                                        .offset((sby >> 5 as libc::c_int) as isize)
                                        as *mut atomic_uint;
                                    let fresh41 = (1 as libc::c_uint)
                                        << (sby & 31 as libc::c_int);
                                    ::core::intrinsics::atomic_or_seqcst(fresh40, fresh41);
                                    if sby != 0 {
                                        let mut prog_1: libc::c_int = ::core::intrinsics::atomic_load_seqcst(
                                            &mut *((*f).frame_thread.copy_lpf_progress)
                                                .offset(
                                                    (sby - 1 as libc::c_int >> 5 as libc::c_int) as isize,
                                                ) as *mut atomic_uint,
                                        ) as libc::c_int;
                                        if !prog_1 as libc::c_uint
                                            & (1 as libc::c_uint)
                                                << (sby - 1 as libc::c_int & 31 as libc::c_int) != 0
                                        {
                                            (*t).type_0 = DAV1D_TASK_TYPE_CDEF;
                                            (*t).deblock_progress = 0 as libc::c_int;
                                            (*t).recon_progress = (*t).deblock_progress;
                                            add_pending(f, t);
                                            pthread_mutex_lock(&mut (*ttd).lock);
                                            continue;
                                        }
                                    }
                                }
                                current_block = 13240214649988793380;
                            }
                            _ => {}
                        }
                        match current_block {
                            13240214649988793380 => {
                                if (*(*f).seq_hdr).cdef != 0 {
                                    if ::core::intrinsics::atomic_load_seqcst(
                                        &mut (*f).task_thread.error as *mut atomic_int,
                                    ) == 0
                                    {
                                        ((*f).bd_fn.filter_sbrow_cdef)
                                            .expect("non-null function pointer")(tc, sby);
                                    }
                                    reset_task_cur_async(ttd, (*t).frame_idx, (*c).n_fc);
                                    let fresh42 = &mut (*ttd).cond_signaled as *mut atomic_int;
                                    let fresh43 = 1 as libc::c_int;
                                    if ::core::intrinsics::atomic_or_seqcst(fresh42, fresh43)
                                        == 0
                                    {
                                        pthread_cond_signal(&mut (*ttd).cond);
                                    }
                                }
                                current_block = 14546186248892381816;
                            }
                            _ => {}
                        }
                        match current_block {
                            14546186248892381816 => {
                                if (*(*f).frame_hdr).width[0 as libc::c_int as usize]
                                    != (*(*f).frame_hdr).width[1 as libc::c_int as usize]
                                {
                                    if ::core::intrinsics::atomic_load_seqcst(
                                        &mut (*f).task_thread.error as *mut atomic_int,
                                    ) == 0
                                    {
                                        ((*f).bd_fn.filter_sbrow_resize)
                                            .expect("non-null function pointer")(f, sby);
                                    }
                                }
                                current_block = 6750403331223000732;
                            }
                            _ => {}
                        }
                        match current_block {
                            6750403331223000732 => {
                                if ::core::intrinsics::atomic_load_seqcst(
                                    &mut (*f).task_thread.error as *mut atomic_int,
                                ) == 0 && (*f).lf.restore_planes != 0
                                {
                                    ((*f).bd_fn.filter_sbrow_lr)
                                        .expect("non-null function pointer")(f, sby);
                                }
                                current_block = 8542881708851390082;
                            }
                            _ => {}
                        }
                        match current_block {
                            8542881708851390082 => {}
                            _ => {}
                        }
                        let uses_2pass_0: libc::c_int = ((*c).n_fc
                            > 1 as libc::c_int as libc::c_uint) as libc::c_int;
                        let sbh: libc::c_int = (*f).sbh;
                        let sbsz: libc::c_int = (*f).sb_step * 4 as libc::c_int;
                        if (*t).type_0 as libc::c_uint
                            == DAV1D_TASK_TYPE_ENTROPY_PROGRESS as libc::c_int
                                as libc::c_uint
                        {
                            error_0 = ::core::intrinsics::atomic_load_seqcst(
                                &mut (*f).task_thread.error,
                            );
                            let y: libc::c_uint = if sby + 1 as libc::c_int == sbh {
                                (2147483647 as libc::c_int as libc::c_uint)
                                    .wrapping_mul(2 as libc::c_uint)
                                    .wrapping_add(1 as libc::c_uint)
                            } else {
                                ((sby + 1 as libc::c_int) as libc::c_uint)
                                    .wrapping_mul(sbsz as libc::c_uint)
                            };
                            if !((*c).n_fc > 1 as libc::c_int as libc::c_uint) {
                                unreachable!();
                            }
                            if !((*f).sr_cur.p.data[0 as libc::c_int as usize]).is_null()
                            {
                                ::core::intrinsics::atomic_store_seqcst(
                                    &mut *((*f).sr_cur.progress)
                                        .offset(0 as libc::c_int as isize) as *mut atomic_uint,
                                    if error_0 != 0 {
                                        (2147483647 as libc::c_int as libc::c_uint)
                                            .wrapping_mul(2 as libc::c_uint)
                                            .wrapping_add(1 as libc::c_uint)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    } else {
                                        y
                                    },
                                );
                            }
                            ::core::intrinsics::atomic_store_seqcst(
                                &mut (*f).frame_thread.entropy_progress,
                                if error_0 != 0 {
                                    2147483647 as libc::c_int - 1 as libc::c_int
                                } else {
                                    sby + 1 as libc::c_int
                                },
                            );
                            if sby + 1 as libc::c_int == sbh {
                                ::core::intrinsics::atomic_store_seqcst(
                                    &mut *((*f).task_thread.done)
                                        .as_mut_ptr()
                                        .offset(1 as libc::c_int as isize) as *mut atomic_int,
                                    1 as libc::c_int,
                                );
                            }
                            pthread_mutex_lock(&mut (*ttd).lock);
                            let fresh44 = &mut (*f).task_thread.task_counter;
                            let fresh45 = 1 as libc::c_int;
                            let num_tasks: libc::c_int = ::core::intrinsics::atomic_xsub_seqcst(
                                fresh44,
                                fresh45,
                            ) - 1 as libc::c_int;
                            if (sby + 1 as libc::c_int) < sbh && num_tasks != 0 {
                                reset_task_cur(c, ttd, (*t).frame_idx);
                                continue;
                            } else {
                                if num_tasks == 0
                                    && ::core::intrinsics::atomic_load_seqcst(
                                        &mut *((*f).task_thread.done)
                                            .as_mut_ptr()
                                            .offset(0 as libc::c_int as isize) as *mut atomic_int,
                                    ) != 0
                                    && ::core::intrinsics::atomic_load_seqcst(
                                        &mut *((*f).task_thread.done)
                                            .as_mut_ptr()
                                            .offset(1 as libc::c_int as isize) as *mut atomic_int,
                                    ) != 0
                                {
                                    dav1d_decode_frame_exit(
                                        f,
                                        if error_0 == 1 as libc::c_int {
                                            -(22 as libc::c_int)
                                        } else if error_0 != 0 {
                                            -(12 as libc::c_int)
                                        } else {
                                            0 as libc::c_int
                                        },
                                    );
                                    (*f).n_tile_data = 0 as libc::c_int;
                                    pthread_cond_signal(&mut (*f).task_thread.cond);
                                }
                                reset_task_cur(c, ttd, (*t).frame_idx);
                                continue;
                            }
                        } else {
                            let fresh46 = &mut *((*f).frame_thread.frame_progress)
                                .offset((sby >> 5 as libc::c_int) as isize)
                                as *mut atomic_uint;
                            let fresh47 = (1 as libc::c_uint)
                                << (sby & 31 as libc::c_int);
                            ::core::intrinsics::atomic_or_seqcst(fresh46, fresh47);
                            pthread_mutex_lock(&mut (*f).task_thread.lock);
                            sby = get_frame_progress(c, f);
                            error_0 = ::core::intrinsics::atomic_load_seqcst(
                                &mut (*f).task_thread.error,
                            );
                            let y_0: libc::c_uint = if sby + 1 as libc::c_int == sbh {
                                (2147483647 as libc::c_int as libc::c_uint)
                                    .wrapping_mul(2 as libc::c_uint)
                                    .wrapping_add(1 as libc::c_uint)
                            } else {
                                ((sby + 1 as libc::c_int) as libc::c_uint)
                                    .wrapping_mul(sbsz as libc::c_uint)
                            };
                            if (*c).n_fc > 1 as libc::c_int as libc::c_uint
                                && !((*f).sr_cur.p.data[0 as libc::c_int as usize])
                                    .is_null()
                            {
                                ::core::intrinsics::atomic_store_seqcst(
                                    &mut *((*f).sr_cur.progress)
                                        .offset(1 as libc::c_int as isize) as *mut atomic_uint,
                                    if error_0 != 0 {
                                        (2147483647 as libc::c_int as libc::c_uint)
                                            .wrapping_mul(2 as libc::c_uint)
                                            .wrapping_add(1 as libc::c_uint)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    } else {
                                        y_0
                                    },
                                );
                            }
                            pthread_mutex_unlock(&mut (*f).task_thread.lock);
                            if sby + 1 as libc::c_int == sbh {
                                ::core::intrinsics::atomic_store_seqcst(
                                    &mut *((*f).task_thread.done)
                                        .as_mut_ptr()
                                        .offset(0 as libc::c_int as isize) as *mut atomic_int,
                                    1 as libc::c_int,
                                );
                            }
                            pthread_mutex_lock(&mut (*ttd).lock);
                            let fresh48 = &mut (*f).task_thread.task_counter;
                            let fresh49 = 1 as libc::c_int;
                            let num_tasks_0: libc::c_int = ::core::intrinsics::atomic_xsub_seqcst(
                                fresh48,
                                fresh49,
                            ) - 1 as libc::c_int;
                            if (sby + 1 as libc::c_int) < sbh && num_tasks_0 != 0 {
                                reset_task_cur(c, ttd, (*t).frame_idx);
                                continue;
                            } else {
                                if num_tasks_0 == 0
                                    && ::core::intrinsics::atomic_load_seqcst(
                                        &mut *((*f).task_thread.done)
                                            .as_mut_ptr()
                                            .offset(0 as libc::c_int as isize) as *mut atomic_int,
                                    ) != 0
                                    && (uses_2pass_0 == 0
                                        || ::core::intrinsics::atomic_load_seqcst(
                                            &mut *((*f).task_thread.done)
                                                .as_mut_ptr()
                                                .offset(1 as libc::c_int as isize) as *mut atomic_int,
                                        ) != 0)
                                {
                                    dav1d_decode_frame_exit(
                                        f,
                                        if error_0 == 1 as libc::c_int {
                                            -(22 as libc::c_int)
                                        } else if error_0 != 0 {
                                            -(12 as libc::c_int)
                                        } else {
                                            0 as libc::c_int
                                        },
                                    );
                                    (*f).n_tile_data = 0 as libc::c_int;
                                    pthread_cond_signal(&mut (*f).task_thread.cond);
                                }
                                reset_task_cur(c, ttd, (*t).frame_idx);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        (*tc).task_thread.flushed = 1 as libc::c_int;
        pthread_cond_signal(&mut (*tc).task_thread.td.cond);
        ::core::intrinsics::atomic_store_seqcst(
            &mut (*ttd).cond_signaled,
            0 as libc::c_int,
        );
        pthread_cond_wait(&mut (*ttd).cond, &mut (*ttd).lock);
        (*tc).task_thread.flushed = 0 as libc::c_int;
        reset_task_cur(
            c,
            ttd,
            (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint),
        );
    }
    pthread_mutex_unlock(&mut (*ttd).lock);
    return 0 as *mut libc::c_void;
}
