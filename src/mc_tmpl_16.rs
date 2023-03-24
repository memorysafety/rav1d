use ::libc;
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    static dav1d_mc_subpel_filters: [[[int8_t; 8]; 15]; 6];
    static dav1d_mc_warp_filter: [[int8_t; 8]; 193];
    static dav1d_resize_filter: [[int8_t; 8]; 64];
    static dav1d_obmc_masks: [uint8_t; 64];
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type ptrdiff_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type intptr_t = libc::c_long;
pub type pixel = uint16_t;
pub type Dav1dFilterMode = libc::c_uint;
pub const DAV1D_FILTER_SWITCHABLE: Dav1dFilterMode = 4;
pub const DAV1D_N_FILTERS: Dav1dFilterMode = 4;
pub const DAV1D_FILTER_BILINEAR: Dav1dFilterMode = 3;
pub const DAV1D_N_SWITCHABLE_FILTERS: Dav1dFilterMode = 3;
pub const DAV1D_FILTER_8TAP_SHARP: Dav1dFilterMode = 2;
pub const DAV1D_FILTER_8TAP_SMOOTH: Dav1dFilterMode = 1;
pub const DAV1D_FILTER_8TAP_REGULAR: Dav1dFilterMode = 0;
pub type Filter2d = libc::c_uint;
pub const N_2D_FILTERS: Filter2d = 10;
pub const FILTER_2D_BILINEAR: Filter2d = 9;
pub const FILTER_2D_8TAP_SMOOTH_SHARP: Filter2d = 8;
pub const FILTER_2D_8TAP_SMOOTH: Filter2d = 7;
pub const FILTER_2D_8TAP_SMOOTH_REGULAR: Filter2d = 6;
pub const FILTER_2D_8TAP_SHARP: Filter2d = 5;
pub const FILTER_2D_8TAP_SHARP_SMOOTH: Filter2d = 4;
pub const FILTER_2D_8TAP_SHARP_REGULAR: Filter2d = 3;
pub const FILTER_2D_8TAP_REGULAR_SHARP: Filter2d = 2;
pub const FILTER_2D_8TAP_REGULAR_SMOOTH: Filter2d = 1;
pub const FILTER_2D_8TAP_REGULAR: Filter2d = 0;
pub type mc_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
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
        *mut pixel,
        ptrdiff_t,
        *const pixel,
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
        *mut pixel,
        ptrdiff_t,
        *const pixel,
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
        *const pixel,
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
        *const pixel,
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
        *const pixel,
        ptrdiff_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type avg_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
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
        *mut pixel,
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
        *mut pixel,
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
        *mut pixel,
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
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        libc::c_int,
        libc::c_int,
        *const uint8_t,
    ) -> (),
>;
pub type blend_dir_fn = Option<
    unsafe extern "C" fn(*mut pixel, ptrdiff_t, *const pixel, libc::c_int, libc::c_int) -> (),
>;
pub type emu_edge_fn = Option<
    unsafe extern "C" fn(
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
    ) -> (),
