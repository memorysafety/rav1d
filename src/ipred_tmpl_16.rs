use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::BitDepth16;
use crate::include::common::bitdepth::DynPixel;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::src::ipred::cfl_ac_rust;
use crate::src::ipred::ipred_cfl_128_c_erased;
use crate::src::ipred::ipred_cfl_c_erased;
use crate::src::ipred::ipred_cfl_left_c_erased;
use crate::src::ipred::ipred_cfl_top_c_erased;
use crate::src::ipred::ipred_dc_128_c_erased;
use crate::src::ipred::ipred_dc_c_erased;
use crate::src::ipred::ipred_dc_left_c_erased;
use crate::src::ipred::ipred_dc_top_c_erased;
use crate::src::ipred::ipred_filter_rust;
use crate::src::ipred::ipred_h_c_erased;
use crate::src::ipred::ipred_paeth_c_erased;
use crate::src::ipred::ipred_smooth_c_erased;
use crate::src::ipred::ipred_smooth_h_c_erased;
use crate::src::ipred::ipred_smooth_v_c_erased;
use crate::src::ipred::ipred_v_c_erased;
use crate::src::ipred::ipred_z1_rust;
use crate::src::ipred::ipred_z2_rust;
use crate::src::ipred::ipred_z3_rust;
use crate::src::ipred::pal_pred_rust;
use crate::src::ipred::Rav1dIntraPredDSPContext;
use crate::src::levels::DC_128_PRED;
use crate::src::levels::DC_PRED;
use crate::src::levels::FILTER_PRED;
use crate::src::levels::HOR_PRED;
use crate::src::levels::LEFT_DC_PRED;
use crate::src::levels::PAETH_PRED;
use crate::src::levels::SMOOTH_H_PRED;
use crate::src::levels::SMOOTH_PRED;
use crate::src::levels::SMOOTH_V_PRED;
use crate::src::levels::TOP_DC_PRED;
use crate::src::levels::VERT_PRED;
use crate::src::levels::Z1_PRED;
use crate::src::levels::Z2_PRED;
use crate::src::levels::Z3_PRED;
use libc::ptrdiff_t;
use std::ffi::c_int;

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
use crate::{src::ipred::ipred_z1_neon, src::ipred::ipred_z2_neon, src::ipred::ipred_z3_neon};

#[cfg(feature = "asm")]
use crate::src::cpu::{rav1d_get_cpu_flags, CpuFlags};

#[cfg(feature = "asm")]
use cfg_if::cfg_if;

unsafe extern "C" fn ipred_z1_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft_in: *const DynPixel,
    width: c_int,
    height: c_int,
    angle: c_int,
    max_width: c_int,
    max_height: c_int,
    bitdepth_max: c_int,
) {
    ipred_z1_rust(
        dst.cast(),
        stride,
        topleft_in.cast(),
        width,
        height,
        angle,
        max_width,
        max_height,
        BitDepth16::from_c(bitdepth_max),
    );
}

unsafe extern "C" fn ipred_z2_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft_in: *const DynPixel,
    width: c_int,
    height: c_int,
    angle: c_int,
    max_width: c_int,
    max_height: c_int,
    bitdepth_max: c_int,
) {
    ipred_z2_rust(
        dst.cast(),
        stride,
        topleft_in.cast(),
        width,
        height,
        angle,
        max_width,
        max_height,
        BitDepth16::from_c(bitdepth_max),
    );
}

unsafe extern "C" fn ipred_z3_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft_in: *const DynPixel,
    width: c_int,
    height: c_int,
    angle: c_int,
    max_width: c_int,
    max_height: c_int,
    bitdepth_max: c_int,
) {
    ipred_z3_rust(
        dst.cast(),
        stride,
        topleft_in.cast(),
        width,
        height,
        angle,
        max_width,
        max_height,
        BitDepth16::from_c(bitdepth_max),
    );
}

unsafe extern "C" fn ipred_filter_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft_in: *const DynPixel,
    width: c_int,
    height: c_int,
    filt_idx: c_int,
    max_width: c_int,
    max_height: c_int,
    bitdepth_max: c_int,
) {
    ipred_filter_rust(
        dst.cast(),
        stride,
        topleft_in.cast(),
        width,
        height,
        filt_idx,
        max_width,
        max_height,
        BitDepth16::from_c(bitdepth_max),
    );
}

