use std::iter;

use crate::include::common::bitdepth::{AsPrimitive, BitDepth};
use crate::include::common::intops::iclip;
use crate::include::dav1d::headers::Dav1dFilterMode;
use crate::src::tables::dav1d_mc_subpel_filters;
use crate::src::tables::dav1d_mc_warp_filter;
use crate::src::tables::dav1d_obmc_masks;
use crate::src::tables::dav1d_resize_filter;

// TODO(kkysen) temporarily `pub` until `mc` callers are deduplicated
#[inline(never)]
pub unsafe fn put_rust<BD: BitDepth>(
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
pub unsafe fn prep_rust<BD: BitDepth>(
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
pub unsafe fn put_8tap_rust<BD: BitDepth>(
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
        put_rust::<BD>(dst.as_mut_ptr(), dst_stride, src, src_stride, w, h);
    }
}

// TODO(kkysen) temporarily `pub` until `mc` callers are deduplicated
#[inline(never)]
pub unsafe fn put_8tap_scaled_rust<BD: BitDepth>(
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

// TODO(kkysen) temporarily `pub` until `mc` callers are deduplicated
#[inline(never)]
pub unsafe fn prep_8tap_rust<BD: BitDepth>(
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
        prep_rust(tmp, src, src_stride, w, h, bd);
    };
}

// TODO(kkysen) temporarily `pub` until `mc` callers are deduplicated
#[inline(never)]
pub unsafe fn prep_8tap_scaled_rust<BD: BitDepth>(
    mut tmp: *mut i16,
    mut src: *const BD::Pixel,
    src_stride: usize,
    w: usize,
    h: usize,
    mut mx: usize,
    mut my: usize,
    dx: usize,
    dy: usize,
    filter_type: Dav1dFilterMode,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let tmp_h = ((h - 1) * dy + my >> 10) + 8;
    let mut mid = [0i16; 128 * (256 + 7)]; // Default::default()
    let mut mid_ptr = &mut mid[..];
    let src_stride = BD::pxstride(src_stride);

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
            *tmp.offset(x as isize) = ((match fv {
                Some(fv) => dav1d_filter_8tap_rnd(mid_ptr.as_ptr(), x, fv, 128, 6),
                None => i32::from(mid_ptr[x]),
            }) - i32::from(BD::PREP_BIAS)) as i16;
        }
        my += dy;
        mid_ptr = &mut mid_ptr[(my >> 10) * 128..];
        my &= 0x3ff;
        tmp = tmp.offset(w as isize);
    }
}

unsafe fn filter_bilin<T: Into<i32>>(src: *const T, x: usize, mxy: usize, stride: usize) -> i32 {
    let src = |i: usize| -> i32 { src.offset(i as isize).read().into() };
    16 * src(x) + ((mxy as i32) * (src(x + stride) - src(x)))
}

unsafe fn filter_bilin_rnd<T: Into<i32>>(
    src: *const T,
    x: usize,
    mxy: usize,
    stride: usize,
    sh: u8,
) -> i32 {
    (filter_bilin(src, x, mxy, stride) + ((1 << sh) >> 1)) >> sh
}

unsafe fn filter_bilin_clip<BD: BitDepth, T: Into<i32>>(
    bd: BD,
    src: *const T,
    x: usize,
    mxy: usize,
    stride: usize,
    sh: u8,
) -> BD::Pixel {
    bd.iclip_pixel(filter_bilin_rnd(src, x, mxy, stride, sh))
}

