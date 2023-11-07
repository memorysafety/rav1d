use crate::include::common::attributes::ctz;
use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::bitdepth::BPC;
use crate::include::common::intops::apply_sign;
use crate::include::common::intops::iclip;
use crate::src::tables::dav1d_sm_weights;
use cfg_if::cfg_if;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_ulong;
use std::ffi::c_ulonglong;
use std::slice;

pub type angular_ipred_fn = unsafe extern "C" fn(
    *mut DynPixel,
    ptrdiff_t,
    *const DynPixel,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
    c_int,
) -> ();

pub type cfl_ac_fn =
    unsafe extern "C" fn(*mut i16, *const DynPixel, ptrdiff_t, c_int, c_int, c_int, c_int) -> ();

pub type cfl_pred_fn = unsafe extern "C" fn(
    *mut DynPixel,
    ptrdiff_t,
    *const DynPixel,
    c_int,
    c_int,
    *const i16,
    c_int,
    c_int,
) -> ();

pub type pal_pred_fn =
    unsafe extern "C" fn(*mut DynPixel, ptrdiff_t, *const u16, *const u8, c_int, c_int) -> ();

#[repr(C)]
pub struct Rav1dIntraPredDSPContext {
    // TODO(legare): Remove `Option` once `dav1d_submit_frame` is no longer checking
    // this field with `is_none`.
    pub intra_pred: [Option<angular_ipred_fn>; 14],
    pub cfl_ac: [cfl_ac_fn; 3],
    pub cfl_pred: [cfl_pred_fn; 6],
    pub pal_pred: pal_pred_fn,
}

// TODO(legare): Generated fns are temporarily pub until init fns are deduplicated.
#[cfg(feature = "asm")]
macro_rules! decl_fn {
    (angular_ipred, $name:ident) => {
        pub(crate) fn $name(
            dst: *mut DynPixel,
            stride: ptrdiff_t,
            topleft: *const DynPixel,
            width: c_int,
            height: c_int,
            angle: c_int,
            max_width: c_int,
            max_height: c_int,
            bitdepth_max: c_int,
        );
    };

    (cfl_pred, $name:ident) => {
        pub(crate) fn $name(
            dst: *mut DynPixel,
            stride: ptrdiff_t,
            topleft: *const DynPixel,
            width: c_int,
            height: c_int,
            ac: *const i16,
            alpha: c_int,
            bitdepth_max: c_int,
        );
    };

    (cfl_ac, $name:ident) => {
        pub(crate) fn $name(
            ac: *mut i16,
            y: *const DynPixel,
            stride: ptrdiff_t,
            w_pad: c_int,
            h_pad: c_int,
            cw: c_int,
            ch: c_int,
        );
    };

    (pal_pred, $name:ident) => {
        pub(crate) fn $name(
            dst: *mut DynPixel,
            stride: ptrdiff_t,
            pal: *const u16,
            idx: *const u8,
            w: c_int,
            h: c_int,
        );
    };
}

#[cfg(feature = "asm")]
macro_rules! decl_fns {
    ($fn_kind:ident, $name:ident) => {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        decl_fns!($fn_kind, $name, ssse3);

        #[cfg(target_arch = "x86_64")]
        decl_fns!($fn_kind, $name, avx2);

        #[cfg(target_arch = "x86_64")]
        decl_fns!($fn_kind, $name, avx512icl);

        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
        decl_fns!($fn_kind, $name, neon);
    };

    ($fn_kind:ident, $name:ident, $asm:ident) => {
        paste::paste! {
            #[cfg(feature = "bitdepth_8")]
            decl_fn!($fn_kind, [<dav1d_ $name _8bpc_ $asm>]);
            #[cfg(feature = "bitdepth_16")]
            decl_fn!($fn_kind, [<dav1d_ $name _16bpc_ $asm>]);
        }
    };
}

