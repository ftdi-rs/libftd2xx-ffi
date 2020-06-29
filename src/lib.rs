//! Rust FFI bindings to the [FTDI D2XX drivers].
//!
//! This crate is **just** the C bindings.
//! There is a separate project, [libftd2xx-rs],
//! which provides safe wrappers around the unsafe C bindings.
//!
//! Bindings are generated at build time with [bindgen].
//!
//! [FTDI D2XX drivers]: https://www.ftdichip.com/Drivers/D2XX.htm
//! [libftd2xx-rs]: https://github.com/newAM/libftd2xx-rs
//! [bindgen]: https://github.com/rust-lang/rust-bindgen

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
