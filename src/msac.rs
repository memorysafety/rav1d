use ::libc;
use cfg_if::cfg_if;

//#[cfg(all(feature = "asm", target_arch = "x86_64"))]
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
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type size_t = libc::c_ulong;
pub type ec_win = size_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MsacContext {
    pub buf_pos: *const uint8_t,
    pub buf_end: *const uint8_t,
    pub dif: ec_win,
    pub rng: libc::c_uint,
    pub cnt: libc::c_int,
    pub allow_update_cdf: libc::c_int,
    //#[cfg(all(feature = "asm", target_arch = "x86_64"))]
    pub symbol_adapt16:
        Option<unsafe extern "C" fn(*mut MsacContext, *mut uint16_t, size_t) -> libc::c_uint>,
}
#[inline]
unsafe extern "C" fn clz(mask: libc::c_uint) -> libc::c_int {
    return mask.leading_zeros() as i32;
}
//cfg_if! {
//    if #[cfg(all(feature = "asm", target_arch = "x86_64"))] {
pub type CpuFlags = libc::c_uint;
pub const DAV1D_X86_CPU_FLAG_SLOW_GATHER: CpuFlags = 32;
pub const DAV1D_X86_CPU_FLAG_AVX512ICL: CpuFlags = 16;
pub const DAV1D_X86_CPU_FLAG_AVX2: CpuFlags = 8;
pub const DAV1D_X86_CPU_FLAG_SSE41: CpuFlags = 4;
pub const DAV1D_X86_CPU_FLAG_SSSE3: CpuFlags = 2;
pub const DAV1D_X86_CPU_FLAG_SSE2: CpuFlags = 1;
//    }
//}
#[inline]
unsafe extern "C" fn inv_recenter(r: libc::c_uint, v: libc::c_uint) -> libc::c_uint {
    if v > r << 1i32 {
        return v;
    } else if v & 1u32 == 0u32 {
        return (v >> 1i32).wrapping_add(r);
    } else {
        return r.wrapping_sub(v.wrapping_add(1u32) >> 1i32);
    };
}
#[inline]
unsafe extern "C" fn dav1d_msac_decode_bools(
    s: *mut MsacContext,
    mut n: libc::c_uint,
) -> libc::c_uint {
    let mut v: libc::c_uint = 0u32;
    loop {
        let fresh0 = n;
        n = n.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        v = v << 1i32 | dav1d_msac_decode_bool_equi(s);
    }
    return v;
}

