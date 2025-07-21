// config.rs
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PackageInfo {
    pub id: String,
    pub list: Option<String>,
    pub description: Option<String>,
    pub dependencies: Option<Vec<String>>,
    #[serde(rename = "neededBy")]
    pub needed_by: Option<Vec<String>>,
    pub labels: Option<Vec<String>>,
    pub removal: Option<String>,
}

pub fn load_uad_list() -> Vec<PackageInfo> {
    const UAD_JSON: &str = include_str!("../assets/data/uad_lists.json");

    serde_json::from_str(UAD_JSON).expect("Failed to parse embedded uad_lists.json")
}