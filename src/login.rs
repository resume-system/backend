use actix_web::{get, HttpResponse, post};
use actix_web::web::{Json, Query};
use lettre::transport::smtp::response::Response;
use serde::{Deserialize, Serialize};
use crate::{query_user_by_username, user_exists};
use crate::Result;

#[derive(Deserialize)]
pub struct RegisterInfo {
    username: String,
    password: String,
    email: String,
    phone: String,
}

#[post("/submit")]
async fn handle_register_submit(info: Json<RegisterInfo>) -> Result<HttpResponse> {
    #[derive(Serialize)]
    struct Response {
        status_code: u8,
        msg: String,
    }

    let user = query_user_by_username(info.username.as_str())?;

    if let Some(user) = user {
        if user.state == "T" {
            return Ok(
                HttpResponse::Ok()
                    .content_type("application/json")
                    .body(serde_json::to_string(&Response {
                        status_code: 200,
                        msg: "已注册".to_string(),
                    })?)
            );
        }
    }

    Ok(
        HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&Response {
                status_code: 200,
                msg: "请求成功".to_string(),
            })?)
    )
}

#[derive(Deserialize, Debug)]
pub struct CaptchaQuery {
    email: String,
}

#[get("/captcha")]
async fn send_captcha(info: Query<CaptchaQuery>) -> Result<HttpResponse> {

}