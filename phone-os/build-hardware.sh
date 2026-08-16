#!/bin/bash

# Build script for real phone hardware targets
# This script helps build the kernel for various devices

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

show_help() {
    echo "Phone OS Hardware Build Script"
    echo ""
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --pinephone     Build for Pine64 PinePhone (Allwinner A64)"
    echo "  --pixel6        Build for Google Pixel 6 (Tensor GS101)"
    echo "  --oneplus9      Build for OnePlus 9 (Snapdragon 888)"
    echo "  --qemu          Build for QEMU virt machine (default)"
    echo "  --binary        Also create raw binary (Image) for booting"
    echo "  --help          Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 --pinephone --binary"
    echo "  $0 --pixel6"
    echo "  $0 --qemu"
}

TARGET=""
CREATE_BINARY=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --pinephone)
            TARGET="pinephone"
            shift
            ;;
        --pixel6)
            TARGET="pixel_6"
            shift
            ;;
        --oneplus9)
            TARGET="oneplus_9"
            shift
            ;;
        --qemu)
            TARGET="qemu"
            shift
            ;;
        --binary)
            CREATE_BINARY=true
            shift
            ;;
        --help)
            show_help
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            show_help
            exit 1
            ;;
    esac
done

# Default to QEMU if no target specified
if [ -z "$TARGET" ]; then
    TARGET="qemu"
fi

echo -e "${GREEN}Building Phone OS for: ${TARGET}${NC}"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}ERROR: Rust/Cargo is not installed${NC}"
    echo ""
    echo "To install Rust, run:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    exit 1
fi

# Check for required toolchain
echo -e "${YELLOW}Checking for AArch64 target...${NC}"
rustup target add aarch64-unknown-none-softfloat 2>/dev/null || true

# Build command
BUILD_CMD="cargo build --release"

if [ "$TARGET" != "qemu" ]; then
    BUILD_CMD="$BUILD_CMD --features $TARGET"
fi

echo -e "${GREEN}Running: $BUILD_CMD${NC}"
$BUILD_CMD

if [ $? -eq 0 ]; then
    echo -e "${GREEN}Build successful!${NC}"
    
    KERNEL_BIN="target/aarch64-unknown-none-softfloat/release/phone_os"
    
    if [ -f "$KERNEL_BIN" ]; then
        echo ""
        echo -e "${GREEN}Kernel ELF: $KERNEL_BIN${NC}"
        echo "Size: $(ls -lh $KERNEL_BIN | awk '{print $5}')"
        
        if [ "$CREATE_BINARY" = true ]; then
            echo ""
            echo -e "${YELLOW}Creating raw binary...${NC}"
            
            if command -v aarch64-linux-gnu-objcopy &> /dev/null; then
                aarch64-linux-gnu-objcopy -O binary "$KERNEL_BIN" Image
                echo -e "${GREEN}Raw binary created: Image${NC}"
                echo "Size: $(ls -lh Image | awk '{print $5}')"
            elif command -v llvm-objcopy &> /dev/null; then
                llvm-objcopy -O binary "$KERNEL_BIN" Image
                echo -e "${GREEN}Raw binary created: Image${NC}"
                echo "Size: $(ls -lh Image | awk '{print $5}')"
            else
                echo -e "${RED}WARNING: No objcopy found. Install binutils-aarch64-linux-gnu or llvm.${NC}"
                echo "To create binary manually:"
                echo "  aarch64-linux-gnu-objcopy -O binary $KERNEL_BIN Image"
            fi
        fi
        
        echo ""
        echo "=========================================="
        echo "Next Steps:"
        echo "=========================================="
        
        if [ "$TARGET" = "qemu" ]; then
            echo "Run in QEMU:"
            echo "  ./run.sh"
        elif [ "$TARGET" = "pinephone" ]; then
            echo "For PinePhone:"
            echo "  1. Copy 'Image' to boot partition"
            echo "  2. Ensure sun50i-a64-pinephone.dtb is available"
            echo "  3. Boot via U-Boot:"
            echo "     load mmc 0:1 \${kernel_addr_r} Image"
            echo "     load mmc 0:1 \${fdt_addr_r} sun50i-a64-pinephone.dtb"
            echo "     booti \${kernel_addr_r} - \${fdt_addr_r}"
        elif [ "$TARGET" = "pixel_6" ]; then
            echo "For Pixel 6:"
            echo "  WARNING: Pixel 6 requires unlocked bootloader"
            echo "  1. Fastboot boot Image (for testing)"
            echo "     fastboot boot Image"
            echo "  2. Or flash to boot partition (advanced)"
        elif [ "$TARGET" = "oneplus_9" ]; then
            echo "For OnePlus 9:"
            echo "  WARNING: OnePlus 9 requires unlocked bootloader"
            echo "  1. Fastboot boot Image (for testing)"
            echo "     fastboot boot Image"
        fi
        
    else
        echo -e "${RED}ERROR: Kernel binary not found at $KERNEL_BIN${NC}"
        exit 1
    fi
else
    echo -e "${RED}Build failed!${NC}"
    exit 1
fi
