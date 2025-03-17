use std::collections::HashSet;

use sea_orm::entity::prelude::*;
use super::super::app_state::AppState;
use super::role::RoleService;
use super::user::UserService;
use sea_orm::DbErr;
use sea_orm::Set;

use crate::entity::system_menu::{self, Entity as SystemMenuEntity, SystemMenu};
pub struct MenuService;

impl MenuService {
    pub async fn get_menus(app_state: &AppState) -> Result<Vec<SystemMenu>, DbErr> {
        let db = &app_state.db;
        SystemMenuEntity::find().all(db).await
    }
    pub async fn get_menu(app_state: &AppState,id:i32) -> Result<SystemMenu, DbErr> {
        let db = &app_state.db;
        SystemMenuEntity::find_by_id(id).one(db).await?
        .ok_or(DbErr::RecordNotFound("Record not found".to_string()))
    }
    pub async fn get_access_menus(app_state: &AppState, user_id:i32) -> Result<Vec<SystemMenu>, DbErr> {
        let db = &app_state.db;
        let menus = SystemMenuEntity::find().all(db).await?;
        let access_roles = UserService::get_user_role_access(app_state,user_id).await?;
        let role_ids = access_roles.into_iter().map(|user_role| user_role.system_role_id).collect::<Vec<i32>>();
        if role_ids.len() == 0 {
            return Ok(vec![]);
        }
        let access_menu_ids = RoleService::get_roles_menu_access(app_state, role_ids).await?;
        let access_menus = MenuService::filter_access_menus(&menus, access_menu_ids);
        Ok(access_menus)
    }
    pub fn filter_access_menus(menus: &Vec<SystemMenu>, access_menu_ids: Vec<i32>) -> Vec<SystemMenu> {
        let access_set: HashSet<i32> = access_menu_ids.into_iter().collect();
        let mut access_menus:Vec<SystemMenu> = menus.into_iter().
            filter(|menu| access_set.contains(&menu.id))
            .cloned().collect();
        //如果子组件存在，则递归过滤父组件
        let parent_ids = access_menus.iter()
            .map(|menu| menu.parent_id)
            .filter(|parent_id| *parent_id > 0 && !access_set.contains(parent_id) ).collect::<Vec<i32>>();
        
        if parent_ids.len() > 0 {
            let parent_access_menus:Vec<SystemMenu> = MenuService::filter_access_menus(menus, parent_ids);
            // parent_access_menus.extend(access_menus);
            // access_menus = parent_access_menus;
            access_menus.extend(parent_access_menus);
        }
        access_menus
    }
    pub async fn update_menu(app_state: &AppState, menu: SystemMenu) -> Result<SystemMenu, DbErr> {
        let db = &app_state.db;
        let update_result = system_menu::ActiveModel {
            parent_id: Set(menu.parent_id),
            name:Set(menu.name), 
            title: Set(menu.title),
            link: Set(menu.link),
            path: Set(menu.path),
            component: Set(menu.component),
            meta: Set(menu.meta),
            created_at: Set(menu.created_at),
            updated_at: Set(menu.updated_at),
            id: Set(menu.id),
        }.update(db).await?;
        Ok(update_result)
    }
    pub async fn add_menu(app_state: &AppState, menu: SystemMenu) -> Result<SystemMenu, DbErr> {
        let db = &app_state.db;
        let insert_result = system_menu::ActiveModel {
            parent_id: Set(menu.parent_id),
            name:Set(menu.name), 
            title: Set(menu.title),
            link: Set(menu.link),
            path: Set(menu.path),
            component: Set(menu.component),
            meta: Set(menu.meta),
            created_at: Set(menu.created_at),
            updated_at: Set(menu.updated_at),
            id: Set(menu.id),
        }.insert(db).await?;
        Ok(insert_result)
    }
}