#!/bin/bash

# Local build testing script for Cyl project
# Tests both LLVM and no-LLVM build modes

echo "=== Cyl Project Build Testing Script ==="
echo ""

# Function to check command existence
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to test LLVM configuration
test_llvm_config() {
    local config_cmd="$1"
    echo "Testing $config_cmd..."
    
    if command_exists "$config_cmd"; then
        echo "✓ $config_cmd found"
        echo "  Version: $($config_cmd --version)"
        echo "  Prefix: $($config_cmd --prefix)"
        echo "  LibDir: $($config_cmd --libdir)"
        echo "  Components: $($config_cmd --libs core || echo 'Component check failed')"
        echo ""
        return 0
    else
        echo "✗ $config_cmd not found"
        echo ""
        return 1
    fi
}

# Check for various LLVM versions
echo "=== Checking for LLVM installations ==="
LLVM_FOUND=false

for version in 14 15 16 17 18 19; do
    if test_llvm_config "llvm-config-$version"; then
        LLVM_FOUND=true
        PREFERRED_VERSION=$version
        break
    fi
done

if test_llvm_config "llvm-config"; then
    LLVM_FOUND=true
    PREFERRED_VERSION="default"
fi

if [ "$LLVM_FOUND" = false ]; then
    echo "⚠️  No LLVM installation found - but that's okay!"
    echo ""
    echo "Cyl supports both LLVM and no-LLVM builds:"
    echo "  • No-LLVM: Fast development builds for parser/AST work"
    echo "  • LLVM: Full compilation to native machine code"
    echo ""
    echo "Testing no-LLVM build first..."
    
    # Test no-LLVM build
    cd compiler
    echo "Building without LLVM..."
    if cargo build --no-default-features; then
        echo "✅ No-LLVM build successful!"
        echo "Running no-LLVM tests..."
        if cargo test --no-default-features; then
            echo "✅ No-LLVM tests passed!"
        else
            echo "❌ No-LLVM tests failed!"
            exit 1
        fi
    else
        echo "❌ No-LLVM build failed!"
        exit 1
    fi
    cd ..
    
    echo ""
    echo "🎉 Project works without LLVM! For full compilation features, install LLVM:"
    echo ""
    
    # Detect OS and provide specific instructions
    if [[ "$OSTYPE" == "darwin"* ]]; then
        echo "macOS - Install with Homebrew:"
        echo "  brew install llvm@14"
        echo "  export LLVM_SYS_140_PREFIX=\"/opt/homebrew/opt/llvm@14\""
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        echo "Linux - Install with apt (Ubuntu/Debian):"
        echo "  sudo apt-get update"
        echo "  sudo apt-get install llvm-14 llvm-14-dev clang-14"
        echo "  export LLVM_SYS_140_PREFIX=\"/usr/lib/llvm-14\""
    else
        echo "Unknown OS. Please install LLVM 14+ manually."
    fi
    
    exit 0
fi

echo "✅ LLVM found! Using version $PREFERRED_VERSION"
echo ""

# Set up environment for testing
if [ "$PREFERRED_VERSION" = "14" ]; then
    export LLVM_SYS_140_PREFIX=$(llvm-config-14 --prefix)
    export LLVM_CONFIG_PATH=$(which llvm-config-14)
    LLVM_CONFIG_CMD="llvm-config-14"
elif [ "$PREFERRED_VERSION" = "15" ]; then
    export LLVM_SYS_150_PREFIX=$(llvm-config-15 --prefix)
    export LLVM_CONFIG_PATH=$(which llvm-config-15)
    LLVM_CONFIG_CMD="llvm-config-15"
elif [ "$PREFERRED_VERSION" = "default" ]; then
    # Try to determine version
    LLVM_VERSION=$(llvm-config --version | cut -d. -f1)
    if [ "$LLVM_VERSION" = "14" ]; then
        export LLVM_SYS_140_PREFIX=$(llvm-config --prefix)
        export LLVM_CONFIG_PATH=$(which llvm-config)
        LLVM_CONFIG_CMD="llvm-config"
    elif [ "$LLVM_VERSION" = "15" ]; then
        export LLVM_SYS_150_PREFIX=$(llvm-config --prefix)
        export LLVM_CONFIG_PATH=$(which llvm-config)
        LLVM_CONFIG_CMD="llvm-config"
    else
        echo "⚠️  LLVM version $LLVM_VERSION found, but we prefer version 14"
        echo "   This might cause compilation issues"
        LLVM_CONFIG_CMD="llvm-config"
    fi
else
    echo "⚠️  LLVM version $PREFERRED_VERSION found, we prefer version 14"
    LLVM_CONFIG_CMD="llvm-config-$PREFERRED_VERSION"
