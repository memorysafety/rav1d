use crate::include::common::attributes::clz;
use crate::include::common::intops::inv_recenter;
use crate::include::common::intops::ulog2;
use crate::include::stddef::*;
use crate::include::stdint::*;
use cfg_if::cfg_if;
use std::mem;
use std::ops::Range;

#[cfg(all(feature = "asm", target_arch = "x86_64"))]
extern "C" {
    fn dav1d_msac_decode_hi_tok_sse2(s: *mut MsacContext, cdf: *mut uint16_t) -> libc::c_uint;
    fn dav1d_msac_decode_bool_sse2(s: *mut MsacContext, f: libc::c_uint) -> libc::c_uint;
    fn dav1d_msac_decode_bool_equi_sse2(s: *mut MsacContext) -> libc::c_uint;
    fn dav1d_msac_decode_bool_adapt_sse2(s: *mut MsacContext, cdf: *mut uint16_t) -> libc::c_uint;
    fn dav1d_msac_decode_symbol_adapt16_avx2(
        s: *mut MsacContext,
        cdf: *mut uint16_t,
        n_symbols: size_t,
    ) -> libc::c_uint;
    fn dav1d_msac_decode_symbol_adapt16_sse2(
        s: *mut MsacContext,
        cdf: *mut uint16_t,
        n_symbols: size_t,
    ) -> libc::c_uint;
    fn dav1d_msac_decode_symbol_adapt8_sse2(
        s: *mut MsacContext,
        cdf: *mut uint16_t,
        n_symbols: size_t,
    ) -> libc::c_uint;
    fn dav1d_msac_decode_symbol_adapt4_sse2(
        s: *mut MsacContext,
        cdf: *mut uint16_t,
        n_symbols: size_t,
    ) -> libc::c_uint;
    static mut dav1d_cpu_flags_mask: libc::c_uint;
    static mut dav1d_cpu_flags: libc::c_uint;
}

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
extern "C" {
    fn dav1d_msac_decode_hi_tok_neon(s: *mut MsacContext, cdf: *mut uint16_t) -> libc::c_uint;
    fn dav1d_msac_decode_bool_neon(s: *mut MsacContext, f: libc::c_uint) -> libc::c_uint;
    fn dav1d_msac_decode_bool_equi_neon(s: *mut MsacContext) -> libc::c_uint;
    fn dav1d_msac_decode_bool_adapt_neon(s: *mut MsacContext, cdf: *mut uint16_t) -> libc::c_uint;
    fn dav1d_msac_decode_symbol_adapt16_neon(
        s: *mut MsacContext,
        cdf: *mut uint16_t,
        n_symbols: size_t,
    ) -> libc::c_uint;
    fn dav1d_msac_decode_symbol_adapt8_neon(
        s: *mut MsacContext,
        cdf: *mut uint16_t,
        n_symbols: size_t,
    ) -> libc::c_uint;
    fn dav1d_msac_decode_symbol_adapt4_neon(
        s: *mut MsacContext,
        cdf: *mut uint16_t,
        n_symbols: size_t,
    ) -> libc::c_uint;
    static mut dav1d_cpu_flags_mask: libc::c_uint;
    static mut dav1d_cpu_flags: libc::c_uint;
}

pub type ec_win = size_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MsacContext {
    buf_pos: *const uint8_t,
    buf_end: *const uint8_t,
    pub dif: ec_win,
    pub rng: libc::c_uint,
    pub cnt: libc::c_int,
    allow_update_cdf: libc::c_int,
    #[cfg(all(feature = "asm", target_arch = "x86_64"))]
    pub symbol_adapt16:
        Option<unsafe extern "C" fn(*mut MsacContext, *mut uint16_t, size_t) -> libc::c_uint>,
}

impl MsacContext {
    fn set_buf(&mut self, buf: &[u8]) {
        let Range { start, end } = buf.as_ptr_range();
        self.buf_pos = start;
        self.buf_end = end;
    }

