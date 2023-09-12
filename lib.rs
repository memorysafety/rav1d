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
    pub mod stdio;
} // mod include
pub mod src {
    pub mod align;
    pub mod cdef;
    #[cfg(feature = "bitdepth_16")]
    pub mod cdef_apply_tmpl_16;
    #[cfg(feature = "bitdepth_8")]
    pub mod cdef_apply_tmpl_8;
    #[cfg(feature = "bitdepth_16")]
    pub mod cdef_tmpl_16;
    #[cfg(feature = "bitdepth_8")]
    pub mod cdef_tmpl_8;
    pub mod cdf;
    pub mod cpu;
    pub mod ctx;
    pub mod cursor;
    pub mod data;
    pub mod decode;
    pub mod dequant_tables;
    pub mod env;
    #[cfg(feature = "bitdepth_16")]
    pub mod fg_apply_tmpl_16;
    #[cfg(feature = "bitdepth_8")]
    pub mod fg_apply_tmpl_8;
    pub mod filmgrain;
    #[cfg(feature = "bitdepth_16")]
    pub mod filmgrain_tmpl_16;
    #[cfg(feature = "bitdepth_8")]
    pub mod filmgrain_tmpl_8;
    pub mod getbits;
    pub mod internal;
    pub mod intra_edge;
    pub mod ipred;
    pub mod ipred_prepare;
    #[cfg(feature = "bitdepth_16")]
    pub mod ipred_prepare_tmpl_16;
    #[cfg(feature = "bitdepth_8")]
    pub mod ipred_prepare_tmpl_8;
    pub mod ipred_tmpl;
    #[cfg(feature = "bitdepth_16")]
    pub mod ipred_tmpl_16;
    #[cfg(feature = "bitdepth_8")]
    pub mod ipred_tmpl_8;
    pub mod itx;
    pub mod itx_1d;
    #[cfg(feature = "bitdepth_16")]
    pub mod itx_tmpl_16;
    pub mod itx_tmpl_8;
    pub mod levels;
    #[cfg(feature = "bitdepth_16")]
    pub mod lf_apply_tmpl_16;
    #[cfg(feature = "bitdepth_8")]
    pub mod lf_apply_tmpl_8;
    pub mod lf_mask;
    pub mod lib;
    pub mod log;
    pub mod loopfilter;
    #[cfg(feature = "bitdepth_16")]
    pub mod loopfilter_tmpl_16;
    #[cfg(feature = "bitdepth_8")]
    pub mod loopfilter_tmpl_8;
    pub mod looprestoration;
    pub mod lr_apply;
    #[cfg(feature = "bitdepth_16")]
    pub mod lr_apply_tmpl_16;
    #[cfg(feature = "bitdepth_8")]
    pub mod lr_apply_tmpl_8;
    pub mod mc;
    #[cfg(feature = "bitdepth_16")]
    pub mod mc_tmpl_16;
    #[cfg(feature = "bitdepth_8")]
    pub mod mc_tmpl_8;
    pub mod mem;
    pub mod msac;
    pub mod obu;
    pub mod picture;
    pub mod qm;
    pub mod recon;
    #[cfg(feature = "bitdepth_16")]
    pub mod recon_tmpl_16;
    #[cfg(feature = "bitdepth_8")]
    pub mod recon_tmpl_8;
    pub mod r#ref;
    pub mod refmvs;
    pub mod scan;
    pub mod tables;
    pub mod thread_data;
    pub mod thread_task;
    pub mod warpmv;
    pub mod wedge;
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub mod x86 {
        pub mod cpu;
    } // mod x86
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    pub mod arm {
        pub mod cpu;
    } // mod arm
} // mod src

// NOTE: temporary code to support Linux and macOS, should be removed eventually
cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        extern "C" {
            pub static mut stdout: *mut libc::FILE;
            pub static mut stderr: *mut libc::FILE;
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
