#[derive(Default, Debug, Clone, PartialEq)]
pub struct EncodeProperties {
    pub codec: String,
    pub max_bitrate: Option<u32>,
    pub bitrate: Option<u32>,
    pub quality: Option<u32>,
    pub fps: Option<u32>,
}

// We can ditch this manual Deserialize impl. after changing the properties to specific types.
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
