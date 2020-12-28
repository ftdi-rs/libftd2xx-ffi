//! Rust FFI bindings to the [FTDI D2XX drivers].
//!
//! This crate is **just** the C bindings.
//! There is a separate crate, [libftd2xx], which provides safe wrappers around
//! the unsafe C bindings.
//!
//! # Usage
//! Simply add this crate as a dependency in your `Cargo.toml`.
//! The static library is distributed in this crate with permission from FTDI.
//!
//! ```toml
//! [dependencies]
//! libftd2xx-ffi = "~0.5.0"
//! ```
//!
//! The default feature set will use pre-generated bindings.
//! This is only available for Windows x86_64 and Linux x86_64 platforms.
//!
//! The bindings can also be generated during compilation using the [bindgen]
//! feature flag.
//! ```toml
//! [dependencies]
//! libftd2xx-ffi = { version = "~0.5.0", features = ["bindgen"] }
//! ```
//!
//! Bindgen has additional dependencies that must be installed in order to
//! compile successfully, see the [bindgen requirements] page for more details.
//!
//! # Supported Targets
//!
//! ## Tested Targets
//!
//! * `x86_64-pc-windows-msvc` (dynamic linking only)
//! * `x86_64-unknown-linux-gnu` (static linking only)
//! * `x86_64-unknown-linux-musl` (static linking only)
//!
//! ## Untested Targets
//!
//! These targets are provided, but they are untested.
//! Use at your own risk.
//!
//! * `aarch64-unknown-linux-gnu` (static linking only)
//! * `aarch64-unknown-linux-musl` (static linking only)
//! * `arm-unknown-linux-gnueabihf` (static linking only)
//! * `arm-unknown-linux-musleabihf` (static linking only)
//! * `armv7-unknown-linux-gnueabihf` (static linking only)
//! * `armv7-unknown-linux-musleabihf` (static linking only)
//! * `i686-pc-windows-msvc` (dynamic linking only)
//! * `i686-unknown-linux-gnu` (static linking only)
//! * `i686-unknown-linux-musl` (static linking only)
//!
//! # References
//!
//! * [D2XX Programmers Guide V1.4]
//! * [D2XX Drivers Download Page]
//!
//! # Troubleshooting
//! ## Unknown Device on Linux
//! Remove the VCP FTDI driver.
//! ```bash
//! sudo rmmod ftdi_sio
//! sudo rmmod usbserial
//! ```
//! See [FTDI Drivers Installation Guide for Linux] for more details.
//!
//! # License
//! FTDI provides the D2XX driver as a compiled library and a header file.
//! These files can be found within the `vendor` directory.
//!
//! The code within the `vendor` directory is licensed by FTDI.
//! Please see the [Driver License Terms] page for their license.
//!
//! All code outside of the `vendor` directory is MIT licensed.
//!
//! **Note:** This crate is not affiliated with FTDI.
//! You will need to contact the vendor for any support requests with the
//! underlying library because it is closed source.
//!
//! [bindgen requirements]: https://rust-lang.github.io/rust-bindgen/requirements.html
//! [bindgen]: https://github.com/rust-lang/rust-bindgen
//! [D2XX Drivers Download Page]: https://www.ftdichip.com/Drivers/D2XX.htm
//! [D2xx Programmers Guide V1.4]: https://www.ftdichip.com/Support/Documents/ProgramGuides/D2XX_Programmer's_Guide(FT_000071).pdf
//! [Driver License Terms]: https://www.ftdichip.com/Drivers/FTDriverLicenceTerms.htm
//! [FTDI D2XX drivers]: https://www.ftdichip.com/Drivers/D2XX.htm
//! [FTDI Drivers Installation Guide for Linux]: http://www.ftdichip.cn/Support/Documents/AppNotes/AN_220_FTDI_Drivers_Installation_Guide_for_Linux.pdf
//! [libftd2xx]: https://github.com/newAM/libftd2xx-rs
//! [Rust Edition Guide]: https://doc.rust-lang.org/edition-guide/rust-2018/platform-and-target-support/musl-support-for-fully-static-binaries.html
#![doc(html_root_url = "https://docs.rs/libftd2xx-ffi/0.5.0")]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]

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
    } else if #[cfg(any(target_os = "linux", target_arch = "arm"))] {
        include!("bindings_linux_armv6.rs");
    } else if #[cfg(any(target_os = "linux", target_arch = "aarch64"))] {
        include!("bindings_linux_armv8.rs");
    } else {
        std::compile_error!("pre-generated bindings are not avaliable for your target");
    }
}
