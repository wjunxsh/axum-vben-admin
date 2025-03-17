use s_derive::Sea;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use super::default_created_at;
// use s_derive::Sea;
#[derive(Clone,Sea, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "system_api", comment = "系统api接口")]
pub struct Model {
    #[sea_orm(primary_key,auto_increment = true, column_name = "id", comment = "菜单id")]
    pub id: i32,
    #[sea_orm(column_name = "method", comment = "方法")]
    pub method: String,
    #[sea_orm(column_name = "api_key", comment = "权限标识")]
    pub api_key: String,
    #[sea_orm(column_name = "group", comment = "分组")]
    pub group: String,
    #[sea_orm(column_name = "description", comment = "描述")]
    pub description: String,
    #[sea_orm(column_name = "path", comment = "路径")]
    pub path: String,
    #[sea_orm(column_name = "created_at", comment = "创建时间")]
    #[serde(default = "default_created_at")]
    pub created_at: i64,
    #[serde(default = "default_created_at")]
    #[sea_orm(column_name = "updated_at", comment = "更新时间")]
    pub updated_at: i64,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl Related<super::system_role::Entity> for Entity {
    fn to() -> RelationDef {
        super::system_role_api::Relation::Role.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::system_role_api::Relation::Api.def().rev())
    }
}
impl ActiveModelBehavior for ActiveModel {}