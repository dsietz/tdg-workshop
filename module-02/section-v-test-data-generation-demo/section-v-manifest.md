# Section V - manifest

> Cargo.toml

At this point we should already have all the dependencies defined in our manifest.

```rust
[package]
name = "rust-tdg"
version = "0.1.0"
authors = ["ec2-user"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[lib]
name = "myapp"
path = "src/lib.rs"

[[bin]]
name = "tdg_service"
path = "src/bin/tdg-service.rs"

[dependencies]
log = "0.4"
env_logger = "0.8"
actix-web = "3"
test-data-generation = "0.2.0"

[dev-dependencies]
actix-rt = "1.1"
```

