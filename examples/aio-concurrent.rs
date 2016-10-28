//! Echo server and blinking LED running concurrently

#![no_main]
#![no_std]

extern crate f3;
extern crate futuro;

use f3::led::LEDS;
use f3::serial::{Serial, Tx, Write};
use f3::timer::Timer;
use futuro::prelude::*;

// XXX can we get rid of this explicit state using Future adapters?
enum TxState {
    Free(Tx),
    Pending(Write),
}

#[no_mangle]
pub fn main() -> ! {
    let Serial { tx, mut rx } = Serial::new().unwrap();

    let mut timer = Timer::new().unwrap();
    let mut on = true;
    let mut blink = timer.periodic(100).map(|_| {
        if on {
            LEDS[0].on();
        } else {
            LEDS[0].off();
        }

        on = !on;
    });

    let mut tx_state = TxState::Free(tx);
    let mut bytes = rx.bytes();
    loop {
        if let Async::Ready(byte) = bytes.poll() {
            let tx = match tx_state {
                TxState::Pending(write) => write.wait(),
                TxState::Free(tx) => tx,
            };
            tx_state = TxState::Pending(tx.write(byte));
        }

        if let TxState::Pending(mut write) = tx_state {
            if let Async::Ready(tx) = write.poll() {
                tx_state = TxState::Free(tx);
            } else {
                tx_state = TxState::Pending(write);
            }
        }

        blink.poll();
    }
}
