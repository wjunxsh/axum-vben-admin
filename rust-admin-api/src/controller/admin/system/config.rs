use sea_orm::entity::prelude::*;
use crate::{app_state::AppState, common::{error::SystemErr, r::R}, entity:: system_config::SystemConfig, service::{ config::ConfigService, query::QsQuery}, types::{config::ConfigQueryPageData, page::{PageData, Pagination}}};
use axum::extract::{Json, State};
use std::sync::Arc;

pub struct ConfigController;
impl ConfigController {
    pub async fn get_configs(State(app_state): State<Arc<AppState>>, QsQuery(query_data): QsQuery<Arc<ConfigQueryPageData>>) ->  Result<R<PageData<SystemConfig>>, SystemErr>  {
        let total_result = ConfigService::get_configs_total(&app_state, &query_data).await;
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
        let data: Result<Vec<SystemConfig>, DbErr> = ConfigService::get_configs_by_page(&app_state,&query_data ).await;
        match data {
            Ok(datas) => {
                Ok(PageData{
                    items: datas,
                    pagination: Pagination { page: query_data.page, page_size: query_data.page_size, total: total, total_page: (total/query_data.page_size as u64) + 1}
                }.into())
            }
            Err(e) => {
                // 返回一个适当的错误响应
                Err(e.into())
            }
        }
        // String::from("Hello, World!")
    }
    pub async fn save_config(State(app_state,): State<Arc<AppState>>,Json(config_data): Json<Arc<SystemConfig>>) ->  Result<R<SystemConfig>, SystemErr>  {
        let config_modal;
        let mut config_data = (*config_data).clone();
        if config_data.id != 0 {
            //查询是否存在
            let config_info = ConfigService::get_config_by_id(&app_state, config_data.id).await?;
            //查询key值是否存在
            if config_info.key != config_data.key {
                let key_exists = ConfigService::get_config_by_key(&app_state, &config_data.key.clone()).await;
                match key_exists {
                    Ok(_) => {
                        return Err(SystemErr::Any("key exists".to_string()).into());
                    }
                    Err(_) => {
                        //do nothing
                    }
                }
            }
            config_data.created_at = config_info.created_at;  
            config_modal = ConfigService::update_config(&app_state, config_data).await; 
        }else {
            let api_key_exists = ConfigService::get_config_by_key(&app_state, &config_data.key.clone()).await;
            match api_key_exists {
                Ok(_) => {
                    return Err(SystemErr::Any("key exists".to_string()).into());
                }
                Err(_) => {
                    //do nothing
                }
            }
            config_modal = ConfigService::add_config(&app_state, config_data).await;
        }
        
        match config_modal {
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

