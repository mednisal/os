#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;

mod drivers;

use drivers::vga::{Color, VgaWriter};

/// Entry point of the kernel
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Initialize VGA writer directly on stack
    let mut vga = VgaWriter::new();
    
    // Clear screen and set colors
    vga.clear();
    vga.set_color(Color::LightGreen, Color::Black);
    
    // Write welcome message
    let _ = writeln!(vga, "================================");
    let _ = writeln!(vga, "       PHONE OS BOOTING         ");
    let _ = writeln!(vga, "================================");
    let _ = writeln!(vga, "");
    let _ = writeln!(vga, "VGA Text Mode Initialized!");
    let _ = writeln!(vga, "Kernel loaded successfully.");
    let _ = writeln!(vga, "");
    
    vga.set_color(Color::LightGray, Color::Black);
    let _ = writeln!(vga, "System Information:");
    let _ = writeln!(vga, "  - Architecture: x86_64");
    let _ = writeln!(vga, "  - Written in: Rust");
    let _ = writeln!(vga, "  - No standard library (no_std)");
    let _ = writeln!(vga, "");
    
    vga.set_color(Color::Yellow, Color::Black);
    let _ = writeln!(vga, "Next steps:");
    let _ = writeln!(vga, "  1. Implement bootloader integration");
    let _ = writeln!(vga, "  2. Add GDT and IDT setup");
    let _ = writeln!(vga, "  3. Implement memory management");
    let _ = writeln!(vga, "  4. Add interrupt handling");
    let _ = writeln!(vga, "");
    
    vga.set_color(Color::LightCyan, Color::Black);
    let _ = write!(vga, "Phone OS ready... ");
    
    loop {}
}

/// Panic handler - required for no_std environments
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut vga = VgaWriter::new();
    vga.set_color(Color::LightRed, Color::Black);
    let _ = writeln!(vga, "");
    let _ = writeln!(vga, "PANIC: {}", info);
    loop {}
}
