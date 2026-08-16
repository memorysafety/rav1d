#[allow(non_camel_case_types)]
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
pub(crate) type intptr_t = isize;

#[allow(non_camel_case_types)]
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
pub(crate) type off_t = i64;

#[allow(non_camel_case_types)]
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
pub(crate) type ptrdiff_t = isize;

#[allow(non_camel_case_types)]
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
pub(crate) type uintptr_t = usize;

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub(crate) use libc::{intptr_t, off_t, ptrdiff_t, uintptr_t};

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
pub(crate) const EAGAIN: u8 = 11;
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub(crate) const EAGAIN: u8 = libc::EAGAIN as u8;

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
pub(crate) const EINVAL: u8 = 22;
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub(crate) const EINVAL: u8 = libc::EINVAL as u8;

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
pub(crate) const ENOENT: u8 = 2;
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub(crate) const ENOENT: u8 = libc::ENOENT as u8;

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
pub(crate) const ENOMEM: u8 = 12;
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub(crate) const ENOMEM: u8 = libc::ENOMEM as u8;

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
pub(crate) const ENOPROTOOPT: u8 = 92;
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub(crate) const ENOPROTOOPT: u8 = libc::ENOPROTOOPT as u8;

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
pub(crate) const ERANGE: u8 = 34;
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub(crate) const ERANGE: u8 = libc::ERANGE as u8;
