#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;

#[cfg(target_arch = "aarch64")]
mod arch {
    pub mod aarch64;
}

mod drivers;
mod kernel;

#[cfg(target_arch = "aarch64")]
use arch::aarch64;
use drivers::framebuffer::{Color, FramebufferWriter};
use drivers::uart;
use drivers::power::PowerManager;
use drivers::touch::TouchEvent;
use kernel::memory::GlobalAllocator;

/// Set up the global allocator for heap allocations
#[global_allocator]
static GLOBAL: GlobalAllocator = GlobalAllocator;

/// Kernel entry point for ARM64
/// 
/// # Arguments
/// * `x0` - Device Tree Blob pointer (passed by bootloader)
#[unsafe(no_mangle)]
pub extern "C" fn main(x0: u64) -> ! {
    #[cfg(target_arch = "aarch64")]
    {
        // Initialize UART first for serial output
        uart::init();
        uart::println("[BOOT] Phone OS starting...");
        
        // Initialize architecture-specific components
        let dtb_ptr = x0 as *const u8;
        
        // Parse device tree and initialize MMU
        unsafe {
            aarch64::init(dtb_ptr);
        }
        uart::println("[BOOT] Architecture initialized");
        
        // Initialize heap allocator (CRITICAL: enables Box, Vec, String, etc.)
        unsafe {
            kernel::memory::init_heap(0x90000000, 16 * 1024 * 1024); // 16MB heap
        }
        uart::println("[BOOT] Heap allocator initialized");
        
        // Initialize Power Manager with default PMU base
        let mut power_mgr = PowerManager::new(drivers::power::PMU_BASE);
        unsafe {
            power_mgr.init();
        }
        uart::println("[BOOT] Power manager initialized");
        
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
        
        // Show battery status
        if let Some(battery) = power_mgr.get_battery_level() {
            let _ = writeln!(fb, "  - Battery: {}%", battery);
        }
        
        let _ = writeln!(fb, "");
        
        fb.set_color(Color::YELLOW, Color::BLACK);
        let _ = writeln!(fb, "System Status:");
        let _ = writeln!(fb, "  ✓ Heap allocator enabled");
        let _ = writeln!(fb, "  ✓ Power manager active");
        let _ = writeln!(fb, "  ✓ Event loop running");
        let _ = writeln!(fb, "  ⚠ Interrupt handling (TODO)");
        let _ = writeln!(fb, "");
        
        fb.set_color(Color::LIGHT_CYAN, Color::BLACK);
        let _ = writeln!(fb, "Phone OS kernel ready!");
        let _ = writeln!(fb, "");
        let _ = writeln!(fb, "Touch the screen to test input...");
        
        uart::println("[BOOT] Phone OS kernel ready!");
        uart::println("[BOOT] Starting event loop...");
        
        // Main event loop - processes touch events and updates UI
        let mut last_touch_time = 0u64;
        let mut touch_count = 0u32;
        
        loop {
            // Check for touch events (polling - would be interrupt-driven in production)
            if let Some(touch_event) = drivers::touch::read_touch_event() {
                match touch_event {
                    TouchEvent::Press(x, y) => {
                        touch_count += 1;
                        last_touch_time = aarch64::get_timer_value();
                        
                        uart::println("[TOUCH] Press at (");
                        // Simple UART output for coordinates
                        crate::drivers::uart::print_u32(x as u32);
                        crate::drivers::uart::print(", ");
                        crate::drivers::uart::print_u32(y as u32);
                        crate::drivers::uart::println(")");
                        
                        // Update display with touch feedback
                        fb.set_color(Color::LIGHT_BLUE, Color::BLACK);
                        let _ = writeln!(fb, "");
                        let _ = writeln!(fb, "Touch #{} detected at ({}, {})", touch_count, x, y);
                        
                        // Show current battery level
                        if let Some(battery) = power_mgr.get_battery_level() {
                            let _ = writeln!(fb, "Battery: {}%", battery);
                        }
                        
                        // Adjust CPU frequency based on load (simple demo)
                        if touch_count % 5 == 0 {
                            unsafe { power_mgr.set_performance_mode(true); }
                            fb.set_color(Color::LIGHT_GREEN, Color::BLACK);
                            let _ = writeln!(fb, "Performance mode: ON");
                        } else {
                            unsafe { power_mgr.set_performance_mode(false); }
                            fb.set_color(Color::LIGHT_CYAN, Color::BLACK);
                            let _ = writeln!(fb, "Performance mode: ECO");
                        }
                    }
                    TouchEvent::Release(x, y) => {
                        uart::println("[TOUCH] Release at (");
                        crate::drivers::uart::print_u32(x as u32);
                        crate::drivers::uart::print(", ");
                        crate::drivers::uart::print_u32(y as u32);
                        crate::drivers::uart::println(")");
                    }
                    TouchEvent::Move(x, y) => {
                        // Optional: track movement
                        let _ = (x, y);
                    }
                    TouchEvent::MultiTouch(id, x, y) => {
                        // Multi-touch event (unused for now)
                        let _ = (id, x, y);
                    }
                    TouchEvent::None => {
                        // No touch event
                    }
                }
            }
            
            // Idle power management - reduce CPU frequency when idle
            if aarch64::get_timer_value() - last_touch_time > 1000000 {
                unsafe { power_mgr.set_performance_mode(false); }
            }
            
            // Small delay to prevent busy-waiting (would use WFI instruction in production)
            for _ in 0..1000 {
                core::hint::spin_loop();
            }
        }
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
    uart::print("[PANIC] ");
    if let Some(msg) = info.message().as_str() {
        uart::println(msg);
    } else {
        uart::println("Panic occurred");
    }
    
    let mut fb = FramebufferWriter::new();
    fb.set_color(Color::LIGHT_RED, Color::BLACK);
    let _ = writeln!(fb, "");
    let _ = writeln!(fb, "PANIC: {}", info);
    loop {}
}
