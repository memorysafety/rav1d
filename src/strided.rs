use crate::include::common::bitdepth::BitDepth;
use std::ops::Deref;
use std::ops::DerefMut;

pub trait Strided {
    /// Stride in number of [`u8`] bytes.
    fn stride(&self) -> isize;

    /// Stride in number of [`BitDepth::Pixel`]s.
    fn pixel_stride<BD: BitDepth>(&self) -> isize {
        BD::pxstride(self.stride())
    }
}

impl<'a, S: Strided> Strided for &'a S {
    fn stride(&self) -> isize {
        (*self).stride()
    }
}

#[derive(Clone, Copy)]
pub struct WithStride<T> {
    pub buf: T,
    pub stride: isize,
}

impl<T> Strided for WithStride<T> {
    fn stride(&self) -> isize {
        self.stride
    }
}

impl<T> Deref for WithStride<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}

impl<T> DerefMut for WithStride<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buf
    }
}
