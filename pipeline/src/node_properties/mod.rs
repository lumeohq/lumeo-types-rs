//! This module contains all kinds of Lumeo pipeline nodes

use serde::{Deserialize, Serialize};

#[macro_use]
pub mod camera_properties;
pub use camera_properties::{CameraProperties, CameraRuntime};
#[macro_use]
pub mod stream_properties;
pub use stream_properties::{StreamProperties, StreamRuntime};
pub mod encode_properties;
pub use encode_properties::EncodeProperties;
pub mod stream_rtsp_out_properties;
pub use stream_rtsp_out_properties::{StreamRtspOutProperties, StreamRtspOutRuntime};
pub mod stream_web_rtc_out_properties;
pub use stream_web_rtc_out_properties::{StreamWebRtcOutProperties, StreamWebRtcOutRuntime};
pub mod csi_camera_properties;
pub use csi_camera_properties::{CsiCameraProperties, CsiCameraRuntime};
pub mod usb_camera_properties;
pub use usb_camera_properties::{UsbCameraProperties, UsbCameraRuntime};
pub mod ip_camera_properties;
pub use ip_camera_properties::{IpCameraProperties, IpCameraRuntime};
pub mod transform_properties;
pub use transform_properties::{FlipDirection, TransformProperties};
pub mod model_inference_properties;
pub use model_inference_properties::{ModelInferenceProperties, ModelInferenceRuntime};
pub mod metadata_inserter_properties;
pub use metadata_inserter_properties::MetadataInserterProperties;
pub mod overlay_properties;
pub use overlay_properties::OverlayProperties;
pub mod clip_properties;
pub use clip_properties::ClipProperties;
pub mod snapshot_properties;
pub use snapshot_properties::SnapshotProperties;
pub mod function_properties;
pub use function_properties::{FunctionProperties, FunctionRuntime};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[allow(clippy::large_enum_variant)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NodeProperties {
    UsbCamera(UsbCameraProperties),
    CsiCamera(CsiCameraProperties),
    #[serde(rename = "camera")]
    IpCamera(IpCameraProperties),
    Encode(EncodeProperties),
    Transform(TransformProperties),
    ModelInference(ModelInferenceProperties),
    MetadataInserter(MetadataInserterProperties),
    Overlay(OverlayProperties),
    Clip(ClipProperties),
    Snapshot(SnapshotProperties),
    StreamRtspOut(StreamRtspOutProperties),
    #[serde(rename = "stream_webrtc_out")]
    StreamWebRtcOut(StreamWebRtcOutProperties),
    Function(FunctionProperties),
}
