//! Reset and Clock Control
//!
//! # References
//!
//! - RM0316: STM32F303xC [Reference Manual] - Section 9.4 RCC Registers
//!
//! [Reference Manual]: http://www.st.com/resource/en/reference_manual/dm00043574.pdf

use volatile_register::RW;

#[repr(C)]
pub struct Registers {
    /// Clock control register
    ///
    /// - `rw` `0` `HSION` Internal High Speed clock enable
    /// - `r-` `1` `HSIRDY` Internal High Speed clock ready flag
    /// - `rw` `3:8` `HSITRIM` Internal High Speed clock trimming
    /// - `r-` `8:16` `HSICAL` Internal High Speed clock Calibration
    /// - `rw` `16` `HSEON` External High Speed clock enable
    /// - `r-` `17` `HSERDY` External High Speed clock ready flag
    /// - `rw` `18` `HSEBYP` External High Speed clock Bypass
    /// - `rw` `19` `CSSON` Clock Security System enable
    /// - `rw` `24` `PLLON` PLL enable
    /// - `r-` `25` `PLLRDY` PLL clock ready flag
    pub cr: RW<u32>,

    /// Clock configuration register (RCC_CFGR)
    ///
    /// - `rw` `0:2` `SW` System clock Switch
    /// - `r-` `2:4` `SWS` System Clock Switch Status
    /// - `rw` `4:8` `HPRE` AHB prescaler
    /// - `rw` `8:11` `PPRE1` APB Low speed prescaler (APB1)
    /// - `rw` `11:14` `PPRE2` APB high speed prescaler (APB2)
    /// - `rw` `16` `PLLSRC` PLL entry clock source
    /// - `rw` `17` `PLLXTPRE` HSE divider for PLL entry
    /// - `rw` `18:22` `PLLMUL` PLL Multiplication Factor
    /// - `rw` `22` `USBPRES` USB prescaler
    /// - `rw` `23` `I2SSRC` I2S external clock source selection
    /// - `rw` `24:27` `MCO` Microcontroller clock output
    /// - `r-` `28:31` `MCOPRE` MCOPRE
    /// - `rw` `31` `PLLNODIV` PLLNODIV
    pub cfgr: RW<u32>,

    /// Clock interrupt register (RCC_CIR)
    ///
    /// - `r-` `0` `LSIRDYF` LSI Ready Interrupt flag
    /// - `r-` `1` `LSERDYF` LSE Ready Interrupt flag
    /// - `r-` `2` `HSIRDYF` HSI Ready Interrupt flag
    /// - `r-` `3` `HSERDYF` HSE Ready Interrupt flag
    /// - `r-` `4` `PLLRDYF` PLL Ready Interrupt flag
    /// - `r-` `7` `CSSF` Clock Security System Interrupt flag
    /// - `rw` `8` `LSIRDYIE` LSI Ready Interrupt Enable
    /// - `rw` `9` `LSERDYIE` LSE Ready Interrupt Enable
    /// - `rw` `10` `HSIRDYIE` HSI Ready Interrupt Enable
    /// - `rw` `11` `HSERDYIE` HSE Ready Interrupt Enable
    /// - `rw` `12` `PLLRDYIE` PLL Ready Interrupt Enable
    /// - `-w` `16` `LSIRDYC` LSI Ready Interrupt Clear
    /// - `-w` `17` `LSERDYC` LSE Ready Interrupt Clear
    /// - `-w` `18` `HSIRDYC` HSI Ready Interrupt Clear
    /// - `-w` `19` `HSERDYC` HSE Ready Interrupt Clear
    /// - `-w` `20` `PLLRDYC` PLL Ready Interrupt Clear
    /// - `-w` `23` `CSSC` Clock security system interrupt clear
    pub cir: RW<u32>,

    /// APB2 peripheral reset register (RCC_APB2RSTR)
    ///
    /// - `rw` `0` `SYSCFGRST` SYSCFG and COMP reset
    /// - `rw` `11` `TIM1RST` TIM1 timer reset
    /// - `rw` `12` `SPI1RST` SPI 1 reset
    /// - `rw` `14` `USART1RST` USART1 reset
    /// - `rw` `16` `TIM15RST` TIM15 timer reset
    /// - `rw` `17` `TIM16RST` TIM16 timer reset
    /// - `rw` `18` `TIM17RST` TIM17 timer reset
    /// - `rw` `29` `HRTIM1RST` HRTIM1 reset
    pub apb2rstr: RW<u32>,

    /// APB1 peripheral reset register (RCC_APB1RSTR)
    ///
    /// - `rw` `0` `TIM2RST` Timer 2 reset
    /// - `rw` `1` `TIM3RST` Timer 3 reset
    /// - `rw` `4` `TIM6RST` Timer 6 reset
    /// - `rw` `5` `TIM7RST` Timer 7 reset
    /// - `rw` `11` `WWDGRST` Window watchdog reset
    /// - `rw` `17` `USART2RST` USART 2 reset
    /// - `rw` `18` `USART3RST` USART3 reset
    /// - `rw` `21` `I2C1RST` I2C1 reset
    /// - `rw` `23` `USBRST` USB reset
    /// - `rw` `25` `CANRST` CAN reset
    /// - `rw` `26` `DAC2RST` DAC2 interface reset
    /// - `rw` `28` `PWRRST` Power interface reset
    /// - `rw` `29` `DAC1RST` DAC1 interface reset
    pub apb1rstr: RW<u32>,