#[cfg(feature = "asm")]
#[allow(dead_code)] // Macro declares more fns than actually exist.
extern "C" {
    decl_fns!(angular_ipred, ipred_dc);
    decl_fns!(angular_ipred, ipred_dc_128);
    decl_fns!(angular_ipred, ipred_dc_top);
    decl_fns!(angular_ipred, ipred_dc_left);
    decl_fns!(angular_ipred, ipred_h);
    decl_fns!(angular_ipred, ipred_v);
    decl_fns!(angular_ipred, ipred_paeth);
    decl_fns!(angular_ipred, ipred_smooth);
    decl_fns!(angular_ipred, ipred_smooth_h);
    decl_fns!(angular_ipred, ipred_smooth_v);
    decl_fns!(angular_ipred, ipred_z1);
    decl_fns!(angular_ipred, ipred_z2);
    decl_fns!(angular_ipred, ipred_z3);
    decl_fns!(angular_ipred, ipred_filter);

    decl_fns!(cfl_pred, ipred_cfl);
    decl_fns!(cfl_pred, ipred_cfl_128);
    decl_fns!(cfl_pred, ipred_cfl_top);
    decl_fns!(cfl_pred, ipred_cfl_left);

    decl_fns!(cfl_ac, ipred_cfl_ac_420);
    decl_fns!(cfl_ac, ipred_cfl_ac_422);
    decl_fns!(cfl_ac, ipred_cfl_ac_444);

    decl_fns!(pal_pred, pal_pred);
}

#[inline(never)]
unsafe fn splat_dc<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    width: c_int,
    height: c_int,
    dc: c_int,
    bd: BD,
) {
    match BD::BPC {
        BPC::BPC8 => {
            if !(dc <= 0xff as c_int) {
                unreachable!();
            }
            if width > 4 {
                let dcN: u64 =
                    (dc as c_ulonglong).wrapping_mul(0x101010101010101 as c_ulonglong) as u64;
                let mut y = 0;
                while y < height {
                    let mut x = 0;
                    while x < width {
                        *(&mut *dst.offset(x as isize) as *mut BD::Pixel as *mut u64) = dcN;
                        x = (x as c_ulong).wrapping_add(::core::mem::size_of::<u64>() as c_ulong)
                            as c_int as c_int;
                    }
                    dst = dst.offset(stride as isize);
                    y += 1;
                }
            } else {
                let dcN_0: c_uint = (dc as c_uint).wrapping_mul(0x1010101 as c_uint);
                let mut y_0 = 0;
                while y_0 < height {
                    let mut x_0 = 0;
                    while x_0 < width {
                        *(&mut *dst.offset(x_0 as isize) as *mut BD::Pixel as *mut c_uint) = dcN_0;
                        x_0 = (x_0 as c_ulong)
                            .wrapping_add(::core::mem::size_of::<c_uint>() as c_ulong)
                            as c_int as c_int;
                    }
                    dst = dst.offset(stride as isize);
                    y_0 += 1;
                }
            };
        }
        BPC::BPC16 => {
            if !(dc <= bd.bitdepth_max().as_::<c_int>()) {
                unreachable!();
            }
            let dcN: u64 = (dc as c_ulonglong).wrapping_mul(0x1000100010001 as c_ulonglong) as u64;
            let mut y = 0;
            while y < height {
                let mut x = 0;
                while x < width {
                    *(&mut *dst.offset(x as isize) as *mut BD::Pixel as *mut u64) = dcN;
                    x = (x as c_ulong).wrapping_add(::core::mem::size_of::<u64>() as c_ulong >> 1)
                        as c_int as c_int;
                }
                dst = dst.offset(BD::pxstride(stride as usize) as isize);
                y += 1;
            }
        }
    }
}

#[inline(never)]
unsafe fn cfl_pred<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    width: c_int,
    height: c_int,
    dc: c_int,
    mut ac: *const i16,
    alpha: c_int,
    bd: BD,
) {
    let mut y = 0;
    while y < height {
        let mut x = 0;
        while x < width {
            let diff = alpha * *ac.offset(x as isize) as c_int;
            *dst.offset(x as isize) = bd.iclip_pixel(dc + apply_sign(diff.abs() + 32 >> 6, diff));
            x += 1;
        }
        ac = ac.offset(width as isize);
        dst = dst.offset(BD::pxstride(stride as usize) as isize);
        y += 1;
    }
}

