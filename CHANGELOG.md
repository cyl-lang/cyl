# cyl-lang

## 0.1.2

### Patch Changes

- Add parser tests for new Cyl language features: generics, default arguments, tuples, nullable and dynamic types, and pattern matching. These tests ensure the parser supports the latest syntax and type system improvements.
- ## Summary

  - Refactored parser to support new Cyl language features:
    - Generics for functions, structs, and enums
    - Default arguments in functions
    - Tuple types and tuple return types
    - Nullable and dynamic types
    - Pattern matching
    - Type inference and angle-bracket type annotations in variable declarations
  - Improved lookahead logic in `parse_statement` to distinguish between declarations and expressions.
  - Fixed `parse_declare` to support identifier-based declarations (e.g., `x = 42;`, `y <float> = 3.14;`).
  - Implemented `parse_match` for pattern matching.
  - Implemented `parse_struct` and `parse_enum` to support generics and robust field/variant parsing.
  - Refactored generics parsing to always advance past the closing `>`.
  - Ensured semicolon handling is only done at the statement level for declarations and expressions.
  - Multiple iterations to address persistent test failures and improve parser robustness.

  ## Remaining Issues

  - Some new feature tests are still failing due to deep parser lookahead/advancement issues, especially for identifier-based declarations and generics.
  - Further review of token advancement and lookahead logic is recommended for full test passing.

  ***

  This changeset documents all parser and test fixes related to new Cyl language features and the ongoing effort to make all new feature tests pass.
