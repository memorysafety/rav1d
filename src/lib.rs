use crate::include::stddef::*;
use crate::include::stdint::*;

use crate::stderr;
use ::libc;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: size_t) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: size_t) -> *mut libc::c_void;
    fn dlsym(__handle: *mut libc::c_void, __name: *const libc::c_char) -> *mut libc::c_void;
    fn calloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn posix_memalign(
        __memptr: *mut *mut libc::c_void,
        __alignment: size_t,
        __size: size_t,
    ) -> libc::c_int;
    fn abort() -> !;
    fn fprintf(_: *mut libc::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn dav1d_init_cpu();
    fn dav1d_num_logical_processors(c: *mut Dav1dContext) -> libc::c_int;
    #[cfg(feature = "bitdepth_16")]
    fn dav1d_apply_grain_16bpc(
        dsp: *const Dav1dFilmGrainDSPContext,
        out: *mut Dav1dPicture,
        in_0: *const Dav1dPicture,
    );
    #[cfg(feature = "bitdepth_8")]
    fn dav1d_apply_grain_8bpc(
        dsp: *const Dav1dFilmGrainDSPContext,
        out: *mut Dav1dPicture,
        in_0: *const Dav1dPicture,
    );
    fn dav1d_data_props_unref_internal(props: *mut Dav1dDataProps);
    fn dav1d_picture_unref_internal(p: *mut Dav1dPicture);
    fn dav1d_data_create_internal(buf: *mut Dav1dData, sz: size_t) -> *mut uint8_t;
    fn dav1d_data_props_copy(dst: *mut Dav1dDataProps, src: *const Dav1dDataProps);
    fn dav1d_data_wrap_internal(
        buf: *mut Dav1dData,
        ptr: *const uint8_t,
        sz: size_t,
        free_callback: Option<unsafe extern "C" fn(*const uint8_t, *mut libc::c_void) -> ()>,
        user_data: *mut libc::c_void,
    ) -> libc::c_int;
    fn dav1d_thread_picture_ref(dst: *mut Dav1dThreadPicture, src: *const Dav1dThreadPicture);
    fn dav1d_picture_get_event_flags(p: *const Dav1dThreadPicture) -> Dav1dEventFlags;
    fn dav1d_picture_move_ref(dst: *mut Dav1dPicture, src: *mut Dav1dPicture);
    fn dav1d_data_wrap_user_data_internal(
        buf: *mut Dav1dData,
        user_data: *const uint8_t,
        free_callback: Option<unsafe extern "C" fn(*const uint8_t, *mut libc::c_void) -> ()>,
        cookie: *mut libc::c_void,
    ) -> libc::c_int;
    fn dav1d_picture_ref(dst: *mut Dav1dPicture, src: *const Dav1dPicture);
    fn dav1d_data_unref_internal(buf: *mut Dav1dData);
    fn dav1d_picture_alloc_copy(
        c: *mut Dav1dContext,
        dst: *mut Dav1dPicture,
        w: libc::c_int,
        src: *const Dav1dPicture,
    ) -> libc::c_int;
    fn dav1d_data_ref(dst: *mut Dav1dData, src: *const Dav1dData);
    fn dav1d_thread_picture_move_ref(dst: *mut Dav1dThreadPicture, src: *mut Dav1dThreadPicture);
    fn pthread_attr_init(__attr: *mut pthread_attr_t) -> libc::c_int;
    fn __sysconf(__name: libc::c_int) -> libc::c_long;
    fn dav1d_data_props_set_defaults(props: *mut Dav1dDataProps);
    fn dav1d_mem_pool_init(pool: *mut *mut Dav1dMemPool) -> libc::c_int;
    fn dav1d_refmvs_init(rf: *mut refmvs_frame);
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn dav1d_refmvs_dsp_init(dsp: *mut Dav1dRefmvsDSPContext);
    fn dav1d_init_mode_tree(root: *mut EdgeNode, nt: *mut EdgeTip, allow_sb128: libc::c_int);
    fn pthread_join(__th: pthread_t, __thread_return: *mut *mut libc::c_void) -> libc::c_int;
    fn dav1d_refmvs_clear(rf: *mut refmvs_frame);
    fn dav1d_cdf_thread_unref(cdf: *mut CdfThreadContext);
    fn dav1d_thread_picture_unref(p: *mut Dav1dThreadPicture);
    fn dav1d_ref_dec(r#ref: *mut *mut Dav1dRef);
    fn dav1d_mem_pool_end(pool: *mut Dav1dMemPool);
    fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> libc::c_int;
    fn dav1d_default_picture_alloc(p: *mut Dav1dPicture, cookie: *mut libc::c_void) -> libc::c_int;
    fn dav1d_default_picture_release(p: *mut Dav1dPicture, cookie: *mut libc::c_void);
    fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: Option<unsafe extern "C" fn() -> ()>,
    ) -> libc::c_int;
    fn pthread_attr_setstacksize(__attr: *mut pthread_attr_t, __stacksize: size_t) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_wait(__cond: *mut pthread_cond_t, __mutex: *mut pthread_mutex_t)
        -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn dav1d_log_default_callback(
        cookie: *mut libc::c_void,
        format: *const libc::c_char,
        ap: ::core::ffi::VaList,
    );
    fn dav1d_log(c: *mut Dav1dContext, format: *const libc::c_char, _: ...);
    fn dav1d_parse_obus(
        c: *mut Dav1dContext,
        in_0: *mut Dav1dData,
        global: libc::c_int,
    ) -> libc::c_int;
    fn dav1d_init_qm_tables();
    fn dav1d_task_delayed_fg(
        c: *mut Dav1dContext,
        out: *mut Dav1dPicture,
        in_0: *const Dav1dPicture,
    );
    fn dav1d_worker_task(data: *mut libc::c_void) -> *mut libc::c_void;
    fn dav1d_decode_frame_exit(f: *mut Dav1dFrameContext, retval: libc::c_int);
    fn dav1d_init_wedge_masks();
    fn dav1d_init_interintra_masks();
}
use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::dav1d::common::Dav1dUserData;
use crate::include::stdatomic::atomic_int;
use crate::src::r#ref::Dav1dRef;

use crate::include::dav1d::headers::Dav1dWarpedMotionParams;

use crate::include::dav1d::headers::Dav1dContentLightLevel;
use crate::include::dav1d::headers::Dav1dITUTT35;
use crate::include::dav1d::headers::Dav1dMasteringDisplay;
use crate::include::dav1d::headers::Dav1dSequenceHeader;

use crate::include::dav1d::headers::Dav1dFilmGrainData;
use crate::include::dav1d::headers::Dav1dFrameHeader;

use crate::include::dav1d::data::Dav1dData;
use crate::include::dav1d::picture::Dav1dPicAllocator;
use crate::include::dav1d::picture::Dav1dPicture;
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

use libc::pthread_mutex_t;

use crate::include::dav1d::dav1d::Dav1dDecodeFrameType;
use crate::include::dav1d::dav1d::Dav1dEventFlags;
use crate::include::dav1d::dav1d::Dav1dInloopFilterType;
use crate::include::dav1d::dav1d::Dav1dLogger;
use crate::include::dav1d::dav1d::DAV1D_DECODEFRAMETYPE_ALL;
use crate::include::dav1d::dav1d::DAV1D_DECODEFRAMETYPE_KEY;
use crate::include::dav1d::dav1d::DAV1D_INLOOPFILTER_ALL;
use crate::src::picture::PictureFlags;
use crate::src::picture::PICTURE_FLAG_NEW_TEMPORAL_UNIT;

