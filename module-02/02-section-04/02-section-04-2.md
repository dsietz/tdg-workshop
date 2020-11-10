# Section IV - library

> [lib.rs](https://github.com/dsietz/daas-workshop/blob/master/rust-daas/src/lib.rs)

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
```

In top portion of the file \(where the global variables would be located after the `extern crates`\), add the following global variable.

```rust
static VER: &str = "v1";
```

At the bottom of the file, add the following module as part of the library.

```rust
pub mod hello_world;
```

> **NOTE**: If you build the code right now, it would raise an error that the module cannot be found.
>
> ```text
> ArchConfWorkshopUser:~/environment/rust-daas (master) $ cargo build
>    Compiling rust-daas v0.1.0 (/home/ec2-user/environment/rust-daas)
> error[E0583]: file not found for module `hello_world`
>  --> src/lib.rs:5:1
>   |
> 5 | pub mod hello_world;
>   | ^^^^^^^^^^^^^^^^^^^^
>   |
>   = help: to create the module `hello_world`, create file "src/hello_world.rs"
>
> error[E0463]: can't find crate for `actix_web`
>  --> src/lib.rs:1:1
>   |
> 1 | extern crate actix_web;
>   | ^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
>
> error: aborting due to 2 previous errors
>
> Some errors have detailed explanations: E0463, E0583.
> For more information about an error, try `rustc --explain E0463`.
> error: could not compile `rust-daas`.
> ```



