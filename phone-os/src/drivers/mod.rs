//! Driver modules for hardware support

#[cfg(target_arch = "aarch64")]
pub mod framebuffer;

#[cfg(target_arch = "aarch64")]
pub mod power;

#[cfg(target_arch = "aarch64")]
pub mod touch;

#[cfg(target_arch = "aarch64")]
pub mod uart;

#[cfg(target_arch = "aarch64")]
pub mod font;

#[cfg(target_arch = "aarch64")]
pub mod ui;
