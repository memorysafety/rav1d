#![deny(clippy::all)]

#[cfg(feature = "asm")]
mod asm {
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

        let arch = env::var("CARGO_CFG_TARGET_ARCH")
            .unwrap()
            .parse::<Arch>()
            .unwrap();
        let vendor = env::var("CARGO_CFG_TARGET_VENDOR").unwrap();
        let pointer_width = env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap();

        let vendor = vendor.as_str();
        let pointer_width = pointer_width.as_str();

        let rustc_cfg = match arch {
            Arch::X86(ArchX86::X86_32) => "nasm_x86",
            Arch::X86(ArchX86::X86_64) => "nasm_x86_64",
            Arch::Arm(..) => "asm_neon",
        };
        println!("cargo:rustc-cfg={rustc_cfg}");

        let mut defines = Vec::new();
        let mut define = |define: Define| {
            defines.push(define);
        };

        // TODO(kkysen) incorrect, dav1d defines these for all arches
        if matches!(arch, Arch::Arm(..)) {
            define(Define::bool("CONFIG_ASM", true));
            define(Define::bool("CONFIG_LOG", true));
        }

        // TODO(kkysen) incorrect since we may cross compile
        if (matches!(arch, Arch::X86(..)) && cfg!(target_os = "macos"))
            || (matches!(arch, Arch::Arm(..)) && vendor == "apple")
        {
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
        }

        if arch == Arch::X86(ArchX86::X86_32) {
            define(Define::new("STACK_ALIGNMENT", 4));
        }
        if arch == Arch::X86(ArchX86::X86_64) {
            define(Define::new("STACK_ALIGNMENT", 16));
        }

        if matches!(arch, Arch::X86(..)) {
            define(Define::bool("PIC", true));
            define(Define::bool("FORCE_VEX_ENCODING", false)); // TODO(kkysen) incorrect, not what dav1d does
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

        let x86_generic = &[
            "cdef_avx2.asm",
            "cdef_sse.asm",
            "itx_avx2.asm",
            "itx_avx512.asm",
            "itx_sse.asm",
            "looprestoration_avx2.asm",
            "msac.asm",
            "refmvs.asm",
        ][..];
        let x86_bpc8 = &[
            "cdef_avx512.asm",
            "filmgrain_avx2.asm",
            "filmgrain_avx512.asm",
            "filmgrain_sse.asm",
            "ipred_avx2.asm",
            "ipred_avx512.asm",
            "ipred_sse.asm",
            "loopfilter_avx2.asm",
            "loopfilter_avx512.asm",
            "loopfilter_sse.asm",
            "looprestoration_avx512.asm",
            "looprestoration_sse.asm",
            "mc_avx2.asm",
            "mc_avx512.asm",
            "mc_sse.asm",
        ][..];
        let x86_bpc16 = &[
            "cdef16_avx2.asm",
            "cdef16_avx512.asm",
            "cdef16_sse.asm",
            "filmgrain16_avx2.asm",
            "filmgrain16_avx512.asm",
            "filmgrain16_sse.asm",
            "ipred16_avx2.asm",
            "ipred16_avx512.asm",
            "ipred16_sse.asm",
            "itx16_avx2.asm",
            "itx16_avx512.asm",
            "itx16_sse.asm",
            "loopfilter16_avx2.asm",
            "loopfilter16_avx512.asm",
            "loopfilter16_sse.asm",
            "looprestoration16_avx2.asm",
            "looprestoration16_avx512.asm",
            "looprestoration16_sse.asm",
            "mc16_avx2.asm",
            "mc16_avx512.asm",
            "mc16_sse.asm",
        ][..];

        let arm_generic = &["itx.S", "msac.S", "refmvs.S", "looprestoration_common.S"][..];
        let arm_bpc8 = &[
            "cdef.S",
            "filmgrain.S",
            "ipred.S",
            "loopfilter.S",
            "looprestoration.S",
            "mc.S",
        ][..];
        let arm_bpc16 = &[
            "cdef16.S",
            "filmgrain16.S",
            "ipred16.S",
            "itx16.S",
            "loopfilter16.S",
            "looprestoration16.S",
            "mc16.S",
        ][..];

        // TODO(kkysen) Should not compile avx on x86.
        let asm_file_names = match arch {
            Arch::X86(..) => [
                x86_generic,
                #[cfg(feature = "bitdepth_8")]
                x86_bpc8,
                #[cfg(feature = "bitdepth_16")]
                x86_bpc16,
            ],
            Arch::Arm(..) => [
                arm_generic,
                #[cfg(feature = "bitdepth_8")]
                arm_bpc8,
                #[cfg(feature = "bitdepth_16")]
                arm_bpc16,
            ],
        };

        let asm_file_dir = match arch {
            Arch::X86(..) => ["x86", "."],
            Arch::Arm(..) => ["arm", pointer_width],
        };

        let asm_file_paths = asm_file_names.iter().flat_map(|a| *a).map(|file_name| {
            [&["src"], &asm_file_dir[..], &[file_name]]
                .into_iter()
                .flatten()
                .collect::<PathBuf>()
        });

        let rav1dasm = "rav1dasm";

        if use_nasm {
            let mut nasm = nasm_rs::Build::new();
            nasm.min_version(2, 14, 0);
            nasm.files(asm_file_paths);
            nasm.flag(&format!("-I{}/", out_dir.as_os_str().to_str().unwrap()));
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
            cc::Build::new()
                .files(asm_file_paths)
                .include(".")
                .include(&out_dir)
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
