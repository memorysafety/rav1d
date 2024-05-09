use crate::include::common::intops::inv_recenter;
use crate::include::common::intops::ulog2;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ops::Index;

#[repr(C)]
pub struct GetBits<'a> {
    state: u64,
    bits_left: c_int,
    error: c_int,
    index: usize,
    data: &'a [u8],
}

impl<'a> GetBits<'a> {
    pub const fn new(data: &'a [u8]) -> Self {
        assert!(!data.is_empty());
        Self {
            state: 0,
            bits_left: 0,
            error: 0,
            index: 0,
            data,
        }
    }

    pub const fn has_error(&self) -> c_int {
        self.error
    }

    pub fn get_bit(&mut self) -> bool {
        if self.bits_left == 0 {
            if self.index >= self.data.len() {
                self.error = 1;
            } else {
                let state = self.data[self.index];
                self.index += 1;
                self.bits_left = 7;
                self.state = (state as u64) << 57;
                return (state >> 7) != 0;
            }
        }
        let state = self.state;
        self.bits_left -= 1;
        self.state = state << 1;
        (state >> 63) != 0
    }

    #[inline]
    fn refill(&mut self, n: c_int) {
        assert!(self.bits_left >= 0 && self.bits_left < 32);
        let mut state = 0;
        loop {
            if self.index >= self.data.len() {
                self.error = 1;
                if state != 0 {
                    break;
                }
                return;
            } else {
                state = (state << 8) | self.data[self.index] as c_uint;
                self.index += 1;
                self.bits_left += 8;
                if !(n > self.bits_left) {
                    break;
                }
            }
        }
        self.state |= (state as u64) << 64 - self.bits_left;
    }

    pub fn get_bits(&mut self, n: c_int) -> c_uint {
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

    pub fn get_sbits(&mut self, n: c_int) -> c_int {
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

    pub fn get_uleb128(&mut self) -> c_uint {
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

    pub fn get_uniform(&mut self, max: c_uint) -> c_uint {
        assert!(max > 1);
        let l = ulog2(max) + 1;
        assert!(l > 1);
        let m = (1 << l) - max;
        let v = self.get_bits(l - 1);
        if v < m {
            v
        } else {
            (v << 1) - m + self.get_bit() as c_uint
        }
    }

    pub fn get_vlc(&mut self) -> c_uint {
        if self.get_bit() {
            return 0;
        }
        let mut n_bits = 0;
        loop {
            n_bits += 1;
            if n_bits == 32 {
                return 0xffffffff;
            }
            if self.get_bit() {
                break;
            }
        }
        (1 << n_bits) - 1 + self.get_bits(n_bits)
    }

    fn get_bits_subexp_u(&mut self, r#ref: c_uint, n: c_uint) -> c_uint {
        let mut v = 0 as c_uint;
        let mut i = 0;
        loop {
            let b = if i != 0 { 3 + i - 1 } else { 3 };
            if n < v + (3 * (1 << b)) {
                v += self.get_uniform(n - v + 1);
                break;
            } else if !self.get_bit() {
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

    pub fn get_bits_subexp(&mut self, r#ref: c_int, n: c_uint) -> c_int {
        self.get_bits_subexp_u((r#ref + (1 << n)) as c_uint, 2 << n) as c_int - (1 << n)
    }

    // Discard bits from the buffer until we're next byte-aligned.
    #[inline]
    pub fn bytealign(&mut self) {
        // `bits_left` is never more than 7, because it is only incremented
        // by `refill()`, called by `get_bits` and that never reads more
        // than 7 bits more than it needs.
        //
        // If this wasn't true, we would need to work out how many bits to
        // discard `(bits_left % 8)`, subtract that from `bits_left` and then
        // shift `state` right by that amount.
        assert!(self.bits_left <= 7);
        self.bits_left = 0;
        self.state = 0;
    }

    #[inline]
    pub const fn pos(&self) -> usize {
        self.index * u8::BITS as usize - self.bits_left as usize
    }

    pub const fn byte_pos(&self) -> usize {
        self.index
    }

    pub const fn is_byte_aligned(&self) -> bool {
        self.bits_left == 0
    }

    pub const fn pending_bits(&self) -> u64 {
        self.state
    }

    pub const fn has_pending_bits(&self) -> bool {
        self.state != 0 || self.bits_left != 0
    }

    pub fn get_bytes(&mut self, n: usize) -> &[u8] {
        assert_eq!(self.bits_left, 0);
        let i = self.index;
        self.index += n;
        &self.data[i..][..n]
    }

    pub fn set_remaining_len(&mut self, len: usize) -> Option<()> {
        self.data = self.data.get(..self.index + len)?;
        Some(())
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn remaining_len(&self) -> usize {
        self.data.len() - self.index
    }
}

impl<'a> Index<usize> for GetBits<'a> {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(self.index + index)
    }
}
