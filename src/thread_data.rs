use libc::pthread_cond_t;
use libc::pthread_mutex_t;
use libc::pthread_t;
use std::ffi::c_int;

#[repr(C)]
pub struct thread_data {
    pub thread: pthread_t,
    pub cond: pthread_cond_t,
    pub lock: pthread_mutex_t,
    pub inited: c_int,
}
