#!/bin/bash
# Run Phone OS on Renode

set -e

cd "$(dirname "$0")/.."

echo "Building Phone OS..."
cargo build --release --target aarch64-unknown-none

KERNEL_BIN="target/aarch64-unknown-none/release/phone_os"

if [ ! -f "$KERNEL_BIN" ]; then
    echo "Error: Kernel binary not found at $KERNEL_BIN"
    exit 1
fi

echo "Starting Renode..."
renode <<EOF
include @renode/platform.repl
sysbus LoadELF $KERNEL_BIN
machine Start
analyze uart
EOF
