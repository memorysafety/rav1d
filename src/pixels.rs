use crate::include::common::bitdepth::BitDepth;
use crate::src::disjoint_mut::AsMutPtr;
use crate::src::disjoint_mut::DisjointMut;
use crate::src::strided::WithStride;
use std::mem;
use std::ops::Deref;

pub trait Pixels {
    /// Length in number of [`u8`] bytes.
    fn byte_len(&self) -> usize;

    /// Absolute ptr to [`u8`] bytes.
    fn as_byte_mut_ptr(&self) -> *mut u8;

    /// Length in number of [`BitDepth::Pixel`]s.
    fn pixel_len<BD: BitDepth>(&self) -> usize {
        self.byte_len() / mem::size_of::<BD::Pixel>()
    }

    /// Absolute ptr to [`BitDepth::Pixel`]s.
    fn as_mut_ptr<BD: BitDepth>(&self) -> *mut BD::Pixel {
        // SAFETY: Transmutation is safe because we verify this with `zerocopy` in `Self::slice`.
        self.as_byte_mut_ptr().cast()
    }

    /// Absolute ptr to [`BitDepth::Pixel`]s.
    fn as_ptr<BD: BitDepth>(&self) -> *const BD::Pixel {
        self.as_mut_ptr::<BD>().cast_const()
    }

    /// Absolute ptr to [`BitDepth::Pixel`]s starting at `pixel_offset`.
    ///
    /// Bounds checked, but not [`DisjointMut`]-checked.
    #[cfg_attr(debug_assertions, track_caller)]
    fn as_mut_ptr_at<BD: BitDepth>(&self, pixel_offset: usize) -> *mut BD::Pixel {
        #[inline(never)]
        #[cfg_attr(debug_assertions, track_caller)]
        fn out_of_bounds(pixel_offset: usize, pixel_len: usize) -> ! {
            panic!(
                "pixel offset {pixel_offset} out of range for slice of pixel length {pixel_len}"
            );
        }

        let pixel_len = self.pixel_len::<BD>();
        if pixel_offset > pixel_len {
            out_of_bounds(pixel_offset, pixel_len);
        }
        // SAFETY: We just checked that `pixel_offset` is in bounds.
        unsafe { self.as_mut_ptr::<BD>().add(pixel_offset) }
    }

    /// Absolute ptr to [`BitDepth::Pixel`]s starting at `pixel_offset`.
    ///
    /// Bounds checked, but not [`DisjointMut`]-checked.
    #[cfg_attr(debug_assertions, track_caller)]
    fn as_ptr_at<BD: BitDepth>(&self, pixel_offset: usize) -> *const BD::Pixel {
        self.as_mut_ptr_at::<BD>(pixel_offset).cast_const()
    }

    /// Absolute ptr to [`BitDepth::Pixel`]s starting at `pixel_offset`.
    ///
    /// There is no bounds checking and this ptr may wrap and go out of bounds.
    fn wrapping_as_mut_ptr_at<BD: BitDepth>(&self, pixel_offset: usize) -> *mut BD::Pixel {
        self.as_mut_ptr::<BD>().wrapping_add(pixel_offset)
    }

    /// Absolute ptr to [`BitDepth::Pixel`]s starting at `pixel_offset`.
    ///
    /// There is no bounds checking and this ptr may wrap and go out of bounds.
    fn wrapping_as_ptr_at<BD: BitDepth>(&self, pixel_offset: usize) -> *const BD::Pixel {
        self.as_ptr::<BD>().wrapping_add(pixel_offset)
    }

    /// Determine if they reference the same data.
    fn ref_eq(&self, other: &Self) -> bool {
        self.as_byte_mut_ptr() == other.as_byte_mut_ptr()
    }
}

impl<'a, P: Pixels> Pixels for &'a P {
    fn byte_len(&self) -> usize {
        (*self).byte_len()
    }

    fn as_byte_mut_ptr(&self) -> *mut u8 {
        (*self).as_byte_mut_ptr()
    }
}

impl<P: Pixels> Pixels for WithStride<P> {
    fn byte_len(&self) -> usize {
        self.deref().byte_len()
    }

    fn as_byte_mut_ptr(&self) -> *mut u8 {
        self.deref().as_byte_mut_ptr()
    }
}

impl<T: AsMutPtr<Target = u8>> Pixels for DisjointMut<T> {
    fn byte_len(&self) -> usize {
        self.len()
    }

    fn as_byte_mut_ptr(&self) -> *mut u8 {
        self.as_mut_ptr()
    }
}