// TODO(kkysen) temporarily `pub` until `mc` callers are deduplicated
pub unsafe fn put_bilin_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: usize,
    mut src: *const BD::Pixel,
    src_stride: usize,
    w: usize,
    h: usize,
    mx: usize,
    my: usize,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let intermediate_rnd = (1 << intermediate_bits) >> 1;
    let [dst_stride, src_stride] = [dst_stride, src_stride].map(BD::pxstride);

    if mx != 0 {
        if my != 0 {
            let mut mid = [0i16; 128 * 129]; // Default::default()
            let mut mid_ptr = &mut mid[..];
            let tmp_h = h + 1;

            for _ in 0..tmp_h {
                for x in 0..w {
                    mid_ptr[x] = filter_bilin_rnd(src, x, mx, 1, 4 - intermediate_bits) as i16;
                }

                mid_ptr = &mut mid_ptr[128..];
                src = src.offset(src_stride as isize);
            }
            mid_ptr = &mut mid[..];
            for _ in 0..h {
                for x in 0..w {
                    *dst.offset(x as isize) =
                        filter_bilin_clip(bd, mid_ptr.as_ptr(), x, my, 128, 4 + intermediate_bits);
                }

                mid_ptr = &mut mid_ptr[128..];
                dst = dst.offset(dst_stride as isize);
            }
        } else {
            for _ in 0..h {
                for x in 0..w {
                    let px = filter_bilin_rnd(src, x, mx, 1, 4 - intermediate_bits);
                    *dst.offset(x as isize) =
                        bd.iclip_pixel((px + intermediate_rnd) >> intermediate_bits);
                }

                dst = dst.offset(dst_stride as isize);
                src = src.offset(src_stride as isize);
            }
        }
    } else if my != 0 {
        for _ in 0..h {
            for x in 0..w {
                *dst.offset(x as isize) = filter_bilin_clip(bd, src, x, my, src_stride, 4);
            }

            dst = dst.offset(dst_stride as isize);
            src = src.offset(src_stride as isize);
        }
    } else {
        put_rust::<BD>(dst, dst_stride, src, src_stride, w, h);
    };
}

// TODO(kkysen) temporarily `pub` until `mc` callers are deduplicated
pub unsafe fn put_bilin_scaled_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    mut dst_stride: usize,
    mut src: *const BD::Pixel,
    mut src_stride: usize,
    w: usize,
    h: usize,
    mut mx: usize,
    mut my: usize,
    dx: usize,
    dy: usize,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let [dst_stride, src_stride] = [dst_stride, src_stride].map(BD::pxstride);
    let tmp_h = ((h - 1) * dy + my >> 10) + 2;
    let mut mid = [0i16; 128 * (256 + 1)];
    let mut mid_ptr = &mut mid[..];

    for _ in 0..tmp_h {
        let mut imx = mx;
        let mut ioff = 0;

        for x in 0..w {
            mid_ptr[x] = filter_bilin_rnd(src, ioff, imx >> 6, 1, 4 - intermediate_bits) as i16;
            imx += dx;
            ioff += imx >> 10;
            imx &= 0x3ff;
        }

        mid_ptr = &mut mid_ptr[128..];
        src = src.offset(src_stride as isize);
    }
    mid_ptr = &mut mid[..];
    for _ in 0..h {
        for x in 0..w {
            *dst.offset(x as isize) =
                filter_bilin_clip(bd, mid_ptr.as_ptr(), x, my >> 6, 128, 4 + intermediate_bits);
        }

        my += dy;
        mid_ptr = &mut mid_ptr[(my >> 10) * 128..];
        my &= 0x3ff;
        dst = dst.offset(dst_stride as isize);
    }
}

// TODO(kkysen) temporarily `pub` until `mc` callers are deduplicated
pub unsafe fn prep_bilin_rust<BD: BitDepth>(
    mut tmp: *mut i16,
    mut src: *const BD::Pixel,
    src_stride: usize,
    w: usize,
    h: usize,
    mx: usize,
    my: usize,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let src_stride = BD::pxstride(src_stride);
    if mx != 0 {
        if my != 0 {
            let mut mid = [0i16; 128 * 129];
            let mut mid_ptr = &mut mid[..];
            let tmp_h = h + 1;

            for _ in 0..tmp_h {
                for x in 0..w {
                    mid_ptr[x] = filter_bilin_rnd(src, x, mx, 1, 4 - intermediate_bits) as i16;
                }

                mid_ptr = &mut mid_ptr[128..];
                src = src.offset(src_stride as isize);
            }
            mid_ptr = &mut mid[..];
            for _ in 0..h {
                for x in 0..w {
                    *tmp.offset(x as isize) = (filter_bilin_rnd(mid_ptr.as_ptr(), x, my, 128, 4)
                        - i32::from(BD::PREP_BIAS))
                        as i16;
                }

                mid_ptr = &mut mid_ptr[128..];
                tmp = tmp.offset(w as isize);
            }
        } else {
            for _ in 0..h {
                for x in 0..w {
                    *tmp.offset(x as isize) =
                        (filter_bilin_rnd(src, x, mx, 1, 4 - intermediate_bits)
                            - i32::from(BD::PREP_BIAS)) as i16;
                }

                tmp = tmp.offset(w as isize);
                src = src.offset(src_stride as isize);
            }
        }
    } else if my != 0 {
        for _ in 0..h {
            for x in 0..w {
                *tmp.offset(x as isize) =
                    (filter_bilin_rnd(src, x, my, src_stride, 4 - intermediate_bits)
                        - i32::from(BD::PREP_BIAS)) as i16;
            }

            tmp = tmp.offset(w as isize);
            src = src.offset(src_stride as isize);
        }
    } else {
        prep_rust(tmp, src, src_stride, w, h, bd);
    };
}

