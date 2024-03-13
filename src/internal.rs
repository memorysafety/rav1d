use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::BitDepth16;
use crate::include::common::bitdepth::BitDepth8;
use crate::include::common::bitdepth::BitDepthDependentType;
use crate::include::common::bitdepth::BitDepthUnion;
use crate::include::common::bitdepth::DynCoef;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::bitdepth::BPC;
use crate::include::dav1d::common::Rav1dDataProps;
use crate::include::dav1d::data::Rav1dData;
use crate::include::dav1d::dav1d::Rav1dDecodeFrameType;
use crate::include::dav1d::dav1d::Rav1dEventFlags;
use crate::include::dav1d::dav1d::Rav1dInloopFilterType;
use crate::include::dav1d::headers::DRav1d;
use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::dav1d::headers::Dav1dITUTT35;
use crate::include::dav1d::headers::Dav1dSequenceHeader;
use crate::include::dav1d::headers::Rav1dContentLightLevel;
use crate::include::dav1d::headers::Rav1dFrameHeader;
use crate::include::dav1d::headers::Rav1dITUTT35;
use crate::include::dav1d::headers::Rav1dMasteringDisplay;
use crate::include::dav1d::headers::Rav1dSequenceHeader;
use crate::include::dav1d::headers::Rav1dWarpedMotionParams;
use crate::include::dav1d::headers::RAV1D_MAX_SEGMENTS;
use crate::include::dav1d::picture::Rav1dPicAllocator;
use crate::include::dav1d::picture::Rav1dPicture;
use crate::src::align::*;
use crate::src::cdef::Rav1dCdefDSPContext;
use crate::src::cdf::CdfContext;
use crate::src::cdf::CdfThreadContext;
use crate::src::env::BlockContext;
use crate::src::error::Rav1dResult;
use crate::src::filmgrain::Rav1dFilmGrainDSPContext;
use crate::src::filmgrain::GRAIN_HEIGHT;
use crate::src::filmgrain::GRAIN_WIDTH;
use crate::src::intra_edge::EdgeFlags;
use crate::src::ipred::Rav1dIntraPredDSPContext;
use crate::src::itx::Rav1dInvTxfmDSPContext;
use crate::src::levels::Av1Block;
use crate::src::levels::BlockSize;
use crate::src::levels::Filter2d;
use crate::src::lf_mask::Av1Filter;
use crate::src::lf_mask::Av1FilterLUT;
use crate::src::lf_mask::Av1Restoration;
use crate::src::lf_mask::Av1RestorationUnit;
use crate::src::log::Rav1dLogger;
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
use crate::src::recon::rav1d_backup_ipred_edge;
use crate::src::recon::rav1d_filter_sbrow;
use crate::src::recon::rav1d_filter_sbrow_cdef;
use crate::src::recon::rav1d_filter_sbrow_deblock_cols;
use crate::src::recon::rav1d_filter_sbrow_deblock_rows;
use crate::src::recon::rav1d_filter_sbrow_lr;
use crate::src::recon::rav1d_filter_sbrow_resize;
use crate::src::recon::rav1d_read_coef_blocks;
use crate::src::recon::rav1d_recon_b_inter;
use crate::src::recon::rav1d_recon_b_intra;
use crate::src::recon::read_coef_blocks_fn;
use crate::src::recon::recon_b_inter_fn;
use crate::src::recon::recon_b_intra_fn;
use crate::src::refmvs::refmvs_frame;
use crate::src::refmvs::refmvs_temporal_block;
use crate::src::refmvs::refmvs_tile;
use crate::src::refmvs::Rav1dRefmvsDSPContext;
use atomig::Atomic;
use libc::ptrdiff_t;
use std::cell::UnsafeCell;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::mem;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Sub;
use std::ptr;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::thread::JoinHandle;

#[repr(C)]
pub(crate) struct Rav1dDSPContext {
    pub fg: Rav1dFilmGrainDSPContext,
    pub ipred: Rav1dIntraPredDSPContext,
    pub mc: Rav1dMCDSPContext,
    pub itx: Rav1dInvTxfmDSPContext,
    pub lf: Rav1dLoopFilterDSPContext,
    pub cdef: Rav1dCdefDSPContext,
    pub lr: Rav1dLoopRestorationDSPContext,
    pub initialized: bool,
}

