use crate::include::common::validate::validate_input;
use crate::include::dav1d::common::Rav1dDataProps;
use crate::include::dav1d::dav1d::Rav1dEventFlags;
use crate::include::dav1d::headers::DRav1d;
use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::dav1d::headers::Dav1dITUTT35;
use crate::include::dav1d::headers::Dav1dSequenceHeader;
use crate::include::dav1d::headers::Rav1dContentLightLevel;
use crate::include::dav1d::headers::Rav1dFrameHeader;
use crate::include::dav1d::headers::Rav1dITUTT35;
use crate::include::dav1d::headers::Rav1dMasteringDisplay;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::include::dav1d::headers::Rav1dSequenceHeader;
use crate::include::dav1d::picture::Dav1dPicture;
use crate::include::dav1d::picture::Rav1dPicAllocator;
use crate::include::dav1d::picture::Rav1dPicture;
use crate::include::stdatomic::atomic_int;
use crate::include::stdatomic::atomic_uint;
use crate::src::error::Dav1dResult;
use crate::src::error::Rav1dError::EGeneric;
use crate::src::error::Rav1dError::ENOMEM;
use crate::src::error::Rav1dResult;
use crate::src::internal::Rav1dContext;
use crate::src::internal::Rav1dFrameContext;
use crate::src::log::Rav1dLog as _;
use crate::src::log::Rav1dLogger;
use crate::src::mem::rav1d_mem_pool_pop;
use crate::src::mem::rav1d_mem_pool_push;
use crate::src::mem::Rav1dMemPool;
use crate::src::mem::Rav1dMemPoolBuffer;
use crate::src::r#ref::rav1d_ref_dec;
use crate::src::r#ref::rav1d_ref_inc;
use crate::src::r#ref::rav1d_ref_wrap;
use bitflags::bitflags;
use libc::free;
use libc::malloc;
use libc::ptrdiff_t;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_ulong;
use std::ffi::c_void;
use std::io;
use std::mem;
use std::ptr;
use std::ptr::addr_of_mut;
use std::sync::Arc;

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct PictureFlags: u8 {
        const NEW_SEQUENCE = 1 << 0;
        const NEW_OP_PARAMS_INFO = 1 << 1;
        const NEW_TEMPORAL_UNIT = 1 << 2;
    }
}

impl From<PictureFlags> for Rav1dEventFlags {
    fn from(value: PictureFlags) -> Self {
        // [`Rav1dEventFlags`] just has one extra flag vs. [`PictureFlags`],
        // which this just truncates off.
        // Otherwise the values are the same so we can convert the bits.
        Self::from_bits_truncate(value.bits())
    }
}

#[repr(C)]
pub(crate) struct Rav1dThreadPicture {
    pub p: Rav1dPicture,
    pub visible: bool,
    pub showable: bool,
    pub flags: PictureFlags,
    pub progress: *mut atomic_uint,
}

// TODO(kkysen) Eventually the [`impl Default`] might not be needed.
impl Default for Rav1dThreadPicture {
    fn default() -> Self {
        Self {
            p: Default::default(),
            visible: Default::default(),
            showable: Default::default(),
            flags: Default::default(),
            progress: ptr::null_mut(),
        }
    }
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
) -> Dav1dResult {
    if !(::core::mem::size_of::<Rav1dMemPoolBuffer>() as c_ulong <= 64 as c_ulong) {
        unreachable!();
    }
    let hbd = ((*p).p.bpc > 8) as c_int;
    let aligned_w = (*p).p.w + 127 & !(127 as c_int);
    let aligned_h = (*p).p.h + 127 & !(127 as c_int);
    let has_chroma =
        ((*p).p.layout as c_uint != Rav1dPixelLayout::I400 as c_int as c_uint) as c_int;
    let ss_ver = ((*p).p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint) as c_int;
    let ss_hor = ((*p).p.layout as c_uint != Rav1dPixelLayout::I444 as c_int as c_uint) as c_int;
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
        return Rav1dResult::<()>::Err(ENOMEM).into();
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
    Rav1dResult::Ok(()).into()
}

