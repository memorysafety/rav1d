use crate::include::stdint::int16_t;

#[inline]
pub unsafe fn ac_dump(
    mut buf: *const int16_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
    what: &str,
) {
    println!("{}", what);
    loop {
        let fresh1 = h;
        h = h - 1;
        if !(fresh1 != 0) {
            break;
        }
        let mut x = 0;
        while x < w {
            print!(" {:03}", *buf.offset(x as isize));
            x += 1;
        }
        buf = buf.offset(w as isize);
        println!();
    }
}
