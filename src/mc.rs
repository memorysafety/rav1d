#![deny(unsafe_op_in_unsafe_fn)]

use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::intops::clip;
use crate::include::common::intops::iclip;
use crate::include::dav1d::headers::Rav1dFilterMode;
use crate::include::dav1d::headers::Rav1dPixelLayoutSubSampled;
use crate::include::dav1d::picture::Rav1dPictureDataComponent;
use crate::include::dav1d::picture::Rav1dPictureDataComponentOffset;
use crate::src::align::AlignedVec64;
use crate::src::cpu::CpuFlags;
use crate::src::enum_map::enum_map;
use crate::src::enum_map::enum_map_ty;
use crate::src::enum_map::DefaultValue;
use crate::src::ffi_safe::FFISafe;
use crate::src::internal::COMPINTER_LEN;
use crate::src::internal::EMU_EDGE_LEN;
use crate::src::internal::SCRATCH_INTER_INTRA_BUF_LEN;
use crate::src::internal::SCRATCH_LAP_LEN;
use crate::src::internal::SEG_MASK_LEN;
use crate::src::levels::Filter2d;
use crate::src::pic_or_buf::PicOrBuf;
use crate::src::strided::Strided as _;
use crate::src::tables::dav1d_mc_subpel_filters;
use crate::src::tables::dav1d_mc_warp_filter;
use crate::src::tables::dav1d_obmc_masks;
use crate::src::tables::dav1d_resize_filter;
use crate::src::with_offset::WithOffset;
use crate::src::wrap_fn_ptr::wrap_fn_ptr;
use std::cmp;
use std::ffi::c_int;
use std::iter;
use std::mem;
use std::ptr;
use std::slice;
use to_method::To;

#[cfg(all(
    feature = "asm",
    not(any(target_arch = "riscv64", target_arch = "riscv32"))
))]
use crate::include::common::bitdepth::bd_fn;

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
use crate::include::common::bitdepth::{bpc_fn, BPC};

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
use crate::include::common::bitdepth::bpc_fn;

#[inline(never)]
fn put_rust<BD: BitDepth>(
    dst: Rav1dPictureDataComponentOffset,
    src: Rav1dPictureDataComponentOffset,
    w: usize,
    h: usize,
) {
    for y in 0..h {
        let src = src + y as isize * src.pixel_stride::<BD>();
        let dst = dst + y as isize * dst.pixel_stride::<BD>();
        let src = &*src.slice::<BD>(w);
        let dst = &mut *dst.slice_mut::<BD>(w);
        BD::pixel_copy(dst, src, w);
    }
}

#[inline(never)]
fn prep_rust<BD: BitDepth>(
    tmp: &mut [i16],
    src: Rav1dPictureDataComponentOffset,
    w: usize,
    h: usize,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    for y in 0..h {
        let src = src + y as isize * src.pixel_stride::<BD>();
        let src = &*src.slice::<BD>(w);
        let tmp = &mut tmp[y * w..][..w];
        for x in 0..w {
            tmp[x] = BD::sub_prep_bias(src[x].as_::<i32>() << intermediate_bits)
        }
    }
}

#[derive(Clone, Copy)]
struct FilterResult {
    pixel: i32,
}

impl FilterResult {
    pub fn get(&self) -> i16 {
        self.pixel as i16
    }

    pub fn apply(&self, f: impl Fn(i32) -> i32) -> Self {
        let pixel = f(self.pixel);
        Self { pixel }
    }

    pub fn rnd(&self, sh: u8) -> Self {
        self.apply(|px| (px + ((1 << sh) >> 1)) >> sh)
    }

    pub fn rnd2(&self, sh: u8, rnd: u8) -> Self {
        self.apply(|px| (px + (rnd as i32)) >> sh)
    }

    pub fn clip<BD: BitDepth>(&self, bd: BD) -> BD::Pixel {
        bd.iclip_pixel(self.pixel)
    }

    pub fn sub_prep_bias<BD: BitDepth>(&self) -> i16 {
        BD::sub_prep_bias(self.pixel)
    }
}

const MID_STRIDE: usize = 128;

fn filter_8tap_mid(mid: &[[i16; MID_STRIDE]], x: usize, f: &[i8; 8]) -> FilterResult {
    let pixel = (0..f.len()).map(|y| f[y] as i32 * mid[y][x] as i32).sum();
    FilterResult { pixel }
}

