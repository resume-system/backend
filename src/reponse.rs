use actix_web::{HttpResponse, HttpResponseBuilder};
use serde::Serialize;
use crate::Result;

#[derive(Serialize, Debug)]
pub struct BaseResponse {
    code: u8,
    msg: String,
}

impl BaseResponse {
    pub fn with_fields(code: u8, msg: impl Into<String>) -> Self {
        Self {
            code,
            msg: msg.into(),
        }
    }

    pub fn json(code: u8, msg: impl Into<String>) -> Result<String> {
        Ok(serde_json::to_string(&Self {
            code,
            msg: msg.into(),
        })?)
    }
}