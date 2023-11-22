use crate::include::common::bitdepth::BitDepth8;
use crate::src::loopfilter::loop_filter_h_sb128uv_c_erased;
use crate::src::loopfilter::loop_filter_h_sb128y_c_erased;
use crate::src::loopfilter::loop_filter_v_sb128uv_c_erased;
use crate::src::loopfilter::loop_filter_v_sb128y_c_erased;
use crate::src::loopfilter::Rav1dLoopFilterDSPContext;

#[cfg(feature = "asm")]
use crate::src::cpu::{rav1d_get_cpu_flags, CpuFlags};

#[cfg(feature = "asm")]
use cfg_if::cfg_if;

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
#[inline(always)]
unsafe fn loop_filter_dsp_init_x86(c: *mut Rav1dLoopFilterDSPContext) {
    // TODO(legare): Temporary import until init fns are deduplicated.
    use crate::src::loopfilter::*;

    let flags = rav1d_get_cpu_flags();

    if !flags.contains(CpuFlags::SSSE3) {
        return;
    }

    (*c).loop_filter_sb[0][0] = dav1d_lpf_h_sb_y_8bpc_ssse3;
    (*c).loop_filter_sb[0][1] = dav1d_lpf_v_sb_y_8bpc_ssse3;
    (*c).loop_filter_sb[1][0] = dav1d_lpf_h_sb_uv_8bpc_ssse3;
    (*c).loop_filter_sb[1][1] = dav1d_lpf_v_sb_uv_8bpc_ssse3;

    #[cfg(target_arch = "x86_64")]
    {
        if !flags.contains(CpuFlags::AVX2) {
            return;
        }

        (*c).loop_filter_sb[0][0] = dav1d_lpf_h_sb_y_8bpc_avx2;
        (*c).loop_filter_sb[0][1] = dav1d_lpf_v_sb_y_8bpc_avx2;
        (*c).loop_filter_sb[1][0] = dav1d_lpf_h_sb_uv_8bpc_avx2;
        (*c).loop_filter_sb[1][1] = dav1d_lpf_v_sb_uv_8bpc_avx2;

        if !flags.contains(CpuFlags::AVX512ICL) {
            return;
        }

        (*c).loop_filter_sb[0][0] = dav1d_lpf_h_sb_y_8bpc_avx512icl;
        (*c).loop_filter_sb[0][1] = dav1d_lpf_v_sb_y_8bpc_avx512icl;
        (*c).loop_filter_sb[1][0] = dav1d_lpf_h_sb_uv_8bpc_avx512icl;
        (*c).loop_filter_sb[1][1] = dav1d_lpf_v_sb_uv_8bpc_avx512icl;
    }
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
#[inline(always)]
unsafe fn loop_filter_dsp_init_arm(c: *mut Rav1dLoopFilterDSPContext) {
    // TODO(legare): Temporary import until init fns are deduplicated.
    use crate::src::loopfilter::*;

    let flags = rav1d_get_cpu_flags();

    if !flags.contains(CpuFlags::NEON) {
        return;
    }

    (*c).loop_filter_sb[0][0] = dav1d_lpf_h_sb_y_8bpc_neon;
    (*c).loop_filter_sb[0][1] = dav1d_lpf_v_sb_y_8bpc_neon;
    (*c).loop_filter_sb[1][0] = dav1d_lpf_h_sb_uv_8bpc_neon;
    (*c).loop_filter_sb[1][1] = dav1d_lpf_v_sb_uv_8bpc_neon;
}

#[cold]
pub unsafe fn rav1d_loop_filter_dsp_init_8bpc(c: *mut Rav1dLoopFilterDSPContext) {
    (*c).loop_filter_sb[0][0] = loop_filter_h_sb128y_c_erased::<BitDepth8>;
    (*c).loop_filter_sb[0][1] = loop_filter_v_sb128y_c_erased::<BitDepth8>;
    (*c).loop_filter_sb[1][0] = loop_filter_h_sb128uv_c_erased::<BitDepth8>;
    (*c).loop_filter_sb[1][1] = loop_filter_v_sb128uv_c_erased::<BitDepth8>;

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            loop_filter_dsp_init_x86(c);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
            loop_filter_dsp_init_arm(c);
        }
    }
}
