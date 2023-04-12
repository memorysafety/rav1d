use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;

use crate::src::cdf::CdfContext;
use crate::src::msac::MsacContext;
use crate::{stdout};
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::size_t,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::size_t,
    ) -> *mut libc::c_void;
    fn fprintf(_: *mut libc::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn llabs(_: libc::c_longlong) -> libc::c_longlong;
    static dav1d_block_dimensions: [[uint8_t; 4]; 22];
    static dav1d_txfm_dimensions: [TxfmInfo; 19];
    static dav1d_txtp_from_uvmode: [uint8_t; 14];
    static dav1d_tx_types_per_set: [uint8_t; 40];
    static dav1d_filter_mode_to_y_mode: [uint8_t; 5];
    static dav1d_lo_ctx_offsets: [[[uint8_t; 5]; 5]; 3];
    static dav1d_skip_ctx: [[uint8_t; 5]; 5];
    static dav1d_tx_type_class: [uint8_t; 17];
    static dav1d_filter_2d: [[uint8_t; 4]; 4];
    fn dav1d_msac_decode_symbol_adapt4(
        s: *mut MsacContext,
        cdf: *mut uint16_t,
        n_symbols: size_t,
    ) -> libc::c_uint;
    fn dav1d_msac_decode_symbol_adapt8(
        s: *mut MsacContext,
        cdf: *mut uint16_t,
        n_symbols: size_t,
    ) -> libc::c_uint;
    fn dav1d_msac_decode_symbol_adapt16(
        s: *mut MsacContext,
        cdf: *mut uint16_t,
        n_symbols: size_t,
    ) -> libc::c_uint;
    fn dav1d_msac_decode_bool_adapt(
        s: *mut MsacContext,
        cdf: *mut uint16_t,
    ) -> libc::c_uint;
    fn dav1d_msac_decode_bool_equi(s: *mut MsacContext) -> libc::c_uint;
    fn dav1d_msac_decode_hi_tok(s: *mut MsacContext, cdf: *mut uint16_t) -> libc::c_uint;
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
    fn dav1d_copy_lpf_16bpc(
        f: *mut Dav1dFrameContext,
        src: *const *mut pixel,
        sby: libc::c_int,
    );
    fn dav1d_lr_sbrow_16bpc(
        f: *mut Dav1dFrameContext,
        dst: *const *mut pixel,
        sby: libc::c_int,
    );
    static dav1d_scans: [*const uint16_t; 19];
    static mut dav1d_wedge_masks: [[[[*const uint8_t; 16]; 2]; 3]; 22];
    static mut dav1d_ii_masks: [[[*const uint8_t; 4]; 3]; 22];
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
use crate::src::internal::Dav1dFrameContext_task_thread;
use crate::src::internal::TaskThreadData;
use crate::include::dav1d::picture::Dav1dPicture;
use crate::include::dav1d::headers::Dav1dITUTT35;
use crate::include::dav1d::headers::Dav1dMasteringDisplay;
use crate::include::dav1d::headers::Dav1dContentLightLevel;

use crate::include::dav1d::headers::Dav1dPixelLayout;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;

use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I400;
use crate::include::dav1d::headers::Dav1dFrameHeader;

use crate::include::dav1d::headers::Dav1dWarpedMotionParams;

use crate::include::dav1d::headers::DAV1D_WM_TYPE_TRANSLATION;

use crate::include::dav1d::headers::Dav1dFilmGrainData;
use crate::include::dav1d::headers::Dav1dSequenceHeader;

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
use crate::src::levels::Av1Block;
use crate::src::levels::mv;

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
use crate::src::internal::Dav1dTaskContext_task_thread;
use crate::src::internal::Dav1dTaskContext_frame_thread;
use crate::src::levels::Filter2d;

use crate::src::levels::FILTER_2D_BILINEAR;

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
    pub cf: *mut coef,
}
use crate::src::internal::Dav1dTileState_tiling;

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

use crate::include::dav1d::dav1d::DAV1D_INLOOPFILTER_RESTORATION;
use crate::include::dav1d::dav1d::DAV1D_INLOOPFILTER_CDEF;
use crate::include::dav1d::dav1d::DAV1D_INLOOPFILTER_DEBLOCK;

use crate::include::dav1d::picture::Dav1dPicAllocator;
use crate::src::internal::Dav1dContext_intra_edge;

use crate::src::intra_edge::EdgeFlags;
use crate::src::intra_edge::EDGE_I420_LEFT_HAS_BOTTOM;

use crate::src::intra_edge::EDGE_I444_LEFT_HAS_BOTTOM;
use crate::src::intra_edge::EDGE_I420_TOP_HAS_RIGHT;

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
use crate::src::looprestoration::LooprestorationParams;

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
use crate::src::levels::TX_64X64;
use crate::src::levels::TX_32X32;
use crate::src::levels::TX_16X16;
use crate::src::levels::TX_8X8;
use crate::src::levels::TX_4X4;
use crate::src::levels::RectTxfmSize;
use crate::src::levels::RTX_4X8;
use crate::src::levels::TxfmType;

use crate::src::levels::WHT_WHT;

use crate::src::levels::H_FLIPADST;
use crate::src::levels::V_FLIPADST;
use crate::src::levels::H_ADST;
use crate::src::levels::V_ADST;
use crate::src::levels::IDTX;
use crate::src::levels::DCT_DCT;
use crate::src::levels::TxClass;
use crate::src::levels::TX_CLASS_V;
use crate::src::levels::TX_CLASS_H;
use crate::src::levels::TX_CLASS_2D;
use crate::src::levels::IntraPredMode;
use crate::src::levels::FILTER_PRED;
use crate::src::levels::CFL_PRED;
use crate::src::levels::SMOOTH_H_PRED;
use crate::src::levels::SMOOTH_V_PRED;
use crate::src::levels::SMOOTH_PRED;
use crate::src::levels::DC_PRED;
use crate::src::levels::II_SMOOTH_PRED;
use crate::src::levels::GLOBALMV;

use crate::src::levels::GLOBALMV_GLOBALMV;

use crate::src::levels::COMP_INTER_NONE;
use crate::src::levels::INTER_INTRA_BLEND;
use crate::src::levels::MM_WARP;
use crate::src::levels::MM_OBMC;

