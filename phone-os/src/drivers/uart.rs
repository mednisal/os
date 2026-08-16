//! PL011 UART Driver for AArch64 QEMU 'virt' machine

use core::ptr::{read_volatile, write_volatile};

// Base address for PL011 UART on QEMU virt machine
const UART_BASE: usize = 0x9000000;

// Register offsets
const UART_DR: usize = 0x00; // Data Register
const UART_FR: usize = 0x18; // Flag Register

// Flag register bits
const UART_FR_TXFF: u32 = 0x20; // Transmit FIFO full

#[repr(usize)]
enum Register {
    DR = 0x00,
    FR = 0x18,
}

pub struct Uart {
    base_address: usize,
}

impl Uart {
    pub const fn new() -> Self {
        Uart {
            base_address: UART_BASE,
        }
    }

    fn write_reg(&self, offset: usize, value: u32) {
        unsafe {
            write_volatile((self.base_address + offset) as *mut u32, value);
        }
    }

    fn read_reg(&self, offset: usize) -> u32 {
        unsafe { read_volatile((self.base_address + offset) as *const u32) }
    }

    pub fn init(&self) {
        // UART is usually initialized by the bootloader (UEFI/u-boot) on QEMU virt.
        // We just ensure it's ready to write.
        // If running on bare metal without bootloader, we would need to set baud rate here.
    }

    pub fn write_byte(&self, byte: u8) {
        // Wait until Transmit FIFO is not full
        while (self.read_reg(UART_FR) & UART_FR_TXFF) != 0 {}
        
        // Write the byte
        self.write_reg(UART_DR, byte as u32);
    }

    pub fn write_str(&self, s: &str) {
        for byte in s.bytes() {
            if byte == b'\n' {
                self.write_byte(b'\r');
            }
            self.write_byte(byte);
        }
    }
}

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            if byte == b'\n' {
                self.write_byte(b'\r');
            }
            self.write_byte(byte);
        }
        Ok(())
    }
}

// Global instance for easy access
pub static mut UART: Uart = Uart::new();

pub fn init() {
    unsafe {
        (*(&raw mut UART)).init();
        (*(&raw mut UART)).write_str("[UART] Initialized\r\n");
    }
}

pub fn print(s: &str) {
    unsafe {
        (*(&raw mut UART)).write_str(s);
    }
}

pub fn println(s: &str) {
    unsafe {
        let uart_ptr = &raw mut UART;
        (*uart_ptr).write_str(s);
        (*uart_ptr).write_str("\r\n");
    }
}
