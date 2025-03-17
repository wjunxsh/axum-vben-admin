use s_derive::Sea;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use super::default_created_at;
// use s_derive::Sea;
#[derive(Clone,Sea, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "system_data_dictionary", comment = "数据字典表")]
pub struct Model {
    #[sea_orm(primary_key,auto_increment = true, column_name = "id", comment = "ID")]
    pub id: i32,
    #[sea_orm(column_name = "status", comment = "状态")]
    pub status: i8,
    #[sea_orm(column_name = "name", comment = "字典名称")]
    pub name: String,
    #[serde(default)]
    #[sea_orm(column_name = "remark", comment = "备注信息")]
    pub remark: String,
    #[sea_orm(column_name = "code", comment = "字典编码")]
    pub code: String,
    #[serde(default = "default_created_at")]
    #[sea_orm(column_name = "created_at", comment = "创建时间")]
    pub created_at: i64,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}
impl ActiveModelBehavior for ActiveModel {}