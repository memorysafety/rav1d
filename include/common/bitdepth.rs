use crate::include::common::intops::clip;
use crate::src::align::Align16;
use crate::src::align::Align8;
use crate::src::align::ArrayDefault;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_void;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Shr;
use zerocopy::AsBytes;
use zerocopy::FromBytes;

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

impl BPC {
    pub fn from_bitdepth_max(bitdepth_max: c_int) -> Self {
        if bitdepth_max == BitDepth8::new(()).bitdepth_max().into() {
            Self::BPC8
        } else {
            Self::BPC16
        }
    }

    pub const fn bitdepth(&self) -> u8 {
        match self {
            Self::BPC8 => 8,
            Self::BPC16 => 16,
        }
    }

    /// Converts a value in bytes to a value in pixel units.
    ///
    /// `T` is generally meant to be `usize` or `isize`.
    pub fn pxstride<T>(&self, n: T) -> T
    where
        T: Copy + Eq + From<u8> + Div<Output = T> + Rem<Output = T>,
    {
        let scale = (self.bitdepth() / 8).into();
        debug_assert!(n % scale == 0.into());
        n / scale
    }

    /// Converts a value in bytes to a value in coef units.
    ///
    /// `T` is generally meant to be `usize` or `isize`.
    pub fn coef_stride<T>(&self, n: T) -> T
    where
        T: Copy + Eq + From<u8> + Div<Output = T> + Rem<Output = T>,
    {
        let scale = (self.bitdepth() / 4).into();
        debug_assert!(n % scale == 0.into());
        n / scale
    }
}

pub trait BitDepth: Clone + Copy {
    const BPC: BPC;
    const BITDEPTH: u8 = Self::BPC.bitdepth();

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
        + FromPrimitive<u16>
        + FromPrimitive<c_int>
        + FromPrimitive<c_uint>
        + ToPrimitive<i16>
        + ToPrimitive<c_int>
        + ToPrimitive<c_uint>
        + FromBytes
        + AsBytes;

    type AlignPixelX8: Copy;

    type Coef: Copy
        + From<i16>
        + Into<i32>
        + FromPrimitive<i16>
        + FromPrimitive<u16>
        + FromPrimitive<c_int>
        + FromPrimitive<c_uint>
        + ToPrimitive<c_int>
        + ToPrimitive<c_uint>
        + Add<Output = Self::Coef>
        + FromBytes
        + AsBytes
        + Display;

    type Entry: Copy
        + Default
        + ArrayDefault
        + FromPrimitive<i16>
        + FromPrimitive<c_int>
        + ToPrimitive<c_int>;

    type Scaling: AsRef<[u8]> + AsMut<[u8]> + ArrayDefault + Copy;
    const SCALING_BITS: usize;
    const SCALING_SIZE: usize = 1 << Self::SCALING_BITS;

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
        T: Copy + Ord + TryInto<Self::Pixel> + ToPrimitive<Self::Pixel>,
        Self::Pixel: Into<T>,
    {
        clip(pixel, 0.into(), self.bitdepth_max())
    }

    /// `T` is generally meant to be `usize` or `isize`.
    fn pxstride<T>(n: T) -> T
    where
        T: Copy + Eq + TryFrom<usize> + From<u8> + Div<Output = T> + Rem<Output = T>,
    {
        Self::BPC.pxstride(n)
    }

    fn bitdepth(&self) -> u8;

    fn bitdepth_max(&self) -> Self::Pixel;

    fn get_intermediate_bits(&self) -> u8;

    const PREP_BIAS: i16;

    fn sub_prep_bias(pixel: i32) -> i16 {
        (pixel - i32::from(Self::PREP_BIAS)) as i16
    }
}

#[derive(Clone, Copy)]
pub struct BitDepth8 {
    #[allow(dead_code)] // For parity with [`BitDepth16`], where it is used.
    bitdepth_max: <Self as BitDepth>::BitDepthMax,
}

impl BitDepth for BitDepth8 {
    const BPC: BPC = BPC::BPC8;

    type Pixel = u8;

    type AlignPixelX8 = Align8<[Self::Pixel; 0]>;

    type Coef = i16;

    type Entry = i8;

    type Scaling = [u8; Self::SCALING_SIZE];
    const SCALING_BITS: usize = 8;

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
    const BPC: BPC = BPC::BPC16;

    type Pixel = u16;

    type AlignPixelX8 = Align16<[Self::Pixel; 0]>;

    type Coef = i32;

    type Entry = i16;

