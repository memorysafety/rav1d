#![deny(unsafe_op_in_unsafe_fn)]

use crate::align::AlignedVec64;
use crate::cpu::CpuFlags;
use crate::cursor::CursorMut;
use crate::disjoint_mut::DisjointMut;
use crate::ffi_safe::FFISafe;
use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::bitdepth::LeftPixelRow;
use crate::include::common::bitdepth::ToPrimitive;
use crate::include::common::bitdepth::BPC;
use crate::include::common::intops::iclip;
use crate::include::dav1d::picture::Rav1dPictureDataComponentOffset;
use crate::strided::Strided as _;
use crate::tables::dav1d_sgr_x_by_x;
use crate::wrap_fn_ptr::wrap_fn_ptr;
use bitflags::bitflags;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::iter;
use std::mem;
use std::ops::Add;
use std::slice;
use to_method::To;
use zerocopy::AsBytes;
use zerocopy::FromBytes;
use zerocopy::FromZeroes;

#[cfg(all(
    feature = "asm",
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")
))]
use crate::include::common::bitdepth::bd_fn;

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
use crate::include::common::bitdepth::bpc_fn;

bitflags! {
    #[derive(Clone, Copy)]
    #[repr(transparent)]
    pub struct LrEdgeFlags: u8 {
        const LEFT = 1 << 0;
        const RIGHT = 1 << 1;
        const TOP = 1 << 2;
        const BOTTOM = 1 << 3;
    }
}

impl LrEdgeFlags {
    pub const fn select(&self, select: bool) -> Self {
        if select {
            *self
        } else {
            Self::empty()
        }
    }
}

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
    /// [`Align16`] moved to [`Self`] because we can't `#[derive(`[`AsBytes`]`)]` on it due to generics.
    ///
    /// [`Align16`]: crate::align::Align16
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
    lpf_ptr: *const DynPixel,
    w: c_int,
    h: c_int,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: c_int,
    _dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
    _lpf: *const FFISafe<DisjointMut<AlignedVec64<u8>>>,
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
    pub fn call<BD: BitDepth>(
        &self,
        dst: Rav1dPictureDataComponentOffset,
        left: &[LeftPixelRow<BD::Pixel>],
        lpf: &DisjointMut<AlignedVec64<u8>>,
        lpf_off: isize,
        w: c_int,
        h: c_int,
        params: &LooprestorationParams,
        edges: LrEdgeFlags,
        bd: BD,
    ) {
        let dst_ptr = dst.as_mut_ptr::<BD>().cast();
        let dst_stride = dst.stride();
        let left = left[..h as usize].as_ptr().cast();
        // NOTE: The calculated pointer may point to before the beginning of
        // `lpf`, so we must use `.wrapping_offset` here. `.wrapping_offset` is
        // needed since `.offset` requires the pointer to be in bounds, which
        // `.wrapping_offset` does not, and delays that requirement to when the
        // pointer is dereferenced.
        let lpf_ptr = lpf
            .as_mut_ptr()
            .cast::<BD::Pixel>()
            .wrapping_offset(lpf_off)
            .cast();
        let bd = bd.into_c();
        let dst = FFISafe::new(&dst);
        let lpf = FFISafe::new(lpf);
        // SAFETY: Fallbacks `fn wiener_rust`, `fn sgr_{3x3,5x5,mix}_rust` are safe; asm is supposed to do the same.
        unsafe {
            self.get()(
                dst_ptr, dst_stride, left, lpf_ptr, w, h, params, edges, bd, dst, lpf,
            )
        }
    }
}

pub struct Rav1dLoopRestorationDSPContext {
    pub wiener: [loop_restoration_filter::Fn; 2],
    pub sgr: [loop_restoration_filter::Fn; 3],
}

const REST_UNIT_STRIDE: usize = 256 * 3 / 2 + 3 + 3;