unsafe fn dc_gen_top<BD: BitDepth>(topleft: *const BD::Pixel, width: c_int) -> c_uint {
    let mut dc: c_uint = (width >> 1) as c_uint;
    let mut i = 0;
    while i < width {
        dc = dc.wrapping_add((*topleft.offset((1 + i) as isize)).as_::<c_uint>());
        i += 1;
    }
    return dc >> ctz(width as c_uint);
}

// TODO(kkysen) Temporarily pub until mod is deduplicated
pub(crate) unsafe extern "C" fn ipred_dc_top_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    bitdepth_max: c_int,
) {
    splat_dc::<BD>(
        dst.cast(),
        stride,
        width,
        height,
        dc_gen_top::<BD>(topleft.cast(), width) as c_int,
        BD::from_c(bitdepth_max),
    );
}

// TODO(kkysen) Temporarily pub until mod is deduplicated
pub(crate) unsafe extern "C" fn ipred_cfl_top_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    ac: *const i16,
    alpha: c_int,
    bitdepth_max: c_int,
) {
    cfl_pred::<BD>(
        dst.cast(),
        stride,
        width,
        height,
        dc_gen_top::<BD>(topleft.cast(), width) as c_int,
        ac,
        alpha,
        BD::from_c(bitdepth_max),
    );
}

unsafe fn dc_gen_left<BD: BitDepth>(topleft: *const BD::Pixel, height: c_int) -> c_uint {
    let mut dc: c_uint = (height >> 1) as c_uint;
    let mut i = 0;
    while i < height {
        dc = dc.wrapping_add((*topleft.offset(-(1 + i) as isize)).as_::<c_uint>());
        i += 1;
    }
    return dc >> ctz(height as c_uint);
}

// TODO(kkysen) Temporarily pub until mod is deduplicated
pub(crate) unsafe extern "C" fn ipred_dc_left_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    bitdepth_max: c_int,
) {
    splat_dc::<BD>(
        dst.cast(),
        stride,
        width,
        height,
        dc_gen_left::<BD>(topleft.cast(), height) as c_int,
        BD::from_c(bitdepth_max),
    );
}

// TODO(kkysen) Temporarily pub until mod is deduplicated
pub(crate) unsafe extern "C" fn ipred_cfl_left_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    ac: *const i16,
    alpha: c_int,
    bitdepth_max: c_int,
) {
    let dc: c_uint = dc_gen_left::<BD>(topleft.cast(), height);
    cfl_pred::<BD>(
        dst.cast(),
        stride,
        width,
        height,
        dc as c_int,
        ac,
        alpha,
        BD::from_c(bitdepth_max),
    );
}

unsafe fn dc_gen<BD: BitDepth>(topleft: *const BD::Pixel, width: c_int, height: c_int) -> c_uint {
    let (multiplier_1x2, multiplier_1x4, base_shift) = match BD::BPC {
        BPC::BPC8 => (0x5556, 0x3334, 16),
        BPC::BPC16 => (0xAAAB, 0x6667, 17),
    };

    let mut dc: c_uint = (width + height >> 1) as c_uint;
    let mut i = 0;
    while i < width {
        dc = dc.wrapping_add((*topleft.offset((i + 1) as isize)).as_::<c_uint>());
        i += 1;
    }
    let mut i_0 = 0;
    while i_0 < height {
        dc = dc.wrapping_add((*topleft.offset(-(i_0 + 1) as isize)).as_::<c_uint>());
        i_0 += 1;
    }
    dc >>= ctz((width + height) as c_uint);
    if width != height {
        dc = dc.wrapping_mul(if width > height * 2 || height > width * 2 {
            multiplier_1x4
        } else {
            multiplier_1x2
        });
        dc >>= base_shift;
    }
    return dc;
}

// TODO(kkysen) Temporarily pub until mod is deduplicated
pub(crate) unsafe extern "C" fn ipred_dc_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    bitdepth_max: c_int,
) {
    splat_dc::<BD>(
        dst.cast(),
        stride,
        width,
        height,
        dc_gen::<BD>(topleft.cast(), width, height) as c_int,
        BD::from_c(bitdepth_max),
    );
}

