use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::BitDepth16;
use crate::include::common::bitdepth::BitDepth8;
use crate::include::common::bitdepth::BPC;
use crate::include::dav1d::common::Rav1dDataProps;
use crate::include::dav1d::data::Rav1dData;
use crate::include::dav1d::dav1d::Rav1dDecodeFrameType;
use crate::include::dav1d::dav1d::Rav1dEventFlags;
use crate::include::dav1d::dav1d::Rav1dInloopFilterType;
use crate::include::dav1d::headers::DRav1d;
use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::dav1d::headers::Dav1dSequenceHeader;
use crate::include::dav1d::headers::Rav1dContentLightLevel;
use crate::include::dav1d::headers::Rav1dFrameHeader;
use crate::include::dav1d::headers::Rav1dITUTT35;
use crate::include::dav1d::headers::Rav1dMasteringDisplay;
use crate::include::dav1d::headers::Rav1dSequenceHeader;
use crate::include::dav1d::headers::Rav1dWarpedMotionParams;
use crate::include::dav1d::picture::Rav1dPicAllocator;
use crate::include::dav1d::picture::Rav1dPicture;
use crate::src::align::Align16;
use crate::src::align::Align64;
use crate::src::align::AlignedVec64;
use crate::src::cdef::Rav1dCdefDSPContext;
use crate::src::cdf::CdfContext;
use crate::src::cdf::CdfThreadContext;
use crate::src::cpu::rav1d_get_cpu_flags;
use crate::src::cpu::CpuFlags;
use crate::src::disjoint_mut::DisjointImmutGuard;
use crate::src::disjoint_mut::DisjointMut;
use crate::src::disjoint_mut::DisjointMutArcSlice;
use crate::src::disjoint_mut::DisjointMutGuard;
use crate::src::env::BlockContext;
use crate::src::error::Rav1dError;
use crate::src::filmgrain::Rav1dFilmGrainDSPContext;
use crate::src::filmgrain::GRAIN_HEIGHT;
use crate::src::filmgrain::GRAIN_WIDTH;
use crate::src::ipred::Rav1dIntraPredDSPContext;
use crate::src::itx::Rav1dInvTxfmDSPContext;
use crate::src::levels::Av1Block;
use crate::src::levels::Filter2d;
use crate::src::levels::SegmentId;
use crate::src::levels::TxfmType;
use crate::src::levels::WHT_WHT;
use crate::src::lf_mask::Av1Filter;
use crate::src::lf_mask::Av1FilterLUT;
use crate::src::lf_mask::Av1Restoration;
use crate::src::lf_mask::Av1RestorationUnit;
use crate::src::log::Rav1dLogger;
use crate::src::loopfilter::Rav1dLoopFilterDSPContext;
use crate::src::looprestoration::Rav1dLoopRestorationDSPContext;
use crate::src::lr_apply::LrRestorePlanes;
use crate::src::mc::Rav1dMCDSPContext;
use crate::src::mem::MemPool;
use crate::src::msac::MsacContext;
use crate::src::msac::Rav1dMsacDSPContext;
use crate::src::pal::Rav1dPalDSPContext;
use crate::src::picture::PictureFlags;
use crate::src::picture::Rav1dThreadPicture;
use crate::src::recon::rav1d_backup_ipred_edge;
use crate::src::recon::rav1d_copy_pal_block_uv;
use crate::src::recon::rav1d_copy_pal_block_y;
use crate::src::recon::rav1d_filter_sbrow;
use crate::src::recon::rav1d_filter_sbrow_cdef;
use crate::src::recon::rav1d_filter_sbrow_deblock_cols;
use crate::src::recon::rav1d_filter_sbrow_deblock_rows;
use crate::src::recon::rav1d_filter_sbrow_lr;
use crate::src::recon::rav1d_filter_sbrow_resize;
use crate::src::recon::rav1d_read_coef_blocks;
use crate::src::recon::rav1d_read_pal_plane;
use crate::src::recon::rav1d_read_pal_uv;
use crate::src::recon::rav1d_recon_b_inter;
use crate::src::recon::rav1d_recon_b_intra;
use crate::src::recon::BackupIpredEdgeFn;
use crate::src::recon::CopyPalBlockFn;
use crate::src::recon::FilterSbrowFn;
use crate::src::recon::ReadCoefBlocksFn;
use crate::src::recon::ReadPalPlaneFn;
use crate::src::recon::ReadPalUVFn;
use crate::src::recon::ReconBInterFn;
use crate::src::recon::ReconBIntraFn;
use crate::src::refmvs::Rav1dRefmvsDSPContext;
use crate::src::refmvs::RefMvsFrame;
use crate::src::refmvs::RefMvsTemporalBlock;
use crate::src::refmvs::RefmvsTile;
use crate::src::relaxed_atomic::RelaxedAtomic;
use crate::src::thread_task::Rav1dTaskIndex;
use crate::src::thread_task::Rav1dTasks;
use atomig::Atom;
use atomig::Atomic;
use libc::ptrdiff_t;
use parking_lot::Condvar;
use parking_lot::Mutex;
use parking_lot::RwLock;
use parking_lot::RwLockReadGuard;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::mem;
use std::ops::Deref;
use std::ops::Range;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::OnceLock;
use std::thread::JoinHandle;
use strum::FromRepr;
use zerocopy::AsBytes;
use zerocopy::FromBytes;
use zerocopy::FromZeroes;

#[derive(Default)]
pub struct Rav1dDSPContext {
    pub pal: Rav1dPalDSPContext,
    pub refmvs: Rav1dRefmvsDSPContext,
    pub msac: Rav1dMsacDSPContext,
}