    fn with_buf(&mut self, mut f: impl FnMut(&[u8]) -> &[u8]) {
        // # Safety
        //
        // [`Self::buf_pos`] and [`Self::buf_end`] are the start and end ptrs of the `buf` slice,
        // and are only set in [`Self::set_buf`], which derives them from a valid slice.
        let buf = unsafe {
            let len = self.buf_end.offset_from(self.buf_pos) as usize;
            std::slice::from_raw_parts(self.buf_pos, len)
        };
        self.set_buf(f(buf));
    }

    fn allow_update_cdf(&self) -> bool {
        self.allow_update_cdf != 0
    }

    fn set_allow_update_cdf(&mut self, val: bool) {
        self.allow_update_cdf = val.into()
    }
}

#[inline]
pub fn dav1d_msac_decode_bools(s: &mut MsacContext, n: libc::c_uint) -> libc::c_uint {
    let mut v = 0;
    for _ in 0..n {
        v = v << 1 | dav1d_msac_decode_bool_equi(s);
    }
    v
}

#[inline]
pub fn dav1d_msac_decode_uniform(s: &mut MsacContext, n: libc::c_uint) -> libc::c_int {
    assert!(n > 0);
    let l = ulog2(n) as libc::c_uint + 1;
    assert!(l > 1);
    let m = (1 << l) - n;
    let v = dav1d_msac_decode_bools(s, l - 1);
    (if v < m {
        v
    } else {
        (v << 1) - m + dav1d_msac_decode_bool_equi(s)
    }) as libc::c_int
}

#[cfg(all(feature = "asm", target_arch = "x86_64"))]
#[inline(always)]
unsafe fn msac_init_x86(s: &mut MsacContext) {
    use crate::src::cpu::dav1d_get_cpu_flags;
    use crate::src::x86::cpu::DAV1D_X86_CPU_FLAG_AVX2;
    use crate::src::x86::cpu::DAV1D_X86_CPU_FLAG_SSE2;

    let flags = dav1d_get_cpu_flags();
    if flags & DAV1D_X86_CPU_FLAG_SSE2 != 0 {
        s.symbol_adapt16 = Some(dav1d_msac_decode_symbol_adapt16_sse2);
    }
    if flags & DAV1D_X86_CPU_FLAG_AVX2 != 0 {
        s.symbol_adapt16 = Some(dav1d_msac_decode_symbol_adapt16_avx2);
    }
}

const EC_PROB_SHIFT: libc::c_uint = 6;
const EC_MIN_PROB: libc::c_uint = 4; // must be <= (1 << EC_PROB_SHIFT) / 16

const EC_WIN_SIZE: usize = mem::size_of::<ec_win>() << 3;

#[inline]
fn ctx_refill(s: &mut MsacContext) {
    let mut c = (EC_WIN_SIZE as libc::c_int) - 24 - s.cnt;
    let mut dif = s.dif;
    s.with_buf(|mut buf| {
        while c >= 0 && !buf.is_empty() {
            dif ^= (buf[0] as ec_win) << c;
            buf = &buf[1..];
            c -= 8;
        }
        buf
    });
    s.dif = dif;
    s.cnt = (EC_WIN_SIZE as libc::c_int) - 24 - c;
}

#[inline]
fn ctx_norm(s: &mut MsacContext, dif: ec_win, rng: libc::c_uint) {
    let d = 15 ^ (31 ^ clz(rng));
    assert!(rng <= 65535);
    s.cnt -= d;
    s.dif = ((dif + 1) << d) - 1;
    s.rng = rng << d;
    if s.cnt < 0 {
        ctx_refill(s);
    }
}

