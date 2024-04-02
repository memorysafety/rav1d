#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(c_variadic)]
#![cfg_attr(target_arch = "arm", feature(stdarch_arm_feature_detection))]
#![allow(clippy::all)]

#[cfg(not(any(feature = "bitdepth_8", feature = "bitdepth_16")))]
compile_error!("No bitdepths enabled. Enable one or more of the following features: `bitdepth_8`, `bitdepth_16`");

pub mod include {
    pub mod common {
        pub(crate) mod attributes;
        pub(crate) mod bitdepth;
        pub(crate) mod dump;
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
} // mod include
pub mod src {
    pub mod align;
    mod assume;
    pub(crate) mod c_arc;
    pub(crate) mod c_box;
    mod cdef;
    mod cdef_apply;
    mod cdf;
    mod const_fn;
    pub mod cpu;
    mod ctx;
    mod cursor;
    mod data;
    mod decode;
    mod dequant_tables;
    mod disjoint_mut;
    pub(crate) mod enum_map;
    mod env;
    pub(crate) mod error;
    mod fg_apply;
    mod filmgrain;
    mod getbits;
    mod unstable_extensions;
    pub(crate) mod wrap_fn_ptr;
    // TODO(kkysen) Temporarily `pub(crate)` due to a `pub use` until TAIT.
    pub(super) mod internal;
    mod intra_edge;
    mod ipred;
    mod ipred_prepare;
    mod itx;
    mod itx_1d;
    mod levels;
    mod lf_apply;
    mod lf_mask;
    pub mod lib;
    pub(crate) mod log;
    mod loopfilter;
    mod looprestoration;
    mod lr_apply;
    mod mc;
    mod mem;
    mod msac;
    mod obu;
    mod picture;
    mod qm;
    mod recon;
    pub(crate) mod r#ref;
    mod refmvs;
    mod scan;
    mod tables;
    mod thread_task;
    mod warpmv;
    mod wedge;
} // mod src

pub use src::error::Dav1dResult;
