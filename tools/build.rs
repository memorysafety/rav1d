use std::env;

fn main() {
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

    let os = os.as_str();
    let env = env.as_str();

    match (os, env) {
        ("windows", "msvc") => {
            if !cfg!(all(target_os = "windows", target_env = "msvc")) {
                panic!("Cannot cross compile to *-windows-msvc");
            }
        }
        #[allow(clippy::needless_return)]
        _ => return,
    }

    // NOTE: we rely on libraries that are only distributed for Windows so
    // targeting Windows/MSVC is not supported when cross compiling.
    #[cfg(all(target_os = "windows", target_env = "msvc"))]
    {
        use cc::windows_registry;

        // for sprintf, snprintf, etc.
        let target = env::var("TARGET").unwrap();
        println!("cargo:rustc-link-lib=static=oldnames");
        let tool = windows_registry::find_tool(&target, "cl.exe")
            .expect("couldn't find cl.exe; are the Visual Studio C++ tools installed?");
        let lib_paths = &tool
            .env()
            .iter()
            .find(|(key, _val)| key == "LIB")
            .expect("LIB path not found")
            .1;
        for path in lib_paths.to_str().unwrap().split(';') {
            if path != "" {
                println!("cargo:rustc-link-search={path}");
            }
        }

        let getopt = "getopt";
        cc::Build::new()
            .files([&"../tools/compat/getopt.c"])
            .include("../src/include/compat")
            .debug(cfg!(debug_assertions))
            .compile(&getopt);
        // cc automatically outputs the following line
        // println!("cargo:rustc-link-lib=static={getopt}");
    }
}
