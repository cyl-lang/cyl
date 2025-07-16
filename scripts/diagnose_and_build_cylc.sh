#!/bin/zsh

# Diagnostic and build script for PyO3/Python linking issues on macOS arm64
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

# Check for Python library presence
LIB_PATH="$PYTHON_PREFIX/lib"
HOMEBREW_LIB_PATH="/opt/homebrew/opt/python@${PYTHON_VERSION}/lib"

ls $LIB_PATH | grep libpython || echo "No libpython found in $LIB_PATH"
if [[ -d "$HOMEBREW_LIB_PATH" ]]; then
  ls $HOMEBREW_LIB_PATH | grep libpython || echo "No libpython found in $HOMEBREW_LIB_PATH"
else
  echo "No Homebrew Python lib dir found: $HOMEBREW_LIB_PATH"
fi

find $LIB_PATH -name 'libpython*' || echo "No libpython* found in $LIB_PATH"
if [[ -d "$HOMEBREW_LIB_PATH" ]]; then
  find $HOMEBREW_LIB_PATH -name 'libpython*' || echo "No libpython* found in $HOMEBREW_LIB_PATH"
fi

# Print Python details
python3 --version
python3 -c "import sys; print(sys.prefix)"
python3 -c "import platform; print(platform.machine())"

## Set environment variables for build
export PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1
export PYO3_PYTHON="$PYTHON_BIN"
export LDFLAGS="-L$LIB_PATH"
export CPPFLAGS="-I$PYTHON_PREFIX/include"
export DYLD_LIBRARY_PATH="$LIB_PATH"
export LIBRARY_PATH="$LIB_PATH"
export LD_LIBRARY_PATH="$DYLD_LIBRARY_PATH"
if [[ -d "$HOMEBREW_LIB_PATH" ]]; then
  export LDFLAGS="$LDFLAGS -L$HOMEBREW_LIB_PATH"
  export CPPFLAGS="$CPPFLAGS -I/opt/homebrew/opt/python@${PYTHON_VERSION}/include"
  export DYLD_LIBRARY_PATH="$DYLD_LIBRARY_PATH:$HOMEBREW_LIB_PATH"
  export LIBRARY_PATH="$LIBRARY_PATH:$HOMEBREW_LIB_PATH"
  # Check for shared library and add -lpython3.x if present
  SHARED_LIB="libpython${PYTHON_VERSION}.dylib"
  if [[ -f "$HOMEBREW_LIB_PATH/$SHARED_LIB" ]]; then
    export LDFLAGS="$LDFLAGS -lpython${PYTHON_VERSION}"
    echo "Found $SHARED_LIB, linking with -lpython${PYTHON_VERSION}"
  else
    echo "WARNING: $SHARED_LIB not found in $HOMEBREW_LIB_PATH. PyO3 may fail to link."
    echo "You may need to build the shared library manually."
  fi
fi

# Try relinking Homebrew Python
if [[ -d "/opt/homebrew/opt/python@${PYTHON_VERSION}" ]]; then
  brew link --overwrite python@${PYTHON_VERSION}
fi

# Clean and build with verbose output
cargo clean
cargo test --verbose
