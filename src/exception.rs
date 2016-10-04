//! Exceptions
//!
//! All the exceptions prefixed with an underscore (`_`) can overridden by the top crate:
//!
//! ```
//! //! Override the hard fault exception
//!
//! #![feature(asm)]
//! #![no_main]
//! #![no_std]
//!
//! #[macro_use]
//! extern crate cortex_m;
//! extern crate f3;
//!
//! use core::ptr;
//!
//! #[export_name = "main"]
//! pub fn main() -> ! {
//!     unsafe {
//!         // Trigger a `hard_fault` exception (using UNSAFE code, of course!) and ...
//!         ptr::write_volatile(0x0000_0000 as *mut u32, 0xdead_beef);
//!     }
//!
//!     loop {}
//! }
//!
//! #[export_name = "_hard_fault"]  // <-- Important! Note the underscore.
//! pub extern "C" fn my_hard_fault_handler() {
//!     unsafe {
//!         // .. you should reach THIS breakpoint!
//!         bkpt!();
//!     }
//!
//!     loop {}
//! }
//! ```

use cortex_m::{self, Handler, StackFrame};
use r0;

/// Kind of exception
#[derive(Debug)]
pub enum Exception {
    /// i.e. currently not servicing an exception
    ThreadMode,
    /// Non-maskable interrupt.
    Nmi,
    /// All class of fault.
    HardFault,
    /// Memory management.
    MemoryManagementFault,
    /// Pre-fetch fault, memory access fault.
    BusFault,
    /// Undefined instruction or illegal state.
    UsageFault,
    /// System service call via SWI instruction
    SVCall,
    /// Pendable request for system service
    PendSV,
    /// System tick timer
    Systick,
    /// An interrupt
    Interrupt(u8),
    // Unreachable variant
    #[doc(hidden)]
    Reserved,
}

impl Exception {
    /// Returns the exception that's currently being serviced
    pub fn current() -> Exception {
        match cortex_m::peripheral::scb().icsr.read() as u8 {
            0 => Exception::ThreadMode,
            2 => Exception::Nmi,
            3 => Exception::HardFault,
            4 => Exception::MemoryManagementFault,
            5 => Exception::BusFault,
            6 => Exception::UsageFault,
            11 => Exception::SVCall,
            14 => Exception::PendSV,
            15 => Exception::Systick,
            n if n >= 16 => Exception::Interrupt(n - 16),
            _ => Exception::Reserved,

        }
    }
}

#[cfg(target_arch = "arm")]
#[doc(hidden)]
#[export_name = "_default_exception_handler"]
#[linkage = "weak"]
#[naked]
pub unsafe extern "C" fn _default_handler() {
    use core::intrinsics;

    // NOTE need asm!, #[naked] and unreachable() to avoid modifying the stack pointer (MSP)
    asm!("mrs r0, MSP
          ldr r1, [r0, #20]
          b _default_exception_handler_impl" :::: "volatile");
    intrinsics::unreachable();
}

#[cfg(not(target_arch = "arm"))]
pub extern "C" fn _default_handler() {}

#[doc(hidden)]
#[export_name = "_default_exception_handler_impl"]
pub unsafe extern "C" fn default_handler(sf: &StackFrame) -> ! {
    iprintln!("EXCEPTION {:?} @ PC=0x{:08x}", Exception::current(), sf.pc);

    bkpt!();

    loop {}
}

/// List of all the exceptions minus the reset handler as allocated in the vector table.
///
/// `None` indicates that the spot is RESERVED.
#[link_section = ".text.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [Option<Handler>; 14] = [Some(_nmi),
                                                Some(_hard_fault),
                                                Some(_memmanage_fault),
                                                Some(_bus_fault),
                                                Some(_usage_fault),
                                                None,
                                                None,
                                                None,
                                                None,
                                                Some(_svcall),
                                                None,
                                                None,
                                                Some(_pendsv),
                                                Some(_systick)];

extern "C" {
    /// Non-maskable interrupt.
    pub fn _nmi();
    /// All class of fault.
    pub fn _hard_fault();
    /// Memory management.
    pub fn _memmanage_fault();
    /// Pre-fetch fault, memory access fault.
    pub fn _bus_fault();
    /// Undefined instruction or illegal state.
    pub fn _usage_fault();
    /// System service call via SWI instruction
    pub fn _svcall();
    /// Pendable request for system service
    pub fn _pendsv();
    /// System tick timer
    pub fn _systick();
}

/// Reset handler
#[export_name = "_reset"]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        static _ebss: u32;
        static _edata: u32;
        static _sidata: u32;
        static mut _sbss: u32;
        static mut _sdata: u32;

    }

    r0::zero_bss(&mut _sbss, &_ebss);
    r0::init_data(&mut _sdata, &_edata, &_sidata);

    ::init();

    ::main();

    // safety net in case `main` returns
    panic!("returned from main!")
}
