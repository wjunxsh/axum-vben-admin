use sea_orm::entity::prelude::*;
use sea_orm::QuerySelect;
use super::super::app_state::AppState;
use sea_orm::DbErr;
use sea_orm::Set;

use crate::entity::system_admin_log;
use crate::entity::system_admin_log::{SystemAdminLog,Entity as SystemAdminLogEntity};
use crate::types::admin_log::AdminLogPageData;
pub struct AdminLogService;

impl AdminLogService {
    pub async fn get_logs_by_page(app_state: &AppState,query_data: &AdminLogPageData) -> Result<Vec<SystemAdminLog>, DbErr> {
        let db = &app_state.db;
        let mut handle = SystemAdminLogEntity::find();
        if query_data.operate_name.len() > 0 {
            handle = handle.filter(system_admin_log::Column::OperateName.like(format!("%{}%",query_data.operate_name)));
        }
        if query_data.path.len() > 0 {
            handle = handle.filter(system_admin_log::Column::Path.eq(&query_data.path));
        }
        if query_data.method.len() > 0 {
            handle = handle.filter(system_admin_log::Column::Method.eq(&query_data.method));
        }
        if query_data.start_time > 0 {
            handle = handle.filter(system_admin_log::Column::CreatedAt.gte(query_data.start_time));
        }
        if query_data.end_time > 0 {
            handle = handle.filter(system_admin_log::Column::CreatedAt.lte(query_data.end_time));
        }
        let logs = handle
        .limit(query_data.page_size)
        .offset((query_data.page-1)*query_data.page_size)
        .all(db).await.unwrap();
        Ok(logs)
    }
    pub async fn get_log_total(app_state: &AppState, query_data: &AdminLogPageData) -> Result<u64, DbErr> {
        let db = &app_state.db;
        let mut handle = SystemAdminLogEntity::find();
        if query_data.operate_name.len() > 0 {
            handle = handle.filter(system_admin_log::Column::OperateName.like(format!("%{}%",query_data.operate_name)));
        }
        if query_data.path.len() > 0 {
            handle = handle.filter(system_admin_log::Column::Path.eq(&query_data.path));
        }
        if query_data.method.len() > 0 {
            handle = handle.filter(system_admin_log::Column::Method.eq(&query_data.method));
        }
        if query_data.start_time > 0 {
            handle = handle.filter(system_admin_log::Column::CreatedAt.gte(query_data.start_time));
        }
        if query_data.end_time > 0 {
            handle = handle.filter(system_admin_log::Column::CreatedAt.lte(query_data.end_time));
        }
        let total =  handle.count(db).await.unwrap();
        Ok(total)
    }
    pub async fn add_log(app_state: &AppState, admin_log: SystemAdminLog) -> Result<SystemAdminLog, DbErr> {
        let db = &app_state.db;
        let insert_result = system_admin_log::ActiveModel {
            system_user_id: Set(admin_log.system_user_id),
            system_real_name: Set(admin_log.system_real_name),
            path: Set(admin_log.path),
            method: Set(admin_log.method),
            ip: Set(admin_log.ip),
            created_at: Set(admin_log.created_at),
            user_agent: Set(admin_log.user_agent),
            request_url: Set(admin_log.request_url),
            request_body: Set(admin_log.request_body),
            response_data: Set(admin_log.response_data),
            response_status: Set(admin_log.response_status),
            response_time_ms: Set(admin_log.response_time_ms),
            operate_name: Set(admin_log.operate_name),
            error_message: Set(admin_log.error_message),
            id: Set(admin_log.id),
        }.insert(db).await?;
        Ok(insert_result)
    }
}