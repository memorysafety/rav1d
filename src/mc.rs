use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::intops::clip;
use crate::include::common::intops::iclip;
use crate::include::dav1d::headers::Rav1dFilterMode;
use crate::include::dav1d::headers::Rav1dPixelLayoutSubSampled;
use crate::src::cpu::CpuFlags;
use crate::src::enum_map::enum_map;
use crate::src::enum_map::enum_map_ty;
use crate::src::enum_map::DefaultValue;
use crate::src::internal::COMPINTER_LEN;
use crate::src::internal::EMU_EDGE_LEN;
use crate::src::levels::Filter2d;
use crate::src::tables::dav1d_mc_subpel_filters;
use crate::src::tables::dav1d_mc_warp_filter;
use crate::src::tables::dav1d_obmc_masks;
use crate::src::tables::dav1d_resize_filter;
use crate::src::wrap_fn_ptr::wrap_fn_ptr;
use libc::intptr_t;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::iter;
use std::slice;
use to_method::To;

#[cfg(all(
    feature = "asm",
    not(any(target_arch = "riscv64", target_arch = "riscv32"))
))]
use crate::include::common::bitdepth::bd_fn;

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
use crate::include::common::bitdepth::{bpc_fn, BPC};

#[inline(never)]
unsafe fn put_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: isize,
    mut src: *const BD::Pixel,
    src_stride: isize,
    w: usize,
    h: usize,
) {
    for _ in 0..h {
        BD::pixel_copy(
            slice::from_raw_parts_mut(dst, w),
            slice::from_raw_parts(src, w),
            w,
        );

        dst = dst.offset(dst_stride);
        src = src.offset(src_stride);
    }
}

#[inline(never)]
unsafe fn prep_rust<BD: BitDepth>(
    tmp: &mut [i16],
    mut src: *const BD::Pixel,
    src_stride: isize,
    w: usize,
    h: usize,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    for tmp in tmp.chunks_exact_mut(w).take(h) {
        for x in 0..w {
            tmp[x] =
                (((*src.add(x)).as_::<i32>() << intermediate_bits) - (BD::PREP_BIAS as i32)) as i16
        }
        src = src.offset(src_stride);
    }
}

unsafe fn filter_8tap<T: Into<i32>>(src: *const T, x: usize, f: &[i8; 8], stride: isize) -> i32 {
    f.into_iter()
        .enumerate()
        .map(|(i, &f)| {
            let [i, x] = [i, x].map(|it| it as isize);
            let j = x + (i - 3) * stride;
            i32::from(f) * src.offset(j).read().into()
        })
        .sum()
}

unsafe fn rav1d_filter_8tap_rnd<T: Into<i32>>(
    src: *const T,
    x: usize,
    f: &[i8; 8],
    stride: isize,
    sh: u8,
) -> i32 {
    (filter_8tap(src, x, f, stride) + ((1 << sh) >> 1)) >> sh
}

unsafe fn rav1d_filter_8tap_rnd2<T: Into<i32>>(
    src: *const T,
    x: usize,
    f: &[i8; 8],
    stride: isize,
    rnd: u8,
    sh: u8,
) -> i32 {
    (filter_8tap(src, x, f, stride) + (rnd as i32)) >> sh
}

unsafe fn rav1d_filter_8tap_clip<BD: BitDepth, T: Into<i32>>(
    bd: BD,
    src: *const T,
    x: usize,
    f: &[i8; 8],
    stride: isize,
    sh: u8,
) -> BD::Pixel {
    bd.iclip_pixel(rav1d_filter_8tap_rnd(src, x, f, stride, sh))
}

unsafe fn rav1d_filter_8tap_clip2<BD: BitDepth, T: Into<i32>>(
    bd: BD,
    src: *const T,
    x: usize,
    f: &[i8; 8],
    stride: isize,
    rnd: u8,
    sh: u8,
) -> BD::Pixel {
    bd.iclip_pixel(rav1d_filter_8tap_rnd2(src, x, f, stride, rnd, sh))
}

fn get_filter(m: usize, d: usize, filter_type: Rav1dFilterMode) -> Option<&'static [i8; 8]> {
    let m = m.checked_sub(1)?;
    let i = if d > 4 {
        filter_type as u8
    } else {
        3 + (filter_type as u8 & 1)
    };
    Some(&dav1d_mc_subpel_filters[i as usize][m])
}

#[inline(never)]
unsafe fn put_8tap_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: isize,
    mut src: *const BD::Pixel,
    src_stride: isize,
    w: usize,
    h: usize,
    mx: usize,
    my: usize,
    (h_filter_type, v_filter_type): (Rav1dFilterMode, Rav1dFilterMode),
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let intermediate_rnd = 32 + (1 << 6 - intermediate_bits >> 1);

    let fh = get_filter(mx, w, h_filter_type);
    let fv = get_filter(my, h, v_filter_type);

    let [dst_stride, src_stride] = [dst_stride, src_stride].map(BD::pxstride);

    if let Some(fh) = fh {
        if let Some(fv) = fv {
            let tmp_h = h + 7;
            let mut mid = [0i16; 128 * 135]; // Default::default()
            let mut mid_ptr = &mut mid[..];

            src = src.offset(-((src_stride * 3) as isize));
            for _ in 0..tmp_h {
                for x in 0..w {
                    mid_ptr[x] = rav1d_filter_8tap_rnd(src, x, fh, 1, 6 - intermediate_bits) as i16;
                }

                mid_ptr = &mut mid_ptr[128..];
                src = src.offset(src_stride as isize);
            }

            mid_ptr = &mut mid[128 * 3..];
            for _ in 0..h {
                for x in 0..w {
                    *dst.add(x) = rav1d_filter_8tap_clip(
                        bd,
                        mid_ptr.as_ptr(),
                        x,
                        fv,
                        128,
                        6 + intermediate_bits,
                    );
                }

                mid_ptr = &mut mid_ptr[128..];
                dst = dst.offset(dst_stride);
            }
        } else {
            for _ in 0..h {
                for x in 0..w {
                    *dst.add(x) = rav1d_filter_8tap_clip2(bd, src, x, fh, 1, intermediate_rnd, 6);
                }

                dst = dst.offset(dst_stride);
                src = src.offset(src_stride);
            }
        }
    } else if let Some(fv) = fv {
        for _ in 0..h {
            for x in 0..w {
                *dst.add(x) = rav1d_filter_8tap_clip(bd, src, x, fv, src_stride, 6);
            }

            dst = dst.offset(dst_stride);
            src = src.offset(src_stride);
        }
    } else {
        put_rust::<BD>(dst, dst_stride, src, src_stride, w, h);
    }
}

#[inline(never)]
unsafe fn put_8tap_scaled_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: isize,
    mut src: *const BD::Pixel,
    src_stride: isize,
    w: usize,
    h: usize,
    mx: usize,
    mut my: usize,
    dx: usize,
    dy: usize,
    (h_filter_type, v_filter_type): (Rav1dFilterMode, Rav1dFilterMode),
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let intermediate_rnd = (1 << intermediate_bits) >> 1;
    let tmp_h = ((h - 1) * dy + my >> 10) + 8;
    let mut mid = [0i16; 128 * (256 + 7)]; // Default::default()
    let mut mid_ptr = &mut mid[..];
    let [dst_stride, src_stride] = [dst_stride, src_stride].map(BD::pxstride);

    src = src.offset(-src_stride * 3);
    for _ in 0..tmp_h {
        let mut imx = mx;
        let mut ioff = 0;

        for x in 0..w {
            let fh = get_filter(imx >> 6, w, h_filter_type);
            mid_ptr[x] = match fh {
                Some(fh) => rav1d_filter_8tap_rnd(src, ioff, fh, 1, 6 - intermediate_bits) as i16,
                None => ((*src.offset(ioff as isize)).as_::<i32>() as i16) << intermediate_bits,
            };
            imx += dx;
            ioff += imx >> 10;
            imx &= 0x3ff;
        }

        mid_ptr = &mut mid_ptr[128..];
        src = src.offset(src_stride);
    }
    mid_ptr = &mut mid[128 * 3..];
    for _ in 0..h {
        let fv = get_filter(my >> 6, h, v_filter_type);

        for x in 0..w {
            *dst.add(x) = match fv {
                Some(fv) => {
                    rav1d_filter_8tap_clip(bd, mid_ptr.as_ptr(), x, fv, 128, 6 + intermediate_bits)
                }
                None => {
                    bd.iclip_pixel((i32::from(mid_ptr[x]) + intermediate_rnd) >> intermediate_bits)
                }
            };
        }

        my += dy;
        mid_ptr = &mut mid_ptr[(my >> 10) * 128..];
        my &= 0x3ff;
        dst = dst.offset(dst_stride);
    }
}

