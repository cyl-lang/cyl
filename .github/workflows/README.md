# GitHub Actions CI/CD

This document explains the GitHub Actions workflows configured for the Cyl language project.

## Workflows

### 1. CI Workflow (`.github/workflows/ci.yml`)

**Triggers:** Push to `main`/`develop` branches, Pull Requests

**Jobs:**

- **test-rust**: Tests the Rust compiler
  - Formatting check (`cargo fmt`)
  - Linting (`cargo clippy`)
  - Build and run Rust tests
  - Run Cyl language tests with `cylc test`
- **test-design-tools**: Tests TypeScript design tools
  - Type checking (`tsc --noEmit`)
  - Jest tests for design tools
  - Grammar validation
  - AST generation
- **test-integration**: Integration tests
  - Full environment setup
  - Run all tests with `make test`
  - CLI installation test
- **security-audit**: Security checks
  - Rust dependency audit (`cargo audit`)
  - npm security audit
- **coverage**: Code coverage reporting
  - Generate coverage with `cargo llvm-cov`
  - Upload to Codecov

### 2. Cross-Platform Tests (`.github/workflows/cross-platform.yml`)

**Triggers:** Push to `main`, Pull Requests, Daily schedule

**Matrix Testing:**

- **Operating Systems**: Ubuntu, Windows, macOS
- **Rust Versions**: stable, beta
- **Node.js Versions**: 16, 18, 20

**Features:**

- Tests minimum supported versions
- Ensures cross-platform compatibility
- Comprehensive environment matrix

### 3. Release Workflow (`.github/workflows/release.yml`)

**Triggers:** Git tags matching `v*.*.*`

**Features:**

- Creates GitHub releases automatically
- Builds binaries for multiple platforms:
  - Linux x86_64
  - Windows x86_64
  - macOS x86_64
  - macOS ARM64
- Uploads release assets

### 4. Dependencies (`.github/workflows/dependencies.yml`)

**Triggers:** Weekly schedule (Sundays), Manual dispatch

**Features:**

- Automated dependency updates
- Separate jobs for Rust and npm dependencies
- Creates pull requests with updates
- Runs tests to ensure compatibility

## Status Badges

The following badges are displayed in the README:

```markdown
[![CI](https://github.com/clxrityy/cyl/actions/workflows/ci.yml/badge.svg)](https://github.com/clxrityy/cyl/actions/workflows/ci.yml)
[![Cross-Platform Tests](https://github.com/clxrityy/cyl/actions/workflows/cross-platform.yml/badge.svg)](https://github.com/clxrityy/cyl/actions/workflows/cross-platform.yml)
[![codecov](https://codecov.io/gh/clxrityy/cyl/branch/main/graph/badge.svg)](https://codecov.io/gh/clxrityy/cyl)
```

## Local Testing

Before pushing, you can run the same tests locally:

```bash
# Run all tests
make test

# Run individual test suites
make test-rust
make test-design

# Format and lint
make format
make lint

# Full check (same as CI)
make full-check
```

## Security

- **Dependency Scanning**: Automated security audits for both Rust and npm dependencies
- **SARIF Upload**: Security findings are uploaded to GitHub Security tab
- **Audit Failure**: CI fails if high-severity vulnerabilities are found

## Coverage

- **Rust Coverage**: Generated using `cargo llvm-cov`
- **TypeScript Coverage**: Jest built-in coverage
- **Upload**: Coverage reports uploaded to Codecov
- **PR Comments**: Coverage diff shown in pull request comments

## Performance

- **Caching**: Aggressive caching of dependencies and build artifacts
- **Parallel Jobs**: Independent jobs run in parallel
- **Matrix Strategy**: Fail-fast disabled to see all platform results
- **Artifact Retention**: Build artifacts kept for 30 days

## Troubleshooting

### Common Issues

1. **Rust Toolchain**: Ensure Rust version is compatible
2. **Node.js Version**: Use LTS versions (16, 18, 20)
3. **Dependencies**: Run `npm ci` instead of `npm install` in CI
4. **Caching**: Clear caches if builds become inconsistent

### Debugging Workflows

- Use `workflow_dispatch` trigger for manual testing
- Add `continue-on-error: true` for debugging
- Check individual job logs in GitHub Actions tab
- Use `tmate` action for interactive debugging if needed

## Contributing

When contributing to CI:

1. Test workflows locally first
2. Use small, focused changes
3. Update documentation when adding new jobs
4. Consider impact on CI costs and runtime
5. Follow existing patterns and naming conventions