pub unsafe extern "C" fn dav1d_default_picture_release(p: *mut Dav1dPicture, cookie: *mut c_void) {
    rav1d_mem_pool_push(
        cookie as *mut Rav1dMemPool,
        (*p).allocator_data as *mut Rav1dMemPoolBuffer,
    );
}

impl Default for Rav1dPicAllocator {
    fn default() -> Self {
        Self {
            cookie: ptr::null_mut(),
            alloc_picture_callback: Some(dav1d_default_picture_alloc),
            release_picture_callback: Some(dav1d_default_picture_release),
        }
    }
}

unsafe extern "C" fn free_buffer(_data: *const u8, user_data: *mut c_void) {
    let pic_ctx: *mut pic_ctx_context = user_data as *mut pic_ctx_context;
    (*pic_ctx).allocator.release_picture(&mut (*pic_ctx).pic);
    free(pic_ctx as *mut c_void);
}

unsafe fn picture_alloc_with_edges(
    logger: &Option<Rav1dLogger>,
    p: &mut Rav1dPicture,
    w: c_int,
    h: c_int,
    seq_hdr: &Option<Arc<DRav1d<Rav1dSequenceHeader, Dav1dSequenceHeader>>>,
    frame_hdr: &Option<Arc<DRav1d<Rav1dFrameHeader, Dav1dFrameHeader>>>,
    content_light: &Option<Arc<Rav1dContentLightLevel>>,
    mastering_display: &Option<Arc<Rav1dMasteringDisplay>>,
    itut_t35: &Option<Arc<DRav1d<Rav1dITUTT35, Dav1dITUTT35>>>,
    bpc: c_int,
    props: &Rav1dDataProps,
    p_allocator: &mut Rav1dPicAllocator,
    extra: usize,
    extra_ptr: *mut *mut c_void,
) -> Rav1dResult {
    if !p.data[0].is_null() {
        writeln!(logger, "Picture already allocated!",);
        return Err(EGeneric);
    }
    if !(bpc > 0 && bpc <= 16) {
        unreachable!();
    }
    let pic_ctx: *mut pic_ctx_context =
        malloc(extra.wrapping_add(::core::mem::size_of::<pic_ctx_context>()))
            as *mut pic_ctx_context;
    if pic_ctx.is_null() {
        return Err(ENOMEM);
    }
    p.p.w = w;
    p.p.h = h;
    p.seq_hdr = seq_hdr.clone();
    p.frame_hdr = frame_hdr.clone();
    p.p.layout = seq_hdr.as_ref().unwrap().layout;
    p.p.bpc = bpc;
    p.m = Default::default();
    let res = p_allocator.alloc_picture(p);
    if res.is_err() {
        free(pic_ctx as *mut c_void);
        return res;
    }
    (*pic_ctx).allocator = p_allocator.clone();
    // TODO(kkysen) A normal assignment here as it used to be
    // calls `fn drop` on `(*pic_ctx).pic`, which segfaults as it is uninitialized.
    // We need to figure out the right thing to do here.
    addr_of_mut!((*pic_ctx).pic).write(p.clone());
    p.r#ref = rav1d_ref_wrap(
        p.data[0] as *const u8,
        Some(free_buffer),
        pic_ctx as *mut c_void,
    );
    if p.r#ref.is_null() {
        p_allocator.release_picture(p);
        free(pic_ctx as *mut c_void);
        writeln!(
            logger,
            "Failed to wrap picture: {}",
            io::Error::last_os_error(),
        );
        return Err(ENOMEM);
    }
    rav1d_picture_copy_props(p, content_light, mastering_display, itut_t35, props);

    if extra != 0 && !extra_ptr.is_null() {
        *extra_ptr = &mut (*pic_ctx).extra_ptr as *mut *mut c_void as *mut c_void;
    }

    Ok(())
}

pub fn rav1d_picture_copy_props(
    p: &mut Rav1dPicture,
    content_light: &Option<Arc<Rav1dContentLightLevel>>,
    mastering_display: &Option<Arc<Rav1dMasteringDisplay>>,
    itut_t35: &Option<Arc<DRav1d<Rav1dITUTT35, Dav1dITUTT35>>>,
    props: &Rav1dDataProps,
) {
    p.m = props.clone();
    p.content_light = content_light.clone();
    p.mastering_display = mastering_display.clone();
    p.itut_t35 = itut_t35.clone();
}

