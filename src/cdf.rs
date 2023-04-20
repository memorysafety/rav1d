use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
use crate::src::align::*;
use crate::src::msac::MsacContext;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn dav1d_ref_create_using_pool(
        pool: *mut Dav1dMemPool,
        size: size_t,
    ) -> *mut Dav1dRef;
    fn dav1d_ref_dec(r#ref: *mut *mut Dav1dRef);
    static dav1d_partition_type_count: [uint8_t; 5];
}

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
use crate::src::internal::FrameTileThreadData;
use crate::src::internal::Dav1dFrameContext_task_thread;
use crate::src::internal::TaskThreadData;
use crate::include::dav1d::picture::Dav1dPicture;
use crate::include::dav1d::headers::Dav1dITUTT35;
use crate::include::dav1d::headers::Dav1dMasteringDisplay;
use crate::include::dav1d::headers::Dav1dContentLightLevel;
use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::dav1d::headers::Dav1dWarpedMotionParams;

use crate::include::dav1d::headers::DAV1D_N_SWITCHABLE_FILTERS;

use crate::include::dav1d::headers::Dav1dFilmGrainData;
use crate::include::dav1d::headers::Dav1dSequenceHeader;

use crate::src::align::Align16;

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
use crate::src::lf_mask::Av1Filter;
pub type pixel = ();
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
    pub cf: *mut libc::c_void,
    pub prog_sz: libc::c_int,
    pub pal_sz: libc::c_int,
    pub pal_idx_sz: libc::c_int,
    pub cf_sz: libc::c_int,
    pub tile_start_off: *mut libc::c_int,
}
pub type coef = ();
use crate::src::internal::CodedBlockInfo;
use crate::src::levels::Av1Block;
use crate::src::refmvs::refmvs_frame;

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
use crate::src::levels::N_BS_SIZES;
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
use crate::src::internal::Dav1dTaskContext_task_thread;
use crate::src::internal::Dav1dTaskContext_frame_thread;
use crate::src::levels::Filter2d;

use crate::src::internal::Dav1dTaskContext_scratch;
use crate::src::internal::Dav1dTaskContext_cf;
use crate::src::refmvs::refmvs_tile;

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
    pub cf: *mut libc::c_void,
}
use crate::src::internal::Dav1dTileState_tiling;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdfContext {
    pub m: CdfModeContext,
    pub kfym: Align32<[[[uint16_t; 16]; 5]; 5]>,
    pub coef: CdfCoefContext,
    pub mv: CdfMvContext,
    pub dmv: CdfMvContext,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdfMvContext {
    pub comp: [CdfMvComponent; 2],
    pub joint: Align8<[uint16_t; 4]>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdfMvComponent {
    pub classes: Align32<[uint16_t; 16]>,
    pub class0_fp: Align8<[[uint16_t; 4]; 2]>,
    pub classN_fp: Align8<[uint16_t; 4]>,
    pub class0_hp: Align4<[uint16_t; 2]>,
    pub classN_hp: Align4<[uint16_t; 2]>,
    pub class0: Align4<[uint16_t; 2]>,
    pub classN: Align4<[[uint16_t; 2]; 10]>,
    pub sign: Align4<[uint16_t; 2]>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdfCoefContext {
    pub eob_bin_16: Align16<[[[uint16_t; 8]; 2]; 2]>,
    pub eob_bin_32: Align16<[[[uint16_t; 8]; 2]; 2]>,
    pub eob_bin_64: Align16<[[[uint16_t; 8]; 2]; 2]>,
    pub eob_bin_128: Align16<[[[uint16_t; 8]; 2]; 2]>,
    pub eob_bin_256: Align32<[[[uint16_t; 16]; 2]; 2]>,
    pub eob_bin_512: Align32<[[uint16_t; 16]; 2]>,
    pub eob_bin_1024: Align32<[[uint16_t; 16]; 2]>,
    pub eob_base_tok: Align8<[[[[uint16_t; 4]; 4]; 2]; 5]>,
    pub base_tok: Align8<[[[[uint16_t; 4]; 41]; 2]; 5]>,
    pub br_tok: Align8<[[[[uint16_t; 4]; 21]; 2]; 4]>,
    pub eob_hi_bit: Align4<[[[[uint16_t; 2]; 11]; 2]; 5]>,
    pub skip: Align4<[[[uint16_t; 2]; 13]; 5]>,
    pub dc_sign: Align4<[[[uint16_t; 2]; 3]; 2]>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdfModeContext {
    pub y_mode: Align32<[[uint16_t; 16]; 4]>,
    pub uv_mode: Align32<[[[uint16_t; 16]; 13]; 2]>,
    pub wedge_idx: Align32<[[uint16_t; 16]; 9]>,
    pub partition: Align32<[[[uint16_t; 16]; 4]; 5]>,
    pub cfl_alpha: Align32<[[uint16_t; 16]; 6]>,
    pub txtp_inter1: Align32<[[uint16_t; 16]; 2]>,
    pub txtp_inter2: Align32<[uint16_t; 16]>,
    pub txtp_intra1: Align16<[[[uint16_t; 8]; 13]; 2]>,
    pub txtp_intra2: Align16<[[[uint16_t; 8]; 13]; 3]>,
    pub cfl_sign: Align16<[uint16_t; 8]>,
    pub angle_delta: Align16<[[uint16_t; 8]; 8]>,
    pub filter_intra: Align16<[uint16_t; 8]>,
    pub comp_inter_mode: Align16<[[uint16_t; 8]; 8]>,
    pub seg_id: Align16<[[uint16_t; 8]; 3]>,
    pub pal_sz: Align16<[[[uint16_t; 8]; 7]; 2]>,
    pub color_map: Align16<[[[[uint16_t; 8]; 5]; 7]; 2]>,
    pub filter: Align8<[[[uint16_t; 4]; 8]; 2]>,
    pub txsz: Align8<[[[uint16_t; 4]; 3]; 4]>,
    pub motion_mode: Align8<[[uint16_t; 4]; 22]>,
    pub delta_q: Align8<[uint16_t; 4]>,
    pub delta_lf: Align8<[[uint16_t; 4]; 5]>,
    pub interintra_mode: Align8<[[uint16_t; 4]; 4]>,
    pub restore_switchable: Align8<[uint16_t; 4]>,
    pub restore_wiener: Align4<[uint16_t; 2]>,
    pub restore_sgrproj: Align4<[uint16_t; 2]>,
    pub interintra: Align4<[[uint16_t; 2]; 7]>,
    pub interintra_wedge: Align4<[[uint16_t; 2]; 7]>,
    pub txtp_inter3: Align4<[[uint16_t; 2]; 4]>,
    pub use_filter_intra: Align4<[[uint16_t; 2]; 22]>,
    pub newmv_mode: Align4<[[uint16_t; 2]; 6]>,
    pub globalmv_mode: Align4<[[uint16_t; 2]; 2]>,
    pub refmv_mode: Align4<[[uint16_t; 2]; 6]>,
    pub drl_bit: Align4<[[uint16_t; 2]; 3]>,
    pub intra: Align4<[[uint16_t; 2]; 4]>,
    pub comp: Align4<[[uint16_t; 2]; 5]>,
    pub comp_dir: Align4<[[uint16_t; 2]; 5]>,
    pub jnt_comp: Align4<[[uint16_t; 2]; 6]>,
    pub mask_comp: Align4<[[uint16_t; 2]; 6]>,
    pub wedge_comp: Align4<[[uint16_t; 2]; 9]>,
    pub r#ref: Align4<[[[uint16_t; 2]; 3]; 6]>,
    pub comp_fwd_ref: Align4<[[[uint16_t; 2]; 3]; 3]>,
    pub comp_bwd_ref: Align4<[[[uint16_t; 2]; 3]; 2]>,
    pub comp_uni_ref: Align4<[[[uint16_t; 2]; 3]; 3]>,
    pub txpart: Align4<[[[uint16_t; 2]; 3]; 7]>,
    pub skip: Align4<[[uint16_t; 2]; 3]>,
    pub skip_mode: Align4<[[uint16_t; 2]; 3]>,
    pub seg_pred: Align4<[[uint16_t; 2]; 3]>,
    pub obmc: Align4<[[uint16_t; 2]; 22]>,
    pub pal_y: Align4<[[[uint16_t; 2]; 3]; 7]>,
    pub pal_uv: Align4<[[uint16_t; 2]; 2]>,
    pub intrabc: Align4<[uint16_t; 2]>,
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
use crate::src::looprestoration::LooprestorationParams;

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
    pub r#ref: *mut Dav1dRef,
    pub data: CdfThreadContext_data,
    pub progress: *mut atomic_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union CdfThreadContext_data {
    pub cdf: *mut CdfContext,
    pub qcat: libc::c_uint,
}
use crate::src::internal::Dav1dContext_refs;
use crate::src::picture::Dav1dThreadPicture;
use crate::src::internal::Dav1dContext_frame_thread;
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

use crate::src::levels::N_TX_SIZES;
use crate::src::levels::N_BL_LEVELS;
use crate::src::levels::N_UV_INTRA_PRED_MODES;

use crate::src::levels::N_INTRA_PRED_MODES;
use crate::src::levels::N_MV_JOINTS;

use crate::src::levels::N_COMP_INTER_PRED_MODES;
use crate::src::r#ref::dav1d_ref_inc;
use crate::include::common::intops::imin;
pub fn av1_default_cdf() -> CdfModeContext {
    let mut init = CdfModeContext {
        y_mode: [
            [
                (32768 - 22801) as uint16_t,
                (32768 - 23489) as uint16_t,
                (32768 - 24293) as uint16_t,
                (32768 - 24756) as uint16_t,
                (32768 - 25601) as uint16_t,
                (32768 - 26123) as uint16_t,
                (32768 - 26606) as uint16_t,
                (32768 - 27418) as uint16_t,
                (32768 - 27945) as uint16_t,
                (32768 - 29228) as uint16_t,
                (32768 - 29685) as uint16_t,
                (32768 - 30349) as uint16_t,
                0,
                0,
                0,
                0,
            ],
            [
                (32768 - 18673) as uint16_t,
                (32768 - 19845) as uint16_t,
                (32768 - 22631) as uint16_t,
                (32768 - 23318) as uint16_t,
                (32768 - 23950) as uint16_t,
                (32768 - 24649) as uint16_t,
                (32768 - 25527) as uint16_t,
                (32768 - 27364) as uint16_t,
                (32768 - 28152) as uint16_t,
                (32768 - 29701) as uint16_t,
                (32768 - 29984) as uint16_t,
                (32768 - 30852) as uint16_t,
                0,
                0,
                0,
                0,
            ],
            [
                (32768 - 19770) as uint16_t,
                (32768 - 20979) as uint16_t,
                (32768 - 23396) as uint16_t,
                (32768 - 23939) as uint16_t,
                (32768 - 24241) as uint16_t,
                (32768 - 24654) as uint16_t,
                (32768 - 25136) as uint16_t,
                (32768 - 27073) as uint16_t,
                (32768 - 27830) as uint16_t,
                (32768 - 29360) as uint16_t,
                (32768 - 29730) as uint16_t,
                (32768 - 30659) as uint16_t,
                0,
                0,
                0,
                0,
            ],
            [
                (32768 - 20155) as uint16_t,
                (32768 - 21301) as uint16_t,
                (32768 - 22838) as uint16_t,
                (32768 - 23178) as uint16_t,
                (32768 - 23261) as uint16_t,
                (32768 - 23533) as uint16_t,
                (32768 - 23703) as uint16_t,
                (32768 - 24804) as uint16_t,
                (32768 - 25352) as uint16_t,
                (32768 - 26575) as uint16_t,
                (32768 - 27016) as uint16_t,
                (32768 - 28049) as uint16_t,
                0,
                0,
                0,
                0,
            ],
        ].into(),
        uv_mode: [
            [
                [
                    (32768 - 22631) as uint16_t,
                    (32768 - 24152) as uint16_t,
                    (32768 - 25378) as uint16_t,
                    (32768 - 25661) as uint16_t,
                    (32768 - 25986) as uint16_t,
                    (32768 - 26520) as uint16_t,
                    (32768 - 27055) as uint16_t,
                    (32768 - 27923) as uint16_t,
                    (32768 - 28244) as uint16_t,
                    (32768 - 30059) as uint16_t,
                    (32768 - 30941) as uint16_t,
                    (32768 - 31961) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 9513) as uint16_t,
                    (32768 - 26881) as uint16_t,
                    (32768 - 26973) as uint16_t,
                    (32768 - 27046) as uint16_t,
                    (32768 - 27118) as uint16_t,
                    (32768 - 27664) as uint16_t,
                    (32768 - 27739) as uint16_t,
                    (32768 - 27824) as uint16_t,
                    (32768 - 28359) as uint16_t,
                    (32768 - 29505) as uint16_t,
                    (32768 - 29800) as uint16_t,
                    (32768 - 31796) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 9845) as uint16_t,
                    (32768 - 9915) as uint16_t,
                    (32768 - 28663) as uint16_t,
                    (32768 - 28704) as uint16_t,
                    (32768 - 28757) as uint16_t,
                    (32768 - 28780) as uint16_t,
                    (32768 - 29198) as uint16_t,
                    (32768 - 29822) as uint16_t,
                    (32768 - 29854) as uint16_t,
                    (32768 - 30764) as uint16_t,
                    (32768 - 31777) as uint16_t,
                    (32768 - 32029) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 13639) as uint16_t,
                    (32768 - 13897) as uint16_t,
                    (32768 - 14171) as uint16_t,
                    (32768 - 25331) as uint16_t,
                    (32768 - 25606) as uint16_t,
                    (32768 - 25727) as uint16_t,
                    (32768 - 25953) as uint16_t,
                    (32768 - 27148) as uint16_t,
                    (32768 - 28577) as uint16_t,
                    (32768 - 30612) as uint16_t,
                    (32768 - 31355) as uint16_t,
                    (32768 - 32493) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 9764) as uint16_t,
                    (32768 - 9835) as uint16_t,
                    (32768 - 9930) as uint16_t,
                    (32768 - 9954) as uint16_t,
                    (32768 - 25386) as uint16_t,
                    (32768 - 27053) as uint16_t,
                    (32768 - 27958) as uint16_t,
                    (32768 - 28148) as uint16_t,
                    (32768 - 28243) as uint16_t,
                    (32768 - 31101) as uint16_t,
                    (32768 - 31744) as uint16_t,
                    (32768 - 32363) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 11825) as uint16_t,
                    (32768 - 13589) as uint16_t,
                    (32768 - 13677) as uint16_t,
                    (32768 - 13720) as uint16_t,
                    (32768 - 15048) as uint16_t,
                    (32768 - 29213) as uint16_t,
                    (32768 - 29301) as uint16_t,
                    (32768 - 29458) as uint16_t,
                    (32768 - 29711) as uint16_t,
                    (32768 - 31161) as uint16_t,
                    (32768 - 31441) as uint16_t,
                    (32768 - 32550) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 14175) as uint16_t,
                    (32768 - 14399) as uint16_t,
                    (32768 - 16608) as uint16_t,
                    (32768 - 16821) as uint16_t,
                    (32768 - 17718) as uint16_t,
                    (32768 - 17775) as uint16_t,
                    (32768 - 28551) as uint16_t,
                    (32768 - 30200) as uint16_t,
                    (32768 - 30245) as uint16_t,
                    (32768 - 31837) as uint16_t,
                    (32768 - 32342) as uint16_t,
                    (32768 - 32667) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 12885) as uint16_t,
                    (32768 - 13038) as uint16_t,
                    (32768 - 14978) as uint16_t,
                    (32768 - 15590) as uint16_t,
                    (32768 - 15673) as uint16_t,
                    (32768 - 15748) as uint16_t,
                    (32768 - 16176) as uint16_t,
                    (32768 - 29128) as uint16_t,
                    (32768 - 29267) as uint16_t,
                    (32768 - 30643) as uint16_t,
                    (32768 - 31961) as uint16_t,
                    (32768 - 32461) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 12026) as uint16_t,
                    (32768 - 13661) as uint16_t,
                    (32768 - 13874) as uint16_t,
                    (32768 - 15305) as uint16_t,
                    (32768 - 15490) as uint16_t,
                    (32768 - 15726) as uint16_t,
                    (32768 - 15995) as uint16_t,
                    (32768 - 16273) as uint16_t,
                    (32768 - 28443) as uint16_t,
                    (32768 - 30388) as uint16_t,
                    (32768 - 30767) as uint16_t,
                    (32768 - 32416) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 19052) as uint16_t,
                    (32768 - 19840) as uint16_t,
                    (32768 - 20579) as uint16_t,
                    (32768 - 20916) as uint16_t,
                    (32768 - 21150) as uint16_t,
                    (32768 - 21467) as uint16_t,
                    (32768 - 21885) as uint16_t,
                    (32768 - 22719) as uint16_t,
                    (32768 - 23174) as uint16_t,
                    (32768 - 28861) as uint16_t,
                    (32768 - 30379) as uint16_t,
                    (32768 - 32175) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 18627) as uint16_t,
                    (32768 - 19649) as uint16_t,
                    (32768 - 20974) as uint16_t,
                    (32768 - 21219) as uint16_t,
                    (32768 - 21492) as uint16_t,
                    (32768 - 21816) as uint16_t,
                    (32768 - 22199) as uint16_t,
                    (32768 - 23119) as uint16_t,
                    (32768 - 23527) as uint16_t,
                    (32768 - 27053) as uint16_t,
                    (32768 - 31397) as uint16_t,
                    (32768 - 32148) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 17026) as uint16_t,
                    (32768 - 19004) as uint16_t,
                    (32768 - 19997) as uint16_t,
                    (32768 - 20339) as uint16_t,
                    (32768 - 20586) as uint16_t,
                    (32768 - 21103) as uint16_t,
                    (32768 - 21349) as uint16_t,
                    (32768 - 21907) as uint16_t,
                    (32768 - 22482) as uint16_t,
                    (32768 - 25896) as uint16_t,
                    (32768 - 26541) as uint16_t,
                    (32768 - 31819) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 12124) as uint16_t,
                    (32768 - 13759) as uint16_t,
                    (32768 - 14959) as uint16_t,
                    (32768 - 14992) as uint16_t,
                    (32768 - 15007) as uint16_t,
                    (32768 - 15051) as uint16_t,
                    (32768 - 15078) as uint16_t,
                    (32768 - 15166) as uint16_t,
                    (32768 - 15255) as uint16_t,
                    (32768 - 15753) as uint16_t,
                    (32768 - 16039) as uint16_t,
                    (32768 - 16606) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
            ],
            [
                [
                    (32768 - 10407) as uint16_t,
                    (32768 - 11208) as uint16_t,
                    (32768 - 12900) as uint16_t,
                    (32768 - 13181) as uint16_t,
                    (32768 - 13823) as uint16_t,
                    (32768 - 14175) as uint16_t,
                    (32768 - 14899) as uint16_t,
                    (32768 - 15656) as uint16_t,
                    (32768 - 15986) as uint16_t,
                    (32768 - 20086) as uint16_t,
                    (32768 - 20995) as uint16_t,
                    (32768 - 22455) as uint16_t,
                    (32768 - 24212) as uint16_t,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 4532) as uint16_t,
                    (32768 - 19780) as uint16_t,
                    (32768 - 20057) as uint16_t,
                    (32768 - 20215) as uint16_t,
                    (32768 - 20428) as uint16_t,
                    (32768 - 21071) as uint16_t,
                    (32768 - 21199) as uint16_t,
                    (32768 - 21451) as uint16_t,
                    (32768 - 22099) as uint16_t,
                    (32768 - 24228) as uint16_t,
                    (32768 - 24693) as uint16_t,
                    (32768 - 27032) as uint16_t,
                    (32768 - 29472) as uint16_t,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 5273) as uint16_t,
                    (32768 - 5379) as uint16_t,
                    (32768 - 20177) as uint16_t,
                    (32768 - 20270) as uint16_t,
                    (32768 - 20385) as uint16_t,
                    (32768 - 20439) as uint16_t,
                    (32768 - 20949) as uint16_t,
                    (32768 - 21695) as uint16_t,
                    (32768 - 21774) as uint16_t,
                    (32768 - 23138) as uint16_t,
                    (32768 - 24256) as uint16_t,
                    (32768 - 24703) as uint16_t,
                    (32768 - 26679) as uint16_t,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6740) as uint16_t,
                    (32768 - 7167) as uint16_t,
                    (32768 - 7662) as uint16_t,
                    (32768 - 14152) as uint16_t,
                    (32768 - 14536) as uint16_t,
                    (32768 - 14785) as uint16_t,
                    (32768 - 15034) as uint16_t,
                    (32768 - 16741) as uint16_t,
                    (32768 - 18371) as uint16_t,
                    (32768 - 21520) as uint16_t,
                    (32768 - 22206) as uint16_t,
                    (32768 - 23389) as uint16_t,
                    (32768 - 24182) as uint16_t,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 4987) as uint16_t,
                    (32768 - 5368) as uint16_t,
                    (32768 - 5928) as uint16_t,
                    (32768 - 6068) as uint16_t,
                    (32768 - 19114) as uint16_t,
                    (32768 - 20315) as uint16_t,
                    (32768 - 21857) as uint16_t,
                    (32768 - 22253) as uint16_t,
                    (32768 - 22411) as uint16_t,
                    (32768 - 24911) as uint16_t,
                    (32768 - 25380) as uint16_t,
                    (32768 - 26027) as uint16_t,
                    (32768 - 26376) as uint16_t,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 5370) as uint16_t,
                    (32768 - 6889) as uint16_t,
                    (32768 - 7247) as uint16_t,
                    (32768 - 7393) as uint16_t,
                    (32768 - 9498) as uint16_t,
                    (32768 - 21114) as uint16_t,
                    (32768 - 21402) as uint16_t,
                    (32768 - 21753) as uint16_t,
                    (32768 - 21981) as uint16_t,
                    (32768 - 24780) as uint16_t,
                    (32768 - 25386) as uint16_t,
                    (32768 - 26517) as uint16_t,
                    (32768 - 27176) as uint16_t,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 4816) as uint16_t,
                    (32768 - 4961) as uint16_t,
                    (32768 - 7204) as uint16_t,
                    (32768 - 7326) as uint16_t,
                    (32768 - 8765) as uint16_t,
                    (32768 - 8930) as uint16_t,
                    (32768 - 20169) as uint16_t,
                    (32768 - 20682) as uint16_t,
                    (32768 - 20803) as uint16_t,
                    (32768 - 23188) as uint16_t,
                    (32768 - 23763) as uint16_t,
                    (32768 - 24455) as uint16_t,
                    (32768 - 24940) as uint16_t,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6608) as uint16_t,
                    (32768 - 6740) as uint16_t,
                    (32768 - 8529) as uint16_t,
                    (32768 - 9049) as uint16_t,
                    (32768 - 9257) as uint16_t,
                    (32768 - 9356) as uint16_t,
                    (32768 - 9735) as uint16_t,
                    (32768 - 18827) as uint16_t,
                    (32768 - 19059) as uint16_t,
                    (32768 - 22336) as uint16_t,
                    (32768 - 23204) as uint16_t,
                    (32768 - 23964) as uint16_t,
                    (32768 - 24793) as uint16_t,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 5998) as uint16_t,
                    (32768 - 7419) as uint16_t,
                    (32768 - 7781) as uint16_t,
                    (32768 - 8933) as uint16_t,
                    (32768 - 9255) as uint16_t,
                    (32768 - 9549) as uint16_t,
                    (32768 - 9753) as uint16_t,
                    (32768 - 10417) as uint16_t,
                    (32768 - 18898) as uint16_t,
                    (32768 - 22494) as uint16_t,
                    (32768 - 23139) as uint16_t,
                    (32768 - 24764) as uint16_t,
                    (32768 - 25989) as uint16_t,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 10660) as uint16_t,
                    (32768 - 11298) as uint16_t,
                    (32768 - 12550) as uint16_t,
                    (32768 - 12957) as uint16_t,
                    (32768 - 13322) as uint16_t,
                    (32768 - 13624) as uint16_t,
                    (32768 - 14040) as uint16_t,
                    (32768 - 15004) as uint16_t,
                    (32768 - 15534) as uint16_t,
                    (32768 - 20714) as uint16_t,
                    (32768 - 21789) as uint16_t,
                    (32768 - 23443) as uint16_t,
                    (32768 - 24861) as uint16_t,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 10522) as uint16_t,
                    (32768 - 11530) as uint16_t,
                    (32768 - 12552) as uint16_t,
                    (32768 - 12963) as uint16_t,
                    (32768 - 13378) as uint16_t,
                    (32768 - 13779) as uint16_t,
                    (32768 - 14245) as uint16_t,
                    (32768 - 15235) as uint16_t,
                    (32768 - 15902) as uint16_t,
                    (32768 - 20102) as uint16_t,
                    (32768 - 22696) as uint16_t,
                    (32768 - 23774) as uint16_t,
                    (32768 - 25838) as uint16_t,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 10099) as uint16_t,
                    (32768 - 10691) as uint16_t,
                    (32768 - 12639) as uint16_t,
                    (32768 - 13049) as uint16_t,
                    (32768 - 13386) as uint16_t,
                    (32768 - 13665) as uint16_t,
                    (32768 - 14125) as uint16_t,
                    (32768 - 15163) as uint16_t,
                    (32768 - 15636) as uint16_t,
                    (32768 - 19676) as uint16_t,
                    (32768 - 20474) as uint16_t,
                    (32768 - 23519) as uint16_t,
                    (32768 - 25208) as uint16_t,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 3144) as uint16_t,
                    (32768 - 5087) as uint16_t,
                    (32768 - 7382) as uint16_t,
                    (32768 - 7504) as uint16_t,
                    (32768 - 7593) as uint16_t,
                    (32768 - 7690) as uint16_t,
                    (32768 - 7801) as uint16_t,
                    (32768 - 8064) as uint16_t,
                    (32768 - 8232) as uint16_t,
                    (32768 - 9248) as uint16_t,
                    (32768 - 9875) as uint16_t,
                    (32768 - 10521) as uint16_t,
                    (32768 - 29048) as uint16_t,
                    0,
                    0,
                    0,
                ],
            ],
        ].into(),
        wedge_idx: [
            [
                (32768 - 2438) as uint16_t,
                (32768 - 4440) as uint16_t,
                (32768 - 6599) as uint16_t,
                (32768 - 8663) as uint16_t,
                (32768 - 11005) as uint16_t,
                (32768 - 12874) as uint16_t,
                (32768 - 15751) as uint16_t,
                (32768 - 18094) as uint16_t,
                (32768 - 20359) as uint16_t,
                (32768 - 22362) as uint16_t,
                (32768 - 24127) as uint16_t,
                (32768 - 25702) as uint16_t,
                (32768 - 27752) as uint16_t,
                (32768 - 29450) as uint16_t,
                (32768 - 31171) as uint16_t,
                0,
            ],
            [
                (32768 - 806) as uint16_t,
                (32768 - 3266) as uint16_t,
                (32768 - 6005) as uint16_t,
                (32768 - 6738) as uint16_t,
                (32768 - 7218) as uint16_t,
                (32768 - 7367) as uint16_t,
                (32768 - 7771) as uint16_t,
                (32768 - 14588) as uint16_t,
                (32768 - 16323) as uint16_t,
                (32768 - 17367) as uint16_t,
                (32768 - 18452) as uint16_t,
                (32768 - 19422) as uint16_t,
                (32768 - 22839) as uint16_t,
                (32768 - 26127) as uint16_t,
                (32768 - 29629) as uint16_t,
                0,
            ],
            [
                (32768 - 2779) as uint16_t,
                (32768 - 3738) as uint16_t,
                (32768 - 4683) as uint16_t,
                (32768 - 7213) as uint16_t,
                (32768 - 7775) as uint16_t,
                (32768 - 8017) as uint16_t,
                (32768 - 8655) as uint16_t,
                (32768 - 14357) as uint16_t,
                (32768 - 17939) as uint16_t,
                (32768 - 21332) as uint16_t,
                (32768 - 24520) as uint16_t,
                (32768 - 27470) as uint16_t,
                (32768 - 29456) as uint16_t,
                (32768 - 30529) as uint16_t,
                (32768 - 31656) as uint16_t,
                0,
            ],
            [
                (32768 - 1684) as uint16_t,
                (32768 - 3625) as uint16_t,
                (32768 - 5675) as uint16_t,
                (32768 - 7108) as uint16_t,
                (32768 - 9302) as uint16_t,
                (32768 - 11274) as uint16_t,
                (32768 - 14429) as uint16_t,
                (32768 - 17144) as uint16_t,
                (32768 - 19163) as uint16_t,
                (32768 - 20961) as uint16_t,
                (32768 - 22884) as uint16_t,
                (32768 - 24471) as uint16_t,
                (32768 - 26719) as uint16_t,
                (32768 - 28714) as uint16_t,
                (32768 - 30877) as uint16_t,
                0,
            ],
            [
                (32768 - 1142) as uint16_t,
                (32768 - 3491) as uint16_t,
                (32768 - 6277) as uint16_t,
                (32768 - 7314) as uint16_t,
                (32768 - 8089) as uint16_t,
                (32768 - 8355) as uint16_t,
                (32768 - 9023) as uint16_t,
                (32768 - 13624) as uint16_t,
                (32768 - 15369) as uint16_t,
                (32768 - 16730) as uint16_t,
                (32768 - 18114) as uint16_t,
                (32768 - 19313) as uint16_t,
                (32768 - 22521) as uint16_t,
                (32768 - 26012) as uint16_t,
                (32768 - 29550) as uint16_t,
                0,
            ],
            [
                (32768 - 2742) as uint16_t,
                (32768 - 4195) as uint16_t,
                (32768 - 5727) as uint16_t,
                (32768 - 8035) as uint16_t,
                (32768 - 8980) as uint16_t,
                (32768 - 9336) as uint16_t,
                (32768 - 10146) as uint16_t,
                (32768 - 14124) as uint16_t,
                (32768 - 17270) as uint16_t,
                (32768 - 20533) as uint16_t,
                (32768 - 23434) as uint16_t,
                (32768 - 25972) as uint16_t,
                (32768 - 27944) as uint16_t,
                (32768 - 29570) as uint16_t,
                (32768 - 31416) as uint16_t,
                0,
            ],
            [
                (32768 - 1727) as uint16_t,
                (32768 - 3948) as uint16_t,
                (32768 - 6101) as uint16_t,
                (32768 - 7796) as uint16_t,
                (32768 - 9841) as uint16_t,
                (32768 - 12344) as uint16_t,
                (32768 - 15766) as uint16_t,
                (32768 - 18944) as uint16_t,
                (32768 - 20638) as uint16_t,
                (32768 - 22038) as uint16_t,
                (32768 - 23963) as uint16_t,
                (32768 - 25311) as uint16_t,
                (32768 - 26988) as uint16_t,
                (32768 - 28766) as uint16_t,
                (32768 - 31012) as uint16_t,
                0,
            ],
            [
                (32768 - 154) as uint16_t,
                (32768 - 987) as uint16_t,
                (32768 - 1925) as uint16_t,
                (32768 - 2051) as uint16_t,
                (32768 - 2088) as uint16_t,
                (32768 - 2111) as uint16_t,
                (32768 - 2151) as uint16_t,
                (32768 - 23033) as uint16_t,
                (32768 - 23703) as uint16_t,
                (32768 - 24284) as uint16_t,
                (32768 - 24985) as uint16_t,
                (32768 - 25684) as uint16_t,
                (32768 - 27259) as uint16_t,
                (32768 - 28883) as uint16_t,
                (32768 - 30911) as uint16_t,
                0,
            ],
            [
                (32768 - 1135) as uint16_t,
                (32768 - 1322) as uint16_t,
                (32768 - 1493) as uint16_t,
                (32768 - 2635) as uint16_t,
                (32768 - 2696) as uint16_t,
                (32768 - 2737) as uint16_t,
                (32768 - 2770) as uint16_t,
                (32768 - 21016) as uint16_t,
                (32768 - 22935) as uint16_t,
                (32768 - 25057) as uint16_t,
                (32768 - 27251) as uint16_t,
                (32768 - 29173) as uint16_t,
                (32768 - 30089) as uint16_t,
                (32768 - 30960) as uint16_t,
                (32768 - 31933) as uint16_t,
                0,
            ],
        ].into(),
        partition: [
            [
                [
                    (32768 - 27899) as uint16_t,
                    (32768 - 28219) as uint16_t,
                    (32768 - 28529) as uint16_t,
                    (32768 - 32484) as uint16_t,
                    (32768 - 32539) as uint16_t,
                    (32768 - 32619) as uint16_t,
                    (32768 - 32639) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6607) as uint16_t,
                    (32768 - 6990) as uint16_t,
                    (32768 - 8268) as uint16_t,
                    (32768 - 32060) as uint16_t,
                    (32768 - 32219) as uint16_t,
                    (32768 - 32338) as uint16_t,
                    (32768 - 32371) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 5429) as uint16_t,
                    (32768 - 6676) as uint16_t,
                    (32768 - 7122) as uint16_t,
                    (32768 - 32027) as uint16_t,
                    (32768 - 32227) as uint16_t,
                    (32768 - 32531) as uint16_t,
                    (32768 - 32582) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 711) as uint16_t,
                    (32768 - 966) as uint16_t,
                    (32768 - 1172) as uint16_t,
                    (32768 - 32448) as uint16_t,
                    (32768 - 32538) as uint16_t,
                    (32768 - 32617) as uint16_t,
                    (32768 - 32664) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            ],
            [
                [
                    (32768 - 20137) as uint16_t,
                    (32768 - 21547) as uint16_t,
                    (32768 - 23078) as uint16_t,
                    (32768 - 29566) as uint16_t,
                    (32768 - 29837) as uint16_t,
                    (32768 - 30261) as uint16_t,
                    (32768 - 30524) as uint16_t,
                    (32768 - 30892) as uint16_t,
                    (32768 - 31724) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6732) as uint16_t,
                    (32768 - 7490) as uint16_t,
                    (32768 - 9497) as uint16_t,
                    (32768 - 27944) as uint16_t,
                    (32768 - 28250) as uint16_t,
                    (32768 - 28515) as uint16_t,
                    (32768 - 28969) as uint16_t,
                    (32768 - 29630) as uint16_t,
                    (32768 - 30104) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 5945) as uint16_t,
                    (32768 - 7663) as uint16_t,
                    (32768 - 8348) as uint16_t,
                    (32768 - 28683) as uint16_t,
                    (32768 - 29117) as uint16_t,
                    (32768 - 29749) as uint16_t,
                    (32768 - 30064) as uint16_t,
                    (32768 - 30298) as uint16_t,
                    (32768 - 32238) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 870) as uint16_t,
                    (32768 - 1212) as uint16_t,
                    (32768 - 1487) as uint16_t,
                    (32768 - 31198) as uint16_t,
                    (32768 - 31394) as uint16_t,
                    (32768 - 31574) as uint16_t,
                    (32768 - 31743) as uint16_t,
                    (32768 - 31881) as uint16_t,
                    (32768 - 32332) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            ],
            [
                [
                    (32768 - 18462) as uint16_t,
                    (32768 - 20920) as uint16_t,
                    (32768 - 23124) as uint16_t,
                    (32768 - 27647) as uint16_t,
                    (32768 - 28227) as uint16_t,
                    (32768 - 29049) as uint16_t,
                    (32768 - 29519) as uint16_t,
                    (32768 - 30178) as uint16_t,
                    (32768 - 31544) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 7689) as uint16_t,
                    (32768 - 9060) as uint16_t,
                    (32768 - 12056) as uint16_t,
                    (32768 - 24992) as uint16_t,
                    (32768 - 25660) as uint16_t,
                    (32768 - 26182) as uint16_t,
                    (32768 - 26951) as uint16_t,
                    (32768 - 28041) as uint16_t,
                    (32768 - 29052) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6015) as uint16_t,
                    (32768 - 9009) as uint16_t,
                    (32768 - 10062) as uint16_t,
                    (32768 - 24544) as uint16_t,
                    (32768 - 25409) as uint16_t,
                    (32768 - 26545) as uint16_t,
                    (32768 - 27071) as uint16_t,
                    (32768 - 27526) as uint16_t,
                    (32768 - 32047) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 1394) as uint16_t,
                    (32768 - 2208) as uint16_t,
                    (32768 - 2796) as uint16_t,
                    (32768 - 28614) as uint16_t,
                    (32768 - 29061) as uint16_t,
                    (32768 - 29466) as uint16_t,
                    (32768 - 29840) as uint16_t,
                    (32768 - 30185) as uint16_t,
                    (32768 - 31899) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            ],
            [
                [
                    (32768 - 15597) as uint16_t,
                    (32768 - 20929) as uint16_t,
                    (32768 - 24571) as uint16_t,
                    (32768 - 26706) as uint16_t,
                    (32768 - 27664) as uint16_t,
                    (32768 - 28821) as uint16_t,
                    (32768 - 29601) as uint16_t,
                    (32768 - 30571) as uint16_t,
                    (32768 - 31902) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 7925) as uint16_t,
                    (32768 - 11043) as uint16_t,
                    (32768 - 16785) as uint16_t,
                    (32768 - 22470) as uint16_t,
                    (32768 - 23971) as uint16_t,
                    (32768 - 25043) as uint16_t,
                    (32768 - 26651) as uint16_t,
                    (32768 - 28701) as uint16_t,
                    (32768 - 29834) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 5414) as uint16_t,
                    (32768 - 13269) as uint16_t,
                    (32768 - 15111) as uint16_t,
                    (32768 - 20488) as uint16_t,
                    (32768 - 22360) as uint16_t,
                    (32768 - 24500) as uint16_t,
                    (32768 - 25537) as uint16_t,
                    (32768 - 26336) as uint16_t,
                    (32768 - 32117) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 2662) as uint16_t,
                    (32768 - 6362) as uint16_t,
                    (32768 - 8614) as uint16_t,
                    (32768 - 20860) as uint16_t,
                    (32768 - 23053) as uint16_t,
                    (32768 - 24778) as uint16_t,
                    (32768 - 26436) as uint16_t,
                    (32768 - 27829) as uint16_t,
                    (32768 - 31171) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            ],
            [
                [
                    (32768 - 19132) as uint16_t,
                    (32768 - 25510) as uint16_t,
                    (32768 - 30392) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 13928) as uint16_t,
                    (32768 - 19855) as uint16_t,
                    (32768 - 28540) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 12522) as uint16_t,
                    (32768 - 23679) as uint16_t,
                    (32768 - 28629) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 9896) as uint16_t,
                    (32768 - 18783) as uint16_t,
                    (32768 - 25853) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            ],
        ].into(),
        cfl_alpha: [
            [
                (32768 - 7637) as uint16_t,
                (32768 - 20719) as uint16_t,
                (32768 - 31401) as uint16_t,
                (32768 - 32481) as uint16_t,
                (32768 - 32657) as uint16_t,
                (32768 - 32688) as uint16_t,
                (32768 - 32692) as uint16_t,
                (32768 - 32696) as uint16_t,
                (32768 - 32700) as uint16_t,
                (32768 - 32704) as uint16_t,
                (32768 - 32708) as uint16_t,
                (32768 - 32712) as uint16_t,
                (32768 - 32716) as uint16_t,
                (32768 - 32720) as uint16_t,
                (32768 - 32724) as uint16_t,
                0,
            ],
            [
                (32768 - 14365) as uint16_t,
                (32768 - 23603) as uint16_t,
                (32768 - 28135) as uint16_t,
                (32768 - 31168) as uint16_t,
                (32768 - 32167) as uint16_t,
                (32768 - 32395) as uint16_t,
                (32768 - 32487) as uint16_t,
                (32768 - 32573) as uint16_t,
                (32768 - 32620) as uint16_t,
                (32768 - 32647) as uint16_t,
                (32768 - 32668) as uint16_t,
                (32768 - 32672) as uint16_t,
                (32768 - 32676) as uint16_t,
                (32768 - 32680) as uint16_t,
                (32768 - 32684) as uint16_t,
                0,
            ],
            [
                (32768 - 11532) as uint16_t,
                (32768 - 22380) as uint16_t,
                (32768 - 28445) as uint16_t,
                (32768 - 31360) as uint16_t,
                (32768 - 32349) as uint16_t,
                (32768 - 32523) as uint16_t,
                (32768 - 32584) as uint16_t,
                (32768 - 32649) as uint16_t,
                (32768 - 32673) as uint16_t,
                (32768 - 32677) as uint16_t,
                (32768 - 32681) as uint16_t,
                (32768 - 32685) as uint16_t,
                (32768 - 32689) as uint16_t,
                (32768 - 32693) as uint16_t,
                (32768 - 32697) as uint16_t,
                0,
            ],
            [
                (32768 - 26990) as uint16_t,
                (32768 - 31402) as uint16_t,
                (32768 - 32282) as uint16_t,
                (32768 - 32571) as uint16_t,
                (32768 - 32692) as uint16_t,
                (32768 - 32696) as uint16_t,
                (32768 - 32700) as uint16_t,
                (32768 - 32704) as uint16_t,
                (32768 - 32708) as uint16_t,
                (32768 - 32712) as uint16_t,
                (32768 - 32716) as uint16_t,
                (32768 - 32720) as uint16_t,
                (32768 - 32724) as uint16_t,
                (32768 - 32728) as uint16_t,
                (32768 - 32732) as uint16_t,
                0,
            ],
            [
                (32768 - 17248) as uint16_t,
                (32768 - 26058) as uint16_t,
                (32768 - 28904) as uint16_t,
                (32768 - 30608) as uint16_t,
                (32768 - 31305) as uint16_t,
                (32768 - 31877) as uint16_t,
                (32768 - 32126) as uint16_t,
                (32768 - 32321) as uint16_t,
                (32768 - 32394) as uint16_t,
                (32768 - 32464) as uint16_t,
                (32768 - 32516) as uint16_t,
                (32768 - 32560) as uint16_t,
                (32768 - 32576) as uint16_t,
                (32768 - 32593) as uint16_t,
                (32768 - 32622) as uint16_t,
                0,
            ],
            [
                (32768 - 14738) as uint16_t,
                (32768 - 21678) as uint16_t,
                (32768 - 25779) as uint16_t,
                (32768 - 27901) as uint16_t,
                (32768 - 29024) as uint16_t,
                (32768 - 30302) as uint16_t,
                (32768 - 30980) as uint16_t,
                (32768 - 31843) as uint16_t,
                (32768 - 32144) as uint16_t,
                (32768 - 32413) as uint16_t,
                (32768 - 32520) as uint16_t,
                (32768 - 32594) as uint16_t,
                (32768 - 32622) as uint16_t,
                (32768 - 32656) as uint16_t,
                (32768 - 32660) as uint16_t,
                0,
            ],
        ].into(),
        txtp_inter1: [
            [
                (32768 - 4458) as uint16_t,
                (32768 - 5560) as uint16_t,
                (32768 - 7695) as uint16_t,
                (32768 - 9709) as uint16_t,
                (32768 - 13330) as uint16_t,
                (32768 - 14789) as uint16_t,
                (32768 - 17537) as uint16_t,
                (32768 - 20266) as uint16_t,
                (32768 - 21504) as uint16_t,
                (32768 - 22848) as uint16_t,
                (32768 - 23934) as uint16_t,
                (32768 - 25474) as uint16_t,
                (32768 - 27727) as uint16_t,
                (32768 - 28915) as uint16_t,
                (32768 - 30631) as uint16_t,
                0,
            ],
            [
                (32768 - 1645) as uint16_t,
                (32768 - 2573) as uint16_t,
                (32768 - 4778) as uint16_t,
                (32768 - 5711) as uint16_t,
                (32768 - 7807) as uint16_t,
                (32768 - 8622) as uint16_t,
                (32768 - 10522) as uint16_t,
                (32768 - 15357) as uint16_t,
                (32768 - 17674) as uint16_t,
                (32768 - 20408) as uint16_t,
                (32768 - 22517) as uint16_t,
                (32768 - 25010) as uint16_t,
                (32768 - 27116) as uint16_t,
                (32768 - 28856) as uint16_t,
                (32768 - 30749) as uint16_t,
                0,
            ],
        ].into(),
        txtp_inter2: [
            (32768 - 770) as uint16_t,
            (32768 - 2421) as uint16_t,
            (32768 - 5225) as uint16_t,
            (32768 - 12907) as uint16_t,
            (32768 - 15819) as uint16_t,
            (32768 - 18927) as uint16_t,
            (32768 - 21561) as uint16_t,
            (32768 - 24089) as uint16_t,
            (32768 - 26595) as uint16_t,
            (32768 - 28526) as uint16_t,
            (32768 - 30529) as uint16_t,
            0,
            0,
            0,
            0,
            0,
        ].into(),
        txtp_intra1: [
            [
                [
                    (32768 - 1535) as uint16_t,
                    (32768 - 8035) as uint16_t,
                    (32768 - 9461) as uint16_t,
                    (32768 - 12751) as uint16_t,
                    (32768 - 23467) as uint16_t,
                    (32768 - 27825) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 564) as uint16_t,
                    (32768 - 3335) as uint16_t,
                    (32768 - 9709) as uint16_t,
                    (32768 - 10870) as uint16_t,
                    (32768 - 18143) as uint16_t,
                    (32768 - 28094) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 672) as uint16_t,
                    (32768 - 3247) as uint16_t,
                    (32768 - 3676) as uint16_t,
                    (32768 - 11982) as uint16_t,
                    (32768 - 19415) as uint16_t,
                    (32768 - 23127) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 5279) as uint16_t,
                    (32768 - 13885) as uint16_t,
                    (32768 - 15487) as uint16_t,
                    (32768 - 18044) as uint16_t,
                    (32768 - 23527) as uint16_t,
                    (32768 - 30252) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 4423) as uint16_t,
                    (32768 - 6074) as uint16_t,
                    (32768 - 7985) as uint16_t,
                    (32768 - 10416) as uint16_t,
                    (32768 - 25693) as uint16_t,
                    (32768 - 29298) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 1486) as uint16_t,
                    (32768 - 4241) as uint16_t,
                    (32768 - 9460) as uint16_t,
                    (32768 - 10662) as uint16_t,
                    (32768 - 16456) as uint16_t,
                    (32768 - 27694) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 439) as uint16_t,
                    (32768 - 2838) as uint16_t,
                    (32768 - 3522) as uint16_t,
                    (32768 - 6737) as uint16_t,
                    (32768 - 18058) as uint16_t,
                    (32768 - 23754) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 1190) as uint16_t,
                    (32768 - 4233) as uint16_t,
                    (32768 - 4855) as uint16_t,
                    (32768 - 11670) as uint16_t,
                    (32768 - 20281) as uint16_t,
                    (32768 - 24377) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 1045) as uint16_t,
                    (32768 - 4312) as uint16_t,
                    (32768 - 8647) as uint16_t,
                    (32768 - 10159) as uint16_t,
                    (32768 - 18644) as uint16_t,
                    (32768 - 29335) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 202) as uint16_t,
                    (32768 - 3734) as uint16_t,
                    (32768 - 4747) as uint16_t,
                    (32768 - 7298) as uint16_t,
                    (32768 - 17127) as uint16_t,
                    (32768 - 24016) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 447) as uint16_t,
                    (32768 - 4312) as uint16_t,
                    (32768 - 6819) as uint16_t,
                    (32768 - 8884) as uint16_t,
                    (32768 - 16010) as uint16_t,
                    (32768 - 23858) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 277) as uint16_t,
                    (32768 - 4369) as uint16_t,
                    (32768 - 5255) as uint16_t,
                    (32768 - 8905) as uint16_t,
                    (32768 - 16465) as uint16_t,
                    (32768 - 22271) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 3409) as uint16_t,
                    (32768 - 5436) as uint16_t,
                    (32768 - 10599) as uint16_t,
                    (32768 - 15599) as uint16_t,
                    (32768 - 19687) as uint16_t,
                    (32768 - 24040) as uint16_t,
                    0,
                    0,
                ],
            ],
            [
                [
                    (32768 - 1870) as uint16_t,
                    (32768 - 13742) as uint16_t,
                    (32768 - 14530) as uint16_t,
                    (32768 - 16498) as uint16_t,
                    (32768 - 23770) as uint16_t,
                    (32768 - 27698) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 326) as uint16_t,
                    (32768 - 8796) as uint16_t,
                    (32768 - 14632) as uint16_t,
                    (32768 - 15079) as uint16_t,
                    (32768 - 19272) as uint16_t,
                    (32768 - 27486) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 484) as uint16_t,
                    (32768 - 7576) as uint16_t,
                    (32768 - 7712) as uint16_t,
                    (32768 - 14443) as uint16_t,
                    (32768 - 19159) as uint16_t,
                    (32768 - 22591) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 1126) as uint16_t,
                    (32768 - 15340) as uint16_t,
                    (32768 - 15895) as uint16_t,
                    (32768 - 17023) as uint16_t,
                    (32768 - 20896) as uint16_t,
                    (32768 - 30279) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 655) as uint16_t,
                    (32768 - 4854) as uint16_t,
                    (32768 - 5249) as uint16_t,
                    (32768 - 5913) as uint16_t,
                    (32768 - 22099) as uint16_t,
                    (32768 - 27138) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 1299) as uint16_t,
                    (32768 - 6458) as uint16_t,
                    (32768 - 8885) as uint16_t,
                    (32768 - 9290) as uint16_t,
                    (32768 - 14851) as uint16_t,
                    (32768 - 25497) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 311) as uint16_t,
                    (32768 - 5295) as uint16_t,
                    (32768 - 5552) as uint16_t,
                    (32768 - 6885) as uint16_t,
                    (32768 - 16107) as uint16_t,
                    (32768 - 22672) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 883) as uint16_t,
                    (32768 - 8059) as uint16_t,
                    (32768 - 8270) as uint16_t,
                    (32768 - 11258) as uint16_t,
                    (32768 - 17289) as uint16_t,
                    (32768 - 21549) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 741) as uint16_t,
                    (32768 - 7580) as uint16_t,
                    (32768 - 9318) as uint16_t,
                    (32768 - 10345) as uint16_t,
                    (32768 - 16688) as uint16_t,
                    (32768 - 29046) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 110) as uint16_t,
                    (32768 - 7406) as uint16_t,
                    (32768 - 7915) as uint16_t,
                    (32768 - 9195) as uint16_t,
                    (32768 - 16041) as uint16_t,
                    (32768 - 23329) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 363) as uint16_t,
                    (32768 - 7974) as uint16_t,
                    (32768 - 9357) as uint16_t,
                    (32768 - 10673) as uint16_t,
                    (32768 - 15629) as uint16_t,
                    (32768 - 24474) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 153) as uint16_t,
                    (32768 - 7647) as uint16_t,
                    (32768 - 8112) as uint16_t,
                    (32768 - 9936) as uint16_t,
                    (32768 - 15307) as uint16_t,
                    (32768 - 19996) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 3511) as uint16_t,
                    (32768 - 6332) as uint16_t,
                    (32768 - 11165) as uint16_t,
                    (32768 - 15335) as uint16_t,
                    (32768 - 19323) as uint16_t,
                    (32768 - 23594) as uint16_t,
                    0,
                    0,
                ],
            ],
        ].into(),
        txtp_intra2: [
            [
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
            ],
            [
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 6554) as uint16_t,
                    (32768 - 13107) as uint16_t,
                    (32768 - 19661) as uint16_t,
                    (32768 - 26214) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
            ],
            [
                [
                    (32768 - 1127) as uint16_t,
                    (32768 - 12814) as uint16_t,
                    (32768 - 22772) as uint16_t,
                    (32768 - 27483) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 145) as uint16_t,
                    (32768 - 6761) as uint16_t,
                    (32768 - 11980) as uint16_t,
                    (32768 - 26667) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 362) as uint16_t,
                    (32768 - 5887) as uint16_t,
                    (32768 - 11678) as uint16_t,
                    (32768 - 16725) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 385) as uint16_t,
                    (32768 - 15213) as uint16_t,
                    (32768 - 18587) as uint16_t,
                    (32768 - 30693) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 25) as uint16_t,
                    (32768 - 2914) as uint16_t,
                    (32768 - 23134) as uint16_t,
                    (32768 - 27903) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 60) as uint16_t,
                    (32768 - 4470) as uint16_t,
                    (32768 - 11749) as uint16_t,
                    (32768 - 23991) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 37) as uint16_t,
                    (32768 - 3332) as uint16_t,
                    (32768 - 14511) as uint16_t,
                    (32768 - 21448) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 157) as uint16_t,
                    (32768 - 6320) as uint16_t,
                    (32768 - 13036) as uint16_t,
                    (32768 - 17439) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 119) as uint16_t,
                    (32768 - 6719) as uint16_t,
                    (32768 - 12906) as uint16_t,
                    (32768 - 29396) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 47) as uint16_t,
                    (32768 - 5537) as uint16_t,
                    (32768 - 12576) as uint16_t,
                    (32768 - 21499) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 269) as uint16_t,
                    (32768 - 6076) as uint16_t,
                    (32768 - 11258) as uint16_t,
                    (32768 - 23115) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 83) as uint16_t,
                    (32768 - 5615) as uint16_t,
                    (32768 - 12001) as uint16_t,
                    (32768 - 17228) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 1968) as uint16_t,
                    (32768 - 5556) as uint16_t,
                    (32768 - 12023) as uint16_t,
                    (32768 - 18547) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                ],
            ],
        ].into(),
        cfl_sign: [
            (32768 - 1418) as uint16_t,
            (32768 - 2123) as uint16_t,
            (32768 - 13340) as uint16_t,
            (32768 - 18405) as uint16_t,
            (32768 - 26972) as uint16_t,
            (32768 - 28343) as uint16_t,
            (32768 - 32294) as uint16_t,
            0,
        ].into(),
        angle_delta: [
            [
                (32768 - 2180) as uint16_t,
                (32768 - 5032) as uint16_t,
                (32768 - 7567) as uint16_t,
                (32768 - 22776) as uint16_t,
                (32768 - 26989) as uint16_t,
                (32768 - 30217) as uint16_t,
                0,
                0,
            ],
            [
                (32768 - 2301) as uint16_t,
                (32768 - 5608) as uint16_t,
                (32768 - 8801) as uint16_t,
                (32768 - 23487) as uint16_t,
                (32768 - 26974) as uint16_t,
                (32768 - 30330) as uint16_t,
                0,
                0,
            ],
            [
                (32768 - 3780) as uint16_t,
                (32768 - 11018) as uint16_t,
                (32768 - 13699) as uint16_t,
                (32768 - 19354) as uint16_t,
                (32768 - 23083) as uint16_t,
                (32768 - 31286) as uint16_t,
                0,
                0,
            ],
            [
                (32768 - 4581) as uint16_t,
                (32768 - 11226) as uint16_t,
                (32768 - 15147) as uint16_t,
                (32768 - 17138) as uint16_t,
                (32768 - 21834) as uint16_t,
                (32768 - 28397) as uint16_t,
                0,
                0,
            ],
            [
                (32768 - 1737) as uint16_t,
                (32768 - 10927) as uint16_t,
                (32768 - 14509) as uint16_t,
                (32768 - 19588) as uint16_t,
                (32768 - 22745) as uint16_t,
                (32768 - 28823) as uint16_t,
                0,
                0,
            ],
            [
                (32768 - 2664) as uint16_t,
                (32768 - 10176) as uint16_t,
                (32768 - 12485) as uint16_t,
                (32768 - 17650) as uint16_t,
                (32768 - 21600) as uint16_t,
                (32768 - 30495) as uint16_t,
                0,
                0,
            ],
            [
                (32768 - 2240) as uint16_t,
                (32768 - 11096) as uint16_t,
                (32768 - 15453) as uint16_t,
                (32768 - 20341) as uint16_t,
                (32768 - 22561) as uint16_t,
                (32768 - 28917) as uint16_t,
                0,
                0,
            ],
            [
                (32768 - 3605) as uint16_t,
                (32768 - 10428) as uint16_t,
                (32768 - 12459) as uint16_t,
                (32768 - 17676) as uint16_t,
                (32768 - 21244) as uint16_t,
                (32768 - 30655) as uint16_t,
                0,
                0,
            ],
        ].into(),
        filter_intra: [
            (32768 - 8949) as uint16_t,
            (32768 - 12776) as uint16_t,
            (32768 - 17211) as uint16_t,
            (32768 - 29558) as uint16_t,
            0,
            0,
            0,
            0,
        ].into(),
        comp_inter_mode: [
            [
                (32768 - 7760) as uint16_t,
                (32768 - 13823) as uint16_t,
                (32768 - 15808) as uint16_t,
                (32768 - 17641) as uint16_t,
                (32768 - 19156) as uint16_t,
                (32768 - 20666) as uint16_t,
                (32768 - 26891) as uint16_t,
                0,
            ],
            [
                (32768 - 10730) as uint16_t,
                (32768 - 19452) as uint16_t,
                (32768 - 21145) as uint16_t,
                (32768 - 22749) as uint16_t,
                (32768 - 24039) as uint16_t,
                (32768 - 25131) as uint16_t,
                (32768 - 28724) as uint16_t,
                0,
            ],
            [
                (32768 - 10664) as uint16_t,
                (32768 - 20221) as uint16_t,
                (32768 - 21588) as uint16_t,
                (32768 - 22906) as uint16_t,
                (32768 - 24295) as uint16_t,
                (32768 - 25387) as uint16_t,
                (32768 - 28436) as uint16_t,
                0,
            ],
            [
                (32768 - 13298) as uint16_t,
                (32768 - 16984) as uint16_t,
                (32768 - 20471) as uint16_t,
                (32768 - 24182) as uint16_t,
                (32768 - 25067) as uint16_t,
                (32768 - 25736) as uint16_t,
                (32768 - 26422) as uint16_t,
                0,
            ],
            [
                (32768 - 18904) as uint16_t,
                (32768 - 23325) as uint16_t,
                (32768 - 25242) as uint16_t,
                (32768 - 27432) as uint16_t,
                (32768 - 27898) as uint16_t,
                (32768 - 28258) as uint16_t,
                (32768 - 30758) as uint16_t,
                0,
            ],
            [
                (32768 - 10725) as uint16_t,
                (32768 - 17454) as uint16_t,
                (32768 - 20124) as uint16_t,
                (32768 - 22820) as uint16_t,
                (32768 - 24195) as uint16_t,
                (32768 - 25168) as uint16_t,
                (32768 - 26046) as uint16_t,
                0,
            ],
            [
                (32768 - 17125) as uint16_t,
                (32768 - 24273) as uint16_t,
                (32768 - 25814) as uint16_t,
                (32768 - 27492) as uint16_t,
                (32768 - 28214) as uint16_t,
                (32768 - 28704) as uint16_t,
                (32768 - 30592) as uint16_t,
                0,
            ],
            [
                (32768 - 13046) as uint16_t,
                (32768 - 23214) as uint16_t,
                (32768 - 24505) as uint16_t,
                (32768 - 25942) as uint16_t,
                (32768 - 27435) as uint16_t,
                (32768 - 28442) as uint16_t,
                (32768 - 29330) as uint16_t,
                0,
            ],
        ].into(),
        seg_id: [
            [
                (32768 - 5622) as uint16_t,
                (32768 - 7893) as uint16_t,
                (32768 - 16093) as uint16_t,
                (32768 - 18233) as uint16_t,
                (32768 - 27809) as uint16_t,
                (32768 - 28373) as uint16_t,
                (32768 - 32533) as uint16_t,
                0,
            ],
            [
                (32768 - 14274) as uint16_t,
                (32768 - 18230) as uint16_t,
                (32768 - 22557) as uint16_t,
                (32768 - 24935) as uint16_t,
                (32768 - 29980) as uint16_t,
                (32768 - 30851) as uint16_t,
                (32768 - 32344) as uint16_t,
                0,
            ],
            [
                (32768 - 27527) as uint16_t,
                (32768 - 28487) as uint16_t,
                (32768 - 28723) as uint16_t,
                (32768 - 28890) as uint16_t,
                (32768 - 32397) as uint16_t,
                (32768 - 32647) as uint16_t,
                (32768 - 32679) as uint16_t,
                0,
            ],
        ].into(),
        pal_sz: [
            [
                [
                    (32768 - 7952) as uint16_t,
                    (32768 - 13000) as uint16_t,
                    (32768 - 18149) as uint16_t,
                    (32768 - 21478) as uint16_t,
                    (32768 - 25527) as uint16_t,
                    (32768 - 29241) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 7139) as uint16_t,
                    (32768 - 11421) as uint16_t,
                    (32768 - 16195) as uint16_t,
                    (32768 - 19544) as uint16_t,
                    (32768 - 23666) as uint16_t,
                    (32768 - 28073) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 7788) as uint16_t,
                    (32768 - 12741) as uint16_t,
                    (32768 - 17325) as uint16_t,
                    (32768 - 20500) as uint16_t,
                    (32768 - 24315) as uint16_t,
                    (32768 - 28530) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 8271) as uint16_t,
                    (32768 - 14064) as uint16_t,
                    (32768 - 18246) as uint16_t,
                    (32768 - 21564) as uint16_t,
                    (32768 - 25071) as uint16_t,
                    (32768 - 28533) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 12725) as uint16_t,
                    (32768 - 19180) as uint16_t,
                    (32768 - 21863) as uint16_t,
                    (32768 - 24839) as uint16_t,
                    (32768 - 27535) as uint16_t,
                    (32768 - 30120) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 9711) as uint16_t,
                    (32768 - 14888) as uint16_t,
                    (32768 - 16923) as uint16_t,
                    (32768 - 21052) as uint16_t,
                    (32768 - 25661) as uint16_t,
                    (32768 - 27875) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 14940) as uint16_t,
                    (32768 - 20797) as uint16_t,
                    (32768 - 21678) as uint16_t,
                    (32768 - 24186) as uint16_t,
                    (32768 - 27033) as uint16_t,
                    (32768 - 28999) as uint16_t,
                    0,
                    0,
                ],
            ],
            [
                [
                    (32768 - 8713) as uint16_t,
                    (32768 - 19979) as uint16_t,
                    (32768 - 27128) as uint16_t,
                    (32768 - 29609) as uint16_t,
                    (32768 - 31331) as uint16_t,
                    (32768 - 32272) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 5839) as uint16_t,
                    (32768 - 15573) as uint16_t,
                    (32768 - 23581) as uint16_t,
                    (32768 - 26947) as uint16_t,
                    (32768 - 29848) as uint16_t,
                    (32768 - 31700) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 4426) as uint16_t,
                    (32768 - 11260) as uint16_t,
                    (32768 - 17999) as uint16_t,
                    (32768 - 21483) as uint16_t,
                    (32768 - 25863) as uint16_t,
                    (32768 - 29430) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 3228) as uint16_t,
                    (32768 - 9464) as uint16_t,
                    (32768 - 14993) as uint16_t,
                    (32768 - 18089) as uint16_t,
                    (32768 - 22523) as uint16_t,
                    (32768 - 27420) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 3768) as uint16_t,
                    (32768 - 8886) as uint16_t,
                    (32768 - 13091) as uint16_t,
                    (32768 - 17852) as uint16_t,
                    (32768 - 22495) as uint16_t,
                    (32768 - 27207) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 2464) as uint16_t,
                    (32768 - 8451) as uint16_t,
                    (32768 - 12861) as uint16_t,
                    (32768 - 21632) as uint16_t,
                    (32768 - 25525) as uint16_t,
                    (32768 - 28555) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 1269) as uint16_t,
                    (32768 - 5435) as uint16_t,
                    (32768 - 10433) as uint16_t,
                    (32768 - 18963) as uint16_t,
                    (32768 - 21700) as uint16_t,
                    (32768 - 25865) as uint16_t,
                    0,
                    0,
                ],
            ],
        ].into(),
        color_map: [
            [
                [
                    [
                        (32768 - 28710) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 16384) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 10553) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 27036) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 31603) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 27877) as uint16_t,
                        (32768 - 30490) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 11532) as uint16_t,
                        (32768 - 25697) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 6544) as uint16_t,
                        (32768 - 30234) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 23018) as uint16_t,
                        (32768 - 28072) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 31915) as uint16_t,
                        (32768 - 32385) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 25572) as uint16_t,
                        (32768 - 28046) as uint16_t,
                        (32768 - 30045) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 9478) as uint16_t,
                        (32768 - 21590) as uint16_t,
                        (32768 - 27256) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 7248) as uint16_t,
                        (32768 - 26837) as uint16_t,
                        (32768 - 29824) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 19167) as uint16_t,
                        (32768 - 24486) as uint16_t,
                        (32768 - 28349) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 31400) as uint16_t,
                        (32768 - 31825) as uint16_t,
                        (32768 - 32250) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 24779) as uint16_t,
                        (32768 - 26955) as uint16_t,
                        (32768 - 28576) as uint16_t,
                        (32768 - 30282) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 8669) as uint16_t,
                        (32768 - 20364) as uint16_t,
                        (32768 - 24073) as uint16_t,
                        (32768 - 28093) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 4255) as uint16_t,
                        (32768 - 27565) as uint16_t,
                        (32768 - 29377) as uint16_t,
                        (32768 - 31067) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 19864) as uint16_t,
                        (32768 - 23674) as uint16_t,
                        (32768 - 26716) as uint16_t,
                        (32768 - 29530) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 31646) as uint16_t,
                        (32768 - 31893) as uint16_t,
                        (32768 - 32147) as uint16_t,
                        (32768 - 32426) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 23132) as uint16_t,
                        (32768 - 25407) as uint16_t,
                        (32768 - 26970) as uint16_t,
                        (32768 - 28435) as uint16_t,
                        (32768 - 30073) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 7443) as uint16_t,
                        (32768 - 17242) as uint16_t,
                        (32768 - 20717) as uint16_t,
                        (32768 - 24762) as uint16_t,
                        (32768 - 27982) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 6300) as uint16_t,
                        (32768 - 24862) as uint16_t,
                        (32768 - 26944) as uint16_t,
                        (32768 - 28784) as uint16_t,
                        (32768 - 30671) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 18916) as uint16_t,
                        (32768 - 22895) as uint16_t,
                        (32768 - 25267) as uint16_t,
                        (32768 - 27435) as uint16_t,
                        (32768 - 29652) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 31270) as uint16_t,
                        (32768 - 31550) as uint16_t,
                        (32768 - 31808) as uint16_t,
                        (32768 - 32059) as uint16_t,
                        (32768 - 32353) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 23105) as uint16_t,
                        (32768 - 25199) as uint16_t,
                        (32768 - 26464) as uint16_t,
                        (32768 - 27684) as uint16_t,
                        (32768 - 28931) as uint16_t,
                        (32768 - 30318) as uint16_t,
                        0,
                        0,
                    ],
                    [
                        (32768 - 6950) as uint16_t,
                        (32768 - 15447) as uint16_t,
                        (32768 - 18952) as uint16_t,
                        (32768 - 22681) as uint16_t,
                        (32768 - 25567) as uint16_t,
                        (32768 - 28563) as uint16_t,
                        0,
                        0,
                    ],
                    [
                        (32768 - 7560) as uint16_t,
                        (32768 - 23474) as uint16_t,
                        (32768 - 25490) as uint16_t,
                        (32768 - 27203) as uint16_t,
                        (32768 - 28921) as uint16_t,
                        (32768 - 30708) as uint16_t,
                        0,
                        0,
                    ],
                    [
                        (32768 - 18544) as uint16_t,
                        (32768 - 22373) as uint16_t,
                        (32768 - 24457) as uint16_t,
                        (32768 - 26195) as uint16_t,
                        (32768 - 28119) as uint16_t,
                        (32768 - 30045) as uint16_t,
                        0,
                        0,
                    ],
                    [
                        (32768 - 31198) as uint16_t,
                        (32768 - 31451) as uint16_t,
                        (32768 - 31670) as uint16_t,
                        (32768 - 31882) as uint16_t,
                        (32768 - 32123) as uint16_t,
                        (32768 - 32391) as uint16_t,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 21689) as uint16_t,
                        (32768 - 23883) as uint16_t,
                        (32768 - 25163) as uint16_t,
                        (32768 - 26352) as uint16_t,
                        (32768 - 27506) as uint16_t,
                        (32768 - 28827) as uint16_t,
                        (32768 - 30195) as uint16_t,
                        0,
                    ],
                    [
                        (32768 - 6892) as uint16_t,
                        (32768 - 15385) as uint16_t,
                        (32768 - 17840) as uint16_t,
                        (32768 - 21606) as uint16_t,
                        (32768 - 24287) as uint16_t,
                        (32768 - 26753) as uint16_t,
                        (32768 - 29204) as uint16_t,
                        0,
                    ],
                    [
                        (32768 - 5651) as uint16_t,
                        (32768 - 23182) as uint16_t,
                        (32768 - 25042) as uint16_t,
                        (32768 - 26518) as uint16_t,
                        (32768 - 27982) as uint16_t,
                        (32768 - 29392) as uint16_t,
                        (32768 - 30900) as uint16_t,
                        0,
                    ],
                    [
                        (32768 - 19349) as uint16_t,
                        (32768 - 22578) as uint16_t,
                        (32768 - 24418) as uint16_t,
                        (32768 - 25994) as uint16_t,
                        (32768 - 27524) as uint16_t,
                        (32768 - 29031) as uint16_t,
                        (32768 - 30448) as uint16_t,
                        0,
                    ],
                    [
                        (32768 - 31028) as uint16_t,
                        (32768 - 31270) as uint16_t,
                        (32768 - 31504) as uint16_t,
                        (32768 - 31705) as uint16_t,
                        (32768 - 31927) as uint16_t,
                        (32768 - 32153) as uint16_t,
                        (32768 - 32392) as uint16_t,
                        0,
                    ],
                ],
            ],
            [
                [
                    [
                        (32768 - 29089) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 16384) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 8713) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 29257) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 31610) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 25257) as uint16_t,
                        (32768 - 29145) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 12287) as uint16_t,
                        (32768 - 27293) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 7033) as uint16_t,
                        (32768 - 27960) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 20145) as uint16_t,
                        (32768 - 25405) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 30608) as uint16_t,
                        (32768 - 31639) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 24210) as uint16_t,
                        (32768 - 27175) as uint16_t,
                        (32768 - 29903) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 9888) as uint16_t,
                        (32768 - 22386) as uint16_t,
                        (32768 - 27214) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 5901) as uint16_t,
                        (32768 - 26053) as uint16_t,
                        (32768 - 29293) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 18318) as uint16_t,
                        (32768 - 22152) as uint16_t,
                        (32768 - 28333) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 30459) as uint16_t,
                        (32768 - 31136) as uint16_t,
                        (32768 - 31926) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 22980) as uint16_t,
                        (32768 - 25479) as uint16_t,
                        (32768 - 27781) as uint16_t,
                        (32768 - 29986) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 8413) as uint16_t,
                        (32768 - 21408) as uint16_t,
                        (32768 - 24859) as uint16_t,
                        (32768 - 28874) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 2257) as uint16_t,
                        (32768 - 29449) as uint16_t,
                        (32768 - 30594) as uint16_t,
                        (32768 - 31598) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 19189) as uint16_t,
                        (32768 - 21202) as uint16_t,
                        (32768 - 25915) as uint16_t,
                        (32768 - 28620) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 31844) as uint16_t,
                        (32768 - 32044) as uint16_t,
                        (32768 - 32281) as uint16_t,
                        (32768 - 32518) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 22217) as uint16_t,
                        (32768 - 24567) as uint16_t,
                        (32768 - 26637) as uint16_t,
                        (32768 - 28683) as uint16_t,
                        (32768 - 30548) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 7307) as uint16_t,
                        (32768 - 16406) as uint16_t,
                        (32768 - 19636) as uint16_t,
                        (32768 - 24632) as uint16_t,
                        (32768 - 28424) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 4441) as uint16_t,
                        (32768 - 25064) as uint16_t,
                        (32768 - 26879) as uint16_t,
                        (32768 - 28942) as uint16_t,
                        (32768 - 30919) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 17210) as uint16_t,
                        (32768 - 20528) as uint16_t,
                        (32768 - 23319) as uint16_t,
                        (32768 - 26750) as uint16_t,
                        (32768 - 29582) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 30674) as uint16_t,
                        (32768 - 30953) as uint16_t,
                        (32768 - 31396) as uint16_t,
                        (32768 - 31735) as uint16_t,
                        (32768 - 32207) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 21239) as uint16_t,
                        (32768 - 23168) as uint16_t,
                        (32768 - 25044) as uint16_t,
                        (32768 - 26962) as uint16_t,
                        (32768 - 28705) as uint16_t,
                        (32768 - 30506) as uint16_t,
                        0,
                        0,
                    ],
                    [
                        (32768 - 6545) as uint16_t,
                        (32768 - 15012) as uint16_t,
                        (32768 - 18004) as uint16_t,
                        (32768 - 21817) as uint16_t,
                        (32768 - 25503) as uint16_t,
                        (32768 - 28701) as uint16_t,
                        0,
                        0,
                    ],
                    [
                        (32768 - 3448) as uint16_t,
                        (32768 - 26295) as uint16_t,
                        (32768 - 27437) as uint16_t,
                        (32768 - 28704) as uint16_t,
                        (32768 - 30126) as uint16_t,
                        (32768 - 31442) as uint16_t,
                        0,
                        0,
                    ],
                    [
                        (32768 - 15889) as uint16_t,
                        (32768 - 18323) as uint16_t,
                        (32768 - 21704) as uint16_t,
                        (32768 - 24698) as uint16_t,
                        (32768 - 26976) as uint16_t,
                        (32768 - 29690) as uint16_t,
                        0,
                        0,
                    ],
                    [
                        (32768 - 30988) as uint16_t,
                        (32768 - 31204) as uint16_t,
                        (32768 - 31479) as uint16_t,
                        (32768 - 31734) as uint16_t,
                        (32768 - 31983) as uint16_t,
                        (32768 - 32325) as uint16_t,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 21442) as uint16_t,
                        (32768 - 23288) as uint16_t,
                        (32768 - 24758) as uint16_t,
                        (32768 - 26246) as uint16_t,
                        (32768 - 27649) as uint16_t,
                        (32768 - 28980) as uint16_t,
                        (32768 - 30563) as uint16_t,
                        0,
                    ],
                    [
                        (32768 - 5863) as uint16_t,
                        (32768 - 14933) as uint16_t,
                        (32768 - 17552) as uint16_t,
                        (32768 - 20668) as uint16_t,
                        (32768 - 23683) as uint16_t,
                        (32768 - 26411) as uint16_t,
                        (32768 - 29273) as uint16_t,
                        0,
                    ],
                    [
                        (32768 - 3415) as uint16_t,
                        (32768 - 25810) as uint16_t,
                        (32768 - 26877) as uint16_t,
                        (32768 - 27990) as uint16_t,
                        (32768 - 29223) as uint16_t,
                        (32768 - 30394) as uint16_t,
                        (32768 - 31618) as uint16_t,
                        0,
                    ],
                    [
                        (32768 - 17965) as uint16_t,
                        (32768 - 20084) as uint16_t,
                        (32768 - 22232) as uint16_t,
                        (32768 - 23974) as uint16_t,
                        (32768 - 26274) as uint16_t,
                        (32768 - 28402) as uint16_t,
                        (32768 - 30390) as uint16_t,
                        0,
                    ],
                    [
                        (32768 - 31190) as uint16_t,
                        (32768 - 31329) as uint16_t,
                        (32768 - 31516) as uint16_t,
                        (32768 - 31679) as uint16_t,
                        (32768 - 31825) as uint16_t,
                        (32768 - 32026) as uint16_t,
                        (32768 - 32322) as uint16_t,
                        0,
                    ],
                ],
            ],
        ].into(),
        filter: [
            [
                [
                    (32768 - 31935) as uint16_t,
                    (32768 - 32720) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 5568) as uint16_t,
                    (32768 - 32719) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 422) as uint16_t,
                    (32768 - 2938) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 28244) as uint16_t,
                    (32768 - 32608) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 31206) as uint16_t,
                    (32768 - 31953) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 4862) as uint16_t,
                    (32768 - 32121) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 770) as uint16_t,
                    (32768 - 1152) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 20889) as uint16_t,
                    (32768 - 25637) as uint16_t,
                    0,
                    0,
                ],
            ],
            [
                [
                    (32768 - 31910) as uint16_t,
                    (32768 - 32724) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 4120) as uint16_t,
                    (32768 - 32712) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 305) as uint16_t,
                    (32768 - 2247) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 27403) as uint16_t,
                    (32768 - 32636) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 31022) as uint16_t,
                    (32768 - 32009) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 2963) as uint16_t,
                    (32768 - 32093) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 601) as uint16_t,
                    (32768 - 943) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 14969) as uint16_t,
                    (32768 - 21398) as uint16_t,
                    0,
                    0,
                ],
            ],
        ].into(),
        txsz: [
            [
                [(32768 - 19968) as uint16_t, 0, 0, 0],
                [(32768 - 19968) as uint16_t, 0, 0, 0],
                [(32768 - 24320) as uint16_t, 0, 0, 0],
            ],
            [
                [
                    (32768 - 12272) as uint16_t,
                    (32768 - 30172) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 12272) as uint16_t,
                    (32768 - 30172) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 18677) as uint16_t,
                    (32768 - 30848) as uint16_t,
                    0,
                    0,
                ],
            ],
            [
                [
                    (32768 - 12986) as uint16_t,
                    (32768 - 15180) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 12986) as uint16_t,
                    (32768 - 15180) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 24302) as uint16_t,
                    (32768 - 25602) as uint16_t,
                    0,
                    0,
                ],
            ],
            [
                [
                    (32768 - 5782) as uint16_t,
                    (32768 - 11475) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 5782) as uint16_t,
                    (32768 - 11475) as uint16_t,
                    0,
                    0,
                ],
                [
                    (32768 - 16803) as uint16_t,
                    (32768 - 22759) as uint16_t,
                    0,
                    0,
                ],
            ],
        ].into(),
        motion_mode: [
            [
                (32768 - 32507) as uint16_t,
                (32768 - 32558) as uint16_t,
                0,
                0,
            ],
            [
                (32768 - 30878) as uint16_t,
                (32768 - 31335) as uint16_t,
                0,
                0,
            ],
            [
                (32768 - 28898) as uint16_t,
                (32768 - 30397) as uint16_t,
                0,
                0,
            ],
            [
                (32768 - 29516) as uint16_t,
                (32768 - 30701) as uint16_t,
                0,
                0,
            ],
            [
                (32768 - 21679) as uint16_t,
                (32768 - 26830) as uint16_t,
                0,
                0,
            ],
            [
                (32768 - 29742) as uint16_t,
                (32768 - 31203) as uint16_t,
                0,
                0,
            ],
            [
                (32768 - 20360) as uint16_t,
                (32768 - 28062) as uint16_t,
                0,
                0,
            ],
            [
                (32768 - 26260) as uint16_t,
                (32768 - 29116) as uint16_t,
                0,
                0,
            ],
            [
                (32768 - 11606) as uint16_t,
                (32768 - 24308) as uint16_t,
                0,
                0,
            ],
            [
                (32768 - 26431) as uint16_t,
                (32768 - 30774) as uint16_t,
                0,
                0,
            ],
            [
                (32768 - 28973) as uint16_t,
                (32768 - 31594) as uint16_t,
                0,
                0,
            ],
            [
                (32768 - 5123) as uint16_t,
                (32768 - 23606) as uint16_t,
                0,
                0,
            ],
            [
                (32768 - 19419) as uint16_t,
                (32768 - 26810) as uint16_t,
                0,
                0,
            ],
            [
                (32768 - 5391) as uint16_t,
                (32768 - 25528) as uint16_t,
                0,
                0,
            ],
            [0; 4],
            [
                (32768 - 28799) as uint16_t,
                (32768 - 31390) as uint16_t,
                0,
                0,
            ],
            [
                (32768 - 4738) as uint16_t,
                (32768 - 24765) as uint16_t,
                0,
                0,
            ],
            [
                (32768 - 7651) as uint16_t,
                (32768 - 24760) as uint16_t,
                0,
                0,
            ],
            [0; 4],
            [0; 4],
            [0; 4],
            [0; 4],
        ].into(),
        delta_q: [
            (32768 - 28160) as uint16_t,
            (32768 - 32120) as uint16_t,
            (32768 - 32677) as uint16_t,
            0,
        ].into(),
        delta_lf: [
            [
                (32768 - 28160) as uint16_t,
                (32768 - 32120) as uint16_t,
                (32768 - 32677) as uint16_t,
                0,
            ],
            [
                (32768 - 28160) as uint16_t,
                (32768 - 32120) as uint16_t,
                (32768 - 32677) as uint16_t,
                0,
            ],
            [
                (32768 - 28160) as uint16_t,
                (32768 - 32120) as uint16_t,
                (32768 - 32677) as uint16_t,
                0,
            ],
            [
                (32768 - 28160) as uint16_t,
                (32768 - 32120) as uint16_t,
                (32768 - 32677) as uint16_t,
                0,
            ],
            [
                (32768 - 28160) as uint16_t,
                (32768 - 32120) as uint16_t,
                (32768 - 32677) as uint16_t,
                0,
            ],
        ].into(),
        interintra_mode: [
            [
                (32768 - 8192) as uint16_t,
                (32768 - 16384) as uint16_t,
                (32768 - 24576) as uint16_t,
                0,
            ],
            [
                (32768 - 1875) as uint16_t,
                (32768 - 11082) as uint16_t,
                (32768 - 27332) as uint16_t,
                0,
            ],
            [
                (32768 - 2473) as uint16_t,
                (32768 - 9996) as uint16_t,
                (32768 - 26388) as uint16_t,
                0,
            ],
            [
                (32768 - 4238) as uint16_t,
                (32768 - 11537) as uint16_t,
                (32768 - 25926) as uint16_t,
                0,
            ],
        ].into(),
        restore_switchable: [
            (32768 - 9413) as uint16_t,
            (32768 - 22581) as uint16_t,
            0,
            0,
        ].into(),
        restore_wiener: [(32768 - 11570) as uint16_t, 0].into(),
        restore_sgrproj: [(32768 - 16855) as uint16_t, 0].into(),
        interintra: [
            [(32768 - 16384) as uint16_t, 0],
            [(32768 - 26887) as uint16_t, 0],
            [(32768 - 27597) as uint16_t, 0],
            [(32768 - 30237) as uint16_t, 0],
            [0; 2],
            [0; 2],
            [0; 2],
        ].into(),
        interintra_wedge: [
            [(32768 - 20036) as uint16_t, 0],
            [(32768 - 24957) as uint16_t, 0],
            [(32768 - 26704) as uint16_t, 0],
            [(32768 - 27530) as uint16_t, 0],
            [(32768 - 29564) as uint16_t, 0],
            [(32768 - 29444) as uint16_t, 0],
            [(32768 - 26872) as uint16_t, 0],
        ].into(),
        txtp_inter3: [
            [(32768 - 16384) as uint16_t, 0],
            [(32768 - 4167) as uint16_t, 0],
            [(32768 - 1998) as uint16_t, 0],
            [(32768 - 748) as uint16_t, 0],
        ].into(),
        use_filter_intra: [
            [(32768 - 16384) as uint16_t, 0],
            [(32768 - 16384) as uint16_t, 0],
            [(32768 - 16384) as uint16_t, 0],
            [(32768 - 16384) as uint16_t, 0],
            [(32768 - 16384) as uint16_t, 0],
            [(32768 - 16384) as uint16_t, 0],
            [(32768 - 16384) as uint16_t, 0],
            [(32768 - 22343) as uint16_t, 0],
            [(32768 - 12756) as uint16_t, 0],
            [(32768 - 18101) as uint16_t, 0],
            [(32768 - 16384) as uint16_t, 0],
            [(32768 - 14301) as uint16_t, 0],
            [(32768 - 12408) as uint16_t, 0],
            [(32768 - 9394) as uint16_t, 0],
            [(32768 - 10368) as uint16_t, 0],
            [(32768 - 20229) as uint16_t, 0],
            [(32768 - 12551) as uint16_t, 0],
            [(32768 - 7866) as uint16_t, 0],
            [(32768 - 5893) as uint16_t, 0],
            [(32768 - 12770) as uint16_t, 0],
            [(32768 - 6743) as uint16_t, 0],
            [(32768 - 4621) as uint16_t, 0],
        ].into(),
        newmv_mode: [
            [(32768 - 24035) as uint16_t, 0],
            [(32768 - 16630) as uint16_t, 0],
            [(32768 - 15339) as uint16_t, 0],
            [(32768 - 8386) as uint16_t, 0],
            [(32768 - 12222) as uint16_t, 0],
            [(32768 - 4676) as uint16_t, 0],
        ].into(),
        globalmv_mode: [
            [(32768 - 2175) as uint16_t, 0],
            [(32768 - 1054) as uint16_t, 0],
        ].into(),
        refmv_mode: [
            [(32768 - 23974) as uint16_t, 0],
            [(32768 - 24188) as uint16_t, 0],
            [(32768 - 17848) as uint16_t, 0],
            [(32768 - 28622) as uint16_t, 0],
            [(32768 - 24312) as uint16_t, 0],
            [(32768 - 19923) as uint16_t, 0],
        ].into(),
        drl_bit: [
            [(32768 - 13104) as uint16_t, 0],
            [(32768 - 24560) as uint16_t, 0],
            [(32768 - 18945) as uint16_t, 0],
        ].into(),
        intra: [
            [(32768 - 806) as uint16_t, 0],
            [(32768 - 16662) as uint16_t, 0],
            [(32768 - 20186) as uint16_t, 0],
            [(32768 - 26538) as uint16_t, 0],
        ].into(),
        comp: [
            [(32768 - 26828) as uint16_t, 0],
            [(32768 - 24035) as uint16_t, 0],
            [(32768 - 12031) as uint16_t, 0],
            [(32768 - 10640) as uint16_t, 0],
            [(32768 - 2901) as uint16_t, 0],
        ].into(),
        comp_dir: [
            [(32768 - 1198) as uint16_t, 0],
            [(32768 - 2070) as uint16_t, 0],
            [(32768 - 9166) as uint16_t, 0],
            [(32768 - 7499) as uint16_t, 0],
            [(32768 - 22475) as uint16_t, 0],
        ].into(),
        jnt_comp: [
            [(32768 - 18244) as uint16_t, 0],
            [(32768 - 12865) as uint16_t, 0],
            [(32768 - 7053) as uint16_t, 0],
            [(32768 - 13259) as uint16_t, 0],
            [(32768 - 9334) as uint16_t, 0],
            [(32768 - 4644) as uint16_t, 0],
        ].into(),
        mask_comp: [
            [(32768 - 26607) as uint16_t, 0],
            [(32768 - 22891) as uint16_t, 0],
            [(32768 - 18840) as uint16_t, 0],
            [(32768 - 24594) as uint16_t, 0],
            [(32768 - 19934) as uint16_t, 0],
            [(32768 - 22674) as uint16_t, 0],
        ].into(),
        wedge_comp: [
            [(32768 - 23431) as uint16_t, 0],
            [(32768 - 13171) as uint16_t, 0],
            [(32768 - 11470) as uint16_t, 0],
            [(32768 - 9770) as uint16_t, 0],
            [(32768 - 9100) as uint16_t, 0],
            [(32768 - 8233) as uint16_t, 0],
            [(32768 - 6172) as uint16_t, 0],
            [(32768 - 11820) as uint16_t, 0],
            [(32768 - 7701) as uint16_t, 0],
        ].into(),
        r#ref: [
            [
                [(32768 - 4897) as uint16_t, 0],
                [(32768 - 16973) as uint16_t, 0],
                [(32768 - 29744) as uint16_t, 0],
            ],
            [
                [(32768 - 1555) as uint16_t, 0],
                [(32768 - 16751) as uint16_t, 0],
                [(32768 - 30279) as uint16_t, 0],
            ],
            [
                [(32768 - 4236) as uint16_t, 0],
                [(32768 - 19647) as uint16_t, 0],
                [(32768 - 31194) as uint16_t, 0],
            ],
            [
                [(32768 - 8650) as uint16_t, 0],
                [(32768 - 24773) as uint16_t, 0],
                [(32768 - 31895) as uint16_t, 0],
            ],
            [
                [(32768 - 904) as uint16_t, 0],
                [(32768 - 11014) as uint16_t, 0],
                [(32768 - 26875) as uint16_t, 0],
            ],
            [
                [(32768 - 1444) as uint16_t, 0],
                [(32768 - 15087) as uint16_t, 0],
                [(32768 - 30304) as uint16_t, 0],
            ],
        ].into(),
        comp_fwd_ref: [
            [
                [(32768 - 4946) as uint16_t, 0],
                [(32768 - 19891) as uint16_t, 0],
                [(32768 - 30731) as uint16_t, 0],
            ],
            [
                [(32768 - 9468) as uint16_t, 0],
                [(32768 - 22441) as uint16_t, 0],
                [(32768 - 31059) as uint16_t, 0],
            ],
            [
                [(32768 - 1503) as uint16_t, 0],
                [(32768 - 15160) as uint16_t, 0],
                [(32768 - 27544) as uint16_t, 0],
            ],
        ].into(),
        comp_bwd_ref: [
            [
                [(32768 - 2235) as uint16_t, 0],
                [(32768 - 17182) as uint16_t, 0],
                [(32768 - 30606) as uint16_t, 0],
            ],
            [
                [(32768 - 1423) as uint16_t, 0],
                [(32768 - 15175) as uint16_t, 0],
                [(32768 - 30489) as uint16_t, 0],
            ],
        ].into(),
        comp_uni_ref: [
            [
                [(32768 - 5284) as uint16_t, 0],
                [(32768 - 23152) as uint16_t, 0],
                [(32768 - 31774) as uint16_t, 0],
            ],
            [
                [(32768 - 3865) as uint16_t, 0],
                [(32768 - 14173) as uint16_t, 0],
                [(32768 - 25120) as uint16_t, 0],
            ],
            [
                [(32768 - 3128) as uint16_t, 0],
                [(32768 - 15270) as uint16_t, 0],
                [(32768 - 26710) as uint16_t, 0],
            ],
        ].into(),
        txpart: [
            [
                [(32768 - 28581) as uint16_t, 0],
                [(32768 - 23846) as uint16_t, 0],
                [(32768 - 20847) as uint16_t, 0],
            ],
            [
                [(32768 - 24315) as uint16_t, 0],
                [(32768 - 18196) as uint16_t, 0],
                [(32768 - 12133) as uint16_t, 0],
            ],
            [
                [(32768 - 18791) as uint16_t, 0],
                [(32768 - 10887) as uint16_t, 0],
                [(32768 - 11005) as uint16_t, 0],
            ],
            [
                [(32768 - 27179) as uint16_t, 0],
                [(32768 - 20004) as uint16_t, 0],
                [(32768 - 11281) as uint16_t, 0],
            ],
            [
                [(32768 - 26549) as uint16_t, 0],
                [(32768 - 19308) as uint16_t, 0],
                [(32768 - 14224) as uint16_t, 0],
            ],
            [
                [(32768 - 28015) as uint16_t, 0],
                [(32768 - 21546) as uint16_t, 0],
                [(32768 - 14400) as uint16_t, 0],
            ],
            [
                [(32768 - 28165) as uint16_t, 0],
                [(32768 - 22401) as uint16_t, 0],
                [(32768 - 16088) as uint16_t, 0],
            ],
        ].into(),
        skip: [
            [(32768 - 31671) as uint16_t, 0],
            [(32768 - 16515) as uint16_t, 0],
            [(32768 - 4576) as uint16_t, 0],
        ].into(),
        skip_mode: [
            [(32768 - 32621) as uint16_t, 0],
            [(32768 - 20708) as uint16_t, 0],
            [(32768 - 8127) as uint16_t, 0],
        ].into(),
        seg_pred: [
            [(32768 - 16384) as uint16_t, 0],
            [(32768 - 16384) as uint16_t, 0],
            [(32768 - 16384) as uint16_t, 0],
        ].into(),
        obmc: [
            [(32768 - 32638) as uint16_t, 0],
            [(32768 - 31560) as uint16_t, 0],
            [(32768 - 31014) as uint16_t, 0],
            [(32768 - 30128) as uint16_t, 0],
            [(32768 - 22083) as uint16_t, 0],
            [(32768 - 26879) as uint16_t, 0],
            [(32768 - 22823) as uint16_t, 0],
            [(32768 - 25817) as uint16_t, 0],
            [(32768 - 15142) as uint16_t, 0],
            [(32768 - 20901) as uint16_t, 0],
            [(32768 - 24008) as uint16_t, 0],
            [(32768 - 14423) as uint16_t, 0],
            [(32768 - 17432) as uint16_t, 0],
            [(32768 - 9301) as uint16_t, 0],
            [0; 2],
            [(32768 - 23664) as uint16_t, 0],
            [(32768 - 9371) as uint16_t, 0],
            [(32768 - 10437) as uint16_t, 0],
            [0; 2],
            [0; 2],
            [0; 2],
            [0; 2],
        ].into(),
        pal_y: [
            [
                [(32768 - 31676) as uint16_t, 0],
                [(32768 - 3419) as uint16_t, 0],
                [(32768 - 1261) as uint16_t, 0],
            ],
            [
                [(32768 - 31912) as uint16_t, 0],
                [(32768 - 2859) as uint16_t, 0],
                [(32768 - 980) as uint16_t, 0],
            ],
            [
                [(32768 - 31823) as uint16_t, 0],
                [(32768 - 3400) as uint16_t, 0],
                [(32768 - 781) as uint16_t, 0],
            ],
            [
                [(32768 - 32030) as uint16_t, 0],
                [(32768 - 3561) as uint16_t, 0],
                [(32768 - 904) as uint16_t, 0],
            ],
            [
                [(32768 - 32309) as uint16_t, 0],
                [(32768 - 7337) as uint16_t, 0],
                [(32768 - 1462) as uint16_t, 0],
            ],
            [
                [(32768 - 32265) as uint16_t, 0],
                [(32768 - 4015) as uint16_t, 0],
                [(32768 - 1521) as uint16_t, 0],
            ],
            [
                [(32768 - 32450) as uint16_t, 0],
                [(32768 - 7946) as uint16_t, 0],
                [(32768 - 129) as uint16_t, 0],
            ],
        ].into(),
        pal_uv: [
            [(32768 - 32461) as uint16_t, 0],
            [(32768 - 21488) as uint16_t, 0],
        ].into(),
        intrabc: [(32768 - 30531) as uint16_t, 0].into(),
    };
    init
}
pub fn default_mv_component_cdf() -> CdfMvComponent {
    let mut init = CdfMvComponent {
        classes: [
            (32768 - 28672) as uint16_t,
            (32768 - 30976) as uint16_t,
            (32768 - 31858) as uint16_t,
            (32768 - 32320) as uint16_t,
            (32768 - 32551) as uint16_t,
            (32768 - 32656) as uint16_t,
            (32768 - 32740) as uint16_t,
            (32768 - 32757) as uint16_t,
            (32768 - 32762) as uint16_t,
            (32768 - 32767) as uint16_t,
            0,
            0,
            0,
            0,
            0,
            0,
        ].into(),
        class0_fp: [
            [
                (32768 - 16384) as uint16_t,
                (32768 - 24576) as uint16_t,
                (32768 - 26624) as uint16_t,
                0,
            ],
            [
                (32768 - 12288) as uint16_t,
                (32768 - 21248) as uint16_t,
                (32768 - 24128) as uint16_t,
                0,
            ],
        ].into(),
        classN_fp: [
            (32768 - 8192) as uint16_t,
            (32768 - 17408) as uint16_t,
            (32768 - 21248) as uint16_t,
            0,
        ].into(),
        class0_hp: [(32768 - 20480) as uint16_t, 0].into(),
        classN_hp: [(32768 - 16384) as uint16_t, 0].into(),
        class0: [(32768 - 27648) as uint16_t, 0].into(),
        classN: [
            [(32768 - 17408) as uint16_t, 0],
            [(32768 - 17920) as uint16_t, 0],
            [(32768 - 18944) as uint16_t, 0],
            [(32768 - 20480) as uint16_t, 0],
            [(32768 - 22528) as uint16_t, 0],
            [(32768 - 24576) as uint16_t, 0],
            [(32768 - 28672) as uint16_t, 0],
            [(32768 - 29952) as uint16_t, 0],
            [(32768 - 29952) as uint16_t, 0],
            [(32768 - 30720) as uint16_t, 0],
        ].into(),
        sign: [(32768 - 16384) as uint16_t, 0].into(),
    };
    init
}
static mut default_mv_joint_cdf: [uint16_t; 4] = [
    (32768 - 4096) as uint16_t,
    (32768 - 11264) as uint16_t,
    (32768 - 19328) as uint16_t,
    0,
];
static mut default_kf_y_mode_cdf: [[[uint16_t; 16]; 5]; 5] = [
    [
        [
            (32768 - 15588) as uint16_t,
            (32768 - 17027) as uint16_t,
            (32768 - 19338) as uint16_t,
            (32768 - 20218) as uint16_t,
            (32768 - 20682) as uint16_t,
            (32768 - 21110) as uint16_t,
            (32768 - 21825) as uint16_t,
            (32768 - 23244) as uint16_t,
            (32768 - 24189) as uint16_t,
            (32768 - 28165) as uint16_t,
            (32768 - 29093) as uint16_t,
            (32768 - 30466) as uint16_t,
            0,
            0,
            0,
            0,
        ],
        [
            (32768 - 12016) as uint16_t,
            (32768 - 18066) as uint16_t,
            (32768 - 19516) as uint16_t,
            (32768 - 20303) as uint16_t,
            (32768 - 20719) as uint16_t,
            (32768 - 21444) as uint16_t,
            (32768 - 21888) as uint16_t,
            (32768 - 23032) as uint16_t,
            (32768 - 24434) as uint16_t,
            (32768 - 28658) as uint16_t,
            (32768 - 30172) as uint16_t,
            (32768 - 31409) as uint16_t,
            0,
            0,
            0,
            0,
        ],
        [
            (32768 - 10052) as uint16_t,
            (32768 - 10771) as uint16_t,
            (32768 - 22296) as uint16_t,
            (32768 - 22788) as uint16_t,
            (32768 - 23055) as uint16_t,
            (32768 - 23239) as uint16_t,
            (32768 - 24133) as uint16_t,
            (32768 - 25620) as uint16_t,
            (32768 - 26160) as uint16_t,
            (32768 - 29336) as uint16_t,
            (32768 - 29929) as uint16_t,
            (32768 - 31567) as uint16_t,
            0,
            0,
            0,
            0,
        ],
        [
            (32768 - 14091) as uint16_t,
            (32768 - 15406) as uint16_t,
            (32768 - 16442) as uint16_t,
            (32768 - 18808) as uint16_t,
            (32768 - 19136) as uint16_t,
            (32768 - 19546) as uint16_t,
            (32768 - 19998) as uint16_t,
            (32768 - 22096) as uint16_t,
            (32768 - 24746) as uint16_t,
            (32768 - 29585) as uint16_t,
            (32768 - 30958) as uint16_t,
            (32768 - 32462) as uint16_t,
            0,
            0,
            0,
            0,
        ],
        [
            (32768 - 12122) as uint16_t,
            (32768 - 13265) as uint16_t,
            (32768 - 15603) as uint16_t,
            (32768 - 16501) as uint16_t,
            (32768 - 18609) as uint16_t,
            (32768 - 20033) as uint16_t,
            (32768 - 22391) as uint16_t,
            (32768 - 25583) as uint16_t,
            (32768 - 26437) as uint16_t,
            (32768 - 30261) as uint16_t,
            (32768 - 31073) as uint16_t,
            (32768 - 32475) as uint16_t,
            0,
            0,
            0,
            0,
        ],
    ],
    [
        [
            (32768 - 10023) as uint16_t,
            (32768 - 19585) as uint16_t,
            (32768 - 20848) as uint16_t,
            (32768 - 21440) as uint16_t,
            (32768 - 21832) as uint16_t,
            (32768 - 22760) as uint16_t,
            (32768 - 23089) as uint16_t,
            (32768 - 24023) as uint16_t,
            (32768 - 25381) as uint16_t,
            (32768 - 29014) as uint16_t,
            (32768 - 30482) as uint16_t,
            (32768 - 31436) as uint16_t,
            0,
            0,
            0,
            0,
        ],
        [
            (32768 - 5983) as uint16_t,
            (32768 - 24099) as uint16_t,
            (32768 - 24560) as uint16_t,
            (32768 - 24886) as uint16_t,
            (32768 - 25066) as uint16_t,
            (32768 - 25795) as uint16_t,
            (32768 - 25913) as uint16_t,
            (32768 - 26423) as uint16_t,
            (32768 - 27610) as uint16_t,
            (32768 - 29905) as uint16_t,
            (32768 - 31276) as uint16_t,
            (32768 - 31794) as uint16_t,
            0,
            0,
            0,
            0,
        ],
        [
            (32768 - 7444) as uint16_t,
            (32768 - 12781) as uint16_t,
            (32768 - 20177) as uint16_t,
            (32768 - 20728) as uint16_t,
            (32768 - 21077) as uint16_t,
            (32768 - 21607) as uint16_t,
            (32768 - 22170) as uint16_t,
            (32768 - 23405) as uint16_t,
            (32768 - 24469) as uint16_t,
            (32768 - 27915) as uint16_t,
            (32768 - 29090) as uint16_t,
            (32768 - 30492) as uint16_t,
            0,
            0,
            0,
            0,
        ],
        [
            (32768 - 8537) as uint16_t,
            (32768 - 14689) as uint16_t,
            (32768 - 15432) as uint16_t,
            (32768 - 17087) as uint16_t,
            (32768 - 17408) as uint16_t,
            (32768 - 18172) as uint16_t,
            (32768 - 18408) as uint16_t,
            (32768 - 19825) as uint16_t,
            (32768 - 24649) as uint16_t,
            (32768 - 29153) as uint16_t,
            (32768 - 31096) as uint16_t,
            (32768 - 32210) as uint16_t,
            0,
            0,
            0,
            0,
        ],
        [
            (32768 - 7543) as uint16_t,
            (32768 - 14231) as uint16_t,
            (32768 - 15496) as uint16_t,
            (32768 - 16195) as uint16_t,
            (32768 - 17905) as uint16_t,
            (32768 - 20717) as uint16_t,
            (32768 - 21984) as uint16_t,
            (32768 - 24516) as uint16_t,
            (32768 - 26001) as uint16_t,
            (32768 - 29675) as uint16_t,
            (32768 - 30981) as uint16_t,
            (32768 - 31994) as uint16_t,
            0,
            0,
            0,
            0,
        ],
    ],
    [
        [
            (32768 - 12613) as uint16_t,
            (32768 - 13591) as uint16_t,
            (32768 - 21383) as uint16_t,
            (32768 - 22004) as uint16_t,
            (32768 - 22312) as uint16_t,
            (32768 - 22577) as uint16_t,
            (32768 - 23401) as uint16_t,
            (32768 - 25055) as uint16_t,
            (32768 - 25729) as uint16_t,
            (32768 - 29538) as uint16_t,
            (32768 - 30305) as uint16_t,
            (32768 - 32077) as uint16_t,
            0,
            0,
            0,
            0,
        ],
        [
            (32768 - 9687) as uint16_t,
            (32768 - 13470) as uint16_t,
            (32768 - 18506) as uint16_t,
            (32768 - 19230) as uint16_t,
            (32768 - 19604) as uint16_t,
            (32768 - 20147) as uint16_t,
            (32768 - 20695) as uint16_t,
            (32768 - 22062) as uint16_t,
            (32768 - 23219) as uint16_t,
            (32768 - 27743) as uint16_t,
            (32768 - 29211) as uint16_t,
            (32768 - 30907) as uint16_t,
            0,
            0,
            0,
            0,
        ],
        [
            (32768 - 6183) as uint16_t,
            (32768 - 6505) as uint16_t,
            (32768 - 26024) as uint16_t,
            (32768 - 26252) as uint16_t,
            (32768 - 26366) as uint16_t,
            (32768 - 26434) as uint16_t,
            (32768 - 27082) as uint16_t,
            (32768 - 28354) as uint16_t,
            (32768 - 28555) as uint16_t,
            (32768 - 30467) as uint16_t,
            (32768 - 30794) as uint16_t,
            (32768 - 32086) as uint16_t,
            0,
            0,
            0,
            0,
        ],
        [
            (32768 - 10718) as uint16_t,
            (32768 - 11734) as uint16_t,
            (32768 - 14954) as uint16_t,
            (32768 - 17224) as uint16_t,
            (32768 - 17565) as uint16_t,
            (32768 - 17924) as uint16_t,
            (32768 - 18561) as uint16_t,
            (32768 - 21523) as uint16_t,
            (32768 - 23878) as uint16_t,
            (32768 - 28975) as uint16_t,
            (32768 - 30287) as uint16_t,
            (32768 - 32252) as uint16_t,
            0,
            0,
            0,
            0,
        ],
        [
            (32768 - 9194) as uint16_t,
            (32768 - 9858) as uint16_t,
            (32768 - 16501) as uint16_t,
            (32768 - 17263) as uint16_t,
            (32768 - 18424) as uint16_t,
            (32768 - 19171) as uint16_t,
            (32768 - 21563) as uint16_t,
            (32768 - 25961) as uint16_t,
            (32768 - 26561) as uint16_t,
            (32768 - 30072) as uint16_t,
            (32768 - 30737) as uint16_t,
            (32768 - 32463) as uint16_t,
            0,
            0,
            0,
            0,
        ],
    ],
    [
        [
            (32768 - 12602) as uint16_t,
            (32768 - 14399) as uint16_t,
            (32768 - 15488) as uint16_t,
            (32768 - 18381) as uint16_t,
            (32768 - 18778) as uint16_t,
            (32768 - 19315) as uint16_t,
            (32768 - 19724) as uint16_t,
            (32768 - 21419) as uint16_t,
            (32768 - 25060) as uint16_t,
            (32768 - 29696) as uint16_t,
            (32768 - 30917) as uint16_t,
            (32768 - 32409) as uint16_t,
            0,
            0,
            0,
            0,
        ],
        [
            (32768 - 8203) as uint16_t,
            (32768 - 13821) as uint16_t,
            (32768 - 14524) as uint16_t,
            (32768 - 17105) as uint16_t,
            (32768 - 17439) as uint16_t,
            (32768 - 18131) as uint16_t,
            (32768 - 18404) as uint16_t,
            (32768 - 19468) as uint16_t,
            (32768 - 25225) as uint16_t,
            (32768 - 29485) as uint16_t,
            (32768 - 31158) as uint16_t,
            (32768 - 32342) as uint16_t,
            0,
            0,
            0,
            0,
        ],
        [
            (32768 - 8451) as uint16_t,
            (32768 - 9731) as uint16_t,
            (32768 - 15004) as uint16_t,
            (32768 - 17643) as uint16_t,
            (32768 - 18012) as uint16_t,
            (32768 - 18425) as uint16_t,
            (32768 - 19070) as uint16_t,
            (32768 - 21538) as uint16_t,
            (32768 - 24605) as uint16_t,
            (32768 - 29118) as uint16_t,
            (32768 - 30078) as uint16_t,
            (32768 - 32018) as uint16_t,
            0,
            0,
            0,
            0,
        ],
        [
            (32768 - 7714) as uint16_t,
            (32768 - 9048) as uint16_t,
            (32768 - 9516) as uint16_t,
            (32768 - 16667) as uint16_t,
            (32768 - 16817) as uint16_t,
            (32768 - 16994) as uint16_t,
            (32768 - 17153) as uint16_t,
            (32768 - 18767) as uint16_t,
            (32768 - 26743) as uint16_t,
            (32768 - 30389) as uint16_t,
            (32768 - 31536) as uint16_t,
            (32768 - 32528) as uint16_t,
            0,
            0,
            0,
            0,
        ],
        [
            (32768 - 8843) as uint16_t,
            (32768 - 10280) as uint16_t,
            (32768 - 11496) as uint16_t,
            (32768 - 15317) as uint16_t,
            (32768 - 16652) as uint16_t,
            (32768 - 17943) as uint16_t,
            (32768 - 19108) as uint16_t,
            (32768 - 22718) as uint16_t,
            (32768 - 25769) as uint16_t,
            (32768 - 29953) as uint16_t,
            (32768 - 30983) as uint16_t,
            (32768 - 32485) as uint16_t,
            0,
            0,
            0,
            0,
        ],
    ],
    [
        [
            (32768 - 12578) as uint16_t,
            (32768 - 13671) as uint16_t,
            (32768 - 15979) as uint16_t,
            (32768 - 16834) as uint16_t,
            (32768 - 19075) as uint16_t,
            (32768 - 20913) as uint16_t,
            (32768 - 22989) as uint16_t,
            (32768 - 25449) as uint16_t,
            (32768 - 26219) as uint16_t,
            (32768 - 30214) as uint16_t,
            (32768 - 31150) as uint16_t,
            (32768 - 32477) as uint16_t,
            0,
            0,
            0,
            0,
        ],
        [
            (32768 - 9563) as uint16_t,
            (32768 - 13626) as uint16_t,
            (32768 - 15080) as uint16_t,
            (32768 - 15892) as uint16_t,
            (32768 - 17756) as uint16_t,
            (32768 - 20863) as uint16_t,
            (32768 - 22207) as uint16_t,
            (32768 - 24236) as uint16_t,
            (32768 - 25380) as uint16_t,
            (32768 - 29653) as uint16_t,
            (32768 - 31143) as uint16_t,
            (32768 - 32277) as uint16_t,
            0,
            0,
            0,
            0,
        ],
        [
            (32768 - 8356) as uint16_t,
            (32768 - 8901) as uint16_t,
            (32768 - 17616) as uint16_t,
            (32768 - 18256) as uint16_t,
            (32768 - 19350) as uint16_t,
            (32768 - 20106) as uint16_t,
            (32768 - 22598) as uint16_t,
            (32768 - 25947) as uint16_t,
            (32768 - 26466) as uint16_t,
            (32768 - 29900) as uint16_t,
            (32768 - 30523) as uint16_t,
            (32768 - 32261) as uint16_t,
            0,
            0,
            0,
            0,
        ],
        [
            (32768 - 10835) as uint16_t,
            (32768 - 11815) as uint16_t,
            (32768 - 13124) as uint16_t,
            (32768 - 16042) as uint16_t,
            (32768 - 17018) as uint16_t,
            (32768 - 18039) as uint16_t,
            (32768 - 18947) as uint16_t,
            (32768 - 22753) as uint16_t,
            (32768 - 24615) as uint16_t,
            (32768 - 29489) as uint16_t,
            (32768 - 30883) as uint16_t,
            (32768 - 32482) as uint16_t,
            0,
            0,
            0,
            0,
        ],
        [
            (32768 - 7618) as uint16_t,
            (32768 - 8288) as uint16_t,
            (32768 - 9859) as uint16_t,
            (32768 - 10509) as uint16_t,
            (32768 - 15386) as uint16_t,
            (32768 - 18657) as uint16_t,
            (32768 - 22903) as uint16_t,
            (32768 - 28776) as uint16_t,
            (32768 - 29180) as uint16_t,
            (32768 - 31355) as uint16_t,
            (32768 - 31802) as uint16_t,
            (32768 - 32593) as uint16_t,
            0,
            0,
            0,
            0,
        ],
    ],
];
pub fn av1_default_coef_cdf() -> [CdfCoefContext; 4] {
[
    {
        let mut init = CdfCoefContext {
            eob_bin_16: [
                [
                    [
                        (32768 - 840) as uint16_t,
                        (32768 - 1039) as uint16_t,
                        (32768 - 1980) as uint16_t,
                        (32768 - 4895) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 370) as uint16_t,
                        (32768 - 671) as uint16_t,
                        (32768 - 1883) as uint16_t,
                        (32768 - 4471) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 3247) as uint16_t,
                        (32768 - 4950) as uint16_t,
                        (32768 - 9688) as uint16_t,
                        (32768 - 14563) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 1904) as uint16_t,
                        (32768 - 3354) as uint16_t,
                        (32768 - 7763) as uint16_t,
                        (32768 - 14647) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                ],
            ].into(),
            eob_bin_32: [
                [
                    [
                        (32768 - 400) as uint16_t,
                        (32768 - 520) as uint16_t,
                        (32768 - 977) as uint16_t,
                        (32768 - 2102) as uint16_t,
                        (32768 - 6542) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 210) as uint16_t,
                        (32768 - 405) as uint16_t,
                        (32768 - 1315) as uint16_t,
                        (32768 - 3326) as uint16_t,
                        (32768 - 7537) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 2636) as uint16_t,
                        (32768 - 4273) as uint16_t,
                        (32768 - 7588) as uint16_t,
                        (32768 - 11794) as uint16_t,
                        (32768 - 20401) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 1786) as uint16_t,
                        (32768 - 3179) as uint16_t,
                        (32768 - 6902) as uint16_t,
                        (32768 - 11357) as uint16_t,
                        (32768 - 19054) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                ],
            ].into(),
            eob_bin_64: [
                [
                    [
                        (32768 - 329) as uint16_t,
                        (32768 - 498) as uint16_t,
                        (32768 - 1101) as uint16_t,
                        (32768 - 1784) as uint16_t,
                        (32768 - 3265) as uint16_t,
                        (32768 - 7758) as uint16_t,
                        0,
                        0,
                    ],
                    [
                        (32768 - 335) as uint16_t,
                        (32768 - 730) as uint16_t,
                        (32768 - 1459) as uint16_t,
                        (32768 - 5494) as uint16_t,
                        (32768 - 8755) as uint16_t,
                        (32768 - 12997) as uint16_t,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 3505) as uint16_t,
                        (32768 - 5304) as uint16_t,
                        (32768 - 10086) as uint16_t,
                        (32768 - 13814) as uint16_t,
                        (32768 - 17684) as uint16_t,
                        (32768 - 23370) as uint16_t,
                        0,
                        0,
                    ],
                    [
                        (32768 - 1563) as uint16_t,
                        (32768 - 2700) as uint16_t,
                        (32768 - 4876) as uint16_t,
                        (32768 - 10911) as uint16_t,
                        (32768 - 14706) as uint16_t,
                        (32768 - 22480) as uint16_t,
                        0,
                        0,
                    ],
                ],
            ].into(),
            eob_bin_128: [
                [
                    [
                        (32768 - 219) as uint16_t,
                        (32768 - 482) as uint16_t,
                        (32768 - 1140) as uint16_t,
                        (32768 - 2091) as uint16_t,
                        (32768 - 3680) as uint16_t,
                        (32768 - 6028) as uint16_t,
                        (32768 - 12586) as uint16_t,
                        0,
                    ],
                    [
                        (32768 - 371) as uint16_t,
                        (32768 - 699) as uint16_t,
                        (32768 - 1254) as uint16_t,
                        (32768 - 4830) as uint16_t,
                        (32768 - 9479) as uint16_t,
                        (32768 - 12562) as uint16_t,
                        (32768 - 17497) as uint16_t,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 5245) as uint16_t,
                        (32768 - 7456) as uint16_t,
                        (32768 - 12880) as uint16_t,
                        (32768 - 15852) as uint16_t,
                        (32768 - 20033) as uint16_t,
                        (32768 - 23932) as uint16_t,
                        (32768 - 27608) as uint16_t,
                        0,
                    ],
                    [
                        (32768 - 2054) as uint16_t,
                        (32768 - 3472) as uint16_t,
                        (32768 - 5869) as uint16_t,
                        (32768 - 14232) as uint16_t,
                        (32768 - 18242) as uint16_t,
                        (32768 - 20590) as uint16_t,
                        (32768 - 26752) as uint16_t,
                        0,
                    ],
                ],
            ].into(),
            eob_bin_256: [
                [
                    [
                        (32768 - 310) as uint16_t,
                        (32768 - 584) as uint16_t,
                        (32768 - 1887) as uint16_t,
                        (32768 - 3589) as uint16_t,
                        (32768 - 6168) as uint16_t,
                        (32768 - 8611) as uint16_t,
                        (32768 - 11352) as uint16_t,
                        (32768 - 15652) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 998) as uint16_t,
                        (32768 - 1850) as uint16_t,
                        (32768 - 2998) as uint16_t,
                        (32768 - 5604) as uint16_t,
                        (32768 - 17341) as uint16_t,
                        (32768 - 19888) as uint16_t,
                        (32768 - 22899) as uint16_t,
                        (32768 - 25583) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 2520) as uint16_t,
                        (32768 - 3240) as uint16_t,
                        (32768 - 5952) as uint16_t,
                        (32768 - 8870) as uint16_t,
                        (32768 - 12577) as uint16_t,
                        (32768 - 17558) as uint16_t,
                        (32768 - 19954) as uint16_t,
                        (32768 - 24168) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 2203) as uint16_t,
                        (32768 - 4130) as uint16_t,
                        (32768 - 7435) as uint16_t,
                        (32768 - 10739) as uint16_t,
                        (32768 - 20652) as uint16_t,
                        (32768 - 23681) as uint16_t,
                        (32768 - 25609) as uint16_t,
                        (32768 - 27261) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                ],
            ].into(),
            eob_bin_512: [
                [
                    (32768 - 641) as uint16_t,
                    (32768 - 983) as uint16_t,
                    (32768 - 3707) as uint16_t,
                    (32768 - 5430) as uint16_t,
                    (32768 - 10234) as uint16_t,
                    (32768 - 14958) as uint16_t,
                    (32768 - 18788) as uint16_t,
                    (32768 - 23412) as uint16_t,
                    (32768 - 26061) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 5095) as uint16_t,
                    (32768 - 6446) as uint16_t,
                    (32768 - 9996) as uint16_t,
                    (32768 - 13354) as uint16_t,
                    (32768 - 16017) as uint16_t,
                    (32768 - 17986) as uint16_t,
                    (32768 - 20919) as uint16_t,
                    (32768 - 26129) as uint16_t,
                    (32768 - 29140) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            ].into(),
            eob_bin_1024: [
                [
                    (32768 - 393) as uint16_t,
                    (32768 - 421) as uint16_t,
                    (32768 - 751) as uint16_t,
                    (32768 - 1623) as uint16_t,
                    (32768 - 3160) as uint16_t,
                    (32768 - 6352) as uint16_t,
                    (32768 - 13345) as uint16_t,
                    (32768 - 18047) as uint16_t,
                    (32768 - 22571) as uint16_t,
                    (32768 - 25830) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 1865) as uint16_t,
                    (32768 - 1988) as uint16_t,
                    (32768 - 2930) as uint16_t,
                    (32768 - 4242) as uint16_t,
                    (32768 - 10533) as uint16_t,
                    (32768 - 16538) as uint16_t,
                    (32768 - 21354) as uint16_t,
                    (32768 - 27255) as uint16_t,
                    (32768 - 28546) as uint16_t,
                    (32768 - 31784) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            ].into(),
            eob_base_tok: [
                [
                    [
                        [
                            (32768 - 17837) as uint16_t,
                            (32768 - 29055) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 29600) as uint16_t,
                            (32768 - 31446) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 30844) as uint16_t,
                            (32768 - 31878) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 24926) as uint16_t,
                            (32768 - 28948) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 21365) as uint16_t,
                            (32768 - 30026) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 30512) as uint16_t,
                            (32768 - 32423) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31658) as uint16_t,
                            (32768 - 32621) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 29630) as uint16_t,
                            (32768 - 31881) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 5717) as uint16_t,
                            (32768 - 26477) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 30491) as uint16_t,
                            (32768 - 31703) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31550) as uint16_t,
                            (32768 - 32158) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 29648) as uint16_t,
                            (32768 - 31491) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 12608) as uint16_t,
                            (32768 - 27820) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 30680) as uint16_t,
                            (32768 - 32225) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 30809) as uint16_t,
                            (32768 - 32335) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31299) as uint16_t,
                            (32768 - 32423) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 1786) as uint16_t,
                            (32768 - 12612) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 30663) as uint16_t,
                            (32768 - 31625) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32339) as uint16_t,
                            (32768 - 32468) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31148) as uint16_t,
                            (32768 - 31833) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 18857) as uint16_t,
                            (32768 - 23865) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31428) as uint16_t,
                            (32768 - 32428) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31744) as uint16_t,
                            (32768 - 32373) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31775) as uint16_t,
                            (32768 - 32526) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 1787) as uint16_t,
                            (32768 - 2532) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 30832) as uint16_t,
                            (32768 - 31662) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31824) as uint16_t,
                            (32768 - 32682) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32133) as uint16_t,
                            (32768 - 32569) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 13751) as uint16_t,
                            (32768 - 22235) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32089) as uint16_t,
                            (32768 - 32409) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 27084) as uint16_t,
                            (32768 - 27920) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 29291) as uint16_t,
                            (32768 - 32594) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 1725) as uint16_t,
                            (32768 - 3449) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31102) as uint16_t,
                            (32768 - 31935) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32457) as uint16_t,
                            (32768 - 32613) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32412) as uint16_t,
                            (32768 - 32649) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 10923) as uint16_t,
                            (32768 - 21845) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 10923) as uint16_t,
                            (32768 - 21845) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 10923) as uint16_t,
                            (32768 - 21845) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 10923) as uint16_t,
                            (32768 - 21845) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                ],
            ].into(),
            base_tok: [
                [
                    [
                        [
                            (32768 - 4034) as uint16_t,
                            (32768 - 8930) as uint16_t,
                            (32768 - 12727) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18082) as uint16_t,
                            (32768 - 29741) as uint16_t,
                            (32768 - 31877) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12596) as uint16_t,
                            (32768 - 26124) as uint16_t,
                            (32768 - 30493) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9446) as uint16_t,
                            (32768 - 21118) as uint16_t,
                            (32768 - 27005) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6308) as uint16_t,
                            (32768 - 15141) as uint16_t,
                            (32768 - 21279) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2463) as uint16_t,
                            (32768 - 6357) as uint16_t,
                            (32768 - 9783) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20667) as uint16_t,
                            (32768 - 30546) as uint16_t,
                            (32768 - 31929) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13043) as uint16_t,
                            (32768 - 26123) as uint16_t,
                            (32768 - 30134) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8151) as uint16_t,
                            (32768 - 18757) as uint16_t,
                            (32768 - 24778) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5255) as uint16_t,
                            (32768 - 12839) as uint16_t,
                            (32768 - 18632) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2820) as uint16_t,
                            (32768 - 7206) as uint16_t,
                            (32768 - 11161) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15736) as uint16_t,
                            (32768 - 27553) as uint16_t,
                            (32768 - 30604) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11210) as uint16_t,
                            (32768 - 23794) as uint16_t,
                            (32768 - 28787) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5947) as uint16_t,
                            (32768 - 13874) as uint16_t,
                            (32768 - 19701) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4215) as uint16_t,
                            (32768 - 9323) as uint16_t,
                            (32768 - 13891) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2833) as uint16_t,
                            (32768 - 6462) as uint16_t,
                            (32768 - 10059) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19605) as uint16_t,
                            (32768 - 30393) as uint16_t,
                            (32768 - 31582) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13523) as uint16_t,
                            (32768 - 26252) as uint16_t,
                            (32768 - 30248) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8446) as uint16_t,
                            (32768 - 18622) as uint16_t,
                            (32768 - 24512) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3818) as uint16_t,
                            (32768 - 10343) as uint16_t,
                            (32768 - 15974) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1481) as uint16_t,
                            (32768 - 4117) as uint16_t,
                            (32768 - 6796) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22649) as uint16_t,
                            (32768 - 31302) as uint16_t,
                            (32768 - 32190) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14829) as uint16_t,
                            (32768 - 27127) as uint16_t,
                            (32768 - 30449) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8313) as uint16_t,
                            (32768 - 17702) as uint16_t,
                            (32768 - 23304) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3022) as uint16_t,
                            (32768 - 8301) as uint16_t,
                            (32768 - 12786) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1536) as uint16_t,
                            (32768 - 4412) as uint16_t,
                            (32768 - 7184) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22354) as uint16_t,
                            (32768 - 29774) as uint16_t,
                            (32768 - 31372) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14723) as uint16_t,
                            (32768 - 25472) as uint16_t,
                            (32768 - 29214) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6673) as uint16_t,
                            (32768 - 13745) as uint16_t,
                            (32768 - 18662) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2068) as uint16_t,
                            (32768 - 5766) as uint16_t,
                            (32768 - 9322) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 6302) as uint16_t,
                            (32768 - 16444) as uint16_t,
                            (32768 - 21761) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23040) as uint16_t,
                            (32768 - 31538) as uint16_t,
                            (32768 - 32475) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15196) as uint16_t,
                            (32768 - 28452) as uint16_t,
                            (32768 - 31496) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10020) as uint16_t,
                            (32768 - 22946) as uint16_t,
                            (32768 - 28514) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6533) as uint16_t,
                            (32768 - 16862) as uint16_t,
                            (32768 - 23501) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3538) as uint16_t,
                            (32768 - 9816) as uint16_t,
                            (32768 - 15076) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24444) as uint16_t,
                            (32768 - 31875) as uint16_t,
                            (32768 - 32525) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15881) as uint16_t,
                            (32768 - 28924) as uint16_t,
                            (32768 - 31635) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9922) as uint16_t,
                            (32768 - 22873) as uint16_t,
                            (32768 - 28466) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6527) as uint16_t,
                            (32768 - 16966) as uint16_t,
                            (32768 - 23691) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4114) as uint16_t,
                            (32768 - 11303) as uint16_t,
                            (32768 - 17220) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20201) as uint16_t,
                            (32768 - 30770) as uint16_t,
                            (32768 - 32209) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14754) as uint16_t,
                            (32768 - 28071) as uint16_t,
                            (32768 - 31258) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8378) as uint16_t,
                            (32768 - 20186) as uint16_t,
                            (32768 - 26517) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5916) as uint16_t,
                            (32768 - 15299) as uint16_t,
                            (32768 - 21978) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4268) as uint16_t,
                            (32768 - 11583) as uint16_t,
                            (32768 - 17901) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24361) as uint16_t,
                            (32768 - 32025) as uint16_t,
                            (32768 - 32581) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18673) as uint16_t,
                            (32768 - 30105) as uint16_t,
                            (32768 - 31943) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10196) as uint16_t,
                            (32768 - 22244) as uint16_t,
                            (32768 - 27576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5495) as uint16_t,
                            (32768 - 14349) as uint16_t,
                            (32768 - 20417) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2676) as uint16_t,
                            (32768 - 7415) as uint16_t,
                            (32768 - 11498) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24678) as uint16_t,
                            (32768 - 31958) as uint16_t,
                            (32768 - 32585) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18629) as uint16_t,
                            (32768 - 29906) as uint16_t,
                            (32768 - 31831) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9364) as uint16_t,
                            (32768 - 20724) as uint16_t,
                            (32768 - 26315) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4641) as uint16_t,
                            (32768 - 12318) as uint16_t,
                            (32768 - 18094) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2758) as uint16_t,
                            (32768 - 7387) as uint16_t,
                            (32768 - 11579) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 25433) as uint16_t,
                            (32768 - 31842) as uint16_t,
                            (32768 - 32469) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18795) as uint16_t,
                            (32768 - 29289) as uint16_t,
                            (32768 - 31411) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7644) as uint16_t,
                            (32768 - 17584) as uint16_t,
                            (32768 - 23592) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3408) as uint16_t,
                            (32768 - 9014) as uint16_t,
                            (32768 - 15047) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 4536) as uint16_t,
                            (32768 - 10072) as uint16_t,
                            (32768 - 14001) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 25459) as uint16_t,
                            (32768 - 31416) as uint16_t,
                            (32768 - 32206) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16605) as uint16_t,
                            (32768 - 28048) as uint16_t,
                            (32768 - 30818) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11008) as uint16_t,
                            (32768 - 22857) as uint16_t,
                            (32768 - 27719) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6915) as uint16_t,
                            (32768 - 16268) as uint16_t,
                            (32768 - 22315) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2625) as uint16_t,
                            (32768 - 6812) as uint16_t,
                            (32768 - 10537) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24257) as uint16_t,
                            (32768 - 31788) as uint16_t,
                            (32768 - 32499) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16880) as uint16_t,
                            (32768 - 29454) as uint16_t,
                            (32768 - 31879) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11958) as uint16_t,
                            (32768 - 25054) as uint16_t,
                            (32768 - 29778) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7916) as uint16_t,
                            (32768 - 18718) as uint16_t,
                            (32768 - 25084) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3383) as uint16_t,
                            (32768 - 8777) as uint16_t,
                            (32768 - 13446) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22720) as uint16_t,
                            (32768 - 31603) as uint16_t,
                            (32768 - 32393) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14960) as uint16_t,
                            (32768 - 28125) as uint16_t,
                            (32768 - 31335) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9731) as uint16_t,
                            (32768 - 22210) as uint16_t,
                            (32768 - 27928) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6304) as uint16_t,
                            (32768 - 15832) as uint16_t,
                            (32768 - 22277) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2910) as uint16_t,
                            (32768 - 7818) as uint16_t,
                            (32768 - 12166) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20375) as uint16_t,
                            (32768 - 30627) as uint16_t,
                            (32768 - 32131) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13904) as uint16_t,
                            (32768 - 27284) as uint16_t,
                            (32768 - 30887) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9368) as uint16_t,
                            (32768 - 21558) as uint16_t,
                            (32768 - 27144) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5937) as uint16_t,
                            (32768 - 14966) as uint16_t,
                            (32768 - 21119) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2667) as uint16_t,
                            (32768 - 7225) as uint16_t,
                            (32768 - 11319) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23970) as uint16_t,
                            (32768 - 31470) as uint16_t,
                            (32768 - 32378) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17173) as uint16_t,
                            (32768 - 29734) as uint16_t,
                            (32768 - 32018) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12795) as uint16_t,
                            (32768 - 25441) as uint16_t,
                            (32768 - 29965) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8981) as uint16_t,
                            (32768 - 19680) as uint16_t,
                            (32768 - 25893) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4728) as uint16_t,
                            (32768 - 11372) as uint16_t,
                            (32768 - 16902) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24287) as uint16_t,
                            (32768 - 31797) as uint16_t,
                            (32768 - 32439) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16703) as uint16_t,
                            (32768 - 29145) as uint16_t,
                            (32768 - 31696) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10833) as uint16_t,
                            (32768 - 23554) as uint16_t,
                            (32768 - 28725) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6468) as uint16_t,
                            (32768 - 16566) as uint16_t,
                            (32768 - 23057) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2415) as uint16_t,
                            (32768 - 6562) as uint16_t,
                            (32768 - 10278) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 26610) as uint16_t,
                            (32768 - 32395) as uint16_t,
                            (32768 - 32659) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18590) as uint16_t,
                            (32768 - 30498) as uint16_t,
                            (32768 - 32117) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12420) as uint16_t,
                            (32768 - 25756) as uint16_t,
                            (32768 - 29950) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7639) as uint16_t,
                            (32768 - 18746) as uint16_t,
                            (32768 - 24710) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3001) as uint16_t,
                            (32768 - 8086) as uint16_t,
                            (32768 - 12347) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 25076) as uint16_t,
                            (32768 - 32064) as uint16_t,
                            (32768 - 32580) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17946) as uint16_t,
                            (32768 - 30128) as uint16_t,
                            (32768 - 32028) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12024) as uint16_t,
                            (32768 - 24985) as uint16_t,
                            (32768 - 29378) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7517) as uint16_t,
                            (32768 - 18390) as uint16_t,
                            (32768 - 24304) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3243) as uint16_t,
                            (32768 - 8781) as uint16_t,
                            (32768 - 13331) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 6037) as uint16_t,
                            (32768 - 16771) as uint16_t,
                            (32768 - 21957) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24774) as uint16_t,
                            (32768 - 31704) as uint16_t,
                            (32768 - 32426) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16830) as uint16_t,
                            (32768 - 28589) as uint16_t,
                            (32768 - 31056) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10602) as uint16_t,
                            (32768 - 22828) as uint16_t,
                            (32768 - 27760) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6733) as uint16_t,
                            (32768 - 16829) as uint16_t,
                            (32768 - 23071) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3250) as uint16_t,
                            (32768 - 8914) as uint16_t,
                            (32768 - 13556) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 25582) as uint16_t,
                            (32768 - 32220) as uint16_t,
                            (32768 - 32668) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18659) as uint16_t,
                            (32768 - 30342) as uint16_t,
                            (32768 - 32223) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12546) as uint16_t,
                            (32768 - 26149) as uint16_t,
                            (32768 - 30515) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8420) as uint16_t,
                            (32768 - 20451) as uint16_t,
                            (32768 - 26801) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4636) as uint16_t,
                            (32768 - 12420) as uint16_t,
                            (32768 - 18344) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27581) as uint16_t,
                            (32768 - 32362) as uint16_t,
                            (32768 - 32639) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18987) as uint16_t,
                            (32768 - 30083) as uint16_t,
                            (32768 - 31978) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11327) as uint16_t,
                            (32768 - 24248) as uint16_t,
                            (32768 - 29084) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7264) as uint16_t,
                            (32768 - 17719) as uint16_t,
                            (32768 - 24120) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3995) as uint16_t,
                            (32768 - 10768) as uint16_t,
                            (32768 - 16169) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 25893) as uint16_t,
                            (32768 - 31831) as uint16_t,
                            (32768 - 32487) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16577) as uint16_t,
                            (32768 - 28587) as uint16_t,
                            (32768 - 31379) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10189) as uint16_t,
                            (32768 - 22748) as uint16_t,
                            (32768 - 28182) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6832) as uint16_t,
                            (32768 - 17094) as uint16_t,
                            (32768 - 23556) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3708) as uint16_t,
                            (32768 - 10110) as uint16_t,
                            (32768 - 15334) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 25904) as uint16_t,
                            (32768 - 32282) as uint16_t,
                            (32768 - 32656) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19721) as uint16_t,
                            (32768 - 30792) as uint16_t,
                            (32768 - 32276) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12819) as uint16_t,
                            (32768 - 26243) as uint16_t,
                            (32768 - 30411) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8572) as uint16_t,
                            (32768 - 20614) as uint16_t,
                            (32768 - 26891) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5364) as uint16_t,
                            (32768 - 14059) as uint16_t,
                            (32768 - 20467) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 26580) as uint16_t,
                            (32768 - 32438) as uint16_t,
                            (32768 - 32677) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20852) as uint16_t,
                            (32768 - 31225) as uint16_t,
                            (32768 - 32340) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12435) as uint16_t,
                            (32768 - 25700) as uint16_t,
                            (32768 - 29967) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8691) as uint16_t,
                            (32768 - 20825) as uint16_t,
                            (32768 - 26976) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4446) as uint16_t,
                            (32768 - 12209) as uint16_t,
                            (32768 - 17269) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27350) as uint16_t,
                            (32768 - 32429) as uint16_t,
                            (32768 - 32696) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21372) as uint16_t,
                            (32768 - 30977) as uint16_t,
                            (32768 - 32272) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12673) as uint16_t,
                            (32768 - 25270) as uint16_t,
                            (32768 - 29853) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9208) as uint16_t,
                            (32768 - 20925) as uint16_t,
                            (32768 - 26640) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5018) as uint16_t,
                            (32768 - 13351) as uint16_t,
                            (32768 - 18732) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27351) as uint16_t,
                            (32768 - 32479) as uint16_t,
                            (32768 - 32713) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21398) as uint16_t,
                            (32768 - 31209) as uint16_t,
                            (32768 - 32387) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12162) as uint16_t,
                            (32768 - 25047) as uint16_t,
                            (32768 - 29842) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7896) as uint16_t,
                            (32768 - 18691) as uint16_t,
                            (32768 - 25319) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4670) as uint16_t,
                            (32768 - 12882) as uint16_t,
                            (32768 - 18881) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 5487) as uint16_t,
                            (32768 - 10460) as uint16_t,
                            (32768 - 13708) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21597) as uint16_t,
                            (32768 - 28303) as uint16_t,
                            (32768 - 30674) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11037) as uint16_t,
                            (32768 - 21953) as uint16_t,
                            (32768 - 26476) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8147) as uint16_t,
                            (32768 - 17962) as uint16_t,
                            (32768 - 22952) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5242) as uint16_t,
                            (32768 - 13061) as uint16_t,
                            (32768 - 18532) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1889) as uint16_t,
                            (32768 - 5208) as uint16_t,
                            (32768 - 8182) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 26774) as uint16_t,
                            (32768 - 32133) as uint16_t,
                            (32768 - 32590) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17844) as uint16_t,
                            (32768 - 29564) as uint16_t,
                            (32768 - 31767) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11690) as uint16_t,
                            (32768 - 24438) as uint16_t,
                            (32768 - 29171) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7542) as uint16_t,
                            (32768 - 18215) as uint16_t,
                            (32768 - 24459) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2993) as uint16_t,
                            (32768 - 8050) as uint16_t,
                            (32768 - 12319) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28023) as uint16_t,
                            (32768 - 32328) as uint16_t,
                            (32768 - 32591) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18651) as uint16_t,
                            (32768 - 30126) as uint16_t,
                            (32768 - 31954) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12164) as uint16_t,
                            (32768 - 25146) as uint16_t,
                            (32768 - 29589) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7762) as uint16_t,
                            (32768 - 18530) as uint16_t,
                            (32768 - 24771) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3492) as uint16_t,
                            (32768 - 9183) as uint16_t,
                            (32768 - 13920) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27591) as uint16_t,
                            (32768 - 32008) as uint16_t,
                            (32768 - 32491) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17149) as uint16_t,
                            (32768 - 28853) as uint16_t,
                            (32768 - 31510) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11485) as uint16_t,
                            (32768 - 24003) as uint16_t,
                            (32768 - 28860) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7697) as uint16_t,
                            (32768 - 18086) as uint16_t,
                            (32768 - 24210) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3075) as uint16_t,
                            (32768 - 7999) as uint16_t,
                            (32768 - 12218) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28268) as uint16_t,
                            (32768 - 32482) as uint16_t,
                            (32768 - 32654) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19631) as uint16_t,
                            (32768 - 31051) as uint16_t,
                            (32768 - 32404) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13860) as uint16_t,
                            (32768 - 27260) as uint16_t,
                            (32768 - 31020) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9605) as uint16_t,
                            (32768 - 21613) as uint16_t,
                            (32768 - 27594) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4876) as uint16_t,
                            (32768 - 12162) as uint16_t,
                            (32768 - 17908) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27248) as uint16_t,
                            (32768 - 32316) as uint16_t,
                            (32768 - 32576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18955) as uint16_t,
                            (32768 - 30457) as uint16_t,
                            (32768 - 32075) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11824) as uint16_t,
                            (32768 - 23997) as uint16_t,
                            (32768 - 28795) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7346) as uint16_t,
                            (32768 - 18196) as uint16_t,
                            (32768 - 24647) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3403) as uint16_t,
                            (32768 - 9247) as uint16_t,
                            (32768 - 14111) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29711) as uint16_t,
                            (32768 - 32655) as uint16_t,
                            (32768 - 32735) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21169) as uint16_t,
                            (32768 - 31394) as uint16_t,
                            (32768 - 32417) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13487) as uint16_t,
                            (32768 - 27198) as uint16_t,
                            (32768 - 30957) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8828) as uint16_t,
                            (32768 - 21683) as uint16_t,
                            (32768 - 27614) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4270) as uint16_t,
                            (32768 - 11451) as uint16_t,
                            (32768 - 17038) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28708) as uint16_t,
                            (32768 - 32578) as uint16_t,
                            (32768 - 32731) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20120) as uint16_t,
                            (32768 - 31241) as uint16_t,
                            (32768 - 32482) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13692) as uint16_t,
                            (32768 - 27550) as uint16_t,
                            (32768 - 31321) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9418) as uint16_t,
                            (32768 - 22514) as uint16_t,
                            (32768 - 28439) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4999) as uint16_t,
                            (32768 - 13283) as uint16_t,
                            (32768 - 19462) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 5673) as uint16_t,
                            (32768 - 14302) as uint16_t,
                            (32768 - 19711) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 26251) as uint16_t,
                            (32768 - 30701) as uint16_t,
                            (32768 - 31834) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12782) as uint16_t,
                            (32768 - 23783) as uint16_t,
                            (32768 - 27803) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9127) as uint16_t,
                            (32768 - 20657) as uint16_t,
                            (32768 - 25808) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6368) as uint16_t,
                            (32768 - 16208) as uint16_t,
                            (32768 - 21462) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2465) as uint16_t,
                            (32768 - 7177) as uint16_t,
                            (32768 - 10822) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29961) as uint16_t,
                            (32768 - 32563) as uint16_t,
                            (32768 - 32719) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18318) as uint16_t,
                            (32768 - 29891) as uint16_t,
                            (32768 - 31949) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11361) as uint16_t,
                            (32768 - 24514) as uint16_t,
                            (32768 - 29357) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7900) as uint16_t,
                            (32768 - 19603) as uint16_t,
                            (32768 - 25607) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4002) as uint16_t,
                            (32768 - 10590) as uint16_t,
                            (32768 - 15546) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29637) as uint16_t,
                            (32768 - 32310) as uint16_t,
                            (32768 - 32595) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18296) as uint16_t,
                            (32768 - 29913) as uint16_t,
                            (32768 - 31809) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10144) as uint16_t,
                            (32768 - 21515) as uint16_t,
                            (32768 - 26871) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5358) as uint16_t,
                            (32768 - 14322) as uint16_t,
                            (32768 - 20394) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3067) as uint16_t,
                            (32768 - 8362) as uint16_t,
                            (32768 - 13346) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28652) as uint16_t,
                            (32768 - 32470) as uint16_t,
                            (32768 - 32676) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17538) as uint16_t,
                            (32768 - 30771) as uint16_t,
                            (32768 - 32209) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13924) as uint16_t,
                            (32768 - 26882) as uint16_t,
                            (32768 - 30494) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10496) as uint16_t,
                            (32768 - 22837) as uint16_t,
                            (32768 - 27869) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7236) as uint16_t,
                            (32768 - 16396) as uint16_t,
                            (32768 - 21621) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 30743) as uint16_t,
                            (32768 - 32687) as uint16_t,
                            (32768 - 32746) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23006) as uint16_t,
                            (32768 - 31676) as uint16_t,
                            (32768 - 32489) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14494) as uint16_t,
                            (32768 - 27828) as uint16_t,
                            (32768 - 31120) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10174) as uint16_t,
                            (32768 - 22801) as uint16_t,
                            (32768 - 28352) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6242) as uint16_t,
                            (32768 - 15281) as uint16_t,
                            (32768 - 21043) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 25817) as uint16_t,
                            (32768 - 32243) as uint16_t,
                            (32768 - 32720) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18618) as uint16_t,
                            (32768 - 31367) as uint16_t,
                            (32768 - 32325) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13997) as uint16_t,
                            (32768 - 28318) as uint16_t,
                            (32768 - 31878) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12255) as uint16_t,
                            (32768 - 26534) as uint16_t,
                            (32768 - 31383) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9561) as uint16_t,
                            (32768 - 21588) as uint16_t,
                            (32768 - 28450) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28188) as uint16_t,
                            (32768 - 32635) as uint16_t,
                            (32768 - 32724) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22060) as uint16_t,
                            (32768 - 32365) as uint16_t,
                            (32768 - 32728) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18102) as uint16_t,
                            (32768 - 30690) as uint16_t,
                            (32768 - 32528) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14196) as uint16_t,
                            (32768 - 28864) as uint16_t,
                            (32768 - 31999) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12262) as uint16_t,
                            (32768 - 25792) as uint16_t,
                            (32768 - 30865) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24176) as uint16_t,
                            (32768 - 32109) as uint16_t,
                            (32768 - 32628) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18280) as uint16_t,
                            (32768 - 29681) as uint16_t,
                            (32768 - 31963) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10205) as uint16_t,
                            (32768 - 23703) as uint16_t,
                            (32768 - 29664) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7889) as uint16_t,
                            (32768 - 20025) as uint16_t,
                            (32768 - 27676) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6060) as uint16_t,
                            (32768 - 16743) as uint16_t,
                            (32768 - 23970) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 5141) as uint16_t,
                            (32768 - 7096) as uint16_t,
                            (32768 - 8260) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27186) as uint16_t,
                            (32768 - 29022) as uint16_t,
                            (32768 - 29789) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6668) as uint16_t,
                            (32768 - 12568) as uint16_t,
                            (32768 - 15682) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2172) as uint16_t,
                            (32768 - 6181) as uint16_t,
                            (32768 - 8638) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1126) as uint16_t,
                            (32768 - 3379) as uint16_t,
                            (32768 - 4531) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 443) as uint16_t,
                            (32768 - 1361) as uint16_t,
                            (32768 - 2254) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 26083) as uint16_t,
                            (32768 - 31153) as uint16_t,
                            (32768 - 32436) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13486) as uint16_t,
                            (32768 - 24603) as uint16_t,
                            (32768 - 28483) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6508) as uint16_t,
                            (32768 - 14840) as uint16_t,
                            (32768 - 19910) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3386) as uint16_t,
                            (32768 - 8800) as uint16_t,
                            (32768 - 13286) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1530) as uint16_t,
                            (32768 - 4322) as uint16_t,
                            (32768 - 7054) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29639) as uint16_t,
                            (32768 - 32080) as uint16_t,
                            (32768 - 32548) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15897) as uint16_t,
                            (32768 - 27552) as uint16_t,
                            (32768 - 30290) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8588) as uint16_t,
                            (32768 - 20047) as uint16_t,
                            (32768 - 25383) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4889) as uint16_t,
                            (32768 - 13339) as uint16_t,
                            (32768 - 19269) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2240) as uint16_t,
                            (32768 - 6871) as uint16_t,
                            (32768 - 10498) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28165) as uint16_t,
                            (32768 - 32197) as uint16_t,
                            (32768 - 32517) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20735) as uint16_t,
                            (32768 - 30427) as uint16_t,
                            (32768 - 31568) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14325) as uint16_t,
                            (32768 - 24671) as uint16_t,
                            (32768 - 27692) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5119) as uint16_t,
                            (32768 - 12554) as uint16_t,
                            (32768 - 17805) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1810) as uint16_t,
                            (32768 - 5441) as uint16_t,
                            (32768 - 8261) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 31212) as uint16_t,
                            (32768 - 32724) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23352) as uint16_t,
                            (32768 - 31766) as uint16_t,
                            (32768 - 32545) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14669) as uint16_t,
                            (32768 - 27570) as uint16_t,
                            (32768 - 31059) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8492) as uint16_t,
                            (32768 - 20894) as uint16_t,
                            (32768 - 27272) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3644) as uint16_t,
                            (32768 - 10194) as uint16_t,
                            (32768 - 15204) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 2461) as uint16_t,
                            (32768 - 7013) as uint16_t,
                            (32768 - 9371) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24749) as uint16_t,
                            (32768 - 29600) as uint16_t,
                            (32768 - 30986) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9466) as uint16_t,
                            (32768 - 19037) as uint16_t,
                            (32768 - 22417) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3584) as uint16_t,
                            (32768 - 9280) as uint16_t,
                            (32768 - 14400) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1505) as uint16_t,
                            (32768 - 3929) as uint16_t,
                            (32768 - 5433) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 677) as uint16_t,
                            (32768 - 1500) as uint16_t,
                            (32768 - 2736) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23987) as uint16_t,
                            (32768 - 30702) as uint16_t,
                            (32768 - 32117) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13554) as uint16_t,
                            (32768 - 24571) as uint16_t,
                            (32768 - 29263) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6211) as uint16_t,
                            (32768 - 14556) as uint16_t,
                            (32768 - 21155) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3135) as uint16_t,
                            (32768 - 10972) as uint16_t,
                            (32768 - 15625) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2435) as uint16_t,
                            (32768 - 7127) as uint16_t,
                            (32768 - 11427) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 31300) as uint16_t,
                            (32768 - 32532) as uint16_t,
                            (32768 - 32550) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14757) as uint16_t,
                            (32768 - 30365) as uint16_t,
                            (32768 - 31954) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4405) as uint16_t,
                            (32768 - 11612) as uint16_t,
                            (32768 - 18553) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 580) as uint16_t,
                            (32768 - 4132) as uint16_t,
                            (32768 - 7322) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1695) as uint16_t,
                            (32768 - 10169) as uint16_t,
                            (32768 - 14124) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 30008) as uint16_t,
                            (32768 - 32282) as uint16_t,
                            (32768 - 32591) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19244) as uint16_t,
                            (32768 - 30108) as uint16_t,
                            (32768 - 31748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11180) as uint16_t,
                            (32768 - 24158) as uint16_t,
                            (32768 - 29555) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5650) as uint16_t,
                            (32768 - 14972) as uint16_t,
                            (32768 - 19209) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2114) as uint16_t,
                            (32768 - 5109) as uint16_t,
                            (32768 - 8456) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 31856) as uint16_t,
                            (32768 - 32716) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23012) as uint16_t,
                            (32768 - 31664) as uint16_t,
                            (32768 - 32572) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13694) as uint16_t,
                            (32768 - 26656) as uint16_t,
                            (32768 - 30636) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8142) as uint16_t,
                            (32768 - 19508) as uint16_t,
                            (32768 - 26093) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4253) as uint16_t,
                            (32768 - 10955) as uint16_t,
                            (32768 - 16724) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 601) as uint16_t,
                            (32768 - 983) as uint16_t,
                            (32768 - 1311) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18725) as uint16_t,
                            (32768 - 23406) as uint16_t,
                            (32768 - 28087) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5461) as uint16_t,
                            (32768 - 8192) as uint16_t,
                            (32768 - 10923) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3781) as uint16_t,
                            (32768 - 15124) as uint16_t,
                            (32768 - 21425) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2587) as uint16_t,
                            (32768 - 7761) as uint16_t,
                            (32768 - 12072) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 106) as uint16_t,
                            (32768 - 458) as uint16_t,
                            (32768 - 810) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22282) as uint16_t,
                            (32768 - 29710) as uint16_t,
                            (32768 - 31894) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8508) as uint16_t,
                            (32768 - 20926) as uint16_t,
                            (32768 - 25984) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3726) as uint16_t,
                            (32768 - 12713) as uint16_t,
                            (32768 - 18083) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1620) as uint16_t,
                            (32768 - 7112) as uint16_t,
                            (32768 - 10893) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 729) as uint16_t,
                            (32768 - 2236) as uint16_t,
                            (32768 - 3495) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 30163) as uint16_t,
                            (32768 - 32474) as uint16_t,
                            (32768 - 32684) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18304) as uint16_t,
                            (32768 - 30464) as uint16_t,
                            (32768 - 32000) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11443) as uint16_t,
                            (32768 - 26526) as uint16_t,
                            (32768 - 29647) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6007) as uint16_t,
                            (32768 - 15292) as uint16_t,
                            (32768 - 21299) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2234) as uint16_t,
                            (32768 - 6703) as uint16_t,
                            (32768 - 8937) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 30954) as uint16_t,
                            (32768 - 32177) as uint16_t,
                            (32768 - 32571) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17363) as uint16_t,
                            (32768 - 29562) as uint16_t,
                            (32768 - 31076) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9686) as uint16_t,
                            (32768 - 22464) as uint16_t,
                            (32768 - 27410) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 21390) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1755) as uint16_t,
                            (32768 - 8046) as uint16_t,
                            (32768 - 11264) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 31168) as uint16_t,
                            (32768 - 32734) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22486) as uint16_t,
                            (32768 - 31441) as uint16_t,
                            (32768 - 32471) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12833) as uint16_t,
                            (32768 - 25627) as uint16_t,
                            (32768 - 29738) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6980) as uint16_t,
                            (32768 - 17379) as uint16_t,
                            (32768 - 23122) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3111) as uint16_t,
                            (32768 - 8887) as uint16_t,
                            (32768 - 13479) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                    ],
                ],
            ].into(),
            br_tok: [
                [
                    [
                        [
                            (32768 - 14298) as uint16_t,
                            (32768 - 20718) as uint16_t,
                            (32768 - 24174) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12536) as uint16_t,
                            (32768 - 19601) as uint16_t,
                            (32768 - 23789) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8712) as uint16_t,
                            (32768 - 15051) as uint16_t,
                            (32768 - 19503) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6170) as uint16_t,
                            (32768 - 11327) as uint16_t,
                            (32768 - 15434) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4742) as uint16_t,
                            (32768 - 8926) as uint16_t,
                            (32768 - 12538) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3803) as uint16_t,
                            (32768 - 7317) as uint16_t,
                            (32768 - 10546) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1696) as uint16_t,
                            (32768 - 3317) as uint16_t,
                            (32768 - 4871) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14392) as uint16_t,
                            (32768 - 19951) as uint16_t,
                            (32768 - 22756) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15978) as uint16_t,
                            (32768 - 23218) as uint16_t,
                            (32768 - 26818) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12187) as uint16_t,
                            (32768 - 19474) as uint16_t,
                            (32768 - 23889) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9176) as uint16_t,
                            (32768 - 15640) as uint16_t,
                            (32768 - 20259) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7068) as uint16_t,
                            (32768 - 12655) as uint16_t,
                            (32768 - 17028) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5656) as uint16_t,
                            (32768 - 10442) as uint16_t,
                            (32768 - 14472) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2580) as uint16_t,
                            (32768 - 4992) as uint16_t,
                            (32768 - 7244) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12136) as uint16_t,
                            (32768 - 18049) as uint16_t,
                            (32768 - 21426) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13784) as uint16_t,
                            (32768 - 20721) as uint16_t,
                            (32768 - 24481) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10836) as uint16_t,
                            (32768 - 17621) as uint16_t,
                            (32768 - 21900) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8372) as uint16_t,
                            (32768 - 14444) as uint16_t,
                            (32768 - 18847) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6523) as uint16_t,
                            (32768 - 11779) as uint16_t,
                            (32768 - 16000) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5337) as uint16_t,
                            (32768 - 9898) as uint16_t,
                            (32768 - 13760) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3034) as uint16_t,
                            (32768 - 5860) as uint16_t,
                            (32768 - 8462) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 15967) as uint16_t,
                            (32768 - 22905) as uint16_t,
                            (32768 - 26286) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13534) as uint16_t,
                            (32768 - 20654) as uint16_t,
                            (32768 - 24579) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9504) as uint16_t,
                            (32768 - 16092) as uint16_t,
                            (32768 - 20535) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6975) as uint16_t,
                            (32768 - 12568) as uint16_t,
                            (32768 - 16903) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5364) as uint16_t,
                            (32768 - 10091) as uint16_t,
                            (32768 - 14020) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4357) as uint16_t,
                            (32768 - 8370) as uint16_t,
                            (32768 - 11857) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2506) as uint16_t,
                            (32768 - 4934) as uint16_t,
                            (32768 - 7218) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23032) as uint16_t,
                            (32768 - 28815) as uint16_t,
                            (32768 - 30936) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19540) as uint16_t,
                            (32768 - 26704) as uint16_t,
                            (32768 - 29719) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15158) as uint16_t,
                            (32768 - 22969) as uint16_t,
                            (32768 - 27097) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11408) as uint16_t,
                            (32768 - 18865) as uint16_t,
                            (32768 - 23650) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8885) as uint16_t,
                            (32768 - 15448) as uint16_t,
                            (32768 - 20250) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7108) as uint16_t,
                            (32768 - 12853) as uint16_t,
                            (32768 - 17416) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4231) as uint16_t,
                            (32768 - 8041) as uint16_t,
                            (32768 - 11480) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19823) as uint16_t,
                            (32768 - 26490) as uint16_t,
                            (32768 - 29156) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18890) as uint16_t,
                            (32768 - 25929) as uint16_t,
                            (32768 - 28932) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15660) as uint16_t,
                            (32768 - 23491) as uint16_t,
                            (32768 - 27433) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12147) as uint16_t,
                            (32768 - 19776) as uint16_t,
                            (32768 - 24488) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9728) as uint16_t,
                            (32768 - 16774) as uint16_t,
                            (32768 - 21649) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7919) as uint16_t,
                            (32768 - 14277) as uint16_t,
                            (32768 - 19066) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5440) as uint16_t,
                            (32768 - 10170) as uint16_t,
                            (32768 - 14185) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 14406) as uint16_t,
                            (32768 - 20862) as uint16_t,
                            (32768 - 24414) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11824) as uint16_t,
                            (32768 - 18907) as uint16_t,
                            (32768 - 23109) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8257) as uint16_t,
                            (32768 - 14393) as uint16_t,
                            (32768 - 18803) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5860) as uint16_t,
                            (32768 - 10747) as uint16_t,
                            (32768 - 14778) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4475) as uint16_t,
                            (32768 - 8486) as uint16_t,
                            (32768 - 11984) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3606) as uint16_t,
                            (32768 - 6954) as uint16_t,
                            (32768 - 10043) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1736) as uint16_t,
                            (32768 - 3410) as uint16_t,
                            (32768 - 5048) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14430) as uint16_t,
                            (32768 - 20046) as uint16_t,
                            (32768 - 22882) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15593) as uint16_t,
                            (32768 - 22899) as uint16_t,
                            (32768 - 26709) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12102) as uint16_t,
                            (32768 - 19368) as uint16_t,
                            (32768 - 23811) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9059) as uint16_t,
                            (32768 - 15584) as uint16_t,
                            (32768 - 20262) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6999) as uint16_t,
                            (32768 - 12603) as uint16_t,
                            (32768 - 17048) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5684) as uint16_t,
                            (32768 - 10497) as uint16_t,
                            (32768 - 14553) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2822) as uint16_t,
                            (32768 - 5438) as uint16_t,
                            (32768 - 7862) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15785) as uint16_t,
                            (32768 - 21585) as uint16_t,
                            (32768 - 24359) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18347) as uint16_t,
                            (32768 - 25229) as uint16_t,
                            (32768 - 28266) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14974) as uint16_t,
                            (32768 - 22487) as uint16_t,
                            (32768 - 26389) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11423) as uint16_t,
                            (32768 - 18681) as uint16_t,
                            (32768 - 23271) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8863) as uint16_t,
                            (32768 - 15350) as uint16_t,
                            (32768 - 20008) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7153) as uint16_t,
                            (32768 - 12852) as uint16_t,
                            (32768 - 17278) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3707) as uint16_t,
                            (32768 - 7036) as uint16_t,
                            (32768 - 9982) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 15460) as uint16_t,
                            (32768 - 21696) as uint16_t,
                            (32768 - 25469) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12170) as uint16_t,
                            (32768 - 19249) as uint16_t,
                            (32768 - 23191) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8723) as uint16_t,
                            (32768 - 15027) as uint16_t,
                            (32768 - 19332) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6428) as uint16_t,
                            (32768 - 11704) as uint16_t,
                            (32768 - 15874) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4922) as uint16_t,
                            (32768 - 9292) as uint16_t,
                            (32768 - 13052) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4139) as uint16_t,
                            (32768 - 7695) as uint16_t,
                            (32768 - 11010) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2291) as uint16_t,
                            (32768 - 4508) as uint16_t,
                            (32768 - 6598) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19856) as uint16_t,
                            (32768 - 26920) as uint16_t,
                            (32768 - 29828) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17923) as uint16_t,
                            (32768 - 25289) as uint16_t,
                            (32768 - 28792) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14278) as uint16_t,
                            (32768 - 21968) as uint16_t,
                            (32768 - 26297) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10910) as uint16_t,
                            (32768 - 18136) as uint16_t,
                            (32768 - 22950) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8423) as uint16_t,
                            (32768 - 14815) as uint16_t,
                            (32768 - 19627) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6771) as uint16_t,
                            (32768 - 12283) as uint16_t,
                            (32768 - 16774) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4074) as uint16_t,
                            (32768 - 7750) as uint16_t,
                            (32768 - 11081) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19852) as uint16_t,
                            (32768 - 26074) as uint16_t,
                            (32768 - 28672) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19371) as uint16_t,
                            (32768 - 26110) as uint16_t,
                            (32768 - 28989) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16265) as uint16_t,
                            (32768 - 23873) as uint16_t,
                            (32768 - 27663) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12758) as uint16_t,
                            (32768 - 20378) as uint16_t,
                            (32768 - 24952) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10095) as uint16_t,
                            (32768 - 17098) as uint16_t,
                            (32768 - 21961) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8250) as uint16_t,
                            (32768 - 14628) as uint16_t,
                            (32768 - 19451) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5205) as uint16_t,
                            (32768 - 9745) as uint16_t,
                            (32768 - 13622) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 10563) as uint16_t,
                            (32768 - 16233) as uint16_t,
                            (32768 - 19763) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9794) as uint16_t,
                            (32768 - 16022) as uint16_t,
                            (32768 - 19804) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6750) as uint16_t,
                            (32768 - 11945) as uint16_t,
                            (32768 - 15759) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4963) as uint16_t,
                            (32768 - 9186) as uint16_t,
                            (32768 - 12752) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3845) as uint16_t,
                            (32768 - 7435) as uint16_t,
                            (32768 - 10627) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3051) as uint16_t,
                            (32768 - 6085) as uint16_t,
                            (32768 - 8834) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1311) as uint16_t,
                            (32768 - 2596) as uint16_t,
                            (32768 - 3830) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11246) as uint16_t,
                            (32768 - 16404) as uint16_t,
                            (32768 - 19689) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12315) as uint16_t,
                            (32768 - 18911) as uint16_t,
                            (32768 - 22731) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10557) as uint16_t,
                            (32768 - 17095) as uint16_t,
                            (32768 - 21289) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8136) as uint16_t,
                            (32768 - 14006) as uint16_t,
                            (32768 - 18249) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6348) as uint16_t,
                            (32768 - 11474) as uint16_t,
                            (32768 - 15565) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5196) as uint16_t,
                            (32768 - 9655) as uint16_t,
                            (32768 - 13400) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2349) as uint16_t,
                            (32768 - 4526) as uint16_t,
                            (32768 - 6587) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13337) as uint16_t,
                            (32768 - 18730) as uint16_t,
                            (32768 - 21569) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19306) as uint16_t,
                            (32768 - 26071) as uint16_t,
                            (32768 - 28882) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15952) as uint16_t,
                            (32768 - 23540) as uint16_t,
                            (32768 - 27254) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12409) as uint16_t,
                            (32768 - 19934) as uint16_t,
                            (32768 - 24430) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9760) as uint16_t,
                            (32768 - 16706) as uint16_t,
                            (32768 - 21389) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8004) as uint16_t,
                            (32768 - 14220) as uint16_t,
                            (32768 - 18818) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4138) as uint16_t,
                            (32768 - 7794) as uint16_t,
                            (32768 - 10961) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 10870) as uint16_t,
                            (32768 - 16684) as uint16_t,
                            (32768 - 20949) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9664) as uint16_t,
                            (32768 - 15230) as uint16_t,
                            (32768 - 18680) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6886) as uint16_t,
                            (32768 - 12109) as uint16_t,
                            (32768 - 15408) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4825) as uint16_t,
                            (32768 - 8900) as uint16_t,
                            (32768 - 12305) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3630) as uint16_t,
                            (32768 - 7162) as uint16_t,
                            (32768 - 10314) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3036) as uint16_t,
                            (32768 - 6429) as uint16_t,
                            (32768 - 9387) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1671) as uint16_t,
                            (32768 - 3296) as uint16_t,
                            (32768 - 4940) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13819) as uint16_t,
                            (32768 - 19159) as uint16_t,
                            (32768 - 23026) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11984) as uint16_t,
                            (32768 - 19108) as uint16_t,
                            (32768 - 23120) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10690) as uint16_t,
                            (32768 - 17210) as uint16_t,
                            (32768 - 21663) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7984) as uint16_t,
                            (32768 - 14154) as uint16_t,
                            (32768 - 18333) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6868) as uint16_t,
                            (32768 - 12294) as uint16_t,
                            (32768 - 16124) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5274) as uint16_t,
                            (32768 - 8994) as uint16_t,
                            (32768 - 12868) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2988) as uint16_t,
                            (32768 - 5771) as uint16_t,
                            (32768 - 8424) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19736) as uint16_t,
                            (32768 - 26647) as uint16_t,
                            (32768 - 29141) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18933) as uint16_t,
                            (32768 - 26070) as uint16_t,
                            (32768 - 28984) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15779) as uint16_t,
                            (32768 - 23048) as uint16_t,
                            (32768 - 27200) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12638) as uint16_t,
                            (32768 - 20061) as uint16_t,
                            (32768 - 24532) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10692) as uint16_t,
                            (32768 - 17545) as uint16_t,
                            (32768 - 22220) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9217) as uint16_t,
                            (32768 - 15251) as uint16_t,
                            (32768 - 20054) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5078) as uint16_t,
                            (32768 - 9284) as uint16_t,
                            (32768 - 12594) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 2331) as uint16_t,
                            (32768 - 3662) as uint16_t,
                            (32768 - 5244) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2891) as uint16_t,
                            (32768 - 4771) as uint16_t,
                            (32768 - 6145) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4598) as uint16_t,
                            (32768 - 7623) as uint16_t,
                            (32768 - 9729) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3520) as uint16_t,
                            (32768 - 6845) as uint16_t,
                            (32768 - 9199) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3417) as uint16_t,
                            (32768 - 6119) as uint16_t,
                            (32768 - 9324) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2601) as uint16_t,
                            (32768 - 5412) as uint16_t,
                            (32768 - 7385) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 600) as uint16_t,
                            (32768 - 1173) as uint16_t,
                            (32768 - 1744) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7672) as uint16_t,
                            (32768 - 13286) as uint16_t,
                            (32768 - 17469) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4232) as uint16_t,
                            (32768 - 7792) as uint16_t,
                            (32768 - 10793) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2915) as uint16_t,
                            (32768 - 5317) as uint16_t,
                            (32768 - 7397) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2318) as uint16_t,
                            (32768 - 4356) as uint16_t,
                            (32768 - 6152) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2127) as uint16_t,
                            (32768 - 4000) as uint16_t,
                            (32768 - 5554) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1850) as uint16_t,
                            (32768 - 3478) as uint16_t,
                            (32768 - 5275) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 977) as uint16_t,
                            (32768 - 1933) as uint16_t,
                            (32768 - 2843) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18280) as uint16_t,
                            (32768 - 24387) as uint16_t,
                            (32768 - 27989) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15852) as uint16_t,
                            (32768 - 22671) as uint16_t,
                            (32768 - 26185) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13845) as uint16_t,
                            (32768 - 20951) as uint16_t,
                            (32768 - 24789) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11055) as uint16_t,
                            (32768 - 17966) as uint16_t,
                            (32768 - 22129) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9138) as uint16_t,
                            (32768 - 15422) as uint16_t,
                            (32768 - 19801) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7454) as uint16_t,
                            (32768 - 13145) as uint16_t,
                            (32768 - 17456) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3370) as uint16_t,
                            (32768 - 6393) as uint16_t,
                            (32768 - 9013) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 5842) as uint16_t,
                            (32768 - 9229) as uint16_t,
                            (32768 - 10838) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2313) as uint16_t,
                            (32768 - 3491) as uint16_t,
                            (32768 - 4276) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2998) as uint16_t,
                            (32768 - 6104) as uint16_t,
                            (32768 - 7496) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2420) as uint16_t,
                            (32768 - 7447) as uint16_t,
                            (32768 - 9868) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3034) as uint16_t,
                            (32768 - 8495) as uint16_t,
                            (32768 - 10923) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4076) as uint16_t,
                            (32768 - 8937) as uint16_t,
                            (32768 - 10975) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1086) as uint16_t,
                            (32768 - 2370) as uint16_t,
                            (32768 - 3299) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9714) as uint16_t,
                            (32768 - 17254) as uint16_t,
                            (32768 - 20444) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8543) as uint16_t,
                            (32768 - 13698) as uint16_t,
                            (32768 - 17123) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4918) as uint16_t,
                            (32768 - 9007) as uint16_t,
                            (32768 - 11910) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4129) as uint16_t,
                            (32768 - 7532) as uint16_t,
                            (32768 - 10553) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2364) as uint16_t,
                            (32768 - 5533) as uint16_t,
                            (32768 - 8058) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1834) as uint16_t,
                            (32768 - 3546) as uint16_t,
                            (32768 - 5563) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1473) as uint16_t,
                            (32768 - 2908) as uint16_t,
                            (32768 - 4133) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15405) as uint16_t,
                            (32768 - 21193) as uint16_t,
                            (32768 - 25619) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15691) as uint16_t,
                            (32768 - 21952) as uint16_t,
                            (32768 - 26561) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12962) as uint16_t,
                            (32768 - 19194) as uint16_t,
                            (32768 - 24165) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10272) as uint16_t,
                            (32768 - 17855) as uint16_t,
                            (32768 - 22129) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8588) as uint16_t,
                            (32768 - 15270) as uint16_t,
                            (32768 - 20718) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8682) as uint16_t,
                            (32768 - 14669) as uint16_t,
                            (32768 - 19500) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4870) as uint16_t,
                            (32768 - 9636) as uint16_t,
                            (32768 - 13205) as uint16_t,
                            0,
                        ],
                    ],
                ],
            ].into(),
            eob_hi_bit: [
                [
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16961) as uint16_t, 0],
                        [(32768 - 17223) as uint16_t, 0],
                        [(32768 - 7621) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 19069) as uint16_t, 0],
                        [(32768 - 22525) as uint16_t, 0],
                        [(32768 - 13377) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                ],
                [
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 20401) as uint16_t, 0],
                        [(32768 - 17025) as uint16_t, 0],
                        [(32768 - 12845) as uint16_t, 0],
                        [(32768 - 12873) as uint16_t, 0],
                        [(32768 - 14094) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 20681) as uint16_t, 0],
                        [(32768 - 20701) as uint16_t, 0],
                        [(32768 - 15250) as uint16_t, 0],
                        [(32768 - 15017) as uint16_t, 0],
                        [(32768 - 14928) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                ],
                [
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 23905) as uint16_t, 0],
                        [(32768 - 17194) as uint16_t, 0],
                        [(32768 - 16170) as uint16_t, 0],
                        [(32768 - 17695) as uint16_t, 0],
                        [(32768 - 13826) as uint16_t, 0],
                        [(32768 - 15810) as uint16_t, 0],
                        [(32768 - 12036) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 23959) as uint16_t, 0],
                        [(32768 - 20799) as uint16_t, 0],
                        [(32768 - 19021) as uint16_t, 0],
                        [(32768 - 16203) as uint16_t, 0],
                        [(32768 - 17886) as uint16_t, 0],
                        [(32768 - 14144) as uint16_t, 0],
                        [(32768 - 12010) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                ],
                [
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 27399) as uint16_t, 0],
                        [(32768 - 16327) as uint16_t, 0],
                        [(32768 - 18071) as uint16_t, 0],
                        [(32768 - 19584) as uint16_t, 0],
                        [(32768 - 20721) as uint16_t, 0],
                        [(32768 - 18432) as uint16_t, 0],
                        [(32768 - 19560) as uint16_t, 0],
                        [(32768 - 10150) as uint16_t, 0],
                        [(32768 - 8805) as uint16_t, 0],
                    ],
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 24932) as uint16_t, 0],
                        [(32768 - 20833) as uint16_t, 0],
                        [(32768 - 12027) as uint16_t, 0],
                        [(32768 - 16670) as uint16_t, 0],
                        [(32768 - 19914) as uint16_t, 0],
                        [(32768 - 15106) as uint16_t, 0],
                        [(32768 - 17662) as uint16_t, 0],
                        [(32768 - 13783) as uint16_t, 0],
                        [(32768 - 28756) as uint16_t, 0],
                    ],
                ],
                [
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 23406) as uint16_t, 0],
                        [(32768 - 21845) as uint16_t, 0],
                        [(32768 - 18432) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 17096) as uint16_t, 0],
                        [(32768 - 12561) as uint16_t, 0],
                        [(32768 - 17320) as uint16_t, 0],
                        [(32768 - 22395) as uint16_t, 0],
                        [(32768 - 21370) as uint16_t, 0],
                    ],
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                ],
            ].into(),
            skip: [
                [
                    [(32768 - 31849) as uint16_t, 0],
                    [(32768 - 5892) as uint16_t, 0],
                    [(32768 - 12112) as uint16_t, 0],
                    [(32768 - 21935) as uint16_t, 0],
                    [(32768 - 20289) as uint16_t, 0],
                    [(32768 - 27473) as uint16_t, 0],
                    [(32768 - 32487) as uint16_t, 0],
                    [(32768 - 7654) as uint16_t, 0],
                    [(32768 - 19473) as uint16_t, 0],
                    [(32768 - 29984) as uint16_t, 0],
                    [(32768 - 9961) as uint16_t, 0],
                    [(32768 - 30242) as uint16_t, 0],
                    [(32768 - 32117) as uint16_t, 0],
                ],
                [
                    [(32768 - 31548) as uint16_t, 0],
                    [(32768 - 1549) as uint16_t, 0],
                    [(32768 - 10130) as uint16_t, 0],
                    [(32768 - 16656) as uint16_t, 0],
                    [(32768 - 18591) as uint16_t, 0],
                    [(32768 - 26308) as uint16_t, 0],
                    [(32768 - 32537) as uint16_t, 0],
                    [(32768 - 5403) as uint16_t, 0],
                    [(32768 - 18096) as uint16_t, 0],
                    [(32768 - 30003) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                ],
                [
                    [(32768 - 29957) as uint16_t, 0],
                    [(32768 - 5391) as uint16_t, 0],
                    [(32768 - 18039) as uint16_t, 0],
                    [(32768 - 23566) as uint16_t, 0],
                    [(32768 - 22431) as uint16_t, 0],
                    [(32768 - 25822) as uint16_t, 0],
                    [(32768 - 32197) as uint16_t, 0],
                    [(32768 - 3778) as uint16_t, 0],
                    [(32768 - 15336) as uint16_t, 0],
                    [(32768 - 28981) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                ],
                [
                    [(32768 - 17920) as uint16_t, 0],
                    [(32768 - 1818) as uint16_t, 0],
                    [(32768 - 7282) as uint16_t, 0],
                    [(32768 - 25273) as uint16_t, 0],
                    [(32768 - 10923) as uint16_t, 0],
                    [(32768 - 31554) as uint16_t, 0],
                    [(32768 - 32624) as uint16_t, 0],
                    [(32768 - 1366) as uint16_t, 0],
                    [(32768 - 15628) as uint16_t, 0],
                    [(32768 - 30462) as uint16_t, 0],
                    [(32768 - 146) as uint16_t, 0],
                    [(32768 - 5132) as uint16_t, 0],
                    [(32768 - 31657) as uint16_t, 0],
                ],
                [
                    [(32768 - 6308) as uint16_t, 0],
                    [(32768 - 117) as uint16_t, 0],
                    [(32768 - 1638) as uint16_t, 0],
                    [(32768 - 2161) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 10923) as uint16_t, 0],
                    [(32768 - 30247) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                ],
            ].into(),
            dc_sign: [
                [
                    [(32768 - 16000) as uint16_t, 0],
                    [(32768 - 13056) as uint16_t, 0],
                    [(32768 - 18816) as uint16_t, 0],
                ],
                [
                    [(32768 - 15232) as uint16_t, 0],
                    [(32768 - 12928) as uint16_t, 0],
                    [(32768 - 17280) as uint16_t, 0],
                ],
            ].into(),
        };
        init
    },
    {
        let mut init = CdfCoefContext {
            eob_bin_16: [
                [
                    [
                        (32768 - 2125) as uint16_t,
                        (32768 - 2551) as uint16_t,
                        (32768 - 5165) as uint16_t,
                        (32768 - 8946) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 513) as uint16_t,
                        (32768 - 765) as uint16_t,
                        (32768 - 1859) as uint16_t,
                        (32768 - 6339) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 7637) as uint16_t,
                        (32768 - 9498) as uint16_t,
                        (32768 - 14259) as uint16_t,
                        (32768 - 19108) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 2497) as uint16_t,
                        (32768 - 4096) as uint16_t,
                        (32768 - 8866) as uint16_t,
                        (32768 - 16993) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                ],
            ].into(),
            eob_bin_32: [
                [
                    [
                        (32768 - 989) as uint16_t,
                        (32768 - 1249) as uint16_t,
                        (32768 - 2019) as uint16_t,
                        (32768 - 4151) as uint16_t,
                        (32768 - 10785) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 313) as uint16_t,
                        (32768 - 441) as uint16_t,
                        (32768 - 1099) as uint16_t,
                        (32768 - 2917) as uint16_t,
                        (32768 - 8562) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 8394) as uint16_t,
                        (32768 - 10352) as uint16_t,
                        (32768 - 13932) as uint16_t,
                        (32768 - 18855) as uint16_t,
                        (32768 - 26014) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 2578) as uint16_t,
                        (32768 - 4124) as uint16_t,
                        (32768 - 8181) as uint16_t,
                        (32768 - 13670) as uint16_t,
                        (32768 - 24234) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                ],
            ].into(),
            eob_bin_64: [
                [
                    [
                        (32768 - 1260) as uint16_t,
                        (32768 - 1446) as uint16_t,
                        (32768 - 2253) as uint16_t,
                        (32768 - 3712) as uint16_t,
                        (32768 - 6652) as uint16_t,
                        (32768 - 13369) as uint16_t,
                        0,
                        0,
                    ],
                    [
                        (32768 - 401) as uint16_t,
                        (32768 - 605) as uint16_t,
                        (32768 - 1029) as uint16_t,
                        (32768 - 2563) as uint16_t,
                        (32768 - 5845) as uint16_t,
                        (32768 - 12626) as uint16_t,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 8609) as uint16_t,
                        (32768 - 10612) as uint16_t,
                        (32768 - 14624) as uint16_t,
                        (32768 - 18714) as uint16_t,
                        (32768 - 22614) as uint16_t,
                        (32768 - 29024) as uint16_t,
                        0,
                        0,
                    ],
                    [
                        (32768 - 1923) as uint16_t,
                        (32768 - 3127) as uint16_t,
                        (32768 - 5867) as uint16_t,
                        (32768 - 9703) as uint16_t,
                        (32768 - 14277) as uint16_t,
                        (32768 - 27100) as uint16_t,
                        0,
                        0,
                    ],
                ],
            ].into(),
            eob_bin_128: [
                [
                    [
                        (32768 - 685) as uint16_t,
                        (32768 - 933) as uint16_t,
                        (32768 - 1488) as uint16_t,
                        (32768 - 2714) as uint16_t,
                        (32768 - 4766) as uint16_t,
                        (32768 - 8562) as uint16_t,
                        (32768 - 19254) as uint16_t,
                        0,
                    ],
                    [
                        (32768 - 217) as uint16_t,
                        (32768 - 352) as uint16_t,
                        (32768 - 618) as uint16_t,
                        (32768 - 2303) as uint16_t,
                        (32768 - 5261) as uint16_t,
                        (32768 - 9969) as uint16_t,
                        (32768 - 17472) as uint16_t,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 8045) as uint16_t,
                        (32768 - 11200) as uint16_t,
                        (32768 - 15497) as uint16_t,
                        (32768 - 19595) as uint16_t,
                        (32768 - 23948) as uint16_t,
                        (32768 - 27408) as uint16_t,
                        (32768 - 30938) as uint16_t,
                        0,
                    ],
                    [
                        (32768 - 2310) as uint16_t,
                        (32768 - 4160) as uint16_t,
                        (32768 - 7471) as uint16_t,
                        (32768 - 14997) as uint16_t,
                        (32768 - 17931) as uint16_t,
                        (32768 - 20768) as uint16_t,
                        (32768 - 30240) as uint16_t,
                        0,
                    ],
                ],
            ].into(),
            eob_bin_256: [
                [
                    [
                        (32768 - 1448) as uint16_t,
                        (32768 - 2109) as uint16_t,
                        (32768 - 4151) as uint16_t,
                        (32768 - 6263) as uint16_t,
                        (32768 - 9329) as uint16_t,
                        (32768 - 13260) as uint16_t,
                        (32768 - 17944) as uint16_t,
                        (32768 - 23300) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 399) as uint16_t,
                        (32768 - 1019) as uint16_t,
                        (32768 - 1749) as uint16_t,
                        (32768 - 3038) as uint16_t,
                        (32768 - 10444) as uint16_t,
                        (32768 - 15546) as uint16_t,
                        (32768 - 22739) as uint16_t,
                        (32768 - 27294) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 6402) as uint16_t,
                        (32768 - 8148) as uint16_t,
                        (32768 - 12623) as uint16_t,
                        (32768 - 15072) as uint16_t,
                        (32768 - 18728) as uint16_t,
                        (32768 - 22847) as uint16_t,
                        (32768 - 26447) as uint16_t,
                        (32768 - 29377) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 1674) as uint16_t,
                        (32768 - 3252) as uint16_t,
                        (32768 - 5734) as uint16_t,
                        (32768 - 10159) as uint16_t,
                        (32768 - 22397) as uint16_t,
                        (32768 - 23802) as uint16_t,
                        (32768 - 24821) as uint16_t,
                        (32768 - 30940) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                ],
            ].into(),
            eob_bin_512: [
                [
                    (32768 - 1230) as uint16_t,
                    (32768 - 2278) as uint16_t,
                    (32768 - 5035) as uint16_t,
                    (32768 - 7776) as uint16_t,
                    (32768 - 11871) as uint16_t,
                    (32768 - 15346) as uint16_t,
                    (32768 - 19590) as uint16_t,
                    (32768 - 24584) as uint16_t,
                    (32768 - 28749) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 7265) as uint16_t,
                    (32768 - 9979) as uint16_t,
                    (32768 - 15819) as uint16_t,
                    (32768 - 19250) as uint16_t,
                    (32768 - 21780) as uint16_t,
                    (32768 - 23846) as uint16_t,
                    (32768 - 26478) as uint16_t,
                    (32768 - 28396) as uint16_t,
                    (32768 - 31811) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            ].into(),
            eob_bin_1024: [
                [
                    (32768 - 696) as uint16_t,
                    (32768 - 948) as uint16_t,
                    (32768 - 3145) as uint16_t,
                    (32768 - 5702) as uint16_t,
                    (32768 - 9706) as uint16_t,
                    (32768 - 13217) as uint16_t,
                    (32768 - 17851) as uint16_t,
                    (32768 - 21856) as uint16_t,
                    (32768 - 25692) as uint16_t,
                    (32768 - 28034) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 2672) as uint16_t,
                    (32768 - 3591) as uint16_t,
                    (32768 - 9330) as uint16_t,
                    (32768 - 17084) as uint16_t,
                    (32768 - 22725) as uint16_t,
                    (32768 - 24284) as uint16_t,
                    (32768 - 26527) as uint16_t,
                    (32768 - 28027) as uint16_t,
                    (32768 - 28377) as uint16_t,
                    (32768 - 30876) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            ].into(),
            eob_base_tok: [
                [
                    [
                        [
                            (32768 - 17560) as uint16_t,
                            (32768 - 29888) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 29671) as uint16_t,
                            (32768 - 31549) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31007) as uint16_t,
                            (32768 - 32056) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 27286) as uint16_t,
                            (32768 - 30006) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 26594) as uint16_t,
                            (32768 - 31212) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31208) as uint16_t,
                            (32768 - 32582) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31835) as uint16_t,
                            (32768 - 32637) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 30595) as uint16_t,
                            (32768 - 32206) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 15239) as uint16_t,
                            (32768 - 29932) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31315) as uint16_t,
                            (32768 - 32095) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32130) as uint16_t,
                            (32768 - 32434) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 30864) as uint16_t,
                            (32768 - 31996) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 26279) as uint16_t,
                            (32768 - 30968) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31142) as uint16_t,
                            (32768 - 32495) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31713) as uint16_t,
                            (32768 - 32540) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31929) as uint16_t,
                            (32768 - 32594) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 2644) as uint16_t,
                            (32768 - 25198) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32038) as uint16_t,
                            (32768 - 32451) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32639) as uint16_t,
                            (32768 - 32695) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32166) as uint16_t,
                            (32768 - 32518) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 17187) as uint16_t,
                            (32768 - 27668) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31714) as uint16_t,
                            (32768 - 32550) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32283) as uint16_t,
                            (32768 - 32678) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31930) as uint16_t,
                            (32768 - 32563) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 1044) as uint16_t,
                            (32768 - 2257) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 30755) as uint16_t,
                            (32768 - 31923) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32208) as uint16_t,
                            (32768 - 32693) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32244) as uint16_t,
                            (32768 - 32615) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 21317) as uint16_t,
                            (32768 - 26207) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 29133) as uint16_t,
                            (32768 - 30868) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 29311) as uint16_t,
                            (32768 - 31231) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 29657) as uint16_t,
                            (32768 - 31087) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 478) as uint16_t,
                            (32768 - 1834) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31005) as uint16_t,
                            (32768 - 31987) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32317) as uint16_t,
                            (32768 - 32724) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 30865) as uint16_t,
                            (32768 - 32648) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 10923) as uint16_t,
                            (32768 - 21845) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 10923) as uint16_t,
                            (32768 - 21845) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 10923) as uint16_t,
                            (32768 - 21845) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 10923) as uint16_t,
                            (32768 - 21845) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                ],
            ].into(),
            base_tok: [
                [
                    [
                        [
                            (32768 - 6041) as uint16_t,
                            (32768 - 11854) as uint16_t,
                            (32768 - 15927) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20326) as uint16_t,
                            (32768 - 30905) as uint16_t,
                            (32768 - 32251) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14164) as uint16_t,
                            (32768 - 26831) as uint16_t,
                            (32768 - 30725) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9760) as uint16_t,
                            (32768 - 20647) as uint16_t,
                            (32768 - 26585) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6416) as uint16_t,
                            (32768 - 14953) as uint16_t,
                            (32768 - 21219) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2966) as uint16_t,
                            (32768 - 7151) as uint16_t,
                            (32768 - 10891) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23567) as uint16_t,
                            (32768 - 31374) as uint16_t,
                            (32768 - 32254) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14978) as uint16_t,
                            (32768 - 27416) as uint16_t,
                            (32768 - 30946) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9434) as uint16_t,
                            (32768 - 20225) as uint16_t,
                            (32768 - 26254) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6658) as uint16_t,
                            (32768 - 14558) as uint16_t,
                            (32768 - 20535) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3916) as uint16_t,
                            (32768 - 8677) as uint16_t,
                            (32768 - 12989) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18088) as uint16_t,
                            (32768 - 29545) as uint16_t,
                            (32768 - 31587) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13062) as uint16_t,
                            (32768 - 25843) as uint16_t,
                            (32768 - 30073) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8940) as uint16_t,
                            (32768 - 16827) as uint16_t,
                            (32768 - 22251) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7654) as uint16_t,
                            (32768 - 13220) as uint16_t,
                            (32768 - 17973) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5733) as uint16_t,
                            (32768 - 10316) as uint16_t,
                            (32768 - 14456) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22879) as uint16_t,
                            (32768 - 31388) as uint16_t,
                            (32768 - 32114) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15215) as uint16_t,
                            (32768 - 27993) as uint16_t,
                            (32768 - 30955) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9397) as uint16_t,
                            (32768 - 19445) as uint16_t,
                            (32768 - 24978) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3442) as uint16_t,
                            (32768 - 9813) as uint16_t,
                            (32768 - 15344) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1368) as uint16_t,
                            (32768 - 3936) as uint16_t,
                            (32768 - 6532) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 25494) as uint16_t,
                            (32768 - 32033) as uint16_t,
                            (32768 - 32406) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16772) as uint16_t,
                            (32768 - 27963) as uint16_t,
                            (32768 - 30718) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9419) as uint16_t,
                            (32768 - 18165) as uint16_t,
                            (32768 - 23260) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2677) as uint16_t,
                            (32768 - 7501) as uint16_t,
                            (32768 - 11797) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1516) as uint16_t,
                            (32768 - 4344) as uint16_t,
                            (32768 - 7170) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 26556) as uint16_t,
                            (32768 - 31454) as uint16_t,
                            (32768 - 32101) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17128) as uint16_t,
                            (32768 - 27035) as uint16_t,
                            (32768 - 30108) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8324) as uint16_t,
                            (32768 - 15344) as uint16_t,
                            (32768 - 20249) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1903) as uint16_t,
                            (32768 - 5696) as uint16_t,
                            (32768 - 9469) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 8455) as uint16_t,
                            (32768 - 19003) as uint16_t,
                            (32768 - 24368) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23563) as uint16_t,
                            (32768 - 32021) as uint16_t,
                            (32768 - 32604) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16237) as uint16_t,
                            (32768 - 29446) as uint16_t,
                            (32768 - 31935) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10724) as uint16_t,
                            (32768 - 23999) as uint16_t,
                            (32768 - 29358) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6725) as uint16_t,
                            (32768 - 17528) as uint16_t,
                            (32768 - 24416) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3927) as uint16_t,
                            (32768 - 10927) as uint16_t,
                            (32768 - 16825) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 26313) as uint16_t,
                            (32768 - 32288) as uint16_t,
                            (32768 - 32634) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17430) as uint16_t,
                            (32768 - 30095) as uint16_t,
                            (32768 - 32095) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11116) as uint16_t,
                            (32768 - 24606) as uint16_t,
                            (32768 - 29679) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7195) as uint16_t,
                            (32768 - 18384) as uint16_t,
                            (32768 - 25269) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4726) as uint16_t,
                            (32768 - 12852) as uint16_t,
                            (32768 - 19315) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22822) as uint16_t,
                            (32768 - 31648) as uint16_t,
                            (32768 - 32483) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16724) as uint16_t,
                            (32768 - 29633) as uint16_t,
                            (32768 - 31929) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10261) as uint16_t,
                            (32768 - 23033) as uint16_t,
                            (32768 - 28725) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7029) as uint16_t,
                            (32768 - 17840) as uint16_t,
                            (32768 - 24528) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4867) as uint16_t,
                            (32768 - 13886) as uint16_t,
                            (32768 - 21502) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 25298) as uint16_t,
                            (32768 - 31892) as uint16_t,
                            (32768 - 32491) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17809) as uint16_t,
                            (32768 - 29330) as uint16_t,
                            (32768 - 31512) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9668) as uint16_t,
                            (32768 - 21329) as uint16_t,
                            (32768 - 26579) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4774) as uint16_t,
                            (32768 - 12956) as uint16_t,
                            (32768 - 18976) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2322) as uint16_t,
                            (32768 - 7030) as uint16_t,
                            (32768 - 11540) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 25472) as uint16_t,
                            (32768 - 31920) as uint16_t,
                            (32768 - 32543) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17957) as uint16_t,
                            (32768 - 29387) as uint16_t,
                            (32768 - 31632) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9196) as uint16_t,
                            (32768 - 20593) as uint16_t,
                            (32768 - 26400) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4680) as uint16_t,
                            (32768 - 12705) as uint16_t,
                            (32768 - 19202) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2917) as uint16_t,
                            (32768 - 8456) as uint16_t,
                            (32768 - 13436) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 26471) as uint16_t,
                            (32768 - 32059) as uint16_t,
                            (32768 - 32574) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18458) as uint16_t,
                            (32768 - 29783) as uint16_t,
                            (32768 - 31909) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8400) as uint16_t,
                            (32768 - 19464) as uint16_t,
                            (32768 - 25956) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3812) as uint16_t,
                            (32768 - 10973) as uint16_t,
                            (32768 - 17206) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 6779) as uint16_t,
                            (32768 - 13743) as uint16_t,
                            (32768 - 17678) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24806) as uint16_t,
                            (32768 - 31797) as uint16_t,
                            (32768 - 32457) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17616) as uint16_t,
                            (32768 - 29047) as uint16_t,
                            (32768 - 31372) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11063) as uint16_t,
                            (32768 - 23175) as uint16_t,
                            (32768 - 28003) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6521) as uint16_t,
                            (32768 - 16110) as uint16_t,
                            (32768 - 22324) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2764) as uint16_t,
                            (32768 - 7504) as uint16_t,
                            (32768 - 11654) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 25266) as uint16_t,
                            (32768 - 32367) as uint16_t,
                            (32768 - 32637) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19054) as uint16_t,
                            (32768 - 30553) as uint16_t,
                            (32768 - 32175) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12139) as uint16_t,
                            (32768 - 25212) as uint16_t,
                            (32768 - 29807) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7311) as uint16_t,
                            (32768 - 18162) as uint16_t,
                            (32768 - 24704) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3397) as uint16_t,
                            (32768 - 9164) as uint16_t,
                            (32768 - 14074) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 25988) as uint16_t,
                            (32768 - 32208) as uint16_t,
                            (32768 - 32522) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16253) as uint16_t,
                            (32768 - 28912) as uint16_t,
                            (32768 - 31526) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9151) as uint16_t,
                            (32768 - 21387) as uint16_t,
                            (32768 - 27372) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5688) as uint16_t,
                            (32768 - 14915) as uint16_t,
                            (32768 - 21496) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2717) as uint16_t,
                            (32768 - 7627) as uint16_t,
                            (32768 - 12004) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23144) as uint16_t,
                            (32768 - 31855) as uint16_t,
                            (32768 - 32443) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16070) as uint16_t,
                            (32768 - 28491) as uint16_t,
                            (32768 - 31325) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8702) as uint16_t,
                            (32768 - 20467) as uint16_t,
                            (32768 - 26517) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5243) as uint16_t,
                            (32768 - 13956) as uint16_t,
                            (32768 - 20367) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2621) as uint16_t,
                            (32768 - 7335) as uint16_t,
                            (32768 - 11567) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 26636) as uint16_t,
                            (32768 - 32340) as uint16_t,
                            (32768 - 32630) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19990) as uint16_t,
                            (32768 - 31050) as uint16_t,
                            (32768 - 32341) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13243) as uint16_t,
                            (32768 - 26105) as uint16_t,
                            (32768 - 30315) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8588) as uint16_t,
                            (32768 - 19521) as uint16_t,
                            (32768 - 25918) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4717) as uint16_t,
                            (32768 - 11585) as uint16_t,
                            (32768 - 17304) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 25844) as uint16_t,
                            (32768 - 32292) as uint16_t,
                            (32768 - 32582) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19090) as uint16_t,
                            (32768 - 30635) as uint16_t,
                            (32768 - 32097) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11963) as uint16_t,
                            (32768 - 24546) as uint16_t,
                            (32768 - 28939) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6218) as uint16_t,
                            (32768 - 16087) as uint16_t,
                            (32768 - 22354) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2340) as uint16_t,
                            (32768 - 6608) as uint16_t,
                            (32768 - 10426) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28046) as uint16_t,
                            (32768 - 32576) as uint16_t,
                            (32768 - 32694) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21178) as uint16_t,
                            (32768 - 31313) as uint16_t,
                            (32768 - 32296) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13486) as uint16_t,
                            (32768 - 26184) as uint16_t,
                            (32768 - 29870) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7149) as uint16_t,
                            (32768 - 17871) as uint16_t,
                            (32768 - 23723) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2833) as uint16_t,
                            (32768 - 7958) as uint16_t,
                            (32768 - 12259) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27710) as uint16_t,
                            (32768 - 32528) as uint16_t,
                            (32768 - 32686) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20674) as uint16_t,
                            (32768 - 31076) as uint16_t,
                            (32768 - 32268) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12413) as uint16_t,
                            (32768 - 24955) as uint16_t,
                            (32768 - 29243) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6676) as uint16_t,
                            (32768 - 16927) as uint16_t,
                            (32768 - 23097) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2966) as uint16_t,
                            (32768 - 8333) as uint16_t,
                            (32768 - 12919) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 8639) as uint16_t,
                            (32768 - 19339) as uint16_t,
                            (32768 - 24429) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24404) as uint16_t,
                            (32768 - 31837) as uint16_t,
                            (32768 - 32525) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16997) as uint16_t,
                            (32768 - 29425) as uint16_t,
                            (32768 - 31784) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11253) as uint16_t,
                            (32768 - 24234) as uint16_t,
                            (32768 - 29149) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6751) as uint16_t,
                            (32768 - 17394) as uint16_t,
                            (32768 - 24028) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3490) as uint16_t,
                            (32768 - 9830) as uint16_t,
                            (32768 - 15191) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 26283) as uint16_t,
                            (32768 - 32471) as uint16_t,
                            (32768 - 32714) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19599) as uint16_t,
                            (32768 - 31168) as uint16_t,
                            (32768 - 32442) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13146) as uint16_t,
                            (32768 - 26954) as uint16_t,
                            (32768 - 30893) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8214) as uint16_t,
                            (32768 - 20588) as uint16_t,
                            (32768 - 26890) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4699) as uint16_t,
                            (32768 - 13081) as uint16_t,
                            (32768 - 19300) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28212) as uint16_t,
                            (32768 - 32458) as uint16_t,
                            (32768 - 32669) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18594) as uint16_t,
                            (32768 - 30316) as uint16_t,
                            (32768 - 32100) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11219) as uint16_t,
                            (32768 - 24408) as uint16_t,
                            (32768 - 29234) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6865) as uint16_t,
                            (32768 - 17656) as uint16_t,
                            (32768 - 24149) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3678) as uint16_t,
                            (32768 - 10362) as uint16_t,
                            (32768 - 16006) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 25825) as uint16_t,
                            (32768 - 32136) as uint16_t,
                            (32768 - 32616) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17313) as uint16_t,
                            (32768 - 29853) as uint16_t,
                            (32768 - 32021) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11197) as uint16_t,
                            (32768 - 24471) as uint16_t,
                            (32768 - 29472) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6947) as uint16_t,
                            (32768 - 17781) as uint16_t,
                            (32768 - 24405) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3768) as uint16_t,
                            (32768 - 10660) as uint16_t,
                            (32768 - 16261) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27352) as uint16_t,
                            (32768 - 32500) as uint16_t,
                            (32768 - 32706) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20850) as uint16_t,
                            (32768 - 31468) as uint16_t,
                            (32768 - 32469) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14021) as uint16_t,
                            (32768 - 27707) as uint16_t,
                            (32768 - 31133) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8964) as uint16_t,
                            (32768 - 21748) as uint16_t,
                            (32768 - 27838) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5437) as uint16_t,
                            (32768 - 14665) as uint16_t,
                            (32768 - 21187) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 26304) as uint16_t,
                            (32768 - 32492) as uint16_t,
                            (32768 - 32698) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20409) as uint16_t,
                            (32768 - 31380) as uint16_t,
                            (32768 - 32385) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13682) as uint16_t,
                            (32768 - 27222) as uint16_t,
                            (32768 - 30632) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8974) as uint16_t,
                            (32768 - 21236) as uint16_t,
                            (32768 - 26685) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4234) as uint16_t,
                            (32768 - 11665) as uint16_t,
                            (32768 - 16934) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 26273) as uint16_t,
                            (32768 - 32357) as uint16_t,
                            (32768 - 32711) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20672) as uint16_t,
                            (32768 - 31242) as uint16_t,
                            (32768 - 32441) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14172) as uint16_t,
                            (32768 - 27254) as uint16_t,
                            (32768 - 30902) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9870) as uint16_t,
                            (32768 - 21898) as uint16_t,
                            (32768 - 27275) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5164) as uint16_t,
                            (32768 - 13506) as uint16_t,
                            (32768 - 19270) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 26725) as uint16_t,
                            (32768 - 32459) as uint16_t,
                            (32768 - 32728) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20991) as uint16_t,
                            (32768 - 31442) as uint16_t,
                            (32768 - 32527) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13071) as uint16_t,
                            (32768 - 26434) as uint16_t,
                            (32768 - 30811) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8184) as uint16_t,
                            (32768 - 20090) as uint16_t,
                            (32768 - 26742) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4803) as uint16_t,
                            (32768 - 13255) as uint16_t,
                            (32768 - 19895) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 7555) as uint16_t,
                            (32768 - 14942) as uint16_t,
                            (32768 - 18501) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24410) as uint16_t,
                            (32768 - 31178) as uint16_t,
                            (32768 - 32287) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14394) as uint16_t,
                            (32768 - 26738) as uint16_t,
                            (32768 - 30253) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8413) as uint16_t,
                            (32768 - 19554) as uint16_t,
                            (32768 - 25195) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4766) as uint16_t,
                            (32768 - 12924) as uint16_t,
                            (32768 - 18785) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2029) as uint16_t,
                            (32768 - 5806) as uint16_t,
                            (32768 - 9207) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 26776) as uint16_t,
                            (32768 - 32364) as uint16_t,
                            (32768 - 32663) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18732) as uint16_t,
                            (32768 - 29967) as uint16_t,
                            (32768 - 31931) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11005) as uint16_t,
                            (32768 - 23786) as uint16_t,
                            (32768 - 28852) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6466) as uint16_t,
                            (32768 - 16909) as uint16_t,
                            (32768 - 23510) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3044) as uint16_t,
                            (32768 - 8638) as uint16_t,
                            (32768 - 13419) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29208) as uint16_t,
                            (32768 - 32582) as uint16_t,
                            (32768 - 32704) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20068) as uint16_t,
                            (32768 - 30857) as uint16_t,
                            (32768 - 32208) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12003) as uint16_t,
                            (32768 - 25085) as uint16_t,
                            (32768 - 29595) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6947) as uint16_t,
                            (32768 - 17750) as uint16_t,
                            (32768 - 24189) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3245) as uint16_t,
                            (32768 - 9103) as uint16_t,
                            (32768 - 14007) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27359) as uint16_t,
                            (32768 - 32465) as uint16_t,
                            (32768 - 32669) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19421) as uint16_t,
                            (32768 - 30614) as uint16_t,
                            (32768 - 32174) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11915) as uint16_t,
                            (32768 - 25010) as uint16_t,
                            (32768 - 29579) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6950) as uint16_t,
                            (32768 - 17676) as uint16_t,
                            (32768 - 24074) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3007) as uint16_t,
                            (32768 - 8473) as uint16_t,
                            (32768 - 13096) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29002) as uint16_t,
                            (32768 - 32676) as uint16_t,
                            (32768 - 32735) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22102) as uint16_t,
                            (32768 - 31849) as uint16_t,
                            (32768 - 32576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14408) as uint16_t,
                            (32768 - 28009) as uint16_t,
                            (32768 - 31405) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9027) as uint16_t,
                            (32768 - 21679) as uint16_t,
                            (32768 - 27931) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4694) as uint16_t,
                            (32768 - 12678) as uint16_t,
                            (32768 - 18748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28216) as uint16_t,
                            (32768 - 32528) as uint16_t,
                            (32768 - 32682) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20849) as uint16_t,
                            (32768 - 31264) as uint16_t,
                            (32768 - 32318) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12756) as uint16_t,
                            (32768 - 25815) as uint16_t,
                            (32768 - 29751) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7565) as uint16_t,
                            (32768 - 18801) as uint16_t,
                            (32768 - 24923) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3509) as uint16_t,
                            (32768 - 9533) as uint16_t,
                            (32768 - 14477) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 30133) as uint16_t,
                            (32768 - 32687) as uint16_t,
                            (32768 - 32739) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23063) as uint16_t,
                            (32768 - 31910) as uint16_t,
                            (32768 - 32515) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14588) as uint16_t,
                            (32768 - 28051) as uint16_t,
                            (32768 - 31132) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9085) as uint16_t,
                            (32768 - 21649) as uint16_t,
                            (32768 - 27457) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4261) as uint16_t,
                            (32768 - 11654) as uint16_t,
                            (32768 - 17264) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29518) as uint16_t,
                            (32768 - 32691) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22451) as uint16_t,
                            (32768 - 31959) as uint16_t,
                            (32768 - 32613) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14864) as uint16_t,
                            (32768 - 28722) as uint16_t,
                            (32768 - 31700) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9695) as uint16_t,
                            (32768 - 22964) as uint16_t,
                            (32768 - 28716) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4932) as uint16_t,
                            (32768 - 13358) as uint16_t,
                            (32768 - 19502) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 6465) as uint16_t,
                            (32768 - 16958) as uint16_t,
                            (32768 - 21688) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 25199) as uint16_t,
                            (32768 - 31514) as uint16_t,
                            (32768 - 32360) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14774) as uint16_t,
                            (32768 - 27149) as uint16_t,
                            (32768 - 30607) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9257) as uint16_t,
                            (32768 - 21438) as uint16_t,
                            (32768 - 26972) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5723) as uint16_t,
                            (32768 - 15183) as uint16_t,
                            (32768 - 21882) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3150) as uint16_t,
                            (32768 - 8879) as uint16_t,
                            (32768 - 13731) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 26989) as uint16_t,
                            (32768 - 32262) as uint16_t,
                            (32768 - 32682) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17396) as uint16_t,
                            (32768 - 29937) as uint16_t,
                            (32768 - 32085) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11387) as uint16_t,
                            (32768 - 24901) as uint16_t,
                            (32768 - 29784) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7289) as uint16_t,
                            (32768 - 18821) as uint16_t,
                            (32768 - 25548) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3734) as uint16_t,
                            (32768 - 10577) as uint16_t,
                            (32768 - 16086) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29728) as uint16_t,
                            (32768 - 32501) as uint16_t,
                            (32768 - 32695) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17431) as uint16_t,
                            (32768 - 29701) as uint16_t,
                            (32768 - 31903) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9921) as uint16_t,
                            (32768 - 22826) as uint16_t,
                            (32768 - 28300) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5896) as uint16_t,
                            (32768 - 15434) as uint16_t,
                            (32768 - 22068) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3430) as uint16_t,
                            (32768 - 9646) as uint16_t,
                            (32768 - 14757) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28614) as uint16_t,
                            (32768 - 32511) as uint16_t,
                            (32768 - 32705) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19364) as uint16_t,
                            (32768 - 30638) as uint16_t,
                            (32768 - 32263) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13129) as uint16_t,
                            (32768 - 26254) as uint16_t,
                            (32768 - 30402) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8754) as uint16_t,
                            (32768 - 20484) as uint16_t,
                            (32768 - 26440) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4378) as uint16_t,
                            (32768 - 11607) as uint16_t,
                            (32768 - 17110) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 30292) as uint16_t,
                            (32768 - 32671) as uint16_t,
                            (32768 - 32744) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21780) as uint16_t,
                            (32768 - 31603) as uint16_t,
                            (32768 - 32501) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14314) as uint16_t,
                            (32768 - 27829) as uint16_t,
                            (32768 - 31291) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9611) as uint16_t,
                            (32768 - 22327) as uint16_t,
                            (32768 - 28263) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4890) as uint16_t,
                            (32768 - 13087) as uint16_t,
                            (32768 - 19065) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 25862) as uint16_t,
                            (32768 - 32567) as uint16_t,
                            (32768 - 32733) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20794) as uint16_t,
                            (32768 - 32050) as uint16_t,
                            (32768 - 32567) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17243) as uint16_t,
                            (32768 - 30625) as uint16_t,
                            (32768 - 32254) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13283) as uint16_t,
                            (32768 - 27628) as uint16_t,
                            (32768 - 31474) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9669) as uint16_t,
                            (32768 - 22532) as uint16_t,
                            (32768 - 28918) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27435) as uint16_t,
                            (32768 - 32697) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24922) as uint16_t,
                            (32768 - 32390) as uint16_t,
                            (32768 - 32714) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21449) as uint16_t,
                            (32768 - 31504) as uint16_t,
                            (32768 - 32536) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16392) as uint16_t,
                            (32768 - 29729) as uint16_t,
                            (32768 - 31832) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11692) as uint16_t,
                            (32768 - 24884) as uint16_t,
                            (32768 - 29076) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24193) as uint16_t,
                            (32768 - 32290) as uint16_t,
                            (32768 - 32735) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18909) as uint16_t,
                            (32768 - 31104) as uint16_t,
                            (32768 - 32563) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12236) as uint16_t,
                            (32768 - 26841) as uint16_t,
                            (32768 - 31403) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8171) as uint16_t,
                            (32768 - 21840) as uint16_t,
                            (32768 - 29082) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7224) as uint16_t,
                            (32768 - 17280) as uint16_t,
                            (32768 - 25275) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 3078) as uint16_t,
                            (32768 - 6839) as uint16_t,
                            (32768 - 9890) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13837) as uint16_t,
                            (32768 - 20450) as uint16_t,
                            (32768 - 24479) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5914) as uint16_t,
                            (32768 - 14222) as uint16_t,
                            (32768 - 19328) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3866) as uint16_t,
                            (32768 - 10267) as uint16_t,
                            (32768 - 14762) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2612) as uint16_t,
                            (32768 - 7208) as uint16_t,
                            (32768 - 11042) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1067) as uint16_t,
                            (32768 - 2991) as uint16_t,
                            (32768 - 4776) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 25817) as uint16_t,
                            (32768 - 31646) as uint16_t,
                            (32768 - 32529) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13708) as uint16_t,
                            (32768 - 26338) as uint16_t,
                            (32768 - 30385) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7328) as uint16_t,
                            (32768 - 18585) as uint16_t,
                            (32768 - 24870) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4691) as uint16_t,
                            (32768 - 13080) as uint16_t,
                            (32768 - 19276) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1825) as uint16_t,
                            (32768 - 5253) as uint16_t,
                            (32768 - 8352) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29386) as uint16_t,
                            (32768 - 32315) as uint16_t,
                            (32768 - 32624) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17160) as uint16_t,
                            (32768 - 29001) as uint16_t,
                            (32768 - 31360) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9602) as uint16_t,
                            (32768 - 21862) as uint16_t,
                            (32768 - 27396) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5915) as uint16_t,
                            (32768 - 15772) as uint16_t,
                            (32768 - 22148) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2786) as uint16_t,
                            (32768 - 7779) as uint16_t,
                            (32768 - 12047) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29246) as uint16_t,
                            (32768 - 32450) as uint16_t,
                            (32768 - 32663) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18696) as uint16_t,
                            (32768 - 29929) as uint16_t,
                            (32768 - 31818) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10510) as uint16_t,
                            (32768 - 23369) as uint16_t,
                            (32768 - 28560) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6229) as uint16_t,
                            (32768 - 16499) as uint16_t,
                            (32768 - 23125) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2608) as uint16_t,
                            (32768 - 7448) as uint16_t,
                            (32768 - 11705) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 30753) as uint16_t,
                            (32768 - 32710) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21638) as uint16_t,
                            (32768 - 31487) as uint16_t,
                            (32768 - 32503) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12937) as uint16_t,
                            (32768 - 26854) as uint16_t,
                            (32768 - 30870) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8182) as uint16_t,
                            (32768 - 20596) as uint16_t,
                            (32768 - 26970) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3637) as uint16_t,
                            (32768 - 10269) as uint16_t,
                            (32768 - 15497) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 5244) as uint16_t,
                            (32768 - 12150) as uint16_t,
                            (32768 - 16906) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20486) as uint16_t,
                            (32768 - 26858) as uint16_t,
                            (32768 - 29701) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7756) as uint16_t,
                            (32768 - 18317) as uint16_t,
                            (32768 - 23735) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3452) as uint16_t,
                            (32768 - 9256) as uint16_t,
                            (32768 - 13146) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2020) as uint16_t,
                            (32768 - 5206) as uint16_t,
                            (32768 - 8229) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1801) as uint16_t,
                            (32768 - 4993) as uint16_t,
                            (32768 - 7903) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27051) as uint16_t,
                            (32768 - 31858) as uint16_t,
                            (32768 - 32531) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15988) as uint16_t,
                            (32768 - 27531) as uint16_t,
                            (32768 - 30619) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9188) as uint16_t,
                            (32768 - 21484) as uint16_t,
                            (32768 - 26719) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6273) as uint16_t,
                            (32768 - 17186) as uint16_t,
                            (32768 - 23800) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3108) as uint16_t,
                            (32768 - 9355) as uint16_t,
                            (32768 - 14764) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 31076) as uint16_t,
                            (32768 - 32520) as uint16_t,
                            (32768 - 32680) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18119) as uint16_t,
                            (32768 - 30037) as uint16_t,
                            (32768 - 31850) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10244) as uint16_t,
                            (32768 - 22969) as uint16_t,
                            (32768 - 27472) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4692) as uint16_t,
                            (32768 - 14077) as uint16_t,
                            (32768 - 19273) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3694) as uint16_t,
                            (32768 - 11677) as uint16_t,
                            (32768 - 17556) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 30060) as uint16_t,
                            (32768 - 32581) as uint16_t,
                            (32768 - 32720) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21011) as uint16_t,
                            (32768 - 30775) as uint16_t,
                            (32768 - 32120) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11931) as uint16_t,
                            (32768 - 24820) as uint16_t,
                            (32768 - 29289) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7119) as uint16_t,
                            (32768 - 17662) as uint16_t,
                            (32768 - 24356) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3833) as uint16_t,
                            (32768 - 10706) as uint16_t,
                            (32768 - 16304) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 31954) as uint16_t,
                            (32768 - 32731) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23913) as uint16_t,
                            (32768 - 31724) as uint16_t,
                            (32768 - 32489) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15520) as uint16_t,
                            (32768 - 28060) as uint16_t,
                            (32768 - 31286) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11517) as uint16_t,
                            (32768 - 23008) as uint16_t,
                            (32768 - 28571) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6193) as uint16_t,
                            (32768 - 14508) as uint16_t,
                            (32768 - 20629) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 1035) as uint16_t,
                            (32768 - 2807) as uint16_t,
                            (32768 - 4156) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13162) as uint16_t,
                            (32768 - 18138) as uint16_t,
                            (32768 - 20939) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2696) as uint16_t,
                            (32768 - 6633) as uint16_t,
                            (32768 - 8755) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1373) as uint16_t,
                            (32768 - 4161) as uint16_t,
                            (32768 - 6853) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1099) as uint16_t,
                            (32768 - 2746) as uint16_t,
                            (32768 - 4716) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 340) as uint16_t,
                            (32768 - 1021) as uint16_t,
                            (32768 - 1599) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22826) as uint16_t,
                            (32768 - 30419) as uint16_t,
                            (32768 - 32135) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10395) as uint16_t,
                            (32768 - 21762) as uint16_t,
                            (32768 - 26942) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4726) as uint16_t,
                            (32768 - 12407) as uint16_t,
                            (32768 - 17361) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2447) as uint16_t,
                            (32768 - 7080) as uint16_t,
                            (32768 - 10593) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1227) as uint16_t,
                            (32768 - 3717) as uint16_t,
                            (32768 - 6011) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28156) as uint16_t,
                            (32768 - 31424) as uint16_t,
                            (32768 - 31934) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16915) as uint16_t,
                            (32768 - 27754) as uint16_t,
                            (32768 - 30373) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9148) as uint16_t,
                            (32768 - 20990) as uint16_t,
                            (32768 - 26431) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5950) as uint16_t,
                            (32768 - 15515) as uint16_t,
                            (32768 - 21148) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2492) as uint16_t,
                            (32768 - 7327) as uint16_t,
                            (32768 - 11526) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 30602) as uint16_t,
                            (32768 - 32477) as uint16_t,
                            (32768 - 32670) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20026) as uint16_t,
                            (32768 - 29955) as uint16_t,
                            (32768 - 31568) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11220) as uint16_t,
                            (32768 - 23628) as uint16_t,
                            (32768 - 28105) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6652) as uint16_t,
                            (32768 - 17019) as uint16_t,
                            (32768 - 22973) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3064) as uint16_t,
                            (32768 - 8536) as uint16_t,
                            (32768 - 13043) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 31769) as uint16_t,
                            (32768 - 32724) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22230) as uint16_t,
                            (32768 - 30887) as uint16_t,
                            (32768 - 32373) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12234) as uint16_t,
                            (32768 - 25079) as uint16_t,
                            (32768 - 29731) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7326) as uint16_t,
                            (32768 - 18816) as uint16_t,
                            (32768 - 25353) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3933) as uint16_t,
                            (32768 - 10907) as uint16_t,
                            (32768 - 16616) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                    ],
                ],
            ].into(),
            br_tok: [
                [
                    [
                        [
                            (32768 - 14995) as uint16_t,
                            (32768 - 21341) as uint16_t,
                            (32768 - 24749) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13158) as uint16_t,
                            (32768 - 20289) as uint16_t,
                            (32768 - 24601) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8941) as uint16_t,
                            (32768 - 15326) as uint16_t,
                            (32768 - 19876) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6297) as uint16_t,
                            (32768 - 11541) as uint16_t,
                            (32768 - 15807) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4817) as uint16_t,
                            (32768 - 9029) as uint16_t,
                            (32768 - 12776) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3731) as uint16_t,
                            (32768 - 7273) as uint16_t,
                            (32768 - 10627) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1847) as uint16_t,
                            (32768 - 3617) as uint16_t,
                            (32768 - 5354) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14472) as uint16_t,
                            (32768 - 19659) as uint16_t,
                            (32768 - 22343) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16806) as uint16_t,
                            (32768 - 24162) as uint16_t,
                            (32768 - 27533) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12900) as uint16_t,
                            (32768 - 20404) as uint16_t,
                            (32768 - 24713) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9411) as uint16_t,
                            (32768 - 16112) as uint16_t,
                            (32768 - 20797) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7056) as uint16_t,
                            (32768 - 12697) as uint16_t,
                            (32768 - 17148) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5544) as uint16_t,
                            (32768 - 10339) as uint16_t,
                            (32768 - 14460) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2954) as uint16_t,
                            (32768 - 5704) as uint16_t,
                            (32768 - 8319) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12464) as uint16_t,
                            (32768 - 18071) as uint16_t,
                            (32768 - 21354) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15482) as uint16_t,
                            (32768 - 22528) as uint16_t,
                            (32768 - 26034) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12070) as uint16_t,
                            (32768 - 19269) as uint16_t,
                            (32768 - 23624) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8953) as uint16_t,
                            (32768 - 15406) as uint16_t,
                            (32768 - 20106) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7027) as uint16_t,
                            (32768 - 12730) as uint16_t,
                            (32768 - 17220) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5887) as uint16_t,
                            (32768 - 10913) as uint16_t,
                            (32768 - 15140) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3793) as uint16_t,
                            (32768 - 7278) as uint16_t,
                            (32768 - 10447) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 15571) as uint16_t,
                            (32768 - 22232) as uint16_t,
                            (32768 - 25749) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14506) as uint16_t,
                            (32768 - 21575) as uint16_t,
                            (32768 - 25374) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10189) as uint16_t,
                            (32768 - 17089) as uint16_t,
                            (32768 - 21569) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7316) as uint16_t,
                            (32768 - 13301) as uint16_t,
                            (32768 - 17915) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5783) as uint16_t,
                            (32768 - 10912) as uint16_t,
                            (32768 - 15190) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4760) as uint16_t,
                            (32768 - 9155) as uint16_t,
                            (32768 - 13088) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2993) as uint16_t,
                            (32768 - 5966) as uint16_t,
                            (32768 - 8774) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23424) as uint16_t,
                            (32768 - 28903) as uint16_t,
                            (32768 - 30778) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20775) as uint16_t,
                            (32768 - 27666) as uint16_t,
                            (32768 - 30290) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16474) as uint16_t,
                            (32768 - 24410) as uint16_t,
                            (32768 - 28299) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12471) as uint16_t,
                            (32768 - 20180) as uint16_t,
                            (32768 - 24987) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9410) as uint16_t,
                            (32768 - 16487) as uint16_t,
                            (32768 - 21439) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7536) as uint16_t,
                            (32768 - 13614) as uint16_t,
                            (32768 - 18529) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5048) as uint16_t,
                            (32768 - 9586) as uint16_t,
                            (32768 - 13549) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21090) as uint16_t,
                            (32768 - 27290) as uint16_t,
                            (32768 - 29756) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20796) as uint16_t,
                            (32768 - 27402) as uint16_t,
                            (32768 - 30026) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17819) as uint16_t,
                            (32768 - 25485) as uint16_t,
                            (32768 - 28969) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13860) as uint16_t,
                            (32768 - 21909) as uint16_t,
                            (32768 - 26462) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11002) as uint16_t,
                            (32768 - 18494) as uint16_t,
                            (32768 - 23529) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8953) as uint16_t,
                            (32768 - 15929) as uint16_t,
                            (32768 - 20897) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6448) as uint16_t,
                            (32768 - 11918) as uint16_t,
                            (32768 - 16454) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 15999) as uint16_t,
                            (32768 - 22208) as uint16_t,
                            (32768 - 25449) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13050) as uint16_t,
                            (32768 - 19988) as uint16_t,
                            (32768 - 24122) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8594) as uint16_t,
                            (32768 - 14864) as uint16_t,
                            (32768 - 19378) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6033) as uint16_t,
                            (32768 - 11079) as uint16_t,
                            (32768 - 15238) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4554) as uint16_t,
                            (32768 - 8683) as uint16_t,
                            (32768 - 12347) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3672) as uint16_t,
                            (32768 - 7139) as uint16_t,
                            (32768 - 10337) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1900) as uint16_t,
                            (32768 - 3771) as uint16_t,
                            (32768 - 5576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15788) as uint16_t,
                            (32768 - 21340) as uint16_t,
                            (32768 - 23949) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16825) as uint16_t,
                            (32768 - 24235) as uint16_t,
                            (32768 - 27758) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12873) as uint16_t,
                            (32768 - 20402) as uint16_t,
                            (32768 - 24810) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9590) as uint16_t,
                            (32768 - 16363) as uint16_t,
                            (32768 - 21094) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7352) as uint16_t,
                            (32768 - 13209) as uint16_t,
                            (32768 - 17733) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5960) as uint16_t,
                            (32768 - 10989) as uint16_t,
                            (32768 - 15184) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3232) as uint16_t,
                            (32768 - 6234) as uint16_t,
                            (32768 - 9007) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15761) as uint16_t,
                            (32768 - 20716) as uint16_t,
                            (32768 - 23224) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19318) as uint16_t,
                            (32768 - 25989) as uint16_t,
                            (32768 - 28759) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15529) as uint16_t,
                            (32768 - 23094) as uint16_t,
                            (32768 - 26929) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11662) as uint16_t,
                            (32768 - 18989) as uint16_t,
                            (32768 - 23641) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8955) as uint16_t,
                            (32768 - 15568) as uint16_t,
                            (32768 - 20366) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7281) as uint16_t,
                            (32768 - 13106) as uint16_t,
                            (32768 - 17708) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4248) as uint16_t,
                            (32768 - 8059) as uint16_t,
                            (32768 - 11440) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 14899) as uint16_t,
                            (32768 - 21217) as uint16_t,
                            (32768 - 24503) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13519) as uint16_t,
                            (32768 - 20283) as uint16_t,
                            (32768 - 24047) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9429) as uint16_t,
                            (32768 - 15966) as uint16_t,
                            (32768 - 20365) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6700) as uint16_t,
                            (32768 - 12355) as uint16_t,
                            (32768 - 16652) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5088) as uint16_t,
                            (32768 - 9704) as uint16_t,
                            (32768 - 13716) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4243) as uint16_t,
                            (32768 - 8154) as uint16_t,
                            (32768 - 11731) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2702) as uint16_t,
                            (32768 - 5364) as uint16_t,
                            (32768 - 7861) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22745) as uint16_t,
                            (32768 - 28388) as uint16_t,
                            (32768 - 30454) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20235) as uint16_t,
                            (32768 - 27146) as uint16_t,
                            (32768 - 29922) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15896) as uint16_t,
                            (32768 - 23715) as uint16_t,
                            (32768 - 27637) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11840) as uint16_t,
                            (32768 - 19350) as uint16_t,
                            (32768 - 24131) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9122) as uint16_t,
                            (32768 - 15932) as uint16_t,
                            (32768 - 20880) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7488) as uint16_t,
                            (32768 - 13581) as uint16_t,
                            (32768 - 18362) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5114) as uint16_t,
                            (32768 - 9568) as uint16_t,
                            (32768 - 13370) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20845) as uint16_t,
                            (32768 - 26553) as uint16_t,
                            (32768 - 28932) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20981) as uint16_t,
                            (32768 - 27372) as uint16_t,
                            (32768 - 29884) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17781) as uint16_t,
                            (32768 - 25335) as uint16_t,
                            (32768 - 28785) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13760) as uint16_t,
                            (32768 - 21708) as uint16_t,
                            (32768 - 26297) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10975) as uint16_t,
                            (32768 - 18415) as uint16_t,
                            (32768 - 23365) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9045) as uint16_t,
                            (32768 - 15789) as uint16_t,
                            (32768 - 20686) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6130) as uint16_t,
                            (32768 - 11199) as uint16_t,
                            (32768 - 15423) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 13549) as uint16_t,
                            (32768 - 19724) as uint16_t,
                            (32768 - 23158) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11844) as uint16_t,
                            (32768 - 18382) as uint16_t,
                            (32768 - 22246) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7919) as uint16_t,
                            (32768 - 13619) as uint16_t,
                            (32768 - 17773) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5486) as uint16_t,
                            (32768 - 10143) as uint16_t,
                            (32768 - 13946) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4166) as uint16_t,
                            (32768 - 7983) as uint16_t,
                            (32768 - 11324) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3364) as uint16_t,
                            (32768 - 6506) as uint16_t,
                            (32768 - 9427) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1598) as uint16_t,
                            (32768 - 3160) as uint16_t,
                            (32768 - 4674) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15281) as uint16_t,
                            (32768 - 20979) as uint16_t,
                            (32768 - 23781) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14939) as uint16_t,
                            (32768 - 22119) as uint16_t,
                            (32768 - 25952) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11363) as uint16_t,
                            (32768 - 18407) as uint16_t,
                            (32768 - 22812) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8609) as uint16_t,
                            (32768 - 14857) as uint16_t,
                            (32768 - 19370) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6737) as uint16_t,
                            (32768 - 12184) as uint16_t,
                            (32768 - 16480) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5506) as uint16_t,
                            (32768 - 10263) as uint16_t,
                            (32768 - 14262) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2990) as uint16_t,
                            (32768 - 5786) as uint16_t,
                            (32768 - 8380) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20249) as uint16_t,
                            (32768 - 25253) as uint16_t,
                            (32768 - 27417) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21070) as uint16_t,
                            (32768 - 27518) as uint16_t,
                            (32768 - 30001) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16854) as uint16_t,
                            (32768 - 24469) as uint16_t,
                            (32768 - 28074) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12864) as uint16_t,
                            (32768 - 20486) as uint16_t,
                            (32768 - 25000) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9962) as uint16_t,
                            (32768 - 16978) as uint16_t,
                            (32768 - 21778) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8074) as uint16_t,
                            (32768 - 14338) as uint16_t,
                            (32768 - 19048) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4494) as uint16_t,
                            (32768 - 8479) as uint16_t,
                            (32768 - 11906) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 13960) as uint16_t,
                            (32768 - 19617) as uint16_t,
                            (32768 - 22829) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11150) as uint16_t,
                            (32768 - 17341) as uint16_t,
                            (32768 - 21228) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7150) as uint16_t,
                            (32768 - 12964) as uint16_t,
                            (32768 - 17190) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5331) as uint16_t,
                            (32768 - 10002) as uint16_t,
                            (32768 - 13867) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4167) as uint16_t,
                            (32768 - 7744) as uint16_t,
                            (32768 - 11057) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3480) as uint16_t,
                            (32768 - 6629) as uint16_t,
                            (32768 - 9646) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1883) as uint16_t,
                            (32768 - 3784) as uint16_t,
                            (32768 - 5686) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18752) as uint16_t,
                            (32768 - 25660) as uint16_t,
                            (32768 - 28912) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16968) as uint16_t,
                            (32768 - 24586) as uint16_t,
                            (32768 - 28030) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13520) as uint16_t,
                            (32768 - 21055) as uint16_t,
                            (32768 - 25313) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10453) as uint16_t,
                            (32768 - 17626) as uint16_t,
                            (32768 - 22280) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8386) as uint16_t,
                            (32768 - 14505) as uint16_t,
                            (32768 - 19116) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6742) as uint16_t,
                            (32768 - 12595) as uint16_t,
                            (32768 - 17008) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4273) as uint16_t,
                            (32768 - 8140) as uint16_t,
                            (32768 - 11499) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22120) as uint16_t,
                            (32768 - 27827) as uint16_t,
                            (32768 - 30233) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20563) as uint16_t,
                            (32768 - 27358) as uint16_t,
                            (32768 - 29895) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17076) as uint16_t,
                            (32768 - 24644) as uint16_t,
                            (32768 - 28153) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13362) as uint16_t,
                            (32768 - 20942) as uint16_t,
                            (32768 - 25309) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10794) as uint16_t,
                            (32768 - 17965) as uint16_t,
                            (32768 - 22695) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9014) as uint16_t,
                            (32768 - 15652) as uint16_t,
                            (32768 - 20319) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5708) as uint16_t,
                            (32768 - 10512) as uint16_t,
                            (32768 - 14497) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 5705) as uint16_t,
                            (32768 - 10930) as uint16_t,
                            (32768 - 15725) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7946) as uint16_t,
                            (32768 - 12765) as uint16_t,
                            (32768 - 16115) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6801) as uint16_t,
                            (32768 - 12123) as uint16_t,
                            (32768 - 16226) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5462) as uint16_t,
                            (32768 - 10135) as uint16_t,
                            (32768 - 14200) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4189) as uint16_t,
                            (32768 - 8011) as uint16_t,
                            (32768 - 11507) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3191) as uint16_t,
                            (32768 - 6229) as uint16_t,
                            (32768 - 9408) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1057) as uint16_t,
                            (32768 - 2137) as uint16_t,
                            (32768 - 3212) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10018) as uint16_t,
                            (32768 - 17067) as uint16_t,
                            (32768 - 21491) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7380) as uint16_t,
                            (32768 - 12582) as uint16_t,
                            (32768 - 16453) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6068) as uint16_t,
                            (32768 - 10845) as uint16_t,
                            (32768 - 14339) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5098) as uint16_t,
                            (32768 - 9198) as uint16_t,
                            (32768 - 12555) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4312) as uint16_t,
                            (32768 - 8010) as uint16_t,
                            (32768 - 11119) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3700) as uint16_t,
                            (32768 - 6966) as uint16_t,
                            (32768 - 9781) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1693) as uint16_t,
                            (32768 - 3326) as uint16_t,
                            (32768 - 4887) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18757) as uint16_t,
                            (32768 - 24930) as uint16_t,
                            (32768 - 27774) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17648) as uint16_t,
                            (32768 - 24596) as uint16_t,
                            (32768 - 27817) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14707) as uint16_t,
                            (32768 - 22052) as uint16_t,
                            (32768 - 26026) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11720) as uint16_t,
                            (32768 - 18852) as uint16_t,
                            (32768 - 23292) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9357) as uint16_t,
                            (32768 - 15952) as uint16_t,
                            (32768 - 20525) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7810) as uint16_t,
                            (32768 - 13753) as uint16_t,
                            (32768 - 18210) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3879) as uint16_t,
                            (32768 - 7333) as uint16_t,
                            (32768 - 10328) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 8278) as uint16_t,
                            (32768 - 13242) as uint16_t,
                            (32768 - 15922) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10547) as uint16_t,
                            (32768 - 15867) as uint16_t,
                            (32768 - 18919) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9106) as uint16_t,
                            (32768 - 15842) as uint16_t,
                            (32768 - 20609) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6833) as uint16_t,
                            (32768 - 13007) as uint16_t,
                            (32768 - 17218) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4811) as uint16_t,
                            (32768 - 9712) as uint16_t,
                            (32768 - 13923) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3985) as uint16_t,
                            (32768 - 7352) as uint16_t,
                            (32768 - 11128) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1688) as uint16_t,
                            (32768 - 3458) as uint16_t,
                            (32768 - 5262) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12951) as uint16_t,
                            (32768 - 21861) as uint16_t,
                            (32768 - 26510) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9788) as uint16_t,
                            (32768 - 16044) as uint16_t,
                            (32768 - 20276) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6309) as uint16_t,
                            (32768 - 11244) as uint16_t,
                            (32768 - 14870) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5183) as uint16_t,
                            (32768 - 9349) as uint16_t,
                            (32768 - 12566) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4389) as uint16_t,
                            (32768 - 8229) as uint16_t,
                            (32768 - 11492) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3633) as uint16_t,
                            (32768 - 6945) as uint16_t,
                            (32768 - 10620) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3600) as uint16_t,
                            (32768 - 6847) as uint16_t,
                            (32768 - 9907) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21748) as uint16_t,
                            (32768 - 28137) as uint16_t,
                            (32768 - 30255) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19436) as uint16_t,
                            (32768 - 26581) as uint16_t,
                            (32768 - 29560) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16359) as uint16_t,
                            (32768 - 24201) as uint16_t,
                            (32768 - 27953) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13961) as uint16_t,
                            (32768 - 21693) as uint16_t,
                            (32768 - 25871) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11544) as uint16_t,
                            (32768 - 18686) as uint16_t,
                            (32768 - 23322) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9372) as uint16_t,
                            (32768 - 16462) as uint16_t,
                            (32768 - 20952) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6138) as uint16_t,
                            (32768 - 11210) as uint16_t,
                            (32768 - 15390) as uint16_t,
                            0,
                        ],
                    ],
                ],
            ].into(),
            eob_hi_bit: [
                [
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 17471) as uint16_t, 0],
                        [(32768 - 20223) as uint16_t, 0],
                        [(32768 - 11357) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 20335) as uint16_t, 0],
                        [(32768 - 21667) as uint16_t, 0],
                        [(32768 - 14818) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                ],
                [
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 20430) as uint16_t, 0],
                        [(32768 - 20662) as uint16_t, 0],
                        [(32768 - 15367) as uint16_t, 0],
                        [(32768 - 16970) as uint16_t, 0],
                        [(32768 - 14657) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 22117) as uint16_t, 0],
                        [(32768 - 22028) as uint16_t, 0],
                        [(32768 - 18650) as uint16_t, 0],
                        [(32768 - 16042) as uint16_t, 0],
                        [(32768 - 15885) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                ],
                [
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 22409) as uint16_t, 0],
                        [(32768 - 21012) as uint16_t, 0],
                        [(32768 - 15650) as uint16_t, 0],
                        [(32768 - 17395) as uint16_t, 0],
                        [(32768 - 15469) as uint16_t, 0],
                        [(32768 - 20205) as uint16_t, 0],
                        [(32768 - 19511) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 24220) as uint16_t, 0],
                        [(32768 - 22480) as uint16_t, 0],
                        [(32768 - 17737) as uint16_t, 0],
                        [(32768 - 18916) as uint16_t, 0],
                        [(32768 - 19268) as uint16_t, 0],
                        [(32768 - 18412) as uint16_t, 0],
                        [(32768 - 18844) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                ],
                [
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 25991) as uint16_t, 0],
                        [(32768 - 20314) as uint16_t, 0],
                        [(32768 - 17731) as uint16_t, 0],
                        [(32768 - 19678) as uint16_t, 0],
                        [(32768 - 18649) as uint16_t, 0],
                        [(32768 - 17307) as uint16_t, 0],
                        [(32768 - 21798) as uint16_t, 0],
                        [(32768 - 17549) as uint16_t, 0],
                        [(32768 - 15630) as uint16_t, 0],
                    ],
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 26585) as uint16_t, 0],
                        [(32768 - 21469) as uint16_t, 0],
                        [(32768 - 20432) as uint16_t, 0],
                        [(32768 - 17735) as uint16_t, 0],
                        [(32768 - 19280) as uint16_t, 0],
                        [(32768 - 15235) as uint16_t, 0],
                        [(32768 - 20297) as uint16_t, 0],
                        [(32768 - 22471) as uint16_t, 0],
                        [(32768 - 28997) as uint16_t, 0],
                    ],
                ],
                [
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 26605) as uint16_t, 0],
                        [(32768 - 11304) as uint16_t, 0],
                        [(32768 - 16726) as uint16_t, 0],
                        [(32768 - 16560) as uint16_t, 0],
                        [(32768 - 20866) as uint16_t, 0],
                        [(32768 - 23524) as uint16_t, 0],
                        [(32768 - 19878) as uint16_t, 0],
                        [(32768 - 13469) as uint16_t, 0],
                        [(32768 - 23084) as uint16_t, 0],
                    ],
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                ],
            ].into(),
            skip: [
                [
                    [(32768 - 30371) as uint16_t, 0],
                    [(32768 - 7570) as uint16_t, 0],
                    [(32768 - 13155) as uint16_t, 0],
                    [(32768 - 20751) as uint16_t, 0],
                    [(32768 - 20969) as uint16_t, 0],
                    [(32768 - 27067) as uint16_t, 0],
                    [(32768 - 32013) as uint16_t, 0],
                    [(32768 - 5495) as uint16_t, 0],
                    [(32768 - 17942) as uint16_t, 0],
                    [(32768 - 28280) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                ],
                [
                    [(32768 - 31782) as uint16_t, 0],
                    [(32768 - 1836) as uint16_t, 0],
                    [(32768 - 10689) as uint16_t, 0],
                    [(32768 - 17604) as uint16_t, 0],
                    [(32768 - 21622) as uint16_t, 0],
                    [(32768 - 27518) as uint16_t, 0],
                    [(32768 - 32399) as uint16_t, 0],
                    [(32768 - 4419) as uint16_t, 0],
                    [(32768 - 16294) as uint16_t, 0],
                    [(32768 - 28345) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                ],
                [
                    [(32768 - 31901) as uint16_t, 0],
                    [(32768 - 10311) as uint16_t, 0],
                    [(32768 - 18047) as uint16_t, 0],
                    [(32768 - 24806) as uint16_t, 0],
                    [(32768 - 23288) as uint16_t, 0],
                    [(32768 - 27914) as uint16_t, 0],
                    [(32768 - 32296) as uint16_t, 0],
                    [(32768 - 4215) as uint16_t, 0],
                    [(32768 - 15756) as uint16_t, 0],
                    [(32768 - 28341) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                ],
                [
                    [(32768 - 26726) as uint16_t, 0],
                    [(32768 - 1045) as uint16_t, 0],
                    [(32768 - 11703) as uint16_t, 0],
                    [(32768 - 20590) as uint16_t, 0],
                    [(32768 - 18554) as uint16_t, 0],
                    [(32768 - 25970) as uint16_t, 0],
                    [(32768 - 31938) as uint16_t, 0],
                    [(32768 - 5583) as uint16_t, 0],
                    [(32768 - 21313) as uint16_t, 0],
                    [(32768 - 29390) as uint16_t, 0],
                    [(32768 - 641) as uint16_t, 0],
                    [(32768 - 22265) as uint16_t, 0],
                    [(32768 - 31452) as uint16_t, 0],
                ],
                [
                    [(32768 - 26584) as uint16_t, 0],
                    [(32768 - 188) as uint16_t, 0],
                    [(32768 - 8847) as uint16_t, 0],
                    [(32768 - 24519) as uint16_t, 0],
                    [(32768 - 22938) as uint16_t, 0],
                    [(32768 - 30583) as uint16_t, 0],
                    [(32768 - 32608) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                ],
            ].into(),
            dc_sign: [
                [
                    [(32768 - 16000) as uint16_t, 0],
                    [(32768 - 13056) as uint16_t, 0],
                    [(32768 - 18816) as uint16_t, 0],
                ],
                [
                    [(32768 - 15232) as uint16_t, 0],
                    [(32768 - 12928) as uint16_t, 0],
                    [(32768 - 17280) as uint16_t, 0],
                ],
            ].into(),
        };
        init
    },
    {
        let mut init = CdfCoefContext {
            eob_bin_16: [
                [
                    [
                        (32768 - 4016) as uint16_t,
                        (32768 - 4897) as uint16_t,
                        (32768 - 8881) as uint16_t,
                        (32768 - 14968) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 716) as uint16_t,
                        (32768 - 1105) as uint16_t,
                        (32768 - 2646) as uint16_t,
                        (32768 - 10056) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 11139) as uint16_t,
                        (32768 - 13270) as uint16_t,
                        (32768 - 18241) as uint16_t,
                        (32768 - 23566) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 3192) as uint16_t,
                        (32768 - 5032) as uint16_t,
                        (32768 - 10297) as uint16_t,
                        (32768 - 19755) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                ],
            ].into(),
            eob_bin_32: [
                [
                    [
                        (32768 - 2515) as uint16_t,
                        (32768 - 3003) as uint16_t,
                        (32768 - 4452) as uint16_t,
                        (32768 - 8162) as uint16_t,
                        (32768 - 16041) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 574) as uint16_t,
                        (32768 - 821) as uint16_t,
                        (32768 - 1836) as uint16_t,
                        (32768 - 5089) as uint16_t,
                        (32768 - 13128) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 13468) as uint16_t,
                        (32768 - 16303) as uint16_t,
                        (32768 - 20361) as uint16_t,
                        (32768 - 25105) as uint16_t,
                        (32768 - 29281) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 3542) as uint16_t,
                        (32768 - 5502) as uint16_t,
                        (32768 - 10415) as uint16_t,
                        (32768 - 16760) as uint16_t,
                        (32768 - 25644) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                ],
            ].into(),
            eob_bin_64: [
                [
                    [
                        (32768 - 2374) as uint16_t,
                        (32768 - 2772) as uint16_t,
                        (32768 - 4583) as uint16_t,
                        (32768 - 7276) as uint16_t,
                        (32768 - 12288) as uint16_t,
                        (32768 - 19706) as uint16_t,
                        0,
                        0,
                    ],
                    [
                        (32768 - 497) as uint16_t,
                        (32768 - 810) as uint16_t,
                        (32768 - 1315) as uint16_t,
                        (32768 - 3000) as uint16_t,
                        (32768 - 7004) as uint16_t,
                        (32768 - 15641) as uint16_t,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 15050) as uint16_t,
                        (32768 - 17126) as uint16_t,
                        (32768 - 21410) as uint16_t,
                        (32768 - 24886) as uint16_t,
                        (32768 - 28156) as uint16_t,
                        (32768 - 30726) as uint16_t,
                        0,
                        0,
                    ],
                    [
                        (32768 - 4034) as uint16_t,
                        (32768 - 6290) as uint16_t,
                        (32768 - 10235) as uint16_t,
                        (32768 - 14982) as uint16_t,
                        (32768 - 21214) as uint16_t,
                        (32768 - 28491) as uint16_t,
                        0,
                        0,
                    ],
                ],
            ].into(),
            eob_bin_128: [
                [
                    [
                        (32768 - 1366) as uint16_t,
                        (32768 - 1738) as uint16_t,
                        (32768 - 2527) as uint16_t,
                        (32768 - 5016) as uint16_t,
                        (32768 - 9355) as uint16_t,
                        (32768 - 15797) as uint16_t,
                        (32768 - 24643) as uint16_t,
                        0,
                    ],
                    [
                        (32768 - 354) as uint16_t,
                        (32768 - 558) as uint16_t,
                        (32768 - 944) as uint16_t,
                        (32768 - 2760) as uint16_t,
                        (32768 - 7287) as uint16_t,
                        (32768 - 14037) as uint16_t,
                        (32768 - 21779) as uint16_t,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 13627) as uint16_t,
                        (32768 - 16246) as uint16_t,
                        (32768 - 20173) as uint16_t,
                        (32768 - 24429) as uint16_t,
                        (32768 - 27948) as uint16_t,
                        (32768 - 30415) as uint16_t,
                        (32768 - 31863) as uint16_t,
                        0,
                    ],
                    [
                        (32768 - 6275) as uint16_t,
                        (32768 - 9889) as uint16_t,
                        (32768 - 14769) as uint16_t,
                        (32768 - 23164) as uint16_t,
                        (32768 - 27988) as uint16_t,
                        (32768 - 30493) as uint16_t,
                        (32768 - 32272) as uint16_t,
                        0,
                    ],
                ],
            ].into(),
            eob_bin_256: [
                [
                    [
                        (32768 - 3089) as uint16_t,
                        (32768 - 3920) as uint16_t,
                        (32768 - 6038) as uint16_t,
                        (32768 - 9460) as uint16_t,
                        (32768 - 14266) as uint16_t,
                        (32768 - 19881) as uint16_t,
                        (32768 - 25766) as uint16_t,
                        (32768 - 29176) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 1084) as uint16_t,
                        (32768 - 2358) as uint16_t,
                        (32768 - 3488) as uint16_t,
                        (32768 - 5122) as uint16_t,
                        (32768 - 11483) as uint16_t,
                        (32768 - 18103) as uint16_t,
                        (32768 - 26023) as uint16_t,
                        (32768 - 29799) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 11514) as uint16_t,
                        (32768 - 13794) as uint16_t,
                        (32768 - 17480) as uint16_t,
                        (32768 - 20754) as uint16_t,
                        (32768 - 24361) as uint16_t,
                        (32768 - 27378) as uint16_t,
                        (32768 - 29492) as uint16_t,
                        (32768 - 31277) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 6571) as uint16_t,
                        (32768 - 9610) as uint16_t,
                        (32768 - 15516) as uint16_t,
                        (32768 - 21826) as uint16_t,
                        (32768 - 29092) as uint16_t,
                        (32768 - 30829) as uint16_t,
                        (32768 - 31842) as uint16_t,
                        (32768 - 32708) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                ],
            ].into(),
            eob_bin_512: [
                [
                    (32768 - 2624) as uint16_t,
                    (32768 - 3936) as uint16_t,
                    (32768 - 6480) as uint16_t,
                    (32768 - 9686) as uint16_t,
                    (32768 - 13979) as uint16_t,
                    (32768 - 17726) as uint16_t,
                    (32768 - 23267) as uint16_t,
                    (32768 - 28410) as uint16_t,
                    (32768 - 31078) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 12015) as uint16_t,
                    (32768 - 14769) as uint16_t,
                    (32768 - 19588) as uint16_t,
                    (32768 - 22052) as uint16_t,
                    (32768 - 24222) as uint16_t,
                    (32768 - 25812) as uint16_t,
                    (32768 - 27300) as uint16_t,
                    (32768 - 29219) as uint16_t,
                    (32768 - 32114) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            ].into(),
            eob_bin_1024: [
                [
                    (32768 - 2784) as uint16_t,
                    (32768 - 3831) as uint16_t,
                    (32768 - 7041) as uint16_t,
                    (32768 - 10521) as uint16_t,
                    (32768 - 14847) as uint16_t,
                    (32768 - 18844) as uint16_t,
                    (32768 - 23155) as uint16_t,
                    (32768 - 26682) as uint16_t,
                    (32768 - 29229) as uint16_t,
                    (32768 - 31045) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 9577) as uint16_t,
                    (32768 - 12466) as uint16_t,
                    (32768 - 17739) as uint16_t,
                    (32768 - 20750) as uint16_t,
                    (32768 - 22061) as uint16_t,
                    (32768 - 23215) as uint16_t,
                    (32768 - 24601) as uint16_t,
                    (32768 - 25483) as uint16_t,
                    (32768 - 25843) as uint16_t,
                    (32768 - 32056) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            ].into(),
            eob_base_tok: [
                [
                    [
                        [
                            (32768 - 20092) as uint16_t,
                            (32768 - 30774) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 30695) as uint16_t,
                            (32768 - 32020) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31131) as uint16_t,
                            (32768 - 32103) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 28666) as uint16_t,
                            (32768 - 30870) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 27258) as uint16_t,
                            (32768 - 31095) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31804) as uint16_t,
                            (32768 - 32623) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31763) as uint16_t,
                            (32768 - 32528) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31438) as uint16_t,
                            (32768 - 32506) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 18049) as uint16_t,
                            (32768 - 30489) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31706) as uint16_t,
                            (32768 - 32286) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32163) as uint16_t,
                            (32768 - 32473) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31550) as uint16_t,
                            (32768 - 32184) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 27116) as uint16_t,
                            (32768 - 30842) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31971) as uint16_t,
                            (32768 - 32598) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32088) as uint16_t,
                            (32768 - 32576) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32067) as uint16_t,
                            (32768 - 32664) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 12854) as uint16_t,
                            (32768 - 29093) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32272) as uint16_t,
                            (32768 - 32558) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32667) as uint16_t,
                            (32768 - 32729) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32306) as uint16_t,
                            (32768 - 32585) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 25476) as uint16_t,
                            (32768 - 30366) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32169) as uint16_t,
                            (32768 - 32687) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32479) as uint16_t,
                            (32768 - 32689) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31673) as uint16_t,
                            (32768 - 32634) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 2809) as uint16_t,
                            (32768 - 19301) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32205) as uint16_t,
                            (32768 - 32622) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32338) as uint16_t,
                            (32768 - 32730) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31786) as uint16_t,
                            (32768 - 32616) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 22737) as uint16_t,
                            (32768 - 29105) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 30810) as uint16_t,
                            (32768 - 32362) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 30014) as uint16_t,
                            (32768 - 32627) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 30528) as uint16_t,
                            (32768 - 32574) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 935) as uint16_t,
                            (32768 - 3382) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 30789) as uint16_t,
                            (32768 - 31909) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32466) as uint16_t,
                            (32768 - 32756) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 30860) as uint16_t,
                            (32768 - 32513) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 10923) as uint16_t,
                            (32768 - 21845) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 10923) as uint16_t,
                            (32768 - 21845) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 10923) as uint16_t,
                            (32768 - 21845) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 10923) as uint16_t,
                            (32768 - 21845) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                ],
            ].into(),
            base_tok: [
                [
                    [
                        [
                            (32768 - 8896) as uint16_t,
                            (32768 - 16227) as uint16_t,
                            (32768 - 20630) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23629) as uint16_t,
                            (32768 - 31782) as uint16_t,
                            (32768 - 32527) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15173) as uint16_t,
                            (32768 - 27755) as uint16_t,
                            (32768 - 31321) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10158) as uint16_t,
                            (32768 - 21233) as uint16_t,
                            (32768 - 27382) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6420) as uint16_t,
                            (32768 - 14857) as uint16_t,
                            (32768 - 21558) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3269) as uint16_t,
                            (32768 - 8155) as uint16_t,
                            (32768 - 12646) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24835) as uint16_t,
                            (32768 - 32009) as uint16_t,
                            (32768 - 32496) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16509) as uint16_t,
                            (32768 - 28421) as uint16_t,
                            (32768 - 31579) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10957) as uint16_t,
                            (32768 - 21514) as uint16_t,
                            (32768 - 27418) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7881) as uint16_t,
                            (32768 - 15930) as uint16_t,
                            (32768 - 22096) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5388) as uint16_t,
                            (32768 - 10960) as uint16_t,
                            (32768 - 15918) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20745) as uint16_t,
                            (32768 - 30773) as uint16_t,
                            (32768 - 32093) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15200) as uint16_t,
                            (32768 - 27221) as uint16_t,
                            (32768 - 30861) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13032) as uint16_t,
                            (32768 - 20873) as uint16_t,
                            (32768 - 25667) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12285) as uint16_t,
                            (32768 - 18663) as uint16_t,
                            (32768 - 23494) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11563) as uint16_t,
                            (32768 - 17481) as uint16_t,
                            (32768 - 21489) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 26260) as uint16_t,
                            (32768 - 31982) as uint16_t,
                            (32768 - 32320) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15397) as uint16_t,
                            (32768 - 28083) as uint16_t,
                            (32768 - 31100) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9742) as uint16_t,
                            (32768 - 19217) as uint16_t,
                            (32768 - 24824) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3261) as uint16_t,
                            (32768 - 9629) as uint16_t,
                            (32768 - 15362) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1480) as uint16_t,
                            (32768 - 4322) as uint16_t,
                            (32768 - 7499) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27599) as uint16_t,
                            (32768 - 32256) as uint16_t,
                            (32768 - 32460) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16857) as uint16_t,
                            (32768 - 27659) as uint16_t,
                            (32768 - 30774) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9551) as uint16_t,
                            (32768 - 18290) as uint16_t,
                            (32768 - 23748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3052) as uint16_t,
                            (32768 - 8933) as uint16_t,
                            (32768 - 14103) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2021) as uint16_t,
                            (32768 - 5910) as uint16_t,
                            (32768 - 9787) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29005) as uint16_t,
                            (32768 - 32015) as uint16_t,
                            (32768 - 32392) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17677) as uint16_t,
                            (32768 - 27694) as uint16_t,
                            (32768 - 30863) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9204) as uint16_t,
                            (32768 - 17356) as uint16_t,
                            (32768 - 23219) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2403) as uint16_t,
                            (32768 - 7516) as uint16_t,
                            (32768 - 12814) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 10808) as uint16_t,
                            (32768 - 22056) as uint16_t,
                            (32768 - 26896) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 25739) as uint16_t,
                            (32768 - 32313) as uint16_t,
                            (32768 - 32676) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17288) as uint16_t,
                            (32768 - 30203) as uint16_t,
                            (32768 - 32221) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11359) as uint16_t,
                            (32768 - 24878) as uint16_t,
                            (32768 - 29896) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6949) as uint16_t,
                            (32768 - 17767) as uint16_t,
                            (32768 - 24893) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4287) as uint16_t,
                            (32768 - 11796) as uint16_t,
                            (32768 - 18071) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27880) as uint16_t,
                            (32768 - 32521) as uint16_t,
                            (32768 - 32705) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19038) as uint16_t,
                            (32768 - 31004) as uint16_t,
                            (32768 - 32414) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12564) as uint16_t,
                            (32768 - 26345) as uint16_t,
                            (32768 - 30768) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8269) as uint16_t,
                            (32768 - 19947) as uint16_t,
                            (32768 - 26779) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5674) as uint16_t,
                            (32768 - 14657) as uint16_t,
                            (32768 - 21674) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 25742) as uint16_t,
                            (32768 - 32319) as uint16_t,
                            (32768 - 32671) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19557) as uint16_t,
                            (32768 - 31164) as uint16_t,
                            (32768 - 32454) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13381) as uint16_t,
                            (32768 - 26381) as uint16_t,
                            (32768 - 30755) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10101) as uint16_t,
                            (32768 - 21466) as uint16_t,
                            (32768 - 26722) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9209) as uint16_t,
                            (32768 - 19650) as uint16_t,
                            (32768 - 26825) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27107) as uint16_t,
                            (32768 - 31917) as uint16_t,
                            (32768 - 32432) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18056) as uint16_t,
                            (32768 - 28893) as uint16_t,
                            (32768 - 31203) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10200) as uint16_t,
                            (32768 - 21434) as uint16_t,
                            (32768 - 26764) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4660) as uint16_t,
                            (32768 - 12913) as uint16_t,
                            (32768 - 19502) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2368) as uint16_t,
                            (32768 - 6930) as uint16_t,
                            (32768 - 12504) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 26960) as uint16_t,
                            (32768 - 32158) as uint16_t,
                            (32768 - 32613) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18628) as uint16_t,
                            (32768 - 30005) as uint16_t,
                            (32768 - 32031) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10233) as uint16_t,
                            (32768 - 22442) as uint16_t,
                            (32768 - 28232) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5471) as uint16_t,
                            (32768 - 14630) as uint16_t,
                            (32768 - 21516) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3235) as uint16_t,
                            (32768 - 10767) as uint16_t,
                            (32768 - 17109) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27696) as uint16_t,
                            (32768 - 32440) as uint16_t,
                            (32768 - 32692) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20032) as uint16_t,
                            (32768 - 31167) as uint16_t,
                            (32768 - 32438) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8700) as uint16_t,
                            (32768 - 21341) as uint16_t,
                            (32768 - 28442) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5662) as uint16_t,
                            (32768 - 14831) as uint16_t,
                            (32768 - 21795) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 9704) as uint16_t,
                            (32768 - 17294) as uint16_t,
                            (32768 - 21132) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 26762) as uint16_t,
                            (32768 - 32278) as uint16_t,
                            (32768 - 32633) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18382) as uint16_t,
                            (32768 - 29620) as uint16_t,
                            (32768 - 31819) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10891) as uint16_t,
                            (32768 - 23475) as uint16_t,
                            (32768 - 28723) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6358) as uint16_t,
                            (32768 - 16583) as uint16_t,
                            (32768 - 23309) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3248) as uint16_t,
                            (32768 - 9118) as uint16_t,
                            (32768 - 14141) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27204) as uint16_t,
                            (32768 - 32573) as uint16_t,
                            (32768 - 32699) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19818) as uint16_t,
                            (32768 - 30824) as uint16_t,
                            (32768 - 32329) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11772) as uint16_t,
                            (32768 - 25120) as uint16_t,
                            (32768 - 30041) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6995) as uint16_t,
                            (32768 - 18033) as uint16_t,
                            (32768 - 25039) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3752) as uint16_t,
                            (32768 - 10442) as uint16_t,
                            (32768 - 16098) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27222) as uint16_t,
                            (32768 - 32256) as uint16_t,
                            (32768 - 32559) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15356) as uint16_t,
                            (32768 - 28399) as uint16_t,
                            (32768 - 31475) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8821) as uint16_t,
                            (32768 - 20635) as uint16_t,
                            (32768 - 27057) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5511) as uint16_t,
                            (32768 - 14404) as uint16_t,
                            (32768 - 21239) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2935) as uint16_t,
                            (32768 - 8222) as uint16_t,
                            (32768 - 13051) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24875) as uint16_t,
                            (32768 - 32120) as uint16_t,
                            (32768 - 32529) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15233) as uint16_t,
                            (32768 - 28265) as uint16_t,
                            (32768 - 31445) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8605) as uint16_t,
                            (32768 - 20570) as uint16_t,
                            (32768 - 26932) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5431) as uint16_t,
                            (32768 - 14413) as uint16_t,
                            (32768 - 21196) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2994) as uint16_t,
                            (32768 - 8341) as uint16_t,
                            (32768 - 13223) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28201) as uint16_t,
                            (32768 - 32604) as uint16_t,
                            (32768 - 32700) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21041) as uint16_t,
                            (32768 - 31446) as uint16_t,
                            (32768 - 32456) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13221) as uint16_t,
                            (32768 - 26213) as uint16_t,
                            (32768 - 30475) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8255) as uint16_t,
                            (32768 - 19385) as uint16_t,
                            (32768 - 26037) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4930) as uint16_t,
                            (32768 - 12585) as uint16_t,
                            (32768 - 18830) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28768) as uint16_t,
                            (32768 - 32448) as uint16_t,
                            (32768 - 32627) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19705) as uint16_t,
                            (32768 - 30561) as uint16_t,
                            (32768 - 32021) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11572) as uint16_t,
                            (32768 - 23589) as uint16_t,
                            (32768 - 28220) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5532) as uint16_t,
                            (32768 - 15034) as uint16_t,
                            (32768 - 21446) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2460) as uint16_t,
                            (32768 - 7150) as uint16_t,
                            (32768 - 11456) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29874) as uint16_t,
                            (32768 - 32619) as uint16_t,
                            (32768 - 32699) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21621) as uint16_t,
                            (32768 - 31071) as uint16_t,
                            (32768 - 32201) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12511) as uint16_t,
                            (32768 - 24747) as uint16_t,
                            (32768 - 28992) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6281) as uint16_t,
                            (32768 - 16395) as uint16_t,
                            (32768 - 22748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3246) as uint16_t,
                            (32768 - 9278) as uint16_t,
                            (32768 - 14497) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29715) as uint16_t,
                            (32768 - 32625) as uint16_t,
                            (32768 - 32712) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20958) as uint16_t,
                            (32768 - 31011) as uint16_t,
                            (32768 - 32283) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11233) as uint16_t,
                            (32768 - 23671) as uint16_t,
                            (32768 - 28806) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6012) as uint16_t,
                            (32768 - 16128) as uint16_t,
                            (32768 - 22868) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3427) as uint16_t,
                            (32768 - 9851) as uint16_t,
                            (32768 - 15414) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 11016) as uint16_t,
                            (32768 - 22111) as uint16_t,
                            (32768 - 26794) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 25946) as uint16_t,
                            (32768 - 32357) as uint16_t,
                            (32768 - 32677) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17890) as uint16_t,
                            (32768 - 30452) as uint16_t,
                            (32768 - 32252) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11678) as uint16_t,
                            (32768 - 25142) as uint16_t,
                            (32768 - 29816) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6720) as uint16_t,
                            (32768 - 17534) as uint16_t,
                            (32768 - 24584) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4230) as uint16_t,
                            (32768 - 11665) as uint16_t,
                            (32768 - 17820) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28400) as uint16_t,
                            (32768 - 32623) as uint16_t,
                            (32768 - 32747) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21164) as uint16_t,
                            (32768 - 31668) as uint16_t,
                            (32768 - 32575) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13572) as uint16_t,
                            (32768 - 27388) as uint16_t,
                            (32768 - 31182) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8234) as uint16_t,
                            (32768 - 20750) as uint16_t,
                            (32768 - 27358) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5065) as uint16_t,
                            (32768 - 14055) as uint16_t,
                            (32768 - 20897) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28981) as uint16_t,
                            (32768 - 32547) as uint16_t,
                            (32768 - 32705) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18681) as uint16_t,
                            (32768 - 30543) as uint16_t,
                            (32768 - 32239) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10919) as uint16_t,
                            (32768 - 24075) as uint16_t,
                            (32768 - 29286) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6431) as uint16_t,
                            (32768 - 17199) as uint16_t,
                            (32768 - 24077) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3819) as uint16_t,
                            (32768 - 10464) as uint16_t,
                            (32768 - 16618) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 26870) as uint16_t,
                            (32768 - 32467) as uint16_t,
                            (32768 - 32693) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19041) as uint16_t,
                            (32768 - 30831) as uint16_t,
                            (32768 - 32347) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11794) as uint16_t,
                            (32768 - 25211) as uint16_t,
                            (32768 - 30016) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6888) as uint16_t,
                            (32768 - 18019) as uint16_t,
                            (32768 - 24970) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4370) as uint16_t,
                            (32768 - 12363) as uint16_t,
                            (32768 - 18992) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29578) as uint16_t,
                            (32768 - 32670) as uint16_t,
                            (32768 - 32744) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23159) as uint16_t,
                            (32768 - 32007) as uint16_t,
                            (32768 - 32613) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15315) as uint16_t,
                            (32768 - 28669) as uint16_t,
                            (32768 - 31676) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9298) as uint16_t,
                            (32768 - 22607) as uint16_t,
                            (32768 - 28782) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6144) as uint16_t,
                            (32768 - 15913) as uint16_t,
                            (32768 - 22968) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28110) as uint16_t,
                            (32768 - 32499) as uint16_t,
                            (32768 - 32669) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21574) as uint16_t,
                            (32768 - 30937) as uint16_t,
                            (32768 - 32015) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12759) as uint16_t,
                            (32768 - 24818) as uint16_t,
                            (32768 - 28727) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6545) as uint16_t,
                            (32768 - 16761) as uint16_t,
                            (32768 - 23042) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3649) as uint16_t,
                            (32768 - 10597) as uint16_t,
                            (32768 - 16833) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28163) as uint16_t,
                            (32768 - 32552) as uint16_t,
                            (32768 - 32728) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22101) as uint16_t,
                            (32768 - 31469) as uint16_t,
                            (32768 - 32464) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13160) as uint16_t,
                            (32768 - 25472) as uint16_t,
                            (32768 - 30143) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7303) as uint16_t,
                            (32768 - 18684) as uint16_t,
                            (32768 - 25468) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5241) as uint16_t,
                            (32768 - 13975) as uint16_t,
                            (32768 - 20955) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28400) as uint16_t,
                            (32768 - 32631) as uint16_t,
                            (32768 - 32744) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22104) as uint16_t,
                            (32768 - 31793) as uint16_t,
                            (32768 - 32603) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13557) as uint16_t,
                            (32768 - 26571) as uint16_t,
                            (32768 - 30846) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7749) as uint16_t,
                            (32768 - 19861) as uint16_t,
                            (32768 - 26675) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4873) as uint16_t,
                            (32768 - 14030) as uint16_t,
                            (32768 - 21234) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 9800) as uint16_t,
                            (32768 - 17635) as uint16_t,
                            (32768 - 21073) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 26153) as uint16_t,
                            (32768 - 31885) as uint16_t,
                            (32768 - 32527) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15038) as uint16_t,
                            (32768 - 27852) as uint16_t,
                            (32768 - 31006) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8718) as uint16_t,
                            (32768 - 20564) as uint16_t,
                            (32768 - 26486) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5128) as uint16_t,
                            (32768 - 14076) as uint16_t,
                            (32768 - 20514) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2636) as uint16_t,
                            (32768 - 7566) as uint16_t,
                            (32768 - 11925) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27551) as uint16_t,
                            (32768 - 32504) as uint16_t,
                            (32768 - 32701) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18310) as uint16_t,
                            (32768 - 30054) as uint16_t,
                            (32768 - 32100) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10211) as uint16_t,
                            (32768 - 23420) as uint16_t,
                            (32768 - 29082) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6222) as uint16_t,
                            (32768 - 16876) as uint16_t,
                            (32768 - 23916) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3462) as uint16_t,
                            (32768 - 9954) as uint16_t,
                            (32768 - 15498) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29991) as uint16_t,
                            (32768 - 32633) as uint16_t,
                            (32768 - 32721) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19883) as uint16_t,
                            (32768 - 30751) as uint16_t,
                            (32768 - 32201) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11141) as uint16_t,
                            (32768 - 24184) as uint16_t,
                            (32768 - 29285) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6420) as uint16_t,
                            (32768 - 16940) as uint16_t,
                            (32768 - 23774) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3392) as uint16_t,
                            (32768 - 9753) as uint16_t,
                            (32768 - 15118) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28465) as uint16_t,
                            (32768 - 32616) as uint16_t,
                            (32768 - 32712) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19850) as uint16_t,
                            (32768 - 30702) as uint16_t,
                            (32768 - 32244) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10983) as uint16_t,
                            (32768 - 24024) as uint16_t,
                            (32768 - 29223) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6294) as uint16_t,
                            (32768 - 16770) as uint16_t,
                            (32768 - 23582) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3244) as uint16_t,
                            (32768 - 9283) as uint16_t,
                            (32768 - 14509) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 30023) as uint16_t,
                            (32768 - 32717) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22940) as uint16_t,
                            (32768 - 32032) as uint16_t,
                            (32768 - 32626) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14282) as uint16_t,
                            (32768 - 27928) as uint16_t,
                            (32768 - 31473) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8562) as uint16_t,
                            (32768 - 21327) as uint16_t,
                            (32768 - 27914) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4846) as uint16_t,
                            (32768 - 13393) as uint16_t,
                            (32768 - 19919) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29981) as uint16_t,
                            (32768 - 32590) as uint16_t,
                            (32768 - 32695) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20465) as uint16_t,
                            (32768 - 30963) as uint16_t,
                            (32768 - 32166) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11479) as uint16_t,
                            (32768 - 23579) as uint16_t,
                            (32768 - 28195) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5916) as uint16_t,
                            (32768 - 15648) as uint16_t,
                            (32768 - 22073) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3031) as uint16_t,
                            (32768 - 8605) as uint16_t,
                            (32768 - 13398) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 31146) as uint16_t,
                            (32768 - 32691) as uint16_t,
                            (32768 - 32739) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23106) as uint16_t,
                            (32768 - 31724) as uint16_t,
                            (32768 - 32444) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13783) as uint16_t,
                            (32768 - 26738) as uint16_t,
                            (32768 - 30439) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7852) as uint16_t,
                            (32768 - 19468) as uint16_t,
                            (32768 - 25807) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3860) as uint16_t,
                            (32768 - 11124) as uint16_t,
                            (32768 - 16853) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 31014) as uint16_t,
                            (32768 - 32724) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23629) as uint16_t,
                            (32768 - 32109) as uint16_t,
                            (32768 - 32628) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14747) as uint16_t,
                            (32768 - 28115) as uint16_t,
                            (32768 - 31403) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8545) as uint16_t,
                            (32768 - 21242) as uint16_t,
                            (32768 - 27478) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4574) as uint16_t,
                            (32768 - 12781) as uint16_t,
                            (32768 - 19067) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 9185) as uint16_t,
                            (32768 - 19694) as uint16_t,
                            (32768 - 24688) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 26081) as uint16_t,
                            (32768 - 31985) as uint16_t,
                            (32768 - 32621) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16015) as uint16_t,
                            (32768 - 29000) as uint16_t,
                            (32768 - 31787) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10542) as uint16_t,
                            (32768 - 23690) as uint16_t,
                            (32768 - 29206) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6732) as uint16_t,
                            (32768 - 17945) as uint16_t,
                            (32768 - 24677) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3916) as uint16_t,
                            (32768 - 11039) as uint16_t,
                            (32768 - 16722) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28224) as uint16_t,
                            (32768 - 32566) as uint16_t,
                            (32768 - 32744) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19100) as uint16_t,
                            (32768 - 31138) as uint16_t,
                            (32768 - 32485) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12528) as uint16_t,
                            (32768 - 26620) as uint16_t,
                            (32768 - 30879) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7741) as uint16_t,
                            (32768 - 20277) as uint16_t,
                            (32768 - 26885) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4566) as uint16_t,
                            (32768 - 12845) as uint16_t,
                            (32768 - 18990) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29933) as uint16_t,
                            (32768 - 32593) as uint16_t,
                            (32768 - 32718) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17670) as uint16_t,
                            (32768 - 30333) as uint16_t,
                            (32768 - 32155) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10385) as uint16_t,
                            (32768 - 23600) as uint16_t,
                            (32768 - 28909) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6243) as uint16_t,
                            (32768 - 16236) as uint16_t,
                            (32768 - 22407) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3976) as uint16_t,
                            (32768 - 10389) as uint16_t,
                            (32768 - 16017) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28377) as uint16_t,
                            (32768 - 32561) as uint16_t,
                            (32768 - 32738) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19366) as uint16_t,
                            (32768 - 31175) as uint16_t,
                            (32768 - 32482) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13327) as uint16_t,
                            (32768 - 27175) as uint16_t,
                            (32768 - 31094) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8258) as uint16_t,
                            (32768 - 20769) as uint16_t,
                            (32768 - 27143) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4703) as uint16_t,
                            (32768 - 13198) as uint16_t,
                            (32768 - 19527) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 31086) as uint16_t,
                            (32768 - 32706) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22853) as uint16_t,
                            (32768 - 31902) as uint16_t,
                            (32768 - 32583) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14759) as uint16_t,
                            (32768 - 28186) as uint16_t,
                            (32768 - 31419) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9284) as uint16_t,
                            (32768 - 22382) as uint16_t,
                            (32768 - 28348) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5585) as uint16_t,
                            (32768 - 15192) as uint16_t,
                            (32768 - 21868) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28291) as uint16_t,
                            (32768 - 32652) as uint16_t,
                            (32768 - 32746) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19849) as uint16_t,
                            (32768 - 32107) as uint16_t,
                            (32768 - 32571) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14834) as uint16_t,
                            (32768 - 26818) as uint16_t,
                            (32768 - 29214) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10306) as uint16_t,
                            (32768 - 22594) as uint16_t,
                            (32768 - 28672) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6615) as uint16_t,
                            (32768 - 17384) as uint16_t,
                            (32768 - 23384) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28947) as uint16_t,
                            (32768 - 32604) as uint16_t,
                            (32768 - 32745) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 25625) as uint16_t,
                            (32768 - 32289) as uint16_t,
                            (32768 - 32646) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18758) as uint16_t,
                            (32768 - 28672) as uint16_t,
                            (32768 - 31403) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10017) as uint16_t,
                            (32768 - 23430) as uint16_t,
                            (32768 - 28523) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6862) as uint16_t,
                            (32768 - 15269) as uint16_t,
                            (32768 - 22131) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23933) as uint16_t,
                            (32768 - 32509) as uint16_t,
                            (32768 - 32739) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19927) as uint16_t,
                            (32768 - 31495) as uint16_t,
                            (32768 - 32631) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11903) as uint16_t,
                            (32768 - 26023) as uint16_t,
                            (32768 - 30621) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7026) as uint16_t,
                            (32768 - 20094) as uint16_t,
                            (32768 - 27252) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5998) as uint16_t,
                            (32768 - 18106) as uint16_t,
                            (32768 - 24437) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 4456) as uint16_t,
                            (32768 - 11274) as uint16_t,
                            (32768 - 15533) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21219) as uint16_t,
                            (32768 - 29079) as uint16_t,
                            (32768 - 31616) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11173) as uint16_t,
                            (32768 - 23774) as uint16_t,
                            (32768 - 28567) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7282) as uint16_t,
                            (32768 - 18293) as uint16_t,
                            (32768 - 24263) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4890) as uint16_t,
                            (32768 - 13286) as uint16_t,
                            (32768 - 19115) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1890) as uint16_t,
                            (32768 - 5508) as uint16_t,
                            (32768 - 8659) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 26651) as uint16_t,
                            (32768 - 32136) as uint16_t,
                            (32768 - 32647) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14630) as uint16_t,
                            (32768 - 28254) as uint16_t,
                            (32768 - 31455) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8716) as uint16_t,
                            (32768 - 21287) as uint16_t,
                            (32768 - 27395) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5615) as uint16_t,
                            (32768 - 15331) as uint16_t,
                            (32768 - 22008) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2675) as uint16_t,
                            (32768 - 7700) as uint16_t,
                            (32768 - 12150) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29954) as uint16_t,
                            (32768 - 32526) as uint16_t,
                            (32768 - 32690) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16126) as uint16_t,
                            (32768 - 28982) as uint16_t,
                            (32768 - 31633) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9030) as uint16_t,
                            (32768 - 21361) as uint16_t,
                            (32768 - 27352) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5411) as uint16_t,
                            (32768 - 14793) as uint16_t,
                            (32768 - 21271) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2943) as uint16_t,
                            (32768 - 8422) as uint16_t,
                            (32768 - 13163) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29539) as uint16_t,
                            (32768 - 32601) as uint16_t,
                            (32768 - 32730) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18125) as uint16_t,
                            (32768 - 30385) as uint16_t,
                            (32768 - 32201) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10422) as uint16_t,
                            (32768 - 24090) as uint16_t,
                            (32768 - 29468) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6468) as uint16_t,
                            (32768 - 17487) as uint16_t,
                            (32768 - 24438) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2970) as uint16_t,
                            (32768 - 8653) as uint16_t,
                            (32768 - 13531) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 30912) as uint16_t,
                            (32768 - 32715) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20666) as uint16_t,
                            (32768 - 31373) as uint16_t,
                            (32768 - 32497) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12509) as uint16_t,
                            (32768 - 26640) as uint16_t,
                            (32768 - 30917) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8058) as uint16_t,
                            (32768 - 20629) as uint16_t,
                            (32768 - 27290) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4231) as uint16_t,
                            (32768 - 12006) as uint16_t,
                            (32768 - 18052) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 10202) as uint16_t,
                            (32768 - 20633) as uint16_t,
                            (32768 - 25484) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27336) as uint16_t,
                            (32768 - 31445) as uint16_t,
                            (32768 - 32352) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12420) as uint16_t,
                            (32768 - 24384) as uint16_t,
                            (32768 - 28552) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7648) as uint16_t,
                            (32768 - 18115) as uint16_t,
                            (32768 - 23856) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5662) as uint16_t,
                            (32768 - 14341) as uint16_t,
                            (32768 - 19902) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3611) as uint16_t,
                            (32768 - 10328) as uint16_t,
                            (32768 - 15390) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 30945) as uint16_t,
                            (32768 - 32616) as uint16_t,
                            (32768 - 32736) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18682) as uint16_t,
                            (32768 - 30505) as uint16_t,
                            (32768 - 32253) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11513) as uint16_t,
                            (32768 - 25336) as uint16_t,
                            (32768 - 30203) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7449) as uint16_t,
                            (32768 - 19452) as uint16_t,
                            (32768 - 26148) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4482) as uint16_t,
                            (32768 - 13051) as uint16_t,
                            (32768 - 18886) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 32022) as uint16_t,
                            (32768 - 32690) as uint16_t,
                            (32768 - 32747) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18578) as uint16_t,
                            (32768 - 30501) as uint16_t,
                            (32768 - 32146) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11249) as uint16_t,
                            (32768 - 23368) as uint16_t,
                            (32768 - 28631) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5645) as uint16_t,
                            (32768 - 16958) as uint16_t,
                            (32768 - 22158) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5009) as uint16_t,
                            (32768 - 11444) as uint16_t,
                            (32768 - 16637) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 31357) as uint16_t,
                            (32768 - 32710) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21552) as uint16_t,
                            (32768 - 31494) as uint16_t,
                            (32768 - 32504) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13891) as uint16_t,
                            (32768 - 27677) as uint16_t,
                            (32768 - 31340) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9051) as uint16_t,
                            (32768 - 22098) as uint16_t,
                            (32768 - 28172) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5190) as uint16_t,
                            (32768 - 13377) as uint16_t,
                            (32768 - 19486) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 32364) as uint16_t,
                            (32768 - 32740) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24839) as uint16_t,
                            (32768 - 31907) as uint16_t,
                            (32768 - 32551) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17160) as uint16_t,
                            (32768 - 28779) as uint16_t,
                            (32768 - 31696) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12452) as uint16_t,
                            (32768 - 24137) as uint16_t,
                            (32768 - 29602) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6165) as uint16_t,
                            (32768 - 15389) as uint16_t,
                            (32768 - 22477) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 2575) as uint16_t,
                            (32768 - 7281) as uint16_t,
                            (32768 - 11077) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14002) as uint16_t,
                            (32768 - 20866) as uint16_t,
                            (32768 - 25402) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6343) as uint16_t,
                            (32768 - 15056) as uint16_t,
                            (32768 - 19658) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4474) as uint16_t,
                            (32768 - 11858) as uint16_t,
                            (32768 - 17041) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2865) as uint16_t,
                            (32768 - 8299) as uint16_t,
                            (32768 - 12534) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1344) as uint16_t,
                            (32768 - 3949) as uint16_t,
                            (32768 - 6391) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24720) as uint16_t,
                            (32768 - 31239) as uint16_t,
                            (32768 - 32459) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12585) as uint16_t,
                            (32768 - 25356) as uint16_t,
                            (32768 - 29968) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7181) as uint16_t,
                            (32768 - 18246) as uint16_t,
                            (32768 - 24444) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5025) as uint16_t,
                            (32768 - 13667) as uint16_t,
                            (32768 - 19885) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2521) as uint16_t,
                            (32768 - 7304) as uint16_t,
                            (32768 - 11605) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29908) as uint16_t,
                            (32768 - 32252) as uint16_t,
                            (32768 - 32584) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17421) as uint16_t,
                            (32768 - 29156) as uint16_t,
                            (32768 - 31575) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9889) as uint16_t,
                            (32768 - 22188) as uint16_t,
                            (32768 - 27782) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5878) as uint16_t,
                            (32768 - 15647) as uint16_t,
                            (32768 - 22123) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2814) as uint16_t,
                            (32768 - 8665) as uint16_t,
                            (32768 - 13323) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 30183) as uint16_t,
                            (32768 - 32568) as uint16_t,
                            (32768 - 32713) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18528) as uint16_t,
                            (32768 - 30195) as uint16_t,
                            (32768 - 32049) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10982) as uint16_t,
                            (32768 - 24606) as uint16_t,
                            (32768 - 29657) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6957) as uint16_t,
                            (32768 - 18165) as uint16_t,
                            (32768 - 25231) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3508) as uint16_t,
                            (32768 - 10118) as uint16_t,
                            (32768 - 15468) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 31761) as uint16_t,
                            (32768 - 32736) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21041) as uint16_t,
                            (32768 - 31328) as uint16_t,
                            (32768 - 32546) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12568) as uint16_t,
                            (32768 - 26732) as uint16_t,
                            (32768 - 31166) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8052) as uint16_t,
                            (32768 - 20720) as uint16_t,
                            (32768 - 27733) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4336) as uint16_t,
                            (32768 - 12192) as uint16_t,
                            (32768 - 18396) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                    ],
                ],
            ].into(),
            br_tok: [
                [
                    [
                        [
                            (32768 - 16138) as uint16_t,
                            (32768 - 22223) as uint16_t,
                            (32768 - 25509) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15347) as uint16_t,
                            (32768 - 22430) as uint16_t,
                            (32768 - 26332) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9614) as uint16_t,
                            (32768 - 16736) as uint16_t,
                            (32768 - 21332) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6600) as uint16_t,
                            (32768 - 12275) as uint16_t,
                            (32768 - 16907) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4811) as uint16_t,
                            (32768 - 9424) as uint16_t,
                            (32768 - 13547) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3748) as uint16_t,
                            (32768 - 7809) as uint16_t,
                            (32768 - 11420) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2254) as uint16_t,
                            (32768 - 4587) as uint16_t,
                            (32768 - 6890) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15196) as uint16_t,
                            (32768 - 20284) as uint16_t,
                            (32768 - 23177) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18317) as uint16_t,
                            (32768 - 25469) as uint16_t,
                            (32768 - 28451) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13918) as uint16_t,
                            (32768 - 21651) as uint16_t,
                            (32768 - 25842) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10052) as uint16_t,
                            (32768 - 17150) as uint16_t,
                            (32768 - 21995) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7499) as uint16_t,
                            (32768 - 13630) as uint16_t,
                            (32768 - 18587) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6158) as uint16_t,
                            (32768 - 11417) as uint16_t,
                            (32768 - 16003) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4014) as uint16_t,
                            (32768 - 7785) as uint16_t,
                            (32768 - 11252) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15048) as uint16_t,
                            (32768 - 21067) as uint16_t,
                            (32768 - 24384) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18202) as uint16_t,
                            (32768 - 25346) as uint16_t,
                            (32768 - 28553) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14302) as uint16_t,
                            (32768 - 22019) as uint16_t,
                            (32768 - 26356) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10839) as uint16_t,
                            (32768 - 18139) as uint16_t,
                            (32768 - 23166) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8715) as uint16_t,
                            (32768 - 15744) as uint16_t,
                            (32768 - 20806) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7536) as uint16_t,
                            (32768 - 13576) as uint16_t,
                            (32768 - 18544) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5413) as uint16_t,
                            (32768 - 10335) as uint16_t,
                            (32768 - 14498) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 17394) as uint16_t,
                            (32768 - 24501) as uint16_t,
                            (32768 - 27895) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15889) as uint16_t,
                            (32768 - 23420) as uint16_t,
                            (32768 - 27185) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11561) as uint16_t,
                            (32768 - 19133) as uint16_t,
                            (32768 - 23870) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8285) as uint16_t,
                            (32768 - 14812) as uint16_t,
                            (32768 - 19844) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6496) as uint16_t,
                            (32768 - 12043) as uint16_t,
                            (32768 - 16550) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4771) as uint16_t,
                            (32768 - 9574) as uint16_t,
                            (32768 - 13677) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3603) as uint16_t,
                            (32768 - 6830) as uint16_t,
                            (32768 - 10144) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21656) as uint16_t,
                            (32768 - 27704) as uint16_t,
                            (32768 - 30200) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21324) as uint16_t,
                            (32768 - 27915) as uint16_t,
                            (32768 - 30511) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17327) as uint16_t,
                            (32768 - 25336) as uint16_t,
                            (32768 - 28997) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13417) as uint16_t,
                            (32768 - 21381) as uint16_t,
                            (32768 - 26033) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10132) as uint16_t,
                            (32768 - 17425) as uint16_t,
                            (32768 - 22338) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8580) as uint16_t,
                            (32768 - 15016) as uint16_t,
                            (32768 - 19633) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5694) as uint16_t,
                            (32768 - 11477) as uint16_t,
                            (32768 - 16411) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24116) as uint16_t,
                            (32768 - 29780) as uint16_t,
                            (32768 - 31450) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23853) as uint16_t,
                            (32768 - 29695) as uint16_t,
                            (32768 - 31591) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20085) as uint16_t,
                            (32768 - 27614) as uint16_t,
                            (32768 - 30428) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15326) as uint16_t,
                            (32768 - 24335) as uint16_t,
                            (32768 - 28575) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11814) as uint16_t,
                            (32768 - 19472) as uint16_t,
                            (32768 - 24810) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10221) as uint16_t,
                            (32768 - 18611) as uint16_t,
                            (32768 - 24767) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7689) as uint16_t,
                            (32768 - 14558) as uint16_t,
                            (32768 - 20321) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 16214) as uint16_t,
                            (32768 - 22380) as uint16_t,
                            (32768 - 25770) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14213) as uint16_t,
                            (32768 - 21304) as uint16_t,
                            (32768 - 25295) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9213) as uint16_t,
                            (32768 - 15823) as uint16_t,
                            (32768 - 20455) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6395) as uint16_t,
                            (32768 - 11758) as uint16_t,
                            (32768 - 16139) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4779) as uint16_t,
                            (32768 - 9187) as uint16_t,
                            (32768 - 13066) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3821) as uint16_t,
                            (32768 - 7501) as uint16_t,
                            (32768 - 10953) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2293) as uint16_t,
                            (32768 - 4567) as uint16_t,
                            (32768 - 6795) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15859) as uint16_t,
                            (32768 - 21283) as uint16_t,
                            (32768 - 23820) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18404) as uint16_t,
                            (32768 - 25602) as uint16_t,
                            (32768 - 28726) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14325) as uint16_t,
                            (32768 - 21980) as uint16_t,
                            (32768 - 26206) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10669) as uint16_t,
                            (32768 - 17937) as uint16_t,
                            (32768 - 22720) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8297) as uint16_t,
                            (32768 - 14642) as uint16_t,
                            (32768 - 19447) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6746) as uint16_t,
                            (32768 - 12389) as uint16_t,
                            (32768 - 16893) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4324) as uint16_t,
                            (32768 - 8251) as uint16_t,
                            (32768 - 11770) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16532) as uint16_t,
                            (32768 - 21631) as uint16_t,
                            (32768 - 24475) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20667) as uint16_t,
                            (32768 - 27150) as uint16_t,
                            (32768 - 29668) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16728) as uint16_t,
                            (32768 - 24510) as uint16_t,
                            (32768 - 28175) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12861) as uint16_t,
                            (32768 - 20645) as uint16_t,
                            (32768 - 25332) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10076) as uint16_t,
                            (32768 - 17361) as uint16_t,
                            (32768 - 22417) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8395) as uint16_t,
                            (32768 - 14940) as uint16_t,
                            (32768 - 19963) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5731) as uint16_t,
                            (32768 - 10683) as uint16_t,
                            (32768 - 14912) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 14433) as uint16_t,
                            (32768 - 21155) as uint16_t,
                            (32768 - 24938) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14658) as uint16_t,
                            (32768 - 21716) as uint16_t,
                            (32768 - 25545) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9923) as uint16_t,
                            (32768 - 16824) as uint16_t,
                            (32768 - 21557) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6982) as uint16_t,
                            (32768 - 13052) as uint16_t,
                            (32768 - 17721) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5419) as uint16_t,
                            (32768 - 10503) as uint16_t,
                            (32768 - 15050) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4852) as uint16_t,
                            (32768 - 9162) as uint16_t,
                            (32768 - 13014) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3271) as uint16_t,
                            (32768 - 6395) as uint16_t,
                            (32768 - 9630) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22210) as uint16_t,
                            (32768 - 27833) as uint16_t,
                            (32768 - 30109) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20750) as uint16_t,
                            (32768 - 27368) as uint16_t,
                            (32768 - 29821) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16894) as uint16_t,
                            (32768 - 24828) as uint16_t,
                            (32768 - 28573) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13247) as uint16_t,
                            (32768 - 21276) as uint16_t,
                            (32768 - 25757) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10038) as uint16_t,
                            (32768 - 17265) as uint16_t,
                            (32768 - 22563) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8587) as uint16_t,
                            (32768 - 14947) as uint16_t,
                            (32768 - 20327) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5645) as uint16_t,
                            (32768 - 11371) as uint16_t,
                            (32768 - 15252) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22027) as uint16_t,
                            (32768 - 27526) as uint16_t,
                            (32768 - 29714) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23098) as uint16_t,
                            (32768 - 29146) as uint16_t,
                            (32768 - 31221) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19886) as uint16_t,
                            (32768 - 27341) as uint16_t,
                            (32768 - 30272) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15609) as uint16_t,
                            (32768 - 23747) as uint16_t,
                            (32768 - 28046) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11993) as uint16_t,
                            (32768 - 20065) as uint16_t,
                            (32768 - 24939) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9637) as uint16_t,
                            (32768 - 18267) as uint16_t,
                            (32768 - 23671) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7625) as uint16_t,
                            (32768 - 13801) as uint16_t,
                            (32768 - 19144) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 14438) as uint16_t,
                            (32768 - 20798) as uint16_t,
                            (32768 - 24089) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12621) as uint16_t,
                            (32768 - 19203) as uint16_t,
                            (32768 - 23097) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8177) as uint16_t,
                            (32768 - 14125) as uint16_t,
                            (32768 - 18402) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5674) as uint16_t,
                            (32768 - 10501) as uint16_t,
                            (32768 - 14456) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4236) as uint16_t,
                            (32768 - 8239) as uint16_t,
                            (32768 - 11733) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3447) as uint16_t,
                            (32768 - 6750) as uint16_t,
                            (32768 - 9806) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1986) as uint16_t,
                            (32768 - 3950) as uint16_t,
                            (32768 - 5864) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16208) as uint16_t,
                            (32768 - 22099) as uint16_t,
                            (32768 - 24930) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16537) as uint16_t,
                            (32768 - 24025) as uint16_t,
                            (32768 - 27585) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12780) as uint16_t,
                            (32768 - 20381) as uint16_t,
                            (32768 - 24867) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9767) as uint16_t,
                            (32768 - 16612) as uint16_t,
                            (32768 - 21416) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7686) as uint16_t,
                            (32768 - 13738) as uint16_t,
                            (32768 - 18398) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6333) as uint16_t,
                            (32768 - 11614) as uint16_t,
                            (32768 - 15964) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3941) as uint16_t,
                            (32768 - 7571) as uint16_t,
                            (32768 - 10836) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22819) as uint16_t,
                            (32768 - 27422) as uint16_t,
                            (32768 - 29202) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22224) as uint16_t,
                            (32768 - 28514) as uint16_t,
                            (32768 - 30721) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17660) as uint16_t,
                            (32768 - 25433) as uint16_t,
                            (32768 - 28913) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13574) as uint16_t,
                            (32768 - 21482) as uint16_t,
                            (32768 - 26002) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10629) as uint16_t,
                            (32768 - 17977) as uint16_t,
                            (32768 - 22938) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8612) as uint16_t,
                            (32768 - 15298) as uint16_t,
                            (32768 - 20265) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5607) as uint16_t,
                            (32768 - 10491) as uint16_t,
                            (32768 - 14596) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 13569) as uint16_t,
                            (32768 - 19800) as uint16_t,
                            (32768 - 23206) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13128) as uint16_t,
                            (32768 - 19924) as uint16_t,
                            (32768 - 23869) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8329) as uint16_t,
                            (32768 - 14841) as uint16_t,
                            (32768 - 19403) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6130) as uint16_t,
                            (32768 - 10976) as uint16_t,
                            (32768 - 15057) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4682) as uint16_t,
                            (32768 - 8839) as uint16_t,
                            (32768 - 12518) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3656) as uint16_t,
                            (32768 - 7409) as uint16_t,
                            (32768 - 10588) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2577) as uint16_t,
                            (32768 - 5099) as uint16_t,
                            (32768 - 7412) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22427) as uint16_t,
                            (32768 - 28684) as uint16_t,
                            (32768 - 30585) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20913) as uint16_t,
                            (32768 - 27750) as uint16_t,
                            (32768 - 30139) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15840) as uint16_t,
                            (32768 - 24109) as uint16_t,
                            (32768 - 27834) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12308) as uint16_t,
                            (32768 - 20029) as uint16_t,
                            (32768 - 24569) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10216) as uint16_t,
                            (32768 - 16785) as uint16_t,
                            (32768 - 21458) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8309) as uint16_t,
                            (32768 - 14203) as uint16_t,
                            (32768 - 19113) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6043) as uint16_t,
                            (32768 - 11168) as uint16_t,
                            (32768 - 15307) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23166) as uint16_t,
                            (32768 - 28901) as uint16_t,
                            (32768 - 30998) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21899) as uint16_t,
                            (32768 - 28405) as uint16_t,
                            (32768 - 30751) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18413) as uint16_t,
                            (32768 - 26091) as uint16_t,
                            (32768 - 29443) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15233) as uint16_t,
                            (32768 - 23114) as uint16_t,
                            (32768 - 27352) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12683) as uint16_t,
                            (32768 - 20472) as uint16_t,
                            (32768 - 25288) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10702) as uint16_t,
                            (32768 - 18259) as uint16_t,
                            (32768 - 23409) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8125) as uint16_t,
                            (32768 - 14464) as uint16_t,
                            (32768 - 19226) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 9040) as uint16_t,
                            (32768 - 14786) as uint16_t,
                            (32768 - 18360) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9979) as uint16_t,
                            (32768 - 15718) as uint16_t,
                            (32768 - 19415) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7913) as uint16_t,
                            (32768 - 13918) as uint16_t,
                            (32768 - 18311) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5859) as uint16_t,
                            (32768 - 10889) as uint16_t,
                            (32768 - 15184) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4593) as uint16_t,
                            (32768 - 8677) as uint16_t,
                            (32768 - 12510) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3820) as uint16_t,
                            (32768 - 7396) as uint16_t,
                            (32768 - 10791) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1730) as uint16_t,
                            (32768 - 3471) as uint16_t,
                            (32768 - 5192) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11803) as uint16_t,
                            (32768 - 18365) as uint16_t,
                            (32768 - 22709) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11419) as uint16_t,
                            (32768 - 18058) as uint16_t,
                            (32768 - 22225) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9418) as uint16_t,
                            (32768 - 15774) as uint16_t,
                            (32768 - 20243) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7539) as uint16_t,
                            (32768 - 13325) as uint16_t,
                            (32768 - 17657) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6233) as uint16_t,
                            (32768 - 11317) as uint16_t,
                            (32768 - 15384) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5137) as uint16_t,
                            (32768 - 9656) as uint16_t,
                            (32768 - 13545) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2977) as uint16_t,
                            (32768 - 5774) as uint16_t,
                            (32768 - 8349) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21207) as uint16_t,
                            (32768 - 27246) as uint16_t,
                            (32768 - 29640) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19547) as uint16_t,
                            (32768 - 26578) as uint16_t,
                            (32768 - 29497) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16169) as uint16_t,
                            (32768 - 23871) as uint16_t,
                            (32768 - 27690) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12820) as uint16_t,
                            (32768 - 20458) as uint16_t,
                            (32768 - 25018) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10224) as uint16_t,
                            (32768 - 17332) as uint16_t,
                            (32768 - 22214) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8526) as uint16_t,
                            (32768 - 15048) as uint16_t,
                            (32768 - 19884) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5037) as uint16_t,
                            (32768 - 9410) as uint16_t,
                            (32768 - 13118) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 12339) as uint16_t,
                            (32768 - 17329) as uint16_t,
                            (32768 - 20140) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13505) as uint16_t,
                            (32768 - 19895) as uint16_t,
                            (32768 - 23225) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9847) as uint16_t,
                            (32768 - 16944) as uint16_t,
                            (32768 - 21564) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7280) as uint16_t,
                            (32768 - 13256) as uint16_t,
                            (32768 - 18348) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4712) as uint16_t,
                            (32768 - 10009) as uint16_t,
                            (32768 - 14454) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4361) as uint16_t,
                            (32768 - 7914) as uint16_t,
                            (32768 - 12477) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2870) as uint16_t,
                            (32768 - 5628) as uint16_t,
                            (32768 - 7995) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20061) as uint16_t,
                            (32768 - 25504) as uint16_t,
                            (32768 - 28526) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15235) as uint16_t,
                            (32768 - 22878) as uint16_t,
                            (32768 - 26145) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12985) as uint16_t,
                            (32768 - 19958) as uint16_t,
                            (32768 - 24155) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9782) as uint16_t,
                            (32768 - 16641) as uint16_t,
                            (32768 - 21403) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9456) as uint16_t,
                            (32768 - 16360) as uint16_t,
                            (32768 - 20760) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6855) as uint16_t,
                            (32768 - 12940) as uint16_t,
                            (32768 - 18557) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5661) as uint16_t,
                            (32768 - 10564) as uint16_t,
                            (32768 - 15002) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 25656) as uint16_t,
                            (32768 - 30602) as uint16_t,
                            (32768 - 31894) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22570) as uint16_t,
                            (32768 - 29107) as uint16_t,
                            (32768 - 31092) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18917) as uint16_t,
                            (32768 - 26423) as uint16_t,
                            (32768 - 29541) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15940) as uint16_t,
                            (32768 - 23649) as uint16_t,
                            (32768 - 27754) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12803) as uint16_t,
                            (32768 - 20581) as uint16_t,
                            (32768 - 25219) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11082) as uint16_t,
                            (32768 - 18695) as uint16_t,
                            (32768 - 23376) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7939) as uint16_t,
                            (32768 - 14373) as uint16_t,
                            (32768 - 19005) as uint16_t,
                            0,
                        ],
                    ],
                ],
            ].into(),
            eob_hi_bit: [
                [
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 18983) as uint16_t, 0],
                        [(32768 - 20512) as uint16_t, 0],
                        [(32768 - 14885) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 20090) as uint16_t, 0],
                        [(32768 - 19444) as uint16_t, 0],
                        [(32768 - 17286) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                ],
                [
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 19139) as uint16_t, 0],
                        [(32768 - 21487) as uint16_t, 0],
                        [(32768 - 18959) as uint16_t, 0],
                        [(32768 - 20910) as uint16_t, 0],
                        [(32768 - 19089) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 20536) as uint16_t, 0],
                        [(32768 - 20664) as uint16_t, 0],
                        [(32768 - 20625) as uint16_t, 0],
                        [(32768 - 19123) as uint16_t, 0],
                        [(32768 - 14862) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                ],
                [
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 19833) as uint16_t, 0],
                        [(32768 - 21502) as uint16_t, 0],
                        [(32768 - 17485) as uint16_t, 0],
                        [(32768 - 20267) as uint16_t, 0],
                        [(32768 - 18353) as uint16_t, 0],
                        [(32768 - 23329) as uint16_t, 0],
                        [(32768 - 21478) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 22041) as uint16_t, 0],
                        [(32768 - 23434) as uint16_t, 0],
                        [(32768 - 20001) as uint16_t, 0],
                        [(32768 - 20554) as uint16_t, 0],
                        [(32768 - 20951) as uint16_t, 0],
                        [(32768 - 20145) as uint16_t, 0],
                        [(32768 - 15562) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                ],
                [
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 23312) as uint16_t, 0],
                        [(32768 - 21607) as uint16_t, 0],
                        [(32768 - 16526) as uint16_t, 0],
                        [(32768 - 18957) as uint16_t, 0],
                        [(32768 - 18034) as uint16_t, 0],
                        [(32768 - 18934) as uint16_t, 0],
                        [(32768 - 24247) as uint16_t, 0],
                        [(32768 - 16921) as uint16_t, 0],
                        [(32768 - 17080) as uint16_t, 0],
                    ],
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 26579) as uint16_t, 0],
                        [(32768 - 24910) as uint16_t, 0],
                        [(32768 - 18637) as uint16_t, 0],
                        [(32768 - 19800) as uint16_t, 0],
                        [(32768 - 20388) as uint16_t, 0],
                        [(32768 - 9887) as uint16_t, 0],
                        [(32768 - 15642) as uint16_t, 0],
                        [(32768 - 30198) as uint16_t, 0],
                        [(32768 - 24721) as uint16_t, 0],
                    ],
                ],
                [
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 26998) as uint16_t, 0],
                        [(32768 - 16737) as uint16_t, 0],
                        [(32768 - 17838) as uint16_t, 0],
                        [(32768 - 18922) as uint16_t, 0],
                        [(32768 - 19515) as uint16_t, 0],
                        [(32768 - 18636) as uint16_t, 0],
                        [(32768 - 17333) as uint16_t, 0],
                        [(32768 - 15776) as uint16_t, 0],
                        [(32768 - 22658) as uint16_t, 0],
                    ],
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                ],
            ].into(),
            skip: [
                [
                    [(32768 - 29614) as uint16_t, 0],
                    [(32768 - 9068) as uint16_t, 0],
                    [(32768 - 12924) as uint16_t, 0],
                    [(32768 - 19538) as uint16_t, 0],
                    [(32768 - 17737) as uint16_t, 0],
                    [(32768 - 24619) as uint16_t, 0],
                    [(32768 - 30642) as uint16_t, 0],
                    [(32768 - 4119) as uint16_t, 0],
                    [(32768 - 16026) as uint16_t, 0],
                    [(32768 - 25657) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                ],
                [
                    [(32768 - 31957) as uint16_t, 0],
                    [(32768 - 3230) as uint16_t, 0],
                    [(32768 - 11153) as uint16_t, 0],
                    [(32768 - 18123) as uint16_t, 0],
                    [(32768 - 20143) as uint16_t, 0],
                    [(32768 - 26536) as uint16_t, 0],
                    [(32768 - 31986) as uint16_t, 0],
                    [(32768 - 3050) as uint16_t, 0],
                    [(32768 - 14603) as uint16_t, 0],
                    [(32768 - 25155) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                ],
                [
                    [(32768 - 32363) as uint16_t, 0],
                    [(32768 - 10692) as uint16_t, 0],
                    [(32768 - 19090) as uint16_t, 0],
                    [(32768 - 24357) as uint16_t, 0],
                    [(32768 - 24442) as uint16_t, 0],
                    [(32768 - 28312) as uint16_t, 0],
                    [(32768 - 32169) as uint16_t, 0],
                    [(32768 - 3648) as uint16_t, 0],
                    [(32768 - 15690) as uint16_t, 0],
                    [(32768 - 26815) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                ],
                [
                    [(32768 - 30669) as uint16_t, 0],
                    [(32768 - 3832) as uint16_t, 0],
                    [(32768 - 11663) as uint16_t, 0],
                    [(32768 - 18889) as uint16_t, 0],
                    [(32768 - 19782) as uint16_t, 0],
                    [(32768 - 23313) as uint16_t, 0],
                    [(32768 - 31330) as uint16_t, 0],
                    [(32768 - 5124) as uint16_t, 0],
                    [(32768 - 18719) as uint16_t, 0],
                    [(32768 - 28468) as uint16_t, 0],
                    [(32768 - 3082) as uint16_t, 0],
                    [(32768 - 20982) as uint16_t, 0],
                    [(32768 - 29443) as uint16_t, 0],
                ],
                [
                    [(32768 - 28573) as uint16_t, 0],
                    [(32768 - 3183) as uint16_t, 0],
                    [(32768 - 17802) as uint16_t, 0],
                    [(32768 - 25977) as uint16_t, 0],
                    [(32768 - 26677) as uint16_t, 0],
                    [(32768 - 27832) as uint16_t, 0],
                    [(32768 - 32387) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                ],
            ].into(),
            dc_sign: [
                [
                    [(32768 - 16000) as uint16_t, 0],
                    [(32768 - 13056) as uint16_t, 0],
                    [(32768 - 18816) as uint16_t, 0],
                ],
                [
                    [(32768 - 15232) as uint16_t, 0],
                    [(32768 - 12928) as uint16_t, 0],
                    [(32768 - 17280) as uint16_t, 0],
                ],
            ].into(),
        };
        init
    },
    {
        let mut init = CdfCoefContext {
            eob_bin_16: [
                [
                    [
                        (32768 - 6708) as uint16_t,
                        (32768 - 8958) as uint16_t,
                        (32768 - 14746) as uint16_t,
                        (32768 - 22133) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 1222) as uint16_t,
                        (32768 - 2074) as uint16_t,
                        (32768 - 4783) as uint16_t,
                        (32768 - 15410) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 19575) as uint16_t,
                        (32768 - 21766) as uint16_t,
                        (32768 - 26044) as uint16_t,
                        (32768 - 29709) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 7297) as uint16_t,
                        (32768 - 10767) as uint16_t,
                        (32768 - 19273) as uint16_t,
                        (32768 - 28194) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                    ],
                ],
            ].into(),
            eob_bin_32: [
                [
                    [
                        (32768 - 4617) as uint16_t,
                        (32768 - 5709) as uint16_t,
                        (32768 - 8446) as uint16_t,
                        (32768 - 13584) as uint16_t,
                        (32768 - 23135) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 1156) as uint16_t,
                        (32768 - 1702) as uint16_t,
                        (32768 - 3675) as uint16_t,
                        (32768 - 9274) as uint16_t,
                        (32768 - 20539) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 22086) as uint16_t,
                        (32768 - 24282) as uint16_t,
                        (32768 - 27010) as uint16_t,
                        (32768 - 29770) as uint16_t,
                        (32768 - 31743) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 7699) as uint16_t,
                        (32768 - 10897) as uint16_t,
                        (32768 - 20891) as uint16_t,
                        (32768 - 26926) as uint16_t,
                        (32768 - 31628) as uint16_t,
                        0,
                        0,
                        0,
                    ],
                ],
            ].into(),
            eob_bin_64: [
                [
                    [
                        (32768 - 6307) as uint16_t,
                        (32768 - 7541) as uint16_t,
                        (32768 - 12060) as uint16_t,
                        (32768 - 16358) as uint16_t,
                        (32768 - 22553) as uint16_t,
                        (32768 - 27865) as uint16_t,
                        0,
                        0,
                    ],
                    [
                        (32768 - 1289) as uint16_t,
                        (32768 - 2320) as uint16_t,
                        (32768 - 3971) as uint16_t,
                        (32768 - 7926) as uint16_t,
                        (32768 - 14153) as uint16_t,
                        (32768 - 24291) as uint16_t,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 24212) as uint16_t,
                        (32768 - 25708) as uint16_t,
                        (32768 - 28268) as uint16_t,
                        (32768 - 30035) as uint16_t,
                        (32768 - 31307) as uint16_t,
                        (32768 - 32049) as uint16_t,
                        0,
                        0,
                    ],
                    [
                        (32768 - 8726) as uint16_t,
                        (32768 - 12378) as uint16_t,
                        (32768 - 19409) as uint16_t,
                        (32768 - 26450) as uint16_t,
                        (32768 - 30038) as uint16_t,
                        (32768 - 32462) as uint16_t,
                        0,
                        0,
                    ],
                ],
            ].into(),
            eob_bin_128: [
                [
                    [
                        (32768 - 3472) as uint16_t,
                        (32768 - 4885) as uint16_t,
                        (32768 - 7489) as uint16_t,
                        (32768 - 12481) as uint16_t,
                        (32768 - 18517) as uint16_t,
                        (32768 - 24536) as uint16_t,
                        (32768 - 29635) as uint16_t,
                        0,
                    ],
                    [
                        (32768 - 886) as uint16_t,
                        (32768 - 1731) as uint16_t,
                        (32768 - 3271) as uint16_t,
                        (32768 - 8469) as uint16_t,
                        (32768 - 15569) as uint16_t,
                        (32768 - 22126) as uint16_t,
                        (32768 - 28383) as uint16_t,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 24313) as uint16_t,
                        (32768 - 26062) as uint16_t,
                        (32768 - 28385) as uint16_t,
                        (32768 - 30107) as uint16_t,
                        (32768 - 31217) as uint16_t,
                        (32768 - 31898) as uint16_t,
                        (32768 - 32345) as uint16_t,
                        0,
                    ],
                    [
                        (32768 - 9165) as uint16_t,
                        (32768 - 13282) as uint16_t,
                        (32768 - 21150) as uint16_t,
                        (32768 - 30286) as uint16_t,
                        (32768 - 31894) as uint16_t,
                        (32768 - 32571) as uint16_t,
                        (32768 - 32712) as uint16_t,
                        0,
                    ],
                ],
            ].into(),
            eob_bin_256: [
                [
                    [
                        (32768 - 5348) as uint16_t,
                        (32768 - 7113) as uint16_t,
                        (32768 - 11820) as uint16_t,
                        (32768 - 15924) as uint16_t,
                        (32768 - 22106) as uint16_t,
                        (32768 - 26777) as uint16_t,
                        (32768 - 30334) as uint16_t,
                        (32768 - 31757) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 2453) as uint16_t,
                        (32768 - 4474) as uint16_t,
                        (32768 - 6307) as uint16_t,
                        (32768 - 8777) as uint16_t,
                        (32768 - 16474) as uint16_t,
                        (32768 - 22975) as uint16_t,
                        (32768 - 29000) as uint16_t,
                        (32768 - 31547) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                ],
                [
                    [
                        (32768 - 23110) as uint16_t,
                        (32768 - 24597) as uint16_t,
                        (32768 - 27140) as uint16_t,
                        (32768 - 28894) as uint16_t,
                        (32768 - 30167) as uint16_t,
                        (32768 - 30927) as uint16_t,
                        (32768 - 31392) as uint16_t,
                        (32768 - 32094) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    [
                        (32768 - 9998) as uint16_t,
                        (32768 - 17661) as uint16_t,
                        (32768 - 25178) as uint16_t,
                        (32768 - 28097) as uint16_t,
                        (32768 - 31308) as uint16_t,
                        (32768 - 32038) as uint16_t,
                        (32768 - 32403) as uint16_t,
                        (32768 - 32695) as uint16_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                ],
            ].into(),
            eob_bin_512: [
                [
                    (32768 - 5927) as uint16_t,
                    (32768 - 7809) as uint16_t,
                    (32768 - 10923) as uint16_t,
                    (32768 - 14597) as uint16_t,
                    (32768 - 19439) as uint16_t,
                    (32768 - 24135) as uint16_t,
                    (32768 - 28456) as uint16_t,
                    (32768 - 31142) as uint16_t,
                    (32768 - 32060) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 21093) as uint16_t,
                    (32768 - 23043) as uint16_t,
                    (32768 - 25742) as uint16_t,
                    (32768 - 27658) as uint16_t,
                    (32768 - 29097) as uint16_t,
                    (32768 - 29716) as uint16_t,
                    (32768 - 30073) as uint16_t,
                    (32768 - 30820) as uint16_t,
                    (32768 - 31956) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            ].into(),
            eob_bin_1024: [
                [
                    (32768 - 6698) as uint16_t,
                    (32768 - 8334) as uint16_t,
                    (32768 - 11961) as uint16_t,
                    (32768 - 15762) as uint16_t,
                    (32768 - 20186) as uint16_t,
                    (32768 - 23862) as uint16_t,
                    (32768 - 27434) as uint16_t,
                    (32768 - 29326) as uint16_t,
                    (32768 - 31082) as uint16_t,
                    (32768 - 32050) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                [
                    (32768 - 20569) as uint16_t,
                    (32768 - 22426) as uint16_t,
                    (32768 - 25569) as uint16_t,
                    (32768 - 26859) as uint16_t,
                    (32768 - 28053) as uint16_t,
                    (32768 - 28913) as uint16_t,
                    (32768 - 29486) as uint16_t,
                    (32768 - 29724) as uint16_t,
                    (32768 - 29807) as uint16_t,
                    (32768 - 32570) as uint16_t,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            ].into(),
            eob_base_tok: [
                [
                    [
                        [
                            (32768 - 22497) as uint16_t,
                            (32768 - 31198) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31715) as uint16_t,
                            (32768 - 32495) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31606) as uint16_t,
                            (32768 - 32337) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 30388) as uint16_t,
                            (32768 - 31990) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 27877) as uint16_t,
                            (32768 - 31584) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32170) as uint16_t,
                            (32768 - 32728) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32155) as uint16_t,
                            (32768 - 32688) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32219) as uint16_t,
                            (32768 - 32702) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 21457) as uint16_t,
                            (32768 - 31043) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31951) as uint16_t,
                            (32768 - 32483) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32153) as uint16_t,
                            (32768 - 32562) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31473) as uint16_t,
                            (32768 - 32215) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 27558) as uint16_t,
                            (32768 - 31151) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32020) as uint16_t,
                            (32768 - 32640) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32097) as uint16_t,
                            (32768 - 32575) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32242) as uint16_t,
                            (32768 - 32719) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 19980) as uint16_t,
                            (32768 - 30591) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32219) as uint16_t,
                            (32768 - 32597) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32581) as uint16_t,
                            (32768 - 32706) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31803) as uint16_t,
                            (32768 - 32287) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 26473) as uint16_t,
                            (32768 - 30507) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32431) as uint16_t,
                            (32768 - 32723) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32196) as uint16_t,
                            (32768 - 32611) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31588) as uint16_t,
                            (32768 - 32528) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 24647) as uint16_t,
                            (32768 - 30463) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32412) as uint16_t,
                            (32768 - 32695) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32468) as uint16_t,
                            (32768 - 32720) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31269) as uint16_t,
                            (32768 - 32523) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 28482) as uint16_t,
                            (32768 - 31505) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32152) as uint16_t,
                            (32768 - 32701) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31732) as uint16_t,
                            (32768 - 32598) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31767) as uint16_t,
                            (32768 - 32712) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 12358) as uint16_t,
                            (32768 - 24977) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 31331) as uint16_t,
                            (32768 - 32385) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 32634) as uint16_t,
                            (32768 - 32756) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 30411) as uint16_t,
                            (32768 - 32548) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 10923) as uint16_t,
                            (32768 - 21845) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 10923) as uint16_t,
                            (32768 - 21845) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 10923) as uint16_t,
                            (32768 - 21845) as uint16_t,
                            0,
                            0,
                        ],
                        [
                            (32768 - 10923) as uint16_t,
                            (32768 - 21845) as uint16_t,
                            0,
                            0,
                        ],
                    ],
                ],
            ].into(),
            base_tok: [
                [
                    [
                        [
                            (32768 - 7062) as uint16_t,
                            (32768 - 16472) as uint16_t,
                            (32768 - 22319) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24538) as uint16_t,
                            (32768 - 32261) as uint16_t,
                            (32768 - 32674) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13675) as uint16_t,
                            (32768 - 28041) as uint16_t,
                            (32768 - 31779) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8590) as uint16_t,
                            (32768 - 20674) as uint16_t,
                            (32768 - 27631) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5685) as uint16_t,
                            (32768 - 14675) as uint16_t,
                            (32768 - 22013) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3655) as uint16_t,
                            (32768 - 9898) as uint16_t,
                            (32768 - 15731) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 26493) as uint16_t,
                            (32768 - 32418) as uint16_t,
                            (32768 - 32658) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16376) as uint16_t,
                            (32768 - 29342) as uint16_t,
                            (32768 - 32090) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10594) as uint16_t,
                            (32768 - 22649) as uint16_t,
                            (32768 - 28970) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8176) as uint16_t,
                            (32768 - 17170) as uint16_t,
                            (32768 - 24303) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5605) as uint16_t,
                            (32768 - 12694) as uint16_t,
                            (32768 - 19139) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23888) as uint16_t,
                            (32768 - 31902) as uint16_t,
                            (32768 - 32542) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18612) as uint16_t,
                            (32768 - 29687) as uint16_t,
                            (32768 - 31987) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16245) as uint16_t,
                            (32768 - 24852) as uint16_t,
                            (32768 - 29249) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15765) as uint16_t,
                            (32768 - 22608) as uint16_t,
                            (32768 - 27559) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19895) as uint16_t,
                            (32768 - 24699) as uint16_t,
                            (32768 - 27510) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28401) as uint16_t,
                            (32768 - 32212) as uint16_t,
                            (32768 - 32457) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15274) as uint16_t,
                            (32768 - 27825) as uint16_t,
                            (32768 - 30980) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9364) as uint16_t,
                            (32768 - 18128) as uint16_t,
                            (32768 - 24332) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2283) as uint16_t,
                            (32768 - 8193) as uint16_t,
                            (32768 - 15082) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1228) as uint16_t,
                            (32768 - 3972) as uint16_t,
                            (32768 - 7881) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29455) as uint16_t,
                            (32768 - 32469) as uint16_t,
                            (32768 - 32620) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17981) as uint16_t,
                            (32768 - 28245) as uint16_t,
                            (32768 - 31388) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10921) as uint16_t,
                            (32768 - 20098) as uint16_t,
                            (32768 - 26240) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3743) as uint16_t,
                            (32768 - 11829) as uint16_t,
                            (32768 - 18657) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2374) as uint16_t,
                            (32768 - 9593) as uint16_t,
                            (32768 - 15715) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 31068) as uint16_t,
                            (32768 - 32466) as uint16_t,
                            (32768 - 32635) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20321) as uint16_t,
                            (32768 - 29572) as uint16_t,
                            (32768 - 31971) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10771) as uint16_t,
                            (32768 - 20255) as uint16_t,
                            (32768 - 27119) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2795) as uint16_t,
                            (32768 - 10410) as uint16_t,
                            (32768 - 17361) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 9320) as uint16_t,
                            (32768 - 22102) as uint16_t,
                            (32768 - 27840) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27057) as uint16_t,
                            (32768 - 32464) as uint16_t,
                            (32768 - 32724) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16331) as uint16_t,
                            (32768 - 30268) as uint16_t,
                            (32768 - 32309) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10319) as uint16_t,
                            (32768 - 23935) as uint16_t,
                            (32768 - 29720) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6189) as uint16_t,
                            (32768 - 16448) as uint16_t,
                            (32768 - 24106) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3589) as uint16_t,
                            (32768 - 10884) as uint16_t,
                            (32768 - 18808) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29026) as uint16_t,
                            (32768 - 32624) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19226) as uint16_t,
                            (32768 - 31507) as uint16_t,
                            (32768 - 32587) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12692) as uint16_t,
                            (32768 - 26921) as uint16_t,
                            (32768 - 31203) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7049) as uint16_t,
                            (32768 - 19532) as uint16_t,
                            (32768 - 27635) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7727) as uint16_t,
                            (32768 - 15669) as uint16_t,
                            (32768 - 23252) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28056) as uint16_t,
                            (32768 - 32625) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22383) as uint16_t,
                            (32768 - 32075) as uint16_t,
                            (32768 - 32669) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15417) as uint16_t,
                            (32768 - 27098) as uint16_t,
                            (32768 - 31749) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18127) as uint16_t,
                            (32768 - 26493) as uint16_t,
                            (32768 - 27190) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5461) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 21845) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27982) as uint16_t,
                            (32768 - 32091) as uint16_t,
                            (32768 - 32584) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19045) as uint16_t,
                            (32768 - 29868) as uint16_t,
                            (32768 - 31972) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10397) as uint16_t,
                            (32768 - 22266) as uint16_t,
                            (32768 - 27932) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5990) as uint16_t,
                            (32768 - 13697) as uint16_t,
                            (32768 - 21500) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 1792) as uint16_t,
                            (32768 - 6912) as uint16_t,
                            (32768 - 15104) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28198) as uint16_t,
                            (32768 - 32501) as uint16_t,
                            (32768 - 32718) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21534) as uint16_t,
                            (32768 - 31521) as uint16_t,
                            (32768 - 32569) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11109) as uint16_t,
                            (32768 - 25217) as uint16_t,
                            (32768 - 30017) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5671) as uint16_t,
                            (32768 - 15124) as uint16_t,
                            (32768 - 26151) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4681) as uint16_t,
                            (32768 - 14043) as uint16_t,
                            (32768 - 18725) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28688) as uint16_t,
                            (32768 - 32580) as uint16_t,
                            (32768 - 32741) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22576) as uint16_t,
                            (32768 - 32079) as uint16_t,
                            (32768 - 32661) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10627) as uint16_t,
                            (32768 - 22141) as uint16_t,
                            (32768 - 28340) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9362) as uint16_t,
                            (32768 - 14043) as uint16_t,
                            (32768 - 28087) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 7754) as uint16_t,
                            (32768 - 16948) as uint16_t,
                            (32768 - 22142) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 25670) as uint16_t,
                            (32768 - 32330) as uint16_t,
                            (32768 - 32691) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15663) as uint16_t,
                            (32768 - 29225) as uint16_t,
                            (32768 - 31994) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9878) as uint16_t,
                            (32768 - 23288) as uint16_t,
                            (32768 - 29158) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6419) as uint16_t,
                            (32768 - 17088) as uint16_t,
                            (32768 - 24336) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3859) as uint16_t,
                            (32768 - 11003) as uint16_t,
                            (32768 - 17039) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27562) as uint16_t,
                            (32768 - 32595) as uint16_t,
                            (32768 - 32725) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17575) as uint16_t,
                            (32768 - 30588) as uint16_t,
                            (32768 - 32399) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10819) as uint16_t,
                            (32768 - 24838) as uint16_t,
                            (32768 - 30309) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7124) as uint16_t,
                            (32768 - 18686) as uint16_t,
                            (32768 - 25916) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4479) as uint16_t,
                            (32768 - 12688) as uint16_t,
                            (32768 - 19340) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28385) as uint16_t,
                            (32768 - 32476) as uint16_t,
                            (32768 - 32673) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15306) as uint16_t,
                            (32768 - 29005) as uint16_t,
                            (32768 - 31938) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8937) as uint16_t,
                            (32768 - 21615) as uint16_t,
                            (32768 - 28322) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5982) as uint16_t,
                            (32768 - 15603) as uint16_t,
                            (32768 - 22786) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3620) as uint16_t,
                            (32768 - 10267) as uint16_t,
                            (32768 - 16136) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27280) as uint16_t,
                            (32768 - 32464) as uint16_t,
                            (32768 - 32667) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15607) as uint16_t,
                            (32768 - 29160) as uint16_t,
                            (32768 - 32004) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9091) as uint16_t,
                            (32768 - 22135) as uint16_t,
                            (32768 - 28740) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6232) as uint16_t,
                            (32768 - 16632) as uint16_t,
                            (32768 - 24020) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4047) as uint16_t,
                            (32768 - 11377) as uint16_t,
                            (32768 - 17672) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29220) as uint16_t,
                            (32768 - 32630) as uint16_t,
                            (32768 - 32718) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19650) as uint16_t,
                            (32768 - 31220) as uint16_t,
                            (32768 - 32462) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13050) as uint16_t,
                            (32768 - 26312) as uint16_t,
                            (32768 - 30827) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9228) as uint16_t,
                            (32768 - 20870) as uint16_t,
                            (32768 - 27468) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6146) as uint16_t,
                            (32768 - 15149) as uint16_t,
                            (32768 - 21971) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 30169) as uint16_t,
                            (32768 - 32481) as uint16_t,
                            (32768 - 32623) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17212) as uint16_t,
                            (32768 - 29311) as uint16_t,
                            (32768 - 31554) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9911) as uint16_t,
                            (32768 - 21311) as uint16_t,
                            (32768 - 26882) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4487) as uint16_t,
                            (32768 - 13314) as uint16_t,
                            (32768 - 20372) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2570) as uint16_t,
                            (32768 - 7772) as uint16_t,
                            (32768 - 12889) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 30924) as uint16_t,
                            (32768 - 32613) as uint16_t,
                            (32768 - 32708) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19490) as uint16_t,
                            (32768 - 30206) as uint16_t,
                            (32768 - 32107) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11232) as uint16_t,
                            (32768 - 23998) as uint16_t,
                            (32768 - 29276) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6769) as uint16_t,
                            (32768 - 17955) as uint16_t,
                            (32768 - 25035) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4398) as uint16_t,
                            (32768 - 12623) as uint16_t,
                            (32768 - 19214) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 30609) as uint16_t,
                            (32768 - 32627) as uint16_t,
                            (32768 - 32722) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19370) as uint16_t,
                            (32768 - 30582) as uint16_t,
                            (32768 - 32287) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10457) as uint16_t,
                            (32768 - 23619) as uint16_t,
                            (32768 - 29409) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6443) as uint16_t,
                            (32768 - 17637) as uint16_t,
                            (32768 - 24834) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4645) as uint16_t,
                            (32768 - 13236) as uint16_t,
                            (32768 - 20106) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 8626) as uint16_t,
                            (32768 - 20271) as uint16_t,
                            (32768 - 26216) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 26707) as uint16_t,
                            (32768 - 32406) as uint16_t,
                            (32768 - 32711) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16999) as uint16_t,
                            (32768 - 30329) as uint16_t,
                            (32768 - 32286) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11445) as uint16_t,
                            (32768 - 25123) as uint16_t,
                            (32768 - 30286) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6411) as uint16_t,
                            (32768 - 18828) as uint16_t,
                            (32768 - 25601) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6801) as uint16_t,
                            (32768 - 12458) as uint16_t,
                            (32768 - 20248) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29918) as uint16_t,
                            (32768 - 32682) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20649) as uint16_t,
                            (32768 - 31739) as uint16_t,
                            (32768 - 32618) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12879) as uint16_t,
                            (32768 - 27773) as uint16_t,
                            (32768 - 31581) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7896) as uint16_t,
                            (32768 - 21751) as uint16_t,
                            (32768 - 28244) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5260) as uint16_t,
                            (32768 - 14870) as uint16_t,
                            (32768 - 23698) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29252) as uint16_t,
                            (32768 - 32593) as uint16_t,
                            (32768 - 32731) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17072) as uint16_t,
                            (32768 - 30460) as uint16_t,
                            (32768 - 32294) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10653) as uint16_t,
                            (32768 - 24143) as uint16_t,
                            (32768 - 29365) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6536) as uint16_t,
                            (32768 - 17490) as uint16_t,
                            (32768 - 23983) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4929) as uint16_t,
                            (32768 - 13170) as uint16_t,
                            (32768 - 20085) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28137) as uint16_t,
                            (32768 - 32518) as uint16_t,
                            (32768 - 32715) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18171) as uint16_t,
                            (32768 - 30784) as uint16_t,
                            (32768 - 32407) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11437) as uint16_t,
                            (32768 - 25436) as uint16_t,
                            (32768 - 30459) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7252) as uint16_t,
                            (32768 - 18534) as uint16_t,
                            (32768 - 26176) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4126) as uint16_t,
                            (32768 - 13353) as uint16_t,
                            (32768 - 20978) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 31162) as uint16_t,
                            (32768 - 32726) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23017) as uint16_t,
                            (32768 - 32222) as uint16_t,
                            (32768 - 32701) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15629) as uint16_t,
                            (32768 - 29233) as uint16_t,
                            (32768 - 32046) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9387) as uint16_t,
                            (32768 - 22621) as uint16_t,
                            (32768 - 29480) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6922) as uint16_t,
                            (32768 - 17616) as uint16_t,
                            (32768 - 25010) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28838) as uint16_t,
                            (32768 - 32265) as uint16_t,
                            (32768 - 32614) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19701) as uint16_t,
                            (32768 - 30206) as uint16_t,
                            (32768 - 31920) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11214) as uint16_t,
                            (32768 - 22410) as uint16_t,
                            (32768 - 27933) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5320) as uint16_t,
                            (32768 - 14177) as uint16_t,
                            (32768 - 23034) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5049) as uint16_t,
                            (32768 - 12881) as uint16_t,
                            (32768 - 17827) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27484) as uint16_t,
                            (32768 - 32471) as uint16_t,
                            (32768 - 32734) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21076) as uint16_t,
                            (32768 - 31526) as uint16_t,
                            (32768 - 32561) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12707) as uint16_t,
                            (32768 - 26303) as uint16_t,
                            (32768 - 31211) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8169) as uint16_t,
                            (32768 - 21722) as uint16_t,
                            (32768 - 28219) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6045) as uint16_t,
                            (32768 - 19406) as uint16_t,
                            (32768 - 27042) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27753) as uint16_t,
                            (32768 - 32572) as uint16_t,
                            (32768 - 32745) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20832) as uint16_t,
                            (32768 - 31878) as uint16_t,
                            (32768 - 32653) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13250) as uint16_t,
                            (32768 - 27356) as uint16_t,
                            (32768 - 31674) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7718) as uint16_t,
                            (32768 - 21508) as uint16_t,
                            (32768 - 29858) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7209) as uint16_t,
                            (32768 - 18350) as uint16_t,
                            (32768 - 25559) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 7876) as uint16_t,
                            (32768 - 16901) as uint16_t,
                            (32768 - 21741) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24001) as uint16_t,
                            (32768 - 31898) as uint16_t,
                            (32768 - 32625) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14529) as uint16_t,
                            (32768 - 27959) as uint16_t,
                            (32768 - 31451) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8273) as uint16_t,
                            (32768 - 20818) as uint16_t,
                            (32768 - 27258) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5278) as uint16_t,
                            (32768 - 14673) as uint16_t,
                            (32768 - 21510) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2983) as uint16_t,
                            (32768 - 8843) as uint16_t,
                            (32768 - 14039) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28016) as uint16_t,
                            (32768 - 32574) as uint16_t,
                            (32768 - 32732) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17471) as uint16_t,
                            (32768 - 30306) as uint16_t,
                            (32768 - 32301) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10224) as uint16_t,
                            (32768 - 24063) as uint16_t,
                            (32768 - 29728) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6602) as uint16_t,
                            (32768 - 17954) as uint16_t,
                            (32768 - 25052) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4002) as uint16_t,
                            (32768 - 11585) as uint16_t,
                            (32768 - 17759) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 30190) as uint16_t,
                            (32768 - 32634) as uint16_t,
                            (32768 - 32739) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17497) as uint16_t,
                            (32768 - 30282) as uint16_t,
                            (32768 - 32270) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10229) as uint16_t,
                            (32768 - 23729) as uint16_t,
                            (32768 - 29538) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6344) as uint16_t,
                            (32768 - 17211) as uint16_t,
                            (32768 - 24440) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3849) as uint16_t,
                            (32768 - 11189) as uint16_t,
                            (32768 - 17108) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28570) as uint16_t,
                            (32768 - 32583) as uint16_t,
                            (32768 - 32726) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17521) as uint16_t,
                            (32768 - 30161) as uint16_t,
                            (32768 - 32238) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10153) as uint16_t,
                            (32768 - 23565) as uint16_t,
                            (32768 - 29378) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6455) as uint16_t,
                            (32768 - 17341) as uint16_t,
                            (32768 - 24443) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3907) as uint16_t,
                            (32768 - 11042) as uint16_t,
                            (32768 - 17024) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 30689) as uint16_t,
                            (32768 - 32715) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21546) as uint16_t,
                            (32768 - 31840) as uint16_t,
                            (32768 - 32610) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13547) as uint16_t,
                            (32768 - 27581) as uint16_t,
                            (32768 - 31459) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8912) as uint16_t,
                            (32768 - 21757) as uint16_t,
                            (32768 - 28309) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5548) as uint16_t,
                            (32768 - 15080) as uint16_t,
                            (32768 - 22046) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 30783) as uint16_t,
                            (32768 - 32540) as uint16_t,
                            (32768 - 32685) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17540) as uint16_t,
                            (32768 - 29528) as uint16_t,
                            (32768 - 31668) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10160) as uint16_t,
                            (32768 - 21468) as uint16_t,
                            (32768 - 26783) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4724) as uint16_t,
                            (32768 - 13393) as uint16_t,
                            (32768 - 20054) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2702) as uint16_t,
                            (32768 - 8174) as uint16_t,
                            (32768 - 13102) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 31648) as uint16_t,
                            (32768 - 32686) as uint16_t,
                            (32768 - 32742) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20954) as uint16_t,
                            (32768 - 31094) as uint16_t,
                            (32768 - 32337) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12420) as uint16_t,
                            (32768 - 25698) as uint16_t,
                            (32768 - 30179) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7304) as uint16_t,
                            (32768 - 19320) as uint16_t,
                            (32768 - 26248) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4366) as uint16_t,
                            (32768 - 12261) as uint16_t,
                            (32768 - 18864) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 31581) as uint16_t,
                            (32768 - 32723) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21373) as uint16_t,
                            (32768 - 31586) as uint16_t,
                            (32768 - 32525) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12744) as uint16_t,
                            (32768 - 26625) as uint16_t,
                            (32768 - 30885) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7431) as uint16_t,
                            (32768 - 20322) as uint16_t,
                            (32768 - 26950) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4692) as uint16_t,
                            (32768 - 13323) as uint16_t,
                            (32768 - 20111) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 7833) as uint16_t,
                            (32768 - 18369) as uint16_t,
                            (32768 - 24095) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 26650) as uint16_t,
                            (32768 - 32273) as uint16_t,
                            (32768 - 32702) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16371) as uint16_t,
                            (32768 - 29961) as uint16_t,
                            (32768 - 32191) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11055) as uint16_t,
                            (32768 - 24082) as uint16_t,
                            (32768 - 29629) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6892) as uint16_t,
                            (32768 - 18644) as uint16_t,
                            (32768 - 25400) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5006) as uint16_t,
                            (32768 - 13057) as uint16_t,
                            (32768 - 19240) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29834) as uint16_t,
                            (32768 - 32666) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19577) as uint16_t,
                            (32768 - 31335) as uint16_t,
                            (32768 - 32570) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12253) as uint16_t,
                            (32768 - 26509) as uint16_t,
                            (32768 - 31122) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7991) as uint16_t,
                            (32768 - 20772) as uint16_t,
                            (32768 - 27711) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5677) as uint16_t,
                            (32768 - 15910) as uint16_t,
                            (32768 - 23059) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 30109) as uint16_t,
                            (32768 - 32532) as uint16_t,
                            (32768 - 32720) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16747) as uint16_t,
                            (32768 - 30166) as uint16_t,
                            (32768 - 32252) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10134) as uint16_t,
                            (32768 - 23542) as uint16_t,
                            (32768 - 29184) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5791) as uint16_t,
                            (32768 - 16176) as uint16_t,
                            (32768 - 23556) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4362) as uint16_t,
                            (32768 - 10414) as uint16_t,
                            (32768 - 17284) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29492) as uint16_t,
                            (32768 - 32626) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19894) as uint16_t,
                            (32768 - 31402) as uint16_t,
                            (32768 - 32525) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12942) as uint16_t,
                            (32768 - 27071) as uint16_t,
                            (32768 - 30869) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8346) as uint16_t,
                            (32768 - 21216) as uint16_t,
                            (32768 - 27405) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6572) as uint16_t,
                            (32768 - 17087) as uint16_t,
                            (32768 - 23859) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 32035) as uint16_t,
                            (32768 - 32735) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22957) as uint16_t,
                            (32768 - 31838) as uint16_t,
                            (32768 - 32618) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14724) as uint16_t,
                            (32768 - 28572) as uint16_t,
                            (32768 - 31772) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10364) as uint16_t,
                            (32768 - 23999) as uint16_t,
                            (32768 - 29553) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7004) as uint16_t,
                            (32768 - 18433) as uint16_t,
                            (32768 - 25655) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27528) as uint16_t,
                            (32768 - 32277) as uint16_t,
                            (32768 - 32681) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16959) as uint16_t,
                            (32768 - 31171) as uint16_t,
                            (32768 - 32096) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10486) as uint16_t,
                            (32768 - 23593) as uint16_t,
                            (32768 - 27962) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 23211) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8937) as uint16_t,
                            (32768 - 17873) as uint16_t,
                            (32768 - 20852) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27715) as uint16_t,
                            (32768 - 32002) as uint16_t,
                            (32768 - 32615) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15073) as uint16_t,
                            (32768 - 29491) as uint16_t,
                            (32768 - 31676) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11264) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            (32768 - 28672) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2341) as uint16_t,
                            (32768 - 18725) as uint16_t,
                            (32768 - 23406) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7282) as uint16_t,
                            (32768 - 18204) as uint16_t,
                            (32768 - 25486) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28547) as uint16_t,
                            (32768 - 32213) as uint16_t,
                            (32768 - 32657) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20788) as uint16_t,
                            (32768 - 29773) as uint16_t,
                            (32768 - 32239) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6780) as uint16_t,
                            (32768 - 21469) as uint16_t,
                            (32768 - 30508) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5958) as uint16_t,
                            (32768 - 14895) as uint16_t,
                            (32768 - 23831) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16384) as uint16_t,
                            (32768 - 21845) as uint16_t,
                            (32768 - 27307) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 5992) as uint16_t,
                            (32768 - 14304) as uint16_t,
                            (32768 - 19765) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22612) as uint16_t,
                            (32768 - 31238) as uint16_t,
                            (32768 - 32456) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13456) as uint16_t,
                            (32768 - 27162) as uint16_t,
                            (32768 - 31087) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8001) as uint16_t,
                            (32768 - 20062) as uint16_t,
                            (32768 - 26504) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5168) as uint16_t,
                            (32768 - 14105) as uint16_t,
                            (32768 - 20764) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2632) as uint16_t,
                            (32768 - 7771) as uint16_t,
                            (32768 - 12385) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27034) as uint16_t,
                            (32768 - 32344) as uint16_t,
                            (32768 - 32709) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15850) as uint16_t,
                            (32768 - 29415) as uint16_t,
                            (32768 - 31997) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9494) as uint16_t,
                            (32768 - 22776) as uint16_t,
                            (32768 - 28841) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6151) as uint16_t,
                            (32768 - 16830) as uint16_t,
                            (32768 - 23969) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3461) as uint16_t,
                            (32768 - 10039) as uint16_t,
                            (32768 - 15722) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 30134) as uint16_t,
                            (32768 - 32569) as uint16_t,
                            (32768 - 32731) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15638) as uint16_t,
                            (32768 - 29422) as uint16_t,
                            (32768 - 31945) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9150) as uint16_t,
                            (32768 - 21865) as uint16_t,
                            (32768 - 28218) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5647) as uint16_t,
                            (32768 - 15719) as uint16_t,
                            (32768 - 22676) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3402) as uint16_t,
                            (32768 - 9772) as uint16_t,
                            (32768 - 15477) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28530) as uint16_t,
                            (32768 - 32586) as uint16_t,
                            (32768 - 32735) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17139) as uint16_t,
                            (32768 - 30298) as uint16_t,
                            (32768 - 32292) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10200) as uint16_t,
                            (32768 - 24039) as uint16_t,
                            (32768 - 29685) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6419) as uint16_t,
                            (32768 - 17674) as uint16_t,
                            (32768 - 24786) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3544) as uint16_t,
                            (32768 - 10225) as uint16_t,
                            (32768 - 15824) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 31333) as uint16_t,
                            (32768 - 32726) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20618) as uint16_t,
                            (32768 - 31487) as uint16_t,
                            (32768 - 32544) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12901) as uint16_t,
                            (32768 - 27217) as uint16_t,
                            (32768 - 31232) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8624) as uint16_t,
                            (32768 - 21734) as uint16_t,
                            (32768 - 28171) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5104) as uint16_t,
                            (32768 - 14191) as uint16_t,
                            (32768 - 20748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 11206) as uint16_t,
                            (32768 - 21090) as uint16_t,
                            (32768 - 26561) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 28759) as uint16_t,
                            (32768 - 32279) as uint16_t,
                            (32768 - 32671) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14171) as uint16_t,
                            (32768 - 27952) as uint16_t,
                            (32768 - 31569) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9743) as uint16_t,
                            (32768 - 22907) as uint16_t,
                            (32768 - 29141) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6871) as uint16_t,
                            (32768 - 17886) as uint16_t,
                            (32768 - 24868) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4960) as uint16_t,
                            (32768 - 13152) as uint16_t,
                            (32768 - 19315) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 31077) as uint16_t,
                            (32768 - 32661) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19400) as uint16_t,
                            (32768 - 31195) as uint16_t,
                            (32768 - 32515) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12752) as uint16_t,
                            (32768 - 26858) as uint16_t,
                            (32768 - 31040) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8370) as uint16_t,
                            (32768 - 22098) as uint16_t,
                            (32768 - 28591) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5457) as uint16_t,
                            (32768 - 15373) as uint16_t,
                            (32768 - 22298) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 31697) as uint16_t,
                            (32768 - 32706) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17860) as uint16_t,
                            (32768 - 30657) as uint16_t,
                            (32768 - 32333) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12510) as uint16_t,
                            (32768 - 24812) as uint16_t,
                            (32768 - 29261) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6180) as uint16_t,
                            (32768 - 19124) as uint16_t,
                            (32768 - 24722) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5041) as uint16_t,
                            (32768 - 13548) as uint16_t,
                            (32768 - 17959) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 31552) as uint16_t,
                            (32768 - 32716) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21908) as uint16_t,
                            (32768 - 31769) as uint16_t,
                            (32768 - 32623) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14470) as uint16_t,
                            (32768 - 28201) as uint16_t,
                            (32768 - 31565) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9493) as uint16_t,
                            (32768 - 22982) as uint16_t,
                            (32768 - 28608) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6858) as uint16_t,
                            (32768 - 17240) as uint16_t,
                            (32768 - 24137) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 32543) as uint16_t,
                            (32768 - 32752) as uint16_t,
                            (32768 - 32756) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24286) as uint16_t,
                            (32768 - 32097) as uint16_t,
                            (32768 - 32666) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15958) as uint16_t,
                            (32768 - 29217) as uint16_t,
                            (32768 - 32024) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10207) as uint16_t,
                            (32768 - 24234) as uint16_t,
                            (32768 - 29958) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6929) as uint16_t,
                            (32768 - 18305) as uint16_t,
                            (32768 - 25652) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 4137) as uint16_t,
                            (32768 - 10847) as uint16_t,
                            (32768 - 15682) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17824) as uint16_t,
                            (32768 - 27001) as uint16_t,
                            (32768 - 30058) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10204) as uint16_t,
                            (32768 - 22796) as uint16_t,
                            (32768 - 28291) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6076) as uint16_t,
                            (32768 - 15935) as uint16_t,
                            (32768 - 22125) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3852) as uint16_t,
                            (32768 - 10937) as uint16_t,
                            (32768 - 16816) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2252) as uint16_t,
                            (32768 - 6324) as uint16_t,
                            (32768 - 10131) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 25840) as uint16_t,
                            (32768 - 32016) as uint16_t,
                            (32768 - 32662) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15109) as uint16_t,
                            (32768 - 28268) as uint16_t,
                            (32768 - 31531) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9385) as uint16_t,
                            (32768 - 22231) as uint16_t,
                            (32768 - 28340) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6082) as uint16_t,
                            (32768 - 16672) as uint16_t,
                            (32768 - 23479) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3318) as uint16_t,
                            (32768 - 9427) as uint16_t,
                            (32768 - 14681) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 30594) as uint16_t,
                            (32768 - 32574) as uint16_t,
                            (32768 - 32718) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16836) as uint16_t,
                            (32768 - 29552) as uint16_t,
                            (32768 - 31859) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9556) as uint16_t,
                            (32768 - 22542) as uint16_t,
                            (32768 - 28356) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6305) as uint16_t,
                            (32768 - 16725) as uint16_t,
                            (32768 - 23540) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3376) as uint16_t,
                            (32768 - 9895) as uint16_t,
                            (32768 - 15184) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 29383) as uint16_t,
                            (32768 - 32617) as uint16_t,
                            (32768 - 32745) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18891) as uint16_t,
                            (32768 - 30809) as uint16_t,
                            (32768 - 32401) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11688) as uint16_t,
                            (32768 - 25942) as uint16_t,
                            (32768 - 30687) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7468) as uint16_t,
                            (32768 - 19469) as uint16_t,
                            (32768 - 26651) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3909) as uint16_t,
                            (32768 - 11358) as uint16_t,
                            (32768 - 17012) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 31564) as uint16_t,
                            (32768 - 32736) as uint16_t,
                            (32768 - 32748) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20906) as uint16_t,
                            (32768 - 31611) as uint16_t,
                            (32768 - 32600) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13191) as uint16_t,
                            (32768 - 27621) as uint16_t,
                            (32768 - 31537) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8768) as uint16_t,
                            (32768 - 22029) as uint16_t,
                            (32768 - 28676) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5079) as uint16_t,
                            (32768 - 14109) as uint16_t,
                            (32768 - 20906) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8192) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                    ],
                ],
            ].into(),
            br_tok: [
                [
                    [
                        [
                            (32768 - 18315) as uint16_t,
                            (32768 - 24289) as uint16_t,
                            (32768 - 27551) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16854) as uint16_t,
                            (32768 - 24068) as uint16_t,
                            (32768 - 27835) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10140) as uint16_t,
                            (32768 - 17927) as uint16_t,
                            (32768 - 23173) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6722) as uint16_t,
                            (32768 - 12982) as uint16_t,
                            (32768 - 18267) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4661) as uint16_t,
                            (32768 - 9826) as uint16_t,
                            (32768 - 14706) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3832) as uint16_t,
                            (32768 - 8165) as uint16_t,
                            (32768 - 12294) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2795) as uint16_t,
                            (32768 - 6098) as uint16_t,
                            (32768 - 9245) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17145) as uint16_t,
                            (32768 - 23326) as uint16_t,
                            (32768 - 26672) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20733) as uint16_t,
                            (32768 - 27680) as uint16_t,
                            (32768 - 30308) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16032) as uint16_t,
                            (32768 - 24461) as uint16_t,
                            (32768 - 28546) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11653) as uint16_t,
                            (32768 - 20093) as uint16_t,
                            (32768 - 25081) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9290) as uint16_t,
                            (32768 - 16429) as uint16_t,
                            (32768 - 22086) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7796) as uint16_t,
                            (32768 - 14598) as uint16_t,
                            (32768 - 19982) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6502) as uint16_t,
                            (32768 - 12378) as uint16_t,
                            (32768 - 17441) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21681) as uint16_t,
                            (32768 - 27732) as uint16_t,
                            (32768 - 30320) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22389) as uint16_t,
                            (32768 - 29044) as uint16_t,
                            (32768 - 31261) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19027) as uint16_t,
                            (32768 - 26731) as uint16_t,
                            (32768 - 30087) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14739) as uint16_t,
                            (32768 - 23755) as uint16_t,
                            (32768 - 28624) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11358) as uint16_t,
                            (32768 - 20778) as uint16_t,
                            (32768 - 25511) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10995) as uint16_t,
                            (32768 - 18073) as uint16_t,
                            (32768 - 24190) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9162) as uint16_t,
                            (32768 - 14990) as uint16_t,
                            (32768 - 20617) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 21425) as uint16_t,
                            (32768 - 27952) as uint16_t,
                            (32768 - 30388) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18062) as uint16_t,
                            (32768 - 25838) as uint16_t,
                            (32768 - 29034) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11956) as uint16_t,
                            (32768 - 19881) as uint16_t,
                            (32768 - 24808) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7718) as uint16_t,
                            (32768 - 15000) as uint16_t,
                            (32768 - 20980) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5702) as uint16_t,
                            (32768 - 11254) as uint16_t,
                            (32768 - 16143) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4898) as uint16_t,
                            (32768 - 9088) as uint16_t,
                            (32768 - 16864) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3679) as uint16_t,
                            (32768 - 6776) as uint16_t,
                            (32768 - 11907) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23294) as uint16_t,
                            (32768 - 30160) as uint16_t,
                            (32768 - 31663) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24397) as uint16_t,
                            (32768 - 29896) as uint16_t,
                            (32768 - 31836) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19245) as uint16_t,
                            (32768 - 27128) as uint16_t,
                            (32768 - 30593) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13202) as uint16_t,
                            (32768 - 19825) as uint16_t,
                            (32768 - 26404) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11578) as uint16_t,
                            (32768 - 19297) as uint16_t,
                            (32768 - 23957) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8073) as uint16_t,
                            (32768 - 13297) as uint16_t,
                            (32768 - 21370) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5461) as uint16_t,
                            (32768 - 10923) as uint16_t,
                            (32768 - 19745) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27367) as uint16_t,
                            (32768 - 30521) as uint16_t,
                            (32768 - 31934) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24904) as uint16_t,
                            (32768 - 30671) as uint16_t,
                            (32768 - 31940) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23075) as uint16_t,
                            (32768 - 28460) as uint16_t,
                            (32768 - 31299) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14400) as uint16_t,
                            (32768 - 23658) as uint16_t,
                            (32768 - 30417) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13885) as uint16_t,
                            (32768 - 23882) as uint16_t,
                            (32768 - 28325) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14746) as uint16_t,
                            (32768 - 22938) as uint16_t,
                            (32768 - 27853) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5461) as uint16_t,
                            (32768 - 16384) as uint16_t,
                            (32768 - 27307) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 18274) as uint16_t,
                            (32768 - 24813) as uint16_t,
                            (32768 - 27890) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15537) as uint16_t,
                            (32768 - 23149) as uint16_t,
                            (32768 - 27003) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9449) as uint16_t,
                            (32768 - 16740) as uint16_t,
                            (32768 - 21827) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6700) as uint16_t,
                            (32768 - 12498) as uint16_t,
                            (32768 - 17261) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4988) as uint16_t,
                            (32768 - 9866) as uint16_t,
                            (32768 - 14198) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4236) as uint16_t,
                            (32768 - 8147) as uint16_t,
                            (32768 - 11902) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2867) as uint16_t,
                            (32768 - 5860) as uint16_t,
                            (32768 - 8654) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17124) as uint16_t,
                            (32768 - 23171) as uint16_t,
                            (32768 - 26101) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20396) as uint16_t,
                            (32768 - 27477) as uint16_t,
                            (32768 - 30148) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16573) as uint16_t,
                            (32768 - 24629) as uint16_t,
                            (32768 - 28492) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12749) as uint16_t,
                            (32768 - 20846) as uint16_t,
                            (32768 - 25674) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10233) as uint16_t,
                            (32768 - 17878) as uint16_t,
                            (32768 - 22818) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8525) as uint16_t,
                            (32768 - 15332) as uint16_t,
                            (32768 - 20363) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6283) as uint16_t,
                            (32768 - 11632) as uint16_t,
                            (32768 - 16255) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20466) as uint16_t,
                            (32768 - 26511) as uint16_t,
                            (32768 - 29286) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23059) as uint16_t,
                            (32768 - 29174) as uint16_t,
                            (32768 - 31191) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19481) as uint16_t,
                            (32768 - 27263) as uint16_t,
                            (32768 - 30241) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15458) as uint16_t,
                            (32768 - 23631) as uint16_t,
                            (32768 - 28137) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12416) as uint16_t,
                            (32768 - 20608) as uint16_t,
                            (32768 - 25693) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10261) as uint16_t,
                            (32768 - 18011) as uint16_t,
                            (32768 - 23261) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8016) as uint16_t,
                            (32768 - 14655) as uint16_t,
                            (32768 - 19666) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 17616) as uint16_t,
                            (32768 - 24586) as uint16_t,
                            (32768 - 28112) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15809) as uint16_t,
                            (32768 - 23299) as uint16_t,
                            (32768 - 27155) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10767) as uint16_t,
                            (32768 - 18890) as uint16_t,
                            (32768 - 23793) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7727) as uint16_t,
                            (32768 - 14255) as uint16_t,
                            (32768 - 18865) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6129) as uint16_t,
                            (32768 - 11926) as uint16_t,
                            (32768 - 16882) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4482) as uint16_t,
                            (32768 - 9704) as uint16_t,
                            (32768 - 14861) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3277) as uint16_t,
                            (32768 - 7452) as uint16_t,
                            (32768 - 11522) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22956) as uint16_t,
                            (32768 - 28551) as uint16_t,
                            (32768 - 30730) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22724) as uint16_t,
                            (32768 - 28937) as uint16_t,
                            (32768 - 30961) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18467) as uint16_t,
                            (32768 - 26324) as uint16_t,
                            (32768 - 29580) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13234) as uint16_t,
                            (32768 - 20713) as uint16_t,
                            (32768 - 25649) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11181) as uint16_t,
                            (32768 - 17592) as uint16_t,
                            (32768 - 22481) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8291) as uint16_t,
                            (32768 - 18358) as uint16_t,
                            (32768 - 24576) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7568) as uint16_t,
                            (32768 - 11881) as uint16_t,
                            (32768 - 14984) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24948) as uint16_t,
                            (32768 - 29001) as uint16_t,
                            (32768 - 31147) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 25674) as uint16_t,
                            (32768 - 30619) as uint16_t,
                            (32768 - 32151) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 20841) as uint16_t,
                            (32768 - 26793) as uint16_t,
                            (32768 - 29603) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14669) as uint16_t,
                            (32768 - 24356) as uint16_t,
                            (32768 - 28666) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11334) as uint16_t,
                            (32768 - 23593) as uint16_t,
                            (32768 - 28219) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8922) as uint16_t,
                            (32768 - 14762) as uint16_t,
                            (32768 - 22873) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8301) as uint16_t,
                            (32768 - 13544) as uint16_t,
                            (32768 - 20535) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 17113) as uint16_t,
                            (32768 - 23733) as uint16_t,
                            (32768 - 27081) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14139) as uint16_t,
                            (32768 - 21406) as uint16_t,
                            (32768 - 25452) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8552) as uint16_t,
                            (32768 - 15002) as uint16_t,
                            (32768 - 19776) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5871) as uint16_t,
                            (32768 - 11120) as uint16_t,
                            (32768 - 15378) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4455) as uint16_t,
                            (32768 - 8616) as uint16_t,
                            (32768 - 12253) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3469) as uint16_t,
                            (32768 - 6910) as uint16_t,
                            (32768 - 10386) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2255) as uint16_t,
                            (32768 - 4553) as uint16_t,
                            (32768 - 6782) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18224) as uint16_t,
                            (32768 - 24376) as uint16_t,
                            (32768 - 27053) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19290) as uint16_t,
                            (32768 - 26710) as uint16_t,
                            (32768 - 29614) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14936) as uint16_t,
                            (32768 - 22991) as uint16_t,
                            (32768 - 27184) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11238) as uint16_t,
                            (32768 - 18951) as uint16_t,
                            (32768 - 23762) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8786) as uint16_t,
                            (32768 - 15617) as uint16_t,
                            (32768 - 20588) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7317) as uint16_t,
                            (32768 - 13228) as uint16_t,
                            (32768 - 18003) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5101) as uint16_t,
                            (32768 - 9512) as uint16_t,
                            (32768 - 13493) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22639) as uint16_t,
                            (32768 - 28222) as uint16_t,
                            (32768 - 30210) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23216) as uint16_t,
                            (32768 - 29331) as uint16_t,
                            (32768 - 31307) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19075) as uint16_t,
                            (32768 - 26762) as uint16_t,
                            (32768 - 29895) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15014) as uint16_t,
                            (32768 - 23113) as uint16_t,
                            (32768 - 27457) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11938) as uint16_t,
                            (32768 - 19857) as uint16_t,
                            (32768 - 24752) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9942) as uint16_t,
                            (32768 - 17280) as uint16_t,
                            (32768 - 22282) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7167) as uint16_t,
                            (32768 - 13144) as uint16_t,
                            (32768 - 17752) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 15820) as uint16_t,
                            (32768 - 22738) as uint16_t,
                            (32768 - 26488) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 13530) as uint16_t,
                            (32768 - 20885) as uint16_t,
                            (32768 - 25216) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8395) as uint16_t,
                            (32768 - 15530) as uint16_t,
                            (32768 - 20452) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6574) as uint16_t,
                            (32768 - 12321) as uint16_t,
                            (32768 - 16380) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5353) as uint16_t,
                            (32768 - 10419) as uint16_t,
                            (32768 - 14568) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4613) as uint16_t,
                            (32768 - 8446) as uint16_t,
                            (32768 - 12381) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3440) as uint16_t,
                            (32768 - 7158) as uint16_t,
                            (32768 - 9903) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24247) as uint16_t,
                            (32768 - 29051) as uint16_t,
                            (32768 - 31224) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22118) as uint16_t,
                            (32768 - 28058) as uint16_t,
                            (32768 - 30369) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16498) as uint16_t,
                            (32768 - 24768) as uint16_t,
                            (32768 - 28389) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12920) as uint16_t,
                            (32768 - 21175) as uint16_t,
                            (32768 - 26137) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10730) as uint16_t,
                            (32768 - 18619) as uint16_t,
                            (32768 - 25352) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 10187) as uint16_t,
                            (32768 - 16279) as uint16_t,
                            (32768 - 22791) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9310) as uint16_t,
                            (32768 - 14631) as uint16_t,
                            (32768 - 22127) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24970) as uint16_t,
                            (32768 - 30558) as uint16_t,
                            (32768 - 32057) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24801) as uint16_t,
                            (32768 - 29942) as uint16_t,
                            (32768 - 31698) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 22432) as uint16_t,
                            (32768 - 28453) as uint16_t,
                            (32768 - 30855) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 19054) as uint16_t,
                            (32768 - 25680) as uint16_t,
                            (32768 - 29580) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14392) as uint16_t,
                            (32768 - 23036) as uint16_t,
                            (32768 - 28109) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12495) as uint16_t,
                            (32768 - 20947) as uint16_t,
                            (32768 - 26650) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12442) as uint16_t,
                            (32768 - 20326) as uint16_t,
                            (32768 - 26214) as uint16_t,
                            0,
                        ],
                    ],
                ],
                [
                    [
                        [
                            (32768 - 12162) as uint16_t,
                            (32768 - 18785) as uint16_t,
                            (32768 - 22648) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12749) as uint16_t,
                            (32768 - 19697) as uint16_t,
                            (32768 - 23806) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 8580) as uint16_t,
                            (32768 - 15297) as uint16_t,
                            (32768 - 20346) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6169) as uint16_t,
                            (32768 - 11749) as uint16_t,
                            (32768 - 16543) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4836) as uint16_t,
                            (32768 - 9391) as uint16_t,
                            (32768 - 13448) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3821) as uint16_t,
                            (32768 - 7711) as uint16_t,
                            (32768 - 11613) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 2228) as uint16_t,
                            (32768 - 4601) as uint16_t,
                            (32768 - 7070) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16319) as uint16_t,
                            (32768 - 24725) as uint16_t,
                            (32768 - 28280) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 15698) as uint16_t,
                            (32768 - 23277) as uint16_t,
                            (32768 - 27168) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12726) as uint16_t,
                            (32768 - 20368) as uint16_t,
                            (32768 - 25047) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9912) as uint16_t,
                            (32768 - 17015) as uint16_t,
                            (32768 - 21976) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7888) as uint16_t,
                            (32768 - 14220) as uint16_t,
                            (32768 - 19179) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6777) as uint16_t,
                            (32768 - 12284) as uint16_t,
                            (32768 - 17018) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 4492) as uint16_t,
                            (32768 - 8590) as uint16_t,
                            (32768 - 12252) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 23249) as uint16_t,
                            (32768 - 28904) as uint16_t,
                            (32768 - 30947) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21050) as uint16_t,
                            (32768 - 27908) as uint16_t,
                            (32768 - 30512) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 17440) as uint16_t,
                            (32768 - 25340) as uint16_t,
                            (32768 - 28949) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14059) as uint16_t,
                            (32768 - 22018) as uint16_t,
                            (32768 - 26541) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11288) as uint16_t,
                            (32768 - 18903) as uint16_t,
                            (32768 - 23898) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9411) as uint16_t,
                            (32768 - 16342) as uint16_t,
                            (32768 - 21428) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6278) as uint16_t,
                            (32768 - 11588) as uint16_t,
                            (32768 - 15944) as uint16_t,
                            0,
                        ],
                    ],
                    [
                        [
                            (32768 - 13981) as uint16_t,
                            (32768 - 20067) as uint16_t,
                            (32768 - 23226) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16922) as uint16_t,
                            (32768 - 23580) as uint16_t,
                            (32768 - 26783) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 11005) as uint16_t,
                            (32768 - 19039) as uint16_t,
                            (32768 - 24487) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 7389) as uint16_t,
                            (32768 - 14218) as uint16_t,
                            (32768 - 19798) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 5598) as uint16_t,
                            (32768 - 11505) as uint16_t,
                            (32768 - 17206) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6090) as uint16_t,
                            (32768 - 11213) as uint16_t,
                            (32768 - 15659) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 3820) as uint16_t,
                            (32768 - 7371) as uint16_t,
                            (32768 - 10119) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21082) as uint16_t,
                            (32768 - 26925) as uint16_t,
                            (32768 - 29675) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21262) as uint16_t,
                            (32768 - 28627) as uint16_t,
                            (32768 - 31128) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18392) as uint16_t,
                            (32768 - 26454) as uint16_t,
                            (32768 - 30437) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14870) as uint16_t,
                            (32768 - 22910) as uint16_t,
                            (32768 - 27096) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12620) as uint16_t,
                            (32768 - 19484) as uint16_t,
                            (32768 - 24908) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 9290) as uint16_t,
                            (32768 - 16553) as uint16_t,
                            (32768 - 22802) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 6668) as uint16_t,
                            (32768 - 14288) as uint16_t,
                            (32768 - 20004) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 27704) as uint16_t,
                            (32768 - 31055) as uint16_t,
                            (32768 - 31949) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 24709) as uint16_t,
                            (32768 - 29978) as uint16_t,
                            (32768 - 31788) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 21668) as uint16_t,
                            (32768 - 29264) as uint16_t,
                            (32768 - 31657) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 18295) as uint16_t,
                            (32768 - 26968) as uint16_t,
                            (32768 - 30074) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 16399) as uint16_t,
                            (32768 - 24422) as uint16_t,
                            (32768 - 29313) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 14347) as uint16_t,
                            (32768 - 23026) as uint16_t,
                            (32768 - 28104) as uint16_t,
                            0,
                        ],
                        [
                            (32768 - 12370) as uint16_t,
                            (32768 - 19806) as uint16_t,
                            (32768 - 24477) as uint16_t,
                            0,
                        ],
                    ],
                ],
            ].into(),
            eob_hi_bit: [
                [
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 20177) as uint16_t, 0],
                        [(32768 - 20789) as uint16_t, 0],
                        [(32768 - 20262) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 21416) as uint16_t, 0],
                        [(32768 - 20855) as uint16_t, 0],
                        [(32768 - 23410) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                ],
                [
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 20238) as uint16_t, 0],
                        [(32768 - 21057) as uint16_t, 0],
                        [(32768 - 19159) as uint16_t, 0],
                        [(32768 - 22337) as uint16_t, 0],
                        [(32768 - 20159) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 20125) as uint16_t, 0],
                        [(32768 - 20559) as uint16_t, 0],
                        [(32768 - 21707) as uint16_t, 0],
                        [(32768 - 22296) as uint16_t, 0],
                        [(32768 - 17333) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                ],
                [
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 19941) as uint16_t, 0],
                        [(32768 - 20527) as uint16_t, 0],
                        [(32768 - 21470) as uint16_t, 0],
                        [(32768 - 22487) as uint16_t, 0],
                        [(32768 - 19558) as uint16_t, 0],
                        [(32768 - 22354) as uint16_t, 0],
                        [(32768 - 20331) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 22752) as uint16_t, 0],
                        [(32768 - 25006) as uint16_t, 0],
                        [(32768 - 22075) as uint16_t, 0],
                        [(32768 - 21576) as uint16_t, 0],
                        [(32768 - 17740) as uint16_t, 0],
                        [(32768 - 21690) as uint16_t, 0],
                        [(32768 - 19211) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                ],
                [
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 21442) as uint16_t, 0],
                        [(32768 - 22358) as uint16_t, 0],
                        [(32768 - 18503) as uint16_t, 0],
                        [(32768 - 20291) as uint16_t, 0],
                        [(32768 - 19945) as uint16_t, 0],
                        [(32768 - 21294) as uint16_t, 0],
                        [(32768 - 21178) as uint16_t, 0],
                        [(32768 - 19400) as uint16_t, 0],
                        [(32768 - 10556) as uint16_t, 0],
                    ],
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 24648) as uint16_t, 0],
                        [(32768 - 24949) as uint16_t, 0],
                        [(32768 - 20708) as uint16_t, 0],
                        [(32768 - 23905) as uint16_t, 0],
                        [(32768 - 20501) as uint16_t, 0],
                        [(32768 - 9558) as uint16_t, 0],
                        [(32768 - 9423) as uint16_t, 0],
                        [(32768 - 30365) as uint16_t, 0],
                        [(32768 - 19253) as uint16_t, 0],
                    ],
                ],
                [
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 26064) as uint16_t, 0],
                        [(32768 - 22098) as uint16_t, 0],
                        [(32768 - 19613) as uint16_t, 0],
                        [(32768 - 20525) as uint16_t, 0],
                        [(32768 - 17595) as uint16_t, 0],
                        [(32768 - 16618) as uint16_t, 0],
                        [(32768 - 20497) as uint16_t, 0],
                        [(32768 - 18989) as uint16_t, 0],
                        [(32768 - 15513) as uint16_t, 0],
                    ],
                    [
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                        [(32768 - 16384) as uint16_t, 0],
                    ],
                ],
            ].into(),
            skip: [
                [
                    [(32768 - 26887) as uint16_t, 0],
                    [(32768 - 6729) as uint16_t, 0],
                    [(32768 - 10361) as uint16_t, 0],
                    [(32768 - 17442) as uint16_t, 0],
                    [(32768 - 15045) as uint16_t, 0],
                    [(32768 - 22478) as uint16_t, 0],
                    [(32768 - 29072) as uint16_t, 0],
                    [(32768 - 2713) as uint16_t, 0],
                    [(32768 - 11861) as uint16_t, 0],
                    [(32768 - 20773) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                ],
                [
                    [(32768 - 31903) as uint16_t, 0],
                    [(32768 - 2044) as uint16_t, 0],
                    [(32768 - 7528) as uint16_t, 0],
                    [(32768 - 14618) as uint16_t, 0],
                    [(32768 - 16182) as uint16_t, 0],
                    [(32768 - 24168) as uint16_t, 0],
                    [(32768 - 31037) as uint16_t, 0],
                    [(32768 - 2786) as uint16_t, 0],
                    [(32768 - 11194) as uint16_t, 0],
                    [(32768 - 20155) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                ],
                [
                    [(32768 - 32510) as uint16_t, 0],
                    [(32768 - 8430) as uint16_t, 0],
                    [(32768 - 17318) as uint16_t, 0],
                    [(32768 - 24154) as uint16_t, 0],
                    [(32768 - 23674) as uint16_t, 0],
                    [(32768 - 28789) as uint16_t, 0],
                    [(32768 - 32139) as uint16_t, 0],
                    [(32768 - 3440) as uint16_t, 0],
                    [(32768 - 13117) as uint16_t, 0],
                    [(32768 - 22702) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                ],
                [
                    [(32768 - 31671) as uint16_t, 0],
                    [(32768 - 2056) as uint16_t, 0],
                    [(32768 - 11746) as uint16_t, 0],
                    [(32768 - 16852) as uint16_t, 0],
                    [(32768 - 18635) as uint16_t, 0],
                    [(32768 - 24715) as uint16_t, 0],
                    [(32768 - 31484) as uint16_t, 0],
                    [(32768 - 4656) as uint16_t, 0],
                    [(32768 - 16074) as uint16_t, 0],
                    [(32768 - 24704) as uint16_t, 0],
                    [(32768 - 1806) as uint16_t, 0],
                    [(32768 - 14645) as uint16_t, 0],
                    [(32768 - 25336) as uint16_t, 0],
                ],
                [
                    [(32768 - 31539) as uint16_t, 0],
                    [(32768 - 8433) as uint16_t, 0],
                    [(32768 - 20576) as uint16_t, 0],
                    [(32768 - 27904) as uint16_t, 0],
                    [(32768 - 27852) as uint16_t, 0],
                    [(32768 - 30026) as uint16_t, 0],
                    [(32768 - 32441) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                    [(32768 - 16384) as uint16_t, 0],
                ],
            ].into(),
            dc_sign: [
                [
                    [(32768 - 16000) as uint16_t, 0],
                    [(32768 - 13056) as uint16_t, 0],
                    [(32768 - 18816) as uint16_t, 0],
                ],
                [
                    [(32768 - 15232) as uint16_t, 0],
                    [(32768 - 12928) as uint16_t, 0],
                    [(32768 - 17280) as uint16_t, 0],
                ],
            ].into(),
        };
        init
    },
]
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_cdf_thread_update(
    hdr: *const Dav1dFrameHeader,
    dst: *mut CdfContext,
    src: *const CdfContext,
) {
    let mut i = 0;
    while i < N_BS_SIZES as libc::c_int {
        (*dst)
            .m
            .use_filter_intra[i
            as usize][0 as libc::c_int
            as usize] = (*src).m.use_filter_intra[i as usize][0];
        (*dst)
            .m
            .use_filter_intra[i
            as usize][1] = 0 as libc::c_int as uint16_t;
        i += 1;
    }
    memcpy(
        ((*dst).m.filter_intra).0.as_mut_ptr() as *mut libc::c_void,
        ((*src).m.filter_intra).0.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
    );
    (*dst).m.filter_intra[4] = 0 as libc::c_int as uint16_t;
    let mut k = 0;
    while k < 2 {
        let mut j = 0;
        while j < N_INTRA_PRED_MODES as libc::c_int {
            memcpy(
                ((*dst).m.uv_mode[k as usize][j as usize]).as_mut_ptr()
                    as *mut libc::c_void,
                ((*src).m.uv_mode[k as usize][j as usize]).as_ptr()
                    as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
            );
            (*dst)
                .m
                .uv_mode[k
                as usize][j
                as usize][(N_UV_INTRA_PRED_MODES as libc::c_int - 1
                - (k == 0) as libc::c_int) as usize] = 0 as libc::c_int as uint16_t;
            j += 1;
        }
        k += 1;
    }
    let mut j_0 = 0;
    while j_0 < 8 {
        memcpy(
            ((*dst).m.angle_delta[j_0 as usize]).as_mut_ptr() as *mut libc::c_void,
            ((*src).m.angle_delta[j_0 as usize]).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
        );
        (*dst)
            .m
            .angle_delta[j_0
            as usize][6] = 0 as libc::c_int as uint16_t;
        j_0 += 1;
    }
    let mut k_0 = 0;
    while k_0 < N_TX_SIZES as libc::c_int - 1 {
        let mut j_1 = 0;
        while j_1 < 3 {
            memcpy(
                ((*dst).m.txsz[k_0 as usize][j_1 as usize]).as_mut_ptr()
                    as *mut libc::c_void,
                ((*src).m.txsz[k_0 as usize][j_1 as usize]).as_ptr()
                    as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
            );
            (*dst)
                .m
                .txsz[k_0
                as usize][j_1
                as usize][imin(k_0 + 1, 2 as libc::c_int)
                as usize] = 0 as libc::c_int as uint16_t;
            j_1 += 1;
        }
        k_0 += 1;
    }
    let mut k_1 = 0;
    while k_1 < 2 {
        let mut j_2 = 0;
        while j_2 < N_INTRA_PRED_MODES as libc::c_int {
            memcpy(
                ((*dst).m.txtp_intra1[k_1 as usize][j_2 as usize]).as_mut_ptr()
                    as *mut libc::c_void,
                ((*src).m.txtp_intra1[k_1 as usize][j_2 as usize]).as_ptr()
                    as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
            );
            (*dst)
                .m
                .txtp_intra1[k_1
                as usize][j_2
                as usize][6] = 0 as libc::c_int as uint16_t;
            j_2 += 1;
        }
        k_1 += 1;
    }
    let mut k_2 = 0;
    while k_2 < 3 {
        let mut j_3 = 0;
        while j_3 < N_INTRA_PRED_MODES as libc::c_int {
            memcpy(
                ((*dst).m.txtp_intra2[k_2 as usize][j_3 as usize]).as_mut_ptr()
                    as *mut libc::c_void,
                ((*src).m.txtp_intra2[k_2 as usize][j_3 as usize]).as_ptr()
                    as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
            );
            (*dst)
                .m
                .txtp_intra2[k_2
                as usize][j_3
                as usize][4] = 0 as libc::c_int as uint16_t;
            j_3 += 1;
        }
        k_2 += 1;
    }
    let mut i_0 = 0;
    while i_0 < 3 {
        (*dst)
            .m
            .skip[i_0
            as usize][0 as libc::c_int
            as usize] = (*src).m.skip[i_0 as usize][0];
        (*dst)
            .m
            .skip[i_0
            as usize][1] = 0 as libc::c_int as uint16_t;
        i_0 += 1;
    }
    let mut k_3 = 0;
    while k_3 < N_BL_LEVELS as libc::c_int {
        let mut j_4 = 0;
        while j_4 < 4 {
            memcpy(
                ((*dst).m.partition[k_3 as usize][j_4 as usize]).as_mut_ptr()
                    as *mut libc::c_void,
                ((*src).m.partition[k_3 as usize][j_4 as usize]).as_ptr()
                    as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
            );
            (*dst)
                .m
                .partition[k_3
                as usize][j_4
                as usize][dav1d_partition_type_count[k_3 as usize]
                as usize] = 0 as libc::c_int as uint16_t;
            j_4 += 1;
        }
        k_3 += 1;
    }
    let mut j_5 = 0;
    while j_5 < N_TX_SIZES as libc::c_int {
        let mut i_1 = 0;
        while i_1 < 13 {
            (*dst)
                .coef
                .skip[j_5
                as usize][i_1
                as usize][0 as libc::c_int
                as usize] = (*src)
                .coef
                .skip[j_5 as usize][i_1 as usize][0];
            (*dst)
                .coef
                .skip[j_5
                as usize][i_1
                as usize][1] = 0 as libc::c_int as uint16_t;
            i_1 += 1;
        }
        j_5 += 1;
    }
    let mut k_4 = 0;
    while k_4 < 2 {
        let mut j_6 = 0;
        while j_6 < 2 {
            memcpy(
                ((*dst).coef.eob_bin_16[k_4 as usize][j_6 as usize]).as_mut_ptr()
                    as *mut libc::c_void,
                ((*src).coef.eob_bin_16[k_4 as usize][j_6 as usize]).as_ptr()
                    as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
            );
            (*dst)
                .coef
                .eob_bin_16[k_4
                as usize][j_6
                as usize][4] = 0 as libc::c_int as uint16_t;
            j_6 += 1;
        }
        k_4 += 1;
    }
    let mut k_5 = 0;
    while k_5 < 2 {
        let mut j_7 = 0;
        while j_7 < 2 {
            memcpy(
                ((*dst).coef.eob_bin_32[k_5 as usize][j_7 as usize]).as_mut_ptr()
                    as *mut libc::c_void,
                ((*src).coef.eob_bin_32[k_5 as usize][j_7 as usize]).as_ptr()
                    as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
            );
            (*dst)
                .coef
                .eob_bin_32[k_5
                as usize][j_7
                as usize][5] = 0 as libc::c_int as uint16_t;
            j_7 += 1;
        }
        k_5 += 1;
    }
    let mut k_6 = 0;
    while k_6 < 2 {
        let mut j_8 = 0;
        while j_8 < 2 {
            memcpy(
                ((*dst).coef.eob_bin_64[k_6 as usize][j_8 as usize]).as_mut_ptr()
                    as *mut libc::c_void,
                ((*src).coef.eob_bin_64[k_6 as usize][j_8 as usize]).as_ptr()
                    as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
            );
            (*dst)
                .coef
                .eob_bin_64[k_6
                as usize][j_8
                as usize][6] = 0 as libc::c_int as uint16_t;
            j_8 += 1;
        }
        k_6 += 1;
    }
    let mut k_7 = 0;
    while k_7 < 2 {
        let mut j_9 = 0;
        while j_9 < 2 {
            memcpy(
                ((*dst).coef.eob_bin_128[k_7 as usize][j_9 as usize]).as_mut_ptr()
                    as *mut libc::c_void,
                ((*src).coef.eob_bin_128[k_7 as usize][j_9 as usize]).as_ptr()
                    as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
            );
            (*dst)
                .coef
                .eob_bin_128[k_7
                as usize][j_9
                as usize][7] = 0 as libc::c_int as uint16_t;
            j_9 += 1;
        }
        k_7 += 1;
    }
    let mut k_8 = 0;
    while k_8 < 2 {
        let mut j_10 = 0;
        while j_10 < 2 {
            memcpy(
                ((*dst).coef.eob_bin_256[k_8 as usize][j_10 as usize]).as_mut_ptr()
                    as *mut libc::c_void,
                ((*src).coef.eob_bin_256[k_8 as usize][j_10 as usize]).as_ptr()
                    as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
            );
            (*dst)
                .coef
                .eob_bin_256[k_8
                as usize][j_10
                as usize][8] = 0 as libc::c_int as uint16_t;
            j_10 += 1;
        }
        k_8 += 1;
    }
    let mut j_11 = 0;
    while j_11 < 2 {
        memcpy(
            ((*dst).coef.eob_bin_512[j_11 as usize]).as_mut_ptr() as *mut libc::c_void,
            ((*src).coef.eob_bin_512[j_11 as usize]).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
        );
        (*dst)
            .coef
            .eob_bin_512[j_11
            as usize][9] = 0 as libc::c_int as uint16_t;
        j_11 += 1;
    }
    let mut j_12 = 0;
    while j_12 < 2 {
        memcpy(
            ((*dst).coef.eob_bin_1024[j_12 as usize]).as_mut_ptr() as *mut libc::c_void,
            ((*src).coef.eob_bin_1024[j_12 as usize]).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
        );
        (*dst)
            .coef
            .eob_bin_1024[j_12
            as usize][10] = 0 as libc::c_int as uint16_t;
        j_12 += 1;
    }
    let mut k_9 = 0;
    while k_9 < N_TX_SIZES as libc::c_int {
        let mut j_13 = 0;
        while j_13 < 2 {
            let mut i_2 = 0;
            while i_2 < 11 {
                (*dst)
                    .coef
                    .eob_hi_bit[k_9
                    as usize][j_13
                    as usize][i_2
                    as usize][0 as libc::c_int
                    as usize] = (*src)
                    .coef
                    .eob_hi_bit[k_9
                    as usize][j_13 as usize][i_2 as usize][0];
                (*dst)
                    .coef
                    .eob_hi_bit[k_9
                    as usize][j_13
                    as usize][i_2
                    as usize][1] = 0 as libc::c_int as uint16_t;
                i_2 += 1;
            }
            j_13 += 1;
        }
        k_9 += 1;
    }
    let mut l = 0;
    while l < N_TX_SIZES as libc::c_int {
        let mut k_10 = 0;
        while k_10 < 2 {
            let mut j_14 = 0;
            while j_14 < 4 {
                memcpy(
                    ((*dst).coef.eob_base_tok[l as usize][k_10 as usize][j_14 as usize])
                        .as_mut_ptr() as *mut libc::c_void,
                    ((*src).coef.eob_base_tok[l as usize][k_10 as usize][j_14 as usize])
                        .as_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
                );
                (*dst)
                    .coef
                    .eob_base_tok[l
                    as usize][k_10
                    as usize][j_14
                    as usize][2] = 0 as libc::c_int as uint16_t;
                j_14 += 1;
            }
            k_10 += 1;
        }
        l += 1;
    }
    let mut l_0 = 0;
    while l_0 < N_TX_SIZES as libc::c_int {
        let mut k_11 = 0;
        while k_11 < 2 {
            let mut j_15 = 0;
            while j_15 < 41 {
                memcpy(
                    ((*dst).coef.base_tok[l_0 as usize][k_11 as usize][j_15 as usize])
                        .as_mut_ptr() as *mut libc::c_void,
                    ((*src).coef.base_tok[l_0 as usize][k_11 as usize][j_15 as usize])
                        .as_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
                );
                (*dst)
                    .coef
                    .base_tok[l_0
                    as usize][k_11
                    as usize][j_15
                    as usize][3] = 0 as libc::c_int as uint16_t;
                j_15 += 1;
            }
            k_11 += 1;
        }
        l_0 += 1;
    }
    let mut j_16 = 0;
    while j_16 < 2 {
        let mut i_3 = 0;
        while i_3 < 3 {
            (*dst)
                .coef
                .dc_sign[j_16
                as usize][i_3
                as usize][0 as libc::c_int
                as usize] = (*src)
                .coef
                .dc_sign[j_16 as usize][i_3 as usize][0];
            (*dst)
                .coef
                .dc_sign[j_16
                as usize][i_3
                as usize][1] = 0 as libc::c_int as uint16_t;
            i_3 += 1;
        }
        j_16 += 1;
    }
    let mut l_1 = 0;
    while l_1 < 4 {
        let mut k_12 = 0;
        while k_12 < 2 {
            let mut j_17 = 0;
            while j_17 < 21 {
                memcpy(
                    ((*dst).coef.br_tok[l_1 as usize][k_12 as usize][j_17 as usize])
                        .as_mut_ptr() as *mut libc::c_void,
                    ((*src).coef.br_tok[l_1 as usize][k_12 as usize][j_17 as usize])
                        .as_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
                );
                (*dst)
                    .coef
                    .br_tok[l_1
                    as usize][k_12
                    as usize][j_17
                    as usize][3] = 0 as libc::c_int as uint16_t;
                j_17 += 1;
            }
            k_12 += 1;
        }
        l_1 += 1;
    }
    let mut j_18 = 0;
    while j_18 < 3 {
        memcpy(
            ((*dst).m.seg_id[j_18 as usize]).as_mut_ptr() as *mut libc::c_void,
            ((*src).m.seg_id[j_18 as usize]).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
        );
        (*dst)
            .m
            .seg_id[j_18
            as usize][(8 - 1)
            as usize] = 0 as libc::c_int as uint16_t;
        j_18 += 1;
    }
    memcpy(
        ((*dst).m.cfl_sign).0.as_mut_ptr() as *mut libc::c_void,
        ((*src).m.cfl_sign).0.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
    );
    (*dst).m.cfl_sign[7] = 0 as libc::c_int as uint16_t;
    let mut j_19 = 0;
    while j_19 < 6 {
        memcpy(
            ((*dst).m.cfl_alpha[j_19 as usize]).as_mut_ptr() as *mut libc::c_void,
            ((*src).m.cfl_alpha[j_19 as usize]).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
        );
        (*dst)
            .m
            .cfl_alpha[j_19
            as usize][15] = 0 as libc::c_int as uint16_t;
        j_19 += 1;
    }
    (*dst)
        .m
        .restore_wiener[0 as libc::c_int
        as usize] = (*src).m.restore_wiener[0];
    (*dst).m.restore_wiener[1] = 0 as libc::c_int as uint16_t;
    (*dst)
        .m
        .restore_sgrproj[0 as libc::c_int
        as usize] = (*src).m.restore_sgrproj[0];
    (*dst).m.restore_sgrproj[1] = 0 as libc::c_int as uint16_t;
    memcpy(
        ((*dst).m.restore_switchable).0.as_mut_ptr() as *mut libc::c_void,
        ((*src).m.restore_switchable).0.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
    );
    (*dst)
        .m
        .restore_switchable[2] = 0 as libc::c_int as uint16_t;
    memcpy(
        ((*dst).m.delta_q).0.as_mut_ptr() as *mut libc::c_void,
        ((*src).m.delta_q).0.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
    );
    (*dst).m.delta_q[3] = 0 as libc::c_int as uint16_t;
    let mut j_20 = 0;
    while j_20 < 5 {
        memcpy(
            ((*dst).m.delta_lf[j_20 as usize]).as_mut_ptr() as *mut libc::c_void,
            ((*src).m.delta_lf[j_20 as usize]).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
        );
        (*dst)
            .m
            .delta_lf[j_20
            as usize][3] = 0 as libc::c_int as uint16_t;
        j_20 += 1;
    }
    let mut j_21 = 0;
    while j_21 < 7 {
        let mut i_4 = 0;
        while i_4 < 3 {
            (*dst)
                .m
                .pal_y[j_21
                as usize][i_4
                as usize][0 as libc::c_int
                as usize] = (*src)
                .m
                .pal_y[j_21 as usize][i_4 as usize][0];
            (*dst)
                .m
                .pal_y[j_21
                as usize][i_4
                as usize][1] = 0 as libc::c_int as uint16_t;
            i_4 += 1;
        }
        j_21 += 1;
    }
    let mut i_5 = 0;
    while i_5 < 2 {
        (*dst)
            .m
            .pal_uv[i_5
            as usize][0 as libc::c_int
            as usize] = (*src).m.pal_uv[i_5 as usize][0];
        (*dst)
            .m
            .pal_uv[i_5
            as usize][1] = 0 as libc::c_int as uint16_t;
        i_5 += 1;
    }
    let mut k_13 = 0;
    while k_13 < 2 {
        let mut j_22 = 0;
        while j_22 < 7 {
            memcpy(
                ((*dst).m.pal_sz[k_13 as usize][j_22 as usize]).as_mut_ptr()
                    as *mut libc::c_void,
                ((*src).m.pal_sz[k_13 as usize][j_22 as usize]).as_ptr()
                    as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
            );
            (*dst)
                .m
                .pal_sz[k_13
                as usize][j_22
                as usize][6] = 0 as libc::c_int as uint16_t;
            j_22 += 1;
        }
        k_13 += 1;
    }
    let mut l_2 = 0;
    while l_2 < 2 {
        let mut k_14 = 0;
        while k_14 < 7 {
            let mut j_23 = 0;
            while j_23 < 5 {
                memcpy(
                    ((*dst).m.color_map[l_2 as usize][k_14 as usize][j_23 as usize])
                        .as_mut_ptr() as *mut libc::c_void,
                    ((*src).m.color_map[l_2 as usize][k_14 as usize][j_23 as usize])
                        .as_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
                );
                (*dst)
                    .m
                    .color_map[l_2
                    as usize][k_14
                    as usize][j_23
                    as usize][(k_14 + 1)
                    as usize] = 0 as libc::c_int as uint16_t;
                j_23 += 1;
            }
            k_14 += 1;
        }
        l_2 += 1;
    }
    let mut j_24 = 0;
    while j_24 < 7 {
        let mut i_6 = 0;
        while i_6 < 3 {
            (*dst)
                .m
                .txpart[j_24
                as usize][i_6
                as usize][0 as libc::c_int
                as usize] = (*src)
                .m
                .txpart[j_24 as usize][i_6 as usize][0];
            (*dst)
                .m
                .txpart[j_24
                as usize][i_6
                as usize][1] = 0 as libc::c_int as uint16_t;
            i_6 += 1;
        }
        j_24 += 1;
    }
    let mut j_25 = 0;
    while j_25 < 2 {
        memcpy(
            ((*dst).m.txtp_inter1[j_25 as usize]).as_mut_ptr() as *mut libc::c_void,
            ((*src).m.txtp_inter1[j_25 as usize]).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
        );
        (*dst)
            .m
            .txtp_inter1[j_25
            as usize][15] = 0 as libc::c_int as uint16_t;
        j_25 += 1;
    }
    memcpy(
        ((*dst).m.txtp_inter2.0).as_mut_ptr() as *mut libc::c_void,
        ((*src).m.txtp_inter2.0).as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
    );
    (*dst).m.txtp_inter2[11] = 0 as libc::c_int as uint16_t;
    let mut i_7 = 0;
    while i_7 < 4 {
        (*dst)
            .m
            .txtp_inter3[i_7
            as usize][0 as libc::c_int
            as usize] = (*src).m.txtp_inter3[i_7 as usize][0];
        (*dst)
            .m
            .txtp_inter3[i_7
            as usize][1] = 0 as libc::c_int as uint16_t;
        i_7 += 1;
    }
    if (*hdr).frame_type as libc::c_uint & 1 as libc::c_uint == 0 {
        (*dst)
            .m
            .intrabc[0 as libc::c_int
            as usize] = (*src).m.intrabc[0];
        (*dst).m.intrabc[1] = 0 as libc::c_int as uint16_t;
        memcpy(
            ((*dst).dmv.joint.0).as_mut_ptr() as *mut libc::c_void,
            ((*src).dmv.joint.0).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
        );
        (*dst)
            .dmv
            .joint[(N_MV_JOINTS as libc::c_int - 1)
            as usize] = 0 as libc::c_int as uint16_t;
        let mut k_15 = 0;
        while k_15 < 2 {
            memcpy(
                ((*dst).dmv.comp[k_15 as usize].classes.0).as_mut_ptr()
                    as *mut libc::c_void,
                ((*src).dmv.comp[k_15 as usize].classes.0).as_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
            );
            (*dst)
                .dmv
                .comp[k_15 as usize]
                .classes[10] = 0 as libc::c_int as uint16_t;
            (*dst)
                .dmv
                .comp[k_15 as usize]
                .class0[0 as libc::c_int
                as usize] = (*src)
                .dmv
                .comp[k_15 as usize]
                .class0[0];
            (*dst)
                .dmv
                .comp[k_15 as usize]
                .class0[1] = 0 as libc::c_int as uint16_t;
            let mut i_8 = 0;
            while i_8 < 10 {
                (*dst)
                    .dmv
                    .comp[k_15 as usize]
                    .classN[i_8
                    as usize][0 as libc::c_int
                    as usize] = (*src)
                    .dmv
                    .comp[k_15 as usize]
                    .classN[i_8 as usize][0];
                (*dst)
                    .dmv
                    .comp[k_15 as usize]
                    .classN[i_8
                    as usize][1] = 0 as libc::c_int as uint16_t;
                i_8 += 1;
            }
            (*dst)
                .dmv
                .comp[k_15 as usize]
                .sign[0 as libc::c_int
                as usize] = (*src)
                .dmv
                .comp[k_15 as usize]
                .sign[0];
            (*dst)
                .dmv
                .comp[k_15 as usize]
                .sign[1] = 0 as libc::c_int as uint16_t;
            k_15 += 1;
        }
        return;
    }
    let mut i_9 = 0;
    while i_9 < 3 {
        (*dst)
            .m
            .skip_mode.0[i_9
            as usize][0 as libc::c_int
            as usize] = (*src).m.skip_mode.0[i_9 as usize][0];
        (*dst)
            .m
            .skip_mode.0[i_9
            as usize][1] = 0 as libc::c_int as uint16_t;
        i_9 += 1;
    }
    let mut j_26 = 0;
    while j_26 < 4 {
        memcpy(
            ((*dst).m.y_mode.0[j_26 as usize]).as_mut_ptr() as *mut libc::c_void,
            ((*src).m.y_mode.0[j_26 as usize]).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
        );
        (*dst)
            .m
            .y_mode.0[j_26
            as usize][(N_INTRA_PRED_MODES as libc::c_int - 1)
            as usize] = 0 as libc::c_int as uint16_t;
        j_26 += 1;
    }
    let mut k_16 = 0;
    while k_16 < 2 {
        let mut j_27 = 0;
        while j_27 < 8 {
            memcpy(
                ((*dst).m.filter.0[k_16 as usize][j_27 as usize]).as_mut_ptr()
                    as *mut libc::c_void,
                ((*src).m.filter.0[k_16 as usize][j_27 as usize]).as_ptr()
                    as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
            );
            (*dst)
                .m
                .filter.0[k_16
                as usize][j_27
                as usize][(DAV1D_N_SWITCHABLE_FILTERS as libc::c_int - 1)
                as usize] = 0 as libc::c_int as uint16_t;
            j_27 += 1;
        }
        k_16 += 1;
    }
    let mut i_10 = 0;
    while i_10 < 6 {
        (*dst)
            .m
            .newmv_mode.0[i_10
            as usize][0 as libc::c_int
            as usize] = (*src).m.newmv_mode.0[i_10 as usize][0];
        (*dst)
            .m
            .newmv_mode.0[i_10
            as usize][1] = 0 as libc::c_int as uint16_t;
        i_10 += 1;
    }
    let mut i_11 = 0;
    while i_11 < 2 {
        (*dst)
            .m
            .globalmv_mode.0[i_11
            as usize][0 as libc::c_int
            as usize] = (*src).m.globalmv_mode.0[i_11 as usize][0];
        (*dst)
            .m
            .globalmv_mode.0[i_11
            as usize][1] = 0 as libc::c_int as uint16_t;
        i_11 += 1;
    }
    let mut i_12 = 0;
    while i_12 < 6 {
        (*dst)
            .m
            .refmv_mode.0[i_12
            as usize][0 as libc::c_int
            as usize] = (*src).m.refmv_mode.0[i_12 as usize][0];
        (*dst)
            .m
            .refmv_mode.0[i_12
            as usize][1] = 0 as libc::c_int as uint16_t;
        i_12 += 1;
    }
    let mut i_13 = 0;
    while i_13 < 3 {
        (*dst)
            .m
            .drl_bit.0[i_13
            as usize][0 as libc::c_int
            as usize] = (*src).m.drl_bit.0[i_13 as usize][0];
        (*dst)
            .m
            .drl_bit.0[i_13
            as usize][1] = 0 as libc::c_int as uint16_t;
        i_13 += 1;
    }
    let mut j_28 = 0;
    while j_28 < 8 {
        memcpy(
            ((*dst).m.comp_inter_mode.0[j_28 as usize]).as_mut_ptr() as *mut libc::c_void,
            ((*src).m.comp_inter_mode.0[j_28 as usize]).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
        );
        (*dst)
            .m
            .comp_inter_mode.0[j_28
            as usize][(N_COMP_INTER_PRED_MODES as libc::c_int - 1)
            as usize] = 0 as libc::c_int as uint16_t;
        j_28 += 1;
    }
    let mut i_14 = 0;
    while i_14 < 4 {
        (*dst)
            .m
            .intra.0[i_14
            as usize][0 as libc::c_int
            as usize] = (*src).m.intra.0[i_14 as usize][0];
        (*dst)
            .m
            .intra.0[i_14
            as usize][1] = 0 as libc::c_int as uint16_t;
        i_14 += 1;
    }
    let mut i_15 = 0;
    while i_15 < 5 {
        (*dst)
            .m
            .comp.0[i_15
            as usize][0 as libc::c_int
            as usize] = (*src).m.comp.0[i_15 as usize][0];
        (*dst)
            .m
            .comp.0[i_15
            as usize][1] = 0 as libc::c_int as uint16_t;
        i_15 += 1;
    }
    let mut i_16 = 0;
    while i_16 < 5 {
        (*dst)
            .m
            .comp_dir.0[i_16
            as usize][0 as libc::c_int
            as usize] = (*src).m.comp_dir.0[i_16 as usize][0];
        (*dst)
            .m
            .comp_dir.0[i_16
            as usize][1] = 0 as libc::c_int as uint16_t;
        i_16 += 1;
    }
    let mut i_17 = 0;
    while i_17 < 6 {
        (*dst)
            .m
            .jnt_comp.0[i_17
            as usize][0 as libc::c_int
            as usize] = (*src).m.jnt_comp.0[i_17 as usize][0];
        (*dst)
            .m
            .jnt_comp.0[i_17
            as usize][1] = 0 as libc::c_int as uint16_t;
        i_17 += 1;
    }
    let mut i_18 = 0;
    while i_18 < 6 {
        (*dst)
            .m
            .mask_comp.0[i_18
            as usize][0 as libc::c_int
            as usize] = (*src).m.mask_comp.0[i_18 as usize][0];
        (*dst)
            .m
            .mask_comp.0[i_18
            as usize][1] = 0 as libc::c_int as uint16_t;
        i_18 += 1;
    }
    let mut i_19 = 0;
    while i_19 < 9 {
        (*dst)
            .m
            .wedge_comp.0[i_19
            as usize][0 as libc::c_int
            as usize] = (*src).m.wedge_comp.0[i_19 as usize][0];
        (*dst)
            .m
            .wedge_comp.0[i_19
            as usize][1] = 0 as libc::c_int as uint16_t;
        i_19 += 1;
    }
    let mut j_29 = 0;
    while j_29 < 9 {
        memcpy(
            ((*dst).m.wedge_idx.0[j_29 as usize]).as_mut_ptr() as *mut libc::c_void,
            ((*src).m.wedge_idx.0[j_29 as usize]).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
        );
        (*dst)
            .m
            .wedge_idx[j_29
            as usize][15] = 0 as libc::c_int as uint16_t;
        j_29 += 1;
    }
    let mut j_30 = 0;
    while j_30 < 6 {
        let mut i_20 = 0;
        while i_20 < 3 {
            (*dst)
                .m
                .r#ref[j_30
                as usize][i_20
                as usize][0 as libc::c_int
                as usize] = (*src)
                .m
                .r#ref[j_30 as usize][i_20 as usize][0];
            (*dst)
                .m
                .r#ref[j_30
                as usize][i_20
                as usize][1] = 0 as libc::c_int as uint16_t;
            i_20 += 1;
        }
        j_30 += 1;
    }
    let mut j_31 = 0;
    while j_31 < 3 {
        let mut i_21 = 0;
        while i_21 < 3 {
            (*dst)
                .m
                .comp_fwd_ref[j_31
                as usize][i_21
                as usize][0 as libc::c_int
                as usize] = (*src)
                .m
                .comp_fwd_ref[j_31 as usize][i_21 as usize][0];
            (*dst)
                .m
                .comp_fwd_ref[j_31
                as usize][i_21
                as usize][1] = 0 as libc::c_int as uint16_t;
            i_21 += 1;
        }
        j_31 += 1;
    }
    let mut j_32 = 0;
    while j_32 < 2 {
        let mut i_22 = 0;
        while i_22 < 3 {
            (*dst)
                .m
                .comp_bwd_ref[j_32
                as usize][i_22
                as usize][0 as libc::c_int
                as usize] = (*src)
                .m
                .comp_bwd_ref[j_32 as usize][i_22 as usize][0];
            (*dst)
                .m
                .comp_bwd_ref[j_32
                as usize][i_22
                as usize][1] = 0 as libc::c_int as uint16_t;
            i_22 += 1;
        }
        j_32 += 1;
    }
    let mut j_33 = 0;
    while j_33 < 3 {
        let mut i_23 = 0;
        while i_23 < 3 {
            (*dst)
                .m
                .comp_uni_ref[j_33
                as usize][i_23
                as usize][0 as libc::c_int
                as usize] = (*src)
                .m
                .comp_uni_ref[j_33 as usize][i_23 as usize][0];
            (*dst)
                .m
                .comp_uni_ref[j_33
                as usize][i_23
                as usize][1] = 0 as libc::c_int as uint16_t;
            i_23 += 1;
        }
        j_33 += 1;
    }
    let mut i_24 = 0;
    while i_24 < 3 {
        (*dst)
            .m
            .seg_pred[i_24
            as usize][0 as libc::c_int
            as usize] = (*src).m.seg_pred[i_24 as usize][0];
        (*dst)
            .m
            .seg_pred[i_24
            as usize][1] = 0 as libc::c_int as uint16_t;
        i_24 += 1;
    }
    let mut i_25 = 0;
    while i_25 < 4 {
        (*dst)
            .m
            .interintra[i_25
            as usize][0 as libc::c_int
            as usize] = (*src).m.interintra[i_25 as usize][0];
        (*dst)
            .m
            .interintra[i_25
            as usize][1] = 0 as libc::c_int as uint16_t;
        i_25 += 1;
    }
    let mut i_26 = 0;
    while i_26 < 7 {
        (*dst)
            .m
            .interintra_wedge[i_26
            as usize][0 as libc::c_int
            as usize] = (*src)
            .m
            .interintra_wedge[i_26 as usize][0];
        (*dst)
            .m
            .interintra_wedge[i_26
            as usize][1] = 0 as libc::c_int as uint16_t;
        i_26 += 1;
    }
    let mut j_34 = 0;
    while j_34 < 4 {
        memcpy(
            ((*dst).m.interintra_mode[j_34 as usize]).as_mut_ptr() as *mut libc::c_void,
            ((*src).m.interintra_mode[j_34 as usize]).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
        );
        (*dst)
            .m
            .interintra_mode[j_34
            as usize][3] = 0 as libc::c_int as uint16_t;
        j_34 += 1;
    }
    let mut j_35 = 0;
    while j_35 < N_BS_SIZES as libc::c_int {
        memcpy(
            ((*dst).m.motion_mode[j_35 as usize]).as_mut_ptr() as *mut libc::c_void,
            ((*src).m.motion_mode[j_35 as usize]).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
        );
        (*dst)
            .m
            .motion_mode[j_35
            as usize][2] = 0 as libc::c_int as uint16_t;
        j_35 += 1;
    }
    let mut i_27 = 0;
    while i_27 < N_BS_SIZES as libc::c_int {
        (*dst)
            .m
            .obmc[i_27
            as usize][0 as libc::c_int
            as usize] = (*src).m.obmc[i_27 as usize][0];
        (*dst)
            .m
            .obmc[i_27
            as usize][1] = 0 as libc::c_int as uint16_t;
        i_27 += 1;
    }
    memcpy(
        ((*dst).mv.joint.0).as_mut_ptr() as *mut libc::c_void,
        ((*src).mv.joint.0).as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
    );
    (*dst)
        .mv
        .joint[(N_MV_JOINTS as libc::c_int - 1)
        as usize] = 0 as libc::c_int as uint16_t;
    let mut k_17 = 0;
    while k_17 < 2 {
        memcpy(
            ((*dst).mv.comp[k_17 as usize].classes.0).as_mut_ptr() as *mut libc::c_void,
            ((*src).mv.comp[k_17 as usize].classes.0).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
        );
        (*dst)
            .mv
            .comp[k_17 as usize]
            .classes[10] = 0 as libc::c_int as uint16_t;
        (*dst)
            .mv
            .comp[k_17 as usize]
            .class0[0 as libc::c_int
            as usize] = (*src).mv.comp[k_17 as usize].class0[0];
        (*dst)
            .mv
            .comp[k_17 as usize]
            .class0[1] = 0 as libc::c_int as uint16_t;
        let mut i_28 = 0;
        while i_28 < 10 {
            (*dst)
                .mv
                .comp[k_17 as usize]
                .classN[i_28
                as usize][0 as libc::c_int
                as usize] = (*src)
                .mv
                .comp[k_17 as usize]
                .classN[i_28 as usize][0];
            (*dst)
                .mv
                .comp[k_17 as usize]
                .classN[i_28
                as usize][1] = 0 as libc::c_int as uint16_t;
            i_28 += 1;
        }
        let mut j_36 = 0;
        while j_36 < 2 {
            memcpy(
                ((*dst).mv.comp[k_17 as usize].class0_fp[j_36 as usize]).as_mut_ptr()
                    as *mut libc::c_void,
                ((*src).mv.comp[k_17 as usize].class0_fp[j_36 as usize]).as_ptr()
                    as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
            );
            (*dst)
                .mv
                .comp[k_17 as usize]
                .class0_fp[j_36
                as usize][3] = 0 as libc::c_int as uint16_t;
            j_36 += 1;
        }
        memcpy(
            ((*dst).mv.comp[k_17 as usize].classN_fp.0).as_mut_ptr() as *mut libc::c_void,
            ((*src).mv.comp[k_17 as usize].classN_fp.0).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
        );
        (*dst)
            .mv
            .comp[k_17 as usize]
            .classN_fp[3] = 0 as libc::c_int as uint16_t;
        (*dst)
            .mv
            .comp[k_17 as usize]
            .class0_hp[0 as libc::c_int
            as usize] = (*src)
            .mv
            .comp[k_17 as usize]
            .class0_hp[0];
        (*dst)
            .mv
            .comp[k_17 as usize]
            .class0_hp[1] = 0 as libc::c_int as uint16_t;
        (*dst)
            .mv
            .comp[k_17 as usize]
            .classN_hp[0 as libc::c_int
            as usize] = (*src)
            .mv
            .comp[k_17 as usize]
            .classN_hp[0];
        (*dst)
            .mv
            .comp[k_17 as usize]
            .classN_hp[1] = 0 as libc::c_int as uint16_t;
        (*dst)
            .mv
            .comp[k_17 as usize]
            .sign[0 as libc::c_int
            as usize] = (*src).mv.comp[k_17 as usize].sign[0];
        (*dst)
            .mv
            .comp[k_17 as usize]
            .sign[1] = 0 as libc::c_int as uint16_t;
        k_17 += 1;
    }
}
#[inline]
unsafe extern "C" fn get_qcat_idx(q: libc::c_int) -> libc::c_int {
    if q <= 20 {
        return 0 as libc::c_int;
    }
    if q <= 60 {
        return 1 as libc::c_int;
    }
    if q <= 120 {
        return 2 as libc::c_int;
    }
    return 3 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_cdf_thread_init_static(
    cdf: *mut CdfThreadContext,
    qidx: libc::c_int,
) {
    (*cdf).r#ref = 0 as *mut Dav1dRef;
    (*cdf).data.qcat = get_qcat_idx(qidx) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_cdf_thread_copy(
    dst: *mut CdfContext,
    src: *const CdfThreadContext,
) {
    if !((*src).r#ref).is_null() {
        memcpy(
            dst as *mut libc::c_void,
            (*src).data.cdf as *const libc::c_void,
            ::core::mem::size_of::<CdfContext>() as libc::c_ulong,
        );
    } else {
        (*dst).m = av1_default_cdf();
        memcpy(
            ((*dst).kfym.0).as_mut_ptr() as *mut libc::c_void,
            default_kf_y_mode_cdf.as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[[[uint16_t; 16]; 5]; 5]>() as libc::c_ulong,
        );
        (*dst).coef = av1_default_coef_cdf()[(*src).data.qcat as usize];
        memcpy(
            ((*dst).mv.joint.0).as_mut_ptr() as *mut libc::c_void,
            default_mv_joint_cdf.as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
        );
        memcpy(
            ((*dst).dmv.joint.0).as_mut_ptr() as *mut libc::c_void,
            default_mv_joint_cdf.as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
        );
        (*dst).dmv.comp[1] = default_mv_component_cdf();
        (*dst)
            .dmv
            .comp[0 as libc::c_int
            as usize] = (*dst).dmv.comp[1];
        (*dst)
            .mv
            .comp[1 as libc::c_int
            as usize] = (*dst).dmv.comp[0];
        (*dst)
            .mv
            .comp[0] = (*dst).mv.comp[1];
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_cdf_thread_alloc(
    c: *mut Dav1dContext,
    cdf: *mut CdfThreadContext,
    have_frame_mt: libc::c_int,
) -> libc::c_int {
    (*cdf)
        .r#ref = dav1d_ref_create_using_pool(
        (*c).cdf_pool,
        (::core::mem::size_of::<CdfContext>())
            .wrapping_add(::core::mem::size_of::<atomic_uint>()),
    );
    if ((*cdf).r#ref).is_null() {
        return -(12 as libc::c_int);
    }
    (*cdf).data.cdf = (*(*cdf).r#ref).data as *mut CdfContext;
    if have_frame_mt != 0 {
        (*cdf)
            .progress = &mut *((*cdf).data.cdf).offset(1)
            as *mut CdfContext as *mut atomic_uint;
        *(*cdf).progress = 0 as libc::c_int as libc::c_uint;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_cdf_thread_ref(
    dst: *mut CdfThreadContext,
    src: *mut CdfThreadContext,
) {
    *dst = *src;
    if !((*src).r#ref).is_null() {
        dav1d_ref_inc((*src).r#ref);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_cdf_thread_unref(cdf: *mut CdfThreadContext) {
    memset(
        &mut (*cdf).data as *mut CdfThreadContext_data as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<CdfThreadContext>() as libc::c_ulong)
            .wrapping_sub(8 as libc::c_ulong),
    );
    dav1d_ref_dec(&mut (*cdf).r#ref);
}
