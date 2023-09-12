use std::{cmp, iter};

use crate::include::common::bitdepth::DynPixel;
use crate::include::common::bitdepth::{AsPrimitive, BitDepth};
use crate::include::common::intops::iclip;
use crate::include::dav1d::headers::Dav1dFilterMode;
use crate::include::dav1d::headers::DAV1D_FILTER_8TAP_REGULAR;
use crate::include::dav1d::headers::DAV1D_FILTER_8TAP_SHARP;
use crate::include::dav1d::headers::DAV1D_FILTER_8TAP_SMOOTH;
use crate::include::stddef::ptrdiff_t;
use crate::include::stdint::int16_t;
use crate::include::stdint::intptr_t;
use crate::include::stdint::uint8_t;
use crate::src::tables::dav1d_mc_subpel_filters;
use crate::src::tables::dav1d_mc_warp_filter;
use crate::src::tables::dav1d_obmc_masks;
use crate::src::tables::dav1d_resize_filter;

#[inline(never)]
unsafe fn put_rust<BD: BitDepth>(
    dst: *mut BD::Pixel,
    dst_stride: usize,
    src: *const BD::Pixel,
    src_stride: usize,
    w: usize,
    h: usize,
) {
    let [dst_len, src_len] =
        [dst_stride, src_stride].map(|stride| if h == 0 { 0 } else { stride * (h - 1) + w });
    let dst = std::slice::from_raw_parts_mut(dst, dst_len);
    let src = std::slice::from_raw_parts(src, src_len);
    for (dst, src) in iter::zip(dst.chunks_mut(dst_stride), src.chunks(src_stride)) {
        BD::pixel_copy(dst, src, w);
    }
}