// TODO Reuse p when no padding is needed (add and remove lpf pixels in p)
// TODO Chroma only requires 2 rows of padding.
#[inline(never)]
fn padding<BD: BitDepth>(
    dst: &mut [BD::Pixel; (64 + 3 + 3) * REST_UNIT_STRIDE],
    p: Rav1dPictureDataComponentOffset,
    left: &[LeftPixelRow<BD::Pixel>],
    lpf: &DisjointMut<AlignedVec64<u8>>,
    lpf_off: isize,
    unit_w: usize,
    stripe_h: usize,
    edges: LrEdgeFlags,
) {
    let left = &left[..stripe_h];
    assert!(stripe_h > 0);
    let stride = p.pixel_stride::<BD>();

    let [have_left, have_right, have_top, have_bottom] = [
        LrEdgeFlags::LEFT,
        LrEdgeFlags::RIGHT,
        LrEdgeFlags::TOP,
        LrEdgeFlags::BOTTOM,
    ]
    .map(|lr_have| edges.contains(lr_have));
    let [have_left_3, have_right_3] = [have_left, have_right].map(|have| 3 * have as usize);

    // Copy more pixels if we don't have to pad them
    let unit_w = unit_w + have_left_3 + have_right_3;
    let dst_l = &mut dst[3 - have_left_3..];
    let p = p - have_left_3;
    let lpf_off = lpf_off - (have_left_3 as isize);
    let abs_stride = stride.unsigned_abs();

    if have_top {
        // Copy previous loop filtered rows
        let lpf_guard;
        let (above_1, above_2) = if stride < 0 {
            lpf_guard = lpf
                .slice_as::<_, BD::Pixel>(((lpf_off + stride) as usize.., ..abs_stride + unit_w));
            let above_2 = &*lpf_guard;
            let above_1 = &above_2[abs_stride..];
            (above_1, above_2)
        } else {
            lpf_guard = lpf.slice_as((lpf_off as usize.., ..abs_stride + unit_w));
            let above_1 = &*lpf_guard;
            let above_2 = &above_1[abs_stride..];
            (above_1, above_2)
        };
        BD::pixel_copy(dst_l, above_1, unit_w);
        BD::pixel_copy(&mut dst_l[REST_UNIT_STRIDE..], above_1, unit_w);
        BD::pixel_copy(&mut dst_l[2 * REST_UNIT_STRIDE..], above_2, unit_w);
    } else {
        // Pad with first row
        let p = &*p.slice::<BD>(unit_w);
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
        let offset = lpf_off + (6 + if stride < 0 { 1 } else { 0 }) * stride;
        let lpf = &*lpf.slice_as((offset as usize.., ..abs_stride + unit_w));
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
        let src = &*src.slice::<BD>(unit_w);
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
            &p.slice::<BD>(len),
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

/// Calculates the offset between `lpf` and `ptr`.
///
/// This behaves like [`offset_from`], but allows for `ptr` to point to outside
/// the allocation of `lpf`. This is necessary because `ptr` may point to before
/// the beginning of `lpf`, which violates the safety conditions of
/// [`offset_from`].
///
/// [`offset_from`]: https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.offset_from
fn reconstruct_lpf_offset<BD: BitDepth>(
    lpf: &DisjointMut<AlignedVec64<u8>>,
    ptr: *const BD::Pixel,
) -> isize {
    let base = lpf.as_mut_ptr().cast::<BD::Pixel>();
    (ptr as isize - base as isize) / (mem::size_of::<BD::Pixel>() as isize)
}

/// # Safety
///
/// Must be called by [`loop_restoration_filter::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn wiener_c_erased<BD: BitDepth>(
    _p_ptr: *mut DynPixel,
    _stride: ptrdiff_t,
    left: *const LeftPixelRow<DynPixel>,
    lpf_ptr: *const DynPixel,
    w: c_int,
    h: c_int,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: c_int,
    p: *const FFISafe<Rav1dPictureDataComponentOffset>,
    lpf: *const FFISafe<DisjointMut<AlignedVec64<u8>>>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `loop_restoration_filter::Fn::call`.
    let p = *unsafe { FFISafe::get(p) };
    let left = left.cast();
    // SAFETY: Was passed as `FFISafe::new(_)` in `loop_restoration_filter::Fn::call`.
    let lpf = unsafe { FFISafe::get(lpf) };
    let lpf_ptr = lpf_ptr.cast();
    let lpf_off = reconstruct_lpf_offset::<BD>(lpf, lpf_ptr);
    let bd = BD::from_c(bitdepth_max);
    let w = w as usize;
    let h = h as usize;
    // SAFETY: Length sliced in `loop_restoration_filter::Fn::call`.
    let left = unsafe { slice::from_raw_parts(left, h) };
    wiener_rust(p, left, lpf, lpf_off, w, h, params, edges, bd)
}

// FIXME Could split into luma and chroma specific functions,
// (since first and last tops are always 0 for chroma)
// FIXME Could implement a version that requires less temporary memory
// (should be possible to implement with only 6 rows of temp storage)
fn wiener_rust<BD: BitDepth>(
    p: Rav1dPictureDataComponentOffset,
    left: &[LeftPixelRow<BD::Pixel>],
    lpf: &DisjointMut<AlignedVec64<u8>>,
    lpf_off: isize,
    w: usize,
    h: usize,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bd: BD,
) {
    // Wiener filtering is applied to a maximum stripe height of 64 + 3 pixels
    // of padding above and below
    let mut tmp = [0.into(); (64 + 3 + 3) * REST_UNIT_STRIDE];

    padding::<BD>(&mut tmp, p, left, lpf, lpf_off, w, h, edges);

    // Values stored between horizontal and vertical filtering don't
    // fit in a u8.
    let mut hor = [0; (64 + 3 + 3) * REST_UNIT_STRIDE];

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

            for (&tmp, &filter) in iter::zip(&tmp[i..i + 7], &filter[0][..7]) {
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

            let p = p + (j as isize * p.pixel_stride::<BD>()) + i;
            *p.index_mut::<BD>() =
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
    sumsq: &mut [i32; (64 + 2 + 2) * REST_UNIT_STRIDE],
    sum: &mut [BD::Coef; (64 + 2 + 2) * REST_UNIT_STRIDE],
    src: &[BD::Pixel; (64 + 3 + 3) * REST_UNIT_STRIDE],
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
    sumsq: &mut [i32; (64 + 2 + 2) * REST_UNIT_STRIDE],
    sum: &mut [BD::Coef; (64 + 2 + 2) * REST_UNIT_STRIDE],
    src: &[BD::Pixel; (64 + 3 + 3) * REST_UNIT_STRIDE],
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
    dst: &mut [BD::Coef; 64 * 384],
    src: &[BD::Pixel; (64 + 3 + 3) * REST_UNIT_STRIDE],
    w: usize,
    h: usize,
    n: c_int,
    s: c_uint,
    bd: BD,
) {
    let sgr_one_by_x = if n == 25 { 164 } else { 455 };

    // Selfguided filter is applied to a maximum stripe height of 64 + 3 pixels
    // of padding above and below
    let mut sumsq = [0; (64 + 2 + 2) * REST_UNIT_STRIDE];
    // By inverting `a` and `b` after the boxsums, `b` can be of `BD::Coef` instead of `i32`.
    let mut sum = [0.as_::<BD::Coef>(); (64 + 2 + 2) * REST_UNIT_STRIDE];

    let step = (n == 25) as usize + 1;
    if n == 25 {
        boxsum5::<BD>(&mut sumsq, &mut sum, src, w + 6, h + 6);
    } else {
        boxsum3::<BD>(&mut sumsq, &mut sum, src, w + 6, h + 6);
    }
    let bitdepth_min_8 = bd.bitdepth() - 8;

    let mut a = CursorMut::new(&mut sumsq) + 2 * REST_UNIT_STRIDE + 3;
    let mut b = CursorMut::new(&mut sum) + 2 * REST_UNIT_STRIDE + 3;

    let mut aa = a.clone() - REST_UNIT_STRIDE;
    let mut bb = b.clone() - REST_UNIT_STRIDE;
    for _ in (-1..h as isize + 1).step_by(step) {
        for i in -1..w as isize + 1 {
            let a = aa[i] + (1 << 2 * bitdepth_min_8 >> 1) >> 2 * bitdepth_min_8;
            let b = bb[i].as_::<c_int>() + (1 << bitdepth_min_8 >> 1) >> bitdepth_min_8;

            let p = cmp::max(a * n - b * b, 0) as c_uint;
            let z = (p * s + (1 << 19)) >> 20;
            let x = dav1d_sgr_x_by_x[cmp::min(z, 255) as usize] as c_uint;

            // This is where we invert A and B, so that B is of size coef.
            aa[i] = ((x * bb[i].as_::<c_uint>() * sgr_one_by_x + (1 << 11)) >> 12) as c_int;
            bb[i] = x.as_::<BD::Coef>();
        }
        aa += step as usize * REST_UNIT_STRIDE;
        bb += step as usize * REST_UNIT_STRIDE;
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

    const MAX_RESTORATION_WIDTH: usize = 256 * 3 / 2;

    let mut src = &src[3 * REST_UNIT_STRIDE + 3..];
    let mut dst = dst.as_mut_slice();
    if n == 25 {
        let mut j = 0;
        while j < h - 1 {
            for i in 0..w {
                let (a, b) = (six_neighbors(&b, i as isize), six_neighbors(&a, i as isize));
                dst[i] = ((b - a * src[i].as_::<c_int>() + (1 << 8)) >> 9).as_();
            }
            dst = &mut dst[MAX_RESTORATION_WIDTH..];
            src = &src[REST_UNIT_STRIDE..];
            b += REST_UNIT_STRIDE;
            a += REST_UNIT_STRIDE;
            for i in 0..w {
                let (a, b) = (
                    b[i].as_::<c_int>() * 6 + (b[i as isize - 1] + b[i + 1]).as_::<c_int>() * 5,
                    a[i] * 6 + (a[i as isize - 1] + a[i + 1]) * 5,
                );
                dst[i] = (b - a * src[i].as_::<c_int>() + (1 << 7) >> 8).as_();
            }
            dst = &mut dst[MAX_RESTORATION_WIDTH..];
            src = &src[REST_UNIT_STRIDE..];
            b += REST_UNIT_STRIDE;
            a += REST_UNIT_STRIDE;
            j += 2;
        }
        // Last row, when number of rows is odd
        if j + 1 == h {
            for i in 0..w {
                let (a, b) = (six_neighbors(&b, i as isize), six_neighbors(&a, i as isize));
                dst[i] = (b - a * src[i].as_::<c_int>() + (1 << 8) >> 9).as_();
            }
        }
    } else {
        for _ in 0..h {
            for i in 0..w {
                let (a, b) = (
                    eight_neighbors(&b, i as isize),
                    eight_neighbors(&a, i as isize),
                );
                dst[i] = (b - a * src[i].as_::<c_int>() + (1 << 8) >> 9).as_();
            }
            dst = &mut dst[384..];
            src = &src[REST_UNIT_STRIDE..];
            b += REST_UNIT_STRIDE;
            a += REST_UNIT_STRIDE;
        }
    };
}

/// # Safety
///
/// Must be called by [`loop_restoration_filter::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn sgr_5x5_c_erased<BD: BitDepth>(
    _p_ptr: *mut DynPixel,
    _stride: ptrdiff_t,
    left: *const LeftPixelRow<DynPixel>,
    lpf_ptr: *const DynPixel,
    w: c_int,
    h: c_int,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: c_int,
    p: *const FFISafe<Rav1dPictureDataComponentOffset>,
    lpf: *const FFISafe<DisjointMut<AlignedVec64<u8>>>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `loop_restoration_filter::Fn::call`.
    let p = *unsafe { FFISafe::get(p) };
    let left = left.cast();
    // SAFETY: Was passed as `FFISafe::new(_)` in `loop_restoration_filter::Fn::call`.
    let lpf = unsafe { FFISafe::get(lpf) };
    let lpf_ptr = lpf_ptr.cast();
    let lpf_off = reconstruct_lpf_offset::<BD>(lpf, lpf_ptr);
    let w = w as usize;
    let h = h as usize;
    let bd = BD::from_c(bitdepth_max);
    // SAFETY: Length sliced in `loop_restoration_filter::Fn::call`.
    let left = unsafe { slice::from_raw_parts(left, h) };
    sgr_5x5_rust(p, left, lpf, lpf_off, w, h, params, edges, bd)
}

fn sgr_5x5_rust<BD: BitDepth>(
    p: Rav1dPictureDataComponentOffset,
    left: &[LeftPixelRow<BD::Pixel>],
    lpf: &DisjointMut<AlignedVec64<u8>>,
    lpf_off: isize,
    w: usize,
    h: usize,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bd: BD,
) {
    // Selfguided filter is applied to a maximum stripe height of 64 + 3 pixels
    // of padding above and below
    let mut tmp = [0.as_(); (64 + 3 + 3) * REST_UNIT_STRIDE];

    // Selfguided filter outputs to a maximum stripe height of 64 and a
    // maximum restoration width of 384 (256 * 1.5)
    let mut dst = [0.as_(); 64 * 384];

    padding::<BD>(&mut tmp, p, left, lpf, lpf_off, w, h, edges);
    let sgr = params.sgr();
    selfguided_filter(&mut dst, &mut tmp, w, h, 25, sgr.s0, bd);

    let w0 = sgr.w0 as c_int;
    for j in 0..h {
        let p = p + (j as isize * p.pixel_stride::<BD>());
        let p = &mut *p.slice_mut::<BD>(w);
        for i in 0..w {
            let v = w0 * dst[j * 384 + i].as_::<c_int>();
            p[i] = bd.iclip_pixel(p[i].as_::<c_int>() + (v + (1 << 10) >> 11));
        }
    }
}

/// # Safety
///
/// Must be called by [`loop_restoration_filter::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn sgr_3x3_c_erased<BD: BitDepth>(
    _p_ptr: *mut DynPixel,
    _stride: ptrdiff_t,
    left: *const LeftPixelRow<DynPixel>,
    lpf_ptr: *const DynPixel,
    w: c_int,
    h: c_int,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: c_int,
    p: *const FFISafe<Rav1dPictureDataComponentOffset>,
    lpf: *const FFISafe<DisjointMut<AlignedVec64<u8>>>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `loop_restoration_filter::Fn::call`.
    let p = *unsafe { FFISafe::get(p) };
    let left = left.cast();
    // SAFETY: Was passed as `FFISafe::new(_)` in `loop_restoration_filter::Fn::call`.
    let lpf = unsafe { FFISafe::get(lpf) };
    let lpf_ptr = lpf_ptr.cast();
    let lpf_off = reconstruct_lpf_offset::<BD>(lpf, lpf_ptr);
    let w = w as usize;
    let h = h as usize;
    let bd = BD::from_c(bitdepth_max);
    // SAFETY: Length sliced in `loop_restoration_filter::Fn::call`.
    let left = unsafe { slice::from_raw_parts(left, h) };
    sgr_3x3_rust(p, left, lpf, lpf_off, w, h, params, edges, bd)
}

fn sgr_3x3_rust<BD: BitDepth>(
    p: Rav1dPictureDataComponentOffset,
    left: &[LeftPixelRow<BD::Pixel>],
    lpf: &DisjointMut<AlignedVec64<u8>>,
    lpf_off: isize,
    w: usize,
    h: usize,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bd: BD,
) {
    let mut tmp = [0.as_(); (64 + 3 + 3) * REST_UNIT_STRIDE];
    let mut dst = [0.as_(); 64 * 384];

    padding::<BD>(&mut tmp, p, left, lpf, lpf_off, w, h, edges);
    let sgr = params.sgr();
    selfguided_filter(&mut dst, &mut tmp, w, h, 9, sgr.s1, bd);

    let w1 = sgr.w1 as c_int;
    for j in 0..h {
        let p = p + (j as isize * p.pixel_stride::<BD>());
        let p = &mut *p.slice_mut::<BD>(w);
        for i in 0..w {
            let v = w1 * dst[j * 384 + i].as_::<c_int>();
            p[i] = bd.iclip_pixel(p[i].as_::<c_int>() + (v + (1 << 10) >> 11));
        }
    }
}

/// # Safety
///
/// Must be called by [`loop_restoration_filter::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn sgr_mix_c_erased<BD: BitDepth>(
    _p_ptr: *mut DynPixel,
    _stride: ptrdiff_t,
    left: *const LeftPixelRow<DynPixel>,
    lpf_ptr: *const DynPixel,
    w: c_int,
    h: c_int,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: c_int,
    p: *const FFISafe<Rav1dPictureDataComponentOffset>,
    lpf: *const FFISafe<DisjointMut<AlignedVec64<u8>>>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `loop_restoration_filter::Fn::call`.
    let p = *unsafe { FFISafe::get(p) };
    let left = left.cast();
    // SAFETY: Was passed as `FFISafe::new(_)` in `loop_restoration_filter::Fn::call`.
    let lpf = unsafe { FFISafe::get(lpf) };
    let lpf_ptr = lpf_ptr.cast();
    let lpf_off = reconstruct_lpf_offset::<BD>(lpf, lpf_ptr);
    let w = w as usize;
    let h = h as usize;
    let bd = BD::from_c(bitdepth_max);
    // SAFETY: Length sliced in `loop_restoration_filter::Fn::call`.
    let left = unsafe { slice::from_raw_parts(left, h) };
    sgr_mix_rust(p, left, lpf, lpf_off, w, h, params, edges, bd)
}

fn sgr_mix_rust<BD: BitDepth>(
    p: Rav1dPictureDataComponentOffset,
    left: &[LeftPixelRow<BD::Pixel>],
    lpf: &DisjointMut<AlignedVec64<u8>>,
    lpf_off: isize,
    w: usize,
    h: usize,
    params: &LooprestorationParams,
    edges: LrEdgeFlags,
    bd: BD,
) {
    let mut tmp = [0.as_(); (64 + 3 + 3) * REST_UNIT_STRIDE];
    let mut dst0 = [0.as_(); 64 * 384];
    let mut dst1 = [0.as_(); 64 * 384];

    padding::<BD>(&mut tmp, p, left, lpf, lpf_off, w, h, edges);
    let sgr = params.sgr();
    selfguided_filter(&mut dst0, &mut tmp, w, h, 25, sgr.s0, bd);
    selfguided_filter(&mut dst1, &mut tmp, w, h, 9, sgr.s1, bd);

    let w0 = sgr.w0 as c_int;
    let w1 = sgr.w1 as c_int;
    for j in 0..h {
        let p = p + (j as isize * p.pixel_stride::<BD>());
        let p = &mut *p.slice_mut::<BD>(w);
        for i in 0..w {
            let v = w0 * dst0[j * 384 + i].as_::<c_int>() + w1 * dst1[j * 384 + i].as_::<c_int>();
            p[i] = bd.iclip_pixel(p[i].as_::<c_int>() + (v + (1 << 10) >> 11));
        }
    }
}

#[deny(unsafe_op_in_unsafe_fn)]
#[cfg(all(feature = "asm", target_arch = "arm"))]
mod neon {
    use super::*;

    use crate::align::Align16;
    use crate::include::common::bitdepth::bd_fn;
    use libc::intptr_t;
    use std::ptr;

    wrap_fn_ptr!(unsafe extern "C" fn wiener_filter_h(
        dst: *mut i16,
        left: *const LeftPixelRow<DynPixel>,
        src: *const DynPixel,
        stride: ptrdiff_t,
        fh: &[i16; 8],
        w: intptr_t,
        h: c_int,
        edges: LrEdgeFlags,
        bitdepth_max: c_int,
    ) -> ());

