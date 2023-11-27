use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::BitDepth8;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::bitdepth::LeftPixelRow2px;
use crate::src::cdef::cdef_filter_block_c;
use crate::src::cdef::CdefEdgeFlags;
use crate::src::cdef::Rav1dCdefDSPContext;
use std::ffi::c_int;
use std::ffi::c_uint;

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
use crate::src::align::Align16;

#[cfg(feature = "asm")]
use cfg_if::cfg_if;
use libc::ptrdiff_t;

pub type pixel = u8;

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
    cdef_filter_block_c::<BitDepth8>(
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
        BitDepth8::new(()),
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
