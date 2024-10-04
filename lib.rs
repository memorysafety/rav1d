#![allow(non_upper_case_globals)]
#![cfg_attr(target_arch = "arm", feature(stdarch_arm_feature_detection))]
#![cfg_attr(
    any(target_arch = "riscv32", target_arch = "riscv64"),
    feature(stdarch_riscv_feature_detection)
)]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(clippy::all)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(clippy::missing_safety_doc)]

#[cfg(not(any(feature = "bitdepth_8", feature = "bitdepth_16")))]
compile_error!("No bitdepths enabled. Enable one or more of the following features: `bitdepth_8`, `bitdepth_16`");

pub mod include {
    pub mod common {
        pub(crate) mod attributes;
        pub(crate) mod bitdepth;
        pub(crate) mod dump;
        pub(crate) mod intops;
        pub(crate) mod validate;
    } // mod common
    pub mod dav1d {
        pub mod common;
        pub mod data;
        pub mod dav1d;
        pub mod headers;
        pub mod picture;
    } // mod dav1d
} // mod include
pub mod src {
    pub mod align;
    pub(crate) mod assume;
    pub(crate) mod c_arc;
    pub(crate) mod c_box;
    mod cdef;
    mod cdef_apply;
    mod cdf;
    mod const_fn;
    pub mod cpu;
    mod ctx;
    mod cursor;
    mod data;
    mod decode;
    mod dequant_tables;
    pub(crate) mod disjoint_mut;
    pub(crate) mod enum_map;
    mod env;
    pub(crate) mod error;
    mod ffi_safe;
    mod fg_apply;
    mod filmgrain;
    mod getbits;
    pub(crate) mod pic_or_buf;
    pub(crate) mod pixels;
    pub(crate) mod relaxed_atomic;
    pub mod send_sync_non_null;
    pub(crate) mod strided;
    pub(crate) mod with_offset;
    pub(crate) mod wrap_fn_ptr;
    // TODO(kkysen) Temporarily `pub(crate)` due to a `pub use` until TAIT.
    mod extensions;
    mod in_range;
    pub(super) mod internal;
    mod intra_edge;
    mod ipred;
    mod ipred_prepare;
    mod iter;
    mod itx;
    mod itx_1d;
    pub(crate) mod levels;
    mod lf_apply;
    mod lf_mask;
    pub mod lib;
    pub(crate) mod log;
    mod loopfilter;
    mod looprestoration;
    mod lr_apply;
    mod mc;
    mod mem;
    mod msac;
    mod obu;
    mod pal;
    mod picture;
    mod qm;
    mod recon;
    mod refmvs;
    mod scan;
    mod tables;
    mod thread_task;
    mod warpmv;
    mod wedge;
} // mod src

pub use src::error::Dav1dResult;

// ---------------------------------------------------------------------------------------

/// Public Rust API.
///
/// This is more or less the same API as <https://crates.io/crates/dav1d>,
/// and is indeed a fork of that work.
pub mod dav1d {
    // This whole module was originally copied from https://github.com/rust-av/dav1d-rs/
    // (specifically https://github.com/rust-av/dav1d-rs/blob/94b1deaa1e25bf29c77bb5cc8a08ddaf7663eede/src/lib.rs)
    // with some modifications.
    // `dav1d-rs` is under the MIT license, replicated here:

    // MIT License
    //
    // Copyright (c) 2018 Luca Barbato
    //
    // Permission is hereby granted, free of charge, to any person obtaining a copy
    // of this software and associated documentation files (the "Software"), to deal
    // in the Software without restriction, including without limitation the rights
    // to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    // copies of the Software, and to permit persons to whom the Software is
    // furnished to do so, subject to the following conditions:
    //
    // The above copyright notice and this permission notice shall be included in all
    // copies or substantial portions of the Software.
    //
    // THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    // IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    // FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    // AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    // LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    // OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    // SOFTWARE.

    // The code below provides a safe API around the rav1d C FFI layer.

    use crate as rav1d;

    pub use av_data::pixel;
    use std::ffi::c_void;
    use std::fmt;
    use std::mem;
    use std::ptr::NonNull;
    use std::sync::Arc;

