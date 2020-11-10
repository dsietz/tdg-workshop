# Section II - manifest

> [Cargo.toml](https://github.com/dsietz/daas-workshop/blob/master/rust-daas/Cargo.toml)

Let's begin by declaring a new executable for the service that will act as the _genesis service_. We will do this by adding a `[[bin]]` section to our `Cargo.toml`manifest file.

```rust
[[bin]]
name = "myapp_genesis"
path = "src/bin/genesis.rs"
```

Next, we need to include the dependent crates into the project. Add the following lines to the `[dependencies]` section in the `Cargo.toml` file

```rust
rusoto_core = "0.43"
kafka = "0.8"
```

