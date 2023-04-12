use crate::include::stdint::uint16_t;
use crate::include::stdint::uint8_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub union alias8 {
    pub u8_0: uint8_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union alias16 {
    pub u16_0: uint16_t,
    pub u8_0: [uint8_t; 2],
}
