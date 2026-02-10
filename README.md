# 🦅 webio_macros

**Procedural macros for the WebIO ultra-low-latency framework.**

`webio_macros` provides the high-level attribute sugar for the [WebIO](https://crates.io/crates/webio) ecosystem. Its primary goal is to provide a clean developer experience without introducing any external dependencies or runtime overhead.

## 🚀 The Entry Point

The flagship macro `#[webio_main]` enables the definition of an application entry point using standard `async fn main()` syntax.

### Why use `#[webio_main]`?

1. **Zero-Dependency Philosophy**: Written using only the built-in `proc_macro` library.
2. **Boilerplate Reduction**: Automatically handles the transition from synchronous OS threads to the WebIO async world.
3. **Turbo Performance**: Wraps code in the `webio::launch` engine, maintaining **70µs - 400µs** response times.

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

## 📖 Usage

```rust,no_run
use webio::*;
use webio_macros::webio_main;

#[webio_main]
async fn main() {
    let mut app = WebIo::new();

    app.route(GET, "/", |_, _| async {
        Reply::new(StatusCode::Ok).body("Hello from 🦅 WebIO!")
    });

    app.run("127.0.0.1", "8080").await;
}
```

## ⚖️ License

Licensed under the MIT License. Built for performance.