// TODO(kkysen) temporarily `pub` until `mc` callers are deduplicated
pub unsafe fn prep_bilin_scaled_rust<BD: BitDepth>(
    mut tmp: *mut i16,
    mut src: *const BD::Pixel,
    src_stride: usize,
    w: usize,
    h: usize,
    mut mx: usize,
    mut my: usize,
    dx: usize,
    dy: usize,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let src_stride = BD::pxstride(src_stride);
    let mut tmp_h = ((h - 1) * dy + my >> 10) + 2;
    let mut mid = [0i16; 128 * (256 + 1)];
    let mut mid_ptr = &mut mid[..];

    for _ in 0..tmp_h {
        let mut imx = mx;
        let mut ioff = 0;

        for x in 0..w {
            mid_ptr[x] = filter_bilin_rnd(src, ioff, imx >> 6, 1, 4 - intermediate_bits) as i16;
            imx += dx;
            ioff += imx >> 10;
            imx &= 0x3ff;
        }

        mid_ptr = &mut mid_ptr[128..];
        src = src.offset(src_stride as isize);
    }
    mid_ptr = &mut mid[..];
    for _ in 0..h {
        for x in 0..w {
            *tmp.offset(x as isize) = (filter_bilin_rnd(mid_ptr.as_ptr(), x, my >> 6, 128, 4)
                - i32::from(BD::PREP_BIAS)) as i16;
        }

        my += dy;
        mid_ptr = &mut mid_ptr[(my >> 10) * 128..];
        my &= 0x3ff;
        tmp = tmp.offset(w as isize);
    }
}

// TODO(kkysen) temporarily `pub` until `mc` callers are deduplicated
pub unsafe fn avg_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: usize,
    mut tmp1: *const i16,
    mut tmp2: *const i16,
    w: usize,
    h: usize,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let sh = intermediate_bits + 1;
    let rnd = (1 << intermediate_bits) + i32::from(BD::PREP_BIAS) * 2;
    let dst_stride = BD::pxstride(dst_stride);
    for _ in 0..h {
        for x in 0..w {
            *dst.offset(x as isize) = bd.iclip_pixel(
                ((*tmp1.offset(x as isize) as i32 + *tmp2.offset(x as isize) as i32 + rnd) >> sh)
                    .into(),
            );
        }

        tmp1 = tmp1.offset(w as isize);
        tmp2 = tmp2.offset(w as isize);
        dst = dst.offset(dst_stride as isize);
    }
}

// TODO(kkysen) temporarily `pub` until `mc` callers are deduplicated
pub unsafe fn w_avg_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: usize,
    mut tmp1: *const i16,
    mut tmp2: *const i16,
    w: usize,
    h: usize,
    weight: i32,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let sh = intermediate_bits + 4;
    let rnd = (8 << intermediate_bits) + i32::from(BD::PREP_BIAS) * 16;
    let dst_stride = BD::pxstride(dst_stride);
    for _ in 0..h {
        for x in 0..w {
            *dst.offset(x as isize) = bd.iclip_pixel(
                (*tmp1.offset(x as isize) as i32 * weight
                    + *tmp2.offset(x as isize) as i32 * (16 - weight)
                    + rnd)
                    >> sh,
            );
        }

        tmp1 = tmp1.offset(w as isize);
        tmp2 = tmp2.offset(w as isize);
        dst = dst.offset(dst_stride as isize);
    }
}

