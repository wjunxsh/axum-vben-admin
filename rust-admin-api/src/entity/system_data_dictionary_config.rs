use s_derive::Sea;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use super::default_created_at;
use super::default_zero;
// use s_derive::Sea;
#[derive(Clone,Sea, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "system_data_dictionary_config", comment = "字典配置表")]
pub struct Model {
    #[sea_orm(primary_key,auto_increment = true, column_name = "id", comment = "用户id")]
    pub id: i32,
    #[sea_orm(column_name = "data_dictionary_id", comment = "数据字典id")]
    pub data_dictionary_id: i32,
    #[sea_orm(column_name = "label", comment = "配置文案")]
    pub label: String,
    #[serde(default)]
    #[sea_orm(column_name = "value", comment = "配置项的值")]
    pub value: String,
    #[sea_orm(column_name = "remark", comment = "备注")]
    pub remark: String,
    #[serde(default = "default_created_at")]
    #[sea_orm(column_name = "created_at", comment = "创建时间")]
    pub created_at: i64,
    #[sea_orm(column_name = "sort", comment = "排序")]
    pub sort: i32,
    #[sea_orm(column_name = "is_del", comment = "是否删除")]
    #[serde(default = "default_zero")]
    pub is_del: i8,
    #[sea_orm(column_name = "status", comment = "状态")]
    pub status: i8,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}
impl ActiveModelBehavior for ActiveModel {}