impl Rav1dDSPContext {
    pub const fn default() -> Self {
        Self {
            pal: Rav1dPalDSPContext::default(),
            refmvs: Rav1dRefmvsDSPContext::default(),
            msac: Rav1dMsacDSPContext::default(),
        }
    }

    pub const fn new(flags: CpuFlags) -> Self {
        Self {
            pal: Rav1dPalDSPContext::new(flags),
            refmvs: Rav1dRefmvsDSPContext::new(flags),
            msac: Rav1dMsacDSPContext::new(flags),
        }
    }

    pub fn get() -> &'static Self {
        static DSP: OnceLock<Rav1dDSPContext> = OnceLock::new();
        DSP.get_or_init(|| {
            let flags = rav1d_get_cpu_flags();
            Self::new(flags)
        })
    }
}

impl Default for &'static Rav1dDSPContext {
    fn default() -> Self {
        Rav1dDSPContext::get()
    }
}

pub(crate) struct Rav1dBitDepthDSPContext {
    pub fg: Rav1dFilmGrainDSPContext,
    pub ipred: Rav1dIntraPredDSPContext,
    pub mc: Rav1dMCDSPContext,
    pub itx: Rav1dInvTxfmDSPContext,
    pub lf: Rav1dLoopFilterDSPContext,
    pub cdef: Rav1dCdefDSPContext,
    pub lr: Rav1dLoopRestorationDSPContext,
}

impl Rav1dBitDepthDSPContext {
    pub const fn _default<BD: BitDepth>() -> Self {
        Self {
            fg: Rav1dFilmGrainDSPContext::default::<BD>(),
            ipred: Rav1dIntraPredDSPContext::default::<BD>(),
            mc: Rav1dMCDSPContext::default::<BD>(),
            itx: Rav1dInvTxfmDSPContext::default::<BD>(),
            lf: Rav1dLoopFilterDSPContext::default::<BD>(),
            cdef: Rav1dCdefDSPContext::default::<BD>(),
            lr: Rav1dLoopRestorationDSPContext::default::<BD>(),
        }
    }

    pub const fn new<BD: BitDepth>(flags: CpuFlags, bpc: u8) -> Self {
        Self {
            fg: Rav1dFilmGrainDSPContext::new::<BD>(flags),
            ipred: Rav1dIntraPredDSPContext::new::<BD>(flags),
            mc: Rav1dMCDSPContext::new::<BD>(flags),
            itx: Rav1dInvTxfmDSPContext::new::<BD>(flags, bpc),
            lf: Rav1dLoopFilterDSPContext::new::<BD>(flags),
            cdef: Rav1dCdefDSPContext::new::<BD>(flags),
            lr: Rav1dLoopRestorationDSPContext::new::<BD>(flags, bpc),
        }
    }

    pub fn get(bpc: u8) -> Option<&'static Self> {
        static BPC8: OnceLock<Rav1dBitDepthDSPContext> = OnceLock::new();
        static BPC10: OnceLock<Rav1dBitDepthDSPContext> = OnceLock::new();
        static BPC12: OnceLock<Rav1dBitDepthDSPContext> = OnceLock::new();
        Some(match bpc {
            8 => BPC8.get_or_init(|| {
                let flags = rav1d_get_cpu_flags();
                Self::new::<BitDepth8>(flags, bpc)
            }),
            10 => BPC10.get_or_init(|| {
                let flags = rav1d_get_cpu_flags();
                Self::new::<BitDepth16>(flags, bpc)
            }),
            12 => BPC12.get_or_init(|| {
                let flags = rav1d_get_cpu_flags();
                Self::new::<BitDepth16>(flags, bpc)
            }),
            _ => return None,
        })
    }
}

impl Default for &'static Rav1dBitDepthDSPContext {
    fn default() -> Self {
        // Just need to choose one for default initialization, not an actual default,
        // but it doesn't hurt to initialize this slightly early.
        Rav1dBitDepthDSPContext::get(8).unwrap()
    }
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Default)]
#[repr(C)]
pub(crate) struct Rav1dContextFrameThread {
    pub out_delayed: Box<[Rav1dThreadPicture]>,
    pub next: c_uint,
}

pub type GrainLut<Entry> = [[Entry; GRAIN_WIDTH]; GRAIN_HEIGHT + 1];

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

pub enum Grain {
    #[cfg(feature = "bitdepth_8")]
    Bpc8(GrainBD<BitDepth8>),
    #[cfg(feature = "bitdepth_16")]
    Bpc16(GrainBD<BitDepth16>),
}

impl Default for Grain {
    fn default() -> Self {
        cfg_if::cfg_if! {
            if #[cfg(feature = "bitdepth_8")] {
                Self::Bpc8(Default::default())
            } else if #[cfg(feature = "bitdepth_16")] {
                Self::Bpc16(Default::default())
            } else {
                compile_error!("No bitdepths enabled");
            }
        }
    }
}

#[derive(Default)]
#[repr(C)]
pub(crate) struct TaskThreadDataDelayedFg {
    pub in_0: Rav1dPicture,
    pub out: Rav1dPicture,
    pub type_0: TaskType,
    pub grain: Grain,
}

// TODO(SJC): Remove when TaskThreadDataDelayedFg is thread-safe
unsafe impl Send for TaskThreadDataDelayedFg {}
// TODO(SJC): Remove when TaskThreadDataDelayedFg is thread-safe
unsafe impl Sync for TaskThreadDataDelayedFg {}

