use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_edge_files: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_retention_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_upload: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_retention_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<Url>,
}
