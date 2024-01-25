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
mod reponse;

pub use config::*;

use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use crate::error::RError;
use crate::login::{handle_register_submit, handle_register_verify, handle_send_captcha};

pub type Result<T> = core::result::Result<T, RError>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(
        env_logger::Env::new().default_filter_or("info"),
    );

    HttpServer::new(|| {
        App::new()
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin()
            )
            .wrap(Logger::default())
            .service(
                web::scope("/register")
                    .service(handle_register_submit)
                    .service(handle_send_captcha)
                    .service(handle_register_verify)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
