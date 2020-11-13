# Section II - Creating a Library

The Rust package comes automatically setup with a unit test in the `src/lib.rs` file. You can build and test your package in one command.

```text
ArchConfWorkshopUser:~/environment/rust-tdg (master) $ cargo test
   Compiling rust-tdg v0.1.0 (/home/ec2-user/environment/rust-tdg)
    Finished test [unoptimized + debuginfo] target(s) in 2.31s
     Running target/debug/deps/rust_tdg-ca5c114553213fc8

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests rust-tdg

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

By default, the manifest file \(`Cargo.toml`\) has only the **\[package\]** section which contains the meta data about the package. The **\[dependencies\]** section is empty. Since we created this package as a library, the **\[lib\]** section is "assumed" and the `src/lib.rs` file is the default location and file name of the library module.

We will overwrite the defaults by adding the following lines to the `Cargo.toml`file after the **\[package\]** section.

```rust
[lib]
name = "myapp"
path = "src/lib.rs"
```

try to rerun the build and test and notice that the library name changed from **rust-tdg** to **myapp**.

```text
ArchConfWorkshopUser:~/environment/rust-tdg (master) $ cargo test
   Compiling rust-tdg v0.1.0 (/home/ec2-user/environment/rust-tdg)
    Finished test [unoptimized + debuginfo] target(s) in 0.29s
     Running target/debug/deps/myapp-3a0fcee6b881b370

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests myapp

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

