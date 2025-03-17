use sea_orm::entity::prelude::*;
use sea_orm::QuerySelect;
use super::super::app_state::AppState;
use super::role::RoleService;
use super::user::UserService;
use sea_orm::DbErr;
use sea_orm::Set;

use crate::entity::system_api::SystemApi;
use crate::entity::system_api::{self, Entity as SystemApiEntity};
use crate::types::api::ApiQueryPageData;
pub struct ApiService;

impl ApiService {
    pub async fn get_apis_by_page(app_state: &AppState,query_data: &ApiQueryPageData) -> Result<Vec<SystemApi>, DbErr> {
        let db = &app_state.db;
        let mut handle = SystemApiEntity::find();
        if !query_data.description.is_empty() {
            handle = handle.filter(system_api::Column::Description.like(format!("%{}%",query_data.description)));
        }
        if !query_data.api_key.is_empty() {
            handle = handle.filter(system_api::Column::ApiKey.eq(&query_data.api_key));
        }
        if !query_data.path.is_empty() {
            handle = handle.filter(system_api::Column::Path.eq(&query_data.path));
        }
        let apis = handle
        .limit(query_data.page_size)
        .offset((query_data.page-1)*query_data.page_size)
        .all(db).await?;
        Ok(apis)
    }
    pub async fn get_api_by_path(app_state: &AppState, path:&str) -> Result<SystemApi, DbErr> {
        let db = &app_state.db;
        SystemApiEntity::find()
            .filter(system_api::Column::Path.eq(path))
            .one(db).await?
            .ok_or(DbErr::RecordNotFound("API Not Found".to_string()))
    }
    pub async fn get_apis_total(app_state: &AppState, query_data: &ApiQueryPageData) -> Result<u64, DbErr> {
        let db = &app_state.db;
        let mut handle = SystemApiEntity::find();
        if !query_data.description.is_empty() {
            handle = handle.filter(system_api::Column::Description.like(format!("%{}%",query_data.description)));
        }
        if !query_data.api_key.is_empty() {
            handle = handle.filter(system_api::Column::ApiKey.eq(&query_data.api_key));
        }
        if !query_data.path.is_empty() {
            handle = handle.filter(system_api::Column::Path.eq(&query_data.path));
        }
        handle.count(db).await
    }
    pub async fn get_access_codes(app_state: &AppState, user_id:i32) -> Result<Vec<String>, DbErr> {
        let access_roles = UserService::get_user_role_access(app_state,user_id).await?;
        let role_ids = access_roles.into_iter().map(|user_role| user_role.system_role_id).collect::<Vec<i32>>();
        if role_ids.is_empty(){
            return Ok(vec![]);
        }
        RoleService::get_roles_code_access(app_state, role_ids).await
    }
    pub async fn get_access_codes_all(app_state: &AppState) -> Result<Vec<String>, DbErr> {
        println!("get_access_codes_all");
        let db = &app_state.db;
        let access_api_codes= SystemApiEntity::find()
            .column(system_api::Column::ApiKey)
            .all(db).await?.into_iter().map(|x| x.api_key).collect();
        println!("{:?}",access_api_codes);
        Ok(access_api_codes)
    }
    pub async fn get_api_by_id(app_state: &AppState,id:i32) -> Result<SystemApi, DbErr> {
        let db = &app_state.db;
        SystemApiEntity::find_by_id(id).one(db).await?.ok_or(DbErr::RecordNotFound("API Not Found".to_string()))
    }
    pub async fn get_api_by_key(app_state: &AppState,api_key:&str) -> Result<SystemApi, DbErr> {
        let db = &app_state.db;
        SystemApiEntity::find()
            .filter(system_api::Column::ApiKey.eq(api_key))
            .one(db).await?
            .ok_or(DbErr::RecordNotFound("API Not Found".to_string()))
    }
    pub async fn update_api(app_state: &AppState, api: SystemApi) -> Result<SystemApi, DbErr> {
        let db = &app_state.db;
        let insert_result = system_api::ActiveModel {
            description: Set(api.description),
            path: Set(api.path),
            group: Set(api.group),
            api_key: Set(api.api_key),
            method: Set(api.method),
            created_at: Set(api.created_at),
            updated_at: Set(api.updated_at),
            id: Set(api.id),
        }.update(db).await?;
        Ok(insert_result)
    }
    pub async fn add_api(app_state: &AppState, api: SystemApi) -> Result<SystemApi, DbErr> {
        let db = &app_state.db;
        let insert_result = system_api::ActiveModel {
            description: Set(api.description),
            path: Set(api.path),
            api_key: Set(api.api_key),
            group: Set(api.group),
            method: Set(api.method),
            created_at: Set(api.created_at),
            updated_at: Set(api.updated_at),
            id: Set(api.id),
        }.insert(db).await?;
        Ok(insert_result)
    }
}