>;
pub type resize_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
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
#[derive(Copy, Clone)]
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
#[inline]
unsafe extern "C" fn clz(mask: libc::c_uint) -> libc::c_int {
    return mask.leading_zeros() as i32;
}
#[inline]
unsafe extern "C" fn imin(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn iclip(v: libc::c_int, min: libc::c_int, max: libc::c_int) -> libc::c_int {
    return if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    };
}
#[inline]
unsafe extern "C" fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1i64 != 0 {
        unreachable!();
    }
    return x >> 1i32;
}
#[inline]
unsafe extern "C" fn pixel_set(dst: *mut pixel, val: libc::c_int, num: libc::c_int) {
    let mut n: libc::c_int = 0i32;
    while n < num {
        *dst.offset(n as isize) = val as pixel;
        n += 1;
    }
}
#[inline(never)]
unsafe extern "C" fn put_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    mut h: libc::c_int,
) {
    loop {
        memcpy(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            (w << 1i32) as libc::c_ulong,
        );
        dst = dst.offset(dst_stride as isize);
        src = src.offset(src_stride as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    }
}
#[inline(never)]
unsafe extern "C" fn prep_c(
    mut tmp: *mut int16_t,
    mut src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    mut h: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14i32 - (32i32 - clz(bitdepth_max as libc::c_uint));
    loop {
        let mut x: libc::c_int = 0i32;
        while x < w {
            *tmp.offset(x as isize) = (((*src.offset(x as isize) as libc::c_int)
                << intermediate_bits)
                - 8192i32) as int16_t;
            x += 1;
        }
        tmp = tmp.offset(w as isize);
        src = src.offset(src_stride as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    }
}
#[inline(never)]
unsafe extern "C" fn put_8tap_c(
    mut dst: *mut pixel,
    mut dst_stride: ptrdiff_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: libc::c_int,
    mut h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    filter_type: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14i32 - (32i32 - clz(bitdepth_max as libc::c_uint));
    let intermediate_rnd: libc::c_int = 32i32 + ((1i32) << 6i32 - intermediate_bits >> 1i32);
    let fh: *const int8_t = if mx == 0 {
        0 as *const int8_t
    } else if w > 4i32 {
        (dav1d_mc_subpel_filters[(filter_type & 3i32) as usize][(mx - 1i32) as usize]).as_ptr()
    } else {
        (dav1d_mc_subpel_filters[(3i32 + (filter_type & 1i32)) as usize][(mx - 1i32) as usize])
            .as_ptr()
    };
    let fv: *const int8_t = if my == 0 {
        0 as *const int8_t
    } else if h > 4i32 {
        (dav1d_mc_subpel_filters[(filter_type >> 2i32) as usize][(my - 1i32) as usize]).as_ptr()
    } else {
        (dav1d_mc_subpel_filters[(3i32 + (filter_type >> 2i32 & 1i32)) as usize]
            [(my - 1i32) as usize])
            .as_ptr()
    };
    dst_stride = PXSTRIDE(dst_stride);
    src_stride = PXSTRIDE(src_stride);
    if !fh.is_null() {
        if !fv.is_null() {
            let mut tmp_h: libc::c_int = h + 7i32;
            let mut mid: [int16_t; 17280] = [0; 17280];
            let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
            src = src.offset(-((src_stride * 3i64) as isize));
            loop {
                let mut x: libc::c_int = 0i32;
                while x < w {
                    *mid_ptr.offset(x as isize) = (*fh.offset(0isize) as libc::c_int
                        * *src.offset((x + -(3i32) * 1i32) as isize) as libc::c_int
                        + *fh.offset(1isize) as libc::c_int
                            * *src.offset((x + -(2i32) * 1i32) as isize) as libc::c_int
                        + *fh.offset(2isize) as libc::c_int
                            * *src.offset((x + -(1i32) * 1i32) as isize) as libc::c_int
                        + *fh.offset(3isize) as libc::c_int
                            * *src.offset((x + 0i32 * 1i32) as isize) as libc::c_int
                        + *fh.offset(4isize) as libc::c_int
                            * *src.offset((x + 1i32 * 1i32) as isize) as libc::c_int
                        + *fh.offset(5isize) as libc::c_int
                            * *src.offset((x + 2i32 * 1i32) as isize) as libc::c_int
                        + *fh.offset(6isize) as libc::c_int
                            * *src.offset((x + 3i32 * 1i32) as isize) as libc::c_int
                        + *fh.offset(7isize) as libc::c_int
                            * *src.offset((x + 4i32 * 1i32) as isize) as libc::c_int
                        + ((1i32) << 6i32 - intermediate_bits >> 1i32)
                        >> 6i32 - intermediate_bits)
                        as int16_t;
                    x += 1;
                }
                mid_ptr = mid_ptr.offset(128isize);
                src = src.offset(src_stride as isize);
                tmp_h -= 1;
                if !(tmp_h != 0) {
                    break;
                }
            }
            mid_ptr = mid.as_mut_ptr().offset((128i32 * 3i32) as isize);
            loop {
                let mut x_0: libc::c_int = 0i32;
                while x_0 < w {
                    *dst.offset(x_0 as isize) = iclip(
                        *fv.offset(0isize) as libc::c_int
                            * *mid_ptr.offset((x_0 + -(3i32) * 128i32) as isize) as libc::c_int
                            + *fv.offset(1isize) as libc::c_int
                                * *mid_ptr.offset((x_0 + -(2i32) * 128i32) as isize) as libc::c_int
                            + *fv.offset(2isize) as libc::c_int
                                * *mid_ptr.offset((x_0 + -(1i32) * 128i32) as isize) as libc::c_int
                            + *fv.offset(3isize) as libc::c_int
                                * *mid_ptr.offset((x_0 + 0i32 * 128i32) as isize) as libc::c_int
                            + *fv.offset(4isize) as libc::c_int
                                * *mid_ptr.offset((x_0 + 1i32 * 128i32) as isize) as libc::c_int
                            + *fv.offset(5isize) as libc::c_int
                                * *mid_ptr.offset((x_0 + 2i32 * 128i32) as isize) as libc::c_int
                            + *fv.offset(6isize) as libc::c_int
                                * *mid_ptr.offset((x_0 + 3i32 * 128i32) as isize) as libc::c_int
                            + *fv.offset(7isize) as libc::c_int
                                * *mid_ptr.offset((x_0 + 4i32 * 128i32) as isize) as libc::c_int
                            + ((1i32) << 6i32 + intermediate_bits >> 1i32)
                            >> 6i32 + intermediate_bits,
                        0i32,
                        bitdepth_max,
                    ) as pixel;
                    x_0 += 1;
                }
                mid_ptr = mid_ptr.offset(128isize);
                dst = dst.offset(dst_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        } else {
            loop {
                let mut x_1: libc::c_int = 0i32;
                while x_1 < w {
                    *dst.offset(x_1 as isize) = iclip(
                        *fh.offset(0isize) as libc::c_int
                            * *src.offset((x_1 + -(3i32) * 1i32) as isize) as libc::c_int
                            + *fh.offset(1isize) as libc::c_int
                                * *src.offset((x_1 + -(2i32) * 1i32) as isize) as libc::c_int
                            + *fh.offset(2isize) as libc::c_int
                                * *src.offset((x_1 + -(1i32) * 1i32) as isize) as libc::c_int
                            + *fh.offset(3isize) as libc::c_int
                                * *src.offset((x_1 + 0i32 * 1i32) as isize) as libc::c_int
                            + *fh.offset(4isize) as libc::c_int
                                * *src.offset((x_1 + 1i32 * 1i32) as isize) as libc::c_int
                            + *fh.offset(5isize) as libc::c_int
                                * *src.offset((x_1 + 2i32 * 1i32) as isize) as libc::c_int
                            + *fh.offset(6isize) as libc::c_int
                                * *src.offset((x_1 + 3i32 * 1i32) as isize) as libc::c_int
                            + *fh.offset(7isize) as libc::c_int
                                * *src.offset((x_1 + 4i32 * 1i32) as isize) as libc::c_int
                            + intermediate_rnd
                            >> 6i32,
                        0i32,
                        bitdepth_max,
                    ) as pixel;
                    x_1 += 1;
                }
                dst = dst.offset(dst_stride as isize);
                src = src.offset(src_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        }
    } else if !fv.is_null() {
        loop {
            let mut x_2: libc::c_int = 0i32;
            while x_2 < w {
                *dst.offset(x_2 as isize) = iclip(
                    *fv.offset(0isize) as libc::c_int
                        * *src.offset((x_2 as libc::c_long + -3i64 * src_stride) as isize)
                            as libc::c_int
                        + *fv.offset(1isize) as libc::c_int
                            * *src.offset((x_2 as libc::c_long + -2i64 * src_stride) as isize)
                                as libc::c_int
                        + *fv.offset(2isize) as libc::c_int
                            * *src.offset((x_2 as libc::c_long + -1i64 * src_stride) as isize)
                                as libc::c_int
                        + *fv.offset(3isize) as libc::c_int
                            * *src.offset((x_2 as libc::c_long + 0i64 * src_stride) as isize)
                                as libc::c_int
                        + *fv.offset(4isize) as libc::c_int
                            * *src.offset((x_2 as libc::c_long + 1i64 * src_stride) as isize)
                                as libc::c_int
                        + *fv.offset(5isize) as libc::c_int
                            * *src.offset((x_2 as libc::c_long + 2i64 * src_stride) as isize)
                                as libc::c_int
                        + *fv.offset(6isize) as libc::c_int
                            * *src.offset((x_2 as libc::c_long + 3i64 * src_stride) as isize)
                                as libc::c_int
                        + *fv.offset(7isize) as libc::c_int
                            * *src.offset((x_2 as libc::c_long + 4i64 * src_stride) as isize)
                                as libc::c_int
                        + ((1i32) << 6i32 >> 1i32)
                        >> 6i32,
                    0i32,
                    bitdepth_max,
                ) as pixel;
                x_2 += 1;
            }
            dst = dst.offset(dst_stride as isize);
            src = src.offset(src_stride as isize);
            h -= 1;
            if !(h != 0) {
                break;
            }
        }
    } else {
        put_c(dst, dst_stride, src, src_stride, w, h);
    };
}
#[inline(never)]
unsafe extern "C" fn put_8tap_scaled_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: libc::c_int,
    mut h: libc::c_int,
    mx: libc::c_int,
    mut my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    filter_type: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14i32 - (32i32 - clz(bitdepth_max as libc::c_uint));
    let intermediate_rnd: libc::c_int = (1i32) << intermediate_bits >> 1i32;
    let mut tmp_h: libc::c_int = ((h - 1i32) * dy + my >> 10i32) + 8i32;
    let mut mid: [int16_t; 33664] = [0; 33664];
    let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
    src_stride = PXSTRIDE(src_stride);
    src = src.offset(-((src_stride * 3i64) as isize));
    loop {
        let mut x: libc::c_int = 0;
        let mut imx: libc::c_int = mx;
        let mut ioff: libc::c_int = 0i32;
        x = 0i32;
        while x < w {
            let fh: *const int8_t = if imx >> 6i32 == 0 {
                0 as *const int8_t
            } else if w > 4i32 {
                (dav1d_mc_subpel_filters[(filter_type & 3i32) as usize]
                    [((imx >> 6i32) - 1i32) as usize])
                    .as_ptr()
            } else {
                (dav1d_mc_subpel_filters[(3i32 + (filter_type & 1i32)) as usize]
                    [((imx >> 6i32) - 1i32) as usize])
                    .as_ptr()
            };
            *mid_ptr.offset(x as isize) = (if !fh.is_null() {
                *fh.offset(0isize) as libc::c_int
                    * *src.offset((ioff + -(3i32) * 1i32) as isize) as libc::c_int
                    + *fh.offset(1isize) as libc::c_int
                        * *src.offset((ioff + -(2i32) * 1i32) as isize) as libc::c_int
                    + *fh.offset(2isize) as libc::c_int
                        * *src.offset((ioff + -(1i32) * 1i32) as isize) as libc::c_int
                    + *fh.offset(3isize) as libc::c_int
                        * *src.offset((ioff + 0i32 * 1i32) as isize) as libc::c_int
                    + *fh.offset(4isize) as libc::c_int
                        * *src.offset((ioff + 1i32 * 1i32) as isize) as libc::c_int
                    + *fh.offset(5isize) as libc::c_int
                        * *src.offset((ioff + 2i32 * 1i32) as isize) as libc::c_int
                    + *fh.offset(6isize) as libc::c_int
                        * *src.offset((ioff + 3i32 * 1i32) as isize) as libc::c_int
                    + *fh.offset(7isize) as libc::c_int
                        * *src.offset((ioff + 4i32 * 1i32) as isize) as libc::c_int
                    + ((1i32) << 6i32 - intermediate_bits >> 1i32)
                    >> 6i32 - intermediate_bits
            } else {
                (*src.offset(ioff as isize) as libc::c_int) << intermediate_bits
            }) as int16_t;
            imx += dx;
            ioff += imx >> 10i32;
            imx &= 0x3ffi32;
            x += 1;
        }
        mid_ptr = mid_ptr.offset(128isize);
        src = src.offset(src_stride as isize);
        tmp_h -= 1;
        if !(tmp_h != 0) {
            break;
        }
    }
    mid_ptr = mid.as_mut_ptr().offset((128i32 * 3i32) as isize);
    let mut y: libc::c_int = 0i32;
    while y < h {
        let mut x_0: libc::c_int = 0;
        let fv: *const int8_t = if my >> 6i32 == 0 {
            0 as *const int8_t
        } else if h > 4i32 {
            (dav1d_mc_subpel_filters[(filter_type >> 2i32) as usize]
                [((my >> 6i32) - 1i32) as usize])
                .as_ptr()
        } else {
            (dav1d_mc_subpel_filters[(3i32 + (filter_type >> 2i32 & 1i32)) as usize]
                [((my >> 6i32) - 1i32) as usize])
                .as_ptr()
        };
        x_0 = 0i32;
        while x_0 < w {
            *dst.offset(x_0 as isize) = (if !fv.is_null() {
                iclip(
                    *fv.offset(0isize) as libc::c_int
                        * *mid_ptr.offset((x_0 + -(3i32) * 128i32) as isize) as libc::c_int
                        + *fv.offset(1isize) as libc::c_int
                            * *mid_ptr.offset((x_0 + -(2i32) * 128i32) as isize) as libc::c_int
                        + *fv.offset(2isize) as libc::c_int
                            * *mid_ptr.offset((x_0 + -(1i32) * 128i32) as isize) as libc::c_int
                        + *fv.offset(3isize) as libc::c_int
                            * *mid_ptr.offset((x_0 + 0i32 * 128i32) as isize) as libc::c_int
                        + *fv.offset(4isize) as libc::c_int
                            * *mid_ptr.offset((x_0 + 1i32 * 128i32) as isize) as libc::c_int
                        + *fv.offset(5isize) as libc::c_int
                            * *mid_ptr.offset((x_0 + 2i32 * 128i32) as isize) as libc::c_int
                        + *fv.offset(6isize) as libc::c_int
                            * *mid_ptr.offset((x_0 + 3i32 * 128i32) as isize) as libc::c_int
                        + *fv.offset(7isize) as libc::c_int
                            * *mid_ptr.offset((x_0 + 4i32 * 128i32) as isize) as libc::c_int
                        + ((1i32) << 6i32 + intermediate_bits >> 1i32)
                        >> 6i32 + intermediate_bits,
                    0i32,
                    bitdepth_max,
                )
            } else {
                iclip(
                    *mid_ptr.offset(x_0 as isize) as libc::c_int + intermediate_rnd
                        >> intermediate_bits,
                    0i32,
                    bitdepth_max,
                )
            }) as pixel;
            x_0 += 1;
        }
        my += dy;
        mid_ptr = mid_ptr.offset(((my >> 10i32) * 128i32) as isize);
        my &= 0x3ffi32;
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        y += 1;
    }
}
#[inline(never)]
unsafe extern "C" fn prep_8tap_c(
    mut tmp: *mut int16_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: libc::c_int,
    mut h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    filter_type: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14i32 - (32i32 - clz(bitdepth_max as libc::c_uint));
    let fh: *const int8_t = if mx == 0 {
        0 as *const int8_t
    } else if w > 4i32 {
        (dav1d_mc_subpel_filters[(filter_type & 3i32) as usize][(mx - 1i32) as usize]).as_ptr()
    } else {
        (dav1d_mc_subpel_filters[(3i32 + (filter_type & 1i32)) as usize][(mx - 1i32) as usize])
            .as_ptr()
    };
    let fv: *const int8_t = if my == 0 {
        0 as *const int8_t
    } else if h > 4i32 {
        (dav1d_mc_subpel_filters[(filter_type >> 2i32) as usize][(my - 1i32) as usize]).as_ptr()
    } else {
        (dav1d_mc_subpel_filters[(3i32 + (filter_type >> 2i32 & 1i32)) as usize]
            [(my - 1i32) as usize])
            .as_ptr()
    };
    src_stride = PXSTRIDE(src_stride);
    if !fh.is_null() {
        if !fv.is_null() {
            let mut tmp_h: libc::c_int = h + 7i32;
            let mut mid: [int16_t; 17280] = [0; 17280];
            let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
            src = src.offset(-((src_stride * 3i64) as isize));
            loop {
                let mut x: libc::c_int = 0i32;
                while x < w {
                    *mid_ptr.offset(x as isize) = (*fh.offset(0isize) as libc::c_int
                        * *src.offset((x + -(3i32) * 1i32) as isize) as libc::c_int
                        + *fh.offset(1isize) as libc::c_int
                            * *src.offset((x + -(2i32) * 1i32) as isize) as libc::c_int
                        + *fh.offset(2isize) as libc::c_int
                            * *src.offset((x + -(1i32) * 1i32) as isize) as libc::c_int
                        + *fh.offset(3isize) as libc::c_int
                            * *src.offset((x + 0i32 * 1i32) as isize) as libc::c_int
                        + *fh.offset(4isize) as libc::c_int
                            * *src.offset((x + 1i32 * 1i32) as isize) as libc::c_int
                        + *fh.offset(5isize) as libc::c_int
                            * *src.offset((x + 2i32 * 1i32) as isize) as libc::c_int
                        + *fh.offset(6isize) as libc::c_int
                            * *src.offset((x + 3i32 * 1i32) as isize) as libc::c_int
                        + *fh.offset(7isize) as libc::c_int
                            * *src.offset((x + 4i32 * 1i32) as isize) as libc::c_int
                        + ((1i32) << 6i32 - intermediate_bits >> 1i32)
                        >> 6i32 - intermediate_bits)
                        as int16_t;
                    x += 1;
                }
                mid_ptr = mid_ptr.offset(128isize);
                src = src.offset(src_stride as isize);
                tmp_h -= 1;
                if !(tmp_h != 0) {
                    break;
                }
            }
            mid_ptr = mid.as_mut_ptr().offset((128i32 * 3i32) as isize);
            loop {
                let mut x_0: libc::c_int = 0i32;
                while x_0 < w {
                    let mut t: libc::c_int = (*fv.offset(0isize) as libc::c_int
                        * *mid_ptr.offset((x_0 + -(3i32) * 128i32) as isize) as libc::c_int
                        + *fv.offset(1isize) as libc::c_int
                            * *mid_ptr.offset((x_0 + -(2i32) * 128i32) as isize) as libc::c_int
                        + *fv.offset(2isize) as libc::c_int
                            * *mid_ptr.offset((x_0 + -(1i32) * 128i32) as isize) as libc::c_int
                        + *fv.offset(3isize) as libc::c_int
                            * *mid_ptr.offset((x_0 + 0i32 * 128i32) as isize) as libc::c_int
                        + *fv.offset(4isize) as libc::c_int
                            * *mid_ptr.offset((x_0 + 1i32 * 128i32) as isize) as libc::c_int
                        + *fv.offset(5isize) as libc::c_int
                            * *mid_ptr.offset((x_0 + 2i32 * 128i32) as isize) as libc::c_int
                        + *fv.offset(6isize) as libc::c_int
                            * *mid_ptr.offset((x_0 + 3i32 * 128i32) as isize) as libc::c_int
                        + *fv.offset(7isize) as libc::c_int
                            * *mid_ptr.offset((x_0 + 4i32 * 128i32) as isize) as libc::c_int
                        + ((1i32) << 6i32 >> 1i32)
                        >> 6i32)
                        - 8192i32;
                    if !(t >= -(32767i32) - 1i32 && t <= 32767i32) {
                        unreachable!();
                    }
                    *tmp.offset(x_0 as isize) = t as int16_t;
                    x_0 += 1;
                }
                mid_ptr = mid_ptr.offset(128isize);
                tmp = tmp.offset(w as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        } else {
            loop {
                let mut x_1: libc::c_int = 0i32;
                while x_1 < w {
                    *tmp.offset(x_1 as isize) = ((*fh.offset(0isize) as libc::c_int
                        * *src.offset((x_1 + -(3i32) * 1i32) as isize) as libc::c_int
                        + *fh.offset(1isize) as libc::c_int
                            * *src.offset((x_1 + -(2i32) * 1i32) as isize) as libc::c_int
                        + *fh.offset(2isize) as libc::c_int
                            * *src.offset((x_1 + -(1i32) * 1i32) as isize) as libc::c_int
                        + *fh.offset(3isize) as libc::c_int
                            * *src.offset((x_1 + 0i32 * 1i32) as isize) as libc::c_int
                        + *fh.offset(4isize) as libc::c_int
                            * *src.offset((x_1 + 1i32 * 1i32) as isize) as libc::c_int
                        + *fh.offset(5isize) as libc::c_int
                            * *src.offset((x_1 + 2i32 * 1i32) as isize) as libc::c_int
                        + *fh.offset(6isize) as libc::c_int
                            * *src.offset((x_1 + 3i32 * 1i32) as isize) as libc::c_int
                        + *fh.offset(7isize) as libc::c_int
                            * *src.offset((x_1 + 4i32 * 1i32) as isize) as libc::c_int
                        + ((1i32) << 6i32 - intermediate_bits >> 1i32)
                        >> 6i32 - intermediate_bits)
                        - 8192i32) as int16_t;
                    x_1 += 1;
                }
                tmp = tmp.offset(w as isize);
                src = src.offset(src_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        }
    } else if !fv.is_null() {
        loop {
            let mut x_2: libc::c_int = 0i32;
            while x_2 < w {
                *tmp.offset(x_2 as isize) = ((*fv.offset(0isize) as libc::c_int
                    * *src.offset((x_2 as libc::c_long + -3i64 * src_stride) as isize)
                        as libc::c_int
                    + *fv.offset(1isize) as libc::c_int
                        * *src.offset((x_2 as libc::c_long + -2i64 * src_stride) as isize)
                            as libc::c_int
                    + *fv.offset(2isize) as libc::c_int
                        * *src.offset((x_2 as libc::c_long + -1i64 * src_stride) as isize)
                            as libc::c_int
                    + *fv.offset(3isize) as libc::c_int
                        * *src.offset((x_2 as libc::c_long + 0i64 * src_stride) as isize)
                            as libc::c_int
                    + *fv.offset(4isize) as libc::c_int
                        * *src.offset((x_2 as libc::c_long + 1i64 * src_stride) as isize)
                            as libc::c_int
                    + *fv.offset(5isize) as libc::c_int
                        * *src.offset((x_2 as libc::c_long + 2i64 * src_stride) as isize)
                            as libc::c_int
                    + *fv.offset(6isize) as libc::c_int
                        * *src.offset((x_2 as libc::c_long + 3i64 * src_stride) as isize)
                            as libc::c_int
                    + *fv.offset(7isize) as libc::c_int
                        * *src.offset((x_2 as libc::c_long + 4i64 * src_stride) as isize)
                            as libc::c_int
                    + ((1i32) << 6i32 - intermediate_bits >> 1i32)
                    >> 6i32 - intermediate_bits)
                    - 8192i32) as int16_t;
                x_2 += 1;
            }
            tmp = tmp.offset(w as isize);
            src = src.offset(src_stride as isize);
            h -= 1;
            if !(h != 0) {
                break;
            }
        }
    } else {
        prep_c(tmp, src, src_stride, w, h, bitdepth_max);
    };
}
#[inline(never)]
unsafe extern "C" fn prep_8tap_scaled_c(
    mut tmp: *mut int16_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: libc::c_int,
    mut h: libc::c_int,
    mx: libc::c_int,
    mut my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    filter_type: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14i32 - (32i32 - clz(bitdepth_max as libc::c_uint));
    let mut tmp_h: libc::c_int = ((h - 1i32) * dy + my >> 10i32) + 8i32;
    let mut mid: [int16_t; 33664] = [0; 33664];
    let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
    src_stride = PXSTRIDE(src_stride);
    src = src.offset(-((src_stride * 3i64) as isize));
    loop {
        let mut x: libc::c_int = 0;
        let mut imx: libc::c_int = mx;
        let mut ioff: libc::c_int = 0i32;
        x = 0i32;
        while x < w {
            let fh: *const int8_t = if imx >> 6i32 == 0 {
                0 as *const int8_t
            } else if w > 4i32 {
                (dav1d_mc_subpel_filters[(filter_type & 3i32) as usize]
                    [((imx >> 6i32) - 1i32) as usize])
                    .as_ptr()
            } else {
                (dav1d_mc_subpel_filters[(3i32 + (filter_type & 1i32)) as usize]
                    [((imx >> 6i32) - 1i32) as usize])
                    .as_ptr()
            };
            *mid_ptr.offset(x as isize) = (if !fh.is_null() {
                *fh.offset(0isize) as libc::c_int
                    * *src.offset((ioff + -(3i32) * 1i32) as isize) as libc::c_int
                    + *fh.offset(1isize) as libc::c_int
                        * *src.offset((ioff + -(2i32) * 1i32) as isize) as libc::c_int
                    + *fh.offset(2isize) as libc::c_int
                        * *src.offset((ioff + -(1i32) * 1i32) as isize) as libc::c_int
                    + *fh.offset(3isize) as libc::c_int
                        * *src.offset((ioff + 0i32 * 1i32) as isize) as libc::c_int
                    + *fh.offset(4isize) as libc::c_int
                        * *src.offset((ioff + 1i32 * 1i32) as isize) as libc::c_int
                    + *fh.offset(5isize) as libc::c_int
                        * *src.offset((ioff + 2i32 * 1i32) as isize) as libc::c_int
                    + *fh.offset(6isize) as libc::c_int
                        * *src.offset((ioff + 3i32 * 1i32) as isize) as libc::c_int
                    + *fh.offset(7isize) as libc::c_int
                        * *src.offset((ioff + 4i32 * 1i32) as isize) as libc::c_int
                    + ((1i32) << 6i32 - intermediate_bits >> 1i32)
                    >> 6i32 - intermediate_bits
            } else {
                (*src.offset(ioff as isize) as libc::c_int) << intermediate_bits
            }) as int16_t;
            imx += dx;
            ioff += imx >> 10i32;
            imx &= 0x3ffi32;
            x += 1;
        }
        mid_ptr = mid_ptr.offset(128isize);
        src = src.offset(src_stride as isize);
        tmp_h -= 1;
        if !(tmp_h != 0) {
            break;
        }
    }
    mid_ptr = mid.as_mut_ptr().offset((128i32 * 3i32) as isize);
    let mut y: libc::c_int = 0i32;
    while y < h {
        let mut x_0: libc::c_int = 0;
        let fv: *const int8_t = if my >> 6i32 == 0 {
            0 as *const int8_t
        } else if h > 4i32 {
            (dav1d_mc_subpel_filters[(filter_type >> 2i32) as usize]
                [((my >> 6i32) - 1i32) as usize])
                .as_ptr()
        } else {
            (dav1d_mc_subpel_filters[(3i32 + (filter_type >> 2i32 & 1i32)) as usize]
                [((my >> 6i32) - 1i32) as usize])
                .as_ptr()
        };
        x_0 = 0i32;
        while x_0 < w {
            *tmp.offset(x_0 as isize) = ((if !fv.is_null() {
                *fv.offset(0isize) as libc::c_int
                    * *mid_ptr.offset((x_0 + -(3i32) * 128i32) as isize) as libc::c_int
                    + *fv.offset(1isize) as libc::c_int
                        * *mid_ptr.offset((x_0 + -(2i32) * 128i32) as isize) as libc::c_int
                    + *fv.offset(2isize) as libc::c_int
                        * *mid_ptr.offset((x_0 + -(1i32) * 128i32) as isize) as libc::c_int
                    + *fv.offset(3isize) as libc::c_int
                        * *mid_ptr.offset((x_0 + 0i32 * 128i32) as isize) as libc::c_int
                    + *fv.offset(4isize) as libc::c_int
                        * *mid_ptr.offset((x_0 + 1i32 * 128i32) as isize) as libc::c_int
                    + *fv.offset(5isize) as libc::c_int
                        * *mid_ptr.offset((x_0 + 2i32 * 128i32) as isize) as libc::c_int
                    + *fv.offset(6isize) as libc::c_int
                        * *mid_ptr.offset((x_0 + 3i32 * 128i32) as isize) as libc::c_int
                    + *fv.offset(7isize) as libc::c_int
                        * *mid_ptr.offset((x_0 + 4i32 * 128i32) as isize) as libc::c_int
                    + ((1i32) << 6i32 >> 1i32)
                    >> 6i32
            } else {
                *mid_ptr.offset(x_0 as isize) as libc::c_int
            }) - 8192i32) as int16_t;
            x_0 += 1;
        }
        my += dy;
        mid_ptr = mid_ptr.offset(((my >> 10i32) * 128i32) as isize);
        my &= 0x3ffi32;
        tmp = tmp.offset(w as isize);
        y += 1;
    }
}
unsafe extern "C" fn put_8tap_regular_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_REGULAR as libc::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_regular_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_REGULAR as libc::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_regular_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_REGULAR as libc::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_regular_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_REGULAR as libc::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_regular_sharp_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_REGULAR as libc::c_int | (DAV1D_FILTER_8TAP_SHARP as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_regular_sharp_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_REGULAR as libc::c_int | (DAV1D_FILTER_8TAP_SHARP as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_regular_sharp_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_REGULAR as libc::c_int | (DAV1D_FILTER_8TAP_SHARP as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_regular_sharp_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_REGULAR as libc::c_int | (DAV1D_FILTER_8TAP_SHARP as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_regular_smooth_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_REGULAR as libc::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_regular_smooth_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_REGULAR as libc::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_regular_smooth_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_REGULAR as libc::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_regular_smooth_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_REGULAR as libc::c_int
            | (DAV1D_FILTER_8TAP_SMOOTH as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_smooth_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SMOOTH as libc::c_int | (DAV1D_FILTER_8TAP_SMOOTH as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_smooth_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SMOOTH as libc::c_int | (DAV1D_FILTER_8TAP_SMOOTH as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_smooth_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SMOOTH as libc::c_int | (DAV1D_FILTER_8TAP_SMOOTH as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_smooth_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SMOOTH as libc::c_int | (DAV1D_FILTER_8TAP_SMOOTH as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_smooth_regular_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SMOOTH as libc::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_smooth_regular_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SMOOTH as libc::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_smooth_regular_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SMOOTH as libc::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_smooth_regular_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SMOOTH as libc::c_int
            | (DAV1D_FILTER_8TAP_REGULAR as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_smooth_sharp_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SMOOTH as libc::c_int | (DAV1D_FILTER_8TAP_SHARP as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_smooth_sharp_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SMOOTH as libc::c_int | (DAV1D_FILTER_8TAP_SHARP as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_smooth_sharp_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SMOOTH as libc::c_int | (DAV1D_FILTER_8TAP_SHARP as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_smooth_sharp_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SMOOTH as libc::c_int | (DAV1D_FILTER_8TAP_SHARP as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_sharp_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SHARP as libc::c_int | (DAV1D_FILTER_8TAP_SHARP as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_sharp_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SHARP as libc::c_int | (DAV1D_FILTER_8TAP_SHARP as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_sharp_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SHARP as libc::c_int | (DAV1D_FILTER_8TAP_SHARP as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_sharp_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SHARP as libc::c_int | (DAV1D_FILTER_8TAP_SHARP as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_sharp_regular_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SHARP as libc::c_int | (DAV1D_FILTER_8TAP_REGULAR as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_sharp_regular_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SHARP as libc::c_int | (DAV1D_FILTER_8TAP_REGULAR as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_sharp_regular_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SHARP as libc::c_int | (DAV1D_FILTER_8TAP_REGULAR as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_sharp_regular_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SHARP as libc::c_int | (DAV1D_FILTER_8TAP_REGULAR as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_sharp_smooth_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SHARP as libc::c_int | (DAV1D_FILTER_8TAP_SMOOTH as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_sharp_smooth_scaled_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_scaled_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SHARP as libc::c_int | (DAV1D_FILTER_8TAP_SMOOTH as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_8tap_sharp_smooth_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    put_8tap_c(
        dst,
        dst_stride,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        DAV1D_FILTER_8TAP_SHARP as libc::c_int | (DAV1D_FILTER_8TAP_SMOOTH as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn prep_8tap_sharp_smooth_scaled_c(
    tmp: *mut int16_t,
    src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    prep_8tap_scaled_c(
        tmp,
        src,
        src_stride,
        w,
        h,
        mx,
        my,
        dx,
        dy,
        DAV1D_FILTER_8TAP_SHARP as libc::c_int | (DAV1D_FILTER_8TAP_SMOOTH as libc::c_int) << 2i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn put_bilin_c(
    mut dst: *mut pixel,
    mut dst_stride: ptrdiff_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: libc::c_int,
    mut h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14i32 - (32i32 - clz(bitdepth_max as libc::c_uint));
    let intermediate_rnd: libc::c_int = (1i32) << intermediate_bits >> 1i32;
    dst_stride = PXSTRIDE(dst_stride);
    src_stride = PXSTRIDE(src_stride);
    if mx != 0 {
        if my != 0 {
            let mut mid: [int16_t; 16512] = [0; 16512];
            let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
            let mut tmp_h: libc::c_int = h + 1i32;
            loop {
                let mut x: libc::c_int = 0i32;
                while x < w {
                    *mid_ptr.offset(x as isize) = (16i32 * *src.offset(x as isize) as libc::c_int
                        + mx * (*src.offset((x + 1i32) as isize) as libc::c_int
                            - *src.offset(x as isize) as libc::c_int)
                        + ((1i32) << 4i32 - intermediate_bits >> 1i32)
                        >> 4i32 - intermediate_bits)
                        as int16_t;
                    x += 1;
                }
                mid_ptr = mid_ptr.offset(128isize);
                src = src.offset(src_stride as isize);
                tmp_h -= 1;
                if !(tmp_h != 0) {
                    break;
                }
            }
            mid_ptr = mid.as_mut_ptr();
            loop {
                let mut x_0: libc::c_int = 0i32;
                while x_0 < w {
                    *dst.offset(x_0 as isize) = iclip(
                        16i32 * *mid_ptr.offset(x_0 as isize) as libc::c_int
                            + my * (*mid_ptr.offset((x_0 + 128i32) as isize) as libc::c_int
                                - *mid_ptr.offset(x_0 as isize) as libc::c_int)
                            + ((1i32) << 4i32 + intermediate_bits >> 1i32)
                            >> 4i32 + intermediate_bits,
                        0i32,
                        bitdepth_max,
                    ) as pixel;
                    x_0 += 1;
                }
                mid_ptr = mid_ptr.offset(128isize);
                dst = dst.offset(dst_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        } else {
            loop {
                let mut x_1: libc::c_int = 0i32;
                while x_1 < w {
                    let px: libc::c_int = 16i32 * *src.offset(x_1 as isize) as libc::c_int
                        + mx * (*src.offset((x_1 + 1i32) as isize) as libc::c_int
                            - *src.offset(x_1 as isize) as libc::c_int)
                        + ((1i32) << 4i32 - intermediate_bits >> 1i32)
                        >> 4i32 - intermediate_bits;
                    *dst.offset(x_1 as isize) = iclip(
                        px + intermediate_rnd >> intermediate_bits,
                        0i32,
                        bitdepth_max,
                    ) as pixel;
                    x_1 += 1;
                }
                dst = dst.offset(dst_stride as isize);
                src = src.offset(src_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        }
    } else if my != 0 {
        loop {
            let mut x_2: libc::c_int = 0i32;
            while x_2 < w {
                *dst.offset(x_2 as isize) = iclip(
                    16i32 * *src.offset(x_2 as isize) as libc::c_int
                        + my * (*src.offset((x_2 as libc::c_long + src_stride) as isize)
                            as libc::c_int
                            - *src.offset(x_2 as isize) as libc::c_int)
                        + ((1i32) << 4i32 >> 1i32)
                        >> 4i32,
                    0i32,
                    bitdepth_max,
                ) as pixel;
                x_2 += 1;
            }
            dst = dst.offset(dst_stride as isize);
            src = src.offset(src_stride as isize);
            h -= 1;
            if !(h != 0) {
                break;
            }
        }
    } else {
        put_c(dst, dst_stride, src, src_stride, w, h);
    };
}
unsafe extern "C" fn put_bilin_scaled_c(
    mut dst: *mut pixel,
    mut dst_stride: ptrdiff_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: libc::c_int,
    mut h: libc::c_int,
    mx: libc::c_int,
    mut my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14i32 - (32i32 - clz(bitdepth_max as libc::c_uint));
    let mut tmp_h: libc::c_int = ((h - 1i32) * dy + my >> 10i32) + 2i32;
    let mut mid: [int16_t; 32896] = [0; 32896];
    let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
    loop {
        let mut x: libc::c_int = 0;
        let mut imx: libc::c_int = mx;
        let mut ioff: libc::c_int = 0i32;
        x = 0i32;
        while x < w {
            *mid_ptr.offset(x as isize) = (16i32 * *src.offset(ioff as isize) as libc::c_int
                + (imx >> 6i32)
                    * (*src.offset((ioff + 1i32) as isize) as libc::c_int
                        - *src.offset(ioff as isize) as libc::c_int)
                + ((1i32) << 4i32 - intermediate_bits >> 1i32)
                >> 4i32 - intermediate_bits) as int16_t;
            imx += dx;
            ioff += imx >> 10i32;
            imx &= 0x3ffi32;
            x += 1;
        }
        mid_ptr = mid_ptr.offset(128isize);
        src = src.offset(PXSTRIDE(src_stride) as isize);
        tmp_h -= 1;
        if !(tmp_h != 0) {
            break;
        }
    }
    mid_ptr = mid.as_mut_ptr();
    loop {
        let mut x_0: libc::c_int = 0;
        x_0 = 0i32;
        while x_0 < w {
            *dst.offset(x_0 as isize) = iclip(
                16i32 * *mid_ptr.offset(x_0 as isize) as libc::c_int
                    + (my >> 6i32)
                        * (*mid_ptr.offset((x_0 + 128i32) as isize) as libc::c_int
                            - *mid_ptr.offset(x_0 as isize) as libc::c_int)
                    + ((1i32) << 4i32 + intermediate_bits >> 1i32)
                    >> 4i32 + intermediate_bits,
                0i32,
                bitdepth_max,
            ) as pixel;
            x_0 += 1;
        }
        my += dy;
        mid_ptr = mid_ptr.offset(((my >> 10i32) * 128i32) as isize);
        my &= 0x3ffi32;
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    }
}
unsafe extern "C" fn prep_bilin_c(
    mut tmp: *mut int16_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: libc::c_int,
    mut h: libc::c_int,
    mx: libc::c_int,
    my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14i32 - (32i32 - clz(bitdepth_max as libc::c_uint));
    src_stride = PXSTRIDE(src_stride);
    if mx != 0 {
        if my != 0 {
            let mut mid: [int16_t; 16512] = [0; 16512];
            let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
            let mut tmp_h: libc::c_int = h + 1i32;
            loop {
                let mut x: libc::c_int = 0i32;
                while x < w {
                    *mid_ptr.offset(x as isize) = (16i32 * *src.offset(x as isize) as libc::c_int
                        + mx * (*src.offset((x + 1i32) as isize) as libc::c_int
                            - *src.offset(x as isize) as libc::c_int)
                        + ((1i32) << 4i32 - intermediate_bits >> 1i32)
                        >> 4i32 - intermediate_bits)
                        as int16_t;
                    x += 1;
                }
                mid_ptr = mid_ptr.offset(128isize);
                src = src.offset(src_stride as isize);
                tmp_h -= 1;
                if !(tmp_h != 0) {
                    break;
                }
            }
            mid_ptr = mid.as_mut_ptr();
            loop {
                let mut x_0: libc::c_int = 0i32;
                while x_0 < w {
                    *tmp.offset(x_0 as isize) = ((16i32
                        * *mid_ptr.offset(x_0 as isize) as libc::c_int
                        + my * (*mid_ptr.offset((x_0 + 128i32) as isize) as libc::c_int
                            - *mid_ptr.offset(x_0 as isize) as libc::c_int)
                        + ((1i32) << 4i32 >> 1i32)
                        >> 4i32)
                        - 8192i32) as int16_t;
                    x_0 += 1;
                }
                mid_ptr = mid_ptr.offset(128isize);
                tmp = tmp.offset(w as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        } else {
            loop {
                let mut x_1: libc::c_int = 0i32;
                while x_1 < w {
                    *tmp.offset(x_1 as isize) = ((16i32 * *src.offset(x_1 as isize) as libc::c_int
                        + mx * (*src.offset((x_1 + 1i32) as isize) as libc::c_int
                            - *src.offset(x_1 as isize) as libc::c_int)
                        + ((1i32) << 4i32 - intermediate_bits >> 1i32)
                        >> 4i32 - intermediate_bits)
                        - 8192i32) as int16_t;
                    x_1 += 1;
                }
                tmp = tmp.offset(w as isize);
                src = src.offset(src_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        }
    } else if my != 0 {
        loop {
            let mut x_2: libc::c_int = 0i32;
            while x_2 < w {
                *tmp.offset(x_2 as isize) = ((16i32 * *src.offset(x_2 as isize) as libc::c_int
                    + my * (*src.offset((x_2 as libc::c_long + src_stride) as isize)
                        as libc::c_int
                        - *src.offset(x_2 as isize) as libc::c_int)
                    + ((1i32) << 4i32 - intermediate_bits >> 1i32)
                    >> 4i32 - intermediate_bits)
                    - 8192i32) as int16_t;
                x_2 += 1;
            }
            tmp = tmp.offset(w as isize);
            src = src.offset(src_stride as isize);
            h -= 1;
            if !(h != 0) {
                break;
            }
        }
    } else {
        prep_c(tmp, src, src_stride, w, h, bitdepth_max);
    };
}
unsafe extern "C" fn prep_bilin_scaled_c(
    mut tmp: *mut int16_t,
    mut src: *const pixel,
    mut src_stride: ptrdiff_t,
    w: libc::c_int,
    mut h: libc::c_int,
    mx: libc::c_int,
    mut my: libc::c_int,
    dx: libc::c_int,
    dy: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14i32 - (32i32 - clz(bitdepth_max as libc::c_uint));
    let mut tmp_h: libc::c_int = ((h - 1i32) * dy + my >> 10i32) + 2i32;
    let mut mid: [int16_t; 32896] = [0; 32896];
    let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
    loop {
        let mut x: libc::c_int = 0;
        let mut imx: libc::c_int = mx;
        let mut ioff: libc::c_int = 0i32;
        x = 0i32;
        while x < w {
            *mid_ptr.offset(x as isize) = (16i32 * *src.offset(ioff as isize) as libc::c_int
                + (imx >> 6i32)
                    * (*src.offset((ioff + 1i32) as isize) as libc::c_int
                        - *src.offset(ioff as isize) as libc::c_int)
                + ((1i32) << 4i32 - intermediate_bits >> 1i32)
                >> 4i32 - intermediate_bits) as int16_t;
            imx += dx;
            ioff += imx >> 10i32;
            imx &= 0x3ffi32;
            x += 1;
        }
        mid_ptr = mid_ptr.offset(128isize);
        src = src.offset(PXSTRIDE(src_stride) as isize);
        tmp_h -= 1;
        if !(tmp_h != 0) {
            break;
        }
    }
    mid_ptr = mid.as_mut_ptr();
    loop {
        let mut x_0: libc::c_int = 0;
        x_0 = 0i32;
        while x_0 < w {
            *tmp.offset(x_0 as isize) = ((16i32 * *mid_ptr.offset(x_0 as isize) as libc::c_int
                + (my >> 6i32)
                    * (*mid_ptr.offset((x_0 + 128i32) as isize) as libc::c_int
                        - *mid_ptr.offset(x_0 as isize) as libc::c_int)
                + ((1i32) << 4i32 >> 1i32)
                >> 4i32)
                - 8192i32) as int16_t;
            x_0 += 1;
        }
        my += dy;
        mid_ptr = mid_ptr.offset(((my >> 10i32) * 128i32) as isize);
        my &= 0x3ffi32;
        tmp = tmp.offset(w as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    }
}
unsafe extern "C" fn avg_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut tmp1: *const int16_t,
    mut tmp2: *const int16_t,
    w: libc::c_int,
    mut h: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14i32 - (32i32 - clz(bitdepth_max as libc::c_uint));
    let sh: libc::c_int = intermediate_bits + 1i32;
    let rnd: libc::c_int = ((1i32) << intermediate_bits) + 8192i32 * 2i32;
    loop {
        let mut x: libc::c_int = 0i32;
        while x < w {
            *dst.offset(x as isize) = iclip(
                *tmp1.offset(x as isize) as libc::c_int
                    + *tmp2.offset(x as isize) as libc::c_int
                    + rnd
                    >> sh,
                0i32,
                bitdepth_max,
            ) as pixel;
            x += 1;
        }
        tmp1 = tmp1.offset(w as isize);
        tmp2 = tmp2.offset(w as isize);
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    }
}
unsafe extern "C" fn w_avg_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut tmp1: *const int16_t,
    mut tmp2: *const int16_t,
    w: libc::c_int,
    mut h: libc::c_int,
    weight: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14i32 - (32i32 - clz(bitdepth_max as libc::c_uint));
    let sh: libc::c_int = intermediate_bits + 4i32;
    let rnd: libc::c_int = ((8i32) << intermediate_bits) + 8192i32 * 16i32;
    loop {
        let mut x: libc::c_int = 0i32;
        while x < w {
            *dst.offset(x as isize) = iclip(
                *tmp1.offset(x as isize) as libc::c_int * weight
                    + *tmp2.offset(x as isize) as libc::c_int * (16i32 - weight)
                    + rnd
                    >> sh,
                0i32,
                bitdepth_max,
            ) as pixel;
            x += 1;
        }
        tmp1 = tmp1.offset(w as isize);
        tmp2 = tmp2.offset(w as isize);
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    }
}
unsafe extern "C" fn mask_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut tmp1: *const int16_t,
    mut tmp2: *const int16_t,
    w: libc::c_int,
    mut h: libc::c_int,
    mut mask: *const uint8_t,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14i32 - (32i32 - clz(bitdepth_max as libc::c_uint));
    let sh: libc::c_int = intermediate_bits + 6i32;
    let rnd: libc::c_int = ((32i32) << intermediate_bits) + 8192i32 * 64i32;
    loop {
        let mut x: libc::c_int = 0i32;
        while x < w {
            *dst.offset(x as isize) = iclip(
                *tmp1.offset(x as isize) as libc::c_int * *mask.offset(x as isize) as libc::c_int
                    + *tmp2.offset(x as isize) as libc::c_int
                        * (64i32 - *mask.offset(x as isize) as libc::c_int)
                    + rnd
                    >> sh,
                0i32,
                bitdepth_max,
            ) as pixel;
            x += 1;
        }
        tmp1 = tmp1.offset(w as isize);
        tmp2 = tmp2.offset(w as isize);
        mask = mask.offset(w as isize);
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    }
}
unsafe extern "C" fn blend_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut tmp: *const pixel,
    w: libc::c_int,
    mut h: libc::c_int,
    mut mask: *const uint8_t,
) {
    loop {
        let mut x: libc::c_int = 0i32;
        while x < w {
            *dst.offset(x as isize) = (*dst.offset(x as isize) as libc::c_int
                * (64i32 - *mask.offset(x as isize) as libc::c_int)
                + *tmp.offset(x as isize) as libc::c_int * *mask.offset(x as isize) as libc::c_int
                + 32i32
                >> 6i32) as pixel;
            x += 1;
        }
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        tmp = tmp.offset(w as isize);
        mask = mask.offset(w as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    }
}
unsafe extern "C" fn blend_v_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut tmp: *const pixel,
    w: libc::c_int,
    mut h: libc::c_int,
) {
    let mask: *const uint8_t = &*dav1d_obmc_masks.as_ptr().offset(w as isize) as *const uint8_t;
    loop {
        let mut x: libc::c_int = 0i32;
        while x < w * 3i32 >> 2i32 {
            *dst.offset(x as isize) = (*dst.offset(x as isize) as libc::c_int
                * (64i32 - *mask.offset(x as isize) as libc::c_int)
                + *tmp.offset(x as isize) as libc::c_int * *mask.offset(x as isize) as libc::c_int
                + 32i32
                >> 6i32) as pixel;
            x += 1;
        }
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        tmp = tmp.offset(w as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    }
}
unsafe extern "C" fn blend_h_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut tmp: *const pixel,
    w: libc::c_int,
    mut h: libc::c_int,
) {
    let mut mask: *const uint8_t = &*dav1d_obmc_masks.as_ptr().offset(h as isize) as *const uint8_t;
    h = h * 3i32 >> 2i32;
    loop {
        let fresh0 = mask;
        mask = mask.offset(1);
        let m: libc::c_int = *fresh0 as libc::c_int;
        let mut x: libc::c_int = 0i32;
        while x < w {
            *dst.offset(x as isize) = (*dst.offset(x as isize) as libc::c_int * (64i32 - m)
                + *tmp.offset(x as isize) as libc::c_int * m
                + 32i32
                >> 6i32) as pixel;
            x += 1;
        }
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        tmp = tmp.offset(w as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    }
}
unsafe extern "C" fn w_mask_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut tmp1: *const int16_t,
    mut tmp2: *const int16_t,
    w: libc::c_int,
    mut h: libc::c_int,
    mut mask: *mut uint8_t,
    sign: libc::c_int,
    ss_hor: libc::c_int,
    ss_ver: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14i32 - (32i32 - clz(bitdepth_max as libc::c_uint));
    let bitdepth: libc::c_int = 32i32 - clz(bitdepth_max as libc::c_uint);
    let sh: libc::c_int = intermediate_bits + 6i32;
    let rnd: libc::c_int = ((32i32) << intermediate_bits) + 8192i32 * 64i32;
    let mask_sh: libc::c_int = bitdepth + intermediate_bits - 4i32;
    let mask_rnd: libc::c_int = (1i32) << mask_sh - 5i32;
    loop {
        let mut x: libc::c_int = 0i32;
        while x < w {
            let m: libc::c_int = imin(
                38i32
                    + (abs(*tmp1.offset(x as isize) as libc::c_int
                        - *tmp2.offset(x as isize) as libc::c_int)
                        + mask_rnd
                        >> mask_sh),
                64i32,
            );
            *dst.offset(x as isize) = iclip(
                *tmp1.offset(x as isize) as libc::c_int * m
                    + *tmp2.offset(x as isize) as libc::c_int * (64i32 - m)
                    + rnd
                    >> sh,
                0i32,
                bitdepth_max,
            ) as pixel;
            if ss_hor != 0 {
                x += 1;
                let n: libc::c_int = imin(
                    38i32
                        + (abs(*tmp1.offset(x as isize) as libc::c_int
                            - *tmp2.offset(x as isize) as libc::c_int)
                            + mask_rnd
                            >> mask_sh),
                    64i32,
                );
                *dst.offset(x as isize) = iclip(
                    *tmp1.offset(x as isize) as libc::c_int * n
                        + *tmp2.offset(x as isize) as libc::c_int * (64i32 - n)
                        + rnd
                        >> sh,
                    0i32,
                    bitdepth_max,
                ) as pixel;
                if h & ss_ver != 0 {
                    *mask.offset((x >> 1i32) as isize) =
                        (m + n + *mask.offset((x >> 1i32) as isize) as libc::c_int + 2i32 - sign
                            >> 2i32) as uint8_t;
                } else if ss_ver != 0 {
                    *mask.offset((x >> 1i32) as isize) = (m + n) as uint8_t;
                } else {
                    *mask.offset((x >> 1i32) as isize) = (m + n + 1i32 - sign >> 1i32) as uint8_t;
                }
            } else {
                *mask.offset(x as isize) = m as uint8_t;
            }
            x += 1;
        }
        tmp1 = tmp1.offset(w as isize);
        tmp2 = tmp2.offset(w as isize);
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        if ss_ver == 0 || h & 1i32 != 0 {
            mask = mask.offset((w >> ss_hor) as isize);
        }
        h -= 1;
        if !(h != 0) {
            break;
        }
    }
}
unsafe extern "C" fn w_mask_444_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    tmp1: *const int16_t,
    tmp2: *const int16_t,
    w: libc::c_int,
    h: libc::c_int,
    mut mask: *mut uint8_t,
    sign: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    w_mask_c(
        dst,
        dst_stride,
        tmp1,
        tmp2,
        w,
        h,
        mask,
        sign,
        0i32,
        0i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn w_mask_422_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    tmp1: *const int16_t,
    tmp2: *const int16_t,
    w: libc::c_int,
    h: libc::c_int,
    mut mask: *mut uint8_t,
    sign: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    w_mask_c(
        dst,
        dst_stride,
        tmp1,
        tmp2,
        w,
        h,
        mask,
        sign,
        1i32,
        0i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn w_mask_420_c(
    dst: *mut pixel,
    dst_stride: ptrdiff_t,
    tmp1: *const int16_t,
    tmp2: *const int16_t,
    w: libc::c_int,
    h: libc::c_int,
    mut mask: *mut uint8_t,
    sign: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    w_mask_c(
        dst,
        dst_stride,
        tmp1,
        tmp2,
        w,
        h,
        mask,
        sign,
        1i32,
        1i32,
        bitdepth_max,
    );
}
unsafe extern "C" fn warp_affine_8x8_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut src: *const pixel,
    src_stride: ptrdiff_t,
    abcd: *const int16_t,
    mut mx: libc::c_int,
    mut my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14i32 - (32i32 - clz(bitdepth_max as libc::c_uint));
    let mut mid: [int16_t; 120] = [0; 120];
    let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
    src = src.offset(-((3i64 * PXSTRIDE(src_stride)) as isize));
    let mut y: libc::c_int = 0i32;
    while y < 15i32 {
        let mut x: libc::c_int = 0i32;
        let mut tmx: libc::c_int = mx;
        while x < 8i32 {
            let filter: *const int8_t =
                (dav1d_mc_warp_filter[(64i32 + (tmx + 512i32 >> 10i32)) as usize]).as_ptr();
            *mid_ptr.offset(x as isize) = (*filter.offset(0isize) as libc::c_int
                * *src.offset((x - 3i32 * 1i32) as isize) as libc::c_int
                + *filter.offset(1isize) as libc::c_int
                    * *src.offset((x - 2i32 * 1i32) as isize) as libc::c_int
                + *filter.offset(2isize) as libc::c_int
                    * *src.offset((x - 1i32 * 1i32) as isize) as libc::c_int
                + *filter.offset(3isize) as libc::c_int
                    * *src.offset((x + 0i32 * 1i32) as isize) as libc::c_int
                + *filter.offset(4isize) as libc::c_int
                    * *src.offset((x + 1i32 * 1i32) as isize) as libc::c_int
                + *filter.offset(5isize) as libc::c_int
                    * *src.offset((x + 2i32 * 1i32) as isize) as libc::c_int
                + *filter.offset(6isize) as libc::c_int
                    * *src.offset((x + 3i32 * 1i32) as isize) as libc::c_int
                + *filter.offset(7isize) as libc::c_int
                    * *src.offset((x + 4i32 * 1i32) as isize) as libc::c_int
                + ((1i32) << 7i32 - intermediate_bits >> 1i32)
                >> 7i32 - intermediate_bits) as int16_t;
            x += 1;
            tmx += *abcd.offset(0isize) as libc::c_int;
        }
        src = src.offset(PXSTRIDE(src_stride) as isize);
        mid_ptr = mid_ptr.offset(8isize);
        y += 1;
        mx += *abcd.offset(1isize) as libc::c_int;
    }
    mid_ptr = &mut *mid.as_mut_ptr().offset((3i32 * 8i32) as isize) as *mut int16_t;
    let mut y_0: libc::c_int = 0i32;
    while y_0 < 8i32 {
        let mut x_0: libc::c_int = 0i32;
        let mut tmy: libc::c_int = my;
        while x_0 < 8i32 {
            let filter_0: *const int8_t =
                (dav1d_mc_warp_filter[(64i32 + (tmy + 512i32 >> 10i32)) as usize]).as_ptr();
            *dst.offset(x_0 as isize) = iclip(
                *filter_0.offset(0isize) as libc::c_int
                    * *mid_ptr.offset((x_0 - 3i32 * 8i32) as isize) as libc::c_int
                    + *filter_0.offset(1isize) as libc::c_int
                        * *mid_ptr.offset((x_0 - 2i32 * 8i32) as isize) as libc::c_int
                    + *filter_0.offset(2isize) as libc::c_int
                        * *mid_ptr.offset((x_0 - 1i32 * 8i32) as isize) as libc::c_int
                    + *filter_0.offset(3isize) as libc::c_int
                        * *mid_ptr.offset((x_0 + 0i32 * 8i32) as isize) as libc::c_int
                    + *filter_0.offset(4isize) as libc::c_int
                        * *mid_ptr.offset((x_0 + 1i32 * 8i32) as isize) as libc::c_int
                    + *filter_0.offset(5isize) as libc::c_int
                        * *mid_ptr.offset((x_0 + 2i32 * 8i32) as isize) as libc::c_int
                    + *filter_0.offset(6isize) as libc::c_int
                        * *mid_ptr.offset((x_0 + 3i32 * 8i32) as isize) as libc::c_int
                    + *filter_0.offset(7isize) as libc::c_int
                        * *mid_ptr.offset((x_0 + 4i32 * 8i32) as isize) as libc::c_int
                    + ((1i32) << 7i32 + intermediate_bits >> 1i32)
                    >> 7i32 + intermediate_bits,
                0i32,
                bitdepth_max,
            ) as pixel;
            x_0 += 1;
            tmy += *abcd.offset(2isize) as libc::c_int;
        }
        mid_ptr = mid_ptr.offset(8isize);
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        y_0 += 1;
        my += *abcd.offset(3isize) as libc::c_int;
    }
}
unsafe extern "C" fn warp_affine_8x8t_c(
    mut tmp: *mut int16_t,
    tmp_stride: ptrdiff_t,
    mut src: *const pixel,
    src_stride: ptrdiff_t,
    abcd: *const int16_t,
    mut mx: libc::c_int,
    mut my: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    let intermediate_bits: libc::c_int = 14i32 - (32i32 - clz(bitdepth_max as libc::c_uint));
    let mut mid: [int16_t; 120] = [0; 120];
    let mut mid_ptr: *mut int16_t = mid.as_mut_ptr();
    src = src.offset(-((3i64 * PXSTRIDE(src_stride)) as isize));
    let mut y: libc::c_int = 0i32;
    while y < 15i32 {
        let mut x: libc::c_int = 0i32;
        let mut tmx: libc::c_int = mx;
        while x < 8i32 {
            let filter: *const int8_t =
                (dav1d_mc_warp_filter[(64i32 + (tmx + 512i32 >> 10i32)) as usize]).as_ptr();
            *mid_ptr.offset(x as isize) = (*filter.offset(0isize) as libc::c_int
                * *src.offset((x - 3i32 * 1i32) as isize) as libc::c_int
                + *filter.offset(1isize) as libc::c_int
                    * *src.offset((x - 2i32 * 1i32) as isize) as libc::c_int
                + *filter.offset(2isize) as libc::c_int
                    * *src.offset((x - 1i32 * 1i32) as isize) as libc::c_int
                + *filter.offset(3isize) as libc::c_int
                    * *src.offset((x + 0i32 * 1i32) as isize) as libc::c_int
                + *filter.offset(4isize) as libc::c_int
                    * *src.offset((x + 1i32 * 1i32) as isize) as libc::c_int
                + *filter.offset(5isize) as libc::c_int
                    * *src.offset((x + 2i32 * 1i32) as isize) as libc::c_int
                + *filter.offset(6isize) as libc::c_int
                    * *src.offset((x + 3i32 * 1i32) as isize) as libc::c_int
                + *filter.offset(7isize) as libc::c_int
                    * *src.offset((x + 4i32 * 1i32) as isize) as libc::c_int
                + ((1i32) << 7i32 - intermediate_bits >> 1i32)
                >> 7i32 - intermediate_bits) as int16_t;
            x += 1;
            tmx += *abcd.offset(0isize) as libc::c_int;
        }
        src = src.offset(PXSTRIDE(src_stride) as isize);
        mid_ptr = mid_ptr.offset(8isize);
        y += 1;
        mx += *abcd.offset(1isize) as libc::c_int;
    }
    mid_ptr = &mut *mid.as_mut_ptr().offset((3i32 * 8i32) as isize) as *mut int16_t;
    let mut y_0: libc::c_int = 0i32;
    while y_0 < 8i32 {
        let mut x_0: libc::c_int = 0i32;
        let mut tmy: libc::c_int = my;
        while x_0 < 8i32 {
            let filter_0: *const int8_t =
                (dav1d_mc_warp_filter[(64i32 + (tmy + 512i32 >> 10i32)) as usize]).as_ptr();
            *tmp.offset(x_0 as isize) = ((*filter_0.offset(0isize) as libc::c_int
                * *mid_ptr.offset((x_0 - 3i32 * 8i32) as isize) as libc::c_int
                + *filter_0.offset(1isize) as libc::c_int
                    * *mid_ptr.offset((x_0 - 2i32 * 8i32) as isize) as libc::c_int
                + *filter_0.offset(2isize) as libc::c_int
                    * *mid_ptr.offset((x_0 - 1i32 * 8i32) as isize) as libc::c_int
                + *filter_0.offset(3isize) as libc::c_int
                    * *mid_ptr.offset((x_0 + 0i32 * 8i32) as isize) as libc::c_int
                + *filter_0.offset(4isize) as libc::c_int
                    * *mid_ptr.offset((x_0 + 1i32 * 8i32) as isize) as libc::c_int
                + *filter_0.offset(5isize) as libc::c_int
                    * *mid_ptr.offset((x_0 + 2i32 * 8i32) as isize) as libc::c_int
                + *filter_0.offset(6isize) as libc::c_int
                    * *mid_ptr.offset((x_0 + 3i32 * 8i32) as isize) as libc::c_int
                + *filter_0.offset(7isize) as libc::c_int
                    * *mid_ptr.offset((x_0 + 4i32 * 8i32) as isize) as libc::c_int
                + ((1i32) << 7i32 >> 1i32)
                >> 7i32)
                - 8192i32) as int16_t;
            x_0 += 1;
            tmy += *abcd.offset(2isize) as libc::c_int;
        }
        mid_ptr = mid_ptr.offset(8isize);
        tmp = tmp.offset(tmp_stride as isize);
        y_0 += 1;
        my += *abcd.offset(3isize) as libc::c_int;
    }
}
unsafe extern "C" fn emu_edge_c(
    bw: intptr_t,
    bh: intptr_t,
    iw: intptr_t,
    ih: intptr_t,
    x: intptr_t,
    y: intptr_t,
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut ref_0: *const pixel,
    ref_stride: ptrdiff_t,
) {
    ref_0 = ref_0.offset(
        (iclip(y as libc::c_int, 0i32, ih as libc::c_int - 1i32) as libc::c_long
            * PXSTRIDE(ref_stride)
            + iclip(x as libc::c_int, 0i32, iw as libc::c_int - 1i32) as libc::c_long)
            as isize,
    );
    let left_ext: libc::c_int = iclip(-x as libc::c_int, 0i32, bw as libc::c_int - 1i32);
    let right_ext: libc::c_int =
        iclip((x + bw - iw) as libc::c_int, 0i32, bw as libc::c_int - 1i32);
    if !(((left_ext + right_ext) as libc::c_long) < bw) {
        unreachable!();
    }
    let top_ext: libc::c_int = iclip(-y as libc::c_int, 0i32, bh as libc::c_int - 1i32);
    let bottom_ext: libc::c_int =
        iclip((y + bh - ih) as libc::c_int, 0i32, bh as libc::c_int - 1i32);
    if !(((top_ext + bottom_ext) as libc::c_long) < bh) {
        unreachable!();
    }
    let mut blk: *mut pixel = dst.offset((top_ext as libc::c_long * PXSTRIDE(dst_stride)) as isize);
    let center_w: libc::c_int =
        (bw - left_ext as libc::c_long - right_ext as libc::c_long) as libc::c_int;
    let center_h: libc::c_int =
        (bh - top_ext as libc::c_long - bottom_ext as libc::c_long) as libc::c_int;
    let mut y_0: libc::c_int = 0i32;
    while y_0 < center_h {
        memcpy(
            blk.offset(left_ext as isize) as *mut libc::c_void,
            ref_0 as *const libc::c_void,
            (center_w << 1i32) as libc::c_ulong,
        );
        if left_ext != 0 {
            pixel_set(blk, *blk.offset(left_ext as isize) as libc::c_int, left_ext);
        }
        if right_ext != 0 {
            pixel_set(
                blk.offset(left_ext as isize).offset(center_w as isize),
                *blk.offset((left_ext + center_w - 1i32) as isize) as libc::c_int,
                right_ext,
            );
        }
        ref_0 = ref_0.offset(PXSTRIDE(ref_stride) as isize);
        blk = blk.offset(PXSTRIDE(dst_stride) as isize);
        y_0 += 1;
    }
    blk = dst.offset((top_ext as libc::c_long * PXSTRIDE(dst_stride)) as isize);
    let mut y_1: libc::c_int = 0i32;
    while y_1 < top_ext {
        memcpy(
            dst as *mut libc::c_void,
            blk as *const libc::c_void,
            (bw << 1i32) as libc::c_ulong,
        );
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        y_1 += 1;
    }
    dst = dst.offset((center_h as libc::c_long * PXSTRIDE(dst_stride)) as isize);
    let mut y_2: libc::c_int = 0i32;
    while y_2 < bottom_ext {
        memcpy(
            dst as *mut libc::c_void,
            &mut *dst.offset(
                -(PXSTRIDE as unsafe extern "C" fn(ptrdiff_t) -> ptrdiff_t)(dst_stride) as isize,
            ) as *mut pixel as *const libc::c_void,
            (bw << 1i32) as libc::c_ulong,
        );
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        y_2 += 1;
    }
}
unsafe extern "C" fn resize_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut src: *const pixel,
    src_stride: ptrdiff_t,
    dst_w: libc::c_int,
    mut h: libc::c_int,
    src_w: libc::c_int,
    dx: libc::c_int,
    mx0: libc::c_int,
    bitdepth_max: libc::c_int,
) {
    loop {
        let mut mx: libc::c_int = mx0;
        let mut src_x: libc::c_int = -(1i32);
        let mut x: libc::c_int = 0i32;
        while x < dst_w {
            let F: *const int8_t = (dav1d_resize_filter[(mx >> 8i32) as usize]).as_ptr();
            *dst.offset(x as isize) = iclip(
                -(*F.offset(0isize) as libc::c_int
                    * *src.offset(iclip(src_x - 3i32, 0i32, src_w - 1i32) as isize) as libc::c_int
                    + *F.offset(1isize) as libc::c_int
                        * *src.offset(iclip(src_x - 2i32, 0i32, src_w - 1i32) as isize)
                            as libc::c_int
                    + *F.offset(2isize) as libc::c_int
                        * *src.offset(iclip(src_x - 1i32, 0i32, src_w - 1i32) as isize)
                            as libc::c_int
                    + *F.offset(3isize) as libc::c_int
                        * *src.offset(iclip(src_x + 0i32, 0i32, src_w - 1i32) as isize)
                            as libc::c_int
                    + *F.offset(4isize) as libc::c_int
                        * *src.offset(iclip(src_x + 1i32, 0i32, src_w - 1i32) as isize)
                            as libc::c_int
                    + *F.offset(5isize) as libc::c_int
                        * *src.offset(iclip(src_x + 2i32, 0i32, src_w - 1i32) as isize)
                            as libc::c_int
                    + *F.offset(6isize) as libc::c_int
                        * *src.offset(iclip(src_x + 3i32, 0i32, src_w - 1i32) as isize)
                            as libc::c_int
                    + *F.offset(7isize) as libc::c_int
                        * *src.offset(iclip(src_x + 4i32, 0i32, src_w - 1i32) as isize)
                            as libc::c_int)
                    + 64i32
                    >> 7i32,
                0i32,
                bitdepth_max,
            ) as pixel;
            mx += dx;
            src_x += mx >> 14i32;
            mx &= 0x3fffi32;
            x += 1;
        }
        dst = dst.offset(PXSTRIDE(dst_stride) as isize);
        src = src.offset(PXSTRIDE(src_stride) as isize);
        h -= 1;
        if !(h != 0) {
            break;
        }
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_mc_dsp_init_16bpc(c: *mut Dav1dMCDSPContext) {
    (*c).mc[FILTER_2D_8TAP_REGULAR as usize] = Some(
        put_8tap_regular_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR as usize] = Some(
        put_8tap_regular_scaled_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mct[FILTER_2D_8TAP_REGULAR as usize] = Some(
        prep_8tap_regular_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR as usize] = Some(
        prep_8tap_regular_scaled_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mc[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] = Some(
        put_8tap_regular_smooth_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] = Some(
        put_8tap_regular_smooth_scaled_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mct[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] = Some(
        prep_8tap_regular_smooth_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] = Some(
        prep_8tap_regular_smooth_scaled_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mc[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(
        put_8tap_regular_sharp_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(
        put_8tap_regular_sharp_scaled_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mct[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(
        prep_8tap_regular_sharp_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(
        prep_8tap_regular_sharp_scaled_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mc[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(
        put_8tap_sharp_regular_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(
        put_8tap_sharp_regular_scaled_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mct[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(
        prep_8tap_sharp_regular_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(
        prep_8tap_sharp_regular_scaled_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mc[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(
        put_8tap_sharp_smooth_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(
        put_8tap_sharp_smooth_scaled_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mct[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(
        prep_8tap_sharp_smooth_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(
        prep_8tap_sharp_smooth_scaled_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mc[FILTER_2D_8TAP_SHARP as usize] = Some(
        put_8tap_sharp_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP as usize] = Some(
        put_8tap_sharp_scaled_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mct[FILTER_2D_8TAP_SHARP as usize] = Some(
        prep_8tap_sharp_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP as usize] = Some(
        prep_8tap_sharp_scaled_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mc[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] = Some(
        put_8tap_smooth_regular_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] = Some(
        put_8tap_smooth_regular_scaled_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mct[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] = Some(
        prep_8tap_smooth_regular_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] = Some(
        prep_8tap_smooth_regular_scaled_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mc[FILTER_2D_8TAP_SMOOTH as usize] = Some(
        put_8tap_smooth_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH as usize] = Some(
        put_8tap_smooth_scaled_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mct[FILTER_2D_8TAP_SMOOTH as usize] = Some(
        prep_8tap_smooth_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH as usize] = Some(
        prep_8tap_smooth_scaled_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mc[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(
        put_8tap_smooth_sharp_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(
        put_8tap_smooth_sharp_scaled_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mct[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(
        prep_8tap_smooth_sharp_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(
        prep_8tap_smooth_sharp_scaled_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mc[FILTER_2D_BILINEAR as usize] = Some(
        put_bilin_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mc_scaled[FILTER_2D_BILINEAR as usize] = Some(
        put_bilin_scaled_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mct[FILTER_2D_BILINEAR as usize] = Some(
        prep_bilin_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mct_scaled[FILTER_2D_BILINEAR as usize] = Some(
        prep_bilin_scaled_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).avg = Some(
        avg_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const int16_t,
                *const int16_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).w_avg = Some(
        w_avg_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const int16_t,
                *const int16_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).mask = Some(
        mask_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const int16_t,
                *const int16_t,
                libc::c_int,
                libc::c_int,
                *const uint8_t,
                libc::c_int,
            ) -> (),
    );
    (*c).blend = Some(
        blend_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                libc::c_int,
                libc::c_int,
                *const uint8_t,
            ) -> (),
    );
    (*c).blend_v = Some(
        blend_v_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).blend_h = Some(
        blend_h_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).w_mask[0usize] = Some(
        w_mask_444_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const int16_t,
                *const int16_t,
                libc::c_int,
                libc::c_int,
                *mut uint8_t,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).w_mask[1usize] = Some(
        w_mask_422_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const int16_t,
                *const int16_t,
                libc::c_int,
                libc::c_int,
                *mut uint8_t,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).w_mask[2usize] = Some(
        w_mask_420_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const int16_t,
                *const int16_t,
                libc::c_int,
                libc::c_int,
                *mut uint8_t,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).warp8x8 = Some(
        warp_affine_8x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                *const int16_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).warp8x8t = Some(
        warp_affine_8x8t_c
            as unsafe extern "C" fn(
                *mut int16_t,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                *const int16_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).emu_edge = Some(
        emu_edge_c
            as unsafe extern "C" fn(
                intptr_t,
                intptr_t,
                intptr_t,
                intptr_t,
                intptr_t,
                intptr_t,
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
            ) -> (),
    );
    (*c).resize = Some(
        resize_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
}
