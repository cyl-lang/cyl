# Cyl Standard Library Documentation

## Overview

The Cyl standard library provides essential functionality for systems programming, web development, and general-purpose computing. It is designed to be safe, efficient, and easy to use while maintaining the language's core principles of memory safety and zero-cost abstractions.

## Core Modules

### `os` - Operating System Interface

The `os` module provides fundamental operating system interaction capabilities.

```cyl
import os;
```

#### Functions

##### `print(message: string) -> void`

Prints a message to standard output without a trailing newline.

```cyl
os.print("Hello");
os.print(" ");
os.print("World");
// Output: Hello World
```

##### `println(message: string) -> void`

Prints a message to standard output with a trailing newline.

```cyl
os.println("Hello, World!");
os.println("Second line");
// Output:
// Hello, World!
// Second line
```

##### `exit(code: int) -> void`

Terminates the program with the specified exit code.

```cyl
if error_occurred {
    os.exit(1);  // Exit with error code
}
os.exit(0);      // Normal exit
```

##### `args() -> Array<string>`

Returns command-line arguments passed to the program.

```cyl
let arguments = os.args();
if arguments.len() > 1 {
    os.println("First argument: " + arguments[1]);
}
```

##### `env(key: string) -> Option<string>`

Retrieves an environment variable value.

```cyl
match os.env("HOME") {
    Some(home_dir) => os.println("Home: " + home_dir),
    None => os.println("HOME not set")
}
```

##### `cwd() -> Result<string, string>`

Gets the current working directory.

```cyl
match os.cwd() {
    Ok(dir) => os.println("Current directory: " + dir),
    Err(error) => os.println("Error: " + error)
}
```

---

### `fs` - File System Operations

The `fs` module provides file and directory manipulation capabilities.

```cyl
import fs;
```

#### Functions

##### `read(path: string) -> Result<string, string>`

Reads the entire contents of a file as a string.

```cyl
match fs.read("config.txt") {
    Ok(content) => os.println("File content: " + content),
    Err(error) => os.println("Failed to read file: " + error)
}
```

##### `write(path: string, content: string) -> Result<void, string>`

Writes content to a file, creating it if it doesn't exist.

```cyl
let data = "Hello, file system!";
match fs.write("output.txt", data) {
    Ok(_) => os.println("File written successfully"),
    Err(error) => os.println("Write failed: " + error)
}
```

##### `append(path: string, content: string) -> Result<void, string>`

Appends content to the end of a file.

```cyl
match fs.append("log.txt", "New log entry\n") {
    Ok(_) => os.println("Log entry added"),
    Err(error) => os.println("Append failed: " + error)
}
```

##### `exists(path: string) -> bool`

Checks if a file or directory exists.

```cyl
if fs.exists("important.txt") {
    os.println("File exists");
} else {
    os.println("File not found");
}
```

##### `copy(src: string, dst: string) -> Result<void, string>`

Copies a file from source to destination.

```cyl
match fs.copy("original.txt", "backup.txt") {
    Ok(_) => os.println("File copied"),
    Err(error) => os.println("Copy failed: " + error)
}
```

##### `remove(path: string) -> Result<void, string>`

Removes a file or empty directory.

```cyl
match fs.remove("temp.txt") {
    Ok(_) => os.println("File removed"),
    Err(error) => os.println("Remove failed: " + error)
}
```

##### `mkdir(path: string) -> Result<void, string>`

Creates a directory.

```cyl
match fs.mkdir("new_directory") {
    Ok(_) => os.println("Directory created"),
    Err(error) => os.println("mkdir failed: " + error)
}
```

##### `list_dir(path: string) -> Result<Array<string>, string>`

Lists the contents of a directory.

```cyl
match fs.list_dir(".") {
    Ok(entries) => {
        for entry in entries {
            os.println(entry);
        }
    },
    Err(error) => os.println("List failed: " + error)
}
```

---

### `net` - Network Operations

The `net` module provides HTTP client functionality for web interactions.

```cyl
import net;
```

#### Types

##### `HttpResponse`

Represents an HTTP response.

```cyl
struct HttpResponse {
    status: int,
    headers: HashMap<string, string>,
    body: string
}
```

##### `HttpMethod`

HTTP request methods.

```cyl
enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options
}
```

