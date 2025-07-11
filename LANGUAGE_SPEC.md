# Cyl Language Specification v0.2.0

## Overview

Cyl is a systems and web programming language designed for safety, performance, and developer productivity. It features a multi-backend compilation system that provides flexibility for different use cases, from rapid development to production optimization.

## Design Goals

- **Safety**: Memory safety without garbage collection
- **Performance**: Native compilation with multiple optimization strategies
- **Simplicity**: Clean, readable syntax inspired by TypeScript
- **Flexibility**: Multi-backend compilation (Cranelift, LLVM, Interpreter)
- **Concurrency**: Built-in async/await and safe concurrency primitives
- **Interoperability**: Easy integration with existing systems and libraries
- **Developer Experience**: Excellent tooling, error messages, and debugging support

## Compilation Backends

Cyl provides three compilation backends optimized for different workflows:

### Cranelift Backend (Default)

- **Purpose**: Fast development compilation
- **Implementation**: Pure Rust code generation
- **Output**: Native object files and executables
- **Dependencies**: None (self-contained)
- **Use Cases**: Development, CI/CD, rapid iteration

### LLVM Backend (Optional)

- **Purpose**: Optimized production compilation
- **Implementation**: LLVM IR generation with optimization passes
- **Output**: Highly optimized native code
- **Dependencies**: LLVM development libraries
- **Use Cases**: Production builds, performance-critical applications

### Interpreter Backend

- **Purpose**: Immediate execution and development
- **Implementation**: Direct AST interpretation
- **Output**: Real-time program execution
- **Dependencies**: None
- **Use Cases**: Education, testing, rapid prototyping, debugging

## Language Features

### Core Language

#### Variables and Types

```cyl
// Variable declarations
let x = 5;                    // Immutable by default
let mut y = 10;               // Mutable variable
const PI: float = 3.14159;    // Compile-time constant

// Type annotations (optional with inference)
let name: string = "Cyl";
let numbers: Array<int> = [1, 2, 3, 4, 5];
```

#### Primitive Types

- `int` - 64-bit signed integer
- `float` - 64-bit floating point
- `string` - UTF-8 encoded strings
- `bool` - Boolean values (true/false)
- `char` - Unicode character
- `void` - No value (unit type)

#### Functions

```cyl
// Basic function
fn greet(name: string) -> void {
    print("Hello, " + name + "!");
}

// Function with return value
fn add(a: int, b: int) -> int {
    return a + b;
}

// Async function
async fn fetchData(url: string) -> string {
    let response = await net.get(url);
    return response.body;
}
```

#### Control Flow

```cyl
// Conditional statements
if x > 0 {
    os.print("Positive");
} else if x < 0 {
    os.print("Negative");
} else {
    os.print("Zero");
}

// While loops
while x > 0 {
    x = x - 1;
}

// For loops
for item in items {
    process(item);
}

for i in 0..10 {
    os.print(i);
}
```

#### Pattern Matching

```cyl
enum Result<T, E> {
    Ok(T),
    Err(E)
}

match result {
    Ok(value) => {
        os.print("Success: " + value);
    },
    Err(error) => {
        os.print("Error: " + error);
    }
}
```

### Data Types

#### Structures

```cyl
struct Person {
    name: string,
    age: int,
    email: string
}

struct Point<T> {
    x: T,
    y: T
}

// Usage
let person = Person {
    name: "Alice",
    age: 30,
    email: "alice@example.com"
};

let point: Point<float> = Point { x: 1.5, y: 2.7 };
```

#### Enumerations

```cyl
enum Color {
    Red,
    Green,
    Blue,
    RGB(int, int, int)
}

enum HttpMethod {
    Get,
    Post,
    Put,
    Delete
}
```

#### Collections

```cyl
// Arrays
let numbers: Array<int> = [1, 2, 3, 4, 5];
let first = numbers[0];

// Hash maps
let scores: HashMap<string, int> = {
    "Alice": 100,
    "Bob": 85,
    "Charlie": 92
};
```

