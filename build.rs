extern crate bindgen;

use flate2::read::GzDecoder;
use reqwest::blocking::get;
use std::env;
use std::fs::File;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use tar::Archive;
use url::Url;
use zip;

const LIB_DIR: &str = "libftd2xx";

fn main() {
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    println!("CARGO_CFG_TARGET_OS={}", os);
    println!("CARGO_CFG_TARGET_ARCH={}", arch);
    let url_str = if cfg!(target_os = "windows") {
        "https://www.ftdichip.com/Drivers/CDM/CDM v2.12.28 WHQL Certified.zip"
    } else if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        "https://www.ftdichip.com/Drivers/D2XX/Linux/libftd2xx-x86_64-1.4.8.gz"
    } else if cfg!(all(target_os = "linux", target_arch = "x86")) {
        "https://www.ftdichip.com/Drivers/D2XX/Linux/libftd2xx-i386-1.4.8.gz"
    } else {
        panic!(
            "unsupported configuration: target_os={} target_arch={}",
            os, arch
        )
    };
    let mut download_dir = PathBuf::from(LIB_DIR);
    download_dir.push(os);
    if cfg!(target_os = "linux") {
        download_dir.push(arch);
    }
    std::fs::create_dir_all(download_dir.as_path()).unwrap();

    let url = Url::parse(url_str).unwrap();
    let filename = url
        .path_segments()
        .and_then(|segments| segments.last())
        .unwrap();
    let extension = Path::new(filename).extension().unwrap();
    let mut download_path = PathBuf::from(download_dir.to_str().unwrap());
    download_path.push(filename);

    if !download_path.exists() {
        let mut response = get(url_str).unwrap();
        let mut out = File::create(download_path.to_str().unwrap()).unwrap();
        io::copy(&mut response, &mut out).unwrap();
    }

    let archive_file = File::open(download_path.to_str().unwrap()).unwrap();
    match extension.to_str().unwrap() {
        "gz" => {
            let tar = GzDecoder::new(archive_file);
            let mut archive = Archive::new(tar);
            archive.unpack(download_dir.as_path()).unwrap();
        }
        "zip" => {
            let mut archive = zip::ZipArchive::new(archive_file).unwrap();
            for i in 0..archive.len() {
                let mut file = archive.by_index(i).unwrap();
                let mut outpath = download_dir.clone();
                outpath.push(file.sanitized_name());
                if (&*file.name()).ends_with('/') {
                    std::fs::create_dir_all(&outpath).unwrap();
                } else {
                    if let Some(p) = outpath.parent() {
                        if !p.exists() {
                            std::fs::create_dir_all(&p).unwrap();
                        }
                    }
                    let mut outfile = std::fs::File::create(&outpath).unwrap();
                    io::copy(&mut file, &mut outfile).unwrap();
                }
                // Get and Set permissions
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Some(mode) = file.unix_mode() {
                        std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode))
                            .unwrap();
                    }
                }
            }
        }
        _ => panic!("unknown extension"),
    }

    let mut search_path = download_dir.clone();
    let mut header_path = download_dir.clone();
    if cfg!(target_os = "linux") {
        header_path.push("release");
    }
    header_path.push("ftd2xx.h");
    let header = header_path.to_str().unwrap();

    if cfg!(target_os = "linux") {
        search_path.push("release");
        search_path.push("build");
    } else if cfg!(target_os = "windows") {
        search_path.push("Static");
        if cfg!(target_arch = "x86_64") {
            search_path.push("amd64")
        } else if cfg!(target_arch = "x86") {
            search_path.push("i386")
        }
    }

    println!("cargo:rustc-link-search={}", search_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=ftd2xx");
    println!("cargo:rerun-if-changed={}", header);

    let bindings = bindgen::Builder::default()
        .header(header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
