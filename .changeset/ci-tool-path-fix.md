---
"cyl": patch
---

Fix CI failure with TypeScript design tools

Corrected npm script paths for TypeScript design tools. The TypeScript compiler outputs files to `compiler/dist/src/tools/` (preserving the source directory structure), but the npm scripts were looking for them in `compiler/dist/tools/`. Updated the following npm scripts:

- `grammar:validate`: Fixed path to `compiler/dist/src/tools/grammar-validator.js`
- `ast:generate`: Fixed path to `compiler/dist/src/tools/ast-generator.js`
- `syntax:check`: Fixed path to `compiler/dist/src/tools/syntax-checker.js`

This resolves the GitHub Actions CI failure where `npm run grammar:validate` couldn't find the compiled tool files. All design tools now work correctly in both local development and CI environments.
