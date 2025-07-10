#!/bin/bash
# CI Debug Script - Use this to test LLVM installation locally

echo "=== Testing LLVM Installation ==="

# Check if LLVM packages are available
echo "Checking available LLVM packages..."
apt-cache search llvm-15 | head -10

# Install LLVM using official repository (like CI)
echo "Installing LLVM 15 from official repository..."
wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | sudo apt-key add -
echo "deb http://apt.llvm.org/jammy/ llvm-toolchain-jammy-15 main" | sudo tee /etc/apt/sources.list.d/llvm.list
sudo apt-get update -y

sudo apt-get install -y \
  llvm-15 \
  llvm-15-dev \
  llvm-15-tools \
  clang-15 \
  libclang-15-dev \
  cmake \
  pkg-config

# Create symlinks
sudo ln -sf /usr/bin/llvm-config-15 /usr/bin/llvm-config

# Check installation
echo "Verifying installation..."
which llvm-config-15
llvm-config-15 --version
llvm-config-15 --prefix
llvm-config-15 --libdir
ls -la /usr/lib/llvm-15/lib/ | head -5

# Test environment variables that llvm-sys looks for
export LLVM_SYS_150_PREFIX=/usr/lib/llvm-15
export LLVM_CONFIG_PATH=/usr/bin/llvm-config-15
export LLVM_CONFIG=/usr/bin/llvm-config-15

echo "=== Testing llvm-sys compatibility ==="
echo "LLVM_SYS_150_PREFIX: $LLVM_SYS_150_PREFIX"
echo "LLVM_CONFIG_PATH: $LLVM_CONFIG_PATH"

# Test if we can compile a simple Rust program that uses llvm-sys
echo "Creating test Rust project..."
cd /tmp
cargo new llvm_test --bin
cd llvm_test
echo 'llvm-sys = "150"' >> Cargo.toml
echo 'fn main() { println!("LLVM linkage test"); }' > src/main.rs

echo "Testing compilation..."
cargo check --verbose

echo "Environment ready for Rust compilation"
