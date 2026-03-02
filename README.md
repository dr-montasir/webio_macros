# 🦅 webio_macros

**Procedural macros for the WebIO ultra-low-latency framework.**

`webio_macros` provides high-level attribute sugar for the [WebIO](https://crates.io/crates/webio) ecosystem. Its primary goal is to provide a clean developer experience without introducing any external dependencies or runtime overhead by default.

By acting as a **compile-time bridge**, `webio_macros` enables the definition of high-performance I/O operations that remain backend-agnostic. It functions as a compile-time code generator, allowing the **WebIO** framework to support high-performance external clients like `http` (powered by `ureq`) with zero boilerplate.

**Note**: To connect with external **APIs** via **HTTP**, `webio_macros` offers an optional feature that wraps the lightweight ureq crate as `http`. This feature is only included when explicitly enabled, ensuring the core remains dependency-free.

### Http Usage Example:
```rust
use webio_macros::http;
use serde_json;

fn example() {
    // Simple GET request
    let res = http!(get("https://httpbin.org").call());
    
    // POST request with headers and JSON payload
    let payload = serde_json::json!({ "id": 1 });
    let res = http! {
        post("https://httpbin.org")
            .header("Authorization", "Bearer TOKEN")
            .send_json(payload)
    };
}
```

## Key Features
- **The Entry Point**: `#[webio_main]` transforms async entry points into high-efficiency 
  execution units managed by the WebIO engine.
- **Template Engine**: `replace!` and `html!` provide zero-dependency string substitution 
  at the compilation phase, optimized for raw string literals and web content.

## 🛠 Installation

Run the following Cargo `command` in your project directory:

```shell
cargo add webio webio_macros
```

Or add `webio` and `webio_macros` as a dependencies in your `Cargo.toml`:

```toml
[dependencies]
webio = "MAJOR.MINOR.PATCH" # Replace with the latest version
webio_macros = "MAJOR.MINOR.PATCH" # Replace with the latest version
```

## 🚀 The Entry Point

The flagship macro `#[webio_main]` enables the definition of an application entry point using standard `async fn main()` syntax.

### Why use `#[webio_main]`?

1. **Zero-Dependency Philosophy**: Written using only the built-in `proc_macro` library.
2. **Boilerplate Reduction**: Automatically handles the transition from synchronous OS threads to the WebIO async world.
3. **Turbo Performance**: Wraps code in the `webio::block_on` engine, maintaining **70µs - 400µs** response times.

## ⚡ High-Speed Templates

WebIO includes a built-in, zero-dependency template engine via the `replace!` and `html!` macros. These perform efficient string transformations during compilation.

### `replace!` & `html!`

The `replace!` macro is a versatile tool for substituting `{{key}}` placeholders in any content. The `html!` macro acts as a semantic alias for web-specific development.

- **Efficiency**: No regex or heavy parsers; uses optimized string replacement.
- **Raw String Support**: Works perfectly with `r#""#` for embedding complex code or HTML.
- **Extensible**: Designed so developers can wrap it in their own `macro_rules!` (e.g., `css!`, `sql!`).

#### Example:
```rust
use webio_macros::{replace, html};

let user = "Ahmed";
// Using the core replace engine
let msg = replace!("Hello {{name}}", name = user);

// Using the semantic HTML alias
let view = html!(r#"<div class="user">{{name}}</div>"#, name = user);
```

## 📖 Usage

```rust,no_run
use webio::*;
use webio_macros::{webio_main, html};

#[webio_main]
async fn main() {
    let mut app = WebIo::new();

    app.route(GET, "/", |_, _| async {
        let content = html!("<h1>Hello from 🦅 {{name}}!</h1>", name = "WebIO");
        Reply::new(StatusCode::Ok)
            .header("Content-Type", "text/html; charset=UTF-8")
            .body(content)
    });

    app.run("127.0.0.1", "8080");
}
```

## ⚖️ License

Licensed under the MIT License. Built for performance.