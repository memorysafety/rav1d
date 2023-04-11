use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
use crate::src::cdf::CdfContext;
use crate::src::msac::MsacContext;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut libc::c_void;
}









pub type pixel = uint16_t;
pub type coef = int32_t;
use crate::include::stdatomic::atomic_int;
use crate::include::stdatomic::atomic_uint;

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
    pub bd_fn: Dav1dFrameContext_bd_fn,
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
    pub frame_thread: Dav1dFrameContext_frame_thread,
    pub lf: Dav1dFrameContext_lf,
    pub task_thread: Dav1dFrameContext_task_thread,
    pub tile_thread: FrameTileThreadData,
}
use crate::src::internal::FrameTileThreadData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameContext_task_thread {
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
    pub pending_tasks: Dav1dFrameContext_task_thread_pending_tasks,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameContext_task_thread_pending_tasks {
    pub merge: atomic_int,
    pub lock: pthread_mutex_t,
    pub head: *mut Dav1dTask,
    pub tail: *mut Dav1dTask,
}
use crate::src::internal::Dav1dTask;
use crate::src::internal::TaskType;













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
    pub delayed_fg: Dav1dContext_task_thread_delayed_fg,
    pub inited: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dContext_task_thread_delayed_fg {
    pub exec: libc::c_int,
    pub cond: pthread_cond_t,
    pub in_0: *const Dav1dPicture,
    pub out: *mut Dav1dPicture,
    pub type_0: TaskType,
    pub progress: [atomic_int; 2],
    pub c2rust_unnamed: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub grain_lut_16bpc: [[[int16_t; 82]; 74]; 3],
    pub scaling_16bpc: [[uint8_t; 4096]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub grain_lut_8bpc: [[[int8_t; 82]; 74]; 3],
    pub scaling_8bpc: [[uint8_t; 256]; 3],
}
use crate::include::dav1d::picture::Dav1dPicture;
use crate::include::dav1d::headers::Dav1dITUTT35;
use crate::include::dav1d::headers::Dav1dMasteringDisplay;
use crate::include::dav1d::headers::Dav1dContentLightLevel;


use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;

use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I400;
use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::dav1d::headers::Dav1dWarpedMotionParams;













































use crate::include::dav1d::headers::Dav1dFilmGrainData;
use crate::include::dav1d::headers::Dav1dSequenceHeader;



























































use crate::include::pthread::pthread_cond_t;



#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameContext_lf {
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
use crate::src::lf_mask::Av1Filter;
use crate::src::lf_mask::Av1FilterLUT;
use crate::src::lf_mask::Av1Restoration;
use crate::src::lf_mask::Av1RestorationUnit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameContext_frame_thread {
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
    pub c2rust_unnamed: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub c2rust_unnamed: C2RustUnnamed_9,
    pub c2rust_unnamed_0: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub c2rust_unnamed: C2RustUnnamed_5,
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
pub union C2RustUnnamed_5 {
    pub c2rust_unnamed: C2RustUnnamed_8,
    pub c2rust_unnamed_0: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub mv2d: mv,
    pub matrix: [int16_t; 4],
}
use crate::src::levels::mv;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub mv: [mv; 2],
    pub wedge_idx: uint8_t,
    pub mask_sign: uint8_t,
    pub interintra_mode: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub y_mode: uint8_t,
    pub uv_mode: uint8_t,
    pub tx: uint8_t,
    pub pal_sz: [uint8_t; 2],
    pub y_angle: int8_t,
    pub uv_angle: int8_t,
    pub cfl_alpha: [int8_t; 2],
}
use crate::src::refmvs::refmvs_frame;
use crate::src::refmvs::refmvs_block;


use crate::src::refmvs::refmvs_temporal_block;
use crate::src::env::BlockContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameContext_bd_fn {
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
    pub scratch: Dav1dTaskContext_scratch,
    pub warpmv: Dav1dWarpedMotionParams,
    pub lf_mask: *mut Av1Filter,
    pub top_pre_cdef_toggle: libc::c_int,
    pub cur_sb_cdef_idx_ptr: *mut int8_t,
    pub tl_4x4_filter: Filter2d,
    pub frame_thread: Dav1dTaskContext_frame_thread,
    pub task_thread: Dav1dTaskContext_task_thread,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTaskContext_task_thread {
    pub td: thread_data,
    pub ttd: *mut TaskThreadData,
    pub fttd: *mut FrameTileThreadData,
    pub flushed: libc::c_int,
    pub die: libc::c_int,
}
use crate::src::thread_data::thread_data;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTaskContext_frame_thread {
    pub pass: libc::c_int,
}
use crate::src::levels::Filter2d;











#[derive(Copy, Clone)]
#[repr(C)]
pub union Dav1dTaskContext_scratch {
    pub c2rust_unnamed: C2RustUnnamed_16,
    pub c2rust_unnamed_0: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub c2rust_unnamed: C2RustUnnamed_14,
    pub ac: [int16_t; 1024],
    pub pal_idx: [uint8_t; 8192],
    pub pal: [[uint16_t; 8]; 3],
    pub c2rust_unnamed_0: Dav1dTaskContext_scratch_interintra_edge,
}
use crate::src::internal::Dav1dTaskContext_scratch_interintra_edge;


#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub levels: [uint8_t; 1088],
    pub c2rust_unnamed: C2RustUnnamed_15,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub pal_order: [[uint8_t; 8]; 64],
    pub pal_ctx: [uint8_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub c2rust_unnamed: C2RustUnnamed_18,
    pub c2rust_unnamed_0: C2RustUnnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_17 {
    pub emu_edge_8bpc: [uint8_t; 84160],
    pub emu_edge_16bpc: [uint16_t; 84160],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_18 {
    pub lap_8bpc: [uint8_t; 4096],
    pub lap_16bpc: [uint16_t; 4096],
    pub c2rust_unnamed: C2RustUnnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
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
    pub tile_col: refmvs_tile_range,
    pub tile_row: refmvs_tile_range,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct refmvs_tile_range {
    pub start: libc::c_int,
    pub end: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTileState {
    pub cdf: CdfContext,
    pub msac: MsacContext,
    pub tiling: Dav1dTileState_tiling,
    pub progress: [atomic_int; 2],
    pub frame_thread: [Dav1dTileState_frame_thread; 2],
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
pub struct Dav1dTileState_frame_thread {
    pub pal_idx: *mut uint8_t,
    pub cf: *mut coef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTileState_tiling {
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
    pub frame_thread: Dav1dContext_frame_thread,
    pub task_thread: TaskThreadData,
    pub segmap_pool: *mut Dav1dMemPool,
    pub refmvs_pool: *mut Dav1dMemPool,
    pub refs: [Dav1dContext_refs; 8],
    pub cdf_pool: *mut Dav1dMemPool,
    pub cdf: [CdfThreadContext; 8],
    pub dsp: [Dav1dDSPContext; 3],
    pub refmvs_dsp: Dav1dRefmvsDSPContext,
    pub intra_edge: Dav1dContext_intra_edge,
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

use crate::include::dav1d::dav1d::Dav1dLogger;
use crate::include::dav1d::dav1d::Dav1dEventFlags;


use crate::src::picture::PictureFlags;



use crate::include::dav1d::dav1d::Dav1dDecodeFrameType;




use crate::include::dav1d::dav1d::Dav1dInloopFilterType;





use crate::include::dav1d::picture::Dav1dPicAllocator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dContext_intra_edge {
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
        *mut pixel,
        ptrdiff_t,
        const_left_pixel_row,
        *const pixel,
        libc::c_int,
        libc::c_int,
        *const LooprestorationParams,
        LrEdgeFlags,
        libc::c_int,
    ) -> (),
>;
use crate::src::looprestoration::LrEdgeFlags;




#[derive(Copy, Clone)]
#[repr(C)]
pub union LooprestorationParams {
    pub filter: [[int16_t; 8]; 2],
    pub sgr: LooprestorationParams_sgr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LooprestorationParams_sgr {
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
        libc::c_int,
    ) -> (),
>;
use crate::src::cdef::CdefEdgeFlags;




pub type const_left_pixel_row_2px = *const [pixel; 2];
pub type cdef_dir_fn = Option::<
    unsafe extern "C" fn(
        *const pixel,
        ptrdiff_t,
        *mut libc::c_uint,
        libc::c_int,
    ) -> libc::c_int,
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
        *mut pixel,
        ptrdiff_t,
        *mut coef,
        libc::c_int,
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
        libc::c_int,
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
        libc::c_int,
    ) -> (),
>;
pub type entry = int16_t;
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
pub type generate_grain_uv_fn = Option::<
    unsafe extern "C" fn(
        *mut [entry; 82],
        *const [entry; 82],
        *const Dav1dFilmGrainData,
        intptr_t,
        libc::c_int,
    ) -> (),
>;
pub type generate_grain_y_fn = Option::<
    unsafe extern "C" fn(*mut [entry; 82], *const Dav1dFilmGrainData, libc::c_int) -> (),
>;
use crate::src::cdf::CdfThreadContext;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dContext_refs {
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
pub struct Dav1dContext_frame_thread {
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
use crate::src::lr_apply::LR_RESTORE_V;
use crate::src::lr_apply::LR_RESTORE_U;
use crate::src::lr_apply::LR_RESTORE_Y;

#[inline]
unsafe extern "C" fn imin(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 != 0 {
        unreachable!();
    }
    return x >> 1 as libc::c_int;
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
        .offset(
            ((stripe_h - 2 as libc::c_int) as isize * PXSTRIDE(src_stride))
                as isize,
        );
    if (*(*f).c).n_tc == 1 as libc::c_int as libc::c_uint {
        if row != 0 {
            let top: libc::c_int = (4 as libc::c_int) << sb128;
            memcpy(
                &mut *dst
                    .offset(
                        ((PXSTRIDE
                            as unsafe extern "C" fn(ptrdiff_t) -> ptrdiff_t)(dst_stride)
                            * 0 as libc::c_int as isize) as isize,
                    ) as *mut pixel as *mut libc::c_void,
                &mut *dst
                    .offset(
                        ((PXSTRIDE
                            as unsafe extern "C" fn(ptrdiff_t) -> ptrdiff_t)(dst_stride)
                            * top as isize) as isize,
                    ) as *mut pixel as *const libc::c_void,
                (dst_w << 1 as libc::c_int) as size_t,
            );
            memcpy(
                &mut *dst
                    .offset(
                        ((PXSTRIDE
                            as unsafe extern "C" fn(ptrdiff_t) -> ptrdiff_t)(dst_stride)
                            * 1 as libc::c_int as isize) as isize,
                    ) as *mut pixel as *mut libc::c_void,
                &mut *dst
                    .offset(
                        ((PXSTRIDE
                            as unsafe extern "C" fn(ptrdiff_t) -> ptrdiff_t)(dst_stride)
                            * (top + 1 as libc::c_int) as isize) as isize,
                    ) as *mut pixel as *const libc::c_void,
                (dst_w << 1 as libc::c_int) as size_t,
            );
            memcpy(
                &mut *dst
                    .offset(
                        ((PXSTRIDE
                            as unsafe extern "C" fn(ptrdiff_t) -> ptrdiff_t)(dst_stride)
                            * 2 as libc::c_int as isize) as isize,
                    ) as *mut pixel as *mut libc::c_void,
                &mut *dst
                    .offset(
                        ((PXSTRIDE
                            as unsafe extern "C" fn(ptrdiff_t) -> ptrdiff_t)(dst_stride)
                            * (top + 2 as libc::c_int) as isize) as isize,
                    ) as *mut pixel as *const libc::c_void,
                (dst_w << 1 as libc::c_int) as size_t,
            );
            memcpy(
                &mut *dst
                    .offset(
                        ((PXSTRIDE
                            as unsafe extern "C" fn(ptrdiff_t) -> ptrdiff_t)(dst_stride)
                            * 3 as libc::c_int as isize) as isize,
                    ) as *mut pixel as *mut libc::c_void,
                &mut *dst
                    .offset(
                        ((PXSTRIDE
                            as unsafe extern "C" fn(ptrdiff_t) -> ptrdiff_t)(dst_stride)
                            * (top + 3 as libc::c_int) as isize) as isize,
                    ) as *mut pixel as *const libc::c_void,
                (dst_w << 1 as libc::c_int) as size_t,
            );
        }
        dst = dst
            .offset(4 * PXSTRIDE(dst_stride));
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
                (*f).bitdepth_max,
            );
            row += stripe_h;
            stripe_h = 64 as libc::c_int >> ss_ver;
            src = src.offset((stripe_h as isize * PXSTRIDE(src_stride)) as isize);
            dst = dst.offset((n_lines as isize * PXSTRIDE(dst_stride)) as isize);
            if n_lines == 3 as libc::c_int {
                memcpy(
                    dst as *mut libc::c_void,
                    &mut *dst
                        .offset(
                            -(PXSTRIDE
                                as unsafe extern "C" fn(ptrdiff_t) -> ptrdiff_t)(dst_stride)
                                as isize,
                        ) as *mut pixel as *const libc::c_void,
                    (dst_w << 1 as libc::c_int) as size_t,
                );
                dst = dst.offset(PXSTRIDE(dst_stride) as isize);
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
                        &mut *dst
                            .offset(
                                -(PXSTRIDE
                                    as unsafe extern "C" fn(ptrdiff_t) -> ptrdiff_t)(dst_stride)
                                    as isize,
                            ) as *mut pixel as *const pixel
                    } else {
                        src
                    }) as *const libc::c_void,
                    (src_w << 1 as libc::c_int) as size_t,
                );
                dst = dst.offset(PXSTRIDE(dst_stride) as isize);
                src = src.offset(PXSTRIDE(src_stride) as isize);
                i += 1;
            }
            row += stripe_h;
            stripe_h = 64 as libc::c_int >> ss_ver;
            src = src
                .offset(
                    ((stripe_h - 4 as libc::c_int) as isize
                        * PXSTRIDE(src_stride)) as isize,
                );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_copy_lpf_16bpc(
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
                (tt_off as isize
                    * PXSTRIDE(*lr_stride.offset(0 as libc::c_int as isize))) as isize,
            ),
        ((*f).lf.lr_lpf_line[1 as libc::c_int as usize])
            .offset(
                (tt_off as isize
                    * PXSTRIDE(*lr_stride.offset(1 as libc::c_int as isize))) as isize,
            ),
        ((*f).lf.lr_lpf_line[2 as libc::c_int as usize])
            .offset(
                (tt_off as isize
                    * PXSTRIDE(*lr_stride.offset(1 as libc::c_int as isize))) as isize,
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
                        -((offset as isize
                            * PXSTRIDE(*src_stride.offset(0 as libc::c_int as isize)))
                            as isize),
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
            let cdef_off_y: ptrdiff_t = (sby * 4 as libc::c_int) as isize
                * PXSTRIDE(*src_stride.offset(0 as libc::c_int as isize));
            backup_lpf(
                f,
                ((*f).lf.cdef_lpf_line[0 as libc::c_int as usize])
                    .offset(cdef_off_y as isize),
                *src_stride.offset(0 as libc::c_int as isize),
                (*src.offset(0 as libc::c_int as isize))
                    .offset(
                        -((offset as isize
                            * PXSTRIDE(*src_stride.offset(0 as libc::c_int as isize)))
                            as isize),
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
        let cdef_off_uv: ptrdiff_t = sby as isize * 4
            * PXSTRIDE(*src_stride.offset(1 as libc::c_int as isize));
        if (*(*f).seq_hdr).cdef != 0 || restore_planes & LR_RESTORE_U as libc::c_int != 0
        {
            if restore_planes & LR_RESTORE_U as libc::c_int != 0 || resize == 0 {
                backup_lpf(
                    f,
                    dst[1 as libc::c_int as usize],
                    *lr_stride.offset(1 as libc::c_int as isize),
                    (*src.offset(1 as libc::c_int as isize))
                        .offset(
                            -(offset_uv as isize * PXSTRIDE(*src_stride.offset(1 as libc::c_int as isize))),
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
                            -(offset_uv as isize * PXSTRIDE(*src_stride.offset(1 as libc::c_int as isize))),
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
                            -(offset_uv as isize * PXSTRIDE(*src_stride.offset(1 as libc::c_int as isize))),
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
                            -(offset_uv as isize
                                * PXSTRIDE(*src_stride.offset(1 as libc::c_int as isize))),
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
                (*f).bitdepth_max,
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
                (*f).bitdepth_max,
            );
        }
        y += 1;
        dst = dst.offset(4 * PXSTRIDE(ls));
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
                (*f).bitdepth_max,
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
                (*f).bitdepth_max,
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
                (*f).bitdepth_max,
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
                (*f).bitdepth_max,
            );
        }
        y += 1;
        off_l += 4 * PXSTRIDE(ls);
        lvl = lvl.offset(b4_stride as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_loopfilter_sbrow_cols_16bpc(
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
        .offset((*f).b4_stride * sby as isize * sbsz as isize);
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
        .offset((*f).b4_stride * (sby * sbsz >> ss_ver) as isize);
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
        uv_off += 128 >> ss_hor;
        level_ptr = level_ptr.offset((32 as libc::c_int >> ss_hor) as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_loopfilter_sbrow_rows_16bpc(
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
        .offset((*f).b4_stride * sby as isize * sbsz as isize);
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
        .offset((*f).b4_stride * (sby * sbsz >> ss_ver) as isize);
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
        uv_off += 128 >> ss_hor;
        level_ptr = level_ptr.offset((32 as libc::c_int >> ss_hor) as isize);
    }
}
