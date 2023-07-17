use crate::include::common::bitdepth::BitDepth;
use crate::include::stddef::ptrdiff_t;
use crate::include::stdint::int16_t;
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dLoopRestorationDSPContext {
    pub wiener: [looprestorationfilter_fn; 2],
    pub sgr: [looprestorationfilter_fn; 3],
}

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

type BD = crate::include::common::bitdepth::BitDepth16;
type pixel = <BD as BitDepth>::Pixel;

// TODO Reuse p when no padding is needed (add and remove lpf pixels in p)
// TODO Chroma only requires 2 rows of padding.
// TODO(randomPoison): Temporarily pub until remaining looprestoration fns have
// been deduplicated.
#[inline(never)]
pub(crate) unsafe fn padding(
    mut dst: &mut [pixel; 70 /*(64 + 3 + 3)*/ * REST_UNIT_STRIDE],
    mut p: *const pixel,
    stride: ptrdiff_t,
    mut left: *const [pixel; 4],
    mut lpf: *const pixel,
    mut unit_w: libc::c_int,
    stripe_h: libc::c_int,
    edges: LrEdgeFlags,
) {
    extern "C" {
        fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }

    #[inline]
    unsafe fn pixel_set(dst: *mut pixel, val: libc::c_int, num: libc::c_int) {
        let mut n = 0;
        while n < num {
            *dst.offset(n as isize) = val as pixel;
            n += 1;
        }
    }

    #[inline]
    unsafe fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
        if x & 1 != 0 {
            unreachable!();
        }
        return x >> 1;
    }

    let have_left = (edges & LR_HAVE_LEFT != 0) as libc::c_int;
    let have_right = (edges & LR_HAVE_RIGHT != 0) as libc::c_int;

    // Copy more pixels if we don't have to pad them
    unit_w += 3 * have_left + 3 * have_right;
    let mut dst_l = &mut dst[(3 * (have_left == 0) as libc::c_int) as usize..];
    p = p.offset(-((3 * have_left) as isize));
    lpf = lpf.offset(-((3 * have_left) as isize));

    if edges & LR_HAVE_TOP != 0 {
        // Copy previous loop filtered rows
        let stride = BD::pxstride(stride as usize);
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
    if edges as libc::c_uint & LR_HAVE_BOTTOM as libc::c_int as libc::c_uint != 0 {
        let below_1: *const pixel = lpf.offset(6 * PXSTRIDE(stride));
        let below_2: *const pixel = below_1.offset(PXSTRIDE(stride) as isize);
        memcpy(
            dst_tl.as_mut_ptr().offset((stripe_h * 390) as isize) as *mut libc::c_void,
            below_1 as *const libc::c_void,
            (unit_w << 1) as libc::c_ulong,
        );
        memcpy(
            dst_tl.as_mut_ptr().offset(((stripe_h + 1) * 390) as isize) as *mut libc::c_void,
            below_2 as *const libc::c_void,
            (unit_w << 1) as libc::c_ulong,
        );
        memcpy(
            dst_tl.as_mut_ptr().offset(((stripe_h + 2) * 390) as isize) as *mut libc::c_void,
            below_2 as *const libc::c_void,
            (unit_w << 1) as libc::c_ulong,
        );
    } else {
        let src: *const pixel = p.offset(((stripe_h - 1) as isize * PXSTRIDE(stride)) as isize);
        memcpy(
            dst_tl.as_mut_ptr().offset((stripe_h * 390) as isize) as *mut libc::c_void,
            src as *const libc::c_void,
            (unit_w << 1) as libc::c_ulong,
        );
        memcpy(
            dst_tl.as_mut_ptr().offset(((stripe_h + 1) * 390) as isize) as *mut libc::c_void,
            src as *const libc::c_void,
            (unit_w << 1) as libc::c_ulong,
        );
        memcpy(
            dst_tl.as_mut_ptr().offset(((stripe_h + 2) * 390) as isize) as *mut libc::c_void,
            src as *const libc::c_void,
            (unit_w << 1) as libc::c_ulong,
        );
        if have_left != 0 {
            memcpy(
                dst_tl.as_mut_ptr().offset((stripe_h * 390) as isize) as *mut libc::c_void,
                &*(*left.offset((stripe_h - 1) as isize)).as_ptr().offset(1) as *const pixel
                    as *const libc::c_void,
                ((3 as libc::c_int) << 1) as libc::c_ulong,
            );
            memcpy(
                dst_tl.as_mut_ptr().offset(((stripe_h + 1) * 390) as isize) as *mut libc::c_void,
                &*(*left.offset((stripe_h - 1) as isize)).as_ptr().offset(1) as *const pixel
                    as *const libc::c_void,
                ((3 as libc::c_int) << 1) as libc::c_ulong,
            );
            memcpy(
                dst_tl.as_mut_ptr().offset(((stripe_h + 2) * 390) as isize) as *mut libc::c_void,
                &*(*left.offset((stripe_h - 1) as isize)).as_ptr().offset(1) as *const pixel
                    as *const libc::c_void,
                ((3 as libc::c_int) << 1) as libc::c_ulong,
            );
        }
    }

    // Inner UNIT_WxSTRIPE_H
    let len = (unit_w - 3 * have_left) as usize;
    for j in 0..stripe_h as usize {
        let p = std::slice::from_raw_parts(
            p.offset((j * BD::pxstride(stride as usize)) as isize + (3 * have_left) as isize),
            len,
        );
        BD::pixel_copy(
            &mut dst_tl[j * REST_UNIT_STRIDE + (3 * have_left) as usize..],
            p,
            len,
        );
    }

    if have_right == 0 {
        let mut pad: *mut pixel = dst_l.as_mut_ptr().offset(unit_w as isize);
        let mut row_last: *mut pixel =
            &mut *dst_l.as_mut_ptr().offset((unit_w - 1) as isize) as *mut pixel;
        let mut j_0 = 0;
        while j_0 < stripe_h + 6 {
            pixel_set(pad, *row_last as libc::c_int, 3 as libc::c_int);
            pad = pad.offset(390);
            row_last = row_last.offset(390);
            j_0 += 1;
        }
    }
    if have_left == 0 {
        for j in 0..stripe_h as usize + 6 {
            let offset = j * REST_UNIT_STRIDE;
            pixel_set(
                dst[offset..].as_mut_ptr(),
                dst[3 + offset] as libc::c_int,
                3,
            );
        }
    } else {
        let dst = &mut dst[3 * REST_UNIT_STRIDE..];

        for j in 0..stripe_h as usize {
            memcpy(
                dst[j * REST_UNIT_STRIDE..].as_mut_ptr() as *mut libc::c_void,
                &*(*left.offset(j as isize)).as_ptr().offset(1) as *const pixel
                    as *const libc::c_void,
                ((3 as libc::c_int) << 1) as libc::c_ulong,
            );
        }
    };
}