#[inline(never)]
unsafe fn prep_8tap_rust<BD: BitDepth>(
    mut tmp: &mut [i16],
    mut src: *const BD::Pixel,
    src_stride: isize,
    w: usize,
    h: usize,
    mx: usize,
    my: usize,
    (h_filter_type, v_filter_type): (Rav1dFilterMode, Rav1dFilterMode),
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let fh = get_filter(mx, w, h_filter_type);
    let fv = get_filter(my, h, v_filter_type);
    let src_stride = BD::pxstride(src_stride);

    if let Some(fh) = fh {
        if let Some(fv) = fv {
            let tmp_h = h + 7;
            let mut mid = [0i16; 128 * 135]; // Default::default()
            let mut mid_ptr = &mut mid[..];

            src = src.offset(-src_stride * 3);
            for _ in 0..tmp_h {
                for x in 0..w {
                    mid_ptr[x] = rav1d_filter_8tap_rnd(src, x, fh, 1, 6 - intermediate_bits) as i16;
                }

                mid_ptr = &mut mid_ptr[128..];
                src = src.offset(src_stride);
            }

            mid_ptr = &mut mid[128 * 3..];
            for _ in 0..h {
                for x in 0..w {
                    tmp[x] = (rav1d_filter_8tap_rnd(mid_ptr.as_ptr(), x, fv, 128, 6)
                        - i32::from(BD::PREP_BIAS))
                    .try_into()
                    .unwrap();
                }

                mid_ptr = &mut mid_ptr[128..];
                tmp = &mut tmp[w..]
            }
        } else {
            for _ in 0..h {
                for x in 0..w {
                    tmp[x] = (rav1d_filter_8tap_rnd(src, x, fh, 1, 6 - intermediate_bits)
                        - i32::from(BD::PREP_BIAS)) as i16;
                }

                tmp = &mut tmp[w..];
                src = src.offset(src_stride);
            }
        }
    } else if let Some(fv) = fv {
        for _ in 0..h {
            for x in 0..w {
                tmp[x] = (rav1d_filter_8tap_rnd(src, x, fv, src_stride, 6 - intermediate_bits)
                    - i32::from(BD::PREP_BIAS)) as i16;
            }

            tmp = &mut tmp[w..];
            src = src.offset(src_stride);
        }
    } else {
        prep_rust(tmp, src, src_stride, w, h, bd);
    };
}

#[inline(never)]
unsafe fn prep_8tap_scaled_rust<BD: BitDepth>(
    mut tmp: *mut i16,
    mut src: *const BD::Pixel,
    src_stride: isize,
    w: usize,
    h: usize,
    mx: usize,
    mut my: usize,
    dx: usize,
    dy: usize,
    (h_filter_type, v_filter_type): (Rav1dFilterMode, Rav1dFilterMode),
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let tmp_h = ((h - 1) * dy + my >> 10) + 8;
    let mut mid = [0i16; 128 * (256 + 7)]; // Default::default()
    let mut mid_ptr = &mut mid[..];
    let src_stride = BD::pxstride(src_stride);

    src = src.offset(-src_stride * 3);
    for _ in 0..tmp_h {
        let mut imx = mx;
        let mut ioff = 0;
        for x in 0..w {
            let fh = get_filter(imx >> 6, w, h_filter_type);
            mid_ptr[x] = match fh {
                Some(fh) => rav1d_filter_8tap_rnd(src, ioff, fh, 1, 6 - intermediate_bits) as i16,
                None => ((*src.offset(ioff as isize)).as_::<i32>() as i16) << intermediate_bits,
            };
            imx += dx;
            ioff += imx >> 10;
            imx &= 0x3ff;
        }

        mid_ptr = &mut mid_ptr[128..];
        src = src.offset(src_stride);
    }

    mid_ptr = &mut mid[128 * 3..];
    for _ in 0..h {
        let fv = get_filter(my >> 6, h, v_filter_type);
        for x in 0..w {
            *tmp.offset(x as isize) = ((match fv {
                Some(fv) => rav1d_filter_8tap_rnd(mid_ptr.as_ptr(), x, fv, 128, 6),
                None => i32::from(mid_ptr[x]),
            }) - i32::from(BD::PREP_BIAS)) as i16;
        }
        my += dy;
        mid_ptr = &mut mid_ptr[(my >> 10) * 128..];
        my &= 0x3ff;
        tmp = tmp.offset(w as isize);
    }
}

unsafe fn filter_bilin<T: Into<i32>>(src: *const T, x: usize, mxy: usize, stride: isize) -> i32 {
    let x = x as isize;
    let src = |i| -> i32 { src.offset(i).read().into() };
    16 * src(x) + ((mxy as i32) * (src(x + stride) - src(x)))
}

unsafe fn filter_bilin_rnd<T: Into<i32>>(
    src: *const T,
    x: usize,
    mxy: usize,
    stride: isize,
    sh: u8,
) -> i32 {
    (filter_bilin(src, x, mxy, stride) + ((1 << sh) >> 1)) >> sh
}

unsafe fn filter_bilin_clip<BD: BitDepth, T: Into<i32>>(
    bd: BD,
    src: *const T,
    x: usize,
    mxy: usize,
    stride: isize,
    sh: u8,
) -> BD::Pixel {
    bd.iclip_pixel(filter_bilin_rnd(src, x, mxy, stride, sh))
}

unsafe fn put_bilin_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: isize,
    mut src: *const BD::Pixel,
    src_stride: isize,
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
                src = src.offset(src_stride);
            }
            mid_ptr = &mut mid[..];
            for _ in 0..h {
                for x in 0..w {
                    *dst.offset(x as isize) =
                        filter_bilin_clip(bd, mid_ptr.as_ptr(), x, my, 128, 4 + intermediate_bits);
                }

                mid_ptr = &mut mid_ptr[128..];
                dst = dst.offset(dst_stride);
            }
        } else {
            for _ in 0..h {
                for x in 0..w {
                    let px = filter_bilin_rnd(src, x, mx, 1, 4 - intermediate_bits);
                    *dst.offset(x as isize) =
                        bd.iclip_pixel((px + intermediate_rnd) >> intermediate_bits);
                }

                dst = dst.offset(dst_stride);
                src = src.offset(src_stride);
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
    mut tmp: &mut [i16],
    mut src: *const BD::Pixel,
    src_stride: isize,
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
                src = src.offset(src_stride);
            }
            mid_ptr = &mut mid[..];
            for _ in 0..h {
                for x in 0..w {
                    tmp[x] = (filter_bilin_rnd(mid_ptr.as_ptr(), x, my, 128, 4)
                        - i32::from(BD::PREP_BIAS)) as i16;
                }

                mid_ptr = &mut mid_ptr[128..];
                tmp = &mut tmp[w..];
            }
        } else {
            for _ in 0..h {
                for x in 0..w {
                    tmp[x] = (filter_bilin_rnd(src, x, mx, 1, 4 - intermediate_bits)
                        - i32::from(BD::PREP_BIAS)) as i16;
                }

                tmp = &mut tmp[w..];
                src = src.offset(src_stride);
            }
        }
    } else if my != 0 {
        for _ in 0..h {
            for x in 0..w {
                tmp[x] = (filter_bilin_rnd(src, x, my, src_stride, 4 - intermediate_bits)
                    - i32::from(BD::PREP_BIAS)) as i16;
            }

            tmp = &mut tmp[w..];
            src = src.offset(src_stride);
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
    dst_stride: isize,
    tmp1: &[i16; COMPINTER_LEN],
    tmp2: &[i16; COMPINTER_LEN],
    w: usize,
    h: usize,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let sh = intermediate_bits + 1;
    let rnd = (1 << intermediate_bits) + i32::from(BD::PREP_BIAS) * 2;
    let dst_stride = BD::pxstride(dst_stride);
    let mut tmp1 = tmp1.as_slice();
    let mut tmp2 = tmp2.as_slice();
    for _ in 0..h {
        for x in 0..w {
            *dst.offset(x as isize) =
                bd.iclip_pixel(((tmp1[x] as i32 + tmp2[x] as i32 + rnd) >> sh).to::<i32>());
        }

        tmp1 = &tmp1[w..];
        tmp2 = &tmp2[w..];
        dst = dst.offset(dst_stride);
    }
}

unsafe fn w_avg_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: isize,
    tmp1: &[i16; COMPINTER_LEN],
    tmp2: &[i16; COMPINTER_LEN],
    w: usize,
    h: usize,
    weight: i32,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let sh = intermediate_bits + 4;
    let rnd = (8 << intermediate_bits) + i32::from(BD::PREP_BIAS) * 16;
    let dst_stride = BD::pxstride(dst_stride);
    let mut tmp1 = tmp1.as_slice();
    let mut tmp2 = tmp2.as_slice();
    for _ in 0..h {
        for x in 0..w {
            *dst.offset(x as isize) = bd.iclip_pixel(
                (tmp1[x] as i32 * weight + tmp2[x] as i32 * (16 - weight) + rnd) >> sh,
            );
        }

        tmp1 = &tmp1[w..];
        tmp2 = &tmp2[w..];
        dst = dst.offset(dst_stride);
    }
}

unsafe fn mask_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: isize,
    tmp1: &[i16; COMPINTER_LEN],
    tmp2: &[i16; COMPINTER_LEN],
    w: usize,
    h: usize,
    mut mask: *const u8,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let sh = intermediate_bits + 6;
    let rnd = (32 << intermediate_bits) + i32::from(BD::PREP_BIAS) * 64;
    let dst_stride = BD::pxstride(dst_stride);
    let mut tmp1 = tmp1.as_slice();
    let mut tmp2 = tmp2.as_slice();
    for _ in 0..h {
        for x in 0..w {
            *dst.offset(x as isize) = bd.iclip_pixel(
                (tmp1[x] as i32 * *mask.offset(x as isize) as i32
                    + tmp2[x] as i32 * (64 - *mask.offset(x as isize) as i32)
                    + rnd)
                    >> sh,
            );
        }

        tmp1 = &tmp1[w..];
        tmp2 = &tmp2[w..];
        mask = mask.offset(w as isize);
        dst = dst.offset(dst_stride);
    }
}

fn blend_px<BD: BitDepth>(a: BD::Pixel, b: BD::Pixel, m: u8) -> BD::Pixel {
    let m = m as u32;
    ((a.as_::<u32>() * (64 - m) + b.as_::<u32>() * m + 32) >> 6).as_::<BD::Pixel>()
}

unsafe fn blend_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: isize,
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

        dst = dst.offset(dst_stride);
        tmp = tmp.offset(w as isize);
        mask = mask.offset(w as isize);
    }
}

unsafe fn blend_v_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: isize,
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

        dst = dst.offset(dst_stride);
        tmp = tmp.offset(w as isize);
    }
}

unsafe fn blend_h_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: isize,
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

        dst = dst.offset(dst_stride);
        tmp = tmp.offset(w as isize);
    }
}