#[derive(Default)]
#[repr(C)]
pub(crate) struct TaskThreadData {
    pub lock: Mutex<()>,
    pub cond: Condvar,
    pub first: AtomicU32,
    pub cur: RelaxedAtomic<u32>,
    /// This is used for delayed reset of the task cur pointer when
    /// such operation is needed but the thread doesn't enter a critical
    /// section (typically when executing the next sbrow task locklessly).
    /// See [`crate::src::thread_task::reset_task_cur`].
    pub reset_task_cur: AtomicU32,
    pub cond_signaled: AtomicI32,
    pub delayed_fg_exec: RelaxedAtomic<i32>,
    pub delayed_fg_cond: Condvar,
    pub delayed_fg_progress: [AtomicI32; 2], /* [0]=started, [1]=completed */
    pub delayed_fg: RwLock<TaskThreadDataDelayedFg>,
}

#[derive(Default)]
#[repr(C)]
pub(crate) struct Rav1dContextRefs {
    pub p: Rav1dThreadPicture,
    pub segmap: Option<DisjointMutArcSlice<SegmentId>>,
    pub refmvs: Option<DisjointMutArcSlice<RefMvsTemporalBlock>>,
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
    pub thread_data: Arc<Rav1dTaskContextTaskThread>,
}

impl Rav1dContextTaskThread {
    pub fn flushed(&self) -> bool {
        self.thread_data.flushed.get()
    }
}

/// State that was formerly part of [`Rav1dContext`]
/// that is flushed/reset in [`rav1d_flush`] and other `DAV1D_API`s.
/// It is not accessed by other threads
/// (not in the call tree of [`rav1d_worker_task`]).
///
/// [`rav1d_flush`]: crate::src::lib::rav1d_flush
/// [`rav1d_worker_task`]: crate::src::thread_task::rav1d_worker_task
#[derive(Default)]
pub struct Rav1dState {
    /// Cache of OBUs that make up a single frame before we submit them
    /// to a frame worker to be decoded.
    pub(crate) tiles: Vec<Rav1dTileGroup>,
    pub(crate) n_tiles: c_int,
    pub(crate) seq_hdr: Option<Arc<DRav1d<Rav1dSequenceHeader, Dav1dSequenceHeader>>>, // TODO(kkysen) Previously pooled.
    pub(crate) frame_hdr: Option<Arc<DRav1d<Rav1dFrameHeader, Dav1dFrameHeader>>>, // TODO(kkysen) Previously pooled.
    pub(crate) content_light: Option<Arc<Rav1dContentLightLevel>>,
    pub(crate) mastering_display: Option<Arc<Rav1dMasteringDisplay>>,
    pub(crate) itut_t35: Arc<Mutex<Vec<Rav1dITUTT35>>>,

    // decoded output picture queue
    pub(crate) in_0: Rav1dData,
    pub(crate) out: Rav1dThreadPicture,
    pub(crate) cache: Rav1dThreadPicture,
    pub(crate) frame_thread: Rav1dContextFrameThread,

    // reference/entropy state
    pub(crate) refs: [Rav1dContextRefs; 8],
    pub(crate) cdf: [CdfThreadContext; 8], // Previously pooled

    pub(crate) operating_point_idc: c_uint,
    pub(crate) max_spatial_id: u8,
    pub(crate) drain: bool,
    pub(crate) frame_flags: PictureFlags,
    pub(crate) event_flags: Rav1dEventFlags,
    pub(crate) cached_error_props: Rav1dDataProps,
    pub(crate) cached_error: Option<Rav1dError>,
}

#[derive(Default)]
#[repr(C)]
#[repr(align(64))]
pub struct Rav1dContext {
    pub(crate) state: Mutex<Rav1dState>,

    pub(crate) fc: Box<[Rav1dFrameContext]>,

    /// Worker thread join handles and communication, or main thread task
    /// context if single-threaded
    pub(crate) tc: Box<[Rav1dContextTaskThread]>,

    pub(crate) flush: AtomicBool,

    // task threading (refer to tc[] for per_thread thingies)
    pub(crate) task_thread: Arc<TaskThreadData>,

    pub dsp: &'static Rav1dDSPContext,

    pub(crate) allocator: Rav1dPicAllocator,
    pub(crate) apply_grain: bool,
    pub(crate) operating_point: u8,
    pub(crate) all_layers: bool,
    pub(crate) frame_size_limit: c_uint,
    pub(crate) strict_std_compliance: bool,
    pub(crate) output_invisible_frames: bool,
    pub(crate) inloop_filters: Rav1dInloopFilterType,
    pub(crate) decode_frame_type: Rav1dDecodeFrameType,

    pub(crate) logger: Option<Rav1dLogger>,

    pub(crate) picture_pool: Arc<MemPool<u8>>,
}

// TODO(SJC): Remove when Rav1dContext is thread-safe
unsafe impl Send for Rav1dContext {}
// TODO(SJC): Remove when Rav1dContext is thread-safe
unsafe impl Sync for Rav1dContext {}

#[derive(Default)]
#[repr(C)]
pub struct Rav1dTask {
    // frame thread id
    pub frame_idx: c_uint,
    pub tile_idx: c_uint,
    // task work
    pub type_0: TaskType,
    // sbrow
    pub sby: c_int,

    // task dependencies
    pub recon_progress: c_int,
    pub deblock_progress: c_int,
    pub deps_skip: RelaxedAtomic<i32>,
    // only used in task queue
    pub next: Atomic<Rav1dTaskIndex>,
}

impl Rav1dTask {
    pub fn init(frame_idx: c_uint) -> Self {
        Self {
            type_0: TaskType::Init,
            frame_idx,
            sby: 0,
            deblock_progress: 0,
            recon_progress: 0,
            ..Default::default()
        }
    }

    pub fn next(&self) -> Rav1dTaskIndex {
        self.next.load(Ordering::SeqCst)
    }

    pub fn set_next(&self, next: Rav1dTaskIndex) {
        self.next.store(next, Ordering::SeqCst)
    }
}

