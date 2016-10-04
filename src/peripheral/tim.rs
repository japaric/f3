//! basic TIMers
//!
//! # References
//!
//! - RM0316: STM32F303xC [Reference Manual] - Section 22.4 TIM6/7 Registers
//!
//! [Reference Manual]: http://www.st.com/resource/en/reference_manual/dm00043574.pdf

use volatile_register::{RW, WO};

#[repr(C)]
pub struct Registers {
    /// control register 1
    ///
    /// - `rw` `0` `CEN` Counter enable
    /// - `rw` `1` `UDIS` Update disable
    /// - `rw` `2` `URS` Update request source
    /// - `rw` `3` `OPM` One-pulse mode
    /// - `rw` `7` `ARPE` Auto-reload preload enable
    /// - `rw` `11` `UIFREMAP` UIF status bit remapping
    pub cr1: RW<u16>,

    reserved0: u16,

    /// control register 2
    ///
    /// - `rw` `4:7` `MMS` Master mode selection
    pub cr2: RW<u16>,

    reserved1: [u16; 3],

    /// DMA/Interrupt enable register
    ///
    /// - `rw` `8` `UDE` Update DMA request enable
    /// - `rw` `0` `UIE` Update interrupt enable
    pub dier: RW<u16>,

    reserved2: u16,

    /// status register
    ///
    /// - `rw` `0` `UIF` Update interrupt flag
    pub sr: RW<u16>,

    reserved3: u16,

    /// event generation register
    ///
    /// - `rw` `0` `UG` Update generation
    pub egr: WO<u16>,

    reserved4: [u16; 7],

    /// counter
    ///
    /// - `rw` `0:16` `CNT` Low counter value
    /// - `r-` `31` `UIFCPY` UIF Copy
    pub cnt: RW<u32>,

    /// prescaler
    ///
    /// - `rw` `0:16` `PSC` Prescaler value
    pub psc: RW<u16>,

    reserved5: u16,

    /// auto-reload register
    ///
    /// - `rw` `0:16` `ARR` Low Auto-reload value
    pub arr: RW<u16>,
}
