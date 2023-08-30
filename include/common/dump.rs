use std::fmt::Display;
use std::io;
use std::io::stdout;

use crate::include::common::bitdepth::BitDepth;

#[inline]
pub unsafe fn hex_fdump<BD: BitDepth>(
    out: &mut impl io::Write,
    buf: *const BD::Pixel,
    stride: usize,
    w: usize,
    h: usize,
    what: &str,
) {
    let len = if h == 0 {
        0
    } else {
        (h - 1) * BD::pxstride(stride) + w
    };
    let buf = std::slice::from_raw_parts(buf, len);

    write!(out, "{}", what).unwrap();
    for buf in buf.chunks(BD::pxstride(stride)) {
        for &x in &buf[..w] {
            write!(out, " {}", BD::display(x)).unwrap();
        }
        writeln!(out).unwrap();
    }
}

#[inline]
pub unsafe fn hex_dump<BD: BitDepth>(
    buf: *const BD::Pixel,
    stride: usize,
    w: usize,
    h: usize,
    what: &str,
) {
    hex_fdump::<BD>(&mut stdout(), buf, stride, w, h, what);
}

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
pub fn ac_dump(buf: &[i16; 32 * 32], w: usize, h: usize, what: &str) {
    println!("{}", what);
    for buf in buf.chunks_exact(w).take(h) {
        for x in buf {
            print!(" {:03}", x);
        }
        println!();
    }
}
