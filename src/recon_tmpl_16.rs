use crate::include::common::bitdepth::BitDepth16;
use crate::include::stddef::*;
use crate::include::stdint::*;
use crate::src::ctx::CaseSet;
use ::libc;

extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::size_t) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::size_t) -> *mut libc::c_void;
    fn fprintf(_: *mut libc::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn dav1d_cdef_brow_16bpc(
        tc: *mut Dav1dTaskContext,
        p: *const *mut pixel,
        lflvl: *const Av1Filter,
        by_start: libc::c_int,
        by_end: libc::c_int,
        sbrow_start: libc::c_int,
        sby: libc::c_int,
    );
    fn dav1d_prepare_intra_edges_16bpc(
        x: libc::c_int,
        have_left: libc::c_int,
        y: libc::c_int,
        have_top: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        edge_flags: EdgeFlags,
        dst: *const pixel,
        stride: ptrdiff_t,
        prefilter_toplevel_sb_edge: *const pixel,
        mode: IntraPredMode,
        angle: *mut libc::c_int,
        tw: libc::c_int,
        th: libc::c_int,
        filter_edge: libc::c_int,
        topleft_out: *mut pixel,
        bitdepth_max: libc::c_int,
    ) -> IntraPredMode;
    fn dav1d_loopfilter_sbrow_cols_16bpc(
        f: *const Dav1dFrameContext,
        p: *const *mut pixel,
        lflvl: *mut Av1Filter,
        sby: libc::c_int,
        start_of_tile_row: libc::c_int,
    );
    fn dav1d_loopfilter_sbrow_rows_16bpc(
        f: *const Dav1dFrameContext,
        p: *const *mut pixel,
        lflvl: *mut Av1Filter,
        sby: libc::c_int,
    );
    fn dav1d_copy_lpf_16bpc(f: *mut Dav1dFrameContext, src: *const *mut pixel, sby: libc::c_int);
    fn dav1d_lr_sbrow_16bpc(f: *mut Dav1dFrameContext, dst: *const *mut pixel, sby: libc::c_int);
    static dav1d_scans: [*const uint16_t; 19];
    static mut dav1d_wedge_masks: [[[[*const uint8_t; 16]; 2]; 3]; 22];
    static mut dav1d_ii_masks: [[[*const uint8_t; 4]; 3]; 22];
}

use crate::src::msac::dav1d_msac_decode_bool_adapt;
use crate::src::msac::dav1d_msac_decode_bool_equi;
use crate::src::msac::dav1d_msac_decode_hi_tok;
use crate::src::msac::dav1d_msac_decode_symbol_adapt16;
use crate::src::msac::dav1d_msac_decode_symbol_adapt4;
use crate::src::msac::dav1d_msac_decode_symbol_adapt8;
use crate::src::tables::dav1d_block_dimensions;
use crate::src::tables::dav1d_filter_2d;
use crate::src::tables::dav1d_filter_mode_to_y_mode;
use crate::src::tables::dav1d_lo_ctx_offsets;

use crate::src::tables::dav1d_tx_type_class;
use crate::src::tables::dav1d_tx_types_per_set;
use crate::src::tables::dav1d_txfm_dimensions;
use crate::src::tables::dav1d_txtp_from_uvmode;

pub type pixel = uint16_t;
pub type coef = int32_t;
use crate::include::stdatomic::atomic_int;

use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::dav1d::data::Dav1dData;
use crate::src::r#ref::Dav1dRef;
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
use crate::include::dav1d::headers::Dav1dContentLightLevel;
use crate::include::dav1d::headers::Dav1dITUTT35;
use crate::include::dav1d::headers::Dav1dMasteringDisplay;
use crate::include::dav1d::picture::Dav1dPicture;
use crate::src::internal::Dav1dFrameContext_task_thread;
use crate::src::internal::FrameTileThreadData;
use crate::src::internal::TaskThreadData;

use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;

use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I400;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;

use crate::include::dav1d::headers::Dav1dWarpedMotionParams;

use crate::include::dav1d::headers::DAV1D_WM_TYPE_TRANSLATION;

use crate::include::dav1d::headers::Dav1dFilmGrainData;
use crate::include::dav1d::headers::Dav1dSequenceHeader;

use crate::src::internal::CodedBlockInfo;
use crate::src::internal::Dav1dFrameContext_frame_thread;
use crate::src::internal::Dav1dFrameContext_lf;
use crate::src::levels::mv;
use crate::src::levels::Av1Block;
use crate::src::lf_mask::Av1Filter;
use crate::src::lf_mask::Av1FilterLUT;

use crate::src::env::BlockContext;
use crate::src::refmvs::refmvs_block;
use crate::src::refmvs::refmvs_frame;
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

use crate::src::levels::FILTER_2D_BILINEAR;

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

use crate::include::dav1d::dav1d::Dav1dEventFlags;
use crate::include::dav1d::dav1d::Dav1dLogger;
use crate::src::picture::PictureFlags;

use crate::include::dav1d::dav1d::Dav1dDecodeFrameType;
use crate::include::dav1d::dav1d::Dav1dInloopFilterType;

use crate::include::dav1d::dav1d::DAV1D_INLOOPFILTER_CDEF;
use crate::include::dav1d::dav1d::DAV1D_INLOOPFILTER_DEBLOCK;
use crate::include::dav1d::dav1d::DAV1D_INLOOPFILTER_RESTORATION;

use crate::include::dav1d::picture::Dav1dPicAllocator;
use crate::src::internal::Dav1dContext_intra_edge;

use crate::src::intra_edge::EdgeFlags;
use crate::src::intra_edge::EDGE_I420_LEFT_HAS_BOTTOM;

use crate::src::intra_edge::EDGE_I420_TOP_HAS_RIGHT;
use crate::src::intra_edge::EDGE_I444_LEFT_HAS_BOTTOM;

use crate::src::intra_edge::EDGE_I444_TOP_HAS_RIGHT;
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
use crate::src::looprestoration::Dav1dLoopRestorationDSPContext;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dCdefDSPContext {
    pub dir: cdef_dir_fn,
    pub fb: [cdef_fn; 3],
}
pub type cdef_fn = Option<
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
pub type cdef_dir_fn = Option<
    unsafe extern "C" fn(*const pixel, ptrdiff_t, *mut libc::c_uint, libc::c_int) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dLoopFilterDSPContext {
    pub loop_filter_sb: [[loopfilter_sb_fn; 2]; 2],
}
pub type loopfilter_sb_fn = Option<
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
pub type itxfm_fn =
    Option<unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int, libc::c_int) -> ()>;
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
pub type emu_edge_fn = Option<
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
pub type warp8x8t_fn = Option<
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
pub type warp8x8_fn = Option<
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
pub type blend_dir_fn = Option<
    unsafe extern "C" fn(*mut pixel, ptrdiff_t, *const pixel, libc::c_int, libc::c_int) -> (),
>;
pub type blend_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        libc::c_int,
        libc::c_int,
        *const uint8_t,
    ) -> (),
>;
pub type w_mask_fn = Option<
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
pub type mask_fn = Option<
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
pub type w_avg_fn = Option<
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
pub type avg_fn = Option<
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
pub type mct_scaled_fn = Option<
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
pub type mct_fn = Option<
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
pub type mc_scaled_fn = Option<
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
pub type mc_fn = Option<
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
pub type pal_pred_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const uint16_t,
        *const uint8_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type cfl_pred_fn = Option<
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
pub type cfl_ac_fn = Option<
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
pub type angular_ipred_fn = Option<
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
pub type fguv_32x32xn_fn = Option<
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
pub type fgy_32x32xn_fn = Option<
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
pub type generate_grain_uv_fn = Option<
    unsafe extern "C" fn(
        *mut [entry; 82],
        *const [entry; 82],
        *const Dav1dFilmGrainData,
        intptr_t,
        libc::c_int,
    ) -> (),
>;
pub type generate_grain_y_fn =
    Option<unsafe extern "C" fn(*mut [entry; 82], *const Dav1dFilmGrainData, libc::c_int) -> ()>;
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
use crate::src::internal::ScalableMotionParams;
use crate::src::levels::IntraPredMode;
use crate::src::levels::RectTxfmSize;
use crate::src::levels::TxClass;
use crate::src::levels::TxfmType;
use crate::src::levels::CFL_PRED;
use crate::src::levels::COMP_INTER_NONE;
use crate::src::levels::DCT_DCT;
use crate::src::levels::DC_PRED;
use crate::src::levels::FILTER_PRED;
use crate::src::levels::GLOBALMV;
use crate::src::levels::GLOBALMV_GLOBALMV;
use crate::src::levels::IDTX;
use crate::src::levels::II_SMOOTH_PRED;
use crate::src::levels::INTER_INTRA_BLEND;
use crate::src::levels::MM_OBMC;
use crate::src::levels::MM_WARP;
use crate::src::levels::RTX_4X8;
use crate::src::levels::SMOOTH_PRED;
use crate::src::levels::TX_16X16;
use crate::src::levels::TX_32X32;
use crate::src::levels::TX_4X4;
use crate::src::levels::TX_64X64;

use crate::src::levels::TX_CLASS_2D;
use crate::src::levels::TX_CLASS_H;
use crate::src::levels::TX_CLASS_V;
use crate::src::levels::WHT_WHT;

use crate::src::tables::TxfmInfo;

use crate::src::recon::define_DEBUG_BLOCK_INFO;

define_DEBUG_BLOCK_INFO!();