fn dav1d_msac_decode_bool_equi_rust(s: &mut MsacContext) -> libc::c_uint {
    let r = s.rng;
    let mut dif = s.dif;
    assert!(dif >> (EC_WIN_SIZE - 16) < r as ec_win);
    let mut v = (r >> 8 << 7) + EC_MIN_PROB;
    let vw = (v as ec_win) << (EC_WIN_SIZE - 16);
    let ret = dif >= vw;
    dif -= (ret as ec_win) * vw;
    v = v.wrapping_add((ret as libc::c_uint) * (r.wrapping_sub(2 * v)));
    ctx_norm(s, dif, v);
    !ret as libc::c_uint
}

fn dav1d_msac_decode_bool_rust(s: &mut MsacContext, f: libc::c_uint) -> libc::c_uint {
    let r = s.rng;
    let mut dif = s.dif;
    assert!(dif >> (EC_WIN_SIZE - 16) < r as ec_win);
    let mut v = ((r >> 8) * (f >> EC_PROB_SHIFT) >> (7 - EC_PROB_SHIFT)) + EC_MIN_PROB;
    let vw = (v as ec_win) << (EC_WIN_SIZE - 16);
    let ret = dif >= vw;
    dif -= (ret as ec_win) * vw;
    v = v.wrapping_add((ret as libc::c_uint) * (r.wrapping_sub(2 * v)));
    ctx_norm(s, dif, v);
    !ret as libc::c_uint
}

