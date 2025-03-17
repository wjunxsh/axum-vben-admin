use s_derive::Sea;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use super::default_created_at;
// use s_derive::Sea;
#[derive(Clone,Sea, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "system_menu", comment = "系统菜单表")]
pub struct Model {
    #[sea_orm(primary_key,auto_increment = true, column_name = "id", comment = "菜单id")]
    pub id: i32,
    #[sea_orm(column_name = "parent_id", comment = "父菜单id")]
    pub parent_id: i32,
    #[sea_orm(column_name = "name", comment = "菜单名称")]
    pub name: String,
    #[sea_orm(column_name = "title", comment = "菜单标题")]
    pub title: String,
    #[sea_orm(column_name = "path", comment = "路径")]
    pub path: String,
    #[sea_orm(column_name = "component", comment = "组件")]
    pub component: String,
    #[sea_orm(column_name = "link", comment = "链接地址")]
    pub link: String,
    #[sea_orm(column_name = "created_at", comment = "创建时间")]
    #[serde(default = "default_created_at")]
    pub created_at: i64,
    #[serde(default = "default_created_at")]
    #[sea_orm(column_name = "updated_at", comment = "更新时间")]
    pub updated_at: i64,
    #[sea_orm(column_name = "meta", comment = "meta数据")]
    pub meta: Json,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::system_role::Entity> for Entity {
    fn to() -> RelationDef {
        super::system_role_menu::Relation::Menu.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::system_role_menu::Relation::Role.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}