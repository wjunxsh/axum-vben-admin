use s_derive::Sea;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use super::default_created_at;
// use s_derive::Sea;
#[derive(Clone,Sea, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "system_user_role", comment = "系统用户角色")]
pub struct Model {
    #[sea_orm(primary_key,auto_increment = true, column_name = "id", comment = "菜单id")]
    pub id: i32,
    #[sea_orm(column_name = "system_role_id", comment = "角色id")]
    pub system_role_id: i32,
    #[sea_orm(column_name = "system_user_id", comment = "用户id")]
    pub system_user_id: i32,
    #[sea_orm(column_name = "created_at", comment = "创建时间")]
    #[serde(default = "default_created_at")]
    pub created_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::system_user::Entity",
        from = "Column::SystemUserId",
        to = "super::system_user::Column::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::system_role::Entity",
        from = "Column::SystemRoleId",
        to = "super::system_role::Column::Id"
    )]
    Role,
}
// impl Related<super::system_user::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::SystemUser.def()
//     }

//     fn via() -> Option<RelationDef> {
//         Some(Relation::SystemRole.def())
//     }
// }

// impl Related<super::system_role::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::SystemRole.def()
//     }

//     fn via() -> Option<RelationDef> {
//         Some(Relation::SystemUser.def())
//     }
// }
// #[derive(Copy, Clone, Debug, EnumIter)]
// pub enum Relation {
//     User,
//     Role,
// }

// impl RelationTrait for Relation {
//     fn def(&self) -> RelationDef {
//         match self {
//             Self::User => Entity::belongs_to(super::system_user::Entity)
//                 .from(Column::SystemUserId)
//                 .to(super::system_user::Column::Id)
//                 .into(),
//             Self::Role => Entity::belongs_to(super::system_role::Entity)
//                 .from(Column::SystemRoleId)
//                 .to(super::system_role::Column::Id)
//                 .into(),
//         }
//     }
// }

impl ActiveModelBehavior for ActiveModel {}