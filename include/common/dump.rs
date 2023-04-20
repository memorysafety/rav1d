use crate::include::stdint::int16_t;

extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}

#[inline]
pub unsafe extern "C" fn ac_dump(
    mut buf: *const int16_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut what: *const libc::c_char,
) {
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, what);
    loop {
        let fresh1 = h;
        h = h - 1;
        if !(fresh1 != 0) {
            break;
        }
        let mut x = 0;
        while x < w {
            printf(
                b" %03d\0" as *const u8 as *const libc::c_char,
                *buf.offset(x as isize) as libc::c_int,
            );
            x += 1;
        }
        buf = buf.offset(w as isize);
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    };
}
