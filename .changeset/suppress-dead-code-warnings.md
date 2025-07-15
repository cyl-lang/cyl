---
'cyl': patch
---

Suppress dead code warnings for unused fields and methods in interpreter and stdlib modules.

- Added #[allow(dead_code)] to unused struct fields and trait methods (Interpreter, StdLibWrapper, StdLib, ModuleTrait)
- Ensures builds and tests pass with -D warnings
- Improves CI reliability and developer experience
- No runtime or API changes

This is a maintenance update to support strict warning policies and future-proof the codebase against unused code errors.
