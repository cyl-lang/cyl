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
$ cylc build hello_world.cyl && ./hello_world
"Hello, World!"
"Welcome to Cyl programming language!"
```

[![CI](https://github.com/clxrityy/cyl/actions/workflows/ci.yml/badge.svg)](https://github.com/clxrityy/cyl/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/clxrityy/cyl/branch/main/graph/badge.svg)](https://codecov.io/gh/clxrityy/cyl)

- [Implementation Plan](IMPLEMENTATION_PLAN.md)
- [Language Specification](LANGUAGE_SPEC.md)
- [Tests](tests/README.md)

**Purpose:** Systems & web programming with simple syntax, native compilation, safe concurrency, OS & network integration

**Paradigm:** Imperative, concurrent, safe systems language with modern syntax inspired by TypeScript/Python and safety from Rust/Go

---

- [Usage](#usage)
  - [Installation](#installation)
  - [Commands](#commands)
- [Current Implementation Status](#current-implementation-status)
- [Development & CI/CD](#development--cicd)
  - [Local Development](#local-development)
  - [Continuous Integration](#continuous-integration)
- [Key Syntax Features](#key-syntax-features)

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
# Compile and run a Cyl program
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

### Current Implementation Status

**🎉 PRODUCTION READY - Runtime Output Functional!**

**Core Language Features - ✅ Complete:**

- [x] **Native Executable Generation** - Direct compilation to optimized machine code (~16KB executables)
- [x] **Runtime I/O System** - Working `print()` and `print_int()` functions with C standard library integration
- [x] **Type System** - Complete with i32 integers, floats, booleans, strings, arrays, and custom structs
- [x] **Control Flow** - if/else, while loops, for loops with range iteration
- [x] **Data Structures** - Arrays, structs with field access, nested structs
- [x] **Function System** - Parameters, return types, recursion, builtin functions
- [x] **Memory Management** - Stack allocation, proper variable scoping

**Advanced Features - ✅ Complete:**

- [x] **LLVM Code Generation** - Full LLVM IR generation with proper type handling (feature-flagged)
- [x] **Flexible Build System** - Supports builds with and without LLVM dependencies
- [x] **Cross-Platform Development** - Works on macOS, Linux, Windows with conditional compilation
- [x] **Multi-Level Optimization** - Support for -O0 through -O3 optimization levels
- [x] **Standard Library Bindings** - C standard library integration for I/O operations

**Development Infrastructure - ✅ Complete:**

- [x] Lexer with comprehensive token recognition
- [x] Recursive descent parser with error recovery
- [x] AST generation and validation
- [x] Automated test system (Rust + TypeScript)
- [x] CLI with multiple commands (run, build, check, ast, test)
- [x] **Feature-flagged LLVM compilation** with graceful fallback
- [x] **Simplified, reliable CI pipeline** focused on Ubuntu with optional LLVM testing
- [x] Security auditing and dependency management

**Currently Working Examples:**

- `examples/hello_world.cyl` - Basic print functionality
- `examples/print_test.cyl` - String and integer output
- `examples/struct_test.cyl` - Struct creation and field access
- `examples/array_test.cyl` - Array creation and indexing

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

---

## Key Syntax Features

| Syntax     | Type                     |
| ---------- | ------------------------ |
| `fn`       | FunctionDeclaration      |
| `if`       | IfStatement              |
| `else`     | ElseStatement            |
| `import`   | ImportStatement          |
| `=`        | AssignmentExpression     |
| `.`        | MemberExpression         |
| `->`       | ReturnType               |
| `;`        | StatementTerminator      |
| `"`        | StringLiteral            |
| `()`       | CallExpression           |
| `[]`       | ArrayLiteral             |
| `{}`       | BlockStatement           |
| `,`        | ParameterSeparator       |
| `:`        | KeyValueSeparator        |
| `void`     | VoidType                 |
| `declare`  | DeclareStatement         |
| `<>`       | TypeParameter            |
| `return`   | ReturnStatement          |
| `struct`   | StructDeclaration        |
| `enum`     | EnumDeclaration          |
| `match`    | MatchStatement           |
| `for`      | ForStatement             |
| `while`    | WhileStatement           |
| `break`    | BreakStatement           |
| `continue` | ContinueStatement        |
| `try`      | TryStatement             |
| `catch`    | CatchStatement           |
| `throw`    | ThrowStatement           |
| `async`    | AsyncFunctionDeclaration |
| `await`    | AwaitExpression          |
