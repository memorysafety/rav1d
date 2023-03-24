use ::libc;
extern "C" {
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    fn dav1d_cpu_cpuid(regs: *mut CpuidRegisters, leaf: libc::c_uint, subleaf: libc::c_uint);
    fn dav1d_cpu_xgetbv(xcr: libc::c_uint) -> uint64_t;
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type CpuFlags = libc::c_uint;
pub const DAV1D_X86_CPU_FLAG_SLOW_GATHER: CpuFlags = 32;
pub const DAV1D_X86_CPU_FLAG_AVX512ICL: CpuFlags = 16;
pub const DAV1D_X86_CPU_FLAG_AVX2: CpuFlags = 8;
pub const DAV1D_X86_CPU_FLAG_SSE41: CpuFlags = 4;
pub const DAV1D_X86_CPU_FLAG_SSSE3: CpuFlags = 2;
pub const DAV1D_X86_CPU_FLAG_SSE2: CpuFlags = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CpuidRegisters {
    pub eax: uint32_t,
    pub ebx: uint32_t,
    pub edx: uint32_t,
    pub ecx: uint32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed {
    pub max_leaf: uint32_t,
    pub vendor: [libc::c_char; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_0 {
    pub r: CpuidRegisters,
    pub c2rust_unnamed: C2RustUnnamed,
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_get_cpu_flags_x86() -> libc::c_uint {
    let mut cpu: C2RustUnnamed_0 = C2RustUnnamed_0 {
        r: CpuidRegisters {
            eax: 0,
            ebx: 0,
            edx: 0,
            ecx: 0,
        },
    };
    dav1d_cpu_cpuid(&mut cpu.r, 0u32, 0u32);
    let mut flags: libc::c_uint = 0u32;
    if cpu.c2rust_unnamed.max_leaf >= 1u32 {
        let mut r: CpuidRegisters = CpuidRegisters {
            eax: 0,
            ebx: 0,
            edx: 0,
            ecx: 0,
        };
        dav1d_cpu_cpuid(&mut r, 1u32, 0u32);
        let model: libc::c_uint = (r.eax >> 4i32 & 0xfu32).wrapping_add(r.eax >> 12i32 & 0xf0u32);
        let family: libc::c_uint = (r.eax >> 8i32 & 0xfu32).wrapping_add(r.eax >> 20i32 & 0xffu32);
        if r.edx & 0x6008000u32 == 0x6008000u32 {
            flags |= DAV1D_X86_CPU_FLAG_SSE2;
            if r.ecx & 0x201u32 == 0x201u32 {
                flags |= DAV1D_X86_CPU_FLAG_SSSE3;
                if r.ecx & 0x80000u32 == 0x80000u32 {
                    flags |= DAV1D_X86_CPU_FLAG_SSE41;
                }
            }
        }

        // We only support >128-bit SIMD on x86-64.
        //#[cfg(target_arch = "x86_64")]
        if r.ecx & 0x18000000u32 == 0x18000000u32 {
            let xcr0: uint64_t = dav1d_cpu_xgetbv(0u32);
            if xcr0 & 0x6u64 == 0x6u64 {
                if cpu.c2rust_unnamed.max_leaf >= 7u32 {
                    dav1d_cpu_cpuid(&mut r, 7u32, 0u32);
                    if r.ebx & 0x128u32 == 0x128u32 {
                        flags |= DAV1D_X86_CPU_FLAG_AVX2;
                        if xcr0 & 0xe0u64 == 0xe0u64 {
                            if r.ebx & 0xd0230000u32 == 0xd0230000u32
                                && r.ecx & 0x5f42u32 == 0x5f42u32
                            {
                                flags |= DAV1D_X86_CPU_FLAG_AVX512ICL;
                            }
                        }
                    }
                }
            }
        }

        if memcmp(
            (cpu.c2rust_unnamed.vendor).as_mut_ptr() as *const libc::c_void,
            b"AuthenticAMD\0" as *const u8 as *const libc::c_void,
            ::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
        ) == 0
        {
            if flags & DAV1D_X86_CPU_FLAG_AVX2 != 0
                && (family < 0x19u32
                    || family == 0x19u32
                        && (model < 0x10u32 || model >= 0x20u32 && model < 0x60u32))
            {
                flags |= DAV1D_X86_CPU_FLAG_SLOW_GATHER;
            }
        }
    }
    return flags;
}
