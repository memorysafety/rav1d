use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::BitDepth16;
use crate::include::common::bitdepth::DynPixel;
use crate::src::lf_mask::Av1FilterLUT;
use crate::src::loopfilter::loop_filter;
use crate::src::loopfilter::loop_filter_h_sb128y_c_erased;
use crate::src::loopfilter::loop_filter_v_sb128y_rust;
use crate::src::loopfilter::Rav1dLoopFilterDSPContext;
use libc::ptrdiff_t;
use std::ffi::c_int;
use std::ffi::c_uint;

#[cfg(feature = "asm")]
use crate::src::cpu::{rav1d_get_cpu_flags, CpuFlags};

#[cfg(feature = "asm")]
use cfg_if::cfg_if;

pub type pixel = u16;

#[inline]
unsafe fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 != 0 {
        unreachable!();
    }
    return x >> 1;
}

unsafe extern "C" fn loop_filter_v_sb128y_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    vmask: *const u32,
    l: *const [u8; 4],
    b4_stride: ptrdiff_t,
    lut: *const Av1FilterLUT,
    w: c_int,
    bitdepth_max: c_int,
) {
    loop_filter_v_sb128y_rust(
        dst.cast(),
        stride,
        vmask,
        l,
        b4_stride,
        lut,
        w,
        BitDepth16::from_c(bitdepth_max),
    );
}

unsafe extern "C" fn loop_filter_h_sb128uv_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    vmask: *const u32,
    l: *const [u8; 4],
    b4_stride: ptrdiff_t,
    lut: *const Av1FilterLUT,
    h: c_int,
    bitdepth_max: c_int,
) {
    loop_filter_h_sb128uv_rust(
        dst.cast(),
        stride,
        vmask,
        l,
        b4_stride,
        lut,
        h,
        bitdepth_max,
    )
}

unsafe fn loop_filter_h_sb128uv_rust(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    vmask: *const u32,
    mut l: *const [u8; 4],
    b4_stride: ptrdiff_t,
    lut: *const Av1FilterLUT,
    _h: c_int,
    bitdepth_max: c_int,
) {
    let vm: c_uint = *vmask.offset(0) | *vmask.offset(1);
    let mut y: c_uint = 1 as c_int as c_uint;
    while vm & !y.wrapping_sub(1 as c_int as c_uint) != 0 {
        if vm & y != 0 {
            let L = if (*l.offset(0))[0] as c_int != 0 {
                (*l.offset(0))[0] as c_int
            } else {
                (*l.offset(-(1 as c_int) as isize))[0] as c_int
            };
            if !(L == 0) {
                let H = L >> 4;
                let E = (*lut).e[L as usize] as c_int;
                let I = (*lut).i[L as usize] as c_int;
                let idx = (*vmask.offset(1) & y != 0) as c_int;
                loop_filter(
                    dst,
                    E,
                    I,
                    H,
                    PXSTRIDE(stride),
                    1 as c_int as ptrdiff_t,
                    4 + 2 * idx,
                    BitDepth16::from_c(bitdepth_max),
                );
            }
        }
        y <<= 1;
        dst = dst.offset((4 * PXSTRIDE(stride)) as isize);
        l = l.offset(b4_stride as isize);
    }
}

unsafe extern "C" fn loop_filter_v_sb128uv_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    vmask: *const u32,
    l: *const [u8; 4],
    b4_stride: ptrdiff_t,
    lut: *const Av1FilterLUT,
    w: c_int,
    bitdepth_max: c_int,
) {
    loop_filter_v_sb128uv_rust(
        dst.cast(),
        stride,
        vmask,
        l,
        b4_stride,
        lut,
        w,
        bitdepth_max,
    )
}

