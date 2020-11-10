# Section IV - tests



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

