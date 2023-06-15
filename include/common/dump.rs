use std::fmt::Display;

#[inline]
pub unsafe fn coef_dump<Coef: Display>(
    buf: *const Coef,
    w: usize,
    h: usize,
    len: usize,
    what: &str,
) {
    let buf = std::slice::from_raw_parts(buf, w * h);
    println!("{}", what);
    for buf in buf.chunks_exact(w).take(h) {
        for x in buf {
            print!(" {:0len$}", x, len = len);
        }
        println!();
    }
}

#[inline]
pub fn ac_dump(mut buf: &[i16; 32 * 32], w: usize, h: usize, what: &str) {
    println!("{}", what);
    for buf in buf.chunks_exact(w).take(h) {
        for x in buf {
            print!(" {:03}", x);
        }
        println!();
    }
}
