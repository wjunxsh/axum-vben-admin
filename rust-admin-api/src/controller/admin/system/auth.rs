use sea_orm::entity::prelude::*;
use crate::{app_state::AppState, common::{error::SystemErr, r::R}, 
    entity::system_user::SystemUser, service::{api::ApiService, claims::Claims, jwt::{ClaimUser, JwtService}, user::UserService}};
use axum::extract::{Json, State};
use std::sync::Arc;
pub struct AuthController;
#[derive(serde::Deserialize,Debug)]
pub struct LoginData {
    pub user_name: String,
    pub password: String
}

#[derive(serde::Serialize, Debug)]
pub struct LoginResponse {
    pub token: String,
    pub user_name: String,
    pub real_name: String,
    pub avatar: String,
    pub is_init_pwd: i8,
}
impl AuthController {
    pub async fn login(State(app_state): State<Arc<AppState>>, Json(body): Json<LoginData>) -> Result<R<LoginResponse>, SystemErr>  {
        let data: Result<Option<SystemUser>, DbErr> = UserService::get_info_by_user_name(&app_state, &body.user_name).await;
        let pwd_secret = std::env::var("PWD_SECRET").unwrap_or("".to_string());
        match data {
            Ok(datas) => {
                match datas {
                    Some(user) => {
                        let is_init_pwd = user.is_init_pwd;
                        if user.password == format!("{:x}",md5::compute(pwd_secret+&body.password)) {
                            let token = JwtService::create_login_token(user.clone().into());
                            Ok(LoginResponse {
                                token: token,
                                user_name: user.user_name,
                                real_name: user.real_name,
                                avatar: user.avatar,
                                is_init_pwd: is_init_pwd,
                            }.into())
                        } else {
                            Ok(R::err(401, "Error: password error"))
                        }
                    }
                    None => {
                        Ok(R::err(404, "Not Found"))
                    }
                }
            }  
            Err(e) => Err(e.into())
        }
        // String::from("Hello, World!")
    }

    pub async fn get_access_codes(State(app_state): State<Arc<AppState>>, Claims(user): Claims<Arc<ClaimUser>>) -> Result<R<Vec<String>>, SystemErr>  {
        let is_admin = user.is_admin;
        println!("ddddddddddddd");
        if is_admin == 1 {
            let data: Result<Vec<String>, DbErr> = ApiService::get_access_codes_all(&app_state).await;
            match data {
                Ok(datas) => {
                    Ok(datas.into())
                }
                Err(e) => {
                    // 返回一个适当的错误响应
                    Err(e.into())
                }
            }
        }else {
            let data: Result<Vec<String>, DbErr> = ApiService::get_access_codes(&app_state, user.user_id).await;
            match data {
                Ok(datas) => {
                    Ok(datas.into())
                }
                Err(e) => {
                    // 返回一个适当的错误响应
                    Err(e.into())
                }
            }
        }
        
        // String::from("Hello, World!")

    }
}