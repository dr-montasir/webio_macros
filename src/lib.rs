//! # WebIO Macros 🦅
//! 
//! Procedural macros for the WebIO ecosystem, built with a **zero-dependency philosophy**.
//! 
//! ## Why WebIO Macros?
//! Procedural macros usually rely on heavy crates like `syn` and `quote`. **WebIO Macros** 
//! strictly utilizes the built-in `proc_macro` library to provide attribute-based 
//! efficiency without increasing compilation overhead or dependency bloat.

#![doc = include_str!("../README.md")]

extern crate proc_macro;
use proc_macro::TokenStream;

/// # WebIO Main Entry Point Macro
/// 
/// `webio_main` is a procedural macro attribute that transforms an `async fn main()` 
/// into a standard synchronous `main` function that automatically invokes the 
/// **WebIO** `launch` engine.
///
/// ### How it works:
/// The body of the async function is captured and wrapped inside a call to 
/// `::webio::launch()`. This ensures the code benefits from WebIO's 
/// **Safe-Turbo** spin-loop execution (70µs - 400µs latency) without 
/// manual boilerplate.
///
/// ### Requirements:
/// - The `webio` crate is required in the `Cargo.toml`.
/// - The attribute requires a function with a body block `{ ... }`.
///
/// ### Example:
/// ```rust
/// use webio_macros::webio_main;
/// 
/// #[webio_main]
/// async fn main() {
///     // Async logic execution
///     println!("WebIO Launched!");
/// }
/// ```
#[proc_macro_attribute]
pub fn webio_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = item.to_string();
    
    // Extract the body of the function (everything inside { ... })
    let body = match input.find('{') {
        Some(index) => &input[index..],
        None => {
            return "compile_error!(\"webio_main attribute must be applied to a function with a body block { }\")"
                .parse()
                .unwrap()
        }
    };

    // Generate a standard synchronous main function alongside a bridge 
    // that executes a direct call to ::webio::launch.
    let output = format!(
        r#"
        fn main() {{
            let fut = async move {};
            ::webio::launch(fut);
        }}
        "#,
        body
    );

    output.parse().expect("Failed to parse generated tokens")
}
