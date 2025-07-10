---
"cyl": patch
---

Comprehensive CI improvements for LLVM compilation

## 🚀 CI/CD Enhancements

### Enhanced LLVM Setup

- Added multi-strategy LLVM 15 installation for Ubuntu CI
- Comprehensive environment variable configuration for llvm-sys
- Added Windows CI support with automated LLVM installation
- Enhanced debugging output for troubleshooting compilation issues

### New Development Tools

- **Local Debug Script**: Added `debug-llvm-local.sh` for local LLVM testing
- **Cross-platform Detection**: Supports macOS and Linux development environments
- **Environment Verification**: Validates LLVM installation and library availability

### Build Improvements

- Fixed integration test dependencies with release build step
- Added explicit llvm-sys and inkwell compilation verification
- Enhanced error handling with multiple fallback strategies
- Comprehensive logging for CI debugging

### Key Technical Changes

- **Environment Variables**: Proper LLVM_SYS_150_PREFIX and LLVM_CONFIG_PATH setup
- **Library Paths**: Configured LD_LIBRARY_PATH and PKG_CONFIG_PATH for compilation
- **Cross-platform Support**: Both Ubuntu Linux and Windows CI jobs
- **Dependency Verification**: Pre-compilation checks for critical LLVM dependencies

This resolves the `LLVM_SYS_NOT_FOUND` errors that were preventing successful CI builds and provides a robust, multi-platform compilation environment for the Cyl programming language.
