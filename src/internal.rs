use crate::include::common::bitdepth::DynCoef;
use crate::include::common::bitdepth::DynPixel;
use crate::include::dav1d::common::Rav1dDataProps;
use crate::include::dav1d::data::Rav1dData;
use crate::include::dav1d::dav1d::Rav1dDecodeFrameType;
use crate::include::dav1d::dav1d::Rav1dEventFlags;
use crate::include::dav1d::dav1d::Rav1dInloopFilterType;
use crate::include::dav1d::dav1d::Rav1dLogger;
use crate::include::dav1d::headers::Rav1dContentLightLevel;
use crate::include::dav1d::headers::Rav1dFrameHeader;
use crate::include::dav1d::headers::Rav1dITUTT35;
use crate::include::dav1d::headers::Rav1dMasteringDisplay;
use crate::include::dav1d::headers::Rav1dSequenceHeader;
use crate::include::dav1d::headers::Rav1dWarpedMotionParams;
use crate::include::dav1d::picture::Rav1dPicAllocator;
use crate::include::dav1d::picture::Rav1dPicture;
use crate::include::stdatomic::atomic_int;
use crate::include::stdatomic::atomic_uint;
use crate::src::align::*;
use crate::src::cdef::Rav1dCdefDSPContext;
use crate::src::cdf::CdfContext;
use crate::src::cdf::CdfThreadContext;
use crate::src::env::BlockContext;
use crate::src::error::Rav1dResult;
use crate::src::filmgrain::Rav1dFilmGrainDSPContext;
use crate::src::intra_edge::EdgeBranch;
use crate::src::intra_edge::EdgeFlags;
use crate::src::intra_edge::EdgeNode;
use crate::src::intra_edge::EdgeTip;
use crate::src::ipred::Rav1dIntraPredDSPContext;
use crate::src::itx::Rav1dInvTxfmDSPContext;
use crate::src::levels::Av1Block;
use crate::src::levels::BlockSize;
use crate::src::levels::Filter2d;
use crate::src::lf_mask::Av1Filter;
use crate::src::lf_mask::Av1FilterLUT;
use crate::src::lf_mask::Av1Restoration;
use crate::src::lf_mask::Av1RestorationUnit;
use crate::src::loopfilter::Rav1dLoopFilterDSPContext;
use crate::src::looprestoration::Rav1dLoopRestorationDSPContext;
use crate::src::mc::Rav1dMCDSPContext;
use crate::src::mem::Rav1dMemPool;
use crate::src::msac::MsacContext;
use crate::src::picture::PictureFlags;
use crate::src::picture::Rav1dThreadPicture;
use crate::src::r#ref::Rav1dRef;
use crate::src::recon::backup_ipred_edge_fn;
use crate::src::recon::filter_sbrow_fn;
use crate::src::recon::read_coef_blocks_fn;
use crate::src::recon::recon_b_inter_fn;
use crate::src::recon::recon_b_intra_fn;
use crate::src::refmvs::refmvs_frame;
use crate::src::refmvs::refmvs_temporal_block;
use crate::src::refmvs::refmvs_tile;
use crate::src::refmvs::Rav1dRefmvsDSPContext;
use crate::src::thread_data::thread_data;
use libc::pthread_cond_t;
use libc::pthread_mutex_t;
use libc::ptrdiff_t;
use std::ffi::c_int;
use std::ffi::c_uint;

#[repr(C)]
pub struct Rav1dDSPContext {
    pub fg: Rav1dFilmGrainDSPContext,
    pub ipred: Rav1dIntraPredDSPContext,
    pub mc: Rav1dMCDSPContext,
    pub itx: Rav1dInvTxfmDSPContext,
    pub lf: Rav1dLoopFilterDSPContext,
    pub cdef: Rav1dCdefDSPContext,
    pub lr: Rav1dLoopRestorationDSPContext,
}

#[derive(Clone, Default)]
#[repr(C)]
pub(crate) struct Rav1dTileGroup {
    pub data: Rav1dData,
    pub start: c_int,
    pub end: c_int,
}

