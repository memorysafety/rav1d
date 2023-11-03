use crate::include::common::attributes::ctz;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::BitDepth8;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::intops::iclip;
use crate::include::common::intops::iclip_u8;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::src::ipred::cfl_pred;
use crate::src::ipred::dc_gen_top;
use crate::src::ipred::get_filter_strength;
use crate::src::ipred::get_upsample;
use crate::src::ipred::splat_dc;
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
use crate::src::tables::dav1d_dr_intra_derivative;
use crate::src::tables::dav1d_filter_intra_taps;
use crate::src::tables::dav1d_sm_weights;
use libc::memcpy;
use libc::memset;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_void;

#[cfg(feature = "asm")]
use crate::src::cpu::{rav1d_get_cpu_flags, CpuFlags};

#[cfg(feature = "asm")]
use cfg_if::cfg_if;

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
extern "C" {
    fn dav1d_ipred_z1_fill2_8bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        top: *const pixel,
        width: c_int,
        height: c_int,
        dx: c_int,
        max_base_x: c_int,
    );
    fn dav1d_ipred_z1_fill1_8bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        top: *const pixel,
        width: c_int,
        height: c_int,
        dx: c_int,
        max_base_x: c_int,
    );
    fn dav1d_ipred_z1_upsample_edge_8bpc_neon(
        out: *mut pixel,
        hsz: c_int,
        in_0: *const pixel,
        end: c_int,
    );
    fn dav1d_ipred_z1_filter_edge_8bpc_neon(
        out: *mut pixel,
        sz: c_int,
        in_0: *const pixel,
        end: c_int,
        strength: c_int,
    );
    fn dav1d_ipred_z2_fill3_8bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        top: *const pixel,
        left: *const pixel,
        width: c_int,
        height: c_int,
        dx: c_int,
        dy: c_int,
    );
    fn dav1d_ipred_z2_fill2_8bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        top: *const pixel,
        left: *const pixel,
        width: c_int,
        height: c_int,
        dx: c_int,
        dy: c_int,
    );
    fn dav1d_ipred_z2_fill1_8bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        top: *const pixel,
        left: *const pixel,
        width: c_int,
        height: c_int,
        dx: c_int,
        dy: c_int,
    );
    fn dav1d_ipred_z2_upsample_edge_8bpc_neon(out: *mut pixel, hsz: c_int, in_0: *const pixel);
    fn dav1d_ipred_reverse_8bpc_neon(dst: *mut pixel, src: *const pixel, n: c_int);
    fn dav1d_ipred_z3_fill2_8bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: *const pixel,
        width: c_int,
        height: c_int,
        dy: c_int,
        max_base_y: c_int,
    );
    fn dav1d_ipred_z3_fill1_8bpc_neon(
        dst: *mut pixel,
        stride: ptrdiff_t,
        left: *const pixel,
        width: c_int,
        height: c_int,
        dy: c_int,
        max_base_y: c_int,
    );
    fn dav1d_ipred_pixel_set_8bpc_neon(out: *mut pixel, px: pixel, n: c_int);
}

pub type pixel = u8;

unsafe extern "C" fn ipred_dc_top_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
) {
    splat_dc::<BitDepth8>(
        dst.cast(),
        stride,
        width,
        height,
        dc_gen_top::<BitDepth8>(topleft.cast(), width) as c_int,
        BitDepth8::new(()),
    );
}

unsafe extern "C" fn ipred_cfl_top_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    ac: *const i16,
    alpha: c_int,
    _bitdepth_max: c_int,
) {
    cfl_pred::<BitDepth8>(
        dst.cast(),
        stride,
        width,
        height,
        dc_gen_top::<BitDepth8>(topleft.cast(), width) as c_int,
        ac,
        alpha,
        BitDepth8::new(()),
    );
}

unsafe fn dc_gen_left(topleft: *const pixel, height: c_int) -> c_uint {
    let mut dc: c_uint = (height >> 1) as c_uint;
    let mut i = 0;
    while i < height {
        dc = dc.wrapping_add(*topleft.offset(-(1 + i) as isize) as c_uint);
        i += 1;
    }
    return dc >> ctz(height as c_uint);
}

unsafe extern "C" fn ipred_dc_left_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
) {
    splat_dc::<BitDepth8>(
        dst.cast(),
        stride,
        width,
        height,
        dc_gen_left(topleft.cast(), height) as c_int,
        BitDepth8::new(()),
    );
}

unsafe extern "C" fn ipred_cfl_left_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    ac: *const i16,
    alpha: c_int,
    _bitdepth_max: c_int,
) {
    let dc: c_uint = dc_gen_left(topleft.cast(), height);
    cfl_pred::<BitDepth8>(
        dst.cast(),
        stride,
        width,
        height,
        dc as c_int,
        ac,
        alpha,
        BitDepth8::new(()),
    );
}

unsafe fn dc_gen(topleft: *const pixel, width: c_int, height: c_int) -> c_uint {
    let mut dc: c_uint = (width + height >> 1) as c_uint;
    let mut i = 0;
    while i < width {
        dc = dc.wrapping_add(*topleft.offset((i + 1) as isize) as c_uint);
        i += 1;
    }
    let mut i_0 = 0;
    while i_0 < height {
        dc = dc.wrapping_add(*topleft.offset(-(i_0 + 1) as isize) as c_uint);
        i_0 += 1;
    }
    dc >>= ctz((width + height) as c_uint);
    if width != height {
        dc = dc.wrapping_mul(
            (if width > height * 2 || height > width * 2 {
                0x3334 as c_int
            } else {
                0x5556 as c_int
            }) as c_uint,
        );
        dc >>= 16;
    }
    return dc;
}

