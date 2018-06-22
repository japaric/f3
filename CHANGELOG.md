# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.6.1] - 2018-06-22

### Added

- Re-add the "rt" feature mentioned in the documentation that was removed by mistake in v0.6.0.

### Fixed

- Building example with recent nightlies.

## [v0.6.0] - 2018-05-12

- [breaking-change] bumped the `stm32f30x-hal` dependency to v0.2.0.

- [breaking-change] this crate now requires `arm-none-eabi-gcc` to be installed and available on
  `$PATH` to build.

## [v0.5.3] - 2018-02-19

### Added

- Example: Madgwick's orientation filter
- Example: Logging sensor data over the ITM

## [v0.5.2] - 2018-01-20

### Added

- A "rt" Cargo feature that enables the "rt" feature of the stm32f30x-hal dependency.

## [v0.5.1] - 2018-01-17

### Changed

- Bumped the version of the cortex-m-rt dependency to 0.3.12

## [v0.5.0] - 2018-01-17

### Added

- Board specific APIs for the user LEDs.
- More concrete re-export of the `L3gd20` driver.
- More concrete re-export of the `Lsm303dlhc` driver.
- Re-export of the HAL provided by the `stm32f30x-hal` crate.

### Removed

- [breaking-change] All non-board specific APIs.

## [v0.4.1] - 2017-05-09

### Changed

- Bumped `stm32f30x` dependency to v0.4.1
- Updated the examples to match the stable release of the cortex-m-rtfm crate

## [v0.4.0] - 2017-04-28

### Changed

- [breaking-change] The startup routine has been removed from this crate. This
  crate is now meant to be used with the [cortex-m-quickstart] template, check
  the crate level documentation for details.

[cortex-m-quickstart]: https://docs.rs/cortex-m-quickstart/0.1.1/cortex_m_quickstart/

- [breaking-change] The whole API is now async only (check the `examples`
  module). Note that for this release we are not on parity with the v0.3.0 API
  in terms of functionality.

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

[Unreleased]: https://github.com/japaric/f3/compare/v0.6.1...HEAD
[v0.6.1]: https://github.com/japaric/f3/compare/v0.6.0...v0.6.1
[v0.6.0]: https://github.com/japaric/f3/compare/v0.5.3...v0.6.0
[v0.5.3]: https://github.com/japaric/f3/compare/v0.5.2...v0.5.3
[v0.5.2]: https://github.com/japaric/f3/compare/v0.5.1...v0.5.2
[v0.5.1]: https://github.com/japaric/f3/compare/v0.5.0...v0.5.1
[v0.5.0]: https://github.com/japaric/f3/compare/v0.4.1...v0.5.0
[v0.4.1]: https://github.com/japaric/f3/compare/v0.4.0...v0.4.1
[v0.4.0]: https://github.com/japaric/f3/compare/v0.3.1...v0.4.0
[v0.3.1]: https://github.com/japaric/f3/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/japaric/f3/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/japaric/f3/compare/v0.1.0...v0.2.0
