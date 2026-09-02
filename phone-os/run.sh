#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
NC='\033[0m' # No Color

echo -e "${GREEN}Building Phone OS for AArch64...${NC}"
cargo build --release --target aarch64-unknown-none-softfloat

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

echo -e "${GREEN}Starting Renode emulation...${NC}"
echo "Press Ctrl+] to exit Renode."

# Check if Renode is installed
if ! command -v renode &> /dev/null; then
    echo ""
    echo "ERROR: renode is not installed!"
    echo ""
    echo "To install Renode:"
    echo "  Ubuntu/Debian: Follow instructions at https://renode.io/"
    echo "  Fedora: Follow instructions at https://renode.io/"
    echo "  macOS: brew install renode"
    echo ""
    echo "Kernel binary built successfully at: $KERNEL_BIN"
    echo "Size: $(ls -lh $KERNEL_BIN | awk '{print $5}')"
    echo ""
    echo "You can test it on real hardware or install Renode for emulation."
    exit 0
fi

# Run Renode with our platform definition
renode <<EOF
include @renode/platform.repl
sysbus LoadELF $KERNEL_BIN
machine Start
analyze uart
EOF

echo -e "${GREEN}Emulation finished.${NC}"
