use crate::include::common::attributes::clz;
use crate::include::common::intops::inv_recenter;
use crate::include::common::intops::ulog2;
use crate::include::stddef::*;
use crate::include::stdint::*;
use cfg_if::cfg_if;
use std::mem;

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
    pub buf_pos: *const uint8_t,
    pub buf_end: *const uint8_t,
    pub dif: ec_win,
    pub rng: libc::c_uint,
    pub cnt: libc::c_int,
    allow_update_cdf: libc::c_int,
    #[cfg(all(feature = "asm", target_arch = "x86_64"))]
    pub symbol_adapt16:
        Option<unsafe extern "C" fn(*mut MsacContext, *mut uint16_t, size_t) -> libc::c_uint>,
}

impl MsacContext {
    fn allow_update_cdf(&self) -> bool {
        self.allow_update_cdf != 0
    }

    fn set_allow_update_cdf(&mut self, val: bool) {
        self.allow_update_cdf = val.into()
    }
}

#[inline]
pub unsafe fn dav1d_msac_decode_bools(s: &mut MsacContext, n: libc::c_uint) -> libc::c_uint {
    let mut v = 0;
    for _ in 0..n {
        v = v << 1 | dav1d_msac_decode_bool_equi(s);
    }
    v
}

#[inline]
pub unsafe fn dav1d_msac_decode_uniform(s: &mut MsacContext, n: libc::c_uint) -> libc::c_int {
    assert!(n > 0);
    let l = ulog2(n) + 1;
    assert!(l > 1);
    let m = ((1 << l) as libc::c_uint).wrapping_sub(n);
    let v = dav1d_msac_decode_bools(s, (l - 1) as libc::c_uint);
    return (if v < m {
        v
    } else {
        (v << 1)
            .wrapping_sub(m)
            .wrapping_add(dav1d_msac_decode_bool_equi(s))
    }) as libc::c_int;
}

