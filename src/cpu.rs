use ::libc;
use cfg_if::cfg_if;
extern "C" {
    pub type Dav1dContext;
    #[cfg(target_arch = "x86_64")]
    fn dav1d_get_cpu_flags_x86() -> libc::c_uint;
    #[cfg(target_arch = "aarch64")]
    fn dav1d_get_cpu_flags_arm() -> libc::c_uint;
    fn dav1d_log(c: *mut Dav1dContext, format: *const libc::c_char, _: ...);
    fn __sched_cpucount(__setsize: size_t, __setp: *const cpu_set_t) -> libc::c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_getaffinity_np(
        __th: pthread_t,
        __cpusetsize: size_t,
        __cpuset: *mut cpu_set_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpu_set_t {
    pub __bits: [__cpu_mask; 16],
}
pub type __cpu_mask = libc::c_ulong;
pub type pthread_t = libc::c_ulong;
#[no_mangle]
pub static mut dav1d_cpu_flags: libc::c_uint = 0 as libc::c_uint;
#[no_mangle]
pub static mut dav1d_cpu_flags_mask: libc::c_uint = !(0 as libc::c_uint);
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_init_cpu() {
    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            dav1d_cpu_flags = dav1d_get_cpu_flags_x86();
        } else if #[cfg(target_arch = "aarch64")] {
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
    c: *mut Dav1dContext,
) -> libc::c_int {
    num_cpus::get() as libc::c_int
}
