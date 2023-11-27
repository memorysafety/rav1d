use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::ToPrimitive;
use crate::src::enum_map::DefaultValue;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

pub trait Integer:
    Copy + Clone + PartialEq + Eq + PartialOrd + Ord + Default + Debug + Display
{
    type UnderlyingType;

    const BITS: usize;
    const SIGNED: bool;
    const ZERO: Self;
    const ONE: Self;
    const MIN: Self;
    const MAX: Self;
}

macro_rules! impl_primitive_integer {
    ($T:ty) => {
        impl Integer for $T {
            type UnderlyingType = $T;

            const BITS: usize = <$T>::BITS as usize;
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const MIN: Self = <$T>::MIN;
            const MAX: Self = <$T>::MAX;
            const SIGNED: bool = Self::MIN != Self::ZERO;
        }
    };

    ($($T:ty),+) => {
        $(impl_primitive_integer!($T);)+
    }
}

impl_primitive_integer!(u8, u16, u32, u64, u128);
impl_primitive_integer!(i8, i16, i32, i64, i128);

impl<T: Integer> DefaultValue for T {
    const DEFAULT: Self = Self::ZERO;
}

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Default)]
pub struct BitInt<Int, const BITS: usize, const SIGNED: bool> {
    value: Int,
}

impl<Int: Integer, const BITS: usize, const SIGNED: bool> BitInt<Int, BITS, SIGNED> {
    pub const BITS: usize = {
        assert!(BITS > 0);
        assert!(BITS <= Int::BITS);
        BITS
    };

    pub const EXTRA_BITS: usize = Int::BITS - BITS;

    pub const SIGNED: bool = {
        assert!(SIGNED == Int::SIGNED);
        SIGNED
    };

    /// # Safety
    ///
    /// `value` must be in the range `[`[`Self::MIN`]`, `[`Self::MAX`]`]`.
    /// Alternatively, only the lower [`Self::BITS`] must be set,
    /// and all of the upper [`Self::EXTRA_BITS`] must be 0.
    #[inline]
    pub const unsafe fn unchecked_new(value: Int) -> Self {
        Self { value }
    }

    /// Get the underlying integer value.
    #[inline]
    pub const fn value(self) -> Int {
        self.value
    }

    // Safety: Always valid since `BITS > 0`.
    pub const ZERO: Self = unsafe { Self::unchecked_new(Int::ZERO) };

    // Safety: Always valid since `BITS > 0`.
    pub const ONE: Self = unsafe { Self::unchecked_new(Int::ONE) };

    fn fmt_suffix(f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{}", if SIGNED { "i" } else { "u" }, BITS)
    }
}

impl<Int: Integer, const BITS: usize, const SIGNED: bool> Display for BitInt<Int, BITS, SIGNED> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.value, f)?;
        Self::fmt_suffix(f)?;
        Ok(())
    }
}

impl<Int: Integer, const BITS: usize, const SIGNED: bool> Debug for BitInt<Int, BITS, SIGNED> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.value, f)?;
        Self::fmt_suffix(f)?;
        Ok(())
    }
}

