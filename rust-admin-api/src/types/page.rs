use std::fmt::Debug;
use serde::{Deserialize, Serialize};
pub use hyper::{HeaderMap, StatusCode};
#[derive(Debug,Clone, Serialize, Deserialize,Default)]
pub struct ClaimPage {
    pub page: u64,
    #[serde(alias = "pageSize")]
    pub page_size: u64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u64,
    pub page_size: u64,
    pub total: u64,
    pub total_page: u64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PageData<T> {
    pub pagination: Pagination,
    pub items: Vec<T>,
}