pub type __cpu_mask = libc::c_ulong;

#[repr(C)]
pub struct cpu_set_t {
    pub __bits: [__cpu_mask; 16],
}
