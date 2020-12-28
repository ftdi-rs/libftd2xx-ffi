# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
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

[Unreleased]: https://github.com/newAM/libftd2xx-ffi-rs/compare/0.4.1...HEAD
[0.4.1]: https://github.com/newAM/libftd2xx-ffi-rs/compare/0.4.0...0.4.1
