use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
use crate::src::cdf::CdfContext;
use crate::src::msac::MsacContext;
use crate::{stderr,errno_location};
extern "C" {
    fn fprintf(_: *mut libc::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: size_t,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn dav1d_mem_pool_push(pool: *mut Dav1dMemPool, buf: *mut Dav1dMemPoolBuffer);
    fn dav1d_mem_pool_pop(
        pool: *mut Dav1dMemPool,
        size: size_t,
    ) -> *mut Dav1dMemPoolBuffer;
    fn dav1d_ref_wrap(
        ptr: *const uint8_t,
        free_callback: Option::<
            unsafe extern "C" fn(*const uint8_t, *mut libc::c_void) -> (),
        >,
        user_data: *mut libc::c_void,
    ) -> *mut Dav1dRef;
    fn dav1d_ref_dec(ref_0: *mut *mut Dav1dRef);
    fn dav1d_data_props_copy(dst: *mut Dav1dDataProps, src: *const Dav1dDataProps);
    fn dav1d_data_props_set_defaults(props: *mut Dav1dDataProps);
    fn dav1d_log(c: *mut Dav1dContext, format: *const libc::c_char, _: ...);
}












#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: libc::c_ulonglong,
    pub __value32: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
<<<<<<< HEAD





use crate::include::pthread::pthread_mutex_t;
use crate::include::pthread::pthread_cond_t;







use crate::include::stdatomic::atomic_int;
use crate::include::stdatomic::atomic_uint;

use crate::src::r#ref::Dav1dRef;
use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::dav1d::data::Dav1dData;
=======
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
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
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type ptrdiff_t = isize;
pub type memory_order = libc::c_uint;
pub const memory_order_seq_cst: memory_order = 5;
pub const memory_order_acq_rel: memory_order = 4;
pub const memory_order_release: memory_order = 3;
pub const memory_order_acquire: memory_order = 2;
pub const memory_order_consume: memory_order = 1;
pub const memory_order_relaxed: memory_order = 0;
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
>>>>>>> bb10c4cd (WIP: additional fixes - incomplete)
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
    pub task_thread: C2RustUnnamed_0,
    pub tile_thread: FrameTileThreadData,
}
use crate::src::internal::FrameTileThreadData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
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
    pub pending_tasks: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub merge: atomic_int,
    pub lock: pthread_mutex_t,
    pub head: *mut Dav1dTask,
    pub tail: *mut Dav1dTask,
}
use crate::src::internal::Dav1dTask;
use crate::src::internal::TaskType;













