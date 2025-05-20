#![deny(unsafe_op_in_unsafe_fn)]

use crate::c_arc::CArc;
use crate::cpu::CpuFlags;
use crate::include::common::attributes::clz;
use crate::include::common::intops::inv_recenter;
use crate::include::common::intops::ulog2;
use cfg_if::cfg_if;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::mem;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Range;
use std::ptr;
use std::slice;

#[cfg(all(feature = "asm", target_feature = "sse2"))]
extern "C" {
    fn dav1d_msac_decode_hi_tok_sse2(s: *mut MsacAsmContext, cdf: *mut u16) -> c_uint;
    fn dav1d_msac_decode_bool_sse2(s: *mut MsacAsmContext, f: c_uint) -> c_uint;
    fn dav1d_msac_decode_bool_equi_sse2(s: *mut MsacAsmContext) -> c_uint;
    fn dav1d_msac_decode_bool_adapt_sse2(s: *mut MsacAsmContext, cdf: *mut u16) -> c_uint;
    fn dav1d_msac_decode_symbol_adapt16_sse2(
        s: &mut MsacAsmContext,
        cdf: *mut u16,
        n_symbols: usize,
        _cdf_len: usize,
    ) -> c_uint;
    fn dav1d_msac_decode_symbol_adapt8_sse2(
        s: *mut MsacAsmContext,
        cdf: *mut u16,
        n_symbols: usize,
    ) -> c_uint;
    fn dav1d_msac_decode_symbol_adapt4_sse2(
        s: *mut MsacAsmContext,
        cdf: *mut u16,
        n_symbols: usize,
    ) -> c_uint;
}

#[cfg(all(feature = "asm", target_arch = "x86_64"))]
extern "C" {
    fn dav1d_msac_decode_symbol_adapt16_avx2(
        s: &mut MsacAsmContext,
        cdf: *mut u16,
        n_symbols: usize,
        _cdf_len: usize,
    ) -> c_uint;
}

#[cfg(all(feature = "asm", target_feature = "neon"))]
extern "C" {
    fn dav1d_msac_decode_hi_tok_neon(s: *mut MsacAsmContext, cdf: *mut u16) -> c_uint;
    fn dav1d_msac_decode_bool_neon(s: *mut MsacAsmContext, f: c_uint) -> c_uint;
    fn dav1d_msac_decode_bool_equi_neon(s: *mut MsacAsmContext) -> c_uint;
    fn dav1d_msac_decode_bool_adapt_neon(s: *mut MsacAsmContext, cdf: *mut u16) -> c_uint;
    fn dav1d_msac_decode_symbol_adapt16_neon(
        s: *mut MsacAsmContext,
        cdf: *mut u16,
        n_symbols: usize,
    ) -> c_uint;
    fn dav1d_msac_decode_symbol_adapt8_neon(
        s: *mut MsacAsmContext,
        cdf: *mut u16,
        n_symbols: usize,
    ) -> c_uint;
    fn dav1d_msac_decode_symbol_adapt4_neon(
        s: *mut MsacAsmContext,
        cdf: *mut u16,
        n_symbols: usize,
    ) -> c_uint;
}

