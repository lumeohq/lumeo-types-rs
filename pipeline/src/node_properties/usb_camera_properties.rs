use crate::resolution::Resolution;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsbCameraProperties {
    pub source: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<Resolution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate: Option<u32>,
    #[serde(flatten)]
    pub runtime: Option<UsbCameraRuntime>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsbCameraRuntime {
    /// Camera URI.
    ///
    /// This field is set by API server.
    ///
    /// This URI is used by `lumeod` to access local camera.
    ///
    /// Example: "file:///dev/video0"
    pub uri: url::Url,
}

impl_camera_props!(UsbCameraProperties, UsbCameraRuntime);
