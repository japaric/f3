#!/bin/bash

set -e

main() {
    local svd=STM32F30x.svd

    svd2rust -i $svd tim6 > src/peripheral/btim.rs
    sed -i 's/\(pub struct \)Tim6/\1BTim/' src/peripheral/btim.rs

    svd2rust -i $svd dbgmcu > src/peripheral/dbgmcu.rs

    svd2rust -i $svd gpioa > src/peripheral/gpio.rs
    sed -i 's/\(pub struct Gpio\)a/\1/' src/peripheral/gpio.rs

    svd2rust -i $svd tim2 > src/peripheral/gptim.rs
    sed -i 's/\(pub struct \)Tim2/\1GpTim/' src/peripheral/gptim.rs

    svd2rust -i $svd i2c1 > src/peripheral/i2c.rs
    sed -i 's/\(pub struct I2c\)1/\1/' src/peripheral/i2c.rs

    svd2rust -i $svd rcc > src/peripheral/rcc.rs

    svd2rust -i $svd spi1 > src/peripheral/spi.rs
    sed -i 's/\(pub struct Spi\)1/\1/' src/peripheral/spi.rs

    svd2rust -i $svd usart1 > src/peripheral/usart.rs
    sed -i 's/\(pub struct Usart\)1/\1/' src/peripheral/usart.rs

    set +e
    rustfmt src/peripheral/*.rs
    set -e

    xargo build --target thumbv7em-none-eabihf
}

main
