use serde;
use crate::entity::{system_user::SystemUser, system_user_role::SystemUserRole};
#[derive(serde::Deserialize,Debug)]
pub struct UserQuery {
    pub id: i32
}
#[derive(Clone,serde::Serialize,serde::Deserialize,Debug, Default)]
pub struct UserData {
    pub user: SystemUser,
    pub role_ids: Vec<i32>,
}

#[derive(Clone,serde::Serialize,serde::Deserialize,Debug, Default)]
pub struct ResUserData {
    pub user: SystemUser,
    pub user_roles: Vec<SystemUserRole>,
}

#[derive(Clone,serde::Serialize,serde::Deserialize,Debug, Default)]
pub struct UserQueryPageData {
    #[serde(default)]
    pub real_name: String,
    pub page: u64,
    #[serde(alias = "pageSize")]
    pub page_size: u64,
    #[serde(default)]
    pub status: String
}