// TODO(kkysen) temporarily `pub` until `mc` callers are deduplicated
pub unsafe fn mask_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: usize,
    mut tmp1: *const i16,
    mut tmp2: *const i16,
    w: usize,
    h: usize,
    mut mask: *const u8,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let sh = intermediate_bits + 6;
    let rnd = (32 << intermediate_bits) + i32::from(BD::PREP_BIAS) * 64;
    let dst_stride = BD::pxstride(dst_stride);
    for _ in 0..h {
        for x in 0..w {
            *dst.offset(x as isize) = bd.iclip_pixel(
                (*tmp1.offset(x as isize) as i32 * *mask.offset(x as isize) as i32
                    + *tmp2.offset(x as isize) as i32 * (64 - *mask.offset(x as isize) as i32)
                    + rnd)
                    >> sh,
            );
        }

        tmp1 = tmp1.offset(w as isize);
        tmp2 = tmp2.offset(w as isize);
        mask = mask.offset(w as isize);
        dst = dst.offset(dst_stride as isize);
    }
}

fn blend_px<BD: BitDepth>(a: BD::Pixel, b: BD::Pixel, m: u8) -> BD::Pixel {
    let m = m as u32;
    ((a.as_::<u32>() * (64 - m) + b.as_::<u32>() * m + 32) >> 6).as_::<BD::Pixel>()
}

// TODO(kkysen) temporarily `pub` until `mc` callers are deduplicated
pub unsafe fn blend_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: usize,
    mut tmp: *const BD::Pixel,
    w: usize,
    h: usize,
    mut mask: *const u8,
) {
    let dst_stride = BD::pxstride(dst_stride);
    for _ in 0..h {
        for x in 0..w {
            *dst.offset(x as isize) = blend_px::<BD>(
                *dst.offset(x as isize),
                *tmp.offset(x as isize),
                *mask.offset(x as isize),
            )
        }

        dst = dst.offset(dst_stride as isize);
        tmp = tmp.offset(w as isize);
        mask = mask.offset(w as isize);
    }
}

// TODO(kkysen) temporarily `pub` until `mc` callers are deduplicated
pub unsafe fn blend_v_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: usize,
    mut tmp: *const BD::Pixel,
    w: usize,
    h: usize,
) {
    let mask = &dav1d_obmc_masks.0[w..];
    let dst_stride = BD::pxstride(dst_stride);
    for _ in 0..h {
        for x in 0..(w * 3 >> 2) {
            *dst.offset(x as isize) =
                blend_px::<BD>(*dst.offset(x as isize), *tmp.offset(x as isize), mask[x])
        }

        dst = dst.offset(dst_stride as isize);
        tmp = tmp.offset(w as isize);
    }
}

// TODO(kkysen) temporarily `pub` until `mc` callers are deduplicated
pub unsafe fn blend_h_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: usize,
    mut tmp: *const BD::Pixel,
    w: usize,
    h: usize,
) {
    let mask = &dav1d_obmc_masks.0[h..];
    let h = h * 3 >> 2;
    let dst_stride = BD::pxstride(dst_stride);
    for y in 0..h {
        for x in 0..w {
            *dst.offset(x as isize) =
                blend_px::<BD>(*dst.offset(x as isize), *tmp.offset(x as isize), mask[y]);
        }

        dst = dst.offset(dst_stride as isize);
        tmp = tmp.offset(w as isize);
    }
}

