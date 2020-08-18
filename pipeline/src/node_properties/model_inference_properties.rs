use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ModelInferenceProperties {
    /// The ID of the inference model.
    pub model_id: Uuid,

    pub runtime: Option<ModelInferenceRuntime>,
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
}
