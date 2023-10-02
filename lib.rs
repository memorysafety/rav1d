#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(c_variadic)]
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![cfg_attr(target_arch = "arm", feature(stdsimd))]

#[cfg(not(any(feature = "bitdepth_8", feature = "bitdepth_16")))]
compile_error!("No bitdepths enabled. Enable one or more of the following features: `bitdepth_8`, `bitdepth_16`");

pub mod include {
    pub mod common {
        pub(crate) mod attributes;
        pub(crate) mod bitdepth;
        pub(crate) mod dump;
        pub mod frame;
        pub(crate) mod intops;
        pub(crate) mod validate;
    } // mod common
    pub mod dav1d {
        pub mod common;
        pub mod data;
        pub mod dav1d;
        pub mod headers;
        pub mod picture;
    } // mod dav1d
    pub(crate) mod stdatomic;
} // mod include
pub mod src {
    mod align;
    mod cdef;
    #[cfg(feature = "bitdepth_16")]
    mod cdef_apply_tmpl_16;
    #[cfg(feature = "bitdepth_8")]
    mod cdef_apply_tmpl_8;
    #[cfg(feature = "bitdepth_16")]
    mod cdef_tmpl_16;
    #[cfg(feature = "bitdepth_8")]
    mod cdef_tmpl_8;
    mod cdf;
    mod const_fn;
    pub mod cpu;
    mod ctx;
    mod cursor;
    mod data;
    mod decode;
    mod dequant_tables;
    mod env;
    pub(crate) mod error;
    #[cfg(feature = "bitdepth_16")]
    mod fg_apply_tmpl_16;
    #[cfg(feature = "bitdepth_8")]
    mod fg_apply_tmpl_8;
    mod filmgrain;
    #[cfg(feature = "bitdepth_16")]
    mod filmgrain_tmpl_16;
    #[cfg(feature = "bitdepth_8")]
    mod filmgrain_tmpl_8;
    mod getbits;
    // TODO(kkysen) Temporarily `pub(crate)` due to a `pub use` until TAIT.
    pub(super) mod internal;
    mod intra_edge;
    mod ipred;
    mod ipred_prepare;
    mod ipred_tmpl;
    #[cfg(feature = "bitdepth_16")]
    mod ipred_tmpl_16;
    #[cfg(feature = "bitdepth_8")]
    mod ipred_tmpl_8;
    mod itx;
    mod itx_1d;
    #[cfg(feature = "bitdepth_16")]
    mod itx_tmpl_16;
    mod itx_tmpl_8;
    mod levels;
    #[cfg(feature = "bitdepth_16")]
    mod lf_apply_tmpl_16;
    #[cfg(feature = "bitdepth_8")]
    mod lf_apply_tmpl_8;
    mod lf_mask;
    pub mod lib;
    mod log;
    mod loopfilter;
    #[cfg(feature = "bitdepth_16")]
    mod loopfilter_tmpl_16;
    #[cfg(feature = "bitdepth_8")]
    mod loopfilter_tmpl_8;
    mod looprestoration;
    mod lr_apply;
    #[cfg(feature = "bitdepth_16")]
    mod lr_apply_tmpl_16;
    #[cfg(feature = "bitdepth_8")]
    mod lr_apply_tmpl_8;
    mod mc;
    mod mem;
    mod msac;
    mod obu;
    mod picture;
    mod qm;
    mod recon;
    #[cfg(feature = "bitdepth_16")]
    mod recon_tmpl_16;
    #[cfg(feature = "bitdepth_8")]
    mod recon_tmpl_8;
    pub(crate) mod r#ref;
    mod refmvs;
    mod scan;
    mod tables;
    mod thread_data;
    mod thread_task;
    mod warpmv;
    mod wedge;
} // mod src

use std::ffi::c_int;

pub use src::error::Dav1dResult;

// NOTE: temporary code to support Linux and macOS, should be removed eventually
cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        extern "C" {
            pub static mut stdout: *mut libc::FILE;

            pub static mut stderr: *mut libc::FILE;
        }

        pub unsafe fn errno_location() -> *mut c_int {
            libc::__errno_location()
        }
    } else if #[cfg(target_os = "macos")] {
        extern "C" {
            #[link_name = "__stdoutp"]
            pub static mut stdout: *mut libc::FILE;

            #[link_name = "__stderrp"]
            pub static mut stderr: *mut libc::FILE;
        }

        pub unsafe fn errno_location() -> *mut c_int {
            libc::__error()
        }
    }
}