#[derive(Clone, Default)]
pub(crate) struct Rav1dTileGroupHeader {
    pub start: c_int,
    pub end: c_int,
}

#[derive(Clone, Default)]
pub(crate) struct Rav1dTileGroup {
    pub data: Rav1dData,
    pub hdr: Rav1dTileGroupHeader,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskType {
    Init = 0,
    InitCdf = 1,
    TileEntropy = 2,
    EntropyProgress = 3,
    TileReconstruction = 4,
    DeblockCols = 5,
    DeblockRows = 6,
    Cdef = 7,
    SuperResolution = 8,
    LoopRestoration = 9,
    ReconstructionProgress = 10,
    FgPrep = 11,
    FgApply = 12,
}

impl Default for TaskType {
    fn default() -> Self {
        Self::Init
    }
}

#[repr(C)]
pub(crate) struct Rav1dContext_frame_thread {
    pub out_delayed: Box<[Rav1dThreadPicture]>,
    pub next: c_uint,
}

pub type GrainLut<Entry> = [[Entry; GRAIN_WIDTH]; GRAIN_HEIGHT + 1];

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GrainBD<BD: BitDepth> {
    pub grain_lut: Align16<[GrainLut<BD::Entry>; 3]>,
    // TODO(kkysen) can use `BD::SCALING_LEN` directly with `#![feature(generic_const_exprs)]` when stabilized
    pub scaling: Align64<[BD::Scaling; 3]>,
}

// Implemented manually since we don't require `BD: Default`.
impl<BD: BitDepth> Default for GrainBD<BD> {
    fn default() -> Self {
        Self {
            grain_lut: Default::default(),
            scaling: Default::default(),
        }
    }
}

pub struct Grain;

impl BitDepthDependentType for Grain {
    type T<BD: BitDepth> = GrainBD<BD>;
}

#[repr(C)]
pub(crate) struct TaskThreadData_delayed_fg {
    pub exec: c_int,
    pub in_0: *const Rav1dPicture,
    pub out: *mut Rav1dPicture,
    pub type_0: TaskType,
    pub grain: BitDepthUnion<Grain>,
}

// TODO(SJC): Remove when TaskThreadData_delayed_fg is thread-safe
unsafe impl Send for TaskThreadData_delayed_fg {}

#[repr(C)]
pub(crate) struct TaskThreadData {
    pub cond: Condvar,
    pub first: AtomicU32,
    pub cur: AtomicU32,
    /// This is used for delayed reset of the task cur pointer when
    /// such operation is needed but the thread doesn't enter a critical
    /// section (typically when executing the next sbrow task locklessly).
    /// See [`crate::src::thread_task::reset_task_cur`].
    pub reset_task_cur: AtomicU32,
    pub cond_signaled: AtomicI32,
    pub delayed_fg_progress: [AtomicI32; 2], /* [0]=started, [1]=completed */
    pub delayed_fg_cond: Condvar,
    /// This lock has a dual purpose - protecting the delayed_fg structure, as
    /// well as synchronizing tasks across threads. Many cases do not use the
    /// inner data when holding the lock but instead use it to sequence
    /// operations. Rather than disentagle these related uses in the original C
    /// code, we have kept a single mutex and put the delayed_fg structure into
    /// it.
    pub delayed_fg: Mutex<TaskThreadData_delayed_fg>,
}

#[repr(C)]
pub(crate) struct Rav1dContext_refs {
    pub p: Rav1dThreadPicture,
    pub segmap: *mut Rav1dRef,
    pub refmvs: *mut Rav1dRef,
    pub refpoc: [c_uint; 7],
}

pub(crate) enum Rav1dContextTaskType {
    /// Worker thread in a multi-threaded context.
    Worker(JoinHandle<()>),
    /// Main thread in a single-threaded context. There are no worker threads so
    /// we need to store a Rav1dTaskContext for work that requires it.
    // This Rav1dTaskContext is heap-allocated because we don't want to bloat
    // the size of Rav1dContext, especially when it isn't used when we have
    // worker threads. This is wrapped in a mutex so we can have inner
    // mutability in rav1d_decode_frame_main where we need a mutable reference
    // to this task context along with an immutable reference to Rav1dContext.
    Single(Mutex<Box<Rav1dTaskContext>>),
}

pub(crate) struct Rav1dContextTaskThread {
    /// Type of the task thread, along with either the thread join handle for
    /// worker threads or the single-threaded task context.
    pub task: Rav1dContextTaskType,
    /// Thread specific data shared between the main thread and a worker thread.
    pub thread_data: Arc<Rav1dTaskContext_task_thread>,
}

impl Rav1dContextTaskThread {
    pub fn flushed(&self) -> bool {
        self.thread_data.flushed.load(Ordering::Relaxed)
    }
}

#[repr(C)]
pub struct Rav1dContext {
    pub(crate) fc: *mut Rav1dFrameData,
    pub(crate) n_fc: c_uint,

