use serde::Serialize;
use std::path::PathBuf;
use url::Url;

// Ditch all manual Serialize + Deserialize code after changing the properties to specific types.
// This includes the use of `serialize_with` attribute.

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct SnapshotProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_option"
    )]
    pub max_edge_files: Option<u64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_option"
    )]
    pub edge_retention_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_upload: Option<bool>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_option"
    )]
    pub cloud_retention_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<Url>,
}

impl<'de> serde::de::Deserialize<'de> for SnapshotProperties {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let props = <std::collections::HashMap<String, String>>::deserialize(deserializer)?;
        Ok(SnapshotProperties {
            path: super::get_option(&props, "path")?,
            max_edge_files: super::get_option(&props, "max_edge_files")?,
            edge_retention_duration: super::get_option(&props, "edge_retention_duration")?,
            cloud_upload: super::get_option(&props, "cloud_upload")?,
            cloud_retention_duration: super::get_option(&props, "cloud_retention_duration")?,
            webhook_url: super::get_option(&props, "webhook_url")?,
        })
    }
}
