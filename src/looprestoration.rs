use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::intops::iclip;
use crate::include::stddef::ptrdiff_t;
use crate::include::stdint::int16_t;
use crate::include::stdint::uint16_t;
use crate::include::stdint::uint32_t;
use crate::src::align::Align16;

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