unsafe extern "C" fn cfl_ac_420_c_erased(
    ac: *mut i16,
    ypx: *const DynPixel,
    stride: ptrdiff_t,
    w_pad: c_int,
    h_pad: c_int,
    cw: c_int,
    ch: c_int,
) {
    cfl_ac_rust::<BitDepth16>(
        ac,
        ypx.cast(),
        stride,
        w_pad,
        h_pad,
        cw,
        ch,
        1 as c_int,
        1 as c_int,
    );
}

unsafe extern "C" fn cfl_ac_422_c_erased(
    ac: *mut i16,
    ypx: *const DynPixel,
    stride: ptrdiff_t,
    w_pad: c_int,
    h_pad: c_int,
    cw: c_int,
    ch: c_int,
) {
    cfl_ac_rust::<BitDepth16>(
        ac,
        ypx.cast(),
        stride,
        w_pad,
        h_pad,
        cw,
        ch,
        1 as c_int,
        0 as c_int,
    );
}

unsafe extern "C" fn cfl_ac_444_c_erased(
    ac: *mut i16,
    ypx: *const DynPixel,
    stride: ptrdiff_t,
    w_pad: c_int,
    h_pad: c_int,
    cw: c_int,
    ch: c_int,
) {
    cfl_ac_rust::<BitDepth16>(
        ac,
        ypx.cast(),
        stride,
        w_pad,
        h_pad,
        cw,
        ch,
        0 as c_int,
        0 as c_int,
    );
}