    use rav1d::include::dav1d::data::*;
    use rav1d::include::dav1d::dav1d::*;
    use rav1d::include::dav1d::headers::*;
    use rav1d::include::dav1d::picture::*;
    use rav1d::src::error::{Rav1dError, Rav1dResult};
    use rav1d::src::lib::*;
    use rav1d::src::send_sync_non_null::SendSyncNonNull;
    use rav1d::Dav1dResult;

    fn option_nonnull<T>(ptr: *mut T) -> Option<NonNull<T>> {
        if ptr.is_null() {
            None
        } else {
            Some(NonNull::new(ptr).unwrap())
        }
    }

    fn option_send_sync_non_null<T: Send + Sync>(r#box: Box<T>) -> Option<SendSyncNonNull<T>> {
        Some(SendSyncNonNull::from_box(r#box))
    }

    fn rav1d_result(ret: Dav1dResult) -> Rav1dResult {
        Rav1dResult::try_from(ret).unwrap()
    }

    /// Error enum return by various `dav1d` operations.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[non_exhaustive]
    pub enum Error {
        /// Try again.
        ///
        /// If this is returned by [`Decoder::send_data`] or [`Decoder::send_pending_data`] then there
        /// are decoded frames pending that first have to be retrieved via [`Decoder::get_picture`]
        /// before processing any further pending data.
        ///
        /// If this is returned by [`Decoder::get_picture`] then no decoded frames are pending
        /// currently and more data needs to be sent to the decoder.
        Again,
        /// Invalid argument.
        ///
        /// One of the arguments passed to the function was invalid.
        InvalidArgument,
        /// Not enough memory.
        ///
        /// Not enough memory is currently available for performing this operation.
        NotEnoughMemory,
        /// Unsupported bitstream.
        ///
        /// The provided bitstream is not supported by `dav1d`.
        UnsupportedBitstream,
        /// Unknown error.
        UnknownError(Rav1dError),
    }

    impl From<Rav1dError> for Error {
        fn from(err: Rav1dError) -> Self {
            match err {
                Rav1dError::EAGAIN => Error::Again,
                Rav1dError::ENOMEM => Error::NotEnoughMemory,
                Rav1dError::EINVAL => Error::InvalidArgument,
                Rav1dError::ENOPROTOOPT => Error::UnsupportedBitstream,
                _ => Error::UnknownError(err),
            }
        }
    }

    impl Error {
        pub const fn is_again(&self) -> bool {
            matches!(self, Error::Again)
        }
    }

    impl fmt::Display for Error {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Error::Again => write!(fmt, "Try again"),
                Error::InvalidArgument => write!(fmt, "Invalid argument"),
                Error::NotEnoughMemory => write!(fmt, "Not enough memory available"),
                Error::UnsupportedBitstream => write!(fmt, "Unsupported bitstream"),
                Error::UnknownError(err) => write!(fmt, "Unknown error {err:?}"),
            }
        }
    }

    impl std::error::Error for Error {}

    /// Settings for creating a new [`Decoder`] instance.
    /// See documentation for native `Dav1dSettings` struct.
    pub struct Settings {
        dav1d_settings: Dav1dSettings,
    }

    unsafe impl Send for Settings {}
    unsafe impl Sync for Settings {}

    impl Default for Settings {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Settings {
        /// Creates a new [`Settings`] instance with default settings.
        pub fn new() -> Self {
            unsafe {
                let mut dav1d_settings = mem::MaybeUninit::uninit();

                dav1d_default_settings(NonNull::new(dav1d_settings.as_mut_ptr()).unwrap());

                Self {
                    dav1d_settings: dav1d_settings.assume_init(),
                }
            }
        }

        pub fn set_n_threads(&mut self, n_threads: u32) {
            self.dav1d_settings.n_threads = n_threads as i32;
        }

        pub fn get_n_threads(&self) -> u32 {
            self.dav1d_settings.n_threads as u32
        }

        pub fn set_max_frame_delay(&mut self, max_frame_delay: u32) {
            self.dav1d_settings.max_frame_delay = max_frame_delay as i32;
        }

