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