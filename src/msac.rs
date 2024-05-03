use crate::include::common::attributes::clz;
use crate::include::common::intops::inv_recenter;
use crate::include::common::intops::ulog2;
use crate::src::cpu::CpuFlags;
use cfg_if::cfg_if;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::mem;
use std::ops::Range;

#[cfg(all(feature = "asm", target_feature = "sse2"))]
extern "C" {
    fn dav1d_msac_decode_hi_tok_sse2(s: *mut MsacContext, cdf: *mut u16) -> c_uint;
    fn dav1d_msac_decode_bool_sse2(s: *mut MsacContext, f: c_uint) -> c_uint;
    fn dav1d_msac_decode_bool_equi_sse2(s: *mut MsacContext) -> c_uint;
    fn dav1d_msac_decode_bool_adapt_sse2(s: *mut MsacContext, cdf: *mut u16) -> c_uint;
    fn dav1d_msac_decode_symbol_adapt16_sse2(
        s: &mut MsacContext,
        cdf: *mut u16,
        n_symbols: usize,
        _cdf_len: usize,
    ) -> c_uint;
    fn dav1d_msac_decode_symbol_adapt8_sse2(
        s: *mut MsacContext,
        cdf: *mut u16,
        n_symbols: usize,
    ) -> c_uint;
    fn dav1d_msac_decode_symbol_adapt4_sse2(
        s: *mut MsacContext,
        cdf: *mut u16,
        n_symbols: usize,
    ) -> c_uint;
}

#[cfg(all(feature = "asm", target_arch = "x86_64"))]
extern "C" {
    fn dav1d_msac_decode_symbol_adapt16_avx2(
        s: &mut MsacContext,
        cdf: *mut u16,
        n_symbols: usize,
        _cdf_len: usize,
    ) -> c_uint;
}

#[cfg(all(feature = "asm", target_feature = "neon"))]
extern "C" {
    fn dav1d_msac_decode_hi_tok_neon(s: *mut MsacContext, cdf: *mut u16) -> c_uint;
    fn dav1d_msac_decode_bool_neon(s: *mut MsacContext, f: c_uint) -> c_uint;
    fn dav1d_msac_decode_bool_equi_neon(s: *mut MsacContext) -> c_uint;
    fn dav1d_msac_decode_bool_adapt_neon(s: *mut MsacContext, cdf: *mut u16) -> c_uint;
    fn dav1d_msac_decode_symbol_adapt16_neon(
        s: *mut MsacContext,
        cdf: *mut u16,
        n_symbols: usize,
    ) -> c_uint;
    fn dav1d_msac_decode_symbol_adapt8_neon(
        s: *mut MsacContext,
        cdf: *mut u16,
        n_symbols: usize,
    ) -> c_uint;
    fn dav1d_msac_decode_symbol_adapt4_neon(
        s: *mut MsacContext,
        cdf: *mut u16,
        n_symbols: usize,
    ) -> c_uint;
}

pub struct Rav1dMsacDSPContext {
    symbol_adapt16: unsafe extern "C" fn(
        s: &mut MsacContext,
        cdf: *mut u16,
        n_symbols: usize,
        _cdf_len: usize,
    ) -> c_uint,
}

impl Rav1dMsacDSPContext {
    pub const fn default() -> Self {
        Self {
            symbol_adapt16: rav1d_msac_decode_symbol_adapt_c,
        }
    }

    #[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
    #[inline(always)]
    const fn init_x86(mut self, flags: CpuFlags) -> Self {
        if !flags.contains(CpuFlags::SSE2) {
            return self;
        }

        self.symbol_adapt16 = dav1d_msac_decode_symbol_adapt16_sse2;

        #[cfg(target_arch = "x86_64")]
        {
            if !flags.contains(CpuFlags::AVX2) {
                return self;
            }

            self.symbol_adapt16 = dav1d_msac_decode_symbol_adapt16_avx2;
        }

        self
    }

    #[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
    #[inline(always)]
    const fn init_arm(self, _flags: CpuFlags) -> Self {
        self
    }

    #[inline(always)]
    const fn init(self, flags: CpuFlags) -> Self {
        #[cfg(feature = "asm")]
        {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            {
                return self.init_x86(flags);
            }
            #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
            {
                return self.init_arm(flags);
            }
        }

        #[allow(unreachable_code)] // Reachable on some #[cfg]s.
        {
            let _ = flags;
            self
        }
    }

    pub const fn new(flags: CpuFlags) -> Self {
        Self::default().init(flags)
    }
}

impl Default for Rav1dMsacDSPContext {
    fn default() -> Self {
        Self::default()
    }
}

