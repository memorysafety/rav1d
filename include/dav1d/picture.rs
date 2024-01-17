use crate::include::common::validate::validate_input;
use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::dav1d::common::Rav1dDataProps;
use crate::include::dav1d::dav1d::Dav1dRef;
use crate::include::dav1d::headers::DRav1d;
use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::dav1d::headers::Dav1dITUTT35;
use crate::include::dav1d::headers::Dav1dPixelLayout;
use crate::include::dav1d::headers::Dav1dSequenceHeader;
use crate::include::dav1d::headers::Rav1dContentLightLevel;
use crate::include::dav1d::headers::Rav1dFrameHeader;
use crate::include::dav1d::headers::Rav1dITUTT35;
use crate::include::dav1d::headers::Rav1dMasteringDisplay;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::include::dav1d::headers::Rav1dSequenceHeader;
use crate::src::c_arc::RawArc;
use crate::src::error::Dav1dResult;
use crate::src::error::Rav1dError;
use crate::src::error::Rav1dError::EINVAL;
use crate::src::error::Rav1dResult;
use crate::src::r#ref::Rav1dRef;
use libc::ptrdiff_t;
use libc::uintptr_t;
use std::ffi::c_int;
use std::ffi::c_void;
use std::ptr;
use std::ptr::NonNull;
use std::sync::Arc;

pub(crate) const RAV1D_PICTURE_ALIGNMENT: usize = 64;
pub const DAV1D_PICTURE_ALIGNMENT: usize = RAV1D_PICTURE_ALIGNMENT;

#[repr(C)]
pub struct Dav1dPictureParameters {
    pub w: c_int,
    pub h: c_int,
    pub layout: Dav1dPixelLayout,
    pub bpc: c_int,
}

// TODO(kkysen) Eventually the [`impl Default`] might not be needed.
#[derive(Clone, Default)]
#[repr(C)]
pub(crate) struct Rav1dPictureParameters {
    pub w: c_int,
    pub h: c_int,
    pub layout: Rav1dPixelLayout,
    pub bpc: c_int,
}

impl From<Dav1dPictureParameters> for Rav1dPictureParameters {
    fn from(value: Dav1dPictureParameters) -> Self {
        let Dav1dPictureParameters { w, h, layout, bpc } = value;
        Self {
            w,
            h,
            layout: layout.try_into().unwrap(),
            bpc,
        }
    }
}

impl From<Rav1dPictureParameters> for Dav1dPictureParameters {
    fn from(value: Rav1dPictureParameters) -> Self {
        let Rav1dPictureParameters { w, h, layout, bpc } = value;
        Self {
            w,
            h,
            layout: layout.into(),
            bpc,
        }
    }
}

#[repr(C)]
pub struct Dav1dPicture {
    pub seq_hdr: Option<NonNull<Dav1dSequenceHeader>>,
    pub frame_hdr: Option<NonNull<Dav1dFrameHeader>>,
    pub data: [Option<NonNull<c_void>>; 3],
    pub stride: [ptrdiff_t; 2],
    pub p: Dav1dPictureParameters,
    pub m: Dav1dDataProps,
    pub content_light: Option<NonNull<Rav1dContentLightLevel>>,
    pub mastering_display: Option<NonNull<Rav1dMasteringDisplay>>,
    pub itut_t35: Option<NonNull<Dav1dITUTT35>>,
    pub reserved: [uintptr_t; 4],
    pub frame_hdr_ref: Option<RawArc<DRav1d<Rav1dFrameHeader, Dav1dFrameHeader>>>, // opaque, so we can change this
    pub seq_hdr_ref: Option<RawArc<DRav1d<Rav1dSequenceHeader, Dav1dSequenceHeader>>>, // opaque, so we can change this
    pub content_light_ref: Option<RawArc<Rav1dContentLightLevel>>, // opaque, so we can change this
    pub mastering_display_ref: Option<RawArc<Rav1dMasteringDisplay>>, // opaque, so we can change this
    pub itut_t35_ref: Option<RawArc<DRav1d<Rav1dITUTT35, Dav1dITUTT35>>>, // opaque, so we can change this
    pub reserved_ref: [uintptr_t; 4],
    pub r#ref: Option<NonNull<Dav1dRef>>,
    pub allocator_data: Option<NonNull<c_void>>,
}

#[derive(Clone)]
pub(crate) struct Rav1dPictureData {
    pub data: [*mut c_void; 3],
    pub allocator_data: Option<NonNull<c_void>>,
}

