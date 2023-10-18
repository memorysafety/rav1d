use crate::errno_location;
use crate::include::dav1d::common::Rav1dDataProps;
use crate::include::dav1d::dav1d::Dav1dEventFlags;
use crate::include::dav1d::dav1d::Rav1dEventFlags;
use crate::include::dav1d::dav1d::RAV1D_EVENT_FLAG_NEW_OP_PARAMS_INFO;
use crate::include::dav1d::dav1d::RAV1D_EVENT_FLAG_NEW_SEQUENCE;
use crate::include::dav1d::headers::Rav1dContentLightLevel;
use crate::include::dav1d::headers::Rav1dFrameHeader;
use crate::include::dav1d::headers::Rav1dITUTT35;
use crate::include::dav1d::headers::Rav1dMasteringDisplay;
use crate::include::dav1d::headers::Rav1dSequenceHeader;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I400;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I444;
use crate::include::dav1d::picture::Dav1dPicture;
use crate::include::dav1d::picture::Rav1dPicAllocator;
use crate::include::dav1d::picture::Rav1dPicture;
use crate::include::stdatomic::atomic_int;
use crate::include::stdatomic::atomic_uint;
use crate::src::data::rav1d_data_props_copy;
use crate::src::data::rav1d_data_props_set_defaults;
use crate::src::internal::Rav1dContext;
use crate::src::internal::Rav1dFrameContext;
use crate::src::log::rav1d_log;
use crate::src::mem::rav1d_mem_pool_pop;
use crate::src::mem::rav1d_mem_pool_push;
use crate::src::mem::Rav1dMemPool;
use crate::src::mem::Rav1dMemPoolBuffer;
use crate::src::r#ref::rav1d_ref_dec;
use crate::src::r#ref::rav1d_ref_inc;
use crate::src::r#ref::rav1d_ref_wrap;
use crate::src::r#ref::Rav1dRef;
use crate::stderr;
use libc::fprintf;
use libc::free;
use libc::malloc;
use libc::memset;
use libc::ptrdiff_t;
use libc::strerror;
use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_ulong;
use std::ffi::c_void;

pub type PictureFlags = c_uint;
pub const PICTURE_FLAG_NEW_TEMPORAL_UNIT: PictureFlags = 4;
pub const PICTURE_FLAG_NEW_OP_PARAMS_INFO: PictureFlags = 2;
pub const PICTURE_FLAG_NEW_SEQUENCE: PictureFlags = 1;

#[repr(C)]
pub(crate) struct Rav1dThreadPicture {
    pub p: Rav1dPicture,
    pub visible: bool,
    pub showable: bool,
    pub flags: PictureFlags,
    pub progress: *mut atomic_uint,
}

#[repr(C)]
pub(crate) struct pic_ctx_context {
    pub allocator: Rav1dPicAllocator,
    pub pic: Rav1dPicture,
    pub extra_ptr: *mut c_void,
}

pub unsafe extern "C" fn dav1d_default_picture_alloc(
    p: *mut Dav1dPicture,
    cookie: *mut c_void,
) -> c_int {
    if !(::core::mem::size_of::<Rav1dMemPoolBuffer>() as c_ulong <= 64 as c_ulong) {
        unreachable!();
    }
    let hbd = ((*p).p.bpc > 8) as c_int;
    let aligned_w = (*p).p.w + 127 & !(127 as c_int);
    let aligned_h = (*p).p.h + 127 & !(127 as c_int);
    let has_chroma =
        ((*p).p.layout as c_uint != RAV1D_PIXEL_LAYOUT_I400 as c_int as c_uint) as c_int;
    let ss_ver = ((*p).p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint) as c_int;
    let ss_hor = ((*p).p.layout as c_uint != RAV1D_PIXEL_LAYOUT_I444 as c_int as c_uint) as c_int;
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
    let y_sz: usize = (y_stride * aligned_h as isize) as usize;
    let uv_sz: usize = (uv_stride * (aligned_h >> ss_ver) as isize) as usize;
    let pic_size: usize = y_sz.wrapping_add(2usize.wrapping_mul(uv_sz));
    let buf: *mut Rav1dMemPoolBuffer = rav1d_mem_pool_pop(
        cookie as *mut Rav1dMemPool,
        pic_size
            .wrapping_add(64)
            .wrapping_sub(::core::mem::size_of::<Rav1dMemPoolBuffer>()),
    );
    if buf.is_null() {
        return -(12 as c_int);
    }
    (*p).allocator_data = buf as *mut c_void;
    let data: *mut u8 = (*buf).data as *mut u8;
    (*p).data[0] = data as *mut c_void;
    (*p).data[1] = (if has_chroma != 0 {
        data.offset(y_sz as isize)
    } else {
        0 as *mut u8
    }) as *mut c_void;
    (*p).data[2] = (if has_chroma != 0 {
        data.offset(y_sz as isize).offset(uv_sz as isize)
    } else {
        0 as *mut u8
    }) as *mut c_void;
    return 0 as c_int;
}