unsafe extern "C" fn ipred_dc_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
) {
    splat_dc::<BitDepth8>(
        dst.cast(),
        stride,
        width,
        height,
        dc_gen(topleft.cast(), width, height) as c_int,
        BitDepth8::new(()),
    );
}

unsafe extern "C" fn ipred_cfl_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    ac: *const i16,
    alpha: c_int,
    _bitdepth_max: c_int,
) {
    let dc: c_uint = dc_gen(topleft.cast(), width, height);
    cfl_pred::<BitDepth8>(
        dst.cast(),
        stride,
        width,
        height,
        dc as c_int,
        ac,
        alpha,
        BitDepth8::new(()),
    );
}

unsafe extern "C" fn ipred_dc_128_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    _topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
) {
    let dc = 128;
    splat_dc::<BitDepth8>(dst.cast(), stride, width, height, dc, BitDepth8::new(()));
}

unsafe extern "C" fn ipred_cfl_128_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    _topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    ac: *const i16,
    alpha: c_int,
    _bitdepth_max: c_int,
) {
    let dc = 128;
    cfl_pred::<BitDepth8>(
        dst.cast(),
        stride,
        width,
        height,
        dc,
        ac,
        alpha,
        BitDepth8::new(()),
    );
}

unsafe extern "C" fn ipred_v_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    a: c_int,
    max_width: c_int,
    max_height: c_int,
    _bitdepth_max: c_int,
) {
    ipred_v_rust(
        dst.cast(),
        stride,
        topleft.cast(),
        width,
        height,
        a,
        max_width,
        max_height,
    );
}

unsafe fn ipred_v_rust(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let mut y = 0;
    while y < height {
        memcpy(
            dst as *mut c_void,
            topleft.offset(1) as *const c_void,
            width as usize,
        );
        dst = dst.offset(stride as isize);
        y += 1;
    }
}

unsafe extern "C" fn ipred_h_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    a: c_int,
    max_width: c_int,
    max_height: c_int,
    _bitdepth_max: c_int,
) {
    ipred_h_rust(
        dst.cast(),
        stride,
        topleft.cast(),
        width,
        height,
        a,
        max_width,
        max_height,
    );
}

unsafe fn ipred_h_rust(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let mut y = 0;
    while y < height {
        memset(
            dst as *mut c_void,
            *topleft.offset(-(1 + y) as isize) as c_int,
            width as usize,
        );
        dst = dst.offset(stride as isize);
        y += 1;
    }
}

unsafe extern "C" fn ipred_paeth_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    tl_ptr: *const DynPixel,
    width: c_int,
    height: c_int,
    a: c_int,
    max_width: c_int,
    max_height: c_int,
    _bitdepth_max: c_int,
) {
    ipred_paeth_rust(
        dst.cast(),
        stride,
        tl_ptr.cast(),
        width,
        height,
        a,
        max_width,
        max_height,
    );
}

unsafe fn ipred_paeth_rust(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    tl_ptr: *const pixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let topleft = *tl_ptr.offset(0) as c_int;
    let mut y = 0;
    while y < height {
        let left = *tl_ptr.offset(-(y + 1) as isize) as c_int;
        let mut x = 0;
        while x < width {
            let top = *tl_ptr.offset((1 + x) as isize) as c_int;
            let base = left + top - topleft;
            let ldiff = (left - base).abs();
            let tdiff = (top - base).abs();
            let tldiff = (topleft - base).abs();
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

unsafe extern "C" fn ipred_smooth_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    a: c_int,
    max_width: c_int,
    max_height: c_int,
    _bitdepth_max: c_int,
) {
    ipred_smooth_rust(
        dst.cast(),
        stride,
        topleft.cast(),
        width,
        height,
        a,
        max_width,
        max_height,
    );
}

unsafe fn ipred_smooth_rust(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let weights_hor: *const u8 = &*dav1d_sm_weights.0.as_ptr().offset(width as isize) as *const u8;
    let weights_ver: *const u8 = &*dav1d_sm_weights.0.as_ptr().offset(height as isize) as *const u8;
    let right = *topleft.offset(width as isize) as c_int;
    let bottom = *topleft.offset(-height as isize) as c_int;
    let mut y = 0;
    while y < height {
        let mut x = 0;
        while x < width {
            let pred = *weights_ver.offset(y as isize) as c_int
                * *topleft.offset((1 + x) as isize) as c_int
                + (256 - *weights_ver.offset(y as isize) as c_int) * bottom
                + *weights_hor.offset(x as isize) as c_int
                    * *topleft.offset(-(1 + y) as isize) as c_int
                + (256 - *weights_hor.offset(x as isize) as c_int) * right;
            *dst.offset(x as isize) = (pred + 256 >> 9) as pixel;
            x += 1;
        }
        dst = dst.offset(stride as isize);
        y += 1;
    }
}

unsafe extern "C" fn ipred_smooth_v_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    a: c_int,
    max_width: c_int,
    max_height: c_int,
    _bitdepth_max: c_int,
) {
    ipred_smooth_v_rust(
        dst.cast(),
        stride,
        topleft.cast(),
        width,
        height,
        a,
        max_width,
        max_height,
    );
}

unsafe fn ipred_smooth_v_rust(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let weights_ver: *const u8 = &*dav1d_sm_weights.0.as_ptr().offset(height as isize) as *const u8;
    let bottom = *topleft.offset(-height as isize) as c_int;
    let mut y = 0;
    while y < height {
        let mut x = 0;
        while x < width {
            let pred = *weights_ver.offset(y as isize) as c_int
                * *topleft.offset((1 + x) as isize) as c_int
                + (256 - *weights_ver.offset(y as isize) as c_int) * bottom;
            *dst.offset(x as isize) = (pred + 128 >> 8) as pixel;
            x += 1;
        }
        dst = dst.offset(stride as isize);
        y += 1;
    }
}

unsafe extern "C" fn ipred_smooth_h_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    a: c_int,
    max_width: c_int,
    max_height: c_int,
    _bitdepth_max: c_int,
) {
    ipred_smooth_h_rust(
        dst.cast(),
        stride,
        topleft.cast(),
        width,
        height,
        a,
        max_width,
        max_height,
    );
}

