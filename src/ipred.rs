use crate::include::common::attributes::ctz;
use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::bitdepth::BPC;
use crate::include::common::intops::apply_sign;
use crate::include::common::intops::iclip;
use crate::src::tables::dav1d_dr_intra_derivative;
use crate::src::tables::dav1d_filter_intra_taps;
use crate::src::tables::dav1d_sm_weights;
use cfg_if::cfg_if;
use libc::memcpy;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_ulong;
use std::ffi::c_ulonglong;
use std::ffi::c_void;
use std::slice;

#[cfg(feature = "bitdepth_8")]
use crate::include::common::bitdepth::BitDepth8;

#[cfg(feature = "bitdepth_16")]
use crate::include::common::bitdepth::BitDepth16;

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

// TODO(kkysen) Temporarily pub until mod is deduplicated
#[cfg(all(feature = "bitdepth_8", feature = "asm", target_arch = "aarch64"))]
extern "C" {
    pub(crate) fn dav1d_ipred_z1_fill2_8bpc_neon(
        dst: *mut <BitDepth8 as BitDepth>::Pixel,
        stride: ptrdiff_t,
        top: *const <BitDepth8 as BitDepth>::Pixel,
        width: c_int,
        height: c_int,
        dx: c_int,
        max_base_x: c_int,
    );
    pub(crate) fn dav1d_ipred_z1_fill1_8bpc_neon(
        dst: *mut <BitDepth8 as BitDepth>::Pixel,
        stride: ptrdiff_t,
        top: *const <BitDepth8 as BitDepth>::Pixel,
        width: c_int,
        height: c_int,
        dx: c_int,
        max_base_x: c_int,
    );
    pub(crate) fn dav1d_ipred_z1_upsample_edge_8bpc_neon(
        out: *mut <BitDepth8 as BitDepth>::Pixel,
        hsz: c_int,
        in_0: *const <BitDepth8 as BitDepth>::Pixel,
        end: c_int,
    );
    pub(crate) fn dav1d_ipred_z1_filter_edge_8bpc_neon(
        out: *mut <BitDepth8 as BitDepth>::Pixel,
        sz: c_int,
        in_0: *const <BitDepth8 as BitDepth>::Pixel,
        end: c_int,
        strength: c_int,
    );
    pub(crate) fn dav1d_ipred_z2_fill3_8bpc_neon(
        dst: *mut <BitDepth8 as BitDepth>::Pixel,
        stride: ptrdiff_t,
        top: *const <BitDepth8 as BitDepth>::Pixel,
        left: *const <BitDepth8 as BitDepth>::Pixel,
        width: c_int,
        height: c_int,
        dx: c_int,
        dy: c_int,
    );
    pub(crate) fn dav1d_ipred_z2_fill2_8bpc_neon(
        dst: *mut <BitDepth8 as BitDepth>::Pixel,
        stride: ptrdiff_t,
        top: *const <BitDepth8 as BitDepth>::Pixel,
        left: *const <BitDepth8 as BitDepth>::Pixel,
        width: c_int,
        height: c_int,
        dx: c_int,
        dy: c_int,
    );
    pub(crate) fn dav1d_ipred_z2_fill1_8bpc_neon(
        dst: *mut <BitDepth8 as BitDepth>::Pixel,
        stride: ptrdiff_t,
        top: *const <BitDepth8 as BitDepth>::Pixel,
        left: *const <BitDepth8 as BitDepth>::Pixel,
        width: c_int,
        height: c_int,
        dx: c_int,
        dy: c_int,
    );
    pub(crate) fn dav1d_ipred_z2_upsample_edge_8bpc_neon(
        out: *mut <BitDepth8 as BitDepth>::Pixel,
        hsz: c_int,
        in_0: *const <BitDepth8 as BitDepth>::Pixel,
    );
    pub(crate) fn dav1d_ipred_reverse_8bpc_neon(
        dst: *mut <BitDepth8 as BitDepth>::Pixel,
        src: *const <BitDepth8 as BitDepth>::Pixel,
        n: c_int,
    );
    pub(crate) fn dav1d_ipred_z3_fill2_8bpc_neon(
        dst: *mut <BitDepth8 as BitDepth>::Pixel,
        stride: ptrdiff_t,
        left: *const <BitDepth8 as BitDepth>::Pixel,
        width: c_int,
        height: c_int,
        dy: c_int,
        max_base_y: c_int,
    );
    pub(crate) fn dav1d_ipred_z3_fill1_8bpc_neon(
        dst: *mut <BitDepth8 as BitDepth>::Pixel,
        stride: ptrdiff_t,
        left: *const <BitDepth8 as BitDepth>::Pixel,
        width: c_int,
        height: c_int,
        dy: c_int,
        max_base_y: c_int,
    );
    pub(crate) fn dav1d_ipred_pixel_set_8bpc_neon(
        out: *mut <BitDepth8 as BitDepth>::Pixel,
        px: <BitDepth8 as BitDepth>::Pixel,
        n: c_int,
    );
}