macro_rules! impl_bit_integer {
    ($Int:ty) => {
        impl<const BITS: usize, const SIGNED: bool> BitInt<$Int, BITS, SIGNED> {
            // Safety: We shift out all the extra bits.
            pub const MIN: Self =
                unsafe { Self::unchecked_new(<$Int as Integer>::MIN >> Self::EXTRA_BITS) };

            // Safety: We shift out all the extra bits.
            pub const MAX: Self =
                unsafe { Self::unchecked_new(<$Int as Integer>::MAX >> Self::EXTRA_BITS) };
        }

        impl<const BITS: usize, const SIGNED: bool> Integer for BitInt<$Int, BITS, SIGNED> {
            type UnderlyingType = $Int;

            const BITS: usize = Self::BITS;
            const SIGNED: bool = Self::SIGNED;
            const ZERO: Self = Self::ZERO;
            const ONE: Self = Self::ONE;
            const MIN: Self = Self::MIN;
            const MAX: Self = Self::MAX;
        }

        impl<const BITS: usize, const SIGNED: bool> BitInt<$Int, BITS, SIGNED> {
            /// Create a new [`Self`], panicking if the `value` is out of bounds.
            #[inline]
            pub const fn checked_new(value: $Int) -> Self {
                assert!(value >= Self::MIN.value);
                assert!(value <= Self::MAX.value);
                // Safety: Explicitly checked bounds above.
                unsafe { Self::unchecked_new(value) }
            }

            /// Try to create a new [`Self`], returning `None` if the `value` is out of bounds.
            #[inline]
            pub const fn try_new(value: $Int) -> Option<Self> {
                if value >= Self::MIN.value && value <= Self::MAX.value {
                    // Safety: Explicitly checked bounds above.
                    Some(unsafe { Self::unchecked_new(value) })
                } else {
                    None
                }
            }

            /// Create a new [`Self`] by truncating to only the lower [`Self::BITS`].
            ///
            /// This should always be preferred over [`Self::unchecked_new`]
            /// as it is safe and the truncation is extremely cheap
            /// (2 bit shifts if [`Self::SIGNED`], and 1 bit and if `!`[`Self::SIGNED`]).
            #[inline]
            pub const fn truncating_new(value: $Int) -> Self {
                // Note that this is identical to `& Self::MAX.value` if `!SIGNED`,
                // but this way is also correct for `SIGNED`.
                // Safety: All extra bits are cleared.
                unsafe { Self::unchecked_new(value << Self::EXTRA_BITS >> Self::EXTRA_BITS) }
            }

            /// Get the underlying integer value,
            /// but also redundantly truncate to the lower [`Self::BITS`].
            ///
            /// This is identical to [`Self::value`] except to the optimizer.
            /// The [`Self::value`] field is guaranteed to
            /// only have its lower [`Self::BITS`] bits set,
            /// but the compiler likely won't be able to see across `fn`s to know that.
            /// By doing the truncation again, which is extremely cheap
            /// (2 bit shifts if [`Self::SIGNED`], and 1 bit and if `!`[`Self::SIGNED`])
            /// and sometimes even a no-op compared to a `mov`,
            /// we tell the optimizer which bits cannot be set.
            /// Thus, an array index into `[T; {1 << BITS}]` won't need a bounds check.
            #[inline]
            pub const fn truncated_value(self) -> $Int {
                self.value() << Self::EXTRA_BITS >> Self::EXTRA_BITS
            }

            #[inline]
            pub const fn widen<const NEW_BITS: usize>(self) -> BitInt<$Int, NEW_BITS, SIGNED> {
                // We use the associated type to run the const `assert!`.
                assert!(BitInt::<$Int, NEW_BITS, SIGNED>::BITS >= BITS);
                // Safety: Above assert.
                unsafe { BitInt::<$Int, NEW_BITS, SIGNED>::unchecked_new(self.value()) }
            }

            /// # Safety
            /// 
            /// See [`Self::unchecked_new`].
            #[inline]
            pub const unsafe fn unchecked_narrow<const NEW_BITS: usize>(self) -> BitInt<$Int, NEW_BITS, SIGNED> {
                // We use the associated type to run the const `assert!`.
                assert!(BitInt::<$Int, NEW_BITS, SIGNED>::BITS <= BITS);
                BitInt::<$Int, NEW_BITS, SIGNED>::unchecked_new(self.value())
            }

            #[inline]
            pub const fn checked_narrow<const NEW_BITS: usize>(self) -> BitInt<$Int, NEW_BITS, SIGNED> {
                // We use the associated type to run the const `assert!`.
                assert!(BitInt::<$Int, NEW_BITS, SIGNED>::BITS <= BITS);
                BitInt::<$Int, NEW_BITS, SIGNED>::checked_new(self.value())
            }

            #[inline]
            pub const fn truncating_narrow<const NEW_BITS: usize>(self) -> BitInt<$Int, NEW_BITS, SIGNED> {
                // We use the associated type to run the const `assert!`.
                assert!(BitInt::<$Int, NEW_BITS, SIGNED>::BITS <= BITS);
                BitInt::<$Int, NEW_BITS, SIGNED>::truncating_new(self.value())
            }

            /// # Safety
            /// 
            /// See [`Self::unchecked_new`].
            #[inline]
            pub unsafe fn unchecked_map(self, f: impl Fn($Int) -> $Int) -> Self {
                Self::unchecked_new(f(self.value()))
            }

            #[inline]
            pub fn checked_map(self, f: impl Fn($Int) -> $Int) -> Self {
                Self::checked_new(f(self.value()))
            }

            #[inline]
            pub fn truncating_map(self, f: impl Fn($Int) -> $Int) -> Self {
                Self::truncating_new(f(self.value()))
            }
        }

        impl<const SIGNED: bool> BitInt<$Int, 1, SIGNED>
        {
            #[inline]
            pub const fn from_bool(value: bool) -> Self {
                Self::checked_new(value as $Int)
            }

            #[inline]
            pub const fn into_bool(self) -> bool {
                self.value != Self::ZERO.value
            }
        }

        impl<const SIGNED: bool> From<bool> for BitInt<$Int, 1, SIGNED>
        {
            #[inline]
            fn from(value: bool) -> Self {
                Self::from_bool(value)
            }
        }

        impl<const SIGNED: bool> Into<bool> for BitInt<$Int, 1, SIGNED>
        {
            #[inline]
            fn into(self) -> bool {
                self.into_bool()
            }
        }
    };

    ($($T:ty),+) => {
        $(impl_bit_integer!($T);)+
    }
}

