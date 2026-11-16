use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::hint::assert_unchecked;

use zerocopy::{AsBytes, FromBytes, FromZeroes};

use crate::const_fn::const_for;
use crate::enum_map::DefaultValue;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, FromZeroes, FromBytes, AsBytes)]
#[repr(transparent)]
pub struct InRange<T, const MIN: i128, const MAX: i128>(T);

impl<T, const MIN: i128, const MAX: i128> InRange<T, MIN, MAX>
where
    T: TryFrom<i128, Error: Debug>,
{
    pub fn min() -> Self {
        Self(MIN.try_into().unwrap())
    }

    pub fn max() -> Self {
        Self(MAX.try_into().unwrap())
    }
}

impl<T, const MIN: i128, const MAX: i128> InRange<T, MIN, MAX>
where
    T: TryFrom<i128, Error: Debug> + PartialEq + Eq + PartialOrd + Ord,
{
    fn in_bounds(&self) -> bool {
        *self >= Self::min() && *self <= Self::max()
    }

    pub fn new(value: T) -> Option<Self> {
        let this = Self(value);
        if this.in_bounds() {
            Some(this)
        } else {
            None
        }
    }

    pub fn get(self) -> T {
        // SAFETY: Checked in `Self::new`.
        unsafe { assert_unchecked(self.in_bounds()) };
        self.0
    }
}

impl<T, const MIN: i128, const MAX: i128> Default for InRange<T, MIN, MAX>
where
    T: TryFrom<i128, Error: Debug>,
{
    fn default() -> Self {
        Self::min()
    }
}

impl<T, const MIN: i128, const MAX: i128> Display for InRange<T, MIN, MAX>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

macro_rules! impl_const_new {
    ($T:ty) => {
        impl<const MIN: i128, const MAX: i128> DefaultValue for InRange<$T, MIN, MAX> {
            const DEFAULT: Self = Self(0);
        }

        impl<const MIN: i128, const MAX: i128> InRange<$T, MIN, MAX> {
            #[allow(unused)]
            pub const fn const_new(value: $T) -> Self {
                assert!(value as i128 >= MIN && value as i128 <= MAX);
                Self(value)
            }

            #[allow(unused)]
            pub const fn new_array<const N: usize>(a: [$T; N]) -> [Self; N] {
                let mut b = [DefaultValue::DEFAULT; N];
                const_for!(i in 0..N => {
                    b[i] = Self::const_new(a[i]);
                });
                b
            }
        }
    };
}

impl_const_new!(u8);
impl_const_new!(u16);
impl_const_new!(u32);
impl_const_new!(u64);
impl_const_new!(u128);

impl_const_new!(i8);
impl_const_new!(i16);
impl_const_new!(i32);
impl_const_new!(i64);
impl_const_new!(i128);
