use bitflags::bitflags;
use cfg_if::cfg_if;
use std::ffi::c_uint;
use std::ffi::c_ulong;

bitflags! {
    pub struct CpuFlags: c_uint {
        const NEON = 1 << 0;
    }
}

pub const NEON_HWCAP: c_ulong = 1 << 12;

#[cold]
pub unsafe fn dav1d_get_cpu_flags_arm() -> CpuFlags {
    let mut flags = CpuFlags::empty();

    cfg_if! {
        if #[cfg(any(
            target_arch = "aarch64",
            target_os = "windows",
            target_os = "macos"
        ))] {
            flags |= CpuFlags::NEON;
        } else if #[cfg(target_arch = "arm")] {
            if (libc::getauxval(libc::AT_HWCAP) & NEON_HWCAP) != 0 {
                flags |= CpuFlags::NEON;
            }
        } else if #[cfg(target_os = "android")] {
            // TODO: Support Android by parsing `/proc/cpuinfo` the way the original C does.
            todo!("Android is not yet supported")
        }
    }

    flags
}
