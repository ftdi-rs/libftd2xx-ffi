use std::{env, path::PathBuf};

fn search_path() -> PathBuf {
    // PathBuf is used so that the path segment separators match the host system
    // This is important for cross-compiling for Windows from Linux
    let mut path: PathBuf = PathBuf::from("vendor");

    match env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
        "windows" => {
            path.push("windows");
            match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
                "x86_64" => {
                    #[cfg(feature = "static")]
                    {
                        path.push("Static");
                    }
                    path.push("amd64");
                }
                "x86" => {
                    #[cfg(feature = "static")]
                    {
                        path.push("Static");
                    }
                    path.push("i386");
                }
                "aarch64" => {
                    path.push("arm64");
                }
                target_arch => panic!("Target architecture not supported: {target_arch}"),
            }
        }
        "linux" => {
            path.push("linux");
            match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
                "x86_64" => path.push("x64"),
                "x86" => path.push("x86"),
                "arm" | "aarch64" => match env::var("TARGET").unwrap().as_str() {
                    "arm-unknown-linux-musleabihf" | "arm-unknown-linux-gnueabihf" => {
                        path.push("armv6-hf");
                    }
                    "armv7-unknown-linux-musleabihf" | "armv7-unknown-linux-gnueabihf" => {
                        path.push("armv7-hf");
                    }
                    "aarch64-unknown-linux-musl" | "aarch64-unknown-linux-gnu" => {
                        path.push("armv8-hf");
                    }
                    target => panic!("Target not supported: {target}"),
                },
                target_arch => panic!("Target architecture not supported: {target_arch}"),
            }
            path.push("build");
        }
        "macos" => match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
            "x86_64" | "aarch64" => {
                path.push("macos");
                path.push("build");
            }
            target_arch => panic!("Target architecture not supported: {target_arch}"),
        },
        target_os => panic!("Target OS not supported: {target_os}"),
    }

    path
}

fn header_path() -> PathBuf {
    // PathBuf is used so that the path segment separators match the host system
    // This is important for cross-compiling for Windows from Linux
    let mut path: PathBuf = PathBuf::from("vendor");

    match env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
        "windows" => path.push("windows"),
        "linux" => {
            path.push("linux");
            match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
                "x86_64" => path.push("x64"),
                "x86" => path.push("x86"),
                "arm" | "aarch64" => match env::var("TARGET").unwrap().as_str() {
                    "arm-unknown-linux-musleabihf" | "arm-unknown-linux-gnueabihf" => {
                        path.push("armv6-hf");
                    }
                    "armv7-unknown-linux-musleabihf" | "armv7-unknown-linux-gnueabihf" => {
                        path.push("armv7-hf");
                    }
                    "aarch64-unknown-linux-musl" | "aarch64-unknown-linux-gnu" => {
                        path.push("armv8-hf");
                    }
                    target => panic!("Target not supported: {target}"),
                },
                target_arch => panic!("Target architecture not supported: {target_arch}"),
            }
        }
        "macos" => match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
            "x86_64" | "aarch64" => path.push("macos"),
            target_arch => panic!("Target architecture not supported: {target_arch}"),
        },
        target_os => panic!("Target OS not supported: {target_os}"),
    }

    path.push("ftd2xx.h");

    path
}

// This adds sysroot to bindgen if cross-compiling.
// This is not great, please open an issue or pull-request if you know of a
// better way to handle this problem.
#[cfg(feature = "bindgen")]
fn clang_args() -> &'static [&'static str] {
    match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        #[cfg(all(target_os = "linux", not(target_arch = "arm")))]
        "arm" => &["--sysroot=/usr/arm-linux-gnueabihf"],
        #[cfg(all(target_os = "linux", not(target_arch = "aarch64")))]
        "aarch64" => &["--sysroot=/usr/aarch64-linux-gnu"],
        _ => &[],
    }
}

#[cfg(not(feature = "static"))]
fn linker_options() {
    println!("cargo:rustc-link-lib=dylib=ftd2xx");
    match env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
        "macos" => {
            println!("cargo:rustc-link-lib=framework=IOKit");
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
        }
        "linux" | "windows" => {}
        target_os => panic!("Target OS not supported: {target_os}"),
    }
}

#[cfg(feature = "static")]
fn linker_options() {
    println!("cargo:rustc-link-lib=static=ftd2xx");

    match env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
        "windows" | "linux" => {}
        "macos" => {
            println!("cargo:rustc-link-lib=framework=IOKit");
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
        }
        target_os => panic!("Target OS not supported: {target_os}"),
    }
}

fn main() {
    let cwd: PathBuf = env::current_dir().unwrap();
    let mut header: PathBuf = cwd.clone();
    header.push(header_path());
    let mut search: PathBuf = cwd;
    search.push(search_path());

    println!(
        "cargo:rustc-link-search=native={}",
        search.to_str().unwrap()
    );
    linker_options();

    println!("cargo:rerun-if-changed={}", header.to_str().unwrap());
    println!("cargo:rerun-if-env-changed=LIBMSVC_PATH");

    #[cfg(feature = "bindgen")]
    {
        let bindings = bindgen::Builder::default()
            .header(header.to_str().unwrap())
            .allowlist_function("FT_.*")
            .allowlist_type("FT_.*")
            .allowlist_var("FT_.*")
            .formatter(bindgen::Formatter::Rustfmt)
            .derive_default(true)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .clang_args(clang_args())
            .generate()
            .expect("Unable to generate bindings");

        let out_path = std::path::PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}
