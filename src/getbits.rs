use crate::include::common::intops::inv_recenter;
use crate::include::common::intops::ulog2;
use std::ffi::c_int;
use std::ffi::c_uint;

#[repr(C)]
pub struct GetBits {
    pub state: u64,
    pub bits_left: c_int,
    pub error: c_int,
    pub ptr: *const u8,
    pub ptr_start: *const u8,
    pub ptr_end: *const u8,
}

pub unsafe fn rav1d_init_get_bits(c: *mut GetBits, data: *const u8, sz: usize) {
    if sz == 0 {
        unreachable!();
    }
    (*c).ptr_start = data;
    (*c).ptr = (*c).ptr_start;
    (*c).ptr_end = &*((*c).ptr_start).offset(sz as isize) as *const u8;
    (*c).state = 0 as c_int as u64;
    (*c).bits_left = 0 as c_int;
    (*c).error = 0 as c_int;
}

pub unsafe fn rav1d_get_bit(c: *mut GetBits) -> c_uint {
    if (*c).bits_left == 0 {
        if (*c).ptr >= (*c).ptr_end {
            (*c).error = 1 as c_int;
        } else {
            let fresh0 = (*c).ptr;
            (*c).ptr = ((*c).ptr).offset(1);
            let state: c_uint = *fresh0 as c_uint;
            (*c).bits_left = 7 as c_int;
            (*c).state = (state as u64) << 57;
            return state >> 7;
        }
    }
    let state_0: u64 = (*c).state;
    (*c).bits_left -= 1;
    (*c).state = state_0 << 1;
    return (state_0 >> 63) as c_uint;
}

#[inline]
unsafe fn refill(c: *mut GetBits, n: c_int) {
    if !((*c).bits_left >= 0 && (*c).bits_left < 32) {
        unreachable!();
    }
    let mut state: c_uint = 0 as c_int as c_uint;
    loop {
        if (*c).ptr >= (*c).ptr_end {
            (*c).error = 1 as c_int;
            if state != 0 {
                break;
            }
            return;
        } else {
            let fresh1 = (*c).ptr;
            (*c).ptr = ((*c).ptr).offset(1);
            state = state << 8 | *fresh1 as c_uint;
            (*c).bits_left += 8 as c_int;
            if !(n > (*c).bits_left) {
                break;
            }
        }
    }
    (*c).state |= (state as u64) << 64 - (*c).bits_left;
}

pub unsafe fn rav1d_get_bits(c: *mut GetBits, n: c_int) -> c_uint {
    assert!(n > 0 && n <= 32);
    /* Unsigned cast avoids refill after eob */
    if n as c_uint > (*c).bits_left as c_uint {
        refill(c, n);
    }
    let state: u64 = (*c).state;
    (*c).bits_left -= n;
    (*c).state = state << n;
    return (state as u64 >> 64 - n) as c_uint;
}

pub unsafe fn rav1d_get_sbits(c: *mut GetBits, n: c_int) -> c_int {
    assert!(n > 0 && n <= 32);
    /* Unsigned cast avoids refill after eob */
    if n as c_uint > (*c).bits_left as c_uint {
        refill(c, n);
    }
    let state: u64 = (*c).state;
    (*c).bits_left -= n;
    (*c).state = state << n;
    return (state as i64 >> 64 - n) as c_int;
}

pub unsafe fn rav1d_get_uleb128(c: *mut GetBits) -> c_uint {
    let mut val: u64 = 0 as c_int as u64;
    let mut i: c_uint = 0 as c_int as c_uint;
    let mut more: c_uint;
    loop {
        let v = rav1d_get_bits(c, 8 as c_int) as c_int;
        more = (v & 0x80 as c_int) as c_uint;
        val |= ((v & 0x7f as c_int) as u64) << i;
        i = i.wrapping_add(7 as c_int as c_uint);
        if !(more != 0 && i < 56 as c_uint) {
            break;
        }
    }
    if val > u32::MAX as u64 || more != 0 {
        (*c).error = 1 as c_int;
        return 0;
    }
    return val as c_uint;
}

pub unsafe fn rav1d_get_uniform(c: *mut GetBits, max: c_uint) -> c_uint {
    if !(max > 1 as c_uint) {
        unreachable!();
    }
    let l = ulog2(max) + 1;
    if !(l > 1) {
        unreachable!();
    }
    let m: c_uint = ((1 as c_uint) << l).wrapping_sub(max);
    let v: c_uint = rav1d_get_bits(c, l - 1);
    return if v < m {
        v
    } else {
        (v << 1).wrapping_sub(m).wrapping_add(rav1d_get_bit(c))
    };
}

pub unsafe fn rav1d_get_vlc(c: *mut GetBits) -> c_uint {
    if rav1d_get_bit(c) != 0 {
        return 0 as c_int as c_uint;
    }
    let mut n_bits = 0;
    loop {
        n_bits += 1;
        if n_bits == 32 {
            return 0xffffffff as c_uint;
        }
        if !(rav1d_get_bit(c) == 0) {
            break;
        }
    }
    return ((1 as c_uint) << n_bits)
        .wrapping_sub(1 as c_int as c_uint)
        .wrapping_add(rav1d_get_bits(c, n_bits));
}

unsafe fn get_bits_subexp_u(c: *mut GetBits, r#ref: c_uint, n: c_uint) -> c_uint {
    let mut v: c_uint = 0 as c_int as c_uint;
    let mut i = 0;
    loop {
        let b = if i != 0 { 3 + i - 1 } else { 3 as c_int };
        if n < v.wrapping_add((3 * ((1 as c_int) << b)) as c_uint) {
            v = v.wrapping_add(rav1d_get_uniform(
                c,
                n.wrapping_sub(v).wrapping_add(1 as c_int as c_uint),
            ));
            break;
        } else if rav1d_get_bit(c) == 0 {
            v = v.wrapping_add(rav1d_get_bits(c, b));
            break;
        } else {
            v = v.wrapping_add(((1 as c_int) << b) as c_uint);
            i += 1;
        }
    }
    return if r#ref.wrapping_mul(2 as c_int as c_uint) <= n {
        inv_recenter(r#ref, v)
    } else {
        n.wrapping_sub(inv_recenter(n.wrapping_sub(r#ref), v))
    };
}

pub unsafe fn rav1d_get_bits_subexp(c: *mut GetBits, r#ref: c_int, n: c_uint) -> c_int {
    return get_bits_subexp_u(
        c,
        (r#ref + ((1 as c_int) << n)) as c_uint,
        ((2 as c_int) << n) as c_uint,
    ) as c_int
        - ((1 as c_int) << n);
}

pub unsafe fn rav1d_bytealign_get_bits(c: *mut GetBits) {
    if !((*c).bits_left <= 7) {
        unreachable!();
    }
    (*c).bits_left = 0 as c_int;
    (*c).state = 0 as c_int as u64;
}
