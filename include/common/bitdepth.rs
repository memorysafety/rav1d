use crate::include::common::intops::clip;
use crate::src::align::ArrayDefault;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_void;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Shr;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BPC {
    BPC8,
    BPC16,
}

pub trait BitDepth: Clone + Copy {
    const BPC: BPC;
    const BITDEPTH: u8;

    type Pixel: Copy
        + Ord
        + Add<Output = Self::Pixel>
        + Mul<Output = Self::Pixel>
        + Shr<u8, Output = Self::Pixel>
        + From<u8>
        + Into<u16>
        + Into<usize>
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
        + ToPrimitive<c_uint>
        + Add<Output = Self::Coef>
        + Display;

    type Entry: Copy
        + Default
        + ArrayDefault
        + FromPrimitive<i16>
        + FromPrimitive<c_int>
        + ToPrimitive<c_int>;

    type Scaling: AsRef<[u8]> + AsMut<[u8]> + ArrayDefault + Copy;
    const SCALING_SIZE: usize;

    type BitDepthMax;

    type DisplayPixel: Display;

    fn new(bitdepth_max: Self::BitDepthMax) -> Self;

    /// While [`BitDepth::new`] is the implementation specific way to
    /// construct a [`BitDepth`], [`BitDepth::from_c`] is a uniform way to
    /// construct a [`BitDepth`] from its C representation, a [`c_int`].
    ///
    /// Since [`BitDepth`]-dependent `fn` ptr types use type erasure, they
    /// always pass the `bitdepth_max` last argument (even for `8bpc``fn`s
    /// where it's superfluous (it's constant)), so we need to convert from
    /// that `bitdepth_max: c_int` arg back to a [`BitDepth`].
    fn from_c(bitdepth_max: c_int) -> Self;

    /// The opposite of [`BitDepth::from_c`].
    fn into_c(self) -> c_int {
        self.bitdepth_max().into()
    }

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

    unsafe fn select<T>(bd: &BitDepthUnion<T>) -> &T::T<Self>
    where
        T: BitDepthDependentType,
        T::T<BitDepth8>: Copy,
        T::T<BitDepth16>: Copy;

    unsafe fn select_mut<T>(bd: &mut BitDepthUnion<T>) -> &mut T::T<Self>
    where
        T: BitDepthDependentType,
        T::T<BitDepth8>: Copy,
        T::T<BitDepth16>: Copy;

    unsafe fn select_into<T>(bd: BitDepthUnion<T>) -> T::T<Self>
    where
        T: BitDepthDependentType,
        T::T<BitDepth8>: Copy,
        T::T<BitDepth16>: Copy;
}

#[derive(Clone, Copy)]
pub struct BitDepth8 {
    #[allow(dead_code)] // For parity with [`BitDepth16`], where it is used.
    bitdepth_max: <Self as BitDepth>::BitDepthMax,
}

impl BitDepth for BitDepth8 {
    const BPC: BPC = BPC::BPC8;
    const BITDEPTH: u8 = 8;

    type Pixel = u8;

    type Coef = i16;

    type Entry = i8;

    type Scaling = [u8; Self::SCALING_SIZE];
    const SCALING_SIZE: usize = 1 << 8;

    type BitDepthMax = ();

    type DisplayPixel = DisplayPixel8;

    fn new(bitdepth_max: Self::BitDepthMax) -> Self {
        Self { bitdepth_max }
    }

    fn from_c(_bitdepth_max: c_int) -> Self {
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

    unsafe fn select<T>(bd: &BitDepthUnion<T>) -> &T::T<Self>
    where
        T: BitDepthDependentType,
        T::T<BitDepth8>: Copy,
        T::T<BitDepth16>: Copy,
    {
        &bd.bpc8
    }

    unsafe fn select_mut<T>(bd: &mut BitDepthUnion<T>) -> &mut T::T<Self>
    where
        T: BitDepthDependentType,
        T::T<BitDepth8>: Copy,
        T::T<BitDepth16>: Copy,
    {
        &mut bd.bpc8
    }

    unsafe fn select_into<T>(bd: BitDepthUnion<T>) -> T::T<Self>
    where
        T: BitDepthDependentType,
        T::T<BitDepth8>: Copy,
        T::T<BitDepth16>: Copy,
    {
        bd.bpc8
    }
}

#[derive(Clone, Copy)]
pub struct BitDepth16 {
    bitdepth_max: <Self as BitDepth>::BitDepthMax,
}

impl BitDepth for BitDepth16 {
    const BPC: BPC = BPC::BPC16;
    const BITDEPTH: u8 = 16;

