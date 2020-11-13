# Section III - Creating an Executable

Although this package was created as a library, we can make it dual binary and library by doing the following:

1. Modify the `Cargo.toml` file to point to a binary during build time

```rust
[[bin]]
name = "tdg_service"
path = "src/bin/tdg-service.rs"
```

1. Create a new **bin** folder in the **src** directory
2. Create a new file `tdg-service.rs` in the **bin** directory with the following code

```rust
pub fn main() {
    println!("Hello World");
}
```

1. Build and run the package 

```text
ArchConfWorkshopUser:~/environment/rust-tdg (master) $ cargo run
   Compiling rust-tdg v0.1.0 (/home/ec2-user/environment/rust-tdg)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/tdg_service`
Hello World
```

> There should now be an executable named **tdg\_service** in the /target/debug directory which you can execute directly.

```text
ArchConfWorkshopUser:~/environment/rust-tdg (master) $ cd ./target/debug/
ArchConfWorkshopUser:~/environment/rust-tdg/target/debug (master) $ ./tdg_service 
Hello World
```

