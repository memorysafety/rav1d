use std::ffi::c_ulong;

pub type __cpu_mask = c_ulong;

#[repr(C)]
pub struct cpu_set_t {
    pub __bits: [__cpu_mask; 16],
}
