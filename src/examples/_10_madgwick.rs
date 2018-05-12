//! Madgwick's orientation filter
//!
//! This demo runs Madgwick's orientation filter and logs the orientation of the board as
//! quaternions via the ITM. The data is encoded in binary format and logged as COBS frame. The
//! binary format is as follows:
//!
//! - `w`: `f32`, LE (Little Endian), 4 bytes
//! - `x`: `f32`, LE, 4 bytes
//! - `y`: `f32`, LE, 4 bytes
//! - `z`: `f32`, LE, 4 bytes
//!
//! where the quaternion is the tuple `(w, x, y, z)`
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
//! You can pipe the quaternions through the `viz` program (shipped with this crate) to get real
//! time [visualization]. The command to run is:
//!
//! [visualization]: https://mobile.twitter.com/japaricious/status/962770003325005824
//!
//! ``` console
//! $ itmdump -f /dev/ttyUSB0 | viz
//! ```
//!
//! ```
//! // #![deny(unsafe_code)]
//! #![deny(warnings)]
//! #![no_main]
//! #![no_std]
//! 
//! #[macro_use(entry, exception)]
//! extern crate cortex_m_rt as rt;
//! extern crate aligned;
//! extern crate byteorder;
//! extern crate cast;
//! extern crate cobs;
//! extern crate cortex_m;
//! extern crate f3;
//! extern crate madgwick;
//! #[macro_use(block)]
//! extern crate nb;
//! extern crate panic_semihosting;
//! 
//! use core::f32::consts::PI;
//! use core::ptr;
//! 
//! use aligned::Aligned;
//! use byteorder::{ByteOrder, LE};
//! use cast::{f32, i32};
//! use cortex_m::itm;
//! use f3::hal::i2c::I2c;
//! use f3::hal::prelude::*;
//! use f3::hal::spi::Spi;
//! use f3::hal::stm32f30x;
//! use f3::hal::timer::Timer;
//! use f3::l3gd20::{self, Odr};
//! use f3::lsm303dlhc::{AccelOdr, MagOdr};
//! use f3::{L3gd20, Lsm303dlhc};
//! use madgwick::{F32x3, Marg};
//! use rt::ExceptionFrame;
//! 
//! // Number of samples to use for gyroscope calibration
//! const NSAMPLES: i32 = 256;
//! 
//! // Magnetometer calibration parameters
//! // NOTE you need to use the right parameters for *your* magnetometer
//! // You can use the `log-sensors` example to calibrate your magnetometer. The producer is explained
//! // in https://github.com/kriswiner/MPU6050/wiki/Simple-and-Effective-Magnetometer-Calibration
//! const M_BIAS_X: f32 = -34.;
//! const M_SCALE_X: f32 = 650.;
//! 
//! const M_BIAS_Y: f32 = -70.;
//! const M_SCALE_Y: f32 = 636.;
//! 
//! const M_BIAS_Z: f32 = -37.5;
//! const M_SCALE_Z: f32 = 589.5;
//! 
//! // Sensitivities of the accelerometer and gyroscope, respectively
//! const K_G: f32 = 2. / (1 << 15) as f32; // LSB -> g
//! const K_AR: f32 = 8.75e-3 * PI / 180.; // LSB -> rad/s
//! 
//! // Madgwick filter parameters
//! const SAMPLE_FREQ: u32 = 220;
//! const BETA: f32 = 1e-3;
//! 
//! entry!(main);
//! 
//! fn main() -> ! {
//!     let mut cp = cortex_m::Peripherals::take().unwrap();
//!     let dp = stm32f30x::Peripherals::take().unwrap();
//! 
//!     let mut flash = dp.FLASH.constrain();
//!     let mut rcc = dp.RCC.constrain();
//! 
//!     let clocks = rcc.cfgr
//!         .sysclk(64.mhz())
//!         .pclk1(32.mhz())
//!         .freeze(&mut flash.acr);
//! 
//!     // enable ITM
//!     // TODO this should be some high level API in the cortex-m crate
//!     unsafe {
//!         // enable TPIU and ITM
//!         cp.DCB.demcr.modify(|r| r | (1 << 24));
//! 
//!         // prescaler
//!         let swo_freq = 2_000_000;
//!         cp.TPIU.acpr.write((clocks.sysclk().0 / swo_freq) - 1);
//! 
//!         // SWO NRZ
//!         cp.TPIU.sppr.write(2);
//! 
//!         cp.TPIU.ffcr.modify(|r| r & !(1 << 1));
//! 
//!         // STM32 specific: enable tracing in the DBGMCU_CR register
//!         const DBGMCU_CR: *mut u32 = 0xe0042004 as *mut u32;
//!         let r = ptr::read_volatile(DBGMCU_CR);
//!         ptr::write_volatile(DBGMCU_CR, r | (1 << 5));
//! 
//!         // unlock the ITM
//!         cp.ITM.lar.write(0xC5ACCE55);
//! 
//!         cp.ITM.tcr.write(
//!             (0b000001 << 16) | // TraceBusID
//!             (1 << 3) | // enable SWO output
//!             (1 << 0), // enable the ITM
//!         );
//! 
//!         // enable stimulus port 0
//!         cp.ITM.ter[0].write(1);
//!     }
//! 
//!     let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
//!     let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
//!     let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
//! 
//!     let mut nss = gpioe
//!         .pe3
//!         .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
//!     nss.set_high();
//!     let mut led = gpioe
//!         .pe9
//!         .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
//! 
//!     let sck = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
//!     let miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
//!     let mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
//! 
//!     let spi = Spi::spi1(
//!         dp.SPI1,
//!         (sck, miso, mosi),
//!         l3gd20::MODE,
//!         1.mhz(),
//!         clocks,
//!         &mut rcc.apb2,
//!     );
//! 
//!     let mut l3gd20 = L3gd20::new(spi, nss).unwrap();
//! 
//!     l3gd20.set_odr(Odr::Hz380).unwrap();
//! 
//!     let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
//!     let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
//! 
//!     let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);
//! 
//!     let mut lsm303dlhc = Lsm303dlhc::new(i2c).unwrap();
//! 
//!     lsm303dlhc.accel_odr(AccelOdr::Hz400).unwrap();
//!     lsm303dlhc.mag_odr(MagOdr::Hz220).unwrap();
//! 
//!     let mut timer = Timer::tim2(dp.TIM2, 380.hz(), clocks, &mut rcc.apb1);
//! 
//!     // Calibrate the gyroscope
//!     let mut ar_bias_x = 0;
//!     let mut ar_bias_y = 0;
//!     let mut ar_bias_z = 0;
//!     for _ in 0..NSAMPLES {
//!         block!(timer.wait()).unwrap();
//! 
//!         let ar = l3gd20.gyro().unwrap();
//! 
//!         ar_bias_x += i32(ar.x);
//!         ar_bias_y += i32(ar.y);
//!         ar_bias_z += i32(ar.z);
//!     }
//!     let ar_bias_x = (ar_bias_x / NSAMPLES) as i16;
//!     let ar_bias_y = (ar_bias_y / NSAMPLES) as i16;
//!     let ar_bias_z = (ar_bias_z / NSAMPLES) as i16;
//! 
//!     // Turn on the LED after calibrating the gyroscope
//!     led.set_high();
//! 
//!     let mut marg = Marg::new(BETA, 1. / f32(SAMPLE_FREQ));
//!     let mut timer = Timer::tim2(timer.free(), SAMPLE_FREQ.hz(), clocks, &mut rcc.apb1);
//! 
//!     let mut tx_buf: Aligned<u32, [u8; 18]> = Aligned([0; 18]);
//!     loop {
//!         block!(timer.wait()).unwrap();
//! 
//!         let m = lsm303dlhc.mag().unwrap();
//!         let ar = l3gd20.gyro().unwrap();
//!         let g = lsm303dlhc.accel().unwrap();
//! 
//!         let m_x = (f32(m.x) - M_BIAS_X) / M_SCALE_X;
//!         let m_y = (f32(m.y) - M_BIAS_Y) / M_SCALE_Y;
//!         let m_z = (f32(m.z) - M_BIAS_Z) / M_SCALE_Z;
//! 
//!         // Fix the X Y Z components of the magnetometer so they match the gyro axes
//!         let m = F32x3 {
//!             x: m_y,
//!             y: -m_x,
//!             z: m_z,
//!         };
//! 
//!         let ar_x = f32(ar.x - ar_bias_x) * K_AR;
//!         let ar_y = f32(ar.y - ar_bias_y) * K_AR;
//!         let ar_z = f32(ar.z - ar_bias_z) * K_AR;
//!         let ar = F32x3 {
//!             x: ar_x,
//!             y: ar_y,
//!             z: ar_z,
//!         };
//! 
//!         // Fix the X Y Z components of the accelerometer so they match the gyro axes
//!         let g_x = f32(g.x) * K_G;
//!         let g_y = f32(g.y) * K_G;
//!         let g_z = f32(g.z) * K_G;
//!         let g = F32x3 {
//!             x: g_y,
//!             y: -g_x,
//!             z: g_z,
//!         };
//! 
//!         // Run the filter
//!         let quat = marg.update(m, ar, g);
//! 
//!         // Serialize the quaternion
//!         let mut start = 0;
//!         let mut buf = [0; 16];
//!         LE::write_f32(&mut buf[start..start + 4], quat.0);
//!         start += 4;
//!         LE::write_f32(&mut buf[start..start + 4], quat.1);
//!         start += 4;
//!         LE::write_f32(&mut buf[start..start + 4], quat.2);
//!         start += 4;
//!         LE::write_f32(&mut buf[start..start + 4], quat.3);
//!         // start += 4;
//! 
//!         // Log data
//!         cobs::encode(&buf, &mut tx_buf.array);
//! 
//!         itm::write_aligned(&mut cp.ITM.stim[0], &tx_buf);
//!     }
//! }
//! 
//! exception!(HardFault, hard_fault);
//! 
//! fn hard_fault(ef: &ExceptionFrame) -> ! {
//!     panic!("{:#?}", ef);
//! }
//! 
//! exception!(*, default_handler);
//! 
//! fn default_handler(irqn: i16) {
//!     panic!("Unhandled exception (IRQn = {})", irqn);
//! }
//! ```
// Auto-generated. Do not modify.
