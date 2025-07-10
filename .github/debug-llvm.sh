#!/bin/bash
# CI Debug Script - Use this to test LLVM installation locally

echo "=== Testing LLVM Installation ==="

# Check if LLVM packages are available
echo "Checking available LLVM packages..."
apt-cache search llvm-15 | head -10

# Check if we can install LLVM
echo "Installing LLVM 15..."
sudo apt-get update -y
sudo apt-get install -y llvm-15 llvm-15-dev llvm-15-tools

# Check installation
echo "Verifying installation..."
which llvm-config-15
llvm-config-15 --version
llvm-config-15 --libdir
ls -la /usr/lib/llvm-15/lib/ | head -5

# Test environment variables
export LLVM_SYS_150_PREFIX=/usr/lib/llvm-15
export LLVM_CONFIG_PATH=/usr/bin/llvm-config-15
export LLVM_CONFIG=/usr/bin/llvm-config-15

echo "Environment ready for Rust compilation"