fi

echo "=== Environment Setup ==="
if [ -n "$LLVM_SYS_140_PREFIX" ]; then
    echo "LLVM_SYS_140_PREFIX: $LLVM_SYS_140_PREFIX"
elif [ -n "$LLVM_SYS_150_PREFIX" ]; then
    echo "LLVM_SYS_150_PREFIX: $LLVM_SYS_150_PREFIX"
fi
echo "LLVM_CONFIG_PATH: $LLVM_CONFIG_PATH"
echo ""

# Test library availability
echo "=== Testing LLVM Libraries ==="
LIBDIR=$($LLVM_CONFIG_CMD --libdir)
echo "Library directory: $LIBDIR"

if [ -d "$LIBDIR" ]; then
    echo "LLVM libraries found:"
    find "$LIBDIR" -name "libLLVM*" 2>/dev/null | head -3 | sed 's/^/  /'
    echo ""
else
    echo "❌ LLVM library directory not found!"
    exit 1
fi

# Test Rust toolchain
echo "=== Testing Rust Environment ==="
if command_exists cargo; then
    echo "✓ Cargo found: $(cargo --version)"
    echo "✓ Rustc found: $(rustc --version)"
else
    echo "❌ Cargo not found! Please install Rust toolchain"
    exit 1
fi
echo ""

# Test llvm-sys compilation
echo "=== Testing Both Build Modes ==="
cd compiler

echo "1. Testing no-LLVM build first..."
cargo clean -q

echo "Building without LLVM dependencies..."
if cargo build --no-default-features; then
    echo "✅ No-LLVM build successful!"
    echo "Running no-LLVM tests..."
    if cargo test --no-default-features; then
        echo "✅ No-LLVM tests passed!"
    else
        echo "❌ No-LLVM tests failed!"
        exit 1
    fi
else
    echo "❌ No-LLVM build failed!"
    exit 1
fi

echo ""
echo "2. Testing LLVM build..."
cargo clean -q

echo "Testing llvm-sys dependency..."
if cargo check --package llvm-sys 2>/dev/null; then
    echo "✅ llvm-sys compilation successful!"
else
    echo "❌ llvm-sys compilation failed!"
    echo ""
    echo "Trying with verbose output..."
    cargo check --package llvm-sys --verbose
    exit 1
fi

echo ""
echo "Testing inkwell dependency..."
if cargo check --package inkwell 2>/dev/null; then
    echo "✅ inkwell compilation successful!"
else
    echo "❌ inkwell compilation failed!"
    echo ""
    echo "Trying with verbose output..."
    cargo check --package inkwell --verbose
    exit 1
fi

echo ""
echo "=== Building Full Project with LLVM ==="
echo "Building release binary with LLVM..."
if cargo build --release; then
    echo "✅ LLVM release build successful!"
else
    echo "❌ LLVM release build failed!"
    echo ""
    echo "Trying with verbose output..."
    cargo build --release --verbose
    exit 1
fi

echo ""
echo "=== Full Project Test with LLVM ==="
echo "Testing clippy..."
if cargo clippy --quiet -- -D warnings; then
    echo "✅ Clippy passed!"
else
    echo "❌ Clippy failed!"
    exit 1
fi

echo ""
echo "Testing cargo test with LLVM..."
if cargo test; then
    echo "✅ LLVM Rust tests passed!"
else
    echo "❌ LLVM Rust tests failed!"
    exit 1
fi

cd ..

# Test TypeScript if Node.js is available
if command_exists node && command_exists npm; then
    echo ""
    echo "=== Testing TypeScript ==="
    if [ -f "package.json" ]; then
        echo "Installing npm dependencies..."
        npm ci --silent
        
        echo "Running TypeScript tests..."
        if npm run test:design --silent; then
            echo "✅ TypeScript tests passed!"
        else
            echo "❌ TypeScript tests failed!"
            exit 1
        fi
    fi
fi

echo ""
echo "🎉 Both build modes work! Your environment supports:"
echo "  ✅ No-LLVM builds (development/testing)"
echo "  ✅ LLVM builds (full compilation)"
echo ""
echo "Environment variables for LLVM builds:"
if [ -n "$LLVM_SYS_140_PREFIX" ]; then
    echo "  export LLVM_SYS_140_PREFIX=\"$LLVM_SYS_140_PREFIX\""
elif [ -n "$LLVM_SYS_150_PREFIX" ]; then
    echo "  export LLVM_SYS_150_PREFIX=\"$LLVM_SYS_150_PREFIX\""
fi
echo "  export LLVM_CONFIG_PATH=\"$LLVM_CONFIG_PATH\""