unsafe extern "C" fn pal_pred_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    pal: *const u16,
    idx: *const u8,
    w: c_int,
    h: c_int,
) {
    pal_pred_rust::<BitDepth16>(dst.cast(), stride, pal, idx, w, h);
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64"),))]
#[inline(always)]
unsafe fn intra_pred_dsp_init_x86(c: *mut Rav1dIntraPredDSPContext) {
    use crate::src::ipred::*; // TODO(legare): Temporary import until init fns are deduplicated.

    let flags = rav1d_get_cpu_flags();

    if !flags.contains(CpuFlags::SSSE3) {
        return;
    }

    (*c).intra_pred[DC_PRED as usize] = Some(dav1d_ipred_dc_16bpc_ssse3);
    (*c).intra_pred[DC_128_PRED as usize] = Some(dav1d_ipred_dc_128_16bpc_ssse3);
    (*c).intra_pred[TOP_DC_PRED as usize] = Some(dav1d_ipred_dc_top_16bpc_ssse3);
    (*c).intra_pred[LEFT_DC_PRED as usize] = Some(dav1d_ipred_dc_left_16bpc_ssse3);
    (*c).intra_pred[HOR_PRED as usize] = Some(dav1d_ipred_h_16bpc_ssse3);
    (*c).intra_pred[VERT_PRED as usize] = Some(dav1d_ipred_v_16bpc_ssse3);
    (*c).intra_pred[PAETH_PRED as usize] = Some(dav1d_ipred_paeth_16bpc_ssse3);
    (*c).intra_pred[SMOOTH_PRED as usize] = Some(dav1d_ipred_smooth_16bpc_ssse3);
    (*c).intra_pred[SMOOTH_H_PRED as usize] = Some(dav1d_ipred_smooth_h_16bpc_ssse3);
    (*c).intra_pred[SMOOTH_V_PRED as usize] = Some(dav1d_ipred_smooth_v_16bpc_ssse3);
    (*c).intra_pred[Z1_PRED as usize] = Some(dav1d_ipred_z1_16bpc_ssse3);
    (*c).intra_pred[Z2_PRED as usize] = Some(dav1d_ipred_z2_16bpc_ssse3);
    (*c).intra_pred[Z3_PRED as usize] = Some(dav1d_ipred_z3_16bpc_ssse3);
    (*c).intra_pred[FILTER_PRED as usize] = Some(dav1d_ipred_filter_16bpc_ssse3);

    (*c).cfl_pred[DC_PRED as usize] = dav1d_ipred_cfl_16bpc_ssse3;
    (*c).cfl_pred[DC_128_PRED as usize] = dav1d_ipred_cfl_128_16bpc_ssse3;
    (*c).cfl_pred[TOP_DC_PRED as usize] = dav1d_ipred_cfl_top_16bpc_ssse3;
    (*c).cfl_pred[LEFT_DC_PRED as usize] = dav1d_ipred_cfl_left_16bpc_ssse3;

    (*c).cfl_ac[Rav1dPixelLayout::I420 as usize - 1] = dav1d_ipred_cfl_ac_420_16bpc_ssse3;
    (*c).cfl_ac[Rav1dPixelLayout::I422 as usize - 1] = dav1d_ipred_cfl_ac_422_16bpc_ssse3;
    (*c).cfl_ac[Rav1dPixelLayout::I444 as usize - 1] = dav1d_ipred_cfl_ac_444_16bpc_ssse3;

    (*c).pal_pred = dav1d_pal_pred_16bpc_ssse3;

    #[cfg(target_arch = "x86_64")]
    {
        if !flags.contains(CpuFlags::AVX2) {
            return;
        }

        (*c).intra_pred[DC_PRED as usize] = Some(dav1d_ipred_dc_16bpc_avx2);
        (*c).intra_pred[DC_128_PRED as usize] = Some(dav1d_ipred_dc_128_16bpc_avx2);
        (*c).intra_pred[TOP_DC_PRED as usize] = Some(dav1d_ipred_dc_top_16bpc_avx2);
        (*c).intra_pred[LEFT_DC_PRED as usize] = Some(dav1d_ipred_dc_left_16bpc_avx2);
        (*c).intra_pred[HOR_PRED as usize] = Some(dav1d_ipred_h_16bpc_avx2);
        (*c).intra_pred[VERT_PRED as usize] = Some(dav1d_ipred_v_16bpc_avx2);
        (*c).intra_pred[PAETH_PRED as usize] = Some(dav1d_ipred_paeth_16bpc_avx2);
        (*c).intra_pred[SMOOTH_PRED as usize] = Some(dav1d_ipred_smooth_16bpc_avx2);
        (*c).intra_pred[SMOOTH_H_PRED as usize] = Some(dav1d_ipred_smooth_h_16bpc_avx2);
        (*c).intra_pred[SMOOTH_V_PRED as usize] = Some(dav1d_ipred_smooth_v_16bpc_avx2);
        (*c).intra_pred[Z1_PRED as usize] = Some(dav1d_ipred_z1_16bpc_avx2);
        (*c).intra_pred[Z2_PRED as usize] = Some(dav1d_ipred_z2_16bpc_avx2);
        (*c).intra_pred[Z3_PRED as usize] = Some(dav1d_ipred_z3_16bpc_avx2);
        (*c).intra_pred[FILTER_PRED as usize] = Some(dav1d_ipred_filter_16bpc_avx2);

        (*c).cfl_pred[DC_PRED as usize] = dav1d_ipred_cfl_16bpc_avx2;
        (*c).cfl_pred[DC_128_PRED as usize] = dav1d_ipred_cfl_128_16bpc_avx2;
        (*c).cfl_pred[TOP_DC_PRED as usize] = dav1d_ipred_cfl_top_16bpc_avx2;
        (*c).cfl_pred[LEFT_DC_PRED as usize] = dav1d_ipred_cfl_left_16bpc_avx2;

        (*c).cfl_ac[Rav1dPixelLayout::I420 as usize - 1] = dav1d_ipred_cfl_ac_420_16bpc_avx2;
        (*c).cfl_ac[Rav1dPixelLayout::I422 as usize - 1] = dav1d_ipred_cfl_ac_422_16bpc_avx2;
        (*c).cfl_ac[Rav1dPixelLayout::I444 as usize - 1] = dav1d_ipred_cfl_ac_444_16bpc_avx2;

        (*c).pal_pred = dav1d_pal_pred_16bpc_avx2;

        if !flags.contains(CpuFlags::AVX512ICL) {
            return;
        }

        (*c).intra_pred[PAETH_PRED as usize] = Some(dav1d_ipred_paeth_16bpc_avx512icl);
        (*c).intra_pred[SMOOTH_PRED as usize] = Some(dav1d_ipred_smooth_16bpc_avx512icl);
        (*c).intra_pred[SMOOTH_H_PRED as usize] = Some(dav1d_ipred_smooth_h_16bpc_avx512icl);
        (*c).intra_pred[SMOOTH_V_PRED as usize] = Some(dav1d_ipred_smooth_v_16bpc_avx512icl);
        (*c).intra_pred[FILTER_PRED as usize] = Some(dav1d_ipred_filter_16bpc_avx512icl);

        (*c).pal_pred = dav1d_pal_pred_16bpc_avx512icl;
    }
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
#[inline(always)]
unsafe fn intra_pred_dsp_init_arm(c: *mut Rav1dIntraPredDSPContext) {
    // TODO(legare): Temporary import until init fns are deduplicated.
    use crate::src::ipred::*;

    let flags = rav1d_get_cpu_flags();
    if !flags.contains(CpuFlags::NEON) {
        return;
    }

    (*c).intra_pred[DC_PRED as usize] = Some(dav1d_ipred_dc_16bpc_neon);
    (*c).intra_pred[DC_128_PRED as usize] = Some(dav1d_ipred_dc_128_16bpc_neon);
    (*c).intra_pred[TOP_DC_PRED as usize] = Some(dav1d_ipred_dc_top_16bpc_neon);
    (*c).intra_pred[LEFT_DC_PRED as usize] = Some(dav1d_ipred_dc_left_16bpc_neon);
    (*c).intra_pred[HOR_PRED as usize] = Some(dav1d_ipred_h_16bpc_neon);
    (*c).intra_pred[VERT_PRED as usize] = Some(dav1d_ipred_v_16bpc_neon);
    (*c).intra_pred[PAETH_PRED as usize] = Some(dav1d_ipred_paeth_16bpc_neon);
    (*c).intra_pred[SMOOTH_PRED as usize] = Some(dav1d_ipred_smooth_16bpc_neon);
    (*c).intra_pred[SMOOTH_V_PRED as usize] = Some(dav1d_ipred_smooth_v_16bpc_neon);
    (*c).intra_pred[SMOOTH_H_PRED as usize] = Some(dav1d_ipred_smooth_h_16bpc_neon);
    #[cfg(target_arch = "aarch64")]
    {
        (*c).intra_pred[Z1_PRED as usize] = Some(ipred_z1_neon_erased);
        (*c).intra_pred[Z2_PRED as usize] = Some(ipred_z2_neon_erased);
        (*c).intra_pred[Z3_PRED as usize] = Some(ipred_z3_neon_erased);
    }
    (*c).intra_pred[FILTER_PRED as usize] = Some(dav1d_ipred_filter_16bpc_neon);

    (*c).cfl_pred[DC_PRED as usize] = dav1d_ipred_cfl_16bpc_neon;
    (*c).cfl_pred[DC_128_PRED as usize] = dav1d_ipred_cfl_128_16bpc_neon;
    (*c).cfl_pred[TOP_DC_PRED as usize] = dav1d_ipred_cfl_top_16bpc_neon;
    (*c).cfl_pred[LEFT_DC_PRED as usize] = dav1d_ipred_cfl_left_16bpc_neon;

    (*c).cfl_ac[Rav1dPixelLayout::I420 as usize - 1] = dav1d_ipred_cfl_ac_420_16bpc_neon;
    (*c).cfl_ac[Rav1dPixelLayout::I422 as usize - 1] = dav1d_ipred_cfl_ac_422_16bpc_neon;
    (*c).cfl_ac[Rav1dPixelLayout::I444 as usize - 1] = dav1d_ipred_cfl_ac_444_16bpc_neon;

    (*c).pal_pred = dav1d_pal_pred_16bpc_neon;
}

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
unsafe extern "C" fn ipred_z3_neon_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft_in: *const DynPixel,
    width: c_int,
    height: c_int,
    angle: c_int,
    max_width: c_int,
    max_height: c_int,
    bitdepth_max: c_int,
) {
    ipred_z3_neon(
        dst.cast(),
        stride,
        topleft_in.cast(),
        width,
        height,
        angle,
        max_width,
        max_height,
        BitDepth16::from_c(bitdepth_max),
    );
}

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
unsafe extern "C" fn ipred_z2_neon_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft_in: *const DynPixel,
    width: c_int,
    height: c_int,
    angle: c_int,
    max_width: c_int,
    max_height: c_int,
    bitdepth_max: c_int,
) {
    ipred_z2_neon(
        dst.cast(),
        stride,
        topleft_in.cast(),
        width,
        height,
        angle,
        max_width,
        max_height,
        BitDepth16::from_c(bitdepth_max),
    );
}

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
unsafe extern "C" fn ipred_z1_neon_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft_in: *const DynPixel,
    width: c_int,
    height: c_int,
    angle: c_int,
    max_width: c_int,
    max_height: c_int,
    bitdepth_max: c_int,
) {
    ipred_z1_neon(
        dst.cast(),
        stride,
        topleft_in.cast(),
        width,
        height,
        angle,
        max_width,
        max_height,
        BitDepth16::from_c(bitdepth_max),
    );
}

