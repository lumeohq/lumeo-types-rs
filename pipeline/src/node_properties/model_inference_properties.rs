use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::num::NonZeroU32;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ModelInferenceProperties {
    /// The ID of the inference model.
    pub model_id: Uuid,

    /// Infer only on objects another previously detected by this node.
    pub infer_on_node: Option<String>,

    /// Inference interval (frames).
    #[serde(default = "default_inference_interval")]
    pub inference_interval: NonZeroU32,

    #[serde(flatten)]
    pub runtime: Option<ModelInferenceRuntime>,
}

fn default_inference_interval() -> NonZeroU32 {
    NonZeroU32::new(1).unwrap()
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ModelInferenceRuntime {
    /// Path to the inferencing node config.
    ///
    /// This field is set to `Some` by `lumeod`.
    ///
    /// Inferencing GStreamer element requires config to be stored locally
    /// so `lumeod` should download everything required for model to run
    /// and generate a config.
    pub config: Option<std::path::PathBuf>,

    /// Inferencing Gstreamer element requires a unique numerical id for itself
    /// and for any other inferencing nodes it operates on.
    /// Map key is pipeline node id, val is unique id assigned to it at runtime.
    pub infer_node_unique_ids: HashMap<String, i32>,
}
