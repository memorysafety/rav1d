pub type pthread_t = libc::pthread_t;
pub type pthread_cond_t = libc::pthread_cond_t;
pub type pthread_attr_t = libc::pthread_attr_t;
pub type pthread_condattr_t = libc::pthread_condattr_t;
pub type pthread_mutexattr_t = libc::pthread_mutexattr_t;
// NOTE: temporary code to support Linux and macOS, should be removed eventually
cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub type pthread_once_t = libc::c_int;

        pub const fn pthread_once_init() -> pthread_once_t {
            0
        }
    } else if #[cfg(target_os = "macos")] {
        #[derive(Copy, Clone)]
        #[repr(C)]
        pub struct _opaque_pthread_once_t {
            pub __sig: libc::c_long,
            pub __opaque: [libc::c_char; 8],
        }
        pub type pthread_once_t = _opaque_pthread_once_t;

        pub const fn pthread_once_init() -> pthread_once_t {
            let init = _opaque_pthread_once_t {
                __sig: 0x30b1bcba,
                __opaque: [0 , 0, 0, 0, 0, 0, 0, 0],
            };
            init
        }
    }
}
