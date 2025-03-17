use sea_orm::entity::prelude::*;
use crate::{app_state::AppState, common::{error::SystemErr, r::R}, 
    entity::system_admin_log::SystemAdminLog, service::{admin_log:: AdminLogService, query::QsQuery}, 
    types::{admin_log::AdminLogPageData, page::{self,PageData}}};
use axum::extract::State;
use std::sync::Arc;

pub struct AdminLogController;
impl AdminLogController {
    pub async fn get_logs(State(app_state): State<Arc<AppState>>, QsQuery(query_data): QsQuery<Arc<AdminLogPageData>>) ->  Result<R<PageData<SystemAdminLog>>, SystemErr>  {
        let total_result = AdminLogService::get_log_total(&app_state, &query_data).await;
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
        let data: Result<Vec<SystemAdminLog>, DbErr> = AdminLogService::get_logs_by_page(&app_state,&query_data ).await;
        match data {
            Ok(datas) => {
                Ok(PageData{
                    items: datas,
                    pagination:page::Pagination { page: query_data.page, page_size: query_data.page_size, total: total, total_page: (total/query_data.page_size as u64) + 1}
                }.into())
            }
            Err(e) => {
                // 返回一个适当的错误响应
                Err(e.into())
            }
        }
        // String::from("Hello, World!")
    }
}