// TODO(kkysen) temporarily `pub` until `mc` callers are deduplicated
pub unsafe fn w_mask_rust<BD: BitDepth>(
    dst: *mut BD::Pixel,
    dst_stride: usize,
    tmp1: *const i16,
    tmp2: *const i16,
    w: usize,
    h: usize,
    mask: *mut u8,
    sign: bool,
    ss_hor: bool,
    ss_ver: bool,
    bd: BD,
) {
    let dst_stride = BD::pxstride(dst_stride);
    let dst =
        std::slice::from_raw_parts_mut(dst, if h == 0 { 0 } else { (h - 1) * dst_stride + w });
    let [tmp1, tmp2] = [tmp1, tmp2].map(|tmp| std::slice::from_raw_parts(tmp, h * w));
    let mut mask =
        std::slice::from_raw_parts_mut(mask, (w >> ss_hor as usize) * (h >> ss_ver as usize));
    let sign = sign as u8;

    // store mask at 2x2 resolution, i.e. store 2x1 sum for even rows,
    // and then load this intermediate to calculate final value for odd rows
    let intermediate_bits = bd.get_intermediate_bits();
    let bitdepth = bd.bitdepth();
    let sh = intermediate_bits + 6;
    let rnd = (32 << intermediate_bits) + i32::from(BD::PREP_BIAS) * 64;
    let mask_sh = bitdepth + intermediate_bits - 4;
    let mask_rnd = 1 << (mask_sh - 5);

    let calc_mn =
        |t1, t2| std::cmp::min(38 + ((i16::abs_diff(t1, t2) + mask_rnd) >> mask_sh), 64) as u8;
    let calc_dst = |t1, t2, mn| {
        bd.iclip_pixel((t1 as i32 * mn as i32 + t2 as i32 * (64 - mn as i32) + rnd) >> sh)
    };

    for (h, ((tmp1, tmp2), dst)) in iter::zip(tmp1.chunks_exact(w), tmp2.chunks_exact(w))
        .zip(dst.chunks_mut(dst_stride))
        .enumerate()
    {
        let mut x = 0;
        while x < w {
            let m = calc_mn(tmp1[x], tmp2[x]);
            dst[x] = calc_dst(tmp1[x], tmp2[x], m);

            if ss_hor {
                x += 1;

                let n = calc_mn(tmp1[x], tmp2[x]);
                dst[x] = calc_dst(tmp1[x], tmp2[x], n);

                mask[x >> 1] = if h & ss_ver as usize != 0 {
                    ((m + n + mask[x >> 1] + 2 - sign) >> 2) as u8
                } else if ss_ver {
                    m + n
                } else {
                    ((m + n + 1 - sign) >> 1) as u8
                };
            } else {
                mask[x] = m;
            }
            x += 1;
        }

        if !ss_ver || h & 1 != 0 {
            mask = &mut mask[w >> ss_hor as usize..];
        }
    }
}

// TODO(kkysen) temporarily `pub` until `mc` callers are deduplicated
pub unsafe fn warp_affine_8x8_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: libc::ptrdiff_t,
    mut src: *const BD::Pixel,
    src_stride: libc::ptrdiff_t,
    abcd: *const i16,
    mut mx: libc::c_int,
    mut my: libc::c_int,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let mut mid: [i16; 120] = [0; 120];
    let mut mid_ptr: *mut i16 = mid.as_mut_ptr();
    src = src.offset(-((3 * BD::pxstride(src_stride as usize)) as isize));
    let mut y = 0;
    while y < 15 {
        let mut x = 0;
        let mut tmx = mx;
        while x < 8 {
            let filter: *const i8 =
                (dav1d_mc_warp_filter[(64 as libc::c_int + (tmx + 512 >> 10)) as usize]).as_ptr();
            *mid_ptr.offset(x as isize) = (*filter.offset(0) as libc::c_int
                * (*src.offset((x - 3 * 1) as isize)).as_::<libc::c_int>()
                + *filter.offset(1) as libc::c_int
                    * (*src.offset((x - 2 * 1) as isize)).as_::<libc::c_int>()
                + *filter.offset(2) as libc::c_int
                    * (*src.offset((x - 1 * 1) as isize)).as_::<libc::c_int>()
                + *filter.offset(3) as libc::c_int
                    * (*src.offset((x + 0 * 1) as isize)).as_::<libc::c_int>()
                + *filter.offset(4) as libc::c_int
                    * (*src.offset((x + 1 * 1) as isize)).as_::<libc::c_int>()
                + *filter.offset(5) as libc::c_int
                    * (*src.offset((x + 2 * 1) as isize)).as_::<libc::c_int>()
                + *filter.offset(6) as libc::c_int
                    * (*src.offset((x + 3 * 1) as isize)).as_::<libc::c_int>()
                + *filter.offset(7) as libc::c_int
                    * (*src.offset((x + 4 * 1) as isize)).as_::<libc::c_int>()
                + ((1 as libc::c_int) << 7 - intermediate_bits >> 1)
                >> 7 - intermediate_bits) as i16;
            x += 1;
            tmx += *abcd.offset(0) as libc::c_int;
        }
        src = src.offset(BD::pxstride(src_stride as usize) as isize);
        mid_ptr = mid_ptr.offset(8);
        y += 1;
        mx += *abcd.offset(1) as libc::c_int;
    }
    mid_ptr = &mut *mid.as_mut_ptr().offset((3 * 8) as isize) as *mut i16;
    let mut y_0 = 0;
    while y_0 < 8 {
        let mut x_0 = 0;
        let mut tmy = my;
        while x_0 < 8 {
            let filter_0: *const i8 =
                (dav1d_mc_warp_filter[(64 as libc::c_int + (tmy + 512 >> 10)) as usize]).as_ptr();
            *dst.offset(x_0 as isize) = bd.iclip_pixel(
                *filter_0.offset(0) as libc::c_int
                    * *mid_ptr.offset((x_0 - 3 * 8) as isize) as libc::c_int
                    + *filter_0.offset(1) as libc::c_int
                        * *mid_ptr.offset((x_0 - 2 * 8) as isize) as libc::c_int
                    + *filter_0.offset(2) as libc::c_int
                        * *mid_ptr.offset((x_0 - 1 * 8) as isize) as libc::c_int
                    + *filter_0.offset(3) as libc::c_int
                        * *mid_ptr.offset((x_0 + 0 * 8) as isize) as libc::c_int
                    + *filter_0.offset(4) as libc::c_int
                        * *mid_ptr.offset((x_0 + 1 * 8) as isize) as libc::c_int
                    + *filter_0.offset(5) as libc::c_int
                        * *mid_ptr.offset((x_0 + 2 * 8) as isize) as libc::c_int
                    + *filter_0.offset(6) as libc::c_int
                        * *mid_ptr.offset((x_0 + 3 * 8) as isize) as libc::c_int
                    + *filter_0.offset(7) as libc::c_int
                        * *mid_ptr.offset((x_0 + 4 * 8) as isize) as libc::c_int
                    + ((1 as libc::c_int) << 7 + intermediate_bits >> 1)
                    >> 7 + intermediate_bits,
            );
            x_0 += 1;
            tmy += *abcd.offset(2) as libc::c_int;
        }
        mid_ptr = mid_ptr.offset(8);
        dst = dst.offset(BD::pxstride(dst_stride as usize) as isize);
        y_0 += 1;
        my += *abcd.offset(3) as libc::c_int;
    }
}

