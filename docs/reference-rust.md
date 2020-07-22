# Rust Language : Getting Started

### Rust Language
- [Rust.org](https://www.rust-lang.org)
- [Crates.io](https://crates.io/)

### Installing Rust
- [Installation](https://www.rust-lang.org/tools/install)

### Further Learning
- [The Rust Book](https://doc.rust-lang.org/1.30.0/book/second-edition/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Cargo Book](https://doc.rust-lang.org/cargo/) 
- [Playground](https://play.rust-lang.org/)
>Actix Web framework
- [Web Service Testing with Actix Web](https://github.com/actix/actix-website/blob/master/content/docs/testing.md)
- [Actix Web Crate](https://crates.io/crates/actix-web)
- [Actix Web Homepage](https://actix.rs/)

In order to perform any builds or tests, you first need to be in the package directory where the Cargo.toml file is loacted.
```
[user@localhost workspace]$ cd /rust-daas
[user@localhost rust-daas]$ pwd
/usr/workspace/rust-daas
[user@localhost rust-daas]$ ls -l | grep Cargo.toml
-rw-rw-r--. 1 dsietz dsietz 127 10:19 Mar 04 Cargo.toml
```

### Build
```
[user@localhost rust-daas]$ cargo build
   Compiling rust-daas v0.0.1 (usr/workspace/rust-daas)
    Finished dev [unoptimized + debuginfo] target(s) in 7.68s
```

### Testing
```
[user@localhost rust-daas]$ cargo test
   Compiling rust-daas v0.0.1 (C:\workspace\rust-daas)
    Finished dev [unoptimized + debuginfo] target(s) in 10.99s
     Running target\debug\deps\daas-beec273dce950394.exe

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target\debug\deps\rest_sourcing-9987fcdf20376507.exe

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target\debug\deps\web_service_tests-b2c1f5aabdd18212.exe

running 1 test
test test_status_code_ok ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests daas

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```