// TODO(kkysen) Temporarily pub until mod is deduplicated
pub(crate) unsafe extern "C" fn ipred_cfl_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    ac: *const i16,
    alpha: c_int,
    bitdepth_max: c_int,
) {
    let dc: c_uint = dc_gen::<BD>(topleft.cast(), width, height);
    cfl_pred::<BD>(
        dst.cast(),
        stride,
        width,
        height,
        dc as c_int,
        ac,
        alpha,
        BD::from_c(bitdepth_max),
    );
}

// TODO(kkysen) Temporarily pub until mod is deduplicated
pub(crate) unsafe extern "C" fn ipred_dc_128_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    _topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    bitdepth_max: c_int,
) {
    let bd = BD::from_c(bitdepth_max);
    let dc = bd.bitdepth_max().as_::<c_int>() + 1 >> 1;
    splat_dc::<BD>(dst.cast(), stride, width, height, dc, bd);
}

// TODO(kkysen) Temporarily pub until mod is deduplicated
pub(crate) unsafe extern "C" fn ipred_cfl_128_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    _topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    ac: *const i16,
    alpha: c_int,
    bitdepth_max: c_int,
) {
    let bd = BD::from_c(bitdepth_max);
    let dc = bd.bitdepth_max().as_::<c_int>() + 1 >> 1;
    cfl_pred::<BD>(dst.cast(), stride, width, height, dc, ac, alpha, bd);
}

unsafe fn ipred_v_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    topleft: *const BD::Pixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bd: BD,
) {
    let width = width.try_into().unwrap();

    let mut y = 0;
    while y < height {
        BD::pixel_copy(
            slice::from_raw_parts_mut(dst, width),
            &slice::from_raw_parts(topleft, width + 1)[1..],
            width,
        );
        dst = dst.offset(BD::pxstride(stride as usize) as isize);
        y += 1;
    }
}

// TODO(kkysen) Temporarily pub until mod is deduplicated
pub(crate) unsafe extern "C" fn ipred_v_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    a: c_int,
    max_width: c_int,
    max_height: c_int,
    bitdepth_max: c_int,
) {
    ipred_v_rust::<BD>(
        dst.cast(),
        stride,
        topleft.cast(),
        width,
        height,
        a,
        max_width,
        max_height,
        BD::from_c(bitdepth_max),
    );
}

unsafe fn ipred_h_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    topleft: *const BD::Pixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bd: BD,
) {
    let width = width.try_into().unwrap();

    let mut y = 0;
    while y < height {
        BD::pixel_set(
            slice::from_raw_parts_mut(dst, width),
            *topleft.offset(-(1 + y) as isize),
            width,
        );
        dst = dst.offset(BD::pxstride(stride as usize) as isize);
        y += 1;
    }
}

// TODO(kkysen) Temporarily pub until mod is deduplicated
pub(crate) unsafe extern "C" fn ipred_h_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    a: c_int,
    max_width: c_int,
    max_height: c_int,
    bitdepth_max: c_int,
) {
    ipred_h_rust::<BD>(
        dst.cast(),
        stride,
        topleft.cast(),
        width,
        height,
        a,
        max_width,
        max_height,
        BD::from_c(bitdepth_max),
    );
}

unsafe fn ipred_paeth_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    tl_ptr: *const BD::Pixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bd: BD,
) {
    let topleft = (*tl_ptr.offset(0)).as_::<c_int>();
    let mut y = 0;
    while y < height {
        let left = (*tl_ptr.offset(-(y + 1) as isize)).as_::<c_int>();
        let mut x = 0;
        while x < width {
            let top = (*tl_ptr.offset((1 + x) as isize)).as_::<c_int>();
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
            })
            .as_::<BD::Pixel>();
            x += 1;
        }
        dst = dst.offset(BD::pxstride(stride as usize) as isize);
        y += 1;
    }
}

// TODO(kkysen) Temporarily pub until mod is deduplicated
pub(crate) unsafe extern "C" fn ipred_paeth_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    tl_ptr: *const DynPixel,
    width: c_int,
    height: c_int,
    a: c_int,
    max_width: c_int,
    max_height: c_int,
    bitdepth_max: c_int,
) {
    ipred_paeth_rust::<BD>(
        dst.cast(),
        stride,
        tl_ptr.cast(),
        width,
        height,
        a,
        max_width,
        max_height,
        BD::from_c(bitdepth_max),
    );
}

