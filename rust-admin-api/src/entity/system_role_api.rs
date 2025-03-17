use s_derive::Sea;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use super::default_created_at;
// use s_derive::Sea;
#[derive(Clone,Sea, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "system_role_api", comment = "系统角色API权限")]
pub struct Model {
    #[sea_orm(primary_key,auto_increment = true, column_name = "id", comment = "菜单id")]
    pub id: i32,
    #[sea_orm(column_name = "system_role_id", comment = "角色id")]
    pub system_role_id: i32,
    #[sea_orm(column_name = "system_api_id", comment = "菜单id")]
    pub system_api_id: i32,
    #[sea_orm(column_name = "created_at", comment = "创建时间")]
    #[serde(default = "default_created_at")]
    pub created_at: i64,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::system_api::Entity",
        from = "Column::SystemApiId",
        to = "super::system_api::Column::Id"
    )]
    Api,
    #[sea_orm(
        belongs_to = "super::system_role::Entity",
        from = "Column::SystemRoleId",
        to = "super::system_role::Column::Id"
    )]
    Role,
}

impl ActiveModelBehavior for ActiveModel {}