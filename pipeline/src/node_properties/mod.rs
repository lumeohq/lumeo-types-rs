//! This module contains all kinds of Lumeo pipeline nodes

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

// FIXME: These functions will go away once props in TOML are more strictly-typed
fn get_option<T, E>(props: &HashMap<String, String>, key: &str) -> Result<Option<T>, E>
where
    T: FromStr,
    T::Err: ToString,
    E: serde::de::Error,
{
    props
        .get(key)
        .filter(|val| !val.is_empty())
        .map(|val| val.parse::<T>())
        .transpose()
        .map_err(|e| serde::de::Error::custom(&e.to_string()))
}

fn get_required<T, E>(props: &HashMap<String, String>, key: &'static str) -> Result<T, E>
where
    T: FromStr,
    T::Err: ToString,
    E: serde::de::Error,
{
    match props.get(key) {
        Some(val) => val
            .parse::<T>()
            .map_err(|e| serde::de::Error::custom(&e.to_string())),
        None => Err(serde::de::Error::missing_field(key)),
    }
}

fn serialize_option<T, S>(option: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
    T: Display,
{
    match option {
        Some(field) => serializer.serialize_some(&field.to_string()),
        None => serializer.serialize_none(),
    }
}

fn serialize_required<T, S>(field: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
    T: Display,
{
    field.to_string().serialize(serializer)
}

pub mod encode_properties;
pub use encode_properties::EncodeProperties;
pub mod stream_rtsp_out_properties;
pub use stream_rtsp_out_properties::StreamRtspOutProperties;
pub mod stream_web_rtc_out_properties;
pub use stream_web_rtc_out_properties::StreamWebRtcOutProperties;
pub mod csi_camera_properties;
pub use csi_camera_properties::CsiCameraProperties;
pub mod usb_camera_properties;
pub use usb_camera_properties::UsbCameraProperties;
pub mod ip_camera_properties;
pub use ip_camera_properties::IpCameraProperties;
pub mod convert_properties;
pub use convert_properties::ConvertProperties;
pub mod model_inference_properties;
pub use model_inference_properties::ModelInferenceProperties;
pub mod overlay_properties;
pub use overlay_properties::OverlayProperties;
pub mod clip_properties;
pub use clip_properties::ClipProperties;
pub mod snapshot_properties;
pub use snapshot_properties::SnapshotProperties;
pub mod function_properties;
pub use function_properties::FunctionProperties;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[allow(clippy::large_enum_variant)]
#[serde(tag = "type", content = "properties", rename_all = "snake_case")]
pub enum NodeProperties {
    UsbCamera(UsbCameraProperties),
    CsiCamera(CsiCameraProperties),
    #[serde(rename = "camera")]
    IpCamera(IpCameraProperties),
    Encode(EncodeProperties),
    Convert(ConvertProperties),
    ModelInference(ModelInferenceProperties),
    Overlay(OverlayProperties),
    Clip(ClipProperties),
    Snapshot(SnapshotProperties),
    StreamRtspOut(StreamRtspOutProperties),
    #[serde(rename = "stream_webrtc_out")]
    StreamWebRtcOut(StreamWebRtcOutProperties),
    Function(FunctionProperties),
}
