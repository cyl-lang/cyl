---
"cyl": patch
---

# Windows CI LLVM Detection Fix

Fixed critical Windows CI pipeline issue where LLVM 15 was not being detected properly, causing build failures with `LLVM_SYS_NOT_FOUND` error.

## 🐛 Bug Fixes

### Windows LLVM Installation

- **Enhanced Chocolatey installation** with version fallback mechanisms
- **Robust path detection** searching multiple possible LLVM installation directories
- **Automatic environment setup** for `LLVM_SYS_150_PREFIX` and `LIBCLANG_PATH`
- **Installation verification** with `llvm-config.exe` version check
- **Debug logging** for troubleshooting LLVM detection issues

### CI Improvements

- **Fallback installation strategy** when specific LLVM version is unavailable
- **Multiple path checking** for different Windows installation scenarios
- **Enhanced error handling** with detailed diagnostic information
- **Cross-platform compatibility** maintained for Ubuntu and macOS

## 🔧 Technical Details

### LLVM Detection Logic

```powershell
# Search multiple possible installation paths
$possiblePaths = @(
  "C:\Program Files\LLVM",
  "C:\Program Files (x86)\LLVM",
  "C:\tools\llvm",
  "C:\ProgramData\chocolatey\lib\llvm\tools"
)
```

### Environment Variables Set

- `LLVM_SYS_150_PREFIX` - Main LLVM installation directory
- `LIBCLANG_PATH` - Path to LLVM bin directory for clang detection
- `PATH` - Updated to include LLVM binaries

### Verification Steps

- Automatic `llvm-config.exe --version` execution
- Directory structure validation
- Debug output for troubleshooting

This fix ensures the Windows CI pipeline can successfully build the Rust compiler with LLVM backend support, maintaining cross-platform compatibility for the Cyl language development.