// TODO(kkysen) Temporarily pub until mod is deduplicated
#[cfg(all(feature = "bitdepth_16", feature = "asm", target_arch = "aarch64"))]
extern "C" {
    pub(crate) fn dav1d_ipred_z1_fill2_16bpc_neon(
        dst: *mut <BitDepth16 as BitDepth>::Pixel,
        stride: ptrdiff_t,
        top: *const <BitDepth16 as BitDepth>::Pixel,
        width: c_int,
        height: c_int,
        dx: c_int,
        max_base_x: c_int,
    );
    pub(crate) fn dav1d_ipred_z1_fill1_16bpc_neon(
        dst: *mut <BitDepth16 as BitDepth>::Pixel,
        stride: ptrdiff_t,
        top: *const <BitDepth16 as BitDepth>::Pixel,
        width: c_int,
        height: c_int,
        dx: c_int,
        max_base_x: c_int,
    );
    pub(crate) fn dav1d_ipred_z1_upsample_edge_16bpc_neon(
        out: *mut <BitDepth16 as BitDepth>::Pixel,
        hsz: c_int,
        in_0: *const <BitDepth16 as BitDepth>::Pixel,
        end: c_int,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_ipred_z1_filter_edge_16bpc_neon(
        out: *mut <BitDepth16 as BitDepth>::Pixel,
        sz: c_int,
        in_0: *const <BitDepth16 as BitDepth>::Pixel,
        end: c_int,
        strength: c_int,
    );
    pub(crate) fn dav1d_ipred_z2_fill3_16bpc_neon(
        dst: *mut <BitDepth16 as BitDepth>::Pixel,
        stride: ptrdiff_t,
        top: *const <BitDepth16 as BitDepth>::Pixel,
        left: *const <BitDepth16 as BitDepth>::Pixel,
        width: c_int,
        height: c_int,
        dx: c_int,
        dy: c_int,
    );
    pub(crate) fn dav1d_ipred_z2_fill2_16bpc_neon(
        dst: *mut <BitDepth16 as BitDepth>::Pixel,
        stride: ptrdiff_t,
        top: *const <BitDepth16 as BitDepth>::Pixel,
        left: *const <BitDepth16 as BitDepth>::Pixel,
        width: c_int,
        height: c_int,
        dx: c_int,
        dy: c_int,
    );
    pub(crate) fn dav1d_ipred_z2_fill1_16bpc_neon(
        dst: *mut <BitDepth16 as BitDepth>::Pixel,
        stride: ptrdiff_t,
        top: *const <BitDepth16 as BitDepth>::Pixel,
        left: *const <BitDepth16 as BitDepth>::Pixel,
        width: c_int,
        height: c_int,
        dx: c_int,
        dy: c_int,
    );
    pub(crate) fn dav1d_ipred_z2_upsample_edge_16bpc_neon(
        out: *mut <BitDepth16 as BitDepth>::Pixel,
        hsz: c_int,
        in_0: *const <BitDepth16 as BitDepth>::Pixel,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_ipred_reverse_16bpc_neon(
        dst: *mut <BitDepth16 as BitDepth>::Pixel,
        src: *const <BitDepth16 as BitDepth>::Pixel,
        n: c_int,
    );
    pub(crate) fn dav1d_ipred_z3_fill2_16bpc_neon(
        dst: *mut <BitDepth16 as BitDepth>::Pixel,
        stride: ptrdiff_t,
        left: *const <BitDepth16 as BitDepth>::Pixel,
        width: c_int,
        height: c_int,
        dy: c_int,
        max_base_y: c_int,
    );
    pub(crate) fn dav1d_ipred_z3_fill1_16bpc_neon(
        dst: *mut <BitDepth16 as BitDepth>::Pixel,
        stride: ptrdiff_t,
        left: *const <BitDepth16 as BitDepth>::Pixel,
        width: c_int,
        height: c_int,
        dy: c_int,
        max_base_y: c_int,
    );
    pub(crate) fn dav1d_ipred_pixel_set_16bpc_neon(
        out: *mut <BitDepth16 as BitDepth>::Pixel,
        px: <BitDepth16 as BitDepth>::Pixel,
        n: c_int,
    );
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

#[inline(never)]
unsafe fn filter_edge<BD: BitDepth>(
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

#[inline(never)]
unsafe fn upsample_edge<BD: BitDepth>(
    out: *mut BD::Pixel,
    hsz: c_int,
    in_0: *const BD::Pixel,
    from: c_int,
    to: c_int,
    bd: BD,
) {
    static kernel: [i8; 4] = [-1, 9, 9, -1];
    let mut i;
    i = 0 as c_int;
    while i < hsz - 1 {
        *out.offset((i * 2) as isize) = *in_0.offset(iclip(i, from, to - 1) as isize);
        let mut s = 0;
        let mut j = 0;
        while j < 4 {
            s += (*in_0.offset(iclip(i + j - 1, from, to - 1) as isize)).as_::<c_int>()
                * kernel[j as usize] as c_int;
            j += 1;
        }
        *out.offset((i * 2 + 1) as isize) =
            iclip(s + 8 >> 4, 0 as c_int, bd.bitdepth_max().as_::<c_int>()).as_::<BD::Pixel>();
        i += 1;
    }
    *out.offset((i * 2) as isize) = *in_0.offset(iclip(i, from, to - 1) as isize);
}

// TODO(kkysen) Temporarily pub until mod is deduplicated
pub(crate) unsafe fn ipred_z1_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    topleft_in: *const BD::Pixel,
    width: c_int,
    height: c_int,
    mut angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
    bd: BD,
) {
    let is_sm = angle >> 9 & 0x1 as c_int;
    let enable_intra_edge_filter = angle >> 10;
    angle &= 511 as c_int;
    if !(angle < 90) {
        unreachable!();
    }
    let mut dx = dav1d_dr_intra_derivative[(angle >> 1) as usize] as c_int;
    let mut top_out: [BD::Pixel; 128] = [0.into(); 128];
    let top: *const BD::Pixel;
    let max_base_x;
    let upsample_above = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, 90 - angle, is_sm)
    } else {
        0 as c_int
    };
    if upsample_above != 0 {
        upsample_edge::<BD>(
            top_out.as_mut_ptr(),
            width + height,
            &*topleft_in.offset(1),
            -(1 as c_int),
            width + cmp::min(width, height),
            bd,
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
            filter_edge::<BD>(
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
            top = &*topleft_in.offset(1) as *const BD::Pixel;
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
                let v = (*top.offset(base as isize)).as_::<c_int>() * (64 - frac)
                    + (*top.offset((base + 1) as isize)).as_::<c_int>() * frac;
                *dst.offset(x as isize) = (v + 32 >> 6).as_::<BD::Pixel>();
                x += 1;
                base += base_inc;
            } else {
                let width = width.try_into().unwrap();
                let x = x as usize;
                BD::pixel_set(
                    &mut slice::from_raw_parts_mut(dst, width)[x..],
                    *top.offset(max_base_x as isize),
                    width - x,
                );
                break;
            }
        }
        y += 1;
        dst = dst.offset(BD::pxstride(stride as usize) as isize);
        xpos += dx;
    }
}

// TODO(kkysen) Temporarily pub until mod is deduplicated
pub(crate) unsafe fn ipred_z2_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    topleft_in: *const BD::Pixel,
    width: c_int,
    height: c_int,
    mut angle: c_int,
    max_width: c_int,
    max_height: c_int,
    bd: BD,
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
    let mut edge: [BD::Pixel; 129] = [0.into(); 129];
    let topleft: *mut BD::Pixel = &mut *edge.as_mut_ptr().offset(64) as *mut BD::Pixel;
    if upsample_above != 0 {
        upsample_edge::<BD>(topleft, width + 1, topleft_in, 0 as c_int, width + 1, bd);
        dx <<= 1;
    } else {
        let filter_strength = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, angle - 90, is_sm)
        } else {
            0 as c_int
        };
        if filter_strength != 0 {
            filter_edge::<BD>(
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
            let width = width.try_into().unwrap();
            BD::pixel_copy(
                &mut slice::from_raw_parts_mut(topleft, width + 1)[1..],
                &slice::from_raw_parts(topleft_in, width + 1)[1..],
                width,
            );
        }
    }
    if upsample_left != 0 {
        upsample_edge::<BD>(
            &mut *topleft.offset((-height * 2) as isize),
            height + 1,
            &*topleft_in.offset(-height as isize),
            0 as c_int,
            height + 1,
            bd,
        );
        dy <<= 1;
    } else {
        let filter_strength_0 = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, 180 - angle, is_sm)
        } else {
            0 as c_int
        };
        if filter_strength_0 != 0 {
            filter_edge::<BD>(
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
            BD::pixel_copy(
                slice::from_raw_parts_mut(
                    topleft.offset(-height as isize),
                    height.try_into().unwrap(),
                ),
                slice::from_raw_parts(
                    topleft_in.offset(-height as isize),
                    height.try_into().unwrap(),
                ),
                height.try_into().unwrap(),
            );
        }
    }
    *topleft = *topleft_in;
    let base_inc_x = 1 + upsample_above;
    let left: *const BD::Pixel =
        &mut *topleft.offset(-(1 + upsample_left) as isize) as *mut BD::Pixel;
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
                v = (*topleft.offset(base_x as isize)).as_::<c_int>() * (64 - frac_x)
                    + (*topleft.offset((base_x + 1) as isize)).as_::<c_int>() * frac_x;
            } else {
                let base_y = ypos >> 6;
                if !(base_y >= -(1 + upsample_left)) {
                    unreachable!();
                }
                let frac_y = ypos & 0x3e as c_int;
                v = (*left.offset(-base_y as isize)).as_::<c_int>() * (64 - frac_y)
                    + (*left.offset(-(base_y + 1) as isize)).as_::<c_int>() * frac_y;
            }
            *dst.offset(x as isize) = (v + 32 >> 6).as_::<BD::Pixel>();
            x += 1;
            base_x += base_inc_x;
            ypos -= dy;
        }
        y += 1;
        xpos -= dx;
        dst = dst.offset(BD::pxstride(stride as usize) as isize);
    }
}