#[cfg(all(feature = "asm", target_arch = "x86_64"))]
#[inline(always)]
unsafe extern "C" fn msac_init_x86(s: &mut MsacContext) {
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

#[cfg(all(feature = "asm", target_arch = "x86_64"))]
use crate::src::cpu::dav1d_get_cpu_flags;

const EC_PROB_SHIFT: libc::c_uint = 6;
const EC_MIN_PROB: libc::c_uint = 4; // must be <= (1 << EC_PROB_SHIFT) / 16

const EC_WIN_SIZE: usize = mem::size_of::<ec_win>() << 3;

#[inline]
unsafe extern "C" fn ctx_refill(s: &mut MsacContext) {
    let mut buf_pos = s.buf_pos;
    let mut buf_end = s.buf_end;
    let mut c = (EC_WIN_SIZE as libc::c_int) - 24 - s.cnt;
    let mut dif = s.dif;
    while c >= 0 && buf_pos < buf_end {
        let fresh1 = buf_pos;
        buf_pos = buf_pos.offset(1);
        dif ^= (*fresh1 as ec_win) << c;
        c -= 8;
    }
    s.dif = dif;
    s.cnt = (EC_WIN_SIZE as libc::c_int) - 24 - c;
    s.buf_pos = buf_pos;
}

#[inline]
unsafe extern "C" fn ctx_norm(s: &mut MsacContext, dif: ec_win, rng: libc::c_uint) {
    let d = 15 ^ (31 ^ clz(rng));
    assert!(rng <= 65535);
    s.cnt -= d;
    s.dif = (dif.wrapping_add(1) << d).wrapping_sub(1);
    s.rng = rng << d;
    if s.cnt < 0 {
        ctx_refill(s);
    }
}

unsafe fn dav1d_msac_decode_bool_equi_rust(s: &mut MsacContext) -> libc::c_uint {
    let r = s.rng;
    let mut dif = s.dif;
    assert!(dif >> (EC_WIN_SIZE - 16) < r as size_t);
    let mut v = ((r >> 8) << 7).wrapping_add(EC_MIN_PROB);
    let vw = (v as ec_win) << (EC_WIN_SIZE - 16);
    let ret = dif >= vw;
    dif = dif.wrapping_sub((ret as size_t).wrapping_mul(vw)) as ec_win;
    v = v.wrapping_add(
        (ret as libc::c_uint).wrapping_mul(r.wrapping_sub((2 as libc::c_uint).wrapping_mul(v))),
    );
    ctx_norm(s, dif, v);
    !ret as libc::c_uint
}

unsafe fn dav1d_msac_decode_bool_rust(s: &mut MsacContext, f: libc::c_uint) -> libc::c_uint {
    let r = s.rng;
    let mut dif = s.dif;
    assert!(dif >> (EC_WIN_SIZE - 16) < r as size_t);
    let mut v = ((r >> 8).wrapping_mul(f >> EC_PROB_SHIFT) >> (7 - EC_PROB_SHIFT))
        .wrapping_add(EC_MIN_PROB);
    let vw = (v as ec_win) << (EC_WIN_SIZE - 16);
    let ret = dif >= vw;
    dif = (dif).wrapping_sub((ret as size_t).wrapping_mul(vw)) as ec_win;
    v = v.wrapping_add(
        (ret as libc::c_uint).wrapping_mul(r.wrapping_sub((2 as libc::c_uint).wrapping_mul(v))),
    );
    ctx_norm(s, dif, v);
    !ret as libc::c_uint
}

pub unsafe fn dav1d_msac_decode_subexp(
    s: &mut MsacContext,
    r#ref: libc::c_int,
    n: libc::c_int,
    mut k: libc::c_uint,
) -> libc::c_int {
    assert!(n >> k == 8);
    let mut a = 0;
    if dav1d_msac_decode_bool_equi(s) != 0 {
        if dav1d_msac_decode_bool_equi(s) != 0 {
            k = k.wrapping_add((dav1d_msac_decode_bool_equi(s)).wrapping_add(1));
        }
        a = 1 << k;
    }
    let v = (dav1d_msac_decode_bools(s, k)).wrapping_add(a);
    (if r#ref * 2 <= n {
        inv_recenter(r#ref as libc::c_uint, v)
    } else {
        ((n - 1) as libc::c_uint).wrapping_sub(inv_recenter((n - 1 - r#ref) as libc::c_uint, v))
    }) as libc::c_int
}

unsafe fn dav1d_msac_decode_symbol_adapt_rust(
    s: &mut MsacContext,
    cdf: &mut [u16],
    n_symbols: size_t,
) -> libc::c_uint {
    let c = (s.dif >> (EC_WIN_SIZE - 16)) as libc::c_uint;
    let r = s.rng >> 8;
    let mut u = 0;
    let mut v = s.rng;
    let mut val = -(1 as libc::c_int) as libc::c_uint;
    assert!(n_symbols <= 15);
    assert!(cdf[n_symbols] <= 32);
    loop {
        val = val.wrapping_add(1);
        u = v;
        v = r.wrapping_mul((cdf[val as usize] >> EC_PROB_SHIFT) as libc::c_uint);
        v >>= 7 - EC_PROB_SHIFT;
        v = v.wrapping_add(EC_MIN_PROB.wrapping_mul((n_symbols as libc::c_uint).wrapping_sub(val)));
        if !(c < v) {
            break;
        }
    }
    assert!(u <= s.rng);
    ctx_norm(
        s,
        s.dif.wrapping_sub((v as ec_win) << (EC_WIN_SIZE - 16)),
        u.wrapping_sub(v),
    );
    if s.allow_update_cdf() {
        let count = cdf[n_symbols];
        let rate = 4 + (count >> 4) + (n_symbols > 2) as u16;
        for i in 0..val {
            cdf[i as usize] += (1 << 15) - cdf[i as usize] >> rate;
        }
        for i in val..n_symbols as libc::c_uint {
            cdf[i as usize] -= cdf[i as usize] >> rate;
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

unsafe fn dav1d_msac_decode_bool_adapt_rust(
    s: &mut MsacContext,
    cdf: &mut [u16; 2],
) -> libc::c_uint {
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

unsafe fn dav1d_msac_decode_hi_tok_rust(s: &mut MsacContext, cdf: &mut [u16; 4]) -> libc::c_uint {
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

pub unsafe fn dav1d_msac_init(
    s: &mut MsacContext,
    data: *const uint8_t,
    sz: size_t,
    disable_cdf_update_flag: bool,
) {
    s.buf_pos = data;
    s.buf_end = data.offset(sz as isize);
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

pub unsafe fn dav1d_msac_decode_symbol_adapt4(
    s: &mut MsacContext,
    cdf: &mut [u16],
    n_symbols: size_t,
) -> libc::c_uint {
    cfg_if! {
        if #[cfg(all(feature = "asm", target_arch = "x86_64"))] {
            dav1d_msac_decode_symbol_adapt4_sse2(s, cdf.as_mut_ptr(), n_symbols)
        } else if #[cfg(all(feature = "asm", target_arch = "aarch64"))] {
            dav1d_msac_decode_symbol_adapt4_neon(s, cdf.as_mut_ptr(), n_symbols)
        } else {
            dav1d_msac_decode_symbol_adapt_rust(s, cdf, n_symbols)
        }
    }
}

pub unsafe fn dav1d_msac_decode_symbol_adapt8(
    s: &mut MsacContext,
    cdf: &mut [u16],
    n_symbols: size_t,
) -> libc::c_uint {
    cfg_if! {
        if #[cfg(all(feature = "asm", target_arch = "x86_64"))] {
             dav1d_msac_decode_symbol_adapt8_sse2(s, cdf.as_mut_ptr(), n_symbols)
        } else if #[cfg(all(feature = "asm", target_arch = "aarch64"))] {
             dav1d_msac_decode_symbol_adapt8_neon(s, cdf.as_mut_ptr(), n_symbols)
        } else {
             dav1d_msac_decode_symbol_adapt_rust(s, cdf, n_symbols)
        }
    }
}

pub unsafe fn dav1d_msac_decode_symbol_adapt16(
    s: &mut MsacContext,
    cdf: &mut [u16],
    n_symbols: size_t,
) -> libc::c_uint {
    cfg_if! {
        if #[cfg(all(feature = "asm", target_arch = "x86_64"))] {
            assert!(n_symbols < cdf.len());
            (s.symbol_adapt16).expect("non-null function pointer")(s, cdf.as_mut_ptr(), n_symbols)
        } else if #[cfg(all(feature = "asm", target_arch = "aarch64"))] {
            dav1d_msac_decode_symbol_adapt16_neon(s, cdf.as_mut_ptr(), n_symbols)
        } else {
            dav1d_msac_decode_symbol_adapt_rust(s, cdf, n_symbols)
        }
    }
}

pub unsafe fn dav1d_msac_decode_bool_adapt(
    s: &mut MsacContext,
    cdf: &mut [u16; 2],
) -> libc::c_uint {
    cfg_if! {
        if #[cfg(all(feature = "asm", target_arch = "x86_64"))] {
            dav1d_msac_decode_bool_adapt_sse2(s, cdf.as_mut_ptr())
        } else if #[cfg(all(feature = "asm", target_arch = "aarch64"))] {
            dav1d_msac_decode_bool_adapt_neon(s, cdf.as_mut_ptr())
        } else {
            dav1d_msac_decode_bool_adapt_rust(s, cdf)
        }
    }
}

pub unsafe fn dav1d_msac_decode_bool_equi(s: &mut MsacContext) -> libc::c_uint {
    cfg_if! {
        if #[cfg(all(feature = "asm", target_arch = "x86_64"))] {
             dav1d_msac_decode_bool_equi_sse2(s)
        } else if #[cfg(all(feature = "asm", target_arch = "aarch64"))] {
             dav1d_msac_decode_bool_equi_neon(s)
        } else {
             dav1d_msac_decode_bool_equi_rust(s)
        }
    }
}

pub unsafe fn dav1d_msac_decode_bool(s: &mut MsacContext, f: libc::c_uint) -> libc::c_uint {
    cfg_if! {
        if #[cfg(all(feature = "asm", target_arch = "x86_64"))] {
             dav1d_msac_decode_bool_sse2(s, f)
        } else if #[cfg(all(feature = "asm", target_arch = "aarch64"))] {
             dav1d_msac_decode_bool_neon(s, f)
        } else {
             dav1d_msac_decode_bool_rust(s, f)
        }
    }
}

pub unsafe fn dav1d_msac_decode_hi_tok(s: &mut MsacContext, cdf: &mut [u16; 4]) -> libc::c_uint {
    cfg_if! {
        if #[cfg(all(feature = "asm", target_arch = "x86_64"))] {
             dav1d_msac_decode_hi_tok_sse2(s, cdf.as_mut_ptr())
        } else if #[cfg(all(feature = "asm", target_arch = "aarch64"))] {
             dav1d_msac_decode_hi_tok_neon(s, cdf.as_mut_ptr())
        } else {
             dav1d_msac_decode_hi_tok_rust(s, cdf)
        }
    }
}