    impl wiener_filter_h::Fn {
        fn call<BD: BitDepth>(
            &self,
            dst: &mut [i16],
            left: *const LeftPixelRow<BD::Pixel>,
            src: *const BD::Pixel,
            stride: ptrdiff_t,
            fh: &[i16; 8],
            w: c_int,
            h: c_int,
            edges: LrEdgeFlags,
            bd: BD,
        ) {
            let dst = dst.as_mut_ptr();
            let left = left.cast();
            let src = src.cast();
            let w = w as intptr_t;
            let bd = bd.into_c();
            // SAFETY: asm should be safe.
            unsafe { self.get()(dst, left, src, stride, fh, w, h, edges, bd) }
        }
    }

    wrap_fn_ptr!(unsafe extern "C" fn wiener_filter_v(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        mid: *const i16,
        w: c_int,
        h: c_int,
        fv: &[i16; 8],
        edges: LrEdgeFlags,
        mid_stride: ptrdiff_t,
        bitdepth_max: c_int,
    ) -> ());

    impl wiener_filter_v::Fn {
        fn call<BD: BitDepth>(
            &self,
            dst: *mut BD::Pixel,
            stride: ptrdiff_t,
            mid: &mut [i16],
            w: c_int,
            h: c_int,
            fv: &[i16; 8],
            edges: LrEdgeFlags,
            mid_stride: usize,
            bd: BD,
        ) {
            let dst = dst.cast();
            let mid = mid.as_mut_ptr();
            let mid_stride = (mid_stride * mem::size_of::<i16>()) as ptrdiff_t;
            let bd = bd.into_c();
            // SAFETY: asm should be safe.
            unsafe { self.get()(dst, stride, mid, w, h, fv, edges, mid_stride, bd) }
        }
    }

    /// # Safety
    ///
    /// Must be called by [`loop_restoration_filter::Fn::call`].
    #[deny(unsafe_op_in_unsafe_fn)]
    pub unsafe extern "C" fn wiener_filter_neon_erased<BD: BitDepth>(
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
        _lpf: *const FFISafe<DisjointMut<AlignedVec64<u8>>>,
    ) {
        let p = p.cast();
        let left = left.cast();
        let lpf = lpf.cast();
        let bd = BD::from_c(bitdepth_max);
        wiener_filter_neon(p, stride, left, lpf, w, h, params, edges, bd)
    }

    fn wiener_filter_neon<BD: BitDepth>(
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
        let mut mid = Align16([0; 68 * 384]);
        let mid_stride = w as usize + 7 & !7;
        bd_fn!(wiener_filter_h::decl_fn, BD, wiener_filter_h, neon).call(
            &mut mid.0[2 * mid_stride..],
            left,
            dst,
            stride,
            &filter[0],
            w,
            h,
            edges,
            bd,
        );
        if edges.contains(LrEdgeFlags::TOP) {
            bd_fn!(wiener_filter_h::decl_fn, BD, wiener_filter_h, neon).call(
                &mut mid.0[..],
                ptr::null(),
                lpf,
                stride,
                &filter[0],
                w,
                2,
                edges,
                bd,
            );
        }
        if edges.contains(LrEdgeFlags::BOTTOM) {
            bd_fn!(wiener_filter_h::decl_fn, BD, wiener_filter_h, neon).call(
                &mut mid.0[(2 + h as usize) * mid_stride..],
                ptr::null(),
                // `lpf` may be negatively out of bounds.
                lpf.wrapping_offset(6 * BD::pxstride(stride)),
                stride,
                &filter[0],
                w,
                2,
                edges,
                bd,
            );
        }
        bd_fn!(wiener_filter_v::decl_fn, BD, wiener_filter_v, neon).call(
            dst,
            stride,
            &mut mid.0[2 * mid_stride..],
            w,
            h,
            &filter[1],
            edges,
            mid_stride,
            bd,
        );
    }

    wrap_fn_ptr!(unsafe extern "C" fn sgr_box3_h(
        sumsq: *mut i32,
        sum: *mut i16,
        left: *const LeftPixelRow<DynPixel>,
        src: *const DynPixel,
        stride: ptrdiff_t,
        w: c_int,
        h: c_int,
        edges: LrEdgeFlags,
    ) -> ());

    impl sgr_box3_h::Fn {
        fn call<BD: BitDepth>(
            &self,
            sumsq: &mut [i32],
            sum: &mut [i16],
            left: Option<&[LeftPixelRow<BD::Pixel>]>,
            src: *const BD::Pixel,
            stride: ptrdiff_t,
            w: c_int,
            h: c_int,
            edges: LrEdgeFlags,
        ) {
            let sumsq = sumsq.as_mut_ptr();
            let sum = sum.as_mut_ptr();
            let left = left
                .map(|left| left.as_ptr().cast())
                .unwrap_or_else(ptr::null);
            let src = src.cast();
            // SAFETY: asm should be safe.
            unsafe { self.get()(sumsq, sum, left, src, stride, w, h, edges) }
        }

        const fn neon<BD: BitDepth>() -> Self {
            bd_fn!(sgr_box3_h::decl_fn, BD, sgr_box3_h, neon)
        }
    }

    wrap_fn_ptr!(unsafe extern "C" fn sgr_box_v(
        sumsq: *mut i32,
        sum: *mut i16,
        w: c_int,
        h: c_int,
        edges: LrEdgeFlags,
    ) -> ());

    impl sgr_box_v::Fn {
        fn call(&self, sumsq: &mut [i32], sum: &mut [i16], w: c_int, h: c_int, edges: LrEdgeFlags) {
            let sumsq = sumsq.as_mut_ptr();
            let sum = sum.as_mut_ptr();
            // SAFETY: asm should be safe.
            unsafe { self.get()(sumsq, sum, w, h, edges) }
        }

        const fn neon3() -> Self {
            sgr_box_v::decl_fn!(fn dav1d_sgr_box3_v_neon)
        }

        const fn neon5() -> Self {
            sgr_box_v::decl_fn!(fn dav1d_sgr_box5_v_neon)
        }
    }

    wrap_fn_ptr!(unsafe extern "C" fn sgr_calc_ab(
        a: *mut i32,
        b: *mut i16,
        w: c_int,
        h: c_int,
        strength: c_int,
        bitdepth_max: c_int,
    ) -> ());

    impl sgr_calc_ab::Fn {
        fn call<BD: BitDepth>(
            &self,
            a: &mut [i32],
            b: &mut [i16],
            w: c_int,
            h: c_int,
            strength: u32,
            bd: BD,
        ) {
            let a = a.as_mut_ptr();
            let b = b.as_mut_ptr();
            let strength = strength as c_int;
            let bd = bd.into_c();
            // SAFETY: asm should be safe.
            unsafe { self.get()(a, b, w, h, strength, bd) }
        }

        const fn neon1() -> Self {
            sgr_calc_ab::decl_fn!(fn dav1d_sgr_calc_ab1_neon)
        }

        const fn neon2() -> Self {
            sgr_calc_ab::decl_fn!(fn dav1d_sgr_calc_ab2_neon)
        }
    }

    wrap_fn_ptr!(unsafe extern "C" fn sgr_finish_filter(
        tmp: &mut Align16<[i16; 64 * 384]>,
        src: *const DynPixel,
        stride: ptrdiff_t,
        a: *const i32,
        b: *const i16,
        w: c_int,
        h: c_int,
    ) -> ());

    impl sgr_finish_filter::Fn {
        fn call<BD: BitDepth>(
            &self,
            tmp: &mut Align16<[i16; 64 * 384]>,
            src: Rav1dPictureDataComponentOffset,
            a: &[i32],
            b: &[i16],
            w: c_int,
            h: c_int,
        ) {
            let src_ptr = src.as_ptr::<BD>().cast();
            let stride = src.stride();
            let a = a.as_ptr();
            let b = b.as_ptr();
            // SAFETY: asm should be safe.
            unsafe { self.get()(tmp, src_ptr, stride, a, b, w, h) }
        }

        const fn neon1<BD: BitDepth>() -> Self {
            bd_fn!(sgr_finish_filter::decl_fn, BD, sgr_finish_filter1, neon)
        }

        const fn neon2<BD: BitDepth>() -> Self {
            bd_fn!(sgr_finish_filter::decl_fn, BD, sgr_finish_filter2, neon)
        }
    }

    /// Filter with a 3x3 box (radius=1).
    fn rav1d_sgr_filter1_neon<BD: BitDepth>(
        tmp: &mut Align16<[i16; 64 * 384]>,
        src: Rav1dPictureDataComponentOffset,
        left: &[LeftPixelRow<BD::Pixel>],
        lpf: *const BD::Pixel,
        w: c_int,
        h: c_int,
        strength: u32,
        edges: LrEdgeFlags,
        bd: BD,
    ) {
        const STRIDE: usize = 384 + 16;

        let mut sumsq_mem = Align16([0; STRIDE * 68 + 8]);
        let sumsq = &mut sumsq_mem.0[8..];
        let mut sum_mem = Align16([0; STRIDE * 68 + 16]);
        let sum = &mut sum_mem.0[16..];
        sgr_box3_h::Fn::neon::<BD>().call::<BD>(
            &mut sumsq[2 * STRIDE..],
            &mut sum[2 * STRIDE..],
            Some(left),
            src.as_ptr::<BD>(),
            src.stride(),
            w,
            h,
            edges,
        );
        if edges.contains(LrEdgeFlags::TOP) {
            sgr_box3_h::Fn::neon::<BD>().call::<BD>(
                sumsq,
                sum,
                None,
                lpf,
                src.stride(),
                w,
                2,
                edges,
            );
        }
        if edges.contains(LrEdgeFlags::BOTTOM) {
            let h = h as usize;
            sgr_box3_h::Fn::neon::<BD>().call::<BD>(
                &mut sumsq[(h + 2) * STRIDE..],
                &mut sum[(h + 2) * STRIDE..],
                None,
                // `lpf` may be negatively out of bounds.
                lpf.wrapping_offset(6 * src.pixel_stride::<BD>()),
                src.stride(),
                w,
                2,
                edges,
            );
        }
        sgr_box_v::Fn::neon3().call(
            &mut sumsq[2 * STRIDE..],
            &mut sum[2 * STRIDE..],
            w,
            h,
            edges,
        );
        let a = &mut sumsq[2 * STRIDE..];
        let b = &mut sum[2 * STRIDE..];
        sgr_calc_ab::Fn::neon1().call(a, b, w, h, strength, bd);
        sgr_finish_filter::Fn::neon1::<BD>().call::<BD>(tmp, src, a, b, w, h);
    }

    wrap_fn_ptr!(unsafe extern "C" fn sgr_box5_h(
        sumsq: *mut i32,
        sum: *mut i16,
        left: *const LeftPixelRow<DynPixel>,
        src: *const DynPixel,
        stride: ptrdiff_t,
        w: c_int,
        h: c_int,
        edges: LrEdgeFlags,
    ) -> ());

    impl sgr_box5_h::Fn {
        fn call<BD: BitDepth>(
            &self,
            sumsq: &mut [i32],
            sum: &mut [i16],
            left: Option<&[LeftPixelRow<BD::Pixel>]>,
            src: *const BD::Pixel,
            stride: ptrdiff_t,
            w: c_int,
            h: c_int,
            edges: LrEdgeFlags,
        ) {
            let sumsq = sumsq.as_mut_ptr();
            let sum = sum.as_mut_ptr();
            let left = left
                .map(|left| left.as_ptr().cast())
                .unwrap_or_else(ptr::null);
            let src = src.cast();
            // SAFETY: asm should be safe.
            unsafe { self.get()(sumsq, sum, left, src, stride, w, h, edges) }
        }