fn filter_8tap<BD: BitDepth>(
    src: Rav1dPictureDataComponentOffset,
    x: usize,
    f: &[i8; 8],
    stride: isize,
) -> FilterResult {
    let pixel = (0..f.len())
        .map(|y| {
            let px = *(src + x + (y as isize - 3) * stride).index::<BD>();
            f[y] as i32 * px.to::<i32>()
        })
        .sum();
    FilterResult { pixel }
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
fn put_8tap_rust<BD: BitDepth>(
    dst: Rav1dPictureDataComponentOffset,
    src: Rav1dPictureDataComponentOffset,
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

    if let Some(fh) = fh {
        if let Some(fv) = fv {
            let tmp_h = h + 7;
            let mut mid = [[0i16; MID_STRIDE]; 135]; // Default::default()

            for y in 0..tmp_h {
                let src = src + (y as isize - 3) * src.pixel_stride::<BD>();
                for x in 0..w {
                    mid[y][x] = filter_8tap::<BD>(src, x, fh, 1)
                        .rnd(6 - intermediate_bits)
                        .get();
                }
            }

            for y in 0..h {
                let dst = dst + y as isize * dst.pixel_stride::<BD>();
                let dst = &mut *dst.slice_mut::<BD>(w);
                for x in 0..w {
                    dst[x] = filter_8tap_mid(&mid[y..], x, fv)
                        .rnd(6 + intermediate_bits)
                        .clip(bd);
                }
            }
        } else {
            for y in 0..h {
                let src = src + y as isize * src.pixel_stride::<BD>();
                let dst = dst + y as isize * dst.pixel_stride::<BD>();
                let dst = &mut *dst.slice_mut::<BD>(w);
                for x in 0..w {
                    dst[x] = filter_8tap::<BD>(src, x, fh, 1)
                        .rnd2(6, intermediate_rnd)
                        .clip(bd);
                }
            }
        }
    } else if let Some(fv) = fv {
        for y in 0..h {
            let src = src + y as isize * src.pixel_stride::<BD>();
            let dst = dst + y as isize * dst.pixel_stride::<BD>();
            let dst = &mut *dst.slice_mut::<BD>(w);
            for x in 0..w {
                dst[x] = filter_8tap::<BD>(src, x, fv, src.pixel_stride::<BD>())
                    .rnd(6)
                    .clip(bd);
            }
        }
    } else {
        put_rust::<BD>(dst, src, w, h);
    }
}

#[inline(never)]
fn put_8tap_scaled_rust<BD: BitDepth>(
    dst: Rav1dPictureDataComponentOffset,
    src: Rav1dPictureDataComponentOffset,
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
    let mut mid = [[0i16; MID_STRIDE]; 256 + 7]; // Default::default()

    for y in 0..tmp_h {
        let src = src + (y as isize - 3) * src.pixel_stride::<BD>();
        let mut imx = mx;
        let mut ioff = 0;

        for x in 0..w {
            let fh = get_filter(imx >> 6, w, h_filter_type);
            mid[y][x] = match fh {
                Some(fh) => filter_8tap::<BD>(src, ioff, fh, 1)
                    .rnd(6 - intermediate_bits)
                    .get(),
                None => (*(src + ioff).index::<BD>()).as_::<i16>() << intermediate_bits,
            };
            imx += dx;
            ioff += imx >> 10;
            imx &= 0x3ff;
        }
    }
    let mut mid = &mut mid[..];
    for y in 0..h {
        let fv = get_filter(my >> 6, h, v_filter_type);

        let dst = dst + y as isize * dst.pixel_stride::<BD>();
        let dst = &mut *dst.slice_mut::<BD>(w);
        for x in 0..w {
            dst[x] = match fv {
                Some(fv) => filter_8tap_mid(mid, x, fv)
                    .rnd(6 + intermediate_bits)
                    .clip(bd),
                None => {
                    bd.iclip_pixel((i32::from(mid[3][x]) + intermediate_rnd) >> intermediate_bits)
                }
            };
        }

        my += dy;
        mid = &mut mid[(my >> 10)..];
        my &= 0x3ff;
    }
}

#[inline(never)]
fn prep_8tap_rust<BD: BitDepth>(
    tmp: &mut [i16],
    src: Rav1dPictureDataComponentOffset,
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

    if let Some(fh) = fh {
        if let Some(fv) = fv {
            let tmp_h = h + 7;
            let mut mid = [[0i16; MID_STRIDE]; 135]; // Default::default()

            for y in 0..tmp_h {
                let src = src + (y as isize - 3) * src.pixel_stride::<BD>();
                for x in 0..w {
                    mid[y][x] = filter_8tap::<BD>(src, x, fh, 1)
                        .rnd(6 - intermediate_bits)
                        .get();
                }
            }

            for y in 0..h {
                let tmp = &mut tmp[y * w..][..w];
                for x in 0..w {
                    tmp[x] = filter_8tap_mid(&mid[y..], x, fv)
                        .rnd(6)
                        .sub_prep_bias::<BD>();
                }
            }
        } else {
            for y in 0..h {
                let src = src + y as isize * src.pixel_stride::<BD>();
                let tmp = &mut tmp[y * w..][..w];
                for x in 0..w {
                    tmp[x] = filter_8tap::<BD>(src, x, fh, 1)
                        .rnd(6 - intermediate_bits)
                        .sub_prep_bias::<BD>();
                }
            }
        }
    } else if let Some(fv) = fv {
        for y in 0..h {
            let src = src + y as isize * src.pixel_stride::<BD>();
            let tmp = &mut tmp[y * w..][..w];
            for x in 0..w {
                tmp[x] = filter_8tap::<BD>(src, x, fv, src.pixel_stride::<BD>())
                    .rnd(6 - intermediate_bits)
                    .sub_prep_bias::<BD>()
            }
        }
    } else {
        prep_rust(tmp, src, w, h, bd);
    };
}

#[inline(never)]
fn prep_8tap_scaled_rust<BD: BitDepth>(
    tmp: &mut [i16],
    src: Rav1dPictureDataComponentOffset,
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
    let mut mid = [[0i16; MID_STRIDE]; 256 + 7]; // Default::default()

    for y in 0..tmp_h {
        let src = src + (y as isize - 3) * src.pixel_stride::<BD>();
        let mut imx = mx;
        let mut ioff = 0;
        for x in 0..w {
            let fh = get_filter(imx >> 6, w, h_filter_type);
            mid[y][x] = match fh {
                Some(fh) => filter_8tap::<BD>(src, ioff, fh, 1)
                    .rnd(6 - intermediate_bits)
                    .get(),
                None => (*(src + ioff).index::<BD>()).as_::<i16>() << intermediate_bits,
            };
            imx += dx;
            ioff += imx >> 10;
            imx &= 0x3ff;
        }
    }

    let mut mid = &mut mid[..];
    for y in 0..h {
        let tmp = &mut tmp[y * w..][..w];
        let fv = get_filter(my >> 6, h, v_filter_type);
        for x in 0..w {
            tmp[x] = match fv {
                Some(fv) => filter_8tap_mid(mid, x, fv).rnd(6),
                None => FilterResult {
                    pixel: mid[3][x].into(),
                },
            }
            .sub_prep_bias::<BD>()
        }
        my += dy;
        mid = &mut mid[(my >> 10)..];
        my &= 0x3ff;
    }
}

fn filter_bilin_mid(mid: &[[i16; MID_STRIDE]], x: usize, mxy: usize) -> FilterResult {
    let x0 = mid[0][x] as i32;
    let x1 = mid[1][x] as i32;
    let pixel = 16 * x0 + mxy as i32 * (x1 - x0);
    FilterResult { pixel }
}

fn filter_bilin<BD: BitDepth>(
    src: Rav1dPictureDataComponentOffset,
    x: usize,
    mxy: usize,
    stride: isize,
) -> FilterResult {
    let src = |y: isize, x: usize| -> i32 { (*(src + x + y * stride).index::<BD>()).to::<i32>() };
    let x0 = src(0, x);
    let x1 = src(1, x);
    let pixel = 16 * x0 + mxy as i32 * (x1 - x0);
    FilterResult { pixel }
}

fn put_bilin_rust<BD: BitDepth>(
    dst: Rav1dPictureDataComponentOffset,
    src: Rav1dPictureDataComponentOffset,
    w: usize,
    h: usize,
    mx: usize,
    my: usize,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let intermediate_rnd = (1 << intermediate_bits) >> 1;

    if mx != 0 {
        if my != 0 {
            let mut mid = [[0i16; MID_STRIDE]; 129]; // Default::default()
            let tmp_h = h + 1;

            for y in 0..tmp_h {
                let src = src + y as isize * src.pixel_stride::<BD>();
                for x in 0..w {
                    mid[y][x] = filter_bilin::<BD>(src, x, mx, 1)
                        .rnd(4 - intermediate_bits)
                        .get();
                }
            }
            for y in 0..h {
                let dst = dst + y as isize * dst.pixel_stride::<BD>();
                let dst = &mut *dst.slice_mut::<BD>(w);
                for x in 0..w {
                    dst[x] = filter_bilin_mid(&mid[y..], x, my)
                        .rnd(4 + intermediate_bits)
                        .clip(bd)
                }
            }
        } else {
            for y in 0..h {
                let src = src + y as isize * src.pixel_stride::<BD>();
                let dst = dst + y as isize * dst.pixel_stride::<BD>();
                let dst = &mut *dst.slice_mut::<BD>(w);
                for x in 0..w {
                    dst[x] = filter_bilin::<BD>(src, x, mx, 1)
                        .rnd(4 - intermediate_bits)
                        .apply(|px| (px + intermediate_rnd) >> intermediate_bits)
                        .clip(bd);
                }
            }
        }
    } else if my != 0 {
        for y in 0..h {
            let src = src + y as isize * src.pixel_stride::<BD>();
            let dst = dst + y as isize * dst.pixel_stride::<BD>();
            let dst = &mut *dst.slice_mut::<BD>(w);
            for x in 0..w {
                dst[x] = filter_bilin::<BD>(src, x, my, src.pixel_stride::<BD>())
                    .rnd(4)
                    .clip(bd)
            }
        }
    } else {
        put_rust::<BD>(dst, src, w, h);
    };
}

fn put_bilin_scaled_rust<BD: BitDepth>(
    dst: Rav1dPictureDataComponentOffset,
    src: Rav1dPictureDataComponentOffset,
    w: usize,
    h: usize,
    mx: usize,
    mut my: usize,
    dx: usize,
    dy: usize,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let tmp_h = ((h - 1) * dy + my >> 10) + 2;
    let mut mid = [[0i16; MID_STRIDE]; 256 + 1];

    for y in 0..tmp_h {
        let src = src + y as isize * src.pixel_stride::<BD>();
        let mut imx = mx;
        let mut ioff = 0;

        for x in 0..w {
            mid[y][x] = filter_bilin::<BD>(src, ioff, imx >> 6, 1)
                .rnd(4 - intermediate_bits)
                .get();
            imx += dx;
            ioff += imx >> 10;
            imx &= 0x3ff;
        }
    }
    let mut mid = &mut mid[..];
    for y in 0..h {
        let dst = dst + y as isize * dst.pixel_stride::<BD>();
        let dst = &mut *dst.slice_mut::<BD>(w);
        for x in 0..w {
            dst[x] = filter_bilin_mid(mid, x, my >> 6)
                .rnd(4 + intermediate_bits)
                .clip(bd)
        }

        my += dy;
        mid = &mut mid[(my >> 10)..];
        my &= 0x3ff;
    }
}

fn prep_bilin_rust<BD: BitDepth>(
    tmp: &mut [i16],
    src: Rav1dPictureDataComponentOffset,
    w: usize,
    h: usize,
    mx: usize,
    my: usize,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    if mx != 0 {
        if my != 0 {
            let mut mid = [[0i16; MID_STRIDE]; 129];
            let tmp_h = h + 1;

            for y in 0..tmp_h {
                let src = src + y as isize * src.pixel_stride::<BD>();
                for x in 0..w {
                    mid[y][x] = filter_bilin::<BD>(src, x, mx, 1)
                        .rnd(4 - intermediate_bits)
                        .get();
                }
            }
            for y in 0..h {
                let tmp = &mut tmp[y * w..][..w];
                for x in 0..w {
                    tmp[x] = filter_bilin_mid(&mid[y..], x, my)
                        .rnd(4)
                        .sub_prep_bias::<BD>()
                }
            }
        } else {
            for y in 0..h {
                let src = src + y as isize * src.pixel_stride::<BD>();
                let tmp = &mut tmp[y * w..][..w];
                for x in 0..w {
                    tmp[x] = filter_bilin::<BD>(src, x, mx, 1)
                        .rnd(4 - intermediate_bits)
                        .sub_prep_bias::<BD>()
                }
            }
        }
    } else if my != 0 {
        for y in 0..h {
            let src = src + y as isize * src.pixel_stride::<BD>();
            let tmp = &mut tmp[y * w..][..w];
            for x in 0..w {
                tmp[x] = filter_bilin::<BD>(src, x, my, src.pixel_stride::<BD>())
                    .rnd(4 - intermediate_bits)
                    .sub_prep_bias::<BD>()
            }
        }
    } else {
        prep_rust(tmp, src, w, h, bd);
    };
}

fn prep_bilin_scaled_rust<BD: BitDepth>(
    tmp: &mut [i16],
    src: Rav1dPictureDataComponentOffset,
    w: usize,
    h: usize,
    mx: usize,
    mut my: usize,
    dx: usize,
    dy: usize,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let tmp_h = ((h - 1) * dy + my >> 10) + 2;
    let mut mid = [[0i16; MID_STRIDE]; 256 + 1];

    for y in 0..tmp_h {
        let src = src + y as isize * src.pixel_stride::<BD>();
        let mut imx = mx;
        let mut ioff = 0;

        for x in 0..w {
            mid[y][x] = filter_bilin::<BD>(src, ioff, imx >> 6, 1)
                .rnd(4 - intermediate_bits)
                .get();
            imx += dx;
            ioff += imx >> 10;
            imx &= 0x3ff;
        }
    }
    let mut mid = &mut mid[..];
    for y in 0..h {
        let tmp = &mut tmp[y * w..][..w];
        for x in 0..w {
            tmp[x] = filter_bilin_mid(mid, x, my >> 6)
                .rnd(4)
                .sub_prep_bias::<BD>()
        }

        my += dy;
        mid = &mut mid[(my >> 10)..];
        my &= 0x3ff;
    }
}

fn avg_rust<BD: BitDepth>(
    dst: Rav1dPictureDataComponentOffset,
    tmp1: &[i16; COMPINTER_LEN],
    tmp2: &[i16; COMPINTER_LEN],
    w: usize,
    h: usize,
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let sh = intermediate_bits + 1;
    let rnd = (1 << intermediate_bits) + i32::from(BD::PREP_BIAS) * 2;
    let tmp1 = &tmp1[..w * h];
    let tmp2 = &tmp2[..w * h];
    for y in 0..h {
        let dst = dst + (y as isize * dst.pixel_stride::<BD>());
        let dst = &mut *dst.slice_mut::<BD>(w);
        for x in 0..w {
            dst[x] = bd.iclip_pixel(
                ((tmp1[y * w + x] as i32 + tmp2[y * w + x] as i32 + rnd) >> sh).to::<i32>(),
            );
        }
    }
}

fn w_avg_rust<BD: BitDepth>(
    dst: Rav1dPictureDataComponentOffset,
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
    let tmp1 = &tmp1[..w * h];
    let tmp2 = &tmp2[..w * h];
    for y in 0..h {
        let dst = dst + (y as isize * dst.pixel_stride::<BD>());
        let dst = &mut *dst.slice_mut::<BD>(w);
        for x in 0..w {
            dst[x] = bd.iclip_pixel(
                (tmp1[y * w + x] as i32 * weight + tmp2[y * w + x] as i32 * (16 - weight) + rnd)
                    >> sh,
            );
        }
    }
}

fn mask_rust<BD: BitDepth>(
    dst: Rav1dPictureDataComponentOffset,
    tmp1: &[i16; COMPINTER_LEN],
    tmp2: &[i16; COMPINTER_LEN],
    w: usize,
    h: usize,
    mask: &[u8],
    bd: BD,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let sh = intermediate_bits + 6;
    let rnd = (32 << intermediate_bits) + i32::from(BD::PREP_BIAS) * 64;
    let tmp1 = &tmp1[..w * h];
    let tmp2 = &tmp2[..w * h];
    for y in 0..h {
        let dst = dst + (y as isize * dst.pixel_stride::<BD>());
        let dst = &mut *dst.slice_mut::<BD>(w);
        for x in 0..w {
            dst[x] = bd.iclip_pixel(
                (tmp1[y * w + x] as i32 * mask[y * w + x] as i32
                    + tmp2[y * w + x] as i32 * (64 - mask[y * w + x] as i32)
                    + rnd)
                    >> sh,
            );
        }
    }
}

fn blend_px<BD: BitDepth>(a: BD::Pixel, b: BD::Pixel, m: u8) -> BD::Pixel {
    let m = m as u32;
    ((a.as_::<u32>() * (64 - m) + b.as_::<u32>() * m + 32) >> 6).as_::<BD::Pixel>()
}

fn blend_rust<BD: BitDepth>(
    dst: Rav1dPictureDataComponentOffset,
    tmp: &[BD::Pixel; SCRATCH_INTER_INTRA_BUF_LEN],
    w: usize,
    h: usize,
    mask: &[u8],
) {
    for y in 0..h {
        let dst = dst + (y as isize * dst.pixel_stride::<BD>());
        let dst = &mut *dst.slice_mut::<BD>(w);
        for x in 0..w {
            dst[x] = blend_px::<BD>(dst[x], tmp[y * w + x], mask[y * w + x]);
        }
    }
}

fn blend_v_rust<BD: BitDepth>(
    dst: Rav1dPictureDataComponentOffset,
    tmp: &[BD::Pixel; SCRATCH_LAP_LEN],
    w: usize,
    h: usize,
) {
    let mask = &dav1d_obmc_masks.0[w..];
    let tmp = &tmp[..w * h];
    for y in 0..h {
        let dst = dst + (y as isize * dst.pixel_stride::<BD>());
        let dst_w = w * 3 >> 2;
        let dst = &mut *dst.slice_mut::<BD>(dst_w);
        for x in 0..dst_w {
            dst[x] = blend_px::<BD>(dst[x], tmp[y * w + x], mask[x]);
        }
    }
}

fn blend_h_rust<BD: BitDepth>(
    dst: Rav1dPictureDataComponentOffset,
    tmp: &[BD::Pixel; SCRATCH_LAP_LEN],
    w: usize,
    h: usize,
) {
    let mask = &dav1d_obmc_masks.0[h..];
    let h = h * 3 >> 2;
    let tmp = &tmp[..w * h];
    for y in 0..h {
        let dst = dst + (y as isize * dst.pixel_stride::<BD>());
        let dst = &mut *dst.slice_mut::<BD>(w);
        for x in 0..w {
            dst[x] = blend_px::<BD>(dst[x], tmp[y * w + x], mask[y]);
        }
    }
}

fn w_mask_rust<BD: BitDepth>(
    mut dst: Rav1dPictureDataComponentOffset,
    tmp1: &[i16; COMPINTER_LEN],
    tmp2: &[i16; COMPINTER_LEN],
    w: usize,
    h: usize,
    mask: &mut [u8; SEG_MASK_LEN],
    sign: bool,
    ss_hor: bool,
    ss_ver: bool,
    bd: BD,
) {
    let mut mask = &mut mask[..(w >> ss_hor as usize) * (h >> ss_ver as usize)];
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
        let dst_slice = &mut *dst.slice_mut::<BD>(w);
        let mut x = 0;
        while x < w {
            let m = cmp::min(
                38 + (tmp1[x].abs_diff(tmp2[x]).saturating_add(mask_rnd) >> mask_sh),
                64,
            ) as u8;
            dst_slice[x] = bd.iclip_pixel(
                (tmp1[x] as i32 * m as i32 + tmp2[x] as i32 * (64 - m as i32) + rnd) >> sh,
            );

            if ss_hor {
                x += 1;

                let n = cmp::min(
                    38 + (tmp1[x].abs_diff(tmp2[x]).saturating_add(mask_rnd) >> mask_sh),
                    64,
                ) as u8;
                dst_slice[x] = bd.iclip_pixel(
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

        dst += dst.pixel_stride::<BD>();
        if !ss_ver || h & 1 != 0 {
            mask = &mut mask[w >> ss_hor as usize..];
        }
    }
}

fn warp_affine_8x8_rust<BD: BitDepth>(
    dst: Rav1dPictureDataComponentOffset,
    src: Rav1dPictureDataComponentOffset,
    abcd: &[i16; 4],
    mx: i32,
    my: i32,
    bd: BD,
) {
    const W: usize = 8;
    const H: usize = 15;

    let intermediate_bits = bd.get_intermediate_bits();
    let mut mid = [[0; W]; H];

    for y in 0..H {
        let src = src + (y as isize - 3) * src.pixel_stride::<BD>();
        let mx = mx + y as i32 * abcd[1] as i32;
        for x in 0..W {
            let tmx = mx + x as i32 * abcd[0] as i32;
            let filter = &dav1d_mc_warp_filter[(64 + (tmx + 512 >> 10)) as usize];
            let n = filter.len();
            let src = &*(src + x - 3usize).slice::<BD>(n);
            mid[y][x] = ((0..n)
                .map(|i| filter[i] as i32 * src[i].as_::<i32>())
                .sum::<i32>()
                + (1 << 7 - intermediate_bits >> 1)
                >> 7 - intermediate_bits) as i16;
        }
    }

    for y in 0..H - 7 {
        let my = my + y as i32 * abcd[3] as i32;
        let dst = dst + y as isize * dst.pixel_stride::<BD>();
        let dst = &mut *dst.slice_mut::<BD>(W);
        for x in 0..W {
            let tmy = my + x as i32 * abcd[2] as i32;
            let filter = &dav1d_mc_warp_filter[(64 + (tmy + 512 >> 10)) as usize];
            let n = filter.len();
            let mid = &mid[y..][..n];
            dst[x] = bd.iclip_pixel(
                (0..n)
                    .map(|i| filter[i] as i32 * mid[i][x] as i32)
                    .sum::<i32>()
                    + (1 << 7 + intermediate_bits >> 1)
                    >> 7 + intermediate_bits,
            );
        }
    }
}

fn warp_affine_8x8t_rust<BD: BitDepth>(
    tmp: &mut [i16],
    tmp_stride: usize,
    src: Rav1dPictureDataComponentOffset,
    abcd: &[i16; 4],
    mx: i32,
    my: i32,
    bd: BD,
) {
    const W: usize = 8;
    const H: usize = 15;

    let intermediate_bits = bd.get_intermediate_bits();
    let mut mid = [[0; W]; H];

    for y in 0..H {
        let src = src + (y as isize - 3) * src.pixel_stride::<BD>();
        let mx = mx + y as i32 * abcd[1] as i32;
        for x in 0..W {
            let tmx = mx + x as i32 * abcd[0] as i32;
            let filter = &dav1d_mc_warp_filter[(64 + (tmx + 512 >> 10)) as usize];
            let n = filter.len();
            let src = &*(src + x - 3usize).slice::<BD>(n);
            mid[y][x] = ((0..n)
                .map(|i| filter[i] as i32 * src[i].as_::<i32>())
                .sum::<i32>()
                + (1 << 7 - intermediate_bits >> 1)
                >> 7 - intermediate_bits) as i16;
        }
    }

    for y in 0..H - 7 {
        let tmp = &mut tmp[y * tmp_stride..][..W];
        let my = my + y as i32 * abcd[3] as i32;
        for x in 0..W {
            let tmy = my + x as i32 * abcd[2] as i32;
            let filter = &dav1d_mc_warp_filter[(64 + (tmy + 512 >> 10)) as usize];
            let n = filter.len();
            let mid = &mid[y..][..n];
            tmp[x] = BD::sub_prep_bias(
                (0..n)
                    .map(|i| filter[i] as i32 * mid[i][x] as i32)
                    .sum::<i32>()
                    + (1 << 7 >> 1)
                    >> 7,
            );
        }
    }
}

fn emu_edge_rust<BD: BitDepth>(
    bw: isize,
    bh: isize,
    iw: isize,
    ih: isize,
    x: isize,
    y: isize,
    dst: &mut [BD::Pixel; EMU_EDGE_LEN],
    dst_stride: usize,
    r#ref: &Rav1dPictureDataComponent,
) {
    let dst_stride = BD::pxstride(dst_stride);
    let ref_stride = r#ref.pixel_stride::<BD>();

    // find offset in reference of visible block to copy
    let mut r#ref =
        r#ref.with_offset::<BD>() + (clip(y, 0, ih - 1) * ref_stride + clip(x, 0, iw - 1));

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
            &r#ref.slice::<BD>(center_w),
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
        r#ref += ref_stride;
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

fn resize_rust<BD: BitDepth>(
    dst: WithOffset<PicOrBuf<AlignedVec64<u8>>>,
    src: Rav1dPictureDataComponentOffset,
    dst_w: usize,
    h: usize,
    src_w: usize,
    dx: i32,
    mx0: i32,
    bd: BD,
) {
    let max = src_w as i32 - 1;
    for y in 0..h {
        let mut mx = mx0;
        let mut src_x = -1 - 3;
        let dst = dst + (y as isize * dst.pixel_stride::<BD>());
        let src = src + (y as isize * src.pixel_stride::<BD>());
        let src = &*src.slice::<BD>(src_w);
        let dst = match dst.data {
            PicOrBuf::Pic(pic) => &mut *pic.slice_mut::<BD, _>((dst.offset.., ..dst_w)),
            PicOrBuf::Buf(buf) => &mut *buf.mut_slice_as((dst.offset.., ..dst_w)),
        };
        for dst_x in 0..dst_w {
            let f = &dav1d_resize_filter[(mx >> 8) as usize];
            dst[dst_x] = bd.iclip_pixel(
                -(0..f.len())
                    .map(|i| {
                        f[i] as i32 * src[iclip(src_x + i as i32, 0, max) as usize].to::<i32>()
                    })
                    .sum::<i32>()
                    + 64
                    >> 7,
            );
            mx += dx;
            src_x += mx >> 14;
            mx &= 0x3fff;
        }
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn mc(
    dst_ptr: *mut DynPixel,
    dst_stride: isize,
    src_ptr: *const DynPixel,
    src_stride: isize,
    w: i32,
    h: i32,
    mx: i32,
    my: i32,
    bitdepth_max: i32,
    _dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
    _src: *const FFISafe<Rav1dPictureDataComponentOffset>,
) -> ());

impl mc::Fn {
    pub fn call<BD: BitDepth>(
        &self,
        dst: Rav1dPictureDataComponentOffset,
        src: Rav1dPictureDataComponentOffset,
        w: i32,
        h: i32,
        mx: i32,
        my: i32,
        bd: BD,
    ) {
        let dst_ptr = dst.as_mut_ptr::<BD>().cast();
        let dst_stride = dst.stride();
        let src_ptr = src.as_ptr::<BD>().cast();
        let src_stride = src.stride();
        let bd = bd.into_c();
        let dst = FFISafe::new(&dst);
        let src = FFISafe::new(&src);
        // SAFETY: Fallbacks `fn put_{8tpap,bilin}_rust` are safe; asm is supposed to do the same.
        unsafe {
            self.get()(
                dst_ptr, dst_stride, src_ptr, src_stride, w, h, mx, my, bd, dst, src,
            )
        }
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn mc_scaled(
    dst_ptr: *mut DynPixel,
    dst_stride: isize,
    src_ptr: *const DynPixel,
    src_stride: isize,
    w: i32,
    h: i32,
    mx: i32,
    my: i32,
    dx: i32,
    dy: i32,
    bitdepth_max: i32,
    _dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
    _src: *const FFISafe<Rav1dPictureDataComponentOffset>,
) -> ());

impl mc_scaled::Fn {
    pub fn call<BD: BitDepth>(
        &self,
        dst: Rav1dPictureDataComponentOffset,
        src: Rav1dPictureDataComponentOffset,
        w: i32,
        h: i32,
        mx: i32,
        my: i32,
        dx: i32,
        dy: i32,
        bd: BD,
    ) {
        let dst_ptr = dst.as_mut_ptr::<BD>().cast();
        let dst_stride = dst.stride();
        let src_ptr = src.as_ptr::<BD>().cast();
        let src_stride = src.stride();
        let bd = bd.into_c();
        let dst = FFISafe::new(&dst);
        let src = FFISafe::new(&src);
        // SAFETY: Fallbacks `fn put_{8tpap,bilin}_scaled_rust` are safe; asm is supposed to do the same.
        unsafe {
            self.get()(
                dst_ptr, dst_stride, src_ptr, src_stride, w, h, mx, my, dx, dy, bd, dst, src,
            )
        }
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn warp8x8(
    dst_ptr: *mut DynPixel,
    dst_stride: isize,
    src_ptr: *const DynPixel,
    src_stride: isize,
    abcd: &[i16; 4],
    mx: i32,
    my: i32,
    bitdepth_max: i32,
    _dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
    _src: *const FFISafe<Rav1dPictureDataComponentOffset>,
) -> ());

impl warp8x8::Fn {
    pub fn call<BD: BitDepth>(
        &self,
        dst: Rav1dPictureDataComponentOffset,
        src: Rav1dPictureDataComponentOffset,
        abcd: &[i16; 4],
        mx: i32,
        my: i32,
        bd: BD,
    ) {
        let dst_ptr = dst.as_mut_ptr::<BD>().cast();
        let dst_stride = dst.stride();
        let src_ptr = src.as_ptr::<BD>().cast();
        let src_stride = src.stride();
        let bd = bd.into_c();
        let dst = FFISafe::new(&dst);
        let src = FFISafe::new(&src);
        // SAFETY: Fallback `fn prep_c_rust` is safe; asm is supposed to do the same.
        unsafe {
            self.get()(
                dst_ptr, dst_stride, src_ptr, src_stride, abcd, mx, my, bd, dst, src,
            )
        }
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn mct(
    tmp: *mut i16,
    src_ptr: *const DynPixel,
    src_stride: isize,
    w: i32,
    h: i32,
    mx: i32,
    my: i32,
    bitdepth_max: i32,
    _src: *const FFISafe<Rav1dPictureDataComponentOffset>,
) -> ());

impl mct::Fn {
    pub fn call<BD: BitDepth>(
        &self,
        tmp: &mut [i16],
        src: Rav1dPictureDataComponentOffset,
        w: i32,
        h: i32,
        mx: i32,
        my: i32,
        bd: BD,
    ) {
        let tmp = tmp[..(w * h) as usize].as_mut_ptr();
        let src_ptr = src.as_ptr::<BD>().cast();
        let src_stride = src.stride();
        let bd = bd.into_c();
        let src = FFISafe::new(&src);
        // SAFETY: Fallbacks `fn prep_{8tpap,bilin}_rust` are safe; asm is supposed to do the same.
        unsafe { self.get()(tmp, src_ptr, src_stride, w, h, mx, my, bd, src) }
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn mct_scaled(
    tmp: *mut i16,
    src_ptr: *const DynPixel,
    src_stride: isize,
    w: i32,
    h: i32,
    mx: i32,
    my: i32,
    dx: i32,
    dy: i32,
    bitdepth_max: i32,
    _src: *const FFISafe<Rav1dPictureDataComponentOffset>,
) -> ());

impl mct_scaled::Fn {
    pub fn call<BD: BitDepth>(
        &self,
        tmp: &mut [i16],
        src: Rav1dPictureDataComponentOffset,
        w: i32,
        h: i32,
        mx: i32,
        my: i32,
        dx: i32,
        dy: i32,
        bd: BD,
    ) {
        let tmp = tmp[..(w * h) as usize].as_mut_ptr();
        let src_ptr = src.as_ptr::<BD>().cast();
        let src_stride = src.stride();
        let bd = bd.into_c();
        let src = FFISafe::new(&src);
        // SAFETY: Fallbacks `fn prep_{8tpap,bilin}_scaled_rust` are safe; asm is supposed to do the same.
        unsafe { self.get()(tmp, src_ptr, src_stride, w, h, mx, my, dx, dy, bd, src) }
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn warp8x8t(
    tmp: *mut i16,
    tmp_stride: usize,
    src_ptr: *const DynPixel,
    src_stride: isize,
    abcd: &[i16; 4],
    mx: i32,
    my: i32,
    bitdepth_max: i32,
    _tmp_len: usize,
    _src: *const FFISafe<Rav1dPictureDataComponentOffset>,
) -> ());

impl warp8x8t::Fn {
    pub fn call<BD: BitDepth>(
        &self,
        tmp: &mut [i16],
        tmp_stride: usize,
        src: Rav1dPictureDataComponentOffset,
        abcd: &[i16; 4],
        mx: i32,
        my: i32,
        bd: BD,
    ) {
        let tmp_len = tmp.len();
        let tmp = tmp.as_mut_ptr();
        let src_ptr = src.as_ptr::<BD>().cast();
        let src_stride = src.stride();
        let bd = bd.into_c();
        let src = FFISafe::new(&src);
        // SAFETY: Fallback `fn warp_affine_8x8t_rust` is safe; asm is supposed to do the same.
        unsafe {
            self.get()(
                tmp, tmp_stride, src_ptr, src_stride, abcd, mx, my, bd, tmp_len, src,
            )
        }
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn avg(
    dst_ptr: *mut DynPixel,
    dst_stride: isize,
    tmp1: &[i16; COMPINTER_LEN],
    tmp2: &[i16; COMPINTER_LEN],
    w: i32,
    h: i32,
    bitdepth_max: i32,
    _dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
) -> ());

impl avg::Fn {
    pub fn call<BD: BitDepth>(
        &self,
        dst: Rav1dPictureDataComponentOffset,
        tmp1: &[i16; COMPINTER_LEN],
        tmp2: &[i16; COMPINTER_LEN],
        w: i32,
        h: i32,
        bd: BD,
    ) {
        let dst_ptr = dst.as_mut_ptr::<BD>().cast();
        let dst_stride = dst.stride();
        let bd = bd.into_c();
        let dst = FFISafe::new(&dst);
        // SAFETY: Fallback `fn avg_rust` is safe; asm is supposed to do the same.
        unsafe { self.get()(dst_ptr, dst_stride, tmp1, tmp2, w, h, bd, dst) }
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn w_avg(
    dst_ptr: *mut DynPixel,
    dst_stride: isize,
    tmp1: &[i16; COMPINTER_LEN],
    tmp2: &[i16; COMPINTER_LEN],
    w: i32,
    h: i32,
    weight: i32,
    bitdepth_max: i32,
    _dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
) -> ());

impl w_avg::Fn {
    pub fn call<BD: BitDepth>(
        &self,
        dst: Rav1dPictureDataComponentOffset,
        tmp1: &[i16; COMPINTER_LEN],
        tmp2: &[i16; COMPINTER_LEN],
        w: i32,
        h: i32,
        weight: i32,
        bd: BD,
    ) {
        let dst_ptr = dst.as_mut_ptr::<BD>().cast();
        let dst_stride = dst.stride();
        let bd = bd.into_c();
        let dst = FFISafe::new(&dst);
        // SAFETY: Fallback `fn w_avg_rust` is safe; asm is supposed to do the same.
        unsafe { self.get()(dst_ptr, dst_stride, tmp1, tmp2, w, h, weight, bd, dst) }
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn mask(
    dst_ptr: *mut DynPixel,
    dst_stride: isize,
    tmp1: &[i16; COMPINTER_LEN],
    tmp2: &[i16; COMPINTER_LEN],
    w: i32,
    h: i32,
    mask: *const u8,
    bitdepth_max: i32,
    _dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
) -> ());

impl mask::Fn {
    pub fn call<BD: BitDepth>(
        &self,
        dst: Rav1dPictureDataComponentOffset,
        tmp1: &[i16; COMPINTER_LEN],
        tmp2: &[i16; COMPINTER_LEN],
        w: i32,
        h: i32,
        mask: &[u8],
        bd: BD,
    ) {
        let dst_ptr = dst.as_mut_ptr::<BD>().cast();
        let dst_stride = dst.stride();
        let mask = mask[..(w * h) as usize].as_ptr();
        let bd = bd.into_c();
        let dst = FFISafe::new(&dst);
        // SAFETY: Fallback `fn mask_rust` is safe; asm is supposed to do the same.
        unsafe { self.get()(dst_ptr, dst_stride, tmp1, tmp2, w, h, mask, bd, dst) }
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn w_mask(
    dst_ptr: *mut DynPixel,
    dst_stride: isize,
    tmp1: &[i16; COMPINTER_LEN],
    tmp2: &[i16; COMPINTER_LEN],
    w: i32,
    h: i32,
    mask: &mut [u8; SEG_MASK_LEN],
    sign: i32,
    bitdepth_max: i32,
    _dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
) -> ());

impl w_mask::Fn {
    pub fn call<BD: BitDepth>(
        &self,
        dst: Rav1dPictureDataComponentOffset,
        tmp1: &[i16; COMPINTER_LEN],
        tmp2: &[i16; COMPINTER_LEN],
        w: i32,
        h: i32,
        mask: &mut [u8; SEG_MASK_LEN],
        sign: i32,
        bd: BD,
    ) {
        let dst_ptr = dst.as_mut_ptr::<BD>().cast();
        let dst_stride = dst.stride();
        let bd = bd.into_c();
        let dst = FFISafe::new(&dst);
        // SAFETY: Fallback `fn w_mask_rust` is safe; asm is supposed to do the same.
        unsafe { self.get()(dst_ptr, dst_stride, tmp1, tmp2, w, h, mask, sign, bd, dst) }
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn blend(
    dst_ptr: *mut DynPixel,
    dst_stride: isize,
    tmp: *const [DynPixel; SCRATCH_INTER_INTRA_BUF_LEN],
    w: i32,
    h: i32,
    mask: *const u8,
    _dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
) -> ());

impl blend::Fn {
    pub fn call<BD: BitDepth>(
        &self,
        dst: Rav1dPictureDataComponentOffset,
        tmp: &[BD::Pixel; SCRATCH_INTER_INTRA_BUF_LEN],
        w: i32,
        h: i32,
        mask: &[u8],
    ) {
        let dst_ptr = dst.as_mut_ptr::<BD>().cast();
        let dst_stride = dst.stride();
        let tmp = ptr::from_ref(tmp).cast();
        let mask = mask[..(w * h) as usize].as_ptr();
        let dst = FFISafe::new(&dst);
        // SAFETY: Fallback `fn blend_rust` is safe; asm is supposed to do the same.
        unsafe { self.get()(dst_ptr, dst_stride, tmp, w, h, mask, dst) }
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn blend_dir(
    dst_ptr: *mut DynPixel,
    dst_stride: isize,
    tmp: *const [DynPixel; SCRATCH_LAP_LEN],
    w: i32,
    h: i32,
    _dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
) -> ());

impl blend_dir::Fn {
    pub fn call<BD: BitDepth>(
        &self,
        dst: Rav1dPictureDataComponentOffset,
        tmp: &[BD::Pixel; SCRATCH_LAP_LEN],
        w: i32,
        h: i32,
    ) {
        let dst_ptr = dst.as_mut_ptr::<BD>().cast();
        let dst_stride = dst.stride();
        let tmp = ptr::from_ref(tmp).cast();
        let dst = FFISafe::new(&dst);
        // SAFETY: Fallback `fn blend_{h,v}_rust` are safe; asm is supposed to do the same.
        unsafe { self.get()(dst_ptr, dst_stride, tmp, w, h, dst) }
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn emu_edge(
    bw: isize,
    bh: isize,
    iw: isize,
    ih: isize,
    x: isize,
    y: isize,
    dst: *mut [DynPixel; EMU_EDGE_LEN],
    dst_stride: isize,
    src_ptr: *const DynPixel,
    src_stride: isize,
    _src: *const FFISafe<Rav1dPictureDataComponent>,
) -> ());

impl emu_edge::Fn {
    pub fn call<BD: BitDepth>(
        &self,
        bw: isize,
        bh: isize,
        iw: isize,
        ih: isize,
        x: isize,
        y: isize,
        dst: &mut [BD::Pixel; EMU_EDGE_LEN],
        dst_pxstride: usize,
        src: &Rav1dPictureDataComponent,
    ) {
        let dst = dst.as_mut_ptr().cast();
        let dst_stride = (dst_pxstride * mem::size_of::<BD::Pixel>()) as isize;
        let src_ptr = src.as_strided_ptr::<BD>().cast();
        let src_stride = src.stride();
        let src = FFISafe::new(src);
        // SAFETY: Fallback `fn emu_edge_rust` is safe; asm is supposed to do the same.
        unsafe {
            self.get()(
                bw, bh, iw, ih, x, y, dst, dst_stride, src_ptr, src_stride, src,
            )
        }
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn resize(
    dst: *mut DynPixel,
    dst_stride: isize,
    src_ptr: *const DynPixel,
    src_stride: isize,
    dst_w: i32,
    h: i32,
    src_w: i32,
    dx: i32,
    mx: i32,
    bitdepth_max: i32,
    _src: *const FFISafe<Rav1dPictureDataComponentOffset>,
    _dst: *const FFISafe<WithOffset<PicOrBuf<AlignedVec64<u8>>>>,
) -> ());

impl resize::Fn {
    pub fn call<BD: BitDepth>(
        &self,
        dst: WithOffset<PicOrBuf<AlignedVec64<u8>>>,
        src: Rav1dPictureDataComponentOffset,
        dst_w: usize,
        h: usize,
        src_w: usize,
        dx: i32,
        mx: i32,
        bd: BD,
    ) {
        let dst_ptr = dst.as_mut_ptr::<BD>().cast();
        let dst_stride = dst.stride();
        let src_ptr = src.as_ptr::<BD>().cast();
        let src_stride = src.stride();
        let dst_w = dst_w as c_int;
        let h = h as c_int;
        let src_w = src_w as c_int;
        let bd = bd.into_c();
        let src = FFISafe::new(&src);
        let dst = FFISafe::new(&dst);
        // SAFETY: Fallback `fn resize_rust` is safe; asm is supposed to do the same.
        unsafe {
            self.get()(
                dst_ptr, dst_stride, src_ptr, src_stride, dst_w, h, src_w, dx, mx, bd, src, dst,
            )
        }
    }
}

pub struct Rav1dMCDSPContext {
    pub mc: enum_map_ty!(Filter2d, mc::Fn),
    pub mc_scaled: enum_map_ty!(Filter2d, mc_scaled::Fn),
    pub mct: enum_map_ty!(Filter2d, mct::Fn),
    pub mct_scaled: enum_map_ty!(Filter2d, mct_scaled::Fn),
    pub avg: avg::Fn,
    pub w_avg: w_avg::Fn,
    pub mask: mask::Fn,
    pub w_mask: enum_map_ty!(Rav1dPixelLayoutSubSampled, w_mask::Fn),
    pub blend: blend::Fn,
    pub blend_v: blend_dir::Fn,
    pub blend_h: blend_dir::Fn,
    pub warp8x8: warp8x8::Fn,
    pub warp8x8t: warp8x8t::Fn,
    pub emu_edge: emu_edge::Fn,
    pub resize: resize::Fn,
}

/// # Safety
///
/// Must be called by [`mc::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn put_c_erased<BD: BitDepth, const FILTER: usize>(
    _dst_ptr: *mut DynPixel,
    _dst_stride: isize,
    _src_ptr: *const DynPixel,
    _src_stride: isize,
    w: i32,
    h: i32,
    mx: i32,
    my: i32,
    bitdepth_max: i32,
    dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
    src: *const FFISafe<Rav1dPictureDataComponentOffset>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `mc::Fn::call`.
    let dst = *unsafe { FFISafe::get(dst) };
    // SAFETY: Was passed as `FFISafe::new(_)` in `mc::Fn::call`.
    let src = *unsafe { FFISafe::get(src) };
    let w = w as usize;
    let h = h as usize;
    let mx = mx as usize;
    let my = my as usize;
    let filter = Filter2d::from_repr(FILTER).unwrap();
    let hv = filter.hv();
    let bd = BD::from_c(bitdepth_max);
    match filter {
        Filter2d::Bilinear => put_bilin_rust(dst, src, w, h, mx, my, bd),
        _ => put_8tap_rust(dst, src, w, h, mx, my, hv, bd),
    }
}

/// # Safety
///
/// Must be called by [`mc_scaled::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn put_scaled_c_erased<BD: BitDepth, const FILTER: usize>(
    _dst_ptr: *mut DynPixel,
    _dst_stride: isize,
    _src_ptr: *const DynPixel,
    _src_stride: isize,
    w: i32,
    h: i32,
    mx: i32,
    my: i32,
    dx: i32,
    dy: i32,
    bitdepth_max: i32,
    dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
    src: *const FFISafe<Rav1dPictureDataComponentOffset>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `mc_scaled::Fn::call`.
    let dst = *unsafe { FFISafe::get(dst) };
    // SAFETY: Was passed as `FFISafe::new(_)` in `mc_scaled::Fn::call`.
    let src = *unsafe { FFISafe::get(src) };
    let w = w as usize;
    let h = h as usize;
    let mx = mx as usize;
    let my = my as usize;
    let dx = dx as usize;
    let dy = dy as usize;
    let filter = Filter2d::from_repr(FILTER).unwrap();
    let hv = filter.hv();
    let bd = BD::from_c(bitdepth_max);
    match filter {
        Filter2d::Bilinear => put_bilin_scaled_rust(dst, src, w, h, mx, my, dx, dy, bd),
        _ => put_8tap_scaled_rust(dst, src, w, h, mx, my, dx, dy, hv, bd),
    }
}

/// # Safety
///
/// Must be called by [`mct::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn prep_c_erased<BD: BitDepth, const FILTER: usize>(
    tmp: *mut i16,
    _src_ptr: *const DynPixel,
    _src_stride: isize,
    w: i32,
    h: i32,
    mx: i32,
    my: i32,
    bitdepth_max: i32,
    src: *const FFISafe<Rav1dPictureDataComponentOffset>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `mct::Fn::call`.
    let src = *unsafe { FFISafe::get(src) };
    let w = w as usize;
    let h = h as usize;
    // SAFETY: Length sliced in `mct::Fn::call`.
    let tmp = unsafe { slice::from_raw_parts_mut(tmp, w * h) };
    let mx = mx as usize;
    let my = my as usize;
    let filter = Filter2d::from_repr(FILTER).unwrap();
    let hv = filter.hv();
    let bd = BD::from_c(bitdepth_max);
    match filter {
        Filter2d::Bilinear => prep_bilin_rust(tmp, src, w, h, mx, my, bd),
        _ => prep_8tap_rust(tmp, src, w, h, mx, my, hv, bd),
    }
}

/// # Safety
///
/// Must be called by [`mct_scaled::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn prep_scaled_c_erased<BD: BitDepth, const FILTER: usize>(
    tmp: *mut i16,
    _src_ptr: *const DynPixel,
    _src_stride: isize,
    w: i32,
    h: i32,
    mx: i32,
    my: i32,
    dx: i32,
    dy: i32,
    bitdepth_max: i32,
    src: *const FFISafe<Rav1dPictureDataComponentOffset>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `mct_scaled::Fn::call`.
    let src = *unsafe { FFISafe::get(src) };
    let w = w as usize;
    let h = h as usize;
    // SAFETY: Length sliced in `mct_scaled::Fn::call`.
    let tmp = unsafe { slice::from_raw_parts_mut(tmp, w * h) };
    let mx = mx as usize;
    let my = my as usize;
    let dx = dx as usize;
    let dy = dy as usize;
    let filter = Filter2d::from_repr(FILTER).unwrap();
    let hv = filter.hv();
    let bd = BD::from_c(bitdepth_max);
    match filter {
        Filter2d::Bilinear => prep_bilin_scaled_rust(tmp, src, w, h, mx, my, dx, dy, bd),
        _ => prep_8tap_scaled_rust(tmp, src, w, h, mx, my, dx, dy, hv, bd),
    }
}

/// # Safety
///
/// Must be called by [`avg::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn avg_c_erased<BD: BitDepth>(
    _dst_ptr: *mut DynPixel,
    _dst_stride: isize,
    tmp1: &[i16; COMPINTER_LEN],
    tmp2: &[i16; COMPINTER_LEN],
    w: i32,
    h: i32,
    bitdepth_max: i32,
    dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `avg::Fn::call`.
    let dst = *unsafe { FFISafe::get(dst) };
    let w = w as usize;
    let h = h as usize;
    let bd = BD::from_c(bitdepth_max);
    avg_rust(dst, tmp1, tmp2, w, h, bd)
}

/// # Safety
///
/// Must be called by [`w_avg::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn w_avg_c_erased<BD: BitDepth>(
    _dst_ptr: *mut DynPixel,
    _dst_stride: isize,
    tmp1: &[i16; COMPINTER_LEN],
    tmp2: &[i16; COMPINTER_LEN],
    w: i32,
    h: i32,
    weight: i32,
    bitdepth_max: i32,
    dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `w_avg::Fn::call`.
    let dst = *unsafe { FFISafe::get(dst) };
    let w = w as usize;
    let h = h as usize;
    let bd = BD::from_c(bitdepth_max);
    w_avg_rust(dst, tmp1, tmp2, w, h, weight, bd)
}

/// # Safety
///
/// Must be called by [`mask::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn mask_c_erased<BD: BitDepth>(
    _dst_ptr: *mut DynPixel,
    _dst_stride: isize,
    tmp1: &[i16; COMPINTER_LEN],
    tmp2: &[i16; COMPINTER_LEN],
    w: i32,
    h: i32,
    mask: *const u8,
    bitdepth_max: i32,
    dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `mask::Fn::call`.
    let dst = *unsafe { FFISafe::get(dst) };
    let w = w as usize;
    let h = h as usize;
    // SAFETY: Length sliced in `mask::Fn::call`.
    let mask = unsafe { slice::from_raw_parts(mask, w * h) };
    let bd = BD::from_c(bitdepth_max);
    mask_rust(dst, tmp1, tmp2, w, h, mask, bd)
}

/// # Safety
///
/// Must be called by [`w_mask::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn w_mask_c_erased<const SS_HOR: bool, const SS_VER: bool, BD: BitDepth>(
    _dst_ptr: *mut DynPixel,
    _dst_stride: isize,
    tmp1: &[i16; COMPINTER_LEN],
    tmp2: &[i16; COMPINTER_LEN],
    w: i32,
    h: i32,
    mask: &mut [u8; SEG_MASK_LEN],
    sign: i32,
    bitdepth_max: i32,
    dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `w_mask::Fn::call`.
    let dst = *unsafe { FFISafe::get(dst) };
    let w = w as usize;
    let h = h as usize;
    debug_assert!(sign == 1 || sign == 0);
    let sign = sign != 0;
    let bd = BD::from_c(bitdepth_max);
    w_mask_rust(dst, tmp1, tmp2, w, h, mask, sign, SS_HOR, SS_VER, bd)
}

/// # Safety
///
/// Must be called by [`blend::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn blend_c_erased<BD: BitDepth>(
    _dst_ptr: *mut DynPixel,
    _dst_stride: isize,
    tmp: *const [DynPixel; SCRATCH_INTER_INTRA_BUF_LEN],
    w: i32,
    h: i32,
    mask: *const u8,
    dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `blend::Fn::call`.
    let dst = *unsafe { FFISafe::get(dst) };
    // SAFETY: Reverse of cast in `blend::Fn::call`.
    let tmp = unsafe { &*tmp.cast() };
    let w = w as usize;
    let h = h as usize;
    // SAFETY: Length sliced in `blend::Fn::call`.
    let mask = unsafe { slice::from_raw_parts(mask, w * h) };
    blend_rust::<BD>(dst, tmp, w, h, mask)
}

/// # Safety
///
/// Must be called by [`blend_dir::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn blend_v_c_erased<BD: BitDepth>(
    _dst_ptr: *mut DynPixel,
    _dst_stride: isize,
    tmp: *const [DynPixel; SCRATCH_LAP_LEN],
    w: i32,
    h: i32,
    dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `blend_dir::Fn::call`.
    let dst = *unsafe { FFISafe::get(dst) };
    // SAFETY: Reverse of cast in `blend_dir::Fn::call`.
    let tmp = unsafe { &*tmp.cast() };
    let w = w as usize;
    let h = h as usize;
    blend_v_rust::<BD>(dst, tmp, w, h)
}

/// # Safety
///
/// Must be called by [`blend_dir::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn blend_h_c_erased<BD: BitDepth>(
    _dst_ptr: *mut DynPixel,
    _dst_stride: isize,
    tmp: *const [DynPixel; SCRATCH_LAP_LEN],
    w: i32,
    h: i32,
    dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `blend_dir::Fn::call`.
    let dst = *unsafe { FFISafe::get(dst) };
    // SAFETY: Reverse of cast in `blend_dir::Fn::call`.
    let tmp = unsafe { &*tmp.cast() };
    let w = w as usize;
    let h = h as usize;
    blend_h_rust::<BD>(dst, tmp, w, h)
}

/// # Safety
///
/// Must be called by [`warp8x8::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn warp_affine_8x8_c_erased<BD: BitDepth>(
    _dst_ptr: *mut DynPixel,
    _dst_stride: isize,
    _src_ptr: *const DynPixel,
    _src_stride: isize,
    abcd: &[i16; 4],
    mx: i32,
    my: i32,
    bitdepth_max: i32,
    dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
    src: *const FFISafe<Rav1dPictureDataComponentOffset>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `warp_8x8::Fn::call`.
    let dst = *unsafe { FFISafe::get(dst) };
    // SAFETY: Was passed as `FFISafe::new(_)` in `warp_8x8::Fn::call`.
    let src = *unsafe { FFISafe::get(src) };
    let bd = BD::from_c(bitdepth_max);
    warp_affine_8x8_rust(dst, src, abcd, mx, my, bd)
}

/// # Safety
///
/// Must be called by [`warp8x8t::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn warp_affine_8x8t_c_erased<BD: BitDepth>(
    tmp: *mut i16,
    tmp_stride: usize,
    _src_ptr: *const DynPixel,
    _src_stride: isize,
    abcd: &[i16; 4],
    mx: i32,
    my: i32,
    bitdepth_max: i32,
    tmp_len: usize,
    src: *const FFISafe<Rav1dPictureDataComponentOffset>,
) {
    // SAFETY: `warp8x8t::Fn::call` passed `tmp.len()` as `tmp_len`.
    let tmp = unsafe { slice::from_raw_parts_mut(tmp, tmp_len) };
    // SAFETY: Was passed as `FFISafe::new(_)` in `warp8x8t::Fn::call`.
    let src = *unsafe { FFISafe::get(src) };
    let bd = BD::from_c(bitdepth_max);
    warp_affine_8x8t_rust(tmp, tmp_stride, src, abcd, mx, my, bd)
}

#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn emu_edge_c_erased<BD: BitDepth>(
    bw: isize,
    bh: isize,
    iw: isize,
    ih: isize,
    x: isize,
    y: isize,
    dst: *mut [DynPixel; EMU_EDGE_LEN],
    dst_stride: isize,
    _ref_ptr: *const DynPixel,
    _ref_stride: isize,
    r#ref: *const FFISafe<Rav1dPictureDataComponent>,
) {
    // SAFETY: Reverse cast is done in `fn emu_edge::Fn::call`.
    let dst = unsafe { &mut *dst.cast() };
    // Is `usize` in `fn emu_edge::Fn::call`.
    let dst_stride = dst_stride as usize;
    // SAFETY: Was passed as `FFISafe::new(_)` in `fn emu_edge::Fn::call`.
    let r#ref = unsafe { FFISafe::get(r#ref) };
    emu_edge_rust::<BD>(bw, bh, iw, ih, x, y, dst, dst_stride, r#ref)
}

unsafe extern "C" fn resize_c_erased<BD: BitDepth>(
    _dst_ptr: *mut DynPixel,
    _dst_stride: isize,
    _src_ptr: *const DynPixel,
    _src_stride: isize,
    dst_w: i32,
    h: i32,
    src_w: i32,
    dx: i32,
    mx0: i32,
    bitdepth_max: i32,
    src: *const FFISafe<Rav1dPictureDataComponentOffset>,
    dst: *const FFISafe<WithOffset<PicOrBuf<AlignedVec64<u8>>>>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `resize::Fn::call`.
    let dst = *unsafe { FFISafe::get(dst) };
    // SAFETY: Was passed as `FFISafe::new(_)` in `resize::Fn::call`.
    let src = *unsafe { FFISafe::get(src) };
    let dst_w = dst_w as usize;
    let h = h as usize;
    let src_w = src_w as usize;
    let bd = BD::from_c(bitdepth_max);
    resize_rust(dst, src, dst_w, h, src_w, dx, mx0, bd)
}

impl Rav1dMCDSPContext {
    pub const fn default<BD: BitDepth>() -> Self {
        Self {
            mc: enum_map!(Filter2d => mc::Fn; match key {
                Regular8Tap => mc::Fn::new(put_c_erased::<BD, {Regular8Tap as _}>),
                RegularSmooth8Tap => mc::Fn::new(put_c_erased::<BD, {RegularSmooth8Tap as _}>),
                RegularSharp8Tap => mc::Fn::new(put_c_erased::<BD, {RegularSharp8Tap as _}>),
                SharpRegular8Tap => mc::Fn::new(put_c_erased::<BD, {SharpRegular8Tap as _}>),
                SharpSmooth8Tap => mc::Fn::new(put_c_erased::<BD, {SharpSmooth8Tap as _}>),
                Sharp8Tap => mc::Fn::new(put_c_erased::<BD, {Sharp8Tap as _}>),
                SmoothRegular8Tap => mc::Fn::new(put_c_erased::<BD, {SmoothRegular8Tap as _}>),
                Smooth8Tap => mc::Fn::new(put_c_erased::<BD, {Smooth8Tap as _}>),
                SmoothSharp8Tap => mc::Fn::new(put_c_erased::<BD, {SmoothSharp8Tap as _}>),
                Bilinear => mc::Fn::new(put_c_erased::<BD, {Bilinear as _}>),
            }),
            mct: enum_map!(Filter2d => mct::Fn; match key {
                Regular8Tap => mct::Fn::new(prep_c_erased::<BD, {Regular8Tap as _}>),
                RegularSmooth8Tap => mct::Fn::new(prep_c_erased::<BD, {RegularSmooth8Tap as _}>),
                RegularSharp8Tap => mct::Fn::new(prep_c_erased::<BD, {RegularSharp8Tap as _}>),
                SharpRegular8Tap => mct::Fn::new(prep_c_erased::<BD, {SharpRegular8Tap as _}>),
                SharpSmooth8Tap => mct::Fn::new(prep_c_erased::<BD, {SharpSmooth8Tap as _}>),
                Sharp8Tap => mct::Fn::new(prep_c_erased::<BD, {Sharp8Tap as _}>),
                SmoothRegular8Tap => mct::Fn::new(prep_c_erased::<BD, {SmoothRegular8Tap as _}>),
                Smooth8Tap => mct::Fn::new(prep_c_erased::<BD, {Smooth8Tap as _}>),
                SmoothSharp8Tap => mct::Fn::new(prep_c_erased::<BD, {SmoothSharp8Tap as _}>),
                Bilinear => mct::Fn::new(prep_c_erased::<BD, {Bilinear as _}>),
            }),
            mc_scaled: enum_map!(Filter2d => mc_scaled::Fn; match key {
                Regular8Tap => mc_scaled::Fn::new(put_scaled_c_erased::<BD, {Regular8Tap as _}>),
                RegularSmooth8Tap => mc_scaled::Fn::new(put_scaled_c_erased::<BD, {RegularSmooth8Tap as _}>),
                RegularSharp8Tap => mc_scaled::Fn::new(put_scaled_c_erased::<BD, {RegularSharp8Tap as _}>),
                SharpRegular8Tap => mc_scaled::Fn::new(put_scaled_c_erased::<BD, {SharpRegular8Tap as _}>),
                SharpSmooth8Tap => mc_scaled::Fn::new(put_scaled_c_erased::<BD, {SharpSmooth8Tap as _}>),
                Sharp8Tap => mc_scaled::Fn::new(put_scaled_c_erased::<BD, {Sharp8Tap as _}>),
                SmoothRegular8Tap => mc_scaled::Fn::new(put_scaled_c_erased::<BD, {SmoothRegular8Tap as _}>),
                Smooth8Tap => mc_scaled::Fn::new(put_scaled_c_erased::<BD, {Smooth8Tap as _}>),
                SmoothSharp8Tap => mc_scaled::Fn::new(put_scaled_c_erased::<BD, {SmoothSharp8Tap as _}>),
                Bilinear => mc_scaled::Fn::new(put_scaled_c_erased::<BD, {Bilinear as _}>),
            }),
            mct_scaled: enum_map!(Filter2d => mct_scaled::Fn; match key {
                Regular8Tap => mct_scaled::Fn::new(prep_scaled_c_erased::<BD, {Regular8Tap as _}>),
                RegularSmooth8Tap => mct_scaled::Fn::new(prep_scaled_c_erased::<BD, {RegularSmooth8Tap as _}>),
                RegularSharp8Tap => mct_scaled::Fn::new(prep_scaled_c_erased::<BD, {RegularSharp8Tap as _}>),
                SharpRegular8Tap => mct_scaled::Fn::new(prep_scaled_c_erased::<BD, {SharpRegular8Tap as _}>),
                SharpSmooth8Tap => mct_scaled::Fn::new(prep_scaled_c_erased::<BD, {SharpSmooth8Tap as _}>),
                Sharp8Tap => mct_scaled::Fn::new(prep_scaled_c_erased::<BD, {Sharp8Tap as _}>),
                SmoothRegular8Tap => mct_scaled::Fn::new(prep_scaled_c_erased::<BD, {SmoothRegular8Tap as _}>),
                Smooth8Tap => mct_scaled::Fn::new(prep_scaled_c_erased::<BD, {Smooth8Tap as _}>),
                SmoothSharp8Tap => mct_scaled::Fn::new(prep_scaled_c_erased::<BD, {SmoothSharp8Tap as _}>),
                Bilinear => mct_scaled::Fn::new(prep_scaled_c_erased::<BD, {Bilinear as _}>),
            }),
            avg: avg::Fn::new(avg_c_erased::<BD>),
            w_avg: w_avg::Fn::new(w_avg_c_erased::<BD>),
            mask: mask::Fn::new(mask_c_erased::<BD>),
            w_mask: enum_map!(Rav1dPixelLayoutSubSampled => w_mask::Fn; match key {
                I420 => w_mask::Fn::new(w_mask_c_erased::<true, true, BD>),
                I422 => w_mask::Fn::new(w_mask_c_erased::<true, false, BD>),
                I444 => w_mask::Fn::new(w_mask_c_erased::<false, false, BD>),
            }),
            blend: blend::Fn::new(blend_c_erased::<BD>),
            blend_v: blend_dir::Fn::new(blend_v_c_erased::<BD>),
            blend_h: blend_dir::Fn::new(blend_h_c_erased::<BD>),
            warp8x8: warp8x8::Fn::new(warp_affine_8x8_c_erased::<BD>),
            warp8x8t: warp8x8t::Fn::new(warp_affine_8x8t_c_erased::<BD>),
            emu_edge: emu_edge::Fn::new(emu_edge_c_erased::<BD>),
            resize: resize::Fn::new(resize_c_erased::<BD>),
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

        self.avg = bd_fn!(avg::decl_fn, BD, avg, ssse3);
        self.w_avg = bd_fn!(w_avg::decl_fn, BD, w_avg, ssse3);
        self.mask = bd_fn!(mask::decl_fn, BD, mask, ssse3);

        self.w_mask = enum_map!(Rav1dPixelLayoutSubSampled => w_mask::Fn; match key {
            I420 => bd_fn!(w_mask::decl_fn, BD, w_mask_420, ssse3),
            I422 => bd_fn!(w_mask::decl_fn, BD, w_mask_422, ssse3),
            I444 => bd_fn!(w_mask::decl_fn, BD, w_mask_444, ssse3),
        });

        self.blend = bd_fn!(blend::decl_fn, BD, blend, ssse3);
        self.blend_v = bd_fn!(blend_dir::decl_fn, BD, blend_v, ssse3);
        self.blend_h = bd_fn!(blend_dir::decl_fn, BD, blend_h, ssse3);
        self.warp8x8 = bd_fn!(warp8x8::decl_fn, BD, warp_affine_8x8, ssse3);
        self.warp8x8t = bd_fn!(warp8x8t::decl_fn, BD, warp_affine_8x8t, ssse3);
        self.emu_edge = bd_fn!(emu_edge::decl_fn, BD, emu_edge, ssse3);
        self.resize = bd_fn!(resize::decl_fn, BD, resize, ssse3);

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

            self.avg = bd_fn!(avg::decl_fn, BD, avg, avx2);
            self.w_avg = bd_fn!(w_avg::decl_fn, BD, w_avg, avx2);
            self.mask = bd_fn!(mask::decl_fn, BD, mask, avx2);

            self.w_mask = enum_map!(Rav1dPixelLayoutSubSampled => w_mask::Fn; match key {
                I420 => bd_fn!(w_mask::decl_fn, BD, w_mask_420, avx2),
                I422 => bd_fn!(w_mask::decl_fn, BD, w_mask_422, avx2),
                I444 => bd_fn!(w_mask::decl_fn, BD, w_mask_444, avx2),
            });

            self.blend = bd_fn!(blend::decl_fn, BD, blend, avx2);
            self.blend_v = bd_fn!(blend_dir::decl_fn, BD, blend_v, avx2);
            self.blend_h = bd_fn!(blend_dir::decl_fn, BD, blend_h, avx2);
            self.warp8x8 = bd_fn!(warp8x8::decl_fn, BD, warp_affine_8x8, avx2);
            self.warp8x8t = bd_fn!(warp8x8t::decl_fn, BD, warp_affine_8x8t, avx2);
            self.emu_edge = bd_fn!(emu_edge::decl_fn, BD, emu_edge, avx2);
            self.resize = bd_fn!(resize::decl_fn, BD, resize, avx2);

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

            self.avg = bd_fn!(avg::decl_fn, BD, avg, avx512icl);
            self.w_avg = bd_fn!(w_avg::decl_fn, BD, w_avg, avx512icl);
            self.mask = bd_fn!(mask::decl_fn, BD, mask, avx512icl);

            self.w_mask = enum_map!(Rav1dPixelLayoutSubSampled => w_mask::Fn; match key {
                I420 => bd_fn!(w_mask::decl_fn, BD, w_mask_420, avx512icl),
                I422 => bd_fn!(w_mask::decl_fn, BD, w_mask_422, avx512icl),
                I444 => bd_fn!(w_mask::decl_fn, BD, w_mask_444, avx512icl),
            });

            self.blend = bd_fn!(blend::decl_fn, BD, blend, avx512icl);
            self.blend_v = bd_fn!(blend_dir::decl_fn, BD, blend_v, avx512icl);
            self.blend_h = bd_fn!(blend_dir::decl_fn, BD, blend_h, avx512icl);

            if !flags.contains(CpuFlags::SLOW_GATHER) {
                self.resize = bd_fn!(resize::decl_fn, BD, resize, avx512icl);
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

        self.avg = bd_fn!(avg::decl_fn, BD, avg, neon);
        self.w_avg = bd_fn!(w_avg::decl_fn, BD, w_avg, neon);
        self.mask = bd_fn!(mask::decl_fn, BD, mask, neon);
        self.blend = bd_fn!(blend::decl_fn, BD, blend, neon);
        self.blend_h = bd_fn!(blend_dir::decl_fn, BD, blend_h, neon);
        self.blend_v = bd_fn!(blend_dir::decl_fn, BD, blend_v, neon);

        self.w_mask = enum_map!(Rav1dPixelLayoutSubSampled => w_mask::Fn; match key {
            I420 => bd_fn!(w_mask::decl_fn, BD, w_mask_420, neon),
            I422 => bd_fn!(w_mask::decl_fn, BD, w_mask_422, neon),
            I444 => bd_fn!(w_mask::decl_fn, BD, w_mask_444, neon),
        });

        self.warp8x8 = bd_fn!(warp8x8::decl_fn, BD, warp_affine_8x8, neon);
        self.warp8x8t = bd_fn!(warp8x8t::decl_fn, BD, warp_affine_8x8t, neon);
        self.emu_edge = bd_fn!(emu_edge::decl_fn, BD, emu_edge, neon);

        #[cfg(all(target_arch = "aarch64", feature = "asm_arm64_dotprod"))]
        if BD::BITDEPTH == 8 {
            if !flags.contains(CpuFlags::DOTPROD) {
                return self;
            }

            self.mc = enum_map!(Filter2d => mc::Fn; match key {
                Regular8Tap => bpc_fn!(mc::decl_fn, 8 bpc, put_8tap_regular, neon_dotprod),
                RegularSmooth8Tap => bpc_fn!(mc::decl_fn, 8 bpc, put_8tap_regular_smooth, neon_dotprod),
                RegularSharp8Tap => bpc_fn!(mc::decl_fn, 8 bpc, put_8tap_regular_sharp, neon_dotprod),
                SmoothRegular8Tap => bpc_fn!(mc::decl_fn, 8 bpc, put_8tap_smooth_regular, neon_dotprod),
                Smooth8Tap => bpc_fn!(mc::decl_fn, 8 bpc, put_8tap_smooth, neon_dotprod),
                SmoothSharp8Tap => bpc_fn!(mc::decl_fn, 8 bpc, put_8tap_smooth_sharp, neon_dotprod),
                SharpRegular8Tap => bpc_fn!(mc::decl_fn, 8 bpc, put_8tap_sharp_regular, neon_dotprod),
                SharpSmooth8Tap => bpc_fn!(mc::decl_fn, 8 bpc, put_8tap_sharp_smooth, neon_dotprod),
                Sharp8Tap => bpc_fn!(mc::decl_fn, 8 bpc, put_8tap_sharp, neon_dotprod),
                Bilinear => bpc_fn!(mc::decl_fn, 8 bpc, put_bilin, neon),
            });
            self.mct = enum_map!(Filter2d => mct::Fn; match key {
                Regular8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_regular, neon_dotprod),
                RegularSmooth8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_regular_smooth, neon_dotprod),
                RegularSharp8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_regular_sharp, neon_dotprod),
                SmoothRegular8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_smooth_regular, neon_dotprod),
                Smooth8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_smooth, neon_dotprod),
                SmoothSharp8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_smooth_sharp, neon_dotprod),
                SharpRegular8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_sharp_regular, neon_dotprod),
                SharpSmooth8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_sharp_smooth, neon_dotprod),
                Sharp8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_sharp, neon_dotprod),
                Bilinear => bpc_fn!(mct::decl_fn, 8 bpc, prep_bilin, neon),
            });
        }

        #[cfg(all(target_arch = "aarch64", feature = "asm_arm64_i8mm"))]
        if BD::BITDEPTH == 8 {
            if !flags.contains(CpuFlags::I8MM) {
                return self;
            }

            self.mc = enum_map!(Filter2d => mc::Fn; match key {
                Regular8Tap => bpc_fn!(mc::decl_fn, 8 bpc, put_8tap_regular, neon_i8mm),
                RegularSmooth8Tap => bpc_fn!(mc::decl_fn, 8 bpc, put_8tap_regular_smooth, neon_i8mm),
                RegularSharp8Tap => bpc_fn!(mc::decl_fn, 8 bpc, put_8tap_regular_sharp, neon_i8mm),
                SmoothRegular8Tap => bpc_fn!(mc::decl_fn, 8 bpc, put_8tap_smooth_regular, neon_i8mm),
                Smooth8Tap => bpc_fn!(mc::decl_fn, 8 bpc, put_8tap_smooth, neon_i8mm),
                SmoothSharp8Tap => bpc_fn!(mc::decl_fn, 8 bpc, put_8tap_smooth_sharp, neon_i8mm),
                SharpRegular8Tap => bpc_fn!(mc::decl_fn, 8 bpc, put_8tap_sharp_regular, neon_i8mm),
                SharpSmooth8Tap => bpc_fn!(mc::decl_fn, 8 bpc, put_8tap_sharp_smooth, neon_i8mm),
                Sharp8Tap => bpc_fn!(mc::decl_fn, 8 bpc, put_8tap_sharp, neon_i8mm),
                Bilinear => bpc_fn!(mc::decl_fn, 8 bpc, put_bilin, neon),
            });
            self.mct = enum_map!(Filter2d => mct::Fn; match key {
                Regular8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_regular, neon_i8mm),
                RegularSmooth8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_regular_smooth, neon_i8mm),
                RegularSharp8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_regular_sharp, neon_i8mm),
                SmoothRegular8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_smooth_regular, neon_i8mm),
                Smooth8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_smooth, neon_i8mm),
                SmoothSharp8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_smooth_sharp, neon_i8mm),
                SharpRegular8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_sharp_regular, neon_i8mm),
                SharpSmooth8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_sharp_smooth, neon_i8mm),
                Sharp8Tap => bpc_fn!(mct::decl_fn, 8 bpc, prep_8tap_sharp, neon_i8mm),
                Bilinear => bpc_fn!(mct::decl_fn, 8 bpc, prep_bilin, neon),
            });
        }

        #[cfg(all(target_arch = "aarch64", feature = "asm_arm64_sve2"))]
        if BD::BITDEPTH == 16 {
            if !flags.contains(CpuFlags::SVE2) {
                return self;
            }
            self.mc = enum_map!(Filter2d => mc::Fn; match key {
                Regular8Tap => bpc_fn!(mc::decl_fn, 16 bpc, put_8tap_regular, sve2),
                RegularSmooth8Tap => bpc_fn!(mc::decl_fn, 16 bpc, put_8tap_regular_smooth, sve2),
                RegularSharp8Tap => bpc_fn!(mc::decl_fn, 16 bpc, put_8tap_regular_sharp, sve2),
                SmoothRegular8Tap => bpc_fn!(mc::decl_fn, 16 bpc, put_8tap_smooth_regular, sve2),
                Smooth8Tap => bpc_fn!(mc::decl_fn, 16 bpc, put_8tap_smooth, sve2),
                SmoothSharp8Tap => bpc_fn!(mc::decl_fn, 16 bpc, put_8tap_smooth_sharp, sve2),
                SharpRegular8Tap => bpc_fn!(mc::decl_fn, 16 bpc, put_8tap_sharp_regular, sve2),
                SharpSmooth8Tap => bpc_fn!(mc::decl_fn, 16 bpc, put_8tap_sharp_smooth, sve2),
                Sharp8Tap => bpc_fn!(mc::decl_fn, 16 bpc, put_8tap_sharp, sve2),
                Bilinear => bpc_fn!(mc::decl_fn, 16 bpc, put_bilin, neon),
            });
            self.mct = enum_map!(Filter2d => mct::Fn; match key {
                Regular8Tap => bpc_fn!(mct::decl_fn, 16 bpc, prep_8tap_regular, sve2),
                RegularSmooth8Tap => bpc_fn!(mct::decl_fn, 16 bpc, prep_8tap_regular_smooth, sve2),
                RegularSharp8Tap => bpc_fn!(mct::decl_fn, 16 bpc, prep_8tap_regular_sharp, sve2),
                SmoothRegular8Tap => bpc_fn!(mct::decl_fn, 16 bpc, prep_8tap_smooth_regular, sve2),
                Smooth8Tap => bpc_fn!(mct::decl_fn, 16 bpc, prep_8tap_smooth, sve2),
                SmoothSharp8Tap => bpc_fn!(mct::decl_fn, 16 bpc, prep_8tap_smooth_sharp, sve2),
                SharpRegular8Tap => bpc_fn!(mct::decl_fn, 16 bpc, prep_8tap_sharp_regular, sve2),
                SharpSmooth8Tap => bpc_fn!(mct::decl_fn, 16 bpc, prep_8tap_sharp_smooth, sve2),
                Sharp8Tap => bpc_fn!(mct::decl_fn, 16 bpc, prep_8tap_sharp, sve2),
                Bilinear => bpc_fn!(mct::decl_fn, 16 bpc, prep_bilin, neon),
            });
        }

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
