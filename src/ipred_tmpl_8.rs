use ::libc;
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    static dav1d_sm_weights: [uint8_t; 128];
    static dav1d_dr_intra_derivative: [uint16_t; 44];
    static dav1d_filter_intra_taps: [[int8_t; 64]; 5];
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type ptrdiff_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type pixel = uint8_t;
pub type Dav1dPixelLayout = libc::c_uint;
pub const DAV1D_PIXEL_LAYOUT_I444: Dav1dPixelLayout = 3;
pub const DAV1D_PIXEL_LAYOUT_I422: Dav1dPixelLayout = 2;
pub const DAV1D_PIXEL_LAYOUT_I420: Dav1dPixelLayout = 1;
pub const DAV1D_PIXEL_LAYOUT_I400: Dav1dPixelLayout = 0;
pub type IntraPredMode = libc::c_uint;
pub const FILTER_PRED: IntraPredMode = 13;
pub const Z3_PRED: IntraPredMode = 8;
pub const Z2_PRED: IntraPredMode = 7;
pub const Z1_PRED: IntraPredMode = 6;
pub const DC_128_PRED: IntraPredMode = 5;
pub const TOP_DC_PRED: IntraPredMode = 4;
pub const LEFT_DC_PRED: IntraPredMode = 3;
pub const N_IMPL_INTRA_PRED_MODES: IntraPredMode = 14;
pub const N_UV_INTRA_PRED_MODES: IntraPredMode = 14;
pub const CFL_PRED: IntraPredMode = 13;
pub const N_INTRA_PRED_MODES: IntraPredMode = 13;
pub const PAETH_PRED: IntraPredMode = 12;
pub const SMOOTH_H_PRED: IntraPredMode = 11;
pub const SMOOTH_V_PRED: IntraPredMode = 10;
pub const SMOOTH_PRED: IntraPredMode = 9;
pub const VERT_LEFT_PRED: IntraPredMode = 8;
pub const HOR_UP_PRED: IntraPredMode = 7;
pub const HOR_DOWN_PRED: IntraPredMode = 6;
pub const VERT_RIGHT_PRED: IntraPredMode = 5;
pub const DIAG_DOWN_RIGHT_PRED: IntraPredMode = 4;
pub const DIAG_DOWN_LEFT_PRED: IntraPredMode = 3;
pub const HOR_PRED: IntraPredMode = 2;
pub const VERT_PRED: IntraPredMode = 1;
pub const DC_PRED: IntraPredMode = 0;
pub type angular_ipred_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type cfl_ac_fn = Option<
    unsafe extern "C" fn(
        *mut int16_t,
        *const pixel,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type cfl_pred_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        libc::c_int,
        libc::c_int,
        *const int16_t,
        libc::c_int,
    ) -> (),
