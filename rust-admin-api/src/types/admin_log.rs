#[derive(Clone,serde::Serialize,serde::Deserialize,Debug, Default)]
pub struct AdminLogPageData {
    #[serde(default)]
    pub operate_name: String,
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub start_time: u64,
    #[serde(default)]
    pub end_time: u64,
    #[serde(default)]
    pub path: String,
    pub page: u64,
    #[serde(alias = "pageSize")]
    pub page_size: u64,
    #[serde(default)]
    pub status: u32
}