    /// Worker thread join handles and communication, or main thread task
    /// context if single-threaded
    pub(crate) tc: Box<[Rav1dContextTaskThread]>,

    /// Cache of OBUs that make up a single frame before we submit them
    /// to a frame worker to be decoded.
    pub(crate) tiles: Vec<Rav1dTileGroup>,
    pub(crate) n_tiles: c_int,
    pub(crate) seq_hdr: Option<Arc<DRav1d<Rav1dSequenceHeader, Dav1dSequenceHeader>>>, // TODO(kkysen) Previously pooled.
    pub(crate) frame_hdr: Option<Arc<DRav1d<Rav1dFrameHeader, Dav1dFrameHeader>>>, // TODO(kkysen) Previously pooled.
    pub(crate) content_light: Option<Arc<Rav1dContentLightLevel>>,
    pub(crate) mastering_display: Option<Arc<Rav1dMasteringDisplay>>,
    pub(crate) itut_t35: Option<Arc<DRav1d<Rav1dITUTT35, Dav1dITUTT35>>>,

    // decoded output picture queue
    pub(crate) in_0: Rav1dData,
    pub(crate) out: Rav1dThreadPicture,
    pub(crate) cache: Rav1dThreadPicture,
    pub(crate) flush: AtomicI32,
    pub(crate) frame_thread: Rav1dContext_frame_thread,

    // task threading (refer to tc[] for per_thread thingies)
    pub(crate) task_thread: Arc<TaskThreadData>,

    // reference/entropy state
    pub(crate) segmap_pool: *mut Rav1dMemPool,
    pub(crate) refmvs_pool: *mut Rav1dMemPool,
    pub(crate) refs: [Rav1dContext_refs; 8],
    pub(crate) cdf_pool: *mut Rav1dMemPool,
    pub(crate) cdf: [CdfThreadContext; 8],

    pub(crate) dsp: [Rav1dDSPContext; 3], /* 8, 10, 12 bits/component */
    pub(crate) refmvs_dsp: Rav1dRefmvsDSPContext,

    pub(crate) allocator: Rav1dPicAllocator,
    pub(crate) apply_grain: bool,
    pub(crate) operating_point: c_int,
    pub(crate) operating_point_idc: c_uint,
    pub(crate) all_layers: bool,
    pub(crate) max_spatial_id: bool,
    pub(crate) frame_size_limit: c_uint,
    pub(crate) strict_std_compliance: bool,
    pub(crate) output_invisible_frames: bool,
    pub(crate) inloop_filters: Rav1dInloopFilterType,
    pub(crate) decode_frame_type: Rav1dDecodeFrameType,
    pub(crate) drain: c_int,
    pub(crate) frame_flags: Atomic<PictureFlags>,
    pub(crate) event_flags: Rav1dEventFlags,
    pub(crate) cached_error_props: Mutex<Rav1dDataProps>,
    pub(crate) cached_error: Rav1dResult,

    pub(crate) logger: Option<Rav1dLogger>,

