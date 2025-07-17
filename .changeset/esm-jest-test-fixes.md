---
"cyl": patch
---

- Updated test implementations in `design/tests` to ensure compatibility with ESM, TypeScript, and Jest. 
- Fixed import paths in test files to use `.js` extensions and reference built output where necessary.
- Removed duplicate and non-top-level import statements in test files.
- Cleaned up Jest configuration to avoid over-broad module mapping and ensure robust ESM+TypeScript support.
- Ensured all tests run against the built output and pass in CI workflows.
- No changes to the public API, but internal test and build reliability is significantly improved.