//#[cfg(all(feature = "asm", target_arch = "x86_64"))]
#[inline(always)]
unsafe extern "C" fn msac_init_x86(s: *mut MsacContext) {
    let flags: libc::c_uint = dav1d_get_cpu_flags();
    if flags & DAV1D_X86_CPU_FLAG_SSE2 != 0 {
        (*s).symbol_adapt16 = Some(
            dav1d_msac_decode_symbol_adapt16_sse2
                as unsafe extern "C" fn(*mut MsacContext, *mut uint16_t, size_t) -> libc::c_uint,
        );
    }
    if flags & DAV1D_X86_CPU_FLAG_AVX2 != 0 {
        (*s).symbol_adapt16 = Some(
            dav1d_msac_decode_symbol_adapt16_avx2
                as unsafe extern "C" fn(*mut MsacContext, *mut uint16_t, size_t) -> libc::c_uint,
        );
    }
}
//#[cfg(all(feature = "asm", target_arch = "x86_64"))]
#[inline(always)]
unsafe extern "C" fn dav1d_get_cpu_flags() -> libc::c_uint {
    let mut flags: libc::c_uint = dav1d_cpu_flags & dav1d_cpu_flags_mask;
    flags |= DAV1D_X86_CPU_FLAG_SSE2;
    return flags;
}
#[inline]
unsafe extern "C" fn ctx_refill(s: *mut MsacContext) {
    let mut buf_pos: *const uint8_t = (*s).buf_pos;
    let mut buf_end: *const uint8_t = (*s).buf_end;
    let mut c: libc::c_int = ((::core::mem::size_of::<ec_win>() as libc::c_ulong) << 3i32)
        .wrapping_sub((*s).cnt as libc::c_ulong)
        .wrapping_sub(24u64) as libc::c_int;
    let mut dif: ec_win = (*s).dif;
    while c >= 0i32 && buf_pos < buf_end {
        let fresh1 = buf_pos;
        buf_pos = buf_pos.offset(1);
        dif ^= (*fresh1 as ec_win) << c;
        c -= 8i32;
    }
    (*s).dif = dif;
    (*s).cnt = ((::core::mem::size_of::<ec_win>() as libc::c_ulong) << 3i32)
        .wrapping_sub(c as libc::c_ulong)
        .wrapping_sub(24u64) as libc::c_int;
    (*s).buf_pos = buf_pos;
}
#[inline]
unsafe extern "C" fn ctx_norm(s: *mut MsacContext, dif: ec_win, rng: libc::c_uint) {
    let d: libc::c_int = 15i32 ^ (31i32 ^ clz(rng));
    if !(rng <= 65535u32) {
        unreachable!();
    }
    (*s).cnt -= d;
    (*s).dif = (dif.wrapping_add(1u64) << d).wrapping_sub(1u64);
    (*s).rng = rng << d;
    if (*s).cnt < 0i32 {
        ctx_refill(s);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_msac_decode_bool_equi_c(s: *mut MsacContext) -> libc::c_uint {
    let r: libc::c_uint = (*s).rng;
    let mut dif: ec_win = (*s).dif;
    if !(dif >> ((::core::mem::size_of::<ec_win>() as libc::c_ulong) << 3i32).wrapping_sub(16u64)
        < r as libc::c_ulong)
    {
        unreachable!();
    }
    let mut v: libc::c_uint = ((r >> 8i32) << 7i32).wrapping_add(4u32);
    let vw: ec_win = (v as ec_win)
        << ((::core::mem::size_of::<ec_win>() as libc::c_ulong) << 3i32).wrapping_sub(16u64);
    let ret: libc::c_uint = (dif >= vw) as libc::c_uint;
    dif = (dif).wrapping_sub((ret as libc::c_ulong).wrapping_mul(vw));
    v = v.wrapping_add(ret.wrapping_mul(r.wrapping_sub((2u32).wrapping_mul(v))));
    ctx_norm(s, dif, v);
    return (ret == 0) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_msac_decode_bool_c(
    s: *mut MsacContext,
    f: libc::c_uint,
) -> libc::c_uint {
    let r: libc::c_uint = (*s).rng;
    let mut dif: ec_win = (*s).dif;
    if !(dif >> ((::core::mem::size_of::<ec_win>() as libc::c_ulong) << 3i32).wrapping_sub(16u64)
        < r as libc::c_ulong)
    {
        unreachable!();
    }
    let mut v: libc::c_uint =
        ((r >> 8i32).wrapping_mul(f >> 6i32) >> 7i32 - 6i32).wrapping_add(4u32);
    let vw: ec_win = (v as ec_win)
        << ((::core::mem::size_of::<ec_win>() as libc::c_ulong) << 3i32).wrapping_sub(16u64);
    let ret: libc::c_uint = (dif >= vw) as libc::c_uint;
    dif = (dif).wrapping_sub((ret as libc::c_ulong).wrapping_mul(vw));
    v = v.wrapping_add(ret.wrapping_mul(r.wrapping_sub((2u32).wrapping_mul(v))));
    ctx_norm(s, dif, v);
    return (ret == 0) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_msac_decode_subexp(
    s: *mut MsacContext,
    ref_0: libc::c_int,
    n: libc::c_int,
    mut k: libc::c_uint,
) -> libc::c_int {
    if !(n >> k == 8i32) {
        unreachable!();
    }
    let mut a: libc::c_uint = 0u32;
    if dav1d_msac_decode_bool_equi(s) != 0 {
        if dav1d_msac_decode_bool_equi(s) != 0 {
            k = k.wrapping_add((dav1d_msac_decode_bool_equi(s)).wrapping_add(1u32));
        }
        a = ((1i32) << k) as libc::c_uint;
    }
    let v: libc::c_uint = (dav1d_msac_decode_bools(s, k)).wrapping_add(a);
    return (if ref_0 * 2i32 <= n {
        inv_recenter(ref_0 as libc::c_uint, v)
    } else {
        ((n - 1i32) as libc::c_uint)
            .wrapping_sub(inv_recenter((n - 1i32 - ref_0) as libc::c_uint, v))
    }) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_msac_decode_symbol_adapt_c(
    s: *mut MsacContext,
    cdf: *mut uint16_t,
    n_symbols: size_t,
) -> libc::c_uint {
    let c: libc::c_uint = ((*s).dif
        >> ((::core::mem::size_of::<ec_win>() as libc::c_ulong) << 3i32).wrapping_sub(16u64))
        as libc::c_uint;
    let r: libc::c_uint = (*s).rng >> 8i32;
    let mut u: libc::c_uint = 0;
    let mut v: libc::c_uint = (*s).rng;
    let mut val: libc::c_uint = -(1i32) as libc::c_uint;
    if !(n_symbols <= 15u64) {
        unreachable!();
    }
    if !(*cdf.offset(n_symbols as isize) as libc::c_int <= 32i32) {
        unreachable!();
    }
    loop {
        val = val.wrapping_add(1);
        u = v;
        v = r.wrapping_mul((*cdf.offset(val as isize) as libc::c_int >> 6i32) as libc::c_uint);
        v >>= 7i32 - 6i32;
        v = v.wrapping_add((4u32).wrapping_mul((n_symbols as libc::c_uint).wrapping_sub(val)));
        if !(c < v) {
            break;
        }
    }
    if !(u <= (*s).rng) {
        unreachable!();
    }
    ctx_norm(
        s,
        ((*s).dif).wrapping_sub(
            (v as ec_win)
                << ((::core::mem::size_of::<ec_win>() as libc::c_ulong) << 3i32)
                    .wrapping_sub(16u64),
        ),
        u.wrapping_sub(v),
    );
    if (*s).allow_update_cdf != 0 {
        let count: libc::c_uint = *cdf.offset(n_symbols as isize) as libc::c_uint;
        let rate: libc::c_uint = (4u32)
            .wrapping_add(count >> 4i32)
            .wrapping_add((n_symbols > 2u64) as libc::c_uint);
        let mut i: libc::c_uint = 0;
        i = 0u32;
        while i < val {
            let ref mut fresh2 = *cdf.offset(i as isize);
            *fresh2 = (*fresh2 as libc::c_int
                + (32768i32 - *cdf.offset(i as isize) as libc::c_int >> rate))
                as uint16_t;
            i = i.wrapping_add(1);
        }
        while (i as libc::c_ulong) < n_symbols {
            let ref mut fresh3 = *cdf.offset(i as isize);
            *fresh3 = (*fresh3 as libc::c_int - (*cdf.offset(i as isize) as libc::c_int >> rate))
                as uint16_t;
            i = i.wrapping_add(1);
        }
        *cdf.offset(n_symbols as isize) =
            count.wrapping_add((count < 32u32) as libc::c_uint) as uint16_t;
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_msac_decode_bool_adapt_c(
    s: *mut MsacContext,
    cdf: *mut uint16_t,
) -> libc::c_uint {
    let bit: libc::c_uint = dav1d_msac_decode_bool(s, *cdf as libc::c_uint);
    if (*s).allow_update_cdf != 0 {
        let count: libc::c_uint = *cdf.offset(1isize) as libc::c_uint;
        let rate: libc::c_int = (4u32).wrapping_add(count >> 4i32) as libc::c_int;
        if bit != 0 {
            let ref mut fresh4 = *cdf.offset(0isize);
            *fresh4 = (*fresh4 as libc::c_int
                + (32768i32 - *cdf.offset(0isize) as libc::c_int >> rate))
                as uint16_t;
        } else {
            let ref mut fresh5 = *cdf.offset(0isize);
            *fresh5 =
                (*fresh5 as libc::c_int - (*cdf.offset(0isize) as libc::c_int >> rate)) as uint16_t;
        }
        *cdf.offset(1isize) = count.wrapping_add((count < 32u32) as libc::c_uint) as uint16_t;
    }
    return bit;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_msac_decode_hi_tok_c(
    s: *mut MsacContext,
    cdf: *mut uint16_t,
) -> libc::c_uint {
    let mut tok_br: libc::c_uint = dav1d_msac_decode_symbol_adapt4(s, cdf, 3u64);
    let mut tok: libc::c_uint = (3u32).wrapping_add(tok_br);
    if tok_br == 3u32 {
        tok_br = dav1d_msac_decode_symbol_adapt4(s, cdf, 3u64);
        tok = (6u32).wrapping_add(tok_br);
        if tok_br == 3u32 {
            tok_br = dav1d_msac_decode_symbol_adapt4(s, cdf, 3u64);
            tok = (9u32).wrapping_add(tok_br);
            if tok_br == 3u32 {
                tok = (12u32).wrapping_add(dav1d_msac_decode_symbol_adapt4(s, cdf, 3u64));
            }
        }
    }
    return tok;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_msac_init(
    s: *mut MsacContext,
    data: *const uint8_t,
    sz: size_t,
    disable_cdf_update_flag: libc::c_int,
) {
    (*s).buf_pos = data;
    (*s).buf_end = data.offset(sz as isize);
    (*s).dif = ((1u64)
        << ((::core::mem::size_of::<ec_win>() as libc::c_ulong) << 3i32).wrapping_sub(1u64))
    .wrapping_sub(1u64);
    (*s).rng = 0x8000u32;
    (*s).cnt = -(15i32);
    (*s).allow_update_cdf = (disable_cdf_update_flag == 0) as libc::c_int;
    ctx_refill(s);

    //#[cfg(all(feature = "asm", target_arch = "x86_64"))]
    {
        (*s).symbol_adapt16 = Some(
            dav1d_msac_decode_symbol_adapt_c
                as unsafe extern "C" fn(*mut MsacContext, *mut uint16_t, size_t) -> libc::c_uint,
        );
        msac_init_x86(s);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_msac_decode_symbol_adapt4(
    mut s: *mut MsacContext,
    mut cdf: *mut uint16_t,
    mut n_symbols: size_t,
) -> libc::c_uint {
    cfg_if! {
        if # [cfg (all (feature = "asm", target_arch = "x86_64"))]
        { return dav1d_msac_decode_symbol_adapt4_sse2(s, cdf, n_symbols); } else
        { return dav1d_msac_decode_symbol_adapt_c (s, cdf, n_symbols) ; }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_msac_decode_symbol_adapt8(
    mut s: *mut MsacContext,
    mut cdf: *mut uint16_t,
    mut n_symbols: size_t,
) -> libc::c_uint {
    cfg_if! {
        if # [cfg (all (feature = "asm", target_arch = "x86_64"))]
        { return dav1d_msac_decode_symbol_adapt8_sse2(s, cdf, n_symbols); } else
        { return dav1d_msac_decode_symbol_adapt_c (s, cdf, n_symbols) ; }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_msac_decode_symbol_adapt16(
    mut s: *mut MsacContext,
    mut cdf: *mut uint16_t,
    mut n_symbols: size_t,
) -> libc::c_uint {
    cfg_if! {
        if # [cfg (all (feature = "asm", target_arch = "x86_64"))]
        {
            return (*s).symbol_adapt16.expect("non-null function pointer")(s, cdf,
                                                                   n_symbols);
        } else { return dav1d_msac_decode_symbol_adapt_c (s, cdf, n_symbols) ; }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_msac_decode_bool_adapt(
    mut s: *mut MsacContext,
    mut cdf: *mut uint16_t,
) -> libc::c_uint {
    cfg_if! {
        if # [cfg (all (feature = "asm", target_arch = "x86_64"))]
        { return dav1d_msac_decode_bool_adapt_sse2(s, cdf); } else
        { return dav1d_msac_decode_bool_adapt_c (s, cdf) ; }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_msac_decode_bool_equi(mut s: *mut MsacContext) -> libc::c_uint {
    cfg_if! {
        if # [cfg (all (feature = "asm", target_arch = "x86_64"))]
        { return dav1d_msac_decode_bool_equi_sse2(s); } else
        { return dav1d_msac_decode_bool_equi_c (s) ; }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_msac_decode_bool(
    mut s: *mut MsacContext,
    mut f: libc::c_uint,
) -> libc::c_uint {
    cfg_if! {
        if # [cfg (all (feature = "asm", target_arch = "x86_64"))]
        { return dav1d_msac_decode_bool_sse2(s, f); } else
        { return dav1d_msac_decode_bool_c (s, f) ; }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_msac_decode_hi_tok(
    mut s: *mut MsacContext,
    mut cdf: *mut uint16_t,
) -> libc::c_uint {
    cfg_if! {
        if # [cfg (all (feature = "asm", target_arch = "x86_64"))]
        { return dav1d_msac_decode_hi_tok_sse2(s, cdf); } else
        { return dav1d_msac_decode_hi_tok_c (s, cdf) ; }
    };
}
