# Build script for Cyl language project

.PHONY: all build test clean install setup dev ci-test

# Default target
all: build

# Setup development environment
setup:
	@echo "Setting up Cyl development environment..."
	npm install
	cd compiler && cargo build
	@echo "Setup complete!"

# Setup documentation environment
setup-docs:
	@echo "Setting up documentation environment..."
	cd docs/generator && python3 -m venv venv
	cd docs/generator && source venv/bin/activate && pip install -r requirements.txt
	@echo "Documentation environment setup complete!"

# Build everything
build: build-design build-compiler

# Build TypeScript design tools
build-design:
	@echo "Building design tools..."
	npm run build

# Build Rust compiler
build-compiler:
	@echo "Building compiler..."
	cd compiler && cargo build --release

# Run tests
test: test-design test-compiler

# Test design tools
test-design:
	@echo "Testing design tools..."
	npm run test

# Test Rust compiler
test-compiler:
	@echo "Testing compiler..."
	cd compiler && cargo test

# Development mode
dev:
	@echo "Starting development mode..."
	npm run dev

# Validate grammar
validate:
	@echo "Validating grammar..."
	npm run grammar:validate

# Generate AST
generate:
	@echo "Generating AST..."
	npm run ast:generate

# Check syntax of example files
check-examples:
	@echo "Checking example syntax..."
	npm run syntax:check examples/hello_world.cyl
	npm run syntax:check examples/web_request.cyl
	npm run syntax:check examples/file_processing.cyl

# Run full validation and generation
full-check:
	@echo "Running full language design check..."
	npm run dev -- full

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	rm -rf dist/
	rm -rf target/
	rm -rf compiler/target/
	rm -rf node_modules/
	@echo "Clean complete!"

# Install compiler globally
install: build-compiler
	@echo "Installing Cyl compiler..."
	cd compiler && cargo install --path .
	@echo "Cyl compiler installed! Use 'cylc' command."

# Uninstall compiler
uninstall:
	@echo "Uninstalling Cyl compiler..."
	cargo uninstall cylc
	@echo "Cyl compiler uninstalled."

# Development helpers
format:
	@echo "Formatting code..."
	cd compiler && cargo fmt
	npm run format 2>/dev/null || echo "No TypeScript formatter configured"

lint:
	@echo "Linting code..."
	cd compiler && cargo clippy
	npm run lint 2>/dev/null || echo "No TypeScript linter configured"

# Documentation
docs:
	@echo "Generating documentation..."
	cd docs/generator && source venv/bin/activate && python generate-docs.py
	@echo "Documentation generated in docs/website/"

# Version info
version:
	@echo "Cyl Language Development Environment"
	@echo "===================================="
	@echo "Rust version:" 
	@rustc --version
	@echo "Cargo version:"
	@cargo --version
	@echo "Node.js version:"
	@node --version
	@echo "npm version:"
	@npm --version

# Help
help:
	@echo "Cyl Language Build System"
	@echo "========================"
	@echo ""
	@echo "Available targets:"
	@echo "  setup           - Set up development environment"
	@echo "  build           - Build design tools and compiler"
	@echo "  test            - Run all tests"
	@echo "  dev             - Start development mode"
	@echo "  validate        - Validate language grammar"
	@echo "  generate        - Generate AST definitions"
	@echo "  check-examples  - Check syntax of example files"
	@echo "  full-check      - Run complete validation and generation"
	@echo "  clean           - Clean build artifacts"
	@echo "  install         - Install compiler globally"
	@echo "  uninstall       - Uninstall compiler"
	@echo "  format          - Format source code"
	@echo "  lint            - Lint source code"
	@echo "  docs            - Generate documentation"
	@echo "  version         - Show version information"
	@echo "  ci-test         - Test GitHub Actions locally (requires act)"
	@echo "  help            - Show this help message"

# Test GitHub Actions locally
ci-test:
	@echo "Testing GitHub Actions workflows locally..."
	@if command -v act >/dev/null 2>&1; then \
		echo "Running CI workflow with act..."; \
		act -W .github/workflows/ci.yml; \
	else \
		echo "❌ 'act' is not installed. Install it with:"; \
		echo "   brew install act  # on macOS"; \
		echo "   # or visit: https://github.com/nektos/act"; \
	fi

# Build everything including docs
build-all: build docs
