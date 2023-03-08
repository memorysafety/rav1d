use std::ops::{Index, IndexMut};

#[derive(Copy, Clone)]
#[repr(C, align(32))]
pub struct Align32<T>(pub T);

impl<T> From<T> for Align32<T> {
    fn from(from: T) -> Self {
        Align32(from)
    }
}

impl<T: Index<usize>> Index<usize> for Align32<T> {
    type Output = T::Output;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: IndexMut<usize>> IndexMut<usize> for Align32<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct Align16<T>(pub T);

impl<T> From<T> for Align16<T> {
    fn from(from: T) -> Self {
        Align16(from)
    }
}

impl<T: Index<usize>> Index<usize> for Align16<T> {
    type Output = T::Output;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: IndexMut<usize>> IndexMut<usize> for Align16<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct Align8<T>(pub T);

impl<T> From<T> for Align8<T> {
    fn from(from: T) -> Self {
        Align8(from)
    }
}

impl<T: Index<usize>> Index<usize> for Align8<T> {
    type Output = T::Output;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: IndexMut<usize>> IndexMut<usize> for Align8<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(Copy, Clone)]
#[repr(C, align(4))]
pub struct Align4<T>(pub T);

impl<T> From<T> for Align4<T> {
    fn from(from: T) -> Self {
        Align4(from)
    }
}

impl<T: Index<usize>> Index<usize> for Align4<T> {
    type Output = T::Output;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: IndexMut<usize>> IndexMut<usize> for Align4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
