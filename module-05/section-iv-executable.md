# Section I - module

> tdg\_service.rs

We will begin by first adding a new function to handle the new resource path that supports path parameters. Open the `bin/tdg_service.rs` file for editing. 

We will begin by writing our new unit test. In the nested `test` module, add the following test function.

```rust
    #[test]
    fn test_get_topic_service_path() {
        assert_eq!(get_service_path_topic(), format!("/tdg/{}/{{topic}}", VER));
    }
```

The `tests` module should look like this:

```rust
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
    fn test_get_topic_service_path() {
        assert_eq!(get_service_path_topic(), format!("/tdg/{}/{{topic}}", VER));
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

{% hint style="warning" %}
If you try to run `cargo test` it should fail.
{% endhint %}

Now we start writing our code by defining the `use` declarations

```rust
use test_data_generation::{Profile, shared};
```

Next, define a global constant for where we will be reading the `profile` files.

```rust
static WORKSPACE_LOCAL_STORAGE: &str = "../profiles";
```

Keeping with format, we will add a function that returns the path to our new resource service.

```rust
pub fn get_service_path_topic() -> String {
    get_service_root() + "/{topic}"
}
```

So as to not impact our default resource path `/`, we will add a new function that supports our new resource path `/{topic}`

```rust
#[get("/tdg/v1/{topic}")] 
pub fn profile(web::Path(topic): web::Path<String>) -> HttpResponse {
    let profile_file = shared::string_to_static_str(format!("{}/{}", WORKSPACE_LOCAL_STORAGE, topic));
    let mut profile = Profile::from_file(&profile_file);

    HttpResponse::build(StatusCode::OK)
    .body(profile.generate().to_string())
}
```

{% hint style="info" %}
Run `cargo test` to ensure everything is passing.
{% endhint %}

