# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Added

- A heap allocator which is enabled by default but can be disabled via the
  "alloc" Cargo feature.

### Changed

- Fixed the override-panic-fmt example to use the panic_fmt lang item rather
  than the more unstable symbol name (`rust_begin_unwind`)

## [v0.3.0] - 2016-11-14

### Added

- A "static-ram" opt-out Cargo feature to remove the RAM initialization routine.
  No `static mut` variables can be used if this feature has been disabled.

- An "interrupts" opt-out Cargo feature to remove the interrupts section of the
  vector table. Interrupts can't be used if this feature has been disabled.

### Changed

- [breaking] The `main` and `init` functions must now be a plain `fn` rather
  than `extern "C" fn`

- [breaking] The `exception::EXCEPTIONS`, `exception::reset` and
  `interrupt::INTERRUPTS` items have been removed.

## [v0.2.0] - 2016-10-27

### Added

- Initialize the FPU before main

- Support for sending `print!` formatted messages over "Serial Port".

- Overridable interrupts

- High level API for the LSM303DLHC and L3GD20

- A `time` module in the spirit of `std::time`

- Opt-out Cargo features to disable the default initialization code (`init`),
  the default exception handler and the default panic formatting (`panic_fmt`).

### Changed

- [breaking] The whole `peripheral` module has been revamped to provide type
  safe access to the *contents* of registers.

## [v0.1.0] - 2016-10-04

### Added

- High level API over LEDs

- A `delay::ms` function

- "Smart" exceptions

- `iprint!` macros

- Default `panic_fmt` implementation

- Default system initialization

- Low level access to some peripherals: DBGMCU, GPIO, RCC and TIM

[Unreleased]: https://github.com/japaric/f3/compare/v0.3.0...HEAD
[v0.3.0]: https://github.com/japaric/f3/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/japaric/f3/compare/v0.1.0...v0.2.0
