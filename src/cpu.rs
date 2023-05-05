#[cfg(feature = "asm")]
use cfg_if::cfg_if;

extern "C" {
    pub type Dav1dContext;
    #[cfg(target_arch = "x86_64")]
    fn dav1d_get_cpu_flags_x86() -> libc::c_uint;
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    fn dav1d_get_cpu_flags_arm() -> libc::c_uint;
}

pub static mut dav1d_cpu_flags: libc::c_uint = 0;
pub static mut dav1d_cpu_flags_mask: libc::c_uint = !0;

#[cfg(feature = "asm")]
#[inline(always)]
pub unsafe fn dav1d_get_cpu_flags() -> libc::c_uint {
    let mut flags = dav1d_cpu_flags & dav1d_cpu_flags_mask;
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
            dav1d_cpu_flags = dav1d_get_cpu_flags_x86();
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
            dav1d_cpu_flags = dav1d_get_cpu_flags_arm();
        }
    }
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_set_cpu_flags_mask(mask: libc::c_uint) {
    dav1d_cpu_flags_mask = mask;
}

#[no_mangle]
#[cold]
pub fn dav1d_num_logical_processors(_c: *mut Dav1dContext) -> libc::c_int {
    num_cpus::get() as libc::c_int
}
