//! This module contains all kinds of Lumeo pipeline nodes

use serde::{Deserialize, Serialize};

pub mod video_source_properties;
pub use video_source_properties::{
    CameraProperties, CameraRuntime, CommonVideoSourceProperties, InputRtspStreamRuntime,
    InputStreamProperties, InputStreamRuntime, InputWebRtcStreamRuntime, UsbCameraRuntime,
    VideoSourceProperties,
};
#[macro_use]
pub mod stream_properties;
pub use stream_properties::{StreamProperties, StreamRuntime};
pub mod encode_properties;
pub use encode_properties::EncodeProperties;
pub mod stream_rtsp_out_properties;
pub use stream_rtsp_out_properties::{StreamRtspOutProperties, StreamRtspOutRuntime};
pub mod stream_web_rtc_out_properties;
pub use stream_web_rtc_out_properties::{StreamWebRtcOutProperties, StreamWebRtcOutRuntime};
pub mod transform_properties;
pub use transform_properties::{FlipDirection, TransformProperties};
pub mod model_inference_properties;
pub use model_inference_properties::{ModelInferenceProperties, ModelInferenceRuntime};
pub mod metadata_inserter_properties;
pub use metadata_inserter_properties::MetadataInserterProperties;
pub mod overlay_properties;
pub use overlay_properties::OverlayProperties;
pub mod clip_properties;
pub use clip_properties::{
    ClipProperties, CommonClipProperties, LocalClipProperties, LumeoCloudClipProperties,
};
pub mod snapshot_properties;
pub use snapshot_properties::{
    CommonSnapshotProperties, LocalSnapshotProperties, LumeoCloudSnapshotProperties,
    SnapshotProperties,
};
pub mod track_properties;
pub use track_properties::{TrackProperties, Tracker};
pub mod function_properties;
pub use function_properties::{FunctionProperties, FunctionRuntime};
pub mod combine_properties;
pub use combine_properties::CombineProperties;
pub mod grid_properties;
pub use grid_properties::GridProperties;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[allow(clippy::large_enum_variant)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NodeProperties {
    #[serde(rename = "video")]
    VideoSource(VideoSourceProperties),
    Encode(EncodeProperties),
    Transform(TransformProperties),
    ModelInference(ModelInferenceProperties),
    #[serde(rename = "metadata_add")]
    MetadataInserter(MetadataInserterProperties),
    Overlay(OverlayProperties),
    Clip(ClipProperties),
    Snapshot(SnapshotProperties),
    StreamRtspOut(StreamRtspOutProperties),
    #[serde(rename = "stream_webrtc_out")]
    StreamWebRtcOut(StreamWebRtcOutProperties),
    Track(TrackProperties),
    Function(FunctionProperties),
    Combine(CombineProperties),
    Grid(GridProperties),
}
