use actix_web::{HttpResponse, post};
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use crate::redis::Captcha;

static SALT: &str = "Azure-Sky";

#[derive(Deserialize)]
pub struct RegisterInfo {
    username: String,
    password: String,
    email: String,
    phone: String,
}

#[post("/register")]
async fn handle_register(info: Json<RegisterInfo>) -> HttpResponse {
    #[derive(Serialize)]
    struct Response {
        status_code: u8,
        msg: String,
        captcha: Option<Captcha>,
    }

    todo!()
}