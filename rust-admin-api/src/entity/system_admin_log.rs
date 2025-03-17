use s_derive::Sea;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone,Sea, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "system_admin_log", comment = "系统管理员操作日志")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: u64, // `bigint unsigned` 对应 Rust 的 `u64`
    pub system_user_id: i32, // 用户 ID
    pub system_real_name: String, // 用户真实姓名
    pub path: String,        // 请求路径
    pub method: String,      // HTTP 方法
    pub request_url: String,  // url完整请求连接
    pub request_body: Option<Value>, // JSON 请求主体
    pub response_data: Option<Value>, // JSON 响应数据
    pub ip: String,          // IP 地址
    pub user_agent: String,  // 浏览器信息
    pub response_time_ms: i64, // 请求耗时 (毫秒)
    pub response_status: u32,  // HTTP 状态码
    pub operate_name: String,  // 操作名称
    pub error_message: String, // 错误信息
    pub created_at: u64, // 创建时间 (Unix 时间戳)
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}