#[cfg(feature = "asm")]
use cfg_if::cfg_if;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

#[cfg(target_arch = "x86_64")]
use crate::src::x86::cpu::dav1d_get_cpu_flags_x86;

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
use crate::src::arm::cpu::dav1d_get_cpu_flags_arm;

extern "C" {
    pub type Dav1dContext;
}

static dav1d_cpu_flags: AtomicU32 = AtomicU32::new(0);
static dav1d_cpu_flags_mask: AtomicU32 = AtomicU32::new(!0);

#[cfg(feature = "asm")]
#[inline(always)]
pub fn dav1d_get_cpu_flags() -> libc::c_uint {
    let mut flags =
        dav1d_cpu_flags.load(Ordering::SeqCst) & dav1d_cpu_flags_mask.load(Ordering::SeqCst);
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            use crate::src::x86::cpu::DAV1D_X86_CPU_FLAG_SSE2;
            flags |= DAV1D_X86_CPU_FLAG_SSE2;
        }
    }
    flags
}

#[no_mangle]
#[cold]
pub unsafe fn dav1d_init_cpu() {
    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            dav1d_cpu_flags.store(dav1d_get_cpu_flags_x86(), Ordering::SeqCst);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
            dav1d_cpu_flags.store(dav1d_get_cpu_flags_arm(), Ordering::SeqCst);
        }
    }
}

#[no_mangle]
#[cold]
pub extern "C" fn dav1d_set_cpu_flags_mask(mask: libc::c_uint) {
    dav1d_cpu_flags_mask.store(mask, Ordering::SeqCst);
}

#[no_mangle]
#[cold]
pub fn dav1d_num_logical_processors(_c: *mut Dav1dContext) -> libc::c_int {
    num_cpus::get() as libc::c_int
}