### Memory Management

Cyl uses a ownership system similar to Rust for memory safety:

```cyl
// Ownership transfer
let s1 = "Hello";
let s2 = s1;  // s1 is no longer valid

// Borrowing (references)
fn length(s: &string) -> int {
    return s.len();
}

let text = "Hello, World!";
let len = length(&text);  // text is still valid
```

### Modules and Imports

```cyl
// Import entire module
import net;
import os;

// Selective imports
import fs { read, write, exists };
import collections { HashMap, Array };

// Using imported functions
let response = net.get("https://api.example.com");
os.print(response.body);

let content = fs.read("/path/to/file.txt");
```

### Error Handling

```cyl
// Result type for error handling
fn divide(a: int, b: int) -> Result<float, string> {
    if b == 0 {
        return Err("Division by zero");
    }
    return Ok(a / b);
}

// Using try/catch for exception handling
try {
    let result = risky_operation();
    os.print("Success: " + result);
} catch error: NetworkError {
    os.print("Network error: " + error.message);
} catch error {
    os.print("Unknown error: " + error);
}
```

### Concurrency and Async

```cyl
// Async functions
async fn fetchMultiple(urls: Array<string>) -> Array<string> {
    let results: Array<string> = [];

    for url in urls {
        let response = await net.get(url);
        results.push(response.body);
    }

    return results;
}

// Spawning async tasks
async fn main() {
    let task1 = spawn async { expensive_computation1() };
    let task2 = spawn async { expensive_computation2() };

    let result1 = await task1;
    let result2 = await task2;

    os.print("Results: " + result1 + ", " + result2);
}
```

## Standard Library

### Core Modules

#### `os` - Operating System Interface

```cyl
import os;

os.print(message: string) -> void;           // Print to stdout
os.println(message: string) -> void;         // Print line to stdout
os.exit(code: int) -> void;                  // Exit program
os.args() -> Array<string>;                  // Command line arguments
os.env(key: string) -> Option<string>;       // Environment variable
```

#### `net` - Network Operations

```cyl
import net;

struct HttpResponse {
    status: int,
    headers: HashMap<string, string>,
    body: string
}

net.get(url: string) -> HttpResponse;
net.post(url: string, data: string) -> HttpResponse;
net.put(url: string, data: string) -> HttpResponse;
net.delete(url: string) -> HttpResponse;
```

#### `fs` - File System Operations

```cyl
import fs;

fs.read(path: string) -> Result<string, string>;
fs.write(path: string, content: string) -> Result<void, string>;
fs.exists(path: string) -> bool;
fs.copy(src: string, dst: string) -> Result<void, string>;
fs.remove(path: string) -> Result<void, string>;
```

#### `json` - JSON Processing

```cyl
import json;

json.parse(text: string) -> Result<JsonValue, string>;
json.stringify(value: JsonValue) -> string;
```

#### `time` - Time and Date Operations

```cyl
import time;

struct DateTime {
    year: int,
    month: int,
    day: int,
    hour: int,
    minute: int,
    second: int
}

time.now() -> DateTime;
time.sleep(milliseconds: int) -> void;
time.format(dt: DateTime, pattern: string) -> string;
```

## Syntax Reference

### Keywords

