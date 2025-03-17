
use std::fmt::Debug;

use hyper::StatusCode;
use sea_orm::DbErr;

use axum::response::{IntoResponse, Response};
use thiserror::Error;
#[derive(Debug, Error)]
pub enum SystemErr {
    #[error("{2}")]
    BizErr(StatusCode, i32, String),
    #[error("Internal Server Error: {0}")]
    StatusCode(StatusCode),
    #[error("UNAUTHORIZED")]
    UNAUTHORIZED,
    #[error(transparent)]
    DbErr(#[from] DbErr),
    #[error("{0}")]
    Any(String),
}

impl SystemErr {
    pub fn any<M: std::fmt::Display>(msg: M) -> Self {
        Self::Any(format!("{msg}"))
    }
}
use SystemErr::*;

use super::r::R;
impl IntoResponse for SystemErr {
     fn into_response(self) -> Response {
        let (state_code, code) = match self {
            BizErr(state_code, code, _) => (state_code, code),
            StatusCode(state_code) => (state_code, 500),
            UNAUTHORIZED => (StatusCode::UNAUTHORIZED, 401),
            DbErr(_) => (StatusCode::INTERNAL_SERVER_ERROR, 500),
            Any(_) => (StatusCode::INTERNAL_SERVER_ERROR, 500),
        };
        R::<()>::with_msg(code, self)
        .status(state_code)
        .into_response()
    }
}