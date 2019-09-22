//! This is documentation for the `awf-help` crate.
//!
//! # Examples
//! Cargo.toml add
//! ```
//! [dependencies]
//! awf-help = "0.1"
//! actix-web = "1.0"
//! inventory = "0.1"
//! ```
//! import namespace
//! ```
//! extern crate awf_help;
//! use awf_help::{config, route,route_res, ServiceFactory};
//! ```
//! config webservice
//! ```
//!     HttpServer::new(|| App::new().configure(config))
//!         .bind("127.0.0.1:8000")
//!         .expect("Can not bind to port 8000")
//!         .run()
//!         .unwrap();
//!  ```
//! add decorator
//!  ```
//! #[route(GET, "/")]
//! fn greet(req: HttpRequest) -> String {
//!    let name = req.match_info().get("name").unwrap_or("World");
//!    format!("Hello {}!", &name)
//! }
//!  #[route(POST, "/")]
//!  #[route(HEAD, "/")]
//!  ```
//! 
//!  ```
//! #[route_res("/hello")]
//! impl Hello {
//!     fn get(req: HttpRequest) -> String {
//!         format!("get Hello !")
//!     }
//!     fn post(req: HttpRequest) -> String {
//!         format!("post Hello !")
//!     }
//! }
//!  ```
//!
use actix_web::web;
pub trait ServiceFactory {
    fn register(&self, config: &mut web::ServiceConfig);
}

inventory::collect!(Box<dyn ServiceFactory>);
pub fn config(cfg: &mut web::ServiceConfig) {
    for route in inventory::iter::<Box<dyn ServiceFactory>> {
        route.register(cfg);
    }
}
extern crate awf_codegen;
pub use awf_codegen::route;
pub use awf_codegen::route_res;
