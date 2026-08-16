//! Driver modules for hardware support

#[cfg(target_arch = "aarch64")]
pub mod framebuffer;

#[cfg(not(target_arch = "aarch64"))]
pub mod vga;

#[cfg(target_arch = "aarch64")]
pub use framebuffer as vga;
