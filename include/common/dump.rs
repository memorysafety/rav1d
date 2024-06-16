use crate::include::common::bitdepth::BitDepth;
use std::fmt::Display;
use std::io;
use std::io::stdout;

#[inline]
pub unsafe fn hex_fdump<BD: BitDepth>(
    out: &mut impl io::Write,
    buf: *const BD::Pixel,
    stride: usize,
    w: usize,
    h: usize,
    what: &str,
) -> io::Result<()> {
    let len = if h == 0 {
        0
    } else {
        (h - 1) * BD::pxstride(stride) + w
    };
    let buf = std::slice::from_raw_parts(buf, len);

    write!(out, "{}", what)?;
    for buf in buf.chunks(BD::pxstride(stride)) {
        for &x in &buf[..w] {
            write!(out, " {}", BD::display(x))?;
        }
        writeln!(out)?;
    }
    Ok(())
}

#[inline]
pub unsafe fn hex_dump<BD: BitDepth>(
    buf: *const BD::Pixel,
    stride: usize,
    w: usize,
    h: usize,
    what: &str,
) {
    hex_fdump::<BD>(&mut stdout(), buf, stride, w, h, what).unwrap();
}

#[inline]
pub fn coef_dump<Coef: Display>(buf: &[Coef], w: usize, h: usize, len: usize, what: &str) {
    println!("{}", what);
    for row in buf[..w * h].chunks_exact(w).take(h) {
        for coef in row {
            print!(" {:0len$}", coef, len = len);
        }
        println!();
    }
}

#[inline]
pub fn ac_dump(buf: &[i16; 32 * 32], w: usize, h: usize, what: &str) {
    println!("{}", what);
    for buf in buf.chunks_exact(w).take(h) {
        for x in buf {
            print!(" {:03}", x);
        }
        println!();
    }
}
