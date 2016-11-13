//! Interrupts
//!
//! **WARNING** Here be dragons. Interrupts are key for writing asynchronous
//! programs but they also open the door to data races. Tread with care and mind
//! your `unsafe`s.
//!
//! All the interrupts prefixed with an underscore (`_`) can be overridden by
//! the top crate.

use cortex_m::Handler;

/// List of all the interrupts as allocated in the vector table.
///
/// `None` indicates that the spot is RESERVED.
#[doc(hidden)]
#[export_name = "_INTERRUPTS"]
pub static INTERRUPTS: [Option<Handler>; 85] = [Some(_wwdg),
                                                Some(_pvd),
                                                Some(_tamper_stamp),
                                                Some(_rtc_wkup),
                                                Some(_flash),
                                                Some(_rcc),
                                                Some(_exti0),
                                                Some(_exti1),
                                                Some(_exti2_ts),
                                                Some(_exti3),
                                                Some(_exti4),
                                                Some(_dma1_channel1),
                                                Some(_dma1_channel2),
                                                Some(_dma1_channel3),
                                                Some(_dma1_channel4),
                                                Some(_dma1_channel5),
                                                Some(_dma1_channel6),
                                                Some(_dma1_channel7),
                                                Some(_adc1_2),
                                                Some(_usb_hp_can_tx),
                                                Some(_usb_lp_can_rx0),
                                                Some(_can_rx1),
                                                Some(_can_sce),
                                                Some(_exti9_5),
                                                Some(_tim1_brk_tim15),
                                                Some(_tim1_up_tim16),
                                                Some(_tim1_trg_com_tim17),
                                                Some(_tim1_cc),
                                                Some(_tim2),
                                                Some(_tim3),
                                                Some(_tim4),
                                                Some(_i2c1_ev),
                                                Some(_i2c1_er),
                                                Some(_i2c2_ev),
                                                Some(_i2c2_er),
                                                Some(_spi1),
                                                Some(_spi2),
                                                Some(_usart1),
                                                Some(_usart2),
                                                Some(_usart3),
                                                Some(_exti15_10),
                                                Some(_rtc_alarm),
                                                Some(_usb_wake_up),
                                                Some(_tim8_brk),
                                                Some(_tim8_up),
                                                Some(_tim8_trg_com),
                                                Some(_tim8_cc),
                                                Some(_adc3),
                                                Some(_fmc),
                                                None,
                                                None,
                                                Some(_spi3),
                                                Some(_uart4),
                                                Some(_uart5),
                                                Some(_tim6_dac),
                                                Some(_tim7),
                                                Some(_dma2_channel1),
                                                Some(_dma2_channel2),
                                                Some(_dma2_channel3),
                                                Some(_dma2_channel4),
                                                Some(_dma2_channel5),
                                                Some(_adc4),
                                                None,
                                                None,
                                                Some(_comp1_2_3),
                                                Some(_comp4_5_6),
                                                Some(_comp7),
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                Some(_i2c3_ev),
                                                Some(_i2c3_er),
                                                Some(_usb_hp),
                                                Some(_usb_lp),
                                                Some(_usb_wake_up_rmp),
                                                Some(_tim20_brk),
                                                Some(_tim20_up),
                                                Some(_tim20_trg_com),
                                                Some(_tim20_cc),
                                                Some(_fpu),
                                                None,
                                                None,
                                                Some(_spi4)];

