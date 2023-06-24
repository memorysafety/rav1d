use std::iter;

use crate::include::common::bitdepth::{AsPrimitive, BitDepth};

// TODO(kkysen) temporarily `pub` until `mc` callers are deduplicated
#[inline(never)]
pub unsafe fn put_c<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: usize,
    mut src: *const BD::Pixel,
    src_stride: usize,
    w: usize,
    h: usize,
) {
    for _ in 0..h {
        BD::pixel_copy(
            std::slice::from_raw_parts_mut(dst, w),
            std::slice::from_raw_parts(src, w),
            w,
        );
        dst = dst.offset(dst_stride as isize);
        src = src.offset(src_stride as isize);
    }
}

// TODO(kkysen) temporarily `pub` until `mc` callers are deduplicated
#[inline(never)]
pub unsafe fn prep_c<BD: BitDepth>(
    mut tmp: *mut i16,
    mut src: *const BD::Pixel,
    src_stride: usize,
    w: usize,
    h: usize,
    bd: BD,
) {
    let mut tmp = std::slice::from_raw_parts_mut(tmp, w * h);
    let mut src =
        std::slice::from_raw_parts(src, if h == 0 { 0 } else { src_stride * (h - 1) + w });
    let intermediate_bits = bd.get_intermediate_bits();
    for (tmp, src) in iter::zip(tmp.chunks_exact_mut(w), src.chunks(src_stride)).take(h) {
        for (tmp, src) in iter::zip(tmp, &src[..w]) {
            *tmp = (((*src).as_::<i32>() << intermediate_bits) - (BD::PREP_BIAS as i32)) as i16;
        }
    }
}
