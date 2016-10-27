# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

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

[Unreleased]: https://github.com/japaric/f3/compare/v0.2.0...HEAD
[v0.2.0]: https://github.com/japaric/f3/compare/v0.1.0...v0.2.0
