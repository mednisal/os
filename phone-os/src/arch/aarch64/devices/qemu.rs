//! QEMU virt machine configuration for aarch64
//! 
//! Hardware: QEMU virt platform with ARM Cortex-A53/A72
//! - Generic ARMv8 virtual hardware
//! - PL011 UART
//! - GICv2 or GICv3 interrupt controller
//! - VirtIO devices
//! - Framebuffer via virtio-gpu or simplefb

use super::DeviceConfig;

pub struct QemuVirtConfig;

impl DeviceConfig for QemuVirtConfig {
    const NAME: &'static str = "QEMU virt";
    const SOC: &'static str = "ARM Cortex-A53/A72 (virt)";
    
    // QEMU typically loads kernel at these addresses
    const KERNEL_LOAD_ADDR: u64 = 0x40000000;
    const DTB_LOAD_ADDR: u64 = 0x41000000;
    
    // PL011 UART at standard QEMU address
    const UART_BASE: u64 = 0x09000000;
    const UART_CLOCK: u32 = 24_000_000; // 24 MHz (QEMU default)
    
    // GICv2 addresses (QEMU default for virt machine)
    const GICD_BASE: u64 = 0x08000000;
    const GICC_BASE: u64 = 0x08010000;
    
    // Framebuffer (QEMU can provide via -device virtio-gpu or -vga)
    // We'll use a safe high address that QEMU can map
    const FRAMEBUFFER_ADDR: Option<u64> = Some(0x50000000);
    const SCREEN_WIDTH: usize = 1024;
    const SCREEN_HEIGHT: usize = 768;
    
    // Memory map (QEMU virt machine)
    const RAM_BASE: u64 = 0x40000000;
    const RAM_SIZE: u64 = 0x40000000; // 1GB default
    
    // Key MMIO regions for QEMU virt
    const MMIO_REGIONS: &'static [(u64, u64)] = &[
        // GICv2/GICv3
        (0x08000000, 0x00020000),
        // UART0 (PL011)
        (0x09000000, 0x1000),
        // RTC (PL031)
        (0x09011000, 0x1000),
        // Timer (Generic ARM timer)
        (0x09030000, 0x1000),
        // GPIO (PL061)
        (0x09040000, 0x1000),
        // VirtIO MMIO transport
        (0x0a000000, 0x00010000),
        // PCI ECAM (if enabled)
        (0x10000000, 0x20000000),
        // High MMIO for PCIe devices
        (0x30000000, 0x10000000),
    ];
}

/// QEMU-specific initialization
pub unsafe fn init_qemu() {
    use core::ptr;
    
    crate::drivers::uart::println("[QEMU] Initializing virt machine...");
    
    // QEMU's PL011 UART is usually already configured by bootloader
    // We just need to ensure it's enabled
    
    let uart_base = QemuVirtConfig::UART_BASE as *mut u32;
    
    // PL011 UART Control Register (offset 0x30)
    // Enable UART, TX, and RX
    ptr::write_volatile(uart_base.add(0x30 / 4), 0x301);
    
    // PL011 Line Control Register (offset 0x2C)
    // 8 bits, no parity, 1 stop bit
    ptr::write_volatile(uart_base.add(0x2C / 4), 0x60);
    
    crate::drivers::uart::println("[QEMU] UART initialized");
    crate::drivers::uart::println("[QEMU] Ready for UI development!");
}