pub type ec_win = usize;

#[repr(C)]
pub struct MsacContext {
    buf_pos: *const u8,
    buf_end: *const u8,
    pub dif: ec_win,
    pub rng: c_uint,
    pub cnt: c_int,
    allow_update_cdf: c_int,
    #[cfg(all(feature = "asm", target_arch = "x86_64"))]
    symbol_adapt16: unsafe extern "C" fn(
        s: &mut MsacContext,
        cdf: *mut u16,
        n_symbols: usize,
        _cdf_len: usize,
    ) -> c_uint,
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
pub fn rav1d_msac_decode_bools(s: &mut MsacContext, n: c_uint) -> c_uint {
    let mut v = 0;
    for _ in 0..n {
        v = v << 1 | rav1d_msac_decode_bool_equi(s) as c_uint;
    }
    v
}

#[inline]
pub fn rav1d_msac_decode_uniform(s: &mut MsacContext, n: c_uint) -> c_int {
    assert!(n > 0);
    let l = ulog2(n) as c_uint + 1;
    assert!(l > 1);
    let m = (1 << l) - n;
    let v = rav1d_msac_decode_bools(s, l - 1);
    (if v < m {
        v
    } else {
        (v << 1) - m + rav1d_msac_decode_bool_equi(s) as c_uint
    }) as c_int
}

const EC_PROB_SHIFT: c_uint = 6;
const EC_MIN_PROB: c_uint = 4; // must be <= (1 << EC_PROB_SHIFT) / 16

const EC_WIN_SIZE: usize = mem::size_of::<ec_win>() << 3;

#[inline]
fn ctx_refill(s: &mut MsacContext) {
    let mut c = (EC_WIN_SIZE as c_int) - 24 - s.cnt;
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
    s.cnt = (EC_WIN_SIZE as c_int) - 24 - c;
}

#[inline]
fn ctx_norm(s: &mut MsacContext, dif: ec_win, rng: c_uint) {
    let d = 15 ^ (31 ^ clz(rng));
    assert!(rng <= 65535);
    s.cnt -= d;
    s.dif = ((dif + 1) << d) - 1;
    s.rng = rng << d;
    if s.cnt < 0 {
        ctx_refill(s);
    }
}

#[cfg_attr(
    all(feature = "asm", any(target_feature = "sse2", target_feature = "neon")),
    allow(dead_code)
)]
fn rav1d_msac_decode_bool_equi_rust(s: &mut MsacContext) -> bool {
    let r = s.rng;
    let mut dif = s.dif;
    assert!(dif >> (EC_WIN_SIZE - 16) < r as ec_win);
    let mut v = (r >> 8 << 7) + EC_MIN_PROB;
    let vw = (v as ec_win) << (EC_WIN_SIZE - 16);
    let ret = dif >= vw;
    dif -= (ret as ec_win) * vw;
    v = v.wrapping_add((ret as c_uint) * (r.wrapping_sub(2 * v)));
    ctx_norm(s, dif, v);
    !ret
}

#[cfg_attr(
    all(feature = "asm", any(target_feature = "sse2", target_feature = "neon")),
    allow(dead_code)
)]
fn rav1d_msac_decode_bool_rust(s: &mut MsacContext, f: c_uint) -> bool {
    let r = s.rng;
    let mut dif = s.dif;
    assert!(dif >> (EC_WIN_SIZE - 16) < r as ec_win);
    let mut v = ((r >> 8) * (f >> EC_PROB_SHIFT) >> (7 - EC_PROB_SHIFT)) + EC_MIN_PROB;
    let vw = (v as ec_win) << (EC_WIN_SIZE - 16);
    let ret = dif >= vw;
    dif -= (ret as ec_win) * vw;
    v = v.wrapping_add((ret as c_uint) * (r.wrapping_sub(2 * v)));
    ctx_norm(s, dif, v);
    !ret
}