        const fn neon<BD: BitDepth>() -> Self {
            bd_fn!(sgr_box5_h::decl_fn, BD, sgr_box5_h, neon)
        }
    }

    /// Filter with a 5x5 box (radius=2).
    fn rav1d_sgr_filter2_neon<BD: BitDepth>(
        tmp: &mut Align16<[i16; 64 * 384]>,
        src: Rav1dPictureDataComponentOffset,
        left: &[LeftPixelRow<BD::Pixel>],
        lpf: *const BD::Pixel,
        w: c_int,
        h: c_int,
        strength: u32,
        edges: LrEdgeFlags,
        bd: BD,
    ) {
        const STRIDE: usize = 384 + 16;

        let mut sumsq_mem = Align16([0; STRIDE * 68 + 8]);
        let sumsq = &mut sumsq_mem.0[8..];
        let mut sum_mem = Align16([0; STRIDE * 68 + 16]);
        let sum = &mut sum_mem.0[16..];
        sgr_box5_h::Fn::neon::<BD>().call::<BD>(
            &mut sumsq[2 * STRIDE..],
            &mut sum[2 * STRIDE..],
            Some(left),
            src.as_ptr::<BD>(),
            src.stride(),
            w,
            h,
            edges,
        );
        if edges.contains(LrEdgeFlags::TOP) {
            sgr_box5_h::Fn::neon::<BD>().call::<BD>(
                sumsq,
                sum,
                None,
                lpf,
                src.stride(),
                w,
                2,
                edges,
            );
        }
        if edges.contains(LrEdgeFlags::BOTTOM) {
            let h = h as usize;
            sgr_box5_h::Fn::neon::<BD>().call::<BD>(
                &mut sumsq[(h + 2) * STRIDE..],
                &mut sum[(h + 2) * STRIDE..],
                None,
                // `lpf` may be negatively out of bounds.
                lpf.wrapping_offset(6 * src.pixel_stride::<BD>()),
                src.stride(),
                w,
                2,
                edges,
            );
        }
        sgr_box_v::Fn::neon5().call(
            &mut sumsq[2 * STRIDE..],
            &mut sum[2 * STRIDE..],
            w,
            h,
            edges,
        );
        let a = &mut sumsq[2 * STRIDE..];
        let b = &mut sum[2 * STRIDE..];
        sgr_calc_ab::Fn::neon2().call(a, b, w, h, strength, bd);
        sgr_finish_filter::Fn::neon2::<BD>().call::<BD>(tmp, src, a, b, w, h);
    }

    wrap_fn_ptr!(unsafe extern "C" fn sgr_weighted1(
        dst: *mut DynPixel,
        dst_stride: ptrdiff_t,
        src: *const DynPixel,
        src_stride: ptrdiff_t,
        t1: &mut Align16<[i16; 64 * 384]>,
        w: c_int,
        h: c_int,
        wt: c_int,
        bitdepth_max: c_int,
    ) -> ());

    impl sgr_weighted1::Fn {
        fn call<BD: BitDepth>(
            &self,
            dst: Rav1dPictureDataComponentOffset,
            src: Rav1dPictureDataComponentOffset,
            t1: &mut Align16<[i16; 64 * 384]>,
            w: c_int,
            h: c_int,
            wt: i16,
            bd: BD,
        ) {
            let dst_ptr = dst.as_mut_ptr::<BD>().cast();
            let dst_stride = dst.stride();
            let src_ptr = src.as_ptr::<BD>().cast();
            let src_stride = src.stride();
            let wt = wt.into();
            let bd = bd.into_c();
            // SAFETY: asm should be safe.
            unsafe { self.get()(dst_ptr, dst_stride, src_ptr, src_stride, t1, w, h, wt, bd) }
        }

        const fn neon<BD: BitDepth>() -> Self {
            bd_fn!(sgr_weighted1::decl_fn, BD, sgr_weighted1, neon)
        }
    }

    wrap_fn_ptr!(unsafe extern "C" fn sgr_weighted2(
        dst: *mut DynPixel,
        dst_stride: ptrdiff_t,
        src: *const DynPixel,
        src_stride: ptrdiff_t,
        t1: &mut Align16<[i16; 64 * 384]>,
        t2: &mut Align16<[i16; 64 * 384]>,
        w: c_int,
        h: c_int,
        wt: &[i16; 2],
        bitdepth_max: c_int,
    ) -> ());

    impl sgr_weighted2::Fn {
        fn call<BD: BitDepth>(
            &self,
            dst: Rav1dPictureDataComponentOffset,
            src: Rav1dPictureDataComponentOffset,
            t1: &mut Align16<[i16; 64 * 384]>,
            t2: &mut Align16<[i16; 64 * 384]>,
            w: c_int,
            h: c_int,
            wt: &[i16; 2],
            bd: BD,
        ) {
            let dst_ptr = dst.as_mut_ptr::<BD>().cast();
            let dst_stride = dst.stride();
            let src_ptr = src.as_ptr::<BD>().cast();
            let src_stride = src.stride();
            let bd = bd.into_c();
            // SAFETY: asm should be safe.
            unsafe {
                self.get()(
                    dst_ptr, dst_stride, src_ptr, src_stride, t1, t2, w, h, wt, bd,
                )
            }
        }

        const fn neon<BD: BitDepth>() -> Self {
            bd_fn!(sgr_weighted2::decl_fn, BD, sgr_weighted2, neon)
        }
    }

    pub fn sgr_filter_5x5_neon<BD: BitDepth>(
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
        rav1d_sgr_filter2_neon(&mut tmp, dst, left, lpf, w, h, sgr.s0, edges, bd);
        sgr_weighted1::Fn::neon::<BD>().call(dst, dst, &mut tmp, w, h, sgr.w0, bd);
    }

    pub fn sgr_filter_3x3_neon<BD: BitDepth>(
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
        rav1d_sgr_filter1_neon(&mut tmp, dst, left, lpf, w, h, sgr.s1, edges, bd);
        sgr_weighted1::Fn::neon::<BD>().call(dst, dst, &mut tmp, w, h, sgr.w1, bd);
    }

    pub fn sgr_filter_mix_neon<BD: BitDepth>(
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
        let mut tmp1 = Align16([0; 64 * 384]);
        let mut tmp2 = Align16([0; 64 * 384]);
        let sgr = params.sgr();
        rav1d_sgr_filter2_neon(&mut tmp1, dst, left, lpf, w, h, sgr.s0, edges, bd);
        rav1d_sgr_filter1_neon(&mut tmp2, dst, left, lpf, w, h, sgr.s1, edges, bd);
        let wt = [sgr.w0, sgr.w1];
        sgr_weighted2::Fn::neon::<BD>().call(dst, dst, &mut tmp1, &mut tmp2, w, h, &wt, bd);
    }
}

#[deny(unsafe_op_in_unsafe_fn)]
#[cfg(all(feature = "asm", target_arch = "aarch64"))]
mod neon {
    use super::*;

    use crate::align::Align16;
    use std::array;
    use std::ptr;

    fn rotate<const LEN: usize, const MID: usize>(
        a: &mut [*mut i32; LEN],
        b: &mut [*mut i16; LEN],
    ) {
        a.rotate_left(MID);
        b.rotate_left(MID);
    }

    wrap_fn_ptr!(unsafe extern "C" fn sgr_box_row_h(
        sumsq: *mut i32,
        sum: *mut i16,
        left: *const LeftPixelRow<DynPixel>,
        src: *const DynPixel,
        w: c_int,
        edges: LrEdgeFlags,
        bitdepth_max: c_int,
    ) -> ());

    impl sgr_box_row_h::Fn {
        fn call<BD: BitDepth>(
            &self,
            sumsq: *mut i32,
            sum: *mut i16,
            left: Option<&[LeftPixelRow<BD::Pixel>]>,
            src: *const BD::Pixel,
            w: c_int,
            edges: LrEdgeFlags,
            bd: BD,
        ) {
            let left = left
                .map(|left| left.as_ptr().cast())
                .unwrap_or_else(ptr::null);
            let src = src.cast();
            let bd = bd.into_c();
            // SAFETY: asm should be safe.
            unsafe { self.get()(sumsq, sum, left, src, w, edges, bd) }
        }

        const fn neon3<BD: BitDepth>() -> Self {
            bd_fn!(sgr_box_row_h::decl_fn, BD, sgr_box3_row_h, neon)
        }

        const fn neon5<BD: BitDepth>() -> Self {
            bd_fn!(sgr_box_row_h::decl_fn, BD, sgr_box5_row_h, neon)
        }
    }

    wrap_fn_ptr!(unsafe extern "C" fn sgr_box35_row_h(
        sumsq3: *mut i32,
        sum3: *mut i16,
        sumsq5: *mut i32,
        sum5: *mut i16,
        left: *const LeftPixelRow<DynPixel>,
        src: *const DynPixel,
        w: c_int,
        edges: LrEdgeFlags,
        bitdepth_max: c_int,
    ) -> ());

    impl sgr_box35_row_h::Fn {
        fn call<BD: BitDepth>(
            &self,
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
            let left = left
                .map(|left| left.as_ptr().cast())
                .unwrap_or_else(ptr::null);
            let src = src.cast();
            let bd = bd.into_c();
            // SAFETY: asm should be safe.
            unsafe { self.get()(sumsq3, sum3, sumsq5, sum5, left, src, w, edges, bd) }
        }

        const fn neon<BD: BitDepth>() -> Self {
            bd_fn!(sgr_box35_row_h::decl_fn, BD, sgr_box35_row_h, neon)
        }
    }

    wrap_fn_ptr!(unsafe extern "C" fn sgr_box_vert(
        sumsq: *mut *mut i32,
        sum: *mut *mut i16,
        aa: *mut i32,
        bb: *mut i16,
        w: c_int,
        s: c_int,
        bitdepth_max: c_int,
    ) -> ());

    impl sgr_box_vert::Fn {
        fn call<BD: BitDepth, const N: usize>(
            &self,
            sumsq: &mut [*mut i32; N],
            sum: &mut [*mut i16; N],
            sumsq_out: *mut i32,
            sum_out: *mut i16,
            w: c_int,
            s: c_int,
            bd: BD,
        ) {
            const { assert!(N == 3 || N == 5) };
            let sumsq = sumsq.as_mut_ptr();
            let sum = sum.as_mut_ptr();
            let bd = bd.into_c();
            // SAFETY: asm should be safe.
            unsafe { self.get()(sumsq, sum, sumsq_out, sum_out, w, s, bd) }
        }

        const fn neon3() -> Self {
            sgr_box_vert::decl_fn!(fn dav1d_sgr_box3_vert_neon)
        }

        const fn neon5() -> Self {
            sgr_box_vert::decl_fn!(fn dav1d_sgr_box5_vert_neon)
        }
    }

    fn sgr_box3_vert_neon<BD: BitDepth>(
        sumsq: &mut [*mut i32; 3],
        sum: &mut [*mut i16; 3],
        sumsq_out: *mut i32,
        sum_out: *mut i16,
        w: c_int,
        s: c_int,
        bd: BD,
    ) {
        sgr_box_vert::Fn::neon3().call(sumsq, sum, sumsq_out, sum_out, w, s, bd);
        rotate::<3, 1>(sumsq, sum);
    }

    fn sgr_box5_vert_neon<BD: BitDepth>(
        sumsq: &mut [*mut i32; 5],
        sum: &mut [*mut i16; 5],
        sumsq_out: *mut i32,
        sum_out: *mut i16,
        w: c_int,
        s: c_int,
        bd: BD,
    ) {
        sgr_box_vert::Fn::neon5().call(sumsq, sum, sumsq_out, sum_out, w, s, bd);
        rotate::<5, 2>(sumsq, sum);
    }

