# Section III - executable

> [reporting.rs](https://github.com/dsietz/daas-workshop/blob/master/rust-daas/src/bin/reporting.rs)

Since the SDKs contain all the modules we will need for our web service, we can go right to writing our executable: `src/bin/reporting.rs`.

We start by declaring our dependent external crates

```rust
extern crate actix_web;
```

We then declare the modules we will be using.

```rust
use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse};
use actix_web::http::{StatusCode};
use actix_web::middleware::Logger;
```

We will be referencing a global variable, so we will include that next.

```rust
static ALL_PRODUCTS: &str = "all";
```

We can now add our supportive functions.

> Just like we did when building the Provisioning Microservice, we will first build the `wrapper` for the service and confirm it works before including our business logic.

```rust
async fn index(req: HttpRequest) -> HttpResponse {
    let product = req.match_info().get("product").unwrap_or(ALL_PRODUCTS);
    
   let content = match &product {
        &"all" => {
            ALL_PRODUCTS.to_string()
        },
        _ => {
            product.to_string()
        },
    };
    
    HttpResponse::build(StatusCode::OK)
        .body(&content)
}
```

Lastly, we write the `main` function that will be executed.

```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .route("/{product}", web::get().to(index))
    })
    .bind("localhost:8001")?
    .run()
    .await
}
```

