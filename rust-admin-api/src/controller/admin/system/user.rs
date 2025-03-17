use chrono::Utc;
use sea_orm::entity::prelude::*;
use crate::{app_state::AppState, common::{error::SystemErr, r::R}, entity::system_user:: SystemUser, service::{ claims::Claims, jwt::ClaimUser, query::QsQuery, user::{UserService, UserWithRoles}}, types::{page::{PageData, Pagination}, user::{UserData, UserQueryPageData}}};
use axum::extract::{Path, State, Json};
use std::sync::Arc;
use md5;
pub struct UserController;

impl UserController {
    pub async fn get_info(State(app_state): State<Arc<AppState>>, Path(id): Path<i32>) -> Result<R<SystemUser>, SystemErr>  {
        let data: Result<Option<SystemUser>, DbErr> = UserService::get_info(&app_state, id).await;
        match data {
            Ok(datas) => {
                let user = datas.unwrap();
                Ok( user.into())
            }  
            Err(e) => {
                // 返回一个适当的错误响应
                Err(e.into())
            }
        }
        // String::from("Hello, World!")
    }
    pub async fn get_users(State(app_state): State<Arc<AppState>>, QsQuery(user_query_data): QsQuery<Arc<UserQueryPageData>>) ->  Result<R<PageData<UserWithRoles>>, SystemErr>  {
        let total_result = UserService::get_user_total(&app_state, &user_query_data).await;
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
        let data:Result<Vec<UserWithRoles>, DbErr>= UserService::get_user_by_page(&app_state, &user_query_data).await;
        match data {
            Ok(datas) => {
                Ok(PageData{
                    items: datas,
                    pagination: Pagination { page: user_query_data.page, page_size: user_query_data.page_size, total: total, total_page: (total/user_query_data.page_size as u64) + 1}
                }.into())
            }
            Err(e) => {
                // 返回一个适当的错误响应
                Err(e.into())
            }
        }
        // String::from("Hello, World!")
    }
    pub async fn save_user(State(app_state,): State<Arc<AppState>>,Json(user_data): Json<Arc<UserData>>) ->  Result<R<SystemUser>, SystemErr>  {
        let user_modal;
        let user_data = (*user_data).clone();
        let user = user_data.user;
        let role_ids = user_data.role_ids;
        if user.id != 0 {
            //查询是否存在
            let mut user_info = UserService::get_user_by_id(&app_state, user.id).await?;
            //user.created_at = user_info.created_at; 
            user_info.user_name = user.user_name.clone();
            user_info.real_name = user.real_name.clone();
            user_info.status = user.status.clone();
            //从env中获取密码密钥,如果未配置则默认为空
            
            let pwd_secret = std::env::var("PWD_SECRET").unwrap_or("".to_string());
            //如果密码不为空则将密码存储
            if user.password.len() > 0 {
                //md5使用密钥加密
                let before_md5 = pwd_secret+&user.password.clone();
                user_info.password = format!("{:x}", md5::compute(&before_md5));
                //user_info.password = pwd_secret+user.password.clone(); 
                user_info.is_init_pwd = 1;
                user_info.last_update_password_time  = Utc::now().timestamp();
            }
            //如果avatar不为空则保存avatar
            if user.avatar.len() > 0 {
                user_info.avatar = user.avatar.clone();
            }
            user_info.updated_at = Utc::now().timestamp();
            user_modal = UserService::update_user(&app_state, user_info, role_ids).await; 
        }else {
            user_modal = UserService::add_user(&app_state, user, role_ids).await;
        }
        
        match user_modal {
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
    pub async fn get_access_info(State(app_state): State<Arc<AppState>>, Claims(user): Claims<Arc<ClaimUser>>) -> Result<R<SystemUser>, SystemErr>  {
        let data = UserService::get_info(&app_state, user.user_id).await?;
        match data {
            Some(user) => {
                Ok(user.into())
            }
            None => {
                // 返回一个适当的错误响应
                Err(SystemErr::Any("user not found".to_string()).into())
            }
        }
    }
}