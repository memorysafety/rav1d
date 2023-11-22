use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::BitDepth8;
use crate::include::common::bitdepth::DynPixel;
use crate::src::lf_mask::Av1FilterLUT;
use crate::src::loopfilter::loop_filter_h_sb128uv_c_erased;
use crate::src::loopfilter::loop_filter_h_sb128y_c_erased;
use crate::src::loopfilter::loop_filter_v_sb128uv_c_erased;
use crate::src::loopfilter::loop_filter_v_sb128y_rust;
use crate::src::loopfilter::Rav1dLoopFilterDSPContext;
use libc::ptrdiff_t;
use std::ffi::c_int;

#[cfg(feature = "asm")]
use crate::src::cpu::{rav1d_get_cpu_flags, CpuFlags};

#[cfg(feature = "asm")]
use cfg_if::cfg_if;

unsafe extern "C" fn loop_filter_v_sb128y_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    vmask: *const u32,
    l: *const [u8; 4],
    b4_stride: ptrdiff_t,
    lut: *const Av1FilterLUT,
    w: c_int,
    _bitdepth_max: c_int,
) {
    loop_filter_v_sb128y_rust(
        dst.cast(),
        stride,
        vmask,
        l,
        b4_stride,
        lut,
        w,
        BitDepth8::new(()),
    );
}

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
    (*c).loop_filter_sb[0][1] = loop_filter_v_sb128y_c_erased;
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
