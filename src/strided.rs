use crate::include::common::bitdepth::BitDepth;

pub trait Strided {
    /// Stride in number of [`u8`] bytes.
    fn stride(&self) -> isize;

    /// Stride in number of [`BitDepth::Pixel`]s.
    fn pixel_stride<BD: BitDepth>(&self) -> isize {
        BD::pxstride(self.stride())
    }
}