unsafe fn ipred_smooth_h_rust(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft: *const pixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let weights_hor: *const u8 = &*dav1d_sm_weights.0.as_ptr().offset(width as isize) as *const u8;
    let right = *topleft.offset(width as isize) as c_int;
    let mut y = 0;
    while y < height {
        let mut x = 0;
        while x < width {
            let pred = *weights_hor.offset(x as isize) as c_int
                * *topleft.offset(-(y + 1) as isize) as c_int
                + (256 - *weights_hor.offset(x as isize) as c_int) * right;
            *dst.offset(x as isize) = (pred + 128 >> 8) as pixel;
            x += 1;
        }
        dst = dst.offset(stride as isize);
        y += 1;
    }
}

#[inline(never)]
unsafe fn filter_edge(
    out: *mut pixel,
    sz: c_int,
    lim_from: c_int,
    lim_to: c_int,
    in_0: *const pixel,
    from: c_int,
    to: c_int,
    strength: c_int,
) {
    static kernel: [[u8; 5]; 3] = [[0, 4, 8, 4, 0], [0, 5, 6, 5, 0], [2, 4, 4, 4, 2]];
    if !(strength > 0) {
        unreachable!();
    }
    let mut i = 0;
    while i < cmp::min(sz, lim_from) {
        *out.offset(i as isize) = *in_0.offset(iclip(i, from, to - 1) as isize);
        i += 1;
    }
    while i < cmp::min(lim_to, sz) {
        let mut s = 0;
        let mut j = 0;
        while j < 5 {
            s += *in_0.offset(iclip(i - 2 + j, from, to - 1) as isize) as c_int
                * kernel[(strength - 1) as usize][j as usize] as c_int;
            j += 1;
        }
        *out.offset(i as isize) = (s + 8 >> 4) as pixel;
        i += 1;
    }
    while i < sz {
        *out.offset(i as isize) = *in_0.offset(iclip(i, from, to - 1) as isize);
        i += 1;
    }
}

#[inline(never)]
unsafe fn upsample_edge(out: *mut pixel, hsz: c_int, in_0: *const pixel, from: c_int, to: c_int) {
    static kernel: [i8; 4] = [-1, 9, 9, -1];
    let mut i;
    i = 0 as c_int;
    while i < hsz - 1 {
        *out.offset((i * 2) as isize) = *in_0.offset(iclip(i, from, to - 1) as isize);
        let mut s = 0;
        let mut j = 0;
        while j < 4 {
            s += *in_0.offset(iclip(i + j - 1, from, to - 1) as isize) as c_int
                * kernel[j as usize] as c_int;
            j += 1;
        }
        *out.offset((i * 2 + 1) as isize) = iclip_u8(s + 8 >> 4) as pixel;
        i += 1;
    }
    *out.offset((i * 2) as isize) = *in_0.offset(iclip(i, from, to - 1) as isize);
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
    _bitdepth_max: c_int,
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
    );
}

unsafe fn ipred_z1_rust(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft_in: *const pixel,
    width: c_int,
    height: c_int,
    mut angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let is_sm = angle >> 9 & 0x1 as c_int;
    let enable_intra_edge_filter = angle >> 10;
    angle &= 511 as c_int;
    if !(angle < 90) {
        unreachable!();
    }
    let mut dx = dav1d_dr_intra_derivative[(angle >> 1) as usize] as c_int;
    let mut top_out: [pixel; 128] = [0; 128];
    let top: *const pixel;
    let max_base_x;
    let upsample_above = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, 90 - angle, is_sm)
    } else {
        0 as c_int
    };
    if upsample_above != 0 {
        upsample_edge(
            top_out.as_mut_ptr(),
            width + height,
            &*topleft_in.offset(1),
            -(1 as c_int),
            width + cmp::min(width, height),
        );
        top = top_out.as_mut_ptr();
        max_base_x = 2 * (width + height) - 2;
        dx <<= 1;
    } else {
        let filter_strength = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, 90 - angle, is_sm)
        } else {
            0 as c_int
        };
        if filter_strength != 0 {
            filter_edge(
                top_out.as_mut_ptr(),
                width + height,
                0 as c_int,
                width + height,
                &*topleft_in.offset(1),
                -(1 as c_int),
                width + cmp::min(width, height),
                filter_strength,
            );
            top = top_out.as_mut_ptr();
            max_base_x = width + height - 1;
        } else {
            top = &*topleft_in.offset(1) as *const pixel;
            max_base_x = width + cmp::min(width, height) - 1;
        }
    }
    let base_inc = 1 + upsample_above;
    let mut y = 0;
    let mut xpos = dx;
    while y < height {
        let frac = xpos & 0x3e as c_int;
        let mut x = 0;
        let mut base = xpos >> 6;
        while x < width {
            if base < max_base_x {
                let v = *top.offset(base as isize) as c_int * (64 - frac)
                    + *top.offset((base + 1) as isize) as c_int * frac;
                *dst.offset(x as isize) = (v + 32 >> 6) as pixel;
                x += 1;
                base += base_inc;
            } else {
                memset(
                    &mut *dst.offset(x as isize) as *mut pixel as *mut c_void,
                    *top.offset(max_base_x as isize) as c_int,
                    (width - x) as usize,
                );
                break;
            }
        }
        y += 1;
        dst = dst.offset(stride as isize);
        xpos += dx;
    }
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
    _bitdepth_max: c_int,
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
    );
}

