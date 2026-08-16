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
- ✅ Cross-compilation to x86_64-unknown-none
- ⏳ Bootloader integration (next step)
- ⏳ VGA/FrameBuffer display output
- ⏳ Keyboard/touch input handling
- ⏳ Memory management
- ⏳ Process scheduling

## Building

### Prerequisites

1. **Rust toolchain** with nightly features:
   ```bash
   rustup install nightly
   rustup default nightly
   rustup target add x86_64-unknown-none
   rustup target add aarch64-unknown-none
   ```

2. **Bootloader tools**:
   ```bash
   cargo install bootimage
   ```

3. **QEMU** for testing:
   ```bash
   # Ubuntu/Debian
   sudo apt install qemu-system-x86 qemu-system-arm
   
   # macOS
   brew install qemu
   ```

### Build Commands

```bash
# Build for x86_64
cargo build --target x86_64-unknown-none

# Build for ARM (mobile devices)
cargo build --target aarch64-unknown-none

# Build release version
cargo build --release --target x86_64-unknown-none
```

## Running with QEMU

```bash
# Run x86_64 version
qemu-system-x86_64 -kernel target/x86_64-unknown-none/debug/phone_os

# With debugging
qemu-system-x86_64 -kernel target/x86_64-unknown-none/debug/phone_os -s -S
```

## Roadmap

### Phase 1: Foundation
- [x] Basic kernel skeleton
- [ ] Bootloader integration (UEFI/Legacy BIOS)
- [ ] VGA text mode output
- [ ] Basic interrupt handling

### Phase 2: Core Systems
- [ ] Physical memory management
- [ ] Virtual memory (paging)
- [ ] Multitasking basics
- [ ] Timer interrupts

### Phase 3: Hardware Support
- [ ] PS/2 keyboard driver
- [ ] FrameBuffer graphics
- [ ] Touch screen controller
- [ ] Battery management
- [ ] Power management

### Phase 4: Mobile Features
- [ ] ARM architecture support
- [ ] GPU acceleration
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
- **Development**: x86_64 for easier QEMU testing

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
