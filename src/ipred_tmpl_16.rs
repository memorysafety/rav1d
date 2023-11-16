use crate::include::common::attributes::ctz;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::BitDepth16;
use crate::include::common::bitdepth::DynPixel;
use crate::include::dav1d::headers::Rav1dPixelLayout;
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
use libc::memcpy;
use libc::ptrdiff_t;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_void;

#[cfg(feature = "asm")]
use crate::src::cpu::{rav1d_get_cpu_flags, CpuFlags};

#[cfg(feature = "asm")]
use cfg_if::cfg_if;

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
use std::cmp;

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
use crate::{
    src::ipred::get_filter_strength, src::ipred::get_upsample,
    src::tables::dav1d_dr_intra_derivative,
};

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
extern "C" {
    fn dav1d_ipred_z1_fill2_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        top: *const pixel,
        width: c_int,
        height: c_int,
        dx: c_int,
        max_base_x: c_int,
    );
    fn dav1d_ipred_z1_fill1_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        top: *const pixel,
        width: c_int,
        height: c_int,
        dx: c_int,
        max_base_x: c_int,
    );
    fn dav1d_ipred_z1_upsample_edge_16bpc_neon(
        out: *mut pixel,
        hsz: c_int,
        in_0: *const pixel,
        end: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_ipred_z1_filter_edge_16bpc_neon(
        out: *mut pixel,
        sz: c_int,
        in_0: *const pixel,
        end: c_int,
        strength: c_int,
    );
    fn dav1d_ipred_z2_fill3_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        top: *const pixel,
        left: *const pixel,
        width: c_int,
        height: c_int,
        dx: c_int,
        dy: c_int,
    );
    fn dav1d_ipred_z2_fill2_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        top: *const pixel,
        left: *const pixel,
        width: c_int,
        height: c_int,
        dx: c_int,
        dy: c_int,
    );
    fn dav1d_ipred_z2_fill1_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        top: *const pixel,
        left: *const pixel,
        width: c_int,
        height: c_int,
        dx: c_int,
        dy: c_int,
    );
    fn dav1d_ipred_z2_upsample_edge_16bpc_neon(
        out: *mut pixel,
        hsz: c_int,
        in_0: *const pixel,
        bitdepth_max: c_int,
    );
    fn dav1d_ipred_reverse_16bpc_neon(dst: *mut pixel, src: *const pixel, n: c_int);
    fn dav1d_ipred_z3_fill2_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: *const pixel,
        width: c_int,
        height: c_int,
        dy: c_int,
        max_base_y: c_int,
    );
    fn dav1d_ipred_z3_fill1_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: *const pixel,
        width: c_int,
        height: c_int,
        dy: c_int,
        max_base_y: c_int,
    );
    fn dav1d_ipred_pixel_set_16bpc_neon(out: *mut pixel, px: pixel, n: c_int);
}

pub type pixel = u16;