unsafe fn ipred_z2_rust(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft_in: *const pixel,
    width: c_int,
    height: c_int,
    mut angle: c_int,
    max_width: c_int,
    max_height: c_int,
) {
    let is_sm = angle >> 9 & 0x1 as c_int;
    let enable_intra_edge_filter = angle >> 10;
    angle &= 511 as c_int;
    if !(angle > 90 && angle < 180) {
        unreachable!();
    }
    let mut dy = dav1d_dr_intra_derivative[(angle - 90 >> 1) as usize] as c_int;
    let mut dx = dav1d_dr_intra_derivative[(180 - angle >> 1) as usize] as c_int;
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
    let mut edge: [pixel; 129] = [0; 129];
    let topleft: *mut pixel = &mut *edge.as_mut_ptr().offset(64) as *mut pixel;
    if upsample_above != 0 {
        upsample_edge(topleft, width + 1, topleft_in, 0 as c_int, width + 1);
        dx <<= 1;
    } else {
        let filter_strength = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, angle - 90, is_sm)
        } else {
            0 as c_int
        };
        if filter_strength != 0 {
            filter_edge(
                &mut *topleft.offset(1),
                width,
                0 as c_int,
                max_width,
                &*topleft_in.offset(1),
                -(1 as c_int),
                width,
                filter_strength,
            );
        } else {
            memcpy(
                &mut *topleft.offset(1) as *mut pixel as *mut c_void,
                &*topleft_in.offset(1) as *const pixel as *const c_void,
                width as usize,
            );
        }
    }
    if upsample_left != 0 {
        upsample_edge(
            &mut *topleft.offset((-height * 2) as isize),
            height + 1,
            &*topleft_in.offset(-height as isize),
            0 as c_int,
            height + 1,
        );
        dy <<= 1;
    } else {
        let filter_strength_0 = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, 180 - angle, is_sm)
        } else {
            0 as c_int
        };
        if filter_strength_0 != 0 {
            filter_edge(
                &mut *topleft.offset(-height as isize),
                height,
                height - max_height,
                height,
                &*topleft_in.offset(-height as isize),
                0 as c_int,
                height + 1,
                filter_strength_0,
            );
        } else {
            memcpy(
                &mut *topleft.offset(-height as isize) as *mut pixel as *mut c_void,
                &*topleft_in.offset(-height as isize) as *const pixel as *const c_void,
                height as usize,
            );
        }
    }
    *topleft = *topleft_in;
    let base_inc_x = 1 + upsample_above;
    let left: *const pixel = &mut *topleft.offset(-(1 + upsample_left) as isize) as *mut pixel;
    let mut y = 0;
    let mut xpos = (1 + upsample_above << 6) - dx;
    while y < height {
        let mut base_x = xpos >> 6;
        let frac_x = xpos & 0x3e as c_int;
        let mut x = 0;
        let mut ypos = (y << 6 + upsample_left) - dy;
        while x < width {
            let v;
            if base_x >= 0 {
                v = *topleft.offset(base_x as isize) as c_int * (64 - frac_x)
                    + *topleft.offset((base_x + 1) as isize) as c_int * frac_x;
            } else {
                let base_y = ypos >> 6;
                if !(base_y >= -(1 + upsample_left)) {
                    unreachable!();
                }
                let frac_y = ypos & 0x3e as c_int;
                v = *left.offset(-base_y as isize) as c_int * (64 - frac_y)
                    + *left.offset(-(base_y + 1) as isize) as c_int * frac_y;
            }
            *dst.offset(x as isize) = (v + 32 >> 6) as pixel;
            x += 1;
            base_x += base_inc_x;
            ypos -= dy;
        }
        y += 1;
        xpos -= dx;
        dst = dst.offset(stride as isize);
    }
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
    _bitdepth_max: c_int,
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
    );
}

