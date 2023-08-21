use crate::include::stddef::*;
use crate::include::stdint::*;

use ::libc;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: size_t) -> *mut libc::c_void;
}

pub type pixel = uint8_t;
pub type coef = int16_t;
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

use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::dav1d::headers::Dav1dPixelLayout;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I400;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I422;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;

use crate::include::dav1d::headers::Dav1dWarpedMotionParams;

use crate::include::dav1d::headers::Dav1dFilmGrainData;
use crate::include::dav1d::headers::Dav1dSequenceHeader;

use crate::src::align::Align16;

use crate::src::internal::Dav1dFrameContext_frame_thread;
use crate::src::internal::Dav1dFrameContext_lf;
use crate::src::lf_mask::Av1Filter;

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

use crate::include::dav1d::dav1d::Dav1dEventFlags;
use crate::include::dav1d::dav1d::Dav1dLogger;
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
    ) -> (),
>;
use crate::src::cdef::CdefEdgeFlags;
use crate::src::cdef::CDEF_HAVE_BOTTOM;
use crate::src::cdef::CDEF_HAVE_LEFT;
use crate::src::cdef::CDEF_HAVE_RIGHT;
use crate::src::cdef::CDEF_HAVE_TOP;
pub type const_left_pixel_row_2px = *const [pixel; 2];
pub type cdef_dir_fn =
    Option<unsafe extern "C" fn(*const pixel, ptrdiff_t, *mut libc::c_uint) -> libc::c_int>;
use crate::src::loopfilter::Dav1dLoopFilterDSPContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dInvTxfmDSPContext {
    pub itxfm_add: [[itxfm_fn; 17]; 19],
}
pub type itxfm_fn =
    Option<unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, libc::c_int) -> ()>;
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
    ) -> (),
>;
pub type entry = int8_t;
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
use crate::src::internal::ScalableMotionParams;
pub type Backup2x8Flags = libc::c_uint;
pub const BACKUP_2X8_UV: Backup2x8Flags = 2;
pub const BACKUP_2X8_Y: Backup2x8Flags = 1;