pub fn rav1d_msac_decode_subexp(
    s: &mut MsacContext,
    r#ref: c_uint,
    n: c_uint,
    mut k: c_uint,
) -> c_int {
    assert!(n >> k == 8);
    let mut a = 0;
    if rav1d_msac_decode_bool_equi(s) {
        if rav1d_msac_decode_bool_equi(s) {
            k += rav1d_msac_decode_bool_equi(s) as c_uint + 1;
        }
        a = 1 << k;
    }
    let v = rav1d_msac_decode_bools(s, k) + a;
    (if r#ref * 2 <= n {
        inv_recenter(r#ref, v)
    } else {
        n - 1 - inv_recenter(n - 1 - r#ref, v)
    }) as c_int
}

fn rav1d_msac_decode_symbol_adapt_rust(
    s: &mut MsacContext,
    cdf: &mut [u16],
    n_symbols: usize,
) -> c_uint {
    let c = (s.dif >> (EC_WIN_SIZE - 16)) as c_uint;
    let r = s.rng >> 8;
    let mut u;
    let mut v = s.rng;
    let mut val = 0;
    assert!(n_symbols <= 15);
    assert!(cdf[n_symbols] <= 32);
    loop {
        u = v;
        v = r * ((cdf[val as usize] >> EC_PROB_SHIFT) as c_uint);
        v >>= 7 - EC_PROB_SHIFT;
        v += EC_MIN_PROB * ((n_symbols as c_uint) - val);
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

#[cfg_attr(not(all(feature = "asm", target_arch = "x86_64")), allow(dead_code))]
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn rav1d_msac_decode_symbol_adapt_c(
    s: &mut MsacContext,
    cdf: *mut u16,
    n_symbols: usize,
    cdf_len: usize,
) -> c_uint {
    // # Safety
    //
    // This is only called from [`dav1d_msac_decode_symbol_adapt16`],
    // where it comes from `cdf.len()`.
    let cdf = unsafe { std::slice::from_raw_parts_mut(cdf, cdf_len) };

    rav1d_msac_decode_symbol_adapt_rust(s, cdf, n_symbols)
}

#[cfg_attr(
    all(feature = "asm", any(target_feature = "sse2", target_feature = "neon")),
    allow(dead_code)
)]
fn rav1d_msac_decode_bool_adapt_rust(s: &mut MsacContext, cdf: &mut [u16; 2]) -> bool {
    let bit = rav1d_msac_decode_bool(s, cdf[0] as c_uint);
    if s.allow_update_cdf() {
        let count = cdf[1];
        let rate = 4 + (count >> 4);
        if bit {
            cdf[0] += (1 << 15) - cdf[0] >> rate;
        } else {
            cdf[0] -= cdf[0] >> rate;
        }
        cdf[1] = count + (count < 32) as u16;
    }
    bit
}

#[cfg_attr(
    all(feature = "asm", any(target_feature = "sse2", target_feature = "neon")),
    allow(dead_code)
)]
fn rav1d_msac_decode_hi_tok_rust(s: &mut MsacContext, cdf: &mut [u16; 4]) -> c_uint {
    let mut tok_br = rav1d_msac_decode_symbol_adapt4(s, cdf, 3);
    let mut tok = 3 + tok_br;
    if tok_br == 3 {
        tok_br = rav1d_msac_decode_symbol_adapt4(s, cdf, 3);
        tok = 6 + tok_br;
        if tok_br == 3 {
            tok_br = rav1d_msac_decode_symbol_adapt4(s, cdf, 3);
            tok = 9 + tok_br;
            if tok_br == 3 {
                tok = 12 + rav1d_msac_decode_symbol_adapt4(s, cdf, 3);
            }
        }
    }
    tok
}

/// # Safety
///
/// `data` and `sz` must form a valid slice,
/// and must live longer than all of the other functions called on [`MsacContext`].
pub unsafe fn rav1d_msac_init(
    s: &mut MsacContext,
    data: *const u8,
    sz: usize,
    disable_cdf_update_flag: bool,
    dsp: &Rav1dMsacDSPContext,
) {
    s.set_buf(std::slice::from_raw_parts(data, sz));
    s.dif = (1 << (EC_WIN_SIZE - 1)) - 1;
    s.rng = 0x8000;
    s.cnt = -15;
    s.set_allow_update_cdf(!disable_cdf_update_flag);
    ctx_refill(s);

    #[cfg(feature = "asm")]
    {
        #[cfg(target_arch = "x86_64")]
        {
            s.symbol_adapt16 = dsp.symbol_adapt16;
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            let _ = dsp.symbol_adapt16;
        }
    }
}

pub fn rav1d_msac_decode_symbol_adapt4(
    s: &mut MsacContext,
    cdf: &mut [u16],
    n_symbols: usize,
) -> c_uint {
    cfg_if! {
        if #[cfg(all(feature = "asm", target_feature = "sse2"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_symbol_adapt_rust`].
            unsafe {
                dav1d_msac_decode_symbol_adapt4_sse2(s, cdf.as_mut_ptr(), n_symbols)
            }
        } else if #[cfg(all(feature = "asm", target_feature = "neon"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_symbol_adapt_rust`].
            unsafe {
                dav1d_msac_decode_symbol_adapt4_neon(s, cdf.as_mut_ptr(), n_symbols)
            }
        } else {
            rav1d_msac_decode_symbol_adapt_rust(s, cdf, n_symbols)
        }
    }
}

