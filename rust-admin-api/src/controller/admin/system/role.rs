use sea_orm::entity::prelude::*;
use crate::{app_state::AppState, common::{error::SystemErr, r::R}, entity:: system_role::SystemRole, service::{ query::QsQuery, role::RoleService}, types::page::{ClaimPage, PageData, Pagination}};
use axum::extract::{Json, State};
use std::sync::Arc;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
#[derive(Debug,Clone, Serialize, Deserialize,Default)]
pub struct RoleQuery {
    pub role_id: i32,
}

#[derive(Debug,Clone, Serialize, Deserialize,Default)]
pub struct RoleMenuData {
    pub system_role_id: i32,
    pub system_menu_ids: Vec<i32>,
}
#[derive(Debug,Clone, Serialize, Deserialize,Default)]
pub struct RoleApiData {
    pub system_role_id: i32,
    pub system_api_ids: Vec<i32>,
}
pub struct RoleController;
impl RoleController {
    pub async fn get_roles(State(app_state): State<Arc<AppState>>, QsQuery(pagination): QsQuery<Arc<ClaimPage>>) ->  Result<R<PageData<SystemRole>>, SystemErr>  {
        let total_result = RoleService::get_roles_total(&app_state).await;
        let total:u64;
        match total_result {
            Ok(count) => {
                total = count;
            }
            Err(e) => {
                // 返回一个适当的错误响应
                return Err(e.into());
            }
        }
        let data: Result<Vec<SystemRole>, DbErr> = RoleService::get_roles_by_page(&app_state, pagination.page, pagination.page_size ).await;
        match data {
            Ok(datas) => {
                Ok(PageData{
                    items: datas,
                    pagination: Pagination { page: pagination.page, page_size: pagination.page_size, total: total, total_page: (total/pagination.page_size as u64) + 1}
                }.into())
            }
            Err(e) => {
                // 返回一个适当的错误响应
                Err(e.into())
            }
        }
        // String::from("Hello, World!")
    }
    pub async fn get_access_menus_with_role(State(app_state): State<Arc<AppState>>, 
        QsQuery(role_query): QsQuery<Arc<RoleQuery>>) 
        ->  Result<R<Vec<i32>>, SystemErr>  {
        let system_role_menu_data = RoleService::get_role_menu_access(&app_state, role_query.role_id).await;
        match system_role_menu_data {
            Ok(datas) => {
                Ok(R::ok(datas.into_iter().map(|x| x.system_menu_id).collect()))
            }
            Err(e) => {
                // 返回一个适当的错误响应
                Err(e.into())
            }
        }
    }
    
    pub async fn save_access_menus_with_role(State(app_state,): State<Arc<AppState>>,Json(role_menu_data): Json<Arc<RoleMenuData>>) ->  Result<R<bool>, SystemErr>  {
        let role_menu_data = (*role_menu_data).clone();
        let role_modal = RoleService::save_role_menu_access(&app_state, role_menu_data.system_role_id, role_menu_data.system_menu_ids).await;
        match role_modal {
            Ok(datas) => {
                Ok(datas.into())
            }
            Err(e) => {
                // 返回一个适当的错误响应
                Err(e.into())
            }
        }
    }
    pub async fn get_access_api_with_role(State(app_state): State<Arc<AppState>>, 
        QsQuery(role_query): QsQuery<Arc<RoleQuery>>) 
        ->  Result<R<Vec<i32>>, SystemErr>  {
        let system_role_api_data = RoleService::get_role_api_access(&app_state, role_query.role_id).await;
        match system_role_api_data {
            Ok(datas) => {
                Ok(R::ok(datas.into_iter().map(|x| x.system_api_id).collect()))
            }
            Err(e) => {
                // 返回一个适当的错误响应
                Err(e.into())
            }
        }
    } 
    pub async fn save_access_api_with_role(State(app_state,): State<Arc<AppState>>,Json(role_api_data): Json<Arc<RoleApiData>>) ->  Result<R<bool>, SystemErr>  {
        let role_api_data = (*role_api_data).clone();
        let role_modal = RoleService::save_role_api_access(&app_state, role_api_data.system_role_id, role_api_data.system_api_ids).await;
        match role_modal {
            Ok(datas) => {
                Ok(datas.into())
            }
            Err(e) => {
                // 返回一个适当的错误响应
                Err(e.into())
            }
        }
    }
    pub async fn save_role(State(app_state,): State<Arc<AppState>>,Json(role): Json<Arc<SystemRole>>) ->  Result<R<SystemRole>, SystemErr>  {
        let role_modal;
        let mut role = (*role).clone();
        if role.id != 0 {
            //查询是否存在
            let role_info = RoleService::get_role_by_id(&app_state, role.id).await?;
            role.created_at = role_info.created_at;  
            role_modal = RoleService::update_role(&app_state, role).await; 
        }else {
            role_modal = RoleService::add_role(&app_state, role).await;
        }
        
        match role_modal {
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

