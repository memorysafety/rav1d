use crate::include::stdint::uint16_t;
use crate::include::stdint::uint32_t;
use crate::include::stdint::uint64_t;
use crate::include::stdint::uint8_t;

pub trait Alias {
    fn set(&mut self, val: u64);
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union alias8 {
    pub u8_0: uint8_t,
}

impl Alias for alias8 {
    fn set(&mut self, val: u64) {
        self.u8_0 = val as u8;
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union alias16 {
    pub u16_0: uint16_t,
    pub u8_0: [uint8_t; 2],
}

impl Alias for alias16 {
    fn set(&mut self, val: u64) {
        self.u16_0 = val as u16;
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union alias32 {
    pub u32_0: uint32_t,
    pub u8_0: [uint8_t; 4],
}

impl Alias for alias32 {
    fn set(&mut self, val: u64) {
        self.u32_0 = val as u32;
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union alias64 {
    pub u64_0: uint64_t,
    pub u8_0: [uint8_t; 8],
}

impl Alias for alias64 {
    fn set(&mut self, val: u64) {
        self.u64_0 = val;
    }
}

#[inline]
pub unsafe fn set_ctx_rep1<T: Alias>(buf: *mut u8, off: isize, val: u64) {
    let buf = buf.offset(off);
    let buf = buf.cast::<T>();
    (*buf).set(val);
}

#[inline]
pub unsafe fn set_ctx_rep2(buf: *mut u8, off: isize, val: u64) {
    set_ctx_rep1::<alias64>(buf, off + 0, val);
    set_ctx_rep1::<alias64>(buf, off + 8, val);
}

#[inline]
pub unsafe fn set_ctx_rep4(buf: *mut u8, off: isize, val: u64) {
    set_ctx_rep1::<alias64>(buf, off + 0, val);
    set_ctx_rep1::<alias64>(buf, off + 8, val);
    set_ctx_rep1::<alias64>(buf, off + 16, val);
    set_ctx_rep1::<alias64>(buf, off + 24, val);
}

pub type SetCtxFn = unsafe fn(*mut u8, isize, u64);

#[inline]
pub unsafe fn case_set<D, F>(
    var: libc::c_int,
    dir: &mut D,
    diridx: usize,
    off: isize,
    set_ctx: &mut F,
) where
    F: FnMut(&mut D, usize, isize, u64, SetCtxFn),
{
    match var {
        1 => set_ctx(dir, diridx, off, 0x01, set_ctx_rep1::<alias8>),
        2 => set_ctx(dir, diridx, off, 0x0101, set_ctx_rep1::<alias16>),
        4 => set_ctx(dir, diridx, off, 0x01010101, set_ctx_rep1::<alias32>),
        8 => set_ctx(
            dir,
            diridx,
            off,
            0x0101010101010101,
            set_ctx_rep1::<alias64>,
        ),
        16 => set_ctx(dir, diridx, off, 0x0101010101010101, set_ctx_rep2),
        32 => set_ctx(dir, diridx, off, 0x0101010101010101, set_ctx_rep4),

        _ => {}
    }
}

#[inline]
pub unsafe fn case_set_upto16<D, F>(
    var: libc::c_int,
    dir: &mut D,
    diridx: usize,
    off: isize,
    set_ctx: &mut F,
) where
    F: FnMut(&mut D, usize, isize, u64, SetCtxFn),
{
    match var {
        1 => set_ctx(dir, diridx, off, 0x01, set_ctx_rep1::<alias8>),
        2 => set_ctx(dir, diridx, off, 0x0101, set_ctx_rep1::<alias16>),
        4 => set_ctx(dir, diridx, off, 0x01010101, set_ctx_rep1::<alias32>),
        8 => set_ctx(
            dir,
            diridx,
            off,
            0x0101010101010101,
            set_ctx_rep1::<alias64>,
        ),
        16 => set_ctx(dir, diridx, off, 0x0101010101010101, set_ctx_rep2),

        _ => {}
    }
}

#[inline]
pub unsafe fn case_set_upto32_with_default<D, F, G>(
    var: libc::c_int,
    dir: &mut D,
    diridx: usize,
    off: isize,
    set_ctx: &mut F,
    mut default_memset: G,
) where
    F: FnMut(&mut D, usize, isize, u64, SetCtxFn),
    G: FnMut(&mut D, usize, isize, libc::c_int),
    D: ?Sized,
{
    match var {
        1 => set_ctx(dir, diridx, off, 0x01, set_ctx_rep1::<alias8>),
        2 => set_ctx(dir, diridx, off, 0x0101, set_ctx_rep1::<alias16>),
        4 => set_ctx(dir, diridx, off, 0x01010101, set_ctx_rep1::<alias32>),
        8 => set_ctx(
            dir,
            diridx,
            off,
            0x0101010101010101,
            set_ctx_rep1::<alias64>,
        ),
        16 => set_ctx(dir, diridx, off, 0x0101010101010101, set_ctx_rep2),
        32 => set_ctx(dir, diridx, off, 0x0101010101010101, set_ctx_rep4),

        _ => default_memset(dir, diridx, off, var),
    }
}

#[inline]
pub unsafe fn case_set_upto16_with_default<D, F, G>(
    var: libc::c_int,
    dir: &mut D,
    diridx: usize,
    off: isize,
    set_ctx: &mut F,
    mut default_memset: G,
) where
    F: FnMut(&mut D, usize, isize, u64, SetCtxFn),
    G: FnMut(&mut D, usize, isize, libc::c_int),
    D: ?Sized,
{
    match var {
        1 => set_ctx(dir, diridx, off, 0x01, set_ctx_rep1::<alias8>),
        2 => set_ctx(dir, diridx, off, 0x0101, set_ctx_rep1::<alias16>),
        4 => set_ctx(dir, diridx, off, 0x01010101, set_ctx_rep1::<alias32>),
        8 => set_ctx(
            dir,
            diridx,
            off,
            0x0101010101010101,
            set_ctx_rep1::<alias64>,
        ),
        16 => set_ctx(dir, diridx, off, 0x0101010101010101, set_ctx_rep2),

        _ => default_memset(dir, diridx, off, var),
    }
}
