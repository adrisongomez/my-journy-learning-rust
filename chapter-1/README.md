# Chapter 1: Setup

Understanding the first Rust tools which rust comes with it.

## How to install rust?

Use this bash commands to download rust and installed on the machine.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This install multiple tools that rust use to help development and the compiler:

- `rustc`: The compiler to compile rust into machine code.

```bash
rustc <filename or path>.rs # compile th rust file
./<filename> # run the file
```

- `rustfmt`: Standar formatter for rust file.

```bash
rustfmt <wildcat or filename or path>.rs
```

- `rustup`: Rust version manager
- `cargo`: Package Manager for dependencies and handle task on the project

```bash
cargo init # initialize a new project
cargo new project_name # Create a new folder with the name of the project and initilize the Cargo,toml
cargo new project_name --vsc=git # Create new project and git initialized.
```

## Cargo keynotes

Cargo generated a `Cargo.toml` file which hold the basic configuration to cargo start working.

```BASH
# Output of the `cargo new cargo-example`

[package]
name = "cargo-example"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

- `[package]` hold all of the project metadata
- `[depdencies]` list all the dependencies for the project

Cargo expect that the `src/` directory hold all the `*.rs` file and the top level of the project
only hold `README.md`or others config files.

### Some Cargos commands

```bash
cargo build 
# Compile your project and genereted a Cargo.lock file similar to package=lock.json... DO NOT TOUCH, please
# A target directory is generated too.
# Binaries are saved in ./target/debug/<project-name>

cargo run
# Compile and run your code, Why they didn't start for this first don't idea...
# Also rust is enought intelligent to dectect when you modify your file so run
# the same project without modify anything don't generated multiple binaries.

cargo check
# Check for error on your code, but it's not generating a executable.
# cargo check it's faster than cargo build so it's recommended to use cargo check periodacally
# and when it ready compile and have fun.

cargo build --release
# Create the executable to prod with multiple optimization so it can perform blaziling fast!!!!!.
```
