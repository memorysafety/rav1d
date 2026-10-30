//! C types that [`std::ffi`] does not have.
//!
//! [`libc`] defines [`ptrdiff_t`], [`intptr_t`] and [`uintptr_t`] like this
//! on every target it supports, and not at all on a target without a C
//! library, such as `wasm32-unknown-unknown`. Defining them here lets
//! `rav1d` build there too.

#![allow(non_camel_case_types)]

pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub type off_t = libc::off_t;

/// There is no C library to take `off_t` from,
/// and no C caller that could disagree with this one.
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
pub type off_t = i64;
