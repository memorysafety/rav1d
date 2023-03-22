use ::libc;
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
use crate::include::stddef::ptrdiff_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type pixel = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Av1FilterLUT {
    pub e: [uint8_t; 64],
    pub i: [uint8_t; 64],
    pub sharp: [uint64_t; 2],
}
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
#[inline]
unsafe extern "C" fn imin(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn iclip(
    v: libc::c_int,
    min: libc::c_int,
    max: libc::c_int,
) -> libc::c_int {
    return if v < min { min } else if v > max { max } else { v };
}
#[inline]
unsafe extern "C" fn iclip_u8(v: libc::c_int) -> libc::c_int {
    return iclip(v, 0 as libc::c_int, 255 as libc::c_int);
}
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
            .offset((strideb * -(2 as libc::c_int) as libc::c_long) as isize)
            as libc::c_int;
        let mut p0: libc::c_int = *dst
            .offset((strideb * -(1 as libc::c_int) as libc::c_long) as isize)
            as libc::c_int;
        let mut q0: libc::c_int = *dst
            .offset((strideb * 0 as libc::c_int as libc::c_long) as isize)
            as libc::c_int;
        let mut q1: libc::c_int = *dst
            .offset((strideb * 1 as libc::c_int as libc::c_long) as isize)
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
            p2 = *dst.offset((strideb * -(3 as libc::c_int) as libc::c_long) as isize)
                as libc::c_int;
            q2 = *dst.offset((strideb * 2 as libc::c_int as libc::c_long) as isize)
                as libc::c_int;
            fm &= (abs(p2 - p1) <= I && abs(q2 - q1) <= I) as libc::c_int;
            if wd > 6 as libc::c_int {
                p3 = *dst
                    .offset((strideb * -(4 as libc::c_int) as libc::c_long) as isize)
                    as libc::c_int;
                q3 = *dst.offset((strideb * 3 as libc::c_int as libc::c_long) as isize)
                    as libc::c_int;
                fm &= (abs(p3 - p2) <= I && abs(q3 - q2) <= I) as libc::c_int;
            }
        }
        if !(fm == 0) {
            if wd >= 16 as libc::c_int {
                p6 = *dst
                    .offset((strideb * -(7 as libc::c_int) as libc::c_long) as isize)
                    as libc::c_int;
                p5 = *dst
                    .offset((strideb * -(6 as libc::c_int) as libc::c_long) as isize)
                    as libc::c_int;
                p4 = *dst
                    .offset((strideb * -(5 as libc::c_int) as libc::c_long) as isize)
                    as libc::c_int;
                q4 = *dst.offset((strideb * 4 as libc::c_int as libc::c_long) as isize)
                    as libc::c_int;
                q5 = *dst.offset((strideb * 5 as libc::c_int as libc::c_long) as isize)
                    as libc::c_int;
                q6 = *dst.offset((strideb * 6 as libc::c_int as libc::c_long) as isize)
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
                        (strideb * -(6 as libc::c_int) as libc::c_long) as isize,
                    ) = (p6 + p6 + p6 + p6 + p6 + p6 * 2 as libc::c_int
                    + p5 * 2 as libc::c_int + p4 * 2 as libc::c_int + p3 + p2 + p1 + p0
                    + q0 + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * -(5 as libc::c_int) as libc::c_long) as isize,
                    ) = (p6 + p6 + p6 + p6 + p6 + p5 * 2 as libc::c_int
                    + p4 * 2 as libc::c_int + p3 * 2 as libc::c_int + p2 + p1 + p0 + q0
                    + q1 + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * -(4 as libc::c_int) as libc::c_long) as isize,
                    ) = (p6 + p6 + p6 + p6 + p5 + p4 * 2 as libc::c_int
                    + p3 * 2 as libc::c_int + p2 * 2 as libc::c_int + p1 + p0 + q0 + q1
                    + q2 + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * -(3 as libc::c_int) as libc::c_long) as isize,
                    ) = (p6 + p6 + p6 + p5 + p4 + p3 * 2 as libc::c_int
                    + p2 * 2 as libc::c_int + p1 * 2 as libc::c_int + p0 + q0 + q1 + q2
                    + q3 + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * -(2 as libc::c_int) as libc::c_long) as isize,
                    ) = (p6 + p6 + p5 + p4 + p3 + p2 * 2 as libc::c_int
                    + p1 * 2 as libc::c_int + p0 * 2 as libc::c_int + q0 + q1 + q2 + q3
                    + q4 + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * -(1 as libc::c_int) as libc::c_long) as isize,
                    ) = (p6 + p5 + p4 + p3 + p2 + p1 * 2 as libc::c_int
                    + p0 * 2 as libc::c_int + q0 * 2 as libc::c_int + q1 + q2 + q3 + q4
                    + q5 + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * 0 as libc::c_int as libc::c_long) as isize,
                    ) = (p5 + p4 + p3 + p2 + p1 + p0 * 2 as libc::c_int
                    + q0 * 2 as libc::c_int + q1 * 2 as libc::c_int + q2 + q3 + q4 + q5
                    + q6 + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * 1 as libc::c_int as libc::c_long) as isize,
                    ) = (p4 + p3 + p2 + p1 + p0 + q0 * 2 as libc::c_int
                    + q1 * 2 as libc::c_int + q2 * 2 as libc::c_int + q3 + q4 + q5 + q6
                    + q6 + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * 2 as libc::c_int as libc::c_long) as isize,
                    ) = (p3 + p2 + p1 + p0 + q0 + q1 * 2 as libc::c_int
                    + q2 * 2 as libc::c_int + q3 * 2 as libc::c_int + q4 + q5 + q6 + q6
                    + q6 + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * 3 as libc::c_int as libc::c_long) as isize,
                    ) = (p2 + p1 + p0 + q0 + q1 + q2 * 2 as libc::c_int
                    + q3 * 2 as libc::c_int + q4 * 2 as libc::c_int + q5 + q6 + q6 + q6
                    + q6 + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * 4 as libc::c_int as libc::c_long) as isize,
                    ) = (p1 + p0 + q0 + q1 + q2 + q3 * 2 as libc::c_int
                    + q4 * 2 as libc::c_int + q5 * 2 as libc::c_int + q6 + q6 + q6 + q6
                    + q6 + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * 5 as libc::c_int as libc::c_long) as isize,
                    ) = (p0 + q0 + q1 + q2 + q3 + q4 * 2 as libc::c_int
                    + q5 * 2 as libc::c_int + q6 * 2 as libc::c_int + q6 + q6 + q6 + q6
                    + q6 + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
            } else if wd >= 8 as libc::c_int && flat8in != 0 {
                *dst
                    .offset(
                        (strideb * -(3 as libc::c_int) as libc::c_long) as isize,
                    ) = (p3 + p3 + p3 + 2 as libc::c_int * p2 + p1 + p0 + q0
                    + 4 as libc::c_int >> 3 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * -(2 as libc::c_int) as libc::c_long) as isize,
                    ) = (p3 + p3 + p2 + 2 as libc::c_int * p1 + p0 + q0 + q1
                    + 4 as libc::c_int >> 3 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * -(1 as libc::c_int) as libc::c_long) as isize,
                    ) = (p3 + p2 + p1 + 2 as libc::c_int * p0 + q0 + q1 + q2
                    + 4 as libc::c_int >> 3 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * 0 as libc::c_int as libc::c_long) as isize,
                    ) = (p2 + p1 + p0 + 2 as libc::c_int * q0 + q1 + q2 + q3
                    + 4 as libc::c_int >> 3 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * 1 as libc::c_int as libc::c_long) as isize,
                    ) = (p1 + p0 + q0 + 2 as libc::c_int * q1 + q2 + q3 + q3
                    + 4 as libc::c_int >> 3 as libc::c_int) as pixel;
                *dst
                    .offset(
                        (strideb * 2 as libc::c_int as libc::c_long) as isize,
                    ) = (p0 + q0 + q1 + 2 as libc::c_int * q2 + q3 + q3 + q3
                    + 4 as libc::c_int >> 3 as libc::c_int) as pixel;
            } else if wd == 6 as libc::c_int && flat8in != 0 {
                *dst
                    .offset(
                        (strideb * -(2 as libc::c_int) as libc::c_long) as isize,
                    ) = (p2 + 2 as libc::c_int * p2 + 2 as libc::c_int * p1
                    + 2 as libc::c_int * p0 + q0 + 4 as libc::c_int >> 3 as libc::c_int)
                    as pixel;
                *dst
                    .offset(
                        (strideb * -(1 as libc::c_int) as libc::c_long) as isize,
                    ) = (p2 + 2 as libc::c_int * p1 + 2 as libc::c_int * p0
                    + 2 as libc::c_int * q0 + q1 + 4 as libc::c_int >> 3 as libc::c_int)
                    as pixel;
                *dst
                    .offset(
                        (strideb * 0 as libc::c_int as libc::c_long) as isize,
                    ) = (p1 + 2 as libc::c_int * p0 + 2 as libc::c_int * q0
                    + 2 as libc::c_int * q1 + q2 + 4 as libc::c_int >> 3 as libc::c_int)
                    as pixel;
                *dst
                    .offset(
                        (strideb * 1 as libc::c_int as libc::c_long) as isize,
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
                            (strideb * -(1 as libc::c_int) as libc::c_long) as isize,
                        ) = iclip_u8(p0 + f2) as pixel;
                    *dst
                        .offset(
                            (strideb * 0 as libc::c_int as libc::c_long) as isize,
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
                            (strideb * -(1 as libc::c_int) as libc::c_long) as isize,
                        ) = iclip_u8(p0 + f2_0) as pixel;
                    *dst
                        .offset(
                            (strideb * 0 as libc::c_int as libc::c_long) as isize,
                        ) = iclip_u8(q0 - f1_0) as pixel;
                    f_0 = f1_0 + 1 as libc::c_int >> 1 as libc::c_int;
                    *dst
                        .offset(
                            (strideb * -(2 as libc::c_int) as libc::c_long) as isize,
                        ) = iclip_u8(p1 + f_0) as pixel;
                    *dst
                        .offset(
                            (strideb * 1 as libc::c_int as libc::c_long) as isize,
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
    h: libc::c_int,
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
        dst = dst.offset((4 as libc::c_int as libc::c_long * stride) as isize);
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
    w: libc::c_int,
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
    h: libc::c_int,
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
        dst = dst.offset((4 as libc::c_int as libc::c_long * stride) as isize);
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
    w: libc::c_int,
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
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_loop_filter_dsp_init_8bpc(
    c: *mut Dav1dLoopFilterDSPContext,
) {
    (*c)
        .loop_filter_sb[0 as libc::c_int
        as usize][0 as libc::c_int
        as usize] = Some(
        loop_filter_h_sb128y_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const uint32_t,
                *const [uint8_t; 4],
                ptrdiff_t,
                *const Av1FilterLUT,
                libc::c_int,
            ) -> (),
    );
    (*c)
        .loop_filter_sb[0 as libc::c_int
        as usize][1 as libc::c_int
        as usize] = Some(
        loop_filter_v_sb128y_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const uint32_t,
                *const [uint8_t; 4],
                ptrdiff_t,
                *const Av1FilterLUT,
                libc::c_int,
            ) -> (),
    );
    (*c)
        .loop_filter_sb[1 as libc::c_int
        as usize][0 as libc::c_int
        as usize] = Some(
        loop_filter_h_sb128uv_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const uint32_t,
                *const [uint8_t; 4],
                ptrdiff_t,
                *const Av1FilterLUT,
                libc::c_int,
            ) -> (),
    );
    (*c)
        .loop_filter_sb[1 as libc::c_int
        as usize][1 as libc::c_int
        as usize] = Some(
        loop_filter_v_sb128uv_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const uint32_t,
                *const [uint8_t; 4],
                ptrdiff_t,
                *const Av1FilterLUT,
                libc::c_int,
            ) -> (),
    );
}
