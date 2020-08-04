use crate::resolution::Resolution;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CsiCameraProperties {
    pub uri: url::Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<Resolution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate: Option<u32>,
    #[serde(flatten)]
    pub runtime: CsiCameraRuntime,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct CsiCameraRuntime {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl_camera_props!(CsiCameraProperties, CsiCameraRuntime);
