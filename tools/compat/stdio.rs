// NOTE: temporary code to support Linux and macOS, should be removed eventually
cfg_if::cfg_if! {
    if #[cfg(any(target_os = "linux", target_os = "android"))] {
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
    }
}
