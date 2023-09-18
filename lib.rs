#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(c_variadic)]
#![feature(core_intrinsics)]
#![feature(extern_types)]

#[cfg(not(any(feature = "bitdepth_8", feature = "bitdepth_16")))]
compile_error!("No bitdepths enabled. Enable one or more of the following features: `bitdepth_8`, `bitdepth_16`");

pub mod include {
    pub mod common {
        pub mod attributes;
        pub mod bitdepth;
        pub mod dump;
        pub mod frame;
        pub mod intops;
    } // mod common
    pub mod dav1d {
        pub mod common;
        pub mod data;
        pub mod dav1d;
        pub mod headers;
        pub mod picture;
    } // mod dav1d
    pub mod pthread;
    pub mod sched;
    pub mod stdatomic;
    pub mod stddef;
    pub mod stdint;
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
    pub mod cpu;
    mod ctx;
    mod cursor;
    mod data;
    mod decode;
    mod dequant_tables;
    mod env;
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
    mod internal;
    mod intra_edge;
    mod ipred;
    mod ipred_prepare;
    #[cfg(feature = "bitdepth_16")]
    mod ipred_prepare_tmpl_16;
    #[cfg(feature = "bitdepth_8")]
    mod ipred_prepare_tmpl_8;
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
    #[cfg(feature = "bitdepth_16")]
    mod mc_tmpl_16;
    #[cfg(feature = "bitdepth_8")]
    mod mc_tmpl_8;
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
    pub mod r#ref;
    mod refmvs;
    mod scan;
    mod tables;
    mod thread_data;
    mod thread_task;
    mod warpmv;
    mod wedge;
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    mod x86 {
        pub mod cpu;
    } // mod x86
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    mod arm {
        pub mod cpu;
    } // mod arm
} // mod src

// NOTE: temporary code to support Linux and macOS, should be removed eventually
cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        extern "C" {
            static mut stderr: *mut libc::FILE;
        }

        unsafe fn errno_location() -> *mut libc::c_int {
            libc::__errno_location()
        }
    } else if #[cfg(target_os = "macos")] {
        extern "C" {
            #[link_name = "__stderrp"]
            static mut stderr: *mut libc::FILE;
        }

        unsafe fn errno_location() -> *mut libc::c_int {
            libc::__error()
        }
    }
}