unsafe fn ipred_smooth_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    topleft: *const BD::Pixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bd: BD,
) {
    let weights_hor: *const u8 = &*dav1d_sm_weights.0.as_ptr().offset(width as isize) as *const u8;
    let weights_ver: *const u8 = &*dav1d_sm_weights.0.as_ptr().offset(height as isize) as *const u8;
    let right = (*topleft.offset(width as isize)).as_::<c_int>();
    let bottom = (*topleft.offset(-height as isize)).as_::<c_int>();
    let mut y = 0;
    while y < height {
        let mut x = 0;
        while x < width {
            let pred = *weights_ver.offset(y as isize) as c_int
                * (*topleft.offset((1 + x) as isize)).as_::<c_int>()
                + (256 - *weights_ver.offset(y as isize) as c_int) * bottom
                + *weights_hor.offset(x as isize) as c_int
                    * (*topleft.offset(-(1 + y) as isize)).as_::<c_int>()
                + (256 - *weights_hor.offset(x as isize) as c_int) * right;
            *dst.offset(x as isize) = (pred + 256 >> 9).as_::<BD::Pixel>();
            x += 1;
        }
        dst = dst.offset(BD::pxstride(stride as usize) as isize);
        y += 1;
    }
}

// TODO(kkysen) Temporarily pub until mod is deduplicated
pub(crate) unsafe extern "C" fn ipred_smooth_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    a: c_int,
    max_width: c_int,
    max_height: c_int,
    bitdepth_max: c_int,
) {
    ipred_smooth_rust::<BD>(
        dst.cast(),
        stride,
        topleft.cast(),
        width,
        height,
        a,
        max_width,
        max_height,
        BD::from_c(bitdepth_max),
    );
}

unsafe fn ipred_smooth_v_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    topleft: *const BD::Pixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bd: BD,
) {
    let weights_ver: *const u8 = &*dav1d_sm_weights.0.as_ptr().offset(height as isize) as *const u8;
    let bottom = (*topleft.offset(-height as isize)).as_::<c_int>();
    let mut y = 0;
    while y < height {
        let mut x = 0;
        while x < width {
            let pred = *weights_ver.offset(y as isize) as c_int
                * (*topleft.offset((1 + x) as isize)).as_::<c_int>()
                + (256 - *weights_ver.offset(y as isize) as c_int) * bottom;
            *dst.offset(x as isize) = (pred + 128 >> 8).as_::<BD::Pixel>();
            x += 1;
        }
        dst = dst.offset(BD::pxstride(stride as usize) as isize);
        y += 1;
    }
}

// TODO(kkysen) Temporarily pub until mod is deduplicated
pub(crate) unsafe extern "C" fn ipred_smooth_v_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    a: c_int,
    max_width: c_int,
    max_height: c_int,
    bitdepth_max: c_int,
) {
    ipred_smooth_v_rust::<BD>(
        dst.cast(),
        stride,
        topleft.cast(),
        width,
        height,
        a,
        max_width,
        max_height,
        BD::from_c(bitdepth_max),
    );
}

unsafe fn ipred_smooth_h_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    topleft: *const BD::Pixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bd: BD,
) {
    let weights_hor: *const u8 = &*dav1d_sm_weights.0.as_ptr().offset(width as isize) as *const u8;
    let right = (*topleft.offset(width as isize)).as_::<c_int>();
    let mut y = 0;
    while y < height {
        let mut x = 0;
        while x < width {
            let pred = *weights_hor.offset(x as isize) as c_int
                * (*topleft.offset(-(y + 1) as isize)).as_::<c_int>()
                + (256 - *weights_hor.offset(x as isize) as c_int) * right;
            *dst.offset(x as isize) = (pred + 128 >> 8).as_::<BD::Pixel>();
            x += 1;
        }
        dst = dst.offset(BD::pxstride(stride as usize) as isize);
        y += 1;
    }
}

// TODO(kkysen) Temporarily pub until mod is deduplicated
pub(crate) unsafe extern "C" fn ipred_smooth_h_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    a: c_int,
    max_width: c_int,
    max_height: c_int,
    bitdepth_max: c_int,
) {
    ipred_smooth_h_rust::<BD>(
        dst.cast(),
        stride,
        topleft.cast(),
        width,
        height,
        a,
        max_width,
        max_height,
        BD::from_c(bitdepth_max),
    );
}

