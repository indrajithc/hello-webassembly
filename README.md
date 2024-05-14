# hello-wasm-pack

This repository contains a sample Rust project compiled into WebAssembly using wasm-pack. It demonstrates how to build a WebAssembly package that can be used in existing web applications, allowing seamless integration of Rust code with JavaScript frontend.

## Compiling from Rust to WebAssembly

If you have some Rust code, you can compile it into WebAssembly (Wasm). This tutorial will show you how to compile a Rust project into WebAssembly and use it in an existing web app.

### Rust Environment Setup

Let's go through all the required steps to get our environment set up.

1. **Install Rust**: Install Rust by going to the Install Rust page and following the instructions. This installs a tool called "rustup", which lets you manage multiple versions of Rust.
   
2. **wasm-pack**: To build the package, we need an additional tool, wasm-pack. This helps compile the code to WebAssembly, as well as produce the right packaging for use in the browser.

### Building our WebAssembly package

Enough setup; let's create a new package in Rust.

```bash
cargo new --lib hello-wasm
```

This creates a new library with everything needed to get started.

### Writing Rust Code

We write Rust code to be compiled into WebAssembly. Check the `src/lib.rs` file to see the Rust code.

### Compiling our code to WebAssembly

To compile our code correctly, we first need to configure it with Cargo.toml. Open this file, and change its contents.

```toml
[package]
name = "hello-wasm"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
description = "A sample project with wasm-pack"
license = "MIT/Apache-2.0"
repository = "https://github.com/yourgithubusername/hello-wasm"
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

Then, build the package:

```bash
wasm-pack build --target web
```

### Using the package on the web

Now that we've got a compiled Wasm module, let's run it in the browser. Load `index.html` from the root directory of the project using a local web server.

```html
<!doctype html>
<html lang="en-US">
  <head>
    <meta charset="utf-8" />
    <title>hello-wasm example</title>
  </head>
  <body>
    <script type="module">
      import init, { greet } from "./pkg/hello_wasm.js";
      init().then(() => {
        greet("WebAssembly");
      });
    </script>
  </body>
</html>
```
