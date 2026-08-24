//! Device-specific configurations for real phone hardware and QEMU
//! 
//! This module contains hardware configurations for various phone platforms.
//! Each device has specific memory maps, peripheral addresses, and initialization sequences.

#[cfg(feature = "qemu")]
pub mod qemu;

#[cfg(feature = "pinephone")]
pub mod pinephone;

#[cfg(feature = "pixel_6")]
pub mod pixel6;

#[cfg(feature = "oneplus_9")]
pub mod oneplus9;

/// Trait for device-specific hardware configuration (methods only, not dyn-compatible)
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

/// Get the current device configuration using feature flags
#[inline]
pub fn get_device_config() -> Option<&'static str> {
    #[cfg(feature = "qemu")]
    return Some("qemu");
    
    #[cfg(feature = "pinephone")]
    return Some("pinephone");
    
    #[cfg(feature = "pixel_6")]
    return Some("pixel_6");
    
    #[cfg(feature = "oneplus_9")]
    return Some("oneplus_9");
    
    #[cfg(not(any(feature = "qemu", feature = "pinephone", feature = "pixel_6", feature = "oneplus_9")))]
    return None;
}

/// Initialize device-specific hardware based on feature flag
pub unsafe fn init_device() {
    #[cfg(feature = "qemu")]
    {
        qemu::init_qemu();
    }
    
    #[cfg(feature = "pinephone")]
    {
        pinephone::init_pinephone();
    }
    
    #[cfg(feature = "pixel_6")]
    {
        pixel6::init_pixel6();
    }
    
    #[cfg(feature = "oneplus_9")]
    {
        oneplus9::init_oneplus9();
    }
}

/// Get device-specific UART base address
pub fn get_uart_base() -> Option<u64> {
    #[cfg(feature = "qemu")]
    return Some(qemu::QemuVirtConfig::UART_BASE);
    
    #[cfg(feature = "pinephone")]
    return Some(pinephone::PinePhoneConfig::UART_BASE);
    
    #[cfg(feature = "pixel_6")]
    return Some(pixel6::Pixel6Config::UART_BASE);
    
    #[cfg(feature = "oneplus_9")]
    return Some(oneplus9::OnePlus9Config::UART_BASE);
    
    #[cfg(not(any(feature = "qemu", feature = "pinephone", feature = "pixel_6", feature = "oneplus_9")))]
    return None;
}

/// Get device-specific GIC addresses
pub fn get_gic_addresses() -> Option<(u64, u64)> {
    #[cfg(feature = "qemu")]
    return Some((qemu::QemuVirtConfig::GICD_BASE, qemu::QemuVirtConfig::GICC_BASE));
    
    #[cfg(feature = "pinephone")]
    return Some((pinephone::PinePhoneConfig::GICD_BASE, pinephone::PinePhoneConfig::GICC_BASE));
    
    #[cfg(feature = "pixel_6")]
    return Some((pixel6::Pixel6Config::GICD_BASE, pixel6::Pixel6Config::GICC_BASE));
    
    #[cfg(feature = "oneplus_9")]
    return Some((oneplus9::OnePlus9Config::GICD_BASE, oneplus9::OnePlus9Config::GICC_BASE));
    
    #[cfg(not(any(feature = "qemu", feature = "pinephone", feature = "pixel_6", feature = "oneplus_9")))]
    return None;
}

/// Get device name string
pub fn get_device_name() -> &'static str {
    #[cfg(feature = "qemu")]
    return qemu::QemuVirtConfig::NAME;
    
    #[cfg(feature = "pinephone")]
    return pinephone::PinePhoneConfig::NAME;
    
    #[cfg(feature = "pixel_6")]
    return pixel6::Pixel6Config::NAME;
    
    #[cfg(feature = "oneplus_9")]
    return oneplus9::OnePlus9Config::NAME;
    
    #[cfg(not(any(feature = "qemu", feature = "pinephone", feature = "pixel_6", feature = "oneplus_9")))]
    return "Unknown Device";
}

/// Get SoC name string
pub fn get_soc_name() -> &'static str {
    #[cfg(feature = "qemu")]
    return qemu::QemuVirtConfig::SOC;
    
    #[cfg(feature = "pinephone")]
    return pinephone::PinePhoneConfig::SOC;
    
    #[cfg(feature = "pixel_6")]
    return pixel6::Pixel6Config::SOC;
    
    #[cfg(feature = "oneplus_9")]
    return oneplus9::OnePlus9Config::SOC;
    
    #[cfg(not(any(feature = "qemu", feature = "pinephone", feature = "pixel_6", feature = "oneplus_9")))]
    return "Unknown SoC";
}
