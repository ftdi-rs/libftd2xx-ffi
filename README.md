# libftd2xx-ffi

Rust FFI bindings to the [FTDI D2XX drivers].

This crate is **just** the C bindings.  There is a separate project, [libftd2xx-rs], which provides safe wrappers around the unsafe C bindings.

## License
The underlying compiled C libraries within `libftd2xx_src` directory is from FTDI.

Please see the [Driver License Terms] for the license of the code within the `libftd2xx_src` directory.

All code outside of the `libftd2xx_src` directory is MIT licensed.

[Driver License Terms]: https://www.ftdichip.com/Drivers/FTDriverLicenceTermsSummary.htm
[FTDI D2XX drivers]: https://www.ftdichip.com/Drivers/D2XX.htm
[libftd2xx-rs]: https://github.com/newAM/libftd2xx-rs
