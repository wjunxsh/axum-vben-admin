use sea_orm::entity::prelude::*;
use sea_orm::QuerySelect;
use super::super::app_state::AppState;
use sea_orm::DbErr;
use sea_orm::Set;
use crate::entity::system_data_disctionary::{self,SystemDataDictionary, Entity as SystemDataDictionaryEntity};
use crate::types::data_dictionary::ClaimQueryData;
pub struct DataDictionaryService;

impl DataDictionaryService {
    pub async fn get_all_by_page(app_state: &AppState,query_data:&ClaimQueryData) -> Result<Vec<SystemDataDictionary>, DbErr> {
        let db = &app_state.db;
        let mut handle = SystemDataDictionaryEntity::find();
        //如果名称不为空
        if query_data.name.len() > 0 {
            handle = handle.filter(system_data_disctionary::Column::Name.like(format!("%{}%",query_data.name)));
        }
        if query_data.status != -1 {
            handle = handle.filter(system_data_disctionary::Column::Status.eq(query_data.status));
        }
        handle.limit(query_data.page_size)
            .offset((query_data.page-1)*query_data.page_size)
            .all(db).await
    }
    pub async fn get_dictionary_by_id(app_state: &AppState,id:i32) -> Result<SystemDataDictionary, DbErr> {
        let db = &app_state.db;
        let info = SystemDataDictionaryEntity::find_by_id(id).one(db).await?
            .expect("Record not found");
        Ok(info)
    }
    pub async fn get_dictionary_by_code(app_state: &AppState,code:&str) -> Result<SystemDataDictionary, DbErr> {
        let db = &app_state.db;
        //如果查不到数据则返回error信息
        let info = SystemDataDictionaryEntity::find().filter(system_data_disctionary::Column::Code.eq(code)).one(db).await?;
        if info.is_none() {
            return Err(DbErr::RecordNotFound("Record not found".to_string()));
        }
        Ok(info.unwrap())
    }
    pub async fn get_total(app_state: &AppState, query_data:&ClaimQueryData) -> Result<u64, DbErr> {
        let db = &app_state.db;
        let mut handle = SystemDataDictionaryEntity::find();
        //如果名称不为空
        if query_data.name.len() > 0 {
            handle = handle.filter(system_data_disctionary::Column::Name.like(format!("%{}%",query_data.name)));
        }
        if query_data.status != -1 {
            handle = handle.filter(system_data_disctionary::Column::Status.eq(query_data.status));
        }
        let total = handle.count(db).await.unwrap();
        Ok(total)
    }
    pub async fn update(app_state: &AppState, dictionary_data: SystemDataDictionary) -> Result<SystemDataDictionary, DbErr> {
        let db = &app_state.db;
        let insert_result = system_data_disctionary::ActiveModel {
            name: Set(dictionary_data.name.to_owned()),
            code: Set(dictionary_data.code.to_owned()),
            created_at: Set(dictionary_data.created_at.to_owned()),
            remark: Set(dictionary_data.remark.to_owned()),
            status: Set(dictionary_data.status.to_owned()),
            id: Set(dictionary_data.id.to_owned()),
        }.save(db).await?;
        let menu = SystemDataDictionaryEntity::find_by_id(insert_result.id.unwrap())
        .one(db)
        .await?
        .expect("Record not found");
        Ok(menu)
    }
    pub async fn create(app_state: &AppState, dictionary_data: SystemDataDictionary) -> Result<SystemDataDictionary, DbErr> {
        let db = &app_state.db;
        let insert_result = system_data_disctionary::ActiveModel {
            name: Set(dictionary_data.name.to_owned()),
            code: Set(dictionary_data.code.to_owned()),
            created_at: Set(dictionary_data.created_at.to_owned()),
            remark: Set(dictionary_data.remark.to_owned()),
            status: Set(dictionary_data.status.to_owned()),
            id: Set(dictionary_data.id.to_owned()),
        }.insert(db).await?;
        Ok(insert_result)
    }
}