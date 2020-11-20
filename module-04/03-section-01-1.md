# Section I - manifest

> [Cargo.toml](https://github.com/dsietz/tdg-workshop/blob/master/rust-tdg/Cargo.toml)

Let's begin by declaring a new executable for the service that will act as the _data sourcing_ RESTful endpoint. We will do this by adding a `[[bin]]` section to our `Cargo.toml`manifest file.

```rust
[[bin]]
name = "tdg_analyzer"
path = "src/bin/tdg-analyzer.rs"
```

Next, we need to include the dependent crates into the project. Add the following lines to the `[dependencies]` section in the `Cargo.toml` file

```rust
kafka = "0.8"
clap = "2.33"
```

The `Cargo.toml` file should now look like this:

```rust
[package]
name = "rust-tdg"
version = "0.1.0"
authors = ["ec2-user"]
edition = "2018"

[lib]
name = "myapp"
path = "src/lib.rs"

[[bin]]
name = "tdg_service"
path = "src/bin/tdg-service.rs"

[[bin]]
name = "tdg_analyzer"
path = "src/bin/tdg-analyzer.rs"

[dependencies]
log = "0.4"
env_logger = "0.8"
actix-web = "3"
test-data-generation = "0.2.0"
kafka = "0.8"
clap = "2.33"

[dev-dependencies]
actix-rt = "1.1"

```

