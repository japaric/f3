//! Universal Synchronous Asynchronous Receiver Transmitter

use core::cell::UnsafeCell;
use core::ptr;

use volatile_register::{RO, RW};

#[repr(C)]
pub struct Registers {
    /// Control register 1
    ///
    /// - `rw` `27` `EOBIE` End of Block interrupt enable
    /// - `rw` `26` `RTOIE` Receiver timeout interrupt enable
    /// - `rw` `21:26` `DEAT` Driver Enable assertion time
    /// - `rw` `16:21` `DEDT` Driver Enable deassertion time
    /// - `rw` `15` `OVER8` Oversampling mode
    /// - `rw` `14` `CMIE` Character match interrupt enable
    /// - `rw` `13` `MME` Mute mode enable
    /// - `rw` `12` `M` Word length
    /// - `rw` `11` `WAKE` Receiver wakeup method
    /// - `rw` `10` `PCE` Parity control enable
    /// - `rw` `9` `PS` Parity selection
    /// - `rw` `8` `PEIE` PE interrupt enable
    /// - `rw` `7` `TXEIE` interrupt enable
    /// - `rw` `6` `TCIE` Transmission complete interrupt enable
    /// - `rw` `5` `RXNEIE` RXNE interrupt enable
    /// - `rw` `4` `IDLEIE` IDLE interrupt enable
    /// - `rw` `3` `TE` Transmitter enable
    /// - `rw` `2` `RE` Receiver enable
    /// - `rw` `1` `UESM` USART enable in Stop mode
    /// - `rw` `0` `UE` USART enable
    pub cr1: RW<u32>,

    /// Control register 2
    ///
    /// - `rw` `28:32` `ADD4` Address of the USART node
    /// - `rw` `24:28` `ADD0` Address of the USART node
    /// - `rw` `23` `RTOEN` Receiver timeout enable
    /// - `rw` `21:23` `ABRMOD` Auto baud rate mode
    /// - `rw` `20` `ABREN` Auto baud rate enable
    /// - `rw` `19` `MSBFIRST` Most significant bit first
    /// - `rw` `18` `DATAINV` Binary data inversion
    /// - `rw` `17` `TXINV` TX pin active level inversion
    /// - `rw` `16` `RXINV` RX pin active level inversion
    /// - `rw` `15` `SWAP` Swap TX/RX pins
    /// - `rw` `14` `LINEN` LIN mode enable
    /// - `rw` `12:14` `STOP` STOP bits
    /// - `rw` `11` `CLKEN` Clock enable
    /// - `rw` `10` `CPOL` Clock polarity
    /// - `rw` `9` `CPHA` Clock phase
    /// - `rw` `8` `LBCL` Last bit clock pulse
    /// - `rw` `6` `LBDIE` LIN break detection interrupt enable
    /// - `rw` `5` `LBDL` LIN break detection length
    /// - `rw` `4` `ADDM7` 7-bit Address Detection/4-bit Address Detection
    pub cr2: RW<u32>,

    /// Control register 3
    ///
    /// - `rw` `22` `WUFIE` Wakeup from Stop mode interrupt enable
    /// - `rw` `20:22` `WUS` Wakeup from Stop mode interrupt flag selection
    /// - `rw` `17:20` `SCARCNT` Smartcard auto-retry count
    /// - `rw` `15` `DEP` Driver enable polarity selection
    /// - `rw` `14` `DEM` Driver enable mode
    /// - `rw` `13` `DDRE` DMA Disable on Reception Error
    /// - `rw` `12` `OVRDIS` Overrun Disable
    /// - `rw` `11` `ONEBIT` One sample bit method enable
    /// - `rw` `10` `CTSIE` CTS interrupt enable
    /// - `rw` `9` `CTSE` CTS enable
    /// - `rw` `8` `RTSE` RTS enable
    /// - `rw` `7` `DMAT` DMA enable transmitter
    /// - `rw` `6` `DMAR` DMA enable receiver
    /// - `rw` `5` `SCEN` Smartcard mode enable
    /// - `rw` `4` `NACK` Smartcard NACK enable
    /// - `rw` `3` `HDSEL` Half-duplex selection
    /// - `rw` `2` `IRLP` IrDA low-power
    /// - `rw` `1` `IREN` IrDA mode enable
    /// - `rw` `0` `EIE` Error interrupt enable
    pub cr3: RW<u32>,

