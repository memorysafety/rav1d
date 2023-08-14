use std::ops::Add;

#[cfg(all(
    feature = "asm",
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")
))]
use crate::include::common::bitdepth::bd_fn;
use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::bitdepth::LeftPixelRow;
use crate::include::common::bitdepth::ToPrimitive;
use crate::include::common::bitdepth::BPC;
use crate::include::common::intops::iclip;
use crate::include::common::intops::imax;
use crate::include::common::intops::umin;
use crate::include::stddef::ptrdiff_t;
use crate::include::stdint::int16_t;
use crate::include::stdint::int32_t;
#[cfg(all(feature = "asm", target_arch = "arm"))]
use crate::include::stdint::intptr_t;
use crate::include::stdint::uint16_t;
use crate::include::stdint::uint32_t;
use crate::src::align::Align16;
use crate::src::cursor::CursorMut;
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

pub type looprestorationfilter_fn = unsafe extern "C" fn(
    *mut DynPixel,
    ptrdiff_t,
    *const LeftPixelRow<DynPixel>,
    *const DynPixel,
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

#[cfg(all(
    feature = "asm",
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")
))]
macro_rules! decl_looprestorationfilter_fn {
    (fn $name:ident) => {{
        extern "C" {
            fn $name(
                dst: *mut DynPixel,
                dst_stride: ptrdiff_t,
                left: *const LeftPixelRow<DynPixel>,
                lpf: *const DynPixel,
                w: libc::c_int,
                h: libc::c_int,
                params: *const LooprestorationParams,
                edges: LrEdgeFlags,
                bitdepth_max: libc::c_int,
            );
        }

        $name
    }};
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
#[inline(never)]
unsafe fn padding<BD: BitDepth>(
    dst: &mut [BD::Pixel; 70 /*(64 + 3 + 3)*/ * REST_UNIT_STRIDE],
    p: *const BD::Pixel,
    stride: usize,
    left: *const [BD::Pixel; 4],
    lpf: *const BD::Pixel,
    unit_w: usize,
    stripe_h: usize,
    edges: LrEdgeFlags,
) {
    let stride = BD::pxstride(stride as usize);

    let [have_left, have_right, have_top, have_bottom] =
        [LR_HAVE_LEFT, LR_HAVE_RIGHT, LR_HAVE_TOP, LR_HAVE_BOTTOM]
            .map(|lr_have| edges & lr_have != 0);
    let [have_left_3, have_right_3] = [have_left, have_right].map(|have| 3 * have as usize);

    // Copy more pixels if we don't have to pad them
    let unit_w = unit_w + have_left_3 + have_right_3;
    let dst_l = &mut dst[3 - have_left_3..];
    let p = p.offset(-(have_left_3 as isize));
    let lpf = lpf.offset(-(have_left_3 as isize));

    if have_top {
        // Copy previous loop filtered rows
        let above_1 = std::slice::from_raw_parts(lpf, stride + unit_w);
        let above_2 = &above_1[stride..];
        BD::pixel_copy(dst_l, above_1, unit_w);
        BD::pixel_copy(&mut dst_l[REST_UNIT_STRIDE..], above_1, unit_w);
        BD::pixel_copy(&mut dst_l[2 * REST_UNIT_STRIDE..], above_2, unit_w);
    } else {
        // Pad with first row
        let p = std::slice::from_raw_parts(p, unit_w);
        BD::pixel_copy(dst_l, p, unit_w);
        BD::pixel_copy(&mut dst_l[REST_UNIT_STRIDE..], p, unit_w);
        BD::pixel_copy(&mut dst_l[2 * REST_UNIT_STRIDE..], p, unit_w);
        if have_left {
            let left = &(*left.offset(0))[1..];
            BD::pixel_copy(dst_l, left, 3);
            BD::pixel_copy(&mut dst_l[REST_UNIT_STRIDE..], left, 3);
            BD::pixel_copy(&mut dst_l[2 * REST_UNIT_STRIDE..], left, 3);
        }
    }

    let dst_tl = &mut dst_l[3 * REST_UNIT_STRIDE..];
    if have_bottom {
        // Copy next loop filtered rows
        let lpf = std::slice::from_raw_parts(lpf, 7 * stride + unit_w);
        let below_1 = &lpf[6 * stride..];
        let below_2 = &below_1[stride..];
        BD::pixel_copy(&mut dst_tl[stripe_h * REST_UNIT_STRIDE..], below_1, unit_w);
        BD::pixel_copy(
            &mut dst_tl[(stripe_h + 1) * REST_UNIT_STRIDE..],
            below_2,
            unit_w,
        );
        BD::pixel_copy(
            &mut dst_tl[(stripe_h + 2) * REST_UNIT_STRIDE..],
            below_2,
            unit_w,
        );
    } else {
        // Pad with last row
        let p = std::slice::from_raw_parts(p, (stripe_h - 1) * stride + unit_w);
        let src = &p[(stripe_h - 1) * stride..];
        BD::pixel_copy(&mut dst_tl[stripe_h * REST_UNIT_STRIDE..], src, unit_w);
        BD::pixel_copy(
            &mut dst_tl[(stripe_h + 1) * REST_UNIT_STRIDE..],
            src,
            unit_w,
        );
        BD::pixel_copy(
            &mut dst_tl[(stripe_h + 2) * REST_UNIT_STRIDE..],
            src,
            unit_w,
        );
        if have_left {
            let left = &(*left.offset((stripe_h - 1) as isize))[1..];
            BD::pixel_copy(&mut dst_tl[stripe_h * REST_UNIT_STRIDE..], left, 3);
            BD::pixel_copy(&mut dst_tl[(stripe_h + 1) * REST_UNIT_STRIDE..], left, 3);
            BD::pixel_copy(&mut dst_tl[(stripe_h + 2) * REST_UNIT_STRIDE..], left, 3);
        }
    }

    // Inner UNIT_WxSTRIPE_H
    let len = unit_w - have_left_3;
    let p = std::slice::from_raw_parts(
        p,
        if stripe_h == 0 {
            0
        } else {
            have_left_3 + (stripe_h - 1) * stride + len
        },
    );
    for j in 0..stripe_h {
        BD::pixel_copy(
            &mut dst_tl[j * REST_UNIT_STRIDE + have_left_3..],
            &p[j * stride + have_left_3..],
            len,
        );
    }

    if !have_right {
        // Pad 3x(STRIPE_H+6) with last column
        for j in 0..stripe_h + 6 {
            let mut row_last = dst_l[(unit_w - 1) + j * REST_UNIT_STRIDE];
            let mut pad = &mut dst_l[unit_w + j * REST_UNIT_STRIDE..];
            BD::pixel_set(pad, row_last, 3);
        }
    }

    if !have_left {
        // Pad 3x(STRIPE_H+6) with first column
        for j in 0..stripe_h + 6 {
            let offset = j * REST_UNIT_STRIDE;
            // This would be `dst_l[offset]` in C,
            // but that results in multiple mutable borrows of `dst`,
            // so we recalculate `dst_l` here.
            // `3 * (have_left == 0) as libc::c_int` simplifies to `3 * 1` and then `3`.
            let val = dst[3 + offset];
            BD::pixel_set(&mut dst[offset..], val, 3);
        }
    } else {
        let dst = &mut dst[3 * REST_UNIT_STRIDE..];
        let left = std::slice::from_raw_parts(left, stripe_h);
        for j in 0..stripe_h {
            BD::pixel_copy(&mut dst[j * REST_UNIT_STRIDE..], &left[j][1..], 3);
        }
    };
}

unsafe extern "C" fn wiener_c_erased<BD: BitDepth>(
    p: *mut DynPixel,
    stride: ptrdiff_t,
    left: *const LeftPixelRow<DynPixel>,
    lpf: *const DynPixel,
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
        &*params,
        edges,
        bd,
    )
}

