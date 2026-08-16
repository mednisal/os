//! ARM64 architecture-specific code

pub mod devices;
pub mod dtb;
pub mod gic;
pub mod mmu;

pub use devices::*;
pub use dtb::*;
pub use gic::*;
pub use mmu::*;

/// Architecture name
pub const ARCH_NAME: &str = "aarch64";

/// Initialize architecture-specific components
/// 
/// # Arguments
/// * `dtb_ptr` - Pointer to device tree blob passed by bootloader
/// 
/// # Safety
/// This function performs low-level hardware initialization
pub unsafe fn init(dtb_ptr: *const u8) {
    // Check for device-specific configuration first
    #[cfg(any(feature = "pinephone", feature = "pixel_6", feature = "oneplus_9"))]
    {
        if let Some(device) = devices::get_device_config() {
            crate::drivers::uart::println(&format!("[INIT] Detected: {}", device.NAME));
            crate::drivers::uart::println(&format!("[INIT] SoC: {}", device.SOC));
            
            // Device-specific initialization
            #[cfg(feature = "pinephone")]
            devices::pinephone::init_pinephone();
            
            #[cfg(feature = "pixel_6")]
            devices::pixel6::init_pixel6();
            
            #[cfg(feature = "oneplus_9")]
            devices::oneplus9::init_oneplus9();
            
            // Reconfigure GIC with device-specific addresses
            gic::init_gic_with_addresses(device.GICD_BASE, device.GICC_BASE);
            
            // Configure UART with device-specific address
            uart::init_with_base(device.UART_BASE);
        } else {
            // Fall back to generic initialization
            init_generic(dtb_ptr);
        }
    }
    
    #[cfg(not(any(feature = "pinephone", feature = "pixel_6", feature = "oneplus_9")))]
    {
        init_generic(dtb_ptr);
    }
}

/// Generic initialization for QEMU or unknown hardware
unsafe fn init_generic(dtb_ptr: *const u8) {
    // Parse device tree
    if let Some(dt_info) = unsafe { dtb::parse_dtb(dtb_ptr) } {
        // Log CPU count (in real implementation, would use serial output)
        let _cpu_count = dt_info.cpu_count;
    }
    
    // Initialize MMU with identity mapping
    mmu::init_mmu(Some(dtb_ptr));
    
    // Initialize interrupt controller (GIC)
    let gic_info = gic::init_gic();
    let _ = gic_info; // Use variable to avoid warning
    
    // Enable interrupts globally
    gic::enable_interrupts();
}

/// Get the number of CPUs from device tree
/// 
/// # Safety
/// Requires valid DTB pointer
pub unsafe fn get_cpu_count(dtb_ptr: *const u8) -> u32 {
    unsafe { dtb::parse_dtb(dtb_ptr) }
        .map(|info| info.cpu_count)
        .unwrap_or(1)
}
