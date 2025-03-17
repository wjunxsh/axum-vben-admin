use sea_orm::entity::prelude::*;
use crate::{app_state::AppState, common::{error::SystemErr, r::R}, entity::system_api::SystemApi, service::{api::ApiService, query::QsQuery}, types::{api::ApiQueryPageData, page::{PageData, Pagination}}};
use axum::extract::{Json, State};
use std::sync::Arc;

pub struct ApiController;
impl ApiController {
    pub async fn get_apis(State(app_state): State<Arc<AppState>>, QsQuery(query_data): QsQuery<Arc<ApiQueryPageData>>) ->  Result<R<PageData<SystemApi>>, SystemErr>  {
        let total_result = ApiService::get_apis_total(&app_state, &query_data).await;
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
        let data: Result<Vec<SystemApi>, DbErr> = ApiService::get_apis_by_page(&app_state,&query_data ).await;
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
    pub async fn save_api(State(app_state,): State<Arc<AppState>>,Json(api): Json<Arc<SystemApi>>) ->  Result<R<SystemApi>, SystemErr>  {
        let api_modal;
        let mut api = (*api).clone();
        if api.id != 0 {
            //查询是否存在
            let api_info = ApiService::get_api_by_id(&app_state, api.id).await;
            
            
            //将menu_info的 created_at 赋值给menu中的created_at
            let api_info = api_info.unwrap();
            //查询key值是否存在
            if api_info.api_key != api.api_key {
                let api_key_exists = ApiService::get_api_by_key(&app_state, &api.api_key.clone()).await;
                match api_key_exists {
                    Ok(_) => {
                        return Err(SystemErr::Any("api_key exists".to_string()).into());
                    }
                    Err(_) => {
                        //do nothing
                    }
                }
            }
            api.created_at = api_info.created_at;  
            api_modal = ApiService::update_api(&app_state, api).await; 
        }else {
            let api_key_exists = ApiService::get_api_by_key(&app_state, &api.api_key.clone()).await;
            match api_key_exists {
                Ok(_) => {
                    return Err(SystemErr::Any("api_key exists".to_string()).into());
                }
                Err(_) => {
                    //do nothing
                }
            }
            api_modal = ApiService::add_api(&app_state, api).await;
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