// FIXME Could split into luma and chroma specific functions,
// (since first and last tops are always 0 for chroma)
// FIXME Could implement a version that requires less temporary memory
// (should be possible to implement with only 6 rows of temp storage)
unsafe fn wiener_rust<BD: BitDepth>(
    p: *mut BD::Pixel,
    stride: ptrdiff_t,
    left: *const [BD::Pixel; 4],
    lpf: *const BD::Pixel,
    w: libc::c_int,
    h: libc::c_int,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bd: BD,
) {
    // Wiener filtering is applied to a maximum stripe height of 64 + 3 pixels
    // of padding above and below
    let mut tmp = [0.into(); 70 /*(64 + 3 + 3)*/ * REST_UNIT_STRIDE];

    padding::<BD>(
        &mut tmp,
        p,
        stride as usize,
        left,
        lpf,
        w as usize,
        h as usize,
        edges,
    );

    // Values stored between horizontal and vertical filtering don't
    // fit in a u8.
    let mut hor = [0; 70 /*(64 + 3 + 3)*/ * REST_UNIT_STRIDE];

    let filter = &params.filter.0;
    let bitdepth = bd.bitdepth().as_::<libc::c_int>();
    let round_bits_h = 3 + (bitdepth == 12) as libc::c_int * 2;
    let rounding_off_h = 1 << round_bits_h - 1;
    let clip_limit = 1 << bitdepth + 1 + 7 - round_bits_h;
    for (tmp, hor) in tmp
        .chunks_exact(REST_UNIT_STRIDE)
        .zip(hor.chunks_exact_mut(REST_UNIT_STRIDE))
        .take((h + 6) as usize)
    {
        for i in 0..w as usize {
            let mut sum = 1 << bitdepth + 6;

            if BD::BPC == BPC::BPC8 {
                sum += tmp[i + 3].into() * 128;
            }

            for (&tmp, &filter) in std::iter::zip(&tmp[i..i + 7], &filter[0][..7]) {
                sum += tmp.into() * filter as libc::c_int;
            }

            hor[i] = iclip(sum + rounding_off_h >> round_bits_h, 0, clip_limit - 1) as uint16_t;
        }
    }

    let round_bits_v = 11 - (bitdepth == 12) as libc::c_int * 2;
    let rounding_off_v = 1 << round_bits_v - 1;
    let round_offset = 1 << bitdepth + (round_bits_v - 1);
    for j in 0..h {
        for i in 0..w {
            let mut sum = -round_offset;

            for k in 0..7 {
                sum += hor[((j + k) * REST_UNIT_STRIDE as libc::c_int + i) as usize] as libc::c_int
                    * filter[1][k as usize] as libc::c_int;
            }

            *p.offset(j as isize * BD::pxstride(stride as usize) as isize + i as isize) =
                iclip(sum + rounding_off_v >> round_bits_v, 0, bd.into_c()).as_();
        }
    }
}

/// Sum over a 3x3 area
///
/// The `dst` and `src` pointers are positioned 3 pixels above and 3 pixels to the
/// left of the top left corner. However, the self guided filter only needs 1
/// pixel above and one pixel to the left. As for the pixels below and to the
/// right they must be computed in the sums, but don't need to be stored.
///
/// Example for a 4x4 block:
///
///     x x x x x x x x x x
///     x c c c c c c c c x
///     x i s s s s s s i x
///     x i s s s s s s i x
///     x i s s s s s s i x
///     x i s s s s s s i x
///     x i s s s s s s i x
///     x i s s s s s s i x
///     x c c c c c c c c x
///     x x x x x x x x x x
///
/// * s: Pixel summed and stored
/// * i: Pixel summed and stored (between loops)
/// * c: Pixel summed not stored
/// * x: Pixel not summed not stored
fn boxsum3<BD: BitDepth>(
    mut sumsq: &mut [int32_t; 68 /*(64 + 2 + 2)*/ * REST_UNIT_STRIDE],
    mut sum: &mut [BD::Coef; 68 /*(64 + 2 + 2)*/ * REST_UNIT_STRIDE],
    mut src: &[BD::Pixel; 70 /*(64 + 3 + 3)*/ * REST_UNIT_STRIDE],
    w: libc::c_int,
    h: libc::c_int,
) {
    // We skip the first row, as it is never used
    let mut src = &src[REST_UNIT_STRIDE..];

    // We skip the first and last columns, as they are never used
    for x in 1..w - 1 {
        let mut sum_v = &mut sum[x as usize..];
        let mut sumsq_v = &mut sumsq[x as usize..];
        let mut s = &src[x as usize..];
        let mut a: libc::c_int = s[0].as_();
        let mut a2 = a * a;
        let mut b: libc::c_int = s[REST_UNIT_STRIDE].as_();
        let mut b2 = b * b;

        // We skip the first 2 rows, as they are skipped in the next loop and
        // we don't need the last 2 row as it is skipped in the next loop
        for _ in 2..h - 2 {
            s = &s[REST_UNIT_STRIDE..];
            let c: libc::c_int = s[REST_UNIT_STRIDE].as_();
            let c2 = c * c;
            sum_v = &mut sum_v[REST_UNIT_STRIDE..];
            sumsq_v = &mut sumsq_v[REST_UNIT_STRIDE..];
            sum_v[0] = (a + b + c).as_();
            sumsq_v[0] = a2 + b2 + c2;
            a = b;
            a2 = b2;
            b = c;
            b2 = c2;
        }
    }

    // We skip the first row as it is never read
    let mut sum = &mut sum[REST_UNIT_STRIDE..];
    let mut sumsq = &mut sumsq[REST_UNIT_STRIDE..];

    // We skip the last 2 rows as it is never read
    for _ in 2..h - 2 {
        let mut a = sum[1];
        let mut a2 = sumsq[1];
        let mut b = sum[2];
        let mut b2 = sumsq[2];

        // We don't store the first column as it is never read and
        // we don't store the last 2 columns as they are never read
        for x in 2..w as usize - 2 {
            let c = sum[x + 1];
            let c2 = sumsq[x + 1];
            sum[x] = a + b + c;
            sumsq[x] = a2 + b2 + c2;
            a = b;
            a2 = b2;
            b = c;
            b2 = c2;
        }

        sum = &mut sum[REST_UNIT_STRIDE..];
        sumsq = &mut sumsq[REST_UNIT_STRIDE..];
    }
}

