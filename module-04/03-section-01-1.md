# Section II - manifest

> [Cargo.toml](https://github.com/dsietz/daas-workshop/blob/master/rust-daas/Cargo.toml)

Let's begin by declaring a new executable for the service that will act as the _data sourcing_ RESTful endpoint. We will do this by adding a `[[bin]]` section to our `Cargo.toml`manifest file.

```rust
[[bin]]
name = "myapp_sourcing"
path = "src/bin/sourcing.rs"
```

Next, we need to include the dependent crates into the project. Add the following lines to the `[dependencies]` section in the `Cargo.toml` file

```rust
serde ="1.0"
serde_derive = "1.0"
serde_json = "1.0"
daas = "0.2.0"
pbd = "0.3.0"
```

The `Cargo.toml` file should now look like this:

```rust
[package]
name = "rust-daas"
version = "0.1.0"
authors = ["dsietz <davidsietz@yahoo.com>"]
edition = "2018"

[lib]
name = "myapp"
path = "src/lib.rs"

[[bin]]
name = "hello_world"
path = "src/bin/hello-world.rs"

[[bin]]
name = "myapp_sourcing"
path = "src/bin/sourcing.rs"

[dependencies]
log = "0.4"
env_logger = "0.8"
actix-web = "3"
serde ="1.0"
serde_derive = "1.0"
serde_json = "1.0"
daas = "0.2.0"
pbd = "0.3.0"

[dev-dependencies]
actix-rt = "1.1"
```

