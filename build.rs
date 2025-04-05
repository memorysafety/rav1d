#![deny(clippy::all)]

#[cfg(feature = "asm")]
mod asm {
    use std::collections::HashSet;
    use std::env;
    use std::fmt::Display;
    use std::fs;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Arch {
        X86(ArchX86),
        Arm(ArchArm),
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum ArchX86 {
        X86_32,
        X86_64,
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum ArchArm {
        Arm32,
        Arm64,
    }

    impl FromStr for Arch {
        type Err = String;

        fn from_str(arch: &str) -> Result<Self, Self::Err> {
            Ok(match arch {
                "x86" => Self::X86(ArchX86::X86_32),
                "x86_64" => Self::X86(ArchX86::X86_64),
                "arm" => Self::Arm(ArchArm::Arm32),
                "aarch64" => Self::Arm(ArchArm::Arm64),
                _ => return Err(format!("unexpected arch: {arch}")),
            })
        }
    }

    struct Define {
        name: &'static str,
        value: String,
    }

    impl Define {
        pub fn new(name: &'static str, value: impl Display) -> Self {
            Self {
                name,
                value: value.to_string(),
            }
        }

        pub fn bool(name: &'static str, value: bool) -> Self {
            Self::new(name, value as u8)
        }
    }

    pub fn main() {
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

        let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
        let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
        let vendor = env::var("CARGO_CFG_TARGET_VENDOR").unwrap();
        let pointer_width = env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap();
        let features = env::var("CARGO_CFG_TARGET_FEATURE").unwrap();

        // Nothing to do on unknown architectures
        let Ok(arch) = arch.parse::<Arch>() else {
            return;
        };
        let os = os.as_str();
        let vendor = vendor.as_str();
        let pointer_width = pointer_width.as_str();
        let features = features.split(',').collect::<HashSet<_>>();

        let mut defines = Vec::new();
        let mut define = |define: Define| {
            defines.push(define);
        };

        define(Define::bool("CONFIG_ASM", true));
        define(Define::bool("CONFIG_LOG", true)); // TODO(kkysen) should be configurable

        if vendor == "apple" || (os == "windows" && matches!(arch, Arch::X86(ArchX86::X86_32))) {
            define(Define::bool("PREFIX", true));
        }

        if matches!(arch, Arch::X86(..)) {
            define(Define::new("private_prefix", "dav1d"));
        }
        if matches!(arch, Arch::Arm(..)) {
            define(Define::new("PRIVATE_PREFIX", "dav1d_"));
        }

        if let Arch::X86(arch) = arch {
            define(Define::bool("ARCH_X86_32", arch == ArchX86::X86_32));
            define(Define::bool("ARCH_X86_64", arch == ArchX86::X86_64));
        }
        if let Arch::Arm(arch) = arch {
            define(Define::bool("ARCH_ARM", arch == ArchArm::Arm32));
            define(Define::bool("ARCH_AARCH64", arch == ArchArm::Arm64));

            if arch == ArchArm::Arm64 {
                define(Define::bool(
                    "HAVE_DOTPROD",
                    cfg!(feature = "asm_arm64_dotprod"),
                ));
                define(Define::bool("HAVE_I8MM", cfg!(feature = "asm_arm64_i8mm")));
                define(Define::bool("HAVE_SVE2", cfg!(feature = "asm_arm64_sve2")));
            }
        }

        if let Arch::X86(arch) = arch {
            let stack_alignment = if arch == ArchX86::X86_64 || os == "linux" || vendor == "apple" {
                16
            } else {
                4
            };
            define(Define::new("STACK_ALIGNMENT", stack_alignment));
        }

        if matches!(arch, Arch::X86(..)) {
            define(Define::bool("PIC", true));

            // Convert SSE asm into (128-bit) AVX when compiler flags are set to use AVX instructions.
            // Note that this checks compile-time CPU features, not runtime features,
            // but that does seem to be what `dav1d` does, too.
            define(Define::bool("FORCE_VEX_ENCODING", features.contains("avx")));
        }

        let use_nasm = match arch {
            Arch::X86(..) => true,
            Arch::Arm(..) => false,
        };

        let define_prefix = if use_nasm { "%" } else { " #" };

        let config_lines = defines
            .iter()
            .map(|Define { name, value }| format!("{define_prefix}define {name} {value}"))
            .collect::<Vec<_>>();

        let config_contents = config_lines.join("\n");
        let config_file_name = if use_nasm { "config.asm" } else { "config.h" };
        let config_path = out_dir.join(config_file_name);
        fs::write(&config_path, &config_contents).unwrap();

        // Note that avx* is never (at runtime) supported on x86.
        let x86_generic = &["cdef_sse", "itx_sse", "msac", "pal", "refmvs"][..];
        let x86_64_generic = &[
            "cdef_avx2",
            "itx_avx2",
            "itx_avx512",
            "looprestoration_avx2",
        ][..];
        let x86_bpc8 = &[
            "filmgrain_sse",
            "ipred_sse",
            "loopfilter_sse",
            "looprestoration_sse",
            "mc_sse",
        ][..];
        let x86_64_bpc8 = &[
            "cdef_avx512",
            "filmgrain_avx2",
            "filmgrain_avx512",
            "ipred_avx2",
            "ipred_avx512",
            "loopfilter_avx2",
            "loopfilter_avx512",
            "looprestoration_avx512",
            "mc_avx2",
            "mc_avx512",
        ][..];
        let x86_bpc16 = &[
            "cdef16_sse",
            "filmgrain16_sse",
            "ipred16_sse",
            "itx16_sse",
            "loopfilter16_sse",
            "looprestoration16_sse",
            "mc16_sse",
            // TODO(kkysen) avx2 shouldn't be in x86,
            // but a const used in sse is defined in avx2 (a bug).
            "ipred16_avx2",
        ][..];
        let x86_64_bpc16 = &[
            "cdef16_avx2",
            "cdef16_avx512",
            "filmgrain16_avx2",
            "filmgrain16_avx512",
            // TODO(kkysen) avx2 should only be in x86_64,
            // but a const used in sse is defined in avx2 (a bug).
            // "ipred16_avx2",
            "ipred16_avx512",
            "itx16_avx2",
            "itx16_avx512",
            "loopfilter16_avx2",
            "loopfilter16_avx512",
            "looprestoration16_avx2",
            "looprestoration16_avx512",
            "mc16_avx2",
            "mc16_avx512",
        ][..];

        let arm_generic = &["itx", "msac", "refmvs", "looprestoration_common"][..];
        let arm_dotprod = &["mc_dotprod"][..];
        let arm_sve_bpc16 = &["mc16_sve"][..];
        let arm_bpc8 = &[
            "cdef",
            "filmgrain",
            "ipred",
            "loopfilter",
            "looprestoration",
            "mc",
        ][..];
        let arm_bpc16 = &[
            "cdef16",
            "filmgrain16",
            "ipred16",
            "itx16",
            "loopfilter16",
            "looprestoration16",
            "mc16",
        ][..];

        let x86_all = &[
            x86_generic,
            #[cfg(feature = "bitdepth_8")]
            x86_bpc8,
            #[cfg(feature = "bitdepth_16")]
            x86_bpc16,
        ][..];
        let x86_64_all = &[
            x86_generic,
            x86_64_generic,
            #[cfg(feature = "bitdepth_8")]
            x86_bpc8,
            #[cfg(feature = "bitdepth_8")]
            x86_64_bpc8,
            #[cfg(feature = "bitdepth_16")]
            x86_bpc16,
            #[cfg(feature = "bitdepth_16")]
            x86_64_bpc16,
        ][..];
        let arm_all = &[
            arm_generic,
            #[cfg(feature = "bitdepth_8")]
            arm_bpc8,
            #[cfg(feature = "bitdepth_16")]
            arm_bpc16,
        ][..];
        let arm64_all = &[
            arm_generic,
            arm_dotprod,
            #[cfg(feature = "bitdepth_16")]
            arm_sve_bpc16,
            #[cfg(feature = "bitdepth_8")]
            arm_bpc8,
            #[cfg(feature = "bitdepth_16")]
            arm_bpc16,
        ][..];

        let asm_file_names = match arch {
            Arch::X86(ArchX86::X86_32) => x86_all,
            Arch::X86(ArchX86::X86_64) => x86_64_all,
            Arch::Arm(ArchArm::Arm32) => arm_all,
            Arch::Arm(ArchArm::Arm64) => arm64_all,
        };

        let asm_file_dir = match arch {
            Arch::X86(..) => ["x86", "."],
            Arch::Arm(..) => ["arm", pointer_width],
        };
        let asm_extension = if use_nasm { "asm" } else { "S" };

        let asm_file_paths = asm_file_names.iter().flat_map(|a| *a).map(|file_name| {
            let mut path = [&["src"], &asm_file_dir[..], &[file_name]]
                .into_iter()
                .flatten()
                .collect::<PathBuf>();
            path.set_extension(asm_extension);
            println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
            path
        });

        let rav1dasm = "rav1dasm";

        if use_nasm {
            let mut nasm = nasm_rs::Build::new();
            nasm.min_version(2, 14, 0);
            nasm.files(asm_file_paths);
            #[cfg(debug_assertions)]
            nasm.flag("-g");
            #[cfg(debug_assertions)]
            nasm.flag(match os {
                "windows" => "-fwin64",
                _ => "-Fdwarf",
            });
            nasm.flag(&format!("-I{}/", out_dir.to_str().unwrap()));
            nasm.flag("-Isrc/");
            let obj = nasm.compile_objects().unwrap_or_else(|e| {
                println!("cargo:warning={e}");
                panic!("NASM build failed. Make sure you have nasm installed or disable the \"asm\" feature.\n\
                    You can get NASM from https://nasm.us or your system's package manager.\n\nerror: {e}");
            });

            // cc is better at finding the correct archiver
            let mut cc = cc::Build::new();
            for o in obj {
                cc.object(o);
            }
            cc.compile(rav1dasm);
        } else {
            let mut cc = cc::Build::new();
            if arch == Arch::Arm(ArchArm::Arm64) {
                if cfg!(feature = "asm_arm64_sve2") {
                    cc.flag("-march=armv9.1-a")
                } else {
                    cc.flag("-march=armv8.6-a")
                };
            }
            cc.files(asm_file_paths)
                .include(".")
                .include(&out_dir)
                .debug(cfg!(debug_assertions))
                .compile(rav1dasm);
        }

        println!("cargo:rustc-link-lib=static={rav1dasm}");
    }
}

fn main() {
    #[cfg(feature = "asm")]
    {
        asm::main();
    }
}
