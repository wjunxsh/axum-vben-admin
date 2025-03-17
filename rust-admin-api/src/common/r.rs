use std::fmt::{Debug, Display};

use hyper::StatusCode;

use axum::{response::{IntoResponse, Response}, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct R<T> {
    #[serde(skip)]
    pub status_code: StatusCode,
    pub code: i32,
    pub message: String,
    pub data: Option<T>
}

impl<T> R<T> {
    pub fn new<M: Into<String>>(code: i32, result: Option<T>, msg: M) -> Self {
        Self {
            code,
            data: result,
            message: msg.into(),
            status_code: StatusCode::OK,
        }
    }

    pub fn ok(data: T) -> Self {
        Self {
            code: 200,
            status_code: StatusCode::OK,
            message: "success".to_string(),
            data: Some(data)
        }
    }
    pub fn status(mut self, state: StatusCode) -> Self {
        self.status_code = state;
        self
    }
    pub fn with_msg<M: Display>(code: i32, msg: M) -> Self {
        Self::new(code, None, format!("{msg}"))
    }
    pub fn err(code: i32,msg:&str) -> Self {
        println!("err code: {}, msg: {}", code, msg);
        Self {
            code: code,
            status_code: StatusCode::OK,
            message: msg.to_string(),
            data: None
        }
    }
}

//实现From trait，这样我们就可以直接将一个T类型的对象转换为R<T>类型的对象
impl<T> From<T> for R<T>
where
    T: Serialize,
{
    fn from(v: T) -> Self {
        Self::ok(v)
    }
}

//handler 返回的结果需要用的这一个trait
impl<T> IntoResponse for R<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status_code = self.status_code;
        (status_code, Json(self)).into_response()
    }
}