    /// Baud rate register
    ///
    /// - `rw` `4:16` `DIV_Mantissa` mantissa of USARTDIV
    /// - `rw` `0:4` `DIV_Fraction` fraction of USARTDIV
    pub brr: RW<u16>,

    reserved0: u16,

    /// Guard time and prescaler register
    ///
    /// - `rw` `8:16` `GT` Guard time value
    /// - `rw` `0:8` `PSC` Prescaler value
    pub gtpr: RW<u16>,

    /// Receiver timeout register
    ///
    /// - `rw` `24:32` `BLEN` Block Length
    /// - `rw` `0:24` `RTO` Receiver timeout value
    pub rtor: RW<u32>,

    /// Request register
    ///
    /// - `rw` `4` `TXFRQ` Transmit data flush request
    /// - `rw` `3` `RXFRQ` Receive data flush request
    /// - `rw` `2` `MMRQ` Mute mode request
    /// - `rw` `1` `SBKRQ` Send break request
    /// - `rw` `0` `ABRRQ` Auto baud rate request
    pub rqr: RW<u8>,

    reserved1: [u8; 3],

    /// Interrupt & status register
    ///
    /// - `rw` `22` `REACK` Receive enable acknowledge flag
    /// - `rw` `21` `TEACK` Transmit enable acknowledge flag
    /// - `rw` `20` `WUF` Wakeup from Stop mode flag
    /// - `rw` `19` `RWU` Receiver wakeup from Mute mode
    /// - `rw` `18` `SBKF` Send break flag
    /// - `rw` `17` `CMF` character match flag
    /// - `rw` `16` `BUSY` Busy flag
    /// - `rw` `15` `ABRF` Auto baud rate flag
    /// - `rw` `14` `ABRE` Auto baud rate error
    /// - `rw` `12` `EOBF` End of block flag
    /// - `rw` `11` `RTOF` Receiver timeout
    /// - `rw` `10` `CTS` CTS flag
    /// - `rw` `9` `CTSIF` CTS interrupt flag
    /// - `rw` `8` `LBDF` LIN break detection flag
    /// - `rw` `7` `TXE` Transmit data register empty
    /// - `rw` `6` `TC` Transmission complete
    /// - `rw` `5` `RXNE` Read data register not empty
    /// - `rw` `4` `IDLE` Idle line detected
    /// - `rw` `3` `ORE` Overrun error
    /// - `rw` `2` `NF` Noise detected flag
    /// - `rw` `1` `FE` Framing error
    /// - `rw` `0` `PE` Parity error
    pub isr: RO<u32>,

    /// Interrupt flag clear register
    ///
    /// - `rw` `20` `WUCF` Wakeup from Stop mode clear flag
    /// - `rw` `17` `CMCF` Character match clear flag
    /// - `rw` `12` `EOBCF` End of timeout clear flag
    /// - `rw` `11` `RTOCF` Receiver timeout clear flag
    /// - `rw` `9` `CTSCF` CTS clear flag
    /// - `rw` `8` `LBDCF` LIN break detection clear flag
    /// - `rw` `6` `TCCF` Transmission complete clear flag
    /// - `rw` `4` `IDLECF` Idle line detected clear flag
    /// - `rw` `3` `ORECF` Overrun error clear flag
    /// - `rw` `2` `NCF` Noise detected clear flag
    /// - `rw` `1` `FECF` Framing error clear flag
    /// - `rw` `0` `PECF` Parity error clear flag
    pub icr: RW<u32>,

    /// Receive data register
    ///
    /// - `rw` `0:9` `RDR` Receive data value
    pub rdr: Rdr,

    reserved3: u16,

    /// Transmit data register
    ///
    /// - `rw` `0:9` `TDR` Transmit data value
    pub tdr: Tdr,
}

#[repr(C)]
pub struct Rdr {
    half_word: u16,
}

impl Rdr {
    pub fn read_u8(&self) -> u8 {
        unsafe {
            ptr::read_volatile(&self.half_word as *const u16 as *const u8)
        }
    }

    pub fn read_u9(&self) -> u16 {
        unsafe { ptr::read_volatile(&self.half_word) }
    }
}

#[repr(C)]
pub struct Tdr {
    half_word: UnsafeCell<u16>,
}

impl Tdr {
    pub fn write_u8(&self, payload: u8) {
        unsafe { ptr::write_volatile(self.half_word.get() as *mut u8, payload) }
    }

    pub fn write_u9(&self, payload: u16) {
        unsafe { ptr::write_volatile(self.half_word.get(), payload) }
    }
}
