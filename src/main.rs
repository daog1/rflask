//extern crate awf_codegen;
extern crate awf_help;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
//use h010::config;
//use awf_codegen::route;
use awf_help::{config, route, ServiceFactory};
//use awf_help::ServiceFactory;
#[route(GET, "/")]
fn greet(req: HttpRequest) -> String {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[route(POST, "/")]
fn greet2(req: HttpRequest) -> String {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("post Hello {}!", &name)
}
#[route(POST, "/1123/{name}")]
fn greet3(req: HttpRequest, name: web::Path<String>) -> String {
    format!("post Hello {}!", &name)
}
fn main() {
    HttpServer::new(|| App::new().configure(config))
        .bind("127.0.0.1:8000")
        .expect("Can not bind to port 8000")
        .run()
        .unwrap();
    //println!("Hello, world!");
}
