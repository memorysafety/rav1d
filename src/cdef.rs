use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::bitdepth::LeftPixelRow2px;
use crate::include::common::intops::apply_sign;
use crate::include::common::intops::iclip;
use crate::include::common::intops::ulog2;
use crate::src::cpu::CpuFlags;
use crate::src::tables::dav1d_cdef_directions;
use crate::src::wrap_fn_ptr::wrap_fn_ptr;
use bitflags::bitflags;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;

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
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    left: *const LeftPixelRow2px<DynPixel>,
    top: *const DynPixel,
    bottom: *const DynPixel,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    edges: CdefEdgeFlags,
    bitdepth_max: c_int,
) -> ());

impl cdef::Fn {
    /// CDEF operates entirely on pre-filter data.
    /// If bottom/right edges are present (according to `edges`),
    /// then the pre-filter data is located in `dst`.
    /// However, the edge pixels above `dst` may be post-filter,
    /// so in order to get access to pre-filter top pixels, use `top`.
    pub unsafe fn call<BD: BitDepth>(
        &self,
        dst: *mut BD::Pixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<BD::Pixel>,
        top: *const BD::Pixel,
        bottom: *const BD::Pixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bd: BD,
    ) {
        let dst = dst.cast();
        let left = left.cast();
        let top = top.cast();
        let bottom = bottom.cast();
        let bd = bd.into_c();
        self.get()(
            dst,
            stride,
            left,
            top,
            bottom,
            pri_strength,
            sec_strength,
            dir,
            damping,
            edges,
            bd,
        )
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn cdef_dir(
    dst: *const DynPixel,
    dst_stride: ptrdiff_t,
    var: *mut c_uint,
    bitdepth_max: c_int,
) -> c_int);

impl cdef_dir::Fn {
    pub unsafe fn call<BD: BitDepth>(
        &self,
        dst: *const BD::Pixel,
        dst_stride: ptrdiff_t,
        var: *mut c_uint,
        bd: BD,
    ) -> c_int {
        let dst = dst.cast();
        let bd = bd.into_c();
        self.get()(dst, dst_stride, var, bd)
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

#[inline]
pub unsafe fn fill(mut tmp: *mut i16, stride: ptrdiff_t, w: c_int, h: c_int) {
    let mut y = 0;
    while y < h {
        let mut x = 0;
        while x < w {
            *tmp.offset(x as isize) = i16::MIN;
            x += 1;
        }
        tmp = tmp.offset(stride as isize);
        y += 1;
    }
}

unsafe fn padding<BD: BitDepth>(
    mut tmp: *mut i16,
    tmp_stride: ptrdiff_t,
    mut src: *const BD::Pixel,
    src_stride: ptrdiff_t,
    left: *const [BD::Pixel; 2],
    mut top: *const BD::Pixel,
    mut bottom: *const BD::Pixel,
    w: c_int,
    h: c_int,
    edges: CdefEdgeFlags,
) {
    let mut x_start = -(2 as c_int);
    let mut x_end = w + 2;
    let mut y_start = -(2 as c_int);
    let mut y_end = h + 2;
    if !edges.contains(CdefEdgeFlags::HAVE_TOP) {
        fill(
            tmp.offset(-2).offset(-((2 * tmp_stride) as isize)),
            tmp_stride,
            w + 4,
            2 as c_int,
        );
        y_start = 0 as c_int;
    }
    if !edges.contains(CdefEdgeFlags::HAVE_BOTTOM) {
        fill(
            tmp.offset((h as isize * tmp_stride) as isize)
                .offset(-(2 as c_int as isize)),
            tmp_stride,
            w + 4,
            2 as c_int,
        );
        y_end -= 2 as c_int;
    }
    if !edges.contains(CdefEdgeFlags::HAVE_LEFT) {
        fill(
            tmp.offset((y_start as isize * tmp_stride) as isize)
                .offset(-(2 as c_int as isize)),
            tmp_stride,
            2 as c_int,
            y_end - y_start,
        );
        x_start = 0 as c_int;
    }
    if !edges.contains(CdefEdgeFlags::HAVE_RIGHT) {
        fill(
            tmp.offset((y_start as isize * tmp_stride) as isize)
                .offset(w as isize),
            tmp_stride,
            2 as c_int,
            y_end - y_start,
        );
        x_end -= 2 as c_int;
    }
    let mut y = y_start;
    while y < 0 {
        let mut x = x_start;
        while x < x_end {
            *tmp.offset((x as isize + y as isize * tmp_stride) as isize) =
                (*top.offset(x as isize)).as_::<i16>();
            x += 1;
        }
        top = top.offset(BD::pxstride(src_stride));
        y += 1;
    }
    let mut y_0 = 0;
    while y_0 < h {
        let mut x_0 = x_start;
        while x_0 < 0 {
            *tmp.offset((x_0 as isize + y_0 as isize * tmp_stride) as isize) =
                (*left.offset(y_0 as isize))[(2 + x_0) as usize].as_::<i16>();
            x_0 += 1;
        }
        y_0 += 1;
    }
    let mut y_1 = 0;
    while y_1 < h {
        let mut x_1 = if y_1 < h { 0 as c_int } else { x_start };
        while x_1 < x_end {
            *tmp.offset(x_1 as isize) = (*src.offset(x_1 as isize)).as_::<i16>();
            x_1 += 1;
        }
        src = src.offset(BD::pxstride(src_stride));
        tmp = tmp.offset(tmp_stride as isize);
        y_1 += 1;
    }
    let mut y_2 = h;
    while y_2 < y_end {
        let mut x_2 = x_start;
        while x_2 < x_end {
            *tmp.offset(x_2 as isize) = (*bottom.offset(x_2 as isize)).as_::<i16>();
            x_2 += 1;
        }
        bottom = bottom.offset(BD::pxstride(src_stride));
        tmp = tmp.offset(tmp_stride as isize);
        y_2 += 1;
    }
}

#[inline(never)]
unsafe fn cdef_filter_block_c<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: ptrdiff_t,
    left: *const [BD::Pixel; 2],
    top: *const BD::Pixel,
    bottom: *const BD::Pixel,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    w: c_int,
    mut h: c_int,
    edges: CdefEdgeFlags,
    bd: BD,
) {
    let tmp_stride: ptrdiff_t = 12 as c_int as ptrdiff_t;
    if !((w == 4 || w == 8) && (h == 4 || h == 8)) {
        unreachable!();
    }
    let mut tmp_buf: [i16; 144] = [0; 144];
    let mut tmp: *mut i16 = tmp_buf
        .as_mut_ptr()
        .offset((2 * tmp_stride) as isize)
        .offset(2);
    padding::<BD>(
        tmp, tmp_stride, dst, dst_stride, left, top, bottom, w, h, edges,
    );
    if pri_strength != 0 {
        let bitdepth_min_8 = bd.bitdepth().as_::<c_int>() - 8;
        let pri_tap = 4 - (pri_strength >> bitdepth_min_8 & 1);
        let pri_shift = cmp::max(0 as c_int, damping - ulog2(pri_strength as c_uint));
        if sec_strength != 0 {
            let sec_shift = damping - ulog2(sec_strength as c_uint);
            loop {
                let mut x = 0;
                while x < w {
                    let px = (*dst.offset(x as isize)).as_::<c_int>();
                    let mut sum = 0;
                    let mut max = px;
                    let mut min = px;
                    let mut pri_tap_k = pri_tap;
                    let mut k = 0;
                    while k < 2 {
                        let off1 = dav1d_cdef_directions[(dir + 2) as usize][k as usize] as c_int;
                        let p0 = *tmp.offset((x + off1) as isize) as c_int;
                        let p1 = *tmp.offset((x - off1) as isize) as c_int;
                        sum += pri_tap_k * constrain(p0 - px, pri_strength, pri_shift);
                        sum += pri_tap_k * constrain(p1 - px, pri_strength, pri_shift);
                        pri_tap_k = pri_tap_k & 3 | 2;
                        min = cmp::min(p0 as c_uint, min as c_uint) as c_int;
                        max = cmp::max(p0, max);
                        min = cmp::min(p1 as c_uint, min as c_uint) as c_int;
                        max = cmp::max(p1, max);
                        let off2 = dav1d_cdef_directions[(dir + 4) as usize][k as usize] as c_int;
                        let off3 = dav1d_cdef_directions[(dir + 0) as usize][k as usize] as c_int;
                        let s0 = *tmp.offset((x + off2) as isize) as c_int;
                        let s1 = *tmp.offset((x - off2) as isize) as c_int;
                        let s2 = *tmp.offset((x + off3) as isize) as c_int;
                        let s3 = *tmp.offset((x - off3) as isize) as c_int;
                        let sec_tap = 2 - k;
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
                        k += 1;
                    }
                    *dst.offset(x as isize) =
                        iclip(px + (sum - (sum < 0) as c_int + 8 >> 4), min, max)
                            .as_::<BD::Pixel>();
                    x += 1;
                }
                dst = dst.offset(BD::pxstride(dst_stride));
                tmp = tmp.offset(tmp_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        } else {
            loop {
                let mut x_0 = 0;
                while x_0 < w {
                    let px_0 = (*dst.offset(x_0 as isize)).as_::<c_int>();
                    let mut sum_0 = 0;
                    let mut pri_tap_k_0 = pri_tap;
                    let mut k_0 = 0;
                    while k_0 < 2 {
                        let off = dav1d_cdef_directions[(dir + 2) as usize][k_0 as usize] as c_int;
                        let p0_0 = *tmp.offset((x_0 + off) as isize) as c_int;
                        let p1_0 = *tmp.offset((x_0 - off) as isize) as c_int;
                        sum_0 += pri_tap_k_0 * constrain(p0_0 - px_0, pri_strength, pri_shift);
                        sum_0 += pri_tap_k_0 * constrain(p1_0 - px_0, pri_strength, pri_shift);
                        pri_tap_k_0 = pri_tap_k_0 & 3 | 2;
                        k_0 += 1;
                    }
                    *dst.offset(x_0 as isize) =
                        (px_0 + (sum_0 - (sum_0 < 0) as c_int + 8 >> 4)).as_::<BD::Pixel>();
                    x_0 += 1;
                }
                dst = dst.offset(BD::pxstride(dst_stride));
                tmp = tmp.offset(tmp_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        }
    } else {
        if sec_strength == 0 {
            unreachable!();
        }
        let sec_shift_0 = damping - ulog2(sec_strength as c_uint);
        loop {
            let mut x_1 = 0;
            while x_1 < w {
                let px_1 = (*dst.offset(x_1 as isize)).as_::<c_int>();
                let mut sum_1 = 0;
                let mut k_1 = 0;
                while k_1 < 2 {
                    let off1_0 = dav1d_cdef_directions[(dir + 4) as usize][k_1 as usize] as c_int;
                    let off2_0 = dav1d_cdef_directions[(dir + 0) as usize][k_1 as usize] as c_int;
                    let s0_0 = *tmp.offset((x_1 + off1_0) as isize) as c_int;
                    let s1_0 = *tmp.offset((x_1 - off1_0) as isize) as c_int;
                    let s2_0 = *tmp.offset((x_1 + off2_0) as isize) as c_int;
                    let s3_0 = *tmp.offset((x_1 - off2_0) as isize) as c_int;
                    let sec_tap_0 = 2 - k_1;
                    sum_1 += sec_tap_0 * constrain(s0_0 - px_1, sec_strength, sec_shift_0);
                    sum_1 += sec_tap_0 * constrain(s1_0 - px_1, sec_strength, sec_shift_0);
                    sum_1 += sec_tap_0 * constrain(s2_0 - px_1, sec_strength, sec_shift_0);
                    sum_1 += sec_tap_0 * constrain(s3_0 - px_1, sec_strength, sec_shift_0);
                    k_1 += 1;
                }
                *dst.offset(x_1 as isize) =
                    (px_1 + (sum_1 - (sum_1 < 0) as c_int + 8 >> 4)).as_::<BD::Pixel>();
                x_1 += 1;
            }
            dst = dst.offset(BD::pxstride(dst_stride));
            tmp = tmp.offset(tmp_stride as isize);
            h -= 1;
            if !(h != 0) {
                break;
            }
        }
    };
}

unsafe extern "C" fn cdef_filter_block_4x4_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    left: *const LeftPixelRow2px<DynPixel>,
    top: *const DynPixel,
    bottom: *const DynPixel,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    edges: CdefEdgeFlags,
    bitdepth_max: c_int,
) {
    cdef_filter_block_c(
        dst.cast(),
        stride,
        left.cast(),
        top.cast(),
        bottom.cast(),
        pri_strength,
        sec_strength,
        dir,
        damping,
        4 as c_int,
        4 as c_int,
        edges,
        BD::from_c(bitdepth_max),
    );
}

unsafe extern "C" fn cdef_filter_block_4x8_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    left: *const LeftPixelRow2px<DynPixel>,
    top: *const DynPixel,
    bottom: *const DynPixel,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    edges: CdefEdgeFlags,
    bitdepth_max: c_int,
) {
    cdef_filter_block_c(
        dst.cast(),
        stride,
        left.cast(),
        top.cast(),
        bottom.cast(),
        pri_strength,
        sec_strength,
        dir,
        damping,
        4 as c_int,
        8 as c_int,
        edges,
        BD::from_c(bitdepth_max),
    );
}

unsafe extern "C" fn cdef_filter_block_8x8_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    left: *const LeftPixelRow2px<DynPixel>,
    top: *const DynPixel,
    bottom: *const DynPixel,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    edges: CdefEdgeFlags,
    bitdepth_max: c_int,
) {
    cdef_filter_block_c(
        dst.cast(),
        stride,
        left.cast(),
        top.cast(),
        bottom.cast(),
        pri_strength,
        sec_strength,
        dir,
        damping,
        8 as c_int,
        8 as c_int,
        edges,
        BD::from_c(bitdepth_max),
    );
}

unsafe extern "C" fn cdef_find_dir_c_erased<BD: BitDepth>(
    img: *const DynPixel,
    stride: ptrdiff_t,
    var: *mut c_uint,
    bitdepth_max: c_int,
) -> c_int {
    cdef_find_dir_rust(img.cast(), stride, var, BD::from_c(bitdepth_max))
}

unsafe fn cdef_find_dir_rust<BD: BitDepth>(
    mut img: *const BD::Pixel,
    stride: ptrdiff_t,
    var: *mut c_uint,
    bd: BD,
) -> c_int {
    let bitdepth_min_8 = bd.bitdepth().as_::<c_int>() - 8;
    let mut partial_sum_hv: [[c_int; 8]; 2] = [[0 as c_int, 0, 0, 0, 0, 0, 0, 0], [0; 8]];
    let mut partial_sum_diag: [[c_int; 15]; 2] = [
        [0 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0; 15],
    ];
    let mut partial_sum_alt: [[c_int; 11]; 4] = [
        [0 as c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0; 11],
        [0; 11],
        [0; 11],
    ];
    let mut y = 0;
    while y < 8 {
        let mut x = 0;
        while x < 8 {
            let px = ((*img.offset(x as isize)).as_::<c_int>() >> bitdepth_min_8) - 128;
            partial_sum_diag[0][(y + x) as usize] += px;
            partial_sum_alt[0][(y + (x >> 1)) as usize] += px;
            partial_sum_hv[0][y as usize] += px;
            partial_sum_alt[1][(3 + y - (x >> 1)) as usize] += px;
            partial_sum_diag[1][(7 + y - x) as usize] += px;
            partial_sum_alt[2][(3 - (y >> 1) + x) as usize] += px;
            partial_sum_hv[1][x as usize] += px;
            partial_sum_alt[3][((y >> 1) + x) as usize] += px;
            x += 1;
        }
        img = img.offset(BD::pxstride(stride));
        y += 1;
    }
    let mut cost: [c_uint; 8] = [0 as c_int as c_uint, 0, 0, 0, 0, 0, 0, 0];
    let mut n = 0;
    while n < 8 {
        cost[2] = (cost[2]).wrapping_add(
            (partial_sum_hv[0][n as usize] * partial_sum_hv[0][n as usize]) as c_uint,
        );
        cost[6] = (cost[6]).wrapping_add(
            (partial_sum_hv[1][n as usize] * partial_sum_hv[1][n as usize]) as c_uint,
        );
        n += 1;
    }
    cost[2] = (cost[2]).wrapping_mul(105 as c_int as c_uint);
    cost[6] = (cost[6]).wrapping_mul(105 as c_int as c_uint);
    static div_table: [u16; 7] = [840, 420, 280, 210, 168, 140, 120];
    let mut n_0 = 0;
    while n_0 < 7 {
        let d = div_table[n_0 as usize] as c_int;
        cost[0] = (cost[0]).wrapping_add(
            ((partial_sum_diag[0][n_0 as usize] * partial_sum_diag[0][n_0 as usize]
                + partial_sum_diag[0][(14 - n_0) as usize]
                    * partial_sum_diag[0][(14 - n_0) as usize])
                * d) as c_uint,
        );
        cost[4] = (cost[4]).wrapping_add(
            ((partial_sum_diag[1][n_0 as usize] * partial_sum_diag[1][n_0 as usize]
                + partial_sum_diag[1][(14 - n_0) as usize]
                    * partial_sum_diag[1][(14 - n_0) as usize])
                * d) as c_uint,
        );
        n_0 += 1;
    }
    cost[0] =
        (cost[0]).wrapping_add((partial_sum_diag[0][7] * partial_sum_diag[0][7] * 105) as c_uint);
    cost[4] =
        (cost[4]).wrapping_add((partial_sum_diag[1][7] * partial_sum_diag[1][7] * 105) as c_uint);
    let mut n_1 = 0;
    while n_1 < 4 {
        let cost_ptr: *mut c_uint =
            &mut *cost.as_mut_ptr().offset((n_1 * 2 + 1) as isize) as *mut c_uint;
        let mut m = 0;
        while m < 5 {
            *cost_ptr = (*cost_ptr).wrapping_add(
                (partial_sum_alt[n_1 as usize][(3 + m) as usize]
                    * partial_sum_alt[n_1 as usize][(3 + m) as usize]) as c_uint,
            );
            m += 1;
        }
        *cost_ptr = (*cost_ptr).wrapping_mul(105 as c_int as c_uint);
        let mut m_0 = 0;
        while m_0 < 3 {
            let d_0 = div_table[(2 * m_0 + 1) as usize] as c_int;
            *cost_ptr = (*cost_ptr).wrapping_add(
                ((partial_sum_alt[n_1 as usize][m_0 as usize]
                    * partial_sum_alt[n_1 as usize][m_0 as usize]
                    + partial_sum_alt[n_1 as usize][(10 - m_0) as usize]
                        * partial_sum_alt[n_1 as usize][(10 - m_0) as usize])
                    * d_0) as c_uint,
            );
            m_0 += 1;
        }
        n_1 += 1;
    }
    let mut best_dir = 0;
    let mut best_cost: c_uint = cost[0];
    let mut n_2 = 1;
    while n_2 < 8 {
        if cost[n_2 as usize] > best_cost {
            best_cost = cost[n_2 as usize];
            best_dir = n_2;
        }
        n_2 += 1;
    }
    *var = best_cost.wrapping_sub(cost[(best_dir ^ 4 as c_int) as usize]) >> 10;
    return best_dir;
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
wrap_fn_ptr!(unsafe extern "C" fn padding4(
    tmp: *mut u16,
    src: *const DynPixel,
    src_stride: ptrdiff_t,
    left: *const LeftPixelRow2px<DynPixel>,
    top: *const DynPixel,
    bottom: *const DynPixel,
    h: c_int,
    edges: CdefEdgeFlags,
) -> ());

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
wrap_fn_ptr!(unsafe extern "C" fn padding8(
    tmp: *mut u16,
    src: *const DynPixel,
    src_stride: ptrdiff_t,
    left: *const LeftPixelRow2px<DynPixel>,
    top: *const DynPixel,
    bottom: *const DynPixel,
    h: c_int,
    edges: CdefEdgeFlags,
) -> ());

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
wrap_fn_ptr!(unsafe extern "C" fn filter4(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    tmp: *const u16,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    h: c_int,
    edges: usize,
    bitdepth_max: c_int,
) -> ());

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
wrap_fn_ptr!(unsafe extern "C" fn filter8(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    tmp: *const u16,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    h: c_int,
    edges: usize,
    bitdepth_max: c_int,
) -> ());

#[inline(always)]
#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn cdef_filter_8x8_neon_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    left: *const LeftPixelRow2px<DynPixel>,
    top: *const DynPixel,
    bottom: *const DynPixel,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    edges: CdefEdgeFlags,
    bitdepth_max: c_int,
) {
    use crate::src::align::Align16;

    let mut tmp_buf = Align16([0; 200]);
    let tmp = tmp_buf.0.as_mut_ptr().offset(2 * 16).offset(8);
    bd_fn!(padding8::decl_fn, BD, cdef_padding8, neon).get()(
        tmp, dst, stride, left, top, bottom, 8, edges,
    );
    bd_fn!(filter8::decl_fn, BD, cdef_filter8, neon).get()(
        dst,
        stride,
        tmp,
        pri_strength,
        sec_strength,
        dir,
        damping,
        8,
        edges.bits() as usize,
        bitdepth_max,
    );
}

#[inline(always)]
#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn cdef_filter_4x8_neon_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    left: *const LeftPixelRow2px<DynPixel>,
    top: *const DynPixel,
    bottom: *const DynPixel,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    edges: CdefEdgeFlags,
    bitdepth_max: c_int,
) {
    let mut tmp_buf: [u16; 104] = [0; 104];
    let tmp = tmp_buf.as_mut_ptr().offset(2 * 8).offset(8);
    bd_fn!(padding4::decl_fn, BD, cdef_padding4, neon).get()(
        tmp, dst, stride, left, top, bottom, 8, edges,
    );
    bd_fn!(filter4::decl_fn, BD, cdef_filter4, neon).get()(
        dst,
        stride,
        tmp,
        pri_strength,
        sec_strength,
        dir,
        damping,
        8,
        edges.bits() as usize,
        bitdepth_max,
    );
}

#[inline(always)]
#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn cdef_filter_4x4_neon_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    left: *const LeftPixelRow2px<DynPixel>,
    top: *const DynPixel,
    bottom: *const DynPixel,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    edges: CdefEdgeFlags,
    bitdepth_max: c_int,
) {
    let mut tmp_buf = [0; 104];
    let tmp = tmp_buf.as_mut_ptr().offset(2 * 8).offset(8);
    bd_fn!(padding4::decl_fn, BD, cdef_padding4, neon).get()(
        tmp, dst, stride, left, top, bottom, 4, edges,
    );
    bd_fn!(filter4::decl_fn, BD, cdef_filter4, neon).get()(
        dst,
        stride,
        tmp,
        pri_strength,
        sec_strength,
        dir,
        damping,
        4,
        edges.bits() as usize,
        bitdepth_max,
    );
}

impl Rav1dCdefDSPContext {
    pub const fn default<BD: BitDepth>() -> Self {
        Self {
            dir: cdef_dir::Fn::new(cdef_find_dir_c_erased::<BD>),
            fb: [
                cdef::Fn::new(cdef_filter_block_8x8_c_erased::<BD>),
                cdef::Fn::new(cdef_filter_block_4x8_c_erased::<BD>),
                cdef::Fn::new(cdef_filter_block_4x4_c_erased::<BD>),
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
        if !flags.contains(CpuFlags::NEON) {
            return self;
        }

        self.dir = bd_fn!(cdef_dir::decl_fn, BD, cdef_find_dir, neon);
        self.fb[0] = cdef::Fn::new(cdef_filter_8x8_neon_erased::<BD>);
        self.fb[1] = cdef::Fn::new(cdef_filter_4x8_neon_erased::<BD>);
        self.fb[2] = cdef::Fn::new(cdef_filter_4x4_neon_erased::<BD>);

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
