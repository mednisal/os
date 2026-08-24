#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
NC='\033[0m' # No Color

# Default feature (can be overridden by command line)
FEATURE="${1:-qemu}"

echo -e "${GREEN}Building Phone OS for AArch64 (feature: $FEATURE)...${NC}"
cargo build --release --target aarch64-unknown-none-softfloat --features "$FEATURE"

if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi

KERNEL_BIN="target/aarch64-unknown-none-softfloat/release/phone_os"

# Check if kernel exists
if [ ! -f "$KERNEL_BIN" ]; then
    echo "Kernel binary not found. Did the build succeed?"
    ls -la target/aarch64-unknown-none-softfloat/release/ 2>/dev/null || echo "Release directory not found"
    exit 1
fi

echo -e "${GREEN}Starting QEMU with Virt board emulation...${NC}"
echo "Press Ctrl+A then X to exit QEMU."

# Check if QEMU is installed
if ! command -v qemu-system-aarch64 &> /dev/null; then
    echo ""
    echo "ERROR: qemu-system-aarch64 is not installed!"
    echo ""
    echo "To install QEMU:"
    echo "  Ubuntu/Debian: sudo apt install qemu-system-arm"
    echo "  Fedora: sudo dnf install qemu-system-arm"
    echo "  macOS: brew install qemu"
    echo ""
    echo "Kernel binary built successfully at: $KERNEL_BIN"
    echo "Size: $(ls -lh $KERNEL_BIN | awk '{print $5}')"
    echo ""
    echo "You can test it on real hardware or install QEMU for emulation."
    exit 0
fi

# Run QEMU with appropriate settings based on feature
if [ "$FEATURE" = "qemu" ]; then
    # QEMU virt machine with graphical output for UI
    qemu-system-aarch64 \
        -M virt,highmem=on \
        -cpu cortex-a72 \
        -m 1G \
        -kernel "$KERNEL_BIN" \
        -serial mon:stdio \
        -device virtio-gpu-pci \
        -vga virtio \
        -d int,cpu_reset \
        -D qemu.log
    
    echo -e "${GREEN}Emulation finished. Check qemu.log for details.${NC}"
else
    # For real hardware features, use nographic mode
    qemu-system-aarch64 \
        -M virt \
        -cpu cortex-a72 \
        -kernel "$KERNEL_BIN" \
        -nographic \
        -serial mon:stdio \
        -d int,cpu_reset \
        -D qemu.log
    
    echo -e "${GREEN}Emulation finished. Check qemu.log for details.${NC}"
fi
