use axum::{routing::post, Router};
use std::sync::Arc;
use crate::{app_state::AppState, controller::admin::system::auth::AuthController};
pub fn init_route(app_state: Arc<AppState>) ->Router {
    let route = Router::new()
        .route("/login", post(AuthController::login))
        .with_state(app_state);
    route
}