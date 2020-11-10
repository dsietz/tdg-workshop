# Section II - manifest

> [Cargo.toml](https://github.com/dsietz/daas-workshop/blob/master/rust-daas/Cargo.toml)

Let's begin by declaring a new executable for the service that will act as the _data reporting service_. We will do this by adding a `[[bin]]` section to our `Cargo.toml`manifest file.

```rust
[[bin]]
name = "myapp_reporting"
path = "src/bin/reporting.rs"
```

> NOTE: We don't need to add any new crates to the `[dependencies]` section in the `Cargo.toml` file



