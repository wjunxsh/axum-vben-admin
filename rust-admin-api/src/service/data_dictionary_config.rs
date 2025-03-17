use sea_orm::entity::prelude::*;
use super::super::app_state::AppState;
use sea_orm::DbErr;
use sea_orm::Set;
use crate::entity::system_data_dictionary_config::{self,SystemDataDictionaryConfig, Entity as SystemDataDictionaryConfigEntity};
pub struct DataDictionaryConfigService;
impl DataDictionaryConfigService {
    pub async fn get_all_configs_by_id(app_state: &AppState,data_dictionary_id:i32) -> Result<Vec<SystemDataDictionaryConfig>, DbErr> {
        let db = &app_state.db;
        SystemDataDictionaryConfigEntity::find()
            .filter(system_data_dictionary_config::Column::DataDictionaryId.eq(data_dictionary_id))
            .filter(system_data_dictionary_config::Column::IsDel.eq(0))
            .all(db).await
    }
    pub async fn get_dictionary_config_by_id(app_state: &AppState,id:i32) -> Result<SystemDataDictionaryConfig, DbErr> {
        let db = &app_state.db;
        SystemDataDictionaryConfigEntity::find_by_id(id).one(db).await?
            .ok_or(DbErr::RecordNotFound("Record not found".to_string()))
    }
    /**
     * 根据id删除数据字典配置
     */
    pub async fn solf_delete(app_state: &AppState,id:i32) -> Result<(), DbErr> {
        let db = &app_state.db;
        let data = SystemDataDictionaryConfigEntity::find()
        .filter(system_data_dictionary_config::Column::Id.eq(id))
        .one(db).await?;
        if let Some(config) = data {
            let mut config_model: system_data_dictionary_config::ActiveModel = config.into();
            config_model.is_del = Set(1);
            config_model.update(db).await?;
            Ok(())
        } else {
            Err(DbErr::RecordNotFound("Record not found".to_string()))
        }
    
    }
    pub async fn update(app_state: &AppState, dictionary_data_config: SystemDataDictionaryConfig) -> Result<SystemDataDictionaryConfig, DbErr> {
        let db = &app_state.db;
        let update_result = system_data_dictionary_config::ActiveModel {
            id: Set(dictionary_data_config.id.to_owned()),
            is_del: Set(dictionary_data_config.is_del.to_owned()),
            status: Set(dictionary_data_config.status.to_owned()),
            sort: Set(dictionary_data_config.sort.to_owned()),
            value: Set(dictionary_data_config.value.to_owned()),
            label: Set(dictionary_data_config.label.to_owned()),
            data_dictionary_id: Set(dictionary_data_config.data_dictionary_id.to_owned()),
            created_at: Set(dictionary_data_config.created_at.to_owned()),
            remark: Set(dictionary_data_config.remark.to_owned()),
        }.update(db).await?;
        Ok(update_result)
    }
    pub async fn create(app_state: &AppState, dictionary_data_config: SystemDataDictionaryConfig) -> Result<SystemDataDictionaryConfig, DbErr> {
        let db = &app_state.db;
        let insert_result = system_data_dictionary_config::ActiveModel {
                id: Set(dictionary_data_config.id.to_owned()),
                is_del: Set(dictionary_data_config.is_del.to_owned()),
                status: Set(dictionary_data_config.status.to_owned()),
                sort: Set(dictionary_data_config.sort.to_owned()),
                value: Set(dictionary_data_config.value.to_owned()),
                label: Set(dictionary_data_config.label.to_owned()),
                data_dictionary_id: Set(dictionary_data_config.data_dictionary_id.to_owned()),
                created_at: Set(dictionary_data_config.created_at.to_owned()),
                remark: Set(dictionary_data_config.remark.to_owned()),
            }.insert(db).await?;
        Ok(insert_result)
    }
}