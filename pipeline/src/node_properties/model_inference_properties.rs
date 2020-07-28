use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ModelInferenceProperties {
    pub runtime: ModelInferenceRuntime,
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
    // TODO: replace url::Url with fs::PathBuf
    pub config: Option<url::Url>,
}
