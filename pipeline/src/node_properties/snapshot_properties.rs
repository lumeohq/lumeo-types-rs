use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::path::PathBuf;
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "location", rename_all = "snake_case")]
pub enum SnapshotProperties {
    Local(LocalSnapshotProperties),
    LumeoCloud(LumeoCloudSnapshotProperties),
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommonSnapshotProperties {
    pub retention_duration: Option<u64>,
    pub webhook_url: Option<Url>,
    pub trigger: Option<String>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalSnapshotProperties {
    #[serde(flatten)]
    pub common: CommonSnapshotProperties,
    pub path: Option<PathBuf>,
    pub max_edge_files: Option<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LumeoCloudSnapshotProperties {
    #[serde(flatten)]
    pub common: CommonSnapshotProperties,
}
