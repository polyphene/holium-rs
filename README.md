<div align="center">

  <h1><code>holium-runtime</code></h1>

<strong>Repository containing the Holium Runtime library and its CLI</strong>
</div>

## About

This repository has been created in order to have a library giving access to the Holium runtime environment.

Along with the library a CLI is developed to use transformations from wasm files.

## 🚴 Usage

### CLI

#### 🏃 Run the transformation with `cargo run`

The CLI can be used with `cargo run`

Example:
```bash
$ cargo run -p holium-runtime-cli -- ./target/wasm32-unknown-unknown/release/simple_rust_test.wasm -i input=\"string\"

------------------- RESULTS -------------------
KEY : output
VALUE : {"string":"string","uint":0}
```

#### 🛠️ Build executable with `cargo build`

The CLI can be built by running `cargo build -p holium-runtime-cli --release`.

Executable is located in `<project-root>/target/release/holium-runtime-cli`