#[inline(never)]
unsafe fn prep_rust<BD: BitDepth>(
    tmp: *mut i16,
    src: *const BD::Pixel,
    src_stride: usize,
    w: usize,
    h: usize,
    bd: BD,
) {
    let tmp = std::slice::from_raw_parts_mut(tmp, w * h);
    let src = std::slice::from_raw_parts(src, if h == 0 { 0 } else { src_stride * (h - 1) + w });
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

#[inline(never)]
unsafe fn put_8tap_rust<BD: BitDepth>(
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

#[inline(never)]
unsafe fn put_8tap_scaled_rust<BD: BitDepth>(
    dst: *mut BD::Pixel,
    dst_stride: usize,
    mut src: *const BD::Pixel,
    src_stride: usize,
    w: usize,
    h: usize,
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
unsafe fn prep_8tap_rust<BD: BitDepth>(
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

#[inline(never)]
unsafe fn prep_8tap_scaled_rust<BD: BitDepth>(
    mut tmp: *mut i16,
    mut src: *const BD::Pixel,
    src_stride: usize,
    w: usize,
    h: usize,
    mx: usize,
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

unsafe fn put_bilin_rust<BD: BitDepth>(
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

unsafe fn put_bilin_scaled_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: usize,
    mut src: *const BD::Pixel,
    src_stride: usize,
    w: usize,
    h: usize,
    mx: usize,
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

unsafe fn prep_bilin_rust<BD: BitDepth>(
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

unsafe fn prep_bilin_scaled_rust<BD: BitDepth>(
    mut tmp: *mut i16,
    mut src: *const BD::Pixel,
    src_stride: usize,
    w: usize,
    h: usize,
    mx: usize,
    mut my: usize,
    dx: usize,
    dy: usize,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let src_stride = BD::pxstride(src_stride);
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
            *tmp.offset(x as isize) = (filter_bilin_rnd(mid_ptr.as_ptr(), x, my >> 6, 128, 4)
                - i32::from(BD::PREP_BIAS)) as i16;
        }

        my += dy;
        mid_ptr = &mut mid_ptr[(my >> 10) * 128..];
        my &= 0x3ff;
        tmp = tmp.offset(w as isize);
    }
}

unsafe fn avg_rust<BD: BitDepth>(
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

unsafe fn w_avg_rust<BD: BitDepth>(
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

unsafe fn mask_rust<BD: BitDepth>(
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

unsafe fn blend_rust<BD: BitDepth>(
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

unsafe fn blend_v_rust<BD: BitDepth>(
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

unsafe fn blend_h_rust<BD: BitDepth>(
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

unsafe fn w_mask_rust<BD: BitDepth>(
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
    for (h, ((tmp1, tmp2), dst)) in iter::zip(tmp1.chunks_exact(w), tmp2.chunks_exact(w))
        .zip(dst.chunks_mut(dst_stride))
        .enumerate()
    {
        let mut x = 0;
        while x < w {
            let m = cmp::min(
                38 + (tmp1[x].abs_diff(tmp2[x]).saturating_add(mask_rnd) >> mask_sh),
                64,
            ) as u8;
            dst[x] = bd.iclip_pixel(
                (tmp1[x] as i32 * m as i32 + tmp2[x] as i32 * (64 - m as i32) + rnd) >> sh,
            );

            if ss_hor {
                x += 1;

                let n = cmp::min(
                    38 + (tmp1[x].abs_diff(tmp2[x]).saturating_add(mask_rnd) >> mask_sh),
                    64,
                ) as u8;
                dst[x] = bd.iclip_pixel(
                    (tmp1[x] as i32 * n as i32 + tmp2[x] as i32 * (64 - n as i32) + rnd) >> sh,
                );

                mask[x >> 1] = if h & ss_ver as usize != 0 {
                    (((m + n + 2 - sign) as u16 + mask[x >> 1] as u16) >> 2) as u8
                } else if ss_ver {
                    m + n
                } else {
                    (m + n + 1 - sign) >> 1
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

unsafe fn warp_affine_8x8_rust<BD: BitDepth>(
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
                (dav1d_mc_warp_filter[(64 + (tmx + 512 >> 10)) as usize]).as_ptr();
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
                (dav1d_mc_warp_filter[(64 + (tmy + 512 >> 10)) as usize]).as_ptr();
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

unsafe fn warp_affine_8x8t_rust<BD: BitDepth>(
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
                (dav1d_mc_warp_filter[(64 + (tmx + 512 >> 10)) as usize]).as_ptr();
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
                (dav1d_mc_warp_filter[(64 + (tmy + 512 >> 10)) as usize]).as_ptr();
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

unsafe fn emu_edge_rust<BD: BitDepth>(
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

unsafe fn resize_rust<BD: BitDepth>(
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

pub type mc_fn = Option<
    unsafe extern "C" fn(
        *mut DynPixel,
        ptrdiff_t,
        *const DynPixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mc_scaled_fn = Option<
    unsafe extern "C" fn(
        *mut DynPixel,
        ptrdiff_t,
        *const DynPixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type warp8x8_fn = Option<
    unsafe extern "C" fn(
        *mut DynPixel,
        ptrdiff_t,
        *const DynPixel,
        ptrdiff_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mct_fn = Option<
    unsafe extern "C" fn(
        *mut int16_t,
        *const DynPixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mct_scaled_fn = Option<
    unsafe extern "C" fn(
        *mut int16_t,
        *const DynPixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type warp8x8t_fn = Option<
    unsafe extern "C" fn(
        *mut int16_t,
        ptrdiff_t,
        *const DynPixel,
        ptrdiff_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type avg_fn = Option<
    unsafe extern "C" fn(
        *mut DynPixel,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type w_avg_fn = Option<
    unsafe extern "C" fn(
        *mut DynPixel,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mask_fn = Option<
    unsafe extern "C" fn(
        *mut DynPixel,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        *const uint8_t,
        libc::c_int,
    ) -> (),
>;
pub type w_mask_fn = Option<
    unsafe extern "C" fn(
        *mut DynPixel,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        *mut uint8_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type blend_fn = Option<
    unsafe extern "C" fn(
        *mut DynPixel,
        ptrdiff_t,
        *const DynPixel,
        libc::c_int,
        libc::c_int,
        *const uint8_t,
    ) -> (),
>;
pub type blend_dir_fn = Option<
    unsafe extern "C" fn(*mut DynPixel, ptrdiff_t, *const DynPixel, libc::c_int, libc::c_int) -> (),
>;
pub type emu_edge_fn = Option<
    unsafe extern "C" fn(
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        *mut DynPixel,
        ptrdiff_t,
        *const DynPixel,
        ptrdiff_t,
    ) -> (),
>;
pub type resize_fn = Option<
    unsafe extern "C" fn(
        *mut DynPixel,
        ptrdiff_t,
        *const DynPixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
#[repr(C)]
pub struct Dav1dMCDSPContext {
    pub mc: [mc_fn; 10],
    pub mc_scaled: [mc_scaled_fn; 10],
    pub mct: [mct_fn; 10],
    pub mct_scaled: [mct_scaled_fn; 10],
    pub avg: avg_fn,
    pub w_avg: w_avg_fn,
    pub mask: mask_fn,
    pub w_mask: [w_mask_fn; 3],
    pub blend: blend_fn,
    pub blend_v: blend_dir_fn,
    pub blend_h: blend_dir_fn,
    pub warp8x8: warp8x8_fn,
    pub warp8x8t: warp8x8t_fn,
    pub emu_edge: emu_edge_fn,
    pub resize: resize_fn,
}

macro_rules! filter_fns {
    ($mc_kind:ident, $type_h:expr, $type_v:expr) => {
        paste::paste! {
            // TODO(legare): Temporarily pub until init fns are deduplicated.
            pub(crate) unsafe extern "C" fn [<put_8tap_ $mc_kind _c_erased>]<BD: BitDepth>(
                dst: *mut DynPixel,
                dst_stride: ptrdiff_t,
                src: *const DynPixel,
                src_stride: ptrdiff_t,
                w: libc::c_int,
                h: libc::c_int,
                mx: libc::c_int,
                my: libc::c_int,
                bitdepth_max: libc::c_int,
            ) {
                put_8tap_rust(
                    dst.cast(),
                    dst_stride as usize,
                    src.cast(),
                    src_stride as usize,
                    w as usize,
                    h as usize,
                    mx as usize,
                    my as usize,
                    $type_h | ($type_v << 2),
                    BD::from_c(bitdepth_max),
                );
            }

            // TODO(legare): Temporarily pub until init fns are deduplicated.
            pub(crate) unsafe extern "C" fn [<put_8tap_ $mc_kind _scaled_c_erased>]<BD: BitDepth>(
                dst: *mut DynPixel,
                dst_stride: ptrdiff_t,
                src: *const DynPixel,
                src_stride: ptrdiff_t,
                w: libc::c_int,
                h: libc::c_int,
                mx: libc::c_int,
                my: libc::c_int,
                dx: libc::c_int,
                dy: libc::c_int,
                bitdepth_max: libc::c_int,
            ) {
                put_8tap_scaled_rust(
                    dst.cast(),
                    dst_stride as usize,
                    src.cast(),
                    src_stride as usize,
                    w as usize,
                    h as usize,
                    mx as usize,
                    my as usize,
                    dx as usize,
                    dy as usize,
                    $type_h | ($type_v << 2),
                    BD::from_c(bitdepth_max),
                );
            }

            // TODO(legare): Temporarily pub until init fns are deduplicated.
            pub(crate) unsafe extern "C" fn [<prep_8tap_ $mc_kind _c_erased>]<BD: BitDepth>(
                tmp: *mut int16_t,
                src: *const DynPixel,
                src_stride: ptrdiff_t,
                w: libc::c_int,
                h: libc::c_int,
                mx: libc::c_int,
                my: libc::c_int,
                bitdepth_max: libc::c_int,
            ) {
                prep_8tap_rust(
                    tmp,
                    src.cast(),
                    src_stride as usize,
                    w as usize,
                    h as usize,
                    mx as usize,
                    my as usize,
                    $type_h | ($type_v << 2),
                    BD::from_c(bitdepth_max),
                );
            }

            // TODO(legare): Temporarily pub until init fns are deduplicated.
            pub(crate) unsafe extern "C" fn [<prep_8tap_ $mc_kind _scaled_c_erased>]<BD: BitDepth>(
                tmp: *mut int16_t,
                src: *const DynPixel,
                src_stride: ptrdiff_t,
                w: libc::c_int,
                h: libc::c_int,
                mx: libc::c_int,
                my: libc::c_int,
                dx: libc::c_int,
                dy: libc::c_int,
                bitdepth_max: libc::c_int,
            ) {
                prep_8tap_scaled_rust(
                    tmp,
                    src.cast(),
                    src_stride as usize,
                    w as usize,
                    h as usize,
                    mx as usize,
                    my as usize,
                    dx as usize,
                    dy as usize,
                    $type_h | ($type_v << 2),
                    BD::from_c(bitdepth_max),
                );
            }
        }
    };
}

filter_fns!(
    regular,
    DAV1D_FILTER_8TAP_REGULAR,
    DAV1D_FILTER_8TAP_REGULAR
);
filter_fns!(
    regular_sharp,
    DAV1D_FILTER_8TAP_REGULAR,
    DAV1D_FILTER_8TAP_SHARP
);
filter_fns!(
    regular_smooth,
    DAV1D_FILTER_8TAP_REGULAR,
    DAV1D_FILTER_8TAP_SMOOTH
);
filter_fns!(smooth, DAV1D_FILTER_8TAP_SMOOTH, DAV1D_FILTER_8TAP_SMOOTH);
filter_fns!(
    smooth_regular,
    DAV1D_FILTER_8TAP_SMOOTH,
    DAV1D_FILTER_8TAP_REGULAR
);
filter_fns!(
    smooth_sharp,
    DAV1D_FILTER_8TAP_SMOOTH,
    DAV1D_FILTER_8TAP_SHARP
);
filter_fns!(sharp, DAV1D_FILTER_8TAP_SHARP, DAV1D_FILTER_8TAP_SHARP);
filter_fns!(
    sharp_regular,
    DAV1D_FILTER_8TAP_SHARP,
    DAV1D_FILTER_8TAP_REGULAR
);
filter_fns!(
    sharp_smooth,
    DAV1D_FILTER_8TAP_SHARP,
    DAV1D_FILTER_8TAP_SMOOTH
);

// TODO(legare): Temporarily pub until init fns are deduplicated.
pub(crate) unsafe extern "C" fn put_bilin_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    src: *const DynPixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_bilin_rust(
        dst.cast(),
        dst_stride as usize,
        src.cast(),
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        BD::from_c(bitdepth_max),
    )
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
pub(crate) unsafe extern "C" fn prep_bilin_c_erased<BD: BitDepth>(
    tmp: *mut int16_t,
    src: *const DynPixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_bilin_rust(
        tmp,
        src.cast(),
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        BD::from_c(bitdepth_max),
    )
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
pub(crate) unsafe extern "C" fn put_bilin_scaled_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    src: *const DynPixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_bilin_scaled_rust(
        dst.cast(),
        dst_stride as usize,
        src.cast(),
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        dx as usize,
        dy as usize,
        BD::from_c(bitdepth_max),
    )
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
pub(crate) unsafe extern "C" fn prep_bilin_scaled_c_erased<BD: BitDepth>(
    tmp: *mut int16_t,
    src: *const DynPixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_bilin_scaled_rust(
        tmp,
        src.cast(),
        src_stride as usize,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        dx as usize,
        dy as usize,
        BD::from_c(bitdepth_max),
    )
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
pub(crate) unsafe extern "C" fn avg_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    tmp1: *const int16_t,
    tmp2: *const int16_t,
    w: libc::c_int,
    h: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    avg_rust(
        dst.cast(),
        dst_stride as usize,
        tmp1,
        tmp2,
        w as usize,
        h as usize,
        BD::from_c(bitdepth_max),
    )
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
pub(crate) unsafe extern "C" fn w_avg_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    tmp1: *const int16_t,
    tmp2: *const int16_t,
    w: libc::c_int,
    h: libc::c_int,
    weight: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    w_avg_rust(
        dst.cast(),
        dst_stride as usize,
        tmp1,
        tmp2,
        w as usize,
        h as usize,
        weight,
        BD::from_c(bitdepth_max),
    )
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
pub(crate) unsafe extern "C" fn mask_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    tmp1: *const int16_t,
    tmp2: *const int16_t,
    w: libc::c_int,
    h: libc::c_int,
    mask: *const uint8_t,
    bitdepth_max: libc::c_int,
) {
    mask_rust(
        dst.cast(),
        dst_stride as usize,
        tmp1,
        tmp2,
        w as usize,
        h as usize,
        mask,
        BD::from_c(bitdepth_max),
    )
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
pub(crate) unsafe extern "C" fn w_mask_444_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    tmp1: *const int16_t,
    tmp2: *const int16_t,
    w: libc::c_int,
    h: libc::c_int,
    mask: *mut uint8_t,
    sign: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    debug_assert!(sign == 1 || sign == 0);
    w_mask_rust(
        dst.cast(),
        dst_stride as usize,
        tmp1,
        tmp2,
        w as usize,
        h as usize,
        mask,
        sign != 0,
        false,
        false,
        BD::from_c(bitdepth_max),
    )
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
pub(crate) unsafe extern "C" fn w_mask_422_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    tmp1: *const int16_t,
    tmp2: *const int16_t,
    w: libc::c_int,
    h: libc::c_int,
    mask: *mut uint8_t,
    sign: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    debug_assert!(sign == 1 || sign == 0);
    w_mask_rust(
        dst.cast(),
        dst_stride as usize,
        tmp1,
        tmp2,
        w as usize,
        h as usize,
        mask,
        sign != 0,
        true,
        false,
        BD::from_c(bitdepth_max),
    )
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
pub(crate) unsafe extern "C" fn w_mask_420_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    tmp1: *const int16_t,
    tmp2: *const int16_t,
    w: libc::c_int,
    h: libc::c_int,
    mask: *mut uint8_t,
    sign: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    debug_assert!(sign == 1 || sign == 0);
    w_mask_rust(
        dst.cast(),
        dst_stride as usize,
        tmp1,
        tmp2,
        w as usize,
        h as usize,
        mask,
        sign != 0,
        true,
        true,
        BD::from_c(bitdepth_max),
    );
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
pub(crate) unsafe extern "C" fn blend_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    tmp: *const DynPixel,
    w: libc::c_int,
    h: libc::c_int,
    mask: *const uint8_t,
) {
    blend_rust::<BD>(
        dst.cast(),
        dst_stride as usize,
        tmp.cast(),
        w as usize,
        h as usize,
        mask,
    )
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
pub(crate) unsafe extern "C" fn blend_v_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    tmp: *const DynPixel,
    w: libc::c_int,
    h: libc::c_int,
) {
    blend_v_rust::<BD>(
        dst.cast(),
        dst_stride as usize,
        tmp.cast(),
        w as usize,
        h as usize,
    )
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
pub(crate) unsafe extern "C" fn blend_h_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    tmp: *const DynPixel,
    w: libc::c_int,
    h: libc::c_int,
) {
    blend_h_rust::<BD>(
        dst.cast(),
        dst_stride as usize,
        tmp.cast(),
        w as usize,
        h as usize,
    )
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
pub(crate) unsafe extern "C" fn warp_affine_8x8_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    src: *const DynPixel,
    src_stride: ptrdiff_t,
    abcd: *const int16_t,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    warp_affine_8x8_rust(
        dst.cast(),
        dst_stride,
        src.cast(),
        src_stride,
        abcd,
        mx,
        my,
        BD::from_c(bitdepth_max),
    )
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
pub(crate) unsafe extern "C" fn warp_affine_8x8t_c_erased<BD: BitDepth>(
    tmp: *mut int16_t,
    tmp_stride: ptrdiff_t,
    src: *const DynPixel,
    src_stride: ptrdiff_t,
    abcd: *const int16_t,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    warp_affine_8x8t_rust(
        tmp,
        tmp_stride,
        src.cast(),
        src_stride,
        abcd,
        mx,
        my,
        BD::from_c(bitdepth_max),
    )
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
pub(crate) unsafe extern "C" fn emu_edge_c_erased<BD: BitDepth>(
    bw: intptr_t,
    bh: intptr_t,
    iw: intptr_t,
    ih: intptr_t,
    x: intptr_t,
    y: intptr_t,
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    r#ref: *const DynPixel,
    ref_stride: ptrdiff_t,
) {
    emu_edge_rust::<BD>(
        bw,
        bh,
        iw,
        ih,
        x,
        y,
        dst.cast(),
        dst_stride,
        r#ref.cast(),
        ref_stride,
    )
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
pub(crate) unsafe extern "C" fn resize_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    src: *const DynPixel,
    src_stride: ptrdiff_t,
    dst_w: libc::c_int,
    h: libc::c_int,
    src_w: libc::c_int,
    dx: libc::c_int,
    mx0: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    resize_rust(
        dst.cast(),
        dst_stride,
        src.cast(),
        src_stride,
        dst_w,
        h,
        src_w,
        dx,
        mx0,
        BD::from_c(bitdepth_max),
    )
}

// TODO(legare): Generated fns are temporarily pub until init fns are deduplicated.
#[cfg(feature = "asm")]
macro_rules! decl_fn {
    (mc, $name:ident) => {
        pub(crate) fn $name(
            dst: *mut DynPixel,
            dst_stride: ptrdiff_t,
            src: *const DynPixel,
            src_stride: ptrdiff_t,
            w: libc::c_int,
            h: libc::c_int,
            mx: libc::c_int,
            my: libc::c_int,
            bitdepth_max: libc::c_int,
        );
    };

    (mct, $name:ident) => {
        pub(crate) fn $name(
            tmp: *mut int16_t,
            src: *const DynPixel,
            src_stride: ptrdiff_t,
            w: libc::c_int,
            h: libc::c_int,
            mx: libc::c_int,
            my: libc::c_int,
            bitdepth_max: libc::c_int,
        );
    };

    (mc_scaled, $name:ident) => {
        pub(crate) fn $name(
            dst: *mut DynPixel,
            dst_stride: ptrdiff_t,
            src: *const DynPixel,
            src_stride: ptrdiff_t,
            w: libc::c_int,
            h: libc::c_int,
            mx: libc::c_int,
            my: libc::c_int,
            dx: libc::c_int,
            dy: libc::c_int,
            bitdepth_max: libc::c_int,
        );
    };

    (mct_scaled, $name:ident) => {
        pub(crate) fn $name(
            tmp: *mut int16_t,
            src: *const DynPixel,
            src_stride: ptrdiff_t,
            w: libc::c_int,
            h: libc::c_int,
            mx: libc::c_int,
            my: libc::c_int,
            dx: libc::c_int,
            dy: libc::c_int,
            bitdepth_max: libc::c_int,
        );
    };

    (avg, $name:ident) => {
        pub(crate) fn $name(
            dst: *mut DynPixel,
            dst_stride: ptrdiff_t,
            tmp1: *const int16_t,
            tmp2: *const int16_t,
            w: libc::c_int,
            h: libc::c_int,
            bitdepth_max: libc::c_int,
        );
    };

    (w_avg, $name:ident) => {
        pub(crate) fn $name(
            dst: *mut DynPixel,
            dst_stride: ptrdiff_t,
            tmp1: *const int16_t,
            tmp2: *const int16_t,
            w: libc::c_int,
            h: libc::c_int,
            weight: libc::c_int,
            bitdepth_max: libc::c_int,
        );
    };

    (mask, $name:ident) => {
        pub(crate) fn $name(
            dst: *mut DynPixel,
            dst_stride: ptrdiff_t,
            tmp1: *const int16_t,
            tmp2: *const int16_t,
            w: libc::c_int,
            h: libc::c_int,
            mask: *const uint8_t,
            bitdepth_max: libc::c_int,
        );
    };

    (w_mask, $name:ident) => {
        pub(crate) fn $name(
            dst: *mut DynPixel,
            dst_stride: ptrdiff_t,
            tmp1: *const int16_t,
            tmp2: *const int16_t,
            w: libc::c_int,
            h: libc::c_int,
            mask: *mut uint8_t,
            sign: libc::c_int,
            bitdepth_max: libc::c_int,
        );
    };

    (blend, $name:ident) => {
        pub(crate) fn $name(
            dst: *mut DynPixel,
            dst_stride: ptrdiff_t,
            tmp: *const DynPixel,
            w: libc::c_int,
            h: libc::c_int,
            mask: *const uint8_t,
        );
    };

    (blend_dir, $name:ident) => {
        pub(crate) fn $name(
            dst: *mut DynPixel,
            dst_stride: ptrdiff_t,
            tmp: *const DynPixel,
            w: libc::c_int,
            h: libc::c_int,
        );
    };

    (warp8x8, $name:ident) => {
        pub(crate) fn $name(
            dst: *mut DynPixel,
            dst_stride: ptrdiff_t,
            src: *const DynPixel,
            src_stride: ptrdiff_t,
            abcd: *const int16_t,
            mx: libc::c_int,
            my: libc::c_int,
            bitdepth_max: libc::c_int,
        );
    };

    (warp8x8t, $name:ident) => {
        pub(crate) fn $name(
            tmp: *mut int16_t,
            tmp_stride: ptrdiff_t,
            src: *const DynPixel,
            src_stride: ptrdiff_t,
            abcd: *const int16_t,
            mx: libc::c_int,
            my: libc::c_int,
            bitdepth_max: libc::c_int,
        );
    };

    (emu_edge, $name:ident) => {
        pub(crate) fn $name(
            bw: intptr_t,
            bh: intptr_t,
            iw: intptr_t,
            ih: intptr_t,
            x: intptr_t,
            y: intptr_t,
            dst: *mut DynPixel,
            dst_stride: ptrdiff_t,
            src: *const DynPixel,
            src_stride: ptrdiff_t,
        );
    };

    (resize, $name:ident) => {
        pub(crate) fn $name(
            dst: *mut DynPixel,
            dst_stride: ptrdiff_t,
            src: *const DynPixel,
            src_stride: ptrdiff_t,
            dst_w: libc::c_int,
            h: libc::c_int,
            src_w: libc::c_int,
            dx: libc::c_int,
            mx: libc::c_int,
            bitdepth_max: libc::c_int,
        );
    };
}

#[cfg(feature = "asm")]
macro_rules! decl_fns {
    ($fn_kind:ident, $name:ident, $asm:ident) => {
        paste::paste! {
            #[cfg(feature = "bitdepth_8")]
            decl_fn!($fn_kind, [<$name _8bpc_ $asm>]);
            #[cfg(feature = "bitdepth_16")]
            decl_fn!($fn_kind, [<$name _16bpc_ $asm>]);
        }
    };

    ($fn_kind:ident, $name:ident) => {
        decl_fns!($fn_kind, $name, sse2);
        decl_fns!($fn_kind, $name, ssse3);

        #[cfg(target_arch = "x86_64")]
        decl_fns!($fn_kind, $name, avx2);

        #[cfg(target_arch = "x86_64")]
        decl_fns!($fn_kind, $name, avx512icl);
    };
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(dead_code)] // Macro invocations generate more fn declarations than are actually used.
extern "C" {
    decl_fns!(mc, dav1d_put_8tap_regular);
    decl_fns!(mc, dav1d_put_8tap_regular_smooth);
    decl_fns!(mc, dav1d_put_8tap_regular_sharp);
    decl_fns!(mc, dav1d_put_8tap_smooth);
    decl_fns!(mc, dav1d_put_8tap_smooth_regular);
    decl_fns!(mc, dav1d_put_8tap_smooth_sharp);
    decl_fns!(mc, dav1d_put_8tap_sharp);
    decl_fns!(mc, dav1d_put_8tap_sharp_regular);
    decl_fns!(mc, dav1d_put_8tap_sharp_smooth);
    decl_fns!(mc, dav1d_put_bilin);

    decl_fns!(mct, dav1d_prep_8tap_regular);
    decl_fns!(mct, dav1d_prep_8tap_regular_smooth);
    decl_fns!(mct, dav1d_prep_8tap_regular_sharp);
    decl_fns!(mct, dav1d_prep_8tap_smooth);
    decl_fns!(mct, dav1d_prep_8tap_smooth_regular);
    decl_fns!(mct, dav1d_prep_8tap_smooth_sharp);
    decl_fns!(mct, dav1d_prep_8tap_sharp);
    decl_fns!(mct, dav1d_prep_8tap_sharp_regular);
    decl_fns!(mct, dav1d_prep_8tap_sharp_smooth);
    decl_fns!(mct, dav1d_prep_bilin);

    decl_fns!(mc_scaled, dav1d_put_8tap_scaled_regular);
    decl_fns!(mc_scaled, dav1d_put_8tap_scaled_regular_smooth);
    decl_fns!(mc_scaled, dav1d_put_8tap_scaled_regular_sharp);
    decl_fns!(mc_scaled, dav1d_put_8tap_scaled_smooth);
    decl_fns!(mc_scaled, dav1d_put_8tap_scaled_smooth_regular);
    decl_fns!(mc_scaled, dav1d_put_8tap_scaled_smooth_sharp);
    decl_fns!(mc_scaled, dav1d_put_8tap_scaled_sharp);
    decl_fns!(mc_scaled, dav1d_put_8tap_scaled_sharp_regular);
    decl_fns!(mc_scaled, dav1d_put_8tap_scaled_sharp_smooth);
    decl_fns!(mc_scaled, dav1d_put_bilin_scaled);

    decl_fns!(mct_scaled, dav1d_prep_8tap_scaled_regular);
    decl_fns!(mct_scaled, dav1d_prep_8tap_scaled_regular_smooth);
    decl_fns!(mct_scaled, dav1d_prep_8tap_scaled_regular_sharp);
    decl_fns!(mct_scaled, dav1d_prep_8tap_scaled_smooth);
    decl_fns!(mct_scaled, dav1d_prep_8tap_scaled_smooth_regular);
    decl_fns!(mct_scaled, dav1d_prep_8tap_scaled_smooth_sharp);
    decl_fns!(mct_scaled, dav1d_prep_8tap_scaled_sharp);
    decl_fns!(mct_scaled, dav1d_prep_8tap_scaled_sharp_regular);
    decl_fns!(mct_scaled, dav1d_prep_8tap_scaled_sharp_smooth);
    decl_fns!(mct_scaled, dav1d_prep_bilin_scaled);

    decl_fns!(avg, dav1d_avg);
    decl_fns!(w_avg, dav1d_w_avg);
    decl_fns!(mask, dav1d_mask);
    decl_fns!(w_mask, dav1d_w_mask_420);
    decl_fns!(w_mask, dav1d_w_mask_422);
    decl_fns!(w_mask, dav1d_w_mask_444);
    decl_fns!(blend, dav1d_blend);
    decl_fns!(blend_dir, dav1d_blend_v);
    decl_fns!(blend_dir, dav1d_blend_h);

    decl_fns!(warp8x8, dav1d_warp_affine_8x8);
    decl_fns!(warp8x8, dav1d_warp_affine_8x8, sse4);
    decl_fns!(warp8x8t, dav1d_warp_affine_8x8t);
    decl_fns!(warp8x8t, dav1d_warp_affine_8x8t, sse4);

    decl_fns!(emu_edge, dav1d_emu_edge);
    decl_fns!(resize, dav1d_resize);
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
extern "C" {
    decl_fns!(mc, dav1d_put_8tap_regular, neon);
    decl_fns!(mc, dav1d_put_8tap_regular_smooth, neon);
    decl_fns!(mc, dav1d_put_8tap_regular_sharp, neon);
    decl_fns!(mc, dav1d_put_8tap_smooth, neon);
    decl_fns!(mc, dav1d_put_8tap_smooth_regular, neon);
    decl_fns!(mc, dav1d_put_8tap_smooth_sharp, neon);
    decl_fns!(mc, dav1d_put_8tap_sharp, neon);
    decl_fns!(mc, dav1d_put_8tap_sharp_regular, neon);
    decl_fns!(mc, dav1d_put_8tap_sharp_smooth, neon);
    decl_fns!(mc, dav1d_put_bilin, neon);

    decl_fns!(mct, dav1d_prep_8tap_regular, neon);
    decl_fns!(mct, dav1d_prep_8tap_regular_smooth, neon);
    decl_fns!(mct, dav1d_prep_8tap_regular_sharp, neon);
    decl_fns!(mct, dav1d_prep_8tap_smooth, neon);
    decl_fns!(mct, dav1d_prep_8tap_smooth_regular, neon);
    decl_fns!(mct, dav1d_prep_8tap_smooth_sharp, neon);
    decl_fns!(mct, dav1d_prep_8tap_sharp, neon);
    decl_fns!(mct, dav1d_prep_8tap_sharp_regular, neon);
    decl_fns!(mct, dav1d_prep_8tap_sharp_smooth, neon);
    decl_fns!(mct, dav1d_prep_bilin, neon);

    decl_fns!(avg, dav1d_avg, neon);
    decl_fns!(w_avg, dav1d_w_avg, neon);
    decl_fns!(mask, dav1d_mask, neon);
    decl_fns!(w_mask, dav1d_w_mask_420, neon);
    decl_fns!(w_mask, dav1d_w_mask_422, neon);
    decl_fns!(w_mask, dav1d_w_mask_444, neon);
    decl_fns!(blend, dav1d_blend, neon);
    decl_fns!(blend_dir, dav1d_blend_v, neon);
    decl_fns!(blend_dir, dav1d_blend_h, neon);

    decl_fns!(warp8x8, dav1d_warp_affine_8x8, neon);
    decl_fns!(warp8x8t, dav1d_warp_affine_8x8t, neon);

    decl_fns!(emu_edge, dav1d_emu_edge, neon);
}