use crate::include::dav1d::dav1d::DAV1D_INLOOPFILTER_NONE;
use crate::src::internal::Dav1dContext_intra_edge;
use crate::src::intra_edge::EdgeFlags;
use crate::src::intra_edge::EdgeNode;
use crate::src::intra_edge::EdgeTip;

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

pub type pixel = ();
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
use crate::src::lf_mask::Av1FilterLUT;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dInvTxfmDSPContext {
    pub itxfm_add: [[itxfm_fn; 17]; 19],
}
pub type itxfm_fn = Option<
    unsafe extern "C" fn(*mut libc::c_void, ptrdiff_t, *mut libc::c_void, libc::c_int) -> (),
>;
pub type coef = ();
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
use crate::include::stdatomic::atomic_uint;
use crate::src::cdf::CdfThreadContext;

use crate::include::pthread::pthread_cond_t;
use crate::src::internal::Dav1dContext_frame_thread;
use crate::src::internal::Dav1dContext_refs;
use crate::src::internal::Dav1dTileGroup;
use crate::src::internal::TaskThreadData;
use crate::src::picture::Dav1dThreadPicture;
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
use crate::src::internal::FrameTileThreadData;

use crate::include::pthread::pthread_t;
use crate::src::internal::Dav1dTaskContext_frame_thread;
use crate::src::levels::Filter2d;

use crate::src::internal::Dav1dTaskContext_cf;
use crate::src::internal::Dav1dTaskContext_scratch;
use crate::src::lf_mask::Av1Filter;
use crate::src::refmvs::refmvs_tile;

use crate::src::env::BlockContext;
use crate::src::internal::Dav1dTileState;
use crate::src::refmvs::refmvs_frame;
use crate::src::refmvs::refmvs_temporal_block;

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

use crate::src::internal::Dav1dTask;

use crate::src::internal::Dav1dFrameContext_lf;

use crate::src::internal::CodedBlockInfo;
use crate::src::internal::Dav1dFrameContext_frame_thread;
use crate::src::levels::Av1Block;
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

pub type backup_ipred_edge_fn = Option<unsafe extern "C" fn(*mut Dav1dTaskContext) -> ()>;
pub type filter_sbrow_fn = Option<unsafe extern "C" fn(*mut Dav1dFrameContext, libc::c_int) -> ()>;
pub type recon_b_inter_fn =
    Option<unsafe extern "C" fn(*mut Dav1dTaskContext, BlockSize, *const Av1Block) -> libc::c_int>;
pub type recon_b_intra_fn = Option<
    unsafe extern "C" fn(*mut Dav1dTaskContext, BlockSize, EdgeFlags, *const Av1Block) -> (),
>;
use crate::src::internal::ScalableMotionParams;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dSettings {
    pub n_threads: libc::c_int,
    pub max_frame_delay: libc::c_int,
    pub apply_grain: libc::c_int,
    pub operating_point: libc::c_int,
    pub all_layers: libc::c_int,
    pub frame_size_limit: libc::c_uint,
    pub allocator: Dav1dPicAllocator,
    pub logger: Dav1dLogger,
    pub strict_std_compliance: libc::c_int,
    pub output_invisible_frames: libc::c_int,
    pub inloop_filters: Dav1dInloopFilterType,
    pub decode_frame_type: Dav1dDecodeFrameType,
    pub reserved: [uint8_t; 16],
}
use crate::include::pthread::pthread_attr_t;
use crate::include::pthread::pthread_condattr_t;
use crate::include::pthread::pthread_mutexattr_t;
use crate::src::levels::BL_128X128;
use crate::src::levels::BL_64X64;

