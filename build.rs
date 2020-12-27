use std::env;

#[cfg(target_os = "linux")]
fn search_path<'a>() -> &'a str {
    match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "x86_64" => "vendor/linux/x64/build",
        "x86" => "vendor/linux/x86/build",
        _ => unreachable!(),
    }
}

#[cfg(target_os = "linux")]
fn header_path<'a>() -> &'a str {
    match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "x86_64" => "vendor/linux/x64/ftd2xx.h",
        "x86" => "vendor/linux/x86/ftd2xx.h",
        _ => unreachable!(),
    }
}

#[cfg(target_os = "windows")]
fn search_path<'a>() -> &'a str {
    match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "x86_64" => "vendor/windows/amd64",
        "x86" => "vendor/windows/i386",
        _ => unreachable!(),
    }
}

#[cfg(target_os = "windows")]
fn header_path<'a>() -> &'a str {
    "vendor/windows/ftd2xx.h"
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
