use crate::include::common::bitdepth::BitDepth;
use crate::src::pixels::Pixels;
use crate::src::strided::Strided;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Clone, Copy)]
pub struct WithOffset<T> {
    pub data: T,
    pub offset: usize,
}

impl<T> AddAssign<usize> for WithOffset<T> {
    #[cfg_attr(debug_assertions, track_caller)]
    fn add_assign(&mut self, rhs: usize) {
        self.offset += rhs;
    }
}

impl<T> SubAssign<usize> for WithOffset<T> {
    #[cfg_attr(debug_assertions, track_caller)]
    fn sub_assign(&mut self, rhs: usize) {
        self.offset -= rhs;
    }
}

impl<T> AddAssign<isize> for WithOffset<T> {
    #[cfg_attr(debug_assertions, track_caller)]
    fn add_assign(&mut self, rhs: isize) {
        self.offset = self.offset.wrapping_add_signed(rhs);
    }
}

impl<T> SubAssign<isize> for WithOffset<T> {
    #[cfg_attr(debug_assertions, track_caller)]
    fn sub_assign(&mut self, rhs: isize) {
        self.offset = self.offset.wrapping_add_signed(-rhs);
    }
}

impl<T> Add<usize> for WithOffset<T> {
    type Output = Self;

    #[cfg_attr(debug_assertions, track_caller)]
    fn add(mut self, rhs: usize) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T> Sub<usize> for WithOffset<T> {
    type Output = Self;

    #[cfg_attr(debug_assertions, track_caller)]
    fn sub(mut self, rhs: usize) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T> Add<isize> for WithOffset<T> {
    type Output = Self;

    #[cfg_attr(debug_assertions, track_caller)]
    fn add(mut self, rhs: isize) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T> Sub<isize> for WithOffset<T> {
    type Output = Self;

    #[cfg_attr(debug_assertions, track_caller)]
    fn sub(mut self, rhs: isize) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<P: Pixels> WithOffset<P> {
    #[inline] // Inline to see bounds checks in order to potentially elide them.
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn as_ptr<BD: BitDepth>(&self) -> *const BD::Pixel {
        self.data.as_ptr_at::<BD>(self.offset)
    }

    #[inline] // Inline to see bounds checks in order to potentially elide them.
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn as_mut_ptr<BD: BitDepth>(&self) -> *mut BD::Pixel {
        self.data.as_mut_ptr_at::<BD>(self.offset)
    }

    pub fn wrapping_as_ptr<BD: BitDepth>(&self) -> *const BD::Pixel {
        self.data.wrapping_as_ptr_at::<BD>(self.offset)
    }

    pub fn wrapping_as_mut_ptr<BD: BitDepth>(&self) -> *const BD::Pixel {
        self.data.wrapping_as_mut_ptr_at::<BD>(self.offset)
    }
}

impl<S: Strided> Strided for WithOffset<S> {
    fn stride(&self) -> isize {
        self.data.stride()
    }
}
