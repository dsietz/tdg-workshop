# Section IV - integrated testing

> [web-service-tests.rs](https://github.com/dsietz/tdg-workshop/blob/master/rust-tdg/tests/web-service-tests.rs)

We now have one last step, which is to add a function that will provide the service object. This is will not be covered by unit testing and is instead should be covered by integrated testing.

To create integrated tests, first create a new file named `web-service-tests.rs` in a new directory named _**tests**_ in the root path \(same level as the **src** directory\). Cargo will automatically parse the _**tests**_ directory and run any tests that are located in any files located here.

```text
.
|-- .git
|-- .gitignore
|-- src
     |-- bin
          |-- tdg-service.rs
     |-- tdg_service.rs
     |-- lib.rs
|-- tests
     | -- web-service-tests.rs
|-- Cargo.toml
```

In order to execute our service test, we will first need to include the some libraries to our project. We do this by adding them in the `[dev-dependencies]` section of the `Cargo.toml` file.

```rust
[dependencies]
actix-web = "3"
test-data-generation = "0.2"
log = "0.4"
env_logger = "0.8"

[dev-dependencies]
actix-rt = "1.1"
```

Once the library has been included in the Manifest, we define which libraries are required in the _**web-service-tests**_ module by adding the following lines at the top of the `web-service-tests.rs` file.

```rust
extern crate actix_web;
```

The `extern` declarations specify the dependent crates \(or libraries\) that will be used in the _**web-service-tests**_ module.

We then declare the bindings \(or shortcuts\) to a resources that will be using in the _**web-service-tests**_ module. This is done by adding the following `use` declarations below the `extern` crate declarations.

```rust
use myapp::tdg_service;
use actix_web::{test, web, App};
```

Now we can add the code for our service test, which is added below the `use` declarations.

```rust
#[actix_rt::test]
async fn test_tdg_service_ok() {

    let mut app = test::init_service(App::new().route("/", web::get().to(tdg_service::index))).await;
    // create the request
    let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
    // call the service
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
}
```

At this point the `web-service-tests.rs` file should look like this:

```rust
extern crate actix_web;

use myapp::tdg_service;
use actix_web::{test, web, App};

#[actix_rt::test]
async fn test_tdg_service_ok() {

    let mut app = test::init_service(App::new().route("/", web::get().to(tdg_service::index))).await;
    // create the request
    let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
    // call the service
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
}
```

Try running your test with the `cargo test` command. There should now be a line in the results referencing that the `web_service_tests` has run.

```text
     Running deps/web_service_tests-8b5defe2e4fcb4b8

running 1 test
test test_tdg_service_ok ... ok
```