        pub fn get_max_frame_delay(&self) -> u32 {
            self.dav1d_settings.max_frame_delay as u32
        }

        pub fn set_apply_grain(&mut self, apply_grain: bool) {
            self.dav1d_settings.apply_grain = i32::from(apply_grain);
        }

        pub fn get_apply_grain(&self) -> bool {
            self.dav1d_settings.apply_grain != 0
        }

        pub fn set_operating_point(&mut self, operating_point: u32) {
            self.dav1d_settings.operating_point = operating_point as i32;
        }

        pub fn get_operating_point(&self) -> u32 {
            self.dav1d_settings.operating_point as u32
        }

        pub fn set_all_layers(&mut self, all_layers: bool) {
            self.dav1d_settings.all_layers = i32::from(all_layers);
        }

        pub fn get_all_layers(&self) -> bool {
            self.dav1d_settings.all_layers != 0
        }

        pub fn set_frame_size_limit(&mut self, frame_size_limit: u32) {
            self.dav1d_settings.frame_size_limit = frame_size_limit;
        }

        pub fn get_frame_size_limit(&self) -> u32 {
            self.dav1d_settings.frame_size_limit
        }

        pub fn set_strict_std_compliance(&mut self, strict_std_compliance: bool) {
            self.dav1d_settings.strict_std_compliance = i32::from(strict_std_compliance);
        }

        pub fn get_strict_std_compliance(&self) -> bool {
            self.dav1d_settings.strict_std_compliance != 0
        }

        pub fn set_output_invisible_frames(&mut self, output_invisible_frames: bool) {
            self.dav1d_settings.output_invisible_frames = i32::from(output_invisible_frames);
        }

        pub fn get_output_invisible_frames(&self) -> bool {
            self.dav1d_settings.output_invisible_frames != 0
        }

        pub fn set_inloop_filters(&mut self, inloop_filters: InloopFilterType) {
            self.dav1d_settings.inloop_filters = inloop_filters.bits();
        }

        pub fn get_inloop_filters(&self) -> InloopFilterType {
            InloopFilterType::from_bits_truncate(self.dav1d_settings.inloop_filters)
        }

        pub fn set_decode_frame_type(&mut self, decode_frame_type: DecodeFrameType) {
            self.dav1d_settings.decode_frame_type = decode_frame_type.into();
        }

        pub fn get_decode_frame_type(&self) -> DecodeFrameType {
            DecodeFrameType::try_from(self.dav1d_settings.decode_frame_type)
                .expect("Invalid Dav1dDecodeFrameType")
        }
    }

    bitflags::bitflags! {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
        pub struct InloopFilterType: u32 {
            const DEBLOCK = DAV1D_INLOOPFILTER_DEBLOCK;
            const CDEF = DAV1D_INLOOPFILTER_CDEF;
            const RESTORATION = DAV1D_INLOOPFILTER_RESTORATION;
        }
    }

    #[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
    pub enum DecodeFrameType {
        #[default]
        All,
        Reference,
        Intra,
        Key,
    }

    impl TryFrom<u32> for DecodeFrameType {
        type Error = TryFromEnumError;

        fn try_from(value: u32) -> Result<Self, Self::Error> {
            match value {
                DAV1D_DECODEFRAMETYPE_ALL => Ok(DecodeFrameType::All),
                DAV1D_DECODEFRAMETYPE_REFERENCE => Ok(DecodeFrameType::Reference),
                DAV1D_DECODEFRAMETYPE_INTRA => Ok(DecodeFrameType::Intra),
                DAV1D_DECODEFRAMETYPE_KEY => Ok(DecodeFrameType::Key),
                _ => Err(TryFromEnumError(())),
            }
        }
    }

    impl From<DecodeFrameType> for u32 {
        fn from(v: DecodeFrameType) -> u32 {
            match v {
                DecodeFrameType::All => DAV1D_DECODEFRAMETYPE_ALL,
                DecodeFrameType::Reference => DAV1D_DECODEFRAMETYPE_REFERENCE,
                DecodeFrameType::Intra => DAV1D_DECODEFRAMETYPE_INTRA,
                DecodeFrameType::Key => DAV1D_DECODEFRAMETYPE_KEY,
            }
        }
    }

