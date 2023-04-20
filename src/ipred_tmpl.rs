use cfg_if::cfg_if;

pub unsafe fn filter_fn(
    flt_ptr: *const i8,
    p0: libc::c_int,
    p1: libc::c_int,
    p2: libc::c_int,
    p3: libc::c_int,
    p4: libc::c_int,
    p5: libc::c_int,
    p6: libc::c_int,
) -> libc::c_int {
    if cfg!(any(target_arch = "x86", target_arch = "x86_64")) {
        *flt_ptr.offset(0) as libc::c_int * p0
            + *flt_ptr.offset(1) as libc::c_int * p1
            + *flt_ptr.offset(16) as libc::c_int * p2
            + *flt_ptr.offset(17) as libc::c_int * p3
            + *flt_ptr.offset(32) as libc::c_int * p4
            + *flt_ptr.offset(33) as libc::c_int * p5
            + *flt_ptr.offset(48) as libc::c_int * p6
    } else {
        *flt_ptr.offset(0) as libc::c_int * p0
            + *flt_ptr.offset(8) as libc::c_int * p1
            + *flt_ptr.offset(16) as libc::c_int * p2
            + *flt_ptr.offset(24) as libc::c_int * p3
            + *flt_ptr.offset(32) as libc::c_int * p4
            + *flt_ptr.offset(40) as libc::c_int * p5
            + *flt_ptr.offset(48) as libc::c_int * p6
    }
}

cfg_if! {
    if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        pub const FLT_INCR: isize = 2;
    } else {
        pub const FLT_INCR: isize = 1;
    }
}