/// Sum over a 5x5 area
///
/// The `dst` and `src` pointers are positioned 3 pixels above and 3 pixels to the
/// left of the top left corner. However, the self guided filter only needs 1
/// pixel above and one pixel to the left. As for the pixels below and to the
/// right they must be computed in the sums, but don't need to be stored.
///
/// Example for a 4x4 block:
///
///     c c c c c c c c c c
///     c c c c c c c c c c
///     i i s s s s s s i i
///     i i s s s s s s i i
///     i i s s s s s s i i
///     i i s s s s s s i i
///     i i s s s s s s i i
///     i i s s s s s s i i
///     c c c c c c c c c c
///     c c c c c c c c c c
///
/// * s: Pixel summed and stored
/// * i: Pixel summed and stored (between loops)
/// * c: Pixel summed not stored
/// * x: Pixel not summed not stored
fn boxsum5<BD: BitDepth>(
    mut sumsq: &mut [int32_t; 68 /*(64 + 2 + 2)*/ * REST_UNIT_STRIDE],
    mut sum: &mut [BD::Coef; 68 /*(64 + 2 + 2)*/ * REST_UNIT_STRIDE],
    src: &[BD::Pixel; 70 /*(64 + 3 + 3)*/ * REST_UNIT_STRIDE],
    w: libc::c_int,
    h: libc::c_int,
) {
    for x in 0..w as usize {
        let mut sum_v = &mut sum[x..];
        let mut sumsq_v = &mut sumsq[x..];
        let mut s = &src[x..];
        let mut a: libc::c_int = (s[0]).as_();
        let mut a2 = a * a;
        let mut b: libc::c_int = (s[1 * REST_UNIT_STRIDE]).as_();
        let mut b2 = b * b;
        let mut c: libc::c_int = (s[2 * REST_UNIT_STRIDE]).as_();
        let mut c2 = c * c;
        let mut d: libc::c_int = (s[3 * REST_UNIT_STRIDE]).as_();
        let mut d2 = d * d;

        let mut s = &src[3 * REST_UNIT_STRIDE + x..];

        // We skip the first 2 rows, as they are skipped in the next loop and
        // we don't need the last 2 row as it is skipped in the next loop
        for _ in 2..h - 2 {
            s = &s[REST_UNIT_STRIDE..];
            let e: libc::c_int = s[0].as_();
            let e2 = e * e;
            sum_v = &mut sum_v[REST_UNIT_STRIDE..];
            sumsq_v = &mut sumsq_v[REST_UNIT_STRIDE..];
            sum_v[0] = (a + b + c + d + e).as_();
            sumsq_v[0] = a2 + b2 + c2 + d2 + e2;
            a = b;
            b = c;
            c = d;
            d = e;
            a2 = b2;
            b2 = c2;
            c2 = d2;
            d2 = e2;
        }
    }

    // We skip the first row as it is never read
    let mut sum = &mut sum[REST_UNIT_STRIDE..];
    let mut sumsq = &mut sumsq[REST_UNIT_STRIDE..];
    for _ in 2..h - 2 {
        let mut a = sum[0];
        let mut a2 = sumsq[0];
        let mut b = sum[1];
        let mut b2 = sumsq[1];
        let mut c = sum[2];
        let mut c2 = sumsq[2];
        let mut d = sum[3];
        let mut d2 = sumsq[3];

        for x in 2..w as usize - 2 {
            let e = sum[x + 2];
            let e2 = sumsq[x + 2];
            sum[x] = a + b + c + d + e;
            sumsq[x] = a2 + b2 + c2 + d2 + e2;
            a = b;
            b = c;
            c = d;
            d = e;
            a2 = b2;
            b2 = c2;
            c2 = d2;
            d2 = e2;
        }
        sum = &mut sum[REST_UNIT_STRIDE..];
        sumsq = &mut sumsq[REST_UNIT_STRIDE..];
    }
}

