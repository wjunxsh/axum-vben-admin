use axum::{
    body::Body, extract::{MatchedPath, State}, http::{Request, StatusCode}, middleware::Next, response::Response
};
use std::sync::Arc;
use crate::{app_state:: AppState, service::{claims::Claims, jwt::ClaimUser, user::UserService}};


pub async fn priority_middleware(
    State(app_state): State<Arc<AppState>>,
    req: Request<Body>,
    next: Next,) -> Result<Response, StatusCode> {
    // 获取路由 path
    let path = req.uri().path().to_string();
    let matched_path = req.extensions()
       .get::<MatchedPath>()
       .map(|p| p.as_str().to_string())
       .unwrap_or_else(|| path.clone());
    // 获取 user
    let user = if let Some(claims) = req.extensions().get::<Claims<Arc<ClaimUser>>>() {
        let user = claims.0.clone();
        user
    } else {
        // 如果没有找到用户信息，返回 401 Unauthorized
        return Err(StatusCode::UNAUTHORIZED);
    };
   if user.is_admin != 1 {
       let result = UserService::check_user_api_access(&app_state, user.user_id, &matched_path)
       .await
       .unwrap_or(false);
       if !result {
           return Err(StatusCode::FORBIDDEN);
       }
   };
    let res = next.run(req).await;
    Ok(res)
}