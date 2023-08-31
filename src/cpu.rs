use ::libc;
extern "C" {
    pub type Dav1dContext;
    fn dav1d_get_cpu_flags_x86() -> libc::c_uint;
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
    dav1d_cpu_flags = dav1d_get_cpu_flags_x86();
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_set_cpu_flags_mask(mask: libc::c_uint) {
    dav1d_cpu_flags_mask = mask;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_num_logical_processors(c: *mut Dav1dContext) -> libc::c_int {
    let mut affinity: cpu_set_t = cpu_set_t { __bits: [0; 16] };
    if pthread_getaffinity_np(
        pthread_self(),
        ::core::mem::size_of::<cpu_set_t>() as libc::c_ulong,
        &mut affinity,
    ) == 0
    {
        return __sched_cpucount(
            ::core::mem::size_of::<cpu_set_t>() as libc::c_ulong,
            &mut affinity,
        );
    }
    if !c.is_null() {
        dav1d_log(
            c,
            b"Unable to detect thread count, defaulting to single-threaded mode\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    return 1 as libc::c_int;
}