use crate::include::common::intops::imin;
use crate::include::common::intops::ulog2;
unsafe extern "C" fn backup2lines(
    mut dst: *const *mut pixel,
    mut src: *const *mut pixel,
    mut stride: *const ptrdiff_t,
    layout: Dav1dPixelLayout,
) {
    let y_stride: ptrdiff_t = *stride.offset(0);
    if y_stride < 0 {
        memcpy(
            (*dst.offset(0)).offset(y_stride as isize) as *mut libc::c_void,
            (*src.offset(0)).offset(7 * y_stride as isize) as *const libc::c_void,
            (-(2) * y_stride) as size_t,
        );
    } else {
        memcpy(
            *dst.offset(0) as *mut libc::c_void,
            (*src.offset(0)).offset((6 * y_stride) as isize) as *const libc::c_void,
            (2 * y_stride) as size_t,
        );
    }
    if layout as libc::c_uint != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint {
        let uv_stride: ptrdiff_t = *stride.offset(1);
        if uv_stride < 0 {
            let uv_off = if layout as libc::c_uint
                == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint
            {
                3 as libc::c_int
            } else {
                7 as libc::c_int
            };
            memcpy(
                (*dst.offset(1)).offset(uv_stride as isize) as *mut libc::c_void,
                (*src.offset(1)).offset((uv_off as isize * uv_stride) as isize)
                    as *const libc::c_void,
                (-(2) * uv_stride) as size_t,
            );
            memcpy(
                (*dst.offset(2)).offset(uv_stride as isize) as *mut libc::c_void,
                (*src.offset(2)).offset((uv_off as isize * uv_stride) as isize)
                    as *const libc::c_void,
                (-(2) * uv_stride) as size_t,
            );
        } else {
            let uv_off_0 = if layout as libc::c_uint
                == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint
            {
                2 as libc::c_int
            } else {
                6 as libc::c_int
            };
            memcpy(
                *dst.offset(1) as *mut libc::c_void,
                (*src.offset(1)).offset((uv_off_0 as isize * uv_stride) as isize)
                    as *const libc::c_void,
                (2 * uv_stride) as size_t,
            );
            memcpy(
                *dst.offset(2) as *mut libc::c_void,
                (*src.offset(2)).offset((uv_off_0 as isize * uv_stride) as isize)
                    as *const libc::c_void,
                (2 * uv_stride) as size_t,
            );
        }
    }
}
unsafe extern "C" fn backup2x8(
    mut dst: *mut [[pixel; 2]; 8],
    mut src: *const *mut pixel,
    mut src_stride: *const ptrdiff_t,
    mut x_off: libc::c_int,
    layout: Dav1dPixelLayout,
    flag: Backup2x8Flags,
) {
    let mut y_off: ptrdiff_t = 0 as libc::c_int as ptrdiff_t;
    if flag as libc::c_uint & BACKUP_2X8_Y as libc::c_int as libc::c_uint != 0 {
        let mut y = 0;
        while y < 8 {
            memcpy(
                ((*dst.offset(0))[y as usize]).as_mut_ptr() as *mut libc::c_void,
                &mut *(*src.offset(0)).offset((y_off + x_off as isize - 2) as isize) as *mut pixel
                    as *const libc::c_void,
                2,
            );
            y += 1;
            y_off += *src_stride.offset(0);
        }
    }
    if layout as libc::c_uint == DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint
        || flag as libc::c_uint & BACKUP_2X8_UV as libc::c_int as libc::c_uint == 0
    {
        return;
    }
    let ss_ver = (layout as libc::c_uint == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint)
        as libc::c_int;
    let ss_hor = (layout as libc::c_uint != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint)
        as libc::c_int;
    x_off >>= ss_hor;
    y_off = 0 as libc::c_int as ptrdiff_t;
    let mut y_0 = 0;
    while y_0 < 8 >> ss_ver {
        memcpy(
            ((*dst.offset(1))[y_0 as usize]).as_mut_ptr() as *mut libc::c_void,
            &mut *(*src.offset(1)).offset((y_off + x_off as isize - 2) as isize) as *mut pixel
                as *const libc::c_void,
            2 as size_t,
        );
        memcpy(
            ((*dst.offset(2))[y_0 as usize]).as_mut_ptr() as *mut libc::c_void,
            &mut *(*src.offset(2)).offset((y_off + x_off as isize - 2 as isize) as isize)
                as *mut pixel as *const libc::c_void,
            2,
        );
        y_0 += 1;
        y_off += *src_stride.offset(1);
    }
}
unsafe extern "C" fn adjust_strength(strength: libc::c_int, var: libc::c_uint) -> libc::c_int {
    if var == 0 {
        return 0 as libc::c_int;
    }
    let i = if var >> 6 != 0 {
        imin(ulog2(var >> 6), 12 as libc::c_int)
    } else {
        0 as libc::c_int
    };
    return strength * (4 + i) + 8 >> 4;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_cdef_brow_8bpc(
    tc: *mut Dav1dTaskContext,
    mut p: *const *mut pixel,
    lflvl: *const Av1Filter,
    by_start: libc::c_int,
    by_end: libc::c_int,
    sbrow_start: libc::c_int,
    sby: libc::c_int,
) {
    let f: *mut Dav1dFrameContext = (*tc).f as *mut Dav1dFrameContext;
    let bitdepth_min_8 = if 8 == 8 {
        0 as libc::c_int
    } else {
        (*f).cur.p.bpc - 8
    };
    let dsp: *const Dav1dDSPContext = (*f).dsp;
    let mut edges: CdefEdgeFlags = (CDEF_HAVE_BOTTOM as libc::c_int
        | (if by_start > 0 {
            CDEF_HAVE_TOP as libc::c_int
        } else {
            0 as libc::c_int
        })) as CdefEdgeFlags;
    let mut ptrs: [*mut pixel; 3] = [*p.offset(0), *p.offset(1), *p.offset(2)];
    let sbsz = 16;
    let sb64w = (*f).sb128w << 1;
    let damping = (*(*f).frame_hdr).cdef.damping + bitdepth_min_8;
    let layout: Dav1dPixelLayout = (*f).cur.p.layout;
    let uv_idx = (DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint)
        .wrapping_sub(layout as libc::c_uint) as libc::c_int;
    let ss_ver = (layout as libc::c_uint == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint)
        as libc::c_int;
    let ss_hor = (layout as libc::c_uint != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint)
        as libc::c_int;
    static mut uv_dirs: [[uint8_t; 8]; 2] = [
        [
            0 as libc::c_int as uint8_t,
            1 as libc::c_int as uint8_t,
            2 as libc::c_int as uint8_t,
            3 as libc::c_int as uint8_t,
            4 as libc::c_int as uint8_t,
            5 as libc::c_int as uint8_t,
            6 as libc::c_int as uint8_t,
            7 as libc::c_int as uint8_t,
        ],
        [
            7 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            2 as libc::c_int as uint8_t,
            4 as libc::c_int as uint8_t,
            5 as libc::c_int as uint8_t,
            6 as libc::c_int as uint8_t,
            6 as libc::c_int as uint8_t,
            6 as libc::c_int as uint8_t,
        ],
    ];
    let mut uv_dir: *const uint8_t = (uv_dirs[(layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I422 as libc::c_int as libc::c_uint)
        as libc::c_int as usize])
        .as_ptr();
    let have_tt = ((*(*f).c).n_tc > 1 as libc::c_uint) as libc::c_int;
    let sb128 = (*(*f).seq_hdr).sb128;
    let resize = ((*(*f).frame_hdr).width[0] != (*(*f).frame_hdr).width[1]) as libc::c_int;
    let y_stride: ptrdiff_t = (*f).cur.stride[0];
    let uv_stride: ptrdiff_t = (*f).cur.stride[1];
    let mut bit = 0;
    let mut by = by_start;
    while by < by_end {
        let tf = (*tc).top_pre_cdef_toggle;
        let by_idx = (by & 30) >> 1;
        if by + 2 >= (*f).bh {
            edges = ::core::mem::transmute::<libc::c_uint, CdefEdgeFlags>(
                edges as libc::c_uint & !(CDEF_HAVE_BOTTOM as libc::c_int) as libc::c_uint,
            );
        }
        if (have_tt == 0 || sbrow_start != 0 || (by + 2) < by_end)
            && edges as libc::c_uint & CDEF_HAVE_BOTTOM as libc::c_int as libc::c_uint != 0
        {
            let cdef_top_bak: [*mut pixel; 3] = [
                ((*f).lf.cdef_line[(tf == 0) as libc::c_int as usize][0] as *mut pixel)
                    .offset(((have_tt * sby * 4) as isize * y_stride) as isize),
                ((*f).lf.cdef_line[(tf == 0) as libc::c_int as usize][1] as *mut pixel)
                    .offset(((have_tt * sby * 8) as isize * uv_stride) as isize),
                ((*f).lf.cdef_line[(tf == 0) as libc::c_int as usize][2] as *mut pixel)
                    .offset(((have_tt * sby * 8) as isize * uv_stride) as isize),
            ];
            backup2lines(
                cdef_top_bak.as_ptr(),
                ptrs.as_mut_ptr() as *const *mut pixel,
                ((*f).cur.stride).as_mut_ptr() as *const ptrdiff_t,
                layout,
            );
        }
        let mut lr_bak: Align16<[[[[pixel; 2]; 8]; 3]; 2]> = Align16([[[[0; 2]; 8]; 3]; 2]);
        let mut iptrs: [*mut pixel; 3] = [ptrs[0], ptrs[1], ptrs[2]];
        edges = ::core::mem::transmute::<libc::c_uint, CdefEdgeFlags>(
            edges as libc::c_uint & !(CDEF_HAVE_LEFT as libc::c_int) as libc::c_uint,
        );
        edges = ::core::mem::transmute::<libc::c_uint, CdefEdgeFlags>(
            edges as libc::c_uint | CDEF_HAVE_RIGHT as libc::c_int as libc::c_uint,
        );
        let mut prev_flag: Backup2x8Flags = 0 as Backup2x8Flags;
        let mut sbx = 0;
        let mut last_skip = 1;
        while sbx < sb64w {
            let mut noskip_row: *const [uint16_t; 2] = 0 as *const [uint16_t; 2];
            let mut noskip_mask: libc::c_uint = 0;
            let mut y_lvl = 0;
            let mut uv_lvl = 0;
            let mut flag: Backup2x8Flags = 0 as Backup2x8Flags;
            let mut y_pri_lvl = 0;
            let mut y_sec_lvl = 0;
            let mut uv_pri_lvl = 0;
            let mut uv_sec_lvl = 0;
            let mut bptrs: [*mut pixel; 3] = [0 as *mut pixel; 3];
            let sb128x = sbx >> 1;
            let sb64_idx = ((by & sbsz) >> 3) + (sbx & 1);
            let cdef_idx =
                (*lflvl.offset(sb128x as isize)).cdef_idx[sb64_idx as usize] as libc::c_int;
            if cdef_idx == -(1 as libc::c_int)
                || (*(*f).frame_hdr).cdef.y_strength[cdef_idx as usize] == 0
                    && (*(*f).frame_hdr).cdef.uv_strength[cdef_idx as usize] == 0
            {
                last_skip = 1 as libc::c_int;
            } else {
                noskip_row = &*((*lflvl.offset(sb128x as isize)).noskip_mask)
                    .as_ptr()
                    .offset(by_idx as isize) as *const [uint16_t; 2];
                noskip_mask = ((*noskip_row.offset(0))[1] as libc::c_uint) << 16
                    | (*noskip_row.offset(0))[0] as libc::c_uint;
                y_lvl = (*(*f).frame_hdr).cdef.y_strength[cdef_idx as usize];
                uv_lvl = (*(*f).frame_hdr).cdef.uv_strength[cdef_idx as usize];
                flag = ((y_lvl != 0) as libc::c_int + (((uv_lvl != 0) as libc::c_int) << 1))
                    as Backup2x8Flags;
                y_pri_lvl = (y_lvl >> 2) << bitdepth_min_8;
                y_sec_lvl = y_lvl & 3;
                y_sec_lvl += (y_sec_lvl == 3) as libc::c_int;
                y_sec_lvl <<= bitdepth_min_8;
                uv_pri_lvl = (uv_lvl >> 2) << bitdepth_min_8;
                uv_sec_lvl = uv_lvl & 3;
                uv_sec_lvl += (uv_sec_lvl == 3) as libc::c_int;
                uv_sec_lvl <<= bitdepth_min_8;
                bptrs = [iptrs[0], iptrs[1], iptrs[2]];
                let mut bx = sbx * sbsz;
                while bx < imin((sbx + 1) * sbsz, (*f).bw) {
                    let mut uvdir = 0;
                    let mut do_left = 0;
                    let mut dir = 0;
                    let mut variance: libc::c_uint = 0;
                    let mut top: *const pixel = 0 as *const pixel;
                    let mut bot: *const pixel = 0 as *const pixel;
                    let mut offset: ptrdiff_t = 0;
                    let mut current_block_84: u64;
                    if bx + 2 >= (*f).bw {
                        edges = ::core::mem::transmute::<libc::c_uint, CdefEdgeFlags>(
                            edges as libc::c_uint
                                & !(CDEF_HAVE_RIGHT as libc::c_int) as libc::c_uint,
                        );
                    }
                    let bx_mask: uint32_t = (3 as libc::c_uint) << (bx & 30);
                    if noskip_mask & bx_mask == 0 {
                        last_skip = 1 as libc::c_int;
                    } else {
                        do_left = (if last_skip != 0 {
                            flag as libc::c_uint
                        } else {
                            (prev_flag as libc::c_uint ^ flag as libc::c_uint)
                                & flag as libc::c_uint
                        }) as libc::c_int;
                        prev_flag = flag;
                        if do_left != 0
                            && edges as libc::c_uint & CDEF_HAVE_LEFT as libc::c_int as libc::c_uint
                                != 0
                        {
                            backup2x8(
                                (lr_bak[bit as usize]).as_mut_ptr(),
                                bptrs.as_mut_ptr() as *const *mut pixel,
                                ((*f).cur.stride).as_mut_ptr() as *const ptrdiff_t,
                                0 as libc::c_int,
                                layout,
                                do_left as Backup2x8Flags,
                            );
                        }
                        if edges as libc::c_uint & CDEF_HAVE_RIGHT as libc::c_int as libc::c_uint
                            != 0
                        {
                            backup2x8(
                                (lr_bak[(bit == 0) as libc::c_int as usize]).as_mut_ptr(),
                                bptrs.as_mut_ptr() as *const *mut pixel,
                                ((*f).cur.stride).as_mut_ptr() as *const ptrdiff_t,
                                8 as libc::c_int,
                                layout,
                                flag,
                            );
                        }
                        dir = 0;
                        variance = 0;
                        if y_pri_lvl != 0 || uv_pri_lvl != 0 {
                            dir = ((*dsp).cdef.dir).expect("non-null function pointer")(
                                bptrs[0],
                                (*f).cur.stride[0],
                                &mut variance,
                            );
                        }
                        top = 0 as *const pixel;
                        bot = 0 as *const pixel;
                        offset = 0;
                        if have_tt == 0 {
                            current_block_84 = 17728966195399430138;
                        } else if sbrow_start != 0 && by == by_start {
                            if resize != 0 {
                                offset = ((sby - 1) * 4) as isize * y_stride + (bx * 4) as isize;
                                top = &mut *((*((*f).lf.cdef_lpf_line).as_mut_ptr().offset(0))
                                    as *mut pixel)
                                    .offset(offset as isize);
                            } else {
                                offset = (sby * ((4 as libc::c_int) << sb128) - 4) as isize
                                    * y_stride
                                    + (bx * 4) as isize;
                                top = &mut *((*((*f).lf.lr_lpf_line).as_mut_ptr().offset(0))
                                    as *mut pixel)
                                    .offset(offset as isize);
                            }
                            bot = (bptrs[0]).offset((8 * y_stride) as isize);
                            current_block_84 = 17075014677070940716;
                        } else if sbrow_start == 0 && by + 2 >= by_end {
                            top = &mut *((*(*((*f).lf.cdef_line).as_mut_ptr().offset(tf as isize))
                                .as_mut_ptr()
                                .offset(0)) as *mut pixel)
                                .offset(
                                    ((sby * 4) as isize * y_stride + (bx * 4) as isize) as isize,
                                );
                            if resize != 0 {
                                offset = (sby * 4 + 2) as isize * y_stride + (bx * 4) as isize;
                                bot = &mut *((*((*f).lf.cdef_lpf_line).as_mut_ptr().offset(0))
                                    as *mut pixel)
                                    .offset(offset as isize);
                            } else {
                                let line = sby * ((4 as libc::c_int) << sb128) + 4 * sb128 + 2;
                                offset = line as isize * y_stride + (bx * 4) as isize;
                                bot = &mut *((*((*f).lf.lr_lpf_line).as_mut_ptr().offset(0))
                                    as *mut pixel)
                                    .offset(offset as isize);
                            }
                            current_block_84 = 17075014677070940716;
                        } else {
                            current_block_84 = 17728966195399430138;
                        }
                        match current_block_84 {
                            17728966195399430138 => {
                                offset = (sby * 4) as isize * y_stride;
                                top = &mut *((*(*((*f).lf.cdef_line)
                                    .as_mut_ptr()
                                    .offset(tf as isize))
                                .as_mut_ptr()
                                .offset(0))
                                    as *mut pixel)
                                    .offset(
                                        (have_tt as isize * offset + (bx * 4) as isize) as isize,
                                    );
                                bot = (bptrs[0]).offset((8 * y_stride) as isize);
                            }
                            _ => {}
                        }
                        if y_pri_lvl != 0 {
                            let adj_y_pri_lvl = adjust_strength(y_pri_lvl, variance);
                            if adj_y_pri_lvl != 0 || y_sec_lvl != 0 {
                                ((*dsp).cdef.fb[0]).expect("non-null function pointer")(
                                    bptrs[0],
                                    (*f).cur.stride[0],
                                    (lr_bak[bit as usize][0]).as_mut_ptr()
                                        as const_left_pixel_row_2px,
                                    top,
                                    bot,
                                    adj_y_pri_lvl,
                                    y_sec_lvl,
                                    dir,
                                    damping,
                                    edges,
                                );
                            }
                        } else if y_sec_lvl != 0 {
                            ((*dsp).cdef.fb[0]).expect("non-null function pointer")(
                                bptrs[0],
                                (*f).cur.stride[0],
                                (lr_bak[bit as usize][0]).as_mut_ptr() as const_left_pixel_row_2px,
                                top,
                                bot,
                                0 as libc::c_int,
                                y_sec_lvl,
                                0 as libc::c_int,
                                damping,
                                edges,
                            );
                        }
                        if !(uv_lvl == 0) {
                            if !(layout as libc::c_uint
                                != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint)
                            {
                                unreachable!();
                            }
                            uvdir = if uv_pri_lvl != 0 {
                                *uv_dir.offset(dir as isize) as libc::c_int
                            } else {
                                0 as libc::c_int
                            };
                            let mut pl = 1;
                            while pl <= 2 {
                                let mut current_block_77: u64;
                                if have_tt == 0 {
                                    current_block_77 = 5687667889785024198;
                                } else if sbrow_start != 0 && by == by_start {
                                    if resize != 0 {
                                        offset = ((sby - 1) * 4) as isize * uv_stride
                                            + (bx * 4 >> ss_hor) as isize;
                                        top = &mut *((*((*f).lf.cdef_lpf_line)
                                            .as_mut_ptr()
                                            .offset(pl as isize))
                                            as *mut pixel)
                                            .offset(offset as isize);
                                    } else {
                                        let line_0 = sby * ((4 as libc::c_int) << sb128) - 4;
                                        offset = line_0 as isize * uv_stride
                                            + (bx * 4 >> ss_hor) as isize;
                                        top = &mut *((*((*f).lf.lr_lpf_line)
                                            .as_mut_ptr()
                                            .offset(pl as isize))
                                            as *mut pixel)
                                            .offset(offset as isize);
                                    }
                                    bot = (bptrs[pl as usize])
                                        .offset(((8 >> ss_ver) as isize * uv_stride) as isize);
                                    current_block_77 = 6540614962658479183;
                                } else if sbrow_start == 0 && by + 2 >= by_end {
                                    let top_offset: ptrdiff_t = (sby * 8) as isize * uv_stride
                                        + (bx * 4 >> ss_hor) as isize;
                                    top = &mut *((*(*((*f).lf.cdef_line)
                                        .as_mut_ptr()
                                        .offset(tf as isize))
                                    .as_mut_ptr()
                                    .offset(pl as isize))
                                        as *mut pixel)
                                        .offset(top_offset as isize);
                                    if resize != 0 {
                                        offset = (sby * 4 + 2) as isize * uv_stride
                                            + (bx * 4 >> ss_hor) as isize;
                                        bot = &mut *((*((*f).lf.cdef_lpf_line)
                                            .as_mut_ptr()
                                            .offset(pl as isize))
                                            as *mut pixel)
                                            .offset(offset as isize);
                                    } else {
                                        let line_1 =
                                            sby * ((4 as libc::c_int) << sb128) + 4 * sb128 + 2;
                                        offset = line_1 as isize * uv_stride
                                            + (bx * 4 >> ss_hor) as isize;
                                        bot = &mut *((*((*f).lf.lr_lpf_line)
                                            .as_mut_ptr()
                                            .offset(pl as isize))
                                            as *mut pixel)
                                            .offset(offset as isize);
                                    }
                                    current_block_77 = 6540614962658479183;
                                } else {
                                    current_block_77 = 5687667889785024198;
                                }
                                match current_block_77 {
                                    5687667889785024198 => {
                                        let offset_0: ptrdiff_t = (sby * 8) as isize * uv_stride;
                                        top = &mut *((*(*((*f).lf.cdef_line)
                                            .as_mut_ptr()
                                            .offset(tf as isize))
                                        .as_mut_ptr()
                                        .offset(pl as isize))
                                            as *mut pixel)
                                            .offset(
                                                (have_tt as isize * offset_0
                                                    + (bx * 4 >> ss_hor) as isize)
                                                    as isize,
                                            );
                                        bot = (bptrs[pl as usize])
                                            .offset(((8 >> ss_ver) as isize * uv_stride) as isize);
                                    }
                                    _ => {}
                                }
                                ((*dsp).cdef.fb[uv_idx as usize])
                                    .expect("non-null function pointer")(
                                    bptrs[pl as usize],
                                    (*f).cur.stride[1],
                                    (lr_bak[bit as usize][pl as usize]).as_mut_ptr()
                                        as const_left_pixel_row_2px,
                                    top,
                                    bot,
                                    uv_pri_lvl,
                                    uv_sec_lvl,
                                    uvdir,
                                    damping - 1,
                                    edges,
                                );
                                pl += 1;
                            }
                        }
                        bit ^= 1 as libc::c_int;
                        last_skip = 0 as libc::c_int;
                    }
                    bptrs[0] = (bptrs[0]).offset(8);
                    bptrs[1] = (bptrs[1]).offset((8 >> ss_hor) as isize);
                    bptrs[2] = (bptrs[2]).offset((8 >> ss_hor) as isize);
                    bx += 2 as libc::c_int;
                    edges = ::core::mem::transmute::<libc::c_uint, CdefEdgeFlags>(
                        edges as libc::c_uint | CDEF_HAVE_LEFT as libc::c_int as libc::c_uint,
                    );
                }
            }
            iptrs[0] = (iptrs[0]).offset((sbsz * 4) as isize);
            iptrs[1] = (iptrs[1]).offset((sbsz * 4 >> ss_hor) as isize);
            iptrs[2] = (iptrs[2]).offset((sbsz * 4 >> ss_hor) as isize);
            sbx += 1;
            edges = ::core::mem::transmute::<libc::c_uint, CdefEdgeFlags>(
                edges as libc::c_uint | CDEF_HAVE_LEFT as libc::c_int as libc::c_uint,
            );
        }
        ptrs[0] = (ptrs[0]).offset((8 * (*f).cur.stride[0]) as isize);
        ptrs[1] = (ptrs[1]).offset((8 * (*f).cur.stride[1] >> ss_ver) as isize);
        ptrs[2] = (ptrs[2]).offset((8 * (*f).cur.stride[1] >> ss_ver) as isize);
        (*tc).top_pre_cdef_toggle ^= 1 as libc::c_int;
        by += 2 as libc::c_int;
        edges = ::core::mem::transmute::<libc::c_uint, CdefEdgeFlags>(
            edges as libc::c_uint | CDEF_HAVE_TOP as libc::c_int as libc::c_uint,
        );
    }
}
