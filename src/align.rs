//! Types for enforcing alignment on struct fields.
//!
//! This module defines a handful of `AlignN` types, where `N` is alignment
//! enforced by that type. These types also implement a few useful traits to
//! make them easier to use in common cases, e.g. [`From`] and
//! [`Index`]/[`IndexMut`] (since it's usually array fields that require
//! specific aligment for use with SIMD instructions).

use std::ops::{Index, IndexMut};

macro_rules! def_align {
    ($align:literal, $name:ident) => {
        #[derive(Clone, Copy)]
        #[repr(C, align($align))]
        pub struct $name<T>(pub T);

        impl<T> From<T> for $name<T> {
            fn from(from: T) -> Self {
                Self(from)
            }
        }

        impl<T: Index<usize>> Index<usize> for $name<T> {
            type Output = T::Output;

            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }

        impl<T: IndexMut<usize>> IndexMut<usize> for $name<T> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.0[index]
            }
        }
    };
}

def_align!(1, Align1);
def_align!(2, Align2);
def_align!(4, Align4);
def_align!(8, Align8);
def_align!(16, Align16);
def_align!(32, Align32);
def_align!(64, Align64);
