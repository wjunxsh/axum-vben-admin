use axum::{extract::FromRequestParts, http::request::Parts};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::common::error::SystemErr;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QsQuery<T>(pub T);

impl<T, S> FromRequestParts<S> for QsQuery<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = SystemErr;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        println!("parts.uri.query():{:?}",parts.uri.query());
        Ok(Self(
            serde_qs::from_str(&parts.uri.query().unwrap_or_default()).map_err(SystemErr::any)?,
        ))
    }
}
