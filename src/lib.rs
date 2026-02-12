//! # WebIO Macros 🦅
//! 
//! Procedural macros for the WebIO ecosystem, built with a **zero-dependency philosophy**.
//! 
//! ## Why WebIO Macros?
//! Procedural macros usually rely on heavy crates like `syn` and `quote`. **WebIO Macros** 
//! strictly utilizes the built-in `proc_macro` library to provide attribute-based 
//! efficiency and compile-time template transformations without increasing 
//! compilation overhead or dependency bloat.
//!
//! ## Key Features
//! - **The Entry Point**: `#[webio_main]` transforms async entry points into high-efficiency 
//!   execution units managed by the WebIO engine.
//! - **Template Engine**: `replace!` and `html!` provide zero-dependency string substitution 
//!   at the compilation phase, optimized for raw string literals and web content.

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

/// # replace!($content, $key = $val, ...)
/// 
/// **WebIO Zero-Dependency Template Engine**
///
/// The `replace` macro takes a template string (content) along with key-value pairs 
/// and substitutes placeholders (formatted as `{{key}}`) with their corresponding values.
/// 
/// Built with a **zero-dependency philosophy**, it performs efficient string 
/// transformations during the compilation phase, making it ideal for high-performance 
/// WebIO applications where latency is critical.
///
/// ## Parameters
/// - `$content`: The template string or variable containing placeholders (e.g., `"Hello {{name}}"`).
/// - `$key`: The identifier matching the placeholder name inside the braces (e.g., `name`).
/// - `$val`: The value (literal or variable) to inject (e.g., `"Alice"` or `user_var`).
///
/// ## Examples
/// ```rust
/// use webio_macros::replace;
///
/// let template = "<p>Hello, {{name}}! Welcome to {{platform}}.</p>";
/// let result = replace!(template, name = "Developer", platform = "WebIO");
/// 
/// assert_eq!(result, "<p>Hello, Developer! Welcome to WebIO.</p>");
/// ```
///
/// ## Handling Raw Strings
/// The macro works seamlessly with Rust's raw strings, which is perfect for 
/// embedding code or HTML without escaping quotes:
/// ```rust
/// use webio_macros::replace;
///
/// let script = r#"console.log("User: {{user}}");"#;
/// let out = replace!(script, user = "Admin");
/// ```
///
/// ### Example: Creating a domain-specific wrapper
/// You can wrap `replace!` inside your own `macro_rules!` to create specialized 
/// tools like an CSS builder:
/// ```rust
/// #[macro_export]
/// macro_rules! css {
///     ($content:expr, $($key:ident = $val:expr),*) => {
///         $crate::replace!($content, $($key = $val),*)
///     };
/// }
/// ```
/// Now, `css!` can be used to easily replace placeholders in css templates.
/// 
/// ## Extensibility: Creating Custom Macros
/// Developers can easily wrap `replace!` to create language-specific macros 
/// for their own WebIO components:
/// 
/// ```rust
/// #[macro_export]
/// macro_rules! glsl {
///     ($content:expr, $($key:ident = $val:expr),*) => {
///         $crate::replace!($content, $($key = $val),*)
///     };
/// }
/// // let shader = glsl!("void main() { gl_FragColor = {{color}}; }", color = "vec4(1.0)");
/// ```
#[proc_macro]
pub fn replace(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    
    let mut parts = input_str.splitn(2, ',');
    let template = parts.next().unwrap_or("\"\"").trim();
    let remaining = parts.next().unwrap_or("");

    let mut output_code = format!("{{ let mut content_string = {}.to_string();", template);

    for pair in remaining.split(',') {
        let pair = pair.trim();
        if pair.is_empty() { continue; }

        if let Some((key, val)) = pair.split_once('=') {
            let key = key.trim();
            let val = val.trim();

            // 1. Build the search pattern "{{key}}" safely
            // We use string interpolation to create "{{name}}"
            let pattern = format!("{{{{") + key + "}}";

            // 2. Build the replacement line
            // We use r#""# (raw strings) to make the code generated easy to read
            let line = format!(
                r#"content_string = content_string.replace("{}", &format!("{{}}", {}));"#,
                pattern, val
            );
            
            output_code.push_str(&line);
        }
    }

    output_code.push_str(" content_string }");
    output_code.parse().expect("Failed to parse replace macro")
}

/// ### html!($content, $key = $val, ...)
/// 
/// **WebIO Semantic HTML Template Macro**
///
/// The `html` macro is a specialized alias for `replace!`. It is designed to improve 
/// code readability when generating HTML structures within the WebIO framework.
/// 
/// It substitutes `{{key}}` placeholders with dynamic values, allowing for 
/// clean, logic-less HTML templates that are processed at high speed.
///
/// ### Parameters
/// - `$content`: The HTML template string (often used with raw strings `r#""#`).
/// - `$key`: The identifier for the HTML placeholder (e.g., `title`, `body`).
/// - `$val`: The content to inject into the HTML (e.g., `user_input` or a static string).
///
/// ### Examples
/// ```rust
/// use webio_macros::html;
///
/// let user = "Ahmed";
/// let card = html!(r#"<div class="user">{{name}}</div>"#, name = user);
/// 
/// assert_eq!(card, r#"<div class="user">Ahmed</div>"#);
/// ```
/// 
/// Using `html!` alongside `webio_main` allows for rapid UI generation 
/// without the overhead of heavy template engines.
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    // Leverages the core replacement engine to provide a domain-specific HTML macro.
    // Acts as a semantic alias by proxying input to the core replacement engine.
    replace(input)
}