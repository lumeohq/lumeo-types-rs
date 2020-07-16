use serde::Serialize;

// Ditch all manual Serialize + Deserialize code after changing the properties to specific types.
// This includes the use of `serialize_with` attribute.

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ConvertProperties {
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_option"
    )]
    pub fps: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<crate::Resolution>,
}

impl<'de> serde::de::Deserialize<'de> for ConvertProperties {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let props = <std::collections::HashMap<String, String>>::deserialize(deserializer)?;
        Ok(ConvertProperties {
            fps: super::get_option(&props, "fps")?,
            resolution: super::get_option(&props, "resolution")?,
        })
    }
}
