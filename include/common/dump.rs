use crate::include::stdint::int16_t;

#[inline]
pub unsafe fn ac_dump(
    mut buf: *const int16_t,
    w: libc::c_int,
    h: libc::c_int,
    what: &str,
) {
    println!("{}", what);
    for _ in 0..h {
        for x in 0..w {
            print!(" {:03}", *buf.offset(x as isize));
        }
        buf = buf.offset(w as isize);
        println!();
    }
}
