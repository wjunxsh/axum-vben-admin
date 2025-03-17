use chrono::Utc;
use sea_orm:: TransactionTrait;
use sea_orm::{entity::prelude::*, QuerySelect};
use serde::{Deserialize, Serialize};
use super::super::app_state::AppState;
use super::role::RoleService;
use sea_orm::DbErr;
use sea_orm::Set;
use crate::entity::system_role::{self, SystemRole};
use crate::entity::{system_user_role,system_user_role::{Entity as SystemUserRoleEntity, SystemUserRole}};
use crate::entity::{system_user,system_user::{Entity as SystemUserEntity, SystemUser}};
use crate::types::user::UserQueryPageData;
pub struct UserService;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserWithRoles {
    pub id: i32,
    pub user_name: String,
    pub real_name: String,
    pub status: String,
    pub is_init_pwd: i8,
    pub avatar: String,
    pub last_update_password_time: i64,
    pub created_at: i64,
    pub updated_at: i64,
    pub is_admin: i8,
    pub roles: Vec<SystemRole>,
}
impl UserService {
    pub async fn get_info(app_state: &AppState,user_id:i32) -> Result<Option<SystemUser>, DbErr> {
        let db = &app_state.db;
        SystemUserEntity::find().filter(system_user::Column::Id.eq(user_id)).one(db).await
    }
    pub async fn get_user_by_page(app_state: &AppState,user_query_data: &UserQueryPageData) -> Result<Vec<UserWithRoles>, DbErr> {
        let db: &DatabaseConnection = &app_state.db;
        let mut handle = SystemUserEntity::find();
        if user_query_data.real_name.len() > 0 {
            handle = handle.filter(system_user::Column::RealName.like(format!("%{}%",user_query_data.real_name)));
        }
        if user_query_data.status.len() > 0 {
            handle = handle.filter(system_user::Column::Status.eq(user_query_data.status.clone()));
        }
        let data = handle
            .find_with_related(system_role::Entity)
            .limit(user_query_data.page_size)
            .offset((user_query_data.page - 1) * user_query_data.page_size) // 为 system_user 表指定一个别名
            .all(db) // 执行查询
            .await?;
        let users = data
            .into_iter()
            .map(|(user, roles)| {
                let roles = roles.into_iter().map(|role| role.into()).collect();
                UserWithRoles {
                    id: user.id,
                    user_name: user.user_name,
                    real_name: user.real_name,
                    status: user.status,
                    is_init_pwd: user.is_init_pwd,
                    avatar: user.avatar,
                    is_admin: user.is_admin,
                    last_update_password_time: user.last_update_password_time,
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                    roles,
                }
            })
            .collect();
        Ok(users)
    }
    pub async fn get_user_total(app_state: &AppState,user_query_data: &UserQueryPageData) -> Result<u64, DbErr> {
        let db = &app_state.db;
        let mut handle = SystemUserEntity::find();
        if user_query_data.real_name.len() > 0 {
            handle = handle.filter(system_user::Column::RealName.like(format!("%{}%",user_query_data.real_name)));
        }
        if user_query_data.status.len() > 0 {
            handle = handle.filter(system_user::Column::Status.eq(user_query_data.status.clone()));
        }
        handle.count(db).await
    }
    pub async fn get_info_by_user_name(app_state: &AppState,user_name:&str) -> Result<Option<SystemUser>, DbErr> {
        let db: &DatabaseConnection = &app_state.db;
        SystemUserEntity::find().filter(system_user::Column::UserName.eq(user_name)).one(db).await
    }
    pub async fn get_user_by_id(app_state: &AppState,id:i32) -> Result<SystemUser, DbErr> {
        let db = &app_state.db;
        SystemUserEntity::find_by_id(id).one(db).await?
            .ok_or(DbErr::RecordNotFound("Record not found".to_string()))
    }
    pub async fn check_user_api_access(app_state: &AppState, user_id:i32,path: &str) -> Result<bool, DbErr> {
        let access_roles = UserService::get_user_role_access(app_state,user_id).await?;
        let role_ids = access_roles.into_iter().map(|user_role| user_role.system_role_id).collect::<Vec<i32>>();
        if role_ids.is_empty() {
            return Ok(false);
        }
        RoleService::check_roles_api_access(app_state, role_ids, path).await
    }
    pub async fn get_user_role_access(app_state: &AppState,user_id:i32) -> Result<Vec<SystemUserRole>, DbErr> {
        let db = &app_state.db;
        SystemUserRoleEntity::find()
            .filter(system_user_role::Column::SystemUserId.eq(user_id)).all(db)
            .await
    }
    pub async fn update_user(app_state: &AppState, user: SystemUser, role_ids:Vec<i32>) -> Result<SystemUser, DbErr> {
        let db = &app_state.db;

        //查询角色存在的所有菜单
        let exists_user_roles = UserService::get_user_role_access(app_state,user.id).await?;
        //批量保存角色和菜单关系
        let mut user_roles = Vec::new();
        for role_id in role_ids.clone() {
            //判断是否存在
            let exists = exists_user_roles.iter().find(|&x| x.system_role_id == role_id);
            if exists.is_some() {
                continue;
            }
            let user_role = system_user_role::ActiveModel {
                id:Set(0),
                system_role_id: Set(role_id),
                system_user_id: Set(user.id),
                created_at:Set(Utc::now().timestamp())
            };
            user_roles.push(user_role);
        }
        //如果老的值不存在新的值中，则删除
        let mut delete_user_roles = Vec::new();
        for exists_user_role in exists_user_roles {
            let exists = role_ids.iter().find(|&x| *x == exists_user_role.system_role_id);
            match exists {
                Some(_) => {
                    continue;
                }
                None => {
                    delete_user_roles.push(exists_user_role.system_role_id);
                }
            }   
        };
        let txn = db.begin().await?;
        //删除原有的关系
        println!("delete_user_roles:{:?}",delete_user_roles);
        if delete_user_roles.len() > 0 {
            SystemUserRoleEntity::delete_many().filter(system_user_role::Column::SystemRoleId.is_in(delete_user_roles)).exec(&txn).await?;
        }
        println!("user_roles:{:?}",user_roles);
        if user_roles.len() > 0 {
            SystemUserRoleEntity::insert_many(user_roles).exec(&txn).await?;
        }
        let update_result = system_user::ActiveModel {
            user_name: Set(user.user_name),
            real_name: Set(user.real_name),
            password: Set(user.password),
            status: Set(user.status),
            is_init_pwd: Set(user.is_init_pwd),
            avatar: Set(user.avatar),
            last_update_password_time: Set(user.last_update_password_time),
            created_at: Set(user.created_at),
            updated_at: Set(user.updated_at),
            is_admin: Set(user.is_admin),
            id: Set(user.id),
        }.update(db).await?;
        txn.commit().await?;
        Ok(update_result)
    }
    pub async fn add_user(app_state: &AppState, user: SystemUser,role_ids: Vec<i32>) -> Result<SystemUser, DbErr> {
        let db = &app_state.db;

        let mut user_roles = Vec::new();
        let txn = db.begin().await?;
        let insert_result = system_user::ActiveModel {
            user_name: Set(user.user_name),
            real_name: Set(user.real_name),
            password: Set(user.password),
            status: Set(user.status),
            is_init_pwd: Set(user.is_init_pwd),
            avatar: Set(user.avatar),
            last_update_password_time: Set(user.last_update_password_time),
            created_at: Set(user.created_at),
            updated_at: Set(user.updated_at),
            is_admin: Set(user.is_admin),
            id: Set(user.id),
        }.insert(db).await?;
        for role_id in role_ids.clone() {
            let user_role = system_user_role::ActiveModel {
                id:Set(0),
                system_role_id: Set(role_id),
                system_user_id: Set(insert_result.id),
                created_at:Set(Utc::now().timestamp())
            };
            user_roles.push(user_role);
        }
        if user_roles.len() > 0 {
            SystemUserRoleEntity::insert_many(user_roles).exec(&txn).await?;
        }
        Ok(insert_result)
    }
}