pub fn dav1d_msac_decode_subexp(
    s: &mut MsacContext,
    r#ref: libc::c_uint,
    n: libc::c_uint,
    mut k: libc::c_uint,
) -> libc::c_int {
    assert!(n >> k == 8);
    let mut a = 0;
    if dav1d_msac_decode_bool_equi(s) != 0 {
        if dav1d_msac_decode_bool_equi(s) != 0 {
            k += dav1d_msac_decode_bool_equi(s) + 1;
        }
        a = 1 << k;
    }
    let v = dav1d_msac_decode_bools(s, k) + a;
    (if r#ref * 2 <= n {
        inv_recenter(r#ref, v)
    } else {
        n - 1 - inv_recenter(n - 1 - r#ref, v)
    }) as libc::c_int
}

fn dav1d_msac_decode_symbol_adapt_rust(
    s: &mut MsacContext,
    cdf: &mut [u16],
    n_symbols: size_t,
) -> libc::c_uint {
    let c = (s.dif >> (EC_WIN_SIZE - 16)) as libc::c_uint;
    let r = s.rng >> 8;
    let mut u = 0;
    let mut v = s.rng;
    let mut val = 0;
    assert!(n_symbols <= 15);
    assert!(cdf[n_symbols] <= 32);
    loop {
        u = v;
        v = r * ((cdf[val as usize] >> EC_PROB_SHIFT) as libc::c_uint);
        v >>= 7 - EC_PROB_SHIFT;
        v += EC_MIN_PROB * ((n_symbols as libc::c_uint) - val);
        if !(c < v) {
            break;
        }
        val += 1;
    }
    assert!(u <= s.rng);
    ctx_norm(
        s,
        s.dif.wrapping_sub((v as ec_win) << (EC_WIN_SIZE - 16)),
        u - v,
    );
    if s.allow_update_cdf() {
        let count = cdf[n_symbols];
        let rate = 4 + (count >> 4) + (n_symbols > 2) as u16;
        let val = val as usize;
        for cdf in &mut cdf[..val] {
            *cdf += (1 << 15) - *cdf >> rate;
        }
        for cdf in &mut cdf[val..n_symbols] {
            *cdf -= *cdf >> rate;
        }
        cdf[n_symbols] = count + (count < 32) as u16;
    }
    val
}

unsafe extern "C" fn dav1d_msac_decode_symbol_adapt_c(
    s: *mut MsacContext,
    cdf: *mut u16,
    n_symbols: size_t,
) -> libc::c_uint {
    // # Safety
    //
    // This is only called from [`dav1d_msac_decode_symbol_adapt16`],
    // where there is an `assert!(n_symbols < cdf.len());`.
    // Thus, `n_symbols + 1` is a valid length for the slice `cdf` came from.
    #[deny(unsafe_op_in_unsafe_fn)]
    let cdf = unsafe { std::slice::from_raw_parts_mut(cdf, n_symbols + 1) };

    dav1d_msac_decode_symbol_adapt_rust(&mut *s, cdf, n_symbols)
}

fn dav1d_msac_decode_bool_adapt_rust(s: &mut MsacContext, cdf: &mut [u16; 2]) -> libc::c_uint {
    let bit = dav1d_msac_decode_bool(s, cdf[0] as libc::c_uint);
    if s.allow_update_cdf() {
        let count = cdf[1];
        let rate = 4 + (count >> 4);
        if bit != 0 {
            cdf[0] += (1 << 15) - cdf[0] >> rate;
        } else {
            cdf[0] -= cdf[0] >> rate;
        }
        cdf[1] = count + (count < 32) as u16;
    }
    bit
}

fn dav1d_msac_decode_hi_tok_rust(s: &mut MsacContext, cdf: &mut [u16; 4]) -> libc::c_uint {
    let mut tok_br = dav1d_msac_decode_symbol_adapt4(s, cdf, 3);
    let mut tok = 3 + tok_br;
    if tok_br == 3 {
        tok_br = dav1d_msac_decode_symbol_adapt4(s, cdf, 3);
        tok = 6 + tok_br;
        if tok_br == 3 {
            tok_br = dav1d_msac_decode_symbol_adapt4(s, cdf, 3);
            tok = 9 + tok_br;
            if tok_br == 3 {
                tok = 12 + dav1d_msac_decode_symbol_adapt4(s, cdf, 3);
            }
        }
    }
    tok
}

/// # Safety
///
/// `data` and `sz` must form a valid slice,
/// and must live longer than all of the other functions called on [`MsacContext`].
pub unsafe fn dav1d_msac_init(
    s: &mut MsacContext,
    data: *const uint8_t,
    sz: size_t,
    disable_cdf_update_flag: bool,
) {
    s.set_buf(std::slice::from_raw_parts(data, sz));
    s.dif = (1 << (EC_WIN_SIZE - 1)) - 1;
    s.rng = 0x8000;
    s.cnt = -15;
    s.set_allow_update_cdf(!disable_cdf_update_flag);
    ctx_refill(s);

    #[cfg(all(feature = "asm", target_arch = "x86_64"))]
    {
        s.symbol_adapt16 = Some(dav1d_msac_decode_symbol_adapt_c);
        msac_init_x86(s);
    }
}

pub fn dav1d_msac_decode_symbol_adapt4(
    s: &mut MsacContext,
    cdf: &mut [u16],
    n_symbols: size_t,
) -> libc::c_uint {
    cfg_if! {
        if #[cfg(all(feature = "asm", target_arch = "x86_64"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_symbol_adapt_rust`].
            unsafe {
                dav1d_msac_decode_symbol_adapt4_sse2(s, cdf.as_mut_ptr(), n_symbols)
            }
        } else if #[cfg(all(feature = "asm", target_arch = "aarch64"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_symbol_adapt_rust`].
            unsafe {
                dav1d_msac_decode_symbol_adapt4_neon(s, cdf.as_mut_ptr(), n_symbols)
            }
        } else {
            dav1d_msac_decode_symbol_adapt_rust(s, cdf, n_symbols)
        }
    }
}

pub fn dav1d_msac_decode_symbol_adapt8(
    s: &mut MsacContext,
    cdf: &mut [u16],
    n_symbols: size_t,
) -> libc::c_uint {
    cfg_if! {
        if #[cfg(all(feature = "asm", target_arch = "x86_64"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_symbol_adapt_rust`].
            unsafe {
                dav1d_msac_decode_symbol_adapt8_sse2(s, cdf.as_mut_ptr(), n_symbols)
            }
        } else if #[cfg(all(feature = "asm", target_arch = "aarch64"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_symbol_adapt_rust`].
            unsafe {
                dav1d_msac_decode_symbol_adapt8_neon(s, cdf.as_mut_ptr(), n_symbols)
            }
        } else {
             dav1d_msac_decode_symbol_adapt_rust(s, cdf, n_symbols)
        }
    }
}

