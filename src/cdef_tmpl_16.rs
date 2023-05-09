use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
#[cfg(feature = "asm")]
use cfg_if::cfg_if;

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
extern "C" {
    fn dav1d_cdef_filter_4x4_16bpc_avx512icl(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: const_left_pixel_row_2px,
        top: *const pixel,
        bottom: *const pixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_cdef_filter_4x8_16bpc_avx512icl(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: const_left_pixel_row_2px,
        top: *const pixel,
        bottom: *const pixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_cdef_filter_8x8_16bpc_avx512icl(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: const_left_pixel_row_2px,
        top: *const pixel,
        bottom: *const pixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_cdef_filter_4x4_16bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: const_left_pixel_row_2px,
        top: *const pixel,
        bottom: *const pixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_cdef_filter_4x8_16bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: const_left_pixel_row_2px,
        top: *const pixel,
        bottom: *const pixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_cdef_filter_8x8_16bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: const_left_pixel_row_2px,
        top: *const pixel,
        bottom: *const pixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_cdef_dir_16bpc_avx2(
        dst: *const pixel,
        dst_stride: ptrdiff_t,
        var: *mut libc::c_uint,
        bitdepth_max: libc::c_int,
    ) -> libc::c_int;
    fn dav1d_cdef_dir_16bpc_sse4(
        dst: *const pixel,
        dst_stride: ptrdiff_t,
        var: *mut libc::c_uint,
        bitdepth_max: libc::c_int,
    ) -> libc::c_int;
    fn dav1d_cdef_filter_4x4_16bpc_ssse3(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: const_left_pixel_row_2px,
        top: *const pixel,
        bottom: *const pixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_cdef_filter_4x8_16bpc_ssse3(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: const_left_pixel_row_2px,
        top: *const pixel,
        bottom: *const pixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_cdef_filter_8x8_16bpc_ssse3(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: const_left_pixel_row_2px,
        top: *const pixel,
        bottom: *const pixel,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_cdef_dir_16bpc_ssse3(
        dst: *const pixel,
        dst_stride: ptrdiff_t,
        var: *mut libc::c_uint,
        bitdepth_max: libc::c_int,
    ) -> libc::c_int;
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
extern "C" {
    fn dav1d_cdef_find_dir_16bpc_neon(
        dst: *const pixel,
        dst_stride: ptrdiff_t,
        var: *mut libc::c_uint,
        bitdepth_max: libc::c_int,
    ) -> libc::c_int;
    fn dav1d_cdef_padding4_16bpc_neon(
        tmp: *mut uint16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        left: *const [pixel; 2],
        top: *const pixel,
        bottom: *const pixel,
        h: libc::c_int,
        edges: CdefEdgeFlags,
    );
    fn dav1d_cdef_padding8_16bpc_neon(
        tmp: *mut uint16_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        left: *const [pixel; 2],
        top: *const pixel,
        bottom: *const pixel,
        h: libc::c_int,
        edges: CdefEdgeFlags,
    );
    fn dav1d_cdef_filter4_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const uint16_t,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        h: libc::c_int,
        edges: size_t,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_cdef_filter8_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        tmp: *const uint16_t,
        pri_strength: libc::c_int,
        sec_strength: libc::c_int,
        dir: libc::c_int,
        damping: libc::c_int,
        h: libc::c_int,
        edges: size_t,
        bitdepth_max: libc::c_int,
    );
}

use crate::src::tables::dav1d_cdef_directions;

pub type pixel = uint16_t;
use crate::src::cdef::CdefEdgeFlags;
use crate::src::cdef::CDEF_HAVE_BOTTOM;
use crate::src::cdef::CDEF_HAVE_LEFT;
use crate::src::cdef::CDEF_HAVE_RIGHT;
use crate::src::cdef::CDEF_HAVE_TOP;
pub type const_left_pixel_row_2px = *const [pixel; 2];
pub type cdef_fn = Option<
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
        libc::c_int,
    ) -> (),
>;
pub type cdef_dir_fn = Option<
    unsafe extern "C" fn(*const pixel, ptrdiff_t, *mut libc::c_uint, libc::c_int) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dCdefDSPContext {
    pub dir: cdef_dir_fn,
    pub fb: [cdef_fn; 3],
}
use crate::include::common::attributes::clz;
use crate::include::common::intops::imax;

use crate::include::common::intops::iclip;
use crate::include::common::intops::umin;

use crate::include::common::intops::ulog2;
#[inline]
unsafe extern "C" fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 != 0 {
        unreachable!();
    }
    return x >> 1;
}
use crate::src::cdef::constrain;
use crate::src::cdef::fill;
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
    let mut x_start = -(2 as libc::c_int);
    let mut x_end = w + 2;
    let mut y_start = -(2 as libc::c_int);
    let mut y_end = h + 2;
    if edges as libc::c_uint & CDEF_HAVE_TOP as libc::c_int as libc::c_uint == 0 {
        fill(
            tmp.offset(-2).offset(-((2 * tmp_stride) as isize)),
            tmp_stride,
            w + 4,
            2 as libc::c_int,
        );
        y_start = 0 as libc::c_int;
    }
    if edges as libc::c_uint & CDEF_HAVE_BOTTOM as libc::c_int as libc::c_uint == 0 {
        fill(
            tmp.offset((h as isize * tmp_stride) as isize)
                .offset(-(2 as libc::c_int as isize)),
            tmp_stride,
            w + 4,
            2 as libc::c_int,
        );
        y_end -= 2 as libc::c_int;
    }
    if edges as libc::c_uint & CDEF_HAVE_LEFT as libc::c_int as libc::c_uint == 0 {
        fill(
            tmp.offset((y_start as isize * tmp_stride) as isize)
                .offset(-(2 as libc::c_int as isize)),
            tmp_stride,
            2 as libc::c_int,
            y_end - y_start,
        );
        x_start = 0 as libc::c_int;
    }
    if edges as libc::c_uint & CDEF_HAVE_RIGHT as libc::c_int as libc::c_uint == 0 {
        fill(
            tmp.offset((y_start as isize * tmp_stride) as isize)
                .offset(w as isize),
            tmp_stride,
            2 as libc::c_int,
            y_end - y_start,
        );
        x_end -= 2 as libc::c_int;
    }
    let mut y = y_start;
    while y < 0 {
        let mut x = x_start;
        while x < x_end {
            *tmp.offset((x as isize + y as isize * tmp_stride) as isize) =
                *top.offset(x as isize) as int16_t;
            x += 1;
        }
        top = top.offset(PXSTRIDE(src_stride) as isize);
        y += 1;
    }
    let mut y_0 = 0;
    while y_0 < h {
        let mut x_0 = x_start;
        while x_0 < 0 {
            *tmp.offset((x_0 as isize + y_0 as isize * tmp_stride) as isize) =
                (*left.offset(y_0 as isize))[(2 + x_0) as usize] as int16_t;
            x_0 += 1;
        }
        y_0 += 1;
    }
    let mut y_1 = 0;
    while y_1 < h {
        let mut x_1 = if y_1 < h { 0 as libc::c_int } else { x_start };
        while x_1 < x_end {
            *tmp.offset(x_1 as isize) = *src.offset(x_1 as isize) as int16_t;
            x_1 += 1;
        }
        src = src.offset(PXSTRIDE(src_stride) as isize);
        tmp = tmp.offset(tmp_stride as isize);
        y_1 += 1;
    }
    let mut y_2 = h;
    while y_2 < y_end {
        let mut x_2 = x_start;
        while x_2 < x_end {
            *tmp.offset(x_2 as isize) = *bottom.offset(x_2 as isize) as int16_t;
            x_2 += 1;
        }
        bottom = bottom.offset(PXSTRIDE(src_stride) as isize);
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
    bitdepth_max: libc::c_int,
) {
    let tmp_stride: ptrdiff_t = 12 as libc::c_int as ptrdiff_t;
    if !((w == 4 || w == 8) && (h == 4 || h == 8)) {
        unreachable!();
    }
    let mut tmp_buf: [int16_t; 144] = [0; 144];
    let mut tmp: *mut int16_t = tmp_buf
        .as_mut_ptr()
        .offset((2 * tmp_stride) as isize)
        .offset(2);
    padding(
        tmp, tmp_stride, dst, dst_stride, left, top, bottom, w, h, edges,
    );
    if pri_strength != 0 {
        let bitdepth_min_8 = 32 as libc::c_int - clz(bitdepth_max as libc::c_uint) - 8;
        let pri_tap = 4 as libc::c_int - (pri_strength >> bitdepth_min_8 & 1);
        let pri_shift = imax(
            0 as libc::c_int,
            damping - ulog2(pri_strength as libc::c_uint),
        );
        if sec_strength != 0 {
            let sec_shift = damping - ulog2(sec_strength as libc::c_uint);
            loop {
                let mut x = 0;
                while x < w {
                    let px = *dst.offset(x as isize) as libc::c_int;
                    let mut sum = 0;
                    let mut max = px;
                    let mut min = px;
                    let mut pri_tap_k = pri_tap;
                    let mut k = 0;
                    while k < 2 {
                        let off1 =
                            dav1d_cdef_directions[(dir + 2) as usize][k as usize] as libc::c_int;
                        let p0 = *tmp.offset((x + off1) as isize) as libc::c_int;
                        let p1 = *tmp.offset((x - off1) as isize) as libc::c_int;
                        sum += pri_tap_k * constrain(p0 - px, pri_strength, pri_shift);
                        sum += pri_tap_k * constrain(p1 - px, pri_strength, pri_shift);
                        pri_tap_k = pri_tap_k & 3 | 2;
                        min = umin(p0 as libc::c_uint, min as libc::c_uint) as libc::c_int;
                        max = imax(p0, max);
                        min = umin(p1 as libc::c_uint, min as libc::c_uint) as libc::c_int;
                        max = imax(p1, max);
                        let off2 =
                            dav1d_cdef_directions[(dir + 4) as usize][k as usize] as libc::c_int;
                        let off3 =
                            dav1d_cdef_directions[(dir + 0) as usize][k as usize] as libc::c_int;
                        let s0 = *tmp.offset((x + off2) as isize) as libc::c_int;
                        let s1 = *tmp.offset((x - off2) as isize) as libc::c_int;
                        let s2 = *tmp.offset((x + off3) as isize) as libc::c_int;
                        let s3 = *tmp.offset((x - off3) as isize) as libc::c_int;
                        let sec_tap = 2 - k;
                        sum += sec_tap * constrain(s0 - px, sec_strength, sec_shift);
                        sum += sec_tap * constrain(s1 - px, sec_strength, sec_shift);
                        sum += sec_tap * constrain(s2 - px, sec_strength, sec_shift);
                        sum += sec_tap * constrain(s3 - px, sec_strength, sec_shift);
                        min = umin(s0 as libc::c_uint, min as libc::c_uint) as libc::c_int;
                        max = imax(s0, max);
                        min = umin(s1 as libc::c_uint, min as libc::c_uint) as libc::c_int;
                        max = imax(s1, max);
                        min = umin(s2 as libc::c_uint, min as libc::c_uint) as libc::c_int;
                        max = imax(s2, max);
                        min = umin(s3 as libc::c_uint, min as libc::c_uint) as libc::c_int;
                        max = imax(s3, max);
                        k += 1;
                    }
                    *dst.offset(x as isize) =
                        iclip(px + (sum - (sum < 0) as libc::c_int + 8 >> 4), min, max) as pixel;
                    x += 1;
                }
                dst = dst.offset(PXSTRIDE(dst_stride) as isize);
                tmp = tmp.offset(tmp_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        } else {
            loop {
                let mut x_0 = 0;
                while x_0 < w {
                    let px_0 = *dst.offset(x_0 as isize) as libc::c_int;
                    let mut sum_0 = 0;
                    let mut pri_tap_k_0 = pri_tap;
                    let mut k_0 = 0;
                    while k_0 < 2 {
                        let off =
                            dav1d_cdef_directions[(dir + 2) as usize][k_0 as usize] as libc::c_int;
                        let p0_0 = *tmp.offset((x_0 + off) as isize) as libc::c_int;
                        let p1_0 = *tmp.offset((x_0 - off) as isize) as libc::c_int;
                        sum_0 += pri_tap_k_0 * constrain(p0_0 - px_0, pri_strength, pri_shift);
                        sum_0 += pri_tap_k_0 * constrain(p1_0 - px_0, pri_strength, pri_shift);
                        pri_tap_k_0 = pri_tap_k_0 & 3 | 2;
                        k_0 += 1;
                    }
                    *dst.offset(x_0 as isize) =
                        (px_0 + (sum_0 - (sum_0 < 0) as libc::c_int + 8 >> 4)) as pixel;
                    x_0 += 1;
                }
                dst = dst.offset(PXSTRIDE(dst_stride) as isize);
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
        let sec_shift_0 = damping - ulog2(sec_strength as libc::c_uint);
        loop {
            let mut x_1 = 0;
            while x_1 < w {
                let px_1 = *dst.offset(x_1 as isize) as libc::c_int;
                let mut sum_1 = 0;
                let mut k_1 = 0;
                while k_1 < 2 {
                    let off1_0 =
                        dav1d_cdef_directions[(dir + 4) as usize][k_1 as usize] as libc::c_int;
                    let off2_0 =
                        dav1d_cdef_directions[(dir + 0) as usize][k_1 as usize] as libc::c_int;
                    let s0_0 = *tmp.offset((x_1 + off1_0) as isize) as libc::c_int;
                    let s1_0 = *tmp.offset((x_1 - off1_0) as isize) as libc::c_int;
                    let s2_0 = *tmp.offset((x_1 + off2_0) as isize) as libc::c_int;
                    let s3_0 = *tmp.offset((x_1 - off2_0) as isize) as libc::c_int;
                    let sec_tap_0 = 2 - k_1;
                    sum_1 += sec_tap_0 * constrain(s0_0 - px_1, sec_strength, sec_shift_0);
                    sum_1 += sec_tap_0 * constrain(s1_0 - px_1, sec_strength, sec_shift_0);
                    sum_1 += sec_tap_0 * constrain(s2_0 - px_1, sec_strength, sec_shift_0);
                    sum_1 += sec_tap_0 * constrain(s3_0 - px_1, sec_strength, sec_shift_0);
                    k_1 += 1;
                }
                *dst.offset(x_1 as isize) =
                    (px_1 + (sum_1 - (sum_1 < 0) as libc::c_int + 8 >> 4)) as pixel;
                x_1 += 1;
            }
            dst = dst.offset(PXSTRIDE(dst_stride) as isize);
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
    bitdepth_max: libc::c_int,
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
        bitdepth_max,
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
    bitdepth_max: libc::c_int,
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
        bitdepth_max,
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
    bitdepth_max: libc::c_int,
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
        bitdepth_max,
    );
}
unsafe extern "C" fn cdef_find_dir_c(
    mut img: *const pixel,
    stride: ptrdiff_t,
    var: *mut libc::c_uint,
    bitdepth_max: libc::c_int,
) -> libc::c_int {
    let bitdepth_min_8 = 32 as libc::c_int - clz(bitdepth_max as libc::c_uint) - 8;
    let mut partial_sum_hv: [[libc::c_int; 8]; 2] =
        [[0 as libc::c_int, 0, 0, 0, 0, 0, 0, 0], [0; 8]];
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
    let mut y = 0;
    while y < 8 {
        let mut x = 0;
        while x < 8 {
            let px = (*img.offset(x as isize) as libc::c_int >> bitdepth_min_8) - 128;
            partial_sum_diag[0][(y + x) as usize] += px;
            partial_sum_alt[0][(y + (x >> 1)) as usize] += px;
            partial_sum_hv[0][y as usize] += px;
            partial_sum_alt[1][(3 + y - (x >> 1)) as usize] += px;
            partial_sum_diag[1][(7 + y - x) as usize] += px;
            partial_sum_alt[2][(3 - (y >> 1) + x) as usize] += px;
            partial_sum_hv[1][x as usize] += px;
            partial_sum_alt[3][((y >> 1) + x) as usize] += px;
            x += 1;
        }
        img = img.offset(PXSTRIDE(stride) as isize);
        y += 1;
    }
    let mut cost: [libc::c_uint; 8] = [0 as libc::c_int as libc::c_uint, 0, 0, 0, 0, 0, 0, 0];
    let mut n = 0;
    while n < 8 {
        cost[2] = (cost[2]).wrapping_add(
            (partial_sum_hv[0][n as usize] * partial_sum_hv[0][n as usize]) as libc::c_uint,
        );
        cost[6] = (cost[6]).wrapping_add(
            (partial_sum_hv[1][n as usize] * partial_sum_hv[1][n as usize]) as libc::c_uint,
        );
        n += 1;
    }
    cost[2] = (cost[2]).wrapping_mul(105 as libc::c_int as libc::c_uint);
    cost[6] = (cost[6]).wrapping_mul(105 as libc::c_int as libc::c_uint);
    static mut div_table: [uint16_t; 7] = [
        840 as libc::c_int as uint16_t,
        420 as libc::c_int as uint16_t,
        280 as libc::c_int as uint16_t,
        210 as libc::c_int as uint16_t,
        168 as libc::c_int as uint16_t,
        140 as libc::c_int as uint16_t,
        120 as libc::c_int as uint16_t,
    ];
    let mut n_0 = 0;
    while n_0 < 7 {
        let d = div_table[n_0 as usize] as libc::c_int;
        cost[0] = (cost[0]).wrapping_add(
            ((partial_sum_diag[0][n_0 as usize] * partial_sum_diag[0][n_0 as usize]
                + partial_sum_diag[0][(14 - n_0) as usize]
                    * partial_sum_diag[0][(14 - n_0) as usize])
                * d) as libc::c_uint,
        );
        cost[4] = (cost[4]).wrapping_add(
            ((partial_sum_diag[1][n_0 as usize] * partial_sum_diag[1][n_0 as usize]
                + partial_sum_diag[1][(14 - n_0) as usize]
                    * partial_sum_diag[1][(14 - n_0) as usize])
                * d) as libc::c_uint,
        );
        n_0 += 1;
    }
    cost[0] = (cost[0])
        .wrapping_add((partial_sum_diag[0][7] * partial_sum_diag[0][7] * 105) as libc::c_uint);
    cost[4] = (cost[4])
        .wrapping_add((partial_sum_diag[1][7] * partial_sum_diag[1][7] * 105) as libc::c_uint);
    let mut n_1 = 0;
    while n_1 < 4 {
        let cost_ptr: *mut libc::c_uint =
            &mut *cost.as_mut_ptr().offset((n_1 * 2 + 1) as isize) as *mut libc::c_uint;
        let mut m = 0;
        while m < 5 {
            *cost_ptr = (*cost_ptr).wrapping_add(
                (partial_sum_alt[n_1 as usize][(3 + m) as usize]
                    * partial_sum_alt[n_1 as usize][(3 + m) as usize])
                    as libc::c_uint,
            );
            m += 1;
        }
        *cost_ptr = (*cost_ptr).wrapping_mul(105 as libc::c_int as libc::c_uint);
        let mut m_0 = 0;
        while m_0 < 3 {
            let d_0 = div_table[(2 * m_0 + 1) as usize] as libc::c_int;
            *cost_ptr = (*cost_ptr).wrapping_add(
                ((partial_sum_alt[n_1 as usize][m_0 as usize]
                    * partial_sum_alt[n_1 as usize][m_0 as usize]
                    + partial_sum_alt[n_1 as usize][(10 - m_0) as usize]
                        * partial_sum_alt[n_1 as usize][(10 - m_0) as usize])
                    * d_0) as libc::c_uint,
            );
            m_0 += 1;
        }
        n_1 += 1;
    }
    let mut best_dir = 0;
    let mut best_cost: libc::c_uint = cost[0];
    let mut n_2 = 1;
    while n_2 < 8 {
        if cost[n_2 as usize] > best_cost {
            best_cost = cost[n_2 as usize];
            best_dir = n_2;
        }
        n_2 += 1;
    }
    *var = best_cost.wrapping_sub(cost[(best_dir ^ 4 as libc::c_int) as usize]) >> 10;
    return best_dir;
}

#[inline(always)]
#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64"),))]
unsafe extern "C" fn cdef_dsp_init_x86(c: *mut Dav1dCdefDSPContext) {
    use crate::src::x86::cpu::*;

    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_X86_CPU_FLAG_SSSE3 == 0 {
        return;
    }

    (*c).dir = Some(dav1d_cdef_dir_16bpc_ssse3);
    (*c).fb[0] = Some(dav1d_cdef_filter_8x8_16bpc_ssse3);
    (*c).fb[1] = Some(dav1d_cdef_filter_4x8_16bpc_ssse3);
    (*c).fb[2] = Some(dav1d_cdef_filter_4x4_16bpc_ssse3);

    if flags & DAV1D_X86_CPU_FLAG_SSE41 == 0 {
        return;
    }

    (*c).dir = Some(dav1d_cdef_dir_16bpc_sse4);

    #[cfg(target_arch = "x86_64")]
    {
        if flags & DAV1D_X86_CPU_FLAG_AVX2 == 0 {
            return;
        }

        (*c).dir = Some(dav1d_cdef_dir_16bpc_avx2);
        (*c).fb[0] = Some(dav1d_cdef_filter_8x8_16bpc_avx2);
        (*c).fb[1] = Some(dav1d_cdef_filter_4x8_16bpc_avx2);
        (*c).fb[2] = Some(dav1d_cdef_filter_4x4_16bpc_avx2);

        if flags & DAV1D_X86_CPU_FLAG_AVX512ICL == 0 {
            return;
        }

        (*c).fb[0] = Some(dav1d_cdef_filter_8x8_16bpc_avx512icl);
        (*c).fb[1] = Some(dav1d_cdef_filter_4x8_16bpc_avx512icl);
        (*c).fb[2] = Some(dav1d_cdef_filter_4x4_16bpc_avx512icl);
    }
}

#[inline(always)]
#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
unsafe extern "C" fn cdef_dsp_init_arm(c: *mut Dav1dCdefDSPContext) {
    use crate::src::arm::cpu::DAV1D_ARM_CPU_FLAG_NEON;

    let flags: libc::c_uint = dav1d_get_cpu_flags();

    if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
        return;
    }

    (*c).dir = Some(dav1d_cdef_find_dir_16bpc_neon);
    (*c).fb[0] = Some(cdef_filter_8x8_neon);
    (*c).fb[1] = Some(cdef_filter_4x8_neon);
    (*c).fb[2] = Some(cdef_filter_4x4_neon);
}

#[inline(always)]
#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
unsafe extern "C" fn cdef_filter_8x8_neon(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    mut left: *const [pixel; 2],
    top: *const pixel,
    bottom: *const pixel,
    pri_strength: libc::c_int,
    sec_strength: libc::c_int,
    dir: libc::c_int,
    damping: libc::c_int,
    edges: CdefEdgeFlags,
    bitdepth_max: libc::c_int,
) {
    let mut tmp_buf = [0; 200];
    let mut tmp = tmp_buf.as_mut_ptr().offset(2 * 16).offset(8);
    dav1d_cdef_padding8_16bpc_neon(tmp, dst, stride, left, top, bottom, 8, edges);
    dav1d_cdef_filter8_16bpc_neon(
        dst,
        stride,
        tmp,
        pri_strength,
        sec_strength,
        dir,
        damping,
        8,
        edges as size_t,
        bitdepth_max,
    );
}

#[inline(always)]
#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
unsafe extern "C" fn cdef_filter_4x8_neon(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    mut left: *const [pixel; 2],
    top: *const pixel,
    bottom: *const pixel,
    pri_strength: libc::c_int,
    sec_strength: libc::c_int,
    dir: libc::c_int,
    damping: libc::c_int,
    edges: CdefEdgeFlags,
    bitdepth_max: libc::c_int,
) {
    let mut tmp_buf: [uint16_t; 104] = [0; 104];
    let mut tmp = tmp_buf.as_mut_ptr().offset(2 * 8).offset(8);
    dav1d_cdef_padding4_16bpc_neon(tmp, dst, stride, left, top, bottom, 8, edges);
    dav1d_cdef_filter4_16bpc_neon(
        dst,
        stride,
        tmp,
        pri_strength,
        sec_strength,
        dir,
        damping,
        8,
        edges as size_t,
        bitdepth_max,
    );
}

#[inline(always)]
#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
unsafe extern "C" fn cdef_filter_4x4_neon(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    mut left: *const [pixel; 2],
    top: *const pixel,
    bottom: *const pixel,
    pri_strength: libc::c_int,
    sec_strength: libc::c_int,
    dir: libc::c_int,
    damping: libc::c_int,
    edges: CdefEdgeFlags,
    bitdepth_max: libc::c_int,
) {
    let mut tmp_buf = [0; 104];
    let mut tmp = tmp_buf.as_mut_ptr().offset(2 * 8).offset(8);
    dav1d_cdef_padding4_16bpc_neon(tmp, dst, stride, left, top, bottom, 4, edges);
    dav1d_cdef_filter4_16bpc_neon(
        dst,
        stride,
        tmp,
        pri_strength,
        sec_strength,
        dir,
        damping,
        4,
        edges as size_t,
        bitdepth_max,
    );
}

#[cfg(feature = "asm")]
use crate::src::cpu::dav1d_get_cpu_flags;

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_cdef_dsp_init_16bpc(c: *mut Dav1dCdefDSPContext) {
    (*c).dir = Some(cdef_find_dir_c);
    (*c).fb[0] = Some(cdef_filter_block_8x8_c);
    (*c).fb[1] = Some(cdef_filter_block_4x8_c);
    (*c).fb[2] = Some(cdef_filter_block_4x4_c);

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            cdef_dsp_init_x86(c);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
            cdef_dsp_init_arm(c);
        }
    }
}