// TODO(kkysen) Temporarily pub until mod is deduplicated
pub(crate) unsafe fn ipred_z3_rust<BD: BitDepth>(
    dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    topleft_in: *const BD::Pixel,
    width: c_int,
    height: c_int,
    mut angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
    bd: BD,
) {
    let is_sm = angle >> 9 & 0x1 as c_int;
    let enable_intra_edge_filter = angle >> 10;
    angle &= 511 as c_int;
    if !(angle > 180) {
        unreachable!();
    }
    let mut dy = dav1d_dr_intra_derivative[(270 - angle >> 1) as usize] as c_int;
    let mut left_out: [BD::Pixel; 128] = [0.into(); 128];
    let left: *const BD::Pixel;
    let max_base_y;
    let upsample_left = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, angle - 180, is_sm)
    } else {
        0 as c_int
    };
    if upsample_left != 0 {
        upsample_edge::<BD>(
            left_out.as_mut_ptr(),
            width + height,
            &*topleft_in.offset(-(width + height) as isize),
            cmp::max(width - height, 0 as c_int),
            width + height + 1,
            bd,
        );
        left = &mut *left_out
            .as_mut_ptr()
            .offset((2 * (width + height) - 2) as isize) as *mut BD::Pixel;
        max_base_y = 2 * (width + height) - 2;
        dy <<= 1;
    } else {
        let filter_strength = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, angle - 180, is_sm)
        } else {
            0 as c_int
        };
        if filter_strength != 0 {
            filter_edge::<BD>(
                left_out.as_mut_ptr(),
                width + height,
                0 as c_int,
                width + height,
                &*topleft_in.offset(-(width + height) as isize),
                cmp::max(width - height, 0 as c_int),
                width + height + 1,
                filter_strength,
            );
            left =
                &mut *left_out.as_mut_ptr().offset((width + height - 1) as isize) as *mut BD::Pixel;
            max_base_y = width + height - 1;
        } else {
            left = &*topleft_in.offset(-(1 as c_int) as isize) as *const BD::Pixel;
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
                let v = (*left.offset(-base as isize)).as_::<c_int>() * (64 - frac)
                    + (*left.offset(-(base + 1) as isize)).as_::<c_int>() * frac;
                *dst.offset(
                    (y as isize * BD::pxstride(stride as usize) as isize + x as isize) as isize,
                ) = (v + 32 >> 6).as_::<BD::Pixel>();
                y += 1;
                base += base_inc;
            } else {
                loop {
                    *dst.offset(
                        (y as isize * BD::pxstride(stride as usize) as isize + x as isize) as isize,
                    ) = *left.offset(-max_base_y as isize);
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

// TODO(kkysen) Temporarily pub until mod is deduplicated
pub(crate) unsafe fn ipred_filter_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    topleft_in: *const BD::Pixel,
    width: c_int,
    height: c_int,
    mut filt_idx: c_int,
    _max_width: c_int,
    _max_height: c_int,
    bd: BD,
) {
    filt_idx &= 511 as c_int;
    if !(filt_idx < 5) {
        unreachable!();
    }
    let filter: *const i8 = (dav1d_filter_intra_taps[filt_idx as usize]).as_ptr();
    let mut top: *const BD::Pixel = &*topleft_in.offset(1) as *const BD::Pixel;
    let mut y = 0;
    while y < height {
        let mut topleft: *const BD::Pixel = &*topleft_in.offset(-y as isize) as *const BD::Pixel;
        let mut left: *const BD::Pixel =
            &*topleft.offset(-(1 as c_int) as isize) as *const BD::Pixel;
        let mut left_stride: ptrdiff_t = -(1 as c_int) as ptrdiff_t;
        let mut x = 0;
        while x < width {
            let p0 = (*topleft).as_::<c_int>();
            let p1 = (*top.offset(0)).as_::<c_int>();
            let p2 = (*top.offset(1)).as_::<c_int>();
            let p3 = (*top.offset(2)).as_::<c_int>();
            let p4 = (*top.offset(3)).as_::<c_int>();
            let p5 = (*left.offset((0 * left_stride) as isize)).as_::<c_int>();
            let p6 = (*left.offset((1 * left_stride) as isize)).as_::<c_int>();
            let mut ptr: *mut BD::Pixel = &mut *dst.offset(x as isize) as *mut BD::Pixel;
            let mut flt_ptr: *const i8 = filter;
            let mut yy = 0;
            while yy < 2 {
                let mut xx = 0;
                while xx < 4 {
                    let acc = filter_fn(flt_ptr, p0, p1, p2, p3, p4, p5, p6);
                    *ptr.offset(xx as isize) = bd.iclip_pixel(acc + 8 >> 4);
                    xx += 1;
                    flt_ptr = flt_ptr.offset(FLT_INCR);
                }
                ptr = ptr.offset(BD::pxstride(stride as usize) as isize);
                yy += 1;
            }
            left = &mut *dst.offset((x + 4 - 1) as isize) as *mut BD::Pixel;
            left_stride = BD::pxstride(stride as usize) as isize;
            top = top.offset(4);
            topleft = &*top.offset(-(1 as c_int) as isize) as *const BD::Pixel;
            x += 4 as c_int;
        }
        top = &mut *dst.offset(BD::pxstride(stride as usize) as isize) as *mut BD::Pixel;
        dst = &mut *dst.offset((BD::pxstride(stride as usize) * 2) as isize) as *mut BD::Pixel;
        y += 2 as c_int;
    }
}

// TODO(kkysen) Temporarily pub until mod is deduplicated
#[inline(never)]
pub(crate) unsafe fn cfl_ac_rust<BD: BitDepth>(
    mut ac: *mut i16,
    mut ypx: *const BD::Pixel,
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
            let mut ac_sum = (*ypx.offset((x << ss_hor) as isize)).as_::<c_int>();
            if ss_hor != 0 {
                ac_sum += (*ypx.offset((x * 2 + 1) as isize)).as_::<c_int>();
            }
            if ss_ver != 0 {
                ac_sum += (*ypx.offset(
                    ((x << ss_hor) as isize + BD::pxstride(stride as usize) as isize) as isize,
                ))
                .as_::<c_int>();
                if ss_hor != 0 {
                    ac_sum += (*ypx.offset(
                        ((x * 2 + 1) as isize + BD::pxstride(stride as usize) as isize) as isize,
                    ))
                    .as_::<c_int>();
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
        ypx = ypx.offset((BD::pxstride(stride as usize) << ss_ver) as isize);
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

// TODO(kkysen) Temporarily pub until mod is deduplicated
pub(crate) unsafe fn pal_pred_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
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
            *dst.offset(x as isize) =
                (*pal.offset(*idx.offset(x as isize) as isize)).as_::<BD::Pixel>();
            x += 1;
        }
        idx = idx.offset(w as isize);
        dst = dst.offset(BD::pxstride(stride as usize) as isize);
        y += 1;
    }
}
