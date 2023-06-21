use crate::include::common::bitdepth::{AsPrimitive, BitDepth};

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
    let intermediate_bits = bd.get_intermediate_bits();
    for _ in 0..h {
        for x in 0..w {
            tmp[x] = (((*src.offset(x as isize)).as_::<i32>() << intermediate_bits)
                - (BD::PREP_BIAS as i32)) as i16;
        }
        tmp = &mut tmp[w..];
        src = src.offset(src_stride as isize);
    }
}
