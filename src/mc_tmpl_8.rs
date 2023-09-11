use crate::include::common::bitdepth::BitDepth8;
#[cfg(feature = "asm")]
use cfg_if::cfg_if;

use crate::src::levels::FILTER_2D_8TAP_REGULAR;
use crate::src::levels::FILTER_2D_8TAP_REGULAR_SHARP;
use crate::src::levels::FILTER_2D_8TAP_REGULAR_SMOOTH;
use crate::src::levels::FILTER_2D_8TAP_SHARP;
use crate::src::levels::FILTER_2D_8TAP_SHARP_REGULAR;
use crate::src::levels::FILTER_2D_8TAP_SHARP_SMOOTH;
use crate::src::levels::FILTER_2D_8TAP_SMOOTH;
use crate::src::levels::FILTER_2D_8TAP_SMOOTH_REGULAR;
use crate::src::levels::FILTER_2D_8TAP_SMOOTH_SHARP;
use crate::src::levels::FILTER_2D_BILINEAR;
use crate::src::mc::Dav1dMCDSPContext;

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
#[inline(always)]
unsafe extern "C" fn mc_dsp_init_x86(c: *mut Dav1dMCDSPContext) {
    use crate::src::x86::cpu::*;
    // TODO(legare): Temporary import until init fns are deduplicated.
    use crate::src::mc::*;

    let flags: libc::c_uint = dav1d_get_cpu_flags();

    if flags & DAV1D_X86_CPU_FLAG_SSE2 == 0 {
        return;
    }

    (*c).mct[FILTER_2D_BILINEAR as usize] = Some(dav1d_prep_bilin_8bpc_sse2);
    (*c).mct[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_prep_8tap_regular_8bpc_sse2);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
        Some(dav1d_prep_8tap_regular_smooth_8bpc_sse2);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(dav1d_prep_8tap_regular_sharp_8bpc_sse2);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
        Some(dav1d_prep_8tap_smooth_regular_8bpc_sse2);
    (*c).mct[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_prep_8tap_smooth_8bpc_sse2);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_prep_8tap_smooth_sharp_8bpc_sse2);
    (*c).mct[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(dav1d_prep_8tap_sharp_regular_8bpc_sse2);
    (*c).mct[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_prep_8tap_sharp_smooth_8bpc_sse2);
    (*c).mct[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_prep_8tap_sharp_8bpc_sse2);

    (*c).warp8x8 = Some(dav1d_warp_affine_8x8_8bpc_sse2);
    (*c).warp8x8t = Some(dav1d_warp_affine_8x8t_8bpc_sse2);

    if flags & DAV1D_X86_CPU_FLAG_SSSE3 == 0 {
        return;
    }

    (*c).mc[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_put_8tap_regular_8bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
        Some(dav1d_put_8tap_regular_smooth_8bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(dav1d_put_8tap_regular_sharp_8bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
        Some(dav1d_put_8tap_smooth_regular_8bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_put_8tap_smooth_8bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_put_8tap_smooth_sharp_8bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(dav1d_put_8tap_sharp_regular_8bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_put_8tap_sharp_smooth_8bpc_ssse3);
    (*c).mc[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_put_8tap_sharp_8bpc_ssse3);
    (*c).mc[FILTER_2D_BILINEAR as usize] = Some(dav1d_put_bilin_8bpc_ssse3);

    (*c).mct[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_prep_8tap_regular_8bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
        Some(dav1d_prep_8tap_regular_smooth_8bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
        Some(dav1d_prep_8tap_regular_sharp_8bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
        Some(dav1d_prep_8tap_smooth_regular_8bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_prep_8tap_smooth_8bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_prep_8tap_smooth_sharp_8bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
        Some(dav1d_prep_8tap_sharp_regular_8bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_prep_8tap_sharp_smooth_8bpc_ssse3);
    (*c).mct[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_prep_8tap_sharp_8bpc_ssse3);
    (*c).mct[FILTER_2D_BILINEAR as usize] = Some(dav1d_prep_bilin_8bpc_ssse3);

    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR as usize] =
        Some(dav1d_put_8tap_scaled_regular_8bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
        Some(dav1d_put_8tap_scaled_regular_smooth_8bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
        Some(dav1d_put_8tap_scaled_regular_sharp_8bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
        Some(dav1d_put_8tap_scaled_smooth_regular_8bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_put_8tap_scaled_smooth_8bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
        Some(dav1d_put_8tap_scaled_smooth_sharp_8bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
        Some(dav1d_put_8tap_scaled_sharp_regular_8bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
        Some(dav1d_put_8tap_scaled_sharp_smooth_8bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_put_8tap_scaled_sharp_8bpc_ssse3);
    (*c).mc_scaled[FILTER_2D_BILINEAR as usize] = Some(dav1d_put_bilin_scaled_8bpc_ssse3);

    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR as usize] =
        Some(dav1d_prep_8tap_scaled_regular_8bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
        Some(dav1d_prep_8tap_scaled_regular_smooth_8bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
        Some(dav1d_prep_8tap_scaled_regular_sharp_8bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
        Some(dav1d_prep_8tap_scaled_smooth_regular_8bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH as usize] =
        Some(dav1d_prep_8tap_scaled_smooth_8bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
        Some(dav1d_prep_8tap_scaled_smooth_sharp_8bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
        Some(dav1d_prep_8tap_scaled_sharp_regular_8bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
        Some(dav1d_prep_8tap_scaled_sharp_smooth_8bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_prep_8tap_scaled_sharp_8bpc_ssse3);
    (*c).mct_scaled[FILTER_2D_BILINEAR as usize] = Some(dav1d_prep_bilin_scaled_8bpc_ssse3);

    (*c).avg = Some(dav1d_avg_8bpc_ssse3);
    (*c).w_avg = Some(dav1d_w_avg_8bpc_ssse3);
    (*c).mask = Some(dav1d_mask_8bpc_ssse3);

    (*c).w_mask[0] = Some(dav1d_w_mask_444_8bpc_ssse3);
    (*c).w_mask[1] = Some(dav1d_w_mask_422_8bpc_ssse3);
    (*c).w_mask[2] = Some(dav1d_w_mask_420_8bpc_ssse3);

    (*c).blend = Some(dav1d_blend_8bpc_ssse3);
    (*c).blend_v = Some(dav1d_blend_v_8bpc_ssse3);
    (*c).blend_h = Some(dav1d_blend_h_8bpc_ssse3);
    (*c).warp8x8 = Some(dav1d_warp_affine_8x8_8bpc_ssse3);
    (*c).warp8x8t = Some(dav1d_warp_affine_8x8t_8bpc_ssse3);
    (*c).emu_edge = Some(dav1d_emu_edge_8bpc_ssse3);
    (*c).resize = Some(dav1d_resize_8bpc_ssse3);

    if flags & DAV1D_X86_CPU_FLAG_SSE41 == 0 {
        return;
    }

    (*c).warp8x8 = Some(dav1d_warp_affine_8x8_8bpc_sse4);
    (*c).warp8x8t = Some(dav1d_warp_affine_8x8t_8bpc_sse4);

    #[cfg(target_arch = "x86_64")]
    {
        if flags & DAV1D_X86_CPU_FLAG_AVX2 == 0 {
            return;
        }

        (*c).mc[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_put_8tap_regular_8bpc_avx2);
        (*c).mc[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
            Some(dav1d_put_8tap_regular_smooth_8bpc_avx2);
        (*c).mc[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
            Some(dav1d_put_8tap_regular_sharp_8bpc_avx2);
        (*c).mc[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
            Some(dav1d_put_8tap_smooth_regular_8bpc_avx2);
        (*c).mc[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_put_8tap_smooth_8bpc_avx2);
        (*c).mc[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_put_8tap_smooth_sharp_8bpc_avx2);
        (*c).mc[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
            Some(dav1d_put_8tap_sharp_regular_8bpc_avx2);
        (*c).mc[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_put_8tap_sharp_smooth_8bpc_avx2);
        (*c).mc[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_put_8tap_sharp_8bpc_avx2);
        (*c).mc[FILTER_2D_BILINEAR as usize] = Some(dav1d_put_bilin_8bpc_avx2);

        (*c).mct[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_prep_8tap_regular_8bpc_avx2);
        (*c).mct[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
            Some(dav1d_prep_8tap_regular_smooth_8bpc_avx2);
        (*c).mct[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
            Some(dav1d_prep_8tap_regular_sharp_8bpc_avx2);
        (*c).mct[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
            Some(dav1d_prep_8tap_smooth_regular_8bpc_avx2);
        (*c).mct[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_prep_8tap_smooth_8bpc_avx2);
        (*c).mct[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
            Some(dav1d_prep_8tap_smooth_sharp_8bpc_avx2);
        (*c).mct[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
            Some(dav1d_prep_8tap_sharp_regular_8bpc_avx2);
        (*c).mct[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
            Some(dav1d_prep_8tap_sharp_smooth_8bpc_avx2);
        (*c).mct[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_prep_8tap_sharp_8bpc_avx2);
        (*c).mct[FILTER_2D_BILINEAR as usize] = Some(dav1d_prep_bilin_8bpc_avx2);

        (*c).mc_scaled[FILTER_2D_8TAP_REGULAR as usize] =
            Some(dav1d_put_8tap_scaled_regular_8bpc_avx2);
        (*c).mc_scaled[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
            Some(dav1d_put_8tap_scaled_regular_smooth_8bpc_avx2);
        (*c).mc_scaled[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
            Some(dav1d_put_8tap_scaled_regular_sharp_8bpc_avx2);
        (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
            Some(dav1d_put_8tap_scaled_smooth_regular_8bpc_avx2);
        (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH as usize] =
            Some(dav1d_put_8tap_scaled_smooth_8bpc_avx2);
        (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
            Some(dav1d_put_8tap_scaled_smooth_sharp_8bpc_avx2);
        (*c).mc_scaled[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
            Some(dav1d_put_8tap_scaled_sharp_regular_8bpc_avx2);
        (*c).mc_scaled[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
            Some(dav1d_put_8tap_scaled_sharp_smooth_8bpc_avx2);
        (*c).mc_scaled[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_put_8tap_scaled_sharp_8bpc_avx2);
        (*c).mc_scaled[FILTER_2D_BILINEAR as usize] = Some(dav1d_put_bilin_scaled_8bpc_avx2);

        (*c).mct_scaled[FILTER_2D_8TAP_REGULAR as usize] =
            Some(dav1d_prep_8tap_scaled_regular_8bpc_avx2);
        (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
            Some(dav1d_prep_8tap_scaled_regular_smooth_8bpc_avx2);
        (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
            Some(dav1d_prep_8tap_scaled_regular_sharp_8bpc_avx2);
        (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
            Some(dav1d_prep_8tap_scaled_smooth_regular_8bpc_avx2);
        (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH as usize] =
            Some(dav1d_prep_8tap_scaled_smooth_8bpc_avx2);
        (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
            Some(dav1d_prep_8tap_scaled_smooth_sharp_8bpc_avx2);
        (*c).mct_scaled[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
            Some(dav1d_prep_8tap_scaled_sharp_regular_8bpc_avx2);
        (*c).mct_scaled[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
            Some(dav1d_prep_8tap_scaled_sharp_smooth_8bpc_avx2);
        (*c).mct_scaled[FILTER_2D_8TAP_SHARP as usize] =
            Some(dav1d_prep_8tap_scaled_sharp_8bpc_avx2);
        (*c).mct_scaled[FILTER_2D_BILINEAR as usize] = Some(dav1d_prep_bilin_scaled_8bpc_avx2);

        (*c).avg = Some(dav1d_avg_8bpc_avx2);
        (*c).w_avg = Some(dav1d_w_avg_8bpc_avx2);
        (*c).mask = Some(dav1d_mask_8bpc_avx2);

        (*c).w_mask[0] = Some(dav1d_w_mask_444_8bpc_avx2);
        (*c).w_mask[1] = Some(dav1d_w_mask_422_8bpc_avx2);
        (*c).w_mask[2] = Some(dav1d_w_mask_420_8bpc_avx2);

        (*c).blend = Some(dav1d_blend_8bpc_avx2);
        (*c).blend_v = Some(dav1d_blend_v_8bpc_avx2);
        (*c).blend_h = Some(dav1d_blend_h_8bpc_avx2);
        (*c).warp8x8 = Some(dav1d_warp_affine_8x8_8bpc_avx2);
        (*c).warp8x8t = Some(dav1d_warp_affine_8x8t_8bpc_avx2);
        (*c).emu_edge = Some(dav1d_emu_edge_8bpc_avx2);
        (*c).resize = Some(dav1d_resize_8bpc_avx2);

        if flags & DAV1D_X86_CPU_FLAG_AVX512ICL == 0 {
            return;
        }

        (*c).mc[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_put_8tap_regular_8bpc_avx512icl);
        (*c).mc[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
            Some(dav1d_put_8tap_regular_smooth_8bpc_avx512icl);
        (*c).mc[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
            Some(dav1d_put_8tap_regular_sharp_8bpc_avx512icl);
        (*c).mc[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
            Some(dav1d_put_8tap_smooth_regular_8bpc_avx512icl);
        (*c).mc[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_put_8tap_smooth_8bpc_avx512icl);
        (*c).mc[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
            Some(dav1d_put_8tap_smooth_sharp_8bpc_avx512icl);
        (*c).mc[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
            Some(dav1d_put_8tap_sharp_regular_8bpc_avx512icl);
        (*c).mc[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
            Some(dav1d_put_8tap_sharp_smooth_8bpc_avx512icl);
        (*c).mc[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_put_8tap_sharp_8bpc_avx512icl);
        (*c).mc[FILTER_2D_BILINEAR as usize] = Some(dav1d_put_bilin_8bpc_avx512icl);

        (*c).mct[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_prep_8tap_regular_8bpc_avx512icl);
        (*c).mct[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
            Some(dav1d_prep_8tap_regular_smooth_8bpc_avx512icl);
        (*c).mct[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
            Some(dav1d_prep_8tap_regular_sharp_8bpc_avx512icl);
        (*c).mct[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
            Some(dav1d_prep_8tap_smooth_regular_8bpc_avx512icl);
        (*c).mct[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_prep_8tap_smooth_8bpc_avx512icl);
        (*c).mct[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
            Some(dav1d_prep_8tap_smooth_sharp_8bpc_avx512icl);
        (*c).mct[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
            Some(dav1d_prep_8tap_sharp_regular_8bpc_avx512icl);
        (*c).mct[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
            Some(dav1d_prep_8tap_sharp_smooth_8bpc_avx512icl);
        (*c).mct[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_prep_8tap_sharp_8bpc_avx512icl);
        (*c).mct[FILTER_2D_BILINEAR as usize] = Some(dav1d_prep_bilin_8bpc_avx512icl);

        (*c).avg = Some(dav1d_avg_8bpc_avx512icl);
        (*c).w_avg = Some(dav1d_w_avg_8bpc_avx512icl);
        (*c).mask = Some(dav1d_mask_8bpc_avx512icl);

        (*c).w_mask[0] = Some(dav1d_w_mask_444_8bpc_avx512icl);
        (*c).w_mask[1] = Some(dav1d_w_mask_422_8bpc_avx512icl);
        (*c).w_mask[2] = Some(dav1d_w_mask_420_8bpc_avx512icl);

        (*c).blend = Some(dav1d_blend_8bpc_avx512icl);
        (*c).blend_v = Some(dav1d_blend_v_8bpc_avx512icl);
        (*c).blend_h = Some(dav1d_blend_h_8bpc_avx512icl);
        (*c).warp8x8 = Some(dav1d_warp_affine_8x8_8bpc_avx512icl);
        (*c).warp8x8t = Some(dav1d_warp_affine_8x8t_8bpc_avx512icl);
        (*c).resize = Some(dav1d_resize_8bpc_avx512icl);
    }
}

#[cfg(feature = "asm")]
use crate::src::cpu::dav1d_get_cpu_flags;

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
#[inline(always)]
unsafe extern "C" fn mc_dsp_init_arm(c: *mut Dav1dMCDSPContext) {
    use crate::src::arm::cpu::DAV1D_ARM_CPU_FLAG_NEON;
    // TODO(legare): Temporary import until init fns are deduplicated.
    use crate::src::mc::*;

    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
        return;
    }

    (*c).mc[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_put_8tap_regular_8bpc_neon);
    (*c).mc[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] = Some(dav1d_put_8tap_regular_smooth_8bpc_neon);
    (*c).mc[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(dav1d_put_8tap_regular_sharp_8bpc_neon);
    (*c).mc[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] = Some(dav1d_put_8tap_smooth_regular_8bpc_neon);
    (*c).mc[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_put_8tap_smooth_8bpc_neon);
    (*c).mc[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_put_8tap_smooth_sharp_8bpc_neon);
    (*c).mc[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(dav1d_put_8tap_sharp_regular_8bpc_neon);
    (*c).mc[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_put_8tap_sharp_smooth_8bpc_neon);
    (*c).mc[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_put_8tap_sharp_8bpc_neon);
    (*c).mc[FILTER_2D_BILINEAR as usize] = Some(dav1d_put_bilin_8bpc_neon);

    (*c).mct[FILTER_2D_8TAP_REGULAR as usize] = Some(dav1d_prep_8tap_regular_8bpc_neon);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
        Some(dav1d_prep_8tap_regular_smooth_8bpc_neon);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SHARP as usize] = Some(dav1d_prep_8tap_regular_sharp_8bpc_neon);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
        Some(dav1d_prep_8tap_smooth_regular_8bpc_neon);
    (*c).mct[FILTER_2D_8TAP_SMOOTH as usize] = Some(dav1d_prep_8tap_smooth_8bpc_neon);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_SHARP as usize] = Some(dav1d_prep_8tap_smooth_sharp_8bpc_neon);
    (*c).mct[FILTER_2D_8TAP_SHARP_REGULAR as usize] = Some(dav1d_prep_8tap_sharp_regular_8bpc_neon);
    (*c).mct[FILTER_2D_8TAP_SHARP_SMOOTH as usize] = Some(dav1d_prep_8tap_sharp_smooth_8bpc_neon);
    (*c).mct[FILTER_2D_8TAP_SHARP as usize] = Some(dav1d_prep_8tap_sharp_8bpc_neon);
    (*c).mct[FILTER_2D_BILINEAR as usize] = Some(dav1d_prep_bilin_8bpc_neon);

    (*c).avg = Some(dav1d_avg_8bpc_neon);
    (*c).w_avg = Some(dav1d_w_avg_8bpc_neon);
    (*c).mask = Some(dav1d_mask_8bpc_neon);
    (*c).blend = Some(dav1d_blend_8bpc_neon);
    (*c).blend_h = Some(dav1d_blend_h_8bpc_neon);
    (*c).blend_v = Some(dav1d_blend_v_8bpc_neon);

    (*c).w_mask[0] = Some(dav1d_w_mask_444_8bpc_neon);
    (*c).w_mask[1] = Some(dav1d_w_mask_422_8bpc_neon);
    (*c).w_mask[2] = Some(dav1d_w_mask_420_8bpc_neon);

    (*c).warp8x8 = Some(dav1d_warp_affine_8x8_8bpc_neon);
    (*c).warp8x8t = Some(dav1d_warp_affine_8x8t_8bpc_neon);
    (*c).emu_edge = Some(dav1d_emu_edge_8bpc_neon);
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_mc_dsp_init_8bpc(c: *mut Dav1dMCDSPContext) {
    // TODO(legare): Temporary import until init fns are deduplicated.
    use crate::src::mc::*;

    (*c).mc[FILTER_2D_8TAP_REGULAR as usize] = Some(put_8tap_regular_c_erased::<BitDepth8>);
    (*c).mc[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
        Some(put_8tap_regular_smooth_c_erased::<BitDepth8>);
    (*c).mc[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
        Some(put_8tap_regular_sharp_c_erased::<BitDepth8>);
    (*c).mc[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
        Some(put_8tap_sharp_regular_c_erased::<BitDepth8>);
    (*c).mc[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
        Some(put_8tap_sharp_smooth_c_erased::<BitDepth8>);
    (*c).mc[FILTER_2D_8TAP_SHARP as usize] = Some(put_8tap_sharp_c_erased::<BitDepth8>);
    (*c).mc[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
        Some(put_8tap_smooth_regular_c_erased::<BitDepth8>);
    (*c).mc[FILTER_2D_8TAP_SMOOTH as usize] = Some(put_8tap_smooth_c_erased::<BitDepth8>);
    (*c).mc[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
        Some(put_8tap_smooth_sharp_c_erased::<BitDepth8>);
    (*c).mc[FILTER_2D_BILINEAR as usize] = Some(put_bilin_c_erased::<BitDepth8>);

    (*c).mct[FILTER_2D_8TAP_REGULAR as usize] = Some(prep_8tap_regular_c_erased::<BitDepth8>);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
        Some(prep_8tap_regular_smooth_c_erased::<BitDepth8>);
    (*c).mct[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
        Some(prep_8tap_regular_sharp_c_erased::<BitDepth8>);
    (*c).mct[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
        Some(prep_8tap_sharp_regular_c_erased::<BitDepth8>);
    (*c).mct[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
        Some(prep_8tap_sharp_smooth_c_erased::<BitDepth8>);
    (*c).mct[FILTER_2D_8TAP_SHARP as usize] = Some(prep_8tap_sharp_c_erased::<BitDepth8>);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
        Some(prep_8tap_smooth_regular_c_erased::<BitDepth8>);
    (*c).mct[FILTER_2D_8TAP_SMOOTH as usize] = Some(prep_8tap_smooth_c_erased::<BitDepth8>);
    (*c).mct[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
        Some(prep_8tap_smooth_sharp_c_erased::<BitDepth8>);
    (*c).mct[FILTER_2D_BILINEAR as usize] = Some(prep_bilin_c_erased::<BitDepth8>);

    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR as usize] =
        Some(put_8tap_regular_scaled_c_erased::<BitDepth8>);
    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
        Some(put_8tap_regular_smooth_scaled_c_erased::<BitDepth8>);
    (*c).mc_scaled[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
        Some(put_8tap_regular_sharp_scaled_c_erased::<BitDepth8>);
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
        Some(put_8tap_sharp_regular_scaled_c_erased::<BitDepth8>);
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
        Some(put_8tap_sharp_smooth_scaled_c_erased::<BitDepth8>);
    (*c).mc_scaled[FILTER_2D_8TAP_SHARP as usize] =
        Some(put_8tap_sharp_scaled_c_erased::<BitDepth8>);
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
        Some(put_8tap_smooth_regular_scaled_c_erased::<BitDepth8>);
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH as usize] =
        Some(put_8tap_smooth_scaled_c_erased::<BitDepth8>);
    (*c).mc_scaled[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
        Some(put_8tap_smooth_sharp_scaled_c_erased::<BitDepth8>);
    (*c).mc_scaled[FILTER_2D_BILINEAR as usize] = Some(put_bilin_scaled_c_erased::<BitDepth8>);

    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR as usize] =
        Some(prep_8tap_regular_scaled_c_erased::<BitDepth8>);
    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SMOOTH as usize] =
        Some(prep_8tap_regular_smooth_scaled_c_erased::<BitDepth8>);
    (*c).mct_scaled[FILTER_2D_8TAP_REGULAR_SHARP as usize] =
        Some(prep_8tap_regular_sharp_scaled_c_erased::<BitDepth8>);
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP_REGULAR as usize] =
        Some(prep_8tap_sharp_regular_scaled_c_erased::<BitDepth8>);
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP_SMOOTH as usize] =
        Some(prep_8tap_sharp_smooth_scaled_c_erased::<BitDepth8>);
    (*c).mct_scaled[FILTER_2D_8TAP_SHARP as usize] =
        Some(prep_8tap_sharp_scaled_c_erased::<BitDepth8>);
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH_REGULAR as usize] =
        Some(prep_8tap_smooth_regular_scaled_c_erased::<BitDepth8>);
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH as usize] =
        Some(prep_8tap_smooth_scaled_c_erased::<BitDepth8>);
    (*c).mct_scaled[FILTER_2D_8TAP_SMOOTH_SHARP as usize] =
        Some(prep_8tap_smooth_sharp_scaled_c_erased::<BitDepth8>);
    (*c).mct_scaled[FILTER_2D_BILINEAR as usize] = Some(prep_bilin_scaled_c_erased::<BitDepth8>);

    (*c).avg = Some(avg_c_erased::<BitDepth8>);
    (*c).w_avg = Some(w_avg_c_erased::<BitDepth8>);
    (*c).mask = Some(mask_c_erased::<BitDepth8>);

    (*c).w_mask[0 as usize] = Some(w_mask_444_c_erased::<BitDepth8>);
    (*c).w_mask[1 as usize] = Some(w_mask_422_c_erased::<BitDepth8>);
    (*c).w_mask[2 as usize] = Some(w_mask_420_c_erased::<BitDepth8>);

    (*c).blend = Some(blend_c_erased::<BitDepth8>);
    (*c).blend_v = Some(blend_v_c_erased::<BitDepth8>);
    (*c).blend_h = Some(blend_h_c_erased::<BitDepth8>);
    (*c).warp8x8 = Some(warp_affine_8x8_c_erased::<BitDepth8>);
    (*c).warp8x8t = Some(warp_affine_8x8t_c_erased::<BitDepth8>);
    (*c).emu_edge = Some(emu_edge_c_erased::<BitDepth8>);
    (*c).resize = Some(resize_c_erased::<BitDepth8>);

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            mc_dsp_init_x86(c);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
            mc_dsp_init_arm(c);
        }
    }
}
