use std::ffi::c_int;

use crate::include::stdint::uint16_t;
use crate::include::stdint::uint32_t;
use crate::include::stdint::uint64_t;
use crate::include::stdint::uint8_t;
use crate::src::align::{Align1, Align16, Align2, Align32, Align4, Align8};

#[derive(Copy, Clone)]
#[repr(C)]
pub union alias8 {
    pub u8_0: uint8_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union alias16 {
    pub u16_0: uint16_t,
    pub u8_0: [uint8_t; 2],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union alias32 {
    pub u32_0: uint32_t,
    pub u8_0: [uint8_t; 4],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union alias64 {
    pub u64_0: uint64_t,
    pub u8_0: [uint8_t; 8],
}

pub struct CaseSetter {
    align: u8,
    offset: u32,
}

impl CaseSetter {
    unsafe fn set_impl(&self, buf: *mut u8, multiplier: u8) {
        let buf = buf.offset(self.offset as isize);
        debug_assert!(buf as usize & ((self.align as usize) - 1) == 0);
        // This compiles to more efficient assembly than the dav1d C approach naively ported to Rust.
        // For example, this way allows Rust to optimize better
        // and use 16-byte moves instead of always 8-byte moves at most
        // (or whatever is best for the architecture).
        // The array val repeats are compiled identically,
        // so the single, repeated `0x01` mask here is a lot simpler.
        // If performance isn't the most important; it's a lot simpler code, too.
        let val = 0x01 * multiplier;
        match self.align {
            1 => *buf.cast() = Align1([val; 1]),
            2 => *buf.cast() = Align2([val; 2]),
            4 => *buf.cast() = Align4([val; 4]),
            8 => *buf.cast() = Align8([val; 8]),
            16 => *buf.cast() = Align16([val; 16]),
            32 => *buf.cast() = Align32([val; 32]),
            _ => {}
        };
    }

    pub unsafe fn set(&self, buf: &mut [u8], multiplier: impl TryInto<u8>) {
        self.set_impl(
            buf.as_mut_ptr(),
            multiplier.try_into().unwrap_unchecked(), // a generic `as` cast
        );
    }
}

pub unsafe fn case_set<T, F>(align: c_int, offset: c_int, dir: &mut T, set_ctx: F)
where
    F: Fn(&CaseSetter, &mut T),
{
    let case = CaseSetter {
        align: align as u8,
        offset: offset as u32,
    };
    set_ctx(&case, dir);
}

pub unsafe fn case_set_many<const N: usize, T, F>(
    aligns: [c_int; N],
    offsets: [c_int; N],
    dirs: [&mut T; N],
    set_ctx: F,
) where
    F: Fn(&CaseSetter, &mut T) + Copy,
{
    for i in 0..N {
        case_set(aligns[i], offsets[i], dirs[i], set_ctx);
    }
}