    /// The error type returned when a conversion from a C enum fails.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct TryFromEnumError(());

    impl std::fmt::Display for TryFromEnumError {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            fmt.write_str("Invalid enum value")
        }
    }

    impl From<std::convert::Infallible> for TryFromEnumError {
        fn from(x: std::convert::Infallible) -> TryFromEnumError {
            match x {}
        }
    }

    impl std::error::Error for TryFromEnumError {}

    /// A `dav1d` decoder instance.
    pub struct Decoder {
        dec: Dav1dContext,
        pending_data: Option<Dav1dData>,
    }

    unsafe extern "C" fn release_wrapped_data<T: AsRef<[u8]>>(
        _data: *const u8,
        cookie: Option<SendSyncNonNull<std::ffi::c_void>>,
    ) {
        let cookie = cookie.unwrap().as_ptr().as_ptr();
        let buf = unsafe { Box::from_raw(cookie as *mut T) };
        drop(buf);
    }

    impl Decoder {
        /// Creates a new [`Decoder`] instance with given [`Settings`].
        pub fn with_settings(settings: &Settings) -> Result<Self, Error> {
            unsafe {
                let mut dec = mem::MaybeUninit::uninit();

                let ret = dav1d_open(
                    Some(NonNull::new(dec.as_mut_ptr()).unwrap()),
                    Some(NonNull::new(&settings.dav1d_settings as *const _ as *mut _).unwrap()),
                );

                match rav1d_result(ret) {
                    Ok(_) => Ok(Decoder {
                        dec: dec.assume_init().unwrap(),
                        pending_data: None,
                    }),
                    Err(err) => Err(Error::from(err)),
                }
            }
        }

        /// Creates a new [`Decoder`] instance with the default settings.
        pub fn new() -> Result<Self, Error> {
            Self::with_settings(&Settings::default())
        }

        /// Flush the decoder.
        ///
        /// This flushes all delayed frames in the decoder and clears the internal decoder state.
        ///
        /// All currently pending frames are available afterwards via [`Decoder::get_picture`].
        pub fn flush(&mut self) {
            unsafe {
                dav1d_flush(self.dec);
                if let Some(mut pending_data) = self.pending_data.take() {
                    dav1d_data_unref(Some(NonNull::new(&mut pending_data).unwrap()));
                }
            }
        }

        /// Send new AV1 data to the decoder.
        ///
        /// After this returned `Ok(())` or `Err([Error::Again])` there might be decoded frames
        /// available via [`Decoder::get_picture`].
        ///
        /// # Panics
        ///
        /// If a previous call returned [`Error::Again`] then this must not be called again until
        /// [`Decoder::send_pending_data`] has returned `Ok(())`.
        pub fn send_data<T: AsRef<[u8]> + Send + Sync + 'static>(
            &mut self,
            buf: T,
            offset: Option<i64>,
            timestamp: Option<i64>,
            duration: Option<i64>,
        ) -> Result<(), Error> {
            assert!(
                self.pending_data.is_none(),
                "Have pending data that needs to be handled first"
            );

            let buf = Box::new(buf);
            let slice = (*buf).as_ref();
            let len = slice.len();

            unsafe {
                let mut data: Dav1dData = mem::zeroed();
                let _ret = dav1d_data_wrap(
                    option_nonnull(&mut data),
                    option_nonnull(slice.as_ptr() as *mut _),
                    len,
                    Some(release_wrapped_data::<T>),
                    option_send_sync_non_null(buf).map(|v| v.cast()),
                );
                if let Some(offset) = offset {
                    data.m.offset = offset as libc::off_t;
                }
                if let Some(timestamp) = timestamp {
                    data.m.timestamp = timestamp;
                }
                if let Some(duration) = duration {
                    data.m.duration = duration;
                }

                let ret = dav1d_send_data(Some(self.dec), option_nonnull(&mut data));
                if let Err(err) = rav1d_result(ret) {
                    let ret = Error::from(err);

                    if ret.is_again() {
                        self.pending_data = Some(data);
                    } else {
                        dav1d_data_unref(option_nonnull(&mut data));
                    }

                    return Err(ret);
                }

                if data.sz > 0 {
                    self.pending_data = Some(data);
                    return Err(Error::Again);
                }

                Ok(())
            }
        }

