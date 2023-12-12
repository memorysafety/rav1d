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
    assert!(sz != 0);
    (*c).ptr_start = data;
    (*c).ptr = (*c).ptr_start;
    (*c).ptr_end = (*c).ptr_start.add(sz);
    (*c).state = 0;
    (*c).bits_left = 0;
    (*c).error = 0;
}

pub unsafe fn rav1d_get_bit(c: *mut GetBits) -> c_uint {
    if (*c).bits_left == 0 {
        if (*c).ptr >= (*c).ptr_end {
            (*c).error = 1;
        } else {
            let state = *(*c).ptr as c_uint;
            (*c).ptr = (*c).ptr.add(1);
            (*c).bits_left = 7;
            (*c).state = (state as u64) << 57;
            return state >> 7;
        }
    }
    let state = (*c).state;
    (*c).bits_left -= 1;
    (*c).state = state << 1;
    (state >> 63) as c_uint
}

#[inline]
unsafe fn refill(c: *mut GetBits, n: c_int) {
    assert!((*c).bits_left >= 0 && (*c).bits_left < 32);
    let mut state = 0;
    loop {
        if (*c).ptr >= (*c).ptr_end {
            (*c).error = 1;
            if state != 0 {
                break;
            }
            return;
        } else {
            state = state << 8 | *(*c).ptr as c_uint;
            (*c).ptr = (*c).ptr.add(1);
            (*c).bits_left += 8;
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
    let state = (*c).state;
    (*c).bits_left -= n;
    (*c).state = state << n;
    (state as u64 >> 64 - n) as c_uint
}

pub unsafe fn rav1d_get_sbits(c: *mut GetBits, n: c_int) -> c_int {
    assert!(n > 0 && n <= 32);
    /* Unsigned cast avoids refill after eob */
    if n as c_uint > (*c).bits_left as c_uint {
        refill(c, n);
    }
    let state = (*c).state;
    (*c).bits_left -= n;
    (*c).state = state << n;
    (state as i64 >> 64 - n) as c_int
}

pub unsafe fn rav1d_get_uleb128(c: *mut GetBits) -> c_uint {
    let mut val = 0;
    let mut i = 0 as c_uint;
    let mut more;
    loop {
        let v = rav1d_get_bits(c, 8) as c_int;
        more = (v & 0x80) as c_uint;
        val |= ((v & 0x7f) as u64) << i;
        i += 7;
        if !(more != 0 && i < 56) {
            break;
        }
    }
    if val > u32::MAX as u64 || more != 0 {
        (*c).error = 1;
        return 0;
    }
    val as c_uint
}

pub unsafe fn rav1d_get_uniform(c: *mut GetBits, max: c_uint) -> c_uint {
    assert!(max > 1);
    let l = ulog2(max) + 1;
    assert!(l > 1);
    let m = (1 << l) - max;
    let v = rav1d_get_bits(c, l - 1);
    if v < m {
        v
    } else {
        (v << 1) - m + rav1d_get_bit(c)
    }
}

pub unsafe fn rav1d_get_vlc(c: *mut GetBits) -> c_uint {
    if rav1d_get_bit(c) != 0 {
        return 0;
    }
    let mut n_bits = 0;
    loop {
        n_bits += 1;
        if n_bits == 32 {
            return 0xffffffff;
        }
        if !(rav1d_get_bit(c) == 0) {
            break;
        }
    }
    (1 << n_bits) - 1 + rav1d_get_bits(c, n_bits)
}

unsafe fn get_bits_subexp_u(c: *mut GetBits, r#ref: c_uint, n: c_uint) -> c_uint {
    let mut v = 0 as c_uint;
    let mut i = 0;
    loop {
        let b = if i != 0 { 3 + i - 1 } else { 3 };
        if n < v + (3 * (1 << b)) {
            v += rav1d_get_uniform(c, n - v + 1);
            break;
        } else if rav1d_get_bit(c) == 0 {
            v += rav1d_get_bits(c, b);
            break;
        } else {
            v += 1 << b;
            i += 1;
        }
    }
    if r#ref * 2 <= n {
        inv_recenter(r#ref, v)
    } else {
        n - inv_recenter(n - r#ref, v)
    }
}

pub unsafe fn rav1d_get_bits_subexp(c: *mut GetBits, r#ref: c_int, n: c_uint) -> c_int {
    get_bits_subexp_u(c, (r#ref + (1 << n)) as c_uint, 2 << n) as c_int - (1 << n)
}

pub unsafe fn rav1d_bytealign_get_bits(c: *mut GetBits) {
    assert!((*c).bits_left <= 7);
    (*c).bits_left = 0;
    (*c).state = 0;
}
