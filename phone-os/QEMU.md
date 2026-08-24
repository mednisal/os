# QEMU Support for Phone OS

This project now supports running on QEMU for easy development and testing!

## Building for QEMU

```bash
# Build with QEMU support (default)
./run.sh qemu

# Or specify the feature explicitly
cargo build --release --target aarch64-unknown-none-softfloat --features qemu
```

## Running on QEMU

```bash
# Run with graphical output (for UI development)
./run.sh qemu

# This will:
# 1. Build the kernel with QEMU feature flags
# 2. Launch QEMU with virt machine emulation
# 3. Enable virtio-gpu for framebuffer display
# 4. Route serial output to terminal
```

## QEMU Configuration

The QEMU virt machine emulates:
- **CPU**: Cortex-A72 (ARMv8-A)
- **Memory**: 1GB RAM
- **UART**: PL011 at 0x09000000
- **GIC**: v2 interrupt controller
- **GPU**: VirtIO-GPU for framebuffer (1024x768)
- **Interrupts**: Timer and external interrupts

## Requirements

Install QEMU for your platform:

**Ubuntu/Debian:**
```bash
sudo apt install qemu-system-arm
```

**Fedora:**
```bash
sudo dnf install qemu-system-arm
```

**macOS:**
```bash
brew install qemu
```

## Controls

- **Ctrl+A, then X**: Exit QEMU
- **Serial output**: Visible in terminal
- **Graphical window**: Shows framebuffer/UI

## Developing UI

With QEMU support, you can now:
1. Build and run quickly without real hardware
2. Test UI widgets and touch interactions (simulated)
3. Debug using QEMU's logging (`qemu.log`)
4. Iterate faster on UI development

## Real Hardware

To build for real hardware instead:
```bash
./run.sh pinephone    # Pine64 PinePhone
./run.sh pixel_6      # Google Pixel 6
./run.sh oneplus_9    # OnePlus 9
```

## Troubleshooting

If QEMU fails to start:
1. Ensure `qemu-system-aarch64` is installed
2. Check that KVM is available (optional, for acceleration)
3. Review `qemu.log` for error messages
4. Try reducing memory: `-m 512M` instead of `-m 1G`

## Next Steps

Now that QEMU is set up, you can:
- Start building your UI with the widget system
- Test framebuffer drawing primitives
- Develop touch event handling logic
- Create app screens and layouts

Happy developing! 🚀
