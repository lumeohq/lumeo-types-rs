use crate::resolution::Resolution;

#[derive(Debug, Clone, PartialEq)]
pub struct CsiCameraProperties {
    pub uri: url::Url,
    pub resolution: Option<Resolution>,
    pub framerate: Option<u32>,
}

// We can ditch this manual Deserialize impl. after changing the properties to specific types.
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