// TODO(kkysen) Temporarily pub until mod is deduplicated
#[inline(never)]
pub(crate) unsafe fn get_filter_strength(wh: c_int, angle: c_int, is_sm: c_int) -> c_int {
    if is_sm != 0 {
        if wh <= 8 {
            if angle >= 64 {
                return 2 as c_int;
            }
            if angle >= 40 {
                return 1 as c_int;
            }
        } else if wh <= 16 {
            if angle >= 48 {
                return 2 as c_int;
            }
            if angle >= 20 {
                return 1 as c_int;
            }
        } else if wh <= 24 {
            if angle >= 4 {
                return 3 as c_int;
            }
        } else {
            return 3 as c_int;
        }
    } else if wh <= 8 {
        if angle >= 56 {
            return 1 as c_int;
        }
    } else if wh <= 16 {
        if angle >= 40 {
            return 1 as c_int;
        }
    } else if wh <= 24 {
        if angle >= 32 {
            return 3 as c_int;
        }
        if angle >= 16 {
            return 2 as c_int;
        }
        if angle >= 8 {
            return 1 as c_int;
        }
    } else if wh <= 32 {
        if angle >= 32 {
            return 3 as c_int;
        }
        if angle >= 4 {
            return 2 as c_int;
        }
        return 1 as c_int;
    } else {
        return 3 as c_int;
    }
    return 0 as c_int;
}

// TODO(kkysen) Temporarily pub until mod is deduplicated
#[inline(never)]
pub(crate) unsafe fn filter_edge<BD: BitDepth>(
    out: *mut BD::Pixel,
    sz: c_int,
    lim_from: c_int,
    lim_to: c_int,
    in_0: *const BD::Pixel,
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
            s += (*in_0.offset(iclip(i - 2 + j, from, to - 1) as isize)).as_::<c_int>()
                * kernel[(strength - 1) as usize][j as usize] as c_int;
            j += 1;
        }
        *out.offset(i as isize) = (s + 8 >> 4).as_::<BD::Pixel>();
        i += 1;
    }
    while i < sz {
        *out.offset(i as isize) = *in_0.offset(iclip(i, from, to - 1) as isize);
        i += 1;
    }
}

// TODO(kkysen) Temporarily pub until mod is deduplicated
#[inline]
pub(crate) unsafe fn get_upsample(wh: c_int, angle: c_int, is_sm: c_int) -> c_int {
    return (angle < 40 && wh <= 16 >> is_sm) as c_int;
}

// TODO(kkysen) Temporarily pub until mod is deduplicated
pub(crate) unsafe fn filter_fn(
    flt_ptr: *const i8,
    p0: c_int,
    p1: c_int,
    p2: c_int,
    p3: c_int,
    p4: c_int,
    p5: c_int,
    p6: c_int,
) -> c_int {
    if cfg!(any(target_arch = "x86", target_arch = "x86_64")) {
        *flt_ptr.offset(0) as c_int * p0
            + *flt_ptr.offset(1) as c_int * p1
            + *flt_ptr.offset(16) as c_int * p2
            + *flt_ptr.offset(17) as c_int * p3
            + *flt_ptr.offset(32) as c_int * p4
            + *flt_ptr.offset(33) as c_int * p5
            + *flt_ptr.offset(48) as c_int * p6
    } else {
        *flt_ptr.offset(0) as c_int * p0
            + *flt_ptr.offset(8) as c_int * p1
            + *flt_ptr.offset(16) as c_int * p2
            + *flt_ptr.offset(24) as c_int * p3
            + *flt_ptr.offset(32) as c_int * p4
            + *flt_ptr.offset(40) as c_int * p5
            + *flt_ptr.offset(48) as c_int * p6
    }
}

cfg_if! {
    if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        // TODO(kkysen) Temporarily pub until mod is deduplicated
        pub(crate) const FLT_INCR: isize = 2;
    } else {
        // TODO(kkysen) Temporarily pub until mod is deduplicated
        pub(crate) const FLT_INCR: isize = 1;
    }
}
