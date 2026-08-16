//! ARM64 architecture-specific code

pub mod dtb;
pub mod mmu;

pub use dtb::*;
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
    // Parse device tree
    if let Some(dt_info) = dtb::parse_dtb(dtb_ptr) {
        // Log CPU count (in real implementation, would use serial output)
        let _cpu_count = dt_info.cpu_count;
    }
    
    // Initialize MMU with identity mapping
    mmu::init_mmu(Some(dtb_ptr));
}

/// Get the number of CPUs from device tree
/// 
/// # Safety
/// Requires valid DTB pointer
pub unsafe fn get_cpu_count(dtb_ptr: *const u8) -> u32 {
    dtb::parse_dtb(dtb_ptr)
        .map(|info| info.cpu_count)
        .unwrap_or(1)
}