pub(crate) unsafe fn rav1d_thread_picture_alloc(
    c: &mut Rav1dContext,
    f: &mut Rav1dFrameContext,
    bpc: c_int,
) -> Rav1dResult {
    let p = &mut f.sr_cur;
    let have_frame_mt = c.n_fc > 1;
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    picture_alloc_with_edges(
        &c.logger,
        &mut p.p,
        frame_hdr.size.width[1],
        frame_hdr.size.height,
        &f.seq_hdr,
        &f.frame_hdr,
        &c.content_light,
        &c.mastering_display,
        &c.itut_t35,
        bpc,
        &mut f.tiles[0].data.m,
        &mut c.allocator,
        if have_frame_mt {
            (::core::mem::size_of::<atomic_int>()).wrapping_mul(2)
        } else {
            0
        },
        &mut p.progress as *mut *mut atomic_uint as *mut *mut c_void,
    )?;
    let _ = mem::take(&mut c.itut_t35);
    let flags_mask = if frame_hdr.show_frame != 0 || c.output_invisible_frames {
        PictureFlags::empty()
    } else {
        PictureFlags::NEW_SEQUENCE | PictureFlags::NEW_OP_PARAMS_INFO
    };
    p.flags = c.frame_flags;
    c.frame_flags &= flags_mask;
    p.visible = frame_hdr.show_frame != 0;
    p.showable = frame_hdr.showable_frame != 0;
    if have_frame_mt {
        *p.progress.add(0) = 0;
        *p.progress.add(1) = 0;
    }
    Ok(())
}

pub(crate) unsafe fn rav1d_picture_alloc_copy(
    c: &mut Rav1dContext,
    dst: *mut Rav1dPicture,
    w: c_int,
    src: *const Rav1dPicture,
) -> Rav1dResult {
    let pic_ctx: *mut pic_ctx_context = (*(*src).r#ref).user_data as *mut pic_ctx_context;
    let res = picture_alloc_with_edges(
        &c.logger,
        &mut *dst,
        w,
        (*src).p.h,
        &(*src).seq_hdr,
        &(*src).frame_hdr,
        &(*src).content_light,
        &(*src).mastering_display,
        &(*src).itut_t35,
        (*src).p.bpc,
        &(*src).m,
        &mut (*pic_ctx).allocator,
        0 as c_int as usize,
        0 as *mut *mut c_void,
    );
    return res;
}

pub(crate) unsafe fn rav1d_picture_ref(dst: &mut Rav1dPicture, src: &Rav1dPicture) {
    if validate_input!(dst.data[0].is_null()).is_err() {
        return;
    }
    if !src.r#ref.is_null() {
        if validate_input!(!src.data[0].is_null()).is_err() {
            return;
        }
        rav1d_ref_inc(src.r#ref);
    }
    *dst = src.clone();
}

pub(crate) unsafe fn rav1d_picture_move_ref(dst: &mut Rav1dPicture, src: &mut Rav1dPicture) {
    if validate_input!(dst.data[0].is_null()).is_err() {
        return;
    }
    if !src.r#ref.is_null() {
        if validate_input!(!src.data[0].is_null()).is_err() {
            return;
        }
    }
    *dst = mem::take(src);
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
    *dst = mem::take(&mut *src);
}

pub(crate) unsafe fn rav1d_picture_unref_internal(p: &mut Rav1dPicture) {
    let Rav1dPicture {
        m: _,
        data,
        mut r#ref,
        ..
    } = mem::take(p);
    if !r#ref.is_null() {
        if validate_input!(!data[0].is_null()).is_err() {
            return;
        }
        rav1d_ref_dec(&mut r#ref);
    }
}

pub(crate) unsafe fn rav1d_thread_picture_unref(p: *mut Rav1dThreadPicture) {
    rav1d_picture_unref_internal(&mut (*p).p);
    (*p).progress = 0 as *mut atomic_uint;
}
