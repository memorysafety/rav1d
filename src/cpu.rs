use crate::src::const_fn::const_for;
use bitflags::bitflags;
use std::ffi::c_uint;
use std::num::NonZero;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::thread::available_parallelism;

#[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "riscv32",
    target_arch = "riscv64",
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
        const DOTPROD = 1 << 1;
        const I8MM = 1 << 2;
        const SVE = 1 << 3;
        const SVE2 = 1 << 4;
    }
}

#[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
bitflags! {
    #[derive(Clone, Copy)]
    pub struct CpuFlags: c_uint {
        const V = 1 << 0;
    }
}

impl CpuFlags {
    pub const fn compile_time_detect() -> Self {
        let individual_flags = [
            #[cfg(target_feature = "sse2")]
            CpuFlags::SSE2,
            #[cfg(target_feature = "sse3")]
            CpuFlags::SSSE3,
            #[cfg(target_feature = "sse4.1")]
            CpuFlags::SSE41,
            #[cfg(target_feature = "avx2")]
            CpuFlags::AVX2,
            #[cfg(all(
                target_feature = "avx512f",
                target_feature = "avx512cd",
                target_feature = "avx512bw",
                target_feature = "avx512dq",
                target_feature = "avx512vl",
                target_feature = "avx512vnni",
                target_feature = "avx512ifma",
                target_feature = "avx512vbmi",
                target_feature = "avx512vbmi2",
                target_feature = "avx512vpopcntdq",
                target_feature = "avx512bitalg",
                target_feature = "gfni",
                target_feature = "vaes",
                target_feature = "vpclmulqdq",
            ))]
            CpuFlags::AVX512ICL,
            #[cfg(target_feature = "neon")]
            CpuFlags::NEON,
            #[cfg(target_feature = "i8mm")]
            CpuFlags::I8MM,
            #[cfg(target_feature = "dotprod")]
            CpuFlags::DOTPROD,
            #[cfg(target_feature = "sve")]
            CpuFlags::SVE,
            #[cfg(target_feature = "sve2")]
            CpuFlags::SVE2,
            #[cfg(target_feature = "v")]
            CpuFlags::V,
        ];

        let mut combined_flags = Self::empty();
        const_for!(i in 0..individual_flags.len() => {
            combined_flags = combined_flags.union(individual_flags[i]);
        });
        combined_flags
    }

    #[cfg(not(feature = "asm"))]
    pub fn run_time_detect() -> Self {
        Self::empty()
    }

    #[cfg(feature = "asm")]
    pub fn run_time_detect() -> Self {
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

            /// Detect Excavator, Zen, Zen+, Zen 2, Zen 3, Zen 3+, Zen 4.
            fn is_slow_gather() -> Option<()> {
                let cpu_id = CpuId::new();

                let vendor = cpu_id.get_vendor_info()?;
                let is_amd = vendor.as_str() == "AuthenticAMD";
                if !is_amd {
                    return None;
                }

                let features = cpu_id.get_feature_info()?;
                let family = features.family_id();

                (family <= 0x19).then_some(())
            }
            if flags.contains(Self::AVX2) && is_slow_gather().is_some() {
                flags |= Self::SLOW_GATHER;
            }
        }

        #[cfg(target_arch = "arm")]
        {
            if std::arch::is_arm_feature_detected!("neon") {
                flags |= Self::NEON;
            }
            if std::arch::is_arm_feature_detected!("dotprod") {
                flags |= Self::DOTPROD;
            }
            if std::arch::is_arm_feature_detected!("i8mm") {
                flags |= Self::I8MM;
            }
        }

        #[cfg(target_arch = "aarch64")]
        {
            if std::arch::is_aarch64_feature_detected!("neon") {
                flags |= Self::NEON;
            }
            if std::arch::is_aarch64_feature_detected!("dotprod") {
                flags |= Self::DOTPROD;
            }
            if std::arch::is_aarch64_feature_detected!("i8mm") {
                flags |= Self::I8MM;
            }
            if std::arch::is_aarch64_feature_detected!("sve") {
                flags |= Self::SVE;
            }
            if std::arch::is_aarch64_feature_detected!("sve2") {
                flags |= Self::SVE2;
            }
        }

        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        if std::arch::is_riscv_feature_detected!("v") {
            flags |= Self::V;
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
static rav1d_cpu_flags: AtomicU32 = AtomicU32::new(0);

/// This is atomic, which has interior mutability,
/// instead of a `static mut`, since the latter is `unsafe` to access.
///
/// It is modifiable through the publicly exported [`dav1d_set_cpu_flags_mask`],
/// so strict safety guarantees about how it's used can't be made.
/// Other than that, it is also only used in init functions (that call [`dav1d_get_cpu_flags`]),
/// so it shouldn't be performance sensitive.
static rav1d_cpu_flags_mask: AtomicU32 = AtomicU32::new(!0);

#[inline(always)]
pub(crate) fn rav1d_get_cpu_flags() -> CpuFlags {
    let flags =
        rav1d_cpu_flags.load(Ordering::SeqCst) & rav1d_cpu_flags_mask.load(Ordering::SeqCst);
    // Note that `bitflags!` `struct`s are `#[repr(transparent)]`.
    CpuFlags::from_bits_truncate(flags) | CpuFlags::compile_time_detect()
}

#[cold]
pub(crate) fn rav1d_init_cpu() {
    rav1d_cpu_flags.store(CpuFlags::run_time_detect().bits(), Ordering::SeqCst);
}

#[cold]
pub fn rav1d_set_cpu_flags_mask(mask: c_uint) {
    rav1d_cpu_flags_mask.store(mask, Ordering::SeqCst);
}

#[no_mangle]
#[cold]
pub extern "C" fn dav1d_set_cpu_flags_mask(mask: c_uint) {
    rav1d_set_cpu_flags_mask(mask)
}

#[cold]
pub(crate) fn rav1d_num_logical_processors() -> NonZero<usize> {
    available_parallelism().unwrap_or(NonZero::new(1).unwrap())
}