pub struct Rav1dMsacDSPContext {
    symbol_adapt16: unsafe extern "C" fn(
        s: &mut MsacAsmContext,
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

pub type EcWin = usize;

/// # Safety
///
/// [`Self`] must be the first field of [`MsacAsmContext`] for asm layout purposes,
/// and that [`MsacAsmContext`] must be a field of [`MsacContext`].
/// And [`Self::pos`] and [`Self::end`] must be either [`ptr::null`],
/// or [`Self::pos`] must point into (or the end of) [`MsacContext::data`],
/// and [`Self::end`] must point to the end of [`MsacContext::data`],
/// where [`MsacContext::data`] is part of the [`MsacContext`]
/// containing [`MsacAsmContext`] and thus also [`Self`].
#[repr(C)]
struct MsacAsmContextBuf {
    pos: *const u8,
    end: *const u8,
}

/// SAFETY: [`MsacAsmContextBuf`] is always contained in [`MsacAsmContext::buf`],
/// which is always contained in [`MsacContext::asm`], whose [`MsacContext::data`] field
/// is what is stored in [`MsacAsmContextBuf::pos`] and [`MsacAsmContextBuf::end`].
/// Since [`MsacContext::data`] is [`Send`], [`MsacAsmContextBuf`] is also [`Send`].
unsafe impl Send for MsacAsmContextBuf {}

/// SAFETY: [`MsacAsmContextBuf`] is always contained in [`MsacAsmContext::buf`],
/// which is always contained in [`MsacContext::asm`], whose [`MsacContext::data`] field
/// is what is stored in [`MsacAsmContextBuf::pos`] and [`MsacAsmContextBuf::end`].
/// Since [`MsacContext::data`] is [`Sync`], [`MsacAsmContextBuf`] is also [`Sync`].
unsafe impl Sync for MsacAsmContextBuf {}

impl Default for MsacAsmContextBuf {
    fn default() -> Self {
        Self {
            pos: ptr::null(),
            end: ptr::null(),
        }
    }
}

impl From<&[u8]> for MsacAsmContextBuf {
    fn from(value: &[u8]) -> Self {
        let Range { start, end } = value.as_ptr_range();
        Self { pos: start, end }
    }
}

#[repr(C)]
pub struct MsacAsmContext {
    buf: MsacAsmContextBuf,
    pub dif: EcWin,
    pub rng: c_uint,
    pub cnt: c_int,
    allow_update_cdf: c_int,
    #[cfg(all(feature = "asm", target_arch = "x86_64"))]
    symbol_adapt16: unsafe extern "C" fn(
        s: &mut MsacAsmContext,
        cdf: *mut u16,
        n_symbols: usize,
        _cdf_len: usize,
    ) -> c_uint,
}

impl Default for MsacAsmContext {
    fn default() -> Self {
        Self {
            buf: Default::default(),
            dif: Default::default(),
            rng: Default::default(),
            cnt: Default::default(),
            allow_update_cdf: Default::default(),

            #[cfg(all(feature = "asm", target_arch = "x86_64"))]
            symbol_adapt16: Rav1dMsacDSPContext::default().symbol_adapt16,
        }
    }
}

impl MsacAsmContext {
    fn allow_update_cdf(&self) -> bool {
        self.allow_update_cdf != 0
    }
}

#[derive(Default)]
pub struct MsacContext {
    asm: MsacAsmContext,
    data: Option<CArc<[u8]>>,
}

impl Deref for MsacContext {
    type Target = MsacAsmContext;

    fn deref(&self) -> &Self::Target {
        &self.asm
    }
}

impl DerefMut for MsacContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.asm
    }
}

impl MsacContext {
    pub fn data(&self) -> &[u8] {
        &**self.data.as_ref().unwrap()
    }

    pub fn buf_index(&self) -> usize {
        // We safely subtract instead of unsafely use `ptr::offset_from`
        // as asm sets `buf_pos`, so we don't need to rely on its safety,
        // and because codegen is no less optimal this way.
        self.buf.pos as usize - self.data().as_ptr() as usize
    }

    fn with_buf(&mut self, mut f: impl FnMut(&[u8]) -> &[u8]) {
        let data = &**self.data.as_ref().unwrap();
        let buf = &data[self.buf_index()..];
        let buf = f(buf);
        self.buf.pos = buf.as_ptr();
        // We don't actually need to set `self.buf_end` since it has not changed.
    }
}

/// Return value uses `n` bits.
#[inline]
pub fn rav1d_msac_decode_bools(s: &mut MsacContext, n: u8) -> c_uint {
    let mut v = 0;
    for _ in 0..n {
        v = v << 1 | rav1d_msac_decode_bool_equi(s) as c_uint;
    }
    v
}

