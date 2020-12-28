use std::env;

fn search_path<'a>() -> &'a str {
    match env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
        "windows" => match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
            "x86_64" => "vendor/windows/amd64",
            "x86" => "vendor/windows/i386",
            target_arch => panic!("Target architecture not supported: {}", target_arch),
        },
        "linux" => match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
            "x86_64" => "vendor/linux/x64/build",
            "x86" => "vendor/linux/x86/build",
            target_arch => panic!("Target architecture not supported: {}", target_arch),
        },
        target_os => panic!("Target OS not supported: {}", target_os),
    }
}

fn header_path<'a>() -> &'a str {
    match env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
        "windows" => "vendor/windows/ftd2xx.h",
        "linux" => match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
            "x86_64" => "vendor/linux/x64/ftd2xx.h",
            "x86" => "vendor/linux/x86/ftd2xx.h",
            target_arch => panic!("Target architecture not supported: {}", target_arch),
        },
        target_os => panic!("Target OS not supported: {}", target_os),
    }
}

fn main() {
    let cwd = env::current_dir().unwrap();
    let mut header = cwd.clone();
    header.push(header_path());
    let mut search = cwd;
    search.push(search_path());

    println!(
        "cargo:rustc-link-search=native={}",
        search.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=static=ftd2xx");
    println!("cargo:rerun-if-changed={}", header.to_str().unwrap());

    #[cfg(feature = "bindgen")]
    {
        let bindings = bindgen::Builder::default()
            .header(header.to_str().unwrap())
            .whitelist_function("FT_.*")
            .whitelist_type("FT_.*")
            .whitelist_var("FT_.*")
            .rustfmt_bindings(true)
            .derive_default(true)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Unable to generate bindings");

        let out_path = std::path::PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}