// TODO(kkysen) temporarily `pub` until `mc` callers are deduplicated
pub unsafe fn warp_affine_8x8t_rust<BD: BitDepth>(
    mut tmp: *mut i16,
    tmp_stride: libc::ptrdiff_t,
    mut src: *const BD::Pixel,
    src_stride: libc::ptrdiff_t,
    abcd: *const i16,
    mut mx: libc::c_int,
    mut my: libc::c_int,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let mut mid: [i16; 120] = [0; 120];
    let mut mid_ptr: *mut i16 = mid.as_mut_ptr();
    src = src.offset(-((3 * BD::pxstride(src_stride as usize)) as isize));
    let mut y = 0;
    while y < 15 {
        let mut x = 0;
        let mut tmx = mx;
        while x < 8 {
            let filter: *const i8 =
                (dav1d_mc_warp_filter[(64 as libc::c_int + (tmx + 512 >> 10)) as usize]).as_ptr();
            *mid_ptr.offset(x as isize) = (*filter.offset(0) as libc::c_int
                * (*src.offset((x - 3 * 1) as isize)).as_::<libc::c_int>()
                + *filter.offset(1) as libc::c_int
                    * (*src.offset((x - 2 * 1) as isize)).as_::<libc::c_int>()
                + *filter.offset(2) as libc::c_int
                    * (*src.offset((x - 1 * 1) as isize)).as_::<libc::c_int>()
                + *filter.offset(3) as libc::c_int
                    * (*src.offset((x + 0 * 1) as isize)).as_::<libc::c_int>()
                + *filter.offset(4) as libc::c_int
                    * (*src.offset((x + 1 * 1) as isize)).as_::<libc::c_int>()
                + *filter.offset(5) as libc::c_int
                    * (*src.offset((x + 2 * 1) as isize)).as_::<libc::c_int>()
                + *filter.offset(6) as libc::c_int
                    * (*src.offset((x + 3 * 1) as isize)).as_::<libc::c_int>()
                + *filter.offset(7) as libc::c_int
                    * (*src.offset((x + 4 * 1) as isize)).as_::<libc::c_int>()
                + ((1 as libc::c_int) << 7 - intermediate_bits >> 1)
                >> 7 - intermediate_bits) as i16;
            x += 1;
            tmx += *abcd.offset(0) as libc::c_int;
        }
        src = src.offset(BD::pxstride(src_stride as usize) as isize);
        mid_ptr = mid_ptr.offset(8);
        y += 1;
        mx += *abcd.offset(1) as libc::c_int;
    }
    mid_ptr = &mut *mid.as_mut_ptr().offset((3 * 8) as isize) as *mut i16;
    let mut y_0 = 0;
    while y_0 < 8 {
        let mut x_0 = 0;
        let mut tmy = my;
        while x_0 < 8 {
            let filter_0: *const i8 =
                (dav1d_mc_warp_filter[(64 as libc::c_int + (tmy + 512 >> 10)) as usize]).as_ptr();
            *tmp.offset(x_0 as isize) = ((*filter_0.offset(0) as libc::c_int
                * *mid_ptr.offset((x_0 - 3 * 8) as isize) as libc::c_int
                + *filter_0.offset(1) as libc::c_int
                    * *mid_ptr.offset((x_0 - 2 * 8) as isize) as libc::c_int
                + *filter_0.offset(2) as libc::c_int
                    * *mid_ptr.offset((x_0 - 1 * 8) as isize) as libc::c_int
                + *filter_0.offset(3) as libc::c_int
                    * *mid_ptr.offset((x_0 + 0 * 8) as isize) as libc::c_int
                + *filter_0.offset(4) as libc::c_int
                    * *mid_ptr.offset((x_0 + 1 * 8) as isize) as libc::c_int
                + *filter_0.offset(5) as libc::c_int
                    * *mid_ptr.offset((x_0 + 2 * 8) as isize) as libc::c_int
                + *filter_0.offset(6) as libc::c_int
                    * *mid_ptr.offset((x_0 + 3 * 8) as isize) as libc::c_int
                + *filter_0.offset(7) as libc::c_int
                    * *mid_ptr.offset((x_0 + 4 * 8) as isize) as libc::c_int
                + ((1 as libc::c_int) << 7 >> 1)
                >> 7)
                - i32::from(BD::PREP_BIAS)) as i16;
            x_0 += 1;
            tmy += *abcd.offset(2) as libc::c_int;
        }
        mid_ptr = mid_ptr.offset(8);
        tmp = tmp.offset(tmp_stride as isize);
        y_0 += 1;
        my += *abcd.offset(3) as libc::c_int;
    }
}