#[inline]
pub fn rav1d_msac_decode_uniform(s: &mut MsacContext, n: c_uint) -> c_int {
    assert!(n > 0);
    let l = ulog2(n) as u8 + 1;
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
const EC_MIN_PROB: c_uint = 4;
const _: () = assert!(EC_MIN_PROB <= (1 << EC_PROB_SHIFT) / 16);

const EC_WIN_SIZE: usize = mem::size_of::<EcWin>() << 3;

#[inline]
fn ctx_refill(s: &mut MsacContext) {
    let mut c = (EC_WIN_SIZE as c_int) - 24 - s.cnt;
    let mut dif = s.dif;
    s.with_buf(|mut buf| {
        loop {
            if buf.is_empty() {
                // set remaining bits to 1;
                dif |= !(!(0xff as EcWin) << c);
                break;
            }
            dif |= ((buf[0] ^ 0xff) as EcWin) << c;
            buf = &buf[1..];
            c -= 8;
            if c < 0 {
                break;
            }
        }
        buf
    });
    s.dif = dif;
    s.cnt = (EC_WIN_SIZE as c_int) - 24 - c;
}

#[inline]
fn ctx_norm(s: &mut MsacContext, dif: EcWin, rng: c_uint) {
    let d = 15 ^ (31 ^ clz(rng));
    let cnt = s.cnt;
    assert!(rng <= 65535);
    s.dif = dif << d;
    s.rng = rng << d;
    s.cnt = cnt - d;
    // unsigned compare avoids redundant refills at eob
    if (cnt as u32) < (d as u32) {
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
    assert!(dif >> (EC_WIN_SIZE - 16) < r as EcWin);
    let mut v = (r >> 8 << 7) + EC_MIN_PROB;
    let vw = (v as EcWin) << (EC_WIN_SIZE - 16);
    let ret = dif >= vw;
    dif -= (ret as EcWin) * vw;
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
    assert!(dif >> (EC_WIN_SIZE - 16) < r as EcWin);
    let mut v = ((r >> 8) * (f >> EC_PROB_SHIFT) >> (7 - EC_PROB_SHIFT)) + EC_MIN_PROB;
    let vw = (v as EcWin) << (EC_WIN_SIZE - 16);
    let ret = dif >= vw;
    dif -= (ret as EcWin) * vw;
    v = v.wrapping_add((ret as c_uint) * (r.wrapping_sub(2 * v)));
    ctx_norm(s, dif, v);
    !ret
}

pub fn rav1d_msac_decode_subexp(s: &mut MsacContext, r#ref: c_uint, n: c_uint, mut k: u8) -> c_int {
    assert!(n >> k == 8);
    let mut a = 0;
    if rav1d_msac_decode_bool_equi(s) {
        if rav1d_msac_decode_bool_equi(s) {
            k += rav1d_msac_decode_bool_equi(s) as u8 + 1;
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

/// Return value is in the range `0..=n_symbols`.
///
/// `n_symbols` is in the range `0..16`, so it is really a `u4`.
fn rav1d_msac_decode_symbol_adapt_rust(s: &mut MsacContext, cdf: &mut [u16], n_symbols: u8) -> u8 {
    let c = (s.dif >> (EC_WIN_SIZE - 16)) as c_uint;
    let r = s.rng >> 8;
    let mut u;
    let mut v = s.rng;
    let mut val = 0;
    assert!(n_symbols < 16);
    assert!(cdf[n_symbols as usize] <= 32);
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
        s.dif.wrapping_sub((v as EcWin) << (EC_WIN_SIZE - 16)),
        u - v,
    );
    if s.allow_update_cdf() {
        let count = cdf[n_symbols as usize];
        let rate = 4 + (count >> 4) + (n_symbols > 2) as u16;
        let val = val as usize;
        for cdf in &mut cdf[..val] {
            *cdf += (1 << 15) - *cdf >> rate;
        }
        for cdf in &mut cdf[val..n_symbols as usize] {
            *cdf -= *cdf >> rate;
        }
        cdf[n_symbols as usize] = count + (count < 32) as u16;
    }
    debug_assert!(val <= n_symbols as _);
    val as u8
}

/// # Safety
///
/// Must be called through [`Rav1dMsacDSPContext::symbol_adapt16`]
/// in [`rav1d_msac_decode_symbol_adapt16`].
#[cfg_attr(not(all(feature = "asm", target_arch = "x86_64")), allow(dead_code))]
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn rav1d_msac_decode_symbol_adapt_c(
    s: &mut MsacAsmContext,
    cdf: *mut u16,
    n_symbols: usize,
    cdf_len: usize,
) -> c_uint {
    // SAFETY: In the `rav1d_msac_decode_symbol_adapt16` caller,
    // `&mut s.asm` is passed, so we can reverse this to get back `s`.
    // The `.sub` is safe since were are subtracting the offset of `asm` within `s`,
    // so that will stay in bounds of the `s: MsacContext` allocated object.
    let s = unsafe {
        &mut *ptr::from_mut(s)
            .sub(mem::offset_of!(MsacContext, asm))
            .cast::<MsacContext>()
    };

    // SAFETY: This is only called from [`dav1d_msac_decode_symbol_adapt16`],
    // where it comes from `cdf.len()`.
    let cdf = unsafe { slice::from_raw_parts_mut(cdf, cdf_len) };

    rav1d_msac_decode_symbol_adapt_rust(s, cdf, n_symbols as u8) as c_uint
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

/// Return value is in the range `0..=15`.
#[cfg_attr(
    all(feature = "asm", any(target_feature = "sse2", target_feature = "neon")),
    allow(dead_code)
)]
fn rav1d_msac_decode_hi_tok_rust(s: &mut MsacContext, cdf: &mut [u16; 4]) -> u8 {
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

impl MsacContext {
    pub fn new(data: CArc<[u8]>, disable_cdf_update_flag: bool, dsp: &Rav1dMsacDSPContext) -> Self {
        let asm = MsacAsmContext {
            buf: data.as_ref().into(),
            dif: 0,
            rng: 0x8000,
            cnt: -15,
            allow_update_cdf: (!disable_cdf_update_flag).into(),
            #[cfg(all(feature = "asm", target_arch = "x86_64"))]
            symbol_adapt16: dsp.symbol_adapt16,
        };
        let mut s = Self {
            asm,
            data: Some(data),
        };
        let _ = dsp.symbol_adapt16; // Silence unused warnings.
        ctx_refill(&mut s);
        s
    }
}

/// Return value is in the range `0..=n_symbols`.
///
/// `n_symbols` is in the range `0..4`.
#[inline(always)]
pub fn rav1d_msac_decode_symbol_adapt4(s: &mut MsacContext, cdf: &mut [u16], n_symbols: u8) -> u8 {
    debug_assert!(n_symbols < 4);
    let ret;
    cfg_if! {
        if #[cfg(all(feature = "asm", target_feature = "sse2"))] {
            // SAFETY: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_symbol_adapt_rust`].
            ret = unsafe {
                dav1d_msac_decode_symbol_adapt4_sse2(&mut s.asm, cdf.as_mut_ptr(), n_symbols as usize)
            };
        } else if #[cfg(all(feature = "asm", target_feature = "neon"))] {
            // SAFETY: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_symbol_adapt_rust`].
            ret = unsafe {
                dav1d_msac_decode_symbol_adapt4_neon(&mut s.asm, cdf.as_mut_ptr(), n_symbols as usize)
            };
        } else {
            ret = rav1d_msac_decode_symbol_adapt_rust(s, cdf, n_symbols);
        }
    }
    debug_assert!(ret < 4);
    ret as u8 % 4
}

/// Return value is in the range `0..=n_symbols`.
///
/// `n_symbols` is in the range `0..8`.
#[inline(always)]
pub fn rav1d_msac_decode_symbol_adapt8(s: &mut MsacContext, cdf: &mut [u16], n_symbols: u8) -> u8 {
    debug_assert!(n_symbols < 8);
    let ret;
    cfg_if! {
        if #[cfg(all(feature = "asm", target_feature = "sse2"))] {
            // SAFETY: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_symbol_adapt_rust`].
            ret = unsafe {
                dav1d_msac_decode_symbol_adapt8_sse2(&mut s.asm, cdf.as_mut_ptr(), n_symbols as usize)
            };
        } else if #[cfg(all(feature = "asm", target_feature = "neon"))] {
            // SAFETY: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_symbol_adapt_rust`].
            ret = unsafe {
                dav1d_msac_decode_symbol_adapt8_neon(&mut s.asm, cdf.as_mut_ptr(), n_symbols as usize)
            };
        } else {
            ret = rav1d_msac_decode_symbol_adapt_rust(s, cdf, n_symbols);
        }
    }
    debug_assert!(ret < 8);
    ret as u8 % 8
}

