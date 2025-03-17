use sea_orm::entity::prelude::*;
use crate::{app_state::AppState, common::{error::SystemErr, r::R}, entity::system_menu::SystemMenu, service::{claims::Claims, jwt::ClaimUser, menu::MenuService}};
use axum::extract::{Json, State};
use std::sync::Arc;
pub struct MenuController;
impl MenuController {
    pub async fn get_menus(State(app_state): State<Arc<AppState>>) ->  Result<R<Vec<SystemMenu>>, SystemErr>  {
        let data: Result<Vec<SystemMenu>, DbErr> = MenuService::get_menus(&app_state).await;
        match data {
            Ok(datas) => {
                Ok(datas.into())
            }
            Err(e) => {
                // 返回一个适当的错误响应
                Err(e.into())
            }
        }
        // String::from("Hello, World!")
    }  
    pub async fn get_access_menus(State(app_state): State<Arc<AppState>>, Claims(user): Claims<Arc<ClaimUser>>) ->  Result<R<Vec<SystemMenu>>, SystemErr>  {
        let data: Result<Vec<SystemMenu>, DbErr>;
        if user.is_admin == 1 {
            data = MenuService::get_menus(&app_state).await;
        }else {
            data = MenuService::get_access_menus(&app_state, user.user_id).await;
        }
        match data {
            Ok(datas) => {
                Ok(datas.into())
            }
            Err(e) => {
                // 返回一个适当的错误响应
                Err(e.into())
            }
        }
        // String::from("Hello, World!")
    }
    pub async fn add_menu(State(app_state,): State<Arc<AppState>>,Json(menu): Json<Arc<SystemMenu>>) ->  Result<R<SystemMenu>, SystemErr>  {
        let menu_modal;
        let mut menu = (*menu).clone();
        if menu.id != 0 {
            //查询是否存在
            let menu_info = MenuService::get_menu(&app_state, menu.id).await?;
            menu.created_at = menu_info.created_at;  
            menu_modal = MenuService::update_menu(&app_state, menu).await; 
        }else {
            menu_modal = MenuService::add_menu(&app_state, menu).await;
        }
        
        match menu_modal {
            Ok(datas) => {
                Ok(datas.into())
            }
            Err(e) => {
                // 返回一个适当的错误响应
                Err(e.into())
            }
        }
        // String::from("Hello, World!")
    }
}