>;
pub type pal_pred_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const uint16_t,
        *const uint8_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav1dIntraPredDSPContext {
    pub intra_pred: [angular_ipred_fn; 14],
    pub cfl_ac: [cfl_ac_fn; 3],
    pub cfl_pred: [cfl_pred_fn; 6],
    pub pal_pred: pal_pred_fn,
}
#[inline]
unsafe extern "C" fn ctz(mask: libc::c_uint) -> libc::c_int {
    return mask.trailing_zeros() as i32;
}
#[inline]
unsafe extern "C" fn imax(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn imin(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn iclip(v: libc::c_int, min: libc::c_int, max: libc::c_int) -> libc::c_int {
    return if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    };
}
#[inline]
unsafe extern "C" fn iclip_u8(v: libc::c_int) -> libc::c_int {
    return iclip(v, 0i32, 255i32);
}
#[inline]
unsafe extern "C" fn apply_sign(v: libc::c_int, s: libc::c_int) -> libc::c_int {
    return if s < 0i32 { -v } else { v };
}
#[inline(never)]
unsafe extern "C" fn splat_dc(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    width: libc::c_int,
    height: libc::c_int,
    dc: libc::c_int,
) {
    if !(dc <= 0xffi32) {
        unreachable!();
    }
    if width > 4i32 {
        let dcN: uint64_t = (dc as libc::c_ulonglong).wrapping_mul(0x101010101010101u64);
        let mut y: libc::c_int = 0i32;
        while y < height {
            let mut x: libc::c_int = 0i32;
            while x < width {
                *(&mut *dst.offset(x as isize) as *mut pixel as *mut uint64_t) = dcN;
                x = (x as libc::c_ulong)
                    .wrapping_add(::core::mem::size_of::<uint64_t>() as libc::c_ulong)
                    as libc::c_int;
            }
            dst = dst.offset(stride as isize);
            y += 1;
        }
    } else {
        let dcN_0: libc::c_uint = (dc as libc::c_uint).wrapping_mul(0x1010101u32);
        let mut y_0: libc::c_int = 0i32;
        while y_0 < height {
            let mut x_0: libc::c_int = 0i32;
            while x_0 < width {
                *(&mut *dst.offset(x_0 as isize) as *mut pixel as *mut libc::c_uint) = dcN_0;
                x_0 = (x_0 as libc::c_ulong)
                    .wrapping_add(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    as libc::c_int;
            }
            dst = dst.offset(stride as isize);
            y_0 += 1;
        }
    };
}
#[inline(never)]
unsafe extern "C" fn cfl_pred(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    width: libc::c_int,
    height: libc::c_int,
    dc: libc::c_int,
    mut ac: *const int16_t,
    alpha: libc::c_int,
) {
    let mut y: libc::c_int = 0i32;
    while y < height {
        let mut x: libc::c_int = 0i32;
        while x < width {
            let diff: libc::c_int = alpha * *ac.offset(x as isize) as libc::c_int;
            *dst.offset(x as isize) =
                iclip_u8(dc + apply_sign(abs(diff) + 32i32 >> 6i32, diff)) as pixel;
            x += 1;
        }
        ac = ac.offset(width as isize);
        dst = dst.offset(stride as isize);
        y += 1;
    }
}
unsafe extern "C" fn dc_gen_top(topleft: *const pixel, width: libc::c_int) -> libc::c_uint {
    let mut dc: libc::c_uint = (width >> 1i32) as libc::c_uint;
    let mut i: libc::c_int = 0i32;
    while i < width {
        dc = dc.wrapping_add(*topleft.offset((1i32 + i) as isize) as libc::c_uint);
        i += 1;
    }
    return dc >> ctz(width as libc::c_uint);
}
unsafe extern "C" fn ipred_dc_top_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    a: libc::c_int,
    max_width: libc::c_int,
    max_height: libc::c_int,
) {
    splat_dc(
        dst,
        stride,
        width,
        height,
        dc_gen_top(topleft, width) as libc::c_int,
    );
}
unsafe extern "C" fn ipred_cfl_top_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    mut ac: *const int16_t,
    alpha: libc::c_int,
) {
    cfl_pred(
        dst,
        stride,
        width,
        height,
        dc_gen_top(topleft, width) as libc::c_int,
        ac,
        alpha,
    );
}
unsafe extern "C" fn dc_gen_left(topleft: *const pixel, height: libc::c_int) -> libc::c_uint {
    let mut dc: libc::c_uint = (height >> 1i32) as libc::c_uint;
    let mut i: libc::c_int = 0i32;
    while i < height {
        dc = dc.wrapping_add(*topleft.offset(-(1i32 + i) as isize) as libc::c_uint);
        i += 1;
    }
    return dc >> ctz(height as libc::c_uint);
}
unsafe extern "C" fn ipred_dc_left_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    a: libc::c_int,
    max_width: libc::c_int,
    max_height: libc::c_int,
) {
    splat_dc(
        dst,
        stride,
        width,
        height,
        dc_gen_left(topleft, height) as libc::c_int,
    );
}
unsafe extern "C" fn ipred_cfl_left_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    mut ac: *const int16_t,
    alpha: libc::c_int,
) {
    let dc: libc::c_uint = dc_gen_left(topleft, height);
    cfl_pred(dst, stride, width, height, dc as libc::c_int, ac, alpha);
}
unsafe extern "C" fn dc_gen(
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
) -> libc::c_uint {
    let mut dc: libc::c_uint = (width + height >> 1i32) as libc::c_uint;
    let mut i: libc::c_int = 0i32;
    while i < width {
        dc = dc.wrapping_add(*topleft.offset((i + 1i32) as isize) as libc::c_uint);
        i += 1;
    }
    let mut i_0: libc::c_int = 0i32;
    while i_0 < height {
        dc = dc.wrapping_add(*topleft.offset(-(i_0 + 1i32) as isize) as libc::c_uint);
        i_0 += 1;
    }
    dc >>= ctz((width + height) as libc::c_uint);
    if width != height {
        dc = dc.wrapping_mul(
            (if width > height * 2i32 || height > width * 2i32 {
                0x3334i32
            } else {
                0x5556i32
            }) as libc::c_uint,
        );
        dc >>= 16i32;
    }
    return dc;
}
unsafe extern "C" fn ipred_dc_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    a: libc::c_int,
    max_width: libc::c_int,
    max_height: libc::c_int,
) {
    splat_dc(
        dst,
        stride,
        width,
        height,
        dc_gen(topleft, width, height) as libc::c_int,
    );
}
unsafe extern "C" fn ipred_cfl_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    mut ac: *const int16_t,
    alpha: libc::c_int,
) {
    let mut dc: libc::c_uint = dc_gen(topleft, width, height);
    cfl_pred(dst, stride, width, height, dc as libc::c_int, ac, alpha);
}
unsafe extern "C" fn ipred_dc_128_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    a: libc::c_int,
    max_width: libc::c_int,
    max_height: libc::c_int,
) {
    let dc: libc::c_int = 128i32;
    splat_dc(dst, stride, width, height, dc);
}
unsafe extern "C" fn ipred_cfl_128_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    mut ac: *const int16_t,
    alpha: libc::c_int,
) {
    let dc: libc::c_int = 128i32;
    cfl_pred(dst, stride, width, height, dc, ac, alpha);
}
unsafe extern "C" fn ipred_v_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    a: libc::c_int,
    max_width: libc::c_int,
    max_height: libc::c_int,
) {
    let mut y: libc::c_int = 0i32;
    while y < height {
        memcpy(
            dst as *mut libc::c_void,
            topleft.offset(1isize) as *const libc::c_void,
            width as libc::c_ulong,
        );
        dst = dst.offset(stride as isize);
        y += 1;
    }
}
unsafe extern "C" fn ipred_h_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    a: libc::c_int,
    max_width: libc::c_int,
    max_height: libc::c_int,
) {
    let mut y: libc::c_int = 0i32;
    while y < height {
        memset(
            dst as *mut libc::c_void,
            *topleft.offset(-(1i32 + y) as isize) as libc::c_int,
            width as libc::c_ulong,
        );
        dst = dst.offset(stride as isize);
        y += 1;
    }
}
unsafe extern "C" fn ipred_paeth_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    tl_ptr: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    a: libc::c_int,
    max_width: libc::c_int,
    max_height: libc::c_int,
) {
    let topleft: libc::c_int = *tl_ptr.offset(0isize) as libc::c_int;
    let mut y: libc::c_int = 0i32;
    while y < height {
        let left: libc::c_int = *tl_ptr.offset(-(y + 1i32) as isize) as libc::c_int;
        let mut x: libc::c_int = 0i32;
        while x < width {
            let top: libc::c_int = *tl_ptr.offset((1i32 + x) as isize) as libc::c_int;
            let base: libc::c_int = left + top - topleft;
            let ldiff: libc::c_int = abs(left - base);
            let tdiff: libc::c_int = abs(top - base);
            let tldiff: libc::c_int = abs(topleft - base);
            *dst.offset(x as isize) = (if ldiff <= tdiff && ldiff <= tldiff {
                left
            } else if tdiff <= tldiff {
                top
            } else {
                topleft
            }) as pixel;
            x += 1;
        }
        dst = dst.offset(stride as isize);
        y += 1;
    }
}
unsafe extern "C" fn ipred_smooth_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    a: libc::c_int,
    max_width: libc::c_int,
    max_height: libc::c_int,
) {
    let weights_hor: *const uint8_t =
        &*dav1d_sm_weights.as_ptr().offset(width as isize) as *const uint8_t;
    let weights_ver: *const uint8_t =
        &*dav1d_sm_weights.as_ptr().offset(height as isize) as *const uint8_t;
    let right: libc::c_int = *topleft.offset(width as isize) as libc::c_int;
    let bottom: libc::c_int = *topleft.offset(-height as isize) as libc::c_int;
    let mut y: libc::c_int = 0i32;
    while y < height {
        let mut x: libc::c_int = 0i32;
        while x < width {
            let pred: libc::c_int = *weights_ver.offset(y as isize) as libc::c_int
                * *topleft.offset((1i32 + x) as isize) as libc::c_int
                + (256i32 - *weights_ver.offset(y as isize) as libc::c_int) * bottom
                + *weights_hor.offset(x as isize) as libc::c_int
                    * *topleft.offset(-(1i32 + y) as isize) as libc::c_int
                + (256i32 - *weights_hor.offset(x as isize) as libc::c_int) * right;
            *dst.offset(x as isize) = (pred + 256i32 >> 9i32) as pixel;
            x += 1;
        }
        dst = dst.offset(stride as isize);
        y += 1;
    }
}
unsafe extern "C" fn ipred_smooth_v_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    a: libc::c_int,
    max_width: libc::c_int,
    max_height: libc::c_int,
) {
    let weights_ver: *const uint8_t =
        &*dav1d_sm_weights.as_ptr().offset(height as isize) as *const uint8_t;
    let bottom: libc::c_int = *topleft.offset(-height as isize) as libc::c_int;
    let mut y: libc::c_int = 0i32;
    while y < height {
        let mut x: libc::c_int = 0i32;
        while x < width {
            let pred: libc::c_int = *weights_ver.offset(y as isize) as libc::c_int
                * *topleft.offset((1i32 + x) as isize) as libc::c_int
                + (256i32 - *weights_ver.offset(y as isize) as libc::c_int) * bottom;
            *dst.offset(x as isize) = (pred + 128i32 >> 8i32) as pixel;
            x += 1;
        }
        dst = dst.offset(stride as isize);
        y += 1;
    }
}
unsafe extern "C" fn ipred_smooth_h_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    a: libc::c_int,
    max_width: libc::c_int,
    max_height: libc::c_int,
) {
    let weights_hor: *const uint8_t =
        &*dav1d_sm_weights.as_ptr().offset(width as isize) as *const uint8_t;
    let right: libc::c_int = *topleft.offset(width as isize) as libc::c_int;
    let mut y: libc::c_int = 0i32;
    while y < height {
        let mut x: libc::c_int = 0i32;
        while x < width {
            let pred: libc::c_int = *weights_hor.offset(x as isize) as libc::c_int
                * *topleft.offset(-(y + 1i32) as isize) as libc::c_int
                + (256i32 - *weights_hor.offset(x as isize) as libc::c_int) * right;
            *dst.offset(x as isize) = (pred + 128i32 >> 8i32) as pixel;
            x += 1;
        }
        dst = dst.offset(stride as isize);
        y += 1;
    }
}
#[inline(never)]
unsafe extern "C" fn get_filter_strength(
    wh: libc::c_int,
    angle: libc::c_int,
    is_sm: libc::c_int,
) -> libc::c_int {
    if is_sm != 0 {
        if wh <= 8i32 {
            if angle >= 64i32 {
                return 2i32;
            }
            if angle >= 40i32 {
                return 1i32;
            }
        } else if wh <= 16i32 {
            if angle >= 48i32 {
                return 2i32;
            }
            if angle >= 20i32 {
                return 1i32;
            }
        } else if wh <= 24i32 {
            if angle >= 4i32 {
                return 3i32;
            }
        } else {
            return 3i32;
        }
    } else if wh <= 8i32 {
        if angle >= 56i32 {
            return 1i32;
        }
    } else if wh <= 16i32 {
        if angle >= 40i32 {
            return 1i32;
        }
    } else if wh <= 24i32 {
        if angle >= 32i32 {
            return 3i32;
        }
        if angle >= 16i32 {
            return 2i32;
        }
        if angle >= 8i32 {
            return 1i32;
        }
    } else if wh <= 32i32 {
        if angle >= 32i32 {
            return 3i32;
        }
        if angle >= 4i32 {
            return 2i32;
        }
        return 1i32;
    } else {
        return 3i32;
    }
    return 0i32;
}
#[inline(never)]
unsafe extern "C" fn filter_edge(
    out: *mut pixel,
    sz: libc::c_int,
    lim_from: libc::c_int,
    lim_to: libc::c_int,
    in_0: *const pixel,
    from: libc::c_int,
    to: libc::c_int,
    strength: libc::c_int,
) {
    static mut kernel: [[uint8_t; 5]; 3] = [
        [0u8, 4u8, 8u8, 4u8, 0u8],
        [0u8, 5u8, 6u8, 5u8, 0u8],
        [2u8, 4u8, 4u8, 4u8, 2u8],
    ];
    if !(strength > 0i32) {
        unreachable!();
    }
    let mut i: libc::c_int = 0i32;
    while i < imin(sz, lim_from) {
        *out.offset(i as isize) = *in_0.offset(iclip(i, from, to - 1i32) as isize);
        i += 1;
    }
    while i < imin(lim_to, sz) {
        let mut s: libc::c_int = 0i32;
        let mut j: libc::c_int = 0i32;
        while j < 5i32 {
            s += *in_0.offset(iclip(i - 2i32 + j, from, to - 1i32) as isize) as libc::c_int
                * kernel[(strength - 1i32) as usize][j as usize] as libc::c_int;
            j += 1;
        }
        *out.offset(i as isize) = (s + 8i32 >> 4i32) as pixel;
        i += 1;
    }
    while i < sz {
        *out.offset(i as isize) = *in_0.offset(iclip(i, from, to - 1i32) as isize);
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn get_upsample(
    wh: libc::c_int,
    angle: libc::c_int,
    is_sm: libc::c_int,
) -> libc::c_int {
    return (angle < 40i32 && wh <= 16i32 >> is_sm) as libc::c_int;
}
#[inline(never)]
unsafe extern "C" fn upsample_edge(
    out: *mut pixel,
    hsz: libc::c_int,
    in_0: *const pixel,
    from: libc::c_int,
    to: libc::c_int,
) {
    static mut kernel: [int8_t; 4] = [-1i8, 9i8, 9i8, -1i8];
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < hsz - 1i32 {
        *out.offset((i * 2i32) as isize) = *in_0.offset(iclip(i, from, to - 1i32) as isize);
        let mut s: libc::c_int = 0i32;
        let mut j: libc::c_int = 0i32;
        while j < 4i32 {
            s += *in_0.offset(iclip(i + j - 1i32, from, to - 1i32) as isize) as libc::c_int
                * kernel[j as usize] as libc::c_int;
            j += 1;
        }
        *out.offset((i * 2i32 + 1i32) as isize) = iclip_u8(s + 8i32 >> 4i32) as pixel;
        i += 1;
    }
    *out.offset((i * 2i32) as isize) = *in_0.offset(iclip(i, from, to - 1i32) as isize);
}
unsafe extern "C" fn ipred_z1_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft_in: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    mut angle: libc::c_int,
    max_width: libc::c_int,
    max_height: libc::c_int,
) {
    let is_sm: libc::c_int = angle >> 9i32 & 0x1i32;
    let enable_intra_edge_filter: libc::c_int = angle >> 10i32;
    angle &= 511i32;
    if !(angle < 90i32) {
        unreachable!();
    }
    let mut dx: libc::c_int = dav1d_dr_intra_derivative[(angle >> 1i32) as usize] as libc::c_int;
    let mut top_out: [pixel; 128] = [0; 128];
    let mut top: *const pixel = 0 as *const pixel;
    let mut max_base_x: libc::c_int = 0;
    let upsample_above: libc::c_int = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, 90i32 - angle, is_sm)
    } else {
        0i32
    };
    if upsample_above != 0 {
        upsample_edge(
            top_out.as_mut_ptr(),
            width + height,
            &*topleft_in.offset(1isize),
            -(1i32),
            width + imin(width, height),
        );
        top = top_out.as_mut_ptr();
        max_base_x = 2i32 * (width + height) - 2i32;
        dx <<= 1i32;
    } else {
        let filter_strength: libc::c_int = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, 90i32 - angle, is_sm)
        } else {
            0i32
        };
        if filter_strength != 0 {
            filter_edge(
                top_out.as_mut_ptr(),
                width + height,
                0i32,
                width + height,
                &*topleft_in.offset(1isize),
                -(1i32),
                width + imin(width, height),
                filter_strength,
            );
            top = top_out.as_mut_ptr();
            max_base_x = width + height - 1i32;
        } else {
            top = &*topleft_in.offset(1isize) as *const pixel;
            max_base_x = width + imin(width, height) - 1i32;
        }
    }
    let base_inc: libc::c_int = 1i32 + upsample_above;
    let mut y: libc::c_int = 0i32;
    let mut xpos: libc::c_int = dx;
    while y < height {
        let frac: libc::c_int = xpos & 0x3ei32;
        let mut x: libc::c_int = 0i32;
        let mut base: libc::c_int = xpos >> 6i32;
        while x < width {
            if base < max_base_x {
                let v: libc::c_int = *top.offset(base as isize) as libc::c_int * (64i32 - frac)
                    + *top.offset((base + 1i32) as isize) as libc::c_int * frac;
                *dst.offset(x as isize) = (v + 32i32 >> 6i32) as pixel;
                x += 1;
                base += base_inc;
            } else {
                memset(
                    &mut *dst.offset(x as isize) as *mut pixel as *mut libc::c_void,
                    *top.offset(max_base_x as isize) as libc::c_int,
                    (width - x) as libc::c_ulong,
                );
                break;
            }
        }
        y += 1;
        dst = dst.offset(stride as isize);
        xpos += dx;
    }
}
unsafe extern "C" fn ipred_z2_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft_in: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    mut angle: libc::c_int,
    max_width: libc::c_int,
    max_height: libc::c_int,
) {
    let is_sm: libc::c_int = angle >> 9i32 & 0x1i32;
    let enable_intra_edge_filter: libc::c_int = angle >> 10i32;
    angle &= 511i32;
    if !(angle > 90i32 && angle < 180i32) {
        unreachable!();
    }
    let mut dy: libc::c_int =
        dav1d_dr_intra_derivative[(angle - 90i32 >> 1i32) as usize] as libc::c_int;
    let mut dx: libc::c_int =
        dav1d_dr_intra_derivative[(180i32 - angle >> 1i32) as usize] as libc::c_int;
    let upsample_left: libc::c_int = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, 180i32 - angle, is_sm)
    } else {
        0i32
    };
    let upsample_above: libc::c_int = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, angle - 90i32, is_sm)
    } else {
        0i32
    };
    let mut edge: [pixel; 129] = [0; 129];
    let topleft: *mut pixel = &mut *edge.as_mut_ptr().offset(64isize) as *mut pixel;
    if upsample_above != 0 {
        upsample_edge(topleft, width + 1i32, topleft_in, 0i32, width + 1i32);
        dx <<= 1i32;
    } else {
        let filter_strength: libc::c_int = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, angle - 90i32, is_sm)
        } else {
            0i32
        };
        if filter_strength != 0 {
            filter_edge(
                &mut *topleft.offset(1isize),
                width,
                0i32,
                max_width,
                &*topleft_in.offset(1isize),
                -(1i32),
                width,
                filter_strength,
            );
        } else {
            memcpy(
                &mut *topleft.offset(1isize) as *mut pixel as *mut libc::c_void,
                &*topleft_in.offset(1isize) as *const pixel as *const libc::c_void,
                width as libc::c_ulong,
            );
        }
    }
    if upsample_left != 0 {
        upsample_edge(
            &mut *topleft.offset((-height * 2i32) as isize),
            height + 1i32,
            &*topleft_in.offset(-height as isize),
            0i32,
            height + 1i32,
        );
        dy <<= 1i32;
    } else {
        let filter_strength_0: libc::c_int = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, 180i32 - angle, is_sm)
        } else {
            0i32
        };
        if filter_strength_0 != 0 {
            filter_edge(
                &mut *topleft.offset(-height as isize),
                height,
                height - max_height,
                height,
                &*topleft_in.offset(-height as isize),
                0i32,
                height + 1i32,
                filter_strength_0,
            );
        } else {
            memcpy(
                &mut *topleft.offset(-height as isize) as *mut pixel as *mut libc::c_void,
                &*topleft_in.offset(-height as isize) as *const pixel as *const libc::c_void,
                height as libc::c_ulong,
            );
        }
    }
    *topleft = *topleft_in;
    let base_inc_x: libc::c_int = 1i32 + upsample_above;
    let left: *const pixel = &mut *topleft.offset(-(1i32 + upsample_left) as isize) as *mut pixel;
    let mut y: libc::c_int = 0i32;
    let mut xpos: libc::c_int = (1i32 + upsample_above << 6i32) - dx;
    while y < height {
        let mut base_x: libc::c_int = xpos >> 6i32;
        let frac_x: libc::c_int = xpos & 0x3ei32;
        let mut x: libc::c_int = 0i32;
        let mut ypos: libc::c_int = (y << 6i32 + upsample_left) - dy;
        while x < width {
            let mut v: libc::c_int = 0;
            if base_x >= 0i32 {
                v = *topleft.offset(base_x as isize) as libc::c_int * (64i32 - frac_x)
                    + *topleft.offset((base_x + 1i32) as isize) as libc::c_int * frac_x;
            } else {
                let base_y: libc::c_int = ypos >> 6i32;
                if !(base_y >= -(1i32 + upsample_left)) {
                    unreachable!();
                }
                let frac_y: libc::c_int = ypos & 0x3ei32;
                v = *left.offset(-base_y as isize) as libc::c_int * (64i32 - frac_y)
                    + *left.offset(-(base_y + 1i32) as isize) as libc::c_int * frac_y;
            }
            *dst.offset(x as isize) = (v + 32i32 >> 6i32) as pixel;
            x += 1;
            base_x += base_inc_x;
            ypos -= dy;
        }
        y += 1;
        xpos -= dx;
        dst = dst.offset(stride as isize);
    }
}
unsafe extern "C" fn ipred_z3_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft_in: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    mut angle: libc::c_int,
    max_width: libc::c_int,
    max_height: libc::c_int,
) {
    let is_sm: libc::c_int = angle >> 9i32 & 0x1i32;
    let enable_intra_edge_filter: libc::c_int = angle >> 10i32;
    angle &= 511i32;
    if !(angle > 180i32) {
        unreachable!();
    }
    let mut dy: libc::c_int =
        dav1d_dr_intra_derivative[(270i32 - angle >> 1i32) as usize] as libc::c_int;
    let mut left_out: [pixel; 128] = [0; 128];
    let mut left: *const pixel = 0 as *const pixel;
    let mut max_base_y: libc::c_int = 0;
    let upsample_left: libc::c_int = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, angle - 180i32, is_sm)
    } else {
        0i32
    };
    if upsample_left != 0 {
        upsample_edge(
            left_out.as_mut_ptr(),
            width + height,
            &*topleft_in.offset(-(width + height) as isize),
            imax(width - height, 0i32),
            width + height + 1i32,
        );
        left = &mut *left_out
            .as_mut_ptr()
            .offset((2i32 * (width + height) - 2i32) as isize) as *mut pixel;
        max_base_y = 2i32 * (width + height) - 2i32;
        dy <<= 1i32;
    } else {
        let filter_strength: libc::c_int = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, angle - 180i32, is_sm)
        } else {
            0i32
        };
        if filter_strength != 0 {
            filter_edge(
                left_out.as_mut_ptr(),
                width + height,
                0i32,
                width + height,
                &*topleft_in.offset(-(width + height) as isize),
                imax(width - height, 0i32),
                width + height + 1i32,
                filter_strength,
            );
            left = &mut *left_out
                .as_mut_ptr()
                .offset((width + height - 1i32) as isize) as *mut pixel;
            max_base_y = width + height - 1i32;
        } else {
            left = &*topleft_in.offset(-1isize) as *const pixel;
            max_base_y = height + imin(width, height) - 1i32;
        }
    }
    let base_inc: libc::c_int = 1i32 + upsample_left;
    let mut x: libc::c_int = 0i32;
    let mut ypos: libc::c_int = dy;
    while x < width {
        let frac: libc::c_int = ypos & 0x3ei32;
        let mut y: libc::c_int = 0i32;
        let mut base: libc::c_int = ypos >> 6i32;
        while y < height {
            if base < max_base_y {
                let v: libc::c_int = *left.offset(-base as isize) as libc::c_int * (64i32 - frac)
                    + *left.offset(-(base + 1i32) as isize) as libc::c_int * frac;
                *dst.offset((y as libc::c_long * stride + x as libc::c_long) as isize) =
                    (v + 32i32 >> 6i32) as pixel;
                y += 1;
                base += base_inc;
            } else {
                loop {
                    *dst.offset((y as libc::c_long * stride + x as libc::c_long) as isize) =
                        *left.offset(-max_base_y as isize);
                    y += 1;
                    if !(y < height) {
                        break;
                    }
                }
                break;
            }
        }
        x += 1;
        ypos += dy;
    }
}
unsafe extern "C" fn ipred_filter_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft_in: *const pixel,
    width: libc::c_int,
    height: libc::c_int,
    mut filt_idx: libc::c_int,
    max_width: libc::c_int,
    max_height: libc::c_int,
) {
    filt_idx &= 511i32;
    if !(filt_idx < 5i32) {
        unreachable!();
    }
    let filter: *const int8_t = (dav1d_filter_intra_taps[filt_idx as usize]).as_ptr();
    let mut top: *const pixel = &*topleft_in.offset(1isize) as *const pixel;
    let mut y: libc::c_int = 0i32;
    while y < height {
        let mut topleft: *const pixel = &*topleft_in.offset(-y as isize) as *const pixel;
        let mut left: *const pixel = &*topleft.offset(-1isize) as *const pixel;
        let mut left_stride: ptrdiff_t = -1i64;
        let mut x: libc::c_int = 0i32;
        while x < width {
            let p0: libc::c_int = *topleft as libc::c_int;
            let p1: libc::c_int = *top.offset(0isize) as libc::c_int;
            let p2: libc::c_int = *top.offset(1isize) as libc::c_int;
            let p3: libc::c_int = *top.offset(2isize) as libc::c_int;
            let p4: libc::c_int = *top.offset(3isize) as libc::c_int;
            let p5: libc::c_int = *left.offset((0i64 * left_stride) as isize) as libc::c_int;
            let p6: libc::c_int = *left.offset((1i64 * left_stride) as isize) as libc::c_int;
            let mut ptr: *mut pixel = &mut *dst.offset(x as isize) as *mut pixel;
            let mut flt_ptr: *const int8_t = filter;
            let mut yy: libc::c_int = 0i32;
            while yy < 2i32 {
                let mut xx: libc::c_int = 0i32;
                while xx < 4i32 {
                    let acc: libc::c_int = *flt_ptr.offset(0isize) as libc::c_int * p0
                        + *flt_ptr.offset(1isize) as libc::c_int * p1
                        + *flt_ptr.offset(16isize) as libc::c_int * p2
                        + *flt_ptr.offset(17isize) as libc::c_int * p3
                        + *flt_ptr.offset(32isize) as libc::c_int * p4
                        + *flt_ptr.offset(33isize) as libc::c_int * p5
                        + *flt_ptr.offset(48isize) as libc::c_int * p6;
                    *ptr.offset(xx as isize) = iclip_u8(acc + 8i32 >> 4i32) as pixel;
                    xx += 1;
                    flt_ptr = flt_ptr.offset(2isize);
                }
                ptr = ptr.offset(stride as isize);
                yy += 1;
            }
            left = &mut *dst.offset((x + 4i32 - 1i32) as isize) as *mut pixel;
            left_stride = stride;
            top = top.offset(4isize);
            topleft = &*top.offset(-1isize) as *const pixel;
            x += 4i32;
        }
        top = &mut *dst.offset(stride as isize) as *mut pixel;
        dst = &mut *dst.offset((stride * 2i64) as isize) as *mut pixel;
        y += 2i32;
    }
}
#[inline(never)]
unsafe extern "C" fn cfl_ac_c(
    mut ac: *mut int16_t,
    mut ypx: *const pixel,
    stride: ptrdiff_t,
    w_pad: libc::c_int,
    h_pad: libc::c_int,
    width: libc::c_int,
    height: libc::c_int,
    ss_hor: libc::c_int,
    ss_ver: libc::c_int,
) {
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let ac_orig: *mut int16_t = ac;
    if !(w_pad >= 0i32 && (w_pad * 4i32) < width) {
        unreachable!();
    }
    if !(h_pad >= 0i32 && (h_pad * 4i32) < height) {
        unreachable!();
    }
    y = 0i32;
    while y < height - 4i32 * h_pad {
        x = 0i32;
        while x < width - 4i32 * w_pad {
            let mut ac_sum: libc::c_int = *ypx.offset((x << ss_hor) as isize) as libc::c_int;
            if ss_hor != 0 {
                ac_sum += *ypx.offset((x * 2i32 + 1i32) as isize) as libc::c_int;
            }
            if ss_ver != 0 {
                ac_sum +=
                    *ypx.offset(((x << ss_hor) as libc::c_long + stride) as isize) as libc::c_int;
                if ss_hor != 0 {
                    ac_sum += *ypx.offset(((x * 2i32 + 1i32) as libc::c_long + stride) as isize)
                        as libc::c_int;
                }
            }
            *ac.offset(x as isize) = (ac_sum
                << 1i32 + (ss_ver == 0) as libc::c_int + (ss_hor == 0) as libc::c_int)
                as int16_t;
            x += 1;
        }
        while x < width {
            *ac.offset(x as isize) = *ac.offset((x - 1i32) as isize);
            x += 1;
        }
        ac = ac.offset(width as isize);
        ypx = ypx.offset((stride << ss_ver) as isize);
        y += 1;
    }
    while y < height {
        memcpy(
            ac as *mut libc::c_void,
            &mut *ac.offset(-width as isize) as *mut int16_t as *const libc::c_void,
            (width as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong),
        );
        ac = ac.offset(width as isize);
        y += 1;
    }
    let log2sz: libc::c_int = ctz(width as libc::c_uint) + ctz(height as libc::c_uint);
    let mut sum: libc::c_int = (1i32) << log2sz >> 1i32;
    ac = ac_orig;
    y = 0i32;
    while y < height {
        x = 0i32;
        while x < width {
            sum += *ac.offset(x as isize) as libc::c_int;
            x += 1;
        }
        ac = ac.offset(width as isize);
        y += 1;
    }
    sum >>= log2sz;
    ac = ac_orig;
    y = 0i32;
    while y < height {
        x = 0i32;
        while x < width {
            let ref mut fresh0 = *ac.offset(x as isize);
            *fresh0 = (*fresh0 as libc::c_int - sum) as int16_t;
            x += 1;
        }
        ac = ac.offset(width as isize);
        y += 1;
    }
}
unsafe extern "C" fn cfl_ac_420_c(
    ac: *mut int16_t,
    ypx: *const pixel,
    stride: ptrdiff_t,
    w_pad: libc::c_int,
    h_pad: libc::c_int,
    cw: libc::c_int,
    ch: libc::c_int,
) {
    cfl_ac_c(ac, ypx, stride, w_pad, h_pad, cw, ch, 1i32, 1i32);
}
unsafe extern "C" fn cfl_ac_422_c(
    ac: *mut int16_t,
    ypx: *const pixel,
    stride: ptrdiff_t,
    w_pad: libc::c_int,
    h_pad: libc::c_int,
    cw: libc::c_int,
    ch: libc::c_int,
) {
    cfl_ac_c(ac, ypx, stride, w_pad, h_pad, cw, ch, 1i32, 0i32);
}
unsafe extern "C" fn cfl_ac_444_c(
    ac: *mut int16_t,
    ypx: *const pixel,
    stride: ptrdiff_t,
    w_pad: libc::c_int,
    h_pad: libc::c_int,
    cw: libc::c_int,
    ch: libc::c_int,
) {
    cfl_ac_c(ac, ypx, stride, w_pad, h_pad, cw, ch, 0i32, 0i32);
}
unsafe extern "C" fn pal_pred_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    pal: *const uint16_t,
    mut idx: *const uint8_t,
    w: libc::c_int,
    h: libc::c_int,
) {
    let mut y: libc::c_int = 0i32;
    while y < h {
        let mut x: libc::c_int = 0i32;
        while x < w {
            *dst.offset(x as isize) = *pal.offset(*idx.offset(x as isize) as isize) as pixel;
            x += 1;
        }
        idx = idx.offset(w as isize);
        dst = dst.offset(stride as isize);
        y += 1;
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_intra_pred_dsp_init_8bpc(c: *mut Dav1dIntraPredDSPContext) {
    (*c).intra_pred[DC_PRED as usize] = Some(
        ipred_dc_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).intra_pred[DC_128_PRED as usize] = Some(
        ipred_dc_128_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).intra_pred[TOP_DC_PRED as usize] = Some(
        ipred_dc_top_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).intra_pred[LEFT_DC_PRED as usize] = Some(
        ipred_dc_left_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).intra_pred[HOR_PRED as usize] = Some(
        ipred_h_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).intra_pred[VERT_PRED as usize] = Some(
        ipred_v_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).intra_pred[PAETH_PRED as usize] = Some(
        ipred_paeth_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).intra_pred[SMOOTH_PRED as usize] = Some(
        ipred_smooth_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).intra_pred[SMOOTH_V_PRED as usize] = Some(
        ipred_smooth_v_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).intra_pred[SMOOTH_H_PRED as usize] = Some(
        ipred_smooth_h_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).intra_pred[Z1_PRED as usize] = Some(
        ipred_z1_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).intra_pred[Z2_PRED as usize] = Some(
        ipred_z2_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).intra_pred[Z3_PRED as usize] = Some(
        ipred_z3_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).intra_pred[FILTER_PRED as usize] = Some(
        ipred_filter_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).cfl_ac[(DAV1D_PIXEL_LAYOUT_I420 as libc::c_int - 1i32) as usize] = Some(
        cfl_ac_420_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).cfl_ac[(DAV1D_PIXEL_LAYOUT_I422 as libc::c_int - 1i32) as usize] = Some(
        cfl_ac_422_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).cfl_ac[(DAV1D_PIXEL_LAYOUT_I444 as libc::c_int - 1i32) as usize] = Some(
        cfl_ac_444_c
            as unsafe extern "C" fn(
                *mut int16_t,
                *const pixel,
                ptrdiff_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*c).cfl_pred[DC_PRED as usize] = Some(
        ipred_cfl_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                libc::c_int,
                libc::c_int,
                *const int16_t,
                libc::c_int,
            ) -> (),
    );
    (*c).cfl_pred[DC_128_PRED as usize] = Some(
        ipred_cfl_128_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                libc::c_int,
                libc::c_int,
                *const int16_t,
                libc::c_int,
            ) -> (),
    );
    (*c).cfl_pred[TOP_DC_PRED as usize] = Some(
        ipred_cfl_top_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                libc::c_int,
                libc::c_int,
                *const int16_t,
                libc::c_int,
            ) -> (),
    );
    (*c).cfl_pred[LEFT_DC_PRED as usize] = Some(
        ipred_cfl_left_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const pixel,
                libc::c_int,
                libc::c_int,
                *const int16_t,
                libc::c_int,
            ) -> (),
    );
    (*c).pal_pred = Some(
        pal_pred_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *const uint16_t,
                *const uint8_t,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
}
