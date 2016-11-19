use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Pass our linker script to the top crate
    let mut ld = String::new();

    let interrupts = env::var_os("CARGO_FEATURE_INTERRUPTS").is_some();
    let static_ram = env::var_os("CARGO_FEATURE_STATIC_RAM").is_some();

    ld.push_str("
MEMORY
{
  CCRAM : ORIGIN = 0x10000000, LENGTH = 8K
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 40K
}

ENTRY(_reset)

SECTIONS
{
  .text ORIGIN(FLASH) :
  {
    /* Vector table */
    _VECTOR_TABLE = .;
    LONG(ORIGIN(CCRAM) + LENGTH(CCRAM));
    LONG(_reset + 1);
    KEEP(*(.rodata._EXCEPTIONS));
    _eexceptions = .;");

    if interrupts {
        ld.push_str("
    KEEP(*(.rodata._INTERRUPTS));
    _einterrupts = .;");
    }

    ld.push_str("
    /* Entry point: reset handler */
    _reset = .;
    *(.text._reset);

    *(.text.*);
    *(.rodata.*);
  } > FLASH");

    if static_ram {
        ld.push_str("
  .bss : ALIGN(4)
  {
    _sbss = .;
    *(.bss.*);
    _ebss = ALIGN(4);
  } > RAM

  .data : ALIGN(4)
  {
    _sdata = .;
    *(.data.*);
    _edata = ALIGN(4);
  } > RAM AT > FLASH

  _sidata = LOADADDR(.data);
  _heap_start = .;
  _heap_end = ORIGIN(RAM) + LENGTH(RAM);
");
    }

    ld.push_str("
  /DISCARD/ :
  {
    *(.ARM.exidx.*)
    *(.note.gnu.build-id.*)");

    if !static_ram {
        ld.push_str("
    *(.bss.*)
    *(.data.*)");
    }

    ld.push_str("
  }
}

/* HACK to make these symbols overrideable by _dependencies_ (they were
   already overridable by the top crate), we declare them as undefined
   (EXTERN) here. */
EXTERN(__aeabi_memclr4);
EXTERN(_default_exception_handler);
EXTERN(_init);

/* Exceptions */
PROVIDE(_nmi = _default_exception_handler);
PROVIDE(_hard_fault = _default_exception_handler);
PROVIDE(_memmanage_fault = _default_exception_handler);
PROVIDE(_bus_fault = _default_exception_handler);
PROVIDE(_usage_fault = _default_exception_handler);
PROVIDE(_svcall = _default_exception_handler);
PROVIDE(_pendsv = _default_exception_handler);
PROVIDE(_systick = _default_exception_handler);");

    if interrupts {

        ld.push_str("
/* Interrupts */
PROVIDE(_wwdg = _default_exception_handler);
PROVIDE(_pvd = _default_exception_handler);
PROVIDE(_tamper_stamp = _default_exception_handler);
PROVIDE(_rtc_wkup = _default_exception_handler);
PROVIDE(_flash = _default_exception_handler);
PROVIDE(_rcc = _default_exception_handler);
PROVIDE(_exti0 = _default_exception_handler);
PROVIDE(_exti1 = _default_exception_handler);
PROVIDE(_exti2_ts = _default_exception_handler);
PROVIDE(_exti3 = _default_exception_handler);
PROVIDE(_exti4 = _default_exception_handler);
PROVIDE(_dma1_channel1 = _default_exception_handler);
PROVIDE(_dma1_channel2 = _default_exception_handler);
PROVIDE(_dma1_channel3 = _default_exception_handler);
PROVIDE(_dma1_channel4 = _default_exception_handler);
PROVIDE(_dma1_channel5 = _default_exception_handler);
PROVIDE(_dma1_channel6 = _default_exception_handler);
PROVIDE(_dma1_channel7 = _default_exception_handler);
PROVIDE(_adc1_2 = _default_exception_handler);
PROVIDE(_usb_hp_can_tx = _default_exception_handler);
PROVIDE(_usb_lp_can_rx0 = _default_exception_handler);
PROVIDE(_can_rx1 = _default_exception_handler);
PROVIDE(_can_sce = _default_exception_handler);
PROVIDE(_exti9_5 = _default_exception_handler);
PROVIDE(_tim1_brk_tim15 = _default_exception_handler);
PROVIDE(_tim1_up_tim16 = _default_exception_handler);
PROVIDE(_tim1_trg_com_tim17 = _default_exception_handler);
PROVIDE(_tim1_cc = _default_exception_handler);
PROVIDE(_tim2 = _default_exception_handler);
PROVIDE(_tim3 = _default_exception_handler);
PROVIDE(_tim4 = _default_exception_handler);
PROVIDE(_i2c1_ev = _default_exception_handler);
PROVIDE(_i2c1_er = _default_exception_handler);
PROVIDE(_i2c2_ev = _default_exception_handler);
PROVIDE(_i2c2_er = _default_exception_handler);
PROVIDE(_spi1 = _default_exception_handler);
PROVIDE(_spi2 = _default_exception_handler);
PROVIDE(_usart1 = _default_exception_handler);
PROVIDE(_usart2 = _default_exception_handler);
PROVIDE(_usart3 = _default_exception_handler);
PROVIDE(_exti15_10 = _default_exception_handler);
PROVIDE(_rtc_alarm = _default_exception_handler);
PROVIDE(_usb_wake_up = _default_exception_handler);
PROVIDE(_tim8_brk = _default_exception_handler);
PROVIDE(_tim8_up = _default_exception_handler);
PROVIDE(_tim8_trg_com = _default_exception_handler);
PROVIDE(_tim8_cc = _default_exception_handler);
PROVIDE(_adc3 = _default_exception_handler);
PROVIDE(_fmc = _default_exception_handler);
PROVIDE(_spi3 = _default_exception_handler);
PROVIDE(_uart4 = _default_exception_handler);
PROVIDE(_uart5 = _default_exception_handler);
PROVIDE(_tim6_dac = _default_exception_handler);
PROVIDE(_tim7 = _default_exception_handler);
PROVIDE(_dma2_channel1 = _default_exception_handler);
PROVIDE(_dma2_channel2 = _default_exception_handler);
PROVIDE(_dma2_channel3 = _default_exception_handler);
PROVIDE(_dma2_channel4 = _default_exception_handler);
PROVIDE(_dma2_channel5 = _default_exception_handler);
PROVIDE(_adc4 = _default_exception_handler);
PROVIDE(_comp1_2_3 = _default_exception_handler);
PROVIDE(_comp4_5_6 = _default_exception_handler);
PROVIDE(_comp7 = _default_exception_handler);
PROVIDE(_i2c3_ev = _default_exception_handler);
PROVIDE(_i2c3_er = _default_exception_handler);
PROVIDE(_usb_hp = _default_exception_handler);
PROVIDE(_usb_lp = _default_exception_handler);
PROVIDE(_usb_wake_up_rmp = _default_exception_handler);
PROVIDE(_tim20_brk = _default_exception_handler);
PROVIDE(_tim20_up = _default_exception_handler);
PROVIDE(_tim20_trg_com = _default_exception_handler);
PROVIDE(_tim20_cc = _default_exception_handler);
PROVIDE(_fpu = _default_exception_handler);
PROVIDE(_spi4 = _default_exception_handler);");
    }

    ld.push_str("
ASSERT(_eexceptions - ORIGIN(FLASH) == 0x40, \"exceptions \
                 not linked where expected\");");

    if interrupts {
        ld.push_str("
ASSERT(_einterrupts - ORIGIN(FLASH) == 0x194, \
                     \"interrupts not linked where expected\");");
    }

    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("stm32f3discovery.ld"))
        .unwrap()
        .write_all(ld.as_bytes())
        .unwrap();

    println!("cargo:rustc-link-search={}", out.display());
}
