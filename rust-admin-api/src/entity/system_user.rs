use s_derive::Sea;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use super::default_created_at;
// use s_derive::Sea;
#[derive(Clone,Sea, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "system_user", comment = "用户表")]
pub struct Model {
    #[sea_orm(primary_key,auto_increment = true, column_name = "id", comment = "用户id")]
    pub id: i32,
    #[sea_orm(column_name = "real_name", comment = "真实名称")]
    pub real_name: String,
    #[sea_orm(column_name = "user_name", comment = "登录用户名")]
    pub user_name: String,
    #[serde(default)]
    #[sea_orm(column_name = "password", comment = "密码")]
    pub password: String,
    #[sea_orm(column_name = "status", comment = "状态")]
    pub status: String,
    #[serde(default)]
    #[sea_orm(column_name = "avatar", comment = "avatar登录头像")]
    pub avatar: String,
    #[serde(default = "default_created_at")]
    #[sea_orm(column_name = "created_at", comment = "创建时间")]
    pub created_at: i64,
    #[serde(default = "default_created_at")]
    #[sea_orm(column_name = "updated_at", comment = "更新时间")]
    pub updated_at: i64,
    #[serde(default)]
    #[sea_orm(column_name = "is_init_pwd", comment = "是否是初始化密码")]
    pub is_init_pwd: i8,
    #[sea_orm(column_name = "is_admin", comment = "是否是管理员")]
    pub is_admin: i8,
    #[serde(default)]
    #[sea_orm(column_name = "last_update_password_time", comment = "最后更新密码时间")]
    pub last_update_password_time: i64,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl Related<super::system_role::Entity> for Entity {
    fn to() -> RelationDef {
        super::system_user_role::Relation::Role.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::system_user_role::Relation::User.def().rev())
    }
}
impl ActiveModelBehavior for ActiveModel {}