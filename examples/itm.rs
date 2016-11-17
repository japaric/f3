//! With the `iprint!` macros you can send `print!`-formatted messages to the
//! HOST
//!
//! Where the HOST is the machine, likely your laptop, that's debugging the F3
//! using OpenOCD+GDB.
//!
//! To receive the messages on the HOST you can use the `itmdump` tool to parse
//! the "instrumentation packets" that the F3 will send:
//!
//! ``` text
//! $ cargo install itm
//! ```
//!
//! `itmdump` uses a named pipe to receive the data that the F3 sends. Call
//! `itmdump` like this to create and use a named pipe in the `/tmp` directory:
//!
//! ``` text
//! $ itmdump /tmp/itm.fifo
//! ```
//!
//! The command will block, let it be. Next, configure OpenOCD to send the F3
//! messages to the named pipe that you just created. You can do this from your
//! current GDB session:
//!
//! ``` text
//! (gdb) monitor tpiu config internal /tmp/itm.fifo uart off 8000000
//! ```
//!
//! (The above command assumes your F3 is running at 8MHz, which is the default
//! clock frequency)
//!
//! After that, the output of every `iprint!` call will appear in `itmdump`
//! stdout! Note that if `itmdump` is connected to a terminal, its output will
//! be line buffered so won't see anything printed on screen until a newline
//! (`\n`) is reached. The example below, produces the following output.
//!
//! ```
//! $ itmdump /tmp/itm.fifo
//! Hello, world!
//! ```
//!
//! # Notes
//!
//! You'll have to enter that `monitor` command on each GDB session so it's a
//! good idea to add it to your `.gdbinit` so it gets done automatically when
//! GDB starts.
//!
//! OTOH, You *don't* have to spawn a new `itmdump` command  for each debug
//! session. The same process will continue to work for all subsequent debug
//! sessions as long as you don't exit it (e.g. with Ctrl+C).

#![no_main]
#![no_std]

#[macro_use]
extern crate f3;

#[no_mangle]
pub fn main() -> ! {
    iprintln!("Hello, world!");

    loop {}
}
