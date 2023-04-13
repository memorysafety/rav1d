#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(core_intrinsics)]
#![feature(extern_types)]

#[cfg(not(any(feature = "bitdepth_8", feature = "bitdepth_16")))]
compile_error!("No bitdepths enabled. Enable one or more of the following features: `bitdepth_8`, `bitdepth_16`");

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod include {
pub mod common {
pub mod attributes;
pub mod dump;
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
pub mod sys {
pub mod types;
} // mod sys
pub mod time;
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
pub mod ipred_prepare;
#[cfg(feature = "bitdepth_16")]
pub mod ipred_prepare_tmpl_16;
#[cfg(feature = "bitdepth_8")]
pub mod ipred_prepare_tmpl_8;
pub mod ipred;
pub mod ipred_tmpl;
#[cfg(feature = "bitdepth_16")]
pub mod ipred_tmpl_16;
#[cfg(feature = "bitdepth_8")]
pub mod ipred_tmpl_8;
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
#[cfg(feature = "bitdepth_16")]
pub mod loopfilter_tmpl_16;
#[cfg(feature = "bitdepth_8")]
pub mod loopfilter_tmpl_8;
pub mod looprestoration;
#[cfg(feature = "bitdepth_16")]
pub mod looprestoration_tmpl_16;
#[cfg(feature = "bitdepth_8")]
pub mod looprestoration_tmpl_8;
pub mod lr_apply;
#[cfg(feature = "bitdepth_16")]
pub mod lr_apply_tmpl_16;
#[cfg(feature = "bitdepth_8")]
pub mod lr_apply_tmpl_8;
#[cfg(feature = "bitdepth_16")]
pub mod mc_tmpl_16;
#[cfg(feature = "bitdepth_8")]
pub mod mc_tmpl_8;
pub mod mem;
pub mod msac;
pub mod obu;
pub mod picture;
pub mod qm;
pub mod r#ref;
pub mod recon;
#[cfg(feature = "bitdepth_16")]
pub mod recon_tmpl_16;
#[cfg(feature = "bitdepth_8")]
pub mod recon_tmpl_8;
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
pub mod tools {
pub mod dav1d_cli_parse;
pub mod input {
pub mod annexb;
pub mod input;
pub mod ivf;
pub mod section5;
} // mod input
pub mod output {
pub mod md5;
pub mod null;
pub mod output;
pub mod y4m2;
pub mod yuv;
} // mod output
} // mod tools

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
            #[link_name = "__stdoutp"]
            pub static mut stdout: *mut libc::FILE;
            #[link_name = "__stderrp"]
            pub static mut stderr: *mut libc::FILE;
        }

        unsafe fn errno_location() -> *mut libc::c_int {
            libc::__error()
        }
    }
}