        /// Sends any pending data to the decoder.
        ///
        /// This has to be called after [`Decoder::send_data`] has returned `Err([Error::Again])` to
        /// consume any futher pending data.
        ///
        /// After this returned `Ok(())` or `Err([Error::Again])` there might be decoded frames
        /// available via [`Decoder::get_picture`].
        pub fn send_pending_data(&mut self) -> Result<(), Error> {
            let mut data = match self.pending_data.take() {
                None => {
                    return Ok(());
                }
                Some(data) => data,
            };

            unsafe {
                let ret = dav1d_send_data(Some(self.dec), option_nonnull(&mut data));
                if let Err(err) = rav1d_result(ret) {
                    let ret = Error::from(err);

                    if ret.is_again() {
                        self.pending_data = Some(data);
                    } else {
                        dav1d_data_unref(option_nonnull(&mut data));
                    }

                    return Err(ret);
                }

                if data.sz > 0 {
                    self.pending_data = Some(data);
                    return Err(Error::Again);
                }

                Ok(())
            }
        }

        /// Get the next decoded frame from the decoder.
        ///
        /// If this returns `Err([Error::Again])` then further data has to be sent to the decoder
        /// before further decoded frames become available.
        ///
        /// To make most use of frame threading this function should only be called once per submitted
        /// input frame and not until it returns `Err([Error::Again])`. Calling it in a loop should
        /// only be done to drain all pending frames at the end.
        pub fn get_picture(&mut self) -> Result<Picture, Error> {
            unsafe {
                let mut pic: Dav1dPicture = mem::zeroed();
                let ret = dav1d_get_picture(Some(self.dec), option_nonnull(&mut pic));

                if let Err(err) = rav1d_result(ret) {
                    Err(Error::from(err))
                } else {
                    let inner = InnerPicture { pic };
                    Ok(Picture {
                        inner: Arc::new(inner),
                    })
                }
            }
        }

