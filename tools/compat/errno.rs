use std::ffi::c_int;

// NOTE: temporary code to support Linux and macOS, should be removed eventually
cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub unsafe fn errno_location() -> *mut c_int {
            libc::__errno_location()
        }
    } else if #[cfg(target_os = "macos")] {
        pub unsafe fn errno_location() -> *mut c_int {
            libc::__error()
        }
    } else if #[cfg(target_os = "android")] {
        pub unsafe fn errno_location() -> *mut c_int {
            libc::__errno()
        }
    } else if #[cfg(target_os = "windows")] {
        extern "C" {
            pub fn _errno() -> *mut c_int;
        }

        pub unsafe fn errno_location() -> *mut c_int {
            _errno()
        }
    }
}
