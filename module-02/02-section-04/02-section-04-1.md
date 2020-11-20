# Section IV - manifest

> Cargo.toml

We already have the binary file defined in the manifest file, but there are dependent packages that we will need to include in order to make it a RESTful service.

In the **\[dependencies\]** section of the `Cargo.toml` file add the following packages.

```rust
actix-web = "3"
test-data-generation = "0.2.0"
```

