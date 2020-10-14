use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "location", rename_all = "snake_case")]
pub enum ClipProperties {
    Local(LocalClipProperties),
    LumeoCloud(LumeoCloudClipProperties),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommonClipProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalClipProperties {
    #[serde(flatten)]
    pub common: CommonClipProperties,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_edge_files: Option<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LumeoCloudClipProperties {
    #[serde(flatten)]
    pub common: CommonClipProperties,
}