pub fn dav1d_msac_decode_symbol_adapt16(
    s: &mut MsacContext,
    cdf: &mut [u16],
    n_symbols: size_t,
) -> libc::c_uint {
    cfg_if! {
        if #[cfg(all(feature = "asm", target_arch = "x86_64"))] {
            assert!(n_symbols < cdf.len());
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_symbol_adapt_rust`].
            unsafe {
                (s.symbol_adapt16).expect("non-null function pointer")(s, cdf.as_mut_ptr(), n_symbols)
            }
        } else if #[cfg(all(feature = "asm", target_arch = "aarch64"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_symbol_adapt_rust`].
            unsafe {
                dav1d_msac_decode_symbol_adapt16_neon(s, cdf.as_mut_ptr(), n_symbols)
            }
        } else {
            dav1d_msac_decode_symbol_adapt_rust(s, cdf, n_symbols)
        }
    }
}

pub fn dav1d_msac_decode_bool_adapt(s: &mut MsacContext, cdf: &mut [u16; 2]) -> libc::c_uint {
    cfg_if! {
        if #[cfg(all(feature = "asm", target_arch = "x86_64"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_bool_adapt_rust`].
            unsafe {
                dav1d_msac_decode_bool_adapt_sse2(s, cdf.as_mut_ptr())
            }
        } else if #[cfg(all(feature = "asm", target_arch = "aarch64"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_bool_adapt_rust`].
            unsafe {
                dav1d_msac_decode_bool_adapt_neon(s, cdf.as_mut_ptr())
            }
        } else {
            dav1d_msac_decode_bool_adapt_rust(s, cdf)
        }
    }
}

pub fn dav1d_msac_decode_bool_equi(s: &mut MsacContext) -> libc::c_uint {
    cfg_if! {
        if #[cfg(all(feature = "asm", target_arch = "x86_64"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_bool_equi_rust`].
            unsafe {
                dav1d_msac_decode_bool_equi_sse2(s)
            }
        } else if #[cfg(all(feature = "asm", target_arch = "aarch64"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_bool_equi_rust`].
            unsafe {
                dav1d_msac_decode_bool_equi_neon(s)
            }
        } else {
            dav1d_msac_decode_bool_equi_rust(s)
        }
    }
}

pub fn dav1d_msac_decode_bool(s: &mut MsacContext, f: libc::c_uint) -> libc::c_uint {
    cfg_if! {
        if #[cfg(all(feature = "asm", target_arch = "x86_64"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_bool_rust`].
            unsafe {
                dav1d_msac_decode_bool_sse2(s, f)
            }
        } else if #[cfg(all(feature = "asm", target_arch = "aarch64"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_bool_rust`].
            unsafe {
                dav1d_msac_decode_bool_neon(s, f)
            }
        } else {
            dav1d_msac_decode_bool_rust(s, f)
        }
    }
}

pub fn dav1d_msac_decode_hi_tok(s: &mut MsacContext, cdf: &mut [u16; 4]) -> libc::c_uint {
    cfg_if! {
        if #[cfg(all(feature = "asm", target_arch = "x86_64"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_hi_tok_rust`].
            unsafe {
                dav1d_msac_decode_hi_tok_sse2(s, cdf.as_mut_ptr())
            }
        } else if #[cfg(all(feature = "asm", target_arch = "aarch64"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_hi_tok_rust`].
            unsafe {
                dav1d_msac_decode_hi_tok_neon(s, cdf.as_mut_ptr())
            }
        } else {
            dav1d_msac_decode_hi_tok_rust(s, cdf)
        }
    }
}