#### Functions

##### `get(url: string) -> Result<HttpResponse, string>`

Performs an HTTP GET request.

```cyl
match net.get("https://api.example.com/data") {
    Ok(response) => {
        if response.status == 200 {
            os.println("Data: " + response.body);
        } else {
            os.println("HTTP error: " + response.status);
        }
    },
    Err(error) => os.println("Request failed: " + error)
}
```

##### `post(url: string, data: string) -> Result<HttpResponse, string>`

Performs an HTTP POST request with data.

```cyl
let json_data = "{\"name\":\"John\",\"age\":30}";
match net.post("https://api.example.com/users", json_data) {
    Ok(response) => os.println("Created user: " + response.body),
    Err(error) => os.println("POST failed: " + error)
}
```

##### `put(url: string, data: string) -> Result<HttpResponse, string>`

Performs an HTTP PUT request.

```cyl
let update_data = "{\"age\":31}";
match net.put("https://api.example.com/users/123", update_data) {
    Ok(response) => os.println("Updated: " + response.body),
    Err(error) => os.println("PUT failed: " + error)
}
```

##### `delete(url: string) -> Result<HttpResponse, string>`

Performs an HTTP DELETE request.

```cyl
match net.delete("https://api.example.com/users/123") {
    Ok(response) => os.println("Deleted successfully"),
    Err(error) => os.println("DELETE failed: " + error)
}
```

##### `request(method: HttpMethod, url: string, data: Option<string>) -> Result<HttpResponse, string>`

Performs a generic HTTP request.

```cyl
let response = net.request(HttpMethod.Patch, "https://api.example.com/users/123", Some("{\"active\":false}"));
```

---

### `json` - JSON Processing

The `json` module provides JSON parsing and serialization capabilities.

```cyl
import json;
```

#### Types

##### `JsonValue`

Represents any JSON value.

```cyl
enum JsonValue {
    Null,
    Bool(bool),
    Number(float),
    String(string),
    Array(Array<JsonValue>),
    Object(HashMap<string, JsonValue>)
}
```

#### Functions

##### `parse(text: string) -> Result<JsonValue, string>`

Parses a JSON string into a JsonValue.

```cyl
let json_text = "{\"name\":\"Alice\",\"age\":25,\"active\":true}";
match json.parse(json_text) {
    Ok(value) => {
        // Access JSON data
        match value {
            JsonValue.Object(obj) => {
                if let Some(JsonValue.String(name)) = obj.get("name") {
                    os.println("Name: " + name);
                }
            },
            _ => os.println("Expected JSON object")
        }
    },
    Err(error) => os.println("JSON parse error: " + error)
}
```

##### `stringify(value: JsonValue) -> string`

Converts a JsonValue to a JSON string.

```cyl
let mut obj = HashMap::new();
obj.insert("name", JsonValue.String("Bob"));
obj.insert("age", JsonValue.Number(30.0));

let json_value = JsonValue.Object(obj);
let json_string = json.stringify(json_value);
os.println("JSON: " + json_string);
```

##### `parse_object(text: string) -> Result<HashMap<string, JsonValue>, string>`

Parses JSON text specifically as an object.

```cyl
match json.parse_object("{\"key\":\"value\"}") {
    Ok(obj) => {
        if let Some(value) = obj.get("key") {
            os.println("Found value");
        }
    },
    Err(error) => os.println("Parse error: " + error)
}
```

---

### `time` - Time and Date Operations

The `time` module provides time-related functionality.

```cyl
import time;
```

#### Types

##### `DateTime`

Represents a date and time.

```cyl
struct DateTime {
    year: int,
    month: int,
    day: int,
    hour: int,
    minute: int,
    second: int,
    millisecond: int
}
```

##### `Duration`

Represents a time duration.

```cyl
struct Duration {
    milliseconds: int
}
```

#### Functions

##### `now() -> DateTime`

Gets the current date and time.

```cyl
let current = time.now();
os.println("Current year: " + current.year);
```

##### `sleep(duration: Duration) -> void`

Pauses execution for the specified duration.

```cyl
os.println("Starting...");
time.sleep(Duration { milliseconds: 1000 });  // Sleep for 1 second
os.println("Done!");
```

##### `format(dt: DateTime, pattern: string) -> string`

