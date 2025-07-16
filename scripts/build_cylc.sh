#!/bin/zsh

# Ensure Homebrew Python is installed and arm64
PYTHON_BIN=$(which python3)
PYTHON_PREFIX=$($PYTHON_BIN -c "import sys; print(sys.prefix)")
PYTHON_VERSION=$($PYTHON_BIN -c "import sys; print(f'{sys.version_info.major}.{sys.version_info.minor}')")
ARCH=$($PYTHON_BIN -c "import platform; print(platform.machine())")

echo "Using Python: $PYTHON_BIN"
echo "Python version: $PYTHON_VERSION"
echo "Python arch: $ARCH"
echo "Python prefix: $PYTHON_PREFIX"

if [[ "$ARCH" != "arm64" ]]; then
  echo "ERROR: Python is not arm64. Please install arm64 Python via Homebrew."
  exit 1
fi

export PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1
export PYO3_PYTHON="$PYTHON_BIN"
export LDFLAGS="-L$PYTHON_PREFIX/lib -framework Python"
export CPPFLAGS="-I$PYTHON_PREFIX/include"
export DYLD_LIBRARY_PATH="$PYTHON_PREFIX/lib"
if [[ -d "/opt/homebrew/opt/python@${PYTHON_VERSION}" ]]; then
  export LDFLAGS="$LDFLAGS -L/opt/homebrew/opt/python@${PYTHON_VERSION}/lib"
  export CPPFLAGS="$CPPFLAGS -I/opt/homebrew/opt/python@${PYTHON_VERSION}/include"
  export DYLD_LIBRARY_PATH="$DYLD_LIBRARY_PATH:/opt/homebrew/opt/python@${PYTHON_VERSION}/lib"
fi

echo "Cleaning build..."
cargo clean

echo "Running build/tests..."
cargo test
