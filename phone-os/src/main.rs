#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;

mod drivers;

use drivers::vga::{Color, FramebufferWriter};

/// Entry point of the kernel
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Initialize Framebuffer writer
    let mut fb = FramebufferWriter::new();
    
    // Clear screen and set colors
    fb.clear();
    fb.set_color(Color::LightGreen, Color::Black);
    
    // Write welcome message
    let _ = writeln!(fb, "================================");
    let _ = writeln!(fb, "       PHONE OS BOOTING         ");
    let _ = writeln!(fb, "================================");
    let _ = writeln!(fb, "");
    let _ = writeln!(fb, "ARM64 Framebuffer Initialized!");
    let _ = writeln!(fb, "Kernel loaded successfully.");
    let _ = writeln!(fb, "");
    
    fb.set_color(Color::LightGray, Color::Black);
    let _ = writeln!(fb, "System Information:");
    let _ = writeln!(fb, "  - Architecture: ARM64 (AArch64)");
    let _ = writeln!(fb, "  - Written in: Rust");
    let _ = writeln!(fb, "  - No standard library (no_std)");
    let _ = writeln!(fb, "");
    
    fb.set_color(Color::Yellow, Color::Black);
    let _ = writeln!(fb, "WARNING: This is a placeholder!");
    let _ = writeln!(fb, "To work on real hardware you need:");
    let _ = writeln!(fb, "  1. Device Tree parsing");
    let _ = writeln!(fb, "  2. Framebuffer address from DTB");
    let _ = writeln!(fb, "  3. MMU setup for memory mapping");
    let _ = writeln!(fb, "  4. Proper bootloader (UEFI/u-boot)");
    let _ = writeln!(fb, "");
    
    fb.set_color(Color::LightCyan, Color::Black);
    let _ = write!(fb, "Phone OS ready... ");
    
    loop {}
}

/// Panic handler - required for no_std environments
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut fb = FramebufferWriter::new();
    fb.set_color(Color::LightRed, Color::Black);
    let _ = writeln!(fb, "");
    let _ = writeln!(fb, "PANIC: {}", info);
    loop {}
}
