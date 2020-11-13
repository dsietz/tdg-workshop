# Section IV - module

> tdg\_service.rs

To create the module, create a new file named `tdg_service.rs` in the **/src** directory.

To begin, we will follow some basic TDD practices and build our tests first.

> NOTE: This is not a TDD workshop, so we will ignore the complete practices and simply illustrate how it would be done.

At the bottom of the file, create an empty nested _testing_ module. This will be where we write our unit test for the hello\_world module. The use `super::*;` line imports all the functionality and variables from the parent `tdg_service` module.

```rust
#[cfg(test)]
mod tests {
    use super::*;
}
```

Our first test will be to return the service root. Add the following test in tests module below the `use super::;` line so it looks like the following.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_service_root() {
        assert_eq!(get_service_root(), format!("/tdg/{}", VER));
    }
}
```

Following TDD practices, we now run our test and confirm that it will fail.

```text
ArchConfWorkshopUser:~/environment/rust-tdg/target/debug (master) $ cargo test
   Compiling rust-tdg v0.1.0 (/home/ec2-user/environment/rust-tdg)
error[E0425]: cannot find function `get_service_root` in this scope
 --> src/tdg_service.rs:7:20
  |
7 |         assert_eq!(get_service_root(), format!("/tdg/{}", VER));
  |                    ^^^^^^^^^^^^^^^^ not found in this scope

error[E0425]: cannot find value `VER` in this scope
 --> src/tdg_service.rs:7:61
  |
7 |         assert_eq!(get_service_root(), format!("/tdg/{}", VER));
  |                                                             ^^^ not found in this scope
  |
help: consider importing this static
  |
3 |     use crate::VER;
  |

warning: unused import: `super::*`
 --> src/tdg_service.rs:3:9
  |
3 |     use super::*;
  |         ^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: static is never used: `VER`
 --> src/lib.rs:5:1
  |
5 | static VER: &str = "v1";
  | ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted

error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0425`.
error: could not compile `rust-tdg`.

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
```

To make the test pass, we will add the `get_service_root()` function to the module.

```rust
use super::*;

pub fn get_service_root() -> String {
    format!("/tdg/{}", VER)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_service_root() {
        assert_eq!(get_service_root(), format!("/tdg/{}", VER));
    }
}
```

If we rerun our test, it will now pass.

```text
ArchConfWorkshopUser:~/environment/rust-daas (master) $ cargo test
   Compiling rust-daas v0.1.0 (/home/ec2-user/environment/rust-daas)
    Finished test [unoptimized + debuginfo] target(s) in 0.89s
     Running target/debug/deps/myapp-deab36d0847aeb68

running 1 test
test hello_world::tests::test_get_service_root ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/hello_world-c97b0cb1d60cefb2

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests myapp

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

We will do the same for the `get_service_path()` function.

```rust
use super::*;

pub fn get_service_root() -> String {
    format!("/hello/{}", VER)
}

pub fn get_service_path() -> String {
    get_service_root() + "/"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_service_root() {
        assert_eq!(get_service_root(), format!("/hello/{}", VER));
    }

    #[test]
    fn test_get_service_path() {
        assert_eq!(get_service_path(), format!("/hello/{}/", VER));
    }
}
```

Now that we have an understanding of how to write our tests, and then add the functionality to make them pass, we will move on to provide our service call.

Our test will be the following.

```rust
#[cfg(test)]
mod tests {
   use super::*;
   #[allow(unused_imports)]
   use actix_web::{test};

   #[test]
    fn test_get_service_root() {
        assert_eq!(get_service_root(), format!("/hello/{}", VER));
    }

    #[test]
    fn test_get_service_path() {
        assert_eq!(get_service_path(), format!("/hello/{}/", VER));
    }

   #[test]
    fn hello_response() {
        let req = test::TestRequest::with_header("content-type", "text/plain")
        .to_http_request();

        let resp = index(req);
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
```

In order to make it pass, we will need to import the web service modules, and provide a `index()` function.

```rust
use actix_web::{HttpRequest, HttpResponse };
use actix_web::http::{StatusCode};

pub fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
    .body("Hello World!".to_string())
}
```

The final file should look like the following.

```rust
use super::*;
use actix_web::{HttpRequest, HttpResponse };
use actix_web::http::{StatusCode};

pub fn get_service_root() -> String {
    format!("/hello/{}", VER)
}

pub fn get_service_path() -> String {
    get_service_root() + "/"
}

pub fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
    .body("Hello World!".to_string())
}

mod tests {
   use super::*;
   #[allow(unused_imports)]
   use actix_web::{test};

   #[test]
    fn test_get_service_root() {
        assert_eq!(get_service_root(), format!("/hello/{}", VER));
    }

    #[test]
    fn test_get_service_path() {
        assert_eq!(get_service_path(), format!("/hello/{}/", VER));
    }

   #[test]
    fn hello_response() {
        let req = test::TestRequest::with_header("content-type", "text/plain")
        .to_http_request();

        let resp = index(req);
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
```

Rerun the tests to make sure it all passes.

