use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::bitdepth::LeftPixelRow;
use crate::include::common::bitdepth::ToPrimitive;
use crate::include::common::bitdepth::BPC;
use crate::include::common::intops::iclip;
use crate::include::dav1d::picture::Rav1dPictureDataComponentOffset;
use crate::src::cpu::CpuFlags;
use crate::src::cursor::CursorMut;
use crate::src::ffi_safe::FFISafe;
use crate::src::tables::dav1d_sgr_x_by_x;
use crate::src::wrap_fn_ptr::wrap_fn_ptr;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::mem;
use std::ops::Add;
use std::slice;
use to_method::To;
use zerocopy::AsBytes;
use zerocopy::FromBytes;
use zerocopy::FromZeroes;

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
use ::{std::ffi::c_void, std::ptr};

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
use crate::src::align::Align16;

#[cfg(all(feature = "asm", target_arch = "arm"))]
use libc::intptr_t;

#[cfg(all(
    feature = "asm",
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")
))]
use crate::include::common::bitdepth::bd_fn;

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
use crate::include::common::bitdepth::bpc_fn;

#[cfg(all(feature = "asm", target_arch = "arm"))]
extern "C" {
    fn dav1d_sgr_box3_v_neon(
        sumsq: *mut i32,
        sum: *mut i16,
        w: c_int,
        h: c_int,
        edges: LrEdgeFlags,
    );

    fn dav1d_sgr_calc_ab1_neon(
        a: *mut i32,
        b: *mut i16,
        w: c_int,
        h: c_int,
        strength: c_int,
        bitdepth_max: c_int,
    );

    fn dav1d_sgr_box5_v_neon(
        sumsq: *mut i32,
        sum: *mut i16,
        w: c_int,
        h: c_int,
        edges: LrEdgeFlags,
    );

    fn dav1d_sgr_calc_ab2_neon(
        a: *mut i32,
        b: *mut i16,
        w: c_int,
        h: c_int,
        strength: c_int,
        bitdepth_max: c_int,
    );
}

pub type LrEdgeFlags = c_uint;
pub const LR_HAVE_BOTTOM: LrEdgeFlags = 8;
pub const LR_HAVE_TOP: LrEdgeFlags = 4;
pub const LR_HAVE_RIGHT: LrEdgeFlags = 2;
pub const LR_HAVE_LEFT: LrEdgeFlags = 1;

#[derive(FromZeroes, FromBytes, AsBytes)]
#[repr(C)]
pub struct LooprestorationParamsSgr {
    pub s0: u32,
    pub s1: u32,
    pub w0: i16,
    pub w1: i16,
}

/// This [`zerocopy`]-based "`union`" has the same layout
/// as an actual `union` would, so it's safe to continue passing to asm,
/// but it's otherwise safe to use from Rust.
///
/// [`zerocopy`]: ::zerocopy
#[derive(Default)]
#[repr(C)]
#[repr(align(16))]
pub struct LooprestorationParams {
    /// [`Align16`] moved to [`Self`] we can't `#[derive(`[`AsBytes`]`)]` on it due to generics.
    ///
    /// [`Align16`]: crate::src::align::Align16
    pub filter: [[i16; 8]; 2],
}

impl LooprestorationParams {
    pub fn sgr(&self) -> &LooprestorationParamsSgr {
        // These asserts ensure this is a no-op.
        const _: () = assert!(
            mem::size_of::<LooprestorationParams>() >= mem::size_of::<LooprestorationParamsSgr>()
        );
        let _: () = assert!(
            mem::align_of::<LooprestorationParams>() >= mem::align_of::<LooprestorationParamsSgr>()
        );
        FromBytes::ref_from_prefix(AsBytes::as_bytes(&self.filter)).unwrap()
    }

    pub fn sgr_mut(&mut self) -> &mut LooprestorationParamsSgr {
        // These asserts ensure this is a no-op.
        const _: () = assert!(
            mem::size_of::<LooprestorationParams>() >= mem::size_of::<LooprestorationParamsSgr>()
        );
        const _: () = assert!(
            mem::align_of::<LooprestorationParams>() >= mem::align_of::<LooprestorationParamsSgr>()
        );
        FromBytes::mut_from_prefix(AsBytes::as_bytes_mut(&mut self.filter)).unwrap()
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn loop_restoration_filter(
    dst_ptr: *mut DynPixel,
    dst_stride: ptrdiff_t,
    left: *const LeftPixelRow<DynPixel>,
    lpf: *const DynPixel,
    w: c_int,
    h: c_int,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: c_int,
    _dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
) -> ());

impl loop_restoration_filter::Fn {
    /// Although the spec applies restoration filters over 4x4 blocks,
    /// they can be applied to a bigger surface.
    ///
    /// * `w` is constrained by the restoration unit size (`w <= 256`).
    /// * `h` is constrained by the stripe height (`h <= 64`).
    ///
    /// The filter functions are allowed to do
    /// aligned writes past the right edge of the buffer,
    /// aligned up to the minimum loop restoration unit size
    /// (which is 32 pixels for subsampled chroma and 64 pixels for luma).
    pub unsafe fn call<BD: BitDepth>(
        &self,
        dst: Rav1dPictureDataComponentOffset,
        left: &[LeftPixelRow<BD::Pixel>],
        lpf: *const BD::Pixel,
        w: c_int,
        h: c_int,
        params: &LooprestorationParams,
        edges: LrEdgeFlags,
        bd: BD,
    ) {
        let dst_ptr = dst.data.as_mut_ptr_at::<BD>(dst.offset).cast();
        let dst_stride = dst.data.stride();
        let left = left[..h as usize].as_ptr().cast();
        let lpf = lpf.cast();
        let bd = bd.into_c();
        let dst = FFISafe::new(&dst);
        self.get()(dst_ptr, dst_stride, left, lpf, w, h, params, edges, bd, dst)
    }
}

pub struct Rav1dLoopRestorationDSPContext {
    pub wiener: [loop_restoration_filter::Fn; 2],
    pub sgr: [loop_restoration_filter::Fn; 3],
}

// 256 * 1.5 + 3 + 3 = 390
const REST_UNIT_STRIDE: usize = 390;

// TODO Reuse p when no padding is needed (add and remove lpf pixels in p)
// TODO Chroma only requires 2 rows of padding.
#[inline(never)]
unsafe fn padding<BD: BitDepth>(
    dst: &mut [BD::Pixel; 70 /*(64 + 3 + 3)*/ * REST_UNIT_STRIDE],
    p: Rav1dPictureDataComponentOffset,
    left: &[LeftPixelRow<BD::Pixel>],
    lpf: *const BD::Pixel,
    unit_w: usize,
    stripe_h: usize,
    edges: LrEdgeFlags,
) {
    let left = &left[..stripe_h];
    assert!(stripe_h > 0);
    let stride = p.data.pixel_stride::<BD>();

    let [have_left, have_right, have_top, have_bottom] =
        [LR_HAVE_LEFT, LR_HAVE_RIGHT, LR_HAVE_TOP, LR_HAVE_BOTTOM]
            .map(|lr_have| edges & lr_have != 0);
    let [have_left_3, have_right_3] = [have_left, have_right].map(|have| 3 * have as usize);

    // Copy more pixels if we don't have to pad them
    let unit_w = unit_w + have_left_3 + have_right_3;
    let dst_l = &mut dst[3 - have_left_3..];
    let p = p - have_left_3;
    let lpf = lpf.offset(-(have_left_3 as isize));
    let abs_stride = stride.unsigned_abs();

    if have_top {
        // Copy previous loop filtered rows
        let (above_1, above_2) = if stride < 0 {
            let above_2 = std::slice::from_raw_parts(lpf.offset(stride), abs_stride + unit_w);
            let above_1 = &above_2[abs_stride..];
            (above_1, above_2)
        } else {
            let above_1 = std::slice::from_raw_parts(lpf, abs_stride + unit_w);
            let above_2 = &above_1[abs_stride..];
            (above_1, above_2)
        };
        BD::pixel_copy(dst_l, above_1, unit_w);
        BD::pixel_copy(&mut dst_l[REST_UNIT_STRIDE..], above_1, unit_w);
        BD::pixel_copy(&mut dst_l[2 * REST_UNIT_STRIDE..], above_2, unit_w);
    } else {
        // Pad with first row
        let p = &*p.data.slice::<BD, _>((p.offset.., ..unit_w));
        BD::pixel_copy(dst_l, p, unit_w);
        BD::pixel_copy(&mut dst_l[REST_UNIT_STRIDE..], p, unit_w);
        BD::pixel_copy(&mut dst_l[2 * REST_UNIT_STRIDE..], p, unit_w);
        if have_left {
            let left = &left[0][1..];
            BD::pixel_copy(dst_l, left, 3);
            BD::pixel_copy(&mut dst_l[REST_UNIT_STRIDE..], left, left.len());
            BD::pixel_copy(&mut dst_l[2 * REST_UNIT_STRIDE..], left, left.len());
        }
    }

    let dst_tl = &mut dst_l[3 * REST_UNIT_STRIDE..];
    if have_bottom {
        // Copy next loop filtered rows
        let lpf = std::slice::from_raw_parts(
            lpf.offset((6 + if stride < 0 { 1 } else { 0 }) * stride),
            abs_stride + unit_w,
        );
        let (below_1, below_2) = if stride < 0 {
            (&lpf[abs_stride..], lpf)
        } else {
            (lpf, &lpf[abs_stride..])
        };
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
        let src = p + ((stripe_h - 1) as isize * stride);
        let src = &*src.data.slice::<BD, _>((src.offset.., ..unit_w));
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
            let left = &left[stripe_h - 1][1..];
            BD::pixel_copy(&mut dst_tl[stripe_h * REST_UNIT_STRIDE..], left, left.len());
            BD::pixel_copy(
                &mut dst_tl[(stripe_h + 1) * REST_UNIT_STRIDE..],
                left,
                left.len(),
            );
            BD::pixel_copy(
                &mut dst_tl[(stripe_h + 2) * REST_UNIT_STRIDE..],
                left,
                left.len(),
            );
        }
    }

    // Inner UNIT_WxSTRIPE_H
    let len = unit_w - have_left_3;
    for j in 0..stripe_h {
        let p = p + have_left_3 + (j as isize * stride);
        BD::pixel_copy(
            &mut dst_tl[j * REST_UNIT_STRIDE + have_left_3..],
            &p.data.slice::<BD, _>((p.offset.., ..len)),
            len,
        );
    }

    if !have_right {
        // Pad 3x(STRIPE_H+6) with last column
        for j in 0..stripe_h + 6 {
            let row_last = dst_l[(unit_w - 1) + j * REST_UNIT_STRIDE];
            let pad = &mut dst_l[unit_w + j * REST_UNIT_STRIDE..];
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
            // `3 * (have_left == 0) as c_int` simplifies to `3 * 1` and then `3`.
            let val = dst[3 + offset];
            BD::pixel_set(&mut dst[offset..], val, 3);
        }
    } else {
        let dst = &mut dst[3 * REST_UNIT_STRIDE..];
        for j in 0..stripe_h {
            BD::pixel_copy(&mut dst[j * REST_UNIT_STRIDE..], &left[j][1..], 3);
        }
    };
}

/// # Safety
///
/// Must be called by [`loop_restoration_filter::Fn::call`].
unsafe extern "C" fn wiener_c_erased<BD: BitDepth>(
    _p_ptr: *mut DynPixel,
    _stride: ptrdiff_t,
    left: *const LeftPixelRow<DynPixel>,
    lpf: *const DynPixel,
    w: c_int,
    h: c_int,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: c_int,
    p: *const FFISafe<Rav1dPictureDataComponentOffset>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `loop_restoration_filter::Fn::call`.
    let p = *unsafe { FFISafe::get(p) };
    let left = left.cast();
    let lpf = lpf.cast();
    let bd = BD::from_c(bitdepth_max);
    let w = w as usize;
    let h = h as usize;
    // SAFETY: Length sliced in `loop_restoration_filter::Fn::call`.
    let left = unsafe { slice::from_raw_parts(left, h) };
    wiener_rust(p, left, lpf, w, h, params, edges, bd)
}

