#![deny(unsafe_op_in_unsafe_fn)]

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
use crate::include::dav1d::picture::Rav1dPictureData;
use crate::include::dav1d::picture::Rav1dPictureParameters;
use crate::include::dav1d::picture::RAV1D_PICTURE_ALIGNMENT;
use crate::src::error::Dav1dResult;
use crate::src::error::Rav1dError::EGeneric;
use crate::src::error::Rav1dResult;
use crate::src::internal::Rav1dContext;
use crate::src::internal::Rav1dFrameData;
use crate::src::log::Rav1dLog as _;
use crate::src::log::Rav1dLogger;
use crate::src::mem::MemPool;
use atomig::Atom;
use atomig::AtomLogic;
use bitflags::bitflags;
use libc::ptrdiff_t;
use std::ffi::c_int;
use std::ffi::c_void;
use std::mem;
use std::mem::MaybeUninit;
use std::ptr;
use std::ptr::NonNull;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use to_method::To as _;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Atom, AtomLogic)]
pub struct PictureFlags(u8);

bitflags! {
    impl PictureFlags: u8 {
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

#[derive(Clone, Default)]
#[repr(C)]
pub(crate) struct Rav1dThreadPicture {
    pub p: Rav1dPicture,
    pub visible: bool,
    /// This can be set for inter frames, non-key intra frames,
    /// or for invisible keyframes that have not yet been made visible
    /// using the show-existing-frame mechanism.
    pub showable: bool,
    pub flags: PictureFlags,
    /// `[0]`: block data (including segmentation map and motion vectors)
    /// `[1]`: pixel data
    pub progress: Option<Arc<[AtomicU32; 2]>>,
}

struct MemPoolBuf<T> {
    /// This is an [`Arc`] because a [`Rav1dPicture`] can outlive
    /// its [`Rav1dContext`], and that's how the API was designed.
    /// If it were changed to require [`Rav1dContext`] to outlive
    /// any [`Rav1dPicture`]s it creates (a reasonable API I think,
    /// and easy to do with Rust lifetimes), then we wouldn't need
    /// the [`Arc`] here.  But it's not the round-tripping through C
    /// that requires this, just how the API was designed.
    pool: Arc<MemPool<T>>,
    buf: Vec<T>,
}

impl Rav1dPictureParameters {
    pub fn pic_len(&self, [y_stride, uv_stride]: [isize; 2]) -> [usize; 2] {
        let ss_ver = (self.layout == Rav1dPixelLayout::I420) as u8;
        let aligned_h = self.h as usize + 127 & !127;
        let y_sz = y_stride.unsigned_abs() * aligned_h;
        let uv_sz = uv_stride.unsigned_abs() * (aligned_h >> ss_ver);
        [y_sz, uv_sz]
    }
}

/// # Safety
///
/// * `p_c` must be from a `&mut Dav1dPicture`.
/// * `cookie` must be from a `&Arc<MemPool<MaybeUninit<u8>>>`.
unsafe extern "C" fn dav1d_default_picture_alloc(
    p_c: *mut Dav1dPicture,
    cookie: *mut c_void,
) -> Dav1dResult {
    // SAFETY: Guaranteed by safety preconditions.
    let p = unsafe { p_c.read() }.to::<Rav1dPicture>();
    let hbd = (p.p.bpc > 8) as c_int;
    let aligned_w = p.p.w + 127 & !127;
    let has_chroma = p.p.layout != Rav1dPixelLayout::I400;
    let ss_hor = (p.p.layout != Rav1dPixelLayout::I444) as c_int;
    let mut y_stride = (aligned_w << hbd) as ptrdiff_t;
    let mut uv_stride = if has_chroma { y_stride >> ss_hor } else { 0 };
    if y_stride & 1023 == 0 {
        y_stride += RAV1D_PICTURE_ALIGNMENT as isize;
    }
    if uv_stride & 1023 == 0 && has_chroma {
        uv_stride += RAV1D_PICTURE_ALIGNMENT as isize;
    }
    let stride = [y_stride, uv_stride];
    let [y_sz, uv_sz] = p.p.pic_len(stride);
    let pic_size = y_sz + 2 * uv_sz;

    // SAFETY: Guaranteed by safety preconditions.
    let pool = unsafe { &*cookie.cast::<Arc<MemPool<MaybeUninit<u8>>>>() };
    let pool = pool.clone();
    let pic_cap = pic_size + RAV1D_PICTURE_ALIGNMENT;
    // TODO fallible allocation
    let mut buf = pool.pop(pic_cap);
    buf.resize_with(pic_cap, MaybeUninit::uninit);
    // We have to `Box` this because `Dav1dPicture::allocator_data` is only 8 bytes.
    let mut buf = Box::new(MemPoolBuf { pool, buf });
    let data = &mut buf.buf[..];
    // SAFETY: `Rav1dPicAllocator::alloc_picture_callback` requires that these are `RAV1D_PICTURE_ALIGNMENT`-aligned.
    let align_offset = data.as_ptr().align_offset(RAV1D_PICTURE_ALIGNMENT);
    let data = &mut data[align_offset..][..pic_size];

    let (data0, data12) = data.split_at_mut(y_sz);
    let (data1, data2) = data12.split_at_mut(uv_sz);
    // Note that `data[1]` and `data[2]`
    // were previously null instead of an empty slice when `!has_chroma`,
    // but this way is simpler and more uniform, especially when we move to slices.
    let data = [data0, data1, data2].map(|data| data.as_mut_ptr().cast());

    // SAFETY: Guaranteed by safety preconditions.
    let p_c = unsafe { &mut *p_c };
    p_c.stride = stride;
    p_c.data = data.map(NonNull::new);
    p_c.allocator_data = NonNull::new(Box::into_raw(buf).cast::<c_void>());
    // The caller will create the real `Rav1dPicture` from the `Dav1dPicture` fields set above,
    // so we don't want to drop the `Rav1dPicture` we created for convenience here.
    mem::forget(p);

    Rav1dResult::Ok(()).into()
}

/// # Safety
///
/// * `p` is from a `&mut Dav1dPicture` initialized by [`dav1d_default_picture_alloc`].
unsafe extern "C" fn dav1d_default_picture_release(p: *mut Dav1dPicture, _cookie: *mut c_void) {
    // SAFETY: Guaranteed by safety preconditions.
    let p = unsafe { &mut *p };
    let buf = p.allocator_data.unwrap().as_ptr();
    // SAFETY: `dav1d_default_picture_alloc` stores `Box::into_raw` of a `MemPoolBuf<MaybeUninit<u8>>` in `Dav1dPicture::allocator_data`,
    // and `(Rav1dPicAllocator::release_picture_callback == dav1d_default_picture_release) == (Rav1dPicAllocator::alloc_picture_callback == dav1d_default_picture_alloc)`.
    let buf = unsafe { Box::from_raw(buf.cast::<MemPoolBuf<MaybeUninit<u8>>>()) };
    let MemPoolBuf { pool, buf } = *buf;
    pool.push(buf);
}

impl Default for Rav1dPicAllocator {
    fn default() -> Self {
        Self {
            cookie: ptr::null_mut(),
            // SAFETY: `dav1d_default_picture_alloc` requires `p_c` be from a `&mut Dav1dPicture`,
            // `Self::alloc_picture_callback` safety preconditions guarantee that.
            // `dav1d_default_picture_alloc` also requires that `cookie` be from a `&Arc<MemPool<MaybeUninit<u8>>>`,
            // which is set if `Self::is_default()` in `rav1d_open`.
            alloc_picture_callback: dav1d_default_picture_alloc,
            // SAFETY: `dav1d_default_picture_release` requires `p` be from a `&mut Dav1dPicture`
            // initialized by `dav1d_default_picture_alloc`.
            // Since these `fn`s are private and we only use them here, that is guaranteed.
            release_picture_callback: dav1d_default_picture_release,
        }
    }
}

impl Rav1dPicAllocator {
    pub fn is_default(&self) -> bool {
        let alloc = self.alloc_picture_callback == dav1d_default_picture_alloc;
        let release = self.release_picture_callback == dav1d_default_picture_release;
        assert!(alloc == release); // This should be impossible since these `fn`s are private.
        alloc && release
    }
}

impl Rav1dPicAllocator {
    pub fn alloc_picture_data(
        &self,
        w: c_int,
        h: c_int,
        seq_hdr: Arc<DRav1d<Rav1dSequenceHeader, Dav1dSequenceHeader>>,
        frame_hdr: Option<Arc<DRav1d<Rav1dFrameHeader, Dav1dFrameHeader>>>,
    ) -> Rav1dResult<Rav1dPicture> {
        let pic = Rav1dPicture {
            p: Rav1dPictureParameters {
                w,
                h,
                layout: seq_hdr.layout,
                bpc: 8 + 2 * seq_hdr.hbd,
            },
            seq_hdr: Some(seq_hdr),
            frame_hdr,
            ..Default::default()
        };
        let mut pic_c = pic.to::<Dav1dPicture>();
        // Safety: `pic_c` is a valid `Dav1dPicture` with `data`, `stride`, `allocator_data` unset.
        let result = unsafe { (self.alloc_picture_callback)(&mut pic_c, self.cookie) };
        result.try_to::<Rav1dResult>().unwrap()?;
        // `data`, `stride`, and `allocator_data` are the only fields set by the allocator.
        // Of those, only `data` and `allocator_data` are read through `r#ref`,
        // so we need to read those directly first and allocate the `Arc`.
        let data = pic_c.data;
        let allocator_data = pic_c.allocator_data;
        let mut pic = pic_c.to::<Rav1dPicture>();
        // TODO fallible allocation
        pic.data = Some(Arc::new(Rav1dPictureData {
            data: data.map(|data| data.unwrap().as_ptr()),
            allocator_data,
            allocator: self.clone(),
        }));
        Ok(pic)
    }

    pub fn dealloc_picture_data(
        &self,
        data: [*mut c_void; 3],
        allocator_data: Option<NonNull<c_void>>,
    ) {
        let data = data.map(NonNull::new);
        let mut pic_c = Dav1dPicture {
            data,
            allocator_data,
            ..Default::default()
        };
        // Safety: `pic_c` contains the same `data` and `allocator_data`
        // that `Self::alloc_picture_data` set, which now get deallocated here.
        unsafe {
            (self.release_picture_callback)(&mut pic_c, self.cookie);
        }
    }
}

fn picture_alloc_with_edges(
    logger: &Option<Rav1dLogger>,
    p: &mut Rav1dPicture,
    w: c_int,
    h: c_int,
    seq_hdr: Option<Arc<DRav1d<Rav1dSequenceHeader, Dav1dSequenceHeader>>>,
    frame_hdr: Option<Arc<DRav1d<Rav1dFrameHeader, Dav1dFrameHeader>>>,
    bpc: u8,
    p_allocator: &Rav1dPicAllocator,
) -> Rav1dResult {
    if p.data.is_some() {
        writeln!(logger, "Picture already allocated!",);
        return Err(EGeneric);
    }
    assert!(bpc > 0 && bpc <= 16);
    let pic = p_allocator.alloc_picture_data(w, h, seq_hdr.unwrap(), frame_hdr)?;
    *p = pic;

    Ok(())
}

pub fn rav1d_picture_copy_props(
    p: &mut Rav1dPicture,
    content_light: Option<Arc<Rav1dContentLightLevel>>,
    mastering_display: Option<Arc<Rav1dMasteringDisplay>>,
    itut_t35: Arc<DRav1d<Box<[Rav1dITUTT35]>, Box<[Dav1dITUTT35]>>>,
    props: Rav1dDataProps,
) {
    p.m = props;
    p.content_light = content_light;
    p.mastering_display = mastering_display;
    p.itut_t35 = itut_t35;
}

// itut_t35 was taken out of the c.itut_t35 originally, but that violates Rust
// borrowing rules so we need to pass it to this function explicitly.
pub(crate) fn rav1d_thread_picture_alloc(
    c: &Rav1dContext,
    f: &mut Rav1dFrameData,
    bpc: u8,
    itut_t35: Arc<Mutex<Vec<Rav1dITUTT35>>>,
) -> Rav1dResult {
    let p = &mut f.sr_cur;
    let have_frame_mt = c.fc.len() > 1;
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    picture_alloc_with_edges(
        &c.logger,
        &mut p.p,
        frame_hdr.size.width[1],
        frame_hdr.size.height,
        f.seq_hdr.clone(),
        f.frame_hdr.clone(),
        bpc,
        &c.allocator,
    )?;

    rav1d_picture_copy_props(
        &mut p.p,
        c.content_light.clone(),
        c.mastering_display.clone(),
        Rav1dITUTT35::to_immut(itut_t35),
        f.tiles[0].data.m.clone(),
    );

    let flags_mask = if frame_hdr.show_frame != 0 || c.output_invisible_frames {
        PictureFlags::empty()
    } else {
        PictureFlags::NEW_SEQUENCE | PictureFlags::NEW_OP_PARAMS_INFO
    };
    p.flags = c.frame_flags.fetch_and(flags_mask, Ordering::Relaxed);
    p.visible = frame_hdr.show_frame != 0;
    p.showable = frame_hdr.showable_frame != 0;
    p.progress = if have_frame_mt {
        Some(Default::default())
    } else {
        None
    };
    Ok(())
}

pub(crate) fn rav1d_picture_alloc_copy(
    c: &Rav1dContext,
    dst: &mut Rav1dPicture,
    w: c_int,
    src: &Rav1dPicture,
) -> Rav1dResult {
    picture_alloc_with_edges(
        &c.logger,
        dst,
        w,
        src.p.h,
        src.seq_hdr.clone(),
        src.frame_hdr.clone(),
        src.p.bpc,
        &src.data.as_ref().unwrap().allocator,
    )?;

    rav1d_picture_copy_props(
        dst,
        src.content_light.clone(),
        src.mastering_display.clone(),
        src.itut_t35.clone(),
        src.m.clone(),
    );
    Ok(())
}