impl Rav1dTask {
    pub fn without_next(&self) -> Self {
        Self {
            frame_idx: self.frame_idx,
            tile_idx: self.tile_idx,
            type_0: self.type_0,
            sby: self.sby,
            recon_progress: self.recon_progress,
            deblock_progress: self.deblock_progress,
            deps_skip: self.deps_skip.clone(),
            next: Default::default(),
        }
    }
}

#[derive(Default)]
#[repr(C)]
pub struct ScalableMotionParams {
    pub scale: c_int, // if no scaling, this is 0
    pub step: c_int,
}

pub(crate) struct Rav1dFrameContextBdFn {
    pub recon_b_intra: ReconBIntraFn,
    pub recon_b_inter: ReconBInterFn,
    pub filter_sbrow: FilterSbrowFn,
    pub filter_sbrow_deblock_cols: FilterSbrowFn,
    pub filter_sbrow_deblock_rows: FilterSbrowFn,
    pub filter_sbrow_cdef: fn(&Rav1dContext, &Rav1dFrameData, &mut Rav1dTaskContext, c_int) -> (),
    pub filter_sbrow_resize: FilterSbrowFn,
    pub filter_sbrow_lr: FilterSbrowFn,
    pub backup_ipred_edge: BackupIpredEdgeFn,
    pub read_coef_blocks: ReadCoefBlocksFn,
    pub copy_pal_block_y: CopyPalBlockFn,
    pub copy_pal_block_uv: CopyPalBlockFn,
    pub read_pal_plane: ReadPalPlaneFn,
    pub read_pal_uv: ReadPalUVFn,
}

impl Rav1dFrameContextBdFn {
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
            copy_pal_block_y: rav1d_copy_pal_block_y::<BD>,
            copy_pal_block_uv: rav1d_copy_pal_block_uv::<BD>,
            read_pal_plane: rav1d_read_pal_plane::<BD>,
            read_pal_uv: rav1d_read_pal_uv::<BD>,
        }
    }

    pub const fn get(bpc: BPC) -> &'static Self {
        const BPC8: Rav1dFrameContextBdFn = Rav1dFrameContextBdFn::new::<BitDepth8>();
        const BPC16: Rav1dFrameContextBdFn = Rav1dFrameContextBdFn::new::<BitDepth16>();
        match bpc {
            BPC::BPC8 => &BPC8,
            BPC::BPC16 => &BPC16,
        }
    }
}

#[derive(Clone, Copy, Default, Atom)]
pub struct CodedBlockInfo(i16);

impl CodedBlockInfo {
    const TXTP_BITS: u8 = (TxfmType::BITS - WHT_WHT.leading_zeros()) as u8;

    pub const fn eob(&self) -> i16 {
        self.0 >> Self::TXTP_BITS
    }

    pub const fn txtp(&self) -> TxfmType {
        (self.0 & ((1 << Self::TXTP_BITS) - 1)) as TxfmType
    }

    pub const fn new(eob: i16, txtp: TxfmType) -> Self {
        debug_assert!(eob << Self::TXTP_BITS >> Self::TXTP_BITS == eob);
        Self((eob << Self::TXTP_BITS) | (txtp as i16))
    }
}

#[derive(Default)]
#[repr(C)]
pub struct Pal {
    data: DisjointMut<AlignedVec64<u8>>,
}

type PalArray<BD> = [[<BD as BitDepth>::Pixel; 8]; 3];

impl Pal {
    pub fn resize(&mut self, n: usize) {
        self.data.resize(n * 8 * 3, Default::default());
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn index<'a: 'b, 'b, BD: BitDepth>(
        &'a self,
        index: usize,
    ) -> DisjointImmutGuard<'b, AlignedVec64<u8>, PalArray<BD>> {
        self.data.element_as(index)
    }

    /// Mutably borrow a pal array.
    ///
    /// This mutable borrow is unchecked and callers must ensure that no other
    /// borrows of a pal overlaps with the mutably borrowed region for the
    /// lifetime of that mutable borrow.
    ///
    /// # Safety
    ///
    /// Caller must ensure that no elements of the resulting borrowed element is
    /// concurrently borrowed (immutably or mutably) at all during the lifetime
    /// of the returned mutable borrow.
    pub fn index_mut<'a: 'b, 'b, BD: BitDepth>(
        &'a self,
        index: usize,
    ) -> DisjointMutGuard<'b, AlignedVec64<u8>, PalArray<BD>> {
        self.data.mut_element_as(index)
    }
}

#[derive(Default)]
#[repr(C)]
pub struct Rav1dFrameContextFrameThread {
    /// Indices: 0: reconstruction, 1: entropy.
    pub next_tile_row: [RelaxedAtomic<i32>; 2],

    /// Indexed using `t.b.y * f.b4_stride + t.b.x`.
    pub b: DisjointMut<Vec<Av1Block>>,

    pub cbi: Vec<RelaxedAtomic<CodedBlockInfo>>,

    /// Indexed using `(t.b.y >> 1) * (f.b4_stride >> 1) + (t.b.x >> 1)`.
    /// Inner indices are `[3 plane][8 idx]`.
    /// Allocated as a flat array. `pal.as_slice` and `pal.as_slice_mut` should be
    /// used to access elements of type `[[BitDepth::Pixel; 8]; 3]`
    pub pal: Pal,

    /// Iterated over inside tile state.
    pub pal_idx: DisjointMut<AlignedVec64<u8>>,

    /// [`AlignedVec64`]`<`[`DynCoef`]`>`
    pub cf: DisjointMut<AlignedVec64<u8>>,

    /// Start offsets per tile
    pub tile_start_off: Vec<u32>,
}