impl_bit_integer!(u8, u16, u32, u64, u128);
impl_bit_integer!(i8, i16, i32, i64, i128);

impl<Int, const BITS: usize, const SIGNED: bool> BitInt<Int, BITS, SIGNED>
where
    Int: Integer + ToPrimitive<usize>,
    Self: Integer,
{
    /// # Safety
    ///
    /// Return if is guaranteed that an unchecked index
    /// of [`Self`] into `a` will be in bounds.
    #[inline]
    fn guaranteed_in_bounds<T>(self, a: &[T]) -> bool {
        !Self::SIGNED && Self::MAX.value().as_() < a.len()
    }

    #[inline]
    pub fn index_into<T>(self, a: &[T]) -> &T {
        let value = self.value().as_();
        if self.guaranteed_in_bounds(a) {
            // Safety: `0 <= value <= max < a.len()`, so `0 < value < a.len()`,
            // so this must be in bounds.
            unsafe { a.get_unchecked(value) }
        } else {
            &a[value]
        }
    }

    #[inline]
    pub fn index_into_mut<T>(self, a: &mut [T]) -> &mut T {
        let value = self.value().as_();
        if self.guaranteed_in_bounds(a) {
            // Safety: `0 <= value <= max < a.len()`, so `0 < value < a.len()`,
            // so this must be in bounds.
            unsafe { a.get_unchecked_mut(value) }
        } else {
            &mut a[value]
        }
    }
}

pub type BitUInt<Int, const BITS: usize> = BitInt<Int, BITS, false>;
pub type BitSInt<Int, const BITS: usize> = BitInt<Int, BITS, true>;

#[allow(non_camel_case_types)]
pub mod aliases {
    use super::BitSInt;
    use super::BitUInt;

    pub type u1 = BitUInt<u8, 1>;
    pub type u2 = BitUInt<u8, 2>;
    pub type u3 = BitUInt<u8, 3>;
    pub type u4 = BitUInt<u8, 4>;
    pub type u5 = BitUInt<u8, 5>;
    pub type u6 = BitUInt<u8, 6>;
    pub type u7 = BitUInt<u8, 7>;

    pub type u9 = BitUInt<u16, 9>;
    pub type u10 = BitUInt<u16, 10>;
    pub type u11 = BitUInt<u16, 11>;
    pub type u12 = BitUInt<u16, 12>;
    pub type u13 = BitUInt<u16, 13>;
    pub type u14 = BitUInt<u16, 14>;
    pub type u15 = BitUInt<u16, 15>;

    pub type u17 = BitUInt<u32, 17>;
    pub type u18 = BitUInt<u32, 18>;
    pub type u19 = BitUInt<u32, 19>;
    pub type u20 = BitUInt<u32, 20>;
    pub type u21 = BitUInt<u32, 21>;
    pub type u22 = BitUInt<u32, 22>;
    pub type u23 = BitUInt<u32, 23>;
    pub type u24 = BitUInt<u32, 24>;
    pub type u25 = BitUInt<u32, 25>;
    pub type u26 = BitUInt<u32, 26>;
    pub type u27 = BitUInt<u32, 27>;
    pub type u28 = BitUInt<u32, 28>;
    pub type u29 = BitUInt<u32, 29>;
    pub type u30 = BitUInt<u32, 30>;
    pub type u31 = BitUInt<u32, 31>;

