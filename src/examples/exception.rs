// Auto-generated. Do not modify this file! Instead modify examples/exception.rs
//! Exceptions handlers are smart! They help you find the source of crashes
//!
//! At the end there's a program that crashes. Run it with `(gdb) continue`:
//!
//! By the time you hit the exception, you'll see the following output in
//! `itmdump`:
//!
//! ``` text
//! $ itmdump /tmp/itm.fifo
//! (..)
//! EXCEPTION HardFault @ PC=0x080000a2
//! ```
//!
//! which indicates that this is a "hard fault" exception. It also tells you
//! were the crash originated! The PC (Program Counter) value is the address of
//! the instruction that generated the crash. You can disassemble your program
//! around that address using GDB:
//!
//! ``` text
//! (gdb) disas /m 0x080000a2
//! Dump of assembler code for function exception::main:
//! 32      pub fn main() -> ! {
//!    0x08000092 <+0>:     sub     sp, #8
//!    0x08000094 <+2>:     b.n     0x8000096 <exception::main+4>
//!
//! 33          // This reads beyond the boundary of available RAM (40KiB starting at
//! 34          // 0x4000_0000) and triggers a hard fault exception
//! 35          let _hard_fault_exception = unsafe {
//! 36              *((0x4000_0000 + 40 * 1024) as *const u32)
//!    0x08000096 <+4>:     b.n     0x8000098 <exception::main+6>
//!    0x08000098 <+6>:     b.n     0x800009a <exception::main+8>
//!    0x0800009a <+8>:     movw    r0, #40960      ; 0xa000
//!    0x0800009e <+12>:    movt    r0, #16384      ; 0x4000
//!    0x080000a2 <+16>:    ldr     r0, [r0, #0]    ; <--
//!    0x080000a4 <+18>:    str     r0, [sp, #4]
//!
//! 37          };
//! 38
//! 39          loop {}
//!    0x080000a6 <+20>:    b.n     0x80000a8 <exception::main+22>
//!    0x080000a8 <+22>:    b.n     0x80000a8 <exception::main+22>
//!
//! End of assembler dump.
//! ```
//!
//! `ldr r0, [r0, #0]` is the faulty instruction. It tries to load the word at
//! the address that `r0` indicates. From the two previous instructions, you can
//! tell that `r0` holds the value `0x4000a000`.
//!
//! Wait! That's not everything. There's also a local `sf` variable that points
//! to the stack frame where the exception occurred. If you print it under GDB,
//! you'll see the state of several registers at the moment of the crash:
//!
//! ``` text
//! (gdb) p/x *sf
//! $1 = {r0 = 0x4000a000, r1 = 0x48001000, r2 = 0x55550000, r3 = 0x48001000,
//!   r12 = 0x8003950, lr = 0x8000085, pc = 0x80000a2, xpsr = 0x1000000}
//! ```
//!
//! See? `r0 = 0x4000a000` that matches our expectations from reading the
//! disassembly.
//!
//! ``` rust,no_run
//! #![no_main]
//! #![no_std]
//!
//! extern crate f3;
//!
//! #[no_mangle]
//! pub fn main() -> ! {
//!     // This reads beyond the boundary of available RAM (40KiB starting at
//!     // 0x4000_0000) and triggers an exception
//!     let _hard_fault_exception =
//!         unsafe { *((0x4000_0000 + 40 * 1024) as *const u32) };
//!
//!     loop {}
//! }
//! ```