#[derive(Default)]
pub(crate) struct TxLpfRightEdge {
    /// `.len() = h * 2`
    inner: DisjointMut<Vec<u8>>,
}

impl TxLpfRightEdge {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            inner: DisjointMut::default(),
        }
    }

    pub fn resize(&mut self, right_edge_size: usize, value: u8) {
        self.inner.resize(right_edge_size * 32 * 2, value)
    }

    pub fn get<'a>(
        &'a self,
        index_y: Range<usize>,
        index_uv: Range<usize>,
    ) -> (
        impl 'a + Deref<Target = [u8]>,
        impl 'a + Deref<Target = [u8]>,
    ) {
        let mid = self.inner.len() / 2;
        assert!(index_y.end <= mid);
        let (uv_start, uv_end) = (index_uv.start + mid, index_uv.end + mid);
        (
            self.inner.index(index_y),
            self.inner.index(uv_start..uv_end),
        )
    }

    pub fn copy_from_slice_y(&self, index: Range<usize>, src: &[u8]) {
        #[allow(unused_mut)]
        let mut slice_mut = self.inner.index_mut(index);
        slice_mut.copy_from_slice(src);
    }

    pub fn copy_from_slice_uv(&self, index: Range<usize>, src: &[u8]) {
        let mid = self.inner.len() / 2;
        #[allow(unused_mut)]
        let mut slice_mut = self.inner.index_mut(index.start + mid..index.end + mid);
        slice_mut.copy_from_slice(src);
    }
}

/// loopfilter
#[derive(Default)]
#[repr(C)]
pub struct Rav1dFrameContextLf {
    pub level: DisjointMut<Vec<u8>>,
    pub mask: Vec<Av1Filter>, /* len = w*h */
    pub lr_mask: Vec<Av1Restoration>,
    pub lim_lut: Align16<Av1FilterLUT>,
    pub lvl: [Align16<[[[u8; 2]; 8]; 4]>; 8], /* [8 seg_id][4 dir][8 ref][2 is_gmv] */
    pub last_sharpness: u8,
    pub tx_lpf_right_edge: TxLpfRightEdge,
    // cdef_line_buf was originally aligned to 32 bytes, but we need to pass
    // both cdef_line_buf and lr_line_buf as the same parameter type to
    // backup2lines.
    pub cdef_line_buf: DisjointMut<AlignedVec64<u8>>, /* AlignedVec32<DynPixel> */
    pub lr_line_buf: DisjointMut<AlignedVec64<u8>>,
    pub cdef_line: [[usize; 3]; 2], /* [2 pre/post][3 plane] */
    pub cdef_lpf_line: [usize; 3],  /* plane */
    pub lr_lpf_line: [usize; 3],    /* plane */

    // in-loop filter per-frame state keeping
    pub start_of_tile_row: Vec<u8>,
    pub restore_planes: LrRestorePlanes,
}

#[derive(Default)]
#[repr(C)]
pub struct Rav1dFrameContextTaskThreadPendingTasks {
    pub head: Rav1dTaskIndex,
    pub tail: Rav1dTaskIndex,
}

#[derive(Default)]
#[repr(C)]
pub(crate) struct Rav1dFrameContextTaskThread {
    pub lock: Mutex<()>,
    pub cond: Condvar,
    pub ttd: Arc<TaskThreadData>,
    pub tasks: Rav1dTasks,
    pub init_done: AtomicI32,
    pub done: [AtomicI32; 2],
    pub retval: Mutex<Option<Rav1dError>>,
    pub finished: AtomicBool, // true when FrameData.tiles is cleared
    pub update_set: RelaxedAtomic<bool>, // whether we need to update CDF reference
    pub error: AtomicI32,
    pub task_counter: AtomicI32,
}

#[derive(Default)]
pub(crate) struct Rav1dFrameContextFrameThreadProgress {
    pub entropy: AtomicI32,
    pub deblock: AtomicI32, // in sby units
    pub frame: RwLock<Vec<AtomicU32>>,
    pub copy_lpf: RwLock<Vec<AtomicU32>>,
}

pub(crate) struct Rav1dFrameContext {
    /// Index in [`Rav1dContext::fc`]
    pub index: usize,

    pub data: RwLock<Rav1dFrameData>,
    pub in_cdf: RwLock<CdfThreadContext>,
    pub task_thread: Rav1dFrameContextTaskThread,
    pub frame_thread_progress: Rav1dFrameContextFrameThreadProgress,
}

impl Rav1dFrameContext {
    pub fn default(index: usize) -> Self {
        Self {
            index,
            data: Default::default(),
            in_cdf: Default::default(),
            task_thread: Default::default(),
            frame_thread_progress: Default::default(),
        }
    }

    pub fn in_cdf<'a>(&'a self) -> RwLockReadGuard<'a, CdfThreadContext> {
        self.in_cdf.try_read().unwrap()
    }

    pub fn frame_hdr(&self) -> Arc<DRav1d<Rav1dFrameHeader, Dav1dFrameHeader>> {
        self.data
            .try_read()
            .unwrap()
            .frame_hdr
            .as_ref()
            .unwrap()
            .clone()
    }
}

