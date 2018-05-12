//! Logs data from the motion sensors over ITM
//!
//! This example logs sensor data over ITM. The data is encoded in a binary format and logged as
//! COBS frames. The binary format is as follows:
//!
//! - Magnetometer readings
//!   - `mx`: `i16`, LE (Little Endian), 2 bytes
//!   - `my`: `i16`, LE, 2 bytes
//!   - `mz`: `i16`, LE, 2 bytes
//!
//! - Gyroscope readings
//!   - `arx`: `i16`, LE, 2 bytes
//!   - `ary`: `i16`, LE, 2 bytes
//!   - `arz`: `i16`, LE, 2 bytes
//!
//! - Accelerometer readings
//!   - `gx`: `i16`, LE, 2 bytes
//!   - `gy`: `i16`, LE, 2 bytes
//!   - `gz`: `i16`, LE, 2 bytes
//!
//! The suggested way to receive this data is to connect the F3 SWO pin to a UART to USB converter
//! and then to read out the associated device file using `itmdump`. Make sure you configure the
//! serial device before calling `itmdump`. The commands to run are:
//!
//! ``` console
//! $ stty -F /dev/ttyUSB0 raw 2000000 -echo
//!
//! $ itmdump -f /dev/ttyUSB0 > data.txt
//! ```
//!
//! You can plot this data using the `plot.py` script in the root of this crate.
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate aligned;
extern crate byteorder;
extern crate cobs;
extern crate cortex_m;
#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate f3;
#[macro_use(block)]
extern crate nb;
extern crate panic_semihosting;

use core::ptr;

use aligned::Aligned;
use byteorder::{ByteOrder, LE};
use cortex_m::{asm, itm};
use f3::hal::i2c::I2c;
use f3::hal::prelude::*;
use f3::hal::spi::Spi;
use f3::hal::stm32f30x;
use f3::hal::timer::Timer;
use f3::l3gd20::{self, Odr};
use f3::lsm303dlhc::{AccelOdr, MagOdr};
use f3::{L3gd20, Lsm303dlhc};
use rt::ExceptionFrame;

// TRY changing the sampling frequency
const FREQUENCY: u32 = 220;
// TRY changing the number of samples
const NSAMPLES: u32 = 32 * FREQUENCY; // = 32 seconds

entry!(main);

fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr
        .sysclk(64.mhz())
        .pclk1(32.mhz())
        .freeze(&mut flash.acr);

    // enable ITM
    // TODO this should be some high level API in the cortex-m crate
    unsafe {
        // enable TPIU and ITM
        cp.DCB.demcr.modify(|r| r | (1 << 24));

        // prescaler
        let swo_freq = 2_000_000;
        cp.TPIU.acpr.write((clocks.sysclk().0 / swo_freq) - 1);

        // SWO NRZ
        cp.TPIU.sppr.write(2);

        cp.TPIU.ffcr.modify(|r| r & !(1 << 1));

        // STM32 specific: enable tracing in the DBGMCU_CR register
        const DBGMCU_CR: *mut u32 = 0xe0042004 as *mut u32;
        let r = ptr::read_volatile(DBGMCU_CR);
        ptr::write_volatile(DBGMCU_CR, r | (1 << 5));

        // unlock the ITM
        cp.ITM.lar.write(0xC5ACCE55);

        cp.ITM.tcr.write(
            (0b000001 << 16) | // TraceBusID
            (1 << 3) | // enable SWO output
            (1 << 0), // enable the ITM
        );

        // enable stimulus port 0
        cp.ITM.ter[0].write(1);
    }

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);

    // I2C
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);

    // LSM303DLHC
    let mut lsm303dlhc = Lsm303dlhc::new(i2c).unwrap();
    lsm303dlhc.accel_odr(AccelOdr::Hz400).unwrap();
    lsm303dlhc.mag_odr(MagOdr::Hz220).unwrap();

    // SPI
    let mut nss = gpioe
        .pe3
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    nss.set_high();
    let sck = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);

    let spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        l3gd20::MODE,
        1.mhz(),
        clocks,
        &mut rcc.apb2,
    );

    // L3GD20
    let mut l3gd20 = L3gd20::new(spi, nss).unwrap();
    l3gd20.set_odr(Odr::Hz380).unwrap();

    // TIMER
    let mut timer = Timer::tim2(dp.TIM2, FREQUENCY.hz(), clocks, &mut rcc.apb1);

    // start of COBS frame
    itm::write_all(&mut cp.ITM.stim[0], &[0]);

    // Capture N samples
    let mut tx_buf: Aligned<u32, [u8; 20]> = Aligned([0; 20]);
    for _ in 0..NSAMPLES {
        block!(timer.wait()).unwrap();

        // Read sensors
        let m = lsm303dlhc.mag().unwrap();
        let ar = l3gd20.gyro().unwrap();
        let g = lsm303dlhc.accel().unwrap();

        // Serialize the data
        let mut buf = [0; 18];

        let mut start = 0;
        LE::write_i16(&mut buf[start..start + 2], m.x);
        start += 2;
        LE::write_i16(&mut buf[start..start + 2], m.y);
        start += 2;
        LE::write_i16(&mut buf[start..start + 2], m.z);
        start += 2;

        LE::write_i16(&mut buf[start..start + 2], ar.x);
        start += 2;
        LE::write_i16(&mut buf[start..start + 2], ar.y);
        start += 2;
        LE::write_i16(&mut buf[start..start + 2], ar.z);
        start += 2;

        LE::write_i16(&mut buf[start..start + 2], g.x);
        start += 2;
        LE::write_i16(&mut buf[start..start + 2], g.y);
        start += 2;
        LE::write_i16(&mut buf[start..start + 2], g.z);

        // Log data
        cobs::encode(&buf, &mut tx_buf);

        itm::write_aligned(&mut cp.ITM.stim[0], &tx_buf);
    }

    // Done
    asm::bkpt();

    loop {}
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
