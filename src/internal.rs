use crate::include::dav1d::data::Dav1dData;
use crate::include::dav1d::picture::Dav1dPicture;
use crate::include::pthread::pthread_cond_t;
use crate::include::stdatomic::atomic_int;
use crate::include::stdatomic::atomic_uint;
use crate::include::stdint::int16_t;
use crate::include::stdint::int32_t;
use crate::include::stdint::int8_t;
use crate::include::stdint::uint16_t;
use crate::include::stdint::uint8_t;
use crate::src::align::*;
use crate::src::cdf::CdfContext;
use crate::src::intra_edge::EdgeBranch;
use crate::src::intra_edge::EdgeNode;
use crate::src::intra_edge::EdgeTip;
use crate::src::levels::Av1Block;
use crate::src::lf_mask::Av1Filter;
use crate::src::lf_mask::Av1FilterLUT;
use crate::src::lf_mask::Av1Restoration;
use crate::src::lf_mask::Av1RestorationUnit;
use crate::src::msac::MsacContext;
use crate::src::picture::Dav1dThreadPicture;
use crate::src::r#ref::Dav1dRef;
use crate::src::thread_data::thread_data;
use libc::pthread_mutex_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTileGroup {
    pub data: Dav1dData,
    pub start: libc::c_int,
    pub end: libc::c_int,
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
pub struct Dav1dContext_frame_thread {
    pub out_delayed: *mut Dav1dThreadPicture,
    pub next: libc::c_uint,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TaskThreadData_grain_lut_scaling_8 {
    pub grain_lut_8bpc: Align16<[[[int8_t; 82]; 74]; 3]>,
    pub scaling_8bpc: Align64<[[uint8_t; 256]; 3]>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TaskThreadData_grain_lut_scaling_16 {
    pub grain_lut_16bpc: Align16<[[[int16_t; 82]; 74]; 3]>,
    pub scaling_16bpc: Align64<[[uint8_t; 4096]; 3]>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union TaskThreadData_grain_lut_scaling {
    pub c2rust_unnamed: TaskThreadData_grain_lut_scaling_8,
    pub c2rust_unnamed_0: TaskThreadData_grain_lut_scaling_16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TaskThreadData_delayed_fg {
    pub exec: libc::c_int,
    pub cond: pthread_cond_t,
    pub in_0: *const Dav1dPicture,
    pub out: *mut Dav1dPicture,
    pub type_0: TaskType,
    pub progress: [atomic_int; 2],
    pub c2rust_unnamed: TaskThreadData_grain_lut_scaling,
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
    pub delayed_fg: TaskThreadData_delayed_fg,
    pub inited: libc::c_int,
}

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
pub struct Dav1dContext_intra_edge {
    pub root: [*mut EdgeNode; 2],
    pub branch_sb128: [EdgeBranch; 85],
    pub branch_sb64: [EdgeBranch; 21],
    pub tip_sb128: [EdgeTip; 256],
    pub tip_sb64: [EdgeTip; 64],
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScalableMotionParams {
    pub scale: libc::c_int,
    pub step: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CodedBlockInfo {
    pub eob: [int16_t; 3],
    pub txtp: [uint8_t; 3],
}

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
    pub cf: *mut libc::c_void,
    pub prog_sz: libc::c_int,
    pub pal_sz: libc::c_int,
    pub pal_idx_sz: libc::c_int,
    pub cf_sz: libc::c_int,
    pub tile_start_off: *mut libc::c_int,
}

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
    pub lim_lut: Align16<Av1FilterLUT>,
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
pub struct Dav1dFrameContext_task_thread_pending_tasks {
    pub merge: atomic_int,
    pub lock: pthread_mutex_t,
    pub head: *mut Dav1dTask,
    pub tail: *mut Dav1dTask,
}

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
pub struct FrameTileThreadData {
    pub lowest_pixel_mem: *mut [[libc::c_int; 2]; 7],
    pub lowest_pixel_mem_sz: libc::c_int,
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
pub struct Dav1dTileState_frame_thread {
    pub pal_idx: *mut uint8_t,
    pub cf: *mut libc::c_void,
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
#[repr(C, align(64))]
pub union Dav1dTaskContext_cf {
    pub cf_8bpc: [int16_t; 1024],
    pub cf_16bpc: [int32_t; 1024],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTaskContext_scratch_compinter_seg_mask {
    pub compinter: [[int16_t; 16384]; 2],
    pub seg_mask: [uint8_t; 16384],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union Dav1dTaskContext_scratch_lap {
    pub lap_8bpc: [uint8_t; 4096],
    pub lap_16bpc: [uint16_t; 4096],
    pub c2rust_unnamed: Dav1dTaskContext_scratch_compinter_seg_mask,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union Dav1dTaskContext_scratch_emu_edge {
    pub emu_edge_8bpc: [uint8_t; 84160],
    pub emu_edge_16bpc: [uint16_t; 84160],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTaskContext_scratch_lap_emu_edge {
    pub c2rust_unnamed: Dav1dTaskContext_scratch_lap,
    pub c2rust_unnamed_0: Dav1dTaskContext_scratch_emu_edge,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTaskContext_scratch_pal {
    pub pal_order: [[uint8_t; 8]; 64],
    pub pal_ctx: [uint8_t; 64],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union Dav1dTaskContext_scratch_levels_pal {
    pub levels: [uint8_t; 1088],
    pub c2rust_unnamed: Dav1dTaskContext_scratch_pal,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTaskContext_scratch_interintra_edge_8 {
    pub interintra_8bpc: [uint8_t; 4096],
    pub edge_8bpc: [uint8_t; 257],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTaskContext_scratch_interintra_edge_16 {
    pub interintra_16bpc: [uint16_t; 4096],
    pub edge_16bpc: [uint16_t; 257],
}

#[derive(Copy, Clone)]
#[repr(C, align(64))]
pub union Dav1dTaskContext_scratch_interintra_edge {
    pub c2rust_unnamed: Dav1dTaskContext_scratch_interintra_edge_8,
    pub c2rust_unnamed_0: Dav1dTaskContext_scratch_interintra_edge_16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTaskContext_scratch_levels_pal_ac_interintra_edge {
    pub c2rust_unnamed: Dav1dTaskContext_scratch_levels_pal,
    pub ac: [int16_t; 1024],
    pub pal_idx: [uint8_t; 8192],
    pub pal: [[uint16_t; 8]; 3],
    pub c2rust_unnamed_0: Dav1dTaskContext_scratch_interintra_edge,
}

#[derive(Copy, Clone)]
#[repr(C, align(64))]
pub union Dav1dTaskContext_scratch {
    pub c2rust_unnamed: Dav1dTaskContext_scratch_lap_emu_edge,
    pub c2rust_unnamed_0: Dav1dTaskContext_scratch_levels_pal_ac_interintra_edge,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTaskContext_frame_thread {
    pub pass: libc::c_int,
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
