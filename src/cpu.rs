use crate::src::internal::Dav1dContext;
use bitflags::bitflags;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

#[cfg(feature = "asm")]
use cfg_if::cfg_if;

#[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "arm",
    target_arch = "aarch64"
)))]
bitflags! {
    #[derive(Clone, Copy)]
    pub struct CpuFlags: c_uint {}
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
bitflags! {
    #[derive(Clone, Copy)]
    pub struct CpuFlags: c_uint {
        const SSE2 = 1 << 0;
        const SSSE3 = 1 << 1;
        const SSE41 = 1 << 2;
        const AVX2 = 1 << 3;

        /// F/CD/BW/DQ/VL/VNNI/IFMA/VBMI/VBMI2/
        /// VPOPCNTDQ/BITALG/GFNI/VAES/VPCLMULQDQ
        const AVX512ICL = 1 << 4;

        /// Flag CPUs where gather instructions are
        /// slow enough to cause performance regressions.
        const SLOW_GATHER = 1 << 5;
    }
}

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
bitflags! {
    #[derive(Clone, Copy)]
    pub struct CpuFlags: c_uint {
        const NEON = 1 << 0;
    }
}

impl CpuFlags {
    #[cfg(not(feature = "asm"))]
    pub fn detect() -> Self {
        Self::empty()
    }

    #[cfg(feature = "asm")]
    pub fn detect() -> Self {
        let mut flags = Self::empty();

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            use raw_cpuid::CpuId;

            if is_x86_feature_detected!("sse2") {
                flags |= Self::SSE2;
            }
            if is_x86_feature_detected!("ssse3") {
                flags |= Self::SSSE3;
            }
            if is_x86_feature_detected!("sse4.1") {
                flags |= Self::SSE41;
            }
            if is_x86_feature_detected!("avx2") {
                flags |= Self::AVX2;
            }
            if is_x86_feature_detected!("avx512f")
                && is_x86_feature_detected!("avx512cd")
                && is_x86_feature_detected!("avx512bw")
                && is_x86_feature_detected!("avx512dq")
                && is_x86_feature_detected!("avx512vl")
                && is_x86_feature_detected!("avx512vnni")
                && is_x86_feature_detected!("avx512ifma")
                && is_x86_feature_detected!("avx512vbmi")
                && is_x86_feature_detected!("avx512vbmi2")
                && is_x86_feature_detected!("avx512vpopcntdq")
                && is_x86_feature_detected!("avx512bitalg")
                && is_x86_feature_detected!("gfni")
                && is_x86_feature_detected!("vaes")
                && is_x86_feature_detected!("vpclmulqdq")
            {
                flags |= Self::AVX512ICL;
            }

            /// Detect Excavator, Zen, Zen+, Zen 2, Zen 3, Zen 3+.
            fn is_slow_gather() -> Option<()> {
                let cpu_id = CpuId::new();

                let vendor = cpu_id.get_vendor_info()?;
                let is_amd = vendor.as_str() == "AuthenticAMD";
                if !is_amd {
                    return None;
                }

                let features = cpu_id.get_feature_info()?;
                let family = features.family_id();
                let model = features.model_id();

                (family < 0x19
                    || (family == 0x19 && (model < 0x10 || (model >= 0x20 && model < 0x60))))
                    .then_some(())
            }
            if flags.contains(Self::AVX2) && is_slow_gather().is_some() {
                flags |= Self::SLOW_GATHER;
            }
        }

        #[cfg(target_arch = "arm")]
        if std::arch::is_arm_feature_detected!("neon") {
            flags |= Self::NEON;
        }

        #[cfg(target_arch = "aarch64")]
        if std::arch::is_aarch64_feature_detected!("neon") {
            flags |= Self::NEON;
        }

        flags
    }
}

/// This is atomic, which has interior mutability,
/// instead of a `static mut`, since the latter is `unsafe` to access.
///
/// It seems to only be used in init functions,
/// should it shouldn't be performance sensitive.
///
/// It is written once by [`dav1d_init_cpu`] in initialization code,
/// and then subsequently read in [`dav1d_get_cpu_flags`] by other initialization code.
static dav1d_cpu_flags: AtomicU32 = AtomicU32::new(0);

/// This is atomic, which has interior mutability,
/// instead of a `static mut`, since the latter is `unsafe` to access.
///
/// It is modifiable through the publicly exported [`dav1d_set_cpu_flags_mask`],
/// so strict safety guarantees about how it's used can't be made.
/// Other than that, it is also only used in init functions (that call [`dav1d_get_cpu_flags`]),
/// so it shouldn't be performance sensitive.
static dav1d_cpu_flags_mask: AtomicU32 = AtomicU32::new(!0);

#[cfg(feature = "asm")]
#[inline(always)]
pub(crate) fn dav1d_get_cpu_flags() -> CpuFlags {
    let flags =
        dav1d_cpu_flags.load(Ordering::SeqCst) & dav1d_cpu_flags_mask.load(Ordering::SeqCst);
    // Note that `bitflags!` `struct`s are `#[repr(transparent)]`.
    let mut flags = CpuFlags::from_bits_truncate(flags);
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            flags |= CpuFlags::SSE2;
        } else {
            // For `unused_mut`.
            flags |= CpuFlags::empty();
        }
    }
    flags
}

#[cold]
pub(crate) fn dav1d_init_cpu() {
    dav1d_cpu_flags.store(CpuFlags::detect().bits(), Ordering::SeqCst);
}

#[no_mangle]
#[cold]
pub extern "C" fn dav1d_set_cpu_flags_mask(mask: c_uint) {
    dav1d_cpu_flags_mask.store(mask, Ordering::SeqCst);
}

#[cold]
pub(crate) fn dav1d_num_logical_processors(_c: *mut Dav1dContext) -> c_int {
    num_cpus::get() as c_int
}