    wrap_fn_ptr!(unsafe extern "C" fn sgr_finish_weighted1(
        dst: *mut DynPixel,
        a_ptrs: *mut *mut i32,
        b_ptrs: *mut *mut i16,
        w: c_int,
        w1: c_int,
        bitdepth_max: c_int,
    ) -> ());

    impl sgr_finish_weighted1::Fn {
        fn call<BD: BitDepth>(
            &self,
            dst: Rav1dPictureDataComponentOffset,
            a_ptrs: &mut [*mut i32; 3],
            b_ptrs: &mut [*mut i16; 3],
            w: c_int,
            w1: c_int,
            bd: BD,
        ) {
            let dst = dst.as_mut_ptr::<BD>().cast();
            let a_ptrs = a_ptrs.as_mut_ptr();
            let b_ptrs = b_ptrs.as_mut_ptr();
            let bd = bd.into_c();
            // SAFETY: asm should be safe.
            unsafe { self.get()(dst, a_ptrs, b_ptrs, w, w1, bd) }
        }

        const fn neon<BD: BitDepth>() -> Self {
            bd_fn!(
                sgr_finish_weighted1::decl_fn,
                BD,
                sgr_finish_weighted1,
                neon
            )
        }
    }

    wrap_fn_ptr!(unsafe extern "C" fn sgr_finish_weighted2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        a_ptrs: *mut *mut i32,
        b_ptrs: *mut *mut i16,
        w: c_int,
        h: c_int,
        w1: c_int,
        bitdepth_max: c_int,
    ) -> ());

    impl sgr_finish_weighted2::Fn {
        fn call<BD: BitDepth>(
            &self,
            dst: Rav1dPictureDataComponentOffset,
            a_ptrs: &mut [*mut i32; 2],
            b_ptrs: &mut [*mut i16; 2],
            w: c_int,
            h: c_int,
            w1: c_int,
            bd: BD,
        ) {
            let dst_ptr = dst.as_mut_ptr::<BD>().cast();
            let dst_stride = dst.stride();
            let a_ptrs = a_ptrs.as_mut_ptr();
            let b_ptrs = b_ptrs.as_mut_ptr();
            let bd = bd.into_c();
            // SAFETY: asm should be safe.
            unsafe { self.get()(dst_ptr, dst_stride, a_ptrs, b_ptrs, w, h, w1, bd) }
        }

        const fn neon<BD: BitDepth>() -> Self {
            bd_fn!(
                sgr_finish_weighted2::decl_fn,
                BD,
                sgr_finish_weighted2,
                neon
            )
        }
    }

    wrap_fn_ptr!(unsafe extern "C" fn sgr_finish_filter_2rows(
        tmp: *mut i16,
        src: *const DynPixel,
        src_stride: ptrdiff_t,
        a_ptrs: *mut *mut i32,
        b_ptrs: *mut *mut i16,
        w: c_int,
        h: c_int,
        bitdepth_max: c_int,
    ) -> ());

    impl sgr_finish_filter_2rows::Fn {
        fn call<BD: BitDepth, const N: usize>(
            &self,
            tmp: &mut Align16<[i16; 2 * FILTER_OUT_STRIDE]>,
            src: Rav1dPictureDataComponentOffset,
            a_ptrs: &mut [*mut i32; N],
            b_ptrs: &mut [*mut i16; N],
            w: c_int,
            h: c_int,
            bd: BD,
        ) {
            const { assert!(N == 2 || N == 4) };
            let tmp = tmp.0.as_mut_ptr();
            let src_ptr = src.as_ptr::<BD>().cast();
            let src_stride = src.stride();
            let a_ptrs = a_ptrs.as_mut_ptr();
            let b_ptrs = b_ptrs.as_mut_ptr();
            let bd = bd.into_c();
            // SAFETY: asm should be safe.
            unsafe { self.get()(tmp, src_ptr, src_stride, a_ptrs, b_ptrs, w, h, bd) }
        }

        const fn neon1<BD: BitDepth>() -> Self {
            bd_fn!(
                sgr_finish_filter_2rows::decl_fn,
                BD,
                sgr_finish_filter1_2rows,
                neon
            )
        }

        const fn neon2<BD: BitDepth>() -> Self {
            bd_fn!(
                sgr_finish_filter_2rows::decl_fn,
                BD,
                sgr_finish_filter2_2rows,
                neon
            )
        }
    }

    fn sgr_box3_hv_neon<BD: BitDepth>(
        sumsq: &mut [*mut i32; 3],
        sum: &mut [*mut i16; 3],
        aa: *mut i32,
        bb: *mut i16,
        left: Option<&[LeftPixelRow<BD::Pixel>]>,
        src: *const BD::Pixel,
        w: c_int,
        s: c_int,
        edges: LrEdgeFlags,
        bd: BD,
    ) {
        sgr_box_row_h::Fn::neon3::<BD>().call(sumsq[2], sum[2], left, src, w, edges, bd);
        sgr_box3_vert_neon(sumsq, sum, aa, bb, w, s, bd);
    }

    fn sgr_finish1_neon<BD: BitDepth>(
        dst: &mut Rav1dPictureDataComponentOffset,
        a_ptrs: &mut [*mut i32; 3],
        b_ptrs: &mut [*mut i16; 3],
        w: c_int,
        w1: c_int,
        bd: BD,
    ) {
        sgr_finish_weighted1::Fn::neon::<BD>().call(*dst, a_ptrs, b_ptrs, w, w1, bd);
        *dst += dst.pixel_stride::<BD>();
        rotate::<3, 1>(a_ptrs, b_ptrs);
    }

    fn sgr_finish2_neon<BD: BitDepth>(
        dst: &mut Rav1dPictureDataComponentOffset,
        a_ptrs: &mut [*mut i32; 2],
        b_ptrs: &mut [*mut i16; 2],
        w: c_int,
        h: c_int,
        w1: c_int,
        bd: BD,
    ) {
        sgr_finish_weighted2::Fn::neon::<BD>().call(*dst, a_ptrs, b_ptrs, w, h, w1, bd);
        *dst += 2 * dst.pixel_stride::<BD>();
        rotate::<2, 1>(a_ptrs, b_ptrs);
    }

    const FILTER_OUT_STRIDE: usize = 384;

    wrap_fn_ptr!(unsafe extern "C" fn sgr_weighted2(
        dst: *mut DynPixel,
        dst_stride: ptrdiff_t,
        t1: *const i16,
        t2: *const i16,
        w: c_int,
        h: c_int,
        wt: *const i16,
        bitdepth_max: c_int,
    ) -> ());

    impl sgr_weighted2::Fn {
        fn call<BD: BitDepth>(
            &self,
            dst: Rav1dPictureDataComponentOffset,
            t1: &Align16<[i16; 2 * FILTER_OUT_STRIDE]>,
            t2: &Align16<[i16; 2 * FILTER_OUT_STRIDE]>,
            w: c_int,
            h: c_int,
            wt: &[i16; 2],
            bd: BD,
        ) {
            let dst_ptr = dst.as_mut_ptr::<BD>().cast();
            let dst_stride = dst.stride();
            let t1 = t1.0.as_ptr();
            let t2 = t2.0.as_ptr();
            let wt = wt.as_ptr();
            let bd = bd.into_c();
            // SAFETY: asm should be safe.
            unsafe { self.get()(dst_ptr, dst_stride, t1, t2, w, h, wt, bd) }
        }

        const fn neon<BD: BitDepth>() -> Self {
            bd_fn!(sgr_weighted2::decl_fn, BD, sgr_weighted2, neon)
        }
    }

    fn sgr_finish_mix_neon<BD: BitDepth>(
        dst: &mut Rav1dPictureDataComponentOffset,
        a5_ptrs: &mut [*mut i32; 2],
        b5_ptrs: &mut [*mut i16; 2],
        a3_ptrs: &mut [*mut i32; 4],
        b3_ptrs: &mut [*mut i16; 4],
        w: c_int,
        h: c_int,
        w0: c_int,
        w1: c_int,
        bd: BD,
    ) {
        let mut tmp5 = Align16([0; 2 * FILTER_OUT_STRIDE]);
        let mut tmp3 = Align16([0; 2 * FILTER_OUT_STRIDE]);

        sgr_finish_filter_2rows::Fn::neon2::<BD>()
            .call(&mut tmp5, *dst, a5_ptrs, b5_ptrs, w, h, bd);
        sgr_finish_filter_2rows::Fn::neon1::<BD>()
            .call(&mut tmp3, *dst, a3_ptrs, b3_ptrs, w, h, bd);

        let wt = [w0 as i16, w1 as i16];
        sgr_weighted2::Fn::neon::<BD>().call(*dst, &tmp5, &tmp3, w, h, &wt, bd);

        *dst += h as isize * dst.pixel_stride::<BD>();
        rotate::<2, 1>(a5_ptrs, b5_ptrs);
        rotate::<4, 1>(a3_ptrs, b3_ptrs);
    }

    pub fn sgr_filter_3x3_neon<BD: BitDepth>(
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

        let stride = dst.pixel_stride::<BD>();

        const BUF_STRIDE: usize = 384 + 16;

        let mut sumsq_buf = Align16([0; BUF_STRIDE * 3 + 16]);
        let mut sum_buf = Align16([0; BUF_STRIDE * 3 + 16]);

        let mut sumsq_ptrs;
        let mut sum_ptrs;
        let sumsq_rows =
            array::from_fn(|i| sumsq_buf.0[i * BUF_STRIDE..][..BUF_STRIDE].as_mut_ptr());
        let sum_rows = array::from_fn(|i| sum_buf.0[i * BUF_STRIDE..][..BUF_STRIDE].as_mut_ptr());

        let mut a_buf = Align16([0; BUF_STRIDE * 3 + 16]);
        let mut b_buf = Align16([0; BUF_STRIDE * 3 + 16]);

        let mut a_ptrs = array::from_fn(|i| a_buf.0[i * BUF_STRIDE..][..BUF_STRIDE].as_mut_ptr());
        let mut b_ptrs = array::from_fn(|i| b_buf.0[i * BUF_STRIDE..][..BUF_STRIDE].as_mut_ptr());

        let mut src = dst;
        // `lpf` may be negatively out of bounds.
        let mut lpf_bottom = lpf.wrapping_offset(6 * stride);

        #[derive(PartialEq, Eq)]
        enum Track {
            Main,
            Vert1,
            Vert2,
        }
        let mut track = Track::Main;

        let sgr = params.sgr();

        if edges.contains(LrEdgeFlags::TOP) {
            sumsq_ptrs = sumsq_rows;
            sum_ptrs = sum_rows;

            sgr_box_row_h::Fn::neon3::<BD>().call(
                sumsq_rows[0],
                sum_rows[0],
                None,
                lpf,
                w,
                edges,
                bd,
            );
            // `lpf` may be negatively out of bounds.
            lpf = lpf.wrapping_offset(stride);
            sgr_box_row_h::Fn::neon3::<BD>().call(
                sumsq_rows[1],
                sum_rows[1],
                None,
                lpf,
                w,
                edges,
                bd,
            );

            sgr_box3_hv_neon(
                &mut sumsq_ptrs,
                &mut sum_ptrs,
                a_ptrs[2],
                b_ptrs[2],
                Some(left),
                src.as_ptr::<BD>(),
                w,
                sgr.s1 as c_int,
                edges,
                bd,
            );

            left = &left[1..];
            src += stride;
            rotate::<3, 1>(&mut a_ptrs, &mut b_ptrs);

            h -= 1;
            if h <= 0 {
                track = Track::Vert1;
            } else {
                sgr_box3_hv_neon(
                    &mut sumsq_ptrs,
                    &mut sum_ptrs,
                    a_ptrs[2],
                    b_ptrs[2],
                    Some(left),
                    src.as_ptr::<BD>(),
                    w,
                    sgr.s1 as c_int,
                    edges,
                    bd,
                );
                left = &left[1..];
                src += stride;
                rotate::<3, 1>(&mut a_ptrs, &mut b_ptrs);

                h -= 1;
                if h <= 0 {
                    track = Track::Vert2;
                }
            }
        } else {
            sumsq_ptrs = [sumsq_rows[0]; 3];
            sum_ptrs = [sum_rows[0]; 3];

            sgr_box_row_h::Fn::neon3::<BD>().call(
                sumsq_rows[0],
                sum_rows[0],
                Some(left),
                src.as_ptr::<BD>(),
                w,
                edges,
                bd,
            );
            left = &left[1..];
            src += stride;

            sgr_box3_vert_neon(
                &mut sumsq_ptrs,
                &mut sum_ptrs,
                a_ptrs[2],
                b_ptrs[2],
                w,
                sgr.s1 as c_int,
                bd,
            );
            rotate::<3, 1>(&mut a_ptrs, &mut b_ptrs);

            h -= 1;
            if h <= 0 {
                track = Track::Vert1;
            } else {
                sumsq_ptrs[2] = sumsq_rows[1];
                sum_ptrs[2] = sum_rows[1];

                sgr_box3_hv_neon(
                    &mut sumsq_ptrs,
                    &mut sum_ptrs,
                    a_ptrs[2],
                    b_ptrs[2],
                    Some(left),
                    src.as_ptr::<BD>(),
                    w,
                    sgr.s1 as c_int,
                    edges,
                    bd,
                );
                left = &left[1..];
                src += stride;
                rotate::<3, 1>(&mut a_ptrs, &mut b_ptrs);

                h -= 1;
                if h <= 0 {
                    track = Track::Vert2;
                } else {
                    sumsq_ptrs[2] = sumsq_rows[2];
                    sum_ptrs[2] = sum_rows[2];
                }
            }
        }

        // `h > 0` can be true only if `track == Track::Main`.
        // The original C code uses `goto`s and skips over this loop when `h <= 0`.
        while h > 0 {
            sgr_box3_hv_neon(
                &mut sumsq_ptrs,
                &mut sum_ptrs,
                a_ptrs[2],
                b_ptrs[2],
                Some(left),
                src.as_ptr::<BD>(),
                w,
                sgr.s1 as c_int,
                edges,
                bd,
            );
            left = &left[1..];
            src += stride;

            sgr_finish1_neon(&mut dst, &mut a_ptrs, &mut b_ptrs, w, sgr.w1 as c_int, bd);
            h -= 1;
        }

        if track == Track::Main && !edges.contains(LrEdgeFlags::BOTTOM) {
            track = Track::Vert2;
        }

        match track {
            Track::Main => {
                sgr_box3_hv_neon(
                    &mut sumsq_ptrs,
                    &mut sum_ptrs,
                    a_ptrs[2],
                    b_ptrs[2],
                    None,
                    lpf_bottom,
                    w,
                    sgr.s1 as c_int,
                    edges,
                    bd,
                );
                // `lpf` and thus `lpf_bottom` may be negatively out of bounds.
                lpf_bottom = lpf_bottom.wrapping_offset(stride);

                sgr_finish1_neon(&mut dst, &mut a_ptrs, &mut b_ptrs, w, sgr.w1 as c_int, bd);

                sgr_box3_hv_neon(
                    &mut sumsq_ptrs,
                    &mut sum_ptrs,
                    a_ptrs[2],
                    b_ptrs[2],
                    None,
                    lpf_bottom,
                    w,
                    sgr.s1 as c_int,
                    edges,
                    bd,
                );

                sgr_finish1_neon(&mut dst, &mut a_ptrs, &mut b_ptrs, w, sgr.w1 as c_int, bd);
            }
            Track::Vert1 => {
                sumsq_ptrs[2] = sumsq_ptrs[1];
                sum_ptrs[2] = sum_ptrs[1];
                sgr_box3_vert_neon(
                    &mut sumsq_ptrs,
                    &mut sum_ptrs,
                    a_ptrs[2],
                    b_ptrs[2],
                    w,
                    sgr.s1 as c_int,
                    bd,
                );
                rotate::<3, 1>(&mut a_ptrs, &mut b_ptrs);
            }
            Track::Vert2 => {
                sumsq_ptrs[2] = sumsq_ptrs[1];
                sum_ptrs[2] = sum_ptrs[1];
                sgr_box3_vert_neon(
                    &mut sumsq_ptrs,
                    &mut sum_ptrs,
                    a_ptrs[2],
                    b_ptrs[2],
                    w,
                    sgr.s1 as c_int,
                    bd,
                );

                sgr_finish1_neon(&mut dst, &mut a_ptrs, &mut b_ptrs, w, sgr.w1 as c_int, bd);
            }
        }

        if track != Track::Main {
            sumsq_ptrs[2] = sumsq_ptrs[1];
            sum_ptrs[2] = sum_ptrs[1];
            sgr_box3_vert_neon(
                &mut sumsq_ptrs,
                &mut sum_ptrs,
                a_ptrs[2],
                b_ptrs[2],
                w,
                sgr.s1 as c_int,
                bd,
            );

            sgr_finish1_neon(&mut dst, &mut a_ptrs, &mut b_ptrs, w, sgr.w1 as c_int, bd);
        }
    }

    pub fn sgr_filter_5x5_neon<BD: BitDepth>(
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

        let stride = dst.pixel_stride::<BD>();

        const BUF_STRIDE: usize = 384 + 16;

        let mut sumsq_buf = Align16([0; BUF_STRIDE * 5 + 16]);
        let mut sum_buf = Align16([0; BUF_STRIDE * 5 + 16]);

        let mut sumsq_ptrs;
        let mut sum_ptrs;
        let sumsq_rows: [_; 5] =
            array::from_fn(|i| sumsq_buf.0[i * BUF_STRIDE..][..BUF_STRIDE].as_mut_ptr());
        let sum_rows: [_; 5] =
            array::from_fn(|i| sum_buf.0[i * BUF_STRIDE..][..BUF_STRIDE].as_mut_ptr());

        let mut a_buf = Align16([0; BUF_STRIDE * 2 + 16]);
        let mut b_buf = Align16([0; BUF_STRIDE * 2 + 16]);

        let mut a_ptrs = array::from_fn(|i| a_buf.0[i * BUF_STRIDE..][..BUF_STRIDE].as_mut_ptr());
        let mut b_ptrs = array::from_fn(|i| b_buf.0[i * BUF_STRIDE..][..BUF_STRIDE].as_mut_ptr());

        let mut src = dst;
        // `lpf` may be negatively out of bounds.
        let mut lpf_bottom = lpf.wrapping_offset(6 * stride);

        #[derive(PartialEq, Eq)]
        enum Track {
            Main,
            Vert1,
            Vert2,
            Odd,
        }
        let mut track = Track::Main;

        let sgr = params.sgr();

        if edges.contains(LrEdgeFlags::TOP) {
            sumsq_ptrs = array::from_fn(|i| sumsq_rows[if i > 0 { i - 1 } else { 0 }]);
            sum_ptrs = array::from_fn(|i| sum_rows[if i > 0 { i - 1 } else { 0 }]);

            sgr_box_row_h::Fn::neon5::<BD>().call(
                sumsq_rows[0],
                sum_rows[0],
                None,
                lpf,
                w,
                edges,
                bd,
            );
            // `lpf` may be negatively out of bounds.
            lpf = lpf.wrapping_offset(stride);
            sgr_box_row_h::Fn::neon5::<BD>().call(
                sumsq_rows[1],
                sum_rows[1],
                None,
                lpf,
                w,
                edges,
                bd,
            );

            sgr_box_row_h::Fn::neon5::<BD>().call(
                sumsq_rows[2],
                sum_rows[2],
                Some(left),
                src.as_ptr::<BD>(),
                w,
                edges,
                bd,
            );

            left = &left[1..];
            src += stride;

            h -= 1;
            if h <= 0 {
                track = Track::Vert1;
            } else {
                sgr_box_row_h::Fn::neon5::<BD>().call(
                    sumsq_rows[3],
                    sum_rows[3],
                    Some(left),
                    src.as_ptr::<BD>(),
                    w,
                    edges,
                    bd,
                );
                left = &left[1..];
                src += stride;
                sgr_box5_vert_neon(
                    &mut sumsq_ptrs,
                    &mut sum_ptrs,
                    a_ptrs[1],
                    b_ptrs[1],
                    w,
                    sgr.s0 as c_int,
                    bd,
                );
                rotate::<2, 1>(&mut a_ptrs, &mut b_ptrs);

                h -= 1;
                if h <= 0 {
                    track = Track::Vert2;
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

            sgr_box_row_h::Fn::neon5::<BD>().call(
                sumsq_rows[0],
                sum_rows[0],
                Some(left),
                src.as_ptr::<BD>(),
                w,
                edges,
                bd,
            );
            left = &left[1..];
            src += stride;

            h -= 1;
            if h <= 0 {
                track = Track::Vert1;
            } else {
                sumsq_ptrs[4] = sumsq_rows[1];
                sum_ptrs[4] = sum_rows[1];

                sgr_box_row_h::Fn::neon5::<BD>().call(
                    sumsq_rows[1],
                    sum_rows[1],
                    Some(left),
                    src.as_ptr::<BD>(),
                    w,
                    edges,
                    bd,
                );
                left = &left[1..];
                src += stride;

                sgr_box5_vert_neon(
                    &mut sumsq_ptrs,
                    &mut sum_ptrs,
                    a_ptrs[1],
                    b_ptrs[1],
                    w,
                    sgr.s0 as c_int,
                    bd,
                );
                rotate::<2, 1>(&mut a_ptrs, &mut b_ptrs);

                h -= 1;
                if h <= 0 {
                    track = Track::Vert2;
                } else {
                    sumsq_ptrs[3] = sumsq_rows[2];
                    sumsq_ptrs[4] = sumsq_rows[3];
                    sum_ptrs[3] = sum_rows[2];
                    sum_ptrs[4] = sum_rows[3];

                    sgr_box_row_h::Fn::neon5::<BD>().call(
                        sumsq_rows[2],
                        sum_rows[2],
                        Some(left),
                        src.as_ptr::<BD>(),
                        w,
                        edges,
                        bd,
                    );
                    left = &left[1..];
                    src += stride;

                    h -= 1;
                    if h <= 0 {
                        track = Track::Odd;
                    } else {
                        sgr_box_row_h::Fn::neon5::<BD>().call(
                            sumsq_rows[3],
                            sum_rows[3],
                            Some(left),
                            src.as_ptr::<BD>(),
                            w,
                            edges,
                            bd,
                        );
                        left = &left[1..];
                        src += stride;

                        sgr_box5_vert_neon(
                            &mut sumsq_ptrs,
                            &mut sum_ptrs,
                            a_ptrs[1],
                            b_ptrs[1],
                            w,
                            sgr.s0 as c_int,
                            bd,
                        );

                        sgr_finish2_neon(
                            &mut dst,
                            &mut a_ptrs,
                            &mut b_ptrs,
                            w,
                            2,
                            sgr.w0 as c_int,
                            bd,
                        );

                        h -= 1;
                        if h <= 0 {
                            track = Track::Vert2;
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

        // `h > 0` can be true only if `track == Track::Main`.
        // The original C code uses `goto`s and skips over this loop when `h <= 0`.
        while h > 0 {
            sgr_box_row_h::Fn::neon5::<BD>().call(
                sumsq_ptrs[3],
                sum_ptrs[3],
                Some(left),
                src.as_ptr::<BD>(),
                w,
                edges,
                bd,
            );
            left = &left[1..];
            src += stride;

            h -= 1;
            if h <= 0 {
                track = Track::Odd;
            } else {
                sgr_box_row_h::Fn::neon5::<BD>().call(
                    sumsq_ptrs[4],
                    sum_ptrs[4],
                    Some(left),
                    src.as_ptr::<BD>(),
                    w,
                    edges,
                    bd,
                );
                left = &left[1..];
                src += stride;

                sgr_box5_vert_neon(
                    &mut sumsq_ptrs,
                    &mut sum_ptrs,
                    a_ptrs[1],
                    b_ptrs[1],
                    w,
                    sgr.s0 as c_int,
                    bd,
                );
                sgr_finish2_neon(
                    &mut dst,
                    &mut a_ptrs,
                    &mut b_ptrs,
                    w,
                    2,
                    sgr.w0 as c_int,
                    bd,
                );
                h -= 1;
            }
        }

        if track == Track::Main && !edges.contains(LrEdgeFlags::BOTTOM) {
            track = Track::Vert2;
        }

        match track {
            Track::Main => {
                sgr_box_row_h::Fn::neon5::<BD>().call(
                    sumsq_ptrs[3],
                    sum_ptrs[3],
                    None,
                    lpf_bottom,
                    w,
                    edges,
                    bd,
                );
                // `lpf` and thus `lpf_bottom` may be negatively out of bounds.
                lpf_bottom = lpf_bottom.wrapping_offset(stride);
                sgr_box_row_h::Fn::neon5::<BD>().call(
                    sumsq_ptrs[4],
                    sum_ptrs[4],
                    None,
                    lpf_bottom,
                    w,
                    edges,
                    bd,
                );
            }
            Track::Vert1 => {
                // Copy the last row as padding once
                sumsq_ptrs[4] = sumsq_ptrs[3];
                sum_ptrs[4] = sum_ptrs[3];
                sgr_box5_vert_neon(
                    &mut sumsq_ptrs,
                    &mut sum_ptrs,
                    a_ptrs[1],
                    b_ptrs[1],
                    w,
                    sgr.s0 as c_int,
                    bd,
                );
                rotate::<2, 1>(&mut a_ptrs, &mut b_ptrs);
            }
            Track::Vert2 => {
                // Duplicate the last row twice more
                sumsq_ptrs[3] = sumsq_ptrs[2];
                sumsq_ptrs[4] = sumsq_ptrs[2];
                sum_ptrs[3] = sum_ptrs[2];
                sum_ptrs[4] = sum_ptrs[2];
            }
            Track::Odd => {
                // Copy the last row as padding once
                sumsq_ptrs[4] = sumsq_ptrs[3];
                sum_ptrs[4] = sum_ptrs[3];

                sgr_box5_vert_neon(
                    &mut sumsq_ptrs,
                    &mut sum_ptrs,
                    a_ptrs[1],
                    b_ptrs[1],
                    w,
                    sgr.s0 as c_int,
                    bd,
                );
                sgr_finish2_neon(
                    &mut dst,
                    &mut a_ptrs,
                    &mut b_ptrs,
                    w,
                    2,
                    sgr.w0 as c_int,
                    bd,
                );
            }
        }

        match track {
            Track::Main | Track::Vert2 => {
                sgr_box5_vert_neon(
                    &mut sumsq_ptrs,
                    &mut sum_ptrs,
                    a_ptrs[1],
                    b_ptrs[1],
                    w,
                    sgr.s0 as c_int,
                    bd,
                );
                sgr_finish2_neon(
                    &mut dst,
                    &mut a_ptrs,
                    &mut b_ptrs,
                    w,
                    2,
                    sgr.w0 as c_int,
                    bd,
                );
            }
            Track::Odd | Track::Vert1 => {
                // Duplicate the last row twice more
                sumsq_ptrs[3] = sumsq_ptrs[2];
                sumsq_ptrs[4] = sumsq_ptrs[2];
                sum_ptrs[3] = sum_ptrs[2];
                sum_ptrs[4] = sum_ptrs[2];

                sgr_box5_vert_neon(
                    &mut sumsq_ptrs,
                    &mut sum_ptrs,
                    a_ptrs[1],
                    b_ptrs[1],
                    w,
                    sgr.s0 as c_int,
                    bd,
                );
                sgr_finish2_neon(
                    &mut dst,
                    &mut a_ptrs,
                    &mut b_ptrs,
                    w,
                    1,
                    sgr.w0 as c_int,
                    bd,
                );
            }
        }
    }

    pub fn sgr_filter_mix_neon<BD: BitDepth>(
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

        let stride = dst.pixel_stride::<BD>();

        const BUF_STRIDE: usize = 384 + 16;

        let mut sumsq5_buf = Align16([0; BUF_STRIDE * 5 + 16]);
        let mut sum5_buf = Align16([0; BUF_STRIDE * 5 + 16]);

        let sumsq5_rows: [_; 5] =
            array::from_fn(|i| sumsq5_buf.0[i * BUF_STRIDE..][..BUF_STRIDE].as_mut_ptr());
        let sum5_rows: [_; 5] =
            array::from_fn(|i| sum5_buf.0[i * BUF_STRIDE..][..BUF_STRIDE].as_mut_ptr());

        let mut sumsq3_buf = Align16([0; BUF_STRIDE * 3 + 16]);
        let mut sum3_buf = Align16([0; BUF_STRIDE * 3 + 16]);

        let sumsq3_rows: [_; 3] =
            array::from_fn(|i| sumsq3_buf.0[i * BUF_STRIDE..][..BUF_STRIDE].as_mut_ptr());
        let sum3_rows: [_; 3] =
            array::from_fn(|i| sum3_buf.0[i * BUF_STRIDE..][..BUF_STRIDE].as_mut_ptr());

        let mut a5_buf = Align16([0; BUF_STRIDE * 2 + 16]);
        let mut b5_buf = Align16([0; BUF_STRIDE * 2 + 16]);

        let mut a5_ptrs = array::from_fn(|i| a5_buf.0[i * BUF_STRIDE..][..BUF_STRIDE].as_mut_ptr());
        let mut b5_ptrs = array::from_fn(|i| b5_buf.0[i * BUF_STRIDE..][..BUF_STRIDE].as_mut_ptr());

        let mut a3_buf = Align16([0; BUF_STRIDE * 4 + 16]);
        let mut b3_buf = Align16([0; BUF_STRIDE * 4 + 16]);

        let mut a3_ptrs = array::from_fn(|i| a3_buf.0[i * BUF_STRIDE..][..BUF_STRIDE].as_mut_ptr());
        let mut b3_ptrs = array::from_fn(|i| b3_buf.0[i * BUF_STRIDE..][..BUF_STRIDE].as_mut_ptr());

        let mut src = dst;
        // `lpf` may be negatively out of bounds.
        let mut lpf_bottom = lpf.wrapping_offset(6 * stride);

        #[derive(PartialEq, Eq)]
        enum Track {
            Main,
            Vert1,
            Vert2,
            Odd,
        }
        let mut track = Track::Main;

        let lr_have_top = edges.contains(LrEdgeFlags::TOP);

        let mut sumsq3_ptrs = array::from_fn(|i| sumsq3_rows[if lr_have_top { i } else { 0 }]);
        let mut sum3_ptrs = array::from_fn(|i| sum3_rows[if lr_have_top { i } else { 0 }]);

        let mut sumsq5_ptrs =
            array::from_fn(|i| sumsq5_rows[if lr_have_top && i > 0 { i - 1 } else { 0 }]);
        let mut sum5_ptrs =
            array::from_fn(|i| sum5_rows[if lr_have_top && i > 0 { i - 1 } else { 0 }]);

        let sgr = params.sgr();

        if lr_have_top {
            sgr_box35_row_h::Fn::neon::<BD>().call(
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
            // `lpf` may be negatively out of bounds.
            lpf = lpf.wrapping_offset(stride);
            sgr_box35_row_h::Fn::neon::<BD>().call(
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

            sgr_box35_row_h::Fn::neon::<BD>().call(
                sumsq3_rows[2],
                sum3_rows[2],
                sumsq5_rows[2],
                sum5_rows[2],
                Some(left),
                src.as_ptr::<BD>(),
                w,
                edges,
                bd,
            );

            left = &left[1..];
            src += stride;

            sgr_box3_vert_neon(
                &mut sumsq3_ptrs,
                &mut sum3_ptrs,
                a3_ptrs[3],
                b3_ptrs[3],
                w,
                sgr.s1 as c_int,
                bd,
            );
            rotate::<4, 1>(&mut a3_ptrs, &mut b3_ptrs);

            h -= 1;
            if h <= 0 {
                track = Track::Vert1;
            } else {
                sgr_box35_row_h::Fn::neon::<BD>().call(
                    sumsq3_ptrs[2],
                    sum3_ptrs[2],
                    sumsq5_rows[3],
                    sum5_rows[3],
                    Some(left),
                    src.as_ptr::<BD>(),
                    w,
                    edges,
                    bd,
                );
                left = &left[1..];
                src += stride;

                sgr_box5_vert_neon(
                    &mut sumsq5_ptrs,
                    &mut sum5_ptrs,
                    a5_ptrs[1],
                    b5_ptrs[1],
                    w,
                    sgr.s0 as c_int,
                    bd,
                );
                rotate::<2, 1>(&mut a5_ptrs, &mut b5_ptrs);

                sgr_box3_vert_neon(
                    &mut sumsq3_ptrs,
                    &mut sum3_ptrs,
                    a3_ptrs[3],
                    b3_ptrs[3],
                    w,
                    sgr.s1 as c_int,
                    bd,
                );
                rotate::<4, 1>(&mut a3_ptrs, &mut b3_ptrs);

                h -= 1;
                if h <= 0 {
                    track = Track::Vert2;
                } else {
                    // ptrs are rotated by 2; both [3] and [4] now point at rows[0]; set
                    // one of them to point at the previously unused rows[4].
                    sumsq5_ptrs[3] = sumsq5_rows[4];
                    sum5_ptrs[3] = sum5_rows[4];
                }
            }
        } else {
            sgr_box35_row_h::Fn::neon::<BD>().call(
                sumsq3_rows[0],
                sum3_rows[0],
                sumsq5_rows[0],
                sum5_rows[0],
                Some(left),
                src.as_ptr::<BD>(),
                w,
                edges,
                bd,
            );
            left = &left[1..];
            src += stride;

            sgr_box3_vert_neon(
                &mut sumsq3_ptrs,
                &mut sum3_ptrs,
                a3_ptrs[3],
                b3_ptrs[3],
                w,
                sgr.s1 as i32,
                bd,
            );
            rotate::<4, 1>(&mut a3_ptrs, &mut b3_ptrs);

            h -= 1;
            if h <= 0 {
                track = Track::Vert1;
            } else {
                sumsq5_ptrs[4] = sumsq5_rows[1];
                sum5_ptrs[4] = sum5_rows[1];

                sumsq3_ptrs[2] = sumsq3_rows[1];
                sum3_ptrs[2] = sum3_rows[1];

                sgr_box35_row_h::Fn::neon::<BD>().call(
                    sumsq3_rows[1],
                    sum3_rows[1],
                    sumsq5_rows[1],
                    sum5_rows[1],
                    Some(left),
                    src.as_ptr::<BD>(),
                    w,
                    edges,
                    bd,
                );
                left = &left[1..];
                src += stride;

                sgr_box5_vert_neon(
                    &mut sumsq5_ptrs,
                    &mut sum5_ptrs,
                    a5_ptrs[1],
                    b5_ptrs[1],
                    w,
                    sgr.s0 as c_int,
                    bd,
                );
                rotate::<2, 1>(&mut a5_ptrs, &mut b5_ptrs);

                sgr_box3_vert_neon(
                    &mut sumsq3_ptrs,
                    &mut sum3_ptrs,
                    a3_ptrs[3],
                    b3_ptrs[3],
                    w,
                    sgr.s1 as c_int,
                    bd,
                );
                rotate::<4, 1>(&mut a3_ptrs, &mut b3_ptrs);

                h -= 1;
                if h <= 0 {
                    track = Track::Vert2;
                } else {
                    sumsq5_ptrs[3] = sumsq5_rows[2];
                    sumsq5_ptrs[4] = sumsq5_rows[3];
                    sum5_ptrs[3] = sum5_rows[2];
                    sum5_ptrs[4] = sum5_rows[3];

                    sumsq3_ptrs[2] = sumsq3_rows[2];
                    sum3_ptrs[2] = sum3_rows[2];

                    sgr_box35_row_h::Fn::neon::<BD>().call(
                        sumsq3_rows[2],
                        sum3_rows[2],
                        sumsq5_rows[2],
                        sum5_rows[2],
                        Some(left),
                        src.as_ptr::<BD>(),
                        w,
                        edges,
                        bd,
                    );
                    left = &left[1..];
                    src += stride;

                    sgr_box3_vert_neon(
                        &mut sumsq3_ptrs,
                        &mut sum3_ptrs,
                        a3_ptrs[3],
                        b3_ptrs[3],
                        w,
                        sgr.s1 as c_int,
                        bd,
                    );
                    rotate::<4, 1>(&mut a3_ptrs, &mut b3_ptrs);

                    h -= 1;
                    if h <= 0 {
                        track = Track::Odd;
                    } else {
                        sgr_box35_row_h::Fn::neon::<BD>().call(
                            sumsq3_ptrs[2],
                            sum3_ptrs[2],
                            sumsq5_rows[3],
                            sum5_rows[3],
                            Some(left),
                            src.as_ptr::<BD>(),
                            w,
                            edges,
                            bd,
                        );
                        left = &left[1..];
                        src += stride;

                        sgr_box5_vert_neon(
                            &mut sumsq5_ptrs,
                            &mut sum5_ptrs,
                            a5_ptrs[1],
                            b5_ptrs[1],
                            w,
                            sgr.s0 as c_int,
                            bd,
                        );
                        sgr_box3_vert_neon(
                            &mut sumsq3_ptrs,
                            &mut sum3_ptrs,
                            a3_ptrs[3],
                            b3_ptrs[3],
                            w,
                            sgr.s1 as c_int,
                            bd,
                        );
                        sgr_finish_mix_neon(
                            &mut dst,
                            &mut a5_ptrs,
                            &mut b5_ptrs,
                            &mut a3_ptrs,
                            &mut b3_ptrs,
                            w,
                            2,
                            sgr.w0 as c_int,
                            sgr.w1 as c_int,
                            bd,
                        );

                        h -= 1;
                        if h <= 0 {
                            track = Track::Vert2;
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

        // `h > 0` can be true only if `track == Track::Main`.
        // The original C code uses `goto`s and skips over this loop when `h <= 0`.
        while h > 0 {
            sgr_box35_row_h::Fn::neon::<BD>().call(
                sumsq3_ptrs[2],
                sum3_ptrs[2],
                sumsq5_ptrs[3],
                sum5_ptrs[3],
                Some(left),
                src.as_ptr::<BD>(),
                w,
                edges,
                bd,
            );
            left = &left[1..];
            src += stride;

            sgr_box3_vert_neon(
                &mut sumsq3_ptrs,
                &mut sum3_ptrs,
                a3_ptrs[3],
                b3_ptrs[3],
                w,
                sgr.s1 as c_int,
                bd,
            );
            rotate::<4, 1>(&mut a3_ptrs, &mut b3_ptrs);

            h -= 1;
            if h <= 0 {
                track = Track::Odd;
            } else {
                sgr_box35_row_h::Fn::neon::<BD>().call(
                    sumsq3_ptrs[2],
                    sum3_ptrs[2],
                    sumsq5_ptrs[4],
                    sum5_ptrs[4],
                    Some(left),
                    src.as_ptr::<BD>(),
                    w,
                    edges,
                    bd,
                );
                left = &left[1..];
                src += stride;

                sgr_box5_vert_neon(
                    &mut sumsq5_ptrs,
                    &mut sum5_ptrs,
                    a5_ptrs[1],
                    b5_ptrs[1],
                    w,
                    sgr.s0 as c_int,
                    bd,
                );
                sgr_box3_vert_neon(
                    &mut sumsq3_ptrs,
                    &mut sum3_ptrs,
                    a3_ptrs[3],
                    b3_ptrs[3],
                    w,
                    sgr.s1 as c_int,
                    bd,
                );
                sgr_finish_mix_neon(
                    &mut dst,
                    &mut a5_ptrs,
                    &mut b5_ptrs,
                    &mut a3_ptrs,
                    &mut b3_ptrs,
                    w,
                    2,
                    sgr.w0 as c_int,
                    sgr.w1 as c_int,
                    bd,
                );
                h -= 1;
            }
        }

        if track == Track::Main && !edges.contains(LrEdgeFlags::BOTTOM) {
            track = Track::Vert2;
        }

        match track {
            Track::Main => {
                sgr_box35_row_h::Fn::neon::<BD>().call(
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
                // `lpf` and thus `lpf_bottom` may be negatively out of bounds.
                lpf_bottom = lpf_bottom.wrapping_offset(stride);

                sgr_box3_vert_neon(
                    &mut sumsq3_ptrs,
                    &mut sum3_ptrs,
                    a3_ptrs[3],
                    b3_ptrs[3],
                    w,
                    sgr.s1 as c_int,
                    bd,
                );
                rotate::<4, 1>(&mut a3_ptrs, &mut b3_ptrs);

                sgr_box35_row_h::Fn::neon::<BD>().call(
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
            Track::Vert1 => {
                // Copy the last row as padding once
                sumsq5_ptrs[4] = sumsq5_ptrs[3];
                sum5_ptrs[4] = sum5_ptrs[3];

                sumsq3_ptrs[2] = sumsq3_ptrs[1];
                sum3_ptrs[2] = sum3_ptrs[1];

                sgr_box5_vert_neon(
                    &mut sumsq5_ptrs,
                    &mut sum5_ptrs,
                    a5_ptrs[1],
                    b5_ptrs[1],
                    w,
                    sgr.s0 as c_int,
                    bd,
                );
                rotate::<2, 1>(&mut a5_ptrs, &mut b5_ptrs);
                sgr_box3_vert_neon(
                    &mut sumsq3_ptrs,
                    &mut sum3_ptrs,
                    a3_ptrs[3],
                    b3_ptrs[3],
                    w,
                    sgr.s1 as c_int,
                    bd,
                );
                rotate::<4, 1>(&mut a3_ptrs, &mut b3_ptrs);
            }
            Track::Vert2 => {
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
                    a3_ptrs[3],
                    b3_ptrs[3],
                    w,
                    sgr.s1 as c_int,
                    bd,
                );
                rotate::<4, 1>(&mut a3_ptrs, &mut b3_ptrs);

                sumsq3_ptrs[2] = sumsq3_ptrs[1];
                sum3_ptrs[2] = sum3_ptrs[1];
            }
            Track::Odd => {
                // Copy the last row as padding once
                sumsq5_ptrs[4] = sumsq5_ptrs[3];
                sum5_ptrs[4] = sum5_ptrs[3];

                sumsq3_ptrs[2] = sumsq3_ptrs[1];
                sum3_ptrs[2] = sum3_ptrs[1];

                sgr_box5_vert_neon(
                    &mut sumsq5_ptrs,
                    &mut sum5_ptrs,
                    a5_ptrs[1],
                    b5_ptrs[1],
                    w,
                    sgr.s0 as c_int,
                    bd,
                );
                sgr_box3_vert_neon(
                    &mut sumsq3_ptrs,
                    &mut sum3_ptrs,
                    a3_ptrs[3],
                    b3_ptrs[3],
                    w,
                    sgr.s1 as c_int,
                    bd,
                );
                sgr_finish_mix_neon(
                    &mut dst,
                    &mut a5_ptrs,
                    &mut b5_ptrs,
                    &mut a3_ptrs,
                    &mut b3_ptrs,
                    w,
                    2,
                    sgr.w0 as c_int,
                    sgr.w1 as c_int,
                    bd,
                );
            }
        }

        match track {
            Track::Main | Track::Vert2 => {
                sgr_box5_vert_neon(
                    &mut sumsq5_ptrs,
                    &mut sum5_ptrs,
                    a5_ptrs[1],
                    b5_ptrs[1],
                    w,
                    sgr.s0 as c_int,
                    bd,
                );
                sgr_box3_vert_neon(
                    &mut sumsq3_ptrs,
                    &mut sum3_ptrs,
                    a3_ptrs[3],
                    b3_ptrs[3],
                    w,
                    sgr.s1 as c_int,
                    bd,
                );
                sgr_finish_mix_neon(
                    &mut dst,
                    &mut a5_ptrs,
                    &mut b5_ptrs,
                    &mut a3_ptrs,
                    &mut b3_ptrs,
                    w,
                    2,
                    sgr.w0 as c_int,
                    sgr.w1 as c_int,
                    bd,
                );
            }
            Track::Vert1 | Track::Odd => {
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
                    a5_ptrs[1],
                    b5_ptrs[1],
                    w,
                    sgr.s0 as c_int,
                    bd,
                );
                sgr_box3_vert_neon(
                    &mut sumsq3_ptrs,
                    &mut sum3_ptrs,
                    a3_ptrs[3],
                    b3_ptrs[3],
                    w,
                    sgr.s1 as c_int,
                    bd,
                );
                rotate::<4, 1>(&mut a3_ptrs, &mut b3_ptrs);
                // Output only one row
                sgr_finish_mix_neon(
                    &mut dst,
                    &mut a5_ptrs,
                    &mut b5_ptrs,
                    &mut a3_ptrs,
                    &mut b3_ptrs,
                    w,
                    1,
                    sgr.w0 as c_int,
                    sgr.w1 as c_int,
                    bd,
                );
            }
        }
    }
}

#[deny(unsafe_op_in_unsafe_fn)]
#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
mod neon_erased {
    use super::*;

    /// # Safety
    ///
    /// Must be called by [`loop_restoration_filter::Fn::call`].
    #[deny(unsafe_op_in_unsafe_fn)]
    pub unsafe extern "C" fn sgr_filter_5x5_neon_erased<BD: BitDepth>(
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
        _lpf: *const FFISafe<DisjointMut<AlignedVec64<u8>>>,
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
        neon::sgr_filter_5x5_neon(p, left, lpf, w, h, params, edges, bd)
    }

    /// # Safety
    ///
    /// Must be called by [`loop_restoration_filter::Fn::call`].
    #[deny(unsafe_op_in_unsafe_fn)]
    pub unsafe extern "C" fn sgr_filter_3x3_neon_erased<BD: BitDepth>(
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
        _lpf: *const FFISafe<DisjointMut<AlignedVec64<u8>>>,
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
        neon::sgr_filter_3x3_neon(p, left, lpf, w, h, params, edges, bd)
    }

    /// # Safety
    ///
    /// Must be called by [`loop_restoration_filter::Fn::call`].
    #[deny(unsafe_op_in_unsafe_fn)]
    pub unsafe extern "C" fn sgr_filter_mix_neon_erased<BD: BitDepth>(
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
        _lpf: *const FFISafe<DisjointMut<AlignedVec64<u8>>>,
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
        neon::sgr_filter_mix_neon(p, left, lpf, w, h, params, edges, bd)
    }
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
            use neon::*;

            self.wiener[0] = loop_restoration_filter::Fn::new(wiener_filter_neon_erased::<BD>);
            self.wiener[1] = loop_restoration_filter::Fn::new(wiener_filter_neon_erased::<BD>);
        }

        if matches!(BD::BPC, BPC::BPC8) || bpc == 10 {
            use neon_erased::*;

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