#[inline(never)]
fn selfguided_filter<BD: BitDepth>(
    mut dst: &mut [BD::Coef; 24576],
    mut src: &[BD::Pixel; 27300],
    _src_stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    n: libc::c_int,
    s: libc::c_uint,
    bd: BD,
) {
    let sgr_one_by_x = if n == 25 { 164 } else { 455 };

    // Selfguided filter is applied to a maximum stripe height of 64 + 3 pixels
    // of padding above and below
    let mut sumsq = [0; 68 /*(64 + 2 + 2)*/ * REST_UNIT_STRIDE];
    // By inverting A and B after the boxsums, B can be of size coef instead
    // of int32_t
    let mut sum = [0.as_::<BD::Coef>(); 68 /*(64 + 2 + 2)*/ * REST_UNIT_STRIDE];

    let step = (n == 25) as libc::c_int + 1;
    if n == 25 {
        boxsum5::<BD>(&mut sumsq, &mut sum, src, w + 6, h + 6);
    } else {
        boxsum3::<BD>(&mut sumsq, &mut sum, src, w + 6, h + 6);
    }
    let bitdepth_min_8 = bd.bitdepth() - 8;

    let mut A = CursorMut::new(&mut sumsq) + 2 * REST_UNIT_STRIDE + 3;
    let mut B = CursorMut::new(&mut sum) + 2 * REST_UNIT_STRIDE + 3;

    let mut AA = A.clone() - REST_UNIT_STRIDE;
    let mut BB = B.clone() - REST_UNIT_STRIDE;
    for _ in (-1..h + 1).step_by(step as usize) {
        for i in -1..w + 1 {
            let a = AA[i] + (1 << 2 * bitdepth_min_8 >> 1) >> 2 * bitdepth_min_8;
            let b = BB[i].as_::<libc::c_int>() + (1 << bitdepth_min_8 >> 1) >> bitdepth_min_8;

            let p = imax(a * n - b * b, 0) as libc::c_uint;
            let z = (p * s + (1 << 19)) >> 20;
            let x = dav1d_sgr_x_by_x[umin(z, 255) as usize] as libc::c_uint;

            // This is where we invert A and B, so that B is of size coef.
            AA[i] =
                ((x * BB[i].as_::<libc::c_uint>() * sgr_one_by_x + (1 << 11)) >> 12) as libc::c_int;
            BB[i] = x.as_::<BD::Coef>();
        }
        AA += step as usize * REST_UNIT_STRIDE;
        BB += step as usize * REST_UNIT_STRIDE;
    }

    fn six_neighbors<P>(p: &CursorMut<P>, i: isize) -> libc::c_int
    where
        P: Add<Output = P> + ToPrimitive<libc::c_int> + Copy,
    {
        let stride = REST_UNIT_STRIDE as isize;
        (p[i - stride] + p[i + stride]).as_::<libc::c_int>() * 6
            + (p[i - 1 - stride] + p[i - 1 + stride] + p[i + 1 - stride] + p[i + 1 + stride])
                .as_::<libc::c_int>()
                * 5
    }

    fn eight_neighbors<P>(p: &CursorMut<P>, i: isize) -> libc::c_int
    where
        P: Add<Output = P> + ToPrimitive<libc::c_int> + Copy,
    {
        let stride = REST_UNIT_STRIDE as isize;
        (p[i] + p[i - 1] + p[i + 1] + p[i - stride] + p[i + stride]).as_::<libc::c_int>() * 4
            + (p[i - 1 - stride] + p[i - 1 + stride] + p[i + 1 - stride] + p[i + 1 + stride])
                .as_::<libc::c_int>()
                * 3
    }

    let mut src = &src[3 * REST_UNIT_STRIDE + 3..];
    let mut dst = dst.as_mut_slice();
    if n == 25 {
        let mut j = 0;
        while j < h - 1 {
            for i in 0..w {
                let a = six_neighbors(&B, i as isize);
                let b = six_neighbors(&A, i as isize);
                dst[i as usize] =
                    ((b - a * (src[i as usize]).as_::<libc::c_int>() + (1 << 8)) >> 9).as_();
            }
            dst = &mut dst[384.. /* Maximum restoration width is 384 (256 * 1.5) */];
            src = &src[REST_UNIT_STRIDE..];
            B += REST_UNIT_STRIDE;
            A += REST_UNIT_STRIDE;
            for i in 0..w {
                let a =
                    B[i].as_::<libc::c_int>() * 6 + (B[i - 1] + B[i + 1]).as_::<libc::c_int>() * 5;
                let b = A[i] * 6 + (A[i - 1] + A[i + 1]) * 5;
                dst[i as usize] =
                    (b - a * (src[i as usize]).as_::<libc::c_int>() + (1 << 7) >> 8).as_();
            }
            dst = &mut dst[384.. /* Maximum restoration width is 384 (256 * 1.5) */];
            src = &src[REST_UNIT_STRIDE..];
            B += REST_UNIT_STRIDE;
            A += REST_UNIT_STRIDE;
            j += 2;
        }
        // Last row, when number of rows is odd
        if j + 1 == h {
            for i in 0..w {
                let a = six_neighbors(&B, i as isize);
                let b = six_neighbors(&A, i as isize);
                dst[i as usize] =
                    (b - a * (src[i as usize]).as_::<libc::c_int>() + (1 << 8) >> 9).as_();
            }
        }
    } else {
        for _ in 0..h {
            for i in 0..w {
                let a = eight_neighbors(&B, i as isize);
                let b = eight_neighbors(&A, i as isize);
                dst[i as usize] =
                    (b - a * (src[i as usize]).as_::<libc::c_int>() + (1 << 8) >> 9).as_();
            }
            dst = &mut dst[384..];
            src = &src[REST_UNIT_STRIDE..];
            B += REST_UNIT_STRIDE;
            A += REST_UNIT_STRIDE;
        }
    };
}

unsafe extern "C" fn sgr_5x5_c_erased<BD: BitDepth>(
    p: *mut DynPixel,
    stride: ptrdiff_t,
    left: *const LeftPixelRow<DynPixel>,
    lpf: *const DynPixel,
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
    // Selfguided filter is applied to a maximum stripe height of 64 + 3 pixels
    // of padding above and below
    let mut tmp = [0.as_(); 70 /*(64 + 3 + 3)*/ * REST_UNIT_STRIDE];

    // Selfguided filter outputs to a maximum stripe height of 64 and a
    // maximum restoration width of 384 (256 * 1.5)
    let mut dst = [0.as_(); 64 * 384];

    padding::<BD>(
        &mut tmp,
        p,
        stride as usize,
        left,
        lpf,
        w as usize,
        h as usize,
        edges,
    );
    selfguided_filter(
        &mut dst,
        &mut tmp,
        REST_UNIT_STRIDE as ptrdiff_t,
        w,
        h,
        25,
        (*params).sgr.s0,
        bd,
    );

    let w0 = (*params).sgr.w0 as libc::c_int;
    for j in 0..h {
        for i in 0..w {
            let v = w0 * dst[(j * 384 + i) as usize].as_::<libc::c_int>();
            *p.offset(i as isize) = bd
                .iclip_pixel((*p.offset(i as isize)).as_::<libc::c_int>() + (v + (1 << 10) >> 11));
        }
        p = p.offset(BD::pxstride(stride as usize) as isize);
    }
}

