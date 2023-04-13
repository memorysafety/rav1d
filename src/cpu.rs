use crate::include::stddef::*;
use ::libc;
use cfg_if::cfg_if;
extern "C" {
    pub type Dav1dContext;
    #[cfg(target_arch = "x86_64")]
    fn dav1d_get_cpu_flags_x86() -> libc::c_uint;
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    fn dav1d_get_cpu_flags_arm() -> libc::c_uint;
    fn __sched_cpucount(__setsize: size_t, __setp: *const cpu_set_t) -> libc::c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_getaffinity_np(
        __th: pthread_t,
        __cpusetsize: size_t,
        __cpuset: *mut cpu_set_t,
    ) -> libc::c_int;
}
use crate::include::sched::cpu_set_t;

use crate::include::pthread::pthread_t;

#[no_mangle]
pub static mut dav1d_cpu_flags: libc::c_uint = 0 as libc::c_uint;
#[no_mangle]
pub static mut dav1d_cpu_flags_mask: libc::c_uint = !(0 as libc::c_uint);

#[cfg(feature = "asm")]
#[inline(always)]
pub unsafe extern "C" fn dav1d_get_cpu_flags() -> libc::c_uint {
    let mut flags: libc::c_uint = dav1d_cpu_flags & dav1d_cpu_flags_mask;
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            use crate::src::x86::cpu::DAV1D_X86_CPU_FLAG_SSE2;
            flags |= DAV1D_X86_CPU_FLAG_SSE2;
        }
    }
    return flags;
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_init_cpu() {
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
pub unsafe extern "C" fn dav1d_num_logical_processors(
    _c: *mut Dav1dContext,
) -> libc::c_int {
    num_cpus::get() as libc::c_int
}
