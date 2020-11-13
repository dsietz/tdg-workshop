# Section II - executable

> sourcing.rs

We first need to create the executable file: `src/bin/tdg-analyzer.rs`.

We start by declaring our dependent external crates

```rust
extern crate kafka;
extern crate clap;
```

We then declare the modules we will be using.

```rust
use daas::service::listener::{DaaSListener, DaaSListenerService};
use daas::service::extractor::{Base64Author};
use pbd::dua::middleware::actix::*;
use pbd::dtc::middleware::actix::*;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
```

Finally, we write the main function that will be called.

```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("DAAS_LOCAL_STORAGE", "../local_storage");
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| App::new()        
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(DUAEnforcer::default())
            .wrap(DTCEnforcer::default())
            .service(
                web::resource(&DaaSListener::get_service_health_path()).route(web::get().to(DaaSListener::health))
            )
            .service(
                web::resource(&DaaSListener::get_service_path()).route(web::post().to(DaaSListener::index::<Base64Author>))
            )
        )
    .bind("localhost:8000")
    .unwrap()
    .run()
    .await
}
```

Since the DaaS Listener that consumes the source data is loosely coupled to the broker, it is important that we keep a local copy of the DaaSDocument in case connection to the broker is lost. We configure the directory path the local storage using the environment variable `DAAS_LOCAL_STORAGE`. If this is not set, the DaaSListener module will use the system's default temporary directory.

```rust
std::env::set_var("DAAS_LOCAL_STORAGE", "../local_storage");
```

> NOTE: Implementing the `Logger` middleware may violate privacy strategies depending on the exposure and storage of the log entries.

When you are finished, the `sourcing.rs` file should look like this:

```rust
extern crate daas;
extern crate actix_web;

use daas::service::listener::{DaaSListener, DaaSListenerService};
use daas::service::extractor::{Base64Author};
use pbd::dua::middleware::actix::*;
use pbd::dtc::middleware::actix::*;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("DAAS_LOCAL_STORAGE", "../local_storage");
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| App::new()        
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(DUAEnforcer::default())
            .wrap(DTCEnforcer::default())
            .service(
                web::resource(&DaaSListener::get_service_health_path()).route(web::get().to(DaaSListener::health))
            )
            .service(
                web::resource(&DaaSListener::get_service_path()).route(web::post().to(DaaSListener::index::<Base64Author>))
            )
        )
    .bind("localhost:8000")
    .unwrap()
    .run()
    .await
}
```

