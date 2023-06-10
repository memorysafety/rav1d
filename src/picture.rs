use crate::include::stddef::*;
use crate::include::stdint::*;
use crate::src::cdf::CdfContext;
use crate::src::msac::MsacContext;
use crate::{errno_location, stderr};
use ::libc;
use ::libc::malloc;
extern "C" {
    fn fprintf(_: *mut libc::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: size_t) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn dav1d_mem_pool_push(pool: *mut Dav1dMemPool, buf: *mut Dav1dMemPoolBuffer);
    fn dav1d_mem_pool_pop(pool: *mut Dav1dMemPool, size: size_t) -> *mut Dav1dMemPoolBuffer;
    fn dav1d_ref_wrap(
        ptr: *const uint8_t,
        free_callback: Option<unsafe extern "C" fn(*const uint8_t, *mut libc::c_void) -> ()>,
        user_data: *mut libc::c_void,
    ) -> *mut Dav1dRef;
    fn dav1d_ref_dec(r#ref: *mut *mut Dav1dRef);
    fn dav1d_data_props_copy(dst: *mut Dav1dDataProps, src: *const Dav1dDataProps);
    fn dav1d_data_props_set_defaults(props: *mut Dav1dDataProps);
    fn dav1d_log(c: *mut Dav1dContext, format: *const libc::c_char, _: ...);
}
use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::stdatomic::atomic_int;
use crate::include::stdatomic::atomic_uint;
use crate::src::r#ref::Dav1dRef;
use crate::include::dav1d::headers::Dav1dContentLightLevel;
use crate::include::dav1d::headers::Dav1dITUTT35;
use crate::include::dav1d::headers::Dav1dMasteringDisplay;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;
use crate::include::dav1d::picture::Dav1dPicture;
use crate::src::internal::Dav1dFrameContext_task_thread;
use crate::src::internal::FrameTileThreadData;
use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I400;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;

use crate::include::dav1d::headers::Dav1dFilmGrainData;
use crate::include::dav1d::headers::Dav1dSequenceHeader;

use crate::src::align::Align16;

use crate::src::lf_mask::Av1Filter;
pub type pixel = ();
use crate::src::lf_mask::Av1FilterLUT;
use crate::src::lf_mask::Av1Restoration;
use crate::src::lf_mask::Av1RestorationUnit;
pub type coef = ();
use crate::src::internal::CodedBlockInfo;
use crate::src::levels::Av1Block;
use crate::src::refmvs::refmvs_frame;

use crate::src::env::BlockContext;
use crate::src::refmvs::refmvs_temporal_block;
use crate::src::internal::read_coef_blocks_fn;
use crate::src::levels::BlockSize;

use crate::src::internal::Dav1dTaskContext;

use crate::src::internal::Dav1dTileState_tiling;
use crate::src::internal::Dav1dContext;
use crate::include::dav1d::dav1d::Dav1dEventFlags;
use crate::include::dav1d::dav1d::DAV1D_EVENT_FLAG_NEW_OP_PARAMS_INFO;
use crate::include::dav1d::dav1d::DAV1D_EVENT_FLAG_NEW_SEQUENCE;
use crate::src::mem::Dav1dMemPool;
use crate::src::mem::Dav1dMemPoolBuffer;
pub type PictureFlags = libc::c_uint;
pub const PICTURE_FLAG_NEW_TEMPORAL_UNIT: PictureFlags = 4;
pub const PICTURE_FLAG_NEW_OP_PARAMS_INFO: PictureFlags = 2;
pub const PICTURE_FLAG_NEW_SEQUENCE: PictureFlags = 1;
use crate::include::dav1d::picture::Dav1dPicAllocator;
use crate::src::intra_edge::EdgeFlags;

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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dThreadPicture {
    pub p: Dav1dPicture,
    pub visible: libc::c_int,
    pub showable: libc::c_int,
    pub flags: PictureFlags,
    pub progress: *mut atomic_uint,
}

use crate::src::internal::Dav1dTileGroup;
use crate::src::internal::Dav1dFrameContext;
use crate::src::internal::backup_ipred_edge_fn;
use crate::src::internal::filter_sbrow_fn;
pub type recon_b_inter_fn =
    Option<unsafe extern "C" fn(*mut Dav1dTaskContext, BlockSize, *const Av1Block) -> libc::c_int>;
pub type recon_b_intra_fn = Option<
    unsafe extern "C" fn(*mut Dav1dTaskContext, BlockSize, EdgeFlags, *const Av1Block) -> (),
>;
use crate::src::internal::ScalableMotionParams;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pic_ctx_context {
    pub allocator: Dav1dPicAllocator,
    pub pic: Dav1dPicture,
    pub extra_ptr: *mut libc::c_void,
}
use crate::src::r#ref::dav1d_ref_inc;
#[no_mangle]
pub unsafe extern "C" fn dav1d_default_picture_alloc(
    p: *mut Dav1dPicture,
    cookie: *mut libc::c_void,
) -> libc::c_int {
    if !(::core::mem::size_of::<Dav1dMemPoolBuffer>() as libc::c_ulong <= 64 as libc::c_ulong) {
        unreachable!();
    }
    let hbd = ((*p).p.bpc > 8) as libc::c_int;
    let aligned_w = (*p).p.w + 127 & !(127 as libc::c_int);
    let aligned_h = (*p).p.h + 127 & !(127 as libc::c_int);
    let has_chroma = ((*p).p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint)
        as libc::c_int;
    let ss_ver = ((*p).p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_hor = ((*p).p.layout as libc::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
    let mut y_stride: ptrdiff_t = (aligned_w << hbd) as ptrdiff_t;
    let mut uv_stride: ptrdiff_t = if has_chroma != 0 {
        y_stride >> ss_hor
    } else {
        0
    };
    if y_stride & 1023 == 0 {
        y_stride += 64;
    }
    if uv_stride & 1023 == 0 && has_chroma != 0 {
        uv_stride += 64;
    }
    (*p).stride[0] = y_stride;
    (*p).stride[1] = uv_stride;
    let y_sz: size_t = (y_stride * aligned_h as isize) as size_t;
    let uv_sz: size_t = (uv_stride * (aligned_h >> ss_ver) as isize) as size_t;
    let pic_size: size_t = y_sz.wrapping_add(2usize.wrapping_mul(uv_sz));
    let buf: *mut Dav1dMemPoolBuffer = dav1d_mem_pool_pop(
        cookie as *mut Dav1dMemPool,
        pic_size
            .wrapping_add(64)
            .wrapping_sub(::core::mem::size_of::<Dav1dMemPoolBuffer>()),
    );
    if buf.is_null() {
        return -(12 as libc::c_int);
    }
    (*p).allocator_data = buf as *mut libc::c_void;
    let data: *mut uint8_t = (*buf).data as *mut uint8_t;
    (*p).data[0] = data as *mut libc::c_void;
    (*p).data[1] = (if has_chroma != 0 {
        data.offset(y_sz as isize)
    } else {
        0 as *mut uint8_t
    }) as *mut libc::c_void;
    (*p).data[2] = (if has_chroma != 0 {
        data.offset(y_sz as isize).offset(uv_sz as isize)
    } else {
        0 as *mut uint8_t
    }) as *mut libc::c_void;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_default_picture_release(
    p: *mut Dav1dPicture,
    cookie: *mut libc::c_void,
) {
    dav1d_mem_pool_push(
        cookie as *mut Dav1dMemPool,
        (*p).allocator_data as *mut Dav1dMemPoolBuffer,
    );
}
unsafe extern "C" fn free_buffer(_data: *const uint8_t, user_data: *mut libc::c_void) {
    let mut pic_ctx: *mut pic_ctx_context = user_data as *mut pic_ctx_context;
    ((*pic_ctx).allocator.release_picture_callback).expect("non-null function pointer")(
        &mut (*pic_ctx).pic,
        (*pic_ctx).allocator.cookie,
    );
    free(pic_ctx as *mut libc::c_void);
}
unsafe extern "C" fn picture_alloc_with_edges(
    c: *mut Dav1dContext,
    p: *mut Dav1dPicture,
    w: libc::c_int,
    h: libc::c_int,
    seq_hdr: *mut Dav1dSequenceHeader,
    seq_hdr_ref: *mut Dav1dRef,
    frame_hdr: *mut Dav1dFrameHeader,
    frame_hdr_ref: *mut Dav1dRef,
    content_light: *mut Dav1dContentLightLevel,
    content_light_ref: *mut Dav1dRef,
    mastering_display: *mut Dav1dMasteringDisplay,
    mastering_display_ref: *mut Dav1dRef,
    itut_t35: *mut Dav1dITUTT35,
    itut_t35_ref: *mut Dav1dRef,
    bpc: libc::c_int,
    props: *const Dav1dDataProps,
    p_allocator: *mut Dav1dPicAllocator,
    extra: size_t,
    extra_ptr: *mut *mut libc::c_void,
) -> libc::c_int {
    if !((*p).data[0]).is_null() {
        dav1d_log(
            c,
            b"Picture already allocated!\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if !(bpc > 0 && bpc <= 16) {
        unreachable!();
    }
    let mut pic_ctx: *mut pic_ctx_context =
        malloc(extra.wrapping_add(::core::mem::size_of::<pic_ctx_context>()))
            as *mut pic_ctx_context;
    if pic_ctx.is_null() {
        return -(12 as libc::c_int);
    }
    (*p).p.w = w;
    (*p).p.h = h;
    (*p).seq_hdr = seq_hdr;
    (*p).frame_hdr = frame_hdr;
    (*p).content_light = content_light;
    (*p).mastering_display = mastering_display;
    (*p).itut_t35 = itut_t35;
    (*p).p.layout = (*seq_hdr).layout;
    (*p).p.bpc = bpc;
    dav1d_data_props_set_defaults(&mut (*p).m);
    let res = ((*p_allocator).alloc_picture_callback).expect("non-null function pointer")(
        p,
        (*p_allocator).cookie,
    );
    if res < 0 {
        free(pic_ctx as *mut libc::c_void);
        return res;
    }
    (*pic_ctx).allocator = *p_allocator;
    (*pic_ctx).pic = *p;
    (*p).r#ref = dav1d_ref_wrap(
        (*p).data[0] as *const uint8_t,
        Some(free_buffer as unsafe extern "C" fn(*const uint8_t, *mut libc::c_void) -> ()),
        pic_ctx as *mut libc::c_void,
    );
    if ((*p).r#ref).is_null() {
        ((*p_allocator).release_picture_callback).expect("non-null function pointer")(
            p,
            (*p_allocator).cookie,
        );
        free(pic_ctx as *mut libc::c_void);
        dav1d_log(
            c,
            b"Failed to wrap picture: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*errno_location()),
        );
        return -(12 as libc::c_int);
    }
    (*p).seq_hdr_ref = seq_hdr_ref;
    if !seq_hdr_ref.is_null() {
        dav1d_ref_inc(seq_hdr_ref);
    }
    (*p).frame_hdr_ref = frame_hdr_ref;
    if !frame_hdr_ref.is_null() {
        dav1d_ref_inc(frame_hdr_ref);
    }
    dav1d_data_props_copy(&mut (*p).m, props);
    if extra != 0 && !extra_ptr.is_null() {
        *extra_ptr = &mut (*pic_ctx).extra_ptr as *mut *mut libc::c_void as *mut libc::c_void;
    }
    (*p).content_light_ref = content_light_ref;
    if !content_light_ref.is_null() {
        dav1d_ref_inc(content_light_ref);
    }
    (*p).mastering_display_ref = mastering_display_ref;
    if !mastering_display_ref.is_null() {
        dav1d_ref_inc(mastering_display_ref);
    }
    (*p).itut_t35_ref = itut_t35_ref;
    if !itut_t35_ref.is_null() {
        dav1d_ref_inc(itut_t35_ref);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_thread_picture_alloc(
    c: *mut Dav1dContext,
    f: *mut Dav1dFrameContext,
    bpc: libc::c_int,
) -> libc::c_int {
    let p: *mut Dav1dThreadPicture = &mut (*f).sr_cur;
    let have_frame_mt = ((*c).n_fc > 1 as libc::c_uint) as libc::c_int;
    let res = picture_alloc_with_edges(
        c,
        &mut (*p).p,
        (*(*f).frame_hdr).width[1],
        (*(*f).frame_hdr).height,
        (*f).seq_hdr,
        (*f).seq_hdr_ref,
        (*f).frame_hdr,
        (*f).frame_hdr_ref,
        (*c).content_light,
        (*c).content_light_ref,
        (*c).mastering_display,
        (*c).mastering_display_ref,
        (*c).itut_t35,
        (*c).itut_t35_ref,
        bpc,
        &mut (*((*f).tile).offset(0)).data.m,
        &mut (*c).allocator,
        if have_frame_mt != 0 {
            (::core::mem::size_of::<atomic_int>()).wrapping_mul(2)
        } else {
            0
        },
        &mut (*p).progress as *mut *mut atomic_uint as *mut *mut libc::c_void,
    );
    if res != 0 {
        return res;
    }
    dav1d_ref_dec(&mut (*c).itut_t35_ref);
    (*c).itut_t35 = 0 as *mut Dav1dITUTT35;
    let flags_mask = if (*(*f).frame_hdr).show_frame != 0 || (*c).output_invisible_frames != 0 {
        0 as libc::c_int
    } else {
        PICTURE_FLAG_NEW_SEQUENCE as libc::c_int | PICTURE_FLAG_NEW_OP_PARAMS_INFO as libc::c_int
    };
    (*p).flags = (*c).frame_flags;
    (*c).frame_flags = ::core::mem::transmute::<libc::c_uint, PictureFlags>(
        (*c).frame_flags as libc::c_uint & flags_mask as libc::c_uint,
    );
    (*p).visible = (*(*f).frame_hdr).show_frame;
    (*p).showable = (*(*f).frame_hdr).showable_frame;
    if have_frame_mt != 0 {
        *(&mut *((*p).progress).offset(0) as *mut atomic_uint) = 0 as libc::c_int as libc::c_uint;
        *(&mut *((*p).progress).offset(1) as *mut atomic_uint) = 0 as libc::c_int as libc::c_uint;
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_picture_alloc_copy(
    c: *mut Dav1dContext,
    dst: *mut Dav1dPicture,
    w: libc::c_int,
    src: *const Dav1dPicture,
) -> libc::c_int {
    let pic_ctx: *mut pic_ctx_context = (*(*src).r#ref).user_data as *mut pic_ctx_context;
    let res = picture_alloc_with_edges(
        c,
        dst,
        w,
        (*src).p.h,
        (*src).seq_hdr,
        (*src).seq_hdr_ref,
        (*src).frame_hdr,
        (*src).frame_hdr_ref,
        (*src).content_light,
        (*src).content_light_ref,
        (*src).mastering_display,
        (*src).mastering_display_ref,
        (*src).itut_t35,
        (*src).itut_t35_ref,
        (*src).p.bpc,
        &(*src).m,
        &mut (*pic_ctx).allocator,
        0 as libc::c_int as size_t,
        0 as *mut *mut libc::c_void,
    );
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_picture_ref(dst: *mut Dav1dPicture, src: *const Dav1dPicture) {
    if dst.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"dst != ((void*)0)\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"dav1d_picture_ref\0"))
                .as_ptr(),
        );
        return;
    }
    if !((*dst).data[0]).is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"dst->data[0] == ((void*)0)\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"dav1d_picture_ref\0"))
                .as_ptr(),
        );
        return;
    }
    if src.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"src != ((void*)0)\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"dav1d_picture_ref\0"))
                .as_ptr(),
        );
        return;
    }
    if !((*src).r#ref).is_null() {
        if ((*src).data[0]).is_null() {
            fprintf(
                stderr,
                b"Input validation check '%s' failed in %s!\n\0" as *const u8
                    as *const libc::c_char,
                b"src->data[0] != ((void*)0)\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"dav1d_picture_ref\0"))
                    .as_ptr(),
            );
            return;
        }
        dav1d_ref_inc((*src).r#ref);
    }
    if !((*src).frame_hdr_ref).is_null() {
        dav1d_ref_inc((*src).frame_hdr_ref);
    }
    if !((*src).seq_hdr_ref).is_null() {
        dav1d_ref_inc((*src).seq_hdr_ref);
    }
    if !((*src).m.user_data.r#ref).is_null() {
        dav1d_ref_inc((*src).m.user_data.r#ref);
    }
    if !((*src).content_light_ref).is_null() {
        dav1d_ref_inc((*src).content_light_ref);
    }
    if !((*src).mastering_display_ref).is_null() {
        dav1d_ref_inc((*src).mastering_display_ref);
    }
    if !((*src).itut_t35_ref).is_null() {
        dav1d_ref_inc((*src).itut_t35_ref);
    }
    *dst = *src;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_picture_move_ref(dst: *mut Dav1dPicture, src: *mut Dav1dPicture) {
    if dst.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"dst != ((void*)0)\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"dav1d_picture_move_ref\0",
            ))
            .as_ptr(),
        );
        return;
    }
    if !((*dst).data[0]).is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"dst->data[0] == ((void*)0)\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"dav1d_picture_move_ref\0",
            ))
            .as_ptr(),
        );
        return;
    }
    if src.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"src != ((void*)0)\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"dav1d_picture_move_ref\0",
            ))
            .as_ptr(),
        );
        return;
    }
    if !((*src).r#ref).is_null() {
        if ((*src).data[0]).is_null() {
            fprintf(
                stderr,
                b"Input validation check '%s' failed in %s!\n\0" as *const u8
                    as *const libc::c_char,
                b"src->data[0] != ((void*)0)\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                    b"dav1d_picture_move_ref\0",
                ))
                .as_ptr(),
            );
            return;
        }
    }
    *dst = *src;
    memset(
        src as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Dav1dPicture>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_thread_picture_ref(
    dst: *mut Dav1dThreadPicture,
    src: *const Dav1dThreadPicture,
) {
    dav1d_picture_ref(&mut (*dst).p, &(*src).p);
    (*dst).visible = (*src).visible;
    (*dst).showable = (*src).showable;
    (*dst).progress = (*src).progress;
    (*dst).flags = (*src).flags;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_thread_picture_move_ref(
    dst: *mut Dav1dThreadPicture,
    src: *mut Dav1dThreadPicture,
) {
    dav1d_picture_move_ref(&mut (*dst).p, &mut (*src).p);
    (*dst).visible = (*src).visible;
    (*dst).showable = (*src).showable;
    (*dst).progress = (*src).progress;
    (*dst).flags = (*src).flags;
    memset(
        src as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Dav1dThreadPicture>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_picture_unref_internal(p: *mut Dav1dPicture) {
    if p.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const libc::c_char,
            b"p != ((void*)0)\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                b"dav1d_picture_unref_internal\0",
            ))
            .as_ptr(),
        );
        return;
    }
    if !((*p).r#ref).is_null() {
        if ((*p).data[0]).is_null() {
            fprintf(
                stderr,
                b"Input validation check '%s' failed in %s!\n\0" as *const u8
                    as *const libc::c_char,
                b"p->data[0] != ((void*)0)\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                    b"dav1d_picture_unref_internal\0",
                ))
                .as_ptr(),
            );
            return;
        }
        dav1d_ref_dec(&mut (*p).r#ref);
    }
    dav1d_ref_dec(&mut (*p).seq_hdr_ref);
    dav1d_ref_dec(&mut (*p).frame_hdr_ref);
    dav1d_ref_dec(&mut (*p).m.user_data.r#ref);
    dav1d_ref_dec(&mut (*p).content_light_ref);
    dav1d_ref_dec(&mut (*p).mastering_display_ref);
    dav1d_ref_dec(&mut (*p).itut_t35_ref);
    memset(
        p as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Dav1dPicture>(),
    );
    dav1d_data_props_set_defaults(&mut (*p).m);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_thread_picture_unref(p: *mut Dav1dThreadPicture) {
    dav1d_picture_unref_internal(&mut (*p).p);
    (*p).progress = 0 as *mut atomic_uint;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_picture_get_event_flags(
    p: *const Dav1dThreadPicture,
) -> Dav1dEventFlags {
    if (*p).flags as u64 == 0 {
        return 0 as Dav1dEventFlags;
    }
    let mut flags: Dav1dEventFlags = 0 as Dav1dEventFlags;
    if (*p).flags as libc::c_uint & PICTURE_FLAG_NEW_SEQUENCE as libc::c_int as libc::c_uint != 0 {
        flags = ::core::mem::transmute::<libc::c_uint, Dav1dEventFlags>(
            flags as libc::c_uint | DAV1D_EVENT_FLAG_NEW_SEQUENCE as libc::c_int as libc::c_uint,
        );
    }
    if (*p).flags as libc::c_uint & PICTURE_FLAG_NEW_OP_PARAMS_INFO as libc::c_int as libc::c_uint
        != 0
    {
        flags = ::core::mem::transmute::<libc::c_uint, Dav1dEventFlags>(
            flags as libc::c_uint
                | DAV1D_EVENT_FLAG_NEW_OP_PARAMS_INFO as libc::c_int as libc::c_uint,
        );
    }
    return flags;
}
