#![deny(unsafe_op_in_unsafe_fn)]

use crate::align::AlignedVec64;
use crate::cpu::CpuFlags;
use crate::disjoint_mut::DisjointMut;
use crate::ffi_safe::FFISafe;
use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::bitdepth::LeftPixelRow2px;
use crate::include::common::intops::apply_sign;
use crate::include::common::intops::iclip;
use crate::include::dav1d::picture::Rav1dPictureDataComponentOffset;
use crate::pic_or_buf::PicOrBuf;
use crate::strided::Strided as _;
use crate::tables::dav1d_cdef_directions;
use crate::with_offset::WithOffset;
use crate::wrap_fn_ptr::wrap_fn_ptr;
use bitflags::bitflags;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ptr;

#[cfg(all(
    feature = "asm",
    not(any(target_arch = "riscv64", target_arch = "riscv32"))
))]
use crate::include::common::bitdepth::bd_fn;

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
use crate::include::common::bitdepth::{bpc_fn, BPC};

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy)]
    pub struct CdefEdgeFlags: u32 {
        const HAVE_LEFT = 1 << 0;
        const HAVE_RIGHT = 1 << 1;
        const HAVE_TOP = 1 << 2;
        const HAVE_BOTTOM = 1 << 3;
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn cdef(
    dst_ptr: *mut DynPixel,
    stride: ptrdiff_t,
    left: *const [LeftPixelRow2px<DynPixel>; 8],
    top_ptr: *const DynPixel,
    bottom_ptr: *const DynPixel,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    edges: CdefEdgeFlags,
    bitdepth_max: c_int,
    _dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
    _top: *const FFISafe<CdefTop>,
    _bottom: *const FFISafe<CdefBottom>,
) -> ());

pub type CdefTop<'a> = WithOffset<&'a DisjointMut<AlignedVec64<u8>>>;
pub type CdefBottom<'a> = WithOffset<PicOrBuf<'a, AlignedVec64<u8>>>;

impl cdef::Fn {
    /// CDEF operates entirely on pre-filter data.
    /// If bottom/right edges are present (according to `edges`),
    /// then the pre-filter data is located in `dst`.
    /// However, the edge pixels above `dst` may be post-filter,
    /// so in order to get access to pre-filter top pixels, use `top`.
    pub fn call<BD: BitDepth>(
        &self,
        dst: Rav1dPictureDataComponentOffset,
        left: &[LeftPixelRow2px<BD::Pixel>; 8],
        top: CdefTop,
        bottom: CdefBottom,
        pri_strength: c_int,
        sec_strength: u8,
        dir: c_int,
        damping: u8,
        edges: CdefEdgeFlags,
        bd: BD,
    ) {
        let dst_ptr = dst.as_mut_ptr::<BD>().cast();
        let stride = dst.stride();
        let left = ptr::from_ref(left).cast();
        let top_ptr = top.as_ptr::<BD>().cast();
        let bottom_ptr = bottom.wrapping_as_ptr::<BD>().cast();
        let top = FFISafe::new(&top);
        let bottom = FFISafe::new(&bottom);
        let sec_strength = sec_strength as c_int;
        let damping = damping as c_int;
        let bd = bd.into_c();
        let dst = FFISafe::new(&dst);
        // SAFETY: Rust fallback is safe, asm is assumed to do the same.
        unsafe {
            self.get()(
                dst_ptr,
                stride,
                left,
                top_ptr,
                bottom_ptr,
                pri_strength,
                sec_strength,
                dir,
                damping,
                edges,
                bd,
                dst,
                top,
                bottom,
            )
        }
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn cdef_dir(
    dst_ptr: *const DynPixel,
    dst_stride: ptrdiff_t,
    variance: &mut c_uint,
    bitdepth_max: c_int,
    _dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
) -> c_int);

impl cdef_dir::Fn {
    pub fn call<BD: BitDepth>(
        &self,
        dst: Rav1dPictureDataComponentOffset,
        variance: &mut c_uint,
        bd: BD,
    ) -> c_int {
        let dst_ptr = dst.as_ptr::<BD>().cast();
        let dst_stride = dst.stride();
        let bd = bd.into_c();
        let dst = FFISafe::new(&dst);
        // SAFETY: Fallback `fn cdef_find_dir_rust` is safe; asm is supposed to do the same.
        unsafe { self.get()(dst_ptr, dst_stride, variance, bd, dst) }
    }
}

pub struct Rav1dCdefDSPContext {
    pub dir: cdef_dir::Fn,