#[derive(Default)]
#[repr(C)]
pub(crate) struct Rav1dFrameData {
    pub seq_hdr: Option<Arc<DRav1d<Rav1dSequenceHeader, Dav1dSequenceHeader>>>,
    pub frame_hdr: Option<Arc<DRav1d<Rav1dFrameHeader, Dav1dFrameHeader>>>,
    pub refp: [Rav1dThreadPicture; 7],
    // during block coding / reconstruction
    pub cur: Rav1dPicture,
    // after super-resolution upscaling
    pub sr_cur: Rav1dThreadPicture,
    pub mvs: Option<DisjointMutArcSlice<RefMvsTemporalBlock>>, // Previously pooled.
    pub ref_mvs: [Option<DisjointMutArcSlice<RefMvsTemporalBlock>>; 7],
    pub cur_segmap: Option<DisjointMutArcSlice<SegmentId>>, // Previously pooled.
    pub prev_segmap: Option<DisjointMutArcSlice<SegmentId>>,
    pub refpoc: [c_uint; 7],
    pub refrefpoc: [[c_uint; 7]; 7],
    pub gmv_warp_allowed: [u8; 7],
    pub out_cdf: CdfThreadContext,
    pub tiles: Vec<Rav1dTileGroup>,

    // for scalable references
    pub svc: [[ScalableMotionParams; 2]; 7], /* [2 x,y][7] */
    pub resize_step: [c_int; 2],             /* y, uv */
    pub resize_start: [c_int; 2],            /* y, uv */

    pub ts: Vec<Rav1dTileState>,
    pub dsp: &'static Rav1dBitDepthDSPContext,

    // `ipred_edge` contains 3 arrays of size `ipred_edge_off`. Use `index *
    // ipred_edge_off` to access one of the sub-arrays. Note that `ipred_edge_off`
    // is in pixel units (not bytes), so use `slice_as`/`mut_slice_as` and an offset
    // in pixel units when slicing.
    pub ipred_edge: DisjointMut<AlignedVec64<u8>>, // DynPixel
    pub ipred_edge_off: usize,

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
    pub dq: [[[RelaxedAtomic<u16>; 2]; 3]; SegmentId::COUNT], /* [SegmentId::COUNT][3 plane][2 dc/ac] */
    pub qm: [[Option<&'static [u8]>; 3]; 19],                 /* [3 plane][19] */
    pub a: Vec<BlockContext>,                                 /* len = w*tile_rows */
    pub rf: RefMvsFrame,
    pub jnt_weights: [[u8; 7]; 7],
    pub bitdepth_max: c_int,

    pub frame_thread: Rav1dFrameContextFrameThread,
    pub lf: Rav1dFrameContextLf,
    pub lowest_pixel_mem: DisjointMut<Vec<[[c_int; 2]; 7]>>,
}

impl Rav1dFrameData {
    pub fn bd_fn(&self) -> &'static Rav1dFrameContextBdFn {
        let bpc = BPC::from_bitdepth_max(self.bitdepth_max);
        Rav1dFrameContextBdFn::get(bpc)
    }

    pub fn frame_hdr(&self) -> &Rav1dFrameHeader {
        self.frame_hdr.as_ref().unwrap()
    }

    pub fn seq_hdr(&self) -> &Rav1dSequenceHeader {
        self.seq_hdr.as_ref().unwrap()
    }
}

#[derive(Default)]
#[repr(C)]
pub struct Rav1dTileStateTiling {
    // in 4px units
    pub col_start: i32,
    pub col_end: i32,
    pub row_start: i32,
    pub row_end: i32,

    // in tile units
    pub col: i32,
    pub row: i32,
}

#[derive(Default)]
#[repr(C)]
pub struct Rav1dTileStateFrameThread {
    /// Offset into [`Rav1dFrameContextFrameThread::pal_idx`].
    pub pal_idx: RelaxedAtomic<u32>,

    /// Offset into [`Rav1dFrameContextFrameThread::cbi`].
    pub cbi_idx: RelaxedAtomic<u32>,

    /// Offset into [`Rav1dFrameContextFrameThread::cf`].
    pub cf: RelaxedAtomic<u32>,
}

#[derive(Default)]
#[repr(C, align(32))]
pub struct Rav1dTileStateContext {
    pub cdf: CdfContext,
    pub msac: MsacContext,
}

#[derive(Default)]
#[repr(C, align(32))]
pub struct Rav1dTileState {
    pub context: Mutex<Rav1dTileStateContext>,

    pub tiling: Rav1dTileStateTiling,

    // in sby units, TILE_ERROR after a decoding error
    pub progress: [AtomicI32; 2], /* 0: reconstruction, 1: entropy */
    pub frame_thread: [Rav1dTileStateFrameThread; 2], /* 0: reconstruction, 1: entropy */

    // in fullpel units, [0] = Y, [1] = UV, used for progress requirements
    // each entry is one tile-sbrow; middle index is refidx
    pub lowest_pixel: usize,

    pub dqmem: [[[RelaxedAtomic<u16>; 2]; 3]; SegmentId::COUNT], /* [SegmentId::COUNT][3 plane][2 dc/ac] */
    pub dq: RelaxedAtomic<TileStateRef>,
    pub last_qidx: RelaxedAtomic<u8>,
    pub last_delta_lf: RelaxedAtomic<[i8; 4]>,
    pub lflvlmem: RwLock<[Align16<[[[u8; 2]; 8]; 4]>; 8]>, /* [8 seg_id][4 dir][8 ref][2 is_gmv] */
    pub lflvl: RelaxedAtomic<TileStateRef>,

    pub lr_ref: RwLock<[Av1RestorationUnit; 3]>,
}

#[derive(Clone, Copy, Default, FromRepr)]
#[repr(u8)]
pub enum TileStateRef {
    #[default]
    Frame,
    Local,
}

impl Atom for TileStateRef {
    type Repr = u8;

    fn pack(self) -> Self::Repr {
        self as u8
    }

