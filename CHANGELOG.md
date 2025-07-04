# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Updated pre-generated bindings from bindgen `0.58.1` to `0.72.0` for Linux and Windows.
- Updated pre-generated bindings from bindgen `0.59.1` to `0.72.0` for Mac.

### Updated

- Updated bindgen dependency from `0.70.1` to `0.72.0`.
- Updated the edition from 2021 to 2024.

## [0.8.7] - 2024-12-01

### Added

- Added support for the `aarch64-pc-windows-msvc` target.

## [0.8.6] - 2022-11-27

### Added

- Added binaries for Windows targets using the GNU toolchain.

## [0.8.5] - 2022-09-17

### Added

- Added the `links` manifest key.

### Fixed

- Fixed path segment separators when cross-compiling for Windows from Linux.

## [0.8.4] - 2021-08-14

### Added

- Added pre-generated bindings for `x86_64-apple-darwin`.

## [0.8.3] - 2021-06-28

### Removed

- Removed `legacy_stdio_definitions` and `user32` linking requirements on
  Windows.

## [0.8.2] - 2021-06-27

### Fixed

- Fixed static linking on Windows.

### Changed

- Updated vendor library from 2.12.36.1 to 2.12.36.2 on Windows.

## [0.8.1] - 2021-06-19

### Added

- Reinstated support for 32-bit Linux targets.

## [0.8.0] - 2021-06-12

### Added

- Added support for `x86_64-apple-darwin`, dynamic linking only.

### Changed

- Changed the license for non-vendor material to dual MIT and 0BSD.
- Updated the Linux library version from `1.4.22` to `1.4.24`.
- Updated the Windows library version to `2.12.36.1`.

### Removed

- Removed support for 32-bit Linux targets, the vendor no longer supports these.

## [0.7.0] - 2021-03-25

### Added

- Added the static feature flag to enable switching between static and dynamic
  linking of the vendor library.

### Changed

- Changed the default linking strategy on Linux targets to dynamic.
  **Note:** To retain previous functionality with dynamic linking on Windows and
  static linking on Linux use cargo's [resolver version] 2.

## [0.6.0] - 2021-02-27

### Changed

- Updated the vendor library from `1.4.8` to `1.4.22` for linux targets.

## [0.5.1] - 2021-01-10

### Fixed

- Fixed a bug with ARM targets selecting the wrong bindings.

## [0.5.0] - 2020-12-28

### Added

- Added pre-generated bindings for `i686-unknown-linux` targets.
- Added pre-generated bindings for the `i686-pc-windows-msvc` target.
- Added support for `arm-unknown-linux`, `armv7-unknown-linux`, and `aarch64-unknown-linux` targets.

## [0.4.1] - 2020-12-27

### Added

- Added a changelog.

### Changed

- Clarified per-target linking.
- Update bindgen to 0.56.

### Fixed

- Fixed `build.rs` script to handle cross compilation.

[Unreleased]: https://github.com/ftdi-rs/libftd2xx-ffi/compare/0.8.7...HEAD
[0.8.7]: https://github.com/ftdi-rs/libftd2xx-ffi/compare/0.8.6...0.8.7
[0.8.6]: https://github.com/ftdi-rs/libftd2xx-ffi/compare/0.8.5...0.8.6
[0.8.5]: https://github.com/ftdi-rs/libftd2xx-ffi/compare/0.8.4...0.8.5
[0.8.4]: https://github.com/ftdi-rs/libftd2xx-ffi/compare/0.8.3...0.8.4
[0.8.3]: https://github.com/ftdi-rs/libftd2xx-ffi/compare/0.8.2...0.8.3
[0.8.2]: https://github.com/ftdi-rs/libftd2xx-ffi/compare/0.8.1...0.8.2
[0.8.1]: https://github.com/ftdi-rs/libftd2xx-ffi/compare/0.8.0...0.8.1
[0.8.0]: https://github.com/ftdi-rs/libftd2xx-ffi/compare/0.7.0...0.8.0
[0.7.0]: https://github.com/ftdi-rs/libftd2xx-ffi/compare/0.6.0...0.7.0
[0.6.0]: https://github.com/ftdi-rs/libftd2xx-ffi/compare/0.5.1...0.6.0
[0.5.1]: https://github.com/ftdi-rs/libftd2xx-ffi/compare/0.5.0...0.5.1
[0.5.0]: https://github.com/ftdi-rs/libftd2xx-ffi/compare/0.4.1...0.5.0
[0.4.1]: https://github.com/ftdi-rs/libftd2xx-ffi/compare/0.4.0...0.4.1
[resolver version]: https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
