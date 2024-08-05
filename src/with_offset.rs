use crate::include::common::bitdepth::BitDepth;
use crate::src::pixels::Pixels;
use crate::src::strided::Strided;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Clone, Copy)]
pub struct WithOffset<T> {
    pub data: T,
    pub offset: usize,
}

pub type CursorMut<'a, T> = WithOffset<&'a mut [T]>;

impl<'a, T> CursorMut<'a, T> {
    pub fn new(data: &'a mut [T]) -> Self {
        WithOffset { data, offset: 0 }
    }

    pub fn clone(&mut self) -> WithOffset<&mut [T]> {
        WithOffset {
            data: self.data,
            offset: self.offset,
        }
    }
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
}

impl<S: Strided> Strided for WithOffset<S> {
    fn stride(&self) -> isize {
        self.data.stride()
    }
}

impl<'a, T> Index<usize> for CursorMut<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[self.offset + index]
    }
}

impl<'a, T> IndexMut<usize> for CursorMut<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[self.offset + index]
    }
}

impl<'a, T> Index<isize> for CursorMut<'a, T> {
    type Output = T;

    fn index(&self, index: isize) -> &Self::Output {
        let index = self.offset as isize + index;
        debug_assert!(index >= 0);
        &self.data[index as usize]
    }
}

impl<'a, T> IndexMut<isize> for CursorMut<'a, T> {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        let index = self.offset as isize + index;
        debug_assert!(index >= 0);
        &mut self.data[index as usize]
    }
}

impl<'a, T> Index<i32> for CursorMut<'a, T> {
    type Output = T;

    fn index(&self, index: i32) -> &Self::Output {
        &self[index as isize]
    }
}

impl<'a, T> IndexMut<i32> for CursorMut<'a, T> {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        &mut self[index as isize]
    }
}
