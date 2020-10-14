use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::path::PathBuf;
use url::Url;

#[skip_serializing_none]
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotProperties {
    pub path: Option<PathBuf>,
    pub max_edge_files: Option<u64>,
    pub edge_retention_duration: Option<u64>,
    pub cloud_upload: Option<bool>,
    pub cloud_retention_duration: Option<u64>,
    pub webhook_url: Option<Url>,
}