pub unsafe extern "C" fn dav1d_default_picture_release(p: *mut Dav1dPicture, cookie: *mut c_void) {
    rav1d_mem_pool_push(
        cookie as *mut Rav1dMemPool,
        (*p).allocator_data as *mut Rav1dMemPoolBuffer,
    );
}

unsafe extern "C" fn free_buffer(_data: *const u8, user_data: *mut c_void) {
    let pic_ctx: *mut pic_ctx_context = user_data as *mut pic_ctx_context;
    (*pic_ctx).allocator.release_picture(&mut (*pic_ctx).pic);
    free(pic_ctx as *mut c_void);
}

unsafe extern "C" fn picture_alloc_with_edges(
    c: *mut Rav1dContext,
    p: *mut Rav1dPicture,
    w: c_int,
    h: c_int,
    seq_hdr: *mut Rav1dSequenceHeader,
    seq_hdr_ref: *mut Rav1dRef,
    frame_hdr: *mut Rav1dFrameHeader,
    frame_hdr_ref: *mut Rav1dRef,
    content_light: *mut Rav1dContentLightLevel,
    content_light_ref: *mut Rav1dRef,
    mastering_display: *mut Rav1dMasteringDisplay,
    mastering_display_ref: *mut Rav1dRef,
    itut_t35: *mut Rav1dITUTT35,
    itut_t35_ref: *mut Rav1dRef,
    bpc: c_int,
    props: *const Rav1dDataProps,
    p_allocator: *mut Rav1dPicAllocator,
    extra: usize,
    extra_ptr: *mut *mut c_void,
) -> c_int {
    if !((*p).data[0]).is_null() {
        rav1d_log(
            c,
            b"Picture already allocated!\n\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    if !(bpc > 0 && bpc <= 16) {
        unreachable!();
    }
    let pic_ctx: *mut pic_ctx_context =
        malloc(extra.wrapping_add(::core::mem::size_of::<pic_ctx_context>()))
            as *mut pic_ctx_context;
    if pic_ctx.is_null() {
        return -(12 as c_int);
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
    rav1d_data_props_set_defaults(&mut (*p).m);
    (*p).seq_hdr_ref = seq_hdr_ref;
    (*p).frame_hdr_ref = frame_hdr_ref;
    (*p).itut_t35_ref = itut_t35_ref;
    let res = (*p_allocator).alloc_picture(p);
    if res < 0 {
        free(pic_ctx as *mut c_void);
        return res;
    }
    (*pic_ctx).allocator = (*p_allocator).clone();
    (*pic_ctx).pic = (*p).clone();
    (*p).r#ref = rav1d_ref_wrap(
        (*p).data[0] as *const u8,
        Some(free_buffer as unsafe extern "C" fn(*const u8, *mut c_void) -> ()),
        pic_ctx as *mut c_void,
    );
    if ((*p).r#ref).is_null() {
        (*p_allocator).release_picture(p);
        free(pic_ctx as *mut c_void);
        rav1d_log(
            c,
            b"Failed to wrap picture: %s\n\0" as *const u8 as *const c_char,
            strerror(*errno_location()),
        );
        return -(12 as c_int);
    }
    if !seq_hdr_ref.is_null() {
        rav1d_ref_inc(seq_hdr_ref);
    }
    if !frame_hdr_ref.is_null() {
        rav1d_ref_inc(frame_hdr_ref);
    }
    rav1d_data_props_copy(&mut (*p).m, props);
    if extra != 0 && !extra_ptr.is_null() {
        *extra_ptr = &mut (*pic_ctx).extra_ptr as *mut *mut c_void as *mut c_void;
    }
    (*p).content_light_ref = content_light_ref;
    if !content_light_ref.is_null() {
        rav1d_ref_inc(content_light_ref);
    }
    (*p).mastering_display_ref = mastering_display_ref;
    if !mastering_display_ref.is_null() {
        rav1d_ref_inc(mastering_display_ref);
    }
    if !itut_t35_ref.is_null() {
        rav1d_ref_inc(itut_t35_ref);
    }
    return 0 as c_int;
}

pub(crate) unsafe fn rav1d_thread_picture_alloc(
    c: *mut Rav1dContext,
    f: *mut Rav1dFrameContext,
    bpc: c_int,
) -> c_int {
    let p: *mut Rav1dThreadPicture = &mut (*f).sr_cur;
    let have_frame_mt = ((*c).n_fc > 1 as c_uint) as c_int;
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
        &mut (*p).progress as *mut *mut atomic_uint as *mut *mut c_void,
    );
    if res != 0 {
        return res;
    }
    rav1d_ref_dec(&mut (*c).itut_t35_ref);
    (*c).itut_t35 = 0 as *mut Rav1dITUTT35;
    let flags_mask = if (*(*f).frame_hdr).show_frame != 0 || (*c).output_invisible_frames != 0 {
        0 as c_int
    } else {
        PICTURE_FLAG_NEW_SEQUENCE as c_int | PICTURE_FLAG_NEW_OP_PARAMS_INFO as c_int
    };
    (*p).flags = (*c).frame_flags;
    (*c).frame_flags = ::core::mem::transmute::<c_uint, PictureFlags>(
        (*c).frame_flags as c_uint & flags_mask as c_uint,
    );
    (*p).visible = (*(*f).frame_hdr).show_frame != 0;
    (*p).showable = (*(*f).frame_hdr).showable_frame != 0;
    if have_frame_mt != 0 {
        *(&mut *((*p).progress).offset(0) as *mut atomic_uint) = 0 as c_int as c_uint;
        *(&mut *((*p).progress).offset(1) as *mut atomic_uint) = 0 as c_int as c_uint;
    }
    return res;
}

