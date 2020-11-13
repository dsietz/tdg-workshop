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