# Section IV - executable

> [Cargo.toml](https://github.com/dsietz/rust-daas/blob/master/Cargo.toml)
>
> [web-service-tests.rs](https://github.com/dsietz/rust-daas/blob/master/tests/web-service-tests.rs)
>
> [daas-sourcing.rs](https://github.com/dsietz/rust-daas/blob/master/src/bin/daas-sourcing.rs)

We are now ready to write the executable. To do this, we will working with the following files:

* Cargo.toml \(manifest\)
* web-service-tsts.rs \(service tests\)
* src/bin/daas-sourcing.rs \(executable\) 

## Tests

Since the `sourcing` module is used for a RESTful service, it is important that we not only add _Unit Tests_, but also _Integration Tests_ to test for the web service. To do this, we will first add the following dependency to the `[dependencies]` section in hte `Cargo.toml` file.

```text
json = "0.11.13"
```

Next, work on the `web-service-tests.rs` file in the `/tests` directory by adding the following `use` declaration.

```text
use serde_json::{Value};
```

Now we will add our _Integration Tests_.

> TDD practices states that we should have positive and negatives tests for all the possible scenrios, but to save time for this Workshop, we will simple add three example tests.

```text
#[test]
fn test_source_data_ok(){
    let uri = daas::sourcing::get_service_path()
        .replace("{category}","order")
        .replace("{subcategory}","clothing")
        .replace("{source_name}","iStore")
        .replace("{source_uid}","5000");  
    let mut srv =actix_web::test::TestServer::new(|app| {
                        app.resource(
                            &daas::sourcing::get_service_path(),
                            |r| r.post().with(daas::sourcing::source)
                        );
                    });
    let request = srv.post()
                    .uri(srv.url(&uri))
                    .header(header::CONTENT_TYPE, "application/json")
                    .header("Authorization","Basic Zm9vOmJhcg==")
                    .body(r#"{"data":"Hello, world!"}"#)
                    .unwrap();
    let response = srv.execute(request.send()).unwrap();

    assert!(response.status().is_success());

    // read response
    let bytes = srv.execute(response.body()).unwrap();
    let body: Value = serde_json::from_str(&String::from_utf8(bytes.to_vec()).unwrap()).unwrap();

    assert_eq!(body["status"], "OK".to_string());
}

#[test]
fn test_source_data_bad_parameter(){
    let uri = daas::sourcing::get_service_path()
        .replace("{category}","order")
        .replace("{subcategory}","clothing")
        .replace("{source_name}","iStore")
        .replace("{source_uid}","word");  
    let mut srv =actix_web::test::TestServer::new(|app| {
                        app.resource(
                            &daas::sourcing::get_service_path(),
                            |r| r.post().with(daas::sourcing::source)
                        );
                    });
    let request = srv.post()
                    .uri(srv.url(&uri))
                    .header(header::CONTENT_TYPE, "application/json")
                    .header("Authorization","Basic Zm9vOmJhcg==")
                    .body(r#"{"data":"Hello, world!"}"#)
                    .unwrap();
    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), http::StatusCode::NOT_FOUND);
}

#[test]
fn test_source_data_bad_payload(){
    let uri = daas::sourcing::get_service_path()
        .replace("{category}","order")
        .replace("{subcategory}","clothing")
        .replace("{source_name}","iStore")
        .replace("{source_uid}", "112233");  
    let mut srv =actix_web::test::TestServer::new(|app| {
                        app.resource(
                            &daas::sourcing::get_service_path(),
                            |r| r.post().with(daas::sourcing::source)
                        );
                    });
    let request = srv.post()
                    .uri(srv.url(&uri))
                    .header(header::CONTENT_TYPE, "application/json")
                    .header("Authorization","Basic Zm9vOmJhcg==")
                    .body(r#"{"data":...}"#)
                    .unwrap();
    let response = srv.execute(request.send()).unwrap();

    assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);
}
```

## Code

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