    pub(crate) picture_pool: *mut Rav1dMemPool,
}

// TODO(SJC): Remove when Rav1dContext is thread-safe
unsafe impl Send for Rav1dContext {}
// TODO(SJC): Remove when Rav1dContext is thread-safe
unsafe impl Sync for Rav1dContext {}

// We assume Rav1dTask is small enough to cheaply clone to avoid borrow check
// issues. If it grows too large for that, this should be revisited.
#[derive(Clone, Default)]
#[repr(C)]
pub struct Rav1dTask {
    // frame thread id
    pub frame_idx: c_uint,
    // task work
    pub type_0: TaskType,
    // sbrow
    pub sby: c_int,

    // task dependencies
    pub recon_progress: c_int,
    pub deblock_progress: c_int,
    pub deps_skip: c_int,
    // only used in task queue
    pub next: Option<Rav1dTaskIndex>,
}

#[repr(C)]
pub struct ScalableMotionParams {
    pub scale: c_int, // if no scaling, this is 0
    pub step: c_int,
}

pub(crate) struct Rav1dFrameContext_bd_fn {
    pub recon_b_intra: recon_b_intra_fn,
    pub recon_b_inter: recon_b_inter_fn,
    pub filter_sbrow: filter_sbrow_fn,
    pub filter_sbrow_deblock_cols: filter_sbrow_fn,
    pub filter_sbrow_deblock_rows: filter_sbrow_fn,
    pub filter_sbrow_cdef:
        unsafe fn(&Rav1dContext, &mut Rav1dFrameData, &mut Rav1dTaskContext, c_int) -> (),
    pub filter_sbrow_resize: filter_sbrow_fn,
    pub filter_sbrow_lr: filter_sbrow_fn,
    pub backup_ipred_edge: backup_ipred_edge_fn,
    pub read_coef_blocks: read_coef_blocks_fn,
}

impl Rav1dFrameContext_bd_fn {
    pub const fn new<BD: BitDepth>() -> Self {
        Self {
            recon_b_inter: rav1d_recon_b_inter::<BD>,
            recon_b_intra: rav1d_recon_b_intra::<BD>,
            filter_sbrow: rav1d_filter_sbrow::<BD>,
            filter_sbrow_deblock_cols: rav1d_filter_sbrow_deblock_cols::<BD>,
            filter_sbrow_deblock_rows: rav1d_filter_sbrow_deblock_rows::<BD>,
            filter_sbrow_cdef: rav1d_filter_sbrow_cdef::<BD>,
            filter_sbrow_resize: rav1d_filter_sbrow_resize::<BD>,
            filter_sbrow_lr: rav1d_filter_sbrow_lr::<BD>,
            backup_ipred_edge: rav1d_backup_ipred_edge::<BD>,
            read_coef_blocks: rav1d_read_coef_blocks::<BD>,
        }
    }

    pub const fn get(bpc: BPC) -> &'static Self {
        const BPC8: Rav1dFrameContext_bd_fn = Rav1dFrameContext_bd_fn::new::<BitDepth8>();
        const BPC16: Rav1dFrameContext_bd_fn = Rav1dFrameContext_bd_fn::new::<BitDepth16>();
        match bpc {
            BPC::BPC8 => &BPC8,
            BPC::BPC16 => &BPC16,
        }
    }

    pub unsafe fn recon_b_intra(
        &self,
        f: &Rav1dFrameData,
        context: &mut Rav1dTaskContext,
        block_size: BlockSize,
        flags: EdgeFlags,
        block: &Av1Block,
    ) {
        (self.recon_b_intra)(f, context, block_size, flags, block);
    }

    pub unsafe fn recon_b_inter(
        &self,
        f: &mut Rav1dFrameData,
        context: &mut Rav1dTaskContext,
        block_size: BlockSize,
        block: &Av1Block,
    ) -> Result<(), ()> {
        match (self.recon_b_inter)(f, context, block_size, block) {
            0 => Ok(()),
            _ => Err(()),
        }
    }

    pub unsafe fn read_coef_blocks(
        &self,
        f: &mut Rav1dFrameData,
        context: &mut Rav1dTaskContext,
        block_size: BlockSize,
        block: &Av1Block,
    ) {
        (self.read_coef_blocks)(f, context, block_size, block);
    }
}

