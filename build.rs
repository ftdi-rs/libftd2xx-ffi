#[deny(unsafe_code)]
use std::env;

#[cfg(feature = "bindgen")]
use bindgen;

cfg_if::cfg_if! {
    if #[cfg(all(target_os = "linux", target_arch = "x86_64"))] {
        mod paths {
            pub const HEADER: &str = "libftd2xx_src/linux/x64/ftd2xx.h";
            pub const SEARCH: &str = "libftd2xx_src/linux/x64/build";
        }
    } else if #[cfg(all(target_os = "linux", target_arch = "x86"))] {
        mod paths {
            pub const HEADER: &str = "libftd2xx_src/linux/x86/ftd2xx.h";
            pub const SEARCH: &str = "libftd2xx_src/linux/x86/build";
        }
    } else if #[cfg(all(target_os = "windows", target_arch = "x86_64"))] {
        mod paths {
            pub const HEADER: &str = "libftd2xx_src/windows/ftd2xx.h";
            pub const SEARCH: &str = "libftd2xx_src/windows/amd64";
        }
    } else if #[cfg(all(target_os = "windows", target_arch = "x86"))] {
        mod paths {
            pub const HEADER: &str = "libftd2xx_src/windows/ftd2xx.h";
            pub const SEARCH: &str = "libftd2xx_src/windows/i386";
        }
    } else {
        std::compile_error!(
            "Unsupported target, please open an issue. \
            target_os = {}, target_arch = {}",
            target_os, target_arch
        );
    }
}

fn main() {
    let cwd = env::current_dir().unwrap();
    let mut header = cwd.clone();
    header.push(paths::HEADER);
    let mut search = cwd.clone();
    search.push(paths::SEARCH);

    println!(
        "cargo:rustc-link-search=native={}",
        search.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=static=ftd2xx");
    println!("cargo:rerun-if-changed={}", header.to_str().unwrap());

    cfg_if::cfg_if! {
        if #[cfg(feature = "bindgen")] {
        let bindings = bindgen::Builder::default()
            .header(header.to_str().unwrap())
            .whitelist_function("FT_.*")
            .whitelist_type("FT_.*")
            .whitelist_var("FT_.*")
            .rustfmt_bindings(true)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Unable to generate bindings");

        let out_path = std::path::PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
        }
    }
}