    pub type u33 = BitUInt<u64, 33>;
    pub type u34 = BitUInt<u64, 34>;
    pub type u35 = BitUInt<u64, 35>;
    pub type u36 = BitUInt<u64, 36>;
    pub type u37 = BitUInt<u64, 37>;
    pub type u38 = BitUInt<u64, 38>;
    pub type u39 = BitUInt<u64, 39>;
    pub type u40 = BitUInt<u64, 40>;
    pub type u41 = BitUInt<u64, 41>;
    pub type u42 = BitUInt<u64, 42>;
    pub type u43 = BitUInt<u64, 43>;
    pub type u44 = BitUInt<u64, 44>;
    pub type u45 = BitUInt<u64, 45>;
    pub type u46 = BitUInt<u64, 46>;
    pub type u47 = BitUInt<u64, 47>;
    pub type u48 = BitUInt<u64, 48>;
    pub type u49 = BitUInt<u64, 49>;
    pub type u50 = BitUInt<u64, 50>;
    pub type u51 = BitUInt<u64, 51>;
    pub type u52 = BitUInt<u64, 52>;
    pub type u53 = BitUInt<u64, 53>;
    pub type u54 = BitUInt<u64, 54>;
    pub type u55 = BitUInt<u64, 55>;
    pub type u56 = BitUInt<u64, 56>;
    pub type u57 = BitUInt<u64, 57>;
    pub type u58 = BitUInt<u64, 58>;
    pub type u59 = BitUInt<u64, 59>;
    pub type u60 = BitUInt<u64, 60>;
    pub type u61 = BitUInt<u64, 61>;
    pub type u62 = BitUInt<u64, 62>;
    pub type u63 = BitUInt<u64, 63>;

    pub type u65 = BitUInt<u128, 65>;
    pub type u66 = BitUInt<u128, 66>;
    pub type u67 = BitUInt<u128, 67>;
    pub type u68 = BitUInt<u128, 68>;
    pub type u69 = BitUInt<u128, 69>;
    pub type u70 = BitUInt<u128, 70>;
    pub type u71 = BitUInt<u128, 71>;
    pub type u72 = BitUInt<u128, 72>;
    pub type u73 = BitUInt<u128, 73>;
    pub type u74 = BitUInt<u128, 74>;
    pub type u75 = BitUInt<u128, 75>;
    pub type u76 = BitUInt<u128, 76>;
    pub type u77 = BitUInt<u128, 77>;
    pub type u78 = BitUInt<u128, 78>;
    pub type u79 = BitUInt<u128, 79>;
    pub type u80 = BitUInt<u128, 80>;
    pub type u81 = BitUInt<u128, 81>;
    pub type u82 = BitUInt<u128, 82>;
    pub type u83 = BitUInt<u128, 83>;
    pub type u84 = BitUInt<u128, 84>;
    pub type u85 = BitUInt<u128, 85>;
    pub type u86 = BitUInt<u128, 86>;
    pub type u87 = BitUInt<u128, 87>;
    pub type u88 = BitUInt<u128, 88>;
    pub type u89 = BitUInt<u128, 89>;
    pub type u90 = BitUInt<u128, 90>;
    pub type u91 = BitUInt<u128, 91>;
    pub type u92 = BitUInt<u128, 92>;
    pub type u93 = BitUInt<u128, 93>;
    pub type u94 = BitUInt<u128, 94>;
    pub type u95 = BitUInt<u128, 95>;
    pub type u96 = BitUInt<u128, 96>;
    pub type u97 = BitUInt<u128, 97>;
    pub type u98 = BitUInt<u128, 98>;
    pub type u99 = BitUInt<u128, 99>;
    pub type u100 = BitUInt<u128, 100>;
    pub type u101 = BitUInt<u128, 101>;
    pub type u102 = BitUInt<u128, 102>;
    pub type u103 = BitUInt<u128, 103>;
    pub type u104 = BitUInt<u128, 104>;
    pub type u105 = BitUInt<u128, 105>;
    pub type u106 = BitUInt<u128, 106>;
    pub type u107 = BitUInt<u128, 107>;
    pub type u108 = BitUInt<u128, 108>;
    pub type u109 = BitUInt<u128, 109>;
    pub type u110 = BitUInt<u128, 110>;
    pub type u111 = BitUInt<u128, 111>;
    pub type u112 = BitUInt<u128, 112>;
    pub type u113 = BitUInt<u128, 113>;
    pub type u114 = BitUInt<u128, 114>;
    pub type u115 = BitUInt<u128, 115>;
    pub type u116 = BitUInt<u128, 116>;
    pub type u117 = BitUInt<u128, 117>;
    pub type u118 = BitUInt<u128, 118>;
    pub type u119 = BitUInt<u128, 119>;
    pub type u120 = BitUInt<u128, 120>;
    pub type u121 = BitUInt<u128, 121>;
    pub type u122 = BitUInt<u128, 122>;
    pub type u123 = BitUInt<u128, 123>;
    pub type u124 = BitUInt<u128, 124>;
    pub type u125 = BitUInt<u128, 125>;
    pub type u126 = BitUInt<u128, 126>;
    pub type u127 = BitUInt<u128, 127>;

