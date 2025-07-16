#!/bin/bash
# local_ci_check.sh
# Run local equivalents of CI jobs for cyl project
set -euo pipefail

# Colors for output
green='\033[0;32m'
red='\033[0;31m'
nc='\033[0m'

function section() {
  echo -e "${green}\n==== $1 ====\n${nc}"
}

# 1. Test without LLVM
section "Test without LLVM (Rust & Node)"
npm ci
cd compiler && cargo build --no-default-features --verbose
cd compiler && cargo test --no-default-features --verbose
cd ..
npm test

# 2. Test with LLVM (if on Ubuntu and LLVM is installed)
if [[ "$(uname -s)" == "Linux" ]]; then
  section "Test with LLVM (Ubuntu only)"
  if command -v llvm-config-14 &>/dev/null; then
    export LLVM_SYS_140_PREFIX="/usr/lib/llvm-14"
    cd compiler && cargo build --verbose
    cd compiler && cargo test --verbose
    cd ..
  else
    echo -e "${red}LLVM 14 not found, skipping LLVM build/test.${nc}"
  fi
else
  echo -e "${red}LLVM build/test only supported on Ubuntu in CI.${nc}"
fi

# 3. Coverage (TypeScript & Rust)
section "TypeScript Coverage"
npm run test:design:coverage

section "Rust Coverage"
cargo install cargo-tarpaulin || true
npm run coverage:rust

section "Local CI checks complete!"
