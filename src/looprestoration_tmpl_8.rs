use crate::include::common::bitdepth::BitDepth8;
use crate::include::stdint::*;
use ::libc;
#[cfg(feature = "asm")]
use cfg_if::cfg_if;

pub type pixel = uint8_t;
pub type coef = int16_t;
pub type const_left_pixel_row = *const [pixel; 4];
use crate::src::looprestoration::sgr_3x3_c_erased;
use crate::src::looprestoration::sgr_5x5_c_erased;
use crate::src::looprestoration::sgr_mix_c_erased;
use crate::src::looprestoration::wiener_c_erased;
use crate::src::looprestoration::Dav1dLoopRestorationDSPContext;

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
#[inline(always)]
unsafe extern "C" fn loop_restoration_dsp_init_x86(
    c: *mut Dav1dLoopRestorationDSPContext,
    _bpc: libc::c_int,
) {
    // TODO(randomPoison): Import temporarily needed until init fns are deduplicated.
    use crate::src::looprestoration::*;
    use crate::src::x86::cpu::*;

    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_X86_CPU_FLAG_SSE2 == 0 {
        return;
    }

    (*c).wiener[0] = dav1d_wiener_filter7_8bpc_sse2;
    (*c).wiener[1] = dav1d_wiener_filter5_8bpc_sse2;

    if flags & DAV1D_X86_CPU_FLAG_SSSE3 == 0 {
        return;
    }

    (*c).wiener[0] = dav1d_wiener_filter7_8bpc_ssse3;
    (*c).wiener[1] = dav1d_wiener_filter5_8bpc_ssse3;

    (*c).sgr[0] = dav1d_sgr_filter_5x5_8bpc_ssse3;
    (*c).sgr[1] = dav1d_sgr_filter_3x3_8bpc_ssse3;
    (*c).sgr[2] = dav1d_sgr_filter_mix_8bpc_ssse3;

    #[cfg(target_arch = "x86_64")]
    {
        if flags & DAV1D_X86_CPU_FLAG_AVX2 == 0 {
            return;
        }

        (*c).wiener[0] = dav1d_wiener_filter7_8bpc_avx2;
        (*c).wiener[1] = dav1d_wiener_filter5_8bpc_avx2;

        (*c).sgr[0] = dav1d_sgr_filter_5x5_8bpc_avx2;
        (*c).sgr[1] = dav1d_sgr_filter_3x3_8bpc_avx2;
        (*c).sgr[2] = dav1d_sgr_filter_mix_8bpc_avx2;

        if flags & DAV1D_X86_CPU_FLAG_AVX512ICL == 0 {
            return;
        }

        (*c).wiener[0] = dav1d_wiener_filter7_8bpc_avx512icl;
        (*c).wiener[1] = (*c).wiener[0];

        (*c).sgr[0] = dav1d_sgr_filter_5x5_8bpc_avx512icl;
        (*c).sgr[1] = dav1d_sgr_filter_3x3_8bpc_avx512icl;
        (*c).sgr[2] = dav1d_sgr_filter_mix_8bpc_avx512icl;
    }
}

#[cfg(feature = "asm")]
use crate::src::cpu::dav1d_get_cpu_flags;

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
#[inline(always)]
unsafe extern "C" fn loop_restoration_dsp_init_arm(
    c: *mut Dav1dLoopRestorationDSPContext,
    mut _bpc: libc::c_int,
) {
    use crate::src::arm::cpu::DAV1D_ARM_CPU_FLAG_NEON;
    // TODO(randomPoison): Import temporarily needed until init fns are deduplicated.
    use crate::src::looprestoration::*;

    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
        return;
    }

    cfg_if! {
        if #[cfg(target_arch = "aarch64")] {
            (*c).wiener[0] = dav1d_wiener_filter7_8bpc_neon;
            (*c).wiener[1] = dav1d_wiener_filter5_8bpc_neon;
        } else {
            (*c).wiener[0] = wiener_filter_neon_erased::<BitDepth8>;
            (*c).wiener[1] = wiener_filter_neon_erased::<BitDepth8>;
        }
    }

    (*c).sgr[0] = sgr_filter_5x5_neon_erased::<BitDepth8>;
    (*c).sgr[1] = sgr_filter_3x3_neon_erased::<BitDepth8>;
    (*c).sgr[2] = sgr_filter_mix_neon_erased::<BitDepth8>;
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_loop_restoration_dsp_init_8bpc(
    c: *mut Dav1dLoopRestorationDSPContext,
    _bpc: libc::c_int,
) {
    (*c).wiener[1] = wiener_c_erased::<BitDepth8>;
    (*c).wiener[0] = (*c).wiener[1];
    (*c).sgr[0] = sgr_5x5_c_erased::<BitDepth8>;
    (*c).sgr[1] = sgr_3x3_c_erased::<BitDepth8>;
    (*c).sgr[2] = sgr_mix_c_erased::<BitDepth8>;

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            loop_restoration_dsp_init_x86(c, _bpc);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]{
            loop_restoration_dsp_init_arm(c, _bpc);
        }
    }
}
