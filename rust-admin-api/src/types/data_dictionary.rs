use serde::{Deserialize, Serialize};
use crate::entity::default_minus_one;
#[derive(Debug,Clone, Serialize, Deserialize,Default)]
pub struct ClaimQueryData {
    pub page: u64,
    #[serde(alias = "pageSize", default)]
    pub page_size: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default="default_minus_one")]
    pub status: i8,
}
#[derive(Debug,Clone, Serialize, Deserialize,Default)]
pub struct ClaimQueryConfigData {
    #[serde(default)]
    pub data_dictionary_id: i32,
}
#[derive(Debug,Clone, Serialize, Deserialize,Default)]
pub struct ClaimDeleteConfigData {
    #[serde(default)]
    pub data_dictionary_id: i32,
    pub id:i32,
}