// FIXME Could split into luma and chroma specific functions,
// (since first and last tops are always 0 for chroma)
// FIXME Could implement a version that requires less temporary memory
// (should be possible to implement with only 6 rows of temp storage)
unsafe fn wiener_rust<BD: BitDepth>(
    p: Rav1dPictureDataComponentOffset,
    left: &[LeftPixelRow<BD::Pixel>],
    lpf: *const BD::Pixel,
    w: usize,
    h: usize,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bd: BD,
) {
    // Wiener filtering is applied to a maximum stripe height of 64 + 3 pixels
    // of padding above and below
    let mut tmp = [0.into(); 70 /*(64 + 3 + 3)*/ * REST_UNIT_STRIDE];

    padding::<BD>(&mut tmp, p, left, lpf, w, h, edges);

    // Values stored between horizontal and vertical filtering don't
    // fit in a u8.
    let mut hor = [0; 70 /*(64 + 3 + 3)*/ * REST_UNIT_STRIDE];

    let filter = &params.filter;
    let bitdepth = bd.bitdepth().as_::<c_int>();
    let round_bits_h = 3 + (bitdepth == 12) as c_int * 2;
    let rounding_off_h = 1 << round_bits_h - 1;
    let clip_limit = 1 << bitdepth + 1 + 7 - round_bits_h;
    for (tmp, hor) in tmp
        .chunks_exact(REST_UNIT_STRIDE)
        .zip(hor.chunks_exact_mut(REST_UNIT_STRIDE))
        .take(h + 6)
    {
        for i in 0..w {
            let mut sum = 1 << bitdepth + 6;

            if BD::BPC == BPC::BPC8 {
                sum += tmp[i + 3].to::<i32>() * 128;
            }

            for (&tmp, &filter) in std::iter::zip(&tmp[i..i + 7], &filter[0][..7]) {
                sum += tmp.to::<i32>() * filter as c_int;
            }

            hor[i] = iclip(sum + rounding_off_h >> round_bits_h, 0, clip_limit - 1) as u16;
        }
    }

    let round_bits_v = 11 - (bitdepth == 12) as c_int * 2;
    let rounding_off_v = 1 << round_bits_v - 1;
    let round_offset = 1 << bitdepth + (round_bits_v - 1);
    for j in 0..h {
        for i in 0..w {
            let mut sum = -round_offset;
            let z = &hor[j * REST_UNIT_STRIDE + i..(j + 7) * REST_UNIT_STRIDE];

            for k in 0..7 {
                sum += z[k * REST_UNIT_STRIDE] as c_int * filter[1][k] as c_int;
            }

            let p = p + (j as isize * p.data.pixel_stride::<BD>()) + i;
            *p.data.index_mut::<BD>(p.offset) =
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
/// ```text
/// x x x x x x x x x x
/// x c c c c c c c c x
/// x i s s s s s s i x
/// x i s s s s s s i x
/// x i s s s s s s i x
/// x i s s s s s s i x
/// x i s s s s s s i x
/// x i s s s s s s i x
/// x c c c c c c c c x
/// x x x x x x x x x x
/// ```
///
/// * s: Pixel summed and stored
/// * i: Pixel summed and stored (between loops)
/// * c: Pixel summed not stored
/// * x: Pixel not summed not stored
fn boxsum3<BD: BitDepth>(
    sumsq: &mut [i32; 68 /*(64 + 2 + 2)*/ * REST_UNIT_STRIDE],
    sum: &mut [BD::Coef; 68 /*(64 + 2 + 2)*/ * REST_UNIT_STRIDE],
    src: &[BD::Pixel; 70 /*(64 + 3 + 3)*/ * REST_UNIT_STRIDE],
    w: usize,
    h: usize,
) {
    // We skip the first row, as it is never used
    let src = &src[REST_UNIT_STRIDE..];

    // We skip the first and last columns, as they are never used
    for x in 1..w - 1 {
        let mut sum_v = &mut sum[x..];
        let mut sumsq_v = &mut sumsq[x..];
        let mut s = &src[x..];
        let mut a: c_int = s[0].as_();
        let mut a2 = a * a;
        let mut b: c_int = s[REST_UNIT_STRIDE].as_();
        let mut b2 = b * b;

        // We skip the first 2 rows, as they are skipped in the next loop and
        // we don't need the last 2 row as it is skipped in the next loop
        for _ in 2..h - 2 {
            s = &s[REST_UNIT_STRIDE..];
            let c: c_int = s[REST_UNIT_STRIDE].as_();
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
        for x in 2..w - 2 {
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
/// ```text
/// c c c c c c c c c c
/// c c c c c c c c c c
/// i i s s s s s s i i
/// i i s s s s s s i i
/// i i s s s s s s i i
/// i i s s s s s s i i
/// i i s s s s s s i i
/// i i s s s s s s i i
/// c c c c c c c c c c
/// c c c c c c c c c c
/// ```
///
/// * s: Pixel summed and stored
/// * i: Pixel summed and stored (between loops)
/// * c: Pixel summed not stored
/// * x: Pixel not summed not stored
fn boxsum5<BD: BitDepth>(
    sumsq: &mut [i32; 68 /*(64 + 2 + 2)*/ * REST_UNIT_STRIDE],
    sum: &mut [BD::Coef; 68 /*(64 + 2 + 2)*/ * REST_UNIT_STRIDE],
    src: &[BD::Pixel; 70 /*(64 + 3 + 3)*/ * REST_UNIT_STRIDE],
    w: usize,
    h: usize,
) {
    for x in 0..w {
        let mut sum_v = &mut sum[x..];
        let mut sumsq_v = &mut sumsq[x..];
        let s = &src[x..];
        let mut a: c_int = (s[0]).as_();
        let mut a2 = a * a;
        let mut b: c_int = (s[1 * REST_UNIT_STRIDE]).as_();
        let mut b2 = b * b;
        let mut c: c_int = (s[2 * REST_UNIT_STRIDE]).as_();
        let mut c2 = c * c;
        let mut d: c_int = (s[3 * REST_UNIT_STRIDE]).as_();
        let mut d2 = d * d;

        let mut s = &src[3 * REST_UNIT_STRIDE + x..];

        // We skip the first 2 rows, as they are skipped in the next loop and
        // we don't need the last 2 row as it is skipped in the next loop
        for _ in 2..h - 2 {
            s = &s[REST_UNIT_STRIDE..];
            let e: c_int = s[0].as_();
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

        for x in 2..w - 2 {
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
    dst: &mut [BD::Coef; 24576],
    src: &[BD::Pixel; 27300],
    _src_stride: ptrdiff_t,
    w: usize,
    h: usize,
    n: c_int,
    s: c_uint,
    bd: BD,
) {
    let sgr_one_by_x = if n == 25 { 164 } else { 455 };

    // Selfguided filter is applied to a maximum stripe height of 64 + 3 pixels
    // of padding above and below
    let mut sumsq = [0; 68 /*(64 + 2 + 2)*/ * REST_UNIT_STRIDE];
    // By inverting A and B after the boxsums, B can be of size coef instead
    // of i32
    let mut sum = [0.as_::<BD::Coef>(); 68 /*(64 + 2 + 2)*/ * REST_UNIT_STRIDE];

    let step = (n == 25) as c_int + 1;
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
    for _ in (-1..h as isize + 1).step_by(step as usize) {
        for i in -1..w as isize + 1 {
            let a = AA[i] + (1 << 2 * bitdepth_min_8 >> 1) >> 2 * bitdepth_min_8;
            let b = BB[i].as_::<c_int>() + (1 << bitdepth_min_8 >> 1) >> bitdepth_min_8;

            let p = cmp::max(a * n - b * b, 0) as c_uint;
            let z = (p * s + (1 << 19)) >> 20;
            let x = dav1d_sgr_x_by_x[cmp::min(z, 255) as usize] as c_uint;

            // This is where we invert A and B, so that B is of size coef.
            AA[i] = ((x * BB[i].as_::<c_uint>() * sgr_one_by_x + (1 << 11)) >> 12) as c_int;
            BB[i] = x.as_::<BD::Coef>();
        }
        AA += step as usize * REST_UNIT_STRIDE;
        BB += step as usize * REST_UNIT_STRIDE;
    }

    fn six_neighbors<P>(p: &CursorMut<P>, i: isize) -> c_int
    where
        P: Add<Output = P> + ToPrimitive<c_int> + Copy,
    {
        let stride = REST_UNIT_STRIDE as isize;
        (p[i - stride] + p[i + stride]).as_::<c_int>() * 6
            + (p[i - 1 - stride] + p[i - 1 + stride] + p[i + 1 - stride] + p[i + 1 + stride])
                .as_::<c_int>()
                * 5
    }

    fn eight_neighbors<P>(p: &CursorMut<P>, i: isize) -> c_int
    where
        P: Add<Output = P> + ToPrimitive<c_int> + Copy,
    {
        let stride = REST_UNIT_STRIDE as isize;
        (p[i] + p[i - 1] + p[i + 1] + p[i - stride] + p[i + stride]).as_::<c_int>() * 4
            + (p[i - 1 - stride] + p[i - 1 + stride] + p[i + 1 - stride] + p[i + 1 + stride])
                .as_::<c_int>()
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
                dst[i] = ((b - a * src[i].as_::<c_int>() + (1 << 8)) >> 9).as_();
            }
            dst = &mut dst[384.. /* Maximum restoration width is 384 (256 * 1.5) */];
            src = &src[REST_UNIT_STRIDE..];
            B += REST_UNIT_STRIDE;
            A += REST_UNIT_STRIDE;
            for i in 0..w {
                let a = B[i].as_::<c_int>() * 6 + (B[i as isize - 1] + B[i + 1]).as_::<c_int>() * 5;
                let b = A[i] * 6 + (A[i as isize - 1] + A[i + 1]) * 5;
                dst[i] = (b - a * src[i].as_::<c_int>() + (1 << 7) >> 8).as_();
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
                dst[i] = (b - a * src[i].as_::<c_int>() + (1 << 8) >> 9).as_();
            }
        }
    } else {
        for _ in 0..h {
            for i in 0..w {
                let a = eight_neighbors(&B, i as isize);
                let b = eight_neighbors(&A, i as isize);
                dst[i] = (b - a * src[i].as_::<c_int>() + (1 << 8) >> 9).as_();
            }
            dst = &mut dst[384..];
            src = &src[REST_UNIT_STRIDE..];
            B += REST_UNIT_STRIDE;
            A += REST_UNIT_STRIDE;
        }
    };
}

/// # Safety
///
/// Must be called by [`loop_restoration_filter::Fn::call`].
unsafe extern "C" fn sgr_5x5_c_erased<BD: BitDepth>(
    _p_ptr: *mut DynPixel,
    _stride: ptrdiff_t,
    left: *const LeftPixelRow<DynPixel>,
    lpf: *const DynPixel,
    w: c_int,
    h: c_int,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: c_int,
    p: *const FFISafe<Rav1dPictureDataComponentOffset>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `loop_restoration_filter::Fn::call`.
    let p = *unsafe { FFISafe::get(p) };
    let left = left.cast();
    let lpf = lpf.cast();
    let w = w as usize;
    let h = h as usize;
    let bd = BD::from_c(bitdepth_max);
    // SAFETY: Length sliced in `loop_restoration_filter::Fn::call`.
    let left = unsafe { slice::from_raw_parts(left, h) };
    sgr_5x5_rust(p, left, lpf, w, h, params, edges, bd)
}

unsafe fn sgr_5x5_rust<BD: BitDepth>(
    p: Rav1dPictureDataComponentOffset,
    left: &[LeftPixelRow<BD::Pixel>],
    lpf: *const BD::Pixel,
    w: usize,
    h: usize,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bd: BD,
) {
    // Selfguided filter is applied to a maximum stripe height of 64 + 3 pixels
    // of padding above and below
    let mut tmp = [0.as_(); 70 /*(64 + 3 + 3)*/ * REST_UNIT_STRIDE];

    // Selfguided filter outputs to a maximum stripe height of 64 and a
    // maximum restoration width of 384 (256 * 1.5)
    let mut dst = [0.as_(); 64 * 384];

    padding::<BD>(&mut tmp, p, left, lpf, w, h, edges);
    let sgr = params.sgr();
    selfguided_filter(
        &mut dst,
        &mut tmp,
        REST_UNIT_STRIDE as ptrdiff_t,
        w,
        h,
        25,
        sgr.s0,
        bd,
    );

    let w0 = sgr.w0 as c_int;
    for j in 0..h {
        let p = p + (j as isize * p.data.pixel_stride::<BD>());
        let p = &mut *p.data.slice_mut::<BD, _>((p.offset.., ..w));
        for i in 0..w {
            let v = w0 * dst[j * 384 + i].as_::<c_int>();
            p[i] = bd.iclip_pixel(p[i].as_::<c_int>() + (v + (1 << 10) >> 11));
        }
    }
}

/// # Safety
///
/// Must be called by [`loop_restoration_filter::Fn::call`].
unsafe extern "C" fn sgr_3x3_c_erased<BD: BitDepth>(
    _p_ptr: *mut DynPixel,
    _stride: ptrdiff_t,
    left: *const LeftPixelRow<DynPixel>,
    lpf: *const DynPixel,
    w: c_int,
    h: c_int,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: c_int,
    p: *const FFISafe<Rav1dPictureDataComponentOffset>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `loop_restoration_filter::Fn::call`.
    let p = *unsafe { FFISafe::get(p) };
    let left = left.cast();
    let lpf = lpf.cast();
    let w = w as usize;
    let h = h as usize;
    let bd = BD::from_c(bitdepth_max);
    // SAFETY: Length sliced in `loop_restoration_filter::Fn::call`.
    let left = unsafe { slice::from_raw_parts(left, h) };
    sgr_3x3_rust(p, left, lpf, w, h, params, edges, bd)
}

unsafe fn sgr_3x3_rust<BD: BitDepth>(
    p: Rav1dPictureDataComponentOffset,
    left: &[LeftPixelRow<BD::Pixel>],
    lpf: *const BD::Pixel,
    w: usize,
    h: usize,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bd: BD,
) {
    let mut tmp = [0.as_(); 70 /*(64 + 3 + 3)*/ * REST_UNIT_STRIDE];
    let mut dst = [0.as_(); 64 * 384];

    padding::<BD>(&mut tmp, p, left, lpf, w, h, edges);
    let sgr = params.sgr();
    selfguided_filter(
        &mut dst,
        &mut tmp,
        REST_UNIT_STRIDE as ptrdiff_t,
        w,
        h,
        9,
        sgr.s1,
        bd,
    );

    let w1 = sgr.w1 as c_int;
    for j in 0..h {
        let p = p + (j as isize * p.data.pixel_stride::<BD>());
        let p = &mut *p.data.slice_mut::<BD, _>((p.offset.., ..w));
        for i in 0..w {
            let v = w1 * dst[j * 384 + i].as_::<c_int>();
            p[i] = bd.iclip_pixel(p[i].as_::<c_int>() + (v + (1 << 10) >> 11));
        }
    }
}

/// # Safety
///
/// Must be called by [`loop_restoration_filter::Fn::call`].
unsafe extern "C" fn sgr_mix_c_erased<BD: BitDepth>(
    _p_ptr: *mut DynPixel,
    _stride: ptrdiff_t,
    left: *const LeftPixelRow<DynPixel>,
    lpf: *const DynPixel,
    w: c_int,
    h: c_int,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: c_int,
    p: *const FFISafe<Rav1dPictureDataComponentOffset>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `loop_restoration_filter::Fn::call`.
    let p = *unsafe { FFISafe::get(p) };
    let left = left.cast();
    let lpf = lpf.cast();
    let w = w as usize;
    let h = h as usize;
    let bd = BD::from_c(bitdepth_max);
    // SAFETY: Length sliced in `loop_restoration_filter::Fn::call`.
    let left = unsafe { slice::from_raw_parts(left, h) };
    sgr_mix_rust(p, left, lpf, w, h, params, edges, bd)
}

unsafe fn sgr_mix_rust<BD: BitDepth>(
    p: Rav1dPictureDataComponentOffset,
    left: &[LeftPixelRow<BD::Pixel>],
    lpf: *const BD::Pixel,
    w: usize,
    h: usize,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bd: BD,
) {
    let mut tmp = [0.as_(); 70 /*(64 + 3 + 3)*/ * REST_UNIT_STRIDE];
    let mut dst0 = [0.as_(); 64 * 384];
    let mut dst1 = [0.as_(); 64 * 384];

    padding::<BD>(&mut tmp, p, left, lpf, w, h, edges);
    let sgr = params.sgr();
    selfguided_filter(
        &mut dst0,
        &mut tmp,
        REST_UNIT_STRIDE as ptrdiff_t,
        w,
        h,
        25,
        sgr.s0,
        bd,
    );
    selfguided_filter(
        &mut dst1,
        &mut tmp,
        REST_UNIT_STRIDE as ptrdiff_t,
        w,
        h,
        9,
        sgr.s1,
        bd,
    );

    let w0 = sgr.w0 as c_int;
    let w1 = sgr.w1 as c_int;
    for j in 0..h {
        let p = p + (j as isize * p.data.pixel_stride::<BD>());
        let p = &mut *p.data.slice_mut::<BD, _>((p.offset.., ..w));
        for i in 0..w {
            let v = w0 * dst0[j * 384 + i].as_::<c_int>() + w1 * dst1[j * 384 + i].as_::<c_int>();
            p[i] = bd.iclip_pixel(p[i].as_::<c_int>() + (v + (1 << 10) >> 11));
        }
    }
}

#[cfg(all(feature = "asm", target_arch = "arm"))]
unsafe fn rav1d_wiener_filter_h_neon<BD: BitDepth>(
    dst: &mut [i16],
    left: *const LeftPixelRow<BD::Pixel>,
    src: *const BD::Pixel,
    stride: ptrdiff_t,
    fh: &[i16; 8],
    w: intptr_t,
    h: c_int,
    edges: LrEdgeFlags,
    bd: BD,
) {
    macro_rules! asm_fn {
        ($name:ident) => {{
            extern "C" {
                fn $name(
                    dst: *mut i16,
                    left: *const c_void,
                    src: *const c_void,
                    stride: ptrdiff_t,
                    fh: *const i16,
                    w: intptr_t,
                    h: c_int,
                    edges: LrEdgeFlags,
                    bitdepth_max: c_int,
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
        fh.as_ptr(),
        w,
        h,
        edges,
        bd.into_c(),
    )
}

#[cfg(all(feature = "asm", target_arch = "arm"))]
unsafe fn rav1d_wiener_filter_v_neon<BD: BitDepth>(
    dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    mid: &mut [i16],
    w: c_int,
    h: c_int,
    fv: &[i16; 8],
    edges: LrEdgeFlags,
    mid_stride: ptrdiff_t,
    bd: BD,
) {
    macro_rules! asm_fn {
        ($name:ident) => {{
            extern "C" {
                fn $name(
                    dst: *mut c_void,
                    stride: ptrdiff_t,
                    mid: *const i16,
                    w: c_int,
                    h: c_int,
                    fv: *const i16,
                    edges: LrEdgeFlags,
                    mid_stride: ptrdiff_t,
                    bitdepth_max: c_int,
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
        fv.as_ptr(),
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
    w: c_int,
    h: c_int,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: c_int,
    _p: *const FFISafe<Rav1dPictureDataComponentOffset>,
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
    left: *const LeftPixelRow<BD::Pixel>,
    lpf: *const BD::Pixel,
    w: c_int,
    h: c_int,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bd: BD,
) {
    let filter = &params.filter;
    let mut mid: Align16<[i16; 68 * 384]> = Align16([0; 68 * 384]);
    let mid_stride: c_int = w + 7 & !7;
    rav1d_wiener_filter_h_neon(
        &mut mid.0[2 * mid_stride as usize..],
        left,
        dst,
        stride,
        &filter[0],
        w as intptr_t,
        h,
        edges,
        bd,
    );
    if edges & LR_HAVE_TOP != 0 {
        rav1d_wiener_filter_h_neon(
            &mut mid.0[..],
            core::ptr::null(),
            lpf,
            stride,
            &filter[0],
            w as intptr_t,
            2,
            edges,
            bd,
        );
    }
    if edges & LR_HAVE_BOTTOM != 0 {
        rav1d_wiener_filter_h_neon(
            &mut mid.0[(2 + h as usize) * mid_stride as usize..],
            core::ptr::null(),
            lpf.offset(6 * BD::pxstride(stride)),
            stride,
            &filter[0],
            w as intptr_t,
            2,
            edges,
            bd,
        );
    }
    rav1d_wiener_filter_v_neon(
        dst,
        stride,
        &mut mid.0[2 * mid_stride as usize..],
        w,
        h,
        &filter[1],
        edges,
        (mid_stride as usize * ::core::mem::size_of::<i16>()) as ptrdiff_t,
        bd,
    );
}

#[cfg(all(feature = "asm", target_arch = "arm"))]
unsafe fn rav1d_sgr_box3_h_neon<BD: BitDepth>(
    sumsq: *mut i32,
    sum: *mut i16,
    left: Option<&[LeftPixelRow<BD::Pixel>]>,
    src: *const BD::Pixel,
    stride: ptrdiff_t,
    w: c_int,
    h: c_int,
    edges: LrEdgeFlags,
) {
    macro_rules! asm_fn {
        ($name:ident) => {{
            extern "C" {
                fn $name(
                    sumsq: *mut i32,
                    sum: *mut i16,
                    left: *const c_void,
                    src: *const c_void,
                    stride: ptrdiff_t,
                    w: c_int,
                    h: c_int,
                    edges: LrEdgeFlags,
                );
            }
            $name
        }};
    }
    (match BD::BPC {
        BPC::BPC8 => asm_fn!(dav1d_sgr_box3_h_8bpc_neon),
        BPC::BPC16 => asm_fn!(dav1d_sgr_box3_h_16bpc_neon),
    })(
        sumsq,
        sum,
        left.map(|left| left.as_ptr().cast())
            .unwrap_or_else(ptr::null),
        src.cast(),
        stride,
        w,
        h,
        edges,
    )
}

#[cfg(all(feature = "asm", any(target_arch = "arm")))]
unsafe fn rav1d_sgr_finish_filter1_neon<BD: BitDepth>(
    tmp: &mut [i16; 64 * 384],
    src: Rav1dPictureDataComponentOffset,
    a: *const i32,
    b: *const i16,
    w: c_int,
    h: c_int,
) {
    macro_rules! asm_fn {
        ($name:ident) => {{
            extern "C" {
                fn $name(
                    tmp: *mut i16,
                    src: *const c_void,
                    stride: ptrdiff_t,
                    a: *const i32,
                    b: *const i16,
                    w: c_int,
                    h: c_int,
                );
            }
            $name
        }};
    }
    (match BD::BPC {
        BPC::BPC8 => asm_fn!(dav1d_sgr_finish_filter1_8bpc_neon),
        BPC::BPC16 => asm_fn!(dav1d_sgr_finish_filter1_16bpc_neon),
    })(
        tmp.as_mut_ptr(),
        src.data.as_ptr_at::<BD>(src.offset).cast(),
        src.data.stride(),
        a,
        b,
        w,
        h,
    )
}

#[cfg(all(feature = "asm", target_arch = "arm"))]
unsafe fn rav1d_sgr_filter1_neon<BD: BitDepth>(
    tmp: &mut [i16; 64 * 384],
    src: Rav1dPictureDataComponentOffset,
    left: &[LeftPixelRow<BD::Pixel>],
    lpf: *const BD::Pixel,
    w: c_int,
    h: c_int,
    strength: u32,
    edges: LrEdgeFlags,
    bd: BD,
) {
    let mut sumsq_mem: Align16<[i32; 27208]> = Align16([0; 27208]);
    let sumsq: *mut i32 = &mut *sumsq_mem
        .0
        .as_mut_ptr()
        .offset(((384 + 16) * 2 + 8) as isize) as *mut i32;
    let a: *mut i32 = sumsq;
    let mut sum_mem: Align16<[i16; 27216]> = Align16([0; 27216]);
    let sum: *mut i16 = &mut *sum_mem
        .0
        .as_mut_ptr()
        .offset(((384 + 16) * 2 + 16) as isize) as *mut i16;
    let b: *mut i16 = sum;
    rav1d_sgr_box3_h_neon::<BD>(
        sumsq,
        sum,
        Some(left),
        src.data.as_ptr_at::<BD>(src.offset),
        src.data.stride(),
        w,
        h,
        edges,
    );
    if edges as c_uint & LR_HAVE_TOP as c_int as c_uint != 0 {
        rav1d_sgr_box3_h_neon::<BD>(
            &mut *sumsq.offset((-(2 as c_int) * (384 + 16)) as isize),
            &mut *sum.offset((-(2 as c_int) * (384 + 16)) as isize),
            None,
            lpf,
            src.data.stride(),
            w,
            2 as c_int,
            edges,
        );
    }
    if edges as c_uint & LR_HAVE_BOTTOM as c_int as c_uint != 0 {
        rav1d_sgr_box3_h_neon::<BD>(
            &mut *sumsq.offset((h * (384 + 16)) as isize),
            &mut *sum.offset((h * (384 + 16)) as isize),
            None,
            lpf.offset(6 * src.data.pixel_stride::<BD>()),
            src.data.stride(),
            w,
            2 as c_int,
            edges,
        );
    }
    dav1d_sgr_box3_v_neon(sumsq, sum, w, h, edges);
    dav1d_sgr_calc_ab1_neon(a, b, w, h, strength as c_int, bd.into_c());
    rav1d_sgr_finish_filter1_neon::<BD>(tmp, src, a, b, w, h);
}

#[cfg(all(feature = "asm", any(target_arch = "arm")))]
unsafe fn rav1d_sgr_box5_h_neon<BD: BitDepth>(
    sumsq: *mut i32,
    sum: *mut i16,
    left: Option<&[LeftPixelRow<BD::Pixel>]>,
    src: *const BD::Pixel,
    stride: ptrdiff_t,
    w: c_int,
    h: c_int,
    edges: LrEdgeFlags,
) {
    macro_rules! asm_fn {
        ($name:ident) => {{
            extern "C" {
                fn $name(
                    sumsq: *mut i32,
                    sum: *mut i16,
                    left: *const c_void,
                    src: *const c_void,
                    stride: ptrdiff_t,
                    w: c_int,
                    h: c_int,
                    edges: LrEdgeFlags,
                );
            }
            $name
        }};
    }
    (match BD::BPC {
        BPC::BPC8 => asm_fn!(dav1d_sgr_box5_h_8bpc_neon),
        BPC::BPC16 => asm_fn!(dav1d_sgr_box5_h_16bpc_neon),
    })(
        sumsq,
        sum,
        left.map(|left| left.as_ptr().cast())
            .unwrap_or_else(ptr::null),
        src.cast(),
        stride,
        w,
        h,
        edges,
    )
}

#[cfg(all(feature = "asm", any(target_arch = "arm")))]
unsafe fn rav1d_sgr_finish_filter2_neon<BD: BitDepth>(
    tmp: &mut [i16; 64 * 384],
    src: Rav1dPictureDataComponentOffset,
    a: *const i32,
    b: *const i16,
    w: c_int,
    h: c_int,
) {
    macro_rules! asm_fn {
        ($name:ident) => {{
            extern "C" {
                fn $name(
                    tmp: *mut i16,
                    src: *const c_void,
                    stride: ptrdiff_t,
                    a: *const i32,
                    b: *const i16,
                    w: c_int,
                    h: c_int,
                );
            }
            $name
        }};
    }
    (match BD::BPC {
        BPC::BPC8 => asm_fn!(dav1d_sgr_finish_filter2_8bpc_neon),
        BPC::BPC16 => asm_fn!(dav1d_sgr_finish_filter2_16bpc_neon),
    })(
        tmp.as_mut_ptr(),
        src.data.as_ptr_at::<BD>(src.offset).cast(),
        src.data.stride(),
        a,
        b,
        w,
        h,
    )
}

#[cfg(all(feature = "asm", target_arch = "arm"))]
unsafe fn rav1d_sgr_filter2_neon<BD: BitDepth>(
    tmp: &mut [i16; 64 * 384],
    src: Rav1dPictureDataComponentOffset,
    left: &[LeftPixelRow<BD::Pixel>],
    lpf: *const BD::Pixel,
    w: c_int,
    h: c_int,
    strength: u32,
    edges: LrEdgeFlags,
    bd: BD,
) {
    let mut sumsq_mem: Align16<[i32; 27208]> = Align16([0; 27208]);
    let sumsq: *mut i32 = &mut *sumsq_mem
        .0
        .as_mut_ptr()
        .offset(((384 + 16) * 2 + 8) as isize) as *mut i32;
    let a: *mut i32 = sumsq;
    let mut sum_mem: Align16<[i16; 27216]> = Align16([0; 27216]);
    let sum: *mut i16 = &mut *sum_mem
        .0
        .as_mut_ptr()
        .offset(((384 + 16) * 2 + 16) as isize) as *mut i16;
    let b: *mut i16 = sum;
    rav1d_sgr_box5_h_neon::<BD>(
        sumsq,
        sum,
        Some(left),
        src.data.as_ptr_at::<BD>(src.offset),
        src.data.stride(),
        w,
        h,
        edges,
    );
    if edges as c_uint & LR_HAVE_TOP as c_int as c_uint != 0 {
        rav1d_sgr_box5_h_neon::<BD>(
            &mut *sumsq.offset((-(2 as c_int) * (384 + 16)) as isize),
            &mut *sum.offset((-(2 as c_int) * (384 + 16)) as isize),
            None,
            lpf,
            src.data.stride(),
            w,
            2,
            edges,
        );
    }
    if edges as c_uint & LR_HAVE_BOTTOM as c_int as c_uint != 0 {
        rav1d_sgr_box5_h_neon::<BD>(
            &mut *sumsq.offset((h * (384 + 16)) as isize),
            &mut *sum.offset((h * (384 + 16)) as isize),
            None,
            lpf.offset(6 * src.data.pixel_stride::<BD>()),
            src.data.stride(),
            w,
            2,
            edges,
        );
    }
    dav1d_sgr_box5_v_neon(sumsq, sum, w, h, edges);
    dav1d_sgr_calc_ab2_neon(a, b, w, h, strength as c_int, bd.into_c());
    rav1d_sgr_finish_filter2_neon::<BD>(tmp, src, a, b, w, h);
}

#[cfg(all(feature = "asm", any(target_arch = "arm")))]
unsafe fn rav1d_sgr_weighted1_neon<BD: BitDepth>(
    dst: Rav1dPictureDataComponentOffset,
    src: Rav1dPictureDataComponentOffset,
    t1: &mut [i16; 64 * 384],
    w: c_int,
    h: c_int,
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
                    t1: *const i16,
                    w: c_int,
                    h: c_int,
                    wt: c_int,
                    bitdepth_max: c_int,
                );
            }
            $name
        }};
    }
    (match BD::BPC {
        BPC::BPC8 => asm_fn!(dav1d_sgr_weighted1_8bpc_neon),
        BPC::BPC16 => asm_fn!(dav1d_sgr_weighted1_16bpc_neon),
    })(
        dst.data.as_mut_ptr_at::<BD>(dst.offset).cast(),
        dst.data.stride(),
        src.data.as_ptr_at::<BD>(src.offset).cast(),
        src.data.stride(),
        t1.as_mut_ptr(),
        w,
        h,
        wt.into(),
        bd.into_c(),
    )
}

#[cfg(all(feature = "asm", any(target_arch = "arm")))]
unsafe fn rav1d_sgr_weighted2_neon<BD: BitDepth>(
    dst: Rav1dPictureDataComponentOffset,
    src: Rav1dPictureDataComponentOffset,
    t1: &mut [i16; 64 * 384],
    t2: &mut [i16; 64 * 384],
    w: c_int,
    h: c_int,
    wt: &[i16; 2],
    bd: BD,
) {
    macro_rules! asm_fn {
        ($name:ident) => {{
            extern "C" {
                fn $name(
                    dst: *mut c_void,
                    dst_stride: ptrdiff_t,
                    src: *const c_void,
                    src_stride: ptrdiff_t,
                    t1: *const i16,
                    t2: *const i16,
                    w: c_int,
                    h: c_int,
                    wt: *const i16,
                    bitdepth_max: c_int,
                );
            }
            $name
        }};
    }
    (match BD::BPC {
        BPC::BPC8 => asm_fn!(dav1d_sgr_weighted2_8bpc_neon),
        BPC::BPC16 => asm_fn!(dav1d_sgr_weighted2_16bpc_neon),
    })(
        dst.data.as_mut_ptr_at::<BD>(dst.offset).cast(),
        dst.data.stride(),
        src.data.as_ptr_at::<BD>(src.offset).cast(),
        src.data.stride(),
        t1.as_mut_ptr(),
        t2.as_mut_ptr(),
        w,
        h,
        wt.as_ptr(),
        bd.into_c(),
    )
}

/// # Safety
///
/// Must be called by [`loop_restoration_filter::Fn::call`].
#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn sgr_filter_5x5_neon_erased<BD: BitDepth>(
    _p_ptr: *mut DynPixel,
    _stride: ptrdiff_t,
    left: *const LeftPixelRow<DynPixel>,
    lpf: *const DynPixel,
    w: c_int,
    h: c_int,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: c_int,
    p: *const FFISafe<Rav1dPictureDataComponentOffset>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `loop_restoration_filter::Fn::call`.
    let p = *unsafe { FFISafe::get(p) };
    let left = left.cast();
    let lpf = lpf.cast();
    let bd = BD::from_c(bitdepth_max);
    let w = w as usize;
    let h = h as usize;
    // SAFETY: Length sliced in `loop_restoration_filter::Fn::call`.
    let left = unsafe { slice::from_raw_parts(left, h) };
    sgr_filter_5x5_neon(p, left, lpf, w, h, params, edges, bd)
}

#[cfg(all(feature = "asm", target_arch = "arm"))]
unsafe fn sgr_filter_5x5_neon<BD: BitDepth>(
    dst: Rav1dPictureDataComponentOffset,
    left: &[LeftPixelRow<BD::Pixel>],
    lpf: *const BD::Pixel,
    w: usize,
    h: usize,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bd: BD,
) {
    let w = w as c_int;
    let h = h as c_int;
    let mut tmp = Align16([0; 64 * 384]);
    let sgr = params.sgr();
    rav1d_sgr_filter2_neon(&mut tmp.0, dst, left, lpf, w, h, sgr.s0, edges, bd);
    rav1d_sgr_weighted1_neon(dst, dst, &mut tmp.0, w, h, sgr.w0, bd);
}

/// # Safety
///
/// Must be called by [`loop_restoration_filter::Fn::call`].
#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn sgr_filter_3x3_neon_erased<BD: BitDepth>(
    _p_ptr: *mut DynPixel,
    _stride: ptrdiff_t,
    left: *const LeftPixelRow<DynPixel>,
    lpf: *const DynPixel,
    w: c_int,
    h: c_int,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: c_int,
    p: *const FFISafe<Rav1dPictureDataComponentOffset>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `loop_restoration_filter::Fn::call`.
    let p = *unsafe { FFISafe::get(p) };
    let left = left.cast();
    let lpf = lpf.cast();
    let w = w as usize;
    let h = h as usize;
    let bd = BD::from_c(bitdepth_max);
    // SAFETY: Length sliced in `loop_restoration_filter::Fn::call`.
    let left = unsafe { slice::from_raw_parts(left, h) };
    sgr_filter_3x3_neon(p, left, lpf, w, h, params, edges, bd)
}

#[cfg(all(feature = "asm", target_arch = "arm"))]
unsafe fn sgr_filter_3x3_neon<BD: BitDepth>(
    dst: Rav1dPictureDataComponentOffset,
    left: &[LeftPixelRow<BD::Pixel>],
    lpf: *const BD::Pixel,
    w: usize,
    h: usize,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bd: BD,
) {
    let w = w as c_int;
    let h = h as c_int;
    let mut tmp = Align16([0; 64 * 384]);
    let sgr = params.sgr();
    rav1d_sgr_filter1_neon(&mut tmp.0, dst, left, lpf, w, h, sgr.s1, edges, bd);
    rav1d_sgr_weighted1_neon(dst, dst, &mut tmp.0, w, h, sgr.w1, bd);
}

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
unsafe fn rotate5_x2(sumsq_ptrs: &mut [*mut i32; 5], sum_ptrs: &mut [*mut i16; 5]) {
    sumsq_ptrs.rotate_left(2);
    sum_ptrs.rotate_left(2);
}

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
unsafe fn rotate_ab_2(A_ptrs: &mut [*mut i32; 2], B_ptrs: &mut [*mut i16; 2]) {
    A_ptrs.rotate_left(1);
    B_ptrs.rotate_left(1);
}

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
unsafe fn rotate_ab_3(A_ptrs: &mut [*mut i32; 3], B_ptrs: &mut [*mut i16; 3]) {
    A_ptrs.rotate_left(1);
    B_ptrs.rotate_left(1);
}

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
unsafe fn rotate_ab_4(A_ptrs: &mut [*mut i32; 4], B_ptrs: &mut [*mut i16; 4]) {
    A_ptrs.rotate_left(1);
    B_ptrs.rotate_left(1);
}

#[cfg(all(feature = "asm", any(target_arch = "aarch64")))]
unsafe fn rav1d_sgr_box3_row_h_neon<BD: BitDepth>(
    sumsq: *mut i32,
    sum: *mut i16,
    left: Option<&[LeftPixelRow<BD::Pixel>]>,
    src: *const BD::Pixel,
    w: c_int,
    edges: LrEdgeFlags,
    bd: BD,
) {
    macro_rules! asm_fn {
        (fn $name:ident) => {{
            extern "C" {
                fn $name(
                    sumsq: *mut i32,
                    sum: *mut i16,
                    left: *const LeftPixelRow<DynPixel>,
                    src: *const DynPixel,
                    w: c_int,
                    edges: LrEdgeFlags,
                    bitdepth_max: c_int,
                );
            }
            $name
        }};
    }

    bd_fn!(asm_fn, BD, sgr_box3_row_h, neon)(
        sumsq,
        sum,
        left.map(|left| left.as_ptr().cast())
            .unwrap_or_else(ptr::null),
        src.cast(),
        w,
        edges,
        bd.into_c(),
    )
}

#[cfg(all(feature = "asm", any(target_arch = "aarch64")))]
unsafe fn rav1d_sgr_box5_row_h_neon<BD: BitDepth>(
    sumsq: *mut i32,
    sum: *mut i16,
    left: Option<&[LeftPixelRow<BD::Pixel>]>,
    src: *const BD::Pixel,
    w: c_int,
    edges: LrEdgeFlags,
    bd: BD,
) {
    macro_rules! asm_fn {
        (fn $name:ident) => {{
            extern "C" {
                fn $name(
                    sumsq: *mut i32,
                    sum: *mut i16,
                    left: *const LeftPixelRow<DynPixel>,
                    src: *const DynPixel,
                    w: c_int,
                    edges: LrEdgeFlags,
                    bitdepth_max: c_int,
                );
            }
            $name
        }};
    }

    bd_fn!(asm_fn, BD, sgr_box5_row_h, neon)(
        sumsq,
        sum,
        left.map(|left| left.as_ptr().cast())
            .unwrap_or_else(ptr::null),
        src.cast(),
        w,
        edges,
        bd.into_c(),
    )
}

#[cfg(all(feature = "asm", any(target_arch = "aarch64")))]
unsafe fn rav1d_sgr_box35_row_h_neon<BD: BitDepth>(
    sumsq3: *mut i32,
    sum3: *mut i16,
    sumsq5: *mut i32,
    sum5: *mut i16,
    left: Option<&[LeftPixelRow<BD::Pixel>]>,
    src: *const BD::Pixel,
    w: c_int,
    edges: LrEdgeFlags,
    bd: BD,
) {
    macro_rules! asm_fn {
        (fn $name:ident) => {{
            extern "C" {
                fn $name(
                    sumsq3: *mut i32,
                    sum3: *mut i16,
                    sumsq5: *mut i32,
                    sum5: *mut i16,
                    left: *const LeftPixelRow<DynPixel>,
                    src: *const DynPixel,
                    w: c_int,
                    edges: LrEdgeFlags,
                    bitdepth_max: c_int,
                );
            }
            $name
        }};
    }
    bd_fn!(asm_fn, BD, sgr_box35_row_h, neon)(
        sumsq3,
        sum3,
        sumsq5,
        sum5,
        left.map(|left| left.as_ptr().cast())
            .unwrap_or_else(ptr::null),
        src.cast(),
        w,
        edges,
        bd.into_c(),
    )
}

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
extern "C" {
    fn dav1d_sgr_box3_vert_neon(
        sumsq: *mut *mut i32,
        sum: *mut *mut i16,
        AA: *mut i32,
        BB: *mut i16,
        w: c_int,
        s: c_int,
        bitdepth_max: c_int,
    );

    fn dav1d_sgr_box5_vert_neon(
        sumsq: *mut *mut i32,
        sum: *mut *mut i16,
        AA: *mut i32,
        BB: *mut i16,
        w: c_int,
        s: c_int,
        bitdepth_max: c_int,
    );
}

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
unsafe fn sgr_box3_vert_neon<BD: BitDepth>(
    sumsq: &mut [*mut i32; 3],
    sum: &mut [*mut i16; 3],
    sumsq_out: *mut i32,
    sum_out: *mut i16,
    w: c_int,
    s: c_int,
    bd: BD,
) {
    dav1d_sgr_box3_vert_neon(
        sumsq.as_mut_ptr(),
        sum.as_mut_ptr(),
        sumsq_out,
        sum_out,
        w,
        s,
        bd.into_c(),
    );
    rotate_ab_3(sumsq, sum);
}

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
unsafe fn sgr_box5_vert_neon<BD: BitDepth>(
    sumsq: &mut [*mut i32; 5],
    sum: &mut [*mut i16; 5],
    sumsq_out: *mut i32,
    sum_out: *mut i16,
    w: c_int,
    s: c_int,
    bd: BD,
) {
    dav1d_sgr_box5_vert_neon(
        sumsq.as_mut_ptr(),
        sum.as_mut_ptr(),
        sumsq_out,
        sum_out,
        w,
        s,
        bd.into_c(),
    );
    rotate5_x2(sumsq, sum);
}

#[cfg(all(feature = "asm", any(target_arch = "aarch64")))]
unsafe fn rav1d_sgr_finish_weighted1_neon<BD: BitDepth>(
    dst: Rav1dPictureDataComponentOffset,
    A_ptrs: &mut [*mut i32; 3],
    B_ptrs: &mut [*mut i16; 3],
    w: c_int,
    w1: c_int,
    bd: BD,
) {
    macro_rules! asm_fn {
        (fn $name:ident) => {{
            extern "C" {
                fn $name(
                    dst: *mut c_void,
                    A_ptrs: *mut *mut i32,
                    B_ptrs: *mut *mut i16,
                    w: c_int,
                    w1: c_int,
                    bitdepth_max: c_int,
                );
            }
            $name
        }};
    }
    bd_fn!(asm_fn, BD, sgr_finish_weighted1, neon)(
        dst.data.as_mut_ptr_at::<BD>(dst.offset).cast(),
        A_ptrs.as_mut_ptr(),
        B_ptrs.as_mut_ptr(),
        w,
        w1,
        bd.into_c(),
    )
}

#[cfg(all(feature = "asm", any(target_arch = "aarch64")))]
unsafe fn rav1d_sgr_finish_weighted2_neon<BD: BitDepth>(
    dst: Rav1dPictureDataComponentOffset,
    A_ptrs: &mut [*mut i32; 2],
    B_ptrs: &mut [*mut i16; 2],
    w: c_int,
    h: c_int,
    w1: c_int,
    bd: BD,
) {
    macro_rules! asm_fn {
        (fn $name:ident) => {{
            extern "C" {
                fn $name(
                    dst: *mut DynPixel,
                    stride: ptrdiff_t,
                    A_ptrs: *mut *mut i32,
                    B_ptrs: *mut *mut i16,
                    w: c_int,
                    h: c_int,
                    w1: c_int,
                    bitdepth_max: c_int,
                );
            }
            $name
        }};
    }
    bd_fn!(asm_fn, BD, sgr_finish_weighted2, neon)(
        dst.data.as_mut_ptr_at::<BD>(dst.offset).cast(),
        dst.data.stride(),
        A_ptrs.as_mut_ptr(),
        B_ptrs.as_mut_ptr(),
        w,
        h,
        w1,
        bd.into_c(),
    )
}

#[cfg(all(feature = "asm", any(target_arch = "aarch64")))]
unsafe fn rav1d_sgr_finish_filter1_2rows_neon<BD: BitDepth>(
    tmp: *mut i16,
    src: Rav1dPictureDataComponentOffset,
    A_ptrs: &mut [*mut i32; 4],
    B_ptrs: &mut [*mut i16; 4],
    w: c_int,
    h: c_int,
    bd: BD,
) {
    macro_rules! asm_fn {
        (fn $name:ident) => {{
            extern "C" {
                fn $name(
                    tmp: *mut i16,
                    src: *const DynPixel,
                    src_stride: ptrdiff_t,
                    A_ptrs: *mut *mut i32,
                    B_ptrs: *mut *mut i16,
                    w: c_int,
                    h: c_int,
                    bitdepth_max: c_int,
                );
            }
            $name
        }};
    }
    bd_fn!(asm_fn, BD, sgr_finish_filter1_2rows, neon)(
        tmp,
        src.data.as_ptr_at::<BD>(src.offset).cast(),
        src.data.stride(),
        A_ptrs.as_mut_ptr(),
        B_ptrs.as_mut_ptr(),
        w,
        h,
        bd.into_c(),
    )
}

#[cfg(all(feature = "asm", any(target_arch = "aarch64")))]
unsafe fn rav1d_sgr_finish_filter2_2rows_neon<BD: BitDepth>(
    tmp: *mut i16,
    src: Rav1dPictureDataComponentOffset,
    A_ptrs: &mut [*mut i32; 2],
    B_ptrs: &mut [*mut i16; 2],
    w: c_int,
    h: c_int,
    bd: BD,
) {
    macro_rules! asm_fn {
        (fn $name:ident) => {{
            extern "C" {
                fn $name(
                    tmp: *mut i16,
                    src: *const DynPixel,
                    src_stride: ptrdiff_t,
                    A_ptrs: *mut *mut i32,
                    B_ptrs: *mut *mut i16,
                    w: c_int,
                    h: c_int,
                    bitdepth_max: c_int,
                );
            }
            $name
        }};
    }
    bd_fn!(asm_fn, BD, sgr_finish_filter2_2rows, neon)(
        tmp,
        src.data.as_ptr_at::<BD>(src.offset).cast(),
        src.data.stride(),
        A_ptrs.as_mut_ptr(),
        B_ptrs.as_mut_ptr(),
        w,
        h,
        bd.into_c(),
    )
}

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
unsafe fn sgr_box3_hv_neon<BD: BitDepth>(
    sumsq: &mut [*mut i32; 3],
    sum: &mut [*mut i16; 3],
    AA: *mut i32,
    BB: *mut i16,
    left: Option<&[LeftPixelRow<BD::Pixel>]>,
    src: *const BD::Pixel,
    w: c_int,
    s: c_int,
    edges: LrEdgeFlags,
    bd: BD,
) {
    rav1d_sgr_box3_row_h_neon(sumsq[2], sum[2], left, src, w, edges, bd);
    sgr_box3_vert_neon(sumsq, sum, AA, BB, w, s, bd);
}

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
unsafe fn sgr_finish1_neon<BD: BitDepth>(
    dst: &mut Rav1dPictureDataComponentOffset,
    A_ptrs: &mut [*mut i32; 3],
    B_ptrs: &mut [*mut i16; 3],
    w: c_int,
    w1: c_int,
    bd: BD,
) {
    rav1d_sgr_finish_weighted1_neon(*dst, A_ptrs, B_ptrs, w, w1, bd);
    *dst += dst.data.pixel_stride::<BD>();
    rotate_ab_3(A_ptrs, B_ptrs);
}

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
unsafe fn sgr_finish2_neon<BD: BitDepth>(
    dst: &mut Rav1dPictureDataComponentOffset,
    A_ptrs: &mut [*mut i32; 2],
    B_ptrs: &mut [*mut i16; 2],
    w: c_int,
    h: c_int,
    w1: c_int,
    bd: BD,
) {
    rav1d_sgr_finish_weighted2_neon(*dst, A_ptrs, B_ptrs, w, h, w1, bd);
    *dst += 2 * dst.data.pixel_stride::<BD>();
    rotate_ab_2(A_ptrs, B_ptrs);
}

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
unsafe fn sgr_finish_mix_neon<BD: BitDepth>(
    dst: &mut Rav1dPictureDataComponentOffset,
    A5_ptrs: &mut [*mut i32; 2],
    B5_ptrs: &mut [*mut i16; 2],
    A3_ptrs: &mut [*mut i32; 4],
    B3_ptrs: &mut [*mut i16; 4],
    w: c_int,
    h: c_int,
    w0: c_int,
    w1: c_int,
    bd: BD,
) {
    const FILTER_OUT_STRIDE: usize = 384;

    let mut tmp5: Align16<[i16; 2 * FILTER_OUT_STRIDE]> = Align16([0; 2 * FILTER_OUT_STRIDE]);
    let mut tmp3: Align16<[i16; 2 * FILTER_OUT_STRIDE]> = Align16([0; 2 * FILTER_OUT_STRIDE]);

    rav1d_sgr_finish_filter2_2rows_neon(tmp5.0.as_mut_ptr(), *dst, A5_ptrs, B5_ptrs, w, h, bd);
    rav1d_sgr_finish_filter1_2rows_neon(tmp3.0.as_mut_ptr(), *dst, A3_ptrs, B3_ptrs, w, h, bd);

    let wt: [i16; 2] = [w0 as i16, w1 as i16];
    macro_rules! asm_fn {
        (fn $name:ident) => {{
            extern "C" {
                fn $name(
                    dst: *mut DynPixel,
                    dst_stride: ptrdiff_t,
                    src: *const DynPixel,
                    src_stride: ptrdiff_t,
                    t1: *const i16,
                    t2: *const i16,
                    w: c_int,
                    h: c_int,
                    wt: *const i16,
                    bitdepth_max: c_int,
                );
            }
            $name
        }};
    }
    bd_fn!(asm_fn, BD, sgr_weighted2, neon)(
        dst.data.as_mut_ptr_at::<BD>(dst.offset).cast(),
        dst.data.stride(),
        dst.data.as_ptr_at::<BD>(dst.offset).cast(),
        dst.data.stride(),
        tmp5.0.as_mut_ptr(),
        tmp3.0.as_mut_ptr(),
        w,
        h,
        wt.as_ptr(),
        bd.into_c(),
    );

    *dst += h as isize * dst.data.pixel_stride::<BD>();
    rotate_ab_2(A5_ptrs, B5_ptrs);
    rotate_ab_4(A3_ptrs, B3_ptrs);
}

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
unsafe fn sgr_filter_3x3_neon<BD: BitDepth>(
    mut dst: Rav1dPictureDataComponentOffset,
    mut left: &[LeftPixelRow<BD::Pixel>],
    mut lpf: *const BD::Pixel,
    w: usize,
    h: usize,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bd: BD,
) {
    let w = w as c_int;
    let mut h = h as c_int;

    let stride = dst.data.pixel_stride::<BD>();

    const BUF_STRIDE: usize = 384 + 16;

    let mut sumsq_buf: Align16<[i32; BUF_STRIDE * 3 + 16]> = Align16([0; BUF_STRIDE * 3 + 16]);
    let mut sum_buf: Align16<[i16; BUF_STRIDE * 3 + 16]> = Align16([0; BUF_STRIDE * 3 + 16]);

    let mut sumsq_ptrs: [*mut i32; 3];
    let mut sum_ptrs: [*mut i16; 3];
    let mut sumsq_rows: [*mut i32; 3] = [0 as *mut i32; 3];
    let mut sum_rows: [*mut i16; 3] = [0 as *mut i16; 3];
    for i in 0..3 {
        sumsq_rows[i] = (sumsq_buf.0[i * BUF_STRIDE..i * BUF_STRIDE + BUF_STRIDE]).as_mut_ptr();
        sum_rows[i] = (sum_buf.0[i * BUF_STRIDE..i * BUF_STRIDE + BUF_STRIDE]).as_mut_ptr();
    }

    let mut A_buf: Align16<[i32; BUF_STRIDE * 3 + 16]> = Align16([0; BUF_STRIDE * 3 + 16]);
    let mut B_buf: Align16<[i16; BUF_STRIDE * 3 + 16]> = Align16([0; BUF_STRIDE * 3 + 16]);

    let mut A_ptrs: [*mut i32; 3] = [0 as *mut i32; 3];
    let mut B_ptrs: [*mut i16; 3] = [0 as *mut i16; 3];
    for i in 0..3 {
        A_ptrs[i] = (A_buf.0[i * BUF_STRIDE..i * BUF_STRIDE + BUF_STRIDE]).as_mut_ptr();
        B_ptrs[i] = (B_buf.0[i * BUF_STRIDE..i * BUF_STRIDE + BUF_STRIDE]).as_mut_ptr();
    }

    let mut src = dst;
    let mut lpf_bottom: *const BD::Pixel = lpf.offset(6 * stride);

    #[derive(PartialEq)]
    enum Track {
        main,
        vert1,
        vert2,
    }
    let mut track = Track::main;

    let sgr = params.sgr();

    if (edges & LR_HAVE_TOP) != 0 {
        sumsq_ptrs = sumsq_rows;
        sum_ptrs = sum_rows;

        rav1d_sgr_box3_row_h_neon(sumsq_rows[0], sum_rows[0], None, lpf, w, edges, bd);
        lpf = lpf.offset(stride);
        rav1d_sgr_box3_row_h_neon(sumsq_rows[1], sum_rows[1], None, lpf, w, edges, bd);

        sgr_box3_hv_neon(
            &mut sumsq_ptrs,
            &mut sum_ptrs,
            A_ptrs[2],
            B_ptrs[2],
            Some(left),
            src.data.as_ptr_at::<BD>(src.offset),
            w,
            sgr.s1 as c_int,
            edges,
            bd,
        );

        left = &left[1..];
        src += stride;
        rotate_ab_3(&mut A_ptrs, &mut B_ptrs);

        h -= 1;
        if h <= 0 {
            track = Track::vert1;
        } else {
            sgr_box3_hv_neon(
                &mut sumsq_ptrs,
                &mut sum_ptrs,
                A_ptrs[2],
                B_ptrs[2],
                Some(left),
                src.data.as_ptr_at::<BD>(src.offset),
                w,
                sgr.s1 as c_int,
                edges,
                bd,
            );
            left = &left[1..];
            src += stride;
            rotate_ab_3(&mut A_ptrs, &mut B_ptrs);

            h -= 1;
            if h <= 0 {
                track = Track::vert2;
            }
        }
    } else {
        sumsq_ptrs = [sumsq_rows[0]; 3];
        sum_ptrs = [sum_rows[0]; 3];

        rav1d_sgr_box3_row_h_neon(
            sumsq_rows[0],
            sum_rows[0],
            Some(left),
            src.data.as_ptr_at::<BD>(src.offset),
            w,
            edges,
            bd,
        );
        left = &left[1..];
        src += stride;

        sgr_box3_vert_neon(
            &mut sumsq_ptrs,
            &mut sum_ptrs,
            A_ptrs[2],
            B_ptrs[2],
            w,
            sgr.s1 as c_int,
            bd,
        );
        rotate_ab_3(&mut A_ptrs, &mut B_ptrs);

        h -= 1;
        if h <= 0 {
            track = Track::vert1;
        } else {
            sumsq_ptrs[2] = sumsq_rows[1];
            sum_ptrs[2] = sum_rows[1];

            sgr_box3_hv_neon(
                &mut sumsq_ptrs,
                &mut sum_ptrs,
                A_ptrs[2],
                B_ptrs[2],
                Some(left),
                src.data.as_ptr_at::<BD>(src.offset),
                w,
                sgr.s1 as c_int,
                edges,
                bd,
            );
            left = &left[1..];
            src += stride;
            rotate_ab_3(&mut A_ptrs, &mut B_ptrs);

            h -= 1;
            if h <= 0 {
                track = Track::vert2;
            } else {
                sumsq_ptrs[2] = sumsq_rows[2];
                sum_ptrs[2] = sum_rows[2];
            }
        }
    }

    // h > 0 can be true only if track == Track::main
    // The original C code uses goto statements and skips over this loop when h <= 0
    while h > 0 {
        sgr_box3_hv_neon(
            &mut sumsq_ptrs,
            &mut sum_ptrs,
            A_ptrs[2],
            B_ptrs[2],
            Some(left),
            src.data.as_ptr_at::<BD>(src.offset),
            w,
            sgr.s1 as c_int,
            edges,
            bd,
        );
        left = &left[1..];
        src += stride;

        sgr_finish1_neon(&mut dst, &mut A_ptrs, &mut B_ptrs, w, sgr.w1 as c_int, bd);
        h -= 1;
    }

    if track == Track::main && (edges & LR_HAVE_BOTTOM) == 0 {
        track = Track::vert2;
    }

    match track {
        Track::main => {
            sgr_box3_hv_neon(
                &mut sumsq_ptrs,
                &mut sum_ptrs,
                A_ptrs[2],
                B_ptrs[2],
                None,
                lpf_bottom,
                w,
                sgr.s1 as c_int,
                edges,
                bd,
            );
            lpf_bottom = lpf_bottom.offset(stride);

            sgr_finish1_neon(&mut dst, &mut A_ptrs, &mut B_ptrs, w, sgr.w1 as c_int, bd);

            sgr_box3_hv_neon(
                &mut sumsq_ptrs,
                &mut sum_ptrs,
                A_ptrs[2],
                B_ptrs[2],
                None,
                lpf_bottom,
                w,
                sgr.s1 as c_int,
                edges,
                bd,
            );

            sgr_finish1_neon(&mut dst, &mut A_ptrs, &mut B_ptrs, w, sgr.w1 as c_int, bd);
        }
        Track::vert1 => {
            sumsq_ptrs[2] = sumsq_ptrs[1];
            sum_ptrs[2] = sum_ptrs[1];
            sgr_box3_vert_neon(
                &mut sumsq_ptrs,
                &mut sum_ptrs,
                A_ptrs[2],
                B_ptrs[2],
                w,
                sgr.s1 as c_int,
                bd,
            );
            rotate_ab_3(&mut A_ptrs, &mut B_ptrs);
        }
        Track::vert2 => {
            sumsq_ptrs[2] = sumsq_ptrs[1];
            sum_ptrs[2] = sum_ptrs[1];
            sgr_box3_vert_neon(
                &mut sumsq_ptrs,
                &mut sum_ptrs,
                A_ptrs[2],
                B_ptrs[2],
                w,
                sgr.s1 as c_int,
                bd,
            );

            sgr_finish1_neon(&mut dst, &mut A_ptrs, &mut B_ptrs, w, sgr.w1 as c_int, bd);
        }
    }

    if track != Track::main {
        sumsq_ptrs[2] = sumsq_ptrs[1];
        sum_ptrs[2] = sum_ptrs[1];
        sgr_box3_vert_neon(
            &mut sumsq_ptrs,
            &mut sum_ptrs,
            A_ptrs[2],
            B_ptrs[2],
            w,
            sgr.s1 as c_int,
            bd,
        );

        sgr_finish1_neon(&mut dst, &mut A_ptrs, &mut B_ptrs, w, sgr.w1 as c_int, bd);
    }
}

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
unsafe fn sgr_filter_5x5_neon<BD: BitDepth>(
    mut dst: Rav1dPictureDataComponentOffset,
    mut left: &[LeftPixelRow<BD::Pixel>],
    mut lpf: *const BD::Pixel,
    w: usize,
    h: usize,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bd: BD,
) {
    let w = w as c_int;
    let mut h = h as c_int;

    let stride = dst.data.pixel_stride::<BD>();

    const BUF_STRIDE: usize = 384 + 16;

    let mut sumsq_buf: Align16<[i32; BUF_STRIDE * 5 + 16]> = Align16([0; BUF_STRIDE * 5 + 16]);
    let mut sum_buf: Align16<[i16; BUF_STRIDE * 5 + 16]> = Align16([0; BUF_STRIDE * 5 + 16]);

    let mut sumsq_ptrs: [*mut i32; 5] = [0 as *mut i32; 5];
    let mut sum_ptrs: [*mut i16; 5] = [0 as *mut i16; 5];
    let mut sumsq_rows: [*mut i32; 5] = [0 as *mut i32; 5];
    let mut sum_rows: [*mut i16; 5] = [0 as *mut i16; 5];
    for i in 0..5 {
        sumsq_rows[i] = (sumsq_buf.0[i * BUF_STRIDE..i * BUF_STRIDE + BUF_STRIDE]).as_mut_ptr();
        sum_rows[i] = (sum_buf.0[i * BUF_STRIDE..i * BUF_STRIDE + BUF_STRIDE]).as_mut_ptr();
    }

    let mut A_buf: Align16<[i32; BUF_STRIDE * 2 + 16]> = Align16([0; BUF_STRIDE * 2 + 16]);
    let mut B_buf: Align16<[i16; BUF_STRIDE * 2 + 16]> = Align16([0; BUF_STRIDE * 2 + 16]);

    let mut A_ptrs: [*mut i32; 2] = [0 as *mut i32; 2];
    let mut B_ptrs: [*mut i16; 2] = [0 as *mut i16; 2];
    for i in 0..2 {
        A_ptrs[i] = (A_buf.0[i * BUF_STRIDE..i * BUF_STRIDE + BUF_STRIDE]).as_mut_ptr();
        B_ptrs[i] = (B_buf.0[i * BUF_STRIDE..i * BUF_STRIDE + BUF_STRIDE]).as_mut_ptr();
    }

    let mut src = dst;
    let mut lpf_bottom: *const BD::Pixel = lpf.offset(6 * stride);

    #[derive(PartialEq)]
    enum Track {
        main,
        vert1,
        vert2,
        odd,
    }
    let mut track = Track::main;

    let sgr = params.sgr();

    if (edges & LR_HAVE_TOP) != 0 {
        for i in 0..5 {
            sumsq_ptrs[i] = sumsq_rows[if i > 0 { i - 1 } else { 0 }];
            sum_ptrs[i] = sum_rows[if i > 0 { i - 1 } else { 0 }];
        }

        rav1d_sgr_box5_row_h_neon(sumsq_rows[0], sum_rows[0], None, lpf, w, edges, bd);
        lpf = lpf.offset(stride);
        rav1d_sgr_box5_row_h_neon(sumsq_rows[1], sum_rows[1], None, lpf, w, edges, bd);

        rav1d_sgr_box5_row_h_neon(
            sumsq_rows[2],
            sum_rows[2],
            Some(left),
            src.data.as_ptr_at::<BD>(src.offset),
            w,
            edges,
            bd,
        );

        left = &left[1..];
        src += stride;

        h -= 1;
        if h <= 0 {
            track = Track::vert1;
        } else {
            rav1d_sgr_box5_row_h_neon(
                sumsq_rows[3],
                sum_rows[3],
                Some(left),
                src.data.as_ptr_at::<BD>(src.offset),
                w,
                edges,
                bd,
            );
            left = &left[1..];
            src += stride;
            sgr_box5_vert_neon(
                &mut sumsq_ptrs,
                &mut sum_ptrs,
                A_ptrs[1],
                B_ptrs[1],
                w,
                sgr.s0 as c_int,
                bd,
            );
            rotate_ab_2(&mut A_ptrs, &mut B_ptrs);

            h -= 1;
            if h <= 0 {
                track = Track::vert2;
            } else {
                // ptrs are rotated by 2; both [3] and [4] now point at rows[0]; set
                // one of them to point at the previously unused rows[4].
                sumsq_ptrs[3] = sumsq_rows[4];
                sum_ptrs[3] = sum_rows[4];
            }
        }
    } else {
        sumsq_ptrs = [sumsq_rows[0]; 5];
        sum_ptrs = [sum_rows[0]; 5];

        rav1d_sgr_box5_row_h_neon(
            sumsq_rows[0],
            sum_rows[0],
            Some(left),
            src.data.as_ptr_at::<BD>(src.offset),
            w,
            edges,
            bd,
        );
        left = &left[1..];
        src += stride;

        h -= 1;
        if h <= 0 {
            track = Track::vert1;
        } else {
            sumsq_ptrs[4] = sumsq_rows[1];
            sum_ptrs[4] = sum_rows[1];

            rav1d_sgr_box5_row_h_neon(
                sumsq_rows[1],
                sum_rows[1],
                Some(left),
                src.data.as_ptr_at::<BD>(src.offset),
                w,
                edges,
                bd,
            );
            left = &left[1..];
            src += stride;

            sgr_box5_vert_neon(
                &mut sumsq_ptrs,
                &mut sum_ptrs,
                A_ptrs[1],
                B_ptrs[1],
                w,
                sgr.s0 as c_int,
                bd,
            );
            rotate_ab_2(&mut A_ptrs, &mut B_ptrs);

            h -= 1;
            if h <= 0 {
                track = Track::vert2;
            } else {
                sumsq_ptrs[3] = sumsq_rows[2];
                sumsq_ptrs[4] = sumsq_rows[3];
                sum_ptrs[3] = sum_rows[2];
                sum_ptrs[4] = sum_rows[3];

                rav1d_sgr_box5_row_h_neon(
                    sumsq_rows[2],
                    sum_rows[2],
                    Some(left),
                    src.data.as_ptr_at::<BD>(src.offset),
                    w,
                    edges,
                    bd,
                );
                left = &left[1..];
                src += stride;

                h -= 1;
                if h <= 0 {
                    track = Track::odd;
                } else {
                    rav1d_sgr_box5_row_h_neon(
                        sumsq_rows[3],
                        sum_rows[3],
                        Some(left),
                        src.data.as_ptr_at::<BD>(src.offset),
                        w,
                        edges,
                        bd,
                    );
                    left = &left[1..];
                    src += stride;

                    sgr_box5_vert_neon(
                        &mut sumsq_ptrs,
                        &mut sum_ptrs,
                        A_ptrs[1],
                        B_ptrs[1],
                        w,
                        sgr.s0 as c_int,
                        bd,
                    );

                    sgr_finish2_neon(
                        &mut dst,
                        &mut A_ptrs,
                        &mut B_ptrs,
                        w,
                        2,
                        sgr.w0 as c_int,
                        bd,
                    );

                    h -= 1;
                    if h <= 0 {
                        track = Track::vert2;
                    } else {
                        // ptrs are rotated by 2; both [3] and [4] now point at rows[0]; set
                        // one of them to point at the previously unused rows[4].
                        sumsq_ptrs[3] = sumsq_rows[4];
                        sum_ptrs[3] = sum_rows[4];
                    }
                }
            }
        }
    }

    // h > 0 can be true only if track == Track::main
    // The original C code uses goto statements and skips over this loop when h <= 0
    while h > 0 {
        rav1d_sgr_box5_row_h_neon(
            sumsq_ptrs[3],
            sum_ptrs[3],
            Some(left),
            src.data.as_ptr_at::<BD>(src.offset),
            w,
            edges,
            bd,
        );
        left = &left[1..];
        src += stride;

        h -= 1;
        if h <= 0 {
            track = Track::odd;
        } else {
            rav1d_sgr_box5_row_h_neon(
                sumsq_ptrs[4],
                sum_ptrs[4],
                Some(left),
                src.data.as_ptr_at::<BD>(src.offset),
                w,
                edges,
                bd,
            );
            left = &left[1..];
            src += stride;

            sgr_box5_vert_neon(
                &mut sumsq_ptrs,
                &mut sum_ptrs,
                A_ptrs[1],
                B_ptrs[1],
                w,
                sgr.s0 as c_int,
                bd,
            );
            sgr_finish2_neon(
                &mut dst,
                &mut A_ptrs,
                &mut B_ptrs,
                w,
                2,
                sgr.w0 as c_int,
                bd,
            );
            h -= 1;
        }
    }

    if track == Track::main && (edges & LR_HAVE_BOTTOM) == 0 {
        track = Track::vert2;
    }

    match track {
        Track::main => {
            rav1d_sgr_box5_row_h_neon(sumsq_ptrs[3], sum_ptrs[3], None, lpf_bottom, w, edges, bd);
            lpf_bottom = lpf_bottom.offset(stride);
            rav1d_sgr_box5_row_h_neon(sumsq_ptrs[4], sum_ptrs[4], None, lpf_bottom, w, edges, bd);
        }
        Track::vert1 => {
            // Copy the last row as padding once
            sumsq_ptrs[4] = sumsq_ptrs[3];
            sum_ptrs[4] = sum_ptrs[3];
            sgr_box5_vert_neon(
                &mut sumsq_ptrs,
                &mut sum_ptrs,
                A_ptrs[1],
                B_ptrs[1],
                w,
                sgr.s0 as c_int,
                bd,
            );
            rotate_ab_2(&mut A_ptrs, &mut B_ptrs);
        }
        Track::vert2 => {
            // Duplicate the last row twice more
            sumsq_ptrs[3] = sumsq_ptrs[2];
            sumsq_ptrs[4] = sumsq_ptrs[2];
            sum_ptrs[3] = sum_ptrs[2];
            sum_ptrs[4] = sum_ptrs[2];
        }
        Track::odd => {
            // Copy the last row as padding once
            sumsq_ptrs[4] = sumsq_ptrs[3];
            sum_ptrs[4] = sum_ptrs[3];

            sgr_box5_vert_neon(
                &mut sumsq_ptrs,
                &mut sum_ptrs,
                A_ptrs[1],
                B_ptrs[1],
                w,
                sgr.s0 as c_int,
                bd,
            );
            sgr_finish2_neon(
                &mut dst,
                &mut A_ptrs,
                &mut B_ptrs,
                w,
                2,
                sgr.w0 as c_int,
                bd,
            );
        }
    }

    match track {
        Track::main | Track::vert2 => {
            sgr_box5_vert_neon(
                &mut sumsq_ptrs,
                &mut sum_ptrs,
                A_ptrs[1],
                B_ptrs[1],
                w,
                sgr.s0 as c_int,
                bd,
            );
            sgr_finish2_neon(
                &mut dst,
                &mut A_ptrs,
                &mut B_ptrs,
                w,
                2,
                sgr.w0 as c_int,
                bd,
            );
        }
        Track::odd | Track::vert1 => {
            // Duplicate the last row twice more
            sumsq_ptrs[3] = sumsq_ptrs[2];
            sumsq_ptrs[4] = sumsq_ptrs[2];
            sum_ptrs[3] = sum_ptrs[2];
            sum_ptrs[4] = sum_ptrs[2];

            sgr_box5_vert_neon(
                &mut sumsq_ptrs,
                &mut sum_ptrs,
                A_ptrs[1],
                B_ptrs[1],
                w,
                sgr.s0 as c_int,
                bd,
            );
            sgr_finish2_neon(
                &mut dst,
                &mut A_ptrs,
                &mut B_ptrs,
                w,
                1,
                sgr.w0 as c_int,
                bd,
            );
        }
    }
}

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
unsafe fn sgr_filter_mix_neon<BD: BitDepth>(
    mut dst: Rav1dPictureDataComponentOffset,
    mut left: &[LeftPixelRow<BD::Pixel>],
    mut lpf: *const BD::Pixel,
    w: usize,
    h: usize,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bd: BD,
) {
    let w = w as c_int;
    let mut h = h as c_int;

    let stride = dst.data.pixel_stride::<BD>();

    const BUF_STRIDE: usize = 384 + 16;

    let mut sumsq5_buf: Align16<[i32; BUF_STRIDE * 5 + 16]> = Align16([0; BUF_STRIDE * 5 + 16]);
    let mut sum5_buf: Align16<[i16; BUF_STRIDE * 5 + 16]> = Align16([0; BUF_STRIDE * 5 + 16]);

    let mut sumsq5_rows: [*mut i32; 5] = [0 as *mut i32; 5];
    let mut sum5_rows: [*mut i16; 5] = [0 as *mut i16; 5];
    for i in 0..5 {
        sumsq5_rows[i] = (sumsq5_buf.0[i * BUF_STRIDE..i * BUF_STRIDE + BUF_STRIDE]).as_mut_ptr();
        sum5_rows[i] = (sum5_buf.0[i * BUF_STRIDE..i * BUF_STRIDE + BUF_STRIDE]).as_mut_ptr();
    }

    let mut sumsq3_buf: Align16<[i32; BUF_STRIDE * 3 + 16]> = Align16([0; BUF_STRIDE * 3 + 16]);
    let mut sum3_buf: Align16<[i16; BUF_STRIDE * 3 + 16]> = Align16([0; BUF_STRIDE * 3 + 16]);

    let mut sumsq3_rows: [*mut i32; 3] = [0 as *mut i32; 3];
    let mut sum3_rows: [*mut i16; 3] = [0 as *mut i16; 3];
    for i in 0..3 {
        sumsq3_rows[i] = (sumsq3_buf.0[i * BUF_STRIDE..i * BUF_STRIDE + BUF_STRIDE]).as_mut_ptr();
        sum3_rows[i] = (sum3_buf.0[i * BUF_STRIDE..i * BUF_STRIDE + BUF_STRIDE]).as_mut_ptr();
    }

    let mut A5_buf: Align16<[i32; BUF_STRIDE * 2 + 16]> = Align16([0; BUF_STRIDE * 2 + 16]);
    let mut B5_buf: Align16<[i16; BUF_STRIDE * 2 + 16]> = Align16([0; BUF_STRIDE * 2 + 16]);

    let mut A5_ptrs: [*mut i32; 2] = [0 as *mut i32; 2];
    let mut B5_ptrs: [*mut i16; 2] = [0 as *mut i16; 2];
    for i in 0..2 {
        A5_ptrs[i] = (A5_buf.0[i * BUF_STRIDE..i * BUF_STRIDE + BUF_STRIDE]).as_mut_ptr();
        B5_ptrs[i] = (B5_buf.0[i * BUF_STRIDE..i * BUF_STRIDE + BUF_STRIDE]).as_mut_ptr();
    }

    let mut A3_buf: Align16<[i32; BUF_STRIDE * 4 + 16]> = Align16([0; BUF_STRIDE * 4 + 16]);
    let mut B3_buf: Align16<[i16; BUF_STRIDE * 4 + 16]> = Align16([0; BUF_STRIDE * 4 + 16]);

    let mut A3_ptrs: [*mut i32; 4] = [0 as *mut i32; 4];
    let mut B3_ptrs: [*mut i16; 4] = [0 as *mut i16; 4];
    for i in 0..4 {
        A3_ptrs[i] = (A3_buf.0[i * BUF_STRIDE..i * BUF_STRIDE + BUF_STRIDE]).as_mut_ptr();
        B3_ptrs[i] = (B3_buf.0[i * BUF_STRIDE..i * BUF_STRIDE + BUF_STRIDE]).as_mut_ptr();
    }

    let mut src = dst;
    let mut lpf_bottom: *const BD::Pixel = lpf.offset(6 * stride);

    #[derive(PartialEq)]
    enum Track {
        main,
        vert1,
        vert2,
        odd,
    }
    let mut track = Track::main;

    let lr_have_top = (edges & LR_HAVE_TOP) != 0;

    let mut sumsq3_ptrs: [*mut i32; 3] = [0 as *mut i32; 3];
    let mut sum3_ptrs: [*mut i16; 3] = [0 as *mut i16; 3];
    for i in 0..3 {
        sumsq3_ptrs[i] = sumsq3_rows[if lr_have_top { i } else { 0 }];
        sum3_ptrs[i] = sum3_rows[if lr_have_top { i } else { 0 }];
    }

    let mut sumsq5_ptrs: [*mut i32; 5] = [0 as *mut i32; 5];
    let mut sum5_ptrs: [*mut i16; 5] = [0 as *mut i16; 5];
    for i in 0..5 {
        sumsq5_ptrs[i] = sumsq5_rows[if lr_have_top && i > 0 { i - 1 } else { 0 }];
        sum5_ptrs[i] = sum5_rows[if lr_have_top && i > 0 { i - 1 } else { 0 }];
    }

    let sgr = params.sgr();

    if lr_have_top {
        rav1d_sgr_box35_row_h_neon(
            sumsq3_rows[0],
            sum3_rows[0],
            sumsq5_rows[0],
            sum5_rows[0],
            None,
            lpf,
            w,
            edges,
            bd,
        );
        lpf = lpf.offset(stride);
        rav1d_sgr_box35_row_h_neon(
            sumsq3_rows[1],
            sum3_rows[1],
            sumsq5_rows[1],
            sum5_rows[1],
            None,
            lpf,
            w,
            edges,
            bd,
        );

        rav1d_sgr_box35_row_h_neon(
            sumsq3_rows[2],
            sum3_rows[2],
            sumsq5_rows[2],
            sum5_rows[2],
            Some(left),
            src.data.as_ptr_at::<BD>(src.offset),
            w,
            edges,
            bd,
        );

        left = &left[1..];
        src += stride;

        sgr_box3_vert_neon(
            &mut sumsq3_ptrs,
            &mut sum3_ptrs,
            A3_ptrs[3],
            B3_ptrs[3],
            w,
            sgr.s1 as c_int,
            bd,
        );
        rotate_ab_4(&mut A3_ptrs, &mut B3_ptrs);

        h -= 1;
        if h <= 0 {
            track = Track::vert1;
        } else {
            rav1d_sgr_box35_row_h_neon(
                sumsq3_ptrs[2],
                sum3_ptrs[2],
                sumsq5_rows[3],
                sum5_rows[3],
                Some(left),
                src.data.as_ptr_at::<BD>(src.offset),
                w,
                edges,
                bd,
            );
            left = &left[1..];
            src += stride;

            sgr_box5_vert_neon(
                &mut sumsq5_ptrs,
                &mut sum5_ptrs,
                A5_ptrs[1],
                B5_ptrs[1],
                w,
                sgr.s0 as c_int,
                bd,
            );
            rotate_ab_2(&mut A5_ptrs, &mut B5_ptrs);

            sgr_box3_vert_neon(
                &mut sumsq3_ptrs,
                &mut sum3_ptrs,
                A3_ptrs[3],
                B3_ptrs[3],
                w,
                sgr.s1 as c_int,
                bd,
            );
            rotate_ab_4(&mut A3_ptrs, &mut B3_ptrs);

            h -= 1;
            if h <= 0 {
                track = Track::vert2;
            } else {
                // ptrs are rotated by 2; both [3] and [4] now point at rows[0]; set
                // one of them to point at the previously unused rows[4].
                sumsq5_ptrs[3] = sumsq5_rows[4];
                sum5_ptrs[3] = sum5_rows[4];
            }
        }
    } else {
        rav1d_sgr_box35_row_h_neon(
            sumsq3_rows[0],
            sum3_rows[0],
            sumsq5_rows[0],
            sum5_rows[0],
            Some(left),
            src.data.as_ptr_at::<BD>(src.offset),
            w,
            edges,
            bd,
        );
        left = &left[1..];
        src += stride;

        sgr_box3_vert_neon(
            &mut sumsq3_ptrs,
            &mut sum3_ptrs,
            A3_ptrs[3],
            B3_ptrs[3],
            w,
            sgr.s1 as i32,
            bd,
        );
        rotate_ab_4(&mut A3_ptrs, &mut B3_ptrs);

        h -= 1;
        if h <= 0 {
            track = Track::vert1;
        } else {
            sumsq5_ptrs[4] = sumsq5_rows[1];
            sum5_ptrs[4] = sum5_rows[1];

            sumsq3_ptrs[2] = sumsq3_rows[1];
            sum3_ptrs[2] = sum3_rows[1];

            rav1d_sgr_box35_row_h_neon(
                sumsq3_rows[1],
                sum3_rows[1],
                sumsq5_rows[1],
                sum5_rows[1],
                Some(left),
                src.data.as_ptr_at::<BD>(src.offset),
                w,
                edges,
                bd,
            );
            left = &left[1..];
            src += stride;

            sgr_box5_vert_neon(
                &mut sumsq5_ptrs,
                &mut sum5_ptrs,
                A5_ptrs[1],
                B5_ptrs[1],
                w,
                sgr.s0 as c_int,
                bd,
            );
            rotate_ab_2(&mut A5_ptrs, &mut B5_ptrs);

            sgr_box3_vert_neon(
                &mut sumsq3_ptrs,
                &mut sum3_ptrs,
                A3_ptrs[3],
                B3_ptrs[3],
                w,
                sgr.s1 as c_int,
                bd,
            );
            rotate_ab_4(&mut A3_ptrs, &mut B3_ptrs);

            h -= 1;
            if h <= 0 {
                track = Track::vert2;
            } else {
                sumsq5_ptrs[3] = sumsq5_rows[2];
                sumsq5_ptrs[4] = sumsq5_rows[3];
                sum5_ptrs[3] = sum5_rows[2];
                sum5_ptrs[4] = sum5_rows[3];

                sumsq3_ptrs[2] = sumsq3_rows[2];
                sum3_ptrs[2] = sum3_rows[2];

                rav1d_sgr_box35_row_h_neon(
                    sumsq3_rows[2],
                    sum3_rows[2],
                    sumsq5_rows[2],
                    sum5_rows[2],
                    Some(left),
                    src.data.as_ptr_at::<BD>(src.offset),
                    w,
                    edges,
                    bd,
                );
                left = &left[1..];
                src += stride;

                sgr_box3_vert_neon(
                    &mut sumsq3_ptrs,
                    &mut sum3_ptrs,
                    A3_ptrs[3],
                    B3_ptrs[3],
                    w,
                    sgr.s1 as c_int,
                    bd,
                );
                rotate_ab_4(&mut A3_ptrs, &mut B3_ptrs);

                h -= 1;
                if h <= 0 {
                    track = Track::odd;
                } else {
                    rav1d_sgr_box35_row_h_neon(
                        sumsq3_ptrs[2],
                        sum3_ptrs[2],
                        sumsq5_rows[3],
                        sum5_rows[3],
                        Some(left),
                        src.data.as_ptr_at::<BD>(src.offset),
                        w,
                        edges,
                        bd,
                    );
                    left = &left[1..];
                    src += stride;

                    sgr_box5_vert_neon(
                        &mut sumsq5_ptrs,
                        &mut sum5_ptrs,
                        A5_ptrs[1],
                        B5_ptrs[1],
                        w,
                        sgr.s0 as c_int,
                        bd,
                    );
                    sgr_box3_vert_neon(
                        &mut sumsq3_ptrs,
                        &mut sum3_ptrs,
                        A3_ptrs[3],
                        B3_ptrs[3],
                        w,
                        sgr.s1 as c_int,
                        bd,
                    );
                    sgr_finish_mix_neon(
                        &mut dst,
                        &mut A5_ptrs,
                        &mut B5_ptrs,
                        &mut A3_ptrs,
                        &mut B3_ptrs,
                        w,
                        2,
                        sgr.w0 as c_int,
                        sgr.w1 as c_int,
                        bd,
                    );

                    h -= 1;
                    if h <= 0 {
                        track = Track::vert2;
                    } else {
                        // ptrs are rotated by 2; both [3] and [4] now point at rows[0]; set
                        // one of them to point at the previously unused rows[4].
                        sumsq5_ptrs[3] = sumsq5_rows[4];
                        sum5_ptrs[3] = sum5_rows[4];
                    }
                }
            }
        }
    }

    // h > 0 can be true only if track == Track::main
    // The original C code uses goto statements and skips over this loop when h <= 0
    while h > 0 {
        rav1d_sgr_box35_row_h_neon(
            sumsq3_ptrs[2],
            sum3_ptrs[2],
            sumsq5_ptrs[3],
            sum5_ptrs[3],
            Some(left),
            src.data.as_ptr_at::<BD>(src.offset),
            w,
            edges,
            bd,
        );
        left = &left[1..];
        src += stride;

        sgr_box3_vert_neon(
            &mut sumsq3_ptrs,
            &mut sum3_ptrs,
            A3_ptrs[3],
            B3_ptrs[3],
            w,
            sgr.s1 as c_int,
            bd,
        );
        rotate_ab_4(&mut A3_ptrs, &mut B3_ptrs);

        h -= 1;
        if h <= 0 {
            track = Track::odd;
        } else {
            rav1d_sgr_box35_row_h_neon(
                sumsq3_ptrs[2],
                sum3_ptrs[2],
                sumsq5_ptrs[4],
                sum5_ptrs[4],
                Some(left),
                src.data.as_ptr_at::<BD>(src.offset),
                w,
                edges,
                bd,
            );
            left = &left[1..];
            src += stride;

            sgr_box5_vert_neon(
                &mut sumsq5_ptrs,
                &mut sum5_ptrs,
                A5_ptrs[1],
                B5_ptrs[1],
                w,
                sgr.s0 as c_int,
                bd,
            );
            sgr_box3_vert_neon(
                &mut sumsq3_ptrs,
                &mut sum3_ptrs,
                A3_ptrs[3],
                B3_ptrs[3],
                w,
                sgr.s1 as c_int,
                bd,
            );
            sgr_finish_mix_neon(
                &mut dst,
                &mut A5_ptrs,
                &mut B5_ptrs,
                &mut A3_ptrs,
                &mut B3_ptrs,
                w,
                2,
                sgr.w0 as c_int,
                sgr.w1 as c_int,
                bd,
            );
            h -= 1;
        }
    }

    if track == Track::main && (edges & LR_HAVE_BOTTOM) == 0 {
        track = Track::vert2;
    }

    match track {
        Track::main => {
            rav1d_sgr_box35_row_h_neon(
                sumsq3_ptrs[2],
                sum3_ptrs[2],
                sumsq5_ptrs[3],
                sum5_ptrs[3],
                None,
                lpf_bottom,
                w,
                edges,
                bd,
            );
            lpf_bottom = lpf_bottom.offset(stride);

            sgr_box3_vert_neon(
                &mut sumsq3_ptrs,
                &mut sum3_ptrs,
                A3_ptrs[3],
                B3_ptrs[3],
                w,
                sgr.s1 as c_int,
                bd,
            );
            rotate_ab_4(&mut A3_ptrs, &mut B3_ptrs);

            rav1d_sgr_box35_row_h_neon(
                sumsq3_ptrs[2],
                sum3_ptrs[2],
                sumsq5_ptrs[4],
                sum5_ptrs[4],
                None,
                lpf_bottom,
                w,
                edges,
                bd,
            );
        }
        Track::vert1 => {
            // Copy the last row as padding once
            sumsq5_ptrs[4] = sumsq5_ptrs[3];
            sum5_ptrs[4] = sum5_ptrs[3];

            sumsq3_ptrs[2] = sumsq3_ptrs[1];
            sum3_ptrs[2] = sum3_ptrs[1];

            sgr_box5_vert_neon(
                &mut sumsq5_ptrs,
                &mut sum5_ptrs,
                A5_ptrs[1],
                B5_ptrs[1],
                w,
                sgr.s0 as c_int,
                bd,
            );
            rotate_ab_2(&mut A5_ptrs, &mut B5_ptrs);
            sgr_box3_vert_neon(
                &mut sumsq3_ptrs,
                &mut sum3_ptrs,
                A3_ptrs[3],
                B3_ptrs[3],
                w,
                sgr.s1 as c_int,
                bd,
            );
            rotate_ab_4(&mut A3_ptrs, &mut B3_ptrs);
        }
        Track::vert2 => {
            // Duplicate the last row twice more
            sumsq5_ptrs[3] = sumsq5_ptrs[2];
            sumsq5_ptrs[4] = sumsq5_ptrs[2];
            sum5_ptrs[3] = sum5_ptrs[2];
            sum5_ptrs[4] = sum5_ptrs[2];

            sumsq3_ptrs[2] = sumsq3_ptrs[1];
            sum3_ptrs[2] = sum3_ptrs[1];
            sgr_box3_vert_neon(
                &mut sumsq3_ptrs,
                &mut sum3_ptrs,
                A3_ptrs[3],
                B3_ptrs[3],
                w,
                sgr.s1 as c_int,
                bd,
            );
            rotate_ab_4(&mut A3_ptrs, &mut B3_ptrs);

            sumsq3_ptrs[2] = sumsq3_ptrs[1];
            sum3_ptrs[2] = sum3_ptrs[1];
        }
        Track::odd => {
            // Copy the last row as padding once
            sumsq5_ptrs[4] = sumsq5_ptrs[3];
            sum5_ptrs[4] = sum5_ptrs[3];

            sumsq3_ptrs[2] = sumsq3_ptrs[1];
            sum3_ptrs[2] = sum3_ptrs[1];

            sgr_box5_vert_neon(
                &mut sumsq5_ptrs,
                &mut sum5_ptrs,
                A5_ptrs[1],
                B5_ptrs[1],
                w,
                sgr.s0 as c_int,
                bd,
            );
            sgr_box3_vert_neon(
                &mut sumsq3_ptrs,
                &mut sum3_ptrs,
                A3_ptrs[3],
                B3_ptrs[3],
                w,
                sgr.s1 as c_int,
                bd,
            );
            sgr_finish_mix_neon(
                &mut dst,
                &mut A5_ptrs,
                &mut B5_ptrs,
                &mut A3_ptrs,
                &mut B3_ptrs,
                w,
                2,
                sgr.w0 as c_int,
                sgr.w1 as c_int,
                bd,
            );
        }
    }

    match track {
        Track::main | Track::vert2 => {
            sgr_box5_vert_neon(
                &mut sumsq5_ptrs,
                &mut sum5_ptrs,
                A5_ptrs[1],
                B5_ptrs[1],
                w,
                sgr.s0 as c_int,
                bd,
            );
            sgr_box3_vert_neon(
                &mut sumsq3_ptrs,
                &mut sum3_ptrs,
                A3_ptrs[3],
                B3_ptrs[3],
                w,
                sgr.s1 as c_int,
                bd,
            );
            sgr_finish_mix_neon(
                &mut dst,
                &mut A5_ptrs,
                &mut B5_ptrs,
                &mut A3_ptrs,
                &mut B3_ptrs,
                w,
                2,
                sgr.w0 as c_int,
                sgr.w1 as c_int,
                bd,
            );
        }
        Track::vert1 | Track::odd => {
            // Duplicate the last row twice more
            sumsq5_ptrs[3] = sumsq5_ptrs[2];
            sumsq5_ptrs[4] = sumsq5_ptrs[2];
            sum5_ptrs[3] = sum5_ptrs[2];
            sum5_ptrs[4] = sum5_ptrs[2];

            sumsq3_ptrs[2] = sumsq3_ptrs[1];
            sum3_ptrs[2] = sum3_ptrs[1];

            sgr_box5_vert_neon(
                &mut sumsq5_ptrs,
                &mut sum5_ptrs,
                A5_ptrs[1],
                B5_ptrs[1],
                w,
                sgr.s0 as c_int,
                bd,
            );
            sgr_box3_vert_neon(
                &mut sumsq3_ptrs,
                &mut sum3_ptrs,
                A3_ptrs[3],
                B3_ptrs[3],
                w,
                sgr.s1 as c_int,
                bd,
            );
            rotate_ab_4(&mut A3_ptrs, &mut B3_ptrs);
            // Output only one row
            sgr_finish_mix_neon(
                &mut dst,
                &mut A5_ptrs,
                &mut B5_ptrs,
                &mut A3_ptrs,
                &mut B3_ptrs,
                w,
                1,
                sgr.w0 as c_int,
                sgr.w1 as c_int,
                bd,
            );
        }
    }
}

/// # Safety
///
/// Must be called by [`loop_restoration_filter::Fn::call`].
#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn sgr_filter_mix_neon_erased<BD: BitDepth>(
    _p_ptr: *mut DynPixel,
    _stride: ptrdiff_t,
    left: *const LeftPixelRow<DynPixel>,
    lpf: *const DynPixel,
    w: c_int,
    h: c_int,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: c_int,
    p: *const FFISafe<Rav1dPictureDataComponentOffset>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `loop_restoration_filter::Fn::call`.
    let p = *unsafe { FFISafe::get(p) };
    let left = left.cast();
    let lpf = lpf.cast();
    let bd = BD::from_c(bitdepth_max);
    let w = w as usize;
    let h = h as usize;
    // SAFETY: Length sliced in `loop_restoration_filter::Fn::call`.
    let left = unsafe { slice::from_raw_parts(left, h) };
    sgr_filter_mix_neon(p, left, lpf, w, h, params, edges, bd)
}

#[cfg(all(feature = "asm", target_arch = "arm"))]
unsafe fn sgr_filter_mix_neon<BD: BitDepth>(
    dst: Rav1dPictureDataComponentOffset,
    left: &[LeftPixelRow<BD::Pixel>],
    lpf: *const BD::Pixel,
    w: usize,
    h: usize,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bd: BD,
) {
    let w = w as c_int;
    let h = h as c_int;
    let mut tmp1: Align16<[i16; 24576]> = Align16([0; 24576]);
    let mut tmp2: Align16<[i16; 24576]> = Align16([0; 24576]);
    let sgr = params.sgr();
    rav1d_sgr_filter2_neon(&mut tmp1.0, dst, left, lpf, w, h, sgr.s0, edges, bd);
    rav1d_sgr_filter1_neon(&mut tmp2.0, dst, left, lpf, w, h, sgr.s1, edges, bd);
    let wt: [i16; 2] = [sgr.w0, sgr.w1];
    rav1d_sgr_weighted2_neon(dst, dst, &mut tmp1.0, &mut tmp2.0, w, h, &wt, bd);
}

impl Rav1dLoopRestorationDSPContext {
    pub const fn default<BD: BitDepth>() -> Self {
        Self {
            wiener: [loop_restoration_filter::Fn::new(wiener_c_erased::<BD>); 2],
            sgr: [
                loop_restoration_filter::Fn::new(sgr_5x5_c_erased::<BD>),
                loop_restoration_filter::Fn::new(sgr_3x3_c_erased::<BD>),
                loop_restoration_filter::Fn::new(sgr_mix_c_erased::<BD>),
            ],
        }
    }

    #[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
    #[inline(always)]
    const fn init_x86<BD: BitDepth>(mut self, flags: CpuFlags, bpc: u8) -> Self {
        if !flags.contains(CpuFlags::SSE2) {
            return self;
        }

        if let BPC::BPC8 = BD::BPC {
            self.wiener[0] = bpc_fn!(loop_restoration_filter::decl_fn, 8 bpc, wiener_filter7, sse2);
            self.wiener[1] = bpc_fn!(loop_restoration_filter::decl_fn, 8 bpc, wiener_filter5, sse2);
        };

        if !flags.contains(CpuFlags::SSSE3) {
            return self;
        }

        self.wiener[0] = bd_fn!(loop_restoration_filter::decl_fn, BD, wiener_filter7, ssse3);
        self.wiener[1] = bd_fn!(loop_restoration_filter::decl_fn, BD, wiener_filter5, ssse3);

        if matches!(BD::BPC, BPC::BPC8) || bpc == 10 {
            self.sgr[0] = bd_fn!(loop_restoration_filter::decl_fn, BD, sgr_filter_5x5, ssse3);
            self.sgr[1] = bd_fn!(loop_restoration_filter::decl_fn, BD, sgr_filter_3x3, ssse3);
            self.sgr[2] = bd_fn!(loop_restoration_filter::decl_fn, BD, sgr_filter_mix, ssse3);
        }

        #[cfg(target_arch = "x86_64")]
        {
            if !flags.contains(CpuFlags::AVX2) {
                return self;
            }

            self.wiener[0] = bd_fn!(loop_restoration_filter::decl_fn, BD, wiener_filter7, avx2);
            self.wiener[1] = bd_fn!(loop_restoration_filter::decl_fn, BD, wiener_filter5, avx2);

            if matches!(BD::BPC, BPC::BPC8) || bpc == 10 {
                self.sgr[0] = bd_fn!(loop_restoration_filter::decl_fn, BD, sgr_filter_5x5, avx2);
                self.sgr[1] = bd_fn!(loop_restoration_filter::decl_fn, BD, sgr_filter_3x3, avx2);
                self.sgr[2] = bd_fn!(loop_restoration_filter::decl_fn, BD, sgr_filter_mix, avx2);
            }

            if !flags.contains(CpuFlags::AVX512ICL) {
                return self;
            }

            self.wiener[0] = bd_fn!(
                loop_restoration_filter::decl_fn,
                BD,
                wiener_filter7,
                avx512icl
            );
            self.wiener[1] = match BD::BPC {
                // With VNNI we don't need a 5-tap version.
                BPC::BPC8 => self.wiener[0],
                BPC::BPC16 => {
                    bpc_fn!(loop_restoration_filter::decl_fn, 16 bpc, wiener_filter5, avx512icl)
                }
            };

            if matches!(BD::BPC, BPC::BPC8) || bpc == 10 {
                self.sgr[0] = bd_fn!(
                    loop_restoration_filter::decl_fn,
                    BD,
                    sgr_filter_5x5,
                    avx512icl
                );
                self.sgr[1] = bd_fn!(
                    loop_restoration_filter::decl_fn,
                    BD,
                    sgr_filter_3x3,
                    avx512icl
                );
                self.sgr[2] = bd_fn!(
                    loop_restoration_filter::decl_fn,
                    BD,
                    sgr_filter_mix,
                    avx512icl
                );
            }
        }

        self
    }

    #[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
    #[inline(always)]
    const fn init_arm<BD: BitDepth>(mut self, flags: CpuFlags, bpc: u8) -> Self {
        if !flags.contains(CpuFlags::NEON) {
            return self;
        }

        #[cfg(target_arch = "aarch64")]
        {
            self.wiener[0] = bd_fn!(loop_restoration_filter::decl_fn, BD, wiener_filter7, neon);
            self.wiener[1] = bd_fn!(loop_restoration_filter::decl_fn, BD, wiener_filter5, neon);
        }

        #[cfg(target_arch = "arm")]
        {
            self.wiener[0] = loop_restoration_filter::Fn::new(wiener_filter_neon_erased::<BD>);
            self.wiener[1] = loop_restoration_filter::Fn::new(wiener_filter_neon_erased::<BD>);
        }

        if matches!(BD::BPC, BPC::BPC8) || bpc == 10 {
            self.sgr[0] = loop_restoration_filter::Fn::new(sgr_filter_5x5_neon_erased::<BD>);
            self.sgr[1] = loop_restoration_filter::Fn::new(sgr_filter_3x3_neon_erased::<BD>);
            self.sgr[2] = loop_restoration_filter::Fn::new(sgr_filter_mix_neon_erased::<BD>);
        }

        self
    }

    #[inline(always)]
    const fn init<BD: BitDepth>(self, flags: CpuFlags, bpc: u8) -> Self {
        #[cfg(feature = "asm")]
        {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            {
                return self.init_x86::<BD>(flags, bpc);
            }
            #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
            {
                return self.init_arm::<BD>(flags, bpc);
            }
        }

        #[allow(unreachable_code)] // Reachable on some #[cfg]s.
        {
            let _ = flags;
            let _ = bpc;
            self
        }
    }

    pub const fn new<BD: BitDepth>(flags: CpuFlags, bpc: u8) -> Self {
        Self::default::<BD>().init::<BD>(flags, bpc)
    }
}
