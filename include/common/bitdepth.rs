use std::fmt::{self, Display, Formatter};

use crate::include::common::intops::clip;
use crate::include::common::intops::clip_u8;

pub trait BitDepth {
    const BITDEPTH: u8;

    type Pixel: Copy;

    type Coef: Copy;

    type BitDepthMax;

    type DisplayPixel: Display;

    fn new(bitdepth_max: Self::BitDepthMax) -> Self;

    fn pixel_copy(dest: &mut [Self::Pixel], src: &[Self::Pixel], n: usize) {
        dest[..n].copy_from_slice(&src[..n]);
    }

    fn pixel_set(dest: &mut [Self::Pixel], val: Self::Pixel, n: usize) {
        dest[..n].fill(val);
    }

    fn iclip_pixel(&self, pixel: Self::Pixel) -> Self::Pixel;

    fn pxstride(n: usize) -> usize;

    fn bitdepth_from_max(&self) -> u8;

    fn bitdepth_max(&self) -> Self::Pixel;
}

struct BitDepth8 {
    bitdepth_max: (),
}

impl BitDepth for BitDepth8 {
    const BITDEPTH: u8 = 8;

    type Pixel = u8;

    type Coef = i16;

    type BitDepthMax = ();

    type DisplayPixel = DisplayPixel8;

    fn new(bitdepth_max: Self::BitDepthMax) -> Self {
        Self { bitdepth_max }
    }

    fn iclip_pixel(&self, pixel: Self::Pixel) -> Self::Pixel {
        clip_u8(pixel)
    }

    fn pxstride(n: usize) -> usize {
        n
    }

    fn bitdepth_from_max(&self) -> u8 {
        Self::BITDEPTH
    }

    fn bitdepth_max(&self) -> Self::Pixel {
        ((1usize << Self::BITDEPTH) - 1) as Self::Pixel
    }
}
struct BitDepth16 {
    bitdepth_max: u16,
}

impl BitDepth for BitDepth16 {
    const BITDEPTH: u8 = 16;

    type Pixel = u16;

    type Coef = i32;

    type BitDepthMax = Self::Pixel;

    type DisplayPixel = DisplayPixel16;

    fn new(bitdepth_max: Self::BitDepthMax) -> Self {
        Self { bitdepth_max }
    }

    fn iclip_pixel(&self, pixel: Self::Pixel) -> Self::Pixel {
        clip(pixel, 0, self.bitdepth_max)
    }

    fn pxstride(n: usize) -> usize {
        debug_assert!(n & 1 == 0);
        n >> 1
    }

    fn bitdepth_from_max(&self) -> u8 {
        (Self::Pixel::BITS - self.bitdepth_max.leading_zeros()) as u8
    }

    fn bitdepth_max(&self) -> Self::Pixel {
        self.bitdepth_max
    }
}

struct DisplayPixel8(u8);

impl Display for DisplayPixel8 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:02x}", self.0)
    }
}

struct DisplayPixel16(u16);

impl Display for DisplayPixel16 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:03x}", self.0)
    }
}
