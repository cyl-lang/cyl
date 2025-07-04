# cyl `.cyl`

- [Implementation Plan](IMPLEMENTATION_PLAN.md)
- [Language Specification](LANGUAGE_SPEC.md)
- [Tests](tests/README.md)

**Purpose:** Systems & web programming with simple syntax, native compilation, safe concurrency, OS & network integration

**Paradigm:** Imperative, concurrent, safe systems language with modern syntax inspired by TypeScript/Python and safety from Rust/Go

```cyl
// main.cyl
import net
import os

fn main() -> void {
    res = net.get("https://example.com");

    if res.status == 200 {
        os.fs.write("/tmp/response.txt", res.body)
        os.print("Response saved.")
    } else {
        os.print("Failed to fetch data.")
    }
};
```

---

## Usage

### Installation

```bash
git clone https://github.com/clxrityy/cyl.git
cd cyl
make install
```

### Commands

```bash
# If installed globally
cylc check file.cyl
cylc ast file.cyl
cylc test

# If not installed, use the full path
./target/release/cylc check file.cyl
./target/release/cylc test
```

### Current Implementation Status

**Completed:**

- [x] Lexer with token recognition
- [x] Parser for basic syntax (functions, imports, structs, variables)
- [x] AST generation
- [x] Automated test system
- [x] CLI with syntax checking

**In Progress:**

- [ ] Code generation (LLVM backend stubbed)
- [ ] Standard library integration
- [ ] Advanced language features

---

## Key Syntax Features

| Syntax     | Type                     |
| ---------- | ------------------------ |
| `fn`       | FunctionDeclaration      |
| `if`       | IfStatement              |
| `else`     | ElseStatement            |
| `import`   | ImportStatement          |
| `=`        | AssignmentExpression     |
| `.`        | MemberExpression         |
| `->`       | ReturnType               |
| `;`        | StatementTerminator      |
| `"`        | StringLiteral            |
| `()`       | CallExpression           |
| `[]`       | ArrayLiteral             |
| `{}`       | BlockStatement           |
| `,`        | ParameterSeparator       |
| `:`        | KeyValueSeparator        |
| `void`     | VoidType                 |
| `declare`  | DeclareStatement         |
| `<>`       | TypeParameter            |
| `return`   | ReturnStatement          |
| `struct`   | StructDeclaration        |
| `enum`     | EnumDeclaration          |
| `match`    | MatchStatement           |
| `for`      | ForStatement             |
| `while`    | WhileStatement           |
| `break`    | BreakStatement           |
| `continue` | ContinueStatement        |
| `try`      | TryStatement             |
| `catch`    | CatchStatement           |
| `throw`    | ThrowStatement           |
| `async`    | AsyncFunctionDeclaration |
| `await`    | AwaitExpression          |
