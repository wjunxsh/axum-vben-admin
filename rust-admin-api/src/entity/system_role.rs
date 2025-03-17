use s_derive::Sea;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use super::default_created_at;
// use s_derive::Sea;
#[derive(Clone,Sea, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "system_role", comment = "系统角色")]
pub struct Model {
    #[sea_orm(primary_key,auto_increment = true, column_name = "id", comment = "角色id")]
    pub id: i32,
    #[sea_orm(column_name = "role_name", comment = "角色名称")]
    pub role_name: String,
    #[sea_orm(column_name = "status", comment = "状态")]
    pub status: String,
    #[sea_orm(column_name = "created_at", comment = "创建时间")]
    #[serde(default = "default_created_at")]
    pub created_at: i64,
    #[serde(default = "default_created_at")]
    #[sea_orm(column_name = "updated_at", comment = "更新时间")]
    pub updated_at: i64,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl Related<super::system_user::Entity> for Entity {
    fn to() -> RelationDef {
        super::system_user_role::Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::system_user_role::Relation::Role.def().rev())
    }
}

impl Related<super::system_menu::Entity> for Entity {
    fn to() -> RelationDef {
        super::system_role_menu::Relation::Menu.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::system_role_menu::Relation::Role.def().rev())
    }
}

impl Related<super::system_api::Entity> for Entity {
    fn to() -> RelationDef {
        super::system_role_api::Relation::Api.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::system_role_api::Relation::Role.def().rev())
    }
}


impl ActiveModelBehavior for ActiveModel {}