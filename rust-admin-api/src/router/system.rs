use axum::{
    routing::{delete, get, post},
    Router,
};
use std::sync::Arc;
use crate::{app_state::AppState, controller::admin::system::{
    admin_log::AdminLogController, 
    api::ApiController, 
    auth::AuthController, 
    config::ConfigController, 
    data_dictionary::DataDictionaryController, 
    menu::MenuController, 
    role::RoleController, 
    user::UserController
}};

pub fn init_route(app_state: Arc<AppState>) -> Router {
    let route = Router::new()
        .route("/menu/list", get(MenuController::get_menus))
        .route("/menu/access_list", get(MenuController::get_access_menus))
        .route("/menu/save", post(MenuController::add_menu))
        .route("/api/list", get(ApiController::get_apis))
        .route("/user/list", get(UserController::get_users))
        .route("/user/save", post(UserController::save_user))
        .route("/api/save", post(ApiController::save_api))
        .route("/role/list", get(RoleController::get_roles))
        .route("/role/menu_list", get(RoleController::get_access_menus_with_role))
        .route("/role/save_menu", post(RoleController::save_access_menus_with_role))
        .route("/role/api_list", get(RoleController::get_access_api_with_role))
        .route("/role/save_api", post(RoleController::save_access_api_with_role))
        .route("/access/codes", get(AuthController::get_access_codes))
        .route("/role/save", post(RoleController::save_role))
        .route("/user/info/{id}", get(UserController::get_info))
        .route("/access/user_info", get(UserController::get_access_info))
        .route("/dictionary/list", get(DataDictionaryController::get_data_dictionaries))
        .route("/dictionary/save", post(DataDictionaryController::save_data_dictionary))
        .route("/dictionary/config_list", get(DataDictionaryController::get_data_dictionary_configs))
        .route("/dictionary/save_config", post(DataDictionaryController::save_data_dictionary_config))
        .route("/dictionary/delete_config/{id}", delete(DataDictionaryController::delete_dictionary_config))
        .route("/config/list", get(ConfigController::get_configs))
        .route("/config/save", post(ConfigController::save_config))
        .route("/log/list", get(AdminLogController::get_logs))
        .with_state(app_state);
    Router::new().nest("/system", route)
}