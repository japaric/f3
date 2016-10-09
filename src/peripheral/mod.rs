//! Low-level access to peripherals
//!
//! > **WARNING** This might void your warranty!
//!
//! # Notes
//!
//! - Although the `*_mut()` functions always return a valid/live reference, the API doesn't prevent
//!   the user from creating multiple mutable aliases. It's up to the user to ensure that no
//!   unsynchonized concurrent access is performed through these references.

pub mod dbgmcu;
pub mod gpio;
pub mod rcc;
pub mod tim;
pub mod usart;

use self::dbgmcu::Dbgmcu;
use self::gpio::Gpio;
use self::rcc::Rcc;
use self::tim::Tim;
use self::usart::Usart;

const GPIOA: usize = 0x48000000;
const GPIOB: usize = 0x48000400;
const GPIOC: usize = 0x48000800;
const GPIOD: usize = 0x48000c00;
const GPIOE: usize = 0x48001000;
const GPIOF: usize = 0x48001400;
// const TSC: usize = 0x40024000;
// const CRC: usize = 0x40023000;
// const Flash: usize = 0x40022000;
const RCC: usize = 0x40021000;
// const DMA1: usize = 0x40020000;
// const DMA2: usize = 0x40020400;
// const TIM2: usize = 0x40000000;
// const TIM3: usize = 0x40000400;
// const TIM4: usize = 0x40000800;
// const TIM15: usize = 0x40014000;
// const TIM16: usize = 0x40014400;
// const TIM17: usize = 0x40014800;
const USART1: usize = 0x40013800;
// const USART2: usize = 0x40004400;
// const USART3: usize = 0x40004800;
// const UART4: usize = 0x40004c00;
// const UART5: usize = 0x40005000;
// const SPI1: usize = 0x40013000;
// const SPI2: usize = 0x40003800;
// const SPI3: usize = 0x40003c00;
// const I2S2ext: usize = 0x40003400;
// const I2S3ext: usize = 0x40004000;
// const EXTI: usize = 0x40010400;
// const COMP: usize = 0x4001001c;
// const PWR: usize = 0x40007000;
// const CAN: usize = 0x40006400;
// const USB_FS: usize = 0x40005c00;
// const I2C1: usize = 0x40005400;
// const I2C2: usize = 0x40005800;
// const IWDG: usize = 0x40003000;
// const WWDG: usize = 0x40002c00;
// const RTC: usize = 0x40002800;
const TIM6: usize = 0x40001000;
const TIM7: usize = 0x40001400;
// const DAC: usize = 0x40007400;
// const NVIC: usize = 0xe000e000;
// const FPU: usize = 0xe000ed88;
const DBGMCU: usize = 0xe0042000;
// const TIM1: usize = 0x40012c00;
// const TIM8: usize = 0x40013400;
// const ADC1: usize = 0x50000000;
// const ADC2: usize = 0x50000100;
// const ADC3: usize = 0x50000400;
// const ADC4: usize = 0x50000500;
// const ADC1_2: usize = 0x50000300;
// const ADC3_4: usize = 0x50000700;
// const SYSCFG: usize = 0x40010000;
// const OPAMP: usize = 0x40010038;

pub fn dbgmcu() -> &'static Dbgmcu {
    unsafe { deref(DBGMCU) }
}

pub unsafe fn dbgmcu_mut() -> &'static mut Dbgmcu {
    deref_mut(DBGMCU)
}

pub fn gpioa() -> &'static Gpio {
    unsafe { deref(GPIOA) }
}

pub unsafe fn gpioa_mut() -> &'static mut Gpio {
    deref_mut(GPIOA)
}

pub fn gpiob() -> &'static Gpio {
    unsafe { deref(GPIOB) }
}

pub unsafe fn gpiob_mut() -> &'static mut Gpio {
    deref_mut(GPIOB)
}

pub fn gpioc() -> &'static Gpio {
    unsafe { deref(GPIOC) }
}

pub unsafe fn gpioc_mut() -> &'static mut Gpio {
    deref_mut(GPIOC)
}

pub fn gpiod() -> &'static Gpio {
    unsafe { deref(GPIOD) }
}

pub unsafe fn gpiod_mut() -> &'static mut Gpio {
    deref_mut(GPIOD)
}

pub fn gpioe() -> &'static Gpio {
    unsafe { deref(GPIOE) }
}

pub unsafe fn gpioe_mut() -> &'static mut Gpio {
    deref_mut(GPIOE)
}

pub fn gpiof() -> &'static Gpio {
    unsafe { deref(GPIOF) }
}

pub unsafe fn gpiof_mut() -> &'static mut Gpio {
    deref_mut(GPIOF)
}

pub fn rcc() -> &'static Rcc {
    unsafe { deref(RCC) }
}

pub unsafe fn rcc_mut() -> &'static mut Rcc {
    deref_mut(RCC)
}

pub fn tim6() -> &'static Tim {
    unsafe { deref(TIM6) }
}

pub unsafe fn tim6_mut() -> &'static mut Tim {
    deref_mut(TIM6)
}

pub fn tim7() -> &'static Tim {
    unsafe { deref(TIM7) }
}

pub unsafe fn tim7_mut() -> &'static mut Tim {
    deref_mut(TIM7)
}

pub fn usart1() -> &'static Usart {
    unsafe { deref(USART1) }
}

pub unsafe fn usart1_mut() -> &'static mut Usart {
    deref_mut(USART1)
}

unsafe fn deref<T>(address: usize) -> &'static T {
    &*(address as *const T)
}

unsafe fn deref_mut<T>(address: usize) -> &'static mut T {
    &mut *(address as *mut T)
}
