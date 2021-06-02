<div align="center">

  <h1><code>holium-runtime-cli</code></h1>

<strong>Crate containing the Holium Runtime CLI</strong>
</div>

## About

This crate has been created in order to use the Holium Runtime from a command line interface.

It is mostly based on the Holium Runtime CLI 

## 🌐 Usage

### 🏃 Run a transformation

The CLI can be used with `holium-runtime-cli run`

```bash
$ holium-runtime-cli run -h

Used to run a transformation

USAGE:
    holium-runtime-cli.exe run [OPTIONS] <WASM_FILE>

ARGS:
    <WASM_FILE>    Sets the wasm file to run

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i <inputs>...        Specify the inputs that should be used to run the transformation
```
The `inputs` are must be specified as `key=value` where key is a string and value a 
serialized type.



#### 🛠️ Build executable with `cargo build`

The CLI can be built by running `cargo build -p holium-runtime-cli --release`.

Executable is located in `<project-root>/target/release/holium-runtime-cli`