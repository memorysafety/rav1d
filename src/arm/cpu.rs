use bitflags::bitflags;
use std::arch;
use std::ffi::c_uint;

bitflags! {
    pub struct CpuFlags: c_uint {
        const NEON = 1 << 0;
    }
}

#[cold]
pub unsafe fn dav1d_get_cpu_flags_arm() -> CpuFlags {
    let mut flags = CpuFlags::empty();

    #[cfg(target_arch = "arm")]
    if arch::is_arm_feature_detected!("neon") {
        flags |= CpuFlags::NEON;
    }

    #[cfg(target_arch = "aarch64")]
    if arch::is_aarch64_feature_detected!("neon") {
        flags |= CpuFlags::NEON;
    }

    flags
}