#[derive(Default)]
#[repr(C)]
pub struct CodedBlockInfo {
    pub eob: [i16; 3], /* plane */
    pub txtp: [u8; 3], /* plane */
}

#[derive(Default)]
#[repr(C)]
pub struct Rav1dFrameContext_frame_thread {
    /// Indices: 0: reconstruction, 1: entropy.
    pub next_tile_row: [c_int; 2],

    /// Indexed using `t.by * f.b4_stride + t.bx`.
    pub b: Vec<Av1Block>,

    pub cbi: Vec<CodedBlockInfo>,

    /// Indexed using `(t.by >> 1) * (f.b4_stride >> 1) + (t.bx >> 1)`.
    /// Inner indices are `[3 plane][8 idx]`.
    pub pal: AlignedVec64<[[u16; 8]; 3]>,

    /// Iterated over inside tile state.
    pub pal_idx: AlignedVec64<u8>,

    /// [`AlignedVec64`]`<`[`DynCoef`]`>`
    pub cf: AlignedVec64<u8>,

    /// Start offsets per tile
    pub tile_start_off: Vec<u32>,
}

#[derive(Default)]
pub(crate) struct TxLpfRightEdge {
    /// `.len() = h * 2`
    inner: Vec<u8>,
}

impl TxLpfRightEdge {
    #[allow(dead_code)]
    pub const fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn resize(&mut self, right_edge_size: usize, value: u8) {
        self.inner.resize(right_edge_size * 32 * 2, value)
    }

    pub fn get(&self) -> (&[u8], &[u8]) {
        let mid = self.inner.len() / 2;
        self.inner.split_at(mid)
    }

    pub fn get_mut(&mut self) -> (&mut [u8], &mut [u8]) {
        let mid = self.inner.len() / 2;
        self.inner.split_at_mut(mid)
    }
}

/// loopfilter
#[repr(C)]
pub struct Rav1dFrameContext_lf {
    pub level: Vec<[u8; 4]>,
    pub mask: Vec<Av1Filter>, /* len = w*h */
    pub lr_mask: Vec<Av1Restoration>,
    pub lim_lut: Align16<Av1FilterLUT>,
    pub last_sharpness: c_int,
    pub lvl: [[[[u8; 2]; 8]; 4]; 8], /* [8 seg_id][4 dir][8 ref][2 is_gmv] */
    pub tx_lpf_right_edge: TxLpfRightEdge,
    pub cdef_line_buf: AlignedVec32<u8>, /* AlignedVec32<DynPixel> */
    pub lr_line_buf: AlignedVec64<u8>,
    pub cdef_line: [[usize; 3]; 2], /* [2 pre/post][3 plane] */
    pub cdef_lpf_line: [usize; 3],  /* plane */
    pub lr_lpf_line: [usize; 3],    /* plane */

    // in-loop filter per-frame state keeping
    pub start_of_tile_row: Vec<u8>,
    pub p: [usize; 3],         // Offsets into `f.cur.data.data`.
    pub sr_p: [usize; 3],      // Offsets into `f.sr_cur.p.data.data`.
    pub restore_planes: c_int, // enum LrRestorePlanes
}

#[derive(Default)]
#[repr(C)]
pub struct Rav1dFrameContext_task_thread_pending_tasks {
    pub head: Option<Rav1dTaskIndex>,
    pub tail: Option<Rav1dTaskIndex>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Rav1dTaskIndex {
    Task(usize),
    TileTask(usize),
    Init,
}

impl Rav1dTaskIndex {
    pub fn raw_index(self) -> Option<usize> {
        match self {
            Self::Task(i) => Some(i),
            Self::TileTask(i) => Some(i),
            Self::Init => None,
        }
    }
}

impl Sub for Rav1dTaskIndex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Task(x), Self::Task(y)) => Self::Task(x - y),
            (Self::TileTask(x), Self::TileTask(y)) => Self::TileTask(x - y),
            _ => panic!("Cannot subtract {rhs:?} from {self:?}"),
        }
    }
}

impl PartialOrd for Rav1dTaskIndex {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match (self, other) {
            (Self::Task(x), Self::Task(y)) => x.partial_cmp(y),
            (Self::TileTask(x), Self::TileTask(y)) => x.partial_cmp(y),
            _ => None,
        }
    }
}

