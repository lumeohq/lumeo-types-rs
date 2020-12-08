use crate::resolution::Resolution;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "source_type", rename_all = "snake_case")]
pub enum VideoSourceProperties {
    Camera(CameraProperties),
    Stream(InputStreamProperties),
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommonVideoSourceProperties {
    /// ID of associated object.
    /// - if source_type=='camera' then source_id is camera ID
    /// - if source_type=='stream' then source_id is stream ID
    pub source_id: Uuid,

    /// Name of video source.
    pub source_name: String,

    /// Resolution of video source.
    /// If unset then some reasonable default is used.
    pub resolution: Option<Resolution>,

    /// Framerate of video source.
    /// If unset then some reasonable default is used.
    #[serde(alias = "fps")]
    pub framerate: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraProperties {
    #[serde(flatten)]
    pub common: CommonVideoSourceProperties,
    #[serde(flatten)]
    pub runtime: Option<CameraRuntime>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputStreamProperties {
    #[serde(flatten)]
    pub common: CommonVideoSourceProperties,
    #[serde(flatten)]
    pub runtime: Option<InputStreamRuntime>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CameraRuntime {
    Usb(UsbCameraRuntime),
    Csi(CsiCameraRuntime),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputStreamRuntime {
    Rtsp(InputRtspStreamRuntime),
    WebRtc(InputWebRtcStreamRuntime),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsbCameraRuntime {
    /// Local USB camera URI.
    ///
    /// Example: "file:///dev/video0"
    pub uri: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CsiCameraRuntime {
    /// Local CSI camera URI.
    ///
    /// Example: "file:///dev/video0"
    pub uri: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputRtspStreamRuntime {
    /// RTSP stream URI.
    ///
    /// Example: "rtsp://192.168.0.42:554/hd_stream"
    pub uri: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputWebRtcStreamRuntime {
    // TODO: define how do we use WebRTC streams as inputs
}