#[cold]
pub unsafe fn rav1d_intra_pred_dsp_init_16bpc(c: *mut Rav1dIntraPredDSPContext) {
    (*c).intra_pred[DC_PRED as usize] = Some(ipred_dc_c_erased::<BitDepth16>);
    (*c).intra_pred[DC_128_PRED as usize] = Some(ipred_dc_128_c_erased::<BitDepth16>);
    (*c).intra_pred[TOP_DC_PRED as usize] = Some(ipred_dc_top_c_erased::<BitDepth16>);
    (*c).intra_pred[LEFT_DC_PRED as usize] = Some(ipred_dc_left_c_erased::<BitDepth16>);
    (*c).intra_pred[HOR_PRED as usize] = Some(ipred_h_c_erased::<BitDepth16>);
    (*c).intra_pred[VERT_PRED as usize] = Some(ipred_v_c_erased::<BitDepth16>);
    (*c).intra_pred[PAETH_PRED as usize] = Some(ipred_paeth_c_erased::<BitDepth16>);
    (*c).intra_pred[SMOOTH_PRED as usize] = Some(ipred_smooth_c_erased::<BitDepth16>);
    (*c).intra_pred[SMOOTH_V_PRED as usize] = Some(ipred_smooth_v_c_erased::<BitDepth16>);
    (*c).intra_pred[SMOOTH_H_PRED as usize] = Some(ipred_smooth_h_c_erased::<BitDepth16>);
    (*c).intra_pred[Z1_PRED as usize] = Some(ipred_z1_c_erased);
    (*c).intra_pred[Z2_PRED as usize] = Some(ipred_z2_c_erased);
    (*c).intra_pred[Z3_PRED as usize] = Some(ipred_z3_c_erased);
    (*c).intra_pred[FILTER_PRED as usize] = Some(ipred_filter_c_erased);

    (*c).cfl_ac[Rav1dPixelLayout::I420 as usize - 1] = cfl_ac_420_c_erased;
    (*c).cfl_ac[Rav1dPixelLayout::I422 as usize - 1] = cfl_ac_422_c_erased;
    (*c).cfl_ac[Rav1dPixelLayout::I444 as usize - 1] = cfl_ac_444_c_erased;
    (*c).cfl_pred[DC_PRED as usize] = ipred_cfl_c_erased::<BitDepth16>;

    (*c).cfl_pred[DC_128_PRED as usize] = ipred_cfl_128_c_erased::<BitDepth16>;
    (*c).cfl_pred[TOP_DC_PRED as usize] = ipred_cfl_top_c_erased::<BitDepth16>;
    (*c).cfl_pred[LEFT_DC_PRED as usize] = ipred_cfl_left_c_erased::<BitDepth16>;

    (*c).pal_pred = pal_pred_c_erased;

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            intra_pred_dsp_init_x86(c);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
            intra_pred_dsp_init_arm(c);
        }
    }
}
