#[derive(Clone,serde::Serialize,serde::Deserialize,Debug, Default)]
pub struct ConfigQueryPageData {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub key: String,
    pub page: u64,
    #[serde(alias = "pageSize")]
    pub page_size: u64,
}