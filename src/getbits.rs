use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;



#[derive(Copy, Clone)]
#[repr(C)]
pub struct GetBits {
    pub state: uint64_t,
    pub bits_left: libc::c_int,
    pub error: libc::c_int,
    pub ptr: *const uint8_t,
    pub ptr_start: *const uint8_t,
    pub ptr_end: *const uint8_t,
}
#[inline]
unsafe extern "C" fn inv_recenter(r: libc::c_uint, v: libc::c_uint) -> libc::c_uint {
    if v > r << 1 as libc::c_int {
        return v
    } else if v & 1 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        return (v >> 1 as libc::c_int).wrapping_add(r)
    } else {
        return r
            .wrapping_sub(
                v.wrapping_add(1 as libc::c_int as libc::c_uint) >> 1 as libc::c_int,
            )
    };
}
#[inline]
unsafe extern "C" fn ulog2(v: libc::c_uint) -> libc::c_int {
    return 31 as libc::c_int - clz(v);
}
#[inline]
unsafe extern "C" fn clz(mask: libc::c_uint) -> libc::c_int {
    return mask.leading_zeros() as i32;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_init_get_bits(
    c: *mut GetBits,
    data: *const uint8_t,
    sz: size_t,
) {
    if sz == 0 {
        unreachable!();
    }
    (*c).ptr_start = data;
    (*c).ptr = (*c).ptr_start;
    (*c).ptr_end = &*((*c).ptr_start).offset(sz as isize) as *const uint8_t;
    (*c).state = 0 as libc::c_int as uint64_t;
    (*c).bits_left = 0 as libc::c_int;
    (*c).error = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_bit(c: *mut GetBits) -> libc::c_uint {
    if (*c).bits_left == 0 {
        if (*c).ptr >= (*c).ptr_end {
            (*c).error = 1 as libc::c_int;
        } else {
            let fresh0 = (*c).ptr;
            (*c).ptr = ((*c).ptr).offset(1);
            let state: libc::c_uint = *fresh0 as libc::c_uint;
            (*c).bits_left = 7 as libc::c_int;
            (*c).state = (state as uint64_t) << 57 as libc::c_int;
            return state >> 7 as libc::c_int;
        }
    }
    let state_0: uint64_t = (*c).state;
    (*c).bits_left -= 1;
    (*c).state = state_0 << 1 as libc::c_int;
    return (state_0 >> 63 as libc::c_int) as libc::c_uint;
}
#[inline]
unsafe extern "C" fn refill(c: *mut GetBits, n: libc::c_int) {
    if !((*c).bits_left >= 0 as libc::c_int && (*c).bits_left < 32 as libc::c_int) {
        unreachable!();
    }
    let mut state: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop {
        if (*c).ptr >= (*c).ptr_end {
            (*c).error = 1 as libc::c_int;
            if state != 0 {
                break;
            }
            return;
        } else {
            let fresh1 = (*c).ptr;
            (*c).ptr = ((*c).ptr).offset(1);
            state = state << 8 as libc::c_int | *fresh1 as libc::c_uint;
            (*c).bits_left += 8 as libc::c_int;
            if !(n > (*c).bits_left) {
                break;
            }
        }
    }
    (*c).state |= (state as uint64_t) << 64 as libc::c_int - (*c).bits_left;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_bits(
    c: *mut GetBits,
    n: libc::c_int,
) -> libc::c_uint {
    assert!(n > 0 && n <= 32);
    /* Unsigned cast avoids refill after eob */
    if n as libc::c_uint > (*c).bits_left as libc::c_uint {
        refill(c, n);
    }
    let state: uint64_t = (*c).state;
    (*c).bits_left -= n;
    (*c).state = state << n;
    return (state as uint64_t >> 64 - n) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_sbits(
    c: *mut GetBits,
    n: libc::c_int,
) -> libc::c_int {
    assert!(n > 0 && n <= 32);
    /* Unsigned cast avoids refill after eob */
    if n as libc::c_uint > (*c).bits_left as libc::c_uint {
        refill(c, n);
    }
    let state: uint64_t = (*c).state;
    (*c).bits_left -= n;
    (*c).state = state << n;
    return (state as int64_t >> 64 - n) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_uleb128(c: *mut GetBits) -> libc::c_uint {
    let mut val: uint64_t = 0 as libc::c_int as uint64_t;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut more: libc::c_uint = 0;
    loop {
        let v: libc::c_int = dav1d_get_bits(c, 8 as libc::c_int) as libc::c_int;
        more = (v & 0x80 as libc::c_int) as libc::c_uint;
        val |= ((v & 0x7f as libc::c_int) as uint64_t) << i;
        i = i.wrapping_add(7 as libc::c_int as libc::c_uint);
        if !(more != 0 && i < 56 as libc::c_int as libc::c_uint) {
            break;
        }
    }
    if val > u32::MAX as uint64_t || more != 0
    {
        (*c).error = 1 as libc::c_int;
        return 0;
    }
    return val as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_uniform(
    c: *mut GetBits,
    max: libc::c_uint,
) -> libc::c_uint {
    if !(max > 1 as libc::c_int as libc::c_uint) {
        unreachable!();
    }
    let l: libc::c_int = ulog2(max) + 1 as libc::c_int;
    if !(l > 1 as libc::c_int) {
        unreachable!();
    }
    let m: libc::c_uint = ((1 as libc::c_uint) << l).wrapping_sub(max);
    let v: libc::c_uint = dav1d_get_bits(c, l - 1 as libc::c_int);
    return if v < m {
        v
    } else {
        (v << 1 as libc::c_int).wrapping_sub(m).wrapping_add(dav1d_get_bit(c))
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_vlc(c: *mut GetBits) -> libc::c_uint {
    if dav1d_get_bit(c) != 0 {
        return 0 as libc::c_int as libc::c_uint;
    }
    let mut n_bits: libc::c_int = 0 as libc::c_int;
    loop {
        n_bits += 1;
        if n_bits == 32 as libc::c_int {
            return 0xffffffff as libc::c_uint;
        }
        if !(dav1d_get_bit(c) == 0) {
            break;
        }
    }
    return ((1 as libc::c_uint) << n_bits)
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        .wrapping_add(dav1d_get_bits(c, n_bits));
}
unsafe extern "C" fn get_bits_subexp_u(
    c: *mut GetBits,
    ref_0: libc::c_uint,
    n: libc::c_uint,
) -> libc::c_uint {
    let mut v: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_int = 0 as libc::c_int;
    loop {
        let b: libc::c_int = if i != 0 {
            3 as libc::c_int + i - 1 as libc::c_int
        } else {
            3 as libc::c_int
        };
        if n
            < v
                .wrapping_add(
                    (3 as libc::c_int * ((1 as libc::c_int) << b)) as libc::c_uint,
                )
        {
            v = v
                .wrapping_add(
                    dav1d_get_uniform(
                        c,
                        n.wrapping_sub(v).wrapping_add(1 as libc::c_int as libc::c_uint),
                    ),
                );
            break;
        } else if dav1d_get_bit(c) == 0 {
            v = v.wrapping_add(dav1d_get_bits(c, b));
            break;
        } else {
            v = v.wrapping_add(((1 as libc::c_int) << b) as libc::c_uint);
            i += 1;
        }
    }
    return if ref_0.wrapping_mul(2 as libc::c_int as libc::c_uint) <= n {
        inv_recenter(ref_0, v)
    } else {
        n.wrapping_sub(inv_recenter(n.wrapping_sub(ref_0), v))
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_bits_subexp(
    c: *mut GetBits,
    ref_0: libc::c_int,
    n: libc::c_uint,
) -> libc::c_int {
    return get_bits_subexp_u(
        c,
        (ref_0 + ((1 as libc::c_int) << n)) as libc::c_uint,
        ((2 as libc::c_int) << n) as libc::c_uint,
    ) as libc::c_int - ((1 as libc::c_int) << n);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_bytealign_get_bits(mut c: *mut GetBits) {
    if !((*c).bits_left <= 7 as libc::c_int) {
        unreachable!();
    }
    (*c).bits_left = 0 as libc::c_int;
    (*c).state = 0 as libc::c_int as uint64_t;
}