pub fn rav1d_msac_decode_symbol_adapt8(
    s: &mut MsacContext,
    cdf: &mut [u16],
    n_symbols: usize,
) -> c_uint {
    cfg_if! {
        if #[cfg(all(feature = "asm", target_feature = "sse2"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_symbol_adapt_rust`].
            unsafe {
                dav1d_msac_decode_symbol_adapt8_sse2(s, cdf.as_mut_ptr(), n_symbols)
            }
        } else if #[cfg(all(feature = "asm", target_feature = "neon"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_symbol_adapt_rust`].
            unsafe {
                dav1d_msac_decode_symbol_adapt8_neon(s, cdf.as_mut_ptr(), n_symbols)
            }
        } else {
            rav1d_msac_decode_symbol_adapt_rust(s, cdf, n_symbols)
        }
    }
}

pub fn rav1d_msac_decode_symbol_adapt16(
    s: &mut MsacContext,
    cdf: &mut [u16],
    n_symbols: usize,
) -> c_uint {
    cfg_if! {
        if #[cfg(all(feature = "asm", target_arch = "x86_64"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_symbol_adapt_rust`].
            unsafe {
                (s.symbol_adapt16)(s, cdf.as_mut_ptr(), n_symbols, cdf.len())
            }
        } else if #[cfg(all(feature = "asm", target_feature = "sse2"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_symbol_adapt_rust`].
            unsafe {
                dav1d_msac_decode_symbol_adapt16_sse2(s, cdf.as_mut_ptr(), n_symbols, cdf.len())
            }
        } else if #[cfg(all(feature = "asm", target_feature = "neon"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_symbol_adapt_rust`].
            unsafe {
                dav1d_msac_decode_symbol_adapt16_neon(s, cdf.as_mut_ptr(), n_symbols)
            }
        } else {
            rav1d_msac_decode_symbol_adapt_rust(s, cdf, n_symbols)
        }
    }
}

pub fn rav1d_msac_decode_bool_adapt(s: &mut MsacContext, cdf: &mut [u16; 2]) -> bool {
    cfg_if! {
        if #[cfg(all(feature = "asm", target_feature = "sse2"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_bool_adapt_rust`].
            unsafe {
                dav1d_msac_decode_bool_adapt_sse2(s, cdf.as_mut_ptr()) != 0
            }
        } else if #[cfg(all(feature = "asm", target_feature = "neon"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_bool_adapt_rust`].
            unsafe {
                dav1d_msac_decode_bool_adapt_neon(s, cdf.as_mut_ptr()) != 0
            }
        } else {
            rav1d_msac_decode_bool_adapt_rust(s, cdf)
        }
    }
}

pub fn rav1d_msac_decode_bool_equi(s: &mut MsacContext) -> bool {
    cfg_if! {
        if #[cfg(all(feature = "asm", target_feature = "sse2"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_bool_equi_rust`].
            unsafe {
                dav1d_msac_decode_bool_equi_sse2(s) != 0
            }
        } else if #[cfg(all(feature = "asm", target_feature = "neon"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_bool_equi_rust`].
            unsafe {
                dav1d_msac_decode_bool_equi_neon(s) != 0
            }
        } else {
            rav1d_msac_decode_bool_equi_rust(s)
        }
    }
}

pub fn rav1d_msac_decode_bool(s: &mut MsacContext, f: c_uint) -> bool {
    cfg_if! {
        if #[cfg(all(feature = "asm", target_feature = "sse2"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_bool_rust`].
            unsafe {
                dav1d_msac_decode_bool_sse2(s, f) != 0
            }
        } else if #[cfg(all(feature = "asm", target_feature = "neon"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_bool_rust`].
            unsafe {
                dav1d_msac_decode_bool_neon(s, f) != 0
            }
        } else {
            rav1d_msac_decode_bool_rust(s, f)
        }
    }
}

pub fn rav1d_msac_decode_hi_tok(s: &mut MsacContext, cdf: &mut [u16; 4]) -> c_uint {
    cfg_if! {
        if #[cfg(all(feature = "asm", target_feature = "sse2"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_hi_tok_rust`].
            unsafe {
                dav1d_msac_decode_hi_tok_sse2(s, cdf.as_mut_ptr())
            }
        } else if #[cfg(all(feature = "asm", target_feature = "neon"))] {
            // Safety: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_hi_tok_rust`].
            unsafe {
                dav1d_msac_decode_hi_tok_neon(s, cdf.as_mut_ptr())
            }
        } else {
            rav1d_msac_decode_hi_tok_rust(s, cdf)
        }
    }
}
