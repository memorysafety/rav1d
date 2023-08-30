use libc::pthread_cond_t;
use libc::pthread_mutex_t;
use libc::pthread_t;

#[repr(C)]
pub struct thread_data {
    pub thread: pthread_t,
    pub cond: pthread_cond_t,
    pub lock: pthread_mutex_t,
    pub inited: libc::c_int,
}