    /// 444/luma, 422, 420
    pub fb: [cdef::Fn; 3],
}

#[inline]
pub fn constrain(diff: c_int, threshold: c_int, shift: c_int) -> c_int {
    let adiff = diff.abs();
    apply_sign(
        cmp::min(adiff, cmp::max(0, threshold - (adiff >> shift))),
        diff,
    )
}

const TMP_STRIDE: usize = 12;

#[inline]
pub fn fill(tmp: &mut [i16], w: usize, h: usize) {
    // Use a value that's a large positive number when interpreted as unsigned,
    // and a large negative number when interpreted as signed.
    for y in 0..h {
        tmp[y * TMP_STRIDE..][..w].fill(i16::MIN);
    }
}

fn padding<BD: BitDepth>(
    tmp: &mut [i16; TMP_STRIDE * TMP_STRIDE],
    src: Rav1dPictureDataComponentOffset,
    left: &[LeftPixelRow2px<BD::Pixel>; 8],
    top: CdefTop,
    bottom: CdefBottom,
    w: usize,
    h: usize,
    edges: CdefEdgeFlags,
) {
    let top = top - 2_usize;
    let bottom = bottom - 2_usize;
    let stride = src.pixel_stride::<BD>();

    // Fill extended input buffer.
    let mut x_start = 2 - 2;
    let mut x_end = w + 2 + 2;
    let mut y_start = 2 - 2;
    let mut y_end = h + 2 + 2;
    if !edges.contains(CdefEdgeFlags::HAVE_TOP) {
        fill(tmp, w + 4, 2);
        y_start += 2;
    }
    if !edges.contains(CdefEdgeFlags::HAVE_BOTTOM) {
        fill(&mut tmp[(h + 2) * TMP_STRIDE..], w + 4, 2);
        y_end -= 2;
    }
    if !edges.contains(CdefEdgeFlags::HAVE_LEFT) {
        fill(&mut tmp[y_start * TMP_STRIDE..], 2, y_end - y_start);
        x_start += 2;
    }
    if !edges.contains(CdefEdgeFlags::HAVE_RIGHT) {
        fill(&mut tmp[y_start * TMP_STRIDE + w + 2..], 2, y_end - y_start);
        x_end -= 2;
    }

    for (i, y) in (y_start..2).enumerate() {
        let top = top + i as isize * stride;
        let top = top.data.slice_as::<_, BD::Pixel>((top.offset.., ..x_end));
        for x in x_start..x_end {
            tmp[x + y * TMP_STRIDE] = top[x].as_::<i16>();
        }
    }
    for y in 0..h {
        for x in x_start..2 {
            tmp[x + (y + 2) * TMP_STRIDE] = left[y][x].as_::<i16>();
        }
    }
    for y in 0..h {
        let tmp = &mut tmp[(y + 2) * TMP_STRIDE..];
        let src = src + (y as isize * stride);
        let src = &*src.slice::<BD>(x_end - 2);
        for x in 2..x_end {
            tmp[x] = src[x - 2].as_::<i16>();
        }
    }
    for (i, y) in (h + 2..y_end).enumerate() {
        let tmp = &mut tmp[y * TMP_STRIDE..];
        let bottom = bottom + i as isize * stride;
        // This is a fallback `fn`, so perf is not as important here, so an extra branch
        // here should be okay.
        let bottom = match bottom.data {
            PicOrBuf::Pic(pic) => &*pic.slice::<BD, _>((bottom.offset.., ..x_end)),
            PicOrBuf::Buf(buf) => &*buf.slice_as((bottom.offset.., ..x_end)),
        };
        for x in x_start..x_end {
            tmp[x] = bottom[x].as_::<i16>();
        }
    }
}

#[inline(never)]
fn cdef_filter_block_rust<BD: BitDepth>(
    dst: Rav1dPictureDataComponentOffset,
    left: &[LeftPixelRow2px<BD::Pixel>; 8],
    top: CdefTop,
    bottom: CdefBottom,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    w: usize,
    h: usize,
    edges: CdefEdgeFlags,
    bd: BD,
) {
    let dir = dir as usize;

    assert!((w == 4 || w == 8) && (h == 4 || h == 8));
    let mut tmp = [0; TMP_STRIDE * TMP_STRIDE]; // `12 * 12` is the maximum value of `TMP_STRIDE * (h + 4)`.

    padding::<BD>(&mut tmp, dst, left, top, bottom, w, h, edges);

    let tmp = tmp;
    let tmp_offset = 2 * TMP_STRIDE + 2;
    let tmp_index = |x: usize, offset: isize| (x + tmp_offset).wrapping_add_signed(offset);

    let dst = |y| {
        let dst = dst + (y as isize * dst.pixel_stride::<BD>());
        dst.slice_mut::<BD>(w)
    };

    if pri_strength != 0 {
        let bitdepth_min_8 = bd.bitdepth() - 8;
        let pri_tap = 4 - (pri_strength >> bitdepth_min_8 & 1);
        let pri_shift = cmp::max(0, damping - pri_strength.ilog2() as c_int);
        if sec_strength != 0 {
            let sec_shift = damping - sec_strength.ilog2() as c_int;
            for y in 0..h {
                let tmp = &tmp[y * TMP_STRIDE..];
                let dst = &mut *dst(y);
                for x in 0..w {
                    let px = dst[x].as_::<c_int>();
                    let mut sum = 0;
                    let mut max = px;
                    let mut min = px;
                    let mut pri_tap_k = pri_tap;
                    for k in 0..2 {
                        let off1 = dav1d_cdef_directions[dir + 2][k] as isize; // dir
                        let p0 = tmp[tmp_index(x, off1)] as c_int;
                        let p1 = tmp[tmp_index(x, -off1)] as c_int;
                        sum += pri_tap_k * constrain(p0 - px, pri_strength, pri_shift);
                        sum += pri_tap_k * constrain(p1 - px, pri_strength, pri_shift);
                        // If `pri_tap_k == 4`, then it becomes 2, else it remains 3.
                        pri_tap_k = pri_tap_k & 3 | 2;
                        min = cmp::min(p0 as c_uint, min as c_uint) as c_int;
                        max = cmp::max(p0, max);
                        min = cmp::min(p1 as c_uint, min as c_uint) as c_int;
                        max = cmp::max(p1, max);
                        let off2 = dav1d_cdef_directions[dir + 4][k] as isize;
                        let off3 = dav1d_cdef_directions[dir + 0][k] as isize;
                        let s0 = tmp[tmp_index(x, off2)] as c_int;
                        let s1 = tmp[tmp_index(x, -off2)] as c_int;
                        let s2 = tmp[tmp_index(x, off3)] as c_int;
                        let s3 = tmp[tmp_index(x, -off3)] as c_int;
                        // `sec_tap` starts at 2 and becomes 1.
                        let sec_tap = 2 - k as c_int;
                        sum += sec_tap * constrain(s0 - px, sec_strength, sec_shift);
                        sum += sec_tap * constrain(s1 - px, sec_strength, sec_shift);
                        sum += sec_tap * constrain(s2 - px, sec_strength, sec_shift);
                        sum += sec_tap * constrain(s3 - px, sec_strength, sec_shift);
                        min = cmp::min(s0 as c_uint, min as c_uint) as c_int;
                        max = cmp::max(s0, max);
                        min = cmp::min(s1 as c_uint, min as c_uint) as c_int;
                        max = cmp::max(s1, max);
                        min = cmp::min(s2 as c_uint, min as c_uint) as c_int;
                        max = cmp::max(s2, max);
                        min = cmp::min(s3 as c_uint, min as c_uint) as c_int;
                        max = cmp::max(s3, max);
                    }
                    dst[x] = iclip(px + (sum - (sum < 0) as c_int + 8 >> 4), min, max)
                        .as_::<BD::Pixel>();
                }
            }
        } else {
            // pri_strength only
            for y in 0..h {
                let tmp = &tmp[y * TMP_STRIDE..];
                let dst = &mut *dst(y);
                for x in 0..w {
                    let px = dst[x].as_::<c_int>();
                    let mut sum = 0;
                    let mut pri_tap_k = pri_tap;
                    for k in 0..2 {
                        let off = dav1d_cdef_directions[dir + 2][k] as isize;
                        let p0 = tmp[tmp_index(x, off)] as c_int;
                        let p1 = tmp[tmp_index(x, -off)] as c_int;
                        sum += pri_tap_k * constrain(p0 - px, pri_strength, pri_shift);
                        sum += pri_tap_k * constrain(p1 - px, pri_strength, pri_shift);
                        pri_tap_k = pri_tap_k & 3 | 2;
                    }
                    dst[x] = (px + (sum - (sum < 0) as c_int + 8 >> 4)).as_::<BD::Pixel>();
                }
            }
        }
    } else {
        // sec_strength only
        let sec_shift = damping - sec_strength.ilog2() as c_int;
        for y in 0..h {
            let tmp = &tmp[y * TMP_STRIDE..];
            let dst = &mut *dst(y);
            for x in 0..w {
                let px = dst[x].as_::<c_int>();
                let mut sum = 0;
                for k in 0..2 {
                    let off1 = dav1d_cdef_directions[dir + 4][k] as isize;
                    let off2 = dav1d_cdef_directions[dir + 0][k] as isize;
                    let s0 = tmp[tmp_index(x, off1)] as c_int;
                    let s1 = tmp[tmp_index(x, -off1)] as c_int;
                    let s2 = tmp[tmp_index(x, off2)] as c_int;
                    let s3 = tmp[tmp_index(x, -off2)] as c_int;
                    let sec_tap = 2 - k as c_int;
                    sum += sec_tap * constrain(s0 - px, sec_strength, sec_shift);
                    sum += sec_tap * constrain(s1 - px, sec_strength, sec_shift);
                    sum += sec_tap * constrain(s2 - px, sec_strength, sec_shift);
                    sum += sec_tap * constrain(s3 - px, sec_strength, sec_shift);
                }
                dst[x] = (px + (sum - (sum < 0) as c_int + 8 >> 4)).as_::<BD::Pixel>();
            }
        }
    };
}

/// # Safety
///
/// Must be called by [`cdef::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn cdef_filter_block_c_erased<BD: BitDepth, const W: usize, const H: usize>(
    _dst_ptr: *mut DynPixel,
    _stride: ptrdiff_t,
    left: *const [LeftPixelRow2px<DynPixel>; 8],
    _top_ptr: *const DynPixel,
    _bottom_ptr: *const DynPixel,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    edges: CdefEdgeFlags,
    bitdepth_max: c_int,
    dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
    top: *const FFISafe<CdefTop>,
    bottom: *const FFISafe<CdefBottom>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `cdef_dir::Fn::call`.
    let dst = *unsafe { FFISafe::get(dst) };
    // SAFETY: Reverse of cast in `cdef::Fn::call`.
    let left = unsafe { &*left.cast() };
    // SAFETY: Was passed as `FFISafe::new(_)` in `cdef::Fn::call`.
    let top = *unsafe { FFISafe::get(top) };
    // SAFETY: Was passed as `FFISafe::new(_)` in `cdef::Fn::call`.
    let bottom = *unsafe { FFISafe::get(bottom) };
    let bd = BD::from_c(bitdepth_max);
    cdef_filter_block_rust(
        dst,
        left,
        top,
        bottom,
        pri_strength,
        sec_strength,
        dir,
        damping,
        W,
        H,
        edges,
        bd,
    )
}