extern "C" {
    /// Window Watchdog interrupt
    pub fn _wwdg();
    /// PVD through EXIT Line16 detection interrupt
    pub fn _pvd();
    /// Tamper and TimeStamp interrupts through EXTI Line19
    pub fn _tamper_stamp();
    /// RTC wakeup timer interrupt through EXTI Line20
    pub fn _rtc_wkup();
    /// Flash global interrupt
    pub fn _flash();
    /// RCC global interrupt
    pub fn _rcc();
    /// EXTI Line0 interrupt
    pub fn _exti0();
    /// EXTI Line1 interrupt
    pub fn _exti1();
    /// EXTI Line2 and Touch sensing interrupts
    pub fn _exti2_ts();
    /// EXTI Line3
    pub fn _exti3();
    /// EXTI Line4
    pub fn _exti4();
    /// DMA1 channel 1 interrupt
    pub fn _dma1_channel1();
    /// DMA1 channel 2 interrupt
    pub fn _dma1_channel2();
    /// DMA1 channel 3 interrupt
    pub fn _dma1_channel3();
    /// DMA1 channel 4 interrupt
    pub fn _dma1_channel4();
    /// DMA1 channel 5 interrupt
    pub fn _dma1_channel5();
    /// DMA1 channel 6 interrupt
    pub fn _dma1_channel6();
    /// DMA1 channel 7 interrupt
    pub fn _dma1_channel7();
    /// ADC1 and ADC2 global interrupt
    pub fn _adc1_2();
    /// USB High priority/CAN_TX interrupts
    pub fn _usb_hp_can_tx();
    /// USB Low priority/CAN_RX0 interrupts
    pub fn _usb_lp_can_rx0();
    /// CAN_RX1 interrupt
    pub fn _can_rx1();
    /// CAN_SCE interrupt
    pub fn _can_sce();
    /// EXTI Line[9:5] interrupts
    pub fn _exti9_5();
    /// TIM1 Break/TIM15 global interrupts
    pub fn _tim1_brk_tim15();
    /// TIM1 Update/TIM16 global interrupts
    pub fn _tim1_up_tim16();
    /// TIM1 trigger and commutation/TIM17 interrupts
    pub fn _tim1_trg_com_tim17();
    /// TIM1 capture compare interrupt
    pub fn _tim1_cc();
    /// TIM2 global interrupt
    pub fn _tim2();
    /// TIM3 global interrupt
    pub fn _tim3();
    /// TIM4 global interrupt
    pub fn _tim4();
    /// I2C1 event interrupt & EXTI Line23 interrupt
    pub fn _i2c1_ev();
    /// I2C1 error interrupt
    pub fn _i2c1_er();
    /// I2C2 event interrupt & EXTI Line24 interrupt
    pub fn _i2c2_ev();
    /// I2C2 error interrupt
    pub fn _i2c2_er();
    /// SPI1 global interrupt
    pub fn _spi1();
    /// SPI2 global interrupt
    pub fn _spi2();
    /// USART1 global interrupt & EXTI Line 25
    pub fn _usart1();
    /// USART2 global interrupt & EXIT Line 26
    pub fn _usart2();
    /// USART3 global interrupt & EXTI Line 28
    pub fn _usart3();
    /// EXTI Line[15:10] interrupts
    pub fn _exti15_10();
    /// RTC alarm interrupt
    pub fn _rtc_alarm();
    /// USB wakeup from Suspend (EXTI line 18)
    pub fn _usb_wake_up();
    /// TIM8 break interrupt
    pub fn _tim8_brk();
    /// TIM8 update interrupt
    pub fn _tim8_up();
    /// TIM8 Trigger and communication interrupts
    pub fn _tim8_trg_com();
    /// TIM8 capture compare interrupt
    pub fn _tim8_cc();
    /// ADC3 global interrupt
    pub fn _adc3();
    /// FMC global interrupt
    pub fn _fmc();
    /// SPI3 global interrupt
    pub fn _spi3();
    /// UART4 gloal and EXTI Line 34 interrupts
    pub fn _uart4();
    /// UART5 gloal and EXTI Line 35 interrupts
    pub fn _uart5();
    /// TIM6 global and DAC1 underrun interrupts
    pub fn _tim6_dac();
    /// TIM7 global interrupt
    pub fn _tim7();
    /// DMA2 channel1 global interrupt
    pub fn _dma2_channel1();
    /// DMA2 channel2 global interrupt
    pub fn _dma2_channel2();
    /// DMA2 channel3 global interrupt
    pub fn _dma2_channel3();
    /// DMA2 channel4 global interrupt
    pub fn _dma2_channel4();
    /// DMA2 channel5 global interrupt
    pub fn _dma2_channel5();
    /// ADC4 global interrupt
    pub fn _adc4();
    /// COMP1 & COMP2 & COMP3 interrupts combined with EXTI Lines 21,22 and 29
    /// interrupts
    pub fn _comp1_2_3();
    /// COMP4 & COMP5 & COMP6 interrupts combined with EXTI Lines 30, 31 and 32
    /// interrupts
    pub fn _comp4_5_6();
    /// COMP7 interrupt combined with EXTI Line 33 interrupt
    pub fn _comp7();
    /// I2C3 Event interrupt
    pub fn _i2c3_ev();
    /// I2C3 Error interrupt
    pub fn _i2c3_er();
    /// USB High priority interrupt
    pub fn _usb_hp();
    /// USB Low priority interrupt
    pub fn _usb_lp();
    /// USB wake up from Suspend and EXTI Line 18
    pub fn _usb_wake_up_rmp();
    /// TIM20 Break interrupt
    pub fn _tim20_brk();
    /// TIM20 Upgrade interrupt
    pub fn _tim20_up();
    /// TIM20 Trigger and Commutation interrupt
    pub fn _tim20_trg_com();
    /// TIM20 Capture Control interrupt
    pub fn _tim20_cc();
    /// Floating point interrupt
    pub fn _fpu();
    /// SPI4 Global interrupt
    pub fn _spi4();
}
