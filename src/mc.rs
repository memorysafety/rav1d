use std::iter;

use crate::include::common::bitdepth::{AsPrimitive, BitDepth};
use crate::include::dav1d::headers::Dav1dFilterMode;
use crate::src::tables::dav1d_mc_subpel_filters;

// TODO(kkysen) temporarily `pub` until `mc` callers are deduplicated
#[inline(never)]
pub unsafe fn put_c<BD: BitDepth>(
    dst: *mut BD::Pixel,
    dst_stride: usize,
    src: *const BD::Pixel,
    src_stride: usize,
    w: usize,
    h: usize,
) {
    let [dst_len, src_len] =
        [dst_stride, src_stride].map(|stride| if h == 0 { 0 } else { stride * (h - 1) + w });
    let mut dst = std::slice::from_raw_parts_mut(dst, dst_len);
    let mut src = std::slice::from_raw_parts(src, src_len);
    for (dst, src) in iter::zip(dst.chunks_mut(dst_stride), src.chunks(src_stride)) {
        BD::pixel_copy(dst, src, w);
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

unsafe fn filter_8tap<T: Into<i32>>(src: *const T, x: usize, f: &[i8; 8], stride: usize) -> i32 {
    f.into_iter()
        .enumerate()
        .map(|(i, &f)| {
            let [i, x, stride] = [i, x, stride].map(|it| it as isize);
            let j = x + (i - 3) * stride;
            i32::from(f) * src.offset(j).read().into()
        })
        .sum()
}

unsafe fn dav1d_filter_8tap_rnd<T: Into<i32>>(
    src: *const T,
    x: usize,
    f: &[i8; 8],
    stride: usize,
    sh: u8,
) -> i32 {
    (filter_8tap(src, x, f, stride) + ((1 << sh) >> 1)) >> sh
}

unsafe fn dav1d_filter_8tap_rnd2<T: Into<i32>>(
    src: *const T,
    x: usize,
    f: &[i8; 8],
    stride: usize,
    rnd: u8,
    sh: u8,
) -> i32 {
    (filter_8tap(src, x, f, stride) + (rnd as i32)) >> sh
}

unsafe fn dav1d_filter_8tap_clip<BD: BitDepth, T: Into<i32>>(
    bd: BD,
    src: *const T,
    x: usize,
    f: &[i8; 8],
    stride: usize,
    sh: u8,
) -> BD::Pixel {
    bd.iclip_pixel(dav1d_filter_8tap_rnd(src, x, f, stride, sh))
}

unsafe fn dav1d_filter_8tap_clip2<BD: BitDepth, T: Into<i32>>(
    bd: BD,
    src: *const T,
    x: usize,
    f: &[i8; 8],
    stride: usize,
    rnd: u8,
    sh: u8,
) -> BD::Pixel {
    bd.iclip_pixel(dav1d_filter_8tap_rnd2(src, x, f, stride, rnd, sh))
}

fn get_h_filter(mx: usize, w: usize, filter_type: Dav1dFilterMode) -> Option<&'static [i8; 8]> {
    let mx = mx.checked_sub(1)?;
    let i = if w > 4 {
        filter_type & 3
    } else {
        3 + (filter_type & 1)
    };
    Some(&dav1d_mc_subpel_filters[i as usize][mx])
}

fn get_v_filter(my: usize, h: usize, filter_type: Dav1dFilterMode) -> Option<&'static [i8; 8]> {
    let mx = my.checked_sub(1)?;
    let i = if h > 4 {
        filter_type >> 2
    } else {
        3 + ((filter_type >> 2) & 1)
    };
    Some(&dav1d_mc_subpel_filters[i as usize][mx])
}

fn get_filters(
    mx: usize,
    my: usize,
    w: usize,
    h: usize,
    filter_type: Dav1dFilterMode,
) -> (Option<&'static [i8; 8]>, Option<&'static [i8; 8]>) {
    (
        get_h_filter(mx, w, filter_type),
        get_v_filter(my, h, filter_type),
    )
}