    type Scaling = [u8; Self::SCALING_SIZE];
    const SCALING_BITS: usize = 12;

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

    fn bitdepth(&self) -> u8 {
        (Self::Pixel::BITS - self.bitdepth_max.leading_zeros()) as u8
    }

    fn bitdepth_max(&self) -> Self::Pixel {
        self.bitdepth_max
    }

    /// - 4 for 10 bits/component.
    /// - 2 for 12 bits/component.
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

/// Select and declare a [`BitDepth`]-dependent `extern "C" fn`.
///
/// That is, it statically selects which [`BitDepth`] `fn`
/// (i.e., `bpc8` or `bpc16`) to return based on `$BD:ty`,
/// declares it inline* with `$decl_fn:path`, and then returns it.
///
/// # Args
///
/// * `$decl_fn:path` (optional):
///     A path to a macro that, given a `fn $fn_name:ident`,
///     declares and returns an `extern "C" fn`
///     with the appropriate signature for this `fn`.
///     This should usually be `mod::decl_fn`,
///     where the `mod` is defined by [`wrap_fn_ptr!`],
///     but it doesn't have to be.
///
///     \* If omitted, this defaults to [`fn_identity`],
///     which returns the `fn` given without declaring one inline.
///     This should be used when the `fn` you are selecting
///     is already declared elsewhere.
///
/// * `$BD:ty`:
///     A `<BD: `[`BitDepth`]`>` generic type parameter.
///     [`BPC::BPC8`] results in `bpc8` and
///     [`BPC::BPC16`] results in `bpc16`.
///
/// * `$name:ident`:
///     The inner name of the asm `fn` to be declared and evaluated to.
///     This excludes the `dav1d_` prefix and the `_bpc{8,16}_$asm` suffix.
///
/// * `$asm:ident`:
///     The asm variant the asm `fn` is named with.
///     The possible values correspond to the [`CpuFlags`]:
///     * `x86`, `x86_64`:
///         * [`sse2`]
///         * [`ssse3`]
///         * [`sse41`]
///     * `x86_64`:
///         * [`avx2`]
///         * [`avx512icl`]
///     * `arm`, `aarch64`:
///         * [`neon`]
///
/// [`wrap_fn_ptr!`]: crate::src::wrap_fn_ptr::wrap_fn_ptr
/// [`CpuFlags`]: crate::src::cpu::CpuFlags
/// [`sse2`]: crate::src::cpu::CpuFlags::SSE2
/// [`sse41`]: crate::src::cpu::CpuFlags::SSE41
/// [`ssse3`]: crate::src::cpu::CpuFlags::SSSE3
/// [`avx2`]: crate::src::cpu::CpuFlags::AVX2
/// [`avx512icl`]: crate::src::cpu::CpuFlags::AVX512ICL
/// [`neon`]: crate::src::cpu::CpuFlags::NEON
#[cfg(all(
    feature = "asm",
    not(any(target_arch = "riscv64", target_arch = "riscv32"))
))]
macro_rules! bd_fn {
    ($decl_fn:path, $BD:ty, $name:ident, $asm:ident) => {{
        use paste::paste;
        use $crate::include::common::bitdepth::BPC;

        paste! {
            match $BD::BPC {
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

/// Select and declare a [`BitDepth`]-dependent `extern "C" fn`.
///
/// Similar to [`bd_fn!`] except that it selects which [`BitDepth`] `fn`
/// based on `$bpc:literal bpc` instead of `$BD:ty`.
#[cfg(all(
    feature = "asm",
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")
))]
macro_rules! bpc_fn {
    ($bpc:literal bpc, $name:ident, $asm:ident) => {{
        use $crate::include::common::bitdepth::fn_identity;

        bpc_fn!(fn_identity, $bpc bpc, $name, $asm)
    }};

    ($decl_fn:path, $bpc:literal bpc, $name:ident, $asm:ident) => {{
        use paste::paste;

        paste! {
            $decl_fn!(fn [<dav1d_ $name _ $bpc bpc_ $asm>])
        }
    }};
}

#[allow(unused)]
macro_rules! fn_identity {
    (fn $name:ident) => {
        $name
    };
}

#[cfg(all(
    feature = "asm",
    not(any(target_arch = "riscv64", target_arch = "riscv32"))
))]
pub(crate) use bd_fn;

#[cfg(all(
    feature = "asm",
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")
))]
pub(crate) use bpc_fn;

#[allow(unused)]
pub(crate) use fn_identity;
