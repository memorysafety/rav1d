use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
use crate::include::common::bitdepth::{BitDepth16, BitDepth8};
use crate::include::common::intops::iclip;
use crate::include::common::intops::imax;
use crate::include::common::intops::umin;
use crate::include::stddef::ptrdiff_t;
use crate::include::stdint::int16_t;
use crate::include::stdint::int32_t;
use crate::include::stdint::uint16_t;
use crate::include::stdint::uint32_t;
use crate::src::align::Align16;
use crate::src::tables::dav1d_sgr_x_by_x;

pub type LrEdgeFlags = libc::c_uint;
pub const LR_HAVE_BOTTOM: LrEdgeFlags = 8;
pub const LR_HAVE_TOP: LrEdgeFlags = 4;
pub const LR_HAVE_RIGHT: LrEdgeFlags = 2;
pub const LR_HAVE_LEFT: LrEdgeFlags = 1;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LooprestorationParams_sgr {
    pub s0: uint32_t,
    pub s1: uint32_t,
    pub w0: int16_t,
    pub w1: int16_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union LooprestorationParams {
    pub filter: Align16<[[int16_t; 8]; 2]>,
    pub sgr: LooprestorationParams_sgr,
}

type pixel = libc::c_void;
pub type const_left_pixel_row = *const libc::c_void; // *const [pixel; 4]

pub type looprestorationfilter_fn = unsafe extern "C" fn(
    *mut pixel,
    ptrdiff_t,
    const_left_pixel_row,
    *const pixel,
    libc::c_int,
    libc::c_int,
    *const LooprestorationParams,
    LrEdgeFlags,
    libc::c_int,
) -> ();

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dLoopRestorationDSPContext {
    pub wiener: [looprestorationfilter_fn; 2],
    pub sgr: [looprestorationfilter_fn; 3],
}

#[cfg(feature = "asm")]
macro_rules! decl_looprestorationfilter_fns {
    ( $( fn $name:ident, )* ) => {
        extern "C" {
            $(
                // TODO(randomPoison): Temporarily pub until init fns are deduplicated.
                pub(crate) fn $name(
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
            )*
        }
    };
}

#[cfg(all(
    feature = "bitdepth_8",
    feature = "asm",
    any(target_arch = "x86", target_arch = "x86_64"),
))]
decl_looprestorationfilter_fns! {
    fn dav1d_wiener_filter7_8bpc_sse2,
    fn dav1d_wiener_filter5_8bpc_sse2,
    fn dav1d_wiener_filter7_8bpc_ssse3,
    fn dav1d_wiener_filter5_8bpc_ssse3,
    fn dav1d_wiener_filter5_8bpc_avx2,
    fn dav1d_wiener_filter7_8bpc_avx2,
    fn dav1d_wiener_filter7_8bpc_avx512icl,
    fn dav1d_sgr_filter_mix_8bpc_avx512icl,
    fn dav1d_sgr_filter_3x3_8bpc_avx512icl,
    fn dav1d_sgr_filter_5x5_8bpc_avx512icl,
    fn dav1d_sgr_filter_mix_8bpc_avx2,
    fn dav1d_sgr_filter_3x3_8bpc_avx2,
    fn dav1d_sgr_filter_5x5_8bpc_avx2,
    fn dav1d_sgr_filter_mix_8bpc_ssse3,
    fn dav1d_sgr_filter_3x3_8bpc_ssse3,
    fn dav1d_sgr_filter_5x5_8bpc_ssse3,
}

#[cfg(all(
    feature = "bitdepth_16",
    feature = "asm",
    any(target_arch = "x86", target_arch = "x86_64"),
))]
decl_looprestorationfilter_fns! {
    fn dav1d_wiener_filter5_16bpc_ssse3,
    fn dav1d_wiener_filter7_16bpc_ssse3,
    fn dav1d_wiener_filter5_16bpc_avx2,
    fn dav1d_wiener_filter7_16bpc_avx2,
    fn dav1d_wiener_filter5_16bpc_avx512icl,
    fn dav1d_wiener_filter7_16bpc_avx512icl,
    fn dav1d_sgr_filter_mix_16bpc_ssse3,
    fn dav1d_sgr_filter_3x3_16bpc_ssse3,
    fn dav1d_sgr_filter_5x5_16bpc_ssse3,
    fn dav1d_sgr_filter_mix_16bpc_avx2,
    fn dav1d_sgr_filter_3x3_16bpc_avx2,
    fn dav1d_sgr_filter_5x5_16bpc_avx2,
    fn dav1d_sgr_filter_5x5_16bpc_avx512icl,
    fn dav1d_sgr_filter_3x3_16bpc_avx512icl,
    fn dav1d_sgr_filter_mix_16bpc_avx512icl,
}

#[cfg(all(
    feature = "bitdepth_8",
    feature = "asm",
    any(target_arch = "arm", target_arch = "aarch64"),
))]
decl_looprestorationfilter_fns! {
    fn dav1d_wiener_filter7_8bpc_neon,
    fn dav1d_wiener_filter5_8bpc_neon,
}

