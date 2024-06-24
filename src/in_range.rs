use crate::src::assume::assume;
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
