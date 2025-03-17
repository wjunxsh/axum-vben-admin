use axum::{extract::FromRequestParts, http::request::Parts};
use hyper::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::common::error::SystemErr;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Claims<T>(pub T);

impl<T, S> FromRequestParts<S> for Claims<T>
where
    S: Sync + Send,
    T: DeserializeOwned + Send + Sync + Clone + 'static,
{
    type Rejection = SystemErr;
    /// 将用户信息注入request
    async fn from_request_parts(req: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let get = req.extensions.get::<Claims<T>>();
        match get {
            Some(c) => {
                let c = c.clone();
                Ok(c)
            }
            None => {
                println!("Claims");
                Err(SystemErr::StatusCode(StatusCode::UNAUTHORIZED))
            }
        }
    }
}