pub type TaskType = c_uint;
pub const RAV1D_TASK_TYPE_FG_APPLY: TaskType = 12;
pub const RAV1D_TASK_TYPE_FG_PREP: TaskType = 11;
pub const RAV1D_TASK_TYPE_RECONSTRUCTION_PROGRESS: TaskType = 10;
pub const RAV1D_TASK_TYPE_LOOP_RESTORATION: TaskType = 9;
pub const RAV1D_TASK_TYPE_SUPER_RESOLUTION: TaskType = 8;
pub const RAV1D_TASK_TYPE_CDEF: TaskType = 7;
pub const RAV1D_TASK_TYPE_DEBLOCK_ROWS: TaskType = 6;
pub const RAV1D_TASK_TYPE_DEBLOCK_COLS: TaskType = 5;
pub const RAV1D_TASK_TYPE_TILE_RECONSTRUCTION: TaskType = 4;
pub const RAV1D_TASK_TYPE_ENTROPY_PROGRESS: TaskType = 3;
pub const RAV1D_TASK_TYPE_TILE_ENTROPY: TaskType = 2;
pub const RAV1D_TASK_TYPE_INIT_CDF: TaskType = 1;
pub const RAV1D_TASK_TYPE_INIT: TaskType = 0;

#[repr(C)]
pub(crate) struct Rav1dContext_frame_thread {
    pub out_delayed: *mut Rav1dThreadPicture,
    pub next: c_uint,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct TaskThreadData_grain_lut_scaling_8 {
    pub grain_lut_8bpc: Align16<[[[i8; 82]; 74]; 3]>,
    pub scaling_8bpc: Align64<[[u8; 256]; 3]>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct TaskThreadData_grain_lut_scaling_16 {
    pub grain_lut_16bpc: Align16<[[[i16; 82]; 74]; 3]>,
    pub scaling_16bpc: Align64<[[u8; 4096]; 3]>,
}

#[repr(C)]
pub union TaskThreadData_grain_lut_scaling {
    pub c2rust_unnamed: TaskThreadData_grain_lut_scaling_8,
    pub c2rust_unnamed_0: TaskThreadData_grain_lut_scaling_16,
}

#[repr(C)]
pub(crate) struct TaskThreadData_delayed_fg {
    pub exec: c_int,
    pub cond: pthread_cond_t,
    pub in_0: *const Rav1dPicture,
    pub out: *mut Rav1dPicture,
    pub type_0: TaskType,
    pub progress: [atomic_int; 2],
    pub c2rust_unnamed: TaskThreadData_grain_lut_scaling,
}

#[repr(C)]
pub(crate) struct TaskThreadData {
    pub lock: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub first: atomic_uint,
    pub cur: c_uint,
    pub reset_task_cur: atomic_uint,
    pub cond_signaled: atomic_int,
    pub delayed_fg: TaskThreadData_delayed_fg,
    pub inited: c_int,
}

#[repr(C)]
pub(crate) struct Rav1dContext_refs {
    pub p: Rav1dThreadPicture,
    pub segmap: *mut Rav1dRef,
    pub refmvs: *mut Rav1dRef,
    pub refpoc: [c_uint; 7],
}

#[repr(C)]
pub struct Rav1dContext_intra_edge {
    pub root: [*mut EdgeNode; 2],
    pub branch_sb128: [EdgeBranch; 85],
    pub branch_sb64: [EdgeBranch; 21],
    pub tip_sb128: [EdgeTip; 256],
    pub tip_sb64: [EdgeTip; 64],
}

#[repr(C)]
pub struct Rav1dContext {
    pub(crate) fc: *mut Rav1dFrameContext,
    pub(crate) n_fc: c_uint,
    pub(crate) tc: *mut Rav1dTaskContext,
    pub(crate) n_tc: c_uint,
    pub(crate) tile: *mut Rav1dTileGroup,
    pub(crate) n_tile_data_alloc: c_int,
    pub(crate) n_tile_data: c_int,
    pub(crate) n_tiles: c_int,
    pub(crate) seq_hdr_pool: *mut Rav1dMemPool,
    pub(crate) seq_hdr_ref: *mut Rav1dRef,
    pub(crate) seq_hdr: *mut Rav1dSequenceHeader,
    pub(crate) frame_hdr_pool: *mut Rav1dMemPool,
    pub(crate) frame_hdr_ref: *mut Rav1dRef,
    pub(crate) frame_hdr: *mut Rav1dFrameHeader,
    pub(crate) content_light_ref: *mut Rav1dRef,
    pub(crate) content_light: *mut Rav1dContentLightLevel,
    pub(crate) mastering_display_ref: *mut Rav1dRef,
    pub(crate) mastering_display: *mut Rav1dMasteringDisplay,
    pub(crate) itut_t35_ref: *mut Rav1dRef,
    pub(crate) itut_t35: *mut Rav1dITUTT35,
    pub(crate) in_0: Rav1dData,
    pub(crate) out: Rav1dThreadPicture,
    pub(crate) cache: Rav1dThreadPicture,
    pub(crate) flush_mem: atomic_int,
    pub(crate) flush: *mut atomic_int,
    pub(crate) frame_thread: Rav1dContext_frame_thread,
    pub(crate) task_thread: TaskThreadData,
    pub(crate) segmap_pool: *mut Rav1dMemPool,
    pub(crate) refmvs_pool: *mut Rav1dMemPool,
    pub(crate) refs: [Rav1dContext_refs; 8],
    pub(crate) cdf_pool: *mut Rav1dMemPool,
    pub(crate) cdf: [CdfThreadContext; 8],
    pub(crate) dsp: [Rav1dDSPContext; 3],
    pub(crate) refmvs_dsp: Rav1dRefmvsDSPContext,
    pub(crate) intra_edge: Rav1dContext_intra_edge,
    pub(crate) allocator: Rav1dPicAllocator,
    pub(crate) apply_grain: c_int,
    pub(crate) operating_point: c_int,
    pub(crate) operating_point_idc: c_uint,
    pub(crate) all_layers: c_int,
    pub(crate) max_spatial_id: c_int,
    pub(crate) frame_size_limit: c_uint,
    pub(crate) strict_std_compliance: c_int,
    pub(crate) output_invisible_frames: c_int,
    pub(crate) inloop_filters: Rav1dInloopFilterType,
    pub(crate) decode_frame_type: Rav1dDecodeFrameType,
    pub(crate) drain: c_int,
    pub(crate) frame_flags: PictureFlags,
    pub(crate) event_flags: Rav1dEventFlags,
    pub(crate) cached_error_props: Rav1dDataProps,
    pub(crate) cached_error: Rav1dResult,
    pub(crate) logger: Rav1dLogger,
    pub(crate) picture_pool: *mut Rav1dMemPool,
}

#[derive(Clone)]
#[repr(C)]
pub struct Rav1dTask {
    pub frame_idx: c_uint,
    pub type_0: TaskType,
    pub sby: c_int,
    pub recon_progress: c_int,
    pub deblock_progress: c_int,
    pub deps_skip: c_int,
    pub next: *mut Rav1dTask,
}

#[repr(C)]
pub struct ScalableMotionParams {
    pub scale: c_int,
    pub step: c_int,
}

#[repr(C)]
pub(crate) struct Rav1dFrameContext_bd_fn {
    pub recon_b_intra: recon_b_intra_fn,
    pub recon_b_inter: recon_b_inter_fn,
    pub filter_sbrow: filter_sbrow_fn,
    pub filter_sbrow_deblock_cols: filter_sbrow_fn,
    pub filter_sbrow_deblock_rows: filter_sbrow_fn,
    pub filter_sbrow_cdef: Option<unsafe extern "C" fn(*mut Rav1dTaskContext, c_int) -> ()>,
    pub filter_sbrow_resize: filter_sbrow_fn,
    pub filter_sbrow_lr: filter_sbrow_fn,
    pub backup_ipred_edge: backup_ipred_edge_fn,
    pub read_coef_blocks: read_coef_blocks_fn,
}

impl Rav1dFrameContext_bd_fn {
    pub unsafe fn recon_b_intra(
        &self,
        context: *mut Rav1dTaskContext,
        block_size: BlockSize,
        flags: EdgeFlags,
        block: *const Av1Block,
    ) {
        self.recon_b_intra.expect("non-null function pointer")(context, block_size, flags, block);
    }

    pub unsafe fn recon_b_inter(
        &self,
        context: *mut Rav1dTaskContext,
        block_size: BlockSize,
        block: *const Av1Block,
    ) -> c_int {
        self.recon_b_inter.expect("non-null function pointer")(context, block_size, block)
    }

    pub unsafe fn read_coef_blocks(
        &self,
        context: *mut Rav1dTaskContext,
        block_size: BlockSize,
        block: *const Av1Block,
    ) {
        self.read_coef_blocks.expect("non-null function pointer")(context, block_size, block);
    }
}

#[repr(C)]
pub struct CodedBlockInfo {
    pub eob: [i16; 3],
    pub txtp: [u8; 3],
}

#[repr(C)]
pub struct Rav1dFrameContext_frame_thread {
    pub next_tile_row: [c_int; 2],
    pub entropy_progress: atomic_int,
    pub deblock_progress: atomic_int,
    pub frame_progress: *mut atomic_uint,
    pub copy_lpf_progress: *mut atomic_uint,
    pub b: *mut Av1Block,
    pub cbi: *mut CodedBlockInfo,
    pub pal: *mut [[u16; 8]; 3],
    pub pal_idx: *mut u8,
    pub cf: *mut DynCoef,
    pub prog_sz: c_int,
    pub pal_sz: c_int,
    pub pal_idx_sz: c_int,
    pub cf_sz: c_int,
    pub tile_start_off: *mut c_int,
}

#[repr(C)]
pub struct Rav1dFrameContext_lf {
    pub level: *mut [u8; 4],
    pub mask: *mut Av1Filter,
    pub lr_mask: *mut Av1Restoration,
    pub mask_sz: c_int,
    pub lr_mask_sz: c_int,
    pub cdef_buf_plane_sz: [c_int; 2],
    pub cdef_buf_sbh: c_int,
    pub lr_buf_plane_sz: [c_int; 2],
    pub re_sz: c_int,
    pub lim_lut: Align16<Av1FilterLUT>,
    pub last_sharpness: c_int,
    pub lvl: [[[[u8; 2]; 8]; 4]; 8],
    pub tx_lpf_right_edge: [*mut u8; 2],
    pub cdef_line_buf: *mut u8,
    pub lr_line_buf: *mut u8,
    pub cdef_line: [[*mut DynPixel; 3]; 2],
    pub cdef_lpf_line: [*mut DynPixel; 3],
    pub lr_lpf_line: [*mut DynPixel; 3],
    pub start_of_tile_row: *mut u8,
    pub start_of_tile_row_sz: c_int,
    pub need_cdef_lpf_copy: c_int,
    pub p: [*mut DynPixel; 3],
    pub sr_p: [*mut DynPixel; 3],
    pub mask_ptr: *mut Av1Filter,
    pub prev_mask_ptr: *mut Av1Filter,
    pub restore_planes: c_int,
}

#[repr(C)]
pub struct Rav1dFrameContext_task_thread_pending_tasks {
    pub merge: atomic_int,
    pub lock: pthread_mutex_t,
    pub head: *mut Rav1dTask,
    pub tail: *mut Rav1dTask,
}

#[repr(C)]
pub(crate) struct Rav1dFrameContext_task_thread {
    pub lock: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub ttd: *mut TaskThreadData,
    pub tasks: *mut Rav1dTask,
    pub tile_tasks: [*mut Rav1dTask; 2],
    pub init_task: Rav1dTask,
    pub num_tasks: c_int,
    pub num_tile_tasks: c_int,
    pub init_done: atomic_int,
    pub done: [atomic_int; 2],
    pub retval: Rav1dResult,
    pub update_set: bool,
    pub error: atomic_int,
    pub task_counter: atomic_int,
    pub task_head: *mut Rav1dTask,
    pub task_tail: *mut Rav1dTask,
    pub task_cur_prev: *mut Rav1dTask,
    pub pending_tasks: Rav1dFrameContext_task_thread_pending_tasks,
}

#[repr(C)]
pub struct FrameTileThreadData {
    pub lowest_pixel_mem: *mut [[c_int; 2]; 7],
    pub lowest_pixel_mem_sz: c_int,
}

#[repr(C)]
pub(crate) struct Rav1dFrameContext {
    pub seq_hdr_ref: *mut Rav1dRef,
    pub seq_hdr: *mut Rav1dSequenceHeader,
    pub frame_hdr_ref: *mut Rav1dRef,
    pub frame_hdr: *mut Rav1dFrameHeader,
    pub refp: [Rav1dThreadPicture; 7],
    pub cur: Rav1dPicture,
    pub sr_cur: Rav1dThreadPicture,
    pub mvs_ref: *mut Rav1dRef,
    pub mvs: *mut refmvs_temporal_block,
    pub ref_mvs: [*mut refmvs_temporal_block; 7],
    pub ref_mvs_ref: [*mut Rav1dRef; 7],
    pub cur_segmap_ref: *mut Rav1dRef,
    pub prev_segmap_ref: *mut Rav1dRef,
    pub cur_segmap: *mut u8,
    pub prev_segmap: *const u8,
    pub refpoc: [c_uint; 7],
    pub refrefpoc: [[c_uint; 7]; 7],
    pub gmv_warp_allowed: [u8; 7],
    pub in_cdf: CdfThreadContext,
    pub out_cdf: CdfThreadContext,
    pub tile: *mut Rav1dTileGroup,
    pub n_tile_data_alloc: c_int,
    pub n_tile_data: c_int,
    pub svc: [[ScalableMotionParams; 2]; 7],
    pub resize_step: [c_int; 2],
    pub resize_start: [c_int; 2],
    pub c: *const Rav1dContext,
    pub ts: *mut Rav1dTileState,
    pub n_ts: c_int,
    pub dsp: *const Rav1dDSPContext,
    pub bd_fn: Rav1dFrameContext_bd_fn,
    pub ipred_edge_sz: c_int,
    pub ipred_edge: [*mut DynPixel; 3],
    pub b4_stride: ptrdiff_t,
    pub w4: c_int,
    pub h4: c_int,
    pub bw: c_int,
    pub bh: c_int,
    pub sb128w: c_int,
    pub sb128h: c_int,
    pub sbh: c_int,
    pub sb_shift: c_int,
    pub sb_step: c_int,
    pub sr_sb128w: c_int,
    pub dq: [[[u16; 2]; 3]; 8],
    pub qm: [[*const u8; 3]; 19],
    pub a: *mut BlockContext,
    pub a_sz: c_int,
    pub rf: refmvs_frame,
    pub jnt_weights: [[u8; 7]; 7],
    pub bitdepth_max: c_int,
    pub frame_thread: Rav1dFrameContext_frame_thread,
    pub lf: Rav1dFrameContext_lf,
    pub task_thread: Rav1dFrameContext_task_thread,
    pub tile_thread: FrameTileThreadData,
}

#[repr(C)]
pub struct Rav1dTileState_tiling {
    pub col_start: c_int,
    pub col_end: c_int,
    pub row_start: c_int,
    pub row_end: c_int,
    pub col: c_int,
    pub row: c_int,
}

#[repr(C)]
pub struct Rav1dTileState_frame_thread {
    pub pal_idx: *mut u8,
    pub cf: *mut DynCoef,
}

#[repr(C)]
pub struct Rav1dTileState {
    pub cdf: CdfContext,
    pub msac: MsacContext,
    pub tiling: Rav1dTileState_tiling,
    pub progress: [atomic_int; 2],
    pub frame_thread: [Rav1dTileState_frame_thread; 2],
    pub lowest_pixel: *mut [[c_int; 2]; 7],
    pub dqmem: [[[u16; 2]; 3]; 8],
    pub dq: *const [[u16; 2]; 3],
    pub last_qidx: c_int,
    pub last_delta_lf: [i8; 4],
    pub lflvlmem: [[[[u8; 2]; 8]; 4]; 8],
    pub lflvl: *const [[[u8; 2]; 8]; 4],
    pub lr_ref: [*mut Av1RestorationUnit; 3],
}

#[repr(C, align(64))]
pub union Rav1dTaskContext_cf {
    pub cf_8bpc: [i16; 1024],
    pub cf_16bpc: [i32; 1024],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Rav1dTaskContext_scratch_compinter_seg_mask {
    pub compinter: [[i16; 16384]; 2],
    pub seg_mask: [u8; 16384],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union Rav1dTaskContext_scratch_lap {
    pub lap_8bpc: [u8; 4096],
    pub lap_16bpc: [u16; 4096],
    pub c2rust_unnamed: Rav1dTaskContext_scratch_compinter_seg_mask,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union Rav1dTaskContext_scratch_emu_edge {
    pub emu_edge_8bpc: [u8; 84160],
    pub emu_edge_16bpc: [u16; 84160],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Rav1dTaskContext_scratch_lap_emu_edge {
    pub c2rust_unnamed: Rav1dTaskContext_scratch_lap,
    pub c2rust_unnamed_0: Rav1dTaskContext_scratch_emu_edge,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Rav1dTaskContext_scratch_pal {
    pub pal_order: [[u8; 8]; 64],
    pub pal_ctx: [u8; 64],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union Rav1dTaskContext_scratch_levels_pal {
    pub levels: [u8; 1088],
    pub c2rust_unnamed: Rav1dTaskContext_scratch_pal,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Rav1dTaskContext_scratch_interintra_edge_8 {
    pub interintra_8bpc: [u8; 4096],
    pub edge_8bpc: [u8; 257],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Rav1dTaskContext_scratch_interintra_edge_16 {
    pub interintra_16bpc: [u16; 4096],
    pub edge_16bpc: [u16; 257],
}

#[derive(Clone, Copy)]
#[repr(C, align(64))]
pub union Rav1dTaskContext_scratch_interintra_edge {
    pub c2rust_unnamed: Rav1dTaskContext_scratch_interintra_edge_8,
    pub c2rust_unnamed_0: Rav1dTaskContext_scratch_interintra_edge_16,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Rav1dTaskContext_scratch_levels_pal_ac_interintra_edge {
    pub c2rust_unnamed: Rav1dTaskContext_scratch_levels_pal,
    pub ac: [i16; 1024],
    pub pal_idx: [u8; 8192],
    pub pal: [[u16; 8]; 3],
    pub c2rust_unnamed_0: Rav1dTaskContext_scratch_interintra_edge,
}

#[repr(C, align(64))]
pub union Rav1dTaskContext_scratch {
    pub c2rust_unnamed: Rav1dTaskContext_scratch_lap_emu_edge,
    pub c2rust_unnamed_0: Rav1dTaskContext_scratch_levels_pal_ac_interintra_edge,
}

#[repr(C)]
pub struct Rav1dTaskContext_frame_thread {
    pub pass: c_int,
}

#[repr(C)]
pub(crate) struct Rav1dTaskContext_task_thread {
    pub td: thread_data,
    pub ttd: *mut TaskThreadData,
    pub fttd: *mut FrameTileThreadData,
    pub flushed: bool,
    pub die: bool,
}

#[repr(C)]
pub(crate) struct Rav1dTaskContext {
    pub c: *const Rav1dContext,
    pub f: *const Rav1dFrameContext,
    pub ts: *mut Rav1dTileState,
    pub bx: c_int,
    pub by: c_int,
    pub l: BlockContext,
    pub a: *mut BlockContext,
    pub rt: refmvs_tile,
    pub c2rust_unnamed: Rav1dTaskContext_cf,
    pub al_pal: [[[[u16; 8]; 3]; 32]; 2],
    pub pal_sz_uv: [[u8; 32]; 2],
    pub txtp_map: [u8; 1024],
    pub scratch: Rav1dTaskContext_scratch,
    pub warpmv: Rav1dWarpedMotionParams,
    pub lf_mask: *mut Av1Filter,
    pub top_pre_cdef_toggle: c_int,
    pub cur_sb_cdef_idx_ptr: *mut i8,
    pub tl_4x4_filter: Filter2d,
    pub frame_thread: Rav1dTaskContext_frame_thread,
    pub task_thread: Rav1dTaskContext_task_thread,
}
