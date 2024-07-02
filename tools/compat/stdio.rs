// NOTE: temporary code to support Linux and macOS, should be removed eventually
cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        extern "C" {
            pub static mut stdout: *mut libc::FILE;

            pub static mut stderr: *mut libc::FILE;
        }
    } else if #[cfg(target_os = "macos")] {
        extern "C" {
            #[link_name = "__stdoutp"]
            pub static mut stdout: *mut libc::FILE;

            #[link_name = "__stderrp"]
            pub static mut stderr: *mut libc::FILE;
        }
    } else if #[cfg(target_os = "windows")] {
        extern "C" {
             fn __acrt_iob_func(fileno: u32) -> *mut libc::FILE;

             pub fn stdout() -> *mut libc::FILE {
                __acrt_iob_func(1)
             }

             pub fn stderr() -> *mut libc::FILE {
                __acrt_iob_func(2)
             }
        }
    }
}