// TODO(kkysen) temporarily `pub` until `mc` callers are deduplicated
pub unsafe fn emu_edge_rust<BD: BitDepth>(
    bw: libc::intptr_t,
    bh: libc::intptr_t,
    iw: libc::intptr_t,
    ih: libc::intptr_t,
    x: libc::intptr_t,
    y: libc::intptr_t,
    mut dst: *mut BD::Pixel,
    dst_stride: libc::ptrdiff_t,
    mut r#ref: *const BD::Pixel,
    ref_stride: libc::ptrdiff_t,
) {
    r#ref = r#ref.offset(
        iclip(y as libc::c_int, 0 as libc::c_int, ih as libc::c_int - 1) as isize
            * BD::pxstride(ref_stride as usize) as isize
            + iclip(x as libc::c_int, 0 as libc::c_int, iw as libc::c_int - 1) as isize,
    );
    let left_ext = iclip(-x as libc::c_int, 0 as libc::c_int, bw as libc::c_int - 1);
    let right_ext = iclip(
        (x + bw - iw) as libc::c_int,
        0 as libc::c_int,
        bw as libc::c_int - 1,
    );
    if !(((left_ext + right_ext) as isize) < bw) {
        unreachable!();
    }
    let top_ext = iclip(-y as libc::c_int, 0 as libc::c_int, bh as libc::c_int - 1);
    let bottom_ext = iclip(
        (y + bh - ih) as libc::c_int,
        0 as libc::c_int,
        bh as libc::c_int - 1,
    );
    if !(((top_ext + bottom_ext) as isize) < bh) {
        unreachable!();
    }
    let mut blk: *mut BD::Pixel =
        dst.offset((top_ext as isize * BD::pxstride(dst_stride as usize) as isize) as isize);
    let center_w = (bw - left_ext as isize - right_ext as isize) as libc::c_int;
    let center_h = (bh - top_ext as isize - bottom_ext as isize) as libc::c_int;
    let mut y_0 = 0;
    while y_0 < center_h {
        BD::pixel_copy(
            std::slice::from_raw_parts_mut(blk.offset(left_ext as isize), center_w as usize),
            std::slice::from_raw_parts(r#ref, center_w as usize),
            center_w as usize,
        );
        if left_ext != 0 {
            BD::pixel_set(
                std::slice::from_raw_parts_mut(blk, left_ext as usize),
                *blk.offset(left_ext as isize),
                left_ext as usize,
            );
        }
        if right_ext != 0 {
            BD::pixel_set(
                std::slice::from_raw_parts_mut(
                    blk.offset(left_ext as isize).offset(center_w as isize),
                    right_ext as usize,
                ),
                *blk.offset((left_ext + center_w - 1) as isize),
                right_ext as usize,
            );
        }
        r#ref = r#ref.offset(BD::pxstride(ref_stride as usize) as isize);
        blk = blk.offset(BD::pxstride(dst_stride as usize) as isize);
        y_0 += 1;
    }
    blk = dst.offset((top_ext as isize * BD::pxstride(dst_stride as usize) as isize) as isize);
    let mut y_1 = 0;
    while y_1 < top_ext {
        BD::pixel_copy(
            std::slice::from_raw_parts_mut(dst, bw as usize),
            std::slice::from_raw_parts(blk, bw as usize),
            bw as usize,
        );
        dst = dst.offset(BD::pxstride(dst_stride as usize) as isize);
        y_1 += 1;
    }
    dst = dst.offset((center_h as isize * BD::pxstride(dst_stride as usize) as isize) as isize);
    let mut y_2 = 0;
    while y_2 < bottom_ext {
        BD::pixel_copy(
            std::slice::from_raw_parts_mut(dst, bw as usize),
            std::slice::from_raw_parts(
                dst.offset(-(BD::pxstride(dst_stride as usize) as isize)),
                bw as usize,
            ),
            bw as usize,
        );
        dst = dst.offset(BD::pxstride(dst_stride as usize) as isize);
        y_2 += 1;
    }
}