#[derive(Copy, Clone)]
#[repr(C)]
pub struct TaskThreadData {
    pub lock: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub first: atomic_uint,
    pub cur: libc::c_uint,
    pub reset_task_cur: atomic_uint,
    pub cond_signaled: atomic_int,
    pub delayed_fg: C2RustUnnamed_2,
    pub inited: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub exec: libc::c_int,
    pub cond: pthread_cond_t,
    pub in_0: *const Dav1dPicture,
    pub out: *mut Dav1dPicture,
    pub type_0: TaskType,
    pub progress: [atomic_int; 2],
    pub c2rust_unnamed: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub c2rust_unnamed_0: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub grain_lut_16bpc: [[[int16_t; 82]; 74]; 3],
    pub scaling_16bpc: [[uint8_t; 4096]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
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
use crate::include::dav1d::picture::Dav1dPictureParameters;

use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;

use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I400;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader {
    pub film_grain: C2RustUnnamed_18,
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
    pub super_res: C2RustUnnamed_17,
    pub have_render_size: libc::c_int,
    pub allow_intrabc: libc::c_int,
    pub frame_ref_short_signaling: libc::c_int,
    pub refidx: [libc::c_int; 7],
    pub hp: libc::c_int,
    pub subpel_filter_mode: Dav1dFilterMode,
    pub switchable_motion_mode: libc::c_int,
    pub use_ref_frame_mvs: libc::c_int,
    pub refresh_context: libc::c_int,
    pub tiling: C2RustUnnamed_16,
    pub quant: C2RustUnnamed_15,
    pub segmentation: C2RustUnnamed_14,
    pub delta: C2RustUnnamed_11,
    pub all_lossless: libc::c_int,
    pub loopfilter: C2RustUnnamed_10,
    pub cdef: C2RustUnnamed_9,
    pub restoration: C2RustUnnamed_8,
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
    pub u: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub p: C2RustUnnamed_7,
    pub abcd: [int16_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub alpha: int16_t,
    pub beta: int16_t,
    pub gamma: int16_t,
    pub delta: int16_t,
}
use crate::include::dav1d::headers::Dav1dWarpedMotionType;




use crate::include::dav1d::headers::Dav1dTxfmMode;




#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub type_0: [Dav1dRestorationType; 3],
    pub unit_size: [libc::c_int; 2],
}
use crate::include::dav1d::headers::Dav1dRestorationType;




#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub damping: libc::c_int,
    pub n_bits: libc::c_int,
    pub y_strength: [libc::c_int; 8],
    pub uv_strength: [libc::c_int; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
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
pub struct C2RustUnnamed_11 {
    pub q: C2RustUnnamed_13,
    pub lf: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub present: libc::c_int,
    pub res_log2: libc::c_int,
    pub multi: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub present: libc::c_int,
    pub res_log2: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub enabled: libc::c_int,
    pub update_map: libc::c_int,
    pub temporal: libc::c_int,
    pub update_data: libc::c_int,
    pub seg_data: Dav1dSegmentationDataSet,
    pub lossless: [libc::c_int; 8],
    pub qidx: [libc::c_int; 8],
}
use crate::include::dav1d::headers::Dav1dSegmentationDataSet;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
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
pub struct C2RustUnnamed_16 {
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







#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub width_scale_denominator: libc::c_int,
    pub enabled: libc::c_int,
}
use crate::include::dav1d::headers::Dav1dFrameHeaderOperatingPoint;
use crate::include::dav1d::headers::Dav1dFrameType;




#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub data: Dav1dFilmGrainData,
    pub present: libc::c_int,
    pub update: libc::c_int,
}
use crate::include::dav1d::headers::Dav1dFilmGrainData;
use crate::include::dav1d::headers::Dav1dSequenceHeader;



























































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
use crate::src::lf_mask::Av1Filter;
pub type pixel = ();
use crate::src::lf_mask::Av1FilterLUT;
use crate::src::lf_mask::Av1Restoration;
use crate::src::lf_mask::Av1RestorationUnit;
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
use crate::src::internal::CodedBlockInfo;
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
use crate::src::env::BlockContext;
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
use crate::src::levels::BlockSize;























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
    pub c2rust_unnamed: Dav1dTaskContext_cf,
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
use crate::src::thread_data::thread_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
    pub pass: libc::c_int,
}
use crate::src::levels::Filter2d;











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
    pub c2rust_unnamed_0: Dav1dTaskContext_scratch_interintra_edge,
}
use crate::src::internal::Dav1dTaskContext_scratch_interintra_edge;


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
use crate::src::internal::Dav1dTaskContext_cf;
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
use crate::include::dav1d::dav1d::Dav1dLogger;
use crate::include::dav1d::dav1d::Dav1dEventFlags;
use crate::include::dav1d::dav1d::DAV1D_EVENT_FLAG_NEW_OP_PARAMS_INFO;
use crate::include::dav1d::dav1d::DAV1D_EVENT_FLAG_NEW_SEQUENCE;
pub type PictureFlags = libc::c_uint;
pub const PICTURE_FLAG_NEW_TEMPORAL_UNIT: PictureFlags = 4;
pub const PICTURE_FLAG_NEW_OP_PARAMS_INFO: PictureFlags = 2;
pub const PICTURE_FLAG_NEW_SEQUENCE: PictureFlags = 1;
use crate::include::dav1d::dav1d::Dav1dDecodeFrameType;




use crate::include::dav1d::dav1d::Dav1dInloopFilterType;





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
use crate::src::intra_edge::EdgeTip;
use crate::src::intra_edge::EdgeFlags;






