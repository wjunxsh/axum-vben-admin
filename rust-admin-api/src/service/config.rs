use sea_orm::entity::prelude::*;
use sea_orm::QuerySelect;
use super::super::app_state::AppState;
use sea_orm::DbErr;
use sea_orm::Set;
use crate::entity::system_config::SystemConfig;
use crate::entity::system_config::{self, Entity as SystemConfigEntity};
use crate::types::config::ConfigQueryPageData;
pub struct ConfigService;

impl ConfigService {
    pub async fn get_configs_by_page(app_state: &AppState,query_data: &ConfigQueryPageData) -> Result<Vec<SystemConfig>, DbErr> {
        let db = &app_state.db;
        let mut handle = SystemConfigEntity::find();
        if query_data.name.len() > 0 {
            handle = handle.filter(system_config::Column::Name.like(format!("%{}%",query_data.name)));
        }
        if query_data.key.len() > 0 {
            handle = handle.filter(system_config::Column::Key.eq(&query_data.key));
        }
        handle.limit(query_data.page_size)
        .offset((query_data.page-1)*query_data.page_size)
        .all(db).await
    }
    pub async fn get_configs_total(app_state: &AppState, query_data: &ConfigQueryPageData) -> Result<u64, DbErr> {
        let db = &app_state.db;
        let mut handle = SystemConfigEntity::find();
        if query_data.name.len() > 0 {
            handle = handle.filter(system_config::Column::Name.like(format!("%{}%",query_data.name)));
        }
        if query_data.key.len() > 0 {
            handle = handle.filter(system_config::Column::Key.eq(&query_data.key));
        }
        handle.count(db).await
    }
    pub async fn get_config_by_id(app_state: &AppState,id:i32) -> Result<SystemConfig, DbErr> {
        let db = &app_state.db;
        SystemConfigEntity::find_by_id(id).one(db).await?
            .ok_or(DbErr::RecordNotFound("Record not found".to_string()))
    }
    pub async fn get_config_by_key(app_state: &AppState,key:&str) -> Result<SystemConfig, DbErr> {
        let db = &app_state.db;
        SystemConfigEntity::find()
            .filter(system_config::Column::Key.eq(key)).one(db).await?
            .ok_or(DbErr::RecordNotFound("Record not found".to_string()))
    }
    pub async fn update_config(app_state: &AppState, config: SystemConfig) -> Result<SystemConfig, DbErr> {
        let db = &app_state.db;
        let update_result = system_config::ActiveModel {
            name: Set(config.name),
            key: Set(config.key),
            value: Set(config.value),
            remark: Set(config.remark),
            created_at: Set(config.created_at),
            id: Set(config.id),
        }.update(db).await?;
        Ok(update_result)
    }
    pub async fn add_config(app_state: &AppState, config: SystemConfig) -> Result<SystemConfig, DbErr> {
        let db = &app_state.db;
        let insert_result = system_config::ActiveModel {
            name: Set(config.name),
            key: Set(config.key),
            value: Set(config.value),
            remark: Set(config.remark),
            created_at: Set(config.created_at),
            id: Set(config.id),
        }.insert(db).await?;
        Ok(insert_result)
    }
}