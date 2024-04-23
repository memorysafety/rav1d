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
use crate::src::r#ref::Rav1dRef;
use libc::ptrdiff_t;
use libc::uintptr_t;
use std::ffi::c_int;
use std::ffi::c_void;
use std::ptr;
use std::ptr::NonNull;
use std::sync::Arc;
use std::sync::Mutex;

pub(crate) const RAV1D_PICTURE_ALIGNMENT: usize = 64;
pub const DAV1D_PICTURE_ALIGNMENT: usize = RAV1D_PICTURE_ALIGNMENT;

#[derive(Default)]
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

#[derive(Default)]
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
    pub n_itut_t35: usize,
    pub reserved: [uintptr_t; 3],
    pub frame_hdr_ref: Option<RawArc<DRav1d<Rav1dFrameHeader, Dav1dFrameHeader>>>, // opaque, so we can change this
    pub seq_hdr_ref: Option<RawArc<DRav1d<Rav1dSequenceHeader, Dav1dSequenceHeader>>>, // opaque, so we can change this
    pub content_light_ref: Option<RawArc<Rav1dContentLightLevel>>, // opaque, so we can change this
    pub mastering_display_ref: Option<RawArc<Rav1dMasteringDisplay>>, // opaque, so we can change this
    pub itut_t35_ref: Option<RawArc<Mutex<DRav1d<Vec<Rav1dITUTT35>, Vec<Dav1dITUTT35>>>>>, // opaque, so we can change this
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

// TODO(kkysen) Eventually the [`impl Default`] might not be needed.
// It's needed currently for a [`mem::take`] that simulates a move,
// but once everything is Rusty, we may not need to clear the `dst` anymore.
// This also applies to the `#[derive(Default)]`
// on [`Rav1dPictureParameters`] and [`Rav1dPixelLayout`].
#[derive(Clone, Default)]
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

    /// [`Option`] wasn't needed here since [`Vec`]`: `[`Default`],
    /// but this does necessitate an allocation of `Vec::default()` every time,
    /// and an [`Arc::clone`] on every clone,
    /// even though having a [`Rav1dITUTT35`] is fairly rare.
    /// This is a small cost compared to pixel data, however,
    /// so until we notice a performance impact, this simpler way should suffice.
    pub itut_t35: Arc<Mutex<DRav1d<Vec<Rav1dITUTT35>, Vec<Dav1dITUTT35>>>>,
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
            n_itut_t35: _,
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
            itut_t35: itut_t35_ref
                .map(|raw| unsafe { raw.into_arc() })
                .unwrap_or_default(),
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
        let (itut_t35_dav1d, n_itut_t35) = {
            let itut_t35 = &*itut_t35.try_lock().unwrap();
            let itut_t35_dav1d = Some(NonNull::new(itut_t35.dav1d.as_ptr().cast_mut()).unwrap());
            let n_itut_t35 = itut_t35.len();
            (itut_t35_dav1d, n_itut_t35)
        };

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
            itut_t35: itut_t35_dav1d,
            n_itut_t35,
            reserved: Default::default(),
            frame_hdr_ref: frame_hdr.map(RawArc::from_arc),
            seq_hdr_ref: seq_hdr.map(RawArc::from_arc),
            content_light_ref: content_light.map(RawArc::from_arc),
            mastering_display_ref: mastering_display.map(RawArc::from_arc),
            itut_t35_ref: Some(itut_t35).map(RawArc::from_arc),
            reserved_ref: Default::default(),
            r#ref,
            allocator_data,
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
    ///     The only fields of `pic` that will be already set are:
    ///     * [`Dav1dPicture::p`]
    ///     * [`Dav1dPicture::seq_hdr`]
    ///     * [`Dav1dPicture::frame_hdr`]
    ///     
    ///     This is not a change from the original `DAV1D_API`,
    ///     just a clarification of it.
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
    ///     
    ///     The only fields of `pic` that will be set are
    ///     the ones allocated by [`Self::alloc_picture_callback`]:
    ///     * [`Dav1dPicture::data`]
    ///     * [`Dav1dPicture::allocator_data`]
    ///     
    ///     NOTE: This is a slight change from the original `DAV1D_API`, which was underspecified.
    ///     However, all known uses of this API follow this already:
    ///     * `libdav1d`: [`dav1d_default_picture_release`](https://code.videolan.org/videolan/dav1d/-/blob/16ed8e8b99f2fcfffe016e929d3626e15267ad3e/src/picture.c#L85-87)
    ///     * `dav1d`: [`picture_release`](https://code.videolan.org/videolan/dav1d/-/blob/16ed8e8b99f2fcfffe016e929d3626e15267ad3e/tools/dav1d.c#L180-182)
    ///     * `dav1dplay`: [`placebo_release_pic`](https://code.videolan.org/videolan/dav1d/-/blob/16ed8e8b99f2fcfffe016e929d3626e15267ad3e/examples/dp_renderer_placebo.c#L375-383)
    ///     * `libplacebo`: [`pl_release_dav1dpicture`](https://github.com/haasn/libplacebo/blob/34e019bfedaa5a64f268d8f9263db352c0a8f67f/src/include/libplacebo/utils/dav1d_internal.h#L594-L607)
    ///     * `ffmpeg`: [`libdav1d_picture_release`](https://github.com/FFmpeg/FFmpeg/blob/00b288da73f45acb78b74bcc40f73c7ba1fff7cb/libavcodec/libdav1d.c#L124-L129)
    ///
    ///     Making this API safe without this slight tightening of the API
    ///     [is very difficult](https://github.com/memorysafety/rav1d/pull/685#discussion_r1458171639).
    ///
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
