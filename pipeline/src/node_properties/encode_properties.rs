use serde::Serialize;

// Ditch all manual Serialize + Deserialize code after changing the properties to specific types.
// This includes the use of `serialize_with` attribute.

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EncodeProperties {
    pub codec: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_option"
    )]
    pub max_bitrate: Option<u32>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_option"
    )]
    pub bitrate: Option<u32>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_option"
    )]
    pub quality: Option<u32>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_option"
    )]
    pub fps: Option<u32>,
}

impl<'de> serde::de::Deserialize<'de> for EncodeProperties {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let props = <std::collections::HashMap<String, String>>::deserialize(deserializer)?;
        Ok(EncodeProperties {
            codec: super::get_required(&props, "codec")?,
            max_bitrate: super::get_option(&props, "max_bitrate")?,
            bitrate: super::get_option(&props, "bitrate")?,
            quality: super::get_option(&props, "quality")?,
            fps: super::get_option(&props, "fps")?,
        })
    }
}
