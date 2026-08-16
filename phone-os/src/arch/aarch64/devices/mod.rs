//! Device-specific configurations for real phone hardware
//! 
//! This module contains hardware configurations for various phone platforms.
//! Each device has specific memory maps, peripheral addresses, and initialization sequences.

#[cfg(feature = "pinephone")]
pub mod pinephone;

#[cfg(feature = "pixel_6")]
pub mod pixel6;

#[cfg(feature = "oneplus_9")]
pub mod oneplus9;

/// Trait for device-specific hardware configuration
pub trait DeviceConfig {
    /// Device name
    const NAME: &'static str;
    
    /// SoC name
    const SOC: &'static str;
    
    /// Kernel load address (physical)
    const KERNEL_LOAD_ADDR: u64;
    
    /// DTB load address
    const DTB_LOAD_ADDR: u64;
    
    /// UART base address for early console
    const UART_BASE: u64;
    
    /// UART clock frequency
    const UART_CLOCK: u32;
    
    /// GIC Distributor base address
    const GICD_BASE: u64;
    
    /// GIC CPU Interface base address  
    const GICC_BASE: u64;
    
    /// Framebuffer address (if available early)
    const FRAMEBUFFER_ADDR: Option<u64>;
    
    /// Screen width
    const SCREEN_WIDTH: usize;
    
    /// Screen height
    const SCREEN_HEIGHT: usize;
    
    /// RAM base address
    const RAM_BASE: u64;
    
    /// RAM size in bytes
    const RAM_SIZE: u64;
    
    /// Additional MMIO regions: [(base, size), ...]
    const MMIO_REGIONS: &'static [(u64, u64)];
}

/// Get the current device configuration
#[inline]
pub fn get_device_config() -> Option<&'static dyn DeviceConfig> {
    #[cfg(feature = "pinephone")]
    return Some(&pinephone::PinePhoneConfig);
    
    #[cfg(feature = "pixel_6")]
    return Some(&pixel6::Pixel6Config);
    
    #[cfg(feature = "oneplus_9")]
    return Some(&oneplus9::OnePlus9Config);
    
    #[cfg(not(any(feature = "pinephone", feature = "pixel_6", feature = "oneplus_9")))]
    return None;
}
