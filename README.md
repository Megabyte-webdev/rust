# Rust Workspace Examples

A small Rust workspace containing a command-line application and a library crate. The workspace currently has two Cargo members: `cli` and `core`. There is also a `src/` example at the workspace root demonstrating a simple `Product` model; because the root manifest is a virtual workspace manifest, that root-level source is not a Cargo package or workspace member.

## Requirements

- Rust 2024 edition toolchain
- Cargo

## Workspace structure

```text
.
+-- Cargo.toml
+-- cli/
|   +-- Cargo.toml
|   +-- src/main.rs
+-- core/
|   +-- Cargo.toml
|   +-- src/lib.rs
+-- src/
    +-- main.rs
    +-- models.rs
```

## Run the CLI example

From the workspace directory:

```sh
cargo run -p cli
```

The CLI calls `greet` from the local `core` library crate and prints its greeting.

## Build and test the workspace

```sh
cargo build --workspace
cargo test --workspace
```

## Examples

- `cli`: a binary that uses the local library crate.
- `core`: a library exposing a `greet` function.
- Root `src/`: a standalone source example defining a `Product` model and printing its name and price. It is not built by Cargo in the current workspace configuration.

There are no third-party dependencies in the workspace.
