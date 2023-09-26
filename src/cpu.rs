use crate::src::internal::Dav1dContext;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

#[cfg(feature = "asm")]
use cfg_if::cfg_if;

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
pub use crate::src::x86::cpu::CpuFlags;

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
pub use crate::src::arm::cpu::CpuFlags;

/// This is atomic, which has interior mutability,
/// instead of a `static mut`, since the latter is `unsafe` to access.
///
/// It seems to only be used in init functions,
/// should it shouldn't be performance sensitive.
///
/// It is written once by [`dav1d_init_cpu`] in initialization code,
/// and then subsequently read in [`dav1d_get_cpu_flags`] by other initialization code.
#[cfg(feature = "asm")]
static dav1d_cpu_flags: AtomicU32 = AtomicU32::new(0);

/// This is atomic, which has interior mutability,
/// instead of a `static mut`, since the latter is `unsafe` to access.
///
/// It is modifiable through the publicly exported [`dav1d_set_cpu_flags_mask`],
/// so strict safety guarantees about how it's used can't be made.
/// Other than that, it is also only used in init functions (that call [`dav1d_get_cpu_flags`]),
/// so it shouldn't be performance sensitive.
static dav1d_cpu_flags_mask: AtomicU32 = AtomicU32::new(!0);

#[cfg(feature = "asm")]
#[inline(always)]
pub(crate) fn dav1d_get_cpu_flags() -> CpuFlags {
    let flags =
        dav1d_cpu_flags.load(Ordering::SeqCst) & dav1d_cpu_flags_mask.load(Ordering::SeqCst);
    // Note that `bitflags!` `struct`s are `#[repr(transparent)]`.
    let mut flags = CpuFlags::from_bits_truncate(flags);
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            flags |= CpuFlags::SSE2;
        } else {
            // For `unused_mut`.
            flags |= CpuFlags::empty();
        }
    }
    flags
}

#[cold]
pub(crate) fn dav1d_init_cpu() {
    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            use crate::src::x86::cpu::dav1d_get_cpu_flags_x86;

            dav1d_cpu_flags.store(dav1d_get_cpu_flags_x86().bits(), Ordering::SeqCst);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
            use crate::src::arm::cpu::dav1d_get_cpu_flags_arm;

            dav1d_cpu_flags.store(dav1d_get_cpu_flags_arm().bits(), Ordering::SeqCst);
        }
    }
}

#[no_mangle]
#[cold]
pub extern "C" fn dav1d_set_cpu_flags_mask(mask: c_uint) {
    dav1d_cpu_flags_mask.store(mask, Ordering::SeqCst);
}

#[cold]
pub(crate) fn dav1d_num_logical_processors(_c: *mut Dav1dContext) -> c_int {
    num_cpus::get() as c_int
}