unsafe fn ipred_z3_rust(
    dst: *mut pixel,
    stride: ptrdiff_t,
    topleft_in: *const pixel,
    width: c_int,
    height: c_int,
    mut angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let is_sm = angle >> 9 & 0x1 as c_int;
    let enable_intra_edge_filter = angle >> 10;
    angle &= 511 as c_int;
    if !(angle > 180) {
        unreachable!();
    }
    let mut dy = dav1d_dr_intra_derivative[(270 - angle >> 1) as usize] as c_int;
    let mut left_out: [pixel; 128] = [0; 128];
    let left: *const pixel;
    let max_base_y;
    let upsample_left = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, angle - 180, is_sm)
    } else {
        0 as c_int
    };
    if upsample_left != 0 {
        upsample_edge(
            left_out.as_mut_ptr(),
            width + height,
            &*topleft_in.offset(-(width + height) as isize),
            cmp::max(width - height, 0 as c_int),
            width + height + 1,
        );
        left = &mut *left_out
            .as_mut_ptr()
            .offset((2 * (width + height) - 2) as isize) as *mut pixel;
        max_base_y = 2 * (width + height) - 2;
        dy <<= 1;
    } else {
        let filter_strength = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, angle - 180, is_sm)
        } else {
            0 as c_int
        };
        if filter_strength != 0 {
            filter_edge(
                left_out.as_mut_ptr(),
                width + height,
                0 as c_int,
                width + height,
                &*topleft_in.offset(-(width + height) as isize),
                cmp::max(width - height, 0 as c_int),
                width + height + 1,
                filter_strength,
            );
            left = &mut *left_out.as_mut_ptr().offset((width + height - 1) as isize) as *mut pixel;
            max_base_y = width + height - 1;
        } else {
            left = &*topleft_in.offset(-(1 as c_int) as isize) as *const pixel;
            max_base_y = height + cmp::min(width, height) - 1;
        }
    }
    let base_inc = 1 + upsample_left;
    let mut x = 0;
    let mut ypos = dy;
    while x < width {
        let frac = ypos & 0x3e as c_int;
        let mut y = 0;
        let mut base = ypos >> 6;
        while y < height {
            if base < max_base_y {
                let v = *left.offset(-base as isize) as c_int * (64 - frac)
                    + *left.offset(-(base + 1) as isize) as c_int * frac;
                *dst.offset((y as isize * stride + x as isize) as isize) = (v + 32 >> 6) as pixel;
                y += 1;
                base += base_inc;
            } else {
                loop {
                    *dst.offset((y as isize * stride + x as isize) as isize) =
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

unsafe extern "C" fn ipred_filter_c_erased(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft_in: *const DynPixel,
    width: c_int,
    height: c_int,
    filt_idx: c_int,
    max_width: c_int,
    max_height: c_int,
    _bitdepth_max: c_int,
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
    );
}

unsafe fn ipred_filter_rust(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    topleft_in: *const pixel,
    width: c_int,
    height: c_int,
    mut filt_idx: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    use crate::src::ipred::{filter_fn, FLT_INCR};

    filt_idx &= 511 as c_int;
    if !(filt_idx < 5) {
        unreachable!();
    }
    let filter: *const i8 = (dav1d_filter_intra_taps[filt_idx as usize]).as_ptr();
    let mut top: *const pixel = &*topleft_in.offset(1) as *const pixel;
    let mut y = 0;
    while y < height {
        let mut topleft: *const pixel = &*topleft_in.offset(-y as isize) as *const pixel;
        let mut left: *const pixel = &*topleft.offset(-(1 as c_int) as isize) as *const pixel;
        let mut left_stride: ptrdiff_t = -(1 as c_int) as ptrdiff_t;
        let mut x = 0;
        while x < width {
            let p0 = *topleft as c_int;
            let p1 = *top.offset(0) as c_int;
            let p2 = *top.offset(1) as c_int;
            let p3 = *top.offset(2) as c_int;
            let p4 = *top.offset(3) as c_int;
            let p5 = *left.offset((0 * left_stride) as isize) as c_int;
            let p6 = *left.offset((1 * left_stride) as isize) as c_int;
            let mut ptr: *mut pixel = &mut *dst.offset(x as isize) as *mut pixel;
            let mut flt_ptr: *const i8 = filter;
            let mut yy = 0;
            while yy < 2 {
                let mut xx = 0;
                while xx < 4 {
                    let acc = filter_fn(flt_ptr, p0, p1, p2, p3, p4, p5, p6);
                    *ptr.offset(xx as isize) = iclip_u8(acc + 8 >> 4) as pixel;
                    xx += 1;
                    flt_ptr = flt_ptr.offset(FLT_INCR);
                }
                ptr = ptr.offset(stride as isize);
                yy += 1;
            }
            left = &mut *dst.offset((x + 4 - 1) as isize) as *mut pixel;
            left_stride = stride;
            top = top.offset(4);
            topleft = &*top.offset(-(1 as c_int) as isize) as *const pixel;
            x += 4 as c_int;
        }
        top = &mut *dst.offset(stride as isize) as *mut pixel;
        dst = &mut *dst.offset((stride * 2) as isize) as *mut pixel;
        y += 2 as c_int;
    }
}

#[inline(never)]
unsafe fn cfl_ac_c(
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
    let mut x;
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
                ac_sum += *ypx.offset(((x << ss_hor) as isize + stride) as isize) as c_int;
                if ss_hor != 0 {
                    ac_sum += *ypx.offset(((x * 2 + 1) as isize + stride) as isize) as c_int;
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
        ypx = ypx.offset((stride << ss_ver) as isize);
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
    cfl_ac_c(
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
    cfl_ac_c(
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
    cfl_ac_c(
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
            *dst.offset(x as isize) = *pal.offset(*idx.offset(x as isize) as isize) as pixel;
            x += 1;
        }
        idx = idx.offset(w as isize);
        dst = dst.offset(stride as isize);
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

    (*c).intra_pred[DC_PRED as usize] = Some(dav1d_ipred_dc_8bpc_ssse3);
    (*c).intra_pred[DC_128_PRED as usize] = Some(dav1d_ipred_dc_128_8bpc_ssse3);
    (*c).intra_pred[TOP_DC_PRED as usize] = Some(dav1d_ipred_dc_top_8bpc_ssse3);
    (*c).intra_pred[LEFT_DC_PRED as usize] = Some(dav1d_ipred_dc_left_8bpc_ssse3);
    (*c).intra_pred[HOR_PRED as usize] = Some(dav1d_ipred_h_8bpc_ssse3);
    (*c).intra_pred[VERT_PRED as usize] = Some(dav1d_ipred_v_8bpc_ssse3);
    (*c).intra_pred[PAETH_PRED as usize] = Some(dav1d_ipred_paeth_8bpc_ssse3);
    (*c).intra_pred[SMOOTH_PRED as usize] = Some(dav1d_ipred_smooth_8bpc_ssse3);
    (*c).intra_pred[SMOOTH_H_PRED as usize] = Some(dav1d_ipred_smooth_h_8bpc_ssse3);
    (*c).intra_pred[SMOOTH_V_PRED as usize] = Some(dav1d_ipred_smooth_v_8bpc_ssse3);
    (*c).intra_pred[Z1_PRED as usize] = Some(dav1d_ipred_z1_8bpc_ssse3);
    (*c).intra_pred[Z2_PRED as usize] = Some(dav1d_ipred_z2_8bpc_ssse3);
    (*c).intra_pred[Z3_PRED as usize] = Some(dav1d_ipred_z3_8bpc_ssse3);
    (*c).intra_pred[FILTER_PRED as usize] = Some(dav1d_ipred_filter_8bpc_ssse3);

    (*c).cfl_pred[DC_PRED as usize] = dav1d_ipred_cfl_8bpc_ssse3;
    (*c).cfl_pred[DC_128_PRED as usize] = dav1d_ipred_cfl_128_8bpc_ssse3;
    (*c).cfl_pred[TOP_DC_PRED as usize] = dav1d_ipred_cfl_top_8bpc_ssse3;
    (*c).cfl_pred[LEFT_DC_PRED as usize] = dav1d_ipred_cfl_left_8bpc_ssse3;

    (*c).cfl_ac[Rav1dPixelLayout::I420 as usize - 1] = dav1d_ipred_cfl_ac_420_8bpc_ssse3;
    (*c).cfl_ac[Rav1dPixelLayout::I422 as usize - 1] = dav1d_ipred_cfl_ac_422_8bpc_ssse3;
    (*c).cfl_ac[Rav1dPixelLayout::I444 as usize - 1] = dav1d_ipred_cfl_ac_444_8bpc_ssse3;

    (*c).pal_pred = dav1d_pal_pred_8bpc_ssse3;

    #[cfg(target_arch = "x86_64")]
    {
        if !flags.contains(CpuFlags::AVX2) {
            return;
        }

        (*c).intra_pred[DC_PRED as usize] = Some(dav1d_ipred_dc_8bpc_avx2);
        (*c).intra_pred[DC_128_PRED as usize] = Some(dav1d_ipred_dc_128_8bpc_avx2);
        (*c).intra_pred[TOP_DC_PRED as usize] = Some(dav1d_ipred_dc_top_8bpc_avx2);
        (*c).intra_pred[LEFT_DC_PRED as usize] = Some(dav1d_ipred_dc_left_8bpc_avx2);
        (*c).intra_pred[HOR_PRED as usize] = Some(dav1d_ipred_h_8bpc_avx2);
        (*c).intra_pred[VERT_PRED as usize] = Some(dav1d_ipred_v_8bpc_avx2);
        (*c).intra_pred[PAETH_PRED as usize] = Some(dav1d_ipred_paeth_8bpc_avx2);
        (*c).intra_pred[SMOOTH_PRED as usize] = Some(dav1d_ipred_smooth_8bpc_avx2);
        (*c).intra_pred[SMOOTH_H_PRED as usize] = Some(dav1d_ipred_smooth_h_8bpc_avx2);
        (*c).intra_pred[SMOOTH_V_PRED as usize] = Some(dav1d_ipred_smooth_v_8bpc_avx2);
        (*c).intra_pred[Z1_PRED as usize] = Some(dav1d_ipred_z1_8bpc_avx2);
        (*c).intra_pred[Z2_PRED as usize] = Some(dav1d_ipred_z2_8bpc_avx2);
        (*c).intra_pred[Z3_PRED as usize] = Some(dav1d_ipred_z3_8bpc_avx2);
        (*c).intra_pred[FILTER_PRED as usize] = Some(dav1d_ipred_filter_8bpc_avx2);

        (*c).cfl_pred[DC_PRED as usize] = dav1d_ipred_cfl_8bpc_avx2;
        (*c).cfl_pred[DC_128_PRED as usize] = dav1d_ipred_cfl_128_8bpc_avx2;
        (*c).cfl_pred[TOP_DC_PRED as usize] = dav1d_ipred_cfl_top_8bpc_avx2;
        (*c).cfl_pred[LEFT_DC_PRED as usize] = dav1d_ipred_cfl_left_8bpc_avx2;

        (*c).cfl_ac[Rav1dPixelLayout::I420 as usize - 1] = dav1d_ipred_cfl_ac_420_8bpc_avx2;
        (*c).cfl_ac[Rav1dPixelLayout::I422 as usize - 1] = dav1d_ipred_cfl_ac_422_8bpc_avx2;
        (*c).cfl_ac[Rav1dPixelLayout::I444 as usize - 1] = dav1d_ipred_cfl_ac_444_8bpc_avx2;

        (*c).pal_pred = dav1d_pal_pred_8bpc_avx2;

        if !flags.contains(CpuFlags::AVX512ICL) {
            return;
        }

        (*c).intra_pred[DC_PRED as usize] = Some(dav1d_ipred_dc_8bpc_avx512icl);
        (*c).intra_pred[DC_128_PRED as usize] = Some(dav1d_ipred_dc_128_8bpc_avx512icl);
        (*c).intra_pred[TOP_DC_PRED as usize] = Some(dav1d_ipred_dc_top_8bpc_avx512icl);
        (*c).intra_pred[LEFT_DC_PRED as usize] = Some(dav1d_ipred_dc_left_8bpc_avx512icl);
        (*c).intra_pred[HOR_PRED as usize] = Some(dav1d_ipred_h_8bpc_avx512icl);
        (*c).intra_pred[VERT_PRED as usize] = Some(dav1d_ipred_v_8bpc_avx512icl);
        (*c).intra_pred[PAETH_PRED as usize] = Some(dav1d_ipred_paeth_8bpc_avx512icl);
        (*c).intra_pred[SMOOTH_PRED as usize] = Some(dav1d_ipred_smooth_8bpc_avx512icl);
        (*c).intra_pred[SMOOTH_H_PRED as usize] = Some(dav1d_ipred_smooth_h_8bpc_avx512icl);
        (*c).intra_pred[SMOOTH_V_PRED as usize] = Some(dav1d_ipred_smooth_v_8bpc_avx512icl);
        (*c).intra_pred[FILTER_PRED as usize] = Some(dav1d_ipred_filter_8bpc_avx512icl);

        (*c).pal_pred = dav1d_pal_pred_8bpc_avx512icl;
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

    (*c).intra_pred[DC_PRED as usize] = Some(dav1d_ipred_dc_8bpc_neon);
    (*c).intra_pred[DC_128_PRED as usize] = Some(dav1d_ipred_dc_128_8bpc_neon);
    (*c).intra_pred[TOP_DC_PRED as usize] = Some(dav1d_ipred_dc_top_8bpc_neon);
    (*c).intra_pred[LEFT_DC_PRED as usize] = Some(dav1d_ipred_dc_left_8bpc_neon);
    (*c).intra_pred[HOR_PRED as usize] = Some(dav1d_ipred_h_8bpc_neon);
    (*c).intra_pred[VERT_PRED as usize] = Some(dav1d_ipred_v_8bpc_neon);
    (*c).intra_pred[PAETH_PRED as usize] = Some(dav1d_ipred_paeth_8bpc_neon);
    (*c).intra_pred[SMOOTH_PRED as usize] = Some(dav1d_ipred_smooth_8bpc_neon);
    (*c).intra_pred[SMOOTH_V_PRED as usize] = Some(dav1d_ipred_smooth_v_8bpc_neon);
    (*c).intra_pred[SMOOTH_H_PRED as usize] = Some(dav1d_ipred_smooth_h_8bpc_neon);
    #[cfg(target_arch = "aarch64")]
    {
        (*c).intra_pred[Z1_PRED as usize] = Some(ipred_z1_neon_erased);
        (*c).intra_pred[Z2_PRED as usize] = Some(ipred_z2_neon_erased);
        (*c).intra_pred[Z3_PRED as usize] = Some(ipred_z3_neon_erased);
    }
    (*c).intra_pred[FILTER_PRED as usize] = Some(dav1d_ipred_filter_8bpc_neon);

    (*c).cfl_pred[DC_PRED as usize] = dav1d_ipred_cfl_8bpc_neon;
    (*c).cfl_pred[DC_128_PRED as usize] = dav1d_ipred_cfl_128_8bpc_neon;
    (*c).cfl_pred[TOP_DC_PRED as usize] = dav1d_ipred_cfl_top_8bpc_neon;
    (*c).cfl_pred[LEFT_DC_PRED as usize] = dav1d_ipred_cfl_left_8bpc_neon;

    (*c).cfl_ac[Rav1dPixelLayout::I420 as usize - 1] = dav1d_ipred_cfl_ac_420_8bpc_neon;
    (*c).cfl_ac[Rav1dPixelLayout::I422 as usize - 1] = dav1d_ipred_cfl_ac_422_8bpc_neon;
    (*c).cfl_ac[Rav1dPixelLayout::I444 as usize - 1] = dav1d_ipred_cfl_ac_444_8bpc_neon;

    (*c).pal_pred = dav1d_pal_pred_8bpc_neon;
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
    _bitdepth_max: c_int,
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
        dav1d_ipred_reverse_8bpc_neon(
            &mut *flipped.as_mut_ptr().offset(1),
            &*topleft_in.offset(0),
            height + cmp::max(width, height),
        );
        dav1d_ipred_z1_upsample_edge_8bpc_neon(
            left_out.as_mut_ptr(),
            width + height,
            flipped.as_mut_ptr(),
            height + cmp::min(width, height),
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
            dav1d_ipred_reverse_8bpc_neon(
                &mut *flipped.as_mut_ptr().offset(1),
                &*topleft_in.offset(0),
                height + cmp::max(width, height),
            );
            dav1d_ipred_z1_filter_edge_8bpc_neon(
                left_out.as_mut_ptr(),
                width + height,
                flipped.as_mut_ptr(),
                height + cmp::min(width, height),
                filter_strength,
            );
            max_base_y = width + height - 1;
        } else {
            dav1d_ipred_reverse_8bpc_neon(
                left_out.as_mut_ptr(),
                &*topleft_in.offset(0),
                height + cmp::min(width, height),
            );
            max_base_y = height + cmp::min(width, height) - 1;
        }
    }
    let base_inc = 1 + upsample_left;
    let pad_pixels = cmp::max(64 - max_base_y - 1, height + 15);
    dav1d_ipred_pixel_set_8bpc_neon(
        &mut *left_out.as_mut_ptr().offset((max_base_y + 1) as isize) as *mut pixel,
        left_out[max_base_y as usize],
        (pad_pixels * base_inc) as c_int,
    );
    if upsample_left != 0 {
        dav1d_ipred_z3_fill2_8bpc_neon(
            dst,
            stride,
            left_out.as_mut_ptr(),
            width,
            height,
            dy,
            max_base_y,
        );
    } else {
        dav1d_ipred_z3_fill1_8bpc_neon(
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
    _bitdepth_max: c_int,
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
        dav1d_ipred_z2_upsample_edge_8bpc_neon(
            buf.as_mut_ptr().offset(top_offset),
            width,
            topleft_in,
        );
        dx <<= 1;
    } else {
        let filter_strength = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, angle - 90, is_sm)
        } else {
            0 as c_int
        };

        if filter_strength != 0 {
            dav1d_ipred_z1_filter_edge_8bpc_neon(
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
            BitDepth8::pixel_copy(
                &mut buf[1 + top_offset as usize..],
                core::slice::from_raw_parts(topleft_in.offset(1), width as usize),
                width as usize,
            );
        }
    }

    if upsample_left != 0 {
        buf[flipped_offset as usize] = *topleft_in;
        dav1d_ipred_reverse_8bpc_neon(
            &mut *buf.as_mut_ptr().offset(1 + flipped_offset),
            topleft_in,
            height,
        );
        dav1d_ipred_z2_upsample_edge_8bpc_neon(
            buf.as_mut_ptr().offset(left_offset),
            height,
            buf.as_ptr().offset(flipped_offset),
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
            dav1d_ipred_reverse_8bpc_neon(
                &mut *buf.as_mut_ptr().offset(1 + flipped_offset),
                topleft_in,
                height,
            );
            dav1d_ipred_z1_filter_edge_8bpc_neon(
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
            dav1d_ipred_reverse_8bpc_neon(
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
        dav1d_ipred_z2_fill1_8bpc_neon(
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
        dav1d_ipred_z2_fill2_8bpc_neon(
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
        dav1d_ipred_z2_fill3_8bpc_neon(
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
    _bitdepth_max: c_int,
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
        dav1d_ipred_z1_upsample_edge_8bpc_neon(
            top_out.as_mut_ptr(),
            width + height,
            topleft_in,
            width + cmp::min(width, height),
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
            dav1d_ipred_z1_filter_edge_8bpc_neon(
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
    dav1d_ipred_pixel_set_8bpc_neon(
        &mut *top_out.as_mut_ptr().offset((max_base_x + 1) as isize) as *mut pixel,
        top_out[max_base_x as usize],
        (pad_pixels * base_inc) as c_int,
    );
    if upsample_above != 0 {
        dav1d_ipred_z1_fill2_8bpc_neon(
            dst,
            stride,
            top_out.as_mut_ptr(),
            width,
            height,
            dx,
            max_base_x,
        );
    } else {
        dav1d_ipred_z1_fill1_8bpc_neon(
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
pub unsafe fn rav1d_intra_pred_dsp_init_8bpc(c: *mut Rav1dIntraPredDSPContext) {
    (*c).intra_pred[DC_PRED as usize] = Some(ipred_dc_c_erased);
    (*c).intra_pred[DC_128_PRED as usize] = Some(ipred_dc_128_c_erased);
    (*c).intra_pred[TOP_DC_PRED as usize] = Some(ipred_dc_top_c_erased);
    (*c).intra_pred[LEFT_DC_PRED as usize] = Some(ipred_dc_left_c_erased);
    (*c).intra_pred[HOR_PRED as usize] = Some(ipred_h_c_erased);
    (*c).intra_pred[VERT_PRED as usize] = Some(ipred_v_c_erased);
    (*c).intra_pred[PAETH_PRED as usize] = Some(ipred_paeth_c_erased);
    (*c).intra_pred[SMOOTH_PRED as usize] = Some(ipred_smooth_c_erased);
    (*c).intra_pred[SMOOTH_V_PRED as usize] = Some(ipred_smooth_v_c_erased);
    (*c).intra_pred[SMOOTH_H_PRED as usize] = Some(ipred_smooth_h_c_erased);
    (*c).intra_pred[Z1_PRED as usize] = Some(ipred_z1_c_erased);
    (*c).intra_pred[Z2_PRED as usize] = Some(ipred_z2_c_erased);
    (*c).intra_pred[Z3_PRED as usize] = Some(ipred_z3_c_erased);
    (*c).intra_pred[FILTER_PRED as usize] = Some(ipred_filter_c_erased);

    (*c).cfl_ac[Rav1dPixelLayout::I420 as usize - 1] = cfl_ac_420_c_erased;
    (*c).cfl_ac[Rav1dPixelLayout::I422 as usize - 1] = cfl_ac_422_c_erased;
    (*c).cfl_ac[Rav1dPixelLayout::I444 as usize - 1] = cfl_ac_444_c_erased;
    (*c).cfl_pred[DC_PRED as usize] = ipred_cfl_c_erased;
    (*c).cfl_pred[DC_128_PRED as usize] = ipred_cfl_128_c_erased;
    (*c).cfl_pred[TOP_DC_PRED as usize] = ipred_cfl_top_c_erased;
    (*c).cfl_pred[LEFT_DC_PRED as usize] = ipred_cfl_left_c_erased;

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
