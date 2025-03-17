use sea_orm::entity::prelude::*;
use crate::{app_state::AppState, common::{error::SystemErr, r::R}, entity::{system_data_dictionary_config::SystemDataDictionaryConfig, system_data_disctionary::SystemDataDictionary}, service::{data_dictionary::DataDictionaryService, data_dictionary_config::DataDictionaryConfigService, query::QsQuery}, types::{data_dictionary::{ClaimQueryConfigData, ClaimQueryData}, page::{PageData, Pagination}}};
use axum::extract::{Json, Path, State};
use std:: sync::Arc;

pub struct DataDictionaryController;
impl DataDictionaryController {
    pub async fn get_data_dictionaries(State(app_state): State<Arc<AppState>>, QsQuery(query_data): QsQuery<Arc<ClaimQueryData>>) ->  Result<R<PageData<SystemDataDictionary>>, SystemErr>  {
        let total_result = DataDictionaryService::get_total(&app_state, &query_data).await;
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
        let data: Result<Vec<SystemDataDictionary>, DbErr> = DataDictionaryService::get_all_by_page(&app_state, &query_data).await;
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
    
    pub async fn save_data_dictionary(State(app_state,): State<Arc<AppState>>,Json(dictionary_data): Json<Arc<SystemDataDictionary>>) ->  Result<R<SystemDataDictionary>, SystemErr>  {
        let api_modal;
        let mut dictionary_data = (*dictionary_data).clone();
        if dictionary_data.id != 0 {
            //查询是否存在
            let dictionary_info = DataDictionaryService::get_dictionary_by_id(&app_state, dictionary_data.id).await?;
            //查询key值是否存在
            if dictionary_info.code != dictionary_data.code {
                let api_key_exists = DataDictionaryService::get_dictionary_by_code(&app_state, &dictionary_data.code.clone()).await;
                match api_key_exists {
                    Ok(_) => {
                        return Err(SystemErr::Any("code exists".to_string()).into());
                    }
                    Err(_) => {
                        //do nothing
                    }
                }
            }
            dictionary_data.created_at = dictionary_info.created_at;  
            api_modal = DataDictionaryService::update(&app_state, dictionary_data).await; 
        }else {
            let api_key_exists = DataDictionaryService::get_dictionary_by_code(&app_state, &dictionary_data.code.clone()).await;
            match api_key_exists {
                Ok(_) => {
                    return Err(SystemErr::Any("api_key exists".to_string()).into());
                }
                Err(_) => {
                    //do nothing
                }
            }
            api_modal = DataDictionaryService::create(&app_state, dictionary_data).await;
        }
        
        match api_modal {
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
    pub async fn get_data_dictionary_configs(State(app_state): State<Arc<AppState>>, QsQuery(query_data): QsQuery<Arc<ClaimQueryConfigData>>) ->  Result<R<Vec<SystemDataDictionaryConfig>>, SystemErr>  {
        let data: Result<Vec<SystemDataDictionaryConfig>, DbErr> = DataDictionaryConfigService::get_all_configs_by_id(&app_state, query_data.data_dictionary_id ).await;
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
    pub async fn delete_dictionary_config(State(app_state): State<Arc<AppState>>, Path(id): Path<i32>) ->  Result<R<()>, SystemErr>  {
        let data = DataDictionaryConfigService::solf_delete(&app_state,id).await;
        match data {
            Ok(_) => {
                Ok(().into())
            }
            Err(e) => {
                // 返回一个适当的错误响应
                Err(e.into())
            }
        }
        // String::from("Hello, World!")
    }  
    pub async fn save_data_dictionary_config(State(app_state,): State<Arc<AppState>>,Json(dictionary_data_config): Json<Arc<SystemDataDictionaryConfig>>) ->  Result<R<SystemDataDictionaryConfig>, SystemErr>  {
        let api_modal;
        let mut dictionary_data_config = (*dictionary_data_config).clone();
        if dictionary_data_config.id != 0 {
            //查询是否存在
            let dictionary_info = DataDictionaryConfigService::get_dictionary_config_by_id(&app_state, dictionary_data_config.id).await?;
            dictionary_data_config.created_at = dictionary_info.created_at;  
            api_modal = DataDictionaryConfigService::update(&app_state, dictionary_data_config).await; 
        }else {
            api_modal = DataDictionaryConfigService::create(&app_state, dictionary_data_config).await;
        }
        
        match api_modal {
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

