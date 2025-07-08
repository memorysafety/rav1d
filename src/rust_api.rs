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

    use std::ffi::c_void;
    use std::fmt::{Debug, Formatter};
    use std::ops::Deref;
    use std::sync::Arc;
    use std::{fmt, mem};

    pub use av_data::pixel;

    use crate::c_arc::CArc;
    use crate::c_box::CBox;
    use crate::error::Rav1dError;
    pub use crate::include::dav1d::dav1d::{
        Rav1dDecodeFrameType as DecodeFrameType, Rav1dInloopFilterType as InloopFilterType,
    };
    use crate::include::dav1d::headers::{
        Rav1dChromaSamplePosition, Rav1dColorPrimaries, Rav1dMatrixCoefficients,
        Rav1dTransferCharacteristics, DAV1D_COLOR_PRI_RESERVED, DAV1D_MC_RESERVED,
        DAV1D_TRC_RESERVED,
    };
    pub use crate::include::dav1d::headers::{
        Rav1dContentLightLevel as ContentLightLevel, Rav1dMasteringDisplay as MasteringDisplay,
        Rav1dPixelLayout as PixelLayout,
    };
    use crate::include::dav1d::picture::Rav1dPicture;
    pub use crate::include::dav1d::picture::RAV1D_PICTURE_ALIGNMENT as PICTURE_ALIGNMENT;
    use crate::internal::Rav1dContext;
    use crate::pixels::Pixels;
    use crate::{
        rav1d_close, rav1d_flush, rav1d_get_frame_delay, rav1d_get_picture, rav1d_open,
        rav1d_send_data, Rav1dData, Rav1dSettings,
    };

    /// Settings for creating a new [`Decoder`] instance.
    /// See documentation for native `Dav1dSettings` struct.
    #[derive(Default)]
    pub struct Settings {
        pub(crate) rav1d_settings: Rav1dSettings,
    }

    static_assertions::assert_impl_all!(Settings: Send, Sync);

    impl Settings {
        /// Creates a new [`Settings`] instance with default settings.
        pub fn new() -> Self {
            Self::default()
        }

        pub fn set_n_threads(&mut self, n_threads: u32) {
            self.rav1d_settings.n_threads = n_threads;
        }

        pub fn get_n_threads(&self) -> u32 {
            self.rav1d_settings.n_threads
        }

        pub fn set_max_frame_delay(&mut self, max_frame_delay: u32) {
            self.rav1d_settings.max_frame_delay = max_frame_delay;
        }

        pub fn get_max_frame_delay(&self) -> u32 {
            self.rav1d_settings.max_frame_delay
        }

        pub fn set_apply_grain(&mut self, apply_grain: bool) {
            self.rav1d_settings.apply_grain = apply_grain;
        }

        pub fn get_apply_grain(&self) -> bool {
            self.rav1d_settings.apply_grain
        }

        pub fn set_operating_point(&mut self, operating_point: u8) {
            self.rav1d_settings.operating_point = operating_point;
        }

        pub fn get_operating_point(&self) -> u8 {
            self.rav1d_settings.operating_point
        }

        pub fn set_all_layers(&mut self, all_layers: bool) {
            self.rav1d_settings.all_layers = all_layers;
        }

        pub fn get_all_layers(&self) -> bool {
            self.rav1d_settings.all_layers
        }

        pub fn set_frame_size_limit(&mut self, frame_size_limit: u32) {
            self.rav1d_settings.frame_size_limit = frame_size_limit;
        }

        pub fn get_frame_size_limit(&self) -> u32 {
            self.rav1d_settings.frame_size_limit
        }

        pub fn set_strict_std_compliance(&mut self, strict_std_compliance: bool) {
            self.rav1d_settings.strict_std_compliance = strict_std_compliance;
        }

        pub fn get_strict_std_compliance(&self) -> bool {
            self.rav1d_settings.strict_std_compliance
        }

        pub fn set_output_invisible_frames(&mut self, output_invisible_frames: bool) {
            self.rav1d_settings.output_invisible_frames = output_invisible_frames;
        }

        pub fn get_output_invisible_frames(&self) -> bool {
            self.rav1d_settings.output_invisible_frames
        }

        pub fn set_inloop_filters(&mut self, inloop_filters: InloopFilterType) {
            self.rav1d_settings.inloop_filters = inloop_filters;
        }

        pub fn get_inloop_filters(&self) -> InloopFilterType {
            self.rav1d_settings.inloop_filters
        }

        pub fn set_decode_frame_type(&mut self, decode_frame_type: DecodeFrameType) {
            self.rav1d_settings.decode_frame_type = decode_frame_type;
        }

        pub fn get_decode_frame_type(&self) -> DecodeFrameType {
            self.rav1d_settings.decode_frame_type
        }
    }

    /// A `dav1d` decoder instance.
    pub struct Decoder {
        ctx: Arc<Rav1dContext>,
        pending_data: Option<Rav1dData>,
        n_threads: u32,
        max_frame_delay: u32,
    }

    impl Decoder {
        /// Creates a new [`Decoder`] instance with given [`Settings`].
        pub fn with_settings(settings: &Settings) -> Result<Self, Rav1dError> {
            rav1d_open(&settings.rav1d_settings).map(|ctx| Decoder {
                ctx,
                pending_data: None,
                n_threads: settings.rav1d_settings.n_threads,
                max_frame_delay: settings.rav1d_settings.max_frame_delay,
            })
        }

        /// Creates a new [`Decoder`] instance with the default settings.
        pub fn new() -> Result<Self, Rav1dError> {
            Self::with_settings(&Settings::default())
        }

        /// Flush the decoder.
        ///
        /// This flushes all delayed frames in the decoder and clears the internal decoder state.
        ///
        /// All currently pending frames are available afterwards via [`Decoder::get_picture`].
        pub fn flush(&mut self) {
            rav1d_flush(&self.ctx);
            if let Some(mut pending_data) = self.pending_data.take() {
                let _ = mem::take(&mut pending_data);
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
        ) -> Result<(), Rav1dError> {
            assert!(
                self.pending_data.is_none(),
                "Have pending data that needs to be handled first"
            );

            let buf = buf.as_ref().to_vec().into_boxed_slice();
            let slice = &*buf;
            let len = slice.len();

            let mut data = Rav1dData::create(len).unwrap();
            data.data = Some(CArc::wrap(CBox::from_box(buf)).unwrap());
            if let Some(offset) = offset {
                data.m.offset = offset;
            }
            if let Some(timestamp) = timestamp {
                data.m.timestamp = timestamp;
            }
            if let Some(duration) = duration {
                data.m.duration = duration;
            }

            let ret = rav1d_send_data(&self.ctx, &mut data);
            if let Err(err) = ret {
                let ret = err;
                if matches!(ret, Rav1dError::Again) {
                    self.pending_data = Some(data);
                } else {
                    let _ = mem::take(&mut data);
                }

                return Err(ret);
            }

            if data.data.as_ref().is_some_and(|d| d.len() > 0) {
                self.pending_data = Some(data);
                return Err(Rav1dError::Again);
            }

            Ok(())
        }

        /// Sends any pending data to the decoder.
        ///
        /// This has to be called after [`Decoder::send_data`] has returned `Err([Error::Again])` to
        /// consume any futher pending data.
        ///
        /// After this returned `Ok(())` or `Err([Error::Again])` there might be decoded frames
        /// available via [`Decoder::get_picture`].
        pub fn send_pending_data(&mut self) -> Result<(), Rav1dError> {
            let mut data = match self.pending_data.take() {
                None => {
                    return Ok(());
                }
                Some(data) => data,
            };

            let ret = rav1d_send_data(&self.ctx, &mut data);
            if let Err(err) = ret {
                let ret = err;

                if matches!(ret, Rav1dError::Again) {
                    self.pending_data = Some(data);
                } else {
                    let _ = mem::take(&mut data);
                }

                return Err(ret);
            }

            if data.data.as_ref().is_some_and(|d| d.len() > 0) {
                self.pending_data = Some(data);
                return Err(Rav1dError::Again);
            }

            Ok(())
        }

        /// Get the next decoded frame from the decoder.
        ///
        /// If this returns `Err([Error::Again])` then further data has to be sent to the decoder
        /// before further decoded frames become available.
        ///
        /// To make most use of frame threading this function should only be called once per submitted
        /// input frame and not until it returns `Err([Error::Again])`. Calling it in a loop should
        /// only be done to drain all pending frames at the end.
        pub fn get_picture(&mut self) -> Result<Picture, Rav1dError> {
            let mut pic = Rav1dPicture::default();
            let ret = rav1d_get_picture(&self.ctx, &mut pic);

            if let Err(err) = ret {
                Err(err)
            } else {
                let inner = InnerPicture { pic };
                Ok(Picture {
                    inner: Arc::new(inner),
                })
            }
        }

        /// Get the decoder delay.
        pub fn get_frame_delay(&self) -> u32 {
            // The only fields this actually needs from Rav1dSettings are n_threads and max_frame_delay so we just pass these in directly

            rav1d_get_frame_delay(&Rav1dSettings {
                n_threads: self.n_threads,
                max_frame_delay: self.max_frame_delay,
                ..Default::default()
            })
            .unwrap() as u32
        }
    }

    impl Drop for Decoder {
        fn drop(&mut self) {
            if let Some(mut pending_data) = self.pending_data.take() {
                let _ = mem::take(&mut pending_data);
            }
            rav1d_close(self.ctx.clone());
        }
    }

    static_assertions::assert_impl_all!(Decoder: Send, Sync);

    struct InnerPicture {
        pub pic: Rav1dPicture,
    }

    /// A decoded frame.
    #[derive(Clone)]
    pub struct Picture {
        inner: Arc<InnerPicture>,
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

    impl Deref for Plane {
        type Target = [u8];

        fn deref(&self) -> &Self::Target {
            self.as_ref()
        }
    }

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
            self.inner.pic.data.as_ref().unwrap().data[index]
                .as_byte_mut_ptr()
                .cast()
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
            match self.inner.pic.seq_hdr.as_ref().unwrap().hbd {
                0 => Some(BitsPerComponent(8)),
                1 => Some(BitsPerComponent(10)),
                2 => Some(BitsPerComponent(12)),
                _ => None,
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
            self.inner.pic.p.layout
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
            match self.inner.pic.seq_hdr.as_ref().unwrap().pri {
                Rav1dColorPrimaries::BT709 => pixel::ColorPrimaries::BT709,
                Rav1dColorPrimaries::UNKNOWN => pixel::ColorPrimaries::Unspecified,
                Rav1dColorPrimaries::BT470M => pixel::ColorPrimaries::BT470M,
                Rav1dColorPrimaries::BT470BG => pixel::ColorPrimaries::BT470BG,
                Rav1dColorPrimaries::BT601 => pixel::ColorPrimaries::BT470BG,
                Rav1dColorPrimaries::SMPTE240 => pixel::ColorPrimaries::ST240M,
                Rav1dColorPrimaries::FILM => pixel::ColorPrimaries::Film,
                Rav1dColorPrimaries::BT2020 => pixel::ColorPrimaries::BT2020,
                Rav1dColorPrimaries::XYZ => pixel::ColorPrimaries::ST428,
                Rav1dColorPrimaries::SMPTE431 => pixel::ColorPrimaries::P3DCI,
                Rav1dColorPrimaries::SMPTE432 => pixel::ColorPrimaries::P3Display,
                Rav1dColorPrimaries::EBU3213 => pixel::ColorPrimaries::Tech3213,
                Rav1dColorPrimaries(x) => {
                    if (23..=DAV1D_COLOR_PRI_RESERVED).contains(&(x as u32)) {
                        pixel::ColorPrimaries::Unspecified
                    } else {
                        unreachable!()
                    }
                }
            }
        }

        /// Transfer characteristics function.
        pub fn transfer_characteristic(&self) -> pixel::TransferCharacteristic {
            match self.inner.pic.seq_hdr.as_ref().unwrap().trc {
                Rav1dTransferCharacteristics::BT709 => pixel::TransferCharacteristic::BT1886,
                Rav1dTransferCharacteristics::UNKNOWN => pixel::TransferCharacteristic::Unspecified,
                Rav1dTransferCharacteristics::BT470M => pixel::TransferCharacteristic::BT470M,
                Rav1dTransferCharacteristics::BT470BG => pixel::TransferCharacteristic::BT470BG,
                Rav1dTransferCharacteristics::BT601 => pixel::TransferCharacteristic::ST170M,
                Rav1dTransferCharacteristics::SMPTE240 => pixel::TransferCharacteristic::ST240M,
                Rav1dTransferCharacteristics::LINEAR => pixel::TransferCharacteristic::Linear,
                Rav1dTransferCharacteristics::LOG100 => {
                    pixel::TransferCharacteristic::Logarithmic100
                }
                Rav1dTransferCharacteristics::LOG100_SQRT10 => {
                    pixel::TransferCharacteristic::Logarithmic316
                }
                Rav1dTransferCharacteristics::IEC61966 => pixel::TransferCharacteristic::SRGB,
                Rav1dTransferCharacteristics::BT1361 => pixel::TransferCharacteristic::BT1886,
                Rav1dTransferCharacteristics::SRGB => pixel::TransferCharacteristic::SRGB,
                Rav1dTransferCharacteristics::BT2020_10BIT => {
                    pixel::TransferCharacteristic::BT2020Ten
                }
                Rav1dTransferCharacteristics::BT2020_12BIT => {
                    pixel::TransferCharacteristic::BT2020Twelve
                }
                Rav1dTransferCharacteristics::SMPTE2084 => {
                    pixel::TransferCharacteristic::PerceptualQuantizer
                }
                Rav1dTransferCharacteristics::SMPTE428 => pixel::TransferCharacteristic::ST428,
                Rav1dTransferCharacteristics::HLG => pixel::TransferCharacteristic::HybridLogGamma,
                Rav1dTransferCharacteristics(x) => {
                    if (19..=DAV1D_TRC_RESERVED).contains(&(x as u32)) {
                        pixel::TransferCharacteristic::Unspecified
                    } else {
                        unreachable!()
                    }
                }
            }
        }

        /// Matrix coefficients used in deriving luma and chroma signals from the
        /// green, blue and red or X, Y and Z primaries.
        pub fn matrix_coefficients(&self) -> pixel::MatrixCoefficients {
            match self.inner.pic.seq_hdr.as_ref().unwrap().mtrx {
                Rav1dMatrixCoefficients::IDENTITY => pixel::MatrixCoefficients::Identity,
                Rav1dMatrixCoefficients::BT709 => pixel::MatrixCoefficients::BT709,
                Rav1dMatrixCoefficients::UNKNOWN => pixel::MatrixCoefficients::Unspecified,
                Rav1dMatrixCoefficients::FCC => pixel::MatrixCoefficients::BT470M,
                Rav1dMatrixCoefficients::BT470BG => pixel::MatrixCoefficients::BT470BG,
                Rav1dMatrixCoefficients::BT601 => pixel::MatrixCoefficients::BT470BG,
                Rav1dMatrixCoefficients::SMPTE240 => pixel::MatrixCoefficients::ST240M,
                Rav1dMatrixCoefficients::SMPTE_YCGCO => pixel::MatrixCoefficients::YCgCo,
                Rav1dMatrixCoefficients::BT2020_NCL => {
                    pixel::MatrixCoefficients::BT2020NonConstantLuminance
                }
                Rav1dMatrixCoefficients::BT2020_CL => {
                    pixel::MatrixCoefficients::BT2020ConstantLuminance
                }
                Rav1dMatrixCoefficients::SMPTE2085 => pixel::MatrixCoefficients::ST2085,
                Rav1dMatrixCoefficients::CHROMAT_NCL => {
                    pixel::MatrixCoefficients::ChromaticityDerivedNonConstantLuminance
                }
                Rav1dMatrixCoefficients::CHROMAT_CL => {
                    pixel::MatrixCoefficients::ChromaticityDerivedConstantLuminance
                }
                Rav1dMatrixCoefficients::ICTCP => pixel::MatrixCoefficients::ICtCp,
                Rav1dMatrixCoefficients(x) => {
                    if (15..=DAV1D_MC_RESERVED).contains(&(x as u32)) {
                        pixel::MatrixCoefficients::Unspecified
                    } else {
                        unreachable!()
                    }
                }
            }
        }

        /// YUV color range.
        pub fn color_range(&self) -> pixel::YUVRange {
            match self.inner.pic.seq_hdr.as_ref().unwrap().color_range {
                0 => pixel::YUVRange::Limited,
                _ => pixel::YUVRange::Full,
            }
        }

        /// Sample position for subsampled chroma.
        pub fn chroma_location(&self) -> pixel::ChromaLocation {
            // According to y4m mapping declared in dav1d's output/y4m2.c and applied from FFmpeg's yuv4mpegdec.c
            match self.inner.pic.seq_hdr.as_ref().unwrap().chr {
                Rav1dChromaSamplePosition::Unknown | Rav1dChromaSamplePosition::Colocated => {
                    pixel::ChromaLocation::Center
                }
                Rav1dChromaSamplePosition::Vertical => pixel::ChromaLocation::Left,
                Rav1dChromaSamplePosition::_Reserved => unreachable!(),
            }
        }
    }

    impl Debug for Picture {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