/// # Safety
///
/// Must be called by [`cdef_dir::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn cdef_find_dir_c_erased<BD: BitDepth>(
    _img_ptr: *const DynPixel,
    _stride: ptrdiff_t,
    variance: &mut c_uint,
    bitdepth_max: c_int,
    img: *const FFISafe<Rav1dPictureDataComponentOffset>,
) -> c_int {
    // SAFETY: Was passed as `FFISafe::new(_)` in `cdef_dir::Fn::call`.
    let img = *unsafe { FFISafe::get(img) };
    let bd = BD::from_c(bitdepth_max);
    cdef_find_dir_rust(img, variance, bd)
}

fn cdef_find_dir_rust<BD: BitDepth>(
    img: Rav1dPictureDataComponentOffset,
    variance: &mut c_uint,
    bd: BD,
) -> c_int {
    let bitdepth_min_8 = bd.bitdepth() - 8;
    let mut partial_sum_hv = [[0; 8]; 2];
    let mut partial_sum_diag = [[0; 15]; 2];
    let mut partial_sum_alt = [[0; 11]; 4];

    let (w, h) = (8, 8);
    for y in 0..h {
        let img = img + (y as isize * img.pixel_stride::<BD>());
        let img = &*img.slice::<BD>(w);
        for x in 0..w {
            let px = (img[x].as_::<c_int>() >> bitdepth_min_8) - 128;

            partial_sum_diag[0][y + x] += px;
            partial_sum_alt[0][y + (x >> 1)] += px;
            partial_sum_hv[0][y] += px;
            partial_sum_alt[1][3 + y - (x >> 1)] += px;
            partial_sum_diag[1][7 + y - x] += px;
            partial_sum_alt[2][3 - (y >> 1) + x] += px;
            partial_sum_hv[1][x] += px;
            partial_sum_alt[3][(y >> 1) + x] += px;
        }
    }

    let mut cost = [0; 8];
    for n in 0..8 {
        cost[2] += (partial_sum_hv[0][n] * partial_sum_hv[0][n]) as c_uint;
        cost[6] += (partial_sum_hv[1][n] * partial_sum_hv[1][n]) as c_uint;
    }
    cost[2] *= 105;
    cost[6] *= 105;

    static div_table: [u16; 7] = [840, 420, 280, 210, 168, 140, 120];
    for n in 0..7 {
        let d = div_table[n] as c_int;
        cost[0] += ((partial_sum_diag[0][n] * partial_sum_diag[0][n]
            + partial_sum_diag[0][14 - n] * partial_sum_diag[0][14 - n])
            * d) as c_uint;
        cost[4] += ((partial_sum_diag[1][n] * partial_sum_diag[1][n]
            + partial_sum_diag[1][14 - n] * partial_sum_diag[1][14 - n])
            * d) as c_uint;
    }
    cost[0] += (partial_sum_diag[0][7] * partial_sum_diag[0][7] * 105) as c_uint;
    cost[4] += (partial_sum_diag[1][7] * partial_sum_diag[1][7] * 105) as c_uint;

    for n in 0..4 {
        let cost_ptr = &mut cost[n * 2 + 1];
        for m in 0..5 {
            *cost_ptr += (partial_sum_alt[n][3 + m] * partial_sum_alt[n][3 + m]) as c_uint;
        }
        *cost_ptr *= 105;
        for m in 0..3 {
            let d = div_table[2 * m + 1] as c_int;
            *cost_ptr += ((partial_sum_alt[n][m] * partial_sum_alt[n][m]
                + partial_sum_alt[n][10 - m] * partial_sum_alt[n][10 - m])
                * d) as c_uint;
        }
    }

    let mut best_dir = 0;
    let mut best_cost = cost[0];
    for n in 0..8 {
        if cost[n] > best_cost {
            best_cost = cost[n];
            best_dir = n;
        }
    }

    *variance = (best_cost - cost[best_dir ^ 4]) >> 10;
    best_dir as c_int
}

