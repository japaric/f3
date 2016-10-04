//! Low-level access to peripherals
//!
//! > **WARNING** This might void your warranty!
//!
//! This API does a good job (I think) at limiting read/write access to (memory mapped) registers
//! and at marking potential sources of data races (mutably aliasing registers) as `unsafe`. **But**
//! it does nothing to validate the contents that will be written to a register so you might end up
//! modifying the "reserved" parts of a register. However, where possible, the documentation
//! indicates what parts of a register can be modified (i.e. are not reserved) and the read/write
//! permissions of the parts of a register.
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

// AHB2
const GPIOA: usize = 0x48000000;
const GPIOB: usize = 0x48000400;
const GPIOC: usize = 0x48000800;
const GPIOD: usize = 0x48000c00;
const GPIOE: usize = 0x48001000;
const GPIOF: usize = 0x48001400;

// AHB1
const RCC: usize = 0x40021000;

// APB1
const TIM6: usize = 0x40001000;
const TIM7: usize = 0x40001400;

// PPB
const DBGMCU: usize = 0xe0042000;

pub fn dbgmcu() -> &'static dbgmcu::Registers {
    unsafe { deref(DBGMCU) }
}

pub unsafe fn dbgmcu_mut() -> &'static mut dbgmcu::Registers {
    deref_mut(DBGMCU)
}

pub fn gpioa() -> &'static gpio::Registers {
    unsafe { deref(GPIOA) }
}

pub unsafe fn gpioa_mut() -> &'static mut gpio::Registers {
    deref_mut(GPIOA)
}

pub fn gpiob() -> &'static gpio::Registers {
    unsafe { deref(GPIOB) }
}

pub unsafe fn gpiob_mut() -> &'static mut gpio::Registers {
    deref_mut(GPIOB)
}

pub fn gpioc() -> &'static gpio::Registers {
    unsafe { deref(GPIOC) }
}

pub unsafe fn gpioc_mut() -> &'static mut gpio::Registers {
    deref_mut(GPIOC)
}

pub fn gpiod() -> &'static gpio::Registers {
    unsafe { deref(GPIOD) }
}

pub unsafe fn gpiod_mut() -> &'static mut gpio::Registers {
    deref_mut(GPIOD)
}

pub fn gpioe() -> &'static gpio::Registers {
    unsafe { deref(GPIOE) }
}

pub unsafe fn gpioe_mut() -> &'static mut gpio::Registers {
    deref_mut(GPIOE)
}

pub fn gpiof() -> &'static gpio::Registers {
    unsafe { deref(GPIOF) }
}

pub unsafe fn gpiof_mut() -> &'static mut gpio::Registers {
    deref_mut(GPIOF)
}

pub fn rcc() -> &'static rcc::Registers {
    unsafe { deref(RCC) }
}

pub unsafe fn rcc_mut() -> &'static mut rcc::Registers {
    deref_mut(RCC)
}

pub fn tim6() -> &'static tim::Registers {
    unsafe { deref(TIM6) }
}

pub unsafe fn tim6_mut() -> &'static mut tim::Registers {
    deref_mut(TIM6)
}

pub fn tim7() -> &'static tim::Registers {
    unsafe { deref(TIM7) }
}

pub unsafe fn tim7_mut() -> &'static mut tim::Registers {
    deref_mut(TIM7)
}

unsafe fn deref<T>(address: usize) -> &'static T {
    &*(address as *const T)
}

unsafe fn deref_mut<T>(address: usize) -> &'static mut T {
    &mut *(address as *mut T)
}
