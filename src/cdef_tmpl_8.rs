use crate::include::common::bitdepth::BitDepth8;
use crate::src::cdef::cdef_filter_block_4x4_c_erased;
use crate::src::cdef::cdef_filter_block_4x8_c_erased;
use crate::src::cdef::cdef_filter_block_8x8_c_erased;
use crate::src::cdef::cdef_find_dir_c_erased;
use crate::src::cdef::Rav1dCdefDSPContext;

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
use crate::src::align::Align16;

#[cfg(feature = "asm")]
use cfg_if::cfg_if;

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
    (*c).dir = cdef_find_dir_c_erased::<BitDepth8>;
    (*c).fb[0] = cdef_filter_block_8x8_c_erased::<BitDepth8>;
    (*c).fb[1] = cdef_filter_block_4x8_c_erased::<BitDepth8>;
    (*c).fb[2] = cdef_filter_block_4x4_c_erased::<BitDepth8>;

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            cdef_dsp_init_x86(c);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
            cdef_dsp_init_arm(c);
        }
    }
}
