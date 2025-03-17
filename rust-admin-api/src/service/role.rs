use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::QuerySelect;
use sea_orm::TransactionTrait;
use super::super::app_state::AppState;
use sea_orm::DbErr;
use sea_orm::Set;

use crate::entity::system_api;
use crate::entity::system_api::Entity as SystemApiEntity;
use crate::entity::system_role::SystemRole;
use crate::entity::system_role::{self, Entity as SystemRoleEntity};
use crate::entity::system_role_api::SystemRoleApi;
use crate::entity::system_role_menu:: {self, Entity as SystemRoleMenuEntity};
use crate::entity::system_role_api:: {self, Entity as SystemRoleApiEntity};
use crate::entity::system_role_menu::SystemRoleMenu;
pub struct RoleService;

impl RoleService {
    pub async fn get_roles_by_page(app_state: &AppState,page:u64,page_size:u64) -> Result<Vec<SystemRole>, DbErr> {
        let db = &app_state.db;
        SystemRoleEntity::find()
            .limit(page_size)
            .offset((page-1)*page_size)
            .all(db).await
    }
    pub async fn get_roles_total(app_state: &AppState) -> Result<u64, DbErr> {
        let db = &app_state.db;
        SystemRoleEntity::find().count(db).await
    }
    pub async fn get_role_by_id(app_state: &AppState,id:i32) -> Result<SystemRole, DbErr> {
        let db = &app_state.db;
        SystemRoleEntity::find_by_id(id).one(db).await?
            .ok_or(DbErr::RecordNotFound("Record not found".to_string()))
    }
    pub async fn get_role_menu_access(app_state: &AppState,role_id:i32) -> Result<Vec<SystemRoleMenu>, DbErr> {
        let db = &app_state.db;
        SystemRoleMenuEntity::find()
            .filter(system_role_menu::Column::SystemRoleId.eq(role_id))
            .all(db).await
    }
    pub async fn get_roles_menu_access(app_state: &AppState,role_ids:Vec<i32>) -> Result<Vec<i32>, DbErr> {
        let db = &app_state.db;
        SystemRoleMenuEntity::find()
            .filter(system_role_menu::Column::SystemRoleId.is_in(role_ids))
            .select_only()
            .column(system_role_menu::Column::SystemMenuId)
            .distinct() 
            .into_tuple::<i32>()
            .all(db).await
    }
    pub async fn get_roles_code_access(app_state: &AppState,role_ids:Vec<i32>) -> Result<Vec<String>, DbErr> {
        let db = &app_state.db;
        let result = SystemApiEntity::find()
            .find_with_related(system_role::Entity)
            .filter(system_role::Column::Id.is_in(role_ids))
            .all(db).await;
        match result {
            Ok(api_keys) => {
                let role_codes = api_keys.into_iter().map(|x| x.0).map(|y| y.api_key).collect();
                Ok(role_codes)
            }
            Err(e) => {
                Err(e)
            }
        }
    }
    pub async fn check_roles_api_access(app_state: &AppState,role_ids:Vec<i32>,path:&str) -> Result<bool, DbErr> {
        let db = &app_state.db;
        let apis= SystemApiEntity::find()
            .find_with_related(system_role::Entity)
            .filter(system_role::Column::Id.is_in(role_ids))
            .filter(system_api::Column::Path.eq(path))
            .all(db).await?;
        //role_codes去除重复项
        if apis.is_empty() {
            return Ok(false);
        }
        Ok(true)
    }
    pub async fn get_role_api_access(app_state: &AppState,role_id:i32) -> Result<Vec<SystemRoleApi>, DbErr> {
        let db = &app_state.db;
        SystemRoleApiEntity::find()
            .filter(system_role_api::Column::SystemRoleId.eq(role_id)).all(db).await
    }
    pub async fn save_role_api_access(app_state: &AppState,role_id:i32,api_ids:Vec<i32>) -> Result<bool, DbErr> {
        let db = &app_state.db;
        //查询角色存在的所有菜单
        let exists_role_apis = RoleService::get_role_api_access(app_state,role_id).await?;
        //批量保存角色和菜单关系
        let mut role_apis = Vec::new();
        for api_id in api_ids.clone() {
            //判断是否存在
            let exists = exists_role_apis.iter().find(|&x| x.system_api_id == api_id);
            if exists.is_some() {
                continue;
            }
            let role_api = system_role_api::ActiveModel {
                id:Set(0),
                system_role_id: Set(role_id),
                system_api_id: Set(api_id),
                created_at:Set(Utc::now().timestamp())
            };
            role_apis.push(role_api);
        }
        //如果老的值不存在新的值中，则删除
        let mut delete_role_apis = Vec::new();
        for exists_role_api in exists_role_apis {
            let exists = api_ids.iter().find(|&x| *x == exists_role_api.system_api_id);
            match exists {
                Some(_) => {
                    continue;
                }
                None => {
                    delete_role_apis.push(exists_role_api.system_api_id);
                }
            }   
        }
        println!("delete_role_apis:{:?}",delete_role_apis);
        let txn = db.begin().await?;
        //删除原有的关系
        SystemRoleApiEntity::delete_many().filter(system_role_api::Column::SystemApiId.is_in(delete_role_apis)).exec(&txn).await?;
        if role_apis.len() > 0 {
            SystemRoleApiEntity::insert_many(role_apis).exec(&txn).await?;
        }
        txn.commit().await?;
        Ok(true)
    }
    pub async fn save_role_menu_access(app_state: &AppState,role_id:i32,menu_ids:Vec<i32>) -> Result<bool, DbErr> {
        let db = &app_state.db;
        //查询角色存在的所有菜单
        let exists_role_menus = RoleService::get_role_menu_access(app_state,role_id).await?;
        //批量保存角色和菜单关系
        let mut role_menus = Vec::new();
        for menu_id in menu_ids.clone() {
            //判断是否存在
            let exists = exists_role_menus.iter().find(|&x| x.system_menu_id == menu_id);
            if exists.is_some() {
                continue;
            }
            let role_menu = system_role_menu::ActiveModel {
                id:Set(0),
                system_role_id: Set(role_id),
                system_menu_id: Set(menu_id),
                created_at:Set(Utc::now().timestamp())
            };
            role_menus.push(role_menu);
        }
        //如果老的值不存在新的值中，则删除
        let mut delete_role_menus = Vec::new();
        for exists_role_menu in exists_role_menus {
            let exists = menu_ids.iter().find(|&x| *x == exists_role_menu.system_menu_id);
            match exists {
                Some(_) => {
                    continue;
                }
                None => {
                    delete_role_menus.push(exists_role_menu.system_menu_id);
                }
            }   
        }
        let txn = db.begin().await?;
        //删除原有的关系
        SystemRoleMenuEntity::delete_many().filter(system_role_menu::Column::SystemMenuId.is_in(delete_role_menus)).exec(&txn).await?;
        if role_menus.len() > 0 {
            SystemRoleMenuEntity::insert_many(role_menus).exec(&txn).await?;
        }
        txn.commit().await?;
        Ok(true)
    }
    pub async fn update_role(app_state: &AppState, role: SystemRole) -> Result<SystemRole, DbErr> {
        let db = &app_state.db;
        let update_result = system_role::ActiveModel {
            role_name: Set(role.role_name),
            created_at: Set(role.created_at),
            updated_at: Set(role.updated_at),
            status: Set(role.status),
            id: Set(role.id),
        }.update(db).await?;
        SystemRoleEntity::find_by_id(update_result.id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Record not found".to_string()))
    }
    pub async fn add_role(app_state: &AppState, role: SystemRole) -> Result<SystemRole, DbErr> {
        let db = &app_state.db;
        let insert_result = system_role::ActiveModel {
            role_name: Set(role.role_name.to_owned()),
            created_at: Set(role.created_at.to_owned()),
            updated_at: Set(role.updated_at.to_owned()),
            status: Set(role.status.to_owned()),
            id: Set(role.id.to_owned()),
        }.insert(db).await?;
        Ok(insert_result)
    }
}