use ::libc;
extern "C" {
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn dav1d_cpu_cpuid(
        regs: *mut CpuidRegisters,
        leaf: libc::c_uint,
        subleaf: libc::c_uint,
    );
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CpuidRegisters {
    pub eax: uint32_t,
    pub ebx: uint32_t,
    pub edx: uint32_t,
    pub ecx: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub max_leaf: uint32_t,
    pub vendor: [libc::c_char; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
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
    dav1d_cpu_cpuid(
        &mut cpu.r,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
    );
    let mut flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if cpu.c2rust_unnamed.max_leaf >= 1 as libc::c_int as libc::c_uint {
        let mut r: CpuidRegisters = CpuidRegisters {
            eax: 0,
            ebx: 0,
            edx: 0,
            ecx: 0,
        };
        dav1d_cpu_cpuid(
            &mut r,
            1 as libc::c_int as libc::c_uint,
            0 as libc::c_int as libc::c_uint,
        );
        let model: libc::c_uint = (r.eax >> 4 as libc::c_int
            & 0xf as libc::c_int as libc::c_uint)
            .wrapping_add(
                r.eax >> 12 as libc::c_int & 0xf0 as libc::c_int as libc::c_uint,
            );
        let family: libc::c_uint = (r.eax >> 8 as libc::c_int
            & 0xf as libc::c_int as libc::c_uint)
            .wrapping_add(
                r.eax >> 20 as libc::c_int & 0xff as libc::c_int as libc::c_uint,
            );
        if r.edx & 0x6008000 as libc::c_int as libc::c_uint
            == 0x6008000 as libc::c_int as libc::c_uint
        {
            flags |= DAV1D_X86_CPU_FLAG_SSE2 as libc::c_int as libc::c_uint;
            if r.ecx & 0x201 as libc::c_int as libc::c_uint
                == 0x201 as libc::c_int as libc::c_uint
            {
                flags |= DAV1D_X86_CPU_FLAG_SSSE3 as libc::c_int as libc::c_uint;
                if r.ecx & 0x80000 as libc::c_int as libc::c_uint
                    == 0x80000 as libc::c_int as libc::c_uint
                {
                    flags |= DAV1D_X86_CPU_FLAG_SSE41 as libc::c_int as libc::c_uint;
                }
            }
        }
        if r.ecx & 0x18000000 as libc::c_int as libc::c_uint
            == 0x18000000 as libc::c_int as libc::c_uint
        {
            let xcr0: uint64_t = dav1d_cpu_xgetbv(0 as libc::c_int as libc::c_uint);
            if xcr0 & 0x6 as libc::c_int as libc::c_ulong
                == 0x6 as libc::c_int as libc::c_ulong
            {
                if cpu.c2rust_unnamed.max_leaf >= 7 as libc::c_int as libc::c_uint {
                    dav1d_cpu_cpuid(
                        &mut r,
                        7 as libc::c_int as libc::c_uint,
                        0 as libc::c_int as libc::c_uint,
                    );
                    if r.ebx & 0x128 as libc::c_int as libc::c_uint
                        == 0x128 as libc::c_int as libc::c_uint
                    {
                        flags |= DAV1D_X86_CPU_FLAG_AVX2 as libc::c_int as libc::c_uint;
                        if xcr0 & 0xe0 as libc::c_int as libc::c_ulong
                            == 0xe0 as libc::c_int as libc::c_ulong
                        {
                            if r.ebx & 0xd0230000 as libc::c_uint
                                == 0xd0230000 as libc::c_uint
                                && r.ecx & 0x5f42 as libc::c_int as libc::c_uint
                                    == 0x5f42 as libc::c_int as libc::c_uint
                            {
                                flags
                                    |= DAV1D_X86_CPU_FLAG_AVX512ICL as libc::c_int
                                        as libc::c_uint;
                            }
                        }
                    }
                }
            }
        }
        if memcmp(
            (cpu.c2rust_unnamed.vendor).as_mut_ptr() as *const libc::c_void,
            b"AuthenticAMD\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            ::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
        ) == 0
        {
            if flags & DAV1D_X86_CPU_FLAG_AVX2 as libc::c_int as libc::c_uint != 0
                && (family < 0x19 as libc::c_int as libc::c_uint
                    || family == 0x19 as libc::c_int as libc::c_uint
                        && (model < 0x10 as libc::c_int as libc::c_uint
                            || model >= 0x20 as libc::c_int as libc::c_uint
                                && model < 0x60 as libc::c_int as libc::c_uint))
            {
                flags |= DAV1D_X86_CPU_FLAG_SLOW_GATHER as libc::c_int as libc::c_uint;
            }
        }
    }
    return flags;
}
