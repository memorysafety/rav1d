use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};

// TODO: AddAssign
pub struct CursorMut<'a, T> {
    data: &'a mut [T],
    index: usize,
}

impl<'a, T> CursorMut<'a, T> {
    pub fn new(data: &'a mut [T]) -> Self {
        CursorMut { data, index: 0 }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data[self.index..]
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data[self.index..]
    }

    pub fn as_ptr(&self) -> *const T {
        self.as_slice().as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.as_mut_slice().as_mut_ptr()
    }

    pub fn clone<'b>(&'b mut self) -> CursorMut<'b, T> {
        CursorMut {
            data: self.data,
            index: self.index,
        }
    }
}

impl<'a, T> Add<usize> for CursorMut<'a, T> {
    type Output = Self;

    fn add(mut self, rhs: usize) -> Self::Output {
        self.index += rhs;
        assert!(self.index <= self.data.len());
        self
    }
}

impl<'a, T> AddAssign<usize> for CursorMut<'a, T> {
    fn add_assign(&mut self, rhs: usize) {
        self.index += rhs;
        assert!(self.index <= self.data.len());
    }
}

impl<'a, T> Sub<usize> for CursorMut<'a, T> {
    type Output = Self;

    fn sub(mut self, rhs: usize) -> Self::Output {
        assert!(rhs <= self.index);
        self.index -= rhs;
        self
    }
}

impl<'a, T> SubAssign<usize> for CursorMut<'a, T> {
    fn sub_assign(&mut self, rhs: usize) {
        assert!(rhs <= self.index);
        self.index -= rhs;
    }
}

impl<'a, T> Index<usize> for CursorMut<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[self.index + index]
    }
}

impl<'a, T> IndexMut<usize> for CursorMut<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[self.index + index]
    }
}

impl<'a, T> Index<isize> for CursorMut<'a, T> {
    type Output = T;

    fn index(&self, index: isize) -> &Self::Output {
        let index = self.index as isize + index;
        assert!(index > 0 && (index as usize) < self.data.len());
        &self.data[index as usize]
    }
}

impl<'a, T> IndexMut<isize> for CursorMut<'a, T> {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        let index = self.index as isize + index;
        assert!(index > 0 && (index as usize) < self.data.len());
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