    pub type i1 = BitSInt<i8, 1>;
    pub type i2 = BitSInt<i8, 2>;
    pub type i3 = BitSInt<i8, 3>;
    pub type i4 = BitSInt<i8, 4>;
    pub type i5 = BitSInt<i8, 5>;
    pub type i6 = BitSInt<i8, 6>;
    pub type i7 = BitSInt<i8, 7>;

    pub type i9 = BitSInt<i16, 9>;
    pub type i10 = BitSInt<i16, 10>;
    pub type i11 = BitSInt<i16, 11>;
    pub type i12 = BitSInt<i16, 12>;
    pub type i13 = BitSInt<i16, 13>;
    pub type i14 = BitSInt<i16, 14>;
    pub type i15 = BitSInt<i16, 15>;

    pub type i17 = BitSInt<i32, 17>;
    pub type i18 = BitSInt<i32, 18>;
    pub type i19 = BitSInt<i32, 19>;
    pub type i20 = BitSInt<i32, 20>;
    pub type i21 = BitSInt<i32, 21>;
    pub type i22 = BitSInt<i32, 22>;
    pub type i23 = BitSInt<i32, 23>;
    pub type i24 = BitSInt<i32, 24>;
    pub type i25 = BitSInt<i32, 25>;
    pub type i26 = BitSInt<i32, 26>;
    pub type i27 = BitSInt<i32, 27>;
    pub type i28 = BitSInt<i32, 28>;
    pub type i29 = BitSInt<i32, 29>;
    pub type i30 = BitSInt<i32, 30>;
    pub type i31 = BitSInt<i32, 31>;

    pub type i33 = BitSInt<i64, 33>;
    pub type i34 = BitSInt<i64, 34>;
    pub type i35 = BitSInt<i64, 35>;
    pub type i36 = BitSInt<i64, 36>;
    pub type i37 = BitSInt<i64, 37>;
    pub type i38 = BitSInt<i64, 38>;
    pub type i39 = BitSInt<i64, 39>;
    pub type i40 = BitSInt<i64, 40>;
    pub type i41 = BitSInt<i64, 41>;
    pub type i42 = BitSInt<i64, 42>;
    pub type i43 = BitSInt<i64, 43>;
    pub type i44 = BitSInt<i64, 44>;
    pub type i45 = BitSInt<i64, 45>;
    pub type i46 = BitSInt<i64, 46>;
    pub type i47 = BitSInt<i64, 47>;
    pub type i48 = BitSInt<i64, 48>;
    pub type i49 = BitSInt<i64, 49>;
    pub type i50 = BitSInt<i64, 50>;
    pub type i51 = BitSInt<i64, 51>;
    pub type i52 = BitSInt<i64, 52>;
    pub type i53 = BitSInt<i64, 53>;
    pub type i54 = BitSInt<i64, 54>;
    pub type i55 = BitSInt<i64, 55>;
    pub type i56 = BitSInt<i64, 56>;
    pub type i57 = BitSInt<i64, 57>;
    pub type i58 = BitSInt<i64, 58>;
    pub type i59 = BitSInt<i64, 59>;
    pub type i60 = BitSInt<i64, 60>;
    pub type i61 = BitSInt<i64, 61>;
    pub type i62 = BitSInt<i64, 62>;
    pub type i63 = BitSInt<i64, 63>;