unsafe extern "C" fn sgr_3x3_c_erased<BD: BitDepth>(
    p: *mut DynPixel,
    stride: ptrdiff_t,
    left: *const LeftPixelRow<DynPixel>,
    lpf: *const DynPixel,
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
    let mut tmp = [0.as_(); 70 /*(64 + 3 + 3)*/ * REST_UNIT_STRIDE];
    let mut dst = [0.as_(); 64 * 384];

    padding::<BD>(
        &mut tmp,
        p,
        stride as usize,
        left,
        lpf,
        w as usize,
        h as usize,
        edges,
    );
    selfguided_filter(
        &mut dst,
        &mut tmp,
        REST_UNIT_STRIDE as ptrdiff_t,
        w,
        h,
        9,
        (*params).sgr.s1,
        bd,
    );

    let w1 = (*params).sgr.w1 as libc::c_int;
    for j in 0..h {
        for i in 0..w {
            let v = w1 * dst[(j * 384 + i) as usize].as_::<libc::c_int>();
            *p.offset(i as isize) = bd
                .iclip_pixel((*p.offset(i as isize)).as_::<libc::c_int>() + (v + (1 << 10) >> 11));
        }
        p = p.offset(BD::pxstride(stride as usize) as isize);
    }
}

unsafe extern "C" fn sgr_mix_c_erased<BD: BitDepth>(
    p: *mut DynPixel,
    stride: ptrdiff_t,
    left: *const LeftPixelRow<DynPixel>,
    lpf: *const DynPixel,
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
    let mut tmp = [0.as_(); 70 /*(64 + 3 + 3)*/ * REST_UNIT_STRIDE];
    let mut dst0 = [0.as_(); 64 * 384];
    let mut dst1 = [0.as_(); 64 * 384];

    padding::<BD>(
        &mut tmp,
        p,
        stride as usize,
        left,
        lpf,
        w as usize,
        h as usize,
        edges,
    );
    selfguided_filter(
        &mut dst0,
        &mut tmp,
        REST_UNIT_STRIDE as ptrdiff_t,
        w,
        h,
        25,
        (*params).sgr.s0,
        bd,
    );
    selfguided_filter(
        &mut dst1,
        &mut tmp,
        REST_UNIT_STRIDE as ptrdiff_t,
        w,
        h,
        9,
        (*params).sgr.s1,
        bd,
    );

    let w0 = (*params).sgr.w0 as libc::c_int;
    let w1 = (*params).sgr.w1 as libc::c_int;
    for j in 0..h {
        for i in 0..w {
            let v = w0 * dst0[(j * 384 + i) as usize].as_::<libc::c_int>()
                + w1 * dst1[(j * 384 + i) as usize].as_::<libc::c_int>();
            *p.offset(i as isize) = bd
                .iclip_pixel((*p.offset(i as isize)).as_::<libc::c_int>() + (v + (1 << 10) >> 11));
        }
        p = p.offset(BD::pxstride(stride as usize) as isize);
    }
}

#[cfg(all(feature = "asm", target_arch = "arm"))]
unsafe fn dav1d_wiener_filter_h_neon<BD: BitDepth>(
    dst: &mut [i16],
    left: *const [BD::Pixel; 4],
    src: *const BD::Pixel,
    stride: ptrdiff_t,
    fh: *const [i16; 8],
    w: intptr_t,
    h: libc::c_int,
    edges: LrEdgeFlags,
    bd: BD,
) {
    macro_rules! asm_fn {
        ($name:ident) => {{
            extern "C" {
                fn $name(
                    dst: *mut int16_t,
                    left: *const libc::c_void,
                    src: *const libc::c_void,
                    stride: ptrdiff_t,
                    fh: *const int16_t,
                    w: intptr_t,
                    h: libc::c_int,
                    edges: LrEdgeFlags,
                    bitdepth_max: libc::c_int,
                );
            }
            $name
        }};
    }
    (match BD::BPC {
        BPC::BPC8 => asm_fn!(dav1d_wiener_filter_h_8bpc_neon),
        BPC::BPC16 => asm_fn!(dav1d_wiener_filter_h_16bpc_neon),
    })(
        dst.as_mut_ptr(),
        left.cast(),
        src.cast(),
        stride,
        fh.cast(),
        w,
        h,
        edges,
        bd.into_c(),
    )
}

#[cfg(all(feature = "asm", target_arch = "arm"))]
unsafe fn dav1d_wiener_filter_v_neon<BD: BitDepth>(
    dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    mid: &mut [i16],
    w: libc::c_int,
    h: libc::c_int,
    fv: *const [i16; 8],
    edges: LrEdgeFlags,
    mid_stride: ptrdiff_t,
    bd: BD,
) {
    macro_rules! asm_fn {
        ($name:ident) => {{
            extern "C" {
                fn $name(
                    dst: *mut libc::c_void,
                    stride: ptrdiff_t,
                    mid: *const int16_t,
                    w: libc::c_int,
                    h: libc::c_int,
                    fv: *const int16_t,
                    edges: LrEdgeFlags,
                    mid_stride: ptrdiff_t,
                    bitdepth_max: libc::c_int,
                );
            }
            $name
        }};
    }
    (match BD::BPC {
        BPC::BPC8 => asm_fn!(dav1d_wiener_filter_v_8bpc_neon),
        BPC::BPC16 => asm_fn!(dav1d_wiener_filter_v_16bpc_neon),
    })(
        dst.cast(),
        stride,
        mid.as_mut_ptr(),
        w,
        h,
        fv.cast(),
        edges,
        mid_stride,
        bd.into_c(),
    )
}

#[cfg(all(feature = "asm", target_arch = "arm"))]
unsafe extern "C" fn wiener_filter_neon_erased<BD: BitDepth>(
    p: *mut DynPixel,
    stride: ptrdiff_t,
    left: *const LeftPixelRow<DynPixel>,
    lpf: *const DynPixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: libc::c_int,
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
        BD::from_c(bitdepth_max),
    )
}

