#![cfg_attr(target_arch = "arm", feature(stdarch_arm_feature_detection))]
#![cfg_attr(
    any(target_arch = "riscv32", target_arch = "riscv64"),
    feature(stdarch_riscv_feature_detection)
)]
#![allow(clippy::derivable_impls, clippy::erasing_op, clippy::ptr_eq)]
#![expect(
    non_upper_case_globals,
    clippy::arc_with_non_send_sync,
    clippy::borrow_deref_ref,
    clippy::borrowed_box,
    clippy::cast_abs_to_unsigned,
    clippy::clone_on_copy,
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::doc_overindented_list_items,
    clippy::duplicate_underscore_argument,
    clippy::explicit_auto_deref,
    clippy::identity_op,
    clippy::incompatible_msrv,
    clippy::int_plus_one,
    clippy::into_iter_on_ref,
    clippy::large_const_arrays,
    clippy::large_enum_variant,
    clippy::len_without_is_empty,
    clippy::len_zero,
    clippy::let_and_return,
    clippy::let_underscore_lock,
    clippy::manual_div_ceil,
    clippy::manual_range_contains,
    clippy::manual_saturating_arithmetic,
    clippy::module_inception,
    clippy::misrefactored_assign_op,
    clippy::needless_borrow,
    clippy::needless_late_init,
    clippy::needless_lifetimes,
    clippy::needless_option_as_deref,
    clippy::needless_range_loop,
    clippy::needless_return,
    clippy::neg_multiply,
    clippy::nonminimal_bool,
    clippy::option_map_unit_fn,
    clippy::partialeq_to_none,
    clippy::precedence,
    clippy::redundant_closure,
    clippy::redundant_pattern_matching,
    clippy::redundant_static_lifetimes,
    clippy::search_is_some,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::unit_arg,
    clippy::uninlined_format_args,
    clippy::unnecessary_cast,
    clippy::unnecessary_fallible_conversions,
    clippy::unnecessary_map_on_constructor,
    clippy::unnecessary_mut_passed,
    clippy::unnecessary_lazy_evaluations,
    clippy::unneeded_wildcard_pattern,
    clippy::upper_case_acronyms,
    clippy::useless_conversion,
    clippy::zero_prefixed_literal,
)]
#![deny(
    unsafe_op_in_unsafe_fn,
    clippy::missing_safety_doc,
    clippy::undocumented_unsafe_blocks
)]

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
    pub(crate) mod assume;
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
    pub(crate) mod disjoint_mut;
    pub(crate) mod enum_map;
    mod env;
    pub(crate) mod error;
    mod ffi_safe;
    mod fg_apply;
    mod filmgrain;
    mod getbits;
    pub(crate) mod pic_or_buf;
    pub(crate) mod pixels;
    pub(crate) mod relaxed_atomic;
    pub mod send_sync_non_null;
    pub(crate) mod strided;
    pub(crate) mod with_offset;
    pub(crate) mod wrap_fn_ptr;
    // TODO(kkysen) Temporarily `pub(crate)` due to a `pub use` until TAIT.
    mod extensions;
    mod in_range;
    pub(super) mod internal;
    mod intra_edge;
    mod ipred;
    mod ipred_prepare;
    mod iter;
    mod itx;
    mod itx_1d;
    pub(crate) mod levels;
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
    mod pal;
    mod picture;
    mod qm;
    mod recon;
    mod refmvs;
    mod scan;
    mod tables;
    mod thread_task;
    mod warpmv;
    mod wedge;
} // mod src

pub use src::error::Dav1dResult;
