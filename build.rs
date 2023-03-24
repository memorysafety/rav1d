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

    let asm_files = &["src/x86/msac.asm", "src/x86/cpuid.asm"];

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
      println!("cargo:warning={}", e);
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
    todo!();
}
