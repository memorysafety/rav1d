use ::libc;
extern "C" {
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn dav1d_submit_frame(c: *mut Dav1dContext) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn dav1d_ref_create(size: size_t) -> *mut Dav1dRef;
    fn dav1d_ref_create_using_pool(
        pool: *mut Dav1dMemPool,
        size: size_t,
    ) -> *mut Dav1dRef;
    fn dav1d_ref_dec(ref_0: *mut *mut Dav1dRef);
    fn dav1d_cdf_thread_ref(dst: *mut CdfThreadContext, src: *mut CdfThreadContext);
    fn dav1d_cdf_thread_unref(cdf: *mut CdfThreadContext);
    fn dav1d_data_ref(dst: *mut Dav1dData, src: *const Dav1dData);
    fn dav1d_data_props_copy(dst: *mut Dav1dDataProps, src: *const Dav1dDataProps);
    fn dav1d_data_unref_internal(buf: *mut Dav1dData);
    static dav1d_default_wm_params: Dav1dWarpedMotionParams;
    fn dav1d_thread_picture_ref(
        dst: *mut Dav1dThreadPicture,
        src: *const Dav1dThreadPicture,
    );
    fn dav1d_thread_picture_unref(p: *mut Dav1dThreadPicture);
    fn dav1d_picture_get_event_flags(p: *const Dav1dThreadPicture) -> Dav1dEventFlags;
    fn dav1d_init_get_bits(c: *mut GetBits, data: *const uint8_t, sz: size_t);
    fn dav1d_get_bit(c: *mut GetBits) -> libc::c_uint;
    fn dav1d_get_bits(c: *mut GetBits, n: libc::c_int) -> libc::c_uint;
    fn dav1d_get_sbits(c: *mut GetBits, n: libc::c_int) -> libc::c_int;
    fn dav1d_get_uleb128(c: *mut GetBits) -> libc::c_uint;
    fn dav1d_get_uniform(c: *mut GetBits, max: libc::c_uint) -> libc::c_uint;
    fn dav1d_get_vlc(c: *mut GetBits) -> libc::c_uint;
    fn dav1d_get_bits_subexp(
        c: *mut GetBits,
        ref_0: libc::c_int,
        n: libc::c_uint,
    ) -> libc::c_int;
    fn dav1d_bytealign_get_bits(c: *mut GetBits);
    fn dav1d_log(c: *mut Dav1dContext, format: *const libc::c_char, _: ...);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
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
pub type ptrdiff_t = libc::c_long;
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
pub type atomic_int = libc::c_int;
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
pub type memory_order = libc::c_uint;
pub const memory_order_seq_cst: memory_order = 5;
pub const memory_order_acq_rel: memory_order = 4;
pub const memory_order_release: memory_order = 3;
pub const memory_order_acquire: memory_order = 2;
pub const memory_order_consume: memory_order = 1;
pub const memory_order_relaxed: memory_order = 0;
pub type atomic_uint = libc::c_uint;
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
pub type Dav1dObuType = libc::c_uint;
pub const DAV1D_OBU_PADDING: Dav1dObuType = 15;
pub const DAV1D_OBU_REDUNDANT_FRAME_HDR: Dav1dObuType = 7;
pub const DAV1D_OBU_FRAME: Dav1dObuType = 6;
pub const DAV1D_OBU_METADATA: Dav1dObuType = 5;
pub const DAV1D_OBU_TILE_GRP: Dav1dObuType = 4;
pub const DAV1D_OBU_FRAME_HDR: Dav1dObuType = 3;
pub const DAV1D_OBU_TD: Dav1dObuType = 2;
pub const DAV1D_OBU_SEQ_HDR: Dav1dObuType = 1;
pub type ObuMetaType = libc::c_uint;
pub const OBU_META_TIMECODE: ObuMetaType = 5;
pub const OBU_META_ITUT_T35: ObuMetaType = 4;
pub const OBU_META_SCALABILITY: ObuMetaType = 3;
pub const OBU_META_HDR_MDCV: ObuMetaType = 2;
pub const OBU_META_HDR_CLL: ObuMetaType = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GetBits {
    pub state: uint64_t,
    pub bits_left: libc::c_int,
    pub error: libc::c_int,
    pub ptr: *const uint8_t,
    pub ptr_start: *const uint8_t,
    pub ptr_end: *const uint8_t,
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
unsafe extern "C" fn iclip(
    v: libc::c_int,
    min: libc::c_int,
    max: libc::c_int,
) -> libc::c_int {
    return if v < min { min } else if v > max { max } else { v };
}
#[inline]
unsafe extern "C" fn iclip_u8(v: libc::c_int) -> libc::c_int {
    return iclip(v, 0 as libc::c_int, 255 as libc::c_int);
}
#[inline]
unsafe extern "C" fn ulog2(v: libc::c_uint) -> libc::c_int {
    return 31 as libc::c_int - clz(v);
}
#[inline]
unsafe extern "C" fn clz(mask: libc::c_uint) -> libc::c_int {
    return mask.leading_zeros() as i32;
}
#[inline]
unsafe extern "C" fn dav1d_ref_inc(ref_0: *mut Dav1dRef) {
    ::core::intrinsics::atomic_xadd_relaxed(&mut (*ref_0).ref_cnt, 1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn get_poc_diff(
    order_hint_n_bits: libc::c_int,
    poc0: libc::c_int,
    poc1: libc::c_int,
) -> libc::c_int {
    if order_hint_n_bits == 0 {
        return 0 as libc::c_int;
    }
    let mask: libc::c_int = (1 as libc::c_int) << order_hint_n_bits - 1 as libc::c_int;
    let diff: libc::c_int = poc0 - poc1;
    return (diff & mask - 1 as libc::c_int) - (diff & mask);
}
#[inline]
unsafe extern "C" fn dav1d_get_bits_pos(mut c: *const GetBits) -> libc::c_uint {
    return (((*c).ptr).offset_from((*c).ptr_start) as libc::c_long as libc::c_uint)
        .wrapping_mul(8 as libc::c_int as libc::c_uint)
        .wrapping_sub((*c).bits_left as libc::c_uint);
}
unsafe extern "C" fn parse_seq_hdr(
    c: *mut Dav1dContext,
    gb: *mut GetBits,
    hdr: *mut Dav1dSequenceHeader,
) -> libc::c_int {
    let mut op_idx: libc::c_int = 0;
    let mut spatial_mask: libc::c_uint = 0;
    let mut current_block: u64;
    memset(
        hdr as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Dav1dSequenceHeader>() as libc::c_ulong,
    );
    (*hdr).profile = dav1d_get_bits(gb, 3 as libc::c_int) as libc::c_int;
    if !((*hdr).profile > 2 as libc::c_int) {
        (*hdr).still_picture = dav1d_get_bit(gb) as libc::c_int;
        (*hdr).reduced_still_picture_header = dav1d_get_bit(gb) as libc::c_int;
        if !((*hdr).reduced_still_picture_header != 0 && (*hdr).still_picture == 0) {
            if (*hdr).reduced_still_picture_header != 0 {
                (*hdr).num_operating_points = 1 as libc::c_int;
                (*hdr)
                    .operating_points[0 as libc::c_int as usize]
                    .major_level = dav1d_get_bits(gb, 3 as libc::c_int) as libc::c_int;
                (*hdr)
                    .operating_points[0 as libc::c_int as usize]
                    .minor_level = dav1d_get_bits(gb, 2 as libc::c_int) as libc::c_int;
                (*hdr)
                    .operating_points[0 as libc::c_int as usize]
                    .initial_display_delay = 10 as libc::c_int;
                current_block = 4090602189656566074;
            } else {
                (*hdr).timing_info_present = dav1d_get_bit(gb) as libc::c_int;
                if (*hdr).timing_info_present != 0 {
                    (*hdr)
                        .num_units_in_tick = dav1d_get_bits(gb, 32 as libc::c_int)
                        as libc::c_int;
                    (*hdr)
                        .time_scale = dav1d_get_bits(gb, 32 as libc::c_int)
                        as libc::c_int;
                    (*hdr).equal_picture_interval = dav1d_get_bit(gb) as libc::c_int;
                    if (*hdr).equal_picture_interval != 0 {
                        let num_ticks_per_picture: libc::c_uint = dav1d_get_vlc(gb);
                        if num_ticks_per_picture == 0xffffffff as libc::c_uint {
                            current_block = 17400737960072300055;
                        } else {
                            (*hdr)
                                .num_ticks_per_picture = num_ticks_per_picture
                                .wrapping_add(1 as libc::c_int as libc::c_uint);
                            current_block = 10048703153582371463;
                        }
                    } else {
                        current_block = 10048703153582371463;
                    }
                    match current_block {
                        17400737960072300055 => {}
                        _ => {
                            (*hdr)
                                .decoder_model_info_present = dav1d_get_bit(gb)
                                as libc::c_int;
                            if (*hdr).decoder_model_info_present != 0 {
                                (*hdr)
                                    .encoder_decoder_buffer_delay_length = (dav1d_get_bits(
                                    gb,
                                    5 as libc::c_int,
                                ))
                                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                                    as libc::c_int;
                                (*hdr)
                                    .num_units_in_decoding_tick = dav1d_get_bits(
                                    gb,
                                    32 as libc::c_int,
                                ) as libc::c_int;
                                (*hdr)
                                    .buffer_removal_delay_length = (dav1d_get_bits(
                                    gb,
                                    5 as libc::c_int,
                                ))
                                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                                    as libc::c_int;
                                (*hdr)
                                    .frame_presentation_delay_length = (dav1d_get_bits(
                                    gb,
                                    5 as libc::c_int,
                                ))
                                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                                    as libc::c_int;
                            }
                            current_block = 4808432441040389987;
                        }
                    }
                } else {
                    current_block = 4808432441040389987;
                }
                match current_block {
                    17400737960072300055 => {}
                    _ => {
                        (*hdr)
                            .display_model_info_present = dav1d_get_bit(gb)
                            as libc::c_int;
                        (*hdr)
                            .num_operating_points = (dav1d_get_bits(
                            gb,
                            5 as libc::c_int,
                        ))
                            .wrapping_add(1 as libc::c_int as libc::c_uint)
                            as libc::c_int;
                        let mut i: libc::c_int = 0 as libc::c_int;
                        loop {
                            if !(i < (*hdr).num_operating_points) {
                                current_block = 4090602189656566074;
                                break;
                            }
                            let op: *mut Dav1dSequenceHeaderOperatingPoint = &mut *((*hdr)
                                .operating_points)
                                .as_mut_ptr()
                                .offset(i as isize)
                                as *mut Dav1dSequenceHeaderOperatingPoint;
                            (*op)
                                .idc = dav1d_get_bits(gb, 12 as libc::c_int) as libc::c_int;
                            if (*op).idc != 0
                                && ((*op).idc & 0xff as libc::c_int == 0
                                    || (*op).idc & 0xf00 as libc::c_int == 0)
                            {
                                current_block = 17400737960072300055;
                                break;
                            }
                            (*op)
                                .major_level = (2 as libc::c_int as libc::c_uint)
                                .wrapping_add(dav1d_get_bits(gb, 3 as libc::c_int))
                                as libc::c_int;
                            (*op)
                                .minor_level = dav1d_get_bits(gb, 2 as libc::c_int)
                                as libc::c_int;
                            if (*op).major_level > 3 as libc::c_int {
                                (*op).tier = dav1d_get_bit(gb) as libc::c_int;
                            }
                            if (*hdr).decoder_model_info_present != 0 {
                                (*op)
                                    .decoder_model_param_present = dav1d_get_bit(gb)
                                    as libc::c_int;
                                if (*op).decoder_model_param_present != 0 {
                                    let opi: *mut Dav1dSequenceHeaderOperatingParameterInfo = &mut *((*hdr)
                                        .operating_parameter_info)
                                        .as_mut_ptr()
                                        .offset(i as isize)
                                        as *mut Dav1dSequenceHeaderOperatingParameterInfo;
                                    (*opi)
                                        .decoder_buffer_delay = dav1d_get_bits(
                                        gb,
                                        (*hdr).encoder_decoder_buffer_delay_length,
                                    ) as libc::c_int;
                                    (*opi)
                                        .encoder_buffer_delay = dav1d_get_bits(
                                        gb,
                                        (*hdr).encoder_decoder_buffer_delay_length,
                                    ) as libc::c_int;
                                    (*opi).low_delay_mode = dav1d_get_bit(gb) as libc::c_int;
                                }
                            }
                            if (*hdr).display_model_info_present != 0 {
                                (*op)
                                    .display_model_param_present = dav1d_get_bit(gb)
                                    as libc::c_int;
                            }
                            (*op)
                                .initial_display_delay = (if (*op)
                                .display_model_param_present != 0
                            {
                                (dav1d_get_bits(gb, 4 as libc::c_int))
                                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                            } else {
                                10 as libc::c_int as libc::c_uint
                            }) as libc::c_int;
                            i += 1;
                        }
                    }
                }
            }
            match current_block {
                17400737960072300055 => {}
                _ => {
                    op_idx = if (*c).operating_point < (*hdr).num_operating_points {
                        (*c).operating_point
                    } else {
                        0 as libc::c_int
                    };
                    (*c)
                        .operating_point_idc = (*hdr)
                        .operating_points[op_idx as usize]
                        .idc as libc::c_uint;
                    spatial_mask = (*c).operating_point_idc >> 8 as libc::c_int;
                    (*c)
                        .max_spatial_id = if spatial_mask != 0 {
                        ulog2(spatial_mask)
                    } else {
                        0 as libc::c_int
                    };
                    (*hdr)
                        .width_n_bits = (dav1d_get_bits(gb, 4 as libc::c_int))
                        .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
                    (*hdr)
                        .height_n_bits = (dav1d_get_bits(gb, 4 as libc::c_int))
                        .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
                    (*hdr)
                        .max_width = (dav1d_get_bits(gb, (*hdr).width_n_bits))
                        .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
                    (*hdr)
                        .max_height = (dav1d_get_bits(gb, (*hdr).height_n_bits))
                        .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
                    if (*hdr).reduced_still_picture_header == 0 {
                        (*hdr)
                            .frame_id_numbers_present = dav1d_get_bit(gb) as libc::c_int;
                        if (*hdr).frame_id_numbers_present != 0 {
                            (*hdr)
                                .delta_frame_id_n_bits = (dav1d_get_bits(
                                gb,
                                4 as libc::c_int,
                            ))
                                .wrapping_add(2 as libc::c_int as libc::c_uint)
                                as libc::c_int;
                            (*hdr)
                                .frame_id_n_bits = (dav1d_get_bits(gb, 3 as libc::c_int))
                                .wrapping_add((*hdr).delta_frame_id_n_bits as libc::c_uint)
                                .wrapping_add(1 as libc::c_int as libc::c_uint)
                                as libc::c_int;
                        }
                    }
                    (*hdr).sb128 = dav1d_get_bit(gb) as libc::c_int;
                    (*hdr).filter_intra = dav1d_get_bit(gb) as libc::c_int;
                    (*hdr).intra_edge_filter = dav1d_get_bit(gb) as libc::c_int;
                    if (*hdr).reduced_still_picture_header != 0 {
                        (*hdr).screen_content_tools = DAV1D_ADAPTIVE;
                        (*hdr).force_integer_mv = DAV1D_ADAPTIVE;
                    } else {
                        (*hdr).inter_intra = dav1d_get_bit(gb) as libc::c_int;
                        (*hdr).masked_compound = dav1d_get_bit(gb) as libc::c_int;
                        (*hdr).warped_motion = dav1d_get_bit(gb) as libc::c_int;
                        (*hdr).dual_filter = dav1d_get_bit(gb) as libc::c_int;
                        (*hdr).order_hint = dav1d_get_bit(gb) as libc::c_int;
                        if (*hdr).order_hint != 0 {
                            (*hdr).jnt_comp = dav1d_get_bit(gb) as libc::c_int;
                            (*hdr).ref_frame_mvs = dav1d_get_bit(gb) as libc::c_int;
                        }
                        (*hdr)
                            .screen_content_tools = (if dav1d_get_bit(gb) != 0 {
                            DAV1D_ADAPTIVE as libc::c_int as libc::c_uint
                        } else {
                            dav1d_get_bit(gb)
                        }) as Dav1dAdaptiveBoolean;
                        (*hdr)
                            .force_integer_mv = (if (*hdr).screen_content_tools
                            as libc::c_uint != 0
                        {
                            if dav1d_get_bit(gb) != 0 {
                                DAV1D_ADAPTIVE as libc::c_int as libc::c_uint
                            } else {
                                dav1d_get_bit(gb)
                            }
                        } else {
                            2 as libc::c_int as libc::c_uint
                        }) as Dav1dAdaptiveBoolean;
                        if (*hdr).order_hint != 0 {
                            (*hdr)
                                .order_hint_n_bits = (dav1d_get_bits(gb, 3 as libc::c_int))
                                .wrapping_add(1 as libc::c_int as libc::c_uint)
                                as libc::c_int;
                        }
                    }
                    (*hdr).super_res = dav1d_get_bit(gb) as libc::c_int;
                    (*hdr).cdef = dav1d_get_bit(gb) as libc::c_int;
                    (*hdr).restoration = dav1d_get_bit(gb) as libc::c_int;
                    (*hdr).hbd = dav1d_get_bit(gb) as libc::c_int;
                    if (*hdr).profile == 2 as libc::c_int && (*hdr).hbd != 0 {
                        (*hdr)
                            .hbd = ((*hdr).hbd as libc::c_uint)
                            .wrapping_add(dav1d_get_bit(gb)) as libc::c_int
                            as libc::c_int;
                    }
                    if (*hdr).profile != 1 as libc::c_int {
                        (*hdr).monochrome = dav1d_get_bit(gb) as libc::c_int;
                    }
                    (*hdr).color_description_present = dav1d_get_bit(gb) as libc::c_int;
                    if (*hdr).color_description_present != 0 {
                        (*hdr)
                            .pri = dav1d_get_bits(gb, 8 as libc::c_int)
                            as Dav1dColorPrimaries;
                        (*hdr)
                            .trc = dav1d_get_bits(gb, 8 as libc::c_int)
                            as Dav1dTransferCharacteristics;
                        (*hdr)
                            .mtrx = dav1d_get_bits(gb, 8 as libc::c_int)
                            as Dav1dMatrixCoefficients;
                    } else {
                        (*hdr).pri = DAV1D_COLOR_PRI_UNKNOWN;
                        (*hdr).trc = DAV1D_TRC_UNKNOWN;
                        (*hdr).mtrx = DAV1D_MC_UNKNOWN;
                    }
                    if (*hdr).monochrome != 0 {
                        (*hdr).color_range = dav1d_get_bit(gb) as libc::c_int;
                        (*hdr).layout = DAV1D_PIXEL_LAYOUT_I400;
                        (*hdr).ss_ver = 1 as libc::c_int;
                        (*hdr).ss_hor = (*hdr).ss_ver;
                        (*hdr).chr = DAV1D_CHR_UNKNOWN;
                        current_block = 14141370668937312244;
                    } else if (*hdr).pri as libc::c_uint
                        == DAV1D_COLOR_PRI_BT709 as libc::c_int as libc::c_uint
                        && (*hdr).trc as libc::c_uint
                            == DAV1D_TRC_SRGB as libc::c_int as libc::c_uint
                        && (*hdr).mtrx as libc::c_uint
                            == DAV1D_MC_IDENTITY as libc::c_int as libc::c_uint
                    {
                        (*hdr).layout = DAV1D_PIXEL_LAYOUT_I444;
                        (*hdr).color_range = 1 as libc::c_int;
                        if (*hdr).profile != 1 as libc::c_int
                            && !((*hdr).profile == 2 as libc::c_int
                                && (*hdr).hbd == 2 as libc::c_int)
                        {
                            current_block = 17400737960072300055;
                        } else {
                            current_block = 14141370668937312244;
                        }
                    } else {
                        (*hdr).color_range = dav1d_get_bit(gb) as libc::c_int;
                        match (*hdr).profile {
                            0 => {
                                (*hdr).layout = DAV1D_PIXEL_LAYOUT_I420;
                                (*hdr).ss_ver = 1 as libc::c_int;
                                (*hdr).ss_hor = (*hdr).ss_ver;
                            }
                            1 => {
                                (*hdr).layout = DAV1D_PIXEL_LAYOUT_I444;
                            }
                            2 => {
                                if (*hdr).hbd == 2 as libc::c_int {
                                    (*hdr).ss_hor = dav1d_get_bit(gb) as libc::c_int;
                                    if (*hdr).ss_hor != 0 {
                                        (*hdr).ss_ver = dav1d_get_bit(gb) as libc::c_int;
                                    }
                                } else {
                                    (*hdr).ss_hor = 1 as libc::c_int;
                                }
                                (*hdr)
                                    .layout = (if (*hdr).ss_hor != 0 {
                                    if (*hdr).ss_ver != 0 {
                                        DAV1D_PIXEL_LAYOUT_I420 as libc::c_int
                                    } else {
                                        DAV1D_PIXEL_LAYOUT_I422 as libc::c_int
                                    }
                                } else {
                                    DAV1D_PIXEL_LAYOUT_I444 as libc::c_int
                                }) as Dav1dPixelLayout;
                            }
                            _ => {}
                        }
                        (*hdr)
                            .chr = (if (*hdr).ss_hor & (*hdr).ss_ver != 0 {
                            dav1d_get_bits(gb, 2 as libc::c_int)
                        } else {
                            DAV1D_CHR_UNKNOWN as libc::c_int as libc::c_uint
                        }) as Dav1dChromaSamplePosition;
                        current_block = 14141370668937312244;
                    }
                    match current_block {
                        17400737960072300055 => {}
                        _ => {
                            if !((*c).strict_std_compliance != 0
                                && (*hdr).mtrx as libc::c_uint
                                    == DAV1D_MC_IDENTITY as libc::c_int as libc::c_uint
                                && (*hdr).layout as libc::c_uint
                                    != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint)
                            {
                                if (*hdr).monochrome == 0 {
                                    (*hdr)
                                        .separate_uv_delta_q = dav1d_get_bit(gb) as libc::c_int;
                                }
                                (*hdr)
                                    .film_grain_present = dav1d_get_bit(gb) as libc::c_int;
                                dav1d_get_bit(gb);
                                return 0 as libc::c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    dav1d_log(
        c,
        b"Error parsing sequence header\n\0" as *const u8 as *const libc::c_char,
    );
    return -(22 as libc::c_int);
}
unsafe extern "C" fn read_frame_size(
    c: *mut Dav1dContext,
    gb: *mut GetBits,
    use_ref: libc::c_int,
) -> libc::c_int {
    let seqhdr: *const Dav1dSequenceHeader = (*c).seq_hdr;
    let hdr: *mut Dav1dFrameHeader = (*c).frame_hdr;
    if use_ref != 0 {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 7 as libc::c_int {
            if dav1d_get_bit(gb) != 0 {
                let ref_0: *const Dav1dThreadPicture = &mut (*((*c).refs)
                    .as_mut_ptr()
                    .offset(
                        *((*(*c).frame_hdr).refidx).as_mut_ptr().offset(i as isize)
                            as isize,
                    ))
                    .p;
                if ((*ref_0).p.frame_hdr).is_null() {
                    return -(1 as libc::c_int);
                }
                (*hdr)
                    .width[1 as libc::c_int
                    as usize] = (*(*ref_0).p.frame_hdr).width[1 as libc::c_int as usize];
                (*hdr).height = (*(*ref_0).p.frame_hdr).height;
                (*hdr).render_width = (*(*ref_0).p.frame_hdr).render_width;
                (*hdr).render_height = (*(*ref_0).p.frame_hdr).render_height;
                (*hdr)
                    .super_res
                    .enabled = ((*seqhdr).super_res != 0 && dav1d_get_bit(gb) != 0)
                    as libc::c_int;
                if (*hdr).super_res.enabled != 0 {
                    (*hdr)
                        .super_res
                        .width_scale_denominator = (9 as libc::c_int as libc::c_uint)
                        .wrapping_add(dav1d_get_bits(gb, 3 as libc::c_int))
                        as libc::c_int;
                    let d: libc::c_int = (*hdr).super_res.width_scale_denominator;
                    (*hdr)
                        .width[0 as libc::c_int
                        as usize] = imax(
                        ((*hdr).width[1 as libc::c_int as usize] * 8 as libc::c_int
                            + (d >> 1 as libc::c_int)) / d,
                        imin(16 as libc::c_int, (*hdr).width[1 as libc::c_int as usize]),
                    );
                } else {
                    (*hdr).super_res.width_scale_denominator = 8 as libc::c_int;
                    (*hdr)
                        .width[0 as libc::c_int
                        as usize] = (*hdr).width[1 as libc::c_int as usize];
                }
                return 0 as libc::c_int;
            }
            i += 1;
        }
    }
    if (*hdr).frame_size_override != 0 {
        (*hdr)
            .width[1 as libc::c_int
            as usize] = (dav1d_get_bits(gb, (*seqhdr).width_n_bits))
            .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
        (*hdr)
            .height = (dav1d_get_bits(gb, (*seqhdr).height_n_bits))
            .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
    } else {
        (*hdr).width[1 as libc::c_int as usize] = (*seqhdr).max_width;
        (*hdr).height = (*seqhdr).max_height;
    }
    (*hdr)
        .super_res
        .enabled = ((*seqhdr).super_res != 0 && dav1d_get_bit(gb) != 0) as libc::c_int;
    if (*hdr).super_res.enabled != 0 {
        (*hdr)
            .super_res
            .width_scale_denominator = (9 as libc::c_int as libc::c_uint)
            .wrapping_add(dav1d_get_bits(gb, 3 as libc::c_int)) as libc::c_int;
        let d_0: libc::c_int = (*hdr).super_res.width_scale_denominator;
        (*hdr)
            .width[0 as libc::c_int
            as usize] = imax(
            ((*hdr).width[1 as libc::c_int as usize] * 8 as libc::c_int
                + (d_0 >> 1 as libc::c_int)) / d_0,
            imin(16 as libc::c_int, (*hdr).width[1 as libc::c_int as usize]),
        );
    } else {
        (*hdr).super_res.width_scale_denominator = 8 as libc::c_int;
        (*hdr)
            .width[0 as libc::c_int as usize] = (*hdr).width[1 as libc::c_int as usize];
    }
    (*hdr).have_render_size = dav1d_get_bit(gb) as libc::c_int;
    if (*hdr).have_render_size != 0 {
        (*hdr)
            .render_width = (dav1d_get_bits(gb, 16 as libc::c_int))
            .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
        (*hdr)
            .render_height = (dav1d_get_bits(gb, 16 as libc::c_int))
            .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
    } else {
        (*hdr).render_width = (*hdr).width[1 as libc::c_int as usize];
        (*hdr).render_height = (*hdr).height;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn tile_log2(sz: libc::c_int, tgt: libc::c_int) -> libc::c_int {
    let mut k: libc::c_int = 0;
    k = 0 as libc::c_int;
    while sz << k < tgt {
        k += 1;
    }
    return k;
}
static mut default_mode_ref_deltas: Dav1dLoopfilterModeRefDeltas = {
    let mut init = Dav1dLoopfilterModeRefDeltas {
        mode_delta: [0 as libc::c_int, 0 as libc::c_int],
        ref_delta: [
            1 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            -(1 as libc::c_int),
            0 as libc::c_int,
            -(1 as libc::c_int),
            -(1 as libc::c_int),
        ],
    };
    init
};
unsafe extern "C" fn parse_frame_hdr(
    c: *mut Dav1dContext,
    gb: *mut GetBits,
) -> libc::c_int {
    let mut sbsz_min1: libc::c_int = 0;
    let mut sbsz_log2: libc::c_int = 0;
    let mut sbw: libc::c_int = 0;
    let mut sbh: libc::c_int = 0;
    let mut max_tile_width_sb: libc::c_int = 0;
    let mut max_tile_area_sb: libc::c_int = 0;
    let mut min_log2_tiles: libc::c_int = 0;
    let mut delta_lossless: libc::c_int = 0;
    let mut current_block: u64;
    let seqhdr: *const Dav1dSequenceHeader = (*c).seq_hdr;
    let hdr: *mut Dav1dFrameHeader = (*c).frame_hdr;
    (*hdr)
        .show_existing_frame = ((*seqhdr).reduced_still_picture_header == 0
        && dav1d_get_bit(gb) != 0) as libc::c_int;
    if (*hdr).show_existing_frame != 0 {
        (*hdr).existing_frame_idx = dav1d_get_bits(gb, 3 as libc::c_int) as libc::c_int;
        if (*seqhdr).decoder_model_info_present != 0
            && (*seqhdr).equal_picture_interval == 0
        {
            (*hdr)
                .frame_presentation_delay = dav1d_get_bits(
                gb,
                (*seqhdr).frame_presentation_delay_length,
            ) as libc::c_int;
        }
        if (*seqhdr).frame_id_numbers_present != 0 {
            (*hdr)
                .frame_id = dav1d_get_bits(gb, (*seqhdr).frame_id_n_bits) as libc::c_int;
            let ref_frame_hdr: *mut Dav1dFrameHeader = (*c)
                .refs[(*hdr).existing_frame_idx as usize]
                .p
                .p
                .frame_hdr;
            if ref_frame_hdr.is_null() || (*ref_frame_hdr).frame_id != (*hdr).frame_id {
                current_block = 13312636450699654424;
            } else {
                current_block = 7351195479953500246;
            }
        } else {
            current_block = 7351195479953500246;
        }
        match current_block {
            13312636450699654424 => {}
            _ => return 0 as libc::c_int,
        }
    } else {
        (*hdr)
            .frame_type = (if (*seqhdr).reduced_still_picture_header != 0 {
            DAV1D_FRAME_TYPE_KEY as libc::c_int as libc::c_uint
        } else {
            dav1d_get_bits(gb, 2 as libc::c_int)
        }) as Dav1dFrameType;
        (*hdr)
            .show_frame = ((*seqhdr).reduced_still_picture_header != 0
            || dav1d_get_bit(gb) != 0) as libc::c_int;
        if (*hdr).show_frame != 0 {
            if (*seqhdr).decoder_model_info_present != 0
                && (*seqhdr).equal_picture_interval == 0
            {
                (*hdr)
                    .frame_presentation_delay = dav1d_get_bits(
                    gb,
                    (*seqhdr).frame_presentation_delay_length,
                ) as libc::c_int;
            }
            (*hdr)
                .showable_frame = ((*hdr).frame_type as libc::c_uint
                != DAV1D_FRAME_TYPE_KEY as libc::c_int as libc::c_uint) as libc::c_int;
        } else {
            (*hdr).showable_frame = dav1d_get_bit(gb) as libc::c_int;
        }
        (*hdr)
            .error_resilient_mode = ((*hdr).frame_type as libc::c_uint
            == DAV1D_FRAME_TYPE_KEY as libc::c_int as libc::c_uint
            && (*hdr).show_frame != 0
            || (*hdr).frame_type as libc::c_uint
                == DAV1D_FRAME_TYPE_SWITCH as libc::c_int as libc::c_uint
            || (*seqhdr).reduced_still_picture_header != 0 || dav1d_get_bit(gb) != 0)
            as libc::c_int;
        (*hdr).disable_cdf_update = dav1d_get_bit(gb) as libc::c_int;
        (*hdr)
            .allow_screen_content_tools = (if (*seqhdr).screen_content_tools
            as libc::c_uint == DAV1D_ADAPTIVE as libc::c_int as libc::c_uint
        {
            dav1d_get_bit(gb)
        } else {
            (*seqhdr).screen_content_tools as libc::c_uint
        }) as libc::c_int;
        if (*hdr).allow_screen_content_tools != 0 {
            (*hdr)
                .force_integer_mv = (if (*seqhdr).force_integer_mv as libc::c_uint
                == DAV1D_ADAPTIVE as libc::c_int as libc::c_uint
            {
                dav1d_get_bit(gb)
            } else {
                (*seqhdr).force_integer_mv as libc::c_uint
            }) as libc::c_int;
        } else {
            (*hdr).force_integer_mv = 0 as libc::c_int;
        }
        if (*hdr).frame_type as libc::c_uint & 1 as libc::c_int as libc::c_uint == 0 {
            (*hdr).force_integer_mv = 1 as libc::c_int;
        }
        if (*seqhdr).frame_id_numbers_present != 0 {
            (*hdr)
                .frame_id = dav1d_get_bits(gb, (*seqhdr).frame_id_n_bits) as libc::c_int;
        }
        (*hdr)
            .frame_size_override = (if (*seqhdr).reduced_still_picture_header != 0 {
            0 as libc::c_int as libc::c_uint
        } else if (*hdr).frame_type as libc::c_uint
            == DAV1D_FRAME_TYPE_SWITCH as libc::c_int as libc::c_uint
        {
            1 as libc::c_int as libc::c_uint
        } else {
            dav1d_get_bit(gb)
        }) as libc::c_int;
        (*hdr)
            .frame_offset = (if (*seqhdr).order_hint != 0 {
            dav1d_get_bits(gb, (*seqhdr).order_hint_n_bits)
        } else {
            0 as libc::c_int as libc::c_uint
        }) as libc::c_int;
        (*hdr)
            .primary_ref_frame = (if (*hdr).error_resilient_mode == 0
            && (*hdr).frame_type as libc::c_uint & 1 as libc::c_int as libc::c_uint != 0
        {
            dav1d_get_bits(gb, 3 as libc::c_int)
        } else {
            7 as libc::c_int as libc::c_uint
        }) as libc::c_int;
        if (*seqhdr).decoder_model_info_present != 0 {
            (*hdr).buffer_removal_time_present = dav1d_get_bit(gb) as libc::c_int;
            if (*hdr).buffer_removal_time_present != 0 {
                let mut i: libc::c_int = 0 as libc::c_int;
                while i < (*(*c).seq_hdr).num_operating_points {
                    let seqop: *const Dav1dSequenceHeaderOperatingPoint = &*((*seqhdr)
                        .operating_points)
                        .as_ptr()
                        .offset(i as isize) as *const Dav1dSequenceHeaderOperatingPoint;
                    let op: *mut Dav1dFrameHeaderOperatingPoint = &mut *((*hdr)
                        .operating_points)
                        .as_mut_ptr()
                        .offset(i as isize) as *mut Dav1dFrameHeaderOperatingPoint;
                    if (*seqop).decoder_model_param_present != 0 {
                        let mut in_temporal_layer: libc::c_int = (*seqop).idc
                            >> (*hdr).temporal_id & 1 as libc::c_int;
                        let mut in_spatial_layer: libc::c_int = (*seqop).idc
                            >> (*hdr).spatial_id + 8 as libc::c_int & 1 as libc::c_int;
                        if (*seqop).idc == 0
                            || in_temporal_layer != 0 && in_spatial_layer != 0
                        {
                            (*op)
                                .buffer_removal_time = dav1d_get_bits(
                                gb,
                                (*seqhdr).buffer_removal_delay_length,
                            ) as libc::c_int;
                        }
                    }
                    i += 1;
                }
            }
        }
        if (*hdr).frame_type as libc::c_uint & 1 as libc::c_int as libc::c_uint == 0 {
            (*hdr)
                .refresh_frame_flags = (if (*hdr).frame_type as libc::c_uint
                == DAV1D_FRAME_TYPE_KEY as libc::c_int as libc::c_uint
                && (*hdr).show_frame != 0
            {
                0xff as libc::c_int as libc::c_uint
            } else {
                dav1d_get_bits(gb, 8 as libc::c_int)
            }) as libc::c_int;
            if (*hdr).refresh_frame_flags != 0xff as libc::c_int
                && (*hdr).error_resilient_mode != 0 && (*seqhdr).order_hint != 0
            {
                let mut i_0: libc::c_int = 0 as libc::c_int;
                while i_0 < 8 as libc::c_int {
                    dav1d_get_bits(gb, (*seqhdr).order_hint_n_bits);
                    i_0 += 1;
                }
            }
            if (*c).strict_std_compliance != 0
                && (*hdr).frame_type as libc::c_uint
                    == DAV1D_FRAME_TYPE_INTRA as libc::c_int as libc::c_uint
                && (*hdr).refresh_frame_flags == 0xff as libc::c_int
            {
                current_block = 13312636450699654424;
            } else if read_frame_size(c, gb, 0 as libc::c_int) < 0 as libc::c_int {
                current_block = 13312636450699654424;
            } else {
                (*hdr)
                    .allow_intrabc = ((*hdr).allow_screen_content_tools != 0
                    && (*hdr).super_res.enabled == 0 && dav1d_get_bit(gb) != 0)
                    as libc::c_int;
                (*hdr).use_ref_frame_mvs = 0 as libc::c_int;
                current_block = 16314074004867283505;
            }
        } else {
            (*hdr).allow_intrabc = 0 as libc::c_int;
            (*hdr)
                .refresh_frame_flags = (if (*hdr).frame_type as libc::c_uint
                == DAV1D_FRAME_TYPE_SWITCH as libc::c_int as libc::c_uint
            {
                0xff as libc::c_int as libc::c_uint
            } else {
                dav1d_get_bits(gb, 8 as libc::c_int)
            }) as libc::c_int;
            if (*hdr).error_resilient_mode != 0 && (*seqhdr).order_hint != 0 {
                let mut i_1: libc::c_int = 0 as libc::c_int;
                while i_1 < 8 as libc::c_int {
                    dav1d_get_bits(gb, (*seqhdr).order_hint_n_bits);
                    i_1 += 1;
                }
            }
            (*hdr)
                .frame_ref_short_signaling = ((*seqhdr).order_hint != 0
                && dav1d_get_bit(gb) != 0) as libc::c_int;
            if (*hdr).frame_ref_short_signaling != 0 {
                (*hdr)
                    .refidx[0 as libc::c_int
                    as usize] = dav1d_get_bits(gb, 3 as libc::c_int) as libc::c_int;
                (*hdr).refidx[2 as libc::c_int as usize] = -(1 as libc::c_int);
                (*hdr)
                    .refidx[1 as libc::c_int
                    as usize] = (*hdr).refidx[2 as libc::c_int as usize];
                (*hdr)
                    .refidx[3 as libc::c_int
                    as usize] = dav1d_get_bits(gb, 3 as libc::c_int) as libc::c_int;
                (*hdr).refidx[6 as libc::c_int as usize] = -(1 as libc::c_int);
                (*hdr)
                    .refidx[5 as libc::c_int
                    as usize] = (*hdr).refidx[6 as libc::c_int as usize];
                (*hdr)
                    .refidx[4 as libc::c_int
                    as usize] = (*hdr).refidx[5 as libc::c_int as usize];
                let mut shifted_frame_offset: [libc::c_int; 8] = [0; 8];
                let current_frame_offset: libc::c_int = (1 as libc::c_int)
                    << (*seqhdr).order_hint_n_bits - 1 as libc::c_int;
                let mut i_2: libc::c_int = 0 as libc::c_int;
                loop {
                    if !(i_2 < 8 as libc::c_int) {
                        current_block = 5159818223158340697;
                        break;
                    }
                    if ((*c).refs[i_2 as usize].p.p.frame_hdr).is_null() {
                        current_block = 13312636450699654424;
                        break;
                    }
                    shifted_frame_offset[i_2
                        as usize] = current_frame_offset
                        + get_poc_diff(
                            (*seqhdr).order_hint_n_bits,
                            (*(*c).refs[i_2 as usize].p.p.frame_hdr).frame_offset,
                            (*hdr).frame_offset,
                        );
                    i_2 += 1;
                }
                match current_block {
                    13312636450699654424 => {}
                    _ => {
                        let mut used_frame: [libc::c_int; 8] = [
                            0 as libc::c_int,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                        ];
                        used_frame[(*hdr).refidx[0 as libc::c_int as usize]
                            as usize] = 1 as libc::c_int;
                        used_frame[(*hdr).refidx[3 as libc::c_int as usize]
                            as usize] = 1 as libc::c_int;
                        let mut latest_frame_offset: libc::c_int = -(1 as libc::c_int);
                        let mut i_3: libc::c_int = 0 as libc::c_int;
                        while i_3 < 8 as libc::c_int {
                            let hint: libc::c_int = shifted_frame_offset[i_3 as usize];
                            if used_frame[i_3 as usize] == 0
                                && hint >= current_frame_offset
                                && hint >= latest_frame_offset
                            {
                                (*hdr).refidx[6 as libc::c_int as usize] = i_3;
                                latest_frame_offset = hint;
                            }
                            i_3 += 1;
                        }
                        if latest_frame_offset != -(1 as libc::c_int) {
                            used_frame[(*hdr).refidx[6 as libc::c_int as usize]
                                as usize] = 1 as libc::c_int;
                        }
                        let mut earliest_frame_offset: libc::c_int = 2147483647
                            as libc::c_int;
                        let mut i_4: libc::c_int = 0 as libc::c_int;
                        while i_4 < 8 as libc::c_int {
                            let hint_0: libc::c_int = shifted_frame_offset[i_4 as usize];
                            if used_frame[i_4 as usize] == 0
                                && hint_0 >= current_frame_offset
                                && hint_0 < earliest_frame_offset
                            {
                                (*hdr).refidx[4 as libc::c_int as usize] = i_4;
                                earliest_frame_offset = hint_0;
                            }
                            i_4 += 1;
                        }
                        if earliest_frame_offset != 2147483647 as libc::c_int {
                            used_frame[(*hdr).refidx[4 as libc::c_int as usize]
                                as usize] = 1 as libc::c_int;
                        }
                        earliest_frame_offset = 2147483647 as libc::c_int;
                        let mut i_5: libc::c_int = 0 as libc::c_int;
                        while i_5 < 8 as libc::c_int {
                            let hint_1: libc::c_int = shifted_frame_offset[i_5 as usize];
                            if used_frame[i_5 as usize] == 0
                                && hint_1 >= current_frame_offset
                                && hint_1 < earliest_frame_offset
                            {
                                (*hdr).refidx[5 as libc::c_int as usize] = i_5;
                                earliest_frame_offset = hint_1;
                            }
                            i_5 += 1;
                        }
                        if earliest_frame_offset != 2147483647 as libc::c_int {
                            used_frame[(*hdr).refidx[5 as libc::c_int as usize]
                                as usize] = 1 as libc::c_int;
                        }
                        let mut i_6: libc::c_int = 1 as libc::c_int;
                        while i_6 < 7 as libc::c_int {
                            if (*hdr).refidx[i_6 as usize] < 0 as libc::c_int {
                                latest_frame_offset = -(1 as libc::c_int);
                                let mut j: libc::c_int = 0 as libc::c_int;
                                while j < 8 as libc::c_int {
                                    let hint_2: libc::c_int = shifted_frame_offset[j as usize];
                                    if used_frame[j as usize] == 0
                                        && hint_2 < current_frame_offset
                                        && hint_2 >= latest_frame_offset
                                    {
                                        (*hdr).refidx[i_6 as usize] = j;
                                        latest_frame_offset = hint_2;
                                    }
                                    j += 1;
                                }
                                if latest_frame_offset != -(1 as libc::c_int) {
                                    used_frame[(*hdr).refidx[i_6 as usize]
                                        as usize] = 1 as libc::c_int;
                                }
                            }
                            i_6 += 1;
                        }
                        earliest_frame_offset = 2147483647 as libc::c_int;
                        let mut ref_0: libc::c_int = -(1 as libc::c_int);
                        let mut i_7: libc::c_int = 0 as libc::c_int;
                        while i_7 < 8 as libc::c_int {
                            let hint_3: libc::c_int = shifted_frame_offset[i_7 as usize];
                            if hint_3 < earliest_frame_offset {
                                ref_0 = i_7;
                                earliest_frame_offset = hint_3;
                            }
                            i_7 += 1;
                        }
                        let mut i_8: libc::c_int = 0 as libc::c_int;
                        while i_8 < 7 as libc::c_int {
                            if (*hdr).refidx[i_8 as usize] < 0 as libc::c_int {
                                (*hdr).refidx[i_8 as usize] = ref_0;
                            }
                            i_8 += 1;
                        }
                        current_block = 16590946904645350046;
                    }
                }
            } else {
                current_block = 16590946904645350046;
            }
            match current_block {
                13312636450699654424 => {}
                _ => {
                    let mut i_9: libc::c_int = 0 as libc::c_int;
                    loop {
                        if !(i_9 < 7 as libc::c_int) {
                            current_block = 5248622017361056354;
                            break;
                        }
                        if (*hdr).frame_ref_short_signaling == 0 {
                            (*hdr)
                                .refidx[i_9
                                as usize] = dav1d_get_bits(gb, 3 as libc::c_int)
                                as libc::c_int;
                        }
                        if (*seqhdr).frame_id_numbers_present != 0 {
                            let delta_ref_frame_id_minus_1: libc::c_int = dav1d_get_bits(
                                gb,
                                (*seqhdr).delta_frame_id_n_bits,
                            ) as libc::c_int;
                            let ref_frame_id: libc::c_int = (*hdr).frame_id
                                + ((1 as libc::c_int) << (*seqhdr).frame_id_n_bits)
                                - delta_ref_frame_id_minus_1 - 1 as libc::c_int
                                & ((1 as libc::c_int) << (*seqhdr).frame_id_n_bits)
                                    - 1 as libc::c_int;
                            let ref_frame_hdr_0: *mut Dav1dFrameHeader = (*c)
                                .refs[(*hdr).refidx[i_9 as usize] as usize]
                                .p
                                .p
                                .frame_hdr;
                            if ref_frame_hdr_0.is_null()
                                || (*ref_frame_hdr_0).frame_id != ref_frame_id
                            {
                                current_block = 13312636450699654424;
                                break;
                            }
                        }
                        i_9 += 1;
                    }
                    match current_block {
                        13312636450699654424 => {}
                        _ => {
                            let use_ref: libc::c_int = ((*hdr).error_resilient_mode == 0
                                && (*hdr).frame_size_override != 0) as libc::c_int;
                            if read_frame_size(c, gb, use_ref) < 0 as libc::c_int {
                                current_block = 13312636450699654424;
                            } else {
                                (*hdr)
                                    .hp = ((*hdr).force_integer_mv == 0
                                    && dav1d_get_bit(gb) != 0) as libc::c_int;
                                (*hdr)
                                    .subpel_filter_mode = (if dav1d_get_bit(gb) != 0 {
                                    DAV1D_FILTER_SWITCHABLE as libc::c_int as libc::c_uint
                                } else {
                                    dav1d_get_bits(gb, 2 as libc::c_int)
                                }) as Dav1dFilterMode;
                                (*hdr)
                                    .switchable_motion_mode = dav1d_get_bit(gb) as libc::c_int;
                                (*hdr)
                                    .use_ref_frame_mvs = ((*hdr).error_resilient_mode == 0
                                    && (*seqhdr).ref_frame_mvs != 0 && (*seqhdr).order_hint != 0
                                    && (*hdr).frame_type as libc::c_uint
                                        & 1 as libc::c_int as libc::c_uint != 0
                                    && dav1d_get_bit(gb) != 0) as libc::c_int;
                                current_block = 16314074004867283505;
                            }
                        }
                    }
                }
            }
        }
        match current_block {
            13312636450699654424 => {}
            _ => {
                (*hdr)
                    .refresh_context = ((*seqhdr).reduced_still_picture_header == 0
                    && (*hdr).disable_cdf_update == 0 && dav1d_get_bit(gb) == 0)
                    as libc::c_int;
                (*hdr).tiling.uniform = dav1d_get_bit(gb) as libc::c_int;
                sbsz_min1 = ((64 as libc::c_int) << (*seqhdr).sb128) - 1 as libc::c_int;
                sbsz_log2 = 6 as libc::c_int + (*seqhdr).sb128;
                sbw = (*hdr).width[0 as libc::c_int as usize] + sbsz_min1 >> sbsz_log2;
                sbh = (*hdr).height + sbsz_min1 >> sbsz_log2;
                max_tile_width_sb = 4096 as libc::c_int >> sbsz_log2;
                max_tile_area_sb = 4096 as libc::c_int * 2304 as libc::c_int
                    >> 2 as libc::c_int * sbsz_log2;
                (*hdr).tiling.min_log2_cols = tile_log2(max_tile_width_sb, sbw);
                (*hdr)
                    .tiling
                    .max_log2_cols = tile_log2(
                    1 as libc::c_int,
                    imin(sbw, 64 as libc::c_int),
                );
                (*hdr)
                    .tiling
                    .max_log2_rows = tile_log2(
                    1 as libc::c_int,
                    imin(sbh, 64 as libc::c_int),
                );
                min_log2_tiles = imax(
                    tile_log2(max_tile_area_sb, sbw * sbh),
                    (*hdr).tiling.min_log2_cols,
                );
                if (*hdr).tiling.uniform != 0 {
                    (*hdr).tiling.log2_cols = (*hdr).tiling.min_log2_cols;
                    while (*hdr).tiling.log2_cols < (*hdr).tiling.max_log2_cols
                        && dav1d_get_bit(gb) != 0
                    {
                        (*hdr).tiling.log2_cols += 1;
                    }
                    let tile_w: libc::c_int = 1 as libc::c_int
                        + (sbw - 1 as libc::c_int >> (*hdr).tiling.log2_cols);
                    (*hdr).tiling.cols = 0 as libc::c_int;
                    let mut sbx: libc::c_int = 0 as libc::c_int;
                    while sbx < sbw {
                        (*hdr)
                            .tiling
                            .col_start_sb[(*hdr).tiling.cols as usize] = sbx as uint16_t;
                        sbx += tile_w;
                        (*hdr).tiling.cols += 1;
                    }
                    (*hdr)
                        .tiling
                        .min_log2_rows = imax(
                        min_log2_tiles - (*hdr).tiling.log2_cols,
                        0 as libc::c_int,
                    );
                    (*hdr).tiling.log2_rows = (*hdr).tiling.min_log2_rows;
                    while (*hdr).tiling.log2_rows < (*hdr).tiling.max_log2_rows
                        && dav1d_get_bit(gb) != 0
                    {
                        (*hdr).tiling.log2_rows += 1;
                    }
                    let tile_h: libc::c_int = 1 as libc::c_int
                        + (sbh - 1 as libc::c_int >> (*hdr).tiling.log2_rows);
                    (*hdr).tiling.rows = 0 as libc::c_int;
                    let mut sby: libc::c_int = 0 as libc::c_int;
                    while sby < sbh {
                        (*hdr)
                            .tiling
                            .row_start_sb[(*hdr).tiling.rows as usize] = sby as uint16_t;
                        sby += tile_h;
                        (*hdr).tiling.rows += 1;
                    }
                } else {
                    (*hdr).tiling.cols = 0 as libc::c_int;
                    let mut widest_tile: libc::c_int = 0 as libc::c_int;
                    let mut max_tile_area_sb_0: libc::c_int = sbw * sbh;
                    let mut sbx_0: libc::c_int = 0 as libc::c_int;
                    while sbx_0 < sbw && (*hdr).tiling.cols < 64 as libc::c_int {
                        let tile_width_sb: libc::c_int = imin(
                            sbw - sbx_0,
                            max_tile_width_sb,
                        );
                        let tile_w_0: libc::c_int = (if tile_width_sb > 1 as libc::c_int
                        {
                            (1 as libc::c_int as libc::c_uint)
                                .wrapping_add(
                                    dav1d_get_uniform(gb, tile_width_sb as libc::c_uint),
                                )
                        } else {
                            1 as libc::c_int as libc::c_uint
                        }) as libc::c_int;
                        (*hdr)
                            .tiling
                            .col_start_sb[(*hdr).tiling.cols
                            as usize] = sbx_0 as uint16_t;
                        sbx_0 += tile_w_0;
                        widest_tile = imax(widest_tile, tile_w_0);
                        (*hdr).tiling.cols += 1;
                    }
                    (*hdr)
                        .tiling
                        .log2_cols = tile_log2(1 as libc::c_int, (*hdr).tiling.cols);
                    if min_log2_tiles != 0 {
                        max_tile_area_sb_0 >>= min_log2_tiles + 1 as libc::c_int;
                    }
                    let max_tile_height_sb: libc::c_int = imax(
                        max_tile_area_sb_0 / widest_tile,
                        1 as libc::c_int,
                    );
                    (*hdr).tiling.rows = 0 as libc::c_int;
                    let mut sby_0: libc::c_int = 0 as libc::c_int;
                    while sby_0 < sbh && (*hdr).tiling.rows < 64 as libc::c_int {
                        let tile_height_sb: libc::c_int = imin(
                            sbh - sby_0,
                            max_tile_height_sb,
                        );
                        let tile_h_0: libc::c_int = (if tile_height_sb > 1 as libc::c_int
                        {
                            (1 as libc::c_int as libc::c_uint)
                                .wrapping_add(
                                    dav1d_get_uniform(gb, tile_height_sb as libc::c_uint),
                                )
                        } else {
                            1 as libc::c_int as libc::c_uint
                        }) as libc::c_int;
                        (*hdr)
                            .tiling
                            .row_start_sb[(*hdr).tiling.rows
                            as usize] = sby_0 as uint16_t;
                        sby_0 += tile_h_0;
                        (*hdr).tiling.rows += 1;
                    }
                    (*hdr)
                        .tiling
                        .log2_rows = tile_log2(1 as libc::c_int, (*hdr).tiling.rows);
                }
                (*hdr)
                    .tiling
                    .col_start_sb[(*hdr).tiling.cols as usize] = sbw as uint16_t;
                (*hdr)
                    .tiling
                    .row_start_sb[(*hdr).tiling.rows as usize] = sbh as uint16_t;
                if (*hdr).tiling.log2_cols != 0 || (*hdr).tiling.log2_rows != 0 {
                    (*hdr)
                        .tiling
                        .update = dav1d_get_bits(
                        gb,
                        (*hdr).tiling.log2_cols + (*hdr).tiling.log2_rows,
                    ) as libc::c_int;
                    if (*hdr).tiling.update >= (*hdr).tiling.cols * (*hdr).tiling.rows {
                        current_block = 13312636450699654424;
                    } else {
                        (*hdr)
                            .tiling
                            .n_bytes = (dav1d_get_bits(gb, 2 as libc::c_int))
                            .wrapping_add(1 as libc::c_int as libc::c_uint);
                        current_block = 1918110639124887667;
                    }
                } else {
                    (*hdr).tiling.update = 0 as libc::c_int;
                    (*hdr).tiling.n_bytes = (*hdr).tiling.update as libc::c_uint;
                    current_block = 1918110639124887667;
                }
                match current_block {
                    13312636450699654424 => {}
                    _ => {
                        (*hdr)
                            .quant
                            .yac = dav1d_get_bits(gb, 8 as libc::c_int) as libc::c_int;
                        (*hdr)
                            .quant
                            .ydc_delta = if dav1d_get_bit(gb) != 0 {
                            dav1d_get_sbits(gb, 7 as libc::c_int)
                        } else {
                            0 as libc::c_int
                        };
                        if (*seqhdr).monochrome == 0 {
                            let diff_uv_delta: libc::c_int = (if (*seqhdr)
                                .separate_uv_delta_q != 0
                            {
                                dav1d_get_bit(gb)
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }) as libc::c_int;
                            (*hdr)
                                .quant
                                .udc_delta = if dav1d_get_bit(gb) != 0 {
                                dav1d_get_sbits(gb, 7 as libc::c_int)
                            } else {
                                0 as libc::c_int
                            };
                            (*hdr)
                                .quant
                                .uac_delta = if dav1d_get_bit(gb) != 0 {
                                dav1d_get_sbits(gb, 7 as libc::c_int)
                            } else {
                                0 as libc::c_int
                            };
                            if diff_uv_delta != 0 {
                                (*hdr)
                                    .quant
                                    .vdc_delta = if dav1d_get_bit(gb) != 0 {
                                    dav1d_get_sbits(gb, 7 as libc::c_int)
                                } else {
                                    0 as libc::c_int
                                };
                                (*hdr)
                                    .quant
                                    .vac_delta = if dav1d_get_bit(gb) != 0 {
                                    dav1d_get_sbits(gb, 7 as libc::c_int)
                                } else {
                                    0 as libc::c_int
                                };
                            } else {
                                (*hdr).quant.vdc_delta = (*hdr).quant.udc_delta;
                                (*hdr).quant.vac_delta = (*hdr).quant.uac_delta;
                            }
                        }
                        (*hdr).quant.qm = dav1d_get_bit(gb) as libc::c_int;
                        if (*hdr).quant.qm != 0 {
                            (*hdr)
                                .quant
                                .qm_y = dav1d_get_bits(gb, 4 as libc::c_int) as libc::c_int;
                            (*hdr)
                                .quant
                                .qm_u = dav1d_get_bits(gb, 4 as libc::c_int) as libc::c_int;
                            (*hdr)
                                .quant
                                .qm_v = if (*seqhdr).separate_uv_delta_q != 0 {
                                dav1d_get_bits(gb, 4 as libc::c_int) as libc::c_int
                            } else {
                                (*hdr).quant.qm_u
                            };
                        }
                        (*hdr).segmentation.enabled = dav1d_get_bit(gb) as libc::c_int;
                        if (*hdr).segmentation.enabled != 0 {
                            if (*hdr).primary_ref_frame == 7 as libc::c_int {
                                (*hdr).segmentation.update_map = 1 as libc::c_int;
                                (*hdr).segmentation.temporal = 0 as libc::c_int;
                                (*hdr).segmentation.update_data = 1 as libc::c_int;
                            } else {
                                (*hdr)
                                    .segmentation
                                    .update_map = dav1d_get_bit(gb) as libc::c_int;
                                (*hdr)
                                    .segmentation
                                    .temporal = (if (*hdr).segmentation.update_map != 0 {
                                    dav1d_get_bit(gb)
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                }) as libc::c_int;
                                (*hdr)
                                    .segmentation
                                    .update_data = dav1d_get_bit(gb) as libc::c_int;
                            }
                            if (*hdr).segmentation.update_data != 0 {
                                (*hdr).segmentation.seg_data.preskip = 0 as libc::c_int;
                                (*hdr)
                                    .segmentation
                                    .seg_data
                                    .last_active_segid = -(1 as libc::c_int);
                                let mut i_10: libc::c_int = 0 as libc::c_int;
                                while i_10 < 8 as libc::c_int {
                                    let seg: *mut Dav1dSegmentationData = &mut *((*hdr)
                                        .segmentation
                                        .seg_data
                                        .d)
                                        .as_mut_ptr()
                                        .offset(i_10 as isize) as *mut Dav1dSegmentationData;
                                    if dav1d_get_bit(gb) != 0 {
                                        (*seg).delta_q = dav1d_get_sbits(gb, 9 as libc::c_int);
                                        (*hdr).segmentation.seg_data.last_active_segid = i_10;
                                    } else {
                                        (*seg).delta_q = 0 as libc::c_int;
                                    }
                                    if dav1d_get_bit(gb) != 0 {
                                        (*seg).delta_lf_y_v = dav1d_get_sbits(gb, 7 as libc::c_int);
                                        (*hdr).segmentation.seg_data.last_active_segid = i_10;
                                    } else {
                                        (*seg).delta_lf_y_v = 0 as libc::c_int;
                                    }
                                    if dav1d_get_bit(gb) != 0 {
                                        (*seg).delta_lf_y_h = dav1d_get_sbits(gb, 7 as libc::c_int);
                                        (*hdr).segmentation.seg_data.last_active_segid = i_10;
                                    } else {
                                        (*seg).delta_lf_y_h = 0 as libc::c_int;
                                    }
                                    if dav1d_get_bit(gb) != 0 {
                                        (*seg).delta_lf_u = dav1d_get_sbits(gb, 7 as libc::c_int);
                                        (*hdr).segmentation.seg_data.last_active_segid = i_10;
                                    } else {
                                        (*seg).delta_lf_u = 0 as libc::c_int;
                                    }
                                    if dav1d_get_bit(gb) != 0 {
                                        (*seg).delta_lf_v = dav1d_get_sbits(gb, 7 as libc::c_int);
                                        (*hdr).segmentation.seg_data.last_active_segid = i_10;
                                    } else {
                                        (*seg).delta_lf_v = 0 as libc::c_int;
                                    }
                                    if dav1d_get_bit(gb) != 0 {
                                        (*seg)
                                            .ref_0 = dav1d_get_bits(gb, 3 as libc::c_int)
                                            as libc::c_int;
                                        (*hdr).segmentation.seg_data.last_active_segid = i_10;
                                        (*hdr).segmentation.seg_data.preskip = 1 as libc::c_int;
                                    } else {
                                        (*seg).ref_0 = -(1 as libc::c_int);
                                    }
                                    (*seg).skip = dav1d_get_bit(gb) as libc::c_int;
                                    if (*seg).skip != 0 {
                                        (*hdr).segmentation.seg_data.last_active_segid = i_10;
                                        (*hdr).segmentation.seg_data.preskip = 1 as libc::c_int;
                                    }
                                    (*seg).globalmv = dav1d_get_bit(gb) as libc::c_int;
                                    if (*seg).globalmv != 0 {
                                        (*hdr).segmentation.seg_data.last_active_segid = i_10;
                                        (*hdr).segmentation.seg_data.preskip = 1 as libc::c_int;
                                    }
                                    i_10 += 1;
                                }
                                current_block = 8075351136037156718;
                            } else {
                                if !((*hdr).primary_ref_frame != 7 as libc::c_int) {
                                    unreachable!();
                                }
                                let pri_ref: libc::c_int = (*hdr)
                                    .refidx[(*hdr).primary_ref_frame as usize];
                                if ((*c).refs[pri_ref as usize].p.p.frame_hdr).is_null() {
                                    current_block = 13312636450699654424;
                                } else {
                                    (*hdr)
                                        .segmentation
                                        .seg_data = (*(*c).refs[pri_ref as usize].p.p.frame_hdr)
                                        .segmentation
                                        .seg_data;
                                    current_block = 8075351136037156718;
                                }
                            }
                        } else {
                            memset(
                                &mut (*hdr).segmentation.seg_data
                                    as *mut Dav1dSegmentationDataSet as *mut libc::c_void,
                                0 as libc::c_int,
                                ::core::mem::size_of::<Dav1dSegmentationDataSet>()
                                    as libc::c_ulong,
                            );
                            let mut i_11: libc::c_int = 0 as libc::c_int;
                            while i_11 < 8 as libc::c_int {
                                (*hdr)
                                    .segmentation
                                    .seg_data
                                    .d[i_11 as usize]
                                    .ref_0 = -(1 as libc::c_int);
                                i_11 += 1;
                            }
                            current_block = 8075351136037156718;
                        }
                        match current_block {
                            13312636450699654424 => {}
                            _ => {
                                (*hdr)
                                    .delta
                                    .q
                                    .present = (if (*hdr).quant.yac != 0 {
                                    dav1d_get_bit(gb)
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                }) as libc::c_int;
                                (*hdr)
                                    .delta
                                    .q
                                    .res_log2 = (if (*hdr).delta.q.present != 0 {
                                    dav1d_get_bits(gb, 2 as libc::c_int)
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                }) as libc::c_int;
                                (*hdr)
                                    .delta
                                    .lf
                                    .present = ((*hdr).delta.q.present != 0
                                    && (*hdr).allow_intrabc == 0 && dav1d_get_bit(gb) != 0)
                                    as libc::c_int;
                                (*hdr)
                                    .delta
                                    .lf
                                    .res_log2 = (if (*hdr).delta.lf.present != 0 {
                                    dav1d_get_bits(gb, 2 as libc::c_int)
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                }) as libc::c_int;
                                (*hdr)
                                    .delta
                                    .lf
                                    .multi = (if (*hdr).delta.lf.present != 0 {
                                    dav1d_get_bit(gb)
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                }) as libc::c_int;
                                delta_lossless = ((*hdr).quant.ydc_delta == 0
                                    && (*hdr).quant.udc_delta == 0
                                    && (*hdr).quant.uac_delta == 0
                                    && (*hdr).quant.vdc_delta == 0
                                    && (*hdr).quant.vac_delta == 0) as libc::c_int;
                                (*hdr).all_lossless = 1 as libc::c_int;
                                let mut i_12: libc::c_int = 0 as libc::c_int;
                                while i_12 < 8 as libc::c_int {
                                    (*hdr)
                                        .segmentation
                                        .qidx[i_12
                                        as usize] = if (*hdr).segmentation.enabled != 0 {
                                        iclip_u8(
                                            (*hdr).quant.yac
                                                + (*hdr).segmentation.seg_data.d[i_12 as usize].delta_q,
                                        )
                                    } else {
                                        (*hdr).quant.yac
                                    };
                                    (*hdr)
                                        .segmentation
                                        .lossless[i_12
                                        as usize] = ((*hdr).segmentation.qidx[i_12 as usize] == 0
                                        && delta_lossless != 0) as libc::c_int;
                                    (*hdr).all_lossless
                                        &= (*hdr).segmentation.lossless[i_12 as usize];
                                    i_12 += 1;
                                }
                                if (*hdr).all_lossless != 0 || (*hdr).allow_intrabc != 0 {
                                    (*hdr)
                                        .loopfilter
                                        .level_y[1 as libc::c_int as usize] = 0 as libc::c_int;
                                    (*hdr)
                                        .loopfilter
                                        .level_y[0 as libc::c_int
                                        as usize] = (*hdr)
                                        .loopfilter
                                        .level_y[1 as libc::c_int as usize];
                                    (*hdr).loopfilter.level_v = 0 as libc::c_int;
                                    (*hdr).loopfilter.level_u = (*hdr).loopfilter.level_v;
                                    (*hdr).loopfilter.sharpness = 0 as libc::c_int;
                                    (*hdr).loopfilter.mode_ref_delta_enabled = 1 as libc::c_int;
                                    (*hdr).loopfilter.mode_ref_delta_update = 1 as libc::c_int;
                                    (*hdr).loopfilter.mode_ref_deltas = default_mode_ref_deltas;
                                    current_block = 1424623445371442388;
                                } else {
                                    (*hdr)
                                        .loopfilter
                                        .level_y[0 as libc::c_int
                                        as usize] = dav1d_get_bits(gb, 6 as libc::c_int)
                                        as libc::c_int;
                                    (*hdr)
                                        .loopfilter
                                        .level_y[1 as libc::c_int
                                        as usize] = dav1d_get_bits(gb, 6 as libc::c_int)
                                        as libc::c_int;
                                    if (*seqhdr).monochrome == 0
                                        && ((*hdr).loopfilter.level_y[0 as libc::c_int as usize]
                                            != 0
                                            || (*hdr).loopfilter.level_y[1 as libc::c_int as usize]
                                                != 0)
                                    {
                                        (*hdr)
                                            .loopfilter
                                            .level_u = dav1d_get_bits(gb, 6 as libc::c_int)
                                            as libc::c_int;
                                        (*hdr)
                                            .loopfilter
                                            .level_v = dav1d_get_bits(gb, 6 as libc::c_int)
                                            as libc::c_int;
                                    }
                                    (*hdr)
                                        .loopfilter
                                        .sharpness = dav1d_get_bits(gb, 3 as libc::c_int)
                                        as libc::c_int;
                                    if (*hdr).primary_ref_frame == 7 as libc::c_int {
                                        (*hdr).loopfilter.mode_ref_deltas = default_mode_ref_deltas;
                                        current_block = 13291976673896753943;
                                    } else {
                                        let ref_1: libc::c_int = (*hdr)
                                            .refidx[(*hdr).primary_ref_frame as usize];
                                        if ((*c).refs[ref_1 as usize].p.p.frame_hdr).is_null() {
                                            current_block = 13312636450699654424;
                                        } else {
                                            (*hdr)
                                                .loopfilter
                                                .mode_ref_deltas = (*(*c)
                                                .refs[ref_1 as usize]
                                                .p
                                                .p
                                                .frame_hdr)
                                                .loopfilter
                                                .mode_ref_deltas;
                                            current_block = 13291976673896753943;
                                        }
                                    }
                                    match current_block {
                                        13312636450699654424 => {}
                                        _ => {
                                            (*hdr)
                                                .loopfilter
                                                .mode_ref_delta_enabled = dav1d_get_bit(gb) as libc::c_int;
                                            if (*hdr).loopfilter.mode_ref_delta_enabled != 0 {
                                                (*hdr)
                                                    .loopfilter
                                                    .mode_ref_delta_update = dav1d_get_bit(gb) as libc::c_int;
                                                if (*hdr).loopfilter.mode_ref_delta_update != 0 {
                                                    let mut i_13: libc::c_int = 0 as libc::c_int;
                                                    while i_13 < 8 as libc::c_int {
                                                        if dav1d_get_bit(gb) != 0 {
                                                            (*hdr)
                                                                .loopfilter
                                                                .mode_ref_deltas
                                                                .ref_delta[i_13
                                                                as usize] = dav1d_get_sbits(gb, 7 as libc::c_int);
                                                        }
                                                        i_13 += 1;
                                                    }
                                                    let mut i_14: libc::c_int = 0 as libc::c_int;
                                                    while i_14 < 2 as libc::c_int {
                                                        if dav1d_get_bit(gb) != 0 {
                                                            (*hdr)
                                                                .loopfilter
                                                                .mode_ref_deltas
                                                                .mode_delta[i_14
                                                                as usize] = dav1d_get_sbits(gb, 7 as libc::c_int);
                                                        }
                                                        i_14 += 1;
                                                    }
                                                }
                                            }
                                            current_block = 1424623445371442388;
                                        }
                                    }
                                }
                                match current_block {
                                    13312636450699654424 => {}
                                    _ => {
                                        if (*hdr).all_lossless == 0 && (*seqhdr).cdef != 0
                                            && (*hdr).allow_intrabc == 0
                                        {
                                            (*hdr)
                                                .cdef
                                                .damping = (dav1d_get_bits(gb, 2 as libc::c_int))
                                                .wrapping_add(3 as libc::c_int as libc::c_uint)
                                                as libc::c_int;
                                            (*hdr)
                                                .cdef
                                                .n_bits = dav1d_get_bits(gb, 2 as libc::c_int)
                                                as libc::c_int;
                                            let mut i_15: libc::c_int = 0 as libc::c_int;
                                            while i_15 < (1 as libc::c_int) << (*hdr).cdef.n_bits {
                                                (*hdr)
                                                    .cdef
                                                    .y_strength[i_15
                                                    as usize] = dav1d_get_bits(gb, 6 as libc::c_int)
                                                    as libc::c_int;
                                                if (*seqhdr).monochrome == 0 {
                                                    (*hdr)
                                                        .cdef
                                                        .uv_strength[i_15
                                                        as usize] = dav1d_get_bits(gb, 6 as libc::c_int)
                                                        as libc::c_int;
                                                }
                                                i_15 += 1;
                                            }
                                        } else {
                                            (*hdr).cdef.n_bits = 0 as libc::c_int;
                                            (*hdr)
                                                .cdef
                                                .y_strength[0 as libc::c_int as usize] = 0 as libc::c_int;
                                            (*hdr)
                                                .cdef
                                                .uv_strength[0 as libc::c_int as usize] = 0 as libc::c_int;
                                        }
                                        if ((*hdr).all_lossless == 0
                                            || (*hdr).super_res.enabled != 0)
                                            && (*seqhdr).restoration != 0 && (*hdr).allow_intrabc == 0
                                        {
                                            (*hdr)
                                                .restoration
                                                .type_0[0 as libc::c_int
                                                as usize] = dav1d_get_bits(gb, 2 as libc::c_int)
                                                as Dav1dRestorationType;
                                            if (*seqhdr).monochrome == 0 {
                                                (*hdr)
                                                    .restoration
                                                    .type_0[1 as libc::c_int
                                                    as usize] = dav1d_get_bits(gb, 2 as libc::c_int)
                                                    as Dav1dRestorationType;
                                                (*hdr)
                                                    .restoration
                                                    .type_0[2 as libc::c_int
                                                    as usize] = dav1d_get_bits(gb, 2 as libc::c_int)
                                                    as Dav1dRestorationType;
                                            } else {
                                                (*hdr)
                                                    .restoration
                                                    .type_0[2 as libc::c_int as usize] = DAV1D_RESTORATION_NONE;
                                                (*hdr)
                                                    .restoration
                                                    .type_0[1 as libc::c_int
                                                    as usize] = (*hdr)
                                                    .restoration
                                                    .type_0[2 as libc::c_int as usize];
                                            }
                                            if (*hdr).restoration.type_0[0 as libc::c_int as usize]
                                                as libc::c_uint != 0
                                                || (*hdr).restoration.type_0[1 as libc::c_int as usize]
                                                    as libc::c_uint != 0
                                                || (*hdr).restoration.type_0[2 as libc::c_int as usize]
                                                    as libc::c_uint != 0
                                            {
                                                (*hdr)
                                                    .restoration
                                                    .unit_size[0 as libc::c_int
                                                    as usize] = 6 as libc::c_int + (*seqhdr).sb128;
                                                if dav1d_get_bit(gb) != 0 {
                                                    (*hdr).restoration.unit_size[0 as libc::c_int as usize]
                                                        += 1;
                                                    if (*seqhdr).sb128 == 0 {
                                                        (*hdr)
                                                            .restoration
                                                            .unit_size[0 as libc::c_int
                                                            as usize] = ((*hdr)
                                                            .restoration
                                                            .unit_size[0 as libc::c_int as usize] as libc::c_uint)
                                                            .wrapping_add(dav1d_get_bit(gb)) as libc::c_int
                                                            as libc::c_int;
                                                    }
                                                }
                                                (*hdr)
                                                    .restoration
                                                    .unit_size[1 as libc::c_int
                                                    as usize] = (*hdr)
                                                    .restoration
                                                    .unit_size[0 as libc::c_int as usize];
                                                if ((*hdr).restoration.type_0[1 as libc::c_int as usize]
                                                    as libc::c_uint != 0
                                                    || (*hdr).restoration.type_0[2 as libc::c_int as usize]
                                                        as libc::c_uint != 0)
                                                    && (*seqhdr).ss_hor == 1 as libc::c_int
                                                    && (*seqhdr).ss_ver == 1 as libc::c_int
                                                {
                                                    (*hdr)
                                                        .restoration
                                                        .unit_size[1 as libc::c_int
                                                        as usize] = ((*hdr)
                                                        .restoration
                                                        .unit_size[1 as libc::c_int as usize] as libc::c_uint)
                                                        .wrapping_sub(dav1d_get_bit(gb)) as libc::c_int
                                                        as libc::c_int;
                                                }
                                            } else {
                                                (*hdr)
                                                    .restoration
                                                    .unit_size[0 as libc::c_int as usize] = 8 as libc::c_int;
                                            }
                                        } else {
                                            (*hdr)
                                                .restoration
                                                .type_0[0 as libc::c_int as usize] = DAV1D_RESTORATION_NONE;
                                            (*hdr)
                                                .restoration
                                                .type_0[1 as libc::c_int as usize] = DAV1D_RESTORATION_NONE;
                                            (*hdr)
                                                .restoration
                                                .type_0[2 as libc::c_int as usize] = DAV1D_RESTORATION_NONE;
                                        }
                                        (*hdr)
                                            .txfm_mode = (if (*hdr).all_lossless != 0 {
                                            DAV1D_TX_4X4_ONLY as libc::c_int
                                        } else if dav1d_get_bit(gb) != 0 {
                                            DAV1D_TX_SWITCHABLE as libc::c_int
                                        } else {
                                            DAV1D_TX_LARGEST as libc::c_int
                                        }) as Dav1dTxfmMode;
                                        (*hdr)
                                            .switchable_comp_refs = (if (*hdr).frame_type
                                            as libc::c_uint & 1 as libc::c_int as libc::c_uint != 0
                                        {
                                            dav1d_get_bit(gb)
                                        } else {
                                            0 as libc::c_int as libc::c_uint
                                        }) as libc::c_int;
                                        (*hdr).skip_mode_allowed = 0 as libc::c_int;
                                        if (*hdr).switchable_comp_refs != 0
                                            && (*hdr).frame_type as libc::c_uint
                                                & 1 as libc::c_int as libc::c_uint != 0
                                            && (*seqhdr).order_hint != 0
                                        {
                                            let poc: libc::c_uint = (*hdr).frame_offset as libc::c_uint;
                                            let mut off_before: libc::c_uint = 0xffffffff
                                                as libc::c_uint;
                                            let mut off_after: libc::c_int = -(1 as libc::c_int);
                                            let mut off_before_idx: libc::c_int = 0;
                                            let mut off_after_idx: libc::c_int = 0;
                                            let mut i_16: libc::c_int = 0 as libc::c_int;
                                            loop {
                                                if !(i_16 < 7 as libc::c_int) {
                                                    current_block = 10953711258009896266;
                                                    break;
                                                }
                                                if ((*c)
                                                    .refs[(*hdr).refidx[i_16 as usize] as usize]
                                                    .p
                                                    .p
                                                    .frame_hdr)
                                                    .is_null()
                                                {
                                                    current_block = 13312636450699654424;
                                                    break;
                                                }
                                                let refpoc: libc::c_uint = (*(*c)
                                                    .refs[(*hdr).refidx[i_16 as usize] as usize]
                                                    .p
                                                    .p
                                                    .frame_hdr)
                                                    .frame_offset as libc::c_uint;
                                                let diff: libc::c_int = get_poc_diff(
                                                    (*seqhdr).order_hint_n_bits,
                                                    refpoc as libc::c_int,
                                                    poc as libc::c_int,
                                                );
                                                if diff > 0 as libc::c_int {
                                                    if off_after == -(1 as libc::c_int)
                                                        || get_poc_diff(
                                                            (*seqhdr).order_hint_n_bits,
                                                            off_after,
                                                            refpoc as libc::c_int,
                                                        ) > 0 as libc::c_int
                                                    {
                                                        off_after = refpoc as libc::c_int;
                                                        off_after_idx = i_16;
                                                    }
                                                } else if diff < 0 as libc::c_int
                                                    && (off_before == 0xffffffff as libc::c_uint
                                                        || get_poc_diff(
                                                            (*seqhdr).order_hint_n_bits,
                                                            refpoc as libc::c_int,
                                                            off_before as libc::c_int,
                                                        ) > 0 as libc::c_int)
                                                {
                                                    off_before = refpoc;
                                                    off_before_idx = i_16;
                                                }
                                                i_16 += 1;
                                            }
                                            match current_block {
                                                13312636450699654424 => {}
                                                _ => {
                                                    if off_before != 0xffffffff as libc::c_uint
                                                        && off_after != -(1 as libc::c_int)
                                                    {
                                                        (*hdr)
                                                            .skip_mode_refs[0 as libc::c_int
                                                            as usize] = imin(off_before_idx, off_after_idx);
                                                        (*hdr)
                                                            .skip_mode_refs[1 as libc::c_int
                                                            as usize] = imax(off_before_idx, off_after_idx);
                                                        (*hdr).skip_mode_allowed = 1 as libc::c_int;
                                                        current_block = 2126221883176060805;
                                                    } else if off_before != 0xffffffff as libc::c_uint {
                                                        let mut off_before2: libc::c_uint = 0xffffffff
                                                            as libc::c_uint;
                                                        let mut off_before2_idx: libc::c_int = 0;
                                                        let mut i_17: libc::c_int = 0 as libc::c_int;
                                                        loop {
                                                            if !(i_17 < 7 as libc::c_int) {
                                                                current_block = 6762054512782224738;
                                                                break;
                                                            }
                                                            if ((*c)
                                                                .refs[(*hdr).refidx[i_17 as usize] as usize]
                                                                .p
                                                                .p
                                                                .frame_hdr)
                                                                .is_null()
                                                            {
                                                                current_block = 13312636450699654424;
                                                                break;
                                                            }
                                                            let refpoc_0: libc::c_uint = (*(*c)
                                                                .refs[(*hdr).refidx[i_17 as usize] as usize]
                                                                .p
                                                                .p
                                                                .frame_hdr)
                                                                .frame_offset as libc::c_uint;
                                                            if get_poc_diff(
                                                                (*seqhdr).order_hint_n_bits,
                                                                refpoc_0 as libc::c_int,
                                                                off_before as libc::c_int,
                                                            ) < 0 as libc::c_int
                                                            {
                                                                if off_before2 == 0xffffffff as libc::c_uint
                                                                    || get_poc_diff(
                                                                        (*seqhdr).order_hint_n_bits,
                                                                        refpoc_0 as libc::c_int,
                                                                        off_before2 as libc::c_int,
                                                                    ) > 0 as libc::c_int
                                                                {
                                                                    off_before2 = refpoc_0;
                                                                    off_before2_idx = i_17;
                                                                }
                                                            }
                                                            i_17 += 1;
                                                        }
                                                        match current_block {
                                                            13312636450699654424 => {}
                                                            _ => {
                                                                if off_before2 != 0xffffffff as libc::c_uint {
                                                                    (*hdr)
                                                                        .skip_mode_refs[0 as libc::c_int
                                                                        as usize] = imin(off_before_idx, off_before2_idx);
                                                                    (*hdr)
                                                                        .skip_mode_refs[1 as libc::c_int
                                                                        as usize] = imax(off_before_idx, off_before2_idx);
                                                                    (*hdr).skip_mode_allowed = 1 as libc::c_int;
                                                                }
                                                                current_block = 2126221883176060805;
                                                            }
                                                        }
                                                    } else {
                                                        current_block = 2126221883176060805;
                                                    }
                                                }
                                            }
                                        } else {
                                            current_block = 2126221883176060805;
                                        }
                                        match current_block {
                                            13312636450699654424 => {}
                                            _ => {
                                                (*hdr)
                                                    .skip_mode_enabled = (if (*hdr).skip_mode_allowed != 0 {
                                                    dav1d_get_bit(gb)
                                                } else {
                                                    0 as libc::c_int as libc::c_uint
                                                }) as libc::c_int;
                                                (*hdr)
                                                    .warp_motion = ((*hdr).error_resilient_mode == 0
                                                    && (*hdr).frame_type as libc::c_uint
                                                        & 1 as libc::c_int as libc::c_uint != 0
                                                    && (*seqhdr).warped_motion != 0 && dav1d_get_bit(gb) != 0)
                                                    as libc::c_int;
                                                (*hdr).reduced_txtp_set = dav1d_get_bit(gb) as libc::c_int;
                                                let mut i_18: libc::c_int = 0 as libc::c_int;
                                                while i_18 < 7 as libc::c_int {
                                                    (*hdr).gmv[i_18 as usize] = dav1d_default_wm_params;
                                                    i_18 += 1;
                                                }
                                                if (*hdr).frame_type as libc::c_uint
                                                    & 1 as libc::c_int as libc::c_uint != 0
                                                {
                                                    let mut i_19: libc::c_int = 0 as libc::c_int;
                                                    loop {
                                                        if !(i_19 < 7 as libc::c_int) {
                                                            current_block = 6933758620287070692;
                                                            break;
                                                        }
                                                        (*hdr)
                                                            .gmv[i_19 as usize]
                                                            .type_0 = (if dav1d_get_bit(gb) == 0 {
                                                            DAV1D_WM_TYPE_IDENTITY as libc::c_int
                                                        } else if dav1d_get_bit(gb) != 0 {
                                                            DAV1D_WM_TYPE_ROT_ZOOM as libc::c_int
                                                        } else if dav1d_get_bit(gb) != 0 {
                                                            DAV1D_WM_TYPE_TRANSLATION as libc::c_int
                                                        } else {
                                                            DAV1D_WM_TYPE_AFFINE as libc::c_int
                                                        }) as Dav1dWarpedMotionType;
                                                        if !((*hdr).gmv[i_19 as usize].type_0 as libc::c_uint
                                                            == DAV1D_WM_TYPE_IDENTITY as libc::c_int as libc::c_uint)
                                                        {
                                                            let mut ref_gmv: *const Dav1dWarpedMotionParams = 0
                                                                as *const Dav1dWarpedMotionParams;
                                                            if (*hdr).primary_ref_frame == 7 as libc::c_int {
                                                                ref_gmv = &dav1d_default_wm_params;
                                                            } else {
                                                                let pri_ref_0: libc::c_int = (*hdr)
                                                                    .refidx[(*hdr).primary_ref_frame as usize];
                                                                if ((*c).refs[pri_ref_0 as usize].p.p.frame_hdr).is_null() {
                                                                    current_block = 13312636450699654424;
                                                                    break;
                                                                }
                                                                ref_gmv = &mut *((*(*((*c).refs)
                                                                    .as_mut_ptr()
                                                                    .offset(pri_ref_0 as isize))
                                                                    .p
                                                                    .p
                                                                    .frame_hdr)
                                                                    .gmv)
                                                                    .as_mut_ptr()
                                                                    .offset(i_19 as isize) as *mut Dav1dWarpedMotionParams;
                                                            }
                                                            let mat: *mut int32_t = ((*hdr).gmv[i_19 as usize].matrix)
                                                                .as_mut_ptr();
                                                            let ref_mat: *const int32_t = ((*ref_gmv).matrix).as_ptr();
                                                            let mut bits: libc::c_int = 0;
                                                            let mut shift: libc::c_int = 0;
                                                            if (*hdr).gmv[i_19 as usize].type_0 as libc::c_uint
                                                                >= DAV1D_WM_TYPE_ROT_ZOOM as libc::c_int as libc::c_uint
                                                            {
                                                                *mat
                                                                    .offset(
                                                                        2 as libc::c_int as isize,
                                                                    ) = ((1 as libc::c_int) << 16 as libc::c_int)
                                                                    + 2 as libc::c_int
                                                                        * dav1d_get_bits_subexp(
                                                                            gb,
                                                                            *ref_mat.offset(2 as libc::c_int as isize)
                                                                                - ((1 as libc::c_int) << 16 as libc::c_int)
                                                                                >> 1 as libc::c_int,
                                                                            12 as libc::c_int as libc::c_uint,
                                                                        );
                                                                *mat
                                                                    .offset(
                                                                        3 as libc::c_int as isize,
                                                                    ) = 2 as libc::c_int
                                                                    * dav1d_get_bits_subexp(
                                                                        gb,
                                                                        *ref_mat.offset(3 as libc::c_int as isize)
                                                                            >> 1 as libc::c_int,
                                                                        12 as libc::c_int as libc::c_uint,
                                                                    );
                                                                bits = 12 as libc::c_int;
                                                                shift = 10 as libc::c_int;
                                                            } else {
                                                                bits = 9 as libc::c_int - ((*hdr).hp == 0) as libc::c_int;
                                                                shift = 13 as libc::c_int + ((*hdr).hp == 0) as libc::c_int;
                                                            }
                                                            if (*hdr).gmv[i_19 as usize].type_0 as libc::c_uint
                                                                == DAV1D_WM_TYPE_AFFINE as libc::c_int as libc::c_uint
                                                            {
                                                                *mat
                                                                    .offset(
                                                                        4 as libc::c_int as isize,
                                                                    ) = 2 as libc::c_int
                                                                    * dav1d_get_bits_subexp(
                                                                        gb,
                                                                        *ref_mat.offset(4 as libc::c_int as isize)
                                                                            >> 1 as libc::c_int,
                                                                        12 as libc::c_int as libc::c_uint,
                                                                    );
                                                                *mat
                                                                    .offset(
                                                                        5 as libc::c_int as isize,
                                                                    ) = ((1 as libc::c_int) << 16 as libc::c_int)
                                                                    + 2 as libc::c_int
                                                                        * dav1d_get_bits_subexp(
                                                                            gb,
                                                                            *ref_mat.offset(5 as libc::c_int as isize)
                                                                                - ((1 as libc::c_int) << 16 as libc::c_int)
                                                                                >> 1 as libc::c_int,
                                                                            12 as libc::c_int as libc::c_uint,
                                                                        );
                                                            } else {
                                                                *mat
                                                                    .offset(
                                                                        4 as libc::c_int as isize,
                                                                    ) = -*mat.offset(3 as libc::c_int as isize);
                                                                *mat
                                                                    .offset(
                                                                        5 as libc::c_int as isize,
                                                                    ) = *mat.offset(2 as libc::c_int as isize);
                                                            }
                                                            *mat
                                                                .offset(
                                                                    0 as libc::c_int as isize,
                                                                ) = dav1d_get_bits_subexp(
                                                                gb,
                                                                *ref_mat.offset(0 as libc::c_int as isize) >> shift,
                                                                bits as libc::c_uint,
                                                            ) * ((1 as libc::c_int) << shift);
                                                            *mat
                                                                .offset(
                                                                    1 as libc::c_int as isize,
                                                                ) = dav1d_get_bits_subexp(
                                                                gb,
                                                                *ref_mat.offset(1 as libc::c_int as isize) >> shift,
                                                                bits as libc::c_uint,
                                                            ) * ((1 as libc::c_int) << shift);
                                                        }
                                                        i_19 += 1;
                                                    }
                                                } else {
                                                    current_block = 6933758620287070692;
                                                }
                                                match current_block {
                                                    13312636450699654424 => {}
                                                    _ => {
                                                        (*hdr)
                                                            .film_grain
                                                            .present = ((*seqhdr).film_grain_present != 0
                                                            && ((*hdr).show_frame != 0 || (*hdr).showable_frame != 0)
                                                            && dav1d_get_bit(gb) != 0) as libc::c_int;
                                                        if (*hdr).film_grain.present != 0 {
                                                            let seed: libc::c_uint = dav1d_get_bits(
                                                                gb,
                                                                16 as libc::c_int,
                                                            );
                                                            (*hdr)
                                                                .film_grain
                                                                .update = ((*hdr).frame_type as libc::c_uint
                                                                != DAV1D_FRAME_TYPE_INTER as libc::c_int as libc::c_uint
                                                                || dav1d_get_bit(gb) != 0) as libc::c_int;
                                                            if (*hdr).film_grain.update == 0 {
                                                                let refidx: libc::c_int = dav1d_get_bits(
                                                                    gb,
                                                                    3 as libc::c_int,
                                                                ) as libc::c_int;
                                                                let mut i_20: libc::c_int = 0;
                                                                i_20 = 0 as libc::c_int;
                                                                while i_20 < 7 as libc::c_int {
                                                                    if (*hdr).refidx[i_20 as usize] == refidx {
                                                                        break;
                                                                    }
                                                                    i_20 += 1;
                                                                }
                                                                if i_20 == 7 as libc::c_int
                                                                    || ((*c).refs[refidx as usize].p.p.frame_hdr).is_null()
                                                                {
                                                                    current_block = 13312636450699654424;
                                                                } else {
                                                                    (*hdr)
                                                                        .film_grain
                                                                        .data = (*(*c).refs[refidx as usize].p.p.frame_hdr)
                                                                        .film_grain
                                                                        .data;
                                                                    (*hdr).film_grain.data.seed = seed;
                                                                    current_block = 17095195114763350366;
                                                                }
                                                            } else {
                                                                let fgd: *mut Dav1dFilmGrainData = &mut (*hdr)
                                                                    .film_grain
                                                                    .data;
                                                                (*fgd).seed = seed;
                                                                (*fgd)
                                                                    .num_y_points = dav1d_get_bits(gb, 4 as libc::c_int)
                                                                    as libc::c_int;
                                                                if (*fgd).num_y_points > 14 as libc::c_int {
                                                                    current_block = 13312636450699654424;
                                                                } else {
                                                                    let mut i_21: libc::c_int = 0 as libc::c_int;
                                                                    loop {
                                                                        if !(i_21 < (*fgd).num_y_points) {
                                                                            current_block = 12030841198858789628;
                                                                            break;
                                                                        }
                                                                        (*fgd)
                                                                            .y_points[i_21
                                                                            as usize][0 as libc::c_int
                                                                            as usize] = dav1d_get_bits(gb, 8 as libc::c_int) as uint8_t;
                                                                        if i_21 != 0
                                                                            && (*fgd)
                                                                                .y_points[(i_21 - 1 as libc::c_int)
                                                                                as usize][0 as libc::c_int as usize] as libc::c_int
                                                                                >= (*fgd).y_points[i_21 as usize][0 as libc::c_int as usize]
                                                                                    as libc::c_int
                                                                        {
                                                                            current_block = 13312636450699654424;
                                                                            break;
                                                                        }
                                                                        (*fgd)
                                                                            .y_points[i_21
                                                                            as usize][1 as libc::c_int
                                                                            as usize] = dav1d_get_bits(gb, 8 as libc::c_int) as uint8_t;
                                                                        i_21 += 1;
                                                                    }
                                                                    match current_block {
                                                                        13312636450699654424 => {}
                                                                        _ => {
                                                                            (*fgd)
                                                                                .chroma_scaling_from_luma = ((*seqhdr).monochrome == 0
                                                                                && dav1d_get_bit(gb) != 0) as libc::c_int;
                                                                            if (*seqhdr).monochrome != 0
                                                                                || (*fgd).chroma_scaling_from_luma != 0
                                                                                || (*seqhdr).ss_ver == 1 as libc::c_int
                                                                                    && (*seqhdr).ss_hor == 1 as libc::c_int
                                                                                    && (*fgd).num_y_points == 0
                                                                            {
                                                                                (*fgd)
                                                                                    .num_uv_points[1 as libc::c_int
                                                                                    as usize] = 0 as libc::c_int;
                                                                                (*fgd)
                                                                                    .num_uv_points[0 as libc::c_int
                                                                                    as usize] = (*fgd).num_uv_points[1 as libc::c_int as usize];
                                                                                current_block = 8773475593684033964;
                                                                            } else {
                                                                                let mut pl: libc::c_int = 0 as libc::c_int;
                                                                                's_1955: loop {
                                                                                    if !(pl < 2 as libc::c_int) {
                                                                                        current_block = 8773475593684033964;
                                                                                        break;
                                                                                    }
                                                                                    (*fgd)
                                                                                        .num_uv_points[pl
                                                                                        as usize] = dav1d_get_bits(gb, 4 as libc::c_int)
                                                                                        as libc::c_int;
                                                                                    if (*fgd).num_uv_points[pl as usize] > 10 as libc::c_int {
                                                                                        current_block = 13312636450699654424;
                                                                                        break;
                                                                                    }
                                                                                    let mut i_22: libc::c_int = 0 as libc::c_int;
                                                                                    while i_22 < (*fgd).num_uv_points[pl as usize] {
                                                                                        (*fgd)
                                                                                            .uv_points[pl
                                                                                            as usize][i_22
                                                                                            as usize][0 as libc::c_int
                                                                                            as usize] = dav1d_get_bits(gb, 8 as libc::c_int) as uint8_t;
                                                                                        if i_22 != 0
                                                                                            && (*fgd)
                                                                                                .uv_points[pl
                                                                                                as usize][(i_22 - 1 as libc::c_int)
                                                                                                as usize][0 as libc::c_int as usize] as libc::c_int
                                                                                                >= (*fgd)
                                                                                                    .uv_points[pl
                                                                                                    as usize][i_22 as usize][0 as libc::c_int as usize]
                                                                                                    as libc::c_int
                                                                                        {
                                                                                            current_block = 13312636450699654424;
                                                                                            break 's_1955;
                                                                                        }
                                                                                        (*fgd)
                                                                                            .uv_points[pl
                                                                                            as usize][i_22
                                                                                            as usize][1 as libc::c_int
                                                                                            as usize] = dav1d_get_bits(gb, 8 as libc::c_int) as uint8_t;
                                                                                        i_22 += 1;
                                                                                    }
                                                                                    pl += 1;
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                13312636450699654424 => {}
                                                                                _ => {
                                                                                    if (*seqhdr).ss_hor == 1 as libc::c_int
                                                                                        && (*seqhdr).ss_ver == 1 as libc::c_int
                                                                                        && ((*fgd).num_uv_points[0 as libc::c_int as usize] != 0)
                                                                                            as libc::c_int
                                                                                            != ((*fgd).num_uv_points[1 as libc::c_int as usize] != 0)
                                                                                                as libc::c_int
                                                                                    {
                                                                                        current_block = 13312636450699654424;
                                                                                    } else {
                                                                                        (*fgd)
                                                                                            .scaling_shift = (dav1d_get_bits(gb, 2 as libc::c_int))
                                                                                            .wrapping_add(8 as libc::c_int as libc::c_uint)
                                                                                            as libc::c_int;
                                                                                        (*fgd)
                                                                                            .ar_coeff_lag = dav1d_get_bits(gb, 2 as libc::c_int)
                                                                                            as libc::c_int;
                                                                                        let num_y_pos: libc::c_int = 2 as libc::c_int
                                                                                            * (*fgd).ar_coeff_lag
                                                                                            * ((*fgd).ar_coeff_lag + 1 as libc::c_int);
                                                                                        if (*fgd).num_y_points != 0 {
                                                                                            let mut i_23: libc::c_int = 0 as libc::c_int;
                                                                                            while i_23 < num_y_pos {
                                                                                                (*fgd)
                                                                                                    .ar_coeffs_y[i_23
                                                                                                    as usize] = (dav1d_get_bits(gb, 8 as libc::c_int))
                                                                                                    .wrapping_sub(128 as libc::c_int as libc::c_uint) as int8_t;
                                                                                                i_23 += 1;
                                                                                            }
                                                                                        }
                                                                                        let mut pl_0: libc::c_int = 0 as libc::c_int;
                                                                                        while pl_0 < 2 as libc::c_int {
                                                                                            if (*fgd).num_uv_points[pl_0 as usize] != 0
                                                                                                || (*fgd).chroma_scaling_from_luma != 0
                                                                                            {
                                                                                                let num_uv_pos: libc::c_int = num_y_pos
                                                                                                    + ((*fgd).num_y_points != 0) as libc::c_int;
                                                                                                let mut i_24: libc::c_int = 0 as libc::c_int;
                                                                                                while i_24 < num_uv_pos {
                                                                                                    (*fgd)
                                                                                                        .ar_coeffs_uv[pl_0
                                                                                                        as usize][i_24
                                                                                                        as usize] = (dav1d_get_bits(gb, 8 as libc::c_int))
                                                                                                        .wrapping_sub(128 as libc::c_int as libc::c_uint) as int8_t;
                                                                                                    i_24 += 1;
                                                                                                }
                                                                                                if (*fgd).num_y_points == 0 {
                                                                                                    (*fgd)
                                                                                                        .ar_coeffs_uv[pl_0
                                                                                                        as usize][num_uv_pos as usize] = 0 as libc::c_int as int8_t;
                                                                                                }
                                                                                            }
                                                                                            pl_0 += 1;
                                                                                        }
                                                                                        (*fgd)
                                                                                            .ar_coeff_shift = (dav1d_get_bits(gb, 2 as libc::c_int))
                                                                                            .wrapping_add(6 as libc::c_int as libc::c_uint) as uint64_t;
                                                                                        (*fgd)
                                                                                            .grain_scale_shift = dav1d_get_bits(gb, 2 as libc::c_int)
                                                                                            as libc::c_int;
                                                                                        let mut pl_1: libc::c_int = 0 as libc::c_int;
                                                                                        while pl_1 < 2 as libc::c_int {
                                                                                            if (*fgd).num_uv_points[pl_1 as usize] != 0 {
                                                                                                (*fgd)
                                                                                                    .uv_mult[pl_1
                                                                                                    as usize] = (dav1d_get_bits(gb, 8 as libc::c_int))
                                                                                                    .wrapping_sub(128 as libc::c_int as libc::c_uint)
                                                                                                    as libc::c_int;
                                                                                                (*fgd)
                                                                                                    .uv_luma_mult[pl_1
                                                                                                    as usize] = (dav1d_get_bits(gb, 8 as libc::c_int))
                                                                                                    .wrapping_sub(128 as libc::c_int as libc::c_uint)
                                                                                                    as libc::c_int;
                                                                                                (*fgd)
                                                                                                    .uv_offset[pl_1
                                                                                                    as usize] = (dav1d_get_bits(gb, 9 as libc::c_int))
                                                                                                    .wrapping_sub(256 as libc::c_int as libc::c_uint)
                                                                                                    as libc::c_int;
                                                                                            }
                                                                                            pl_1 += 1;
                                                                                        }
                                                                                        (*fgd).overlap_flag = dav1d_get_bit(gb) as libc::c_int;
                                                                                        (*fgd)
                                                                                            .clip_to_restricted_range = dav1d_get_bit(gb)
                                                                                            as libc::c_int;
                                                                                        current_block = 17095195114763350366;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            memset(
                                                                &mut (*hdr).film_grain.data as *mut Dav1dFilmGrainData
                                                                    as *mut libc::c_void,
                                                                0 as libc::c_int,
                                                                ::core::mem::size_of::<Dav1dFilmGrainData>()
                                                                    as libc::c_ulong,
                                                            );
                                                            current_block = 17095195114763350366;
                                                        }
                                                        match current_block {
                                                            13312636450699654424 => {}
                                                            _ => return 0 as libc::c_int,
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    dav1d_log(c, b"Error parsing frame header\n\0" as *const u8 as *const libc::c_char);
    return -(22 as libc::c_int);
}
unsafe extern "C" fn parse_tile_hdr(c: *mut Dav1dContext, gb: *mut GetBits) {
    let n_tiles: libc::c_int = (*(*c).frame_hdr).tiling.cols
        * (*(*c).frame_hdr).tiling.rows;
    let have_tile_pos: libc::c_int = (if n_tiles > 1 as libc::c_int {
        dav1d_get_bit(gb)
    } else {
        0 as libc::c_int as libc::c_uint
    }) as libc::c_int;
    if have_tile_pos != 0 {
        let n_bits: libc::c_int = (*(*c).frame_hdr).tiling.log2_cols
            + (*(*c).frame_hdr).tiling.log2_rows;
        (*((*c).tile).offset((*c).n_tile_data as isize))
            .start = dav1d_get_bits(gb, n_bits) as libc::c_int;
        (*((*c).tile).offset((*c).n_tile_data as isize))
            .end = dav1d_get_bits(gb, n_bits) as libc::c_int;
    } else {
        (*((*c).tile).offset((*c).n_tile_data as isize)).start = 0 as libc::c_int;
        (*((*c).tile).offset((*c).n_tile_data as isize))
            .end = n_tiles - 1 as libc::c_int;
    };
}
unsafe extern "C" fn check_for_overrun(
    c: *mut Dav1dContext,
    gb: *mut GetBits,
    init_bit_pos: libc::c_uint,
    obu_len: libc::c_uint,
) -> libc::c_int {
    if (*gb).error != 0 {
        dav1d_log(
            c,
            b"Overrun in OBU bit buffer\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    let pos: libc::c_uint = dav1d_get_bits_pos(gb);
    if !(init_bit_pos <= pos) {
        unreachable!();
    }
    if pos.wrapping_sub(init_bit_pos)
        > (8 as libc::c_int as libc::c_uint).wrapping_mul(obu_len)
    {
        dav1d_log(
            c,
            b"Overrun in OBU bit buffer into next OBU\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_parse_obus(
    c: *mut Dav1dContext,
    in_0: *mut Dav1dData,
    global: libc::c_int,
) -> libc::c_int {
    let mut init_bit_pos: libc::c_uint = 0;
    let mut init_byte_pos: libc::c_uint = 0;
    let mut current_block: u64;
    let mut gb: GetBits = GetBits {
        state: 0,
        bits_left: 0,
        error: 0,
        ptr: 0 as *const uint8_t,
        ptr_start: 0 as *const uint8_t,
        ptr_end: 0 as *const uint8_t,
    };
    let mut res: libc::c_int = 0;
    dav1d_init_get_bits(&mut gb, (*in_0).data, (*in_0).sz);
    dav1d_get_bit(&mut gb);
    let type_0: Dav1dObuType = dav1d_get_bits(&mut gb, 4 as libc::c_int) as Dav1dObuType;
    let has_extension: libc::c_int = dav1d_get_bit(&mut gb) as libc::c_int;
    let has_length_field: libc::c_int = dav1d_get_bit(&mut gb) as libc::c_int;
    dav1d_get_bit(&mut gb);
    let mut temporal_id: libc::c_int = 0 as libc::c_int;
    let mut spatial_id: libc::c_int = 0 as libc::c_int;
    if has_extension != 0 {
        temporal_id = dav1d_get_bits(&mut gb, 3 as libc::c_int) as libc::c_int;
        spatial_id = dav1d_get_bits(&mut gb, 2 as libc::c_int) as libc::c_int;
        dav1d_get_bits(&mut gb, 3 as libc::c_int);
    }
    let len: libc::c_uint = if has_length_field != 0 {
        dav1d_get_uleb128(&mut gb)
    } else {
        ((*in_0).sz as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_sub(has_extension as libc::c_uint)
    };
    if !(gb.error != 0) {
        init_bit_pos = dav1d_get_bits_pos(&mut gb);
        init_byte_pos = init_bit_pos >> 3 as libc::c_int;
        if !(init_bit_pos & 7 as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint)
        {
            unreachable!();
        }
        if !((*in_0).sz >= init_byte_pos as libc::c_ulong) {
            unreachable!();
        }
        if !(len as libc::c_ulong
            > ((*in_0).sz).wrapping_sub(init_byte_pos as libc::c_ulong))
        {
            if type_0 as libc::c_uint != DAV1D_OBU_SEQ_HDR as libc::c_int as libc::c_uint
                && type_0 as libc::c_uint != DAV1D_OBU_TD as libc::c_int as libc::c_uint
                && has_extension != 0
                && (*c).operating_point_idc != 0 as libc::c_int as libc::c_uint
            {
                let in_temporal_layer: libc::c_int = ((*c).operating_point_idc
                    >> temporal_id & 1 as libc::c_int as libc::c_uint) as libc::c_int;
                let in_spatial_layer: libc::c_int = ((*c).operating_point_idc
                    >> spatial_id + 8 as libc::c_int & 1 as libc::c_int as libc::c_uint)
                    as libc::c_int;
                if in_temporal_layer == 0 || in_spatial_layer == 0 {
                    return len.wrapping_add(init_byte_pos) as libc::c_int;
                }
            }
            match type_0 as libc::c_uint {
                1 => {
                    let mut ref_0: *mut Dav1dRef = dav1d_ref_create_using_pool(
                        (*c).seq_hdr_pool,
                        ::core::mem::size_of::<Dav1dSequenceHeader>() as libc::c_ulong,
                    );
                    if ref_0.is_null() {
                        return -(12 as libc::c_int);
                    }
                    let mut seq_hdr: *mut Dav1dSequenceHeader = (*ref_0).data
                        as *mut Dav1dSequenceHeader;
                    res = parse_seq_hdr(c, &mut gb, seq_hdr);
                    if res < 0 as libc::c_int {
                        dav1d_ref_dec(&mut ref_0);
                        current_block = 13588377604982898435;
                    } else if check_for_overrun(c, &mut gb, init_bit_pos, len) != 0 {
                        dav1d_ref_dec(&mut ref_0);
                        current_block = 13588377604982898435;
                    } else {
                        if ((*c).seq_hdr).is_null() {
                            (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
                            (*c)
                                .frame_flags = ::core::mem::transmute::<
                                libc::c_uint,
                                PictureFlags,
                            >(
                                (*c).frame_flags as libc::c_uint
                                    | PICTURE_FLAG_NEW_SEQUENCE as libc::c_int as libc::c_uint,
                            );
                        } else if memcmp(
                            seq_hdr as *const libc::c_void,
                            (*c).seq_hdr as *const libc::c_void,
                            1100 as libc::c_ulong,
                        ) != 0
                        {
                            (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
                            (*c).mastering_display = 0 as *mut Dav1dMasteringDisplay;
                            (*c).content_light = 0 as *mut Dav1dContentLightLevel;
                            dav1d_ref_dec(&mut (*c).mastering_display_ref);
                            dav1d_ref_dec(&mut (*c).content_light_ref);
                            let mut i: libc::c_int = 0 as libc::c_int;
                            while i < 8 as libc::c_int {
                                if !((*c).refs[i as usize].p.p.frame_hdr).is_null() {
                                    dav1d_thread_picture_unref(
                                        &mut (*((*c).refs).as_mut_ptr().offset(i as isize)).p,
                                    );
                                }
                                dav1d_ref_dec(
                                    &mut (*((*c).refs).as_mut_ptr().offset(i as isize)).segmap,
                                );
                                dav1d_ref_dec(
                                    &mut (*((*c).refs).as_mut_ptr().offset(i as isize)).refmvs,
                                );
                                dav1d_cdf_thread_unref(
                                    &mut *((*c).cdf).as_mut_ptr().offset(i as isize),
                                );
                                i += 1;
                            }
                            (*c)
                                .frame_flags = ::core::mem::transmute::<
                                libc::c_uint,
                                PictureFlags,
                            >(
                                (*c).frame_flags as libc::c_uint
                                    | PICTURE_FLAG_NEW_SEQUENCE as libc::c_int as libc::c_uint,
                            );
                        } else if memcmp(
                            ((*seq_hdr).operating_parameter_info).as_mut_ptr()
                                as *const libc::c_void,
                            ((*(*c).seq_hdr).operating_parameter_info).as_mut_ptr()
                                as *const libc::c_void,
                            ::core::mem::size_of::<
                                [Dav1dSequenceHeaderOperatingParameterInfo; 32],
                            >() as libc::c_ulong,
                        ) != 0
                        {
                            (*c)
                                .frame_flags = ::core::mem::transmute::<
                                libc::c_uint,
                                PictureFlags,
                            >(
                                (*c).frame_flags as libc::c_uint
                                    | PICTURE_FLAG_NEW_OP_PARAMS_INFO as libc::c_int
                                        as libc::c_uint,
                            );
                        }
                        dav1d_ref_dec(&mut (*c).seq_hdr_ref);
                        (*c).seq_hdr_ref = ref_0;
                        (*c).seq_hdr = seq_hdr;
                        current_block = 2704538829018177290;
                    }
                }
                7 => {
                    if !((*c).frame_hdr).is_null() {
                        current_block = 2704538829018177290;
                    } else {
                        current_block = 916061708005926980;
                    }
                }
                6 | 3 => {
                    current_block = 916061708005926980;
                }
                4 => {
                    current_block = 919954187481050311;
                }
                5 => {
                    let meta_type: ObuMetaType = dav1d_get_uleb128(&mut gb)
                        as ObuMetaType;
                    let meta_type_len: libc::c_int = ((dav1d_get_bits_pos(&mut gb))
                        .wrapping_sub(init_bit_pos) >> 3 as libc::c_int) as libc::c_int;
                    if gb.error != 0 {
                        current_block = 13588377604982898435;
                    } else {
                        match meta_type as libc::c_uint {
                            1 => {
                                let mut ref_1: *mut Dav1dRef = dav1d_ref_create(
                                    ::core::mem::size_of::<Dav1dContentLightLevel>()
                                        as libc::c_ulong,
                                );
                                if ref_1.is_null() {
                                    return -(12 as libc::c_int);
                                }
                                let content_light: *mut Dav1dContentLightLevel = (*ref_1)
                                    .data as *mut Dav1dContentLightLevel;
                                (*content_light)
                                    .max_content_light_level = dav1d_get_bits(
                                    &mut gb,
                                    16 as libc::c_int,
                                ) as libc::c_int;
                                (*content_light)
                                    .max_frame_average_light_level = dav1d_get_bits(
                                    &mut gb,
                                    16 as libc::c_int,
                                ) as libc::c_int;
                                dav1d_get_bit(&mut gb);
                                dav1d_bytealign_get_bits(&mut gb);
                                if check_for_overrun(c, &mut gb, init_bit_pos, len) != 0 {
                                    dav1d_ref_dec(&mut ref_1);
                                    current_block = 13588377604982898435;
                                } else {
                                    dav1d_ref_dec(&mut (*c).content_light_ref);
                                    (*c).content_light = content_light;
                                    (*c).content_light_ref = ref_1;
                                    current_block = 2704538829018177290;
                                }
                            }
                            2 => {
                                let mut ref_2: *mut Dav1dRef = dav1d_ref_create(
                                    ::core::mem::size_of::<Dav1dMasteringDisplay>()
                                        as libc::c_ulong,
                                );
                                if ref_2.is_null() {
                                    return -(12 as libc::c_int);
                                }
                                let mastering_display: *mut Dav1dMasteringDisplay = (*ref_2)
                                    .data as *mut Dav1dMasteringDisplay;
                                let mut i_1: libc::c_int = 0 as libc::c_int;
                                while i_1 < 3 as libc::c_int {
                                    (*mastering_display)
                                        .primaries[i_1
                                        as usize][0 as libc::c_int
                                        as usize] = dav1d_get_bits(&mut gb, 16 as libc::c_int)
                                        as uint16_t;
                                    (*mastering_display)
                                        .primaries[i_1
                                        as usize][1 as libc::c_int
                                        as usize] = dav1d_get_bits(&mut gb, 16 as libc::c_int)
                                        as uint16_t;
                                    i_1 += 1;
                                }
                                (*mastering_display)
                                    .white_point[0 as libc::c_int
                                    as usize] = dav1d_get_bits(&mut gb, 16 as libc::c_int)
                                    as uint16_t;
                                (*mastering_display)
                                    .white_point[1 as libc::c_int
                                    as usize] = dav1d_get_bits(&mut gb, 16 as libc::c_int)
                                    as uint16_t;
                                (*mastering_display)
                                    .max_luminance = dav1d_get_bits(&mut gb, 32 as libc::c_int);
                                (*mastering_display)
                                    .min_luminance = dav1d_get_bits(&mut gb, 32 as libc::c_int);
                                dav1d_get_bit(&mut gb);
                                dav1d_bytealign_get_bits(&mut gb);
                                if check_for_overrun(c, &mut gb, init_bit_pos, len) != 0 {
                                    dav1d_ref_dec(&mut ref_2);
                                    current_block = 13588377604982898435;
                                } else {
                                    dav1d_ref_dec(&mut (*c).mastering_display_ref);
                                    (*c).mastering_display = mastering_display;
                                    (*c).mastering_display_ref = ref_2;
                                    current_block = 2704538829018177290;
                                }
                            }
                            4 => {
                                let mut payload_size: libc::c_int = len as libc::c_int;
                                while payload_size > 0 as libc::c_int
                                    && *((*in_0).data)
                                        .offset(
                                            init_byte_pos
                                                .wrapping_add(payload_size as libc::c_uint)
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                        ) == 0
                                {
                                    payload_size -= 1;
                                }
                                payload_size -= 1;
                                payload_size -= meta_type_len;
                                let mut country_code_extension_byte: libc::c_int = 0
                                    as libc::c_int;
                                let country_code: libc::c_int = dav1d_get_bits(
                                    &mut gb,
                                    8 as libc::c_int,
                                ) as libc::c_int;
                                payload_size -= 1;
                                if country_code == 0xff as libc::c_int {
                                    country_code_extension_byte = dav1d_get_bits(
                                        &mut gb,
                                        8 as libc::c_int,
                                    ) as libc::c_int;
                                    payload_size -= 1;
                                }
                                if payload_size <= 0 as libc::c_int {
                                    dav1d_log(
                                        c,
                                        b"Malformed ITU-T T.35 metadata message format\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                } else {
                                    let mut ref_3: *mut Dav1dRef = dav1d_ref_create(
                                        (::core::mem::size_of::<Dav1dITUTT35>() as libc::c_ulong)
                                            .wrapping_add(
                                                (payload_size as libc::c_ulong)
                                                    .wrapping_mul(
                                                        ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
                                                    ),
                                            ),
                                    );
                                    if ref_3.is_null() {
                                        return -(12 as libc::c_int);
                                    }
                                    let itut_t35_metadata: *mut Dav1dITUTT35 = (*ref_3).data
                                        as *mut Dav1dITUTT35;
                                    (*itut_t35_metadata)
                                        .payload = &mut *itut_t35_metadata
                                        .offset(1 as libc::c_int as isize) as *mut Dav1dITUTT35
                                        as *mut uint8_t;
                                    (*itut_t35_metadata).country_code = country_code as uint8_t;
                                    (*itut_t35_metadata)
                                        .country_code_extension_byte = country_code_extension_byte
                                        as uint8_t;
                                    let mut i_2: libc::c_int = 0 as libc::c_int;
                                    while i_2 < payload_size {
                                        *((*itut_t35_metadata).payload)
                                            .offset(
                                                i_2 as isize,
                                            ) = dav1d_get_bits(&mut gb, 8 as libc::c_int) as uint8_t;
                                        i_2 += 1;
                                    }
                                    (*itut_t35_metadata).payload_size = payload_size as size_t;
                                    dav1d_ref_dec(&mut (*c).itut_t35_ref);
                                    (*c).itut_t35 = itut_t35_metadata;
                                    (*c).itut_t35_ref = ref_3;
                                }
                                current_block = 2704538829018177290;
                            }
                            3 | 5 => {
                                current_block = 2704538829018177290;
                            }
                            _ => {
                                dav1d_log(
                                    c,
                                    b"Unknown Metadata OBU type %d\n\0" as *const u8
                                        as *const libc::c_char,
                                    meta_type as libc::c_uint,
                                );
                                current_block = 2704538829018177290;
                            }
                        }
                    }
                }
                2 => {
                    (*c)
                        .frame_flags = ::core::mem::transmute::<
                        libc::c_uint,
                        PictureFlags,
                    >(
                        (*c).frame_flags as libc::c_uint
                            | PICTURE_FLAG_NEW_TEMPORAL_UNIT as libc::c_int
                                as libc::c_uint,
                    );
                    current_block = 2704538829018177290;
                }
                15 => {
                    current_block = 2704538829018177290;
                }
                _ => {
                    dav1d_log(
                        c,
                        b"Unknown OBU type %d of size %u\n\0" as *const u8
                            as *const libc::c_char,
                        type_0 as libc::c_uint,
                        len,
                    );
                    current_block = 2704538829018177290;
                }
            }
            match current_block {
                13588377604982898435 => {}
                _ => {
                    match current_block {
                        916061708005926980 => {
                            if global != 0 {
                                current_block = 2704538829018177290;
                            } else if ((*c).seq_hdr).is_null() {
                                current_block = 13588377604982898435;
                            } else {
                                if ((*c).frame_hdr_ref).is_null() {
                                    (*c)
                                        .frame_hdr_ref = dav1d_ref_create_using_pool(
                                        (*c).frame_hdr_pool,
                                        ::core::mem::size_of::<Dav1dFrameHeader>() as libc::c_ulong,
                                    );
                                    if ((*c).frame_hdr_ref).is_null() {
                                        return -(12 as libc::c_int);
                                    }
                                }
                                (*c)
                                    .frame_hdr = (*(*c).frame_hdr_ref).data
                                    as *mut Dav1dFrameHeader;
                                memset(
                                    (*c).frame_hdr as *mut libc::c_void,
                                    0 as libc::c_int,
                                    ::core::mem::size_of::<Dav1dFrameHeader>() as libc::c_ulong,
                                );
                                (*(*c).frame_hdr).temporal_id = temporal_id;
                                (*(*c).frame_hdr).spatial_id = spatial_id;
                                res = parse_frame_hdr(c, &mut gb);
                                if res < 0 as libc::c_int {
                                    (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
                                    current_block = 13588377604982898435;
                                } else {
                                    let mut n: libc::c_int = 0 as libc::c_int;
                                    while n < (*c).n_tile_data {
                                        dav1d_data_unref_internal(
                                            &mut (*((*c).tile).offset(n as isize)).data,
                                        );
                                        n += 1;
                                    }
                                    (*c).n_tile_data = 0 as libc::c_int;
                                    (*c).n_tiles = 0 as libc::c_int;
                                    if type_0 as libc::c_uint
                                        != DAV1D_OBU_FRAME as libc::c_int as libc::c_uint
                                    {
                                        dav1d_get_bit(&mut gb);
                                        if check_for_overrun(c, &mut gb, init_bit_pos, len) != 0 {
                                            (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
                                            current_block = 13588377604982898435;
                                        } else {
                                            current_block = 4216521074440650966;
                                        }
                                    } else {
                                        current_block = 4216521074440650966;
                                    }
                                    match current_block {
                                        13588377604982898435 => {}
                                        _ => {
                                            if (*c).frame_size_limit != 0
                                                && (*(*c).frame_hdr).width[1 as libc::c_int as usize]
                                                    as int64_t * (*(*c).frame_hdr).height as libc::c_long
                                                    > (*c).frame_size_limit as libc::c_long
                                            {
                                                dav1d_log(
                                                    c,
                                                    b"Frame size %dx%d exceeds limit %u\n\0" as *const u8
                                                        as *const libc::c_char,
                                                    (*(*c).frame_hdr).width[1 as libc::c_int as usize],
                                                    (*(*c).frame_hdr).height,
                                                    (*c).frame_size_limit,
                                                );
                                                (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
                                                return -(34 as libc::c_int);
                                            }
                                            if type_0 as libc::c_uint
                                                != DAV1D_OBU_FRAME as libc::c_int as libc::c_uint
                                            {
                                                current_block = 2704538829018177290;
                                            } else if (*(*c).frame_hdr).show_existing_frame != 0 {
                                                (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
                                                current_block = 13588377604982898435;
                                            } else {
                                                dav1d_bytealign_get_bits(&mut gb);
                                                current_block = 919954187481050311;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    match current_block {
                        13588377604982898435 => {}
                        _ => {
                            match current_block {
                                919954187481050311 => {
                                    if global != 0 {
                                        current_block = 2704538829018177290;
                                    } else if ((*c).frame_hdr).is_null() {
                                        current_block = 13588377604982898435;
                                    } else {
                                        if (*c).n_tile_data_alloc
                                            < (*c).n_tile_data + 1 as libc::c_int
                                        {
                                            if (*c).n_tile_data + 1 as libc::c_int
                                                > 2147483647 as libc::c_int
                                                    / ::core::mem::size_of::<Dav1dTileGroup>() as libc::c_ulong
                                                        as libc::c_int
                                            {
                                                current_block = 13588377604982898435;
                                            } else {
                                                let mut tile: *mut Dav1dTileGroup = realloc(
                                                    (*c).tile as *mut libc::c_void,
                                                    (((*c).n_tile_data + 1 as libc::c_int) as libc::c_ulong)
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<Dav1dTileGroup>() as libc::c_ulong,
                                                        ),
                                                ) as *mut Dav1dTileGroup;
                                                if tile.is_null() {
                                                    current_block = 13588377604982898435;
                                                } else {
                                                    (*c).tile = tile;
                                                    memset(
                                                        ((*c).tile).offset((*c).n_tile_data as isize)
                                                            as *mut libc::c_void,
                                                        0 as libc::c_int,
                                                        ::core::mem::size_of::<Dav1dTileGroup>() as libc::c_ulong,
                                                    );
                                                    (*c)
                                                        .n_tile_data_alloc = (*c).n_tile_data + 1 as libc::c_int;
                                                    current_block = 17711149709958600598;
                                                }
                                            }
                                        } else {
                                            current_block = 17711149709958600598;
                                        }
                                        match current_block {
                                            13588377604982898435 => {}
                                            _ => {
                                                parse_tile_hdr(c, &mut gb);
                                                dav1d_bytealign_get_bits(&mut gb);
                                                if check_for_overrun(c, &mut gb, init_bit_pos, len) != 0 {
                                                    current_block = 13588377604982898435;
                                                } else {
                                                    let pkt_bytelen: libc::c_uint = init_byte_pos
                                                        .wrapping_add(len);
                                                    let bit_pos: libc::c_uint = dav1d_get_bits_pos(&mut gb);
                                                    if !(bit_pos & 7 as libc::c_int as libc::c_uint
                                                        == 0 as libc::c_int as libc::c_uint)
                                                    {
                                                        unreachable!();
                                                    }
                                                    if !(pkt_bytelen >= bit_pos >> 3 as libc::c_int) {
                                                        unreachable!();
                                                    }
                                                    dav1d_data_ref(
                                                        &mut (*((*c).tile).offset((*c).n_tile_data as isize)).data,
                                                        in_0,
                                                    );
                                                    let ref mut fresh2 = (*((*c).tile)
                                                        .offset((*c).n_tile_data as isize))
                                                        .data
                                                        .data;
                                                    *fresh2 = (*fresh2)
                                                        .offset((bit_pos >> 3 as libc::c_int) as isize);
                                                    (*((*c).tile).offset((*c).n_tile_data as isize))
                                                        .data
                                                        .sz = pkt_bytelen.wrapping_sub(bit_pos >> 3 as libc::c_int)
                                                        as size_t;
                                                    if (*((*c).tile).offset((*c).n_tile_data as isize)).start
                                                        > (*((*c).tile).offset((*c).n_tile_data as isize)).end
                                                        || (*((*c).tile).offset((*c).n_tile_data as isize)).start
                                                            != (*c).n_tiles
                                                    {
                                                        let mut i_0: libc::c_int = 0 as libc::c_int;
                                                        while i_0 <= (*c).n_tile_data {
                                                            dav1d_data_unref_internal(
                                                                &mut (*((*c).tile).offset(i_0 as isize)).data,
                                                            );
                                                            i_0 += 1;
                                                        }
                                                        (*c).n_tile_data = 0 as libc::c_int;
                                                        (*c).n_tiles = 0 as libc::c_int;
                                                        current_block = 13588377604982898435;
                                                    } else {
                                                        (*c).n_tiles
                                                            += 1 as libc::c_int
                                                                + (*((*c).tile).offset((*c).n_tile_data as isize)).end
                                                                - (*((*c).tile).offset((*c).n_tile_data as isize)).start;
                                                        (*c).n_tile_data += 1;
                                                        current_block = 2704538829018177290;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                            match current_block {
                                13588377604982898435 => {}
                                _ => {
                                    if !((*c).seq_hdr).is_null() && !((*c).frame_hdr).is_null()
                                    {
                                        if (*(*c).frame_hdr).show_existing_frame != 0 {
                                            if ((*c)
                                                .refs[(*(*c).frame_hdr).existing_frame_idx as usize]
                                                .p
                                                .p
                                                .frame_hdr)
                                                .is_null()
                                            {
                                                current_block = 13588377604982898435;
                                            } else {
                                                match (*(*c)
                                                    .refs[(*(*c).frame_hdr).existing_frame_idx as usize]
                                                    .p
                                                    .p
                                                    .frame_hdr)
                                                    .frame_type as libc::c_uint
                                                {
                                                    1 | 3 => {
                                                        if (*c).decode_frame_type as libc::c_uint
                                                            > DAV1D_DECODEFRAMETYPE_REFERENCE as libc::c_int
                                                                as libc::c_uint
                                                        {
                                                            current_block = 13679153167263055587;
                                                        } else {
                                                            current_block = 12969817083969514432;
                                                        }
                                                    }
                                                    2 => {
                                                        if (*c).decode_frame_type as libc::c_uint
                                                            > DAV1D_DECODEFRAMETYPE_INTRA as libc::c_int as libc::c_uint
                                                        {
                                                            current_block = 13679153167263055587;
                                                        } else {
                                                            current_block = 12969817083969514432;
                                                        }
                                                    }
                                                    _ => {
                                                        current_block = 12969817083969514432;
                                                    }
                                                }
                                                match current_block {
                                                    13679153167263055587 => {}
                                                    _ => {
                                                        if ((*c)
                                                            .refs[(*(*c).frame_hdr).existing_frame_idx as usize]
                                                            .p
                                                            .p
                                                            .data[0 as libc::c_int as usize])
                                                            .is_null()
                                                        {
                                                            current_block = 13588377604982898435;
                                                        } else if (*c).strict_std_compliance != 0
                                                            && (*c)
                                                                .refs[(*(*c).frame_hdr).existing_frame_idx as usize]
                                                                .p
                                                                .showable == 0
                                                        {
                                                            current_block = 13588377604982898435;
                                                        } else {
                                                            if (*c).n_fc == 1 as libc::c_int as libc::c_uint {
                                                                dav1d_thread_picture_ref(
                                                                    &mut (*c).out,
                                                                    &mut (*((*c).refs)
                                                                        .as_mut_ptr()
                                                                        .offset((*(*c).frame_hdr).existing_frame_idx as isize))
                                                                        .p,
                                                                );
                                                                dav1d_data_props_copy(&mut (*c).out.p.m, &mut (*in_0).m);
                                                                (*c)
                                                                    .event_flags = ::core::mem::transmute::<
                                                                    libc::c_uint,
                                                                    Dav1dEventFlags,
                                                                >(
                                                                    (*c).event_flags as libc::c_uint
                                                                        | dav1d_picture_get_event_flags(
                                                                            &mut (*((*c).refs)
                                                                                .as_mut_ptr()
                                                                                .offset((*(*c).frame_hdr).existing_frame_idx as isize))
                                                                                .p,
                                                                        ) as libc::c_uint,
                                                                );
                                                            } else {
                                                                pthread_mutex_lock(&mut (*c).task_thread.lock);
                                                                let fresh3 = (*c).frame_thread.next;
                                                                (*c)
                                                                    .frame_thread
                                                                    .next = ((*c).frame_thread.next).wrapping_add(1);
                                                                let next: libc::c_uint = fresh3;
                                                                if (*c).frame_thread.next == (*c).n_fc {
                                                                    (*c).frame_thread.next = 0 as libc::c_int as libc::c_uint;
                                                                }
                                                                let f: *mut Dav1dFrameContext = &mut *((*c).fc)
                                                                    .offset(next as isize) as *mut Dav1dFrameContext;
                                                                while (*f).n_tile_data > 0 as libc::c_int {
                                                                    pthread_cond_wait(
                                                                        &mut (*f).task_thread.cond,
                                                                        &mut (*(*f).task_thread.ttd).lock,
                                                                    );
                                                                }
                                                                let out_delayed: *mut Dav1dThreadPicture = &mut *((*c)
                                                                    .frame_thread
                                                                    .out_delayed)
                                                                    .offset(next as isize) as *mut Dav1dThreadPicture;
                                                                if !((*out_delayed).p.data[0 as libc::c_int as usize])
                                                                    .is_null()
                                                                    || ::core::intrinsics::atomic_load_seqcst(
                                                                        &mut (*f).task_thread.error as *mut atomic_int,
                                                                    ) != 0
                                                                {
                                                                    let mut first: libc::c_uint = ::core::intrinsics::atomic_load_seqcst(
                                                                        &mut (*c).task_thread.first,
                                                                    );
                                                                    if first.wrapping_add(1 as libc::c_uint) < (*c).n_fc {
                                                                        ::core::intrinsics::atomic_xadd_seqcst(
                                                                            &mut (*c).task_thread.first,
                                                                            1 as libc::c_uint,
                                                                        );
                                                                    } else {
                                                                        ::core::intrinsics::atomic_store_seqcst(
                                                                            &mut (*c).task_thread.first,
                                                                            0 as libc::c_int as libc::c_uint,
                                                                        );
                                                                    }
                                                                    let fresh6 = ::core::intrinsics::atomic_cxchg_seqcst_seqcst(
                                                                        &mut (*c).task_thread.reset_task_cur,
                                                                        *&mut first,
                                                                        (2147483647 as libc::c_int as libc::c_uint)
                                                                            .wrapping_mul(2 as libc::c_uint)
                                                                            .wrapping_add(1 as libc::c_uint),
                                                                    );
                                                                    *&mut first = fresh6.0;
                                                                    fresh6.1;
                                                                    if (*c).task_thread.cur != 0
                                                                        && (*c).task_thread.cur < (*c).n_fc
                                                                    {
                                                                        (*c)
                                                                            .task_thread
                                                                            .cur = ((*c).task_thread.cur).wrapping_sub(1);
                                                                    }
                                                                }
                                                                let error: libc::c_int = (*f).task_thread.retval;
                                                                if error != 0 {
                                                                    (*c).cached_error = error;
                                                                    (*f).task_thread.retval = 0 as libc::c_int;
                                                                    dav1d_data_props_copy(
                                                                        &mut (*c).cached_error_props,
                                                                        &mut (*out_delayed).p.m,
                                                                    );
                                                                    dav1d_thread_picture_unref(out_delayed);
                                                                } else if !((*out_delayed)
                                                                    .p
                                                                    .data[0 as libc::c_int as usize])
                                                                    .is_null()
                                                                {
                                                                    let progress: libc::c_uint = ::core::intrinsics::atomic_load_relaxed(
                                                                        &mut *((*out_delayed).progress)
                                                                            .offset(1 as libc::c_int as isize) as *mut atomic_uint,
                                                                    );
                                                                    if ((*out_delayed).visible != 0
                                                                        || (*c).output_invisible_frames != 0)
                                                                        && progress
                                                                            != (2147483647 as libc::c_int as libc::c_uint)
                                                                                .wrapping_mul(2 as libc::c_uint)
                                                                                .wrapping_add(1 as libc::c_uint)
                                                                                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                                                    {
                                                                        dav1d_thread_picture_ref(&mut (*c).out, out_delayed);
                                                                        (*c)
                                                                            .event_flags = ::core::mem::transmute::<
                                                                            libc::c_uint,
                                                                            Dav1dEventFlags,
                                                                        >(
                                                                            (*c).event_flags as libc::c_uint
                                                                                | dav1d_picture_get_event_flags(out_delayed) as libc::c_uint,
                                                                        );
                                                                    }
                                                                    dav1d_thread_picture_unref(out_delayed);
                                                                }
                                                                dav1d_thread_picture_ref(
                                                                    out_delayed,
                                                                    &mut (*((*c).refs)
                                                                        .as_mut_ptr()
                                                                        .offset((*(*c).frame_hdr).existing_frame_idx as isize))
                                                                        .p,
                                                                );
                                                                (*out_delayed).visible = 1 as libc::c_int;
                                                                dav1d_data_props_copy(
                                                                    &mut (*out_delayed).p.m,
                                                                    &mut (*in_0).m,
                                                                );
                                                                pthread_mutex_unlock(&mut (*c).task_thread.lock);
                                                            }
                                                            if (*(*c)
                                                                .refs[(*(*c).frame_hdr).existing_frame_idx as usize]
                                                                .p
                                                                .p
                                                                .frame_hdr)
                                                                .frame_type as libc::c_uint
                                                                == DAV1D_FRAME_TYPE_KEY as libc::c_int as libc::c_uint
                                                            {
                                                                let r: libc::c_int = (*(*c).frame_hdr).existing_frame_idx;
                                                                (*c).refs[r as usize].p.showable = 0 as libc::c_int;
                                                                let mut i_3: libc::c_int = 0 as libc::c_int;
                                                                while i_3 < 8 as libc::c_int {
                                                                    if !(i_3 == r) {
                                                                        if !((*c).refs[i_3 as usize].p.p.frame_hdr).is_null() {
                                                                            dav1d_thread_picture_unref(
                                                                                &mut (*((*c).refs).as_mut_ptr().offset(i_3 as isize)).p,
                                                                            );
                                                                        }
                                                                        dav1d_thread_picture_ref(
                                                                            &mut (*((*c).refs).as_mut_ptr().offset(i_3 as isize)).p,
                                                                            &mut (*((*c).refs).as_mut_ptr().offset(r as isize)).p,
                                                                        );
                                                                        dav1d_cdf_thread_unref(
                                                                            &mut *((*c).cdf).as_mut_ptr().offset(i_3 as isize),
                                                                        );
                                                                        dav1d_cdf_thread_ref(
                                                                            &mut *((*c).cdf).as_mut_ptr().offset(i_3 as isize),
                                                                            &mut *((*c).cdf).as_mut_ptr().offset(r as isize),
                                                                        );
                                                                        dav1d_ref_dec(
                                                                            &mut (*((*c).refs).as_mut_ptr().offset(i_3 as isize)).segmap,
                                                                        );
                                                                        (*c)
                                                                            .refs[i_3 as usize]
                                                                            .segmap = (*c).refs[r as usize].segmap;
                                                                        if !((*c).refs[r as usize].segmap).is_null() {
                                                                            dav1d_ref_inc((*c).refs[r as usize].segmap);
                                                                        }
                                                                        dav1d_ref_dec(
                                                                            &mut (*((*c).refs).as_mut_ptr().offset(i_3 as isize)).refmvs,
                                                                        );
                                                                    }
                                                                    i_3 += 1;
                                                                }
                                                            }
                                                            (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
                                                            current_block = 16221891950104054966;
                                                        }
                                                    }
                                                }
                                            }
                                        } else if (*c).n_tiles
                                            == (*(*c).frame_hdr).tiling.cols
                                                * (*(*c).frame_hdr).tiling.rows
                                        {
                                            match (*(*c).frame_hdr).frame_type as libc::c_uint {
                                                1 | 3 => {
                                                    if (*c).decode_frame_type as libc::c_uint
                                                        > DAV1D_DECODEFRAMETYPE_REFERENCE as libc::c_int
                                                            as libc::c_uint
                                                        || (*c).decode_frame_type as libc::c_uint
                                                            == DAV1D_DECODEFRAMETYPE_REFERENCE as libc::c_int
                                                                as libc::c_uint
                                                            && (*(*c).frame_hdr).refresh_frame_flags == 0
                                                    {
                                                        current_block = 13679153167263055587;
                                                    } else {
                                                        current_block = 1622976744501948573;
                                                    }
                                                }
                                                2 => {
                                                    if (*c).decode_frame_type as libc::c_uint
                                                        > DAV1D_DECODEFRAMETYPE_INTRA as libc::c_int as libc::c_uint
                                                        || (*c).decode_frame_type as libc::c_uint
                                                            == DAV1D_DECODEFRAMETYPE_REFERENCE as libc::c_int
                                                                as libc::c_uint
                                                            && (*(*c).frame_hdr).refresh_frame_flags == 0
                                                    {
                                                        current_block = 13679153167263055587;
                                                    } else {
                                                        current_block = 1622976744501948573;
                                                    }
                                                }
                                                _ => {
                                                    current_block = 1622976744501948573;
                                                }
                                            }
                                            match current_block {
                                                13679153167263055587 => {}
                                                _ => {
                                                    if (*c).n_tile_data == 0 {
                                                        current_block = 13588377604982898435;
                                                    } else {
                                                        res = dav1d_submit_frame(c);
                                                        if res < 0 as libc::c_int {
                                                            return res;
                                                        }
                                                        if (*c).n_tile_data != 0 {
                                                            unreachable!();
                                                        }
                                                        (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
                                                        (*c).n_tiles = 0 as libc::c_int;
                                                        current_block = 16221891950104054966;
                                                    }
                                                }
                                            }
                                        } else {
                                            current_block = 16221891950104054966;
                                        }
                                        match current_block {
                                            16221891950104054966 => {}
                                            13588377604982898435 => {}
                                            _ => {
                                                let mut i_4: libc::c_int = 0 as libc::c_int;
                                                while i_4 < 8 as libc::c_int {
                                                    if (*(*c).frame_hdr).refresh_frame_flags
                                                        & (1 as libc::c_int) << i_4 != 0
                                                    {
                                                        dav1d_thread_picture_unref(
                                                            &mut (*((*c).refs).as_mut_ptr().offset(i_4 as isize)).p,
                                                        );
                                                        (*c).refs[i_4 as usize].p.p.frame_hdr = (*c).frame_hdr;
                                                        (*c).refs[i_4 as usize].p.p.seq_hdr = (*c).seq_hdr;
                                                        (*c)
                                                            .refs[i_4 as usize]
                                                            .p
                                                            .p
                                                            .frame_hdr_ref = (*c).frame_hdr_ref;
                                                        (*c).refs[i_4 as usize].p.p.seq_hdr_ref = (*c).seq_hdr_ref;
                                                        dav1d_ref_inc((*c).frame_hdr_ref);
                                                        dav1d_ref_inc((*c).seq_hdr_ref);
                                                    }
                                                    i_4 += 1;
                                                }
                                                dav1d_ref_dec(&mut (*c).frame_hdr_ref);
                                                (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
                                                (*c).n_tiles = 0 as libc::c_int;
                                                return len.wrapping_add(init_byte_pos) as libc::c_int;
                                            }
                                        }
                                    } else {
                                        current_block = 16221891950104054966;
                                    }
                                    match current_block {
                                        13588377604982898435 => {}
                                        _ => return len.wrapping_add(init_byte_pos) as libc::c_int,
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    dav1d_data_props_copy(&mut (*c).cached_error_props, &mut (*in_0).m);
    dav1d_log(c, b"Error parsing OBU data\n\0" as *const u8 as *const libc::c_char);
    return -(22 as libc::c_int);
}