impl Add<usize> for Rav1dTaskIndex {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        match self {
            Self::Task(i) => Self::Task(i + rhs),
            Self::TileTask(i) => Self::TileTask(i + rhs),
            Self::Init => panic!("Cannot add to the init task"),
        }
    }
}

impl AddAssign<usize> for Rav1dTaskIndex {
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs;
    }
}

#[derive(Default)]
pub struct Rav1dTasks {
    tasks: Vec<Rav1dTask>,
    tile_tasks_vec: Vec<Rav1dTask>,
    init_task: Rav1dTask,
    pub tile_tasks: [Option<Rav1dTaskIndex>; 2],
    pub head: Option<Rav1dTaskIndex>,
    pub tail: Option<Rav1dTaskIndex>,
    // Points to the task directly before the cur pointer in the queue.
    // This cur pointer is theoretical here, we actually keep track of the
    // "prev_t" variable. This is needed to not loose the tasks in
    // [head;cur-1] when picking one for execution.
    pub cur_prev: Option<Rav1dTaskIndex>,
}

impl Rav1dTasks {
    pub fn grow_tasks(&mut self, new_len: usize) {
        if new_len > self.tasks.len() {
            self.tasks.clear();
            self.tasks.resize_with(new_len, Default::default);
        }
    }

    pub fn grow_tile_tasks(&mut self, new_len: usize) {
        if new_len > self.tile_tasks_vec.len() {
            self.tile_tasks_vec.clear();
            self.tile_tasks_vec.resize_with(new_len, Default::default);
            self.tile_tasks[0] = Some(Rav1dTaskIndex::TileTask(0));
        }
    }
}

impl Index<Rav1dTaskIndex> for Rav1dTasks {
    type Output = Rav1dTask;

    fn index(&self, index: Rav1dTaskIndex) -> &Self::Output {
        match index {
            Rav1dTaskIndex::Task(index) => &self.tasks[index],
            Rav1dTaskIndex::TileTask(index) => &self.tile_tasks_vec[index],
            Rav1dTaskIndex::Init => &self.init_task,
        }
    }
}

impl IndexMut<Rav1dTaskIndex> for Rav1dTasks {
    fn index_mut(&mut self, index: Rav1dTaskIndex) -> &mut Self::Output {
        match index {
            Rav1dTaskIndex::Task(index) => &mut self.tasks[index],
            Rav1dTaskIndex::TileTask(index) => &mut self.tile_tasks_vec[index],
            Rav1dTaskIndex::Init => &mut self.init_task,
        }
    }
}

#[repr(C)]
pub(crate) struct Rav1dFrameContext_task_thread {
    pub lock: Mutex<()>,
    pub cond: Condvar,
    pub ttd: Arc<TaskThreadData>,
    pub tasks: UnsafeCell<Rav1dTasks>,
    pub init_done: AtomicI32,
    pub done: [AtomicI32; 2],
    pub retval: Rav1dResult,
    pub update_set: bool, // whether we need to update CDF reference
    pub error: AtomicI32,
    pub task_counter: AtomicI32,
    // async task insertion
    pub pending_tasks_merge: AtomicI32,
    pub pending_tasks: Mutex<Rav1dFrameContext_task_thread_pending_tasks>,
}

impl Rav1dFrameContext_task_thread {
    pub unsafe fn tasks(&self) -> *mut Rav1dTasks {
        self.tasks.get()
    }
}

pub(crate) struct Rav1dFrameContext_frame_thread_progress {
    pub entropy: AtomicI32,
    pub deblock: AtomicI32, // in sby units
    pub frame: Vec<AtomicU32>,
    pub copy_lpf: Vec<AtomicU32>,
}

#[repr(C)]
pub(crate) struct Rav1dFrameData {
    /// Index in [`Rav1dContext::fc`]
    pub index: usize,

