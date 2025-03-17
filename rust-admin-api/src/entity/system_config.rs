use s_derive::Sea;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use super::default_created_at;
// use s_derive::Sea;
#[derive(Clone,Sea, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "system_config", comment = "系统配置表")]
pub struct Model {
    #[sea_orm(primary_key,auto_increment = true, column_name = "id", comment = "ID")]
    pub id: i32,
    #[sea_orm(column_name = "name", comment = "参数名")]
    pub name: String,
    #[sea_orm(column_name = "key", comment = "参数键")]
    pub key: String,
    #[sea_orm(column_name = "value", comment = "赞数值")]
    pub value: String,
    #[sea_orm(column_name = "remark", comment = "备注")]
    pub remark: String,
    #[sea_orm(column_name = "created_at", comment = "创建时间")]
    #[serde(default = "default_created_at")]
    pub created_at: i64,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}
impl ActiveModelBehavior for ActiveModel {}