#[deny(unsafe_op_in_unsafe_fn)]
#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
mod neon {
    use std::mem::MaybeUninit;

    use super::*;

    wrap_fn_ptr!(unsafe extern "C" fn padding(
        tmp: *mut MaybeUninit<u16>,
        src: *const DynPixel,
        src_stride: ptrdiff_t,
        left: *const [LeftPixelRow2px<DynPixel>; 8],
        top: *const DynPixel,
        bottom: *const DynPixel,
        h: c_int,
        edges: CdefEdgeFlags,
    ) -> ());

    impl padding::Fn {
        fn call<BD: BitDepth>(
            &self,
            tmp: &mut [MaybeUninit<u16>],
            src: *const BD::Pixel,
            src_stride: ptrdiff_t,
            left: *const [LeftPixelRow2px<BD::Pixel>; 8],
            top: *const BD::Pixel,
            bottom: *const BD::Pixel,
            h: usize,
            edges: CdefEdgeFlags,
        ) {
            let tmp = tmp.as_mut_ptr();
            let src = src.cast();
            let left = left.cast();
            let top = top.cast();
            let bottom = bottom.cast();
            let h = h as c_int;
            // SAFETY: asm should be safe.
            unsafe { self.get()(tmp, src, src_stride, left, top, bottom, h, edges) }
        }

