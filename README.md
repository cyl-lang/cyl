<!--
  Cyl: Universal Language Engine
  =============================
-->

# Cyl

**Cyl** is a universal, embeddable language engine and programmable meta-runtime. It is designed to let you:

- Inject, extend, or override language syntax and semantics in any host language (Python, JS, Rust, etc.)
- Register new syntax, macros, or runtime hooks
- Load plugins/add-ons to customize language behavior
- Use a project-based config (`config.cyl`) to declaratively control your environment
- Integrate with any language via adapters

---

## Quick Start

1. **Install Cyl CLI** (coming soon)
2. **Create a `config.cyl` in your project root**
3. **Run, build, or customize your code with the CLI**

---

## Project Structure

- `cli/` — CLI entrypoint and command dispatch
- `engine/` — Core engine logic and API (syntax, plugins, events, adapters)
- `plugins/` — Built-in and user plugins
- `adapters/` — Host language integration (Python, JS, etc.)
- `docs/` — Documentation system logic
- `config.cyl` — Project configuration (syntax, plugins, events, engine settings)

---

## Example: config.cyl

```cyl
declare config {
    syntax: [
        { operator: "|>", transform: (left, right) => `${right}(${left})` }
    ],
    plugins: [
        "./plugins/log_calls.cyl"
    ],
    events: {
        function_call: (fn, args) => println("Called", fn, args)
    },
    engine: {
        backend: "cranelift",
        optimize: true
    },
    language: {
        host: "python",
        version: "3.12"
    }
}
```

---

## CLI Usage (planned)

- `cyl run <file>` — Run a file with all config, plugins, and customizations applied
- `cyl build <file>` — Build code to native, bytecode, or transformed source
- `cyl check` — Validate `config.cyl` and all plugins
- `cyl plugins` — Manage plugins
- `cyl ast <file>` — Print the transformed AST after config/plugins
- `cyl config` — Manage or validate the current config

---

## Plugin Example

```rust
// plugins/log_calls.rs
pub struct LogCallsPlugin;

impl LogCallsPlugin {
    pub fn on_function_call(&self, fn_name: &str, args: &[Value]) {
        println!("[Cyl] {} called with {:?}", fn_name, args);
    }
}
```

---

## Adapter Example (Python)

```python
import cyl

engine = cyl.Engine()
engine.register_syntax("|>", lambda left, right: f"{right}({left})")
engine.on_event("function_call", lambda fn, args: print(f"[Cyl] {fn} called with {args}"))
engine.run("result = 5 |> str")
```

---

### Status

See [changelog](CHANGELOG.md).
