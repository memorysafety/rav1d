use crate::include::common::bitdepth::BitDepth8;
use crate::include::stddef::*;
use crate::include::stdint::*;
#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
use crate::src::align::Align16;
use ::libc;
#[cfg(feature = "asm")]
use cfg_if::cfg_if;

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
extern "C" {
    fn dav1d_sgr_box5_h_8bpc_neon(
        sumsq: *mut int32_t,
        sum: *mut int16_t,
        left: *const [pixel; 4],
        src: *const pixel,
        stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        edges: LrEdgeFlags,
    );
    fn dav1d_sgr_box5_v_neon(
        sumsq: *mut int32_t,
        sum: *mut int16_t,
        w: libc::c_int,
        h: libc::c_int,
        edges: LrEdgeFlags,
    );
    fn dav1d_sgr_calc_ab2_neon(
        a: *mut int32_t,
        b: *mut int16_t,
        w: libc::c_int,
        h: libc::c_int,
        strength: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_sgr_finish_filter2_8bpc_neon(
        tmp: *mut int16_t,
        src: *const pixel,
        stride: ptrdiff_t,
        a: *const int32_t,
        b: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_sgr_box3_v_neon(
        sumsq: *mut int32_t,
        sum: *mut int16_t,
        w: libc::c_int,
        h: libc::c_int,
        edges: LrEdgeFlags,
    );
    fn dav1d_sgr_calc_ab1_neon(
        a: *mut int32_t,
        b: *mut int16_t,
        w: libc::c_int,
        h: libc::c_int,
        strength: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_sgr_finish_filter1_8bpc_neon(
        tmp: *mut int16_t,
        src: *const pixel,
        stride: ptrdiff_t,
        a: *const int32_t,
        b: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_sgr_weighted1_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        t1: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        wt: libc::c_int,
    );
    fn dav1d_sgr_weighted2_8bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        t1: *const int16_t,
        t2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        wt: *const int16_t,
    );
    fn dav1d_sgr_box3_h_8bpc_neon(
        sumsq: *mut int32_t,
        sum: *mut int16_t,
        left: *const [pixel; 4],
        src: *const pixel,
        stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        edges: LrEdgeFlags,
    );
}

#[cfg(all(feature = "asm", target_arch = "arm"))]
extern "C" {
    fn dav1d_wiener_filter_h_8bpc_neon(
        dst: *mut int16_t,
        left: *const [pixel; 4],
        src: *const pixel,
        stride: ptrdiff_t,
        fh: *const int16_t,
        w: intptr_t,
        h: libc::c_int,
        edges: LrEdgeFlags,
    );
    fn dav1d_wiener_filter_v_8bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        mid: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        fv: *const int16_t,
        edges: LrEdgeFlags,
        mid_stride: ptrdiff_t,
    );
}

use crate::src::tables::dav1d_sgr_x_by_x;

pub type pixel = uint8_t;
pub type coef = int16_t;
use crate::src::looprestoration::LrEdgeFlags;
pub type const_left_pixel_row = *const [pixel; 4];
use crate::src::looprestoration::padding;
use crate::src::looprestoration::wiener_c_erased;
use crate::src::looprestoration::Dav1dLoopRestorationDSPContext;
use crate::src::looprestoration::LooprestorationParams;

use crate::include::common::intops::iclip_u8;
use crate::include::common::intops::imax;
use crate::include::common::intops::umin;

unsafe extern "C" fn boxsum3(
    mut sumsq: *mut int32_t,
    mut sum: *mut coef,
    mut src: *const pixel,
    w: libc::c_int,
    h: libc::c_int,
) {
    src = src.offset(390);
    let mut x = 1;
    while x < w - 1 {
        let mut sum_v: *mut coef = sum.offset(x as isize);
        let mut sumsq_v: *mut int32_t = sumsq.offset(x as isize);
        let mut s: *const pixel = src.offset(x as isize);
        let mut a = *s.offset(0) as libc::c_int;
        let mut a2 = a * a;
        let mut b = *s.offset(390) as libc::c_int;
        let mut b2 = b * b;
        let mut y = 2;
        while y < h - 2 {
            s = s.offset(390);
            let c = *s.offset(390) as libc::c_int;
            let c2 = c * c;
            sum_v = sum_v.offset(390);
            sumsq_v = sumsq_v.offset(390);
            *sum_v = (a + b + c) as coef;
            *sumsq_v = a2 + b2 + c2;
            a = b;
            a2 = b2;
            b = c;
            b2 = c2;
            y += 1;
        }
        x += 1;
    }
    sum = sum.offset(390);
    sumsq = sumsq.offset(390);
    let mut y_0 = 2;
    while y_0 < h - 2 {
        let mut a_0 = *sum.offset(1) as libc::c_int;
        let mut a2_0 = *sumsq.offset(1);
        let mut b_0 = *sum.offset(2) as libc::c_int;
        let mut b2_0 = *sumsq.offset(2);
        let mut x_0 = 2;
        while x_0 < w - 2 {
            let c_0 = *sum.offset((x_0 + 1) as isize) as libc::c_int;
            let c2_0 = *sumsq.offset((x_0 + 1) as isize);
            *sum.offset(x_0 as isize) = (a_0 + b_0 + c_0) as coef;
            *sumsq.offset(x_0 as isize) = a2_0 + b2_0 + c2_0;
            a_0 = b_0;
            a2_0 = b2_0;
            b_0 = c_0;
            b2_0 = c2_0;
            x_0 += 1;
        }
        sum = sum.offset(390);
        sumsq = sumsq.offset(390);
        y_0 += 1;
    }
}
unsafe extern "C" fn boxsum5(
    mut sumsq: *mut int32_t,
    mut sum: *mut coef,
    src: *const pixel,
    w: libc::c_int,
    h: libc::c_int,
) {
    let mut x = 0;
    while x < w {
        let mut sum_v: *mut coef = sum.offset(x as isize);
        let mut sumsq_v: *mut int32_t = sumsq.offset(x as isize);
        let mut s: *const pixel = src.offset((3 * 390) as isize).offset(x as isize);
        let mut a = *s.offset((-(3 as libc::c_int) * 390) as isize) as libc::c_int;
        let mut a2 = a * a;
        let mut b = *s.offset((-(2 as libc::c_int) * 390) as isize) as libc::c_int;
        let mut b2 = b * b;
        let mut c = *s.offset((-(1 as libc::c_int) * 390) as isize) as libc::c_int;
        let mut c2 = c * c;
        let mut d = *s.offset(0) as libc::c_int;
        let mut d2 = d * d;
        let mut y = 2;
        while y < h - 2 {
            s = s.offset(390);
            let e = *s as libc::c_int;
            let e2 = e * e;
            sum_v = sum_v.offset(390);
            sumsq_v = sumsq_v.offset(390);
            *sum_v = (a + b + c + d + e) as coef;
            *sumsq_v = a2 + b2 + c2 + d2 + e2;
            a = b;
            b = c;
            c = d;
            d = e;
            a2 = b2;
            b2 = c2;
            c2 = d2;
            d2 = e2;
            y += 1;
        }
        x += 1;
    }
    sum = sum.offset(390);
    sumsq = sumsq.offset(390);
    let mut y_0 = 2;
    while y_0 < h - 2 {
        let mut a_0 = *sum.offset(0) as libc::c_int;
        let mut a2_0 = *sumsq.offset(0);
        let mut b_0 = *sum.offset(1) as libc::c_int;
        let mut b2_0 = *sumsq.offset(1);
        let mut c_0 = *sum.offset(2) as libc::c_int;
        let mut c2_0 = *sumsq.offset(2);
        let mut d_0 = *sum.offset(3) as libc::c_int;
        let mut d2_0 = *sumsq.offset(3);
        let mut x_0 = 2;
        while x_0 < w - 2 {
            let e_0 = *sum.offset((x_0 + 2) as isize) as libc::c_int;
            let e2_0 = *sumsq.offset((x_0 + 2) as isize);
            *sum.offset(x_0 as isize) = (a_0 + b_0 + c_0 + d_0 + e_0) as coef;
            *sumsq.offset(x_0 as isize) = a2_0 + b2_0 + c2_0 + d2_0 + e2_0;
            a_0 = b_0;
            b_0 = c_0;
            c_0 = d_0;
            d_0 = e_0;
            a2_0 = b2_0;
            b2_0 = c2_0;
            c2_0 = d2_0;
            d2_0 = e2_0;
            x_0 += 1;
        }
        sum = sum.offset(390);
        sumsq = sumsq.offset(390);
        y_0 += 1;
    }
}
#[inline(never)]
unsafe extern "C" fn selfguided_filter(
    mut dst: *mut coef,
    mut src: *const pixel,
    _src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    n: libc::c_int,
    s: libc::c_uint,
) {
    let sgr_one_by_x: libc::c_uint = (if n == 25 {
        164 as libc::c_int
    } else {
        455 as libc::c_int
    }) as libc::c_uint;
    let mut sumsq: [int32_t; 26520] = [0; 26520];
    let mut A: *mut int32_t = sumsq.as_mut_ptr().offset((2 * 390) as isize).offset(3);
    let mut sum: [coef; 26520] = [0; 26520];
    let mut B: *mut coef = sum.as_mut_ptr().offset((2 * 390) as isize).offset(3);
    let step = (n == 25) as libc::c_int + 1;
    if n == 25 {
        boxsum5(sumsq.as_mut_ptr(), sum.as_mut_ptr(), src, w + 6, h + 6);
    } else {
        boxsum3(sumsq.as_mut_ptr(), sum.as_mut_ptr(), src, w + 6, h + 6);
    }
    let bitdepth_min_8 = 8 - 8;
    let mut AA: *mut int32_t = A.offset(-(390 as libc::c_int as isize));
    let mut BB: *mut coef = B.offset(-(390 as libc::c_int as isize));
    let mut j = -(1 as libc::c_int);
    while j < h + 1 {
        let mut i = -(1 as libc::c_int);
        while i < w + 1 {
            let a = *AA.offset(i as isize) + ((1 as libc::c_int) << 2 * bitdepth_min_8 >> 1)
                >> 2 * bitdepth_min_8;
            let b = *BB.offset(i as isize) as libc::c_int
                + ((1 as libc::c_int) << bitdepth_min_8 >> 1)
                >> bitdepth_min_8;
            let p: libc::c_uint = imax(a * n - b * b, 0 as libc::c_int) as libc::c_uint;
            let z: libc::c_uint = p
                .wrapping_mul(s)
                .wrapping_add(((1 as libc::c_int) << 19) as libc::c_uint)
                >> 20;
            let x: libc::c_uint = dav1d_sgr_x_by_x
                [umin(z, 255 as libc::c_int as libc::c_uint) as usize]
                as libc::c_uint;
            *AA.offset(i as isize) = (x
                .wrapping_mul(*BB.offset(i as isize) as libc::c_uint)
                .wrapping_mul(sgr_one_by_x)
                .wrapping_add(((1 as libc::c_int) << 11) as libc::c_uint)
                >> 12) as int32_t;
            *BB.offset(i as isize) = x as coef;
            i += 1;
        }
        AA = AA.offset((step * 390) as isize);
        BB = BB.offset((step * 390) as isize);
        j += step;
    }
    src = src.offset((3 * 390 + 3) as isize);
    if n == 25 {
        let mut j_0 = 0;
        while j_0 < h - 1 {
            let mut i_0 = 0;
            while i_0 < w {
                let a_0 = (*B.offset((i_0 - 390) as isize) as libc::c_int
                    + *B.offset((i_0 + 390) as isize) as libc::c_int)
                    * 6
                    + (*B.offset((i_0 - 1 - 390) as isize) as libc::c_int
                        + *B.offset((i_0 - 1 + 390) as isize) as libc::c_int
                        + *B.offset((i_0 + 1 - 390) as isize) as libc::c_int
                        + *B.offset((i_0 + 1 + 390) as isize) as libc::c_int)
                        * 5;
                let b_0 = (*A.offset((i_0 - 390) as isize) + *A.offset((i_0 + 390) as isize)) * 6
                    + (*A.offset((i_0 - 1 - 390) as isize)
                        + *A.offset((i_0 - 1 + 390) as isize)
                        + *A.offset((i_0 + 1 - 390) as isize)
                        + *A.offset((i_0 + 1 + 390) as isize))
                        * 5;
                *dst.offset(i_0 as isize) = (b_0 - a_0 * *src.offset(i_0 as isize) as libc::c_int
                    + ((1 as libc::c_int) << 8)
                    >> 9) as coef;
                i_0 += 1;
            }
            dst = dst.offset(384);
            src = src.offset(390);
            B = B.offset(390);
            A = A.offset(390);
            let mut i_1 = 0;
            while i_1 < w {
                let a_1 = *B.offset(i_1 as isize) as libc::c_int * 6
                    + (*B.offset((i_1 - 1) as isize) as libc::c_int
                        + *B.offset((i_1 + 1) as isize) as libc::c_int)
                        * 5;
                let b_1 = *A.offset(i_1 as isize) * 6
                    + (*A.offset((i_1 - 1) as isize) + *A.offset((i_1 + 1) as isize)) * 5;
                *dst.offset(i_1 as isize) = (b_1 - a_1 * *src.offset(i_1 as isize) as libc::c_int
                    + ((1 as libc::c_int) << 7)
                    >> 8) as coef;
                i_1 += 1;
            }
            dst = dst.offset(384);
            src = src.offset(390);
            B = B.offset(390);
            A = A.offset(390);
            j_0 += 2 as libc::c_int;
        }
        if j_0 + 1 == h {
            let mut i_2 = 0;
            while i_2 < w {
                let a_2 = (*B.offset((i_2 - 390) as isize) as libc::c_int
                    + *B.offset((i_2 + 390) as isize) as libc::c_int)
                    * 6
                    + (*B.offset((i_2 - 1 - 390) as isize) as libc::c_int
                        + *B.offset((i_2 - 1 + 390) as isize) as libc::c_int
                        + *B.offset((i_2 + 1 - 390) as isize) as libc::c_int
                        + *B.offset((i_2 + 1 + 390) as isize) as libc::c_int)
                        * 5;
                let b_2 = (*A.offset((i_2 - 390) as isize) + *A.offset((i_2 + 390) as isize)) * 6
                    + (*A.offset((i_2 - 1 - 390) as isize)
                        + *A.offset((i_2 - 1 + 390) as isize)
                        + *A.offset((i_2 + 1 - 390) as isize)
                        + *A.offset((i_2 + 1 + 390) as isize))
                        * 5;
                *dst.offset(i_2 as isize) = (b_2 - a_2 * *src.offset(i_2 as isize) as libc::c_int
                    + ((1 as libc::c_int) << 8)
                    >> 9) as coef;
                i_2 += 1;
            }
        }
    } else {
        let mut j_1 = 0;
        while j_1 < h {
            let mut i_3 = 0;
            while i_3 < w {
                let a_3 = (*B.offset(i_3 as isize) as libc::c_int
                    + *B.offset((i_3 - 1) as isize) as libc::c_int
                    + *B.offset((i_3 + 1) as isize) as libc::c_int
                    + *B.offset((i_3 - 390) as isize) as libc::c_int
                    + *B.offset((i_3 + 390) as isize) as libc::c_int)
                    * 4
                    + (*B.offset((i_3 - 1 - 390) as isize) as libc::c_int
                        + *B.offset((i_3 - 1 + 390) as isize) as libc::c_int
                        + *B.offset((i_3 + 1 - 390) as isize) as libc::c_int
                        + *B.offset((i_3 + 1 + 390) as isize) as libc::c_int)
                        * 3;
                let b_3 = (*A.offset(i_3 as isize)
                    + *A.offset((i_3 - 1) as isize)
                    + *A.offset((i_3 + 1) as isize)
                    + *A.offset((i_3 - 390) as isize)
                    + *A.offset((i_3 + 390) as isize))
                    * 4
                    + (*A.offset((i_3 - 1 - 390) as isize)
                        + *A.offset((i_3 - 1 + 390) as isize)
                        + *A.offset((i_3 + 1 - 390) as isize)
                        + *A.offset((i_3 + 1 + 390) as isize))
                        * 3;
                *dst.offset(i_3 as isize) = (b_3 - a_3 * *src.offset(i_3 as isize) as libc::c_int
                    + ((1 as libc::c_int) << 8)
                    >> 9) as coef;
                i_3 += 1;
            }
            dst = dst.offset(384);
            src = src.offset(390);
            B = B.offset(390);
            A = A.offset(390);
            j_1 += 1;
        }
    };
}

unsafe extern "C" fn sgr_5x5_c(
    mut p: *mut libc::c_void,
    stride: ptrdiff_t,
    left: *const libc::c_void,
    mut lpf: *const libc::c_void,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    _bitdepth_max: libc::c_int,
) {
    sgr_5x5_rust(
        p.cast(),
        stride,
        left.cast(),
        lpf.cast(),
        w,
        h,
        params,
        edges,
    )
}

unsafe extern "C" fn sgr_5x5_rust(
    mut p: *mut pixel,
    stride: ptrdiff_t,
    left: *const [pixel; 4],
    mut lpf: *const pixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
) {
    let mut tmp: [pixel; 27300] = [0; 27300];
    let mut dst: [coef; 24576] = [0; 24576];
    padding::<BitDepth8>(&mut tmp, p, stride, left, lpf, w, h, edges);
    selfguided_filter(
        dst.as_mut_ptr(),
        tmp.as_mut_ptr(),
        390 as libc::c_int as ptrdiff_t,
        w,
        h,
        25 as libc::c_int,
        (*params).sgr.s0,
    );
    let w0 = (*params).sgr.w0 as libc::c_int;
    let mut j = 0;
    while j < h {
        let mut i = 0;
        while i < w {
            let v = w0 * dst[(j * 384 + i) as usize] as libc::c_int;
            *p.offset(i as isize) = iclip_u8(
                *p.offset(i as isize) as libc::c_int + (v + ((1 as libc::c_int) << 10) >> 11),
            ) as pixel;
            i += 1;
        }
        p = p.offset(stride as isize);
        j += 1;
    }
}

unsafe extern "C" fn sgr_3x3_c(
    mut p: *mut libc::c_void,
    stride: ptrdiff_t,
    left: *const libc::c_void,
    mut lpf: *const libc::c_void,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    _bitdepth_max: libc::c_int,
) {
    sgr_3x3_rust(
        p.cast(),
        stride,
        left.cast(),
        lpf.cast(),
        w,
        h,
        params,
        edges,
    )
}

unsafe extern "C" fn sgr_3x3_rust(
    mut p: *mut pixel,
    stride: ptrdiff_t,
    left: *const [pixel; 4],
    mut lpf: *const pixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
) {
    let mut tmp: [pixel; 27300] = [0; 27300];
    let mut dst: [coef; 24576] = [0; 24576];
    padding::<BitDepth8>(&mut tmp, p, stride, left, lpf, w, h, edges);
    selfguided_filter(
        dst.as_mut_ptr(),
        tmp.as_mut_ptr(),
        390 as libc::c_int as ptrdiff_t,
        w,
        h,
        9 as libc::c_int,
        (*params).sgr.s1,
    );
    let w1 = (*params).sgr.w1 as libc::c_int;
    let mut j = 0;
    while j < h {
        let mut i = 0;
        while i < w {
            let v = w1 * dst[(j * 384 + i) as usize] as libc::c_int;
            *p.offset(i as isize) = iclip_u8(
                *p.offset(i as isize) as libc::c_int + (v + ((1 as libc::c_int) << 10) >> 11),
            ) as pixel;
            i += 1;
        }
        p = p.offset(stride as isize);
        j += 1;
    }
}

unsafe extern "C" fn sgr_mix_c(
    mut p: *mut libc::c_void,
    stride: ptrdiff_t,
    left: *const libc::c_void,
    mut lpf: *const libc::c_void,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    _bitdepth_max: libc::c_int,
) {
    sgr_mix_rust(
        p.cast(),
        stride,
        left.cast(),
        lpf.cast(),
        w,
        h,
        params,
        edges,
    )
}

unsafe extern "C" fn sgr_mix_rust(
    mut p: *mut pixel,
    stride: ptrdiff_t,
    left: *const [pixel; 4],
    mut lpf: *const pixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
) {
    let mut tmp: [pixel; 27300] = [0; 27300];
    let mut dst0: [coef; 24576] = [0; 24576];
    let mut dst1: [coef; 24576] = [0; 24576];
    padding::<BitDepth8>(&mut tmp, p, stride, left, lpf, w, h, edges);
    selfguided_filter(
        dst0.as_mut_ptr(),
        tmp.as_mut_ptr(),
        390 as libc::c_int as ptrdiff_t,
        w,
        h,
        25 as libc::c_int,
        (*params).sgr.s0,
    );
    selfguided_filter(
        dst1.as_mut_ptr(),
        tmp.as_mut_ptr(),
        390 as libc::c_int as ptrdiff_t,
        w,
        h,
        9 as libc::c_int,
        (*params).sgr.s1,
    );
    let w0 = (*params).sgr.w0 as libc::c_int;
    let w1 = (*params).sgr.w1 as libc::c_int;
    let mut j = 0;
    while j < h {
        let mut i = 0;
        while i < w {
            let v = w0 * dst0[(j * 384 + i) as usize] as libc::c_int
                + w1 * dst1[(j * 384 + i) as usize] as libc::c_int;
            *p.offset(i as isize) = iclip_u8(
                *p.offset(i as isize) as libc::c_int + (v + ((1 as libc::c_int) << 10) >> 11),
            ) as pixel;
            i += 1;
        }
        p = p.offset(stride as isize);
        j += 1;
    }
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
#[inline(always)]
unsafe extern "C" fn loop_restoration_dsp_init_x86(
    c: *mut Dav1dLoopRestorationDSPContext,
    _bpc: libc::c_int,
) {
    // TODO(randomPoison): Import temporarily needed until init fns are deduplicated.
    use crate::src::looprestoration::*;
    use crate::src::x86::cpu::*;

    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_X86_CPU_FLAG_SSE2 == 0 {
        return;
    }

    (*c).wiener[0] = dav1d_wiener_filter7_8bpc_sse2;
    (*c).wiener[1] = dav1d_wiener_filter5_8bpc_sse2;

    if flags & DAV1D_X86_CPU_FLAG_SSSE3 == 0 {
        return;
    }

    (*c).wiener[0] = dav1d_wiener_filter7_8bpc_ssse3;
    (*c).wiener[1] = dav1d_wiener_filter5_8bpc_ssse3;

    (*c).sgr[0] = dav1d_sgr_filter_5x5_8bpc_ssse3;
    (*c).sgr[1] = dav1d_sgr_filter_3x3_8bpc_ssse3;
    (*c).sgr[2] = dav1d_sgr_filter_mix_8bpc_ssse3;

    #[cfg(target_arch = "x86_64")]
    {
        if flags & DAV1D_X86_CPU_FLAG_AVX2 == 0 {
            return;
        }

        (*c).wiener[0] = dav1d_wiener_filter7_8bpc_avx2;
        (*c).wiener[1] = dav1d_wiener_filter5_8bpc_avx2;

        (*c).sgr[0] = dav1d_sgr_filter_5x5_8bpc_avx2;
        (*c).sgr[1] = dav1d_sgr_filter_3x3_8bpc_avx2;
        (*c).sgr[2] = dav1d_sgr_filter_mix_8bpc_avx2;

        if flags & DAV1D_X86_CPU_FLAG_AVX512ICL == 0 {
            return;
        }

        (*c).wiener[0] = dav1d_wiener_filter7_8bpc_avx512icl;
        (*c).wiener[1] = (*c).wiener[0];

        (*c).sgr[0] = dav1d_sgr_filter_5x5_8bpc_avx512icl;
        (*c).sgr[1] = dav1d_sgr_filter_3x3_8bpc_avx512icl;
        (*c).sgr[2] = dav1d_sgr_filter_mix_8bpc_avx512icl;
    }
}

#[cfg(all(feature = "asm", target_arch = "arm"))]
unsafe extern "C" fn wiener_filter_neon_erased(
    p: *mut libc::c_void,
    stride: ptrdiff_t,
    left: *const libc::c_void,
    lpf: *const libc::c_void,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    _bitdepth_max: libc::c_int,
) {
    wiener_filter_neon(
        p.cast(),
        stride,
        left.cast(),
        lpf.cast(),
        w,
        h,
        params,
        edges,
    )
}

#[cfg(all(feature = "asm", target_arch = "arm"))]
unsafe extern "C" fn wiener_filter_neon(
    dst: *mut pixel,
    stride: ptrdiff_t,
    left: *const [pixel; 4],
    mut lpf: *const pixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
) {
    use crate::src::looprestoration::LR_HAVE_BOTTOM;
    use crate::src::looprestoration::LR_HAVE_TOP;

    let filter: *const [int16_t; 8] = (*params).filter.0.as_ptr();
    let mut mid: Align16<[int16_t; 68 * 384]> = Align16([0; 68 * 384]);
    let mut mid_stride: libc::c_int = w + 7 & !7;
    // Horizontal filter
    dav1d_wiener_filter_h_8bpc_neon(
        &mut *mid.0.as_mut_ptr().offset((2 * mid_stride) as isize),
        left,
        dst,
        stride,
        (*filter.offset(0)).as_ptr(),
        w as isize,
        h,
        edges,
    );
    if edges & LR_HAVE_TOP != 0 {
        dav1d_wiener_filter_h_8bpc_neon(
            mid.0.as_mut_ptr(),
            core::ptr::null(),
            lpf,
            stride,
            (*filter.offset(0)).as_ptr(),
            w as isize,
            2,
            edges,
        );
    }
    if edges & LR_HAVE_BOTTOM != 0 {
        dav1d_wiener_filter_h_8bpc_neon(
            &mut *mid
                .0
                .as_mut_ptr()
                .offset(((2 as libc::c_int + h) * mid_stride) as isize),
            core::ptr::null(),
            lpf.offset((6 * stride) as isize),
            stride,
            (*filter.offset(0)).as_ptr(),
            w as isize,
            2,
            edges,
        );
    }
    // Vertical filter
    dav1d_wiener_filter_v_8bpc_neon(
        dst,
        stride,
        &mut *mid.0.as_mut_ptr().offset((2 * mid_stride) as isize),
        w,
        h,
        (*filter.offset(1)).as_ptr(),
        edges,
        (mid_stride as usize * ::core::mem::size_of::<int16_t>()) as ptrdiff_t,
    );
}
#[cfg(feature = "asm")]
use crate::src::cpu::dav1d_get_cpu_flags;

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
#[inline(always)]
unsafe extern "C" fn loop_restoration_dsp_init_arm(
    c: *mut Dav1dLoopRestorationDSPContext,
    mut _bpc: libc::c_int,
) {
    use crate::src::arm::cpu::DAV1D_ARM_CPU_FLAG_NEON;
    // TODO(randomPoison): Import temporarily needed until init fns are deduplicated.
    #[cfg(target_arch = "aarch64")]
    use crate::src::looprestoration::*;

    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
        return;
    }

    cfg_if! {
        if #[cfg(target_arch = "aarch64")] {
            (*c).wiener[0] = dav1d_wiener_filter7_8bpc_neon;
            (*c).wiener[1] = dav1d_wiener_filter5_8bpc_neon;
        } else {
            (*c).wiener[0] = wiener_filter_neon_erased;
            (*c).wiener[1] = wiener_filter_neon_erased;
        }
    }

    (*c).sgr[0] = sgr_filter_5x5_neon_erased;
    (*c).sgr[1] = sgr_filter_3x3_neon_erased;
    (*c).sgr[2] = sgr_filter_mix_neon_erased;
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn dav1d_sgr_filter1_neon(
    mut tmp: *mut int16_t,
    mut src: *const pixel,
    stride: ptrdiff_t,
    mut left: *const [pixel; 4],
    mut lpf: *const pixel,
    w: libc::c_int,
    h: libc::c_int,
    strength: libc::c_int,
    edges: LrEdgeFlags,
) {
    use crate::src::looprestoration::LR_HAVE_BOTTOM;
    use crate::src::looprestoration::LR_HAVE_TOP;

    let mut sumsq_mem: Align16<[int32_t; 27208]> = Align16([0; 27208]);
    let sumsq: *mut int32_t = &mut *sumsq_mem
        .0
        .as_mut_ptr()
        .offset(((384 + 16) * 2 + 8) as isize) as *mut int32_t;
    let a: *mut int32_t = sumsq;
    let mut sum_mem: Align16<[int16_t; 27216]> = Align16([0; 27216]);
    let sum: *mut int16_t = &mut *sum_mem
        .0
        .as_mut_ptr()
        .offset(((384 + 16) * 2 + 16) as isize) as *mut int16_t;
    let b: *mut int16_t = sum;
    dav1d_sgr_box3_h_8bpc_neon(sumsq, sum, left, src, stride, w, h, edges);
    if edges as libc::c_uint & LR_HAVE_TOP as libc::c_int as libc::c_uint != 0 {
        dav1d_sgr_box3_h_8bpc_neon(
            &mut *sumsq.offset((-(2 as libc::c_int) * (384 + 16)) as isize),
            &mut *sum.offset((-(2 as libc::c_int) * (384 + 16)) as isize),
            0 as *const [pixel; 4],
            lpf,
            stride,
            w,
            2 as libc::c_int,
            edges,
        );
    }
    if edges as libc::c_uint & LR_HAVE_BOTTOM as libc::c_int as libc::c_uint != 0 {
        dav1d_sgr_box3_h_8bpc_neon(
            &mut *sumsq.offset((h * (384 + 16)) as isize),
            &mut *sum.offset((h * (384 + 16)) as isize),
            0 as *const [pixel; 4],
            lpf.offset((6 * stride) as isize),
            stride,
            w,
            2 as libc::c_int,
            edges,
        );
    }
    dav1d_sgr_box3_v_neon(sumsq, sum, w, h, edges);
    dav1d_sgr_calc_ab1_neon(a, b, w, h, strength, 0xff as libc::c_int);
    dav1d_sgr_finish_filter1_8bpc_neon(tmp, src, stride, a, b, w, h);
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn dav1d_sgr_filter2_neon(
    mut tmp: *mut int16_t,
    mut src: *const pixel,
    stride: ptrdiff_t,
    mut left: *const [pixel; 4],
    mut lpf: *const pixel,
    w: libc::c_int,
    h: libc::c_int,
    strength: libc::c_int,
    edges: LrEdgeFlags,
) {
    use crate::src::looprestoration::LR_HAVE_BOTTOM;
    use crate::src::looprestoration::LR_HAVE_TOP;

    let mut sumsq_mem: Align16<[int32_t; 27208]> = Align16([0; 27208]);
    let sumsq: *mut int32_t = &mut *sumsq_mem
        .0
        .as_mut_ptr()
        .offset(((384 + 16) * 2 + 8) as isize) as *mut int32_t;
    let a: *mut int32_t = sumsq;
    let mut sum_mem: Align16<[int16_t; 27216]> = Align16([0; 27216]);
    let sum: *mut int16_t = &mut *sum_mem
        .0
        .as_mut_ptr()
        .offset(((384 + 16) * 2 + 16) as isize) as *mut int16_t;
    let b: *mut int16_t = sum;
    dav1d_sgr_box5_h_8bpc_neon(sumsq, sum, left, src, stride, w, h, edges);
    if edges as libc::c_uint & LR_HAVE_TOP as libc::c_int as libc::c_uint != 0 {
        dav1d_sgr_box5_h_8bpc_neon(
            &mut *sumsq.offset((-(2 as libc::c_int) * (384 + 16)) as isize),
            &mut *sum.offset((-(2 as libc::c_int) * (384 + 16)) as isize),
            0 as *const [pixel; 4],
            lpf,
            stride,
            w,
            2 as libc::c_int,
            edges,
        );
    }
    if edges as libc::c_uint & LR_HAVE_BOTTOM as libc::c_int as libc::c_uint != 0 {
        dav1d_sgr_box5_h_8bpc_neon(
            &mut *sumsq.offset((h * (384 + 16)) as isize),
            &mut *sum.offset((h * (384 + 16)) as isize),
            0 as *const [pixel; 4],
            lpf.offset((6 * stride) as isize),
            stride,
            w,
            2 as libc::c_int,
            edges,
        );
    }
    dav1d_sgr_box5_v_neon(sumsq, sum, w, h, edges);
    dav1d_sgr_calc_ab2_neon(a, b, w, h, strength, 0xff as libc::c_int);
    dav1d_sgr_finish_filter2_8bpc_neon(tmp, src, stride, a, b, w, h);
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn sgr_filter_5x5_neon_erased(
    p: *mut libc::c_void,
    stride: ptrdiff_t,
    left: *const libc::c_void,
    lpf: *const libc::c_void,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    _bitdepth_max: libc::c_int,
) {
    sgr_filter_5x5_neon(
        p.cast(),
        stride,
        left.cast(),
        lpf.cast(),
        w,
        h,
        params,
        edges,
    )
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn sgr_filter_5x5_neon(
    dst: *mut pixel,
    stride: ptrdiff_t,
    left: *const [pixel; 4],
    mut lpf: *const pixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
) {
    let mut tmp: Align16<[int16_t; 24576]> = Align16([0; 24576]);
    dav1d_sgr_filter2_neon(
        tmp.0.as_mut_ptr(),
        dst,
        stride,
        left,
        lpf,
        w,
        h,
        (*params).sgr.s0 as libc::c_int,
        edges,
    );
    dav1d_sgr_weighted1_8bpc_neon(
        dst,
        stride,
        dst,
        stride,
        tmp.0.as_mut_ptr(),
        w,
        h,
        (*params).sgr.w0 as libc::c_int,
    );
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn sgr_filter_3x3_neon_erased(
    p: *mut libc::c_void,
    stride: ptrdiff_t,
    left: *const libc::c_void,
    lpf: *const libc::c_void,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    _bitdepth_max: libc::c_int,
) {
    sgr_filter_3x3_neon(
        p.cast(),
        stride,
        left.cast(),
        lpf.cast(),
        w,
        h,
        params,
        edges,
    )
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn sgr_filter_3x3_neon(
    dst: *mut pixel,
    stride: ptrdiff_t,
    left: *const [pixel; 4],
    mut lpf: *const pixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
) {
    let mut tmp: Align16<[int16_t; 24576]> = Align16([0; 24576]);
    dav1d_sgr_filter1_neon(
        tmp.0.as_mut_ptr(),
        dst,
        stride,
        left,
        lpf,
        w,
        h,
        (*params).sgr.s1 as libc::c_int,
        edges,
    );
    dav1d_sgr_weighted1_8bpc_neon(
        dst,
        stride,
        dst,
        stride,
        tmp.0.as_mut_ptr(),
        w,
        h,
        (*params).sgr.w1 as libc::c_int,
    );
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn sgr_filter_mix_neon_erased(
    p: *mut libc::c_void,
    stride: ptrdiff_t,
    left: *const libc::c_void,
    lpf: *const libc::c_void,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    _bitdepth_max: libc::c_int,
) {
    sgr_filter_mix_neon(
        p.cast(),
        stride,
        left.cast(),
        lpf.cast(),
        w,
        h,
        params,
        edges,
    )
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn sgr_filter_mix_neon(
    dst: *mut pixel,
    stride: ptrdiff_t,
    left: *const [pixel; 4],
    mut lpf: *const pixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
) {
    let mut tmp1: Align16<[int16_t; 24576]> = Align16([0; 24576]);
    let mut tmp2: Align16<[int16_t; 24576]> = Align16([0; 24576]);
    dav1d_sgr_filter2_neon(
        tmp1.0.as_mut_ptr(),
        dst,
        stride,
        left,
        lpf,
        w,
        h,
        (*params).sgr.s0 as libc::c_int,
        edges,
    );
    dav1d_sgr_filter1_neon(
        tmp2.0.as_mut_ptr(),
        dst,
        stride,
        left,
        lpf,
        w,
        h,
        (*params).sgr.s1 as libc::c_int,
        edges,
    );
    let wt: [int16_t; 2] = [(*params).sgr.w0, (*params).sgr.w1];
    dav1d_sgr_weighted2_8bpc_neon(
        dst,
        stride,
        dst,
        stride,
        tmp1.0.as_mut_ptr(),
        tmp2.0.as_mut_ptr(),
        w,
        h,
        wt.as_ptr(),
    );
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_loop_restoration_dsp_init_8bpc(
    c: *mut Dav1dLoopRestorationDSPContext,
    _bpc: libc::c_int,
) {
    (*c).wiener[1] = wiener_c_erased::<BitDepth8>;
    (*c).wiener[0] = (*c).wiener[1];
    (*c).sgr[0] = sgr_5x5_c;
    (*c).sgr[1] = sgr_3x3_c;
    (*c).sgr[2] = sgr_mix_c;

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            loop_restoration_dsp_init_x86(c, _bpc);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]{
            loop_restoration_dsp_init_arm(c, _bpc);
        }
    }
}
