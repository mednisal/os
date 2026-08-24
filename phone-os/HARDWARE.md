# Phone OS - AArch64 Kernel for Real Phone Hardware

A bare-metal Rust kernel designed to run on real ARM64 phone hardware (Snapdragon, MediaTek, Allwinner) as well as QEMU emulation.

## Supported Devices

### Currently Implemented
- **Pine64 PinePhone** (Allwinner A64) - Best supported for development
- **Google Pixel 6** (Tensor GS101) - Configuration provided
- **OnePlus 9** (Snapdragon 888) - Configuration provided

### Building for Specific Devices

```bash
# Default build for QEMU virt machine
cargo build --release

# Build for PinePhone (recommended for real hardware testing)
cargo build --release --features pinephone

# Build for Pixel 6
cargo build --release --features pixel_6

# Build for OnePlus 9
cargo build --release --features oneplus_9

# Generic real hardware mode
cargo build --release --features real_hardware
```

## Boot Requirements

### For QEMU Emulation
```bash
./run.sh
```

### For Real Hardware (PinePhone Example)

1. **Build the kernel:**
   ```bash
   cargo build --release --features pinephone
   ```

2. **Extract the raw binary:**
   ```bash
   aarch64-linux-gnu-objcopy -O binary target/aarch64-unknown-none-softfloat/release/phone_os Image
   ```

3. **Prepare boot files:**
   - Copy `Image` to the boot partition
   - Ensure device tree blob (`.dtb`) is available
   - Configure U-Boot bootcmd

4. **U-Boot boot commands:**
   ```
   setenv bootargs 'console=ttyS0,115200 earlycon=uart,mmio32,0x01c28000'
   load mmc 0:1 ${kernel_addr_r} Image
   load mmc 0:1 ${fdt_addr_r} sun50i-a64-pinephone.dtb
   booti ${kernel_addr_r} - ${fdt_addr_r}
   ```

### For Other Devices

Each device requires:
- Correct kernel load address (configured in device module)
- Appropriate device tree blob from vendor/mainline Linux
- UART configuration for early console
- Bootloader (U-Boot, LK, or proprietary) configured to load kernel

## Hardware Abstraction

The kernel uses a device abstraction layer in `src/arch/aarch64/devices/`:

```rust
pub trait DeviceConfig {
    const NAME: &'static str;
    const SOC: &'static str;
    const KERNEL_LOAD_ADDR: u64;
    const DTB_LOAD_ADDR: u64;
    const UART_BASE: u64;
    const UART_CLOCK: u32;
    const GICD_BASE: u64;
    const GICC_BASE: u64;
    // ... more hardware specifics
}
```

## Key Drivers

- **UART**: PL011-compatible serial driver with configurable base addresses
- **GIC**: Generic Interrupt Controller (v2/v3) support
- **MMU**: ARM64 page table setup with identity mapping
- **Framebuffer**: Basic display output (address from DTB/bootloader)

## Memory Map

Typical layout for most devices:
```
0x00000000 - 0x7FFFFFFF: Device MMIO regions
0x80000000 - 0xFFFFFFFF: RAM (kernel loaded here)
```

## Development Notes

### Adding New Device Support

1. Create new module in `src/arch/aarch64/devices/<device>.rs`
2. Implement `DeviceConfig` trait with hardware addresses
3. Add initialization function for device-specific setup
4. Register in `devices/mod.rs`
5. Add feature flag in `Cargo.toml`

### Debugging on Real Hardware

- Use UART for early console output
- Check bootloader logs for memory map
- Verify device tree is passed correctly
- Use JTAG if available for low-level debugging

## Safety Warnings

⚠️ **Running on real hardware carries risks:**
- May void warranty
- Could potentially brick device if bootloader is modified incorrectly
- Always backup original firmware before experimentation
- Test thoroughly in QEMU first

## License

MIT/Apache-2.0
