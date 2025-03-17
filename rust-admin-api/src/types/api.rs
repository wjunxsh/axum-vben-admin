#[derive(Clone,serde::Serialize,serde::Deserialize,Debug, Default)]
pub struct ApiQueryPageData {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub api_key: String,
    #[serde(default)]
    pub path: String,
    pub page: u64,
    #[serde(alias = "pageSize")]
    pub page_size: u64,
    #[serde(default)]
    pub status: String
}