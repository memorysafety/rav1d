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
pub mod cdef_apply_tmpl;
pub mod cdef_tmpl;
pub mod cdf;
pub mod cpu;
pub mod data;
pub mod decode;
pub mod dequant_tables;
pub mod fg_apply_tmpl;
pub mod filmgrain_tmpl;
pub mod getbits;
pub mod intra_edge;
pub mod ipred_prepare_tmpl;
pub mod ipred_tmpl;
pub mod itx_1d;
pub mod itx_tmpl;
pub mod lf_apply_tmpl;
pub mod lf_mask;
pub mod lib;
pub mod log;
pub mod loopfilter_tmpl;
pub mod looprestoration_tmpl;
pub mod lr_apply_tmpl;
pub mod mc_tmpl;
pub mod mem;
pub mod msac;
pub mod obu;
pub mod picture;
pub mod qm;
pub mod r#ref;
pub mod recon_tmpl;
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
