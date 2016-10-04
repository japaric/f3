//! General Purpose I/O
//!
//! # References
//!
//! - RM0316: STM32F303xC [Reference Manual] - Section 11.4 GPIO Registers
//!
//! [Reference Manual]: http://www.st.com/resource/en/reference_manual/dm00043574.pdf

use volatile_register::{RO, RW, WO};

#[repr(C)]
pub struct Registers {
    /// GPIO port mode register
    pub moder: RW<u32>,

    /// GPIO port output type register
    pub otyper: RW<u32>,

    /// GPIO port output speed register
    pub ospeedr: RW<u32>,

    /// GPIO port pull-up/pull-down register
    pub pupdr: RW<u32>,

    /// GPIO port input data register
    pub idr: RO<u32>,

    /// GPIO port output data register
    pub odr: RW<u32>,

    /// GPIO port bit set/reset register
    pub bsrr: WO<u32>,

    /// GPIO port configuration lock register
    pub lckr: RW<u32>,

    /// GPIO alternate function low register
    pub afrl: RW<u32>,

    /// GPIO alternate function high register
    pub afrh: RW<u32>,

    /// Port bit reset register
    pub brr: WO<u32>,
}
