![Maintenance](https://img.shields.io/badge/maintenance-passively--maintained-yellowgreen.svg)
[![crates.io](https://img.shields.io/crates/v/libftd2xx-ffi.svg)](https://crates.io/crates/libftd2xx-ffi)
[![docs.rs](https://docs.rs/libftd2xx-ffi/badge.svg)](https://docs.rs/libftd2xx-ffi/)
[![CI](https://github.com/newAM/libftd2xx-ffi-rs/workflows/CI/badge.svg)](https://github.com/newAM/libftd2xx-ffi-rs/actions)

# libftd2xx-ffi

Rust FFI bindings to the [FTDI D2XX drivers].

This crate is **just** the C bindings.
There is a separate crate, [libftd2xx], which provides safe wrappers around
the unsafe C bindings.

## Usage
Simply add this crate as a dependency in your `Cargo.toml`.
The static library is distributed in this crate with permission from FTDI.
The default feature set will use dynamic linking.

```toml
[dependencies]
libftd2xx-ffi = "~0.8.0"
```

### Bindgen
The default feature set will use pre-generated bindings.
This is only available for Windows x86_64 and Linux x86_64 platforms.

The bindings can also be generated during compilation using the [bindgen]
feature flag.
```toml
[dependencies]
libftd2xx-ffi = { version = "~0.8.0", features = ["bindgen"] }
```

Bindgen has additional dependencies that must be installed in order to
compile successfully, see the [bindgen requirements] page for more details.

### Static Linking
Static linking the FTD2XX library into this crate can be done by using
the static feature flag.
```toml
[dependencies]
libftd2xx-ffi = { version = "~0.8.0", features = ["static"] }
```
For GNU/Linux users, no further work is needed.
Technically this may be preferred, however there may be license
incompatibilities (static linking with GPL code).
If in doubt, check the FTDI [driver license terms].

On Windows, we rely on MSVC and a manually set "LIBMSVC_PATH" environment
variable.
For example a possible 2019 Community installation path may be:
```
C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Tools\MSVC\14.28.29333\lib\
```
This brings in `legacy_stdio_definitions.lib` and `user32.lib`.
It seems to play nicely with rust, but you may end up with multiple defined
symbol errors if using this crate as a c/c++ dependency.

## Supported Targets

### Tested Targets

* `aarch64-unknown-linux-gnu` (dynamic + static)
* `aarch64-unknown-linux-musl` (static)
* `i686-pc-windows-msvc` (dynamic + static)
* `i686-unknown-linux-gnu` (dynamic + static)
* `i686-unknown-linux-musl` (static)
* `x86_64-pc-windows-msvc` (dynamic + static)
* `x86_64-unknown-linux-gnu` (dynamic + static)
* `x86_64-unknown-linux-musl` (static)

### Untested Targets

These targets are provided, but they are untested.
Use at your own risk.

* `arm-unknown-linux-gnueabihf` (dynamic + static)
* `arm-unknown-linux-musleabihf` (static)
* `armv7-unknown-linux-gnueabihf` (dynamic + static)
* `armv7-unknown-linux-musleabihf` (static)
* `x86_64-apple-darwin` (dynamic)

## References

* [D2XX Programmers Guide V1.4]
* [D2XX Drivers Download Page]

## Troubleshooting
### Unknown Device on Linux
Remove the VCP FTDI driver.
```bash
sudo rmmod ftdi_sio
sudo rmmod usbserial
```
See [FTDI Drivers Installation Guide for Linux] for more details.

## License
FTDI provides the D2XX driver as a compiled library and a header file.
These files can be found within the `vendor` directory.

The code within the `vendor` directory is licensed by FTDI.
Please see the [driver license terms] page for their license.

All code outside of the `vendor` directory is MIT licensed.

**Note:** This crate is not affiliated with FTDI.
You will need to contact the vendor for any support requests with the
underlying library because it is closed source.

[bindgen requirements]: https://rust-lang.github.io/rust-bindgen/requirements.html
[bindgen]: https://github.com/rust-lang/rust-bindgen
[D2XX Drivers Download Page]: https://www.ftdichip.com/Drivers/D2XX.htm
[D2xx Programmers Guide V1.4]: https://ftdichip.com/document/programming-guides/
[driver license terms]: https://ftdichip.com/driver-licence-terms-details/
[FTDI D2XX drivers]: https://www.ftdichip.com/Drivers/D2XX.htm
[FTDI Drivers Installation Guide for Linux]: http://www.ftdichip.cn/Support/Documents/AppNotes/AN_220_FTDI_Drivers_Installation_Guide_for_Linux.pdf
[libftd2xx]: https://github.com/newAM/libftd2xx-rs
[Rust Edition Guide]: https://doc.rust-lang.org/edition-guide/rust-2018/platform-and-target-support/musl-support-for-fully-static-binaries.html
