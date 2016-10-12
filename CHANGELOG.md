# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Added

- Initialize the FPU before main
- Support for sending `print!` formatted messages over "Serial Port".
- Overridable interrupts
- the low level API over peripherals is now aware of the bit fields/flags of each register
- low level access to I2C and SPI
- high level API for the LSM303DLHC and L3GD20

### Changed

- The `delay` module now uses an interrupt instead of busy polling.
- The whole `peripheral` module has been revamped to provide type safe access to the *contents* of
  the registers.

## [v0.1.0] - 2016-10-04

### Added

- High level API over LEDs
- A `delay::ms` function
- "Smart" exceptions
- `iprint!` macros
- Default `panic_fmt` implementation
- Default system initialization
- Low level access to some peripherals: DBGMCU, GPIO, RCC and TIM

[Unreleased]: https://github.com/japaric/f3/compare/v0.1.0...HEAD
