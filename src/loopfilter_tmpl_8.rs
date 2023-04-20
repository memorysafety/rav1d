use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
use cfg_if::cfg_if;

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

pub type pixel = uint8_t;
use crate::src::lf_mask::Av1FilterLUT;
pub type loopfilter_sb_fn = Option<
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
use crate::include::common::intops::iclip;
use crate::include::common::intops::iclip_u8;
use crate::include::common::intops::imin;
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
    let bitdepth_min_8 = 8 - 8;
    let F = (1 as libc::c_int) << bitdepth_min_8;
    E <<= bitdepth_min_8;
    I <<= bitdepth_min_8;
    H <<= bitdepth_min_8;
    let mut i = 0;
    while i < 4 {
        let mut p6 = 0;
        let mut p5 = 0;
        let mut p4 = 0;
        let mut p3 = 0;
        let mut p2 = 0;
        let mut p1 = *dst.offset((strideb * -(2 as libc::c_int) as isize) as isize) as libc::c_int;
        let mut p0 = *dst.offset((strideb * -(1 as libc::c_int) as isize) as isize) as libc::c_int;
        let mut q0 = *dst.offset((strideb * 0) as isize) as libc::c_int;
        let mut q1 = *dst.offset((strideb * 1) as isize) as libc::c_int;
        let mut q2 = 0;
        let mut q3 = 0;
        let mut q4 = 0;
        let mut q5 = 0;
        let mut q6 = 0;
        let mut fm = 0;
        let mut flat8out = 0;
        let mut flat8in = 0;
        fm = ((p1 - p0).abs() <= I
            && (q1 - q0).abs() <= I
            && (p0 - q0).abs() * 2 + ((p1 - q1).abs() >> 1) <= E) as libc::c_int;
        if wd > 4 {
            p2 = *dst.offset((strideb * -(3 as libc::c_int) as isize) as isize) as libc::c_int;
            q2 = *dst.offset((strideb * 2) as isize) as libc::c_int;
            fm &= ((p2 - p1).abs() <= I && (q2 - q1).abs() <= I) as libc::c_int;
            if wd > 6 {
                p3 = *dst.offset((strideb * -(4 as libc::c_int) as isize) as isize) as libc::c_int;
                q3 = *dst.offset((strideb * 3) as isize) as libc::c_int;
                fm &= ((p3 - p2).abs() <= I && (q3 - q2).abs() <= I) as libc::c_int;
            }
        }
        if !(fm == 0) {
            if wd >= 16 {
                p6 = *dst.offset((strideb * -(7 as libc::c_int) as isize) as isize) as libc::c_int;
                p5 = *dst.offset((strideb * -(6 as libc::c_int) as isize) as isize) as libc::c_int;
                p4 = *dst.offset((strideb * -(5 as libc::c_int) as isize) as isize) as libc::c_int;
                q4 = *dst.offset((strideb * 4) as isize) as libc::c_int;
                q5 = *dst.offset((strideb * 5) as isize) as libc::c_int;
                q6 = *dst.offset((strideb * 6) as isize) as libc::c_int;
                flat8out = ((p6 - p0).abs() <= F
                    && (p5 - p0).abs() <= F
                    && (p4 - p0).abs() <= F
                    && (q4 - q0).abs() <= F
                    && (q5 - q0).abs() <= F
                    && (q6 - q0).abs() <= F) as libc::c_int;
            }
            if wd >= 6 {
                flat8in = ((p2 - p0).abs() <= F
                    && (p1 - p0).abs() <= F
                    && (q1 - q0).abs() <= F
                    && (q2 - q0).abs() <= F) as libc::c_int;
            }
            if wd >= 8 {
                flat8in &= ((p3 - p0).abs() <= F && (q3 - q0).abs() <= F) as libc::c_int;
            }
            if wd >= 16 && flat8out & flat8in != 0 {
                *dst.offset((strideb * -(6 as libc::c_int) as isize) as isize) =
                    (p6 + p6 + p6 + p6 + p6 + p6 * 2 + p5 * 2 + p4 * 2 + p3 + p2 + p1 + p0 + q0 + 8
                        >> 4) as pixel;
                *dst.offset((strideb * -(5 as libc::c_int) as isize) as isize) =
                    (p6 + p6 + p6 + p6 + p6 + p5 * 2 + p4 * 2 + p3 * 2 + p2 + p1 + p0 + q0 + q1 + 8
                        >> 4) as pixel;
                *dst.offset((strideb * -(4 as libc::c_int) as isize) as isize) =
                    (p6 + p6 + p6 + p6 + p5 + p4 * 2 + p3 * 2 + p2 * 2 + p1 + p0 + q0 + q1 + q2 + 8
                        >> 4) as pixel;
                *dst.offset((strideb * -(3 as libc::c_int) as isize) as isize) =
                    (p6 + p6 + p6 + p5 + p4 + p3 * 2 + p2 * 2 + p1 * 2 + p0 + q0 + q1 + q2 + q3 + 8
                        >> 4) as pixel;
                *dst.offset((strideb * -(2 as libc::c_int) as isize) as isize) =
                    (p6 + p6 + p5 + p4 + p3 + p2 * 2 + p1 * 2 + p0 * 2 + q0 + q1 + q2 + q3 + q4 + 8
                        >> 4) as pixel;
                *dst.offset((strideb * -(1 as libc::c_int) as isize) as isize) =
                    (p6 + p5 + p4 + p3 + p2 + p1 * 2 + p0 * 2 + q0 * 2 + q1 + q2 + q3 + q4 + q5 + 8
                        >> 4) as pixel;
                *dst.offset((strideb * 0) as isize) =
                    (p5 + p4 + p3 + p2 + p1 + p0 * 2 + q0 * 2 + q1 * 2 + q2 + q3 + q4 + q5 + q6 + 8
                        >> 4) as pixel;
                *dst.offset((strideb * 1) as isize) =
                    (p4 + p3 + p2 + p1 + p0 + q0 * 2 + q1 * 2 + q2 * 2 + q3 + q4 + q5 + q6 + q6 + 8
                        >> 4) as pixel;
                *dst.offset((strideb * 2) as isize) =
                    (p3 + p2 + p1 + p0 + q0 + q1 * 2 + q2 * 2 + q3 * 2 + q4 + q5 + q6 + q6 + q6 + 8
                        >> 4) as pixel;
                *dst.offset((strideb * 3) as isize) =
                    (p2 + p1 + p0 + q0 + q1 + q2 * 2 + q3 * 2 + q4 * 2 + q5 + q6 + q6 + q6 + q6 + 8
                        >> 4) as pixel;
                *dst.offset((strideb * 4) as isize) =
                    (p1 + p0 + q0 + q1 + q2 + q3 * 2 + q4 * 2 + q5 * 2 + q6 + q6 + q6 + q6 + q6 + 8
                        >> 4) as pixel;
                *dst.offset((strideb * 5) as isize) =
                    (p0 + q0 + q1 + q2 + q3 + q4 * 2 + q5 * 2 + q6 * 2 + q6 + q6 + q6 + q6 + q6 + 8
                        >> 4) as pixel;
            } else if wd >= 8 && flat8in != 0 {
                *dst.offset((strideb * -(3 as libc::c_int) as isize) as isize) =
                    (p3 + p3 + p3 + 2 * p2 + p1 + p0 + q0 + 4 >> 3) as pixel;
                *dst.offset((strideb * -(2 as libc::c_int) as isize) as isize) =
                    (p3 + p3 + p2 + 2 * p1 + p0 + q0 + q1 + 4 >> 3) as pixel;
                *dst.offset((strideb * -(1 as libc::c_int) as isize) as isize) =
                    (p3 + p2 + p1 + 2 * p0 + q0 + q1 + q2 + 4 >> 3) as pixel;
                *dst.offset((strideb * 0) as isize) =
                    (p2 + p1 + p0 + 2 * q0 + q1 + q2 + q3 + 4 >> 3) as pixel;
                *dst.offset((strideb * 1) as isize) =
                    (p1 + p0 + q0 + 2 * q1 + q2 + q3 + q3 + 4 >> 3) as pixel;
                *dst.offset((strideb * 2) as isize) =
                    (p0 + q0 + q1 + 2 * q2 + q3 + q3 + q3 + 4 >> 3) as pixel;
            } else if wd == 6 && flat8in != 0 {
                *dst.offset((strideb * -(2 as libc::c_int) as isize) as isize) =
                    (p2 + 2 * p2 + 2 * p1 + 2 * p0 + q0 + 4 >> 3) as pixel;
                *dst.offset((strideb * -(1 as libc::c_int) as isize) as isize) =
                    (p2 + 2 * p1 + 2 * p0 + 2 * q0 + q1 + 4 >> 3) as pixel;
                *dst.offset((strideb * 0) as isize) =
                    (p1 + 2 * p0 + 2 * q0 + 2 * q1 + q2 + 4 >> 3) as pixel;
                *dst.offset((strideb * 1) as isize) =
                    (p0 + 2 * q0 + 2 * q1 + 2 * q2 + q2 + 4 >> 3) as pixel;
            } else {
                let hev = ((p1 - p0).abs() > H || (q1 - q0).abs() > H) as libc::c_int;
                if hev != 0 {
                    let mut f = iclip(
                        p1 - q1,
                        -(128 as libc::c_int) * ((1 as libc::c_int) << bitdepth_min_8),
                        128 * ((1 as libc::c_int) << bitdepth_min_8) - 1,
                    );
                    let mut f1 = 0;
                    let mut f2 = 0;
                    f = iclip(
                        3 * (q0 - p0) + f,
                        -(128 as libc::c_int) * ((1 as libc::c_int) << bitdepth_min_8),
                        128 * ((1 as libc::c_int) << bitdepth_min_8) - 1,
                    );
                    f1 = imin(f + 4, ((128 as libc::c_int) << bitdepth_min_8) - 1) >> 3;
                    f2 = imin(f + 3, ((128 as libc::c_int) << bitdepth_min_8) - 1) >> 3;
                    *dst.offset((strideb * -(1 as libc::c_int) as isize) as isize) =
                        iclip_u8(p0 + f2) as pixel;
                    *dst.offset((strideb * 0) as isize) = iclip_u8(q0 - f1) as pixel;
                } else {
                    let mut f_0 = iclip(
                        3 * (q0 - p0),
                        -(128 as libc::c_int) * ((1 as libc::c_int) << bitdepth_min_8),
                        128 * ((1 as libc::c_int) << bitdepth_min_8) - 1,
                    );
                    let mut f1_0 = 0;
                    let mut f2_0 = 0;
                    f1_0 = imin(f_0 + 4, ((128 as libc::c_int) << bitdepth_min_8) - 1) >> 3;
                    f2_0 = imin(f_0 + 3, ((128 as libc::c_int) << bitdepth_min_8) - 1) >> 3;
                    *dst.offset((strideb * -(1 as libc::c_int) as isize) as isize) =
                        iclip_u8(p0 + f2_0) as pixel;
                    *dst.offset((strideb * 0) as isize) = iclip_u8(q0 - f1_0) as pixel;
                    f_0 = f1_0 + 1 >> 1;
                    *dst.offset((strideb * -(2 as libc::c_int) as isize) as isize) =
                        iclip_u8(p1 + f_0) as pixel;
                    *dst.offset((strideb * 1) as isize) = iclip_u8(q1 - f_0) as pixel;
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
    let vm: libc::c_uint = *vmask.offset(0) | *vmask.offset(1) | *vmask.offset(2);
    let mut y: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    while vm & !y.wrapping_sub(1 as libc::c_int as libc::c_uint) != 0 {
        if vm & y != 0 {
            let L = if (*l.offset(0))[0] as libc::c_int != 0 {
                (*l.offset(0))[0] as libc::c_int
            } else {
                (*l.offset(-(1 as libc::c_int) as isize))[0] as libc::c_int
            };
            if !(L == 0) {
                let H = L >> 4;
                let E = (*lut).e[L as usize] as libc::c_int;
                let I = (*lut).i[L as usize] as libc::c_int;
                let idx = if *vmask.offset(2) & y != 0 {
                    2 as libc::c_int
                } else {
                    (*vmask.offset(1) & y != 0) as libc::c_int
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
        y <<= 1;
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
    let vm: libc::c_uint = *vmask.offset(0) | *vmask.offset(1) | *vmask.offset(2);
    let mut x: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    while vm & !x.wrapping_sub(1 as libc::c_int as libc::c_uint) != 0 {
        if vm & x != 0 {
            let L = if (*l.offset(0))[0] as libc::c_int != 0 {
                (*l.offset(0))[0] as libc::c_int
            } else {
                (*l.offset(-b4_stride as isize))[0] as libc::c_int
            };
            if !(L == 0) {
                let H = L >> 4;
                let E = (*lut).e[L as usize] as libc::c_int;
                let I = (*lut).i[L as usize] as libc::c_int;
                let idx = if *vmask.offset(2) & x != 0 {
                    2 as libc::c_int
                } else {
                    (*vmask.offset(1) & x != 0) as libc::c_int
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
        x <<= 1;
        dst = dst.offset(4);
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
    let vm: libc::c_uint = *vmask.offset(0) | *vmask.offset(1);
    let mut y: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    while vm & !y.wrapping_sub(1 as libc::c_int as libc::c_uint) != 0 {
        if vm & y != 0 {
            let L = if (*l.offset(0))[0] as libc::c_int != 0 {
                (*l.offset(0))[0] as libc::c_int
            } else {
                (*l.offset(-(1 as libc::c_int) as isize))[0] as libc::c_int
            };
            if !(L == 0) {
                let H = L >> 4;
                let E = (*lut).e[L as usize] as libc::c_int;
                let I = (*lut).i[L as usize] as libc::c_int;
                let idx = (*vmask.offset(1) & y != 0) as libc::c_int;
                loop_filter(
                    dst,
                    E,
                    I,
                    H,
                    stride,
                    1 as libc::c_int as ptrdiff_t,
                    4 + 2 * idx,
                );
            }
        }
        y <<= 1;
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
    let vm: libc::c_uint = *vmask.offset(0) | *vmask.offset(1);
    let mut x: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    while vm & !x.wrapping_sub(1 as libc::c_int as libc::c_uint) != 0 {
        if vm & x != 0 {
            let L = if (*l.offset(0))[0] as libc::c_int != 0 {
                (*l.offset(0))[0] as libc::c_int
            } else {
                (*l.offset(-b4_stride as isize))[0] as libc::c_int
            };
            if !(L == 0) {
                let H = L >> 4;
                let E = (*lut).e[L as usize] as libc::c_int;
                let I = (*lut).i[L as usize] as libc::c_int;
                let idx = (*vmask.offset(1) & x != 0) as libc::c_int;
                loop_filter(
                    dst,
                    E,
                    I,
                    H,
                    1 as libc::c_int as ptrdiff_t,
                    stride,
                    4 + 2 * idx,
                );
            }
        }
        x <<= 1;
        dst = dst.offset(4);
        l = l.offset(1);
    }
}

#[cfg(feature = "asm")]
use crate::src::cpu::dav1d_get_cpu_flags;

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
#[inline(always)]
unsafe extern "C" fn loop_filter_dsp_init_x86(c: *mut Dav1dLoopFilterDSPContext) {
    use crate::src::x86::cpu::*;

    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_X86_CPU_FLAG_SSSE3 == 0 {
        return;
    }

    (*c).loop_filter_sb[0][0] = Some(dav1d_lpf_h_sb_y_8bpc_ssse3);
    (*c).loop_filter_sb[0][1] = Some(dav1d_lpf_v_sb_y_8bpc_ssse3);
    (*c).loop_filter_sb[1][0] = Some(dav1d_lpf_h_sb_uv_8bpc_ssse3);
    (*c).loop_filter_sb[1][1] = Some(dav1d_lpf_v_sb_uv_8bpc_ssse3);

    #[cfg(target_arch = "x86_64")]
    {
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
pub unsafe extern "C" fn dav1d_loop_filter_dsp_init_8bpc(c: *mut Dav1dLoopFilterDSPContext) {
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