        const fn neon<BD: BitDepth, const W: usize>() -> Self {
            match W {
                4 => bd_fn!(padding::decl_fn, BD, cdef_padding4, neon),
                8 => bd_fn!(padding::decl_fn, BD, cdef_padding8, neon),
                _ => unreachable!(),
            }
        }
    }

    wrap_fn_ptr!(unsafe extern "C" fn filter(
        dst: *mut DynPixel,
        dst_stride: ptrdiff_t,
        tmp: *const MaybeUninit<u16>,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        h: c_int,
        edges: usize,
        bitdepth_max: c_int,
    ) -> ());

    impl filter::Fn {
        fn call<BD: BitDepth>(
            &self,
            dst: *mut BD::Pixel,
            dst_stride: ptrdiff_t,
            tmp: &[MaybeUninit<u16>],
            pri_strength: c_int,
            sec_strength: c_int,
            dir: c_int,
            damping: c_int,
            h: usize,
            edges: CdefEdgeFlags,
            bd: BD,
        ) {
            let dst = dst.cast();
            let tmp = tmp.as_ptr();
            let h = h as c_int;
            let edges = edges.bits() as usize;
            let bd = bd.into_c();
            // SAFETY: asm should be safe.
            unsafe {
                self.get()(
                    dst,
                    dst_stride,
                    tmp,
                    pri_strength,
                    sec_strength,
                    dir,
                    damping,
                    h,
                    edges,
                    bd,
                )
            }
        }

