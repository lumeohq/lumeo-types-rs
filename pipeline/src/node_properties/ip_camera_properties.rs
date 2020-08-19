use crate::resolution::Resolution;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpCameraProperties {
    pub source: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<Resolution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate: Option<u32>,
    #[serde(flatten)]
    pub runtime: Option<IpCameraRuntime>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpCameraRuntime {
    pub uri: url::Url,

    /// Stream ID.
    ///
    /// This field is set by API server.
    ///
    /// Stream ID is used by `lumeod` to add a WebRTC endpoint to webrtcstreamer service.
    pub stream_id: Uuid,

    /// UDP port.
    ///
    /// Currently unused.
    pub udp_port: Option<u16>,
}

impl_camera_props!(IpCameraProperties, IpCameraRuntime);
impl_stream_props!(IpCameraProperties, IpCameraRuntime, "camera");
