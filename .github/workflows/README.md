# GitHub Actions CI/CD

This document explains the GitHub Actions workflows configured for the Cyl language project, supporting the multi-backend architecture (Cranelift, LLVM, Interpreter).

## Workflows

### 1. CI Workflow (`.github/workflows/ci.yml`)

**Triggers:** Push to `master`/`development` branches, Pull Requests

**Strategy:** Reliability-focused CI with multi-backend testing approach

**Jobs:**

#### **test-no-llvm** (Primary Build)

- **Purpose**: Test core functionality with Cranelift backend (default)
- **Environment**: Ubuntu 22.04
- **Features**:
  - Rust toolchain with clippy
  - Node.js 22 with npm dependency caching
  - **Multi-Backend Testing**:
    - Builds compiler with `--no-default-features` (Cranelift-only)
    - Tests Cranelift backend compilation
    - Tests interpreter backend execution
    - Integration tests across all backends
  - TypeScript design tools testing
  - **Zero External Dependencies**: No LLVM required

#### **test-with-llvm-ubuntu** (Optional Enhanced Build)

- **Purpose**: Test LLVM backend when available
- **Environment**: Ubuntu 22.04 (best LLVM compatibility)
- **Dependencies**: Runs only after `test-no-llvm` succeeds
- **Features**:
  - LLVM 14 installation attempt
  - **Graceful Degradation**: Continues if LLVM unavailable
  - Tests full feature set when LLVM is present
  - Production optimization testing

**Architecture Benefits:**

- **Guaranteed Success**: Core functionality always works (Cranelift + Interpreter)
- **Enhanced Features**: LLVM optimizations when possible
- **Fast Feedback**: Primary pipeline completes quickly
- **Cross-Backend Validation**: Ensures feature parity

### 2. Multi-Backend Testing Strategy

**Current Focus:** Reliability and compatibility over complexity

**Supported Platforms:**

- **Primary**: Ubuntu 22.04 (stable, reliable CI environment)
- **Backends**: Cranelift (default), LLVM (optional), Interpreter (educational)

**Testing Philosophy:**

- **Core First**: Ensure basic functionality always works
- **Enhanced Second**: Add optimizations when environment supports them
- **Graceful Degradation**: Never fail due to optional dependencies

### 3. Future Workflow Expansions

**Planned Additions:**

- **Cross-Platform Tests**: macOS, Windows support when core is stable
- **Release Workflow**: Automated binary releases with multi-backend builds
- **Performance Benchmarks**: Backend comparison and regression detection
- **Security Audits**: Dependency scanning and vulnerability assessment
- **Coverage Reporting**: Multi-backend code coverage analysis

## Multi-Backend Architecture Integration

### Backend Testing Matrix

| Backend         | CI Status        | Use Case                | Dependencies     |
| --------------- | ---------------- | ----------------------- | ---------------- |
| **Cranelift**   | ✅ Always Tested | Development, CI/CD      | None (Pure Rust) |
| **LLVM**        | 🔄 Opportunistic | Production Optimization | LLVM 14+         |
| **Interpreter** | ✅ Always Tested | Education, Testing      | None             |

### Compilation Commands Tested

```bash
# Default (Cranelift) backend
cargo build --no-default-features
cargo test --no-default-features

# LLVM backend (when available)
cargo build --features="llvm"
cargo test --features="llvm"

# Integration tests across backends
cylc run --backend cranelift examples/hello_world.cyl
cylc run --backend interpreter examples/hello_world.cyl
cylc run --backend llvm examples/hello_world.cyl  # if available
```

## Status Badges

The following badges reflect the current CI status:

```markdown
[![CI](https://github.com/clxrityy/cyl/actions/workflows/ci.yml/badge.svg)](https://github.com/clxrityy/cyl/actions/workflows/ci.yml)
```

**Badge Meaning:**

- ✅ **Green**: Cranelift + Interpreter backends working, TypeScript tools passing
- 🔄 **Yellow**: LLVM backend issues (non-blocking)
- ❌ **Red**: Core functionality broken

## Local Testing

Run the same tests locally using the Makefile:

```bash
# Complete test suite (matches CI)
make test

# Individual components
make test-compiler      # Rust compiler tests
make test-design        # TypeScript design tools

# Multi-backend testing
cd compiler
cargo test --no-default-features  # Cranelift + Interpreter
cargo test                         # All backends (if LLVM available)

# Integration tests
cargo test --test integration

# Build variants
cargo build --no-default-features  # Cranelift-only build
cargo build                         # Full build with LLVM if available

# Code quality
make format             # Format all code
make lint              # Lint with clippy
make full-check        # Complete validation
```

## Multi-Backend Development Workflow

### Daily Development

