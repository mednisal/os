#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;

#[cfg(target_arch = "aarch64")]
mod arch {
    pub mod aarch64;
}

mod drivers;

#[cfg(target_arch = "aarch64")]
use arch::aarch64;
use drivers::framebuffer::{Color, FramebufferWriter};

/// Kernel entry point for ARM64
/// 
/// # Arguments
/// * `x0` - Device Tree Blob pointer (passed by bootloader)
#[unsafe(no_mangle)]
pub extern "C" fn main(x0: u64) -> ! {
    #[cfg(target_arch = "aarch64")]
    {
        // Initialize architecture-specific components
        let dtb_ptr = x0 as *const u8;
        
        // Parse device tree and initialize MMU
        unsafe {
            aarch64::init(dtb_ptr);
        }
        
        // Initialize Framebuffer writer with address from DTB
        let mut fb = FramebufferWriter::new();
        
        // Clear screen and set colors
        fb.clear();
        fb.set_color(Color::LIGHT_GREEN, Color::BLACK);
        
        // Write welcome message
        let _ = writeln!(fb, "================================");
        let _ = writeln!(fb, "       PHONE OS BOOTING         ");
        let _ = writeln!(fb, "================================");
        let _ = writeln!(fb, "");
        let _ = writeln!(fb, "ARM64 Kernel Initialized!");
        let _ = writeln!(fb, "Device Tree parsed successfully.");
        let _ = writeln!(fb, "MMU configured with page tables.");
        let _ = writeln!(fb, "");
        
        fb.set_color(Color::LIGHT_GRAY, Color::BLACK);
        let _ = writeln!(fb, "System Information:");
        let _ = writeln!(fb, "  - Architecture: {}", aarch64::ARCH_NAME);
        let _ = writeln!(fb, "  - Written in: Rust");
        let _ = writeln!(fb, "  - No standard library (no_std)");
        
        // Get CPU count from device tree
        unsafe {
            let cpu_count = aarch64::get_cpu_count(dtb_ptr);
            let _ = writeln!(fb, "  - CPUs detected: {}", cpu_count);
        }
        
        let _ = writeln!(fb, "");
        
        fb.set_color(Color::YELLOW, Color::BLACK);
        let _ = writeln!(fb, "Next steps for real hardware:");
        let _ = writeln!(fb, "  1. Integrate UEFI/u-boot bootloader");
        let _ = writeln!(fb, "  2. Add full DTB node parsing");
        let _ = writeln!(fb, "  3. Implement interrupt handling");
        let _ = writeln!(fb, "  4. Add driver support (display, touch, etc.)");
        let _ = writeln!(fb, "");
        
        fb.set_color(Color::LIGHT_CYAN, Color::BLACK);
        let _ = write!(fb, "Phone OS kernel ready... ");
        
        loop {}
    }
    
    #[cfg(not(target_arch = "aarch64"))]
    {
        // Fallback for other architectures
        let mut fb = FramebufferWriter::new();
        fb.clear();
        fb.set_color(Color::LightRed, Color::Black);
        let _ = writeln!(fb, "Error: Unsupported architecture");
        loop {}
    }
}

/// Panic handler - required for no_std environments
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut fb = FramebufferWriter::new();
    fb.set_color(Color::LIGHT_RED, Color::BLACK);
    let _ = writeln!(fb, "");
    let _ = writeln!(fb, "PANIC: {}", info);
    loop {}
}
