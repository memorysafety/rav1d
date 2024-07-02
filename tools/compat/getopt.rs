use libc::c_char;
use libc::c_int;

#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

extern "C" {
    pub fn getopt_long(
        nargc: c_int,
        nargv: *const *mut c_char,
        options: *const c_char,
        long_options: *const option,
        id: *mut c_int,
    ) -> c_int;
}