    fn unpack(src: Self::Repr) -> Self {
        Self::from_repr(src).unwrap_or_default()
    }
}

pub const CF_LEN: usize = 32 * 32;

/// Array of 32 * 32 coef elements (either `i16` or `i32`).
#[derive(FromZeroes)]
#[repr(align(64))]
pub struct Cf([i32; CF_LEN]);

impl Cf {
    #[allow(unused)]
    pub fn select<BD: BitDepth>(&self) -> &[BD::Coef; CF_LEN] {
        FromBytes::ref_from_prefix(AsBytes::as_bytes(&self.0)).unwrap()
    }

    pub fn select_mut<BD: BitDepth>(&mut self) -> &mut [BD::Coef; CF_LEN] {
        FromBytes::mut_from_prefix(AsBytes::as_bytes_mut(&mut self.0)).unwrap()
    }
}

/// 4D array of pixel elements (either `u8` or `u16`).
///
/// Indices are `[2 a/l][32 bx/y4][3 plane][8 palette_idx]`.
#[derive(FromZeroes)]
pub struct AlPal([u16; 8 * 3 * 32 * 2]);

impl AlPal {
    pub fn select_mut<BD: BitDepth>(&mut self) -> &mut [[[[BD::Pixel; 8]; 3]; 32]; 2] {
        FromBytes::mut_from_prefix(AsBytes::as_bytes_mut(&mut self.0)).unwrap()
    }
}

pub const COMPINTER_LEN: usize = 128 * 128;
pub const SEG_MASK_LEN: usize = 128 * 128;

#[derive(FromZeroes, FromBytes, AsBytes)]
#[repr(C, align(64))]
pub struct ScratchCompinter {
    pub compinter: [[i16; COMPINTER_LEN]; 2],
    pub seg_mask: [u8; SEG_MASK_LEN],
}

// Larger of the two between `ScratchCompinter` and `[BD::Pixel; 128 * 32]`.
const SCRATCH_COMPINTER_SIZE: usize = mem::size_of::<ScratchCompinter>();
pub const SCRATCH_LAP_LEN: usize = 128 * 32;

#[derive(FromZeroes, FromBytes, AsBytes)]
#[repr(C, align(64))]
pub struct ScratchLapInter([u8; SCRATCH_COMPINTER_SIZE]);

impl ScratchLapInter {
    pub fn lap_mut<BD: BitDepth>(&mut self) -> &mut [BD::Pixel; SCRATCH_LAP_LEN] {
        FromBytes::mut_from_prefix(&mut self.0).unwrap()
    }

    pub fn inter_mut(&mut self) -> &mut ScratchCompinter {
        FromBytes::mut_from_prefix(&mut self.0).unwrap()
    }
}

pub const EMU_EDGE_LEN: usize = 320 * (256 + 7);
// stride=192 for non-SVC, or 320 for SVC
#[derive(FromZeroes, FromBytes, AsBytes)]
#[repr(C, align(64))]
pub struct ScratchEmuEdge([u8; EMU_EDGE_LEN * 2]);

impl ScratchEmuEdge {
    pub fn buf_mut<BD: BitDepth>(&mut self) -> &mut [BD::Pixel; EMU_EDGE_LEN] {
        FromBytes::mut_from_prefix(&mut self.0).unwrap()
    }
}

#[derive(FromZeroes, FromBytes, AsBytes)]
#[repr(C)]
pub struct ScratchInter {
    pub lap_inter: ScratchLapInter,
    pub emu_edge: ScratchEmuEdge,
}

#[derive(FromZeroes, FromBytes, AsBytes)]
#[repr(C, align(64))]
pub struct ScratchPal {
    pub pal_order: [[u8; 8]; 64],
    pub pal_ctx: [u8; 64],
}

#[derive(FromZeroes, FromBytes, AsBytes)]
#[repr(C, align(64))]
pub struct ScratchLevelsPal([u8; 32 * 34]);

impl ScratchLevelsPal {
    pub fn levels_mut(&mut self) -> &mut [u8; 32 * 34] {
        &mut self.0
    }

    pub fn pal_mut(&mut self) -> &mut ScratchPal {
        FromBytes::mut_from_prefix(&mut self.0).unwrap()
    }
}

pub const SCRATCH_INTER_INTRA_BUF_LEN: usize = 64 * 64;

#[derive(Clone, Copy, FromZeroes, FromBytes, AsBytes)]
#[repr(C, align(64))]
pub struct ScratchInterIntraBuf([u16; SCRATCH_INTER_INTRA_BUF_LEN * 2]);

impl ScratchInterIntraBuf {
    pub fn buf_mut<BD: BitDepth>(&mut self) -> &mut [BD::Pixel; SCRATCH_INTER_INTRA_BUF_LEN] {
        FromBytes::mut_from_prefix(AsBytes::as_bytes_mut(&mut self.0)).unwrap()
    }
}

pub const SCRATCH_EDGE_LEN: usize = 257;

#[derive(Clone, Copy, FromZeroes, FromBytes, AsBytes)]
#[repr(C, align(64))]
pub struct ScratchEdgeBuf([u8; SCRATCH_EDGE_LEN * 2 + 62]); // 257 Pixel elements + 62 padding bytes

impl ScratchEdgeBuf {
    pub fn buf_mut<BD: BitDepth>(&mut self) -> &mut [BD::Pixel; SCRATCH_EDGE_LEN] {
        FromBytes::mut_from_prefix(&mut self.0).unwrap()
    }
}

#[derive(Clone, Copy, FromZeroes, FromBytes, AsBytes)]
#[repr(C, align(16))] // Over-aligned for 8bpc (needs to be `align(8)` for 8bpc, `align(16)` for 16bpc).
pub struct ScratchPalBuf([u8; 8 * 3 * 2]); /* [3 plane][8 palette_idx] */

impl ScratchPalBuf {
    pub fn buf<BD: BitDepth>(&self) -> &[[BD::Pixel; 8]; 3] {
        FromBytes::ref_from_prefix(&self.0).unwrap()
    }

