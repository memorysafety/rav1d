use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    static dav1d_cdef_directions: [[int8_t; 2]; 12];
}




pub type pixel = uint8_t;
pub type CdefEdgeFlags = libc::c_uint;
pub const CDEF_HAVE_BOTTOM: CdefEdgeFlags = 8;
pub const CDEF_HAVE_TOP: CdefEdgeFlags = 4;
pub const CDEF_HAVE_RIGHT: CdefEdgeFlags = 2;
pub const CDEF_HAVE_LEFT: CdefEdgeFlags = 1;
pub type const_left_pixel_row_2px = *const [pixel; 2];
pub type cdef_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        const_left_pixel_row_2px,
        *const pixel,
        *const pixel,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        CdefEdgeFlags,
    ) -> (),
>;
pub type cdef_dir_fn = Option::<
    unsafe extern "C" fn(*const pixel, ptrdiff_t, *mut libc::c_uint) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dCdefDSPContext {
    pub dir: cdef_dir_fn,
    pub fb: [cdef_fn; 3],
}
#[inline]
unsafe extern "C" fn imax(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
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
unsafe extern "C" fn umin(a: libc::c_uint, b: libc::c_uint) -> libc::c_uint {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn iclip(
    v: libc::c_int,
    min: libc::c_int,
    max: libc::c_int,
) -> libc::c_int {
    return if v < min { min } else if v > max { max } else { v };
}
#[inline]
unsafe extern "C" fn apply_sign(v: libc::c_int, s: libc::c_int) -> libc::c_int {
    return if s < 0 as libc::c_int { -v } else { v };
}
#[inline]
unsafe extern "C" fn ulog2(v: libc::c_uint) -> libc::c_int {
    return 31 as libc::c_int - clz(v);
}
#[inline]
unsafe extern "C" fn constrain(
    diff: libc::c_int,
    threshold: libc::c_int,
    shift: libc::c_int,
) -> libc::c_int {
    let adiff: libc::c_int = abs(diff);
    return apply_sign(
        imin(adiff, imax(0 as libc::c_int, threshold - (adiff >> shift))),
        diff,
    );
}
#[inline]
unsafe extern "C" fn fill(
    mut tmp: *mut int16_t,
    stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < h {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < w {
            *tmp
                .offset(
                    x as isize,
                ) = (-(32767 as libc::c_int) - 1 as libc::c_int) as int16_t;
            x += 1;
        }
        tmp = tmp.offset(stride as isize);
        y += 1;
    }
}
unsafe extern "C" fn padding(
    mut tmp: *mut int16_t,
    tmp_stride: ptrdiff_t,
    mut src: *const pixel,
    src_stride: ptrdiff_t,
    mut left: *const [pixel; 2],
    mut top: *const pixel,
    mut bottom: *const pixel,
    w: libc::c_int,
    h: libc::c_int,
    edges: CdefEdgeFlags,
) {
    let mut x_start: libc::c_int = -(2 as libc::c_int);
    let mut x_end: libc::c_int = w + 2 as libc::c_int;
    let mut y_start: libc::c_int = -(2 as libc::c_int);
    let mut y_end: libc::c_int = h + 2 as libc::c_int;
    if edges as libc::c_uint & CDEF_HAVE_TOP as libc::c_int as libc::c_uint == 0 {
        fill(
            tmp
                .offset(-(2 as libc::c_int as isize))
                .offset(-((2 as libc::c_int as libc::c_long * tmp_stride) as isize)),
            tmp_stride,
            w + 4 as libc::c_int,
            2 as libc::c_int,
        );
        y_start = 0 as libc::c_int;
    }
    if edges as libc::c_uint & CDEF_HAVE_BOTTOM as libc::c_int as libc::c_uint == 0 {
        fill(
            tmp
                .offset((h as libc::c_long * tmp_stride) as isize)
                .offset(-(2 as libc::c_int as isize)),
            tmp_stride,
            w + 4 as libc::c_int,
            2 as libc::c_int,
        );
        y_end -= 2 as libc::c_int;
    }
    if edges as libc::c_uint & CDEF_HAVE_LEFT as libc::c_int as libc::c_uint == 0 {
        fill(
            tmp
                .offset((y_start as libc::c_long * tmp_stride) as isize)
                .offset(-(2 as libc::c_int as isize)),
            tmp_stride,
            2 as libc::c_int,
            y_end - y_start,
        );
        x_start = 0 as libc::c_int;
    }
    if edges as libc::c_uint & CDEF_HAVE_RIGHT as libc::c_int as libc::c_uint == 0 {
        fill(
            tmp
                .offset((y_start as libc::c_long * tmp_stride) as isize)
                .offset(w as isize),
            tmp_stride,
            2 as libc::c_int,
            y_end - y_start,
        );
        x_end -= 2 as libc::c_int;
    }
    let mut y: libc::c_int = y_start;
    while y < 0 as libc::c_int {
        let mut x: libc::c_int = x_start;
        while x < x_end {
            *tmp
                .offset(
                    (x as libc::c_long + y as libc::c_long * tmp_stride) as isize,
                ) = *top.offset(x as isize) as int16_t;
            x += 1;
        }
        top = top.offset(src_stride as isize);
        y += 1;
    }
    let mut y_0: libc::c_int = 0 as libc::c_int;
    while y_0 < h {
        let mut x_0: libc::c_int = x_start;
        while x_0 < 0 as libc::c_int {
            *tmp
                .offset(
                    (x_0 as libc::c_long + y_0 as libc::c_long * tmp_stride) as isize,
                ) = (*left.offset(y_0 as isize))[(2 as libc::c_int + x_0) as usize]
                as int16_t;
            x_0 += 1;
        }
        y_0 += 1;
    }
    let mut y_1: libc::c_int = 0 as libc::c_int;
    while y_1 < h {
        let mut x_1: libc::c_int = if y_1 < h { 0 as libc::c_int } else { x_start };
        while x_1 < x_end {
            *tmp.offset(x_1 as isize) = *src.offset(x_1 as isize) as int16_t;
            x_1 += 1;
        }
        src = src.offset(src_stride as isize);
        tmp = tmp.offset(tmp_stride as isize);
        y_1 += 1;
    }
    let mut y_2: libc::c_int = h;
    while y_2 < y_end {
        let mut x_2: libc::c_int = x_start;
        while x_2 < x_end {
            *tmp.offset(x_2 as isize) = *bottom.offset(x_2 as isize) as int16_t;
            x_2 += 1;
        }
        bottom = bottom.offset(src_stride as isize);
        tmp = tmp.offset(tmp_stride as isize);
        y_2 += 1;
    }
}
#[inline(never)]
unsafe extern "C" fn cdef_filter_block_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut left: *const [pixel; 2],
    top: *const pixel,
    bottom: *const pixel,
    pri_strength: libc::c_int,
    sec_strength: libc::c_int,
    dir: libc::c_int,
    damping: libc::c_int,
    w: libc::c_int,
    mut h: libc::c_int,
    edges: CdefEdgeFlags,
) {
    let tmp_stride: ptrdiff_t = 12 as libc::c_int as ptrdiff_t;
    if !((w == 4 as libc::c_int || w == 8 as libc::c_int)
        && (h == 4 as libc::c_int || h == 8 as libc::c_int))
    {
        unreachable!();
    }
    let mut tmp_buf: [int16_t; 144] = [0; 144];
    let mut tmp: *mut int16_t = tmp_buf
        .as_mut_ptr()
        .offset((2 as libc::c_int as libc::c_long * tmp_stride) as isize)
        .offset(2 as libc::c_int as isize);
    padding(tmp, tmp_stride, dst, dst_stride, left, top, bottom, w, h, edges);
    if pri_strength != 0 {
        let bitdepth_min_8: libc::c_int = 8 as libc::c_int - 8 as libc::c_int;
        let pri_tap: libc::c_int = 4 as libc::c_int
            - (pri_strength >> bitdepth_min_8 & 1 as libc::c_int);
        let pri_shift: libc::c_int = imax(
            0 as libc::c_int,
            damping - ulog2(pri_strength as libc::c_uint),
        );
        if sec_strength != 0 {
            let sec_shift: libc::c_int = damping - ulog2(sec_strength as libc::c_uint);
            loop {
                let mut x: libc::c_int = 0 as libc::c_int;
                while x < w {
                    let px: libc::c_int = *dst.offset(x as isize) as libc::c_int;
                    let mut sum: libc::c_int = 0 as libc::c_int;
                    let mut max: libc::c_int = px;
                    let mut min: libc::c_int = px;
                    let mut pri_tap_k: libc::c_int = pri_tap;
                    let mut k: libc::c_int = 0 as libc::c_int;
                    while k < 2 as libc::c_int {
                        let off1: libc::c_int = dav1d_cdef_directions[(dir
                            + 2 as libc::c_int) as usize][k as usize] as libc::c_int;
                        let p0: libc::c_int = *tmp.offset((x + off1) as isize)
                            as libc::c_int;
                        let p1: libc::c_int = *tmp.offset((x - off1) as isize)
                            as libc::c_int;
                        sum += pri_tap_k * constrain(p0 - px, pri_strength, pri_shift);
                        sum += pri_tap_k * constrain(p1 - px, pri_strength, pri_shift);
                        pri_tap_k = pri_tap_k & 3 as libc::c_int | 2 as libc::c_int;
                        min = umin(p0 as libc::c_uint, min as libc::c_uint)
                            as libc::c_int;
                        max = imax(p0, max);
                        min = umin(p1 as libc::c_uint, min as libc::c_uint)
                            as libc::c_int;
                        max = imax(p1, max);
                        let off2: libc::c_int = dav1d_cdef_directions[(dir
                            + 4 as libc::c_int) as usize][k as usize] as libc::c_int;
                        let off3: libc::c_int = dav1d_cdef_directions[(dir
                            + 0 as libc::c_int) as usize][k as usize] as libc::c_int;
                        let s0: libc::c_int = *tmp.offset((x + off2) as isize)
                            as libc::c_int;
                        let s1: libc::c_int = *tmp.offset((x - off2) as isize)
                            as libc::c_int;
                        let s2: libc::c_int = *tmp.offset((x + off3) as isize)
                            as libc::c_int;
                        let s3: libc::c_int = *tmp.offset((x - off3) as isize)
                            as libc::c_int;
                        let sec_tap: libc::c_int = 2 as libc::c_int - k;
                        sum += sec_tap * constrain(s0 - px, sec_strength, sec_shift);
                        sum += sec_tap * constrain(s1 - px, sec_strength, sec_shift);
                        sum += sec_tap * constrain(s2 - px, sec_strength, sec_shift);
                        sum += sec_tap * constrain(s3 - px, sec_strength, sec_shift);
                        min = umin(s0 as libc::c_uint, min as libc::c_uint)
                            as libc::c_int;
                        max = imax(s0, max);
                        min = umin(s1 as libc::c_uint, min as libc::c_uint)
                            as libc::c_int;
                        max = imax(s1, max);
                        min = umin(s2 as libc::c_uint, min as libc::c_uint)
                            as libc::c_int;
                        max = imax(s2, max);
                        min = umin(s3 as libc::c_uint, min as libc::c_uint)
                            as libc::c_int;
                        max = imax(s3, max);
                        k += 1;
                    }
                    *dst
                        .offset(
                            x as isize,
                        ) = iclip(
                        px
                            + (sum - (sum < 0 as libc::c_int) as libc::c_int
                                + 8 as libc::c_int >> 4 as libc::c_int),
                        min,
                        max,
                    ) as pixel;
                    x += 1;
                }
                dst = dst.offset(dst_stride as isize);
                tmp = tmp.offset(tmp_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        } else {
            loop {
                let mut x_0: libc::c_int = 0 as libc::c_int;
                while x_0 < w {
                    let px_0: libc::c_int = *dst.offset(x_0 as isize) as libc::c_int;
                    let mut sum_0: libc::c_int = 0 as libc::c_int;
                    let mut pri_tap_k_0: libc::c_int = pri_tap;
                    let mut k_0: libc::c_int = 0 as libc::c_int;
                    while k_0 < 2 as libc::c_int {
                        let off: libc::c_int = dav1d_cdef_directions[(dir
                            + 2 as libc::c_int) as usize][k_0 as usize] as libc::c_int;
                        let p0_0: libc::c_int = *tmp.offset((x_0 + off) as isize)
                            as libc::c_int;
                        let p1_0: libc::c_int = *tmp.offset((x_0 - off) as isize)
                            as libc::c_int;
                        sum_0
                            += pri_tap_k_0
                                * constrain(p0_0 - px_0, pri_strength, pri_shift);
                        sum_0
                            += pri_tap_k_0
                                * constrain(p1_0 - px_0, pri_strength, pri_shift);
                        pri_tap_k_0 = pri_tap_k_0 & 3 as libc::c_int | 2 as libc::c_int;
                        k_0 += 1;
                    }
                    *dst
                        .offset(
                            x_0 as isize,
                        ) = (px_0
                        + (sum_0 - (sum_0 < 0 as libc::c_int) as libc::c_int
                            + 8 as libc::c_int >> 4 as libc::c_int)) as pixel;
                    x_0 += 1;
                }
                dst = dst.offset(dst_stride as isize);
                tmp = tmp.offset(tmp_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        }
    } else {
        if sec_strength == 0 {
            unreachable!();
        }
        let sec_shift_0: libc::c_int = damping - ulog2(sec_strength as libc::c_uint);
        loop {
            let mut x_1: libc::c_int = 0 as libc::c_int;
            while x_1 < w {
                let px_1: libc::c_int = *dst.offset(x_1 as isize) as libc::c_int;
                let mut sum_1: libc::c_int = 0 as libc::c_int;
                let mut k_1: libc::c_int = 0 as libc::c_int;
                while k_1 < 2 as libc::c_int {
                    let off1_0: libc::c_int = dav1d_cdef_directions[(dir
                        + 4 as libc::c_int) as usize][k_1 as usize] as libc::c_int;
                    let off2_0: libc::c_int = dav1d_cdef_directions[(dir
                        + 0 as libc::c_int) as usize][k_1 as usize] as libc::c_int;
                    let s0_0: libc::c_int = *tmp.offset((x_1 + off1_0) as isize)
                        as libc::c_int;
                    let s1_0: libc::c_int = *tmp.offset((x_1 - off1_0) as isize)
                        as libc::c_int;
                    let s2_0: libc::c_int = *tmp.offset((x_1 + off2_0) as isize)
                        as libc::c_int;
                    let s3_0: libc::c_int = *tmp.offset((x_1 - off2_0) as isize)
                        as libc::c_int;
                    let sec_tap_0: libc::c_int = 2 as libc::c_int - k_1;
                    sum_1
                        += sec_tap_0 * constrain(s0_0 - px_1, sec_strength, sec_shift_0);
                    sum_1
                        += sec_tap_0 * constrain(s1_0 - px_1, sec_strength, sec_shift_0);
                    sum_1
                        += sec_tap_0 * constrain(s2_0 - px_1, sec_strength, sec_shift_0);
                    sum_1
                        += sec_tap_0 * constrain(s3_0 - px_1, sec_strength, sec_shift_0);
                    k_1 += 1;
                }
                *dst
                    .offset(
                        x_1 as isize,
                    ) = (px_1
                    + (sum_1 - (sum_1 < 0 as libc::c_int) as libc::c_int
                        + 8 as libc::c_int >> 4 as libc::c_int)) as pixel;
                x_1 += 1;
            }
            dst = dst.offset(dst_stride as isize);
            tmp = tmp.offset(tmp_stride as isize);
            h -= 1;
            if !(h != 0) {
                break;
            }
        }
    };
}
unsafe extern "C" fn cdef_filter_block_4x4_c(
    dst: *mut pixel,
    stride: ptrdiff_t,
    mut left: *const [pixel; 2],
    top: *const pixel,
    bottom: *const pixel,
    pri_strength: libc::c_int,
    sec_strength: libc::c_int,
    dir: libc::c_int,
    damping: libc::c_int,
    edges: CdefEdgeFlags,
) {
    cdef_filter_block_c(
        dst,
        stride,
        left,
        top,
        bottom,
        pri_strength,
        sec_strength,
        dir,
        damping,
        4 as libc::c_int,
        4 as libc::c_int,
        edges,
    );
}
unsafe extern "C" fn cdef_filter_block_4x8_c(
    dst: *mut pixel,
    stride: ptrdiff_t,
    mut left: *const [pixel; 2],
    top: *const pixel,
    bottom: *const pixel,
    pri_strength: libc::c_int,
    sec_strength: libc::c_int,
    dir: libc::c_int,
    damping: libc::c_int,
    edges: CdefEdgeFlags,
) {
    cdef_filter_block_c(
        dst,
        stride,
        left,
        top,
        bottom,
        pri_strength,
        sec_strength,
        dir,
        damping,
        4 as libc::c_int,
        8 as libc::c_int,
        edges,
    );
}
unsafe extern "C" fn cdef_filter_block_8x8_c(
    dst: *mut pixel,
    stride: ptrdiff_t,
    mut left: *const [pixel; 2],
    top: *const pixel,
    bottom: *const pixel,
    pri_strength: libc::c_int,
    sec_strength: libc::c_int,
    dir: libc::c_int,
    damping: libc::c_int,
    edges: CdefEdgeFlags,
) {
    cdef_filter_block_c(
        dst,
        stride,
        left,
        top,
        bottom,
        pri_strength,
        sec_strength,
        dir,
        damping,
        8 as libc::c_int,
        8 as libc::c_int,
        edges,
    );
}
unsafe extern "C" fn cdef_find_dir_c(
    mut img: *const pixel,
    stride: ptrdiff_t,
    var: *mut libc::c_uint,
) -> libc::c_int {
    let bitdepth_min_8: libc::c_int = 8 as libc::c_int - 8 as libc::c_int;
    let mut partial_sum_hv: [[libc::c_int; 8]; 2] = [
        [0 as libc::c_int, 0, 0, 0, 0, 0, 0, 0],
        [0; 8],
    ];
    let mut partial_sum_diag: [[libc::c_int; 15]; 2] = [
        [0 as libc::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0; 15],
    ];
    let mut partial_sum_alt: [[libc::c_int; 11]; 4] = [
        [0 as libc::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0; 11],
        [0; 11],
        [0; 11],
    ];
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 8 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 8 as libc::c_int {
            let px: libc::c_int = (*img.offset(x as isize) as libc::c_int
                >> bitdepth_min_8) - 128 as libc::c_int;
            partial_sum_diag[0 as libc::c_int as usize][(y + x) as usize] += px;
            partial_sum_alt[0 as libc::c_int
                as usize][(y + (x >> 1 as libc::c_int)) as usize] += px;
            partial_sum_hv[0 as libc::c_int as usize][y as usize] += px;
            partial_sum_alt[1 as libc::c_int
                as usize][(3 as libc::c_int + y - (x >> 1 as libc::c_int)) as usize]
                += px;
            partial_sum_diag[1 as libc::c_int
                as usize][(7 as libc::c_int + y - x) as usize] += px;
            partial_sum_alt[2 as libc::c_int
                as usize][(3 as libc::c_int - (y >> 1 as libc::c_int) + x) as usize]
                += px;
            partial_sum_hv[1 as libc::c_int as usize][x as usize] += px;
            partial_sum_alt[3 as libc::c_int
                as usize][((y >> 1 as libc::c_int) + x) as usize] += px;
            x += 1;
        }
        img = img.offset(stride as isize);
        y += 1;
    }
    let mut cost: [libc::c_uint; 8] = [
        0 as libc::c_int as libc::c_uint,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut n: libc::c_int = 0 as libc::c_int;
    while n < 8 as libc::c_int {
        cost[2 as libc::c_int
            as usize] = (cost[2 as libc::c_int as usize])
            .wrapping_add(
                (partial_sum_hv[0 as libc::c_int as usize][n as usize]
                    * partial_sum_hv[0 as libc::c_int as usize][n as usize])
                    as libc::c_uint,
            );
        cost[6 as libc::c_int
            as usize] = (cost[6 as libc::c_int as usize])
            .wrapping_add(
                (partial_sum_hv[1 as libc::c_int as usize][n as usize]
                    * partial_sum_hv[1 as libc::c_int as usize][n as usize])
                    as libc::c_uint,
            );
        n += 1;
    }
    cost[2 as libc::c_int
        as usize] = (cost[2 as libc::c_int as usize])
        .wrapping_mul(105 as libc::c_int as libc::c_uint);
    cost[6 as libc::c_int
        as usize] = (cost[6 as libc::c_int as usize])
        .wrapping_mul(105 as libc::c_int as libc::c_uint);
    static mut div_table: [uint16_t; 7] = [
        840 as libc::c_int as uint16_t,
        420 as libc::c_int as uint16_t,
        280 as libc::c_int as uint16_t,
        210 as libc::c_int as uint16_t,
        168 as libc::c_int as uint16_t,
        140 as libc::c_int as uint16_t,
        120 as libc::c_int as uint16_t,
    ];
    let mut n_0: libc::c_int = 0 as libc::c_int;
    while n_0 < 7 as libc::c_int {
        let d: libc::c_int = div_table[n_0 as usize] as libc::c_int;
        cost[0 as libc::c_int
            as usize] = (cost[0 as libc::c_int as usize])
            .wrapping_add(
                ((partial_sum_diag[0 as libc::c_int as usize][n_0 as usize]
                    * partial_sum_diag[0 as libc::c_int as usize][n_0 as usize]
                    + partial_sum_diag[0 as libc::c_int
                        as usize][(14 as libc::c_int - n_0) as usize]
                        * partial_sum_diag[0 as libc::c_int
                            as usize][(14 as libc::c_int - n_0) as usize]) * d)
                    as libc::c_uint,
            );
        cost[4 as libc::c_int
            as usize] = (cost[4 as libc::c_int as usize])
            .wrapping_add(
                ((partial_sum_diag[1 as libc::c_int as usize][n_0 as usize]
                    * partial_sum_diag[1 as libc::c_int as usize][n_0 as usize]
                    + partial_sum_diag[1 as libc::c_int
                        as usize][(14 as libc::c_int - n_0) as usize]
                        * partial_sum_diag[1 as libc::c_int
                            as usize][(14 as libc::c_int - n_0) as usize]) * d)
                    as libc::c_uint,
            );
        n_0 += 1;
    }
    cost[0 as libc::c_int
        as usize] = (cost[0 as libc::c_int as usize])
        .wrapping_add(
            (partial_sum_diag[0 as libc::c_int as usize][7 as libc::c_int as usize]
                * partial_sum_diag[0 as libc::c_int as usize][7 as libc::c_int as usize]
                * 105 as libc::c_int) as libc::c_uint,
        );
    cost[4 as libc::c_int
        as usize] = (cost[4 as libc::c_int as usize])
        .wrapping_add(
            (partial_sum_diag[1 as libc::c_int as usize][7 as libc::c_int as usize]
                * partial_sum_diag[1 as libc::c_int as usize][7 as libc::c_int as usize]
                * 105 as libc::c_int) as libc::c_uint,
        );
    let mut n_1: libc::c_int = 0 as libc::c_int;
    while n_1 < 4 as libc::c_int {
        let cost_ptr: *mut libc::c_uint = &mut *cost
            .as_mut_ptr()
            .offset((n_1 * 2 as libc::c_int + 1 as libc::c_int) as isize)
            as *mut libc::c_uint;
        let mut m: libc::c_int = 0 as libc::c_int;
        while m < 5 as libc::c_int {
            *cost_ptr = (*cost_ptr)
                .wrapping_add(
                    (partial_sum_alt[n_1 as usize][(3 as libc::c_int + m) as usize]
                        * partial_sum_alt[n_1 as usize][(3 as libc::c_int + m) as usize])
                        as libc::c_uint,
                );
            m += 1;
        }
        *cost_ptr = (*cost_ptr).wrapping_mul(105 as libc::c_int as libc::c_uint);
        let mut m_0: libc::c_int = 0 as libc::c_int;
        while m_0 < 3 as libc::c_int {
            let d_0: libc::c_int = div_table[(2 as libc::c_int * m_0 + 1 as libc::c_int)
                as usize] as libc::c_int;
            *cost_ptr = (*cost_ptr)
                .wrapping_add(
                    ((partial_sum_alt[n_1 as usize][m_0 as usize]
                        * partial_sum_alt[n_1 as usize][m_0 as usize]
                        + partial_sum_alt[n_1
                            as usize][(10 as libc::c_int - m_0) as usize]
                            * partial_sum_alt[n_1
                                as usize][(10 as libc::c_int - m_0) as usize]) * d_0)
                        as libc::c_uint,
                );
            m_0 += 1;
        }
        n_1 += 1;
    }
    let mut best_dir: libc::c_int = 0 as libc::c_int;
    let mut best_cost: libc::c_uint = cost[0 as libc::c_int as usize];
    let mut n_2: libc::c_int = 1 as libc::c_int;
    while n_2 < 8 as libc::c_int {
        if cost[n_2 as usize] > best_cost {
            best_cost = cost[n_2 as usize];
            best_dir = n_2;
        }
        n_2 += 1;
    }
    *var = best_cost.wrapping_sub(cost[(best_dir ^ 4 as libc::c_int) as usize])
        >> 10 as libc::c_int;
    return best_dir;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_cdef_dsp_init_8bpc(c: *mut Dav1dCdefDSPContext) {
    (*c)
        .dir = Some(
        cdef_find_dir_c
            as unsafe extern "C" fn(
                *const pixel,
                ptrdiff_t,
                *mut libc::c_uint,
            ) -> libc::c_int,
    );
    (*c)
        .fb[0 as libc::c_int
        as usize] = Some(
        cdef_filter_block_8x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const [pixel; 2],
                *const pixel,
                *const pixel,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                CdefEdgeFlags,
            ) -> (),
    );
    (*c)
        .fb[1 as libc::c_int
        as usize] = Some(
        cdef_filter_block_4x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const [pixel; 2],
                *const pixel,
                *const pixel,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                CdefEdgeFlags,
            ) -> (),
    );
    (*c)
        .fb[2 as libc::c_int
        as usize] = Some(
        cdef_filter_block_4x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const [pixel; 2],
                *const pixel,
                *const pixel,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                CdefEdgeFlags,
            ) -> (),
    );
}
