use crate::include::common::bitdepth::BitDepth16;
use crate::include::stddef::*;
use crate::include::stdint::*;
#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
use crate::src::align::Align16;
use ::libc;
#[cfg(feature = "asm")]
use cfg_if::cfg_if;

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
extern "C" {
    fn dav1d_sgr_weighted2_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        t1: *const int16_t,
        t2: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        wt: *const int16_t,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_sgr_weighted1_16bpc_neon(
        dst: *mut pixel,
        dst_stride: ptrdiff_t,
        src: *const pixel,
        src_stride: ptrdiff_t,
        t1: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        wt: libc::c_int,
        bitdepth_max: libc::c_int,
    );
}
#[cfg(all(feature = "asm", target_arch = "arm"))]
extern "C" {
    fn dav1d_wiener_filter_h_16bpc_neon(
        dst: *mut int16_t,
        left: *const [pixel; 4],
        src: *const pixel,
        stride: ptrdiff_t,
        fh: *const int16_t,
        w: intptr_t,
        h: libc::c_int,
        edges: LrEdgeFlags,
        bitdepth_max: libc::c_int,
    );
    fn dav1d_wiener_filter_v_16bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        mid: *const int16_t,
        w: libc::c_int,
        h: libc::c_int,
        fv: *const int16_t,
        edges: LrEdgeFlags,
        mid_stride: ptrdiff_t,
        bitdepth_max: libc::c_int,
    );
}

pub type pixel = uint16_t;
pub type coef = int32_t;
pub type const_left_pixel_row = *const [pixel; 4];

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
#[rustfmt::skip]
use crate::{
    src::looprestoration::LrEdgeFlags,
    src::looprestoration::LooprestorationParams,
};

use crate::src::looprestoration::sgr_3x3_c_erased;
use crate::src::looprestoration::sgr_5x5_c_erased;
use crate::src::looprestoration::sgr_mix_c_erased;
use crate::src::looprestoration::wiener_c_erased;
use crate::src::looprestoration::Dav1dLoopRestorationDSPContext;

#[inline]
unsafe extern "C" fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 != 0 {
        unreachable!();
    }
    return x >> 1;
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
#[inline(always)]
unsafe extern "C" fn loop_restoration_dsp_init_x86(
    c: *mut Dav1dLoopRestorationDSPContext,
    bpc: libc::c_int,
) {
    // TODO(randomPoison): Import temporarily needed until init fns are deduplicated.
    use crate::src::looprestoration::*;
    use crate::src::x86::cpu::*;

    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_X86_CPU_FLAG_SSE2 == 0 {
        return;
    }

    if flags & DAV1D_X86_CPU_FLAG_SSSE3 == 0 {
        return;
    }

    (*c).wiener[0] = dav1d_wiener_filter7_16bpc_ssse3;
    (*c).wiener[1] = dav1d_wiener_filter5_16bpc_ssse3;

    if bpc == 10 {
        (*c).sgr[0] = dav1d_sgr_filter_5x5_16bpc_ssse3;
        (*c).sgr[1] = dav1d_sgr_filter_3x3_16bpc_ssse3;
        (*c).sgr[2] = dav1d_sgr_filter_mix_16bpc_ssse3;
    }

    #[cfg(target_arch = "x86_64")]
    {
        if flags & DAV1D_X86_CPU_FLAG_AVX2 == 0 {
            return;
        }

        (*c).wiener[0] = dav1d_wiener_filter7_16bpc_avx2;
        (*c).wiener[1] = dav1d_wiener_filter5_16bpc_avx2;

        if bpc == 10 {
            (*c).sgr[0] = dav1d_sgr_filter_5x5_16bpc_avx2;
            (*c).sgr[1] = dav1d_sgr_filter_3x3_16bpc_avx2;
            (*c).sgr[2] = dav1d_sgr_filter_mix_16bpc_avx2;
        }

        if flags & DAV1D_X86_CPU_FLAG_AVX512ICL == 0 {
            return;
        }

        (*c).wiener[0] = dav1d_wiener_filter7_16bpc_avx512icl;
        (*c).wiener[1] = dav1d_wiener_filter5_16bpc_avx512icl;

        if bpc == 10 {
            (*c).sgr[0] = dav1d_sgr_filter_5x5_16bpc_avx512icl;
            (*c).sgr[1] = dav1d_sgr_filter_3x3_16bpc_avx512icl;
            (*c).sgr[2] = dav1d_sgr_filter_mix_16bpc_avx512icl;
        }
    }
}

