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

    fn display(pixel: Self::Pixel) -> Self::DisplayPixel;

    fn iclip_pixel(&self, pixel: Self::Pixel) -> Self::Pixel;

    fn pxstride(n: usize) -> usize;

    fn bitdepth(&self) -> u8;

    fn bitdepth_max(&self) -> Self::Pixel;

    fn get_intermediate_bits(&self) -> u8;

    const PREP_BIAS: i16;
}

pub struct BitDepth8 {
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

    fn display(pixel: Self::Pixel) -> Self::DisplayPixel {
        DisplayPixel8(pixel)
    }

    fn iclip_pixel(&self, pixel: Self::Pixel) -> Self::Pixel {
        clip_u8(pixel)
    }

    fn pxstride(n: usize) -> usize {
        n
    }

    fn bitdepth(&self) -> u8 {
        Self::BITDEPTH
    }

    fn bitdepth_max(&self) -> Self::Pixel {
        ((1usize << Self::BITDEPTH) - 1) as Self::Pixel
    }

    fn get_intermediate_bits(&self) -> u8 {
        4
    }

    /// Output in interval `[-5132, 9212]`; fits in [`i16`] as is.
    const PREP_BIAS: i16 = 0;
}
pub struct BitDepth16 {
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

    fn display(pixel: Self::Pixel) -> Self::DisplayPixel {
        DisplayPixel16(pixel)
    }

    fn iclip_pixel(&self, pixel: Self::Pixel) -> Self::Pixel {
        clip(pixel, 0, self.bitdepth_max)
    }

    fn pxstride(n: usize) -> usize {
        debug_assert!(n & 1 == 0);
        n >> 1
    }

    fn bitdepth(&self) -> u8 {
        (Self::Pixel::BITS - self.bitdepth_max.leading_zeros()) as u8
    }

    fn bitdepth_max(&self) -> Self::Pixel {
        self.bitdepth_max
    }

    /// 4 for 10 bits/component.  
    /// 2 for 12 bits/component.
    fn get_intermediate_bits(&self) -> u8 {
        14 - self.bitdepth()
    }

    /// Output in interval `[-20588, 36956]` (10-bit), `[-20602, 36983]` (12-bit)
    /// Subtract a bias to ensure the output fits in [`i16`].
    const PREP_BIAS: i16 = 8192;
}

pub struct DisplayPixel8(u8);

impl Display for DisplayPixel8 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:02x}", self.0)
    }
}

pub struct DisplayPixel16(u16);

impl Display for DisplayPixel16 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:03x}", self.0)
    }
}
