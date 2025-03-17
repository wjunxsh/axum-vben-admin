use axum::{
    middleware, Router
};
pub mod system;
pub mod auth;
use std::sync::Arc;
use crate::{app_state:: AppState, middleware::{login_auth::auth_middleware, priority_auth::priority_middleware, request_logger::log_middleware}};

pub fn init_route(app_state: Arc<AppState>) -> Router {
    let route = Router::new()//初始化系统路由
        .merge(system::init_route(app_state.clone()))
        //初始化登录验证路由
        .layer(middleware::from_fn_with_state(app_state.clone(), log_middleware))
        .layer(middleware::from_fn_with_state(app_state.clone(),priority_middleware))
        .layer(middleware::from_fn_with_state(app_state.clone(),auth_middleware))
        .merge(auth::init_route(app_state.clone()));
    let adm_route = Router::new().nest("/adm", route);
    adm_route
}