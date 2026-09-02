//! ARM64 architecture-specific code

pub mod devices;
pub mod dtb;
pub mod gic;
pub mod mmu;

use core::sync::atomic::{AtomicU64, Ordering};

/// Architecture name
pub const ARCH_NAME: &str = "aarch64";

/// Simple timer value counter (increments on each read for demo purposes)
static TIMER_VALUE: AtomicU64 = AtomicU64::new(0);

/// Get current timer value (in a real impl, would read hardware timer)
pub fn get_timer_value() -> u64 {
    // In production: read CNTPCT_EL0 system register
    // For now, just increment a counter
    TIMER_VALUE.fetch_add(1, Ordering::Relaxed)
}

/// Initialize architecture-specific components
/// 
/// # Arguments
/// * `dtb_ptr` - Pointer to device tree blob passed by bootloader (null for Renode)
/// 
/// # Safety
/// This function performs low-level hardware initialization
pub unsafe fn init(dtb_ptr: *const u8) {
    // Check for device-specific configuration first
    #[cfg(any(feature = "pinephone", feature = "pixel_6", feature = "oneplus_9"))]
    {
        if let Some(_device_name) = devices::get_device_config() {
            let dev_name = devices::get_device_name();
            let soc_name = devices::get_soc_name();
            crate::drivers::uart::println("[INIT] Detected: ");
            crate::drivers::uart::println(dev_name);
            crate::drivers::uart::println("[INIT] SoC: ");
            crate::drivers::uart::println(soc_name);
            
            // Device-specific initialization
            devices::init_device();
            
            // Reconfigure GIC with device-specific addresses
            if let Some((gicd, gicc)) = devices::get_gic_addresses() {
                gic::init_gic_with_addresses(gicd, gicc);
            }
            
            // Configure UART with device-specific address
            if let Some(uart_base) = devices::get_uart_base() {
                crate::drivers::uart::init_with_base(uart_base);
            }
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

/// Generic initialization for Renode or unknown hardware
unsafe fn init_generic(dtb_ptr: *const u8) {
    // For Renode, dtb_ptr is null, so skip DTB parsing
    if !dtb_ptr.is_null() {
        // Parse device tree
        if let Some(dt_info) = unsafe { dtb::parse_dtb(dtb_ptr) } {
            // Log CPU count (in real implementation, would use serial output)
            let _cpu_count = dt_info.cpu_count;
        }
    } else {
        crate::drivers::uart::println("[INIT] Renode detected - using hardcoded addresses");
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
