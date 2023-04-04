use crate::include::dav1d::data::Dav1dData;
use crate::include::stdint::int16_t;
use crate::include::stdint::int32_t;
use crate::include::stdint::uint8_t;

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
pub struct FrameTileThreadData {
    pub lowest_pixel_mem: *mut [[libc::c_int; 2]; 7],
    pub lowest_pixel_mem_sz: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union Dav1dTaskContext_cf {
    pub cf_8bpc: [int16_t; 1024],
    pub cf_16bpc: [int32_t; 1024],
}
