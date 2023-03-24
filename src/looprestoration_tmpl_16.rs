use ::libc;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    static dav1d_sgr_x_by_x: [uint8_t; 256];
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type ptrdiff_t = libc::c_long;
pub type pixel = uint16_t;
pub type coef = int32_t;
pub type LrEdgeFlags = libc::c_uint;
pub const LR_HAVE_BOTTOM: LrEdgeFlags = 8;
pub const LR_HAVE_TOP: LrEdgeFlags = 4;
pub const LR_HAVE_RIGHT: LrEdgeFlags = 2;
pub const LR_HAVE_LEFT: LrEdgeFlags = 1;
pub type const_left_pixel_row = *const [pixel; 4];

#[repr(C)]
#[derive(Copy, Clone)]
pub union LooprestorationParams {
    pub filter: [[int16_t; 8]; 2],
    pub sgr: C2RustUnnamed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed {
    pub s0: uint32_t,
    pub s1: uint32_t,
    pub w0: int16_t,
    pub w1: int16_t,
}
pub type looprestorationfilter_fn = Option<
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav1dLoopRestorationDSPContext {
    pub wiener: [looprestorationfilter_fn; 2],
    pub sgr: [looprestorationfilter_fn; 3],
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
unsafe extern "C" fn umin(a: libc::c_uint, b: libc::c_uint) -> libc::c_uint {
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
unsafe extern "C" fn pixel_set(dst: *mut pixel, val: libc::c_int, num: libc::c_int) {
    let mut n: libc::c_int = 0 as libc::c_int;
    while n < num {
        *dst.offset(n as isize) = val as pixel;
        n += 1;
    }
}
#[inline]
unsafe extern "C" fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 as libc::c_int as libc::c_long != 0 {
        unreachable!();
    }
    return x >> 1 as libc::c_int;
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
    let have_left: libc::c_int =
        (edges as libc::c_uint & LR_HAVE_LEFT as libc::c_int as libc::c_uint != 0) as libc::c_int;
    let have_right: libc::c_int =
        (edges as libc::c_uint & LR_HAVE_RIGHT as libc::c_int as libc::c_uint != 0) as libc::c_int;
    unit_w += 3 as libc::c_int * have_left + 3 as libc::c_int * have_right;
    let mut dst_l: *mut pixel =
        dst.offset((3 as libc::c_int * (have_left == 0) as libc::c_int) as isize);
    p = p.offset(-((3 as libc::c_int * have_left) as isize));
    lpf = lpf.offset(-((3 as libc::c_int * have_left) as isize));
    if edges as libc::c_uint & LR_HAVE_TOP as libc::c_int as libc::c_uint != 0 {
        let above_1: *const pixel = lpf;
        let above_2: *const pixel = above_1.offset(PXSTRIDE(stride) as isize);
        memcpy(
            dst_l as *mut libc::c_void,
            above_1 as *const libc::c_void,
            (unit_w << 1 as libc::c_int) as libc::c_ulong,
        );
        memcpy(
            dst_l.offset(390 as libc::c_int as isize) as *mut libc::c_void,
            above_1 as *const libc::c_void,
            (unit_w << 1 as libc::c_int) as libc::c_ulong,
        );
        memcpy(
            dst_l.offset((2 as libc::c_int * 390 as libc::c_int) as isize) as *mut libc::c_void,
            above_2 as *const libc::c_void,
            (unit_w << 1 as libc::c_int) as libc::c_ulong,
        );
    } else {
        memcpy(
            dst_l as *mut libc::c_void,
            p as *const libc::c_void,
            (unit_w << 1 as libc::c_int) as libc::c_ulong,
        );
        memcpy(
            dst_l.offset(390 as libc::c_int as isize) as *mut libc::c_void,
            p as *const libc::c_void,
            (unit_w << 1 as libc::c_int) as libc::c_ulong,
        );
        memcpy(
            dst_l.offset((2 as libc::c_int * 390 as libc::c_int) as isize) as *mut libc::c_void,
            p as *const libc::c_void,
            (unit_w << 1 as libc::c_int) as libc::c_ulong,
        );
        if have_left != 0 {
            memcpy(
                dst_l as *mut libc::c_void,
                &*(*left.offset(0 as libc::c_int as isize))
                    .as_ptr()
                    .offset(1 as libc::c_int as isize) as *const pixel
                    as *const libc::c_void,
                ((3 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong,
            );
            memcpy(
                dst_l.offset(390 as libc::c_int as isize) as *mut libc::c_void,
                &*(*left.offset(0 as libc::c_int as isize))
                    .as_ptr()
                    .offset(1 as libc::c_int as isize) as *const pixel
                    as *const libc::c_void,
                ((3 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong,
            );
            memcpy(
                dst_l.offset((2 as libc::c_int * 390 as libc::c_int) as isize) as *mut libc::c_void,
                &*(*left.offset(0 as libc::c_int as isize))
                    .as_ptr()
                    .offset(1 as libc::c_int as isize) as *const pixel
                    as *const libc::c_void,
                ((3 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong,
            );
        }
    }
    let mut dst_tl: *mut pixel = dst_l.offset((3 as libc::c_int * 390 as libc::c_int) as isize);
    if edges as libc::c_uint & LR_HAVE_BOTTOM as libc::c_int as libc::c_uint != 0 {
        let below_1: *const pixel =
            lpf.offset((6 as libc::c_int as libc::c_long * PXSTRIDE(stride)) as isize);
        let below_2: *const pixel = below_1.offset(PXSTRIDE(stride) as isize);
        memcpy(
            dst_tl.offset((stripe_h * 390 as libc::c_int) as isize) as *mut libc::c_void,
            below_1 as *const libc::c_void,
            (unit_w << 1 as libc::c_int) as libc::c_ulong,
        );
        memcpy(
            dst_tl.offset(((stripe_h + 1 as libc::c_int) * 390 as libc::c_int) as isize)
                as *mut libc::c_void,
            below_2 as *const libc::c_void,
            (unit_w << 1 as libc::c_int) as libc::c_ulong,
        );
        memcpy(
            dst_tl.offset(((stripe_h + 2 as libc::c_int) * 390 as libc::c_int) as isize)
                as *mut libc::c_void,
            below_2 as *const libc::c_void,
            (unit_w << 1 as libc::c_int) as libc::c_ulong,
        );
    } else {
        let src: *const pixel =
            p.offset(((stripe_h - 1 as libc::c_int) as libc::c_long * PXSTRIDE(stride)) as isize);
        memcpy(
            dst_tl.offset((stripe_h * 390 as libc::c_int) as isize) as *mut libc::c_void,
            src as *const libc::c_void,
            (unit_w << 1 as libc::c_int) as libc::c_ulong,
        );
        memcpy(
            dst_tl.offset(((stripe_h + 1 as libc::c_int) * 390 as libc::c_int) as isize)
                as *mut libc::c_void,
            src as *const libc::c_void,
            (unit_w << 1 as libc::c_int) as libc::c_ulong,
        );
        memcpy(
            dst_tl.offset(((stripe_h + 2 as libc::c_int) * 390 as libc::c_int) as isize)
                as *mut libc::c_void,
            src as *const libc::c_void,
            (unit_w << 1 as libc::c_int) as libc::c_ulong,
        );
        if have_left != 0 {
            memcpy(
                dst_tl.offset((stripe_h * 390 as libc::c_int) as isize) as *mut libc::c_void,
                &*(*left.offset((stripe_h - 1 as libc::c_int) as isize))
                    .as_ptr()
                    .offset(1 as libc::c_int as isize) as *const pixel
                    as *const libc::c_void,
                ((3 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong,
            );
            memcpy(
                dst_tl.offset(((stripe_h + 1 as libc::c_int) * 390 as libc::c_int) as isize)
                    as *mut libc::c_void,
                &*(*left.offset((stripe_h - 1 as libc::c_int) as isize))
                    .as_ptr()
                    .offset(1 as libc::c_int as isize) as *const pixel
                    as *const libc::c_void,
                ((3 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong,
            );
            memcpy(
                dst_tl.offset(((stripe_h + 2 as libc::c_int) * 390 as libc::c_int) as isize)
                    as *mut libc::c_void,
                &*(*left.offset((stripe_h - 1 as libc::c_int) as isize))
                    .as_ptr()
                    .offset(1 as libc::c_int as isize) as *const pixel
                    as *const libc::c_void,
                ((3 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong,
            );
        }
    }
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < stripe_h {
        memcpy(
            dst_tl.offset((3 as libc::c_int * have_left) as isize) as *mut libc::c_void,
            p.offset((3 as libc::c_int * have_left) as isize) as *const libc::c_void,
            (unit_w - 3 as libc::c_int * have_left << 1 as libc::c_int) as libc::c_ulong,
        );
        dst_tl = dst_tl.offset(390 as libc::c_int as isize);
        p = p.offset(PXSTRIDE(stride) as isize);
        j += 1;
    }
    if have_right == 0 {
        let mut pad: *mut pixel = dst_l.offset(unit_w as isize);
        let mut row_last: *mut pixel =
            &mut *dst_l.offset((unit_w - 1 as libc::c_int) as isize) as *mut pixel;
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 < stripe_h + 6 as libc::c_int {
            pixel_set(pad, *row_last as libc::c_int, 3 as libc::c_int);
            pad = pad.offset(390 as libc::c_int as isize);
            row_last = row_last.offset(390 as libc::c_int as isize);
            j_0 += 1;
        }
    }
    if have_left == 0 {
        let mut j_1: libc::c_int = 0 as libc::c_int;
        while j_1 < stripe_h + 6 as libc::c_int {
            pixel_set(dst, *dst_l as libc::c_int, 3 as libc::c_int);
            dst = dst.offset(390 as libc::c_int as isize);
            dst_l = dst_l.offset(390 as libc::c_int as isize);
            j_1 += 1;
        }
    } else {
        dst = dst.offset((3 as libc::c_int * 390 as libc::c_int) as isize);
        let mut j_2: libc::c_int = 0 as libc::c_int;
        while j_2 < stripe_h {
            memcpy(
                dst as *mut libc::c_void,
                &*(*left.offset(j_2 as isize))
                    .as_ptr()
                    .offset(1 as libc::c_int as isize) as *const pixel
                    as *const libc::c_void,
                ((3 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong,
            );
            dst = dst.offset(390 as libc::c_int as isize);
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
    let filter: *const [int16_t; 8] = ((*params).filter).as_ptr();
    let bitdepth: libc::c_int = 32 as libc::c_int - clz(bitdepth_max as libc::c_uint);
    let round_bits_h: libc::c_int =
        3 as libc::c_int + (bitdepth == 12 as libc::c_int) as libc::c_int * 2 as libc::c_int;
    let rounding_off_h: libc::c_int = (1 as libc::c_int) << round_bits_h - 1 as libc::c_int;
    let clip_limit: libc::c_int =
        (1 as libc::c_int) << bitdepth + 1 as libc::c_int + 7 as libc::c_int - round_bits_h;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < h + 6 as libc::c_int {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < w {
            let mut sum: libc::c_int = (1 as libc::c_int) << bitdepth + 6 as libc::c_int;
            let mut k: libc::c_int = 0 as libc::c_int;
            while k < 7 as libc::c_int {
                sum += *tmp_ptr.offset((i + k) as isize) as libc::c_int
                    * (*filter.offset(0 as libc::c_int as isize))[k as usize] as libc::c_int;
                k += 1;
            }
            *hor_ptr.offset(i as isize) = iclip(
                sum + rounding_off_h >> round_bits_h,
                0 as libc::c_int,
                clip_limit - 1 as libc::c_int,
            ) as uint16_t;
            i += 1;
        }
        tmp_ptr = tmp_ptr.offset(390 as libc::c_int as isize);
        hor_ptr = hor_ptr.offset(390 as libc::c_int as isize);
        j += 1;
    }
    let round_bits_v: libc::c_int =
        11 as libc::c_int - (bitdepth == 12 as libc::c_int) as libc::c_int * 2 as libc::c_int;
    let rounding_off_v: libc::c_int = (1 as libc::c_int) << round_bits_v - 1 as libc::c_int;
    let round_offset: libc::c_int =
        (1 as libc::c_int) << bitdepth + (round_bits_v - 1 as libc::c_int);
    let mut j_0: libc::c_int = 0 as libc::c_int;
    while j_0 < h {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < w {
            let mut sum_0: libc::c_int = -round_offset;
            let mut k_0: libc::c_int = 0 as libc::c_int;
            while k_0 < 7 as libc::c_int {
                sum_0 += hor[((j_0 + k_0) * 390 as libc::c_int + i_0) as usize] as libc::c_int
                    * (*filter.offset(1 as libc::c_int as isize))[k_0 as usize] as libc::c_int;
                k_0 += 1;
            }
            *p.offset((j_0 as libc::c_long * PXSTRIDE(stride) + i_0 as libc::c_long) as isize) =
                iclip(
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
    src = src.offset(390 as libc::c_int as isize);
    let mut x: libc::c_int = 1 as libc::c_int;
    while x < w - 1 as libc::c_int {
        let mut sum_v: *mut coef = sum.offset(x as isize);
        let mut sumsq_v: *mut int32_t = sumsq.offset(x as isize);
        let mut s: *const pixel = src.offset(x as isize);
        let mut a: libc::c_int = *s.offset(0 as libc::c_int as isize) as libc::c_int;
        let mut a2: libc::c_int = a * a;
        let mut b: libc::c_int = *s.offset(390 as libc::c_int as isize) as libc::c_int;
        let mut b2: libc::c_int = b * b;
        let mut y: libc::c_int = 2 as libc::c_int;
        while y < h - 2 as libc::c_int {
            s = s.offset(390 as libc::c_int as isize);
            let c: libc::c_int = *s.offset(390 as libc::c_int as isize) as libc::c_int;
            let c2: libc::c_int = c * c;
            sum_v = sum_v.offset(390 as libc::c_int as isize);
            sumsq_v = sumsq_v.offset(390 as libc::c_int as isize);
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
    sum = sum.offset(390 as libc::c_int as isize);
    sumsq = sumsq.offset(390 as libc::c_int as isize);
    let mut y_0: libc::c_int = 2 as libc::c_int;
    while y_0 < h - 2 as libc::c_int {
        let mut a_0: libc::c_int = *sum.offset(1 as libc::c_int as isize);
        let mut a2_0: libc::c_int = *sumsq.offset(1 as libc::c_int as isize);
        let mut b_0: libc::c_int = *sum.offset(2 as libc::c_int as isize);
        let mut b2_0: libc::c_int = *sumsq.offset(2 as libc::c_int as isize);
        let mut x_0: libc::c_int = 2 as libc::c_int;
        while x_0 < w - 2 as libc::c_int {
            let c_0: libc::c_int = *sum.offset((x_0 + 1 as libc::c_int) as isize);
            let c2_0: libc::c_int = *sumsq.offset((x_0 + 1 as libc::c_int) as isize);
            *sum.offset(x_0 as isize) = a_0 + b_0 + c_0;
            *sumsq.offset(x_0 as isize) = a2_0 + b2_0 + c2_0;
            a_0 = b_0;
            a2_0 = b2_0;
            b_0 = c_0;
            b2_0 = c2_0;
            x_0 += 1;
        }
        sum = sum.offset(390 as libc::c_int as isize);
        sumsq = sumsq.offset(390 as libc::c_int as isize);
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
    let mut x: libc::c_int = 0 as libc::c_int;
    while x < w {
        let mut sum_v: *mut coef = sum.offset(x as isize);
        let mut sumsq_v: *mut int32_t = sumsq.offset(x as isize);
        let mut s: *const pixel = src
            .offset((3 as libc::c_int * 390 as libc::c_int) as isize)
            .offset(x as isize);
        let mut a: libc::c_int =
            *s.offset((-(3 as libc::c_int) * 390 as libc::c_int) as isize) as libc::c_int;
        let mut a2: libc::c_int = a * a;
        let mut b: libc::c_int =
            *s.offset((-(2 as libc::c_int) * 390 as libc::c_int) as isize) as libc::c_int;
        let mut b2: libc::c_int = b * b;
        let mut c: libc::c_int =
            *s.offset((-(1 as libc::c_int) * 390 as libc::c_int) as isize) as libc::c_int;
        let mut c2: libc::c_int = c * c;
        let mut d: libc::c_int = *s.offset(0 as libc::c_int as isize) as libc::c_int;
        let mut d2: libc::c_int = d * d;
        let mut y: libc::c_int = 2 as libc::c_int;
        while y < h - 2 as libc::c_int {
            s = s.offset(390 as libc::c_int as isize);
            let e: libc::c_int = *s as libc::c_int;
            let e2: libc::c_int = e * e;
            sum_v = sum_v.offset(390 as libc::c_int as isize);
            sumsq_v = sumsq_v.offset(390 as libc::c_int as isize);
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
    sum = sum.offset(390 as libc::c_int as isize);
    sumsq = sumsq.offset(390 as libc::c_int as isize);
    let mut y_0: libc::c_int = 2 as libc::c_int;
    while y_0 < h - 2 as libc::c_int {
        let mut a_0: libc::c_int = *sum.offset(0 as libc::c_int as isize);
        let mut a2_0: libc::c_int = *sumsq.offset(0 as libc::c_int as isize);
        let mut b_0: libc::c_int = *sum.offset(1 as libc::c_int as isize);
        let mut b2_0: libc::c_int = *sumsq.offset(1 as libc::c_int as isize);
        let mut c_0: libc::c_int = *sum.offset(2 as libc::c_int as isize);
        let mut c2_0: libc::c_int = *sumsq.offset(2 as libc::c_int as isize);
        let mut d_0: libc::c_int = *sum.offset(3 as libc::c_int as isize);
        let mut d2_0: libc::c_int = *sumsq.offset(3 as libc::c_int as isize);
        let mut x_0: libc::c_int = 2 as libc::c_int;
        while x_0 < w - 2 as libc::c_int {
            let e_0: libc::c_int = *sum.offset((x_0 + 2 as libc::c_int) as isize);
            let e2_0: libc::c_int = *sumsq.offset((x_0 + 2 as libc::c_int) as isize);
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
        sum = sum.offset(390 as libc::c_int as isize);
        sumsq = sumsq.offset(390 as libc::c_int as isize);
        y_0 += 1;
    }
}
#[inline(never)]
unsafe extern "C" fn selfguided_filter(
    mut dst: *mut coef,
    mut src: *const pixel,
    src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    n: libc::c_int,
    s: libc::c_uint,
    bitdepth_max: libc::c_int,
) {
    let sgr_one_by_x: libc::c_uint = (if n == 25 as libc::c_int {
        164 as libc::c_int
    } else {
        455 as libc::c_int
    }) as libc::c_uint;
    let mut sumsq: [int32_t; 26520] = [0; 26520];
    let mut A: *mut int32_t = sumsq
        .as_mut_ptr()
        .offset((2 as libc::c_int * 390 as libc::c_int) as isize)
        .offset(3 as libc::c_int as isize);
    let mut sum: [coef; 26520] = [0; 26520];
    let mut B: *mut coef = sum
        .as_mut_ptr()
        .offset((2 as libc::c_int * 390 as libc::c_int) as isize)
        .offset(3 as libc::c_int as isize);
    let step: libc::c_int = (n == 25 as libc::c_int) as libc::c_int + 1 as libc::c_int;
    if n == 25 as libc::c_int {
        boxsum5(
            sumsq.as_mut_ptr(),
            sum.as_mut_ptr(),
            src,
            w + 6 as libc::c_int,
            h + 6 as libc::c_int,
        );
    } else {
        boxsum3(
            sumsq.as_mut_ptr(),
            sum.as_mut_ptr(),
            src,
            w + 6 as libc::c_int,
            h + 6 as libc::c_int,
        );
    }
    let bitdepth_min_8: libc::c_int =
        32 as libc::c_int - clz(bitdepth_max as libc::c_uint) - 8 as libc::c_int;
    let mut AA: *mut int32_t = A.offset(-(390 as libc::c_int as isize));
    let mut BB: *mut coef = B.offset(-(390 as libc::c_int as isize));
    let mut j: libc::c_int = -(1 as libc::c_int);
    while j < h + 1 as libc::c_int {
        let mut i: libc::c_int = -(1 as libc::c_int);
        while i < w + 1 as libc::c_int {
            let a: libc::c_int = *AA.offset(i as isize)
                + ((1 as libc::c_int) << 2 as libc::c_int * bitdepth_min_8 >> 1 as libc::c_int)
                >> 2 as libc::c_int * bitdepth_min_8;
            let b: libc::c_int = *BB.offset(i as isize)
                + ((1 as libc::c_int) << bitdepth_min_8 >> 1 as libc::c_int)
                >> bitdepth_min_8;
            let p: libc::c_uint = imax(a * n - b * b, 0 as libc::c_int) as libc::c_uint;
            let z: libc::c_uint = p
                .wrapping_mul(s)
                .wrapping_add(((1 as libc::c_int) << 19 as libc::c_int) as libc::c_uint)
                >> 20 as libc::c_int;
            let x: libc::c_uint = dav1d_sgr_x_by_x
                [umin(z, 255 as libc::c_int as libc::c_uint) as usize]
                as libc::c_uint;
            *AA.offset(i as isize) = (x
                .wrapping_mul(*BB.offset(i as isize) as libc::c_uint)
                .wrapping_mul(sgr_one_by_x)
                .wrapping_add(((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint)
                >> 12 as libc::c_int) as int32_t;
            *BB.offset(i as isize) = x as coef;
            i += 1;
        }
        AA = AA.offset((step * 390 as libc::c_int) as isize);
        BB = BB.offset((step * 390 as libc::c_int) as isize);
        j += step;
    }
    src = src.offset((3 as libc::c_int * 390 as libc::c_int + 3 as libc::c_int) as isize);
    if n == 25 as libc::c_int {
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 < h - 1 as libc::c_int {
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < w {
                let a_0: libc::c_int = (*B.offset((i_0 - 390 as libc::c_int) as isize)
                    + *B.offset((i_0 + 390 as libc::c_int) as isize))
                    * 6 as libc::c_int
                    + (*B.offset((i_0 - 1 as libc::c_int - 390 as libc::c_int) as isize)
                        + *B.offset((i_0 - 1 as libc::c_int + 390 as libc::c_int) as isize)
                        + *B.offset((i_0 + 1 as libc::c_int - 390 as libc::c_int) as isize)
                        + *B.offset((i_0 + 1 as libc::c_int + 390 as libc::c_int) as isize))
                        * 5 as libc::c_int;
                let b_0: libc::c_int = (*A.offset((i_0 - 390 as libc::c_int) as isize)
                    + *A.offset((i_0 + 390 as libc::c_int) as isize))
                    * 6 as libc::c_int
                    + (*A.offset((i_0 - 1 as libc::c_int - 390 as libc::c_int) as isize)
                        + *A.offset((i_0 - 1 as libc::c_int + 390 as libc::c_int) as isize)
                        + *A.offset((i_0 + 1 as libc::c_int - 390 as libc::c_int) as isize)
                        + *A.offset((i_0 + 1 as libc::c_int + 390 as libc::c_int) as isize))
                        * 5 as libc::c_int;
                *dst.offset(i_0 as isize) = b_0 - a_0 * *src.offset(i_0 as isize) as libc::c_int
                    + ((1 as libc::c_int) << 8 as libc::c_int)
                    >> 9 as libc::c_int;
                i_0 += 1;
            }
            dst = dst.offset(384 as libc::c_int as isize);
            src = src.offset(390 as libc::c_int as isize);
            B = B.offset(390 as libc::c_int as isize);
            A = A.offset(390 as libc::c_int as isize);
            let mut i_1: libc::c_int = 0 as libc::c_int;
            while i_1 < w {
                let a_1: libc::c_int = *B.offset(i_1 as isize) * 6 as libc::c_int
                    + (*B.offset((i_1 - 1 as libc::c_int) as isize)
                        + *B.offset((i_1 + 1 as libc::c_int) as isize))
                        * 5 as libc::c_int;
                let b_1: libc::c_int = *A.offset(i_1 as isize) * 6 as libc::c_int
                    + (*A.offset((i_1 - 1 as libc::c_int) as isize)
                        + *A.offset((i_1 + 1 as libc::c_int) as isize))
                        * 5 as libc::c_int;
                *dst.offset(i_1 as isize) = b_1 - a_1 * *src.offset(i_1 as isize) as libc::c_int
                    + ((1 as libc::c_int) << 7 as libc::c_int)
                    >> 8 as libc::c_int;
                i_1 += 1;
            }
            dst = dst.offset(384 as libc::c_int as isize);
            src = src.offset(390 as libc::c_int as isize);
            B = B.offset(390 as libc::c_int as isize);
            A = A.offset(390 as libc::c_int as isize);
            j_0 += 2 as libc::c_int;
        }
        if j_0 + 1 as libc::c_int == h {
            let mut i_2: libc::c_int = 0 as libc::c_int;
            while i_2 < w {
                let a_2: libc::c_int = (*B.offset((i_2 - 390 as libc::c_int) as isize)
                    + *B.offset((i_2 + 390 as libc::c_int) as isize))
                    * 6 as libc::c_int
                    + (*B.offset((i_2 - 1 as libc::c_int - 390 as libc::c_int) as isize)
                        + *B.offset((i_2 - 1 as libc::c_int + 390 as libc::c_int) as isize)
                        + *B.offset((i_2 + 1 as libc::c_int - 390 as libc::c_int) as isize)
                        + *B.offset((i_2 + 1 as libc::c_int + 390 as libc::c_int) as isize))
                        * 5 as libc::c_int;
                let b_2: libc::c_int = (*A.offset((i_2 - 390 as libc::c_int) as isize)
                    + *A.offset((i_2 + 390 as libc::c_int) as isize))
                    * 6 as libc::c_int
                    + (*A.offset((i_2 - 1 as libc::c_int - 390 as libc::c_int) as isize)
                        + *A.offset((i_2 - 1 as libc::c_int + 390 as libc::c_int) as isize)
                        + *A.offset((i_2 + 1 as libc::c_int - 390 as libc::c_int) as isize)
                        + *A.offset((i_2 + 1 as libc::c_int + 390 as libc::c_int) as isize))
                        * 5 as libc::c_int;
                *dst.offset(i_2 as isize) = b_2 - a_2 * *src.offset(i_2 as isize) as libc::c_int
                    + ((1 as libc::c_int) << 8 as libc::c_int)
                    >> 9 as libc::c_int;
                i_2 += 1;
            }
        }
    } else {
        let mut j_1: libc::c_int = 0 as libc::c_int;
        while j_1 < h {
            let mut i_3: libc::c_int = 0 as libc::c_int;
            while i_3 < w {
                let a_3: libc::c_int = (*B.offset(i_3 as isize)
                    + *B.offset((i_3 - 1 as libc::c_int) as isize)
                    + *B.offset((i_3 + 1 as libc::c_int) as isize)
                    + *B.offset((i_3 - 390 as libc::c_int) as isize)
                    + *B.offset((i_3 + 390 as libc::c_int) as isize))
                    * 4 as libc::c_int
                    + (*B.offset((i_3 - 1 as libc::c_int - 390 as libc::c_int) as isize)
                        + *B.offset((i_3 - 1 as libc::c_int + 390 as libc::c_int) as isize)
                        + *B.offset((i_3 + 1 as libc::c_int - 390 as libc::c_int) as isize)
                        + *B.offset((i_3 + 1 as libc::c_int + 390 as libc::c_int) as isize))
                        * 3 as libc::c_int;
                let b_3: libc::c_int = (*A.offset(i_3 as isize)
                    + *A.offset((i_3 - 1 as libc::c_int) as isize)
                    + *A.offset((i_3 + 1 as libc::c_int) as isize)
                    + *A.offset((i_3 - 390 as libc::c_int) as isize)
                    + *A.offset((i_3 + 390 as libc::c_int) as isize))
                    * 4 as libc::c_int
                    + (*A.offset((i_3 - 1 as libc::c_int - 390 as libc::c_int) as isize)
                        + *A.offset((i_3 - 1 as libc::c_int + 390 as libc::c_int) as isize)
                        + *A.offset((i_3 + 1 as libc::c_int - 390 as libc::c_int) as isize)
                        + *A.offset((i_3 + 1 as libc::c_int + 390 as libc::c_int) as isize))
                        * 3 as libc::c_int;
                *dst.offset(i_3 as isize) = b_3 - a_3 * *src.offset(i_3 as isize) as libc::c_int
                    + ((1 as libc::c_int) << 8 as libc::c_int)
                    >> 9 as libc::c_int;
                i_3 += 1;
            }
            dst = dst.offset(384 as libc::c_int as isize);
            src = src.offset(390 as libc::c_int as isize);
            B = B.offset(390 as libc::c_int as isize);
            A = A.offset(390 as libc::c_int as isize);
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
    let w0: libc::c_int = (*params).sgr.w0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < h {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < w {
            let v: libc::c_int = w0 * dst[(j * 384 as libc::c_int + i) as usize];
            *p.offset(i as isize) = iclip(
                *p.offset(i as isize) as libc::c_int
                    + (v + ((1 as libc::c_int) << 10 as libc::c_int) >> 11 as libc::c_int),
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
    let w1: libc::c_int = (*params).sgr.w1 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < h {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < w {
            let v: libc::c_int = w1 * dst[(j * 384 as libc::c_int + i) as usize];
            *p.offset(i as isize) = iclip(
                *p.offset(i as isize) as libc::c_int
                    + (v + ((1 as libc::c_int) << 10 as libc::c_int) >> 11 as libc::c_int),
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
    let w0: libc::c_int = (*params).sgr.w0 as libc::c_int;
    let w1: libc::c_int = (*params).sgr.w1 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < h {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < w {
            let v: libc::c_int = w0 * dst0[(j * 384 as libc::c_int + i) as usize]
                + w1 * dst1[(j * 384 as libc::c_int + i) as usize];
            *p.offset(i as isize) = iclip(
                *p.offset(i as isize) as libc::c_int
                    + (v + ((1 as libc::c_int) << 10 as libc::c_int) >> 11 as libc::c_int),
                0 as libc::c_int,
                bitdepth_max,
            ) as pixel;
            i += 1;
        }
        p = p.offset(PXSTRIDE(stride) as isize);
        j += 1;
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_loop_restoration_dsp_init_16bpc(
    c: *mut Dav1dLoopRestorationDSPContext,
    bpc: libc::c_int,
) {
    (*c).wiener[1 as libc::c_int as usize] = Some(
        wiener_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const [pixel; 4],
                *const pixel,
                libc::c_int,
                libc::c_int,
                *const LooprestorationParams,
                LrEdgeFlags,
                libc::c_int,
            ) -> (),
    );
    (*c).wiener[0 as libc::c_int as usize] = (*c).wiener[1 as libc::c_int as usize];
    (*c).sgr[0 as libc::c_int as usize] = Some(
        sgr_5x5_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const [pixel; 4],
                *const pixel,
                libc::c_int,
                libc::c_int,
                *const LooprestorationParams,
                LrEdgeFlags,
                libc::c_int,
            ) -> (),
    );
    (*c).sgr[1 as libc::c_int as usize] = Some(
        sgr_3x3_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const [pixel; 4],
                *const pixel,
                libc::c_int,
                libc::c_int,
                *const LooprestorationParams,
                LrEdgeFlags,
                libc::c_int,
            ) -> (),
    );
    (*c).sgr[2 as libc::c_int as usize] = Some(
        sgr_mix_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const [pixel; 4],
                *const pixel,
                libc::c_int,
                libc::c_int,
                *const LooprestorationParams,
                LrEdgeFlags,
                libc::c_int,
            ) -> (),
    );
}
