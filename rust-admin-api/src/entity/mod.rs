
pub mod system_user;
pub mod system_menu;
pub mod system_api;
pub mod system_role;
pub mod system_role_menu;
pub mod system_role_api;
pub mod system_user_role;
pub mod system_data_disctionary;
pub mod system_data_dictionary_config;
pub mod system_config;
pub mod system_admin_log;
use chrono::Utc;
pub fn default_created_at() -> i64 {
    Utc::now().timestamp() // 或者你希望的其他默认值
}
pub fn default_zero() -> i8 {
    0
}
//默认-1
pub fn default_minus_one() -> i8 {
    -1
}