pub type CpuFlags = libc::c_uint;
pub const DAV1D_ARM_CPU_FLAG_NEON: CpuFlags = 1 << 0;

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_get_cpu_flags_arm() -> libc::c_uint {
    let mut flags: libc::c_uint = 0;

    // #[cfg(target_arch = "aarch64")]
    flags |= DAV1D_ARM_CPU_FLAG_NEON;
    // TODO: handle other supported arm platforms such as Android

    return flags;
}