#[inline]
unsafe extern "C" fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 != 0 {
        unreachable!();
    }
    return x >> 1;
}
use crate::include::common::dump::ac_dump;
use crate::include::common::dump::coef_dump;
use crate::include::common::dump::hex_dump;
use crate::include::common::intops::apply_sign64;
use crate::include::common::intops::iclip;
use crate::include::common::intops::imax;
use crate::include::common::intops::imin;
use crate::include::common::intops::umin;
use crate::src::env::get_uv_inter_txtp;
use crate::src::ipred_prepare::sm_flag;
use crate::src::ipred_prepare::sm_uv_flag;
use crate::src::msac::dav1d_msac_decode_bools;
use crate::src::recon::get_dc_sign_ctx;
use crate::src::recon::get_lo_ctx;
use crate::src::recon::get_skip_ctx;
use crate::src::recon::read_golomb;
unsafe fn decode_coefs(
    t: *mut Dav1dTaskContext,
    a: &mut [u8],
    l: &mut [u8],
    tx: RectTxfmSize,
    bs: BlockSize,
    b: *const Av1Block,
    intra: libc::c_int,
    plane: libc::c_int,
    mut cf: *mut coef,
    txtp: *mut TxfmType,
    mut res_ctx: *mut uint8_t,
) -> libc::c_int {
    let mut dc_sign_ctx = 0;
    let mut dc_sign = 0;
    let mut dc_dq = 0;
    let mut current_block: u64;
    let ts: *mut Dav1dTileState = (*t).ts;
    let chroma = (plane != 0) as libc::c_int;
    let f: *const Dav1dFrameContext = (*t).f;
    let lossless = (*(*f).frame_hdr).segmentation.lossless[(*b).seg_id as usize];
    let t_dim = &dav1d_txfm_dimensions[tx as usize];
    let dbg = DEBUG_BLOCK_INFO(&*f, &*t) as libc::c_int;
    if dbg != 0 {
        printf(
            b"Start: r=%d\n\0" as *const u8 as *const libc::c_char,
            (*ts).msac.rng,
        );
    }
    let sctx = get_skip_ctx(t_dim, bs, a, l, chroma, (*f).cur.p.layout) as libc::c_int;
    let all_skip = dav1d_msac_decode_bool_adapt(
        &mut (*ts).msac,
        &mut (*ts).cdf.coef.skip[(*t_dim).ctx as usize][sctx as usize],
    ) as libc::c_int;
    if dbg != 0 {
        printf(
            b"Post-non-zero[%d][%d][%d]: r=%d\n\0" as *const u8 as *const libc::c_char,
            (*t_dim).ctx as libc::c_int,
            sctx,
            all_skip,
            (*ts).msac.rng,
        );
    }
    if all_skip != 0 {
        *res_ctx = 0x40 as libc::c_int as uint8_t;
        *txtp = (lossless * WHT_WHT as libc::c_int) as TxfmType;
        return -(1 as libc::c_int);
    }
    if lossless != 0 {
        if !((*t_dim).max as libc::c_int == TX_4X4 as libc::c_int) {
            unreachable!();
        }
        *txtp = WHT_WHT;
    } else if (*t_dim).max as libc::c_int + intra >= TX_64X64 as libc::c_int {
        *txtp = DCT_DCT;
    } else if chroma != 0 {
        *txtp = (if intra != 0 {
            dav1d_txtp_from_uvmode[(*b).c2rust_unnamed.c2rust_unnamed.uv_mode as usize]
                as libc::c_uint
        } else {
            get_uv_inter_txtp(&*t_dim, *txtp) as libc::c_uint
        }) as TxfmType;
    } else if (*(*f).frame_hdr).segmentation.qidx[(*b).seg_id as usize] == 0 {
        *txtp = DCT_DCT;
    } else {
        let mut idx: libc::c_uint = 0;
        if intra != 0 {
            let y_mode_nofilt: IntraPredMode = (if (*b).c2rust_unnamed.c2rust_unnamed.y_mode
                as libc::c_int
                == FILTER_PRED as libc::c_int
            {
                dav1d_filter_mode_to_y_mode[(*b).c2rust_unnamed.c2rust_unnamed.y_angle as usize]
                    as libc::c_int
            } else {
                (*b).c2rust_unnamed.c2rust_unnamed.y_mode as libc::c_int
            }) as IntraPredMode;
            if (*(*f).frame_hdr).reduced_txtp_set != 0
                || (*t_dim).min as libc::c_int == TX_16X16 as libc::c_int
            {
                idx = dav1d_msac_decode_symbol_adapt4(
                    &mut (*ts).msac,
                    &mut (*ts).cdf.m.txtp_intra2[(*t_dim).min as usize][y_mode_nofilt as usize],
                    4 as libc::c_int as size_t,
                );
                *txtp = dav1d_tx_types_per_set
                    [idx.wrapping_add(0 as libc::c_int as libc::c_uint) as usize]
                    as TxfmType;
            } else {
                idx = dav1d_msac_decode_symbol_adapt8(
                    &mut (*ts).msac,
                    &mut (*ts).cdf.m.txtp_intra1[(*t_dim).min as usize][y_mode_nofilt as usize],
                    6 as libc::c_int as size_t,
                );
                *txtp = dav1d_tx_types_per_set
                    [idx.wrapping_add(5 as libc::c_int as libc::c_uint) as usize]
                    as TxfmType;
            }
            if dbg != 0 {
                printf(
                    b"Post-txtp-intra[%d->%d][%d][%d->%d]: r=%d\n\0" as *const u8
                        as *const libc::c_char,
                    tx as libc::c_uint,
                    (*t_dim).min as libc::c_int,
                    y_mode_nofilt as libc::c_uint,
                    idx,
                    *txtp as libc::c_uint,
                    (*ts).msac.rng,
                );
            }
        } else {
            if (*(*f).frame_hdr).reduced_txtp_set != 0
                || (*t_dim).max as libc::c_int == TX_32X32 as libc::c_int
            {
                idx = dav1d_msac_decode_bool_adapt(
                    &mut (*ts).msac,
                    &mut (*ts).cdf.m.txtp_inter3[(*t_dim).min as usize],
                ) as libc::c_uint;
                *txtp = (idx.wrapping_sub(1 as libc::c_int as libc::c_uint)
                    & IDTX as libc::c_int as libc::c_uint) as TxfmType;
            } else if (*t_dim).min as libc::c_int == TX_16X16 as libc::c_int {
                idx = dav1d_msac_decode_symbol_adapt16(
                    &mut (*ts).msac,
                    &mut (*ts).cdf.m.txtp_inter2.0,
                    11 as libc::c_int as size_t,
                );
                *txtp = dav1d_tx_types_per_set
                    [idx.wrapping_add(12 as libc::c_int as libc::c_uint) as usize]
                    as TxfmType;
            } else {
                idx = dav1d_msac_decode_symbol_adapt16(
                    &mut (*ts).msac,
                    &mut (*ts).cdf.m.txtp_inter1[(*t_dim).min as usize],
                    15 as libc::c_int as size_t,
                );
                *txtp = dav1d_tx_types_per_set
                    [idx.wrapping_add(24 as libc::c_int as libc::c_uint) as usize]
                    as TxfmType;
            }
            if dbg != 0 {
                printf(
                    b"Post-txtp-inter[%d->%d][%d->%d]: r=%d\n\0" as *const u8
                        as *const libc::c_char,
                    tx as libc::c_uint,
                    (*t_dim).min as libc::c_int,
                    idx,
                    *txtp as libc::c_uint,
                    (*ts).msac.rng,
                );
            }
        }
    }
    let mut eob_bin = 0;
    let tx2dszctx = imin((*t_dim).lw as libc::c_int, TX_32X32 as libc::c_int)
        + imin((*t_dim).lh as libc::c_int, TX_32X32 as libc::c_int);
    let tx_class: TxClass = dav1d_tx_type_class[*txtp as usize] as TxClass;
    let is_1d =
        (tx_class as libc::c_uint != TX_CLASS_2D as libc::c_int as libc::c_uint) as libc::c_int;
    match tx2dszctx {
        0 => {
            let eob_bin_cdf = &mut (*ts).cdf.coef.eob_bin_16[chroma as usize][is_1d as usize];
            eob_bin =
                dav1d_msac_decode_symbol_adapt4(&mut (*ts).msac, eob_bin_cdf, (4 + 0) as size_t)
                    as libc::c_int;
        }
        1 => {
            let eob_bin_cdf_0 = &mut (*ts).cdf.coef.eob_bin_32[chroma as usize][is_1d as usize];
            eob_bin =
                dav1d_msac_decode_symbol_adapt8(&mut (*ts).msac, eob_bin_cdf_0, (4 + 1) as size_t)
                    as libc::c_int;
        }
        2 => {
            let eob_bin_cdf_1 = &mut (*ts).cdf.coef.eob_bin_64[chroma as usize][is_1d as usize];
            eob_bin =
                dav1d_msac_decode_symbol_adapt8(&mut (*ts).msac, eob_bin_cdf_1, (4 + 2) as size_t)
                    as libc::c_int;
        }
        3 => {
            let eob_bin_cdf_2 = &mut (*ts).cdf.coef.eob_bin_128[chroma as usize][is_1d as usize];
            eob_bin =
                dav1d_msac_decode_symbol_adapt8(&mut (*ts).msac, eob_bin_cdf_2, (4 + 3) as size_t)
                    as libc::c_int;
        }
        4 => {
            let eob_bin_cdf_3 = &mut (*ts).cdf.coef.eob_bin_256[chroma as usize][is_1d as usize];
            eob_bin =
                dav1d_msac_decode_symbol_adapt16(&mut (*ts).msac, eob_bin_cdf_3, (4 + 4) as size_t)
                    as libc::c_int;
        }
        5 => {
            let eob_bin_cdf_4 = &mut (*ts).cdf.coef.eob_bin_512[chroma as usize];
            eob_bin =
                dav1d_msac_decode_symbol_adapt16(&mut (*ts).msac, eob_bin_cdf_4, (4 + 5) as size_t)
                    as libc::c_int;
        }
        6 => {
            let eob_bin_cdf_5 = &mut (*ts).cdf.coef.eob_bin_1024[chroma as usize];
            eob_bin =
                dav1d_msac_decode_symbol_adapt16(&mut (*ts).msac, eob_bin_cdf_5, (4 + 6) as size_t)
                    as libc::c_int;
        }
        _ => {}
    }
    if dbg != 0 {
        printf(
            b"Post-eob_bin_%d[%d][%d][%d]: r=%d\n\0" as *const u8 as *const libc::c_char,
            (16 as libc::c_int) << tx2dszctx,
            chroma,
            is_1d,
            eob_bin,
            (*ts).msac.rng,
        );
    }
    let mut eob = 0;
    if eob_bin > 1 {
        let eob_hi_bit_cdf = &mut (*ts).cdf.coef.eob_hi_bit[(*t_dim).ctx as usize][chroma as usize]
            [eob_bin as usize];
        let eob_hi_bit =
            dav1d_msac_decode_bool_adapt(&mut (*ts).msac, eob_hi_bit_cdf) as libc::c_int;
        if dbg != 0 {
            printf(
                b"Post-eob_hi_bit[%d][%d][%d][%d]: r=%d\n\0" as *const u8 as *const libc::c_char,
                (*t_dim).ctx as libc::c_int,
                chroma,
                eob_bin,
                eob_hi_bit,
                (*ts).msac.rng,
            );
        }
        eob = (((eob_hi_bit | 2) << eob_bin - 2) as libc::c_uint
            | dav1d_msac_decode_bools(&mut (*ts).msac, (eob_bin - 2) as libc::c_uint))
            as libc::c_int;
        if dbg != 0 {
            printf(
                b"Post-eob[%d]: r=%d\n\0" as *const u8 as *const libc::c_char,
                eob,
                (*ts).msac.rng,
            );
        }
    } else {
        eob = eob_bin;
    }
    if !(eob >= 0) {
        unreachable!();
    }
    let eob_cdf: *mut [uint16_t; 4] =
        ((*ts).cdf.coef.eob_base_tok[(*t_dim).ctx as usize][chroma as usize]).as_mut_ptr();
    let hi_cdf: *mut [uint16_t; 4] = ((*ts).cdf.coef.br_tok
        [imin((*t_dim).ctx as libc::c_int, 3 as libc::c_int) as usize][chroma as usize])
        .as_mut_ptr();
    let mut rc: libc::c_uint = 0;
    let mut dc_tok: libc::c_uint = 0;
    if eob != 0 {
        let lo_cdf: *mut [uint16_t; 4] =
            ((*ts).cdf.coef.base_tok[(*t_dim).ctx as usize][chroma as usize]).as_mut_ptr();
        let levels = &mut (*t).scratch.c2rust_unnamed_0.c2rust_unnamed.levels;
        let sw = imin((*t_dim).w as libc::c_int, 8 as libc::c_int);
        let sh = imin((*t_dim).h as libc::c_int, 8 as libc::c_int);
        let mut ctx: libc::c_uint = (1 as libc::c_int
            + (eob > sw * sh * 2) as libc::c_int
            + (eob > sw * sh * 4) as libc::c_int)
            as libc::c_uint;
        let mut eob_tok = dav1d_msac_decode_symbol_adapt4(
            &mut (*ts).msac,
            &mut *eob_cdf.offset(ctx as isize),
            2 as libc::c_int as size_t,
        ) as libc::c_int;
        let mut tok = eob_tok + 1;
        let mut level_tok = tok * 0x41 as libc::c_int;
        let mut mag: libc::c_uint = 0;
        let mut scan: *const uint16_t = 0 as *const uint16_t;
        match tx_class as libc::c_uint {
            0 => {
                let nonsquare_tx: libc::c_uint = (tx as libc::c_uint
                    >= RTX_4X8 as libc::c_int as libc::c_uint)
                    as libc::c_int as libc::c_uint;
                let lo_ctx_offsets = Some(
                    &dav1d_lo_ctx_offsets
                        [nonsquare_tx.wrapping_add(tx as libc::c_uint & nonsquare_tx) as usize],
                );
                scan = dav1d_scans[tx as usize];
                let stride: ptrdiff_t = (4 * sh) as ptrdiff_t;
                let shift: libc::c_uint = (if ((*t_dim).lh as libc::c_int) < 4 {
                    (*t_dim).lh as libc::c_int + 2
                } else {
                    5 as libc::c_int
                }) as libc::c_uint;
                let shift2: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                let mask: libc::c_uint = (4 * sh - 1) as libc::c_uint;
                memset(
                    levels.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    (stride * (4 * sw as isize + 2)) as size_t,
                );
                let mut x: libc::c_uint = 0;
                let mut y: libc::c_uint = 0;
                if TX_CLASS_2D as libc::c_int == TX_CLASS_2D as libc::c_int {
                    rc = *scan.offset(eob as isize) as libc::c_uint;
                    x = rc >> shift;
                    y = rc & mask;
                } else if TX_CLASS_2D as libc::c_int == TX_CLASS_H as libc::c_int {
                    x = eob as libc::c_uint & mask;
                    y = (eob >> shift) as libc::c_uint;
                    rc = eob as libc::c_uint;
                } else {
                    x = eob as libc::c_uint & mask;
                    y = (eob >> shift) as libc::c_uint;
                    rc = x << shift2 | y;
                }
                if dbg != 0 {
                    printf(
                        b"Post-lo_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                            as *const libc::c_char,
                        (*t_dim).ctx as libc::c_int,
                        chroma,
                        ctx,
                        eob,
                        rc,
                        tok,
                        (*ts).msac.rng,
                    );
                }
                if eob_tok == 2 {
                    ctx = (if if TX_CLASS_2D as libc::c_int == TX_CLASS_2D as libc::c_int {
                        (x | y > 1 as libc::c_uint) as libc::c_int
                    } else {
                        (y != 0 as libc::c_int as libc::c_uint) as libc::c_int
                    } != 0
                    {
                        14 as libc::c_int
                    } else {
                        7 as libc::c_int
                    }) as libc::c_uint;
                    tok = dav1d_msac_decode_hi_tok(
                        &mut (*ts).msac,
                        &mut *hi_cdf.offset(ctx as isize),
                    ) as libc::c_int;
                    level_tok = tok + ((3 as libc::c_int) << 6);
                    if dbg != 0 {
                        printf(
                            b"Post-hi_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                as *const libc::c_char,
                            imin((*t_dim).ctx as libc::c_int, 3 as libc::c_int),
                            chroma,
                            ctx,
                            eob,
                            rc,
                            tok,
                            (*ts).msac.rng,
                        );
                    }
                }
                *cf.offset(rc as isize) = tok << 11;
                levels[(x as isize * stride + y as isize) as usize] = level_tok as uint8_t;
                let mut i = eob - 1;
                while i > 0 {
                    let mut rc_i: libc::c_uint = 0;
                    if TX_CLASS_2D as libc::c_int == TX_CLASS_2D as libc::c_int {
                        rc_i = *scan.offset(i as isize) as libc::c_uint;
                        x = rc_i >> shift;
                        y = rc_i & mask;
                    } else if TX_CLASS_2D as libc::c_int == TX_CLASS_H as libc::c_int {
                        x = i as libc::c_uint & mask;
                        y = (i >> shift) as libc::c_uint;
                        rc_i = i as libc::c_uint;
                    } else {
                        x = i as libc::c_uint & mask;
                        y = (i >> shift) as libc::c_uint;
                        rc_i = x << shift2 | y;
                    }
                    if !(x < 32 as libc::c_uint && y < 32 as libc::c_uint) {
                        unreachable!();
                    }
                    let level = &mut levels[(x as isize * stride + y as isize) as usize..];
                    ctx = get_lo_ctx(
                        level,
                        TX_CLASS_2D,
                        &mut mag,
                        lo_ctx_offsets,
                        x as usize,
                        y as usize,
                        stride as usize,
                    ) as libc::c_uint;
                    if TX_CLASS_2D as libc::c_int == TX_CLASS_2D as libc::c_int {
                        y |= x;
                    }
                    tok = dav1d_msac_decode_symbol_adapt4(
                        &mut (*ts).msac,
                        &mut *lo_cdf.offset(ctx as isize),
                        3 as libc::c_int as size_t,
                    ) as libc::c_int;
                    if dbg != 0 {
                        printf(
                            b"Post-lo_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                as *const libc::c_char,
                            (*t_dim).ctx as libc::c_int,
                            chroma,
                            ctx,
                            i,
                            rc_i,
                            tok,
                            (*ts).msac.rng,
                        );
                    }
                    if tok == 3 {
                        mag &= 63 as libc::c_int as libc::c_uint;
                        ctx = ((if y
                            > (TX_CLASS_2D as libc::c_int == TX_CLASS_2D as libc::c_int)
                                as libc::c_int as libc::c_uint
                        {
                            14 as libc::c_int
                        } else {
                            7 as libc::c_int
                        }) as libc::c_uint)
                            .wrapping_add(if mag > 12 as libc::c_uint {
                                6 as libc::c_int as libc::c_uint
                            } else {
                                mag.wrapping_add(1 as libc::c_int as libc::c_uint) >> 1
                            });
                        tok = dav1d_msac_decode_hi_tok(
                            &mut (*ts).msac,
                            &mut *hi_cdf.offset(ctx as isize),
                        ) as libc::c_int;
                        if dbg != 0 {
                            printf(
                                b"Post-hi_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                    as *const libc::c_char,
                                imin((*t_dim).ctx as libc::c_int, 3 as libc::c_int),
                                chroma,
                                ctx,
                                i,
                                rc_i,
                                tok,
                                (*ts).msac.rng,
                            );
                        }
                        level[0] = (tok + ((3 as libc::c_int) << 6)) as uint8_t;
                        *cf.offset(rc_i as isize) = ((tok << 11) as libc::c_uint | rc) as coef;
                        rc = rc_i;
                    } else {
                        tok *= 0x17ff41 as libc::c_int;
                        level[0] = tok as uint8_t;
                        tok = ((tok >> 9) as libc::c_uint
                            & rc.wrapping_add(!(0x7ff as libc::c_uint)))
                            as libc::c_int;
                        if tok != 0 {
                            rc = rc_i;
                        }
                        *cf.offset(rc_i as isize) = tok;
                    }
                    i -= 1;
                }
                ctx = if TX_CLASS_2D as libc::c_int == TX_CLASS_2D as libc::c_int {
                    0 as libc::c_int as libc::c_uint
                } else {
                    get_lo_ctx(
                        levels,
                        TX_CLASS_2D,
                        &mut mag,
                        lo_ctx_offsets,
                        0,
                        0,
                        stride as usize,
                    ) as libc::c_uint
                };
                dc_tok = dav1d_msac_decode_symbol_adapt4(
                    &mut (*ts).msac,
                    &mut *lo_cdf.offset(ctx as isize),
                    3 as libc::c_int as size_t,
                );
                if dbg != 0 {
                    printf(
                        b"Post-dc_lo_tok[%d][%d][%d][%d]: r=%d\n\0" as *const u8
                            as *const libc::c_char,
                        (*t_dim).ctx as libc::c_int,
                        chroma,
                        ctx,
                        dc_tok,
                        (*ts).msac.rng,
                    );
                }
                if dc_tok == 3 as libc::c_uint {
                    if TX_CLASS_2D as libc::c_int == TX_CLASS_2D as libc::c_int {
                        mag = (levels[(0 * stride + 1) as usize] as libc::c_int
                            + levels[(1 * stride + 0) as usize] as libc::c_int
                            + levels[(1 * stride + 1) as usize] as libc::c_int)
                            as libc::c_uint;
                    }
                    mag &= 63 as libc::c_int as libc::c_uint;
                    ctx = if mag > 12 as libc::c_uint {
                        6 as libc::c_int as libc::c_uint
                    } else {
                        mag.wrapping_add(1 as libc::c_int as libc::c_uint) >> 1
                    };
                    dc_tok = dav1d_msac_decode_hi_tok(
                        &mut (*ts).msac,
                        &mut *hi_cdf.offset(ctx as isize),
                    );
                    if dbg != 0 {
                        printf(
                            b"Post-dc_hi_tok[%d][%d][0][%d]: r=%d\n\0" as *const u8
                                as *const libc::c_char,
                            imin((*t_dim).ctx as libc::c_int, 3 as libc::c_int),
                            chroma,
                            dc_tok,
                            (*ts).msac.rng,
                        );
                    }
                }
            }
            1 => {
                let lo_ctx_offsets_0 = None;
                let stride_0: ptrdiff_t = 16 as libc::c_int as ptrdiff_t;
                let shift_0: libc::c_uint = ((*t_dim).lh as libc::c_int + 2) as libc::c_uint;
                let shift2_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                let mask_0: libc::c_uint = (4 * sh - 1) as libc::c_uint;
                memset(
                    levels.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    (stride_0 * (4 * sh + 2) as isize) as usize,
                );
                let mut x_0: libc::c_uint = 0;
                let mut y_0: libc::c_uint = 0;
                if TX_CLASS_H as libc::c_int == TX_CLASS_2D as libc::c_int {
                    rc = *scan.offset(eob as isize) as libc::c_uint;
                    x_0 = rc >> shift_0;
                    y_0 = rc & mask_0;
                } else if TX_CLASS_H as libc::c_int == TX_CLASS_H as libc::c_int {
                    x_0 = eob as libc::c_uint & mask_0;
                    y_0 = (eob >> shift_0) as libc::c_uint;
                    rc = eob as libc::c_uint;
                } else {
                    x_0 = eob as libc::c_uint & mask_0;
                    y_0 = (eob >> shift_0) as libc::c_uint;
                    rc = x_0 << shift2_0 | y_0;
                }
                if dbg != 0 {
                    printf(
                        b"Post-lo_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                            as *const libc::c_char,
                        (*t_dim).ctx as libc::c_int,
                        chroma,
                        ctx,
                        eob,
                        rc,
                        tok,
                        (*ts).msac.rng,
                    );
                }
                if eob_tok == 2 {
                    ctx = (if if TX_CLASS_H as libc::c_int == TX_CLASS_2D as libc::c_int {
                        (x_0 | y_0 > 1 as libc::c_uint) as libc::c_int
                    } else {
                        (y_0 != 0 as libc::c_int as libc::c_uint) as libc::c_int
                    } != 0
                    {
                        14 as libc::c_int
                    } else {
                        7 as libc::c_int
                    }) as libc::c_uint;
                    tok = dav1d_msac_decode_hi_tok(
                        &mut (*ts).msac,
                        &mut *hi_cdf.offset(ctx as isize),
                    ) as libc::c_int;
                    level_tok = tok + ((3 as libc::c_int) << 6);
                    if dbg != 0 {
                        printf(
                            b"Post-hi_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                as *const libc::c_char,
                            imin((*t_dim).ctx as libc::c_int, 3 as libc::c_int),
                            chroma,
                            ctx,
                            eob,
                            rc,
                            tok,
                            (*ts).msac.rng,
                        );
                    }
                }
                *cf.offset(rc as isize) = tok << 11;
                levels[(x_0 as isize * stride_0 + y_0 as isize) as usize] = level_tok as uint8_t;
                let mut i_0 = eob - 1;
                while i_0 > 0 {
                    let mut rc_i_0: libc::c_uint = 0;
                    if TX_CLASS_H as libc::c_int == TX_CLASS_2D as libc::c_int {
                        rc_i_0 = *scan.offset(i_0 as isize) as libc::c_uint;
                        x_0 = rc_i_0 >> shift_0;
                        y_0 = rc_i_0 & mask_0;
                    } else if TX_CLASS_H as libc::c_int == TX_CLASS_H as libc::c_int {
                        x_0 = i_0 as libc::c_uint & mask_0;
                        y_0 = (i_0 >> shift_0) as libc::c_uint;
                        rc_i_0 = i_0 as libc::c_uint;
                    } else {
                        x_0 = i_0 as libc::c_uint & mask_0;
                        y_0 = (i_0 >> shift_0) as libc::c_uint;
                        rc_i_0 = x_0 << shift2_0 | y_0;
                    }
                    if !(x_0 < 32 as libc::c_uint && y_0 < 32 as libc::c_uint) {
                        unreachable!();
                    }
                    let level_0 =
                        &mut levels[(x_0 as isize * stride_0 as isize + y_0 as isize) as usize..];
                    ctx = get_lo_ctx(
                        level_0,
                        TX_CLASS_H,
                        &mut mag,
                        lo_ctx_offsets_0,
                        x_0 as usize,
                        y_0 as usize,
                        stride_0 as usize,
                    ) as libc::c_uint;
                    if TX_CLASS_H as libc::c_int == TX_CLASS_2D as libc::c_int {
                        y_0 |= x_0;
                    }
                    tok = dav1d_msac_decode_symbol_adapt4(
                        &mut (*ts).msac,
                        &mut *lo_cdf.offset(ctx as isize),
                        3 as libc::c_int as size_t,
                    ) as libc::c_int;
                    if dbg != 0 {
                        printf(
                            b"Post-lo_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                as *const libc::c_char,
                            (*t_dim).ctx as libc::c_int,
                            chroma,
                            ctx,
                            i_0,
                            rc_i_0,
                            tok,
                            (*ts).msac.rng,
                        );
                    }
                    if tok == 3 {
                        mag &= 63 as libc::c_int as libc::c_uint;
                        ctx = ((if y_0
                            > (TX_CLASS_H as libc::c_int == TX_CLASS_2D as libc::c_int)
                                as libc::c_int as libc::c_uint
                        {
                            14 as libc::c_int
                        } else {
                            7 as libc::c_int
                        }) as libc::c_uint)
                            .wrapping_add(if mag > 12 as libc::c_uint {
                                6 as libc::c_int as libc::c_uint
                            } else {
                                mag.wrapping_add(1 as libc::c_int as libc::c_uint) >> 1
                            });
                        tok = dav1d_msac_decode_hi_tok(
                            &mut (*ts).msac,
                            &mut *hi_cdf.offset(ctx as isize),
                        ) as libc::c_int;
                        if dbg != 0 {
                            printf(
                                b"Post-hi_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                    as *const libc::c_char,
                                imin((*t_dim).ctx as libc::c_int, 3 as libc::c_int),
                                chroma,
                                ctx,
                                i_0,
                                rc_i_0,
                                tok,
                                (*ts).msac.rng,
                            );
                        }
                        level_0[0] = (tok + ((3 as libc::c_int) << 6)) as uint8_t;
                        *cf.offset(rc_i_0 as isize) = ((tok << 11) as libc::c_uint | rc) as coef;
                        rc = rc_i_0;
                    } else {
                        tok *= 0x17ff41 as libc::c_int;
                        level_0[0] = tok as uint8_t;
                        tok = ((tok >> 9) as libc::c_uint
                            & rc.wrapping_add(!(0x7ff as libc::c_uint)))
                            as libc::c_int;
                        if tok != 0 {
                            rc = rc_i_0;
                        }
                        *cf.offset(rc_i_0 as isize) = tok;
                    }
                    i_0 -= 1;
                }
                ctx = if TX_CLASS_H as libc::c_int == TX_CLASS_2D as libc::c_int {
                    0 as libc::c_int as libc::c_uint
                } else {
                    get_lo_ctx(
                        levels,
                        TX_CLASS_H,
                        &mut mag,
                        lo_ctx_offsets_0,
                        0,
                        0,
                        stride_0 as usize,
                    ) as libc::c_uint
                };
                dc_tok = dav1d_msac_decode_symbol_adapt4(
                    &mut (*ts).msac,
                    &mut *lo_cdf.offset(ctx as isize),
                    3 as libc::c_int as size_t,
                );
                if dbg != 0 {
                    printf(
                        b"Post-dc_lo_tok[%d][%d][%d][%d]: r=%d\n\0" as *const u8
                            as *const libc::c_char,
                        (*t_dim).ctx as libc::c_int,
                        chroma,
                        ctx,
                        dc_tok,
                        (*ts).msac.rng,
                    );
                }
                if dc_tok == 3 as libc::c_uint {
                    if TX_CLASS_H as libc::c_int == TX_CLASS_2D as libc::c_int {
                        mag = (levels[(0 * stride_0 + 1) as usize] as libc::c_int
                            + levels[(1 * stride_0 + 0) as usize] as libc::c_int
                            + levels[(1 * stride_0 + 1) as usize] as libc::c_int)
                            as libc::c_uint;
                    }
                    mag &= 63 as libc::c_int as libc::c_uint;
                    ctx = if mag > 12 as libc::c_uint {
                        6 as libc::c_int as libc::c_uint
                    } else {
                        mag.wrapping_add(1 as libc::c_int as libc::c_uint) >> 1
                    };
                    dc_tok = dav1d_msac_decode_hi_tok(
                        &mut (*ts).msac,
                        &mut *hi_cdf.offset(ctx as isize),
                    );
                    if dbg != 0 {
                        printf(
                            b"Post-dc_hi_tok[%d][%d][0][%d]: r=%d\n\0" as *const u8
                                as *const libc::c_char,
                            imin((*t_dim).ctx as libc::c_int, 3 as libc::c_int),
                            chroma,
                            dc_tok,
                            (*ts).msac.rng,
                        );
                    }
                }
            }
            2 => {
                let lo_ctx_offsets_1 = None;
                let stride_1: ptrdiff_t = 16 as libc::c_int as ptrdiff_t;
                let shift_1: libc::c_uint = ((*t_dim).lw as libc::c_int + 2) as libc::c_uint;
                let shift2_1: libc::c_uint = ((*t_dim).lh as libc::c_int + 2) as libc::c_uint;
                let mask_1: libc::c_uint = (4 * sw - 1) as libc::c_uint;
                memset(
                    levels.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    (stride_1 * (4 * sw + 2) as isize) as size_t,
                );
                let mut x_1: libc::c_uint = 0;
                let mut y_1: libc::c_uint = 0;
                if TX_CLASS_V as libc::c_int == TX_CLASS_2D as libc::c_int {
                    rc = *scan.offset(eob as isize) as libc::c_uint;
                    x_1 = rc >> shift_1;
                    y_1 = rc & mask_1;
                } else if TX_CLASS_V as libc::c_int == TX_CLASS_H as libc::c_int {
                    x_1 = eob as libc::c_uint & mask_1;
                    y_1 = (eob >> shift_1) as libc::c_uint;
                    rc = eob as libc::c_uint;
                } else {
                    x_1 = eob as libc::c_uint & mask_1;
                    y_1 = (eob >> shift_1) as libc::c_uint;
                    rc = x_1 << shift2_1 | y_1;
                }
                if dbg != 0 {
                    printf(
                        b"Post-lo_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                            as *const libc::c_char,
                        (*t_dim).ctx as libc::c_int,
                        chroma,
                        ctx,
                        eob,
                        rc,
                        tok,
                        (*ts).msac.rng,
                    );
                }
                if eob_tok == 2 {
                    ctx = (if if TX_CLASS_V as libc::c_int == TX_CLASS_2D as libc::c_int {
                        (x_1 | y_1 > 1 as libc::c_uint) as libc::c_int
                    } else {
                        (y_1 != 0 as libc::c_int as libc::c_uint) as libc::c_int
                    } != 0
                    {
                        14 as libc::c_int
                    } else {
                        7 as libc::c_int
                    }) as libc::c_uint;
                    tok = dav1d_msac_decode_hi_tok(
                        &mut (*ts).msac,
                        &mut *hi_cdf.offset(ctx as isize),
                    ) as libc::c_int;
                    level_tok = tok + ((3 as libc::c_int) << 6);
                    if dbg != 0 {
                        printf(
                            b"Post-hi_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                as *const libc::c_char,
                            imin((*t_dim).ctx as libc::c_int, 3 as libc::c_int),
                            chroma,
                            ctx,
                            eob,
                            rc,
                            tok,
                            (*ts).msac.rng,
                        );
                    }
                }
                *cf.offset(rc as isize) = tok << 11;
                levels[(x_1 as isize * stride_1 + y_1 as isize) as usize] = level_tok as uint8_t;
                let mut i_1 = eob - 1;
                while i_1 > 0 {
                    let mut rc_i_1: libc::c_uint = 0;
                    if TX_CLASS_V as libc::c_int == TX_CLASS_2D as libc::c_int {
                        rc_i_1 = *scan.offset(i_1 as isize) as libc::c_uint;
                        x_1 = rc_i_1 >> shift_1;
                        y_1 = rc_i_1 & mask_1;
                    } else if TX_CLASS_V as libc::c_int == TX_CLASS_H as libc::c_int {
                        x_1 = i_1 as libc::c_uint & mask_1;
                        y_1 = (i_1 >> shift_1) as libc::c_uint;
                        rc_i_1 = i_1 as libc::c_uint;
                    } else {
                        x_1 = i_1 as libc::c_uint & mask_1;
                        y_1 = (i_1 >> shift_1) as libc::c_uint;
                        rc_i_1 = x_1 << shift2_1 | y_1;
                    }
                    if !(x_1 < 32 as libc::c_uint && y_1 < 32 as libc::c_uint) {
                        unreachable!();
                    }
                    let level_1 = &mut levels[(x_1 as isize * stride_1 + y_1 as isize) as usize..];
                    ctx = get_lo_ctx(
                        level_1,
                        TX_CLASS_V,
                        &mut mag,
                        lo_ctx_offsets_1,
                        x_1 as usize,
                        y_1 as usize,
                        stride_1 as usize,
                    ) as libc::c_uint;
                    if TX_CLASS_V as libc::c_int == TX_CLASS_2D as libc::c_int {
                        y_1 |= x_1;
                    }
                    tok = dav1d_msac_decode_symbol_adapt4(
                        &mut (*ts).msac,
                        &mut *lo_cdf.offset(ctx as isize),
                        3 as libc::c_int as size_t,
                    ) as libc::c_int;
                    if dbg != 0 {
                        printf(
                            b"Post-lo_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                as *const libc::c_char,
                            (*t_dim).ctx as libc::c_int,
                            chroma,
                            ctx,
                            i_1,
                            rc_i_1,
                            tok,
                            (*ts).msac.rng,
                        );
                    }
                    if tok == 3 {
                        mag &= 63 as libc::c_int as libc::c_uint;
                        ctx = ((if y_1
                            > (TX_CLASS_V as libc::c_int == TX_CLASS_2D as libc::c_int)
                                as libc::c_int as libc::c_uint
                        {
                            14 as libc::c_int
                        } else {
                            7 as libc::c_int
                        }) as libc::c_uint)
                            .wrapping_add(if mag > 12 as libc::c_uint {
                                6 as libc::c_int as libc::c_uint
                            } else {
                                mag.wrapping_add(1 as libc::c_int as libc::c_uint) >> 1
                            });
                        tok = dav1d_msac_decode_hi_tok(
                            &mut (*ts).msac,
                            &mut *hi_cdf.offset(ctx as isize),
                        ) as libc::c_int;
                        if dbg != 0 {
                            printf(
                                b"Post-hi_tok[%d][%d][%d][%d=%d=%d]: r=%d\n\0" as *const u8
                                    as *const libc::c_char,
                                imin((*t_dim).ctx as libc::c_int, 3 as libc::c_int),
                                chroma,
                                ctx,
                                i_1,
                                rc_i_1,
                                tok,
                                (*ts).msac.rng,
                            );
                        }
                        level_1[0] = (tok + ((3 as libc::c_int) << 6)) as uint8_t;
                        *cf.offset(rc_i_1 as isize) = ((tok << 11) as libc::c_uint | rc) as coef;
                        rc = rc_i_1;
                    } else {
                        tok *= 0x17ff41 as libc::c_int;
                        level_1[0] = tok as uint8_t;
                        tok = ((tok >> 9) as libc::c_uint
                            & rc.wrapping_add(!(0x7ff as libc::c_uint)))
                            as libc::c_int;
                        if tok != 0 {
                            rc = rc_i_1;
                        }
                        *cf.offset(rc_i_1 as isize) = tok;
                    }
                    i_1 -= 1;
                }
                ctx = if TX_CLASS_V as libc::c_int == TX_CLASS_2D as libc::c_int {
                    0 as libc::c_int as libc::c_uint
                } else {
                    get_lo_ctx(
                        levels,
                        TX_CLASS_V,
                        &mut mag,
                        lo_ctx_offsets_1,
                        0,
                        0,
                        stride_1 as usize,
                    ) as libc::c_uint
                };
                dc_tok = dav1d_msac_decode_symbol_adapt4(
                    &mut (*ts).msac,
                    &mut *lo_cdf.offset(ctx as isize),
                    3 as libc::c_int as size_t,
                );
                if dbg != 0 {
                    printf(
                        b"Post-dc_lo_tok[%d][%d][%d][%d]: r=%d\n\0" as *const u8
                            as *const libc::c_char,
                        (*t_dim).ctx as libc::c_int,
                        chroma,
                        ctx,
                        dc_tok,
                        (*ts).msac.rng,
                    );
                }
                if dc_tok == 3 as libc::c_uint {
                    if TX_CLASS_V as libc::c_int == TX_CLASS_2D as libc::c_int {
                        mag = (levels[(0 * stride_1 + 1) as usize] as libc::c_int
                            + levels[(1 * stride_1 + 0) as usize] as libc::c_int
                            + levels[(1 * stride_1 + 1) as usize] as libc::c_int)
                            as libc::c_uint;
                    }
                    mag &= 63 as libc::c_int as libc::c_uint;
                    ctx = if mag > 12 as libc::c_uint {
                        6 as libc::c_int as libc::c_uint
                    } else {
                        mag.wrapping_add(1 as libc::c_int as libc::c_uint) >> 1
                    };
                    dc_tok = dav1d_msac_decode_hi_tok(
                        &mut (*ts).msac,
                        &mut *hi_cdf.offset(ctx as isize),
                    );
                    if dbg != 0 {
                        printf(
                            b"Post-dc_hi_tok[%d][%d][0][%d]: r=%d\n\0" as *const u8
                                as *const libc::c_char,
                            imin((*t_dim).ctx as libc::c_int, 3 as libc::c_int),
                            chroma,
                            dc_tok,
                            (*ts).msac.rng,
                        );
                    }
                }
            }
            _ => {
                if 0 == 0 {
                    unreachable!();
                }
            }
        }
    } else {
        let mut tok_br = dav1d_msac_decode_symbol_adapt4(
            &mut (*ts).msac,
            &mut *eob_cdf.offset(0),
            2 as libc::c_int as size_t,
        ) as libc::c_int;
        dc_tok = (1 + tok_br) as libc::c_uint;
        if dbg != 0 {
            printf(
                b"Post-dc_lo_tok[%d][%d][%d][%d]: r=%d\n\0" as *const u8 as *const libc::c_char,
                (*t_dim).ctx as libc::c_int,
                chroma,
                0 as libc::c_int,
                dc_tok,
                (*ts).msac.rng,
            );
        }
        if tok_br == 2 {
            dc_tok = dav1d_msac_decode_hi_tok(&mut (*ts).msac, &mut *hi_cdf.offset(0));
            if dbg != 0 {
                printf(
                    b"Post-dc_hi_tok[%d][%d][0][%d]: r=%d\n\0" as *const u8 as *const libc::c_char,
                    imin((*t_dim).ctx as libc::c_int, 3 as libc::c_int),
                    chroma,
                    dc_tok,
                    (*ts).msac.rng,
                );
            }
        }
        rc = 0 as libc::c_int as libc::c_uint;
    }
    let dq_tbl: *const uint16_t =
        ((*((*ts).dq).offset((*b).seg_id as isize))[plane as usize]).as_ptr();
    let qm_tbl: *const uint8_t = if (*txtp as libc::c_uint) < IDTX as libc::c_int as libc::c_uint {
        (*f).qm[tx as usize][plane as usize]
    } else {
        0 as *const uint8_t
    };
    let dq_shift = imax(0 as libc::c_int, (*t_dim).ctx as libc::c_int - 2);
    let cf_max = !(!(127 as libc::c_uint)
        << (if 16 == 8 {
            8 as libc::c_int
        } else {
            (*f).cur.p.bpc
        })) as libc::c_int;
    let mut cul_level: libc::c_uint = 0;
    let mut dc_sign_level: libc::c_uint = 0;
    if dc_tok == 0 {
        cul_level = 0 as libc::c_int as libc::c_uint;
        dc_sign_level = ((1 as libc::c_int) << 6) as libc::c_uint;
        if !qm_tbl.is_null() {
            current_block = 1669574575799829731;
        } else {
            current_block = 2404388531445638768;
        }
    } else {
        dc_sign_ctx = get_dc_sign_ctx(tx, a, l) as libc::c_int;
        let dc_sign_cdf = &mut (*ts).cdf.coef.dc_sign[chroma as usize][dc_sign_ctx as usize];
        dc_sign = dav1d_msac_decode_bool_adapt(&mut (*ts).msac, dc_sign_cdf) as libc::c_int;
        if dbg != 0 {
            printf(
                b"Post-dc_sign[%d][%d][%d]: r=%d\n\0" as *const u8 as *const libc::c_char,
                chroma,
                dc_sign_ctx,
                dc_sign,
                (*ts).msac.rng,
            );
        }
        dc_dq = *dq_tbl.offset(0) as libc::c_int;
        dc_sign_level = (dc_sign - 1 & (2 as libc::c_int) << 6) as libc::c_uint;
        if !qm_tbl.is_null() {
            dc_dq = dc_dq * *qm_tbl.offset(0) as libc::c_int + 16 >> 5;
            if dc_tok == 15 as libc::c_uint {
                dc_tok =
                    (read_golomb(&mut (*ts).msac)).wrapping_add(15 as libc::c_int as libc::c_uint);
                if dbg != 0 {
                    printf(
                        b"Post-dc_residual[%d->%d]: r=%d\n\0" as *const u8 as *const libc::c_char,
                        dc_tok.wrapping_sub(15 as libc::c_int as libc::c_uint),
                        dc_tok,
                        (*ts).msac.rng,
                    );
                }
                dc_tok &= 0xfffff as libc::c_int as libc::c_uint;
                dc_dq = ((dc_dq as libc::c_uint).wrapping_mul(dc_tok)
                    & 0xffffff as libc::c_int as libc::c_uint)
                    as libc::c_int;
            } else {
                dc_dq = (dc_dq as libc::c_uint).wrapping_mul(dc_tok) as libc::c_int as libc::c_int;
                if !(dc_dq <= 0xffffff as libc::c_int) {
                    unreachable!();
                }
            }
            cul_level = dc_tok;
            dc_dq >>= dq_shift;
            dc_dq = umin(dc_dq as libc::c_uint, (cf_max + dc_sign) as libc::c_uint) as libc::c_int;
            *cf.offset(0) = if dc_sign != 0 { -dc_dq } else { dc_dq };
            if rc != 0 {
                current_block = 1669574575799829731;
            } else {
                current_block = 15494703142406051947;
            }
        } else {
            if dc_tok == 15 as libc::c_uint {
                dc_tok =
                    (read_golomb(&mut (*ts).msac)).wrapping_add(15 as libc::c_int as libc::c_uint);
                if dbg != 0 {
                    printf(
                        b"Post-dc_residual[%d->%d]: r=%d\n\0" as *const u8 as *const libc::c_char,
                        dc_tok.wrapping_sub(15 as libc::c_int as libc::c_uint),
                        dc_tok,
                        (*ts).msac.rng,
                    );
                }
                dc_tok &= 0xfffff as libc::c_int as libc::c_uint;
                dc_dq = (((dc_dq as libc::c_uint).wrapping_mul(dc_tok)
                    & 0xffffff as libc::c_int as libc::c_uint)
                    >> dq_shift) as libc::c_int;
                dc_dq =
                    umin(dc_dq as libc::c_uint, (cf_max + dc_sign) as libc::c_uint) as libc::c_int;
            } else {
                dc_dq = ((dc_dq as libc::c_uint).wrapping_mul(dc_tok) >> dq_shift) as libc::c_int;
                if !(dc_dq <= cf_max) {
                    unreachable!();
                }
            }
            cul_level = dc_tok;
            *cf.offset(0) = if dc_sign != 0 { -dc_dq } else { dc_dq };
            if rc != 0 {
                current_block = 2404388531445638768;
            } else {
                current_block = 15494703142406051947;
            }
        }
    }
    match current_block {
        1669574575799829731 => {
            let ac_dq: libc::c_uint = *dq_tbl.offset(1) as libc::c_uint;
            loop {
                let sign = dav1d_msac_decode_bool_equi(&mut (*ts).msac) as libc::c_int;
                if dbg != 0 {
                    printf(
                        b"Post-sign[%d=%d]: r=%d\n\0" as *const u8 as *const libc::c_char,
                        rc,
                        sign,
                        (*ts).msac.rng,
                    );
                }
                let rc_tok: libc::c_uint = *cf.offset(rc as isize) as libc::c_uint;
                let mut tok_0: libc::c_uint = 0;
                let mut dq: libc::c_uint = ac_dq
                    .wrapping_mul(*qm_tbl.offset(rc as isize) as libc::c_uint)
                    .wrapping_add(16 as libc::c_int as libc::c_uint)
                    >> 5;
                let mut dq_sat = 0;
                if rc_tok >= ((15 as libc::c_int) << 11) as libc::c_uint {
                    tok_0 = (read_golomb(&mut (*ts).msac))
                        .wrapping_add(15 as libc::c_int as libc::c_uint);
                    if dbg != 0 {
                        printf(
                            b"Post-residual[%d=%d->%d]: r=%d\n\0" as *const u8
                                as *const libc::c_char,
                            rc,
                            tok_0.wrapping_sub(15 as libc::c_int as libc::c_uint),
                            tok_0,
                            (*ts).msac.rng,
                        );
                    }
                    tok_0 &= 0xfffff as libc::c_int as libc::c_uint;
                    dq = dq.wrapping_mul(tok_0) & 0xffffff as libc::c_int as libc::c_uint;
                } else {
                    tok_0 = rc_tok >> 11;
                    dq = dq.wrapping_mul(tok_0);
                    if !(dq <= 0xffffff as libc::c_int as libc::c_uint) {
                        unreachable!();
                    }
                }
                cul_level = cul_level.wrapping_add(tok_0);
                dq >>= dq_shift;
                dq_sat = umin(dq, (cf_max + sign) as libc::c_uint) as libc::c_int;
                *cf.offset(rc as isize) = if sign != 0 { -dq_sat } else { dq_sat };
                rc = rc_tok & 0x3ff as libc::c_int as libc::c_uint;
                if !(rc != 0) {
                    break;
                }
            }
        }
        2404388531445638768 => {
            let ac_dq_0: libc::c_uint = *dq_tbl.offset(1) as libc::c_uint;
            loop {
                let sign_0 = dav1d_msac_decode_bool_equi(&mut (*ts).msac) as libc::c_int;
                if dbg != 0 {
                    printf(
                        b"Post-sign[%d=%d]: r=%d\n\0" as *const u8 as *const libc::c_char,
                        rc,
                        sign_0,
                        (*ts).msac.rng,
                    );
                }
                let rc_tok_0: libc::c_uint = *cf.offset(rc as isize) as libc::c_uint;
                let mut tok_1: libc::c_uint = 0;
                let mut dq_0 = 0;
                if rc_tok_0 >= ((15 as libc::c_int) << 11) as libc::c_uint {
                    tok_1 = (read_golomb(&mut (*ts).msac))
                        .wrapping_add(15 as libc::c_int as libc::c_uint);
                    if dbg != 0 {
                        printf(
                            b"Post-residual[%d=%d->%d]: r=%d\n\0" as *const u8
                                as *const libc::c_char,
                            rc,
                            tok_1.wrapping_sub(15 as libc::c_int as libc::c_uint),
                            tok_1,
                            (*ts).msac.rng,
                        );
                    }
                    tok_1 &= 0xfffff as libc::c_int as libc::c_uint;
                    dq_0 = ((ac_dq_0.wrapping_mul(tok_1) & 0xffffff as libc::c_int as libc::c_uint)
                        >> dq_shift) as libc::c_int;
                    dq_0 = umin(dq_0 as libc::c_uint, (cf_max + sign_0) as libc::c_uint)
                        as libc::c_int;
                } else {
                    tok_1 = rc_tok_0 >> 11;
                    dq_0 = (ac_dq_0.wrapping_mul(tok_1) >> dq_shift) as libc::c_int;
                    if !(dq_0 <= cf_max) {
                        unreachable!();
                    }
                }
                cul_level = cul_level.wrapping_add(tok_1);
                *cf.offset(rc as isize) = if sign_0 != 0 { -dq_0 } else { dq_0 };
                rc = rc_tok_0 & 0x3ff as libc::c_int as libc::c_uint;
                if !(rc != 0) {
                    break;
                }
            }
        }
        _ => {}
    }
    *res_ctx = (umin(cul_level, 63 as libc::c_int as libc::c_uint) | dc_sign_level) as uint8_t;
    return eob;
}
unsafe extern "C" fn read_coef_tree(
    t: *mut Dav1dTaskContext,
    bs: BlockSize,
    b: *const Av1Block,
    ytx: RectTxfmSize,
    depth: libc::c_int,
    tx_split: *const uint16_t,
    x_off: libc::c_int,
    y_off: libc::c_int,
    mut dst: *mut pixel,
) {
    let f: *const Dav1dFrameContext = (*t).f;
    let ts: *mut Dav1dTileState = (*t).ts;
    let dsp: *const Dav1dDSPContext = (*f).dsp;
    let t_dim: *const TxfmInfo =
        &*dav1d_txfm_dimensions.as_ptr().offset(ytx as isize) as *const TxfmInfo;
    let txw = (*t_dim).w as libc::c_int;
    let txh = (*t_dim).h as libc::c_int;
    if depth < 2
        && *tx_split.offset(depth as isize) as libc::c_int != 0
        && *tx_split.offset(depth as isize) as libc::c_int & (1 as libc::c_int) << y_off * 4 + x_off
            != 0
    {
        let sub: RectTxfmSize = (*t_dim).sub as RectTxfmSize;
        let sub_t_dim: *const TxfmInfo =
            &*dav1d_txfm_dimensions.as_ptr().offset(sub as isize) as *const TxfmInfo;
        let txsw = (*sub_t_dim).w as libc::c_int;
        let txsh = (*sub_t_dim).h as libc::c_int;
        read_coef_tree(
            t,
            bs,
            b,
            sub,
            depth + 1,
            tx_split,
            x_off * 2 + 0,
            y_off * 2 + 0,
            dst,
        );
        (*t).bx += txsw;
        if txw >= txh && (*t).bx < (*f).bw {
            read_coef_tree(
                t,
                bs,
                b,
                sub,
                depth + 1,
                tx_split,
                x_off * 2 + 1,
                y_off * 2 + 0,
                if !dst.is_null() {
                    &mut *dst.offset((4 * txsw) as isize)
                } else {
                    0 as *mut pixel
                },
            );
        }
        (*t).bx -= txsw;
        (*t).by += txsh;
        if txh >= txw && (*t).by < (*f).bh {
            if !dst.is_null() {
                dst = dst.offset(((4 * txsh) as isize * PXSTRIDE((*f).cur.stride[0])) as isize);
            }
            read_coef_tree(
                t,
                bs,
                b,
                sub,
                depth + 1,
                tx_split,
                x_off * 2 + 0,
                y_off * 2 + 1,
                dst,
            );
            (*t).bx += txsw;
            if txw >= txh && (*t).bx < (*f).bw {
                read_coef_tree(
                    t,
                    bs,
                    b,
                    sub,
                    depth + 1,
                    tx_split,
                    x_off * 2 + 1,
                    y_off * 2 + 1,
                    if !dst.is_null() {
                        &mut *dst.offset((4 * txsw) as isize)
                    } else {
                        0 as *mut pixel
                    },
                );
            }
            (*t).bx -= txsw;
        }
        (*t).by -= txsh;
    } else {
        let bx4 = (*t).bx & 31;
        let by4 = (*t).by & 31;
        let mut txtp: TxfmType = DCT_DCT;
        let mut cf_ctx: uint8_t = 0;
        let mut eob = 0;
        let mut cf: *mut coef = 0 as *mut coef;
        let mut cbi: *mut CodedBlockInfo = 0 as *mut CodedBlockInfo;
        if (*t).frame_thread.pass != 0 {
            let p = (*t).frame_thread.pass & 1;
            if ((*ts).frame_thread[p as usize].cf).is_null() {
                unreachable!();
            }
            cf = (*ts).frame_thread[p as usize].cf as *mut coef;
            (*ts).frame_thread[p as usize].cf = ((*ts).frame_thread[p as usize].cf as *mut coef)
                .offset(
                    (imin((*t_dim).w as libc::c_int, 8 as libc::c_int)
                        * imin((*t_dim).h as libc::c_int, 8 as libc::c_int)
                        * 16) as isize,
                ) as *mut libc::c_void;
            cbi = &mut *((*f).frame_thread.cbi)
                .offset(((*t).by as isize * (*f).b4_stride + (*t).bx as isize) as isize)
                as *mut CodedBlockInfo;
        } else {
            cf = ((*t).c2rust_unnamed.cf_16bpc).as_mut_ptr();
        }
        if (*t).frame_thread.pass != 2 as libc::c_int {
            eob = decode_coefs(
                t,
                &mut (*(*t).a).lcoef.0[bx4 as usize..],
                &mut (*t).l.lcoef.0[by4 as usize..],
                ytx,
                bs,
                b,
                0 as libc::c_int,
                0 as libc::c_int,
                cf,
                &mut txtp,
                &mut cf_ctx,
            );
            if DEBUG_BLOCK_INFO(&*f, &*t) {
                printf(
                    b"Post-y-cf-blk[tx=%d,txtp=%d,eob=%d]: r=%d\n\0" as *const u8
                        as *const libc::c_char,
                    ytx as libc::c_uint,
                    txtp as libc::c_uint,
                    eob,
                    (*ts).msac.rng,
                );
            }
            CaseSet::<16, true>::many(
                [&mut (*t).l, &mut *(*t).a],
                [
                    imin(txh, (*f).bh - (*t).by) as usize,
                    imin(txw, (*f).bw - (*t).bx) as usize,
                ],
                [by4 as usize, bx4 as usize],
                |case, dir| {
                    case.set(&mut dir.lcoef.0, cf_ctx);
                },
            );
            let mut txtp_map = &mut (*t).txtp_map[(by4 * 32 + bx4) as usize..];
            CaseSet::<16, false>::one((), txw as usize, 0, |case, ()| {
                for txtp_map in txtp_map.chunks_mut(32).take(txh as usize) {
                    case.set(txtp_map, txtp);
                }
            });
            if (*t).frame_thread.pass == 1 {
                (*cbi).eob[0] = eob as int16_t;
                (*cbi).txtp[0] = txtp as uint8_t;
            }
        } else {
            eob = (*cbi).eob[0] as libc::c_int;
            txtp = (*cbi).txtp[0] as TxfmType;
        }
        if (*t).frame_thread.pass & 1 == 0 {
            if dst.is_null() {
                unreachable!();
            }
            if eob >= 0 {
                if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                    coef_dump(
                        cf,
                        std::cmp::min((*t_dim).h as usize, 8) * 4,
                        std::cmp::min((*t_dim).w as usize, 8) * 4,
                        3,
                        "dq",
                    );
                }
                ((*dsp).itx.itxfm_add[ytx as usize][txtp as usize])
                    .expect("non-null function pointer")(
                    dst,
                    (*f).cur.stride[0],
                    cf,
                    eob,
                    (*f).bitdepth_max,
                );
                if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                    hex_dump::<BitDepth16>(
                        dst,
                        (*f).cur.stride[0] as usize,
                        (*t_dim).w as usize * 4,
                        (*t_dim).h as usize * 4,
                        "recon",
                    );
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_read_coef_blocks_16bpc(
    t: *mut Dav1dTaskContext,
    bs: BlockSize,
    b: *const Av1Block,
) {
    let f: *const Dav1dFrameContext = (*t).f;
    let ss_ver = ((*f).cur.p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_hor = ((*f).cur.p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
    let bx4 = (*t).bx & 31;
    let by4 = (*t).by & 31;
    let cbx4 = bx4 >> ss_hor;
    let cby4 = by4 >> ss_ver;
    let b_dim: *const uint8_t = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bw4 = *b_dim.offset(0) as libc::c_int;
    let bh4 = *b_dim.offset(1) as libc::c_int;
    let cbw4 = bw4 + ss_hor >> ss_hor;
    let cbh4 = bh4 + ss_ver >> ss_ver;
    let has_chroma = ((*f).cur.p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint
        && (bw4 > ss_hor || (*t).bx & 1 != 0)
        && (bh4 > ss_ver || (*t).by & 1 != 0)) as libc::c_int;
    if (*b).skip != 0 {
        CaseSet::<32, false>::many(
            [&mut (*t).l, &mut *(*t).a],
            [bh4 as usize, bw4 as usize],
            [by4 as usize, bx4 as usize],
            |case, dir| {
                case.set(&mut dir.lcoef.0, 0x40);
            },
        );
        if has_chroma != 0 {
            CaseSet::<32, false>::many(
                [&mut (*t).l, &mut *(*t).a],
                [cbh4 as usize, cbw4 as usize],
                [cby4 as usize, cbx4 as usize],
                |case, dir| {
                    case.set(&mut dir.ccoef.0[0], 0x40);
                    case.set(&mut dir.ccoef.0[1], 0x40);
                },
            );
        }
        return;
    }
    let ts: *mut Dav1dTileState = (*t).ts;
    let w4 = imin(bw4, (*f).bw - (*t).bx);
    let h4 = imin(bh4, (*f).bh - (*t).by);
    let cw4 = w4 + ss_hor >> ss_hor;
    let ch4 = h4 + ss_ver >> ss_ver;
    if !((*t).frame_thread.pass == 1) {
        unreachable!();
    }
    if (*b).skip != 0 {
        unreachable!();
    }
    let uv_t_dim: *const TxfmInfo =
        &*dav1d_txfm_dimensions.as_ptr().offset((*b).uvtx as isize) as *const TxfmInfo;
    let t_dim: *const TxfmInfo = &*dav1d_txfm_dimensions.as_ptr().offset(
        (if (*b).intra as libc::c_int != 0 {
            (*b).c2rust_unnamed.c2rust_unnamed.tx as libc::c_int
        } else {
            (*b).c2rust_unnamed.c2rust_unnamed_0.max_ytx as libc::c_int
        }) as isize,
    ) as *const TxfmInfo;
    let tx_split: [uint16_t; 2] = [
        (*b).c2rust_unnamed.c2rust_unnamed_0.tx_split0 as uint16_t,
        (*b).c2rust_unnamed.c2rust_unnamed_0.tx_split1,
    ];
    let mut init_y = 0;
    while init_y < h4 {
        let sub_h4 = imin(h4, 16 + init_y);
        let mut init_x = 0;
        while init_x < w4 {
            let sub_w4 = imin(w4, init_x + 16);
            let mut y_off = (init_y != 0) as libc::c_int;
            let mut y = 0;
            let mut x = 0;
            y = init_y;
            (*t).by += init_y;
            while y < sub_h4 {
                let cbi: *mut CodedBlockInfo = &mut *((*f).frame_thread.cbi)
                    .offset(((*t).by as isize * (*f).b4_stride) as isize)
                    as *mut CodedBlockInfo;
                let mut x_off = (init_x != 0) as libc::c_int;
                x = init_x;
                (*t).bx += init_x;
                while x < sub_w4 {
                    if (*b).intra == 0 {
                        read_coef_tree(
                            t,
                            bs,
                            b,
                            (*b).c2rust_unnamed.c2rust_unnamed_0.max_ytx as RectTxfmSize,
                            0 as libc::c_int,
                            tx_split.as_ptr(),
                            x_off,
                            y_off,
                            0 as *mut pixel,
                        );
                    } else {
                        let mut cf_ctx: uint8_t = 0x40 as libc::c_int as uint8_t;
                        let mut txtp: TxfmType = DCT_DCT;
                        let ref mut fresh4 = (*cbi.offset((*t).bx as isize)).eob[0];
                        *fresh4 = decode_coefs(
                            t,
                            &mut (*(*t).a).lcoef.0[(bx4 + x) as usize..],
                            &mut (*t).l.lcoef.0[(by4 + y) as usize..],
                            (*b).c2rust_unnamed.c2rust_unnamed.tx as RectTxfmSize,
                            bs,
                            b,
                            1 as libc::c_int,
                            0 as libc::c_int,
                            (*ts).frame_thread[1].cf as *mut coef,
                            &mut txtp,
                            &mut cf_ctx,
                        ) as int16_t;
                        let eob = *fresh4 as libc::c_int;
                        if DEBUG_BLOCK_INFO(&*f, &*t) {
                            printf(
                                b"Post-y-cf-blk[tx=%d,txtp=%d,eob=%d]: r=%d\n\0" as *const u8
                                    as *const libc::c_char,
                                (*b).c2rust_unnamed.c2rust_unnamed.tx as libc::c_int,
                                txtp as libc::c_uint,
                                eob,
                                (*ts).msac.rng,
                            );
                        }
                        (*cbi.offset((*t).bx as isize)).txtp[0] = txtp as uint8_t;
                        (*ts).frame_thread[1].cf = ((*ts).frame_thread[1].cf as *mut coef).offset(
                            (imin((*t_dim).w as libc::c_int, 8 as libc::c_int)
                                * imin((*t_dim).h as libc::c_int, 8 as libc::c_int)
                                * 16) as isize,
                        ) as *mut libc::c_void;
                        CaseSet::<16, true>::many(
                            [&mut (*t).l, &mut *(*t).a],
                            [
                                imin((*t_dim).h as i32, (*f).bh - (*t).by) as usize,
                                imin((*t_dim).w as i32, (*f).bw - (*t).bx) as usize,
                            ],
                            [(by4 + y) as usize, (bx4 + x) as usize],
                            |case, dir| {
                                case.set(&mut dir.lcoef.0, cf_ctx);
                            },
                        );
                    }
                    x += (*t_dim).w as libc::c_int;
                    (*t).bx += (*t_dim).w as libc::c_int;
                    x_off += 1;
                }
                (*t).bx -= x;
                y += (*t_dim).h as libc::c_int;
                (*t).by += (*t_dim).h as libc::c_int;
                y_off += 1;
            }
            (*t).by -= y;
            if !(has_chroma == 0) {
                let sub_ch4 = imin(ch4, init_y + 16 >> ss_ver);
                let sub_cw4 = imin(cw4, init_x + 16 >> ss_hor);
                let mut pl = 0;
                while pl < 2 {
                    y = init_y >> ss_ver;
                    (*t).by += init_y;
                    while y < sub_ch4 {
                        let cbi_0: *mut CodedBlockInfo = &mut *((*f).frame_thread.cbi)
                            .offset(((*t).by as isize * (*f).b4_stride) as isize)
                            as *mut CodedBlockInfo;
                        x = init_x >> ss_hor;
                        (*t).bx += init_x;
                        while x < sub_cw4 {
                            let mut cf_ctx_0: uint8_t = 0x40 as libc::c_int as uint8_t;
                            let mut txtp_0: TxfmType = DCT_DCT;
                            if (*b).intra == 0 {
                                txtp_0 = (*t).txtp_map
                                    [((by4 + (y << ss_ver)) * 32 + bx4 + (x << ss_hor)) as usize]
                                    as TxfmType;
                            }
                            let ref mut fresh5 =
                                (*cbi_0.offset((*t).bx as isize)).eob[(1 + pl) as usize];
                            *fresh5 = decode_coefs(
                                t,
                                &mut (*(*t).a).ccoef.0[pl as usize][(cbx4 + x) as usize..],
                                &mut (*t).l.ccoef.0[pl as usize][(cby4 + y) as usize..],
                                (*b).uvtx as RectTxfmSize,
                                bs,
                                b,
                                (*b).intra as libc::c_int,
                                1 + pl,
                                (*ts).frame_thread[1].cf as *mut coef,
                                &mut txtp_0,
                                &mut cf_ctx_0,
                            ) as int16_t;
                            let eob_0 = *fresh5 as libc::c_int;
                            if DEBUG_BLOCK_INFO(&*f, &*t) {
                                printf(
                                    b"Post-uv-cf-blk[pl=%d,tx=%d,txtp=%d,eob=%d]: r=%d\n\0"
                                        as *const u8
                                        as *const libc::c_char,
                                    pl,
                                    (*b).uvtx as libc::c_int,
                                    txtp_0 as libc::c_uint,
                                    eob_0,
                                    (*ts).msac.rng,
                                );
                            }
                            (*cbi_0.offset((*t).bx as isize)).txtp[(1 + pl) as usize] =
                                txtp_0 as uint8_t;
                            (*ts).frame_thread[1].cf =
                                ((*ts).frame_thread[1].cf as *mut coef).offset(
                                    ((*uv_t_dim).w as libc::c_int
                                        * (*uv_t_dim).h as libc::c_int
                                        * 16) as isize,
                                ) as *mut libc::c_void;
                            CaseSet::<16, true>::many(
                                [&mut (*t).l, &mut *(*t).a],
                                [
                                    imin((*uv_t_dim).h as i32, (*f).bh - (*t).by + ss_ver >> ss_ver)
                                        as usize,
                                    imin((*uv_t_dim).w as i32, (*f).bw - (*t).bx + ss_hor >> ss_hor)
                                        as usize,
                                ],
                                [(cby4 + y) as usize, (cbx4 + x) as usize],
                                |case, dir| {
                                    case.set(&mut dir.ccoef.0[pl as usize], cf_ctx_0);
                                },
                            );
                            x += (*uv_t_dim).w as libc::c_int;
                            (*t).bx += ((*uv_t_dim).w as libc::c_int) << ss_hor;
                        }
                        (*t).bx -= x << ss_hor;
                        y += (*uv_t_dim).h as libc::c_int;
                        (*t).by += ((*uv_t_dim).h as libc::c_int) << ss_ver;
                    }
                    (*t).by -= y << ss_ver;
                    pl += 1;
                }
            }
            init_x += 16 as libc::c_int;
        }
        init_y += 16 as libc::c_int;
    }
}
unsafe extern "C" fn mc(
    t: *mut Dav1dTaskContext,
    dst8: *mut pixel,
    dst16: *mut int16_t,
    dst_stride: ptrdiff_t,
    bw4: libc::c_int,
    bh4: libc::c_int,
    bx: libc::c_int,
    by: libc::c_int,
    pl: libc::c_int,
    mv: mv,
    refp: *const Dav1dThreadPicture,
    refidx: libc::c_int,
    filter_2d: Filter2d,
) -> libc::c_int {
    if (dst8 != 0 as *mut libc::c_void as *mut pixel) as libc::c_int
        ^ (dst16 != 0 as *mut libc::c_void as *mut int16_t) as libc::c_int
        == 0
    {
        unreachable!();
    }
    let f: *const Dav1dFrameContext = (*t).f;
    let ss_ver = (pl != 0
        && (*f).cur.p.layout as libc::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint)
        as libc::c_int;
    let ss_hor = (pl != 0
        && (*f).cur.p.layout as libc::c_uint
            != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint)
        as libc::c_int;
    let h_mul = 4 >> ss_hor;
    let v_mul = 4 >> ss_ver;
    let mvx = mv.x as libc::c_int;
    let mvy = mv.y as libc::c_int;
    let mx = mvx & 15 >> (ss_hor == 0) as libc::c_int;
    let my = mvy & 15 >> (ss_ver == 0) as libc::c_int;
    let mut ref_stride: ptrdiff_t = (*refp).p.stride[(pl != 0) as libc::c_int as usize];
    let mut r#ref: *const pixel = 0 as *const pixel;
    if (*refp).p.p.w == (*f).cur.p.w && (*refp).p.p.h == (*f).cur.p.h {
        let dx = bx * h_mul + (mvx >> 3 + ss_hor);
        let dy = by * v_mul + (mvy >> 3 + ss_ver);
        let mut w = 0;
        let mut h = 0;
        if (*refp).p.data[0] != (*f).cur.data[0] {
            w = (*f).cur.p.w + ss_hor >> ss_hor;
            h = (*f).cur.p.h + ss_ver >> ss_ver;
        } else {
            w = (*f).bw * 4 >> ss_hor;
            h = (*f).bh * 4 >> ss_ver;
        }
        if dx < (mx != 0) as libc::c_int * 3
            || dy < (my != 0) as libc::c_int * 3
            || dx + bw4 * h_mul + (mx != 0) as libc::c_int * 4 > w
            || dy + bh4 * v_mul + (my != 0) as libc::c_int * 4 > h
        {
            let emu_edge_buf: *mut pixel =
                ((*t).scratch.c2rust_unnamed.c2rust_unnamed_0.emu_edge_16bpc).as_mut_ptr();
            ((*(*f).dsp).mc.emu_edge).expect("non-null function pointer")(
                (bw4 * h_mul + (mx != 0) as libc::c_int * 7) as intptr_t,
                (bh4 * v_mul + (my != 0) as libc::c_int * 7) as intptr_t,
                w as intptr_t,
                h as intptr_t,
                (dx - (mx != 0) as libc::c_int * 3) as intptr_t,
                (dy - (my != 0) as libc::c_int * 3) as intptr_t,
                emu_edge_buf,
                (192 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong)
                    as ptrdiff_t,
                (*refp).p.data[pl as usize] as *const pixel,
                ref_stride,
            );
            r#ref = &mut *emu_edge_buf.offset(
                (192 * (my != 0) as libc::c_int * 3 + (mx != 0) as libc::c_int * 3) as isize,
            ) as *mut pixel;
            ref_stride = (192 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong)
                as ptrdiff_t;
        } else {
            r#ref = ((*refp).p.data[pl as usize] as *mut pixel)
                .offset(PXSTRIDE(ref_stride) * dy as isize)
                .offset(dx as isize);
        }
        if !dst8.is_null() {
            ((*(*f).dsp).mc.mc[filter_2d as usize]).expect("non-null function pointer")(
                dst8,
                dst_stride,
                r#ref,
                ref_stride,
                bw4 * h_mul,
                bh4 * v_mul,
                mx << (ss_hor == 0) as libc::c_int,
                my << (ss_ver == 0) as libc::c_int,
                (*f).bitdepth_max,
            );
        } else {
            ((*(*f).dsp).mc.mct[filter_2d as usize]).expect("non-null function pointer")(
                dst16,
                r#ref,
                ref_stride,
                bw4 * h_mul,
                bh4 * v_mul,
                mx << (ss_hor == 0) as libc::c_int,
                my << (ss_ver == 0) as libc::c_int,
                (*f).bitdepth_max,
            );
        }
    } else {
        if !(refp != &(*f).sr_cur as *const Dav1dThreadPicture) {
            unreachable!();
        }
        let orig_pos_y =
            (by * v_mul << 4) + mvy * ((1 as libc::c_int) << (ss_ver == 0) as libc::c_int);
        let orig_pos_x =
            (bx * h_mul << 4) + mvx * ((1 as libc::c_int) << (ss_hor == 0) as libc::c_int);
        let mut pos_y = 0;
        let mut pos_x = 0;
        let tmp: int64_t = orig_pos_x as int64_t * (*f).svc[refidx as usize][0].scale as int64_t
            + (((*f).svc[refidx as usize][0].scale - 0x4000 as libc::c_int) * 8) as int64_t;
        pos_x = apply_sign64(
            ((tmp as libc::c_longlong).abs() + 128 as libc::c_longlong >> 8) as libc::c_int,
            tmp,
        ) + 32;
        let tmp_0: int64_t = orig_pos_y as int64_t * (*f).svc[refidx as usize][1].scale as int64_t
            + (((*f).svc[refidx as usize][1].scale - 0x4000 as libc::c_int) * 8) as int64_t;
        pos_y = apply_sign64(
            ((tmp_0 as libc::c_longlong).abs() + 128 as libc::c_longlong >> 8) as libc::c_int,
            tmp_0,
        ) + 32;
        let left = pos_x >> 10;
        let top = pos_y >> 10;
        let right = (pos_x + (bw4 * h_mul - 1) * (*f).svc[refidx as usize][0].step >> 10) + 1;
        let bottom = (pos_y + (bh4 * v_mul - 1) * (*f).svc[refidx as usize][1].step >> 10) + 1;
        if DEBUG_BLOCK_INFO(&*f, &*t) {
            printf(
                b"Off %dx%d [%d,%d,%d], size %dx%d [%d,%d]\n\0" as *const u8 as *const libc::c_char,
                left,
                top,
                orig_pos_x,
                (*f).svc[refidx as usize][0].scale,
                refidx,
                right - left,
                bottom - top,
                (*f).svc[refidx as usize][0].step,
                (*f).svc[refidx as usize][1].step,
            );
        }
        let w_0 = (*refp).p.p.w + ss_hor >> ss_hor;
        let h_0 = (*refp).p.p.h + ss_ver >> ss_ver;
        if left < 3 || top < 3 || right + 4 > w_0 || bottom + 4 > h_0 {
            let emu_edge_buf_0: *mut pixel =
                ((*t).scratch.c2rust_unnamed.c2rust_unnamed_0.emu_edge_16bpc).as_mut_ptr();
            ((*(*f).dsp).mc.emu_edge).expect("non-null function pointer")(
                (right - left + 7) as intptr_t,
                (bottom - top + 7) as intptr_t,
                w_0 as intptr_t,
                h_0 as intptr_t,
                (left - 3) as intptr_t,
                (top - 3) as intptr_t,
                emu_edge_buf_0,
                (320 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong)
                    as ptrdiff_t,
                (*refp).p.data[pl as usize] as *const pixel,
                ref_stride,
            );
            r#ref = &mut *emu_edge_buf_0.offset((320 * 3 + 3) as isize) as *mut pixel;
            ref_stride = (320 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong)
                as ptrdiff_t;
            if DEBUG_BLOCK_INFO(&*f, &*t) {
                printf(b"Emu\n\0" as *const u8 as *const libc::c_char);
            }
        } else {
            r#ref = ((*refp).p.data[pl as usize] as *mut pixel)
                .offset(PXSTRIDE(ref_stride) * top as isize)
                .offset(left as isize);
        }
        if !dst8.is_null() {
            ((*(*f).dsp).mc.mc_scaled[filter_2d as usize]).expect("non-null function pointer")(
                dst8,
                dst_stride,
                r#ref,
                ref_stride,
                bw4 * h_mul,
                bh4 * v_mul,
                pos_x & 0x3ff as libc::c_int,
                pos_y & 0x3ff as libc::c_int,
                (*f).svc[refidx as usize][0].step,
                (*f).svc[refidx as usize][1].step,
                (*f).bitdepth_max,
            );
        } else {
            ((*(*f).dsp).mc.mct_scaled[filter_2d as usize]).expect("non-null function pointer")(
                dst16,
                r#ref,
                ref_stride,
                bw4 * h_mul,
                bh4 * v_mul,
                pos_x & 0x3ff as libc::c_int,
                pos_y & 0x3ff as libc::c_int,
                (*f).svc[refidx as usize][0].step,
                (*f).svc[refidx as usize][1].step,
                (*f).bitdepth_max,
            );
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn obmc(
    t: *mut Dav1dTaskContext,
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    b_dim: *const uint8_t,
    pl: libc::c_int,
    bx4: libc::c_int,
    by4: libc::c_int,
    w4: libc::c_int,
    h4: libc::c_int,
) -> libc::c_int {
    if !((*t).bx & 1 == 0 && (*t).by & 1 == 0) {
        unreachable!();
    }
    let f: *const Dav1dFrameContext = (*t).f;
    let mut r: *mut *mut refmvs_block = &mut *((*t).rt.r)
        .as_mut_ptr()
        .offset((((*t).by & 31) + 5) as isize)
        as *mut *mut refmvs_block;
    let lap: *mut pixel = ((*t).scratch.c2rust_unnamed.c2rust_unnamed.lap_16bpc).as_mut_ptr();
    let ss_ver = (pl != 0
        && (*f).cur.p.layout as libc::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint)
        as libc::c_int;
    let ss_hor = (pl != 0
        && (*f).cur.p.layout as libc::c_uint
            != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint)
        as libc::c_int;
    let h_mul = 4 >> ss_hor;
    let v_mul = 4 >> ss_ver;
    let mut res = 0;
    if (*t).by > (*(*t).ts).tiling.row_start
        && (pl == 0
            || *b_dim.offset(0) as libc::c_int * h_mul + *b_dim.offset(1) as libc::c_int * v_mul
                >= 16)
    {
        let mut i = 0;
        let mut x = 0;
        while x < w4 && i < imin(*b_dim.offset(2) as libc::c_int, 4 as libc::c_int) {
            let a_r: *const refmvs_block = &mut *(*r.offset(-(1 as libc::c_int) as isize))
                .offset(((*t).bx + x + 1) as isize)
                as *mut refmvs_block;
            let a_b_dim: *const uint8_t = (dav1d_block_dimensions[(*a_r).0.bs as usize]).as_ptr();
            let step4 = iclip(
                *a_b_dim.offset(0) as libc::c_int,
                2 as libc::c_int,
                16 as libc::c_int,
            );
            if (*a_r).0.r#ref.r#ref[0] as libc::c_int > 0 {
                let ow4 = imin(step4, *b_dim.offset(0) as libc::c_int);
                let oh4 = imin(*b_dim.offset(1) as libc::c_int, 16 as libc::c_int) >> 1;
                res = mc(
                    t,
                    lap,
                    0 as *mut int16_t,
                    ((ow4 * h_mul) as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong)
                        as ptrdiff_t,
                    ow4,
                    oh4 * 3 + 3 >> 2,
                    (*t).bx + x,
                    (*t).by,
                    pl,
                    (*a_r).0.mv.mv[0],
                    &*((*f).refp).as_ptr().offset(
                        (*((*a_r).0.r#ref.r#ref).as_ptr().offset(0) as libc::c_int - 1) as isize,
                    ),
                    (*a_r).0.r#ref.r#ref[0] as libc::c_int - 1,
                    dav1d_filter_2d[(*(*t).a).filter[1][(bx4 + x + 1) as usize] as usize]
                        [(*(*t).a).filter[0][(bx4 + x + 1) as usize] as usize]
                        as Filter2d,
                );
                if res != 0 {
                    return res;
                }
                ((*(*f).dsp).mc.blend_h).expect("non-null function pointer")(
                    &mut *dst.offset((x * h_mul) as isize),
                    dst_stride,
                    lap,
                    h_mul * ow4,
                    v_mul * oh4,
                );
                i += 1;
            }
            x += step4;
        }
    }
    if (*t).bx > (*(*t).ts).tiling.col_start {
        let mut i_0 = 0;
        let mut y = 0;
        while y < h4 && i_0 < imin(*b_dim.offset(3) as libc::c_int, 4 as libc::c_int) {
            let l_r: *const refmvs_block = &mut *(*r.offset((y + 1) as isize))
                .offset(((*t).bx - 1) as isize)
                as *mut refmvs_block;
            let l_b_dim: *const uint8_t = (dav1d_block_dimensions[(*l_r).0.bs as usize]).as_ptr();
            let step4_0 = iclip(
                *l_b_dim.offset(1) as libc::c_int,
                2 as libc::c_int,
                16 as libc::c_int,
            );
            if (*l_r).0.r#ref.r#ref[0] as libc::c_int > 0 {
                let ow4_0 = imin(*b_dim.offset(0) as libc::c_int, 16 as libc::c_int) >> 1;
                let oh4_0 = imin(step4_0, *b_dim.offset(1) as libc::c_int);
                res = mc(
                    t,
                    lap,
                    0 as *mut int16_t,
                    ((h_mul * ow4_0) as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong)
                        as ptrdiff_t,
                    ow4_0,
                    oh4_0,
                    (*t).bx,
                    (*t).by + y,
                    pl,
                    (*l_r).0.mv.mv[0],
                    &*((*f).refp).as_ptr().offset(
                        (*((*l_r).0.r#ref.r#ref).as_ptr().offset(0) as libc::c_int - 1) as isize,
                    ),
                    (*l_r).0.r#ref.r#ref[0] as libc::c_int - 1,
                    dav1d_filter_2d[(*t).l.filter[1][(by4 + y + 1) as usize] as usize]
                        [(*t).l.filter[0][(by4 + y + 1) as usize] as usize]
                        as Filter2d,
                );
                if res != 0 {
                    return res;
                }
                ((*(*f).dsp).mc.blend_v).expect("non-null function pointer")(
                    &mut *dst.offset(
                        ((y * v_mul) as isize
                            * (PXSTRIDE as unsafe extern "C" fn(ptrdiff_t) -> ptrdiff_t)(
                                dst_stride,
                            )) as isize,
                    ),
                    dst_stride,
                    lap,
                    h_mul * ow4_0,
                    v_mul * oh4_0,
                );
                i_0 += 1;
            }
            y += step4_0;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn warp_affine(
    t: *mut Dav1dTaskContext,
    mut dst8: *mut pixel,
    mut dst16: *mut int16_t,
    dstride: ptrdiff_t,
    b_dim: *const uint8_t,
    pl: libc::c_int,
    refp: *const Dav1dThreadPicture,
    wmp: *const Dav1dWarpedMotionParams,
) -> libc::c_int {
    if (dst8 != 0 as *mut libc::c_void as *mut pixel) as libc::c_int
        ^ (dst16 != 0 as *mut libc::c_void as *mut int16_t) as libc::c_int
        == 0
    {
        unreachable!();
    }
    let f: *const Dav1dFrameContext = (*t).f;
    let dsp: *const Dav1dDSPContext = (*f).dsp;
    let ss_ver = (pl != 0
        && (*f).cur.p.layout as libc::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint)
        as libc::c_int;
    let ss_hor = (pl != 0
        && (*f).cur.p.layout as libc::c_uint
            != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint)
        as libc::c_int;
    let h_mul = 4 >> ss_hor;
    let v_mul = 4 >> ss_ver;
    if !(*b_dim.offset(0) as libc::c_int * h_mul & 7 == 0
        && *b_dim.offset(1) as libc::c_int * v_mul & 7 == 0)
    {
        unreachable!();
    }
    let mat: *const int32_t = ((*wmp).matrix).as_ptr();
    let width = (*refp).p.p.w + ss_hor >> ss_hor;
    let height = (*refp).p.p.h + ss_ver >> ss_ver;
    let mut y = 0;
    while y < *b_dim.offset(1) as libc::c_int * v_mul {
        let src_y = (*t).by * 4 + ((y + 4) << ss_ver);
        let mat3_y: int64_t =
            *mat.offset(3) as int64_t * src_y as int64_t + *mat.offset(0) as int64_t;
        let mat5_y: int64_t =
            *mat.offset(5) as int64_t * src_y as int64_t + *mat.offset(1) as int64_t;
        let mut x = 0;
        while x < *b_dim.offset(0) as libc::c_int * h_mul {
            let src_x = (*t).bx * 4 + ((x + 4) << ss_hor);
            let mvx: int64_t = *mat.offset(2) as int64_t * src_x as int64_t + mat3_y >> ss_hor;
            let mvy: int64_t = *mat.offset(4) as int64_t * src_x as int64_t + mat5_y >> ss_ver;
            let dx = (mvx >> 16) as libc::c_int - 4;
            let mx = (mvx as libc::c_int & 0xffff as libc::c_int)
                - (*wmp).alpha() as libc::c_int * 4
                - (*wmp).beta() as libc::c_int * 7
                & !(0x3f as libc::c_int);
            let dy = (mvy >> 16) as libc::c_int - 4;
            let my = (mvy as libc::c_int & 0xffff as libc::c_int)
                - (*wmp).gamma() as libc::c_int * 4
                - (*wmp).delta() as libc::c_int * 4
                & !(0x3f as libc::c_int);
            let mut ref_ptr: *const pixel = 0 as *const pixel;
            let mut ref_stride: ptrdiff_t = (*refp).p.stride[(pl != 0) as libc::c_int as usize];
            if dx < 3 || dx + 8 + 4 > width || dy < 3 || dy + 8 + 4 > height {
                let emu_edge_buf: *mut pixel =
                    ((*t).scratch.c2rust_unnamed.c2rust_unnamed_0.emu_edge_16bpc).as_mut_ptr();
                ((*(*f).dsp).mc.emu_edge).expect("non-null function pointer")(
                    15 as libc::c_int as intptr_t,
                    15 as libc::c_int as intptr_t,
                    width as intptr_t,
                    height as intptr_t,
                    (dx - 3) as intptr_t,
                    (dy - 3) as intptr_t,
                    emu_edge_buf,
                    (32 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong)
                        as ptrdiff_t,
                    (*refp).p.data[pl as usize] as *const pixel,
                    ref_stride,
                );
                ref_ptr = &mut *emu_edge_buf.offset((32 * 3 + 3) as isize) as *mut pixel;
                ref_stride = (32 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong)
                    as ptrdiff_t;
            } else {
                ref_ptr = ((*refp).p.data[pl as usize] as *mut pixel)
                    .offset((PXSTRIDE(ref_stride) * dy as isize) as isize)
                    .offset(dx as isize);
            }
            if !dst16.is_null() {
                ((*dsp).mc.warp8x8t).expect("non-null function pointer")(
                    &mut *dst16.offset(x as isize),
                    dstride,
                    ref_ptr,
                    ref_stride,
                    ((*wmp).abcd).as_ptr(),
                    mx,
                    my,
                    (*f).bitdepth_max,
                );
            } else {
                ((*dsp).mc.warp8x8).expect("non-null function pointer")(
                    &mut *dst8.offset(x as isize),
                    dstride,
                    ref_ptr,
                    ref_stride,
                    ((*wmp).abcd).as_ptr(),
                    mx,
                    my,
                    (*f).bitdepth_max,
                );
            }
            x += 8 as libc::c_int;
        }
        if !dst8.is_null() {
            dst8 = dst8.offset((8 * PXSTRIDE(dstride)) as isize);
        } else {
            dst16 = dst16.offset((8 * dstride) as isize);
        }
        y += 8 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_recon_b_intra_16bpc(
    t: *mut Dav1dTaskContext,
    bs: BlockSize,
    intra_edge_flags: EdgeFlags,
    b: *const Av1Block,
) {
    let ts: *mut Dav1dTileState = (*t).ts;
    let f: *const Dav1dFrameContext = (*t).f;
    let dsp: *const Dav1dDSPContext = (*f).dsp;
    let bx4 = (*t).bx & 31;
    let by4 = (*t).by & 31;
    let ss_ver = ((*f).cur.p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_hor = ((*f).cur.p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
    let cbx4 = bx4 >> ss_hor;
    let cby4 = by4 >> ss_ver;
    let b_dim: *const uint8_t = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bw4 = *b_dim.offset(0) as libc::c_int;
    let bh4 = *b_dim.offset(1) as libc::c_int;
    let w4 = imin(bw4, (*f).bw - (*t).bx);
    let h4 = imin(bh4, (*f).bh - (*t).by);
    let cw4 = w4 + ss_hor >> ss_hor;
    let ch4 = h4 + ss_ver >> ss_ver;
    let has_chroma = ((*f).cur.p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint
        && (bw4 > ss_hor || (*t).bx & 1 != 0)
        && (bh4 > ss_ver || (*t).by & 1 != 0)) as libc::c_int;
    let t_dim: *const TxfmInfo = &*dav1d_txfm_dimensions
        .as_ptr()
        .offset((*b).c2rust_unnamed.c2rust_unnamed.tx as isize)
        as *const TxfmInfo;
    let uv_t_dim: *const TxfmInfo =
        &*dav1d_txfm_dimensions.as_ptr().offset((*b).uvtx as isize) as *const TxfmInfo;
    let edge: *mut pixel = ((*t)
        .scratch
        .c2rust_unnamed_0
        .c2rust_unnamed_0
        .c2rust_unnamed_0
        .edge_16bpc)
        .as_mut_ptr()
        .offset(128);
    let cbw4 = bw4 + ss_hor >> ss_hor;
    let cbh4 = bh4 + ss_ver >> ss_ver;
    let intra_edge_filter_flag = (*(*f).seq_hdr).intra_edge_filter << 10;
    let mut init_y = 0;
    while init_y < h4 {
        let sub_h4 = imin(h4, 16 + init_y);
        let sub_ch4 = imin(ch4, init_y + 16 >> ss_ver);
        let mut init_x = 0;
        while init_x < w4 {
            if (*b).c2rust_unnamed.c2rust_unnamed.pal_sz[0] != 0 {
                let mut dst: *mut pixel = ((*f).cur.data[0] as *mut pixel).offset(
                    (4 * ((*t).by as isize * PXSTRIDE((*f).cur.stride[0]) + (*t).bx as isize))
                        as isize,
                );
                let mut pal_idx: *const uint8_t = 0 as *const uint8_t;
                if (*t).frame_thread.pass != 0 {
                    let p = (*t).frame_thread.pass & 1;
                    if ((*ts).frame_thread[p as usize].pal_idx).is_null() {
                        unreachable!();
                    }
                    pal_idx = (*ts).frame_thread[p as usize].pal_idx;
                    (*ts).frame_thread[p as usize].pal_idx =
                        ((*ts).frame_thread[p as usize].pal_idx).offset((bw4 * bh4 * 16) as isize);
                } else {
                    pal_idx = ((*t).scratch.c2rust_unnamed_0.pal_idx).as_mut_ptr();
                }
                let pal: *const uint16_t = if (*t).frame_thread.pass != 0 {
                    ((*((*f).frame_thread.pal).offset(
                        ((((*t).by as isize >> 1) + ((*t).bx as isize & 1)) * ((*f).b4_stride >> 1)
                            + (((*t).bx >> 1) + ((*t).by & 1)) as isize)
                            as isize,
                    ))[0])
                        .as_mut_ptr()
                } else {
                    ((*t).scratch.c2rust_unnamed_0.pal[0]).as_mut_ptr()
                };
                ((*(*f).dsp).ipred.pal_pred).expect("non-null function pointer")(
                    dst,
                    (*f).cur.stride[0],
                    pal,
                    pal_idx,
                    bw4 * 4,
                    bh4 * 4,
                );
                if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                    hex_dump::<BitDepth16>(
                        dst,
                        PXSTRIDE((*f).cur.stride[0]) as usize,
                        bw4 as usize * 4,
                        bh4 as usize * 4,
                        "y-pal-pred",
                    );
                }
            }
            let intra_flags =
                sm_flag((*t).a, bx4) | sm_flag(&mut (*t).l, by4) | intra_edge_filter_flag;
            let sb_has_tr = (if (init_x + 16) < w4 {
                1 as libc::c_int as libc::c_uint
            } else if init_y != 0 {
                0 as libc::c_int as libc::c_uint
            } else {
                intra_edge_flags as libc::c_uint
                    & EDGE_I444_TOP_HAS_RIGHT as libc::c_int as libc::c_uint
            }) as libc::c_int;
            let sb_has_bl = (if init_x != 0 {
                0 as libc::c_int as libc::c_uint
            } else if (init_y + 16) < h4 {
                1 as libc::c_int as libc::c_uint
            } else {
                intra_edge_flags as libc::c_uint
                    & EDGE_I444_LEFT_HAS_BOTTOM as libc::c_int as libc::c_uint
            }) as libc::c_int;
            let mut y = 0;
            let mut x = 0;
            let sub_w4 = imin(w4, init_x + 16);
            y = init_y;
            (*t).by += init_y;
            while y < sub_h4 {
                let mut dst_0: *mut pixel = ((*f).cur.data[0] as *mut pixel).offset(
                    (4 * ((*t).by as isize * PXSTRIDE((*f).cur.stride[0])
                        + (*t).bx as isize
                        + init_x as isize)) as isize,
                );
                x = init_x;
                (*t).bx += init_x;
                while x < sub_w4 {
                    let mut angle = 0;
                    let mut edge_flags: EdgeFlags = 0 as EdgeFlags;
                    let mut top_sb_edge: *const pixel = 0 as *const pixel;
                    let mut m: IntraPredMode = DC_PRED;
                    if !((*b).c2rust_unnamed.c2rust_unnamed.pal_sz[0] != 0) {
                        angle = (*b).c2rust_unnamed.c2rust_unnamed.y_angle as libc::c_int;
                        edge_flags = ((if (y > init_y || sb_has_tr == 0)
                            && x + (*t_dim).w as libc::c_int >= sub_w4
                        {
                            0 as libc::c_int
                        } else {
                            EDGE_I444_TOP_HAS_RIGHT as libc::c_int
                        }) | (if x > init_x
                            || sb_has_bl == 0 && y + (*t_dim).h as libc::c_int >= sub_h4
                        {
                            0 as libc::c_int
                        } else {
                            EDGE_I444_LEFT_HAS_BOTTOM as libc::c_int
                        })) as EdgeFlags;
                        top_sb_edge = 0 as *const pixel;
                        if (*t).by & (*f).sb_step - 1 == 0 {
                            top_sb_edge = (*f).ipred_edge[0] as *mut pixel;
                            let sby = (*t).by >> (*f).sb_shift;
                            top_sb_edge =
                                top_sb_edge.offset(((*f).sb128w * 128 * (sby - 1)) as isize);
                        }
                        m = dav1d_prepare_intra_edges_16bpc(
                            (*t).bx,
                            ((*t).bx > (*ts).tiling.col_start) as libc::c_int,
                            (*t).by,
                            ((*t).by > (*ts).tiling.row_start) as libc::c_int,
                            (*ts).tiling.col_end,
                            (*ts).tiling.row_end,
                            edge_flags,
                            dst_0,
                            (*f).cur.stride[0],
                            top_sb_edge,
                            (*b).c2rust_unnamed.c2rust_unnamed.y_mode as IntraPredMode,
                            &mut angle,
                            (*t_dim).w as libc::c_int,
                            (*t_dim).h as libc::c_int,
                            (*(*f).seq_hdr).intra_edge_filter,
                            edge,
                            (*f).bitdepth_max,
                        );
                        ((*dsp).ipred.intra_pred[m as usize]).expect("non-null function pointer")(
                            dst_0,
                            (*f).cur.stride[0],
                            edge,
                            (*t_dim).w as libc::c_int * 4,
                            (*t_dim).h as libc::c_int * 4,
                            angle | intra_flags,
                            4 * (*f).bw - 4 * (*t).bx,
                            4 * (*f).bh - 4 * (*t).by,
                            (*f).bitdepth_max,
                        );
                        if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                            hex_dump::<BitDepth16>(
                                edge.offset(-(((*t_dim).h as libc::c_int * 4) as isize)),
                                (*t_dim).h as usize * 4,
                                (*t_dim).h as usize * 4,
                                2,
                                "l",
                            );
                            hex_dump::<BitDepth16>(edge, 0, 1, 1, "tl");
                            hex_dump::<BitDepth16>(
                                edge.offset(1),
                                (*t_dim).w as usize * 4,
                                (*t_dim).w as usize * 4,
                                2,
                                "t",
                            );
                            hex_dump::<BitDepth16>(
                                dst_0,
                                (*f).cur.stride[0] as usize,
                                (*t_dim).w as usize * 4,
                                (*t_dim).h as usize * 4,
                                "y-intra-pred",
                            );
                        }
                    }
                    if (*b).skip == 0 {
                        let mut cf: *mut coef = 0 as *mut coef;
                        let mut eob = 0;
                        let mut txtp: TxfmType = DCT_DCT;
                        if (*t).frame_thread.pass != 0 {
                            let p_0 = (*t).frame_thread.pass & 1;
                            cf = (*ts).frame_thread[p_0 as usize].cf as *mut coef;
                            (*ts).frame_thread[p_0 as usize].cf =
                                ((*ts).frame_thread[p_0 as usize].cf as *mut coef).offset(
                                    (imin((*t_dim).w as libc::c_int, 8 as libc::c_int)
                                        * imin((*t_dim).h as libc::c_int, 8 as libc::c_int)
                                        * 16) as isize,
                                ) as *mut libc::c_void;
                            let cbi: *const CodedBlockInfo = &mut *((*f).frame_thread.cbi).offset(
                                ((*t).by as isize * (*f).b4_stride + (*t).bx as isize) as isize,
                            )
                                as *mut CodedBlockInfo;
                            eob = (*cbi).eob[0] as libc::c_int;
                            txtp = (*cbi).txtp[0] as TxfmType;
                        } else {
                            let mut cf_ctx: uint8_t = 0;
                            cf = ((*t).c2rust_unnamed.cf_16bpc).as_mut_ptr();
                            eob = decode_coefs(
                                t,
                                &mut (*(*t).a).lcoef.0[(bx4 + x) as usize..],
                                &mut (*t).l.lcoef.0[(by4 + y) as usize..],
                                (*b).c2rust_unnamed.c2rust_unnamed.tx as RectTxfmSize,
                                bs,
                                b,
                                1 as libc::c_int,
                                0 as libc::c_int,
                                cf,
                                &mut txtp,
                                &mut cf_ctx,
                            );
                            if DEBUG_BLOCK_INFO(&*f, &*t) {
                                printf(
                                    b"Post-y-cf-blk[tx=%d,txtp=%d,eob=%d]: r=%d\n\0" as *const u8
                                        as *const libc::c_char,
                                    (*b).c2rust_unnamed.c2rust_unnamed.tx as libc::c_int,
                                    txtp as libc::c_uint,
                                    eob,
                                    (*ts).msac.rng,
                                );
                            }
                            CaseSet::<16, true>::many(
                                [&mut (*t).l, &mut *(*t).a],
                                [
                                    imin((*t_dim).h as i32, (*f).bh - (*t).by) as usize,
                                    imin((*t_dim).w as i32, (*f).bw - (*t).bx) as usize,
                                ],
                                [(by4 + y) as usize, (bx4 + x) as usize],
                                |case, dir| {
                                    case.set(&mut dir.lcoef.0, cf_ctx);
                                },
                            );
                        }
                        if eob >= 0 {
                            if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                                coef_dump(
                                    cf,
                                    std::cmp::min((*t_dim).h as usize, 8) * 4,
                                    std::cmp::min((*t_dim).w as usize, 8) * 4,
                                    3,
                                    "dq",
                                );
                            }
                            ((*dsp).itx.itxfm_add[(*b).c2rust_unnamed.c2rust_unnamed.tx as usize]
                                [txtp as usize])
                                .expect("non-null function pointer")(
                                dst_0,
                                (*f).cur.stride[0],
                                cf,
                                eob,
                                (*f).bitdepth_max,
                            );
                            if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                                hex_dump::<BitDepth16>(
                                    dst_0,
                                    (*f).cur.stride[0] as usize,
                                    (*t_dim).w as usize * 4,
                                    (*t_dim).h as usize * 4,
                                    "recon",
                                );
                            }
                        }
                    } else if (*t).frame_thread.pass == 0 {
                        CaseSet::<16, false>::many(
                            [&mut (*t).l, &mut *(*t).a],
                            [(*t_dim).h as usize, (*t_dim).w as usize],
                            [(by4 + y) as usize, (bx4 + x) as usize],
                            |case, dir| {
                                case.set(&mut dir.lcoef.0, 0x40);
                            },
                        );
                    }
                    dst_0 = dst_0.offset((4 * (*t_dim).w as libc::c_int) as isize);
                    x += (*t_dim).w as libc::c_int;
                    (*t).bx += (*t_dim).w as libc::c_int;
                }
                (*t).bx -= x;
                y += (*t_dim).h as libc::c_int;
                (*t).by += (*t_dim).h as libc::c_int;
            }
            (*t).by -= y;
            if !(has_chroma == 0) {
                let stride: ptrdiff_t = (*f).cur.stride[1];
                if (*b).c2rust_unnamed.c2rust_unnamed.uv_mode as libc::c_int
                    == CFL_PRED as libc::c_int
                {
                    if !(init_x == 0 && init_y == 0) {
                        unreachable!();
                    }
                    let ac = &mut (*t).scratch.c2rust_unnamed_0.ac;
                    let mut y_src: *mut pixel = ((*f).cur.data[0] as *mut pixel)
                        .offset((4 * ((*t).bx & !ss_hor)) as isize)
                        .offset(
                            ((4 * ((*t).by & !ss_ver)) as isize * PXSTRIDE((*f).cur.stride[0]))
                                as isize,
                        );
                    let uv_off: ptrdiff_t = 4
                        * (((*t).bx >> ss_hor) as isize
                            + ((*t).by >> ss_ver) as isize * PXSTRIDE(stride));
                    let uv_dst: [*mut pixel; 2] = [
                        ((*f).cur.data[1] as *mut pixel).offset(uv_off as isize),
                        ((*f).cur.data[2] as *mut pixel).offset(uv_off as isize),
                    ];
                    let furthest_r = (cw4 << ss_hor) + (*t_dim).w as libc::c_int - 1
                        & !((*t_dim).w as libc::c_int - 1);
                    let furthest_b = (ch4 << ss_ver) + (*t_dim).h as libc::c_int - 1
                        & !((*t_dim).h as libc::c_int - 1);
                    ((*dsp).ipred.cfl_ac[((*f).cur.p.layout as libc::c_uint)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        as usize])
                        .expect("non-null function pointer")(
                        ac.as_mut_ptr(),
                        y_src,
                        (*f).cur.stride[0],
                        cbw4 - (furthest_r >> ss_hor),
                        cbh4 - (furthest_b >> ss_ver),
                        cbw4 * 4,
                        cbh4 * 4,
                    );
                    let mut pl = 0;
                    while pl < 2 {
                        if !((*b).c2rust_unnamed.c2rust_unnamed.cfl_alpha[pl as usize] == 0) {
                            let mut angle_0 = 0;
                            let mut top_sb_edge_0: *const pixel = 0 as *const pixel;
                            if (*t).by & !ss_ver & (*f).sb_step - 1 == 0 {
                                top_sb_edge_0 = (*f).ipred_edge[(pl + 1) as usize] as *mut pixel;
                                let sby_0 = (*t).by >> (*f).sb_shift;
                                top_sb_edge_0 = top_sb_edge_0
                                    .offset(((*f).sb128w * 128 * (sby_0 - 1)) as isize);
                            }
                            let xpos = (*t).bx >> ss_hor;
                            let ypos = (*t).by >> ss_ver;
                            let xstart = (*ts).tiling.col_start >> ss_hor;
                            let ystart = (*ts).tiling.row_start >> ss_ver;
                            let m_0: IntraPredMode = dav1d_prepare_intra_edges_16bpc(
                                xpos,
                                (xpos > xstart) as libc::c_int,
                                ypos,
                                (ypos > ystart) as libc::c_int,
                                (*ts).tiling.col_end >> ss_hor,
                                (*ts).tiling.row_end >> ss_ver,
                                0 as EdgeFlags,
                                uv_dst[pl as usize],
                                stride,
                                top_sb_edge_0,
                                DC_PRED,
                                &mut angle_0,
                                (*uv_t_dim).w as libc::c_int,
                                (*uv_t_dim).h as libc::c_int,
                                0 as libc::c_int,
                                edge,
                                (*f).bitdepth_max,
                            );
                            ((*dsp).ipred.cfl_pred[m_0 as usize])
                                .expect("non-null function pointer")(
                                uv_dst[pl as usize],
                                stride,
                                edge,
                                (*uv_t_dim).w as libc::c_int * 4,
                                (*uv_t_dim).h as libc::c_int * 4,
                                ac.as_mut_ptr(),
                                (*b).c2rust_unnamed.c2rust_unnamed.cfl_alpha[pl as usize]
                                    as libc::c_int,
                                (*f).bitdepth_max,
                            );
                        }
                        pl += 1;
                    }
                    if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                        ac_dump(ac, 4 * cbw4 as usize, 4 * cbh4 as usize, "ac");
                        hex_dump::<BitDepth16>(
                            uv_dst[0],
                            stride as usize,
                            cbw4 as usize * 4,
                            cbh4 as usize * 4,
                            "u-cfl-pred",
                        );
                        hex_dump::<BitDepth16>(
                            uv_dst[1],
                            stride as usize,
                            cbw4 as usize * 4,
                            cbh4 as usize * 4,
                            "v-cfl-pred",
                        );
                    }
                } else if (*b).c2rust_unnamed.c2rust_unnamed.pal_sz[1] != 0 {
                    let uv_dstoff: ptrdiff_t = 4
                        * (((*t).bx >> ss_hor) as isize
                            + ((*t).by >> ss_ver) as isize * PXSTRIDE((*f).cur.stride[1]));
                    let mut pal_0: *const [uint16_t; 8] = 0 as *const [uint16_t; 8];
                    let mut pal_idx_0: *const uint8_t = 0 as *const uint8_t;
                    if (*t).frame_thread.pass != 0 {
                        let p_1 = (*t).frame_thread.pass & 1;
                        if ((*ts).frame_thread[p_1 as usize].pal_idx).is_null() {
                            unreachable!();
                        }
                        pal_0 = (*((*f).frame_thread.pal).offset(
                            ((((*t).by >> 1) + ((*t).bx & 1)) as isize * ((*f).b4_stride >> 1)
                                + (((*t).bx as isize >> 1) as isize + ((*t).by as isize & 1))
                                    as isize) as isize,
                        ))
                        .as_mut_ptr() as *const [uint16_t; 8];
                        pal_idx_0 = (*ts).frame_thread[p_1 as usize].pal_idx;
                        (*ts).frame_thread[p_1 as usize].pal_idx =
                            ((*ts).frame_thread[p_1 as usize].pal_idx)
                                .offset((cbw4 * cbh4 * 16) as isize);
                    } else {
                        pal_0 = ((*t).scratch.c2rust_unnamed_0.pal).as_mut_ptr()
                            as *const [uint16_t; 8];
                        pal_idx_0 = &mut *((*t).scratch.c2rust_unnamed_0.pal_idx)
                            .as_mut_ptr()
                            .offset((bw4 * bh4 * 16) as isize)
                            as *mut uint8_t;
                    }
                    ((*(*f).dsp).ipred.pal_pred).expect("non-null function pointer")(
                        ((*f).cur.data[1] as *mut pixel).offset(uv_dstoff as isize),
                        (*f).cur.stride[1],
                        (*pal_0.offset(1)).as_ptr(),
                        pal_idx_0,
                        cbw4 * 4,
                        cbh4 * 4,
                    );
                    ((*(*f).dsp).ipred.pal_pred).expect("non-null function pointer")(
                        ((*f).cur.data[2] as *mut pixel).offset(uv_dstoff as isize),
                        (*f).cur.stride[1],
                        (*pal_0.offset(2)).as_ptr(),
                        pal_idx_0,
                        cbw4 * 4,
                        cbh4 * 4,
                    );
                    if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                        hex_dump::<BitDepth16>(
                            ((*f).cur.data[1] as *mut pixel).offset(uv_dstoff as isize),
                            PXSTRIDE((*f).cur.stride[1]) as usize,
                            cbw4 as usize * 4,
                            cbh4 as usize * 4,
                            "u-pal-pred",
                        );
                        hex_dump::<BitDepth16>(
                            ((*f).cur.data[2] as *mut pixel).offset(uv_dstoff as isize),
                            PXSTRIDE((*f).cur.stride[1]) as usize,
                            cbw4 as usize * 4,
                            cbh4 as usize * 4,
                            "v-pal-pred",
                        );
                    }
                }
                let sm_uv_fl = sm_uv_flag((*t).a, cbx4) | sm_uv_flag(&mut (*t).l, cby4);
                let uv_sb_has_tr = (if init_x + 16 >> ss_hor < cw4 {
                    1 as libc::c_int as libc::c_uint
                } else if init_y != 0 {
                    0 as libc::c_int as libc::c_uint
                } else {
                    intra_edge_flags as libc::c_uint
                        & (EDGE_I420_TOP_HAS_RIGHT as libc::c_int
                            >> ((*f).cur.p.layout as libc::c_uint)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint))
                            as libc::c_uint
                }) as libc::c_int;
                let uv_sb_has_bl = (if init_x != 0 {
                    0 as libc::c_int as libc::c_uint
                } else if init_y + 16 >> ss_ver < ch4 {
                    1 as libc::c_int as libc::c_uint
                } else {
                    intra_edge_flags as libc::c_uint
                        & (EDGE_I420_LEFT_HAS_BOTTOM as libc::c_int
                            >> ((*f).cur.p.layout as libc::c_uint)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint))
                            as libc::c_uint
                }) as libc::c_int;
                let sub_cw4 = imin(cw4, init_x + 16 >> ss_hor);
                let mut pl_0 = 0;
                while pl_0 < 2 {
                    y = init_y >> ss_ver;
                    (*t).by += init_y;
                    while y < sub_ch4 {
                        let mut dst_1: *mut pixel =
                            ((*f).cur.data[(1 + pl_0) as usize] as *mut pixel).offset(
                                (4 * (((*t).by >> ss_ver) as isize * PXSTRIDE(stride)
                                    + ((*t).bx + init_x >> ss_hor) as isize))
                                    as isize,
                            );
                        x = init_x >> ss_hor;
                        (*t).bx += init_x;
                        while x < sub_cw4 {
                            let mut angle_1 = 0;
                            let mut edge_flags_0: EdgeFlags = 0 as EdgeFlags;
                            let mut top_sb_edge_1: *const pixel = 0 as *const pixel;
                            let mut uv_mode: IntraPredMode = DC_PRED;
                            let mut xpos_0 = 0;
                            let mut ypos_0 = 0;
                            let mut xstart_0 = 0;
                            let mut ystart_0 = 0;
                            let mut m_1: IntraPredMode = DC_PRED;
                            if !((*b).c2rust_unnamed.c2rust_unnamed.uv_mode as libc::c_int
                                == CFL_PRED as libc::c_int
                                && (*b).c2rust_unnamed.c2rust_unnamed.cfl_alpha[pl_0 as usize]
                                    as libc::c_int
                                    != 0
                                || (*b).c2rust_unnamed.c2rust_unnamed.pal_sz[1] as libc::c_int != 0)
                            {
                                angle_1 =
                                    (*b).c2rust_unnamed.c2rust_unnamed.uv_angle as libc::c_int;
                                edge_flags_0 = ((if (y > init_y >> ss_ver || uv_sb_has_tr == 0)
                                    && x + (*uv_t_dim).w as libc::c_int >= sub_cw4
                                {
                                    0 as libc::c_int
                                } else {
                                    EDGE_I444_TOP_HAS_RIGHT as libc::c_int
                                }) | (if x > init_x >> ss_hor
                                    || uv_sb_has_bl == 0
                                        && y + (*uv_t_dim).h as libc::c_int >= sub_ch4
                                {
                                    0 as libc::c_int
                                } else {
                                    EDGE_I444_LEFT_HAS_BOTTOM as libc::c_int
                                })) as EdgeFlags;
                                top_sb_edge_1 = 0 as *const pixel;
                                if (*t).by & !ss_ver & (*f).sb_step - 1 == 0 {
                                    top_sb_edge_1 =
                                        (*f).ipred_edge[(1 + pl_0) as usize] as *mut pixel;
                                    let sby_1 = (*t).by >> (*f).sb_shift;
                                    top_sb_edge_1 = top_sb_edge_1
                                        .offset(((*f).sb128w * 128 * (sby_1 - 1)) as isize);
                                }
                                uv_mode = (if (*b).c2rust_unnamed.c2rust_unnamed.uv_mode
                                    as libc::c_int
                                    == CFL_PRED as libc::c_int
                                {
                                    DC_PRED as libc::c_int
                                } else {
                                    (*b).c2rust_unnamed.c2rust_unnamed.uv_mode as libc::c_int
                                }) as IntraPredMode;
                                xpos_0 = (*t).bx >> ss_hor;
                                ypos_0 = (*t).by >> ss_ver;
                                xstart_0 = (*ts).tiling.col_start >> ss_hor;
                                ystart_0 = (*ts).tiling.row_start >> ss_ver;
                                m_1 = dav1d_prepare_intra_edges_16bpc(
                                    xpos_0,
                                    (xpos_0 > xstart_0) as libc::c_int,
                                    ypos_0,
                                    (ypos_0 > ystart_0) as libc::c_int,
                                    (*ts).tiling.col_end >> ss_hor,
                                    (*ts).tiling.row_end >> ss_ver,
                                    edge_flags_0,
                                    dst_1,
                                    stride,
                                    top_sb_edge_1,
                                    uv_mode,
                                    &mut angle_1,
                                    (*uv_t_dim).w as libc::c_int,
                                    (*uv_t_dim).h as libc::c_int,
                                    (*(*f).seq_hdr).intra_edge_filter,
                                    edge,
                                    (*f).bitdepth_max,
                                );
                                angle_1 |= intra_edge_filter_flag;
                                ((*dsp).ipred.intra_pred[m_1 as usize])
                                    .expect("non-null function pointer")(
                                    dst_1,
                                    stride,
                                    edge,
                                    (*uv_t_dim).w as libc::c_int * 4,
                                    (*uv_t_dim).h as libc::c_int * 4,
                                    angle_1 | sm_uv_fl,
                                    4 * (*f).bw + ss_hor - 4 * ((*t).bx & !ss_hor) >> ss_hor,
                                    4 * (*f).bh + ss_ver - 4 * ((*t).by & !ss_ver) >> ss_ver,
                                    (*f).bitdepth_max,
                                );
                                if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                                    hex_dump::<BitDepth16>(
                                        edge.offset(-(((*uv_t_dim).h as libc::c_int * 4) as isize)),
                                        (*uv_t_dim).h as usize * 4,
                                        (*uv_t_dim).h as usize * 4,
                                        2,
                                        "l",
                                    );
                                    hex_dump::<BitDepth16>(edge, 0, 1, 1, "tl");
                                    hex_dump::<BitDepth16>(
                                        edge.offset(1),
                                        (*uv_t_dim).w as usize * 4,
                                        (*uv_t_dim).w as usize * 4,
                                        2,
                                        "t",
                                    );
                                    hex_dump::<BitDepth16>(
                                        dst_1,
                                        stride as usize,
                                        (*uv_t_dim).w as usize * 4,
                                        (*uv_t_dim).h as usize * 4,
                                        if pl_0 != 0 {
                                            "v-intra-pred"
                                        } else {
                                            "u-intra-pred"
                                        },
                                    );
                                }
                            }
                            if (*b).skip == 0 {
                                let mut txtp_0: TxfmType = DCT_DCT;
                                let mut eob_0 = 0;
                                let mut cf_0: *mut coef = 0 as *mut coef;
                                if (*t).frame_thread.pass != 0 {
                                    let p_2 = (*t).frame_thread.pass & 1;
                                    cf_0 = (*ts).frame_thread[p_2 as usize].cf as *mut coef;
                                    (*ts).frame_thread[p_2 as usize].cf =
                                        ((*ts).frame_thread[p_2 as usize].cf as *mut coef).offset(
                                            ((*uv_t_dim).w as libc::c_int
                                                * (*uv_t_dim).h as libc::c_int
                                                * 16)
                                                as isize,
                                        )
                                            as *mut libc::c_void;
                                    let cbi_0: *const CodedBlockInfo = &mut *((*f).frame_thread.cbi)
                                        .offset(
                                            ((*t).by as isize * (*f).b4_stride + (*t).bx as isize)
                                                as isize,
                                        )
                                        as *mut CodedBlockInfo;
                                    eob_0 = (*cbi_0).eob[(pl_0 + 1) as usize] as libc::c_int;
                                    txtp_0 = (*cbi_0).txtp[(pl_0 + 1) as usize] as TxfmType;
                                } else {
                                    let mut cf_ctx_0: uint8_t = 0;
                                    cf_0 = ((*t).c2rust_unnamed.cf_16bpc).as_mut_ptr();
                                    eob_0 = decode_coefs(
                                        t,
                                        &mut (*(*t).a).ccoef.0[pl_0 as usize]
                                            [(cbx4 + x) as usize..],
                                        &mut (*t).l.ccoef.0[pl_0 as usize][(cby4 + y) as usize..],
                                        (*b).uvtx as RectTxfmSize,
                                        bs,
                                        b,
                                        1 as libc::c_int,
                                        1 + pl_0,
                                        cf_0,
                                        &mut txtp_0,
                                        &mut cf_ctx_0,
                                    );
                                    if DEBUG_BLOCK_INFO(&*f, &*t) {
                                        printf(
                                            b"Post-uv-cf-blk[pl=%d,tx=%d,txtp=%d,eob=%d]: r=%d [x=%d,cbx4=%d]\n\0"
                                                as *const u8 as *const libc::c_char,
                                            pl_0,
                                            (*b).uvtx as libc::c_int,
                                            txtp_0 as libc::c_uint,
                                            eob_0,
                                            (*ts).msac.rng,
                                            x,
                                            cbx4,
                                        );
                                    }
                                    CaseSet::<16, true>::many(
                                        [&mut (*t).l, &mut *(*t).a],
                                        [
                                            imin(
                                                (*uv_t_dim).h as i32,
                                                (*f).bh - (*t).by + ss_ver >> ss_ver,
                                            ) as usize,
                                            imin(
                                                (*uv_t_dim).w as i32,
                                                (*f).bw - (*t).bx + ss_hor >> ss_hor,
                                            ) as usize,
                                        ],
                                        [(cby4 + y) as usize, (cbx4 + x) as usize],
                                        |case, dir| {
                                            case.set(&mut dir.ccoef.0[pl_0 as usize], cf_ctx_0);
                                        },
                                    );
                                }
                                if eob_0 >= 0 {
                                    if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                                        coef_dump(
                                            cf_0,
                                            (*uv_t_dim).h as usize * 4,
                                            (*uv_t_dim).w as usize * 4,
                                            3,
                                            "dq",
                                        );
                                    }
                                    ((*dsp).itx.itxfm_add[(*b).uvtx as usize][txtp_0 as usize])
                                        .expect("non-null function pointer")(
                                        dst_1,
                                        stride,
                                        cf_0,
                                        eob_0,
                                        (*f).bitdepth_max,
                                    );
                                    if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                                        hex_dump::<BitDepth16>(
                                            dst_1,
                                            stride as usize,
                                            (*uv_t_dim).w as usize * 4,
                                            (*uv_t_dim).h as usize * 4,
                                            "recon",
                                        );
                                    }
                                }
                            } else if (*t).frame_thread.pass == 0 {
                                CaseSet::<16, false>::many(
                                    [&mut (*t).l, &mut *(*t).a],
                                    [(*uv_t_dim).h as usize, (*uv_t_dim).w as usize],
                                    [(cby4 + y) as usize, (cbx4 + x) as usize],
                                    |case, dir| {
                                        case.set(&mut dir.ccoef.0[pl_0 as usize], 0x40);
                                    },
                                );
                            }
                            dst_1 = dst_1.offset(((*uv_t_dim).w as libc::c_int * 4) as isize);
                            x += (*uv_t_dim).w as libc::c_int;
                            (*t).bx += ((*uv_t_dim).w as libc::c_int) << ss_hor;
                        }
                        (*t).bx -= x << ss_hor;
                        y += (*uv_t_dim).h as libc::c_int;
                        (*t).by += ((*uv_t_dim).h as libc::c_int) << ss_ver;
                    }
                    (*t).by -= y << ss_ver;
                    pl_0 += 1;
                }
            }
            init_x += 16 as libc::c_int;
        }
        init_y += 16 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_recon_b_inter_16bpc(
    t: *mut Dav1dTaskContext,
    bs: BlockSize,
    b: *const Av1Block,
) -> libc::c_int {
    let ts: *mut Dav1dTileState = (*t).ts;
    let f: *const Dav1dFrameContext = (*t).f;
    let dsp: *const Dav1dDSPContext = (*f).dsp;
    let bx4 = (*t).bx & 31;
    let by4 = (*t).by & 31;
    let ss_ver = ((*f).cur.p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_hor = ((*f).cur.p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
    let cbx4 = bx4 >> ss_hor;
    let cby4 = by4 >> ss_ver;
    let b_dim: *const uint8_t = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bw4 = *b_dim.offset(0) as libc::c_int;
    let bh4 = *b_dim.offset(1) as libc::c_int;
    let w4 = imin(bw4, (*f).bw - (*t).bx);
    let h4 = imin(bh4, (*f).bh - (*t).by);
    let has_chroma = ((*f).cur.p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint
        && (bw4 > ss_hor || (*t).bx & 1 != 0)
        && (bh4 > ss_ver || (*t).by & 1 != 0)) as libc::c_int;
    let chr_layout_idx = (if (*f).cur.p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint
    {
        0 as libc::c_int as libc::c_uint
    } else {
        (DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint)
            .wrapping_sub((*f).cur.p.layout as libc::c_uint)
    }) as libc::c_int;
    let mut res = 0;
    let cbh4 = bh4 + ss_ver >> ss_ver;
    let cbw4 = bw4 + ss_hor >> ss_hor;
    let mut dst: *mut pixel = ((*f).cur.data[0] as *mut pixel).offset(
        (4 * ((*t).by as isize * PXSTRIDE((*f).cur.stride[0]) + (*t).bx as isize)) as isize,
    );
    let uvdstoff: ptrdiff_t = 4
        * (((*t).bx >> ss_hor) as isize
            + ((*t).by >> ss_ver) as isize * PXSTRIDE((*f).cur.stride[1]));
    if (*(*f).frame_hdr).frame_type as libc::c_uint & 1 as libc::c_uint == 0 {
        if (*(*f).frame_hdr).super_res.enabled != 0 {
            unreachable!();
        }
        res = mc(
            t,
            dst,
            0 as *mut int16_t,
            (*f).cur.stride[0],
            bw4,
            bh4,
            (*t).bx,
            (*t).by,
            0 as libc::c_int,
            (*b).c2rust_unnamed
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c2rust_unnamed
                .mv[0],
            &(*f).sr_cur,
            0 as libc::c_int,
            FILTER_2D_BILINEAR,
        );
        if res != 0 {
            return res;
        }
        if has_chroma != 0 {
            let mut pl = 1;
            while pl < 3 {
                res = mc(
                    t,
                    ((*f).cur.data[pl as usize] as *mut pixel).offset(uvdstoff as isize),
                    0 as *mut int16_t,
                    (*f).cur.stride[1],
                    bw4 << (bw4 == ss_hor) as libc::c_int,
                    bh4 << (bh4 == ss_ver) as libc::c_int,
                    (*t).bx & !ss_hor,
                    (*t).by & !ss_ver,
                    pl,
                    (*b).c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0],
                    &(*f).sr_cur,
                    0 as libc::c_int,
                    FILTER_2D_BILINEAR,
                );
                if res != 0 {
                    return res;
                }
                pl += 1;
            }
        }
    } else if (*b).c2rust_unnamed.c2rust_unnamed_0.comp_type as libc::c_int
        == COMP_INTER_NONE as libc::c_int
    {
        let mut is_sub8x8 = 0;
        let mut r: *const *mut refmvs_block = 0 as *const *mut refmvs_block;
        let refp: *const Dav1dThreadPicture = &*((*f).refp).as_ptr().offset(
            *((*b).c2rust_unnamed.c2rust_unnamed_0.r#ref)
                .as_ptr()
                .offset(0) as isize,
        ) as *const Dav1dThreadPicture;
        let filter_2d: Filter2d = (*b).c2rust_unnamed.c2rust_unnamed_0.filter2d as Filter2d;
        if imin(bw4, bh4) > 1
            && ((*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_int
                == GLOBALMV as libc::c_int
                && (*f).gmv_warp_allowed[(*b).c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as usize]
                    as libc::c_int
                    != 0
                || (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode as libc::c_int
                    == MM_WARP as libc::c_int
                    && (*t).warpmv.type_0 as libc::c_uint
                        > DAV1D_WM_TYPE_TRANSLATION as libc::c_int as libc::c_uint)
        {
            res = warp_affine(
                t,
                dst,
                0 as *mut int16_t,
                (*f).cur.stride[0],
                b_dim,
                0 as libc::c_int,
                refp,
                if (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode as libc::c_int
                    == MM_WARP as libc::c_int
                {
                    &mut (*t).warpmv
                } else {
                    &mut *((*(*f).frame_hdr).gmv).as_mut_ptr().offset(
                        *((*b).c2rust_unnamed.c2rust_unnamed_0.r#ref)
                            .as_ptr()
                            .offset(0) as isize,
                    )
                },
            );
            if res != 0 {
                return res;
            }
        } else {
            res = mc(
                t,
                dst,
                0 as *mut int16_t,
                (*f).cur.stride[0],
                bw4,
                bh4,
                (*t).bx,
                (*t).by,
                0 as libc::c_int,
                (*b).c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .mv[0],
                refp,
                (*b).c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as libc::c_int,
                filter_2d,
            );
            if res != 0 {
                return res;
            }
            if (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode as libc::c_int
                == MM_OBMC as libc::c_int
            {
                res = obmc(
                    t,
                    dst,
                    (*f).cur.stride[0],
                    b_dim,
                    0 as libc::c_int,
                    bx4,
                    by4,
                    w4,
                    h4,
                );
                if res != 0 {
                    return res;
                }
            }
        }
        if (*b).c2rust_unnamed.c2rust_unnamed_0.interintra_type != 0 {
            let tl_edge: *mut pixel = ((*t)
                .scratch
                .c2rust_unnamed_0
                .c2rust_unnamed_0
                .c2rust_unnamed_0
                .edge_16bpc)
                .as_mut_ptr()
                .offset(32);
            let mut m: IntraPredMode = (if (*b)
                .c2rust_unnamed
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c2rust_unnamed
                .interintra_mode as libc::c_int
                == II_SMOOTH_PRED as libc::c_int
            {
                SMOOTH_PRED as libc::c_int
            } else {
                (*b).c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .interintra_mode as libc::c_int
            }) as IntraPredMode;
            let tmp: *mut pixel = ((*t)
                .scratch
                .c2rust_unnamed_0
                .c2rust_unnamed_0
                .c2rust_unnamed_0
                .interintra_16bpc)
                .as_mut_ptr();
            let mut angle = 0;
            let mut top_sb_edge: *const pixel = 0 as *const pixel;
            if (*t).by & (*f).sb_step - 1 == 0 {
                top_sb_edge = (*f).ipred_edge[0] as *mut pixel;
                let sby = (*t).by >> (*f).sb_shift;
                top_sb_edge = top_sb_edge.offset(((*f).sb128w * 128 * (sby - 1)) as isize);
            }
            m = dav1d_prepare_intra_edges_16bpc(
                (*t).bx,
                ((*t).bx > (*ts).tiling.col_start) as libc::c_int,
                (*t).by,
                ((*t).by > (*ts).tiling.row_start) as libc::c_int,
                (*ts).tiling.col_end,
                (*ts).tiling.row_end,
                0 as EdgeFlags,
                dst,
                (*f).cur.stride[0],
                top_sb_edge,
                m,
                &mut angle,
                bw4,
                bh4,
                0 as libc::c_int,
                tl_edge,
                (*f).bitdepth_max,
            );
            ((*dsp).ipred.intra_pred[m as usize]).expect("non-null function pointer")(
                tmp,
                ((4 * bw4) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong)
                    as ptrdiff_t,
                tl_edge,
                bw4 * 4,
                bh4 * 4,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                (*f).bitdepth_max,
            );
            let ii_mask: *const uint8_t = if (*b).c2rust_unnamed.c2rust_unnamed_0.interintra_type
                as libc::c_int
                == INTER_INTRA_BLEND as libc::c_int
            {
                dav1d_ii_masks[bs as usize][0][(*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .interintra_mode as usize]
            } else {
                dav1d_wedge_masks[bs as usize][0][0][(*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .wedge_idx as usize]
            };
            ((*dsp).mc.blend).expect("non-null function pointer")(
                dst,
                (*f).cur.stride[0],
                tmp,
                bw4 * 4,
                bh4 * 4,
                ii_mask,
            );
        }
        if !(has_chroma == 0) {
            is_sub8x8 = (bw4 == ss_hor || bh4 == ss_ver) as libc::c_int;
            r = 0 as *const *mut refmvs_block;
            if is_sub8x8 != 0 {
                if !(ss_hor == 1) {
                    unreachable!();
                }
                r = &mut *((*t).rt.r)
                    .as_mut_ptr()
                    .offset((((*t).by & 31) + 5) as isize)
                    as *mut *mut refmvs_block;
                if bw4 == 1 {
                    is_sub8x8 &= ((*(*r.offset(0)).offset(((*t).bx - 1) as isize))
                        .0
                        .r#ref
                        .r#ref[0] as libc::c_int
                        > 0) as libc::c_int;
                }
                if bh4 == ss_ver {
                    is_sub8x8 &= ((*(*r.offset(-(1 as libc::c_int) as isize))
                        .offset((*t).bx as isize))
                    .0
                    .r#ref
                    .r#ref[0] as libc::c_int
                        > 0) as libc::c_int;
                }
                if bw4 == 1 && bh4 == ss_ver {
                    is_sub8x8 &= ((*(*r.offset(-(1 as libc::c_int) as isize))
                        .offset(((*t).bx - 1) as isize))
                    .0
                    .r#ref
                    .r#ref[0] as libc::c_int
                        > 0) as libc::c_int;
                }
            }
            if is_sub8x8 != 0 {
                if !(ss_hor == 1) {
                    unreachable!();
                }
                let mut h_off: ptrdiff_t = 0 as libc::c_int as ptrdiff_t;
                let mut v_off: ptrdiff_t = 0 as libc::c_int as ptrdiff_t;
                if bw4 == 1 && bh4 == ss_ver {
                    let mut pl_0 = 0;
                    while pl_0 < 2 {
                        res = mc(
                            t,
                            ((*f).cur.data[(1 + pl_0) as usize] as *mut pixel)
                                .offset(uvdstoff as isize),
                            0 as *mut int16_t,
                            (*f).cur.stride[1],
                            bw4,
                            bh4,
                            (*t).bx - 1,
                            (*t).by - 1,
                            1 + pl_0,
                            (*(*r.offset(-(1 as libc::c_int) as isize))
                                .offset(((*t).bx - 1) as isize))
                            .0
                            .mv
                            .mv[0],
                            &*((*f).refp).as_ptr().offset(
                                (*((*(*r.offset(-(1 as libc::c_int) as isize))
                                    .offset(((*t).bx - 1) as isize))
                                .0
                                .r#ref
                                .r#ref)
                                    .as_mut_ptr()
                                    .offset(0) as libc::c_int
                                    - 1) as isize,
                            ),
                            (*(*r.offset(-(1 as libc::c_int) as isize))
                                .offset(((*t).bx - 1) as isize))
                            .0
                            .r#ref
                            .r#ref[0] as libc::c_int
                                - 1,
                            (if (*t).frame_thread.pass != 2 as libc::c_int {
                                (*t).tl_4x4_filter as libc::c_uint
                            } else {
                                (*((*f).frame_thread.b).offset(
                                    (((*t).by - 1) as isize * (*f).b4_stride + (*t).bx as isize - 1)
                                        as isize,
                                ))
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .filter2d as libc::c_uint
                            }) as Filter2d,
                        );
                        if res != 0 {
                            return res;
                        }
                        pl_0 += 1;
                    }
                    v_off = 2 * PXSTRIDE((*f).cur.stride[1]);
                    h_off = 2 as libc::c_int as ptrdiff_t;
                }
                if bw4 == 1 {
                    let left_filter_2d: Filter2d = dav1d_filter_2d
                        [(*t).l.filter[1][by4 as usize] as usize]
                        [(*t).l.filter[0][by4 as usize] as usize]
                        as Filter2d;
                    let mut pl_1 = 0;
                    while pl_1 < 2 {
                        res = mc(
                            t,
                            ((*f).cur.data[(1 + pl_1) as usize] as *mut pixel)
                                .offset(uvdstoff as isize)
                                .offset(v_off as isize),
                            0 as *mut int16_t,
                            (*f).cur.stride[1],
                            bw4,
                            bh4,
                            (*t).bx - 1,
                            (*t).by,
                            1 + pl_1,
                            (*(*r.offset(0)).offset(((*t).bx - 1) as isize)).0.mv.mv[0],
                            &*((*f).refp).as_ptr().offset(
                                (*((*(*r.offset(0)).offset(((*t).bx - 1) as isize))
                                    .0
                                    .r#ref
                                    .r#ref)
                                    .as_mut_ptr()
                                    .offset(0) as libc::c_int
                                    - 1) as isize,
                            ),
                            (*(*r.offset(0)).offset(((*t).bx - 1) as isize))
                                .0
                                .r#ref
                                .r#ref[0] as libc::c_int
                                - 1,
                            (if (*t).frame_thread.pass != 2 as libc::c_int {
                                left_filter_2d as libc::c_uint
                            } else {
                                (*((*f).frame_thread.b).offset(
                                    ((*t).by as isize * (*f).b4_stride + (*t).bx as isize - 1)
                                        as isize,
                                ))
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .filter2d as libc::c_uint
                            }) as Filter2d,
                        );
                        if res != 0 {
                            return res;
                        }
                        pl_1 += 1;
                    }
                    h_off = 2 as libc::c_int as ptrdiff_t;
                }
                if bh4 == ss_ver {
                    let top_filter_2d: Filter2d = dav1d_filter_2d
                        [(*(*t).a).filter[1][bx4 as usize] as usize]
                        [(*(*t).a).filter[0][bx4 as usize] as usize]
                        as Filter2d;
                    let mut pl_2 = 0;
                    while pl_2 < 2 {
                        res = mc(
                            t,
                            ((*f).cur.data[(1 + pl_2) as usize] as *mut pixel)
                                .offset(uvdstoff as isize)
                                .offset(h_off as isize),
                            0 as *mut int16_t,
                            (*f).cur.stride[1],
                            bw4,
                            bh4,
                            (*t).bx,
                            (*t).by - 1,
                            1 + pl_2,
                            (*(*r.offset(-(1 as libc::c_int) as isize)).offset((*t).bx as isize))
                                .0
                                .mv
                                .mv[0],
                            &*((*f).refp).as_ptr().offset(
                                (*((*(*r.offset(-(1 as libc::c_int) as isize))
                                    .offset((*t).bx as isize))
                                .0
                                .r#ref
                                .r#ref)
                                    .as_mut_ptr()
                                    .offset(0) as libc::c_int
                                    - 1) as isize,
                            ),
                            (*(*r.offset(-(1 as libc::c_int) as isize)).offset((*t).bx as isize))
                                .0
                                .r#ref
                                .r#ref[0] as libc::c_int
                                - 1,
                            (if (*t).frame_thread.pass != 2 as libc::c_int {
                                top_filter_2d as libc::c_uint
                            } else {
                                (*((*f).frame_thread.b).offset(
                                    (((*t).by - 1) as isize * (*f).b4_stride + (*t).bx as isize)
                                        as isize,
                                ))
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .filter2d as libc::c_uint
                            }) as Filter2d,
                        );
                        if res != 0 {
                            return res;
                        }
                        pl_2 += 1;
                    }
                    v_off = 2 * PXSTRIDE((*f).cur.stride[1]);
                }
                let mut pl_3 = 0;
                while pl_3 < 2 {
                    res = mc(
                        t,
                        ((*f).cur.data[(1 + pl_3) as usize] as *mut pixel)
                            .offset(uvdstoff as isize)
                            .offset(h_off as isize)
                            .offset(v_off as isize),
                        0 as *mut int16_t,
                        (*f).cur.stride[1],
                        bw4,
                        bh4,
                        (*t).bx,
                        (*t).by,
                        1 + pl_3,
                        (*b).c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mv[0],
                        refp,
                        (*b).c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as libc::c_int,
                        filter_2d,
                    );
                    if res != 0 {
                        return res;
                    }
                    pl_3 += 1;
                }
            } else {
                if imin(cbw4, cbh4) > 1
                    && ((*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_int
                        == GLOBALMV as libc::c_int
                        && (*f).gmv_warp_allowed
                            [(*b).c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as usize]
                            as libc::c_int
                            != 0
                        || (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode as libc::c_int
                            == MM_WARP as libc::c_int
                            && (*t).warpmv.type_0 as libc::c_uint
                                > DAV1D_WM_TYPE_TRANSLATION as libc::c_int as libc::c_uint)
                {
                    let mut pl_4 = 0;
                    while pl_4 < 2 {
                        res = warp_affine(
                            t,
                            ((*f).cur.data[(1 + pl_4) as usize] as *mut pixel)
                                .offset(uvdstoff as isize),
                            0 as *mut int16_t,
                            (*f).cur.stride[1],
                            b_dim,
                            1 + pl_4,
                            refp,
                            if (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode as libc::c_int
                                == MM_WARP as libc::c_int
                            {
                                &mut (*t).warpmv
                            } else {
                                &mut *((*(*f).frame_hdr).gmv).as_mut_ptr().offset(
                                    *((*b).c2rust_unnamed.c2rust_unnamed_0.r#ref)
                                        .as_ptr()
                                        .offset(0) as isize,
                                )
                            },
                        );
                        if res != 0 {
                            return res;
                        }
                        pl_4 += 1;
                    }
                } else {
                    let mut pl_5 = 0;
                    while pl_5 < 2 {
                        res = mc(
                            t,
                            ((*f).cur.data[(1 + pl_5) as usize] as *mut pixel)
                                .offset(uvdstoff as isize),
                            0 as *mut int16_t,
                            (*f).cur.stride[1],
                            bw4 << (bw4 == ss_hor) as libc::c_int,
                            bh4 << (bh4 == ss_ver) as libc::c_int,
                            (*t).bx & !ss_hor,
                            (*t).by & !ss_ver,
                            1 + pl_5,
                            (*b).c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .mv[0],
                            refp,
                            (*b).c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as libc::c_int,
                            filter_2d,
                        );
                        if res != 0 {
                            return res;
                        }
                        if (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode as libc::c_int
                            == MM_OBMC as libc::c_int
                        {
                            res = obmc(
                                t,
                                ((*f).cur.data[(1 + pl_5) as usize] as *mut pixel)
                                    .offset(uvdstoff as isize),
                                (*f).cur.stride[1],
                                b_dim,
                                1 + pl_5,
                                bx4,
                                by4,
                                w4,
                                h4,
                            );
                            if res != 0 {
                                return res;
                            }
                        }
                        pl_5 += 1;
                    }
                }
                if (*b).c2rust_unnamed.c2rust_unnamed_0.interintra_type != 0 {
                    let ii_mask_0: *const uint8_t =
                        if (*b).c2rust_unnamed.c2rust_unnamed_0.interintra_type as libc::c_int
                            == INTER_INTRA_BLEND as libc::c_int
                        {
                            dav1d_ii_masks[bs as usize][chr_layout_idx as usize][(*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .interintra_mode
                                as usize]
                        } else {
                            dav1d_wedge_masks[bs as usize][chr_layout_idx as usize][0][(*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .wedge_idx
                                as usize]
                        };
                    let mut pl_6 = 0;
                    while pl_6 < 2 {
                        let tmp_0: *mut pixel = ((*t)
                            .scratch
                            .c2rust_unnamed_0
                            .c2rust_unnamed_0
                            .c2rust_unnamed_0
                            .interintra_16bpc)
                            .as_mut_ptr();
                        let tl_edge_0: *mut pixel = ((*t)
                            .scratch
                            .c2rust_unnamed_0
                            .c2rust_unnamed_0
                            .c2rust_unnamed_0
                            .edge_16bpc)
                            .as_mut_ptr()
                            .offset(32);
                        let mut m_0: IntraPredMode = (if (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .interintra_mode
                            as libc::c_int
                            == II_SMOOTH_PRED as libc::c_int
                        {
                            SMOOTH_PRED as libc::c_int
                        } else {
                            (*b).c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .interintra_mode as libc::c_int
                        }) as IntraPredMode;
                        let mut angle_0 = 0;
                        let uvdst: *mut pixel = ((*f).cur.data[(1 + pl_6) as usize] as *mut pixel)
                            .offset(uvdstoff as isize);
                        let mut top_sb_edge_0: *const pixel = 0 as *const pixel;
                        if (*t).by & (*f).sb_step - 1 == 0 {
                            top_sb_edge_0 = (*f).ipred_edge[(pl_6 + 1) as usize] as *mut pixel;
                            let sby_0 = (*t).by >> (*f).sb_shift;
                            top_sb_edge_0 =
                                top_sb_edge_0.offset(((*f).sb128w * 128 * (sby_0 - 1)) as isize);
                        }
                        m_0 = dav1d_prepare_intra_edges_16bpc(
                            (*t).bx >> ss_hor,
                            ((*t).bx >> ss_hor > (*ts).tiling.col_start >> ss_hor) as libc::c_int,
                            (*t).by >> ss_ver,
                            ((*t).by >> ss_ver > (*ts).tiling.row_start >> ss_ver) as libc::c_int,
                            (*ts).tiling.col_end >> ss_hor,
                            (*ts).tiling.row_end >> ss_ver,
                            0 as EdgeFlags,
                            uvdst,
                            (*f).cur.stride[1],
                            top_sb_edge_0,
                            m_0,
                            &mut angle_0,
                            cbw4,
                            cbh4,
                            0 as libc::c_int,
                            tl_edge_0,
                            (*f).bitdepth_max,
                        );
                        ((*dsp).ipred.intra_pred[m_0 as usize]).expect("non-null function pointer")(
                            tmp_0,
                            ((cbw4 * 4) as libc::c_ulong)
                                .wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong)
                                as ptrdiff_t,
                            tl_edge_0,
                            cbw4 * 4,
                            cbh4 * 4,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            (*f).bitdepth_max,
                        );
                        ((*dsp).mc.blend).expect("non-null function pointer")(
                            uvdst,
                            (*f).cur.stride[1],
                            tmp_0,
                            cbw4 * 4,
                            cbh4 * 4,
                            ii_mask_0,
                        );
                        pl_6 += 1;
                    }
                }
            }
        }
        (*t).tl_4x4_filter = filter_2d;
    } else {
        let filter_2d_0: Filter2d = (*b).c2rust_unnamed.c2rust_unnamed_0.filter2d as Filter2d;
        let mut tmp_1: *mut [int16_t; 16384] = ((*t)
            .scratch
            .c2rust_unnamed
            .c2rust_unnamed
            .c2rust_unnamed
            .compinter)
            .as_mut_ptr();
        let mut jnt_weight = 0;
        let seg_mask: *mut uint8_t = ((*t)
            .scratch
            .c2rust_unnamed
            .c2rust_unnamed
            .c2rust_unnamed
            .seg_mask)
            .as_mut_ptr();
        let mut mask: *const uint8_t = 0 as *const uint8_t;
        let mut i = 0;
        while i < 2 {
            let refp_0: *const Dav1dThreadPicture = &*((*f).refp).as_ptr().offset(
                *((*b).c2rust_unnamed.c2rust_unnamed_0.r#ref)
                    .as_ptr()
                    .offset(i as isize) as isize,
            ) as *const Dav1dThreadPicture;
            if (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_int
                == GLOBALMV_GLOBALMV as libc::c_int
                && (*f).gmv_warp_allowed
                    [(*b).c2rust_unnamed.c2rust_unnamed_0.r#ref[i as usize] as usize]
                    as libc::c_int
                    != 0
            {
                res = warp_affine(
                    t,
                    0 as *mut pixel,
                    (*tmp_1.offset(i as isize)).as_mut_ptr(),
                    (bw4 * 4) as ptrdiff_t,
                    b_dim,
                    0 as libc::c_int,
                    refp_0,
                    &mut *((*(*f).frame_hdr).gmv).as_mut_ptr().offset(
                        *((*b).c2rust_unnamed.c2rust_unnamed_0.r#ref)
                            .as_ptr()
                            .offset(i as isize) as isize,
                    ),
                );
                if res != 0 {
                    return res;
                }
            } else {
                res = mc(
                    t,
                    0 as *mut pixel,
                    (*tmp_1.offset(i as isize)).as_mut_ptr(),
                    0 as libc::c_int as ptrdiff_t,
                    bw4,
                    bh4,
                    (*t).bx,
                    (*t).by,
                    0 as libc::c_int,
                    (*b).c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[i as usize],
                    refp_0,
                    (*b).c2rust_unnamed.c2rust_unnamed_0.r#ref[i as usize] as libc::c_int,
                    filter_2d_0,
                );
                if res != 0 {
                    return res;
                }
            }
            i += 1;
        }
        match (*b).c2rust_unnamed.c2rust_unnamed_0.comp_type as libc::c_int {
            2 => {
                ((*dsp).mc.avg).expect("non-null function pointer")(
                    dst,
                    (*f).cur.stride[0],
                    (*tmp_1.offset(0)).as_mut_ptr(),
                    (*tmp_1.offset(1)).as_mut_ptr(),
                    bw4 * 4,
                    bh4 * 4,
                    (*f).bitdepth_max,
                );
            }
            1 => {
                jnt_weight = (*f).jnt_weights
                    [(*b).c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as usize]
                    [(*b).c2rust_unnamed.c2rust_unnamed_0.r#ref[1] as usize]
                    as libc::c_int;
                ((*dsp).mc.w_avg).expect("non-null function pointer")(
                    dst,
                    (*f).cur.stride[0],
                    (*tmp_1.offset(0)).as_mut_ptr(),
                    (*tmp_1.offset(1)).as_mut_ptr(),
                    bw4 * 4,
                    bh4 * 4,
                    jnt_weight,
                    (*f).bitdepth_max,
                );
            }
            3 => {
                ((*dsp).mc.w_mask[chr_layout_idx as usize]).expect("non-null function pointer")(
                    dst,
                    (*f).cur.stride[0],
                    (*tmp_1.offset(
                        (*b).c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mask_sign as isize,
                    ))
                    .as_mut_ptr(),
                    (*tmp_1.offset(
                        ((*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mask_sign
                            == 0) as libc::c_int as isize,
                    ))
                    .as_mut_ptr(),
                    bw4 * 4,
                    bh4 * 4,
                    seg_mask,
                    (*b).c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mask_sign as libc::c_int,
                    (*f).bitdepth_max,
                );
                mask = seg_mask;
            }
            4 => {
                mask = dav1d_wedge_masks[bs as usize][0][0][(*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .wedge_idx
                    as usize];
                ((*dsp).mc.mask).expect("non-null function pointer")(
                    dst,
                    (*f).cur.stride[0],
                    (*tmp_1.offset(
                        (*b).c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mask_sign as isize,
                    ))
                    .as_mut_ptr(),
                    (*tmp_1.offset(
                        ((*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mask_sign
                            == 0) as libc::c_int as isize,
                    ))
                    .as_mut_ptr(),
                    bw4 * 4,
                    bh4 * 4,
                    mask,
                    (*f).bitdepth_max,
                );
                if has_chroma != 0 {
                    mask = dav1d_wedge_masks[bs as usize][chr_layout_idx as usize][(*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mask_sign
                        as usize][(*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .wedge_idx as usize];
                }
            }
            _ => {}
        }
        if has_chroma != 0 {
            let mut pl_7 = 0;
            while pl_7 < 2 {
                let mut i_0 = 0;
                while i_0 < 2 {
                    let refp_1: *const Dav1dThreadPicture = &*((*f).refp).as_ptr().offset(
                        *((*b).c2rust_unnamed.c2rust_unnamed_0.r#ref)
                            .as_ptr()
                            .offset(i_0 as isize) as isize,
                    )
                        as *const Dav1dThreadPicture;
                    if (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_int
                        == GLOBALMV_GLOBALMV as libc::c_int
                        && imin(cbw4, cbh4) > 1
                        && (*f).gmv_warp_allowed
                            [(*b).c2rust_unnamed.c2rust_unnamed_0.r#ref[i_0 as usize] as usize]
                            as libc::c_int
                            != 0
                    {
                        res = warp_affine(
                            t,
                            0 as *mut pixel,
                            (*tmp_1.offset(i_0 as isize)).as_mut_ptr(),
                            (bw4 * 4 >> ss_hor) as ptrdiff_t,
                            b_dim,
                            1 + pl_7,
                            refp_1,
                            &mut *((*(*f).frame_hdr).gmv).as_mut_ptr().offset(
                                *((*b).c2rust_unnamed.c2rust_unnamed_0.r#ref)
                                    .as_ptr()
                                    .offset(i_0 as isize) as isize,
                            ),
                        );
                        if res != 0 {
                            return res;
                        }
                    } else {
                        res = mc(
                            t,
                            0 as *mut pixel,
                            (*tmp_1.offset(i_0 as isize)).as_mut_ptr(),
                            0 as libc::c_int as ptrdiff_t,
                            bw4,
                            bh4,
                            (*t).bx,
                            (*t).by,
                            1 + pl_7,
                            (*b).c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .mv[i_0 as usize],
                            refp_1,
                            (*b).c2rust_unnamed.c2rust_unnamed_0.r#ref[i_0 as usize] as libc::c_int,
                            filter_2d_0,
                        );
                        if res != 0 {
                            return res;
                        }
                    }
                    i_0 += 1;
                }
                let uvdst_0: *mut pixel =
                    ((*f).cur.data[(1 + pl_7) as usize] as *mut pixel).offset(uvdstoff as isize);
                match (*b).c2rust_unnamed.c2rust_unnamed_0.comp_type as libc::c_int {
                    2 => {
                        ((*dsp).mc.avg).expect("non-null function pointer")(
                            uvdst_0,
                            (*f).cur.stride[1],
                            (*tmp_1.offset(0)).as_mut_ptr(),
                            (*tmp_1.offset(1)).as_mut_ptr(),
                            bw4 * 4 >> ss_hor,
                            bh4 * 4 >> ss_ver,
                            (*f).bitdepth_max,
                        );
                    }
                    1 => {
                        ((*dsp).mc.w_avg).expect("non-null function pointer")(
                            uvdst_0,
                            (*f).cur.stride[1],
                            (*tmp_1.offset(0)).as_mut_ptr(),
                            (*tmp_1.offset(1)).as_mut_ptr(),
                            bw4 * 4 >> ss_hor,
                            bh4 * 4 >> ss_ver,
                            jnt_weight,
                            (*f).bitdepth_max,
                        );
                    }
                    4 | 3 => {
                        ((*dsp).mc.mask).expect("non-null function pointer")(
                            uvdst_0,
                            (*f).cur.stride[1],
                            (*tmp_1.offset(
                                (*b).c2rust_unnamed
                                    .c2rust_unnamed_0
                                    .c2rust_unnamed
                                    .c2rust_unnamed
                                    .mask_sign as isize,
                            ))
                            .as_mut_ptr(),
                            (*tmp_1.offset(
                                ((*b)
                                    .c2rust_unnamed
                                    .c2rust_unnamed_0
                                    .c2rust_unnamed
                                    .c2rust_unnamed
                                    .mask_sign
                                    == 0) as libc::c_int as isize,
                            ))
                            .as_mut_ptr(),
                            bw4 * 4 >> ss_hor,
                            bh4 * 4 >> ss_ver,
                            mask,
                            (*f).bitdepth_max,
                        );
                    }
                    _ => {}
                }
                pl_7 += 1;
            }
        }
    }
    if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
        hex_dump::<BitDepth16>(
            dst,
            (*f).cur.stride[0] as usize,
            *b_dim.offset(0) as usize * 4,
            *b_dim.offset(1) as usize * 4,
            "y-pred",
        );
        if has_chroma != 0 {
            hex_dump::<BitDepth16>(
                &mut *(*((*f).cur.data).as_ptr().offset(1) as *mut pixel).offset(uvdstoff as isize),
                (*f).cur.stride[1] as usize,
                cbw4 as usize * 4,
                cbh4 as usize * 4,
                "u-pred",
            );
            hex_dump::<BitDepth16>(
                &mut *(*((*f).cur.data).as_ptr().offset(2) as *mut pixel).offset(uvdstoff as isize),
                (*f).cur.stride[1] as usize,
                cbw4 as usize * 4,
                cbh4 as usize * 4,
                "v-pred",
            );
        }
    }
    let cw4 = w4 + ss_hor >> ss_hor;
    let ch4 = h4 + ss_ver >> ss_ver;
    if (*b).skip != 0 {
        CaseSet::<32, false>::many(
            [&mut (*t).l, &mut *(*t).a],
            [bh4 as usize, bw4 as usize],
            [by4 as usize, bx4 as usize],
            |case, dir| {
                case.set(&mut dir.lcoef.0, 0x40);
            },
        );
        if has_chroma != 0 {
            CaseSet::<32, false>::many(
                [&mut (*t).l, &mut *(*t).a],
                [cbh4 as usize, cbw4 as usize],
                [cby4 as usize, cbx4 as usize],
                |case, dir| {
                    case.set(&mut dir.ccoef.0[0], 0x40);
                    case.set(&mut dir.ccoef.0[1], 0x40);
                },
            );
        }
        return 0 as libc::c_int;
    }
    let uvtx: *const TxfmInfo =
        &*dav1d_txfm_dimensions.as_ptr().offset((*b).uvtx as isize) as *const TxfmInfo;
    let ytx: *const TxfmInfo = &*dav1d_txfm_dimensions
        .as_ptr()
        .offset((*b).c2rust_unnamed.c2rust_unnamed_0.max_ytx as isize)
        as *const TxfmInfo;
    let tx_split: [uint16_t; 2] = [
        (*b).c2rust_unnamed.c2rust_unnamed_0.tx_split0 as uint16_t,
        (*b).c2rust_unnamed.c2rust_unnamed_0.tx_split1,
    ];
    let mut init_y = 0;
    while init_y < bh4 {
        let mut init_x = 0;
        while init_x < bw4 {
            let mut y_off = (init_y != 0) as libc::c_int;
            let mut y = 0;
            dst = dst.offset((PXSTRIDE((*f).cur.stride[0]) * 4 * init_y as isize) as isize);
            y = init_y;
            (*t).by += init_y;
            while y < imin(h4, init_y + 16) {
                let mut x = 0;
                let mut x_off = (init_x != 0) as libc::c_int;
                x = init_x;
                (*t).bx += init_x;
                while x < imin(w4, init_x + 16) {
                    read_coef_tree(
                        t,
                        bs,
                        b,
                        (*b).c2rust_unnamed.c2rust_unnamed_0.max_ytx as RectTxfmSize,
                        0 as libc::c_int,
                        tx_split.as_ptr(),
                        x_off,
                        y_off,
                        &mut *dst.offset((x * 4) as isize),
                    );
                    (*t).bx += (*ytx).w as libc::c_int;
                    x += (*ytx).w as libc::c_int;
                    x_off += 1;
                }
                dst = dst.offset((PXSTRIDE((*f).cur.stride[0]) * 4 * (*ytx).h as isize) as isize);
                (*t).bx -= x;
                (*t).by += (*ytx).h as libc::c_int;
                y += (*ytx).h as libc::c_int;
                y_off += 1;
            }
            dst = dst.offset(-((PXSTRIDE((*f).cur.stride[0]) * 4 * y as isize) as isize));
            (*t).by -= y;
            if has_chroma != 0 {
                let mut pl_8 = 0;
                while pl_8 < 2 {
                    let mut uvdst_1: *mut pixel = ((*f).cur.data[(1 + pl_8) as usize]
                        as *mut pixel)
                        .offset(uvdstoff as isize)
                        .offset(
                            (PXSTRIDE((*f).cur.stride[1]) * init_y as isize * 4 >> ss_ver) as isize,
                        );
                    y = init_y >> ss_ver;
                    (*t).by += init_y;
                    while y < imin(ch4, init_y + 16 >> ss_ver) {
                        let mut x_0 = 0;
                        x_0 = init_x >> ss_hor;
                        (*t).bx += init_x;
                        while x_0 < imin(cw4, init_x + 16 >> ss_hor) {
                            let mut cf: *mut coef = 0 as *mut coef;
                            let mut eob = 0;
                            let mut txtp: TxfmType = DCT_DCT;
                            if (*t).frame_thread.pass != 0 {
                                let p = (*t).frame_thread.pass & 1;
                                cf = (*ts).frame_thread[p as usize].cf as *mut coef;
                                (*ts).frame_thread[p as usize].cf =
                                    ((*ts).frame_thread[p as usize].cf as *mut coef).offset(
                                        ((*uvtx).w as libc::c_int * (*uvtx).h as libc::c_int * 16)
                                            as isize,
                                    ) as *mut libc::c_void;
                                let cbi: *const CodedBlockInfo =
                                    &mut *((*f).frame_thread.cbi).offset(
                                        ((*t).by as isize * (*f).b4_stride + (*t).bx as isize)
                                            as isize,
                                    ) as *mut CodedBlockInfo;
                                eob = (*cbi).eob[(1 + pl_8) as usize] as libc::c_int;
                                txtp = (*cbi).txtp[(1 + pl_8) as usize] as TxfmType;
                            } else {
                                let mut cf_ctx: uint8_t = 0;
                                cf = ((*t).c2rust_unnamed.cf_16bpc).as_mut_ptr();
                                txtp = (*t).txtp_map
                                    [((by4 + (y << ss_ver)) * 32 + bx4 + (x_0 << ss_hor)) as usize]
                                    as TxfmType;
                                eob = decode_coefs(
                                    t,
                                    &mut (*(*t).a).ccoef.0[pl_8 as usize][(cbx4 + x_0) as usize..],
                                    &mut (*t).l.ccoef.0[pl_8 as usize][(cby4 + y) as usize..],
                                    (*b).uvtx as RectTxfmSize,
                                    bs,
                                    b,
                                    0 as libc::c_int,
                                    1 + pl_8,
                                    cf,
                                    &mut txtp,
                                    &mut cf_ctx,
                                );
                                if DEBUG_BLOCK_INFO(&*f, &*t) {
                                    printf(
                                        b"Post-uv-cf-blk[pl=%d,tx=%d,txtp=%d,eob=%d]: r=%d\n\0"
                                            as *const u8
                                            as *const libc::c_char,
                                        pl_8,
                                        (*b).uvtx as libc::c_int,
                                        txtp as libc::c_uint,
                                        eob,
                                        (*ts).msac.rng,
                                    );
                                }
                                CaseSet::<16, true>::many(
                                    [&mut (*t).l, &mut *(*t).a],
                                    [
                                        imin((*uvtx).h as i32, (*f).bh - (*t).by + ss_ver >> ss_ver)
                                            as usize,
                                        imin((*uvtx).w as i32, (*f).bw - (*t).bx + ss_hor >> ss_hor)
                                            as usize,
                                    ],
                                    [(cby4 + y) as usize, (cbx4 + x_0) as usize],
                                    |case, dir| {
                                        case.set(&mut dir.ccoef.0[pl_8 as usize], cf_ctx);
                                    },
                                );
                            }
                            if eob >= 0 {
                                if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                                    coef_dump(
                                        cf,
                                        (*uvtx).h as usize * 4,
                                        (*uvtx).w as usize * 4,
                                        3,
                                        "dq",
                                    );
                                }
                                ((*dsp).itx.itxfm_add[(*b).uvtx as usize][txtp as usize])
                                    .expect("non-null function pointer")(
                                    &mut *uvdst_1.offset((4 * x_0) as isize),
                                    (*f).cur.stride[1],
                                    cf,
                                    eob,
                                    (*f).bitdepth_max,
                                );
                                if DEBUG_BLOCK_INFO(&*f, &*t) && 0 != 0 {
                                    hex_dump::<BitDepth16>(
                                        &mut *uvdst_1.offset((4 * x_0) as isize),
                                        (*f).cur.stride[1] as usize,
                                        (*uvtx).w as usize * 4,
                                        (*uvtx).h as usize * 4,
                                        "recon",
                                    );
                                }
                            }
                            (*t).bx += ((*uvtx).w as libc::c_int) << ss_hor;
                            x_0 += (*uvtx).w as libc::c_int;
                        }
                        uvdst_1 = uvdst_1.offset(
                            (PXSTRIDE((*f).cur.stride[1]) * 4 * (*uvtx).h as isize) as isize,
                        );
                        (*t).bx -= x_0 << ss_hor;
                        (*t).by += ((*uvtx).h as libc::c_int) << ss_ver;
                        y += (*uvtx).h as libc::c_int;
                    }
                    (*t).by -= y << ss_ver;
                    pl_8 += 1;
                }
            }
            init_x += 16 as libc::c_int;
        }
        init_y += 16 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_filter_sbrow_deblock_cols_16bpc(
    f: *mut Dav1dFrameContext,
    sby: libc::c_int,
) {
    if (*(*f).c).inloop_filters as libc::c_uint
        & DAV1D_INLOOPFILTER_DEBLOCK as libc::c_int as libc::c_uint
        == 0
        || (*(*f).frame_hdr).loopfilter.level_y[0] == 0
            && (*(*f).frame_hdr).loopfilter.level_y[1] == 0
    {
        return;
    }
    let y = sby * (*f).sb_step * 4;
    let ss_ver = ((*f).cur.p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let p: [*mut pixel; 3] = [
        ((*f).lf.p[0] as *mut pixel).offset((y as isize * PXSTRIDE((*f).cur.stride[0])) as isize),
        ((*f).lf.p[1] as *mut pixel)
            .offset((y as isize * PXSTRIDE((*f).cur.stride[1]) >> ss_ver) as isize),
        ((*f).lf.p[2] as *mut pixel)
            .offset((y as isize * PXSTRIDE((*f).cur.stride[1]) >> ss_ver) as isize),
    ];
    let mut mask: *mut Av1Filter = ((*f).lf.mask)
        .offset(((sby >> ((*(*f).seq_hdr).sb128 == 0) as libc::c_int) * (*f).sb128w) as isize);
    dav1d_loopfilter_sbrow_cols_16bpc(
        f,
        p.as_ptr(),
        mask,
        sby,
        *((*f).lf.start_of_tile_row).offset(sby as isize) as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_filter_sbrow_deblock_rows_16bpc(
    f: *mut Dav1dFrameContext,
    sby: libc::c_int,
) {
    let y = sby * (*f).sb_step * 4;
    let ss_ver = ((*f).cur.p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let p: [*mut pixel; 3] = [
        ((*f).lf.p[0] as *mut pixel).offset((y as isize * PXSTRIDE((*f).cur.stride[0])) as isize),
        ((*f).lf.p[1] as *mut pixel)
            .offset((y as isize * PXSTRIDE((*f).cur.stride[1]) >> ss_ver) as isize),
        ((*f).lf.p[2] as *mut pixel)
            .offset((y as isize * PXSTRIDE((*f).cur.stride[1]) >> ss_ver) as isize),
    ];
    let mut mask: *mut Av1Filter = ((*f).lf.mask)
        .offset(((sby >> ((*(*f).seq_hdr).sb128 == 0) as libc::c_int) * (*f).sb128w) as isize);
    if (*(*f).c).inloop_filters as libc::c_uint
        & DAV1D_INLOOPFILTER_DEBLOCK as libc::c_int as libc::c_uint
        != 0
        && ((*(*f).frame_hdr).loopfilter.level_y[0] != 0
            || (*(*f).frame_hdr).loopfilter.level_y[1] != 0)
    {
        dav1d_loopfilter_sbrow_rows_16bpc(f, p.as_ptr(), mask, sby);
    }
    if (*(*f).seq_hdr).cdef != 0 || (*f).lf.restore_planes != 0 {
        dav1d_copy_lpf_16bpc(f, p.as_ptr(), sby);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_filter_sbrow_cdef_16bpc(
    tc: *mut Dav1dTaskContext,
    sby: libc::c_int,
) {
    let f: *const Dav1dFrameContext = (*tc).f;
    if (*(*f).c).inloop_filters as libc::c_uint
        & DAV1D_INLOOPFILTER_CDEF as libc::c_int as libc::c_uint
        == 0
    {
        return;
    }
    let sbsz = (*f).sb_step;
    let y = sby * sbsz * 4;
    let ss_ver = ((*f).cur.p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let p: [*mut pixel; 3] = [
        ((*f).lf.p[0] as *mut pixel).offset((y as isize * PXSTRIDE((*f).cur.stride[0])) as isize),
        ((*f).lf.p[1] as *mut pixel)
            .offset((y as isize * PXSTRIDE((*f).cur.stride[1]) >> ss_ver) as isize),
        ((*f).lf.p[2] as *mut pixel)
            .offset((y as isize * PXSTRIDE((*f).cur.stride[1]) >> ss_ver) as isize),
    ];
    let mut prev_mask: *mut Av1Filter = ((*f).lf.mask)
        .offset(((sby - 1 >> ((*(*f).seq_hdr).sb128 == 0) as libc::c_int) * (*f).sb128w) as isize);
    let mut mask: *mut Av1Filter = ((*f).lf.mask)
        .offset(((sby >> ((*(*f).seq_hdr).sb128 == 0) as libc::c_int) * (*f).sb128w) as isize);
    let start = sby * sbsz;
    if sby != 0 {
        let ss_ver_0 = ((*f).cur.p.layout as libc::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint)
            as libc::c_int;
        let mut p_up: [*mut pixel; 3] = [
            (p[0]).offset(-((8 * PXSTRIDE((*f).cur.stride[0])) as isize)),
            (p[1]).offset(-((8 * PXSTRIDE((*f).cur.stride[1]) >> ss_ver_0) as isize)),
            (p[2]).offset(-((8 * PXSTRIDE((*f).cur.stride[1]) >> ss_ver_0) as isize)),
        ];
        dav1d_cdef_brow_16bpc(
            tc,
            p_up.as_mut_ptr() as *const *mut pixel,
            prev_mask,
            start - 2,
            start,
            1 as libc::c_int,
            sby,
        );
    }
    let n_blks = sbsz - 2 * ((sby + 1) < (*f).sbh) as libc::c_int;
    let end = imin(start + n_blks, (*f).bh);
    dav1d_cdef_brow_16bpc(tc, p.as_ptr(), mask, start, end, 0 as libc::c_int, sby);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_filter_sbrow_resize_16bpc(
    f: *mut Dav1dFrameContext,
    sby: libc::c_int,
) {
    let sbsz = (*f).sb_step;
    let y = sby * sbsz * 4;
    let ss_ver = ((*f).cur.p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let p: [*const pixel; 3] = [
        ((*f).lf.p[0] as *mut pixel).offset((y as isize * PXSTRIDE((*f).cur.stride[0])) as isize)
            as *const pixel,
        ((*f).lf.p[1] as *mut pixel)
            .offset((y as isize * PXSTRIDE((*f).cur.stride[1]) >> ss_ver) as isize)
            as *const pixel,
        ((*f).lf.p[2] as *mut pixel)
            .offset((y as isize * PXSTRIDE((*f).cur.stride[1]) >> ss_ver) as isize)
            as *const pixel,
    ];
    let sr_p: [*mut pixel; 3] = [
        ((*f).lf.sr_p[0] as *mut pixel)
            .offset((y as isize * PXSTRIDE((*f).sr_cur.p.stride[0])) as isize),
        ((*f).lf.sr_p[1] as *mut pixel)
            .offset((y as isize * PXSTRIDE((*f).sr_cur.p.stride[1]) >> ss_ver) as isize),
        ((*f).lf.sr_p[2] as *mut pixel)
            .offset((y as isize * PXSTRIDE((*f).sr_cur.p.stride[1]) >> ss_ver) as isize),
    ];
    let has_chroma = ((*f).cur.p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint)
        as libc::c_int;
    let mut pl = 0;
    while pl < 1 + 2 * has_chroma {
        let ss_ver_0 = (pl != 0
            && (*f).cur.p.layout as libc::c_uint
                == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint)
            as libc::c_int;
        let h_start = 8 * (sby != 0) as libc::c_int >> ss_ver_0;
        let dst_stride: ptrdiff_t = (*f).sr_cur.p.stride[(pl != 0) as libc::c_int as usize];
        let mut dst: *mut pixel =
            (sr_p[pl as usize]).offset(-((h_start as isize * PXSTRIDE(dst_stride)) as isize));
        let src_stride: ptrdiff_t = (*f).cur.stride[(pl != 0) as libc::c_int as usize];
        let mut src: *const pixel =
            (p[pl as usize]).offset(-(h_start as isize * PXSTRIDE(src_stride)));
        let h_end =
            4 as libc::c_int * (sbsz - 2 * ((sby + 1) < (*f).sbh) as libc::c_int) >> ss_ver_0;
        let ss_hor = (pl != 0
            && (*f).cur.p.layout as libc::c_uint
                != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint)
            as libc::c_int;
        let dst_w = (*f).sr_cur.p.p.w + ss_hor >> ss_hor;
        let src_w = 4 * (*f).bw + ss_hor >> ss_hor;
        let img_h = (*f).cur.p.h - sbsz * 4 * sby + ss_ver_0 >> ss_ver_0;
        ((*(*f).dsp).mc.resize).expect("non-null function pointer")(
            dst,
            dst_stride,
            src,
            src_stride,
            dst_w,
            imin(img_h, h_end) + h_start,
            src_w,
            (*f).resize_step[(pl != 0) as libc::c_int as usize],
            (*f).resize_start[(pl != 0) as libc::c_int as usize],
            (*f).bitdepth_max,
        );
        pl += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_filter_sbrow_lr_16bpc(f: *mut Dav1dFrameContext, sby: libc::c_int) {
    if (*(*f).c).inloop_filters as libc::c_uint
        & DAV1D_INLOOPFILTER_RESTORATION as libc::c_int as libc::c_uint
        == 0
    {
        return;
    }
    let y = sby * (*f).sb_step * 4;
    let ss_ver = ((*f).cur.p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let sr_p: [*mut pixel; 3] = [
        ((*f).lf.sr_p[0] as *mut pixel).offset(y as isize * PXSTRIDE((*f).sr_cur.p.stride[0])),
        ((*f).lf.sr_p[1] as *mut pixel)
            .offset(y as isize * PXSTRIDE((*f).sr_cur.p.stride[1]) >> ss_ver),
        ((*f).lf.sr_p[2] as *mut pixel)
            .offset(y as isize * PXSTRIDE((*f).sr_cur.p.stride[1]) >> ss_ver),
    ];
    dav1d_lr_sbrow_16bpc(f, sr_p.as_ptr(), sby);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_filter_sbrow_16bpc(f: *mut Dav1dFrameContext, sby: libc::c_int) {
    dav1d_filter_sbrow_deblock_cols_16bpc(f, sby);
    dav1d_filter_sbrow_deblock_rows_16bpc(f, sby);
    if (*(*f).seq_hdr).cdef != 0 {
        dav1d_filter_sbrow_cdef_16bpc((*(*f).c).tc, sby);
    }
    if (*(*f).frame_hdr).width[0] != (*(*f).frame_hdr).width[1] {
        dav1d_filter_sbrow_resize_16bpc(f, sby);
    }
    if (*f).lf.restore_planes != 0 {
        dav1d_filter_sbrow_lr_16bpc(f, sby);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_backup_ipred_edge_16bpc(t: *mut Dav1dTaskContext) {
    let f: *const Dav1dFrameContext = (*t).f;
    let ts: *mut Dav1dTileState = (*t).ts;
    let sby = (*t).by >> (*f).sb_shift;
    let sby_off = (*f).sb128w * 128 * sby;
    let x_off = (*ts).tiling.col_start;
    let y: *const pixel = ((*f).cur.data[0] as *const pixel)
        .offset((x_off * 4) as isize)
        .offset(
            ((((*t).by + (*f).sb_step) * 4 - 1) as isize * PXSTRIDE((*f).cur.stride[0])) as isize,
        );
    memcpy(
        &mut *(*((*f).ipred_edge).as_ptr().offset(0) as *mut pixel)
            .offset((sby_off + x_off * 4) as isize) as *mut pixel as *mut libc::c_void,
        y as *const libc::c_void,
        (4 * ((*ts).tiling.col_end - x_off) << 1) as size_t,
    );
    if (*f).cur.p.layout as libc::c_uint != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint {
        let ss_ver = ((*f).cur.p.layout as libc::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint)
            as libc::c_int;
        let ss_hor = ((*f).cur.p.layout as libc::c_uint
            != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint)
            as libc::c_int;
        let uv_off: ptrdiff_t = (x_off * 4 >> ss_hor) as isize
            + ((((*t).by + (*f).sb_step) * 4 >> ss_ver) - 1) as isize
                * PXSTRIDE((*f).cur.stride[1]);
        let mut pl = 1;
        while pl <= 2 {
            memcpy(
                &mut *(*((*f).ipred_edge).as_ptr().offset(pl as isize) as *mut pixel)
                    .offset((sby_off + (x_off * 4 >> ss_hor)) as isize)
                    as *mut pixel as *mut libc::c_void,
                &*(*((*f).cur.data).as_ptr().offset(pl as isize) as *const pixel)
                    .offset(uv_off as isize) as *const pixel as *const libc::c_void,
                (4 * ((*ts).tiling.col_end - x_off) >> ss_hor << 1) as size_t,
            );
            pl += 1;
        }
    }
}
