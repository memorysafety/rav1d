#[cfg(feature = "asm")]
use core::panic;
#[cfg(feature = "asm")]
use std::env;
#[cfg(feature = "asm")]
use std::path::Path;

fn main() {
    #[cfg(feature = "asm")]
    {
        let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

        if arch == "x86_64" {
            println!("cargo:rustc-cfg={}", "nasm_x86_64");
            build_nasm_files(true)
        } else if arch == "x86" {
            println!("cargo:rustc-cfg={}", "nasm_x86");
            build_nasm_files(false)
        } else if arch == "aarch64" {
            println!("cargo:rustc-cfg={}", "asm_neon");
            build_asm_files(true)
        } else if arch == "arm" {
            println!("cargo:rustc-cfg={}", "asm_neon");
            build_asm_files(false)
        } else {
            panic!("unknown arch: {}", arch);
        }
    }
}

#[cfg(feature = "asm")]
fn build_nasm_files(x86_64: bool) {
    use std::fs::File;
    use std::io::Write;
    let out_dir = env::var("OUT_DIR").unwrap();

    let dest_path = Path::new(&out_dir).join("config.asm");
    let mut config_file = File::create(&dest_path).unwrap();
    config_file
        .write(b"%define private_prefix dav1d\n")
        .unwrap();
    if x86_64 {
        config_file.write(b"%define ARCH_X86_32 0\n").unwrap();
        config_file.write(b"%define ARCH_X86_64 1\n").unwrap();
        config_file.write(b"%define STACK_ALIGNMENT 16\n").unwrap();
    } else {
        config_file.write(b"%define ARCH_X86_32 1\n").unwrap();
        config_file.write(b"%define ARCH_X86_64 0\n").unwrap();
        config_file.write(b"%define STACK_ALIGNMENT 4\n").unwrap();
    }
    config_file.write(b"%define PIC 1\n").unwrap();
    config_file
        .write(b"%define FORCE_VEX_ENCODING 0\n")
        .unwrap();

    let mut asm_files = vec![
        "src/x86/cdef_avx2.asm",
        "src/x86/cdef_sse.asm",
        "src/x86/cpuid.asm",
        "src/x86/itx_avx2.asm",
        "src/x86/itx_avx512.asm",
        "src/x86/itx_sse.asm",
        "src/x86/looprestoration_avx2.asm",
        "src/x86/msac.asm",
        "src/x86/refmvs.asm",
    ];

    #[cfg(feature = "bitdepth_8")]
    asm_files.extend_from_slice(&[
        "src/x86/cdef_avx512.asm",
        "src/x86/filmgrain_avx2.asm",
        "src/x86/filmgrain_avx512.asm",
        "src/x86/filmgrain_sse.asm",
        "src/x86/ipred_avx2.asm",
        "src/x86/ipred_avx512.asm",
        "src/x86/ipred_sse.asm",
        "src/x86/loopfilter_avx2.asm",
        "src/x86/loopfilter_avx512.asm",
        "src/x86/loopfilter_sse.asm",
        "src/x86/looprestoration_avx512.asm",
        "src/x86/looprestoration_sse.asm",
        "src/x86/mc_avx2.asm",
        "src/x86/mc_avx512.asm",
        "src/x86/mc_sse.asm",
    ]);

    #[cfg(feature = "bitdepth_16")]
    asm_files.extend_from_slice(&[
        "src/x86/cdef16_avx2.asm",
        "src/x86/cdef16_avx512.asm",
        "src/x86/cdef16_sse.asm",
        "src/x86/filmgrain16_avx2.asm",
        "src/x86/filmgrain16_avx512.asm",
        "src/x86/filmgrain16_sse.asm",
        "src/x86/ipred16_avx2.asm",
        "src/x86/ipred16_avx512.asm",
        "src/x86/ipred16_sse.asm",
        "src/x86/itx16_avx2.asm",
        "src/x86/itx16_avx512.asm",
        "src/x86/itx16_sse.asm",
        "src/x86/loopfilter16_avx2.asm",
        "src/x86/loopfilter16_avx512.asm",
        "src/x86/loopfilter16_sse.asm",
        "src/x86/looprestoration16_avx2.asm",
        "src/x86/looprestoration16_avx512.asm",
        "src/x86/looprestoration16_sse.asm",
        "src/x86/mc16_avx2.asm",
        "src/x86/mc16_avx512.asm",
        "src/x86/mc16_sse.asm",
    ]);

    let mut config_include_arg = String::from("-I");
    config_include_arg.push_str(&out_dir);
    config_include_arg.push('/');

    let mut nasm = nasm_rs::Build::new();
    nasm.min_version(2, 14, 0);
    for file in asm_files {
        nasm.file(file);
    }
    nasm.flag(&config_include_arg);
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
    cc.compile("rav1dasm");

    println!("cargo:rustc-link-lib=static=rav1dasm");
}

#[cfg(feature = "asm")]
fn build_asm_files(aarch64: bool) {
    use std::fs::File;
    use std::io::Write;
    let out_dir = env::var("OUT_DIR").unwrap();

    let dest_path = Path::new(&out_dir).join("config.h");
    let mut config_file = File::create(&dest_path).unwrap();
    if env::var("CARGO_CFG_TARGET_VENDOR").unwrap() == "apple" {
        config_file.write(b" #define PREFIX 1\n").unwrap();
    }
    config_file
        .write(b" #define PRIVATE_PREFIX dav1d_\n")
        .unwrap();

    if aarch64 {
        config_file.write(b" #define ARCH_AARCH64 1\n").unwrap();
        config_file.write(b" #define ARCH_ARM 0\n").unwrap();
    } else {
        config_file.write(b" #define ARCH_AARCH64 0\n").unwrap();
        config_file.write(b" #define ARCH_ARM 1\n").unwrap();
    }
    config_file.write(b" #define CONFIG_LOG 1 \n").unwrap();
    config_file.write(b" #define HAVE_ASM 1\n").unwrap();
    config_file.sync_all().unwrap();

    let arch_dir = if aarch64 { "64" } else { "32" };
    let mut asm_files = vec![
        format!("src/arm/{}/itx.S", arch_dir),
        format!("src/arm/{}/msac.S", arch_dir),
        format!("src/arm/{}/refmvs.S", arch_dir),
        format!("src/arm/{}/looprestoration_common.S", arch_dir),
    ];

    #[cfg(feature = "bitdepth_8")]
    asm_files.extend_from_slice(&[
        format!("src/arm/{}/cdef.S", arch_dir),
        format!("src/arm/{}/filmgrain.S", arch_dir),
        format!("src/arm/{}/ipred.S", arch_dir),
        format!("src/arm/{}/loopfilter.S", arch_dir),
        format!("src/arm/{}/looprestoration.S", arch_dir),
        format!("src/arm/{}/mc.S", arch_dir),
    ]);

    #[cfg(feature = "bitdepth_16")]
    asm_files.extend_from_slice(&[
        format!("src/arm/{}/cdef16.S", arch_dir),
        format!("src/arm/{}/filmgrain16.S", arch_dir),
        format!("src/arm/{}/ipred16.S", arch_dir),
        format!("src/arm/{}/itx16.S", arch_dir),
        format!("src/arm/{}/loopfilter16.S", arch_dir),
        format!("src/arm/{}/looprestoration16.S", arch_dir),
        format!("src/arm/{}/mc16.S", arch_dir),
    ]);

    cc::Build::new()
        .files(asm_files)
        .include(".")
        .include(&out_dir)
        .compile("rav1dasm");

    println!("cargo:rustc-link-lib=static=rav1dasm");
}
