# Actix web flask decorator

* Cargo package: [awf-help](https://crates.io/crates/awf-help)

## Example
```rust
extern crate awf_help;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use awf_help::{config, route,route_res, ServiceFactory};
#[route(GET, "/")]
fn greet(req: HttpRequest) -> String {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}
struct Hello{
}
#[route_res("/api/auth")]
impl Hello {
    fn get(req: HttpRequest) -> String {
        format!("get Hello !")
    }
    fn post(req: HttpRequest) -> String {
        format!("post Hello !")
    }
}
fn main() {
        HttpServer::new(|| App::new().configure(config))
        .bind("127.0.0.1:8000")
        .expect("Can not bind to port 8000")
        .run()
        .unwrap();
}
```