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
