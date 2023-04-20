use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
use cfg_if::cfg_if;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static dav1d_sgr_x_by_x: [uint8_t; 256];
}

#[cfg(feature = "asm")]
extern "C" {
    static mut dav1d_cpu_flags_mask: libc::c_uint;
    static mut dav1d_cpu_flags: libc::c_uint;
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
extern "C" {
    fn dav1d_wiener_filter5_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        left: const_left_pixel_row,
        lpf: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
        params: *const LooprestorationParams,
        edges: LrEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_wiener_filter7_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        left: const_left_pixel_row,
        lpf: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
        params: *const LooprestorationParams,
        edges: LrEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_wiener_filter5_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        left: const_left_pixel_row,
        lpf: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
        params: *const LooprestorationParams,
        edges: LrEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_wiener_filter7_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        left: const_left_pixel_row,
        lpf: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
        params: *const LooprestorationParams,
        edges: LrEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_wiener_filter5_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        left: const_left_pixel_row,
        lpf: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
        params: *const LooprestorationParams,
        edges: LrEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_wiener_filter7_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        left: const_left_pixel_row,
        lpf: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
        params: *const LooprestorationParams,
        edges: LrEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_sgr_filter_mix_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        left: const_left_pixel_row,
        lpf: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
        params: *const LooprestorationParams,
        edges: LrEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_sgr_filter_3x3_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        left: const_left_pixel_row,
        lpf: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
        params: *const LooprestorationParams,
        edges: LrEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_sgr_filter_5x5_16bpc_ssse3(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        left: const_left_pixel_row,
        lpf: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
        params: *const LooprestorationParams,
        edges: LrEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_sgr_filter_mix_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        left: const_left_pixel_row,
        lpf: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
        params: *const LooprestorationParams,
        edges: LrEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_sgr_filter_3x3_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        left: const_left_pixel_row,
        lpf: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
        params: *const LooprestorationParams,
        edges: LrEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_sgr_filter_5x5_16bpc_avx2(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        left: const_left_pixel_row,
        lpf: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
        params: *const LooprestorationParams,
        edges: LrEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_sgr_filter_5x5_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        left: const_left_pixel_row,
        lpf: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
        params: *const LooprestorationParams,
        edges: LrEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_sgr_filter_3x3_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        left: const_left_pixel_row,
        lpf: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
        params: *const LooprestorationParams,
        edges: LrEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_sgr_filter_mix_16bpc_avx512icl(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        left: const_left_pixel_row,
        lpf: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
        params: *const LooprestorationParams,
        edges: LrEdgeFlags,
        bitdepth_max: libc::c_int,
    );
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
extern "C" {
    fn dav1d_wiener_filter7_16bpc_neon(
        p: *mut pixel,
        stride: ptrdiff_t,
        left: *const [pixel; 4],
        lpf: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
        params: *const LooprestorationParams,
        edges: LrEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_wiener_filter5_16bpc_neon(
        p: *mut pixel,
        stride: ptrdiff_t,
        left: *const [pixel; 4],
        lpf: *const pixel,
        w: libc::c_int,
        h: libc::c_int,
        params: *const LooprestorationParams,
        edges: LrEdgeFlags,
        bitdepth_max: libc::c_int,
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
    fn dav1d_sgr_finish_filter2_16bpc_neon(
        tmp: *mut int16_t,
        src: *const pixel,
        stride: ptrdiff_t,
        a: *const int32_t,
        b: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_sgr_box3_h_16bpc_neon(
        sumsq: *mut int32_t,
        sum: *mut int16_t,
        left: *const [pixel; 4],
        src: *const pixel,
        stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        edges: LrEdgeFlags,
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
    fn dav1d_sgr_finish_filter1_16bpc_neon(
        tmp: *mut int16_t,
        src: *const pixel,
        stride: ptrdiff_t,
        a: *const int32_t,
        b: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_sgr_weighted2_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        t1: *const int16_t,
        t2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        wt: *const int16_t,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_sgr_weighted1_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        t1: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        wt: libc::c_int,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_sgr_box5_h_16bpc_neon(
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

pub type pixel = uint16_t;
pub type coef = int32_t;
use crate::src::looprestoration::LrEdgeFlags;
use crate::src::looprestoration::LR_HAVE_BOTTOM;
use crate::src::looprestoration::LR_HAVE_TOP;
use crate::src::looprestoration::LR_HAVE_RIGHT;
use crate::src::looprestoration::LR_HAVE_LEFT;
pub type const_left_pixel_row = *const [pixel; 4];
use crate::src::looprestoration::LooprestorationParams;

pub type looprestorationfilter_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        const_left_pixel_row,
        *const pixel,
        libc::c_int,
        libc::c_int,
        *const LooprestorationParams,
        LrEdgeFlags,
        libc::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dLoopRestorationDSPContext {
    pub wiener: [looprestorationfilter_fn; 2],
    pub sgr: [looprestorationfilter_fn; 3],
}
use crate::include::common::intops::imax;
use crate::include::common::attributes::clz;
use crate::include::common::intops::umin;
use crate::include::common::intops::iclip;
#[inline]
unsafe extern "C" fn pixel_set(dst: *mut pixel, val: libc::c_int, num: libc::c_int) {
    let mut n = 0;
    while n < num {
        *dst.offset(n as isize) = val as pixel;
        n += 1;
    }
}
#[inline]
unsafe extern "C" fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 != 0 {
        unreachable!();
    }
    return x >> 1;
}
#[inline(never)]
unsafe extern "C" fn padding(
    mut dst: *mut pixel,
    mut p: *const pixel,
    stride: ptrdiff_t,
    mut left: *const [pixel; 4],
    mut lpf: *const pixel,
    mut unit_w: libc::c_int,
    stripe_h: libc::c_int,
    edges: LrEdgeFlags,
) {
    let have_left = (edges as libc::c_uint
        & LR_HAVE_LEFT as libc::c_int as libc::c_uint != 0) as libc::c_int;
    let have_right = (edges as libc::c_uint
        & LR_HAVE_RIGHT as libc::c_int as libc::c_uint != 0) as libc::c_int;
    unit_w += 3 * have_left + 3 * have_right;
    let mut dst_l: *mut pixel = dst
        .offset((3 * (have_left == 0) as libc::c_int) as isize);
    p = p.offset(-((3 * have_left) as isize));
    lpf = lpf.offset(-((3 * have_left) as isize));
    if edges as libc::c_uint & LR_HAVE_TOP as libc::c_int as libc::c_uint != 0 {
        let above_1: *const pixel = lpf;
        let above_2: *const pixel = above_1.offset(PXSTRIDE(stride) as isize);
        memcpy(
            dst_l as *mut libc::c_void,
            above_1 as *const libc::c_void,
            (unit_w << 1) as libc::c_ulong,
        );
        memcpy(
            dst_l.offset(390) as *mut libc::c_void,
            above_1 as *const libc::c_void,
            (unit_w << 1) as libc::c_ulong,
        );
        memcpy(
            dst_l.offset((2 * 390) as isize)
                as *mut libc::c_void,
            above_2 as *const libc::c_void,
            (unit_w << 1) as libc::c_ulong,
        );
    } else {
        memcpy(
            dst_l as *mut libc::c_void,
            p as *const libc::c_void,
            (unit_w << 1) as libc::c_ulong,
        );
        memcpy(
            dst_l.offset(390) as *mut libc::c_void,
            p as *const libc::c_void,
            (unit_w << 1) as libc::c_ulong,
        );
        memcpy(
            dst_l.offset((2 * 390) as isize)
                as *mut libc::c_void,
            p as *const libc::c_void,
            (unit_w << 1) as libc::c_ulong,
        );
        if have_left != 0 {
            memcpy(
                dst_l as *mut libc::c_void,
                &*(*left.offset(0))
                    .as_ptr()
                    .offset(1) as *const pixel
                    as *const libc::c_void,
                ((3 as libc::c_int) << 1) as libc::c_ulong,
            );
            memcpy(
                dst_l.offset(390) as *mut libc::c_void,
                &*(*left.offset(0))
                    .as_ptr()
                    .offset(1) as *const pixel
                    as *const libc::c_void,
                ((3 as libc::c_int) << 1) as libc::c_ulong,
            );
            memcpy(
                dst_l.offset((2 * 390) as isize)
                    as *mut libc::c_void,
                &*(*left.offset(0))
                    .as_ptr()
                    .offset(1) as *const pixel
                    as *const libc::c_void,
                ((3 as libc::c_int) << 1) as libc::c_ulong,
            );
        }
    }
    let mut dst_tl: *mut pixel = dst_l
        .offset((3 * 390) as isize);
    if edges as libc::c_uint & LR_HAVE_BOTTOM as libc::c_int as libc::c_uint != 0 {
        let below_1: *const pixel = lpf
            .offset(6 * PXSTRIDE(stride));
        let below_2: *const pixel = below_1.offset(PXSTRIDE(stride) as isize);
        memcpy(
            dst_tl.offset((stripe_h * 390) as isize) as *mut libc::c_void,
            below_1 as *const libc::c_void,
            (unit_w << 1) as libc::c_ulong,
        );
        memcpy(
            dst_tl.offset(((stripe_h + 1) * 390) as isize)
                as *mut libc::c_void,
            below_2 as *const libc::c_void,
            (unit_w << 1) as libc::c_ulong,
        );
        memcpy(
            dst_tl.offset(((stripe_h + 2) * 390) as isize)
                as *mut libc::c_void,
            below_2 as *const libc::c_void,
            (unit_w << 1) as libc::c_ulong,
        );
    } else {
        let src: *const pixel = p
            .offset(
                ((stripe_h - 1) as isize * PXSTRIDE(stride))
                    as isize,
            );
        memcpy(
            dst_tl.offset((stripe_h * 390) as isize) as *mut libc::c_void,
            src as *const libc::c_void,
            (unit_w << 1) as libc::c_ulong,
        );
        memcpy(
            dst_tl.offset(((stripe_h + 1) * 390) as isize)
                as *mut libc::c_void,
            src as *const libc::c_void,
            (unit_w << 1) as libc::c_ulong,
        );
        memcpy(
            dst_tl.offset(((stripe_h + 2) * 390) as isize)
                as *mut libc::c_void,
            src as *const libc::c_void,
            (unit_w << 1) as libc::c_ulong,
        );
        if have_left != 0 {
            memcpy(
                dst_tl.offset((stripe_h * 390) as isize)
                    as *mut libc::c_void,
                &*(*left.offset((stripe_h - 1) as isize))
                    .as_ptr()
                    .offset(1) as *const pixel
                    as *const libc::c_void,
                ((3 as libc::c_int) << 1) as libc::c_ulong,
            );
            memcpy(
                dst_tl
                    .offset(
                        ((stripe_h + 1) * 390) as isize,
                    ) as *mut libc::c_void,
                &*(*left.offset((stripe_h - 1) as isize))
                    .as_ptr()
                    .offset(1) as *const pixel
                    as *const libc::c_void,
                ((3 as libc::c_int) << 1) as libc::c_ulong,
            );
            memcpy(
                dst_tl
                    .offset(
                        ((stripe_h + 2) * 390) as isize,
                    ) as *mut libc::c_void,
                &*(*left.offset((stripe_h - 1) as isize))
                    .as_ptr()
                    .offset(1) as *const pixel
                    as *const libc::c_void,
                ((3 as libc::c_int) << 1) as libc::c_ulong,
            );
        }
    }
    let mut j = 0;
    while j < stripe_h {
        memcpy(
            dst_tl.offset((3 * have_left) as isize) as *mut libc::c_void,
            p.offset((3 * have_left) as isize) as *const libc::c_void,
            (unit_w - 3 * have_left << 1) as libc::c_ulong,
        );
        dst_tl = dst_tl.offset(390);
        p = p.offset(PXSTRIDE(stride) as isize);
        j += 1;
    }
    if have_right == 0 {
        let mut pad: *mut pixel = dst_l.offset(unit_w as isize);
        let mut row_last: *mut pixel = &mut *dst_l
            .offset((unit_w - 1) as isize) as *mut pixel;
        let mut j_0 = 0;
        while j_0 < stripe_h + 6 {
            pixel_set(pad, *row_last as libc::c_int, 3 as libc::c_int);
            pad = pad.offset(390);
            row_last = row_last.offset(390);
            j_0 += 1;
        }
    }
    if have_left == 0 {
        let mut j_1 = 0;
        while j_1 < stripe_h + 6 {
            pixel_set(dst, *dst_l as libc::c_int, 3 as libc::c_int);
            dst = dst.offset(390);
            dst_l = dst_l.offset(390);
            j_1 += 1;
        }
    } else {
        dst = dst.offset((3 * 390) as isize);
        let mut j_2 = 0;
        while j_2 < stripe_h {
            memcpy(
                dst as *mut libc::c_void,
                &*(*left.offset(j_2 as isize)).as_ptr().offset(1)
                    as *const pixel as *const libc::c_void,
                ((3 as libc::c_int) << 1) as libc::c_ulong,
            );
            dst = dst.offset(390);
            j_2 += 1;
        }
    };
}
unsafe extern "C" fn wiener_c(
    mut p: *mut pixel,
    stride: ptrdiff_t,
    left: *const [pixel; 4],
    mut lpf: *const pixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: libc::c_int,
) {
    let mut tmp: [pixel; 27300] = [0; 27300];
    let mut tmp_ptr: *mut pixel = tmp.as_mut_ptr();
    padding(tmp.as_mut_ptr(), p, stride, left, lpf, w, h, edges);
    let mut hor: [uint16_t; 27300] = [0; 27300];
    let mut hor_ptr: *mut uint16_t = hor.as_mut_ptr();
    let filter: *const [int16_t; 8] = ((*params).filter.0).as_ptr();
    let bitdepth = 32 - clz(bitdepth_max as libc::c_uint);
    let round_bits_h = 3 as libc::c_int
        + (bitdepth == 12) as libc::c_int * 2;
    let rounding_off_h = (1 as libc::c_int)
        << round_bits_h - 1;
    let clip_limit = (1 as libc::c_int)
        << bitdepth + 1 + 7 - round_bits_h;
    let mut j = 0;
    while j < h + 6 {
        let mut i = 0;
        while i < w {
            let mut sum = (1 as libc::c_int) << bitdepth + 6;
            let mut k = 0;
            while k < 7 {
                sum
                    += *tmp_ptr.offset((i + k) as isize) as libc::c_int
                        * (*filter.offset(0))[k as usize]
                            as libc::c_int;
                k += 1;
            }
            *hor_ptr
                .offset(
                    i as isize,
                ) = iclip(
                sum + rounding_off_h >> round_bits_h,
                0 as libc::c_int,
                clip_limit - 1,
            ) as uint16_t;
            i += 1;
        }
        tmp_ptr = tmp_ptr.offset(390);
        hor_ptr = hor_ptr.offset(390);
        j += 1;
    }
    let round_bits_v = 11 as libc::c_int
        - (bitdepth == 12) as libc::c_int * 2;
    let rounding_off_v = (1 as libc::c_int)
        << round_bits_v - 1;
    let round_offset = (1 as libc::c_int)
        << bitdepth + (round_bits_v - 1);
    let mut j_0 = 0;
    while j_0 < h {
        let mut i_0 = 0;
        while i_0 < w {
            let mut sum_0 = -round_offset;
            let mut k_0 = 0;
            while k_0 < 7 {
                sum_0
                    += hor[((j_0 + k_0) * 390 + i_0) as usize]
                        as libc::c_int
                        * (*filter.offset(1))[k_0 as usize]
                            as libc::c_int;
                k_0 += 1;
            }
            *p
                .offset(
                    j_0 as isize * PXSTRIDE(stride) + i_0 as isize,
                ) = iclip(
                sum_0 + rounding_off_v >> round_bits_v,
                0 as libc::c_int,
                bitdepth_max,
            ) as pixel;
            i_0 += 1;
        }
        j_0 += 1;
    }
}
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
            *sum_v = a + b + c;
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
        let mut a_0 = *sum.offset(1);
        let mut a2_0 = *sumsq.offset(1);
        let mut b_0 = *sum.offset(2);
        let mut b2_0 = *sumsq.offset(2);
        let mut x_0 = 2;
        while x_0 < w - 2 {
            let c_0 = *sum.offset((x_0 + 1) as isize);
            let c2_0 = *sumsq.offset((x_0 + 1) as isize);
            *sum.offset(x_0 as isize) = a_0 + b_0 + c_0;
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
        let mut s: *const pixel = src
            .offset((3 * 390) as isize)
            .offset(x as isize);
        let mut a = *s
            .offset((-(3 as libc::c_int) * 390) as isize) as libc::c_int;
        let mut a2 = a * a;
        let mut b = *s
            .offset((-(2 as libc::c_int) * 390) as isize) as libc::c_int;
        let mut b2 = b * b;
        let mut c = *s
            .offset((-(1 as libc::c_int) * 390) as isize) as libc::c_int;
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
            *sum_v = a + b + c + d + e;
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
        let mut a_0 = *sum.offset(0);
        let mut a2_0 = *sumsq.offset(0);
        let mut b_0 = *sum.offset(1);
        let mut b2_0 = *sumsq.offset(1);
        let mut c_0 = *sum.offset(2);
        let mut c2_0 = *sumsq.offset(2);
        let mut d_0 = *sum.offset(3);
        let mut d2_0 = *sumsq.offset(3);
        let mut x_0 = 2;
        while x_0 < w - 2 {
            let e_0 = *sum.offset((x_0 + 2) as isize);
            let e2_0 = *sumsq.offset((x_0 + 2) as isize);
            *sum.offset(x_0 as isize) = a_0 + b_0 + c_0 + d_0 + e_0;
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
    bitdepth_max: libc::c_int,
) {
    let sgr_one_by_x: libc::c_uint = (if n == 25 {
        164 as libc::c_int
    } else {
        455 as libc::c_int
    }) as libc::c_uint;
    let mut sumsq: [int32_t; 26520] = [0; 26520];
    let mut A: *mut int32_t = sumsq
        .as_mut_ptr()
        .offset((2 * 390) as isize)
        .offset(3);
    let mut sum: [coef; 26520] = [0; 26520];
    let mut B: *mut coef = sum
        .as_mut_ptr()
        .offset((2 * 390) as isize)
        .offset(3);
    let step = (n == 25) as libc::c_int + 1;
    if n == 25 {
        boxsum5(
            sumsq.as_mut_ptr(),
            sum.as_mut_ptr(),
            src,
            w + 6,
            h + 6,
        );
    } else {
        boxsum3(
            sumsq.as_mut_ptr(),
            sum.as_mut_ptr(),
            src,
            w + 6,
            h + 6,
        );
    }
    let bitdepth_min_8 = 32 as libc::c_int
        - clz(bitdepth_max as libc::c_uint) - 8;
    let mut AA: *mut int32_t = A.offset(-(390 as libc::c_int as isize));
    let mut BB: *mut coef = B.offset(-(390 as libc::c_int as isize));
    let mut j = -(1 as libc::c_int);
    while j < h + 1 {
        let mut i = -(1 as libc::c_int);
        while i < w + 1 {
            let a = *AA.offset(i as isize)
                + ((1 as libc::c_int) << 2 * bitdepth_min_8
                    >> 1) >> 2 * bitdepth_min_8;
            let b = *BB.offset(i as isize)
                + ((1 as libc::c_int) << bitdepth_min_8 >> 1)
                >> bitdepth_min_8;
            let p: libc::c_uint = imax(a * n - b * b, 0 as libc::c_int) as libc::c_uint;
            let z: libc::c_uint = p
                .wrapping_mul(s)
                .wrapping_add(((1 as libc::c_int) << 19) as libc::c_uint)
                >> 20;
            let x: libc::c_uint = dav1d_sgr_x_by_x[umin(
                z,
                255 as libc::c_int as libc::c_uint,
            ) as usize] as libc::c_uint;
            *AA
                .offset(
                    i as isize,
                ) = (x
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
    src = src
        .offset((3 * 390 + 3) as isize);
    if n == 25 {
        let mut j_0 = 0;
        while j_0 < h - 1 {
            let mut i_0 = 0;
            while i_0 < w {
                let a_0 = (*B.offset((i_0 - 390) as isize)
                    + *B.offset((i_0 + 390) as isize)) * 6
                    + (*B.offset((i_0 - 1 - 390) as isize)
                        + *B
                            .offset(
                                (i_0 - 1 + 390) as isize,
                            )
                        + *B
                            .offset(
                                (i_0 + 1 - 390) as isize,
                            )
                        + *B
                            .offset(
                                (i_0 + 1 + 390) as isize,
                            )) * 5;
                let b_0 = (*A.offset((i_0 - 390) as isize)
                    + *A.offset((i_0 + 390) as isize)) * 6
                    + (*A.offset((i_0 - 1 - 390) as isize)
                        + *A
                            .offset(
                                (i_0 - 1 + 390) as isize,
                            )
                        + *A
                            .offset(
                                (i_0 + 1 - 390) as isize,
                            )
                        + *A
                            .offset(
                                (i_0 + 1 + 390) as isize,
                            )) * 5;
                *dst
                    .offset(
                        i_0 as isize,
                    ) = b_0 - a_0 * *src.offset(i_0 as isize) as libc::c_int
                    + ((1 as libc::c_int) << 8) >> 9;
                i_0 += 1;
            }
            dst = dst.offset(384);
            src = src.offset(390);
            B = B.offset(390);
            A = A.offset(390);
            let mut i_1 = 0;
            while i_1 < w {
                let a_1 = *B.offset(i_1 as isize) * 6
                    + (*B.offset((i_1 - 1) as isize)
                        + *B.offset((i_1 + 1) as isize))
                        * 5;
                let b_1 = *A.offset(i_1 as isize) * 6
                    + (*A.offset((i_1 - 1) as isize)
                        + *A.offset((i_1 + 1) as isize))
                        * 5;
                *dst
                    .offset(
                        i_1 as isize,
                    ) = b_1 - a_1 * *src.offset(i_1 as isize) as libc::c_int
                    + ((1 as libc::c_int) << 7) >> 8;
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
                let a_2 = (*B.offset((i_2 - 390) as isize)
                    + *B.offset((i_2 + 390) as isize)) * 6
                    + (*B.offset((i_2 - 1 - 390) as isize)
                        + *B
                            .offset(
                                (i_2 - 1 + 390) as isize,
                            )
                        + *B
                            .offset(
                                (i_2 + 1 - 390) as isize,
                            )
                        + *B
                            .offset(
                                (i_2 + 1 + 390) as isize,
                            )) * 5;
                let b_2 = (*A.offset((i_2 - 390) as isize)
                    + *A.offset((i_2 + 390) as isize)) * 6
                    + (*A.offset((i_2 - 1 - 390) as isize)
                        + *A
                            .offset(
                                (i_2 - 1 + 390) as isize,
                            )
                        + *A
                            .offset(
                                (i_2 + 1 - 390) as isize,
                            )
                        + *A
                            .offset(
                                (i_2 + 1 + 390) as isize,
                            )) * 5;
                *dst
                    .offset(
                        i_2 as isize,
                    ) = b_2 - a_2 * *src.offset(i_2 as isize) as libc::c_int
                    + ((1 as libc::c_int) << 8) >> 9;
                i_2 += 1;
            }
        }
    } else {
        let mut j_1 = 0;
        while j_1 < h {
            let mut i_3 = 0;
            while i_3 < w {
                let a_3 = (*B.offset(i_3 as isize)
                    + *B.offset((i_3 - 1) as isize)
                    + *B.offset((i_3 + 1) as isize)
                    + *B.offset((i_3 - 390) as isize)
                    + *B.offset((i_3 + 390) as isize)) * 4
                    + (*B.offset((i_3 - 1 - 390) as isize)
                        + *B
                            .offset(
                                (i_3 - 1 + 390) as isize,
                            )
                        + *B
                            .offset(
                                (i_3 + 1 - 390) as isize,
                            )
                        + *B
                            .offset(
                                (i_3 + 1 + 390) as isize,
                            )) * 3;
                let b_3 = (*A.offset(i_3 as isize)
                    + *A.offset((i_3 - 1) as isize)
                    + *A.offset((i_3 + 1) as isize)
                    + *A.offset((i_3 - 390) as isize)
                    + *A.offset((i_3 + 390) as isize)) * 4
                    + (*A.offset((i_3 - 1 - 390) as isize)
                        + *A
                            .offset(
                                (i_3 - 1 + 390) as isize,
                            )
                        + *A
                            .offset(
                                (i_3 + 1 - 390) as isize,
                            )
                        + *A
                            .offset(
                                (i_3 + 1 + 390) as isize,
                            )) * 3;
                *dst
                    .offset(
                        i_3 as isize,
                    ) = b_3 - a_3 * *src.offset(i_3 as isize) as libc::c_int
                    + ((1 as libc::c_int) << 8) >> 9;
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
    mut p: *mut pixel,
    stride: ptrdiff_t,
    left: *const [pixel; 4],
    mut lpf: *const pixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: libc::c_int,
) {
    let mut tmp: [pixel; 27300] = [0; 27300];
    let mut dst: [coef; 24576] = [0; 24576];
    padding(tmp.as_mut_ptr(), p, stride, left, lpf, w, h, edges);
    selfguided_filter(
        dst.as_mut_ptr(),
        tmp.as_mut_ptr(),
        390 as libc::c_int as ptrdiff_t,
        w,
        h,
        25 as libc::c_int,
        (*params).sgr.s0,
        bitdepth_max,
    );
    let w0 = (*params).sgr.w0 as libc::c_int;
    let mut j = 0;
    while j < h {
        let mut i = 0;
        while i < w {
            let v = w0 * dst[(j * 384 + i) as usize];
            *p
                .offset(
                    i as isize,
                ) = iclip(
                *p.offset(i as isize) as libc::c_int
                    + (v + ((1 as libc::c_int) << 10)
                        >> 11),
                0 as libc::c_int,
                bitdepth_max,
            ) as pixel;
            i += 1;
        }
        p = p.offset(PXSTRIDE(stride) as isize);
        j += 1;
    }
}
unsafe extern "C" fn sgr_3x3_c(
    mut p: *mut pixel,
    stride: ptrdiff_t,
    left: *const [pixel; 4],
    mut lpf: *const pixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: libc::c_int,
) {
    let mut tmp: [pixel; 27300] = [0; 27300];
    let mut dst: [coef; 24576] = [0; 24576];
    padding(tmp.as_mut_ptr(), p, stride, left, lpf, w, h, edges);
    selfguided_filter(
        dst.as_mut_ptr(),
        tmp.as_mut_ptr(),
        390 as libc::c_int as ptrdiff_t,
        w,
        h,
        9 as libc::c_int,
        (*params).sgr.s1,
        bitdepth_max,
    );
    let w1 = (*params).sgr.w1 as libc::c_int;
    let mut j = 0;
    while j < h {
        let mut i = 0;
        while i < w {
            let v = w1 * dst[(j * 384 + i) as usize];
            *p
                .offset(
                    i as isize,
                ) = iclip(
                *p.offset(i as isize) as libc::c_int
                    + (v + ((1 as libc::c_int) << 10)
                        >> 11),
                0 as libc::c_int,
                bitdepth_max,
            ) as pixel;
            i += 1;
        }
        p = p.offset(PXSTRIDE(stride) as isize);
        j += 1;
    }
}
unsafe extern "C" fn sgr_mix_c(
    mut p: *mut pixel,
    stride: ptrdiff_t,
    left: *const [pixel; 4],
    mut lpf: *const pixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: libc::c_int,
) {
    let mut tmp: [pixel; 27300] = [0; 27300];
    let mut dst0: [coef; 24576] = [0; 24576];
    let mut dst1: [coef; 24576] = [0; 24576];
    padding(tmp.as_mut_ptr(), p, stride, left, lpf, w, h, edges);
    selfguided_filter(
        dst0.as_mut_ptr(),
        tmp.as_mut_ptr(),
        390 as libc::c_int as ptrdiff_t,
        w,
        h,
        25 as libc::c_int,
        (*params).sgr.s0,
        bitdepth_max,
    );
    selfguided_filter(
        dst1.as_mut_ptr(),
        tmp.as_mut_ptr(),
        390 as libc::c_int as ptrdiff_t,
        w,
        h,
        9 as libc::c_int,
        (*params).sgr.s1,
        bitdepth_max,
    );
    let w0 = (*params).sgr.w0 as libc::c_int;
    let w1 = (*params).sgr.w1 as libc::c_int;
    let mut j = 0;
    while j < h {
        let mut i = 0;
        while i < w {
            let v = w0 * dst0[(j * 384 + i) as usize]
                + w1 * dst1[(j * 384 + i) as usize];
            *p
                .offset(
                    i as isize,
                ) = iclip(
                *p.offset(i as isize) as libc::c_int
                    + (v + ((1 as libc::c_int) << 10)
                        >> 11),
                0 as libc::c_int,
                bitdepth_max,
            ) as pixel;
            i += 1;
        }
        p = p.offset(PXSTRIDE(stride) as isize);
        j += 1;
    }
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
#[inline(always)]
unsafe extern "C" fn loop_restoration_dsp_init_x86(
    c: *mut Dav1dLoopRestorationDSPContext,
    bpc: libc::c_int,
) {
    use crate::src::x86::cpu::DAV1D_X86_CPU_FLAG_AVX512ICL;
    use crate::src::x86::cpu::DAV1D_X86_CPU_FLAG_SSE2;
    use crate::src::x86::cpu::DAV1D_X86_CPU_FLAG_AVX2;
    use crate::src::x86::cpu::DAV1D_X86_CPU_FLAG_SSSE3;

    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_X86_CPU_FLAG_SSE2 == 0 {
        return;
    }

    if flags & DAV1D_X86_CPU_FLAG_SSSE3 == 0 {
        return;
    }

    (*c).wiener[0] = Some(dav1d_wiener_filter7_16bpc_ssse3);
    (*c).wiener[1] = Some(dav1d_wiener_filter5_16bpc_ssse3);

    if bpc == 10 {
        (*c).sgr[0] = Some(dav1d_sgr_filter_5x5_16bpc_ssse3);
        (*c).sgr[1] = Some(dav1d_sgr_filter_3x3_16bpc_ssse3);
        (*c).sgr[2] = Some(dav1d_sgr_filter_mix_16bpc_ssse3);
    }

    if flags & DAV1D_X86_CPU_FLAG_AVX2 == 0 {
        return;
    }

    (*c).wiener[0] = Some(dav1d_wiener_filter7_16bpc_avx2);
    (*c).wiener[1] = Some(dav1d_wiener_filter5_16bpc_avx2);

    if bpc == 10 {
        (*c).sgr[0] = Some(dav1d_sgr_filter_5x5_16bpc_avx2);
        (*c).sgr[1] = Some(dav1d_sgr_filter_3x3_16bpc_avx2);
        (*c).sgr[2] = Some(dav1d_sgr_filter_mix_16bpc_avx2);
    }

    if flags & DAV1D_X86_CPU_FLAG_AVX512ICL == 0 {
        return;
    }

    (*c).wiener[0] = Some(dav1d_wiener_filter7_16bpc_avx512icl);
    (*c).wiener[1] = Some(dav1d_wiener_filter5_16bpc_avx512icl);

    if bpc == 10 {
        (*c).sgr[0] = Some(dav1d_sgr_filter_5x5_16bpc_avx512icl);
        (*c).sgr[1] = Some(dav1d_sgr_filter_3x3_16bpc_avx512icl);
        (*c).sgr[2] = Some(dav1d_sgr_filter_mix_16bpc_avx512icl);
    }
}

#[cfg(feature = "asm")]
use crate::src::cpu::dav1d_get_cpu_flags;

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
#[inline(always)]
unsafe extern "C" fn loop_restoration_dsp_init_arm(
    c: *mut Dav1dLoopRestorationDSPContext,
    mut bpc: libc::c_int,
) {
    use crate::src::arm::cpu::DAV1D_ARM_CPU_FLAG_NEON;

    let flags: libc::c_uint = dav1d_get_cpu_flags();

    if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
        return;
    }

    (*c).wiener[0] = Some(dav1d_wiener_filter7_16bpc_neon);
    (*c).wiener[1] = Some(dav1d_wiener_filter5_16bpc_neon);

    if bpc == 10 {
        (*c).sgr[0] = Some(sgr_filter_5x5_neon);
        (*c).sgr[1] = Some(sgr_filter_3x3_neon);
        (*c).sgr[2] = Some(sgr_filter_mix_neon);
    }
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
    bitdepth_max: libc::c_int,
) {
    let mut tmp: [int16_t; 24576] = [0; 24576];
    dav1d_sgr_filter1_neon(
        tmp.as_mut_ptr(),
        dst,
        stride,
        left,
        lpf,
        w,
        h,
        (*params).sgr.s1 as libc::c_int,
        edges,
        bitdepth_max,
    );
    dav1d_sgr_weighted1_16bpc_neon(
        dst,
        stride,
        dst,
        stride,
        tmp.as_mut_ptr(),
        w,
        h,
        (*params).sgr.w1 as libc::c_int,
        bitdepth_max,
    );
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
    bitdepth_max: libc::c_int,
) {
    let mut sumsq_mem: [int32_t; 27208] = [0; 27208];
    let sumsq: *mut int32_t = &mut *sumsq_mem
        .as_mut_ptr()
        .offset(
            ((384 + 16) * 2
                + 8) as isize,
        ) as *mut int32_t;
    let a: *mut int32_t = sumsq;
    let mut sum_mem: [int16_t; 27216] = [0; 27216];
    let sum: *mut int16_t = &mut *sum_mem
        .as_mut_ptr()
        .offset(
            ((384 + 16) * 2
                + 16) as isize,
        ) as *mut int16_t;
    let b: *mut int16_t = sum;
    dav1d_sgr_box3_h_16bpc_neon(sumsq, sum, left, src, stride, w, h, edges);
    if edges as libc::c_uint & LR_HAVE_TOP as libc::c_int as libc::c_uint != 0 {
        dav1d_sgr_box3_h_16bpc_neon(
            &mut *sumsq
                .offset(
                    (-(2 as libc::c_int) * (384 + 16))
                        as isize,
                ),
            &mut *sum
                .offset(
                    (-(2 as libc::c_int) * (384 + 16))
                        as isize,
                ),
            0 as *const [pixel; 4],
            lpf,
            stride,
            w,
            2 as libc::c_int,
            edges,
        );
    }
    if edges as libc::c_uint & LR_HAVE_BOTTOM as libc::c_int as libc::c_uint != 0 {
        dav1d_sgr_box3_h_16bpc_neon(
            &mut *sumsq.offset((h * (384 + 16)) as isize),
            &mut *sum.offset((h * (384 + 16)) as isize),
            0 as *const [pixel; 4],
            lpf.offset((6 * PXSTRIDE(stride)) as isize),
            stride,
            w,
            2 as libc::c_int,
            edges,
        );
    }
    dav1d_sgr_box3_v_neon(sumsq, sum, w, h, edges);
    dav1d_sgr_calc_ab1_neon(a, b, w, h, strength, bitdepth_max);
    dav1d_sgr_finish_filter1_16bpc_neon(tmp, src, stride, a, b, w, h);
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
    bitdepth_max: libc::c_int,
) {
    let mut sumsq_mem: [int32_t; 27208] = [0; 27208];
    let sumsq: *mut int32_t = &mut *sumsq_mem
        .as_mut_ptr()
        .offset(
            ((384 + 16) * 2
                + 8) as isize,
        ) as *mut int32_t;
    let a: *mut int32_t = sumsq;
    let mut sum_mem: [int16_t; 27216] = [0; 27216];
    let sum: *mut int16_t = &mut *sum_mem
        .as_mut_ptr()
        .offset(
            ((384 + 16) * 2
                + 16) as isize,
        ) as *mut int16_t;
    let b: *mut int16_t = sum;
    dav1d_sgr_box5_h_16bpc_neon(sumsq, sum, left, src, stride, w, h, edges);
    if edges as libc::c_uint & LR_HAVE_TOP as libc::c_int as libc::c_uint != 0 {
        dav1d_sgr_box5_h_16bpc_neon(
            &mut *sumsq
                .offset(
                    (-(2 as libc::c_int) * (384 + 16))
                        as isize,
                ),
            &mut *sum
                .offset(
                    (-(2 as libc::c_int) * (384 + 16))
                        as isize,
                ),
            0 as *const [pixel; 4],
            lpf,
            stride,
            w,
            2 as libc::c_int,
            edges,
        );
    }
    if edges as libc::c_uint & LR_HAVE_BOTTOM as libc::c_int as libc::c_uint != 0 {
        dav1d_sgr_box5_h_16bpc_neon(
            &mut *sumsq.offset((h * (384 + 16)) as isize),
            &mut *sum.offset((h * (384 + 16)) as isize),
            0 as *const [pixel; 4],
            lpf.offset((6 * PXSTRIDE(stride)) as isize),
            stride,
            w,
            2 as libc::c_int,
            edges,
        );
    }
    dav1d_sgr_box5_v_neon(sumsq, sum, w, h, edges);
    dav1d_sgr_calc_ab2_neon(a, b, w, h, strength, bitdepth_max);
    dav1d_sgr_finish_filter2_16bpc_neon(tmp, src, stride, a, b, w, h);
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
    bitdepth_max: libc::c_int,
) {
    let mut tmp: [int16_t; 24576] = [0; 24576];
    dav1d_sgr_filter2_neon(
        tmp.as_mut_ptr(),
        dst,
        stride,
        left,
        lpf,
        w,
        h,
        (*params).sgr.s0 as libc::c_int,
        edges,
        bitdepth_max,
    );
    dav1d_sgr_weighted1_16bpc_neon(
        dst,
        stride,
        dst,
        stride,
        tmp.as_mut_ptr(),
        w,
        h,
        (*params).sgr.w0 as libc::c_int,
        bitdepth_max,
    );
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
    bitdepth_max: libc::c_int,
) {
    let mut tmp1: [int16_t; 24576] = [0; 24576];
    let mut tmp2: [int16_t; 24576] = [0; 24576];
    dav1d_sgr_filter2_neon(
        tmp1.as_mut_ptr(),
        dst,
        stride,
        left,
        lpf,
        w,
        h,
        (*params).sgr.s0 as libc::c_int,
        edges,
        bitdepth_max,
    );
    dav1d_sgr_filter1_neon(
        tmp2.as_mut_ptr(),
        dst,
        stride,
        left,
        lpf,
        w,
        h,
        (*params).sgr.s1 as libc::c_int,
        edges,
        bitdepth_max,
    );
    let wt: [int16_t; 2] = [(*params).sgr.w0, (*params).sgr.w1];
    dav1d_sgr_weighted2_16bpc_neon(
        dst,
        stride,
        dst,
        stride,
        tmp1.as_mut_ptr(),
        tmp2.as_mut_ptr(),
        w,
        h,
        wt.as_ptr(),
        bitdepth_max,
    );
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_loop_restoration_dsp_init_16bpc(
    c: *mut Dav1dLoopRestorationDSPContext,
    bpc: libc::c_int,
) {
    (*c).wiener[1] = Some(wiener_c);
    (*c).wiener[0] = (*c).wiener[1];
    (*c).sgr[0] = Some(sgr_5x5_c);
    (*c).sgr[1] = Some(sgr_3x3_c);
    (*c).sgr[2] = Some(sgr_mix_c);

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            loop_restoration_dsp_init_x86(c, bpc);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]{
            loop_restoration_dsp_init_arm(c, bpc);
        }
    }
}