use crate::src::tables::TxfmInfo;
use crate::src::ctx::alias64;
use crate::src::ctx::alias32;
use crate::src::ctx::alias16;
use crate::src::ctx::alias8;
#[inline]
unsafe extern "C" fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 != 0 {
        unreachable!();
    }
    return x >> 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn hex_fdump(
    mut out: *mut libc::FILE,
    mut buf: *const pixel,
    mut stride: ptrdiff_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut what: *const libc::c_char,
) {
    fprintf(out, b"%s\n\0" as *const u8 as *const libc::c_char, what);
    loop {
        let fresh0 = h;
        h = h - 1;
        if !(fresh0 != 0) {
            break;
        }
        let mut x: libc::c_int = 0;
        x = 0 as libc::c_int;
        while x < w {
            fprintf(
                out,
                b" %03x\0" as *const u8 as *const libc::c_char,
                *buf.offset(x as isize) as libc::c_int,
            );
            x += 1;
        }
        buf = buf.offset(PXSTRIDE(stride) as isize);
        fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    };
}
#[inline]
unsafe extern "C" fn hex_dump(
    mut buf: *const pixel,
    mut stride: ptrdiff_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut what: *const libc::c_char,
) {
    hex_fdump(stdout, buf, stride, w, h, what);
}
#[inline]
unsafe extern "C" fn coef_dump(
    mut buf: *const coef,
    w: libc::c_int,
    h: libc::c_int,
    len: libc::c_int,
    mut what: *const libc::c_char,
) {
    let mut y: libc::c_int = 0;
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, what);
    y = 0 as libc::c_int;
    while y < h {
        let mut x: libc::c_int = 0;
        x = 0 as libc::c_int;
        while x < w {
            printf(
                b" %*d\0" as *const u8 as *const libc::c_char,
                len,
                *buf.offset(x as isize),
            );
            x += 1;
        }
        buf = buf.offset(w as isize);
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        y += 1;
    }
}
#[inline]
unsafe extern "C" fn ac_dump(
    mut buf: *const int16_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut what: *const libc::c_char,
) {
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, what);
    loop {
        let fresh1 = h;
        h = h - 1;
        if !(fresh1 != 0) {
            break;
        }
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < w {
            printf(
                b" %03d\0" as *const u8 as *const libc::c_char,
                *buf.offset(x as isize) as libc::c_int,
            );
            x += 1;
        }
        buf = buf.offset(w as isize);
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    };
}
use crate::include::common::intops::imax;
#[inline]
unsafe extern "C" fn imin(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn umin(a: libc::c_uint, b: libc::c_uint) -> libc::c_uint {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn iclip(
    v: libc::c_int,
    min: libc::c_int,
    max: libc::c_int,
) -> libc::c_int {
    return if v < min { min } else if v > max { max } else { v };
}
#[inline]
unsafe extern "C" fn apply_sign64(v: libc::c_int, s: int64_t) -> libc::c_int {
    return if s < 0 { -v } else { v };
}
#[inline]
unsafe extern "C" fn get_uv_inter_txtp(
    uvt_dim: *const TxfmInfo,
    ytxtp: TxfmType,
) -> TxfmType {
    if (*uvt_dim).max as libc::c_int == TX_32X32 as libc::c_int {
        return (if ytxtp as libc::c_uint == IDTX as libc::c_int as libc::c_uint {
            IDTX as libc::c_int
        } else {
            DCT_DCT as libc::c_int
        }) as TxfmType;
    }
    if (*uvt_dim).min as libc::c_int == TX_16X16 as libc::c_int
        && (1 as libc::c_int) << ytxtp as libc::c_uint
            & ((1 as libc::c_int) << H_FLIPADST as libc::c_int
                | (1 as libc::c_int) << V_FLIPADST as libc::c_int
                | (1 as libc::c_int) << H_ADST as libc::c_int
                | (1 as libc::c_int) << V_ADST as libc::c_int) != 0
    {
        return DCT_DCT;
    }
    return ytxtp;
}
#[inline]
unsafe extern "C" fn dav1d_msac_decode_bools(
    s: *mut MsacContext,
    mut n: libc::c_uint,
) -> libc::c_uint {
    let mut v: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop {
        let fresh2 = n;
        n = n.wrapping_sub(1);
        if !(fresh2 != 0) {
            break;
        }
        v = v << 1 as libc::c_int | dav1d_msac_decode_bool_equi(s);
    }
    return v;
}
#[inline]
unsafe extern "C" fn sm_flag(b: *const BlockContext, idx: libc::c_int) -> libc::c_int {
    if (*b).intra[idx as usize] == 0 {
        return 0 as libc::c_int;
    }
    let m: IntraPredMode = (*b).mode[idx as usize] as IntraPredMode;
    return if m as libc::c_uint == SMOOTH_PRED as libc::c_int as libc::c_uint
        || m as libc::c_uint == SMOOTH_H_PRED as libc::c_int as libc::c_uint
        || m as libc::c_uint == SMOOTH_V_PRED as libc::c_int as libc::c_uint
    {
        512 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn sm_uv_flag(
    b: *const BlockContext,
    idx: libc::c_int,
) -> libc::c_int {
    let m: IntraPredMode = (*b).uvmode[idx as usize] as IntraPredMode;
    return if m as libc::c_uint == SMOOTH_PRED as libc::c_int as libc::c_uint
        || m as libc::c_uint == SMOOTH_H_PRED as libc::c_int as libc::c_uint
        || m as libc::c_uint == SMOOTH_V_PRED as libc::c_int as libc::c_uint
    {
        512 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn read_golomb(msac: *mut MsacContext) -> libc::c_uint {
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut val: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    while dav1d_msac_decode_bool_equi(msac) == 0 && len < 32 as libc::c_int {
        len += 1;
    }
    loop {
        let fresh3 = len;
        len = len - 1;
        if !(fresh3 != 0) {
            break;
        }
        val = (val << 1 as libc::c_int).wrapping_add(dav1d_msac_decode_bool_equi(msac));
    }
    return val.wrapping_sub(1 as libc::c_int as libc::c_uint);
}
// If the C macro is called like `MERGE_CTX(a, uint8_t,  0x40)`, the
// corresponding call to this macro is `MERGE_CTX(ca, a, uint8_t,  0x40)`.
macro_rules! MERGE_CTX {
    ($dest:ident, $dir:ident, $type:tt, $no_val:literal) => {
        $dest = (*($dir as *const $type) != $no_val as $type) as libc::c_uint
    }
}
// corresponds to the second definition of MERGE_CTX inside get_skip_ctx.
macro_rules! MERGE_CTX_TX {
    ($dest:ident, $dir:ident, $type:tt, $tx:expr) => {
        {
            if $tx as libc::c_int == TX_64X64 as libc::c_int {
                let mut tmp: uint64_t = *($dir as *const uint64_t);
                tmp |= *(&*$dir.offset(8) as *const uint8_t
                        as *const uint64_t);
                $dest = tmp.wrapping_shr(32) as libc::c_uint | tmp as libc::c_uint;
            } else {
                $dest = *($dir as *const $type) as libc::c_uint;
            }
            if $tx as libc::c_int == TX_32X32 as libc::c_int {
                let off = ::core::mem::size_of::<$type>() as isize;
                $dest |= *(&*$dir.offset(off) as *const uint8_t as *const $type) as libc::c_uint;
            }
            if $tx as libc::c_int >= TX_16X16 as libc::c_int {
                $dest |= $dest >> 16 as libc::c_int;
            }
            if $tx as libc::c_int >= TX_8X8 as libc::c_int {
                $dest |= $dest >> 8 as libc::c_int;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn get_skip_ctx(
    t_dim: *const TxfmInfo,
    bs: BlockSize,
    a: *const uint8_t,
    l: *const uint8_t,
    chroma: libc::c_int,
    layout: Dav1dPixelLayout,
) -> libc::c_uint {
    let b_dim: *const uint8_t = (dav1d_block_dimensions[bs as usize]).as_ptr();
    if chroma != 0 {
        let ss_ver: libc::c_int = (layout as libc::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
        let ss_hor: libc::c_int = (layout as libc::c_uint
            != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
        let not_one_blk: libc::c_int = (*b_dim.offset(2 as libc::c_int as isize)
            as libc::c_int
            - (*b_dim.offset(2 as libc::c_int as isize) != 0 && ss_hor != 0)
                as libc::c_int > (*t_dim).lw as libc::c_int
            || *b_dim.offset(3 as libc::c_int as isize) as libc::c_int
                - (*b_dim.offset(3 as libc::c_int as isize) != 0 && ss_ver != 0)
                    as libc::c_int > (*t_dim).lh as libc::c_int) as libc::c_int;
        let mut ca: libc::c_uint = 0;
        let mut cl: libc::c_uint = 0;
        match (*t_dim).lw as libc::c_uint {
            TX_4X4 =>   MERGE_CTX!(ca, a, uint8_t,  0x40),
            TX_8X8 =>   MERGE_CTX!(ca, a, uint16_t, 0x4040),
            TX_16X16 => MERGE_CTX!(ca, a, uint32_t, 0x40404040),
            TX_32X32 => MERGE_CTX!(ca, a, uint64_t, 0x4040404040404040u64),
            _ => unreachable!()
        }
        match (*t_dim).lh as libc::c_uint {
            TX_4X4 =>   MERGE_CTX!(cl, l, uint8_t,  0x40),
            TX_8X8 =>   MERGE_CTX!(cl, l, uint16_t, 0x4040),
            TX_16X16 => MERGE_CTX!(cl, l, uint32_t, 0x40404040),
            TX_32X32 => MERGE_CTX!(cl, l, uint64_t, 0x4040404040404040u64),
            _ => unreachable!()
        }
        return ((7 as libc::c_int + not_one_blk * 3 as libc::c_int) as libc::c_uint)
            .wrapping_add(ca)
            .wrapping_add(cl);
    } else if *b_dim.offset(2 as libc::c_int as isize) as libc::c_int
        == (*t_dim).lw as libc::c_int
        && *b_dim.offset(3 as libc::c_int as isize) as libc::c_int
            == (*t_dim).lh as libc::c_int
    {
        return 0 as libc::c_int as libc::c_uint
    } else {
        let mut la: libc::c_uint = 0;
        let mut ll: libc::c_uint = 0;
        let mut _current_block_80: u64;
        match (*t_dim).lw  as libc::c_uint {
            TX_4X4 =>   MERGE_CTX_TX!(la, a, uint8_t,  TX_4X4),
            TX_8X8 =>   MERGE_CTX_TX!(la, a, uint16_t, TX_8X8),
            TX_16X16 => MERGE_CTX_TX!(la, a, uint32_t, TX_16X16),
            TX_32X32 => MERGE_CTX_TX!(la, a, uint32_t, TX_32X32),
            TX_64X64 => MERGE_CTX_TX!(la, a, uint32_t, TX_64X64),
            _ => unreachable!()
        }
        match (*t_dim).lh as libc::c_uint {
            TX_4X4 =>   MERGE_CTX_TX!(ll, l, uint8_t,  TX_4X4),
            TX_8X8 =>   MERGE_CTX_TX!(ll, l, uint16_t, TX_8X8),
            TX_16X16 => MERGE_CTX_TX!(ll, l, uint32_t, TX_16X16),
            TX_32X32 => MERGE_CTX_TX!(ll, l, uint32_t, TX_32X32),
            TX_64X64 => MERGE_CTX_TX!(ll, l, uint32_t, TX_64X64),
            _ => unreachable!()
        }
        return dav1d_skip_ctx[umin(
            la & 0x3f as libc::c_int as libc::c_uint,
            4 as libc::c_int as libc::c_uint,
        )
            as usize][umin(
            ll & 0x3f as libc::c_int as libc::c_uint,
            4 as libc::c_int as libc::c_uint,
        ) as usize] as libc::c_uint;
    };
}
#[inline]
unsafe extern "C" fn get_dc_sign_ctx(
    tx: libc::c_int,
    a: *const uint8_t,
    l: *const uint8_t,
) -> libc::c_uint {
    let mut mask: uint64_t = 0xc0c0c0c0c0c0c0c0 as libc::c_ulonglong as uint64_t;
    let mut mul: uint64_t = 0x101010101010101 as libc::c_ulonglong as uint64_t;
    let mut s: libc::c_int = 0;
    let mut current_block_66: u64;
    match tx {
        0 => {
            current_block_66 = 6873731126896040597;
        }
        1 => {
            let mut t_0: uint32_t = *(a as *const uint16_t) as libc::c_uint
                & mask as uint32_t;
            t_0 = (t_0 as libc::c_uint)
                .wrapping_add(*(l as *const uint16_t) as libc::c_uint & mask as uint32_t)
                as uint32_t as uint32_t;
            t_0 = (t_0 as libc::c_uint).wrapping_mul(0x4040404 as libc::c_uint)
                as uint32_t as uint32_t;
            s = (t_0 >> 24 as libc::c_int) as libc::c_int - 2 as libc::c_int
                - 2 as libc::c_int;
            current_block_66 = 2606304779496145856;
        }
        2 => {
            let mut t_1: uint32_t = (*(a as *const uint32_t) & mask as uint32_t)
                >> 6 as libc::c_int;
            t_1 = (t_1 as libc::c_uint)
                .wrapping_add(
                    (*(l as *const uint32_t) & mask as uint32_t) >> 6 as libc::c_int,
                ) as uint32_t as uint32_t;
            t_1 = (t_1 as libc::c_uint).wrapping_mul(mul as uint32_t) as uint32_t
                as uint32_t;
            s = (t_1 >> 24 as libc::c_int) as libc::c_int - 4 as libc::c_int
                - 4 as libc::c_int;
            current_block_66 = 2606304779496145856;
        }
        3 => {
            let mut t_2: uint64_t = (*(a as *const uint64_t) & mask) >> 6 as libc::c_int;
            t_2 = t_2
                .wrapping_add((*(l as *const uint64_t) & mask) >> 6 as libc::c_int)
                as uint64_t as uint64_t;
            t_2 = t_2.wrapping_mul(mul) as uint64_t as uint64_t;
            s = t_2.wrapping_shr(56) as libc::c_int - 8 as libc::c_int
                - 8 as libc::c_int;
            current_block_66 = 2606304779496145856;
        }
        4 => {
            let mut t_3: uint64_t = (*(&*a.offset(0 as libc::c_int as isize)
                as *const uint8_t as *const uint64_t) & mask) >> 6 as libc::c_int;
            t_3 = t_3
                .wrapping_add(
                    (*(&*a.offset(8 as libc::c_int as isize) as *const uint8_t
                        as *const uint64_t) & mask) >> 6 as libc::c_int,
                ) as uint64_t as uint64_t;
            t_3 = t_3
                .wrapping_add(
                    (*(&*l.offset(0 as libc::c_int as isize) as *const uint8_t
                        as *const uint64_t) & mask) >> 6 as libc::c_int,
                ) as uint64_t as uint64_t;
            t_3 = t_3
                .wrapping_add(
                    (*(&*l.offset(8 as libc::c_int as isize) as *const uint8_t
                        as *const uint64_t) & mask) >> 6 as libc::c_int,
                ) as uint64_t as uint64_t;
            t_3 = t_3.wrapping_mul(mul) as uint64_t as uint64_t;
            s = t_3.wrapping_shr(56) as libc::c_int - 16 as libc::c_int
                - 16 as libc::c_int;
            current_block_66 = 2606304779496145856;
        }
        5 => {
            let mut t_4: uint32_t = *a as libc::c_uint & mask as uint32_t;
            t_4 = (t_4 as libc::c_uint)
                .wrapping_add(*(l as *const uint16_t) as libc::c_uint & mask as uint32_t)
                as uint32_t as uint32_t;
            t_4 = t_4.wrapping_mul(0x4040404 as libc::c_uint)
                as uint32_t as uint32_t;
            s = (t_4 >> 24 as libc::c_int) as libc::c_int - 1 as libc::c_int
                - 2 as libc::c_int;
            current_block_66 = 2606304779496145856;
        }
        6 => {
            let mut t_5: uint32_t = *(a as *const uint16_t) as libc::c_uint
                & mask as uint32_t;
            t_5 = (t_5 as libc::c_uint)
                .wrapping_add(*l as libc::c_uint & mask as uint32_t) as uint32_t
                as uint32_t;
            t_5 = (t_5 as libc::c_uint).wrapping_mul(0x4040404 as libc::c_uint)
                as uint32_t as uint32_t;
            s = (t_5 >> 24 as libc::c_int) as libc::c_int - 2 as libc::c_int
                - 1 as libc::c_int;
            current_block_66 = 2606304779496145856;
        }
        7 => {
            let mut t_6: uint32_t = *(a as *const uint16_t) as libc::c_uint
                & mask as uint32_t;
            t_6 = (t_6 as libc::c_uint)
                .wrapping_add(*(l as *const uint32_t) & mask as uint32_t) as uint32_t
                as uint32_t;
            t_6 = (t_6 >> 6 as libc::c_int).wrapping_mul(mul as uint32_t);
            s = (t_6 >> 24 as libc::c_int) as libc::c_int - 2 as libc::c_int
                - 4 as libc::c_int;
            current_block_66 = 2606304779496145856;
        }
        8 => {
            let mut t_7: uint32_t = *(a as *const uint32_t) & mask as uint32_t;
            t_7 = (t_7 as libc::c_uint)
                .wrapping_add(*(l as *const uint16_t) as libc::c_uint & mask as uint32_t)
                as uint32_t as uint32_t;
            t_7 = (t_7 >> 6 as libc::c_int).wrapping_mul(mul as uint32_t);
            s = (t_7 >> 24 as libc::c_int) as libc::c_int - 4 as libc::c_int
                - 2 as libc::c_int;
            current_block_66 = 2606304779496145856;
        }
        9 => {
            let mut t_8: uint64_t = (*(a as *const uint32_t) & mask as uint32_t)
                as uint64_t;
            t_8 = t_8.wrapping_add(*(l as *const uint64_t) & mask)
                as uint64_t as uint64_t;
            t_8 = (t_8 >> 6 as libc::c_int).wrapping_mul(mul);
            s = t_8.wrapping_shr(56) as libc::c_int - 4 as libc::c_int
                - 8 as libc::c_int;
            current_block_66 = 2606304779496145856;
        }
        10 => {
            let mut t_9: uint64_t = *(a as *const uint64_t) & mask;
            t_9 = t_9.wrapping_add(
                    (*(l as *const uint32_t) & mask as uint32_t) as uint64_t,
                ) as uint64_t as uint64_t;
            t_9 = (t_9 >> 6 as libc::c_int).wrapping_mul(mul);
            s = t_9.wrapping_shr(56) as libc::c_int - 8 as libc::c_int
                - 4 as libc::c_int;
            current_block_66 = 2606304779496145856;
        }
        11 => {
            let mut t_10: uint64_t = (*(&*a.offset(0 as libc::c_int as isize)
                as *const uint8_t as *const uint64_t) & mask) >> 6 as libc::c_int;
            t_10 = t_10.wrapping_add(
                    (*(&*l.offset(0 as libc::c_int as isize) as *const uint8_t
                        as *const uint64_t) & mask) >> 6 as libc::c_int,
                ) as uint64_t as uint64_t;
            t_10 = t_10.wrapping_add(
                    (*(&*l.offset(8 as libc::c_int as isize) as *const uint8_t
                        as *const uint64_t) & mask) >> 6 as libc::c_int,
                ) as uint64_t as uint64_t;
            t_10 = t_10.wrapping_mul(mul) as uint64_t as uint64_t;
            s = t_10.wrapping_shr(56) as libc::c_int - 8 as libc::c_int
                - 16 as libc::c_int;
            current_block_66 = 2606304779496145856;
        }
        12 => {
            let mut t_11: uint64_t = (*(&*a.offset(0 as libc::c_int as isize)
                as *const uint8_t as *const uint64_t) & mask) >> 6 as libc::c_int;
            t_11 = t_11.wrapping_add(
                    (*(&*a.offset(8 as libc::c_int as isize) as *const uint8_t
                        as *const uint64_t) & mask) >> 6 as libc::c_int,
                ) as uint64_t as uint64_t;
            t_11 = t_11
                .wrapping_add(
                    (*(&*l.offset(0 as libc::c_int as isize) as *const uint8_t
                        as *const uint64_t) & mask) >> 6 as libc::c_int,
                ) as uint64_t as uint64_t;
            t_11 = t_11.wrapping_mul(mul) as uint64_t as uint64_t;
            s = t_11.wrapping_shr(56) as libc::c_int - 16 as libc::c_int
                - 8 as libc::c_int;
            current_block_66 = 2606304779496145856;
        }
        13 => {
            let mut t_12: uint32_t = *a as libc::c_uint & mask as uint32_t;
            t_12 = (t_12 as libc::c_uint)
                .wrapping_add(*(l as *const uint32_t) & mask as uint32_t) as uint32_t
                as uint32_t;
            t_12 = (t_12 >> 6 as libc::c_int).wrapping_mul(mul as uint32_t);
            s = (t_12 >> 24 as libc::c_int) as libc::c_int - 1 as libc::c_int
                - 4 as libc::c_int;
            current_block_66 = 2606304779496145856;
        }
        14 => {
            let mut t_13: uint32_t = *(a as *const uint32_t) & mask as uint32_t;
            t_13 = (t_13 as libc::c_uint)
                .wrapping_add(*l as libc::c_uint & mask as uint32_t) as uint32_t
                as uint32_t;
            t_13 = (t_13 >> 6 as libc::c_int).wrapping_mul(mul as uint32_t);
            s = (t_13 >> 24 as libc::c_int) as libc::c_int - 4 as libc::c_int
                - 1 as libc::c_int;
            current_block_66 = 2606304779496145856;
        }
        15 => {
            let mut t_14: uint64_t = (*(a as *const uint16_t) as libc::c_uint
                & mask as uint32_t) as uint64_t;
            t_14 = t_14.wrapping_add(*(l as *const uint64_t) & mask);
            t_14 = (t_14 >> 6 as libc::c_int).wrapping_mul(mul);
            s = t_14.wrapping_shr(56) as libc::c_int - 2 as libc::c_int
                - 8 as libc::c_int;
            current_block_66 = 2606304779496145856;
        }
        16 => {
            let mut t_15: uint64_t = *(a as *const uint64_t) & mask;
            t_15 = t_15 + (*(l as *const uint16_t) as libc::c_uint & mask as uint32_t) as uint64_t;
            t_15 = (t_15 >> 6 as libc::c_int).wrapping_mul(mul);
            s = t_15.wrapping_shr(56) as libc::c_int - 8 as libc::c_int
                - 2 as libc::c_int;
            current_block_66 = 2606304779496145856;
        }
        17 => {
            let mut t_16: uint64_t = (*(a as *const uint32_t) & mask as uint32_t)
                as uint64_t;
            t_16 = t_16.wrapping_add(
                    *(&*l.offset(0 as libc::c_int as isize) as *const uint8_t
                        as *const uint64_t) & mask,
                ) as uint64_t as uint64_t;
            t_16 = (t_16 >> 6 as libc::c_int)
                .wrapping_add(
                    (*(&*l.offset(8 as libc::c_int as isize) as *const uint8_t
                        as *const uint64_t) & mask) >> 6 as libc::c_int,
                );
            t_16 = t_16.wrapping_mul(mul) as uint64_t as uint64_t;
            s = t_16.wrapping_shr(56) as libc::c_int - 4 as libc::c_int
                - 16 as libc::c_int;
            current_block_66 = 2606304779496145856;
        }
        18 => {
            let mut t_17: uint64_t = *(&*a.offset(0 as libc::c_int as isize)
                as *const uint8_t as *const uint64_t) & mask;
            t_17 = t_17 + (*(l as *const uint32_t) & mask as uint32_t) as uint64_t;
            t_17 = (t_17 >> 6 as libc::c_int)
                .wrapping_add(
                    (*(&*a.offset(8 as libc::c_int as isize) as *const uint8_t
                        as *const uint64_t) & mask) >> 6 as libc::c_int,
                );
            t_17 = t_17.wrapping_mul(mul) as uint64_t as uint64_t;
            s = t_17.wrapping_shr(56) as libc::c_int - 16 as libc::c_int
                - 4 as libc::c_int;
            current_block_66 = 2606304779496145856;
        }
        _ => {
            if 0 as libc::c_int == 0 {
                unreachable!();
            }
            current_block_66 = 6873731126896040597;
        }
    }
    match current_block_66 {
        6873731126896040597 => {
            let mut t: libc::c_int = *a as libc::c_int >> 6 as libc::c_int;
            t += *l as libc::c_int >> 6 as libc::c_int;
            s = t - 1 as libc::c_int - 1 as libc::c_int;
        }
        _ => {}
    }
    return ((s != 0 as libc::c_int) as libc::c_int
        + (s > 0 as libc::c_int) as libc::c_int) as libc::c_uint;
}
#[inline]
unsafe extern "C" fn get_lo_ctx(
    levels: *const uint8_t,
    tx_class: TxClass,
    hi_mag: *mut libc::c_uint,
    ctx_offsets: *const [uint8_t; 5],
    x: libc::c_uint,
    y: libc::c_uint,
    stride: ptrdiff_t,
) -> libc::c_uint {
    let mut mag: libc::c_uint = (*levels
        .offset(
            (0 * stride
                + 1) as isize,
        ) as libc::c_int
        + *levels
            .offset(
                (1 * stride
                    + 0) as isize,
            ) as libc::c_int) as libc::c_uint;
    let mut offset: libc::c_uint = 0;
    if tx_class as libc::c_uint == TX_CLASS_2D as libc::c_int as libc::c_uint {
        mag = mag
            .wrapping_add(
                *levels
                    .offset(
                        (1 * stride
                            + 1) as isize,
                    ) as libc::c_uint,
            );
        *hi_mag = mag;
        mag = mag
            .wrapping_add(
                (*levels
                    .offset(
                        0 * stride as isize + 2,
                    ) as libc::c_int
                    + *levels
                        .offset(
                            2 * stride as isize + 0,
                        ) as libc::c_int) as libc::c_uint,
            );
        offset = (*ctx_offsets
            .offset(
                umin(y, 4 as libc::c_int as libc::c_uint) as isize,
            ))[umin(x, 4 as libc::c_int as libc::c_uint) as usize] as libc::c_uint;
    } else {
        mag = mag
            .wrapping_add(
                *levels
                    .offset( 0 * stride as isize + 2,
                    ) as libc::c_uint,
            );
        *hi_mag = mag;
        mag = mag
            .wrapping_add(
                (*levels
                    .offset(
                        0 * stride as isize + 3,
                    ) as libc::c_int
                    + *levels
                        .offset(
                            (0 * stride
                                + 4) as isize,
                        ) as libc::c_int) as libc::c_uint,
            );
        offset = (26 as libc::c_int as libc::c_uint)
            .wrapping_add(
                if y > 1 as libc::c_int as libc::c_uint {
                    10 as libc::c_int as libc::c_uint
                } else {
                    y.wrapping_mul(5 as libc::c_int as libc::c_uint)
                },
            );
    }
    return offset
        .wrapping_add(
            if mag > 512 as libc::c_int as libc::c_uint {
                4 as libc::c_int as libc::c_uint
            } else {
                mag.wrapping_add(64 as libc::c_int as libc::c_uint) >> 7 as libc::c_int
            },
        );
}
unsafe extern "C" fn decode_coefs(
    t: *mut Dav1dTaskContext,
    a: *mut uint8_t,
    l: *mut uint8_t,
    tx: RectTxfmSize,
    bs: BlockSize,
    b: *const Av1Block,
    intra: libc::c_int,
    plane: libc::c_int,
    mut cf: *mut coef,
    txtp: *mut TxfmType,
    mut res_ctx: *mut uint8_t,
) -> libc::c_int {
    let mut dc_sign_ctx: libc::c_int = 0;
    let mut dc_sign_cdf: *mut uint16_t = 0 as *mut uint16_t;
    let mut dc_sign: libc::c_int = 0;
    let mut dc_dq: libc::c_int = 0;
    let mut current_block: u64;
    let ts: *mut Dav1dTileState = (*t).ts;
    let chroma: libc::c_int = (plane != 0) as libc::c_int;
    let f: *const Dav1dFrameContext = (*t).f;
    let lossless: libc::c_int = (*(*f).frame_hdr)
        .segmentation
        .lossless[(*b).seg_id as usize];
    let t_dim: *const TxfmInfo = &*dav1d_txfm_dimensions.as_ptr().offset(tx as isize)
        as *const TxfmInfo;
    let dbg: libc::c_int = (0 as libc::c_int != 0
        && (*(*f).frame_hdr).frame_offset == 2 as libc::c_int
        && (*t).by >= 0 as libc::c_int && (*t).by < 4 as libc::c_int
        && (*t).bx >= 8 as libc::c_int && (*t).bx < 12 as libc::c_int && plane != 0
        && 0 as libc::c_int != 0) as libc::c_int;
    if dbg != 0 {
        printf(b"Start: r=%d\n\0" as *const u8 as *const libc::c_char, (*ts).msac.rng);
    }
    let sctx: libc::c_int = get_skip_ctx(t_dim, bs, a, l, chroma, (*f).cur.p.layout)
        as libc::c_int;
    let all_skip: libc::c_int = dav1d_msac_decode_bool_adapt(
        &mut (*ts).msac,
        ((*ts).cdf.coef.skip[(*t_dim).ctx as usize][sctx as usize]).as_mut_ptr(),
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
            get_uv_inter_txtp(t_dim, *txtp) as libc::c_uint
        }) as TxfmType;
    } else if (*(*f).frame_hdr).segmentation.qidx[(*b).seg_id as usize] == 0 {
        *txtp = DCT_DCT;
    } else {
        let mut idx: libc::c_uint = 0;
        if intra != 0 {
            let y_mode_nofilt: IntraPredMode = (if (*b)
                .c2rust_unnamed
                .c2rust_unnamed
                .y_mode as libc::c_int == FILTER_PRED as libc::c_int
            {
                dav1d_filter_mode_to_y_mode[(*b).c2rust_unnamed.c2rust_unnamed.y_angle
                    as usize] as libc::c_int
            } else {
                (*b).c2rust_unnamed.c2rust_unnamed.y_mode as libc::c_int
            }) as IntraPredMode;
            if (*(*f).frame_hdr).reduced_txtp_set != 0
                || (*t_dim).min as libc::c_int == TX_16X16 as libc::c_int
            {
                idx = dav1d_msac_decode_symbol_adapt4(
                    &mut (*ts).msac,
                    ((*ts)
                        .cdf
                        .m
                        .txtp_intra2[(*t_dim).min as usize][y_mode_nofilt as usize])
                        .as_mut_ptr(),
                    4 as libc::c_int as size_t,
                );
                *txtp = dav1d_tx_types_per_set[idx
                    .wrapping_add(0 as libc::c_int as libc::c_uint) as usize]
                    as TxfmType;
            } else {
                idx = dav1d_msac_decode_symbol_adapt8(
                    &mut (*ts).msac,
                    ((*ts)
                        .cdf
                        .m
                        .txtp_intra1[(*t_dim).min as usize][y_mode_nofilt as usize])
                        .as_mut_ptr(),
                    6 as libc::c_int as size_t,
                );
                *txtp = dav1d_tx_types_per_set[idx
                    .wrapping_add(5 as libc::c_int as libc::c_uint) as usize]
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
                    ((*ts).cdf.m.txtp_inter3[(*t_dim).min as usize]).as_mut_ptr(),
                );
                *txtp = (idx.wrapping_sub(1 as libc::c_int as libc::c_uint)
                    & IDTX as libc::c_int as libc::c_uint) as TxfmType;
            } else if (*t_dim).min as libc::c_int == TX_16X16 as libc::c_int {
                idx = dav1d_msac_decode_symbol_adapt16(
                    &mut (*ts).msac,
                    ((*ts).cdf.m.txtp_inter2.0).as_mut_ptr(),
                    11 as libc::c_int as size_t,
                );
                *txtp = dav1d_tx_types_per_set[idx
                    .wrapping_add(12 as libc::c_int as libc::c_uint) as usize]
                    as TxfmType;
            } else {
                idx = dav1d_msac_decode_symbol_adapt16(
                    &mut (*ts).msac,
                    ((*ts).cdf.m.txtp_inter1[(*t_dim).min as usize]).as_mut_ptr(),
                    15 as libc::c_int as size_t,
                );
                *txtp = dav1d_tx_types_per_set[idx
                    .wrapping_add(24 as libc::c_int as libc::c_uint) as usize]
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
    let mut eob_bin: libc::c_int = 0;
    let tx2dszctx: libc::c_int = imin(
        (*t_dim).lw as libc::c_int,
        TX_32X32 as libc::c_int,
    ) + imin((*t_dim).lh as libc::c_int, TX_32X32 as libc::c_int);
    let tx_class: TxClass = dav1d_tx_type_class[*txtp as usize] as TxClass;
    let is_1d: libc::c_int = (tx_class as libc::c_uint
        != TX_CLASS_2D as libc::c_int as libc::c_uint) as libc::c_int;
    match tx2dszctx {
        0 => {
            let eob_bin_cdf: *mut uint16_t = ((*ts)
                .cdf
                .coef
                .eob_bin_16[chroma as usize][is_1d as usize])
                .as_mut_ptr();
            eob_bin = dav1d_msac_decode_symbol_adapt4(
                &mut (*ts).msac,
                eob_bin_cdf,
                (4 as libc::c_int + 0 as libc::c_int) as size_t,
            ) as libc::c_int;
        }
        1 => {
            let eob_bin_cdf_0: *mut uint16_t = ((*ts)
                .cdf
                .coef
                .eob_bin_32[chroma as usize][is_1d as usize])
                .as_mut_ptr();
            eob_bin = dav1d_msac_decode_symbol_adapt8(
                &mut (*ts).msac,
                eob_bin_cdf_0,
                (4 as libc::c_int + 1 as libc::c_int) as size_t,
            ) as libc::c_int;
        }
        2 => {
            let eob_bin_cdf_1: *mut uint16_t = ((*ts)
                .cdf
                .coef
                .eob_bin_64[chroma as usize][is_1d as usize])
                .as_mut_ptr();
            eob_bin = dav1d_msac_decode_symbol_adapt8(
                &mut (*ts).msac,
                eob_bin_cdf_1,
                (4 as libc::c_int + 2 as libc::c_int) as size_t,
            ) as libc::c_int;
        }
        3 => {
            let eob_bin_cdf_2: *mut uint16_t = ((*ts)
                .cdf
                .coef
                .eob_bin_128[chroma as usize][is_1d as usize])
                .as_mut_ptr();
            eob_bin = dav1d_msac_decode_symbol_adapt8(
                &mut (*ts).msac,
                eob_bin_cdf_2,
                (4 as libc::c_int + 3 as libc::c_int) as size_t,
            ) as libc::c_int;
        }
        4 => {
            let eob_bin_cdf_3: *mut uint16_t = ((*ts)
                .cdf
                .coef
                .eob_bin_256[chroma as usize][is_1d as usize])
                .as_mut_ptr();
            eob_bin = dav1d_msac_decode_symbol_adapt16(
                &mut (*ts).msac,
                eob_bin_cdf_3,
                (4 as libc::c_int + 4 as libc::c_int) as size_t,
            ) as libc::c_int;
        }
        5 => {
            let eob_bin_cdf_4: *mut uint16_t = ((*ts)
                .cdf
                .coef
                .eob_bin_512[chroma as usize])
                .as_mut_ptr();
            eob_bin = dav1d_msac_decode_symbol_adapt16(
                &mut (*ts).msac,
                eob_bin_cdf_4,
                (4 as libc::c_int + 5 as libc::c_int) as size_t,
            ) as libc::c_int;
        }
        6 => {
            let eob_bin_cdf_5: *mut uint16_t = ((*ts)
                .cdf
                .coef
                .eob_bin_1024[chroma as usize])
                .as_mut_ptr();
            eob_bin = dav1d_msac_decode_symbol_adapt16(
                &mut (*ts).msac,
                eob_bin_cdf_5,
                (4 as libc::c_int + 6 as libc::c_int) as size_t,
            ) as libc::c_int;
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
    let mut eob: libc::c_int = 0;
    if eob_bin > 1 as libc::c_int {
        let eob_hi_bit_cdf: *mut uint16_t = ((*ts)
            .cdf
            .coef
            .eob_hi_bit[(*t_dim).ctx as usize][chroma as usize][eob_bin as usize])
            .as_mut_ptr();
        let eob_hi_bit: libc::c_int = dav1d_msac_decode_bool_adapt(
            &mut (*ts).msac,
            eob_hi_bit_cdf,
        ) as libc::c_int;
        if dbg != 0 {
            printf(
                b"Post-eob_hi_bit[%d][%d][%d][%d]: r=%d\n\0" as *const u8
                    as *const libc::c_char,
                (*t_dim).ctx as libc::c_int,
                chroma,
                eob_bin,
                eob_hi_bit,
                (*ts).msac.rng,
            );
        }
        eob = (((eob_hi_bit | 2 as libc::c_int) << eob_bin - 2 as libc::c_int)
            as libc::c_uint
            | dav1d_msac_decode_bools(
                &mut (*ts).msac,
                (eob_bin - 2 as libc::c_int) as libc::c_uint,
            )) as libc::c_int;
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
    if !(eob >= 0 as libc::c_int) {
        unreachable!();
    }
    let eob_cdf: *mut [uint16_t; 4] = ((*ts)
        .cdf
        .coef
        .eob_base_tok[(*t_dim).ctx as usize][chroma as usize])
        .as_mut_ptr();
    let hi_cdf: *mut [uint16_t; 4] = ((*ts)
        .cdf
        .coef
        .br_tok[imin((*t_dim).ctx as libc::c_int, 3 as libc::c_int)
        as usize][chroma as usize])
        .as_mut_ptr();
    let mut rc: libc::c_uint = 0;
    let mut dc_tok: libc::c_uint = 0;
    if eob != 0 {
        let lo_cdf: *mut [uint16_t; 4] = ((*ts)
            .cdf
            .coef
            .base_tok[(*t_dim).ctx as usize][chroma as usize])
            .as_mut_ptr();
        let levels: *mut uint8_t = ((*t).scratch.c2rust_unnamed_0.c2rust_unnamed.levels)
            .as_mut_ptr();
        let sw: libc::c_int = imin((*t_dim).w as libc::c_int, 8 as libc::c_int);
        let sh: libc::c_int = imin((*t_dim).h as libc::c_int, 8 as libc::c_int);
        let mut ctx: libc::c_uint = (1 as libc::c_int
            + (eob > sw * sh * 2 as libc::c_int) as libc::c_int
            + (eob > sw * sh * 4 as libc::c_int) as libc::c_int) as libc::c_uint;
        let mut eob_tok: libc::c_int = dav1d_msac_decode_symbol_adapt4(
            &mut (*ts).msac,
            (*eob_cdf.offset(ctx as isize)).as_mut_ptr(),
            2 as libc::c_int as size_t,
        ) as libc::c_int;
        let mut tok: libc::c_int = eob_tok + 1 as libc::c_int;
        let mut level_tok: libc::c_int = tok * 0x41 as libc::c_int;
        let mut mag: libc::c_uint = 0;
        let mut scan: *const uint16_t = 0 as *const uint16_t;
        match tx_class as libc::c_uint {
            0 => {
                let nonsquare_tx: libc::c_uint = (tx as libc::c_uint
                    >= RTX_4X8 as libc::c_int as libc::c_uint) as libc::c_int
                    as libc::c_uint;
                let lo_ctx_offsets: *const [uint8_t; 5] = (dav1d_lo_ctx_offsets[nonsquare_tx
                    .wrapping_add(tx as libc::c_uint & nonsquare_tx) as usize])
                    .as_ptr();
                scan = dav1d_scans[tx as usize];
                let stride: ptrdiff_t = (4 as libc::c_int * sh) as ptrdiff_t;
                let shift: libc::c_uint = (if ((*t_dim).lh as libc::c_int)
                    < 4 as libc::c_int
                {
                    (*t_dim).lh as libc::c_int + 2 as libc::c_int
                } else {
                    5 as libc::c_int
                }) as libc::c_uint;
                let shift2: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                let mask: libc::c_uint = (4 as libc::c_int * sh - 1 as libc::c_int)
                    as libc::c_uint;
                memset(
                    levels as *mut libc::c_void,
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
                if eob_tok == 2 as libc::c_int {
                    ctx = (if if TX_CLASS_2D as libc::c_int == TX_CLASS_2D as libc::c_int
                    {
                        (x | y > 1 as libc::c_int as libc::c_uint) as libc::c_int
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
                        (*hi_cdf.offset(ctx as isize)).as_mut_ptr(),
                    ) as libc::c_int;
                    level_tok = tok + ((3 as libc::c_int) << 6 as libc::c_int);
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
                *cf.offset(rc as isize) = tok << 11 as libc::c_int;
                *levels
                    .offset(
                        x as isize * stride + y as isize
                    ) = level_tok as uint8_t;
                let mut i: libc::c_int = eob - 1 as libc::c_int;
                while i > 0 as libc::c_int {
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
                    if !(x < 32 as libc::c_int as libc::c_uint
                        && y < 32 as libc::c_int as libc::c_uint)
                    {
                        unreachable!();
                    }
                    let level: *mut uint8_t = levels
                        .offset(x as isize * stride)
                        .offset(y as isize);
                    ctx = get_lo_ctx(
                        level,
                        TX_CLASS_2D,
                        &mut mag,
                        lo_ctx_offsets,
                        x,
                        y,
                        stride,
                    );
                    if TX_CLASS_2D as libc::c_int == TX_CLASS_2D as libc::c_int {
                        y |= x;
                    }
                    tok = dav1d_msac_decode_symbol_adapt4(
                        &mut (*ts).msac,
                        (*lo_cdf.offset(ctx as isize)).as_mut_ptr(),
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
                    if tok == 3 as libc::c_int {
                        mag &= 63 as libc::c_int as libc::c_uint;
                        ctx = ((if y
                            > (TX_CLASS_2D as libc::c_int == TX_CLASS_2D as libc::c_int)
                                as libc::c_int as libc::c_uint
                        {
                            14 as libc::c_int
                        } else {
                            7 as libc::c_int
                        }) as libc::c_uint)
                            .wrapping_add(
                                if mag > 12 as libc::c_int as libc::c_uint {
                                    6 as libc::c_int as libc::c_uint
                                } else {
                                    mag.wrapping_add(1 as libc::c_int as libc::c_uint)
                                        >> 1 as libc::c_int
                                },
                            );
                        tok = dav1d_msac_decode_hi_tok(
                            &mut (*ts).msac,
                            (*hi_cdf.offset(ctx as isize)).as_mut_ptr(),
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
                        *level = (tok + ((3 as libc::c_int) << 6 as libc::c_int))
                            as uint8_t;
                        *cf
                            .offset(
                                rc_i as isize,
                            ) = ((tok << 11 as libc::c_int) as libc::c_uint | rc)
                            as coef;
                        rc = rc_i;
                    } else {
                        tok *= 0x17ff41 as libc::c_int;
                        *level = tok as uint8_t;
                        tok = ((tok >> 9 as libc::c_int) as libc::c_uint
                            & rc.wrapping_add(!(0x7ff as libc::c_uint))) as libc::c_int;
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
                        0 as libc::c_int as libc::c_uint,
                        0 as libc::c_int as libc::c_uint,
                        stride,
                    )
                };
                dc_tok = dav1d_msac_decode_symbol_adapt4(
                    &mut (*ts).msac,
                    (*lo_cdf.offset(ctx as isize)).as_mut_ptr(),
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
                if dc_tok == 3 as libc::c_int as libc::c_uint {
                    if TX_CLASS_2D as libc::c_int == TX_CLASS_2D as libc::c_int {
                        mag = (*levels
                            .offset(
                                (0 * stride
                                    + 1) as isize,
                            ) as libc::c_int
                            + *levels
                                .offset(
                                    (1 * stride
                                        + 0) as isize,
                                ) as libc::c_int
                            + *levels
                                .offset(
                                    (1 * stride
                                        + 1) as isize,
                                ) as libc::c_int) as libc::c_uint;
                    }
                    mag &= 63 as libc::c_int as libc::c_uint;
                    ctx = if mag > 12 as libc::c_int as libc::c_uint {
                        6 as libc::c_int as libc::c_uint
                    } else {
                        mag.wrapping_add(1 as libc::c_int as libc::c_uint)
                            >> 1 as libc::c_int
                    };
                    dc_tok = dav1d_msac_decode_hi_tok(
                        &mut (*ts).msac,
                        (*hi_cdf.offset(ctx as isize)).as_mut_ptr(),
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
                let lo_ctx_offsets_0: *const [uint8_t; 5] = 0 as *const [uint8_t; 5];
                let stride_0: ptrdiff_t = 16 as libc::c_int as ptrdiff_t;
                let shift_0: libc::c_uint = ((*t_dim).lh as libc::c_int
                    + 2 as libc::c_int) as libc::c_uint;
                let shift2_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                let mask_0: libc::c_uint = (4 as libc::c_int * sh - 1 as libc::c_int)
                    as libc::c_uint;
                memset(
                    levels as *mut libc::c_void,
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
                if eob_tok == 2 as libc::c_int {
                    ctx = (if if TX_CLASS_H as libc::c_int == TX_CLASS_2D as libc::c_int
                    {
                        (x_0 | y_0 > 1 as libc::c_int as libc::c_uint) as libc::c_int
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
                        (*hi_cdf.offset(ctx as isize)).as_mut_ptr(),
                    ) as libc::c_int;
                    level_tok = tok + ((3 as libc::c_int) << 6 as libc::c_int);
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
                *cf.offset(rc as isize) = tok << 11 as libc::c_int;
                *levels
                    .offset(
                        x_0 as isize * stride_0 + y_0 as isize,
                    ) = level_tok as uint8_t;
                let mut i_0: libc::c_int = eob - 1 as libc::c_int;
                while i_0 > 0 as libc::c_int {
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
                    if !(x_0 < 32 as libc::c_int as libc::c_uint
                        && y_0 < 32 as libc::c_int as libc::c_uint)
                    {
                        unreachable!();
                    }
                    let level_0: *mut uint8_t = levels
                        .offset(x_0 as isize * stride_0 as isize)
                        .offset(y_0 as isize);
                    ctx = get_lo_ctx(
                        level_0,
                        TX_CLASS_H,
                        &mut mag,
                        lo_ctx_offsets_0,
                        x_0,
                        y_0,
                        stride_0,
                    );
                    if TX_CLASS_H as libc::c_int == TX_CLASS_2D as libc::c_int {
                        y_0 |= x_0;
                    }
                    tok = dav1d_msac_decode_symbol_adapt4(
                        &mut (*ts).msac,
                        (*lo_cdf.offset(ctx as isize)).as_mut_ptr(),
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
                    if tok == 3 as libc::c_int {
                        mag &= 63 as libc::c_int as libc::c_uint;
                        ctx = ((if y_0
                            > (TX_CLASS_H as libc::c_int == TX_CLASS_2D as libc::c_int)
                                as libc::c_int as libc::c_uint
                        {
                            14 as libc::c_int
                        } else {
                            7 as libc::c_int
                        }) as libc::c_uint)
                            .wrapping_add(
                                if mag > 12 as libc::c_int as libc::c_uint {
                                    6 as libc::c_int as libc::c_uint
                                } else {
                                    mag.wrapping_add(1 as libc::c_int as libc::c_uint)
                                        >> 1 as libc::c_int
                                },
                            );
                        tok = dav1d_msac_decode_hi_tok(
                            &mut (*ts).msac,
                            (*hi_cdf.offset(ctx as isize)).as_mut_ptr(),
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
                        *level_0 = (tok + ((3 as libc::c_int) << 6 as libc::c_int))
                            as uint8_t;
                        *cf
                            .offset(
                                rc_i_0 as isize,
                            ) = ((tok << 11 as libc::c_int) as libc::c_uint | rc)
                            as coef;
                        rc = rc_i_0;
                    } else {
                        tok *= 0x17ff41 as libc::c_int;
                        *level_0 = tok as uint8_t;
                        tok = ((tok >> 9 as libc::c_int) as libc::c_uint
                            & rc.wrapping_add(!(0x7ff as libc::c_uint))) as libc::c_int;
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
                        0 as libc::c_int as libc::c_uint,
                        0 as libc::c_int as libc::c_uint,
                        stride_0,
                    )
                };
                dc_tok = dav1d_msac_decode_symbol_adapt4(
                    &mut (*ts).msac,
                    (*lo_cdf.offset(ctx as isize)).as_mut_ptr(),
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
                if dc_tok == 3 as libc::c_int as libc::c_uint {
                    if TX_CLASS_H as libc::c_int == TX_CLASS_2D as libc::c_int {
                        mag = (*levels
                            .offset(
                                (0 * stride_0
                                    + 1) as isize,
                            ) as libc::c_int
                            + *levels
                                .offset(
                                    (1 * stride_0
                                        + 0) as isize,
                                ) as libc::c_int
                            + *levels
                                .offset(
                                    (1 * stride_0
                                        + 1) as isize,
                                ) as libc::c_int) as libc::c_uint;
                    }
                    mag &= 63 as libc::c_int as libc::c_uint;
                    ctx = if mag > 12 as libc::c_int as libc::c_uint {
                        6 as libc::c_int as libc::c_uint
                    } else {
                        mag.wrapping_add(1 as libc::c_int as libc::c_uint)
                            >> 1 as libc::c_int
                    };
                    dc_tok = dav1d_msac_decode_hi_tok(
                        &mut (*ts).msac,
                        (*hi_cdf.offset(ctx as isize)).as_mut_ptr(),
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
                let lo_ctx_offsets_1: *const [uint8_t; 5] = 0 as *const [uint8_t; 5];
                let stride_1: ptrdiff_t = 16 as libc::c_int as ptrdiff_t;
                let shift_1: libc::c_uint = ((*t_dim).lw as libc::c_int
                    + 2 as libc::c_int) as libc::c_uint;
                let shift2_1: libc::c_uint = ((*t_dim).lh as libc::c_int
                    + 2 as libc::c_int) as libc::c_uint;
                let mask_1: libc::c_uint = (4 as libc::c_int * sw - 1 as libc::c_int)
                    as libc::c_uint;
                memset(
                    levels as *mut libc::c_void,
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
                if eob_tok == 2 as libc::c_int {
                    ctx = (if if TX_CLASS_V as libc::c_int == TX_CLASS_2D as libc::c_int
                    {
                        (x_1 | y_1 > 1 as libc::c_int as libc::c_uint) as libc::c_int
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
                        (*hi_cdf.offset(ctx as isize)).as_mut_ptr(),
                    ) as libc::c_int;
                    level_tok = tok + ((3 as libc::c_int) << 6 as libc::c_int);
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
                *cf.offset(rc as isize) = tok << 11 as libc::c_int;
                *levels
                    .offset(
                        (x_1 as isize * stride_1 + y_1 as isize) as isize,
                    ) = level_tok as uint8_t;
                let mut i_1: libc::c_int = eob - 1 as libc::c_int;
                while i_1 > 0 as libc::c_int {
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
                    if !(x_1 < 32 as libc::c_int as libc::c_uint
                        && y_1 < 32 as libc::c_int as libc::c_uint)
                    {
                        unreachable!();
                    }
                    let level_1: *mut uint8_t = levels
                        .offset(x_1 as isize * stride_1)
                        .offset(y_1 as isize);
                    ctx = get_lo_ctx(
                        level_1,
                        TX_CLASS_V,
                        &mut mag,
                        lo_ctx_offsets_1,
                        x_1,
                        y_1,
                        stride_1,
                    );
                    if TX_CLASS_V as libc::c_int == TX_CLASS_2D as libc::c_int {
                        y_1 |= x_1;
                    }
                    tok = dav1d_msac_decode_symbol_adapt4(
                        &mut (*ts).msac,
                        (*lo_cdf.offset(ctx as isize)).as_mut_ptr(),
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
                    if tok == 3 as libc::c_int {
                        mag &= 63 as libc::c_int as libc::c_uint;
                        ctx = ((if y_1
                            > (TX_CLASS_V as libc::c_int == TX_CLASS_2D as libc::c_int)
                                as libc::c_int as libc::c_uint
                        {
                            14 as libc::c_int
                        } else {
                            7 as libc::c_int
                        }) as libc::c_uint)
                            .wrapping_add(
                                if mag > 12 as libc::c_int as libc::c_uint {
                                    6 as libc::c_int as libc::c_uint
                                } else {
                                    mag.wrapping_add(1 as libc::c_int as libc::c_uint)
                                        >> 1 as libc::c_int
                                },
                            );
                        tok = dav1d_msac_decode_hi_tok(
                            &mut (*ts).msac,
                            (*hi_cdf.offset(ctx as isize)).as_mut_ptr(),
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
                        *level_1 = (tok + ((3 as libc::c_int) << 6 as libc::c_int))
                            as uint8_t;
                        *cf
                            .offset(
                                rc_i_1 as isize,
                            ) = ((tok << 11 as libc::c_int) as libc::c_uint | rc)
                            as coef;
                        rc = rc_i_1;
                    } else {
                        tok *= 0x17ff41 as libc::c_int;
                        *level_1 = tok as uint8_t;
                        tok = ((tok >> 9 as libc::c_int) as libc::c_uint
                            & rc.wrapping_add(!(0x7ff as libc::c_uint))) as libc::c_int;
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
                        0 as libc::c_int as libc::c_uint,
                        0 as libc::c_int as libc::c_uint,
                        stride_1,
                    )
                };
                dc_tok = dav1d_msac_decode_symbol_adapt4(
                    &mut (*ts).msac,
                    (*lo_cdf.offset(ctx as isize)).as_mut_ptr(),
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
                if dc_tok == 3 as libc::c_int as libc::c_uint {
                    if TX_CLASS_V as libc::c_int == TX_CLASS_2D as libc::c_int {
                        mag = (*levels
                            .offset(
                                (0 * stride_1
                                    + 1) as isize,
                            ) as libc::c_int
                            + *levels
                                .offset(
                                    (1 * stride_1
                                        + 0) as isize,
                                ) as libc::c_int
                            + *levels
                                .offset(
                                    (1 * stride_1
                                        + 1) as isize,
                                ) as libc::c_int) as libc::c_uint;
                    }
                    mag &= 63 as libc::c_int as libc::c_uint;
                    ctx = if mag > 12 as libc::c_int as libc::c_uint {
                        6 as libc::c_int as libc::c_uint
                    } else {
                        mag.wrapping_add(1 as libc::c_int as libc::c_uint)
                            >> 1 as libc::c_int
                    };
                    dc_tok = dav1d_msac_decode_hi_tok(
                        &mut (*ts).msac,
                        (*hi_cdf.offset(ctx as isize)).as_mut_ptr(),
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
                if 0 as libc::c_int == 0 {
                    unreachable!();
                }
            }
        }
    } else {
        let mut tok_br: libc::c_int = dav1d_msac_decode_symbol_adapt4(
            &mut (*ts).msac,
            (*eob_cdf.offset(0 as libc::c_int as isize)).as_mut_ptr(),
            2 as libc::c_int as size_t,
        ) as libc::c_int;
        dc_tok = (1 as libc::c_int + tok_br) as libc::c_uint;
        if dbg != 0 {
            printf(
                b"Post-dc_lo_tok[%d][%d][%d][%d]: r=%d\n\0" as *const u8
                    as *const libc::c_char,
                (*t_dim).ctx as libc::c_int,
                chroma,
                0 as libc::c_int,
                dc_tok,
                (*ts).msac.rng,
            );
        }
        if tok_br == 2 as libc::c_int {
            dc_tok = dav1d_msac_decode_hi_tok(
                &mut (*ts).msac,
                (*hi_cdf.offset(0 as libc::c_int as isize)).as_mut_ptr(),
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
        rc = 0 as libc::c_int as libc::c_uint;
    }
    let dq_tbl: *const uint16_t = ((*((*ts).dq)
        .offset((*b).seg_id as isize))[plane as usize])
        .as_ptr();
    let qm_tbl: *const uint8_t = if (*txtp as libc::c_uint)
        < IDTX as libc::c_int as libc::c_uint
    {
        (*f).qm[tx as usize][plane as usize]
    } else {
        0 as *const uint8_t
    };
    let dq_shift: libc::c_int = imax(
        0 as libc::c_int,
        (*t_dim).ctx as libc::c_int - 2 as libc::c_int,
    );
    let cf_max: libc::c_int = !(!(127 as libc::c_uint)
        << (if 16 as libc::c_int == 8 as libc::c_int {
            8 as libc::c_int
        } else {
            (*f).cur.p.bpc
        })) as libc::c_int;
    let mut cul_level: libc::c_uint = 0;
    let mut dc_sign_level: libc::c_uint = 0;
    if dc_tok == 0 {
        cul_level = 0 as libc::c_int as libc::c_uint;
        dc_sign_level = ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint;
        if !qm_tbl.is_null() {
            current_block = 1669574575799829731;
        } else {
            current_block = 2404388531445638768;
        }
    } else {
        dc_sign_ctx = get_dc_sign_ctx(tx as libc::c_int, a, l) as libc::c_int;
        dc_sign_cdf = ((*ts).cdf.coef.dc_sign[chroma as usize][dc_sign_ctx as usize])
            .as_mut_ptr();
        dc_sign = dav1d_msac_decode_bool_adapt(&mut (*ts).msac, dc_sign_cdf)
            as libc::c_int;
        if dbg != 0 {
            printf(
                b"Post-dc_sign[%d][%d][%d]: r=%d\n\0" as *const u8
                    as *const libc::c_char,
                chroma,
                dc_sign_ctx,
                dc_sign,
                (*ts).msac.rng,
            );
        }
        dc_dq = *dq_tbl.offset(0 as libc::c_int as isize) as libc::c_int;
        dc_sign_level = (dc_sign - 1 as libc::c_int
            & (2 as libc::c_int) << 6 as libc::c_int) as libc::c_uint;
        if !qm_tbl.is_null() {
            dc_dq = dc_dq * *qm_tbl.offset(0 as libc::c_int as isize) as libc::c_int
                + 16 as libc::c_int >> 5 as libc::c_int;
            if dc_tok == 15 as libc::c_int as libc::c_uint {
                dc_tok = (read_golomb(&mut (*ts).msac))
                    .wrapping_add(15 as libc::c_int as libc::c_uint);
                if dbg != 0 {
                    printf(
                        b"Post-dc_residual[%d->%d]: r=%d\n\0" as *const u8
                            as *const libc::c_char,
                        dc_tok.wrapping_sub(15 as libc::c_int as libc::c_uint),
                        dc_tok,
                        (*ts).msac.rng,
                    );
                }
                dc_tok &= 0xfffff as libc::c_int as libc::c_uint;
                dc_dq = ((dc_dq as libc::c_uint).wrapping_mul(dc_tok)
                    & 0xffffff as libc::c_int as libc::c_uint) as libc::c_int;
            } else {
                dc_dq = (dc_dq as libc::c_uint).wrapping_mul(dc_tok) as libc::c_int
                    as libc::c_int;
                if !(dc_dq <= 0xffffff as libc::c_int) {
                    unreachable!();
                }
            }
            cul_level = dc_tok;
            dc_dq >>= dq_shift;
            dc_dq = umin(dc_dq as libc::c_uint, (cf_max + dc_sign) as libc::c_uint)
                as libc::c_int;
            *cf
                .offset(
                    0 as libc::c_int as isize,
                ) = if dc_sign != 0 { -dc_dq } else { dc_dq };
            if rc != 0 {
                current_block = 1669574575799829731;
            } else {
                current_block = 15494703142406051947;
            }
        } else {
            if dc_tok == 15 as libc::c_int as libc::c_uint {
                dc_tok = (read_golomb(&mut (*ts).msac))
                    .wrapping_add(15 as libc::c_int as libc::c_uint);
                if dbg != 0 {
                    printf(
                        b"Post-dc_residual[%d->%d]: r=%d\n\0" as *const u8
                            as *const libc::c_char,
                        dc_tok.wrapping_sub(15 as libc::c_int as libc::c_uint),
                        dc_tok,
                        (*ts).msac.rng,
                    );
                }
                dc_tok &= 0xfffff as libc::c_int as libc::c_uint;
                dc_dq = (((dc_dq as libc::c_uint).wrapping_mul(dc_tok)
                    & 0xffffff as libc::c_int as libc::c_uint) >> dq_shift)
                    as libc::c_int;
                dc_dq = umin(dc_dq as libc::c_uint, (cf_max + dc_sign) as libc::c_uint)
                    as libc::c_int;
            } else {
                dc_dq = ((dc_dq as libc::c_uint).wrapping_mul(dc_tok) >> dq_shift)
                    as libc::c_int;
                if !(dc_dq <= cf_max) {
                    unreachable!();
                }
            }
            cul_level = dc_tok;
            *cf
                .offset(
                    0 as libc::c_int as isize,
                ) = if dc_sign != 0 { -dc_dq } else { dc_dq };
            if rc != 0 {
                current_block = 2404388531445638768;
            } else {
                current_block = 15494703142406051947;
            }
        }
    }
    match current_block {
        1669574575799829731 => {
            let ac_dq: libc::c_uint = *dq_tbl.offset(1 as libc::c_int as isize)
                as libc::c_uint;
            loop {
                let sign: libc::c_int = dav1d_msac_decode_bool_equi(&mut (*ts).msac)
                    as libc::c_int;
                if dbg != 0 {
                    printf(
                        b"Post-sign[%d=%d]: r=%d\n\0" as *const u8
                            as *const libc::c_char,
                        rc,
                        sign,
                        (*ts).msac.rng,
                    );
                }
                let rc_tok: libc::c_uint = *cf.offset(rc as isize) as libc::c_uint;
                let mut tok_0: libc::c_uint = 0;
                let mut dq: libc::c_uint = ac_dq
                    .wrapping_mul(*qm_tbl.offset(rc as isize) as libc::c_uint)
                    .wrapping_add(16 as libc::c_int as libc::c_uint) >> 5 as libc::c_int;
                let mut dq_sat: libc::c_int = 0;
                if rc_tok >= ((15 as libc::c_int) << 11 as libc::c_int) as libc::c_uint {
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
                    dq = dq.wrapping_mul(tok_0)
                        & 0xffffff as libc::c_int as libc::c_uint;
                } else {
                    tok_0 = rc_tok >> 11 as libc::c_int;
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
            let ac_dq_0: libc::c_uint = *dq_tbl.offset(1 as libc::c_int as isize)
                as libc::c_uint;
            loop {
                let sign_0: libc::c_int = dav1d_msac_decode_bool_equi(&mut (*ts).msac)
                    as libc::c_int;
                if dbg != 0 {
                    printf(
                        b"Post-sign[%d=%d]: r=%d\n\0" as *const u8
                            as *const libc::c_char,
                        rc,
                        sign_0,
                        (*ts).msac.rng,
                    );
                }
                let rc_tok_0: libc::c_uint = *cf.offset(rc as isize) as libc::c_uint;
                let mut tok_1: libc::c_uint = 0;
                let mut dq_0: libc::c_int = 0;
                if rc_tok_0 >= ((15 as libc::c_int) << 11 as libc::c_int) as libc::c_uint
                {
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
                    dq_0 = ((ac_dq_0.wrapping_mul(tok_1)
                        & 0xffffff as libc::c_int as libc::c_uint) >> dq_shift)
                        as libc::c_int;
                    dq_0 = umin(dq_0 as libc::c_uint, (cf_max + sign_0) as libc::c_uint)
                        as libc::c_int;
                } else {
                    tok_1 = rc_tok_0 >> 11 as libc::c_int;
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
    *res_ctx = (umin(cul_level, 63 as libc::c_int as libc::c_uint) | dc_sign_level)
        as uint8_t;
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
    let t_dim: *const TxfmInfo = &*dav1d_txfm_dimensions.as_ptr().offset(ytx as isize)
        as *const TxfmInfo;
    let txw: libc::c_int = (*t_dim).w as libc::c_int;
    let txh: libc::c_int = (*t_dim).h as libc::c_int;
    if depth < 2 as libc::c_int && *tx_split.offset(depth as isize) as libc::c_int != 0
        && *tx_split.offset(depth as isize) as libc::c_int
            & (1 as libc::c_int) << y_off * 4 as libc::c_int + x_off != 0
    {
        let sub: RectTxfmSize = (*t_dim).sub as RectTxfmSize;
        let sub_t_dim: *const TxfmInfo = &*dav1d_txfm_dimensions
            .as_ptr()
            .offset(sub as isize) as *const TxfmInfo;
        let txsw: libc::c_int = (*sub_t_dim).w as libc::c_int;
        let txsh: libc::c_int = (*sub_t_dim).h as libc::c_int;
        read_coef_tree(
            t,
            bs,
            b,
            sub,
            depth + 1 as libc::c_int,
            tx_split,
            x_off * 2 as libc::c_int + 0 as libc::c_int,
            y_off * 2 as libc::c_int + 0 as libc::c_int,
            dst,
        );
        (*t).bx += txsw;
        if txw >= txh && (*t).bx < (*f).bw {
            read_coef_tree(
                t,
                bs,
                b,
                sub,
                depth + 1 as libc::c_int,
                tx_split,
                x_off * 2 as libc::c_int + 1 as libc::c_int,
                y_off * 2 as libc::c_int + 0 as libc::c_int,
                if !dst.is_null() {
                    &mut *dst.offset((4 as libc::c_int * txsw) as isize)
                } else {
                    0 as *mut pixel
                },
            );
        }
        (*t).bx -= txsw;
        (*t).by += txsh;
        if txh >= txw && (*t).by < (*f).bh {
            if !dst.is_null() {
                dst = dst
                    .offset(
                        ((4 as libc::c_int * txsh) as isize
                            * PXSTRIDE((*f).cur.stride[0 as libc::c_int as usize]))
                            as isize,
                    );
            }
            read_coef_tree(
                t,
                bs,
                b,
                sub,
                depth + 1 as libc::c_int,
                tx_split,
                x_off * 2 as libc::c_int + 0 as libc::c_int,
                y_off * 2 as libc::c_int + 1 as libc::c_int,
                dst,
            );
            (*t).bx += txsw;
            if txw >= txh && (*t).bx < (*f).bw {
                read_coef_tree(
                    t,
                    bs,
                    b,
                    sub,
                    depth + 1 as libc::c_int,
                    tx_split,
                    x_off * 2 as libc::c_int + 1 as libc::c_int,
                    y_off * 2 as libc::c_int + 1 as libc::c_int,
                    if !dst.is_null() {
                        &mut *dst.offset((4 as libc::c_int * txsw) as isize)
                    } else {
                        0 as *mut pixel
                    },
                );
            }
            (*t).bx -= txsw;
        }
        (*t).by -= txsh;
    } else {
        let bx4: libc::c_int = (*t).bx & 31 as libc::c_int;
        let by4: libc::c_int = (*t).by & 31 as libc::c_int;
        let mut txtp: TxfmType = DCT_DCT;
        let mut cf_ctx: uint8_t = 0;
        let mut eob: libc::c_int = 0;
        let mut cf: *mut coef = 0 as *mut coef;
        let mut cbi: *mut CodedBlockInfo = 0 as *mut CodedBlockInfo;
        if (*t).frame_thread.pass != 0 {
            let p: libc::c_int = (*t).frame_thread.pass & 1 as libc::c_int;
            if ((*ts).frame_thread[p as usize].cf).is_null() {
                unreachable!();
            }
            cf = (*ts).frame_thread[p as usize].cf;
            (*ts)
                .frame_thread[p as usize]
                .cf = ((*ts).frame_thread[p as usize].cf)
                .offset(
                    (imin((*t_dim).w as libc::c_int, 8 as libc::c_int)
                        * imin((*t_dim).h as libc::c_int, 8 as libc::c_int)
                        * 16 as libc::c_int) as isize,
                );
            cbi = &mut *((*f).frame_thread.cbi)
                .offset(
                    ((*t).by as isize * (*f).b4_stride + (*t).bx as isize)
                        as isize,
                ) as *mut CodedBlockInfo;
        } else {
            cf = ((*t).c2rust_unnamed.cf_16bpc).as_mut_ptr();
        }
        if (*t).frame_thread.pass != 2 as libc::c_int {
            eob = decode_coefs(
                t,
                &mut *((*(*t).a).lcoef).as_mut_ptr().offset(bx4 as isize),
                &mut *((*t).l.lcoef).as_mut_ptr().offset(by4 as isize),
                ytx,
                bs,
                b,
                0 as libc::c_int,
                0 as libc::c_int,
                cf,
                &mut txtp,
                &mut cf_ctx,
            );
            if 0 as libc::c_int != 0
                && (*(*f).frame_hdr).frame_offset == 2 as libc::c_int
                && (*t).by >= 0 as libc::c_int && (*t).by < 4 as libc::c_int
                && (*t).bx >= 8 as libc::c_int && (*t).bx < 12 as libc::c_int
            {
                printf(
                    b"Post-y-cf-blk[tx=%d,txtp=%d,eob=%d]: r=%d\n\0" as *const u8
                        as *const libc::c_char,
                    ytx as libc::c_uint,
                    txtp as libc::c_uint,
                    eob,
                    (*ts).msac.rng,
                );
            }
            match imin(txh, (*f).bh - (*t).by) {
                1 => {
                    (*(&mut *((*t).l.lcoef).as_mut_ptr().offset(by4 as isize)
                        as *mut uint8_t as *mut alias8))
                        .u8_0 = (0x1 as libc::c_int * cf_ctx as libc::c_int) as uint8_t;
                }
                2 => {
                    (*(&mut *((*t).l.lcoef).as_mut_ptr().offset(by4 as isize)
                        as *mut uint8_t as *mut alias16))
                        .u16_0 = (0x101 as libc::c_int * cf_ctx as libc::c_int)
                        as uint16_t;
                }
                4 => {
                    (*(&mut *((*t).l.lcoef).as_mut_ptr().offset(by4 as isize)
                        as *mut uint8_t as *mut alias32))
                        .u32_0 = (0x1010101 as libc::c_uint)
                        .wrapping_mul(cf_ctx as libc::c_uint);
                }
                8 => {
                    (*(&mut *((*t).l.lcoef).as_mut_ptr().offset(by4 as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(cf_ctx as libc::c_ulonglong) as uint64_t;
                }
                16 => {
                    let const_val: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(cf_ctx as libc::c_ulonglong) as uint64_t;
                    (*(&mut *((*t).l.lcoef)
                        .as_mut_ptr()
                        .offset((by4 + 0 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val;
                    (*(&mut *((*t).l.lcoef)
                        .as_mut_ptr()
                        .offset((by4 + 8 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val;
                }
                _ => {
                    memset(
                        &mut *((*t).l.lcoef).as_mut_ptr().offset(by4 as isize)
                            as *mut uint8_t as *mut libc::c_void,
                        cf_ctx as libc::c_int,
                        imin(txh, (*f).bh - (*t).by) as size_t,
                    );
                }
            }
            match imin(txw, (*f).bw - (*t).bx) {
                1 => {
                    (*(&mut *((*(*t).a).lcoef).as_mut_ptr().offset(bx4 as isize)
                        as *mut uint8_t as *mut alias8))
                        .u8_0 = (0x1 as libc::c_int * cf_ctx as libc::c_int) as uint8_t;
                }
                2 => {
                    (*(&mut *((*(*t).a).lcoef).as_mut_ptr().offset(bx4 as isize)
                        as *mut uint8_t as *mut alias16))
                        .u16_0 = (0x101 as libc::c_int * cf_ctx as libc::c_int)
                        as uint16_t;
                }
                4 => {
                    (*(&mut *((*(*t).a).lcoef).as_mut_ptr().offset(bx4 as isize)
                        as *mut uint8_t as *mut alias32))
                        .u32_0 = (0x1010101 as libc::c_uint)
                        .wrapping_mul(cf_ctx as libc::c_uint);
                }
                8 => {
                    (*(&mut *((*(*t).a).lcoef).as_mut_ptr().offset(bx4 as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(cf_ctx as libc::c_ulonglong) as uint64_t;
                }
                16 => {
                    let const_val_0: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(cf_ctx as libc::c_ulonglong) as uint64_t;
                    (*(&mut *((*(*t).a).lcoef)
                        .as_mut_ptr()
                        .offset((bx4 + 0 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_0;
                    (*(&mut *((*(*t).a).lcoef)
                        .as_mut_ptr()
                        .offset((bx4 + 8 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_0;
                }
                _ => {
                    memset(
                        &mut *((*(*t).a).lcoef).as_mut_ptr().offset(bx4 as isize)
                            as *mut uint8_t as *mut libc::c_void,
                        cf_ctx as libc::c_int,
                        imin(txw, (*f).bw - (*t).bx) as size_t,
                    );
                }
            }
            let mut txtp_map: *mut uint8_t = &mut *((*t).txtp_map)
                .as_mut_ptr()
                .offset((by4 * 32 as libc::c_int + bx4) as isize) as *mut uint8_t;
            match txw {
                1 => {
                    let mut y: libc::c_int = 0 as libc::c_int;
                    while y < txh {
                        (*(&mut *txtp_map.offset(0 as libc::c_int as isize)
                            as *mut uint8_t as *mut alias8))
                            .u8_0 = (0x1 as libc::c_int as libc::c_uint)
                            .wrapping_mul(txtp as libc::c_uint) as uint8_t;
                        txtp_map = txtp_map.offset(32 as libc::c_int as isize);
                        y += 1;
                    }
                }
                2 => {
                    let mut y_0: libc::c_int = 0 as libc::c_int;
                    while y_0 < txh {
                        (*(&mut *txtp_map.offset(0 as libc::c_int as isize)
                            as *mut uint8_t as *mut alias16))
                            .u16_0 = (0x101 as libc::c_int as libc::c_uint)
                            .wrapping_mul(txtp as libc::c_uint) as uint16_t;
                        txtp_map = txtp_map.offset(32 as libc::c_int as isize);
                        y_0 += 1;
                    }
                }
                4 => {
                    let mut y_1: libc::c_int = 0 as libc::c_int;
                    while y_1 < txh {
                        (*(&mut *txtp_map.offset(0 as libc::c_int as isize)
                            as *mut uint8_t as *mut alias32))
                            .u32_0 = (0x1010101 as libc::c_uint)
                            .wrapping_mul(txtp as libc::c_uint);
                        txtp_map = txtp_map.offset(32 as libc::c_int as isize);
                        y_1 += 1;
                    }
                }
                8 => {
                    let mut y_2: libc::c_int = 0 as libc::c_int;
                    while y_2 < txh {
                        (*(&mut *txtp_map.offset(0 as libc::c_int as isize)
                            as *mut uint8_t as *mut alias64))
                            .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                            .wrapping_mul(txtp as libc::c_ulonglong) as uint64_t;
                        txtp_map = txtp_map.offset(32 as libc::c_int as isize);
                        y_2 += 1;
                    }
                }
                16 => {
                    let mut y_3: libc::c_int = 0 as libc::c_int;
                    while y_3 < txh {
                        let const_val_1: uint64_t = (0x101010101010101
                            as libc::c_ulonglong)
                            .wrapping_mul(txtp as libc::c_ulonglong) as uint64_t;
                        (*(&mut *txtp_map
                            .offset((0 as libc::c_int + 0 as libc::c_int) as isize)
                            as *mut uint8_t as *mut alias64))
                            .u64_0 = const_val_1;
                        (*(&mut *txtp_map
                            .offset((0 as libc::c_int + 8 as libc::c_int) as isize)
                            as *mut uint8_t as *mut alias64))
                            .u64_0 = const_val_1;
                        txtp_map = txtp_map.offset(32 as libc::c_int as isize);
                        y_3 += 1;
                    }
                }
                _ => {}
            }
            if (*t).frame_thread.pass == 1 as libc::c_int {
                (*cbi).eob[0 as libc::c_int as usize] = eob as int16_t;
                (*cbi).txtp[0 as libc::c_int as usize] = txtp as uint8_t;
            }
        } else {
            eob = (*cbi).eob[0 as libc::c_int as usize] as libc::c_int;
            txtp = (*cbi).txtp[0 as libc::c_int as usize] as TxfmType;
        }
        if (*t).frame_thread.pass & 1 as libc::c_int == 0 {
            if dst.is_null() {
                unreachable!();
            }
            if eob >= 0 as libc::c_int {
                if 0 as libc::c_int != 0
                    && (*(*f).frame_hdr).frame_offset == 2 as libc::c_int
                    && (*t).by >= 0 as libc::c_int && (*t).by < 4 as libc::c_int
                    && (*t).bx >= 8 as libc::c_int && (*t).bx < 12 as libc::c_int
                    && 0 as libc::c_int != 0
                {
                    coef_dump(
                        cf,
                        imin((*t_dim).h as libc::c_int, 8 as libc::c_int)
                            * 4 as libc::c_int,
                        imin((*t_dim).w as libc::c_int, 8 as libc::c_int)
                            * 4 as libc::c_int,
                        3 as libc::c_int,
                        b"dq\0" as *const u8 as *const libc::c_char,
                    );
                }
                ((*dsp).itx.itxfm_add[ytx as usize][txtp as usize])
                    .expect(
                        "non-null function pointer",
                    )(
                    dst,
                    (*f).cur.stride[0 as libc::c_int as usize],
                    cf,
                    eob,
                    (*f).bitdepth_max,
                );
                if 0 as libc::c_int != 0
                    && (*(*f).frame_hdr).frame_offset == 2 as libc::c_int
                    && (*t).by >= 0 as libc::c_int && (*t).by < 4 as libc::c_int
                    && (*t).bx >= 8 as libc::c_int && (*t).bx < 12 as libc::c_int
                    && 0 as libc::c_int != 0
                {
                    hex_dump(
                        dst,
                        (*f).cur.stride[0 as libc::c_int as usize],
                        (*t_dim).w as libc::c_int * 4 as libc::c_int,
                        (*t_dim).h as libc::c_int * 4 as libc::c_int,
                        b"recon\0" as *const u8 as *const libc::c_char,
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
    let ss_ver: libc::c_int = ((*f).cur.p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_hor: libc::c_int = ((*f).cur.p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
    let bx4: libc::c_int = (*t).bx & 31 as libc::c_int;
    let by4: libc::c_int = (*t).by & 31 as libc::c_int;
    let cbx4: libc::c_int = bx4 >> ss_hor;
    let cby4: libc::c_int = by4 >> ss_ver;
    let b_dim: *const uint8_t = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bw4: libc::c_int = *b_dim.offset(0 as libc::c_int as isize) as libc::c_int;
    let bh4: libc::c_int = *b_dim.offset(1 as libc::c_int as isize) as libc::c_int;
    let cbw4: libc::c_int = bw4 + ss_hor >> ss_hor;
    let cbh4: libc::c_int = bh4 + ss_ver >> ss_ver;
    let has_chroma: libc::c_int = ((*f).cur.p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint
        && (bw4 > ss_hor || (*t).bx & 1 as libc::c_int != 0)
        && (bh4 > ss_ver || (*t).by & 1 as libc::c_int != 0)) as libc::c_int;
    if (*b).skip != 0 {
        match bh4 {
            1 => {
                (*(&mut *((*t).l.lcoef).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias8))
                    .u8_0 = (0x1 as libc::c_int * 0x40 as libc::c_int) as uint8_t;
            }
            2 => {
                (*(&mut *((*t).l.lcoef).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias16))
                    .u16_0 = (0x101 as libc::c_int * 0x40 as libc::c_int) as uint16_t;
            }
            4 => {
                (*(&mut *((*t).l.lcoef).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint)
                    .wrapping_mul(0x40 as libc::c_int as libc::c_uint);
            }
            8 => {
                (*(&mut *((*t).l.lcoef).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong) as uint64_t;
            }
            16 => {
                let const_val: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong) as uint64_t;
                (*(&mut *((*t).l.lcoef)
                    .as_mut_ptr()
                    .offset((by4 + 0 as libc::c_int) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val;
                (*(&mut *((*t).l.lcoef)
                    .as_mut_ptr()
                    .offset((by4 + 8 as libc::c_int) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val;
            }
            32 => {
                let const_val_0: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong) as uint64_t;
                (*(&mut *((*t).l.lcoef)
                    .as_mut_ptr()
                    .offset((by4 + 0 as libc::c_int) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_0;
                (*(&mut *((*t).l.lcoef)
                    .as_mut_ptr()
                    .offset((by4 + 8 as libc::c_int) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_0;
                (*(&mut *((*t).l.lcoef)
                    .as_mut_ptr()
                    .offset((by4 + 16 as libc::c_int) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_0;
                (*(&mut *((*t).l.lcoef)
                    .as_mut_ptr()
                    .offset((by4 + 24 as libc::c_int) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_0;
            }
            _ => {}
        }
        match bw4 {
            1 => {
                (*(&mut *((*(*t).a).lcoef).as_mut_ptr().offset(bx4 as isize)
                    as *mut uint8_t as *mut alias8))
                    .u8_0 = (0x1 as libc::c_int * 0x40 as libc::c_int) as uint8_t;
            }
            2 => {
                (*(&mut *((*(*t).a).lcoef).as_mut_ptr().offset(bx4 as isize)
                    as *mut uint8_t as *mut alias16))
                    .u16_0 = (0x101 as libc::c_int * 0x40 as libc::c_int) as uint16_t;
            }
            4 => {
                (*(&mut *((*(*t).a).lcoef).as_mut_ptr().offset(bx4 as isize)
                    as *mut uint8_t as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint)
                    .wrapping_mul(0x40 as libc::c_int as libc::c_uint);
            }
            8 => {
                (*(&mut *((*(*t).a).lcoef).as_mut_ptr().offset(bx4 as isize)
                    as *mut uint8_t as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong) as uint64_t;
            }
            16 => {
                let const_val_1: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong) as uint64_t;
                (*(&mut *((*(*t).a).lcoef)
                    .as_mut_ptr()
                    .offset((bx4 + 0 as libc::c_int) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_1;
                (*(&mut *((*(*t).a).lcoef)
                    .as_mut_ptr()
                    .offset((bx4 + 8 as libc::c_int) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_1;
            }
            32 => {
                let const_val_2: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong) as uint64_t;
                (*(&mut *((*(*t).a).lcoef)
                    .as_mut_ptr()
                    .offset((bx4 + 0 as libc::c_int) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_2;
                (*(&mut *((*(*t).a).lcoef)
                    .as_mut_ptr()
                    .offset((bx4 + 8 as libc::c_int) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_2;
                (*(&mut *((*(*t).a).lcoef)
                    .as_mut_ptr()
                    .offset((bx4 + 16 as libc::c_int) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_2;
                (*(&mut *((*(*t).a).lcoef)
                    .as_mut_ptr()
                    .offset((bx4 + 24 as libc::c_int) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_2;
            }
            _ => {}
        }
        if has_chroma != 0 {
            match cbh4 {
                1 => {
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cby4 as isize) as *mut uint8_t as *mut alias8))
                        .u8_0 = (0x1 as libc::c_int * 0x40 as libc::c_int) as uint8_t;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cby4 as isize) as *mut uint8_t as *mut alias8))
                        .u8_0 = (0x1 as libc::c_int * 0x40 as libc::c_int) as uint8_t;
                }
                2 => {
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cby4 as isize) as *mut uint8_t as *mut alias16))
                        .u16_0 = (0x101 as libc::c_int * 0x40 as libc::c_int)
                        as uint16_t;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cby4 as isize) as *mut uint8_t as *mut alias16))
                        .u16_0 = (0x101 as libc::c_int * 0x40 as libc::c_int)
                        as uint16_t;
                }
                4 => {
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cby4 as isize) as *mut uint8_t as *mut alias32))
                        .u32_0 = (0x1010101 as libc::c_uint)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_uint);
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cby4 as isize) as *mut uint8_t as *mut alias32))
                        .u32_0 = (0x1010101 as libc::c_uint)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_uint);
                }
                8 => {
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cby4 as isize) as *mut uint8_t as *mut alias64))
                        .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cby4 as isize) as *mut uint8_t as *mut alias64))
                        .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                }
                16 => {
                    let const_val_3: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cby4 + 0 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_3;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cby4 + 8 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_3;
                    let const_val_4: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cby4 + 0 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_4;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cby4 + 8 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_4;
                }
                32 => {
                    let const_val_5: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cby4 + 0 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_5;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cby4 + 8 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_5;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cby4 + 16 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_5;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cby4 + 24 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_5;
                    let const_val_6: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cby4 + 0 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_6;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cby4 + 8 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_6;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cby4 + 16 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_6;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cby4 + 24 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_6;
                }
                _ => {}
            }
            match cbw4 {
                1 => {
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cbx4 as isize) as *mut uint8_t as *mut alias8))
                        .u8_0 = (0x1 as libc::c_int * 0x40 as libc::c_int) as uint8_t;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cbx4 as isize) as *mut uint8_t as *mut alias8))
                        .u8_0 = (0x1 as libc::c_int * 0x40 as libc::c_int) as uint8_t;
                }
                2 => {
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cbx4 as isize) as *mut uint8_t as *mut alias16))
                        .u16_0 = (0x101 as libc::c_int * 0x40 as libc::c_int)
                        as uint16_t;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cbx4 as isize) as *mut uint8_t as *mut alias16))
                        .u16_0 = (0x101 as libc::c_int * 0x40 as libc::c_int)
                        as uint16_t;
                }
                4 => {
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cbx4 as isize) as *mut uint8_t as *mut alias32))
                        .u32_0 = (0x1010101 as libc::c_uint)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_uint);
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cbx4 as isize) as *mut uint8_t as *mut alias32))
                        .u32_0 = (0x1010101 as libc::c_uint)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_uint);
                }
                8 => {
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cbx4 as isize) as *mut uint8_t as *mut alias64))
                        .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cbx4 as isize) as *mut uint8_t as *mut alias64))
                        .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                }
                16 => {
                    let const_val_7: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cbx4 + 0 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_7;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cbx4 + 8 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_7;
                    let const_val_8: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cbx4 + 0 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_8;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cbx4 + 8 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_8;
                }
                32 => {
                    let const_val_9: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cbx4 + 0 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_9;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cbx4 + 8 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_9;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cbx4 + 16 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_9;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cbx4 + 24 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_9;
                    let const_val_10: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cbx4 + 0 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_10;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cbx4 + 8 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_10;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cbx4 + 16 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_10;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cbx4 + 24 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_10;
                }
                _ => {}
            }
        }
        return;
    }
    let ts: *mut Dav1dTileState = (*t).ts;
    let w4: libc::c_int = imin(bw4, (*f).bw - (*t).bx);
    let h4: libc::c_int = imin(bh4, (*f).bh - (*t).by);
    let cw4: libc::c_int = w4 + ss_hor >> ss_hor;
    let ch4: libc::c_int = h4 + ss_ver >> ss_ver;
    if !((*t).frame_thread.pass == 1 as libc::c_int) {
        unreachable!();
    }
    if (*b).skip != 0 {
        unreachable!();
    }
    let uv_t_dim: *const TxfmInfo = &*dav1d_txfm_dimensions
        .as_ptr()
        .offset((*b).uvtx as isize) as *const TxfmInfo;
    let t_dim: *const TxfmInfo = &*dav1d_txfm_dimensions
        .as_ptr()
        .offset(
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
    let mut init_y: libc::c_int = 0 as libc::c_int;
    while init_y < h4 {
        let sub_h4: libc::c_int = imin(h4, 16 as libc::c_int + init_y);
        let mut init_x: libc::c_int = 0 as libc::c_int;
        while init_x < w4 {
            let sub_w4: libc::c_int = imin(w4, init_x + 16 as libc::c_int);
            let mut y_off: libc::c_int = (init_y != 0) as libc::c_int;
            let mut y: libc::c_int = 0;
            let mut x: libc::c_int = 0;
            y = init_y;
            (*t).by += init_y;
            while y < sub_h4 {
                let cbi: *mut CodedBlockInfo = &mut *((*f).frame_thread.cbi)
                    .offset(((*t).by as isize * (*f).b4_stride) as isize)
                    as *mut CodedBlockInfo;
                let mut x_off: libc::c_int = (init_x != 0) as libc::c_int;
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
                        let ref mut fresh4 = (*cbi.offset((*t).bx as isize))
                            .eob[0 as libc::c_int as usize];
                        *fresh4 = decode_coefs(
                            t,
                            &mut *((*(*t).a).lcoef)
                                .as_mut_ptr()
                                .offset((bx4 + x) as isize),
                            &mut *((*t).l.lcoef).as_mut_ptr().offset((by4 + y) as isize),
                            (*b).c2rust_unnamed.c2rust_unnamed.tx as RectTxfmSize,
                            bs,
                            b,
                            1 as libc::c_int,
                            0 as libc::c_int,
                            (*ts).frame_thread[1 as libc::c_int as usize].cf,
                            &mut txtp,
                            &mut cf_ctx,
                        ) as int16_t;
                        let eob: libc::c_int = *fresh4 as libc::c_int;
                        if 0 as libc::c_int != 0
                            && (*(*f).frame_hdr).frame_offset == 2 as libc::c_int
                            && (*t).by >= 0 as libc::c_int && (*t).by < 4 as libc::c_int
                            && (*t).bx >= 8 as libc::c_int && (*t).bx < 12 as libc::c_int
                        {
                            printf(
                                b"Post-y-cf-blk[tx=%d,txtp=%d,eob=%d]: r=%d\n\0"
                                    as *const u8 as *const libc::c_char,
                                (*b).c2rust_unnamed.c2rust_unnamed.tx as libc::c_int,
                                txtp as libc::c_uint,
                                eob,
                                (*ts).msac.rng,
                            );
                        }
                        (*cbi.offset((*t).bx as isize))
                            .txtp[0 as libc::c_int as usize] = txtp as uint8_t;
                        (*ts)
                            .frame_thread[1 as libc::c_int as usize]
                            .cf = ((*ts).frame_thread[1 as libc::c_int as usize].cf)
                            .offset(
                                (imin((*t_dim).w as libc::c_int, 8 as libc::c_int)
                                    * imin((*t_dim).h as libc::c_int, 8 as libc::c_int)
                                    * 16 as libc::c_int) as isize,
                            );
                        match imin((*t_dim).h as libc::c_int, (*f).bh - (*t).by) {
                            1 => {
                                (*(&mut *((*t).l.lcoef)
                                    .as_mut_ptr()
                                    .offset((by4 + y) as isize) as *mut uint8_t as *mut alias8))
                                    .u8_0 = (0x1 as libc::c_int * cf_ctx as libc::c_int)
                                    as uint8_t;
                            }
                            2 => {
                                (*(&mut *((*t).l.lcoef)
                                    .as_mut_ptr()
                                    .offset((by4 + y) as isize) as *mut uint8_t
                                    as *mut alias16))
                                    .u16_0 = (0x101 as libc::c_int * cf_ctx as libc::c_int)
                                    as uint16_t;
                            }
                            4 => {
                                (*(&mut *((*t).l.lcoef)
                                    .as_mut_ptr()
                                    .offset((by4 + y) as isize) as *mut uint8_t
                                    as *mut alias32))
                                    .u32_0 = (0x1010101 as libc::c_uint)
                                    .wrapping_mul(cf_ctx as libc::c_uint);
                            }
                            8 => {
                                (*(&mut *((*t).l.lcoef)
                                    .as_mut_ptr()
                                    .offset((by4 + y) as isize) as *mut uint8_t
                                    as *mut alias64))
                                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                                    .wrapping_mul(cf_ctx as libc::c_ulonglong) as uint64_t;
                            }
                            16 => {
                                let const_val_11: uint64_t = (0x101010101010101
                                    as libc::c_ulonglong)
                                    .wrapping_mul(cf_ctx as libc::c_ulonglong) as uint64_t;
                                (*(&mut *((*t).l.lcoef)
                                    .as_mut_ptr()
                                    .offset((by4 + y + 0 as libc::c_int) as isize)
                                    as *mut uint8_t as *mut alias64))
                                    .u64_0 = const_val_11;
                                (*(&mut *((*t).l.lcoef)
                                    .as_mut_ptr()
                                    .offset((by4 + y + 8 as libc::c_int) as isize)
                                    as *mut uint8_t as *mut alias64))
                                    .u64_0 = const_val_11;
                            }
                            _ => {
                                memset(
                                    &mut *((*t).l.lcoef).as_mut_ptr().offset((by4 + y) as isize)
                                        as *mut uint8_t as *mut libc::c_void,
                                    cf_ctx as libc::c_int,
                                    imin((*t_dim).h as libc::c_int, (*f).bh - (*t).by)
                                        as size_t,
                                );
                            }
                        }
                        match imin((*t_dim).w as libc::c_int, (*f).bw - (*t).bx) {
                            1 => {
                                (*(&mut *((*(*t).a).lcoef)
                                    .as_mut_ptr()
                                    .offset((bx4 + x) as isize) as *mut uint8_t as *mut alias8))
                                    .u8_0 = (0x1 as libc::c_int * cf_ctx as libc::c_int)
                                    as uint8_t;
                            }
                            2 => {
                                (*(&mut *((*(*t).a).lcoef)
                                    .as_mut_ptr()
                                    .offset((bx4 + x) as isize) as *mut uint8_t
                                    as *mut alias16))
                                    .u16_0 = (0x101 as libc::c_int * cf_ctx as libc::c_int)
                                    as uint16_t;
                            }
                            4 => {
                                (*(&mut *((*(*t).a).lcoef)
                                    .as_mut_ptr()
                                    .offset((bx4 + x) as isize) as *mut uint8_t
                                    as *mut alias32))
                                    .u32_0 = (0x1010101 as libc::c_uint)
                                    .wrapping_mul(cf_ctx as libc::c_uint);
                            }
                            8 => {
                                (*(&mut *((*(*t).a).lcoef)
                                    .as_mut_ptr()
                                    .offset((bx4 + x) as isize) as *mut uint8_t
                                    as *mut alias64))
                                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                                    .wrapping_mul(cf_ctx as libc::c_ulonglong) as uint64_t;
                            }
                            16 => {
                                let const_val_12: uint64_t = (0x101010101010101
                                    as libc::c_ulonglong)
                                    .wrapping_mul(cf_ctx as libc::c_ulonglong) as uint64_t;
                                (*(&mut *((*(*t).a).lcoef)
                                    .as_mut_ptr()
                                    .offset((bx4 + x + 0 as libc::c_int) as isize)
                                    as *mut uint8_t as *mut alias64))
                                    .u64_0 = const_val_12;
                                (*(&mut *((*(*t).a).lcoef)
                                    .as_mut_ptr()
                                    .offset((bx4 + x + 8 as libc::c_int) as isize)
                                    as *mut uint8_t as *mut alias64))
                                    .u64_0 = const_val_12;
                            }
                            _ => {
                                memset(
                                    &mut *((*(*t).a).lcoef)
                                        .as_mut_ptr()
                                        .offset((bx4 + x) as isize) as *mut uint8_t
                                        as *mut libc::c_void,
                                    cf_ctx as libc::c_int,
                                    imin((*t_dim).w as libc::c_int, (*f).bw - (*t).bx)
                                        as size_t,
                                );
                            }
                        }
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
                let sub_ch4: libc::c_int = imin(
                    ch4,
                    init_y + 16 as libc::c_int >> ss_ver,
                );
                let sub_cw4: libc::c_int = imin(
                    cw4,
                    init_x + 16 as libc::c_int >> ss_hor,
                );
                let mut pl: libc::c_int = 0 as libc::c_int;
                while pl < 2 as libc::c_int {
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
                                txtp_0 = (*t)
                                    .txtp_map[((by4 + (y << ss_ver)) * 32 as libc::c_int + bx4
                                    + (x << ss_hor)) as usize] as TxfmType;
                            }
                            let ref mut fresh5 = (*cbi_0.offset((*t).bx as isize))
                                .eob[(1 as libc::c_int + pl) as usize];
                            *fresh5 = decode_coefs(
                                t,
                                &mut *(*((*(*t).a).ccoef).as_mut_ptr().offset(pl as isize))
                                    .as_mut_ptr()
                                    .offset((cbx4 + x) as isize),
                                &mut *(*((*t).l.ccoef).as_mut_ptr().offset(pl as isize))
                                    .as_mut_ptr()
                                    .offset((cby4 + y) as isize),
                                (*b).uvtx as RectTxfmSize,
                                bs,
                                b,
                                (*b).intra as libc::c_int,
                                1 as libc::c_int + pl,
                                (*ts).frame_thread[1 as libc::c_int as usize].cf,
                                &mut txtp_0,
                                &mut cf_ctx_0,
                            ) as int16_t;
                            let eob_0: libc::c_int = *fresh5 as libc::c_int;
                            if 0 as libc::c_int != 0
                                && (*(*f).frame_hdr).frame_offset == 2 as libc::c_int
                                && (*t).by >= 0 as libc::c_int && (*t).by < 4 as libc::c_int
                                && (*t).bx >= 8 as libc::c_int
                                && (*t).bx < 12 as libc::c_int
                            {
                                printf(
                                    b"Post-uv-cf-blk[pl=%d,tx=%d,txtp=%d,eob=%d]: r=%d\n\0"
                                        as *const u8 as *const libc::c_char,
                                    pl,
                                    (*b).uvtx as libc::c_int,
                                    txtp_0 as libc::c_uint,
                                    eob_0,
                                    (*ts).msac.rng,
                                );
                            }
                            (*cbi_0.offset((*t).bx as isize))
                                .txtp[(1 as libc::c_int + pl) as usize] = txtp_0 as uint8_t;
                            (*ts)
                                .frame_thread[1 as libc::c_int as usize]
                                .cf = ((*ts).frame_thread[1 as libc::c_int as usize].cf)
                                .offset(
                                    ((*uv_t_dim).w as libc::c_int * (*uv_t_dim).h as libc::c_int
                                        * 16 as libc::c_int) as isize,
                                );
                            match imin(
                                (*uv_t_dim).h as libc::c_int,
                                (*f).bh - (*t).by + ss_ver >> ss_ver,
                            ) {
                                1 => {
                                    (*(&mut *(*((*t).l.ccoef).as_mut_ptr().offset(pl as isize))
                                        .as_mut_ptr()
                                        .offset((cby4 + y) as isize) as *mut uint8_t
                                        as *mut alias8))
                                        .u8_0 = (0x1 as libc::c_int * cf_ctx_0 as libc::c_int)
                                        as uint8_t;
                                }
                                2 => {
                                    (*(&mut *(*((*t).l.ccoef).as_mut_ptr().offset(pl as isize))
                                        .as_mut_ptr()
                                        .offset((cby4 + y) as isize) as *mut uint8_t
                                        as *mut alias16))
                                        .u16_0 = (0x101 as libc::c_int * cf_ctx_0 as libc::c_int)
                                        as uint16_t;
                                }
                                4 => {
                                    (*(&mut *(*((*t).l.ccoef).as_mut_ptr().offset(pl as isize))
                                        .as_mut_ptr()
                                        .offset((cby4 + y) as isize) as *mut uint8_t
                                        as *mut alias32))
                                        .u32_0 = (0x1010101 as libc::c_uint)
                                        .wrapping_mul(cf_ctx_0 as libc::c_uint);
                                }
                                8 => {
                                    (*(&mut *(*((*t).l.ccoef).as_mut_ptr().offset(pl as isize))
                                        .as_mut_ptr()
                                        .offset((cby4 + y) as isize) as *mut uint8_t
                                        as *mut alias64))
                                        .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                                        .wrapping_mul(cf_ctx_0 as libc::c_ulonglong) as uint64_t;
                                }
                                16 => {
                                    let const_val_13: uint64_t = (0x101010101010101
                                        as libc::c_ulonglong)
                                        .wrapping_mul(cf_ctx_0 as libc::c_ulonglong) as uint64_t;
                                    (*(&mut *(*((*t).l.ccoef).as_mut_ptr().offset(pl as isize))
                                        .as_mut_ptr()
                                        .offset((cby4 + y + 0 as libc::c_int) as isize)
                                        as *mut uint8_t as *mut alias64))
                                        .u64_0 = const_val_13;
                                    (*(&mut *(*((*t).l.ccoef).as_mut_ptr().offset(pl as isize))
                                        .as_mut_ptr()
                                        .offset((cby4 + y + 8 as libc::c_int) as isize)
                                        as *mut uint8_t as *mut alias64))
                                        .u64_0 = const_val_13;
                                }
                                _ => {
                                    memset(
                                        &mut *(*((*t).l.ccoef).as_mut_ptr().offset(pl as isize))
                                            .as_mut_ptr()
                                            .offset((cby4 + y) as isize) as *mut uint8_t
                                            as *mut libc::c_void,
                                        cf_ctx_0 as libc::c_int,
                                        imin(
                                            (*uv_t_dim).h as libc::c_int,
                                            (*f).bh - (*t).by + ss_ver >> ss_ver,
                                        ) as size_t,
                                    );
                                }
                            }
                            match imin(
                                (*uv_t_dim).w as libc::c_int,
                                (*f).bw - (*t).bx + ss_hor >> ss_hor,
                            ) {
                                1 => {
                                    (*(&mut *(*((*(*t).a).ccoef)
                                        .as_mut_ptr()
                                        .offset(pl as isize))
                                        .as_mut_ptr()
                                        .offset((cbx4 + x) as isize) as *mut uint8_t
                                        as *mut alias8))
                                        .u8_0 = (0x1 as libc::c_int * cf_ctx_0 as libc::c_int)
                                        as uint8_t;
                                }
                                2 => {
                                    (*(&mut *(*((*(*t).a).ccoef)
                                        .as_mut_ptr()
                                        .offset(pl as isize))
                                        .as_mut_ptr()
                                        .offset((cbx4 + x) as isize) as *mut uint8_t
                                        as *mut alias16))
                                        .u16_0 = (0x101 as libc::c_int * cf_ctx_0 as libc::c_int)
                                        as uint16_t;
                                }
                                4 => {
                                    (*(&mut *(*((*(*t).a).ccoef)
                                        .as_mut_ptr()
                                        .offset(pl as isize))
                                        .as_mut_ptr()
                                        .offset((cbx4 + x) as isize) as *mut uint8_t
                                        as *mut alias32))
                                        .u32_0 = (0x1010101 as libc::c_uint)
                                        .wrapping_mul(cf_ctx_0 as libc::c_uint);
                                }
                                8 => {
                                    (*(&mut *(*((*(*t).a).ccoef)
                                        .as_mut_ptr()
                                        .offset(pl as isize))
                                        .as_mut_ptr()
                                        .offset((cbx4 + x) as isize) as *mut uint8_t
                                        as *mut alias64))
                                        .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                                        .wrapping_mul(cf_ctx_0 as libc::c_ulonglong) as uint64_t;
                                }
                                16 => {
                                    let const_val_14: uint64_t = (0x101010101010101
                                        as libc::c_ulonglong)
                                        .wrapping_mul(cf_ctx_0 as libc::c_ulonglong) as uint64_t;
                                    (*(&mut *(*((*(*t).a).ccoef)
                                        .as_mut_ptr()
                                        .offset(pl as isize))
                                        .as_mut_ptr()
                                        .offset((cbx4 + x + 0 as libc::c_int) as isize)
                                        as *mut uint8_t as *mut alias64))
                                        .u64_0 = const_val_14;
                                    (*(&mut *(*((*(*t).a).ccoef)
                                        .as_mut_ptr()
                                        .offset(pl as isize))
                                        .as_mut_ptr()
                                        .offset((cbx4 + x + 8 as libc::c_int) as isize)
                                        as *mut uint8_t as *mut alias64))
                                        .u64_0 = const_val_14;
                                }
                                _ => {
                                    memset(
                                        &mut *(*((*(*t).a).ccoef).as_mut_ptr().offset(pl as isize))
                                            .as_mut_ptr()
                                            .offset((cbx4 + x) as isize) as *mut uint8_t
                                            as *mut libc::c_void,
                                        cf_ctx_0 as libc::c_int,
                                        imin(
                                            (*uv_t_dim).w as libc::c_int,
                                            (*f).bw - (*t).bx + ss_hor >> ss_hor,
                                        ) as size_t,
                                    );
                                }
                            }
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
        ^ (dst16 != 0 as *mut libc::c_void as *mut int16_t) as libc::c_int == 0
    {
        unreachable!();
    }
    let f: *const Dav1dFrameContext = (*t).f;
    let ss_ver: libc::c_int = (pl != 0
        && (*f).cur.p.layout as libc::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_hor: libc::c_int = (pl != 0
        && (*f).cur.p.layout as libc::c_uint
            != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
    let h_mul: libc::c_int = 4 as libc::c_int >> ss_hor;
    let v_mul: libc::c_int = 4 as libc::c_int >> ss_ver;
    let mvx: libc::c_int = mv.c2rust_unnamed.x as libc::c_int;
    let mvy: libc::c_int = mv.c2rust_unnamed.y as libc::c_int;
    let mx: libc::c_int = mvx & 15 as libc::c_int >> (ss_hor == 0) as libc::c_int;
    let my: libc::c_int = mvy & 15 as libc::c_int >> (ss_ver == 0) as libc::c_int;
    let mut ref_stride: ptrdiff_t = (*refp).p.stride[(pl != 0) as libc::c_int as usize];
    let mut ref_0: *const pixel = 0 as *const pixel;
    if (*refp).p.p.w == (*f).cur.p.w && (*refp).p.p.h == (*f).cur.p.h {
        let dx: libc::c_int = bx * h_mul + (mvx >> 3 as libc::c_int + ss_hor);
        let dy: libc::c_int = by * v_mul + (mvy >> 3 as libc::c_int + ss_ver);
        let mut w: libc::c_int = 0;
        let mut h: libc::c_int = 0;
        if (*refp).p.data[0 as libc::c_int as usize]
            != (*f).cur.data[0 as libc::c_int as usize]
        {
            w = (*f).cur.p.w + ss_hor >> ss_hor;
            h = (*f).cur.p.h + ss_ver >> ss_ver;
        } else {
            w = (*f).bw * 4 as libc::c_int >> ss_hor;
            h = (*f).bh * 4 as libc::c_int >> ss_ver;
        }
        if dx < (mx != 0) as libc::c_int * 3 as libc::c_int
            || dy < (my != 0) as libc::c_int * 3 as libc::c_int
            || dx + bw4 * h_mul + (mx != 0) as libc::c_int * 4 as libc::c_int > w
            || dy + bh4 * v_mul + (my != 0) as libc::c_int * 4 as libc::c_int > h
        {
            let emu_edge_buf: *mut pixel = ((*t)
                .scratch
                .c2rust_unnamed
                .c2rust_unnamed_0
                .emu_edge_16bpc)
                .as_mut_ptr();
            ((*(*f).dsp).mc.emu_edge)
                .expect(
                    "non-null function pointer",
                )(
                (bw4 * h_mul + (mx != 0) as libc::c_int * 7 as libc::c_int) as intptr_t,
                (bh4 * v_mul + (my != 0) as libc::c_int * 7 as libc::c_int) as intptr_t,
                w as intptr_t,
                h as intptr_t,
                (dx - (mx != 0) as libc::c_int * 3 as libc::c_int) as intptr_t,
                (dy - (my != 0) as libc::c_int * 3 as libc::c_int) as intptr_t,
                emu_edge_buf,
                (192 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong)
                    as ptrdiff_t,
                (*refp).p.data[pl as usize] as *const pixel,
                ref_stride,
            );
            ref_0 = &mut *emu_edge_buf
                .offset(
                    (192 as libc::c_int * (my != 0) as libc::c_int * 3 as libc::c_int
                        + (mx != 0) as libc::c_int * 3 as libc::c_int) as isize,
                ) as *mut pixel;
            ref_stride = (192 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong)
                as ptrdiff_t;
        } else {
            ref_0 = ((*refp).p.data[pl as usize] as *mut pixel)
                .offset(PXSTRIDE(ref_stride) * dy as isize)
                .offset(dx as isize);
        }
        if !dst8.is_null() {
            ((*(*f).dsp).mc.mc[filter_2d as usize])
                .expect(
                    "non-null function pointer",
                )(
                dst8,
                dst_stride,
                ref_0,
                ref_stride,
                bw4 * h_mul,
                bh4 * v_mul,
                mx << (ss_hor == 0) as libc::c_int,
                my << (ss_ver == 0) as libc::c_int,
                (*f).bitdepth_max,
            );
        } else {
            ((*(*f).dsp).mc.mct[filter_2d as usize])
                .expect(
                    "non-null function pointer",
                )(
                dst16,
                ref_0,
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
        let orig_pos_y: libc::c_int = (by * v_mul << 4 as libc::c_int)
            + mvy * ((1 as libc::c_int) << (ss_ver == 0) as libc::c_int);
        let orig_pos_x: libc::c_int = (bx * h_mul << 4 as libc::c_int)
            + mvx * ((1 as libc::c_int) << (ss_hor == 0) as libc::c_int);
        let mut pos_y: libc::c_int = 0;
        let mut pos_x: libc::c_int = 0;
        let tmp: int64_t = orig_pos_x as int64_t
            * (*f).svc[refidx as usize][0 as libc::c_int as usize].scale as int64_t
            + (((*f).svc[refidx as usize][0 as libc::c_int as usize].scale
                - 0x4000 as libc::c_int) * 8 as libc::c_int) as int64_t;
        pos_x = apply_sign64(
            (llabs(tmp as libc::c_longlong) + 128 as libc::c_longlong
                >> 8 as libc::c_int) as libc::c_int,
            tmp,
        ) + 32 as libc::c_int;
        let tmp_0: int64_t = orig_pos_y as int64_t
            * (*f).svc[refidx as usize][1 as libc::c_int as usize].scale as int64_t
            + (((*f).svc[refidx as usize][1 as libc::c_int as usize].scale
                - 0x4000 as libc::c_int) * 8 as libc::c_int) as int64_t;
        pos_y = apply_sign64(
            (llabs(tmp_0 as libc::c_longlong) + 128 as libc::c_longlong
                >> 8 as libc::c_int) as libc::c_int,
            tmp_0,
        ) + 32 as libc::c_int;
        let left: libc::c_int = pos_x >> 10 as libc::c_int;
        let top: libc::c_int = pos_y >> 10 as libc::c_int;
        let right: libc::c_int = (pos_x
            + (bw4 * h_mul - 1 as libc::c_int)
                * (*f).svc[refidx as usize][0 as libc::c_int as usize].step
            >> 10 as libc::c_int) + 1 as libc::c_int;
        let bottom: libc::c_int = (pos_y
            + (bh4 * v_mul - 1 as libc::c_int)
                * (*f).svc[refidx as usize][1 as libc::c_int as usize].step
            >> 10 as libc::c_int) + 1 as libc::c_int;
        if 0 as libc::c_int != 0 && (*(*f).frame_hdr).frame_offset == 2 as libc::c_int
            && (*t).by >= 0 as libc::c_int && (*t).by < 4 as libc::c_int
            && (*t).bx >= 8 as libc::c_int && (*t).bx < 12 as libc::c_int
        {
            printf(
                b"Off %dx%d [%d,%d,%d], size %dx%d [%d,%d]\n\0" as *const u8
                    as *const libc::c_char,
                left,
                top,
                orig_pos_x,
                (*f).svc[refidx as usize][0 as libc::c_int as usize].scale,
                refidx,
                right - left,
                bottom - top,
                (*f).svc[refidx as usize][0 as libc::c_int as usize].step,
                (*f).svc[refidx as usize][1 as libc::c_int as usize].step,
            );
        }
        let w_0: libc::c_int = (*refp).p.p.w + ss_hor >> ss_hor;
        let h_0: libc::c_int = (*refp).p.p.h + ss_ver >> ss_ver;
        if left < 3 as libc::c_int || top < 3 as libc::c_int
            || right + 4 as libc::c_int > w_0 || bottom + 4 as libc::c_int > h_0
        {
            let emu_edge_buf_0: *mut pixel = ((*t)
                .scratch
                .c2rust_unnamed
                .c2rust_unnamed_0
                .emu_edge_16bpc)
                .as_mut_ptr();
            ((*(*f).dsp).mc.emu_edge)
                .expect(
                    "non-null function pointer",
                )(
                (right - left + 7 as libc::c_int) as intptr_t,
                (bottom - top + 7 as libc::c_int) as intptr_t,
                w_0 as intptr_t,
                h_0 as intptr_t,
                (left - 3 as libc::c_int) as intptr_t,
                (top - 3 as libc::c_int) as intptr_t,
                emu_edge_buf_0,
                (320 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong)
                    as ptrdiff_t,
                (*refp).p.data[pl as usize] as *const pixel,
                ref_stride,
            );
            ref_0 = &mut *emu_edge_buf_0
                .offset(
                    (320 as libc::c_int * 3 as libc::c_int + 3 as libc::c_int) as isize,
                ) as *mut pixel;
            ref_stride = (320 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong)
                as ptrdiff_t;
            if 0 as libc::c_int != 0
                && (*(*f).frame_hdr).frame_offset == 2 as libc::c_int
                && (*t).by >= 0 as libc::c_int && (*t).by < 4 as libc::c_int
                && (*t).bx >= 8 as libc::c_int && (*t).bx < 12 as libc::c_int
            {
                printf(b"Emu\n\0" as *const u8 as *const libc::c_char);
            }
        } else {
            ref_0 = ((*refp).p.data[pl as usize] as *mut pixel)
                .offset(PXSTRIDE(ref_stride) * top as isize)
                .offset(left as isize);
        }
        if !dst8.is_null() {
            ((*(*f).dsp).mc.mc_scaled[filter_2d as usize])
                .expect(
                    "non-null function pointer",
                )(
                dst8,
                dst_stride,
                ref_0,
                ref_stride,
                bw4 * h_mul,
                bh4 * v_mul,
                pos_x & 0x3ff as libc::c_int,
                pos_y & 0x3ff as libc::c_int,
                (*f).svc[refidx as usize][0 as libc::c_int as usize].step,
                (*f).svc[refidx as usize][1 as libc::c_int as usize].step,
                (*f).bitdepth_max,
            );
        } else {
            ((*(*f).dsp).mc.mct_scaled[filter_2d as usize])
                .expect(
                    "non-null function pointer",
                )(
                dst16,
                ref_0,
                ref_stride,
                bw4 * h_mul,
                bh4 * v_mul,
                pos_x & 0x3ff as libc::c_int,
                pos_y & 0x3ff as libc::c_int,
                (*f).svc[refidx as usize][0 as libc::c_int as usize].step,
                (*f).svc[refidx as usize][1 as libc::c_int as usize].step,
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
    if !((*t).bx & 1 as libc::c_int == 0 && (*t).by & 1 as libc::c_int == 0) {
        unreachable!();
    }
    let f: *const Dav1dFrameContext = (*t).f;
    let mut r: *mut *mut refmvs_block = &mut *((*t).rt.r)
        .as_mut_ptr()
        .offset((((*t).by & 31 as libc::c_int) + 5 as libc::c_int) as isize)
        as *mut *mut refmvs_block;
    let lap: *mut pixel = ((*t).scratch.c2rust_unnamed.c2rust_unnamed.lap_16bpc)
        .as_mut_ptr();
    let ss_ver: libc::c_int = (pl != 0
        && (*f).cur.p.layout as libc::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_hor: libc::c_int = (pl != 0
        && (*f).cur.p.layout as libc::c_uint
            != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
    let h_mul: libc::c_int = 4 as libc::c_int >> ss_hor;
    let v_mul: libc::c_int = 4 as libc::c_int >> ss_ver;
    let mut res: libc::c_int = 0;
    if (*t).by > (*(*t).ts).tiling.row_start
        && (pl == 0
            || *b_dim.offset(0 as libc::c_int as isize) as libc::c_int * h_mul
                + *b_dim.offset(1 as libc::c_int as isize) as libc::c_int * v_mul
                >= 16 as libc::c_int)
    {
        let mut i: libc::c_int = 0 as libc::c_int;
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < w4
            && i
                < imin(
                    *b_dim.offset(2 as libc::c_int as isize) as libc::c_int,
                    4 as libc::c_int,
                )
        {
            let a_r: *const refmvs_block = &mut *(*r
                .offset(-(1 as libc::c_int) as isize))
                .offset(((*t).bx + x + 1 as libc::c_int) as isize) as *mut refmvs_block;
            let a_b_dim: *const uint8_t = (dav1d_block_dimensions[(*a_r).bs as usize])
                .as_ptr();
            let step4: libc::c_int = iclip(
                *a_b_dim.offset(0 as libc::c_int as isize) as libc::c_int,
                2 as libc::c_int,
                16 as libc::c_int,
            );
            if (*a_r).ref_0.ref_0[0 as libc::c_int as usize] as libc::c_int
                > 0 as libc::c_int
            {
                let ow4: libc::c_int = imin(
                    step4,
                    *b_dim.offset(0 as libc::c_int as isize) as libc::c_int,
                );
                let oh4: libc::c_int = imin(
                    *b_dim.offset(1 as libc::c_int as isize) as libc::c_int,
                    16 as libc::c_int,
                ) >> 1 as libc::c_int;
                res = mc(
                    t,
                    lap,
                    0 as *mut int16_t,
                    ((ow4 * h_mul) as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong)
                        as ptrdiff_t,
                    ow4,
                    oh4 * 3 as libc::c_int + 3 as libc::c_int >> 2 as libc::c_int,
                    (*t).bx + x,
                    (*t).by,
                    pl,
                    (*a_r).mv.mv[0 as libc::c_int as usize],
                    &*((*f).refp)
                        .as_ptr()
                        .offset(
                            (*((*a_r).ref_0.ref_0)
                                .as_ptr()
                                .offset(0 as libc::c_int as isize) as libc::c_int
                                - 1 as libc::c_int) as isize,
                        ),
                    (*a_r).ref_0.ref_0[0 as libc::c_int as usize] as libc::c_int
                        - 1 as libc::c_int,
                    dav1d_filter_2d[(*(*t).a)
                        .filter[1 as libc::c_int
                        as usize][(bx4 + x + 1 as libc::c_int) as usize]
                        as usize][(*(*t).a)
                        .filter[0 as libc::c_int
                        as usize][(bx4 + x + 1 as libc::c_int) as usize] as usize]
                        as Filter2d,
                );
                if res != 0 {
                    return res;
                }
                ((*(*f).dsp).mc.blend_h)
                    .expect(
                        "non-null function pointer",
                    )(
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
        let mut i_0: libc::c_int = 0 as libc::c_int;
        let mut y: libc::c_int = 0 as libc::c_int;
        while y < h4
            && i_0
                < imin(
                    *b_dim.offset(3 as libc::c_int as isize) as libc::c_int,
                    4 as libc::c_int,
                )
        {
            let l_r: *const refmvs_block = &mut *(*r
                .offset((y + 1 as libc::c_int) as isize))
                .offset(((*t).bx - 1 as libc::c_int) as isize) as *mut refmvs_block;
            let l_b_dim: *const uint8_t = (dav1d_block_dimensions[(*l_r).bs as usize])
                .as_ptr();
            let step4_0: libc::c_int = iclip(
                *l_b_dim.offset(1 as libc::c_int as isize) as libc::c_int,
                2 as libc::c_int,
                16 as libc::c_int,
            );
            if (*l_r).ref_0.ref_0[0 as libc::c_int as usize] as libc::c_int
                > 0 as libc::c_int
            {
                let ow4_0: libc::c_int = imin(
                    *b_dim.offset(0 as libc::c_int as isize) as libc::c_int,
                    16 as libc::c_int,
                ) >> 1 as libc::c_int;
                let oh4_0: libc::c_int = imin(
                    step4_0,
                    *b_dim.offset(1 as libc::c_int as isize) as libc::c_int,
                );
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
                    (*l_r).mv.mv[0 as libc::c_int as usize],
                    &*((*f).refp)
                        .as_ptr()
                        .offset(
                            (*((*l_r).ref_0.ref_0)
                                .as_ptr()
                                .offset(0 as libc::c_int as isize) as libc::c_int
                                - 1 as libc::c_int) as isize,
                        ),
                    (*l_r).ref_0.ref_0[0 as libc::c_int as usize] as libc::c_int
                        - 1 as libc::c_int,
                    dav1d_filter_2d[(*t)
                        .l
                        .filter[1 as libc::c_int
                        as usize][(by4 + y + 1 as libc::c_int) as usize]
                        as usize][(*t)
                        .l
                        .filter[0 as libc::c_int
                        as usize][(by4 + y + 1 as libc::c_int) as usize] as usize]
                        as Filter2d,
                );
                if res != 0 {
                    return res;
                }
                ((*(*f).dsp).mc.blend_v)
                    .expect(
                        "non-null function pointer",
                    )(
                    &mut *dst
                        .offset(
                            ((y * v_mul) as isize
                                * (PXSTRIDE
                                    as unsafe extern "C" fn(
                                        ptrdiff_t,
                                    ) -> ptrdiff_t)(dst_stride)) as isize,
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
        ^ (dst16 != 0 as *mut libc::c_void as *mut int16_t) as libc::c_int == 0
    {
        unreachable!();
    }
    let f: *const Dav1dFrameContext = (*t).f;
    let dsp: *const Dav1dDSPContext = (*f).dsp;
    let ss_ver: libc::c_int = (pl != 0
        && (*f).cur.p.layout as libc::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_hor: libc::c_int = (pl != 0
        && (*f).cur.p.layout as libc::c_uint
            != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
    let h_mul: libc::c_int = 4 as libc::c_int >> ss_hor;
    let v_mul: libc::c_int = 4 as libc::c_int >> ss_ver;
    if !(*b_dim.offset(0 as libc::c_int as isize) as libc::c_int * h_mul
        & 7 as libc::c_int == 0
        && *b_dim.offset(1 as libc::c_int as isize) as libc::c_int * v_mul
            & 7 as libc::c_int == 0)
    {
        unreachable!();
    }
    let mat: *const int32_t = ((*wmp).matrix).as_ptr();
    let width: libc::c_int = (*refp).p.p.w + ss_hor >> ss_hor;
    let height: libc::c_int = (*refp).p.p.h + ss_ver >> ss_ver;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < *b_dim.offset(1 as libc::c_int as isize) as libc::c_int * v_mul {
        let src_y: libc::c_int = (*t).by * 4 as libc::c_int
            + ((y + 4 as libc::c_int) << ss_ver);
        let mat3_y: int64_t = *mat.offset(3 as libc::c_int as isize) as int64_t
            * src_y as int64_t
            + *mat.offset(0 as libc::c_int as isize) as int64_t;
        let mat5_y: int64_t = *mat.offset(5 as libc::c_int as isize) as int64_t
            * src_y as int64_t
            + *mat.offset(1 as libc::c_int as isize) as int64_t;
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < *b_dim.offset(0 as libc::c_int as isize) as libc::c_int * h_mul {
            let src_x: libc::c_int = (*t).bx * 4 as libc::c_int
                + ((x + 4 as libc::c_int) << ss_hor);
            let mvx: int64_t = *mat.offset(2 as libc::c_int as isize) as int64_t
                * src_x as int64_t + mat3_y >> ss_hor;
            let mvy: int64_t = *mat.offset(4 as libc::c_int as isize) as int64_t
                * src_x as int64_t + mat5_y >> ss_ver;
            let dx: libc::c_int = (mvx >> 16 as libc::c_int) as libc::c_int
                - 4 as libc::c_int;
            let mx: libc::c_int = (mvx as libc::c_int & 0xffff as libc::c_int)
                - (*wmp).u.p.alpha as libc::c_int * 4 as libc::c_int
                - (*wmp).u.p.beta as libc::c_int * 7 as libc::c_int
                & !(0x3f as libc::c_int);
            let dy: libc::c_int = (mvy >> 16 as libc::c_int) as libc::c_int
                - 4 as libc::c_int;
            let my: libc::c_int = (mvy as libc::c_int & 0xffff as libc::c_int)
                - (*wmp).u.p.gamma as libc::c_int * 4 as libc::c_int
                - (*wmp).u.p.delta as libc::c_int * 4 as libc::c_int
                & !(0x3f as libc::c_int);
            let mut ref_ptr: *const pixel = 0 as *const pixel;
            let mut ref_stride: ptrdiff_t = (*refp)
                .p
                .stride[(pl != 0) as libc::c_int as usize];
            if dx < 3 as libc::c_int || dx + 8 as libc::c_int + 4 as libc::c_int > width
                || dy < 3 as libc::c_int
                || dy + 8 as libc::c_int + 4 as libc::c_int > height
            {
                let emu_edge_buf: *mut pixel = ((*t)
                    .scratch
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .emu_edge_16bpc)
                    .as_mut_ptr();
                ((*(*f).dsp).mc.emu_edge)
                    .expect(
                        "non-null function pointer",
                    )(
                    15 as libc::c_int as intptr_t,
                    15 as libc::c_int as intptr_t,
                    width as intptr_t,
                    height as intptr_t,
                    (dx - 3 as libc::c_int) as intptr_t,
                    (dy - 3 as libc::c_int) as intptr_t,
                    emu_edge_buf,
                    (32 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong)
                        as ptrdiff_t,
                    (*refp).p.data[pl as usize] as *const pixel,
                    ref_stride,
                );
                ref_ptr = &mut *emu_edge_buf
                    .offset((32 * 3  + 3) as isize,) as *mut pixel;
                ref_stride = (32 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong)
                    as ptrdiff_t;
            } else {
                ref_ptr = ((*refp).p.data[pl as usize] as *mut pixel)
                    .offset((PXSTRIDE(ref_stride) * dy as isize) as isize)
                    .offset(dx as isize);
            }
            if !dst16.is_null() {
                ((*dsp).mc.warp8x8t)
                    .expect(
                        "non-null function pointer",
                    )(
                    &mut *dst16.offset(x as isize),
                    dstride,
                    ref_ptr,
                    ref_stride,
                    ((*wmp).u.abcd).as_ptr(),
                    mx,
                    my,
                    (*f).bitdepth_max,
                );
            } else {
                ((*dsp).mc.warp8x8)
                    .expect(
                        "non-null function pointer",
                    )(
                    &mut *dst8.offset(x as isize),
                    dstride,
                    ref_ptr,
                    ref_stride,
                    ((*wmp).u.abcd).as_ptr(),
                    mx,
                    my,
                    (*f).bitdepth_max,
                );
            }
            x += 8 as libc::c_int;
        }
        if !dst8.is_null() {
            dst8 = dst8
                .offset((8 * PXSTRIDE(dstride)) as isize);
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
    let bx4: libc::c_int = (*t).bx & 31 as libc::c_int;
    let by4: libc::c_int = (*t).by & 31 as libc::c_int;
    let ss_ver: libc::c_int = ((*f).cur.p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_hor: libc::c_int = ((*f).cur.p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
    let cbx4: libc::c_int = bx4 >> ss_hor;
    let cby4: libc::c_int = by4 >> ss_ver;
    let b_dim: *const uint8_t = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bw4: libc::c_int = *b_dim.offset(0 as libc::c_int as isize) as libc::c_int;
    let bh4: libc::c_int = *b_dim.offset(1 as libc::c_int as isize) as libc::c_int;
    let w4: libc::c_int = imin(bw4, (*f).bw - (*t).bx);
    let h4: libc::c_int = imin(bh4, (*f).bh - (*t).by);
    let cw4: libc::c_int = w4 + ss_hor >> ss_hor;
    let ch4: libc::c_int = h4 + ss_ver >> ss_ver;
    let has_chroma: libc::c_int = ((*f).cur.p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint
        && (bw4 > ss_hor || (*t).bx & 1 as libc::c_int != 0)
        && (bh4 > ss_ver || (*t).by & 1 as libc::c_int != 0)) as libc::c_int;
    let t_dim: *const TxfmInfo = &*dav1d_txfm_dimensions
        .as_ptr()
        .offset((*b).c2rust_unnamed.c2rust_unnamed.tx as isize) as *const TxfmInfo;
    let uv_t_dim: *const TxfmInfo = &*dav1d_txfm_dimensions
        .as_ptr()
        .offset((*b).uvtx as isize) as *const TxfmInfo;
    let edge: *mut pixel = ((*t)
        .scratch
        .c2rust_unnamed_0
        .c2rust_unnamed_0
        .c2rust_unnamed_0
        .edge_16bpc)
        .as_mut_ptr()
        .offset(128 as libc::c_int as isize);
    let cbw4: libc::c_int = bw4 + ss_hor >> ss_hor;
    let cbh4: libc::c_int = bh4 + ss_ver >> ss_ver;
    let intra_edge_filter_flag: libc::c_int = (*(*f).seq_hdr).intra_edge_filter
        << 10 as libc::c_int;
    let mut init_y: libc::c_int = 0 as libc::c_int;
    while init_y < h4 {
        let sub_h4: libc::c_int = imin(h4, 16 as libc::c_int + init_y);
        let sub_ch4: libc::c_int = imin(ch4, init_y + 16 as libc::c_int >> ss_ver);
        let mut init_x: libc::c_int = 0 as libc::c_int;
        while init_x < w4 {
            if (*b).c2rust_unnamed.c2rust_unnamed.pal_sz[0 as libc::c_int as usize] != 0
            {
                let mut dst: *mut pixel = ((*f).cur.data[0 as libc::c_int as usize]
                    as *mut pixel)
                    .offset(
                        (4
                            * ((*t).by as isize
                                * PXSTRIDE((*f).cur.stride[0 as libc::c_int as usize])
                                + (*t).bx as isize)) as isize,
                    );
                let mut pal_idx: *const uint8_t = 0 as *const uint8_t;
                if (*t).frame_thread.pass != 0 {
                    let p: libc::c_int = (*t).frame_thread.pass & 1 as libc::c_int;
                    if ((*ts).frame_thread[p as usize].pal_idx).is_null() {
                        unreachable!();
                    }
                    pal_idx = (*ts).frame_thread[p as usize].pal_idx;
                    (*ts)
                        .frame_thread[p as usize]
                        .pal_idx = ((*ts).frame_thread[p as usize].pal_idx)
                        .offset((bw4 * bh4 * 16 as libc::c_int) as isize);
                } else {
                    pal_idx = ((*t).scratch.c2rust_unnamed_0.pal_idx).as_mut_ptr();
                }
                let pal: *const uint16_t = if (*t).frame_thread.pass != 0 {
                    ((*((*f).frame_thread.pal)
                        .offset(
                            ((((*t).by as isize >> 1)
                                + ((*t).bx as isize & 1))
                                * ((*f).b4_stride >> 1)
                                + (((*t).bx >> 1 as libc::c_int)
                                    + ((*t).by & 1 as libc::c_int)) as isize) as isize,
                        ))[0 as libc::c_int as usize])
                        .as_mut_ptr()
                } else {
                    ((*t).scratch.c2rust_unnamed_0.pal[0 as libc::c_int as usize])
                        .as_mut_ptr()
                };
                ((*(*f).dsp).ipred.pal_pred)
                    .expect(
                        "non-null function pointer",
                    )(
                    dst,
                    (*f).cur.stride[0 as libc::c_int as usize],
                    pal,
                    pal_idx,
                    bw4 * 4 as libc::c_int,
                    bh4 * 4 as libc::c_int,
                );
                if 0 as libc::c_int != 0
                    && (*(*f).frame_hdr).frame_offset == 2 as libc::c_int
                    && (*t).by >= 0 as libc::c_int && (*t).by < 4 as libc::c_int
                    && (*t).bx >= 8 as libc::c_int && (*t).bx < 12 as libc::c_int
                    && 0 as libc::c_int != 0
                {
                    hex_dump(
                        dst,
                        PXSTRIDE((*f).cur.stride[0 as libc::c_int as usize]),
                        bw4 * 4 as libc::c_int,
                        bh4 * 4 as libc::c_int,
                        b"y-pal-pred\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            let intra_flags: libc::c_int = sm_flag((*t).a, bx4)
                | sm_flag(&mut (*t).l, by4) | intra_edge_filter_flag;
            let sb_has_tr: libc::c_int = (if (init_x + 16 as libc::c_int) < w4 {
                1 as libc::c_int as libc::c_uint
            } else if init_y != 0 {
                0 as libc::c_int as libc::c_uint
            } else {
                intra_edge_flags as libc::c_uint
                    & EDGE_I444_TOP_HAS_RIGHT as libc::c_int as libc::c_uint
            }) as libc::c_int;
            let sb_has_bl: libc::c_int = (if init_x != 0 {
                0 as libc::c_int as libc::c_uint
            } else if (init_y + 16 as libc::c_int) < h4 {
                1 as libc::c_int as libc::c_uint
            } else {
                intra_edge_flags as libc::c_uint
                    & EDGE_I444_LEFT_HAS_BOTTOM as libc::c_int as libc::c_uint
            }) as libc::c_int;
            let mut y: libc::c_int = 0;
            let mut x: libc::c_int = 0;
            let sub_w4: libc::c_int = imin(w4, init_x + 16 as libc::c_int);
            y = init_y;
            (*t).by += init_y;
            while y < sub_h4 {
                let mut dst_0: *mut pixel = ((*f).cur.data[0 as libc::c_int as usize]
                    as *mut pixel)
                    .offset(
                        (4
                            * ((*t).by as isize
                                * PXSTRIDE((*f).cur.stride[0 as libc::c_int as usize])
                                + (*t).bx as isize + init_x as isize))
                            as isize,
                    );
                x = init_x;
                (*t).bx += init_x;
                while x < sub_w4 {
                    let mut angle: libc::c_int = 0;
                    let mut edge_flags: EdgeFlags = 0 as EdgeFlags;
                    let mut top_sb_edge: *const pixel = 0 as *const pixel;
                    let mut m: IntraPredMode = DC_PRED;
                    if !((*b)
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .pal_sz[0 as libc::c_int as usize] != 0)
                    {
                        angle = (*b).c2rust_unnamed.c2rust_unnamed.y_angle
                            as libc::c_int;
                        edge_flags = ((if (y > init_y || sb_has_tr == 0)
                            && x + (*t_dim).w as libc::c_int >= sub_w4
                        {
                            0 as libc::c_int
                        } else {
                            EDGE_I444_TOP_HAS_RIGHT as libc::c_int
                        })
                            | (if x > init_x
                                || sb_has_bl == 0 && y + (*t_dim).h as libc::c_int >= sub_h4
                            {
                                0 as libc::c_int
                            } else {
                                EDGE_I444_LEFT_HAS_BOTTOM as libc::c_int
                            })) as EdgeFlags;
                        top_sb_edge = 0 as *const pixel;
                        if (*t).by & (*f).sb_step - 1 as libc::c_int == 0 {
                            top_sb_edge = (*f).ipred_edge[0 as libc::c_int as usize];
                            let sby: libc::c_int = (*t).by >> (*f).sb_shift;
                            top_sb_edge = top_sb_edge
                                .offset(
                                    ((*f).sb128w * 128 as libc::c_int
                                        * (sby - 1 as libc::c_int)) as isize,
                                );
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
                            (*f).cur.stride[0 as libc::c_int as usize],
                            top_sb_edge,
                            (*b).c2rust_unnamed.c2rust_unnamed.y_mode as IntraPredMode,
                            &mut angle,
                            (*t_dim).w as libc::c_int,
                            (*t_dim).h as libc::c_int,
                            (*(*f).seq_hdr).intra_edge_filter,
                            edge,
                            (*f).bitdepth_max,
                        );
                        ((*dsp).ipred.intra_pred[m as usize])
                            .expect(
                                "non-null function pointer",
                            )(
                            dst_0,
                            (*f).cur.stride[0 as libc::c_int as usize],
                            edge,
                            (*t_dim).w as libc::c_int * 4 as libc::c_int,
                            (*t_dim).h as libc::c_int * 4 as libc::c_int,
                            angle | intra_flags,
                            4 as libc::c_int * (*f).bw - 4 as libc::c_int * (*t).bx,
                            4 as libc::c_int * (*f).bh - 4 as libc::c_int * (*t).by,
                            (*f).bitdepth_max,
                        );
                        if 0 as libc::c_int != 0
                            && (*(*f).frame_hdr).frame_offset == 2 as libc::c_int
                            && (*t).by >= 0 as libc::c_int && (*t).by < 4 as libc::c_int
                            && (*t).bx >= 8 as libc::c_int && (*t).bx < 12 as libc::c_int
                            && 0 as libc::c_int != 0
                        {
                            hex_dump(
                                edge
                                    .offset(
                                        -(((*t_dim).h as libc::c_int * 4 as libc::c_int) as isize),
                                    ),
                                ((*t_dim).h as libc::c_int * 4 as libc::c_int) as ptrdiff_t,
                                (*t_dim).h as libc::c_int * 4 as libc::c_int,
                                2 as libc::c_int,
                                b"l\0" as *const u8 as *const libc::c_char,
                            );
                            hex_dump(
                                edge,
                                0 as libc::c_int as ptrdiff_t,
                                1 as libc::c_int,
                                1 as libc::c_int,
                                b"tl\0" as *const u8 as *const libc::c_char,
                            );
                            hex_dump(
                                edge.offset(1 as libc::c_int as isize),
                                ((*t_dim).w as libc::c_int * 4 as libc::c_int) as ptrdiff_t,
                                (*t_dim).w as libc::c_int * 4 as libc::c_int,
                                2 as libc::c_int,
                                b"t\0" as *const u8 as *const libc::c_char,
                            );
                            hex_dump(
                                dst_0,
                                (*f).cur.stride[0 as libc::c_int as usize],
                                (*t_dim).w as libc::c_int * 4 as libc::c_int,
                                (*t_dim).h as libc::c_int * 4 as libc::c_int,
                                b"y-intra-pred\0" as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                    if (*b).skip == 0 {
                        let mut cf: *mut coef = 0 as *mut coef;
                        let mut eob: libc::c_int = 0;
                        let mut txtp: TxfmType = DCT_DCT;
                        if (*t).frame_thread.pass != 0 {
                            let p_0: libc::c_int = (*t).frame_thread.pass
                                & 1 as libc::c_int;
                            cf = (*ts).frame_thread[p_0 as usize].cf;
                            (*ts)
                                .frame_thread[p_0 as usize]
                                .cf = ((*ts).frame_thread[p_0 as usize].cf)
                                .offset(
                                    (imin((*t_dim).w as libc::c_int, 8 as libc::c_int)
                                        * imin((*t_dim).h as libc::c_int, 8 as libc::c_int)
                                        * 16 as libc::c_int) as isize,
                                );
                            let cbi: *const CodedBlockInfo = &mut *((*f)
                                .frame_thread
                                .cbi)
                                .offset(
                                    ((*t).by as isize * (*f).b4_stride
                                        + (*t).bx as isize) as isize,
                                ) as *mut CodedBlockInfo;
                            eob = (*cbi).eob[0 as libc::c_int as usize] as libc::c_int;
                            txtp = (*cbi).txtp[0 as libc::c_int as usize] as TxfmType;
                        } else {
                            let mut cf_ctx: uint8_t = 0;
                            cf = ((*t).c2rust_unnamed.cf_16bpc).as_mut_ptr();
                            eob = decode_coefs(
                                t,
                                &mut *((*(*t).a).lcoef)
                                    .as_mut_ptr()
                                    .offset((bx4 + x) as isize),
                                &mut *((*t).l.lcoef)
                                    .as_mut_ptr()
                                    .offset((by4 + y) as isize),
                                (*b).c2rust_unnamed.c2rust_unnamed.tx as RectTxfmSize,
                                bs,
                                b,
                                1 as libc::c_int,
                                0 as libc::c_int,
                                cf,
                                &mut txtp,
                                &mut cf_ctx,
                            );
                            if 0 as libc::c_int != 0
                                && (*(*f).frame_hdr).frame_offset == 2 as libc::c_int
                                && (*t).by >= 0 as libc::c_int && (*t).by < 4 as libc::c_int
                                && (*t).bx >= 8 as libc::c_int
                                && (*t).bx < 12 as libc::c_int
                            {
                                printf(
                                    b"Post-y-cf-blk[tx=%d,txtp=%d,eob=%d]: r=%d\n\0"
                                        as *const u8 as *const libc::c_char,
                                    (*b).c2rust_unnamed.c2rust_unnamed.tx as libc::c_int,
                                    txtp as libc::c_uint,
                                    eob,
                                    (*ts).msac.rng,
                                );
                            }
                            match imin((*t_dim).h as libc::c_int, (*f).bh - (*t).by) {
                                1 => {
                                    (*(&mut *((*t).l.lcoef)
                                        .as_mut_ptr()
                                        .offset((by4 + y) as isize) as *mut uint8_t as *mut alias8))
                                        .u8_0 = (0x1 as libc::c_int * cf_ctx as libc::c_int)
                                        as uint8_t;
                                }
                                2 => {
                                    (*(&mut *((*t).l.lcoef)
                                        .as_mut_ptr()
                                        .offset((by4 + y) as isize) as *mut uint8_t
                                        as *mut alias16))
                                        .u16_0 = (0x101 as libc::c_int * cf_ctx as libc::c_int)
                                        as uint16_t;
                                }
                                4 => {
                                    (*(&mut *((*t).l.lcoef)
                                        .as_mut_ptr()
                                        .offset((by4 + y) as isize) as *mut uint8_t
                                        as *mut alias32))
                                        .u32_0 = (0x1010101 as libc::c_uint)
                                        .wrapping_mul(cf_ctx as libc::c_uint);
                                }
                                8 => {
                                    (*(&mut *((*t).l.lcoef)
                                        .as_mut_ptr()
                                        .offset((by4 + y) as isize) as *mut uint8_t
                                        as *mut alias64))
                                        .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                                        .wrapping_mul(cf_ctx as libc::c_ulonglong) as uint64_t;
                                }
                                16 => {
                                    let const_val: uint64_t = (0x101010101010101
                                        as libc::c_ulonglong)
                                        .wrapping_mul(cf_ctx as libc::c_ulonglong) as uint64_t;
                                    (*(&mut *((*t).l.lcoef)
                                        .as_mut_ptr()
                                        .offset((by4 + y + 0 as libc::c_int) as isize)
                                        as *mut uint8_t as *mut alias64))
                                        .u64_0 = const_val;
                                    (*(&mut *((*t).l.lcoef)
                                        .as_mut_ptr()
                                        .offset((by4 + y + 8 as libc::c_int) as isize)
                                        as *mut uint8_t as *mut alias64))
                                        .u64_0 = const_val;
                                }
                                _ => {
                                    memset(
                                        &mut *((*t).l.lcoef).as_mut_ptr().offset((by4 + y) as isize)
                                            as *mut uint8_t as *mut libc::c_void,
                                        cf_ctx as libc::c_int,
                                        imin((*t_dim).h as libc::c_int, (*f).bh - (*t).by)
                                            as size_t,
                                    );
                                }
                            }
                            match imin((*t_dim).w as libc::c_int, (*f).bw - (*t).bx) {
                                1 => {
                                    (*(&mut *((*(*t).a).lcoef)
                                        .as_mut_ptr()
                                        .offset((bx4 + x) as isize) as *mut uint8_t as *mut alias8))
                                        .u8_0 = (0x1 as libc::c_int * cf_ctx as libc::c_int)
                                        as uint8_t;
                                }
                                2 => {
                                    (*(&mut *((*(*t).a).lcoef)
                                        .as_mut_ptr()
                                        .offset((bx4 + x) as isize) as *mut uint8_t
                                        as *mut alias16))
                                        .u16_0 = (0x101 as libc::c_int * cf_ctx as libc::c_int)
                                        as uint16_t;
                                }
                                4 => {
                                    (*(&mut *((*(*t).a).lcoef)
                                        .as_mut_ptr()
                                        .offset((bx4 + x) as isize) as *mut uint8_t
                                        as *mut alias32))
                                        .u32_0 = (0x1010101 as libc::c_uint)
                                        .wrapping_mul(cf_ctx as libc::c_uint);
                                }
                                8 => {
                                    (*(&mut *((*(*t).a).lcoef)
                                        .as_mut_ptr()
                                        .offset((bx4 + x) as isize) as *mut uint8_t
                                        as *mut alias64))
                                        .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                                        .wrapping_mul(cf_ctx as libc::c_ulonglong) as uint64_t;
                                }
                                16 => {
                                    let const_val_0: uint64_t = (0x101010101010101
                                        as libc::c_ulonglong)
                                        .wrapping_mul(cf_ctx as libc::c_ulonglong) as uint64_t;
                                    (*(&mut *((*(*t).a).lcoef)
                                        .as_mut_ptr()
                                        .offset((bx4 + x + 0 as libc::c_int) as isize)
                                        as *mut uint8_t as *mut alias64))
                                        .u64_0 = const_val_0;
                                    (*(&mut *((*(*t).a).lcoef)
                                        .as_mut_ptr()
                                        .offset((bx4 + x + 8 as libc::c_int) as isize)
                                        as *mut uint8_t as *mut alias64))
                                        .u64_0 = const_val_0;
                                }
                                _ => {
                                    memset(
                                        &mut *((*(*t).a).lcoef)
                                            .as_mut_ptr()
                                            .offset((bx4 + x) as isize) as *mut uint8_t
                                            as *mut libc::c_void,
                                        cf_ctx as libc::c_int,
                                        imin((*t_dim).w as libc::c_int, (*f).bw - (*t).bx)
                                            as size_t,
                                    );
                                }
                            }
                        }
                        if eob >= 0 as libc::c_int {
                            if 0 as libc::c_int != 0
                                && (*(*f).frame_hdr).frame_offset == 2 as libc::c_int
                                && (*t).by >= 0 as libc::c_int && (*t).by < 4 as libc::c_int
                                && (*t).bx >= 8 as libc::c_int
                                && (*t).bx < 12 as libc::c_int && 0 as libc::c_int != 0
                            {
                                coef_dump(
                                    cf,
                                    imin((*t_dim).h as libc::c_int, 8 as libc::c_int)
                                        * 4 as libc::c_int,
                                    imin((*t_dim).w as libc::c_int, 8 as libc::c_int)
                                        * 4 as libc::c_int,
                                    3 as libc::c_int,
                                    b"dq\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            ((*dsp)
                                .itx
                                .itxfm_add[(*b).c2rust_unnamed.c2rust_unnamed.tx
                                as usize][txtp as usize])
                                .expect(
                                    "non-null function pointer",
                                )(
                                dst_0,
                                (*f).cur.stride[0 as libc::c_int as usize],
                                cf,
                                eob,
                                (*f).bitdepth_max,
                            );
                            if 0 as libc::c_int != 0
                                && (*(*f).frame_hdr).frame_offset == 2 as libc::c_int
                                && (*t).by >= 0 as libc::c_int && (*t).by < 4 as libc::c_int
                                && (*t).bx >= 8 as libc::c_int
                                && (*t).bx < 12 as libc::c_int && 0 as libc::c_int != 0
                            {
                                hex_dump(
                                    dst_0,
                                    (*f).cur.stride[0 as libc::c_int as usize],
                                    (*t_dim).w as libc::c_int * 4 as libc::c_int,
                                    (*t_dim).h as libc::c_int * 4 as libc::c_int,
                                    b"recon\0" as *const u8 as *const libc::c_char,
                                );
                            }
                        }
                    } else if (*t).frame_thread.pass == 0 {
                        match (*t_dim).h as libc::c_int {
                            1 => {
                                (*(&mut *((*t).l.lcoef)
                                    .as_mut_ptr()
                                    .offset((by4 + y) as isize) as *mut uint8_t as *mut alias8))
                                    .u8_0 = (0x1 as libc::c_int * 0x40 as libc::c_int)
                                    as uint8_t;
                            }
                            2 => {
                                (*(&mut *((*t).l.lcoef)
                                    .as_mut_ptr()
                                    .offset((by4 + y) as isize) as *mut uint8_t
                                    as *mut alias16))
                                    .u16_0 = (0x101 as libc::c_int * 0x40 as libc::c_int)
                                    as uint16_t;
                            }
                            4 => {
                                (*(&mut *((*t).l.lcoef)
                                    .as_mut_ptr()
                                    .offset((by4 + y) as isize) as *mut uint8_t
                                    as *mut alias32))
                                    .u32_0 = (0x1010101 as libc::c_uint)
                                    .wrapping_mul(0x40 as libc::c_int as libc::c_uint);
                            }
                            8 => {
                                (*(&mut *((*t).l.lcoef)
                                    .as_mut_ptr()
                                    .offset((by4 + y) as isize) as *mut uint8_t
                                    as *mut alias64))
                                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                                    .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                                    as uint64_t;
                            }
                            16 => {
                                let const_val_1: uint64_t = (0x101010101010101
                                    as libc::c_ulonglong)
                                    .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                                    as uint64_t;
                                (*(&mut *((*t).l.lcoef)
                                    .as_mut_ptr()
                                    .offset((by4 + y + 0 as libc::c_int) as isize)
                                    as *mut uint8_t as *mut alias64))
                                    .u64_0 = const_val_1;
                                (*(&mut *((*t).l.lcoef)
                                    .as_mut_ptr()
                                    .offset((by4 + y + 8 as libc::c_int) as isize)
                                    as *mut uint8_t as *mut alias64))
                                    .u64_0 = const_val_1;
                            }
                            _ => {}
                        }
                        match (*t_dim).w as libc::c_int {
                            1 => {
                                (*(&mut *((*(*t).a).lcoef)
                                    .as_mut_ptr()
                                    .offset((bx4 + x) as isize) as *mut uint8_t as *mut alias8))
                                    .u8_0 = (0x1 as libc::c_int * 0x40 as libc::c_int)
                                    as uint8_t;
                            }
                            2 => {
                                (*(&mut *((*(*t).a).lcoef)
                                    .as_mut_ptr()
                                    .offset((bx4 + x) as isize) as *mut uint8_t
                                    as *mut alias16))
                                    .u16_0 = (0x101 as libc::c_int * 0x40 as libc::c_int)
                                    as uint16_t;
                            }
                            4 => {
                                (*(&mut *((*(*t).a).lcoef)
                                    .as_mut_ptr()
                                    .offset((bx4 + x) as isize) as *mut uint8_t
                                    as *mut alias32))
                                    .u32_0 = (0x1010101 as libc::c_uint)
                                    .wrapping_mul(0x40 as libc::c_int as libc::c_uint);
                            }
                            8 => {
                                (*(&mut *((*(*t).a).lcoef)
                                    .as_mut_ptr()
                                    .offset((bx4 + x) as isize) as *mut uint8_t
                                    as *mut alias64))
                                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                                    .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                                    as uint64_t;
                            }
                            16 => {
                                let const_val_2: uint64_t = (0x101010101010101
                                    as libc::c_ulonglong)
                                    .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                                    as uint64_t;
                                (*(&mut *((*(*t).a).lcoef)
                                    .as_mut_ptr()
                                    .offset((bx4 + x + 0 as libc::c_int) as isize)
                                    as *mut uint8_t as *mut alias64))
                                    .u64_0 = const_val_2;
                                (*(&mut *((*(*t).a).lcoef)
                                    .as_mut_ptr()
                                    .offset((bx4 + x + 8 as libc::c_int) as isize)
                                    as *mut uint8_t as *mut alias64))
                                    .u64_0 = const_val_2;
                            }
                            _ => {}
                        }
                    }
                    dst_0 = dst_0
                        .offset((4 as libc::c_int * (*t_dim).w as libc::c_int) as isize);
                    x += (*t_dim).w as libc::c_int;
                    (*t).bx += (*t_dim).w as libc::c_int;
                }
                (*t).bx -= x;
                y += (*t_dim).h as libc::c_int;
                (*t).by += (*t_dim).h as libc::c_int;
            }
            (*t).by -= y;
            if !(has_chroma == 0) {
                let stride: ptrdiff_t = (*f).cur.stride[1 as libc::c_int as usize];
                if (*b).c2rust_unnamed.c2rust_unnamed.uv_mode as libc::c_int
                    == CFL_PRED as libc::c_int
                {
                    if !(init_x == 0 && init_y == 0) {
                        unreachable!();
                    }
                    let ac: *mut int16_t = ((*t).scratch.c2rust_unnamed_0.ac)
                        .as_mut_ptr();
                    let mut y_src: *mut pixel = ((*f).cur.data[0 as libc::c_int as usize]
                        as *mut pixel)
                        .offset((4 * ((*t).bx & !ss_hor)) as isize)
                        .offset(
                            ((4 * ((*t).by & !ss_ver)) as isize
                                * PXSTRIDE((*f).cur.stride[0 as libc::c_int as usize]))
                                as isize,
                        );
                    let uv_off: ptrdiff_t = 4
                        * (((*t).bx >> ss_hor) as isize
                            + ((*t).by >> ss_ver) as isize * PXSTRIDE(stride));
                    let uv_dst: [*mut pixel; 2] = [
                        ((*f).cur.data[1 as libc::c_int as usize] as *mut pixel)
                            .offset(uv_off as isize),
                        ((*f).cur.data[2 as libc::c_int as usize] as *mut pixel)
                            .offset(uv_off as isize),
                    ];
                    let furthest_r: libc::c_int = (cw4 << ss_hor)
                        + (*t_dim).w as libc::c_int - 1 as libc::c_int
                        & !((*t_dim).w as libc::c_int - 1 as libc::c_int);
                    let furthest_b: libc::c_int = (ch4 << ss_ver)
                        + (*t_dim).h as libc::c_int - 1 as libc::c_int
                        & !((*t_dim).h as libc::c_int - 1 as libc::c_int);
                    ((*dsp)
                        .ipred
                        .cfl_ac[((*f).cur.p.layout as libc::c_uint)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as usize])
                        .expect(
                            "non-null function pointer",
                        )(
                        ac,
                        y_src,
                        (*f).cur.stride[0 as libc::c_int as usize],
                        cbw4 - (furthest_r >> ss_hor),
                        cbh4 - (furthest_b >> ss_ver),
                        cbw4 * 4 as libc::c_int,
                        cbh4 * 4 as libc::c_int,
                    );
                    let mut pl: libc::c_int = 0 as libc::c_int;
                    while pl < 2 as libc::c_int {
                        if !((*b).c2rust_unnamed.c2rust_unnamed.cfl_alpha[pl as usize]
                            == 0)
                        {
                            let mut angle_0: libc::c_int = 0 as libc::c_int;
                            let mut top_sb_edge_0: *const pixel = 0 as *const pixel;
                            if (*t).by & !ss_ver & (*f).sb_step - 1 as libc::c_int == 0 {
                                top_sb_edge_0 = (*f)
                                    .ipred_edge[(pl + 1 as libc::c_int) as usize];
                                let sby_0: libc::c_int = (*t).by >> (*f).sb_shift;
                                top_sb_edge_0 = top_sb_edge_0
                                    .offset(
                                        ((*f).sb128w * 128 as libc::c_int
                                            * (sby_0 - 1 as libc::c_int)) as isize,
                                    );
                            }
                            let xpos: libc::c_int = (*t).bx >> ss_hor;
                            let ypos: libc::c_int = (*t).by >> ss_ver;
                            let xstart: libc::c_int = (*ts).tiling.col_start >> ss_hor;
                            let ystart: libc::c_int = (*ts).tiling.row_start >> ss_ver;
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
                                .expect(
                                    "non-null function pointer",
                                )(
                                uv_dst[pl as usize],
                                stride,
                                edge,
                                (*uv_t_dim).w as libc::c_int * 4 as libc::c_int,
                                (*uv_t_dim).h as libc::c_int * 4 as libc::c_int,
                                ac,
                                (*b).c2rust_unnamed.c2rust_unnamed.cfl_alpha[pl as usize]
                                    as libc::c_int,
                                (*f).bitdepth_max,
                            );
                        }
                        pl += 1;
                    }
                    if 0 as libc::c_int != 0
                        && (*(*f).frame_hdr).frame_offset == 2 as libc::c_int
                        && (*t).by >= 0 as libc::c_int && (*t).by < 4 as libc::c_int
                        && (*t).bx >= 8 as libc::c_int && (*t).bx < 12 as libc::c_int
                        && 0 as libc::c_int != 0
                    {
                        ac_dump(
                            ac,
                            4 as libc::c_int * cbw4,
                            4 as libc::c_int * cbh4,
                            b"ac\0" as *const u8 as *const libc::c_char,
                        );
                        hex_dump(
                            uv_dst[0 as libc::c_int as usize],
                            stride,
                            cbw4 * 4 as libc::c_int,
                            cbh4 * 4 as libc::c_int,
                            b"u-cfl-pred\0" as *const u8 as *const libc::c_char,
                        );
                        hex_dump(
                            uv_dst[1 as libc::c_int as usize],
                            stride,
                            cbw4 * 4 as libc::c_int,
                            cbh4 * 4 as libc::c_int,
                            b"v-cfl-pred\0" as *const u8 as *const libc::c_char,
                        );
                    }
                } else if (*b)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .pal_sz[1 as libc::c_int as usize] != 0
                {
                    let uv_dstoff: ptrdiff_t = 4
                        * (((*t).bx >> ss_hor) as isize
                            + ((*t).by >> ss_ver) as isize
                                * PXSTRIDE((*f).cur.stride[1 as libc::c_int as usize]));
                    let mut pal_0: *const [uint16_t; 8] = 0 as *const [uint16_t; 8];
                    let mut pal_idx_0: *const uint8_t = 0 as *const uint8_t;
                    if (*t).frame_thread.pass != 0 {
                        let p_1: libc::c_int = (*t).frame_thread.pass & 1 as libc::c_int;
                        if ((*ts).frame_thread[p_1 as usize].pal_idx).is_null() {
                            unreachable!();
                        }
                        pal_0 = (*((*f).frame_thread.pal)
                            .offset(
                                ((((*t).by >> 1 as libc::c_int)
                                    + ((*t).bx & 1 as libc::c_int)) as isize
                                    * ((*f).b4_stride >> 1)
                                    + (((*t).bx as isize >> 1) as isize
                                    + ((*t).by as isize & 1)) as isize) as isize,
                            ))
                            .as_mut_ptr() as *const [uint16_t; 8];
                        pal_idx_0 = (*ts).frame_thread[p_1 as usize].pal_idx;
                        (*ts)
                            .frame_thread[p_1 as usize]
                            .pal_idx = ((*ts).frame_thread[p_1 as usize].pal_idx)
                            .offset((cbw4 * cbh4 * 16 as libc::c_int) as isize);
                    } else {
                        pal_0 = ((*t).scratch.c2rust_unnamed_0.pal).as_mut_ptr()
                            as *const [uint16_t; 8];
                        pal_idx_0 = &mut *((*t).scratch.c2rust_unnamed_0.pal_idx)
                            .as_mut_ptr()
                            .offset((bw4 * bh4 * 16 as libc::c_int) as isize)
                            as *mut uint8_t;
                    }
                    ((*(*f).dsp).ipred.pal_pred)
                        .expect(
                            "non-null function pointer",
                        )(
                        ((*f).cur.data[1 as libc::c_int as usize] as *mut pixel)
                            .offset(uv_dstoff as isize),
                        (*f).cur.stride[1 as libc::c_int as usize],
                        (*pal_0.offset(1 as libc::c_int as isize)).as_ptr(),
                        pal_idx_0,
                        cbw4 * 4 as libc::c_int,
                        cbh4 * 4 as libc::c_int,
                    );
                    ((*(*f).dsp).ipred.pal_pred)
                        .expect(
                            "non-null function pointer",
                        )(
                        ((*f).cur.data[2 as libc::c_int as usize] as *mut pixel)
                            .offset(uv_dstoff as isize),
                        (*f).cur.stride[1 as libc::c_int as usize],
                        (*pal_0.offset(2 as libc::c_int as isize)).as_ptr(),
                        pal_idx_0,
                        cbw4 * 4 as libc::c_int,
                        cbh4 * 4 as libc::c_int,
                    );
                    if 0 as libc::c_int != 0
                        && (*(*f).frame_hdr).frame_offset == 2 as libc::c_int
                        && (*t).by >= 0 as libc::c_int && (*t).by < 4 as libc::c_int
                        && (*t).bx >= 8 as libc::c_int && (*t).bx < 12 as libc::c_int
                        && 0 as libc::c_int != 0
                    {
                        hex_dump(
                            ((*f).cur.data[1 as libc::c_int as usize] as *mut pixel)
                                .offset(uv_dstoff as isize),
                            PXSTRIDE((*f).cur.stride[1 as libc::c_int as usize]),
                            cbw4 * 4 as libc::c_int,
                            cbh4 * 4 as libc::c_int,
                            b"u-pal-pred\0" as *const u8 as *const libc::c_char,
                        );
                        hex_dump(
                            ((*f).cur.data[2 as libc::c_int as usize] as *mut pixel)
                                .offset(uv_dstoff as isize),
                            PXSTRIDE((*f).cur.stride[1 as libc::c_int as usize]),
                            cbw4 * 4 as libc::c_int,
                            cbh4 * 4 as libc::c_int,
                            b"v-pal-pred\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
                let sm_uv_fl: libc::c_int = sm_uv_flag((*t).a, cbx4)
                    | sm_uv_flag(&mut (*t).l, cby4);
                let uv_sb_has_tr: libc::c_int = (if init_x + 16 as libc::c_int >> ss_hor
                    < cw4
                {
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
                let uv_sb_has_bl: libc::c_int = (if init_x != 0 {
                    0 as libc::c_int as libc::c_uint
                } else if init_y + 16 as libc::c_int >> ss_ver < ch4 {
                    1 as libc::c_int as libc::c_uint
                } else {
                    intra_edge_flags as libc::c_uint
                        & (EDGE_I420_LEFT_HAS_BOTTOM as libc::c_int
                            >> ((*f).cur.p.layout as libc::c_uint)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint))
                            as libc::c_uint
                }) as libc::c_int;
                let sub_cw4: libc::c_int = imin(
                    cw4,
                    init_x + 16 as libc::c_int >> ss_hor,
                );
                let mut pl_0: libc::c_int = 0 as libc::c_int;
                while pl_0 < 2 as libc::c_int {
                    y = init_y >> ss_ver;
                    (*t).by += init_y;
                    while y < sub_ch4 {
                        let mut dst_1: *mut pixel = ((*f)
                            .cur
                            .data[(1 as libc::c_int + pl_0) as usize] as *mut pixel)
                            .offset(
                                (4
                                    * (((*t).by >> ss_ver) as isize * PXSTRIDE(stride)
                                        + ((*t).bx + init_x >> ss_hor) as isize)) as isize,
                            );
                        x = init_x >> ss_hor;
                        (*t).bx += init_x;
                        while x < sub_cw4 {
                            let mut angle_1: libc::c_int = 0;
                            let mut edge_flags_0: EdgeFlags = 0 as EdgeFlags;
                            let mut top_sb_edge_1: *const pixel = 0 as *const pixel;
                            let mut uv_mode: IntraPredMode = DC_PRED;
                            let mut xpos_0: libc::c_int = 0;
                            let mut ypos_0: libc::c_int = 0;
                            let mut xstart_0: libc::c_int = 0;
                            let mut ystart_0: libc::c_int = 0;
                            let mut m_1: IntraPredMode = DC_PRED;
                            if !((*b).c2rust_unnamed.c2rust_unnamed.uv_mode
                                as libc::c_int == CFL_PRED as libc::c_int
                                && (*b)
                                    .c2rust_unnamed
                                    .c2rust_unnamed
                                    .cfl_alpha[pl_0 as usize] as libc::c_int != 0
                                || (*b)
                                    .c2rust_unnamed
                                    .c2rust_unnamed
                                    .pal_sz[1 as libc::c_int as usize] as libc::c_int != 0)
                            {
                                angle_1 = (*b).c2rust_unnamed.c2rust_unnamed.uv_angle
                                    as libc::c_int;
                                edge_flags_0 = ((if (y > init_y >> ss_ver
                                    || uv_sb_has_tr == 0)
                                    && x + (*uv_t_dim).w as libc::c_int >= sub_cw4
                                {
                                    0 as libc::c_int
                                } else {
                                    EDGE_I444_TOP_HAS_RIGHT as libc::c_int
                                })
                                    | (if x > init_x >> ss_hor
                                        || uv_sb_has_bl == 0
                                            && y + (*uv_t_dim).h as libc::c_int >= sub_ch4
                                    {
                                        0 as libc::c_int
                                    } else {
                                        EDGE_I444_LEFT_HAS_BOTTOM as libc::c_int
                                    })) as EdgeFlags;
                                top_sb_edge_1 = 0 as *const pixel;
                                if (*t).by & !ss_ver & (*f).sb_step - 1 as libc::c_int == 0
                                {
                                    top_sb_edge_1 = (*f)
                                        .ipred_edge[(1 as libc::c_int + pl_0) as usize];
                                    let sby_1: libc::c_int = (*t).by >> (*f).sb_shift;
                                    top_sb_edge_1 = top_sb_edge_1
                                        .offset(
                                            ((*f).sb128w * 128 as libc::c_int
                                                * (sby_1 - 1 as libc::c_int)) as isize,
                                        );
                                }
                                uv_mode = (if (*b).c2rust_unnamed.c2rust_unnamed.uv_mode
                                    as libc::c_int == CFL_PRED as libc::c_int
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
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    dst_1,
                                    stride,
                                    edge,
                                    (*uv_t_dim).w as libc::c_int * 4 as libc::c_int,
                                    (*uv_t_dim).h as libc::c_int * 4 as libc::c_int,
                                    angle_1 | sm_uv_fl,
                                    4 as libc::c_int * (*f).bw + ss_hor
                                        - 4 as libc::c_int * ((*t).bx & !ss_hor) >> ss_hor,
                                    4 as libc::c_int * (*f).bh + ss_ver
                                        - 4 as libc::c_int * ((*t).by & !ss_ver) >> ss_ver,
                                    (*f).bitdepth_max,
                                );
                                if 0 as libc::c_int != 0
                                    && (*(*f).frame_hdr).frame_offset == 2 as libc::c_int
                                    && (*t).by >= 0 as libc::c_int && (*t).by < 4 as libc::c_int
                                    && (*t).bx >= 8 as libc::c_int
                                    && (*t).bx < 12 as libc::c_int && 0 as libc::c_int != 0
                                {
                                    hex_dump(
                                        edge
                                            .offset(
                                                -(((*uv_t_dim).h as libc::c_int * 4 as libc::c_int)
                                                    as isize),
                                            ),
                                        ((*uv_t_dim).h as libc::c_int * 4 as libc::c_int)
                                            as ptrdiff_t,
                                        (*uv_t_dim).h as libc::c_int * 4 as libc::c_int,
                                        2 as libc::c_int,
                                        b"l\0" as *const u8 as *const libc::c_char,
                                    );
                                    hex_dump(
                                        edge,
                                        0 as libc::c_int as ptrdiff_t,
                                        1 as libc::c_int,
                                        1 as libc::c_int,
                                        b"tl\0" as *const u8 as *const libc::c_char,
                                    );
                                    hex_dump(
                                        edge.offset(1 as libc::c_int as isize),
                                        ((*uv_t_dim).w as libc::c_int * 4 as libc::c_int)
                                            as ptrdiff_t,
                                        (*uv_t_dim).w as libc::c_int * 4 as libc::c_int,
                                        2 as libc::c_int,
                                        b"t\0" as *const u8 as *const libc::c_char,
                                    );
                                    hex_dump(
                                        dst_1,
                                        stride,
                                        (*uv_t_dim).w as libc::c_int * 4 as libc::c_int,
                                        (*uv_t_dim).h as libc::c_int * 4 as libc::c_int,
                                        if pl_0 != 0 {
                                            b"v-intra-pred\0" as *const u8 as *const libc::c_char
                                        } else {
                                            b"u-intra-pred\0" as *const u8 as *const libc::c_char
                                        },
                                    );
                                }
                            }
                            if (*b).skip == 0 {
                                let mut txtp_0: TxfmType = DCT_DCT;
                                let mut eob_0: libc::c_int = 0;
                                let mut cf_0: *mut coef = 0 as *mut coef;
                                if (*t).frame_thread.pass != 0 {
                                    let p_2: libc::c_int = (*t).frame_thread.pass
                                        & 1 as libc::c_int;
                                    cf_0 = (*ts).frame_thread[p_2 as usize].cf;
                                    (*ts)
                                        .frame_thread[p_2 as usize]
                                        .cf = ((*ts).frame_thread[p_2 as usize].cf)
                                        .offset(
                                            ((*uv_t_dim).w as libc::c_int * (*uv_t_dim).h as libc::c_int
                                                * 16 as libc::c_int) as isize,
                                        );
                                    let cbi_0: *const CodedBlockInfo = &mut *((*f)
                                        .frame_thread
                                        .cbi)
                                        .offset(
                                            ((*t).by as isize * (*f).b4_stride
                                                + (*t).bx as isize) as isize,
                                        ) as *mut CodedBlockInfo;
                                    eob_0 = (*cbi_0).eob[(pl_0 + 1 as libc::c_int) as usize]
                                        as libc::c_int;
                                    txtp_0 = (*cbi_0).txtp[(pl_0 + 1 as libc::c_int) as usize]
                                        as TxfmType;
                                } else {
                                    let mut cf_ctx_0: uint8_t = 0;
                                    cf_0 = ((*t).c2rust_unnamed.cf_16bpc).as_mut_ptr();
                                    eob_0 = decode_coefs(
                                        t,
                                        &mut *(*((*(*t).a).ccoef)
                                            .as_mut_ptr()
                                            .offset(pl_0 as isize))
                                            .as_mut_ptr()
                                            .offset((cbx4 + x) as isize),
                                        &mut *(*((*t).l.ccoef).as_mut_ptr().offset(pl_0 as isize))
                                            .as_mut_ptr()
                                            .offset((cby4 + y) as isize),
                                        (*b).uvtx as RectTxfmSize,
                                        bs,
                                        b,
                                        1 as libc::c_int,
                                        1 as libc::c_int + pl_0,
                                        cf_0,
                                        &mut txtp_0,
                                        &mut cf_ctx_0,
                                    );
                                    if 0 as libc::c_int != 0
                                        && (*(*f).frame_hdr).frame_offset == 2 as libc::c_int
                                        && (*t).by >= 0 as libc::c_int && (*t).by < 4 as libc::c_int
                                        && (*t).bx >= 8 as libc::c_int
                                        && (*t).bx < 12 as libc::c_int
                                    {
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
                                    match imin(
                                        (*uv_t_dim).h as libc::c_int,
                                        (*f).bh - (*t).by + ss_ver >> ss_ver,
                                    ) {
                                        1 => {
                                            (*(&mut *(*((*t).l.ccoef)
                                                .as_mut_ptr()
                                                .offset(pl_0 as isize))
                                                .as_mut_ptr()
                                                .offset((cby4 + y) as isize) as *mut uint8_t
                                                as *mut alias8))
                                                .u8_0 = (0x1 as libc::c_int * cf_ctx_0 as libc::c_int)
                                                as uint8_t;
                                        }
                                        2 => {
                                            (*(&mut *(*((*t).l.ccoef)
                                                .as_mut_ptr()
                                                .offset(pl_0 as isize))
                                                .as_mut_ptr()
                                                .offset((cby4 + y) as isize) as *mut uint8_t
                                                as *mut alias16))
                                                .u16_0 = (0x101 as libc::c_int * cf_ctx_0 as libc::c_int)
                                                as uint16_t;
                                        }
                                        4 => {
                                            (*(&mut *(*((*t).l.ccoef)
                                                .as_mut_ptr()
                                                .offset(pl_0 as isize))
                                                .as_mut_ptr()
                                                .offset((cby4 + y) as isize) as *mut uint8_t
                                                as *mut alias32))
                                                .u32_0 = (0x1010101 as libc::c_uint)
                                                .wrapping_mul(cf_ctx_0 as libc::c_uint);
                                        }
                                        8 => {
                                            (*(&mut *(*((*t).l.ccoef)
                                                .as_mut_ptr()
                                                .offset(pl_0 as isize))
                                                .as_mut_ptr()
                                                .offset((cby4 + y) as isize) as *mut uint8_t
                                                as *mut alias64))
                                                .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                                                .wrapping_mul(cf_ctx_0 as libc::c_ulonglong) as uint64_t;
                                        }
                                        16 => {
                                            let const_val_3: uint64_t = (0x101010101010101
                                                as libc::c_ulonglong)
                                                .wrapping_mul(cf_ctx_0 as libc::c_ulonglong) as uint64_t;
                                            (*(&mut *(*((*t).l.ccoef)
                                                .as_mut_ptr()
                                                .offset(pl_0 as isize))
                                                .as_mut_ptr()
                                                .offset((cby4 + y + 0 as libc::c_int) as isize)
                                                as *mut uint8_t as *mut alias64))
                                                .u64_0 = const_val_3;
                                            (*(&mut *(*((*t).l.ccoef)
                                                .as_mut_ptr()
                                                .offset(pl_0 as isize))
                                                .as_mut_ptr()
                                                .offset((cby4 + y + 8 as libc::c_int) as isize)
                                                as *mut uint8_t as *mut alias64))
                                                .u64_0 = const_val_3;
                                        }
                                        _ => {
                                            memset(
                                                &mut *(*((*t).l.ccoef).as_mut_ptr().offset(pl_0 as isize))
                                                    .as_mut_ptr()
                                                    .offset((cby4 + y) as isize) as *mut uint8_t
                                                    as *mut libc::c_void,
                                                cf_ctx_0 as libc::c_int,
                                                imin(
                                                    (*uv_t_dim).h as libc::c_int,
                                                    (*f).bh - (*t).by + ss_ver >> ss_ver,
                                                ) as size_t,
                                            );
                                        }
                                    }
                                    match imin(
                                        (*uv_t_dim).w as libc::c_int,
                                        (*f).bw - (*t).bx + ss_hor >> ss_hor,
                                    ) {
                                        1 => {
                                            (*(&mut *(*((*(*t).a).ccoef)
                                                .as_mut_ptr()
                                                .offset(pl_0 as isize))
                                                .as_mut_ptr()
                                                .offset((cbx4 + x) as isize) as *mut uint8_t
                                                as *mut alias8))
                                                .u8_0 = (0x1 as libc::c_int * cf_ctx_0 as libc::c_int)
                                                as uint8_t;
                                        }
                                        2 => {
                                            (*(&mut *(*((*(*t).a).ccoef)
                                                .as_mut_ptr()
                                                .offset(pl_0 as isize))
                                                .as_mut_ptr()
                                                .offset((cbx4 + x) as isize) as *mut uint8_t
                                                as *mut alias16))
                                                .u16_0 = (0x101 as libc::c_int * cf_ctx_0 as libc::c_int)
                                                as uint16_t;
                                        }
                                        4 => {
                                            (*(&mut *(*((*(*t).a).ccoef)
                                                .as_mut_ptr()
                                                .offset(pl_0 as isize))
                                                .as_mut_ptr()
                                                .offset((cbx4 + x) as isize) as *mut uint8_t
                                                as *mut alias32))
                                                .u32_0 = (0x1010101 as libc::c_uint)
                                                .wrapping_mul(cf_ctx_0 as libc::c_uint);
                                        }
                                        8 => {
                                            (*(&mut *(*((*(*t).a).ccoef)
                                                .as_mut_ptr()
                                                .offset(pl_0 as isize))
                                                .as_mut_ptr()
                                                .offset((cbx4 + x) as isize) as *mut uint8_t
                                                as *mut alias64))
                                                .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                                                .wrapping_mul(cf_ctx_0 as libc::c_ulonglong) as uint64_t;
                                        }
                                        16 => {
                                            let const_val_4: uint64_t = (0x101010101010101
                                                as libc::c_ulonglong)
                                                .wrapping_mul(cf_ctx_0 as libc::c_ulonglong) as uint64_t;
                                            (*(&mut *(*((*(*t).a).ccoef)
                                                .as_mut_ptr()
                                                .offset(pl_0 as isize))
                                                .as_mut_ptr()
                                                .offset((cbx4 + x + 0 as libc::c_int) as isize)
                                                as *mut uint8_t as *mut alias64))
                                                .u64_0 = const_val_4;
                                            (*(&mut *(*((*(*t).a).ccoef)
                                                .as_mut_ptr()
                                                .offset(pl_0 as isize))
                                                .as_mut_ptr()
                                                .offset((cbx4 + x + 8 as libc::c_int) as isize)
                                                as *mut uint8_t as *mut alias64))
                                                .u64_0 = const_val_4;
                                        }
                                        _ => {
                                            memset(
                                                &mut *(*((*(*t).a).ccoef)
                                                    .as_mut_ptr()
                                                    .offset(pl_0 as isize))
                                                    .as_mut_ptr()
                                                    .offset((cbx4 + x) as isize) as *mut uint8_t
                                                    as *mut libc::c_void,
                                                cf_ctx_0 as libc::c_int,
                                                imin(
                                                    (*uv_t_dim).w as libc::c_int,
                                                    (*f).bw - (*t).bx + ss_hor >> ss_hor,
                                                ) as size_t,
                                            );
                                        }
                                    }
                                }
                                if eob_0 >= 0 as libc::c_int {
                                    if 0 as libc::c_int != 0
                                        && (*(*f).frame_hdr).frame_offset == 2 as libc::c_int
                                        && (*t).by >= 0 as libc::c_int && (*t).by < 4 as libc::c_int
                                        && (*t).bx >= 8 as libc::c_int
                                        && (*t).bx < 12 as libc::c_int && 0 as libc::c_int != 0
                                    {
                                        coef_dump(
                                            cf_0,
                                            (*uv_t_dim).h as libc::c_int * 4 as libc::c_int,
                                            (*uv_t_dim).w as libc::c_int * 4 as libc::c_int,
                                            3 as libc::c_int,
                                            b"dq\0" as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    ((*dsp).itx.itxfm_add[(*b).uvtx as usize][txtp_0 as usize])
                                        .expect(
                                            "non-null function pointer",
                                        )(dst_1, stride, cf_0, eob_0, (*f).bitdepth_max);
                                    if 0 as libc::c_int != 0
                                        && (*(*f).frame_hdr).frame_offset == 2 as libc::c_int
                                        && (*t).by >= 0 as libc::c_int && (*t).by < 4 as libc::c_int
                                        && (*t).bx >= 8 as libc::c_int
                                        && (*t).bx < 12 as libc::c_int && 0 as libc::c_int != 0
                                    {
                                        hex_dump(
                                            dst_1,
                                            stride,
                                            (*uv_t_dim).w as libc::c_int * 4 as libc::c_int,
                                            (*uv_t_dim).h as libc::c_int * 4 as libc::c_int,
                                            b"recon\0" as *const u8 as *const libc::c_char,
                                        );
                                    }
                                }
                            } else if (*t).frame_thread.pass == 0 {
                                match (*uv_t_dim).h as libc::c_int {
                                    1 => {
                                        (*(&mut *(*((*t).l.ccoef)
                                            .as_mut_ptr()
                                            .offset(pl_0 as isize))
                                            .as_mut_ptr()
                                            .offset((cby4 + y) as isize) as *mut uint8_t
                                            as *mut alias8))
                                            .u8_0 = (0x1 as libc::c_int * 0x40 as libc::c_int)
                                            as uint8_t;
                                    }
                                    2 => {
                                        (*(&mut *(*((*t).l.ccoef)
                                            .as_mut_ptr()
                                            .offset(pl_0 as isize))
                                            .as_mut_ptr()
                                            .offset((cby4 + y) as isize) as *mut uint8_t
                                            as *mut alias16))
                                            .u16_0 = (0x101 as libc::c_int * 0x40 as libc::c_int)
                                            as uint16_t;
                                    }
                                    4 => {
                                        (*(&mut *(*((*t).l.ccoef)
                                            .as_mut_ptr()
                                            .offset(pl_0 as isize))
                                            .as_mut_ptr()
                                            .offset((cby4 + y) as isize) as *mut uint8_t
                                            as *mut alias32))
                                            .u32_0 = (0x1010101 as libc::c_uint)
                                            .wrapping_mul(0x40 as libc::c_int as libc::c_uint);
                                    }
                                    8 => {
                                        (*(&mut *(*((*t).l.ccoef)
                                            .as_mut_ptr()
                                            .offset(pl_0 as isize))
                                            .as_mut_ptr()
                                            .offset((cby4 + y) as isize) as *mut uint8_t
                                            as *mut alias64))
                                            .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                                            .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                                            as uint64_t;
                                    }
                                    16 => {
                                        let const_val_5: uint64_t = (0x101010101010101
                                            as libc::c_ulonglong)
                                            .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                                            as uint64_t;
                                        (*(&mut *(*((*t).l.ccoef)
                                            .as_mut_ptr()
                                            .offset(pl_0 as isize))
                                            .as_mut_ptr()
                                            .offset((cby4 + y + 0 as libc::c_int) as isize)
                                            as *mut uint8_t as *mut alias64))
                                            .u64_0 = const_val_5;
                                        (*(&mut *(*((*t).l.ccoef)
                                            .as_mut_ptr()
                                            .offset(pl_0 as isize))
                                            .as_mut_ptr()
                                            .offset((cby4 + y + 8 as libc::c_int) as isize)
                                            as *mut uint8_t as *mut alias64))
                                            .u64_0 = const_val_5;
                                    }
                                    _ => {}
                                }
                                match (*uv_t_dim).w as libc::c_int {
                                    1 => {
                                        (*(&mut *(*((*(*t).a).ccoef)
                                            .as_mut_ptr()
                                            .offset(pl_0 as isize))
                                            .as_mut_ptr()
                                            .offset((cbx4 + x) as isize) as *mut uint8_t
                                            as *mut alias8))
                                            .u8_0 = (0x1 as libc::c_int * 0x40 as libc::c_int)
                                            as uint8_t;
                                    }
                                    2 => {
                                        (*(&mut *(*((*(*t).a).ccoef)
                                            .as_mut_ptr()
                                            .offset(pl_0 as isize))
                                            .as_mut_ptr()
                                            .offset((cbx4 + x) as isize) as *mut uint8_t
                                            as *mut alias16))
                                            .u16_0 = (0x101 as libc::c_int * 0x40 as libc::c_int)
                                            as uint16_t;
                                    }
                                    4 => {
                                        (*(&mut *(*((*(*t).a).ccoef)
                                            .as_mut_ptr()
                                            .offset(pl_0 as isize))
                                            .as_mut_ptr()
                                            .offset((cbx4 + x) as isize) as *mut uint8_t
                                            as *mut alias32))
                                            .u32_0 = (0x1010101 as libc::c_uint)
                                            .wrapping_mul(0x40 as libc::c_int as libc::c_uint);
                                    }
                                    8 => {
                                        (*(&mut *(*((*(*t).a).ccoef)
                                            .as_mut_ptr()
                                            .offset(pl_0 as isize))
                                            .as_mut_ptr()
                                            .offset((cbx4 + x) as isize) as *mut uint8_t
                                            as *mut alias64))
                                            .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                                            .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                                            as uint64_t;
                                    }
                                    16 => {
                                        let const_val_6: uint64_t = (0x101010101010101
                                            as libc::c_ulonglong)
                                            .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                                            as uint64_t;
                                        (*(&mut *(*((*(*t).a).ccoef)
                                            .as_mut_ptr()
                                            .offset(pl_0 as isize))
                                            .as_mut_ptr()
                                            .offset((cbx4 + x + 0 as libc::c_int) as isize)
                                            as *mut uint8_t as *mut alias64))
                                            .u64_0 = const_val_6;
                                        (*(&mut *(*((*(*t).a).ccoef)
                                            .as_mut_ptr()
                                            .offset(pl_0 as isize))
                                            .as_mut_ptr()
                                            .offset((cbx4 + x + 8 as libc::c_int) as isize)
                                            as *mut uint8_t as *mut alias64))
                                            .u64_0 = const_val_6;
                                    }
                                    _ => {}
                                }
                            }
                            dst_1 = dst_1
                                .offset(
                                    ((*uv_t_dim).w as libc::c_int * 4 as libc::c_int) as isize,
                                );
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
    let bx4: libc::c_int = (*t).bx & 31 as libc::c_int;
    let by4: libc::c_int = (*t).by & 31 as libc::c_int;
    let ss_ver: libc::c_int = ((*f).cur.p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_hor: libc::c_int = ((*f).cur.p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
    let cbx4: libc::c_int = bx4 >> ss_hor;
    let cby4: libc::c_int = by4 >> ss_ver;
    let b_dim: *const uint8_t = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bw4: libc::c_int = *b_dim.offset(0 as libc::c_int as isize) as libc::c_int;
    let bh4: libc::c_int = *b_dim.offset(1 as libc::c_int as isize) as libc::c_int;
    let w4: libc::c_int = imin(bw4, (*f).bw - (*t).bx);
    let h4: libc::c_int = imin(bh4, (*f).bh - (*t).by);
    let has_chroma: libc::c_int = ((*f).cur.p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint
        && (bw4 > ss_hor || (*t).bx & 1 as libc::c_int != 0)
        && (bh4 > ss_ver || (*t).by & 1 as libc::c_int != 0)) as libc::c_int;
    let chr_layout_idx: libc::c_int = (if (*f).cur.p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint
    {
        0 as libc::c_int as libc::c_uint
    } else {
        (DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint)
            .wrapping_sub((*f).cur.p.layout as libc::c_uint)
    }) as libc::c_int;
    let mut res: libc::c_int = 0;
    let cbh4: libc::c_int = bh4 + ss_ver >> ss_ver;
    let cbw4: libc::c_int = bw4 + ss_hor >> ss_hor;
    let mut dst: *mut pixel = ((*f).cur.data[0 as libc::c_int as usize] as *mut pixel)
        .offset(
            (4
                * ((*t).by as isize
                    * PXSTRIDE((*f).cur.stride[0 as libc::c_int as usize])
                    + (*t).bx as isize)) as isize,
        );
    let uvdstoff: ptrdiff_t = 4
        * (((*t).bx >> ss_hor) as isize
            + ((*t).by >> ss_ver) as isize
                * PXSTRIDE((*f).cur.stride[1 as libc::c_int as usize]));
    if (*(*f).frame_hdr).frame_type as libc::c_uint & 1 as libc::c_int as libc::c_uint
        == 0
    {
        if (*(*f).frame_hdr).super_res.enabled != 0 {
            unreachable!();
        }
        res = mc(
            t,
            dst,
            0 as *mut int16_t,
            (*f).cur.stride[0 as libc::c_int as usize],
            bw4,
            bh4,
            (*t).bx,
            (*t).by,
            0 as libc::c_int,
            (*b)
                .c2rust_unnamed
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c2rust_unnamed
                .mv[0 as libc::c_int as usize],
            &(*f).sr_cur,
            0 as libc::c_int,
            FILTER_2D_BILINEAR,
        );
        if res != 0 {
            return res;
        }
        if has_chroma != 0 {
            let mut pl: libc::c_int = 1 as libc::c_int;
            while pl < 3 as libc::c_int {
                res = mc(
                    t,
                    ((*f).cur.data[pl as usize] as *mut pixel).offset(uvdstoff as isize),
                    0 as *mut int16_t,
                    (*f).cur.stride[1 as libc::c_int as usize],
                    bw4 << (bw4 == ss_hor) as libc::c_int,
                    bh4 << (bh4 == ss_ver) as libc::c_int,
                    (*t).bx & !ss_hor,
                    (*t).by & !ss_ver,
                    pl,
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0 as libc::c_int as usize],
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
        let mut is_sub8x8: libc::c_int = 0;
        let mut r: *const *mut refmvs_block = 0 as *const *mut refmvs_block;
        let refp: *const Dav1dThreadPicture = &*((*f).refp)
            .as_ptr()
            .offset(
                *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                    .as_ptr()
                    .offset(0 as libc::c_int as isize) as isize,
            ) as *const Dav1dThreadPicture;
        let filter_2d: Filter2d = (*b).c2rust_unnamed.c2rust_unnamed_0.filter2d
            as Filter2d;
        if imin(bw4, bh4) > 1 as libc::c_int
            && ((*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_int
                == GLOBALMV as libc::c_int
                && (*f)
                    .gmv_warp_allowed[(*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .ref_0[0 as libc::c_int as usize] as usize] as libc::c_int != 0
                || (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode as libc::c_int
                    == MM_WARP as libc::c_int
                    && (*t).warpmv.type_0 as libc::c_uint
                        > DAV1D_WM_TYPE_TRANSLATION as libc::c_int as libc::c_uint)
        {
            res = warp_affine(
                t,
                dst,
                0 as *mut int16_t,
                (*f).cur.stride[0 as libc::c_int as usize],
                b_dim,
                0 as libc::c_int,
                refp,
                if (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode as libc::c_int
                    == MM_WARP as libc::c_int
                {
                    &mut (*t).warpmv
                } else {
                    &mut *((*(*f).frame_hdr).gmv)
                        .as_mut_ptr()
                        .offset(
                            *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                                .as_ptr()
                                .offset(0 as libc::c_int as isize) as isize,
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
                (*f).cur.stride[0 as libc::c_int as usize],
                bw4,
                bh4,
                (*t).bx,
                (*t).by,
                0 as libc::c_int,
                (*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .mv[0 as libc::c_int as usize],
                refp,
                (*b).c2rust_unnamed.c2rust_unnamed_0.ref_0[0 as libc::c_int as usize]
                    as libc::c_int,
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
                    (*f).cur.stride[0 as libc::c_int as usize],
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
                .offset(32 as libc::c_int as isize);
            let mut m: IntraPredMode = (if (*b)
                .c2rust_unnamed
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c2rust_unnamed
                .interintra_mode as libc::c_int == II_SMOOTH_PRED as libc::c_int
            {
                SMOOTH_PRED as libc::c_int
            } else {
                (*b)
                    .c2rust_unnamed
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
            let mut angle: libc::c_int = 0 as libc::c_int;
            let mut top_sb_edge: *const pixel = 0 as *const pixel;
            if (*t).by & (*f).sb_step - 1 as libc::c_int == 0 {
                top_sb_edge = (*f).ipred_edge[0 as libc::c_int as usize];
                let sby: libc::c_int = (*t).by >> (*f).sb_shift;
                top_sb_edge = top_sb_edge
                    .offset(
                        ((*f).sb128w * 128 as libc::c_int * (sby - 1 as libc::c_int))
                            as isize,
                    );
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
                (*f).cur.stride[0 as libc::c_int as usize],
                top_sb_edge,
                m,
                &mut angle,
                bw4,
                bh4,
                0 as libc::c_int,
                tl_edge,
                (*f).bitdepth_max,
            );
            ((*dsp).ipred.intra_pred[m as usize])
                .expect(
                    "non-null function pointer",
                )(
                tmp,
                ((4 as libc::c_int * bw4) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<pixel>() as libc::c_ulong)
                    as ptrdiff_t,
                tl_edge,
                bw4 * 4 as libc::c_int,
                bh4 * 4 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                (*f).bitdepth_max,
            );
            let ii_mask: *const uint8_t = if (*b)
                .c2rust_unnamed
                .c2rust_unnamed_0
                .interintra_type as libc::c_int == INTER_INTRA_BLEND as libc::c_int
            {
                dav1d_ii_masks[bs
                    as usize][0 as libc::c_int
                    as usize][(*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .interintra_mode as usize]
            } else {
                dav1d_wedge_masks[bs
                    as usize][0 as libc::c_int
                    as usize][0 as libc::c_int
                    as usize][(*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .wedge_idx as usize]
            };
            ((*dsp).mc.blend)
                .expect(
                    "non-null function pointer",
                )(
                dst,
                (*f).cur.stride[0 as libc::c_int as usize],
                tmp,
                bw4 * 4 as libc::c_int,
                bh4 * 4 as libc::c_int,
                ii_mask,
            );
        }
        if !(has_chroma == 0) {
            is_sub8x8 = (bw4 == ss_hor || bh4 == ss_ver) as libc::c_int;
            r = 0 as *const *mut refmvs_block;
            if is_sub8x8 != 0 {
                if !(ss_hor == 1 as libc::c_int) {
                    unreachable!();
                }
                r = &mut *((*t).rt.r)
                    .as_mut_ptr()
                    .offset((((*t).by & 31 as libc::c_int) + 5 as libc::c_int) as isize)
                    as *mut *mut refmvs_block;
                if bw4 == 1 as libc::c_int {
                    is_sub8x8
                        &= ((*(*r.offset(0 as libc::c_int as isize))
                            .offset(((*t).bx - 1 as libc::c_int) as isize))
                            .ref_0
                            .ref_0[0 as libc::c_int as usize] as libc::c_int
                            > 0 as libc::c_int) as libc::c_int;
                }
                if bh4 == ss_ver {
                    is_sub8x8
                        &= ((*(*r.offset(-(1 as libc::c_int) as isize))
                            .offset((*t).bx as isize))
                            .ref_0
                            .ref_0[0 as libc::c_int as usize] as libc::c_int
                            > 0 as libc::c_int) as libc::c_int;
                }
                if bw4 == 1 as libc::c_int && bh4 == ss_ver {
                    is_sub8x8
                        &= ((*(*r.offset(-(1 as libc::c_int) as isize))
                            .offset(((*t).bx - 1 as libc::c_int) as isize))
                            .ref_0
                            .ref_0[0 as libc::c_int as usize] as libc::c_int
                            > 0 as libc::c_int) as libc::c_int;
                }
            }
            if is_sub8x8 != 0 {
                if !(ss_hor == 1 as libc::c_int) {
                    unreachable!();
                }
                let mut h_off: ptrdiff_t = 0 as libc::c_int as ptrdiff_t;
                let mut v_off: ptrdiff_t = 0 as libc::c_int as ptrdiff_t;
                if bw4 == 1 as libc::c_int && bh4 == ss_ver {
                    let mut pl_0: libc::c_int = 0 as libc::c_int;
                    while pl_0 < 2 as libc::c_int {
                        res = mc(
                            t,
                            ((*f).cur.data[(1 as libc::c_int + pl_0) as usize]
                                as *mut pixel)
                                .offset(uvdstoff as isize),
                            0 as *mut int16_t,
                            (*f).cur.stride[1 as libc::c_int as usize],
                            bw4,
                            bh4,
                            (*t).bx - 1 as libc::c_int,
                            (*t).by - 1 as libc::c_int,
                            1 as libc::c_int + pl_0,
                            (*(*r.offset(-(1 as libc::c_int) as isize))
                                .offset(((*t).bx - 1 as libc::c_int) as isize))
                                .mv
                                .mv[0 as libc::c_int as usize],
                            &*((*f).refp)
                                .as_ptr()
                                .offset(
                                    (*((*(*r.offset(-(1 as libc::c_int) as isize))
                                        .offset(((*t).bx - 1 as libc::c_int) as isize))
                                        .ref_0
                                        .ref_0)
                                        .as_mut_ptr()
                                        .offset(0 as libc::c_int as isize) as libc::c_int
                                        - 1 as libc::c_int) as isize,
                                ),
                            (*(*r.offset(-(1 as libc::c_int) as isize))
                                .offset(((*t).bx - 1 as libc::c_int) as isize))
                                .ref_0
                                .ref_0[0 as libc::c_int as usize] as libc::c_int
                                - 1 as libc::c_int,
                            (if (*t).frame_thread.pass != 2 as libc::c_int {
                                (*t).tl_4x4_filter as libc::c_uint
                            } else {
                                (*((*f).frame_thread.b)
                                    .offset(
                                        (((*t).by - 1 as libc::c_int) as isize
                                            * (*f).b4_stride + (*t).bx as isize
                                            - 1) as isize,
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
                    v_off = 2 * PXSTRIDE((*f).cur.stride[1 as libc::c_int as usize]);
                    h_off = 2 as libc::c_int as ptrdiff_t;
                }
                if bw4 == 1 as libc::c_int {
                    let left_filter_2d: Filter2d = dav1d_filter_2d[(*t)
                        .l
                        .filter[1 as libc::c_int as usize][by4 as usize]
                        as usize][(*t).l.filter[0 as libc::c_int as usize][by4 as usize]
                        as usize] as Filter2d;
                    let mut pl_1: libc::c_int = 0 as libc::c_int;
                    while pl_1 < 2 as libc::c_int {
                        res = mc(
                            t,
                            ((*f).cur.data[(1 as libc::c_int + pl_1) as usize]
                                as *mut pixel)
                                .offset(uvdstoff as isize)
                                .offset(v_off as isize),
                            0 as *mut int16_t,
                            (*f).cur.stride[1 as libc::c_int as usize],
                            bw4,
                            bh4,
                            (*t).bx - 1 as libc::c_int,
                            (*t).by,
                            1 as libc::c_int + pl_1,
                            (*(*r.offset(0 as libc::c_int as isize))
                                .offset(((*t).bx - 1 as libc::c_int) as isize))
                                .mv
                                .mv[0 as libc::c_int as usize],
                            &*((*f).refp)
                                .as_ptr()
                                .offset(
                                    (*((*(*r.offset(0 as libc::c_int as isize))
                                        .offset(((*t).bx - 1 as libc::c_int) as isize))
                                        .ref_0
                                        .ref_0)
                                        .as_mut_ptr()
                                        .offset(0 as libc::c_int as isize) as libc::c_int
                                        - 1 as libc::c_int) as isize,
                                ),
                            (*(*r.offset(0 as libc::c_int as isize))
                                .offset(((*t).bx - 1 as libc::c_int) as isize))
                                .ref_0
                                .ref_0[0 as libc::c_int as usize] as libc::c_int
                                - 1 as libc::c_int,
                            (if (*t).frame_thread.pass != 2 as libc::c_int {
                                left_filter_2d as libc::c_uint
                            } else {
                                (*((*f).frame_thread.b)
                                    .offset(
                                        ((*t).by as isize * (*f).b4_stride
                                            + (*t).bx as isize
                                            - 1) as isize,
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
                    let top_filter_2d: Filter2d = dav1d_filter_2d[(*(*t).a)
                        .filter[1 as libc::c_int as usize][bx4 as usize]
                        as usize][(*(*t).a)
                        .filter[0 as libc::c_int as usize][bx4 as usize] as usize]
                        as Filter2d;
                    let mut pl_2: libc::c_int = 0 as libc::c_int;
                    while pl_2 < 2 as libc::c_int {
                        res = mc(
                            t,
                            ((*f).cur.data[(1 as libc::c_int + pl_2) as usize]
                                as *mut pixel)
                                .offset(uvdstoff as isize)
                                .offset(h_off as isize),
                            0 as *mut int16_t,
                            (*f).cur.stride[1 as libc::c_int as usize],
                            bw4,
                            bh4,
                            (*t).bx,
                            (*t).by - 1 as libc::c_int,
                            1 as libc::c_int + pl_2,
                            (*(*r.offset(-(1 as libc::c_int) as isize))
                                .offset((*t).bx as isize))
                                .mv
                                .mv[0 as libc::c_int as usize],
                            &*((*f).refp)
                                .as_ptr()
                                .offset(
                                    (*((*(*r.offset(-(1 as libc::c_int) as isize))
                                        .offset((*t).bx as isize))
                                        .ref_0
                                        .ref_0)
                                        .as_mut_ptr()
                                        .offset(0 as libc::c_int as isize) as libc::c_int
                                        - 1 as libc::c_int) as isize,
                                ),
                            (*(*r.offset(-(1 as libc::c_int) as isize))
                                .offset((*t).bx as isize))
                                .ref_0
                                .ref_0[0 as libc::c_int as usize] as libc::c_int
                                - 1 as libc::c_int,
                            (if (*t).frame_thread.pass != 2 as libc::c_int {
                                top_filter_2d as libc::c_uint
                            } else {
                                (*((*f).frame_thread.b)
                                    .offset(
                                        (((*t).by - 1 as libc::c_int) as isize
                                            * (*f).b4_stride + (*t).bx as isize) as isize,
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
                    v_off = 2 * PXSTRIDE((*f).cur.stride[1 as libc::c_int as usize]);
                }
                let mut pl_3: libc::c_int = 0 as libc::c_int;
                while pl_3 < 2 as libc::c_int {
                    res = mc(
                        t,
                        ((*f).cur.data[(1 as libc::c_int + pl_3) as usize] as *mut pixel)
                            .offset(uvdstoff as isize)
                            .offset(h_off as isize)
                            .offset(v_off as isize),
                        0 as *mut int16_t,
                        (*f).cur.stride[1 as libc::c_int as usize],
                        bw4,
                        bh4,
                        (*t).bx,
                        (*t).by,
                        1 as libc::c_int + pl_3,
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mv[0 as libc::c_int as usize],
                        refp,
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .ref_0[0 as libc::c_int as usize] as libc::c_int,
                        filter_2d,
                    );
                    if res != 0 {
                        return res;
                    }
                    pl_3 += 1;
                }
            } else {
                if imin(cbw4, cbh4) > 1 as libc::c_int
                    && ((*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_int
                        == GLOBALMV as libc::c_int
                        && (*f)
                            .gmv_warp_allowed[(*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .ref_0[0 as libc::c_int as usize] as usize] as libc::c_int
                            != 0
                        || (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode
                            as libc::c_int == MM_WARP as libc::c_int
                            && (*t).warpmv.type_0 as libc::c_uint
                                > DAV1D_WM_TYPE_TRANSLATION as libc::c_int as libc::c_uint)
                {
                    let mut pl_4: libc::c_int = 0 as libc::c_int;
                    while pl_4 < 2 as libc::c_int {
                        res = warp_affine(
                            t,
                            ((*f).cur.data[(1 as libc::c_int + pl_4) as usize]
                                as *mut pixel)
                                .offset(uvdstoff as isize),
                            0 as *mut int16_t,
                            (*f).cur.stride[1 as libc::c_int as usize],
                            b_dim,
                            1 as libc::c_int + pl_4,
                            refp,
                            if (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode
                                as libc::c_int == MM_WARP as libc::c_int
                            {
                                &mut (*t).warpmv
                            } else {
                                &mut *((*(*f).frame_hdr).gmv)
                                    .as_mut_ptr()
                                    .offset(
                                        *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                                            .as_ptr()
                                            .offset(0 as libc::c_int as isize) as isize,
                                    )
                            },
                        );
                        if res != 0 {
                            return res;
                        }
                        pl_4 += 1;
                    }
                } else {
                    let mut pl_5: libc::c_int = 0 as libc::c_int;
                    while pl_5 < 2 as libc::c_int {
                        res = mc(
                            t,
                            ((*f).cur.data[(1 as libc::c_int + pl_5) as usize]
                                as *mut pixel)
                                .offset(uvdstoff as isize),
                            0 as *mut int16_t,
                            (*f).cur.stride[1 as libc::c_int as usize],
                            bw4 << (bw4 == ss_hor) as libc::c_int,
                            bh4 << (bh4 == ss_ver) as libc::c_int,
                            (*t).bx & !ss_hor,
                            (*t).by & !ss_ver,
                            1 as libc::c_int + pl_5,
                            (*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .mv[0 as libc::c_int as usize],
                            refp,
                            (*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .ref_0[0 as libc::c_int as usize] as libc::c_int,
                            filter_2d,
                        );
                        if res != 0 {
                            return res;
                        }
                        if (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode
                            as libc::c_int == MM_OBMC as libc::c_int
                        {
                            res = obmc(
                                t,
                                ((*f).cur.data[(1 as libc::c_int + pl_5) as usize]
                                    as *mut pixel)
                                    .offset(uvdstoff as isize),
                                (*f).cur.stride[1 as libc::c_int as usize],
                                b_dim,
                                1 as libc::c_int + pl_5,
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
                    let ii_mask_0: *const uint8_t = if (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .interintra_type as libc::c_int
                        == INTER_INTRA_BLEND as libc::c_int
                    {
                        dav1d_ii_masks[bs
                            as usize][chr_layout_idx
                            as usize][(*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .interintra_mode as usize]
                    } else {
                        dav1d_wedge_masks[bs
                            as usize][chr_layout_idx
                            as usize][0 as libc::c_int
                            as usize][(*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .wedge_idx as usize]
                    };
                    let mut pl_6: libc::c_int = 0 as libc::c_int;
                    while pl_6 < 2 as libc::c_int {
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
                            .offset(32 as libc::c_int as isize);
                        let mut m_0: IntraPredMode = (if (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .interintra_mode as libc::c_int
                            == II_SMOOTH_PRED as libc::c_int
                        {
                            SMOOTH_PRED as libc::c_int
                        } else {
                            (*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .interintra_mode as libc::c_int
                        }) as IntraPredMode;
                        let mut angle_0: libc::c_int = 0 as libc::c_int;
                        let uvdst: *mut pixel = ((*f)
                            .cur
                            .data[(1 as libc::c_int + pl_6) as usize] as *mut pixel)
                            .offset(uvdstoff as isize);
                        let mut top_sb_edge_0: *const pixel = 0 as *const pixel;
                        if (*t).by & (*f).sb_step - 1 as libc::c_int == 0 {
                            top_sb_edge_0 = (*f)
                                .ipred_edge[(pl_6 + 1 as libc::c_int) as usize];
                            let sby_0: libc::c_int = (*t).by >> (*f).sb_shift;
                            top_sb_edge_0 = top_sb_edge_0
                                .offset(
                                    ((*f).sb128w * 128 as libc::c_int
                                        * (sby_0 - 1 as libc::c_int)) as isize,
                                );
                        }
                        m_0 = dav1d_prepare_intra_edges_16bpc(
                            (*t).bx >> ss_hor,
                            ((*t).bx >> ss_hor > (*ts).tiling.col_start >> ss_hor)
                                as libc::c_int,
                            (*t).by >> ss_ver,
                            ((*t).by >> ss_ver > (*ts).tiling.row_start >> ss_ver)
                                as libc::c_int,
                            (*ts).tiling.col_end >> ss_hor,
                            (*ts).tiling.row_end >> ss_ver,
                            0 as EdgeFlags,
                            uvdst,
                            (*f).cur.stride[1 as libc::c_int as usize],
                            top_sb_edge_0,
                            m_0,
                            &mut angle_0,
                            cbw4,
                            cbh4,
                            0 as libc::c_int,
                            tl_edge_0,
                            (*f).bitdepth_max,
                        );
                        ((*dsp).ipred.intra_pred[m_0 as usize])
                            .expect(
                                "non-null function pointer",
                            )(
                            tmp_0,
                            ((cbw4 * 4 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(
                                    ::core::mem::size_of::<pixel>() as libc::c_ulong,
                                ) as ptrdiff_t,
                            tl_edge_0,
                            cbw4 * 4 as libc::c_int,
                            cbh4 * 4 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            (*f).bitdepth_max,
                        );
                        ((*dsp).mc.blend)
                            .expect(
                                "non-null function pointer",
                            )(
                            uvdst,
                            (*f).cur.stride[1 as libc::c_int as usize],
                            tmp_0,
                            cbw4 * 4 as libc::c_int,
                            cbh4 * 4 as libc::c_int,
                            ii_mask_0,
                        );
                        pl_6 += 1;
                    }
                }
            }
        }
        (*t).tl_4x4_filter = filter_2d;
    } else {
        let filter_2d_0: Filter2d = (*b).c2rust_unnamed.c2rust_unnamed_0.filter2d
            as Filter2d;
        let mut tmp_1: *mut [int16_t; 16384] = ((*t)
            .scratch
            .c2rust_unnamed
            .c2rust_unnamed
            .c2rust_unnamed
            .compinter)
            .as_mut_ptr();
        let mut jnt_weight: libc::c_int = 0;
        let seg_mask: *mut uint8_t = ((*t)
            .scratch
            .c2rust_unnamed
            .c2rust_unnamed
            .c2rust_unnamed
            .seg_mask)
            .as_mut_ptr();
        let mut mask: *const uint8_t = 0 as *const uint8_t;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            let refp_0: *const Dav1dThreadPicture = &*((*f).refp)
                .as_ptr()
                .offset(
                    *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                        .as_ptr()
                        .offset(i as isize) as isize,
                ) as *const Dav1dThreadPicture;
            if (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_int
                == GLOBALMV_GLOBALMV as libc::c_int
                && (*f)
                    .gmv_warp_allowed[(*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .ref_0[i as usize] as usize] as libc::c_int != 0
            {
                res = warp_affine(
                    t,
                    0 as *mut pixel,
                    (*tmp_1.offset(i as isize)).as_mut_ptr(),
                    (bw4 * 4 as libc::c_int) as ptrdiff_t,
                    b_dim,
                    0 as libc::c_int,
                    refp_0,
                    &mut *((*(*f).frame_hdr).gmv)
                        .as_mut_ptr()
                        .offset(
                            *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
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
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[i as usize],
                    refp_0,
                    (*b).c2rust_unnamed.c2rust_unnamed_0.ref_0[i as usize]
                        as libc::c_int,
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
                ((*dsp).mc.avg)
                    .expect(
                        "non-null function pointer",
                    )(
                    dst,
                    (*f).cur.stride[0 as libc::c_int as usize],
                    (*tmp_1.offset(0 as libc::c_int as isize)).as_mut_ptr(),
                    (*tmp_1.offset(1 as libc::c_int as isize)).as_mut_ptr(),
                    bw4 * 4 as libc::c_int,
                    bh4 * 4 as libc::c_int,
                    (*f).bitdepth_max,
                );
            }
            1 => {
                jnt_weight = (*f)
                    .jnt_weights[(*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .ref_0[0 as libc::c_int as usize]
                    as usize][(*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .ref_0[1 as libc::c_int as usize] as usize] as libc::c_int;
                ((*dsp).mc.w_avg)
                    .expect(
                        "non-null function pointer",
                    )(
                    dst,
                    (*f).cur.stride[0 as libc::c_int as usize],
                    (*tmp_1.offset(0 as libc::c_int as isize)).as_mut_ptr(),
                    (*tmp_1.offset(1 as libc::c_int as isize)).as_mut_ptr(),
                    bw4 * 4 as libc::c_int,
                    bh4 * 4 as libc::c_int,
                    jnt_weight,
                    (*f).bitdepth_max,
                );
            }
            3 => {
                ((*dsp).mc.w_mask[chr_layout_idx as usize])
                    .expect(
                        "non-null function pointer",
                    )(
                    dst,
                    (*f).cur.stride[0 as libc::c_int as usize],
                    (*tmp_1
                        .offset(
                            (*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .mask_sign as isize,
                        ))
                        .as_mut_ptr(),
                    (*tmp_1
                        .offset(
                            ((*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .mask_sign == 0) as libc::c_int as isize,
                        ))
                        .as_mut_ptr(),
                    bw4 * 4 as libc::c_int,
                    bh4 * 4 as libc::c_int,
                    seg_mask,
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mask_sign as libc::c_int,
                    (*f).bitdepth_max,
                );
                mask = seg_mask;
            }
            4 => {
                mask = dav1d_wedge_masks[bs
                    as usize][0 as libc::c_int
                    as usize][0 as libc::c_int
                    as usize][(*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .wedge_idx as usize];
                ((*dsp).mc.mask)
                    .expect(
                        "non-null function pointer",
                    )(
                    dst,
                    (*f).cur.stride[0 as libc::c_int as usize],
                    (*tmp_1
                        .offset(
                            (*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .mask_sign as isize,
                        ))
                        .as_mut_ptr(),
                    (*tmp_1
                        .offset(
                            ((*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .mask_sign == 0) as libc::c_int as isize,
                        ))
                        .as_mut_ptr(),
                    bw4 * 4 as libc::c_int,
                    bh4 * 4 as libc::c_int,
                    mask,
                    (*f).bitdepth_max,
                );
                if has_chroma != 0 {
                    mask = dav1d_wedge_masks[bs
                        as usize][chr_layout_idx
                        as usize][(*b)
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
            let mut pl_7: libc::c_int = 0 as libc::c_int;
            while pl_7 < 2 as libc::c_int {
                let mut i_0: libc::c_int = 0 as libc::c_int;
                while i_0 < 2 as libc::c_int {
                    let refp_1: *const Dav1dThreadPicture = &*((*f).refp)
                        .as_ptr()
                        .offset(
                            *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                                .as_ptr()
                                .offset(i_0 as isize) as isize,
                        ) as *const Dav1dThreadPicture;
                    if (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_int
                        == GLOBALMV_GLOBALMV as libc::c_int
                        && imin(cbw4, cbh4) > 1 as libc::c_int
                        && (*f)
                            .gmv_warp_allowed[(*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .ref_0[i_0 as usize] as usize] as libc::c_int != 0
                    {
                        res = warp_affine(
                            t,
                            0 as *mut pixel,
                            (*tmp_1.offset(i_0 as isize)).as_mut_ptr(),
                            (bw4 * 4 as libc::c_int >> ss_hor) as ptrdiff_t,
                            b_dim,
                            1 as libc::c_int + pl_7,
                            refp_1,
                            &mut *((*(*f).frame_hdr).gmv)
                                .as_mut_ptr()
                                .offset(
                                    *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
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
                            1 as libc::c_int + pl_7,
                            (*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .mv[i_0 as usize],
                            refp_1,
                            (*b).c2rust_unnamed.c2rust_unnamed_0.ref_0[i_0 as usize]
                                as libc::c_int,
                            filter_2d_0,
                        );
                        if res != 0 {
                            return res;
                        }
                    }
                    i_0 += 1;
                }
                let uvdst_0: *mut pixel = ((*f)
                    .cur
                    .data[(1 as libc::c_int + pl_7) as usize] as *mut pixel)
                    .offset(uvdstoff as isize);
                match (*b).c2rust_unnamed.c2rust_unnamed_0.comp_type as libc::c_int {
                    2 => {
                        ((*dsp).mc.avg)
                            .expect(
                                "non-null function pointer",
                            )(
                            uvdst_0,
                            (*f).cur.stride[1 as libc::c_int as usize],
                            (*tmp_1.offset(0 as libc::c_int as isize)).as_mut_ptr(),
                            (*tmp_1.offset(1 as libc::c_int as isize)).as_mut_ptr(),
                            bw4 * 4 as libc::c_int >> ss_hor,
                            bh4 * 4 as libc::c_int >> ss_ver,
                            (*f).bitdepth_max,
                        );
                    }
                    1 => {
                        ((*dsp).mc.w_avg)
                            .expect(
                                "non-null function pointer",
                            )(
                            uvdst_0,
                            (*f).cur.stride[1 as libc::c_int as usize],
                            (*tmp_1.offset(0 as libc::c_int as isize)).as_mut_ptr(),
                            (*tmp_1.offset(1 as libc::c_int as isize)).as_mut_ptr(),
                            bw4 * 4 as libc::c_int >> ss_hor,
                            bh4 * 4 as libc::c_int >> ss_ver,
                            jnt_weight,
                            (*f).bitdepth_max,
                        );
                    }
                    4 | 3 => {
                        ((*dsp).mc.mask)
                            .expect(
                                "non-null function pointer",
                            )(
                            uvdst_0,
                            (*f).cur.stride[1 as libc::c_int as usize],
                            (*tmp_1
                                .offset(
                                    (*b)
                                        .c2rust_unnamed
                                        .c2rust_unnamed_0
                                        .c2rust_unnamed
                                        .c2rust_unnamed
                                        .mask_sign as isize,
                                ))
                                .as_mut_ptr(),
                            (*tmp_1
                                .offset(
                                    ((*b)
                                        .c2rust_unnamed
                                        .c2rust_unnamed_0
                                        .c2rust_unnamed
                                        .c2rust_unnamed
                                        .mask_sign == 0) as libc::c_int as isize,
                                ))
                                .as_mut_ptr(),
                            bw4 * 4 as libc::c_int >> ss_hor,
                            bh4 * 4 as libc::c_int >> ss_ver,
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
    if 0 as libc::c_int != 0 && (*(*f).frame_hdr).frame_offset == 2 as libc::c_int
        && (*t).by >= 0 as libc::c_int && (*t).by < 4 as libc::c_int
        && (*t).bx >= 8 as libc::c_int && (*t).bx < 12 as libc::c_int
        && 0 as libc::c_int != 0
    {
        hex_dump(
            dst,
            (*f).cur.stride[0 as libc::c_int as usize],
            *b_dim.offset(0 as libc::c_int as isize) as libc::c_int * 4 as libc::c_int,
            *b_dim.offset(1 as libc::c_int as isize) as libc::c_int * 4 as libc::c_int,
            b"y-pred\0" as *const u8 as *const libc::c_char,
        );
        if has_chroma != 0 {
            hex_dump(
                &mut *(*((*f).cur.data).as_ptr().offset(1 as libc::c_int as isize)
                    as *mut pixel)
                    .offset(uvdstoff as isize),
                (*f).cur.stride[1 as libc::c_int as usize],
                cbw4 * 4 as libc::c_int,
                cbh4 * 4 as libc::c_int,
                b"u-pred\0" as *const u8 as *const libc::c_char,
            );
            hex_dump(
                &mut *(*((*f).cur.data).as_ptr().offset(2 as libc::c_int as isize)
                    as *mut pixel)
                    .offset(uvdstoff as isize),
                (*f).cur.stride[1 as libc::c_int as usize],
                cbw4 * 4 as libc::c_int,
                cbh4 * 4 as libc::c_int,
                b"v-pred\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    let cw4: libc::c_int = w4 + ss_hor >> ss_hor;
    let ch4: libc::c_int = h4 + ss_ver >> ss_ver;
    if (*b).skip != 0 {
        match bh4 {
            1 => {
                (*(&mut *((*t).l.lcoef).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias8))
                    .u8_0 = (0x1 as libc::c_int * 0x40 as libc::c_int) as uint8_t;
            }
            2 => {
                (*(&mut *((*t).l.lcoef).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias16))
                    .u16_0 = (0x101 as libc::c_int * 0x40 as libc::c_int) as uint16_t;
            }
            4 => {
                (*(&mut *((*t).l.lcoef).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint)
                    .wrapping_mul(0x40 as libc::c_int as libc::c_uint);
            }
            8 => {
                (*(&mut *((*t).l.lcoef).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong) as uint64_t;
            }
            16 => {
                let const_val: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong) as uint64_t;
                (*(&mut *((*t).l.lcoef)
                    .as_mut_ptr()
                    .offset((by4 + 0 as libc::c_int) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val;
                (*(&mut *((*t).l.lcoef)
                    .as_mut_ptr()
                    .offset((by4 + 8 as libc::c_int) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val;
            }
            32 => {
                let const_val_0: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong) as uint64_t;
                (*(&mut *((*t).l.lcoef)
                    .as_mut_ptr()
                    .offset((by4 + 0 as libc::c_int) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_0;
                (*(&mut *((*t).l.lcoef)
                    .as_mut_ptr()
                    .offset((by4 + 8 as libc::c_int) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_0;
                (*(&mut *((*t).l.lcoef)
                    .as_mut_ptr()
                    .offset((by4 + 16 as libc::c_int) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_0;
                (*(&mut *((*t).l.lcoef)
                    .as_mut_ptr()
                    .offset((by4 + 24 as libc::c_int) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_0;
            }
            _ => {}
        }
        match bw4 {
            1 => {
                (*(&mut *((*(*t).a).lcoef).as_mut_ptr().offset(bx4 as isize)
                    as *mut uint8_t as *mut alias8))
                    .u8_0 = (0x1 as libc::c_int * 0x40 as libc::c_int) as uint8_t;
            }
            2 => {
                (*(&mut *((*(*t).a).lcoef).as_mut_ptr().offset(bx4 as isize)
                    as *mut uint8_t as *mut alias16))
                    .u16_0 = (0x101 as libc::c_int * 0x40 as libc::c_int) as uint16_t;
            }
            4 => {
                (*(&mut *((*(*t).a).lcoef).as_mut_ptr().offset(bx4 as isize)
                    as *mut uint8_t as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint)
                    .wrapping_mul(0x40 as libc::c_int as libc::c_uint);
            }
            8 => {
                (*(&mut *((*(*t).a).lcoef).as_mut_ptr().offset(bx4 as isize)
                    as *mut uint8_t as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong) as uint64_t;
            }
            16 => {
                let const_val_1: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong) as uint64_t;
                (*(&mut *((*(*t).a).lcoef)
                    .as_mut_ptr()
                    .offset((bx4 + 0 as libc::c_int) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_1;
                (*(&mut *((*(*t).a).lcoef)
                    .as_mut_ptr()
                    .offset((bx4 + 8 as libc::c_int) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_1;
            }
            32 => {
                let const_val_2: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong) as uint64_t;
                (*(&mut *((*(*t).a).lcoef)
                    .as_mut_ptr()
                    .offset((bx4 + 0 as libc::c_int) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_2;
                (*(&mut *((*(*t).a).lcoef)
                    .as_mut_ptr()
                    .offset((bx4 + 8 as libc::c_int) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_2;
                (*(&mut *((*(*t).a).lcoef)
                    .as_mut_ptr()
                    .offset((bx4 + 16 as libc::c_int) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_2;
                (*(&mut *((*(*t).a).lcoef)
                    .as_mut_ptr()
                    .offset((bx4 + 24 as libc::c_int) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_2;
            }
            _ => {}
        }
        if has_chroma != 0 {
            match cbh4 {
                1 => {
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cby4 as isize) as *mut uint8_t as *mut alias8))
                        .u8_0 = (0x1 as libc::c_int * 0x40 as libc::c_int) as uint8_t;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cby4 as isize) as *mut uint8_t as *mut alias8))
                        .u8_0 = (0x1 as libc::c_int * 0x40 as libc::c_int) as uint8_t;
                }
                2 => {
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cby4 as isize) as *mut uint8_t as *mut alias16))
                        .u16_0 = (0x101 as libc::c_int * 0x40 as libc::c_int)
                        as uint16_t;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cby4 as isize) as *mut uint8_t as *mut alias16))
                        .u16_0 = (0x101 as libc::c_int * 0x40 as libc::c_int)
                        as uint16_t;
                }
                4 => {
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cby4 as isize) as *mut uint8_t as *mut alias32))
                        .u32_0 = (0x1010101 as libc::c_uint)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_uint);
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cby4 as isize) as *mut uint8_t as *mut alias32))
                        .u32_0 = (0x1010101 as libc::c_uint)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_uint);
                }
                8 => {
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cby4 as isize) as *mut uint8_t as *mut alias64))
                        .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cby4 as isize) as *mut uint8_t as *mut alias64))
                        .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                }
                16 => {
                    let const_val_3: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cby4 + 0 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_3;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cby4 + 8 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_3;
                    let const_val_4: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cby4 + 0 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_4;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cby4 + 8 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_4;
                }
                32 => {
                    let const_val_5: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cby4 + 0 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_5;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cby4 + 8 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_5;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cby4 + 16 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_5;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cby4 + 24 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_5;
                    let const_val_6: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cby4 + 0 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_6;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cby4 + 8 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_6;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cby4 + 16 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_6;
                    (*(&mut *(*((*t).l.ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cby4 + 24 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_6;
                }
                _ => {}
            }
            match cbw4 {
                1 => {
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cbx4 as isize) as *mut uint8_t as *mut alias8))
                        .u8_0 = (0x1 as libc::c_int * 0x40 as libc::c_int) as uint8_t;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cbx4 as isize) as *mut uint8_t as *mut alias8))
                        .u8_0 = (0x1 as libc::c_int * 0x40 as libc::c_int) as uint8_t;
                }
                2 => {
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cbx4 as isize) as *mut uint8_t as *mut alias16))
                        .u16_0 = (0x101 as libc::c_int * 0x40 as libc::c_int)
                        as uint16_t;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cbx4 as isize) as *mut uint8_t as *mut alias16))
                        .u16_0 = (0x101 as libc::c_int * 0x40 as libc::c_int)
                        as uint16_t;
                }
                4 => {
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cbx4 as isize) as *mut uint8_t as *mut alias32))
                        .u32_0 = (0x1010101 as libc::c_uint)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_uint);
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cbx4 as isize) as *mut uint8_t as *mut alias32))
                        .u32_0 = (0x1010101 as libc::c_uint)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_uint);
                }
                8 => {
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cbx4 as isize) as *mut uint8_t as *mut alias64))
                        .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(cbx4 as isize) as *mut uint8_t as *mut alias64))
                        .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                }
                16 => {
                    let const_val_7: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cbx4 + 0 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_7;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cbx4 + 8 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_7;
                    let const_val_8: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cbx4 + 0 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_8;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cbx4 + 8 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_8;
                }
                32 => {
                    let const_val_9: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cbx4 + 0 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_9;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cbx4 + 8 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_9;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cbx4 + 16 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_9;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cbx4 + 24 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_9;
                    let const_val_10: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(0x40 as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cbx4 + 0 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_10;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cbx4 + 8 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_10;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cbx4 + 16 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_10;
                    (*(&mut *(*((*(*t).a).ccoef)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((cbx4 + 24 as libc::c_int) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_10;
                }
                _ => {}
            }
        }
        return 0 as libc::c_int;
    }
    let uvtx: *const TxfmInfo = &*dav1d_txfm_dimensions
        .as_ptr()
        .offset((*b).uvtx as isize) as *const TxfmInfo;
    let ytx: *const TxfmInfo = &*dav1d_txfm_dimensions
        .as_ptr()
        .offset((*b).c2rust_unnamed.c2rust_unnamed_0.max_ytx as isize)
        as *const TxfmInfo;
    let tx_split: [uint16_t; 2] = [
        (*b).c2rust_unnamed.c2rust_unnamed_0.tx_split0 as uint16_t,
        (*b).c2rust_unnamed.c2rust_unnamed_0.tx_split1,
    ];
    let mut init_y: libc::c_int = 0 as libc::c_int;
    while init_y < bh4 {
        let mut init_x: libc::c_int = 0 as libc::c_int;
        while init_x < bw4 {
            let mut y_off: libc::c_int = (init_y != 0) as libc::c_int;
            let mut y: libc::c_int = 0;
            dst = dst
                .offset(
                    (PXSTRIDE((*f).cur.stride[0 as libc::c_int as usize])
                        * 4 * init_y as isize)
                        as isize,
                );
            y = init_y;
            (*t).by += init_y;
            while y < imin(h4, init_y + 16 as libc::c_int) {
                let mut x: libc::c_int = 0;
                let mut x_off: libc::c_int = (init_x != 0) as libc::c_int;
                x = init_x;
                (*t).bx += init_x;
                while x < imin(w4, init_x + 16 as libc::c_int) {
                    read_coef_tree(
                        t,
                        bs,
                        b,
                        (*b).c2rust_unnamed.c2rust_unnamed_0.max_ytx as RectTxfmSize,
                        0 as libc::c_int,
                        tx_split.as_ptr(),
                        x_off,
                        y_off,
                        &mut *dst.offset((x * 4 as libc::c_int) as isize),
                    );
                    (*t).bx += (*ytx).w as libc::c_int;
                    x += (*ytx).w as libc::c_int;
                    x_off += 1;
                }
                dst = dst
                    .offset(
                        (PXSTRIDE((*f).cur.stride[0 as libc::c_int as usize])
                            * 4
                            * (*ytx).h as isize) as isize,
                    );
                (*t).bx -= x;
                (*t).by += (*ytx).h as libc::c_int;
                y += (*ytx).h as libc::c_int;
                y_off += 1;
            }
            dst = dst
                .offset(
                    -((PXSTRIDE((*f).cur.stride[0 as libc::c_int as usize])
                        * 4 * y as isize) as isize),
                );
            (*t).by -= y;
            if has_chroma != 0 {
                let mut pl_8: libc::c_int = 0 as libc::c_int;
                while pl_8 < 2 as libc::c_int {
                    let mut uvdst_1: *mut pixel = ((*f)
                        .cur
                        .data[(1 as libc::c_int + pl_8) as usize] as *mut pixel)
                        .offset(uvdstoff as isize)
                        .offset(
                            (PXSTRIDE((*f).cur.stride[1 as libc::c_int as usize])
                                * init_y as isize * 4 >> ss_ver) as isize,
                        );
                    y = init_y >> ss_ver;
                    (*t).by += init_y;
                    while y < imin(ch4, init_y + 16 as libc::c_int >> ss_ver) {
                        let mut x_0: libc::c_int = 0;
                        x_0 = init_x >> ss_hor;
                        (*t).bx += init_x;
                        while x_0 < imin(cw4, init_x + 16 as libc::c_int >> ss_hor) {
                            let mut cf: *mut coef = 0 as *mut coef;
                            let mut eob: libc::c_int = 0;
                            let mut txtp: TxfmType = DCT_DCT;
                            if (*t).frame_thread.pass != 0 {
                                let p: libc::c_int = (*t).frame_thread.pass
                                    & 1 as libc::c_int;
                                cf = (*ts).frame_thread[p as usize].cf;
                                (*ts)
                                    .frame_thread[p as usize]
                                    .cf = ((*ts).frame_thread[p as usize].cf)
                                    .offset(
                                        ((*uvtx).w as libc::c_int * (*uvtx).h as libc::c_int
                                            * 16 as libc::c_int) as isize,
                                    );
                                let cbi: *const CodedBlockInfo = &mut *((*f)
                                    .frame_thread
                                    .cbi)
                                    .offset(
                                        ((*t).by as isize * (*f).b4_stride
                                            + (*t).bx as isize) as isize,
                                    ) as *mut CodedBlockInfo;
                                eob = (*cbi).eob[(1 as libc::c_int + pl_8) as usize]
                                    as libc::c_int;
                                txtp = (*cbi).txtp[(1 as libc::c_int + pl_8) as usize]
                                    as TxfmType;
                            } else {
                                let mut cf_ctx: uint8_t = 0;
                                cf = ((*t).c2rust_unnamed.cf_16bpc).as_mut_ptr();
                                txtp = (*t)
                                    .txtp_map[((by4 + (y << ss_ver)) * 32 as libc::c_int + bx4
                                    + (x_0 << ss_hor)) as usize] as TxfmType;
                                eob = decode_coefs(
                                    t,
                                    &mut *(*((*(*t).a).ccoef)
                                        .as_mut_ptr()
                                        .offset(pl_8 as isize))
                                        .as_mut_ptr()
                                        .offset((cbx4 + x_0) as isize),
                                    &mut *(*((*t).l.ccoef).as_mut_ptr().offset(pl_8 as isize))
                                        .as_mut_ptr()
                                        .offset((cby4 + y) as isize),
                                    (*b).uvtx as RectTxfmSize,
                                    bs,
                                    b,
                                    0 as libc::c_int,
                                    1 as libc::c_int + pl_8,
                                    cf,
                                    &mut txtp,
                                    &mut cf_ctx,
                                );
                                if 0 as libc::c_int != 0
                                    && (*(*f).frame_hdr).frame_offset == 2 as libc::c_int
                                    && (*t).by >= 0 as libc::c_int && (*t).by < 4 as libc::c_int
                                    && (*t).bx >= 8 as libc::c_int
                                    && (*t).bx < 12 as libc::c_int
                                {
                                    printf(
                                        b"Post-uv-cf-blk[pl=%d,tx=%d,txtp=%d,eob=%d]: r=%d\n\0"
                                            as *const u8 as *const libc::c_char,
                                        pl_8,
                                        (*b).uvtx as libc::c_int,
                                        txtp as libc::c_uint,
                                        eob,
                                        (*ts).msac.rng,
                                    );
                                }
                                match imin(
                                    (*uvtx).h as libc::c_int,
                                    (*f).bh - (*t).by + ss_ver >> ss_ver,
                                ) {
                                    1 => {
                                        (*(&mut *(*((*t).l.ccoef)
                                            .as_mut_ptr()
                                            .offset(pl_8 as isize))
                                            .as_mut_ptr()
                                            .offset((cby4 + y) as isize) as *mut uint8_t
                                            as *mut alias8))
                                            .u8_0 = (0x1 as libc::c_int * cf_ctx as libc::c_int)
                                            as uint8_t;
                                    }
                                    2 => {
                                        (*(&mut *(*((*t).l.ccoef)
                                            .as_mut_ptr()
                                            .offset(pl_8 as isize))
                                            .as_mut_ptr()
                                            .offset((cby4 + y) as isize) as *mut uint8_t
                                            as *mut alias16))
                                            .u16_0 = (0x101 as libc::c_int * cf_ctx as libc::c_int)
                                            as uint16_t;
                                    }
                                    4 => {
                                        (*(&mut *(*((*t).l.ccoef)
                                            .as_mut_ptr()
                                            .offset(pl_8 as isize))
                                            .as_mut_ptr()
                                            .offset((cby4 + y) as isize) as *mut uint8_t
                                            as *mut alias32))
                                            .u32_0 = (0x1010101 as libc::c_uint)
                                            .wrapping_mul(cf_ctx as libc::c_uint);
                                    }
                                    8 => {
                                        (*(&mut *(*((*t).l.ccoef)
                                            .as_mut_ptr()
                                            .offset(pl_8 as isize))
                                            .as_mut_ptr()
                                            .offset((cby4 + y) as isize) as *mut uint8_t
                                            as *mut alias64))
                                            .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                                            .wrapping_mul(cf_ctx as libc::c_ulonglong) as uint64_t;
                                    }
                                    16 => {
                                        let const_val_11: uint64_t = (0x101010101010101
                                            as libc::c_ulonglong)
                                            .wrapping_mul(cf_ctx as libc::c_ulonglong) as uint64_t;
                                        (*(&mut *(*((*t).l.ccoef)
                                            .as_mut_ptr()
                                            .offset(pl_8 as isize))
                                            .as_mut_ptr()
                                            .offset((cby4 + y + 0 as libc::c_int) as isize)
                                            as *mut uint8_t as *mut alias64))
                                            .u64_0 = const_val_11;
                                        (*(&mut *(*((*t).l.ccoef)
                                            .as_mut_ptr()
                                            .offset(pl_8 as isize))
                                            .as_mut_ptr()
                                            .offset((cby4 + y + 8 as libc::c_int) as isize)
                                            as *mut uint8_t as *mut alias64))
                                            .u64_0 = const_val_11;
                                    }
                                    _ => {
                                        memset(
                                            &mut *(*((*t).l.ccoef).as_mut_ptr().offset(pl_8 as isize))
                                                .as_mut_ptr()
                                                .offset((cby4 + y) as isize) as *mut uint8_t
                                                as *mut libc::c_void,
                                            cf_ctx as libc::c_int,
                                            imin(
                                                (*uvtx).h as libc::c_int,
                                                (*f).bh - (*t).by + ss_ver >> ss_ver,
                                            ) as size_t,
                                        );
                                    }
                                }
                                match imin(
                                    (*uvtx).w as libc::c_int,
                                    (*f).bw - (*t).bx + ss_hor >> ss_hor,
                                ) {
                                    1 => {
                                        (*(&mut *(*((*(*t).a).ccoef)
                                            .as_mut_ptr()
                                            .offset(pl_8 as isize))
                                            .as_mut_ptr()
                                            .offset((cbx4 + x_0) as isize) as *mut uint8_t
                                            as *mut alias8))
                                            .u8_0 = (0x1 as libc::c_int * cf_ctx as libc::c_int)
                                            as uint8_t;
                                    }
                                    2 => {
                                        (*(&mut *(*((*(*t).a).ccoef)
                                            .as_mut_ptr()
                                            .offset(pl_8 as isize))
                                            .as_mut_ptr()
                                            .offset((cbx4 + x_0) as isize) as *mut uint8_t
                                            as *mut alias16))
                                            .u16_0 = (0x101 as libc::c_int * cf_ctx as libc::c_int)
                                            as uint16_t;
                                    }
                                    4 => {
                                        (*(&mut *(*((*(*t).a).ccoef)
                                            .as_mut_ptr()
                                            .offset(pl_8 as isize))
                                            .as_mut_ptr()
                                            .offset((cbx4 + x_0) as isize) as *mut uint8_t
                                            as *mut alias32))
                                            .u32_0 = (0x1010101 as libc::c_uint)
                                            .wrapping_mul(cf_ctx as libc::c_uint);
                                    }
                                    8 => {
                                        (*(&mut *(*((*(*t).a).ccoef)
                                            .as_mut_ptr()
                                            .offset(pl_8 as isize))
                                            .as_mut_ptr()
                                            .offset((cbx4 + x_0) as isize) as *mut uint8_t
                                            as *mut alias64))
                                            .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                                            .wrapping_mul(cf_ctx as libc::c_ulonglong) as uint64_t;
                                    }
                                    16 => {
                                        let const_val_12: uint64_t = (0x101010101010101
                                            as libc::c_ulonglong)
                                            .wrapping_mul(cf_ctx as libc::c_ulonglong) as uint64_t;
                                        (*(&mut *(*((*(*t).a).ccoef)
                                            .as_mut_ptr()
                                            .offset(pl_8 as isize))
                                            .as_mut_ptr()
                                            .offset((cbx4 + x_0 + 0 as libc::c_int) as isize)
                                            as *mut uint8_t as *mut alias64))
                                            .u64_0 = const_val_12;
                                        (*(&mut *(*((*(*t).a).ccoef)
                                            .as_mut_ptr()
                                            .offset(pl_8 as isize))
                                            .as_mut_ptr()
                                            .offset((cbx4 + x_0 + 8 as libc::c_int) as isize)
                                            as *mut uint8_t as *mut alias64))
                                            .u64_0 = const_val_12;
                                    }
                                    _ => {
                                        memset(
                                            &mut *(*((*(*t).a).ccoef)
                                                .as_mut_ptr()
                                                .offset(pl_8 as isize))
                                                .as_mut_ptr()
                                                .offset((cbx4 + x_0) as isize) as *mut uint8_t
                                                as *mut libc::c_void,
                                            cf_ctx as libc::c_int,
                                            imin(
                                                (*uvtx).w as libc::c_int,
                                                (*f).bw - (*t).bx + ss_hor >> ss_hor,
                                            ) as size_t,
                                        );
                                    }
                                }
                            }
                            if eob >= 0 as libc::c_int {
                                if 0 as libc::c_int != 0
                                    && (*(*f).frame_hdr).frame_offset == 2 as libc::c_int
                                    && (*t).by >= 0 as libc::c_int && (*t).by < 4 as libc::c_int
                                    && (*t).bx >= 8 as libc::c_int
                                    && (*t).bx < 12 as libc::c_int && 0 as libc::c_int != 0
                                {
                                    coef_dump(
                                        cf,
                                        (*uvtx).h as libc::c_int * 4 as libc::c_int,
                                        (*uvtx).w as libc::c_int * 4 as libc::c_int,
                                        3 as libc::c_int,
                                        b"dq\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                ((*dsp).itx.itxfm_add[(*b).uvtx as usize][txtp as usize])
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    &mut *uvdst_1.offset((4 as libc::c_int * x_0) as isize),
                                    (*f).cur.stride[1 as libc::c_int as usize],
                                    cf,
                                    eob,
                                    (*f).bitdepth_max,
                                );
                                if 0 as libc::c_int != 0
                                    && (*(*f).frame_hdr).frame_offset == 2 as libc::c_int
                                    && (*t).by >= 0 as libc::c_int && (*t).by < 4 as libc::c_int
                                    && (*t).bx >= 8 as libc::c_int
                                    && (*t).bx < 12 as libc::c_int && 0 as libc::c_int != 0
                                {
                                    hex_dump(
                                        &mut *uvdst_1.offset((4 as libc::c_int * x_0) as isize),
                                        (*f).cur.stride[1 as libc::c_int as usize],
                                        (*uvtx).w as libc::c_int * 4 as libc::c_int,
                                        (*uvtx).h as libc::c_int * 4 as libc::c_int,
                                        b"recon\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                            }
                            (*t).bx += ((*uvtx).w as libc::c_int) << ss_hor;
                            x_0 += (*uvtx).w as libc::c_int;
                        }
                        uvdst_1 = uvdst_1
                            .offset(
                                (PXSTRIDE((*f).cur.stride[1 as libc::c_int as usize])
                                    * 4
                                    * (*uvtx).h as isize) as isize,
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
        & DAV1D_INLOOPFILTER_DEBLOCK as libc::c_int as libc::c_uint == 0
        || (*(*f).frame_hdr).loopfilter.level_y[0 as libc::c_int as usize] == 0
            && (*(*f).frame_hdr).loopfilter.level_y[1 as libc::c_int as usize] == 0
    {
        return;
    }
    let y: libc::c_int = sby * (*f).sb_step * 4 as libc::c_int;
    let ss_ver: libc::c_int = ((*f).cur.p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let p: [*mut pixel; 3] = [
        ((*f).lf.p[0 as libc::c_int as usize])
            .offset(
                (y as isize
                    * PXSTRIDE((*f).cur.stride[0 as libc::c_int as usize])) as isize,
            ),
        ((*f).lf.p[1 as libc::c_int as usize])
            .offset(
                (y as isize * PXSTRIDE((*f).cur.stride[1 as libc::c_int as usize])
                    >> ss_ver) as isize,
            ),
        ((*f).lf.p[2 as libc::c_int as usize])
            .offset(
                (y as isize * PXSTRIDE((*f).cur.stride[1 as libc::c_int as usize])
                    >> ss_ver) as isize,
            ),
    ];
    let mut mask: *mut Av1Filter = ((*f).lf.mask)
        .offset(
            ((sby >> ((*(*f).seq_hdr).sb128 == 0) as libc::c_int) * (*f).sb128w) as isize,
        );
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
    let y: libc::c_int = sby * (*f).sb_step * 4 as libc::c_int;
    let ss_ver: libc::c_int = ((*f).cur.p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let p: [*mut pixel; 3] = [
        ((*f).lf.p[0 as libc::c_int as usize])
            .offset(
                (y as isize
                    * PXSTRIDE((*f).cur.stride[0 as libc::c_int as usize])) as isize,
            ),
        ((*f).lf.p[1 as libc::c_int as usize])
            .offset(
                (y as isize * PXSTRIDE((*f).cur.stride[1 as libc::c_int as usize])
                    >> ss_ver) as isize,
            ),
        ((*f).lf.p[2 as libc::c_int as usize])
            .offset(
                (y as isize * PXSTRIDE((*f).cur.stride[1 as libc::c_int as usize])
                    >> ss_ver) as isize,
            ),
    ];
    let mut mask: *mut Av1Filter = ((*f).lf.mask)
        .offset(
            ((sby >> ((*(*f).seq_hdr).sb128 == 0) as libc::c_int) * (*f).sb128w) as isize,
        );
    if (*(*f).c).inloop_filters as libc::c_uint
        & DAV1D_INLOOPFILTER_DEBLOCK as libc::c_int as libc::c_uint != 0
        && ((*(*f).frame_hdr).loopfilter.level_y[0 as libc::c_int as usize] != 0
            || (*(*f).frame_hdr).loopfilter.level_y[1 as libc::c_int as usize] != 0)
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
        & DAV1D_INLOOPFILTER_CDEF as libc::c_int as libc::c_uint == 0
    {
        return;
    }
    let sbsz: libc::c_int = (*f).sb_step;
    let y: libc::c_int = sby * sbsz * 4 as libc::c_int;
    let ss_ver: libc::c_int = ((*f).cur.p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let p: [*mut pixel; 3] = [
        ((*f).lf.p[0 as libc::c_int as usize])
            .offset(
                (y as isize
                    * PXSTRIDE((*f).cur.stride[0 as libc::c_int as usize])) as isize,
            ),
        ((*f).lf.p[1 as libc::c_int as usize])
            .offset(
                (y as isize * PXSTRIDE((*f).cur.stride[1 as libc::c_int as usize])
                    >> ss_ver) as isize,
            ),
        ((*f).lf.p[2 as libc::c_int as usize])
            .offset(
                (y as isize * PXSTRIDE((*f).cur.stride[1 as libc::c_int as usize])
                    >> ss_ver) as isize,
            ),
    ];
    let mut prev_mask: *mut Av1Filter = ((*f).lf.mask)
        .offset(
            ((sby - 1 as libc::c_int >> ((*(*f).seq_hdr).sb128 == 0) as libc::c_int)
                * (*f).sb128w) as isize,
        );
    let mut mask: *mut Av1Filter = ((*f).lf.mask)
        .offset(
            ((sby >> ((*(*f).seq_hdr).sb128 == 0) as libc::c_int) * (*f).sb128w) as isize,
        );
    let start: libc::c_int = sby * sbsz;
    if sby != 0 {
        let ss_ver_0: libc::c_int = ((*f).cur.p.layout as libc::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
        let mut p_up: [*mut pixel; 3] = [
            (p[0 as libc::c_int as usize])
                .offset(
                    -((8 * PXSTRIDE((*f).cur.stride[0 as libc::c_int as usize])) as isize),
                ),
            (p[1 as libc::c_int as usize])
                .offset(
                    -((8 * PXSTRIDE((*f).cur.stride[1 as libc::c_int as usize])
                        >> ss_ver_0) as isize),
                ),
            (p[2 as libc::c_int as usize])
                .offset(
                    -((8 * PXSTRIDE((*f).cur.stride[1 as libc::c_int as usize])
                        >> ss_ver_0) as isize),
                ),
        ];
        dav1d_cdef_brow_16bpc(
            tc,
            p_up.as_mut_ptr() as *const *mut pixel,
            prev_mask,
            start - 2 as libc::c_int,
            start,
            1 as libc::c_int,
            sby,
        );
    }
    let n_blks: libc::c_int = sbsz
        - 2 as libc::c_int * ((sby + 1 as libc::c_int) < (*f).sbh) as libc::c_int;
    let end: libc::c_int = imin(start + n_blks, (*f).bh);
    dav1d_cdef_brow_16bpc(tc, p.as_ptr(), mask, start, end, 0 as libc::c_int, sby);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_filter_sbrow_resize_16bpc(
    f: *mut Dav1dFrameContext,
    sby: libc::c_int,
) {
    let sbsz: libc::c_int = (*f).sb_step;
    let y: libc::c_int = sby * sbsz * 4 as libc::c_int;
    let ss_ver: libc::c_int = ((*f).cur.p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let p: [*const pixel; 3] = [
        ((*f).lf.p[0 as libc::c_int as usize])
            .offset(
                (y as isize
                    * PXSTRIDE((*f).cur.stride[0 as libc::c_int as usize])) as isize,
            ) as *const pixel,
        ((*f).lf.p[1 as libc::c_int as usize])
            .offset(
                (y as isize * PXSTRIDE((*f).cur.stride[1 as libc::c_int as usize])
                    >> ss_ver) as isize,
            ) as *const pixel,
        ((*f).lf.p[2 as libc::c_int as usize])
            .offset(
                (y as isize * PXSTRIDE((*f).cur.stride[1 as libc::c_int as usize])
                    >> ss_ver) as isize,
            ) as *const pixel,
    ];
    let sr_p: [*mut pixel; 3] = [
        ((*f).lf.sr_p[0 as libc::c_int as usize])
            .offset(
                (y as isize
                    * PXSTRIDE((*f).sr_cur.p.stride[0 as libc::c_int as usize])) as isize,
            ),
        ((*f).lf.sr_p[1 as libc::c_int as usize])
            .offset(
                (y as isize
                    * PXSTRIDE((*f).sr_cur.p.stride[1 as libc::c_int as usize])
                    >> ss_ver) as isize,
            ),
        ((*f).lf.sr_p[2 as libc::c_int as usize])
            .offset(
                (y as isize
                    * PXSTRIDE((*f).sr_cur.p.stride[1 as libc::c_int as usize])
                    >> ss_ver) as isize,
            ),
    ];
    let has_chroma: libc::c_int = ((*f).cur.p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint) as libc::c_int;
    let mut pl: libc::c_int = 0 as libc::c_int;
    while pl < 1 as libc::c_int + 2 as libc::c_int * has_chroma {
        let ss_ver_0: libc::c_int = (pl != 0
            && (*f).cur.p.layout as libc::c_uint
                == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint)
            as libc::c_int;
        let h_start: libc::c_int = 8 as libc::c_int * (sby != 0) as libc::c_int
            >> ss_ver_0;
        let dst_stride: ptrdiff_t = (*f)
            .sr_cur
            .p
            .stride[(pl != 0) as libc::c_int as usize];
        let mut dst: *mut pixel = (sr_p[pl as usize])
            .offset(-((h_start as isize * PXSTRIDE(dst_stride)) as isize));
        let src_stride: ptrdiff_t = (*f).cur.stride[(pl != 0) as libc::c_int as usize];
        let mut src: *const pixel = (p[pl as usize])
            .offset(-(h_start as isize * PXSTRIDE(src_stride)));
        let h_end: libc::c_int = 4 as libc::c_int
            * (sbsz
                - 2 as libc::c_int
                    * ((sby + 1 as libc::c_int) < (*f).sbh) as libc::c_int) >> ss_ver_0;
        let ss_hor: libc::c_int = (pl != 0
            && (*f).cur.p.layout as libc::c_uint
                != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint)
            as libc::c_int;
        let dst_w: libc::c_int = (*f).sr_cur.p.p.w + ss_hor >> ss_hor;
        let src_w: libc::c_int = 4 as libc::c_int * (*f).bw + ss_hor >> ss_hor;
        let img_h: libc::c_int = (*f).cur.p.h - sbsz * 4 as libc::c_int * sby + ss_ver_0
            >> ss_ver_0;
        ((*(*f).dsp).mc.resize)
            .expect(
                "non-null function pointer",
            )(
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
pub unsafe extern "C" fn dav1d_filter_sbrow_lr_16bpc(
    f: *mut Dav1dFrameContext,
    sby: libc::c_int,
) {
    if (*(*f).c).inloop_filters as libc::c_uint
        & DAV1D_INLOOPFILTER_RESTORATION as libc::c_int as libc::c_uint == 0
    {
        return;
    }
    let y: libc::c_int = sby * (*f).sb_step * 4 as libc::c_int;
    let ss_ver: libc::c_int = ((*f).cur.p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let sr_p: [*mut pixel; 3] = [
        ((*f).lf.sr_p[0 as libc::c_int as usize])
            .offset(y as isize
                    * PXSTRIDE((*f).sr_cur.p.stride[0 as libc::c_int as usize]),
            ),
        ((*f).lf.sr_p[1 as libc::c_int as usize])
            .offset(y as isize
                    * PXSTRIDE((*f).sr_cur.p.stride[1 as libc::c_int as usize])
                    >> ss_ver,
            ),
        ((*f).lf.sr_p[2 as libc::c_int as usize])
            .offset(y as isize
                    * PXSTRIDE((*f).sr_cur.p.stride[1 as libc::c_int as usize])
                    >> ss_ver
            ),
    ];
    dav1d_lr_sbrow_16bpc(f, sr_p.as_ptr(), sby);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_filter_sbrow_16bpc(
    f: *mut Dav1dFrameContext,
    sby: libc::c_int,
) {
    dav1d_filter_sbrow_deblock_cols_16bpc(f, sby);
    dav1d_filter_sbrow_deblock_rows_16bpc(f, sby);
    if (*(*f).seq_hdr).cdef != 0 {
        dav1d_filter_sbrow_cdef_16bpc((*(*f).c).tc, sby);
    }
    if (*(*f).frame_hdr).width[0 as libc::c_int as usize]
        != (*(*f).frame_hdr).width[1 as libc::c_int as usize]
    {
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
    let sby: libc::c_int = (*t).by >> (*f).sb_shift;
    let sby_off: libc::c_int = (*f).sb128w * 128 as libc::c_int * sby;
    let x_off: libc::c_int = (*ts).tiling.col_start;
    let y: *const pixel = ((*f).cur.data[0 as libc::c_int as usize] as *const pixel)
        .offset((x_off * 4 as libc::c_int) as isize)
        .offset(
            ((((*t).by + (*f).sb_step) * 4 as libc::c_int - 1 as libc::c_int)
                as isize * PXSTRIDE((*f).cur.stride[0 as libc::c_int as usize]))
                as isize,
        );
    memcpy(
        &mut *(*((*f).ipred_edge).as_ptr().offset(0 as libc::c_int as isize))
            .offset((sby_off + x_off * 4 as libc::c_int) as isize) as *mut pixel
            as *mut libc::c_void,
        y as *const libc::c_void,
        (4 as libc::c_int * ((*ts).tiling.col_end - x_off) << 1 as libc::c_int) as size_t,
    );
    if (*f).cur.p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint
    {
        let ss_ver: libc::c_int = ((*f).cur.p.layout as libc::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
        let ss_hor: libc::c_int = ((*f).cur.p.layout as libc::c_uint
            != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
        let uv_off: ptrdiff_t = (x_off * 4 >> ss_hor) as isize
            + ((((*t).by + (*f).sb_step) * 4 >> ss_ver)
                - 1) as isize
                * PXSTRIDE((*f).cur.stride[1 as libc::c_int as usize]);
        let mut pl: libc::c_int = 1 as libc::c_int;
        while pl <= 2 as libc::c_int {
            memcpy(
                &mut *(*((*f).ipred_edge).as_ptr().offset(pl as isize))
                    .offset((sby_off + (x_off * 4 as libc::c_int >> ss_hor)) as isize)
                    as *mut pixel as *mut libc::c_void,
                &*(*((*f).cur.data).as_ptr().offset(pl as isize) as *const pixel)
                    .offset(uv_off as isize) as *const pixel as *const libc::c_void,
                (4 as libc::c_int * ((*ts).tiling.col_end - x_off) >> ss_hor
                    << 1 as libc::c_int) as size_t,
            );
            pl += 1;
        }
    }
}
