# Phone OS - A Rust-based Mobile Operating System

A bare-metal operating system written primarily in Rust, designed for mobile devices.

## Project Structure

```
phone-os/
├── .cargo/
│   └── config.toml      # Cargo configuration for cross-compilation
├── src/
│   ├── main.rs          # Kernel entry point and panic handler
│   ├── arch/            # Architecture-specific code (x86_64, ARM)
│   ├── drivers/         # Hardware drivers (display, touch, battery, etc.)
│   ├── kernel/          # Core kernel components
│   │   ├── memory.rs    # Memory management
│   │   ├── process.rs   # Process scheduling
│   │   └── interrupt.rs # Interrupt handling
│   ├── hal/             # Hardware Abstraction Layer
│   └── ui/              # User interface components
├── Cargo.toml           # Rust package manifest
└── README.md            # This file
```

## Current Status

- ✅ Basic no_std kernel setup
- ✅ Custom panic handler
- ✅ Cross-compilation to aarch64-unknown-none-softfloat (ARM64)
- ⏳ Bootloader integration (next step)
- ⏳ Framebuffer display output (placeholder)
- ⏳ Device Tree parsing
- ⏳ MMU setup
- ⏳ Keyboard/touch input handling
- ⏳ Memory management
- ⏳ Process scheduling

## Building

### Prerequisites

1. **Rust toolchain** with nightly features:
   ```bash
   rustup install nightly
   rustup default nightly
   rustup target add aarch64-unknown-none-softfloat
   ```

2. **Bootloader tools**:
   ```bash
   cargo install bootimage
   ```

3. **QEMU** for testing:
   ```bash
   # Ubuntu/Debian
   sudo apt install qemu-system-arm qemu-system-x86
   
   # macOS
   brew install qemu
   ```

### Build Commands

```bash
# Build for ARM64 (mobile devices) - Default
cargo build --release

# Build for x86_64 (development/testing)
cargo build --target x86_64-unknown-none --release
```

## Running with QEMU

```bash
# Run ARM64 version (requires proper bootloader)
qemu-system-aarch64 -M virt -cpu cortex-a57 -kernel target/aarch64-unknown-none-softfloat/release/phone_os

# With serial output
qemu-system-aarch64 -M virt -cpu cortex-a57 -kernel target/aarch64-unknown-none-softfloat/release/phone_os -serial stdio

# Run x86_64 version
qemu-system-x86_64 -kernel target/x86_64-unknown-none/debug/phone_os
```

## Important Notes

### ⚠️ Real Hardware Requirements

This is currently a **placeholder kernel**. To run on real ARM hardware (phones), you need:

1. **Device Tree Blob (DTB)**: Each phone has unique hardware that must be described via device tree
2. **Proper Bootloader**: UEFI or u-boot to load the kernel and pass DTB
3. **Framebuffer Setup**: Parse device tree to find framebuffer address, resolution, and pixel format
4. **MMU Configuration**: Set up page tables to map physical memory (including framebuffer) to virtual addresses
5. **Exception Levels**: Understand ARM exception levels (EL0-EL3) and boot at appropriate level

### Supported Hardware

Currently **no real hardware is supported**. This is an educational/experimental project. For actual phone development, consider:

- **PinePhone** - Open hardware with mainline Linux support
- **Librem 5** - Privacy-focused with open documentation
- **Raspberry Pi** - Great for learning ARM development
- **QEMU virt machine** - Best for initial development and testing

## Roadmap

### Phase 1: Foundation
- [x] Basic kernel skeleton
- [x] ARM64 target configuration
- [ ] Bootloader integration (UEFI/u-boot)
- [ ] Device Tree parsing
- [ ] MMU and page table setup
- [ ] Framebuffer initialization from DTB

### Phase 2: Core Systems
- [ ] Physical memory management
- [ ] Virtual memory (paging)
- [ ] Exception handlers (EL1)
- [ ] Timer interrupts
- [ ] Basic console output

### Phase 3: Hardware Support
- [ ] PL011 UART driver (serial console)
- [ ] GIC interrupt controller
- [ ] Touch screen controller
- [ ] Battery management
- [ ] Power management

### Phase 4: Mobile Features
- [ ] GPU acceleration (Adreno/Mali)
- [ ] Cellular modem interface
- [ ] WiFi/Bluetooth
- [ ] Sensor hub (accelerometer, gyroscope)

### Phase 5: User Experience
- [ ] Graphics compositor
- [ ] Touch gesture recognition
- [ ] UI framework
- [ ] Application launcher
- [ ] System apps (dialer, messages, settings)

## Architecture Decisions

### Why Rust?
- Memory safety without garbage collection
- Zero-cost abstractions
- Strong type system
- Growing ecosystem for embedded/OS development

### Target Architecture
- **Primary**: ARM64 (aarch64) for mobile devices
- **Development**: QEMU virt machine for testing

### Design Principles
1. **Microkernel-inspired**: Keep the kernel minimal
2. **Driver isolation**: Run drivers in userspace where possible
3. **Async-first**: Leverage Rust's async capabilities
4. **Security-focused**: Capability-based security model

## Contributing

This is a learning/experimental project. Contributions welcome!

## License

MIT License - see LICENSE file for details

## Resources

- [Philipp Oppermann's OS Blog](https://os.phil-opp.com/)
- [Rust Embedded Book](https://docs.rust-embedded.org/book/)
- [ARM Trusted Firmware](https://github.com/ARM-software/arm-trusted-firmware)
- [UEFI Specification](https://uefi.org/specifications)
- [Device Tree Specification](https://devicetree-specification.readthedocs.io/)
- [ARM Architecture Reference Manual](https://developer.arm.com/documentation)
- [Writing an OS in Rust (AArch64)](https://os.phil-opp.com/aarch64-support/)
