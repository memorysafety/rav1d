use crate::include::pthread::pthread_cond_t;
use crate::include::pthread::pthread_t;
use libc::pthread_mutex_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct thread_data {
    pub thread: pthread_t,
    pub cond: pthread_cond_t,
    pub lock: pthread_mutex_t,
    pub inited: libc::c_int,
}
