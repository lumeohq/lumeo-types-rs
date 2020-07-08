use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ModelInferenceProperties {
    pub config: url::Url,
}