#[inline]
unsafe fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 != 0 {
        unreachable!();
    }
    return x >> 1;
}

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
    ipred_z1_rust::<BitDepth16>(
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
    ipred_z2_rust::<BitDepth16>(
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
    ipred_z3_rust::<BitDepth16>(
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
    ipred_filter_rust::<BitDepth16>(
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

#[inline(never)]
unsafe fn cfl_ac_rust(
    mut ac: *mut i16,
    mut ypx: *const pixel,
    stride: ptrdiff_t,
    w_pad: c_int,
    h_pad: c_int,
    width: c_int,
    height: c_int,
    ss_hor: c_int,
    ss_ver: c_int,
) {
    let mut y;
    let mut x: i32;
    let ac_orig: *mut i16 = ac;
    if !(w_pad >= 0 && (w_pad * 4) < width) {
        unreachable!();
    }
    if !(h_pad >= 0 && (h_pad * 4) < height) {
        unreachable!();
    }
    y = 0 as c_int;
    while y < height - 4 * h_pad {
        x = 0 as c_int;
        while x < width - 4 * w_pad {
            let mut ac_sum = *ypx.offset((x << ss_hor) as isize) as c_int;
            if ss_hor != 0 {
                ac_sum += *ypx.offset((x * 2 + 1) as isize) as c_int;
            }
            if ss_ver != 0 {
                ac_sum +=
                    *ypx.offset(((x << ss_hor) as isize + PXSTRIDE(stride)) as isize) as c_int;
                if ss_hor != 0 {
                    ac_sum +=
                        *ypx.offset(((x * 2 + 1) as isize + PXSTRIDE(stride)) as isize) as c_int;
                }
            }
            *ac.offset(x as isize) =
                (ac_sum << 1 + (ss_ver == 0) as c_int + (ss_hor == 0) as c_int) as i16;
            x += 1;
        }
        while x < width {
            *ac.offset(x as isize) = *ac.offset((x - 1) as isize);
            x += 1;
        }
        ac = ac.offset(width as isize);
        ypx = ypx.offset((PXSTRIDE(stride) << ss_ver) as isize);
        y += 1;
    }
    while y < height {
        memcpy(
            ac as *mut c_void,
            &mut *ac.offset(-width as isize) as *mut i16 as *const c_void,
            (width as usize).wrapping_mul(::core::mem::size_of::<i16>()),
        );
        ac = ac.offset(width as isize);
        y += 1;
    }
    let log2sz = ctz(width as c_uint) + ctz(height as c_uint);
    let mut sum = (1 as c_int) << log2sz >> 1;
    ac = ac_orig;
    y = 0 as c_int;
    while y < height {
        x = 0 as c_int;
        while x < width {
            sum += *ac.offset(x as isize) as c_int;
            x += 1;
        }
        ac = ac.offset(width as isize);
        y += 1;
    }
    sum >>= log2sz;
    ac = ac_orig;
    y = 0 as c_int;
    while y < height {
        x = 0 as c_int;
        while x < width {
            let ref mut fresh0 = *ac.offset(x as isize);
            *fresh0 = (*fresh0 as c_int - sum) as i16;
            x += 1;
        }
        ac = ac.offset(width as isize);
        y += 1;
    }
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
    cfl_ac_rust(
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
    cfl_ac_rust(
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
    cfl_ac_rust(
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
    pal_pred_rust(dst.cast(), stride, pal, idx, w, h);
}

unsafe fn pal_pred_rust(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    pal: *const u16,
    mut idx: *const u8,
    w: c_int,
    h: c_int,
) {
    let mut y = 0;
    while y < h {
        let mut x = 0;
        while x < w {
            *dst.offset(x as isize) = *pal.offset(*idx.offset(x as isize) as isize);
            x += 1;
        }
        idx = idx.offset(w as isize);
        dst = dst.offset(PXSTRIDE(stride) as isize);
        y += 1;
    }
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
        bitdepth_max,
    );
}

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
unsafe fn ipred_z3_neon(
    dst: *mut pixel,
    stride: ptrdiff_t,
    topleft_in: *const pixel,
    width: c_int,
    height: c_int,
    mut angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
    bitdepth_max: c_int,
) {
    let is_sm = angle >> 9 & 0x1 as c_int;
    let enable_intra_edge_filter = angle >> 10;
    angle &= 511 as c_int;
    if !(angle > 180) {
        unreachable!();
    }
    let mut dy = dav1d_dr_intra_derivative[(270 - angle >> 1) as usize] as c_int;
    let mut flipped: [pixel; 144] = [0; 144];
    let mut left_out: [pixel; 286] = [0; 286];
    let max_base_y;
    let upsample_left = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, angle - 180, is_sm)
    } else {
        0 as c_int
    };
    if upsample_left != 0 {
        flipped[0] = *topleft_in.offset(0);
        dav1d_ipred_reverse_16bpc_neon(
            &mut *flipped.as_mut_ptr().offset(1),
            &*topleft_in.offset(0),
            height + cmp::max(width, height),
        );
        dav1d_ipred_z1_upsample_edge_16bpc_neon(
            left_out.as_mut_ptr(),
            width + height,
            flipped.as_mut_ptr(),
            height + cmp::min(width, height),
            bitdepth_max,
        );
        max_base_y = 2 * (width + height) - 2;
        dy <<= 1;
    } else {
        let filter_strength = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, angle - 180, is_sm)
        } else {
            0 as c_int
        };
        if filter_strength != 0 {
            flipped[0] = *topleft_in.offset(0);
            dav1d_ipred_reverse_16bpc_neon(
                &mut *flipped.as_mut_ptr().offset(1),
                &*topleft_in.offset(0),
                height + cmp::max(width, height),
            );
            dav1d_ipred_z1_filter_edge_16bpc_neon(
                left_out.as_mut_ptr(),
                width + height,
                flipped.as_mut_ptr(),
                height + cmp::min(width, height),
                filter_strength,
            );
            max_base_y = width + height - 1;
        } else {
            dav1d_ipred_reverse_16bpc_neon(
                left_out.as_mut_ptr(),
                &*topleft_in.offset(0),
                height + cmp::min(width, height),
            );
            max_base_y = height + cmp::min(width, height) - 1;
        }
    }
    let base_inc = 1 + upsample_left;
    let pad_pixels = cmp::max(64 - max_base_y - 1, height + 15);
    dav1d_ipred_pixel_set_16bpc_neon(
        &mut *left_out.as_mut_ptr().offset((max_base_y + 1) as isize) as *mut pixel,
        left_out[max_base_y as usize],
        (pad_pixels * base_inc) as c_int,
    );
    if upsample_left != 0 {
        dav1d_ipred_z3_fill2_16bpc_neon(
            dst,
            stride,
            left_out.as_mut_ptr(),
            width,
            height,
            dy,
            max_base_y,
        );
    } else {
        dav1d_ipred_z3_fill1_16bpc_neon(
            dst,
            stride,
            left_out.as_mut_ptr(),
            width,
            height,
            dy,
            max_base_y,
        );
    };
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
        bitdepth_max,
    );
}

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
unsafe fn ipred_z2_neon(
    dst: *mut pixel,
    stride: ptrdiff_t,
    topleft_in: *const pixel,
    width: c_int,
    height: c_int,
    mut angle: c_int,
    max_width: c_int,
    max_height: c_int,
    bitdepth_max: c_int,
) {
    let is_sm = angle >> 9 & 0x1 as c_int;
    let enable_intra_edge_filter = angle >> 10;
    angle &= 511 as c_int;
    if !(angle > 90 && angle < 180) {
        unreachable!();
    }
    let mut dy = dav1d_dr_intra_derivative[((angle - 90) >> 1) as usize] as c_int;
    let mut dx = dav1d_dr_intra_derivative[((180 - angle) >> 1) as usize] as c_int;
    let mut buf: [pixel; 3 * (64 + 1)] = [0; 3 * (64 + 1)]; // NOTE: C code doesn't initialize

    // The asm can underread below the start of top[] and left[]; to avoid
    // surprising behaviour, make sure this is within the allocated stack space.
    let left_offset: isize = 2 * (64 + 1);
    let top_offset: isize = 1 * (64 + 1);
    let flipped_offset: isize = 0 * (64 + 1);

    let upsample_left = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, 180 - angle, is_sm)
    } else {
        0 as c_int
    };
    let upsample_above = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, angle - 90, is_sm)
    } else {
        0 as c_int
    };

    if upsample_above != 0 {
        dav1d_ipred_z2_upsample_edge_16bpc_neon(
            buf.as_mut_ptr().offset(top_offset),
            width,
            topleft_in,
            bitdepth_max,
        );
        dx <<= 1;
    } else {
        let filter_strength = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, angle - 90, is_sm)
        } else {
            0 as c_int
        };

        if filter_strength != 0 {
            dav1d_ipred_z1_filter_edge_16bpc_neon(
                buf.as_mut_ptr().offset(1 + top_offset),
                cmp::min(max_width, width),
                topleft_in,
                width,
                filter_strength,
            );

            if max_width < width {
                memcpy(
                    buf.as_mut_ptr().offset(top_offset + 1 + max_width as isize) as *mut c_void,
                    topleft_in.offset(1 + max_width as isize) as *const c_void,
                    ((width - max_width) as usize).wrapping_mul(::core::mem::size_of::<pixel>()),
                );
            }
        } else {
            BitDepth16::pixel_copy(
                &mut buf[1 + top_offset as usize..],
                core::slice::from_raw_parts(topleft_in.offset(1), width as usize),
                width as usize,
            );
        }
    }

    if upsample_left != 0 {
        buf[flipped_offset as usize] = *topleft_in;
        dav1d_ipred_reverse_16bpc_neon(
            &mut *buf.as_mut_ptr().offset(1 + flipped_offset),
            topleft_in,
            height,
        );
        dav1d_ipred_z2_upsample_edge_16bpc_neon(
            buf.as_mut_ptr().offset(left_offset),
            height,
            buf.as_ptr().offset(flipped_offset),
            bitdepth_max,
        );
        dy <<= 1;
    } else {
        let filter_strength = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, 180 - angle, is_sm)
        } else {
            0 as c_int
        };
        if filter_strength != 0 {
            buf[flipped_offset as usize] = *topleft_in;
            dav1d_ipred_reverse_16bpc_neon(
                &mut *buf.as_mut_ptr().offset(1 + flipped_offset),
                topleft_in,
                height,
            );
            dav1d_ipred_z1_filter_edge_16bpc_neon(
                buf.as_mut_ptr().offset(1 + left_offset),
                cmp::min(max_height, height),
                buf.as_ptr().offset(flipped_offset),
                height,
                filter_strength,
            );
            if max_height < height {
                memcpy(
                    buf.as_mut_ptr()
                        .offset(left_offset + 1 + max_height as isize)
                        as *mut c_void,
                    buf.as_mut_ptr()
                        .offset(flipped_offset + 1 + max_height as isize)
                        as *const c_void,
                    ((height - max_height) as usize).wrapping_mul(::core::mem::size_of::<pixel>()),
                );
            }
        } else {
            dav1d_ipred_reverse_16bpc_neon(
                buf.as_mut_ptr().offset(left_offset + 1),
                topleft_in,
                height,
            );
        }
    }
    buf[top_offset as usize] = *topleft_in;
    buf[left_offset as usize] = *topleft_in;

    if upsample_above != 0 && upsample_left != 0 {
        unreachable!();
    }

    if upsample_above == 0 && upsample_left == 0 {
        dav1d_ipred_z2_fill1_16bpc_neon(
            dst,
            stride,
            buf.as_ptr().offset(top_offset),
            buf.as_ptr().offset(left_offset),
            width,
            height,
            dx,
            dy,
        );
    } else if upsample_above != 0 {
        dav1d_ipred_z2_fill2_16bpc_neon(
            dst,
            stride,
            buf.as_ptr().offset(top_offset),
            buf.as_ptr().offset(left_offset),
            width,
            height,
            dx,
            dy,
        );
    } else {
        dav1d_ipred_z2_fill3_16bpc_neon(
            dst,
            stride,
            buf.as_ptr().offset(top_offset),
            buf.as_ptr().offset(left_offset),
            width,
            height,
            dx,
            dy,
        );
    };
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
        bitdepth_max,
    );
}

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
unsafe fn ipred_z1_neon(
    dst: *mut pixel,
    stride: ptrdiff_t,
    topleft_in: *const pixel,
    width: c_int,
    height: c_int,
    mut angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
    bitdepth_max: c_int,
) {
    let is_sm = angle >> 9 & 0x1 as c_int;
    let enable_intra_edge_filter = angle >> 10;
    angle &= 511 as c_int;
    let mut dx = dav1d_dr_intra_derivative[(angle >> 1) as usize] as c_int;
    const top_out_size: usize = 64 + 64 * (64 + 15) * 2 + 16;
    let mut top_out: [pixel; top_out_size] = [0; top_out_size];
    let max_base_x;
    let upsample_above = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, 90 - angle, is_sm)
    } else {
        0 as c_int
    };
    if upsample_above != 0 {
        dav1d_ipred_z1_upsample_edge_16bpc_neon(
            top_out.as_mut_ptr(),
            width + height,
            topleft_in,
            width + cmp::min(width, height),
            bitdepth_max,
        );
        max_base_x = 2 * (width + height) - 2;
        dx <<= 1;
    } else {
        let filter_strength = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, 90 - angle, is_sm)
        } else {
            0 as c_int
        };
        if filter_strength != 0 {
            dav1d_ipred_z1_filter_edge_16bpc_neon(
                top_out.as_mut_ptr(),
                width + height,
                topleft_in,
                width + cmp::min(width, height),
                filter_strength,
            );
            max_base_x = width + height - 1;
        } else {
            max_base_x = width + cmp::min(width, height) - 1;
            memcpy(
                top_out.as_mut_ptr() as *mut c_void,
                &*topleft_in.offset(1) as *const pixel as *const c_void,
                ((max_base_x + 1) as usize).wrapping_mul(::core::mem::size_of::<pixel>()),
            );
        }
    }
    let base_inc = 1 + upsample_above;
    let pad_pixels = width + 15;
    dav1d_ipred_pixel_set_16bpc_neon(
        &mut *top_out.as_mut_ptr().offset((max_base_x + 1) as isize) as *mut pixel,
        top_out[max_base_x as usize],
        (pad_pixels * base_inc) as c_int,
    );
    if upsample_above != 0 {
        dav1d_ipred_z1_fill2_16bpc_neon(
            dst,
            stride,
            top_out.as_mut_ptr(),
            width,
            height,
            dx,
            max_base_x,
        );
    } else {
        dav1d_ipred_z1_fill1_16bpc_neon(
            dst,
            stride,
            top_out.as_mut_ptr(),
            width,
            height,
            dx,
            max_base_x,
        );
    };
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