        const fn neon<BD: BitDepth, const W: usize>() -> Self {
            match W {
                4 => bd_fn!(filter::decl_fn, BD, cdef_filter4, neon),
                8 => bd_fn!(filter::decl_fn, BD, cdef_filter8, neon),
                _ => unreachable!(),
            }
        }
    }

    #[deny(unsafe_op_in_unsafe_fn)]
    pub unsafe extern "C" fn cdef_filter_neon_erased<
        BD: BitDepth,
        const W: usize,
        const H: usize,
        const TMP_STRIDE: usize,
        const TMP_LEN: usize,
    >(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const [LeftPixelRow2px<DynPixel>; 8],
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
        _dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
        _top: *const FFISafe<CdefTop>,
        _bottom: *const FFISafe<CdefBottom>,
    ) {
        use crate::align::Align16;

        let dst = dst.cast();
        let left = left.cast();
        let top = top.cast();
        let bottom = bottom.cast();
        let bd = BD::from_c(bitdepth_max);

        // Use `MaybeUninit` here to avoid over-initialization.
        // C doesn't initialize this either and only partially initializes it in `padding`.
        // Since we're just passing this to a few asm calls that are `unsafe` anyways,
        // initializing this in Rust doesn't really add any extra safety.
        let mut tmp_buf = Align16([MaybeUninit::uninit(); TMP_LEN]);
        let tmp = &mut tmp_buf.0[2 * TMP_STRIDE + 8..];
        padding::Fn::neon::<BD, W>().call::<BD>(tmp, dst, stride, left, top, bottom, H, edges);
        filter::Fn::neon::<BD, W>().call(
            dst,
            stride,
            tmp,
            pri_strength,
            sec_strength,
            dir,
            damping,
            H,
            edges,
            bd,
        );
    }
}

impl Rav1dCdefDSPContext {
    pub const fn default<BD: BitDepth>() -> Self {
        Self {
            dir: cdef_dir::Fn::new(cdef_find_dir_c_erased::<BD>),
            fb: [
                cdef::Fn::new(cdef_filter_block_c_erased::<BD, 8, 8>),
                cdef::Fn::new(cdef_filter_block_c_erased::<BD, 4, 8>),
                cdef::Fn::new(cdef_filter_block_c_erased::<BD, 4, 4>),
            ],
        }
    }

