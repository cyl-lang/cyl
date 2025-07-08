# cyl

## 0.2.0

### Minor Changes

- Add diagnostics for function return types:

  - Checks that all return statements and the final expression in a function match the declared return type.
  - If a function omits the return type, attempts to infer it from all returns; emits an error if ambiguous.
  - Reports a diagnostic if a function with a non-void return type is missing a return, or if a return type is mismatched.
  - Errors include function name, expected/actual type, and line (where available).

- **Improvements to type inference, error handling, and diagnostics**

  - Variable declarations now attempt type inference if no type is given. If inference fails, a clear error is reported with the variable name.
  - Interpreter and runtime errors now include more descriptive messages.
  - Diagnostics output: errors are printed with context and clear messages.
  - Lays groundwork for further improvements to function return type inference and parameter type checking.

### Patch Changes

- **Fix:** Prevent struct literal parsing in match subject expressions and align parser operator tokens with lexer/AST definitions.

  - The parser now uses a `stop_at_left_brace` flag to prevent struct literal parsing as a match subject in `match` statements.
  - Operator tokens and unary operator variants in the parser are now aligned with the actual enum definitions (`Multiply`, `Divide`, `Modulo`, `Minus`, etc.).
  - All debug output and temporary comments have been removed for a clean, production-ready parser implementation.

  This resolves the long-standing bug where match arm patterns were incorrectly parsed as struct literals, causing pattern matching tests to fail.

- **Parser polish and robustness improvements**

  - `parse_generics`: Robustly parses generic parameter lists, forgiving of trailing/missing commas, and leaves the parser at the next token after the closing `>`.
  - `parse_type`: Handles tuple types, optionals, all built-in/custom types, and generic types with `<...>` or `<<...>>`. Accepts arrays, optionals, and custom types. Forgiving of extra commas/whitespace in tuple and generic type lists.
  - `parse_block`: Cleaned up, all debug print statements removed for a clean final version. Robustly parses a block of statements between `{ ... }`.
  - Top-level parser: Skips stray semicolons and right braces before/after parsing each top-level statement. Never requires a semicolon after a function/struct/enum declaration, but skips one if present.
  - `parse_statement`: Removed all debug output. After parsing a function/struct/enum declaration, an optional semicolon is skipped but never required.
  - `examples/web_request.cyl`: Removed semicolons after `match` blocks to fix parse errors.

  Result: The parser is now robust, forgiving of stray tokens at the top level, and clean of debug output. Real-world Cyl programs now parse and run without errors.

## 0.1.3

### Patch Changes

- Aggressive parser refactor for new Cyl language features:

  - Rewrote generics and declaration parsing logic for functions, structs, enums, and variable declarations.
  - Ensured robust lookahead and token advancement for all forms: identifier = ..., identifier <type> = ..., identifier <T, U> = ..., identifier : type = ...
  - Collected and stored generics/type parameters in AST nodes.
  - Multiple iterations to address persistent test failures and improve parser robustness.

  **Current status:**

  - Some new feature tests are still failing due to deep parser lookahead/advancement issues, especially for identifier-based declarations and generics.
  - Further investigation into token advancement and lookahead logic is required for full test passing.

  This changeset documents the aggressive parser refactor and the ongoing effort to make all new feature tests pass.

- All parser new feature tests now pass:

  - Function argument parsing supports type inference, <type> and :type annotations, and default values.
  - Tuple literals are supported in expressions and return values.
  - All debug output removed for clean CI/test runs.
  - Variable, struct, enum, and function generics and type annotations are robust.
  - Return statements require a semicolon and are parsed correctly.

  This unblocks code coverage and CI for new Cyl language features.

## 0.1.2

### Patch Changes

- All parser new feature tests now pass:
  - Function argument parsing supports type inference, <type> and :type annotations, and default values.
  - Tuple literals are supported in expressions and return values.
  - All debug output removed for clean CI/test runs.
  - Variable, struct, enum, and function generics and type annotations are robust.
  - Return statements require a semicolon and are parsed correctly.
- This unblocks code coverage and CI for new Cyl language features.