#[cfg(all(feature = "asm", target_arch = "arm"))]
unsafe extern "C" fn wiener_filter_neon_erased(
    p: *mut libc::c_void,
    stride: ptrdiff_t,
    left: *const libc::c_void,
    lpf: *const libc::c_void,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: libc::c_int,
) {
    wiener_filter_neon(
        p.cast(),
        stride,
        left.cast(),
        lpf.cast(),
        w,
        h,
        params,
        edges,
        bitdepth_max,
    )
}

#[cfg(all(feature = "asm", target_arch = "arm"))]
unsafe extern "C" fn wiener_filter_neon(
    dst: *mut pixel,
    stride: ptrdiff_t,
    left: *const [pixel; 4],
    mut lpf: *const pixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: libc::c_int,
) {
    use crate::src::looprestoration::LR_HAVE_BOTTOM;
    use crate::src::looprestoration::LR_HAVE_TOP;

    let filter: *const [int16_t; 8] = (*params).filter.0.as_ptr();
    let mut mid: Align16<[int16_t; 68 * 384]> = Align16([0; 68 * 384]);
    let mut mid_stride: libc::c_int = w + 7 & !7;
    dav1d_wiener_filter_h_16bpc_neon(
        &mut *mid.0.as_mut_ptr().offset((2 * mid_stride) as isize),
        left,
        dst,
        stride,
        (*filter.offset(0)).as_ptr(),
        w as intptr_t,
        h,
        edges,
        bitdepth_max,
    );
    if edges & LR_HAVE_TOP != 0 {
        dav1d_wiener_filter_h_16bpc_neon(
            mid.0.as_mut_ptr(),
            core::ptr::null(),
            lpf,
            stride,
            (*filter.offset(0)).as_ptr(),
            w as intptr_t,
            2,
            edges,
            bitdepth_max,
        );
    }
    if edges & LR_HAVE_BOTTOM != 0 {
        dav1d_wiener_filter_h_16bpc_neon(
            &mut *mid
                .0
                .as_mut_ptr()
                .offset(((2 as libc::c_int + h) * mid_stride) as isize),
            core::ptr::null(),
            lpf.offset((6 * PXSTRIDE(stride)) as isize),
            stride,
            (*filter.offset(0)).as_ptr(),
            w as intptr_t,
            2,
            edges,
            bitdepth_max,
        );
    }
    dav1d_wiener_filter_v_16bpc_neon(
        dst,
        stride,
        &mut *mid
            .0
            .as_mut_ptr()
            .offset((2 as libc::c_int * mid_stride) as isize),
        w,
        h,
        (*filter.offset(1)).as_ptr(),
        edges,
        (mid_stride as usize * ::core::mem::size_of::<int16_t>()) as ptrdiff_t,
        bitdepth_max,
    );
}
#[cfg(feature = "asm")]
use crate::src::cpu::dav1d_get_cpu_flags;

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
#[inline(always)]
unsafe extern "C" fn loop_restoration_dsp_init_arm(
    c: *mut Dav1dLoopRestorationDSPContext,
    mut bpc: libc::c_int,
) {
    use crate::src::arm::cpu::DAV1D_ARM_CPU_FLAG_NEON;
    // TODO(randomPoison): Import temporarily needed until init fns are deduplicated.
    #[cfg(target_arch = "aarch64")]
    use crate::src::looprestoration::*;

    let flags: libc::c_uint = dav1d_get_cpu_flags();

    if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
        return;
    }

    cfg_if! {
        if #[cfg(target_arch = "aarch64")] {
            (*c).wiener[0] = dav1d_wiener_filter7_16bpc_neon;
            (*c).wiener[1] = dav1d_wiener_filter5_16bpc_neon;
        } else {
            (*c).wiener[0] = wiener_filter_neon_erased;
            (*c).wiener[1] = wiener_filter_neon_erased;
        }
    }

    if bpc == 10 {
        (*c).sgr[0] = sgr_filter_5x5_neon_erased;
        (*c).sgr[1] = sgr_filter_3x3_neon_erased;
        (*c).sgr[2] = sgr_filter_mix_neon_erased;
    }
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn sgr_filter_3x3_neon_erased(
    p: *mut libc::c_void,
    stride: ptrdiff_t,
    left: *const libc::c_void,
    lpf: *const libc::c_void,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: libc::c_int,
) {
    sgr_filter_3x3_neon(
        p.cast(),
        stride,
        left.cast(),
        lpf.cast(),
        w,
        h,
        params,
        edges,
        bitdepth_max,
    )
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn sgr_filter_3x3_neon(
    dst: *mut pixel,
    stride: ptrdiff_t,
    left: *const [pixel; 4],
    mut lpf: *const pixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: libc::c_int,
) {
    use crate::include::common::bitdepth::BitDepth;
    use crate::src::looprestoration::dav1d_sgr_filter1_neon;

    let mut tmp: Align16<[int16_t; 24576]> = Align16([0; 24576]);
    dav1d_sgr_filter1_neon(
        tmp.0.as_mut_ptr(),
        dst,
        stride,
        left,
        lpf,
        w,
        h,
        (*params).sgr.s1 as libc::c_int,
        edges,
        BitDepth16::from_c(bitdepth_max),
    );
    dav1d_sgr_weighted1_16bpc_neon(
        dst,
        stride,
        dst,
        stride,
        tmp.0.as_mut_ptr(),
        w,
        h,
        (*params).sgr.w1 as libc::c_int,
        bitdepth_max,
    );
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn sgr_filter_5x5_neon_erased(
    p: *mut libc::c_void,
    stride: ptrdiff_t,
    left: *const libc::c_void,
    lpf: *const libc::c_void,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: libc::c_int,
) {
    sgr_filter_5x5_neon(
        p.cast(),
        stride,
        left.cast(),
        lpf.cast(),
        w,
        h,
        params,
        edges,
        bitdepth_max,
    )
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn sgr_filter_5x5_neon(
    dst: *mut pixel,
    stride: ptrdiff_t,
    left: *const [pixel; 4],
    mut lpf: *const pixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: libc::c_int,
) {
    use crate::include::common::bitdepth::BitDepth;
    use crate::src::looprestoration::dav1d_sgr_filter2_neon;

    let mut tmp: Align16<[int16_t; 24576]> = Align16([0; 24576]);
    dav1d_sgr_filter2_neon(
        tmp.0.as_mut_ptr(),
        dst,
        stride,
        left,
        lpf,
        w,
        h,
        (*params).sgr.s0 as libc::c_int,
        edges,
        BitDepth16::from_c(bitdepth_max),
    );
    dav1d_sgr_weighted1_16bpc_neon(
        dst,
        stride,
        dst,
        stride,
        tmp.0.as_mut_ptr(),
        w,
        h,
        (*params).sgr.w0 as libc::c_int,
        bitdepth_max,
    );
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn sgr_filter_mix_neon_erased(
    p: *mut libc::c_void,
    stride: ptrdiff_t,
    left: *const libc::c_void,
    lpf: *const libc::c_void,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: libc::c_int,
) {
    sgr_filter_mix_neon(
        p.cast(),
        stride,
        left.cast(),
        lpf.cast(),
        w,
        h,
        params,
        edges,
        bitdepth_max,
    )
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn sgr_filter_mix_neon(
    dst: *mut pixel,
    stride: ptrdiff_t,
    left: *const [pixel; 4],
    mut lpf: *const pixel,
    w: libc::c_int,
    h: libc::c_int,
    params: *const LooprestorationParams,
    edges: LrEdgeFlags,
    bitdepth_max: libc::c_int,
) {
    use crate::include::common::bitdepth::BitDepth;
    use crate::src::looprestoration::dav1d_sgr_filter1_neon;
    use crate::src::looprestoration::dav1d_sgr_filter2_neon;

    let mut tmp1: Align16<[int16_t; 24576]> = Align16([0; 24576]);
    let mut tmp2: Align16<[int16_t; 24576]> = Align16([0; 24576]);
    dav1d_sgr_filter2_neon(
        tmp1.0.as_mut_ptr(),
        dst,
        stride,
        left,
        lpf,
        w,
        h,
        (*params).sgr.s0 as libc::c_int,
        edges,
        BitDepth16::from_c(bitdepth_max),
    );
    dav1d_sgr_filter1_neon(
        tmp2.0.as_mut_ptr(),
        dst,
        stride,
        left,
        lpf,
        w,
        h,
        (*params).sgr.s1 as libc::c_int,
        edges,
        BitDepth16::from_c(bitdepth_max),
    );
    let wt: [int16_t; 2] = [(*params).sgr.w0, (*params).sgr.w1];
    dav1d_sgr_weighted2_16bpc_neon(
        dst,
        stride,
        dst,
        stride,
        tmp1.0.as_mut_ptr(),
        tmp2.0.as_mut_ptr(),
        w,
        h,
        wt.as_ptr(),
        bitdepth_max,
    );
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_loop_restoration_dsp_init_16bpc(
    c: *mut Dav1dLoopRestorationDSPContext,
    _bpc: libc::c_int,
) {
    (*c).wiener[1] = wiener_c_erased::<BitDepth16>;
    (*c).wiener[0] = (*c).wiener[1];
    (*c).sgr[0] = sgr_5x5_c_erased::<BitDepth16>;
    (*c).sgr[1] = sgr_3x3_c_erased::<BitDepth16>;
    (*c).sgr[2] = sgr_mix_c_erased::<BitDepth16>;

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            loop_restoration_dsp_init_x86(c, _bpc);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]{
            loop_restoration_dsp_init_arm(c, _bpc);
        }
    }
}
