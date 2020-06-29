#[deny(unsafe_code)]
use std::env;
use std::path::PathBuf;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
mod paths {
    pub const HEADER: &str = "libftd2xx_src/linux/x64/ftd2xx.h";
    pub const SEARCH: &str = "libftd2xx_src/linux/x64/build";
}

#[cfg(all(target_os = "linux", target_arch = "x86"))]
mod paths {
    pub const HEADER: &str = "libftd2xx_src/linux/x86/ftd2xx.h";
    pub const SEARCH: &str = "libftd2xx_src/linux/x86/build";
}

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
mod paths {
    pub const HEADER: &str = "libftd2xx_src/windows/ftd2xx.h";
    pub const SEARCH: &str = "libftd2xx_src/windows/amd64";
}

#[cfg(all(target_os = "windows", target_arch = "x86"))]
mod paths {
    pub const HEADER: &str = "libftd2xx_src/windows/ftd2xx.h";
    pub const SEARCH: &str = "libftd2xx_src/windows/i386";
}

fn main() {
    let cwd = env::current_dir().unwrap();
    let mut header = cwd.clone();
    header.push(paths::HEADER);
    let mut search = cwd.clone();
    search.push(paths::SEARCH);

    println!("cargo:rustc-link-search={}", search.to_str().unwrap());
    println!("cargo:rustc-link-lib=ftd2xx");
    println!("cargo:rerun-if-changed={}", header.to_str().unwrap());

    let bindings = bindgen::Builder::default()
        .header(header.to_str().unwrap())
        .whitelist_function("FT_.*")
        .whitelist_type("FT_.*")
        .whitelist_var("FT_.*")
        .rustfmt_bindings(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
