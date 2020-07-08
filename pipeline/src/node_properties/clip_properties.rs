use std::path::PathBuf;
use url::Url;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClipProperties {
    pub path: Option<PathBuf>,
    pub max_duration_sec: Option<u64>,
    pub max_size_bytes: Option<u64>,
    pub max_edge_files: Option<u64>,
    pub edge_retention_duration: Option<u64>,
    pub cloud_upload: Option<bool>,
    pub cloud_retention_duration: Option<u64>,
    pub webhook_url: Option<Url>,
}

// We can ditch this manual Deserialize impl. after changing the properties to specific types.
impl<'de> serde::de::Deserialize<'de> for ClipProperties {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let props = <std::collections::HashMap<String, String>>::deserialize(deserializer)?;
        Ok(ClipProperties {
            path: super::get_option(&props, "path")?,
            max_duration_sec: super::get_option(&props, "max_duration_sec")?,
            max_size_bytes: super::get_option(&props, "max_size_bytes")?,
            max_edge_files: super::get_option(&props, "max_edge_files")?,
            edge_retention_duration: super::get_option(&props, "edge_retention_duration")?,
            cloud_upload: super::get_option(&props, "cloud_upload")?,
            cloud_retention_duration: super::get_option(&props, "cloud_retention_duration")?,
            webhook_url: super::get_option(&props, "webhook_url")?,
        })
    }
}