1. **Primary**: Use Cranelift backend for fast iteration
2. **Testing**: Interpreter backend for immediate feedback
3. **Production**: LLVM backend for optimized releases

### CI Integration

1. **Always Pass**: Cranelift + Interpreter functionality
2. **Optional Enhancement**: LLVM optimizations when available
3. **Future-Proof**: Easy addition of new backends

## Architecture Benefits

### Reliability-First Design

- **Zero-Dependency Core**: Cranelift backend requires no external tools
- **Graceful Degradation**: LLVM unavailability doesn't break builds
- **Fast Feedback**: Core tests complete quickly for rapid development

### Multi-Backend Validation

- **Feature Parity**: Same language features across all backends
- **Consistent Behavior**: Integration tests ensure uniform output
- **Backend-Specific Optimization**: Each backend optimized for its use case

### Development Experience

- **Immediate Execution**: Interpreter backend for rapid prototyping
- **Fast Compilation**: Cranelift for development cycles
- **Production Ready**: LLVM for optimized deployments

## Security

- **Dependency Minimization**: Core functionality has zero external dependencies
- **Rust Memory Safety**: All backends benefit from Rust's safety guarantees
- **Feature Flags**: Optional features don't compromise core security
- **Isolation**: Backend failures don't affect other backends

## Performance Considerations

### CI Performance

- **Parallel Testing**: Backend tests run independently
- **Caching Strategy**: Rust and npm dependencies cached aggressively
- **Fail-Fast**: Core failures prevent unnecessary LLVM testing
- **Resource Optimization**: Minimal external dependency installation

### Build Performance

- **Cranelift Speed**: Fast compilation for development workflows
- **LLVM Optimization**: Maximum performance for production builds
- **Interpreter Immediacy**: Zero compilation time for testing

## Troubleshooting

### Common Issues

1. **LLVM Build Failures**

   - ✅ **Expected Behavior**: Core functionality still works
   - 🔧 **Solution**: Use Cranelift backend (`--no-default-features`)
   - 📝 **Note**: LLVM is optional enhancement, not requirement

2. **Integration Test Failures**

   - 🔍 **Check**: Backend selection in test configuration
   - 🎯 **Verify**: Correct backend used for feature requirements
   - 🛠️ **Fix**: Ensure interpreter used for immediate execution tests

3. **Multi-Backend Inconsistencies**
   - 📊 **Compare**: Output across all backends
   - 🔄 **Validate**: Feature parity maintenance
   - 🧪 **Test**: Cross-backend integration tests

### Debugging Workflows

```bash
# Test specific backend locally
cargo test --no-default-features  # Cranelift + Interpreter only
cargo test --features="llvm"       # LLVM if available

# Run integration tests with specific backend
cargo test --test integration -- --nocapture

# Local CI simulation (requires 'act')
make ci-test
```

## Contributing

When contributing to the multi-backend CI system:

### Development Guidelines

1. **Test Locally First**: Use `make test` to validate changes
2. **Backend Compatibility**: Ensure changes work across all backends
3. **Core vs Enhancement**: Distinguish between required and optional features
4. **Documentation**: Update this README when adding new workflows

### Adding New Backends

1. **Feature Flag**: Add to `Cargo.toml` features section
2. **CI Integration**: Update both build jobs appropriately
3. **Testing**: Add backend-specific test cases
4. **Documentation**: Update architecture documentation

### Workflow Modifications

1. **Small Changes**: Test individual components first
2. **Dependency Updates**: Consider impact on all backends
3. **Performance**: Monitor CI runtime and resource usage
4. **Reliability**: Prioritize consistency over complexity

### Best Practices

- **Zero-Dependency Principle**: Keep core functionality independent
- **Graceful Degradation**: Optional features should never break builds
- **Clear Separation**: Distinguish between development and production workflows
- **Comprehensive Testing**: Validate feature parity across backends

## Future Roadmap

### Short Term

- [ ] Add code coverage reporting for multi-backend testing
- [ ] Implement cross-platform testing (macOS, Windows)
- [ ] Add performance benchmarking between backends
- [ ] Create automated release workflow with multi-backend binaries

### Medium Term

- [ ] WebAssembly target integration via Cranelift
- [ ] Advanced optimization comparison between backends
- [ ] Automated dependency updates with backend compatibility testing
- [ ] Security scanning with multi-backend vulnerability assessment

### Long Term

- [ ] Custom backend development framework
- [ ] Cloud-based optimization service integration
- [ ] Advanced debugging workflow with backend switching
- [ ] Community backend contribution system

---

This CI/CD architecture reflects Cyl's commitment to reliability, flexibility, and developer productivity while maintaining the highest standards of code quality and system compatibility.