    #[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
    #[inline(always)]
    const fn init_x86<BD: BitDepth>(mut self, flags: CpuFlags) -> Self {
        if matches!(BD::BPC, BPC::BPC8) {
            if !flags.contains(CpuFlags::SSE2) {
                return self;
            }

            self.fb[0] = bpc_fn!(cdef::decl_fn, 8 bpc, cdef_filter_8x8, sse2);
            self.fb[1] = bpc_fn!(cdef::decl_fn, 8 bpc, cdef_filter_4x8, sse2);
            self.fb[2] = bpc_fn!(cdef::decl_fn, 8 bpc, cdef_filter_4x4, sse2);
        }

        if !flags.contains(CpuFlags::SSSE3) {
            return self;
        }

        self.dir = bd_fn!(cdef_dir::decl_fn, BD, cdef_dir, ssse3);
        self.fb[0] = bd_fn!(cdef::decl_fn, BD, cdef_filter_8x8, ssse3);
        self.fb[1] = bd_fn!(cdef::decl_fn, BD, cdef_filter_4x8, ssse3);
        self.fb[2] = bd_fn!(cdef::decl_fn, BD, cdef_filter_4x4, ssse3);

        if !flags.contains(CpuFlags::SSE41) {
            return self;
        }

        self.dir = bd_fn!(cdef_dir::decl_fn, BD, cdef_dir, sse4);
        if matches!(BD::BPC, BPC::BPC8) {
            self.fb[0] = bpc_fn!(cdef::decl_fn, 8 bpc, cdef_filter_8x8, sse4);
            self.fb[1] = bpc_fn!(cdef::decl_fn, 8 bpc, cdef_filter_4x8, sse4);
            self.fb[2] = bpc_fn!(cdef::decl_fn, 8 bpc, cdef_filter_4x4, sse4);
        }

        #[cfg(target_arch = "x86_64")]
        {
            if !flags.contains(CpuFlags::AVX2) {
                return self;
            }

            self.dir = bd_fn!(cdef_dir::decl_fn, BD, cdef_dir, avx2);
            self.fb[0] = bd_fn!(cdef::decl_fn, BD, cdef_filter_8x8, avx2);
            self.fb[1] = bd_fn!(cdef::decl_fn, BD, cdef_filter_4x8, avx2);
            self.fb[2] = bd_fn!(cdef::decl_fn, BD, cdef_filter_4x4, avx2);

            if !flags.contains(CpuFlags::AVX512ICL) {
                return self;
            }

            self.fb[0] = bd_fn!(cdef::decl_fn, BD, cdef_filter_8x8, avx512icl);
            self.fb[1] = bd_fn!(cdef::decl_fn, BD, cdef_filter_4x8, avx512icl);
            self.fb[2] = bd_fn!(cdef::decl_fn, BD, cdef_filter_4x4, avx512icl);
        }

        self
    }

    #[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
    #[inline(always)]
    const fn init_arm<BD: BitDepth>(mut self, flags: CpuFlags) -> Self {
        use self::neon::cdef_filter_neon_erased;

        if !flags.contains(CpuFlags::NEON) {
            return self;
        }

        self.dir = bd_fn!(cdef_dir::decl_fn, BD, cdef_find_dir, neon);
        self.fb[0] = cdef::Fn::new(cdef_filter_neon_erased::<BD, 8, 8, 16, { 12 * 16 + 8 }>);
        self.fb[1] = cdef::Fn::new(cdef_filter_neon_erased::<BD, 4, 8, 8, { 12 * 8 + 8 }>);
        self.fb[2] = cdef::Fn::new(cdef_filter_neon_erased::<BD, 4, 4, 8, { 12 * 8 + 8 }>);

        self
    }

    #[inline(always)]
    const fn init<BD: BitDepth>(self, flags: CpuFlags) -> Self {
        #[cfg(feature = "asm")]
        {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            {
                return self.init_x86::<BD>(flags);
            }
            #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
            {
                return self.init_arm::<BD>(flags);
            }
        }

        #[allow(unreachable_code)] // Reachable on some #[cfg]s.
        {
            let _ = flags;
            self
        }
    }

    pub const fn new<BD: BitDepth>(flags: CpuFlags) -> Self {
        Self::default::<BD>().init::<BD>(flags)
    }
}
