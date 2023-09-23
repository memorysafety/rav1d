use bitflags::bitflags;
use libc::memcmp;
use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_void;

extern "C" {
    fn dav1d_cpu_cpuid(regs: *mut CpuidRegisters, leaf: c_uint, subleaf: c_uint);
}

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

#[derive(Clone, Copy)]
#[repr(C)]
pub struct CpuidRegisters {
    pub eax: u32,
    pub ebx: u32,
    pub edx: u32,
    pub ecx: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub max_leaf: u32,
    pub vendor: [c_char; 12],
}

#[repr(C)]
pub union C2RustUnnamed_0 {
    pub r: CpuidRegisters,
    pub c2rust_unnamed: C2RustUnnamed,
}

#[cold]
pub unsafe fn dav1d_get_cpu_flags_x86() -> CpuFlags {
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

    let mut cpu: C2RustUnnamed_0 = C2RustUnnamed_0 {
        r: CpuidRegisters {
            eax: 0,
            ebx: 0,
            edx: 0,
            ecx: 0,
        },
    };
    dav1d_cpu_cpuid(&mut cpu.r, 0 as c_int as c_uint, 0 as c_int as c_uint);
    if cpu.c2rust_unnamed.max_leaf >= 1 as c_uint {
        let mut r: CpuidRegisters = CpuidRegisters {
            eax: 0,
            ebx: 0,
            edx: 0,
            ecx: 0,
        };
        dav1d_cpu_cpuid(&mut r, 1 as c_int as c_uint, 0 as c_int as c_uint);
        let model: c_uint = (r.eax >> 4 & 0xf as c_int as c_uint)
            .wrapping_add(r.eax >> 12 & 0xf0 as c_int as c_uint);
        let family: c_uint = (r.eax >> 8 & 0xf as c_int as c_uint)
            .wrapping_add(r.eax >> 20 & 0xff as c_int as c_uint);
        if memcmp(
            (cpu.c2rust_unnamed.vendor).as_mut_ptr() as *const c_void,
            b"AuthenticAMD\0" as *const u8 as *const c_char as *const c_void,
            ::core::mem::size_of::<[c_char; 12]>(),
        ) == 0
        {
            if flags.contains(CpuFlags::AVX2)
                && (family < 0x19 as c_int as c_uint
                    || family == 0x19 as c_int as c_uint
                        && (model < 0x10 as c_int as c_uint
                            || model >= 0x20 as c_int as c_uint && model < 0x60 as c_int as c_uint))
            {
                flags |= CpuFlags::SLOW_GATHER;
            }
        }
    }

    flags
}