// TODO(kkysen) temporarily `pub` until `mc` callers are deduplicated
pub unsafe fn resize_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: libc::ptrdiff_t,
    mut src: *const BD::Pixel,
    src_stride: libc::ptrdiff_t,
    dst_w: libc::c_int,
    mut h: libc::c_int,
    src_w: libc::c_int,
    dx: libc::c_int,
    mx0: libc::c_int,
    bd: BD,
) {
    loop {
        let mut mx = mx0;
        let mut src_x = -(1 as libc::c_int);
        let mut x = 0;
        while x < dst_w {
            let F: *const i8 = (dav1d_resize_filter[(mx >> 8) as usize]).as_ptr();
            *dst.offset(x as isize) = bd.iclip_pixel(
                -(*F.offset(0) as libc::c_int
                    * (*src.offset(iclip(src_x - 3, 0 as libc::c_int, src_w - 1) as isize))
                        .as_::<libc::c_int>()
                    + *F.offset(1) as libc::c_int
                        * (*src.offset(iclip(src_x - 2, 0 as libc::c_int, src_w - 1) as isize))
                            .as_::<libc::c_int>()
                    + *F.offset(2) as libc::c_int
                        * (*src.offset(iclip(src_x - 1, 0 as libc::c_int, src_w - 1) as isize))
                            .as_::<libc::c_int>()
                    + *F.offset(3) as libc::c_int
                        * (*src.offset(iclip(src_x + 0, 0 as libc::c_int, src_w - 1) as isize))
                            .as_::<libc::c_int>()
                    + *F.offset(4) as libc::c_int
                        * (*src.offset(iclip(src_x + 1, 0 as libc::c_int, src_w - 1) as isize))
                            .as_::<libc::c_int>()
                    + *F.offset(5) as libc::c_int
                        * (*src.offset(iclip(src_x + 2, 0 as libc::c_int, src_w - 1) as isize))
                            .as_::<libc::c_int>()
                    + *F.offset(6) as libc::c_int
                        * (*src.offset(iclip(src_x + 3, 0 as libc::c_int, src_w - 1) as isize))
                            .as_::<libc::c_int>()
                    + *F.offset(7) as libc::c_int
                        * (*src.offset(iclip(src_x + 4, 0 as libc::c_int, src_w - 1) as isize))
                            .as_::<libc::c_int>())
                    + 64
                    >> 7,
            );
            mx += dx;
            src_x += mx >> 14;
            mx &= 0x3fff as libc::c_int;
            x += 1;
        }
        dst = dst.offset(BD::pxstride(dst_stride as usize) as isize);
        src = src.offset(BD::pxstride(src_stride as usize) as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    }
}