use crate::src::intra_edge::EdgeNode;
use crate::src::intra_edge::EdgeBranch;
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
use crate::src::looprestoration::LrEdgeFlags;




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
use crate::src::cdef::CdefEdgeFlags;




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
use crate::src::internal::Dav1dTileGroup;
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
use crate::src::internal::ScalableMotionParams;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pic_ctx_context {
    pub allocator: Dav1dPicAllocator,
    pub pic: Dav1dPicture,
    pub extra_ptr: *mut libc::c_void,
}
#[inline]
unsafe extern "C" fn dav1d_ref_inc(ref_0: *mut Dav1dRef) {
    ::core::intrinsics::atomic_xadd_relaxed(&mut (*ref_0).ref_cnt, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_default_picture_alloc(
    p: *mut Dav1dPicture,
    cookie: *mut libc::c_void,
) -> libc::c_int {
    if !(::core::mem::size_of::<Dav1dMemPoolBuffer>() as libc::c_ulong
        <= 64 as libc::c_int as libc::c_ulong)
    {
        unreachable!();
    }
    let hbd: libc::c_int = ((*p).p.bpc > 8 as libc::c_int) as libc::c_int;
    let aligned_w: libc::c_int = (*p).p.w + 127 as libc::c_int & !(127 as libc::c_int);
    let aligned_h: libc::c_int = (*p).p.h + 127 as libc::c_int & !(127 as libc::c_int);
    let has_chroma: libc::c_int = ((*p).p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_ver: libc::c_int = ((*p).p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_hor: libc::c_int = ((*p).p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
    let mut y_stride: ptrdiff_t = (aligned_w << hbd) as ptrdiff_t;
    let mut uv_stride: ptrdiff_t = if has_chroma != 0 {
        y_stride >> ss_hor
    } else {
        0
    };
    if y_stride & 1023 == 0 {
        y_stride += 64;
    }
    if uv_stride & 1023 == 0 && has_chroma != 0 {
        uv_stride += 64;
    }
    (*p).stride[0 as libc::c_int as usize] = y_stride;
    (*p).stride[1 as libc::c_int as usize] = uv_stride;
    let y_sz: size_t = (y_stride * aligned_h as isize) as size_t;
    let uv_sz: size_t = (uv_stride * (aligned_h >> ss_ver) as isize) as size_t;
    let pic_size: size_t = y_sz
        .wrapping_add(2usize.wrapping_mul(uv_sz));
    let buf: *mut Dav1dMemPoolBuffer = dav1d_mem_pool_pop(
        cookie as *mut Dav1dMemPool,
        pic_size
            .wrapping_add(64)
            .wrapping_sub(::core::mem::size_of::<Dav1dMemPoolBuffer>()),
    );
    if buf.is_null() {
        return -(12 as libc::c_int);
    }
    (*p).allocator_data = buf as *mut libc::c_void;
    let data: *mut uint8_t = (*buf).data as *mut uint8_t;
    (*p).data[0 as libc::c_int as usize] = data as *mut libc::c_void;
    (*p)
        .data[1 as libc::c_int
        as usize] = (if has_chroma != 0 {
        data.offset(y_sz as isize)
    } else {
        0 as *mut uint8_t
    }) as *mut libc::c_void;
    (*p)
        .data[2 as libc::c_int
        as usize] = (if has_chroma != 0 {
        data.offset(y_sz as isize).offset(uv_sz as isize)
    } else {
        0 as *mut uint8_t
    }) as *mut libc::c_void;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_default_picture_release(
    p: *mut Dav1dPicture,
    cookie: *mut libc::c_void,
) {
    dav1d_mem_pool_push(
        cookie as *mut Dav1dMemPool,
        (*p).allocator_data as *mut Dav1dMemPoolBuffer,
    );
}
unsafe extern "C" fn free_buffer(_data: *const uint8_t, user_data: *mut libc::c_void) {
    let mut pic_ctx: *mut pic_ctx_context = user_data as *mut pic_ctx_context;
    ((*pic_ctx).allocator.release_picture_callback)
        .expect(
            "non-null function pointer",
        )(&mut (*pic_ctx).pic, (*pic_ctx).allocator.cookie);
    free(pic_ctx as *mut libc::c_void);
}
unsafe extern "C" fn picture_alloc_with_edges(
    c: *mut Dav1dContext,
    p: *mut Dav1dPicture,
    w: libc::c_int,
    h: libc::c_int,
    seq_hdr: *mut Dav1dSequenceHeader,
    seq_hdr_ref: *mut Dav1dRef,
    frame_hdr: *mut Dav1dFrameHeader,
    frame_hdr_ref: *mut Dav1dRef,
    content_light: *mut Dav1dContentLightLevel,
    content_light_ref: *mut Dav1dRef,
    mastering_display: *mut Dav1dMasteringDisplay,
    mastering_display_ref: *mut Dav1dRef,
    itut_t35: *mut Dav1dITUTT35,
    itut_t35_ref: *mut Dav1dRef,
    bpc: libc::c_int,
    props: *const Dav1dDataProps,
    p_allocator: *mut Dav1dPicAllocator,
    extra: size_t,
    extra_ptr: *mut *mut libc::c_void,
) -> libc::c_int {
    if !((*p).data[0 as libc::c_int as usize]).is_null() {
        dav1d_log(
            c,
            b"Picture already allocated!\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if !(bpc > 0 as libc::c_int && bpc <= 16 as libc::c_int) {
        unreachable!();
    }
    let mut pic_ctx: *mut pic_ctx_context = malloc(
        extra.wrapping_add(::core::mem::size_of::<pic_ctx_context>()),
    ) as *mut pic_ctx_context;
    if pic_ctx.is_null() {
        return -(12 as libc::c_int);
    }
    (*p).p.w = w;
    (*p).p.h = h;
    (*p).seq_hdr = seq_hdr;
    (*p).frame_hdr = frame_hdr;
    (*p).content_light = content_light;
    (*p).mastering_display = mastering_display;
    (*p).itut_t35 = itut_t35;
    (*p).p.layout = (*seq_hdr).layout;
    (*p).p.bpc = bpc;
    dav1d_data_props_set_defaults(&mut (*p).m);
    let res: libc::c_int = ((*p_allocator).alloc_picture_callback)
        .expect("non-null function pointer")(p, (*p_allocator).cookie);
    if res < 0 as libc::c_int {
        free(pic_ctx as *mut libc::c_void);
        return res;
    }
    (*pic_ctx).allocator = *p_allocator;
    (*pic_ctx).pic = *p;
    (*p)
        .ref_0 = dav1d_ref_wrap(
        (*p).data[0 as libc::c_int as usize] as *const uint8_t,
        Some(
            free_buffer as unsafe extern "C" fn(*const uint8_t, *mut libc::c_void) -> (),
        ),
        pic_ctx as *mut libc::c_void,
    );
    if ((*p).ref_0).is_null() {
        ((*p_allocator).release_picture_callback)
            .expect("non-null function pointer")(p, (*p_allocator).cookie);
        free(pic_ctx as *mut libc::c_void);
        dav1d_log(
            c,
            b"Failed to wrap picture: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*errno_location()),
        );
        return -(12 as libc::c_int);
    }
    (*p).seq_hdr_ref = seq_hdr_ref;
    if !seq_hdr_ref.is_null() {
        dav1d_ref_inc(seq_hdr_ref);
    }
    (*p).frame_hdr_ref = frame_hdr_ref;
    if !frame_hdr_ref.is_null() {
        dav1d_ref_inc(frame_hdr_ref);
    }
    dav1d_data_props_copy(&mut (*p).m, props);
    if extra != 0 && !extra_ptr.is_null() {
        *extra_ptr = &mut (*pic_ctx).extra_ptr as *mut *mut libc::c_void
            as *mut libc::c_void;
    }
    (*p).content_light_ref = content_light_ref;
    if !content_light_ref.is_null() {
        dav1d_ref_inc(content_light_ref);
    }
    (*p).mastering_display_ref = mastering_display_ref;
    if !mastering_display_ref.is_null() {
        dav1d_ref_inc(mastering_display_ref);
    }
    (*p).itut_t35_ref = itut_t35_ref;
    if !itut_t35_ref.is_null() {
        dav1d_ref_inc(itut_t35_ref);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_thread_picture_alloc(
    c: *mut Dav1dContext,
    f: *mut Dav1dFrameContext,
    bpc: libc::c_int,
) -> libc::c_int {
    let p: *mut Dav1dThreadPicture = &mut (*f).sr_cur;
    let have_frame_mt: libc::c_int = ((*c).n_fc > 1 as libc::c_int as libc::c_uint)
        as libc::c_int;
    let res: libc::c_int = picture_alloc_with_edges(
        c,
        &mut (*p).p,
        (*(*f).frame_hdr).width[1 as libc::c_int as usize],
        (*(*f).frame_hdr).height,
        (*f).seq_hdr,
        (*f).seq_hdr_ref,
        (*f).frame_hdr,
        (*f).frame_hdr_ref,
        (*c).content_light,
        (*c).content_light_ref,
        (*c).mastering_display,
        (*c).mastering_display_ref,
        (*c).itut_t35,
        (*c).itut_t35_ref,
        bpc,
        &mut (*((*f).tile).offset(0 as libc::c_int as isize)).data.m,
        &mut (*c).allocator,
        if have_frame_mt != 0 {
            (::core::mem::size_of::<atomic_int>())
                .wrapping_mul(2)
        } else {
            0
        },
        &mut (*p).progress as *mut *mut atomic_uint as *mut *mut libc::c_void,
    );
    if res != 0 {
        return res;
    }
    dav1d_ref_dec(&mut (*c).itut_t35_ref);
    (*c).itut_t35 = 0 as *mut Dav1dITUTT35;
    let flags_mask: libc::c_int = if (*(*f).frame_hdr).show_frame != 0
        || (*c).output_invisible_frames != 0
    {
        0 as libc::c_int
    } else {
        PICTURE_FLAG_NEW_SEQUENCE as libc::c_int
            | PICTURE_FLAG_NEW_OP_PARAMS_INFO as libc::c_int
    };
    (*p).flags = (*c).frame_flags;
    (*c)
        .frame_flags = ::core::mem::transmute::<
        libc::c_uint,
        PictureFlags,
    >((*c).frame_flags as libc::c_uint & flags_mask as libc::c_uint);
    (*p).visible = (*(*f).frame_hdr).show_frame;
    (*p).showable = (*(*f).frame_hdr).showable_frame;
    if have_frame_mt != 0 {
        *(&mut *((*p).progress).offset(0 as libc::c_int as isize)
            as *mut atomic_uint) = 0 as libc::c_int as libc::c_uint;
        *(&mut *((*p).progress).offset(1 as libc::c_int as isize)
            as *mut atomic_uint) = 0 as libc::c_int as libc::c_uint;
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_picture_alloc_copy(
    c: *mut Dav1dContext,
    dst: *mut Dav1dPicture,
    w: libc::c_int,
    src: *const Dav1dPicture,
) -> libc::c_int {
    let pic_ctx: *mut pic_ctx_context = (*(*src).ref_0).user_data
        as *mut pic_ctx_context;
    let res: libc::c_int = picture_alloc_with_edges(
        c,
        dst,
        w,
        (*src).p.h,
        (*src).seq_hdr,
        (*src).seq_hdr_ref,
        (*src).frame_hdr,
        (*src).frame_hdr_ref,
        (*src).content_light,
        (*src).content_light_ref,
        (*src).mastering_display,
        (*src).mastering_display_ref,
        (*src).itut_t35,
        (*src).itut_t35_ref,
        (*src).p.bpc,
        &(*src).m,
        &mut (*pic_ctx).allocator,
        0 as libc::c_int as size_t,
        0 as *mut *mut libc::c_void,
    );
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_picture_ref(
    dst: *mut Dav1dPicture,
    src: *const Dav1dPicture,
) {
    if dst.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const libc::c_char,
            b"dst != ((void*)0)\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"dav1d_picture_ref\0"))
                .as_ptr(),
        );
        return;
    }
    if !((*dst).data[0 as libc::c_int as usize]).is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const libc::c_char,
            b"dst->data[0] == ((void*)0)\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"dav1d_picture_ref\0"))
                .as_ptr(),
        );
        return;
    }
    if src.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const libc::c_char,
            b"src != ((void*)0)\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"dav1d_picture_ref\0"))
                .as_ptr(),
        );
        return;
    }
    if !((*src).ref_0).is_null() {
        if ((*src).data[0 as libc::c_int as usize]).is_null() {
            fprintf(
                stderr,
                b"Input validation check '%s' failed in %s!\n\0" as *const u8
                    as *const libc::c_char,
                b"src->data[0] != ((void*)0)\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 18],
                    &[libc::c_char; 18],
                >(b"dav1d_picture_ref\0"))
                    .as_ptr(),
            );
            return;
        }
        dav1d_ref_inc((*src).ref_0);
    }
    if !((*src).frame_hdr_ref).is_null() {
        dav1d_ref_inc((*src).frame_hdr_ref);
    }
    if !((*src).seq_hdr_ref).is_null() {
        dav1d_ref_inc((*src).seq_hdr_ref);
    }
    if !((*src).m.user_data.ref_0).is_null() {
        dav1d_ref_inc((*src).m.user_data.ref_0);
    }
    if !((*src).content_light_ref).is_null() {
        dav1d_ref_inc((*src).content_light_ref);
    }
    if !((*src).mastering_display_ref).is_null() {
        dav1d_ref_inc((*src).mastering_display_ref);
    }
    if !((*src).itut_t35_ref).is_null() {
        dav1d_ref_inc((*src).itut_t35_ref);
    }
    *dst = *src;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_picture_move_ref(
    dst: *mut Dav1dPicture,
    src: *mut Dav1dPicture,
) {
    if dst.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const libc::c_char,
            b"dst != ((void*)0)\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"dav1d_picture_move_ref\0"))
                .as_ptr(),
        );
        return;
    }
    if !((*dst).data[0 as libc::c_int as usize]).is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const libc::c_char,
            b"dst->data[0] == ((void*)0)\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"dav1d_picture_move_ref\0"))
                .as_ptr(),
        );
        return;
    }
    if src.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const libc::c_char,
            b"src != ((void*)0)\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"dav1d_picture_move_ref\0"))
                .as_ptr(),
        );
        return;
    }
    if !((*src).ref_0).is_null() {
        if ((*src).data[0 as libc::c_int as usize]).is_null() {
            fprintf(
                stderr,
                b"Input validation check '%s' failed in %s!\n\0" as *const u8
                    as *const libc::c_char,
                b"src->data[0] != ((void*)0)\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"dav1d_picture_move_ref\0"))
                    .as_ptr(),
            );
            return;
        }
    }
    *dst = *src;
    memset(
        src as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Dav1dPicture>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_thread_picture_ref(
    dst: *mut Dav1dThreadPicture,
    src: *const Dav1dThreadPicture,
) {
    dav1d_picture_ref(&mut (*dst).p, &(*src).p);
    (*dst).visible = (*src).visible;
    (*dst).showable = (*src).showable;
    (*dst).progress = (*src).progress;
    (*dst).flags = (*src).flags;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_thread_picture_move_ref(
    dst: *mut Dav1dThreadPicture,
    src: *mut Dav1dThreadPicture,
) {
    dav1d_picture_move_ref(&mut (*dst).p, &mut (*src).p);
    (*dst).visible = (*src).visible;
    (*dst).showable = (*src).showable;
    (*dst).progress = (*src).progress;
    (*dst).flags = (*src).flags;
    memset(
        src as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Dav1dThreadPicture>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_picture_unref_internal(p: *mut Dav1dPicture) {
    if p.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const libc::c_char,
            b"p != ((void*)0)\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"dav1d_picture_unref_internal\0"))
                .as_ptr(),
        );
        return;
    }
    if !((*p).ref_0).is_null() {
        if ((*p).data[0 as libc::c_int as usize]).is_null() {
            fprintf(
                stderr,
                b"Input validation check '%s' failed in %s!\n\0" as *const u8
                    as *const libc::c_char,
                b"p->data[0] != ((void*)0)\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"dav1d_picture_unref_internal\0"))
                    .as_ptr(),
            );
            return;
        }
        dav1d_ref_dec(&mut (*p).ref_0);
    }
    dav1d_ref_dec(&mut (*p).seq_hdr_ref);
    dav1d_ref_dec(&mut (*p).frame_hdr_ref);
    dav1d_ref_dec(&mut (*p).m.user_data.ref_0);
    dav1d_ref_dec(&mut (*p).content_light_ref);
    dav1d_ref_dec(&mut (*p).mastering_display_ref);
    dav1d_ref_dec(&mut (*p).itut_t35_ref);
    memset(
        p as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Dav1dPicture>(),
    );
    dav1d_data_props_set_defaults(&mut (*p).m);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_thread_picture_unref(p: *mut Dav1dThreadPicture) {
    dav1d_picture_unref_internal(&mut (*p).p);
    (*p).progress = 0 as *mut atomic_uint;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_picture_get_event_flags(
    p: *const Dav1dThreadPicture,
) -> Dav1dEventFlags {
    if (*p).flags as u64 == 0 {
        return 0 as Dav1dEventFlags;
    }
    let mut flags: Dav1dEventFlags = 0 as Dav1dEventFlags;
    if (*p).flags as libc::c_uint
        & PICTURE_FLAG_NEW_SEQUENCE as libc::c_int as libc::c_uint != 0
    {
        flags = ::core::mem::transmute::<
            libc::c_uint,
            Dav1dEventFlags,
        >(
            flags as libc::c_uint
                | DAV1D_EVENT_FLAG_NEW_SEQUENCE as libc::c_int as libc::c_uint,
        );
    }
    if (*p).flags as libc::c_uint
        & PICTURE_FLAG_NEW_OP_PARAMS_INFO as libc::c_int as libc::c_uint != 0
    {
        flags = ::core::mem::transmute::<
            libc::c_uint,
            Dav1dEventFlags,
        >(
            flags as libc::c_uint
                | DAV1D_EVENT_FLAG_NEW_OP_PARAMS_INFO as libc::c_int as libc::c_uint,
        );
    }
    return flags;
}
