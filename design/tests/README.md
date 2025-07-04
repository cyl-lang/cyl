# Design Tests

This directory contains tests for the Cyl language design tools and TypeScript components.

## Test Files

- `grammar.test.ts` - Tests for grammar loading, saving, and default grammar structure
- `grammar-validator.test.ts` - Tests for the grammar validation system
- `index.test.ts` - Integration tests for the main CylLanguageDesign class

## Running Tests

```bash
# Run all design tests
npm run test:design

# Run tests in watch mode
npm run test:design:watch

# Run tests with coverage
npm run test:design:coverage

# Run both design and compiler tests
npm test
```

## Test Structure

Tests use Jest with TypeScript support and cover:

- Grammar loading and validation
- AST generation
- Syntax checking
- Error handling
- TypeScript type safety

## Mock Dependencies

- `__mocks__/chalk.ts` - Mock for chalk styling library to avoid ES module issues

## Current Test Coverage

- ✅ Grammar module functionality
- ✅ Grammar validator
- ✅ Language design tool integration
- ✅ Error handling and edge cases
