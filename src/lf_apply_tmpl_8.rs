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
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;
use crate::include::dav1d::picture::Dav1dPicture;
use crate::src::internal::Dav1dFrameContext_task_thread;
use crate::src::internal::FrameTileThreadData;
use crate::src::internal::TaskThreadData;

use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I400;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;

use crate::include::dav1d::headers::Dav1dWarpedMotionParams;

use crate::include::dav1d::headers::Dav1dFilmGrainData;
use crate::include::dav1d::headers::Dav1dSequenceHeader;

use crate::src::internal::Dav1dFrameContext_lf;
use crate::src::lf_mask::Av1Filter;
use crate::src::lf_mask::Av1FilterLUT;

use crate::src::internal::Dav1dFrameContext_frame_thread;

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
pub type const_left_pixel_row_2px = *const [pixel; 2];
pub type cdef_dir_fn =
    Option<unsafe extern "C" fn(*const pixel, ptrdiff_t, *mut libc::c_uint) -> libc::c_int>;
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
    ) -> (),
>;
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
use crate::src::lr_apply::LR_RESTORE_U;
use crate::src::lr_apply::LR_RESTORE_V;
use crate::src::lr_apply::LR_RESTORE_Y;

use crate::include::common::intops::imin;
unsafe extern "C" fn backup_lpf(
    f: *const Dav1dFrameContext,
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut src: *const pixel,
    src_stride: ptrdiff_t,
    ss_ver: libc::c_int,
    sb128: libc::c_int,
    mut row: libc::c_int,
    row_h: libc::c_int,
    src_w: libc::c_int,
    h: libc::c_int,
    ss_hor: libc::c_int,
    lr_backup: libc::c_int,
) {
    let cdef_backup = (lr_backup == 0) as libc::c_int;
    let dst_w = if (*(*f).frame_hdr).super_res.enabled != 0 {
        (*(*f).frame_hdr).width[1] + ss_hor >> ss_hor
    } else {
        src_w
    };
    let mut stripe_h =
        ((64 as libc::c_int) << (cdef_backup & sb128)) - 8 * (row == 0) as libc::c_int >> ss_ver;
    src = src.offset(((stripe_h - 2) as isize * src_stride) as isize);
    if (*(*f).c).n_tc == 1 as libc::c_uint {
        if row != 0 {
            let top = (4 as libc::c_int) << sb128;
            memcpy(
                &mut *dst.offset((dst_stride * 0isize) as isize) as *mut pixel as *mut libc::c_void,
                &mut *dst.offset((dst_stride * top as isize) as isize) as *mut pixel
                    as *const libc::c_void,
                dst_w as size_t,
            );
            memcpy(
                &mut *dst.offset((dst_stride * 1) as isize) as *mut pixel as *mut libc::c_void,
                &mut *dst.offset((dst_stride * (top + 1) as isize) as isize) as *mut pixel
                    as *const libc::c_void,
                dst_w as size_t,
            );
            memcpy(
                &mut *dst.offset((dst_stride * 2) as isize) as *mut pixel as *mut libc::c_void,
                &mut *dst.offset((dst_stride * (top + 2) as isize) as isize) as *mut pixel
                    as *const libc::c_void,
                dst_w as size_t,
            );
            memcpy(
                &mut *dst.offset((dst_stride * 3 as isize) as isize) as *mut pixel
                    as *mut libc::c_void,
                &mut *dst.offset((dst_stride * (top + 3) as isize) as isize) as *mut pixel
                    as *const libc::c_void,
                dst_w as size_t,
            );
        }
        dst = dst.offset((4 as libc::c_int as isize * dst_stride) as isize);
    }
    if lr_backup != 0 && (*(*f).frame_hdr).width[0] != (*(*f).frame_hdr).width[1] {
        while row + stripe_h <= row_h {
            let n_lines = 4 as libc::c_int - (row + stripe_h + 1 == h) as libc::c_int;
            ((*(*f).dsp).mc.resize).expect("non-null function pointer")(
                dst,
                dst_stride,
                src,
                src_stride,
                dst_w,
                n_lines,
                src_w,
                (*f).resize_step[ss_hor as usize],
                (*f).resize_start[ss_hor as usize],
            );
            row += stripe_h;
            stripe_h = 64 >> ss_ver;
            src = src.offset((stripe_h as isize * src_stride) as isize);
            dst = dst.offset((n_lines as isize * dst_stride) as isize);
            if n_lines == 3 {
                memcpy(
                    dst as *mut libc::c_void,
                    &mut *dst.offset(-dst_stride as isize) as *mut pixel as *const libc::c_void,
                    dst_w as size_t,
                );
                dst = dst.offset(dst_stride as isize);
            }
        }
    } else {
        while row + stripe_h <= row_h {
            let n_lines_0 = 4 as libc::c_int - (row + stripe_h + 1 == h) as libc::c_int;
            let mut i = 0;
            while i < 4 {
                memcpy(
                    dst as *mut libc::c_void,
                    (if i == n_lines_0 {
                        &mut *dst.offset(-dst_stride as isize) as *mut pixel as *const pixel
                    } else {
                        src
                    }) as *const libc::c_void,
                    src_w as size_t,
                );
                dst = dst.offset(dst_stride as isize);
                src = src.offset(src_stride as isize);
                i += 1;
            }
            row += stripe_h;
            stripe_h = 64 >> ss_ver;
            src = src.offset(((stripe_h - 4) as isize * src_stride) as isize);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_copy_lpf_8bpc(
    f: *mut Dav1dFrameContext,
    mut src: *const *mut pixel,
    sby: libc::c_int,
) {
    let have_tt = ((*(*f).c).n_tc > 1 as libc::c_uint) as libc::c_int;
    let resize = ((*(*f).frame_hdr).width[0] != (*(*f).frame_hdr).width[1]) as libc::c_int;
    let offset = 8 * (sby != 0) as libc::c_int;
    let src_stride: *const ptrdiff_t = ((*f).cur.stride).as_mut_ptr();
    let lr_stride: *const ptrdiff_t = ((*f).sr_cur.p.stride).as_mut_ptr();
    let tt_off = have_tt * sby * ((4 as libc::c_int) << (*(*f).seq_hdr).sb128);
    let dst: [*mut pixel; 3] = [
        ((*f).lf.lr_lpf_line[0] as *mut pixel)
            .offset((tt_off as isize * *lr_stride.offset(0)) as isize),
        ((*f).lf.lr_lpf_line[1] as *mut pixel)
            .offset((tt_off as isize * *lr_stride.offset(1)) as isize),
        ((*f).lf.lr_lpf_line[2] as *mut pixel)
            .offset((tt_off as isize * *lr_stride.offset(1)) as isize),
    ];
    let restore_planes = (*f).lf.restore_planes;
    if (*(*f).seq_hdr).cdef != 0 || restore_planes & LR_RESTORE_Y as libc::c_int != 0 {
        let h = (*f).cur.p.h;
        let w = (*f).bw << 2;
        let row_h = imin((sby + 1) << 6 + (*(*f).seq_hdr).sb128, h - 1);
        let y_stripe = (sby << 6 + (*(*f).seq_hdr).sb128) - offset;
        if restore_planes & LR_RESTORE_Y as libc::c_int != 0 || resize == 0 {
            backup_lpf(
                f,
                dst[0],
                *lr_stride.offset(0),
                (*src.offset(0)).offset(-((offset as isize * *src_stride.offset(0)) as isize)),
                *src_stride.offset(0),
                0 as libc::c_int,
                (*(*f).seq_hdr).sb128,
                y_stripe,
                row_h,
                w,
                h,
                0 as libc::c_int,
                1 as libc::c_int,
            );
        }
        if have_tt != 0 && resize != 0 {
            let cdef_off_y: ptrdiff_t = (sby * 4) as isize * *src_stride.offset(0);
            backup_lpf(
                f,
                ((*f).lf.cdef_lpf_line[0] as *mut pixel).offset(cdef_off_y as isize),
                *src_stride.offset(0),
                (*src.offset(0)).offset(-((offset as isize * *src_stride.offset(0)) as isize)),
                *src_stride.offset(0),
                0 as libc::c_int,
                (*(*f).seq_hdr).sb128,
                y_stripe,
                row_h,
                w,
                h,
                0 as libc::c_int,
                0 as libc::c_int,
            );
        }
    }
    if ((*(*f).seq_hdr).cdef != 0
        || restore_planes & (LR_RESTORE_U as libc::c_int | LR_RESTORE_V as libc::c_int) != 0)
        && (*f).cur.p.layout as libc::c_uint
            != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint
    {
        let ss_ver = ((*f).sr_cur.p.p.layout as libc::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint)
            as libc::c_int;
        let ss_hor = ((*f).sr_cur.p.p.layout as libc::c_uint
            != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint)
            as libc::c_int;
        let h_0 = (*f).cur.p.h + ss_ver >> ss_ver;
        let w_0 = (*f).bw << 2 - ss_hor;
        let row_h_0 = imin((sby + 1) << 6 - ss_ver + (*(*f).seq_hdr).sb128, h_0 - 1);
        let offset_uv = offset >> ss_ver;
        let y_stripe_0 = (sby << 6 - ss_ver + (*(*f).seq_hdr).sb128) - offset_uv;
        let cdef_off_uv: ptrdiff_t = (sby * 4) as isize * *src_stride.offset(1);
        if (*(*f).seq_hdr).cdef != 0 || restore_planes & LR_RESTORE_U as libc::c_int != 0 {
            if restore_planes & LR_RESTORE_U as libc::c_int != 0 || resize == 0 {
                backup_lpf(
                    f,
                    dst[1],
                    *lr_stride.offset(1),
                    (*src.offset(1))
                        .offset(-((offset_uv as isize * *src_stride.offset(1)) as isize)),
                    *src_stride.offset(1),
                    ss_ver,
                    (*(*f).seq_hdr).sb128,
                    y_stripe_0,
                    row_h_0,
                    w_0,
                    h_0,
                    ss_hor,
                    1 as libc::c_int,
                );
            }
            if have_tt != 0 && resize != 0 {
                backup_lpf(
                    f,
                    ((*f).lf.cdef_lpf_line[1] as *mut pixel).offset(cdef_off_uv as isize),
                    *src_stride.offset(1),
                    (*src.offset(1))
                        .offset(-((offset_uv as isize * *src_stride.offset(1)) as isize)),
                    *src_stride.offset(1),
                    ss_ver,
                    (*(*f).seq_hdr).sb128,
                    y_stripe_0,
                    row_h_0,
                    w_0,
                    h_0,
                    ss_hor,
                    0 as libc::c_int,
                );
            }
        }
        if (*(*f).seq_hdr).cdef != 0 || restore_planes & LR_RESTORE_V as libc::c_int != 0 {
            if restore_planes & LR_RESTORE_V as libc::c_int != 0 || resize == 0 {
                backup_lpf(
                    f,
                    dst[2],
                    *lr_stride.offset(1),
                    (*src.offset(2))
                        .offset(-((offset_uv as isize * *src_stride.offset(1)) as isize)),
                    *src_stride.offset(1),
                    ss_ver,
                    (*(*f).seq_hdr).sb128,
                    y_stripe_0,
                    row_h_0,
                    w_0,
                    h_0,
                    ss_hor,
                    1 as libc::c_int,
                );
            }
            if have_tt != 0 && resize != 0 {
                backup_lpf(
                    f,
                    ((*f).lf.cdef_lpf_line[2] as *mut pixel).offset(cdef_off_uv as isize),
                    *src_stride.offset(1),
                    (*src.offset(2))
                        .offset(-((offset_uv as isize * *src_stride.offset(1)) as isize)),
                    *src_stride.offset(1),
                    ss_ver,
                    (*(*f).seq_hdr).sb128,
                    y_stripe_0,
                    row_h_0,
                    w_0,
                    h_0,
                    ss_hor,
                    0 as libc::c_int,
                );
            }
        }
    }
}
#[inline]
unsafe extern "C" fn filter_plane_cols_y(
    f: *const Dav1dFrameContext,
    have_left: libc::c_int,
    mut lvl: *const [uint8_t; 4],
    b4_stride: ptrdiff_t,
    mask: *const [[uint16_t; 2]; 3],
    mut dst: *mut pixel,
    ls: ptrdiff_t,
    w: libc::c_int,
    starty4: libc::c_int,
    endy4: libc::c_int,
) {
    let dsp: *const Dav1dDSPContext = (*f).dsp;
    let mut x = 0;
    while x < w {
        if !(have_left == 0 && x == 0) {
            let mut hmask: [uint32_t; 4] = [0; 4];
            if starty4 == 0 {
                hmask[0] = (*mask.offset(x as isize))[0][0] as uint32_t;
                hmask[1] = (*mask.offset(x as isize))[1][0] as uint32_t;
                hmask[2] = (*mask.offset(x as isize))[2][0] as uint32_t;
                if endy4 > 16 {
                    hmask[0] |= ((*mask.offset(x as isize))[0][1] as libc::c_uint) << 16;
                    hmask[1] |= ((*mask.offset(x as isize))[1][1] as libc::c_uint) << 16;
                    hmask[2] |= ((*mask.offset(x as isize))[2][1] as libc::c_uint) << 16;
                }
            } else {
                hmask[0] = (*mask.offset(x as isize))[0][1] as uint32_t;
                hmask[1] = (*mask.offset(x as isize))[1][1] as uint32_t;
                hmask[2] = (*mask.offset(x as isize))[2][1] as uint32_t;
            }
            hmask[3] = 0 as libc::c_int as uint32_t;
            ((*dsp).lf.loop_filter_sb[0][0]).expect("non-null function pointer")(
                &mut *dst.offset((x * 4) as isize),
                ls,
                hmask.as_mut_ptr(),
                &*(*lvl.offset(x as isize)).as_ptr().offset(0) as *const uint8_t
                    as *const [uint8_t; 4],
                b4_stride,
                &(*f).lf.lim_lut.0,
                endy4 - starty4,
            );
        }
        x += 1;
    }
}
#[inline]
unsafe extern "C" fn filter_plane_rows_y(
    f: *const Dav1dFrameContext,
    have_top: libc::c_int,
    mut lvl: *const [uint8_t; 4],
    b4_stride: ptrdiff_t,
    mask: *const [[uint16_t; 2]; 3],
    mut dst: *mut pixel,
    ls: ptrdiff_t,
    w: libc::c_int,
    starty4: libc::c_int,
    endy4: libc::c_int,
) {
    let dsp: *const Dav1dDSPContext = (*f).dsp;
    let mut y = starty4;
    while y < endy4 {
        if !(have_top == 0 && y == 0) {
            let vmask: [uint32_t; 4] = [
                (*mask.offset(y as isize))[0][0] as libc::c_uint
                    | ((*mask.offset(y as isize))[0][1] as libc::c_uint) << 16,
                (*mask.offset(y as isize))[1][0] as libc::c_uint
                    | ((*mask.offset(y as isize))[1][1] as libc::c_uint) << 16,
                (*mask.offset(y as isize))[2][0] as libc::c_uint
                    | ((*mask.offset(y as isize))[2][1] as libc::c_uint) << 16,
                0 as libc::c_int as uint32_t,
            ];
            ((*dsp).lf.loop_filter_sb[0][1]).expect("non-null function pointer")(
                dst,
                ls,
                vmask.as_ptr(),
                &*(*lvl.offset(0)).as_ptr().offset(1) as *const uint8_t as *const [uint8_t; 4],
                b4_stride,
                &(*f).lf.lim_lut.0,
                w,
            );
        }
        y += 1;
        dst = dst.offset((4 * ls) as isize);
        lvl = lvl.offset(b4_stride as isize);
    }
}
#[inline]
unsafe extern "C" fn filter_plane_cols_uv(
    f: *const Dav1dFrameContext,
    have_left: libc::c_int,
    mut lvl: *const [uint8_t; 4],
    b4_stride: ptrdiff_t,
    mask: *const [[uint16_t; 2]; 2],
    u: *mut pixel,
    v: *mut pixel,
    ls: ptrdiff_t,
    w: libc::c_int,
    starty4: libc::c_int,
    endy4: libc::c_int,
    ss_ver: libc::c_int,
) {
    let dsp: *const Dav1dDSPContext = (*f).dsp;
    let mut x = 0;
    while x < w {
        if !(have_left == 0 && x == 0) {
            let mut hmask: [uint32_t; 3] = [0; 3];
            if starty4 == 0 {
                hmask[0] = (*mask.offset(x as isize))[0][0] as uint32_t;
                hmask[1] = (*mask.offset(x as isize))[1][0] as uint32_t;
                if endy4 > 16 >> ss_ver {
                    hmask[0] |=
                        ((*mask.offset(x as isize))[0][1] as libc::c_uint) << (16 >> ss_ver);
                    hmask[1] |=
                        ((*mask.offset(x as isize))[1][1] as libc::c_uint) << (16 >> ss_ver);
                }
            } else {
                hmask[0] = (*mask.offset(x as isize))[0][1] as uint32_t;
                hmask[1] = (*mask.offset(x as isize))[1][1] as uint32_t;
            }
            hmask[2] = 0 as libc::c_int as uint32_t;
            ((*dsp).lf.loop_filter_sb[1][0]).expect("non-null function pointer")(
                &mut *u.offset((x * 4) as isize),
                ls,
                hmask.as_mut_ptr(),
                &*(*lvl.offset(x as isize)).as_ptr().offset(2) as *const uint8_t
                    as *const [uint8_t; 4],
                b4_stride,
                &(*f).lf.lim_lut.0,
                endy4 - starty4,
            );
            ((*dsp).lf.loop_filter_sb[1][0]).expect("non-null function pointer")(
                &mut *v.offset((x * 4) as isize),
                ls,
                hmask.as_mut_ptr(),
                &*(*lvl.offset(x as isize)).as_ptr().offset(3) as *const uint8_t
                    as *const [uint8_t; 4],
                b4_stride,
                &(*f).lf.lim_lut.0,
                endy4 - starty4,
            );
        }
        x += 1;
    }
}
#[inline]
unsafe extern "C" fn filter_plane_rows_uv(
    f: *const Dav1dFrameContext,
    have_top: libc::c_int,
    mut lvl: *const [uint8_t; 4],
    b4_stride: ptrdiff_t,
    mask: *const [[uint16_t; 2]; 2],
    u: *mut pixel,
    v: *mut pixel,
    ls: ptrdiff_t,
    w: libc::c_int,
    starty4: libc::c_int,
    endy4: libc::c_int,
    ss_hor: libc::c_int,
) {
    let dsp: *const Dav1dDSPContext = (*f).dsp;
    let mut off_l: ptrdiff_t = 0 as libc::c_int as ptrdiff_t;
    let mut y = starty4;
    while y < endy4 {
        if !(have_top == 0 && y == 0) {
            let vmask: [uint32_t; 3] = [
                (*mask.offset(y as isize))[0][0] as libc::c_uint
                    | ((*mask.offset(y as isize))[0][1] as libc::c_uint) << (16 >> ss_hor),
                (*mask.offset(y as isize))[1][0] as libc::c_uint
                    | ((*mask.offset(y as isize))[1][1] as libc::c_uint) << (16 >> ss_hor),
                0 as libc::c_int as uint32_t,
            ];
            ((*dsp).lf.loop_filter_sb[1][1]).expect("non-null function pointer")(
                &mut *u.offset(off_l as isize),
                ls,
                vmask.as_ptr(),
                &*(*lvl.offset(0)).as_ptr().offset(2) as *const uint8_t as *const [uint8_t; 4],
                b4_stride,
                &(*f).lf.lim_lut.0,
                w,
            );
            ((*dsp).lf.loop_filter_sb[1][1]).expect("non-null function pointer")(
                &mut *v.offset(off_l as isize),
                ls,
                vmask.as_ptr(),
                &*(*lvl.offset(0)).as_ptr().offset(3) as *const uint8_t as *const [uint8_t; 4],
                b4_stride,
                &(*f).lf.lim_lut.0,
                w,
            );
        }
        y += 1;
        off_l += 4 * ls;
        lvl = lvl.offset(b4_stride as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_loopfilter_sbrow_cols_8bpc(
    f: *const Dav1dFrameContext,
    mut p: *const *mut pixel,
    lflvl: *mut Av1Filter,
    mut sby: libc::c_int,
    start_of_tile_row: libc::c_int,
) {
    let mut x = 0;
    let mut have_left = 0;
    let is_sb64 = ((*(*f).seq_hdr).sb128 == 0) as libc::c_int;
    let starty4 = (sby & is_sb64) << 4;
    let sbsz = 32 >> is_sb64;
    let sbl2 = 5 - is_sb64;
    let halign = (*f).bh + 31 & !(31 as libc::c_int);
    let ss_ver = ((*f).cur.p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_hor = ((*f).cur.p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
    let vmask = 16 >> ss_ver;
    let hmask = 16 >> ss_hor;
    let vmax: libc::c_uint = (1 as libc::c_uint) << vmask;
    let hmax: libc::c_uint = (1 as libc::c_uint) << hmask;
    let endy4: libc::c_uint = (starty4 + imin((*f).h4 - sby * sbsz, sbsz)) as libc::c_uint;
    let uv_endy4: libc::c_uint = endy4.wrapping_add(ss_ver as libc::c_uint) >> ss_ver;
    let mut lpf_y: *const uint8_t = &mut *(*((*f).lf.tx_lpf_right_edge).as_ptr().offset(0))
        .offset((sby << sbl2) as isize) as *mut uint8_t;
    let mut lpf_uv: *const uint8_t = &mut *(*((*f).lf.tx_lpf_right_edge).as_ptr().offset(1))
        .offset((sby << sbl2 - ss_ver) as isize)
        as *mut uint8_t;
    let mut tile_col = 1;
    loop {
        x = (*(*f).frame_hdr).tiling.col_start_sb[tile_col as usize] as libc::c_int;
        if x << sbl2 >= (*f).bw {
            break;
        }
        let bx4 = if x & is_sb64 != 0 {
            16 as libc::c_int
        } else {
            0 as libc::c_int
        };
        let cbx4 = bx4 >> ss_hor;
        x >>= is_sb64;
        let y_hmask: *mut [uint16_t; 2] =
            ((*lflvl.offset(x as isize)).filter_y[0][bx4 as usize]).as_mut_ptr();
        let mut y: libc::c_uint = starty4 as libc::c_uint;
        let mut mask: libc::c_uint = ((1 as libc::c_int) << y) as libc::c_uint;
        while y < endy4 {
            let sidx = (mask >= 0x10000 as libc::c_uint) as libc::c_int;
            let smask: libc::c_uint = mask >> (sidx << 4);
            let idx = 2 as libc::c_int
                * ((*y_hmask.offset(2))[sidx as usize] as libc::c_uint & smask != 0) as libc::c_int
                + ((*y_hmask.offset(1))[sidx as usize] as libc::c_uint & smask != 0) as libc::c_int;
            let ref mut fresh0 = (*y_hmask.offset(2))[sidx as usize];
            *fresh0 = (*fresh0 as libc::c_uint & !smask) as uint16_t;
            let ref mut fresh1 = (*y_hmask.offset(1))[sidx as usize];
            *fresh1 = (*fresh1 as libc::c_uint & !smask) as uint16_t;
            let ref mut fresh2 = (*y_hmask.offset(0))[sidx as usize];
            *fresh2 = (*fresh2 as libc::c_uint & !smask) as uint16_t;
            let ref mut fresh3 = (*y_hmask.offset(imin(
                idx,
                *lpf_y.offset(y.wrapping_sub(starty4 as libc::c_uint) as isize) as libc::c_int,
            ) as isize))[sidx as usize];
            *fresh3 = (*fresh3 as libc::c_uint | smask) as uint16_t;
            y = y.wrapping_add(1);
            mask <<= 1;
        }
        if (*f).cur.p.layout as libc::c_uint
            != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint
        {
            let uv_hmask: *mut [uint16_t; 2] =
                ((*lflvl.offset(x as isize)).filter_uv[0][cbx4 as usize]).as_mut_ptr();
            let mut y_0: libc::c_uint = (starty4 >> ss_ver) as libc::c_uint;
            let mut uv_mask: libc::c_uint = ((1 as libc::c_int) << y_0) as libc::c_uint;
            while y_0 < uv_endy4 {
                let sidx_0 = (uv_mask >= vmax) as libc::c_int;
                let smask_0: libc::c_uint = uv_mask >> (sidx_0 << 4 - ss_ver);
                let idx_0 = ((*uv_hmask.offset(1))[sidx_0 as usize] as libc::c_uint & smask_0 != 0)
                    as libc::c_int;
                let ref mut fresh4 = (*uv_hmask.offset(1))[sidx_0 as usize];
                *fresh4 = (*fresh4 as libc::c_uint & !smask_0) as uint16_t;
                let ref mut fresh5 = (*uv_hmask.offset(0))[sidx_0 as usize];
                *fresh5 = (*fresh5 as libc::c_uint & !smask_0) as uint16_t;
                let ref mut fresh6 = (*uv_hmask.offset(imin(
                    idx_0,
                    *lpf_uv.offset(y_0.wrapping_sub((starty4 >> ss_ver) as libc::c_uint) as isize)
                        as libc::c_int,
                ) as isize))[sidx_0 as usize];
                *fresh6 = (*fresh6 as libc::c_uint | smask_0) as uint16_t;
                y_0 = y_0.wrapping_add(1);
                uv_mask <<= 1;
            }
        }
        lpf_y = lpf_y.offset(halign as isize);
        lpf_uv = lpf_uv.offset((halign >> ss_ver) as isize);
        tile_col += 1;
    }
    if start_of_tile_row != 0 {
        let mut a: *const BlockContext = 0 as *const BlockContext;
        x = 0 as libc::c_int;
        a = &mut *((*f).a).offset(((*f).sb128w * (start_of_tile_row - 1)) as isize)
            as *mut BlockContext;
        while x < (*f).sb128w {
            let y_vmask: *mut [uint16_t; 2] =
                ((*lflvl.offset(x as isize)).filter_y[1][starty4 as usize]).as_mut_ptr();
            let w: libc::c_uint = imin(32 as libc::c_int, (*f).w4 - (x << 5)) as libc::c_uint;
            let mut mask_0: libc::c_uint = 1 as libc::c_int as libc::c_uint;
            let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while i < w {
                let sidx_1 = (mask_0 >= 0x10000 as libc::c_uint) as libc::c_int;
                let smask_1: libc::c_uint = mask_0 >> (sidx_1 << 4);
                let idx_1 = 2 as libc::c_int
                    * ((*y_vmask.offset(2))[sidx_1 as usize] as libc::c_uint & smask_1 != 0)
                        as libc::c_int
                    + ((*y_vmask.offset(1))[sidx_1 as usize] as libc::c_uint & smask_1 != 0)
                        as libc::c_int;
                let ref mut fresh7 = (*y_vmask.offset(2))[sidx_1 as usize];
                *fresh7 = (*fresh7 as libc::c_uint & !smask_1) as uint16_t;
                let ref mut fresh8 = (*y_vmask.offset(1))[sidx_1 as usize];
                *fresh8 = (*fresh8 as libc::c_uint & !smask_1) as uint16_t;
                let ref mut fresh9 = (*y_vmask.offset(0))[sidx_1 as usize];
                *fresh9 = (*fresh9 as libc::c_uint & !smask_1) as uint16_t;
                let ref mut fresh10 = (*y_vmask
                    .offset(imin(idx_1, (*a).tx_lpf_y[i as usize] as libc::c_int) as isize))
                    [sidx_1 as usize];
                *fresh10 = (*fresh10 as libc::c_uint | smask_1) as uint16_t;
                mask_0 <<= 1;
                i = i.wrapping_add(1);
            }
            if (*f).cur.p.layout as libc::c_uint
                != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint
            {
                let cw: libc::c_uint = w.wrapping_add(ss_hor as libc::c_uint) >> ss_hor;
                let uv_vmask: *mut [uint16_t; 2] = ((*lflvl.offset(x as isize)).filter_uv[1]
                    [(starty4 >> ss_ver) as usize])
                    .as_mut_ptr();
                let mut uv_mask_0: libc::c_uint = 1 as libc::c_int as libc::c_uint;
                let mut i_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                while i_0 < cw {
                    let sidx_2 = (uv_mask_0 >= hmax) as libc::c_int;
                    let smask_2: libc::c_uint = uv_mask_0 >> (sidx_2 << 4 - ss_hor);
                    let idx_2 = ((*uv_vmask.offset(1))[sidx_2 as usize] as libc::c_uint & smask_2
                        != 0) as libc::c_int;
                    let ref mut fresh11 = (*uv_vmask.offset(1))[sidx_2 as usize];
                    *fresh11 = (*fresh11 as libc::c_uint & !smask_2) as uint16_t;
                    let ref mut fresh12 = (*uv_vmask.offset(0))[sidx_2 as usize];
                    *fresh12 = (*fresh12 as libc::c_uint & !smask_2) as uint16_t;
                    let ref mut fresh13 = (*uv_vmask
                        .offset(imin(idx_2, (*a).tx_lpf_uv[i_0 as usize] as libc::c_int) as isize))
                        [sidx_2 as usize];
                    *fresh13 = (*fresh13 as libc::c_uint | smask_2) as uint16_t;
                    uv_mask_0 <<= 1;
                    i_0 = i_0.wrapping_add(1);
                }
            }
            x += 1;
            a = a.offset(1);
        }
    }
    let mut ptr: *mut pixel = 0 as *mut pixel;
    let mut level_ptr: *mut [uint8_t; 4] =
        ((*f).lf.level).offset(((*f).b4_stride * sby as isize * sbsz as isize) as isize);
    ptr = *p.offset(0);
    have_left = 0 as libc::c_int;
    x = 0 as libc::c_int;
    while x < (*f).sb128w {
        filter_plane_cols_y(
            f,
            have_left,
            level_ptr as *const [uint8_t; 4],
            (*f).b4_stride,
            ((*lflvl.offset(x as isize)).filter_y[0]).as_mut_ptr() as *const [[uint16_t; 2]; 3],
            ptr,
            (*f).cur.stride[0],
            imin(32 as libc::c_int, (*f).w4 - x * 32),
            starty4,
            endy4 as libc::c_int,
        );
        x += 1;
        have_left = 1 as libc::c_int;
        ptr = ptr.offset(128);
        level_ptr = level_ptr.offset(32);
    }
    if (*(*f).frame_hdr).loopfilter.level_u == 0 && (*(*f).frame_hdr).loopfilter.level_v == 0 {
        return;
    }
    let mut uv_off: ptrdiff_t = 0;
    level_ptr = ((*f).lf.level).offset(((*f).b4_stride * (sby * sbsz >> ss_ver) as isize) as isize);
    uv_off = 0 as libc::c_int as ptrdiff_t;
    have_left = 0 as libc::c_int;
    x = 0 as libc::c_int;
    while x < (*f).sb128w {
        filter_plane_cols_uv(
            f,
            have_left,
            level_ptr as *const [uint8_t; 4],
            (*f).b4_stride,
            ((*lflvl.offset(x as isize)).filter_uv[0]).as_mut_ptr() as *const [[uint16_t; 2]; 2],
            &mut *(*p.offset(1)).offset(uv_off as isize),
            &mut *(*p.offset(2)).offset(uv_off as isize),
            (*f).cur.stride[1],
            imin(32 as libc::c_int, (*f).w4 - x * 32) + ss_hor >> ss_hor,
            starty4 >> ss_ver,
            uv_endy4 as libc::c_int,
            ss_ver,
        );
        x += 1;
        have_left = 1 as libc::c_int;
        uv_off += (128 >> ss_hor) as isize;
        level_ptr = level_ptr.offset((32 >> ss_hor) as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_loopfilter_sbrow_rows_8bpc(
    f: *const Dav1dFrameContext,
    mut p: *const *mut pixel,
    lflvl: *mut Av1Filter,
    mut sby: libc::c_int,
) {
    let mut x = 0;
    let have_top = (sby > 0) as libc::c_int;
    let is_sb64 = ((*(*f).seq_hdr).sb128 == 0) as libc::c_int;
    let starty4 = (sby & is_sb64) << 4;
    let sbsz = 32 >> is_sb64;
    let ss_ver = ((*f).cur.p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_hor = ((*f).cur.p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
    let endy4: libc::c_uint = (starty4 + imin((*f).h4 - sby * sbsz, sbsz)) as libc::c_uint;
    let uv_endy4: libc::c_uint = endy4.wrapping_add(ss_ver as libc::c_uint) >> ss_ver;
    let mut ptr: *mut pixel = 0 as *mut pixel;
    let mut level_ptr: *mut [uint8_t; 4] =
        ((*f).lf.level).offset(((*f).b4_stride * sby as isize * sbsz as isize) as isize);
    ptr = *p.offset(0);
    x = 0 as libc::c_int;
    while x < (*f).sb128w {
        filter_plane_rows_y(
            f,
            have_top,
            level_ptr as *const [uint8_t; 4],
            (*f).b4_stride,
            ((*lflvl.offset(x as isize)).filter_y[1]).as_mut_ptr() as *const [[uint16_t; 2]; 3],
            ptr,
            (*f).cur.stride[0],
            imin(32 as libc::c_int, (*f).w4 - x * 32),
            starty4,
            endy4 as libc::c_int,
        );
        x += 1;
        ptr = ptr.offset(128);
        level_ptr = level_ptr.offset(32);
    }
    if (*(*f).frame_hdr).loopfilter.level_u == 0 && (*(*f).frame_hdr).loopfilter.level_v == 0 {
        return;
    }
    let mut uv_off: ptrdiff_t = 0;
    level_ptr = ((*f).lf.level).offset(((*f).b4_stride * (sby * sbsz >> ss_ver) as isize) as isize);
    uv_off = 0 as libc::c_int as ptrdiff_t;
    x = 0 as libc::c_int;
    while x < (*f).sb128w {
        filter_plane_rows_uv(
            f,
            have_top,
            level_ptr as *const [uint8_t; 4],
            (*f).b4_stride,
            ((*lflvl.offset(x as isize)).filter_uv[1]).as_mut_ptr() as *const [[uint16_t; 2]; 2],
            &mut *(*p.offset(1)).offset(uv_off as isize),
            &mut *(*p.offset(2)).offset(uv_off as isize),
            (*f).cur.stride[1],
            imin(32 as libc::c_int, (*f).w4 - x * 32) + ss_hor >> ss_hor,
            starty4 >> ss_ver,
            uv_endy4 as libc::c_int,
            ss_hor,
        );
        x += 1;
        uv_off += (128 >> ss_hor) as isize;
        level_ptr = level_ptr.offset((32 >> ss_hor) as isize);
    }
}
