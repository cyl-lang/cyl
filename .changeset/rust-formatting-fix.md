---
"cyl": patch
---

Fix Rust code formatting for CI compliance

Applied `cargo fmt` to resolve formatting violations that were causing GitHub Actions CI to fail. The formatting changes include:

- Proper line breaking for long function signatures in parser expressions
- Consistent formatting for match arms and block structures  
- Standardized indentation and spacing throughout parser modules
- Removal of unnecessary blank lines and trailing commas

All functionality remains unchanged - 19/19 language tests still pass after formatting.