    pub seq_hdr: Option<Arc<DRav1d<Rav1dSequenceHeader, Dav1dSequenceHeader>>>,
    pub frame_hdr: Option<Arc<DRav1d<Rav1dFrameHeader, Dav1dFrameHeader>>>,
    pub refp: [Rav1dThreadPicture; 7],
    // during block coding / reconstruction
    pub cur: Rav1dPicture,
    // after super-resolution upscaling
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
    pub tiles: Vec<Rav1dTileGroup>,

    // for scalable references
    pub svc: [[ScalableMotionParams; 2]; 7], /* [2 x,y][7] */
    pub resize_step: [c_int; 2],             /* y, uv */
    pub resize_start: [c_int; 2],            /* y, uv */

    pub ts: *mut Rav1dTileState,
    pub n_ts: c_int,
    pub dsp: *const Rav1dDSPContext,

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
    pub dq: [[[u16; 2]; 3]; RAV1D_MAX_SEGMENTS as usize], /* [RAV1D_MAX_SEGMENTS][3 plane][2 dc/ac] */
    pub qm: [[*const u8; 3]; 19],                         /* [3 plane][19] */
    pub a: *mut BlockContext,
    pub a_sz: c_int, /* w*tile_rows */
    pub rf: refmvs_frame,
    pub jnt_weights: [[u8; 7]; 7],
    pub bitdepth_max: c_int,

    pub frame_thread: Rav1dFrameContext_frame_thread,
    pub frame_thread_progress: Rav1dFrameContext_frame_thread_progress,
    pub lf: Rav1dFrameContext_lf,
    pub task_thread: Rav1dFrameContext_task_thread,
    pub lowest_pixel_mem: Vec<[[c_int; 2]; 7]>,
}

impl Rav1dFrameData {
    pub fn bd_fn(&self) -> &'static Rav1dFrameContext_bd_fn {
        let bpc = BPC::from_bitdepth_max(self.bitdepth_max);
        Rav1dFrameContext_bd_fn::get(bpc)
    }

    pub fn frame_hdr(&self) -> &Rav1dFrameHeader {
        self.frame_hdr.as_ref().unwrap()
    }

    pub fn seq_hdr(&self) -> &Rav1dSequenceHeader {
        self.seq_hdr.as_ref().unwrap()
    }
}

#[repr(C)]
pub struct Rav1dTileState_tiling {
    // in 4px units
    pub col_start: c_int,
    pub col_end: c_int,
    pub row_start: c_int,
    pub row_end: c_int,

    // in tile units
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

    // in sby units, TILE_ERROR after a decoding error
    pub progress: [AtomicI32; 2], /* 0: reconstruction, 1: entropy */
    pub frame_thread: [Rav1dTileState_frame_thread; 2], /* 0: reconstruction, 1: entropy */

    // in fullpel units, [0] = Y, [1] = UV, used for progress requirements
    // each entry is one tile-sbrow; middle index is refidx
    pub lowest_pixel: usize,

    pub dqmem: [[[u16; 2]; 3]; RAV1D_MAX_SEGMENTS as usize], /* [RAV1D_MAX_SEGMENTS][3 plane][2 dc/ac] */
    pub dq: TileStateRef,
    pub last_qidx: c_int,
    pub last_delta_lf: [i8; 4],
    pub lflvlmem: [[[[u8; 2]; 8]; 4]; 8], /* [8 seg_id][4 dir][8 ref][2 is_gmv] */
    pub lflvl: TileStateRef,

    pub lr_ref: [Av1RestorationUnit; 3],
}

#[derive(Clone, Copy)]
pub enum TileStateRef {
    Frame,
    Local,
}

pub struct Cf;