    pub type i65 = BitSInt<i128, 65>;
    pub type i66 = BitSInt<i128, 66>;
    pub type i67 = BitSInt<i128, 67>;
    pub type i68 = BitSInt<i128, 68>;
    pub type i69 = BitSInt<i128, 69>;
    pub type i70 = BitSInt<i128, 70>;
    pub type i71 = BitSInt<i128, 71>;
    pub type i72 = BitSInt<i128, 72>;
    pub type i73 = BitSInt<i128, 73>;
    pub type i74 = BitSInt<i128, 74>;
    pub type i75 = BitSInt<i128, 75>;
    pub type i76 = BitSInt<i128, 76>;
    pub type i77 = BitSInt<i128, 77>;
    pub type i78 = BitSInt<i128, 78>;
    pub type i79 = BitSInt<i128, 79>;
    pub type i80 = BitSInt<i128, 80>;
    pub type i81 = BitSInt<i128, 81>;
    pub type i82 = BitSInt<i128, 82>;
    pub type i83 = BitSInt<i128, 83>;
    pub type i84 = BitSInt<i128, 84>;
    pub type i85 = BitSInt<i128, 85>;
    pub type i86 = BitSInt<i128, 86>;
    pub type i87 = BitSInt<i128, 87>;
    pub type i88 = BitSInt<i128, 88>;
    pub type i89 = BitSInt<i128, 89>;
    pub type i90 = BitSInt<i128, 90>;
    pub type i91 = BitSInt<i128, 91>;
    pub type i92 = BitSInt<i128, 92>;
    pub type i93 = BitSInt<i128, 93>;
    pub type i94 = BitSInt<i128, 94>;
    pub type i95 = BitSInt<i128, 95>;
    pub type i96 = BitSInt<i128, 96>;
    pub type i97 = BitSInt<i128, 97>;
    pub type i98 = BitSInt<i128, 98>;
    pub type i99 = BitSInt<i128, 99>;
    pub type i100 = BitSInt<i128, 100>;
    pub type i101 = BitSInt<i128, 101>;
    pub type i102 = BitSInt<i128, 102>;
    pub type i103 = BitSInt<i128, 103>;
    pub type i104 = BitSInt<i128, 104>;
    pub type i105 = BitSInt<i128, 105>;
    pub type i106 = BitSInt<i128, 106>;
    pub type i107 = BitSInt<i128, 107>;
    pub type i108 = BitSInt<i128, 108>;
    pub type i109 = BitSInt<i128, 109>;
    pub type i110 = BitSInt<i128, 110>;
    pub type i111 = BitSInt<i128, 111>;
    pub type i112 = BitSInt<i128, 112>;
    pub type i113 = BitSInt<i128, 113>;
    pub type i114 = BitSInt<i128, 114>;
    pub type i115 = BitSInt<i128, 115>;
    pub type i116 = BitSInt<i128, 116>;
    pub type i117 = BitSInt<i128, 117>;
    pub type i118 = BitSInt<i128, 118>;
    pub type i119 = BitSInt<i128, 119>;
    pub type i120 = BitSInt<i128, 120>;
    pub type i121 = BitSInt<i128, 121>;
    pub type i122 = BitSInt<i128, 122>;
    pub type i123 = BitSInt<i128, 123>;
    pub type i124 = BitSInt<i128, 124>;
    pub type i125 = BitSInt<i128, 125>;
    pub type i126 = BitSInt<i128, 126>;
    pub type i127 = BitSInt<i128, 127>;
}

pub mod test {
    use crate::src::bit_int::aliases::u2;

    pub fn f1(i: u2, a: &[u8; 4]) -> u8 {
        a[i.truncated_value() as usize]
    }

    pub fn f2(i: u2, a: &[u8; 4]) -> u8 {
        *i.index_into(a)
    }
}
