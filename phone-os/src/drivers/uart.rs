//! PL011 UART Driver for AArch64 QEMU 'virt' machine and real hardware
//! 
//! This driver supports configurable base addresses for different hardware platforms.

use core::ptr::{read_volatile, write_volatile};

// Default base address for PL011 UART on QEMU virt machine
const UART_BASE_DEFAULT: usize = 0x9000000;

// Register offsets (PL011 standard)
const UART_DR: usize = 0x00;   // Data Register
const UART_FR: usize = 0x18;   // Flag Register
const UART_IBRD: usize = 0x24; // Integer Baud Rate Divisor
const UART_FBRD: usize = 0x28; // Fractional Baud Rate Divisor
const UART_LCRH: usize = 0x2C; // Line Control Register
const UART_CR: usize = 0x30;   // Control Register
const UART_IMSC: usize = 0x38; // Interrupt Mask Set/Clear

// Flag register bits
const UART_FR_TXFF: u32 = 0x20; // Transmit FIFO full
const UART_FR_BUSY: u32 = 0x08; // UART busy

// Line control bits
const UART_LCRH_WLEN_8: u32 = 0x60; // 8-bit word length
const UART_LCRH_FEN: u32 = 0x10;    // FIFO enable

// Control register bits
const UART_CR_UARTEN: u32 = 0x01;   // UART enable
const UART_CR_TXE: u32 = 0x100;     // Transmit enable
const UART_CR_RXE: u32 = 0x200;     // Receive enable

pub struct Uart {
    base_address: usize,
    initialized: bool,
}

impl Uart {
    /// Create a new UART with default base address
    pub const fn new() -> Self {
        Uart {
            base_address: UART_BASE_DEFAULT,
            initialized: false,
        }
    }
    
    /// Create a new UART with custom base address
    pub const fn with_base(base: usize) -> Self {
        Uart {
            base_address: base,
            initialized: false,
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

    /// Initialize UART with default settings (115200 8N1)
    /// Assumes 48MHz clock - adjust for your hardware
    pub fn init(&mut self) {
        self.init_with_baud(115200, 48_000_000);
    }
    
    /// Initialize UART with specific baud rate and clock frequency
    pub fn init_with_baud(&mut self, baud: u32, clock_hz: u32) {
        // Disable UART before reconfiguration
        self.write_reg(UART_CR, 0);
        
        // Calculate baud rate divisors
        // baud = clock / (16 * (IBRD + FBRD/64))
        let divisor = clock_hz / (16 * baud);
        let ibrd = divisor;
        let fbrd = ((divisor % 1) * 64) as u32;
        
        // Set baud rate registers
        self.write_reg(UART_IBRD, ibrd);
        self.write_reg(UART_FBRD, fbrd);
        
        // Set line control: 8 bits, FIFO enabled
        self.write_reg(UART_LCRH, UART_LCRH_WLEN_8 | UART_LCRH_FEN);
        
        // Clear interrupts
        self.write_reg(UART_IMSC, 0);
        
        // Enable UART, TX, and RX
        self.write_reg(UART_CR, UART_CR_UARTEN | UART_CR_TXE | UART_CR_RXE);
        
        self.initialized = true;
    }

    /// Check if UART transmitter is ready
    fn is_tx_ready(&self) -> bool {
        (self.read_reg(UART_FR) & UART_FR_TXFF) == 0
    }

    pub fn write_byte(&self, byte: u8) {
        // Wait until Transmit FIFO is not full
        while !self.is_tx_ready() {}
        
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
    
    /// Get current base address
    pub fn base_address(&self) -> usize {
        self.base_address
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
static mut UART: Uart = Uart::new();

/// Initialize UART with default settings
pub fn init() {
    unsafe {
        (*(&raw mut UART)).init();
        (*(&raw mut UART)).write_str("[UART] Initialized\r\n");
    }
}

/// Initialize UART with custom base address (for real hardware)
pub fn init_with_base(base_addr: u64) {
    unsafe {
        // Create new UART instance with specified base
        let mut new_uart = Uart::with_base(base_addr as usize);
        new_uart.init();
        UART = new_uart;
        (*(&raw mut UART)).write_str("[UART] Initialized with custom base\r\n");
    }
}

/// Initialize UART with custom baud rate and clock
pub fn init_with_baud(baud: u32, clock_hz: u32) {
    unsafe {
        (*(&raw mut UART)).init_with_baud(baud, clock_hz);
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

/// Write a formatted string to UART
pub fn write_fmt(args: core::fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        let _ = (*(&raw mut UART)).write_fmt(args);
    }
}
