//! Echo server over Serial

#![no_main]
#![no_std]

extern crate f3;
extern crate futuro;

use futuro::prelude::*;
use f3::serial::{Serial, Tx, Write};

// XXX can we get rid of this explicit state using Future adapters?
enum TxState {
    Free(Tx),
    Pending(Write),
}

#[no_mangle]
pub fn main() -> ! {
    let Serial { tx, mut rx } = Serial::new().unwrap();

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
    }
}