Formats a DateTime according to a pattern.

```cyl
let now = time.now();
let formatted = time.format(now, "YYYY-MM-DD HH:mm:ss");
os.println("Formatted time: " + formatted);
```

##### `parse(text: string, pattern: string) -> Result<DateTime, string>`

Parses a date/time string according to a pattern.

```cyl
match time.parse("2023-12-25 15:30:00", "YYYY-MM-DD HH:mm:ss") {
    Ok(dt) => os.println("Parsed date: " + dt.year),
    Err(error) => os.println("Parse error: " + error)
}
```

##### `elapsed(start: DateTime, end: DateTime) -> Duration`

Calculates the duration between two times.

```cyl
let start = time.now();
// ... do some work ...
let end = time.now();
let elapsed = time.elapsed(start, end);
os.println("Elapsed: " + elapsed.milliseconds + "ms");
```

---

### `collections` - Data Structures

The `collections` module provides additional data structures beyond basic arrays.

```cyl
import collections;
```

#### Types

##### `HashMap<K, V>`

A hash map (dictionary) with key-value pairs.

```cyl
let mut scores = HashMap<string, int>::new();
scores.insert("Alice", 100);
scores.insert("Bob", 85);

match scores.get("Alice") {
    Some(score) => os.println("Alice's score: " + score),
    None => os.println("Alice not found")
}
```

##### `HashSet<T>`

A hash set containing unique values.

```cyl
let mut unique_items = HashSet<string>::new();
unique_items.insert("apple");
unique_items.insert("banana");
unique_items.insert("apple");  // Duplicate, won't be added

os.println("Set size: " + unique_items.len());  // Output: 2
```

##### `Vec<T>` (Alternative to Array)

A growable array implementation.

```cyl
let mut numbers = Vec<int>::new();
numbers.push(1);
numbers.push(2);
numbers.push(3);

for number in numbers {
    os.println("Number: " + number);
}
```

---

### `math` - Mathematical Operations

The `math` module provides mathematical functions and constants.

```cyl
import math;
```

#### Constants

```cyl
const PI: float = 3.141592653589793;
const E: float = 2.718281828459045;
```

#### Functions

##### Trigonometric Functions

```cyl
math.sin(x: float) -> float;
math.cos(x: float) -> float;
math.tan(x: float) -> float;
math.asin(x: float) -> float;
math.acos(x: float) -> float;
math.atan(x: float) -> float;
```

##### Exponential and Logarithmic

```cyl
math.exp(x: float) -> float;       // e^x
math.log(x: float) -> float;       // Natural logarithm
math.log10(x: float) -> float;     // Base-10 logarithm
math.pow(base: float, exp: float) -> float;
math.sqrt(x: float) -> float;
```

##### Utility Functions

```cyl
math.abs(x: float) -> float;
math.ceil(x: float) -> float;
math.floor(x: float) -> float;
math.round(x: float) -> float;
math.min(a: float, b: float) -> float;
math.max(a: float, b: float) -> float;
```

---

### `random` - Random Number Generation

The `random` module provides random number generation capabilities.

```cyl
import random;
```

#### Functions

##### `seed(value: int) -> void`

Seeds the random number generator.

```cyl
random.seed(12345);
```

##### `int() -> int`

Generates a random integer.

```cyl
let random_number = random.int();
```

##### `int_range(min: int, max: int) -> int`

Generates a random integer within a range (inclusive).

```cyl
let dice_roll = random.int_range(1, 6);
os.println("Rolled: " + dice_roll);
```

##### `float() -> float`

Generates a random float between 0.0 and 1.0.

```cyl
let probability = random.float();
```

##### `bool() -> bool`

Generates a random boolean.

```cyl
if random.bool() {
    os.println("Heads");
} else {
    os.println("Tails");
}
```

##### `choice<T>(items: Array<T>) -> Option<T>`

Randomly selects an item from an array.

```cyl
let colors = ["red", "green", "blue"];
match random.choice(colors) {
    Some(color) => os.println("Selected: " + color),
    None => os.println("Empty array")
}
```

---

### `string` - String Manipulation

The `string` module provides additional string processing functions.

```cyl
import string;
```

#### Functions

##### `split(text: string, delimiter: string) -> Array<string>`

Splits a string by a delimiter.

