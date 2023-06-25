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

// TODO(kkysen) temporarily `pub` until `mc` callers are deduplicated
#[inline(never)]
pub unsafe fn put_8tap_c<BD: BitDepth>(
    dst: *mut BD::Pixel,
    dst_stride: usize,
    mut src: *const BD::Pixel,
    src_stride: usize,
    w: usize,
    h: usize,
    mx: usize,
    my: usize,
    filter_type: Dav1dFilterMode,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let intermediate_rnd = 32 + (1 << 6 - intermediate_bits >> 1);

    let (fh, fv) = get_filters(mx, my, w, h, filter_type);
    let [dst_stride, src_stride] = [dst_stride, src_stride].map(BD::pxstride);

    let mut dst = std::slice::from_raw_parts_mut(dst, dst_stride * h);

    if let Some(fh) = fh {
        if let Some(fv) = fv {
            let tmp_h = h + 7;
            let mut mid = [0i16; 128 * 135]; // Default::default()
            let mut mid_ptr = &mut mid[..];

            src = src.offset(-((src_stride * 3) as isize));
            for _ in 0..tmp_h {
                for x in 0..w {
                    mid_ptr[x] = dav1d_filter_8tap_rnd(src, x, fh, 1, 6 - intermediate_bits) as i16;
                }

                mid_ptr = &mut mid_ptr[128..];
                src = src.offset(src_stride as isize);
            }

            mid_ptr = &mut mid[128 * 3..];
            for _ in 0..h {
                for x in 0..w {
                    dst[x] = dav1d_filter_8tap_clip(
                        bd,
                        mid_ptr.as_ptr(),
                        x,
                        fv,
                        128,
                        6 + intermediate_bits,
                    );
                }

                mid_ptr = &mut mid_ptr[128..];
                dst = &mut dst[dst_stride..];
            }
        } else {
            let mut src = std::slice::from_raw_parts(src, src_stride * h);
            for _ in 0..h {
                for x in 0..w {
                    dst[x] =
                        dav1d_filter_8tap_clip2(bd, src.as_ptr(), x, fh, 1, intermediate_rnd, 6);
                }

                dst = &mut dst[dst_stride..];
                src = &src[src_stride..];
            }
        }
    } else if let Some(fv) = fv {
        let mut src = std::slice::from_raw_parts(src, src_stride * h);
        for _ in 0..h {
            for x in 0..w {
                dst[x] = dav1d_filter_8tap_clip(bd, src.as_ptr(), x, fv, src_stride, 6);
            }

            dst = &mut dst[dst_stride..];
            src = &src[src_stride..];
        }
    } else {
        put_c::<BD>(dst.as_mut_ptr(), dst_stride, src, src_stride, w, h);
    }
}

// TODO(kkysen) temporarily `pub` until `mc` callers are deduplicated
#[inline(never)]
pub unsafe fn put_8tap_scaled_c<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: usize,
    mut src: *const BD::Pixel,
    src_stride: usize,
    w: usize,
    mut h: usize,
    mx: usize,
    mut my: usize,
    dx: usize,
    dy: usize,
    filter_type: Dav1dFilterMode,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let intermediate_rnd = (1 << intermediate_bits) >> 1;
    let tmp_h = ((h - 1) * dy + my >> 10) + 8;
    let mut mid = [0i16; 128 * (256 + 7)]; // Default::default()
    let mut mid_ptr = &mut mid[..];
    let [dst_stride, src_stride] = [dst_stride, src_stride].map(BD::pxstride);

    let mut dst = std::slice::from_raw_parts_mut(dst, dst_stride * h);

    src = src.offset(-((src_stride * 3) as isize));
    for _ in 0..tmp_h {
        let mut imx = mx;
        let mut ioff = 0;

        for x in 0..w {
            let fh = get_h_filter(imx >> 6, w, filter_type);
            mid_ptr[x] = match fh {
                Some(fh) => dav1d_filter_8tap_rnd(src, ioff, fh, 1, 6 - intermediate_bits) as i16,
                None => ((*src.offset(ioff as isize)).as_::<i32>() as i16) << intermediate_bits,
            };
            imx += dx;
            ioff += imx >> 10;
            imx &= 0x3ff;
        }

        mid_ptr = &mut mid_ptr[128..];
        src = src.offset(src_stride as isize);
    }
    mid_ptr = &mut mid[128 * 3..];
    for _ in 0..h {
        let fv = get_v_filter(my >> 6, h, filter_type);

        for x in 0..w {
            dst[x] = match fv {
                Some(fv) => {
                    dav1d_filter_8tap_clip(bd, mid_ptr.as_ptr(), x, fv, 128, 6 + intermediate_bits)
                }
                None => {
                    bd.iclip_pixel((i32::from(mid_ptr[x]) + intermediate_rnd) >> intermediate_bits)
                }
            };
        }

        my += dy;
        mid_ptr = &mut mid_ptr[(my >> 10) * 128..];
        my &= 0x3ff;
        dst = &mut dst[dst_stride..];
    }
}

#[inline(never)]
unsafe fn prep_8tap_c<BD: BitDepth>(
    mut tmp: *mut i16,
    mut src: *const BD::Pixel,
    src_stride: usize,
    w: usize,
    h: usize,
    mx: usize,
    my: usize,
    filter_type: Dav1dFilterMode,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let (fh, fv) = get_filters(mx, my, w, h, filter_type);
    let src_stride = BD::pxstride(src_stride);

    if let Some(fh) = fh {
        if let Some(fv) = fv {
            let tmp_h = h + 7;
            let mut mid = [0i16; 128 * 135]; // Default::default()
            let mut mid_ptr = &mut mid[..];

            src = src.offset(-((src_stride * 3) as isize));
            for _ in 0..tmp_h {
                for x in 0..w {
                    mid_ptr[x] = dav1d_filter_8tap_rnd(src, x, fh, 1, 6 - intermediate_bits) as i16;
                }

                mid_ptr = &mut mid_ptr[128..];
                src = src.offset(src_stride as isize);
            }

            mid_ptr = &mut mid[128 * 3..];
            for _ in 0..h {
                for x in 0..w {
                    *tmp.offset(x as isize) =
                        (dav1d_filter_8tap_rnd(mid_ptr.as_ptr(), x, fv, 128, 6)
                            - i32::from(BD::PREP_BIAS))
                        .try_into()
                        .unwrap();
                }

                mid_ptr = &mut mid_ptr[128..];
                tmp = tmp.offset(w as isize);
            }
        } else {
            for _ in 0..h {
                for x in 0..w {
                    *tmp.offset(x as isize) =
                        (dav1d_filter_8tap_rnd(src, x, fh, 1, 6 - intermediate_bits)
                            - i32::from(BD::PREP_BIAS)) as i16;
                }

                tmp = tmp.offset(w as isize);
                src = src.offset(src_stride as isize);
            }
        }
    } else if let Some(fv) = fv {
        for _ in 0..h {
            for x in 0..w {
                *tmp.offset(x as isize) =
                    (dav1d_filter_8tap_rnd(src, x, fv, src_stride, 6 - intermediate_bits)
                        - i32::from(BD::PREP_BIAS)) as i16;
            }

            tmp = tmp.offset(w as isize);
            src = src.offset(src_stride as isize);
        }
    } else {
        prep_c(tmp, src, src_stride, w, h, bd);
    };
}