unsafe fn w_mask_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: isize,
    tmp1: &[i16; COMPINTER_LEN],
    tmp2: &[i16; COMPINTER_LEN],
    w: usize,
    h: usize,
    mask: *mut u8,
    sign: bool,
    ss_hor: bool,
    ss_ver: bool,
    bd: BD,
) {
    let dst_stride = BD::pxstride(dst_stride);
    let mut mask = slice::from_raw_parts_mut(mask, (w >> ss_hor as usize) * (h >> ss_ver as usize));
    let sign = sign as u8;

    // store mask at 2x2 resolution, i.e. store 2x1 sum for even rows,
    // and then load this intermediate to calculate final value for odd rows
    let intermediate_bits = bd.get_intermediate_bits();
    let bitdepth = bd.bitdepth();
    let sh = intermediate_bits + 6;
    let rnd = (32 << intermediate_bits) + i32::from(BD::PREP_BIAS) * 64;
    let mask_sh = bitdepth + intermediate_bits - 4;
    let mask_rnd = 1 << (mask_sh - 5);
    for (h, (tmp1, tmp2)) in iter::zip(tmp1.chunks_exact(w), tmp2.chunks_exact(w))
        .take(h)
        .enumerate()
    {
        let mut x = 0;
        while x < w {
            let m = cmp::min(
                38 + (tmp1[x].abs_diff(tmp2[x]).saturating_add(mask_rnd) >> mask_sh),
                64,
            ) as u8;
            *dst.add(x) = bd.iclip_pixel(
                (tmp1[x] as i32 * m as i32 + tmp2[x] as i32 * (64 - m as i32) + rnd) >> sh,
            );

            if ss_hor {
                x += 1;

                let n = cmp::min(
                    38 + (tmp1[x].abs_diff(tmp2[x]).saturating_add(mask_rnd) >> mask_sh),
                    64,
                ) as u8;
                *dst.add(x) = bd.iclip_pixel(
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

        dst = dst.offset(dst_stride);
        if !ss_ver || h & 1 != 0 {
            mask = &mut mask[w >> ss_hor as usize..];
        }
    }
}

unsafe fn warp_affine_8x8_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: ptrdiff_t,
    mut src: *const BD::Pixel,
    src_stride: ptrdiff_t,
    abcd: &[i16; 4],
    mx: c_int,
    my: c_int,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let mut mid = [[0; 8]; 15];

    src = src.offset(-3 * BD::pxstride(src_stride));
    for (y, mid) in mid.iter_mut().enumerate() {
        let mx = mx + y as c_int * abcd[1] as c_int;
        for (x, mid) in mid.iter_mut().enumerate() {
            let tmx = mx + x as c_int * abcd[0] as c_int;
            let filter = &dav1d_mc_warp_filter[(64 + (tmx + 512 >> 10)) as usize];
            *mid = (filter[0] as c_int * (*src.offset(x as isize - 3 * 1)).as_::<c_int>()
                + filter[1] as c_int * (*src.offset(x as isize - 2 * 1)).as_::<c_int>()
                + filter[2] as c_int * (*src.offset(x as isize - 1 * 1)).as_::<c_int>()
                + filter[3] as c_int * (*src.offset(x as isize + 0 * 1)).as_::<c_int>()
                + filter[4] as c_int * (*src.offset(x as isize + 1 * 1)).as_::<c_int>()
                + filter[5] as c_int * (*src.offset(x as isize + 2 * 1)).as_::<c_int>()
                + filter[6] as c_int * (*src.offset(x as isize + 3 * 1)).as_::<c_int>()
                + filter[7] as c_int * (*src.offset(x as isize + 4 * 1)).as_::<c_int>()
                + (1 << 7 - intermediate_bits >> 1)
                >> 7 - intermediate_bits) as i16;
        }
        src = src.offset(BD::pxstride(src_stride));
    }

    for y in 0..8 {
        let my = my + y as c_int * abcd[3] as c_int;
        for x in 0..8 {
            let tmy = my + x as c_int * abcd[2] as c_int;
            let filter = &dav1d_mc_warp_filter[(64 + (tmy + 512 >> 10)) as usize];
            *dst.add(x) = bd.iclip_pixel(
                filter[0] as c_int * mid[y + 0][x] as c_int
                    + filter[1] as c_int * mid[y + 1][x] as c_int
                    + filter[2] as c_int * mid[y + 2][x] as c_int
                    + filter[3] as c_int * mid[y + 3][x] as c_int
                    + filter[4] as c_int * mid[y + 4][x] as c_int
                    + filter[5] as c_int * mid[y + 5][x] as c_int
                    + filter[6] as c_int * mid[y + 6][x] as c_int
                    + filter[7] as c_int * mid[y + 7][x] as c_int
                    + (1 << 7 + intermediate_bits >> 1)
                    >> 7 + intermediate_bits,
            );
        }
        dst = dst.offset(BD::pxstride(dst_stride));
    }
}

unsafe fn warp_affine_8x8t_rust<BD: BitDepth>(
    tmp: &mut [i16],
    tmp_stride: usize,
    mut src: *const BD::Pixel,
    src_stride: ptrdiff_t,
    abcd: &[i16; 4],
    mx: c_int,
    my: c_int,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let mut mid = [[0; 8]; 15];

    src = src.offset(-3 * BD::pxstride(src_stride));
    for (y, mid) in mid.iter_mut().enumerate() {
        let mx = mx + y as c_int * abcd[1] as c_int;
        for (x, mid) in mid.iter_mut().enumerate() {
            let tmx = mx + x as c_int * abcd[0] as c_int;
            let filter = &dav1d_mc_warp_filter[(64 + (tmx + 512 >> 10)) as usize];
            *mid = (filter[0] as c_int * (*src.offset(x as isize - 3 * 1)).as_::<c_int>()
                + filter[1] as c_int * (*src.offset(x as isize - 2 * 1)).as_::<c_int>()
                + filter[2] as c_int * (*src.offset(x as isize - 1 * 1)).as_::<c_int>()
                + filter[3] as c_int * (*src.offset(x as isize + 0 * 1)).as_::<c_int>()
                + filter[4] as c_int * (*src.offset(x as isize + 1 * 1)).as_::<c_int>()
                + filter[5] as c_int * (*src.offset(x as isize + 2 * 1)).as_::<c_int>()
                + filter[6] as c_int * (*src.offset(x as isize + 3 * 1)).as_::<c_int>()
                + filter[7] as c_int * (*src.offset(x as isize + 4 * 1)).as_::<c_int>()
                + (1 << 7 - intermediate_bits >> 1)
                >> 7 - intermediate_bits) as i16;
        }
        src = src.offset(BD::pxstride(src_stride));
    }

    for y in 0..8 {
        let tmp = &mut tmp[y * tmp_stride..];
        let my = my + y as c_int * abcd[3] as c_int;
        for x in 0..8 {
            let tmy = my + x as c_int * abcd[2] as c_int;
            let filter = &dav1d_mc_warp_filter[(64 + (tmy + 512 >> 10)) as usize];
            tmp[x] = ((filter[0] as c_int * mid[y + 0][x] as c_int
                + filter[1] as c_int * mid[y + 1][x] as c_int
                + filter[2] as c_int * mid[y + 2][x] as c_int
                + filter[3] as c_int * mid[y + 3][x] as c_int
                + filter[4] as c_int * mid[y + 4][x] as c_int
                + filter[5] as c_int * mid[y + 5][x] as c_int
                + filter[6] as c_int * mid[y + 6][x] as c_int
                + filter[7] as c_int * mid[y + 7][x] as c_int
                + (1 << 7 >> 1)
                >> 7)
                - i32::from(BD::PREP_BIAS)) as i16;
        }
    }
}

unsafe fn emu_edge_rust<BD: BitDepth>(
    bw: intptr_t,
    bh: intptr_t,
    iw: intptr_t,
    ih: intptr_t,
    x: intptr_t,
    y: intptr_t,
    dst: *mut [BD::Pixel; EMU_EDGE_LEN],
    dst_stride: usize,
    mut r#ref: *const BD::Pixel,
    ref_stride: ptrdiff_t,
) {
    let dst = &mut *dst;
    let dst_stride = BD::pxstride(dst_stride);
    let ref_stride = BD::pxstride(ref_stride);

    // find offset in reference of visible block to copy
    r#ref = r#ref.offset(clip(y, 0, ih - 1) * ref_stride + clip(x, 0, iw - 1));

    // number of pixels to extend (left, right, top, bottom)
    let left_ext = clip(-x, 0, bw - 1) as usize;
    let right_ext = clip(x + bw - iw, 0, bw - 1) as usize;
    assert!(((left_ext + right_ext) as isize) < bw);
    let top_ext = clip(-y, 0, bh - 1) as usize;
    let bottom_ext = clip(y + bh - ih, 0, bh - 1) as usize;
    assert!(((top_ext + bottom_ext) as isize) < bh);

    let bw = bw as usize;
    let bh = bh as usize;

    // copy visible portion first
    let mut blk = top_ext * dst_stride;
    let center_w = bw - left_ext - right_ext;
    let center_h = bh - top_ext - bottom_ext;
    for _ in 0..center_h {
        BD::pixel_copy(
            &mut dst[blk + left_ext..][..center_w],
            slice::from_raw_parts(r#ref, center_w),
            center_w,
        );
        // extend left edge for this line
        if left_ext != 0 {
            let val = dst[blk + left_ext];
            BD::pixel_set(&mut dst[blk..], val, left_ext);
        }
        // extend right edge for this line
        if right_ext != 0 {
            let val = dst[blk + left_ext + center_w - 1];
            BD::pixel_set(&mut dst[blk + left_ext + center_w..], val, right_ext);
        }
        r#ref = r#ref.offset(ref_stride);
        blk += dst_stride;
    }

    // copy top
    let mut dst_off = 0;
    let blk = top_ext * dst_stride;
    let (front, back) = dst.split_at_mut(blk);
    for _ in 0..top_ext {
        BD::pixel_copy(&mut front[dst_off..][..bw], &back[..bw], bw);
        dst_off += dst_stride;
    }

    // copy bottom
    dst_off += center_h * dst_stride;
    for _ in 0..bottom_ext {
        let (front, back) = dst.split_at_mut(dst_off);
        BD::pixel_copy(&mut back[..bw], &front[dst_off - dst_stride..][..bw], bw);
        dst_off += dst_stride;
    }
}

unsafe fn resize_rust<BD: BitDepth>(
    mut dst_ptr: *mut BD::Pixel,
    dst_stride: ptrdiff_t,
    mut src_ptr: *const BD::Pixel,
    src_stride: ptrdiff_t,
    dst_w: c_int,
    h: c_int,
    src_w: c_int,
    dx: c_int,
    mx0: c_int,
    bd: BD,
) {
    let max = src_w - 1;
    for _ in 0..h {
        let mut mx = mx0;
        let mut src_x = -1;
        let src = slice::from_raw_parts(src_ptr, src_w as usize);
        let dst = slice::from_raw_parts_mut(dst_ptr, dst_w as usize);
        for dst in dst {
            let F = &dav1d_resize_filter[(mx >> 8) as usize];
            *dst = bd.iclip_pixel(
                -(F[0] as c_int * src[iclip(src_x - 3, 0, max) as usize].as_::<c_int>()
                    + F[1] as c_int * src[iclip(src_x - 2, 0, max) as usize].as_::<c_int>()
                    + F[2] as c_int * src[iclip(src_x - 1, 0, max) as usize].as_::<c_int>()
                    + F[3] as c_int * src[iclip(src_x + 0, 0, max) as usize].as_::<c_int>()
                    + F[4] as c_int * src[iclip(src_x + 1, 0, max) as usize].as_::<c_int>()
                    + F[5] as c_int * src[iclip(src_x + 2, 0, max) as usize].as_::<c_int>()
                    + F[6] as c_int * src[iclip(src_x + 3, 0, max) as usize].as_::<c_int>()
                    + F[7] as c_int * src[iclip(src_x + 4, 0, max) as usize].as_::<c_int>())
                    + 64
                    >> 7,
            );
            mx += dx;
            src_x += mx >> 14;
            mx &= 0x3fff;
        }
        dst_ptr = dst_ptr.offset(BD::pxstride(dst_stride));
        src_ptr = src_ptr.offset(BD::pxstride(src_stride));
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn mc(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    src: *const DynPixel,
    src_stride: ptrdiff_t,
    w: c_int,
    h: c_int,
    mx: c_int,
    my: c_int,
    bitdepth_max: c_int,
) -> ());

impl mc::Fn {
    pub unsafe fn call<BD: BitDepth>(
        &self,
        dst: *mut BD::Pixel,
        dst_stride: ptrdiff_t,
        src: *const BD::Pixel,
        src_stride: ptrdiff_t,
        w: c_int,
        h: c_int,
        mx: c_int,
        my: c_int,
        bd: BD,
    ) {
        let dst = dst.cast();
        let src = src.cast();
        let bd = bd.into_c();
        self.get()(dst, dst_stride, src, src_stride, w, h, mx, my, bd)
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn mc_scaled(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    src: *const DynPixel,
    src_stride: ptrdiff_t,
    w: c_int,
    h: c_int,
    mx: c_int,
    my: c_int,
    dx: c_int,
    dy: c_int,
    bitdepth_max: c_int,
) -> ());

impl mc_scaled::Fn {
    pub unsafe fn call<BD: BitDepth>(
        &self,
        dst: *mut BD::Pixel,
        dst_stride: ptrdiff_t,
        src: *const BD::Pixel,
        src_stride: ptrdiff_t,
        w: c_int,
        h: c_int,
        mx: c_int,
        my: c_int,
        dx: c_int,
        dy: c_int,
        bd: BD,
    ) {
        let dst = dst.cast();
        let src = src.cast();
        let bd = bd.into_c();
        self.get()(dst, dst_stride, src, src_stride, w, h, mx, my, dx, dy, bd)
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn warp8x8(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    src: *const DynPixel,
    src_stride: ptrdiff_t,
    abcd: &[i16; 4],
    mx: c_int,
    my: c_int,
    bitdepth_max: c_int,
) -> ());

impl warp8x8::Fn {
    pub unsafe fn call<BD: BitDepth>(
        &self,
        dst: *mut BD::Pixel,
        dst_stride: ptrdiff_t,
        src: *const BD::Pixel,
        src_stride: ptrdiff_t,
        abcd: &[i16; 4],
        mx: c_int,
        my: c_int,
        bd: BD,
    ) {
        let dst = dst.cast();
        let src = src.cast();
        let bd = bd.into_c();
        self.get()(dst, dst_stride, src, src_stride, abcd, mx, my, bd)
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn mct(
    tmp: *mut i16,
    src: *const DynPixel,
    src_stride: ptrdiff_t,
    w: c_int,
    h: c_int,
    mx: c_int,
    my: c_int,
    bitdepth_max: c_int,
) -> ());

impl mct::Fn {
    pub unsafe fn call<BD: BitDepth>(
        &self,
        tmp: &mut [i16],
        src: *const BD::Pixel,
        src_stride: ptrdiff_t,
        w: c_int,
        h: c_int,
        mx: c_int,
        my: c_int,
        bd: BD,
    ) {
        let tmp = tmp.as_mut_ptr();
        let src = src.cast();
        let bd = bd.into_c();
        self.get()(tmp, src, src_stride, w, h, mx, my, bd)
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn mct_scaled(
    tmp: *mut i16,
    src: *const DynPixel,
    src_stride: ptrdiff_t,
    w: c_int,
    h: c_int,
    mx: c_int,
    my: c_int,
    dx: c_int,
    dy: c_int,
    bitdepth_max: c_int,
) -> ());

impl mct_scaled::Fn {
    pub unsafe fn call<BD: BitDepth>(
        &self,
        tmp: &mut [i16],
        src: *const BD::Pixel,
        src_stride: ptrdiff_t,
        w: c_int,
        h: c_int,
        mx: c_int,
        my: c_int,
        dx: c_int,
        dy: c_int,
        bd: BD,
    ) {
        let tmp = tmp.as_mut_ptr();
        let src = src.cast();
        let bd = bd.into_c();
        self.get()(tmp, src, src_stride, w, h, mx, my, dx, dy, bd)
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn warp8x8t(
    tmp: *mut i16,
    tmp_stride: usize,
    src: *const DynPixel,
    src_stride: ptrdiff_t,
    abcd: &[i16; 4],
    mx: c_int,
    my: c_int,
    bitdepth_max: c_int,
    tmp_len: usize,
) -> ());

impl warp8x8t::Fn {
    pub unsafe fn call<BD: BitDepth>(
        &self,
        tmp: &mut [i16],
        tmp_stride: usize,
        src: *const BD::Pixel,
        src_stride: ptrdiff_t,
        abcd: &[i16; 4],
        mx: c_int,
        my: c_int,
        bd: BD,
    ) {
        let tmp_len = tmp.len();
        let tmp = tmp.as_mut_ptr();
        let src = src.cast();
        let bd = bd.into_c();
        self.get()(tmp, tmp_stride, src, src_stride, abcd, mx, my, bd, tmp_len)
    }
}

pub type avg_fn = unsafe extern "C" fn(
    *mut DynPixel,
    ptrdiff_t,
    &[i16; COMPINTER_LEN],
    &[i16; COMPINTER_LEN],
    c_int,
    c_int,
    c_int,
) -> ();

pub type w_avg_fn = unsafe extern "C" fn(
    *mut DynPixel,
    ptrdiff_t,
    &[i16; COMPINTER_LEN],
    &[i16; COMPINTER_LEN],
    c_int,
    c_int,
    c_int,
    c_int,
) -> ();

pub type mask_fn = unsafe extern "C" fn(
    *mut DynPixel,
    ptrdiff_t,
    &[i16; COMPINTER_LEN],
    &[i16; COMPINTER_LEN],
    c_int,
    c_int,
    *const u8,
    c_int,
) -> ();

wrap_fn_ptr!(pub unsafe extern "C" fn w_mask(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    tmp1: &[i16; COMPINTER_LEN],
    tmp2: &[i16; COMPINTER_LEN],
    w: c_int,
    h: c_int,
    mask: *mut u8,
    sign: c_int,
    bitdepth_max: c_int,
) -> ());

impl w_mask::Fn {
    pub unsafe fn call<BD: BitDepth>(
        &self,
        dst: *mut BD::Pixel,
        dst_stride: ptrdiff_t,
        tmp1: &[i16; COMPINTER_LEN],
        tmp2: &[i16; COMPINTER_LEN],
        w: c_int,
        h: c_int,
        mask: *mut u8,
        sign: c_int,
        bd: BD,
    ) {
        let dst = dst.cast();
        let bd = bd.into_c();
        self.get()(dst, dst_stride, tmp1, tmp2, w, h, mask, sign, bd)
    }
}

pub type blend_fn =
    unsafe extern "C" fn(*mut DynPixel, ptrdiff_t, *const DynPixel, c_int, c_int, *const u8) -> ();

pub type blend_dir_fn =
    unsafe extern "C" fn(*mut DynPixel, ptrdiff_t, *const DynPixel, c_int, c_int) -> ();

pub type emu_edge_fn = unsafe extern "C" fn(
    intptr_t,
    intptr_t,
    intptr_t,
    intptr_t,
    intptr_t,
    intptr_t,
    *mut [DynPixel; EMU_EDGE_LEN],
    usize,
    *const DynPixel,
    ptrdiff_t,
) -> ();

pub type resize_fn = unsafe extern "C" fn(
    *mut DynPixel,
    ptrdiff_t,
    *const DynPixel,
    ptrdiff_t,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
) -> ();

pub struct Rav1dMCDSPContext {
    pub mc: enum_map_ty!(Filter2d, mc::Fn),
    pub mc_scaled: enum_map_ty!(Filter2d, mc_scaled::Fn),
    pub mct: enum_map_ty!(Filter2d, mct::Fn),
    pub mct_scaled: enum_map_ty!(Filter2d, mct_scaled::Fn),
    pub avg: avg_fn,
    pub w_avg: w_avg_fn,
    pub mask: mask_fn,
    pub w_mask: enum_map_ty!(Rav1dPixelLayoutSubSampled, w_mask::Fn),
    pub blend: blend_fn,
    pub blend_v: blend_dir_fn,
    pub blend_h: blend_dir_fn,
    pub warp8x8: warp8x8::Fn,
    pub warp8x8t: warp8x8t::Fn,
    pub emu_edge: emu_edge_fn,
    pub resize: resize_fn,
}

macro_rules! filter_fns {
    ($mc_kind:ident, $type_h:expr, $type_v:expr) => {
        paste::paste! {
            unsafe extern "C" fn [<put_8tap_ $mc_kind _c_erased>]<BD: BitDepth>(
                dst: *mut DynPixel,
                dst_stride: ptrdiff_t,
                src: *const DynPixel,
                src_stride: ptrdiff_t,
                w: c_int,
                h: c_int,
                mx: c_int,
                my: c_int,
                bitdepth_max: c_int,
            ) {
                put_8tap_rust(
                    dst.cast(),
                    dst_stride,
                    src.cast(),
                    src_stride,
                    w as usize,
                    h as usize,
                    mx as usize,
                    my as usize,
                    ($type_h, $type_v),
                    BD::from_c(bitdepth_max),
                );
            }

            unsafe extern "C" fn [<put_8tap_ $mc_kind _scaled_c_erased>]<BD: BitDepth>(
                dst: *mut DynPixel,
                dst_stride: ptrdiff_t,
                src: *const DynPixel,
                src_stride: ptrdiff_t,
                w: c_int,
                h: c_int,
                mx: c_int,
                my: c_int,
                dx: c_int,
                dy: c_int,
                bitdepth_max: c_int,
            ) {
                put_8tap_scaled_rust(
                    dst.cast(),
                    dst_stride,
                    src.cast(),
                    src_stride,
                    w as usize,
                    h as usize,
                    mx as usize,
                    my as usize,
                    dx as usize,
                    dy as usize,
                    ($type_h, $type_v),
                    BD::from_c(bitdepth_max),
                );
            }

            unsafe extern "C" fn [<prep_8tap_ $mc_kind _c_erased>]<BD: BitDepth>(
                tmp: *mut i16,
                src: *const DynPixel,
                src_stride: ptrdiff_t,
                w: c_int,
                h: c_int,
                mx: c_int,
                my: c_int,
                bitdepth_max: c_int,
            ) {
                let tmp = std::slice::from_raw_parts_mut(tmp, (w * h) as usize);
                prep_8tap_rust(
                    tmp,
                    src.cast(),
                    src_stride,
                    w as usize,
                    h as usize,
                    mx as usize,
                    my as usize,
                    ($type_h, $type_v),
                    BD::from_c(bitdepth_max),
                );
            }

            unsafe extern "C" fn [<prep_8tap_ $mc_kind _scaled_c_erased>]<BD: BitDepth>(
                tmp: *mut i16,
                src: *const DynPixel,
                src_stride: ptrdiff_t,
                w: c_int,
                h: c_int,
                mx: c_int,
                my: c_int,
                dx: c_int,
                dy: c_int,
                bitdepth_max: c_int,
            ) {
                prep_8tap_scaled_rust(
                    tmp,
                    src.cast(),
                    src_stride,
                    w as usize,
                    h as usize,
                    mx as usize,
                    my as usize,
                    dx as usize,
                    dy as usize,
                    ($type_h, $type_v),
                    BD::from_c(bitdepth_max),
                );
            }
        }
    };
}

filter_fns!(
    regular,
    Rav1dFilterMode::Regular8Tap,
    Rav1dFilterMode::Regular8Tap
);
filter_fns!(
    regular_sharp,
    Rav1dFilterMode::Regular8Tap,
    Rav1dFilterMode::Sharp8Tap
);
filter_fns!(
    regular_smooth,
    Rav1dFilterMode::Regular8Tap,
    Rav1dFilterMode::Smooth8Tap
);
filter_fns!(
    smooth,
    Rav1dFilterMode::Smooth8Tap,
    Rav1dFilterMode::Smooth8Tap
);
filter_fns!(
    smooth_regular,
    Rav1dFilterMode::Smooth8Tap,
    Rav1dFilterMode::Regular8Tap
);
filter_fns!(
    smooth_sharp,
    Rav1dFilterMode::Smooth8Tap,
    Rav1dFilterMode::Sharp8Tap
);
filter_fns!(
    sharp,
    Rav1dFilterMode::Sharp8Tap,
    Rav1dFilterMode::Sharp8Tap
);
filter_fns!(
    sharp_regular,
    Rav1dFilterMode::Sharp8Tap,
    Rav1dFilterMode::Regular8Tap
);
filter_fns!(
    sharp_smooth,
    Rav1dFilterMode::Sharp8Tap,
    Rav1dFilterMode::Smooth8Tap
);

unsafe extern "C" fn put_bilin_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    src: *const DynPixel,
    src_stride: ptrdiff_t,
    w: c_int,
    h: c_int,
    mx: c_int,
    my: c_int,
    bitdepth_max: c_int,
) {
    put_bilin_rust(
        dst.cast(),
        dst_stride,
        src.cast(),
        src_stride,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        BD::from_c(bitdepth_max),
    )
}

unsafe extern "C" fn prep_bilin_c_erased<BD: BitDepth>(
    tmp: *mut i16,
    src: *const DynPixel,
    src_stride: ptrdiff_t,
    w: c_int,
    h: c_int,
    mx: c_int,
    my: c_int,
    bitdepth_max: c_int,
) {
    let tmp = std::slice::from_raw_parts_mut(tmp, (w * h) as usize);
    prep_bilin_rust(
        tmp,
        src.cast(),
        src_stride,
        w as usize,
        h as usize,
        mx as usize,
        my as usize,
        BD::from_c(bitdepth_max),
    )
}

unsafe extern "C" fn put_bilin_scaled_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    src: *const DynPixel,
    src_stride: ptrdiff_t,
    w: c_int,
    h: c_int,
    mx: c_int,
    my: c_int,
    dx: c_int,
    dy: c_int,
    bitdepth_max: c_int,
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

unsafe extern "C" fn prep_bilin_scaled_c_erased<BD: BitDepth>(
    tmp: *mut i16,
    src: *const DynPixel,
    src_stride: ptrdiff_t,
    w: c_int,
    h: c_int,
    mx: c_int,
    my: c_int,
    dx: c_int,
    dy: c_int,
    bitdepth_max: c_int,
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

unsafe extern "C" fn avg_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    tmp1: &[i16; COMPINTER_LEN],
    tmp2: &[i16; COMPINTER_LEN],
    w: c_int,
    h: c_int,
    bitdepth_max: c_int,
) {
    avg_rust(
        dst.cast(),
        dst_stride,
        tmp1,
        tmp2,
        w as usize,
        h as usize,
        BD::from_c(bitdepth_max),
    )
}

unsafe extern "C" fn w_avg_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    tmp1: &[i16; COMPINTER_LEN],
    tmp2: &[i16; COMPINTER_LEN],
    w: c_int,
    h: c_int,
    weight: c_int,
    bitdepth_max: c_int,
) {
    w_avg_rust(
        dst.cast(),
        dst_stride,
        tmp1,
        tmp2,
        w as usize,
        h as usize,
        weight,
        BD::from_c(bitdepth_max),
    )
}

unsafe extern "C" fn mask_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    tmp1: &[i16; COMPINTER_LEN],
    tmp2: &[i16; COMPINTER_LEN],
    w: c_int,
    h: c_int,
    mask: *const u8,
    bitdepth_max: c_int,
) {
    mask_rust(
        dst.cast(),
        dst_stride,
        tmp1,
        tmp2,
        w as usize,
        h as usize,
        mask,
        BD::from_c(bitdepth_max),
    )
}

unsafe extern "C" fn w_mask_444_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    tmp1: &[i16; COMPINTER_LEN],
    tmp2: &[i16; COMPINTER_LEN],
    w: c_int,
    h: c_int,
    mask: *mut u8,
    sign: c_int,
    bitdepth_max: c_int,
) {
    debug_assert!(sign == 1 || sign == 0);
    w_mask_rust(
        dst.cast(),
        dst_stride,
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

unsafe extern "C" fn w_mask_422_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    tmp1: &[i16; COMPINTER_LEN],
    tmp2: &[i16; COMPINTER_LEN],
    w: c_int,
    h: c_int,
    mask: *mut u8,
    sign: c_int,
    bitdepth_max: c_int,
) {
    debug_assert!(sign == 1 || sign == 0);
    w_mask_rust(
        dst.cast(),
        dst_stride,
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

unsafe extern "C" fn w_mask_420_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    tmp1: &[i16; COMPINTER_LEN],
    tmp2: &[i16; COMPINTER_LEN],
    w: c_int,
    h: c_int,
    mask: *mut u8,
    sign: c_int,
    bitdepth_max: c_int,
) {
    debug_assert!(sign == 1 || sign == 0);
    w_mask_rust(
        dst.cast(),
        dst_stride,
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

unsafe extern "C" fn blend_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    tmp: *const DynPixel,
    w: c_int,
    h: c_int,
    mask: *const u8,
) {
    blend_rust::<BD>(
        dst.cast(),
        dst_stride,
        tmp.cast(),
        w as usize,
        h as usize,
        mask,
    )
}

unsafe extern "C" fn blend_v_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    tmp: *const DynPixel,
    w: c_int,
    h: c_int,
) {
    blend_v_rust::<BD>(dst.cast(), dst_stride, tmp.cast(), w as usize, h as usize)
}

unsafe extern "C" fn blend_h_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    tmp: *const DynPixel,
    w: c_int,
    h: c_int,
) {
    blend_h_rust::<BD>(dst.cast(), dst_stride, tmp.cast(), w as usize, h as usize)
}

unsafe extern "C" fn warp_affine_8x8_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    src: *const DynPixel,
    src_stride: ptrdiff_t,
    abcd: &[i16; 4],
    mx: c_int,
    my: c_int,
    bitdepth_max: c_int,
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

unsafe extern "C" fn warp_affine_8x8t_c_erased<BD: BitDepth>(
    tmp: *mut i16,
    tmp_stride: usize,
    src: *const DynPixel,
    src_stride: ptrdiff_t,
    abcd: &[i16; 4],
    mx: c_int,
    my: c_int,
    bitdepth_max: c_int,
    tmp_len: usize,
) {
    let tmp = slice::from_raw_parts_mut(tmp, tmp_len);
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

unsafe extern "C" fn emu_edge_c_erased<BD: BitDepth>(
    bw: intptr_t,
    bh: intptr_t,
    iw: intptr_t,
    ih: intptr_t,
    x: intptr_t,
    y: intptr_t,
    dst: *mut [DynPixel; EMU_EDGE_LEN],
    dst_stride: usize,
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

unsafe extern "C" fn resize_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    src: *const DynPixel,
    src_stride: ptrdiff_t,
    dst_w: c_int,
    h: c_int,
    src_w: c_int,
    dx: c_int,
    mx0: c_int,
    bitdepth_max: c_int,
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

#[cfg(all(
    feature = "asm",
    not(any(target_arch = "riscv64", target_arch = "riscv32"))
))]
macro_rules! decl_fn {
    (avg, $name:ident) => {
        fn $name(
            dst: *mut DynPixel,
            dst_stride: ptrdiff_t,
            tmp1: &[i16; COMPINTER_LEN],
            tmp2: &[i16; COMPINTER_LEN],
            w: c_int,
            h: c_int,
            bitdepth_max: c_int,
        );
    };

    (w_avg, $name:ident) => {
        fn $name(
            dst: *mut DynPixel,
            dst_stride: ptrdiff_t,
            tmp1: &[i16; COMPINTER_LEN],
            tmp2: &[i16; COMPINTER_LEN],
            w: c_int,
            h: c_int,
            weight: c_int,
            bitdepth_max: c_int,
        );
    };

    (mask, $name:ident) => {
        fn $name(
            dst: *mut DynPixel,
            dst_stride: ptrdiff_t,
            tmp1: &[i16; COMPINTER_LEN],
            tmp2: &[i16; COMPINTER_LEN],
            w: c_int,
            h: c_int,
            mask: *const u8,
            bitdepth_max: c_int,
        );
    };

    (blend, $name:ident) => {
        fn $name(
            dst: *mut DynPixel,
            dst_stride: ptrdiff_t,
            tmp: *const DynPixel,
            w: c_int,
            h: c_int,
            mask: *const u8,
        );
    };

    (blend_dir, $name:ident) => {
        fn $name(
            dst: *mut DynPixel,
            dst_stride: ptrdiff_t,
            tmp: *const DynPixel,
            w: c_int,
            h: c_int,
        );
    };

    (emu_edge, $name:ident) => {
        fn $name(
            bw: intptr_t,
            bh: intptr_t,
            iw: intptr_t,
            ih: intptr_t,
            x: intptr_t,
            y: intptr_t,
            dst: *mut [DynPixel; EMU_EDGE_LEN],
            dst_stride: usize,
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
            dst_w: c_int,
            h: c_int,
            src_w: c_int,
            dx: c_int,
            mx: c_int,
            bitdepth_max: c_int,
        );
    };
}

#[cfg(all(
    feature = "asm",
    not(any(target_arch = "riscv64", target_arch = "riscv32"))
))]
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
    decl_fns!(avg, dav1d_avg);
    decl_fns!(w_avg, dav1d_w_avg);
    decl_fns!(mask, dav1d_mask);
    decl_fns!(blend, dav1d_blend);
    decl_fns!(blend_dir, dav1d_blend_v);
    decl_fns!(blend_dir, dav1d_blend_h);

    decl_fns!(emu_edge, dav1d_emu_edge);
    decl_fns!(resize, dav1d_resize);
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
extern "C" {
    decl_fns!(avg, dav1d_avg, neon);
    decl_fns!(w_avg, dav1d_w_avg, neon);
    decl_fns!(mask, dav1d_mask, neon);
    decl_fns!(blend, dav1d_blend, neon);
    decl_fns!(blend_dir, dav1d_blend_v, neon);
    decl_fns!(blend_dir, dav1d_blend_h, neon);

    decl_fns!(emu_edge, dav1d_emu_edge, neon);
}

impl Rav1dMCDSPContext {
    pub const fn default<BD: BitDepth>() -> Self {
        Self {
            mc: enum_map!(Filter2d => mc::Fn; match key {
                Regular8Tap => mc::Fn::new(put_8tap_regular_c_erased::<BD>),
                RegularSmooth8Tap => mc::Fn::new(put_8tap_regular_smooth_c_erased::<BD>),
                RegularSharp8Tap => mc::Fn::new(put_8tap_regular_sharp_c_erased::<BD>),
                SharpRegular8Tap => mc::Fn::new(put_8tap_sharp_regular_c_erased::<BD>),
                SharpSmooth8Tap => mc::Fn::new(put_8tap_sharp_smooth_c_erased::<BD>),
                Sharp8Tap => mc::Fn::new(put_8tap_sharp_c_erased::<BD>),
                SmoothRegular8Tap => mc::Fn::new(put_8tap_smooth_regular_c_erased::<BD>),
                Smooth8Tap => mc::Fn::new(put_8tap_smooth_c_erased::<BD>),
                SmoothSharp8Tap => mc::Fn::new(put_8tap_smooth_sharp_c_erased::<BD>),
                Bilinear => mc::Fn::new(put_bilin_c_erased::<BD>),
            }),
            mct: enum_map!(Filter2d => mct::Fn; match key {
                Regular8Tap => mct::Fn::new(prep_8tap_regular_c_erased::<BD>),
                RegularSmooth8Tap => mct::Fn::new(prep_8tap_regular_smooth_c_erased::<BD>),
                RegularSharp8Tap => mct::Fn::new(prep_8tap_regular_sharp_c_erased::<BD>),
                SharpRegular8Tap => mct::Fn::new(prep_8tap_sharp_regular_c_erased::<BD>),
                SharpSmooth8Tap => mct::Fn::new(prep_8tap_sharp_smooth_c_erased::<BD>),
                Sharp8Tap => mct::Fn::new(prep_8tap_sharp_c_erased::<BD>),
                SmoothRegular8Tap => mct::Fn::new(prep_8tap_smooth_regular_c_erased::<BD>),
                Smooth8Tap => mct::Fn::new(prep_8tap_smooth_c_erased::<BD>),
                SmoothSharp8Tap => mct::Fn::new(prep_8tap_smooth_sharp_c_erased::<BD>),
                Bilinear => mct::Fn::new(prep_bilin_c_erased::<BD>),
            }),
            mc_scaled: enum_map!(Filter2d => mc_scaled::Fn; match key {
                Regular8Tap => mc_scaled::Fn::new(put_8tap_regular_scaled_c_erased::<BD>),
                RegularSmooth8Tap => mc_scaled::Fn::new(put_8tap_regular_smooth_scaled_c_erased::<BD>),
                RegularSharp8Tap => mc_scaled::Fn::new(put_8tap_regular_sharp_scaled_c_erased::<BD>),
                SharpRegular8Tap => mc_scaled::Fn::new(put_8tap_sharp_regular_scaled_c_erased::<BD>),
                SharpSmooth8Tap => mc_scaled::Fn::new(put_8tap_sharp_smooth_scaled_c_erased::<BD>),
                Sharp8Tap => mc_scaled::Fn::new(put_8tap_sharp_scaled_c_erased::<BD>),
                SmoothRegular8Tap => mc_scaled::Fn::new(put_8tap_smooth_regular_scaled_c_erased::<BD>),
                Smooth8Tap => mc_scaled::Fn::new(put_8tap_smooth_scaled_c_erased::<BD>),
                SmoothSharp8Tap => mc_scaled::Fn::new(put_8tap_smooth_sharp_scaled_c_erased::<BD>),
                Bilinear => mc_scaled::Fn::new(put_bilin_scaled_c_erased::<BD>),
            }),
            mct_scaled: enum_map!(Filter2d => mct_scaled::Fn; match key {
                Regular8Tap => mct_scaled::Fn::new(prep_8tap_regular_scaled_c_erased::<BD>),
                RegularSmooth8Tap => mct_scaled::Fn::new(prep_8tap_regular_smooth_scaled_c_erased::<BD>),
                RegularSharp8Tap => mct_scaled::Fn::new(prep_8tap_regular_sharp_scaled_c_erased::<BD>),
                SharpRegular8Tap => mct_scaled::Fn::new(prep_8tap_sharp_regular_scaled_c_erased::<BD>),
                SharpSmooth8Tap => mct_scaled::Fn::new(prep_8tap_sharp_smooth_scaled_c_erased::<BD>),
                Sharp8Tap => mct_scaled::Fn::new(prep_8tap_sharp_scaled_c_erased::<BD>),
                SmoothRegular8Tap => mct_scaled::Fn::new(prep_8tap_smooth_regular_scaled_c_erased::<BD>),
                Smooth8Tap => mct_scaled::Fn::new(prep_8tap_smooth_scaled_c_erased::<BD>),
                SmoothSharp8Tap => mct_scaled::Fn::new(prep_8tap_smooth_sharp_scaled_c_erased::<BD>),
                Bilinear => mct_scaled::Fn::new(prep_bilin_scaled_c_erased::<BD>),
            }),
            avg: avg_c_erased::<BD>,
            w_avg: w_avg_c_erased::<BD>,
            mask: mask_c_erased::<BD>,
            w_mask: enum_map!(Rav1dPixelLayoutSubSampled => w_mask::Fn; match key {
                I420 => w_mask::Fn::new(w_mask_420_c_erased::<BD>),
                I422 => w_mask::Fn::new(w_mask_422_c_erased::<BD>),
                I444 => w_mask::Fn::new(w_mask_444_c_erased::<BD>),
            }),
            blend: blend_c_erased::<BD>,
            blend_v: blend_v_c_erased::<BD>,
            blend_h: blend_h_c_erased::<BD>,
            warp8x8: warp8x8::Fn::new(warp_affine_8x8_c_erased::<BD>),
            warp8x8t: warp8x8t::Fn::new(warp_affine_8x8t_c_erased::<BD>),
            emu_edge: emu_edge_c_erased::<BD>,
            resize: resize_c_erased::<BD>,
        }
    }

    #[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
    #[inline(always)]
    const fn init_x86<BD: BitDepth>(mut self, flags: CpuFlags) -> Self {
        if !flags.contains(CpuFlags::SSE2) {
            return self;
        }

        if let BPC::BPC8 = BD::BPC {
            self.mct = enum_map!(Filter2d => mct::Fn; match key {
                Bilinear => bpc_fn!(mct::decl_fn, 8 bpc, prep_bilin, sse2),
                Regular8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_regular, sse2),
                RegularSmooth8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_regular_smooth, sse2),
                RegularSharp8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_regular_sharp, sse2),
                SmoothRegular8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_smooth_regular, sse2),
                Smooth8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_smooth, sse2),
                SmoothSharp8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_smooth_sharp, sse2),
                SharpRegular8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_sharp_regular, sse2),
                SharpSmooth8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_sharp_smooth, sse2),
                Sharp8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_sharp, sse2),
            });

            self.warp8x8 = bpc_fn!(warp8x8::decl_fn, 8 bpc, warp_affine_8x8, sse2);
            self.warp8x8t = bpc_fn!(warp8x8t::decl_fn, 8 bpc, warp_affine_8x8t, sse2);
        }

        if !flags.contains(CpuFlags::SSSE3) {
            return self;
        }

        self.mc = enum_map!(Filter2d => mc::Fn; match key {
            Regular8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_regular, ssse3),
            RegularSmooth8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_regular_smooth, ssse3),
            RegularSharp8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_regular_sharp, ssse3),
            SmoothRegular8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_smooth_regular, ssse3),
            Smooth8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_smooth, ssse3),
            SmoothSharp8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_smooth_sharp, ssse3),
            SharpRegular8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_sharp_regular, ssse3),
            SharpSmooth8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_sharp_smooth, ssse3),
            Sharp8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_sharp, ssse3),
            Bilinear => bd_fn!(mc::decl_fn, BD, put_bilin, ssse3),
        });
        self.mct = enum_map!(Filter2d => mct::Fn; match key {
            Regular8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_regular, ssse3),
            RegularSmooth8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_regular_smooth, ssse3),
            RegularSharp8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_regular_sharp, ssse3),
            SmoothRegular8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_smooth_regular, ssse3),
            Smooth8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_smooth, ssse3),
            SmoothSharp8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_smooth_sharp, ssse3),
            SharpRegular8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_sharp_regular, ssse3),
            SharpSmooth8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_sharp_smooth, ssse3),
            Sharp8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_sharp, ssse3),
            Bilinear => bd_fn!(mct::decl_fn, BD, prep_bilin, ssse3),
        });
        self.mc_scaled = enum_map!(Filter2d => mc_scaled::Fn; match key {
            Regular8Tap => bd_fn!(mc_scaled::decl_fn, BD, put_8tap_scaled_regular, ssse3),
            RegularSmooth8Tap => bd_fn!(mc_scaled::decl_fn, BD, put_8tap_scaled_regular_smooth, ssse3),
            RegularSharp8Tap => bd_fn!(mc_scaled::decl_fn, BD, put_8tap_scaled_regular_sharp, ssse3),
            SmoothRegular8Tap => bd_fn!(mc_scaled::decl_fn, BD, put_8tap_scaled_smooth_regular, ssse3),
            Smooth8Tap => bd_fn!(mc_scaled::decl_fn, BD, put_8tap_scaled_smooth, ssse3),
            SmoothSharp8Tap => bd_fn!(mc_scaled::decl_fn, BD, put_8tap_scaled_smooth_sharp, ssse3),
            SharpRegular8Tap => bd_fn!(mc_scaled::decl_fn, BD, put_8tap_scaled_sharp_regular, ssse3),
            SharpSmooth8Tap => bd_fn!(mc_scaled::decl_fn, BD, put_8tap_scaled_sharp_smooth, ssse3),
            Sharp8Tap => bd_fn!(mc_scaled::decl_fn, BD, put_8tap_scaled_sharp, ssse3),
            Bilinear => bd_fn!(mc_scaled::decl_fn, BD, put_bilin_scaled, ssse3),
        });
        self.mct_scaled = enum_map!(Filter2d => mct_scaled::Fn; match key {
            Regular8Tap => bd_fn!(mct_scaled::decl_fn, BD, prep_8tap_scaled_regular, ssse3),
            RegularSmooth8Tap => bd_fn!(mct_scaled::decl_fn, BD, prep_8tap_scaled_regular_smooth, ssse3),
            RegularSharp8Tap => bd_fn!(mct_scaled::decl_fn, BD, prep_8tap_scaled_regular_sharp, ssse3),
            SmoothRegular8Tap => bd_fn!(mct_scaled::decl_fn, BD, prep_8tap_scaled_smooth_regular, ssse3),
            Smooth8Tap => bd_fn!(mct_scaled::decl_fn, BD, prep_8tap_scaled_smooth, ssse3),
            SmoothSharp8Tap => bd_fn!(mct_scaled::decl_fn, BD, prep_8tap_scaled_smooth_sharp, ssse3),
            SharpRegular8Tap => bd_fn!(mct_scaled::decl_fn, BD, prep_8tap_scaled_sharp_regular, ssse3),
            SharpSmooth8Tap => bd_fn!(mct_scaled::decl_fn, BD, prep_8tap_scaled_sharp_smooth, ssse3),
            Sharp8Tap => bd_fn!(mct_scaled::decl_fn, BD, prep_8tap_scaled_sharp, ssse3),
            Bilinear => bd_fn!(mct_scaled::decl_fn, BD, prep_bilin_scaled, ssse3),
        });

        self.avg = bd_fn!(BD, avg, ssse3);
        self.w_avg = bd_fn!(BD, w_avg, ssse3);
        self.mask = bd_fn!(BD, mask, ssse3);

        self.w_mask = enum_map!(Rav1dPixelLayoutSubSampled => w_mask::Fn; match key {
            I420 => bd_fn!(w_mask::decl_fn, BD, w_mask_420, ssse3),
            I422 => bd_fn!(w_mask::decl_fn, BD, w_mask_422, ssse3),
            I444 => bd_fn!(w_mask::decl_fn, BD, w_mask_444, ssse3),
        });

        self.blend = bd_fn!(BD, blend, ssse3);
        self.blend_v = bd_fn!(BD, blend_v, ssse3);
        self.blend_h = bd_fn!(BD, blend_h, ssse3);
        self.warp8x8 = bd_fn!(warp8x8::decl_fn, BD, warp_affine_8x8, ssse3);
        self.warp8x8t = bd_fn!(warp8x8t::decl_fn, BD, warp_affine_8x8t, ssse3);
        self.emu_edge = bd_fn!(BD, emu_edge, ssse3);
        self.resize = bd_fn!(BD, resize, ssse3);

        if !flags.contains(CpuFlags::SSE41) {
            return self;
        }

        if let BPC::BPC8 = BD::BPC {
            self.warp8x8 = bpc_fn!(warp8x8::decl_fn, 8 bpc, warp_affine_8x8, sse4);
            self.warp8x8t = bpc_fn!(warp8x8t::decl_fn, 8 bpc, warp_affine_8x8t, sse4);
        }

        #[cfg(target_arch = "x86_64")]
        {
            if !flags.contains(CpuFlags::AVX2) {
                return self;
            }

            self.mc = enum_map!(Filter2d => mc::Fn; match key {
                Regular8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_regular, avx2),
                RegularSmooth8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_regular_smooth, avx2),
                RegularSharp8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_regular_sharp, avx2),
                SmoothRegular8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_smooth_regular, avx2),
                Smooth8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_smooth, avx2),
                SmoothSharp8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_smooth_sharp, avx2),
                SharpRegular8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_sharp_regular, avx2),
                SharpSmooth8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_sharp_smooth, avx2),
                Sharp8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_sharp, avx2),
                Bilinear => bd_fn!(mc::decl_fn, BD, put_bilin, avx2),
            });
            self.mct = enum_map!(Filter2d => mct::Fn; match key {
                Regular8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_regular, avx2),
                RegularSmooth8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_regular_smooth, avx2),
                RegularSharp8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_regular_sharp, avx2),
                SmoothRegular8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_smooth_regular, avx2),
                Smooth8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_smooth, avx2),
                SmoothSharp8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_smooth_sharp, avx2),
                SharpRegular8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_sharp_regular, avx2),
                SharpSmooth8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_sharp_smooth, avx2),
                Sharp8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_sharp, avx2),
                Bilinear => bd_fn!(mct::decl_fn, BD, prep_bilin, avx2),
            });
            self.mc_scaled = enum_map!(Filter2d => mc_scaled::Fn; match key {
                Regular8Tap => bd_fn!(mc_scaled::decl_fn, BD, put_8tap_scaled_regular, avx2),
                RegularSmooth8Tap => bd_fn!(mc_scaled::decl_fn, BD, put_8tap_scaled_regular_smooth, avx2),
                RegularSharp8Tap => bd_fn!(mc_scaled::decl_fn, BD, put_8tap_scaled_regular_sharp, avx2),
                SmoothRegular8Tap => bd_fn!(mc_scaled::decl_fn, BD, put_8tap_scaled_smooth_regular, avx2),
                Smooth8Tap => bd_fn!(mc_scaled::decl_fn, BD, put_8tap_scaled_smooth, avx2),
                SmoothSharp8Tap => bd_fn!(mc_scaled::decl_fn, BD, put_8tap_scaled_smooth_sharp, avx2),
                SharpRegular8Tap => bd_fn!(mc_scaled::decl_fn, BD, put_8tap_scaled_sharp_regular, avx2),
                SharpSmooth8Tap => bd_fn!(mc_scaled::decl_fn, BD, put_8tap_scaled_sharp_smooth, avx2),
                Sharp8Tap => bd_fn!(mc_scaled::decl_fn, BD, put_8tap_scaled_sharp, avx2),
                Bilinear => bd_fn!(mc_scaled::decl_fn, BD, put_bilin_scaled, avx2),
            });
            self.mct_scaled = enum_map!(Filter2d => mct_scaled::Fn; match key {
                Regular8Tap => bd_fn!(mct_scaled::decl_fn, BD, prep_8tap_scaled_regular, avx2),
                RegularSmooth8Tap => bd_fn!(mct_scaled::decl_fn, BD, prep_8tap_scaled_regular_smooth, avx2),
                RegularSharp8Tap => bd_fn!(mct_scaled::decl_fn, BD, prep_8tap_scaled_regular_sharp, avx2),
                SmoothRegular8Tap => bd_fn!(mct_scaled::decl_fn, BD, prep_8tap_scaled_smooth_regular, avx2),
                Smooth8Tap => bd_fn!(mct_scaled::decl_fn, BD, prep_8tap_scaled_smooth, avx2),
                SmoothSharp8Tap => bd_fn!(mct_scaled::decl_fn, BD, prep_8tap_scaled_smooth_sharp, avx2),
                SharpRegular8Tap => bd_fn!(mct_scaled::decl_fn, BD, prep_8tap_scaled_sharp_regular, avx2),
                SharpSmooth8Tap => bd_fn!(mct_scaled::decl_fn, BD, prep_8tap_scaled_sharp_smooth, avx2),
                Sharp8Tap => bd_fn!(mct_scaled::decl_fn, BD, prep_8tap_scaled_sharp, avx2),
                Bilinear => bd_fn!(mct_scaled::decl_fn, BD, prep_bilin_scaled, avx2),
            });

            self.avg = bd_fn!(BD, avg, avx2);
            self.w_avg = bd_fn!(BD, w_avg, avx2);
            self.mask = bd_fn!(BD, mask, avx2);

            self.w_mask = enum_map!(Rav1dPixelLayoutSubSampled => w_mask::Fn; match key {
                I420 => bd_fn!(w_mask::decl_fn, BD, w_mask_420, avx2),
                I422 => bd_fn!(w_mask::decl_fn, BD, w_mask_422, avx2),
                I444 => bd_fn!(w_mask::decl_fn, BD, w_mask_444, avx2),
            });

            self.blend = bd_fn!(BD, blend, avx2);
            self.blend_v = bd_fn!(BD, blend_v, avx2);
            self.blend_h = bd_fn!(BD, blend_h, avx2);
            self.warp8x8 = bd_fn!(warp8x8::decl_fn, BD, warp_affine_8x8, avx2);
            self.warp8x8t = bd_fn!(warp8x8t::decl_fn, BD, warp_affine_8x8t, avx2);
            self.emu_edge = bd_fn!(BD, emu_edge, avx2);
            self.resize = bd_fn!(BD, resize, avx2);

            if !flags.contains(CpuFlags::AVX512ICL) {
                return self;
            }

            self.mc = enum_map!(Filter2d => mc::Fn; match key {
                Regular8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_regular, avx512icl),
                RegularSmooth8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_regular_smooth, avx512icl),
                RegularSharp8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_regular_sharp, avx512icl),
                SmoothRegular8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_smooth_regular, avx512icl),
                Smooth8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_smooth, avx512icl),
                SmoothSharp8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_smooth_sharp, avx512icl),
                SharpRegular8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_sharp_regular, avx512icl),
                SharpSmooth8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_sharp_smooth, avx512icl),
                Sharp8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_sharp, avx512icl),
                Bilinear => bd_fn!(mc::decl_fn, BD, put_bilin, avx512icl),
            });
            self.mct = enum_map!(Filter2d => mct::Fn; match key {
                Regular8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_regular, avx512icl),
                RegularSmooth8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_regular_smooth, avx512icl),
                RegularSharp8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_regular_sharp, avx512icl),
                SmoothRegular8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_smooth_regular, avx512icl),
                Smooth8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_smooth, avx512icl),
                SmoothSharp8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_smooth_sharp, avx512icl),
                SharpRegular8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_sharp_regular, avx512icl),
                SharpSmooth8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_sharp_smooth, avx512icl),
                Sharp8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_sharp, avx512icl),
                Bilinear => bd_fn!(mct::decl_fn, BD, prep_bilin, avx512icl),
            });

            self.avg = bd_fn!(BD, avg, avx512icl);
            self.w_avg = bd_fn!(BD, w_avg, avx512icl);
            self.mask = bd_fn!(BD, mask, avx512icl);

            self.w_mask = enum_map!(Rav1dPixelLayoutSubSampled => w_mask::Fn; match key {
                I420 => bd_fn!(w_mask::decl_fn, BD, w_mask_420, avx512icl),
                I422 => bd_fn!(w_mask::decl_fn, BD, w_mask_422, avx512icl),
                I444 => bd_fn!(w_mask::decl_fn, BD, w_mask_444, avx512icl),
            });

            self.blend = bd_fn!(BD, blend, avx512icl);
            self.blend_v = bd_fn!(BD, blend_v, avx512icl);
            self.blend_h = bd_fn!(BD, blend_h, avx512icl);

            if !flags.contains(CpuFlags::SLOW_GATHER) {
                self.resize = bd_fn!(BD, resize, avx512icl);
                self.warp8x8 = bd_fn!(warp8x8::decl_fn, BD, warp_affine_8x8, avx512icl);
                self.warp8x8t = bd_fn!(warp8x8t::decl_fn, BD, warp_affine_8x8t, avx512icl);
            }
        }

        self
    }

    #[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
    #[inline(always)]
    const fn init_arm<BD: BitDepth>(mut self, flags: CpuFlags) -> Self {
        if !flags.contains(CpuFlags::NEON) {
            return self;
        }

        self.mc = enum_map!(Filter2d => mc::Fn; match key {
            Regular8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_regular, neon),
            RegularSmooth8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_regular_smooth, neon),
            RegularSharp8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_regular_sharp, neon),
            SmoothRegular8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_smooth_regular, neon),
            Smooth8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_smooth, neon),
            SmoothSharp8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_smooth_sharp, neon),
            SharpRegular8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_sharp_regular, neon),
            SharpSmooth8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_sharp_smooth, neon),
            Sharp8Tap => bd_fn!(mc::decl_fn, BD, put_8tap_sharp, neon),
            Bilinear => bd_fn!(mc::decl_fn, BD, put_bilin, neon),
        });
        self.mct = enum_map!(Filter2d => mct::Fn; match key {
            Regular8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_regular, neon),
            RegularSmooth8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_regular_smooth, neon),
            RegularSharp8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_regular_sharp, neon),
            SmoothRegular8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_smooth_regular, neon),
            Smooth8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_smooth, neon),
            SmoothSharp8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_smooth_sharp, neon),
            SharpRegular8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_sharp_regular, neon),
            SharpSmooth8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_sharp_smooth, neon),
            Sharp8Tap => bd_fn!(mct::decl_fn, BD, prep_8tap_sharp, neon),
            Bilinear => bd_fn!(mct::decl_fn, BD, prep_bilin, neon),
        });

        self.avg = bd_fn!(BD, avg, neon);
        self.w_avg = bd_fn!(BD, w_avg, neon);
        self.mask = bd_fn!(BD, mask, neon);
        self.blend = bd_fn!(BD, blend, neon);
        self.blend_h = bd_fn!(BD, blend_h, neon);
        self.blend_v = bd_fn!(BD, blend_v, neon);

        self.w_mask = enum_map!(Rav1dPixelLayoutSubSampled => w_mask::Fn; match key {
            I420 => bd_fn!(w_mask::decl_fn, BD, w_mask_420, neon),
            I422 => bd_fn!(w_mask::decl_fn, BD, w_mask_422, neon),
            I444 => bd_fn!(w_mask::decl_fn, BD, w_mask_444, neon),
        });

        self.warp8x8 = bd_fn!(warp8x8::decl_fn, BD, warp_affine_8x8, neon);
        self.warp8x8t = bd_fn!(warp8x8t::decl_fn, BD, warp_affine_8x8t, neon);
        self.emu_edge = bd_fn!(BD, emu_edge, neon);

        self
    }

    #[inline(always)]
    const fn init<BD: BitDepth>(self, flags: CpuFlags) -> Self {
        #[cfg(feature = "asm")]
        {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            {
                return self.init_x86::<BD>(flags);
            }
            #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
            {
                return self.init_arm::<BD>(flags);
            }
        }

        #[allow(unreachable_code)] // Reachable on some #[cfg]s.
        {
            let _ = flags;
            self
        }
    }

    pub const fn new<BD: BitDepth>(flags: CpuFlags) -> Self {
        Self::default::<BD>().init::<BD>(flags)
    }
}
