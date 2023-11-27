use crate::include::common::bitdepth::BitDepth8;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::bitdepth::LeftPixelRow2px;
use crate::include::common::intops::iclip;
use crate::include::common::intops::ulog2;
use crate::src::cdef::constrain;
use crate::src::cdef::padding;
use crate::src::cdef::CdefEdgeFlags;
use crate::src::cdef::Rav1dCdefDSPContext;
use crate::src::tables::dav1d_cdef_directions;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
use crate::src::align::Align16;

#[cfg(feature = "asm")]
use cfg_if::cfg_if;

pub type pixel = u8;

#[inline(never)]
unsafe fn cdef_filter_block_c(
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    left: *const [pixel; 2],
    top: *const pixel,
    bottom: *const pixel,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    w: c_int,
    mut h: c_int,
    edges: CdefEdgeFlags,
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
    padding::<BitDepth8>(
        tmp, tmp_stride, dst, dst_stride, left, top, bottom, w, h, edges,
    );
    if pri_strength != 0 {
        let bitdepth_min_8 = 8 - 8;
        let pri_tap = 4 - (pri_strength >> bitdepth_min_8 & 1);
        let pri_shift = cmp::max(0 as c_int, damping - ulog2(pri_strength as c_uint));
        if sec_strength != 0 {
            let sec_shift = damping - ulog2(sec_strength as c_uint);
            loop {
                let mut x = 0;
                while x < w {
                    let px = *dst.offset(x as isize) as c_int;
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
                        iclip(px + (sum - (sum < 0) as c_int + 8 >> 4), min, max) as pixel;
                    x += 1;
                }
                dst = dst.offset(dst_stride as isize);
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
                    let px_0 = *dst.offset(x_0 as isize) as c_int;
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
                        (px_0 + (sum_0 - (sum_0 < 0) as c_int + 8 >> 4)) as pixel;
                    x_0 += 1;
                }
                dst = dst.offset(dst_stride as isize);
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
                let px_1 = *dst.offset(x_1 as isize) as c_int;
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
                    (px_1 + (sum_1 - (sum_1 < 0) as c_int + 8 >> 4)) as pixel;
                x_1 += 1;
            }
            dst = dst.offset(dst_stride as isize);
            tmp = tmp.offset(tmp_stride as isize);
            h -= 1;
            if !(h != 0) {
                break;
            }
        }
    };
}

unsafe extern "C" fn cdef_filter_block_4x4_c_erased(
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
    _bitdepth_max: c_int,
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
    );
}

unsafe extern "C" fn cdef_filter_block_4x8_c_erased(
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
    _bitdepth_max: c_int,
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
    );
}

unsafe extern "C" fn cdef_filter_block_8x8_c_erased(
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
    _bitdepth_max: c_int,
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
    );
}

unsafe extern "C" fn cdef_find_dir_c_erased(
    img: *const DynPixel,
    stride: ptrdiff_t,
    var: *mut c_uint,
    _bitdepth_max: c_int,
) -> c_int {
    cdef_find_dir_rust(img.cast(), stride, var)
}

