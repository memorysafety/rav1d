#[inline]
pub fn ac_dump(mut buf: &[i16], w: libc::c_int, h: libc::c_int, what: &str) {
    println!("{}", what);
    for buf in buf.chunks_exact(w as usize).take(h as usize) {
        for x in buf {
            print!(" {:03}", x);
        }
        println!();
    }
}
