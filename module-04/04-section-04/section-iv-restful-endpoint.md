# Section IV - RESTful endpoint



**Declaring the Executable**

In the `Cargo.toml` file, after the that last `[[bin]]` statement for _hello\_world_, add the following declaration.

```text
[[bin]]
name = "sourcing"
path = "src/bin/daas-sourcing.rs"
```

This will tell Cargo that there is a binary file to be compiled and to name the exeutable **sourcing**.

**Coding the Executable**

Let's begin by creating a file named `daas-sourcing.rs` in the `src/bin/` directory.

At the top of the file, we will declare our `use` statements.

```text
use daas::sourcing;
use actix_web::{server};
```

Understandabily, an executable file needs to have a `main()` function that is called, so we will add that code next. This function will reference the `source` module `service()` function which provides the `App` object.

```text
pub fn main() {    
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    server::new( || {sourcing::service()})
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}
```

> Try to rerun the `cargo test` command, and ensure that all the test pass.