unsafe fn cdef_find_dir_rust(mut img: *const pixel, stride: ptrdiff_t, var: *mut c_uint) -> c_int {
    let bitdepth_min_8 = 8 - 8;
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
            let px = (*img.offset(x as isize) as c_int >> bitdepth_min_8) - 128;
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
        img = img.offset(stride as isize);
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

#[cfg(feature = "asm")]
use crate::src::cpu::{rav1d_get_cpu_flags, CpuFlags};

#[inline(always)]
#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64"),))]
unsafe fn cdef_dsp_init_x86(c: *mut Rav1dCdefDSPContext) {
    // TODO(legare): Temporary import until init fns are deduplicated.
    use crate::src::cdef::*;

    let flags = rav1d_get_cpu_flags();

    if !flags.contains(CpuFlags::SSE2) {
        return;
    }

    (*c).fb[0] = dav1d_cdef_filter_8x8_8bpc_sse2;
    (*c).fb[1] = dav1d_cdef_filter_4x8_8bpc_sse2;
    (*c).fb[2] = dav1d_cdef_filter_4x4_8bpc_sse2;

    if !flags.contains(CpuFlags::SSSE3) {
        return;
    }

    (*c).dir = dav1d_cdef_dir_8bpc_ssse3;
    (*c).fb[0] = dav1d_cdef_filter_8x8_8bpc_ssse3;
    (*c).fb[1] = dav1d_cdef_filter_4x8_8bpc_ssse3;
    (*c).fb[2] = dav1d_cdef_filter_4x4_8bpc_ssse3;

    if !flags.contains(CpuFlags::SSE41) {
        return;
    }

    (*c).dir = dav1d_cdef_dir_8bpc_sse4;
    (*c).fb[0] = dav1d_cdef_filter_8x8_8bpc_sse4;
    (*c).fb[1] = dav1d_cdef_filter_4x8_8bpc_sse4;
    (*c).fb[2] = dav1d_cdef_filter_4x4_8bpc_sse4;

    #[cfg(target_arch = "x86_64")]
    {
        if !flags.contains(CpuFlags::AVX2) {
            return;
        }

        (*c).dir = dav1d_cdef_dir_8bpc_avx2;
        (*c).fb[0] = dav1d_cdef_filter_8x8_8bpc_avx2;
        (*c).fb[1] = dav1d_cdef_filter_4x8_8bpc_avx2;
        (*c).fb[2] = dav1d_cdef_filter_4x4_8bpc_avx2;

        if !flags.contains(CpuFlags::AVX512ICL) {
            return;
        }

        (*c).fb[0] = dav1d_cdef_filter_8x8_8bpc_avx512icl;
        (*c).fb[1] = dav1d_cdef_filter_4x8_8bpc_avx512icl;
        (*c).fb[2] = dav1d_cdef_filter_4x4_8bpc_avx512icl;
    }
}

#[inline(always)]
#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
unsafe fn cdef_dsp_init_arm(c: *mut Rav1dCdefDSPContext) {
    // TODO(legare): Temporary import until init fns are deduplicated.
    use crate::src::cdef::*;

    let flags = rav1d_get_cpu_flags();

    if !flags.contains(CpuFlags::NEON) {
        return;
    }

    (*c).dir = dav1d_cdef_find_dir_8bpc_neon;
    (*c).fb[0] = cdef_filter_8x8_neon_erased;
    (*c).fb[1] = cdef_filter_4x8_neon_erased;
    (*c).fb[2] = cdef_filter_4x4_neon_erased;
}

#[inline(always)]
#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
unsafe extern "C" fn cdef_filter_4x4_neon_erased(
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
    _bitdepth_max: c_int,
) {
    // TODO(legare): Temporary import until this fn is deduplicated.
    use crate::src::cdef::*;

    let mut tmp_buf = Align16([0; 104]);
    let tmp = tmp_buf.0.as_mut_ptr().offset(2 * 8).offset(8);
    dav1d_cdef_padding4_8bpc_neon(tmp, dst, stride, left, top, bottom, 4, edges);
    dav1d_cdef_filter4_8bpc_neon(
        dst,
        stride,
        tmp,
        pri_strength,
        sec_strength,
        dir,
        damping,
        4,
        edges as usize,
    );
}

#[inline(always)]
#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
unsafe extern "C" fn cdef_filter_4x8_neon_erased(
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
    _bitdepth_max: c_int,
) {
    // TODO(legare): Temporary import until this fn is deduplicated.
    use crate::src::cdef::*;

    let mut tmp_buf = Align16([0; 104]);
    let tmp = tmp_buf.0.as_mut_ptr().offset(2 * 8).offset(8);
    dav1d_cdef_padding4_8bpc_neon(tmp, dst, stride, left, top, bottom, 8, edges);
    dav1d_cdef_filter4_8bpc_neon(
        dst,
        stride,
        tmp,
        pri_strength,
        sec_strength,
        dir,
        damping,
        8,
        edges as usize,
    );
}

#[inline(always)]
#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
unsafe extern "C" fn cdef_filter_8x8_neon_erased(
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
    _bitdepth_max: c_int,
) {
    // TODO(legare): Temporary import until this fn is deduplicated.
    use crate::src::cdef::*;

    let mut tmp_buf = Align16([0; 200]);
    let tmp = tmp_buf.0.as_mut_ptr().offset(2 * 16).offset(8);
    dav1d_cdef_padding8_8bpc_neon(tmp, dst, stride, left, top, bottom, 8, edges);
    dav1d_cdef_filter8_8bpc_neon(
        dst,
        stride,
        tmp,
        pri_strength,
        sec_strength,
        dir,
        damping,
        8,
        edges as usize,
    );
}

#[cold]
pub unsafe fn rav1d_cdef_dsp_init_8bpc(c: *mut Rav1dCdefDSPContext) {
    (*c).dir = cdef_find_dir_c_erased;
    (*c).fb[0] = cdef_filter_block_8x8_c_erased;
    (*c).fb[1] = cdef_filter_block_4x8_c_erased;
    (*c).fb[2] = cdef_filter_block_4x4_c_erased;

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            cdef_dsp_init_x86(c);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
            cdef_dsp_init_arm(c);
        }
    }
}
