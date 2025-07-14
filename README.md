# cyl `.cyl`

```cyl
// hello_world.cyl
fn main() -> void {
    print("Hello, World!");
    print("Welcome to Cyl programming language!");
}
```

**Runtime Output:**

```
$ cylc run hello_world.cyl
Hello, World!
Welcome to Cyl programming language!
```

[![CI](https://github.com/cyl-lang/cyl/actions/workflows/ci.yml/badge.svg)](https://github.com/cyl-lang/cyl/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/cyl-lang/cyl/branch/master/graph/badge.svg)](https://codecov.io/gh/cyl-lang/cyl)

## Documentation

- [Implementation Plan](IMPLEMENTATION_PLAN.md)
- [Language Specification](LANGUAGE_SPEC.md)
- [Standard Library](STDLIB.md)
- [Tests](tests/README.md)

## Quick Start

```bash
# Setup development environment
make setup

# Generate documentation
make setup-docs
make docs

# Build everything
make build-all
```

**Purpose:** Systems & web programming with simple syntax, multi-backend compilation, safe concurrency, OS & network integration

**Paradigm:** Imperative, concurrent, safe systems language with modern syntax inspired by TypeScript/Python and safety from Rust/Go

**Architecture:** Multi-backend compilation system supporting Cranelift (fast development), LLVM (optimized production), and interpreter (immediate execution)

---

- [Usage](#usage)
  - [Installation](#installation)
  - [Commands](#commands)
  - [Backend Selection](#backend-selection)
- [Multi-Backend Architecture](#multi-backend-architecture)
- [Development & CI/CD](#development--cicd)
  - [Local Development](#local-development)
  - [Continuous Integration](#continuous-integration)

---

## Usage

### Installation

```bash
git clone https://github.com/clxrityy/cyl.git
cd cyl
make install

# For development without LLVM dependencies:
cd compiler && cargo build --no-default-features
```

### Commands

```bash
# Compile and run a Cyl program (default: Cranelift backend)
cylc run examples/hello_world.cyl

# Build a Cyl program to executable
cylc build examples/hello_world.cyl
./examples/hello_world

# Check syntax without compiling
cylc check examples/hello_world.cyl

# Show AST for debugging
cylc ast examples/hello_world.cyl

# Run automated tests
cylc test
```

### Backend Selection

Cyl supports multiple compilation backends for different use cases:

```bash
# Fast development compilation (default)
cylc run --backend cranelift examples/hello_world.cyl

# Optimized production compilation (requires LLVM)
cylc run --backend llvm examples/hello_world.cyl

# Immediate execution for testing/development
cylc run --backend interpreter examples/hello_world.cyl

# Quiet mode for clean output (useful in CI/scripts)
cylc run --backend interpreter --quiet examples/hello_world.cyl
```

## Multi-Backend Architecture

Cyl features a flexible multi-backend compilation system designed to optimize for different workflows:

### 🏗️ **Cranelift Backend** (Default)

- **Use Case**: Fast development cycles, CI/CD pipelines
- **Advantages**: Pure Rust implementation, fast compilation, no external dependencies
- **Output**: Native object files and executables
- **Best For**: Development, testing, and deployment where compile speed matters

### 🚀 **LLVM Backend** (Optional)

- **Use Case**: Production builds requiring maximum optimization
- **Advantages**: Industry-standard optimizations, mature toolchain
- **Output**: Highly optimized native code
- **Best For**: Production releases and performance-critical applications

### 💻 **Interpreter Backend**

- **Use Case**: Educational, rapid prototyping, integration testing
- **Advantages**: Immediate execution, no compilation step, full language support
- **Output**: Direct program execution with real-time output
- **Best For**: Learning, debugging, and scenarios requiring immediate feedback

### Backend Feature Matrix

| Feature              | Cranelift    | LLVM             | Interpreter  |
| -------------------- | ------------ | ---------------- | ------------ |
| Compilation Speed    | ⚡ Fast      | 🐌 Slow          | ⚡⚡ Instant |
| Runtime Performance  | 🚀 Good      | 🚀🚀 Excellent   | 🐌 Moderate  |
| Dependencies         | ✅ None      | ❌ LLVM Required | ✅ None      |
| Development Workflow | ✅ Excellent | ❌ Slow          | ✅ Excellent |
| Production Ready     | ✅ Yes       | ✅ Yes           | ❌ No        |
| Debugging Support    | ✅ Good      | ✅ Excellent     | ✅ Excellent |

### Build Options

Cyl supports flexible compilation with optional LLVM:

```bash
# Full build with LLVM (requires LLVM 14+ installed)
cargo build

# Development build without LLVM (faster, fewer dependencies)
cargo build --no-default-features

# Testing both modes
cargo test --no-default-features  # Test without LLVM
cargo test                         # Test with LLVM
```

---

## Development & CI/CD

### Local Development

```bash
make setup          # Set up development environment
make test           # Run all tests
make install        # Install cylc globally

# Development builds:
cd compiler && cargo build --no-default-features  # Without LLVM
cd compiler && cargo build                         # With LLVM (if available)
```

### Continuous Integration

- ✅ **Automated testing** on push/PR to main branches
- ✅ **Ubuntu-focused reliable CI** with progressive testing (no-LLVM first, then optional LLVM)
- ✅ **Feature-flagged compilation** supporting both LLVM and non-LLVM builds
- ✅ **Security auditing** for Rust and npm dependencies
- ✅ **Code coverage** reporting with Codecov
- ✅ **Automated releases** on version tags
- ✅ **Weekly dependency updates** via automated PRs

See [CI/CD Documentation](.github/workflows/README.md) for details.