impl Default for Rav1dPictureData {
    fn default() -> Self {
        Self {
            data: [ptr::null_mut(); 3],
            allocator_data: Default::default(),
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dPicture {
    pub seq_hdr: Option<Arc<DRav1d<Rav1dSequenceHeader, Dav1dSequenceHeader>>>,
    pub frame_hdr: Option<Arc<DRav1d<Rav1dFrameHeader, Dav1dFrameHeader>>>,
    pub data: Rav1dPictureData,
    pub stride: [ptrdiff_t; 2],
    pub p: Rav1dPictureParameters,
    pub m: Rav1dDataProps,
    pub content_light: Option<Arc<Rav1dContentLightLevel>>,
    pub mastering_display: Option<Arc<Rav1dMasteringDisplay>>,
    pub itut_t35: Option<Arc<DRav1d<Rav1dITUTT35, Dav1dITUTT35>>>,
    pub r#ref: Option<NonNull<Rav1dRef>>,
}

impl From<Dav1dPicture> for Rav1dPicture {
    fn from(value: Dav1dPicture) -> Self {
        let Dav1dPicture {
            seq_hdr: _,
            frame_hdr: _,
            data,
            stride,
            p,
            m,
            content_light: _,
            mastering_display: _,
            itut_t35: _,
            reserved: _,
            frame_hdr_ref,
            seq_hdr_ref,
            content_light_ref,
            mastering_display_ref,
            itut_t35_ref,
            reserved_ref: _,
            r#ref,
            allocator_data,
        } = value;
        Self {
            // We don't `.update_rav1d()` [`Rav1dSequenceHeader`] because it's meant to be read-only.
            // Safety: `raw` came from [`RawArc::from_arc`].
            seq_hdr: seq_hdr_ref.map(|raw| unsafe { raw.into_arc() }),
            // We don't `.update_rav1d()` [`Rav1dFrameHeader`] because it's meant to be read-only.
            // Safety: `raw` came from [`RawArc::from_arc`].
            frame_hdr: frame_hdr_ref.map(|raw| unsafe { raw.into_arc() }),
            data: Rav1dPictureData {
                data: data.map(|data| data.map_or_else(ptr::null_mut, NonNull::as_ptr)),
                allocator_data,
            },
            stride,
            p: p.into(),
            m: m.into(),
            // Safety: `raw` came from [`RawArc::from_arc`].
            content_light: content_light_ref.map(|raw| unsafe { raw.into_arc() }),
            // Safety: `raw` came from [`RawArc::from_arc`].
            mastering_display: mastering_display_ref.map(|raw| unsafe { raw.into_arc() }),
            // We don't `.update_rav1d` [`Rav1dITUTT35`] because never read it.
            // Safety: `raw` came from [`RawArc::from_arc`].
            itut_t35: itut_t35_ref.map(|raw| unsafe { raw.into_arc() }),
            r#ref,
        }
    }
}

impl From<Rav1dPicture> for Dav1dPicture {
    fn from(value: Rav1dPicture) -> Self {
        let Rav1dPicture {
            seq_hdr,
            frame_hdr,
            data:
                Rav1dPictureData {
                    data,
                    allocator_data,
                },
            stride,
            p,
            m,
            content_light,
            mastering_display,
            itut_t35,
            r#ref,
        } = value;
        Self {
            // [`DRav1d::from_rav1d`] is called right after [`parse_seq_hdr`].
            seq_hdr: seq_hdr.as_ref().map(|arc| (&arc.as_ref().dav1d).into()),
            // [`DRav1d::from_rav1d`] is called in [`parse_frame_hdr`].
            frame_hdr: frame_hdr.as_ref().map(|arc| (&arc.as_ref().dav1d).into()),
            data: data.map(NonNull::new),
            stride,
            p: p.into(),
            m: m.into(),
            content_light: content_light.as_ref().map(|arc| arc.as_ref().into()),
            mastering_display: mastering_display.as_ref().map(|arc| arc.as_ref().into()),
            // [`DRav1d::from_rav1d`] is called in [`rav1d_parse_obus`].
            itut_t35: itut_t35.as_ref().map(|arc| (&arc.as_ref().dav1d).into()),
            reserved: Default::default(),
            frame_hdr_ref: frame_hdr.map(RawArc::from_arc),
            seq_hdr_ref: seq_hdr.map(RawArc::from_arc),
            content_light_ref: content_light.map(RawArc::from_arc),
            mastering_display_ref: mastering_display.map(RawArc::from_arc),
            itut_t35_ref: itut_t35.map(RawArc::from_arc),
            reserved_ref: Default::default(),
            r#ref,
            allocator_data,
        }
    }
}

// TODO(kkysen) Eventually the [`impl Default`] might not be needed.
// It's needed currently for a [`mem::take`] that simulates a move,
// but once everything is Rusty, we may not need to clear the `dst` anymore.
// This also applies to the `#[derive(Default)]`
// on [`Rav1dPictureParameters`] and [`Rav1dPixelLayout`].
impl Default for Rav1dPicture {
    fn default() -> Self {
        Self {
            seq_hdr: None,
            frame_hdr: None,
            data: Default::default(),
            stride: Default::default(),
            p: Default::default(),
            m: Default::default(),
            content_light: None,
            mastering_display: None,
            itut_t35: None,
            r#ref: None,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dPicAllocator {
    /// Custom data to pass to the allocator callbacks.
    pub cookie: *mut c_void,

    /// Allocate the picture buffer based on the [`Dav1dPictureParameters`].
    ///
    /// [`data`]`[0]`, [`data`]`[1]` and [`data`]`[2]`
    /// must be [`DAV1D_PICTURE_ALIGNMENT`]-byte aligned
    /// and with a pixel width/height multiple of 128 pixels.
    /// Any allocated memory area should also be padded by [`DAV1D_PICTURE_ALIGNMENT`] bytes.
    /// [`data`]`[1]` and [`data`]`[2]` must share the same [`stride`]`[1]`.
    ///
    /// # Safety
    ///
    /// If frame threading is used, accesses to [`Self::cookie`] must be thread-safe.
    ///
    /// # Args
    ///
    /// * `pic`: The picture to allocate the buffer for.
    ///     The callback needs to fill the picture
    ///     [`data`]`[0]`, [`data`]`[1]`, [`data`]`[2]`,
    ///     [`stride`]`[0]`, and [`stride`]`[1]`.
    ///     The allocator can fill the pic [`allocator_data`] pointer
    ///     with a custom pointer that will be passed to
    ///     [`release_picture_callback`].
    ///
    /// * `cookie`: Custom pointer passed to all calls.
    ///
    /// *Note*: No fields other than [`data`], [`stride`] and [`allocator_data`]
    /// must be filled by this callback.
    ///
    /// # Return
    ///
    /// 0 on success. A negative `DAV1D_ERR` value on error.
    /// <!--- TODO(kkysen) Translate `DAV1D_ERR` -->
    ///
    /// [`data`]: Dav1dPicture::data
    /// [`stride`]: Dav1dPicture::data
    /// [`allocator_data`]: Dav1dPicture::allocator_data
    /// [`release_picture_callback`]: Self::release_picture_callback
    pub alloc_picture_callback:
        Option<unsafe extern "C" fn(pic: *mut Dav1dPicture, cookie: *mut c_void) -> Dav1dResult>,

    /// Release the picture buffer.
    ///
    /// # Safety
    ///
    /// If frame threading is used, accesses to `cookie` must be thread-safe.
    ///
    /// If frame threading is used, this function may be called by the main thread
    /// (the thread which calls [`dav1d_get_picture`]),
    /// or any of the frame threads and thus must be thread-safe.
    /// If frame threading is not used, this function will only be called on the main thread.
    ///
    /// # Args
    ///
    /// * `pic`: The picture that was filled by [`alloc_picture_callback`].
    /// * `cookie`: Custom pointer passed to all calls.
    ///
    /// [`dav1d_get_picture`]: crate::src::lib::dav1d_get_picture
    /// [`alloc_picture_callback`]: Self::alloc_picture_callback
    pub release_picture_callback:
        Option<unsafe extern "C" fn(pic: *mut Dav1dPicture, cookie: *mut c_void) -> ()>,
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dPicAllocator {
    /// See [`Dav1dPicAllocator::cookie`].
    pub cookie: *mut c_void,

    /// See [`Dav1dPicAllocator::alloc_picture_callback`].
    ///
    /// # Safety
    ///
    /// If frame threading is used, accesses to [`Self::cookie`] must be thread-safe,
    /// i.e. [`Self::cookie`] must be [`Send`]` + `[`Sync`].
    pub alloc_picture_callback:
        unsafe extern "C" fn(pic: *mut Dav1dPicture, cookie: *mut c_void) -> Dav1dResult,

    /// See [`Dav1dPicAllocator::release_picture_callback`].
    ///
    /// # Safety
    ///
    /// If frame threading is used, accesses to [`Self::cookie`] must be thread-safe,
    /// i.e. [`Self::cookie`] must be [`Send`]` + `[`Sync`].
    pub release_picture_callback:
        unsafe extern "C" fn(pic: *mut Dav1dPicture, cookie: *mut c_void) -> (),
}

impl TryFrom<Dav1dPicAllocator> for Rav1dPicAllocator {
    type Error = Rav1dError;

    fn try_from(value: Dav1dPicAllocator) -> Result<Self, Self::Error> {
        let Dav1dPicAllocator {
            cookie,
            alloc_picture_callback,
            release_picture_callback,
        } = value;
        Ok(Self {
            cookie,
            alloc_picture_callback: validate_input!(alloc_picture_callback.ok_or(EINVAL))?,
            release_picture_callback: validate_input!(release_picture_callback.ok_or(EINVAL))?,
        })
    }
}

impl From<Rav1dPicAllocator> for Dav1dPicAllocator {
    fn from(value: Rav1dPicAllocator) -> Self {
        let Rav1dPicAllocator {
            cookie,
            alloc_picture_callback,
            release_picture_callback,
        } = value;
        Self {
            cookie,
            alloc_picture_callback: Some(alloc_picture_callback),
            release_picture_callback: Some(release_picture_callback),
        }
    }
}

impl Rav1dPicAllocator {
    pub unsafe fn alloc_picture(&self, p: *mut Rav1dPicture) -> Rav1dResult {
        let mut p_c = p.read().into();
        let result = (self.alloc_picture_callback)(&mut p_c, self.cookie);
        p.write(p_c.into());
        result.try_into().unwrap()
    }

    pub unsafe fn release_picture(&self, p: *mut Rav1dPicture) {
        let mut p_c = p.read().into();
        (self.release_picture_callback)(&mut p_c, self.cookie);
        p.write(p_c.into());
    }
}