/// Return value is in the range `0..=n_symbols`.
///
/// `n_symbols` is in the range `0..16`.
#[inline(always)]
pub fn rav1d_msac_decode_symbol_adapt16(s: &mut MsacContext, cdf: &mut [u16], n_symbols: u8) -> u8 {
    debug_assert!(n_symbols < 16);
    let ret;
    cfg_if! {
        if #[cfg(all(feature = "asm", target_arch = "x86_64"))] {
            // SAFETY: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_symbol_adapt_rust`].
            ret = unsafe {
                (s.symbol_adapt16)(&mut s.asm, cdf.as_mut_ptr(), n_symbols as usize, cdf.len())
            };
        } else if #[cfg(all(feature = "asm", target_feature = "sse2"))] {
            // SAFETY: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_symbol_adapt_rust`].
            ret = unsafe {
                dav1d_msac_decode_symbol_adapt16_sse2(&mut s.asm, cdf.as_mut_ptr(), n_symbols as usize, cdf.len())
            };
        } else if #[cfg(all(feature = "asm", target_feature = "neon"))] {
            // SAFETY: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_symbol_adapt_rust`].
            ret = unsafe {
                dav1d_msac_decode_symbol_adapt16_neon(&mut s.asm, cdf.as_mut_ptr(), n_symbols as usize)
            };
        } else {
            ret = rav1d_msac_decode_symbol_adapt_rust(s, cdf, n_symbols);
        }
    }
    debug_assert!(ret < 16);
    ret as u8 % 16
}

