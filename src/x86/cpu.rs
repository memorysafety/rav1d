use bitflags::bitflags;
use raw_cpuid::CpuId;
use std::ffi::c_uint;

bitflags! {
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

#[cold]
pub fn dav1d_get_cpu_flags_x86() -> CpuFlags {
    let mut flags = CpuFlags::empty();

    if is_x86_feature_detected!("sse2") {
        flags |= CpuFlags::SSE2;
    }
    if is_x86_feature_detected!("ssse3") {
        flags |= CpuFlags::SSSE3;
    }
    if is_x86_feature_detected!("sse4.1") {
        flags |= CpuFlags::SSE41;
    }
    if is_x86_feature_detected!("avx2") {
        flags |= CpuFlags::AVX2;
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
        flags |= CpuFlags::AVX512ICL;
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

        (family < 0x19 || (family == 0x19 && (model < 0x10 || (model >= 0x20 && model < 0x60))))
            .then_some(())
    }
    if flags.contains(CpuFlags::AVX2) && is_slow_gather().is_some() {
        flags |= CpuFlags::SLOW_GATHER;
    }

    flags
}