    /// AHB Peripheral Clock enable register (RCC_AHBENR)
    ///
    /// - `rw` `0` `DMAEN` DMA1 clock enable
    /// - `rw` `2` `SRAMEN` SRAM interface clock enable
    /// - `rw` `4` `FLITFEN` FLITF clock enable
    /// - `rw` `6` `CRCEN` CRC clock enable
    /// - `rw` `17` `IOPAEN` I/O port A clock enable
    /// - `rw` `18` `IOPBEN` I/O port B clock enable
    /// - `rw` `19` `IOPCEN` I/O port C clock enable
    /// - `rw` `20` `IOPDEN` I/O port D clock enable
    /// - `rw` `21` `IOPEEN` I/O port E clock enable
    /// - `rw` `22` `IOPFEN` I/O port F clock enable
    /// - `rw` `24` `TSCEN` Touch sensing controller clock enable
    /// - `rw` `28` `ADC12EN` ADC1 and ADC2 clock enable
    pub ahbenr: RW<u32>,

    /// APB2 peripheral clock enable register (RCC_APB2ENR)
    ///
    /// - `rw` `0` `SYSCFGEN` SYSCFG clock enable
    /// - `rw` `11` `TIM1EN` TIM1 Timer clock enable
    /// - `rw` `12` `SPI1EN` SPI 1 clock enable
    /// - `rw` `14` `USART1EN` USART1 clock enable
    /// - `rw` `16` `TIM15EN` TIM15 timer clock enable
    /// - `rw` `17` `TIM16EN` TIM16 timer clock enable
    /// - `rw` `18` `TIM17EN` TIM17 timer clock enable
    /// - `rw` `29` `HRTIM1EN` HRTIM1 clock enable
    pub apb2enr: RW<u32>,

    /// APB1 peripheral clock enable register (RCC_APB1ENR)
    ///
    /// - `rw` `0` `TIM2EN` Timer 2 clock enable
    /// - `rw` `1` `TIM3EN` Timer 3 clock enable
    /// - `rw` `4` `TIM6EN` Timer 6 clock enable
    /// - `rw` `5` `TIM7EN` Timer 7 clock enable
    /// - `rw` `11` `WWDGEN` Window watchdog clock enable
    /// - `rw` `17` `USART2EN` USART 2 clock enable
    /// - `rw` `21` `I2C1EN` I2C 1 clock enable
    /// - `rw` `25` `CANEN` CAN clock enable
    /// - `rw` `28` `PWREN` Power interface clock enable
    /// - `rw` `29` `DAC1EN` DAC1 interface clock enable
    /// - `rw` `18` `USART3EN` USART 3 clock enable
    /// - `rw` `26` `DAC2EN` DAC2 clock enable
    pub apb1enr: RW<u32>,

    /// Backup domain control register (RCC_BDCR)
    ///
    /// - `rw` `0` `LSEON` External Low Speed oscillator enable
    /// - `r-` `1` `LSERDY` External Low Speed oscillator ready
    /// - `rw` `2` `LSEBYP` External Low Speed oscillator bypass
    /// - `rw` `3:5` `LSEDRV` LSE oscillator drive capability
    /// - `rw` `8:10` `RTCSEL` RTC clock source selection
    /// - `rw` `15` `RTCEN` RTC clock enable
    /// - `rw` `16` `BDRST` Backup domain software reset
    pub bdcr: RW<u32>,

    /// Control/status register (RCC_CSR)
    ///
    /// - `rw` `0` `LSION` Internal low speed oscillator enable
    /// - `r-` `1` `LSIRDY` Internal low speed oscillator ready
    /// - `rw` `24` `RMVF` Remove reset flag
    /// - `rw` `25` `OBLRSTF` Option byte loader reset flag
    /// - `rw` `26` `PINRSTF` PIN reset flag
    /// - `rw` `27` `PORRSTF` POR/PDR reset flag
    /// - `rw` `28` `SFTRSTF` Software reset flag
    /// - `rw` `29` `IWDGRSTF` Independent watchdog reset flag
    /// - `rw` `30` `WWDGRSTF` Window watchdog reset flag
    /// - `rw` `31` `LPWRRSTF` Low-power reset flag
    pub csr: RW<u32>,

    /// AHB peripheral reset register
    ///
    /// - `rw` `17` `IOPARST` I/O port A reset
    /// - `rw` `18` `IOPBRST` I/O port B reset
    /// - `rw` `19` `IOPCRST` I/O port C reset
    /// - `rw` `20` `IOPDRST` I/O port D reset
    /// - `rw` `22` `IOPFRST` I/O port F reset
    /// - `rw` `24` `TSCRST` Touch sensing controller reset
    /// - `rw` `28` `ADC12RST` ADC1 and ADC2 reset
    pub ahbrstr: RW<u32>,

    /// Clock configuration register 2
    ///
    /// - `rw` `0:4` `PREDIV` PREDIV division factor
    /// - `rw` `4:9` `ADC12PRES` ADC1 and ADC2 prescaler
    pub cfgr2: RW<u32>,

    /// Clock configuration register 3
    ///
    /// - `rw` `0:2` `USART1SW` USART1 clock source selection
    /// - `rw` `4` `I2C1SW` I2C1 clock source selection
    /// - `rw` `8` `TIM1SW` Timer1 clock source selection
    /// - `rw` `12` `HRTIM1SW` HRTIM1 clock selection
    pub cfgr3: RW<u32>,
}
