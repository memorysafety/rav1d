#![deny(unsafe_code)]

use crate::include::common::bitdepth::BitDepth;
use crate::include::dav1d::picture::Rav1dPictureDataComponentOffset;
use crate::src::strided::Strided as _;
use std::fmt::Display;
use std::io;
use std::io::stdout;

#[inline]
pub fn hex_fdump<BD: BitDepth>(
    out: &mut impl io::Write,
    buf: &[BD::Pixel],
    stride: usize,
    w: usize,
    h: usize,
    what: &str,
) -> io::Result<()> {
    write!(out, "{}", what)?;
    for y in 0..h {
        let buf = &buf[y * stride..][..w];
        for &x in buf {
            write!(out, " {}", BD::display(x))?;
        }
        writeln!(out)?;
    }
    Ok(())
}

#[inline]
pub fn hex_dump<BD: BitDepth>(buf: &[BD::Pixel], stride: usize, w: usize, h: usize, what: &str) {
    hex_fdump::<BD>(&mut stdout(), buf, stride, w, h, what).unwrap();
}

#[inline]
pub fn hex_fdump_pic<BD: BitDepth>(
    out: &mut impl io::Write,
    buf: Rav1dPictureDataComponentOffset,
    w: usize,
    h: usize,
    what: &str,
) -> io::Result<()> {
    write!(out, "{}", what)?;
    for y in 0..h {
        let buf = buf + (y as isize * buf.pixel_stride::<BD>());
        let buf = &*buf.slice::<BD>(w);
        for &x in buf {
            write!(out, " {}", BD::display(x))?;
        }
        writeln!(out)?;
    }
    Ok(())
}

#[inline]
pub fn hex_dump_pic<BD: BitDepth>(
    buf: Rav1dPictureDataComponentOffset,
    w: usize,
    h: usize,
    what: &str,
) {
    hex_fdump_pic::<BD>(&mut stdout(), buf, w, h, what).unwrap();
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