    type Pixel = u16;

    type Coef = i32;

    type Entry = i16;

    type Scaling = [u8; Self::SCALING_SIZE];
    const SCALING_SIZE: usize = 1 << 12;

    type BitDepthMax = Self::Pixel;

    type DisplayPixel = DisplayPixel16;

    fn new(bitdepth_max: Self::BitDepthMax) -> Self {
        Self { bitdepth_max }
    }

    fn from_c(bitdepth_max: c_int) -> Self {
        Self::new(bitdepth_max.as_())
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

    unsafe fn select<T>(bd: &BitDepthUnion<T>) -> &T::T<Self>
    where
        T: BitDepthDependentType,
        T::T<BitDepth8>: Copy,
        T::T<BitDepth16>: Copy,
    {
        &bd.bpc16
    }

    unsafe fn select_mut<T>(bd: &mut BitDepthUnion<T>) -> &mut T::T<Self>
    where
        T: BitDepthDependentType,
        T::T<BitDepth8>: Copy,
        T::T<BitDepth16>: Copy,
    {
        &mut bd.bpc16
    }

    unsafe fn select_into<T>(bd: BitDepthUnion<T>) -> T::T<Self>
    where
        T: BitDepthDependentType,
        T::T<BitDepth8>: Copy,
        T::T<BitDepth16>: Copy,
    {
        bd.bpc16
    }
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

#[repr(transparent)]
pub struct DynPixel(c_void);

#[repr(transparent)]
pub struct DynCoef(c_void);

#[repr(transparent)]
pub struct DynEntry(c_void);

#[repr(transparent)]
pub struct DynScaling([u8; 1]);

pub type LeftPixelRow<Pixel> = [Pixel; 4];
pub type LeftPixelRow2px<Pixel> = [Pixel; 2];

pub trait BitDepthDependentType {
    type T<BD: BitDepth>;
}

// #[derive(Clone, Copy)]
pub union BitDepthUnion<T: BitDepthDependentType>
where
    T::T<BitDepth8>: Copy,
    T::T<BitDepth16>: Copy,
{
    bpc8: T::T<BitDepth8>,
    bpc16: T::T<BitDepth16>,
}

// Implemented manually to not require `T: Copy`.
impl<T: BitDepthDependentType> Copy for BitDepthUnion<T>
where
    T::T<BitDepth8>: Copy,
    T::T<BitDepth16>: Copy,
{
}

// Implemented manually to not require `T: Clone`.
impl<T: BitDepthDependentType> Clone for BitDepthUnion<T>
where
    T::T<BitDepth8>: Copy,
    T::T<BitDepth16>: Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

#[cfg(feature = "asm")]
macro_rules! bd_fn {
    ($decl_fn:path, $BD:ty, $name:ident, $asm:ident) => {{
        use paste::paste;
        use $crate::include::common::bitdepth::BPC;

        paste! {
            match BD::BPC {
                BPC::BPC8 => $decl_fn!(fn [<dav1d_ $name _8bpc_ $asm>]),
                BPC::BPC16 => $decl_fn!(fn [<dav1d_ $name _16bpc_ $asm>]),
            }
        }
    }};

    ($BD:ty, $name:ident, $asm:ident) => {{
        use $crate::include::common::bitdepth::fn_identity;

        bd_fn!(fn_identity, $BD, $name, $asm)
    }};
}

#[cfg(feature = "asm")]
macro_rules! fn_identity {
    (fn $name:ident) => {
        $name
    };
}

#[cfg(feature = "asm")]
pub(crate) use bd_fn;

#[cfg(feature = "asm")]
pub(crate) use fn_identity;
