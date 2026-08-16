//! Pine64 PinePhone configuration
//! 
//! Hardware: Allwinner A64 SoC
//! - Quad-core Cortex-A53
//! - Mali-400MP2 GPU
//! - 1GB/2GB LPDDR3 RAM
//! - eMMC / microSD boot
//! - LCD: 720x1280 IPS

use super::DeviceConfig;

pub struct PinePhoneConfig;

impl DeviceConfig for PinePhoneConfig {
    const NAME: &'static str = "Pine64 PinePhone";
    const SOC: &'static str = "Allwinner A64";
    
    // Boot addresses (U-Boot typically loads kernel here)
    const KERNEL_LOAD_ADDR: u64 = 0x41000000;
    const DTB_LOAD_ADDR: u64 = 0x41800000;
    
    // UART0 for early console (PL011 compatible)
    const UART_BASE: u64 = 0x01c28000;
    const UART_CLOCK: u32 = 24_000_000; // 24 MHz
    
    // GICv2 addresses
    const GICD_BASE: u64 = 0x01c81000;
    const GICC_BASE: u64 = 0x01c82000;
    
    // Framebuffer (set by bootloader, typical address)
    const FRAMEBUFFER_ADDR: Option<u64> = Some(0x5A000000);
    const SCREEN_WIDTH: usize = 720;
    const SCREEN_HEIGHT: usize = 1280;
    
    // Memory map
    const RAM_BASE: u64 = 0x40000000;
    const RAM_SIZE: u64 = 0x80000000; // 2GB max
    
    // Key MMIO regions
    const MMIO_REGIONS: &'static [(u64, u64)] = &[
        // UART0
        (0x01c28000, 0x400),
        // PRCM (Power/Reset/Clock Management)
        (0x01f01400, 0x400),
        // CCU (Clock Control Unit)
        (0x01c20000, 0x400),
        // PIO (GPIO controller)
        (0x01c20800, 0x400),
        // GIC
        (0x01c81000, 0x2000),
        // Timer
        (0x01c20c00, 0x400),
        // SD/MMC controllers
        (0x01c0f000, 0x400),
        (0x01c10000, 0x400),
        // USB OTG
        (0x01c19000, 0x400),
        // Display engine
        (0x01e60000, 0x10000),
        // HDMI
        (0x01ee0000, 0x1000),
    ];
}

/// PinePhone-specific initialization
pub unsafe fn init_pinephone() {
    use core::ptr;
    
    crate::drivers::uart::println("[PinePhone] Initializing Allwinner A64...");
    
    // Configure UART0 pins (PB0/TX, PB1/RX)
    let pio_base = 0x01c20800 as *mut u32;
    
    // Set PB0 and PB1 to UART0 function (mode 2)
    // GPB_CFG0 register at offset 0x24
    let gpb_cfg0 = pio_base.add(0x24 / 4);
    ptr::write_volatile(gpb_cfg0, 0x00000022);
    
    // Enable UART0 clock via CCU
    let ccu_base = 0x01c20000 as *mut u32;
    let uart_clk_reg = ccu_base.add(0x068 / 4); // UART_CLK_REG
    ptr::write_volatile(uart_clk_reg, 0x80000000); // Enable gate
    
    // Configure UART0: 115200 8N1
    let uart_base = Self::UART_BASE as *mut u32;
    
    // Disable FIFO and clear FIFOs
    ptr::write_volatile(uart_base.add(0x08 / 4), 0x07); // FCR
    
    // Set divisor for 115200 baud (24MHz / (16 * 115200) = 13)
    let divisor = 13;
    ptr::write_volatile(uart_base.add(0x04 / 4), divisor & 0xFF); // DLL
    
    // Line control: 8 bits, no parity, 1 stop bit
    ptr::write_volatile(uart_base.add(0x0C / 4), 0x03); // LCR
    
    crate::drivers::uart::println("[PinePhone] UART initialized");
}
