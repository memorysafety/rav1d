use std::env;
use std::path::Path;

fn main() {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    #[cfg(feature = "asm")]
    {
        if arch == "x86_64" {
            println!("cargo:rustc-cfg={}", "nasm_x86_64");
            build_nasm_files()
        }
        if arch == "aarch64" {
            println!("cargo:rustc-cfg={}", "asm_neon");
            build_asm_files()
        }
    }
}

#[cfg(feature = "asm")]
fn build_nasm_files() {
    use std::fs::File;
    use std::io::Write;
    let out_dir = env::var("OUT_DIR").unwrap();

    let dest_path = Path::new(&out_dir).join("config.asm");
    let mut config_file = File::create(&dest_path).unwrap();
    config_file
        .write(b"%define private_prefix dav1d\n")
        .unwrap();
    config_file.write(b"%define ARCH_X86_32 0\n").unwrap();
    config_file.write(b"%define ARCH_X86_64 1\n").unwrap();
    config_file.write(b"%define PIC 1\n").unwrap();
    config_file.write(b"%define STACK_ALIGNMENT 16\n").unwrap();
    config_file
        .write(b"%define FORCE_VEX_ENCODING 0\n")
        .unwrap();

    let mut asm_files = vec![
        "src/x86/cdef_avx2.asm",
        "src/x86/cdef_sse.asm",
        "src/x86/cpuid.asm",
        "src/x86/msac.asm",
        "src/x86/refmvs.asm",
    ];

    #[cfg(feature = "bitdepth_8")]
    asm_files.extend_from_slice(&[
        "src/x86/cdef_avx512.asm",
        "src/x86/filmgrain_avx2.asm",
        "src/x86/filmgrain_avx512.asm",
        "src/x86/filmgrain_sse.asm",
    ]);

    #[cfg(feature = "bitdepth_16")]
    asm_files.extend_from_slice(&[
        "src/x86/cdef16_avx2.asm",
        "src/x86/cdef16_avx512.asm",
        "src/x86/cdef16_sse.asm",
        "src/x86/filmgrain16_avx2.asm",
        "src/x86/filmgrain16_avx512.asm",
        "src/x86/filmgrain16_sse.asm",
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
fn build_asm_files() {
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
    config_file.write(b" #define ARCH_AARCH64 1\n").unwrap();
    config_file.write(b" #define ARCH_ARM 0\n").unwrap();
    config_file.write(b" #define CONFIG_LOG 1 \n").unwrap();
    config_file.write(b" #define HAVE_ASM 1\n").unwrap();
    config_file.sync_all().unwrap();

    let asm_files = &[
        "src/arm/64/msac.S",
        "src/arm/64/refmvs.S",
        #[cfg(feature = "bitdepth_8")]
        "src/arm/64/cdef.S",
        #[cfg(feature = "bitdepth_16")]
        "src/arm/64/cdef16.S",
    ];

    cc::Build::new()
        .files(asm_files)
        .include(".")
        .include(&out_dir)
        .compile("rav1d-aarch64");

    println!("cargo:rustc-link-lib=static=rav1d-aarch64");
}