    pub fn buf_mut<BD: BitDepth>(&mut self) -> &mut [[BD::Pixel; 8]; 3] {
        FromBytes::mut_from_prefix(&mut self.0).unwrap()
    }
}

#[derive(Clone, Copy, FromZeroes, FromBytes, AsBytes)]
#[repr(C)]
pub struct ScratchInterIntraEdgePal {
    pub interintra: ScratchInterIntraBuf,
    pub edge: ScratchEdgeBuf,
    pub pal: ScratchPalBuf,

    /// For `AsBytes`, so there's no implicit padding.
    _padding: [u8; 16],
}

pub const SCRATCH_AC_TXTP_LEN: usize = 32 * 32;

#[derive(Clone, Copy, FromZeroes, FromBytes, AsBytes)]
#[repr(C, align(64))]
pub struct ScratchAcTxtpMap([u8; SCRATCH_AC_TXTP_LEN * 2]);

impl ScratchAcTxtpMap {
    pub fn ac_mut(&mut self) -> &mut [i16; SCRATCH_AC_TXTP_LEN] {
        FromBytes::mut_from_prefix(&mut self.0).unwrap()
    }

    pub fn txtp_map(&self) -> &[TxfmType; SCRATCH_AC_TXTP_LEN] {
        FromBytes::ref_from_prefix(&self.0).unwrap()
    }

    pub fn txtp_map_mut(&mut self) -> &mut [TxfmType; SCRATCH_AC_TXTP_LEN] {
        FromBytes::mut_from_prefix(&mut self.0).unwrap()
    }
}

#[derive(FromZeroes, FromBytes, AsBytes)]
#[repr(C)]
pub struct ScratchInterIntra {
    pub levels_pal: ScratchLevelsPal,
    pub ac_txtp_map: ScratchAcTxtpMap,
    pub pal_idx_y: [u8; 32 * 64],
    pub pal_idx_uv: [u8; 64 * 64], // also used as pre-pack scratch buffer
    pub interintra_edge_pal: ScratchInterIntraEdgePal,
}

// Larger of the two between `ScratchInter` and `ScratchInterIntra`.
const SCRATCH_SIZE: usize = mem::size_of::<ScratchInter>();
#[derive(FromZeroes)]
#[repr(C)]
pub struct TaskContextScratch([u8; SCRATCH_SIZE]);

impl TaskContextScratch {
    pub fn inter_mut(&mut self) -> &mut ScratchInter {
        FromBytes::mut_from_prefix(&mut self.0).unwrap()
    }

    pub fn inter_intra(&self) -> &ScratchInterIntra {
        FromBytes::ref_from_prefix(&self.0).unwrap()
    }

    pub fn inter_intra_mut(&mut self) -> &mut ScratchInterIntra {
        FromBytes::mut_from_prefix(&mut self.0).unwrap()
    }
}

#[derive(Default)]
#[repr(C)]
pub struct Rav1dTaskContextFrameThread {
    pub pass: c_int,
}

#[repr(C)]
pub(crate) struct Rav1dTaskContextTaskThread {
    pub cond: Condvar,
    pub ttd: Arc<TaskThreadData>,
    pub flushed: RelaxedAtomic<bool>,
    pub die: RelaxedAtomic<bool>,
    pub c: Mutex<Option<Arc<Rav1dContext>>>,
}

impl Rav1dTaskContextTaskThread {
    pub(crate) fn new(ttd: Arc<TaskThreadData>) -> Self {
        Self {
            cond: Condvar::new(),
            ttd,
            flushed: Default::default(),
            die: Default::default(),
            c: Default::default(),
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Bxy {
    pub x: c_int,
    pub y: c_int,
}

#[repr(C)]
pub(crate) struct Rav1dTaskContext {
    pub ts: usize, // Index into `f.ts`
    pub b: Bxy,
    pub l: BlockContext,
    pub a: usize, // Offset into `f.a`
    pub rt: RefmvsTile,
    pub cf: Cf,
    pub al_pal: AlPal,
    pub pal_sz_uv: [[u8; 32]; 2], /* [2 a/l][32 bx4/by4] */
    pub scratch: TaskContextScratch,

    pub warpmv: Rav1dWarpedMotionParams,
    /// Index into the relevant `Rav1dFrameContext::lf.mask` array.
    pub lf_mask: Option<usize>,
    pub top_pre_cdef_toggle: c_int,
    pub cur_sb_cdef_idx: usize, // index into `Rav1dFrameContext::lf.mask.cdef_idx`
    // for chroma sub8x8, we need to know the filter for all 4 subblocks in
    // a 4x4 area, but the top/left one can go out of cache already, so this
    // keeps it accessible
    pub tl_4x4_filter: Filter2d,
    pub frame_thread: Rav1dTaskContextFrameThread,
    pub task_thread: Arc<Rav1dTaskContextTaskThread>,
}

impl Rav1dTaskContext {
    pub(crate) fn new(task_thread: Arc<Rav1dTaskContextTaskThread>) -> Self {
        Self {
            ts: Default::default(),
            b: Default::default(),
            l: Default::default(),
            a: Default::default(),
            rt: Default::default(),
            cf: FromZeroes::new_zeroed(),
            al_pal: FromZeroes::new_zeroed(),
            pal_sz_uv: Default::default(),
            scratch: FromZeroes::new_zeroed(),
            warpmv: Default::default(),
            lf_mask: Default::default(),
            top_pre_cdef_toggle: Default::default(),
            cur_sb_cdef_idx: Default::default(),
            tl_4x4_filter: Default::default(),
            frame_thread: Default::default(),
            task_thread,
        }
    }
}
