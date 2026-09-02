# Phone OS on Renode

This guide explains how to run Phone OS on Renode, an open-source hardware simulation framework.

## What is Renode?

Renode is a powerful tool for simulating and debugging embedded systems. It allows you to:
- Simulate ARM64 hardware without physical devices
- Debug your OS with detailed logging and analysis
- Test hardware interactions in a controlled environment
- Reproduce bugs consistently

## Quick Start

### 1. Install Renode

**Ubuntu/Debian:**
```bash
wget https://github.com/renode/renode/releases/download/v1.14.0/renode_1.14.0_amd64.deb
sudo dpkg -i renode_1.14.0_amd64.deb
```

**macOS:**
```bash
brew install renode
```

**Windows:**
Download installer from https://renode.io/

### 2. Build Phone OS

```bash
cd phone-os
cargo build --release
```

### 3. Run on Renode

**Using the run script (recommended):**
```bash
./run.sh
```

**Manual method:**
```bash
renode
include @renode/platform.repl
sysbus LoadELF target/aarch64-unknown-none-softfloat/release/phone_os
machine Start
analyze uart
```

## Renode Commands

Once Renode is running, useful commands include:

| Command | Description |
|---------|-------------|
| `machine Start` | Start the simulated machine |
| `machine Stop` | Stop the machine |
| `machine Reset` | Reset the machine |
| `analyze uart` | Show UART output in terminal |
| `sysbus LoadELF <file>` | Load kernel ELF file |
| `logLevel 3` | Set log verbosity (0-5) |
| `peripherals` | List all peripherals |
| `cpu` | Show CPU state |
| `monitor` | Open interactive monitor |

## Platform Configuration

The `renode/platform.repl` file defines the simulated hardware:

- **CPU**: Cortex-A72 @ sysbus
- **UART**: PL011 at 0x9000000 (serial console)
- **GIC**: GICv2 interrupt controller at 0x8000000
- **Framebuffer**: Linear framebuffer at 0x50000000 (1080x2400, 32bpp)
- **RAM**: 256MB starting at 0x40000000

## Debugging Tips

### View Serial Output
```bash
renode> analyze uart
```

### Enable Detailed Logging
```bash
renode> logLevel 5
renode> log -1 "system"  # Log everything
```

### Inspect Memory
```bash
renode> sysbus ReadDoubleWord 0x50000000  # Read framebuffer start
renode> sysbus DumpMemory 0x40000000 0x1000  # Dump RAM region
```

### Check CPU State
```bash
renode> cpu  # Show registers
renode> sysbus.elzer  # Show exception level
```

## Framebuffer Display

Renode automatically opens a window for the framebuffer. If it doesn't appear:

1. Check that the framebuffer peripheral is defined in `platform.repl`
2. Ensure the address matches your driver (0x50000000)
3. Verify dimensions match (1080x2400)

## Troubleshooting

### "Machine Start" fails
- Check that the ELF file was built successfully
- Verify the path to the kernel binary
- Ensure Renode version is 1.12 or newer

### No serial output
- Run `analyze uart` to enable UART monitoring
- Check UART base address matches (0x9000000)
- Verify baud rate settings in driver

### Framebuffer not displaying
- Confirm framebuffer address in driver matches platform file
- Check that dimensions are reasonable (< 4096x4096)
- Try reducing resolution for testing

## Advanced Usage

### Scripting Renode

Create a `.resc` script file:
```python
# my_script.resc
include @renode/platform.repl
sysbus LoadELF target/aarch64-unknown-none-softfloat/release/phone_os
machine Start
analyze uart
logLevel 3
```

Run with:
```bash
renode my_script.resc
```

### Multiple Machines

Renode can simulate multiple machines connected via network:
```python
machines:
    machine1: 
        peripherals: sysbus1
        cpu: cpu1
    machine2:
        peripherals: sysbus2
        cpu: cpu2
```

### Custom Peripherals

Add custom devices to your platform file:
```python
my_device: Miscellaneous.MyDevice @ sysbus 0xA000000
    param1: 0x1234
```

## Resources

- [Renode Documentation](https://renode.readthedocs.io/)
- [Renode GitHub](https://github.com/renode/renode)
- [Renode Examples](https://github.com/renode/renode-examples)
- [Phone OS README](../README.md)
