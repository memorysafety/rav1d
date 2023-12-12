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

impl GetBits {
    pub unsafe fn init(&mut self, data: *const u8, sz: usize) {
        assert!(sz != 0);
        self.ptr_start = data;
        self.ptr = self.ptr_start;
        self.ptr_end = self.ptr_start.add(sz);
        self.state = 0;
        self.bits_left = 0;
        self.error = 0;
    }

    pub unsafe fn get_bit(&mut self) -> c_uint {
        if self.bits_left == 0 {
            if self.ptr >= self.ptr_end {
                self.error = 1;
            } else {
                let state = *self.ptr as c_uint;
                self.ptr = self.ptr.add(1);
                self.bits_left = 7;
                self.state = (state as u64) << 57;
                return state >> 7;
            }
        }
        let state = self.state;
        self.bits_left -= 1;
        self.state = state << 1;
        (state >> 63) as c_uint
    }

    #[inline]
    unsafe fn refill(&mut self, n: c_int) {
        assert!(self.bits_left >= 0 && self.bits_left < 32);
        let mut state = 0;
        loop {
            if self.ptr >= self.ptr_end {
                self.error = 1;
                if state != 0 {
                    break;
                }
                return;
            } else {
                state = state << 8 | *self.ptr as c_uint;
                self.ptr = self.ptr.add(1);
                self.bits_left += 8;
                if !(n > self.bits_left) {
                    break;
                }
            }
        }
        self.state |= (state as u64) << 64 - self.bits_left;
    }

    pub unsafe fn get_bits(&mut self, n: c_int) -> c_uint {
        assert!(n > 0 && n <= 32);
        // Unsigned cast avoids refill after eob.
        if n as c_uint > self.bits_left as c_uint {
            self.refill(n);
        }
        let state = self.state;
        self.bits_left -= n;
        self.state = state << n;
        (state as u64 >> 64 - n) as c_uint
    }

    pub unsafe fn get_sbits(&mut self, n: c_int) -> c_int {
        assert!(n > 0 && n <= 32);
        // Unsigned cast avoids refill after eob.
        if n as c_uint > self.bits_left as c_uint {
            self.refill(n);
        }
        let state = self.state;
        self.bits_left -= n;
        self.state = state << n;
        (state as i64 >> 64 - n) as c_int
    }

    pub unsafe fn get_uleb128(&mut self) -> c_uint {
        let mut val = 0;
        let mut i = 0 as c_uint;
        let mut more;
        loop {
            let v = self.get_bits(8) as c_int;
            more = (v & 0x80) as c_uint;
            val |= ((v & 0x7f) as u64) << i;
            i += 7;
            if !(more != 0 && i < 56) {
                break;
            }
        }
        if val > u32::MAX as u64 || more != 0 {
            self.error = 1;
            return 0;
        }
        val as c_uint
    }

    pub unsafe fn get_uniform(&mut self, max: c_uint) -> c_uint {
        assert!(max > 1);
        let l = ulog2(max) + 1;
        assert!(l > 1);
        let m = (1 << l) - max;
        let v = self.get_bits(l - 1);
        if v < m {
            v
        } else {
            (v << 1) - m + self.get_bit()
        }
    }

    pub unsafe fn get_vlc(&mut self) -> c_uint {
        if self.get_bit() != 0 {
            return 0;
        }
        let mut n_bits = 0;
        loop {
            n_bits += 1;
            if n_bits == 32 {
                return 0xffffffff;
            }
            if !(self.get_bit() == 0) {
                break;
            }
        }
        (1 << n_bits) - 1 + self.get_bits(n_bits)
    }

    unsafe fn get_bits_subexp_u(&mut self, r#ref: c_uint, n: c_uint) -> c_uint {
        let mut v = 0 as c_uint;
        let mut i = 0;
        loop {
            let b = if i != 0 { 3 + i - 1 } else { 3 };
            if n < v + (3 * (1 << b)) {
                v += self.get_uniform(n - v + 1);
                break;
            } else if self.get_bit() == 0 {
                v += self.get_bits(b);
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

    pub unsafe fn get_bits_subexp(&mut self, r#ref: c_int, n: c_uint) -> c_int {
        self.get_bits_subexp_u((r#ref + (1 << n)) as c_uint, 2 << n) as c_int - (1 << n)
    }

    pub fn bytealign(&mut self) {
        assert!(self.bits_left <= 7);
        self.bits_left = 0;
        self.state = 0;
    }
}
