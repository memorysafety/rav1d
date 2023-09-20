use crate::include::common::bitdepth::DynPixel;
use crate::include::stddef::ptrdiff_t;

#[inline]
pub unsafe extern "C" fn get_upsample(
    wh: libc::c_int,
    angle: libc::c_int,
    is_sm: libc::c_int,
) -> libc::c_int {
    return (angle < 40 && wh <= 16 >> is_sm) as libc::c_int;
}

pub type angular_ipred_fn = unsafe extern "C" fn(
    *mut DynPixel,
    ptrdiff_t,
    *const DynPixel,
    libc::c_int,
    libc::c_int,
    libc::c_int,
    libc::c_int,
    libc::c_int,
    libc::c_int,
) -> ();

pub type cfl_ac_fn = unsafe extern "C" fn(
    *mut i16,
    *const DynPixel,
    ptrdiff_t,
    libc::c_int,
    libc::c_int,
    libc::c_int,
    libc::c_int,
) -> ();

pub type cfl_pred_fn = unsafe extern "C" fn(
    *mut DynPixel,
    ptrdiff_t,
    *const DynPixel,
    libc::c_int,
    libc::c_int,
    *const i16,
    libc::c_int,
    libc::c_int,
) -> ();

pub type pal_pred_fn = unsafe extern "C" fn(
    *mut DynPixel,
    ptrdiff_t,
    *const u16,
    *const u8,
    libc::c_int,
    libc::c_int,
) -> ();

#[repr(C)]
pub struct Dav1dIntraPredDSPContext {
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
            width: libc::c_int,
            height: libc::c_int,
            angle: libc::c_int,
            max_width: libc::c_int,
            max_height: libc::c_int,
            bitdepth_max: libc::c_int,
        );
    };

    (cfl_pred, $name:ident) => {
        pub(crate) fn $name(
            dst: *mut DynPixel,
            stride: ptrdiff_t,
            topleft: *const DynPixel,
            width: libc::c_int,
            height: libc::c_int,
            ac: *const i16,
            alpha: libc::c_int,
            bitdepth_max: libc::c_int,
        );
    };

    (cfl_ac, $name:ident) => {
        pub(crate) fn $name(
            ac: *mut i16,
            y: *const DynPixel,
            stride: ptrdiff_t,
            w_pad: libc::c_int,
            h_pad: libc::c_int,
            cw: libc::c_int,
            ch: libc::c_int,
        );
    };

    (pal_pred, $name:ident) => {
        pub(crate) fn $name(
            dst: *mut DynPixel,
            stride: ptrdiff_t,
            pal: *const u16,
            idx: *const u8,
            w: libc::c_int,
            h: libc::c_int,
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
