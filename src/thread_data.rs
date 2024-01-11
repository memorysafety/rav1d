use libc::pthread_mutex_t;
use libc::pthread_t;
use std::ffi::c_int;
use std::sync::Condvar;

#[repr(C)]
pub struct thread_data {
    pub thread: pthread_t,
    pub cond: Condvar,
    pub lock: pthread_mutex_t,
    pub inited: c_int,
}