#[cfg(all(
    feature = "bitdepth_16",
    feature = "asm",
    any(target_arch = "arm", target_arch = "aarch64"),
))]
decl_looprestorationfilter_fns! {
    fn dav1d_wiener_filter7_16bpc_neon,
    fn dav1d_wiener_filter5_16bpc_neon,
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
extern "C" {
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
}

// 256 * 1.5 + 3 + 3 = 390
const REST_UNIT_STRIDE: usize = 390;

// TODO Reuse p when no padding is needed (add and remove lpf pixels in p)
// TODO Chroma only requires 2 rows of padding.
// TODO(randomPoison): Temporarily pub until remaining looprestoration fns have
// been deduplicated.
#[inline(never)]
pub(crate) unsafe fn padding<BD: BitDepth>(
    mut dst: &mut [BD::Pixel; 70 /*(64 + 3 + 3)*/ * REST_UNIT_STRIDE],
    mut p: *const BD::Pixel,
    stride: ptrdiff_t,
    mut left: *const [BD::Pixel; 4],
    mut lpf: *const BD::Pixel,
    mut unit_w: libc::c_int,
    stripe_h: libc::c_int,
    edges: LrEdgeFlags,
) {
    let stride = BD::pxstride(stride as usize);

    let have_left = (edges & LR_HAVE_LEFT != 0) as libc::c_int;
    let have_right = (edges & LR_HAVE_RIGHT != 0) as libc::c_int;

    // Copy more pixels if we don't have to pad them
    unit_w += 3 * have_left + 3 * have_right;
    let mut dst_l = &mut dst[(3 * (have_left == 0) as libc::c_int) as usize..];
    p = p.offset(-((3 * have_left) as isize));
    lpf = lpf.offset(-((3 * have_left) as isize));

    if edges & LR_HAVE_TOP != 0 {
        // Copy previous loop filtered rows
        let above_1 = std::slice::from_raw_parts(lpf, stride + unit_w as usize);
        let above_2 = &above_1[stride..];

        BD::pixel_copy(dst_l, above_1, unit_w as usize);
        BD::pixel_copy(&mut dst_l[REST_UNIT_STRIDE..], above_1, unit_w as usize);
        BD::pixel_copy(&mut dst_l[2 * REST_UNIT_STRIDE..], above_2, unit_w as usize);
    } else {
        // Pad with first row
        let p = std::slice::from_raw_parts(p, unit_w as usize);

        BD::pixel_copy(dst_l, p, unit_w as usize);
        BD::pixel_copy(&mut dst_l[REST_UNIT_STRIDE..], p, unit_w as usize);
        BD::pixel_copy(&mut dst_l[2 * REST_UNIT_STRIDE..], p, unit_w as usize);

        if have_left != 0 {
            let left = &(*left.offset(0))[1..];
            BD::pixel_copy(dst_l, left, 3);
            BD::pixel_copy(&mut dst_l[REST_UNIT_STRIDE..], left, 3);
            BD::pixel_copy(&mut dst_l[2 * REST_UNIT_STRIDE..], left, 3);
        }
    }

    let mut dst_tl = &mut dst_l[3 * REST_UNIT_STRIDE..];
    if edges & LR_HAVE_BOTTOM != 0 {
        // Copy next loop filtered rows
        let below_1 = std::slice::from_raw_parts(lpf.offset(6 * stride as isize), unit_w as usize);
        let below_2 = std::slice::from_raw_parts(lpf.offset(7 * stride as isize), unit_w as usize);

        BD::pixel_copy(
            &mut dst_tl[stripe_h as usize * REST_UNIT_STRIDE..],
            below_1,
            unit_w as usize,
        );
        BD::pixel_copy(
            &mut dst_tl[(stripe_h + 1) as usize * REST_UNIT_STRIDE..],
            below_2,
            unit_w as usize,
        );
        BD::pixel_copy(
            &mut dst_tl[(stripe_h + 2) as usize * REST_UNIT_STRIDE..],
            below_2,
            unit_w as usize,
        );
    } else {
        // Pad with last row
        let src = std::slice::from_raw_parts(
            p.offset(((stripe_h - 1) as isize * stride as isize) as isize),
            unit_w as usize,
        );

        BD::pixel_copy(
            &mut dst_tl[stripe_h as usize * REST_UNIT_STRIDE..],
            src,
            unit_w as usize,
        );
        BD::pixel_copy(
            &mut dst_tl[(stripe_h + 1) as usize * REST_UNIT_STRIDE..],
            src,
            unit_w as usize,
        );
        BD::pixel_copy(
            &mut dst_tl[(stripe_h + 2) as usize * REST_UNIT_STRIDE..],
            src,
            unit_w as usize,
        );

        if have_left != 0 {
            let left = &(*left.offset((stripe_h - 1) as isize))[1..];

            BD::pixel_copy(&mut dst_tl[stripe_h as usize * REST_UNIT_STRIDE..], left, 3);
            BD::pixel_copy(
                &mut dst_tl[(stripe_h + 1) as usize * REST_UNIT_STRIDE..],
                left,
                3,
            );
            BD::pixel_copy(
                &mut dst_tl[(stripe_h + 2) as usize * REST_UNIT_STRIDE..],
                left,
                3,
            );
        }
    }

    // Inner UNIT_WxSTRIPE_H
    let len = (unit_w - 3 * have_left) as usize;
    for j in 0..stripe_h as usize {
        let p = std::slice::from_raw_parts(
            p.offset((j * stride + 3 * have_left as usize) as isize),
            len,
        );
        BD::pixel_copy(
            &mut dst_tl[j * REST_UNIT_STRIDE + (3 * have_left) as usize..],
            p,
            len,
        );
    }

    if have_right == 0 {
        // Pad 3x(STRIPE_H+6) with last column
        for j in 0..stripe_h as usize + 6 {
            let mut row_last = dst_l[(unit_w - 1) as usize + j * REST_UNIT_STRIDE];
            let mut pad = &mut dst_l[unit_w as usize + j * REST_UNIT_STRIDE..];
            BD::pixel_set(pad, row_last, 3);
        }
    }

    if have_left == 0 {
        // Pad 3x(STRIPE_H+6) with first column
        for j in 0..stripe_h as usize + 6 {
            let offset = j * REST_UNIT_STRIDE;
            let val = dst[3 + offset];
            BD::pixel_set(&mut dst[offset..], val, 3);
        }
    } else {
        let dst = &mut dst[3 * REST_UNIT_STRIDE..];

        for j in 0..stripe_h as usize {
            BD::pixel_copy(
                &mut dst[j * REST_UNIT_STRIDE..],
                &(*left.offset(j as isize))[1..],
                3,
            );
        }
    };
}

// TODO(randompoison): Temporarily public until init logic is deduplicated.
pub(crate) unsafe extern "C" fn wiener_c_erased<BD: BitDepth>(
    mut p: *mut libc::c_void,
    stride: ptrdiff_t,
    left: *const libc::c_void,
    mut lpf: *const libc::c_void,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: libc::c_int,
) {
    let bd = BD::from_c(bitdepth_max);
    wiener_rust::<BD>(
        p.cast(),
        stride,
        left.cast(),
        lpf.cast(),
        w,
        h,
        params,
        edges,
        bd,
    )
}

unsafe fn wiener_rust<BD: BitDepth>(
    mut p: *mut BD::Pixel,
    stride: ptrdiff_t,
    left: *const [BD::Pixel; 4],
    mut lpf: *const BD::Pixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bd: BD,
) {
    let mut tmp: [BD::Pixel; 27300] = [0.as_(); 27300];
    let mut tmp_ptr: *mut BD::Pixel = tmp.as_mut_ptr();

    padding::<BD>(&mut tmp, p, stride, left, lpf, w, h, edges);

    let mut hor: [uint16_t; 27300] = [0; 27300];
    let mut hor_ptr: *mut uint16_t = hor.as_mut_ptr();
    let filter: *const [int16_t; 8] = ((*params).filter.0).as_ptr();
    let bitdepth = bd.bitdepth().as_::<libc::c_int>();
    let round_bits_h = 3 as libc::c_int + (bitdepth == 12) as libc::c_int * 2;
    let rounding_off_h = (1 as libc::c_int) << round_bits_h - 1;
    let clip_limit = (1 as libc::c_int) << bitdepth + 1 + 7 - round_bits_h;
    let mut j = 0;
    while j < h + 6 {
        let mut i = 0;
        while i < w {
            let mut sum = (1 as libc::c_int) << bitdepth + 6;

            if BD::BITDEPTH == 8 {
                sum += (*tmp_ptr.offset((i + 3) as isize)).as_::<libc::c_int>() * 128;
            }

            let mut k = 0;
            while k < 7 {
                sum += (*tmp_ptr.offset((i + k) as isize)).as_::<libc::c_int>()
                    * (*filter.offset(0))[k as usize] as libc::c_int;
                k += 1;
            }
            *hor_ptr.offset(i as isize) = iclip(
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
    let round_bits_v = 11 as libc::c_int - (bitdepth == 12) as libc::c_int * 2;
    let rounding_off_v = (1 as libc::c_int) << round_bits_v - 1;
    let round_offset = (1 as libc::c_int) << bitdepth + (round_bits_v - 1);
    let mut j_0 = 0;
    while j_0 < h {
        let mut i_0 = 0;
        while i_0 < w {
            let mut sum_0 = -round_offset;
            let mut k_0 = 0;
            while k_0 < 7 {
                sum_0 += hor[((j_0 + k_0) * 390 + i_0) as usize] as libc::c_int
                    * (*filter.offset(1))[k_0 as usize] as libc::c_int;
                k_0 += 1;
            }
            *p.offset(j_0 as isize * BD::pxstride(stride as usize) as isize + i_0 as isize) =
                iclip(
                    sum_0 + rounding_off_v >> round_bits_v,
                    0 as libc::c_int,
                    bd.bitdepth_max().as_(),
                )
                .as_();
            i_0 += 1;
        }
        j_0 += 1;
    }
}

unsafe fn boxsum3<BD: BitDepth>(
    mut sumsq: *mut int32_t,
    mut sum: *mut BD::Coef,
    mut src: *const BD::Pixel,
    w: libc::c_int,
    h: libc::c_int,
) {
    src = src.offset(390);
    let mut x = 1;
    while x < w - 1 {
        let mut sum_v: *mut BD::Coef = sum.offset(x as isize);
        let mut sumsq_v: *mut int32_t = sumsq.offset(x as isize);
        let mut s: *const BD::Pixel = src.offset(x as isize);
        let mut a: libc::c_int = (*s.offset(0)).as_();
        let mut a2 = a * a;
        let mut b: libc::c_int = (*s.offset(390)).as_();
        let mut b2 = b * b;
        let mut y = 2;
        while y < h - 2 {
            s = s.offset(390);
            let c: libc::c_int = (*s.offset(390)).as_();
            let c2 = c * c;
            sum_v = sum_v.offset(390);
            sumsq_v = sumsq_v.offset(390);
            *sum_v = (a + b + c).as_();
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

unsafe fn boxsum5<BD: BitDepth>(
    mut sumsq: *mut int32_t,
    mut sum: *mut BD::Coef,
    src: *const BD::Pixel,
    w: libc::c_int,
    h: libc::c_int,
) {
    let mut x = 0;
    while x < w {
        let mut sum_v: *mut BD::Coef = sum.offset(x as isize);
        let mut sumsq_v: *mut int32_t = sumsq.offset(x as isize);
        let mut s: *const BD::Pixel = src.offset((3 * 390) as isize).offset(x as isize);
        let mut a: libc::c_int = (*s.offset((-(3 as libc::c_int) * 390) as isize)).as_();
        let mut a2 = a * a;
        let mut b: libc::c_int = (*s.offset((-(2 as libc::c_int) * 390) as isize)).as_();
        let mut b2 = b * b;
        let mut c: libc::c_int = (*s.offset((-(1 as libc::c_int) * 390) as isize)).as_();
        let mut c2 = c * c;
        let mut d: libc::c_int = (*s.offset(0)).as_();
        let mut d2 = d * d;
        let mut y = 2;
        while y < h - 2 {
            s = s.offset(390);
            let e: libc::c_int = (*s).as_();
            let e2 = e * e;
            sum_v = sum_v.offset(390);
            sumsq_v = sumsq_v.offset(390);
            *sum_v = (a + b + c + d + e).as_();
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

// TODO(randomPoison): Temporarily pub until callers are deduplicated.
#[inline(never)]
pub(crate) unsafe extern "C" fn selfguided_filter<BD: BitDepth>(
    mut dst: *mut BD::Coef,
    mut src: *const BD::Pixel,
    _src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    n: libc::c_int,
    s: libc::c_uint,
    bd: BD,
) {
    let sgr_one_by_x: libc::c_uint = (if n == 25 {
        164 as libc::c_int
    } else {
        455 as libc::c_int
    }) as libc::c_uint;
    let mut sumsq: [int32_t; 26520] = [0; 26520];
    let mut A: *mut int32_t = sumsq.as_mut_ptr().offset((2 * 390) as isize).offset(3);
    let mut sum: [BD::Coef; 26520] = [0.as_(); 26520];
    let mut B: *mut BD::Coef = sum.as_mut_ptr().offset((2 * 390) as isize).offset(3);
    let step = (n == 25) as libc::c_int + 1;
    if n == 25 {
        boxsum5::<BD>(sumsq.as_mut_ptr(), sum.as_mut_ptr(), src, w + 6, h + 6);
    } else {
        boxsum3::<BD>(sumsq.as_mut_ptr(), sum.as_mut_ptr(), src, w + 6, h + 6);
    }
    let bitdepth_min_8 = bd.bitdepth() - 8;
    let mut AA: *mut int32_t = A.offset(-(390 as libc::c_int as isize));
    let mut BB: *mut BD::Coef = B.offset(-(390 as libc::c_int as isize));
    let mut j = -(1 as libc::c_int);
    while j < h + 1 {
        let mut i = -(1 as libc::c_int);
        while i < w + 1 {
            let a = *AA.offset(i as isize) + ((1 as libc::c_int) << 2 * bitdepth_min_8 >> 1)
                >> 2 * bitdepth_min_8;
            let b = (*BB.offset(i as isize)).as_::<libc::c_int>()
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
                .wrapping_mul((*BB.offset(i as isize)).as_::<libc::c_uint>())
                .wrapping_mul(sgr_one_by_x)
                .wrapping_add(((1 as libc::c_int) << 11) as libc::c_uint)
                >> 12) as int32_t;
            *BB.offset(i as isize) = x.as_::<BD::Coef>();
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
                let a_0: libc::c_int = (*B.offset((i_0 - 390) as isize)
                    + (*B.offset((i_0 + 390) as isize)))
                .as_::<libc::c_int>()
                    * 6
                    + (*B.offset((i_0 - 1 - 390) as isize)
                        + *B.offset((i_0 - 1 + 390) as isize)
                        + *B.offset((i_0 + 1 - 390) as isize)
                        + *B.offset((i_0 + 1 + 390) as isize))
                    .as_::<libc::c_int>()
                        * 5;
                let b_0 = (*A.offset((i_0 - 390) as isize) + *A.offset((i_0 + 390) as isize)) * 6
                    + (*A.offset((i_0 - 1 - 390) as isize)
                        + *A.offset((i_0 - 1 + 390) as isize)
                        + *A.offset((i_0 + 1 - 390) as isize)
                        + *A.offset((i_0 + 1 + 390) as isize))
                        * 5;
                *dst.offset(i_0 as isize) = (b_0
                    - a_0 * (*src.offset(i_0 as isize)).as_::<libc::c_int>()
                    + ((1 as libc::c_int) << 8)
                    >> 9)
                    .as_();
                i_0 += 1;
            }
            dst = dst.offset(384);
            src = src.offset(390);
            B = B.offset(390);
            A = A.offset(390);
            let mut i_1 = 0;
            while i_1 < w {
                let a_1: libc::c_int = (*B.offset(i_1 as isize)).as_::<libc::c_int>() * 6
                    + (*B.offset((i_1 - 1) as isize) + *B.offset((i_1 + 1) as isize))
                        .as_::<libc::c_int>()
                        * 5;
                let b_1 = *A.offset(i_1 as isize) * 6
                    + (*A.offset((i_1 - 1) as isize) + *A.offset((i_1 + 1) as isize)) * 5;
                *dst.offset(i_1 as isize) = (b_1
                    - a_1 * (*src.offset(i_1 as isize)).as_::<libc::c_int>()
                    + ((1 as libc::c_int) << 7)
                    >> 8)
                    .as_();
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
                let a_2: libc::c_int = (*B.offset((i_2 - 390) as isize)
                    + *B.offset((i_2 + 390) as isize))
                .as_::<libc::c_int>()
                    * 6
                    + (*B.offset((i_2 - 1 - 390) as isize)
                        + *B.offset((i_2 - 1 + 390) as isize)
                        + *B.offset((i_2 + 1 - 390) as isize)
                        + *B.offset((i_2 + 1 + 390) as isize))
                    .as_::<libc::c_int>()
                        * 5;
                let b_2 = (*A.offset((i_2 - 390) as isize) + *A.offset((i_2 + 390) as isize)) * 6
                    + (*A.offset((i_2 - 1 - 390) as isize)
                        + *A.offset((i_2 - 1 + 390) as isize)
                        + *A.offset((i_2 + 1 - 390) as isize)
                        + *A.offset((i_2 + 1 + 390) as isize))
                        * 5;
                *dst.offset(i_2 as isize) = (b_2
                    - a_2 * (*src.offset(i_2 as isize)).as_::<libc::c_int>()
                    + ((1 as libc::c_int) << 8)
                    >> 9)
                    .as_();
                i_2 += 1;
            }
        }
    } else {
        let mut j_1 = 0;
        while j_1 < h {
            let mut i_3 = 0;
            while i_3 < w {
                let a_3: libc::c_int = (*B.offset(i_3 as isize)
                    + *B.offset((i_3 - 1) as isize)
                    + *B.offset((i_3 + 1) as isize)
                    + *B.offset((i_3 - 390) as isize)
                    + *B.offset((i_3 + 390) as isize))
                .as_::<libc::c_int>()
                    * 4
                    + (*B.offset((i_3 - 1 - 390) as isize)
                        + *B.offset((i_3 - 1 + 390) as isize)
                        + *B.offset((i_3 + 1 - 390) as isize)
                        + *B.offset((i_3 + 1 + 390) as isize))
                    .as_::<libc::c_int>()
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
                *dst.offset(i_3 as isize) = (b_3
                    - a_3 * (*src.offset(i_3 as isize)).as_::<libc::c_int>()
                    + ((1 as libc::c_int) << 8)
                    >> 9)
                    .as_();
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

// TODO(randomPoison): Temporarily pub until init logic is deduplicated.
pub(crate) unsafe extern "C" fn sgr_5x5_c_erased<BD: BitDepth>(
    p: *mut libc::c_void,
    stride: ptrdiff_t,
    left: *const libc::c_void,
    lpf: *const libc::c_void,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: libc::c_int,
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
        BD::from_c(bitdepth_max),
    )
}

unsafe fn sgr_5x5_rust<BD: BitDepth>(
    mut p: *mut BD::Pixel,
    stride: ptrdiff_t,
    left: *const [BD::Pixel; 4],
    lpf: *const BD::Pixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bd: BD,
) {
    let mut tmp: [BD::Pixel; 27300] = [0.as_(); 27300];
    let mut dst: [BD::Coef; 24576] = [0.as_(); 24576];
    padding::<BD>(&mut tmp, p, stride, left, lpf, w, h, edges);
    selfguided_filter(
        dst.as_mut_ptr(),
        tmp.as_mut_ptr(),
        390 as libc::c_int as ptrdiff_t,
        w,
        h,
        25 as libc::c_int,
        (*params).sgr.s0,
        bd,
    );
    let w0 = (*params).sgr.w0 as libc::c_int;
    let mut j = 0;
    while j < h {
        let mut i = 0;
        while i < w {
            let v = w0 * dst[(j * 384 + i) as usize].as_::<libc::c_int>();
            *p.offset(i as isize) = bd.iclip_pixel(
                (*p.offset(i as isize)).as_::<libc::c_int>()
                    + (v + ((1 as libc::c_int) << 10) >> 11),
            );
            i += 1;
        }
        p = p.offset(BD::pxstride(stride as usize) as isize);
        j += 1;
    }
}

// TODO(randomPoison): Temporarily pub until init logic is deduplicated.
pub(crate) unsafe extern "C" fn sgr_3x3_c_erased<BD: BitDepth>(
    p: *mut libc::c_void,
    stride: ptrdiff_t,
    left: *const libc::c_void,
    lpf: *const libc::c_void,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: libc::c_int,
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
        BD::from_c(bitdepth_max),
    )
}

unsafe fn sgr_3x3_rust<BD: BitDepth>(
    mut p: *mut BD::Pixel,
    stride: ptrdiff_t,
    left: *const [BD::Pixel; 4],
    lpf: *const BD::Pixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bd: BD,
) {
    let mut tmp: [BD::Pixel; 27300] = [0.as_(); 27300];
    let mut dst: [BD::Coef; 24576] = [0.as_(); 24576];
    padding::<BD>(&mut tmp, p, stride, left, lpf, w, h, edges);
    selfguided_filter(
        dst.as_mut_ptr(),
        tmp.as_mut_ptr(),
        390 as libc::c_int as ptrdiff_t,
        w,
        h,
        9 as libc::c_int,
        (*params).sgr.s1,
        bd,
    );
    let w1 = (*params).sgr.w1 as libc::c_int;
    let mut j = 0;
    while j < h {
        let mut i = 0;
        while i < w {
            let v = w1 * dst[(j * 384 + i) as usize].as_::<libc::c_int>();
            *p.offset(i as isize) = bd.iclip_pixel(
                (*p.offset(i as isize)).as_::<libc::c_int>()
                    + (v + ((1 as libc::c_int) << 10) >> 11),
            );
            i += 1;
        }
        p = p.offset(BD::pxstride(stride as usize) as isize);
        j += 1;
    }
}

// TODO(randomPoison): Temporarily pub until init logic is deduplicated.
pub(crate) unsafe extern "C" fn sgr_mix_c_erased<BD: BitDepth>(
    p: *mut libc::c_void,
    stride: ptrdiff_t,
    left: *const libc::c_void,
    lpf: *const libc::c_void,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: libc::c_int,
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
        BD::from_c(bitdepth_max),
    )
}

unsafe fn sgr_mix_rust<BD: BitDepth>(
    mut p: *mut BD::Pixel,
    stride: ptrdiff_t,
    left: *const [BD::Pixel; 4],
    lpf: *const BD::Pixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bd: BD,
) {
    let mut tmp: [BD::Pixel; 27300] = [0.as_(); 27300];
    let mut dst0: [BD::Coef; 24576] = [0.as_(); 24576];
    let mut dst1: [BD::Coef; 24576] = [0.as_(); 24576];
    padding::<BD>(&mut tmp, p, stride, left, lpf, w, h, edges);
    selfguided_filter(
        dst0.as_mut_ptr(),
        tmp.as_mut_ptr(),
        390 as libc::c_int as ptrdiff_t,
        w,
        h,
        25 as libc::c_int,
        (*params).sgr.s0,
        bd,
    );
    selfguided_filter(
        dst1.as_mut_ptr(),
        tmp.as_mut_ptr(),
        390 as libc::c_int as ptrdiff_t,
        w,
        h,
        9 as libc::c_int,
        (*params).sgr.s1,
        bd,
    );
    let w0 = (*params).sgr.w0 as libc::c_int;
    let w1 = (*params).sgr.w1 as libc::c_int;
    let mut j = 0;
    while j < h {
        let mut i = 0;
        while i < w {
            let v = w0 * dst0[(j * 384 + i) as usize].as_::<libc::c_int>()
                + w1 * dst1[(j * 384 + i) as usize].as_::<libc::c_int>();
            *p.offset(i as isize) = bd.iclip_pixel(
                (*p.offset(i as isize)).as_::<libc::c_int>()
                    + (v + ((1 as libc::c_int) << 10) >> 11),
            );
            i += 1;
        }
        p = p.offset(BD::pxstride(stride as usize) as isize);
        j += 1;
    }
}

// TODO(randomPoison): Temporarily pub until all usages can be made private.
#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
pub(crate) trait BitDepthLooprestorationArm: BitDepth {
    fn dav1d_sgr_box3_h_neon(
        sumsq: *mut int32_t,
        sum: *mut int16_t,
        left: *const [Self::Pixel; 4],
        src: *const Self::Pixel,
        stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        edges: LrEdgeFlags,
    );
    fn dav1d_sgr_box5_h_neon(
        sumsq: *mut int32_t,
        sum: *mut int16_t,
        left: *const [Self::Pixel; 4],
        src: *const Self::Pixel,
        stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        edges: LrEdgeFlags,
    );
    fn dav1d_sgr_finish_filter1_neon(
        tmp: *mut int16_t,
        src: *const Self::Pixel,
        stride: ptrdiff_t,
        a: *const int32_t,
        b: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn dav1d_sgr_finish_filter2_neon(
        tmp: *mut int16_t,
        src: *const Self::Pixel,
        stride: ptrdiff_t,
        a: *const int32_t,
        b: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
    );
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
impl BitDepthLooprestorationArm for BitDepth8 {
    #[inline(always)]
    fn dav1d_sgr_box3_h_neon(
        sumsq: *mut int32_t,
        sum: *mut int16_t,
        left: *const [Self::Pixel; 4],
        src: *const Self::Pixel,
        stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        edges: LrEdgeFlags,
    ) {
        extern "C" {
            fn dav1d_sgr_box3_h_8bpc_neon(
                sumsq: *mut int32_t,
                sum: *mut int16_t,
                left: *const [<BitDepth8 as BitDepth>::Pixel; 4],
                src: *const <BitDepth8 as BitDepth>::Pixel,
                stride: ptrdiff_t,
                w: libc::c_int,
                h: libc::c_int,
                edges: LrEdgeFlags,
            );
        }

        unsafe { dav1d_sgr_box3_h_8bpc_neon(sumsq, sum, left, src, stride, w, h, edges) }
    }

    #[inline(always)]
    fn dav1d_sgr_box5_h_neon(
        sumsq: *mut int32_t,
        sum: *mut int16_t,
        left: *const [Self::Pixel; 4],
        src: *const Self::Pixel,
        stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        edges: LrEdgeFlags,
    ) {
        extern "C" {
            fn dav1d_sgr_box5_h_8bpc_neon(
                sumsq: *mut int32_t,
                sum: *mut int16_t,
                left: *const [<BitDepth8 as BitDepth>::Pixel; 4],
                src: *const <BitDepth8 as BitDepth>::Pixel,
                stride: ptrdiff_t,
                w: libc::c_int,
                h: libc::c_int,
                edges: LrEdgeFlags,
            );
        }

        unsafe { dav1d_sgr_box5_h_8bpc_neon(sumsq, sum, left, src, stride, w, h, edges) }
    }

    #[inline(always)]
    fn dav1d_sgr_finish_filter1_neon(
        tmp: *mut int16_t,
        src: *const Self::Pixel,
        stride: ptrdiff_t,
        a: *const int32_t,
        b: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
    ) {
        extern "C" {
            fn dav1d_sgr_finish_filter1_8bpc_neon(
                tmp: *mut int16_t,
                src: *const <BitDepth8 as BitDepth>::Pixel,
                stride: ptrdiff_t,
                a: *const int32_t,
                b: *const int16_t,
                w: libc::c_int,
                h: libc::c_int,
            );
        }

        unsafe { dav1d_sgr_finish_filter1_8bpc_neon(tmp, src, stride, a, b, w, h) }
    }

    #[inline(always)]
    fn dav1d_sgr_finish_filter2_neon(
        tmp: *mut int16_t,
        src: *const Self::Pixel,
        stride: ptrdiff_t,
        a: *const int32_t,
        b: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
    ) {
        extern "C" {
            fn dav1d_sgr_finish_filter2_8bpc_neon(
                tmp: *mut int16_t,
                src: *const <BitDepth8 as BitDepth>::Pixel,
                stride: ptrdiff_t,
                a: *const int32_t,
                b: *const int16_t,
                w: libc::c_int,
                h: libc::c_int,
            );
        }

        unsafe { dav1d_sgr_finish_filter2_8bpc_neon(tmp, src, stride, a, b, w, h) }
    }
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
impl BitDepthLooprestorationArm for BitDepth16 {
    #[inline(always)]
    fn dav1d_sgr_box3_h_neon(
        sumsq: *mut int32_t,
        sum: *mut int16_t,
        left: *const [Self::Pixel; 4],
        src: *const Self::Pixel,
        stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        edges: LrEdgeFlags,
    ) {
        extern "C" {
            fn dav1d_sgr_box3_h_16bpc_neon(
                sumsq: *mut int32_t,
                sum: *mut int16_t,
                left: *const [<BitDepth16 as BitDepth>::Pixel; 4],
                src: *const <BitDepth16 as BitDepth>::Pixel,
                stride: ptrdiff_t,
                w: libc::c_int,
                h: libc::c_int,
                edges: LrEdgeFlags,
            );
        }

        unsafe { dav1d_sgr_box3_h_16bpc_neon(sumsq, sum, left, src, stride, w, h, edges) }
    }

    #[inline(always)]
    fn dav1d_sgr_box5_h_neon(
        sumsq: *mut int32_t,
        sum: *mut int16_t,
        left: *const [Self::Pixel; 4],
        src: *const Self::Pixel,
        stride: ptrdiff_t,
        w: libc::c_int,
        h: libc::c_int,
        edges: LrEdgeFlags,
    ) {
        extern "C" {
            fn dav1d_sgr_box5_h_16bpc_neon(
                sumsq: *mut int32_t,
                sum: *mut int16_t,
                left: *const [<BitDepth16 as BitDepth>::Pixel; 4],
                src: *const <BitDepth16 as BitDepth>::Pixel,
                stride: ptrdiff_t,
                w: libc::c_int,
                h: libc::c_int,
                edges: LrEdgeFlags,
            );
        }

        unsafe { dav1d_sgr_box5_h_16bpc_neon(sumsq, sum, left, src, stride, w, h, edges) }
    }

    #[inline(always)]
    fn dav1d_sgr_finish_filter1_neon(
        tmp: *mut int16_t,
        src: *const Self::Pixel,
        stride: ptrdiff_t,
        a: *const int32_t,
        b: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
    ) {
        extern "C" {
            fn dav1d_sgr_finish_filter1_16bpc_neon(
                tmp: *mut int16_t,
                src: *const <BitDepth16 as BitDepth>::Pixel,
                stride: ptrdiff_t,
                a: *const int32_t,
                b: *const int16_t,
                w: libc::c_int,
                h: libc::c_int,
            );
        }

        unsafe { dav1d_sgr_finish_filter1_16bpc_neon(tmp, src, stride, a, b, w, h) }
    }

    #[inline(always)]
    fn dav1d_sgr_finish_filter2_neon(
        tmp: *mut int16_t,
        src: *const Self::Pixel,
        stride: ptrdiff_t,
        a: *const int32_t,
        b: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
    ) {
        extern "C" {
            fn dav1d_sgr_finish_filter2_16bpc_neon(
                tmp: *mut int16_t,
                src: *const <BitDepth16 as BitDepth>::Pixel,
                stride: ptrdiff_t,
                a: *const int32_t,
                b: *const int16_t,
                w: libc::c_int,
                h: libc::c_int,
            );
        }

        unsafe { dav1d_sgr_finish_filter2_16bpc_neon(tmp, src, stride, a, b, w, h) }
    }
}

// TODO(randomPoison): Temporarily pub until callers are deduplicated.
#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
pub(crate) unsafe fn dav1d_sgr_filter1_neon<BD: BitDepthLooprestorationArm>(
    mut tmp: *mut int16_t,
    mut src: *const BD::Pixel,
    stride: ptrdiff_t,
    mut left: *const [BD::Pixel; 4],
    mut lpf: *const BD::Pixel,
    w: libc::c_int,
    h: libc::c_int,
    strength: libc::c_int,
    edges: LrEdgeFlags,
    bd: BD,
) {
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
    BD::dav1d_sgr_box3_h_neon(sumsq, sum, left, src, stride, w, h, edges);
    if edges as libc::c_uint & LR_HAVE_TOP as libc::c_int as libc::c_uint != 0 {
        BD::dav1d_sgr_box3_h_neon(
            &mut *sumsq.offset((-(2 as libc::c_int) * (384 + 16)) as isize),
            &mut *sum.offset((-(2 as libc::c_int) * (384 + 16)) as isize),
            0 as *const [BD::Pixel; 4],
            lpf,
            stride,
            w,
            2 as libc::c_int,
            edges,
        );
    }
    if edges as libc::c_uint & LR_HAVE_BOTTOM as libc::c_int as libc::c_uint != 0 {
        BD::dav1d_sgr_box3_h_neon(
            &mut *sumsq.offset((h * (384 + 16)) as isize),
            &mut *sum.offset((h * (384 + 16)) as isize),
            0 as *const [BD::Pixel; 4],
            lpf.offset((6 * BD::pxstride(stride as usize)) as isize),
            stride,
            w,
            2 as libc::c_int,
            edges,
        );
    }
    dav1d_sgr_box3_v_neon(sumsq, sum, w, h, edges);
    dav1d_sgr_calc_ab1_neon(a, b, w, h, strength, bd.bitdepth_max().as_());
    BD::dav1d_sgr_finish_filter1_neon(tmp, src, stride, a, b, w, h);
}

// TODO(randomPoison): Temporarily pub until callers are deduplicated.
#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
pub(crate) unsafe fn dav1d_sgr_filter2_neon<BD: BitDepthLooprestorationArm>(
    mut tmp: *mut int16_t,
    mut src: *const BD::Pixel,
    stride: ptrdiff_t,
    mut left: *const [BD::Pixel; 4],
    mut lpf: *const BD::Pixel,
    w: libc::c_int,
    h: libc::c_int,
    strength: libc::c_int,
    edges: LrEdgeFlags,
    bd: BD,
) {
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
    BD::dav1d_sgr_box5_h_neon(sumsq, sum, left, src, stride, w, h, edges);
    if edges as libc::c_uint & LR_HAVE_TOP as libc::c_int as libc::c_uint != 0 {
        BD::dav1d_sgr_box5_h_neon(
            &mut *sumsq.offset((-(2 as libc::c_int) * (384 + 16)) as isize),
            &mut *sum.offset((-(2 as libc::c_int) * (384 + 16)) as isize),
            0 as *const [BD::Pixel; 4],
            lpf,
            stride,
            w,
            2 as libc::c_int,
            edges,
        );
    }
    if edges as libc::c_uint & LR_HAVE_BOTTOM as libc::c_int as libc::c_uint != 0 {
        BD::dav1d_sgr_box5_h_neon(
            &mut *sumsq.offset((h * (384 + 16)) as isize),
            &mut *sum.offset((h * (384 + 16)) as isize),
            0 as *const [BD::Pixel; 4],
            lpf.offset((6 * BD::pxstride(stride as usize)) as isize),
            stride,
            w,
            2 as libc::c_int,
            edges,
        );
    }
    dav1d_sgr_box5_v_neon(sumsq, sum, w, h, edges);
    dav1d_sgr_calc_ab2_neon(a, b, w, h, strength, bd.bitdepth_max().as_());
    BD::dav1d_sgr_finish_filter2_neon(tmp, src, stride, a, b, w, h);
}
