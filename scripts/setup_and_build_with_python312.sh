#!/bin/zsh
# Automated setup for PyO3 + Rust + Python 3.12 on macOS arm64 (Homebrew)
# Installs Python 3.12, sets up environment, and builds the project.

set -e

# 1. Install Python 3.12 with Homebrew
if ! brew list python@3.12 &>/dev/null; then
  echo "Installing Python 3.12 via Homebrew..."
  brew install python@3.12
else
  echo "Python 3.12 already installed."
fi

PYTHON_BIN="/opt/homebrew/bin/python3.12"
PYTHON_LIB="/opt/homebrew/opt/python@3.12/lib"

# 2. Check for shared library, search and symlink if needed
if [ ! -f "$PYTHON_LIB/libpython3.12.dylib" ]; then
  # Try to find it in the Frameworks directory
  FRAMEWORKS_DYLIB="/opt/homebrew/opt/python@3.12/Frameworks/Python.framework/Versions/3.12/lib/libpython3.12.dylib"
  if [ -f "$FRAMEWORKS_DYLIB" ]; then
    echo "Symlinking libpython3.12.dylib from Frameworks to $PYTHON_LIB..."
    ln -sf "$FRAMEWORKS_DYLIB" "$PYTHON_LIB/libpython3.12.dylib"
  else
    echo "ERROR: libpython3.12.dylib not found in $PYTHON_LIB or Frameworks. Aborting."
    exit 1
  fi
fi

# 3. Set environment variables
export PYO3_PYTHON="$PYTHON_BIN"
export LDFLAGS="-undefined dynamic_lookup"
export DYLD_LIBRARY_PATH="/opt/homebrew/opt/python@3.12/lib"
export LIBRARY_PATH="/opt/homebrew/opt/python@3.12/lib"


# 4. Show Python version and location
echo "[DEBUG] Python binary: $PYTHON_BIN"
$PYTHON_BIN --version || { echo '[ERROR] Python binary not working!'; exit 1; }
$PYTHON_BIN -c "import sys; print(sys.prefix)" || { echo '[ERROR] Python sys.prefix failed!'; exit 1; }

# 5. Check for shared library at runtime
if [ ! -f "$PYTHON_LIB/libpython3.12.dylib" ]; then
  echo "[ERROR] libpython3.12.dylib missing at runtime: $PYTHON_LIB/libpython3.12.dylib"
  otool -L $PYTHON_BIN || true
  exit 1
fi

echo "[DEBUG] DYLD_LIBRARY_PATH: $DYLD_LIBRARY_PATH"
echo "[DEBUG] LIBRARY_PATH: $LIBRARY_PATH"
echo "[DEBUG] PYO3_PYTHON: $PYO3_PYTHON"
env | grep PY

# 6. Clean and build

# 5. Clean and build
cargo clean
cargo test --verbose
