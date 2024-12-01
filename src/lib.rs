//! Rust FFI bindings to the [FTDI D2XX drivers](https://www.ftdichip.com/Drivers/D2XX.htm).
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(clippy::all, clippy::pedantic, clippy::nursery)]

cfg_if::cfg_if! {
    if #[cfg(feature = "bindgen")] {
        include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    } else if #[cfg(all(target_os = "linux", target_arch = "x86_64"))] {
        include!("bindings_linux_x64.rs");
    } else if #[cfg(all(target_os = "linux", target_arch = "x86"))] {
        include!("bindings_linux_x86.rs");
    } else if #[cfg(all(target_os = "windows", target_arch = "x86_64"))] {
        include!("bindings_windows_x64.rs");
    } else if #[cfg(all(target_os = "windows", target_arch = "x86"))] {
        include!("bindings_windows_x86.rs");
    } else if #[cfg(all(target_os = "windows", target_arch = "aarch64"))] {
        include!("bindings_windows_arm64.rs");
    } else if #[cfg(all(target_os = "linux", target_arch = "arm"))] {
        include!("bindings_linux_armv6.rs");
    } else if #[cfg(all(target_os = "linux", target_arch = "aarch64"))] {
        include!("bindings_linux_armv8.rs");
    } else if #[cfg(all(target_os = "macos", target_arch = "x86_64"))] {
        include!("bindings_macos_x64.rs");
    } else if #[cfg(all(target_os = "macos", target_arch = "aarch64"))] {
        include!("bindings_macos_aarch64.rs");
    } else {
        std::compile_error!("pre-generated bindings are not avaliable for your target");
    }
}
