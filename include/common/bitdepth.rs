use std::ffi::{c_int, c_uint};
use std::fmt::{self, Display, Formatter};

use crate::include::common::intops::clip;

pub trait FromPrimitive<T> {
    fn from_prim(t: T) -> Self;
}

pub trait ToPrimitive<T> {
    fn to_prim(self) -> T;
}

pub trait AsPrimitive {
    fn as_<T>(self) -> T
    where
        Self: ToPrimitive<T>;
}

impl<T, U> ToPrimitive<U> for T
where
    U: FromPrimitive<T>,
{
    fn to_prim(self) -> U {
        FromPrimitive::from_prim(self)
    }
}

impl<U> AsPrimitive for U {
    fn as_<T>(self) -> T
    where
        Self: ToPrimitive<T>,
    {
        self.to_prim()
    }
}

macro_rules! impl_FromPrimitive {
    ($T:ty => $U:ty) => {
        impl FromPrimitive<$T> for $U {
            fn from_prim(t: $T) -> $U {
                t as $U
            }
        }
    };
    ($T:ty => {$($U:ty),*}) => {
        $(impl_FromPrimitive!($T => $U);)*
    };
    ($T:ty => {$($U:ty),*, ...}) => {
        $(impl_FromPrimitive!($T => $U);)*
        impl_FromPrimitive!($T => {u8, u16, u32, u64, u128, usize});
        impl_FromPrimitive!($T => {i8, i16, i32, i64, i128, isize});
        impl_FromPrimitive!($T => {f32, f64});
    };
}

impl_FromPrimitive!(u8 => {char, ...});
impl_FromPrimitive!(u16 => {, ...});
impl_FromPrimitive!(u32 => {, ...});
impl_FromPrimitive!(u64 => {, ...});
impl_FromPrimitive!(u128 => {, ...});
impl_FromPrimitive!(usize => {, ...});

impl_FromPrimitive!(i8 => {, ...});
impl_FromPrimitive!(i16 => {, ...});
impl_FromPrimitive!(i32 => {, ...});
impl_FromPrimitive!(i64 => {, ...});
impl_FromPrimitive!(i128 => {, ...});
impl_FromPrimitive!(isize => {, ...});

impl_FromPrimitive!(f32 => {, ...});
impl_FromPrimitive!(f64 => {, ...});

pub trait BitDepth: Clone + Copy {
    const BITDEPTH: u8;

    type Pixel: Copy
        + Ord
        + From<u8>
        + Into<i32>
        + TryFrom<i32>
        + FromPrimitive<c_int>
        + FromPrimitive<c_uint>
        + ToPrimitive<c_int>
        + ToPrimitive<c_uint>;

    type Coef: Copy
        + FromPrimitive<c_int>
        + FromPrimitive<c_uint>
        + ToPrimitive<c_int>
        + ToPrimitive<c_uint>;

    type BitDepthMax;

    type DisplayPixel: Display;

    fn new(bitdepth_max: Self::BitDepthMax) -> Self;

    fn from_c(bitdepth_max: libc::c_int) -> Self;

    fn pixel_copy(dest: &mut [Self::Pixel], src: &[Self::Pixel], n: usize) {
        dest[..n].copy_from_slice(&src[..n]);
    }

    fn pixel_set(dest: &mut [Self::Pixel], val: Self::Pixel, n: usize) {
        dest[..n].fill(val);
    }

    fn display(pixel: Self::Pixel) -> Self::DisplayPixel;

    fn iclip_pixel<T>(&self, pixel: T) -> Self::Pixel
    where
        T: Copy + Ord + TryInto<Self::Pixel>,
        Self::Pixel: Into<T>,
    {
        clip(pixel, 0.into(), self.bitdepth_max())
    }

    fn pxstride(n: usize) -> usize;

    fn bitdepth(&self) -> u8;

    fn bitdepth_max(&self) -> Self::Pixel;

    fn get_intermediate_bits(&self) -> u8;

    const PREP_BIAS: i16;
}

#[derive(Clone, Copy)]
pub struct BitDepth8 {
    bitdepth_max: <Self as BitDepth>::BitDepthMax,
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

    fn from_c(_bitdepth_max: libc::c_int) -> Self {
        Self::new(())
    }

    fn display(pixel: Self::Pixel) -> Self::DisplayPixel {
        DisplayPixel8(pixel)
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

#[derive(Clone, Copy)]
pub struct BitDepth16 {
    bitdepth_max: <Self as BitDepth>::BitDepthMax,
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

    fn from_c(bitdepth_max: libc::c_int) -> Self {
        Self::new(bitdepth_max as Self::BitDepthMax)
    }

    fn display(pixel: Self::Pixel) -> Self::DisplayPixel {
        DisplayPixel16(pixel)
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

pub struct DisplayPixel8(<BitDepth8 as BitDepth>::Pixel);

impl Display for DisplayPixel8 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:02x}", self.0)
    }
}

pub struct DisplayPixel16(<BitDepth16 as BitDepth>::Pixel);

impl Display for DisplayPixel16 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:03x}", self.0)
    }
}