#[cfg(all(feature = "asm", target_arch = "arm"))]
unsafe fn wiener_filter_neon<BD: BitDepth>(
    dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    left: *const [BD::Pixel; 4],
    lpf: *const BD::Pixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bd: BD,
) {
    let filter: *const [int16_t; 8] = (*params).filter.0.as_ptr();
    let mut mid: Align16<[int16_t; 68 * 384]> = Align16([0; 68 * 384]);
    let mut mid_stride: libc::c_int = w + 7 & !7;
    dav1d_wiener_filter_h_neon(
        &mut mid.0[2 * mid_stride as usize..],
        left,
        dst,
        stride,
        filter.offset(0),
        w as intptr_t,
        h,
        edges,
        bd,
    );
    if edges & LR_HAVE_TOP != 0 {
        dav1d_wiener_filter_h_neon(
            &mut mid.0[..],
            core::ptr::null(),
            lpf,
            stride,
            filter.offset(0),
            w as intptr_t,
            2,
            edges,
            bd,
        );
    }
    if edges & LR_HAVE_BOTTOM != 0 {
        dav1d_wiener_filter_h_neon(
            &mut mid.0[(2 + h as usize) * mid_stride as usize..],
            core::ptr::null(),
            lpf.offset((6 * BD::pxstride(stride as usize)) as isize),
            stride,
            filter.offset(0),
            w as intptr_t,
            2,
            edges,
            bd,
        );
    }
    dav1d_wiener_filter_v_neon(
        dst,
        stride,
        &mut mid.0[2 * mid_stride as usize..],
        w,
        h,
        filter.offset(1),
        edges,
        (mid_stride as usize * ::core::mem::size_of::<int16_t>()) as ptrdiff_t,
        bd,
    );
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe fn dav1d_sgr_box3_h_neon<BD: BitDepth>(
    sumsq: *mut int32_t,
    sum: *mut int16_t,
    left: *const [BD::Pixel; 4],
    src: *const BD::Pixel,
    stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    edges: LrEdgeFlags,
) {
    macro_rules! asm_fn {
        ($name:ident) => {{
            extern "C" {
                fn $name(
                    sumsq: *mut int32_t,
                    sum: *mut int16_t,
                    left: *const libc::c_void,
                    src: *const libc::c_void,
                    stride: ptrdiff_t,
                    w: libc::c_int,
                    h: libc::c_int,
                    edges: LrEdgeFlags,
                );
            }
            $name
        }};
    }
    (match BD::BPC {
        BPC::BPC8 => asm_fn!(dav1d_sgr_box3_h_8bpc_neon),
        BPC::BPC16 => asm_fn!(dav1d_sgr_box3_h_16bpc_neon),
    })(sumsq, sum, left.cast(), src.cast(), stride, w, h, edges)
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe fn dav1d_sgr_finish_filter1_neon<BD: BitDepth>(
    tmp: &mut [i16; 64 * 384],
    src: *const BD::Pixel,
    stride: ptrdiff_t,
    a: *const int32_t,
    b: *const int16_t,
    w: libc::c_int,
    h: libc::c_int,
) {
    macro_rules! asm_fn {
        ($name:ident) => {{
            extern "C" {
                fn $name(
                    tmp: *mut int16_t,
                    src: *const libc::c_void,
                    stride: ptrdiff_t,
                    a: *const int32_t,
                    b: *const int16_t,
                    w: libc::c_int,
                    h: libc::c_int,
                );
            }
            $name
        }};
    }
    (match BD::BPC {
        BPC::BPC8 => asm_fn!(dav1d_sgr_finish_filter1_8bpc_neon),
        BPC::BPC16 => asm_fn!(dav1d_sgr_finish_filter1_16bpc_neon),
    })(tmp.as_mut_ptr(), src.cast(), stride, a, b, w, h)
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe fn dav1d_sgr_filter1_neon<BD: BitDepth>(
    tmp: &mut [i16; 64 * 384],
    src: *const BD::Pixel,
    stride: ptrdiff_t,
    left: *const [BD::Pixel; 4],
    lpf: *const BD::Pixel,
    w: libc::c_int,
    h: libc::c_int,
    strength: u32,
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
    dav1d_sgr_box3_h_neon::<BD>(sumsq, sum, left.cast(), src.cast(), stride, w, h, edges);
    if edges as libc::c_uint & LR_HAVE_TOP as libc::c_int as libc::c_uint != 0 {
        dav1d_sgr_box3_h_neon::<BD>(
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
        dav1d_sgr_box3_h_neon::<BD>(
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
    dav1d_sgr_calc_ab1_neon(a, b, w, h, strength as libc::c_int, bd.into_c());
    dav1d_sgr_finish_filter1_neon::<BD>(tmp, src, stride, a, b, w, h);
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe fn dav1d_sgr_box5_h_neon<BD: BitDepth>(
    sumsq: *mut int32_t,
    sum: *mut int16_t,
    left: *const [BD::Pixel; 4],
    src: *const BD::Pixel,
    stride: ptrdiff_t,
    w: libc::c_int,
    h: libc::c_int,
    edges: LrEdgeFlags,
) {
    macro_rules! asm_fn {
        ($name:ident) => {{
            extern "C" {
                fn $name(
                    sumsq: *mut int32_t,
                    sum: *mut int16_t,
                    left: *const libc::c_void,
                    src: *const libc::c_void,
                    stride: ptrdiff_t,
                    w: libc::c_int,
                    h: libc::c_int,
                    edges: LrEdgeFlags,
                );
            }
            $name
        }};
    }
    (match BD::BPC {
        BPC::BPC8 => asm_fn!(dav1d_sgr_box5_h_8bpc_neon),
        BPC::BPC16 => asm_fn!(dav1d_sgr_box5_h_16bpc_neon),
    })(sumsq, sum, left.cast(), src.cast(), stride, w, h, edges)
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe fn dav1d_sgr_finish_filter2_neon<BD: BitDepth>(
    tmp: &mut [i16; 64 * 384],
    src: *const BD::Pixel,
    stride: ptrdiff_t,
    a: *const int32_t,
    b: *const int16_t,
    w: libc::c_int,
    h: libc::c_int,
) {
    macro_rules! asm_fn {
        ($name:ident) => {{
            extern "C" {
                fn $name(
                    tmp: *mut int16_t,
                    src: *const libc::c_void,
                    stride: ptrdiff_t,
                    a: *const int32_t,
                    b: *const int16_t,
                    w: libc::c_int,
                    h: libc::c_int,
                );
            }
            $name
        }};
    }
    (match BD::BPC {
        BPC::BPC8 => asm_fn!(dav1d_sgr_finish_filter2_8bpc_neon),
        BPC::BPC16 => asm_fn!(dav1d_sgr_finish_filter2_16bpc_neon),
    })(tmp.as_mut_ptr(), src.cast(), stride, a, b, w, h)
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe fn dav1d_sgr_filter2_neon<BD: BitDepth>(
    tmp: &mut [i16; 64 * 384],
    src: *const BD::Pixel,
    stride: ptrdiff_t,
    left: *const [BD::Pixel; 4],
    lpf: *const BD::Pixel,
    w: libc::c_int,
    h: libc::c_int,
    strength: u32,
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
    dav1d_sgr_box5_h_neon::<BD>(sumsq, sum, left, src, stride, w, h, edges);
    if edges as libc::c_uint & LR_HAVE_TOP as libc::c_int as libc::c_uint != 0 {
        dav1d_sgr_box5_h_neon::<BD>(
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
        dav1d_sgr_box5_h_neon::<BD>(
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
    dav1d_sgr_calc_ab2_neon(a, b, w, h, strength as libc::c_int, bd.into_c());
    dav1d_sgr_finish_filter2_neon::<BD>(tmp, src, stride, a, b, w, h);
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe fn dav1d_sgr_weighted1_neon<BD: BitDepth>(
    dst: *mut BD::Pixel,
    dst_stride: ptrdiff_t,
    src: *const BD::Pixel,
    src_stride: ptrdiff_t,
    t1: &mut [i16; 64 * 384],
    w: libc::c_int,
    h: libc::c_int,
    wt: i16,
    bd: BD,
) {
    macro_rules! asm_fn {
        ($name:ident) => {{
            extern "C" {
                fn $name(
                    dst: *mut DynPixel,
                    dst_stride: ptrdiff_t,
                    src: *const DynPixel,
                    src_stride: ptrdiff_t,
                    t1: *const int16_t,
                    w: libc::c_int,
                    h: libc::c_int,
                    wt: libc::c_int,
                    bitdepth_max: libc::c_int,
                );
            }
            $name
        }};
    }
    (match BD::BPC {
        BPC::BPC8 => asm_fn!(dav1d_sgr_weighted1_8bpc_neon),
        BPC::BPC16 => asm_fn!(dav1d_sgr_weighted1_16bpc_neon),
    })(
        dst.cast(),
        dst_stride,
        src.cast(),
        src_stride,
        t1.as_mut_ptr(),
        w,
        h,
        wt.into(),
        bd.into_c(),
    )
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe fn dav1d_sgr_weighted2_neon<BD: BitDepth>(
    dst: *mut BD::Pixel,
    dst_stride: ptrdiff_t,
    src: *const BD::Pixel,
    src_stride: ptrdiff_t,
    t1: &mut [i16; 64 * 384],
    t2: &mut [i16; 64 * 384],
    w: libc::c_int,
    h: libc::c_int,
    wt: &[i16; 2],
    bd: BD,
) {
    macro_rules! asm_fn {
        ($name:ident) => {{
            extern "C" {
                fn $name(
                    dst: *mut libc::c_void,
                    dst_stride: ptrdiff_t,
                    src: *const libc::c_void,
                    src_stride: ptrdiff_t,
                    t1: *const int16_t,
                    t2: *const int16_t,
                    w: libc::c_int,
                    h: libc::c_int,
                    wt: *const int16_t,
                    bitdepth_max: libc::c_int,
                );
            }
            $name
        }};
    }
    (match BD::BPC {
        BPC::BPC8 => asm_fn!(dav1d_sgr_weighted2_8bpc_neon),
        BPC::BPC16 => asm_fn!(dav1d_sgr_weighted2_16bpc_neon),
    })(
        dst.cast(),
        dst_stride,
        src.cast(),
        src_stride,
        t1.as_mut_ptr(),
        t2.as_mut_ptr(),
        w,
        h,
        wt.as_ptr(),
        bd.into_c(),
    )
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn sgr_filter_5x5_neon_erased<BD: BitDepth>(
    p: *mut DynPixel,
    stride: ptrdiff_t,
    left: *const LeftPixelRow<DynPixel>,
    lpf: *const DynPixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: libc::c_int,
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
        BD::from_c(bitdepth_max),
    )
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe fn sgr_filter_5x5_neon<BD: BitDepth>(
    dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    left: *const [BD::Pixel; 4],
    lpf: *const BD::Pixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bd: BD,
) {
    let mut tmp: Align16<[int16_t; 24576]> = Align16([0; 24576]);
    dav1d_sgr_filter2_neon(
        &mut tmp.0,
        dst,
        stride,
        left,
        lpf,
        w,
        h,
        (*params).sgr.s0,
        edges,
        bd,
    );
    dav1d_sgr_weighted1_neon(
        dst,
        stride,
        dst,
        stride,
        &mut tmp.0,
        w,
        h,
        (*params).sgr.w0,
        bd,
    );
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn sgr_filter_3x3_neon_erased<BD: BitDepth>(
    p: *mut DynPixel,
    stride: ptrdiff_t,
    left: *const LeftPixelRow<DynPixel>,
    lpf: *const DynPixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: libc::c_int,
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
        BD::from_c(bitdepth_max),
    )
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe fn sgr_filter_3x3_neon<BD: BitDepth>(
    dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    left: *const [BD::Pixel; 4],
    lpf: *const BD::Pixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bd: BD,
) {
    let mut tmp: Align16<[int16_t; 24576]> = Align16([0; 24576]);
    dav1d_sgr_filter1_neon(
        &mut tmp.0,
        dst,
        stride,
        left,
        lpf,
        w,
        h,
        (*params).sgr.s1,
        edges,
        bd,
    );
    dav1d_sgr_weighted1_neon(
        dst,
        stride,
        dst,
        stride,
        &mut tmp.0,
        w,
        h,
        (*params).sgr.w1,
        bd,
    );
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn sgr_filter_mix_neon_erased<BD: BitDepth>(
    p: *mut DynPixel,
    stride: ptrdiff_t,
    left: *const LeftPixelRow<DynPixel>,
    lpf: *const DynPixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: libc::c_int,
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
        BD::from_c(bitdepth_max),
    )
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn sgr_filter_mix_neon<BD: BitDepth>(
    dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    left: *const [BD::Pixel; 4],
    lpf: *const BD::Pixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bd: BD,
) {
    let mut tmp1: Align16<[int16_t; 24576]> = Align16([0; 24576]);
    let mut tmp2: Align16<[int16_t; 24576]> = Align16([0; 24576]);
    dav1d_sgr_filter2_neon(
        &mut tmp1.0,
        dst,
        stride,
        left,
        lpf,
        w,
        h,
        (*params).sgr.s0,
        edges,
        bd,
    );
    dav1d_sgr_filter1_neon(
        &mut tmp2.0,
        dst,
        stride,
        left,
        lpf,
        w,
        h,
        (*params).sgr.s1,
        edges,
        bd,
    );
    let wt: [int16_t; 2] = [(*params).sgr.w0, (*params).sgr.w1];
    dav1d_sgr_weighted2_neon(
        dst,
        stride,
        dst,
        stride,
        &mut tmp1.0,
        &mut tmp2.0,
        w,
        h,
        &wt,
        bd,
    );
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
#[inline(always)]
fn loop_restoration_dsp_init_x86<BD: BitDepth>(
    c: &mut Dav1dLoopRestorationDSPContext,
    bpc: libc::c_int,
) {
    use crate::src::x86::cpu::*;

    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_X86_CPU_FLAG_SSE2 == 0 {
        return;
    }

    if BD::BPC == BPC::BPC8 {
        c.wiener[0] = decl_looprestorationfilter_fn!(fn dav1d_wiener_filter7_8bpc_sse2);
        c.wiener[1] = decl_looprestorationfilter_fn!(fn dav1d_wiener_filter5_8bpc_sse2);
    }

    if flags & DAV1D_X86_CPU_FLAG_SSSE3 == 0 {
        return;
    }

    c.wiener[0] = bd_fn!(decl_looprestorationfilter_fn, BD, wiener_filter7, ssse3);
    c.wiener[1] = bd_fn!(decl_looprestorationfilter_fn, BD, wiener_filter5, ssse3);

    if BD::BPC == BPC::BPC8 || bpc == 10 {
        c.sgr[0] = bd_fn!(decl_looprestorationfilter_fn, BD, sgr_filter_5x5, ssse3);
        c.sgr[1] = bd_fn!(decl_looprestorationfilter_fn, BD, sgr_filter_3x3, ssse3);
        c.sgr[2] = bd_fn!(decl_looprestorationfilter_fn, BD, sgr_filter_mix, ssse3);
    }

    #[cfg(target_arch = "x86_64")]
    {
        if flags & DAV1D_X86_CPU_FLAG_AVX2 == 0 {
            return;
        }

        c.wiener[0] = bd_fn!(decl_looprestorationfilter_fn, BD, wiener_filter7, avx2);
        c.wiener[1] = bd_fn!(decl_looprestorationfilter_fn, BD, wiener_filter5, avx2);

        if BD::BPC == BPC::BPC8 || bpc == 10 {
            c.sgr[0] = bd_fn!(decl_looprestorationfilter_fn, BD, sgr_filter_5x5, avx2);
            c.sgr[1] = bd_fn!(decl_looprestorationfilter_fn, BD, sgr_filter_3x3, avx2);
            c.sgr[2] = bd_fn!(decl_looprestorationfilter_fn, BD, sgr_filter_mix, avx2);
        }

        if flags & DAV1D_X86_CPU_FLAG_AVX512ICL == 0 {
            return;
        }

        c.wiener[0] = bd_fn!(decl_looprestorationfilter_fn, BD, wiener_filter7, avx512icl);
        c.wiener[1] = match BD::BPC {
            // With VNNI we don't need a 5-tap version.
            BPC::BPC8 => c.wiener[0],
            BPC::BPC16 => decl_looprestorationfilter_fn!(fn dav1d_wiener_filter5_16bpc_avx512icl),
        };

        if BD::BPC == BPC::BPC8 || bpc == 10 {
            c.sgr[0] = bd_fn!(decl_looprestorationfilter_fn, BD, sgr_filter_5x5, avx512icl);
            c.sgr[1] = bd_fn!(decl_looprestorationfilter_fn, BD, sgr_filter_3x3, avx512icl);
            c.sgr[2] = bd_fn!(decl_looprestorationfilter_fn, BD, sgr_filter_mix, avx512icl);
        }
    }
}

#[cfg(feature = "asm")]
use crate::src::cpu::dav1d_get_cpu_flags;

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
#[inline(always)]
fn loop_restoration_dsp_init_arm<BD: BitDepth>(
    c: &mut Dav1dLoopRestorationDSPContext,
    mut bpc: libc::c_int,
) {
    use crate::src::arm::cpu::DAV1D_ARM_CPU_FLAG_NEON;

    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
        return;
    }

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "aarch64")] {
            c.wiener[0] = bd_fn!(decl_looprestorationfilter_fn, BD, wiener_filter7, neon);
            c.wiener[1] = bd_fn!(decl_looprestorationfilter_fn, BD, wiener_filter5, neon);
        } else {
            c.wiener[0] = wiener_filter_neon_erased::<BD>;
            c.wiener[1] = wiener_filter_neon_erased::<BD>;
        }
    }

    if BD::BPC == BPC::BPC8 || bpc == 10 {
        c.sgr[0] = sgr_filter_5x5_neon_erased::<BD>;
        c.sgr[1] = sgr_filter_3x3_neon_erased::<BD>;
        c.sgr[2] = sgr_filter_mix_neon_erased::<BD>;
    }
}

#[cold]
pub fn dav1d_loop_restoration_dsp_init<BD: BitDepth>(
    c: &mut Dav1dLoopRestorationDSPContext,
    _bpc: libc::c_int,
) {
    c.wiener[1] = wiener_c_erased::<BD>;
    c.wiener[0] = c.wiener[1];
    c.sgr[0] = sgr_5x5_c_erased::<BD>;
    c.sgr[1] = sgr_3x3_c_erased::<BD>;
    c.sgr[2] = sgr_mix_c_erased::<BD>;

    #[cfg(feature = "asm")]
    cfg_if::cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            loop_restoration_dsp_init_x86::<BD>(c, _bpc);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]{
            loop_restoration_dsp_init_arm::<BD>(c, _bpc);
        }
    }
}
