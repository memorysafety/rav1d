// NOTE: temporary code to support Linux and macOS, should be removed eventually
cfg_if::cfg_if! {
    if #[cfg(any(target_os = "linux", target_os = "android"))] {
        pub use libc::snprintf;
        pub use libc::fseeko;
        pub use libc::ftello;

        extern "C" {
            #[link_name = "stdout"]
            pub static mut __stdoutp: *mut libc::FILE;

            #[link_name = "stderr"]
            pub static mut __stderrp: *mut libc::FILE;
        }

        pub unsafe extern "C" fn stdout() -> *mut libc::FILE {
            __stdoutp
        }

        pub unsafe extern "C" fn stderr() -> *mut libc::FILE {
            __stderrp
        }
    } else if #[cfg(target_os = "macos")] {
        pub use libc::snprintf;
        pub use libc::fseeko;
        pub use libc::ftello;

        extern "C" {
            pub static mut __stdoutp: *mut libc::FILE;

            pub static mut __stderrp: *mut libc::FILE;
        }

        pub unsafe extern "C" fn stdout() -> *mut libc::FILE {
            __stdoutp
        }

        pub unsafe extern "C" fn stderr() -> *mut libc::FILE {
            __stderrp
        }
    } else if #[cfg(target_os = "windows")] {
        use libc::fseek;
        use libc::ftell;

        extern "C" {
            fn __acrt_iob_func(fileno: u32) -> *mut libc::FILE;
            pub fn snprintf(
                s: *mut libc::c_char,
                n: libc::size_t,
                format: *const libc::c_char,
                ...
            ) -> libc::c_int;
        }

        pub unsafe extern "C" fn stdout() -> *mut libc::FILE {
            __acrt_iob_func(1)
        }

        pub unsafe extern "C" fn stderr() -> *mut libc::FILE {
            __acrt_iob_func(2)
        }

        pub unsafe extern "C" fn fseeko(
            stream: *mut libc::FILE,
            offset: libc::off_t,
            whence: libc::c_int
        ) -> libc::c_int {
            fseek(stream, offset.into(), whence)
        }

        pub unsafe extern "C" fn ftello(stream: *mut libc::FILE) -> libc::off_t {
            ftell(stream).into()
        }
    }
}