unsafe fn loop_filter_v_sb128uv_rust(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    vmask: *const u32,
    mut l: *const [u8; 4],
    b4_stride: ptrdiff_t,
    lut: *const Av1FilterLUT,
    _w: c_int,
    bitdepth_max: c_int,
) {
    let vm: c_uint = *vmask.offset(0) | *vmask.offset(1);
    let mut x: c_uint = 1 as c_int as c_uint;
    while vm & !x.wrapping_sub(1 as c_int as c_uint) != 0 {
        if vm & x != 0 {
            let L = if (*l.offset(0))[0] as c_int != 0 {
                (*l.offset(0))[0] as c_int
            } else {
                (*l.offset(-b4_stride as isize))[0] as c_int
            };
            if !(L == 0) {
                let H = L >> 4;
                let E = (*lut).e[L as usize] as c_int;
                let I = (*lut).i[L as usize] as c_int;
                let idx = (*vmask.offset(1) & x != 0) as c_int;
                loop_filter(
                    dst,
                    E,
                    I,
                    H,
                    1 as c_int as ptrdiff_t,
                    PXSTRIDE(stride),
                    4 + 2 * idx,
                    BitDepth16::from_c(bitdepth_max),
                );
            }
        }
        x <<= 1;
        dst = dst.offset(4);
        l = l.offset(1);
    }
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
#[inline(always)]
unsafe fn loop_filter_dsp_init_x86(c: *mut Rav1dLoopFilterDSPContext) {
    // TODO(legare): Temporarily import until init fns are deduplicated.
    use crate::src::loopfilter::*;

    let flags = rav1d_get_cpu_flags();

    if !flags.contains(CpuFlags::SSSE3) {
        return;
    }

    (*c).loop_filter_sb[0][0] = dav1d_lpf_h_sb_y_16bpc_ssse3;
    (*c).loop_filter_sb[0][1] = dav1d_lpf_v_sb_y_16bpc_ssse3;
    (*c).loop_filter_sb[1][0] = dav1d_lpf_h_sb_uv_16bpc_ssse3;
    (*c).loop_filter_sb[1][1] = dav1d_lpf_v_sb_uv_16bpc_ssse3;

    #[cfg(target_arch = "x86_64")]
    {
        if !flags.contains(CpuFlags::AVX2) {
            return;
        }

        (*c).loop_filter_sb[0][0] = dav1d_lpf_h_sb_y_16bpc_avx2;
        (*c).loop_filter_sb[0][1] = dav1d_lpf_v_sb_y_16bpc_avx2;
        (*c).loop_filter_sb[1][0] = dav1d_lpf_h_sb_uv_16bpc_avx2;
        (*c).loop_filter_sb[1][1] = dav1d_lpf_v_sb_uv_16bpc_avx2;

        if !flags.contains(CpuFlags::AVX512ICL) {
            return;
        }

        (*c).loop_filter_sb[0][0] = dav1d_lpf_h_sb_y_16bpc_avx512icl;
        (*c).loop_filter_sb[0][1] = dav1d_lpf_v_sb_y_16bpc_avx512icl;
        (*c).loop_filter_sb[1][0] = dav1d_lpf_h_sb_uv_16bpc_avx512icl;
        (*c).loop_filter_sb[1][1] = dav1d_lpf_v_sb_uv_16bpc_avx512icl;
    }
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
#[inline(always)]
unsafe fn loop_filter_dsp_init_arm(c: *mut Rav1dLoopFilterDSPContext) {
    // TODO(legare): Temporarily import until init fns are deduplicated.
    use crate::src::loopfilter::*;

    let flags = rav1d_get_cpu_flags();

    if !flags.contains(CpuFlags::NEON) {
        return;
    }

    (*c).loop_filter_sb[0][0] = dav1d_lpf_h_sb_y_16bpc_neon;
    (*c).loop_filter_sb[0][1] = dav1d_lpf_v_sb_y_16bpc_neon;
    (*c).loop_filter_sb[1][0] = dav1d_lpf_h_sb_uv_16bpc_neon;
    (*c).loop_filter_sb[1][1] = dav1d_lpf_v_sb_uv_16bpc_neon;
}

#[cold]
pub unsafe fn rav1d_loop_filter_dsp_init_16bpc(c: *mut Rav1dLoopFilterDSPContext) {
    (*c).loop_filter_sb[0][0] = loop_filter_h_sb128y_c_erased::<BitDepth16>;
    (*c).loop_filter_sb[0][1] = loop_filter_v_sb128y_c_erased;
    (*c).loop_filter_sb[1][0] = loop_filter_h_sb128uv_c_erased;
    (*c).loop_filter_sb[1][1] = loop_filter_v_sb128uv_c_erased;

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            loop_filter_dsp_init_x86(c);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
            loop_filter_dsp_init_arm(c);
        }
    }
}
