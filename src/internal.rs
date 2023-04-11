use crate::include::dav1d::data::Dav1dData;
use crate::include::dav1d::picture::Dav1dPicture;
use crate::include::pthread::pthread_cond_t;
use crate::include::pthread::pthread_mutex_t;
use crate::include::stdatomic::atomic_int;
use crate::include::stdatomic::atomic_uint;
use crate::include::stdint::int16_t;
use crate::include::stdint::int32_t;
use crate::include::stdint::int8_t;
use crate::include::stdint::uint16_t;
use crate::include::stdint::uint8_t;
use crate::src::picture::Dav1dThreadPicture;
use crate::src::thread_data::thread_data;

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
    pub grain_lut_8bpc: [[[int8_t; 82]; 74]; 3],
    pub scaling_8bpc: [[uint8_t; 256]; 3],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TaskThreadData_grain_lut_scaling_16 {
    pub grain_lut_16bpc: [[[int16_t; 82]; 74]; 3],
    pub scaling_16bpc: [[uint8_t; 4096]; 3],
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
#[repr(C, align(64))]
pub union Dav1dTaskContext_cf {
    pub cf_8bpc: [int16_t; 1024],
    pub cf_16bpc: [int32_t; 1024],
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
