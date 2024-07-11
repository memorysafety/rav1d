use crate::src::assume::assume;
use crate::src::const_fn::const_for;
use crate::src::enum_map::DefaultValue;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct InRange<T, const MIN: u128, const MAX: u128>(T);

impl<T, const MIN: u128, const MAX: u128> InRange<T, MIN, MAX>
where
    T: TryFrom<u128, Error: Debug>,
{
    pub fn min() -> Self {
        Self(MIN.try_into().unwrap())
    }

    pub fn max() -> Self {
        Self(MAX.try_into().unwrap())
    }
}

impl<T, const MIN: u128, const MAX: u128> InRange<T, MIN, MAX>
where
    T: TryFrom<u128, Error: Debug> + PartialEq + Eq + PartialOrd + Ord,
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
        unsafe { assume(self.in_bounds()) };
        self.0
    }
}

impl<T, const MIN: u128, const MAX: u128> Default for InRange<T, MIN, MAX>
where
    T: TryFrom<u128, Error: Debug>,
{
    fn default() -> Self {
        Self::min()
    }
}

impl<T, const MIN: u128, const MAX: u128> Display for InRange<T, MIN, MAX>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

macro_rules! impl_const_new {
    ($T:ty) => {
        impl<const MIN: u128, const MAX: u128> DefaultValue for InRange<$T, MIN, MAX> {
            const DEFAULT: Self = Self(0);
        }

        impl<const MIN: u128, const MAX: u128> InRange<$T, MIN, MAX> {
            #[allow(unused)]
            pub const fn const_new(value: $T) -> Self {
                assert!(value as u128 >= MIN && value as u128 <= MAX);
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
