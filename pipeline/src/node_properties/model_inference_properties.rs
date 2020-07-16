use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ModelInferenceProperties {
    pub config: url::Url,
}
