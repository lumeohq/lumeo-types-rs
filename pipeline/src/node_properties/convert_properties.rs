#[derive(Default, Debug, Clone, PartialEq)]
pub struct ConvertProperties {
    pub fps: Option<u32>,
    pub resolution: Option<crate::Resolution>,
}

// We can ditch this manual Deserialize impl. after changing the properties to specific types.
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
