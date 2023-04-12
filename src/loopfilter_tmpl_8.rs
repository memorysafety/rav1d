use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
use cfg_if::cfg_if;
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
}

#[cfg(feature = "asm")]
extern "C" {
    static mut dav1d_cpu_flags_mask: libc::c_uint;
    static mut dav1d_cpu_flags: libc::c_uint;
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
extern "C" {
    fn dav1d_lpf_v_sb_uv_8bpc_avx512icl(
        dst: *mut pixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
    );
    fn dav1d_lpf_h_sb_uv_8bpc_avx512icl(
        dst: *mut pixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
    );
    fn dav1d_lpf_v_sb_y_8bpc_avx512icl(
        dst: *mut pixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
    );
    fn dav1d_lpf_h_sb_y_8bpc_avx512icl(
        dst: *mut pixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
    );
    fn dav1d_lpf_v_sb_uv_8bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
    );
    fn dav1d_lpf_h_sb_uv_8bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
    );
    fn dav1d_lpf_v_sb_y_8bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
    );
    fn dav1d_lpf_h_sb_y_8bpc_avx2(
        dst: *mut pixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
    );
    fn dav1d_lpf_v_sb_uv_8bpc_ssse3(
        dst: *mut pixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
    );
    fn dav1d_lpf_h_sb_uv_8bpc_ssse3(
        dst: *mut pixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
    );
    fn dav1d_lpf_v_sb_y_8bpc_ssse3(
        dst: *mut pixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
    );
    fn dav1d_lpf_h_sb_y_8bpc_ssse3(
        dst: *mut pixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
    );
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
extern "C" {
    fn dav1d_lpf_h_sb_uv_8bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
    );
    fn dav1d_lpf_v_sb_y_8bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
    );
    fn dav1d_lpf_h_sb_y_8bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
    );
    fn dav1d_lpf_v_sb_uv_8bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        mask: *const uint32_t,
        lvl: *const [uint8_t; 4],
        lvl_stride: ptrdiff_t,
        lut: *const Av1FilterLUT,
        w: libc::c_int,
    );
}

