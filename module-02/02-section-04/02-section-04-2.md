# Section IV - library

> [lib.rs](https://github.com/dsietz/tdg-workshop/blob/master/rust-tdg/src/lib.rs)

The `lib.rs` file is our centralized library file. This file is where globally shared functions, constants and variables are located. It is also where we include modules that make up the library.

**Remove** the test module section of code at the bottom of the file. This is where your integrated testing \(module dependencies\) would be tested to ensure the library if working as expected. However, we will be ignoring this level of testing for the purpose of this workshop.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

At the very top of the file, add [declarations of dependencies ](https://doc.rust-lang.org/reference/items/extern-crates.html)to the external web service crates \(packages\) we will be using.

```rust
extern crate actix_web;
extern crate test_data_generation;
```

In top portion of the file \(where the global variables would be located after the `extern crates`\), add the following global variable.

```rust
static VER: &str = "v1";
```

At the bottom of the file, add the following module as part of the library.

```rust
pub mod tdg_service;
```

> **NOTE**: If you build the code right now, it would raise an error that the module cannot be found.
>
> ```text
> ArchConfWorkshopUser:~/environment/rust-tdg/target/debug (master) $ cargo test
>    Compiling rust-tdg v0.1.0 (/home/ec2-user/environment/rust-tdg)
> error[E0583]: file not found for module `tdg_service`
>  --> src/lib.rs:7:1
>   |
> 7 | pub mod tdg_service;
>   | ^^^^^^^^^^^^^^^^^^^^
>   |
>   = help: to create the module `tdg_service`, create file "src/tdg_service.rs"
>
> error[E0583]: file not found for module `tdg_service`
>  --> src/lib.rs:7:1
>   |
> 7 | pub mod tdg_service;
>   | ^^^^^^^^^^^^^^^^^^^^
>   |
>   = help: to create the module `tdg_service`, create file "src/tdg_service.rs"
>
> error: aborting due to previous error
>
> For more information about this error, try `rustc --explain E0583`.
> error: aborting due to previous error
>
> For more information about this error, try `rustc --explain E0583`.
> error: could not compile `rust-tdg`.
>
> To learn more, run the command again with --verbose.
> warning: build failed, waiting for other jobs to finish...
> error: build failed
> ```

