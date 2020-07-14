use crate::resolution::Resolution;
use serde::Serialize;

// Ditch all manual Serialize + Deserialize code after changing the properties to specific types.
// This includes the use of `serialize_with` attribute.

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CsiCameraProperties {
    pub uri: url::Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<Resolution>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_option"
    )]
    pub framerate: Option<u32>,
}

impl<'de> serde::de::Deserialize<'de> for CsiCameraProperties {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let props = <std::collections::HashMap<String, String>>::deserialize(deserializer)?;
        Ok(CsiCameraProperties {
            uri: super::get_required(&props, "uri")?,
            resolution: super::get_option(&props, "resolution")?,
            framerate: super::get_option(&props, "framerate")?,
        })
    }
}
