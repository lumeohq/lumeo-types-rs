use crate::resolution::Resolution;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsbCameraProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<Resolution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate: Option<u32>,
    #[serde(flatten)]
    pub runtime: UsbCameraRuntime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsbCameraRuntime {
    pub uri: url::Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl_camera_props!(UsbCameraProperties, UsbCameraRuntime);