        /// Get the decoder delay.
        pub fn get_frame_delay(&self) -> u32 {
            unsafe {
                dav1d_get_frame_delay(option_nonnull(&self.dec as *const _ as *mut _)).0 as u32
            }
        }
    }

    impl Drop for Decoder {
        fn drop(&mut self) {
            unsafe {
                if let Some(mut pending_data) = self.pending_data.take() {
                    dav1d_data_unref(option_nonnull(&mut pending_data));
                }
                let mut dec = Some(self.dec);
                dav1d_close(option_nonnull(&mut dec));
            };
        }
    }

    unsafe impl Send for Decoder {}
    unsafe impl Sync for Decoder {}

    struct InnerPicture {
        pub pic: Dav1dPicture,
    }

    /// A decoded frame.
    #[derive(Clone)]
    pub struct Picture {
        inner: Arc<InnerPicture>,
    }

    /// Pixel layout of a frame.
    #[derive(Debug, Eq, PartialEq, Copy, Clone)]
    pub enum PixelLayout {
        /// Monochrome.
        I400,
        /// 4:2:0 planar.
        I420,
        /// 4:2:2 planar.
        I422,
        /// 4:4:4 planar.
        I444,
    }

    /// Frame component.
    #[derive(Eq, PartialEq, Copy, Clone, Debug)]
    pub enum PlanarImageComponent {
        /// Y component.
        Y,
        /// U component.
        U,
        /// V component.
        V,
    }

    impl From<usize> for PlanarImageComponent {
        fn from(index: usize) -> Self {
            match index {
                0 => PlanarImageComponent::Y,
                1 => PlanarImageComponent::U,
                2 => PlanarImageComponent::V,
                _ => panic!("Invalid YUV index: {}", index),
            }
        }
    }

    impl From<PlanarImageComponent> for usize {
        fn from(component: PlanarImageComponent) -> Self {
            match component {
                PlanarImageComponent::Y => 0,
                PlanarImageComponent::U => 1,
                PlanarImageComponent::V => 2,
            }
        }
    }

    /// A single plane of a decoded frame.
    ///
    /// This can be used like a `&[u8]`.
    #[derive(Clone)]
    pub struct Plane(Picture, PlanarImageComponent);

    impl AsRef<[u8]> for Plane {
        fn as_ref(&self) -> &[u8] {
            let (stride, height) = self.0.plane_data_geometry(self.1);
            unsafe {
                std::slice::from_raw_parts(
                    self.0.plane_data_ptr(self.1) as *const u8,
                    (stride * height) as usize,
                )
            }
        }
    }

    impl std::ops::Deref for Plane {
        type Target = [u8];

        fn deref(&self) -> &Self::Target {
            self.as_ref()
        }
    }

    static_assertions::assert_impl_all!(Plane: Send, Sync);

    /// Number of bits per component.
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct BitsPerComponent(pub usize);

    impl Picture {
        /// Stride in pixels of the `component` for the decoded frame.
        pub fn stride(&self, component: PlanarImageComponent) -> u32 {
            let s = match component {
                PlanarImageComponent::Y => 0,
                _ => 1,
            };
            self.inner.pic.stride[s] as u32
        }

        /// Raw pointer to the data of the `component` for the decoded frame.
        pub fn plane_data_ptr(&self, component: PlanarImageComponent) -> *mut c_void {
            let index: usize = component.into();
            self.inner.pic.data[index].unwrap().as_ptr()
        }

        /// Plane geometry of the `component` for the decoded frame.
        ///
        /// This returns the stride and height.
        pub fn plane_data_geometry(&self, component: PlanarImageComponent) -> (u32, u32) {
            let height = match component {
                PlanarImageComponent::Y => self.height(),
                _ => match self.pixel_layout() {
                    PixelLayout::I420 => (self.height() + 1) / 2,
                    PixelLayout::I400 | PixelLayout::I422 | PixelLayout::I444 => self.height(),
                },
            };
            (self.stride(component), height)
        }

        /// Plane data of the `component` for the decoded frame.
        pub fn plane(&self, component: PlanarImageComponent) -> Plane {
            Plane(self.clone(), component)
        }

        /// Bit depth of the plane data.
        ///
        /// This returns 8 or 16 for the underlying integer type used for the plane data.
        ///
        /// Check [`Picture::bits_per_component`] for the number of bits that are used.
        pub fn bit_depth(&self) -> usize {
            self.inner.pic.p.bpc as usize
        }

        /// Bits used per component of the plane data.
        ///
        /// Check [`Picture::bit_depth`] for the number of storage bits.
        pub fn bits_per_component(&self) -> Option<BitsPerComponent> {
            unsafe {
                match (*self.inner.pic.seq_hdr.unwrap().as_ptr()).hbd {
                    0 => Some(BitsPerComponent(8)),
                    1 => Some(BitsPerComponent(10)),
                    2 => Some(BitsPerComponent(12)),
                    _ => None,
                }
            }
        }

        /// Width of the frame.
        pub fn width(&self) -> u32 {
            self.inner.pic.p.w as u32
        }

        /// Height of the frame.
        pub fn height(&self) -> u32 {
            self.inner.pic.p.h as u32
        }

        /// Pixel layout of the frame.
        pub fn pixel_layout(&self) -> PixelLayout {
            #[allow(non_upper_case_globals)]
            match self.inner.pic.p.layout {
                DAV1D_PIXEL_LAYOUT_I400 => PixelLayout::I400,
                DAV1D_PIXEL_LAYOUT_I420 => PixelLayout::I420,
                DAV1D_PIXEL_LAYOUT_I422 => PixelLayout::I422,
                DAV1D_PIXEL_LAYOUT_I444 => PixelLayout::I444,
                _ => unreachable!(),
            }
        }

        /// Timestamp of the frame.
        ///
        /// This is the same timestamp as the one provided to [`Decoder::send_data`].
        pub fn timestamp(&self) -> Option<i64> {
            let ts = self.inner.pic.m.timestamp;
            if ts == i64::MIN {
                None
            } else {
                Some(ts)
            }
        }

        /// Duration of the frame.
        ///
        /// This is the same duration as the one provided to [`Decoder::send_data`] or `0` if none was
        /// provided.
        pub fn duration(&self) -> i64 {
            self.inner.pic.m.duration
        }

        /// Offset of the frame.
        ///
        /// This is the same offset as the one provided to [`Decoder::send_data`] or `-1` if none was
        /// provided.
        pub fn offset(&self) -> i64 {
            self.inner.pic.m.offset as i64
        }

        /// Chromaticity coordinates of the source colour primaries.
        pub fn color_primaries(&self) -> pixel::ColorPrimaries {
            unsafe {
                #[allow(non_upper_case_globals)]
                match (*self.inner.pic.seq_hdr.unwrap().as_ptr()).pri {
                    DAV1D_COLOR_PRI_BT709 => pixel::ColorPrimaries::BT709,
                    DAV1D_COLOR_PRI_UNKNOWN => pixel::ColorPrimaries::Unspecified,
                    DAV1D_COLOR_PRI_BT470M => pixel::ColorPrimaries::BT470M,
                    DAV1D_COLOR_PRI_BT470BG => pixel::ColorPrimaries::BT470BG,
                    DAV1D_COLOR_PRI_BT601 => pixel::ColorPrimaries::BT470BG,
                    DAV1D_COLOR_PRI_SMPTE240 => pixel::ColorPrimaries::ST240M,
                    DAV1D_COLOR_PRI_FILM => pixel::ColorPrimaries::Film,
                    DAV1D_COLOR_PRI_BT2020 => pixel::ColorPrimaries::BT2020,
                    DAV1D_COLOR_PRI_XYZ => pixel::ColorPrimaries::ST428,
                    DAV1D_COLOR_PRI_SMPTE431 => pixel::ColorPrimaries::P3DCI,
                    DAV1D_COLOR_PRI_SMPTE432 => pixel::ColorPrimaries::P3Display,
                    DAV1D_COLOR_PRI_EBU3213 => pixel::ColorPrimaries::Tech3213,
                    23..=DAV1D_COLOR_PRI_RESERVED => pixel::ColorPrimaries::Unspecified,
                    _ => unreachable!(),
                }
            }
        }

        /// Transfer characteristics function.
        pub fn transfer_characteristic(&self) -> pixel::TransferCharacteristic {
            unsafe {
                #[allow(non_upper_case_globals)]
                match (*self.inner.pic.seq_hdr.unwrap().as_ptr()).trc {
                    DAV1D_TRC_BT709 => pixel::TransferCharacteristic::BT1886,
                    DAV1D_TRC_UNKNOWN => pixel::TransferCharacteristic::Unspecified,
                    DAV1D_TRC_BT470M => pixel::TransferCharacteristic::BT470M,
                    DAV1D_TRC_BT470BG => pixel::TransferCharacteristic::BT470BG,
                    DAV1D_TRC_BT601 => pixel::TransferCharacteristic::ST170M,
                    DAV1D_TRC_SMPTE240 => pixel::TransferCharacteristic::ST240M,
                    DAV1D_TRC_LINEAR => pixel::TransferCharacteristic::Linear,
                    DAV1D_TRC_LOG100 => pixel::TransferCharacteristic::Logarithmic100,
                    DAV1D_TRC_LOG100_SQRT10 => pixel::TransferCharacteristic::Logarithmic316,
                    DAV1D_TRC_IEC61966 => pixel::TransferCharacteristic::SRGB,
                    DAV1D_TRC_BT1361 => pixel::TransferCharacteristic::BT1886,
                    DAV1D_TRC_SRGB => pixel::TransferCharacteristic::SRGB,
                    DAV1D_TRC_BT2020_10BIT => pixel::TransferCharacteristic::BT2020Ten,
                    DAV1D_TRC_BT2020_12BIT => pixel::TransferCharacteristic::BT2020Twelve,
                    DAV1D_TRC_SMPTE2084 => pixel::TransferCharacteristic::PerceptualQuantizer,
                    DAV1D_TRC_SMPTE428 => pixel::TransferCharacteristic::ST428,
                    DAV1D_TRC_HLG => pixel::TransferCharacteristic::HybridLogGamma,
                    19..=DAV1D_TRC_RESERVED => pixel::TransferCharacteristic::Unspecified,
                    _ => unreachable!(),
                }
            }
        }

        /// Matrix coefficients used in deriving luma and chroma signals from the
        /// green, blue and red or X, Y and Z primaries.
        pub fn matrix_coefficients(&self) -> pixel::MatrixCoefficients {
            unsafe {
                #[allow(non_upper_case_globals)]
                match (*self.inner.pic.seq_hdr.unwrap().as_ptr()).mtrx {
                    DAV1D_MC_IDENTITY => pixel::MatrixCoefficients::Identity,
                    DAV1D_MC_BT709 => pixel::MatrixCoefficients::BT709,
                    DAV1D_MC_UNKNOWN => pixel::MatrixCoefficients::Unspecified,
                    DAV1D_MC_FCC => pixel::MatrixCoefficients::BT470M,
                    DAV1D_MC_BT470BG => pixel::MatrixCoefficients::BT470BG,
                    DAV1D_MC_BT601 => pixel::MatrixCoefficients::BT470BG,
                    DAV1D_MC_SMPTE240 => pixel::MatrixCoefficients::ST240M,
                    DAV1D_MC_SMPTE_YCGCO => pixel::MatrixCoefficients::YCgCo,
                    DAV1D_MC_BT2020_NCL => pixel::MatrixCoefficients::BT2020NonConstantLuminance,
                    DAV1D_MC_BT2020_CL => pixel::MatrixCoefficients::BT2020ConstantLuminance,
                    DAV1D_MC_SMPTE2085 => pixel::MatrixCoefficients::ST2085,
                    DAV1D_MC_CHROMAT_NCL => {
                        pixel::MatrixCoefficients::ChromaticityDerivedNonConstantLuminance
                    }
                    DAV1D_MC_CHROMAT_CL => {
                        pixel::MatrixCoefficients::ChromaticityDerivedConstantLuminance
                    }
                    DAV1D_MC_ICTCP => pixel::MatrixCoefficients::ICtCp,
                    15..=DAV1D_MC_RESERVED => pixel::MatrixCoefficients::Unspecified,
                    _ => unreachable!(),
                }
            }
        }

        /// YUV color range.
        pub fn color_range(&self) -> pixel::YUVRange {
            unsafe {
                match (*self.inner.pic.seq_hdr.unwrap().as_ptr()).color_range {
                    0 => pixel::YUVRange::Limited,
                    _ => pixel::YUVRange::Full,
                }
            }
        }

        /// Sample position for subsampled chroma.
        pub fn chroma_location(&self) -> pixel::ChromaLocation {
            // According to y4m mapping declared in dav1d's output/y4m2.c and applied from FFmpeg's yuv4mpegdec.c
            unsafe {
                match (*self.inner.pic.seq_hdr.unwrap().as_ptr()).chr {
                    DAV1D_CHR_UNKNOWN | DAV1D_CHR_COLOCATED => pixel::ChromaLocation::Center,
                    DAV1D_CHR_VERTICAL => pixel::ChromaLocation::Left,
                    _ => unreachable!(),
                }
            }
        }
    }

    static_assertions::assert_impl_all!(Picture: Send, Sync);

    unsafe impl Send for InnerPicture {}
    unsafe impl Sync for InnerPicture {}

    impl Drop for InnerPicture {
        fn drop(&mut self) {
            unsafe {
                dav1d_picture_unref(option_nonnull(&mut self.pic));
            }
        }
    }

    impl std::fmt::Debug for Picture {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Picture")
                .field("width", &self.width())
                .field("height", &self.height())
                .field("bit_depth", &self.bit_depth())
                .field("pixel_layout", &self.pixel_layout())
                .field("timestamp", &self.timestamp())
                .field("duration", &self.duration())
                .field("offset", &self.offset())
                .field("color_primaries", &self.color_primaries())
                .field("transfer_characteristic", &self.transfer_characteristic())
                .field("matrix_coefficients", &self.matrix_coefficients())
                .field("color_range", &self.color_range())
                .field("chroma_location", &self.chroma_location())
                .finish()
        }
    }
}

pub use dav1d::*;