pub fn rav1d_msac_decode_bool_adapt(s: &mut MsacContext, cdf: &mut [u16; 2]) -> bool {
    cfg_if! {
        if #[cfg(all(feature = "asm", target_feature = "sse2"))] {
            // SAFETY: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_bool_adapt_rust`].
            unsafe {
                dav1d_msac_decode_bool_adapt_sse2(&mut s.asm, cdf.as_mut_ptr()) != 0
            }
        } else if #[cfg(all(feature = "asm", target_feature = "neon"))] {
            // SAFETY: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_bool_adapt_rust`].
            unsafe {
                dav1d_msac_decode_bool_adapt_neon(&mut s.asm, cdf.as_mut_ptr()) != 0
            }
        } else {
            rav1d_msac_decode_bool_adapt_rust(s, cdf)
        }
    }
}

pub fn rav1d_msac_decode_bool_equi(s: &mut MsacContext) -> bool {
    cfg_if! {
        if #[cfg(all(feature = "asm", target_feature = "sse2"))] {
            // SAFETY: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_bool_equi_rust`].
            unsafe {
                dav1d_msac_decode_bool_equi_sse2(&mut s.asm) != 0
            }
        } else if #[cfg(all(feature = "asm", target_feature = "neon"))] {
            // SAFETY: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_bool_equi_rust`].
            unsafe {
                dav1d_msac_decode_bool_equi_neon(&mut s.asm) != 0
            }
        } else {
            rav1d_msac_decode_bool_equi_rust(s)
        }
    }
}

pub fn rav1d_msac_decode_bool(s: &mut MsacContext, f: c_uint) -> bool {
    cfg_if! {
        if #[cfg(all(feature = "asm", target_feature = "sse2"))] {
            // SAFETY: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_bool_rust`].
            unsafe {
                dav1d_msac_decode_bool_sse2(&mut s.asm, f) != 0
            }
        } else if #[cfg(all(feature = "asm", target_feature = "neon"))] {
            // SAFETY: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_bool_rust`].
            unsafe {
                dav1d_msac_decode_bool_neon(&mut s.asm, f) != 0
            }
        } else {
            rav1d_msac_decode_bool_rust(s, f)
        }
    }
}

/// Return value is in the range `0..16`.
#[inline(always)]
pub fn rav1d_msac_decode_hi_tok(s: &mut MsacContext, cdf: &mut [u16; 4]) -> u8 {
    let ret;
    cfg_if! {
        if #[cfg(all(feature = "asm", target_feature = "sse2"))] {
            // SAFETY: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_hi_tok_rust`].
            ret = (unsafe {
                dav1d_msac_decode_hi_tok_sse2(&mut s.asm, cdf.as_mut_ptr())
            }) as u8;
        } else if #[cfg(all(feature = "asm", target_feature = "neon"))] {
            // SAFETY: `checkasm` has verified that it is equivalent to [`dav1d_msac_decode_hi_tok_rust`].
            ret = unsafe {
                dav1d_msac_decode_hi_tok_neon(&mut s.asm, cdf.as_mut_ptr())
            } as u8;
        } else {
            ret = rav1d_msac_decode_hi_tok_rust(s, cdf);
        }
    }
    debug_assert!(ret < 16);
    ret % 16
}