cfg_if! {
    if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        use crate::src::x86::cpu::DAV1D_X86_CPU_FLAG_AVX512ICL;
        use crate::src::x86::cpu::DAV1D_X86_CPU_FLAG_AVX2;
        use crate::src::x86::cpu::DAV1D_X86_CPU_FLAG_SSSE3;
    }
}
pub type pixel = uint8_t;
use crate::src::lf_mask::Av1FilterLUT;
pub type loopfilter_sb_fn = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const uint32_t,
        *const [uint8_t; 4],
        ptrdiff_t,
        *const Av1FilterLUT,
        libc::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dLoopFilterDSPContext {
    pub loop_filter_sb: [[loopfilter_sb_fn; 2]; 2],
}
use crate::include::common::intops::imin;
use crate::include::common::intops::iclip;
use crate::include::common::intops::iclip_u8;
#[inline(never)]
unsafe extern "C" fn loop_filter(
    mut dst: *mut pixel,
    mut E: libc::c_int,
    mut I: libc::c_int,
    mut H: libc::c_int,
    stridea: ptrdiff_t,
    strideb: ptrdiff_t,
    wd: libc::c_int,
) {
    let bitdepth_min_8: libc::c_int = 8 as libc::c_int - 8 as libc::c_int;
    let F: libc::c_int = (1 as libc::c_int) << bitdepth_min_8;
    E <<= bitdepth_min_8;
    I <<= bitdepth_min_8;
    H <<= bitdepth_min_8;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let mut p6: libc::c_int = 0;
        let mut p5: libc::c_int = 0;
        let mut p4: libc::c_int = 0;
        let mut p3: libc::c_int = 0;
        let mut p2: libc::c_int = 0;
        let mut p1: libc::c_int = *dst
            .offset((strideb * -(2 as libc::c_int) as isize) as isize)
            as libc::c_int;
        let mut p0: libc::c_int = *dst
            .offset((strideb * -(1 as libc::c_int) as isize) as isize)
            as libc::c_int;
        let mut q0: libc::c_int = *dst
            .offset((strideb * 0) as isize)
            as libc::c_int;
        let mut q1: libc::c_int = *dst
            .offset((strideb * 1) as isize)
            as libc::c_int;
        let mut q2: libc::c_int = 0;
        let mut q3: libc::c_int = 0;
        let mut q4: libc::c_int = 0;
        let mut q5: libc::c_int = 0;
        let mut q6: libc::c_int = 0;
        let mut fm: libc::c_int = 0;
        let mut flat8out: libc::c_int = 0;
        let mut flat8in: libc::c_int = 0;
        fm = (abs(p1 - p0) <= I && abs(q1 - q0) <= I
            && abs(p0 - q0) * 2 as libc::c_int + (abs(p1 - q1) >> 1 as libc::c_int) <= E)
            as libc::c_int;
        if wd > 4 as libc::c_int {
            p2 = *dst.offset((strideb * -(3 as libc::c_int) as isize) as isize)
                as libc::c_int;
            q2 = *dst.offset((strideb * 2) as isize)
                as libc::c_int;
            fm &= (abs(p2 - p1) <= I && abs(q2 - q1) <= I) as libc::c_int;
            if wd > 6 as libc::c_int {
                p3 = *dst
                    .offset((strideb * -(4 as libc::c_int) as isize) as isize)
                    as libc::c_int;
                q3 = *dst.offset((strideb * 3) as isize)
                    as libc::c_int;
                fm &= (abs(p3 - p2) <= I && abs(q3 - q2) <= I) as libc::c_int;
            }
        }
        if !(fm == 0) {
            if wd >= 16 as libc::c_int {
                p6 = *dst
                    .offset((strideb * -(7 as libc::c_int) as isize) as isize)
                    as libc::c_int;
                p5 = *dst
                    .offset((strideb * -(6 as libc::c_int) as isize) as isize)
                    as libc::c_int;
                p4 = *dst
                    .offset((strideb * -(5 as libc::c_int) as isize) as isize)
                    as libc::c_int;
                q4 = *dst.offset((strideb * 4) as isize)
                    as libc::c_int;
                q5 = *dst.offset((strideb * 5) as isize)
                    as libc::c_int;
                q6 = *dst.offset((strideb * 6) as isize)
                    as libc::c_int;
                flat8out = (abs(p6 - p0) <= F && abs(p5 - p0) <= F && abs(p4 - p0) <= F
                    && abs(q4 - q0) <= F && abs(q5 - q0) <= F && abs(q6 - q0) <= F)
                    as libc::c_int;
            }
            if wd >= 6 as libc::c_int {
                flat8in = (abs(p2 - p0) <= F && abs(p1 - p0) <= F && abs(q1 - q0) <= F
                    && abs(q2 - q0) <= F) as libc::c_int;
            }
            if wd >= 8 as libc::c_int {
                flat8in &= (abs(p3 - p0) <= F && abs(q3 - q0) <= F) as libc::c_int;
            }
            if wd >= 16 as libc::c_int && flat8out & flat8in != 0 {
                *dst
                    .offset(
                        (strideb * -(6 as libc::c_int) as isize) as isize,
                    ) = (p6 + p6 + p6 + p6 + p6 + p6 * 2 as libc::c_int
                    + p5 * 2 as libc::c_int + p4 * 2 as libc::c_int + p3 + p2 + p1 + p0
                    + q0 + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * -(5 as libc::c_int) as isize) as isize,
                    ) = (p6 + p6 + p6 + p6 + p6 + p5 * 2 as libc::c_int
                    + p4 * 2 as libc::c_int + p3 * 2 as libc::c_int + p2 + p1 + p0 + q0
                    + q1 + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * -(4 as libc::c_int) as isize) as isize,
                    ) = (p6 + p6 + p6 + p6 + p5 + p4 * 2 as libc::c_int
                    + p3 * 2 as libc::c_int + p2 * 2 as libc::c_int + p1 + p0 + q0 + q1
                    + q2 + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * -(3 as libc::c_int) as isize) as isize,
                    ) = (p6 + p6 + p6 + p5 + p4 + p3 * 2 as libc::c_int
                    + p2 * 2 as libc::c_int + p1 * 2 as libc::c_int + p0 + q0 + q1 + q2
                    + q3 + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * -(2 as libc::c_int) as isize) as isize,
                    ) = (p6 + p6 + p5 + p4 + p3 + p2 * 2 as libc::c_int
                    + p1 * 2 as libc::c_int + p0 * 2 as libc::c_int + q0 + q1 + q2 + q3
                    + q4 + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * -(1 as libc::c_int) as isize) as isize,
                    ) = (p6 + p5 + p4 + p3 + p2 + p1 * 2 as libc::c_int
                    + p0 * 2 as libc::c_int + q0 * 2 as libc::c_int + q1 + q2 + q3 + q4
                    + q5 + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * 0) as isize,
                    ) = (p5 + p4 + p3 + p2 + p1 + p0 * 2 as libc::c_int
                    + q0 * 2 as libc::c_int + q1 * 2 as libc::c_int + q2 + q3 + q4 + q5
                    + q6 + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * 1) as isize,
                    ) = (p4 + p3 + p2 + p1 + p0 + q0 * 2 as libc::c_int
                    + q1 * 2 as libc::c_int + q2 * 2 as libc::c_int + q3 + q4 + q5 + q6
                    + q6 + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * 2) as isize,
                    ) = (p3 + p2 + p1 + p0 + q0 + q1 * 2 as libc::c_int
                    + q2 * 2 as libc::c_int + q3 * 2 as libc::c_int + q4 + q5 + q6 + q6
                    + q6 + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * 3) as isize,
                    ) = (p2 + p1 + p0 + q0 + q1 + q2 * 2 as libc::c_int
                    + q3 * 2 as libc::c_int + q4 * 2 as libc::c_int + q5 + q6 + q6 + q6
                    + q6 + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * 4) as isize,
                    ) = (p1 + p0 + q0 + q1 + q2 + q3 * 2 as libc::c_int
                    + q4 * 2 as libc::c_int + q5 * 2 as libc::c_int + q6 + q6 + q6 + q6
                    + q6 + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * 5) as isize,
                    ) = (p0 + q0 + q1 + q2 + q3 + q4 * 2 as libc::c_int
                    + q5 * 2 as libc::c_int + q6 * 2 as libc::c_int + q6 + q6 + q6 + q6
                    + q6 + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
            } else if wd >= 8 as libc::c_int && flat8in != 0 {
                *dst
                    .offset(
                        (strideb * -(3 as libc::c_int) as isize) as isize,
                    ) = (p3 + p3 + p3 + 2 as libc::c_int * p2 + p1 + p0 + q0
                    + 4 as libc::c_int >> 3 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * -(2 as libc::c_int) as isize) as isize,
                    ) = (p3 + p3 + p2 + 2 as libc::c_int * p1 + p0 + q0 + q1
                    + 4 as libc::c_int >> 3 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * -(1 as libc::c_int) as isize) as isize,
                    ) = (p3 + p2 + p1 + 2 as libc::c_int * p0 + q0 + q1 + q2
                    + 4 as libc::c_int >> 3 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * 0) as isize,
                    ) = (p2 + p1 + p0 + 2 as libc::c_int * q0 + q1 + q2 + q3
                    + 4 as libc::c_int >> 3 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * 1) as isize,
                    ) = (p1 + p0 + q0 + 2 as libc::c_int * q1 + q2 + q3 + q3
                    + 4 as libc::c_int >> 3 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * 2) as isize,
                    ) = (p0 + q0 + q1 + 2 as libc::c_int * q2 + q3 + q3 + q3
                    + 4 as libc::c_int >> 3 as libc::c_int) as pixel;
            } else if wd == 6 as libc::c_int && flat8in != 0 {
                *dst
                    .offset(
                        (strideb * -(2 as libc::c_int) as isize) as isize,
                    ) = (p2 + 2 as libc::c_int * p2 + 2 as libc::c_int * p1
                    + 2 as libc::c_int * p0 + q0 + 4 as libc::c_int >> 3 as libc::c_int)
                    as pixel;
                *dst
                    .offset(
                        (strideb * -(1 as libc::c_int) as isize) as isize,
                    ) = (p2 + 2 as libc::c_int * p1 + 2 as libc::c_int * p0
                    + 2 as libc::c_int * q0 + q1 + 4 as libc::c_int >> 3 as libc::c_int)
                    as pixel;
                *dst
                    .offset(
                        (strideb * 0) as isize,
                    ) = (p1 + 2 as libc::c_int * p0 + 2 as libc::c_int * q0
                    + 2 as libc::c_int * q1 + q2 + 4 as libc::c_int >> 3 as libc::c_int)
                    as pixel;
                *dst
                    .offset(
                        (strideb * 1) as isize,
                    ) = (p0 + 2 as libc::c_int * q0 + 2 as libc::c_int * q1
                    + 2 as libc::c_int * q2 + q2 + 4 as libc::c_int >> 3 as libc::c_int)
                    as pixel;
            } else {
                let hev: libc::c_int = (abs(p1 - p0) > H || abs(q1 - q0) > H)
                    as libc::c_int;
                if hev != 0 {
                    let mut f: libc::c_int = iclip(
                        p1 - q1,
                        -(128 as libc::c_int) * ((1 as libc::c_int) << bitdepth_min_8),
                        128 as libc::c_int * ((1 as libc::c_int) << bitdepth_min_8)
                            - 1 as libc::c_int,
                    );
                    let mut f1: libc::c_int = 0;
                    let mut f2: libc::c_int = 0;
                    f = iclip(
                        3 as libc::c_int * (q0 - p0) + f,
                        -(128 as libc::c_int) * ((1 as libc::c_int) << bitdepth_min_8),
                        128 as libc::c_int * ((1 as libc::c_int) << bitdepth_min_8)
                            - 1 as libc::c_int,
                    );
                    f1 = imin(
                        f + 4 as libc::c_int,
                        ((128 as libc::c_int) << bitdepth_min_8) - 1 as libc::c_int,
                    ) >> 3 as libc::c_int;
                    f2 = imin(
                        f + 3 as libc::c_int,
                        ((128 as libc::c_int) << bitdepth_min_8) - 1 as libc::c_int,
                    ) >> 3 as libc::c_int;
                    *dst
                        .offset(
                            (strideb * -(1 as libc::c_int) as isize) as isize,
                        ) = iclip_u8(p0 + f2) as pixel;
                    *dst
                        .offset(
                            (strideb * 0) as isize,
                        ) = iclip_u8(q0 - f1) as pixel;
                } else {
                    let mut f_0: libc::c_int = iclip(
                        3 as libc::c_int * (q0 - p0),
                        -(128 as libc::c_int) * ((1 as libc::c_int) << bitdepth_min_8),
                        128 as libc::c_int * ((1 as libc::c_int) << bitdepth_min_8)
                            - 1 as libc::c_int,
                    );
                    let mut f1_0: libc::c_int = 0;
                    let mut f2_0: libc::c_int = 0;
                    f1_0 = imin(
                        f_0 + 4 as libc::c_int,
                        ((128 as libc::c_int) << bitdepth_min_8) - 1 as libc::c_int,
                    ) >> 3 as libc::c_int;
                    f2_0 = imin(
                        f_0 + 3 as libc::c_int,
                        ((128 as libc::c_int) << bitdepth_min_8) - 1 as libc::c_int,
                    ) >> 3 as libc::c_int;
                    *dst
                        .offset(
                            (strideb * -(1 as libc::c_int) as isize) as isize,
                        ) = iclip_u8(p0 + f2_0) as pixel;
                    *dst
                        .offset(
                            (strideb * 0) as isize,
                        ) = iclip_u8(q0 - f1_0) as pixel;
                    f_0 = f1_0 + 1 as libc::c_int >> 1 as libc::c_int;
                    *dst
                        .offset(
                            (strideb * -(2 as libc::c_int) as isize) as isize,
                        ) = iclip_u8(p1 + f_0) as pixel;
                    *dst
                        .offset(
                            (strideb * 1) as isize,
                        ) = iclip_u8(q1 - f_0) as pixel;
                }
            }
        }
        i += 1;
        dst = dst.offset(stridea as isize);
    }
}
unsafe extern "C" fn loop_filter_h_sb128y_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    vmask: *const uint32_t,
    mut l: *const [uint8_t; 4],
    mut b4_stride: ptrdiff_t,
    mut lut: *const Av1FilterLUT,
    _h: libc::c_int,
) {
    let vm: libc::c_uint = *vmask.offset(0 as libc::c_int as isize)
        | *vmask.offset(1 as libc::c_int as isize)
        | *vmask.offset(2 as libc::c_int as isize);
    let mut y: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    while vm & !y.wrapping_sub(1 as libc::c_int as libc::c_uint) != 0 {
        if vm & y != 0 {
            let L: libc::c_int = if (*l
                .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
                as libc::c_int != 0
            {
                (*l.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
                    as libc::c_int
            } else {
                (*l.offset(-(1 as libc::c_int) as isize))[0 as libc::c_int as usize]
                    as libc::c_int
            };
            if !(L == 0) {
                let H: libc::c_int = L >> 4 as libc::c_int;
                let E: libc::c_int = (*lut).e[L as usize] as libc::c_int;
                let I: libc::c_int = (*lut).i[L as usize] as libc::c_int;
                let idx: libc::c_int = if *vmask.offset(2 as libc::c_int as isize) & y
                    != 0
                {
                    2 as libc::c_int
                } else {
                    (*vmask.offset(1 as libc::c_int as isize) & y != 0) as libc::c_int
                };
                loop_filter(
                    dst,
                    E,
                    I,
                    H,
                    stride,
                    1 as libc::c_int as ptrdiff_t,
                    (4 as libc::c_int) << idx,
                );
            }
        }
        y <<= 1 as libc::c_int;
        dst = dst.offset((4 * stride) as isize);
        l = l.offset(b4_stride as isize);
    }
}
unsafe extern "C" fn loop_filter_v_sb128y_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    vmask: *const uint32_t,
    mut l: *const [uint8_t; 4],
    mut b4_stride: ptrdiff_t,
    mut lut: *const Av1FilterLUT,
    _w: libc::c_int,
) {
    let vm: libc::c_uint = *vmask.offset(0 as libc::c_int as isize)
        | *vmask.offset(1 as libc::c_int as isize)
        | *vmask.offset(2 as libc::c_int as isize);
    let mut x: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    while vm & !x.wrapping_sub(1 as libc::c_int as libc::c_uint) != 0 {
        if vm & x != 0 {
            let L: libc::c_int = if (*l
                .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
                as libc::c_int != 0
            {
                (*l.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
                    as libc::c_int
            } else {
                (*l.offset(-b4_stride as isize))[0 as libc::c_int as usize]
                    as libc::c_int
            };
            if !(L == 0) {
                let H: libc::c_int = L >> 4 as libc::c_int;
                let E: libc::c_int = (*lut).e[L as usize] as libc::c_int;
                let I: libc::c_int = (*lut).i[L as usize] as libc::c_int;
                let idx: libc::c_int = if *vmask.offset(2 as libc::c_int as isize) & x
                    != 0
                {
                    2 as libc::c_int
                } else {
                    (*vmask.offset(1 as libc::c_int as isize) & x != 0) as libc::c_int
                };
                loop_filter(
                    dst,
                    E,
                    I,
                    H,
                    1 as libc::c_int as ptrdiff_t,
                    stride,
                    (4 as libc::c_int) << idx,
                );
            }
        }
        x <<= 1 as libc::c_int;
        dst = dst.offset(4 as libc::c_int as isize);
        l = l.offset(1);
    }
}
unsafe extern "C" fn loop_filter_h_sb128uv_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    vmask: *const uint32_t,
    mut l: *const [uint8_t; 4],
    mut b4_stride: ptrdiff_t,
    mut lut: *const Av1FilterLUT,
    _h: libc::c_int,
) {
    let vm: libc::c_uint = *vmask.offset(0 as libc::c_int as isize)
        | *vmask.offset(1 as libc::c_int as isize);
    let mut y: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    while vm & !y.wrapping_sub(1 as libc::c_int as libc::c_uint) != 0 {
        if vm & y != 0 {
            let L: libc::c_int = if (*l
                .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
                as libc::c_int != 0
            {
                (*l.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
                    as libc::c_int
            } else {
                (*l.offset(-(1 as libc::c_int) as isize))[0 as libc::c_int as usize]
                    as libc::c_int
            };
            if !(L == 0) {
                let H: libc::c_int = L >> 4 as libc::c_int;
                let E: libc::c_int = (*lut).e[L as usize] as libc::c_int;
                let I: libc::c_int = (*lut).i[L as usize] as libc::c_int;
                let idx: libc::c_int = (*vmask.offset(1 as libc::c_int as isize) & y
                    != 0) as libc::c_int;
                loop_filter(
                    dst,
                    E,
                    I,
                    H,
                    stride,
                    1 as libc::c_int as ptrdiff_t,
                    4 as libc::c_int + 2 as libc::c_int * idx,
                );
            }
        }
        y <<= 1 as libc::c_int;
        dst = dst.offset((4 * stride) as isize);
        l = l.offset(b4_stride as isize);
    }
}
unsafe extern "C" fn loop_filter_v_sb128uv_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    vmask: *const uint32_t,
    mut l: *const [uint8_t; 4],
    mut b4_stride: ptrdiff_t,
    mut lut: *const Av1FilterLUT,
    _w: libc::c_int,
) {
    let vm: libc::c_uint = *vmask.offset(0 as libc::c_int as isize)
        | *vmask.offset(1 as libc::c_int as isize);
    let mut x: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    while vm & !x.wrapping_sub(1 as libc::c_int as libc::c_uint) != 0 {
        if vm & x != 0 {
            let L: libc::c_int = if (*l
                .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
                as libc::c_int != 0
            {
                (*l.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
                    as libc::c_int
            } else {
                (*l.offset(-b4_stride as isize))[0 as libc::c_int as usize]
                    as libc::c_int
            };
            if !(L == 0) {
                let H: libc::c_int = L >> 4 as libc::c_int;
                let E: libc::c_int = (*lut).e[L as usize] as libc::c_int;
                let I: libc::c_int = (*lut).i[L as usize] as libc::c_int;
                let idx: libc::c_int = (*vmask.offset(1 as libc::c_int as isize) & x
                    != 0) as libc::c_int;
                loop_filter(
                    dst,
                    E,
                    I,
                    H,
                    1 as libc::c_int as ptrdiff_t,
                    stride,
                    4 as libc::c_int + 2 as libc::c_int * idx,
                );
            }
        }
        x <<= 1 as libc::c_int;
        dst = dst.offset(4 as libc::c_int as isize);
        l = l.offset(1);
    }
}

use crate::src::cpu::dav1d_get_cpu_flags;

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
#[inline(always)]
unsafe extern "C" fn loop_filter_dsp_init_x86(c: *mut Dav1dLoopFilterDSPContext) {
    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_X86_CPU_FLAG_SSSE3 == 0 {
        return;
    }

    (*c).loop_filter_sb[0][0] = Some(dav1d_lpf_h_sb_y_8bpc_ssse3);
    (*c).loop_filter_sb[0][1] = Some(dav1d_lpf_v_sb_y_8bpc_ssse3);
    (*c).loop_filter_sb[1][0] = Some(dav1d_lpf_h_sb_uv_8bpc_ssse3);
    (*c).loop_filter_sb[1][1] = Some(dav1d_lpf_v_sb_uv_8bpc_ssse3);

    if flags & DAV1D_X86_CPU_FLAG_AVX2 == 0 {
        return;
    }

    (*c).loop_filter_sb[0][0] = Some(dav1d_lpf_h_sb_y_8bpc_avx2);
    (*c).loop_filter_sb[0][1] = Some(dav1d_lpf_v_sb_y_8bpc_avx2);
    (*c).loop_filter_sb[1][0] = Some(dav1d_lpf_h_sb_uv_8bpc_avx2);
    (*c).loop_filter_sb[1][1] = Some(dav1d_lpf_v_sb_uv_8bpc_avx2);

    if flags & DAV1D_X86_CPU_FLAG_AVX512ICL == 0 {
        return;
    }

    (*c).loop_filter_sb[0][0] = Some(dav1d_lpf_h_sb_y_8bpc_avx512icl);
    (*c).loop_filter_sb[0][1] = Some(dav1d_lpf_v_sb_y_8bpc_avx512icl);
    (*c).loop_filter_sb[1][0] = Some(dav1d_lpf_h_sb_uv_8bpc_avx512icl);
    (*c).loop_filter_sb[1][1] = Some(dav1d_lpf_v_sb_uv_8bpc_avx512icl);
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
#[inline(always)]
unsafe extern "C" fn loop_filter_dsp_init_arm(c: *mut Dav1dLoopFilterDSPContext) {
    use crate::src::arm::cpu::DAV1D_ARM_CPU_FLAG_NEON;

    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
        return;
    }

    (*c).loop_filter_sb[0][0] = Some(dav1d_lpf_h_sb_y_8bpc_neon);
    (*c).loop_filter_sb[0][1] = Some(dav1d_lpf_v_sb_y_8bpc_neon);
    (*c).loop_filter_sb[1][0] = Some(dav1d_lpf_h_sb_uv_8bpc_neon);
    (*c).loop_filter_sb[1][1] = Some(dav1d_lpf_v_sb_uv_8bpc_neon);
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_loop_filter_dsp_init_8bpc(
    c: *mut Dav1dLoopFilterDSPContext,
) {
    (*c).loop_filter_sb[0][0] = Some(loop_filter_h_sb128y_c);
    (*c).loop_filter_sb[0][1] = Some(loop_filter_v_sb128y_c);
    (*c).loop_filter_sb[1][0] = Some(loop_filter_h_sb128uv_c);
    (*c).loop_filter_sb[1][1] = Some(loop_filter_v_sb128uv_c);

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            loop_filter_dsp_init_x86(c);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
            loop_filter_dsp_init_arm(c);
        }
    }
}
