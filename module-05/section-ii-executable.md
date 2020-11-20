# Section II - executable

> tdg-service.rs

We can now add the new resource path to our executable \(RESTful service\). To do this, we will modify the `bin/tdg-service.rs` file.

To add the new service path \(which calls a funciton with a defined path\), we just need to add the line `.service(tdg_service::profile)` to the `App`

```rust
use myapp::tdg_service;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| App::new()
        .wrap(Logger::default())
        .wrap(Logger::new("%a %{User-Agent}i"))
        .route(
            &tdg_service::get_service_path(), 
            web::get().to(tdg_service::index))
        .service(tdg_service::profile)
        )
        .bind("127.0.0.1:7999")?
        .run()
        .await
}
```



