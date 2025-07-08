# Cyl Language Test Suite

This directory contains the automated test suite for the Cyl programming language compiler. The test system is designed to validate both correct and incorrect Cyl code, ensuring the parser and compiler behave as expected.

## Directory Structure

```
tests/
├── README.md           # This file
├── fixtures/           # Test cases organized by expected outcome
│   ├── valid/         # Tests that should parse successfully
│   └── invalid/       # Tests that should fail to parse
└── integration/       # Future: Integration tests (not yet implemented)
```

## Test Categories

### Valid Tests (`fixtures/valid/`)

These tests contain syntactically correct Cyl code that the parser should accept. Examples include:

- `hello_world.cyl` - Basic function and print statement
- `simple_function.cyl` - Function definition and calling
- `simple_import.cyl` - Import statements
- `simple_struct_def.cyl` - Struct definitions
- `test_func.cyl` - Function with parameters and return types

### Invalid Tests (`fixtures/invalid/`)

These tests contain syntactically incorrect Cyl code that the parser should reject. Examples include:

- `invalid_identifier.cyl` - Using invalid characters in identifiers
- `mismatched_parens.cyl` - Unbalanced parentheses
- `missing_function_body.cyl` - Function declarations without bodies
- `missing_semicolon.cyl` - Missing required semicolons
- `unclosed_brace.cyl` - Unmatched braces

## Running Tests

The test system is integrated into the Cyl compiler CLI. Use the `cyl test` command to run tests:

### Basic Usage

```bash
# Run all tests
cylc test

# Run with verbose output (shows individual test results)
cylc test --verbose

# Continue running tests even after failures
cylc test --continue-on-failure

# Filter tests by name pattern
cylc test --pattern "simple"

# Combine options
cylc test --verbose --pattern "function" --continue-on-failure
```

### Command Options

- `--verbose` (`-v`): Show detailed output for each test case
- `--pattern` (`-p`): Filter tests by filename pattern (case-sensitive substring match)
- `--continue-on-failure` (`-c`): Don't stop after the first test failure

## Test Output

The test runner provides colored output with emojis for better readability:

```
🧪 Running Cyl automated tests...

📂 Running valid test cases...
  ✅ hello_world.cyl
  ✅ simple_function.cyl
  ❌ complex_feature.cyl
  Ran 3 tests: 2 passed, 1 failed

📂 Running invalid test cases...
  ✅ invalid_identifier.cyl
  ✅ mismatched_parens.cyl
  Ran 2 tests: 2 passed, 0 failed

📊 Test Summary:
   Total:  5
   Passed: 4 ✅
   Failed: 1 ❌

❌ Some tests failed
```

## Adding New Tests

### Valid Test Cases

To add a new valid test case:

1. Create a `.cyl` file in `tests/fixtures/valid/`
2. Write syntactically correct Cyl code
3. The test will automatically be discovered and run

Example:

```cyl
// tests/fixtures/valid/my_test.cyl
fn greet(name: string) -> void {
    println("Hello, " + name + "!");
}

fn main() -> void {
    greet("World");
}
```

### Invalid Test Cases

To add a new invalid test case:

1. Create a `.cyl` file in `tests/fixtures/invalid/`
2. Write Cyl code with intentional syntax errors
3. The test will automatically be discovered and run

Example:

```cyl
// tests/fixtures/invalid/missing_brace.cyl
fn broken_function() -> void {
    println("This is missing a closing brace");
    // Missing }
```

## Test Implementation Details

The test system works by:

1. **Discovery**: Scanning the `tests/fixtures/` directory for `.cyl` files
2. **Categorization**: Separating tests into `valid` and `invalid` categories
3. **Execution**: Running the Cyl parser on each test file
4. **Validation**: Checking if the parse result matches expectations:
   - Valid tests should parse successfully
   - Invalid tests should fail to parse
5. **Reporting**: Providing detailed success/failure information

### Test Runner Logic

```rust
// Simplified test logic
fn run_single_test(file: &PathBuf, should_succeed: bool) -> Result<bool> {
    let source = std::fs::read_to_string(file)?;
    let parse_result = try_parse_file(&source);

    match (should_succeed, parse_result) {
        (true, Ok(_)) => Ok(true),   // Valid test passed
        (false, Err(_)) => Ok(true), // Invalid test passed (failed as expected)
        _ => Ok(false)               // Test failed
    }
}
```

## Current Limitations

The current test system has some limitations that may be addressed in future versions:

1. **Parse-only testing**: Tests only validate parsing, not semantic analysis or code generation
2. **No expected output**: Tests don't verify program output or behavior
3. **No performance testing**: No benchmarks or performance regression tests
4. **Limited error checking**: Only checks if parsing succeeds/fails, not specific error messages

## Future Enhancements

Planned improvements to the test system include:

- **Semantic testing**: Validate type checking and semantic analysis
- **Output verification**: Compare program output against expected results
- **Error message testing**: Verify specific error messages for invalid code
- **Integration tests**: End-to-end testing of compilation and execution
- **Performance benchmarks**: Regression testing for compilation speed
- **Code generation tests**: Validate generated code correctness

## Writing Good Tests

### For Valid Tests

- Keep tests focused on a single feature or concept
- Use descriptive filenames (e.g., `struct_with_methods.cyl`)
- Include comments explaining what the test validates
- Start with simple cases before adding complex scenarios

### For Invalid Tests

- Create one test per type of syntax error
- Use descriptive names that indicate the error (e.g., `missing_semicolon.cyl`)
- Include comments explaining what error is expected
- Test edge cases and common mistakes

### Best Practices

- Keep test files small and focused
- Use consistent naming conventions
- Add both positive and negative test cases for new features
- Test boundary conditions and edge cases
- Document any special requirements or dependencies

## Troubleshooting

### Common Issues

**Tests not being discovered:**

- Ensure files have `.cyl` extension
- Check that files are in the correct directory (`tests/fixtures/valid/` or `tests/fixtures/invalid/`)
- Verify file permissions allow reading

**Unexpected test failures:**

- Run individual tests with `cyl check <filename>` to see detailed error messages
- Use `--verbose` flag to see what the parser is doing
- Check for unsupported language features (async/await, advanced types, etc.)

**Parser errors:**

- The current parser implementation is still evolving
- Some advanced features may not be implemented yet
- Complex expressions or statements might need to be simplified

## Contributing

When contributing new tests:

1. Follow the naming conventions
2. Add tests for both valid and invalid cases
3. Update this README if adding new test categories
4. Ensure tests are deterministic and don't depend on external resources
5. Test your tests before submitting (make sure they pass/fail as expected)

---

The test system is designed to grow with the Cyl language implementation. As new features are added to the language, corresponding tests should be added to ensure reliability and prevent regressions.
