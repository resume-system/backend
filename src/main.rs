// #![deny(warnings)]

#[macro_use]
extern crate diesel;
mod login;
mod db;
pub use db::*;

mod config;
mod error;
mod redis;
mod email;

pub use config::*;

use actix_web::{App, HttpServer};
use crate::error::RError;

pub type Result<T> = core::result::Result<T, RError>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin()
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