```cyl
let parts = string.split("apple,banana,cherry", ",");
for part in parts {
    os.println("Part: " + part);
}
```

##### `join(parts: Array<string>, delimiter: string) -> string`

Joins an array of strings with a delimiter.

```cyl
let words = ["Hello", "beautiful", "world"];
let sentence = string.join(words, " ");
os.println(sentence);  // "Hello beautiful world"
```

##### `trim(text: string) -> string`

Removes whitespace from both ends of a string.

```cyl
let cleaned = string.trim("  hello world  ");
os.println("'" + cleaned + "'");  // "'hello world'"
```

##### `replace(text: string, old: string, new: string) -> string`

Replaces all occurrences of a substring.

```cyl
let updated = string.replace("Hello World", "World", "Cyl");
os.println(updated);  // "Hello Cyl"
```

##### `to_upper(text: string) -> string`

Converts string to uppercase.

```cyl
let upper = string.to_upper("hello");
os.println(upper);  // "HELLO"
```

##### `to_lower(text: string) -> string`

Converts string to lowercase.

```cyl
let lower = string.to_lower("HELLO");
os.println(lower);  // "hello"
```

##### `contains(text: string, substring: string) -> bool`

Checks if string contains a substring.

```cyl
if string.contains("Hello World", "World") {
    os.println("Found 'World'");
}
```

##### `starts_with(text: string, prefix: string) -> bool`

Checks if string starts with a prefix.

```cyl
if string.starts_with("Hello World", "Hello") {
    os.println("Starts with 'Hello'");
}
```

##### `ends_with(text: string, suffix: string) -> bool`

Checks if string ends with a suffix.

```cyl
if string.ends_with("hello.txt", ".txt") {
    os.println("It's a text file");
}
```

---

## Usage Examples

### Simple Web Request

```cyl
import net;
import json;
import os;

async fn fetch_user_data(id: int) -> Result<string, string> {
    let url = "https://jsonplaceholder.typicode.com/users/" + id;
    let response = await net.get(url)?;

    if response.status == 200 {
        return Ok(response.body);
    } else {
        return Err("Failed to fetch user data");
    }
}

async fn main() {
    match await fetch_user_data(1) {
        Ok(data) => {
            match json.parse(data) {
                Ok(user) => os.println("User data retrieved successfully"),
                Err(error) => os.println("JSON parse error: " + error)
            }
        },
        Err(error) => os.println("Request error: " + error)
    }
}
```

### File Processing

```cyl
import fs;
import string;
import os;

fn process_config_file(path: string) -> Result<HashMap<string, string>, string> {
    let content = fs.read(path)?;
    let mut config = HashMap<string, string>::new();

    let lines = string.split(content, "\n");
    for line in lines {
        let trimmed = string.trim(line);
        if !string.starts_with(trimmed, "#") && string.contains(trimmed, "=") {
            let parts = string.split(trimmed, "=");
            if parts.len() == 2 {
                config.insert(string.trim(parts[0]), string.trim(parts[1]));
            }
        }
    }

    return Ok(config);
}

fn main() {
    match process_config_file("app.conf") {
        Ok(config) => {
            for (key, value) in config {
                os.println(key + " = " + value);
            }
        },
        Err(error) => os.println("Config error: " + error)
    }
}
```

### Mathematical Calculations

```cyl
import math;
import os;

fn calculate_circle_properties(radius: float) {
    let area = math.PI * math.pow(radius, 2.0);
    let circumference = 2.0 * math.PI * radius;

    os.println("Circle with radius " + radius + ":");
    os.println("  Area: " + area);
    os.println("  Circumference: " + circumference);
}

fn main() {
    calculate_circle_properties(5.0);
    calculate_circle_properties(10.0);
}
```

## Future Additions

The standard library will continue to evolve with additional modules planned for:

- **crypto**: Cryptographic functions and utilities
- **regex**: Regular expression support
- **xml**: XML parsing and generation
- **csv**: CSV file processing
- **zip**: Archive compression and extraction
- **image**: Basic image processing
- **sql**: Database connectivity
- **uuid**: UUID generation
- **base64**: Base64 encoding/decoding
- **url**: URL parsing and manipulation

---

This documentation represents the planned standard library for Cyl v0.1.0 and will be updated as the implementation progresses.
