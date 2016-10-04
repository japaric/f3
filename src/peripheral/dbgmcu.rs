//! MCU DeBuG component

use volatile_register::{RO, RW};

#[repr(C)]
pub struct Registers {
    /// MCU Device ID Code Register
    ///
    /// - `rw` `0:12` `DEV_ID` Device Identifier
    /// - `rw` `16:32` `REV_ID` Revision Identifier
    pub idcode: RO<u32>,

    /// Debug MCU Configuration Register
    ///
    /// - `rw` `0` `DBG_SLEEP` Debug Sleep mode
    /// - `rw` `1` `DBG_STOP` Debug Stop Mode
    /// - `rw` `2` `DBG_STANDBY` Debug Standby Mode
    /// - `rw` `5` `TRACE_IOEN` Trace pin assignment control
    /// - `rw` `6:8` `TRACE_MODE` Trace pin assignment control
    pub cr: RW<u32>,

    /// APB Low Freeze Register
    ///
    /// - `rw` `0` `DBG_TIM2_STOP` Debug Timer 2 stopped when Core is halted
    /// - `rw` `1` `DBG_TIM3_STOP` Debug Timer 3 stopped when Core is halted
    /// - `rw` `2` `DBG_TIM4_STOP` Debug Timer 4 stopped when Core is halted
    /// - `rw` `3` `DBG_TIM5_STOP` Debug Timer 5 stopped when Core is halted
    /// - `rw` `4` `DBG_TIM6_STOP` Debug Timer 6 stopped when Core is halted
    /// - `rw` `5` `DBG_TIM7_STOP` Debug Timer 7 stopped when Core is halted
    /// - `rw` `6` `DBG_TIM12_STOP` Debug Timer 12 stopped when Core is halted
    /// - `rw` `7` `DBG_TIM13_STOP` Debug Timer 13 stopped when Core is halted
    /// - `rw` `8` `DBG_TIMER14_STOP` Debug Timer 14 stopped when Core is halted
    /// - `rw` `9` `DBG_TIM18_STOP` Debug Timer 18 stopped when Core is halted
    /// - `rw` `10` `DBG_RTC_STOP` Debug RTC stopped when Core is halted
    /// - `rw` `11` `DBG_WWDG_STOP` Debug Window Wachdog stopped when Core is halted
    /// - `rw` `12` `DBG_IWDG_STOP` Debug Independent Wachdog stopped when Core is halted
    /// - `rw` `21` `I2C1_SMBUS_TIMEOUT` SMBUS timeout mode stopped when Core is halted
    /// - `rw` `22` `I2C2_SMBUS_TIMEOUT` SMBUS timeout mode stopped when Core is halted
    /// - `rw` `25` `DBG_CAN_STOP` Debug CAN stopped when core is halted
    pub apb1fz: RW<u32>,

    /// APB High Freeze Register
    ///
    /// - `rw` `2` `DBG_TIM15_STOP` Debug Timer 15 stopped when Core is halted
    /// - `rw` `3` `DBG_TIM16_STOP` Debug Timer 16 stopped when Core is halted
    /// - `rw` `4` `DBG_TIM17_STO` Debug Timer 17 stopped when Core is halted
    /// - `rw` `5` `DBG_TIM19_STOP` Debug Timer 19 stopped when Core is halted
    pub apb2fz: RW<u32>,
}
