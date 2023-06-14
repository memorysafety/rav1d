#[inline]
pub unsafe fn ac_dump(mut buf: &[i16], w: libc::c_int, h: libc::c_int, what: &str) {
    println!("{}", what);
    for _ in 0..h {
        for x in 0..w {
            print!(" {:03}", buf[x as usize]);
        }
        buf = &buf[w as usize..];
        println!();
    }
}