| Keyword    | Purpose                    | Example                              |
| ---------- | -------------------------- | ------------------------------------ |
| `fn`       | Function declaration       | `fn main() -> void {}`               |
| `let`      | Variable declaration       | `let x = 5;`                         |
| `const`    | Constant declaration       | `const PI = 3.14159;`                |
| `mut`      | Mutability modifier        | `let mut x = 5;`                     |
| `if`       | Conditional statement      | `if x > 0 { ... }`                   |
| `else`     | Alternative branch         | `if x > 0 { ... } else { ... }`      |
| `while`    | While loop                 | `while x > 0 { ... }`                |
| `for`      | For loop                   | `for item in items { ... }`          |
| `match`    | Pattern matching           | `match value { ... }`                |
| `return`   | Return from function       | `return 42;`                         |
| `break`    | Break from loop            | `break;`                             |
| `continue` | Continue loop iteration    | `continue;`                          |
| `struct`   | Structure definition       | `struct Point { x: int, y: int }`    |
| `enum`     | Enumeration definition     | `enum Color { Red, Green, Blue }`    |
| `import`   | Module import              | `import net;`                        |
| `async`    | Async function declaration | `async fn fetch() -> string { ... }` |
| `await`    | Await async operation      | `let result = await operation();`    |
| `try`      | Exception handling block   | `try { ... } catch { ... }`          |
| `catch`    | Exception catching         | `catch error { ... }`                |
| `throw`    | Throw exception            | `throw "Error message";`             |

### Operators

#### Arithmetic

- `+` - Addition
- `-` - Subtraction
- `*` - Multiplication
- `/` - Division
- `%` - Modulo

#### Comparison

- `==` - Equality
- `!=` - Inequality
- `<` - Less than
- `<=` - Less than or equal
- `>` - Greater than
- `>=` - Greater than or equal

#### Logical

- `&&` - Logical AND
- `||` - Logical OR
- `!` - Logical NOT

#### Bitwise

- `&` - Bitwise AND
- `|` - Bitwise OR
- `^` - Bitwise XOR
- `<<` - Left shift
- `>>` - Right shift
- `~` - Bitwise NOT

#### Assignment

- `=` - Assignment

#### Special

- `->` - Return type indicator
- `.` - Member access
- `?` - Optional chaining (future feature)

### Comments

```cyl
// Single line comment

/*
   Multi-line
   comment
*/

/// Documentation comment
fn documented_function() {
    // Implementation
}
```

## Compilation and Execution

### Compiler Usage

```bash
# Compile and run
cylc run main.cyl

# Compile to executable
cylc build main.cyl -o myprogram

# Check syntax
cylc check main.cyl

# Show AST
cylc ast main.cyl --format=pretty
```

### Project Structure

```
my_project/
├── main.cyl          # Entry point
├── lib/             # Library modules
│   ├── utils.cyl
│   └── models.cyl
├── tests/           # Test files
│   └── main_test.cyl
└── Cyl.toml         # Project configuration
```

### Configuration File (Cyl.toml)

```toml
[package]
name = "my_project"
version = "0.1.0"
authors = ["Your Name <email@example.com>"]

[dependencies]
http_client = "1.0.0"
json_parser = "2.1.0"

[build]
optimization = "release"
target = "native"
```

## Advanced Features

### Generics

```cyl
// Generic function
fn swap<T>(a: T, b: T) -> (T, T) {
    return (b, a);
}

// Generic struct
struct Container<T> {
    value: T
}

// Generic enum
enum Option<T> {
    Some(T),
    None
}
```

### Traits (Interfaces)

```cyl
trait Display {
    fn display(self) -> string;
}

trait Comparable<T> {
    fn compare(self, other: T) -> int;
}

struct Person {
    name: string,
    age: int
}

impl Display for Person {
    fn display(self) -> string {
        return self.name + " (" + self.age + ")";
    }
}
```

### Macros

```cyl
// Simple macro
macro_rules! debug_print {
    ($expr:expr) => {
        os.println("DEBUG: " + stringify!($expr) + " = " + $expr);
    };
}

// Usage
debug_print!(x + y);
```

## Future Features

- Package manager and registry
- WebAssembly compilation target
- Foreign Function Interface (FFI)
- Reflection and metaprogramming
- Advanced pattern matching
- Algebraic data types
- Actor-based concurrency model

## Version History

- **v0.1.0**: Initial specification with basic language features
  - Variables, functions, control flow
  - Basic types and collections
  - Module system
  - Async/await support
  - Error handling with Result types

---

This specification is a living document and will evolve as the Cyl language develops.
