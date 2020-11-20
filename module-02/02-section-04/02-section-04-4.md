# Section IV - executable

> [tdg-service.rs](https://github.com/dsietz/tdg-workshop/blob/master/rust-tdg/src/bin/tdg-service.rs)

Once we have created our service and all the tests have passed, we are ready to build out the executable and run our service.

## Add Log functionality

> Since [Logs](https://12factor.net/logs) is eleventh factor in a 12 Factor Application, we will enable this attribute by implementing automated logging for this RESTful endpoint. by including the [`log`](https://crates.io/crates/log) and [`env_logger`](https://crates.io/crates/env_logger) creates.

We start by first adding the `log` and `env_logger` crates to the `Cargo.toml` manifest.

```rust
[dependencies]
log = "0.4"
env_logger = "0.8"
actix-web = "3"
test-data-generation = "0.2"
```

Next, we update the `lib.rs` file to include the logging creates and modules.

We place the `extern crate` declarations for these crates at the top \(so that they are shared in the project\).

```rust
extern crate log;
extern crate env_logger;
extern crate actix_web;
extern crate test_data_generation;
```

The final `lib.rs` file should look like this:

```rust
extern crate log;
extern crate env_logger;
extern crate actix_web;
extern crate test_data_generation;


static VER: &str = "v1";

pub mod tdg_service;
```

## Writing the executable

Now that we have the dependent crates and modules declared in our library, we can call the service in an executable.

In the `tdg-service.rs` file in the `bin` directory, rewrite the file so it looks like the following:

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
            web::get().to(tdg_service::index)))
        .bind("127.0.0.1:7999")?
        .run()
        .await
}
```

> Noteworthy: we call the module and its functionality by using the following code snippets:
>
> * `use myapp::tdg_service;`
> * `&tdg_service::get_service_path()`
> * `tdg_service::index`

Make sure all your tests are still passing by using the `cargo test` command.

## Starting the service

We are now ready to start the RESTful service. There are 2 ways to start the service.

1. Running using `cargo run` command while developing \(local service testing\)

```text
ArchConfWorkshopUser:~/environment/rust-tdg (master) $ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/tdg_service`
```

Since we are working on a virtual machine , we will use `curl` to call our services.

Run the following script.

> NOTE: Make sure you are in the environment directory. `cd $HOME/environment`

```text
./scripts/curl-tdg.sh
```

You should see the message `Hello World!`

On the command line where the service is running, you will notice that the calls are being logged and printed to the console.

```text
[2020-11-13T20:51:01Z INFO  actix_web::middleware::logger] 127.0.0.1:49624 curl/7.61.1
[2020-11-13T20:51:01Z INFO  actix_web::middleware::logger] 127.0.0.1:49624 "GET /tdg/v1/ HTTP/1.1" 200 12 "-" "curl/7.61.1" 0.000214
```

To stop the service, use `ctrl` + `c`.

1. Running using the executable.

```text
ArchConfWorkshopUser:~/environment/rust-tdg (master) $ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
```

Whenever you use the `cargo build` command, it places the created executable in the target/debug directory with the same name that was defined in the `Cargo.toml` manifest.

Since it is an executable, simple run the executable from the command terminal, and make the same URL call from the browser.

```text
ArchConfWorkshopUser:~/environment/rust-tdg/target/debug (master) $ ./tdg_service 
[2020-11-13T20:53:31Z INFO  actix_web::middleware::logger] 127.0.0.1:49630 curl/7.61.1
[2020-11-13T20:53:31Z INFO  actix_web::middleware::logger] 127.0.0.1:49630 "GET /tdg/v1/ HTTP/1.1" 200 12 "-" "curl/7.61.1" 0.000220
```

