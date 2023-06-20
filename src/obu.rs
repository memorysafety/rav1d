use crate::include::stddef::*;
use crate::include::stdint::*;

use ::libc;
extern "C" {
    fn realloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: size_t) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: size_t) -> libc::c_int;
    fn dav1d_submit_frame(c: *mut Dav1dContext) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_wait(__cond: *mut pthread_cond_t, __mutex: *mut pthread_mutex_t)
        -> libc::c_int;
    fn dav1d_ref_create(size: size_t) -> *mut Dav1dRef;
    fn dav1d_ref_create_using_pool(pool: *mut Dav1dMemPool, size: size_t) -> *mut Dav1dRef;
    fn dav1d_ref_dec(r#ref: *mut *mut Dav1dRef);
    fn dav1d_cdf_thread_ref(dst: *mut CdfThreadContext, src: *mut CdfThreadContext);
    fn dav1d_cdf_thread_unref(cdf: *mut CdfThreadContext);
    fn dav1d_data_ref(dst: *mut Dav1dData, src: *const Dav1dData);
    fn dav1d_data_props_copy(dst: *mut Dav1dDataProps, src: *const Dav1dDataProps);
    fn dav1d_data_unref_internal(buf: *mut Dav1dData);
    fn dav1d_thread_picture_ref(dst: *mut Dav1dThreadPicture, src: *const Dav1dThreadPicture);
    fn dav1d_thread_picture_unref(p: *mut Dav1dThreadPicture);
    fn dav1d_picture_get_event_flags(p: *const Dav1dThreadPicture) -> Dav1dEventFlags;
    fn dav1d_init_get_bits(c: *mut GetBits, data: *const uint8_t, sz: size_t);
    fn dav1d_get_bit(c: *mut GetBits) -> libc::c_uint;
    fn dav1d_get_bits(c: *mut GetBits, n: libc::c_int) -> libc::c_uint;
    fn dav1d_get_sbits(c: *mut GetBits, n: libc::c_int) -> libc::c_int;
    fn dav1d_get_uleb128(c: *mut GetBits) -> libc::c_uint;
    fn dav1d_get_uniform(c: *mut GetBits, max: libc::c_uint) -> libc::c_uint;
    fn dav1d_get_vlc(c: *mut GetBits) -> libc::c_uint;
    fn dav1d_get_bits_subexp(c: *mut GetBits, r#ref: libc::c_int, n: libc::c_uint) -> libc::c_int;
    fn dav1d_bytealign_get_bits(c: *mut GetBits);
    fn dav1d_log(c: *mut Dav1dContext, format: *const libc::c_char, _: ...);
}

use crate::src::tables::dav1d_default_wm_params;

use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::dav1d::data::Dav1dData;
use crate::include::stdatomic::atomic_int;
use crate::src::r#ref::Dav1dRef;

use crate::include::stdatomic::atomic_uint;
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
    pub frame_thread: Dav1dFrameContext_frame_thread,
    pub lf: Dav1dFrameContext_lf,
    pub task_thread: Dav1dFrameContext_task_thread,
    pub tile_thread: FrameTileThreadData,
}
use crate::src::internal::Dav1dFrameContext_task_thread;
use crate::src::internal::FrameTileThreadData;
use libc::pthread_mutex_t;

use crate::include::dav1d::headers::Dav1dContentLightLevel;
use crate::include::dav1d::headers::Dav1dITUTT35;
use crate::include::dav1d::headers::Dav1dMasteringDisplay;
use crate::include::dav1d::picture::Dav1dPicture;
use crate::src::internal::TaskThreadData;

use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::dav1d::headers::Dav1dPixelLayout;
use crate::include::dav1d::headers::Dav1dTxfmMode;
use crate::include::dav1d::headers::Dav1dWarpedMotionParams;
use crate::include::dav1d::headers::Dav1dWarpedMotionType;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I400;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I422;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;
use crate::include::dav1d::headers::DAV1D_WM_TYPE_AFFINE;
use crate::include::dav1d::headers::DAV1D_WM_TYPE_IDENTITY;
use crate::include::dav1d::headers::DAV1D_WM_TYPE_ROT_ZOOM;
use crate::include::dav1d::headers::DAV1D_WM_TYPE_TRANSLATION;

use crate::include::dav1d::headers::DAV1D_TX_4X4_ONLY;
use crate::include::dav1d::headers::DAV1D_TX_LARGEST;
use crate::include::dav1d::headers::DAV1D_TX_SWITCHABLE;

use crate::include::dav1d::headers::Dav1dRestorationType;

use crate::include::dav1d::headers::Dav1dFilterMode;
use crate::include::dav1d::headers::Dav1dLoopfilterModeRefDeltas;
use crate::include::dav1d::headers::Dav1dSegmentationData;
use crate::include::dav1d::headers::Dav1dSegmentationDataSet;
use crate::include::dav1d::headers::DAV1D_FILTER_SWITCHABLE;
use crate::include::dav1d::headers::DAV1D_RESTORATION_NONE;

use crate::include::dav1d::headers::Dav1dFrameHeaderOperatingPoint;
use crate::include::dav1d::headers::Dav1dFrameType;
use crate::include::dav1d::headers::DAV1D_FRAME_TYPE_INTER;
use crate::include::dav1d::headers::DAV1D_FRAME_TYPE_INTRA;
use crate::include::dav1d::headers::DAV1D_FRAME_TYPE_KEY;
use crate::include::dav1d::headers::DAV1D_FRAME_TYPE_SWITCH;

use crate::include::dav1d::headers::Dav1dAdaptiveBoolean;
use crate::include::dav1d::headers::Dav1dChromaSamplePosition;
use crate::include::dav1d::headers::Dav1dFilmGrainData;
use crate::include::dav1d::headers::Dav1dMatrixCoefficients;
use crate::include::dav1d::headers::Dav1dSequenceHeader;
use crate::include::dav1d::headers::Dav1dSequenceHeaderOperatingParameterInfo;
use crate::include::dav1d::headers::Dav1dSequenceHeaderOperatingPoint;
use crate::include::dav1d::headers::DAV1D_ADAPTIVE;
use crate::include::dav1d::headers::DAV1D_CHR_UNKNOWN;
use crate::include::dav1d::headers::DAV1D_MC_UNKNOWN;

use crate::include::dav1d::headers::Dav1dTransferCharacteristics;
use crate::include::dav1d::headers::DAV1D_MC_IDENTITY;
use crate::include::dav1d::headers::DAV1D_TRC_SRGB;

use crate::include::dav1d::headers::DAV1D_TRC_UNKNOWN;

use crate::include::dav1d::headers::Dav1dColorPrimaries;

use crate::include::dav1d::headers::DAV1D_COLOR_PRI_BT709;
use crate::include::dav1d::headers::DAV1D_COLOR_PRI_UNKNOWN;
use crate::include::pthread::pthread_cond_t;

use crate::src::internal::Dav1dFrameContext_lf;
use crate::src::lf_mask::Av1Filter;
pub type pixel = ();
use crate::src::lf_mask::Av1FilterLUT;

use crate::src::internal::Dav1dFrameContext_frame_thread;

pub type coef = ();

use crate::src::levels::Av1Block;
use crate::src::refmvs::refmvs_frame;

use crate::src::env::BlockContext;
use crate::src::refmvs::refmvs_temporal_block;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameContext_bd_fn {
    pub recon_b_intra: recon_b_intra_fn,
    pub recon_b_inter: recon_b_inter_fn,
    pub filter_sbrow: filter_sbrow_fn,
    pub filter_sbrow_deblock_cols: filter_sbrow_fn,
    pub filter_sbrow_deblock_rows: filter_sbrow_fn,
    pub filter_sbrow_cdef: Option<unsafe extern "C" fn(*mut Dav1dTaskContext, libc::c_int) -> ()>,
    pub filter_sbrow_resize: filter_sbrow_fn,
    pub filter_sbrow_lr: filter_sbrow_fn,
    pub backup_ipred_edge: backup_ipred_edge_fn,
    pub read_coef_blocks: read_coef_blocks_fn,
}
pub type read_coef_blocks_fn =
    Option<unsafe extern "C" fn(*mut Dav1dTaskContext, BlockSize, *const Av1Block) -> ()>;
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
use crate::src::internal::Dav1dTaskContext_frame_thread;
use crate::src::internal::Dav1dTaskContext_task_thread;
use crate::src::levels::Filter2d;

use crate::src::internal::Dav1dTaskContext_cf;
use crate::src::internal::Dav1dTaskContext_scratch;
use crate::src::refmvs::refmvs_tile;

use crate::src::internal::Dav1dTileState;

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

use crate::include::dav1d::dav1d::Dav1dDecodeFrameType;
use crate::include::dav1d::dav1d::Dav1dEventFlags;
use crate::include::dav1d::dav1d::Dav1dLogger;
use crate::src::picture::PictureFlags;
use crate::src::picture::PICTURE_FLAG_NEW_OP_PARAMS_INFO;
use crate::src::picture::PICTURE_FLAG_NEW_SEQUENCE;
use crate::src::picture::PICTURE_FLAG_NEW_TEMPORAL_UNIT;

use crate::include::dav1d::dav1d::DAV1D_DECODEFRAMETYPE_INTRA;
use crate::include::dav1d::dav1d::DAV1D_DECODEFRAMETYPE_REFERENCE;

use crate::include::dav1d::dav1d::Dav1dInloopFilterType;

use crate::include::dav1d::picture::Dav1dPicAllocator;
use crate::src::internal::Dav1dContext_intra_edge;

use crate::src::intra_edge::EdgeFlags;
use crate::src::refmvs::Dav1dRefmvsDSPContext;

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
pub type looprestorationfilter_fn = Option<
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
use crate::src::looprestoration::LooprestorationParams;
use crate::src::looprestoration::LrEdgeFlags;

pub type const_left_pixel_row = *const libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dCdefDSPContext {
    pub dir: cdef_dir_fn,
    pub fb: [cdef_fn; 3],
}
pub type cdef_fn = Option<
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
pub type cdef_dir_fn =
    Option<unsafe extern "C" fn(*const libc::c_void, ptrdiff_t, *mut libc::c_uint) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dLoopFilterDSPContext {
    pub loop_filter_sb: [[loopfilter_sb_fn; 2]; 2],
}
pub type loopfilter_sb_fn = Option<
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
pub type itxfm_fn = Option<
    unsafe extern "C" fn(*mut libc::c_void, ptrdiff_t, *mut libc::c_void, libc::c_int) -> (),
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
pub type resize_fn = Option<
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
pub type emu_edge_fn = Option<
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
pub type warp8x8t_fn = Option<
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
pub type warp8x8_fn = Option<
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
pub type blend_dir_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type blend_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
        libc::c_int,
        libc::c_int,
        *const uint8_t,
    ) -> (),
>;
pub type w_mask_fn = Option<
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
pub type mask_fn = Option<
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
pub type w_avg_fn = Option<
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
pub type avg_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mct_scaled_fn = Option<
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
pub type mct_fn = Option<
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
pub type mc_scaled_fn = Option<
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
pub type mc_fn = Option<
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
pub type pal_pred_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const uint16_t,
        *const uint8_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type cfl_pred_fn = Option<
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
pub type cfl_ac_fn = Option<
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
pub type angular_ipred_fn = Option<
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
pub type fguv_32x32xn_fn = Option<
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
pub type fgy_32x32xn_fn = Option<
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
pub type generate_grain_uv_fn = Option<
    unsafe extern "C" fn(
        *mut [entry; 82],
        *const [entry; 82],
        *const Dav1dFilmGrainData,
        intptr_t,
    ) -> (),
>;
pub type generate_grain_y_fn =
    Option<unsafe extern "C" fn(*mut [entry; 82], *const Dav1dFilmGrainData) -> ()>;
use crate::src::cdf::CdfThreadContext;

use crate::src::internal::Dav1dContext_frame_thread;
use crate::src::internal::Dav1dContext_refs;
use crate::src::internal::Dav1dTileGroup;
use crate::src::picture::Dav1dThreadPicture;
pub type backup_ipred_edge_fn = Option<unsafe extern "C" fn(*mut Dav1dTaskContext) -> ()>;
pub type filter_sbrow_fn = Option<unsafe extern "C" fn(*mut Dav1dFrameContext, libc::c_int) -> ()>;
pub type recon_b_inter_fn =
    Option<unsafe extern "C" fn(*mut Dav1dTaskContext, BlockSize, *const Av1Block) -> libc::c_int>;
pub type recon_b_intra_fn = Option<
    unsafe extern "C" fn(*mut Dav1dTaskContext, BlockSize, EdgeFlags, *const Av1Block) -> (),
>;
use crate::include::dav1d::headers::Dav1dObuType;
use crate::include::dav1d::headers::DAV1D_OBU_FRAME;
use crate::src::internal::ScalableMotionParams;

use crate::include::dav1d::headers::DAV1D_OBU_SEQ_HDR;
use crate::include::dav1d::headers::DAV1D_OBU_TD;
use crate::src::levels::ObuMetaType;

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
use crate::include::common::intops::iclip_u8;
use crate::include::common::intops::imax;
use crate::include::common::intops::imin;
use crate::include::common::intops::ulog2;

