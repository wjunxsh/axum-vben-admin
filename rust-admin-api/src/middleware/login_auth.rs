
use axum::{
    body::Body, http::{Request, StatusCode}, middleware::Next, response::Response
};
use std::{sync::Arc,  time::{SystemTime, UNIX_EPOCH}};
use crate::{ service::claims::Claims, service::jwt::{ClaimUser, JwtService}};

pub async fn auth_middleware(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = match req.headers().get("Authorization") {
        Some(token) => token.to_str().unwrap_or(""),
        None => return Err(StatusCode::UNAUTHORIZED)
    };
    let data = JwtService::verify_login_token(token);
    let user: ClaimUser = match data {
        Ok(datas) => {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0); // 避免 unwrap panic

            // 检查是否过期
            if datas.exp < now as usize {
                return Err(StatusCode::UNAUTHORIZED);
            }
            datas.user
        }
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };
     // 插入用户信息
     req.extensions_mut().insert(Claims(Arc::new(user)));
    let res = next.run(req).await;
    Ok(res)
}