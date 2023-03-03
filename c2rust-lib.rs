#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(asm)]
#![feature(c_variadic)]
#![feature(core_intrinsics)]
#![feature(extern_types)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod src {
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
pub mod data;
pub mod decode;
pub mod dequant_tables;
#[cfg(feature = "bitdepth_16")]
pub mod fg_apply_tmpl_16;
#[cfg(feature = "bitdepth_8")]
pub mod fg_apply_tmpl_8;
#[cfg(feature = "bitdepth_16")]
pub mod filmgrain_tmpl_16;
#[cfg(feature = "bitdepth_8")]
pub mod filmgrain_tmpl_8;
pub mod getbits;
pub mod intra_edge;
#[cfg(feature = "bitdepth_16")]
pub mod ipred_prepare_tmpl_16;
#[cfg(feature = "bitdepth_8")]
pub mod ipred_prepare_tmpl_8;
#[cfg(feature = "bitdepth_16")]
pub mod ipred_tmpl_16;
#[cfg(feature = "bitdepth_8")]
pub mod ipred_tmpl_8;
pub mod itx_1d;
#[cfg(feature = "bitdepth_16")]
pub mod itx_tmpl_16;
pub mod itx_tmpl_8;
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
#[cfg(feature = "bitdepth_16")]
pub mod looprestoration_tmpl_16;
#[cfg(feature = "bitdepth_8")]
pub mod looprestoration_tmpl_8;
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
#[cfg(feature = "bitdepth_16")]
pub mod recon_tmpl_16;
#[cfg(feature = "bitdepth_8")]
pub mod recon_tmpl_8;
pub mod refmvs;
pub mod scan;
pub mod tables;
pub mod thread_task;
pub mod warpmv;
pub mod wedge;
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
