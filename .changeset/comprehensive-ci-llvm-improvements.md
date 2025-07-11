---
"cyl": patch
---

Optimized CI workflows for maximum cost efficiency and reliable LLVM compilation

## 🚀 CI/CD Enhancements

### Cost-Optimized Workflow Strategy

- **Ubuntu-Only CI**: Disabled Windows CI completely to minimize runner costs
- **Single-Platform Testing**: Focus on Linux for primary development and testing
- **Aggressive Caching**: Comprehensive dependency and build artifact caching
- **Fast-Fail Strategy**: Quick validation before expensive operations

### Enhanced LLVM Setup

- Reliable LLVM 15 installation for Ubuntu CI with proper package management
- Comprehensive environment variable configuration for llvm-sys
- Windows testing disabled by default (can be enabled with `[test-windows]` commit message)
- Enhanced debugging output for troubleshooting compilation issues

### New Development Tools

- **Local Debug Script**: `debug-llvm-local.sh` for local LLVM testing
- **Cross-platform Support**: macOS and Linux development environments
- **Environment Verification**: Validates LLVM installation locally

### Build Improvements

- Fixed integration test dependencies with release build step
- Added explicit llvm-sys and inkwell compilation verification
- Enhanced error handling with multiple fallback strategies
- Comprehensive logging for CI debugging
- **Critical PIE Fix**: Resolved Linux linker errors by configuring Position Independent Code
  - Changed LLVM target from `RelocMode::Default` to `RelocMode::PIC`
  - Added `-pie` linker flag for Linux to generate Position Independent Executables
  - Fixes "relocation R_X86_64_32 against `.rodata.str1.1` can not be used when making a PIE object" errors

### Key Technical Changes

- **Environment Variables**: Proper LLVM_SYS_150_PREFIX setup for Ubuntu LLVM 15
- **Package Installation**: Direct apt-get installation of llvm-15, llvm-15-dev, clang-15
- **Ubuntu-Only CI**: Focused testing on single platform to minimize costs
- **Dependency Verification**: Pre-compilation checks for critical LLVM dependencies
- **Linux PIE Compatibility**: Position Independent Code/Executable configuration for modern security standards
- **Cost Control**: Windows CI disabled by default, can be enabled manually when needed

This resolves the `LLVM_SYS_NOT_FOUND` errors that were preventing successful CI builds and provides a robust, multi-platform compilation environment for the Cyl programming language.
