# Section V - module

> [tdg\_service.toml](https://github.com/dsietz/tdg-workshop/blob/master/rust-tdg/src/tdg_service.rs)

To make changes to the logic of our service, we only need to modify the `tdg_service.rs` module that the binary service calls.

Let's replace the `Hello Wolrd!` message with some generated test data.

We first make our `use` declarations.

```rust
use test_data_generation::data_sample_parser::DataSampleParser;
```

Next, add a global constant under the `use` declarations that will be referenced in our funciton.

```rust
static WORKSPACE_LOCAL_STORAGE: &str = "../profiles";
```

We then modify our `index` function to load a Data Sample Parser file \(`Profile`\) and generate some test data.



```rust
pub fn index(_req: HttpRequest) -> HttpResponse {
    let dsp_file = &format!("{}/{}", WORKSPACE_LOCAL_STORAGE, "sample-01-dsp");
    let mut dsp = DataSampleParser::from_file(&dsp_file);

    HttpResponse::build(StatusCode::OK)
    .body(dsp.generate_record()[0].to_string())
}
```

Your final code should look like this:

```rust
use super::*;
use actix_web::{HttpRequest, HttpResponse };
use actix_web::http::{StatusCode};
use test_data_generation::data_sample_parser::DataSampleParser;

static WORKSPACE_LOCAL_STORAGE: &str = "../profiles";

pub fn get_service_root() -> String {
    format!("/tdg/{}", VER)
}

pub fn get_service_path() -> String {
    get_service_root() + "/"
}

pub fn index(_req: HttpRequest) -> HttpResponse {
    let dsp_file = &format!("{}/{}", WORKSPACE_LOCAL_STORAGE, "sample-01-dsp");
    let mut dsp = DataSampleParser::from_file(&dsp_file);

    HttpResponse::build(StatusCode::OK)
    .body(dsp.generate_record()[0].to_string())
}

#[cfg(test)]
mod tests {
   use super::*;
   #[allow(unused_imports)]
   use actix_web::{test};

    #[test]
    fn test_get_service_root() {
        assert_eq!(get_service_root(), format!("/tdg/{}", VER));
    }
    
    #[test]
    fn test_get_service_path() {
        assert_eq!(get_service_path(), format!("/tdg/{}/", VER));
    }
    
    #[test]
    fn ok_response() {
        let req = test::TestRequest::with_header("content-type", "text/plain")
        .to_http_request();

        let resp = index(req);
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
```

Make sure to rerun your tests using `cargo test`

