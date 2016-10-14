#!/bin/bash

set -e

main() {
    svd2rust -i STM32F30x.svd dbgmcu > src/peripheral/dbgmcu.rs
    svd2rust -i STM32F30x.svd gpioa > src/peripheral/gpio.rs
    sed -i 's/\(pub struct Gpio\)a/\1/' src/peripheral/gpio.rs
    svd2rust -i STM32F30x.svd i2c1 > src/peripheral/i2c.rs
    sed -i 's/\(pub struct I2c\)1/\1/' src/peripheral/i2c.rs
    svd2rust -i STM32F30x.svd rcc > src/peripheral/rcc.rs
    svd2rust -i STM32F30x.svd spi1 > src/peripheral/spi.rs
    sed -i 's/\(pub struct Spi\)1/\1/' src/peripheral/spi.rs
    svd2rust -i STM32F30x.svd tim6 > src/peripheral/tim.rs
    sed -i 's/\(pub struct Tim\)6/\1/' src/peripheral/tim.rs
    svd2rust -i STM32F30x.svd usart1 > src/peripheral/usart.rs
    sed -i 's/\(pub struct Usart\)1/\1/' src/peripheral/usart.rs

    set +e
    rustfmt src/peripheral/*.rs
    set -e

    xargo build --target thumbv7em-none-eabi
}

main