impl BitDepthDependentType for Cf {
    type T<BD: BitDepth> = Align64<[BD::Coef; 32 * 32]>;
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Rav1dTaskContext_scratch_compinter_seg_mask {
    pub compinter: [[i16; 16384]; 2],
    pub seg_mask: [u8; 16384],
}

pub struct Lap;

impl BitDepthDependentType for Lap {
    type T<BD: BitDepth> = [BD::Pixel; 128 * 32];
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union Rav1dTaskContext_scratch_lap {
    pub lap: BitDepthUnion<Lap>,
    pub c2rust_unnamed: Rav1dTaskContext_scratch_compinter_seg_mask,
}

// stride=192 for non-SVC, or 320 for SVC
pub struct EmuEdge;

impl BitDepthDependentType for EmuEdge {
    type T<BD: BitDepth> = [BD::Pixel; 320 * (256 + 7)];
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Rav1dTaskContext_scratch_lap_emu_edge {
    pub c2rust_unnamed: Rav1dTaskContext_scratch_lap,
    pub emu_edge: BitDepthUnion<EmuEdge>,
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
pub struct InterIntraEdgeBD<BD: BitDepth> {
    pub interintra: [BD::Pixel; 64 * 64],
    pub edge: [BD::Pixel; 257],
}

pub struct InterIntraEdge;

impl BitDepthDependentType for InterIntraEdge {
    type T<BD: BitDepth> = Align64<InterIntraEdgeBD<BD>>;
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union Rav1dTaskContext_scratch_ac_txtp_map {
    pub ac: [i16; 1024],      // intra only
    pub txtp_map: [u8; 1024], // inter only
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Rav1dTaskContext_scratch_levels_pal_ac_interintra_edge {
    pub c2rust_unnamed: Rav1dTaskContext_scratch_levels_pal,
    pub ac_txtp_map: Rav1dTaskContext_scratch_ac_txtp_map,
    pub pal_idx: [u8; 8192],
    pub pal: [[u16; 8]; 3], /* [3 plane][8 palette_idx] */
    pub interintra_edge: BitDepthUnion<InterIntraEdge>,
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
    pub cond: Condvar,
    pub ttd: Arc<TaskThreadData>,
    pub flushed: AtomicBool,
    pub die: AtomicBool,
}

impl Rav1dTaskContext_task_thread {
    pub(crate) fn new(ttd: Arc<TaskThreadData>) -> Self {
        Self {
            cond: Condvar::new(),
            ttd,
            flushed: AtomicBool::new(false),
            die: AtomicBool::new(false),
        }
    }
}

#[repr(C)]
pub(crate) struct Rav1dTaskContext {
    pub ts: *mut Rav1dTileState,
    pub bx: c_int,
    pub by: c_int,
    pub l: BlockContext,
    pub a: *mut BlockContext,
    pub rt: refmvs_tile,
    pub cf: BitDepthUnion<Cf>,
    // FIXME types can be changed to pixel (and dynamically allocated)
    // which would make copy/assign operations slightly faster?
    pub al_pal: [[[[u16; 8]; 3]; 32]; 2], /* [2 a/l][32 bx/y4][3 plane][8 palette_idx] */
    pub pal_sz_uv: [[u8; 32]; 2],         /* [2 a/l][32 bx4/by4] */
    pub scratch: Rav1dTaskContext_scratch,

    pub warpmv: Rav1dWarpedMotionParams,
    pub lf_mask: *mut Av1Filter,
    pub top_pre_cdef_toggle: c_int,
    pub cur_sb_cdef_idx_ptr: *mut i8,
    // for chroma sub8x8, we need to know the filter for all 4 subblocks in
    // a 4x4 area, but the top/left one can go out of cache already, so this
    // keeps it accessible
    pub tl_4x4_filter: Filter2d,
    pub frame_thread: Rav1dTaskContext_frame_thread,
    pub task_thread: Arc<Rav1dTaskContext_task_thread>,
}

impl Rav1dTaskContext {
    pub(crate) unsafe fn new(task_thread: Arc<Rav1dTaskContext_task_thread>) -> Self {
        Self {
            ts: ptr::null_mut(),
            bx: 0,
            by: 0,
            l: mem::zeroed(),
            a: ptr::null_mut(),
            rt: mem::zeroed(),
            cf: Default::default(),
            al_pal: Default::default(),
            pal_sz_uv: Default::default(),
            scratch: mem::zeroed(),
            warpmv: mem::zeroed(),
            lf_mask: ptr::null_mut(),
            top_pre_cdef_toggle: 0,
            cur_sb_cdef_idx_ptr: ptr::null_mut(),
            tl_4x4_filter: mem::zeroed(),
            frame_thread: Rav1dTaskContext_frame_thread { pass: 0 },
            task_thread,
        }
    }
}
