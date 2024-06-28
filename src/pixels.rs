use std::mem;

use crate::include::common::bitdepth::BitDepth;
use crate::src::disjoint_mut::AsMutPtr;
use crate::src::disjoint_mut::DisjointMut;

pub trait Pixels {
    type Buf: AsMutPtr<Target = u8>;

    fn as_buf(&self) -> &DisjointMut<Self::Buf>;

    /// Length in number of [`u8`] bytes.
    fn len(&self) -> usize {
        self.as_buf().len()
    }

    /// Length in number of [`BitDepth::Pixel`]s.
    fn pixel_len<BD: BitDepth>(&self) -> usize {
        self.len() / mem::size_of::<BD::Pixel>()
    }

    /// Absolute ptr to [`BitDepth::Pixel`]s.
    fn as_mut_ptr<BD: BitDepth>(&self) -> *mut BD::Pixel {
        // SAFETY: Transmutation is safe because we verify this with `zerocopy` in `Self::slice`.
        self.as_buf().as_mut_ptr().cast()
    }

    /// Absolute ptr to [`BitDepth::Pixel`]s.
    fn _as_ptr<BD: BitDepth>(&self) -> *const BD::Pixel {
        self.as_mut_ptr::<BD>().cast_const()
    }

    /// Absolute ptr to [`BitDepth::Pixel`]s starting at `offset`.
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

    /// Absolute ptr to [`BitDepth::Pixel`]s starting at `offset`.
    ///
    /// Bounds checked, but not [`DisjointMut`]-checked.
    #[cfg_attr(debug_assertions, track_caller)]
    fn as_ptr_at<BD: BitDepth>(&self, offset: usize) -> *const BD::Pixel {
        self.as_mut_ptr_at::<BD>(offset).cast_const()
    }

    /// Determine if they reference the same data.
    fn ref_eq(&self, other: &Self) -> bool {
        self.as_buf().as_mut_ptr() == other.as_buf().as_mut_ptr()
    }
}
