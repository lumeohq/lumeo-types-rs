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
    /// Camera URI.
    ///
    /// This field is set by API server.
    ///
    /// This URI is used by `lumeod` to access IP camera.
    ///
    /// Example: "http://192.168.1.42:8000/onvif/device_service"
    pub uri: url::Url,

    /// Stream ID.
    ///
    /// This field is set to `Some` by API server.
    ///
    /// Stream ID is used by `lumeod` to add a WebRTC endpoint to webrtcstreamer service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<Uuid>,

    /// UDP port.
    ///
    /// Currently unused.
    pub udp_port: Option<u16>,
}

impl_camera_props!(IpCameraProperties, IpCameraRuntime);
impl_stream_props!(IpCameraProperties, IpCameraRuntime, "camera");