pub(crate) unsafe fn rav1d_picture_alloc_copy(
    c: *mut Rav1dContext,
    dst: *mut Rav1dPicture,
    w: c_int,
    src: *const Rav1dPicture,
) -> c_int {
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
        0 as c_int as usize,
        0 as *mut *mut c_void,
    );
    return res;
}

pub(crate) unsafe fn rav1d_picture_ref(dst: *mut Rav1dPicture, src: *const Rav1dPicture) {
    if dst.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"dst != ((void*)0)\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"dav1d_picture_ref\0")).as_ptr(),
        );
        return;
    }
    if !((*dst).data[0]).is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"dst->data[0] == ((void*)0)\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"dav1d_picture_ref\0")).as_ptr(),
        );
        return;
    }
    if src.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"src != ((void*)0)\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"dav1d_picture_ref\0")).as_ptr(),
        );
        return;
    }
    if !((*src).r#ref).is_null() {
        if ((*src).data[0]).is_null() {
            fprintf(
                stderr,
                b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
                b"src->data[0] != ((void*)0)\0" as *const u8 as *const c_char,
                (*::core::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"dav1d_picture_ref\0"))
                    .as_ptr(),
            );
            return;
        }
        rav1d_ref_inc((*src).r#ref);
    }
    if !((*src).frame_hdr_ref).is_null() {
        rav1d_ref_inc((*src).frame_hdr_ref);
    }
    if !((*src).seq_hdr_ref).is_null() {
        rav1d_ref_inc((*src).seq_hdr_ref);
    }
    if !((*src).m.user_data.r#ref).is_null() {
        rav1d_ref_inc((*src).m.user_data.r#ref);
    }
    if !((*src).content_light_ref).is_null() {
        rav1d_ref_inc((*src).content_light_ref);
    }
    if !((*src).mastering_display_ref).is_null() {
        rav1d_ref_inc((*src).mastering_display_ref);
    }
    if !((*src).itut_t35_ref).is_null() {
        rav1d_ref_inc((*src).itut_t35_ref);
    }
    *dst = (*src).clone();
}

