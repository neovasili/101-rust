# 101 Rust

Repository containing Rust code used for [101 Rust talk](https://www.meetup.com/es-ES/CaceresTech/events/292010650/) in CaceresTech meetup.

## How to use

If you want to run the examples in your machine you need to have `Rust` installed. Use the official [install Rust](https://www.rust-lang.org/tools/install) proceddure to do so if you still don't have it.

Once you have `Rust` in your machine, you have to go to each specific folder to run the code. This is the structure so far:

```shell
.
├── LICENSE
├── README.md
├── hello-world
└── modules
    ├── 01-basic-syntax
    ├── 02-control-flow
    ├── 03-ownership
    ├── 04-error-handling
    └── 05-testing
```

### Hello world

A simple Hello world example that can be run just using `cargo run`:

```shell
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/rust_101`

Hello, world!
```

### Modules

Each module has a set of different examples to demonstrate different features of Rust as a programming language.

Examples can be run using the following command:

```shell
cargo run --example NAME_OF_THE_EXAMPLE
```

You can find the available examples for each module in the `Cargo.toml` of each module (e.g. `modules/01-basic-syntax/Cargo.toml`):

```toml
[package]
name = "basic-syntax"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[[example]]
name = "compound_types"

[[example]]
name = "references"

[[example]]
name = "slices"

[[example]]
name = "functions"

[[example]]
name = "types_inference"

```

For the testing module you need to follow a different approach:

```shell
cargo test --example NAME_OF_THE_EXAMPLE
```

Or to run the whole set of unit tests and integration tests:

```shell
cargo test
```

## References

- [Oficial website](https://www.rust-lang.org/)
- [Rust by example](https://doc.rust-lang.org/rust-by-example/index.html)
- [The cargo book](https://doc.rust-lang.org/cargo/index.html)
- [Community registry](https://crates.io/)
- [Go Rust course](https://google.github.io/comprehensive-rust/welcome.html)
- [Install Rust](https://www.rust-lang.org/tools/install)
- [Thiserror (error handling crate)](https://docs.rs/thiserror/latest/thiserror/)
- [Anyhow (error handling crate)](https://docs.rs/anyhow/latest/anyhow/)