use crate::include::common::intops::iclip;
use crate::include::common::intops::umin;
use crate::include::pthread::pthread_once_t;
use crate::src::mem::dav1d_alloc_aligned;
use crate::src::mem::dav1d_free_aligned;
use crate::src::mem::dav1d_freep_aligned;
use crate::src::mem::freep;
#[cold]
unsafe extern "C" fn init_internal() {
    dav1d_init_cpu();
    dav1d_init_interintra_masks();
    dav1d_init_qm_tables();
    dav1d_init_wedge_masks();
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_version() -> *const libc::c_char {
    return b"1.0.0-130-g26eca15\0" as *const u8 as *const libc::c_char;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_default_settings(s: *mut Dav1dSettings) {
    (*s).n_threads = 0 as libc::c_int;
    (*s).max_frame_delay = 0 as libc::c_int;
    (*s).apply_grain = 1 as libc::c_int;
    (*s).allocator.cookie = 0 as *mut libc::c_void;
    (*s).allocator.alloc_picture_callback = Some(
        dav1d_default_picture_alloc
            as unsafe extern "C" fn(*mut Dav1dPicture, *mut libc::c_void) -> libc::c_int,
    );
    (*s).allocator.release_picture_callback = Some(
        dav1d_default_picture_release
            as unsafe extern "C" fn(*mut Dav1dPicture, *mut libc::c_void) -> (),
    );
    (*s).logger.cookie = 0 as *mut libc::c_void;
    (*s).logger.callback = Some(
        dav1d_log_default_callback
            as unsafe extern "C" fn(
                *mut libc::c_void,
                *const libc::c_char,
                ::core::ffi::VaList,
            ) -> (),
    );
    (*s).operating_point = 0 as libc::c_int;
    (*s).all_layers = 1 as libc::c_int;
    (*s).frame_size_limit = 0 as libc::c_int as libc::c_uint;
    (*s).strict_std_compliance = 0 as libc::c_int;
    (*s).output_invisible_frames = 0 as libc::c_int;
    (*s).inloop_filters = DAV1D_INLOOPFILTER_ALL;
    (*s).decode_frame_type = DAV1D_DECODEFRAMETYPE_ALL;
}
#[cold]
unsafe extern "C" fn get_stack_size_internal(thread_attr: *const pthread_attr_t) -> size_t {
    if 0 != 0 {
        // TODO(perl): migrate the compile-time guard expression for this:
        // #if defined(__linux__) && defined(HAVE_DLSYM) && defined(__GLIBC__)
        let get_minstack: Option<unsafe extern "C" fn(*const pthread_attr_t) -> size_t> =
            ::core::mem::transmute::<
                *mut libc::c_void,
                Option<unsafe extern "C" fn(*const pthread_attr_t) -> size_t>,
            >(dlsym(
                0 as *mut libc::c_void,
                b"__pthread_get_minstack\0" as *const u8 as *const libc::c_char,
            ));
        if get_minstack.is_some() {
            return (get_minstack.expect("non-null function pointer")(thread_attr))
                .wrapping_sub(__sysconf(75) as size_t);
        }
    }
    return 0;
}
#[cold]
unsafe extern "C" fn get_num_threads(
    c: *mut Dav1dContext,
    s: *const Dav1dSettings,
    mut n_tc: *mut libc::c_uint,
    mut n_fc: *mut libc::c_uint,
) {
    static mut fc_lut: [uint8_t; 49] = [
        1 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
    ];
    *n_tc = (if (*s).n_threads != 0 {
        (*s).n_threads
    } else {
        iclip(
            dav1d_num_logical_processors(c),
            1 as libc::c_int,
            256 as libc::c_int,
        )
    }) as libc::c_uint;
    *n_fc = if (*s).max_frame_delay != 0 {
        umin((*s).max_frame_delay as libc::c_uint, *n_tc)
    } else {
        (if *n_tc < 50 as libc::c_uint {
            fc_lut[(*n_tc).wrapping_sub(1 as libc::c_int as libc::c_uint) as usize] as libc::c_int
        } else {
            8 as libc::c_int
        }) as libc::c_uint
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_get_frame_delay(s: *const Dav1dSettings) -> libc::c_int {
    let mut n_tc: libc::c_uint = 0;
    let mut n_fc: libc::c_uint = 0;
    if s.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"s != NULL\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"dav1d_get_frame_delay\0"))
                .as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    if !((*s).n_threads >= 0 && (*s).n_threads <= 256) {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"s->n_threads >= 0 && s->n_threads <= DAV1D_MAX_THREADS\0" as *const u8
                as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"dav1d_get_frame_delay\0"))
                .as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    if !((*s).max_frame_delay >= 0 && (*s).max_frame_delay <= 256) {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"s->max_frame_delay >= 0 && s->max_frame_delay <= DAV1D_MAX_FRAME_DELAY\0" as *const u8
                as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"dav1d_get_frame_delay\0"))
                .as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    get_num_threads(0 as *mut Dav1dContext, s, &mut n_tc, &mut n_fc);
    return n_fc as libc::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_open(
    c_out: *mut *mut Dav1dContext,
    s: *const Dav1dSettings,
) -> libc::c_int {
    let mut current_block: u64;
    static mut initted: pthread_once_t = 0 as libc::c_int;
    pthread_once(
        &mut initted,
        Some(init_internal as unsafe extern "C" fn() -> ()),
    );
    if c_out.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"c_out != NULL\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"dav1d_open\0")).as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    if s.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"s != NULL\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"dav1d_open\0")).as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    if !((*s).n_threads >= 0 && (*s).n_threads <= 256) {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"s->n_threads >= 0 && s->n_threads <= DAV1D_MAX_THREADS\0" as *const u8
                as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"dav1d_open\0")).as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    if !((*s).max_frame_delay >= 0 && (*s).max_frame_delay <= 256) {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"s->max_frame_delay >= 0 && s->max_frame_delay <= DAV1D_MAX_FRAME_DELAY\0" as *const u8
                as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"dav1d_open\0")).as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    if ((*s).allocator.alloc_picture_callback).is_none() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"s->allocator.alloc_picture_callback != NULL\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"dav1d_open\0")).as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    if ((*s).allocator.release_picture_callback).is_none() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"s->allocator.release_picture_callback != NULL\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"dav1d_open\0")).as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    if !((*s).operating_point >= 0 && (*s).operating_point <= 31) {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"s->operating_point >= 0 && s->operating_point <= 31\0" as *const u8
                as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"dav1d_open\0")).as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    if !((*s).decode_frame_type as libc::c_uint
        >= DAV1D_DECODEFRAMETYPE_ALL as libc::c_int as libc::c_uint
        && (*s).decode_frame_type as libc::c_uint
            <= DAV1D_DECODEFRAMETYPE_KEY as libc::c_int as libc::c_uint)
    {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const libc::c_char,
            b"s->decode_frame_type >= DAV1D_DECODEFRAMETYPE_ALL && s->decode_frame_type <= DAV1D_DECODEFRAMETYPE_KEY\0"
                as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"dav1d_open\0"))
                .as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    let mut thread_attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    if pthread_attr_init(&mut thread_attr) != 0 {
        return -(12 as libc::c_int);
    }
    let mut stack_size: size_t = 1024 * 1024 * get_stack_size_internal(&mut thread_attr);
    pthread_attr_setstacksize(&mut thread_attr, stack_size);
    *c_out = dav1d_alloc_aligned(::core::mem::size_of::<Dav1dContext>(), 64) as *mut Dav1dContext;
    let c: *mut Dav1dContext = *c_out;
    if !c.is_null() {
        memset(
            c as *mut libc::c_void,
            0,
            ::core::mem::size_of::<Dav1dContext>(),
        );
        (*c).allocator = (*s).allocator;
        (*c).logger = (*s).logger;
        (*c).apply_grain = (*s).apply_grain;
        (*c).operating_point = (*s).operating_point;
        (*c).all_layers = (*s).all_layers;
        (*c).frame_size_limit = (*s).frame_size_limit;
        (*c).strict_std_compliance = (*s).strict_std_compliance;
        (*c).output_invisible_frames = (*s).output_invisible_frames;
        (*c).inloop_filters = (*s).inloop_filters;
        (*c).decode_frame_type = (*s).decode_frame_type;
        dav1d_data_props_set_defaults(&mut (*c).cached_error_props);
        if !(dav1d_mem_pool_init(&mut (*c).seq_hdr_pool) != 0
            || dav1d_mem_pool_init(&mut (*c).frame_hdr_pool) != 0
            || dav1d_mem_pool_init(&mut (*c).segmap_pool) != 0
            || dav1d_mem_pool_init(&mut (*c).refmvs_pool) != 0
            || dav1d_mem_pool_init(&mut (*c).cdf_pool) != 0)
        {
            if (*c).allocator.alloc_picture_callback
                == Some(
                    dav1d_default_picture_alloc
                        as unsafe extern "C" fn(
                            *mut Dav1dPicture,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                )
                && (*c).allocator.release_picture_callback
                    == Some(
                        dav1d_default_picture_release
                            as unsafe extern "C" fn(*mut Dav1dPicture, *mut libc::c_void) -> (),
                    )
            {
                if !((*c).allocator.cookie).is_null() {
                    current_block = 16409883578687858768;
                } else if dav1d_mem_pool_init(&mut (*c).picture_pool) != 0 {
                    current_block = 16409883578687858768;
                } else {
                    (*c).allocator.cookie = (*c).picture_pool as *mut libc::c_void;
                    current_block = 13619784596304402172;
                }
            } else if (*c).allocator.alloc_picture_callback
                == Some(
                    dav1d_default_picture_alloc
                        as unsafe extern "C" fn(
                            *mut Dav1dPicture,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                )
                || (*c).allocator.release_picture_callback
                    == Some(
                        dav1d_default_picture_release
                            as unsafe extern "C" fn(*mut Dav1dPicture, *mut libc::c_void) -> (),
                    )
            {
                current_block = 16409883578687858768;
            } else {
                current_block = 13619784596304402172;
            }
            match current_block {
                16409883578687858768 => {}
                _ => {
                    if (::core::mem::size_of::<size_t>() as libc::c_ulong) < 8 as libc::c_ulong
                        && ((*s).frame_size_limit).wrapping_sub(1 as libc::c_int as libc::c_uint)
                            >= (8192 * 8192) as libc::c_uint
                    {
                        (*c).frame_size_limit = (8192 as libc::c_int * 8192) as libc::c_uint;
                        if (*s).frame_size_limit != 0 {
                            dav1d_log(
                                c,
                                b"Frame size limit reduced from %u to %u.\n\0" as *const u8
                                    as *const libc::c_char,
                                (*s).frame_size_limit,
                                (*c).frame_size_limit,
                            );
                        }
                    }
                    (*c).flush = &mut (*c).flush_mem;
                    *(*c).flush = 0 as libc::c_int;
                    get_num_threads(c, s, &mut (*c).n_tc, &mut (*c).n_fc);
                    (*c).fc = dav1d_alloc_aligned(
                        (::core::mem::size_of::<Dav1dFrameContext>())
                            .wrapping_mul((*c).n_fc as size_t),
                        32 as libc::c_int as size_t,
                    ) as *mut Dav1dFrameContext;
                    if !((*c).fc).is_null() {
                        memset(
                            (*c).fc as *mut libc::c_void,
                            0,
                            ::core::mem::size_of::<Dav1dFrameContext>()
                                .wrapping_mul((*c).n_fc as size_t),
                        );
                        (*c).tc = dav1d_alloc_aligned(
                            (::core::mem::size_of::<Dav1dTaskContext>())
                                .wrapping_mul((*c).n_tc as size_t),
                            64 as libc::c_int as size_t,
                        ) as *mut Dav1dTaskContext;
                        if !((*c).tc).is_null() {
                            memset(
                                (*c).tc as *mut libc::c_void,
                                0,
                                ::core::mem::size_of::<Dav1dTaskContext>()
                                    .wrapping_mul((*c).n_tc as size_t),
                            );
                            if (*c).n_tc > 1 as libc::c_uint {
                                if pthread_mutex_init(
                                    &mut (*c).task_thread.lock,
                                    0 as *const pthread_mutexattr_t,
                                ) != 0
                                {
                                    current_block = 16409883578687858768;
                                } else if pthread_cond_init(
                                    &mut (*c).task_thread.cond,
                                    0 as *const pthread_condattr_t,
                                ) != 0
                                {
                                    pthread_mutex_destroy(&mut (*c).task_thread.lock);
                                    current_block = 16409883578687858768;
                                } else if pthread_cond_init(
                                    &mut (*c).task_thread.delayed_fg.cond,
                                    0 as *const pthread_condattr_t,
                                ) != 0
                                {
                                    pthread_cond_destroy(&mut (*c).task_thread.cond);
                                    pthread_mutex_destroy(&mut (*c).task_thread.lock);
                                    current_block = 16409883578687858768;
                                } else {
                                    (*c).task_thread.cur = (*c).n_fc;
                                    *&mut (*c).task_thread.reset_task_cur =
                                        (2147483647 as libc::c_int as libc::c_uint)
                                            .wrapping_mul(2 as libc::c_uint)
                                            .wrapping_add(1 as libc::c_uint);
                                    *&mut (*c).task_thread.cond_signaled = 0 as libc::c_int;
                                    (*c).task_thread.inited = 1 as libc::c_int;
                                    current_block = 1868291631715963762;
                                }
                            } else {
                                current_block = 1868291631715963762;
                            }
                            match current_block {
                                16409883578687858768 => {}
                                _ => {
                                    if (*c).n_fc > 1 as libc::c_uint {
                                        (*c).frame_thread.out_delayed = calloc(
                                            (*c).n_fc as size_t,
                                            ::core::mem::size_of::<Dav1dThreadPicture>(),
                                        )
                                            as *mut Dav1dThreadPicture;
                                        if ((*c).frame_thread.out_delayed).is_null() {
                                            current_block = 16409883578687858768;
                                        } else {
                                            current_block = 12961834331865314435;
                                        }
                                    } else {
                                        current_block = 12961834331865314435;
                                    }
                                    match current_block {
                                        16409883578687858768 => {}
                                        _ => {
                                            let mut n: libc::c_uint =
                                                0 as libc::c_int as libc::c_uint;
                                            loop {
                                                if !(n < (*c).n_fc) {
                                                    current_block = 12027283704867122503;
                                                    break;
                                                }
                                                let f: *mut Dav1dFrameContext = &mut *((*c).fc)
                                                    .offset(n as isize)
                                                    as *mut Dav1dFrameContext;
                                                if (*c).n_tc > 1 as libc::c_uint {
                                                    if pthread_mutex_init(
                                                        &mut (*f).task_thread.lock,
                                                        0 as *const pthread_mutexattr_t,
                                                    ) != 0
                                                    {
                                                        current_block = 16409883578687858768;
                                                        break;
                                                    }
                                                    if pthread_cond_init(
                                                        &mut (*f).task_thread.cond,
                                                        0 as *const pthread_condattr_t,
                                                    ) != 0
                                                    {
                                                        pthread_mutex_destroy(
                                                            &mut (*f).task_thread.lock,
                                                        );
                                                        current_block = 16409883578687858768;
                                                        break;
                                                    } else if pthread_mutex_init(
                                                        &mut (*f).task_thread.pending_tasks.lock,
                                                        0 as *const pthread_mutexattr_t,
                                                    ) != 0
                                                    {
                                                        pthread_cond_destroy(
                                                            &mut (*f).task_thread.cond,
                                                        );
                                                        pthread_mutex_destroy(
                                                            &mut (*f).task_thread.lock,
                                                        );
                                                        current_block = 16409883578687858768;
                                                        break;
                                                    }
                                                }
                                                (*f).c = c;
                                                (*f).task_thread.ttd = &mut (*c).task_thread;
                                                (*f).lf.last_sharpness = -(1 as libc::c_int);
                                                dav1d_refmvs_init(&mut (*f).rf);
                                                n = n.wrapping_add(1);
                                            }
                                            match current_block {
                                                16409883578687858768 => {}
                                                _ => {
                                                    let mut m: libc::c_uint =
                                                        0 as libc::c_int as libc::c_uint;
                                                    loop {
                                                        if !(m < (*c).n_tc) {
                                                            current_block = 15734707049249739970;
                                                            break;
                                                        }
                                                        let t: *mut Dav1dTaskContext =
                                                            &mut *((*c).tc).offset(m as isize)
                                                                as *mut Dav1dTaskContext;
                                                        (*t).f = &mut *((*c).fc).offset(0)
                                                            as *mut Dav1dFrameContext;
                                                        (*t).task_thread.ttd =
                                                            &mut (*c).task_thread;
                                                        (*t).c = c;
                                                        memset(
                                                            ((*t).c2rust_unnamed.cf_16bpc)
                                                                .as_mut_ptr()
                                                                as *mut libc::c_void,
                                                            0 as libc::c_int,
                                                            ::core::mem::size_of::<[int32_t; 1024]>(
                                                            ),
                                                        );
                                                        if (*c).n_tc > 1 as libc::c_uint {
                                                            if pthread_mutex_init(
                                                                &mut (*t).task_thread.td.lock,
                                                                0 as *const pthread_mutexattr_t,
                                                            ) != 0
                                                            {
                                                                current_block =
                                                                    16409883578687858768;
                                                                break;
                                                            }
                                                            if pthread_cond_init(
                                                                &mut (*t).task_thread.td.cond,
                                                                0 as *const pthread_condattr_t,
                                                            ) != 0
                                                            {
                                                                pthread_mutex_destroy(&mut (*t).task_thread.td.lock);
                                                                current_block = 16409883578687858768;
                                                                break;
                                                            } else if pthread_create(
                                                                &mut (*t).task_thread.td.thread,
                                                                &mut thread_attr,
                                                                Some(
                                                                    dav1d_worker_task
                                                                        as unsafe extern "C" fn(
                                                                            *mut libc::c_void,
                                                                        ) -> *mut libc::c_void,
                                                                ),
                                                                t as *mut libc::c_void,
                                                            ) != 0
                                                            {
                                                                pthread_cond_destroy(&mut (*t).task_thread.td.cond);
                                                                pthread_mutex_destroy(&mut (*t).task_thread.td.lock);
                                                                current_block = 16409883578687858768;
                                                                break;
                                                            } else {
                                                                (*t).task_thread.td.inited = 1 as libc::c_int;
                                                            }
                                                        }
                                                        m = m.wrapping_add(1);
                                                    }
                                                    match current_block {
                                                        16409883578687858768 => {}
                                                        _ => {
                                                            dav1d_refmvs_dsp_init(
                                                                &mut (*c).refmvs_dsp,
                                                            );
                                                            (*c).intra_edge.root[BL_128X128
                                                                as libc::c_int
                                                                as usize] = &mut (*((*c)
                                                                .intra_edge
                                                                .branch_sb128)
                                                                .as_mut_ptr()
                                                                .offset(0))
                                                            .node;
                                                            dav1d_init_mode_tree(
                                                                (*c).intra_edge.root[BL_128X128
                                                                    as libc::c_int
                                                                    as usize],
                                                                ((*c).intra_edge.tip_sb128)
                                                                    .as_mut_ptr(),
                                                                1 as libc::c_int,
                                                            );
                                                            (*c).intra_edge.root[BL_64X64
                                                                as libc::c_int
                                                                as usize] = &mut (*((*c)
                                                                .intra_edge
                                                                .branch_sb64)
                                                                .as_mut_ptr()
                                                                .offset(0))
                                                            .node;
                                                            dav1d_init_mode_tree(
                                                                (*c).intra_edge.root[BL_64X64
                                                                    as libc::c_int
                                                                    as usize],
                                                                ((*c).intra_edge.tip_sb64)
                                                                    .as_mut_ptr(),
                                                                0 as libc::c_int,
                                                            );
                                                            pthread_attr_destroy(&mut thread_attr);
                                                            return 0 as libc::c_int;
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
    if !c.is_null() {
        close_internal(c_out, 0 as libc::c_int);
    }
    pthread_attr_destroy(&mut thread_attr);
    return -(12 as libc::c_int);
}
unsafe extern "C" fn dummy_free(data: *const uint8_t, user_data: *mut libc::c_void) {
    if !(!data.is_null() && user_data.is_null()) {
        unreachable!();
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_parse_sequence_header(
    out: *mut Dav1dSequenceHeader,
    ptr: *const uint8_t,
    sz: size_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut buf: Dav1dData = {
        let mut init = Dav1dData {
            data: 0 as *const uint8_t,
            sz: 0,
            r#ref: 0 as *mut Dav1dRef,
            m: Dav1dDataProps {
                timestamp: 0,
                duration: 0,
                offset: 0,
                size: 0,
                user_data: Dav1dUserData {
                    data: 0 as *const uint8_t,
                    r#ref: 0 as *mut Dav1dRef,
                },
            },
        };
        init
    };
    let mut res = 0;
    if out.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"out != NULL\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                b"dav1d_parse_sequence_header\0",
            ))
            .as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    let mut s: Dav1dSettings = Dav1dSettings {
        n_threads: 0,
        max_frame_delay: 0,
        apply_grain: 0,
        operating_point: 0,
        all_layers: 0,
        frame_size_limit: 0,
        allocator: Dav1dPicAllocator {
            cookie: 0 as *mut libc::c_void,
            alloc_picture_callback: None,
            release_picture_callback: None,
        },
        logger: Dav1dLogger {
            cookie: 0 as *mut libc::c_void,
            callback: None,
        },
        strict_std_compliance: 0,
        output_invisible_frames: 0,
        inloop_filters: DAV1D_INLOOPFILTER_NONE,
        decode_frame_type: DAV1D_DECODEFRAMETYPE_ALL,
        reserved: [0; 16],
    };
    dav1d_default_settings(&mut s);
    s.n_threads = 1 as libc::c_int;
    s.logger.callback = None;
    let mut c: *mut Dav1dContext = 0 as *mut Dav1dContext;
    res = dav1d_open(&mut c, &mut s);
    if res < 0 {
        return res;
    }
    if !ptr.is_null() {
        res = dav1d_data_wrap_internal(
            &mut buf,
            ptr,
            sz,
            Some(dummy_free as unsafe extern "C" fn(*const uint8_t, *mut libc::c_void) -> ()),
            0 as *mut libc::c_void,
        );
        if res < 0 {
            current_block = 10647346020414903899;
        } else {
            current_block = 5399440093318478209;
        }
    } else {
        current_block = 5399440093318478209;
    }
    loop {
        match current_block {
            10647346020414903899 => {
                dav1d_data_unref_internal(&mut buf);
                break;
            }
            _ => {
                if buf.sz > 0 {
                    res = dav1d_parse_obus(c, &mut buf, 1 as libc::c_int);
                    if res < 0 {
                        current_block = 10647346020414903899;
                        continue;
                    }
                    if !(res as size_t <= buf.sz) {
                        unreachable!();
                    }
                    buf.sz = (buf.sz as libc::c_ulong).wrapping_sub(res as libc::c_ulong) as size_t
                        as size_t;
                    buf.data = (buf.data).offset(res as isize);
                    current_block = 5399440093318478209;
                } else if ((*c).seq_hdr).is_null() {
                    res = -(2 as libc::c_int);
                    current_block = 10647346020414903899;
                } else {
                    memcpy(
                        out as *mut libc::c_void,
                        (*c).seq_hdr as *const libc::c_void,
                        ::core::mem::size_of::<Dav1dSequenceHeader>(),
                    );
                    res = 0 as libc::c_int;
                    current_block = 10647346020414903899;
                }
            }
        }
    }
    dav1d_close(&mut c);
    return res;
}
unsafe extern "C" fn has_grain(pic: *const Dav1dPicture) -> libc::c_int {
    let mut fgdata: *const Dav1dFilmGrainData = &mut (*(*pic).frame_hdr).film_grain.data;
    return ((*fgdata).num_y_points != 0
        || (*fgdata).num_uv_points[0] != 0
        || (*fgdata).num_uv_points[1] != 0
        || (*fgdata).clip_to_restricted_range != 0 && (*fgdata).chroma_scaling_from_luma != 0)
        as libc::c_int;
}
unsafe extern "C" fn output_image(c: *mut Dav1dContext, out: *mut Dav1dPicture) -> libc::c_int {
    let mut res = 0;
    let in_0: *mut Dav1dThreadPicture = if (*c).all_layers != 0 || (*c).max_spatial_id == 0 {
        &mut (*c).out
    } else {
        &mut (*c).cache
    };
    if (*c).apply_grain == 0 || has_grain(&mut (*in_0).p) == 0 {
        dav1d_picture_move_ref(out, &mut (*in_0).p);
        dav1d_thread_picture_unref(in_0);
    } else {
        res = dav1d_apply_grain(c, out, &mut (*in_0).p);
        dav1d_thread_picture_unref(in_0);
    }
    if (*c).all_layers == 0 && (*c).max_spatial_id != 0 && !((*c).out.p.data[0]).is_null() {
        dav1d_thread_picture_move_ref(in_0, &mut (*c).out);
    }
    return res;
}
unsafe extern "C" fn output_picture_ready(c: *mut Dav1dContext, drain: libc::c_int) -> libc::c_int {
    if (*c).cached_error != 0 {
        return 1 as libc::c_int;
    }
    if (*c).all_layers == 0 && (*c).max_spatial_id != 0 {
        if !((*c).out.p.data[0]).is_null() && !((*c).cache.p.data[0]).is_null() {
            if (*c).max_spatial_id == (*(*c).cache.p.frame_hdr).spatial_id
                || (*c).out.flags as libc::c_uint
                    & PICTURE_FLAG_NEW_TEMPORAL_UNIT as libc::c_int as libc::c_uint
                    != 0
            {
                return 1 as libc::c_int;
            }
            dav1d_thread_picture_unref(&mut (*c).cache);
            dav1d_thread_picture_move_ref(&mut (*c).cache, &mut (*c).out);
            return 0 as libc::c_int;
        } else {
            if !((*c).cache.p.data[0]).is_null() && drain != 0 {
                return 1 as libc::c_int;
            } else {
                if !((*c).out.p.data[0]).is_null() {
                    dav1d_thread_picture_move_ref(&mut (*c).cache, &mut (*c).out);
                    return 0 as libc::c_int;
                }
            }
        }
    }
    return !((*c).out.p.data[0]).is_null() as libc::c_int;
}
unsafe extern "C" fn drain_picture(c: *mut Dav1dContext, out: *mut Dav1dPicture) -> libc::c_int {
    let mut drain_count: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut drained: libc::c_int = 0 as libc::c_int;
    loop {
        let next: libc::c_uint = (*c).frame_thread.next;
        let f: *mut Dav1dFrameContext =
            &mut *((*c).fc).offset(next as isize) as *mut Dav1dFrameContext;
        pthread_mutex_lock(&mut (*c).task_thread.lock);
        while (*f).n_tile_data > 0 {
            pthread_cond_wait(
                &mut (*f).task_thread.cond,
                &mut (*(*f).task_thread.ttd).lock,
            );
        }
        let out_delayed: *mut Dav1dThreadPicture =
            &mut *((*c).frame_thread.out_delayed).offset(next as isize) as *mut Dav1dThreadPicture;
        if !((*out_delayed).p.data[0 as libc::c_int as usize]).is_null()
            || ::core::intrinsics::atomic_load_seqcst(
                &mut (*f).task_thread.error as *mut atomic_int,
            ) != 0
        {
            let mut first: libc::c_uint =
                ::core::intrinsics::atomic_load_seqcst(&mut (*c).task_thread.first);
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
            let fresh0 = ::core::intrinsics::atomic_cxchg_seqcst_seqcst(
                &mut (*c).task_thread.reset_task_cur,
                *&mut first,
                (2147483647 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_uint)
                    .wrapping_add(1 as libc::c_uint),
            );
            *&mut first = fresh0.0;
            fresh0.1;
            if (*c).task_thread.cur != 0 && (*c).task_thread.cur < (*c).n_fc {
                (*c).task_thread.cur = ((*c).task_thread.cur).wrapping_sub(1);
            }
            drained = 1 as libc::c_int;
        } else if drained != 0 {
            pthread_mutex_unlock(&mut (*c).task_thread.lock);
            break;
        }
        (*c).frame_thread.next = ((*c).frame_thread.next).wrapping_add(1);
        if (*c).frame_thread.next == (*c).n_fc {
            (*c).frame_thread.next = 0 as libc::c_int as libc::c_uint;
        }
        pthread_mutex_unlock(&mut (*c).task_thread.lock);
        let error = (*f).task_thread.retval;
        if error != 0 {
            (*f).task_thread.retval = 0 as libc::c_int;
            dav1d_data_props_copy(&mut (*c).cached_error_props, &mut (*out_delayed).p.m);
            dav1d_thread_picture_unref(out_delayed);
            return error;
        }
        if !((*out_delayed).p.data[0]).is_null() {
            let progress: libc::c_uint = ::core::intrinsics::atomic_load_relaxed(
                &mut *((*out_delayed).progress).offset(1) as *mut atomic_uint,
            );
            if ((*out_delayed).visible != 0 || (*c).output_invisible_frames != 0)
                && progress
                    != (2147483647 as libc::c_int as libc::c_uint)
                        .wrapping_mul(2 as libc::c_uint)
                        .wrapping_add(1 as libc::c_uint)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
            {
                dav1d_thread_picture_ref(&mut (*c).out, out_delayed);
                (*c).event_flags = ::core::mem::transmute::<libc::c_uint, Dav1dEventFlags>(
                    (*c).event_flags as libc::c_uint
                        | dav1d_picture_get_event_flags(out_delayed) as libc::c_uint,
                );
            }
            dav1d_thread_picture_unref(out_delayed);
            if output_picture_ready(c, 0 as libc::c_int) != 0 {
                return output_image(c, out);
            }
        }
        drain_count = drain_count.wrapping_add(1);
        if !(drain_count < (*c).n_fc) {
            break;
        }
    }
    if output_picture_ready(c, 1 as libc::c_int) != 0 {
        return output_image(c, out);
    }
    return -(11 as libc::c_int);
}
unsafe extern "C" fn gen_picture(c: *mut Dav1dContext) -> libc::c_int {
    let mut res = 0;
    let in_0: *mut Dav1dData = &mut (*c).in_0;
    if output_picture_ready(c, 0 as libc::c_int) != 0 {
        return 0 as libc::c_int;
    }
    while (*in_0).sz > 0 {
        res = dav1d_parse_obus(c, in_0, 0 as libc::c_int);
        if res < 0 {
            dav1d_data_unref_internal(in_0);
        } else {
            if !(res as size_t <= (*in_0).sz) {
                unreachable!();
            }
            (*in_0).sz = ((*in_0).sz as libc::c_ulong).wrapping_sub(res as libc::c_ulong) as size_t
                as size_t;
            (*in_0).data = ((*in_0).data).offset(res as isize);
            if (*in_0).sz == 0 {
                dav1d_data_unref_internal(in_0);
            }
        }
        if output_picture_ready(c, 0 as libc::c_int) != 0 {
            break;
        }
        if res < 0 {
            return res;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_send_data(
    c: *mut Dav1dContext,
    in_0: *mut Dav1dData,
) -> libc::c_int {
    if c.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"c != NULL\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"dav1d_send_data\0"))
                .as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    if in_0.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"in != NULL\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"dav1d_send_data\0"))
                .as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    if !(((*in_0).data).is_null() || (*in_0).sz != 0) {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"in->data == NULL || in->sz\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"dav1d_send_data\0"))
                .as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    if !((*in_0).data).is_null() {
        (*c).drain = 0 as libc::c_int;
    }
    if !((*c).in_0.data).is_null() {
        return -(11 as libc::c_int);
    }
    dav1d_data_ref(&mut (*c).in_0, in_0);
    let mut res = gen_picture(c);
    if res == 0 {
        dav1d_data_unref_internal(in_0);
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_picture(
    c: *mut Dav1dContext,
    out: *mut Dav1dPicture,
) -> libc::c_int {
    if c.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"c != NULL\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"dav1d_get_picture\0"))
                .as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    if out.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"out != NULL\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"dav1d_get_picture\0"))
                .as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    let drain = (*c).drain;
    (*c).drain = 1 as libc::c_int;
    let mut res = gen_picture(c);
    if res < 0 {
        return res;
    }
    if (*c).cached_error != 0 {
        let res_0 = (*c).cached_error;
        (*c).cached_error = 0 as libc::c_int;
        return res_0;
    }
    if output_picture_ready(c, ((*c).n_fc == 1 as libc::c_uint) as libc::c_int) != 0 {
        return output_image(c, out);
    }
    if (*c).n_fc > 1 as libc::c_uint && drain != 0 {
        return drain_picture(c, out);
    }
    return -(11 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_apply_grain(
    c: *mut Dav1dContext,
    out: *mut Dav1dPicture,
    in_0: *const Dav1dPicture,
) -> libc::c_int {
    if c.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"c != NULL\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"dav1d_apply_grain\0"))
                .as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    if out.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"out != NULL\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"dav1d_apply_grain\0"))
                .as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    if in_0.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"in != NULL\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"dav1d_apply_grain\0"))
                .as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    if has_grain(in_0) == 0 {
        dav1d_picture_ref(out, in_0);
        return 0 as libc::c_int;
    }
    let mut res = dav1d_picture_alloc_copy(c, out, (*in_0).p.w, in_0);
    if res < 0 {
        dav1d_picture_unref_internal(out);
        return res;
    } else {
        if (*c).n_tc > 1 as libc::c_uint {
            dav1d_task_delayed_fg(c, out, in_0);
        } else {
            match (*out).p.bpc {
                #[cfg(feature = "bitdepth_8")]
                8 => {
                    dav1d_apply_grain_8bpc(&mut (*((*c).dsp).as_mut_ptr().offset(0)).fg, out, in_0);
                }
                #[cfg(feature = "bitdepth_16")]
                10 | 12 => {
                    dav1d_apply_grain_16bpc(
                        &mut (*((*c).dsp)
                            .as_mut_ptr()
                            .offset((((*out).p.bpc >> 1) - 4) as isize))
                        .fg,
                        out,
                        in_0,
                    );
                }
                _ => {
                    abort();
                }
            }
        }
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_flush(c: *mut Dav1dContext) {
    dav1d_data_unref_internal(&mut (*c).in_0);
    if !((*c).out.p.frame_hdr).is_null() {
        dav1d_thread_picture_unref(&mut (*c).out);
    }
    if !((*c).cache.p.frame_hdr).is_null() {
        dav1d_thread_picture_unref(&mut (*c).cache);
    }
    (*c).drain = 0 as libc::c_int;
    (*c).cached_error = 0 as libc::c_int;
    let mut i = 0;
    while i < 8 {
        if !((*c).refs[i as usize].p.p.frame_hdr).is_null() {
            dav1d_thread_picture_unref(&mut (*((*c).refs).as_mut_ptr().offset(i as isize)).p);
        }
        dav1d_ref_dec(&mut (*((*c).refs).as_mut_ptr().offset(i as isize)).segmap);
        dav1d_ref_dec(&mut (*((*c).refs).as_mut_ptr().offset(i as isize)).refmvs);
        dav1d_cdf_thread_unref(&mut *((*c).cdf).as_mut_ptr().offset(i as isize));
        i += 1;
    }
    (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
    (*c).seq_hdr = 0 as *mut Dav1dSequenceHeader;
    dav1d_ref_dec(&mut (*c).seq_hdr_ref);
    (*c).mastering_display = 0 as *mut Dav1dMasteringDisplay;
    (*c).content_light = 0 as *mut Dav1dContentLightLevel;
    (*c).itut_t35 = 0 as *mut Dav1dITUTT35;
    dav1d_ref_dec(&mut (*c).mastering_display_ref);
    dav1d_ref_dec(&mut (*c).content_light_ref);
    dav1d_ref_dec(&mut (*c).itut_t35_ref);
    dav1d_data_props_unref_internal(&mut (*c).cached_error_props);
    if (*c).n_fc == 1 as libc::c_uint && (*c).n_tc == 1 as libc::c_uint {
        return;
    }
    ::core::intrinsics::atomic_store_seqcst((*c).flush, 1 as libc::c_int);
    if (*c).n_tc > 1 as libc::c_uint {
        pthread_mutex_lock(&mut (*c).task_thread.lock);
        let mut i_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i_0 < (*c).n_tc {
            let tc: *mut Dav1dTaskContext =
                &mut *((*c).tc).offset(i_0 as isize) as *mut Dav1dTaskContext;
            while (*tc).task_thread.flushed == 0 {
                pthread_cond_wait(&mut (*tc).task_thread.td.cond, &mut (*c).task_thread.lock);
            }
            i_0 = i_0.wrapping_add(1);
        }
        let mut i_1: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i_1 < (*c).n_fc {
            let ref mut fresh1 = (*((*c).fc).offset(i_1 as isize)).task_thread.task_head;
            *fresh1 = 0 as *mut Dav1dTask;
            let ref mut fresh2 = (*((*c).fc).offset(i_1 as isize)).task_thread.task_tail;
            *fresh2 = 0 as *mut Dav1dTask;
            let ref mut fresh3 = (*((*c).fc).offset(i_1 as isize)).task_thread.task_cur_prev;
            *fresh3 = 0 as *mut Dav1dTask;
            let ref mut fresh4 = (*((*c).fc).offset(i_1 as isize))
                .task_thread
                .pending_tasks
                .head;
            *fresh4 = 0 as *mut Dav1dTask;
            let ref mut fresh5 = (*((*c).fc).offset(i_1 as isize))
                .task_thread
                .pending_tasks
                .tail;
            *fresh5 = 0 as *mut Dav1dTask;
            *&mut (*((*c).fc).offset(i_1 as isize))
                .task_thread
                .pending_tasks
                .merge = 0 as libc::c_int;
            i_1 = i_1.wrapping_add(1);
        }
        *&mut (*c).task_thread.first = 0 as libc::c_int as libc::c_uint;
        (*c).task_thread.cur = (*c).n_fc;
        ::core::intrinsics::atomic_store_seqcst(
            &mut (*c).task_thread.reset_task_cur,
            (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint),
        );
        ::core::intrinsics::atomic_store_seqcst(
            &mut (*c).task_thread.cond_signaled,
            0 as libc::c_int,
        );
        pthread_mutex_unlock(&mut (*c).task_thread.lock);
    }
    if (*c).n_fc > 1 as libc::c_uint {
        let mut n: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut next: libc::c_uint = (*c).frame_thread.next;
        while n < (*c).n_fc {
            if next == (*c).n_fc {
                next = 0 as libc::c_int as libc::c_uint;
            }
            let f: *mut Dav1dFrameContext =
                &mut *((*c).fc).offset(next as isize) as *mut Dav1dFrameContext;
            dav1d_decode_frame_exit(f, -(1 as libc::c_int));
            (*f).n_tile_data = 0 as libc::c_int;
            (*f).task_thread.retval = 0 as libc::c_int;
            let mut out_delayed: *mut Dav1dThreadPicture = &mut *((*c).frame_thread.out_delayed)
                .offset(next as isize)
                as *mut Dav1dThreadPicture;
            if !((*out_delayed).p.frame_hdr).is_null() {
                dav1d_thread_picture_unref(out_delayed);
            }
            n = n.wrapping_add(1);
            next = next.wrapping_add(1);
        }
        (*c).frame_thread.next = 0 as libc::c_int as libc::c_uint;
    }
    ::core::intrinsics::atomic_store_seqcst((*c).flush, 0 as libc::c_int);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_close(c_out: *mut *mut Dav1dContext) {
    if c_out.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"c_out != ((void*)0)\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"dav1d_close\0")).as_ptr(),
        );
        return;
    }
    close_internal(c_out, 1 as libc::c_int);
}
#[cold]
unsafe extern "C" fn close_internal(c_out: *mut *mut Dav1dContext, mut flush: libc::c_int) {
    let c: *mut Dav1dContext = *c_out;
    if c.is_null() {
        return;
    }
    if flush != 0 {
        dav1d_flush(c);
    }
    if !((*c).tc).is_null() {
        let mut ttd: *mut TaskThreadData = &mut (*c).task_thread;
        if (*ttd).inited != 0 {
            pthread_mutex_lock(&mut (*ttd).lock);
            let mut n: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while n < (*c).n_tc && (*((*c).tc).offset(n as isize)).task_thread.td.inited != 0 {
                (*((*c).tc).offset(n as isize)).task_thread.die = 1 as libc::c_int;
                n = n.wrapping_add(1);
            }
            pthread_cond_broadcast(&mut (*ttd).cond);
            pthread_mutex_unlock(&mut (*ttd).lock);
            let mut n_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while n_0 < (*c).n_tc {
                let pf: *mut Dav1dTaskContext =
                    &mut *((*c).tc).offset(n_0 as isize) as *mut Dav1dTaskContext;
                if (*pf).task_thread.td.inited == 0 {
                    break;
                }
                pthread_join((*pf).task_thread.td.thread, 0 as *mut *mut libc::c_void);
                pthread_cond_destroy(&mut (*pf).task_thread.td.cond);
                pthread_mutex_destroy(&mut (*pf).task_thread.td.lock);
                n_0 = n_0.wrapping_add(1);
            }
            pthread_cond_destroy(&mut (*ttd).delayed_fg.cond);
            pthread_cond_destroy(&mut (*ttd).cond);
            pthread_mutex_destroy(&mut (*ttd).lock);
        }
        dav1d_free_aligned((*c).tc as *mut libc::c_void);
    }
    let mut n_1: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while !((*c).fc).is_null() && n_1 < (*c).n_fc {
        let f: *mut Dav1dFrameContext =
            &mut *((*c).fc).offset(n_1 as isize) as *mut Dav1dFrameContext;
        if (*c).n_fc > 1 as libc::c_uint {
            freep(
                &mut (*f).tile_thread.lowest_pixel_mem as *mut *mut [[libc::c_int; 2]; 7]
                    as *mut libc::c_void,
            );
            freep(&mut (*f).frame_thread.b as *mut *mut Av1Block as *mut libc::c_void);
            dav1d_freep_aligned(
                &mut (*f).frame_thread.pal_idx as *mut *mut uint8_t as *mut libc::c_void,
            );
            dav1d_freep_aligned(
                &mut (*f).frame_thread.cf as *mut *mut libc::c_void as *mut libc::c_void,
            );
            freep(
                &mut (*f).frame_thread.tile_start_off as *mut *mut libc::c_int as *mut libc::c_void,
            );
            dav1d_freep_aligned(
                &mut (*f).frame_thread.pal as *mut *mut [[uint16_t; 8]; 3] as *mut libc::c_void,
            );
            freep(&mut (*f).frame_thread.cbi as *mut *mut CodedBlockInfo as *mut libc::c_void);
        }
        if (*c).n_tc > 1 as libc::c_uint {
            pthread_mutex_destroy(&mut (*f).task_thread.pending_tasks.lock);
            pthread_cond_destroy(&mut (*f).task_thread.cond);
            pthread_mutex_destroy(&mut (*f).task_thread.lock);
        }
        freep(&mut (*f).frame_thread.frame_progress as *mut *mut atomic_uint as *mut libc::c_void);
        freep(&mut (*f).task_thread.tasks as *mut *mut Dav1dTask as *mut libc::c_void);
        freep(
            &mut *((*f).task_thread.tile_tasks).as_mut_ptr().offset(0) as *mut *mut Dav1dTask
                as *mut libc::c_void,
        );
        dav1d_free_aligned((*f).ts as *mut libc::c_void);
        dav1d_free_aligned((*f).ipred_edge[0]);
        free((*f).a as *mut libc::c_void);
        free((*f).tile as *mut libc::c_void);
        free((*f).lf.mask as *mut libc::c_void);
        free((*f).lf.lr_mask as *mut libc::c_void);
        free((*f).lf.level as *mut libc::c_void);
        free((*f).lf.tx_lpf_right_edge[0] as *mut libc::c_void);
        free((*f).lf.start_of_tile_row as *mut libc::c_void);
        dav1d_refmvs_clear(&mut (*f).rf);
        dav1d_free_aligned((*f).lf.cdef_line_buf as *mut libc::c_void);
        dav1d_free_aligned((*f).lf.lr_line_buf as *mut libc::c_void);
        n_1 = n_1.wrapping_add(1);
    }
    dav1d_free_aligned((*c).fc as *mut libc::c_void);
    if (*c).n_fc > 1 as libc::c_uint && !((*c).frame_thread.out_delayed).is_null() {
        let mut n_2: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while n_2 < (*c).n_fc {
            if !((*((*c).frame_thread.out_delayed).offset(n_2 as isize))
                .p
                .frame_hdr)
                .is_null()
            {
                dav1d_thread_picture_unref(
                    &mut *((*c).frame_thread.out_delayed).offset(n_2 as isize),
                );
            }
            n_2 = n_2.wrapping_add(1);
        }
        free((*c).frame_thread.out_delayed as *mut libc::c_void);
    }
    let mut n_3 = 0;
    while n_3 < (*c).n_tile_data {
        dav1d_data_unref_internal(&mut (*((*c).tile).offset(n_3 as isize)).data);
        n_3 += 1;
    }
    free((*c).tile as *mut libc::c_void);
    let mut n_4 = 0;
    while n_4 < 8 {
        dav1d_cdf_thread_unref(&mut *((*c).cdf).as_mut_ptr().offset(n_4 as isize));
        if !((*c).refs[n_4 as usize].p.p.frame_hdr).is_null() {
            dav1d_thread_picture_unref(&mut (*((*c).refs).as_mut_ptr().offset(n_4 as isize)).p);
        }
        dav1d_ref_dec(&mut (*((*c).refs).as_mut_ptr().offset(n_4 as isize)).refmvs);
        dav1d_ref_dec(&mut (*((*c).refs).as_mut_ptr().offset(n_4 as isize)).segmap);
        n_4 += 1;
    }
    dav1d_ref_dec(&mut (*c).seq_hdr_ref);
    dav1d_ref_dec(&mut (*c).frame_hdr_ref);
    dav1d_ref_dec(&mut (*c).mastering_display_ref);
    dav1d_ref_dec(&mut (*c).content_light_ref);
    dav1d_ref_dec(&mut (*c).itut_t35_ref);
    dav1d_mem_pool_end((*c).seq_hdr_pool);
    dav1d_mem_pool_end((*c).frame_hdr_pool);
    dav1d_mem_pool_end((*c).segmap_pool);
    dav1d_mem_pool_end((*c).refmvs_pool);
    dav1d_mem_pool_end((*c).cdf_pool);
    dav1d_mem_pool_end((*c).picture_pool);
    dav1d_freep_aligned(c_out as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_event_flags(
    c: *mut Dav1dContext,
    flags: *mut Dav1dEventFlags,
) -> libc::c_int {
    if c.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"c != NULL\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"dav1d_get_event_flags\0"))
                .as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    if flags.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"flags != NULL\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"dav1d_get_event_flags\0"))
                .as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    *flags = (*c).event_flags;
    (*c).event_flags = 0 as Dav1dEventFlags;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_decode_error_data_props(
    c: *mut Dav1dContext,
    out: *mut Dav1dDataProps,
) -> libc::c_int {
    if c.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"c != NULL\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                b"dav1d_get_decode_error_data_props\0",
            ))
            .as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    if out.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"out != NULL\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                b"dav1d_get_decode_error_data_props\0",
            ))
            .as_ptr(),
        );
        return -(22 as libc::c_int);
    }
    dav1d_data_props_unref_internal(out);
    *out = (*c).cached_error_props;
    dav1d_data_props_set_defaults(&mut (*c).cached_error_props);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_picture_unref(p: *mut Dav1dPicture) {
    dav1d_picture_unref_internal(p);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_create(buf: *mut Dav1dData, sz: size_t) -> *mut uint8_t {
    return dav1d_data_create_internal(buf, sz);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_wrap(
    buf: *mut Dav1dData,
    ptr: *const uint8_t,
    sz: size_t,
    free_callback: Option<unsafe extern "C" fn(*const uint8_t, *mut libc::c_void) -> ()>,
    user_data: *mut libc::c_void,
) -> libc::c_int {
    return dav1d_data_wrap_internal(buf, ptr, sz, free_callback, user_data);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_wrap_user_data(
    buf: *mut Dav1dData,
    user_data: *const uint8_t,
    free_callback: Option<unsafe extern "C" fn(*const uint8_t, *mut libc::c_void) -> ()>,
    cookie: *mut libc::c_void,
) -> libc::c_int {
    return dav1d_data_wrap_user_data_internal(buf, user_data, free_callback, cookie);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_unref(buf: *mut Dav1dData) {
    dav1d_data_unref_internal(buf);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_props_unref(props: *mut Dav1dDataProps) {
    dav1d_data_props_unref_internal(props);
}