pub(crate) unsafe fn rav1d_picture_move_ref(dst: *mut Rav1dPicture, src: *mut Rav1dPicture) {
    if dst.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"dst != ((void*)0)\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 23], &[c_char; 23]>(b"rav1d_picture_move_ref\0"))
                .as_ptr(),
        );
        return;
    }
    if !((*dst).data[0]).is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"dst->data[0] == ((void*)0)\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 23], &[c_char; 23]>(b"rav1d_picture_move_ref\0"))
                .as_ptr(),
        );
        return;
    }
    if src.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"src != ((void*)0)\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 23], &[c_char; 23]>(b"rav1d_picture_move_ref\0"))
                .as_ptr(),
        );
        return;
    }
    if !((*src).r#ref).is_null() {
        if ((*src).data[0]).is_null() {
            fprintf(
                stderr,
                b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
                b"src->data[0] != ((void*)0)\0" as *const u8 as *const c_char,
                (*::core::mem::transmute::<&[u8; 23], &[c_char; 23]>(b"rav1d_picture_move_ref\0"))
                    .as_ptr(),
            );
            return;
        }
    }
    *dst = (*src).clone();
    memset(
        src as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<Rav1dPicture>(),
    );
}

pub(crate) unsafe fn rav1d_thread_picture_ref(
    dst: *mut Rav1dThreadPicture,
    src: *const Rav1dThreadPicture,
) {
    rav1d_picture_ref(&mut (*dst).p, &(*src).p);
    (*dst).visible = (*src).visible;
    (*dst).showable = (*src).showable;
    (*dst).progress = (*src).progress;
    (*dst).flags = (*src).flags;
}

pub(crate) unsafe fn rav1d_thread_picture_move_ref(
    dst: *mut Rav1dThreadPicture,
    src: *mut Rav1dThreadPicture,
) {
    rav1d_picture_move_ref(&mut (*dst).p, &mut (*src).p);
    (*dst).visible = (*src).visible;
    (*dst).showable = (*src).showable;
    (*dst).progress = (*src).progress;
    (*dst).flags = (*src).flags;
    memset(
        src as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<Rav1dThreadPicture>(),
    );
}

pub(crate) unsafe fn rav1d_picture_unref_internal(p: *mut Rav1dPicture) {
    if p.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"p != ((void*)0)\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 29], &[c_char; 29]>(
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
                b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
                b"p->data[0] != ((void*)0)\0" as *const u8 as *const c_char,
                (*::core::mem::transmute::<&[u8; 29], &[c_char; 29]>(
                    b"dav1d_picture_unref_internal\0",
                ))
                .as_ptr(),
            );
            return;
        }
        rav1d_ref_dec(&mut (*p).r#ref);
    }
    rav1d_ref_dec(&mut (*p).seq_hdr_ref);
    rav1d_ref_dec(&mut (*p).frame_hdr_ref);
    rav1d_ref_dec(&mut (*p).m.user_data.r#ref);
    rav1d_ref_dec(&mut (*p).content_light_ref);
    rav1d_ref_dec(&mut (*p).mastering_display_ref);
    rav1d_ref_dec(&mut (*p).itut_t35_ref);
    memset(
        p as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<Dav1dPicture>(),
    );
    rav1d_data_props_set_defaults(&mut (*p).m);
}

pub(crate) unsafe fn rav1d_thread_picture_unref(p: *mut Rav1dThreadPicture) {
    rav1d_picture_unref_internal(&mut (*p).p);
    (*p).progress = 0 as *mut atomic_uint;
}

pub(crate) unsafe fn rav1d_picture_get_event_flags(
    p: *const Rav1dThreadPicture,
) -> Rav1dEventFlags {
    if (*p).flags as u64 == 0 {
        return 0 as Dav1dEventFlags;
    }
    let mut flags: Dav1dEventFlags = 0 as Dav1dEventFlags;
    if (*p).flags as c_uint & PICTURE_FLAG_NEW_SEQUENCE as c_int as c_uint != 0 {
        flags = ::core::mem::transmute::<c_uint, Dav1dEventFlags>(
            flags as c_uint | RAV1D_EVENT_FLAG_NEW_SEQUENCE as c_int as c_uint,
        );
    }
    if (*p).flags as c_uint & PICTURE_FLAG_NEW_OP_PARAMS_INFO as c_int as c_uint != 0 {
        flags = ::core::mem::transmute::<c_uint, Dav1dEventFlags>(
            flags as c_uint | RAV1D_EVENT_FLAG_NEW_OP_PARAMS_INFO as c_int as c_uint,
        );
    }
    return flags;
}