use crate::src::env::get_poc_diff;
use crate::src::r#ref::dav1d_ref_inc;
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
    let mut op_idx = 0;
    let mut spatial_mask: libc::c_uint = 0;
    let mut current_block: u64;
    memset(
        hdr as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Dav1dSequenceHeader>(),
    );
    (*hdr).profile = dav1d_get_bits(gb, 3 as libc::c_int) as libc::c_int;
    if !((*hdr).profile > 2) {
        (*hdr).still_picture = dav1d_get_bit(gb) as libc::c_int;
        (*hdr).reduced_still_picture_header = dav1d_get_bit(gb) as libc::c_int;
        if !((*hdr).reduced_still_picture_header != 0 && (*hdr).still_picture == 0) {
            if (*hdr).reduced_still_picture_header != 0 {
                (*hdr).num_operating_points = 1 as libc::c_int;
                (*hdr).operating_points[0].major_level =
                    dav1d_get_bits(gb, 3 as libc::c_int) as libc::c_int;
                (*hdr).operating_points[0].minor_level =
                    dav1d_get_bits(gb, 2 as libc::c_int) as libc::c_int;
                (*hdr).operating_points[0].initial_display_delay = 10 as libc::c_int;
                current_block = 4090602189656566074;
            } else {
                (*hdr).timing_info_present = dav1d_get_bit(gb) as libc::c_int;
                if (*hdr).timing_info_present != 0 {
                    (*hdr).num_units_in_tick = dav1d_get_bits(gb, 32 as libc::c_int) as libc::c_int;
                    (*hdr).time_scale = dav1d_get_bits(gb, 32 as libc::c_int) as libc::c_int;
                    (*hdr).equal_picture_interval = dav1d_get_bit(gb) as libc::c_int;
                    if (*hdr).equal_picture_interval != 0 {
                        let num_ticks_per_picture: libc::c_uint = dav1d_get_vlc(gb);
                        if num_ticks_per_picture == 0xffffffff as libc::c_uint {
                            current_block = 181392771181400725;
                        } else {
                            (*hdr).num_ticks_per_picture = num_ticks_per_picture
                                .wrapping_add(1 as libc::c_int as libc::c_uint);
                            current_block = 10048703153582371463;
                        }
                    } else {
                        current_block = 10048703153582371463;
                    }
                    match current_block {
                        181392771181400725 => {}
                        _ => {
                            (*hdr).decoder_model_info_present = dav1d_get_bit(gb) as libc::c_int;
                            if (*hdr).decoder_model_info_present != 0 {
                                (*hdr).encoder_decoder_buffer_delay_length =
                                    (dav1d_get_bits(gb, 5 as libc::c_int))
                                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                                        as libc::c_int;
                                (*hdr).num_units_in_decoding_tick =
                                    dav1d_get_bits(gb, 32 as libc::c_int) as libc::c_int;
                                (*hdr).buffer_removal_delay_length =
                                    (dav1d_get_bits(gb, 5 as libc::c_int))
                                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                                        as libc::c_int;
                                (*hdr).frame_presentation_delay_length =
                                    (dav1d_get_bits(gb, 5 as libc::c_int))
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
                    181392771181400725 => {}
                    _ => {
                        (*hdr).display_model_info_present = dav1d_get_bit(gb) as libc::c_int;
                        (*hdr).num_operating_points = (dav1d_get_bits(gb, 5 as libc::c_int))
                            .wrapping_add(1 as libc::c_int as libc::c_uint)
                            as libc::c_int;
                        let mut i = 0;
                        loop {
                            if !(i < (*hdr).num_operating_points) {
                                current_block = 4090602189656566074;
                                break;
                            }
                            let op: *mut Dav1dSequenceHeaderOperatingPoint =
                                &mut *((*hdr).operating_points).as_mut_ptr().offset(i as isize)
                                    as *mut Dav1dSequenceHeaderOperatingPoint;
                            (*op).idc = dav1d_get_bits(gb, 12 as libc::c_int) as libc::c_int;
                            if (*op).idc != 0
                                && ((*op).idc & 0xff as libc::c_int == 0 || (*op).idc & 0xf00 == 0)
                            {
                                current_block = 181392771181400725;
                                break;
                            }
                            (*op).major_level = (2 as libc::c_int as libc::c_uint)
                                .wrapping_add(dav1d_get_bits(gb, 3 as libc::c_int))
                                as libc::c_int;
                            (*op).minor_level = dav1d_get_bits(gb, 2 as libc::c_int) as libc::c_int;
                            if (*op).major_level > 3 {
                                (*op).tier = dav1d_get_bit(gb) as libc::c_int;
                            }
                            if (*hdr).decoder_model_info_present != 0 {
                                (*op).decoder_model_param_present =
                                    dav1d_get_bit(gb) as libc::c_int;
                                if (*op).decoder_model_param_present != 0 {
                                    let opi: *mut Dav1dSequenceHeaderOperatingParameterInfo =
                                        &mut *((*hdr).operating_parameter_info)
                                            .as_mut_ptr()
                                            .offset(i as isize)
                                            as *mut Dav1dSequenceHeaderOperatingParameterInfo;
                                    (*opi).decoder_buffer_delay = dav1d_get_bits(
                                        gb,
                                        (*hdr).encoder_decoder_buffer_delay_length,
                                    )
                                        as libc::c_int;
                                    (*opi).encoder_buffer_delay = dav1d_get_bits(
                                        gb,
                                        (*hdr).encoder_decoder_buffer_delay_length,
                                    )
                                        as libc::c_int;
                                    (*opi).low_delay_mode = dav1d_get_bit(gb) as libc::c_int;
                                }
                            }
                            if (*hdr).display_model_info_present != 0 {
                                (*op).display_model_param_present =
                                    dav1d_get_bit(gb) as libc::c_int;
                            }
                            (*op).initial_display_delay =
                                (if (*op).display_model_param_present != 0 {
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
                181392771181400725 => {}
                _ => {
                    op_idx = if (*c).operating_point < (*hdr).num_operating_points {
                        (*c).operating_point
                    } else {
                        0 as libc::c_int
                    };
                    (*c).operating_point_idc =
                        (*hdr).operating_points[op_idx as usize].idc as libc::c_uint;
                    spatial_mask = (*c).operating_point_idc >> 8;
                    (*c).max_spatial_id = if spatial_mask != 0 {
                        ulog2(spatial_mask)
                    } else {
                        0 as libc::c_int
                    };
                    (*hdr).width_n_bits = (dav1d_get_bits(gb, 4 as libc::c_int))
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                        as libc::c_int;
                    (*hdr).height_n_bits = (dav1d_get_bits(gb, 4 as libc::c_int))
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                        as libc::c_int;
                    (*hdr).max_width = (dav1d_get_bits(gb, (*hdr).width_n_bits))
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                        as libc::c_int;
                    (*hdr).max_height = (dav1d_get_bits(gb, (*hdr).height_n_bits))
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                        as libc::c_int;
                    if (*hdr).reduced_still_picture_header == 0 {
                        (*hdr).frame_id_numbers_present = dav1d_get_bit(gb) as libc::c_int;
                        if (*hdr).frame_id_numbers_present != 0 {
                            (*hdr).delta_frame_id_n_bits = (dav1d_get_bits(gb, 4 as libc::c_int))
                                .wrapping_add(2 as libc::c_int as libc::c_uint)
                                as libc::c_int;
                            (*hdr).frame_id_n_bits = (dav1d_get_bits(gb, 3 as libc::c_int))
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
                        (*hdr).screen_content_tools = (if dav1d_get_bit(gb) != 0 {
                            DAV1D_ADAPTIVE as libc::c_int as libc::c_uint
                        } else {
                            dav1d_get_bit(gb)
                        })
                            as Dav1dAdaptiveBoolean;
                        (*hdr).force_integer_mv =
                            (if (*hdr).screen_content_tools as libc::c_uint != 0 {
                                if dav1d_get_bit(gb) != 0 {
                                    DAV1D_ADAPTIVE as libc::c_int as libc::c_uint
                                } else {
                                    dav1d_get_bit(gb)
                                }
                            } else {
                                2 as libc::c_int as libc::c_uint
                            }) as Dav1dAdaptiveBoolean;
                        if (*hdr).order_hint != 0 {
                            (*hdr).order_hint_n_bits = (dav1d_get_bits(gb, 3 as libc::c_int))
                                .wrapping_add(1 as libc::c_int as libc::c_uint)
                                as libc::c_int;
                        }
                    }
                    (*hdr).super_res = dav1d_get_bit(gb) as libc::c_int;
                    (*hdr).cdef = dav1d_get_bit(gb) as libc::c_int;
                    (*hdr).restoration = dav1d_get_bit(gb) as libc::c_int;
                    (*hdr).hbd = dav1d_get_bit(gb) as libc::c_int;
                    if (*hdr).profile == 2 && (*hdr).hbd != 0 {
                        (*hdr).hbd = ((*hdr).hbd as libc::c_uint).wrapping_add(dav1d_get_bit(gb))
                            as libc::c_int as libc::c_int;
                    }
                    if (*hdr).profile != 1 as libc::c_int {
                        (*hdr).monochrome = dav1d_get_bit(gb) as libc::c_int;
                    }
                    (*hdr).color_description_present = dav1d_get_bit(gb) as libc::c_int;
                    if (*hdr).color_description_present != 0 {
                        (*hdr).pri = dav1d_get_bits(gb, 8 as libc::c_int) as Dav1dColorPrimaries;
                        (*hdr).trc =
                            dav1d_get_bits(gb, 8 as libc::c_int) as Dav1dTransferCharacteristics;
                        (*hdr).mtrx =
                            dav1d_get_bits(gb, 8 as libc::c_int) as Dav1dMatrixCoefficients;
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
                            && !((*hdr).profile == 2 && (*hdr).hbd == 2)
                        {
                            current_block = 181392771181400725;
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
                                if (*hdr).hbd == 2 {
                                    (*hdr).ss_hor = dav1d_get_bit(gb) as libc::c_int;
                                    if (*hdr).ss_hor != 0 {
                                        (*hdr).ss_ver = dav1d_get_bit(gb) as libc::c_int;
                                    }
                                } else {
                                    (*hdr).ss_hor = 1 as libc::c_int;
                                }
                                (*hdr).layout = (if (*hdr).ss_hor != 0 {
                                    if (*hdr).ss_ver != 0 {
                                        DAV1D_PIXEL_LAYOUT_I420 as libc::c_int
                                    } else {
                                        DAV1D_PIXEL_LAYOUT_I422 as libc::c_int
                                    }
                                } else {
                                    DAV1D_PIXEL_LAYOUT_I444 as libc::c_int
                                })
                                    as Dav1dPixelLayout;
                            }
                            _ => {}
                        }
                        (*hdr).chr = (if (*hdr).ss_hor & (*hdr).ss_ver != 0 {
                            dav1d_get_bits(gb, 2 as libc::c_int)
                        } else {
                            DAV1D_CHR_UNKNOWN as libc::c_int as libc::c_uint
                        }) as Dav1dChromaSamplePosition;
                        current_block = 14141370668937312244;
                    }
                    match current_block {
                        181392771181400725 => {}
                        _ => {
                            if !((*c).strict_std_compliance != 0
                                && (*hdr).mtrx as libc::c_uint
                                    == DAV1D_MC_IDENTITY as libc::c_int as libc::c_uint
                                && (*hdr).layout as libc::c_uint
                                    != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint)
                            {
                                if (*hdr).monochrome == 0 {
                                    (*hdr).separate_uv_delta_q = dav1d_get_bit(gb) as libc::c_int;
                                }
                                (*hdr).film_grain_present = dav1d_get_bit(gb) as libc::c_int;
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
        let mut i = 0;
        while i < 7 {
            if dav1d_get_bit(gb) != 0 {
                let r#ref: *const Dav1dThreadPicture = &mut (*((*c).refs)
                    .as_mut_ptr()
                    .offset(*((*(*c).frame_hdr).refidx).as_mut_ptr().offset(i as isize) as isize))
                .p;
                if ((*r#ref).p.frame_hdr).is_null() {
                    return -(1 as libc::c_int);
                }
                (*hdr).width[1] = (*(*r#ref).p.frame_hdr).width[1];
                (*hdr).height = (*(*r#ref).p.frame_hdr).height;
                (*hdr).render_width = (*(*r#ref).p.frame_hdr).render_width;
                (*hdr).render_height = (*(*r#ref).p.frame_hdr).render_height;
                (*hdr).super_res.enabled =
                    ((*seqhdr).super_res != 0 && dav1d_get_bit(gb) != 0) as libc::c_int;
                if (*hdr).super_res.enabled != 0 {
                    (*hdr).super_res.width_scale_denominator = (9 as libc::c_int as libc::c_uint)
                        .wrapping_add(dav1d_get_bits(gb, 3 as libc::c_int))
                        as libc::c_int;
                    let d = (*hdr).super_res.width_scale_denominator;
                    (*hdr).width[0] = imax(
                        ((*hdr).width[1] * 8 + (d >> 1)) / d,
                        imin(16 as libc::c_int, (*hdr).width[1]),
                    );
                } else {
                    (*hdr).super_res.width_scale_denominator = 8 as libc::c_int;
                    (*hdr).width[0] = (*hdr).width[1];
                }
                return 0 as libc::c_int;
            }
            i += 1;
        }
    }
    if (*hdr).frame_size_override != 0 {
        (*hdr).width[1] = (dav1d_get_bits(gb, (*seqhdr).width_n_bits))
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            as libc::c_int;
        (*hdr).height = (dav1d_get_bits(gb, (*seqhdr).height_n_bits))
            .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
    } else {
        (*hdr).width[1] = (*seqhdr).max_width;
        (*hdr).height = (*seqhdr).max_height;
    }
    (*hdr).super_res.enabled = ((*seqhdr).super_res != 0 && dav1d_get_bit(gb) != 0) as libc::c_int;
    if (*hdr).super_res.enabled != 0 {
        (*hdr).super_res.width_scale_denominator = (9 as libc::c_int as libc::c_uint)
            .wrapping_add(dav1d_get_bits(gb, 3 as libc::c_int))
            as libc::c_int;
        let d_0 = (*hdr).super_res.width_scale_denominator;
        (*hdr).width[0] = imax(
            ((*hdr).width[1] * 8 + (d_0 >> 1)) / d_0,
            imin(16 as libc::c_int, (*hdr).width[1]),
        );
    } else {
        (*hdr).super_res.width_scale_denominator = 8 as libc::c_int;
        (*hdr).width[0] = (*hdr).width[1];
    }
    (*hdr).have_render_size = dav1d_get_bit(gb) as libc::c_int;
    if (*hdr).have_render_size != 0 {
        (*hdr).render_width = (dav1d_get_bits(gb, 16 as libc::c_int))
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            as libc::c_int;
        (*hdr).render_height = (dav1d_get_bits(gb, 16 as libc::c_int))
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            as libc::c_int;
    } else {
        (*hdr).render_width = (*hdr).width[1];
        (*hdr).render_height = (*hdr).height;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn tile_log2(sz: libc::c_int, tgt: libc::c_int) -> libc::c_int {
    let mut k = 0;
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
unsafe extern "C" fn parse_frame_hdr(c: *mut Dav1dContext, gb: *mut GetBits) -> libc::c_int {
    let mut sbsz_min1 = 0;
    let mut sbsz_log2 = 0;
    let mut sbw = 0;
    let mut sbh = 0;
    let mut max_tile_width_sb = 0;
    let mut max_tile_area_sb = 0;
    let mut min_log2_tiles = 0;
    let mut delta_lossless = 0;
    let mut current_block: u64;
    let seqhdr: *const Dav1dSequenceHeader = (*c).seq_hdr;
    let hdr: *mut Dav1dFrameHeader = (*c).frame_hdr;
    (*hdr).show_existing_frame =
        ((*seqhdr).reduced_still_picture_header == 0 && dav1d_get_bit(gb) != 0) as libc::c_int;
    if (*hdr).show_existing_frame != 0 {
        (*hdr).existing_frame_idx = dav1d_get_bits(gb, 3 as libc::c_int) as libc::c_int;
        if (*seqhdr).decoder_model_info_present != 0 && (*seqhdr).equal_picture_interval == 0 {
            (*hdr).frame_presentation_delay =
                dav1d_get_bits(gb, (*seqhdr).frame_presentation_delay_length) as libc::c_int;
        }
        if (*seqhdr).frame_id_numbers_present != 0 {
            (*hdr).frame_id = dav1d_get_bits(gb, (*seqhdr).frame_id_n_bits) as libc::c_int;
            let ref_frame_hdr: *mut Dav1dFrameHeader =
                (*c).refs[(*hdr).existing_frame_idx as usize].p.p.frame_hdr;
            if ref_frame_hdr.is_null() || (*ref_frame_hdr).frame_id != (*hdr).frame_id {
                current_block = 17922947093064792850;
            } else {
                current_block = 7351195479953500246;
            }
        } else {
            current_block = 7351195479953500246;
        }
        match current_block {
            17922947093064792850 => {}
            _ => return 0 as libc::c_int,
        }
    } else {
        (*hdr).frame_type = (if (*seqhdr).reduced_still_picture_header != 0 {
            DAV1D_FRAME_TYPE_KEY as libc::c_int as libc::c_uint
        } else {
            dav1d_get_bits(gb, 2 as libc::c_int)
        }) as Dav1dFrameType;
        (*hdr).show_frame =
            ((*seqhdr).reduced_still_picture_header != 0 || dav1d_get_bit(gb) != 0) as libc::c_int;
        if (*hdr).show_frame != 0 {
            if (*seqhdr).decoder_model_info_present != 0 && (*seqhdr).equal_picture_interval == 0 {
                (*hdr).frame_presentation_delay =
                    dav1d_get_bits(gb, (*seqhdr).frame_presentation_delay_length) as libc::c_int;
            }
            (*hdr).showable_frame = ((*hdr).frame_type as libc::c_uint
                != DAV1D_FRAME_TYPE_KEY as libc::c_int as libc::c_uint)
                as libc::c_int;
        } else {
            (*hdr).showable_frame = dav1d_get_bit(gb) as libc::c_int;
        }
        (*hdr).error_resilient_mode = ((*hdr).frame_type as libc::c_uint
            == DAV1D_FRAME_TYPE_KEY as libc::c_int as libc::c_uint
            && (*hdr).show_frame != 0
            || (*hdr).frame_type as libc::c_uint
                == DAV1D_FRAME_TYPE_SWITCH as libc::c_int as libc::c_uint
            || (*seqhdr).reduced_still_picture_header != 0
            || dav1d_get_bit(gb) != 0) as libc::c_int;
        (*hdr).disable_cdf_update = dav1d_get_bit(gb) as libc::c_int;
        (*hdr).allow_screen_content_tools = (if (*seqhdr).screen_content_tools as libc::c_uint
            == DAV1D_ADAPTIVE as libc::c_int as libc::c_uint
        {
            dav1d_get_bit(gb)
        } else {
            (*seqhdr).screen_content_tools as libc::c_uint
        }) as libc::c_int;
        if (*hdr).allow_screen_content_tools != 0 {
            (*hdr).force_integer_mv = (if (*seqhdr).force_integer_mv as libc::c_uint
                == DAV1D_ADAPTIVE as libc::c_int as libc::c_uint
            {
                dav1d_get_bit(gb)
            } else {
                (*seqhdr).force_integer_mv as libc::c_uint
            }) as libc::c_int;
        } else {
            (*hdr).force_integer_mv = 0 as libc::c_int;
        }
        if (*hdr).frame_type as libc::c_uint & 1 as libc::c_uint == 0 {
            (*hdr).force_integer_mv = 1 as libc::c_int;
        }
        if (*seqhdr).frame_id_numbers_present != 0 {
            (*hdr).frame_id = dav1d_get_bits(gb, (*seqhdr).frame_id_n_bits) as libc::c_int;
        }
        (*hdr).frame_size_override = (if (*seqhdr).reduced_still_picture_header != 0 {
            0 as libc::c_int as libc::c_uint
        } else if (*hdr).frame_type as libc::c_uint
            == DAV1D_FRAME_TYPE_SWITCH as libc::c_int as libc::c_uint
        {
            1 as libc::c_int as libc::c_uint
        } else {
            dav1d_get_bit(gb)
        }) as libc::c_int;
        (*hdr).frame_offset = (if (*seqhdr).order_hint != 0 {
            dav1d_get_bits(gb, (*seqhdr).order_hint_n_bits)
        } else {
            0 as libc::c_int as libc::c_uint
        }) as libc::c_int;
        (*hdr).primary_ref_frame = (if (*hdr).error_resilient_mode == 0
            && (*hdr).frame_type as libc::c_uint & 1 as libc::c_uint != 0
        {
            dav1d_get_bits(gb, 3 as libc::c_int)
        } else {
            7 as libc::c_int as libc::c_uint
        }) as libc::c_int;
        if (*seqhdr).decoder_model_info_present != 0 {
            (*hdr).buffer_removal_time_present = dav1d_get_bit(gb) as libc::c_int;
            if (*hdr).buffer_removal_time_present != 0 {
                let mut i = 0;
                while i < (*(*c).seq_hdr).num_operating_points {
                    let seqop: *const Dav1dSequenceHeaderOperatingPoint =
                        &*((*seqhdr).operating_points).as_ptr().offset(i as isize)
                            as *const Dav1dSequenceHeaderOperatingPoint;
                    let op: *mut Dav1dFrameHeaderOperatingPoint =
                        &mut *((*hdr).operating_points).as_mut_ptr().offset(i as isize)
                            as *mut Dav1dFrameHeaderOperatingPoint;
                    if (*seqop).decoder_model_param_present != 0 {
                        let mut in_temporal_layer = (*seqop).idc >> (*hdr).temporal_id & 1;
                        let mut in_spatial_layer = (*seqop).idc >> (*hdr).spatial_id + 8 & 1;
                        if (*seqop).idc == 0 || in_temporal_layer != 0 && in_spatial_layer != 0 {
                            (*op).buffer_removal_time =
                                dav1d_get_bits(gb, (*seqhdr).buffer_removal_delay_length)
                                    as libc::c_int;
                        }
                    }
                    i += 1;
                }
            }
        }
        if (*hdr).frame_type as libc::c_uint & 1 as libc::c_uint == 0 {
            (*hdr).refresh_frame_flags = (if (*hdr).frame_type as libc::c_uint
                == DAV1D_FRAME_TYPE_KEY as libc::c_int as libc::c_uint
                && (*hdr).show_frame != 0
            {
                0xff as libc::c_int as libc::c_uint
            } else {
                dav1d_get_bits(gb, 8 as libc::c_int)
            }) as libc::c_int;
            if (*hdr).refresh_frame_flags != 0xff as libc::c_int
                && (*hdr).error_resilient_mode != 0
                && (*seqhdr).order_hint != 0
            {
                let mut i_0 = 0;
                while i_0 < 8 {
                    dav1d_get_bits(gb, (*seqhdr).order_hint_n_bits);
                    i_0 += 1;
                }
            }
            if (*c).strict_std_compliance != 0
                && (*hdr).frame_type as libc::c_uint
                    == DAV1D_FRAME_TYPE_INTRA as libc::c_int as libc::c_uint
                && (*hdr).refresh_frame_flags == 0xff as libc::c_int
            {
                current_block = 17922947093064792850;
            } else if read_frame_size(c, gb, 0 as libc::c_int) < 0 {
                current_block = 17922947093064792850;
            } else {
                (*hdr).allow_intrabc = ((*hdr).allow_screen_content_tools != 0
                    && (*hdr).super_res.enabled == 0
                    && dav1d_get_bit(gb) != 0)
                    as libc::c_int;
                (*hdr).use_ref_frame_mvs = 0 as libc::c_int;
                current_block = 16314074004867283505;
            }
        } else {
            (*hdr).allow_intrabc = 0 as libc::c_int;
            (*hdr).refresh_frame_flags = (if (*hdr).frame_type as libc::c_uint
                == DAV1D_FRAME_TYPE_SWITCH as libc::c_int as libc::c_uint
            {
                0xff as libc::c_int as libc::c_uint
            } else {
                dav1d_get_bits(gb, 8 as libc::c_int)
            }) as libc::c_int;
            if (*hdr).error_resilient_mode != 0 && (*seqhdr).order_hint != 0 {
                let mut i_1 = 0;
                while i_1 < 8 {
                    dav1d_get_bits(gb, (*seqhdr).order_hint_n_bits);
                    i_1 += 1;
                }
            }
            (*hdr).frame_ref_short_signaling =
                ((*seqhdr).order_hint != 0 && dav1d_get_bit(gb) != 0) as libc::c_int;
            if (*hdr).frame_ref_short_signaling != 0 {
                (*hdr).refidx[0] = dav1d_get_bits(gb, 3 as libc::c_int) as libc::c_int;
                (*hdr).refidx[2] = -(1 as libc::c_int);
                (*hdr).refidx[1] = (*hdr).refidx[2];
                (*hdr).refidx[3] = dav1d_get_bits(gb, 3 as libc::c_int) as libc::c_int;
                (*hdr).refidx[6] = -(1 as libc::c_int);
                (*hdr).refidx[5] = (*hdr).refidx[6];
                (*hdr).refidx[4] = (*hdr).refidx[5];
                let mut shifted_frame_offset: [libc::c_int; 8] = [0; 8];
                let current_frame_offset = (1 as libc::c_int) << (*seqhdr).order_hint_n_bits - 1;
                let mut i_2 = 0;
                loop {
                    if !(i_2 < 8) {
                        current_block = 5159818223158340697;
                        break;
                    }
                    if ((*c).refs[i_2 as usize].p.p.frame_hdr).is_null() {
                        current_block = 17922947093064792850;
                        break;
                    }
                    shifted_frame_offset[i_2 as usize] = current_frame_offset
                        + get_poc_diff(
                            (*seqhdr).order_hint_n_bits,
                            (*(*c).refs[i_2 as usize].p.p.frame_hdr).frame_offset,
                            (*hdr).frame_offset,
                        );
                    i_2 += 1;
                }
                match current_block {
                    17922947093064792850 => {}
                    _ => {
                        let mut used_frame: [libc::c_int; 8] =
                            [0 as libc::c_int, 0, 0, 0, 0, 0, 0, 0];
                        used_frame[(*hdr).refidx[0] as usize] = 1 as libc::c_int;
                        used_frame[(*hdr).refidx[3] as usize] = 1 as libc::c_int;
                        let mut latest_frame_offset = -(1 as libc::c_int);
                        let mut i_3 = 0;
                        while i_3 < 8 {
                            let hint = shifted_frame_offset[i_3 as usize];
                            if used_frame[i_3 as usize] == 0
                                && hint >= current_frame_offset
                                && hint >= latest_frame_offset
                            {
                                (*hdr).refidx[6] = i_3;
                                latest_frame_offset = hint;
                            }
                            i_3 += 1;
                        }
                        if latest_frame_offset != -(1 as libc::c_int) {
                            used_frame[(*hdr).refidx[6] as usize] = 1 as libc::c_int;
                        }
                        let mut earliest_frame_offset = 2147483647 as libc::c_int;
                        let mut i_4 = 0;
                        while i_4 < 8 {
                            let hint_0 = shifted_frame_offset[i_4 as usize];
                            if used_frame[i_4 as usize] == 0
                                && hint_0 >= current_frame_offset
                                && hint_0 < earliest_frame_offset
                            {
                                (*hdr).refidx[4] = i_4;
                                earliest_frame_offset = hint_0;
                            }
                            i_4 += 1;
                        }
                        if earliest_frame_offset != 2147483647 as libc::c_int {
                            used_frame[(*hdr).refidx[4] as usize] = 1 as libc::c_int;
                        }
                        earliest_frame_offset = 2147483647 as libc::c_int;
                        let mut i_5 = 0;
                        while i_5 < 8 {
                            let hint_1 = shifted_frame_offset[i_5 as usize];
                            if used_frame[i_5 as usize] == 0
                                && hint_1 >= current_frame_offset
                                && hint_1 < earliest_frame_offset
                            {
                                (*hdr).refidx[5] = i_5;
                                earliest_frame_offset = hint_1;
                            }
                            i_5 += 1;
                        }
                        if earliest_frame_offset != 2147483647 as libc::c_int {
                            used_frame[(*hdr).refidx[5] as usize] = 1 as libc::c_int;
                        }
                        let mut i_6 = 1;
                        while i_6 < 7 {
                            if (*hdr).refidx[i_6 as usize] < 0 {
                                latest_frame_offset = -(1 as libc::c_int);
                                let mut j = 0;
                                while j < 8 {
                                    let hint_2 = shifted_frame_offset[j as usize];
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
                                    used_frame[(*hdr).refidx[i_6 as usize] as usize] =
                                        1 as libc::c_int;
                                }
                            }
                            i_6 += 1;
                        }
                        earliest_frame_offset = 2147483647 as libc::c_int;
                        let mut r#ref = -(1 as libc::c_int);
                        let mut i_7 = 0;
                        while i_7 < 8 {
                            let hint_3 = shifted_frame_offset[i_7 as usize];
                            if hint_3 < earliest_frame_offset {
                                r#ref = i_7;
                                earliest_frame_offset = hint_3;
                            }
                            i_7 += 1;
                        }
                        let mut i_8 = 0;
                        while i_8 < 7 {
                            if (*hdr).refidx[i_8 as usize] < 0 {
                                (*hdr).refidx[i_8 as usize] = r#ref;
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
                17922947093064792850 => {}
                _ => {
                    let mut i_9 = 0;
                    loop {
                        if !(i_9 < 7) {
                            current_block = 5248622017361056354;
                            break;
                        }
                        if (*hdr).frame_ref_short_signaling == 0 {
                            (*hdr).refidx[i_9 as usize] =
                                dav1d_get_bits(gb, 3 as libc::c_int) as libc::c_int;
                        }
                        if (*seqhdr).frame_id_numbers_present != 0 {
                            let delta_ref_frame_id_minus_1 =
                                dav1d_get_bits(gb, (*seqhdr).delta_frame_id_n_bits) as libc::c_int;
                            let ref_frame_id = (*hdr).frame_id
                                + ((1 as libc::c_int) << (*seqhdr).frame_id_n_bits)
                                - delta_ref_frame_id_minus_1
                                - 1
                                & ((1 as libc::c_int) << (*seqhdr).frame_id_n_bits) - 1;
                            let ref_frame_hdr_0: *mut Dav1dFrameHeader = (*c).refs
                                [(*hdr).refidx[i_9 as usize] as usize]
                                .p
                                .p
                                .frame_hdr;
                            if ref_frame_hdr_0.is_null()
                                || (*ref_frame_hdr_0).frame_id != ref_frame_id
                            {
                                current_block = 17922947093064792850;
                                break;
                            }
                        }
                        i_9 += 1;
                    }
                    match current_block {
                        17922947093064792850 => {}
                        _ => {
                            let use_ref = ((*hdr).error_resilient_mode == 0
                                && (*hdr).frame_size_override != 0)
                                as libc::c_int;
                            if read_frame_size(c, gb, use_ref) < 0 {
                                current_block = 17922947093064792850;
                            } else {
                                (*hdr).hp = ((*hdr).force_integer_mv == 0 && dav1d_get_bit(gb) != 0)
                                    as libc::c_int;
                                (*hdr).subpel_filter_mode = (if dav1d_get_bit(gb) != 0 {
                                    DAV1D_FILTER_SWITCHABLE as libc::c_int as libc::c_uint
                                } else {
                                    dav1d_get_bits(gb, 2 as libc::c_int)
                                })
                                    as Dav1dFilterMode;
                                (*hdr).switchable_motion_mode = dav1d_get_bit(gb) as libc::c_int;
                                (*hdr).use_ref_frame_mvs = ((*hdr).error_resilient_mode == 0
                                    && (*seqhdr).ref_frame_mvs != 0
                                    && (*seqhdr).order_hint != 0
                                    && (*hdr).frame_type as libc::c_uint & 1 as libc::c_uint != 0
                                    && dav1d_get_bit(gb) != 0)
                                    as libc::c_int;
                                current_block = 16314074004867283505;
                            }
                        }
                    }
                }
            }
        }
        match current_block {
            17922947093064792850 => {}
            _ => {
                (*hdr).refresh_context = ((*seqhdr).reduced_still_picture_header == 0
                    && (*hdr).disable_cdf_update == 0
                    && dav1d_get_bit(gb) == 0)
                    as libc::c_int;
                (*hdr).tiling.uniform = dav1d_get_bit(gb) as libc::c_int;
                sbsz_min1 = ((64 as libc::c_int) << (*seqhdr).sb128) - 1;
                sbsz_log2 = 6 + (*seqhdr).sb128;
                sbw = (*hdr).width[0] + sbsz_min1 >> sbsz_log2;
                sbh = (*hdr).height + sbsz_min1 >> sbsz_log2;
                max_tile_width_sb = 4096 >> sbsz_log2;
                max_tile_area_sb = 4096 * 2304 >> 2 * sbsz_log2;
                (*hdr).tiling.min_log2_cols = tile_log2(max_tile_width_sb, sbw);
                (*hdr).tiling.max_log2_cols =
                    tile_log2(1 as libc::c_int, imin(sbw, 64 as libc::c_int));
                (*hdr).tiling.max_log2_rows =
                    tile_log2(1 as libc::c_int, imin(sbh, 64 as libc::c_int));
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
                    let tile_w = 1 as libc::c_int + (sbw - 1 >> (*hdr).tiling.log2_cols);
                    (*hdr).tiling.cols = 0 as libc::c_int;
                    let mut sbx = 0;
                    while sbx < sbw {
                        (*hdr).tiling.col_start_sb[(*hdr).tiling.cols as usize] = sbx as uint16_t;
                        sbx += tile_w;
                        (*hdr).tiling.cols += 1;
                    }
                    (*hdr).tiling.min_log2_rows =
                        imax(min_log2_tiles - (*hdr).tiling.log2_cols, 0 as libc::c_int);
                    (*hdr).tiling.log2_rows = (*hdr).tiling.min_log2_rows;
                    while (*hdr).tiling.log2_rows < (*hdr).tiling.max_log2_rows
                        && dav1d_get_bit(gb) != 0
                    {
                        (*hdr).tiling.log2_rows += 1;
                    }
                    let tile_h = 1 as libc::c_int + (sbh - 1 >> (*hdr).tiling.log2_rows);
                    (*hdr).tiling.rows = 0 as libc::c_int;
                    let mut sby = 0;
                    while sby < sbh {
                        (*hdr).tiling.row_start_sb[(*hdr).tiling.rows as usize] = sby as uint16_t;
                        sby += tile_h;
                        (*hdr).tiling.rows += 1;
                    }
                } else {
                    (*hdr).tiling.cols = 0 as libc::c_int;
                    let mut widest_tile = 0;
                    let mut max_tile_area_sb_0 = sbw * sbh;
                    let mut sbx_0 = 0;
                    while sbx_0 < sbw && (*hdr).tiling.cols < 64 {
                        let tile_width_sb = imin(sbw - sbx_0, max_tile_width_sb);
                        let tile_w_0 = (if tile_width_sb > 1 {
                            (1 as libc::c_int as libc::c_uint)
                                .wrapping_add(dav1d_get_uniform(gb, tile_width_sb as libc::c_uint))
                        } else {
                            1 as libc::c_int as libc::c_uint
                        }) as libc::c_int;
                        (*hdr).tiling.col_start_sb[(*hdr).tiling.cols as usize] = sbx_0 as uint16_t;
                        sbx_0 += tile_w_0;
                        widest_tile = imax(widest_tile, tile_w_0);
                        (*hdr).tiling.cols += 1;
                    }
                    (*hdr).tiling.log2_cols = tile_log2(1 as libc::c_int, (*hdr).tiling.cols);
                    if min_log2_tiles != 0 {
                        max_tile_area_sb_0 >>= min_log2_tiles + 1;
                    }
                    let max_tile_height_sb =
                        imax(max_tile_area_sb_0 / widest_tile, 1 as libc::c_int);
                    (*hdr).tiling.rows = 0 as libc::c_int;
                    let mut sby_0 = 0;
                    while sby_0 < sbh && (*hdr).tiling.rows < 64 {
                        let tile_height_sb = imin(sbh - sby_0, max_tile_height_sb);
                        let tile_h_0 = (if tile_height_sb > 1 {
                            (1 as libc::c_int as libc::c_uint)
                                .wrapping_add(dav1d_get_uniform(gb, tile_height_sb as libc::c_uint))
                        } else {
                            1 as libc::c_int as libc::c_uint
                        }) as libc::c_int;
                        (*hdr).tiling.row_start_sb[(*hdr).tiling.rows as usize] = sby_0 as uint16_t;
                        sby_0 += tile_h_0;
                        (*hdr).tiling.rows += 1;
                    }
                    (*hdr).tiling.log2_rows = tile_log2(1 as libc::c_int, (*hdr).tiling.rows);
                }
                (*hdr).tiling.col_start_sb[(*hdr).tiling.cols as usize] = sbw as uint16_t;
                (*hdr).tiling.row_start_sb[(*hdr).tiling.rows as usize] = sbh as uint16_t;
                if (*hdr).tiling.log2_cols != 0 || (*hdr).tiling.log2_rows != 0 {
                    (*hdr).tiling.update =
                        dav1d_get_bits(gb, (*hdr).tiling.log2_cols + (*hdr).tiling.log2_rows)
                            as libc::c_int;
                    if (*hdr).tiling.update >= (*hdr).tiling.cols * (*hdr).tiling.rows {
                        current_block = 17922947093064792850;
                    } else {
                        (*hdr).tiling.n_bytes = (dav1d_get_bits(gb, 2 as libc::c_int))
                            .wrapping_add(1 as libc::c_int as libc::c_uint);
                        current_block = 1918110639124887667;
                    }
                } else {
                    (*hdr).tiling.update = 0 as libc::c_int;
                    (*hdr).tiling.n_bytes = (*hdr).tiling.update as libc::c_uint;
                    current_block = 1918110639124887667;
                }
                match current_block {
                    17922947093064792850 => {}
                    _ => {
                        (*hdr).quant.yac = dav1d_get_bits(gb, 8 as libc::c_int) as libc::c_int;
                        (*hdr).quant.ydc_delta = if dav1d_get_bit(gb) != 0 {
                            dav1d_get_sbits(gb, 7 as libc::c_int)
                        } else {
                            0 as libc::c_int
                        };
                        if (*seqhdr).monochrome == 0 {
                            let diff_uv_delta = (if (*seqhdr).separate_uv_delta_q != 0 {
                                dav1d_get_bit(gb)
                            } else {
                                0 as libc::c_int as libc::c_uint
                            }) as libc::c_int;
                            (*hdr).quant.udc_delta = if dav1d_get_bit(gb) != 0 {
                                dav1d_get_sbits(gb, 7 as libc::c_int)
                            } else {
                                0 as libc::c_int
                            };
                            (*hdr).quant.uac_delta = if dav1d_get_bit(gb) != 0 {
                                dav1d_get_sbits(gb, 7 as libc::c_int)
                            } else {
                                0 as libc::c_int
                            };
                            if diff_uv_delta != 0 {
                                (*hdr).quant.vdc_delta = if dav1d_get_bit(gb) != 0 {
                                    dav1d_get_sbits(gb, 7 as libc::c_int)
                                } else {
                                    0 as libc::c_int
                                };
                                (*hdr).quant.vac_delta = if dav1d_get_bit(gb) != 0 {
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
                            (*hdr).quant.qm_y = dav1d_get_bits(gb, 4 as libc::c_int) as libc::c_int;
                            (*hdr).quant.qm_u = dav1d_get_bits(gb, 4 as libc::c_int) as libc::c_int;
                            (*hdr).quant.qm_v = if (*seqhdr).separate_uv_delta_q != 0 {
                                dav1d_get_bits(gb, 4 as libc::c_int) as libc::c_int
                            } else {
                                (*hdr).quant.qm_u
                            };
                        }
                        (*hdr).segmentation.enabled = dav1d_get_bit(gb) as libc::c_int;
                        if (*hdr).segmentation.enabled != 0 {
                            if (*hdr).primary_ref_frame == 7 {
                                (*hdr).segmentation.update_map = 1 as libc::c_int;
                                (*hdr).segmentation.temporal = 0 as libc::c_int;
                                (*hdr).segmentation.update_data = 1 as libc::c_int;
                            } else {
                                (*hdr).segmentation.update_map = dav1d_get_bit(gb) as libc::c_int;
                                (*hdr).segmentation.temporal =
                                    (if (*hdr).segmentation.update_map != 0 {
                                        dav1d_get_bit(gb)
                                    } else {
                                        0 as libc::c_int as libc::c_uint
                                    }) as libc::c_int;
                                (*hdr).segmentation.update_data = dav1d_get_bit(gb) as libc::c_int;
                            }
                            if (*hdr).segmentation.update_data != 0 {
                                (*hdr).segmentation.seg_data.preskip = 0 as libc::c_int;
                                (*hdr).segmentation.seg_data.last_active_segid =
                                    -(1 as libc::c_int);
                                let mut i_10 = 0;
                                while i_10 < 8 {
                                    let seg: *mut Dav1dSegmentationData =
                                        &mut *((*hdr).segmentation.seg_data.d)
                                            .as_mut_ptr()
                                            .offset(i_10 as isize)
                                            as *mut Dav1dSegmentationData;
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
                                        (*seg).r#ref =
                                            dav1d_get_bits(gb, 3 as libc::c_int) as libc::c_int;
                                        (*hdr).segmentation.seg_data.last_active_segid = i_10;
                                        (*hdr).segmentation.seg_data.preskip = 1 as libc::c_int;
                                    } else {
                                        (*seg).r#ref = -(1 as libc::c_int);
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
                                let pri_ref = (*hdr).refidx[(*hdr).primary_ref_frame as usize];
                                if ((*c).refs[pri_ref as usize].p.p.frame_hdr).is_null() {
                                    current_block = 17922947093064792850;
                                } else {
                                    (*hdr).segmentation.seg_data =
                                        (*(*c).refs[pri_ref as usize].p.p.frame_hdr)
                                            .segmentation
                                            .seg_data;
                                    current_block = 8075351136037156718;
                                }
                            }
                        } else {
                            memset(
                                &mut (*hdr).segmentation.seg_data as *mut Dav1dSegmentationDataSet
                                    as *mut libc::c_void,
                                0 as libc::c_int,
                                ::core::mem::size_of::<Dav1dSegmentationDataSet>(),
                            );
                            let mut i_11 = 0;
                            while i_11 < 8 {
                                (*hdr).segmentation.seg_data.d[i_11 as usize].r#ref =
                                    -(1 as libc::c_int);
                                i_11 += 1;
                            }
                            current_block = 8075351136037156718;
                        }
                        match current_block {
                            17922947093064792850 => {}
                            _ => {
                                (*hdr).delta.q.present = (if (*hdr).quant.yac != 0 {
                                    dav1d_get_bit(gb)
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                })
                                    as libc::c_int;
                                (*hdr).delta.q.res_log2 = (if (*hdr).delta.q.present != 0 {
                                    dav1d_get_bits(gb, 2 as libc::c_int)
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                })
                                    as libc::c_int;
                                (*hdr).delta.lf.present = ((*hdr).delta.q.present != 0
                                    && (*hdr).allow_intrabc == 0
                                    && dav1d_get_bit(gb) != 0)
                                    as libc::c_int;
                                (*hdr).delta.lf.res_log2 = (if (*hdr).delta.lf.present != 0 {
                                    dav1d_get_bits(gb, 2 as libc::c_int)
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                })
                                    as libc::c_int;
                                (*hdr).delta.lf.multi = (if (*hdr).delta.lf.present != 0 {
                                    dav1d_get_bit(gb)
                                } else {
                                    0 as libc::c_int as libc::c_uint
                                })
                                    as libc::c_int;
                                delta_lossless = ((*hdr).quant.ydc_delta == 0
                                    && (*hdr).quant.udc_delta == 0
                                    && (*hdr).quant.uac_delta == 0
                                    && (*hdr).quant.vdc_delta == 0
                                    && (*hdr).quant.vac_delta == 0)
                                    as libc::c_int;
                                (*hdr).all_lossless = 1 as libc::c_int;
                                let mut i_12 = 0;
                                while i_12 < 8 {
                                    (*hdr).segmentation.qidx[i_12 as usize] =
                                        if (*hdr).segmentation.enabled != 0 {
                                            iclip_u8(
                                                (*hdr).quant.yac
                                                    + (*hdr).segmentation.seg_data.d[i_12 as usize]
                                                        .delta_q,
                                            )
                                        } else {
                                            (*hdr).quant.yac
                                        };
                                    (*hdr).segmentation.lossless[i_12 as usize] =
                                        ((*hdr).segmentation.qidx[i_12 as usize] == 0
                                            && delta_lossless != 0)
                                            as libc::c_int;
                                    (*hdr).all_lossless &=
                                        (*hdr).segmentation.lossless[i_12 as usize];
                                    i_12 += 1;
                                }
                                if (*hdr).all_lossless != 0 || (*hdr).allow_intrabc != 0 {
                                    (*hdr).loopfilter.level_y[1] = 0 as libc::c_int;
                                    (*hdr).loopfilter.level_y[0] = (*hdr).loopfilter.level_y[1];
                                    (*hdr).loopfilter.level_v = 0 as libc::c_int;
                                    (*hdr).loopfilter.level_u = (*hdr).loopfilter.level_v;
                                    (*hdr).loopfilter.sharpness = 0 as libc::c_int;
                                    (*hdr).loopfilter.mode_ref_delta_enabled = 1 as libc::c_int;
                                    (*hdr).loopfilter.mode_ref_delta_update = 1 as libc::c_int;
                                    (*hdr).loopfilter.mode_ref_deltas = default_mode_ref_deltas;
                                    current_block = 1424623445371442388;
                                } else {
                                    (*hdr).loopfilter.level_y[0] =
                                        dav1d_get_bits(gb, 6 as libc::c_int) as libc::c_int;
                                    (*hdr).loopfilter.level_y[1] =
                                        dav1d_get_bits(gb, 6 as libc::c_int) as libc::c_int;
                                    if (*seqhdr).monochrome == 0
                                        && ((*hdr).loopfilter.level_y[0] != 0
                                            || (*hdr).loopfilter.level_y[1] != 0)
                                    {
                                        (*hdr).loopfilter.level_u =
                                            dav1d_get_bits(gb, 6 as libc::c_int) as libc::c_int;
                                        (*hdr).loopfilter.level_v =
                                            dav1d_get_bits(gb, 6 as libc::c_int) as libc::c_int;
                                    }
                                    (*hdr).loopfilter.sharpness =
                                        dav1d_get_bits(gb, 3 as libc::c_int) as libc::c_int;
                                    if (*hdr).primary_ref_frame == 7 {
                                        (*hdr).loopfilter.mode_ref_deltas = default_mode_ref_deltas;
                                        current_block = 13291976673896753943;
                                    } else {
                                        let ref_1 =
                                            (*hdr).refidx[(*hdr).primary_ref_frame as usize];
                                        if ((*c).refs[ref_1 as usize].p.p.frame_hdr).is_null() {
                                            current_block = 17922947093064792850;
                                        } else {
                                            (*hdr).loopfilter.mode_ref_deltas =
                                                (*(*c).refs[ref_1 as usize].p.p.frame_hdr)
                                                    .loopfilter
                                                    .mode_ref_deltas;
                                            current_block = 13291976673896753943;
                                        }
                                    }
                                    match current_block {
                                        17922947093064792850 => {}
                                        _ => {
                                            (*hdr).loopfilter.mode_ref_delta_enabled =
                                                dav1d_get_bit(gb) as libc::c_int;
                                            if (*hdr).loopfilter.mode_ref_delta_enabled != 0 {
                                                (*hdr).loopfilter.mode_ref_delta_update =
                                                    dav1d_get_bit(gb) as libc::c_int;
                                                if (*hdr).loopfilter.mode_ref_delta_update != 0 {
                                                    let mut i_13 = 0;
                                                    while i_13 < 8 {
                                                        if dav1d_get_bit(gb) != 0 {
                                                            (*hdr)
                                                                .loopfilter
                                                                .mode_ref_deltas
                                                                .ref_delta
                                                                [i_13 as usize] = dav1d_get_sbits(
                                                                gb,
                                                                7 as libc::c_int,
                                                            );
                                                        }
                                                        i_13 += 1;
                                                    }
                                                    let mut i_14 = 0;
                                                    while i_14 < 2 {
                                                        if dav1d_get_bit(gb) != 0 {
                                                            (*hdr)
                                                                .loopfilter
                                                                .mode_ref_deltas
                                                                .mode_delta
                                                                [i_14 as usize] = dav1d_get_sbits(
                                                                gb,
                                                                7 as libc::c_int,
                                                            );
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
                                    17922947093064792850 => {}
                                    _ => {
                                        if (*hdr).all_lossless == 0
                                            && (*seqhdr).cdef != 0
                                            && (*hdr).allow_intrabc == 0
                                        {
                                            (*hdr).cdef.damping =
                                                (dav1d_get_bits(gb, 2 as libc::c_int))
                                                    .wrapping_add(3 as libc::c_int as libc::c_uint)
                                                    as libc::c_int;
                                            (*hdr).cdef.n_bits =
                                                dav1d_get_bits(gb, 2 as libc::c_int) as libc::c_int;
                                            let mut i_15 = 0;
                                            while i_15 < (1 as libc::c_int) << (*hdr).cdef.n_bits {
                                                (*hdr).cdef.y_strength[i_15 as usize] =
                                                    dav1d_get_bits(gb, 6 as libc::c_int)
                                                        as libc::c_int;
                                                if (*seqhdr).monochrome == 0 {
                                                    (*hdr).cdef.uv_strength[i_15 as usize] =
                                                        dav1d_get_bits(gb, 6 as libc::c_int)
                                                            as libc::c_int;
                                                }
                                                i_15 += 1;
                                            }
                                        } else {
                                            (*hdr).cdef.n_bits = 0 as libc::c_int;
                                            (*hdr).cdef.y_strength[0] = 0 as libc::c_int;
                                            (*hdr).cdef.uv_strength[0] = 0 as libc::c_int;
                                        }
                                        if ((*hdr).all_lossless == 0
                                            || (*hdr).super_res.enabled != 0)
                                            && (*seqhdr).restoration != 0
                                            && (*hdr).allow_intrabc == 0
                                        {
                                            (*hdr).restoration.type_0[0] =
                                                dav1d_get_bits(gb, 2 as libc::c_int)
                                                    as Dav1dRestorationType;
                                            if (*seqhdr).monochrome == 0 {
                                                (*hdr).restoration.type_0[1] =
                                                    dav1d_get_bits(gb, 2 as libc::c_int)
                                                        as Dav1dRestorationType;
                                                (*hdr).restoration.type_0[2] =
                                                    dav1d_get_bits(gb, 2 as libc::c_int)
                                                        as Dav1dRestorationType;
                                            } else {
                                                (*hdr).restoration.type_0[2] =
                                                    DAV1D_RESTORATION_NONE;
                                                (*hdr).restoration.type_0[1] =
                                                    (*hdr).restoration.type_0[2];
                                            }
                                            if (*hdr).restoration.type_0[0] as libc::c_uint != 0
                                                || (*hdr).restoration.type_0[1] as libc::c_uint != 0
                                                || (*hdr).restoration.type_0[2] as libc::c_uint != 0
                                            {
                                                (*hdr).restoration.unit_size[0] =
                                                    6 + (*seqhdr).sb128;
                                                if dav1d_get_bit(gb) != 0 {
                                                    (*hdr).restoration.unit_size[0] += 1;
                                                    if (*seqhdr).sb128 == 0 {
                                                        (*hdr).restoration.unit_size[0] =
                                                            ((*hdr).restoration.unit_size[0]
                                                                as libc::c_uint)
                                                                .wrapping_add(dav1d_get_bit(gb))
                                                                as libc::c_int
                                                                as libc::c_int;
                                                    }
                                                }
                                                (*hdr).restoration.unit_size[1] =
                                                    (*hdr).restoration.unit_size[0];
                                                if ((*hdr).restoration.type_0[1] as libc::c_uint
                                                    != 0
                                                    || (*hdr).restoration.type_0[2] as libc::c_uint
                                                        != 0)
                                                    && (*seqhdr).ss_hor == 1
                                                    && (*seqhdr).ss_ver == 1
                                                {
                                                    (*hdr).restoration.unit_size[1] =
                                                        ((*hdr).restoration.unit_size[1]
                                                            as libc::c_uint)
                                                            .wrapping_sub(dav1d_get_bit(gb))
                                                            as libc::c_int
                                                            as libc::c_int;
                                                }
                                            } else {
                                                (*hdr).restoration.unit_size[0] = 8 as libc::c_int;
                                            }
                                        } else {
                                            (*hdr).restoration.type_0[0] = DAV1D_RESTORATION_NONE;
                                            (*hdr).restoration.type_0[1] = DAV1D_RESTORATION_NONE;
                                            (*hdr).restoration.type_0[2] = DAV1D_RESTORATION_NONE;
                                        }
                                        (*hdr).txfm_mode = (if (*hdr).all_lossless != 0 {
                                            DAV1D_TX_4X4_ONLY as libc::c_int
                                        } else if dav1d_get_bit(gb) != 0 {
                                            DAV1D_TX_SWITCHABLE as libc::c_int
                                        } else {
                                            DAV1D_TX_LARGEST as libc::c_int
                                        })
                                            as Dav1dTxfmMode;
                                        (*hdr).switchable_comp_refs = (if (*hdr).frame_type
                                            as libc::c_uint
                                            & 1 as libc::c_uint
                                            != 0
                                        {
                                            dav1d_get_bit(gb)
                                        } else {
                                            0 as libc::c_int as libc::c_uint
                                        })
                                            as libc::c_int;
                                        (*hdr).skip_mode_allowed = 0 as libc::c_int;
                                        if (*hdr).switchable_comp_refs != 0
                                            && (*hdr).frame_type as libc::c_uint & 1 as libc::c_uint
                                                != 0
                                            && (*seqhdr).order_hint != 0
                                        {
                                            let poc: libc::c_uint =
                                                (*hdr).frame_offset as libc::c_uint;
                                            let mut off_before: libc::c_uint =
                                                0xffffffff as libc::c_uint;
                                            let mut off_after = -(1 as libc::c_int);
                                            let mut off_before_idx = 0;
                                            let mut off_after_idx = 0;
                                            let mut i_16 = 0;
                                            loop {
                                                if !(i_16 < 7) {
                                                    current_block = 10953711258009896266;
                                                    break;
                                                }
                                                if ((*c).refs
                                                    [(*hdr).refidx[i_16 as usize] as usize]
                                                    .p
                                                    .p
                                                    .frame_hdr)
                                                    .is_null()
                                                {
                                                    current_block = 17922947093064792850;
                                                    break;
                                                }
                                                let refpoc: libc::c_uint = (*(*c).refs
                                                    [(*hdr).refidx[i_16 as usize] as usize]
                                                    .p
                                                    .p
                                                    .frame_hdr)
                                                    .frame_offset
                                                    as libc::c_uint;
                                                let diff = get_poc_diff(
                                                    (*seqhdr).order_hint_n_bits,
                                                    refpoc as libc::c_int,
                                                    poc as libc::c_int,
                                                );
                                                if diff > 0 {
                                                    if off_after == -(1 as libc::c_int)
                                                        || get_poc_diff(
                                                            (*seqhdr).order_hint_n_bits,
                                                            off_after,
                                                            refpoc as libc::c_int,
                                                        ) > 0
                                                    {
                                                        off_after = refpoc as libc::c_int;
                                                        off_after_idx = i_16;
                                                    }
                                                } else if diff < 0
                                                    && (off_before == 0xffffffff as libc::c_uint
                                                        || get_poc_diff(
                                                            (*seqhdr).order_hint_n_bits,
                                                            refpoc as libc::c_int,
                                                            off_before as libc::c_int,
                                                        ) > 0)
                                                {
                                                    off_before = refpoc;
                                                    off_before_idx = i_16;
                                                }
                                                i_16 += 1;
                                            }
                                            match current_block {
                                                17922947093064792850 => {}
                                                _ => {
                                                    if off_before != 0xffffffff as libc::c_uint
                                                        && off_after != -(1 as libc::c_int)
                                                    {
                                                        (*hdr).skip_mode_refs[0] =
                                                            imin(off_before_idx, off_after_idx);
                                                        (*hdr).skip_mode_refs[1] =
                                                            imax(off_before_idx, off_after_idx);
                                                        (*hdr).skip_mode_allowed = 1 as libc::c_int;
                                                        current_block = 2126221883176060805;
                                                    } else if off_before
                                                        != 0xffffffff as libc::c_uint
                                                    {
                                                        let mut off_before2: libc::c_uint =
                                                            0xffffffff as libc::c_uint;
                                                        let mut off_before2_idx = 0;
                                                        let mut i_17 = 0;
                                                        loop {
                                                            if !(i_17 < 7) {
                                                                current_block = 6762054512782224738;
                                                                break;
                                                            }
                                                            if ((*c).refs[(*hdr).refidx
                                                                [i_17 as usize]
                                                                as usize]
                                                                .p
                                                                .p
                                                                .frame_hdr)
                                                                .is_null()
                                                            {
                                                                current_block =
                                                                    17922947093064792850;
                                                                break;
                                                            }
                                                            let refpoc_0: libc::c_uint = (*(*c)
                                                                .refs
                                                                [(*hdr).refidx[i_17 as usize]
                                                                    as usize]
                                                                .p
                                                                .p
                                                                .frame_hdr)
                                                                .frame_offset
                                                                as libc::c_uint;
                                                            if get_poc_diff(
                                                                (*seqhdr).order_hint_n_bits,
                                                                refpoc_0 as libc::c_int,
                                                                off_before as libc::c_int,
                                                            ) < 0
                                                            {
                                                                if off_before2
                                                                    == 0xffffffff as libc::c_uint
                                                                    || get_poc_diff(
                                                                        (*seqhdr).order_hint_n_bits,
                                                                        refpoc_0 as libc::c_int,
                                                                        off_before2 as libc::c_int,
                                                                    ) > 0
                                                                {
                                                                    off_before2 = refpoc_0;
                                                                    off_before2_idx = i_17;
                                                                }
                                                            }
                                                            i_17 += 1;
                                                        }
                                                        match current_block {
                                                            17922947093064792850 => {}
                                                            _ => {
                                                                if off_before2
                                                                    != 0xffffffff as libc::c_uint
                                                                {
                                                                    (*hdr).skip_mode_refs[0
                                                                        as libc::c_int
                                                                        as usize] = imin(
                                                                        off_before_idx,
                                                                        off_before2_idx,
                                                                    );
                                                                    (*hdr).skip_mode_refs[1
                                                                        as libc::c_int
                                                                        as usize] = imax(
                                                                        off_before_idx,
                                                                        off_before2_idx,
                                                                    );
                                                                    (*hdr).skip_mode_allowed =
                                                                        1 as libc::c_int;
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
                                            17922947093064792850 => {}
                                            _ => {
                                                (*hdr).skip_mode_enabled =
                                                    (if (*hdr).skip_mode_allowed != 0 {
                                                        dav1d_get_bit(gb)
                                                    } else {
                                                        0 as libc::c_int as libc::c_uint
                                                    })
                                                        as libc::c_int;
                                                (*hdr).warp_motion = ((*hdr).error_resilient_mode
                                                    == 0
                                                    && (*hdr).frame_type as libc::c_uint
                                                        & 1 as libc::c_uint
                                                        != 0
                                                    && (*seqhdr).warped_motion != 0
                                                    && dav1d_get_bit(gb) != 0)
                                                    as libc::c_int;
                                                (*hdr).reduced_txtp_set =
                                                    dav1d_get_bit(gb) as libc::c_int;
                                                let mut i_18 = 0;
                                                while i_18 < 7 {
                                                    (*hdr).gmv[i_18 as usize] =
                                                        dav1d_default_wm_params;
                                                    i_18 += 1;
                                                }
                                                if (*hdr).frame_type as libc::c_uint
                                                    & 1 as libc::c_uint
                                                    != 0
                                                {
                                                    let mut i_19 = 0;
                                                    loop {
                                                        if !(i_19 < 7) {
                                                            current_block = 6933758620287070692;
                                                            break;
                                                        }
                                                        (*hdr).gmv[i_19 as usize].type_0 =
                                                            (if dav1d_get_bit(gb) == 0 {
                                                                DAV1D_WM_TYPE_IDENTITY
                                                                    as libc::c_int
                                                            } else if dav1d_get_bit(gb) != 0 {
                                                                DAV1D_WM_TYPE_ROT_ZOOM
                                                                    as libc::c_int
                                                            } else if dav1d_get_bit(gb) != 0 {
                                                                DAV1D_WM_TYPE_TRANSLATION
                                                                    as libc::c_int
                                                            } else {
                                                                DAV1D_WM_TYPE_AFFINE as libc::c_int
                                                            })
                                                                as Dav1dWarpedMotionType;
                                                        if !((*hdr).gmv[i_19 as usize].type_0
                                                            as libc::c_uint
                                                            == DAV1D_WM_TYPE_IDENTITY as libc::c_int
                                                                as libc::c_uint)
                                                        {
                                                            let mut ref_gmv: *const Dav1dWarpedMotionParams = 0
                                                                as *const Dav1dWarpedMotionParams;
                                                            if (*hdr).primary_ref_frame == 7 {
                                                                ref_gmv = &dav1d_default_wm_params;
                                                            } else {
                                                                let pri_ref_0 = (*hdr).refidx[(*hdr)
                                                                    .primary_ref_frame
                                                                    as usize];
                                                                if ((*c).refs[pri_ref_0 as usize]
                                                                    .p
                                                                    .p
                                                                    .frame_hdr)
                                                                    .is_null()
                                                                {
                                                                    current_block =
                                                                        17922947093064792850;
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
                                                                    .offset(i_19 as isize)
                                                                    as *mut Dav1dWarpedMotionParams;
                                                            }
                                                            let mat: *mut int32_t =
                                                                ((*hdr).gmv[i_19 as usize].matrix)
                                                                    .as_mut_ptr();
                                                            let ref_mat: *const int32_t =
                                                                ((*ref_gmv).matrix).as_ptr();
                                                            let mut bits = 0;
                                                            let mut shift = 0;
                                                            if (*hdr).gmv[i_19 as usize].type_0
                                                                as libc::c_uint
                                                                >= DAV1D_WM_TYPE_ROT_ZOOM
                                                                    as libc::c_int
                                                                    as libc::c_uint
                                                            {
                                                                *mat.offset(
                                                                    2 as libc::c_int as isize,
                                                                ) = ((1 as libc::c_int) << 16)
                                                                    + 2 * dav1d_get_bits_subexp(
                                                                        gb,
                                                                        *ref_mat.offset(2)
                                                                            - ((1 as libc::c_int)
                                                                                << 16)
                                                                            >> 1,
                                                                        12 as libc::c_int
                                                                            as libc::c_uint,
                                                                    );
                                                                *mat.offset(
                                                                    3 as libc::c_int as isize,
                                                                ) = 2 as libc::c_int
                                                                    * dav1d_get_bits_subexp(
                                                                        gb,
                                                                        *ref_mat.offset(3) >> 1,
                                                                        12 as libc::c_int
                                                                            as libc::c_uint,
                                                                    );
                                                                bits = 12 as libc::c_int;
                                                                shift = 10 as libc::c_int;
                                                            } else {
                                                                bits = 9
                                                                    - ((*hdr).hp == 0)
                                                                        as libc::c_int;
                                                                shift = 13
                                                                    + ((*hdr).hp == 0)
                                                                        as libc::c_int;
                                                            }
                                                            if (*hdr).gmv[i_19 as usize].type_0
                                                                as libc::c_uint
                                                                == DAV1D_WM_TYPE_AFFINE
                                                                    as libc::c_int
                                                                    as libc::c_uint
                                                            {
                                                                *mat.offset(
                                                                    4 as libc::c_int as isize,
                                                                ) = 2 as libc::c_int
                                                                    * dav1d_get_bits_subexp(
                                                                        gb,
                                                                        *ref_mat.offset(4) >> 1,
                                                                        12 as libc::c_int
                                                                            as libc::c_uint,
                                                                    );
                                                                *mat.offset(
                                                                    5 as libc::c_int as isize,
                                                                ) = ((1 as libc::c_int) << 16)
                                                                    + 2 * dav1d_get_bits_subexp(
                                                                        gb,
                                                                        *ref_mat.offset(5)
                                                                            - ((1 as libc::c_int)
                                                                                << 16)
                                                                            >> 1,
                                                                        12 as libc::c_int
                                                                            as libc::c_uint,
                                                                    );
                                                            } else {
                                                                *mat.offset(
                                                                    4 as libc::c_int as isize,
                                                                ) = -*mat.offset(3);
                                                                *mat.offset(
                                                                    5 as libc::c_int as isize,
                                                                ) = *mat.offset(2);
                                                            }
                                                            *mat.offset(
                                                                0 as libc::c_int as isize,
                                                            ) = dav1d_get_bits_subexp(
                                                                gb,
                                                                *ref_mat.offset(0) >> shift,
                                                                bits as libc::c_uint,
                                                            ) * ((1 as libc::c_int) << shift);
                                                            *mat.offset(
                                                                1 as libc::c_int as isize,
                                                            ) = dav1d_get_bits_subexp(
                                                                gb,
                                                                *ref_mat.offset(1) >> shift,
                                                                bits as libc::c_uint,
                                                            ) * ((1 as libc::c_int) << shift);
                                                        }
                                                        i_19 += 1;
                                                    }
                                                } else {
                                                    current_block = 6933758620287070692;
                                                }
                                                match current_block {
                                                    17922947093064792850 => {}
                                                    _ => {
                                                        (*hdr).film_grain.present =
                                                            ((*seqhdr).film_grain_present != 0
                                                                && ((*hdr).show_frame != 0
                                                                    || (*hdr).showable_frame != 0)
                                                                && dav1d_get_bit(gb) != 0)
                                                                as libc::c_int;
                                                        if (*hdr).film_grain.present != 0 {
                                                            let seed: libc::c_uint = dav1d_get_bits(
                                                                gb,
                                                                16 as libc::c_int,
                                                            );
                                                            (*hdr).film_grain.update =
                                                                ((*hdr).frame_type as libc::c_uint
                                                                    != DAV1D_FRAME_TYPE_INTER
                                                                        as libc::c_int
                                                                        as libc::c_uint
                                                                    || dav1d_get_bit(gb) != 0)
                                                                    as libc::c_int;
                                                            if (*hdr).film_grain.update == 0 {
                                                                let refidx = dav1d_get_bits(
                                                                    gb,
                                                                    3 as libc::c_int,
                                                                )
                                                                    as libc::c_int;
                                                                let mut i_20 = 0;
                                                                i_20 = 0 as libc::c_int;
                                                                while i_20 < 7 {
                                                                    if (*hdr).refidx[i_20 as usize]
                                                                        == refidx
                                                                    {
                                                                        break;
                                                                    }
                                                                    i_20 += 1;
                                                                }
                                                                if i_20 == 7
                                                                    || ((*c).refs[refidx as usize]
                                                                        .p
                                                                        .p
                                                                        .frame_hdr)
                                                                        .is_null()
                                                                {
                                                                    current_block =
                                                                        17922947093064792850;
                                                                } else {
                                                                    (*hdr).film_grain.data =
                                                                        (*(*c).refs
                                                                            [refidx as usize]
                                                                            .p
                                                                            .p
                                                                            .frame_hdr)
                                                                            .film_grain
                                                                            .data;
                                                                    (*hdr).film_grain.data.seed =
                                                                        seed;
                                                                    current_block =
                                                                        17095195114763350366;
                                                                }
                                                            } else {
                                                                let fgd: *mut Dav1dFilmGrainData =
                                                                    &mut (*hdr).film_grain.data;
                                                                (*fgd).seed = seed;
                                                                (*fgd).num_y_points = dav1d_get_bits(
                                                                    gb,
                                                                    4 as libc::c_int,
                                                                )
                                                                    as libc::c_int;
                                                                if (*fgd).num_y_points > 14 {
                                                                    current_block =
                                                                        17922947093064792850;
                                                                } else {
                                                                    let mut i_21 = 0;
                                                                    loop {
                                                                        if !(i_21
                                                                            < (*fgd).num_y_points)
                                                                        {
                                                                            current_block = 12030841198858789628;
                                                                            break;
                                                                        }
                                                                        (*fgd).y_points
                                                                            [i_21 as usize]
                                                                            [0 as libc::c_int
                                                                                as usize] =
                                                                            dav1d_get_bits(
                                                                                gb,
                                                                                8 as libc::c_int,
                                                                            )
                                                                                as uint8_t;
                                                                        if i_21 != 0
                                                                            && (*fgd).y_points[(i_21
                                                                                - 1)
                                                                                as usize][0]
                                                                                as libc::c_int
                                                                                >= (*fgd).y_points
                                                                                    [i_21 as usize]
                                                                                    [0]
                                                                                    as libc::c_int
                                                                        {
                                                                            current_block = 17922947093064792850;
                                                                            break;
                                                                        }
                                                                        (*fgd).y_points
                                                                            [i_21 as usize]
                                                                            [1 as libc::c_int
                                                                                as usize] =
                                                                            dav1d_get_bits(
                                                                                gb,
                                                                                8 as libc::c_int,
                                                                            )
                                                                                as uint8_t;
                                                                        i_21 += 1;
                                                                    }
                                                                    match current_block {
                                                                        17922947093064792850 => {}
                                                                        _ => {
                                                                            (*fgd)
                                                                                .chroma_scaling_from_luma = ((*seqhdr).monochrome == 0
                                                                                && dav1d_get_bit(gb) != 0) as libc::c_int;
                                                                            if (*seqhdr).monochrome != 0
                                                                                || (*fgd).chroma_scaling_from_luma != 0
                                                                                || (*seqhdr).ss_ver == 1
                                                                                    && (*seqhdr).ss_hor == 1
                                                                                    && (*fgd).num_y_points == 0
                                                                            {
                                                                                (*fgd)
                                                                                    .num_uv_points[1 as libc::c_int
                                                                                    as usize] = 0 as libc::c_int;
                                                                                (*fgd)
                                                                                    .num_uv_points[0 as libc::c_int
                                                                                    as usize] = (*fgd).num_uv_points[1];
                                                                                current_block = 8773475593684033964;
                                                                            } else {
                                                                                let mut pl = 0;
                                                                                's_1955: loop {
                                                                                    if !(pl < 2) {
                                                                                        current_block = 8773475593684033964;
                                                                                        break;
                                                                                    }
                                                                                    (*fgd)
                                                                                        .num_uv_points[pl
                                                                                        as usize] = dav1d_get_bits(gb, 4 as libc::c_int)
                                                                                        as libc::c_int;
                                                                                    if (*fgd).num_uv_points[pl as usize] > 10 {
                                                                                        current_block = 17922947093064792850;
                                                                                        break;
                                                                                    }
                                                                                    let mut i_22 = 0;
                                                                                    while i_22 < (*fgd).num_uv_points[pl as usize] {
                                                                                        (*fgd)
                                                                                            .uv_points[pl
                                                                                            as usize][i_22
                                                                                            as usize][0 as libc::c_int
                                                                                            as usize] = dav1d_get_bits(gb, 8 as libc::c_int) as uint8_t;
                                                                                        if i_22 != 0
                                                                                            && (*fgd)
                                                                                                .uv_points[pl
                                                                                                as usize][(i_22 - 1)
                                                                                                as usize][0] as libc::c_int
                                                                                                >= (*fgd)
                                                                                                    .uv_points[pl
                                                                                                    as usize][i_22 as usize][0]
                                                                                                    as libc::c_int
                                                                                        {
                                                                                            current_block = 17922947093064792850;
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
                                                                                17922947093064792850 => {}
                                                                                _ => {
                                                                                    if (*seqhdr).ss_hor == 1
                                                                                        && (*seqhdr).ss_ver == 1
                                                                                        && ((*fgd).num_uv_points[0] != 0)
                                                                                            as libc::c_int
                                                                                            != ((*fgd).num_uv_points[1] != 0)
                                                                                                as libc::c_int
                                                                                    {
                                                                                        current_block = 17922947093064792850;
                                                                                    } else {
                                                                                        (*fgd)
                                                                                            .scaling_shift = (dav1d_get_bits(gb, 2 as libc::c_int))
                                                                                            .wrapping_add(8 as libc::c_int as libc::c_uint)
                                                                                            as libc::c_int;
                                                                                        (*fgd)
                                                                                            .ar_coeff_lag = dav1d_get_bits(gb, 2 as libc::c_int)
                                                                                            as libc::c_int;
                                                                                        let num_y_pos = 2 as libc::c_int
                                                                                            * (*fgd).ar_coeff_lag
                                                                                            * ((*fgd).ar_coeff_lag + 1);
                                                                                        if (*fgd).num_y_points != 0 {
                                                                                            let mut i_23 = 0;
                                                                                            while i_23 < num_y_pos {
                                                                                                (*fgd)
                                                                                                    .ar_coeffs_y[i_23
                                                                                                    as usize] = (dav1d_get_bits(gb, 8 as libc::c_int))
                                                                                                    .wrapping_sub(128 as libc::c_int as libc::c_uint) as int8_t;
                                                                                                i_23 += 1;
                                                                                            }
                                                                                        }
                                                                                        let mut pl_0 = 0;
                                                                                        while pl_0 < 2 {
                                                                                            if (*fgd).num_uv_points[pl_0 as usize] != 0
                                                                                                || (*fgd).chroma_scaling_from_luma != 0
                                                                                            {
                                                                                                let num_uv_pos = num_y_pos
                                                                                                    + ((*fgd).num_y_points != 0) as libc::c_int;
                                                                                                let mut i_24 = 0;
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
                                                                                        let mut pl_1 = 0;
                                                                                        while pl_1 < 2 {
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
                                                                &mut (*hdr).film_grain.data
                                                                    as *mut Dav1dFilmGrainData
                                                                    as *mut libc::c_void,
                                                                0 as libc::c_int,
                                                                ::core::mem::size_of::<
                                                                    Dav1dFilmGrainData,
                                                                >(
                                                                ),
                                                            );
                                                            current_block = 17095195114763350366;
                                                        }
                                                        match current_block {
                                                            17922947093064792850 => {}
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
    dav1d_log(
        c,
        b"Error parsing frame header\n\0" as *const u8 as *const libc::c_char,
    );
    return -(22 as libc::c_int);
}
unsafe extern "C" fn parse_tile_hdr(c: *mut Dav1dContext, gb: *mut GetBits) {
    let n_tiles = (*(*c).frame_hdr).tiling.cols * (*(*c).frame_hdr).tiling.rows;
    let have_tile_pos = (if n_tiles > 1 {
        dav1d_get_bit(gb)
    } else {
        0 as libc::c_int as libc::c_uint
    }) as libc::c_int;
    if have_tile_pos != 0 {
        let n_bits = (*(*c).frame_hdr).tiling.log2_cols + (*(*c).frame_hdr).tiling.log2_rows;
        (*((*c).tile).offset((*c).n_tile_data as isize)).start =
            dav1d_get_bits(gb, n_bits) as libc::c_int;
        (*((*c).tile).offset((*c).n_tile_data as isize)).end =
            dav1d_get_bits(gb, n_bits) as libc::c_int;
    } else {
        (*((*c).tile).offset((*c).n_tile_data as isize)).start = 0 as libc::c_int;
        (*((*c).tile).offset((*c).n_tile_data as isize)).end = n_tiles - 1;
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
    if pos.wrapping_sub(init_bit_pos) > (8 as libc::c_int as libc::c_uint).wrapping_mul(obu_len) {
        dav1d_log(
            c,
            b"Overrun in OBU bit buffer into next OBU\n\0" as *const u8 as *const libc::c_char,
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
    let mut res = 0;
    dav1d_init_get_bits(&mut gb, (*in_0).data, (*in_0).sz);
    dav1d_get_bit(&mut gb);
    let type_0: Dav1dObuType = dav1d_get_bits(&mut gb, 4 as libc::c_int) as Dav1dObuType;
    let has_extension = dav1d_get_bit(&mut gb) as libc::c_int;
    let has_length_field = dav1d_get_bit(&mut gb) as libc::c_int;
    dav1d_get_bit(&mut gb);
    let mut temporal_id = 0;
    let mut spatial_id = 0;
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
        init_byte_pos = init_bit_pos >> 3;
        if !(init_bit_pos & 7 as libc::c_uint == 0 as libc::c_uint) {
            unreachable!();
        }
        if !((*in_0).sz >= init_byte_pos as size_t) {
            unreachable!();
        }
        if !(len as size_t > ((*in_0).sz).wrapping_sub(init_byte_pos as size_t)) {
            if type_0 as libc::c_uint != DAV1D_OBU_SEQ_HDR as libc::c_int as libc::c_uint
                && type_0 as libc::c_uint != DAV1D_OBU_TD as libc::c_int as libc::c_uint
                && has_extension != 0
                && (*c).operating_point_idc != 0 as libc::c_int as libc::c_uint
            {
                let in_temporal_layer =
                    ((*c).operating_point_idc >> temporal_id & 1 as libc::c_uint) as libc::c_int;
                let in_spatial_layer =
                    ((*c).operating_point_idc >> spatial_id + 8 & 1 as libc::c_uint) as libc::c_int;
                if in_temporal_layer == 0 || in_spatial_layer == 0 {
                    return len.wrapping_add(init_byte_pos) as libc::c_int;
                }
            }
            match type_0 as libc::c_uint {
                1 => {
                    let mut r#ref: *mut Dav1dRef = dav1d_ref_create_using_pool(
                        (*c).seq_hdr_pool,
                        ::core::mem::size_of::<Dav1dSequenceHeader>(),
                    );
                    if r#ref.is_null() {
                        return -(12 as libc::c_int);
                    }
                    let mut seq_hdr: *mut Dav1dSequenceHeader =
                        (*r#ref).data as *mut Dav1dSequenceHeader;
                    res = parse_seq_hdr(c, &mut gb, seq_hdr);
                    if res < 0 {
                        dav1d_ref_dec(&mut r#ref);
                        current_block = 2084488458830559219;
                    } else if check_for_overrun(c, &mut gb, init_bit_pos, len) != 0 {
                        dav1d_ref_dec(&mut r#ref);
                        current_block = 2084488458830559219;
                    } else {
                        if ((*c).seq_hdr).is_null() {
                            (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
                            (*c).frame_flags = ::core::mem::transmute::<libc::c_uint, PictureFlags>(
                                (*c).frame_flags as libc::c_uint
                                    | PICTURE_FLAG_NEW_SEQUENCE as libc::c_int as libc::c_uint,
                            );
                        } else if memcmp(
                            seq_hdr as *const libc::c_void,
                            (*c).seq_hdr as *const libc::c_void,
                            1100,
                        ) != 0
                        {
                            (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
                            (*c).mastering_display = 0 as *mut Dav1dMasteringDisplay;
                            (*c).content_light = 0 as *mut Dav1dContentLightLevel;
                            dav1d_ref_dec(&mut (*c).mastering_display_ref);
                            dav1d_ref_dec(&mut (*c).content_light_ref);
                            let mut i = 0;
                            while i < 8 {
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
                            (*c).frame_flags = ::core::mem::transmute::<libc::c_uint, PictureFlags>(
                                (*c).frame_flags as libc::c_uint
                                    | PICTURE_FLAG_NEW_SEQUENCE as libc::c_int as libc::c_uint,
                            );
                        } else if memcmp(
                            ((*seq_hdr).operating_parameter_info).as_mut_ptr()
                                as *const libc::c_void,
                            ((*(*c).seq_hdr).operating_parameter_info).as_mut_ptr()
                                as *const libc::c_void,
                            ::core::mem::size_of::<[Dav1dSequenceHeaderOperatingParameterInfo; 32]>(
                            ),
                        ) != 0
                        {
                            (*c).frame_flags = ::core::mem::transmute::<libc::c_uint, PictureFlags>(
                                (*c).frame_flags as libc::c_uint
                                    | PICTURE_FLAG_NEW_OP_PARAMS_INFO as libc::c_int
                                        as libc::c_uint,
                            );
                        }
                        dav1d_ref_dec(&mut (*c).seq_hdr_ref);
                        (*c).seq_hdr_ref = r#ref;
                        (*c).seq_hdr = seq_hdr;
                        current_block = 2704538829018177290;
                    }
                }
                7 => {
                    if !((*c).frame_hdr).is_null() {
                        current_block = 2704538829018177290;
                    } else {
                        current_block = 15432521118199951273;
                    }
                }
                6 | 3 => {
                    current_block = 15432521118199951273;
                }
                4 => {
                    current_block = 919954187481050311;
                }
                5 => {
                    let meta_type: ObuMetaType = dav1d_get_uleb128(&mut gb) as ObuMetaType;
                    let meta_type_len = ((dav1d_get_bits_pos(&mut gb)).wrapping_sub(init_bit_pos)
                        >> 3) as libc::c_int;
                    if gb.error != 0 {
                        current_block = 2084488458830559219;
                    } else {
                        match meta_type as libc::c_uint {
                            1 => {
                                let mut ref_1: *mut Dav1dRef = dav1d_ref_create(
                                    ::core::mem::size_of::<Dav1dContentLightLevel>(),
                                );
                                if ref_1.is_null() {
                                    return -(12 as libc::c_int);
                                }
                                let content_light: *mut Dav1dContentLightLevel =
                                    (*ref_1).data as *mut Dav1dContentLightLevel;
                                (*content_light).max_content_light_level =
                                    dav1d_get_bits(&mut gb, 16 as libc::c_int) as libc::c_int;
                                (*content_light).max_frame_average_light_level =
                                    dav1d_get_bits(&mut gb, 16 as libc::c_int) as libc::c_int;
                                dav1d_get_bit(&mut gb);
                                dav1d_bytealign_get_bits(&mut gb);
                                if check_for_overrun(c, &mut gb, init_bit_pos, len) != 0 {
                                    dav1d_ref_dec(&mut ref_1);
                                    current_block = 2084488458830559219;
                                } else {
                                    dav1d_ref_dec(&mut (*c).content_light_ref);
                                    (*c).content_light = content_light;
                                    (*c).content_light_ref = ref_1;
                                    current_block = 2704538829018177290;
                                }
                            }
                            2 => {
                                let mut ref_2: *mut Dav1dRef = dav1d_ref_create(
                                    ::core::mem::size_of::<Dav1dMasteringDisplay>(),
                                );
                                if ref_2.is_null() {
                                    return -(12 as libc::c_int);
                                }
                                let mastering_display: *mut Dav1dMasteringDisplay =
                                    (*ref_2).data as *mut Dav1dMasteringDisplay;
                                let mut i_1 = 0;
                                while i_1 < 3 {
                                    (*mastering_display).primaries[i_1 as usize][0] =
                                        dav1d_get_bits(&mut gb, 16 as libc::c_int) as uint16_t;
                                    (*mastering_display).primaries[i_1 as usize][1] =
                                        dav1d_get_bits(&mut gb, 16 as libc::c_int) as uint16_t;
                                    i_1 += 1;
                                }
                                (*mastering_display).white_point[0] =
                                    dav1d_get_bits(&mut gb, 16 as libc::c_int) as uint16_t;
                                (*mastering_display).white_point[1] =
                                    dav1d_get_bits(&mut gb, 16 as libc::c_int) as uint16_t;
                                (*mastering_display).max_luminance =
                                    dav1d_get_bits(&mut gb, 32 as libc::c_int);
                                (*mastering_display).min_luminance =
                                    dav1d_get_bits(&mut gb, 32 as libc::c_int);
                                dav1d_get_bit(&mut gb);
                                dav1d_bytealign_get_bits(&mut gb);
                                if check_for_overrun(c, &mut gb, init_bit_pos, len) != 0 {
                                    dav1d_ref_dec(&mut ref_2);
                                    current_block = 2084488458830559219;
                                } else {
                                    dav1d_ref_dec(&mut (*c).mastering_display_ref);
                                    (*c).mastering_display = mastering_display;
                                    (*c).mastering_display_ref = ref_2;
                                    current_block = 2704538829018177290;
                                }
                            }
                            4 => {
                                let mut payload_size = len as libc::c_int;
                                while payload_size > 0
                                    && *((*in_0).data).offset(
                                        init_byte_pos
                                            .wrapping_add(payload_size as libc::c_uint)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                            as isize,
                                    ) == 0
                                {
                                    payload_size -= 1;
                                }
                                payload_size -= 1;
                                payload_size -= meta_type_len;
                                let mut country_code_extension_byte = 0 as libc::c_int;
                                let country_code =
                                    dav1d_get_bits(&mut gb, 8 as libc::c_int) as libc::c_int;
                                payload_size -= 1;
                                if country_code == 0xff as libc::c_int {
                                    country_code_extension_byte =
                                        dav1d_get_bits(&mut gb, 8 as libc::c_int) as libc::c_int;
                                    payload_size -= 1;
                                }
                                if payload_size <= 0 {
                                    dav1d_log(
                                        c,
                                        b"Malformed ITU-T T.35 metadata message format\n\0"
                                            as *const u8
                                            as *const libc::c_char,
                                    );
                                } else {
                                    let mut ref_3: *mut Dav1dRef = dav1d_ref_create(
                                        (::core::mem::size_of::<Dav1dITUTT35>()).wrapping_add(
                                            (payload_size as size_t)
                                                .wrapping_mul(::core::mem::size_of::<uint8_t>()),
                                        ),
                                    );
                                    if ref_3.is_null() {
                                        return -(12 as libc::c_int);
                                    }
                                    let itut_t35_metadata: *mut Dav1dITUTT35 =
                                        (*ref_3).data as *mut Dav1dITUTT35;
                                    (*itut_t35_metadata).payload = &mut *itut_t35_metadata.offset(1)
                                        as *mut Dav1dITUTT35
                                        as *mut uint8_t;
                                    (*itut_t35_metadata).country_code = country_code as uint8_t;
                                    (*itut_t35_metadata).country_code_extension_byte =
                                        country_code_extension_byte as uint8_t;
                                    let mut i_2 = 0;
                                    while i_2 < payload_size {
                                        *((*itut_t35_metadata).payload).offset(i_2 as isize) =
                                            dav1d_get_bits(&mut gb, 8 as libc::c_int) as uint8_t;
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
                    (*c).frame_flags = ::core::mem::transmute::<libc::c_uint, PictureFlags>(
                        (*c).frame_flags as libc::c_uint
                            | PICTURE_FLAG_NEW_TEMPORAL_UNIT as libc::c_int as libc::c_uint,
                    );
                    current_block = 2704538829018177290;
                }
                15 => {
                    current_block = 2704538829018177290;
                }
                _ => {
                    dav1d_log(
                        c,
                        b"Unknown OBU type %d of size %u\n\0" as *const u8 as *const libc::c_char,
                        type_0 as libc::c_uint,
                        len,
                    );
                    current_block = 2704538829018177290;
                }
            }
            match current_block {
                2084488458830559219 => {}
                _ => {
                    match current_block {
                        15432521118199951273 => {
                            if global != 0 {
                                current_block = 2704538829018177290;
                            } else if ((*c).seq_hdr).is_null() {
                                current_block = 2084488458830559219;
                            } else {
                                if ((*c).frame_hdr_ref).is_null() {
                                    (*c).frame_hdr_ref = dav1d_ref_create_using_pool(
                                        (*c).frame_hdr_pool,
                                        ::core::mem::size_of::<Dav1dFrameHeader>(),
                                    );
                                    if ((*c).frame_hdr_ref).is_null() {
                                        return -(12 as libc::c_int);
                                    }
                                }
                                (*c).frame_hdr =
                                    (*(*c).frame_hdr_ref).data as *mut Dav1dFrameHeader;
                                memset(
                                    (*c).frame_hdr as *mut libc::c_void,
                                    0 as libc::c_int,
                                    ::core::mem::size_of::<Dav1dFrameHeader>(),
                                );
                                (*(*c).frame_hdr).temporal_id = temporal_id;
                                (*(*c).frame_hdr).spatial_id = spatial_id;
                                res = parse_frame_hdr(c, &mut gb);
                                if res < 0 {
                                    (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
                                    current_block = 2084488458830559219;
                                } else {
                                    let mut n = 0;
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
                                            current_block = 2084488458830559219;
                                        } else {
                                            current_block = 4216521074440650966;
                                        }
                                    } else {
                                        current_block = 4216521074440650966;
                                    }
                                    match current_block {
                                        2084488458830559219 => {}
                                        _ => {
                                            if (*c).frame_size_limit != 0
                                                && (*(*c).frame_hdr).width[1] as int64_t
                                                    * (*(*c).frame_hdr).height as int64_t
                                                    > (*c).frame_size_limit as int64_t
                                            {
                                                dav1d_log(
                                                    c,
                                                    b"Frame size %dx%d exceeds limit %u\n\0"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                    (*(*c).frame_hdr).width[1],
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
                                                current_block = 2084488458830559219;
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
                        2084488458830559219 => {}
                        _ => {
                            match current_block {
                                919954187481050311 => {
                                    if global != 0 {
                                        current_block = 2704538829018177290;
                                    } else if ((*c).frame_hdr).is_null() {
                                        current_block = 2084488458830559219;
                                    } else {
                                        if (*c).n_tile_data_alloc < (*c).n_tile_data + 1 {
                                            if (*c).n_tile_data + 1
                                                > 2147483647
                                                    / ::core::mem::size_of::<Dav1dTileGroup>()
                                                        as libc::c_ulong
                                                        as libc::c_int
                                            {
                                                current_block = 2084488458830559219;
                                            } else {
                                                let mut tile: *mut Dav1dTileGroup = realloc(
                                                    (*c).tile as *mut libc::c_void,
                                                    (((*c).n_tile_data + 1) as size_t)
                                                        .wrapping_mul(::core::mem::size_of::<
                                                            Dav1dTileGroup,
                                                        >(
                                                        )),
                                                )
                                                    as *mut Dav1dTileGroup;
                                                if tile.is_null() {
                                                    current_block = 2084488458830559219;
                                                } else {
                                                    (*c).tile = tile;
                                                    memset(
                                                        ((*c).tile)
                                                            .offset((*c).n_tile_data as isize)
                                                            as *mut libc::c_void,
                                                        0 as libc::c_int,
                                                        ::core::mem::size_of::<Dav1dTileGroup>(),
                                                    );
                                                    (*c).n_tile_data_alloc = (*c).n_tile_data + 1;
                                                    current_block = 17711149709958600598;
                                                }
                                            }
                                        } else {
                                            current_block = 17711149709958600598;
                                        }
                                        match current_block {
                                            2084488458830559219 => {}
                                            _ => {
                                                parse_tile_hdr(c, &mut gb);
                                                dav1d_bytealign_get_bits(&mut gb);
                                                if check_for_overrun(c, &mut gb, init_bit_pos, len)
                                                    != 0
                                                {
                                                    current_block = 2084488458830559219;
                                                } else {
                                                    let pkt_bytelen: libc::c_uint =
                                                        init_byte_pos.wrapping_add(len);
                                                    let bit_pos: libc::c_uint =
                                                        dav1d_get_bits_pos(&mut gb);
                                                    if !(bit_pos & 7 as libc::c_uint
                                                        == 0 as libc::c_uint)
                                                    {
                                                        unreachable!();
                                                    }
                                                    if !(pkt_bytelen >= bit_pos >> 3) {
                                                        unreachable!();
                                                    }
                                                    dav1d_data_ref(
                                                        &mut (*((*c).tile)
                                                            .offset((*c).n_tile_data as isize))
                                                        .data,
                                                        in_0,
                                                    );
                                                    let ref mut fresh0 = (*((*c).tile)
                                                        .offset((*c).n_tile_data as isize))
                                                    .data
                                                    .data;
                                                    *fresh0 =
                                                        (*fresh0).offset((bit_pos >> 3) as isize);
                                                    (*((*c).tile)
                                                        .offset((*c).n_tile_data as isize))
                                                    .data
                                                    .sz = pkt_bytelen.wrapping_sub(bit_pos >> 3)
                                                        as size_t;
                                                    if (*((*c).tile)
                                                        .offset((*c).n_tile_data as isize))
                                                    .start
                                                        > (*((*c).tile)
                                                            .offset((*c).n_tile_data as isize))
                                                        .end
                                                        || (*((*c).tile)
                                                            .offset((*c).n_tile_data as isize))
                                                        .start
                                                            != (*c).n_tiles
                                                    {
                                                        let mut i_0 = 0;
                                                        while i_0 <= (*c).n_tile_data {
                                                            dav1d_data_unref_internal(
                                                                &mut (*((*c).tile)
                                                                    .offset(i_0 as isize))
                                                                .data,
                                                            );
                                                            i_0 += 1;
                                                        }
                                                        (*c).n_tile_data = 0 as libc::c_int;
                                                        (*c).n_tiles = 0 as libc::c_int;
                                                        current_block = 2084488458830559219;
                                                    } else {
                                                        (*c).n_tiles += 1 as libc::c_int
                                                            + (*((*c).tile)
                                                                .offset((*c).n_tile_data as isize))
                                                            .end
                                                            - (*((*c).tile)
                                                                .offset((*c).n_tile_data as isize))
                                                            .start;
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
                                2084488458830559219 => {}
                                _ => {
                                    if !((*c).seq_hdr).is_null() && !((*c).frame_hdr).is_null() {
                                        if (*(*c).frame_hdr).show_existing_frame != 0 {
                                            if ((*c).refs
                                                [(*(*c).frame_hdr).existing_frame_idx as usize]
                                                .p
                                                .p
                                                .frame_hdr)
                                                .is_null()
                                            {
                                                current_block = 2084488458830559219;
                                            } else {
                                                match (*(*c).refs
                                                    [(*(*c).frame_hdr).existing_frame_idx as usize]
                                                    .p
                                                    .p
                                                    .frame_hdr)
                                                    .frame_type
                                                    as libc::c_uint
                                                {
                                                    1 | 3 => {
                                                        if (*c).decode_frame_type as libc::c_uint
                                                            > DAV1D_DECODEFRAMETYPE_REFERENCE
                                                                as libc::c_int
                                                                as libc::c_uint
                                                        {
                                                            current_block = 16317588828440302375;
                                                        } else {
                                                            current_block = 12969817083969514432;
                                                        }
                                                    }
                                                    2 => {
                                                        if (*c).decode_frame_type as libc::c_uint
                                                            > DAV1D_DECODEFRAMETYPE_INTRA
                                                                as libc::c_int
                                                                as libc::c_uint
                                                        {
                                                            current_block = 16317588828440302375;
                                                        } else {
                                                            current_block = 12969817083969514432;
                                                        }
                                                    }
                                                    _ => {
                                                        current_block = 12969817083969514432;
                                                    }
                                                }
                                                match current_block {
                                                    16317588828440302375 => {}
                                                    _ => {
                                                        if ((*c).refs[(*(*c).frame_hdr)
                                                            .existing_frame_idx
                                                            as usize]
                                                            .p
                                                            .p
                                                            .data[0])
                                                            .is_null()
                                                        {
                                                            current_block = 2084488458830559219;
                                                        } else if (*c).strict_std_compliance != 0
                                                            && (*c).refs[(*(*c).frame_hdr)
                                                                .existing_frame_idx
                                                                as usize]
                                                                .p
                                                                .showable
                                                                == 0
                                                        {
                                                            current_block = 2084488458830559219;
                                                        } else {
                                                            if (*c).n_fc == 1 as libc::c_uint {
                                                                dav1d_thread_picture_ref(
                                                                    &mut (*c).out,
                                                                    &mut (*((*c).refs)
                                                                        .as_mut_ptr()
                                                                        .offset(
                                                                            (*(*c).frame_hdr)
                                                                                .existing_frame_idx
                                                                                as isize,
                                                                        ))
                                                                    .p,
                                                                );
                                                                dav1d_data_props_copy(
                                                                    &mut (*c).out.p.m,
                                                                    &mut (*in_0).m,
                                                                );
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
                                                                pthread_mutex_lock(
                                                                    &mut (*c).task_thread.lock,
                                                                );
                                                                let fresh1 = (*c).frame_thread.next;
                                                                (*c).frame_thread.next =
                                                                    ((*c).frame_thread.next)
                                                                        .wrapping_add(1);
                                                                let next: libc::c_uint = fresh1;
                                                                if (*c).frame_thread.next
                                                                    == (*c).n_fc
                                                                {
                                                                    (*c).frame_thread.next = 0
                                                                        as libc::c_int
                                                                        as libc::c_uint;
                                                                }
                                                                let f: *mut Dav1dFrameContext =
                                                                    &mut *((*c).fc)
                                                                        .offset(next as isize)
                                                                        as *mut Dav1dFrameContext;
                                                                while (*f).n_tile_data > 0 {
                                                                    pthread_cond_wait(
                                                                        &mut (*f).task_thread.cond,
                                                                        &mut (*(*f)
                                                                            .task_thread
                                                                            .ttd)
                                                                            .lock,
                                                                    );
                                                                }
                                                                let out_delayed: *mut Dav1dThreadPicture = &mut *((*c)
                                                                    .frame_thread
                                                                    .out_delayed)
                                                                    .offset(next as isize) as *mut Dav1dThreadPicture;
                                                                if !((*out_delayed).p.data[0])
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
                                                                    let fresh2 = ::core::intrinsics::atomic_cxchg_seqcst_seqcst(
                                                                        &mut (*c).task_thread.reset_task_cur,
                                                                        *&mut first,
                                                                        (2147483647 as libc::c_int as libc::c_uint)
                                                                            .wrapping_mul(2 as libc::c_uint)
                                                                            .wrapping_add(1 as libc::c_uint),
                                                                    );
                                                                    *&mut first = fresh2.0;
                                                                    fresh2.1;
                                                                    if (*c).task_thread.cur != 0
                                                                        && (*c).task_thread.cur < (*c).n_fc
                                                                    {
                                                                        (*c)
                                                                            .task_thread
                                                                            .cur = ((*c).task_thread.cur).wrapping_sub(1);
                                                                    }
                                                                }
                                                                let error = (*f).task_thread.retval;
                                                                if error != 0 {
                                                                    (*c).cached_error = error;
                                                                    (*f).task_thread.retval =
                                                                        0 as libc::c_int;
                                                                    dav1d_data_props_copy(
                                                                        &mut (*c)
                                                                            .cached_error_props,
                                                                        &mut (*out_delayed).p.m,
                                                                    );
                                                                    dav1d_thread_picture_unref(
                                                                        out_delayed,
                                                                    );
                                                                } else if !((*out_delayed).p.data
                                                                    [0])
                                                                .is_null()
                                                                {
                                                                    let progress: libc::c_uint = ::core::intrinsics::atomic_load_relaxed(
                                                                        &mut *((*out_delayed).progress)
                                                                            .offset(1) as *mut atomic_uint,
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
                                                                    dav1d_thread_picture_unref(
                                                                        out_delayed,
                                                                    );
                                                                }
                                                                dav1d_thread_picture_ref(
                                                                    out_delayed,
                                                                    &mut (*((*c).refs)
                                                                        .as_mut_ptr()
                                                                        .offset(
                                                                            (*(*c).frame_hdr)
                                                                                .existing_frame_idx
                                                                                as isize,
                                                                        ))
                                                                    .p,
                                                                );
                                                                (*out_delayed).visible =
                                                                    1 as libc::c_int;
                                                                dav1d_data_props_copy(
                                                                    &mut (*out_delayed).p.m,
                                                                    &mut (*in_0).m,
                                                                );
                                                                pthread_mutex_unlock(
                                                                    &mut (*c).task_thread.lock,
                                                                );
                                                            }
                                                            if (*(*c).refs[(*(*c).frame_hdr)
                                                                .existing_frame_idx
                                                                as usize]
                                                                .p
                                                                .p
                                                                .frame_hdr)
                                                                .frame_type
                                                                as libc::c_uint
                                                                == DAV1D_FRAME_TYPE_KEY
                                                                    as libc::c_int
                                                                    as libc::c_uint
                                                            {
                                                                let r = (*(*c).frame_hdr)
                                                                    .existing_frame_idx;
                                                                (*c).refs[r as usize].p.showable =
                                                                    0 as libc::c_int;
                                                                let mut i_3 = 0;
                                                                while i_3 < 8 {
                                                                    if !(i_3 == r) {
                                                                        if !((*c).refs
                                                                            [i_3 as usize]
                                                                            .p
                                                                            .p
                                                                            .frame_hdr)
                                                                            .is_null()
                                                                        {
                                                                            dav1d_thread_picture_unref(
                                                                                &mut (*((*c).refs).as_mut_ptr().offset(i_3 as isize)).p,
                                                                            );
                                                                        }
                                                                        dav1d_thread_picture_ref(
                                                                            &mut (*((*c).refs)
                                                                                .as_mut_ptr()
                                                                                .offset(
                                                                                    i_3 as isize,
                                                                                ))
                                                                            .p,
                                                                            &mut (*((*c).refs)
                                                                                .as_mut_ptr()
                                                                                .offset(
                                                                                    r as isize,
                                                                                ))
                                                                            .p,
                                                                        );
                                                                        dav1d_cdf_thread_unref(
                                                                            &mut *((*c).cdf)
                                                                                .as_mut_ptr()
                                                                                .offset(
                                                                                    i_3 as isize,
                                                                                ),
                                                                        );
                                                                        dav1d_cdf_thread_ref(
                                                                            &mut *((*c).cdf)
                                                                                .as_mut_ptr()
                                                                                .offset(
                                                                                    i_3 as isize,
                                                                                ),
                                                                            &mut *((*c).cdf)
                                                                                .as_mut_ptr()
                                                                                .offset(r as isize),
                                                                        );
                                                                        dav1d_ref_dec(
                                                                            &mut (*((*c).refs)
                                                                                .as_mut_ptr()
                                                                                .offset(
                                                                                    i_3 as isize,
                                                                                ))
                                                                            .segmap,
                                                                        );
                                                                        (*c).refs[i_3 as usize]
                                                                            .segmap = (*c).refs
                                                                            [r as usize]
                                                                            .segmap;
                                                                        if !((*c).refs[r as usize]
                                                                            .segmap)
                                                                            .is_null()
                                                                        {
                                                                            dav1d_ref_inc(
                                                                                (*c).refs
                                                                                    [r as usize]
                                                                                    .segmap,
                                                                            );
                                                                        }
                                                                        dav1d_ref_dec(
                                                                            &mut (*((*c).refs)
                                                                                .as_mut_ptr()
                                                                                .offset(
                                                                                    i_3 as isize,
                                                                                ))
                                                                            .refmvs,
                                                                        );
                                                                    }
                                                                    i_3 += 1;
                                                                }
                                                            }
                                                            (*c).frame_hdr =
                                                                0 as *mut Dav1dFrameHeader;
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
                                                        > DAV1D_DECODEFRAMETYPE_REFERENCE
                                                            as libc::c_int
                                                            as libc::c_uint
                                                        || (*c).decode_frame_type as libc::c_uint
                                                            == DAV1D_DECODEFRAMETYPE_REFERENCE
                                                                as libc::c_int
                                                                as libc::c_uint
                                                            && (*(*c).frame_hdr).refresh_frame_flags
                                                                == 0
                                                    {
                                                        current_block = 16317588828440302375;
                                                    } else {
                                                        current_block = 1622976744501948573;
                                                    }
                                                }
                                                2 => {
                                                    if (*c).decode_frame_type as libc::c_uint
                                                        > DAV1D_DECODEFRAMETYPE_INTRA as libc::c_int
                                                            as libc::c_uint
                                                        || (*c).decode_frame_type as libc::c_uint
                                                            == DAV1D_DECODEFRAMETYPE_REFERENCE
                                                                as libc::c_int
                                                                as libc::c_uint
                                                            && (*(*c).frame_hdr).refresh_frame_flags
                                                                == 0
                                                    {
                                                        current_block = 16317588828440302375;
                                                    } else {
                                                        current_block = 1622976744501948573;
                                                    }
                                                }
                                                _ => {
                                                    current_block = 1622976744501948573;
                                                }
                                            }
                                            match current_block {
                                                16317588828440302375 => {}
                                                _ => {
                                                    if (*c).n_tile_data == 0 {
                                                        current_block = 2084488458830559219;
                                                    } else {
                                                        res = dav1d_submit_frame(c);
                                                        if res < 0 {
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
                                            2084488458830559219 => {}
                                            _ => {
                                                let mut i_4 = 0;
                                                while i_4 < 8 {
                                                    if (*(*c).frame_hdr).refresh_frame_flags
                                                        & (1 as libc::c_int) << i_4
                                                        != 0
                                                    {
                                                        dav1d_thread_picture_unref(
                                                            &mut (*((*c).refs)
                                                                .as_mut_ptr()
                                                                .offset(i_4 as isize))
                                                            .p,
                                                        );
                                                        (*c).refs[i_4 as usize].p.p.frame_hdr =
                                                            (*c).frame_hdr;
                                                        (*c).refs[i_4 as usize].p.p.seq_hdr =
                                                            (*c).seq_hdr;
                                                        (*c).refs[i_4 as usize].p.p.frame_hdr_ref =
                                                            (*c).frame_hdr_ref;
                                                        (*c).refs[i_4 as usize].p.p.seq_hdr_ref =
                                                            (*c).seq_hdr_ref;
                                                        dav1d_ref_inc((*c).frame_hdr_ref);
                                                        dav1d_ref_inc((*c).seq_hdr_ref);
                                                    }
                                                    i_4 += 1;
                                                }
                                                dav1d_ref_dec(&mut (*c).frame_hdr_ref);
                                                (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
                                                (*c).n_tiles = 0 as libc::c_int;
                                                return len.wrapping_add(init_byte_pos)
                                                    as libc::c_int;
                                            }
                                        }
                                    } else {
                                        current_block = 16221891950104054966;
                                    }
                                    match current_block {
                                        2084488458830559219 => {}
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
    dav1d_log(
        c,
        b"Error parsing OBU data\n\0" as *const u8 as *const libc::c_char,
    );
    return -(22 as libc::c_int);
}
