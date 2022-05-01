#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use std::env;

mod db;
mod errors;
mod handlers;
mod models;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    db::init();

    let server = || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(handlers::init_routes)
    };
    HttpServer::new(server).bind("127.0.0.1:8080")?.run().await
}
