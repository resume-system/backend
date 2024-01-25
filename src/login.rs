use actix_web::{get, HttpResponse, post};
use actix_web::web::{Json, Query};
use serde::{Deserialize, Serialize};
use crate::{insert_user, insert_user_with_fields, query_user_by_username, SHA256, User, user_exists, verify_user};
use crate::email::EMAIL;
use crate::redis::Captcha;
use crate::reponse::BaseResponse;
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

    let flag = user_exists(info.username.as_str())?;

    if flag {
        return Ok(
            HttpResponse::Ok()
                .content_type("application/json")
                .body(serde_json::to_string(&BaseResponse::with_fields(
                    201, "用户已存在".to_string(),
                ))?)
        );
    }

    insert_user_with_fields(
        info.username.as_str(),
        info.password.as_str(),
        info.email.as_str(),
        info.phone.as_str()
    )?;

    Ok(
        HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&BaseResponse::with_fields(
                200, "请求成功".to_string(),
            ))?)
    )
}

#[derive(Deserialize, Debug)]
pub struct CaptchaQuery {
    username: String,
    email: String,
}

#[get("/captcha")]
async fn handle_send_captcha(info: Query<CaptchaQuery>) -> Result<HttpResponse> {
    EMAIL.send_captcha(info.email.as_str(), info.username.as_str())?;

    Ok(
        HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&BaseResponse::with_fields(
                200, "请求成功".to_string(),
            ))?)
    )
}

#[derive(Deserialize, Debug)]
pub struct Verify {
    captcha: String,
    email: String,
    username: String,
}

#[post("/verify")]
async fn handle_register_verify(info: Json<Verify>) -> Result<HttpResponse> {
    let captcha = Captcha::get_captcha(info.username.as_str())?.unwrap();

    if captcha.text != info.captcha {
        EMAIL.send_captcha(info.email.as_str(), info.username.as_str())?;

        return Ok(
            HttpResponse::Ok()
                .content_type("application/json")
                .body(BaseResponse::json(201, "验证码错误")?)
        )
    }

    if captcha.text == info.captcha && captcha.expiration < chrono::Local::now().naive_local() {
        EMAIL.send_captcha(info.email.as_str(), info.username.as_str())?;

        return Ok(
            HttpResponse::Ok()
                .content_type("application/json")
                .body(serde_json::to_string(&BaseResponse::with_fields(
                    202, "验证码已过期".to_string(),
                ))?)
        )
    }

    verify_user(info.username.as_str())?;

    return Ok(
        HttpResponse::Ok()
            .content_type("application/json")
            .body(BaseResponse::json(200, "登录成功")?)
    )
}